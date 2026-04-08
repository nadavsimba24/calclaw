#!/bin/bash

echo "🧪 בדיקת Calclaw Hebrew NLP"
echo "=========================="

# בדוק אם Python יכול לטעון את המודול
echo "1. בדיקת מודול NLP..."
if python3 -c "from hebrew_nlp import HebrewNLP; print('✅ מודול NLP נטען בהצלחה')" 2>/dev/null; then
    echo "   ✅ מודול NLP עובד"
else
    echo "   ❌ שגיאה בטעינת מודול NLP"
    exit 1
fi

# בדוק הבנה בסיסית
echo ""
echo "2. בדיקת הבנת עברית בסיסית..."
python3 -c "
from hebrew_nlp import HebrewNLP
nlp = HebrewNLP()

test_cases = [
    'תגבה לי את הקבצים',
    'תנקה את הלוגים',
    'בדוק את המערכת'
]

for test in test_cases:
    intent = nlp.extract_intent(test)
    print(f'📝 \"{test}\" → {intent.get(\"action\", \"לא ידוע\")} ({intent.get(\"confidence\", 0):.0%})')
"

# בדוק אם Ollama רץ
echo ""
echo "3. בדיקת Ollama..."
if curl -s http://localhost:11434/api/tags > /dev/null 2>&1; then
    echo "   ✅ Ollama רץ"
    echo "   🇮🇱 NLP מתקדם זמין"
else
    echo "   ⚠️  Ollama לא רץ"
    echo "   💡 NLP בסיסי בלבד"
fi

echo ""
echo "🎉 בדיקת עברית הושלמה!"
echo ""
echo "🚀 לנסות בעצמך:"
echo "  ./run_hebrew_chat.sh"
echo "  python3 hebrew_chat.py"