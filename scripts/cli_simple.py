#!/usr/bin/env python3
"""
Calclaw CLI - ממשק שורת פקודה פשוט
"""

import cmd
import json
import requests
import sys
import time
from typing import List, Dict, Any

class CalclawCLI(cmd.Cmd):
    """ממשק שורת פקודה ל-Calclaw"""
    
    intro = """
    🦾 Calclaw CLI - ממשק ניהול
    
    פקודות זמינות:
      status    - הצג סטטוס מערכת
      models    - רשימת מודלים
      generate  - צור טקסט עם AI
      test      - בדוק את המערכת
      help      הצג עזרה
      exit      יציאה
    
    הקלד 'help' לפקודה ספציפית.
    """
    
    prompt = 'calclaw> '
    
    def __init__(self):
        super().__init__()
        self.ollama_url = "http://localhost:11434"
        self.current_model = "phi3:mini"
    
    def check_ollama(self) -> bool:
        """בדוק אם Ollama רץ"""
        try:
            response = requests.get(f"{self.ollama_url}/api/tags", timeout=2)
            return response.status_code == 200
        except:
            return False
    
    def get_models(self) -> List[Dict]:
        """קבל רשימת מודלים"""
        try:
            response = requests.get(f"{self.ollama_url}/api/tags", timeout=2)
            if response.status_code == 200:
                data = response.json()
                return data.get("models", [])
        except:
            pass
        return []
    
    def do_status(self, arg):
        """הצג סטטוס מערכת"""
        print("\n📊 סטטוס Calclaw")
        print("=" * 40)
        
        # בדוק Ollama
        ollama_running = self.check_ollama()
        print(f"🤖 Ollama: {'✅ פועל' if ollama_running else '❌ לא פועל'}")
        
        if ollama_running:
            models = self.get_models()
            print(f"📚 מודלים: {len(models)} זמינים")
            for model in models[:3]:  # רק 3 הראשונים
                size_gb = model.get("size", 0) / 1024 / 1024 / 1024
                print(f"   • {model.get('name')} ({size_gb:.1f}GB)")
            if len(models) > 3:
                print(f"   ... ועוד {len(models) - 3} מודלים")
        
        print(f"🎯 מודל נוכחי: {self.current_model}")
        print()
    
    def do_models(self, arg):
        """הצג רשימת מודלים זמינים"""
        if not self.check_ollama():
            print("❌ Ollama לא רץ. הפעל עם: ollama serve")
            return
        
        models = self.get_models()
        if not models:
            print("❌ אין מודלים זמינים")
            print("💡 הורד מודל: ollama pull phi3:mini")
            return
        
        print("\n📚 מודלי AI זמינים")
        print("=" * 40)
        
        for i, model in enumerate(models, 1):
            size_gb = model.get("size", 0) / 1024 / 1024 / 1024
            print(f"{i:2}. {model.get('name'):20} {size_gb:5.1f} GB")
        
        print(f"\n🎯 מודל נוכחי: {self.current_model}")
        print("💡 שנה מודל עם: use <מספר>")
    
    def do_use(self, arg):
        """שנה מודל נוכחי: use <מספר>"""
        if not arg:
            print("❌ אנא ספק מספר מודל")
            print("💡 ראה רשימה עם: models")
            return
        
        try:
            model_num = int(arg) - 1
            models = self.get_models()
            
            if 0 <= model_num < len(models):
                self.current_model = models[model_num]["name"]
                print(f"✅ שיניתי למודל: {self.current_model}")
            else:
                print(f"❌ מספר לא תקין. יש {len(models)} מודלים זמינים")
        except ValueError:
            print("❌ אנא ספק מספר תקין")
    
    def do_generate(self, arg):
        """צור טקסט עם AI: generate <טקסט>"""
        if not arg:
            print("❌ אנא ספק טקסט ליצירה")
            print("💡 דוגמה: generate כתוב לי שיר")
            return
        
        if not self.check_ollama():
            print("❌ Ollama לא רץ. הפעל עם: ollama serve")
            return
        
        print(f"\n🤖 יוצר עם {self.current_model}...")
        
        try:
            data = {
                "model": self.current_model,
                "prompt": arg,
                "stream": False
            }
            
            start_time = time.time()
            response = requests.post(
                f"{self.ollama_url}/api/generate",
                json=data,
                timeout=60
            )
            elapsed = time.time() - start_time
            
            if response.status_code == 200:
                result = response.json()
                text = result.get("response", "לא התקבלה תשובה")
                
                print(f"\n✅ נוצר תוך {elapsed:.1f} שניות")
                print("=" * 40)
                print(text)
                print("=" * 40)
            else:
                print(f"❌ שגיאה: {response.status_code}")
                print(response.text[:200])
        
        except requests.exceptions.Timeout:
            print("❌ פסק זמן. המודל כנראה גדול מדי או לא זמין")
        except Exception as e:
            print(f"❌ שגיאה: {e}")
    
    def do_test(self, arg):
        """בדוק את המערכת"""
        print("\n🧪 בודק Calclaw...")
        print("=" * 40)
        
        # בדיקה 1: Ollama
        print("1. בדיקת Ollama...", end=" ")
        if self.check_ollama():
            print("✅")
        else:
            print("❌")
            print("   💡 הפעל עם: ollama serve &")
            return
        
        # בדיקה 2: מודלים
        print("2. בדיקת מודלים...", end=" ")
        models = self.get_models()
        if models:
            print(f"✅ ({len(models)} מודלים)")
        else:
            print("❌")
            print("   💡 הורד מודל: ollama pull phi3:mini")
            return
        
        # בדיקה 3: יצירת טקסט
        print("3. בדיקת יצירת טקסט...", end=" ")
        try:
            data = {
                "model": self.current_model,
                "prompt": "תגיד שלום",
                "stream": False
            }
            response = requests.post(
                f"{self.ollama_url}/api/generate",
                json=data,
                timeout=10
            )
            
            if response.status_code == 200:
                print("✅")
                result = response.json()
                text = result.get("response", "")[:50]
                print(f"   📝 תשובה: {text}...")
            else:
                print("❌")
        
        except Exception as e:
            print(f"❌ ({e})")
        
        print("\n🎉 כל הבדיקות הושלמו!")
    
    def do_hebrew(self, arg):
        """צור טקסט בעברית: hebrew <טקסט>"""
        if not arg:
            print("❌ אנא ספק טקסט בעברית")
            return
        
        # הוסף בקשת תשובה בעברית
        hebrew_prompt = f"{arg} תשובה בעברית בבקשה."
        self.do_generate(hebrew_prompt)
    
    def do_code(self, arg):
        """צור קוד: code <שפה> <תיאור>"""
        args = arg.split(" ", 1)
        if len(args) < 2:
            print("❌ אנא ספק שפה ותיאור")
            print("💡 דוגמה: code Python פונקציה לחישוב עצרת")
            return
        
        language, description = args
        code_prompt = f"Write {language} code for: {description}"
        self.do_generate(code_prompt)
    
    def do_exit(self, arg):
        """יציאה מה-CLI"""
        print("\n👋 להתראות!")
        return True
    
    def do_quit(self, arg):
        """יציאה מה-CLI"""
        return self.do_exit(arg)
    
    def default(self, line):
        """טיפול בפקודות לא מוכרות"""
        print(f"❌ פקודה לא מוכרת: {line}")
        print("💡 הקלד 'help' לרשימת פקודות")
    
    def emptyline(self):
        """אל תעשה כלום על שורה ריקה"""
        pass
    
    def precmd(self, line):
        """לפני ביצוע פקודה"""
        # רענן סטטוס כל פעם
        return line

def main():
    """פונקציה ראשית"""
    print("🚀 מפעיל Calclaw CLI...")
    
    # בדוק אם Ollama רץ
    cli = CalclawCLI()
    
    try:
        cli.cmdloop()
    except KeyboardInterrupt:
        print("\n\n👋 Calclaw CLI נסגר. להתראות!")
    except Exception as e:
        print(f"\n❌ שגיאה: {e}")

if __name__ == "__main__":
    main()