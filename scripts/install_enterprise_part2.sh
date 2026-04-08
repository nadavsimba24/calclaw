רציות לפי בחירה
echo ""
echo "4. 🔗 מתקין אינטגרציות..."

# פענח את רשימת האינטגרציות
IFS=',' read -ra INTEGRATION_LIST <<< "$INTEGRATIONS"

for integration in "${INTEGRATION_LIST[@]}"; do
    case $integration in
        "slack")
            echo "   💬 Slack integration..."
            cat > "$CONFIG_DIR/slack_config.json" << EOF
{
  "enabled": true,
  "bot_token": "xoxb-YOUR-TOKEN-HERE",
  "signing_secret": "YOUR-SIGNING-SECRET",
  "app_token": "xapp-YOUR-APP-TOKEN",
  "channels": {
    "general": "C12345678",
    "approvals": "C87654321",
    "notifications": "C11223344"
  },
  "features": {
    "slash_commands": true,
    "interactive_components": true,
    "event_subscriptions": true,
    "bot_users": true
  }
}
EOF
            echo "   ✅ קובץ Slack config נוצר"
            ;;
            
        "github")
            echo "   🐙 GitHub integration..."
            cat > "$CONFIG_DIR/github_config.json" << EOF
{
  "enabled": true,
  "personal_access_token": "ghp_YOUR_TOKEN_HERE",
  "organization": "${ORG_NAME:-your-org}",
  "repositories": ["calclaw", "docs", "infrastructure"],
  "webhook_secret": "YOUR_WEBHOOK_SECRET",
  "features": {
    "pull_requests": true,
    "issues": true,
    "actions": true,
    "packages": true
  }
}
EOF
            echo "   ✅ קובץ GitHub config נוצר"
            ;;
            
        "docker")
            echo "   🐳 Docker integration..."
            cat > "$CONFIG_DIR/docker_config.json" << EOF
{
  "enabled": true,
  "registry": "docker.io",
  "username": "your-username",
  "password": "your-password",
  "images": ["calclaw/enterprise", "calclaw/api", "calclaw/ui"],
  "features": {
    "build": true,
    "push": true,
    "pull": true,
    "scan": true
  }
}
EOF
            echo "   ✅ קובץ Docker config נוצר"
            ;;
            
        "kubernetes")
            echo "   🐙 Kubernetes integration..."
            cat > "$CONFIG_DIR/kubernetes_integration.json" << EOF
{
  "enabled": true,
  "context": "production",
  "namespace": "calclaw-enterprise",
  "resources": {
    "deployments": true,
    "services": true,
    "configmaps": true,
    "secrets": true,
    "ingresses": true
  },
  "features": {
    "apply": true,
    "delete": true,
    "logs": true,
    "exec": true
  }
}
EOF
            echo "   ✅ קובץ Kubernetes integration נוצר"
            ;;
            
        "saas"|"custom")
            echo "   🔌 אינטגרציות SaaS/מותאמות..."
            cat > "$CONFIG_DIR/custom_integrations.json" << EOF
{
  "enabled": true,
  "mcp_servers": [
    {
      "name": "filesystem",
      "command": "npx",
      "args": ["@modelcontextprotocol/server-filesystem", "/data"],
      "requires_approval": false
    },
    {
      "name": "postgres",
      "command": "npx",
      "args": ["@modelcontextprotocol/server-postgres"],
      "requires_approval": true,
      "env": {
        "PGHOST": "localhost",
        "PGDATABASE": "calclaw",
        "PGUSER": "calclaw_user"
      }
    },
    {
      "name": "redis",
      "command": "npx",
      "args": ["@modelcontextprotocol/server-redis"],
      "requires_approval": true,
      "env": {
        "REDIS_URL": "redis://localhost:6379"
      }
    }
  ]
}
EOF
            echo "   ✅ קובץ custom integrations נוצר"
            ;;
    esac
done

if [[ -z "$INTEGRATIONS" ]]; then
    echo "   ⏭️  אין אינטגרציות נדרשות"
fi

