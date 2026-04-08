# 🦾 Calclaw עם Timeless Cal Squads - התקנה ובדיקה מלאה

## 🚀 מה חדש בגרסה 0.3.0

### ✨ פיצ'רים חדשים:
- **Timeless Cal Squads** - צוותי סוכני AI אוטונומיים
- **ירושה ארגונית** - סוכנים יורשים ידע מהארגון
- **עיבוד פגישות** - זיהוי החלטות ויצירת משימות אוטומטית
- **ממשק וובי מודרני** - דשבורד Timeless Squads
- **API מלא** - REST API לשילוב עם מערכות אחרות

### 🔒 יתרונות:
- **פרטיות מלאה** - כל המודלים מקומיים עם Ollama
- **ביצועים טובים** - Rust + async/await
- **תמיכה מלאה בעברית** - RTL, NLP, תרבות מקומית
- **אינטגרציה מלאה** - עם כל הפיצ'רים הקיימים של Calclaw

## 📋 דרישות מוקדמות

### 1. Ollama (AI מקומי)
```bash
# הורד והתקן Ollama
curl -fsSL https://ollama.com/install.sh | sh

# הפעל את שרת Ollama
ollama serve &

# הורד מודלים מומלצים
ollama pull phi3:mini      # קל ומהיר (2.2GB) - מומלץ להתחלה
ollama pull gemma2:9b      # חזק (5.4GB) - לראשי צוותים
ollama pull phi3:3.8b      # כללי (2.2GB) - לסוכנים מתקדמים
```

### 2. Rust (לבניית Calclaw)
```bash
# אם אין Rust מותקן
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# בדוק שההתקנה הצליחה
rustc --version
cargo --version
```

### 3. Python (לא חובה, רק לסקריפטים)
```bash
# בדוק אם Python 3 מותקן
python3 --version

# התקן אם חסר (Ubuntu/Debian)
sudo apt update
sudo apt install python3 python3-pip
```

## 🚀 התקנת Calclaw עם Timeless Squads

### אפשרות 1: התקנה מהירה (מומלץ)
```bash
# היכנס לתיקיית Calclaw
cd /home/erez/.openclaw/workspace/calclaw

# הרץ סקריפט ההתקנה המשודרג
chmod +x install_with_timeless.sh
./install_with_timeless.sh
```

### אפשרות 2: התקנה ידנית
```bash
cd /home/erez/.openclaw/workspace/calclaw

# שמור גיבוי של הקוד המקורי
cp src/main_backup.rs src/main_original.rs

# החלף בגרסה עם Timeless Squads
cp src/main.rs src/main_with_timeless.rs  # אם כבר קיים

# ודא שכל הקבצים הנדרשים קיימים
ls -la src/timeless_squads.rs  # צריך להיות קיים
ls -la src/ollama_simple.rs    # צריך להיות קיים

# עדכן את Cargo.toml לגרסה החדשה
cp Cargo.toml Cargo.old.toml   # גיבוי
# (הקובץ כבר מעודכן בגרסה 0.3.0)

# בנה את הפרויקט
cargo build --release

# הפעל את השרת
./target/release/calclaw &
```

### אפשרות 3: התקנה פשוטה (ללא Rust)
```bash
# השתמש בגרסת Python הפשוטה
cd /home/erez/.openclaw/workspace/calclaw
chmod +x simple_install.sh
./simple_install.sh
```

## 🔧 סקריפטי ניהול חדשים

### `start_calclaw_with_timeless.sh` - הפעלת המערכת המלאה
```bash
#!/bin/bash
echo "🚀 מתחיל Calclaw עם Timeless Cal Squads..."

# בדוק אם Ollama רץ
if ! curl -s http://localhost:11434/api/tags > /dev/null; then
    echo "❌ Ollama לא רץ. מפעיל..."
    ollama serve > /tmp/ollama_calclaw.log 2>&1 &
    sleep 5
fi

# בנה את Calclaw אם צריך
cd /home/erez/.openclaw/workspace/calclaw
cargo build --release

# הפעל את Calclaw
./target/release/calclaw > /tmp/calclaw_timeless.log 2>&1 &

echo "✅ Calclaw עם Timeless Squads רץ על http://localhost:3000"
echo "🚀 דשבורד: http://localhost:3000/timeless"
```

