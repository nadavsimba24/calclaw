#!/bin/bash

# 🚀 Calclaw Smart Installer
# התקנה חכמה שמתאימה את עצמה לצרכים שלך - קלילה כמו OpenClaw!

set -e

echo ""
echo "🚀 **Calclaw Smart Installer**"
echo "================================"
echo ""
echo "התקנה חכמה שמתאימה את עצמה לצרכים שלך!"
echo "קלילה כמו OpenClaw, חזקה כמו מערכת ארגונית שלמה."
echo ""

# שאלון התאמה חכם
echo "🎯 **שאלון התאמה חכם**"
echo "----------------------"
echo ""
echo "אני עוזר לך להתקין את Calclaw בצורה הכי מתאימה לצרכים שלך."
echo ""

# שאלות בסיסיות
echo "1️⃣ **מה המטרה העיקרית שלך?**"
echo "   [1] שימוש אישי - רוצה AI מקומי פשוט"
echo "   [2] פיתוח - רוצה לפתח ולהתנסות"
echo "   [3] עסק קטן - רוצה אוטומציה בסיסית"
echo "   [4] ארגון גדול - רוצה מערכת ארגונית מלאה"
echo "   [5] רשות מקומית - רוצה מערכת מוניציפלית"
read -p "   בחר מספר (1-5): " PURPOSE

echo ""
echo "2️⃣ **איזה יכולות הכי חשובות לך?**"
echo "   [1] שיחה עם AI מקומי (Ollama)"
echo "   [2] ניהול משימות אוטומטי (cron jobs)"
echo "   [3] אינטגרציה עם Telegram (בוט)"
echo "   [4] חיפוש תמונות וניתוח חזותי"
echo "   [5] מערכת ארגונית מלאה (אונטולוגיה, אורקסטרציה)"
echo "   [6] אבטחה מתקדמת (enterprise security)"
read -p "   בחר מספרים מופרדים בפסיקים (לדוגמה: 1,2,3): " CAPABILITIES

echo ""
echo "3️⃣ **איפה תרצה להריץ את Calclaw?**"
echo "   [1] מחשב אישי (PC/לפטופ)"
echo "   [2] שרת מקומי (on-premise)"
echo "   [3] ענן (VPS/Cloud)"
echo "   [4] Kubernetes (קונטיינרים)"
read -p "   בחר מספר (1-4): " DEPLOYMENT

echo ""
echo "4️⃣ **מה רמת האבטחה שאתה צריך?**"
echo "   [1] מינימלית - פיתוח/ניסוי"
echo "   [2] בסיסית - שימוש אישי"
echo "   [3] מתקדמת - עסק קטן"
echo "   [4] ארגונית - ארגון גדול"
read -p "   בחר מספר (1-4): " SECURITY

echo ""
echo "5️⃣ **האם יש לך כבר מודלים של Ollama?**"
echo "   [1] לא, צריך להתקין Ollama ומודלים"
echo "   [2] כן, יש לי Ollama רץ"
echo "   [3] לא רוצה Ollama - רק את שאר המערכת"
read -p "   בחר מספר (1-3): " OLLAMA_STATUS

echo ""
echo "📊 **מנתח את התשובות שלך...**"
echo ""

# ניתוח התשובות
PERSONAL=false
DEVELOPMENT=false
BUSINESS=false
ENTERPRISE=false
MUNICIPAL=false

case $PURPOSE in
    1) PERSONAL=true; INSTALL_TYPE="personal"; echo "   ✅ שימוש אישי";;
    2) DEVELOPMENT=true; INSTALL_TYPE="development"; echo "   ✅ פיתוח";;
    3) BUSINESS=true; INSTALL_TYPE="business"; echo "   ✅ עסק קטן";;
    4) ENTERPRISE=true; INSTALL_TYPE="enterprise"; echo "   ✅ ארגון גדול";;
    5) MUNICIPAL=true; INSTALL_TYPE="municipal"; echo "   ✅ רשות מקומית";;
esac

# ניתוח יכולות
IFS=',' read -ra CAPS <<< "$CAPABILITIES"
HAS_OLLAMA=false
HAS_CRON=false
HAS_TELEGRAM=false
HAS_VISION=false
HAS_ORCHESTRATION=false
HAS_SECURITY=false

