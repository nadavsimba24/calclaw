// 🚀 Orchestration Engine עם State ו-Heartbeat
// מבוסס על מנועי Calclaw הקיימים עם שדרוגים לאונטולוגיה

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::organization_ontology::*;

/// 🎯 מנוע אורקסטרציה משודרג
/// משלב את יכולות ה-cron manager הקיים עם state management ו-heartbeat
pub struct OrchestrationEngine {
    // 🧠 State Management
    pub agent_state: AgentState,
    pub task_queue: VecDeque<AgentTask>,
    pub active_tasks: HashMap<Uuid, ActiveTask>,
    pub completed_tasks: Vec<CompletedTask>,
    
    // 💓 Heartbeat System
    pub heartbeat_manager: HeartbeatManager,
    
    // 📊 Performance & Learning
    pub performance_tracker: PerformanceTracker,
    pub learning_loop: LearningLoop,
    
    // 🔗 Integration עם מערכות קיימות
    pub cron_integration: CronIntegration,
    pub update_integration: UpdateIntegration,
}

/// 🧠 Agent State
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentState {
    pub agent_id: Uuid,
    pub status: AgentStatus,
    pub capabilities: Vec<AgentCapability>,
    pub current_context: Context,
    pub memory: AgentMemory,
    pub last_activity: DateTime<Utc>,
}

/// 🟢 Agent Status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentStatus {
    Idle,
    Processing,
    WaitingForInput,
    Learning,
    Error(String),
}

/// 📋 Context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    pub current_goal: Option<Uuid>,
    pub active_department: Option<Uuid>,
    pub relevant_data_entities: Vec<Uuid>,
    pub constraints: Vec<Constraint>,
    pub priorities: Vec<Priority>,
}

/// 🧠 Agent Memory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMemory {
    pub recent_decisions: VecDeque<Decision>,
    pub learned_patterns: Vec<Pattern>,
    pub successful_actions: Vec<SuccessfulAction>,
    pub failed_actions: Vec<FailedAction>,
}

/// 💓 Heartbeat Manager
/// מבוסס על מערכת ה-heartbeat הקיימת ב-Calclaw
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartbeatManager {
    pub heartbeat_interval_seconds: u64,
    pub last_heartbeat: DateTime<Utc>,
    pub heartbeat_count: u64,
    pub health_status: HealthStatus,
    pub subsystem_status: HashMap<String, SubsystemStatus>,
    pub metrics_snapshot: MetricsSnapshot,
}

/// 🩺 Health Status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub overall: HealthLevel,
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub disk_usage: f64,
    pub network_latency: f64,
    pub error_rate: f64,
}

/// 📈 Health Level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthLevel {
    Healthy,
    Warning,
    Critical,
    Unknown,
}

/// 🔧 Subsystem Status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubsystemStatus {
    pub name: String,
    pub status: SubsystemHealth,
    pub last_check: DateTime<Utc>,
    pub error_message: Option<String>,
}

/// 📊 Subsystem Health
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SubsystemHealth {
    Running,
    Starting,
    Stopping,
    Stopped,
    Error,
    Unknown,
}

/// 📈 Metrics Snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsSnapshot {
    pub timestamp: DateTime<Utc>,
    pub tasks_processed: u64,
    pub tasks_succeeded: u64,
    pub tasks_failed: u64,
    pub average_processing_time: f64,
    pub memory_usage_mb: f64,
    pub cpu_percent: f64,
}

/// 📊 Performance Tracker
/// מבוסס על מערכת ה-performance הקיימת
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceTracker {
    pub start_time: DateTime<Utc>,
    pub total_tasks: u64,
    pub successful_tasks: u64,
    pub failed_tasks: u64,
    pub total_processing_time: f64,
    pub efficiency_score: f64,
    pub learning_score: f64,
    pub historical_data: Vec<PerformancePoint>,
}

/// 📈 Performance Point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformancePoint {
    pub timestamp: DateTime<Utc>,
    pub tasks_completed: u64,
    pub success_rate: f64,
    pub efficiency: f64,
    pub learning_rate: f64,
}