### `create_municipal_squad.sh` - יצירת צוות עירוני לדוגמה
```bash
#!/bin/bash
# יצירת צוות AI עירוני לדוגמה
curl -X POST http://localhost:3000/api/timeless/squads \
  -H 'Content-Type: application/json' \
  -d '{
    "name": "צוות AI עירוני",
    "description": "צוות לניהול עירוני חכם עם Mapillary, GIS, ואוטומציה",
    "settings": {
      "inheritance_enabled": true,
      "real_time_processing": true,
      "auto_task_creation": true,
      "allowed_integrations": ["mapillary", "gis", "municipal_apis"],
      "notification_channels": ["slack", "telegram"]
    }
  }'
```

### `test_timeless_api.sh` - בדיקת API של Timeless Squads
```bash
#!/bin/bash
echo "🧪 בודק Timeless Squads API..."

# בדוק סטטוס
curl -s http://localhost:3000/api/timeless/squads/status

# צור צוות לדוגמה
curl -X POST http://localhost:3000/api/timeless/squads \
  -H 'Content-Type: application/json' \
  -d '{
    "name": "צוות בדיקה",
    "description": "צוות לבדיקת המערכת",
    "settings": {
      "inheritance_enabled": true,
      "real_time_processing": false,
      "auto_task_creation": true,
      "allowed_integrations": [],
      "notification_channels": []
    }
  }'

echo "✅ בדיקות הושלמו"
```

## 🌐 ממשקים זמינים

לאחר ההפעלה, הממשקים הבאים זמינים:

### 🏠 דשבורדים:
- **דשבורד ראשי**: `http://localhost:3000`
- **Timeless Squads Dashboard**: `http://localhost:3000/timeless`
- **Ollama Dashboard**: `http://localhost:3000/ollama_dashboard.html` (HTML)

### 🔧 API endpoints:
- **בריאות המערכת**: `GET /api/health`
- **עיבוד עברית**: `POST /api/hebrew`
- **ניהול משתמשים**: `GET /api/users`
- **Ollama Health**: `GET /api/ollama/health`
- **Ollama Generate**: `POST /api/ollama/generate`

### 🚀 Timeless Squads API:
- **יצירת צוות**: `POST /api/timeless/squads`
- **שיחה עם סוכן**: `POST /api/timeless/agents/:agent_id/chat`
- **עיבוד פגישה**: `POST /api/timeless/squads/:squad_id/meeting`
- **סטטוס צוות**: `GET /api/timeless/squads/:squad_id/status`
- **הוספת ידע**: `POST /api/timeless/squads/:squad_id/knowledge`

## 📖 דוגמאות שימוש

### יצירת צוות עירוני:
```bash
curl -X POST http://localhost:3000/api/timeless/squads \
  -H 'Content-Type: application/json' \
  -d '{
    "name": "צוות AI עירוני",
    "description": "צוות לניהול עירוני חכם",
    "settings": {
      "inheritance_enabled": true,
      "real_time_processing": true,
      "auto_task_creation": true,
      "allowed_integrations": ["mapillary", "gis"],
      "notification_channels": ["slack"]
    }
  }'
```

### שיחה עם סוכן:
```bash
curl -X POST http://localhost:3000/api/timeless/agents/AGENT_ID/chat \
  -H 'Content-Type: application/json' \
  -d '{
    "message": "תנתח את נתוני האשפה מהשבוע",
    "user_id": "user123"
  }'
```

### עיבוד פגישה:
```bash
curl -X POST http://localhost:3000/api/timeless/squads/SQUAD_ID/meeting \
  -H 'Content-Type: application/json' \
  -d '{
    "transcript": "סיכמנו לשלוח הצעת מחיר עד יום חמישי ולהכין דוח על מערכת ההשקייה",
    "platform": "zoom"
  }'
```

## 🎯 שילוב עם מערכות קיימות

