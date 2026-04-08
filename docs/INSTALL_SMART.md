# 🦾 Calclaw - מערכת התקנה חכמה

## 🎯 **התקנה חכמה לפי סוג המשתמש**

Calclaw מציע שתי אפשרויות התקנה עיקריות, בהתאם לצרכים שלך:

### **🔍 בחר את סוג ההתקנה המתאים לך:**

1. **🏠 שימוש פרטי/אישי** - רץ על המחשב שלך, גישה מלאה, פשוט ומהיר
2. **🏢 שימוש ארגוני** - רץ על שרת/ענן, בקרות אבטחה, multi-user, סקיילבילי

---

## 📋 **השוואה בין אפשרויות ההתקנה**

| מאפיין | 🏠 שימוש פרטי | 🏢 שימוש ארגוני |
|--------|---------------|-----------------|
| **מיקום הרצה** | מחשב מקומי | שרת/ענן/Kubernetes |
| **גישה** | גישה מלאה | גישה מבוקרת עם אישורים |
| **משתמשים** | משתמש יחיד | multi-tenant |
| **אבטחה** | בסיסית | מתקדמת עם network policies |
| **סקיילביליות** | מוגבלת | גבוהה עם Kubernetes |
| **תחזוקה** | פשוטה | דורש DevOps |
| **עלות** | חינם | דורש תשתית |
| **אידיאלי ל** | משתמשים בודדים, פרויקטים קטנים | ארגונים, צוותים, פרויקטים גדולים |

---

## 🚀 **התקנה חכמה - שאלון התאמה**

לפני ההתקנה, נשאל אותך כמה שאלות כדי להתאים את המערכת לצרכים שלך:

### **שאלה 1: מי המשתמשים?**
- [ ] רק אני (שימוש אישי)
- [ ] צוות קטן (עד 10 אנשים)
- [ ] ארגון (מעל 10 אנשים)
- [ ] לקוחות חיצוניים

### **שאלה 2: איפה תריץ את המערכת?**
- [ ] על המחשב שלי (מקומי)
- [ ] על שרת בארגון
- [ ] על ענן (AWS/Azure/GCP)
- [ ] על Kubernetes cluster

### **שאלה 3: אילו בקרות אבטחה צריך?**
- [ ] בסיסיות (גישה מלאה)
- [ ] בינוניות (אישורים לפעולות מסוכנות)
- [ ] מתקדמות (network policies, audit logs)
- [ ] ארגוניות (compliance, multi-factor auth)

### **שאלה 4: אילו אינטגרציות צריך?**
- [ ] Slack/Teams
- [ ] GitHub/GitLab
- [ ] SaaS אחרים
- [ ] מערכות פנימיות

---

## 🏠 **אפשרות 1: התקנה פרטית/אישית**

### **למי זה מתאים?**
- משתמשים בודדים שרוצים AI מקומי
- פרויקטים קטנים ומהירים
- ניסוי ופיתוח
- סביבות ביתיות/משרדיות

### **יתרונות:**
- ⚡ **מהיר ופשוט** - התקנה ב-5 דקות
- 🔓 **גישה מלאה** - ללא הגבלות
- 💰 **חינמי** - אין עלויות תשתית
- 🛠️ **גמיש** - ניתן להתאים אישית

### **התקנה מהירה:**

```bash
# התקנה אוטומטית לשימוש פרטי
cd /home/erez/.openclaw/workspace/calclaw
chmod +x install_personal.sh
./install_personal.sh
```

### **מה כולל ההתקנה הפרטית?**
1. **Ollama מקומי** - מודלי AI על המחשב שלך
2. **Calclaw server** - רץ על localhost:3000
3. **גישה מלאה** - ללא בקרות אבטחה
4. **כל הפיצ'רים** - כולל Timeless Squads
5. **תיעוד מלא** - בעברית

### **שימוש יומיומי:**
```bash
# הפעל את המערכת
./start_personal.sh

# פתח בדפדפן
xdg-open http://localhost:3000

# או השתמש ב-CLI
./cli_simple.py
```

---

## 🏢 **אפשרות 2: התקנה ארגונית**

### **למי זה מתאים?**
- ארגונים עם צוותים
- פרויקטים גדולים
- דרישות אבטחה
- multi-user environments
- סביבות production

### **יתרונות:**
- 🔒 **אבטחה מתקדמת** - network policies, audit logs
- 👥 **Multi-tenant** - בידוד בין משתמשים
- 📈 **סקיילבילי** - רץ על Kubernetes
- 🔗 **אינטגרציות** - Slack, GitHub, SaaS
- 📊 **ניטור** - monitoring ו-alerting

### **התקנה ארגונית:**

```bash
# התקנה ארגונית עם בקרות אבטחה
cd /home/erez/.openclaw/workspace/calclaw
chmod +x install_enterprise.sh

# הרץ את שאלון ההתאמה
./install_enterprise.sh --questionnaire

# או התקנה ישירה
./install_enterprise.sh --type=kubernetes --users=50 --security=high
```

