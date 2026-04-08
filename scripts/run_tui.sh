#!/bin/bash

# 🦾 Calclaw TUI Launcher

echo "🚀 מפעיל ממשק גרפי ל-Calclaw..."
echo ""

# בדוק אם Ollama רץ
if ! curl -s http://localhost:11434/api/tags > /dev/null 2>&1; then
    echo "❌ Ollama לא רץ"
    echo "💡 הפעל עם: ollama serve &"
    echo ""
    read -p "האם להפעיל Ollama עכשיו? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        echo "🤖 מפעיל Ollama..."
        ollama serve > ollama_tui.log 2>&1 &
        OLLAMA_PID=$!
        echo "✅ Ollama הופעל (PID: $OLLAMA_PID)"
        sleep 3
    else
        echo "❌ לא ניתן להפעיל את Calclaw ללא Ollama"
        exit 1
    fi
fi

# בדוק דרישות Python
echo "🔍 בודק דרישות Python..."
if ! python3 -c "import curses" 2>/dev/null; then
    echo "⚠️  ספריית curses לא מותקנת"
    echo "📥 מתקין..."
    sudo apt-get update && sudo apt-get install -y python3-curses 2>/dev/null || \
    echo "❌ לא ניתן להתקין curses. השתמש ב-CLI במקום."
fi

# הצג אפשרויות
echo ""
echo "🎯 בחר ממשק:"
echo "1. TUI גרפי (מלא עם curses)"
echo "2. CLI פשוט (שורת פקודה)"
echo "3. דשבורד אינטרנטי"
echo "4. API בלבד"
echo ""

read -p "בחר אפשרות (1-4): " choice

case $choice in
    1)
        echo "🚀 מפעיל TUI גרפי..."
        if python3 -c "import curses" 2>/dev/null; then
            python3 tui_simple.py
        else
            echo "❌ curses לא מותקן. מפעיל CLI במקום..."
            python3 cli_simple.py
        fi
        ;;
    2)
        echo "🚀 מפעיל CLI..."
        python3 cli_simple.py
        ;;
    3)
        echo "🌐 מפעיל דשבורד אינטרנטי..."
        if command -v xdg-open > /dev/null; then
            xdg-open ollama_dashboard.html
        elif command -v open > /dev/null; then
            open ollama_dashboard.html
        else
            echo "📄 פתח את הקובץ: ollama_dashboard.html בדפדפן"
        fi
        ;;
    4)
        echo "🔧 API בלבד"
        echo ""
        echo "📊 סטטוס:"
        curl -s http://localhost:11434/api/tags | python3 -m json.tool | head -20
        echo ""
        echo "🤖 דוגמאות:"
        echo '  curl -X POST http://localhost:11434/api/generate \'
        echo '    -H "Content-Type: application/json" \'
        echo '    -d '\''{"model": "phi3:mini", "prompt": "שלום", "stream": false}'\'''
        ;;
    *)
        echo "❌ בחירה לא תקינה"
        ;;
esac

echo ""
echo "🦾 Calclaw TUI נסגר"