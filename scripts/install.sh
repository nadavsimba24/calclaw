#!/bin/bash

# 🦾 Calclaw Installation Script
echo "🚀 מתחיל התקנת Calclaw..."

# בדוק אם אנחנו בתיקייה הנכונה
if [ ! -f "Cargo.toml" ]; then
    echo "❌ לא נמצא Cargo.toml. הרץ מתוך תיקיית calclaw"
    exit 1
fi

# 1. שמור גיבויים
echo "📦 שומר גיבויים..."
if [ -f "src/main_backup.rs" ]; then
    cp src/main_backup.rs src/main_backup_$(date +%Y%m%d_%H%M%S).rs
    echo "✅ גיבוי main_backup נשמר"
fi

if [ -f "src/main.rs" ]; then
    cp src/main.rs src/main_original_$(date +%Y%m%d_%H%M%S).rs
    echo "✅ גיבוי main נשמר"
fi

# 2. התקן את הגרסה המשופרת
echo "🔄 מתקין גרסה משופרת עם Ollama..."

# בדוק אם הקבצים הדרושים קיימים
if [ ! -f "src/main_improved_final.rs" ]; then
    echo "❌ לא נמצא src/main_improved_final.rs"
    echo "📥 מוריד מהאינטרנט..."
    # כאן אפשר להוסיף הורדה אם צריך
    exit 1
fi

if [ ! -f "Cargo_improved.toml" ]; then
    echo "❌ לא נמצא Cargo_improved.toml"
    exit 1
fi

if [ ! -f "src/ollama_simple.rs" ]; then
    echo "❌ לא נמצא src/ollama_simple.rs"
    exit 1
fi

# החלף קבצים
cp src/main_improved_final.rs src/main.rs
cp Cargo_improved.toml Cargo.toml
echo "✅ הקבצים הוחלפו"

# 3. בדוק דרישות מוקדמות
echo "🔍 בודק דרישות מוקדמות..."

# בדוק אם Rust מותקן
if ! command -v cargo &> /dev/null; then
    echo "⚠️  Rust לא מותקן"
    echo "📥 מתקין Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
    echo "✅ Rust הותקן"
else
    echo "✅ Rust מותקן: $(cargo --version)"
fi