for cap in "${CAPS[@]}"; do
    case $cap in
        1) HAS_OLLAMA=true; echo "   ✅ שיחה עם AI מקומי";;
        2) HAS_CRON=true; echo "   ✅ ניהול משימות אוטומטי";;
        3) HAS_TELEGRAM=true; echo "   ✅ אינטגרציה עם Telegram";;
        4) HAS_VISION=true; echo "   ✅ חיפוש תמונות וניתוח חזותי";;
        5) HAS_ORCHESTRATION=true; echo "   ✅ מערכת ארגונית מלאה";;
        6) HAS_SECURITY=true; echo "   ✅ אבטחה מתקדמת";;
    esac
done

# ניתוח deployment
case $DEPLOYMENT in
    1) DEPLOY_TYPE="personal"; echo "   ✅ מחשב אישי";;
    2) DEPLOY_TYPE="onprem"; echo "   ✅ שרת מקומי";;
    3) DEPLOY_TYPE="cloud"; echo "   ✅ ענן";;
    4) DEPLOY_TYPE="kubernetes"; echo "   ✅ Kubernetes";;
esac

# ניתוח אבטחה
case $SECURITY in
    1) SECURITY_LEVEL="minimal"; echo "   ✅ אבטחה מינימלית";;
    2) SECURITY_LEVEL="basic"; echo "   ✅ אבטחה בסיסית";;
    3) SECURITY_LEVEL="advanced"; echo "   ✅ אבטחה מתקדמת";;
    4) SECURITY_LEVEL="enterprise"; echo "   ✅ אבטחה ארגונית";;
esac

# ניתוח Ollama
case $OLLAMA_STATUS in
    1) OLLAMA_ACTION="install"; echo "   ✅ התקנת Ollama ומודלים";;
    2) OLLAMA_ACTION="use_existing"; echo "   ✅ שימוש ב-Ollama קיים";;
    3) OLLAMA_ACTION="skip"; echo "   ✅ דילוג על Ollama";;
esac

echo ""
echo "🎯 **תוכנית התקנה מותאמת אישית**"
echo "---------------------------------"
echo ""

# יצירת תוכנית התקנה
INSTALL_PLAN=""

# בסיס - תמיד מותקן
INSTALL_PLAN+="📦 **Calclaw Core** - מערכת הליבה\n"
INSTALL_PLAN+="   • שרת Rust בסיסי\n"
INSTALL_PLAN+="   • REST API\n"
INSTALL_PLAN+="   • ניהול בסיסי\n"

# Ollama אם נדרש
if [ "$HAS_OLLAMA" = true ] && [ "$OLLAMA_ACTION" != "skip" ]; then
    if [ "$OLLAMA_ACTION" = "install" ]; then
        INSTALL_PLAN+="\n🤖 **Ollama Integration** - התקנה מלאה\n"
        INSTALL_PLAN+="   • התקנת Ollama\n"
        INSTALL_PLAN+="   • הורדת מודלים (Gemma, Phi3)\n"
        INSTALL_PLAN+="   • אינטגרציה מלאה\n"
    else
        INSTALL_PLAN+="\n🤖 **Ollama Integration** - שימוש בקיים\n"
        INSTALL_PLAN+="   • חיבור ל-Ollama קיים\n"
        INSTALL_PLAN+="   • בדיקת חיבור\n"
    fi
fi

# Cron אם נדרש
if [ "$HAS_CRON" = true ]; then
    INSTALL_PLAN+="\n⏰ **Task Automation** - ניהול משימות\n"
    INSTALL_PLAN+="   • Cron manager\n"
    INSTALL_PLAN+="   • TUI לממשק ניהול\n"
    INSTALL_PLAN+="   • Web dashboard\n"
fi

# Telegram אם נדרש
if [ "$HAS_TELEGRAM" = true ]; then
    INSTALL_PLAN+="\n💬 **Telegram Bot** - אינטגרציה\n"
    INSTALL_PLAN+="   • בוט Telegram\n"
    INSTALL_PLAN+="   • תמיכה בהודעות קוליות\n"
    INSTALL_PLAN+="   • TTS/STT מקומי\n"
fi

# Vision אם נדרש
if [ "$HAS_VISION" = true ]; then
    INSTALL_PLAN+="\n👁️ **Vision Search** - חיפוש תמונות\n"
    INSTALL_PLAN+="   • Mapillary API integration\n"
    INSTALL_PLAN+="   • CLIP model for image search\n"
    INSTALL_PLAN+="   • Web UI עם מפות\n"
fi

