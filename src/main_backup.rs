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
    (StatusCode::OK, "CalcLaw Complete is running! 🦾🎤🔧")
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
            </style>
        </head>
        <body>
            <div class="header">
                <h1>CalcLaw Complete 🦾🎤🔧</h1>
                <p>ניהול ארגוני עם תמיכה בעברית + NVIDIA AI + TTS + מערכת Skills</p>
            </div>
            
            <div class="status">
                <h2>✅ המערכת פועלת</h2>
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
                <h3>🔗 API Endpoints זמינים</h3>
                <div class="hebrew">
                    <p><strong>GET</strong> <span class="endpoint">/health</span> - בדיקת סטטוס</p>
                    <p><strong>POST</strong> <span class="endpoint">/api/hebrew</span> - עיבוד טקסט עברית</p>
                    <p><strong>GET</strong> <span class="endpoint">/api/users</span> - ניהול משתמשים</p>
                    <p><strong>GET</strong> <span class="endpoint">/admin</span> - לוח בקרה (דף זה)</p>
                </div>
            </div>
            
            <div class="hebrew">
                <h3>🚀 התחלה מהירה:</h3>
                <ol>
                    <li>הגדר את מפתח ה-API של NVIDIA ב-config.toml</li>
                    <li>הפעל את המערכת עם: <span class="endpoint">cargo run --release</span></li>
                    <li>גש ל- <span class="endpoint">http://localhost:3000/admin</span></li>
                    <li>התחל ליצור Skills לארגון שלך</li>
                </ol>
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

    info!("🚀 Starting CalcLaw Complete server...");

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

    let state = Arc::new(AppState { users });

    // Build our application
    let app = Router::new()
        .route("/", get(health_check))
        .route("/health", get(health_check))
        .route("/api/hebrew", post(process_hebrew))
        .route("/api/users", get(list_users))
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
    info!("🚀 Features ready for implementation:");
    info!("  🤖 NVIDIA AI - Add your API key to config.toml");
    info!("  🎤 TTS - Configure TTS providers");
    info!("  🔧 Skills - Self-service skill creator");
    info!("");
    info!("🇮🇱 Test Hebrew: curl -X POST http://127.0.0.1:3000/api/hebrew \\");
    info!("  -H 'Content-Type: application/json' \\");
    info!("  -d '{{\"text\": \"שלום עולם מ-CalcLaw\"}}'");

    axum::serve(listener, app).await?;

    Ok(())
}