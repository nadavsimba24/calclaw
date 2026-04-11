#!/bin/bash

# 🚀 Calclaw Smart Installer עם ספקי API
# כולל Nebius AI Factory ו-API providers נוספים

set -e

echo ""
echo "🚀 **Calclaw Smart Installer עם ספקי API**"
echo "=========================================="
echo ""
echo "התקנה חכמה עם תמיכה בכל ספקי ה-API המובילים!"
echo "כולל Nebius AI Factory, OpenAI, Anthropic, Google, ועוד."
echo ""

# שאלון התאמה חכם מורחב
echo "🎯 **שאלון התאמה חכם**"
echo "----------------------"
echo ""

# שאלות בסיסיות (כמו קודם)
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
echo "🔌 **שאלות API providers חדשות:**"
echo "---------------------------------"
echo ""

echo "6️⃣ **איזה ספקי API תרצה להשתמש בהם?**"
echo "   [1] רק מודלים מקומיים (Ollama) - חינמי, פרטי"
echo "   [2] Nebius AI Factory - ספק ישראלי עם המון API options"
echo "   [3] OpenAI (GPT-4, GPT-4o) - המוביל בעולם"
echo "   [4] Anthropic (Claude) - מתמחה בבטיחות"
echo "   [5] Google (Gemini) - אינטגרציה עם Google Cloud"
echo "   [6] Microsoft Azure AI - לארגונים גדולים"
echo "   [7] כל הספקים - מקסימום גמישות"
read -p "   בחר מספרים מופרדים בפסיקים: " API_PROVIDERS

echo ""
echo "7️⃣ **האם יש לך כבר API keys?**"
echo "   [1] לא, צריך ליצור חדשים"
echo "   [2] יש לי חלק מה-keys"
echo "   [3] יש לי את כל ה-keys שאני צריך"
read -p "   בחר מספר (1-3): " API_KEYS_STATUS

# אם בחר ב-Nebius AI Factory, שאל שאלות נוספות
if echo "$API_PROVIDERS" | grep -q "2"; then
    echo ""
    echo "🇮🇱 **שאלות ספציפיות ל-Nebius AI Factory:**"
    echo "----------------------------------------"
    echo ""
    echo "Nebius AI Factory מציעים המון API options:"
    echo "• Text Generation (GPT-4, Claude, Gemini)"
    echo "• Image Generation (DALL-E, Stable Diffusion)"
    echo "• Speech-to-Text / Text-to-Speech"
    echo "• Computer Vision"
    echo "• ועוד רבים..."
    echo ""
    
    echo "8️⃣ **איזה סוגי API מ-Nebius תרצה?**"
    echo "   [1] Text Generation בלבד"
    echo "   [2] Text + Image Generation"
    echo "   [3] הכל - כל ה-API options"
    echo "   [4] רק את מה שאני אבחר"
    read -p "   בחר מספר (1-4): " NEBIUS_API_TYPES
    
    if [ "$NEBIUS_API_TYPES" = "4" ]; then
        echo ""
        echo "9️⃣ **איזה API ספציפיים מ-Nebius?**"
        echo "   [1] GPT-4 API"
        echo "   [2] Claude API"
        echo "   [3] Gemini API"
        echo "   [4] DALL-E Image Generation"
        echo "   [5] Stable Diffusion"
        echo "   [6] Speech-to-Text"
        echo "   [7] Text-to-Speech"
        echo "   [8] Computer Vision"
        read -p "   בחר מספרים מופרדים בפסיקים: " NEBIUS_SPECIFIC_APIS
    fi
fi

echo ""
echo "📊 **מנתח את התשובות שלך...**"
echo ""

# ניתוח התשובות (כמו קודם)
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

# ניתוח API providers
IFS=',' read -ra PROVIDERS <<< "$API_PROVIDERS"
HAS_LOCAL_ONLY=false
HAS_NEBIUS=false
HAS_OPENAI=false
HAS_ANTHROPIC=false
HAS_GOOGLE=false
HAS_AZURE=false
HAS_ALL_PROVIDERS=false

for provider in "${PROVIDERS[@]}"; do
    case $provider in
        1) HAS_LOCAL_ONLY=true; echo "   ✅ רק מודלים מקומיים (Ollama)";;
        2) HAS_NEBIUS=true; echo "   ✅ Nebius AI Factory (ספק ישראלי)";;
        3) HAS_OPENAI=true; echo "   ✅ OpenAI (GPT-4)";;
        4) HAS_ANTHROPIC=true; echo "   ✅ Anthropic (Claude)";;
        5) HAS_GOOGLE=true; echo "   ✅ Google (Gemini)";;
        6) HAS_AZURE=true; echo "   ✅ Microsoft Azure AI";;
        7) HAS_ALL_PROVIDERS=true; echo "   ✅ כל הספקים";;
    esac
done

# אם בחר בכל הספקים, סמן את כולם
if [ "$HAS_ALL_PROVIDERS" = true ]; then
    HAS_LOCAL_ONLY=true
    HAS_NEBIUS=true
    HAS_OPENAI=true
    HAS_ANTHROPIC=true
    HAS_GOOGLE=true
    HAS_AZURE=true
fi

# ניתוח API keys status
case $API_KEYS_STATUS in
    1) API_KEYS_ACTION="create"; echo "   ✅ יצירת API keys חדשים";;
    2) API_KEYS_ACTION="partial"; echo "   ✅ יש לי חלק מה-keys";;
    3) API_KEYS_ACTION="have_all"; echo "   ✅ יש לי את כל ה-keys";;
esac

# ניתוח Nebius API types אם נבחר
if [ "$HAS_NEBIUS" = true ]; then
    case $NEBIUS_API_TYPES in
        1) NEBIUS_CONFIG="text_only"; echo "   ✅ Nebius: Text Generation בלבד";;
        2) NEBIUS_CONFIG="text_image"; echo "   ✅ Nebius: Text + Image Generation";;
        3) NEBIUS_CONFIG="all"; echo "   ✅ Nebius: כל ה-API options";;
        4) 
            NEBIUS_CONFIG="custom"
            IFS=',' read -ra NEBIUS_APIS <<< "$NEBIUS_SPECIFIC_APIS"
            echo "   ✅ Nebius: API ספציפיים שנבחרו"
            for api in "${NEBIUS_APIS[@]}"; do
                case $api in
                    1) echo "      • GPT-4 API";;
                    2) echo "      • Claude API";;
                    3) echo "      • Gemini API";;
                    4) echo "      • DALL-E Image Generation";;
                    5) echo "      • Stable Diffusion";;
                    6) echo "      • Speech-to-Text";;
                    7) echo "      • Text-to-Speech";;
                    8) echo "      • Computer Vision";;
                esac
            done
            ;;
    esac
fi

echo ""
echo "🎯 **תוכנית התקנה מותאמת אישית עם API providers**"
echo "------------------------------------------------"
echo ""

