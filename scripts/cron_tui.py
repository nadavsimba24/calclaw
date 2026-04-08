#!/usr/bin/env python3
"""
Calclaw Cron TUI - ממשק טקסטואלי לניהול cron jobs
"""

import curses
import json
import subprocess
import time
from datetime import datetime
from typing import List, Dict, Any

class CronTUI:
    """TUI לניהול cron jobs"""
    
    def __init__(self):
        self.current_screen = "main"
        self.selected_index = 0
        self.jobs = []
        self.filter = "all"  # all, active, inactive
        
    def load_jobs(self):
        """טען cron jobs"""
        try:
            # נסה לטעון מקובץ JSON
            with open("/home/erez/.openclaw/workspace/calclaw/cron_jobs.json", "r") as f:
                self.jobs = json.load(f)
        except:
            # אם אין קובץ, צור דוגמאות
            self.jobs = [
                {
                    "id": 1,
                    "name": "גיבוי יומי",
                    "schedule": "0 2 * * *",
                    "command": "/home/erez/backup.sh",
                    "description": "גיבוי של כל הקבצים החשובים",
                    "enabled": True,
                    "created": "2026-04-06T20:00:00"
                },
                {
                    "id": 2,
                    "name": "ניקוי לוגים",
                    "schedule": "0 0 * * 0",
                    "command": "find /var/log -name '*.log' -mtime +7 -delete",
                    "description": "ניקוי לוגים ישנים",
                    "enabled": True,
                    "created": "2026-04-06T20:00:00"
                },
                {
                    "id": 3,
                    "name": "בדיקת מערכת",
                    "schedule": "*/30 * * * *",
                    "command": "/home/erez/check_system.sh",
                    "description": "בדיקת סטטוס מערכת כל 30 דקות",
                    "enabled": False,
                    "created": "2026-04-06T20:00:00"
                }
            ]
    
    def save_jobs(self):
        """שמור cron jobs"""
        with open("/home/erez/.openclaw/workspace/calclaw/cron_jobs.json", "w") as f:
            json.dump(self.jobs, f, ensure_ascii=False, indent=2)
    
    def get_filtered_jobs(self) -> List[Dict]:
        """קבל jobs לפי פילטר"""
        if self.filter == "all":
            return self.jobs
        elif self.filter == "active":
            return [j for j in self.jobs if j.get("enabled", True)]
        else:  # inactive
            return [j for j in self.jobs if not j.get("enabled", True)]
    
    def draw_main_menu(self, stdscr):
        """צייר תפריט ראשי"""
        stdscr.clear()
        height, width = stdscr.getmaxyx()
        
        # כותרת
        title = "🕒 Calclaw Cron Manager - TUI"
        stdscr.addstr(0, (width - len(title)) // 2, title, curses.A_BOLD)
        
        # קו מפריד
        stdscr.addstr(2, 0, "=" * width)
        
        # סטטוס
        active_jobs = len([j for j in self.jobs if j.get("enabled", True)])
        total_jobs = len(self.jobs)
        status = f"📊 {active_jobs}/{total_jobs} jobs פעילים | פילטר: {self.filter}"
        stdscr.addstr(4, 2, status)
        
        # תפריט
        menu_items = [
            "📋 רשימת Cron Jobs",
            "➕ הוסף Cron Job חדש",
            "🔧 ניהול Jobs",
            "⚙️  הגדרות פילטר",
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
    
    def draw_jobs_list(self, stdscr):
        """צייר רשימת jobs"""
        stdscr.clear()
        height, width = stdscr.getmaxyx()
        
        title = f"📋 Cron Jobs ({self.filter})"
        stdscr.addstr(0, (width - len(title)) // 2, title, curses.A_BOLD)
        
        stdscr.addstr(2, 0, "=" * width)
        
        filtered_jobs = self.get_filtered_jobs()
        
        if not filtered_jobs:
            stdscr.addstr(4, 4, "📭 אין cron jobs")
            stdscr.addstr(5, 4, "הוסף עם '+' מהתפריט הראשי")
        else:
            # כותרות טבלה
            headers = ["ID", "שם", "לוח זמנים", "סטטוס"]
            col_widths = [5, 20, 15, 10]
            
            # הדפס כותרות
            col = 2
            for i, header in enumerate(headers):
                stdscr.addstr(4, col, header[:col_widths[i]], curses.A_UNDERLINE)
                col += col_widths[i] + 2
            
            # הדפס jobs
            start_idx = max(0, self.selected_index - (height - 10))
            end_idx = min(len(filtered_jobs), start_idx + (height - 10))
            
            for i in range(start_idx, end_idx):
                job = filtered_jobs[i]
                row = 6 + (i - start_idx)
                
                # הדגש שורה נבחרת
                attr = curses.A_REVERSE if i == self.selected_index else curses.A_NORMAL
                
                # ID
                stdscr.addstr(row, 2, f"{job['id']:3}", attr)
                
                # שם
                name = job['name'][:18] + "..." if len(job['name']) > 18 else job['name']
                stdscr.addstr(row, 8, name, attr)
                
                # לוח זמנים
                stdscr.addstr(row, 30, job['schedule'], attr)
                
                # סטטוס
                status = "✅" if job.get('enabled', True) else "❌"
                stdscr.addstr(row, 47, status, attr)
                
                # תיאור (אם יש מקום)
                if job.get('description') and width > 60:
                    desc = job['description'][:width-60]
                    stdscr.addstr(row, 55, desc, attr)
        
        # הוראות
        instructions = "↑↓: גלילה | Enter: פרטים | a: הפעל/כבה | d: מחק | b: חזרה"
        stdscr.addstr(height - 2, (width - len(instructions)) // 2, instructions)
        
        stdscr.refresh()
    
    def draw_job_details(self, stdscr, job: Dict):
        """צייר פרטי job"""
        stdscr.clear()
        height, width = stdscr.getmaxyx()
        
        title = f"📄 פרטי Cron Job: {job['name']}"
        stdscr.addstr(0, (width - len(title)) // 2, title, curses.A_BOLD)
        
        stdscr.addstr(2, 0, "=" * width)
        
        # פרטי ה-job
        row = 4
        stdscr.addstr(row, 4, f"🆔 ID: {job['id']}")
        row += 1
        
        stdscr.addstr(row, 4, f"📝 שם: {job['name']}")
        row += 1
        
        stdscr.addstr(row, 4, f"🕒 לוח זמנים: {job['schedule']}")
        row += 1
        
        stdscr.addstr(row, 4, f"💻 פקודה: {job['command']}")
        row += 2
        
        if job.get('description'):
            stdscr.addstr(row, 4, f"📋 תיאור: {job['description']}")
            row += 2
        
        stdscr.addstr(row, 4, f"📅 נוצר: {job['created']}")
        row += 1
        
        status = "✅ פעיל" if job.get('enabled', True) else "❌ לא פעיל"
        stdscr.addstr(row, 4, f"🔧 סטטוס: {status}")
        row += 2
        
        # פעולות
        stdscr.addstr(row, 4, "🔧 פעולות:")
        row += 1
        
        actions = [
            "🚀 הרץ עכשיו",
            "🔁 הפעל/כבה",
            "🗑️ מחק",
            "📋 חזרה לרשימה"
        ]
        
        for i, action in enumerate(actions):
            if i == self.selected_index:
                stdscr.addstr(row + i, 6, f"> {action}", curses.A_REVERSE)
            else:
                stdscr.addstr(row + i, 6, f"  {action}")
        
        # הוראות
        instructions = "↑↓: בחירה | Enter: הרץ פעולה | b: חזרה"
        stdscr.addstr(height - 2, (width - len(instructions)) // 2, instructions)
        
        stdscr.refresh()
    
    def draw_add_job(self, stdscr):
        """צייר מסך הוספת job"""
        stdscr.clear()
        height, width = stdscr.getmaxyx()
        
        title = "➕ הוסף Cron Job חדש"
        stdscr.addstr(0, (width - len(title)) // 2, title, curses.A_BOLD)
        
        stdscr.addstr(2, 0, "=" * width)
        
        # שדות
        fields = [
            ("שם:", ""),
            ("לוח זמנים (cron):", ""),
            ("פקודה:", ""),
            ("תיאור (אופציונלי):", "")
        ]
        
        current_field = 0
        field_values = [""] * len(fields)
        
        while True:
            stdscr.clear()
            stdscr.addstr(0, (width - len(title)) // 2, title, curses.A_BOLD)
            stdscr.addstr(2, 0, "=" * width)
            
            # הדפס שדות
            row = 4
            for i, (label, value) in enumerate(fields):
                if i == current_field:
                    stdscr.addstr(row, 4, f"> {label}", curses.A_REVERSE)
                    stdscr.addstr(row, 20, field_values[i], curses.A_REVERSE)
                else:
                    stdscr.addstr(row, 4, f"  {label}")
                    stdscr.addstr(row, 20, field_values[i])
                row += 2
            
            # דוגמאות ל-cron
            row += 1
            stdscr.addstr(row, 4, "💡 דוגמאות ל-cron:")
            row += 1
            stdscr.addstr(row, 6, "כל שעה: 0 * * * *")
            row += 1
            stdscr.addstr(row, 6, "כל יום ב-2 בלילה: 0 2 * * *")
            row += 1
            stdscr.addstr(row, 6, "כל 5 דקות: */5 * * * *")
            
            # הוראות
            instructions = "Tab: שדה הבא | Shift+Tab: שדה קודם | Enter: שמור | Esc: ביטול"
            stdscr.addstr(height - 2, (width - len(instructions)) // 2, instructions)
            
            stdscr.refresh()
            
            # קבל קלט
            key = stdscr.getch()
            
            if key == ord('\t'):  # Tab
                current_field = (current_field + 1) % len(fields)
            elif key == 353:  # Shift+Tab
                current_field = (current_field - 1) % len(fields)
            elif key == ord('\n'):  # Enter - שמור
                # בדוק שדות חובה
                if not field_values[0] or not field_values[1] or not field_values[2]:
                    stdscr.addstr(height - 4, 4, "❌ מלא את כל השדות החובה!", curses.A_BOLD)
                    stdscr.refresh()
                    stdscr.getch()
                    continue
                
                # צור job חדש
                new_id = max([j['id'] for j in self.jobs], default=0) + 1
                new_job = {
                    "id": new_id,
                    "name": field_values[0],
                    "schedule": field_values[1],
                    "command": field_values[2],
                    "description": field_values[3],
                    "enabled": True,
                    "created": datetime.now().isoformat()
                }
                
                self.jobs.append(new_job)
                self.save_jobs()
                
                stdscr.addstr(height - 4, 4, f"✅ נוסף cron job: {field_values[0]}", curses.A_BOLD)
                stdscr.refresh()
                stdscr.getch()
                break
                
            elif key == 27:  # Esc
                break
            elif key == curses.KEY_BACKSPACE or key == 127:
                # מחק תו
                if field_values[current_field]:
                    field_values[current_field] = field_values[current_field][:-1]
            elif 32 <= key <= 126:  # תווים רגילים
                field_values[current_field] += chr(key)
    
    def draw_filter_menu(self, stdscr):
        """צייר תפריט פילטר"""
        stdscr.clear()
        height, width = stdscr.getmaxyx()
        
        title = "⚙️  הגדרות פילטר"
        stdscr.addstr(0, (width - len(title)) // 2, title, curses.A_BOLD)
        
        stdscr.addstr(2, 0, "=" * width)
        
        # אפשרויות פילטר
        filters = [
            ("הכל", "all"),
            ("פעילים בלבד", "active"),
            ("לא פעילים", "inactive")
        ]
        
        stdscr.addstr(4, 4, "בחר פילטר להצגה:")
        
        for i, (label, value) in enumerate(filters):
            if value == self.filter:
                stdscr.addstr(6 + i, 6, f"✅ {label}", curses.A_BOLD)
            else:
                stdscr.addstr(6 + i, 6, f"  {label}")
            
            if i == self.selected_index:
                stdscr.addstr(6 + i, 4, ">", curses.A_REVERSE)
        
        # הוראות
        instructions = "↑↓: בחירה | Enter: החל פילטר | b: חזרה"
        stdscr.addstr(height - 2, (width - len(instructions)) // 2, instructions)
        
        stdscr.refresh()
    
    def run_job_now(self, job: Dict):
        """הרץ job עכשיו"""
        try:
            result = subprocess.run(
                job['command'],
                shell=True,
                capture_output=True,
                text=True,
                timeout=60
            )
            return {
                "success": result.returncode == 0,
                "output": result.stdout[:500],
                "error": result.stderr[:500] if result.stderr else None
            }
        except subprocess.TimeoutExpired:
            return {"success": False, "error": "פסק זמן"}
        except Exception as e:
            return {"success": False, "error": str(e)}
    
    def run(self, stdscr):
        """הרץ את ה-TUI"""
        curses.curs_set(0)
        stdscr.timeout(100)
        
        self.load_jobs()
        
        while True:
            if self.current_screen == "main":
                self.draw_main_menu(stdscr)
                
                key = stdscr.getch()
                if key == curses.KEY_UP:
                    self.selected_index = (self.selected_index - 1) % 5
                elif key == curses.KEY_DOWN:
                    self.selected_index = (self.selected_index + 1) % 5
                elif key == ord('\n'):
                    if self.selected_index == 0:  # רשימת jobs
                        self.current_screen = "jobs_list"
                        self.selected_index = 0
                    elif self.selected_index == 1:  # הוסף job
                        self.current_screen = "add_job"
                        self.draw_add_job(stdscr)
                        self.current_screen = "main"
                    elif self.selected_index == 2:  # ניהול jobs
                        self.current_screen = "jobs_list"
                        self.selected_index = 0
                    elif self.selected_index == 3:  # הגדרות פילטר
                        self.current_screen = "filter"
                        self.selected_index = 0
                    elif self.selected_index == 4:  # יציאה
                        break
                elif key == ord('q'):
                    break
            
            elif self.current_screen == "jobs_list":
                self.draw_jobs_list(stdscr)
                
                key = stdscr.getch()
                filtered_jobs = self.get_filtered_jobs()
                
                if not filtered_jobs:
                if not filtered_jobs:
                    if key == ord('b'):
                        self.current_screen = "main"
                        self.selected_index = 0
                    continue
                
                if key == curses.KEY_UP:
                    self.selected_index = max(0, self.selected_index - 1)
                elif key == curses.KEY_DOWN:
                    self.selected_index = min(len(filtered_jobs) - 1, self.selected_index + 1)
                elif key == ord('b'):
                    self.current_screen = "main"
                    self.selected_index = 0
                elif key == ord('\n'):
                    # הצג פרטי job
                    selected_job = filtered_jobs[self.selected_index]
                    self.current_screen = "job_details"
                    self.selected_index = 0
                elif key == ord('a'):  # הפעל/כבה
                    selected_job = filtered_jobs[self.selected_index]
                    selected_job['enabled'] = not selected_job.get('enabled', True)
                    self.save_jobs()
                elif key == ord('d'):  # מחק
                    selected_job = filtered_jobs[self.selected_index]
                    # מצא את ה-job המקורי ב-list המלא
                    for i, job in enumerate(self.jobs):
                        if job['id'] == selected_job['id']:
                            del self.jobs[i]
                            self.save_jobs()
                            break
            
            elif self.current_screen == "job_details":
                filtered_jobs = self.get_filtered_jobs()
                if self.selected_index < len(filtered_jobs):
                    selected_job = filtered_jobs[self.selected_index]
                    self.draw_job_details(stdscr, selected_job)
                    
                    key = stdscr.getch()
                    if key == curses.KEY_UP:
                        self.selected_index = (self.selected_index - 1) % 4
                    elif key == curses.KEY_DOWN:
                        self.selected_index = (self.selected_index + 1) % 4
                    elif key == ord('\n'):
                        if self.selected_index == 0:  # הרץ עכשיו
                            result = self.run_job_now(selected_job)
                            # הצג תוצאה
                            stdscr.clear()
                            stdscr.addstr(0, 0, "🚀 תוצאות הרצה:", curses.A_BOLD)
                            if result['success']:
                                stdscr.addstr(2, 2, "✅ הצלחה!")
                                if result['output']:
                                    stdscr.addstr(4, 2, "📤 פלט:")
                                    stdscr.addstr(5, 4, result['output'][:200])
                            else:
                                stdscr.addstr(2, 2, "❌ שגיאה!")
                                stdscr.addstr(4, 2, f"📛 {result['error']}")
                            stdscr.addstr(8, 2, "לחץ על מקש כלשהו להמשך...")
                            stdscr.refresh()
                            stdscr.getch()
                        elif self.selected_index == 1:  # הפעל/כבה
                            selected_job['enabled'] = not selected_job.get('enabled', True)
                            self.save_jobs()
                        elif self.selected_index == 2:  # מחק
                            # מצא ומחק
                            for i, job in enumerate(self.jobs):
                                if job['id'] == selected_job['id']:
                                    del self.jobs[i]
                                    self.save_jobs()
                                    break
                            self.current_screen = "jobs_list"
                            self.selected_index = 0
                        elif self.selected_index == 3:  # חזרה
                            self.current_screen = "jobs_list"
                    elif key == ord('b'):
                        self.current_screen = "jobs_list"
            
            elif self.current_screen == "filter":
                self.draw_filter_menu(stdscr)
                
                key = stdscr.getch()
                if key == curses.KEY_UP:
                    self.selected_index = (self.selected_index - 1) % 3
                elif key == curses.KEY_DOWN:
                    self.selected_index = (self.selected_index + 1) % 3
                elif key == ord('\n'):
                    filters = ["all", "active", "inactive"]
                    self.filter = filters[self.selected_index]
                    self.current_screen = "main"
                    self.selected_index = 3
                elif key == ord('b'):
                    self.current_screen = "main"
                    self.selected_index = 3

def main():
    """פונקציה ראשית"""
    try:
        tui = CronTUI()
        curses.wrapper(tui.run)
        print("\n👋 Calclaw Cron TUI נסגר. להתראות!")
    except KeyboardInterrupt:
        print("\n👋 Calclaw Cron TUI נסגר. להתראות!")
    except Exception as e:
        print(f"❌ שגיאה: {e}")

if __name__ == "__main__":
    main()