/// 🔄 Learning Loop
/// משלב למידה עם מערכת ה-context compaction הקיימת
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningLoop {
    pub decisions_made: u64,
    pub successful_decisions: u64,
    pub failed_decisions: u64,
    pub learned_patterns: Vec<LearnedPattern>,
    pub decision_history: Vec<DecisionRecord>,
    pub improvement_rate: f64,
}

/// 🧩 Learned Pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearnedPattern {
    pub pattern_id: Uuid,
    pub pattern_type: PatternType,
    pub confidence: f64,
    pub applications: u64,
    pub success_rate: f64,
    pub last_applied: DateTime<Utc>,
}

/// 📝 Decision Record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionRecord {
    pub decision_id: Uuid,
    pub context_hash: String,
    pub options_considered: Vec<DecisionOption>,
    pub chosen_option: Uuid,
    pub outcome: DecisionOutcome,
    pub timestamp: DateTime<Utc>,
    pub learning_applied: bool,
}

/// 🔗 Cron Integration
/// מתחבר למערכת ה-cron manager הקיימת
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CronIntegration {
    pub cron_jobs: Vec<CronJob>,
    pub last_sync: DateTime<Utc>,
    pub sync_interval_seconds: u64,
}

/// ⏰ Cron Job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CronJob {
    pub id: Uuid,
    pub name: String,
    pub schedule: String,
    pub command: String,
    pub description: String,
    pub enabled: bool,
    pub last_run: Option<DateTime<Utc>>,
    pub next_run: Option<DateTime<Utc>>,
    pub success_count: u64,
    pub failure_count: u64,
}

/// 🔄 Update Integration
/// מתחבר למערכת העדכונים הקיימת
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateIntegration {
    pub current_version: String,
    pub last_check: DateTime<Utc>,
    pub update_available: bool,
    pub update_url: Option<String>,
    pub changelog: Option<String>,
}

/// 🎯 Active Task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveTask {
    pub task: AgentTask,
    pub start_time: DateTime<Utc>,
    pub progress: f64,
    pub status: TaskExecutionStatus,
    pub heartbeat_count: u64,
    pub last_heartbeat: DateTime<Utc>,
}

/// 📊 Task Execution Status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskExecutionStatus {
    Starting,
    Running,
    Paused,
    WaitingForDependency,
    Completing,
}

/// ✅ Completed Task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletedTask {
    pub task: AgentTask,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub result: TaskResult,
    pub resources_used: ResourcesUsed,
    pub learning_applied: bool,
}

impl OrchestrationEngine {
    /// צור מנוע אורקסטרציה חדש
    pub fn new(ontology: OrganizationOntology) -> Self {
        let capabilities = Self::generate_capabilities(&ontology);
        
        Self {
            agent_state: AgentState {
                agent_id: Uuid::new_v4(),
                status: AgentStatus::Idle,
                capabilities,
                current_context: Context {
                    current_goal: None,
                    active_department: None,
                    relevant_data_entities: Vec::new(),
                    constraints: Vec::new(),
                    priorities: Vec::new(),
                },
                memory: AgentMemory {
                    recent_decisions: VecDeque::with_capacity(100),
                    learned_patterns: Vec::new(),
                    successful_actions: Vec::new(),
                    failed_actions: Vec::new(),
                },
                last_activity: Utc::now(),
            },
            task_queue: VecDeque::new(),
            active_tasks: HashMap::new(),
            completed_tasks: Vec::new(),
            
            heartbeat_manager: HeartbeatManager {
                heartbeat_interval_seconds: 30,
                last_heartbeat: Utc::now(),
                heartbeat_count: 0,
                health_status: HealthStatus {
                    overall: HealthLevel::Healthy,
                    cpu_usage: 0.0,
                    memory_usage: 0.0,
                    disk_usage: 0.0,
                    network_latency: 0.0,
                    error_rate: 0.0,
                },
                subsystem_status: HashMap::new(),
                metrics_snapshot: MetricsSnapshot {
                    timestamp: Utc::now(),
                    tasks_processed: 0,
                    tasks_succeeded: 0,
                    tasks_failed: 0,
                    average_processing_time: 0.0,
                    memory_usage_mb: 0.0,
                    cpu_percent: 0.0,
                },
            },
            
            performance_tracker: PerformanceTracker {
                start_time: Utc::now(),
                total_tasks: 0,
                successful_tasks: 0,
                failed_tasks: 0,
                total_processing_time: 0.0,
                efficiency_score: 1.0,
                learning_score: 0.0,
                historical_data: Vec::new(),
            },
            
            learning_loop: LearningLoop {
                decisions_made: 0,
                successful_decisions: 0,
                failed_decisions: 0,
                learned_patterns: Vec::new(),
                decision_history: Vec::new(),
                improvement_rate: 0.0,
            },
            
            cron_integration: CronIntegration {
                cron_jobs: Vec::new(),
                last_sync: Utc::now(),
                sync_interval_seconds: 300, // 5 דקות
            },
            
            update_integration: UpdateIntegration {
                current_version: env!("CARGO_PKG_VERSION").to_string(),
                last_check: Utc::now(),
                update_available: false,
                update_url: None,
                changelog: None,
            },
        }
    }
    
