//! Timeless Cal Squads - צוותי סוכני AI אוטונומיים ל-Calclaw
//! 
//! מאפשר יצירת צוותי סוכנים עם:
//! - ירושה ארגונית
//! - עיבוד פגישות בזמן אמת
//! - הקצאת משימות אוטומטית
//! - אינטגרציה עם Ollama מקומי

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use anyhow::Result;
use reqwest::Client;
use tokio::sync::RwLock;
use std::sync::Arc;

/// סוג סוכן
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentType {
    TeamLead,           // ראש צוות
    DataAnalyst,        // אנליסט נתונים
    IntegrationSpecialist, // מומחה אינטגרציות
    MunicipalExpert,    // מומחה עירוני
    Custom(String),     // סוכן מותאם אישית
}

/// סוכן AI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: Uuid,
    pub name: String,
    pub agent_type: AgentType,
    pub model: String,           // מודל Ollama (לדוגמה: "phi3:mini")
    pub knowledge_base: Vec<KnowledgeItem>,
    pub skills: Vec<String>,
    pub conversation_history: Vec<ConversationMessage>,
    pub permissions: AgentPermissions,
}

/// פריט ידע
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeItem {
    pub id: Uuid,
    pub content: String,
    pub source: String,          // "organizational", "team", "personal"
    pub timestamp: DateTime<Utc>,
    pub tags: Vec<String>,
}

/// הודעת שיחה
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationMessage {
    pub id: Uuid,
    pub role: MessageRole,
    pub content: String,
    pub timestamp: DateTime<Utc>,
    pub agent_id: Option<Uuid>,
}

/// תפקיד בהודעה
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageRole {
    User,
    Agent,
    System,
}

/// הרשאות סוכן
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentPermissions {
    pub can_read_team_knowledge: bool,
    pub can_create_tasks: bool,
    pub can_assign_tasks: bool,
    pub can_access_sensitive_data: bool,
    pub can_integrate_external_apis: bool,
}

/// משימה
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub description: String,
    pub assigned_to: Option<Uuid>,   // ID של סוכן
    pub created_by: Uuid,            // ID של יוצר המשימה
    pub deadline: Option<DateTime<Utc>>,
    pub status: TaskStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub dependencies: Vec<Uuid>,     // משימות תלויות
    pub tags: Vec<String>,
}

/// סטטוס משימה
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Blocked,
    Completed,
    Cancelled,
}

/// צוות Timeless
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelessSquad {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub team_lead_id: Uuid,
    pub members: Vec<Uuid>,          // IDs של סוכנים
    pub organizational_knowledge: Vec<KnowledgeItem>,
    pub active_projects: Vec<Project>,
    pub settings: SquadSettings,
}

/// פרויקט
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub squad_id: Uuid,
    pub tasks: Vec<Uuid>,
    pub status: ProjectStatus,
    pub created_at: DateTime<Utc>,
    pub deadline: Option<DateTime<Utc>>,
}

/// סטטוס פרויקט
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectStatus {
    Planning,
    Active,
    OnHold,
    Completed,
    Archived,
}

/// הגדרות צוות
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SquadSettings {
    pub inheritance_enabled: bool,           // האם סוכנים יורשים ידע
    pub real_time_processing: bool,          // עיבוד פגישות בזמן אמת
    pub auto_task_creation: bool,            // יצירת משימות אוטומטית
    pub allowed_integrations: Vec<String>,   // אינטגרציות מותרות
    pub notification_channels: Vec<String>,  // ערוצי התראה
}

/// לקוח Ollama
pub struct OllamaClient {
    client: Client,
    base_url: String,
}

