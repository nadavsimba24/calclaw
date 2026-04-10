                "learning_applied": completed_task.learning_applied,
                "result": completed_task.result.output,
            }
        }));
    }
    
    (StatusCode::NOT_FOUND, Json(json!({"error": "Task not found"})))
}

/// 🚀 Execute a specific task
async fn execute_task(
    State(state): State<OrchestrationAppState>,
    Path(task_id): Path<Uuid>,
) -> impl IntoResponse {
    let mut engine = state.engine.write().await;
    
    // חפש את ה-task בתור
    let task_index = engine.task_queue.iter()
        .position(|t| t.id == task_id);
    
    match task_index {
        Some(index) => {
            // הוצא את ה-task מהתור
            let task = engine.task_queue.remove(index).unwrap();
            
            // הפעל את ה-task ישירות
            let result = engine.execute_task_internal(&task).await;
            
            // הוסף ל-completed tasks
            let completed_task = CompletedTask {
                task,
                start_time: chrono::Utc::now(),
                end_time: chrono::Utc::now(),
                result: result.clone(),
                resources_used: ResourcesUsed {
                    cpu_seconds: 0.0,
                    memory_mb: 0.0,
                    network_mb: 0.0,
                    api_calls: 0,
                },
                learning_applied: false,
            };
            
            engine.completed_tasks.push(completed_task);
            
            // עדכן performance tracker
            engine.performance_tracker.total_tasks += 1;
            if result.success {
                engine.performance_tracker.successful_tasks += 1;
            } else {
                engine.performance_tracker.failed_tasks += 1;
            }
            
            Json(json!({
                "status": "executed",
                "task_id": task_id,
                "success": result.success,
                "execution_time_seconds": result.execution_time_seconds,
                "result": result.output,
            }))
        }
        None => {
            (StatusCode::NOT_FOUND, Json(json!({
                "error": "Task not found in queue",
                "suggestion": "Task might be already executing or completed"
            })))
        }
    }
}

/// ❌ Cancel a task
async fn cancel_task(
    State(state): State<OrchestrationAppState>,
    Path(task_id): Path<Uuid>,
) -> impl IntoResponse {
    let mut engine = state.engine.write().await;
    
    // בדוק אם המשימה בתור
    let queue_index = engine.task_queue.iter()
        .position(|t| t.id == task_id);
    
    if let Some(index) = queue_index {
        // הסר מהתור
        let task = engine.task_queue.remove(index).unwrap();
        
        // עדכן את הסטטוס
        let mut cancelled_task = task;
        cancelled_task.status = TaskStatus::Cancelled;
        cancelled_task.result = Some(TaskResult {
            success: false,
            output: serde_json::json!({"status": "cancelled"}),
            error_message: Some("Task cancelled by user".to_string()),
            execution_time_seconds: 0,
            resources_used: ResourcesUsed {
                cpu_seconds: 0.0,
                memory_mb: 0.0,
                network_mb: 0.0,
                api_calls: 0,
            },
        });
        
        // הוסף ל-completed tasks
        let completed_task = CompletedTask {
            task: cancelled_task,
            start_time: chrono::Utc::now(),
            end_time: chrono::Utc::now(),
            result: TaskResult {
                success: false,
                output: serde_json::json!({"status": "cancelled"}),
                error_message: Some("Task cancelled by user".to_string()),
                execution_time_seconds: 0,
                resources_used: ResourcesUsed {
                    cpu_seconds: 0.0,
                    memory_mb: 0.0,
                    network_mb: 0.0,
                    api_calls: 0,
                },
            },
            resources_used: ResourcesUsed {
                cpu_seconds: 0.0,
                memory_mb: 0.0,
                network_mb: 0.0,
                api_calls: 0,
            },
            learning_applied: false,
        };
        
        engine.completed_tasks.push(completed_task);
        
        Json(json!({
            "status": "cancelled",
            "task_id": task_id,
            "queue_position": index + 1,
        }))
    } else if engine.active_tasks.contains_key(&task_id) {
        // המשימה פעילה - לא ניתן לבטל כרגע
        // בפועל, זה היה שולח signal להפסקה
        (StatusCode::CONFLICT, Json(json!({
            "error": "Task is currently executing",
            "suggestion": "Wait for task to complete or implement graceful cancellation"
        })))
    } else {
        (StatusCode::NOT_FOUND, Json(json!({"error": "Task not found"})))
    }
}