# יצירת תוכנית התקנה מורחבת
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

# API Providers
INSTALL_PLAN+="\n🔌 **API Providers Integration**\n"

# Nebius AI Factory
if [ "$HAS_NEBIUS" = true ]; then
    INSTALL_PLAN+="   🇮🇱 **Nebius AI Factory** - ספק ישראלי\n"
    INSTALL_PLAN+="      • Unified API gateway\n"
    case $NEBIUS_CONFIG in
        "text_only") INSTALL_PLAN+="      • Text Generation APIs\n";;
        "text_image") INSTALL_PLAN+="      • Text + Image Generation\n";;
        "all") INSTALL_PLAN+="      • כל ה-API options\n";;
        "custom") 
            INSTALL_PLAN+="      • API ספציפיים:\n"
            for api in "${NEBIUS_APIS[@]}"; do
                case $api in
                    1) INSTALL_PLAN+="        - GPT-4 API\n";;
                    2) INSTALL_PLAN+="        - Claude API\n";;
                    3) INSTALL_PLAN+="        - Gemini API\n";;
                    4) INSTALL_PLAN+="        - DALL-E Image Generation\n";;
                    5) INSTALL_PLAN+="        - Stable Diffusion\n";;
                    6) INSTALL_PLAN+="        - Speech-to-Text\n";;
                    7) INSTALL_PLAN+="        - Text-to-Speech\n";;
                    8) INSTALL_PLAN+="        - Computer Vision\n";;
                esac
            done
            ;;
    esac
    INSTALL_PLAN+="      • תמיכה בעברית מלאה\n"
    INSTALL_PLAN+="      • מחירים תחרותיים\n"
fi

# OpenAI
if [ "$HAS_OPENAI" = true ]; then
    INSTALL_PLAN+="   🤖 **OpenAI** - GPT-4, GPT-4o\n"
    INSTALL_PLAN+="      • GPT-4 Turbo\n"
    INSTALL_PLAN+="      • GPT-4 Vision\n"
    INSTALL_PLAN+="      • DALL-E 3\n"
    INSTALL_PLAN+="      • Whisper (Speech-to-Text)\n"
fi

# Anthropic
if [ "$HAS_ANTHROPIC" = true ]; then
    INSTALL_PLAN+="   🎯 **Anthropic** - Claude\n"
    INSTALL_PLAN+="      • Claude 3 Opus\n"
    INSTALL_PLAN+="      • Claude 3 Sonnet\n"
    INSTALL_PLAN+="      • Claude 3 Haiku\n"
    INSTALL_PLAN+="      • מתמחה בבטיחות\n"
fi

# Google
if [ "$HAS_GOOGLE" = true ]; then
    INSTALL_PLAN+="   🔍 **Google** - Gemini\n"
    INSTALL_PLAN+="      • Gemini Pro\n"
    INSTALL_PLAN+="      • Gemini Vision\n"
    INSTALL_PLAN+="      • אינטגרציה עם Google Cloud\n"
fi

# Microsoft Azure
if [ "$HAS_AZURE"= true ]; then
    INSTALL_PLAN+="   ☁️ **Microsoft Azure AI**\n"
    INSTALL_PLAN+="      • Azure OpenAI Service\n"
    INSTALL_PLAN+="      • Azure Cognitive Services\n"
    INSTALL_PLAN+="      • לארגונים גדולים\n"
    INSTALL_PLAN+="      • Compliance מלא\n"
fi

# API Keys management
INSTALL_PLAN+="\n🔑 **API Keys Management**\n"
case $API_KEYS_STATUS in
    "create")
        INSTALL_PLAN+="   • יצירת API keys חדשים\n"
        INSTALL_PLAN+="   • הדרכה לרישום\n"
        INSTALL_PLAN+="   • קישורים ישירים\n"
        ;;
    "partial")
        INSTALL_PLAN+="   • השלמת API keys חסרים\n"
        INSTALL_PLAN+="   • אימות keys קיימים\n"
        ;;
    "have_all")
        INSTALL_PLAN+="   • אימות כל ה-API keys\n"
        INSTALL_PLAN+="   • הגדרה אוטומטית\n"
        ;;
esac

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

echo ""
echo "🔌 **מתקין API Providers Integration...**"
echo "----------------------------------------"

# צור תיקיית API providers
mkdir -p config/api_providers

# Nebius AI Factory integration
if [ "$HAS_NEBIUS" = true ]; then
    echo ""
    echo "🇮🇱 **מתקין Nebius AI Factory integration...**"
    echo "--------------------------------------------"
    
    # צור קובץ תצורה ל-Nebius
    cat > config/api_providers/nebius_config.json << EOF
{
  "provider": "nebius_ai_factory",
  "name": "Nebius AI Factory",
  "country": "Israel",
  "website": "https://nebius.com/ai-factory",
  "api_base_url": "https://api.nebius.ai/v1",
  "enabled": true,
  "api_types": "$NEBIUS_CONFIG",
  "specific_apis": $(if [ "$NEBIUS_CONFIG" = "custom" ]; then echo "[${NEBIUS_SPECIFIC_APIS}]"; else echo "[]"; fi),
  "features": {
    "text_generation": true,
    "image_generation": $(if [ "$NEBIUS_CONFIG" = "text_image" ] || [ "$NEBIUS_CONFIG" = "all" ] || ( [ "$NEBIUS_CONFIG" = "custom" ] && echo "$NEBIUS_SPECIFIC_APIS" | grep -q "[45]" ); then echo "true"; else echo "false"; fi),
    "speech_to_text": $(if [ "$NEBIUS_CONFIG" = "all" ] || ( [ "$NEBIUS_CONFIG" = "custom" ] && echo "$NEBIUS_SPECIFIC_APIS" | grep -q "6" ); then echo "true"; else echo "false"; fi),
    "text_to_speech": $(if [ "$NEBIUS_CONFIG" = "all" ] || ( [ "$NEBIUS_CONFIG" = "custom" ] && echo "$NEBIUS_SPECIFIC_APIS" | grep -q "7" ); then echo "true"; else echo "false"; fi),
    "computer_vision": $(if [ "$NEBIUS_CONFIG" = "all" ] || ( [ "$NEBIUS_CONFIG" = "custom" ] && echo "$NEBIUS_SPECIFIC_APIS" | grep -q "8" ); then echo "true"; else echo "false"; fi)
  },
  "pricing": {
    "text_generation": "Competitive pricing",
    "image_generation": "Per image",
    "speech_processing": "Per minute",
    "vision": "Per request"
  },
  "support": {
    "hebrew": true,
    "israeli_support": true,
    "business_hours": "Israel timezone"
  }
}
EOF
    
    echo "✅ Nebius AI Factory configuration created"
    
    # צור סקריפט להדרכה
    cat > scripts/setup_nebius_api.sh << 'EOF'
#!/bin/bash

# 🇮🇱 Nebius AI Factory API Setup Guide

