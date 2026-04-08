#!/bin/bash

# 🏠 Calclaw - התקנה פרטית/אישית
# גרסה פשוטה ומהירה לשימוש על המחשב המקומי

set -e

echo "🏠 Calclaw - התקנה פרטית/אישית"
echo "=============================="
echo ""
echo "התקנה מהירה לשימוש אישי על המחשב שלך."
echo "מושלם לניסוי, פיתוח, ושימוש יומיומי."
echo ""

# בדוק מערכת הפעלה
echo "🔍 בודק מערכת הפעלה..."
OS=$(uname -s)
if [[ "$OS" != "Linux" && "$OS" != "Darwin" ]]; then
    echo "❌ מערכת הפעלה לא נתמכת: $OS"
    echo "   Calclaw תומך ב-Linux ו-macOS בלבד"
    exit 1
fi
echo "✅ מערכת הפעלה: $OS"

# בדוק אם יש הרשאות sudo
echo "🔐 בודק הרשאות..."
if [[ "$OS" == "Linux" && "$EUID" -eq 0 ]]; then
    echo "⚠️  רצה כ-root. מומלץ לרוץ כמשתמש רגיל."
    read -p "להמשיך? (y/n): " CONTINUE
    if [[ "$CONTINUE" != "y" && "$CONTINUE" != "Y" ]]; then
        exit 1
    fi
fi

# שאל את המשתמש אם רוצה התקנה מינימלית או מלאה
echo ""
echo "📦 סוג התקנה:"
echo "   1. התקנה מינימלית (רק Calclaw בסיסי)"
echo "   2. התקנה מלאה (כולל Ollama, Timeless Squads, וכל הפיצ'רים)"
echo "   3. התקנה מותאמת (בחר מה להתקין)"
echo ""
read -p "בחר אפשרות (1-3): " INSTALL_TYPE

# הגדר משתנים לפי סוג ההתקנה
case $INSTALL_TYPE in
    1)
        INSTALL_OLLAMA=false
        INSTALL_TIMELESS=false
        INSTALL_EXTRAS=false
        echo "🎯 התקנה מינימלית - רק Calclaw בסיסי"
        ;;
    2)
        INSTALL_OLLAMA=true
        INSTALL_TIMELESS=true
        INSTALL_EXTRAS=true
        echo "🎯 התקנה מלאה - כולל הכל"
        ;;
    3)
        echo ""
        echo "🔧 בחר מה להתקין:"
        read -p "התקן Ollama (מודלי AI מקומיים)? (y/n): " INSTALL_OLLAMA_INPUT
        read -p "התקן Timeless Squads (צוותי AI)? (y/n): " INSTALL_TIMELESS_INPUT
        read -p "התקן פיצ'רים נוספים (Telegram bot, TTS, cron)? (y/n): " INSTALL_EXTRAS_INPUT
        
        INSTALL_OLLAMA=$([[ "$INSTALL_OLLAMA_INPUT" == "y" || "$INSTALL_OLLAMA_INPUT" == "Y" ]] && echo true || echo false)
        INSTALL_TIMELESS=$([[ "$INSTALL_TIMELESS_INPUT" == "y" || "$INSTALL_TIMELESS_INPUT" == "Y" ]] && echo true || echo false)
        INSTALL_EXTRAS=$([[ "$INSTALL_EXTRAS_INPUT" == "y" || "$INSTALL_EXTRAS_INPUT" == "Y" ]] && echo true || echo false)
        echo "🎯 התקנה מותאמת אישית"
        ;;
    *)
        echo "❌ בחירה לא תקינה"
        exit 1
        ;;
esac

echo ""
echo "⚙️  התחלת התקנה..."
echo ""