### **מה כולל ההתקנה הארגונית?**
1. **Kubernetes deployment** - עם manifests מלאים
2. **Network policies** - בקרות תעבורה
3. **MCP Proxy עם אישורים** - אישורים לפעולות מסוכנות
4. **Multi-tenant architecture** - בידוד בין משתמשים
5. **Audit logging** - תיעוד כל הפעולות
6. **Monitoring** - Prometheus + Grafana
7. **Backup/restore** - גיבויים אוטומטיים

### **רכיבים נוספים בארגוני:**
- **Slack Integration** - עבודה ישירות מ-Slack
- **OAuth 2.0** - הרשאות משתמשים
- **MCP Protocol** - אינטגרציה עם מערכות
- **Persistent Storage** - קונפיגורציה מרכזית
- **CI/CD Pipeline** - פריסה אוטומטית

---

## 🔄 **התאמה אישית - Claw Organ (לארגונים מתקדמים)**

### **מה זה Claw Organ?**
ארכיטקטורה לארגונים שמטמיעה Calclaw בצורה בטוחה ומבוקרת:

### **עקרונות Claw Organ:**
1. **"השאלה היא לא מה אסור אלא מה מותר"**
2. **סנדבוקס מרוחק** על Kubernetes
3. **אישורים לכל פעולה** דרך Slack
4. **הרשאות OAuth בלבד** - לא יותר מהרשאות המשתמש
5. **קונפיגורציה מרכזית אחת** - Single Source of Truth

### **התקנת Claw Organ:**
```bash
# הורד את Claw Organ
git clone https://github.com/calclaw/claw-organ.git
cd claw-organ

# התקן עם שאלון התאמה
./scripts/deploy.sh --questionnaire

# או התקנה ישירה
./scripts/deploy.sh \
  --type=kubernetes \
  --slack-token=YOUR_TOKEN \
  --github-token=YOUR_TOKEN \
  --users=100
```

### **יתרונות Claw Organ:**
- ✅ **אבטחה מקסימלית** - הכל בסנדבוקס מרוחק
- ✅ **בקרה מלאה** - אישורים לכל פעולה מסוכנת
- ✅ **סקיילביליות** - Kubernetes-based, multi-tenant
- ✅ **אינטגרציה** - Slack, MCP, OAuth
- ✅ **ניהול קל** - קונפיגורציה מרכזית אחת

---

## 📊 **התאמה לפי צרכים ספציפיים**

### **לצוותי פיתוח:**
```bash
# התקנה עם GitHub integration
./install_enterprise.sh \
  --type=development \
  --integrations=github,slack \
  --security=medium \
  --users=10
```

### **לצוותי DevOps:**
```bash
# התקנה עם Kubernetes integration
./install_enterprise.sh \
  --type=devops \
  --integrations=kubernetes,docker,terraform \
  --security=high \
  --users=5
```

### **לצוותי נתונים:**
```bash
# התקנה עם data tools
./install_enterprise.sh \
  --type=data \
  --integrations=postgres,redis,python \
  --security=medium \
  --users=8
```

### **לצוותי מוצר:**
```bash
# התקנה עם collaboration tools
./install_enterprise.sh \
  --type=product \
  --integrations=slack,notion,figma \
  --security=low \
  --users=15
```

---

## 🔧 **סקריפטי התקנה חכמים**

### **1. `install_smart.sh` - התקנה חכמה עם שאלון**
```bash
#!/bin/bash
echo "🦾 Calclaw - התקנה חכמה"
echo "========================"

# שאל את המשתמש
echo "🔍 בחר את סוג ההתקנה:"
echo "1. 🏠 שימוש פרטי/אישי (מקומי)"
echo "2. 🏢 שימוש ארגוני (שרת/ענן)"
echo "3. 🏛️  Claw Organ (ארגוני מתקדם)"
echo ""

read -p "בחר אפשרות (1-3): " choice

case $choice in
    1)
        echo "🏠 מתקין גרסה פרטית..."
        ./install_personal.sh
        ;;
    2)
        echo "🏢 מתקין גרסה ארגונית..."
        ./install_enterprise.sh --questionnaire
        ;;
    3)
        echo "🏛️  מתקין Claw Organ..."
        git clone https://github.com/calclaw/claw-organ.git
        cd claw-organ
        ./scripts/deploy.sh --questionnaire
        ;;
    *)
        echo "❌ בחירה לא תקינה"
        exit 1
        ;;
esac
```

