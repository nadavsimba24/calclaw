#!/bin/bash

# 🕒 Calclaw Cron Manager Launcher

echo "🕒 מפעיל מנהל Cron Jobs ל-Calclaw..."
echo ""

# בדוק דרישות
echo "🔍 בודק דרישות..."

# בדוק אם Python 3 מותקן
if ! command -v python3 &> /dev/null; then
    echo "❌ Python 3 לא מותקן"
    echo "📥 התקן עם: sudo apt-get install python3"
    exit 1
fi

# בדוק אם curses מותקן
if ! python3 -c "import curses" 2>/dev/null; then
    echo "⚠️  ספריית curses לא מותקנת"
    echo "📥 מתקין..."
    sudo apt-get update && sudo apt-get install -y python3-curses 2>/dev/null || {
        echo "❌ לא ניתן להתקין curses"
        echo "💡 השתמש בממשק הווב במקום"
    }
fi

# צור קובץ jobs אם לא קיים
if [ ! -f "/home/erez/.openclaw/workspace/calclaw/cron_jobs.json" ]; then
    echo "📄 יוצר קובץ cron jobs..."
    cat > /home/erez/.openclaw/workspace/calclaw/cron_jobs.json << 'EOF'
[
  {
    "id": 1,
    "name": "ניטור Calclaw",
    "schedule": "0 * * * *",
    "command": "/home/erez/.openclaw/workspace/calclaw/cron_monitor.sh",
    "description": "בדיקת סטטוס Calclaw כל שעה",
    "enabled": true,
    "created": "2026-04-06T20:00:00"
  },
  {
    "id": 2,
    "name": "הפעלת Ollama",
    "schedule": "*/10 * * * *",
    "command": "curl -s http://localhost:11434/api/tags > /dev/null || ollama serve >> /home/erez/.openclaw/workspace/calclaw/ollama.log 2>&1 &",
    "description": "הפעל Ollama אם נפל",
    "enabled": true,
    "created": "2026-04-06T20:00:00"
  },
  {
    "id": 3,
    "name": "ניקוי לוגים",
    "schedule": "0 2 * * *",
    "command": "find /home/erez/.openclaw/workspace/calclaw -name \"*.log\" -mtime +7 -delete 2>/dev/null",
    "description": "ניקוי לוגים ישנים כל יום",
    "enabled": true,
    "created": "2026-04-06T20:00:00"
  }
]
EOF
    echo "✅ קובץ cron jobs נוצר"
fi

# הצג אפשרויות
echo ""
echo "🎯 בחר ממשק לניהול Cron Jobs:"
echo "1. TUI גרפי (מלא עם curses)"
echo "2. CLI פשוט (שורת פקודה)"
echo "3. דשבורד אינטרנטי"
echo "4. התקן cron jobs אוטומטית"
echo "5. בדוק cron jobs קיימים"
echo ""

read -p "בחר אפשרות (1-5): " choice

case $choice in
    1)
        echo "🚀 מפעיל TUI גרפי..."
        if python3 -c "import curses" 2>/dev/null; then
            python3 cron_tui.py
        else
            echo "❌ curses לא מותקן. מפעיל CLI במקום..."
            python3 cron_manager.py list
        fi
        ;;
    2)
        echo "🚀 מפעיל CLI..."
        echo ""
        echo "📖 פקודות זמינות:"
        echo "  list                    - הצג cron jobs"
        echo "  add --name ...          - הוסף cron job"
        echo "  delete --id X           - מחק cron job"
        echo "  toggle --id X           - הפעל/כבה cron job"
        echo "  run --id X              - הרץ cron job עכשיו"
        echo ""
        echo "💡 דוגמאות:"
        echo "  python3 cron_manager.py list"
        echo "  python3 cron_manager.py add --name \"גיבוי\" --schedule \"0 2 * * *\" --command \"/home/erez/backup.sh\""
        echo ""
        read -p "הקלד פקודה (או Enter לביטול): " cmd
        if [ -n "$cmd" ]; then
            python3 cron_manager.py $cmd
        fi
        ;;
    3)
        echo "🌐 מפעיל דשבורד אינטרנטי..."
        if command -v xdg-open > /dev/null; then
            xdg-open cron_dashboard.html
        elif command -v open > /dev/null; then
            open cron_dashboard.html
        else
            echo "📄 פתח את הקובץ: cron_dashboard.html בדפדפן"
        fi
        ;;
    4)
        echo "🔧 מתקין cron jobs אוטומטית..."
        if [ -f "setup_cron.sh" ]; then
            ./setup_cron.sh
        else
            echo "❌ לא נמצא סקריפט התקנה"
            echo "💡 צור קודם cron jobs דרך הממשק"
        fi
        ;;
    5)
        echo "🔍 בודק cron jobs קיימים..."
        echo ""
        
        # בדוק cron של המשתמש
        echo "📋 Cron Jobs של המשתמש:"
        crontab -l 2>/dev/null | grep -v "^#" | grep -v "^$" || echo "  אין cron jobs"
        
        echo ""
        echo "📁 Cron Jobs ב-Calclaw:"
        if [ -f "cron_jobs.json" ]; then
            python3 -c "
import json
try:
    with open('cron_jobs.json', 'r') as f:
        jobs = json.load(f)
    print(f'  📊 {len(jobs)} jobs רשומים')
    for job in jobs:
        status = '✅' if job.get('enabled', True) else '❌'
        print(f'  {status} {job[\"name\"]}: {job[\"schedule\"]}')
except:
    print('  ❌ לא ניתן לקרוא את הקובץ')
"
        else
            echo "  ❌ קובץ cron_jobs.json לא קיים"
        fi
        ;;
    *)
        echo "❌ בחירה לא תקינה"
        ;;
esac

echo ""
echo "🕒 Cron Manager נסגר"