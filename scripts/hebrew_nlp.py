#!/usr/bin/env python3
"""
Calclaw Hebrew NLP - מודול הבנת עברית
"""

import re
import json
from typing import Dict, List, Optional, Tuple
import requests

class HebrewNLP:
    """מערכת NLP בעברית ל-Calclaw"""
    
    def __init__(self, ollama_url: str = "http://localhost:11434"):
        self.ollama_url = ollama_url
        self.model = "phi3:mini"  # מודל שתומך בעברית
        
        # מילון פעלים בעברית
        self.hebrew_verbs = {
            # גיבוי
            "גבה": "backup", "גבה לי": "backup", "תגבה": "backup",
            "גיבוי": "backup", "עשה גיבוי": "backup",
            
            # ניקוי
            "נקה": "clean", "תנקה": "clean", "נקה לי": "clean",
            "ניקוי": "clean", "תעשה ניקוי": "clean",
            "מחק": "delete", "תמחק": "delete",
            
            # בדיקה
            "בדוק": "check", "תבדוק": "check", "בדוק לי": "check",
            "סטטוס": "status", "תראה סטטוס": "status",
            
            # התקנה
            "התקן": "install", "תתקין": "install",
            "הורד": "download", "תוריד": "download",
            
            # הפעלה
            "הפעל": "start", "תפעיל": "start",
            "עצור": "stop", "תעצור": "stop",
            
            # ניהול
            "נהל": "manage", "תנהל": "manage",
            "צור": "create", "תיצור": "create",
            "ערוך": "edit", "תערוך": "edit",
        }
        
        # מילון עצמים בעברית
        self.hebrew_objects = {
            "קבצים": "files",
            "לוגים": "logs",
            "מערכת": "system",
            "שרת": "server",
            "בסיס נתונים": "database",
            "אתר": "website",
            "אפליקציה": "application",
            "תוכנה": "software",
        }
        
        # מילון תדירויות בעברית
        self.hebrew_frequencies = {
            "כל יום": "daily",
            "יומי": "daily",
            "כל שבוע": "weekly",
            "שבועי": "weekly",
            "כל חודש": "monthly",
            "חודשי": "monthly",
            "כל שעה": "hourly",
            "כל דקה": "minutely",
            "תמיד": "always",
            "פעם אחת": "once",
        }
        
        # מילון זמנים בעברית
        self.hebrew_times = {
            "בלילה": "night",
            "בבוקר": "morning",
            "בצהריים": "noon",
            "בערב": "evening",
            "בחצות": "midnight",
        }
    
    def is_hebrew(self, text: str) -> bool:
        """בדוק אם הטקסט מכיל עברית"""
        hebrew_range = '\u0590-\u05FF'
        return bool(re.search(f'[{hebrew_range}]', text))
    
    def extract_intent(self, text: str) -> Dict:
        """חלץ כוונה מטקסט בעברית"""
        text_lower = text.lower()
        
        # ניתוח בסיסי עם regex
        intent = {
            "action": None,
            "object": None,
            "frequency": None,
            "time": None,
            "raw_text": text,
            "is_hebrew": self.is_hebrew(text),
            "confidence": 0.0
        }
        
        # חלץ פעולה
        for hebrew_verb, action in self.hebrew_verbs.items():
            if hebrew_verb in text_lower:
                intent["action"] = action
                intent["confidence"] += 0.3
                break
        
        # חלץ עצם
        for hebrew_obj, obj in self.hebrew_objects.items():
            if hebrew_obj in text_lower:
                intent["object"] = obj
                intent["confidence"] += 0.2
                break
        
        # חלץ תדירות
        for hebrew_freq, freq in self.hebrew_frequencies.items():
            if hebrew_freq in text_lower:
                intent["frequency"] = freq
                intent["confidence"] += 0.2
                break
        
        # חלץ זמן
        for hebrew_time, time in self.hebrew_times.items():
            if hebrew_time in text_lower:
                intent["time"] = time
                intent["confidence"] += 0.1
                break
        
        # בדוק מספרים (שעות)
        time_match = re.search(r'(\d+)\s*(שעה|דקה|שניות)?', text)
        if time_match:
            intent["time"] = time_match.group(1)
            intent["confidence"] += 0.1
        
        # אם לא מצאנו כוונה ברורה, נשתמש ב-Ollama
        if intent["confidence"] < 0.5:
            return self.analyze_with_llm(text)
        
        return intent
    
    def analyze_with_llm(self, text: str) -> Dict:
        """ניתוח טקסט מתקדם עם Ollama"""
        try:
            prompt = f"""
            המשתמש אמר: "{text}"
            
            אנא פרק את המשפט לכוונה ברורה בעברית.
            החזר ב-JSON עם השדות:
            - action: הפעולה המבוקשת (backup, clean, check, install, etc.)
            - object: העצם שעליו לפעול (files, system, logs, etc.)
            - frequency: תדירות (daily, weekly, once, etc.)
            - time: זמן ספציפי אם מוזכר
            - description: תיאור הפעולה בעברית
            
            דוגמה:
            קלט: "תגבה לי את הקבצים כל יום ב-2 בלילה"
            פלט: {{
                "action": "backup",
                "object": "files",
                "frequency": "daily",
                "time": "2",
                "description": "גיבוי קבצים כל יום ב-2 בלילה"
            }}
            
            קלט: "תנקה לי את הלוגים כל שבוע"
            פלט: {{
                "action": "clean",
                "object": "logs",
                "frequency": "weekly",
                "time": null,
                "description": "ניקוי לוגים כל שבוע"
            }}
            
            JSON בלבד:
            """
            
            response = requests.post(
                f"{self.ollama_url}/api/generate",
                json={
                    "model": self.model,
                    "prompt": prompt,
                    "stream": False,
                    "options": {"temperature": 0.1}
                },
                timeout=30
            )
            
            if response.status_code == 200:
                result = response.json()
                llm_text = result.get("response", "")
                
                # חלץ JSON מהתשובה
                json_match = re.search(r'\{.*\}', llm_text, re.DOTALL)
                if json_match:
                    llm_json = json.loads(json_match.group())
                    llm_json["confidence"] = 0.8
                    llm_json["raw_text"] = text
                    llm_json["is_hebrew"] = self.is_hebrew(text)
                    return llm_json
        
        except Exception as e:
            print(f"שגיאה בניתוח עם LLM: {e}")
        
        # נפילה חזרה לניתוח בסיסי
        basic_intent = self.extract_intent(text)
        basic_intent["confidence"] = 0.3  # ביטחון נמוך
        return basic_intent
    
    def generate_response(self, intent: Dict, success: bool = True) -> str:
        """צור תשובה בעברית לפי הכוונה"""
        
        responses = {
            "backup": {
                "success": [
                    "✅ יצרתי גיבוי אוטומטי!",
                    "🎯 הגיבוי מוכן וירוץ לפי לוח הזמנים שקבעת",
                    "📦 הקבצים שלך יגובו אוטומטית",
                ],
                "failure": [
                    "❌ לא הצלחתי ליצור גיבוי",
                    "⚠️ יש בעיה עם הגדרות הגיבוי",
                    "🔧 צריך לבדוק את ההרשאות",
                ]
            },
            "clean": {
                "success": [
                    "🧹 ניקיתי את הלוגים הישנים!",
                    "✅ המערכת נקייה ומסודרת",
                    "🗑️ הקבצים המיותרים נמחקו",
                ],
                "failure": [
                    "❌ לא הצלחתי לנקות",
                    "⚠️ יש קבצים נעולים שלא ניתן למחוק",
                    "🔧 צריך הרשאות נוספות",
                ]
            },
            "check": {
                "success": [
                    "🔍 בדקתי את המערכת - הכל תקין!",
                    "✅ כל השירותים פועלים כשורה",
                    "📊 הנה דוח סטטוס מלא",
                ],
                "failure": [
                    "❌ לא הצלחתי לבדוק את המערכת",
                    "⚠️ יש בעיות בחיבור",
                    "🔧 צריך להתקין כלי ניטור",
                ]
            },
            "install": {
                "success": [
                    "⚡ התקנתי את התוכנה בהצלחה!",
                    "✅ הכל מותקן ומוכן לשימוש",
                    "🎉 ההתקנה הושלמה בהצלחה",
                ],
                "failure": [
                    "❌ ההתקנה נכשלה",
                    "⚠️ יש בעיות תלויות",
                    "🔧 צריך להתקין חבילות נוספות",
                ]
            }
        }
        
        # קבל רשימת תגובות לפי סוג פעולה
        action = intent.get("action", "unknown")
        if action in responses:
            response_list = responses[action]["success" if success else "failure"]
        else:
            response_list = ["✅ הפעולה הושלמה" if success else "❌ הפעולה נכשלה"]
        
        # בחר תגובה אקראית
        import random
        base_response = random.choice(response_list)
        
        # הוסף פרטים ספציפיים
        details = []
        
        if intent.get("object"):
            objects_hebrew = {v: k for k, v in self.hebrew_objects.items()}
            obj_hebrew = objects_hebrew.get(intent["object"], intent["object"])
            details.append(f"עבור {obj_hebrew}")
        
        if intent.get("frequency"):
            frequencies_hebrew = {v: k for k, v in self.hebrew_frequencies.items()}
            freq_hebrew = frequencies_hebrew.get(intent["frequency"], intent["frequency"])
            details.append(f"בתדירות {freq_hebrew}")
        
        if intent.get("time"):
            details.append(f"בשעה {intent['time']}")
        
        if details:
            return f"{base_response} ({', '.join(details)})"
        
        return base_response
    
    def create_command_from_intent(self, intent: Dict) -> str:
        """צור פקודה bash לפי הכוונה"""
        action = intent.get("action")
        obj = intent.get("object")
        freq = intent.get("frequency")
        time = intent.get("time")
        
        commands = {
            "backup": {
                "files": "tar -czf /backup/$(date +%Y%m%d).tar.gz /home/erez/Documents",
                "database": "mysqldump -u root dbname > /backup/db_$(date +%Y%m%d).sql",
                "system": "rsync -av /etc /backup/etc_$(date +%Y%m%d)/",
            },
            "clean": {
                "logs": "find /var/log -name '*.log' -mtime +7 -delete",
                "temp": "rm -rf /tmp/*",
                "cache": "apt-get clean && apt-get autoclean",
            },
            "check": {
                "system": "df -h && free -h && uptime",
                "services": "systemctl list-units --type=service --state=running",
                "network": "netstat -tulpn",
            }
        }
        
        # מצא את הפקודה המתאימה
        if action in commands and obj in commands[action]:
            return commands[action][obj]
        
        # פקודת ברירת מחדל
        return f"echo 'ביצוע: {intent.get('description', 'פעולה')}'"
    
    def create_cron_schedule(self, intent: Dict) -> str:
        """צור לוח זמנים cron לפי הכוונה"""
        freq = intent.get("frequency")
        time = intent.get("time")
        
        # המרת זמן לערך מספרי
        hour = 2  # ברירת מחדל: 2 בלילה
        minute = 0
        
        if time:
            if isinstance(time, str) and time.isdigit():
                hour = int(time)
            elif time == "morning":
                hour = 8
            elif time == "noon":
                hour = 12
            elif time == "evening":
                hour = 18
            elif time == "night":
                hour = 22
        
        # המרת תדירות ל-cron
        if freq == "daily":
            return f"{minute} {hour} * * *"
        elif freq == "weekly":
            return f"{minute} {hour} * * 0"  # יום ראשון
        elif freq == "monthly":
            return f"{minute} {hour} 1 * *"
        elif freq == "hourly":
            return f"{minute} * * * *"
        elif freq == "minutely":
            return f"* * * * *"
        else:
            return f"{minute} {hour} * * *"  # ברירת מחדל: יומי
    
    def test_hebrew_understanding(self):
        """בדוק את הבנת העברית"""
        test_cases = [
            "תגבה לי את הקבצים כל יום ב-2 בלילה",
            "תנקה את הלוגים כל שבוע",
            "בדוק את סטטוס המערכת",
            "תתקין לי את התוכנה החדשה",
            "הפעל את השרת",
            "עצור את האפליקציה",
            "צור לי גיבוי של בסיס הנתונים",
            "תראה לי את הלוגים",
        ]
        
        print("🧪 בדיקת הבנת עברית:")
        print("=" * 50)
        
        for test in test_cases:
            intent = self.extract_intent(test)
            print(f"\n📝 קלט: {test}")
            print(f"   🎯 פעולה: {intent.get('action')}")
            print(f"   📦 עצם: {intent.get('object')}")
            print(f"   🕒 תדירות: {intent.get('frequency')}")
            print(f"   ⏰ זמן: {intent.get('time')}")
            print(f"   🔧 פקודה: {self.create_command_from_intent(intent)[:50]}...")
            print(f"   🕒 cron: {self.create_cron_schedule(intent)}")
            print(f"   💬 תגובה: {self.generate_response(intent)}")

