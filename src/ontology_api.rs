// 🚀 Ontology API Module
// REST API endpoints for ontology management

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

use crate::organization_ontology::*;

/// 🏢 Application State
#[derive(Clone)]
pub struct AppState {
    pub super_agent: Arc<RwLock<SuperAgent>>,
    pub ui_manager: Arc<RwLock<OntologyUIManager>>,
    pub questionnaire: Arc<RwLock<OnboardingQuestionnaire>>,
}

/// 🚀 Create API router
pub fn create_ontology_api() -> Router<AppState> {
    Router::new()
        // Organization endpoints
        .route("/api/ontology/organization", get(get_organization))
        .route("/api/ontology/organization", put(update_organization))
        
        // Questionnaire endpoints
        .route("/api/ontology/questionnaire", get(get_questionnaire))
        .route("/api/ontology/questionnaire/questions", get(get_questions))
        .route("/api/ontology/questionnaire/answer", post(answer_question))
        .route("/api/ontology/questionnaire/generate", post(generate_ontology))
        .route("/api/ontology/questionnaire/completion", get(get_completion))
        
        // Department endpoints
        .route("/api/ontology/departments", get(get_departments))
        .route("/api/ontology/departments", post(create_department))
        .route("/api/ontology/departments/:id", get(get_department))
        .route("/api/ontology/departments/:id", put(update_department))
        .route("/api/ontology/departments/:id", delete(delete_department))
        
        // Process endpoints
        .route("/api/ontology/processes", get(get_processes))
        .route("/api/ontology/processes", post(create_process))
        .route("/api/ontology/processes/:id", get(get_process))
        .route("/api/ontology/processes/:id", put(update_process))
        
        // Data entity endpoints
        .route("/api/ontology/data-entities", get(get_data_entities))
        .route("/api/ontology/data-entities", post(create_data_entity))
        .route("/api/ontology/data-entities/:id", get(get_data_entity))
        .route("/api/ontology/data-entities/:id", put(update_data_entity))
        
        // System endpoints
        .route("/api/ontology/systems", get(get_systems))
        .route("/api/ontology/systems", post(create_system))
        .route("/api/ontology/systems/:id", get(get_system))
        
        // Integration endpoints
        .route("/api/ontology/integrations", get(get_integrations))
        .route("/api/ontology/integrations", post(create_integration))
        
        // Goal endpoints
        .route("/api/ontology/goals", get(get_goals))
        .route("/api/ontology/goals", post(create_goal))
        
        // Metric endpoints
        .route("/api/ontology/metrics", get(get_metrics))
        .route("/api/ontology/metrics", post(create_metric))
        
        // Agent endpoints
        .route("/api/ontology/agent/status", get(get_agent_status))
        .route("/api/ontology/agent/capabilities", get(get_capabilities))
        .route("/api/ontology/agent/tasks", get(get_tasks))
        .route("/api/ontology/agent/tasks", post(create_task))
        .route("/api/ontology/agent/tasks/:id/execute", post(execute_task))
        .route("/api/ontology/agent/recommendations", get(get_recommendations))
        
        // UI endpoints
        .route("/api/ontology/ui/visualization", get(get_visualization))
        .route("/api/ontology/ui/view", put(change_view))
        .route("/api/ontology/ui/interaction", post(record_interaction))
        .route("/api/ontology/ui/analytics", get(get_interaction_analytics))
        
        // Export/Import
        .route("/api/ontology/export", get(export_ontology))
        .route("/api/ontology/import", post(import_ontology))
        
        // Health check
        .route("/api/ontology/health", get(health_check))
}

/// 🩺 Health check
async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"status": "healthy", "service": "ontology"})))
}