impl OllamaClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }
    
    /// שליחת prompt למודל Ollama
    pub async fn generate(&self, model: &str, prompt: &str, system: Option<&str>) -> Result<String> {
        let url = format!("{}/api/generate", self.base_url);
        
        let mut request_body = serde_json::json!({
            "model": model,
            "prompt": prompt,
            "stream": false,
            "options": {
                "temperature": 0.7,
                "top_p": 0.9,
                "num_predict": 512
            }
        });
        
        if let Some(system_prompt) = system {
            request_body["system"] = serde_json::Value::String(system_prompt.to_string());
        }
        
        let response = self.client
            .post(&url)
            .json(&request_body)
            .send()
            .await?;
        
        if response.status().is_success() {
            let response_json: serde_json::Value = response.json().await?;
            Ok(response_json["response"].as_str().unwrap_or("").to_string())
        } else {
            Err(anyhow::anyhow!("Ollama API error: {}", response.status()))
        }
    }
    
    /// רשימת מודלים זמינים
    pub async fn list_models(&self) -> Result<Vec<String>> {
        let url = format!("{}/api/tags", self.base_url);
        
        let response = self.client
            .get(&url)
            .send()
            .await?;
        
        if response.status().is_success() {
            let response_json: serde_json::Value = response.json().await?;
            let models = response_json["models"]
                .as_array()
                .unwrap_or(&vec![])
                .iter()
                .filter_map(|m| m["name"].as_str().map(|s| s.to_string()))
                .collect();
            
            Ok(models)
        } else {
            Err(anyhow::anyhow!("Failed to list models: {}", response.status()))
        }
    }
}

/// מנהל Timeless Squads
pub struct TimelessSquadManager {
    squads: Arc<RwLock<HashMap<Uuid, TimelessSquad>>>,
    agents: Arc<RwLock<HashMap<Uuid, Agent>>>,
    tasks: Arc<RwLock<HashMap<Uuid, Task>>>,
    ollama_client: OllamaClient,
}

impl TimelessSquadManager {
    pub fn new(ollama_url: &str) -> Self {
        Self {
            squads: Arc::new(RwLock::new(HashMap::new())),
            agents: Arc::new(RwLock::new(HashMap::new())),
            tasks: Arc::new(RwLock::new(HashMap::new())),
            ollama_client: OllamaClient::new(ollama_url),
        }
    }
    
    /// יצירת צוות חדש
    pub async fn create_squad(
        &self,
        name: &str,
        description: &str,
        settings: SquadSettings,
    ) -> Result<TimelessSquad> {
        let squad_id = Uuid::new_v4();
        
        // יצירת ראש צוות אוטומטי
        let team_lead = self.create_default_team_lead().await?;
        
        let squad = TimelessSquad {
            id: squad_id,
            name: name.to_string(),
            description: description.to_string(),
            team_lead_id: team_lead.id,
            members: vec![team_lead.id],
            organizational_knowledge: Vec::new(),
            active_projects: Vec::new(),
            settings,
        };
        
        // שמירת הסוכן והצוות
        {
            let mut agents = self.agents.write().await;
            agents.insert(team_lead.id, team_lead);
            
            let mut squads = self.squads.write().await;
            squads.insert(squad_id, squad.clone());
        }
        
        Ok(squad)
    }
    
    /// יצירת ראש צוות ברירת מחדל
    async fn create_default_team_lead(&self) -> Result<Agent> {
        let agent_id = Uuid::new_v4();
        
        let agent = Agent {
            id: agent_id,
            name: "מיקי - ראש צוות AI".to_string(),
            agent_type: AgentType::TeamLead,
            model: "gemma2:9b".to_string(), // מודל גדול יותר לראש צוות
            knowledge_base: vec![
                KnowledgeItem {
                    id: Uuid::new_v4(),
                    content: "אני ראש צוות AI עירוני".to_string(),
                    source: "organizational".to_string(),
                    timestamp: Utc::now(),
                    tags: vec!["role".to_string(), "introduction".to_string()],
                },
                KnowledgeItem {
                    id: Uuid::new_v4(),
                    content: "אני מנהל סוכנים אחרים בצוות".to_string(),
                    source: "organizational".to_string(),
                    timestamp: Utc::now(),
                    tags: vec!["management".to_string(), "responsibility".to_string()],
                },
            ],
            skills: vec![
                "ניהול צוות".to_string(),
                "הקצאת משימות".to_string(),
                "מעקב דדליינים".to_string(),
                "תיאום בין סוכנים".to_string(),
            ],
            conversation_history: Vec::new(),
            permissions: AgentPermissions {
                can_read_team_knowledge: true,
                can_create_tasks: true,
                can_assign_tasks: true,
                can_access_sensitive_data: true,
                can_integrate_external_apis: true,
            },
        };
        
        Ok(agent)
    }
    