def main():
    """פונקציה ראשית לבדיקה"""
    nlp = HebrewNLP()
    
    print("🦾 Calclaw Hebrew NLP - בדיקת מערכת")
    print("=" * 50)
    
    # בדוק אם Ollama רץ
    try:
        response = requests.get("http://localhost:11434/api/tags", timeout=2)
        if response.status_code == 200:
            print("✅ Ollama רץ וזמין")
        else:
            print("⚠️  Ollama לא זמין - NLP בסיסי בלבד")
    except:
        print("⚠️  Ollama לא זמין - NLP בסיסי בלבד")
    
    # הרץ בדיקות
    nlp.test_hebrew_understanding()
    
    # דוגמת שימוש אינטראקטיבית
    print("\n" + "=" * 50)
    print("💬 נסה בעצמך (הקלד 'יציאה' לסיום):")
    
    while True:
        user_input = input("\n📝 אתה: ").strip()
        if user_input.lower() in ['יציאה', 'exit', 'quit']:
            break
        
        if not user_input:
            continue
        
        # ניתוח הקלט
        intent = nlp.extract_intent(user_input)
        
        print(f"\n🤖 Calclaw:")
        print(f"   הבנתי: {intent.get('description', 'לא הבנתי לגמרי')}")
        print(f"   ביטחון: {intent.get('confidence', 0):.1%}")
        
        if intent.get('confidence', 0) > 0.5:
            # צור פקודה
            command = nlp.create_command_from_intent(intent)
            cron_schedule = nlp.create_cron_schedule(intent)
            
            print(f"   🔧 אבצע: {command}")
            print(f"   🕒 מתי: {cron_schedule}")
            print(f"   💬 {nlp.generate_response(intent)}")
            
            # שאל אם לבצע
            confirm = input("\n   ✅ לבצע? (y/n): ").lower()
            if confirm == 'y':
                print("   🚀 מבצע... (בגרסה מלאה זה היה רץ באמת)")
        else:
            print("   🤔 לא בטוח שהבנתי. תוכל לנסח שוב?")

if __name__ == "__main__":
    main()