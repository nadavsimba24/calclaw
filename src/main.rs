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

// מודולים של Calclaw
mod hebrew;
mod ollama_simple;
mod timeless_squads;  // מודול Timeless Squads החדש!

use ollama_simple::{OllamaSimple, OllamaApiRequest};
use timeless_squads::{init_timeless_squads, create_timeless_router};

// תפקידי משתמש
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum UserRole {
    Admin,
    Superuser(String), // שם מחלקה
    User(String),      // שם מחלקה
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: String,
    name: String,
    role: UserRole,
    department: String,
}

// מצב האפליקציה
struct AppState {
    users: Vec<User>,
    ollama: OllamaSimple,
    timeless_manager: Arc<timeless_squads::TimelessSquadManager>,
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

// API handlers בסיסיים
async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "CalcLaw Complete with Timeless Squads is running! 🦾🎤🔧🤖👥")
}

async fn process_hebrew(
    State(_state): State<Arc<AppState>>,
    Json(payload): Json<HebrewRequest>,
) -> impl IntoResponse {
    let is_hebrew = hebrew_utils::is_hebrew(&payload.text);
    let rtl_wrapped = hebrew_utils::ensure_rtl(&payload.text);
    
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

// Ollama endpoints
async fn ollama_health(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let is_available = state.ollama.is_available().await;
    
    let response = serde_json::json!({
        "status": if is_available { "available" } else { "unavailable" },
        "service": "ollama",
        "timeless_squads": "integrated"
    });
    
    (StatusCode::OK, Json(response))
}

async fn ollama_generate(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<OllamaApiRequest>,
) -> impl IntoResponse {
    match state.ollama.generate(&payload.model, &payload.prompt).await {
        Ok(response) => {
            let response_json = serde_json::json!({ "response": response });
            (StatusCode::OK, Json(response_json))
        },
        Err(e) => {
            let error_response = serde_json::json!({
                "error": e.to_string(),
                "status": "error"
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        }
    }
}

// Timeless Squads dashboard
async fn timeless_dashboard() -> impl IntoResponse {
    let html = r#"
    <!DOCTYPE html>
    <html lang="he" dir="rtl">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>🚀 Timeless Cal Squads - Calclaw</title>
        <style>
            * {
                margin: 0;
                padding: 0;
                box-sizing: border-box;
                font-family: Arial, sans-serif;
            }
            
            body {
                background: linear-gradient(135deg, #1a237e 0%, #311b92 100%);
                color: white;
                min-height: 100vh;
                padding: 30px;
            }
            
            .container {
                max-width: 1200px;
                margin: 0 auto;
                background: rgba(255, 255, 255, 0.1);
                border-radius: 20px;
                padding: 30px;
                backdrop-filter: blur(10px);
                border: 1px solid rgba(255, 255, 255, 0.2);
            }
            
            header {
                text-align: center;
                margin-bottom: 40px;
            }
            
            h1 {
                font-size: 2.5rem;
                margin-bottom: 10px;
                display: flex;
                align-items: center;
                justify-content: center;
                gap: 15px;
            }
            
            .subtitle {
                font-size: 1.2rem;
                opacity: 0.9;
                margin-bottom: 30px;
            }
            
            .features {
                display: grid;
                grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
                gap: 25px;
                margin-bottom: 40px;
            }
            
            .feature-card {
                background: rgba(255, 255, 255, 0.15);
                border-radius: 15px;
                padding: 25px;
                border: 1px solid rgba(255, 255, 255, 0.3);
                transition: all 0.3s;
            }
            
            .feature-card:hover {
                transform: translateY(-5px);
                background: rgba(255, 255, 255, 0.25);
            }
            
            .feature-icon {
                font-size: 2.5rem;
                margin-bottom: 15px;
            }
            
            .feature-title {
                font-size: 1.3rem;
                margin-bottom: 10px;
                font-weight: bold;
            }
            
            .feature-desc {
                opacity: 0.9;
                line-height: 1.6;
            }
            
            .api-links {
                background: rgba(0, 0, 0, 0.3);
                border-radius: 15px;
                padding: 25px;
                margin-top: 30px;
            }
            
            .api-title {
                font-size: 1.4rem;
                margin-bottom: 20px;
                color: #4FC3F7;
            }
            
            .api-endpoint {
                background: rgba(255, 255, 255, 0.1);
                padding: 15px;
                border-radius: 10px;
                margin-bottom: 15px;
                font-family: monospace;
                border-left: 4px solid #4FC3F7;
            }
            
            .method {
                display: inline-block;
                padding: 5px 10px;
                background: #4FC3F7;
                color: black;
                border-radius: 5px;
                font-weight: bold;
                margin-right: 10px;
            }
            
            .endpoint {
                color: #E1F5FE;
            }
            
            .quick-actions {
                display: flex;
                gap: 15px;
                margin-top: 30px;
                flex-wrap: wrap;
            }
            
            .action-btn {
                padding: 15px 25px;
                background: linear-gradient(90deg, #4FC3F7, #29B6F6);
                border: none;
                border-radius: 10px;
                color: white;
                font-weight: bold;
                cursor: pointer;
                text-decoration: none;
                display: inline-block;
                transition: all 0.3s;
            }
            
            .action-btn:hover {
                background: linear-gradient(90deg, #29B6F6, #0288D1);
                transform: translateY(-3px);
            }
            
            footer {
                text-align: center;
                margin-top: 40px;
                opacity: 0.7;
                font-size: 0.9rem;
            }
        </style>
    </head>
    <body>
        <div class="container">
            <header>
                <h1>
                    <span>🚀</span>
                    Timeless Cal Squads
                    <span>🤖</span>
                </h1>
                <div class="subtitle">
                    צוותי סוכני AI אוטונומיים משולבים ב-Calclaw | מודלים מקומיים | פרטיות מלאה
                </div>
            </header>
            
            <div class="features">
                <div class="feature-card">
                    <div class="feature-icon">👥</div>
                    <div class="feature-title">צוותים אוטונומיים</div>
                    <div class="feature-desc">
                        צור צוותי סוכנים עם התמחויות שונות. כל סוכן בעל ידע ומיומנויות ייחודיות.
                    </div>
                </div>
                
                <div class="feature-card">
                    <div class="feature-icon">🧬</div>
                    <div class="feature-title">ירושה ארגונית</div>
                    <div class="feature-desc">
                        סוכנים יורשים ידע מהארגון, הצוות, והפרויקט. למידה מתמשכת ושיתוף ידע.
                    </div>
                </div>
                
                <div class="feature-card">
                    <div class="feature-icon">🎯</div>
                    <div class="feature-title">עיבוד פגישות</div>
                    <div class="feature-desc">
                        העלה סיכום פגישה והמערכת תזהה החלטות, תצור משימות, ותעקוב אחרי דדליינים.
                    </div>
                </div>
                
                <div class="feature-card">
                    <div class="feature-icon">🔒</div>
                    <div class="feature-title">פרטיות מלאה</div>
                    <div class="feature-desc">
                        כל המודלים מקומיים עם Ollama. אין שליחה לענן. כל הנתונים נשארים אצלך.
                    </div>
                </div>
            </div>
            
            <div class="api-links">
                <div class="api-title">📡 API Endpoints</div>
                
                <div class="api-endpoint">
                    <span class="method">POST</span>
                    <span class="endpoint">/api/timeless/squads</span>
                    <div style="margin-top: 10px; font-size: 0.9rem; opacity: 0.8;">
                        יצירת צוות Timeless חדש
                    </div>
                </div>
                
                <div class="api-endpoint">
                    <span class="method">POST</span>
                    <span class="endpoint">/api/timeless/agents/:agent_id/chat</span>
                    <div style="margin-top: 10px; font-size: 0.9rem; opacity: 0.8;">
                        שיחה עם סוכן ספציפי
                    </div>
                </div>
                
                <div class="api-endpoint">
                    <span class="method">POST</span>
                    <span class="endpoint">/api/timeless/squads/:squad_id/meeting</span>
                    <div style="margin-top: 10px; font-size: 0.9rem; opacity: 0.8;">
                        עיבוד סיכום פגישה
                    </div>
                </div>
                
                <div class="api-endpoint">
                    <span class="method">GET</span>
                    <span class="endpoint">/api/timeless/squads/:squad_id/status</span>
                    <div style="margin-top: 10px; font-size: 0.9rem; opacity: 0.8;">
                        סטטוס צוות ומשימות
                    </div>
                </div>
            </div>
            
            <div class="quick-actions">
                <a href="/api/timeless/squads" class="action-btn" target="_blank">
                    צור צוות חדש
                </a>
                <a href="/api/ollama/health" class="action-btn" target="_blank">
                    בדוק Ollama
                </a>
                <a href="/api/hebrew" class="action-btn" target="_blank">
                    עיבוד עברית
                </a>
                <a href="/api/users" class="action-btn" target="_blank">
                    רשימת משתמשים
                </a>
            </div>
            
            <footer>
                <p>🚀 Calclaw עם Timeless Squads | 🇮🇱 תמיכה מלאה בעברית | 🔗 מודלים מקומיים עם Ollama</p>
                <p>📁 קוד: /home/erez/.openclaw/workspace/calclaw | 🐙 גרסה: 0.3.0 עם Timeless</p>
            </footer>
        </div>
        
        <script>
            // דוגמת שימוש ב-API
            async function createMunicipalSquad() {
                const response = await fetch('/api/timeless/squads', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify({
                        name: 'צוות AI עירוני',
                        description: 'צוות AI לניהול עירוני חכם',
                        settings: {
                            inheritance_enabled: true,
                            real_time_processing: true,
                            auto_task_creation: true,
                            allowed_integrations: ['mapillary', 'gis', 'municipal_apis'],
                            notification_channels: ['slack', 'telegram']
                        }
                    })
                });
                
                const data = await response.json();
                alert(`צוות נוצר: ${JSON.stringify(data, null, 2)}`);
            }
            
            // הוסף כפתור לדוגמה
            document.addEventListener('DOMContentLoaded', function() {
                const container = document.querySelector('.quick-actions');
                const demoBtn = document.createElement('a');
                demoBtn.className = 'action-btn';
                demoBtn.href = '#';
                demoBtn.textContent = '🎮 הדגמה (צור צוות)';
                demoBtn.onclick = (e) => {
                    e.preventDefault();
                    createMunicipalSquad();
                };
                container.appendChild(demoBtn);
            });
        </script>
    </body>
    </html>
    "#;
    
    Html(html)
}

// יצירת נתיבי Calclaw עם Timeless Squads
fn create_app_router(state: Arc<AppState>) -> Router {
    // נתיבים בסיסיים של Calclaw
    let calclaw_routes = Router::new()
        .route("/", get(health_check))
        .route("/api/health", get(health_check))
        .route("/api/hebrew", post(process_hebrew))
        .route("/api/users", get(list_users))
        .route("/api/ollama/health", get(ollama_health))
        .route("/api/ollama/generate", post(ollama_generate))
        .route("/timeless", get(timeless_dashboard))
        .with_state(state.clone());
    
    // נתיבי Timeless Squads
    let timeless_routes = create_timeless_router(state.timeless_manager.clone());
    
    // שילוב כל הנתיבים
    calclaw_routes.merge(timeless_routes)
}

#[tokio::main]
async fn main() {
    // אתחול logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();
    
    info!("🚀 Starting CalcLaw with Timeless Cal Squads...");
    
    // אתחול Ollama
    let ollama = OllamaSimple::new("http://localhost:11434".to_string());
    
    // אתחול Timeless Squads Manager
    let timeless_manager = init_timeless_squads("http://localhost:11434").await;
    
    // יצירת משתמשים לדוגמה
    let users = vec![
        User {
            id: "1".to_string(),
            name: "ארז".to_string(),
            role: UserRole::Admin,
            department: "IT".to_string(),
        },
        User {
            id: "2".to_string(),
            name: "מיקי".to_string(),
            role: UserRole::Superuser("Municipal".to_string()),
            department: "Municipal AI".to_string(),
        },
        User {
            id: "3".to_string(),
            name: "גל".to_string(),
            role: UserRole::User("Data Analysis".to_string()),
            department: "Analytics".to_string(),
        },
    ];
    
    // יצירת state
    let state = Arc::new(AppState {
        users,
        ollama,
        timeless_manager,
    });
    
    // יצירת router
    let app = create_app_router(state);
    
    // הרצת השרת
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind port 3000");
    
    info!("🌐 Server running on http://localhost:3000");
    info!("🚀 Timeless Squads dashboard: http://localhost:3000/timeless");
    info!("🤖 Ollama integration: http://localhost:3000/api/ollama/health");
    info!("🇮🇱 Hebrew processing: http://localhost:3000/api/hebrew");
    
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

// פונקציות עיבוד עברית (לא מודול נוסף)
mod hebrew_utils {
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