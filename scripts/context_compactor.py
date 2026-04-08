#!/usr/bin/env python3
"""
Calclaw Context Compactor - מערכת קומפקטינג חכמה
"""

import json
import re
from datetime import datetime, timedelta
from typing import List, Dict, Any, Tuple
from hebrew_nlp import HebrewNLP

class ContextCompactor:
    """מערכת קומפקטינג חכמה לקונטקסט"""
    
    def __init__(self, max_tokens: int = 1000, max_messages: int = 20):
        self.max_tokens = max_tokens
        self.max_messages = max_messages
        self.nlp = HebrewNLP()
        
        # הגדרות חשיבות
        self.importance_keywords = {
            "high": [
                "גבה", "גיבוי", "backup", "שמור",
                "נקה", "ניקוי", "clean", "מחק",
                "התקן", "install", "הורד",
                "סכנה", "error", "שגיאה", "בעיה"
            ],
            "medium": [
                "בדוק", "check", "סטטוס", "status",
                "הפעל", "start", "עצור", "stop",
                "צור", "create", "ערוך", "edit"
            ],
            "low": [
                "היי", "שלום", "ביי", "תודה",
                "מה נשמע", "איך הולך", "בבקשה"
            ]
        }
    
    def calculate_importance(self, text: str) -> str:
        """חשב חשיבות של הודעה"""
        text_lower = text.lower()
        
        # בדוק מילות מפתח
        for importance, keywords in self.importance_keywords.items():
            for keyword in keywords:
                if keyword in text_lower:
                    return importance
        
        # ניתוח עם NLP
        intent = self.nlp.extract_intent(text)
        confidence = intent.get("confidence", 0)
        
        if confidence > 0.7:
            return "high"
        elif confidence > 0.4:
            return "medium"
        else:
            return "low"
    
    def estimate_tokens(self, text: str) -> int:
        """הערך מספר טוקנים בטקסט"""
        # הערכה פשוטה: 1 טוקן = 4 תווים (בעברית)
        return len(text) // 4
    
    def summarize_conversation(self, messages: List[Dict]) -> str:
        """צור סיכום של שיחה"""
        if not messages:
            return "אין היסטוריה"
        
        # קבץ הודעות לפי נושא
        topics = {}
        for msg in messages:
            if msg.get("user"):
                intent = self.nlp.extract_intent(msg["user"])
                topic = intent.get("action", "כללי")
                
                if topic not in topics:
                    topics[topic] = []
                topics[topic].append(msg)
        
        # צור סיכום
        summary_parts = []
        for topic, topic_msgs in topics.items():
            if topic != "unknown":
                topic_hebrew = {
                    "backup": "גיבוי",
                    "clean": "ניקוי",
                    "check": "בדיקה",
                    "install": "התקנה",
                    "start": "הפעלה",
                    "stop": "עצירה"
                }.get(topic, topic)
                
                count = len(topic_msgs)
                last_time = topic_msgs[-1].get("timestamp", "")
                summary_parts.append(f"{topic_hebrew} ({count} פעמים, אחרון: {last_time[:10]})")
        
        if summary_parts:
            return "נושאים שנדונו: " + ", ".join(summary_parts)
        else:
            return "שיחה כללית"
    
    def compact_messages(self, messages: List[Dict], current_tokens: int = 0) -> Tuple[List[Dict], int]:
        """דחוס הודעות כדי לא לעבור את מגבלת הטוקנים"""
        if not messages:
            return [], 0
        
        # חשב טוקנים נוכחיים
        total_tokens = current_tokens
        
        # מיין הודעות לפי חשיבות וזמן (חדשות וחשובות קודם)
        sorted_messages = sorted(
            messages,
            key=lambda x: (
                self.calculate_importance(x.get("user", "")) != "low",
                x.get("timestamp", ""),
            ),
            reverse=True
        )
        
        # שמור רק את ההודעות החשובות ביותר
        compacted = []
        for msg in sorted_messages:
            msg_tokens = self.estimate_tokens(msg.get("user", "")) + self.estimate_tokens(msg.get("response", ""))
            
            # אם יש מקום, הוסף
            if total_tokens + msg_tokens <= self.max_tokens and len(compacted) < self.max_messages:
                compacted.append(msg)
                total_tokens += msg_tokens
            else:
                # במקום למחוק לגמרי, אפשר ליצור סיכום
                break
        
        # אם יש יותר מדי הודעות, צור סיכום להודעות הישנות
        if len(compacted) < len(messages):
            old_messages = messages[len(compacted):]
            summary = self.summarize_conversation(old_messages)
            summary_tokens = self.estimate_tokens(summary)
            
            # הוסף סיכום אם יש מקום
            if total_tokens + summary_tokens <= self.max_tokens:
                summary_msg = {
                    "timestamp": datetime.now().isoformat(),
                    "user": "[סיכום היסטוריה]",
                    "response": summary,
                    "compacted": True
                }
                compacted.append(summary_msg)
                total_tokens += summary_tokens
        
        # מיין חזרה לפי זמן
        compacted.sort(key=lambda x: x.get("timestamp", ""))
        
        return compacted, total_tokens
    
    def create_context_window(self, messages: List[Dict], new_message: str = None) -> str:
        """צור חלון קונטקסט אופטימלי"""
        # הוסף הודעה חדשה אם יש
        all_messages = messages.copy()
        if new_message:
            new_msg = {
                "timestamp": datetime.now().isoformat(),
                "user": new_message,
                "response": None
            }
            all_messages.append(new_msg)
        
        # דחוס את ההודעות
        compacted, tokens_used = self.compact_messages(all_messages)
        
        # צור טקסט קונטקסט
        context_lines = []
        for msg in compacted[-10:]:  # רק 10 האחרונות להצגה
            timestamp = msg.get("timestamp", "")[:19]
            user = msg.get("user", "")
            response = msg.get("response", "")
            
            if msg.get("compacted"):
                context_lines.append(f"[{timestamp}] {user}: {response}")
            else:
                context_lines.append(f"[{timestamp}] 👤: {user}")
                if response:
                    context_lines.append(f"[{timestamp}] 🤖: {response}")
        
        context_text = "\n".join(context_lines)
        
        # הוסף סיכום סטטיסטי
        stats = f"\n\n[סטטיסטיקה: {len(compacted)}/{len(all_messages)} הודעות, {tokens_used}/{self.max_tokens} טוקנים]"
        
        return context_text + stats
    
    def save_compacted_history(self, messages: List[Dict], filepath: str):
        """שמור היסטוריה דחוסה"""
        # דחוס לפני שמירה
        compacted, _ = self.compact_messages(messages)
        
        # הוסף מטא-דאטה
        history_data = {
            "messages": compacted,
            "compacted_at": datetime.now().isoformat(),
            "original_count": len(messages),
            "compacted_count": len(compacted),
            "compression_ratio": f"{len(compacted)}/{len(messages)}" if messages else "0/0"
        }
        
        with open(filepath, "w", encoding="utf-8") as f:
            json.dump(history_data, f, ensure_ascii=False, indent=2)
    
    def load_compacted_history(self, filepath: str) -> List[Dict]:
        """טען היסטוריה דחוסה"""
        try:
            with open(filepath, "r", encoding="utf-8") as f:
                data = json.load(f)
            return data.get("messages", [])
        except:
            return []

