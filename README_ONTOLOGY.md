# 🧠 Calclaw Organizational Ontology

## 🚀 **SuperAgent עם הבנה ארגונית מלאה**

Calclaw עכשיו יכול **להבין את הארגון שלך** ולבצע **אוטומציה חכמה** של תהליכים, ניתוח נתונים, וקבלת החלטות מבוססת נתונים.

## 📊 **מה נוסף?**

### 🏢 **מערכת אונטולוגיה מלאה:**
- **OrganizationProfile** - פרופיל ארגוני מלא
- **Department** - מחלקות והיררכיה
- **Process** - תהליכים עסקיים עם שלבים
- **DataEntity** - ישויות נתונים עם רגישות ובעלים
- **System** - מערכות IT ואינטגרציות
- **Goal** - מטרות עסקיות עם מדדים
- **Metric** - מדדי ביצוע ויזואליזציה

### 📋 **שאלון התאמה ארגוני:**
- 20 שאלות חכמות להבנת הארגון
- תשובות אוטומטיות ליצירת אונטולוגיה
- וולידציה וסיוע בהשלמה

### 🤖 **SuperAgent עם 5 יכולות:**
1. **Data Analysis** - ניתוח נתונים ארגוניים
2. **Process Automation** - אוטומציה של תהליכים
3. **Report Generation** - יצירת דוחות אוטומטית
4. **Decision Support** - תמיכה בקבלת החלטות
5. **Integration Orchestration** - תיאום בין מערכות

### 🧠 **Knowledge Base:**
- **Facts** - עובדות על הארגון
- **Rules** - כללים עסקיים
- **Patterns** - זיהוי תבניות
- **Decisions** - החלטות ולמידה
- **Learnings** - למידה מהניסיון

### 🚀 **REST API מלא:**
- 30+ endpoints לניהול האונטולוגיה
- Questionnaire API
- Agent tasks and capabilities
- UI visualizations
- Export/import

### 🖥️ **ממשק ויזואלי:**
- 8 סוגי תצוגות (overview, departments, processes, data map, etc.)
- אינטראקציה וניתוח שימוש
- המלצות חכמות
- דשבורד ביצועים

## 🛠️ **התקנה מהירה:**

```bash
# הורד את הקוד
git clone https://github.com/nadavsimba24/calclaw.git
cd calclaw

# התקן עם ontology
chmod +x scripts/install_with_ontology.sh
./scripts/install_with_ontology.sh
```

## 📈 **איך זה עובד?**

1. **התאמה ארגונית** - שאלון חכם מבין את הארגון שלך
2. **יצירת אונטולוגיה** - מיפוי אוטומטי של מחלקות, תהליכים, נתונים
3. **הפעלת SuperAgent** - סופר-אג'נט עם יכולות מותאמות
4. **אוטומציה חכמה** - ביצוע tasks אוטומטיים
5. **למידה ושיפור** - למידה מהניסיון ושיפור מתמיד

## 🎯 **דוגמאות לשימוש:**

### 1. **יצירת אונטולוגיה:**
```rust
let mut questionnaire = OnboardingQuestionnaire::new();
questionnaire.answer_question(question_id, answer);
let ontology = questionnaire.generate_ontology().unwrap();
```

### 2. **הפעלת SuperAgent:**
```rust
let mut super_agent = SuperAgent::new(ontology);
let task_id = super_agent.create_task(
    "Analyze Customer Data".to_string(),
    "Perform analysis on customer data".to_string(),
    "Data Analysis",
    Priority::High
);
let result = super_agent.execute_task(task_id).await;
```

### 3. **קבלת המלצות:**
```rust
let recommendations = super_agent.get_recommendations();
for rec in recommendations {
    println!("• {} (Priority: {:?})", rec.title, rec.priority);
}
```

### 4. **ויזואליזציה:**
```rust
let ui_manager = OntologyUIManager::new(ontology);
ui_manager.ui_state.current_view = ViewType::DepartmentView;
let visualization = ui_manager.generate_visualization();
```

## 🔗 **אינטגרציות:**

### עם **Calclaw הבסיסי:**
- שימוש ב-Ollama למודלים מקומיים
- ניהול cron jobs אוטומטי
- עיבוד טבעי בעברית
- ממשקי TUI ו-CLI

### עם **Timeless Squads:**
- צוותי AI ארגוניים
- שיתוף פעולה בין סוכנים
- ניהול פרויקטים חכם

### עם **Municipal Vision AI:**
- חיפוש תמונות עירוני
- ניתוח תמונות עם CLIP
- אינטגרציה עם Mapillary

## 📊 **מדדי ביצוע:**

- **Success Rate** - אחוז הצלחה של tasks
- **Learning Rate** - קצב למידה מהניסיון
- **Process Coverage** - כיסוי תהליכים במחלקות
- **Data Governance** - ניהול נתונים
- **System Integration** - אינטגרציה בין מערכות
- **Goal Alignment** - יישור מטרות

## 🚀 **התחלה מהירה:**

1. **התקן:** `./scripts/install_with_ontology.sh`
2. **הפעל:** `cargo run --bin calclaw_ontology`
3. **גש ל-UI:** `http://localhost:8080/ontology`
4. **השלם שאלון** - תן ל-Calclaw להבין את הארגון שלך
5. **הפעל tasks** - תן לסופר-אג'נט לעבוד בשבילך

## 📚 **קישורים:**

- [GitHub Repository](https://github.com/nadavsimba24/calclaw)
- [API Documentation](docs/api.md)
- [UI Guide](docs/ui.md)
- [Examples](examples/)

## 🎉 **יתרונות:**

✅ **הבנה עמוקה** של הארגון שלך  
✅ **אוטומציה חכמה** של תהליכים  
✅ **קבלת החלטות** מבוססת נתונים  
✅ **למידה מתמדת** מהניסיון  
✅ **ויזואליזציה מלאה** של המערכת  
✅ **אינטגרציה מלאה** עם Calclaw  
✅ **פרטיות מלאה** - הכל רץ מקומית  
✅ **קוד פתוח** - ניתן להתאמה אישית  

---

**Calclaw עם Organizational Ontology = 🧠 סופר-אג'נט שמבין את העסק שלך ומבצע את העבודה בשבילך!**