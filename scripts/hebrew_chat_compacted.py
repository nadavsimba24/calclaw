#!/usr/bin/env python3
"""
Calclaw Hebrew Chat עם קומפקטינג חכם
"""

import sys
import json
from datetime import datetime
from hebrew_chat import HebrewChat
from context_compactor import ContextCompactor

class CompactHebrewChat(HebrewChat):
    """Hebrew Chat עם קומפקטינג אוטומטי"""
    
    def __init__(self, max_context_tokens: int = 800, max_messages: int = 15):
        super().__init__()
        self.compactor = ContextCompactor(
            max_tokens=max_context_tokens,
            max_messages=max_messages
        )
        self.context_tokens = 0
        
    def load_history(self):
        """טען היסטוריה עם קומפקטינג"""
        history_file = "/home/erez/.openclaw/workspace/calclaw/chat_history.json"
        try:
            with open(history_file, "r", encoding="utf-8") as f:
                self.history = json.load(f)
            
            # דחוס את ההיסטוריה הנטענת
            self.history, self.context_tokens = self.compactor.compact_messages(self.history)
            print(f"📂 נטענו {len(self.history)} הודעות ({self.context_tokens} טוקנים)")
            
        except FileNotFoundError:
            self.history = []
            self.context_tokens = 0
            print("📭 אין היסטוריה קודמת")
        except Exception as e:
            print(f"⚠️  שגיאה בטעינת היסטוריה: {e}")
            self.history = []
            self.context_tokens = 0
    
    def save_history(self):
        """שמור היסטוריה עם קומפקטינג"""
        history_file = "/home/erez/.openclaw/workspace/calclaw/chat_history.json"
        
        # דחוס לפני שמירה
        compacted_history, tokens = self.compactor.compact_messages(self.history, self.context_tokens)
        
        try:
            with open(history_file, "w", encoding="utf-8") as f:
                json.dump(compacted_history, f, ensure_ascii=False, indent=2)
            
            print(f"💾 נשמרו {len(compacted_history)} הודעות ({tokens} טוקנים)")
            
        except Exception as e:
            print(f"❌ שגיאה בשמירת היסטוריה: {e}")
    
    def get_context_summary(self) -> str:
        """קבל סיכום קונטקסט"""
        if not self.history:
            return "אין היסטוריה קודמת"
        
        return self.compactor.summarize_conversation(self.history[-10:])  # רק 10 האחרונות
    
    def show_context_status(self):
        """הצג סטטוס קונטקסט"""
        if self.history:
            latest = self.history[-1]
            time_ago = self.get_time_ago(latest.get("timestamp", ""))
            
            print(f"📊 קונטקסט: {len(self.history)} הודעות, {self.context_tokens} טוקנים")
            print(f"⏰ אחרונה: {time_ago}")
            print(f"📝 סיכום: {self.get_context_summary()[:100]}...")
        else:
            print("📊 קונטקסט: ריק")
    
    def get_time_ago(self, timestamp: str) -> str:
        """קבל זמן שעבר מטקסט timestamp"""
        try:
            msg_time = datetime.fromisoformat(timestamp.replace('Z', '+00:00'))
            now = datetime.now(msg_time.tzinfo if msg_time.tzinfo else None)
            diff = now - msg_time
            
            if diff.days > 0:
                return f"לפני {diff.days} ימים"
            elif diff.seconds > 3600:
                hours = diff.seconds // 3600
                return f"לפני {hours} שעות"
            elif diff.seconds > 60:
                minutes = diff.seconds // 60
                return f"לפני {minutes} דקות"
            else:
                return "ממש עכשיו"
        except:
            return "זמן לא ידוע"
    
    def welcome_message(self):
        """הודעת ברוכים הבאים עם סטטוס קונטקסט"""
        welcome = super().welcome_message()
        
        print("\n" + "=" * 50)
        self.show_context_status()
        print("=" * 50)
        
        return welcome
    
    def process_command(self, user_input: str):
        """עבד פקודה עם ניהול קונטקסט"""
        # בדוק אם זו פקודת ניהול קונטקסט
        if user_input.lower() in ['קונטקסט', 'context', 'היסטוריה']:
            self.show_context_status()
            return "📊 הנה סטטוס הקונטקסט הנוכחי"
        
        elif user_input.lower() in ['נקה קונטקסט', 'clear context', 'אפס']:
            self.history = []
            self.context_tokens = 0
            self.save_history()
            return "🧹 ניקיתי את הקונטקסט. מתחילים מחדש!"
        
        elif user_input.lower().startswith('סיכום'):
            summary = self.get_context_summary()
            return f"📝 סיכום השיחה:\n{summary}"
        
        # עבד פקודה רגילה
        response = super().process_command(user_input)
        
        # עדכן סטטוס קונטקסט
        if self.history:
            latest = self.history[-1]
            if latest.get("user") == user_input and latest.get("response") == response:
                # ההודעה כבר נוספה - רק עדכן טוקנים
                self.history, self.context_tokens = self.compactor.compact_messages(
                    self.history, 
                    self.context_tokens
                )
            else:
                # הוסף הודעה חדשה
                self.history.append({
                    "timestamp": datetime.now().isoformat(),
                    "user": user_input,
                    "intent": latest.get("intent") if len(self.history) > 0 else None,
                    "response": response
                })
                
                # דחוס
                self.history, self.context_tokens = self.compactor.compact_messages(
                    self.history, 
                    self.context_tokens
                )
        
        # הצג סטטוס מעודכן כל 5 הודעות
        if len(self.history) % 5 == 0:
            print(f"\n📊 קונטקסט: {len(self.history)} הודעות, {self.context_tokens} טוקנים")
        
        return response
    
    def run_interactive(self):
        """הרץ שיחה אינטראקטיבית עם קומפקטינג"""
        print("🚀 מפעיל Calclaw עם קומפקטינג חכם...")
        
        # טען היסטוריה
        self.load_history()
        
        # הצג ברוכים הבאים
        self.welcome_message()
        
        print("\n💡 פקודות קונטקסט:")
        print("• 'קונטקסט' - הצג סטטוס")
        print("• 'נקה קונטקסט' - אפס היסטוריה")
        print("• 'סיכום' - סיכום השיחה")
        print("• 'יציאה' - סיום")
        
        print("\n" + "=" * 50)
        
        message_count = 0
        while True:
            try:
                user_input = input("\n📝 אתה: ").strip()
                
                if user_input.lower() in ['יציאה', 'exit', 'quit', 'bye']:
                    print("\n👋 נשמור את הקונטקסט ונתנתק. להתראות!")
                    self.save_history()
                    break
                
                if not user_input:
                    continue
                
                # עבד את הפקודה
                response = self.process_command(user_input)
                
                # הדפס את התגובה
                print(f"\n🤖 Calclaw: {response}")
                
                message_count += 1
                
                # שמור אוטומטית כל 3 הודעות
                if message_count % 3 == 0:
                    self.save_history()
                
            except KeyboardInterrupt:
                print("\n\n💾 שומר קונטקסט...")
                self.save_history()
                print("👋 נתנתקים. להתראות!")
                break
            except Exception as e:
                print(f"\n❌ שגיאה: {e}")
                print("💡 נסה שוב או פנה לתמיכה")

def main():
    """פונקציה ראשית"""
    # קבל הגדרות מהמשתמש (אופציונלי)
    import argparse
    
    parser = argparse.ArgumentParser(description="Calclaw עם קומפקטינג")
    parser.add_argument("--tokens", type=int, default=800, help="מקסימום טוקנים בקונטקסט")
    parser.add_argument("--messages", type=int, default=15, help="מקסימום הודעות בקונטקסט")
    
    args = parser.parse_args()
    
    # הרץ את הצ'אט
    chat = CompactHebrewChat(
        max_context_tokens=args.tokens,
        max_messages=args.messages
    )
    chat.run_interactive()

if __name__ == "__main__":
    main()