    /// הוספת סוכן לצוות
    pub async fn add_agent_to_squad(
        &self,
        squad_id: Uuid,
        agent: Agent,
    ) -> Result<()> {
        let mut squads = self.squads.write().await;
        let mut agents = self.agents.write().await;
        
        if let Some(squad) = squads.get_mut(&squad_id) {
            // הוסף את הסוכן למאגר הסוכנים
            agents.insert(agent.id, agent.clone());
            
            // הוסף את הסוכן לצוות
            if !squad.members.contains(&agent.id) {
                squad.members.push(agent.id);
            }
            
            // העבר ידע ארגוני לסוכן החדש
            if squad.settings.inheritance_enabled {
                // כאן נוסיף לוגיקה להעברת ידע
                // (בגרסה מלאה, נשמור את הידע המשותף ונעביר אותו)
            }
            
            Ok(())
        } else {
            Err(anyhow::anyhow!("Squad not found"))
        }
    }
    
    /// שיחה עם סוכן
    pub async fn chat_with_agent(
        &self,
        agent_id: Uuid,
        message: &str,
        user_id: Option<Uuid>,
    ) -> Result<String> {
        let mut agents = self.agents.write().await;
        
        if let Some(agent) = agents.get_mut(&agent_id) {
            // בניית קונטקסט מההיסטוריה והידע
            let context = self.build_agent_context(agent).await;
            
            // יצירת system prompt
            let system_prompt = format!(
                "אתה {}, {}. 
                
ידע שלך:
{}
                
מיומנויות שלך:
{}
                
היה מועיל, מדויק, ופרודוקטיבי.",
                agent.name,
                match agent.agent_type {
                    AgentType::TeamLead => "ראש צוות AI",
                    AgentType::DataAnalyst => "אנליסט נתונים",
                    AgentType::IntegrationSpecialist => "מומחה אינטגרציות",
                    AgentType::MunicipalExpert => "מומחה עירוני",
                    AgentType::Custom(ref custom) => custom,
                },
                context,
                agent.skills.join(", ")
            );
            
            // שליחה ל-Ollama
            let response = self.ollama_client.generate(
                &agent.model,
                message,
                Some(&system_prompt),
            ).await?;
            
            // שמירת השיחה בהיסטוריה
            agent.conversation_history.push(ConversationMessage {
                id: Uuid::new_v4(),
                role: MessageRole::User,
                content: message.to_string(),
                timestamp: Utc::now(),
                agent_id: user_id,
            });
            
            agent.conversation_history.push(ConversationMessage {
                id: Uuid::new_v4(),
                role: MessageRole::Agent,
                content: response.clone(),
                timestamp: Utc::now(),
                agent_id: Some(agent_id),
            });
            
            Ok(response)
        } else {
            Err(anyhow::anyhow!("Agent not found"))
        }
    }
    
    /// בניית קונטקסט לסוכן
    async fn build_agent_context(&self, agent: &Agent) -> String {
        let mut context_parts = Vec::new();
        
        // ידע בסיסי
        if !agent.knowledge_base.is_empty() {
            context_parts.push("ידע בסיסי:".to_string());
            for item in agent.knowledge_base.iter().take(5) {
                context_parts.push(format!("- {}", item.content));
            }
        }
        
        // היסטוריית שיחה אחרונה
        if !agent.conversation_history.is_empty() {
            context_parts.push("\nשיחה אחרונה:".to_string());
            for msg in agent.conversation_history.iter().rev().take(3) {
                let role = match msg.role {
                    MessageRole::User => "משתמש",
                    MessageRole::Agent => "אתה",
                    MessageRole::System => "מערכת",
                };
                context_parts.push(format!("{}: {}", role, msg.content));
            }
        }
        
        context_parts.join("\n")
    }
    
