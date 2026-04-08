#!/usr/bin/env python3
"""
Calclaw Cron Manager - API לניהול cron jobs
"""

import json
import os
import subprocess
import tempfile
from datetime import datetime
from typing import List, Dict, Any
import shlex

class CronManager:
    """מנהל cron jobs ל-Calclaw"""
    
    def __init__(self, user: str = "erez"):
        self.user = user
        self.cron_file = f"/tmp/calclaw_cron_{user}.txt"
        self.jobs_file = f"/home/erez/.openclaw/workspace/calclaw/cron_jobs.json"
        
    def load_jobs(self) -> List[Dict]:
        """טען cron jobs מקובץ JSON"""
        try:
            if os.path.exists(self.jobs_file):
                with open(self.jobs_file, 'r', encoding='utf-8') as f:
                    return json.load(f)
        except:
            pass
        return []
    
    def save_jobs(self, jobs: List[Dict]):
        """שמור cron jobs לקובץ JSON"""
        os.makedirs(os.path.dirname(self.jobs_file), exist_ok=True)
        with open(self.jobs_file, 'w', encoding='utf-8') as f:
            json.dump(jobs, f, ensure_ascii=False, indent=2)
    
    def get_current_cron(self) -> List[str]:
        """קבל את ה-cron הנוכחי של המשתמש"""
        try:
            result = subprocess.run(
                ['crontab', '-l'],
                capture_output=True,
                text=True,
                check=False
            )
            if result.returncode == 0:
                return result.stdout.strip().split('\n')
        except:
            pass
        return []
    
    def add_job(self, name: str, schedule: str, command: str, description: str = "") -> Dict:
        """הוסף cron job חדש"""
        # טען jobs קיימים
        jobs = self.load_jobs()
        
        # צור job חדש
        job_id = len(jobs) + 1
        new_job = {
            "id": job_id,
            "name": name,
            "schedule": schedule,
            "command": command,
            "description": description,
            "created": datetime.now().isoformat(),
            "enabled": True,
            "last_run": None,
            "next_run": self.calculate_next_run(schedule)
        }
        
        jobs.append(new_job)
        self.save_jobs(jobs)
        
        # עדכן את ה-cron בפועל
        self.update_system_cron()
        
        return new_job
    
    def delete_job(self, job_id: int) -> bool:
        """מחק cron job"""
        jobs = self.load_jobs()
        new_jobs = [j for j in jobs if j["id"] != job_id]
        
        if len(new_jobs) < len(jobs):
            self.save_jobs(new_jobs)
            self.update_system_cron()
            return True
        return False
    
    def toggle_job(self, job_id: int) -> bool:
        """הפעל/כבה cron job"""
        jobs = self.load_jobs()
        for job in jobs:
            if job["id"] == job_id:
                job["enabled"] = not job.get("enabled", True)
                self.save_jobs(jobs)
                self.update_system_cron()
                return True
        return False
    
    def update_system_cron(self):
        """עדכן את ה-cron של המערכת"""
        jobs = self.load_jobs()
        
        # קבל cron קיים (ללא Calclaw jobs)
        current_cron = self.get_current_cron()
        filtered_cron = [line for line in current_cron if "# Calclaw Job" not in line]
        
        # צור cron חדש
        new_cron = filtered_cron.copy()
        new_cron.append("")
        new_cron.append("# ============================================")
        new_cron.append("# 🦾 Calclaw Cron Jobs")
        new_cron.append("# ============================================")
        new_cron.append("")
        
        # הוסף את ה-jobs הפעילים
        for job in jobs:
            if job.get("enabled", True):
                comment = f"# Calclaw Job: {job['name']} - {job['description']}"
                cron_line = f"{job['schedule']} {job['command']}"
                new_cron.append(comment)
                new_cron.append(cron_line)
                new_cron.append("")
        
        # שמור לקובץ זמני והתקן
        with tempfile.NamedTemporaryFile(mode='w', delete=False) as f:
            f.write('\n'.join(new_cron))
            temp_file = f.name
        
        try:
            subprocess.run(['crontab', temp_file], check=True)
            os.unlink(temp_file)
            return True
        except:
            return False
    
    def calculate_next_run(self, schedule: str) -> str:
        """חשב מתי ירוץ ה-job הבא (פישוט)"""
        # זו פונקציה פשוטה - בגרסה מלאה צריך parser אמיתי
        parts = schedule.split()
        if len(parts) >= 5:
            minute, hour, day, month, weekday = parts[:5]
            
            now = datetime.now()
            next_run = now.replace(
                minute=int(minute) if minute != "*" else now.minute,
                hour=int(hour) if hour != "*" else now.hour,
                day=int(day) if day != "*" else now.day,
                month=int(month) if month != "*" else now.month
            )
            
            return next_run.isoformat()
        return ""
    
    def run_job_now(self, job_id: int) -> Dict:
        """הרץ cron job עכשיו"""
        jobs = self.load_jobs()
        for job in jobs:
            if job["id"] == job_id:
                try:
                    # הרץ את הפקודה
                    result = subprocess.run(
                        job["command"],
                        shell=True,
                        capture_output=True,
                        text=True,
                        timeout=300  # 5 דקות מקסימום
                    )
                    
                    # עדכן סטטוס
                    job["last_run"] = datetime.now().isoformat()
                    job["last_output"] = result.stdout[:1000]  # שמור 1000 תווים ראשונים
                    job["last_error"] = result.stderr[:1000] if result.stderr else None
                    job["last_exit_code"] = result.returncode
                    
                    self.save_jobs(jobs)
                    
                    return {
                        "success": result.returncode == 0,
                        "output": result.stdout,
                        "error": result.stderr,
                        "exit_code": result.returncode
                    }
                except subprocess.TimeoutExpired:
                    return {"success": False, "error": "פסק זמן"}
                except Exception as e:
                    return {"success": False, "error": str(e)}
        
        return {"success": False, "error": "Job לא נמצא"}
    
    def get_job_logs(self, job_id: int, limit: int = 10) -> List[Dict]:
        """קבל לוגים של job מסוים"""
        # בגרסה מלאה - קרא מקובץ לוגים
        return []

