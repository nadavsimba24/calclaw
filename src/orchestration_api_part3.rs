                "last_applied": pattern.last_applied.to_rfc3339(),
            })
        })
        .collect();
    
    Json(json!({
        "patterns": patterns,
        "count": patterns.len(),
        "by_type": {
            "anomaly": patterns.iter().filter(|p| p["pattern_type"] == "Anomaly").count(),
            "trend": patterns.iter().filter(|p| p["pattern_type"] == "Trend").count(),
            "seasonality": patterns.iter().filter(|p| p["pattern_type"] == "Seasonality").count(),
            "correlation": patterns.iter().filter(|p| p["pattern_type"] == "Correlation").count(),
            "cluster": patterns.iter().filter(|p| p["pattern_type"] == "Cluster").count(),
            "sequence": patterns.iter().filter(|p| p["pattern_type"] == "Sequence").count(),
        }
    }))
}

/// 🎯 Get improvement recommendations
async fn get_recommendations(
    State(state): State<OrchestrationAppState>,
) -> impl IntoResponse {
    let engine = state.engine.read().await;
    let recommendations = engine.get_improvement_recommendations();
    
    let recommendations_json: Vec<serde_json::Value> = recommendations.iter()
        .map(|rec| {
            json!({
                "id": rec.id,
                "title": rec.title,
                "description": rec.description,
                "priority": format!("{:?}", rec.priority),
                "action": rec.action,
                "estimated_impact": rec.estimated_impact,
            })
        })
        .collect();
    
    Json(json!({
        "recommendations": recommendations_json,
        "count": recommendations_json.len(),
        "by_priority": {
            "high": recommendations.iter().filter(|r| matches!(r.priority, Priority::High)).count(),
            "medium": recommendations.iter().filter(|r| matches!(r.priority, Priority::Medium)).count(),
            "low": recommendations.iter().filter(|r| matches!(r.priority, Priority::Low)).count(),
        }
    }))
}

/// ✅ Apply a recommendation
async fn apply_recommendation(
    State(state): State<OrchestrationAppState>,
    Path(recommendation_id): Path<Uuid>,
) -> impl IntoResponse {
    let engine = state.engine.read().await;
    let recommendations = engine.get_improvement_recommendations();
    
    let recommendation = recommendations.iter()
        .find(|r| r.id == recommendation_id);
    
    match recommendation {
        Some(rec) => {
            // בפועל, זה היה מיישם את ההמלצה
            // כרגע - רק מדמה יישום
            
            Json(json!({
                "status": "applied",
                "recommendation_id": recommendation_id,
                "title": rec.title,
                "action": rec.action,
                "estimated_impact": rec.estimated_impact,
                "message": "Recommendation applied successfully (simulated)",
            }))
        }
        None => {
            (StatusCode::NOT_FOUND, Json(json!({
                "error": "Recommendation not found",
                "suggestion": "Refresh recommendations list"
            })))
        }
    }
}

/// 🔧 Get subsystem status
async fn get_subsystem_status(
    State(state): State<OrchestrationAppState>,
) -> impl IntoResponse {
    let engine = state.engine.read().await;
    
    let subsystems: Vec<serde_json::Value> = engine.heartbeat_manager.subsystem_status.values()
        .map(|status| {
            json!({
                "name": status.name,
                "status": format!("{:?}", status.status),
                "last_check": status.last_check.to_rfc3339(),
                "error_message": status.error_message,
            })
        })
        .collect();
    
    Json(json!({
        "subsystems": subsystems,
        "count": subsystems.len(),
        "overall_health": format!("{:?}", engine.heartbeat_manager.health_status.overall),
    }))
}

/// 🩺 Get specific subsystem health
async fn get_subsystem_health(
    State(state): State<OrchestrationAppState>,
    Path(subsystem_name): Path<String>,
) -> impl IntoResponse {
    let engine = state.engine.read().await;
    
    match engine.heartbeat_manager.subsystem_status.get(&subsystem_name) {
        Some(status) => {
            Json(json!({
                "name": status.name,
                "status": format!("{:?}", status.status),
                "last_check": status.last_check.to_rfc3339(),
                "error_message": status.error_message,
                "healthy": matches!(status.status, SubsystemHealth::Running),
            }))
        }
        None => {
            (StatusCode::NOT_FOUND, Json(json!({
                "error": format!("Subsystem '{}' not found", subsystem_name),
                "available_subsystems": engine.heartbeat_manager.subsystem_status.keys()
                    .cloned()
                    .collect::<Vec<_>>()
            })))
        }
    }
}