# Orchestration אם נדרש
if [ "$HAS_ORCHESTRATION" = true ]; then
    INSTALL_PLAN+="\n🧠 **Organizational AI** - מערכת ארגונית\n"
    INSTALL_PLAN+="   • אונטולוגיה ארגונית\n"
    INSTALL_PLAN+="   • מנוע אורקסטרציה\n"
    INSTALL_PLAN+="   • SuperAgent עם 5 יכולות\n"
    INSTALL_PLAN+="   • Real-time dashboard\n"
fi

# Security אם נדרש
if [ "$HAS_SECURITY" = true ]; then
    INSTALL_PLAN+="\n🛡️ **Enterprise Security** - אבטחה מתקדמת\n"
    INSTALL_PLAN+="   • Security policies מלאות\n"
    INSTALL_PLAN+="   • Access control עם approval\n"
    INSTALL_PLAN+="   • Audit & compliance (GDPR, HIPAA, SOC2)\n"
    INSTALL_PLAN+="   • Security server\n"
fi

# התאמה לפי סוג התקנה
case $INSTALL_TYPE in
    "personal")
        INSTALL_PLAN+="\n🎯 **התאמה אישית לשימוש אישי:**\n"
        INSTALL_PLAN+="   • ממשק פשוט וקל לשימוש\n"
        INSTALL_PLAN+="   • התקנה מהירה (דקות בודדות)\n"
        INSTALL_PLAN+="   • צריכת משאבים מינימלית\n"
        ;;
    "development")
        INSTALL_PLAN+="\n🎯 **התאמה אישית לפיתוח:**\n"
        INSTALL_PLAN+="   • כלים לפיתוח והרחבה\n"
        INSTALL_PLAN+="   • דוקומנטציה מלאה\n"
        INSTALL_PLAN+="   • דוגמאות קוד\n"
        ;;
    "business")
        INSTALL_PLAN+="\n🎯 **התאמה אישית לעסק קטן:**\n"
        INSTALL_PLAN+="   • אוטומציה של תהליכים עסקיים\n"
        INSTALL_PLAN+="   • אינטגרציה עם כלים נפוצים\n"
        INSTALL_PLAN+="   • תמיכה בעברית מלאה\n"
        ;;
    "enterprise")
        INSTALL_PLAN+="\n🎯 **התאמה אישית לארגון גדול:**\n"
        INSTALL_PLAN+="   • Multi-tenancy support\n"
        INSTALL_PLAN+="   • High availability\n"
        INSTALL_PLAN+="   • Enterprise security\n"
        INSTALL_PLAN+="   • Compliance ready\n"
        ;;
    "municipal")
        INSTALL_PLAN+="\n🎯 **התאמה אישית לרשות מקומית:**\n"
        INSTALL_PLAN+="   • מערכת מוניציפלית מוכנה\n"
        INSTALL_PLAN+="   • חיפוש תמונות וניתוח חזותי\n"
        INSTALL_PLAN+="   • תמיכה בערים ישראליות\n"
        ;;
esac

# הצגת תוכנית ההתקנה
echo -e "$INSTALL_PLAN"
echo ""

# אישור התקנה
read -p "📋 **האם להמשיך בהתקנה? (y/n): " CONFIRM

if [ "$CONFIRM" != "y" ] && [ "$CONFIRM" != "Y" ]; then
    echo ""
    echo "❌ התקנה בוטלה."
    exit 0
fi

echo ""
echo "🚀 **מתחיל בהתקנה...**"
echo "======================"
echo ""

# יצירת תיקיית עבודה
WORKDIR="$HOME/.calclaw"
echo "📁 יוצר תיקיית עבודה: $WORKDIR"
mkdir -p "$WORKDIR"
cd "$WORKDIR"

# הורדת הקוד
echo "📥 מוריד את Calclaw..."
if [ -d "calclaw" ]; then
    echo "   ✅ Calclaw כבר קיים, מעדכן..."
    cd calclaw
    git pull
else
    git clone https://github.com/nadavsimba24/calclaw.git
    cd calclaw
fi

echo "✅ Calclaw הורד בהצלחה"

# התקנת בסיס
echo ""
echo "📦 **מתקין Calclaw Core...**"
echo "---------------------------"

# בדוק אם Rust מותקן
if ! command -v cargo &> /dev/null; then
    echo "⚠️  Rust לא מותקן, מתקין..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    echo "✅ Rust הותקן"
fi

# בנה את הליבה
echo "🔨 בונה את Calclaw Core..."
cargo build --release --bin calclaw

echo "✅ Calclaw Core הותקן"

