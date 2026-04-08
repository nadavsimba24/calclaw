#!/bin/bash

echo "🚀 מתקין Calclaw עם Timeless Cal Squads..."
echo "=========================================="

# בדוק אם אנחנו בתיקייה הנכונה
if [ ! -f "Cargo.toml" ]; then
    echo "❌ לא נמצא קובץ Cargo.toml. ודא שאתה בתיקיית calclaw."
    exit 1
fi

echo "📦 גרסה: Calclaw 0.3.0 עם Timeless Squads"

# בדוק אם Rust מותקן
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust לא מותקן. מתקין..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
    echo "✅ Rust הותקן"
else
    echo "✅ Rust כבר מותקן"
fi

# בדוק אם Ollama מותקן
if ! command -v ollama &> /dev/null; then
    echo "⚠️  Ollama לא מותקן. ממליץ להתקין:"
    echo "   curl -fsSL https://ollama.com/install.sh | sh"
    echo "ממשיך בכל זאת..."
else
    echo "✅ Ollama מותקן"
fi

# שמור גיבוי של הקוד המקורי
echo "💾 שומר גיבויים..."
if [ -f "src/main.rs" ]; then
    cp src/main.rs src/main.backup.$(date +%Y%m%d_%H%M%S).rs
    echo "✅ גיבוי של main.rs נשמר"
fi

if [ -f "Cargo.toml" ]; then
    cp Cargo.toml Cargo.backup.$(date +%Y%m%d_%H%M%S).toml
    echo "✅ גיבוי של Cargo.toml נשמר"
fi

# ודא שכל הקבצים הנדרשים קיימים
echo "🔍 בודק קבצים נדרשים..."
REQUIRED_FILES=(
    "src/timeless_squads.rs"
    "src/ollama_simple.rs"
    "src/main.rs"
    "Cargo.toml"
)

for file in "${REQUIRED_FILES[@]}"; do
    if [ ! -f "$file" ]; then
        echo "❌ קובץ חסר: $file"
        exit 1
    fi
done

echo "✅ כל הקבצים הנדרשים קיימים"

# בדוק אם הקוד תקין
echo "🔧 בודק תקינות קוד..."
cargo check

if [ $? -ne 0 ]; then
    echo "⚠️  יש שגיאות בקוד. מנסה לתקן אוטומטית..."
    
    # נסה לתקן שגיאות נפוצות
    echo "🔧 מתקן שגיאות..."
    
    # אם יש שגיאת double dot
    sed -i 's/\.        \.await/.await/g' src/main.rs 2>/dev/null || true
    
    # אם יש שגיאת duplicate let
    sed -i 's/let                //g' src/timeless_squads.rs 2>/dev/null || true
    
    echo "🔄 בודק שוב..."
    cargo check
    
    if [ $? -ne 0 ]; then
        echo "❌ עדיין יש שגיאות. בדוק ידנית:"
        cargo check
        exit 1
    fi
fi

echo "✅ הקוד תקין"

# בנה את הפרויקט
echo "🔨 בונה את Calclaw עם Timeless Squads..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "❌ בנייה נכשלה. בדוק את השגיאות למעלה."
    exit 1
fi

echo "✅ Calclaw עם Timeless Squads נבנה בהצלחה!"

# צור סקריפטי ניהול
echo "📝 יוצר סקריפטי ניהול..."

# סקריפט הפעלה
cat > start_calclaw_with_timeless.sh << 'EOF'
#!/bin/bash

echo "🚀 מתחיל Calclaw עם Timeless Cal Squads..."
echo "=========================================="

# בדוק אם Ollama רץ
if ! curl -s http://localhost:11434/api/tags > /dev/null; then
    echo "❌ Ollama לא רץ. מפעיל..."
    ollama serve > /tmp/ollama_calclaw.log 2>&1 &
    OLLAMA_PID=$!
    sleep 5
    echo "✅ Ollama רץ (PID: $OLLAMA_PID)"
else
    echo "✅ Ollama כבר רץ"
fi