/// 🏢 Get organization information
async fn get_organization(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let agent = state.super_agent.read().await;
    let profile = &agent.ontology.profile;
    
    Json(json!({
        "id": profile.id,
        "name": profile.name,
        "description": profile.description,
        "industry": profile.industry,
        "size": format!("{:?}", profile.size),
        "location": profile.location,
        "timezone": profile.timezone,
        "language": profile.language,
        "website": profile.website,
        "founded_year": profile.founded_year,
        "created_at": profile.created_at.to_rfc3339(),
        "updated_at": profile.updated_at.to_rfc3339(),
    }))
}

/// 📝 Update organization information
async fn update_organization(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    let mut agent = state.super_agent.write().await;
    
    if let Some(name) = payload.get("name").and_then(|v| v.as_str()) {
        agent.ontology.profile.name = name.to_string();
    }
    
    if let Some(description) = payload.get("description").and_then(|v| v.as_str()) {
        agent.ontology.profile.description = description.to_string();
    }
    
    if let Some(industry) = payload.get("industry").and_then(|v| v.as_str()) {
        agent.ontology.profile.industry = industry.to_string();
    }
    
    agent.ontology.profile.updated_at = chrono::Utc::now();
    
    (StatusCode::OK, Json(json!({"status": "updated"})))
}

/// 📋 Get questionnaire
async fn get_questionnaire(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let questionnaire = state.questionnaire.read().await;
    
    Json(json!({
        "completed": questionnaire.completed,
        "completion_percentage": questionnaire.completion_percentage(),
        "total_questions": questionnaire.questions.len(),
        "answered_questions": questionnaire.answers.len(),
    }))
}

/// ❓ Get all questions
async fn get_questions(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let questionnaire = state.questionnaire.read().await;
    
    let questions: Vec<serde_json::Value> = questionnaire.questions.iter()
        .map(|q| {
            json!({
                "id": q.id,
                "category": format!("{:?}", q.category),
                "question": q.question,
                "description": q.description,
                "answer_type": format!("{:?}", q.answer_type),
                "required": q.required,
                "options": q.options,
            })
        })
        .collect();
    
    Json(json!({"questions": questions}))
}

/// ✅ Answer a question
async fn answer_question(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    let mut questionnaire = state.questionnaire.write().await;
    
    let question_id = payload.get("question_id")
        .and_then(|v| v.as_str())
        .and_then(|s| Uuid::parse_str(s).ok())
        .ok_or_else(|| {
            (StatusCode::BAD_REQUEST, Json(json!({"error": "Invalid question_id"})))
        })?;
    
    let answer = payload.get("answer")
        .cloned()
        .ok_or_else(|| {
            (StatusCode::BAD_REQUEST, Json(json!({"error": "Missing answer"})))
        })?;
    
    match questionnaire.answer_question(question_id, answer) {
        Ok(_) => {
            let completion = questionnaire.completion_percentage();
            (StatusCode::OK, Json(json!({
                "status": "answered",
                "completion_percentage": completion,
            })))
        }
        Err(err) => {
            (StatusCode::BAD_REQUEST, Json(json!({"error": err})))
        }
    }
}

/// 🧠 Generate ontology from questionnaire
async fn generate_ontology(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let questionnaire = state.questionnaire.read().await;
    
    match questionnaire.generate_ontology() {
        Ok(ontology) => {
            // Update the agent with new ontology
            let mut agent = state.super_agent.write().await;
            agent.ontology = ontology.clone();
            
            // Update UI manager
            let mut ui_manager = state.ui_manager.write().await;
            ui_manager.ontology = ontology;
            
            // Mark questionnaire as completed
            drop(questionnaire); // Release read lock
            let mut questionnaire = state.questionnaire.write().await;
            questionnaire.completed = true;
            
            (StatusCode::OK, Json(json!({
                "status": "generated",
                "organization": agent.ontology.profile.name,
                "departments": agent.ontology.departments.len(),
                "processes": agent.ontology.processes.len(),
                "data_entities": agent.ontology.data_entities.len(),
            })))
        }
        Err(err) => {
            (StatusCode::BAD_REQUEST, Json(json!({"error": err})))
        }
    }
}