    /// יצירת משימה חדשה
    pub async fn create_task(
        &self,
        description: &str,
        squad_id: Uuid,
        assigned_to: Option<Uuid>,
        deadline: Option<DateTime<Utc>>,
        tags: Vec<String>,
    ) -> Result<Task> {
        let task_id = Uuid::new_v4();
        
        let task = Task {
            id: task_id,
            description: description.to_string(),
            assigned_to,
            created_by: squad_id, // במקרה אמיתי, זה יהיה ID של יוצר המשימה
            deadline,
            status: TaskStatus::Pending,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            dependencies: Vec::new(),
            tags,
        };
        
        // שמירת המשימה
        {
            let mut tasks = self.tasks.write().await;
            tasks.insert(task_id, task.clone());
        }
        
        // הוספת המשימה לפרויקט פעיל (אם יש)
        // (בגרסה מלאה, נוסיף לוגיקה לניהול פרויקטים)
        
        Ok(task)
    }
    
    /// עיבוד סיכום פגישה
    pub async fn process_meeting_transcript(
        &self,
        squad_id: Uuid,
        transcript: &str,
        platform: &str,
    ) -> Result<Vec<Task>> {
        let squads = self.squads.read().await;
        
        if let Some(squad) = squads.get(&squad_id) {
            // מצא את ראש הצוות
            let agents = self.agents.read().await;
            if let Some(team_lead) = agents.get(&squad.team_lead_id) {
                // שליחת הטרנסקריפט ל-Ollama לניתוח
                let prompt = format!(
                    "סיכום פגישה מ-{}:
{}

זהה:
1. החלטות שהתקבלו
2. משימות שצריך לבצע
3. דדליינים שסוכמו
4. אנשים אחראיים

החזר רשימה ממוספרת של משימות.",
                    platform, transcript
                );
                
                let analysis = self.ollama_client.generate(
                    &team_lead.model,
                    &prompt,
                    Some("אתה ראש צוות AI. זהה החלטות ומשימות מסיכומי פגישות."),
                ).await?;
                
                // יצירת משימות מהניתוח
                let tasks = self.extract_tasks_from_analysis(&analysis).await;
                
                // יצירת משימות במערכת
                let mut created_tasks = Vec::new();
                
                for task_desc in tasks {
                    let task = self.create_task(
                        &task_desc,
                        squad_id,
                        None, // לא מוקצה עדיין
                        None, // אין דדליין מהניתוח האוטומטי
                        vec!["meeting".to_string(), platform.to_string()],
                    ).await?;
                    
                    created_tasks.push(task);
                }
                
                Ok(created_tasks)
            } else {
                Err(anyhow::anyhow!("Team lead not found"))
            }
        } else {
            Err(anyhow::anyhow!("Squad not found"))
        }
    }
    
    /// חילוץ משימות מניתוח AI
    async fn extract_tasks_from_analysis(&self, analysis: &str) -> Vec<String> {
        // פשוט - מחזיר את כל השורות שמכילות מילות מפתח
        // בגרסה מלאה, נשתמש ב-NLP מתקדם יותר
        
        let mut tasks = Vec::new();
        let keywords = ["לבצע", "לעשות", "להכין", "לשלוח", "לדווח", "לתאם", "לבדוק"];
        
        for line in analysis.lines() {
            let line = line.trim();
            if !line.is_empty() && keywords.iter().any(|kw| line.contains(kw)) {
                tasks.push(line.to_string());
            }
        }
        
        // אם לא מצאנו משימות ספציפיות, צור משימה כללית
        if tasks.is_empty() {
            tasks.push("עיבוד פגישה והפקת תובנות".to_string());
        }
        
        tasks
    }
    
