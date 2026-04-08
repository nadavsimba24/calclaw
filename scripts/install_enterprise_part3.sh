        ;;
        esac
    done
else
    echo "### אין אינטגרציות מופעלות"
fi)

## 🐳 פריסה

### סביבה: $ENVIRONMENT

$(case $ENVIRONMENT in
    "local"|"server")
        echo "#### 🖥️ סביבה מקומית/שרת"
        echo "- רץ על Docker/Docker Compose"
        echo "- קובץ קונפיגורציה: \`config/enterprise/docker-compose.ollama.yml\`"
        echo "- הרצה: \`docker-compose -f config/enterprise/docker-compose.ollama.yml up -d\`"
        ;;
    "cloud")
        echo "#### ☁️ סביבת ענן"
        echo "- קובץ קונפיגורציה: \`config/enterprise/cloud_config.json\`"
        echo "- הוראות פריסה: https://docs.calclaw.com/deployment/cloud"
        echo "- תמיכה ב-AWS, Azure, GCP"
        ;;
    "k8s")
        echo "#### 🐳 סביבת Kubernetes"
        echo "- קובץ manifests: \`config/enterprise/kubernetes_config.yaml\`"
        echo "- הרצה: \`kubectl apply -f config/enterprise/kubernetes_config.yaml\`"
        echo "- Helm chart זמין: \`helm install calclaw ./charts/calclaw\`"
        ;;
esac)

## 📊 ניטור וניהול