# פונקציות עזר לשימוש ב-CLI
def test_compaction():
    """בדוק את מערכת הקומפקטינג"""
    compactor = ContextCompactor(max_tokens=500, max_messages=10)
    
    # צור הודעות דוגמה
    test_messages = [
        {
            "timestamp": "2026-04-06T10:00:00",
            "user": "היי, מה נשמע?",
            "response": "שלום! אני Calclaw, איך אוכל לעזור?"
        },
        {
            "timestamp": "2026-04-06T10:01:00",
            "user": "תגבה לי את הקבצים כל יום",
            "response": "✅ יצרתי גיבוי אוטומטי שירוץ כל יום ב-2 בלילה"
        },
        {
            "timestamp": "2026-04-06T10:02:00",
            "user": "תודה רבה!",
            "response": "בכיף! תמיד כאן לעזור"
        },
        {
            "timestamp": "2026-04-06T10:03:00",
            "user": "תנקה את הלוגים כל שבוע",
            "response": "🧹 הגדרתי ניקוי לוגים אוטומטי כל יום ראשון"
        },
        {
            "timestamp": "2026-04-06T10:04:00",
            "user": "בדוק את סטטוס המערכת",
            "response": "🔍 הכל תקין! זיכרון: 45%, דיסק: 60% מלא"
        },
        {
            "timestamp": "2026-04-06T10:05:00",
            "user": "מה השעה?",
            "response": "השעה היא 10:05"
        },
        {
            "timestamp": "2026-04-06T10:06:00",
            "user": "תעשה לי גיבוי נוסף",
            "response": "✅ הוספתי גיבוי נוסף לרשימה"
        },
        {
            "timestamp": "2026-04-06T10:07:00",
            "user": "אוקיי תודה",
            "response": "בשמחה!"
        },
        {
            "timestamp": "2026-04-06T10:08:00",
            "user": "תמחק קבצים זמניים",
            "response": "🗑️ מחקתי קבצים זמניים מהמערכת"
        },
        {
            "timestamp": "2026-04-06T10:09:00",
            "user": "תראה לי את הלוגים",
            "response": "📄 הנה הלוגים האחרונים..."
        },
        {
            "timestamp": "2026-04-06T10:10:00",
            "user": "עוד משהו?",
            "response": "לא, זה הכל תודה!"
        }
    ]
    
    print("🧪 בדיקת מערכת קומפקטינג")
    print("=" * 50)
    
    print(f"📊 לפני קומפקטינג: {len(test_messages)} הודעות")
    
    # דחוס
    compacted, tokens = compactor.compact_messages(test_messages)
    
    print(f"📦 אחרי קומפקטינג: {len(compacted)} הודעות")
    print(f"🔢 טוקנים בשימוש: {tokens}")
    
    print("\n📝 חלון קונטקסט:")
    print("-" * 30)
    context = compactor.create_context_window(test_messages)
    print(context)
    
    print("\n🎯 חשיבות הודעות:")
    for msg in test_messages[:5]:
        importance = compactor.calculate_importance(msg["user"])
        print(f"  {importance.upper():6} - {msg['user'][:30]}...")
    
    print("\n💾 שמירת היסטוריה דחוסה...")
    compactor.save_compacted_history(test_messages, "/tmp/test_history.json")
    
    print("📂 טעינת היסטוריה דחוסה...")
    loaded = compactor.load_compacted_history("/tmp/test_history.json")
    print(f"   נטענו: {len(loaded)} הודעות")

