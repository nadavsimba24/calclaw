#!/bin/bash

# 🐳 Calclaw - Run with Docker
# הרצה קלה של Calclaw עם Docker

set -e

echo "🐳 Calclaw - הרצה עם Docker"
echo "==========================="
echo ""

# בדוק אם Docker מותקן ורץ
if ! command -v docker &> /dev/null; then
    echo "❌ Docker לא מותקן"
    echo ""
    echo "📖 התקן Docker:"
    echo "   macOS: https://docs.docker.com/desktop/install/mac-install/"
    echo "   Linux: https://docs.docker.com/engine/install/"
    echo "   Windows: https://docs.docker.com/desktop/install/windows-install/"
    exit 1
fi

if ! docker info > /dev/null 2>&1; then
    echo "❌ Docker לא רץ"
    echo "🚀 הפעל את Docker Desktop או Docker daemon"
    exit 1
fi

echo "✅ Docker מותקן ורץ: $(docker --version)"
echo ""

# שאל איזו גרסה להריץ
echo "🔧 בחר גרסה להרצה:"
echo "   1. 🏠 Personal (קל ומהיר, מומלץ להתחלה)"
echo "   2. 🏢 Enterprise (מתקדם עם Ollama)"
echo "   3. 🏛️  Claw Organ (ארגוני מלא)"
echo "   4. 🔄 Custom (התאמה אישית)"
echo ""

read -p "בחר אפשרות (1-4): " CHOICE

case $CHOICE in
    1)
        # Personal
        echo "🏠 מריץ Calclaw Personal..."
        IMAGE="calclaw/personal:latest"
        PORT=3000
        COMPOSE_FILE="docker-compose.yml"
        ;;
    2)
        # Enterprise
        echo "🏢 מריץ Calclaw Enterprise..."
        IMAGE="calclaw/enterprise:latest"
        PORT=3000
        COMPOSE_FILE="docker-compose.enterprise.yml"
        ;;
    3)
        # Claw Organ
        echo "🏛️  מריץ Claw Organ..."
        IMAGE="calclaw/claw-organ:latest"
        PORT=3000
        COMPOSE_FILE="docker-compose.enterprise.yml"
        ;;
    4)
        # Custom
        echo "🔧 הרצה מותאמת אישית..."
        read -p "📦 Docker image (default: calclaw/personal:latest): " CUSTOM_IMAGE
        IMAGE="${CUSTOM_IMAGE:-calclaw/personal:latest}"
        read -p "🔢 Port (default: 3000): " CUSTOM_PORT
        PORT="${CUSTOM_PORT:-3000}"
        COMPOSE_FILE=""
        ;;
    *)
        echo "❌ בחירה לא תקינה"
        exit 1
        ;;
esac

echo ""
echo "⚙️  הגדרות:"
echo "   📦 Image: $IMAGE"
echo "   🔢 Port: $PORT"
echo ""

# בדוק אם ה-image קיים מקומית
echo "🔍 בודק אם ה-image קיים..."
if ! docker images "$IMAGE" | grep -q "$(echo "$IMAGE" | cut -d: -f1)"; then
    echo "📥 Image לא קיים מקומית. מוריד..."
    docker pull "$IMAGE"
    
    if [[ $? -ne 0 ]]; then
        echo "❌ לא ניתן להוריד את ה-image"
        echo ""
        echo "🚀 בונה image מקומי..."
        read -p "לבנות image מקומי? (y/n): " BUILD_LOCAL
        
        if [[ "$BUILD_LOCAL" == "y" || "$BUILD_LOCAL" == "Y" ]]; then
            chmod +x build_docker.sh
            ./build_docker.sh
        else
            echo "❌ הרצה בוטלה"
            exit 1
        fi
    fi
else
    echo "✅ Image קיים מקומית"
fi

# שאל אם להשתמש ב-docker-compose או docker run
echo ""
if [[ ! -z "$COMPOSE_FILE" && -f "$COMPOSE_FILE" ]]; then
    echo "📄 נמצא קובץ docker-compose: $COMPOSE_FILE"
    read -p "🚀 להשתמש ב-docker-compose? (y/n): " USE_COMPOSE
else
    USE_COMPOSE="n"
fi

if [[ "$USE_COMPOSE" == "y" || "$USE_COMPOSE" == "Y" ]]; then
    # הרץ עם docker-compose
    echo ""
    echo "🚀 מריץ עם docker-compose..."
    
    # צור קובץ .env אם צריך
    if [[ "$CHOICE" == "2" || "$CHOICE" == "3" ]] && [[ ! -f ".env.enterprise" ]]; then
        echo "📝 יוצר קובץ .env.enterprise..."
        cat > .env.enterprise << EOF