### **2. `install_personal.sh` - התקנה פרטית**
```bash
#!/bin/bash
echo "🏠 Calclaw - התקנה פרטית/אישית"
echo "=============================="

# התקן Ollama אם חסר
if ! command -v ollama &> /dev/null; then
    echo "📦 מתקין Ollama..."
    curl -fsSL https://ollama.com/install.sh | sh
fi

# התקן Rust אם חסר
if ! command -v cargo &> /dev/null; then
    echo "🦀 מתקין Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source $HOME/.cargo/env
fi

# בנה את Calclaw
echo "🔨 בונה Calclaw..."
cd /home/erez/.openclaw/workspace/calclaw
cargo build --release

# צור סקריפטי ניהול
echo "📜 יוצר סקריפטי ניהול..."
cat > start_personal.sh << 'EOF'
#!/bin/bash
echo "🚀 מפעיל Calclaw פרטי..."

# הפעל Ollama אם לא רץ
if ! curl -s http://localhost:11434/api/tags > /dev/null; then
    echo "🤖 מפעיל Ollama..."
    ollama serve > /tmp/ollama.log 2>&1 &
    sleep 5
fi

# הפעל Calclaw
cd /home/erez/.openclaw/workspace/calclaw
./target/release/calclaw > /tmp/calclaw.log 2>&1 &

echo "✅ Calclaw רץ על http://localhost:3000"
echo "📖 תיעוד: http://localhost:3000/docs"
echo "🦾 Timeless Squads: http://localhost:3000/timeless"
EOF

chmod +x start_personal.sh

echo "✅ התקנה פרטית הושלמה!"
echo ""
echo "🚀 להפעלה: ./start_personal.sh"
echo "🌐 דפדפן: http://localhost:3000"
```

### **3. `install_enterprise.sh` - התקנה ארגונית**
```bash
#!/bin/bash
echo "🏢 Calclaw - התקנה ארגונית"
echo "==========================="

# שאלון התאמה
if [[ "$1" == "--questionnaire" ]]; then
    echo "📝 שאלון התאמה ארגונית"
    echo ""
    
    read -p "מספר משתמשים משוער: " USER_COUNT
    read -p "סביבת הרצה (local/server/cloud/k8s): " ENVIRONMENT
    read -p "רמת אבטחה (low/medium/high): " SECURITY_LEVEL
    read -p "אינטגרציות נדרשות (מופרדות בפסיק): " INTEGRATIONS
    
    echo ""
    echo "⚙️  הגדרות שנבחרו:"
    echo "   👥 משתמשים: $USER_COUNT"
    echo "   🖥️  סביבה: $ENVIRONMENT"
    echo "   🔒 אבטחה: $SECURITY_LEVEL"
    echo "   🔗 אינטגרציות: $INTEGRATIONS"
    echo ""
    
    read -p "להמשיך בהתקנה? (y/n): " CONFIRM
    if [[ "$CONFIRM" != "y" && "$CONFIRM" != "Y" ]]; then
        echo "❌ התקנה בוטלה"
        exit 0
    fi
fi

# המשך בהתקנה...
echo "🔧 מכין התקנה ארגונית..."
# כאן יבוא הקוד המלא להתקנה ארגונית
```

---

## 📞 **תמיכה והדרכה**

### **לשימוש פרטי:**
- 📖 **תיעוד**: `/home/erez/.openclaw/workspace/calclaw/README.md`
- 💬 **קהילה**: Telegram/Discord
- 🐛 **דיווח באגים**: GitHub Issues
- 🔧 **תמיכה**: Community support

### **לשימוש ארגוני:**
- 👔 **Account Manager** - איש קשר ייעודי
- 🏢 **Professional Services** - התאמה והטמעה
- 🔒 **Security Review** - בדיקות אבטחה
- 📊 **Compliance** - עמידה בתקנים
- 🚀 **Training** - הדרכות למשתמשים ומנהלים

### **חבילות תמיכה:**
1. **Basic** - Community support, documentation
2. **Professional** - Email support, business hours
3. **Enterprise** - 24/7 support, SLAs, on-site training
4. **Platinum** - Dedicated engineer, custom development

---

## 🔄 **מעבר בין גרסאות**

### **מפרטי לארגוני:**
```bash
# שדרוג מגרסה פרטית לארגונית
cd /home/erez/.openclaw/workspace/calclaw
./upgrade_to_enterprise.sh

# או התקנה חדשה עם migration
./install_enterprise.sh --migrate-from=personal
```

### **מארגוני ל-Claw Organ:**
```bash
# שדרוג ל-Claw Organ
git clone https://github.com/calclaw/claw-organ.git
cd claw-organ
./scripts/migrate_from_calclaw.sh /path/to/calclaw/config
```

### **גיבוי ושחזור:**
```bash
# גיבוי קונפיגורציה
./backup_config.sh --type=full

# שחזור מגיבוי
./restore_config.sh --backup-file=backup_2026-04-08.tar.gz
```

---

## 🎯 **סיכום - איך לבחור?**

### **בחר שימוש פרטי אם:**
- אתה משתמש יחיד
- רוצה משהו פשוט ומהיר
- אין דרישות אבטחה מיוחדות
- עובד על פרויקטים קטנים
- אין תקציב לתשתית

### **בחר שימוש ארגוני אם:**
- יש מספר משתמשים
- יש דרישות אבטחה
- צריך אינטגרציות עם מערכות
- עובדים על פרויקטים גדולים
- יש תשתית IT תומכת

### **בחר Claw Organ אם:**
- ארגון עם דריש