/// 🚀 Execute next task in queue
async fn execute_next_task(
    State(state): State<OrchestrationAppState>,
) -> impl IntoResponse {
    let mut engine = state.engine.write().await;
    
    match engine.execute_next_task().await {
        Some(result) => {
            Json(json!({
                "status": "executed",
                "success": result.success,
                "execution_time_seconds": result.execution_time_seconds,
                "result": result.output,
                "tasks_remaining": engine.task_queue.len(),
            }))
        }
        None => {
            (StatusCode::NO_CONTENT, Json(json!({
                "status": "no_tasks",
                "message": "No tasks in queue to execute"
            })))
        }
    }
}

/// 🔄 Execute batch of tasks
async fn execute_batch_tasks(
    State(state): State<OrchestrationAppState>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    let mut engine = state.engine.write().await;
    
    let count = payload.get("count")
        .and_then(|v| v.as_u64())
        .unwrap_or(5) as usize;
    
    let mut results = Vec::new();
    let mut executed = 0;
    
    for _ in 0..count {
        if let Some(result) = engine.execute_next_task().await {
            results.push(json!({
                "success": result.success,
                "execution_time": result.execution_time_seconds,
            }));
            executed += 1;
        } else {
            break;
        }
    }
    
    Json(json!({
        "status": "batch_executed",
        "executed": executed,
        "requested": count,
        "results": results,
        "tasks_remaining": engine.task_queue.len(),
    }))
}

/// 🤖 Get agent status
async fn get_agent_status(
    State(state): State<OrchestrationAppState>,
) -> impl IntoResponse {
    let engine = state.engine.read().await;
    
    Json(json!({
        "agent_id": engine.agent_state.agent_id,
        "status": format!("{:?}", engine.agent_state.status),
        "last_activity": engine.agent_state.last_activity.to_rfc3339(),
        "memory": {
            "recent_decisions": engine.agent_state.memory.recent_decisions.len(),
            "learned_patterns": engine.agent_state.memory.learned_patterns.len(),
            "successful_actions": engine.agent_state.memory.successful_actions.len(),
            "failed_actions": engine.agent_state.memory.failed_actions.len(),
        },
        "context": {
            "current_goal": engine.agent_state.current_context.current_goal,
            "active_department": engine.agent_state.current_context.active_department,
            "relevant_data_entities": engine.agent_state.current_context.relevant_data_entities.len(),
            "constraints": engine.agent_state.current_context.constraints.len(),
            "priorities": engine.agent_state.current_context.priorities.len(),
        },
    }))
}

/// 🛠️ Get agent capabilities
async fn get_agent_capabilities(
    State(state): State<OrchestrationAppState>,
) -> impl IntoResponse {
    let engine = state.engine.read().await;
    
    let capabilities: Vec<serde_json::Value> = engine.agent_state.capabilities.iter()
        .map(|cap| {
            json!({
                "id": cap.id,
                "name": cap.name,
                "description": cap.description,
                "category": format!("{:?}", cap.category),
                "complexity": format!("{:?}", cap.complexity),
                "execution_time_estimate": cap.execution_time_estimate,
            })
        })
        .collect();
    
    Json(json!({
        "capabilities": capabilities,
        "count": capabilities.len(),
    }))
}

