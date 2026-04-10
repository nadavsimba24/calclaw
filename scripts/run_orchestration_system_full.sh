#!/bin/bash

# 🚀 הרצת מערכת האורקסטרציה המלאה של Calclaw
# כולל מנוע, API, ודשבורד

set -e

echo "🚀 Calclaw Orchestration System"
echo "================================"
echo ""
echo "מערכת אורקסטרציה חכמה עם:"
echo "• 🤖 מנוע stateful עם heartbeat"
echo "• 🚀 REST API מלא"
echo "• 📊 דשבורד ניטור בזמן אמת"
echo "• 🔗 אינטגרציה עם מערכות קיימות"
echo ""

# בדוק אם אנחנו בספריית calclaw
if [[ ! -f "Cargo.toml" ]]; then
    echo "❌ אנחנו לא בספריית calclaw"
    echo "   עבור לספריית calclaw ונסה שוב"
    exit 1
fi

# הגדר פורטים
ORCHESTRATION_PORT=3001
DASHBOARD_PORT=3002

echo "🔧 הגדרות:"
echo "   • Orchestration API: http://localhost:$ORCHESTRATION_PORT"
echo "   • Dashboard: http://localhost:$DASHBOARD_PORT"
echo ""

# בדוק אם יש תלותיות חסרות
echo "📦 בודק תלותיות..."

if ! command -v cargo &> /dev/null; then
    echo "❌ Rust לא מותקן"
    echo "   התקן Rust עם: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# בנה את הפרויקט
echo "🏗️ בונה את מנוע האורקסטרציה..."

if ! cargo build --release --bin calclaw-orchestration 2>/dev/null; then
    echo "⚠️  לא ניתן לבנות calclaw-orchestration"
    echo "   מנסה להוסיף את הקובץ הראשי..."
    
    # צור את הקובץ הראשי אם חסר
    cat > src/main_orchestration.rs << 'EOF'
// 🚀 Calclaw Orchestration System - Main Entry Point

use calclaw::orchestration_api;

#[tokio::main]
async fn main() {
    println!("🚀 Calclaw Orchestration System");
    println!("================================");
    println!("");
    
    // הפעל את שרת האורקסטרציה
    orchestration_api::run_orchestration_server(3001).await;
}
EOF
    
    # עדכן את Cargo.toml
    if ! grep -q "name = \"calclaw-orchestration\"" Cargo.toml; then
        cat >> Cargo.toml << 'EOF'

[[bin]]
name = "calclaw-orchestration"
path = "src/main_orchestration.rs"
EOF
    fi
    
    # נסה לבנות שוב
    echo "🔄 בונה מחדש..."
    cargo build --release --bin calclaw-orchestration
fi

echo "✅ בנייה הושלמה"

# צור סקריפט הרצה
echo "📝 יוצר סקריפטי הרצה..."

cat > run_orchestration.sh << 'EOF'
#!/bin/bash

# 🚀 הרצת מנוע האורקסטרציה

set -e

echo "🤖 מפעיל את Calclaw Orchestration Engine..."
echo ""

# הפעל את מנוע האורקסטרציה
./target/release/calclaw-orchestration
EOF

chmod +x run_orchestration.sh

# צור סקריפט לדשבורד
cat > run_dashboard.sh << 'EOF'
#!/bin/bash

# 🌐 הרצת דשבורד האורקסטרציה

set -e

echo "📊 מפעיל את Orchestration Dashboard..."
echo ""

# בדוק אם Python זמין
if command -v python3 &> /dev/null; then
    echo "🚀 מפעיל שרת HTTP על פורט 3002..."
    python3 -m http.server 3002 --directory scripts
elif command -v python &> /dev/null; then
    echo "🚀 מפעיל שרת HTTP על פורט 3002..."
    python -m http.server 3002 --directory scripts
else
    echo "❌ Python לא מותקן"
    echo "   התקן Python 3 או פתח את הקובץ ישירות:"
    echo "   file://$(pwd)/scripts/orchestration_dashboard.html"
    exit 1
fi
EOF

chmod +x run_dashboard.sh

# צור סקריפט הרצה מלא
cat > start_orchestration_system.sh << 'EOF'
#!/bin/bash

# 🚀 הרצת כל מערכת האורקסטרציה

set -e

echo "🎯 Calclaw Orcheration System - Full Startup"
echo "============================================"
echo ""