### לוגים:
- **Calclaw**: \`/var/log/calclaw/enterprise.log\`
- **Ollama**: \`docker logs ollama_ollama_1\`
- **Access logs**: \`/var/log/calclaw/access.log\`
- **Audit logs**: \`/var/log/calclaw/audit.log\`

### ניטור:
- **Metrics**: Prometheus endpoint ב-\`/metrics\`
- **Health checks**: \`/health\`, \`/ready\`, \`/live\`
- **Alerting**: אינטגרציה עם Prometheus Alertmanager

### גיבויים:
- **אוטומטיים**: יומיים
- **שמירה**: 30 יום
- **שחזור**: \`./scripts/restore_backup.sh <backup-file>\`

## 👥 ניהול משתמשים

### הרשאות:
- **Admin**: גישה מלאה
- **Manager**: ניהול צוותים
- **User**: גישה מוגבלת
- **Guest**: גישה לקריאה בלבד

### אימות:
- **OAuth 2.0**: עם GitHub, Google, Microsoft
- **SAML**: לארגונים עם IdP
- **LDAP**: אינטגרציה עם Active Directory
- **Multi-factor**: עם Google Authenticator/Authy

## 🆘 תמיכה

### רמות תמיכה:
1. **Community**: תיעוד ופורומים
2. **Standard**: תמיכת email ב-business hours
3. **Premium**: תמיכה 24/7 עם SLA
4. **Enterprise**: מהנדס צמוד, on-site training

### משאבים:
- **תיעוד**: https://docs.calclaw.com/enterprise
- **קהילה**: https://community.calclaw.com
- **תמיכה**: enterprise-support@calclaw.com
- **חירום**: +1-800-CALCLAW

## 📈 סקיילביליות

### הרחבה אופקית:
- הוספת nodes ל-Kubernetes cluster
- Load balancing עם Traefik/NGINX
- Caching עם Redis
- Database clustering עם PostgreSQL

### הרחבה אנכית:
- שדרוג instance types
- הוספת storage
- הגדלת memory/CPU

## 🔄 שדרוגים

### עדכונים:
- **Patch releases**: אוטומטיים
- **Minor releases**: חצי-אוטומטיים
- **Major releases**: ידניים עם migration

### גיבוי לפני שדרוג:
\`\`\`bash
./scripts/backup_system.sh
./update_enterprise.sh --version=<new-version>
\`\`\`

## 🎯 best practices

### אבטחה:
1. שמור tokens ו-secrets ב-secret management
2. הפעל regular security audits
3. עדכן dependencies באופן קבוע
4. השתמש ב-network policies

### ביצועים:
1. Monitor resource usage
2. הגדר resource quotas
3. השתמש ב-caching
4. Optimize database queries

### אמינות:
1. הגדר health checks
2. השתמש ב-circuit breakers
3. Implement retry logic
4. שמור backup ו-disaster recovery plans

---

**Calclaw Enterprise גרסה 1.0.0** 🏢
*מותאם אישית עבור $ORG_NAME*
*איש קשר: $CONTACT_EMAIL*
*תאריך התקנה: $(date +"%Y-%m-%d")*
EOF

echo "   ✅ תיעוד ארגוני נוצר: ENTERPRISE_GUIDE.md"

# שלב 7: צור סקריפטי utilities נוספים
echo ""
echo "7. 🛠️ יוצר סקריפטי utilities..."

# סקריפט גיבוי
cat > scripts/backup_system.sh << 'EOF'
#!/bin/bash

# 💾 Calclaw Enterprise - סקריפט גיבוי

set -e

BACKUP_DIR="/backup/calclaw"
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
BACKUP_FILE="$BACKUP_DIR/calclaw_backup_$TIMESTAMP.tar.gz"

echo "💾 Calclaw Enterprise - גיבוי מערכת"
echo "=================================="

# צור תיקיית גיבויים אם לא קיימת
mkdir -p "$BACKUP_DIR"

echo "📦 מגבה קבצי קונפיגורציה..."
tar -czf "$BACKUP_FILE" \
    config/enterprise/ \
    scripts/ \
    ENTERPRISE_GUIDE.md \
    Cargo.toml \
    Cargo.lock 2>/dev/null || true

# גבה את ה-database אם קיים
if [[ -f "data/calclaw.db" ]]; then
    echo "🗄️  מגבה database..."
    sqlite3 data/calclaw.db ".backup '$BACKUP_DIR/calclaw_db_$TIMESTAMP.db'"
fi

# גבה את ה-docker volumes אם קיימים
if docker volume ls | grep -q ollama_models; then
    echo "🐳 מגבה Docker volumes..."
    docker run --rm \
        -v ollama_models:/source \
        -v "$BACKUP_DIR":/backup \
        alpine tar -czf /backup/ollama_models_$TIMESTAMP.tar.gz -C /source .
fi

echo ""
echo "✅ גיבוי הושלם!"
echo "📁 קבצי גיבוי:"
ls -la "$BACKUP_DIR"/*"$TIMESTAMP"*
echo ""
echo "📊 גודל גיבוי:"
du -h "$BACKUP_FILE"
echo ""
echo "🔒 המלצה: העתק את קבצי הגיבוי ל-offsite storage"
EOF

chmod +x scripts/backup_system.sh

# סקריפט שחזור
cat > scripts/restore_backup.sh << 'EOF'
#!/bin/bash

# 🔄 Calclaw Enterprise - סקריפט שחזור

set -e

echo "🔄 Calclaw Enterprise - שחזור מגיבוי"
echo "==================================="

if [[ $# -eq 0 ]]; then
    echo "שימוש: $0 <backup-file>"
    echo ""
    echo "קבצי גיבוי זמינים:"
    ls -la /backup/calclaw/calclaw_backup_*.tar.gz 2>/dev/null || echo "אין קבצי גיבוי"
    exit 1
fi

BACKUP_FILE="$1"

if [[ ! -f "$BACKUP_FILE" ]]; then
    echo "❌ קובץ גיבוי לא נמצא: $BACKUP_FILE"
    exit 1
fi

echo "📦 משחזר מ: $BACKUP_FILE"
echo "⚠️  זה ידרוס את הקבצים הקיימים!"
read -p "להמשיך? (y/n): " CONFIRM

if [[ "$CONFIRM" != "y" && "$CONFIRM" != "Y" ]]; then
    echo "❌ שחזור בוטל"
    exit 0
fi

# עצור את המערכת אם רץ
echo "🛑 עוצר את המערכת..."
./stop_enterprise.sh 2>/dev/null || true

# שחזר קבצים
echo "📂 משחזר קבצים..."
tar -xzf "$BACKUP_FILE" -C /

# שחזר database אם קיים
DB_BACKUP="${BACKUP_FILE/_backup_/_db_}"
DB_BACKUP="${DB_BACKUP/.tar.gz/.db}"
if [[ -f "$DB_BACKUP" ]]; then
    echo "🗄️  משחזר database..."
    cp "$DB_BACKUP" data/calclaw.db
fi

# שחזר docker volumes אם קיימים
VOLUME_BACKUP="${BACKUP_FILE/_backup_/_ollama_models_}"
if [[ -f "$VOLUME_BACKUP" ]]; then
    echo "🐳 משחזר Docker volumes..."
    docker run --rm \
        -v ollama_models:/target \
        -v "$(dirname "$VOLUME_BACKUP")":/backup \
        alpine tar -xzf "/backup/$(basename "$VOLUME_BACKUP")" -C /target
fi

echo ""
echo "✅ שחזור הושלם!"
echo "🚀 להפעלת המערכת: ./start_enterprise.sh"
EOF

chmod +x scripts/restore_backup.sh

# סקריפט עדכון
cat > update_enterprise.sh << 'EOF'
#!/bin/bash

# 🔄 Calclaw Enterprise - סקריפט עדכון

set -e

echo "🔄 Calclaw Enterprise - עדכון מערכת"
echo "=================================="

VERSION="${1:-latest}"

echo "⚙️  מעדכן לגרסה: $VERSION"

# גבה לפני עדכון
echo "💾 מבצע גיבוי לפני עדכון..."
./scripts/backup_system.sh

# עצור את המערכת
echo "🛑 עוצר את המערכת..."
./stop_enterprise.sh

# עדכן קוד
echo "📥 מוריד עדכונים..."
if [[ "$VERSION" == "latest" ]]; then
    git pull origin main
else
    git fetch --tags
    git checkout "v$VERSION"
fi

# עדכן dependencies
echo "📦 מעדכן dependencies..."
cargo update

# בנה מחדש
echo "🔨 בונה מחדש..."
cargo build --release

# עדכן Docker images אם קיימים
if [[ -f "config/enterprise/docker-compose.ollama.yml" ]]; then
    echo "🐳 מעדכן Docker images..."
    docker-compose -f config/enterprise/docker-compose.ollama.yml pull
fi

echo ""
echo "✅ עדכון הושלם!"
echo "🚀 להפעלת המערכת: ./start_enterprise.sh"
echo "📊 גרסה חדשה: $(./target/release/calclaw --version 2>/dev/null || echo "unknown")"
EOF

chmod +x update_enterprise.sh

echo "   ✅ סקריפטי utilities נוצרו:"
echo "      💾 scripts/backup_system.sh - גיבוי מערכת"
echo "      🔄 scripts/restore_backup.sh - שחזור מגיבוי"
echo "      🔄 update_enterprise.sh - עדכון מערכת"

# סיכום התקנה
echo ""
echo "🎉 התקנה ארגונית הושלמה!"
echo ""
echo "📋 סיכום ההתקנה:"
echo "================="

echo "✅ קונפיגורציה:"
echo "   📁 תיקייה: config/enterprise/"
echo "   📄 קובץ ראשי: install_config.json"
echo "   🔒 אבטחה: $SECURITY_LEVEL config"
echo "   🔗 אינטגרציות: ${INTEGRATIONS:-none}"

echo ""
echo "✅ סקריפטי ניהול:"
echo "   🚀 ./start_enterprise.sh - הפעלה"
echo "   🛑 ./stop_enterprise.sh - עצירה"
echo "   🧪 ./test_enterprise.sh - בדיקות"
echo "   💾 scripts/backup_system.sh - גיבוי"
echo "   🔄 scripts/restore_backup.sh - שחזור"
echo "   🔄 ./update_enterprise.sh - עדכון"

echo ""
echo "✅ תיעוד:"
echo "   📖 ENTERPRISE_GUIDE.md - מדריך ארגוני מלא"
echo "   📚 תיעוד מקוון: https://docs.calclaw.com/enterprise"

echo ""
echo "🚀 שלבים הבאים:"
echo "   1. ערוך קבצי קונפיגורציה ב-config/enterprise/"
echo "   2. הגדר tokens ו-secrets"
echo "   3. הרץ בדיקות: ./test_enterprise.sh"
echo "   4. הפעל את המערכת: ./start_enterprise.sh"
echo "   5. פתח בדפדפן: http://localhost:3000/enterprise"

echo ""
echo "📞 תמיכה ארגונית:"
echo "   📧 Email: enterprise-support@calclaw.com"
echo "   📞 Phone: 1-800-CALCLAW"
echo "   💬 Slack: enterprise-support.calclaw.com"

echo ""
echo "🏢 Calclaw Enterprise מוכן לפריסה! 🎉"

# שמור מידע נוסף אם יש פרטי ארגון
if [[ ! -z "$ORG_NAME" ]]; then
    cat > deployment_info.txt << EOF
========================================
Calclaw Enterprise - Deployment Information
========================================

Organization: $ORG_NAME
Contact: $CONTACT_EMAIL
Phone: ${CONTACT_PHONE:-Not specified}
Date: $(date)

Installation Details:
- Users: $USERS
- Environment: $ENVIRONMENT  
- Security: $SECURITY_LEVEL
- Integrations: ${INTEGRATIONS:-none}

Next Steps:
1. Configure files in config/enterprise/
2. Set up tokens and secrets
3. Run tests: ./test_enterprise.sh
4. Start system: ./start_enterprise.sh
5. Access dashboard: http://localhost:3000/enterprise

Support:
- Email: enterprise-support@calclaw.com
- Phone: 1-800-CALCLAW
- Documentation: https://docs.calclaw.com/enterprise

Backup Location: /backup/calclaw/
Configuration: config/enterprise/

========================================
EOF

    echo ""
    echo "📄 מידע פריסה נשמר: deployment_info.txt"
fi