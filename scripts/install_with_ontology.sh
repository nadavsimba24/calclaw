#!/bin/bash

# 🧠 Calclaw עם Ontology - התקנה חכמה עם הבנה ארגונית
# מוסיף יכולת הבנה של הארגון ונתוניו

set -e

echo "🧠 Calclaw עם Ontology - התקנה חכמה"
echo "==================================="
echo ""
echo "התקנה מתקדמת עם יכולת הבנה של הארגון שלך:"
echo "• שאלון התאמה ארגוני"
echo "• אונטולוגיה של נתונים ותהליכים"
echo "• סופר-אג'נט שיודע לתאם הכל"
echo "• ממשק ויזואלי לאונטולוגיה"
echo ""

# בדוק אם Calclaw כבר מותקן
if [[ -f "target/release/calclaw" ]]; then
    echo "✅ Calclaw כבר מותקן"
    echo ""
    read -p "לעדכן עם יכולות ontology? (y/n): " UPDATE_CALCLAW
    
    if [[ "$UPDATE_CALCLAW" != "y" && "$UPDATE_CALCLAW" != "Y" ]]; then
        echo "❌ התקנה בוטלה"
        exit 0
    fi
fi

# שאל את המשתמש על סוג ההתקנה
echo ""
echo "🔧 סוג התקנת Ontology:"
echo "   1. התקנה מלאה (כולל UI ו-API)"
echo "   2. התקנה בסיסית (רק backend)"
echo "   3. אינטגרציה עם Calclaw קיים"
echo ""

read -p "בחר אפשרות (1-3): " ONTOLOGY_TYPE

case $ONTOLOGY_TYPE in
    1)
        echo "🚀 מתקין גרסה מלאה עם UI..."
        INSTALL_UI=true
        INSTALL_API=true
        ;;
    2)
        echo "🛠️ מתקין גרסה בסיסית..."
        INSTALL_UI=false
        INSTALL_API=true
        ;;
    3)
        echo "🔗 מתחבר ל-Calclaw קיים..."
        INSTALL_UI=false
        INSTALL_API=true
        ;;
    *)
        echo "❌ בחירה לא תקינה"
        exit 1
        ;;
esac

# שלב 1: התקן תלויות
echo ""
echo "1. 📦 מתקין תלויות..."

# בדוק אם Rust מותקן
if ! command -v cargo &> /dev/null; then
    echo "   🦀 מתקין Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
else
    echo "   ✅ Rust כבר מותקן: $(rustc --version)"
fi

# התקן ת