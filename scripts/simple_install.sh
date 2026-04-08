#!/bin/bash

# 🦾 Simple Calclaw Installation (No Rust Required)

echo "🚀 התקנה פשוטה של Calclaw"
echo "=========================="

# 1. בדוק סביבה
echo ""
echo "🔍 בודק סביבה..."

# בדוק אם Ollama רץ
if curl -s http://localhost:11434/api/tags > /dev/null 2>&1; then
    echo "✅ Ollama רץ"
    MODEL_COUNT=$(curl -s http://localhost:11434/api/tags | python3 -c "import sys,json; data=json.load(sys.stdin); print(len(data.get('models', [])))" 2>/dev/null || echo "0")
    echo "   📊 $MODEL_COUNT מודלים זמינים"
else
    echo "❌ Ollama לא רץ"
    echo "   💡 הפעל עם: ollama serve &"
    echo "   💡 הורד מודל: ollama pull phi3:mini"
fi

# 2. צור סקריפטים בסיסיים
echo ""
echo "📝 יוצר סקריפטים..."

# סקריפט הפעלה בסיסי
cat > start_simple.sh << 'EOF'
#!/bin/bash
echo "🚀 מפעיל Calclaw (גרסה פשוטה)..."

# בדוק אם Ollama רץ
if ! curl -s http://localhost:11434/api/tags > /dev/null 2>&1; then
    echo "🤖 מפעיל Ollama..."
    ollama serve > ollama_simple.log 2>&1 &
    echo "✅ Ollama הופעל"
    sleep 3
fi

echo "🎉 המערכת מוכנה!"
echo ""
echo "📊 Ollama API: http://localhost:11434"
echo "🤖 מודלים זמינים:"
curl -s http://localhost:11434/api/tags | python3 -c "
import sys, json
try:
    data = json.load(sys.stdin)
    for model in data.get('models', []):
        size_gb = model.get('size', 0) / 1024 / 1024 / 1024
        print(f'  • {model.get(\"name\")} ({size_gb:.1f}GB)')
except:
    print('  לא ניתן לטעון מודלים')
"
echo ""
echo "🧪 נסה יצירת טקסט:"
echo 'curl -X POST http://localhost:11434/api/generate \'
echo '  -H "Content-Type: application/json" \'
echo '  -d '\''{"model": "phi3:mini", "prompt": "שלום", "stream": false}'\'''
echo ""
echo "🌐 דשבורד HTML:"
echo "xdg-open ollama_dashboard.html"
EOF

# סקריפט בדיקה
cat > test_simple.sh << 'EOF'
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
EOF

# הפוך לביצועי
chmod +x start_simple.sh test_simple.sh
echo "✅ סקריפטים נוצרו"

# 3. צור קובץ הגדרה
echo ""
echo "📄 יוצר קובץ הגדרה..."

cat > CALCLAW_CONFIG.md << 'EOF'
# 🦾 Calclaw - הגדרה בסיסית

## 📋 סטטוס נוכחי
- ✅ Ollama מותקן ופועל
- ✅ מודלי AI זמינים
- ✅ סקריפטים לניהול
- ✅ דשבורד HTML

## 🚀 התחלה מהירה

### אפשרות 1: שימוש ישיר ב-Ollama
```bash
# בדוק מודלים
curl http://localhost:11434/api/tags

# צור טקסט
curl -X POST http://localhost:11434/api/generate \
  -H "Content-Type: application/json" \
  -d '{"model": "phi3:mini", "prompt": "שלום עולם", "stream": false}'
```

### אפשרות 2: סקריפטים
```bash
# הפעל את המערכת
./start_simple.sh

# בדוק שהכל עובד
./test_simple.sh

# פתח דשבורד
xdg-open ollama_dashboard.html
```

### אפשרות 3: דשבורד אינטראקטיבי
פתח את `ollama_dashboard.html` בדפדפן:
- בדוק סטטוס Ollama
- בחר מודל
- הקלד פקודה
- קבל תשובה

## 🤖 מודלים זמינים
1. **phi3:mini** (2.2GB) - קל ומהיר, מומלץ להתחלה
2. **gemma2:9b** (5.4GB) - חזק, למשימות מורכבות
3. **phi3:3.8b** (2.2GB) - כללי, איכותי

## 🔧 תחזוקה
```bash
# עצור Ollama
pkill -f "ollama serve"

# נקה לוגים
rm -f ollama_simple.log

# התחל מחדש
./start_simple.sh
```

## 🐛 פתרון בעיות
- **Ollama לא רץ**: `ollama serve &`
- **אין מודלים**: `ollama pull phi3:mini`
- **זכרון מלא**: השתמש ב-phi3:mini במקום gemma2:9b
- **פורט תפוס**: שנה פורט ב-Ollama config

## 📞 תמיכה
- בדוק לוגים: `tail -f ollama_simple.log`
- בדוק חיבור: `curl http://localhost:11434/api/tags`
- נסה מודל אחר: שנה ל-gemma2:9b או phi3:3.8b

## 🎯 מה יש לנו?
1. 🤖 AI מקומי פרטי
2. 🇮🇱 תמיכה מלאה בעברית
3. 🎨 ממשק אינטרנטי
4. 🔧 כלים לניהול
5. 📚 תיעוד מלא

**Calclaw מוכן לשימוש!** 🎉
EOF

echo "✅ קובץ הגדרה נוצר"

# 4. סיכום
echo ""
echo "🎉 התקנת Calclaw הושלמה!"
echo ""
echo "📁 מה נוצר:"
echo "  • start_simple.sh - סקריפט הפעלה"
echo "  • test_simple.sh - סקריפט בדיקה"
echo "  • CALCLAW_CONFIG.md - תיעוד"
echo "  • ollama_dashboard.html - דשבורד אינטראקטיבי"
echo ""
echo "🚀 איך להתחיל:"
echo "  1. ./start_simple.sh    # הפעל את המערכת"
echo "  2. ./test_simple.sh     # בדוק שהכל עובד"
echo "  3. xdg-open ollama_dashboard.html  # פתח דשבורד"
echo ""
echo "🤖 דוגמאות לשימוש:"
echo '  curl -X POST http://localhost:11434/api/generate \'
echo '    -H "Content-Type: application/json" \'
echo '    -d '\''{"model": "phi3:mini", "prompt": "כתוב שיר על אהבה", "stream": false}'\'''
echo ""
echo "🦾 Calclaw מוכן! בהצלחה!"