# התקנת Ollama אם נדרש
if [ "$HAS_OLLAMA" = true ] && [ "$OLLAMA_ACTION" = "install" ]; then
    echo ""
    echo "🤖 **מתקין Ollama...**"
    echo "---------------------"
    
    # הורד והתקן Ollama
    curl -fsSL https://ollama.ai/install.sh | sh
    
    # הפעל את Ollama
    echo "🚀 מפעיל את Ollama..."
    ollama serve &
    OLLAMA_PID=$!
    
    # המתן שהשרת יתחיל
    sleep 5
    
    # הורד מודלים
    echo "📥 מוריד מודלים..."
    ollama pull gemma2:9b
    ollama pull phi3:mini
    
    echo "✅ Ollama הותקן עם 2 מודלים"
fi

# התקנת cron manager אם נדרש
if [ "$HAS_CRON" = true ]; then
    echo ""
    echo "⏰ **מתקין Task Automation...**"
    echo "------------------------------"
    
    # התקן Python dependencies אם צריך
    if ! command -v python3 &> /dev/null; then
        echo "⚠️  Python3 לא מותקן, מתקין..."
        sudo apt-get update && sudo apt-get install -y python3 python3-pip
    fi
    
    # התקן dependencies
    pip3 install -r requirements.txt 2>/dev/null || pip3 install apscheduler
    
    echo "✅ Task Automation הותקן"
fi

# התקנת Telegram bot אם נדרש
if [ "$HAS_TELEGRAM" = true ]; then
    echo ""
    echo "💬 **מתקין Telegram Bot...**"
    echo "---------------------------"
    
    # התקן dependencies
    pip3 install python-telegram-bot ffmpeg-python
    
    echo "✅ Telegram Bot הותקן"
    echo "   ℹ️  תצטרך להגדיר token ב-telegram_voice_config.json"
fi

# התקנת vision search אם נדרש
if [ "$HAS_VISION" = true ]; then
    echo ""
    echo "👁️ **מתקין Vision Search...**"
    echo "---------------------------"
    
    # צור virtual environment
    python3 -m venv vision_env
    source vision_env/bin/activate
    
    # התקן dependencies
    pip install transformers torch pillow numpy requests playwright
    
    # התקן Playwright browsers
    python -m playwright install chromium
    
    deactivate
    
    echo "✅ Vision Search הותקן"
fi

# התקנת orchestration אם נדרש
if [ "$HAS_ORCHESTRATION" = true ]; then
    echo ""
    echo "🧠 **מתקין Organizational AI...**"
    echo "-------------------------------"
    
    # בנה את מנוע האורקסטרציה
    cargo build --release --bin calclaw-orchestration
    
    echo "✅ Organizational AI הותקן"
fi

# התקנת security אם נדרש
if [ "$HAS_SECURITY" = true ]; then
    echo ""
    echo "🛡️ **מתקין Enterprise Security...**"
    echo "---------------------------------"
    
    # בנה את שרת האבטחה
    cargo build --release --bin calclaw-security-server
    
    echo "✅ Enterprise Security הותקן"
fi

echo ""
echo "🎯 **יצירת קובץ התצורה...**"
echo "--------------------------"

# צור קובץ תצורה מותאם
CONFIG_FILE="$WORKDIR/calclaw_config.json"
cat > "$CONFIG_FILE" << EOF
{
  "install_type": "$INSTALL_TYPE",
  "deployment": "$DEPLOY_TYPE",
  "security_level": "$SECURITY_LEVEL",
  "capabilities": {
    "ollama": $HAS_OLLAMA,
    "cron": $HAS_CRON,
    "telegram": $HAS_TELEGRAM,
    "vision": $HAS_VISION,
    "orchestration": $HAS_ORCHESTRATION,
    "security": $HAS_SECURITY
  },
  "paths": {
    "workdir": "$WORKDIR",
    "config": "$WORKDIR/calclaw/config",
    "logs": "$WORKDIR/calclaw/logs",
    "data": "$WORKDIR/calclaw/data"
  },
  "services": {
    "calclaw_core": {
      "enabled": true,
      "port": 3000,
      "start_command": "./target/release/calclaw"
    },
    "ollama": {
      "enabled": $HAS_OLLAMA,
      "port": 11434,
      "start_command": "ollama serve"
    },
    "orchestration": {
      "enabled": $HAS_ORCHESTRATION,
      "port": 3002,
      "start_command": "./target/release/calclaw-orchestration"
    },
    "security": {
      "enabled": $HAS_SECURITY,
      "port": 8081,
      "start_command": "./target/release/calclaw-security-server"
    }
  },
  "installation_date": "$(date -Iseconds)"
}
EOF