# Calclaw Enterprise Environment
PORT=$PORT
DOMAIN=localhost
ACME_EMAIL=admin@localhost
DB_USER=calclaw
DB_NAME=calclaw
DB_PASSWORD=$(openssl rand -base64 32)
REDIS_PASSWORD=$(openssl rand -base64 32)
JWT_SECRET=$(openssl rand -base64 64)
GRAFANA_PASSWORD=admin
TRAEFIK_AUTH=admin:$(openssl passwd -apr1 admin 2>/dev/null || echo "admin")
REPLICAS=1
RUST_LOG=info
EOF
        echo "✅ קובץ .env.enterprise נוצר"
    fi
    
    # הרץ docker-compose
    echo "🐳 מריץ: docker-compose -f $COMPOSE_FILE up -d"
    docker-compose -f "$COMPOSE_FILE" up -d
    
    echo ""
    echo "⏳ ממתין לשירותים להתחיל..."
    sleep 10
    
    # בדוק סטטוס
    echo "📊 סטטוס שירותים:"
    docker-compose -f "$COMPOSE_FILE" ps
    
    echo ""
    echo "🎉 Calclaw רץ עם docker-compose!"
    echo ""
    echo "🌐 ממשקים זמינים:"
    
    # הצג URLs לפי גרסה
    case $CHOICE in
        1)
            echo "   🦾 Calclaw: http://localhost:$PORT"
            echo "   🤖 Ollama: http://localhost:11434"
            ;;
        2|3)
            echo "   🦾 Calclaw: http://localhost:$PORT"
            echo "   🤖 Ollama: http://localhost:11434"
            echo "   📊 Grafana: http://localhost:3001 (user: admin, password: admin)"
            echo "   📈 Prometheus: http://localhost:9090"
            echo "   🎯 Traefik Dashboard: http://localhost:8080"
            ;;
    esac
    
    echo ""
    echo "📊 לוגים:"
    echo "   docker-compose -f $COMPOSE_FILE logs -f"
    echo ""
    echo "🛑 עצירה:"
    echo "   docker-compose -f $COMPOSE_FILE down"
    
else
    # הרץ עם docker run פשוט
    echo ""
    echo "🚀 מריץ עם docker run..."
    
    # שאל אם לשמור data
    read -p "💾 לשמור data בין הרצות? (y/n): " PERSIST_DATA
    
    if [[ "$PERSIST_DATA" == "y" || "$PERSIST_DATA" == "Y" ]]; then
        # צור volume
        VOLUME_NAME="calclaw_data_$(date +%s)"
        echo "💾 יוצר volume: $VOLUME_NAME"
        docker volume create "$VOLUME_NAME"
        
        RUN_CMD="docker run -d \
            --name calclaw \
            -p $PORT:3000 \
            -v $VOLUME_NAME:/app/data \
            $IMAGE"
    else
        RUN_CMD="docker run -d \
            --name calclaw \
            -p $PORT:3000 \
            $IMAGE"
    fi
    
    echo "🐳 מריץ: $RUN_CMD"
    eval "$RUN_CMD"
    
    echo ""
    echo "⏳ ממתין ל-Calclaw להתחיל..."
    sleep 5
    
    # בדוק אם רץ
    if docker ps | grep -q calclaw; then
        echo "✅ Calclaw רץ!"
        
        # בדוק health
        echo "🔍 בודק בריאות..."
        if curl -s http://localhost:$PORT/api/health > /dev/null; then
            echo "✅ בריאות תקינה"
        else
            echo "⚠️  בריאות לא תקינה, אבל השרת רץ"
        fi
        
        echo ""
        echo "🎉 Calclaw רץ עם Docker!"
        echo ""
        echo "🌐 ממשקים:"
        echo "   🦾 Calclaw: http://localhost:$PORT"
        echo "   📖 API docs: http://localhost:$PORT/docs"
        echo ""
        echo "📊 לוגים:"
        echo "   docker logs -f calclaw"
        echo ""
        echo "🛑 עצירה:"
        echo "   docker stop calclaw && docker rm calclaw"
        
        if [[ "$PERSIST_DATA" == "y" || "$PERSIST_DATA" == "Y" ]]; then
            echo ""
            echo "💾 Data נשמר ב-volume: $VOLUME_NAME"
            echo "🗑️  מחיקת volume: docker volume rm $VOLUME_NAME"
        fi
        
    else
        echo "❌ Calclaw לא רץ"
        echo ""
        echo "📝 בדוק לוגים:"
        echo "   docker logs calclaw"
        exit 1
    fi
fi

echo ""
echo "📚 משאבים נוספים:"
echo "   📖 תיעוד: https://docs.calclaw.com"
echo "   💬 קהילה: https://community.calclaw.com"
echo "   🐙 קוד: https://github.com/calclaw"
echo ""
echo "🚀 Calclaw מופעל עם Docker! 🎉"

# שאל אם לפתוח בדפדפן
echo ""
read -p "🌐 לפתוח בדפדפן? (y/n): " OPEN_BROWSER

if [[ "$OPEN_BROWSER" == "y" || "$OPEN_BROWSER" == "Y" ]]; then
    echo "🌐 פותח בדפדפן..."
    
    if command -v xdg-open &> /dev/null; then
        xdg-open "http://localhost:$PORT"
    elif command -v open &> /dev/null; then
        open "http://localhost:$PORT"
    elif command -v start &> /dev/null; then
        start "http://localhost:$PORT"
    else
        echo "⚠️  לא ניתן לפתוח דפדפן אוטומטית"
        echo "   פתח ידנית: http://localhost:$PORT"
    fi
fi

echo ""
echo "🦾 Calclaw עם Docker מוכן לשימוש! 🎉"