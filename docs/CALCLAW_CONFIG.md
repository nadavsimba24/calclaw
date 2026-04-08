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