ORCHESTRATION_PORT=3001
DASHBOARD_PORT=3002

# בדוק אם המנוע כבר רץ
if lsof -ti:$ORCHESTRATION_PORT > /dev/null 2>&1; then
    echo "⚠️  Orchestration engine כבר רץ על פורט $ORCHESTRATION_PORT"
    read -p "להמשיך בכל זאת? (y/n): " CONTINUE
    if [[ "$CONTINUE" != "y" && "$CONTINUE" != "Y" ]]; then
        echo "❌ ביטול"
        exit 0
    fi
fi

# הפעל את מנוע האורקסטרציה ברקע
echo "🤖 מפעיל את Orchestration Engine..."
./target/release/calclaw-orchestration &
ORCHESTRATION_PID=$!

echo "   ✅ PID: $ORCHESTRATION_PID"
echo "   🌐 API: http://localhost:$ORCHESTRATION_PORT"

# המתן שהמנוע יתחיל
echo "⏳ ממתין לאתחול המנוע..."
sleep 3

# בדוק אם המנוע רץ
if ! kill -0 $ORCHESTRATION_PID 2>/dev/null; then
    echo "❌ Orchestration engine נכשל בהפעלה"
    exit 1
fi

# הפעל את הדשבורד
echo ""
echo "📊 מפעיל את Orchestration Dashboard..."

if command -v python3 &> /dev/null; then
    python3 -m http.server $DASHBOARD_PORT --directory scripts > /dev/null 2>&1 &
elif command -v python &> /dev/null; then
    python -m http.server $DASHBOARD_PORT --directory scripts > /dev/null 2>&1 &
else
    echo "⚠️  Python לא זמין - הדשבורד לא יופעל"
    DASHBOARD_PID=""
fi

DASHBOARD_PID=$!
echo "   ✅ PID: $DASHBOARD_PID"
echo "   🌐 Dashboard: http://localhost:$DASHBOARD_PORT/orchestration_dashboard.html"

echo ""
echo "🎉 **המערכת מוכנה!**"
echo ""
echo "🔗 **קישורים חשובים:**"
echo "   • Dashboard: http://localhost:$DASHBOARD_PORT/orchestration_dashboard.html"
echo "   • API Status: http://localhost:$ORCHESTRATION_PORT/api/orchestration/status"
echo "   • API Health: http://localhost:$ORCHESTRATION_PORT/api/orchestration/health"
echo ""
echo "🛠️ **דוגמאות לשימוש ב-API:**"
echo "   curl http://localhost:$ORCHESTRATION_PORT/api/orchestration/status"
echo "   curl http://localhost:$ORCHESTRATION_PORT/api/orchestration/tasks"
echo "   curl -X POST http://localhost:$ORCHESTRATION_PORT/api/orchestration/tasks/execute-next"
echo ""
echo "📋 **ניהול המערכת:**"
echo "   • עצור את המערכת: kill $ORCHESTRATION_PID $DASHBOARD_PID"
echo "   • רק מנוע: ./run_orchestration.sh"
echo "   • רק דשבורד: ./run_dashboard.sh"
echo ""
echo "💡 **טיפים:**"
echo "   • רענן את הדשבורד אוטומטית כל 10 שניות"
echo "   • השתמש ב-API ליצירת tasks אוטומטית"
echo "   • עקוב אחר הביצועים במדדים בזמן אמת"
echo ""
echo "🤖 **המנוע מוכן לאורקסטרציה חכמה!**"

# המתן ל-Ctrl+C
trap 'echo ""; echo "🛑 עוצר את המערכת..."; kill $ORCHESTRATION_PID $DASHBOARD_PID 2>/dev/null; exit 0' INT

echo ""
echo "📝 לחץ Ctrl+C כדי לעצור את המערכת"
echo ""

wait
EOF

chmod +x start_orchestration_system.sh

# צור קובץ README
cat > ORCHESTRATION_README.md << 'EOF'
# 🚀 Calclaw Orchestration System

## 📖 מבוא

מערכת אורקסטרציה חכמה ל-Calclaw עם **state management**, **heartbeat monitoring**, ו**real-time dashboard**.

## 🏗️ ארכיטקטורה