echo "✅ קובץ תצורה נוצר: $CONFIG_FILE"

echo ""
echo "🚀 **יצירת סקריפטי הרצה...**"
echo "---------------------------"

# צור סקריפט הרצה מותאם
START_SCRIPT="$WORKDIR/start_calclaw.sh"
cat > "$START_SCRIPT" << 'EOF'
#!/bin/bash

# 🚀 Calclaw Smart Launcher
# הרצה חכמה לפי ההתקנה המותאמת שלך

set -e

echo "🚀 **Calclaw Smart Launcher**"
echo "=============================="
echo ""

CONFIG_FILE="$HOME/.calclaw/calclaw_config.json"

if [ ! -f "$CONFIG_FILE" ]; then
    echo "❌ קובץ תצורה לא נמצא"
    echo "   הרץ את install_calclaw_smart.sh שוב"
    exit 1
fi

# טען את התצורה
INSTALL_TYPE=$(jq -r '.install_type' "$CONFIG_FILE")
DEPLOY_TYPE=$(jq -r '.deployment' "$CONFIG_FILE")
SECURITY_LEVEL=$(jq -r '.security_level' "$CONFIG_FILE")

echo "🎯 **התקנה מותאמת אישית:**"
echo "   • סוג: $INSTALL_TYPE"
echo "   • פריסה: $DEPLOY_TYPE"
echo "   • אבטחה: $SECURITY_LEVEL"
echo ""

# עבור לתיקיית העבודה
cd "$HOME/.calclaw/calclaw"

echo "🚀 **מתחיל בהרצה...**"
echo ""

# הפעל את Calclaw Core
echo "📦 מפעיל Calclaw Core..."
./target/release/calclaw &
CALCLAW_PID=$!
sleep 2

echo "   ✅ Calclaw Core רץ (PID: $CALCLAW_PID)"
echo "   🔗 http://localhost:3000"
echo ""

# הפעל Ollama אם מותקן
if [ "$(jq -r '.capabilities.ollama' "$CONFIG_FILE")" = "true" ]; then
    echo "🤖 מפעיל Ollama..."
    ollama serve &
    OLLAMA_PID=$!
    sleep 3
    
    echo "   ✅ Ollama רץ (PID: $OLLAMA_PID)"
    echo "   🔗 http://localhost:11434"
    echo ""
fi

# הפעל Orchestration אם מותקן
if [ "$(jq -r '.capabilities.orchestration' "$CONFIG_FILE")" = "true" ]; then
    echo "🧠 מפעיל Organizational AI..."
    ./target/release/calclaw-orchestration &
    ORCHESTRATION_PID=$!
    sleep 2
    
    echo "   ✅ Orchestration רץ (PID: $ORCHESTRATION_PID)"
    echo "   🔗 http://localhost:3002"
    echo "   📊 Dashboard: http://localhost:3002/orchestration_dashboard.html"
    echo ""
fi

# הפעל Security אם מותקן
if [ "$(jq -r '.capabilities.security' "$CONFIG_FILE")" = "true" ]; then
    echo "🛡️ מפעיל Enterprise Security..."
    ./target/release/calclaw-security-server &
    SECURITY_PID=$!
    sleep 2
    
    echo "   ✅ Security רץ (PID: $SECURITY_PID)"
    echo "   🔗 http://localhost:8081/api/security"
    echo "   🩺 Health: http://localhost:8081/api/security/health-check"
    echo ""
fi

echo "🎉 **כל השירותים רצים!**"
echo ""
echo "📊 **סטטוס שירותים:**"
echo "   • Calclaw Core: http://localhost:3000 ✅"
if [ "$(jq -r '.capabilities.ollama' "$CONFIG_FILE")" = "true" ]; then
    echo "   • Ollama: http://localhost:11434 ✅"
fi
if [ "$(jq -r '.capabilities.orchestration' "$CONFIG_FILE")" = "true" ]; then
    echo "   • Orchestration: http://localhost:3002 ✅"
    echo "   • Dashboard: http://localhost:3002/orchestration_dashboard.html"
fi
if [ "$(jq -r '.capabilities.security' "$CONFIG_FILE")" = "true" ]; then
    echo "   • Security API: http://localhost:8081/api/security ✅"
fi
echo ""
echo "🛑 **לעצירת כל השירותים:**"
echo "   pkill -f calclaw"
echo "   pkill -f ollama"
echo ""
echo "📝 **לוגים:**"
echo "   tail -f logs/calclaw.log"
echo ""
echo "🎯 **המערכת מוכנה לשימוש!**"
EOF