# שלב 5: צור סקריפטי ניהול ארגוניים
echo ""
echo "5. 📜 יוצר סקריפטי ניהול ארגוניים..."

# סקריפט הפעלה ארגוני
cat > start_enterprise.sh << 'EOF'
#!/bin/bash

# 🏢 Calclaw - סקריפט הפעלה ארגונית
# מפעיל את Calclaw Enterprise בסביבה ארגונית

set -e

echo "🏢 Calclaw Enterprise - הפעלה"
echo "============================="

# טען קונפיגורציה
CONFIG_FILE="config/enterprise/install_config.json"
if [[ ! -f "$CONFIG_FILE" ]]; then
    echo "❌ קובץ קונפיגורציה לא נמצא: $CONFIG_FILE"
    echo "   הרץ קודם: ./install_enterprise.sh"
    exit 1
fi

# קרא את הקונפיגורציה
ENVIRONMENT=$(jq -r '.environment' "$CONFIG_FILE")
SECURITY_LEVEL=$(jq -r '.security_level' "$CONFIG_FILE")
USERS=$(jq -r '.users' "$CONFIG_FILE")

echo "⚙️  הגדרות:"
echo "   🖥️  סביבה: $ENVIRONMENT"
echo "   🔒 אבטחה: $SECURITY_LEVEL"
echo "   👥 משתמשים: $USERS"
echo ""

# הפעל לפי סביבה
case $ENVIRONMENT in
    "local"|"server")
        echo "🚀 מפעיל בסביבה מקומית/שרת..."
        
        # הפעל Ollama עם Docker Compose אם קיים
        if [[ -f "config/enterprise/docker-compose.ollama.yml" ]]; then
            echo "🤖 מפעיל Ollama..."
            docker-compose -f config/enterprise/docker-compose.ollama.yml up -d
            sleep 5
        fi
        
        # הפעל את Calclaw
        echo "🦾 מפעיל Calclaw Enterprise..."
        ./target/release/calclaw \
            --config config/enterprise/install_config.json \
            > /var/log/calclaw/enterprise.log 2>&1 &
        
        CALCLAW_PID=$!
        echo "   ✅ Calclaw Enterprise רץ (PID: $CALCLAW_PID)"
        ;;
        
    "cloud")
        echo "☁️  סביבת ענן - ראה הוראות פריסה ב-cloud_config.json"
        echo "   📁 קובץ קונפיגורציה: config/enterprise/cloud_config.json"
        echo "   📖 הוראות: https://docs.calclaw.com/deployment/cloud"
        ;;
        
    "k8s")
        echo "🐳 סביבת Kubernetes - ראה manifests ב-kubernetes_config.yaml"
        echo "   📁 קובץ manifests: config/enterprise/kubernetes_config.yaml"
        echo "   📖 הוראות: https://docs.calclaw.com/deployment/kubernetes"
        ;;
esac

echo ""
echo "🎉 Calclaw Enterprise מופעל!"
echo ""
echo "📊 לוגים:"
echo "   📝 Calclaw: /var/log/calclaw/enterprise.log"
echo "   🤖 Ollama: docker logs ollama_ollama_1"
echo ""
echo "🌐 ממשקים:"
echo "   🏢 דשבורד: http://localhost:3000/enterprise"
echo "   📖 API docs: http://localhost:3000/api/docs"
echo "   🔒 Admin: http://localhost:3000/admin"
echo ""
echo "🛑 לעצירה: ./stop_enterprise.sh"
EOF

chmod +x start_enterprise.sh

# סקריפט עצירה ארגוני
cat > stop_enterprise.sh << 'EOF'
#!/bin/bash

# 🛑 Calclaw - סקריפט עצירה ארגונית

echo "🛑 עוצר Calclaw Enterprise..."

# עצור את Calclaw
pkill -f "target/release/calclaw" || true
echo "✅ Calclaw Enterprise נעצר"