def main():
    """פונקציה ראשית"""
    import argparse
    
    parser = argparse.ArgumentParser(description="Calclaw Context Compactor")
    parser.add_argument("--test", action="store_true", help="הרץ בדיקות")
    parser.add_argument("--compact", help="קובץ JSON לדחוס")
    parser.add_argument("--max-tokens", type=int, default=1000, help="מקסימום טוקנים")
    parser.add_argument("--max-messages", type=int, default=20, help="מקסימום הודעות")
    
    args = parser.parse_args()
    
    if args.test:
        test_compaction()
    elif args.compact:
        compactor = ContextCompactor(args.max_tokens, args.max_messages)
        
        try:
            with open(args.compact, "r", encoding="utf-8") as f:
                messages = json.load(f)
            
            compacted, tokens = compactor.compact_messages(messages)
            
            print(f"📊 קומפקטינג: {len(messages)} → {len(compacted)} הודעות")
            print(f"🔢 טוקנים: {tokens}/{args.max_tokens}")
            
            # שמור את הגרסה הדחוסה
            output_file = args.compact.replace(".json", "_compacted.json")
            compactor.save_compacted_history(messages, output_file)
            print(f"💾 נשמר ב: {output_file}")
            
        except Exception as e:
            print(f"❌ שגיאה: {e}")
    else:
        parser.print_help()

if __name__ == "__main__":
    main()