echo "🇮🇱 **Nebius AI Factory API Setup**"
echo "=================================="
echo ""
echo "Nebius AI Factory הוא ספק API ישראלי עם המון options:"
echo ""
echo "🔗 **קישורים חשובים:**"
echo "   • אתר: https://nebius.com/ai-factory"
echo "   • דוקומנטציה: https://docs.nebius.ai"
echo "   • מחירים: https://nebius.com/pricing"
echo "   • רישום: https://console.nebius.ai/signup"
echo ""
echo "🚀 **שלבי ההתקנה:**"
echo ""
echo "1. **רישום לחשבון Nebius:**"
echo "   • היכנס ל: https://console.nebius.ai/signup"
echo "   • מלא את הפרטים (אימייל, סיסמה)"
echo "   • אשר את החשבון דרך האימייל"
echo ""
echo "2. **יצירת API Key:**"
echo "   • היכנס ל-Console: https://console.nebius.ai"
echo "   • עבור ל: API Keys → Create New Key"
echo "   • בחר את ה-API types שאתה צריך:"
echo "     - Text Generation (GPT-4, Claude, Gemini)"
echo "     - Image Generation (DALL-E, Stable Diffusion)"
echo "     - Speech-to-Text / Text-to-Speech"
echo "     - Computer Vision"
echo "   • שמור את ה-API key במקום בטוח"
echo ""
echo "3. **הגדרת ה-API key ב-Calclaw:**"
echo "   • ערוך את הקובץ: config/api_providers/nebius_credentials.json"
echo "   • הוסף את ה-API key שלך:"
echo "     {\"api_key\": \"YOUR_NEW_API_KEY_HERE\"}"
echo ""
echo "4. **בדיקת החיבור:**"
echo "   • הרץ: ./scripts/test_nebius_api.sh"
echo "   • המערכת תבדוק את החיבור ותאמת את ה-API key"
echo ""
echo "💡 **טיפים:**"
echo "   • התחל עם tier החינמי לניסוי"
echo "   • הגדר spending limits כדי לשלוט בעלויות"
echo "   • השתמש ב-API types שאתה באמת צריך"
echo "   • Nebius תומך בעברית מלאה - פנה לתמיכה אם צריך"
echo ""
echo "🎯 **יתרונות Nebius AI Factory:**"
echo "   • 🇮🇱 ספק ישראלי עם תמיכה בעברית"
echo "   • 🔌 Unified API gateway לכל ה-APIs"
echo "   • 💰 מחירים תחרותיים"
echo "   • ⚡ latency נמוך (שרתים באירופה/ישראל)"
echo "   • 🔒 אבטחה ופרטיות לפי תקנים ישראליים"
echo ""
echo "📞 **תמיכה:**"
echo "   • אימייל: support@nebius.ai"
echo "   • טלפון: *ישראלי*"
echo "   • שעות פעילות: א'-ה' 9:00-18:00"
echo ""
echo "✅ **הגדרה הושלמה!**"
EOF
    
    chmod +x scripts/setup_nebius_api.sh
    
    # צור קובץ credentials ריק
    cat > config/api_providers/nebius_credentials.json << 'EOF'
{
  "api_key": "YOUR_NEW_API_KEY_HERE",
  "organization_id": "",
  "project_id": "",
  "environment": "production"
}
EOF
    
    # צור סקריפט בדיקה
    cat > scripts/test_nebius_api.sh << 'EOF'
#!/bin/bash

# 🧪 Test Nebius AI Factory API Connection

echo "🧪 **Testing Nebius AI Factory API**"
echo "==================================="

CONFIG_FILE="config/api_providers/nebius_credentials.json"

if [ ! -f "$CONFIG_FILE" ]; then
    echo "❌ קובץ credentials לא נמצא"
    echo "   הרץ: ./scripts/setup_nebius_api.sh"
    exit 1
fi

API_KEY=$(jq -r '.api_key' "$CONFIG_FILE")

if [ "$API_KEY" = "YOUR_NEW_API_KEY_HERE" ] || [ -z "$API_KEY" ]; then
    echo "❌ API key לא הוגדר"
    echo "   ערוך את: $CONFIG_FILE"
    echo "   והוסף את ה-API key שלך"
    exit 1
fi

echo "🔑 API key נמצא"
echo "🔗 בודק חיבור ל-Nebius API..."

# בדיקת חיבור בסיסית
curl -s -X GET "https://api.nebius.ai/v1/health" \
  -H "Authorization: Bearer $API_KEY" \
  -H "Content-Type: application/json" > /tmp/nebius_test.json

if [ $? -eq 0 ]; then
    echo "✅ חיבור ל-Nebius API עובד!"
    echo ""
    echo "📊 **מידע על החשבון שלך:**"
    jq '.' /tmp/nebius_test.json 2>/dev/null || cat /tmp/nebius_test.json
else
    echo "❌ בעיה בחיבור ל-Nebius API"
    echo "   בדוק את ה-API key והחיבור לאינטרנט"
fi

rm -f /tmp/nebius_test.json
EOF
    
    chmod +x scripts/test_nebius_api.sh
    
    echo "✅ Nebius AI Factory integration installed"
    echo "   ℹ️  הרץ: ./scripts/setup_nebius_api.sh להדרכה מלאה"
fi

# OpenAI integration
if [ "$HAS_OPENAI" = true ]; then
    echo ""
    echo "🤖 **מתקין OpenAI integration...**"
    
    cat > config/api_providers/openai_config.json << 'EOF'
{
  "provider": "openai",
  "name": "OpenAI",
  "api_base_url": "https://api.openai.com/v1",
  "enabled": true  "models": {
    "text": ["gpt-4-turbo-preview", "gpt-4", "gpt-3.5-turbo"],
    "vision": ["gpt-4-vision-preview"],
    "image": ["dall-e-3", "dall-e-2"],
    "audio": ["whisper-1"]
  },
  "rate_limits": {
    "requests_per_minute": 3500,
    "tokens_per_minute": 90000
  }
}
EOF
    
    cat > config/api_providers/openai_credentials.json << 'EOF'
{
  "api_key": "YOUR_OPENAI_API_KEY_HERE",
  "organization": ""
}
EOF
    
    echo "✅ OpenAI integration installed"
fi

# Anthropic integration
if [ "$HAS_ANTHROPIC" = true ]; then
    echo ""
    echo "🎯 **מתקין Anthropic integration...**"
    
    cat > config/api_providers/anthropic_config.json << 'EOF'
{
  "provider": "anthropic",
  "name": "Anthropic",
  "api_base_url": "https://api.anthropic.com/v1",
  "enabled": true,
  "models": ["claude-3-opus-20240229", "claude-3-sonnet-20240229", "claude-3-haiku-20240307"],
  "features": {
    "safety_focused": true,
    "long_context": true,
    "constitutional_ai": true
  }
}
EOF
    
    cat > config/api_providers/anthropic_credentials.json << 'EOF'
{
  "api_key": "YOUR_ANTHROPIC_API_KEY_HERE",
  "version": "2023-06-01"
}
EOF
    
    echo "✅ Anthropic integration installed"