```
┌─────────────────────────────────────────────┐
│            Orchestration Dashboard          │
│                 (Port: 3002)                │
└───────────────────┬─────────────────────────┘
                    │ HTTP
┌───────────────────▼─────────────────────────┐
│          Orchestration Engine API           │
│                 (Port: 3001)                │
└───────────────────┬─────────────────────────┘
                    │ Internal
┌───────────────────▼─────────────────────────┐
│            Orchestration Engine             │
│  • State Management                         │
│  • Task Queue                               │
│  • Heartbeat System                         │
│  • Performance Tracking                     │
│  • Learning Loop                            │
└───────────────────┬─────────────────────────┘
                    │ Integration
┌───────────────────▼─────────────────────────┐
│          Existing Calclaw Systems           │
│  • Cron Manager                             │
│  • Update System                            │
│  • Context Compactor                        │
└─────────────────────────────────────────────┘
```

## 🚀 התחלה מהירה

### אפשרות 1: הרצה מלאה (מומלץ)
```bash
./start_orchestration_system.sh
```

### אפשרות 2: רק מנוע
```bash
./run_orchestration.sh
```

### אפשרות 3: רק דשבורד
```bash
./run_dashboard.sh
```

## 🌐 ממשקים

### 📊 Dashboard
- **URL**: http://localhost:3002/orchestration_dashboard.html
- **תכונות**: ניטור בזמן אמת, שליטה במנוע, המלצות שיפור

### 🚀 REST API
- **Base URL**: http://localhost:3001/api/orchestration
- **Endpoints**: 40+ endpoints לניהול מלא

## 🔧 API Endpoints עיקריים

### 📊 Status & Health
- `GET /status` - סטטוס מנוע
- `GET /health` - בריאות מערכת
- `POST /heartbeat` - הפעלת heartbeat

### 📋 Task Management
- `GET /tasks` - כל המשימות
- `POST /tasks` - יצירת task חדש
- `POST /tasks/execute-next` - ביצוע task הבא
- `GET /tasks/queue` - תור משימות

### 🤖 Agent Management
- `GET /agent/status` - סטטוס סוכן
- `GET /agent/capabilities` - יכולות סוכן
- `GET /agent/memory` - זיכרון סוכן

### 📈 Performance & Learning
- `GET /performance` - מדדי ביצועים
- `GET /performance/history` - היסטוריית ביצועים
- `GET /learning` - סטטוס למידה
- `GET /recommendations` - המלצות שיפור

### 🔗 Integrations
- `GET /integrations/cron` - סטטוס cron
- `POST /integrations/cron/sync` - סנכרון cron
- `GET /integrations/updates` - סטטוס עדכונים

### 🎮 Engine Control
- `POST /engine/start` - הפעלת מנוע
- `POST /engine/stop` - עצירת מנוע
- `POST /engine/pause` - השהיית מנוע
- `POST /engine/resume` - חידוש מנוע
- `POST /engine/reset` - איפוס מנוע

## 📁 מבנה קבצים

```
calclaw/
├── src/
│   ├── orchestration_engine.rs          # מנוע אורקסטרציה
│   ├── orchestration_engine_part2.rs    # חלק 2 של המנוע
│   ├── orchestration_api.rs             # REST API
│   ├── orchestration_api_part2.rs       # חלק 2 של API
│   ├── orchestration_api_part3.rs       # חלק 3 של API
│   └── main_orchestration.rs            # נקודת כניסה
├── scripts/
│   ├── orchestration_dashboard.html     # דשבורד
│   ├── orchestration_dashboard_part2.html
│   ├── run_orchestration.sh             # סקריפט הרצה
│   ├── run_dashboard.sh                 # סקריפט דשבורד
│   └── start_orchestration_system.sh    # סקריפט מלא
└── ORCHESTRATION_README.md              # דוקומנטציה זו
```

## 🎯 דוגמאות שימוש

### יצירת task חדש
```bash
curl -X POST http://localhost:3001/api/orchestration/tasks \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Analyze Sales Data",
    "description": "Perform analysis on Q1 sales data",
    "capability": "Data Analysis",
    "priority": "High"
  }'
```

### ביצוע task
```bash
curl -X POST http://localhost:3001/api/orchestration/tasks/execute-next
```

### קבלת סטטוס
```bash
curl http://localhost:3001/api/orchestration/status
```

### ניטור בריאות
```bash
curl http://localhost:3001/api/orchestration/health
```

## 🔗 אינטגרציה עם Calclaw הקיים