/// 🔄 Restart a subsystem
async fn restart_subsystem(
    State(state): State<OrchestrationAppState>,
    Path(subsystem_name): Path<String>,
) -> impl IntoResponse {
    // בפועל, זה היה מפעיל מחדש את התת-מערכת
    // כרגע - רק מדמה
    
    Json(json!({
        "status": "restarted",
        "subsystem": subsystem_name,
        "message": "Subsystem restart initiated (simulated)",
        "timestamp": chrono::Utc::now().to_rfc3339(),
    }))
}

/// ⏰ Get cron integration status
async fn get_cron_integration(
    State(state): State<OrchestrationAppState>,
) -> impl IntoResponse {
    let engine = state.engine.read().await;
    
    let cron_jobs: Vec<serde_json::Value> = engine.cron_integration.cron_jobs.iter()
        .map(|job| {
            json!({
                "id": job.id,
                "name": job.name,
                "schedule": job.schedule,
                "description": job.description,
                "enabled": job.enabled,
                "last_run": job.last_run.map(|t| t.to_rfc3339()),
                "next_run": job.next_run.map(|t| t.to_rfc3339()),
                "success_count": job.success_count,
                "failure_count": job.failure_count,
            })
        })
        .collect();
    
    Json(json!({
        "cron_jobs": cron_jobs,
        "count": cron_jobs.len(),
        "last_sync": engine.cron_integration.last_sync.to_rfc3339(),
        "sync_interval_seconds": engine.cron_integration.sync_interval_seconds,
        "enabled_jobs": cron_jobs.iter().filter(|j| j["enabled"] == true).count(),
    }))
}

/// 🔄 Sync cron jobs
async fn sync_cron_jobs(
    State(state): State<OrchestrationAppState>,
) -> impl IntoResponse {
    let mut engine = state.engine.write().await;
    
    // בפועל, זה היה סינכרון עם מערכת ה-cron
    // כרגע - רק מדמה
    
    engine.cron_integration.last_sync = chrono::Utc::now();
    
    Json(json!({
        "status": "synced",
        "last_sync": engine.cron_integration.last_sync.to_rfc3339(),
        "message": "Cron jobs synchronized (simulated)",
    }))
}

/// 🔄 Get update status
async fn get_update_status(
    State(state): State<OrchestrationAppState>,
) -> impl IntoResponse {
    let engine = state.engine.read().await;
    
    Json(json!({
        "current_version": engine.update_integration.current_version,
        "last_check": engine.update_integration.last_check.to_rfc3339(),
        "update_available": engine.update_integration.update_available,
        "update_url": engine.update_integration.update_url,
        "changelog": engine.update_integration.changelog,
    }))
}

/// 🔍 Check for updates
async fn check_for_updates(
    State(state): State<OrchestrationAppState>,
) -> impl IntoResponse {
    let mut engine = state.engine.write().await;
    
    // בפועל, זה היה בודק ב-GitHub
    // כרגע - רק מדמה
    
    engine.update_integration.last_check = chrono::Utc::now();
    engine.update_integration.update_available = false;
    
    Json(json!({
        "status": "checked",
        "last_check": engine.update_integration.last_check.to_rfc3339(),
        "update_available": false,
        "message": "No updates available (simulated check)",
    }))
}

/// 🚀 Start the engine
async fn start_engine(
    State(state): State<OrchestrationAppState>,
) -> impl IntoResponse {
    let mut engine = state.engine.write().await;
    
    engine.agent_state.status = AgentStatus::Idle;
    engine.performance_tracker.start_time = chrono::Utc::now();
    
    Json(json!({
        "status": "started",
        "agent_status": format!("{:?}", engine.agent_state.status),
        "start_time": engine.performance_tracker.start_time.to_rfc3339(),
        "message": "Orchestration engine started",
    }))
}

/// ⏹️ Stop the engine
async fn stop_engine(
    State(state): State<OrchestrationAppState>,
) -> impl IntoResponse {
    let mut engine = state.engine.write().await;
    
    engine.agent_state.status = AgentStatus::Error("Stopped by user".to_string());
    
    // נקה את התור (בפועל, אולי תרצה לשמור את המשימות)
    engine.task_queue.clear();
    
    Json(json!({
        "status": "stopped",
        "agent_status": format!("{:?}", engine.agent_state.status),
        "tasks_cancelled": engine.task_queue.len(),
        "message": "Orchestration engine stopped, task queue cleared",
    }))
}

