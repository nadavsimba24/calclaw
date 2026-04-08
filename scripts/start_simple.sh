#!/bin/bash
echo "🚀 מפעיל Calclaw (גרסה פשוטה)..."

# בדוק אם Ollama רץ
if ! curl -s http://localhost:11434/api/tags > /dev/null 2>&1; then
    echo "🤖 מפעיל Ollama..."
    ollama serve > ollama_simple.log 2>&1 &
    echo "✅ Ollama הופעל"
    sleep 3
fi

echo "🎉 המערכת מוכנה!"
echo ""
echo "📊 Ollama API: http://localhost:11434"
echo "🤖 מודלים זמינים:"
curl -s http://localhost:11434/api/tags | python3 -c "
import sys, json
try:
    data = json.load(sys.stdin)
    for model in data.get('models', []):
        size_gb = model.get('size', 0) / 1024 / 1024 / 1024
        print(f'  • {model.get(\"name\")} ({size_gb:.1f}GB)')
except:
    print('  לא ניתן לטעון מודלים')
"
echo ""
echo "🧪 נסה יצירת טקסט:"
echo 'curl -X POST http://localhost:11434/api/generate \'
echo '  -H "Content-Type: application/json" \'
echo '  -d '\''{"model": "phi3:mini", "prompt": "שלום", "stream": false}'\'''
echo ""
echo "🌐 דשבורד HTML:"
echo "xdg-open ollama_dashboard.html"