# בדוק אם Ollama רץ
echo "🤖 בודק Ollama..."
if curl -s http://localhost:11434/api/tags > /dev/null 2>&1; then
    echo "✅ Ollama רץ"
    
    # בדוק אילו מודלים יש
    MODELS_COUNT=$(curl -s http://localhost:11434/api/tags | python3 -c "import sys,json; data=json.load(sys.stdin); print(len(data.get('models', [])))" 2>/dev/null || echo "0")
    echo "📊 מספר מודלים: $MODELS_COUNT"
    
    if [ "$MODELS_COUNT" -eq "0" ]; then
        echo "⚠️  אין מודלים. הורד עם: ollama pull phi3:mini"
    fi
else
    echo "⚠️  Ollama לא רץ. הפעל עם: ollama serve &"
    echo "💡 אפשרות: הרץ ./start_calclaw.sh שיפעיל אוטומטית"
fi

# 4. בנה את הפרויקט
echo "🔨 בונה את Calclaw..."
if cargo build --release; then
    echo "✅ Calclaw נבנה בהצלחה!"
    
    # בדוק גודל הבינארי
    BINARY_SIZE=$(stat -c%s target/release/calclaw 2>/dev/null || echo "0")
    BINARY_SIZE_MB=$((BINARY_SIZE / 1024 / 1024))
    echo "📦 גודל קובץ: ${BINARY_SIZE_MB}MB"
else
    echo "❌ בנייה נכשלה"
    echo "🔧 פותר בעיות..."
    
    # נקה cache
    cargo clean
    echo "🧹 Cache נוקה"
    
    # נסה שוב
    echo "🔨 מנסה לבנות שוב..."
    if cargo build --release; then
        echo "✅ Calclaw נבנה בהצלחה בנסיון השני!"
    else
        echo "❌ בנייה נכשלה שוב. בדוק את השגיאות למעלה."
        exit 1
    fi
fi

# 5. צור סקריפטים לניהול
echo "📝 יוצר סקריפטים לניהול..."

# סקריפט הפעלה
cat > start_calclaw.sh << 'EOF'
#!/bin/bash
# 🚀 Calclaw Startup Script

echo "🤖 בודק Ollama..."
if ! curl -s http://localhost:11434/api/tags > /dev/null 2>&1; then
    echo "🚀 מפעיל Ollama..."
    ollama serve > ollama.log 2>&1 &
    OLLAMA_PID=$!
    echo "✅ Ollama הופעל (PID: $OLLAMA_PID)"
    sleep 3
fi

echo "🦾 מפעיל Calclaw..."
cd "$(dirname "$0")"
./target/release/calclaw > calclaw.log 2>&1 &
CALCLAW_PID=$!

echo "✅ Calclaw הופעל (PID: $CALCLAW_PID)"
echo ""
echo "🎉 המערכת פועלת!"
echo "📊 דשבורד: http://localhost:3000/admin"
echo "🔧 Health check: http://localhost:3000/health"
echo "🤖 Ollama status: http://localhost:3000/api/ollama/health"
echo ""
echo "📝 לוגים:"
echo "  Calclaw: tail -f calclaw.log"
echo "  Ollama:  tail -f ollama.log"
echo ""
echo "🛑 לעצור: pkill calclaw && pkill ollama"
EOF

# סקריפט בדיקה
cat > test_calclaw.sh << 'EOF'
#!/bin/bash
# 🧪 Calclaw Test Script

echo "🧪 מתחיל בדיקות Calclaw..."
echo ""

# בדוק אם Calclaw רץ
echo "1. בדיקת Calclaw..."
if curl -s http://localhost:3000/health > /dev/null 2>&1; then
    echo "   ✅ Calclaw רץ"
else
    echo "   ❌ Calclaw לא רץ"
    echo "   💡 הרץ: ./start_calclaw.sh"
    exit 1
fi

# בדוק Ollama דרך Calclaw
echo "2. בדיקת Ollama integration..."
curl -s http://localhost:3000/api/ollama/health | python3 -c "
import sys, json
try:
    data = json.load(sys.stdin)
    if data.get('ollama_available'):
        print('   ✅ Ollama פועל דרך Calclaw')
        print(f'   📊 מודלים: {', '.join(data.get(\"default_models\", []))}')
    else:
        print(f'   ❌ Ollama לא פועל: {data.get(\"message\", \"לא ידוע\")}')
except:
    print('   ❌ שגיאה בבדיקת Ollama')
"

# בדוק עיבוד עברית
echo "3. בדיקת עיבוד עברית..."
curl -s -X POST http://localhost:3000/api/hebrew \
  -H "Content-Type: application/json" \
  -d '{"text": "שלום עולם"}' | python3 -c "
import sys, json
try:
    data = json.load(sys.stdin)
    if data.get('is_hebrew'):
        print(f'   ✅ עברית מזוהה: {data.get(\"rtl_wrapped\", \"\")}')
    else:
        print('   ❌ עברית לא מזוהה')
except:
    print('   ❌ שגיאה בבדיקת עברית')
"

# בדוק יצירת טקסט (רק אם Ollama רץ)
echo "4. בדיקת יצירת טקסט עם AI..."
curl -s -X POST http://localhost:3000/api/ollama/generate \
  -H "Content-Type: application/json" \
  -d '{"model": "phi3:mini", "prompt": "תגיד שלום", "task_type": "hebrew"}' | python3 -c "
import sys, json
try:
    data = json.load(sys.stdin)
    if data.get('success'):
        response = data.get('response', '')
        print(f'   ✅ AI עובד: {response[:50]}...')
    else:
        print(f'   ⚠️  AI לא עובד: {data.get(\"error\", \"לא ידוע\")}')
except Exception as e:
    print(f'   ❌ שגיאה: {e}')
"

echo ""
echo "🎉 כל הבדיקות הושלמו!"
echo ""
echo "🚀 לבדיקה ידנית:"
echo "  גלוש ל: http://localhost:3000/admin"
echo "  לחץ על 'בדוק יצירת טקסט'"
EOF

# סקריפט עצירה
cat > stop_calclaw.sh << 'EOF'
#!/bin/bash
# 🛑 Calclaw Stop Script

echo "🛑 עוצר Calclaw..."
pkill -f "calclaw" 2>/dev/null && echo "✅ Calclaw נעצר" || echo "⚠️  Calclaw לא היה רץ"

echo "🛑 עוצר Ollama..."
pkill -f "ollama serve" 2>/dev/null && echo "✅ Ollama נעצר" || echo "⚠️  Ollama לא היה רץ"

echo ""
echo "🧹 מנקה לוגים ישנים..."
rm -f calclaw.log ollama.log 2>/dev/null && echo "✅ לוגים נוקו" || echo "⚠️  אין לוגים למחיקה"

echo ""
echo "📊 סטטוס אחרון:"
ps aux | grep -E "(calclaw|ollama)" | grep -v grep || echo "✅ אין תהליכים רצים"
EOF

# הפוך את הסקריפטים לביצועיים
chmod +x start_calclaw.sh test_calclaw.sh stop_calclaw.sh
echo "✅ סקריפטים נוצרו והופעלו"

# 6. בדיקה סופית
echo ""
echo "🎉 התקנת Calclaw הושלמה!"
echo ""
echo "📋 מה הותקן:"
echo "  ✅ Calclaw עם Ollama integration"
echo "  ✅ 3 סקריפטים לניהול"
echo "  ✅ קובץ התקנה משופר"
echo ""
echo "🚀 איך להתחיל:"
echo "  1. ./start_calclaw.sh  # הפעל את המערכת"
echo "  2. ./test_calclaw.sh   # בדוק שהכל עובד"
echo "  3. גלוש ל: http://localhost:3000/admin"
echo ""
echo "🔧 פותר בעיות:"
echo "  • אם Ollama לא מותקן: curl -fsSL https://ollama.com/install.sh | sh"
echo "  • אם אין מודלים: ollama pull phi3:mini"
echo "  • אם Rust לא מותקן: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
echo ""
echo "📞 לתמיכה:"
echo "  • בדוק את הלוגים: tail -f calclaw.log"
echo "  • הרץ בדיקות: ./test_calclaw.sh"
echo "  • עיין ב: INSTALL_IMPROVED.md"
echo ""
echo "🦾 Calclaw מוכן לשימוש! בהצלחה!"