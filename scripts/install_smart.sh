#!/bin/bash

# 🦾 Calclaw - התקנה חכמה עם שאלון התאמה
# בוחר את סוג ההתקנה המתאים לפי צרכי המשתמש

set -e

echo "🦾 Calclaw - התקנה חכמה"
echo "========================"
echo ""
echo "ברוך הבא! נעזור לך להתקין את Calclaw בצורה המתאימה ביותר לצרכים שלך."
echo ""

# שאלון התאמה
echo "🔍 שאלון התאמה - בחר את האפשרויות המתאימות לך:"
echo ""

# שאלה 1: סוג השימוש
echo "1. 📋 מי המשתמשים?"
echo "   a) רק אני (שימוש אישי)"
echo "   b) צוות קטן (עד 10 אנשים)"
echo "   c) ארגון (מעל 10 אנשים)"
echo "   d) לקוחות חיצוניים"
read -p "   בחר אפשרות (a-d): " USER_TYPE

# שאלה 2: סביבת הרצה
echo ""
echo "2. 🖥️ איפה תריץ את המערכת?"
echo "   a) על המחשב שלי (מקומי)"
echo "   b) על שרת בארגון"
echo "   c) על ענן (AWS/Azure/GCP)"
echo "   d) על Kubernetes cluster"
read -p "   בחר אפשרות (a-d): " ENVIRONMENT

# שאלה 3: רמת אבטחה
echo ""
echo "3. 🔒 אילו בקרות אבטחה צריך?"
echo "   a) בסיסיות (גישה מלאה)"
echo "   b) בינוניות (אישורים לפעולות מסוכנות)"
echo "   c) מתקדמות (network policies, audit logs)"
echo "   d) ארגוניות (compliance, multi-factor auth)"
read -p "   בחר אפשרות (a-d): " SECURITY

# שאלה 4: אינטגרציות
echo ""
echo "4. 🔗 אילו אינטגרציות צריך?"
echo "   a) אין צורך באינטגרציות מיוחדות"
echo "   b) Slack/Teams"
echo "   c) GitHub/GitLab"
echo "   d) SaaS אחרים ומערכות פנימיות"
read -p "   בחר אפשרות (a-d): " INTEGRATIONS

# שאלה 5: תקציב/משאבים
echo ""
echo "5. 💰 מה התקציב/משאבים הזמינים?"
echo "   a) חינם/נמוך (שימוש במשאבים קיימים)"
echo "   b) בינוני (יכול לשלם עבור שירותים בסיסיים)"
echo "   c) גבוה (תקציב לתשתית ותמיכה)"
echo "   d) ארגוני (תקציב מלא לפיתוח והטמעה)"
read -p "   בחר אפשרות (a-d): " BUDGET

# ניתוח התשובות
echo ""
echo "📊 מנתח את התשובות שלך..."
echo ""

# הגדר משתנים לפי התשובות
case $USER_TYPE in
    a) USER_COUNT=1; USER_DESC="שימוש אישי" ;;
    b) USER_COUNT=10; USER_DESC="צוות קטן" ;;
    c) USER_COUNT=50; USER_DESC="ארגון" ;;
    d) USER_COUNT=100; USER_DESC="לקוחות חיצוניים" ;;
esac

case $ENVIRONMENT in
    a) ENV_DESC="מקומי (Local)" ;;
    b) ENV_DESC="שרת ארגוני" ;;
    c) ENV_DESC="ענן" ;;
    d) ENV_DESC="Kubernetes" ;;
esac

case $SECURITY in
    a) SEC_DESC="בסיסית" ;;
    b) SEC_DESC="בינונית" ;;
    c) SEC_DESC="מתקדמת" ;;
    d) SEC_DESC="ארגונית" ;;
esac

# הצג סיכום
echo "✅ סיכום הבחירות שלך:"
echo "   👥 משתמשים: $USER_DESC ($USER_COUNT משתמשים משוער)"
echo "   🖥️  סביבה: $ENV_DESC"
echo "   🔒 אבטחה: $SEC_DESC"
echo "   💰 תקציב: $(case $BUDGET in a) echo "חינם/נמוך";; b) echo "בינוני";; c) echo "גבוה";; d) echo "ארגוני";; esac)"
echo ""

# המלצה חכמה
echo "🎯 המלצת המערכת:"

if [[ "$USER_TYPE" == "a" && "$ENVIRONMENT" == "a" && "$SECURITY" == "a" ]]; then
    echo "   🏠 **התקנה פרטית/אישית** - מושלם עבורך!"
    RECOMMENDATION="personal"
elif [[ "$USER_COUNT" -gt 1 && "$SECURITY" != "a" ]]; then
    echo "   🏢 **התקנה ארגונית** - מתאים לצרכים שלך"
    RECOMMENDATION="enterprise"
    
    if [[ "$ENVIRONMENT" == "d" || "$SECURITY" == "d" || "$BUDGET" == "d" ]]; then
        echo "   🏛️  **Claw Organ** - מומלץ לארגונים מתקדמים"
        RECOMMENDATION="claw-organ"
    fi
else
    echo "   🔄 **התקנה מותאמת אישית** - נבנה לפי הצרכים הספציפיים שלך"
    RECOMMENDATION="custom"
fi

echo ""
echo "📝 פרטי ההמלצה:"