# API פשוט לשימוש ב-CLI
def main():
    """ממשק שורת פקודה פשוט"""
    import argparse
    
    parser = argparse.ArgumentParser(description="Calclaw Cron Manager")
    subparsers = parser.add_subparsers(dest="command", help="פקודות")
    
    # פקודת list
    list_parser = subparsers.add_parser("list", help="הצג cron jobs")
    
    # פקודת add
    add_parser = subparsers.add_parser("add", help="הוסף cron job")
    add_parser.add_argument("--name", required=True, help="שם ה-job")
    add_parser.add_argument("--schedule", required=True, help="לוח זמנים (cron syntax)")
    add_parser.add_argument("--command", required=True, help="פקודה להרצה")
    add_parser.add_argument("--desc", help="תיאור")
    
    # פקודת delete
    delete_parser = subparsers.add_parser("delete", help="מחק cron job")
    delete_parser.add_argument("--id", type=int, required=True, help="ID של ה-job")
    
    # פקודת toggle
    toggle_parser = subparsers.add_parser("toggle", help="הפעל/כבה cron job")
    toggle_parser.add_argument("--id", type=int, required=True, help="ID של ה-job")
    
    # פקודת run
    run_parser = subparsers.add_parser("run", help="הרץ cron job עכשיו")
    run_parser.add_argument("--id", type=int, required=True, help="ID של ה-job")
    
    args = parser.parse_args()
    manager = CronManager()
    
    if args.command == "list":
        jobs = manager.load_jobs()
        if not jobs:
            print("❌ אין cron jobs")
        else:
            print(f"📋 {len(jobs)} cron jobs:")
            for job in jobs:
                status = "✅" if job.get("enabled", True) else "❌"
                print(f"{status} [{job['id']}] {job['name']}: {job['schedule']}")
                if job.get('description'):
                    print(f"   📝 {job['description']}")
                print()
    
    elif args.command == "add":
        job = manager.add_job(args.name, args.schedule, args.command, args.desc or "")
        print(f"✅ נוסף cron job: {job['name']} (ID: {job['id']})")
    
    elif args.command == "delete":
        if manager.delete_job(args.id):
            print(f"✅ נמחק cron job ID: {args.id}")
        else:
            print(f"❌ לא נמצא cron job עם ID: {args.id}")
    
    elif args.command == "toggle":
        if manager.toggle_job(args.id):
            print(f"✅ שונה סטטוס cron job ID: {args.id}")
        else:
            print(f"❌ לא נמצא cron job עם ID: {args.id}")
    
    elif args.command == "run":
        result = manager.run_job_now(args.id)
        if result.get("success"):
            print("✅ Job הורץ בהצלחה")
            if result.get("output"):
                print(f"📤 פלט: {result['output'][:200]}...")
        else:
            print(f"❌ שגיאה: {result.get('error', 'לא ידוע')}")
    
    else:
        parser.print_help()

if __name__ == "__main__":
    main()