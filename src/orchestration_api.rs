// 🚀 Orchestration Engine API
// REST API endpoints for managing the orchestration engine

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put, delete},
    Json, Router,
};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::orchestration_engine::*;
use crate::organization_ontology::*;

/// 🏢 App State for orchestration engine
#[derive(Clone)]
pub struct OrchestrationAppState {
    pub engine: Arc<RwLock<OrchestrationEngine>>,
}

/// 🚀 Create orchestration API router
pub fn create_orchestration_api() -> Router<OrchestrationAppState> {
    Router::new()
        // Engine status and control
        .route("/api/orchestration/status", get(get_engine_status))
        .route("/api/orchestration/health", get(get_engine_health))
        .route("/api/orchestration/heartbeat", post(trigger_heartbeat))
        .route("/api/orchestration/metrics", get(get_engine_metrics))
        
        // Task management
        .route("/api/orchestration/tasks", get(get_all_tasks))
        .route("/api/orchestration/tasks", post(create_task))
        .route("/api/orchestration/tasks/queue", get(get_task_queue))
        .route("/api/orchestration/tasks/active", get(get_active_tasks))
        .route("/api/orchestration/tasks/completed", get(get_completed_tasks))
        .route("/api/orchestration/tasks/:id", get(get_task))
        .route("/api/orchestration/tasks/:id/execute", post(execute_task))
        .route("/api/orchestration/tasks/:id/cancel", post(cancel_task))
        .route("/api/orchestration/tasks/execute-next", post(execute_next_task))
        .route("/api/orchestration/tasks/batch-execute", post(execute_batch_tasks))
        
        // Agent management
        .route("/api/orchestration/agent/status", get(get_agent_status))
        .route("/api/orchestration/agent/capabilities", get(get_agent_capabilities))
        .route("/api/orchestration/agent/memory", get(get_agent_memory))
        .route("/api/orchestration/agent/context", get(get_agent_context))
        .route("/api/orchestration/agent/context", put(update_agent_context))
        
        // Performance and learning
        .route("/api/orchestration/performance", get(get_performance))
        .route("/api/orchestration/performance/history", get(get_performance_history))
        .route("/api/orchestration/learning", get(get_learning_status))
        .route("/api/orchestration/learning/decisions", get(get_decision_history))
        .route("/api/orchestration/learning/patterns", get(get_learned_patterns))
        
        // Recommendations
        .route("/api/orchestration/recommendations", get(get_recommendations))
        .route("/api/orchestration/recommendations/:id/apply", post(apply_recommendation))
        
        // Subsystem management
        .route("/api/orchestration/subsystems", get(get_subsystem_status))
        .route("/api/orchestration/subsystems/:name/health", get(get_subsystem_health))
        .route("/api/orchestration/subsystems/:name/restart", post(restart_subsystem))
        
        // Integration endpoints
        .route("/api/orchestration/integrations/cron", get(get_cron_integration))
        .route("/api/orchestration/integrations/cron/sync", post(sync_cron_jobs))
        .route("/api/orchestration/integrations/updates", get(get_update_status))
        .route("/api/orchestration/integrations/updates/check", post(check_for_updates))
        
        // Engine control
        .route("/api/orchestration/engine/start", post(start_engine))
        .route("/api/orchestration/engine/stop", post(stop_engine))
        .route("/api/orchestration/engine/pause", post(pause_engine))
        .route("/api/orchestration/engine/resume", post(resume_engine))
        .route("/api/orchestration/engine/reset", post(reset_engine))
        
        // Health check
        .route("/api/orchestration/health-check", get(health_check))
}