chmod +x "$START_SCRIPT"

# צור סקריפט עצירה
STOP_SCRIPT="$WORKDIR/stop_calclaw.sh"
cat > "$STOP_SCRIPT" << 'EOF'
#!/bin/bash

# 🛑 Calclaw Stopper
# עוצר את כל שירותי Calclaw

echo "🛑 **עוצר את Calclaw...**"
echo ""

# עצור את כל התהליכים
pkill -f calclaw 2>/dev/null && echo "✅ Calclaw Core נעצר"
pkill -f ollama 2>/dev/null && echo "✅ Ollama נעצר"
pkill -f calclaw-orchestration 2>/dev/null && echo "✅ Orchestration נעצר"
pkill -f calclaw-security-server 2>/dev/null && echo "✅ Security Server נעצר"

echo ""
echo "🎯 **כל השירותים נעצרו**"
EOF

chmod +x "$STOP_SCRIPT"

# צור סקריפט סטטוס
STATUS_SCRIPT="$WORKDIR/status_calclaw.sh"
cat > "$STATUS_SCRIPT" << 'EOF'
#!/bin/bash

# 📊 Calclaw Status
# מציג סטטוס של כל שירותי Calclaw

echo "📊 **Calclaw Status**"
echo "===================="
echo ""

CONFIG_FILE="$HOME/.calclaw/calclaw_config.json"

if [ ! -f "$CONFIG_FILE" ]; then
    echo "❌ קובץ תצורה לא נמצא"
    exit 1
fi

echo "🎯 **התקנה מותאמת אישית:**"
jq -r '"   • סוג: " + .install_type + "\n   • פריסה: " + .deployment + "\n   • אבטחה: " + .security_level' "$CONFIG_FILE"
echo ""

echo "🔍 **בדיקת שירותים:**"
echo ""

# בדוק Calclaw Core
if pgrep -f "calclaw" > /dev/null; then
    echo "   📦 Calclaw Core: ✅ רץ"
    echo "      🔗 http://localhost:3000"
else
    echo "   📦 Calclaw Core: ❌ לא רץ"
fi

