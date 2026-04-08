לחה!"
echo ""
echo "📋 סיכום ההתקנה:"
echo "================="

if [[ "$INSTALL_OLLAMA" == true ]]; then
    echo "✅ Ollama - מותקן ופועל"
    echo "   📦 מודלים: phi3:mini, gemma2:9b, phi3:3.8b"
    echo "   🌐 API: http://localhost:11434"
fi

echo "✅ Rust - מותקן: $(rustc --version | cut -d' ' -f2)"
echo "✅ Calclaw - נבנה ומוכן"
echo "   📁 בינארי: ./target/release/calclaw"
echo "   🌐 שרת: http://localhost:3000"

echo "✅ סקריפטי ניהול:"
echo "   🚀 ./start_calclaw.sh - הפעלת המערכת"
echo "   🛑 ./stop_calclaw.sh - עצירת המערכת"
echo "   🔄 ./update_calclaw.sh - עדכון המערכת"

if [[ "$INSTALL_TIMELESS" == true ]]; then
    echo "✅ Timeless Squads - מותקן"
    echo "   🌐 דשבורד: http://localhost:3000/timeless"
fi

if [[ "$INSTALL_EXTRAS" == true ]]; then
    echo "✅ פיצ'רים נוספים - מותקנים"
    echo "   🐍 Python scripts - זמינים להרצה"
fi

echo ""
echo "🚀 שלבים הבאים:"
echo "   1. הפעל את המערכת: ./start_calclaw.sh"
echo "   2. פתח בדפדפן: http://localhost:3000"
echo "   3. קרא את התיעוד: cat QUICK_START.md"
echo "   4. התחל לעבוד עם Calclaw!"
echo ""
echo "📚 משאבים נוספים:"
echo "   📖 תיעוד מלא: https://docs.calclaw.com"
echo "   💬 קהילה: https://community.calclaw.com"
echo "   🎥 הדרכות: https://youtube.com/calclaw"
echo "   🐙 קוד מקור: https://github.com/calclaw"
echo ""
echo "🎯 טיפים לשימוש:"
echo "   • השתמש ב-Ctrl+C בטרמינל לעצירת השרת"
echo "   • שמור את הלוגים לפתרון בעיות"
echo "   • עדכן מעת לעת עם ./update_calclaw.sh"
echo "   • גבה את הקונפיגורציה שלך"
echo ""
echo "🦾 Calclaw פרטי מוכן לשימוש! בהצלחה! 🎉"

# שאל אם לרוץ בדיקות
echo ""
read -p "🧪 להריץ בדיקות מערכת? (y/n): " RUN_TESTS

if [[ "$RUN_TESTS" == "y" || "$RUN_TESTS" == "Y" ]]; then
    echo ""
    echo "🧪 מריץ בדיקות מערכת..."
    
    # בדוק אם Calclaw רץ
    echo "1. 🔍 בודק אם Calclaw רץ..."
    ./start_calclaw.sh &
    sleep 5
    
    if curl -s http://localhost:3000 > /dev/null; then
        echo "   ✅ Calclaw רץ בהצלחה"
    else
        echo "   ❌ Calclaw לא רץ. בדוק: tail -f /tmp/calclaw.log"
    fi
    
    # בדוק אם Ollama רץ
    if [[ "$INSTALL_OLLAMA" == true ]]; then
        echo ""
        echo "2. 🔍 בודק אם Ollama רץ..."
        if curl -s http://localhost:11434/api/tags > /dev/null; then
            echo "   ✅ Ollama רץ בהצלחה"
            
            # בדוק מודלים
            echo ""
            echo "3. 📦 בודק מודלי AI..."
            MODELS=$(curl -s http://localhost:11434/api/tags | python3 -c "import sys,json;d=json.load(sys.stdin);print(len(d.get('models',[])))" 2>/dev/null || echo "0")
            echo "   ✅ נמצאו $MODELS מודלים"
        else
            echo "   ❌ Ollama לא רץ. הפעל עם: ollama serve &"
        fi
    fi
    
    # בדוק API endpoints
    echo ""
    echo "4. 🌐 בודק API endpoints..."
    if curl -s http://localhost:3000/api/health > /dev/null; then
        echo "   ✅ API health check עובד"
    else
        echo "   ❌ API health check נכשל"
    fi
    
    echo ""
    echo "✅ בדיקות הושלמו!"
    echo ""
    echo "🎯 אם היו שגיאות, בדוק:"
    echo "   • הלוגים: tail -f /tmp/calclaw.log"
    echo "   • תיעוד פתרון בעיות ב-QUICK_START.md"
    echo "   • שאל בקהילה: https://community.calclaw.com"
fi

echo ""
echo "🏁 התקנה הושלמה! 🎉"
echo ""
echo "להפעלה: ./start_calclaw.sh"
echo "לדפדפן: http://localhost:3000"
echo ""
echo "Calclaw פרטי - AI מקומי לשימוש אישי 🦾"