/// 🩺 Health check
async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({
        "status": "healthy",
        "service": "orchestration",
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

/// 📊 Get engine status
async fn get_engine_status(
    State(state): State<OrchestrationAppState>,
) -> impl IntoResponse {
    let engine = state.engine.read().await;
    let status = engine.get_engine_status();
    
    Json(status)
}

/// 🩺 Get engine health
async fn get_engine_health(
    State(state): State<OrchestrationAppState>,
) -> impl IntoResponse {
    let engine = state.engine.read().await;
    
    Json(json!({
        "heartbeat": {
            "count": engine.heartbeat_manager.heartbeat_count,
            "last_heartbeat": engine.heartbeat_manager.last_heartbeat.to_rfc3339(),
            "interval_seconds": engine.heartbeat_manager.heartbeat_interval_seconds,
        },
        "health_status": {
            "overall": format!("{:?}", engine.heartbeat_manager.health_status.overall),
            "cpu_usage": engine.heartbeat_manager.health_status.cpu_usage,
            "memory_usage": engine.heartbeat_manager.health_status.memory_usage,
            "disk_usage": engine.heartbeat_manager.health_status.disk_usage,
            "network_latency": engine.heartbeat_manager.health_status.network_latency,
            "error_rate": engine.heartbeat_manager.health_status.error_rate,
        },
        "subsystems": engine.heartbeat_manager.subsystem_status.len(),
        "timestamp": chrono::Utc::now().to_rfc3339(),
    }))
}

/// 💓 Trigger heartbeat manually
async fn trigger_heartbeat(
    State(state): State<OrchestrationAppState>,
) -> impl IntoResponse {
    let mut engine = state.engine.write().await;
    engine.heartbeat().await;
    
    Json(json!({
        "status": "heartbeat_triggered",
        "count": engine.heartbeat_manager.heartbeat_count,
        "timestamp": engine.heartbeat_manager.last_heartbeat.to_rfc3339(),
    }))
}

/// 📈 Get engine metrics
async fn get_engine_metrics(
    State(state): State<OrchestrationAppState>,
) -> impl IntoResponse {
    let engine = state.engine.read().await;
    let snapshot = &engine.heartbeat_manager.metrics_snapshot;
    
    Json(json!({
        "metrics": {
            "tasks_processed": snapshot.tasks_processed,
            "tasks_succeeded": snapshot.tasks_succeeded,
            "tasks_failed": snapshot.tasks_failed,
            "average_processing_time": snapshot.average_processing_time,
            "memory_usage_mb": snapshot.memory_usage_mb,
            "cpu_percent": snapshot.cpu_percent,
        },
        "performance": {
            "total_tasks": engine.performance_tracker.total_tasks,
            "successful_tasks": engine.performance_tracker.successful_tasks,
            "failed_tasks": engine.performance_tracker.failed_tasks,
            "efficiency_score": engine.performance_tracker.efficiency_score,
            "learning_score": engine.performance_tracker.learning_score,
        },
        "timestamp": snapshot.timestamp.to_rfc3339(),
    }))
}

/// 📋 Get all tasks (queued, active, completed)
async fn get_all_tasks(
    State(state): State<OrchestrationAppState>,
) -> impl IntoResponse {
    let engine = state.engine.read().await;
    
    let queued_tasks: Vec<serde_json::Value> = engine.task_queue.iter()
        .map(|task| {
            json!({
                "id": task.id,
                "name": task.name,
                "description": task.description,
                "status": format!("{:?}", task.status),
                "priority": format!("{:?}", task.priority),
                "progress": task.progress,
            })
        })
        .collect();
    
    let active_tasks: Vec<serde_json::Value> = engine.active_tasks.values()
        .map(|active_task| {
            json!({
                "id": active_task.task.id,
                "name": active_task.task.name,
                "start_time": active_task.start_time.to_rfc3339(),
                "progress": active_task.progress,
                "status": format!("{:?}", active_task.status),
                "heartbeat_count": active_task.heartbeat_count,
                "last_heartbeat": active_task.last_heartbeat.to_rfc3339(),
            })
        })
        .collect();
    
    let completed_tasks: Vec<serde_json::Value> = engine.completed_tasks.iter()
        .map(|completed_task| {
            json!({
                "id": completed_task.task.id,
                "name": completed_task.task.name,
                "start_time": completed_task.start_time.to_rfc3339(),
                "end_time": completed_task.end_time.to_rfc3339(),
                "success": completed_task.result.success,
                "execution_time_seconds": completed_task.result.execution_time_seconds,
                "learning_applied": completed_task.learning_applied,
            })
        })
        .collect();
    
    Json(json!({
        "queued": queued_tasks,
        "active": active_tasks,
        "completed": completed_tasks,
        "counts": {
            "queued": queued_tasks.len(),
            "active": active_tasks.len(),
            "completed": completed_tasks.len(),
            "total": queued_tasks.len() + active_tasks.len() + completed_tasks.len(),
        }
    }))
}

/// ➕ Create a new task
async fn create_task(
    State(state): State<OrchestrationAppState>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    let mut engine = state.engine.write().await;
    
    let name = payload.get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            (StatusCode::BAD_REQUEST, Json(json!({"error": "Missing task name"})))
        })?;
    
    let description = payload.get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    
    let capability_name = payload.get("capability")
        .and_then(|v| v.as_str())
        .unwrap_or("Generic Task");
    
    let priority_str = payload.get("priority")
        .and_then(|v| v.as_str())
        .unwrap_or("Medium");
    
    let priority = match priority_str.to_lowercase().as_str() {
        "low" => Priority::Low,
        "medium" => Priority::Medium,
        "high" => Priority::High,
        "critical" => Priority::Critical,
        _ => Priority::Medium,
    };
    
    // מצא את ה-capability
    let capability = engine.agent_state.capabilities.iter()
        .find(|c| c.name == capability_name)
        .ok_or_else(|| {
            (StatusCode::BAD_REQUEST, Json(json!({
                "error": format!("Capability '{}' not found", capability_name),
                "available_capabilities": engine.agent_state.capabilities.iter()
                    .map(|c| &c.name)
                    .collect::<Vec<_>>()
            })))
        })?;
    
    let task = AgentTask {
        id: Uuid::new_v4(),
        name: name.to_string(),
        description: description.to_string(),
        capability_id: capability.id,
        status: TaskStatus::Pending,
        priority,
        assigned_to: None,
        deadline: None,
        dependencies: Vec::new(),
        progress: 0.0,
        result: None,
    };
    
    let task_id = engine.enqueue_task(task);
    
    Json(json!({
        "status": "created",
        "task_id": task_id,
        "task_name": name,
        "capability": capability_name,
        "priority": format!("{:?}", priority),
        "queue_position": engine.task_queue.len(),
    }))
}