#### ⏸️ Pause the engine
async fn pause_engine(
    State(state): State<OrchestrationAppState>,
) -> impl IntoResponse {
    let mut engine = state.engine.write().await;
    
    let previous_status = std::mem::replace(&mut engine.agent_state.status, AgentStatus::WaitingForInput);
    
    Json(json!({
        "status": "paused",
        "previous_status": format!("{:?}", previous_status),
        "current_status": format!("{:?}", engine.agent_state.status),
        "message": "Orchestration engine paused",
    }))
}

#### ▶️ Resume the engine
async fn resume_engine(
    State(state): State<OrchestrationAppState>,
) -> impl IntoResponse {
    let mut engine = state.engine.write().await;
    
    engine.agent_state.status = if engine.task_queue.is_empty() && engine.active_tasks.is_empty() {
        AgentStatus::Idle
    } else {
        AgentStatus::Processing
    };
    
    Json(json!({
        "status": "resumed",
        "agent_status": format!("{:?}", engine.agent_state.status),
        "queued_tasks": engine.task_queue.len(),
        "active_tasks": engine.active_tasks.len(),
        "message": "Orchestration engine resumed",
    }))
}

#### 🔄 Reset the engine
async fn reset_engine(
    State(state): State<OrchestrationAppState>,
) -> impl IntoResponse {
    let mut engine = state.engine.write().await;
    
    // שמור את האונטולוגיה
    let ontology = engine.agent_state.capabilities.clone();
    
    // אתחל מחדש
    *engine = OrchestrationEngine::new(OrganizationOntology {
        profile: OrganizationProfile {
            id: uuid::Uuid::new_v4(),
            name: "Reset Organization".to_string(),
            description: "Organization after engine reset".to_string(),
            industry: "Technology".to_string(),
            size: OrganizationSize::Startup(1),
            location: "".to_string(),
            timezone: "UTC".to_string(),
            language: "English".to_string(),
            website: None,
            founded_year: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        },
        departments: Vec::new(),
        processes: Vec::new(),
        data_entities: Vec::new(),
        systems: Vec::new(),
        integrations: Vec::new(),
        goals: Vec::new(),
        metrics: Vec::new(),
    });
    
    // שחזר את ה-capabilities
    engine.agent_state.capabilities = ontology;
    
    Json(json!({
        "status": "reset",
        "agent_status": format!("{:?}", engine.agent_state.status),
        "capabilities_restored": engine.agent_state.capabilities.len(),
        "message": "Orchestration engine reset to initial state",
    }))
}

#### 🚀 Main function to run orchestration server
pub async fn run_orchestration_server(port: u16) {
    println!("🚀 Starting Orchestration Engine Server...");
    println!("   Port: {}", port);
    println!("   API: http://localhost:{}/api/orchestration", port);
    println!("");
    
    // צור את המנוע
    let engine = OrchestrationEngine::new(OrganizationOntology {
        profile: OrganizationProfile {
            id: uuid::Uuid::new_v4(),
            name: "Default Organization".to_string(),
            description: "Organization for orchestration engine".to_string(),
            industry: "Technology".to_string(),
            size: OrganizationSize::Startup(10),
            location: "".to_string(),
            timezone: "UTC".to_string(),
            language: "English".to_string(),
            website: None,
            founded_year: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        },
        departments: Vec::new(),
        processes: Vec::new(),
        data_entities: Vec::new(),
        systems: Vec::new(),
        integrations: Vec::new(),
        goals: Vec::new(),
        metrics: Vec::new(),
    });
    
    // צור את ה-app state
    let app_state = OrchestrationAppState {
        engine: Arc::new(RwLock::new(engine)),
    };
    
    // צור את ה-API router
    let app = Router::new()
        .nest("/api/orchestration", create_orchestration_api())
        .with_state(app_state);
    
    // הפעל background heartbeat
    let engine_clone = app_state.engine.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(30));
        
        loop {
            interval.tick().await;
            let mut engine = engine_clone.write().await;
            engine.heartbeat().await;
        }
    });
    
    println!("✅ Orchestration Engine Ready!");
    println!("");
    println!("📊 Available Endpoints:");
    println!("   • Status: http://localhost:{}/api/orchestration/status", port);
    println!("   • Health: http://localhost:{}/api/orchestration/health", port);
    println!("   • Tasks: http://localhost:{}/api/orchestration/tasks", port);
    println!("   • Agent: http://localhost:{}/api/orchestration/agent/status", port);
    println!("   • Performance: http://localhost:{}/api/orchestration/performance", port);
    println!("");
    println!("💓 Heartbeat running every 30 seconds");
    println!("🤖 Agent is ready to orchestrate tasks!");
    
    // הפעל את השרת
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    
    axum::serve(listener, app).await.unwrap();
}