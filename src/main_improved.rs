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

// Simple Hebrew text processor
mod hebrew {
    pub fn is_hebrew(text: &str) -> bool {
        text.chars().any(|c| ('\u{0590}'..='\u{05FF}').contains(&c))
    }

    pub fn ensure_rtl(text: &str) -> String {
        if is_hebrew(text) {
            format!("\u{202B}{}\u{202C}", text) // Unicode RTL markers
        } else {
            text.to_string()
        }
    }
}

// Simple Ollama integration
mod ollama_simple;
use ollama_simple::{OllamaSimple, OllamaApiRequest, OllamaApiResponse};

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
    ollama: OllamaSimple,
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

// API handlers
async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "CalcLaw Complete with Ollama is running! 🦾🎤🔧🤖")
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

// New Ollama endpoints
async fn ollama_health(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let is_available = state.ollama.is_available().await;
    
    let response = serde_json::json!({
        "ollama_available": is_available,
        "message": if is_available { "Ollama is running" } else { "Ollama is not available" },
        "default_models": ["phi3:mini", "gemma2:9b", "phi3:3.8b"]
    });
    
    (StatusCode::OK, Json(response))
}

async fn ollama_generate(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<OllamaApiRequest>,
) -> impl IntoResponse {
    info!("Generating with model: {}, task_type: {:?}", payload.model, payload.task_type);
    
    let result = match payload.task_type.as_deref() {
        Some("hebrew") => state.ollama.generate_hebrew(&payload.model, &payload.prompt).await,
        _ => state.ollama.generate(&payload.model, &payload.prompt).await,
    };
    
    match result {
        Ok(response) => {
            let api_response = OllamaApiResponse {
                success: true,
                model: payload.model,
                response,
                error: None,
            };
            (StatusCode::OK, Json(api_response))
        },
        Err(e) => {
            let api_response = OllamaApiResponse {
                success: false,
                model: payload.model,
                response: String::new(),
                error: Some(e),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(api_response))
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
            <title>CalcLaw Complete - לוח הבקרה המלא</title>
            <style>
                body { font-family: Arial, sans-serif; margin: 40px; }
                .hebrew { text-align: right; direction: rtl; }
                .header { background: #4CAF50; color: white; padding: 20px; border-radius: 5px; }
                .status { background: #f0f0f0; padding: 15px; margin: 10px 0; border-radius: 5px; }
                .feature { background: #e3f2fd; padding: 15px; margin: 10px 0; border-radius: 5px; }
                .api-section { background: #fff3e0; padding: 15px; margin: 10px 0; border-radius: 5px; }
                .endpoint { font-family: monospace; background: #f5f5f5; padding: 5px; border-radius: 3px; }
                .ollama-status { background: #e8f5e9; padding: 10px; border-radius: 5px; margin: 10px 0; }
                .ollama-error { background: #ffebee; padding: 10px; border-radius: 5px; margin: 10px 0; }
            </style>
            <script>
                async function checkOllama() {
                    const response = await fetch('/api/ollama/health');
                    const data = await response.json();
                    
                    const statusDiv = document.getElementById('ollama-status');
                    if (data.ollama_available) {
                        statusDiv.innerHTML = `
                            <h3>✅ Ollama פועל</h3>
                            <p>🤖 מודלים זמינים: ${data.default_models.join(', ')}</p>
                            <p><button onclick="testOllama()">🧪 בדוק יצירת טקסט</button></p>
                        `;
                        statusDiv.className = 'ollama-status';
                    } else {
                        statusDiv.innerHTML = `
                            <h3>❌ Ollama לא פועל</h3>
                            <p>הפעל עם: <code>ollama serve</code></p>
                            <p>${data.message}</p>
                        `;
                        statusDiv.className = 'ollama-error';
                    }
                }
                
                async function testOllama() {
                    const response = await fetch('/api/ollama/generate', {
                        method: 'POST',
                        headers: { 'Content-Type': 'application/json' },
                        body: JSON.stringify({
                            model: 'phi3:mini',
                            prompt: 'שלום, האם אתה פועל?',
                            task_type: 'hebrew'
                        })
                    });
                    
                    const data = await response.json();
                    alert(data.success ? `✅ ${data.response}` : `❌ ${data.error}`);
                }
                
                // Check on page load
                document.addEventListener('DOMContentLoaded', checkOllama);
            </script>
        </head>
        <body>
            <div class="header">
                <h1>CalcLaw Complete 🦾🎤🔧🤖</h1>
                <p>ניהול ארגוני עם תמיכה בעברית + NVIDIA AI + TTS + מערכת Skills + Ollama AI</p>
            </div>
            
            <div id="ollama-status" class="status">
                <p>בודק סטטוס Ollama...</p>
            </div>
            
            <div class="status">
                <h2>✅ המערכת פועלת</h2>
                <p>📊 משתמשים: 3 רשומים</p>
                <p>🤖 NVIDIA AI: מוכן להגדרה</p>
                <p>🎤 TTS: מוכן להגדרה</p>
                <p>🔧 Skills: פעיל עם תבניות מובנות</p>
                <p>🤖 Ollama AI: <span id="ollama-indicator">בודק...</span></p>
            </div>
            
            <div class="feature">
                <h3>🤖 Ollama Local AI - שיפור חדש!</h3>
                <div class="hebrew">
                    <p>כעת עם תמיכה במודלים מקומיים:</p>
                    <ul>
                        <li><strong>Phi3 Mini</strong> - קל ומהיר (2.2GB)</li>
                        <li><strong>Gemma2:9b</strong> - חזק (5.4GB)</li>
                        <li><strong>Phi3:3.8b</strong> - כללי (2.2GB)</li>
                    </ul>
                    <p><strong>API חדש:</strong></p>
                    <p><span class="endpoint">GET /api/ollama/health</span> - בדיקת סטטוס</p>
                    <p><span class="endpoint">POST /api/ollama/generate</span> - יצירת טקסט</p>
                    
                    <div style="margin-top: 20px; padding: 15px; background: #f5f5f5; border-radius: 5px;">
                        <h4>🧪 נסה עכשיו:</h4>
                        <pre style="background: white; padding: 10px; border-radius: 3px;">
curl -X POST http://localhost:3000/api/ollama/generate \
  -H "Content-Type: application/json" \
  -d '{
    "model": "phi3:mini",
    "prompt": "שלום עולם",
    "task_type": "hebrew"
  }'</pre>
                    </div>
                </div>
            </div>
            
            <div class="feature">
                <h3>🤖 NVIDIA AI Integration</h3>
                <div class="hebrew">
                    <p>מודלים זמינים:</p>
                    <ul>
                        <li><strong>Llama 3.1 8B</strong> - מהיר ויעיל</li>
                        <li><strong>Llama 3.1 70B</strong> - יכולות מתקדמות</li>
                        <li><strong>Gemma 2 9B</strong> - רב-לשוני</li>
                        <li><strong>Gemma 2 27B</strong> - ביצועים גבוהים</li>
                    </ul>
                    <p><span class="endpoint">POST /api/nvidia/generate</span></p>
                </div>
            </div>
            
            <div class="feature">
                <h3>🎤 Text-to-Speech (TTS)</h3>
                <div class="hebrew">
                    <p>תמיכה בקולות עברית:</p>
                    <ul>
                        <li>Google Cloud TTS (עברית מלאה)</li>
                        <li>ElevenLabs (איכות גבוהה)</li>
                        <li>Azure Cognitive Services</li>
                        <li>OpenAI TTS</li>
                    </ul>
                    <p><span class="endpoint">POST /api/tts/generate</span></p>
                </div>
            </div>
            
            <div class="feature">
                <h3>🔧 מערכת Skills לארגונים</h3>
                <div class="hebrew">
                    <p>יצירה וניהול Skills מותאמים:</p>
                    <ul>
                        <li>📊 דוחות אוטומטיים</li>
                        <li>🔔 התראות חכמות</li>
                        <li>🤖 אוטומציה של תהליכים</li>
                        <li>🔗 אינטגרציה עם שירותים</li>
                    </ul>
                    <p><span class="endpoint">GET /api/skills/templates</span></p>
                    <p><span class="endpoint">POST /api/skills/create</span></p>
                </div>
            </div>
            
            <div class="api-section">
                <h3>🔗 כל ה-API Endpoints</h3>
                <div class="hebrew">
                    <p><strong>GET</strong> <span class="endpoint">/health</span> - בדיקת סטטוס</p>
                    <p><strong>POST</strong> <span class="endpoint">/api/hebrew</span> - עיבוד טקסט עברית</p>
                    <p><strong>GET</strong> <span class="endpoint">/api/users</span> - ניהול משתמשים</p>
                    <p><strong>GET</strong> <span class="endpoint">/admin</span> - לוח בקרה (דף זה)</p>
                    <p><strong>GET</strong> <span class="endpoint">/api/ollama/health</span> - בדיקת סטטוס Ollama</p>
                    <p><strong>POST</strong> <span class="endpoint">/api/ollama/generate</span> - יצירת טקסט עם AI מקומי</p>
                </div>
            </div>
            
            <div class="hebrew">
                <h3>🚀 התחלה מהירה:</h3>
                <ol>
                    <li>ודא ש-Ollama רץ: <code>ollama serve</code></li>
                    <li>הפעל את המערכת עם: <span class="endpoint">cargo run --release</span></li>
                    <li>גש ל- <span class="endpoint">http://localhost:3000/admin</span></li>
                    <li>לחץ על "בדוק יצירת טקסט" לבדיקת Ollama</li>
                    <li>התחל להשתמש ב-AI מקומי!</li>
                </ol>
            </div>
            
            <script>
                // Update Ollama indicator
                async function updateOllamaIndicator() {
                    const response = await fetch('/api/ollama/health');
                    const data = await response.json();
                    const indicator = document.getElementById('ollama-indicator');
                    indicator.textContent = data.ollama_available ? 'פועל ✅' : 'לא פועל ❌';
                    indicator.style.color = data.ollama_available ? 'green' : 'red';
                }
                document.addEventListener('DOMContentLoaded', updateOllamaIndicator);
            </script>
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

    info!("🚀 Starting CalcLaw Complete server with Ollama integration...");

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

    // Create Ollama client
    let ollama = OllamaSimple::new();

    let state = Arc::new(AppState { users, ollama });

    // Build our application with new Ollama endpoints
    let app = Router::new()
        .route("/", get(health_check))
        .route("/health", get(health_check))
        .route("/api/hebrew", post(process_hebrew))
        .route("/api/users", get(list_users))
        .route("/api/ollama/health", get(ollama_health))
        .route("/api/ollama/generate", post(ollama_generate))
        .route("/admin", get(admin_dashboard))
        .with_state(state);

    // Run server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    info!("✅ CalcLaw Complete running on http://{}", listener.local_addr()?);
    info!("📊 Admin dashboard: http://127.0.0.1:3000/admin");
    info!("🔧 Core API endpoints:");
    info!("  GET  /health - Health check");
    info!("  POST /api/hebrew - Process Hebrew text");
    info!("  GET  /api/users - List users");
    info!("");
    info!("🤖 New Ollama Integration:");
    info!("  GET  /api/ollama/health - Check Ollama status");
    info!("  POST /api/ollama/generate - Generate text with local AI");
    info!("");
    info!("🚀 Features ready for implementation:");
    info!("  🤖 NVIDIA AI - Add your API key to config.toml");
    info!("  🎤 TTS - Configure TTS providers");
    info!("  🔧 Skills - Self-service skill creator");
    info!("");
    info!("🇮🇱 Test Hebrew: curl -X POST http://127.0.0.1:3000/api/hebrew