/// 📋 Get task queue
async fn get_task_queue(
    State(state): State<OrchestrationAppState>,
) -> impl IntoResponse {
    let engine = state.engine.read().await;
    
    let tasks: Vec<serde_json::Value> = engine.task_queue.iter()
        .enumerate()
        .map(|(index, task)| {
            json!({
                "position": index + 1,
                "id": task.id,
                "name": task.name,
                "description": task.description,
                "priority": format!("{:?}", task.priority),
                "progress": task.progress,
            })
        })
        .collect();
    
    Json(json!({
        "tasks": tasks,
        "count": tasks.len(),
        "estimated_wait_time": tasks.len() * 30, // 30 שניות למשימה בממוצע
    }))
}

/// 🔄 Get active tasks
async fn get_active_tasks(
    State(state): State<OrchestrationAppState>,
) -> impl IntoResponse {
    let engine = state.engine.read().await;
    
    let tasks: Vec<serde_json::Value> = engine.active_tasks.values()
        .map(|active_task| {
            let duration = chrono::Utc::now().signed_duration_since(active_task.start_time);
            
            json!({
                "id": active_task.task.id,
                "name": active_task.task.name,
                "start_time": active_task.start_time.to_rfc3339(),
                "duration_seconds": duration.num_seconds(),
                "progress": active_task.progress,
                "status": format!("{:?}", active_task.status),
                "heartbeat_count": active_task.heartbeat_count,
                "last_heartbeat": active_task.last_heartbeat.to_rfc3339(),
            })
        })
        .collect();
    
    Json(json!({
        "tasks": tasks,
        "count": tasks.len(),
    }))
}

/// ✅ Get completed tasks
async fn get_completed_tasks(
    State(state): State<OrchestrationAppState>,
) -> impl IntoResponse {
    let engine = state.engine.read().await;
    
    let tasks: Vec<serde_json::Value> = engine.completed_tasks.iter()
        .rev() // הראה חדשות קודם
        .take(50) // הגבל ל-50 האחרונות
        .map(|completed_task| {
            let duration = completed_task.end_time.signed_duration_since(completed_task.start_time);
            
            json!({
                "id": completed_task.task.id,
                "name": completed_task.task.name,
                "start_time": completed_task.start_time.to_rfc3339(),
                "end_time": completed_task.end_time.to_rfc3339(),
                "duration_seconds": duration.num_seconds(),
                "success": completed_task.result.success,
                "execution_time_seconds": completed_task.result.execution_time_seconds,
                "learning_applied": completed_task.learning_applied,
            })
        })
        .collect();
    
    Json(json!({
        "tasks": tasks,
        "count": tasks.len(),
        "total_completed": engine.completed_tasks.len(),
    }))
}

/// 👁️ Get specific task
async fn get_task(
    State(state): State<OrchestrationAppState>,
    Path(task_id): Path<Uuid>,
) -> impl IntoResponse {
    let engine = state.engine.read().await;
    
    // חפש בתור
    if let Some(task) = engine.task_queue.iter().find(|t| t.id == task_id) {
        return Json(json!({
            "found_in": "queue",
            "task": {
                "id": task.id,
                "name": task.name,
                "description": task.description,
                "status": format!("{:?}", task.status),
                "priority": format!("{:?}", task.priority),
                "progress": task.progress,
                "queue_position": engine.task_queue.iter()
                    .position(|t| t.id == task_id)
                    .map(|pos| pos + 1),
            }
        }));
    }
    
    // חפש בפעילות
    if let Some(active_task) = engine.active_tasks.get(&task_id) {
        let duration = chrono::Utc::now().signed_duration_since(active_task.start_time);
        
        return Json(json!({
            "found_in": "active",
            "task": {
                "id": active_task.task.id,
                "name": active_task.task.name,
                "start_time": active_task.start_time.to_rfc3339(),
                "duration_seconds": duration.num_seconds(),
                "progress": active_task.progress,
                "status": format!("{:?}", active_task.status),
                "heartbeat_count": active_task.heartbeat_count,
                "last_heartbeat": active_task.last_heartbeat.to_rfc3339(),
            }
        }));
    }
    
    // חפש בהושלמו
    if let Some(completed_task) = engine.completed_tasks.iter().find(|t| t.task.id == task_id) {
        let duration = completed_task.end_time.signed_duration_since(completed_task.start_time);
        
        return Json(json!({
            "found_in": "completed",
            "task": {
                "id": completed_task.task.id,
                "name": completed_task.task.name,
                "start_time": completed_task.start_time.to_rfc3339(),
                "end_time": completed_task.end_time.to_rfc3339(),
                "duration_seconds": duration.num_seconds(),
                "success": completed_task.result.success,
                "execution_time_seconds": completed_task.result.execution_time_seconds