    /// קבלת סטטוס צוות
    pub async fn get_squad_status(&self, squad_id: Uuid) -> Result<SquadStatus> {
        let squads = self.squads.read().await;
        let agents = self.agents.read().await;
        let tasks = self.tasks.read().await;
        
        if let Some(squad) = squads.get(&squad_id) {
            // ספירת משימות לפי סטטוס
            let mut pending_tasks = 0;
            let mut in_progress_tasks = 0;
            let mut completed_tasks = 0;
            
            for task in tasks.values() {
                // בדוק אם המשימה שייכת לצוות זה (בגרסה מלאה, נוסיף קישור)
                match task.status {
                    TaskStatus::Pending => pending_tasks += 1,
                    TaskStatus::InProgress => in_progress_tasks += 1,
                    TaskStatus::Completed => completed_tasks += 1,
                    _ => {}
                }
            }
            
            // מידע על הסוכנים
            let mut agent_info = Vec::new();
            for agent_id in &squad.members {
                if let Some(agent) = agents.get(agent_id) {
                    agent_info.push(AgentInfo {
                        id: agent.id,
                        name: agent.name.clone(),
                        agent_type: match agent.agent_type {
                            AgentType::TeamLead => "ראש צוות".to_string(),
                            AgentType::DataAnalyst => "אנליסט נתונים".to_string(),
                            AgentType::IntegrationSpecialist => "מומחה אינטגרציות".to_string(),
                            AgentType::MunicipalExpert => "מומחה עירוני".to_string(),
                            AgentType::Custom(ref custom) => custom.clone(),
                        },
                        model: agent.model.clone(),
                        conversation_count: agent.conversation_history.len(),
                    });
                }
            }
            
            Ok(SquadStatus {
                squad_id,
                name: squad.name.clone(),
                member_count: squad.members.len(),
                active_projects: squad.active_projects.len(),
                pending_tasks,
                in_progress_tasks,
                completed_tasks,
                agents: agent_info,
                knowledge_count: squad.organizational_knowledge.len(),
                last_updated: Utc::now(),
            })
        } else {
            Err(anyhow::anyhow!("Squad not found"))
        }
    }
    
    /// הוספת ידע ארגוני
    pub async fn add_organizational_knowledge(
        &self,
        squad_id: Uuid,
        content: &str,
        source: &str,
        tags: Vec<String>,
    ) -> Result<()> {
        let mut squads = self.squads.write().await;
        
        if let Some(squad) = squads.get_mut(&squad_id) {
            let knowledge_item = KnowledgeItem {
                id: Uuid::new_v4(),
                content: content.to_string(),
                source: source.to_string(),
                timestamp: Utc::now(),
                tags,
            };
            
            squad.organizational_knowledge.push(knowledge_item);
            
            // אם ירושה מופעלת, העבר את הידע לכל הסוכנים
            if squad.settings.inheritance_enabled {
                let mut agents = self.agents.write().await;
                
                for agent_id in &squad.members {
                    if let Some(agent) = agents.get_mut(agent_id) {
                        // הוסף את הידע לסוכן (ללא שכפול)
                        // (בגרסה מלאה, נבדוק אם הידע כבר קיים)
                        agent.knowledge_base.push(KnowledgeItem {
                            id: Uuid::new_v4(),
                            content: content.to_string(),
                            source: "inherited".to_string(),
                            timestamp: Utc::now(),
                            tags: vec!["organizational".to_string()],
                        });
                    }
                }
            }
            
            Ok(())
        } else {
            Err(anyhow::anyhow!("Squad not found"))
        }
    }
}

/// סטטוס צוות
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SquadStatus {
    pub squad_id: Uuid,
    pub name: String,
    pub member_count: usize,
    pub active_projects: usize,
    pub pending_tasks: usize,
    pub in_progress_tasks: usize,
    pub completed_tasks: usize,
    pub agents: Vec<AgentInfo>,
    pub knowledge_count: usize,
    pub last_updated: DateTime<Utc>,
}

/// מידע על סוכן
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentInfo {
    pub id: Uuid,
    pub name: String,
    pub agent_type: String,
    pub model: String,
    pub conversation_count: usize,
}

/// handlers ל-Axum
pub mod handlers {
    use super::*;
    use axum::{
        extract::{Path, State},
        Json,
        response::IntoResponse,
    };
    use std::sync::Arc;
    
    /// State של האפליקציה
    pub struct AppState {
        pub squad_manager: Arc<TimelessSquadManager>,
    }
    
    /// יצירת צוות חדש
    pub async fn create_squad(
        State(state): State<Arc<AppState>>,
        Json(payload): Json<CreateSquadRequest>,
    ) -> impl IntoResponse {
        match state.squad_manager.create_squad(
            &payload.name,
            &payload.description,
            payload.settings,
        ).await {
            Ok(squad) => Json(CreateSquadResponse::Success { squad }).into_response(),
            Err(err) => Json(CreateSquadResponse::Error { error: err.to_string() }).into_response(),
        }
    }
    
