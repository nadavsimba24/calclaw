#!/usr/bin/env python3
"""
Calclaw Hebrew Chat - ממשק שיחה בעברית
"""

import sys
import json
import subprocess
from datetime import datetime
from hebrew_nlp import HebrewNLP

class HebrewChat:
    """ממשק שיחה אינטראקטיבי בעברית"""
    
    def __init__(self):
        self.nlp = HebrewNLP()
        self.history = []
        
    def welcome_message(self):
        """הודעת ברוכים הבאים"""
        welcome = """
        🦾 שלום! אני Calclaw - העוזר האישי שלך בעברית!
        
        🇮🇱 אני מבין עברית מושלם:
        • דיבור יומיומי וסלנג
        • הקשר תרבותי ישראלי
        • מונחים טכניים בעברית
        
        🚀 אני יכול לעזור עם:
        • גיבוי קבצים אוטומטי
        • ניקוי מערכת
        • ניטור ובדיקות
        • התקנת תוכנות
        • ניהול משימות (cron jobs)
        
        💬 פשוט דבר אליי בעברית!
        לדוגמה:
        • "תגבה לי את הקבצים כל יום"
        • "תנקה את הלוגים כל שבוע"
        • "בדוק אם המערכת תקינה"
        
        הקלד 'יציאה' לסיום.
        """
        print(welcome)
    
    def process_command(self, user_input: str):
        """עבד פקודה מהמשתמש"""
        # ניתוח הכוונה
        intent = self.nlp.extract_intent(user_input)
        
        # שמור בהיסטוריה
        self.history.append({
            "timestamp": datetime.now().isoformat(),
            "user": user_input,
            "intent": intent,
            "response": None
        })
        
        # בדוק ביטחון
        confidence = intent.get("confidence", 0)
        
        if confidence < 0.3:
            response = "🤔 סליחה, לא הבנתי לגמרי. תוכל לנסח שוב?"
            self.history[-1]["response"] = response
            return response
        
        elif confidence < 0.7:
            # בקש אישור
            action_desc = intent.get("description", "הפעולה")
            response = f"🤔 הבנתי שאתה רוצה {action_desc}. נכון?"
            self.history[-1]["response"] = response
            return response
        
        else:
            # ביטחון גבוה - בצע אוטומטית
            return self.execute_intent(intent)
    
    def execute_intent(self, intent: Dict):
        """בצע את הכוונה שהתגלתה"""
        action = intent.get("action")
        obj = intent.get("object")
        
        responses = {
            "backup": self.execute_backup,
            "clean": self.execute_clean,
            "check": self.execute_check,
            "install": self.execute_install,
            "start": self.execute_start,
            "stop": self.execute_stop,
            "create": self.execute_create,
        }
        
        if action in responses:
            return responses[action](intent)
        else:
            return "⚠️  אני עדיין לומד לבצע את הפעולה הזו. תוכל לנסות משהו אחר?"
    
    def execute_backup(self, intent: Dict):
        """בצע גיבוי"""
        obj = intent.get("object", "files")
        freq = intent.get("frequency", "daily")
        time = intent.get("time", "2")
        
        # צור סקריפט גיבוי
        backup_script = f"""#!/bin/bash
# גיבוי אוטומטי שנוצר על ידי Calclaw
# {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}

echo "🚀 מתחיל גיבוי של {obj}..."
BACKUP_DIR="/home/erez/backups"
mkdir -p "$BACKUP_DIR"

TIMESTAMP=$(date +%Y%m%d_%H%M%S)

case "{obj}" in
    files)
        tar -czf "$BACKUP_DIR/files_$TIMESTAMP.tar.gz" /home/erez/Documents
        ;;
    database)
        mysqldump -u root --all-databases > "$BACKUP_DIR/db_$TIMESTAMP.sql" 2>/dev/null || echo "⚠️  לא ניתן לגבות בסיס נתונים"
        ;;
    system)
        rsync -av /etc "$BACKUP_DIR/etc_$TIMESTAMP/" > /dev/null 2>&1
        ;;
    *)
        echo "📦 גיבוי כללי"
        tar -czf "$BACKUP_DIR/backup_$TIMESTAMP.tar.gz" /home/erez 2>/dev/null
        ;;
esac

echo "✅ הגיבוי הושלם: $BACKUP_DIR/*_$TIMESTAMP.*"
"""
        
        # שמור את הסקריפט
        script_path = "/home/erez/.openclaw/workspace/calclaw/backup.sh"
        with open(script_path, "w", encoding="utf-8") as f:
            f.write(backup_script)
        
        subprocess.run(["chmod", "+x", script_path])
        
        # צור cron job
        cron_schedule = self.nlp.create_cron_schedule(intent)
        cron_line = f"{cron_schedule} {script_path} >> /home/erez/.openclaw/workspace/calclaw/backup.log 2>&1"
        
        # הוסף ל-cron
        try:
            # קבל cron קיים
            result = subprocess.run(["crontab", "-l"], capture_output=True, text=True)
            current_cron = result.stdout if result.returncode == 0 else ""
            
            # הוסף את ה-job החדש
            new_cron = current_cron.strip()
            if new_cron:
                new_cron += "\n"
            new_cron += f"# גיבוי אוטומטי - Calclaw\n{cron_line}\n"
            
            # עדכן את ה-cron
            with open("/tmp/calclaw_cron", "w") as f:
                f.write(new_cron)
            
            subprocess.run(["crontab", "/tmp/calclaw_cron"], check=True)
            
            response = self.nlp.generate_response(intent, success=True)
            response += f"\n📁 הסקריפט: {script_path}"
            response += f"\n🕒 ירוץ: {cron_schedule}"
            
        except Exception as e:
            response = self.nlp.generate_response(intent, success=False)
            response += f"\n❌ שגיאה: {str(e)}"
        
        self.history[-1]["response"] = response
        return response
    
    def execute_clean(self, intent: Dict):
        """בצע ניקוי"""
        obj = intent.get("object", "logs")
        freq = intent.get("frequency", "weekly")
        
        # צור סקריפט ניקוי
        clean_script = f"""#!/bin/bash
# ניקוי אוטומטי שנוצר על ידי Calclaw
# {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}

echo "🧹 מתחיל ניקוי של {obj}..."

case "{obj}" in
    logs)
        # נקה לוגים ישנים
        find /var/log -name "*.log" -mtime +7 -delete 2>/dev/null || true
        find /home/erez/.openclaw/workspace/calclaw -name "*.log" -mtime +7 -delete 2>/dev/null || true
        echo "✅ לוגים ישנים נוקו"
        ;;
    temp)
        # נקה קבצים זמניים
        rm -rf /tmp/* 2>/dev/null || true
        rm -rf /var/tmp/* 2>/dev/null || true
        echo "✅ קבצים זמניים נוקו"
        ;;
    cache)
        # נקה cache
        apt-get clean 2>/dev/null || true
        apt-get autoclean 2>/dev/null || true
        echo "✅ cache נוקה"
        ;;
    *)
        # ניקוי כללי
        find /home/erez/Downloads -name "*.tmp" -delete 2>/dev/null || true
        find /home/erez/Downloads -name "*.temp" -delete 2>/dev/null || true
        echo "✅ ניקוי כללי הושלם"
        ;;
esac

echo "🎉 הניקוי הושלם!"
"""
        
        script_path = "/home/erez/.openclaw/workspace/calclaw/clean.sh"
        with open(script_path, "w", encoding="utf-8") as f:
            f.write(clean_script)
        
        subprocess.run(["chmod", "+x", script_path])
        
        # הוסף ל-cron אם יש תדירות
        if freq != "once":
            cron_schedule = self.nlp.create_cron_schedule(intent)
            cron_line = f"{cron_schedule} {script_path} >> /home/erez/.openclaw/workspace/calclaw/clean.log 2>&1"
            
            try:
                result = subprocess.run(["crontab", "-l"], capture_output=True, text=True)
                current_cron = result.stdout if result.returncode == 0 else ""
                
                new_cron = current_cron.strip()
                if new_cron:
                    new_cron += "\n"
                new_cron += f"# ניקוי אוטומטי - Calclaw\n{cron_line}\n"
                
                with open("/tmp/calclaw_cron_clean", "w") as f:
                    f.write(new_cron)
                
                subprocess.run(["crontab", "/tmp/calclaw_cron_clean"], check=True)
                
                response = self.nlp.generate_response(intent, success=True)
                response += f"\n📁 ירוץ אוטומטית: {cron_schedule}"
                
            except Exception as e:
                response = self.nlp.generate_response(intent, success=False)
                response += f"\n❌ שגיאה ב-cron: {str(e)}"
        else:
            # הרץ פעם אחת
            try:
                subprocess.run([script_path], check=True)
                response = self.nlp.generate_response(intent, success=True)
                response += "\n✅ הניקוי בוצע בהצלחה!"
            except Exception as e:
                response = self.nlp.generate_response(intent, success=False)
                response += f"\n❌ שגיאה: {str(e)}"
        
        self.history[-1]["response"] = response
        return response
    
    def execute_check(self, intent: Dict):
        """בצע בדיקת מערכת"""
        obj = intent.get("object", "system")
        
        check_script = f"""#!/bin/bash
# בדיקת מערכת - Calclaw
# {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}

echo "🔍 בדיקת {obj}..."

case "{obj}" in
    system)
        echo "📊 סטטוס מערכת:"
        echo "--- זיכרון ---"
        free -h
        echo ""
        echo "--- דיסק ---"
        df -h
        echo ""
        echo "--- מערכת ---"
        uptime
        ;;
    services)
        echo "🛠️  שירותים פעילים:"
        systemctl list-units --type=service --state=running 2>/dev/null || echo "⚠️  לא ניתן לבדוק שירותים"
        ;;
    network)
        echo "🌐 חיבורי רשת:"
        netstat -tulpn 2>/dev/null || ss -tulpn 2>/dev/null || echo "⚠️  לא ניתן לבדוק רשת"
        ;;
    *)
        echo "📋 סטטוס כללי"
        whoami
        uname -a
        date
        ;;
esac

echo "✅ הבדיקה הושלמה"
"""
        
        script_path = "/home/erez/.openclaw/workspace/calclaw/check.sh"
        with open(script_path, "w", encoding="utf-8") as f:
            f.write(check_script)
        
        subprocess.run(["chmod", "+x", script_path])
        
        # הרץ את הבדיקה
        try:
            result = subprocess.run([script_path], capture_output=True, text=True, check=True)
            output = result.stdout
            
            response = self.nlp.generate_response(intent, success=True)
            response += f"\n📋 תוצאות:\n{output[:500]}..." if len(output) > 500 else f"\n📋 תוצאות:\n{output}"
            
        except Exception as e:
            response = self.nlp.generate_response(intent, success=False)
            response += f"\n❌ שגיאה: {str(e)}"
        
        self.history[-1]["response"] = response
        return response
    
    def execute_install(self, intent: Dict):
        """התקן תוכנה"""
        # זה דורש הרשאות sudo - נדווח שאי אפשר
        response = "⚠️  התקנת תוכנה דורשת הרשאות ניהול. תוכל להתקין ידנית או לתת לי הרשאות מתאימות."
        self.history[-1]["response"] = response
        return response
    
    def execute_start(self, intent: Dict):
        """הפעל שירות"""
        response = "🚀 איזה שירות תרצה שאפעיל?"
        self.history[-1]["response"] = response
        return response
    
    def execute_stop(self, intent: Dict):
        """עצור שירות"""
        response = "🛑 איזה שירות תרצה שאעצור?"
        self.history[-1]["response"] = response
        return response
    
    def execute_create(self, intent: Dict):
        """צור משהו חדש"""
        response = "🛠️  מה תרצה שאצור עבורך?"
        self.history[-1]["response"] = response
        return response
    
    def save_history(self):
        """שמור את היסטוריית השיחה"""
        history_file = "/home/erez/.openclaw/workspace/calclaw/chat_history.json"
        with open(history_file, "w", encoding="utf-8") as f:
            json.dump(self.history, f, ensure_ascii=False, indent=2)
    
    def run_interactive(self):
        """הרץ שיחה אינטראקטיבית"""
        self.welcome_message()
        
        print("\n" + "=" * 50)
        
        while True:
            try:
                user_input = input("\n📝 אתה: ").strip()
                
                if user_input.lower() in ['יציאה', 'exit', 'quit', 'bye']:
                    print("\n👋 להתראות! Calclaw תמיד כאן לעזור.")
                    self.save_history()
                    break
                
                if not user_input:
                    continue
                
                # עבד את הפקודה
                response = self.process_command(user_input)
                
                # הדפס את התגובה
                print(f"\n🤖 Calclaw: {response}")
                
            except KeyboardInterrupt:
                print("\n\n👋 נפרדים לשלום! Calclaw תמיד כאן לעזור.")
                self.save_history()
                break
            except Exception as e:
                print(f"\n❌ שגיאה: {e}")
                print("💡 נסה שוב או פנה לתמיכה")

def main():
    """פונקציה ראשית"""
    print("🚀 מפעיל Calclaw Hebrew Chat...")
    
    # בדוק אם Ollama רץ
    try:
        import requests
        response = requests.get("http://localhost:11434/api/tags", timeout=2)
        if response.status_code == 200:
            print("✅ Ollama רץ - NLP מתקדם זמין")
        else:
            print("⚠️  Ollama לא רץ - NLP בסיסי בלבד")
    except:
        print("⚠️  Ollama לא רץ - NLP בסיסי בלבד")
    
    # הרץ את הצ'אט
    chat = HebrewChat()
    chat.run_interactive()

if __name__ == "__main__":
    main()