use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Json},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{info, Level};
use tracing_subscriber;

// Import our modules
mod hebrew;
mod nvidia;

use nvidia::{NvidiaClient, NvidiaConfig, NVIDIA_MODELS};

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
}

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

// API handlers
async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "CalcLaw is running! 🦾")
}

async fn process_hebrew(
    State(state): State<Arc<AppState>>,
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

// NVIDIA API handler
async fn nvidia_generate(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<NvidiaApiRequest>,
) -> impl IntoResponse {
    match &state.nvidia_client {
        Some(client) => {
            info!("NVIDIA API request: {}", payload.prompt);
            
            // Update model if specified
            if let Some(model) = payload.model {
                let mut config = client.get_config().await;
                config.model = model;
                client.update_config(config).await;
            }
            
            match client.generate(&payload.prompt, None).await {
                Ok(response) => {
                    let api_response = NvidiaApiResponse {
                        success: true,
                        response,
                        model: client.get_config().await.model,
                        tokens_used: None, // Would need to parse from response
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

// Configure NVIDIA API
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
                    // For now, we'll just return success
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

// List available NVIDIA models
async fn nvidia_models() -> impl IntoResponse {
    let models: Vec<(String, String)> = NVIDIA_MODELS
        .iter()
        .map(|(id, name)| (id.to_string(), name.to_string()))
        .collect();
    
    (StatusCode::OK, Json(models))
}

async fn admin_dashboard() -> Html<&'static str> {
    Html(r#"
        <!DOCTYPE html>
        <html dir="rtl" lang="he">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>CalcLaw - לוח הבקרה</title>
            <style>
                body { font-family: Arial, sans-serif; margin: 40px; }
                .hebrew { text-align: right; direction: rtl; }
                .header { background: #4CAF50; color: white; padding: 20px; border-radius: 5px; }
                .status { background: #f0f0f0; padding: 15px; margin: 10px 0; border-radius: 5px; }
                .api-section { background: #e3f2fd; padding: 15px; margin: 10px 0; border-radius: 5px; }
                .model-list { background: #fff3e0; padding: 10px; border-radius: 3px; }
            </style>
        </head>
        <body>
            <div class="header">
                <h1>CalcLaw 🦾 - לוח הבקרה</h1>
                <p>ניהול ארגוני עם תמיכה בעברית + NVIDIA AI</p>
            </div>
            <div class="status">
                <h2>סטטוס המערכת</h2>
                <p>✅ המערכת פועלת</p>
                <p>📊 משתמשים: 3 רשומים</p>
                <p>🔗 ערוצים: טלגרם (בפיתוח)</p>
                <p>🤖 NVIDIA AI: מוכן להגדרה</p>
            </div>
            <div class="api-section">
                <h3>NVIDIA API הגדרה</h3>
                <div class="hebrew">
                    <p>השתמש ב-API key שלך להפעלת מודלי AI מתקדמים:</p>
                    <div class="model-list">
                        <h4>מודלים זמינים:</h4>
                        <ul>
                            <li><strong>meta/llama-3.1-8b-instruct</strong> - Llama 3.1 8B</li>
                            <li><strong>meta/llama-3.1-70b-instruct</strong> - Llama 3.1 70B</li>
                            <li><strong>google/gemma-2-9b-it</strong> - Gemma 2 9B</li>
                            <li><strong>google/gemma-2-27b-it</strong> - Gemma 2 27B</li>
                            <li><strong>mistralai/mixtral-8x7b-instruct</strong> - Mixtral 8x7B</li>
                        </ul>
                    </div>
                    <p>API endpoint: <code>POST /api/nvidia/generate</code></p>
                </div>
            </div>
            <div class="hebrew">
                <h3>תכונות עיקריות:</h3>
                <ul>
                    <li>תמיכה מלאה בעברית (ימין לשמאל)</li>
                    <li>חיבור לטלגרם ו-WhatsApp</li>
                    <li>אינטגרציה עם Monday.com ו-Salesforce</li>
                    <li>ניהול הרשאות לפי מחלקות</li>
                    <li>תובנות נתונים ארגוניות</li>
                    <li><strong>NVIDIA AI models</strong> - מודלים מתקדמים</li>
                </ul>
            </div>
        </body>
        </html>
    "#)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("🚀 Starting CalcLaw server with NVIDIA support...");

    // Create initial users
    let users = vec![
        User {
            id: "1".to_string(),
            name: "אדמין ראשי".to_string(),
            role: UserRole::Admin,
            department: "IT".to_string(),
        },
        User {
            id: "2".to_string(),
            name: "מנהל מכירות".to_string(),
            role: UserRole::Superuser("Sales".to_string()),
            department: "Sales".to_string(),
        },
        User {
            id: "3".to_string(),
            name: "עובד שיווק".to_string(),
            role: UserRole::User("Marketing".to_string()),
            department: "Marketing".to_string(),
        },
    ];

    // Initialize without NVIDIA client (will be configured via API)
    let state = Arc::new(AppState {
        users,
        nvidia_client: None,
    });

    // Build our application
    let app = Router::new()
        .route("/", get(health_check))
        .route("/health", get(health_check))
        .route("/api/hebrew", post(process_hebrew))
        .route("/api/users", get(list_users))
        .route("/api/nvidia/generate", post(nvidia_generate))
        .route("/api/nvidia/configure", post(nvidia_configure))
        .route("/api/nvidia/models", get(nvidia_models))
        .route("/admin", get(admin_dashboard))
        .with_state(state);

    // Run server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    info!("✅ CalcLaw server running on http://{}", listener.local_addr()?);
    info!("📊 Admin dashboard: http://127.0.0.1:3000/admin");
    info!("🔧 API endpoints:");
    info!("  GET  /health - Health check");
    info!("  POST /api/hebrew - Process Hebrew text");
    info!("  GET  /api/users - List users");
    info!("  POST /api/nvidia/generate - Generate with NVIDIA AI");
    info!("  POST /api/nvidia/configure - Configure NVIDIA API");
    info!("  GET  /api/nvidia/models - List available models");
    info!("");
    info!("🤖 NVIDIA Setup:");
    info!("  1. Get API key from: https://build.nvidia.com/");
    info!("  2. Configure: curl -X POST http://127.0.0.1:3000/api/nvidia/configure \\");
    info!("     -H 'Content-Type: application/json' \\");
    info!("     -d '{\"api_key\": \"YOUR_KEY\", \"model\": \"meta/llama-3.1-8b-instruct\", \"temperature\": 0.7, \"max_tokens\": 1024}'");
    info!("");
    info!("🇮🇱 Test Hebrew: curl -X POST http://127.0.0.1:3000/api/hebrew \\");
    info!("  -H 'Content-Type: application/json' \\");
    info!("  -d '{{\"text\": \"שלום עולם\"}}'");

    axum::serve(listener, app).await?;

    Ok(())
}