# עצור את Ollama אם רץ עם Docker Compose
if [[ -f "config/enterprise/docker-compose.ollama.yml" ]]; then
    echo "🤖 עוצר Ollama..."
    docker-compose -f config/enterprise/docker-compose.ollama.yml down
    echo "✅ Ollama נעצר"
fi

echo ""
echo "🎯 כל השירותים נעצרו"
EOF

chmod +x stop_enterprise.sh

# סקריפט בדיקות ארגוני
cat > test_enterprise.sh << 'EOF'
#!/bin/bash

# 🧪 Calclaw - סקריפט בדיקות ארגוניות

echo "🧪 Calclaw Enterprise - בדיקות מערכת"
echo "===================================="

# טען קונפיגורציה
CONFIG_FILE="config/enterprise/install_config.json"
if [[ ! -f "$CONFIG_FILE" ]]; then
    echo "❌ קובץ קונפיגורציה לא נמצא"
    exit 1
fi

ENVIRONMENT=$(jq -r '.environment' "$CONFIG_FILE")
SECURITY_LEVEL=$(jq -r '.security_level' "$CONFIG_FILE")

echo "⚙️  סביבה: $ENVIRONMENT"
echo "🔒 אבטחה: $SECURITY_LEVEL"
echo ""

# בדיקות לפי סביבה
case $ENVIRONMENT in
    "local"|"server")
        echo "1. 🔍 בדיקת שירותים מקומיים..."
        
        # בדוק אם Docker רץ
        if docker ps > /dev/null 2>&1; then
            echo "   ✅ Docker רץ"
        else
            echo "   ❌ Docker לא רץ"
        fi
        
        # בדוק אם Ollama רץ
        if curl -s http://localhost:11434/api/tags > /dev/null 2>&1; then
            echo "   ✅ Ollama רץ"
        else
            echo "   ❌ Ollama לא רץ"
        fi
        
        # בדוק אם Calclaw רץ
        if curl -s http://localhost:3000/api/health > /dev/null 2>&1; then
            echo "   ✅ Calclaw רץ"
        else
            echo "   ❌ Calclaw לא רץ"
        fi
        ;;
        
    "cloud"|"k8s")
        echo "1. 🔍 בדיקות ענן/Kubernetes..."
        echo "   📖 ראה הוראות בדיקה ב:"
        echo "      https://docs.calclaw.com/testing/enterprise"
        ;;
esac

# בדיקות אבטחה
echo ""
echo "2. 🔒 בדיקות אבטחה ($SECURITY_LEVEL)..."

case $SECURITY_LEVEL in
    "low")
        echo "   🟢 אבטחה בסיסית - בדיקות מינימליות"
        ;;
    "medium")
        echo "   🟡 אבטחה בינונית - בדיקת MCP Proxy"
        if [[ -f "config/enterprise/mcp_proxy_config.json" ]]; then
            echo "   ✅ קובץ MCP Proxy קיים"
        else
            echo "   ❌ קובץ MCP Proxy חסר"
        fi
        ;;
    "high")
        echo "   🔴 אבטחה מתקדמת - בדיקות מלאות"
        if [[ -f "config/enterprise/security_advanced.json" ]]; then
            echo "   ✅ קובץ אבטחה מתקדמת קיים"
        else
            echo "   ❌ קובץ אבטחה מתקדמת חסר"
        fi
        ;;
esac

# בדיקות אינטגרציות
echo ""
echo "3. 🔗 בדיקות אינטגרציות..."

INTEGRATIONS=$(jq -r '.integrations' "$CONFIG_FILE")
if [[ "$INTEGRATIONS" != "null" && ! -z "$INTEGRATIONS" ]]; then
    IFS=',' read -ra INT_LIST <<< "$INTEGRATIONS"
    for integration in "${INT_LIST[@]}"; do
        CONFIG_FILE="config/enterprise/${integration}_config.json"
        if [[ -f "$CONFIG_FILE" ]]; then
            echo "   ✅ $integration - קובץ קונפיגורציה קיים"
        else
            echo "   ❌ $integration - קובץ קונפיגורציה חסר"
        fi
    done
