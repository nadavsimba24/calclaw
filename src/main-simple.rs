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
    (StatusCode::OK, Json(&state.users))
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
            </style>
        </head>
        <body>
            <div class="header">
                <h1>CalcLaw 🦾 - לוח הבקרה</h1>
                <p>ניהול ארגוני עם תמיכה בעברית</p>
            </div>
            <div class="status">
                <h2>סטטוס המערכת</h2>
                <p>✅ המערכת פועלת</p>
                <p>📊 משתמשים: 3 רשומים</p>
                <p>🔗 ערוצים: טלגרם (בפיתוח)</p>
            </div>
            <div class="hebrew">
                <h3>תכונות עיקריות:</h3>
                <ul>
                    <li>תמיכה מלאה בעברית (ימין לשמאל)</li>
                    <li>חיבור לטלגרם ו-WhatsApp</li>
                    <li>אינטגרציה עם Monday.com ו-Salesforce</li>
                    <li>ניהול הרשאות לפי מחלקות</li>
                    <li>תובנות נתונים ארגוניות</li>
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

    info!("🚀 Starting CalcLaw server...");

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
    info!("✅ CalcLaw server running on http://{}", listener.local_addr()?);
    info!("📊 Admin dashboard: http://127.0.0.1:3000/admin");
    info!("🔧 API endpoints:");
    info!("  GET  /health - Health check");
    info!("  POST /api/hebrew - Process Hebrew text");
    info!("  GET  /api/users - List users");
    info!("");
    info!("🇮🇱 Test Hebrew: curl -X POST http://127.0.0.1:3000/api/hebrew \\");
    info!("  -H 'Content-Type: application/json' \\");
    info!("  -d '{\"text\": \"שלום עולם\"}'");

    axum::serve(listener, app).await?;

    Ok(())
}