/// 📊 Get completion percentage
async fn get_completion(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let questionnaire = state.questionnaire.read().await;
    let completion = questionnaire.completion_percentage();
    
    Json(json!({
        "completion_percentage": completion,
        "answered": questionnaire.answers.len(),
        "total": questionnaire.questions.len(),
    }))
}

/// 👥 Get all departments
async fn get_departments(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let agent = state.super_agent.read().await;
    
    let departments: Vec<serde_json::Value> = agent.ontology.departments.iter()
        .map(|dept| {
            let processes = agent.ontology.processes.iter()
                .filter(|p| p.department_id == dept.id)
                .count();
            
            json!({
                "id": dept.id,
                "name": dept.name,
                "description": dept.description,
                "employee_count": dept.employee_count,
                "process_count": processes,
                "responsibilities": dept.responsibilities,
            })
        })
        .collect();
    
    Json(json!({"departments": departments}))
}

/// ➕ Create a department
async fn create_department(
    State(state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    let mut agent = state.super_agent.write().await;
    
    let name = payload.get("name")
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            (StatusCode::BAD_REQUEST, Json(json!({"error": "Missing name"})))
        })?;
    
    let description = payload.get("description")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    
    let employee_count = payload.get("employee_count")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as usize;
    
    let department = Department {
        id: Uuid::new_v4(),
        name: name.to_string(),
        description: description.to_string(),
        parent_department_id: None,
        manager_id: None,
        employee_count,
        responsibilities: Vec::new(),
        key_processes: Vec::new(),
    };
    
    agent.ontology.departments.push(department.clone());
    
    // Update UI manager
    let mut ui_manager = state.ui_manager.write().await;
    ui_manager.ontology.departments.push(department);
    
    (StatusCode::CREATED, Json(json!({
        "status": "created",
        "id": department.id,
    })))
}

/// 👥 Get a specific department
async fn get_department(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let agent = state.super_agent.read().await;
    
    let department = agent.ontology.departments.iter()
        .find(|dept| dept.id == id)
        .ok_or_else(|| {
            (StatusCode::NOT_FOUND, Json(json!({"error": "Department not found"})))
        })?;
    
    let processes: Vec<serde_json::Value> = agent.ontology.processes.iter()
        .filter(|p| p.department_id == id)
        .map(|p| {
            json!({
                "id": p.id,
                "name": p.name,
                "description": p.description,
            })
        })
        .collect();
    
    Json(json!({
        "id": department.id,
        "name": department.name,
        "description": department.description,
        "employee_count": department.employee_count,
        "responsibilities": department.responsibilities,
        "processes": processes,
    }))
}

/// ✏️ Update a department
async fn update_department(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    let mut agent = state.super_agent.write().await;
    
    let department = agent.ontology.departments.iter_mut()
        .find(|dept| dept.id == id)
        .ok_or_else(|| {
            (StatusCode::NOT_FOUND, Json(json!({"error": "Department not found"})))
        })?;
    
    if let Some(name) = payload.get("name").and_then(|v| v.as_str()) {
        department.name = name.to_string();
    }
    
    if let Some(description) = payload.get("description").and_then(|v| v.as_str()) {
        department.description = description.to_string();
    }
    
    if let Some(employee_count) = payload.get("employee_count").and_then(|v| v.as_u64()) {
        department.employee_count = employee_count as usize;
    }
    
    if let Some(responsibilities) = payload.get("responsibilities").and_then(|v| v.as_array()) {
        department.responsibilities = responsibilities.iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();
    }
    
    // Update UI manager
    let mut ui_manager = state.ui_manager.write().await;
    if let Some(ui_dept) = ui_manager.ontology.departments.iter_mut().find(|d| d.id == id) {
        *ui_dept = department.clone();
    }
    
    (StatusCode::OK, Json(json!({"status": "updated"})))
}

/// 🗑️ Delete a department
async fn delete_department(
    State(state): State<AppState>,