### עם Cron Manager
```rust
// סנכרון אוטומטי עם cron jobs
engine.cron_integration.sync_cron_jobs().await;
```

### עם Update System
```rust
// בדיקת עדכונים אוטומטית
engine.update_integration.check_for_updates().await;
```

### עם Context Compactor
```rust
// דחיסת הקשר לניהול זיכרון
engine.agent_state.memory = context_compactor::compact_memory(memory);
```

## 📊 מדדי ביצועים

### 🎯 Success Rate
- אחוז הצלחה של tasks
- יעד: >90%

### ⚡ Efficiency Score
- מדד יעילות ביצוע
- יעד: >0.8

### 🧠 Learning Score
- מדד למידה מהניסיון
- יעד: >0.6

### 💓 Heartbeat Health
- בריאות מערכת
- יעד: Healthy

## 🚀 יתרונות המערכת

### ✅ מבוסס על קוד קיים
- משתמש במערכות Calclaw הקיימות
- פחות באגים, קל לתחזוקה

### ✅ Stateful Orchestration
- זיכרון בין ריצות
- למידה מתמדת
- התאמה דינמית

### ✅ ניטור מלא
- Health monitoring בזמן אמת
- Performance analytics
- Alerting system

### ✅ Scalability
- תור משימות לניהול עומסים
- Resource management
- Load balancing

## 🛠️ פתרון בעיות

### המנוע לא נבנה
```bash
# נקה ונסה שוב
cargo clean
cargo build --release --bin calclaw-orchestration
```

### פורט תפוס
```bash
# מצא את התהליך ועצור אותו
lsof -ti:3001 | xargs kill -9
lsof -ti:3002 | xargs kill -9
```

### Dashboard לא נטען
```bash
# פתח ישירות בדפדפן
file://$(pwd)/scripts/orchestration_dashboard.html
```

## 📞 תמיכה

- **GitHub**: https://github.com/nadavsimba24/calclaw
- **Issues**: דווח על באגים או בקשות תכונה
- **Discussions**: שאלות ודיונים

## 🎉 סיכום

**Calclaw Orchestration System** הוא מנוע אורקסטרציה חכם שמבוסס על המערכות הקיימות של Calclaw, עם יכולות מתקדמות של state management, heartbeat monitoring, ו-real-time dashboard.

**המנוע מוכן לאורקסטרציה חכמה של tasks, ניתוח ביצועים, ולמידה מתמדת!** 🚀
EOF

echo "✅ כל הקבצים נוצרו"

echo ""
echo "🎯echo "🎯 **המערכת מוכנה להפעלה!**"
echo ""
echo "📁 **קבצים שנוצרו:**"
echo "   • run_orchestration.sh - הרצת מנוע בלבד"
echo "   • run_dashboard.sh - הרצת דשבורד בלבד"
echo "   • start_orchestration_system.sh - הרצה מלאה (מומלץ)"
echo "   • ORCHESTRATION_README.md - דוקומנטציה מלאה"
echo ""
echo "🚀 **להפעלת המערכת המלאה:**"
echo "   ./start_orchestration_system.sh"
echo ""
echo "🌐 **לאחר ההפעלה:**"
echo "   • Dashboard: http://localhost:3002/orchestration_dashboard.html"
echo "   • API: http://localhost:3001/api/orchestration"
echo ""
echo "📚 **לקריאת הדוקומנטציה המלאה:**"
echo "   cat ORCHESTRATION_README.md"
echo ""
echo "🤖 **Calclaw Orchestration System מוכן לשימוש!**"

# הוסף את הקבצים ל-git
echo ""
echo "📦 מוסיף קבצים ל-git..."

git add \
  src/orchestration_api.rs \
  src/orchestration_api_part2.rs \
  src/orchestration_api_part3.rs \
  src/main_orchestration.rs \
  scripts/orchestration_dashboard.html \
  scripts/orchestration_dashboard_part2.html \
  scripts/run_orchestration.sh \
  scripts/run_dashboard.sh \
  scripts/start_orchestration_system.sh \
  ORCHESTRATION_README.md \
  2>/dev/null || true

echo "✅ הקבצים הוכנו להעלאה ל-git"
echo ""
echo "🔗 **להעלאה ל-GitHub:**"
echo "   git commit -m 'feat: Add complete orchestration system with API and dashboard'"
echo "   git push origin main"
echo ""
echo "🎉 **הפרויקט הושלם!**"