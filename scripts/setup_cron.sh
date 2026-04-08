#!/bin/bash

# 🕒 Calclaw Cron Setup

echo "🕒 מתקין cron jobs ל-Calclaw..."
echo ""

CRON_FILE="/tmp/calclaw_cron_$$"

# נקה cron ישן (אם יש)
crontab -l 2>/dev/null | grep -v "calclaw" > "$CRON_FILE"

# הוסף cron jobs חדשים
cat >> "$CRON_FILE" << EOF

# ============================================
# 🦾 Calclaw Cron Jobs
# ============================================

# ניטור כל שעה
0 * * * * /home/erez/.openclaw/workspace/calclaw/cron_monitor.sh

# הפעל Ollama אם נפל (כל 10 דקות)
*/10 * * * * curl -s http://localhost:11434/api/tags > /dev/null 2>&1 || (echo "\$(date): מפעיל Ollama מחדש" >> /home/erez/.openclaw/workspace/calclaw/restart.log && ollama serve >> /home/erez/.openclaw/workspace/calclaw/ollama.log 2>&1 &)

# נקה לוגים ישנים כל יום ב-2 בלילה
0 2 * * * find /home/erez/.openclaw/workspace/calclaw -name "*.log" -mtime +7 -delete 2>/dev/null

# דוח יומי ב-8 בבוקר
0 8 * * * /home/erez/.openclaw/workspace/calclaw/cron_monitor.sh && echo "=== דוח יומי Calclaw ===" >> /home/erez/.openclaw/workspace/calclaw/daily_report.log

# בדיקת עדכוני מודלים כל יום ראשון ב-3 בלילה
0 3 * * 0 echo "\$(date): בדיקת עדכוני מודלים" >> /home/erez/.openclaw/workspace/calclaw/update.log && ollama pull phi3:mini 2>&1 | tee -a /home/erez/.openclaw/workspace/calclaw/update.log

EOF

# התקן את ה-cron
crontab "$CRON_FILE"
rm -f "$CRON_FILE"

echo "✅ Cron jobs הותקנו!"
echo ""
echo "📋 מה הותקן:"
echo "  1. ניטור כל שעה"
echo "  2. הפעלה אוטומטית של Ollama אם נפל"
echo "  3. ניקוי לוגים שבועי"
echo "  4. דוח יומי"
echo "  5. עדכון מודלים שבועי"
echo ""
echo "📁 קבצי לוג:"
echo "  • cron.log - לוג ניטור"
echo "  • restart.log - הפעלות מחדש"
echo "  • ollama.log - לוג Ollama"
echo "  • daily_report.log - דוחות יומיים"
echo "  • update.log - עדכוני מודלים"
echo ""
echo "🔧 ניהול:"
echo "  • צפה ב-cron: crontab -l"
echo "  • ערוך cron: crontab -e"
echo "  • מחק cron: crontab -r (זהירות!)"
echo ""
echo "🧪 בדוק עכשיו:"
echo "  ./cron_monitor.sh"
echo ""
echo "🕒 Cron פועל! המערכת תנוטר אוטומטית."