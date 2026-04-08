# 🦾 Calclaw - התקנה ובדיקה משופרת

## 📋 דרישות מוקדמות

### 1. Ollama (AI מקומי)
```bash
# הורד והתקן Ollama
curl -fsSL https://ollama.com/install.sh | sh

# הפעל את שרת Ollama
ollama serve &

# הורד מודלים (בחר אחד או יותר)
ollama pull phi3:mini      # קל ומהיר (2.2GB)
ollama pull gemma2:9b      # חזק (5.4GB)
ollama pull phi3:3.8b      # כללי (2.2GB)
```

### 2. Rust (לבניית Calclaw)
```bash
# אם אין Rust מותקן
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

## 🚀 התקנת Calclaw

### אפשרות 1: התקנה מהירה
```bash
# היכנס לתיקיית Calclaw
cd /home/erez/.openclaw/workspace/calclaw

# הרץ סקריפט ההתקנה
chmod +x install.sh
./install.sh
```

### אפשרות 2: התקנה ידנית
```bash
cd /home/erez/.openclaw/workspace/calclaw

# שמור גיבוי של הקוד המקורי
cp src/main_backup.rs src/main_original.rs

# החלף בגרסה המשופרת עם Ollama
cp src/main_improved_final.rs src/main.rs
cp Cargo_improved.toml Cargo.toml

# הוסף את מודול Ollama
cp src/ollama_simple.rs src/

# בנה את הפרויקט
cargo build --release

# הפעל את השרת
./target/release/calclaw &
```

## 🔧 סקריפטי ניהול

### `start_calclaw.sh` - הפעלת המערכת
```bash
#!/bin/bash
# הפעל את Ollama במידע ולא רץ
if ! curl -s http://localhost:11434/api/tags > /dev/null; then
    echo "🚀 מפעיל Ollama..."
    ollama serve &
    sleep 5
fi

# הפעל את Calclaw
echo "🚀 מפעיל Calclaw..."
cd /home/erez/.openclaw/workspace/calclaw
./target/release/calclaw > calclaw.log 2>&1 &

echo "✅ Calclaw פועל על http://localhost:3000"
echo "📊 דשבורד: http://localhost:3000/admin"
```

### `stop_calclaw.sh` - עצירת המערכת
```bash
#!/bin/bash
echo "🛑 עוצר Calclaw..."
pkill -f "calclaw" || true
pkill -f "ollama serve" || true
echo "✅ Calclaw נעצר"
```

### `test_calclaw.sh` - בדיקת המערכת
```bash
#!/bin/bash
echo "🧪 בודק Calclaw..."

# בדוק אם השרת רץ
if curl -s http://localhost:3000/health > /dev/null; then
    echo "✅ Calclaw רץ"
else
    echo "❌ Calclaw לא רץ"
    exit 1
fi

# בדוק Ollama
echo "🤖 בודק Ollama..."
curl -s http://localhost:3000/api/ollama/health | python3 -m json.tool

# בדוק עברית
echo "🇮🇱 בודק עיבוד עברית..."
curl -X POST http://localhost:3000/api/hebrew \
  -H "Content-Type: application/json" \
  -d '{"text": "שלום עולם מ-Calclaw"}' | python3 -m json.tool

# בדוק יצירת טקסט
echo "🤖 בודק יצירת טקסט..."
curl -X POST http://localhost:3000/api/ollama/generate \
  -H "Content-Type: application/json" \
  -d '{"model": "phi3:mini", "prompt": "שלום", "task_type": "hebrew"}' | python3 -m json.tool

echo "✅ כל הבדיקות הושלמו!"
```

## 🧪 בדיקות אוטומטיות

### בדיקה מקיפה:
```bash
cd /home/erez/.openclaw/workspace/calclaw
chmod +x test_all.sh
./test_all.sh
```

### בדיקת ביצועים:
```bash
# בדיקת זמן תגובה
time curl -s http://localhost:3000/health > /dev/null

# בדיקת יצירת טקסט
time curl -X POST http://localhost:3000/api/ollama/generate \
  -H "Content-Type: application/json" \
  -d '{"model": "phi3:mini", "prompt": "תכתוב משפט אחד", "task_type": "hebrew"}'
```

## 📊 ניטור

### צפה בלוגים:
```bash
# לוג Calclaw
tail -f /home/erez/.openclaw/workspace/calclaw/calclaw.log

# לוג Ollama (אם רץ עם logging)
ollama serve 2>&1 | tee ollama.log
```

### בדוק סטטוס:
```bash
# סטטוס Calclaw
curl -s http://localhost:3000/health

# סטטוס Ollama
curl -s http://localhost:11434/api/tags | jq '.models | length'

# משתמשים רשומים
curl -s http://localhost:3000/api/users | jq '. | length'
```

## 🐛 פתרון בעיות

### בעיה: Ollama לא רץ
```bash
# בדוק אם Ollama רץ
ps aux | grep ollama

# הפעל מחדש
pkill ollama
ollama serve &

# בדוק חיבור
curl -s http://localhost:11434/api/tags
```

### בעיה: Calclaw לא נבנה
```bash
# נקה cache
cargo clean

# עדכן dependencies
cargo update

# בנה מחדש
cargo build --release
```

### בעיה: פורט 3000 תפוס
```bash
# מצא תהליך שתפס את הפורט
sudo lsof -i :3000

# עצור את התהליך
kill <PID>
```

## 🎯 מה בדקנו?

### ✅ תכונות בסיסיות:
1. שרת רץ על פורט 3000
2. עיבוד טקסט עברית
3. ניהול משתמשים
4. דשבורד אדמין

### ✅ שיפורים חדשים:
1. אינטגרציה עם Ollama
2. יצירת טקסט עם AI מקומי
3. דשבורד אינטראקטיבי
4. API מורחב

### ✅ בדיקות אוטומטיות:
1. בדיקת חיבור
2. בדיקת עברית
3. בדיקת AI
4. בדיקת ביצועים

## 📞 תמיכה

### בעיות נפוצות:
1. **Ollama לא מותקן** - ראה דרישות מוקדמות
2. **Rust לא מותקן** - התקן עם rustup
3. **פורטים תפוסים** - שנה פורט ב-config.toml
4. **זכרון לא מספיק** - השתמש ב-phi3:mini במקום gemma2:9b

### עזרה:
```bash
# בדוק גרסאות
ollama --version
cargo --version
rustc --version

# בדוק לוגים
cat /home/erez/.openclaw/workspace/calclaw/calclaw.log
```

## 🚀 התחל להשתמש!

### שלב 1: התקן
```bash
./install.sh
```

### שלב 2: הפעל
```bash
./start_calclaw.sh
```

### שלב 3: בדוק
```bash
./test_calclaw.sh
```

### שלב 4: השתמש
1. גלוש ל: http://localhost:3000/admin
2. לחץ על "בדוק יצירת טקסט"
3. התחל לפתח Skills משלך!

---

**מערכת Calclaw מוכנה לשימוש!** 🎉

לכל שאלה או בעיה - בדוק את הלוגים והרץ את סקריפט הבדיקה.