/// 🧠 Get agent memory
async fn get_agent_memory(
    State(state): State<OrchestrationAppState>,
) -> impl IntoResponse {
    let engine = state.engine.read().await;
    
    let recent_decisions: Vec<serde_json::Value> = engine.agent_state.memory.recent_decisions.iter()
        .map(|decision| {
            json!({
                "id": decision.id,
                "context": decision.context,
                "chosen_option": decision.chosen_option,
                "success": decision.outcome.success,
                "learned": decision.learned,
            })
        })
        .collect();
    
    let learned_patterns: Vec<serde_json::Value> = engine.agent_state.memory.learned_patterns.iter()
        .map(|pattern| {
            json!({
                "id": pattern.id,
                "name": pattern.name,
                "pattern_type": format!("{:?}", pattern.pattern_type),
                "detection_rule": pattern.detection_rule,
                "frequency": pattern.frequency,
                "significance": pattern.significance,
            })
        })
        .collect();
    
    Json(json!({
        "recent_decisions": {
            "items": recent_decisions,
            "count": recent_decisions.len(),
            "capacity": engine.agent_state.memory.recent_decisions.capacity(),
        },
        "learned_patterns": {
            "items": learned_patterns,
            "count": learned_patterns.len(),
        },
        "successful_actions": engine.agent_state.memory.successful_actions.len(),
        "failed_actions": engine.agent_state.memory.failed_actions.len(),
    }))
}

/// 📋 Get agent context
async fn get_agent_context(
    State(state): State<OrchestrationAppState>,
) -> impl IntoResponse {
    let engine = state.engine.read().await;
    
    Json(json!({
        "current_goal": engine.agent_state.current_context.current_goal,
        "active_department": engine.agent_state.current_context.active_department,
        "relevant_data_entities": engine.agent_state.current_context.relevant_data_entities,
        "constraints": engine.agent_state.current_context.constraints,
        "priorities": engine.agent_state.current_context.priorities,
    }))
}

/// ✏️ Update agent context
async fn update_agent_context(
    State(state): State<OrchestrationAppState>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    let mut engine = state.engine.write().await;
    
    if let Some(goal_id) = payload.get("current_goal") {
        if let Some(goal_str) = goal_id.as_str() {
            if let Ok(goal_uuid) = Uuid::parse_str(goal_str) {
                engine.agent_state.current_context.current_goal = Some(goal_uuid);
            }
        } else if goal_id.is_null() {
            engine.agent_state.current_context.current_goal = None;
        }
    }
    
    if let Some(dept_id) = payload.get("active_department") {
        if let Some(dept_str) = dept_id.as_str() {
            if let Ok(dept_uuid) = Uuid::parse_str(dept_str) {
                engine.agent_state.current_context.active_department = Some(dept_uuid);
            }
        } else if dept_id.is_null() {
            engine.agent_state.current_context.active_department = None;
        }
    }
    
    if let Some(data_entities) = payload.get("relevant_data_entities") {
        if let Some(array) = data_entities.as_array() {
            let entities: Vec<Uuid> = array.iter()
                .filter_map(|v| v.as_str())
                .filter_map(|s| Uuid::parse_str(s).ok())
                .collect();
            engine.agent_state.current_context.relevant_data_entities = entities;
        }
    }
    
    Json(json!({
        "status": "updated",
        "context": {
            "current_goal": engine.agent_state.current_context.current_goal,
            "active_department": engine.agent_state.current_context.active_department,
            "relevant_data_entities": engine.agent_state.current_context.relevant_data_entities.len(),
        }
    }))
}

