#!/bin/bash

# 🐳 התקנת Docker ב-WSL2
# מתקין Docker ו-Docker Compose ב-WSL2 (Ubuntu)

set -e

echo "🐳 התקנת Docker ב-WSL2"
echo "======================"
echo ""

# בדוק אם אנחנו ב-WSL2
if [[ ! -f /proc/version ]] || ! grep -q "Microsoft" /proc/version; then
    echo "⚠️  זה לא WSL2. סקריפט זה מיועד ל-WSL2 בלבד."
    echo "📖 התקן Docker לפי ההוראות הרשמיות:"
    echo "   https://docs.docker.com/get-docker/"
    exit 1
fi

echo "✅ מזוהה כ-WSL2"
echo ""

# בדוק אם Docker כבר מותקן
if command -v docker &> /dev/null; then
    echo "✅ Docker כבר מותקן: $(docker --version)"
    echo ""
    read -p "להמשיך בהתקנה בכל זאת? (y/n): " CONTINUE
    if [[ "$CONTINUE" != "y" && "$CONTINUE" != "Y" ]]; then
        exit 0
    fi
fi

echo "📦 מתחיל התקנת Docker..."
echo ""

# עדכן את המערכת
echo "1. 🔄 מעדכן את המערכת..."
sudo apt update

# התקן תלויות
echo "2. 📦 מתקין תלויות..."
sudo apt install -y \
    apt-transport-https \
    ca-certificates \
    curl \
    software-properties-common \
    gnupg \
    lsb-release

# הוסף את Docker GPG key
echo "3. 🔑 מוסיף Docker GPG key..."
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg

# הוסף את Docker repository
echo "4. 📁 מוסיף Docker repository..."
echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu \
  $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

# עדכן שוב
echo "5. 🔄 מעדכן רשימות packages..."
sudo apt update

# התקן Docker
echo "6. 🐳 מתקין Docker..."
sudo apt install -y docker-ce docker-ce-cli containerd.io

# התקן Docker Compose
echo "7. 🐳 מתקין Docker Compose..."
sudo curl -L "https://github.com/docker/compose/releases/latest/download/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
sudo chmod +x /usr/local/bin/docker-compose

# הוסף את המשתמש ל-docker group
echo "8. 👥 מוסיף את המשתמש ל-docker group..."
sudo usermod -aG docker $USER

# הגדר את Docker daemon להתחיל אוטומטית
echo "9. 🚀 מגדיר Docker daemon..."
sudo systemctl enable docker

# התחל את Docker daemon
echo "10. 🚀 מתחיל Docker daemon..."
sudo service docker start

# בדוק את ההתקנה
echo ""
echo "🧪 בודק את ההתקנה..."
sleep 2

if docker --version; then
    echo "✅ Docker הותקן בהצלחה: $(docker --version)"
else
    echo "❌ שגיאה בהתקנת Docker"
    exit 1
fi

if docker-compose --version; then
    echo "✅ Docker Compose הותקן בהצלחה: $(docker-compose --version)"
else
    echo "❌ שגיאה בהתקנת Docker Compose"
    exit 1
fi

# בדוק אם Docker רץ
echo ""
echo "🔍 בודק אם Docker רץ..."
if sudo docker info > /dev/null 2>&1; then
    echo "✅ Docker רץ"
else
    echo "⚠️  Docker לא רץ. מנסה להפעיל..."
    sudo service docker restart
    sleep 3
    if sudo docker info > /dev/null 2>&1; then
        echo "✅ Docker רץ לאחר הפעלה מחדש"
    else
        echo "❌ Docker עדיין לא רץ"
        echo "📖 פתרון בעיות:"
        echo "   1. הפעל מחדש את WSL2"
        echo "   2. הרץ: sudo service docker start"
        echo "   3. בדוק: sudo systemctl status docker"
    fi
fi

# הוראות להפעלה עם Windows Docker Desktop
echo ""
echo "📖 הוראות להפעלה עם Windows Docker Desktop:"
echo "=========================================="
echo ""
echo "אם אתה משתמש ב-Docker Desktop ב-Windows:"
echo ""
echo "1. 📥 הורד והתקן Docker Desktop מ:"
echo "   https://www.docker.com/products/docker-desktop/"
echo ""
echo "2. ⚙️  בהתקנה, סמן את האפשרויות:"
echo "   ☑️ Install required Windows components for WSL 2"
echo "   ☑️ Add shortcut to desktop"
echo ""
echo "3. 🔧 בהגדרות Docker Desktop:"
echo "   • Resources → WSL Integration → Enable integration with Ubuntu"
echo "   • Apply & Restart"
echo ""
echo "4. 🔄 הפעל מחדש את WSL2:"
echo "   wsl --shutdown"
echo "   wsl"
echo ""
echo "5. ✅ בדוק:"
echo "   docker --version"
echo "   docker-compose --version"
echo ""

# שאל אם ליצור קובץ בדיקה
echo ""
read -p "🧪 ליצור קובץ בדיקה ל-Docker? (y/n): " CREATE_TEST

if [[ "$CREATE_TEST" == "y" || "$CREATE_TEST" == "Y" ]]; then
    echo ""
    echo "📝 יוצר קובץ בדיקה..."
    
    cat > docker_test.sh << 'EOF'
#!/bin/bash

echo "🧪 Docker Test Script"
echo "===================="

# בדוק Docker
echo "1. 🔍 בדוק Docker:"
docker --version
docker info --format '{{.ServerVersion}}'

# הרץ container פשוט
echo ""
echo "2. 🐳 הרץ test container:"
docker run --rm hello-world

# בדוק Docker Compose
echo ""
echo "3. 🐳 בדוק Docker Compose:"
docker-compose --version

# צור קובץ docker-compose לבדיקה
echo ""
echo "4. 📄 יוצר docker-compose test..."
cat > docker-compose.test.yml << 'TESTEOF'
version: '3.8'

services:
  test:
    image: alpine:latest
    command: echo "Docker Compose works!"
TESTEOF

docker-compose -f docker-compose.test.yml up

echo ""
echo "✅ בדיקות הושלמו!"
echo ""
echo "🎉 Docker מותקן ופועל ב-WSL2!"
EOF
    
    chmod +x docker_test.sh
    echo "✅ קובץ בדיקה נוצר: docker_test.sh"
    echo "🚀 הרץ עם: ./docker_test.sh"
fi

echo ""
echo "🎉 התקנת Docker ב-WSL2 הושלמה!"
echo ""
echo "📚 משאבים נוספים:"
echo "   📖 תיעוד: https://docs.docker.com/desktop/wsl/"
echo "   💬 קהילה: https://discord.gg/docker"
echo "   🐛 באגים: https://github.com/docker/for-win/issues"
echo ""
echo "🚀 כעת אתה יכול לבנות ולהריץ Docker images ב-WSL2! 🐳"