else
    echo "   ⏭️  אין אינטגרציות לבדיקה"
fi

echo ""
echo "✅ בדיקות הושלמו!"
echo ""
echo "📝 המלצות:"
echo "   • הרץ את המערכת: ./start_enterprise.sh"
echo "   • בדוק לוגים: tail -f /var/log/calclaw/enterprise.log"
echo "   • בצע penetration testing אם רמת אבטחה גבוהה"
EOF

chmod +x test_enterprise.sh

echo "   ✅ סקריפטי ניהול ארגוניים נוצרו:"
echo "      🚀 start_enterprise.sh - הפעלת המערכת"
echo "      🛑 stop_enterprise.sh - עצירת המערכת"
echo "      🧪 test_enterprise.sh - בדיקות מערכת"

# שלב 6: צור תיעוד ארגוני
echo ""
echo "6. 📝 יוצר תיעוד ארגוני..."

cat > ENTERPRISE_GUIDE.md << EOF
# 🏢 Calclaw Enterprise - מדריך ארגוני

## 🎯 סקירה כללית

Calclaw Enterprise היא גרסה מתקדמת של Calclaw המותאמת לארגונים עם דרישות אבטחה, multi-user, ואינטגרציות.

### 📊 מפרט טכני:
- **משתמשים**: עד $USERS משתמשים
- **סביבה**: $ENVIRONMENT
- **אבטחה**: $SECURITY_LEVEL
- **אינטגרציות**: ${INTEGRATIONS:-none}

## 🚀 התחלה מהירה

### הפעלה:
\`\`\`bash
./start_enterprise.sh
\`\`\`

### עצירה:
\`\`\`bash
./stop_enterprise.sh
\`\`\`

### בדיקות:
\`\`\`bash
./test_enterprise.sh
\`\`\`

## 🌐 ממשקים

### דשבורדים:
- **דשבורד ארגוני**: http://localhost:3000/enterprise
- **ניהול משתמשים**: http://localhost:3000/admin/users
- **ניטור מערכת**: http://localhost:3000/admin/monitoring
- **לוגים ואודיט**: http://localhost:3000/admin/audit

### API:
- **בריאות**: \`GET /api/health\`
- **משתמשים**: \`GET /api/users\`
- **אודיט**: \`GET /api/audit\`
- **ניטור**: \`GET /api/metrics\`

## 🔒 אבטחה

### רמת אבטחה: $SECURITY_LEVEL

$(case $SECURITY_LEVEL in
    "low")
        echo "#### 🟢 אבטחה בסיסית"
        echo "- הגבלות מינימליות"
        echo "- גישה מלאה למשתמשים מאושרים"
        echo "- אין אישורים נדרשים"
        ;;
    "medium")
        echo "#### 🟡 אבטחה בינונית"
        echo "- MCP Proxy עם אישורים"
        echo "- Audit logging"
        echo "- הגבלות על פעולות מסוכנות"
        echo "- אישורים דרך Slack/email"
        ;;
    "high")
        echo "#### 🔴 אבטחה מתקדמת"
        echo "- Network policies"
        echo "- Multi-factor authentication"
        echo "- Encryption at rest ובתעבורה"
        echo "- SIEM integration"
        echo "- Compliance (GDPR, etc.)"
        ;;
esac)

## 🔗 אינטגרציות

$(if [[ ! -z "$INTEGRATIONS" ]]; then
    IFS=',' read -ra INT_LIST <<< "$INTEGRATIONS"
    echo "### אינטגרציות מופעלות:"
    for integration in "${INT_LIST[@]}"; do
        case $integration in
            "slack") echo "- **Slack**: אינטגרציה מלאה עם Slack workspace" ;;
            "github") echo "- **GitHub**: ניהול repositories, pull requests, issues" ;;
            "docker") echo "- **Docker**: ניהול containers, images, registry" ;;
            "kubernetes") echo "- **Kubernetes**: ניהול clusters, deployments, services" ;;
            "saas"|"custom") echo "- **SaaS/Custom**: אינטגרציות מותאמות עם MCP servers"