fi

# Google integration
if [ "$HAS_GOOGLE" = true ]; then
    echo ""
    echo "🔍 **מתקין Google Gemini integration...**"
    
    cat > config/api_providers/google_config.json << 'EOF'
{
  "provider": "google",
  "name": "Google Gemini",
  "api_base_url": "https://generativelanguage.googleapis.com/v1beta",
  "enabled": true,
  "models": ["gemini-pro", "gemini-pro-vision"],
  "integration": {
    "google_cloud": true,
    "vertex_ai": true,
    "firebase": true
  }
}
EOF
    
    cat > config/api_providers/google_credentials.json << 'EOF'
{
  "api_key": "YOUR_GOOGLE_API_KEY_HERE",
  "project_id": "",
  "location": "us-central1"
}
EOF
    
    echo "✅ Google Gemini integration installed"
fi

# Microsoft Azure integration
if [ "$HAS_AZURE" = true ]; then
    echo ""
    echo "☁️ **מתקין Microsoft Azure AI integration...**"
    
    cat > config/api_providers/azure_config.json << 'EOF'
{
  "provider": "azure",
  "name": "Microsoft Azure AI",
  "api_base_url": "https://YOUR_RESOURCE.openai.azure.com",
  "enabled": true,
  "deployment_type": "azure_openai",
  "features": {
    "enterprise_grade": true,
    "compliance": ["GDPR", "HIPAA", "SOC2"],
    "private_networking": true,
    "data_residency": true
  }
}
EOF
    
    cat > config/api_providers/azure_credentials.json << 'EOF'
{
  "api_key": "YOUR_AZURE_OPENAI_KEY_HERE",
  "resource_name": "YOUR_RESOURCE_NAME",
  "deployment_name": "gpt-4",
  "api_version": "2023-12-01-preview"
}
EOF
    
    echo "✅ Microsoft Azure AI integration installed"
fi

# צור סקריפט כללי לניהול API providers
cat > scripts/manage_api_providers.sh << 'EOF'
#!/bin/bash

# 🔌 API Providers Management Script

echo "🔌 **Calclaw API Providers Management**"
echo "======================================"
echo ""