    /// צור יכולות מהאונטולוגיה
    fn generate_capabilities(ontology: &OrganizationOntology) -> Vec<AgentCapability> {
        // שימוש באותו לוגיקה כמו ב-SuperAgent
        let mut capabilities = Vec::new();
        
        if !ontology.data_entities.is_empty() {
            capabilities.push(AgentCapability {
                id: Uuid::new_v4(),
                name: "Data Analysis".to_string(),
                description: "Analyze organizational data for insights".to_string(),
                category: CapabilityCategory::DataAnalysis,
                required_data: ontology.data_entities.iter().map(|de| de.id).collect(),
                required_systems: Vec::new(),
                complexity: ComplexityLevel::Moderate,
                execution_time_estimate: 30,
            });
        }
        
        if !ontology.processes.is_empty() {
            capabilities.push(AgentCapability {
                id: Uuid::new_v4(),
                name: "Process Automation".to_string(),
                description: "Automate business processes".to_string(),
                category: CapabilityCategory::ProcessAutomation,
                required_data: Vec::new(),
                required_systems: ontology.systems.iter().map(|s| s.id).collect(),
                complexity: ComplexityLevel::Complex,
                execution_time_estimate: 60,
            });
        }
        
        capabilities.push(AgentCapability {
            id: Uuid::new_v4(),
            name: "Cron Job Management".to_string(),
            description: "Manage scheduled tasks".to_string(),
            category: CapabilityCategory::ProcessAutomation,
            required_data: Vec::new(),
            required_systems: Vec::new(),
            complexity: ComplexityLevel::Simple,
            execution_time_estimate: 10,
        });
        
        capabilities.push(AgentCapability {
            id: Uuid::new_v4(),
            name: "System Monitoring".to_string(),
            description: "Monitor system health and performance".to_string(),
            category: CapabilityCategory::DataAnalysis,
            required_data: Vec::new(),
            required_systems: Vec::new(),
            complexity: ComplexityLevel::Simple,
            execution_time_estimate: 5,
        });
        
        capabilities
    }
    
    /// הפעל heartbeat
    pub async fn heartbeat(&mut self) {
        self.heartbeat_manager.heartbeat_count += 1;
        self.heartbeat_manager.last_heartbeat = Utc::now();
        
        // עדכן metrics
        self.update_metrics_snapshot().await;
        
        // בדוק בריאות תתי-מערכות
        self.check_subsystem_health().await;
        
        // בדוק אם יש עדכונים
        self.check_for_updates().await;
        
        // סנכרן עם cron jobs
        self.sync_cron_jobs().await;
        
        // עדכן סטטוס סוכן
        self.update_agent_status().await;
    }
    
    /// עדכן metrics snapshot
    async fn update_metrics_snapshot(&mut self) {