    /// שיחה עם סוכן
    pub async fn chat_with_agent(
        State(state): State<Arc<AppState>>,
        Path(agent_id): Path<Uuid>,
        Json(payload): Json<ChatRequest>,
    ) -> impl IntoResponse {
        match state.squad_manager.chat_with_agent(
            agent_id,
            &payload.message,
            payload.user_id,
        ).await {
            Ok(response) => Json(ChatResponse::Success { response }).into_response(),
            Err(err) => Json(ChatResponse::Error { error: err.to_string() }).into_response(),
        }
    }
    
    /// עיבוד פגישה
    pub async fn process_meeting(
        State(state): State<Arc<AppState>>,
        Path(squad_id): Path<Uuid>,
        Json(payload): Json<ProcessMeetingRequest>,
    ) -> impl IntoResponse {
        match state.squad_manager.process_meeting_transcript(
            squad_id,
            &payload.transcript,
            &payload.platform,
        ).await {
            Ok(tasks) => Json(ProcessMeetingResponse::Success { tasks_created: tasks.len(), tasks }).into_response(),
            Err(err) => Json(ProcessMeetingResponse::Error { error: err.to_string() }).into_response(),
        }
    }
    
    /// סטטוס צוות
    pub async fn get_squad_status(
        State(state): State<Arc<AppState>>,
        Path(squad_id): Path<Uuid>,
    ) -> impl IntoResponse {
        match state.squad_manager.get_squad_status(squad_id).await {
            Ok(status) => Json(GetSquadStatusResponse::Success { status }).into_response(),
            Err(err) => Json(GetSquadStatusResponse::Error { error: err.to_string() }).into_response(),
        }
    }
    
    /// הוספת ידע ארגוני
    pub async fn add_knowledge(
        State(state): State<Arc<AppState>>,
        Path(squad_id): Path<Uuid>,
        Json(payload): Json<AddKnowledgeRequest>,
    ) -> impl IntoResponse {
        match state.squad_manager.add_organizational_knowledge(
            squad_id,
            &payload.content,
            &payload.source,
            payload.tags,
        ).await {
            Ok(()) => Json(AddKnowledgeResponse::Success).into_response(),
            Err(err) => Json(AddKnowledgeResponse::Error { error: err.to_string() }).into_response(),
        }
    }
}

/// בקשות ו-responses
#[derive(Debug, Deserialize)]
pub struct CreateSquadRequest {
    pub name: String,
    pub description: String,
    pub settings: SquadSettings,
}

#[derive(Debug, Serialize)]
pub enum CreateSquadResponse {
    Success { squad: TimelessSquad },
    Error { error: String },
}

#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub message: String,
    pub user_id: Option<Uuid>,
}

#[derive(Debug, Serialize)]
pub enum ChatResponse {
    Success { response: String },
    Error { error: String },
}

#[derive(Debug, Deserialize)]
pub struct ProcessMeetingRequest {
    pub transcript: String,
    pub platform: String,
}

#[derive(Debug, Serialize)]
pub enum ProcessMeetingResponse {
    Success { tasks_created: usize, tasks: Vec<Task> },
    Error { error: String },
}

#[derive(Debug, Serialize)]
pub enum GetSquadStatusResponse {
    Success { status: SquadStatus },
    Error { error: String },
}

#[derive(Debug, Deserialize)]
pub struct AddKnowledgeRequest {
    pub content: String,
    pub source: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize)]
pub enum AddKnowledgeResponse {
    Success,
    Error { error: String },
}

/// פונקציה ליצירת router ל-Axum
pub fn create_timeless_router(squad_manager: Arc<TimelessSquadManager>) -> axum::Router {
    use axum::routing::{post, get};
    use handlers::*;
    
    let state = Arc::new(AppState { squad_manager });
    
    axum::Router::new()
        .route("/api/timeless/squads", post(create_squad))
        .route("/api/timeless/squads/:squad_id/status", get(get_squad_status))
        .route("/api/timeless/agents/:agent_id/chat", post(chat_with_agent))
        .route("/api/timeless/squads/:squad_id/meeting", post(process_meeting))
        .route("/api/timeless/squads/:squad_id/knowledge", post(add_knowledge))
        .with_state(state)
}

/// אתחול Timeless Squads ב-Calclaw
pub async fn init_timeless_squads(ollama_url: &str) -> Arc<TimelessSquadManager> {
    let manager = TimelessSquadManager::new(ollama_url);
    Arc::new(manager)
}