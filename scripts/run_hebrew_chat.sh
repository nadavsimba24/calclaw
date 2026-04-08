#!/bin/bash

# 🇮🇱 Calclaw Hebrew Chat Launcher

echo "🦾 מפעיל Calclaw - העוזר האישי בעברית!"
echo ""

# בדוק דרישות
echo "🔍 בודק דרישות..."

# בדוק Python
if ! command -v python3 &> /dev/null; then
    echo "❌ Python 3 לא מותקן"
    echo "📥 התקן עם: sudo apt-get install python3"
    exit 1
fi

# בדוק Ollama
echo "🤖 בודק Ollama..."
if curl -s http://localhost:11434/api/tags > /dev/null 2>&1; then
    echo "✅ Ollama רץ - NLP מתקדם זמין"
    
    # בדוק מודלים
    MODEL_COUNT=$(curl -s http://localhost:11434/api/tags | python3 -c "import sys,json; data=json.load(sys.stdin); print(len(data.get('models', [])))" 2>/dev/null || echo "0")
    echo "📊 $MODEL_COUNT מודלים זמינים"
    
    # בדוק אם יש מודל שתומך בעברית
    HAS_HEBREW=$(curl -s http://localhost:11434/api/tags | python3 -c "
import sys,json
data=json.load(sys.stdin)
models = [m['name'] for m in data.get('models', [])]
hebrew_models = ['phi3', 'gemma', 'llama']
for model in models:
    for hebrew in hebrew_models:
        if hebrew in model.lower():
            print('yes')
            sys.exit(0)
print('no')
" 2>/dev/null || echo "no")
    
    if [ "$HAS_HEBREW" = "yes" ]; then
        echo "🇮🇱 נמצאו מודלים שתומכים בעברית"
    else
        echo "⚠️  לא נמצאו מודלים שתומכים בעברית"
        echo "💡 הורד: ollama pull phi3:mini"
    fi
else
    echo "⚠️  Ollama לא רץ - NLP בסיסי בלבד"
    echo "💡 הפעל עם: ollama serve &"
fi

# בדוק ספריות Python
echo "🐍 בודק ספריות Python..."
REQUIRED_LIBS=("requests")
MISSING_LIBS=()

for lib in "${REQUIRED_LIBS[@]}"; do
    if ! python3 -c "import $lib" 2>/dev/null; then
        MISSING_LIBS+=("$lib")
    fi
done

if [ ${#MISSING_LIBS[@]} -gt 0 ]; then
    echo "📥 מתקין ספריות חסרות: ${MISSING_LIBS[*]}"
    pip3 install "${MISSING_LIBS[@]}" 2>/dev/null || {
        echo "❌ לא ניתן להתקין ספריות"
        echo "💡 התקן ידנית: pip3 install ${MISSING_LIBS[*]}"
    }
fi

# הצג אפשרויות
echo ""
echo "🎯 בחר מצב שיחה:"
echo "1. שיחה אינטראקטיבית (מומלץ)"
echo "2. בדיקת הבנת עברית"
echo "3. הרץ פקודה ספציפית"
echo "4. צפה בהיסטוריה"
echo ""

read -p "בחר אפשרות (1-4): " choice

case $choice in
    1)
        echo "💬 מפעיל שיחה אינטראקטיבית..."
        echo ""
        echo "💡 טיפים:"
        echo "• דבר בעברית טבעית"
        echo "• בקש גיבוי, ניקוי, בדיקה"
        echo "• הקלד 'יציאה' לסיום"
        echo ""
        python3 hebrew_chat.py
        ;;
    2)
        echo "🧪 מפעיל בדיקת הבנת עברית..."
        echo ""
        python3 hebrew_nlp.py
        ;;
    3)
        echo "🚀 הרץ פקודה ספציפית..."
        echo ""
        echo "💡 דוגמאות:"
        echo "• תגבה לי את הקבצים"
        echo "• תנקה את הלוגים"
        echo "• בדוק את המערכת"
        echo ""
        read -p "📝 הקלד פקודה בעברית: " cmd
        
        if [ -n "$cmd" ]; then
            echo ""
            echo "🤖 מעבד..."
            python3 -c "
from hebrew_nlp import HebrewNLP
nlp = HebrewNLP()
intent = nlp.extract_intent('$cmd')
print(f'🎯 הבנתי: {intent.get(\"description\", \"לא ברור\")}')
print(f'🔧 אבצע: {nlp.create_command_from_intent(intent)}')
print(f'💬 {nlp.generate_response(intent)}')
"
        else
            echo "❌ לא הוקלדה פקודה"
        fi
        ;;
    4)
        echo "📜 צופה בהיסטוריה..."
        if [ -f "/home/erez/.openclaw/workspace/calclaw/chat_history.json" ]; then
            python3 -c "
import json
try:
    with open('/home/erez/.openclaw/workspace/calclaw/chat_history.json', 'r') as f:
        history = json.load(f)
    
    print(f'📊 {len(history)} שיחות בהיסטוריה:')
    print('=' * 50)
    
    for i, item in enumerate(history, 1):
        print(f'{i}. {item[\"timestamp\"][:19]}')
        print(f'   👤: {item[\"user\"][:50]}...')
        if item.get('response'):
            print(f'   🤖: {item[\"response\"][:50]}...')
        print()
        
except Exception as e:
    print(f'❌ שגיאה: {e}')
"
        else
            echo "📭 אין היסטוריה לשמור"
        fi
        ;;
    *)
        echo "❌ בחירה לא תקינה"
        ;;
esac

echo ""
echo "🦾 Calclaw Hebrew Chat נסגר"
echo "💡 תמיד זמין עם: ./run_hebrew_chat.sh"