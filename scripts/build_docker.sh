#!/bin/bash

# 🐳 Calclaw - Docker Build Script
# בונה Docker images לכל הגרסאות

set -e

echo "🐳 Calclaw - Docker Build"
echo "========================"
echo ""

# שאל איזה image לבנות
echo "🔧 בחר image לבנייה:"
echo "   1. 🏠 Personal (קל ומהיר)"
echo "   2. 🏢 Enterprise (מתקדם עם אבטחה)"
echo "   3. 🏛️  Claw Organ (ארגוני מתקדם)"
echo "   4. 🔄 All (כל ה-images)"
echo "   5. 📦 Multi-arch (ל-production)"
echo ""

read -p "בחר אפשרות (1-5): " CHOICE

# הגדר משתנים
TAG="${TAG:-latest}"
REGISTRY="${REGISTRY:-calclaw}"
BUILD_ARGS=""
PLATFORMS="linux/amd64"

case $CHOICE in
    1)
        # Personal image
        echo "🏠 בונה Personal image..."
        DOCKERFILE="Dockerfile.personal"
        IMAGE_NAME="$REGISTRY/personal:$TAG"
        BUILD_ARGS="--build-arg BUILDKIT_INLINE_CACHE=1"
        ;;
    2)
        # Enterprise image
        echo "🏢 בונה Enterprise image..."
        DOCKERFILE="Dockerfile.enterprise"
        IMAGE_NAME="$REGISTRY/enterprise:$TAG"
        BUILD_ARGS="--build-arg BUILDKIT_INLINE_CACHE=1 --secret id=github_token,src=$HOME/.github_token"
        ;;
    3)
        # Claw Organ image
        echo "🏛️  בונה Claw Organ image..."
        DOCKERFILE="Dockerfile.enterprise"  # להשתמש ב-enterprise כ-base
        IMAGE_NAME="$REGISTRY/claw-organ:$TAG"
        BUILD_ARGS="--build-arg BUILDKIT_INLINE_CACHE=1 --build-arg VERSION=organ-1.0.0"
        ;;
    4)
        # All images
        echo "🔄 בונה כל ה-images..."
        
        # Personal
        echo ""
        echo "🏠 בונה Personal..."
        docker build -f Dockerfile.personal -t "$REGISTRY/personal:$TAG" .
        
        # Enterprise
        echo ""
        echo "🏢 בונה Enterprise..."
        docker build -f Dockerfile.enterprise -t "$REGISTRY/enterprise:$TAG" .
        
        # Claw Organ
        echo ""
        echo "🏛️  בונה Claw Organ..."
        docker build -f Dockerfile.enterprise \
            --build-arg VERSION=organ-1.0.0 \
            -t "$REGISTRY/claw-organ:$TAG" .
        
        echo ""
        echo "✅ כל ה-images נבנו!"
        echo ""
        echo "📦 Images שנוצרו:"
        docker images | grep "$REGISTRY"
        exit 0
        ;;
    5)
        # Multi-arch build
        echo "📦 בונה multi-arch images..."
        PLATFORMS="linux/amd64,linux/arm64"
        
        # Personal multi-arch
        echo ""
        echo "🏠 בונה Personal multi-arch..."
        docker buildx build \
            --platform "$PLATFORMS" \
            -f Dockerfile.personal \
            -t "$REGISTRY/personal:$TAG" \
            -t "$REGISTRY/personal:latest" \
            --push .
        
        # Enterprise multi-arch
        echo ""
        echo "🏢 בונה Enterprise multi-arch..."
        docker buildx build \
            --platform "$PLATFORMS" \
            -f Dockerfile.enterprise \
            -t "$REGISTRY/enterprise:$TAG" \
            -t "$REGISTRY/enterprise:latest" \
            --push .
        
        echo ""
        echo "✅ Multi-arch images נבנו והועלו ל-registry!"
        exit 0
        ;;
    *)
        echo "❌ בחירה לא תקינה"
        exit 1
        ;;
esac