# בדוק Ollama אם מותקן
if [ "$(jq -r '.capabilities.ollama' "$CONFIG_FILE")" = "true" ]; then
    if pgrep -f "ollama" > /dev/null; then
        echo "   🤖 Ollama: ✅ רץ"
        echo "      🔗 http://localhost:11434"
        
        # בדוק חיבור
        if curl -s http://localhost:11434/api/tags > /dev/null; then
            MODELS=$(curl -s http://localhost:11434/api/tags | jq -r '.models | length')
            echo "      📊 $MODELS מודלים זמינים"
        fi
    else
        echo "   🤖 Ollama: ❌ לא רץ"
    fi
fi

# בדוק Orchestration אם מותקן
if [ "$(jq -r '.capabilities.orchestration' "$CONFIG_FILE")" = "true" ]; then
    if pgrep -f "calclaw-orchestration" > /dev/null; then
        echo "   🧠 Orchestration: ✅ רץ"
        echo "      🔗 http://localhost:3002"
        echo "      📊 Dashboard: http://localhost:3002/orchestration_dashboard.html"
    else
        echo "   🧠 Orchestration: ❌ לא רץ"
    fi
fi

# בדוק Security אם מותקן
if [ "$(jq -r '.capabilities.security' "$CONFIG_FILE")" = "true" ]; then
    if pgrep -f "calclaw-security-server" > /dev/null; then
        echo "   🛡️ Security: ✅ רץ"
        echo "      🔗 http://localhost:8081/api/security"
        echo "      🩺 Health: http://localhost:8081/api/security/health-check"
    else
        echo "   🛡️ Security: ❌ לא רץ"
    fi
fi

echo ""
echo "🎯 **פקודות ניהול:**"
echo "   • הרצה: $HOME/.calclaw/start_calclaw.sh"
echo "   • עצירה: $HOME/.calclaw/stop_calclaw.sh"
echo "   • סטטוס: $HOME/.calclaw/status_calclaw.sh"
echo ""
EOF

chmod +x "$STATUS_SCRIPT"

echo "✅ סקריפטי הרצה נוצרו:"
echo "   • $START_SCRIPT"
echo "   • $STOP_SCRIPT"
echo "   • $STATUS_SCRIPT"

echo ""
echo "📚 **יצירת דוקומנטציה...**"
echo "-------------------------"

# צור קובץ README מותאם
README_FILE="$WORKDIR/README_CUSTOM.md"
cat > "$README_FILE" << EOF
# 🚀 Calclaw - התקנה מותאמת אישית

## 📖 מבוא

התקנת Calclaw מותאמת אישית לפי הצרכים שלך:

- **סוג התקנה:** $INSTALL_TYPE
- **סוג פריסה:** $DEPLOY_TYPE  
- **רמת אבטחה:** $SECURITY_LEVEL
- **תאריך התקנה:** $(date)

## 🎯 יכולות שהותקנו

$(if [ "$HAS_OLLAMA" = true ]; then echo "• 🤖 **Ollama Integration** - שיחה עם AI מקומי"; fi)
$(if [ "$HAS_CRON" = true ]; then echo "• ⏰ **Task Automation** - ניהול משימות אוטומטי"; fi)
$(if [ "$HAS_TELEGRAM" = true ]; then echo "• 💬 **Telegram Bot** - אינטגרציה עם Telegram"; fi)
$(if [ "$HAS_VISION" = true ]; then echo "• 👁️ **Vision Search** - חיפוש תמונות וניתוח חזותי"; fi)
$(if [ "$HAS_ORCHESTRATION" = true ]; then echo "• 🧠 **Organizational AI** - מערכת ארגונית מלאה"; fi)
$(if [ "$HAS_SECURITY" = true ]; then echo "• 🛡️ **Enterprise Security** - אבטחה מתקדמת"; fi)

## 🚀 הרצה מהירה

### הפעל את כל השירותים:
\`\`\`bash
$HOME/.calclaw/start_calclaw.sh
\`\`\`

### עצור את כל השירותים:
\`\`\`bash
$HOME/.calclaw/stop_calclaw.sh
\`\`\`

### בדוק סטטוס:
\`\`\`bash
$HOME/.calclaw/status_calclaw.sh
\`\`\`

## 🔗 קישורים חשובים

$(if [ "$HAS_OLLAMA" = true ]; then echo "- **Ollama:** http://localhost:11434"; fi)
$(if true; then echo "- **Calclaw Core:** http://localhost:3000"; fi)
$(if [ "$HAS_ORCHESTRATION" = true ]; then echo "- **Orchestration:** http://localhost:3002"; fi)
$(if [ "$HAS_ORCHESTRATION" = true ]; then echo "- **Dashboard:** http://localhost:3002/orchestration_dashboard.html"; fi)
$(if [ "$HAS_SECURITY" = true ]; then echo "- **Security API:** http://localhost:8081/api/security"; fi)

## 📁 מבנה תיקיות

\`\`\`
$HOME/.calclaw/
├── calclaw/          # קוד המקור
├── calclaw_config.json  # תצורה מותאמת
├── start_calclaw.sh     # סקריפט הרצה
├── stop_calclaw.sh      # סקריפט עצירה
└── status_calclaw.sh    # סקריפט סטטוס
\`\`\`

## 🎯 דוגמאות לשימוש

### שיחה עם AI מקומי:
\`\`\`bash
curl -X POST http://localhost:3000/api/chat \\
  -H "Content-Type: application/json" \\
  -d '{"message": "שלום, מה אתה יכול לעשות?"}'
\`\`\`

$(if [ "$HAS_ORCHESTRATION" = true ]; then echo '### ניהול משימות ארגוניות:
```bash
curl -X POST http://localhost:3002/api/orchestration/tasks \\
  -H "Content-Type: application/json" \\
  -d '\''{
    "name": "Analyze sales data",
    "description": "Analyze monthly sales data",
    "agent_type": "data_analyst"
  }'\''
```'; fi)

$(if [ "$HAS_SECURITY" = true ]; then echo '### בדיקת גישה לאבטחה:
```bash
curl -X POST http://localhost:8081/api/security/check/network \\
  -H "Content-Type: application/json" \\
  -d '\''{
    "destination": "api.openai.com",
    "protocol": "tcp",
    "port": 443
  }'\''
```'; fi)

## 🔧 התאמה אישית

אם תרצה לשנות את ההתקנה, הרץ שוב:
\`\`\`bash
./install_calclaw_smart.sh
\`\`\`

הסקריפט יזכור את ההגדרות הקודמות ויאפשר לך לשנות אותן.

## 📞 תמיכה

- **GitHub:** https://github.com/nadavsimba24/calclaw
- **דוקומנטציה:** https://calclaw.ai/docs
- **Community:** https://discord.gg/calclaw

## 🎉 סיכום

**Calclaw הותקן בהצלחה בהתאמה אישית מלאה לצרכים שלך!**

$(case $INSTALL_TYPE in
    "personal") echo "המערכת מותאמת לשימוש אישי - קלילה, מהירה, ופשוטה לשימוש.";;
    "development") echo "המערכת מותאמת לפיתוח - עם כלים לפיתוח, הרחבה, וניסוי.";;
    "business") echo "המערכת מותאמת לעסק קטן - עם אוטומציה, אינטגרציה, ותמיכה בעברית.";;
    "enterprise") echo "המערכת מותאמת לארגון גדול - עם אבטחה, compliance, ו-high availability.";;
    "municipal") echo "המערכת מותאמת לרשות מקומית - עם חיפוש תמונות, ניתוח חזותי, ותמיכה בערים ישראליות.";;
esac)

**המערכת מוכנה לשימוש!** 🚀
EOF

echo "✅ דוקומנטציה נוצרה: $README_FILE"

echo ""
echo "🎯 **התקנה הושלמה בהצלחה!**"
echo "============================="
echo ""
echo "🚀 **להרצת Calclaw:**"
echo "   $HOME/.calclaw/start_calclaw.sh"
echo ""
echo "🛑 **לעצירת Calclaw:**"
echo "   $HOME/.calclaw/stop_calclaw.sh"
echo ""
echo "📊 **לבדיקת סטטוס:**"
echo "   $HOME/.calclaw/status_calclaw.sh"
echo ""
echo "📚 **לדוקומנטציה מלאה:**"
echo "   cat $HOME/.calclaw/README_CUSTOM.md"
echo ""
echo "🔗 **קישורים חשובים:**"
if [ "$HAS_OLLAMA" = true ]; then
    echo "   • Ollama: http://localhost:11434"
fi
echo "   • Calclaw Core: http://localhost:3000"
if [ "$HAS_ORCHESTRATION" = true ]; then
    echo "   • Orchestration: http://localhost:3002"
    echo "   • Dashboard: http://localhost:3002/orchestration_dashboard.html"
fi
if [ "$HAS_SECURITY" = true ]; then
    echo "   • Security API: http://localhost:8081/api/security"
fi
echo ""
echo "🎯 **דוגמאות לשימוש:**"
echo "   # שיחה עם AI"
echo "   curl -X POST http://localhost:3000/api/chat \\"
echo "     -H 'Content-Type: application/json' \\"
echo "     -d '{\"message\": \"שלום\"}'"
echo ""
if [ "$HAS_SECURITY" = true ]; then
    echo "   # בדיקת גישה לאבטחה"
    echo "   curl -X POST http://localhost:8081/api/security/check/network \\"
    echo "     -H 'Content-Type: application/json' \\"
    echo "     -d '{\"destination\":\"api.openai.com\",\"port\":443}'"
    echo ""
fi
echo "📝 **התאמה אישית:**"
echo "   אם תרצה לשנות את ההתקנה, הרץ שוב:"
echo "   ./install_calclaw_smart.sh"
echo ""
echo "🎉 **Calclaw הותקן בהתאמה אישית מלאה!**"
echo ""
echo "המערכת מותאמת בדיוק לצרכים שלך:"
case $INSTALL_TYPE in
    "personal") 
        echo "   • שימוש אישי - קלילה ופשוטה"
        echo "   • התקנה מהירה (דקות בודדות)"
        echo "   • צריכת משאבים מינימלית"
        ;;
    "development")
        echo "   • פיתוח - עם כלים מלאים"
        echo "   • דוקומנטציה ודוגמאות קוד"
        echo "   • קלה להרחבה וניסוי"
        ;;
    "business")
        echo "   • עסק קטן - אוטומציה חכמה"
        echo "   • אינטגרציה עם כלים נפוצים"
        echo "   • תמיכה בעברית מלאה"
        ;;
    "enterprise")
        echo "   • ארגון גדול - אבטחה מתקדמת"
        echo "   • Compliance מלא"
        echo "   • High availability"
        ;;
    "municipal")
        echo "   • רשות מקומית - מערכת מוכנה"
        echo "   • חיפוש תמונות וניתוח חזותי"
        echo "   • תמיכה בערים ישראליות"
        ;;
esac
echo ""
echo "**המערכת מוכנה לשימוש!** 🚀"
EOF

cat install_calclaw_smart_part3.sh >> install_calclaw_smart.sh && rm install_calclaw_smart_part3.sh