case $RECOMMENDATION in
    "personal")
        echo "   • התקנה מקומית על המחשב שלך"
        echo "   • גישה מלאה ללא הגבלות"
        echo "   • התקנה מהירה ב-5 דקות"
        echo "   • חינמי - אין עלויות נוספות"
        echo "   • מושלם לניסוי, פיתוח, ושימוש אישי"
        ;;
    "enterprise")
        echo "   • התקנה על שרת/ענן"
        echo "   • בקרות אבטחה מתאימות"
        echo "   • תמיכה במספר משתמשים"
        echo "   • אינטגרציות עם מערכות קיימות"
        echo "   • דורש תשתית IT בסיסית"
        ;;
    "claw-organ")
        echo "   • ארכיטקטורה ארגונית מתקדמת"
        echo "   • סנדבוקס מרוחק על Kubernetes"
        echo "   • אישורים לכל פעולה מסוכנת"
        echo "   • הרשאות OAuth בלבד"
        echo "   • קונפיגורציה מרכזית אחת"
        ;;
    "custom")
        echo "   • התאמה אישית לפי הצרכים הספציפיים שלך"
        echo "   • שילוב של פיצ'רים מכל הגרסאות"
        echo "   • פיתוח מותאם אישית אם נדרש"
        echo "   • ליווי והדרכה מלאים"
        ;;
esac

echo ""
read -p "📌 האם להמשיך עם ההמלצה? (y/n): " CONFIRM

if [[ "$CONFIRM" != "y" && "$CONFIRM" != "Y" ]]; then
    echo ""
    echo "🔄 בחר אפשרות התקנה אחרת:"
    echo "   1. 🏠 התקנה פרטית/אישית"
    echo "   2. 🏢 התקנה ארגונית"
    echo "   3. 🏛️  Claw Organ (ארגוני מתקדם)"
    echo "   4. ❌ ביטול"
    echo ""
    read -p "בחר אפשרות (1-4): " MANUAL_CHOICE
    
    case $MANUAL_CHOICE in
        1) RECOMMENDATION="personal" ;;
        2) RECOMMENDATION="enterprise" ;;
        3) RECOMMENDATION="claw-organ" ;;
        4) 
            echo "❌ התקנה בוטלה"
            exit 0
            ;;
        *)
            echo "❌ בחירה לא תקינה"
            exit 1
            ;;
    esac
fi

# המשך בהתקנה לפי ההמלצה
echo ""
echo "🚀 ממשיך עם התקנת $RECOMMENDATION..."
echo ""

case $RECOMMENDATION in
    "personal")
        # התקנה פרטית
        if [ ! -f "install_personal.sh" ]; then
            echo "📥 מוריד סקריפט התקנה פרטית..."
            curl -s -o install_personal.sh https://raw.githubusercontent.com/calclaw/calclaw/main/scripts/install_personal.sh
            chmod +x install_personal.sh
        fi
        
        ./install_personal.sh
        ;;
        
    "enterprise")
        # התקנה ארגונית
        if [ ! -f "install_enterprise.sh" ]; then
            echo "📥 מוריד סקריפט התקנה ארגונית..."
            curl -s -o install_enterprise.sh https://raw.githubusercontent.com/calclaw/calclaw/main/scripts/install_enterprise.sh
            chmod +x install_enterprise.sh
        fi
        
        # העבר פרמטרים לסקריפט הארגוני
        ./install_enterprise.sh \
            --users=$USER_COUNT \
            --environment=$ENVIRONMENT \
            --security=$SECURITY \
            --integrations=$INTEGRATIONS \
            --budget=$BUDGET
        ;;
        
    "claw-organ")
        # התקנת Claw Organ
        echo "🏛️  מתקין Claw Organ - ארכיטקטורה ארגונית מתקדמת"
        
        # בדוק אם קיים כבר
        if [ -d "../claw_organ" ]; then
            echo "📁 Claw Organ כבר קיים. מעדכן..."
            cd ../claw_organ
            git pull
        else
            echo "📥 מוריד Claw Organ..."
            git clone https://github.com/calclaw/claw-organ.git ../claw_organ
            cd ../claw_organ
        fi
        
        # הרץ את שאלון ההתאמה של Claw Organ
        ./scripts/deploy.sh --questionnaire
        ;;
        
    "custom")
        # התקנה מותאמת אישית
        echo "🔧 מכין התקנה מותאמת אישית..."
        
        # צור קובץ קונפיגורציה מותאם
        CONFIG_FILE="custom_install_config.json"
        cat > $CONFIG_FILE << EOF
{
  "install_type": "custom",
  "user_type": "$USER_TYPE",
  "user_count": $USER_COUNT,
  "environment": "$ENVIRONMENT",
  "security_level": "$SECURITY",
  "integrations": "$INTEGRATIONS",
  "budget": "$BUDGET",
  "timestamp": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
  "recommendations": [
    "התקנה מותאמת אישית לפי הצרכים הספציפיים",
    "שילוב פיצ'רים מכל הגרסאות",
    "ליווי והדרכה מלאים"
  ]
}
EOF
        
        echo "✅ קובץ קונפיגורציה נוצר: $CONFIG_FILE"
        echo ""
        echo "📞 צור קשר עם צוות Calclaw להתאמה אישית:"
        echo "   📧 Email: enterprise@calclaw.com"
        echo "   💬 Slack: join.calclaw.com"
        echo "   📞 Phone: 1-800-CALCLAW"
        echo ""
        echo "📋 נשלח אליך הצעת מחיר והצעת התאמה בתוך 24 שעות."
        ;;
esac

echo ""
echo "🎉 תהליך ההתקנה הושלם!"
echo ""
echo "📚 משאבים נוספים:"
echo "   📖 תיעוד: https://docs.calclaw.com"
echo "   💬 קהילה: https://community.calclaw.com"
echo "   🐙 קוד: https://github.com/calclaw"
echo "   🆘 תמיכה: support@calclaw.com"
echo ""
echo "🚀 Calclaw מוכן לשימוש! בהצלחה! 🦾"