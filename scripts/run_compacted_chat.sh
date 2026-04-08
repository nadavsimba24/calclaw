#!/bin/bash

# 🧠 Calclaw עם קומפקטינג חכם

echo "🧠 מפעיל Calclaw עם קומפקטינג אוטומטי..."
echo ""

# בדוק דרישות
echo "🔍 בודק דרישות..."

# בדוק Python
if ! command -v python3 &> /dev/null; then
    echo "❌ Python 3 לא מותקן"
    exit 1
fi

# בדוק ספריות
echo "🐍 בודק ספריות Python..."
REQUIRED_LIBS=("requests")
MISSING_LIBS=()

for lib in "${REQUIRED_LIBS[@]}"; do
    if ! python3 -c "import $lib" 2>/dev/null; then
        MISSING_LIBS+=("$lib")
    fi
done

if [ ${#MISSING_LIBS[@]} -gt 0 ]; then
    echo "📥 מתקין: ${MISSING_LIBS[*]}"
    pip3 install "${MISSING_LIBS[@]}" 2>/dev/null || echo "⚠️  לא ניתן להתקין"
fi

# בדוק מודולי Calclaw
echo "🦾 בודק מודולי Calclaw..."
MODULES=("hebrew_nlp.py" "hebrew_chat.py" "context_compactor.py")
MISSING_MODULES=()

for module in "${MODULES[@]}"; do
    if [ ! -f "$module" ]; then
        MISSING_MODULES+=("$module")
    fi
done

if [ ${#MISSING_MODULES[@]} -gt 0 ]; then
    echo "❌ חסרים מודולים: ${MISSING_MODULES[*]}"
    echo "💡 הרץ את סקריפט ההתקנה המלא"
    exit 1
fi

# הצג אפשרויות
echo ""
echo "🎯 בחר מצב קומפקטינג:"
echo "1. קומפקטינג חכם (מומלץ) - 800 טוקנים, 15 הודעות"
echo "2. קומפקטינג אגרסיבי - 500 טוקנים, 10 הודעות"
echo "3. קומפקטינג מתון - 1200 טוקנים, 20 הודעות"
echo "4. ללא קומפקטינג - היסטוריה מלאה"
echo "5. בדוק קומפקטינג"
echo ""

read -p "בחר אפשרות (1-5): " choice

case $choice in
    1)
        echo "🧠 מפעיל עם קומפקטינג חכם..."
        python3 hebrew_chat_compacted.py --tokens 800 --messages 15
        ;;
    2)
        echo "⚡ מפעיל עם קומפקטינג אגרסיבי..."
        python3 hebrew_chat_compacted.py --tokens 500 --messages 10
        ;;
    3)
        echo "🐢 מפעיל עם קומפקטינג מתון..."
        python3 hebrew_chat_compacted.py --tokens 1200 --messages 20
        ;;
    4)
        echo "📚 מפעיל ללא קומפקטינג..."
        python3 hebrew_chat.py
        ;;
    5)
        echo "🧪 בודק מערכת קומפקטינג..."
        python3 context_compactor.py --test
        ;;
    *)
        echo "❌ בחירה לא תקינה"
        ;;
esac

echo ""
echo "📊 סטטוס קונטקסט אחרון:"
if [ -f "/home/erez/.openclaw/workspace/calclaw/chat_history.json" ]; then
    python3 -c "
import json
try:
    with open('/home/erez/.openclaw/workspace/calclaw/chat_history.json', 'r') as f:
        history = json.load(f)
    
    print(f'   📝 {len(history)} הודעות בהיסטוריה')
    
    # חשב טוקנים משוערים
    total_chars = sum(len(str(msg)) for msg in history)
    estimated_tokens = total_chars // 4
    print(f'   🔢 ~{estimated_tokens} טוקנים')
    
    if history:
        from datetime import datetime
        latest = history[-1].get('timestamp', '')
        if latest:
            latest_time = datetime.fromisoformat(latest.replace('Z', '+00:00'))
            now = datetime.now(latest_time.tzinfo if latest_time.tzinfo else None)
            diff = now - latest_time
            
            if diff.days > 0:
                print(f'   ⏰ אחרונה: לפני {diff.days} ימים')
            elif diff.seconds > 3600:
                print(f'   ⏰ אחרונה: לפני {diff.seconds // 3600} שעות')
            else:
                print(f'   ⏰ אחרונה: לפני {diff.seconds // 60} דקות')
    
except Exception as e:
    print(f'   ❌ שגיאה: {e}')
"
else
    echo "   📭 אין היסטוריה"
fi

echo ""
echo "💡 טיפים לקומפקטינג:"
echo "• הודעות 'היי', 'תודה' נמחקות אוטומטית"
echo "• הודעות חשובות (גיבוי, ניקוי) נשמרות"
echo "• היסטוריה ישנה מסוכמת אוטומטית"
echo "• ניתן לנקות קונטקסט עם 'נקה קונטקסט'"