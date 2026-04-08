#!/usr/bin/env python3
"""
Calclaw TUI - ממשק טקסטואלי לניהול
דומה ל-OpenClaw TUI אבל עבור Calclaw
"""

import curses
import json
import subprocess
import time
import sys
from typing import List, Dict, Any
import requests

class CalclawTUI:
    def __init__(self):
        self.current_screen = "main"
        self.selected_index = 0
        self.ollama_status = "לא ידוע"
        self.models = []
        self.messages = []
        
    def check_ollama(self):
        """בדוק סטטוס Ollama"""
        try:
            response = requests.get("http://localhost:11434/api/tags", timeout=2)
            if response.status_code == 200:
                data = response.json()
                self.models = data.get("models", [])
                self.ollama_status = f"פועל ({len(self.models)} מודלים)"
                return True
            else:
                self.ollama_status = "שגיאה"
                return False
        except:
            self.ollama_status = "לא פועל"
            return False
    
    def generate_text(self, model: str, prompt: str):
        """צור טקסט עם Ollama"""
        try:
            data = {
                "model": model,
                "prompt": prompt,
                "stream": False
            }
            response = requests.post(
                "http://localhost:11434/api/generate",
                json=data,
                timeout=30
            )
            if response.status_code == 200:
                result = response.json()
                return result.get("response", "לא התקבלה תשובה")
            else:
                return f"שגיאה: {response.status_code}"
        except Exception as e:
            return f"שגיאה: {str(e)}"
    
    def draw_main_menu(self, stdscr):
        """צייר תפריט ראשי"""
        stdscr.clear()
        height, width = stdscr.getmaxyx()
        
        # כותרת
        title = "🦾 Calclaw TUI - ממשק ניהול"
        stdscr.addstr(0, (width - len(title)) // 2, title, curses.A_BOLD)
        
        # קו מפריד
        stdscr.addstr(2, 0, "=" * width)
        
        # סטטוס
        status_line = f"📊 סטטוס: Ollama - {self.ollama_status}"
        stdscr.addstr(4, 2, status_line)
        
        # תפריט
        menu_items = [
            "🤖 צור טקסט עם AI",
            "📚 רשימת מודלים",
            "🧪 בדיקת מערכת",
            "⚙️  הגדרות",
            "🚪 יציאה"
        ]
        
        for i, item in enumerate(menu_items):
            if i == self.selected_index:
                stdscr.addstr(6 + i, 4, f"> {item}", curses.A_REVERSE)
            else:
                stdscr.addstr(6 + i, 4, f"  {item}")
        
        # הוראות
        instructions = "↑↓: ניווט | Enter: בחירה | q: יציאה"
        stdscr.addstr(height - 2, (width - len(instructions)) // 2, instructions)
        
        stdscr.refresh()
    
    def draw_models_screen(self, stdscr):
        """צייר רשימת מודלים"""
        stdscr.clear()
        height, width = stdscr.getmaxyx()
        
        title = "📚 מודלי AI זמינים"
        stdscr.addstr(0, (width - len(title)) // 2, title, curses.A_BOLD)
        
        stdscr.addstr(2, 0, "=" * width)
        
        if not self.models:
            stdscr.addstr(4, 4, "❌ אין מודלים זמינים")
            stdscr.addstr(5, 4, "הפעל Ollama והורד מודלים")
        else:
            for i, model in enumerate(self.models):
                size_gb = model.get("size", 0) / 1024 / 1024 / 1024
                line = f"• {model.get('name', 'לא ידוע')} ({size_gb:.1f}GB)"
                if i == self.selected_index:
                    stdscr.addstr(4 + i, 4, line, curses.A_REVERSE)
                else:
                    stdscr.addstr(4 + i, 4, line)
        
        instructions = "↑↓: ניווט | Enter: בחירת מודל | b: חזרה"
        stdscr.addstr(height - 2, (width - len(instructions)) // 2, instructions)
        
        stdscr.refresh()
    
    def draw_generate_screen(self, stdscr):
        """צייר מסך יצירת טקסט"""
        stdscr.clear()
        height, width = stdscr.getmaxyx()
        
        title = "🤖 יצירת טקסט עם AI"
        stdscr.addstr(0, (width - len(title)) // 2, title, curses.A_BOLD)
        
        stdscr.addstr(2, 0, "=" * width)
        
        # בחר מודל
        stdscr.addstr(4, 4, "בחר מודל:")
        if self.models:
            for i, model in enumerate(self.models[:5]):  # רק 5 הראשונים
                name = model.get('name', 'לא ידוע')
                if i == self.selected_index:
                    stdscr.addstr(5 + i, 6, f"> {name}", curses.A_REVERSE)
                else:
                    stdscr.addstr(5 + i, 6, f"  {name}")
        
        # הוראות
        instructions = "↑↓: בחירת מודל | Enter: אישור | b: חזרה"
        stdscr.addstr(height - 2, (width - len(instructions)) // 2, instructions)
        
        stdscr.refresh()
    
    def draw_input_screen(self, stdscr, model_name: str):
        """מסך קלט טקסט"""
        stdscr.clear()
        height, width = stdscr.getmaxyx()
        
        title = f"📝 הקלד פקודה עבור {model_name}"
        stdscr.addstr(0, (width - len(title)) // 2, title, curses.A_BOLD)
        
        stdscr.addstr(2, 0, "=" * width)
        
        stdscr.addstr(4, 4, "פקודה:")
        stdscr.addstr(5, 4, "_" * (width - 8))
        
        curses.echo()
        stdscr.addstr(5, 4, "")
        prompt = stdscr.getstr(5, 4, width - 9).decode('utf-8')
        curses.noecho()
        
        if prompt:
            return prompt
        return None
    
    def draw_result_screen(self, stdscr, model: str, prompt: str, result: str):
        """הצג תוצאה"""
        stdscr.clear()
        height, width = stdscr.getmaxyx()
        
        title = "✅ תוצאה"
        stdscr.addstr(0, (width - len(title)) // 2, title, curses.A_BOLD)
        
        stdscr.addstr(2, 0, "=" * width)
        
        stdscr.addstr(4, 4, f"מודל: {model}")
        stdscr.addstr(5, 4, f"פקודה: {prompt[:50]}..." if len(prompt) > 50 else f"פקודה: {prompt}")
        
        stdscr.addstr(7, 4, "תשובה:")
        
        # הצג תשובה עם גלילה
        lines = result.split('\n')
        max_lines = height - 12
        start_line = 0
        
        while True:
            stdscr.clear()
            stdscr.addstr(0, (width - len(title)) // 2, title, curses.A_BOLD)
            stdscr.addstr(2, 0, "=" * width)
            stdscr.addstr(4, 4, f"מודל: {model}")
            stdscr.addstr(5, 4, f"פקודה: {prompt[:50]}..." if len(prompt) > 50 else f"פקודה: {prompt}")
            stdscr.addstr(7, 4, "תשובה:")
            
            for i, line in enumerate(lines[start_line:start_line + max_lines]):
                stdscr.addstr(9 + i, 4, line[:width - 8])
            
            instructions = "↑↓: גלילה | Enter: צור שוב | b: חזרה"
            stdscr.addstr(height - 2, (width - len(instructions)) // 2, instructions)
            
            stdscr.refresh()
            
            key = stdscr.getch()
            if key == curses.KEY_UP and start_line > 0:
                start_line -= 1
            elif key == curses.KEY_DOWN and start_line + max_lines < len(lines):
                start_line += 1
            elif key == ord('\n'):
                return "generate_again"
            elif key == ord('b'):
                return "back"
    
    def draw_test_screen(self, stdscr):
        """מסך בדיקת מערכת"""
        stdscr.clear()
        height, width = stdscr.getmaxyx()
        
        title = "🧪 בדיקת מערכת"
        stdscr.addstr(0, (width - len(title)) // 2, title, curses.A_BOLD)
        
        stdscr.addstr(2, 0, "=" * width)
        
        # בדיקות
        tests = [
            ("בדיקת Ollama", self.check_ollama()),
            ("מודלים זמינים", len(self.models) > 0),
            ("יצירת טקסט", False)  # יבדק בהמשך
        ]
        
        for i, (test_name, test_result) in enumerate(tests):
            status = "✅" if test_result else "❌"
            stdscr.addstr(4 + i, 4, f"{status} {test_name}")
        
        # בדיקת יצירת טקסט
        if len(self.models) > 0:
            stdscr.addstr(7, 4, "בודק יצירת טקסט...")
            stdscr.refresh()
            
            test_result = self.generate_text(self.models[0]["name"], "תגיד שלום")
            tests[2] = ("יצירת טקסט", "שגיאה" not in test_result)
            
            stdscr.addstr(7, 4, f"{'✅' if tests[2][1] else '❌'} יצירת טקסט")
        
        instructions = "Enter: חזרה לתפריט"
        stdscr.addstr(height - 2, (width - len(instructions)) // 2, instructions)
        
        stdscr.refresh()
        stdscr.getch()
    
    def run(self, stdscr):
        """הרץ את ה-TUI"""
        curses.curs_set(0)  # הסר סמן
        stdscr.timeout(100)  # זמן המתנה לקלט
        
        self.check_ollama()
        
        while True:
            if self.current_screen == "main":
                self.draw_main_menu(stdscr)
                
                key = stdscr.getch()
                if key == curses.KEY_UP:
                    self.selected_index = (self.selected_index - 1) % 5
                elif key == curses.KEY_DOWN:
                    self.selected_index = (self.selected_index + 1) % 5
                elif key == ord('\n'):  # Enter
                    if self.selected_index == 0:  # צור טקסט
                        self.current_screen = "generate"
                        self.selected_index = 0
                    elif self.selected_index == 1:  # רשימת מודלים
                        self.current_screen = "models"
                        self.selected_index = 0
                    elif self.selected_index == 2:  # בדיקת מערכת
                        self.draw_test_screen(stdscr)
                        self.check_ollama()
                    elif self.selected_index == 3:  # הגדרות
                        # TODO: הוסף הגדרות
                        pass
                    elif self.selected_index == 4:  # יציאה
                        break
                elif key == ord('q'):
                    break
            
            elif self.current_screen == "models":
                self.draw_models_screen(stdscr)
                
                key = stdscr.getch()
                if key == curses.KEY_UP:
                    self.selected_index = max(0, self.selected_index - 1)
                elif key == curses.KEY_DOWN:
                    self.selected_index = min(len(self.models) - 1, self.selected_index + 1)
                elif key == ord('b'):
                    self.current_screen = "main"
                    self.selected_index = 1
                elif key == ord('\n') and self.models:
                    # TODO: הוסף פעולה לבחירת מודל
                    pass
            
            elif self.current_screen == "generate":
                self.draw_generate_screen(stdscr)
                
                key = stdscr.getch()
                if key == curses.KEY_UP:
                    self.selected_index = max(0, self.selected_index - 1)
                elif key == curses.KEY_DOWN:
                    self.selected_index = min(min(4, len(self.models) - 1), self.selected_index + 1)
                elif key == ord('b'):
                    self.current_screen = "main"
                    self.selected_index = 0
                elif key == ord('\n') and self.models:
                    selected_model = self.models[self.selected_index]["name"]
                    prompt = self.draw_input_screen(stdscr, selected_model)
                    
                    if prompt:
                        stdscr.clear()
                        stdscr.addstr(10, 10, "⚡ מעבד...")
                        stdscr.refresh()
                        
                        result = self.generate_text(selected_model, prompt)
                        action = self.draw_result_screen(stdscr, selected_model, prompt, result)
                        
                        if action == "generate_again":
                            continue
                        elif action == "back":
                            self.current_screen = "generate"
            
            # רענן סטטוס כל 10 שניות
            if int(time.time()) % 10 == 0:
                self.check_ollama()

def main():
    """פונקציה ראשית"""
    try:
        tui = CalclawTUI()
        curses.wrapper(tui.run)
        print("\n👋 Calclaw TUI נסגר. להתראות!")
    except KeyboardInterrupt:
        print("\n👋 Calclaw TUI נסגר. להתראות!")
    except Exception as e:
        print(f"❌ שגיאה: {e}")
        print("💡 ודא ש-Ollama רץ: ollama serve &")

if __name__ == "__main__":
    main()