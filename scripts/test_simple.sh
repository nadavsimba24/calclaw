#!/bin/bash
echo "🧪 בדיקת Calclaw בסיסית"
echo "======================"

# בדוק Ollama
echo "1. בדיקת Ollama..."
if curl -s http://localhost:11434/api/tags > /dev/null 2>&1; then
    echo "   ✅ Ollama רץ"
    
    # בדוק מודלים
    MODEL_COUNT=$(curl -s http://localhost:11434/api/tags | python3 -c "import sys,json; data=json.load(sys.stdin); print(len(data.get('models', [])))" 2>/dev/null || echo "0")
    echo "   📊 $MODEL_COUNT מודלים זמינים"
    
    # נסה יצירת טקסט
    echo "2. בדיקת יצירת טקסט..."
    RESPONSE=$(curl -s -X POST http://localhost:11434/api/generate \
      -H "Content-Type: application/json" \
      -d '{"model": "phi3:mini", "prompt": "תגיד שלום בעברית", "stream": false}' 2>/dev/null)
    
    if echo "$RESPONSE" | grep -q "response"; then
        echo "   ✅ AI עובד"
        TEXT=$(echo "$RESPONSE" | python3 -c "import sys,json; data=json.load(sys.stdin); print(data.get('response', '')[:50])" 2>/dev/null || echo "")
        echo "   📝 תשובה: $TEXT..."
    else
        echo "   ⚠️  AI לא עובד"
    fi
else
    echo "   ❌ Ollama לא רץ"
fi

echo ""
echo "3. בדיקת דשבורד HTML..."
if [ -f "ollama_dashboard.html" ]; then
    echo "   ✅ דשבורד HTML קיים"
    echo "   🌐 פתח עם: xdg-open ollama_dashboard.html"
else
    echo "   ❌ דשבורד HTML לא קיים"
fi

echo ""
echo "🎉 בדיקה הושלמה!"