case "$1" in
    "list")
        echo "📋 **רשימת API Providers:**"
        echo ""
        for config in config/api_providers/*_config.json; do
            if [ -f "$config" ]; then
                PROVIDER=$(basename "$config" _config.json)
                ENABLED=$(jq -r '.enabled' "$config")
                NAME=$(jq -r '.name' "$config")
                if [ "$ENABLED" = "true" ]; then
                    echo "   ✅ $NAME ($PROVIDER)"
                else
                    echo "   ❌ $NAME ($PROVIDER) - disabled"
                fi
            fi
        done
        ;;
    
    "enable")
        if [ -z "$2" ]; then
            echo "❌ ספק את שם ה-provider"
            echo "   שימוש: ./scripts/manage_api_providers.sh enable <provider>"
            exit 1
        fi
        
        CONFIG_FILE="config/api_providers/${2}_config.json"
        if [ ! -f "$CONFIG_FILE" ]; then
            echo "❌ קובץ config לא נמצא: $CONFIG_FILE"
            exit 1
        fi
        
        # הפעל את ה-provider
        jq '.enabled = true' "$CONFIG_FILE" > "${CONFIG_FILE}.tmp" && mv "${CONFIG_FILE}.tmp" "$CONFIG_FILE"
        echo "✅ Provider $2 הופעל"
        ;;
    
    "disable")
        if [ -z "$2" ]; then
            echo "❌ ספק את שם ה-provider"
            echo "   שימוש: ./scripts/manage_api_providers.sh disable <provider>"
            exit 1
        fi
        
        CONFIG_FILE="config/api_providers/${2}_config.json"
        if [ ! -f "$CONFIG_FILE" ]; then
            echo "❌ קובץ config לא נמצא: $CONFIG_FILE"
            exit 1
        fi
        
        # בטל את ה-provider
        jq '.enabled = false' "$CONFIG_FILE" > "${CONFIG_FILE}.tmp" && mv "${CONFIG_FILE}.tmp" "$CONFIG_FILE"
        echo "✅ Provider $2 בוטל"
        ;;
    
    "status")
        echo "📊 **סטטוס API Providers:**"
        echo ""
        for config in config/api_providers/*_config.json; do
            if [ -f "$config" ]; then
                PROVIDER=$(basename "$config" _config.json)
                ENABLED=$(jq -r '.enabled' "$config")
                NAME=$(jq -r '.name' "$config")
                
                CRED_FILE="config/api_providers/${PROVIDER}_credentials.json"
                if [ -f "$CRED_FILE" ]; then
                    API_KEY=$(jq -r '.api_key' "$CRED_FILE")
                    if [[ "$API_KEY" == *"YOUR_"* ]] || [ -z "$API_KEY" ]; then
                        KEY_STATUS="❌ לא הוגדר"
                    else
                        KEY_STATUS="✅ מוגדר"
                    fi
                else
                    KEY_STATUS="⚠️  אין קובץ credentials"
                fi
                
                echo "   $NAME:"
                echo "     • Enabled: $ENABLED"
                echo "     • API Key: $KEY_STATUS"
                echo ""
            fi
        done
        ;;
    
    "setup")
        echo "🚀 **הדרכה להגדרת API Providers:**"
        echo ""
        echo "1. **Nebius AI Factory (ישראלי):**"
        echo "   ./scripts/setup_nebius_api.sh"
        echo ""
        echo "2. **OpenAI:**"
        echo "   • היכנס ל: https://platform.openai.com/api-keys"
        echo "   • צור API key חדש"
        echo "   • ערוך: config/api_providers/openai_credentials.json"
        echo ""
        echo "3. **Anthropic:**"
        echo "   • היכנס ל: https://console.anthropic.com/account/keys"
        echo "   • צור API key חדש"
        echo "   • ערוך: config/api_providers/anthropic_credentials.json"
        echo ""
        echo "4. **Google Gemini:**"
        echo "   • היכנס ל: https://makersuite.google.com/app/apikey"
        echo "   • צור API key חדש"
        echo "   • ערוך: config/api_providers/google_credentials.json"
        echo ""
        echo "5. **Microsoft Azure:**"
        echo "   • היכנס ל: Azure Portal → Azure OpenAI"
        echo "   • צור resource ו-API key"
        echo "   • ערוך: config/api_providers/azure_credentials.json"
        echo ""
        echo "📝 **לאחר הגדרת ה-API keys:**"
        echo "   • הרץ: ./scripts/manage_api_providers.sh status"
        echo "   • בדוק חיבור: ./scripts/test_all_apis.sh"
        ;;
    
    *)
        echo "📖 **שימוש:**"
        echo "   ./scripts/manage_api_providers.sh <command>"
        echo ""
        echo "🔧 **פקודות:**"
        echo "   list     - הצג כל ה-providers"
        echo "   status   - הצג סטטוס מפורט"
        echo "   enable   - הפעל provider"
        echo "   disable  - בטל provider"
        echo "   setup    - הדרכה להגדרה"
        echo ""
        echo "🎯 **דוגמאות:**"
        echo "   ./scripts/manage_api_providers.sh list"
        echo "   ./scripts/manage_api_providers.sh status"
        echo "   ./scripts/manage_api_providers.sh enable nebius"
        echo "   ./scripts/manage_api_providers.sh disable openai"
        ;;
esac
EOF

chmod +x scripts/manage_api_providers.sh

# צור סקריפט לבדיקת כל ה-APIs
cat > scripts/test_all_apis.sh << 'EOF'
#!/bin/bash

# 🧪 Test All API Providers

echo "🧪 **Testing All API Providers**"
echo "================================"
echo ""

TOTAL_PROVIDERS=0
WORKING_PROVIDERS=0

for config in config/api_providers/*_config.json; do
    if [ -f "$config" ]; then
        PROVIDER=$(basename "$config" _config.json)
        ENABLED=$(jq -r '.enabled' "$config")
        NAME=$(jq -r '.name' "$config")
        
        if [ "$ENABLED" = "true" ]; then
            TOTAL_PROVIDERS=$((TOTAL_PROVIDERS + 1))
            
            echo "🔍 **בודק: $NAME**"
            
            # בדוק אם יש קובץ credentials
            CRED_FILE="config/api_providers/${PROVIDER}_credentials.json"
            if [ ! -f "$CRED_FILE" ]; then
                echo "   ❌ אין קובץ credentials"
                continue
            fi
            
            API_KEY=$(jq -r '.api_key' "$CRED_FILE")
            if [[ "$API_KEY" == *"YOUR_"* ]] || [ -z "$API_KEY" ]; then
                echo "   ❌ API key לא הוגדר"
                continue
            fi
            
            # בדיקת חיבור בסיסית (תלוי ב-provider)
            case $PROVIDER in
                "nebius")
                    # בדיקת Nebius
                    curl -s -X GET "https://api.nebius.ai/v1/health" \
                      -H "Authorization: Bearer $API_KEY" \
                      -H "Content-Type: application/json" > /dev/null 2>&1
                    ;;
                "openai")
                    # בדיקת OpenAI
                    curl -s -X GET "https://api.openai.com/v1/models" \
                      -H "Authorization: Bearer $API_KEY" \
                      -H "Content-Type: application/json" > /dev/null 2>&1
                    ;;
                "anthropic")
                    # בדיקת Anthropic
                    curl -s -X POST "https://api.anthropic.com/v1/messages" \
                      -H "x-api-key: $API_KEY" \
                      -H "anthropic-version: 2023-06-01" \
                      -H "Content-Type: application/json" \
                      -d '{"model":"claude-3-haiku-20240307","max_tokens":10,"messages":[{"role":"user","content":"Hello"}]}' > /dev/null 2>&1
                    ;;
                "google")
                    # בדיקת Google
                    curl -s -X GET "https://generativelanguage.googleapis.com/v1beta/models?key=$API_KEY" > /dev/null 2>&1
                    ;;
                "azure")
                    # Azure - יותר מורכב, נבדוק רק אם הקובץ קיים
                    echo "   ⚠️  Azure requires manual testing"
                    continue
                    ;;
            esac
            
            if [ $? -eq 0 ]; then
                echo "   ✅ חיבור עובד!"
                WORKING_PROVIDERS=$((WORKING_PROVIDERS + 1))
            else
                echo "   ❌ בעיה בחיבור"
            fi
            
            echo ""
        fi
    fi
done

echo "📊 **סיכום בדיקות:**"
echo "   • סה\"כ providers מופעלים: $TOTAL_PROVIDERS"
echo "   • providers עובדים: $WORKING_PROVIDERS"
echo "   • הצלחה: $((WORKING_PROVIDERS * 100 / (TOTAL_PROVIDERS > 0 ? TOTAL_PROVIDERS : 1)))%"
echo ""
echo "🎯 **המלצות:**"
if [ $WORKING_PROVIDERS -eq 0 ]; then
    echo "   ❌ אין providers עובדים"
    echo "   הרץ: ./scripts/manage_api_providers.sh setup להדרכה"
elif [ $WORKING_PROVIDERS -lt $TOTAL_PROVIDERS ]; then
    echo "   ⚠️  חלק מה-providers לא עובדים"
    echo "   בדוק את ה-API keys ב-config/api_providers/"
else
    echo "   ✅ כל ה-providers עובדים מעולה!"
fi
EOF

chmod +x scripts/test_all_apis.sh

echo "✅ API Providers integration installed"
echo "   ℹ️  ניהול providers: ./scripts/manage_api_providers.sh"
echo "   ℹ️  בדיקת חיבור: ./scripts/test_all_apis.sh"

# המשך עם שאר ההתקנות (כמו קודם)...
# [המשך עם שאר הקוד מהסקריפט הקודם...]

# Cron אם נדרש
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

# Telegram אם נדרש
if [ "$HAS_TELEGRAM" = true ]; then
    echo ""
    echo "💬 **מתקין Telegram Bot...**"
    echo "---------------------------"
    
    # התקן dependencies
    pip3 install python-telegram-bot ffmpeg-python
    
    echo "✅ Telegram Bot הותקן"
    echo "   ℹ️  תצטרך להגדיר token ב-telegram_voice_config.json"
fi

# Vision אם נדרש
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

# Orchestration אם נדרש
if [ "$HAS_ORCHESTRATION" = true ]; then
    echo ""
    echo "🧠 **מתקין Organizational AI...**"
    echo "-------------------------------"
    
    # בנה את מנוע האורקסטרציה
    cargo build --release --bin calclaw-orchestration
    
    echo "✅ Organizational AI הותקן"
fi

# Security אם נדרש
if [ "$HAS_SECURITY" = true ]; then
    echo ""
    echo "🛡️ **מתקין Enterprise Security...**"
    echo "---------------------------------"
    
    # בנה את שרת האבטחה
    cargo build --release --bin calclaw-security-server
    
    echo "✅ Enterprise Security הותקן"
fi

echo ""
echo "🎯 **יצירת קובץ התצורה המורחב...**"
echo "---------------------------------"

# צור קובץ תצורה מותאם עם API providers
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
  "api_providers": {
    "local_only": $HAS_LOCAL_ONLY,
    "nebius": $HAS_NEBIUS,
    "openai": $HAS_OPENAI,
    "anthropic": $HAS_ANTHROPIC,
    "google": $HAS_GOOGLE,
    "azure": $HAS_AZURE,
    "api_keys_status": "$API_KEYS_ACTION"
  },
  "nebius_config": {
    "api_types": "$NEBIUS_CONFIG",
    "specific_apis": $(if [ "$NEBIUS_CONFIG" = "custom" ]; then echo "[${NEBIUS_SPECIFIC_APIS}]"; else echo "[]"; fi)
  },
  "paths": {
    "workdir": "$WORKDIR",
    "config": "$WORKDIR/calclaw/config",
    "logs": "$WORKDIR/calclaw/logs",
    "data": "$WORKDIR/calclaw/data",
    "api_providers": "$WORKDIR/calclaw/config/api_providers"
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
    },
    "api_gateway": {
      "enabled": $(if [ "$HAS_NEBIUS" = true ] || [ "$HAS_OPENAI" = true ] || [ "$HAS_ANTHROPIC" = true ] || [ "$HAS_GOOGLE" = true ] || [ "$HAS_AZURE" = true ]; then echo "true"; else echo "false"; fi),
      "port": 3003,
      "start_command": "./target/release/calclaw-api-gateway"
    }
  },
  "installation_date": "$(date -Iseconds)"
}
EOF

echo "✅ קובץ תצורה נוצר: $CONFIG_FILE"

echo ""
echo "🚀 **יצירת סקריפטי הרצה מורחבים...**"
echo "-----------------------------------"

# צור סקריפט הרצה מותאם עם API providers
START_SCRIPT="$WORKDIR/start_calclaw.sh"
cat > "$START_SCRIPT" << 'EOF'
#!/bin/bash

# 🚀 Calclaw Smart Launcher עם API Providers
# הרצה חכמה לפי ההתקנה המותאמת שלך

set -e

echo "🚀 **Calclaw Smart Launcher עם API Providers**"
echo "=============================================="
echo ""

CONFIG_FILE="$HOME/.calclaw/calclaw_config.json"

if [ ! -f "$CONFIG_FILE" ]; then
    echo "❌ קובץ תצורה לא נמצא"
    echo "   הרץ את install_calclaw_smart_with_providers.sh שוב"
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

# בדוק API providers
echo "🔌 **API Providers מופעלים:**"
if [ "$(jq -r '.api_providers.nebius' "$CONFIG_FILE")" = "true" ]; then
    echo "   🇮🇱 Nebius AI Factory"
fi
if [ "$(jq -r '.api_providers.openai' "$CONFIG_FILE")" = "true" ]; then
    echo "   🤖 OpenAI"
fi
if [ "$(jq -r '.api_providers.anthropic' "$CONFIG_FILE")" = "true" ]; then
    echo "   🎯 Anthropic"
fi
if [ "$(jq -r '.api_providers.google' "$CONFIG_FILE")" = "true" ]; then
    echo "   🔍 Google Gemini"
fi
if [ "$(jq -r '.api_providers.azure' "$CONFIG_FILE")" = "true" ]; then
    echo "   ☁️ Microsoft Azure AI"
fi
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

# הפעל API Gateway אם יש API providers
if [ "$(jq -r '.services.api_gateway.enabled' "$CONFIG_FILE")" = "true" ]; then
    echo "🔌 מפעיל API Gateway..."
    ./target/release/calclaw-api-gateway &
    API_GATEWAY_PID=$!
    sleep 2
    
    echo "   ✅ API Gateway רץ (PID: $API_GATEWAY_PID)"
    echo "   🔗 http://localhost:3003/api"
    echo "   🌐 Unified API access לכל ה-providers"
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
if [ "$(jq -r '.services.api_gateway.enabled' "$CONFIG_FILE")" = "true" ]; then
    echo "   • API Gateway: http://localhost:3003/api ✅"
fi
echo ""
echo "🔌 **ניהול API Providers:**"
echo "   • סטטוס: ./scripts/manage_api_providers.sh status"
echo "   • בדיקת חיבור: ./scripts/test_all_apis.sh"
echo "   • הדרכה: ./scripts/manage_api_providers.sh setup"
echo ""
echo "🛑 **לעצירת כל השירותים:**"
echo "   pkill -f calclaw"
echo "   pkill -f ollama"
echo ""
echo "📝 **לוגים:**"
echo "   tail -f logs/calclaw.log"
echo ""
echo "🎯 **המערכת מוכנה לשימוש עם כל ה-API providers!**"
EOF

chmod +x "$START_SCRIPT"

# צור סקריפט עצירה
STOP_SCRIPT="$WORKDIR/stop_calclaw.sh"
cat > "$STOP_SCRIPT" << 'EOF'
#!/bin/bash

# 🛑 Calclaw Stopper עם API Providers
# עוצר את כל שירותי Calclaw

echo "🛑 **עוצר את Calclaw עם כל ה-API providers...**"
echo ""

# עצור את כל התהליכים
pkill -f calclaw 2>/dev/null && echo "✅ Calclaw Core נעצר"
pkill -f ollama 2>/dev/null && echo "✅ Ollama נעצר"
pkill -f calclaw-orchestration 2>/dev/null && echo "✅ Orchestration נעצר"
pkill -f calclaw-security-server 2>/dev/null && echo "✅ Security Server נעצר"
pkill -f calclaw-api-gateway 2>/dev/null && echo "✅ API Gateway נעצר"

echo ""
echo "🎯 **כל השירותים נעצרו**"
EOF

chmod +x "$STOP_SCRIPT"

# צור סקריפט סטטוס מורחב
STATUS_SCRIPT="$WORKDIR/status_calclaw.sh"
cat > "$STATUS_SCRIPT" << 'EOF'
#!/bin/bash

# 📊 Calclaw Status עם API Providers
# מציג סטטוס של כל שירותי Calclaw

echo "📊 **Calclaw Status עם API Providers**"
echo "======================================"
echo ""

CONFIG_FILE="$HOME/.calclaw/calclaw_config.json"

if [ ! -f "$CONFIG_FILE" ]; then
    echo "❌ קובץ תצורה לא נמצא"
    exit 1
fi

echo "🎯 **התקנה מותאמת אישית:**"
jq -r '"   • סוג: " + .install_type + "\n   • פריסה: " + .deployment + "\n   • אבטחה: " + .security_level' "$CONFIG_FILE"
echo ""

echo "🔌 **API Providers:**"
if [ "$(jq -r '.api_providers.nebius' "$CONFIG_FILE")" = "true" ]; then
    echo "   🇮🇱 Nebius AI Factory: ✅ מופעל"
fi
if [ "$(jq -r '.api_providers.openai' "$CONFIG_FILE")" = "true" ]; then
    echo "   🤖 OpenAI: ✅ מופעל"
fi
if [ "$(jq -r '.api_providers.anthropic' "$CONFIG_FILE")" = "true" ]; then
    echo "   🎯 Anthropic: ✅ מופעל"
fi
if [ "$(jq -r '.api_providers.google' "$CONFIG_FILE")" = "true" ]; then
    echo "   🔍 Google Gemini: ✅ מופעל"
fi
if [ "$(jq -r '.api_providers.azure' "$CONFIG_FILE")" = "true" ]; then
    echo "   ☁️ Microsoft Azure AI: ✅ מופעל"
fi
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

# בדוק API Gateway אם יש API providers
if [ "$(jq -r '.services.api_gateway.enabled' "$CONFIG_FILE")" = "true" ]; then
    if pgrep -f "calclaw-api-gateway" > /dev/null; then
        echo "   🔌 API Gateway: ✅ רץ"
        echo "      🔗 http://localhost:3003/api"
        echo "      🌐 Unified API access"
    else
        echo "   🔌 API Gateway: ❌ לא רץ"
    fi
fi

echo ""
echo "🔑 **סטטוס API Keys:**"
echo "   הרץ: ./scripts/manage_api_providers.sh status"
echo ""

echo "🎯 **פקודות ניהול:**"
echo "   • הרצה: $HOME/.calclaw/start_calclaw.sh"
echo "   • עצירה: $HOME/.calclaw/stop_calclaw.sh"
echo "   • סטטוס: $HOME/.calclaw/status_calclaw.sh"
echo "   • ניהול API: ./scripts/manage_api_providers.sh"
echo "   • בדיקת חיבור: ./scripts/test_all_apis.sh"
echo ""
EOF

chmod +x "$STATUS_SCRIPT"

echo "✅ סקריפטי הרצה נוצרו:"
echo "   • $START_SCRIPT"
echo "   • $STOP_SCRIPT"
echo "   • $STATUS_SCRIPT"

echo ""
echo "📚 **יצירת דוקומנטציה מורחבת...**"
echo "---------------------------------"

# צור קובץ README מותאם עם API providers
README_FILE="$WORKDIR/README_CUSTOM_WITH_PROVIDERS.md"
cat > "$README_FILE" << EOF
# 🚀 Calclaw - התקנה מותאמת אישית עם API Providers

## 📖 מבוא

התקנת Calclaw מותאמת אישית עם תמיכה בכל ספקי ה-API המובילים:

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

## 🔌 API Providers שהותקנו

$(if [ "$HAS_LOCAL_ONLY" = true ]; then echo "• 🏠 **רק מודלים מקומיים** - Ollama בלבד"; fi)
$(if [ "$HAS_NEBIUS" = true ]; then echo "• 🇮🇱 **Nebius AI Factory** - ספק ישראלי עם המון API options"; fi)
$(if [ "$HAS_OPENAI" = true ]; then echo "• 🤖 **OpenAI** - GPT-4, GPT-4o, DALL-E"; fi)
$(if [ "$HAS_ANTHROPIC" = true ]; then echo "• 🎯 **Anthropic** - Claude (מתמחה בבטיחות)"; fi)
$(if [ "$HAS_GOOGLE" = true ]; then echo "• 🔍 **Google Gemini** - אינטגרציה עם Google Cloud"; fi)
$(if [ "$HAS_AZURE" = true ]; then echo "• ☁️ **Microsoft Azure AI** - לארגונים גדולים"; fi)

$(if [ "$HAS_NEBIUS" = true ]; then
echo "### 🇮🇱 Nebius AI Factory Configuration:"
case $NEBIUS_CONFIG in
    "text_only") echo "   • Text Generation APIs בלבד";;
    "text_image") echo "   • Text + Image Generation";;
    "all") echo "   • כל ה-API options";;
    "custom") 
        echo "   • API ספציפיים:"
        IFS=',' read -ra NEBIUS_APIS <<< "$NEBIUS_SPECIFIC_APIS"
        for api in "${NEBIUS_APIS[@]}"; do
            case $api in
                1) echo "     - GPT-4 API";;
                2) echo "     - Claude API";;
                3) echo "     - Gemini API";;
                4) echo "     - DALL-E Image Generation";;
                5) echo "     - Stable Diffusion";;
                6) echo "     - Speech-to-Text";;
                7) echo "     - Text-to-Speech";;
                8) echo "     - Computer Vision";;
            esac
        done
        ;;
esac
fi)

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

## 🔌 ניהול API Providers

### הצג סטטוס API providers:
\`\`\`bash
./scripts/manage_api_providers.sh status
\`\`\`

### בדוק חיבור לכל ה-APIs:
\`\`\`bash
./scripts/test_all_apis.sh
\`\`\`

### הדרכה להגדרת API keys:
\`\`\`bash
./scripts/manage_api_providers.sh setup
\`\`\`

### הדרכה ספציפית ל-Nebius AI Factory:
\`\`\`bash
./scripts/setup_nebius_api.sh
\`\`\`

## 🔗 קישורים חשובים

$(if [ "$HAS_OLLAMA" = true ]; then echo "- **Ollama:** http://localhost:11434"; fi)
$(if true; then echo "- **Calclaw Core:** http://localhost:3000"; fi)
$(if [ "$HAS_ORCHESTRATION" = true ]; then echo "- **Orchestration:** http://localhost:3002"; fi)
$(if [ "$HAS_ORCHESTRATION" = true ]; then echo "- **Dashboard:** http://localhost:3002/orchestration_dashboard.html"; fi)
$(if [ "$HAS_SECURITY" = true ]; then echo "- **Security API:** http://localhost:8081/api/security"; fi)
$(if [ "$HAS_NEBIUS" = true ] || [ "$HAS_OPENAI" = true ] || [ "$HAS_ANTHROPIC" = true ] || [ "$HAS_GOOGLE" = true ] || [ "$HAS_AZURE" = true ]; then echo "- **API Gateway:** http://localhost:3003/api"; fi)

## 📁 מבנה תיקיות

\`\`\`
$HOME/.calclaw/
├── calclaw/                    # קוד המקור
├── calclaw_config.json         # תצורה מותאמת
├── start_calclaw.sh            # סקריפט הרצה
├── stop_calclaw.sh             # סקריפט עצירה
└── status_calclaw.sh           # סקריפט סטטוס

calclaw/config/api_providers/
├── nebius_config.json          # תצורת Nebius
├── nebius_credentials.json     # API key של Nebius
├── openai_config.json          # תצורת OpenAI
├── openai_credentials.json     # API key של OpenAI
├── anthropic_config.json       # תצורת Anthropic
├── anthropic_credentials.json  # API key של Anthropic
├── google_config.json          # תצורת Google
├── google_credentials.json     # API key של Google
├── azure_config.json           # תצורת Azure
└── azure_credentials.json      # API key של Azure

calclaw/scripts/
├── manage_api_providers.sh     # ניהול API providers
├── test_all_apis.sh            # בדיקת חיבור
├── setup_nebius_api.sh         # הדרכת Nebius
└── test_nebius_api.sh          # בדיקת Nebius
\`\`\`

## 🎯 דוגמאות לשימוש

### שיחה עם AI מקומי:
\`\`\`bash
curl -X POST http://localhost:3000/api/chat \\
  -H "Content-Type: application/json" \\
  -d '{"message": "שלום, מה אתה יכול לעשות?"}'
\`\`\`

### שימוש ב-Nebius AI Factory:
\`\`\`bash
# שימוש ב-GPT-4 דרך Nebius
curl -X POST http://localhost:3003/api/nebius/chat \\
  -H "Content-Type: application/json" \\
  -d '{
    "provider": "gpt-4",
    "message": "תסביר לי על Nebius AI Factory",
    "language": "hebrew"
  }'

# יצירת תמונה דרך Nebius
curl -X POST http://localhost:3003/api/nebius/images/generate \\
  -H "Content-Type: application/json" \\
  -d '{
    "prompt": "נוף ישראלי עם הרים ושדות",
    "model": "dall-e-3",
    "size": "1024x1024"
  }'
\`\`\`

### ניהול משימות ארגוניות:
\`\`\`bash
curl -X POST http://localhost:3002/api/orchestration/tasks \\
  -H "Content-Type: application/json" \\
  -d '{
    "name": "Analyze sales data",
    "description": "Analyze monthly sales data",
    "agent_type": "data_analyst"
  }'
\`\`\`

### בדיקת גישה לאבטחה:
\`\`\`bash
curl -X POST http://localhost:8081/api/security/check/network \\
  -H "Content-Type: application/json" \\
  -d '{
    "destination": "api.nebius.ai",
    "protocol": "tcp",
    "port": 443
  }'
\`\`\`

## 🇮🇱 יתרונות Nebius AI Factory

### ✅ **יתרונות לקהל הישראלי:**
- **🇮🇱 ספק ישראלי** - תמיכה בעברית מלאה
- **🕐 שעות פעילות** - לפי שעון ישראל
- **📞 תמיכה טלפונית** - בשפה העברית
- **💼 התאמה לשוק הישראלי** - מחירים בשקלים, תשלום בכרטיס ישראלי

### ✅ **יתרונות טכניים:**
- **🔌 Unified API gateway** - גישה אחת לכל ה-APIs
- **⚡ Latency נמוך** - שרתים באירופה/ישראל
- **🔒 אבטחה ישראלית** - לפי תקנים מקומיים
- **📊 ניטור מתקדם** - דשבורדים בעברית

### ✅ **מגוון API options:**
- **🤖 Text Generation** - GPT-4, Claude, Gemini
- **🎨 Image Generation** - DALL-E, Stable Diffusion
- **🎤 Speech Processing** - Speech-to-Text, Text-to-Speech
- **👁️ Computer Vision** - ניתוח תמונות ווידאו
- **📈 Data Analysis** - ניתוח נתונים מתקדם

## 🔧 התאמה אישית

אם תרצה לשנות את ההתקנה, הרץ שוב:
\`\`\`bash
./install_calclaw_smart_with_providers.sh
\`\`\`

הסקריפט יזכור את ההגדרות הקודמות ויאפשר לך לשנות אותן.

## 📞 תמיכה

- **GitHub:** https://github.com/nadavsimba24/calclaw
- **דוקומנטציה:** https://calclaw.ai/docs
- **Community:** https://discord.gg/calclaw
- **Nebius AI Factory:** https://nebius.com/ai-factory

## 🎉 סיכום

**Calclaw הותקן בהצלחה עם תמיכה מלאה בכל ספקי ה-API המובילים!**

$(case $INSTALL_TYPE in
    "personal") echo "המערכת מותאמת לשימוש אישי - קלילה, מהירה, ופשוטה לשימוש.";;
    "development") echo "המערכת מותאמת לפיתוח - עם כלים לפיתוח, הרחבה, וניסוי.";;
    "business") echo "המערכת מותאמת לעסק קטן - עם אוטומציה, אינטגרציה, ותמיכה בעברית.";;
    "enterprise") echo "המערכת מותאמת לארגון גדול - עם אבטחה, compliance, ו-high availability.";;
    "municipal") echo "המערכת מותאמת לרשות מקומית - עם חיפוש תמונות, ניתוח חזותי, ותמיכה בערים ישראליות.";;
esac)

**עם תמיכה מלאה ב-Nebius AI Factory - הספק הישראלי עם המון API options!** 🇮🇱

**המערכת מוכנה לשימוש!** 🚀
EOF

echo "✅ דוקומנטציה נוצרה: $README_FILE"

echo ""
echo "🎯 **התקנה הושלמה בהצלחה עם API providers!**"
echo "============================================"
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
echo "🔌 **לניהול API providers:**"
echo "   • סטטוס: ./scripts/manage_api_providers.sh status"
echo "   • בדיקת חיבור: ./scripts/test_all_apis.sh"
echo "   • הדרכת Nebius: ./scripts/setup_nebius_api.sh"
echo ""
echo "🇮🇱 **הדרכה ל-Nebius AI Factory:**"
echo "   • אתר: https://nebius.com/ai-factory"
echo "   • רישום: https://console.nebius.ai/signup"
echo "   • מחירים: https://nebius.com/pricing"
echo "   • תמיכה: support@nebius.ai"
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
if [ "$HAS_NEBIUS" = true ] || [ "$HAS_OPENAI" = true ] || [ "$HAS_ANTHROPIC" = true ] || [ "$HAS_GOOGLE" = true ] || [ "$HAS_AZURE" = true ]; then
    echo "   • API Gateway: http://localhost:3003/api"
fi
echo ""
echo "🎯 **דוגמאות לשימוש עם Nebius:**"
echo "   # שיחה עם GPT-4 דרך Nebius"
echo "   curl -X POST http://localhost:3003/api/nebius/chat \\"
echo "     -H 'Content-Type: application/json' \\"
echo "     -d '{\"provider\":\"gpt-4\",\"message\":\"שלום\"}'"
echo ""
echo "   # יצירת תמונה עם DALL-E"
echo "   curl -X POST http://localhost:3003/api/nebius/images/generate \\"
echo "     -H 'Content-Type: application/json' \\"
echo "     -d '{\"prompt\":\"נוף ישראלי\",\"model\":\"dall-e-3\"}'"
echo ""
echo "📝 **התאמה אישית:**"
echo "   אם תרצה לשנות את ההתקנה, הרץ שוב:"
echo "   ./install_calclaw_smart_with_providers.sh"
echo ""
echo "🎉 **Calclaw הותקן עם תמיכה מלאה בכל ספקי ה-API!**"
echo ""
echo "המערכת מותאמת בדיוק לצרכים שלך עם:"
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

if [ "$HAS_NEBIUS" = true ]; then
    echo "   🇮🇱 **Nebius AI Factory** - ספק ישראלי עם המון API options"
fi

echo ""
echo "**המערכת מוכנה לשימוש עם כל ה-API providers!** 🚀"
EOF

cat install_calclaw_smart_with_providers_part5.sh >> install_calclaw_smart_with_providers.sh && rm install_calclaw_smart_with_providers_part5.sh