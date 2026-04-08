#!/bin/bash

# 🔍 בדיקת אפשרויות Docker ב-WSL2
# בודק אילו אפשרויות Docker זמינות

echo "🔍 בדיקת אפשרויות Docker ב-WSL2"
echo "================================"
echo ""

# בדוק מערכת
echo "📊 מידע מערכת:"
echo "   OS: $(uname -s) $(uname -r)"
echo "   WSL2: $(grep -q "Microsoft" /proc/version && echo "Yes" || echo "No")"
echo ""

# אפשרות 1: Docker Desktop ב-Windows
echo "1. 🪟 Docker Desktop ב-Windows:"
if [[ -d "/mnt/c/Program Files/Docker" ]] || [[ -d "/mnt/c/Program Files (x86)/Docker" ]]; then
    echo "   ✅ נמצא Docker Desktop ב-Windows"
    echo "   📁 נתיב: /mnt/c/Program Files/Docker/"
    
    # בדוק אם WSL2 integration מופעל
    if [[ -f "/mnt/c/Users/$USER/.docker/daemon.json" ]]; then
        echo "   ⚙️  קובץ daemon.json קיים"
    fi
    
    echo "   📖 הוראות:"
    echo "      • פתח Docker Desktop"
    echo "      • Settings → Resources → WSL Integration"
    echo "      • הפעל integration עם Ubuntu"
    echo "      • Apply & Restart"
    
else
    echo "   ❌ Docker Desktop לא מותקן ב-Windows"
    echo "   📥 הורד מ: https://www.docker.com/products/docker-desktop/"
fi

echo ""

# אפשרות 2: Docker ישירות ב-WSL2
echo "2. 🐧 Docker ישירות ב-WSL2:"
if command -v docker &> /dev/null; then
    echo "   ✅ Docker מותקן ב-WSL2: $(docker --version)"
    
    if docker info > /dev/null 2>&1; then
        echo "   ✅ Docker daemon רץ"
    else
        echo "   ❌ Docker daemon לא רץ"
        echo "   🚀 הפעל עם: sudo service docker start"
    fi
else
    echo "   ❌ Docker לא מותקן ב-WSL2"
    echo "   📥 התקן עם:"
    echo "      sudo apt update"
    echo "      sudo apt install docker.io docker-compose"
    echo "      sudo usermod -aG docker $USER"
fi

echo ""

# אפשרות 3: Podman (אלטרנטיבה ל-Docker)
echo "3. 🐙 Podman (אלטרנטיבה):"
if command -v podman &> /dev/null; then
    echo "   ✅ Podman מותקן: $(podman --version)"
    echo "   📖 Podman תואם Docker CLI"
    echo "   🚀 ניתן להשתמש ב: podman במקום docker"
else
    echo "   ❌ Podman לא מותקן"
    echo "   📥 התקן עם: sudo apt install podman"
fi

echo ""

# אפשרות 4: Buildah (לבניית images בלבד)
echo "4. 🔨 Buildah (בניית images):"
if command -v buildah &> /dev/null; then
    echo "   ✅ Buildah מותקן: $(buildah --version)"
    echo "   🚀 מתאים רק לבניית images, לא להרצה"
else
    echo "   ❌ Buildah לא מותקן"
fi

echo ""

# המלצה
echo "🎯 המלצה:"
echo "========="

if [[ -d "/mnt/c/Program Files/Docker" ]]; then
    echo "🏆 **השתמש ב-Docker Desktop ב-Windows**"
    echo "   • קל יותר לניהול"
    echo "   • ממשק משתמש גרפי"
    echo "   • אוטומטית עם WSL2"
    echo ""
    echo "🚀 שלבים:"
    echo "   1. פתח Docker Desktop"
    echo "   2. הפעל WSL2 integration"
    echo "   3. הפעל מחדש WSL2"
    echo "   4. הרץ: docker --version"
    
elif command -v docker &> /dev/null && docker info > /dev/null 2>&1; then
    echo "🏆 **השתמש ב-Docker ישירות ב-WSL2**"
    echo "   • מהיר יותר"
    echo "   • פחות תלוי ב-Windows"
    echo "   • יותר control"
    echo ""
    echo "🚀 הכל מוכן! הרץ:"
    echo "   ./build_docker.sh"
    
elif command -v podman &> /dev/null; then
    echo "🏆 **השתמש ב-Podman**"
    echo "   • תואם Docker"
    echo "   • לא דורש daemon"
    echo "   • יותר secure"
    echo ""
    echo "🚀 שנה את הסקריפטים להשתמש ב-podman:"
    echo "   sed -i 's/docker/podman/g' build_docker.sh"
    
else
    echo "🏆 **התקן Docker Desktop ב-Windows**"
    echo "   • הכי קל להתחלה"
    echo "   • הכי תואם"
    echo "   • הכי נתמך"
    echo ""
    echo "🚀 שלבים:"
    echo "   1. הורד Docker Desktop מ-docker.com"
    echo "   2. התקן עם WSL2 integration"
    echo "   3. הפעל מחדש WSL2"
    echo "   4. בדוק עם: docker --version"
fi

echo ""
echo "📚 משאבים:"
echo "   • Docker Desktop: https://www.docker.com/products/docker-desktop/"
echo "   • Docker in WSL2: https://docs.docker.com/desktop/wsl/"
echo "   • Podman: https://podman.io/"
echo "   • Buildah: https://buildah.io/"
echo ""
echo "🔧 לאחר ההתקנה, הרץ:"
echo "   ./build_docker.sh"
echo "   או"
echo "   ./run_with_docker.sh"