use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Json},
    routing::{get, post, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{info, Level};
use tracing_subscriber;

// Import our modules
mod hebrew;
mod nvidia;
mod tts;
mod skills;

use nvidia::{NvidiaClient, NvidiaConfig, NVIDIA_MODELS};
use tts::{TtsClient, TtsConfig, TtsRequest, TtsProvider};
use skills::{SkillCreator, SkillTemplate, SkillDefinition, SkillInstance, SkillInstanceUpdate};

// User roles
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum UserRole {
    Admin,
    Superuser(String), // Department name
    User(String),      // Department name
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: String,
    name: String,
    role: UserRole,
    department: String,
}

// App state
struct AppState {
    users: Vec<User>,
    nvidia_client: Option<Arc<NvidiaClient>>,
    tts_client: Option<Arc<TtsClient>>,
    skill_creator: Arc<SkillCreator>,
}

// Request/Response structures
#[derive(Debug, Serialize, Deserialize)]
struct HebrewRequest {
    text: String,
    user_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct HebrewResponse {
    original: String,
    processed: String,
    is_hebrew: bool,
    rtl_wrapped: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct NvidiaApiRequest {
    prompt: String,
    model: Option<String>,
    temperature: Option<f32>,
    max_tokens: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct NvidiaApiResponse {
    success: bool,
    response: String,
    model: String,
    tokens_used: Option<u32>,
    error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct NvidiaConfigRequest {
    api_key: String,
    model: String,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct NvidiaConfigResponse {
    success: bool,
    message: String,
    models: Vec<(String, String)>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TtsGenerateRequest {
    text: String,
    provider: Option<String>,
    voice_id: Option<String>,
    language: Option<String>,
    speed: Option<f32>,
    save_to_file: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SkillCreateRequest {
    template_name: String,
    organization_id: String,
    department: Option<String>,
    author: String,
    config: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SkillInstanceRequest {
    skill_id: String,
    organization_id: String,
    department: Option<String>,
    config: serde_json::Value,
}

// API handlers
async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "CalcLaw with TTS & Skills is running! 🦾🎤🔧")
}

async fn process_hebrew(
    State(_state): State<Arc<AppState>>,
    Json(payload): Json<HebrewRequest>,
) -> impl IntoResponse {
    let is_hebrew = hebrew::is_hebrew(&payload.text);
    let rtl_wrapped = hebrew::ensure_rtl(&payload.text);
    
    let response = HebrewResponse {
        original: payload.text.clone(),
        processed: format!("Processed: {}", payload.text),
        is_hebrew,
        rtl_wrapped,
    };
    
    info!("Processed Hebrew text: is_hebrew={}", is_hebrew);
    
    (StatusCode::OK, Json(response))
}

async fn list_users(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let users = state.users.clone();
    (StatusCode::OK, Json(users))
}

// NVIDIA API handlers
async fn nvidia_generate(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<NvidiaApiRequest>,
) -> impl IntoResponse {
    match &state.nvidia_client {
        Some(client) => {
            info!("NVIDIA API request: {}", payload.prompt);
            
            match client.generate(&payload.prompt, None).await {
                Ok(response) => {
                    let api_response = NvidiaApiResponse {
                        success: true,
                        response,
                        model: client.get_config().await.model,
                        tokens_used: None,
                        error: None,
                    };
                    (StatusCode::OK, Json(api_response))
                }
                Err(e) => {
                    let api_response = NvidiaApiResponse {
                        success: false,
                        response: String::new(),
                        model: String::new(),
                        tokens_used: None,
                        error: Some(format!("NVIDIA API error: {}", e)),
                    };
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(api_response))
                }
            }
        }
        None => {
            let api_response = NvidiaApiResponse {
                success: false,
                response: String::new(),
                model: String::new(),
                tokens_used: None,
                error: Some("NVIDIA API not configured".to_string()),
            };
            (StatusCode::SERVICE_UNAVAILABLE, Json(api_response))
        }
    }
}

async fn nvidia_configure(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<NvidiaConfigRequest>,
) -> impl IntoResponse {
    info!("Configuring NVIDIA API with model: {}", payload.model);
    
    let config = NvidiaConfig {
        api_key: payload.api_key,
        base_url: "https://integrate.api.nvidia.com/v1".to_string(),
        model: payload.model,
        temperature: payload.temperature,
        max_tokens: payload.max_tokens,
    };
    
    match NvidiaClient::new(config) {
        Ok(client) => {
            // Test connection
            match client.test_connection().await {
                Ok(test_result) => {
                    info!("NVIDIA API configured successfully: {}", test_result);
                    
                    let models_list: Vec<(String, String)> = NVIDIA_MODELS
                        .iter()
                        .map(|(id, name)| (id.to_string(), name.to_string()))
                        .collect();
                    
                    let response = NvidiaConfigResponse {
                        success: true,
                        message: format!("NVIDIA API configured successfully. {}", test_result),
                        models: models_list,
                    };
                    
                    // Would need to update state with new client
                    (StatusCode::OK, Json(response))
                }
                Err(e) => {
                    let response = NvidiaConfigResponse {
                        success: false,
                        message: format!("NVIDIA API test failed: {}", e),
                        models: vec![],
                    };
                    (StatusCode::BAD_REQUEST, Json(response))
                }
            }
        }
        Err(e) => {
            let response = NvidiaConfigResponse {
                success: false,
                message: format!("Failed to create NVIDIA client: {}", e),
                models: vec![],
            };
            (StatusCode::BAD_REQUEST, Json(response))
        }
    }
}

async fn nvidia_models() -> impl IntoResponse {
    let models: Vec<(String, String)> = NVIDIA_MODELS
        .iter()
        .map(|(id, name)| (id.to_string(), name.to_string()))
        .collect();
    
    (StatusCode::OK, Json(models))
}

// TTS handlers
async fn tts_generate(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<TtsGenerateRequest>,
) -> impl IntoResponse {
    match &state.tts_client {
        Some(client) => {
            let request = TtsRequest {
                text: payload.text,
                voice_config: Some(tts::VoiceConfig {
                    provider: match payload.provider.as_deref() {
                        Some("elevenlabs") => TtsProvider::ElevenLabs,
                        Some("azure") => TtsProvider::Azure,
                        Some("openai") => TtsProvider::OpenAITts,
                        Some("local") => TtsProvider::Local,
                        _ => TtsProvider::GoogleCloud,
                    },
                    voice_id: payload.voice_id.unwrap_or_else(|| "he-IL-Standard-A".to_string()),
                    language: payload.language.unwrap_or_else(|| "he-IL".to_string()),
                    speed: payload.speed.unwrap_or(1.0),
                    pitch: 0.0,
                    volume: 1.0,
                }),
                output_format: "mp3".to_string(),
                save_to_file: payload.save_to_file.unwrap_or(false),
                filename: None,
            };
            
            match client.generate_speech(request).await {
                Ok(tts_response) => {
                    (StatusCode::OK, Json(tts_response))
                }
                Err(e) => {
                    let error_response = tts::TtsResponse {
                        success: false,
                        audio_data: None,
                        file_path: None,
                        duration_ms: None,
                        provider: "".to_string(),
                        error: Some(format!("TTS error: {}", e)),
                    };
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
                }
            }
        }
        None => {
            let error_response = tts::TtsResponse {
                success: false,
                audio_data: None,
                file_path: None,
                duration_ms: None,
                provider: "".to_string(),
                error: Some("TTS not configured".to_string()),
            };
            (StatusCode::SERVICE_UNAVAILABLE, Json(error_response))
        }
    }
}

// Skills handlers
async fn skills_list_templates(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    match state.skill_creator.load_skill_templates().await {
        Ok(templates) => (StatusCode::OK, Json(templates)),
        Err(e) => {
            let error = serde_json::json!({
                "error": format!("Failed to load skill templates: {}", e)
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error))
        }
    }
}

async fn skills_create_from_template(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SkillCreateRequest>,
) -> impl IntoResponse {
    // Load template first
    let templates = match state.skill_creator.load_skill_templates().await {
        Ok(t) => t,
        Err(e) => {
            let error = serde_json::json!({
                "error": format!("Failed to load templates: {}", e)
            });
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(error));
        }
    };
    
    let template = templates.iter()
        .find(|t| t.name == payload.template_name);
    
    match template {
        Some(template) => {
            match state.skill_creator.create_skill_from_template(
                template,
                &payload.organization_id,
                payload.department.as_deref(),
                &payload.author,
            ).await {
                Ok(skill) => (StatusCode::CREATED, Json(skill)),
                Err(e) => {
                    let error = serde_json::json!({
                        "error": format!("Failed to create skill: {}", e)
                    });
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(error))
                }
            }
        }
        None => {
            let error = serde_json::json!({
                "error": format!("Template not found: {}", payload.template_name)
            });
            (StatusCode::NOT_FOUND, Json(error))
        }
    }
}

async fn skills_create_instance(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SkillInstanceRequest>,
) -> impl IntoResponse {
    match state.skill_creator.create_skill_instance(
        &payload.skill_id,
        &payload.organization_id,
        payload.department.as_deref(),
        payload.config,
    ).await {
        Ok(instance) => (StatusCode::CREATED, Json(instance)),
        Err(e) => {
            let error = serde_json::json!({
                "error": format!("Failed to create skill instance: {}", e)
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error))
        }
    }
}

async fn skills_list_organization(
    Path(organization_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    match state.skill_creator.list_organization_skills(&organization_id).await {
        Ok(skills) => (StatusCode::OK, Json(skills)),
        Err(e) => {
            let error = serde_json::json!({
                "error": format!("Failed to list skills: {}", e)
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error))
        }
    }
}

async fn skills_list_instances(
    Path(organization_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    match state.skill_creator.list_organization_instances(&organization_id).await {
        Ok(instances) => (StatusCode::OK, Json(instances)),
        Err(e) => {
            let error = serde_json::json!({
                "error": format!("Failed to list skill instances: {}", e)
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error))
        }
    }
}

async fn admin_dashboard() -> Html<&'static str> {
    Html(r#"
        <!DOCTYPE html>
        <html dir="rtl" lang="he">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>CalcLaw - לוח הבקרה המלא</title>
            <style>
                body { font-family: Arial, sans-serif; margin: 40px; }
                .hebrew { text-align: right; direction: rtl; }
                .header { background: #4CAF50; color: white; padding: 20px; border-radius: 5px; }
                .status { background: #f0f0f0; padding: 15px; margin: 10px 0; border-radius: 5px; }
                .feature { background: #e3f2fd; padding: 15px; margin: 10px 0; border-radius: 5px; }
                .api-section { background: #fff3e0; padding: 15px; margin: 10px 0; border-radius: 5px; }
                .endpoint { font-family: monospace; background: #f5f5f5; padding: 5px; border-radius: 3px; }
            </style>
        </head>
        <body>
            <div class="header">
                <h1>CalcLaw 🦾🎤🔧 - לוח הבקרה המלא</h1>
                <p>ניהול ארגוני עם תמיכה בעברית + NVIDIA AI + TTS + מערכת Skills</p>
            </div>
            
            <div class="status">
                <h2>סטטוס המערכת</h2>
                <p>✅ המערכת פועלת</p>
                <p>📊 משתמשים: 3 רשומים</p>
                <p>🤖 NVIDIA AI: מוכן להגדרה</p>
                <p>🎤 TTS: מוכן להגדרה</p>
                <p>🔧 Skills: פעיל עם תבניות מובנות</p>
            </div>
            
            <div class="feature">
                <h3>🤖 NVIDIA AI Integration</h3>
                <div class="hebrew">
                    <p>מודלים זמינים:</p>
                    <ul>
                        <li><strong>meta/llama-3.1-8b-instruct</strong> - Llama 3.1 8B</li>
                        <li><strong>meta/llama-3.1-70b-instruct</strong> - Llama 3.1 70B</li>
                        <li><strong>google/gemma-2-9b-it</strong> - Gemma 2 9B</li>
                        <li><strong>google/gemma-2-27b-it</strong> - Gemma 2 27B</li>
                    </ul>
                    <p class="endpoint">POST /api/nvidia/generate</p>
                </div>
            </div>
            
            <div class="feature">
                <h3>🎤 Text-to-Speech (TTS)</h3>
                <div class="hebrew">
                    <p>תמיכה בקולות עברית ואנגלית:</p>
                    <ul>
                        <li>Google Cloud TTS (עברית מלאה)</li>
                        <li>ElevenLabs (איכות גבוהה)</li>
                        <li>Azure Cognitive Services</li>
                        <li>OpenAI TTS</li>
                    </ul>
                    <p class="endpoint">POST /api/tts/generate</p>
                </div>
            </div>
            
            <div class="feature">
                <h3>🔧 מערכת Skills לארגונים</h3>
                <div class="hebrew">
                    <p>יצירה וניהול Skills מותאמים לארגון:</p>
                    <ul>
                        <li>📊 דוחות אוטומטיים</li>
                        <li>🔔 התראות חכמות</li>
                        <li>🤖 אוטומציה של תהליכים</li>
                        <li>🔗 אינטגרציה עם שירותים חיצוניים</li>
                    </ul>
                    <p class="endpoint">GET /api/skills/templates</p>
                    <p class="endpoint">POST /api/skills/create</p>
                </div>
            </div>
            
            <div class="api-section">
                <h3>🔗 API Endpoints</h3>
                <div class="hebrew">
                    <p><strong>GET</strong> <span class="endpoint">/health</span> - בדיקת סטטוס</p>
                    <p><strong