#!/bin/bash

# 🔗 עדכון Calclaw עם יכולות Ontology
# מחבר את מערכת האונטולוגיה ל-Calclaw הראשי

set -e

echo "🔗 עדכון Calclaw עם יכולות Ontology"
echo "====================================="
echo ""

# בדוק אם אנחנו בספריית calclaw
if [[ ! -f "Cargo.toml" ]]; then
    echo "❌ אנחנו לא בספריית calclaw"
    echo "   עבור לספריית calclaw ונסה שוב"
    exit 1
fi

# בדוק אם src/ontology_api.rs קיים
if [[ ! -f "src/ontology_api.rs" ]]; then
    echo "❌ קובץ ontology_api.rs לא נמצא"
    echo "   ודא שהורדת את הגרסה המלאה עם ontology"
    exit 1
fi

echo "📦 מעדכן את Cargo.toml עם תלויות חדשות..."

# הוסף תלויות אם חסרות
if ! grep -q "serde_json" Cargo.toml; then
    echo 'serde_json = "1.0"' >> Cargo.toml
fi

if ! grep -q "uuid" Cargo.toml; then
    echo 'uuid = { version = "1.0", features = ["serde", "v4"] }' >> Cargo.toml
fi

if ! grep -q "chrono" Cargo.toml; then
    echo 'chrono = { version = "0.4", features = ["serde"] }' >> Cargo.toml
fi

if ! grep -q "axum" Cargo.toml; then
    echo 'axum = "0.7"' >> Cargo.toml
fi

if ! grep -q "tokio" Cargo.toml; then
    echo 'tokio = { version = "1.0", features = ["full"] }' >> Cargo.toml
fi

echo "✅ Cargo.toml עודכן"

echo ""
echo "🔗 מחבר את מודולי האונטולוגיה..."

# צור קובץ main חדש שמשלב את האונטולוגיה
cat > src/main_with_ontology.rs << 'EOF'
// 🚀 Calclaw עם Organizational Ontology
// גרסה משודרגת עם הבנה ארגונית

use axum::Router;
use std::sync::Arc;
use tokio::sync::RwLock;

mod organization_ontology;
mod ontology_api;
mod ollama_simple;

use organization_ontology::calclaw_ontology;
use ontology_api::{AppState, create_ontology_api};

#[tokio::main]
async fn main() {
    println!("🧠 Calclaw עם Organizational Ontology");
    println!("======================================");
    println!("");
    
    // בדוק אם Ollama רץ
    println!("🔍 בודק חיבור ל-Ollama...");
    let ollama_client = ollama_simple::OllamaClient::new("http://localhost:11434".to_string());
    match ollama_client.list_models().await {
        Ok(models) => {
            println!("✅ Ollama מחובר עם {} מודלים", models.len());
            for model in &models {
                println!("   • {}", model.name);
            }
        }
        Err(_) => {
            println!("⚠️  Ollama לא זמין - הרץ: ollama serve");
        }
    }
    
    println!("");
    println("🏢 מאתחל מערכת אונטולוגיה...");
    
    // צור את ה-App state
    let (super_agent, ui_manager) = calclaw_ontology::initialize_calclaw().await;
    
    let app_state = AppState {
        super_agent: Arc::new(RwLock::new(super_agent)),
        ui_manager: Arc::new(RwLock::new(ui_manager)),
        questionnaire: Arc::new(RwLock::new(organization_ontology::OnboardingQuestionnaire::new())),
    };
    
    // צור את ה-API router
    let app = Router::new()
        .nest("/api", create_ontology_api())
        .with_state(app_state);
    
    println!("");
    println!("🚀 שרת API מוכן!");
    println!("   • Ontology API: http://localhost:3000/api/ontology");
    println!("   • Health check: http://localhost:3000/api/ontology/health");
    println!("");
    println!("🎯 התחל עם שאלון ההתאמה הארגוני:");
    println!("   http://localhost:3000/api/ontology/questionnaire");
    println!("");
    println!("📊 או בדוק את סטטוס הסופר-אג'נט:");
    println!("   http://localhost:3000/api/ontology/agent/status");
    println!("");
    
    // הפעל את השרת
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🌐 שרת רץ על: http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
EOF

echo "✅ קובץ main_with_ontology.rs נוצר"

echo ""
echo "📁 מעדכן את ספריית src..."

# צור mod.rs שמחבר את כל המודולים
cat > src/lib.rs << 'EOF'
// 🧠 Calclaw עם Organizational Ontology
// ספרייה ראשית

pub mod organization_ontology;
pub mod ontology_api;
pub mod ollama_simple;

// Re-exports
pub use organization_ontology::*;
pub use ontology_api::*;
pub use ollama_simple::*;
EOF

echo "✅ src/lib.rs עודכן"

echo ""
echo "🔧 עדכן את Cargo.toml עם בינארי חדש..."

# הוסף בינארי חדש ל-Cargo.toml
if ! grep -q "name = \"calclaw-ontology\"" Cargo.toml; then
    cat >> Cargo.toml << 'EOF'

[[bin]]
name = "calclaw-ontology"
path = "src/main_with_ontology.rs"
EOF
fi

echo "✅ Cargo.toml עודכן עם בינארי חדש"

echo ""
echo "🏗️ בונה את הפרויקט..."

# בנה את הפרויקט
cargo build --release --bin calclaw-ontology

echo ""
echo "🎉 **התקנה הושלמה!**"
echo ""
echo "🚀 **הפעל את Calclaw עם Ontology:**"
echo "   ./target/release/calclaw-ontology"
echo ""
echo "🌐 **גש לממשקים:**"
echo "   • API: http://localhost:3000/api/ontology"
echo "   • Questionnaire: http://localhost:3000/api/ontology/questionnaire"
echo "   • Agent Status: http://localhost:3000/api/ontology/agent/status"
echo ""
echo "📚 **דוקומנטציה:**"
echo "   • קרא את README_ONTOLOGY.md"
echo "   • בדוק את הדוגמאות ב-examples/"
echo ""
echo "🔗 **אינטגרציות זמינות:**"
echo "   • עם Ollama למודלים מקומיים"
echo "   • עם Calclaw הבסיסי לניהול cron"
echo "   • עם Timeless Squads לצוותי AI"
echo "   • עם Municipal Vision AI לחיפוש תמונות"
echo ""
echo "🧠 **Calclaw עכשיו מבין את הארגון שלך ומבצע את העבודה בשבילך!**"
EOF

chmod +x scripts/update_main_with_ontology.sh

echo "✅ סקריפט עדכון נוצר"

echo ""
echo "🎯 **השלבים הבאים:**"
echo "   1. הרץ: ./scripts/update_main_with_ontology.sh"
echo "   2. הפעל: ./target/release/calclaw-ontology"
echo "   3. השלם את שאלון ההתאמה"
echo "   4. תן לסופר-אג'נט לעבוד בשבילך!"
echo ""
echo "🧠 **Calclaw עם Organizational Ontology מוכן לשימוש!**"