### עם Mapillary Vision Search:
```bash
# שימוש ב-Timeless Squads לניתוח תמונות
curl -X POST http://localhost:3000/api/timeless/agents/ANALYST_ID/chat \
  -H 'Content-Type: application/json' \
  -d '{
    "message": "תנתח את תמונות Mapillary מרחוב דיזנגוף בתל אביב וזהה בעיות תשתית",
    "user_id": "municipal_user"
  }'
```

### עם Telegram Bot:
```bash
# שליחת התראות מצוות Timeless ל-Telegram
# (דוגמה - יש לשלב עם Telegram bot קיים)
```

### עם מערכת Cron של Calclaw:
```bash
# יצירת cron job אוטומטי מהחלטות פגישה
# המערכת יוצרת משימות אוטומטית מהחלטות פגישות
```

## 🔍 פתרון בעיות

### בעיה: Ollama לא רץ
```bash
# בדוק אם Ollama מותקן
which ollama

# הפעל ידנית
ollama serve &

# בדוק אם השרת רץ
curl http://localhost:11434/api/tags
```

### בעיה: Calclaw לא נבנה
```bash
# בדוק אם Rust מותקן
rustc --version

# נקה ונסה שוב
cd /home/erez/.openclaw/workspace/calclaw
cargo clean
cargo build
```

### בעיה: פורט 3000 תפוס
```bash
# חפש תהליכים שתפסו את הפורט
sudo lsof -i :3000

# עצור את התהליך אם צריך
pkill -f "calclaw"
```

### בעיה: שגיאות בקוד
```bash
# בדוק שגיאות קומפילציה
cd /home/erez/.openclaw/workspace/calclaw
cargo check

# תיקן שגיאות ידנית
# (ראה את השגיאות בפלט)
```

## 📈 סקיילביליות

### להרחבה ליותר סוכנים:
1. **הוסף סוגי סוכנים חדשים** ב-`src/timeless_squads.rs`
2. **הגדל את כמות המודלים** ב-Ollama
3. **שפר את ניהול הזיכרון** של סוכנים

### לשילוב עם ענן (אופציונלי):
1. **שמור ידע ארגוני** במסד נתונים מרוחק
2. **סנכרן סוכנים** בין מספר שרתים
3. **השתמש במודלים גדולים יותר** במידת הצורך

### לפרודקשן:
1. **השתמש ב-systemd** לניהול שירותים
2. **הגדר logging מתקדם**
3. **הוסף monitoring ו-alerts**
4. **הגדר backup אוטומטי לידע ארגוני**

## 📞 תמיכה וקהילה

### משאבים:
- **קוד מקור**: `/home/erez/.openclaw/workspace/calclaw`
- **תיעוד**: קובץ זה + הערות בקוד
- **דוגמאות**: בקבצי `examples/` (אם קיימים)

### דיווח באגים:
1. **שמור את השגיאה** מהלוגים
2. **צור issue** עם פרטים מלאים
3. **צרף דוגמת קוד** שמייצרת את הבעיה

### בקשות פיצ'רים:
1. **תאר את הפיצ'ר** המבוקש
2. **הסבר את השימוש** המקרה
3. **הצע מימוש** אם אפשר

## 🎉 התחלה מהירה

להתחלה מהירה, הרץ את הפקודות הבאות:

```bash
# 1. התקן Ollama ומודלים
curl -fsSL https://ollama.com/install.sh | sh
ollama serve &
ollama pull phi3:mini

# 2. התקן Rust אם חסר
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 3. התקן את Calclaw עם Timeless Squads
cd /home/erez/.openclaw/workspace/calclaw
chmod +x install_with_timeless.sh
./install_with_timeless.sh

# 4. הפעל את המערכת
./start_calclaw_with_timeless.sh

# 5. פתח בדפדפן
#    http://localhost:3000/timeless
```

**מזל טוב!** 🎉 יש לך כעת Calclaw עם Timeless Cal Squads - מערכת AI ארגונית מלאה, פרטית, ובעברית!

---

*גרסה: 0.3.0 עם Timeless Squads | עודכן: 2026-04-08 | מחבר: Calclaw Team*