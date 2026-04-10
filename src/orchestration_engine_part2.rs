        let total_tasks = self.performance_tracker.total_tasks;
        let successful_tasks = self.performance_tracker.successful_tasks;
        let failed_tasks = self.performance_tracker.failed_tasks;
        
        let success_rate = if total_tasks > 0 {
            successful_tasks as f64 / total_tasks as f64
        } else {
            0.0
        };
        
        self.heartbeat_manager.metrics_snapshot = MetricsSnapshot {
            timestamp: Utc::now(),
            tasks_processed: total_tasks,
            tasks_succeeded: successful_tasks,
            tasks_failed: failed_tasks,
            average_processing_time: if total_tasks > 0 {
                self.performance_tracker.total_processing_time / total_tasks as f64
            } else {
                0.0
            },
            memory_usage_mb: self.get_memory_usage().await,
            cpu_percent: self.get_cpu_usage().await,
        };
        
        // עדכן health status
        self.heartbeat_manager.health_status.error_rate = if total_tasks > 0 {
            failed_tasks as f64 / total_tasks as f64
        } else {
            0.0
        };
        
        // קבע overall health
        self.heartbeat_manager.health_status.overall = 
            self.determine_overall_health().await;
    }
    
    /// קבל שימוש זיכרון
    async fn get_memory_usage(&self) -> f64 {
        // בפועל, זה היה קורא מ-system metrics
        // כרגע - ערך דמה
        128.0 // MB
    }
    
    /// קבל שימוש CPU
    async fn get_cpu_usage(&self) -> f64 {
        // בפועל, זה היה קורא מ-system metrics
        // כרגע - ערך דמה
        15.0 // אחוז
    }
    
    /// קבע overall health
    async fn determine_overall_health(&self) -> HealthLevel {
        let error_rate = self.heartbeat_manager.health_status.error_rate;
        let cpu_usage = self.heartbeat_manager.health_status.cpu_usage;
        let memory_usage = self.heartbeat_manager.health_status.memory_usage;
        
        if error_rate > 0.3 || cpu_usage > 90.0 || memory_usage > 90.0 {
            HealthLevel::Critical
        } else if error_rate > 0.1 || cpu_usage > 70.0 || memory_usage > 70.0 {
            HealthLevel::Warning
        } else {
            HealthLevel::Healthy
        }
    }
    
    /// בדוק בריאות תתי-מערכות
    async fn check_subsystem_health(&mut self) {
        let mut subsystems = HashMap::new();
        
        // בדוק Ollama
        subsystems.insert("ollama".to_string(), self.check_ollama_health().await);
        
        // בדוק Calclaw
        subsystems.insert("calclaw".to_string(), self.check_calclaw_health().await);
        
        // בדוק Cron Manager
        subsystems.insert("cron_manager".to_string(), self.check_cron_manager_health().await);
        
        // בדוק Database
        subsystems.insert("database".to_string(), self.check_database_health().await);
        
        self.heartbeat_manager.subsystem_status = subsystems;
    }
    
    /// בדוק בריאות Ollama
    async fn check_ollama_health(&self) -> SubsystemStatus {
        // בפועל, זה היה בודק חיבור ל-Ollama API
        SubsystemStatus {
            name: "Ollama".to_string(),
            status: SubsystemHealth::Running,
            last_check: Utc::now(),
            error_message: None,
        }
    }
    
    /// בדוק בריאות Calclaw
    async fn check_calclaw_health(&self) -> SubsystemStatus {
        // בפועל, זה היה בודק אם השרת רץ
        SubsystemStatus {
            name: "Calclaw".to_string(),
            status: SubsystemHealth::Running,
            last_check: Utc::now(),
            error_message: None,
        }
    }
    
    /// בדוק בריאות Cron Manager
    async fn check_cron_manager_health(&self) -> SubsystemStatus {
        // בפועל, זה היה בודק את מערכת ה-cron
        SubsystemStatus {
            name: "Cron Manager".to_string(),
            status: SubsystemHealth::Running,
            last_check: Utc::now(),
            error_message: None,
        }
    }
    
    /// בדוק בריאות Database
    async fn check_database_health(&self) -> SubsystemStatus {
        // בפועל, זה היה בודק חיבור למסד נתונים
        SubsystemStatus {
            name: "Database".to_string(),
            status: SubsystemHealth::Running,
            last_check: Utc::now(),
            error_message: None,
        }
    }
    
    /// בדוק אם יש עדכונים
    async fn check_for_updates(&mut self) {
        // בפועל, זה היה קורא ל-GitHub API
        // כרגע - דמה
        self.update_integration.last_check = Utc::now();
        self.update_integration.update_available = false;
    }
    
    /// סנכרן עם cron jobs
    async fn sync_cron_jobs(&mut self) {
        // בפועל, זה היה קורא ל-cron manager API
        // כרגע - דמה
        self.cron_integration.last_sync = Utc::now();
    }
    
    /// עדכן סטטוס סוכן
    async fn update_agent_status(&mut self) {
        self.agent_state.last_activity = Utc::now();
        
        // עדכן סטטוס לפי מצב נוכחי
        if !self.active_tasks.is_empty() {
            self.agent_state.status = AgentStatus::Processing;
        } else if !self.task_queue.is_empty() {
            self.agent_state.status = AgentStatus::Processing;
        } else {
            self.agent_state.status = AgentStatus::Idle;
        }
    }
    
    /// הוסף task לתור
    pub fn enqueue_task(&mut self, task: AgentTask) -> Uuid {
        let task_id = task.id;
        self.task_queue.push_back(task);
        task_id
    }
    
    /// הפעל task מהתור
    pub async fn execute_next_task(&mut self) -> Option<TaskResult> {
        if let Some(task) = self.task_queue.pop_front() {
            let task_id = task.id;
            let start_time = Utc::now();
            
            // הוסף ל-active tasks
            self.active_tasks.insert(task_id, ActiveTask {
                task: task.clone(),
                start_time,
                progress: 0.0,
                status: TaskExecutionStatus::Starting,
                heartbeat_count: 0,
                last_heartbeat: Utc::now(),
            });
            
            // עדכן סטטוס
            self.agent_state.status = AgentStatus::Processing;
            
            // הפעל את ה-task
            let result = self.execute_task_internal(&task).await;
            let end_time = Utc::now();
            
            // הסר מ-active tasks
            self.active_tasks.remove(&task_id);
            
            // הוסף ל-completed tasks
            let completed_task = CompletedTask {
                task,
                start_time,
                end_time,
                result: result.clone(),
                resources_used: ResourcesUsed {
                    cpu_seconds: 0.0,
                    memory_mb: 0.0,
                    network_mb: 0.0,
                    api_calls: 0,
                },
                learning_applied: false,
            };
            
            self.completed_tasks.push(completed_task);
            
            // עדכן performance tracker
            self.performance_tracker.total_tasks += 1;
            if result.success {
                self.performance_tracker.successful_tasks += 1;
            } else {
                self.performance_tracker.failed_tasks += 1;
            }
            
            self.performance_tracker.total_processing_time += 
                end_time.signed_duration_since(start_time).num_seconds() as f64;
            
            // עדכן historical data
            self.performance_tracker.historical_data.push(PerformancePoint {
                timestamp: Utc::now(),
                tasks_completed: self.performance_tracker.total_tasks,
                success_rate: if self.performance_tracker.total_tasks > 0 {
                    self.performance_tracker.successful_tasks as f64 / 
                    self.performance_tracker.total_tasks as f64
                } else {
                    0.0
                },
                efficiency: self.performance_tracker.efficiency_score,
                learning_rate: self.performance_tracker.learning_score,
            });
            
            // שמור רק 100 נקודות היסטוריות
            if self.performance_tracker.historical_data.len() > 100 {
                self.performance_tracker.historical_data.remove(0);
            }
            
            Some(result)
        } else {
            None
        }
    }
    
    /// הפעל task פנימי
    async fn execute_task_internal(&self, task: &AgentTask) -> TaskResult {
        // מצא את ה-capability
        let capability = self.agent_state.capabilities.iter()
            .find(|c| c.id == task.capability_id);
        
        match capability {
            Some(cap) => {
                // הפעל לפי סוג ה-capability
                match cap.category {
                    CapabilityCategory::DataAnalysis => {
                        self.execute_data_analysis_task(task).await
                    }
                    CapabilityCategory::ProcessAutomation => {
                        self.execute_process_automation_task(task).await
                    }
                    _ => {
                        self.execute_generic_task(task).await
                    }
                }
            }
            None => {
                TaskResult {
                    success: false,
                    output: serde_json::json!({"error": "Capability not found"}),
                    error_message: Some("Capability not found".to_string()),
                    execution_time_seconds: 0,
                    resources_used: ResourcesUsed {
                        cpu_seconds: 0.0,
                        memory_mb: 0.0,
                        network_mb: 0.0,
                        api_calls: 0,
                    },
                }
            }
        }
    }
    
    /// הפעל data analysis task
    async fn execute_data_analysis_task(&self, task: &AgentTask) -> TaskResult {
        // דמה - בפועל זה היה מנתח נתונים
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        
        TaskResult {
            success: true,
            output: serde_json::json!({
                "analysis_type": "data_analysis",
                "task": task.name,
                "insights": ["Insight 1", "Insight 2"],
                "recommendations": ["Recommendation 1"],
            }),
            error_message: None,
            execution_time_seconds: 2,
            resources_used: ResourcesUsed {
                cpu_seconds: 1.0,
                memory_mb: 256.0,
                network_mb: 5.0,
                api_calls: 3,
            },
        }
    }
    
    /// הפעל process automation task
    async fn execute_process_automation_task(&self, task: &AgentTask) -> TaskResult {
        // דמה - בפועל זה היה מפעיל תהליכים
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        
        TaskResult {
            success: true,
            output: serde_json::json!({
                "automation_type": "process_automation",
                "task": task.name,
                "processes_executed": 2,
                "time_saved_minutes": 30,
            }),
            error_message: None,
            execution_time_seconds: 3,
            resources_used: ResourcesUsed {
                cpu_seconds: 2.0,
                memory_mb: 512.0,
                network_mb: 10.0,
                api_calls: 5,
            },
        }
    }
    
    /// הפעל generic task
    async fn execute_generic_task(&self, task: &AgentTask) -> TaskResult {
        // דמה
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        
        TaskResult {
            success: true,
            output: serde_json::json!({
                "task": task.name,
                "status": "completed",
            }),
            error_message: None,
            execution_time_seconds: 1,
            resources_used: ResourcesUsed {
                cpu_seconds: 0.5,
                memory_mb: 128.0,
                network_mb: 2.0,
                api_calls: 1,
            },
        }
    }
    
    /// קבל סטטוס מנוע
    pub fn get_engine_status(&self) -> serde_json::Value {
        serde_json::json!({
            "agent_state": {
                "status": format!("{:?}", self.agent_state.status),
                "capabilities": self.agent_state.capabilities.len(),
                "last_activity": self.agent_state.last_activity.to_rfc3339(),
            },
            "tasks": {
                "queued": self.task_queue.len(),
                "active": self.active_tasks.len(),
                "completed": self.completed_tasks.len(),
            },
            "heartbeat": {
                "count": self.heartbeat_manager.heartbeat_count,
                "last_heartbeat": self.heartbeat_manager.last_heartbeat.to_rfc3339(),
                "health": format!("{:?}", self.heartbeat_manager.health_status.overall),
            },
            "performance": {
                "total_tasks": self.performance_tracker.total_tasks,
                "success_rate": if self.performance_tracker.total_tasks > 0 {
                    self.performance_tracker.successful_tasks as f64 / 
                    self.performance_tracker.total_tasks as f64
                } else {
                    0.0
                },
                "efficiency": self.performance_tracker.efficiency_score,
                "learning_score": self.performance_tracker.learning_score,
            },
            "timestamp": Utc::now().to_rfc3339(),
        })
    }
    
    /// קבל המלצות לשיפור
    pub fn get_improvement_recommendations(&self) -> Vec<ImprovementRecommendation> {
        let mut recommendations = Vec::new();
        
        // ניתוח performance
        let success_rate = if self.performance_tracker.total_tasks > 0 {
            self.performance_tracker.successful_tasks as f64 / 
            self.performance_tracker.total_tasks as f64
        } else {
            0.0
        };
        
        if success_rate < 0.8 {
            recommendations.push(ImprovementRecommendation {
                id: Uuid::new_v4(),
                title: "Improve Task Success Rate".to_string(),
                description: format!("Current success rate is {:.1}%. Consider adding more error handling and retry logic.", success_rate * 100.0),
                priority: if success_rate < 0.5 { Priority::High } else { Priority::Medium },
                action: "Review failed tasks and implement better error handling".to_string(),
                estimated_impact: 0.3,
            });
        }
        
        if self.performance_tracker.efficiency_score < 0.7 {
            recommendations.push(ImprovementRecommendation {
                id: Uuid::new_v4(),
                title: "Improve Efficiency".to_string(),
                description: format!("Current efficiency score is {:.2}. Consider optimizing task execution and resource usage.", self.performance_tracker.efficiency_score),
                priority: Priority::Medium,
                action: "Analyze resource usage and optimize task scheduling".to_string(),
                estimated_impact: 0.2,
            });
        }
        
        if self.performance_tracker.learning_score < 0.5 {
            recommendations.push(ImprovementRecommendation {
                id: Uuid::new_v4(),
                title: "Enhance Learning Capabilities".to_string(),
                description: format!("Current learning score is {:.2}. The system could learn more from past decisions.", self.performance_tracker.learning_score),
                priority: Priority::Low,
                action: "Implement more sophisticated learning algorithms".to_string(),
                estimated_impact: 0.4,
            });
        }
        
        // בדוק אם יש הרבה tasks בתור
        if self.task_queue.len() > 10 {
            recommendations.push(ImprovementRecommendation {
                id: Uuid::new_v4(),
                title: "Reduce Task Queue Backlog".to_string(),
                description: format!("There are {} tasks waiting in the queue. Consider increasing processing capacity or prioritizing tasks.", self.task_queue.len()),
                priority: Priority::High,
                action: "Review task priorities and allocate more resources".to_string(),
                estimated_impact: 0.25,
            });
        }
        
        recommendations
    }
}

/// 🎯 Improvement Recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImprovementRecommendation {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub priority: Priority,
    pub action: String,
    pub estimated_impact: f64, // 0.0 to 1.0
}

/// 🚀 Orchestration Engine API
pub mod orchestration_api {
    use super::*;
    use axum::{
        extract::{Path, State},
        http::StatusCode,
        response::IntoResponse,
        routing::{get, post},
        Json, Router,
    };
    use std::sync::Arc;
    use tokio::sync::RwLock;
    
    /// App State for orchestration engine
    pub struct OrchestrationAppState {
        pub engine: Arc<RwLock<OrchestrationEngine>>,
    }
    
    /// Create orchestration API router
    pub fn create_orchestration_api() -> Router<OrchestrationAppState> {
        Router::new()
            .route("/api/orchestration/status", get(get_engine_status))
            .route("/api/orche