# בדוק אם Docker מותקן
if ! command -v docker &> /dev/null; then
    echo "❌ Docker לא מותקן"
    echo "📖 התקן Docker: https://docs.docker.com/get-docker/"
    exit 1
fi

# בדוק אם Docker רץ
if ! docker info > /dev/null 2>&1; then
    echo "❌ Docker לא רץ"
    echo "🚀 הפעל את Docker daemon"
    exit 1
fi

# שאל tag אם לא הוגדר
if [[ "$TAG" == "latest" ]]; then
    read -p "🏷️  Enter tag (default: latest): " CUSTOM_TAG
    if [[ ! -z "$CUSTOM_TAG" ]]; then
        TAG="$CUSTOM_TAG"
        IMAGE_NAME="${IMAGE_NAME%:*}:$TAG"
    fi
fi

# שאל registry אם לא הוגדר
read -p "📦 Docker registry (default: $REGISTRY): " CUSTOM_REGISTRY
if [[ ! -z "$CUSTOM_REGISTRY" ]]; then
    REGISTRY="$CUSTOM_REGISTRY"
    IMAGE_NAME="$REGISTRY/${IMAGE_NAME#*/}"
fi

# בנה את ה-image
echo ""
echo "🔨 בונה $IMAGE_NAME..."
echo "📄 Dockerfile: $DOCKERFILE"
echo "🏗️  Platforms: $PLATFORMS"
echo ""

# הרץ את הבנייה
docker build \
    -f "$DOCKERFILE" \
    -t "$IMAGE_NAME" \
    $BUILD_ARGS \
    .

# בדוק אם הבנייה הצליחה
if [[ $? -eq 0 ]]; then
    echo ""
    echo "✅ Image נבנה בהצלחה!"
    echo ""
    echo "📦 Image details:"
    docker images "$IMAGE_NAME"
    
    # שאל אם להריץ בדיקות
    echo ""
    read -p "🧪 להריץ בדיקות על ה-image? (y/n): " RUN_TESTS
    
    if [[ "$RUN_TESTS" == "y" || "$RUN_TESTS" == "Y" ]]; then
        echo ""
        echo "🧪 מריץ בדיקות..."
        
        # בדוק גודל image
        echo "1. 📊 בודק גודל image..."
        SIZE=$(docker images "$IMAGE_NAME" --format "{{.Size}}")
        echo "   ✅ גודל: $SIZE"
        
        # הרץ container לבדיקה
        echo "2. 🐳 בודק הרצת container..."
        TEST_CONTAINER="calclaw-test-$(date +%s)"
        docker run -d --name "$TEST_CONTAINER" -p 9999:3000 "$IMAGE_NAME" > /dev/null 2>&1
        
        sleep 5
        
        # בדוק אם השרת רץ
        if curl -s http://localhost:9999/api/health > /dev/null; then
            echo "   ✅ Server רץ בהצלחה"
        else
            echo "   ❌ Server לא רץ"
        fi
        
        # עצור ונקה
        docker stop "$TEST_CONTAINER" > /dev/null 2>&1
        docker rm "$TEST_CONTAINER" > /dev/null 2>&1
        echo "   ✅ Container נוקה"
        
        # בדוק security vulnerabilities
        echo "3. 🔒 בודק security vulnerabilities..."
        if command -v trivy &> /dev/null; then
            trivy image --severity HIGH,CRITICAL "$IMAGE_NAME" | head -20
        else
            echo "   ⚠️  Trivy לא מותקן. התקן עם:"
            echo "      brew install aquasecurity/trivy/trivy  # macOS"
            echo "      sudo apt-get install trivy             # Ubuntu"
        fi
        
        echo ""
        echo "✅ בדיקות הושלמו!"
    fi
    
    # שאל אם להעלות ל-registry
    echo ""
    read -p "☁️  להעלות ל-Docker registry? (y/n): " PUSH_TO_REGISTRY
    
    if [[ "$PUSH_TO_REGISTRY" == "y" || "$PUSH_TO_REGISTRY" == "Y" ]]; then
        echo ""
        echo "☁️  מעלה ל-registry..."
        
        # התחבר ל-registry אם צריך
        if [[ "$REGISTRY" != "calclaw" ]]; then
            echo "🔐 התחברות ל-$REGISTRY..."
            docker login "$REGISTRY"
        fi
        
        # העלה את ה-image
        docker push "$IMAGE_NAME"
        
        if [[ $? -eq 0 ]]; then
            echo ""
            echo "✅ Image הועלה בהצלחה ל-$REGISTRY!"
            echo ""
            echo "📦 Pull command:"
            echo "   docker pull $IMAGE_NAME"
            echo ""
            echo "🐳 Run command:"
            echo "   docker run -p 3000:3000 $IMAGE_NAME"
        else
            echo "❌ שגיאה בהעלאת ה-image"
        fi
    fi
    
    # שאל אם להריץ עם docker-compose
    echo ""
    read -p "🚀 להריץ עם docker-compose? (y/n): " RUN_COMPOSE
    
    if [[ "$RUN_COMPOSE" == "y" || "$RUN_COMPOSE" == "Y" ]]; then
        echo ""
        echo "🚀 מריץ docker-compose..."
        
        # בחר קובץ docker-compose מתאים
        case $CHOICE in
            1)
                COMPOSE_FILE="docker-compose.yml"
                echo "📄 משתמש ב: $COMPOSE_FILE"
                ;;
            2|3)
                COMPOSE_FILE="docker-compose.enterprise.yml"
                echo "📄 משתמש ב: $COMPOSE_FILE"
                
                # צור קובץ .env אם לא קיים
                if [[ ! -f ".env.enterprise" ]]; then
                    echo "📝 יוצר קובץ .env.enterprise..."
                    cat > .env.enterprise << EOF
