#!/bin/bash

# 🕒 Calclaw Cron Monitor
# רצה כל שעה לניטור ותחזוקה

LOG_FILE="/home/erez/.openclaw/workspace/calclaw/cron.log"
TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S')

echo "" >> "$LOG_FILE"
echo "=== Calclaw Cron Check - $TIMESTAMP ===" >> "$LOG_FILE"

# 1. בדוק Ollama
echo "[$TIMESTAMP] בדיקת Ollama..." >> "$LOG_FILE"
if curl -s http://localhost:11434/api/tags > /dev/null 2>&1; then
    MODEL_COUNT=$(curl -s http://localhost:11434/api/tags | python3 -c "import sys,json; data=json.load(sys.stdin); print(len(data.get('models', [])))" 2>/dev/null || echo "0")
    echo "  ✅ Ollama רץ עם $MODEL_COUNT מודלים" >> "$LOG_FILE"
    
    # בדוק זיכרון
    MEMORY_USAGE=$(ps aux | grep ollama | grep -v grep | awk '{sum += $4} END {print sum}')
    if [ -n "$MEMORY_USAGE" ]; then
        echo "  📊 זיכרון: ${MEMORY_USAGE}%" >> "$LOG_FILE"
    fi
else
    echo "  ❌ Ollama לא רץ" >> "$LOG_FILE"
    echo "  🚀 מנסה להפעיל..." >> "$LOG_FILE"
    ollama serve >> /home/erez/.openclaw/workspace/calclaw/ollama_cron.log 2>&1 &
    sleep 3
fi

# 2. בדוק דיסק
echo "[$TIMESTAMP] בדיקת דיסק..." >> "$LOG_FILE"
OLLAMA_SIZE=$(du -sh ~/.ollama 2>/dev/null | cut -f1)
CALCLAW_SIZE=$(du -sh /home/erez/.openclaw/workspace/calclaw 2>/dev/null | cut -f1)
echo "  💾 Ollama: ${OLLAMA_SIZE:-לא ידוע}" >> "$LOG_FILE"
echo "  💾 Calclaw: ${CALCLAW_SIZE:-לא ידוע}" >> "$LOG_FILE"

# 3. בדוק לוגים
echo "[$TIMESTAMP] בדיקת לוגים..." >> "$LOG_FILE"
LOG_FILES=$(find /home/erez/.openclaw/workspace/calclaw -name "*.log" -type f 2>/dev/null | wc -l)
echo "  📝 $LOG_FILES קבצי לוג" >> "$LOG_FILE"

# 4. בדוק גישה
echo "[$TIMESTAMP] בדיקת גישה..." >> "$LOG_FILE"
if [ -f "/home/erez/.openclaw/workspace/calclaw/ollama_dashboard.html" ]; then
    echo "  ✅ דשבורד HTML זמין" >> "$LOG_FILE"
else
    echo "  ⚠️  דשבורד HTML חסר" >> "$LOG_FILE"
fi

# 5. בדוק סקריפטים
echo "[$TIMESTAMP] בדיקת סקריפטים..." >> "$LOG_FILE"
SCRIPTS=("start_simple.sh" "test_simple.sh" "cli_simple.py")
MISSING=0
for script in "${SCRIPTS[@]}"; do
    if [ ! -f "/home/erez/.openclaw/workspace/calclaw/$script" ]; then
        MISSING=$((MISSING + 1))
    fi
done
if [ $MISSING -eq 0 ]; then
    echo "  ✅ כל הסקריפטים זמינים" >> "$LOG_FILE"
else
    echo "  ⚠️  חסרים $MISSING סקריפטים" >> "$LOG_FILE"
fi

# 6. בדוק יצירת טקסט (פעם ביום)
HOUR=$(date +%H)
if [ "$HOUR" = "03" ]; then  # רק ב-3 בלילה
    echo "[$TIMESTAMP] בדיקת AI (יומי)..." >> "$LOG_FILE"
    RESPONSE=$(curl -s -X POST http://localhost:11434/api/generate \
      -H "Content-Type: application/json" \
      -d '{"model": "phi3:mini", "prompt": "תגיד מה השעה", "stream": false}' 2>/dev/null)
    
    if echo "$RESPONSE" | grep -q "response"; then
        echo "  ✅ AI עובד" >> "$LOG_FILE"
    else
        echo "  ❌ AI לא עובד" >> "$LOG_FILE"
    fi
fi

echo "=== סיום בדיקה - $TIMESTAMP ===" >> "$LOG_FILE"

# שמור רק 1000 שורות אחרונות
tail -n 1000 "$LOG_FILE" > "${LOG_FILE}.tmp" && mv "${LOG_FILE}.tmp" "$LOG_FILE"