#!/bin/bash

echo "🚀 מתחיל Calclaw עם Timeless Cal Squads..."
echo "=========================================="

# בדוק אם Ollama רץ
if ! curl -s http://localhost:11434/api/tags > /dev/null; then
    echo "❌ Ollama לא רץ. מפעיל..."
    cd /home/erez
    ~/.local/bin/ollama serve > /tmp/ollama_calclaw.log 2>&1 &
    OLLAMA_PID=$!
    sleep 5
    echo "✅ Ollama רץ (PID: $OLLAMA_PID)"
else
    echo "✅ Ollama כבר רץ"
fi

# בדוק מודלים
echo "📦 בדיקת מודלים זמינים..."
MODELS=$(curl -s http://localhost:11434/api/tags | python3 -c "import sys, json; data=json.load(sys.stdin); print('\n'.join([m['name'] for m in data.get('models', [])]))")

if [ -z "$MODELS" ]; then
    echo "⚠️  אין מודלים זמינים. ממליץ להוריד:"
    echo "   ollama pull phi3:mini"
    echo "   ollama pull gemma2:9b"
    echo "ממשיך בכל זאת..."
else
    echo "✅ מודלים זמינים:"
    echo "$MODELS"
fi

# בנה את Calclaw
echo "🔨 בונה את Calclaw עם Timeless Squads..."
cd /home/erez/.openclaw/workspace/calclaw

if [ ! -f "Cargo.toml" ]; then
    echo "❌ קובץ Cargo.toml לא נמצא!"
    exit 1
fi

echo "📦 מתקין תלויות Rust..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "❌ בנייה נכשלה. בדוק את השגיאות למעלה."
    exit 1
fi

echo "✅ Calclaw נבנה בהצלחה!"

# הרץ את Calclaw
echo "🌐 מפעיל את Calclaw על פורט 3000..."
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
echo "   👥 ניהול משתמשים:"
echo "      http://localhost:3000/api/users"
echo ""
echo "📖 דוגמת שימוש ב-Timeless API:"
echo ""
echo "   # צור צוות עירוני"
echo "   curl -X POST http://localhost:3000/api/timeless/squads \\"
echo "        -H 'Content-Type: application/json' \\"
echo "        -d '{\"name\":\"צוות AI עירוני\",\"description\":\"צוות לניהול עירוני חכם\",\"settings\":{\"inheritance_enabled\":true,\"real_time_processing\":true,\"auto_task_creation\":true,\"allowed_integrations\":[\"mapillary\",\"gis\"],\"notification_channels\":[\"slack\"]}}'"
echo ""
echo "   # שיחה עם סוכן"
echo "   curl -X POST http://localhost:3000/api/timeless/agents/:agent_id/chat \\"
echo "        -H 'Content-Type: application/json' \\"
echo "        -d '{\"message\":\"תנתח את נתוני האשפה\",\"user_id\":\"user123\"}'"
echo ""
echo "   # עיבוד פגישה"
echo "   curl -X POST http://localhost:3000/api/timeless/squads/:squad_id/meeting \\"
echo "        -H 'Content-Type: application/json' \\"
echo "        -d '{\"transcript\":\"סיכמנו לשלוח הצעת מחיר עד יום חמישי\",\"platform\":\"zoom\"}'"
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