# Calclaw Enterprise Environment
PORT=3000
DOMAIN=calclaw.local
ACME_EMAIL=admin@calclaw.local
DB_USER=calclaw
DB_NAME=calclaw
DB_PASSWORD=$(openssl rand -base64 32)
REDIS_PASSWORD=$(openssl rand -base64 32)
JWT_SECRET=$(openssl rand -base64 64)
GRAFANA_PASSWORD=admin
TRAEFIK_AUTH=admin:\$(openssl passwd -apr1 admin)
REPLICAS=2
RUST_LOG=info
EOF
                    echo "✅ קובץ .env.enterprise נוצר"
                fi
                ;;
        esac
        
        # הרץ docker-compose
        if [[ -f "$COMPOSE_FILE" ]]; then
            echo "🐳 מריץ: docker-compose -f $COMPOSE_FILE up -d"
            docker-compose -f "$COMPOSE_FILE" up -d
            
            echo ""
            echo "🎉 Calclaw רץ עם Docker!"
            echo ""
            echo "🌐 ממשקים:"
            echo "   🦾 Calclaw: http://localhost:3000"
            
            if [[ "$CHOICE" == "2" || "$CHOICE" == "3" ]]; then
                echo "   📊 Grafana: http://localhost:3001"
                echo "   📈 Prometheus: http://localhost:9090"
                echo "   🎯 Traefik Dashboard: http://localhost:8080"
            fi
            
            echo ""
            echo "📊 לוגים:"
            echo "   docker-compose -f $COMPOSE_FILE logs -f"
            echo ""
            echo "🛑 עצירה:"
            echo "   docker-compose -f $COMPOSE_FILE down"
        else
            echo "❌ קובץ $COMPOSE_FILE לא נמצא"
        fi
    fi
    
    echo ""
    echo "🎉 Docker build הושלם!"
    echo ""
    echo "📦 Image: $IMAGE_NAME"
    echo "🐳 ניתן להריץ עם: docker run -p 3000:3000 $IMAGE_NAME"
    echo ""
    echo "🚀 Calclaw מוכן עם Docker! 🎉"
    
else
    echo "❌ שגיאה בבניית Docker image"
    exit 1
fi