# שלב 1: התקן Ollama אם נבחר
if [[ "$INSTALL_OLLAMA" == true ]]; then
    echo "1. 🤖 מתקין Ollama (מודלי AI מקומיים)..."
    
    if command -v ollama &> /dev/null; then
        echo "   ✅ Ollama כבר מותקן"
    else
        echo "   📥 מוריד ומתקין Ollama..."
        
        if [[ "$OS" == "Linux" ]]; then
            curl -fsSL https://ollama.com/install.sh | sh
        elif [[ "$OS" == "Darwin" ]]; then
            # macOS
            if command -v brew &> /dev/null; then
                brew install ollama
            else
                echo "   ❌ Homebrew לא מותקן. התקן עם:"
                echo "      /bin/bash -c \"\$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\""
                echo "   ואז הרץ: brew install ollama"
                exit 1
            fi
        fi
        
        echo "   ✅ Ollama הותקן"
    fi
    
    # הפעל את Ollama
    echo "   🚀 מפעיל Ollama..."
    if ! curl -s http://localhost:11434/api/tags > /dev/null 2>&1; then
        ollama serve > /tmp/ollama_install.log 2>&1 &
        OLLAMA_PID=$!
        echo "   ⏳ ממתין ל-Ollama להתחיל..."
        sleep 10
        
        # בדוק אם Ollama רץ
        if curl -s http://localhost:11434/api/tags > /dev/null 2>&1; then
            echo "   ✅ Ollama רץ (PID: $OLLAMA_PID)"
        else
            echo "   ⚠️  Ollama לא התחיל. בדוק: tail -f /tmp/ollama_install.log"
        fi
    else
        echo "   ✅ Ollama כבר רץ"
    fi
    
    # הורד מודלים מומלצים
    echo ""
    echo "   📥 מוריד מודלי AI מומלצים..."
    echo "   ℹ️  זה יכול לקחת כמה דקות בהתאם לחיבור האינטרנט"
    
    MODELS=("phi3:mini" "gemma2:9b" "phi3:3.8b")
    for model in "${MODELS[@]}"; do
        echo "   📦 מוריד $model..."
        ollama pull $model > /tmp/ollama_pull_${model//:/_}.log 2>&1 &
    done
    
    echo "   ⏳ הורדת מודלים מתבצעת ברקע..."
    echo "   📊 ניתן לעקוב: tail -f /tmp/ollama_pull_*.log"
else
    echo "1. ⏭️  מדלג על התקנת Ollama"
fi

# שלב 2: התקן Rust אם חסר
echo ""
echo "2. 🦀 בודק אם Rust מותקן..."
if command -v cargo &> /dev/null; then
    echo "   ✅ Rust כבר מותקן: $(rustc --version)"
else
    echo "   📥 מתקין Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
    echo "   ✅ Rust הותקן: $(rustc --version)"
fi

# שלב 3: בנה את Calclaw
echo ""
echo "3. 🔨 בונה Calclaw..."

# ודא שאנחנו בתיקיית Calclaw
if [[ ! -f "Cargo.toml" ]]; then
    echo "   ❌ לא נמצא קובץ Cargo.toml"
    echo "   📁 עבור לתיקיית Calclaw: cd /home/erez/.openclaw/workspace/calclaw"
    exit 1
fi

# גבה את הקוד המקורי אם צריך
if [[ ! -f "src/main_backup.rs" ]]; then
    echo "   💾 יוצר גיבוי של הקוד המקורי..."
    cp src/main.rs src/main_backup.rs 2>/dev/null || true
fi

# בחר גרסה מתאימה
if [[ "$INSTALL_TIMELESS" == true ]]; then
    echo "   🚀 בונה גרסה עם Timeless Squads..."
    # ודא שקובץ Timeless Squads קיים
    if [[ ! -f "src/timeless_squads.rs" ]]; then
        echo "   ⚠️  קובץ timeless_squads.rs לא נמצא. משתמש בגרסה בסיסית."
    fi
else
    echo "   🏠 בונה גרסה בסיסית..."
    # השתמש בגרסה בסיסית אם קיימת
    if [[ -f "src/main_basic.rs" ]]; then
        cp src/main_basic.rs src/main.rs
    fi
fi

# בנה את הפרויקט
echo "   🔧 מריץ cargo build..."
cargo build --release

if [[ $? -eq 0 ]]; then
    echo "   ✅ Calclaw נבנה בהצלחה!"
else
    echo "   ❌ שגיאה בבניית Calclaw"
    echo "   📝 בדוק את השגיאות למעלה"
    exit 1
fi

# שלב 4: צור סקריפטי ניהול
echo ""
echo "4. 📜 יוצר סקריפטי ניהול..."

# סקריפט הפעלה
cat > start_calclaw.sh << 'EOF'
#!/bin/bash

# 🦾 Calclaw - סקריפט הפעלה פרטי
# מפעיל את Calclaw על המחשב המקומי

set -e

echo "🚀 Calclaw - הפעלה פרטית"
echo "========================"

# בדוק אם Ollama רץ (אם מותקן)
if command -v ollama &> /dev/null; then
    if ! curl -s http://localhost:11434/api/tags > /dev/null 2>&1; then
        echo "🤖 מפעיל Ollama..."
        ollama serve > /tmp/ollama_calclaw.log 2>&1 &
        OLLAMA_PID=$!
        echo "   ✅ Ollama רץ (PID: $OLLAMA_PID)"
        sleep 5
    else
        echo "🤖 Ollama כבר רץ"
    fi
fi

# הפעל את Calclaw
echo "🦾 מפעיל Calclaw..."
cd "$(dirname "$0")"
./target/release/calclaw > /tmp/calclaw.log 2>&1 &
CALCLAW_PID=$!

echo "   ✅ Calclaw רץ (PID: $CALCLAW_PID)"
echo ""
echo "🎉 Calclaw מופעל בהצלחה!"
echo ""
echo "🌐 ממשקים זמינים:"
echo "   🏠 דף הבית: http://localhost:3000"
echo "   📖 תיעוד API: http://localhost:3000/docs"
if [[ -f "src/timeless_squads.rs" ]]; then
    echo "   🚀 Timeless Squads: http://localhost:3000/timeless"
fi
echo "   🤖 Ollama Dashboard: http://localhost:3000/ollama_dashboard.html"
echo ""
echo "📊 לוגים:"
echo "   📝 Calclaw: tail -f /tmp/calclaw.log"
echo "   🤖 Ollama: tail -f /tmp/ollama_calclaw.log"
echo ""
echo "🛑 לעצירה: pkill -f \"calclaw\" && pkill -f \"ollama serve\""
EOF

chmod +x start_calclaw.sh

# סקריפט עצירה
cat > stop_calclaw.sh << 'EOF'
#!/bin/bash

# 🛑 Calclaw - סקריפט עצירה

echo "🛑 עוצר Calclaw..."

# עצור את Calclaw
pkill -f "target/release/calclaw" || true
echo "✅ Calclaw נעצר"

# עצור את Ollama אם רץ מהסקריפט שלנו
pkill -f "ollama serve" || true
echo "✅ Ollama נעצר"

echo ""
echo "🎯 כל השירותים נעצרו"
EOF

chmod +x stop_calclaw.sh

# סקריפט עדכון
cat > update_calclaw.sh << 'EOF'
#!/bin/bash

# 🔄 Calclaw - סקריפט עדכון

echo "🔄 מעדכן Calclaw..."

# עצור את Calclaw אם רץ
pkill -f "target/release/calclaw" || true

# עדכן קוד אם זה git repository
if [[ -d ".git" ]]; then
    echo "📥 מושך עדכונים מ-git..."
    git pull
fi

# בנה מחדש
echo "🔨 בונה מחדש..."
cargo build --release

echo "✅ Calclaw עודכן!"
echo "🚀 להפעלה: ./start_calclaw.sh"
EOF

chmod +x update_calclaw.sh

echo "   ✅ סקריפטי ניהול נוצרו:"
echo "      🚀 start_calclaw.sh - הפעלת המערכת"
echo "      🛑 stop_calclaw.sh - עצירת המערכת"
echo "      🔄 update_calclaw.sh - עדכון המערכת"

# שלב 5: התקן פיצ'רים נוספים אם נבחר
if [[ "$INSTALL_EXTRAS" == true ]]; then
    echo ""
    echo "5. 🔧 מתקין פיצ'רים נוספים..."
    
    # בדוק אם Python מותקן
    if command -v python3 &> /dev/null; then
        echo "   🐍 Python מותקן: $(python3 --version)"
        
        # התקן תלויות Python
        if [[ -f "requirements.txt" ]]; then
            echo "   📦 מתקין תלויות Python..."
            pip3 install -r requirements.txt 2>/dev/null || true
        fi
        
        # הפוך סקריפטי Python לברי הרצה
        for script in *.py; do
            if [[ -f "$script" ]]; then
                chmod +x "$script" 2>/dev/null || true
            fi
        done
        
        echo "   ✅ פיצ'רי Python הותקנו"
    else
        echo "   ⚠️  Python 3 לא מותקן. מדלג על פיצ'רים נוספים."
    fi
else
    echo ""
    echo "5. ⏭️  מדלג על פיצ'רים נוספים"
fi

# שלב 6: צור תיעוד מקומי
echo ""
echo "6. 📝 יוצר תיעוד מקומי..."

cat > QUICK_START.md << 'EOF'
# 🦾 Calclaw - התחלה מהירה (גרסה פרטית)

## 🚀 הפעלה
```bash
./start_calclaw.sh
```

## 🌐 ממשקים זמינים
- **דף הבית**: http://localhost:3000
- **תיעוד API**: http://localhost:3000/docs
- **Timeless Squads**: http://localhost:3000/timeless
- **Ollama Dashboard**: http://localhost:3000/ollama_dashboard.html

## 🔧 ניהול
- **הפעלה**: `./start_calclaw.sh`
- **עצירה**: `./stop_calclaw.sh`
- **עדכון**: `./update_calclaw.sh`

## 🤖 מודלי AI
המודלים הבאים זמינים (אם Ollama מותקן):
- `phi3:mini` - קל ומהיר (2.2GB)
- `gemma2:9b` - חזק (5.4GB)
- `phi3:3.8b` - כללי (2.2GB)

להורדת מודל נוסף:
```bash
ollama pull שם_המודל
```

## 📊 לוגים
- Calclaw: `/tmp/calclaw.log`
- Ollama: `/tmp/ollama_calclaw.log`

צפה בלוגים:
```bash
tail -f /tmp/calclaw.log
```

## 🆘 פתרון בעיות

### Ollama לא רץ:
```bash
# הפעל ידנית
ollama serve &
```

### פורט 3000 תפוס:
```bash
# חפש תהליכים
sudo lsof -i :3000

# עצור תהליכים
pkill -f "calclaw"
```

### שגיאות קומפילציה:
```bash
# נקה ובנה מחדש
cargo clean
cargo build --release
```

## 📞 תמיכה
- 📖 תיעוד: https://docs.calclaw.com
- 💬 קהילה: https://community.calclaw.com
- 🐙 קוד: https://github.com/calclaw
- 🐛 באגים: https://github.com/calclaw/calclaw/issues

---

**Calclaw גרסה פרטית** 🏠
*מושלם לשימוש אישי, ניסוי, ופיתוח*
EOF

echo "   ✅ תיעוד נוצר: QUICK_START.md"

# סיכום
echo ""
echo "🎉 התקנה פרטית הושלמה בהצ