/// 📊 Get performance data
async fn get_performance(
    State(state): State<OrchestrationAppState>,
) -> impl IntoResponse {
    let engine = state.engine.read().await;
    
    let total_tasks = engine.performance_tracker.total_tasks;
    let successful_tasks = engine.performance_tracker.successful_tasks;
    let failed_tasks = engine.performance_tracker.failed_tasks;
    
    let success_rate = if total_tasks > 0 {
        successful_tasks as f64 / total_tasks as f64
    } else {
        0.0
    };
    
    let average_processing_time = if total_tasks > 0 {
        engine.performance_tracker.total_processing_time / total_tasks as f64
    } else {
        0.0
    };
    
    Json(json!({
        "total_tasks": total_tasks,
        "successful_tasks": successful_tasks,
        "failed_tasks": failed_tasks,
        "success_rate": success_rate,
        "average_processing_time": average_processing_time,
        "efficiency_score": engine.performance_tracker.efficiency_score,
        "learning_score": engine.performance_tracker.learning_score,
        "uptime_seconds": chrono::Utc::now()
            .signed_duration_since(engine.performance_tracker.start_time)
            .num_seconds(),
    }))
}

/// 📈 Get performance history
async fn get_performance_history(
    State(state): State<OrchestrationAppState>,
) -> impl IntoResponse {
    let engine = state.engine.read().await;
    
    let history: Vec<serde_json::Value> = engine.performance_tracker.historical_data.iter()
        .map(|point| {
            json!({
                "timestamp": point.timestamp.to_rfc3339(),
                "tasks_completed": point.tasks_completed,
                "success_rate": point.success_rate,
                "efficiency": point.efficiency,
                "learning_rate": point.learning_rate,
            })
        })
        .collect();
    
    Json(json!({
        "history": history,
        "count": history.len(),
        "time_range": if history.len() > 1 {
            let first = &history[0];
            let last = &history[history.len() - 1];
            format!("{} to {}", first["timestamp"], last["timestamp"])
        } else {
            "Single point".to_string()
        },
    }))
}

/// 🧠 Get learning status
async fn get_learning_status(
    State(state): State<OrchestrationAppState>,
) -> impl IntoResponse {
    let engine = state.engine.read().await;
    
    let decisions_made = engine.learning_loop.decisions_made;
    let successful_decisions = engine.learning_loop.successful_decisions;
    let failed_decisions = engine.learning_loop.failed_decisions;
    
    let decision_success_rate = if decisions_made > 0 {
        successful_decisions as f64 / decisions_made as f64
    } else {
        0.0
    };
    
    Json(json!({
        "decisions": {
            "made": decisions_made,
            "successful": successful_decisions,
            "failed": failed_decisions,
            "success_rate": decision_success_rate,
        },
        "learned_patterns": engine.learning_loop.learned_patterns.len(),
        "decision_history": engine.learning_loop.decision_history.len(),
        "improvement_rate": engine.learning_loop.improvement_rate,
    }))
}

/// 📝 Get decision history
async fn get_decision_history(
    State(state): State<OrchestrationAppState>,
) -> impl IntoResponse {
    let engine = state.engine.read().await;
    
    let history: Vec<serde_json::Value> = engine.learning_loop.decision_history.iter()
        .rev() // הראה חדשות קודם
        .take(20) // הגבל ל-20 האחרונות
        .map(|record| {
            json!({
                "decision_id": record.decision_id,
                "context_hash": &record.context_hash[..20], // רק תחילת ה-hash
                "options_considered": record.options_considered.len(),
                "chosen_option": record.chosen_option,
                "success": record.outcome.success,
                "timestamp": record.timestamp.to_rfc3339(),
                "learning_applied": record.learning_applied,
            })
        })
        .collect();
    
    Json(json!({
        "history": history,
        "count": history.len(),
        "total_decisions": engine.learning_loop.decision_history.len(),
    }))
}

/// 🧩 Get learned patterns
async fn get_learned_patterns(
    State(state): State<OrchestrationAppState>,
) -> impl IntoResponse {
    let engine = state.engine.read().await;
    
    let patterns: Vec<serde_json::Value> = engine.learning_loop.learned_patterns.iter()
        .map(|pattern| {
            json!({
                "pattern_id": pattern.pattern_id,
                "pattern_type": format!("{:?}", pattern.pattern_type),
                "confidence": pattern.confidence,
                "applications": pattern.applications,
                "success_rate": pattern.success_rate,
                "last_applied": pattern.last