# בדוק מודלים
echo "📦 בדיקת מודלים זמינים..."
MODELS=$(curl -s http://localhost:11434/api/tags | python3 -c "import sys, json; data=json.load(sys.stdin); print('\n'.join([m['name'] for m in data.get('models', [])]))" 2>/dev/null || echo "לא ניתן לבדוק מודלים")

if [ -z "$MODELS" ] || [ "$MODELS" = "לא ניתן לבדוק מודלים" ]; then
    echo "⚠️  אין מודלים זמינים או לא ניתן לבדוק. ממליץ:"
    echo "   ollama pull phi3:mini"
    echo "ממשיך בכל זאת..."
else
    echo "✅ מודלים זמינים:"
    echo "$MODELS"
fi

# הפעל את Calclaw
echo "🌐 מפעיל את Calclaw על פורט 3000..."
cd /home/erez/.openclaw/workspace/calclaw
./target/release/calclaw > /tmp/calclaw_timeless.log 2>&1 &
CALCLAW_PID=$!

sleep 3

# בדוק אם השרת רץ
if curl -s http://localhost:3000/api/health > /dev/null; then
    echo "✅ Calclaw רץ על http://localhost:3000"
else
    echo "❌ Calclaw לא נטען. בדוק את הלוגים: /tmp/calclaw_timeless.log"
    exit 1
fi

echo ""
echo "🎉 Calclaw עם Timeless Cal Squads מוכן!"
echo ""
echo "🔗 ממשקים זמינים:"
echo ""
echo "   🏠 דשבורד ראשי:"
echo "      http://localhost:3000"
echo ""
echo "   🚀 Timeless Squads Dashboard:"
echo "      http://localhost:3000/timeless"
echo ""
echo "   🤖 Ollama Health Check:"
echo "      http://localhost:3000/api/ollama/health"
echo ""
echo "   🇮🇱 עיבוד עברית:"
echo "      http://localhost:3000/api/hebrew"
echo ""
echo "🛑 כדי לעצור:"
echo "   kill $CALCLAW_PID $OLLAMA_PID 2>/dev/null"
echo ""
echo "📝 לוגים:"
echo "   Calclaw: /tmp/calclaw_timeless.log"
echo "   Ollama: /tmp/ollama_calclaw.log"
echo ""
echo "💡 טיפ: פתח את הדפדפן ונווט ל: http://localhost:3000/timeless"

# שמור את ה-PIDs לקובץ
echo "$CALCLAW_PID $OLLAMA_PID" > /tmp/calclaw_timeless_pids.txt

# המתן להקשה
read -p "הקש Enter כדי לעצור את כל השרתים..."

echo "🛑 עוצר שרתים..."
kill $CALCLAW_PID $OLLAMA_PID 2>/dev/null
echo "✅ Calclaw עם Timeless Squads נעצר"
EOF

chmod +x start_calclaw_with_timeless.sh
echo "✅ סקריפט הפעלה נוצר: start_calclaw_with_timeless.sh"

# סקריפט בדיקה
cat > test_timeless_api.sh << 'EOF'
#!/bin/bash
echo "🧪 בודק Timeless Squads API..."

# המתן קצת אם השרת רק התחיל
sleep 2

# בדוק סטטוס כללי
echo "🔍 בדיקת בריאות המערכת..."
curl -s http://localhost:3000/api/health

echo ""
echo "🔍 בדיקת Ollama..."
curl -s http://localhost:3000/api/ollama/health

echo ""
echo "🧪 יצירת צוות לדוגמה..."
curl -X POST http://localhost:3000/api/timeless/squads \
  -H 'Content-Type: application/json' \
  -d '{
    "name": "צוות בדיקה",
    "description": "צוות לבדיקת המערכת",
    "settings": {
      "inheritance_enabled": true,
      "real_time_processing": false,
      "auto_task_creation": true,
      "allowed_integrations": [],
      "notification_channels": []
    }
  }' 2>/dev/null || echo "❌ לא ניתן ליצור צוות (ייתכן שהשרת לא רץ)"

echo ""
echo "✅ בדיקות הושלמו"
EOF

chmod +x test_timeless_api.sh
echo "✅ סקריפט בדיקה נוצר: test_timeless_api.sh"

# סקריפט יצירת צוות עירוני
cat > create_municipal_squad.sh << 'EOF'
#!/bin/bash
echo "🏙️ יוצר צוות AI עירוני לדוגמה..."

curl -X POST http://localhost:3000/api/timeless/squads \
  -H 'Content-Type: application/json' \
  -d '{
    "name": "צוות AI עירוני",
    "description": "צוות לניהול עירוני חכם עם Mapillary, GIS, ואוטומציה",
    "settings": {
      "inheritance_enabled": true,
      "real_time_processing": true,
      "auto_task_creation": true,
      "allowed_integrations": ["mapillary", "gis", "municipal_apis"],
      "notification_channels": ["slack", "telegram"]
    }
  }'

echo ""
echo "✅ צוות נוצר (אם השרת רץ)"
EOF

chmod +x create_municipal_squad.sh
echo "✅ סקריפט יצירת צוות נוצר: create_municipal_squad.sh"

echo ""
echo "🎉 התקנת Calclaw עם Timeless Squads הושלמה!"
echo ""
echo "📖 מה לעשות עכשיו:"
echo ""
echo "1. 🔧 ודא ש-Ollama רץ עם מודלים:"
echo "   ollama serve &"
echo "   ollama pull phi3:mini"
echo ""
echo "2. 🚀 הפעל את Calclaw:"
echo "   ./start_calclaw_with_timeless.sh"
echo ""
echo "3. 🌐 פתח בדפדפן:"
echo "   http://localhost:3000/timeless"
echo ""
echo "4. 🧪 בדוק את המערכת:"
echo "   ./test_timeless_api.sh"
echo ""
echo "5. 🏙️ צור צוות עירוני:"
echo "   ./create_municipal_squad.sh"
echo ""
echo "📚 תיעוד מלא:"
echo "   קרא את INSTALL_WITH_TIMELESS.md לפרטים נוספים"
echo ""
echo "🦾 בהצלחה עם Calclaw ועם Timeless Cal Squads!"