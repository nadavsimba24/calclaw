#!/bin/bash

# 🏢 Calclaw - התקנה ארגונית
# גרסה מתקדמת עם בקרות אבטחה, multi-user, ואינטגרציות

set -e

echo "🏢 Calclaw - התקנה ארגונית"
echo "==========================="
echo ""
echo "התקנה מתקדמת לארגונים עם דרישות אבטחה, multi-user, ואינטגרציות."
echo ""

# פרמטרים ברירת מחדל
USERS=10
ENVIRONMENT="server"
SECURITY_LEVEL="medium"
INTEGRATIONS=""
BUDGET="medium"
RUN_QUESTIONNAIRE=false

# פרס פרמטרים
while [[ $# -gt 0 ]]; do
    case $1 in
        --users=*)
            USERS="${1#*=}"
            shift
            ;;
        --environment=*)
            ENVIRONMENT="${1#*=}"
            shift
            ;;
        --security=*)
            SECURITY_LEVEL="${1#*=}"
            shift
            ;;
        --integrations=*)
            INTEGRATIONS="${1#*=}"
            shift
            ;;
        --budget=*)
            BUDGET="${1#*=}"
            shift
            ;;
        --questionnaire)
            RUN_QUESTIONNAIRE=true
            shift
            ;;
        --help)
            echo "שימוש: $0 [אפשרויות]"
            echo ""
            echo "אפשרויות:"
            echo "  --users=NUMBER         מספר משתמשים משוער (ברירת מחדל: 10)"
            echo "  --environment=TYPE     סביבת הרצה: local/server/cloud/k8s (ברירת מחדל: server)"
            echo "  --security=LEVEL       רמת אבטחה: low/medium/high (ברירת מחדל: medium)"
            echo "  --integrations=LIST    אינטגרציות (מופרדות בפסיק): slack,github,etc"
            echo "  --budget=LEVEL         תקציב: low/medium/high (ברירת מחדל: medium)"
            echo "  --questionnaire        הרץ שאלון התאמה אינטראקטיבי"
            echo "  --help                 הצג עזרה זו"
            echo ""
            echo "דוגמאות:"
            echo "  $0 --questionnaire"
            echo "  $0 --users=50 --environment=k8s --security=high"
            echo "  $0 --users=20 --environment=cloud --integrations=slack,github"
            exit 0
            ;;
        *)
            echo "❌ אופציה לא ידועה: $1"
            echo "   השתמש ב --help להצגת עזרה"
            exit 1
            ;;
    esac
done

# שאלון התאמה אינטראקטיבי
if [[ "$RUN_QUESTIONNAIRE" == true ]]; then
    echo "📝 שאלון התאמה ארגונית"
    echo "======================"
    echo ""
    
    read -p "מספר משתמשים משוער: " USERS
    read -p "סביבת הרצה (local/server/cloud/k8s): " ENVIRONMENT
    read -p "רמת אבטחה (low/medium/high): " SECURITY_LEVEL
    
    echo ""
    echo "🔗 אינטגרציות אפשריות:"
    echo "   • slack - אינטגרציה עם Slack"
    echo "   • github - אינטגרציה עם GitHub/GitLab"
    echo "   • docker - ניהול Docker containers"
    echo "   • kubernetes - ניהול Kubernetes"
    echo "   • saas - אינטגרציה עם SaaS אחרים"
    echo "   • custom - מערכות פנימיות מותאמות"
    echo ""
    read -p "אינטגרציות נדרשות (מופרדות בפסיק, או none): " INTEGRATIONS_INPUT
    
    if [[ "$INTEGRATIONS_INPUT" != "none" && ! -z "$INTEGRATIONS_INPUT" ]]; then
        INTEGRATIONS="$INTEGRATIONS_INPUT"
    fi
    
    read -p "תקציב/משאבים (low/medium/high): " BUDGET
    
    echo ""
    echo "📧 פרטי קשר לפרויקט ארגוני:"
    read -p "שם הארגון: " ORG_NAME
    read -p "אימייל איש קשר: " CONTACT_EMAIL
    read -p "טלפון (אופציונלי): " CONTACT_PHONE
fi

# הצג סיכום ההגדרות
echo ""
echo "⚙️  הגדרות התקנה:"
echo "================="
echo "   👥 משתמשים: $USERS"
echo "   🖥️  סביבה: $ENVIRONMENT"
echo "   🔒 אבטחה: $SECURITY_LEVEL"
echo "   🔗 אינטגרציות: ${INTEGRATIONS:-none}"
echo "   💰 תקציב: $BUDGET"

if [[ ! -z "$ORG_NAME" ]]; then
    echo "   🏢 ארגון: $ORG_NAME"
    echo "   📧 איש קשר: $CONTACT_EMAIL"
fi

echo ""
read -p "להמשיך בהתקנה? (y/n): " CONFIRM
if [[ "$CONFIRM" != "y" && "$CONFIRM" != "Y" ]]; then
    echo "❌ התקנה בוטלה"
    exit 0
fi

# התאם את ההתקנה לפי הפרמטרים
echo ""
echo "🔧 מכין התקנה מותאמת..."

# צור תיקיית קונפיגורציה
CONFIG_DIR="config/enterprise"
mkdir -p "$CONFIG_DIR"

# צור קובץ קונפיגורציה ראשי
cat > "$CONFIG_DIR/install_config.json" << EOF
{
  "install_type": "enterprise",
  "users": $USERS,
  "environment": "$ENVIRONMENT",
  "security_level": "$SECURITY_LEVEL",
  "integrations": "$INTEGRATIONS",
  "budget": "$BUDGET",
  "organization": {
    "name": "${ORG_NAME:-Not specified}",
    "contact_email": "${CONTACT_EMAIL:-Not specified}",
    "contact_phone": "${CONTACT_PHONE:-Not specified}"
  },
  "timestamp": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
  "version": "1.0.0"
}
EOF

echo "✅ קובץ קונפיגורציה נוצר: $CONFIG_DIR/install_config.json"

# שלב 1: התקן תלויות לפי סביבה
echo ""
echo "1. 📦 מתקין תלויות מערכת..."

case $ENVIRONMENT in
    "local"|"server")
        echo "   🖥️  סביבה מקומית/שרת - התקנת תלויות בסיסיות"
        
        # בדוק אם Docker מותקן
        if ! command -v docker &> /dev/null; then
            echo "   🐳 מתקין Docker..."
            if [[ -f /etc/os-release ]]; then
                . /etc/os-release
                if [[ "$ID" == "ubuntu" || "$ID" == "debian" ]]; then
                    sudo apt update
                    sudo apt install -y docker.io docker-compose
                    sudo usermod -aG docker $USER
                elif [[ "$ID" == "centos" || "$ID" == "rhel" ]]; then
                    sudo yum install -y docker
                    sudo systemctl start docker
                    sudo systemctl enable docker
                    sudo usermod -aG docker $USER
                fi
            fi
        else
            echo "   ✅ Docker כבר מותקן: $(docker --version)"
        fi
        
        # בדוק אם Docker Compose מותקן
        if ! command -v docker-compose &> /dev/null; then
            echo "   🐳 מתקין Docker Compose..."
            sudo curl -L "https://github.com/docker/compose/releases/latest/download/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
            sudo chmod +x /usr/local/bin/docker-compose
        else
            echo "   ✅ Docker Compose כבר מותקן: $(docker-compose --version)"
        fi
        ;;
        
    "cloud")
        echo "   ☁️  סביבת ענן - מכין התקנה ל-AWS/Azure/GCP"
        
        # צור קובץ קונפיגורציה ל-Cloud
        cat > "$CONFIG_DIR/cloud_config.json" << EOF
{
  "provider": "aws",
  "region": "us-east-1",
  "instance_type": "t3.medium",
  "storage_gb": 50,
  "vpc": {
    "create_new": true,
    "cidr": "10.0.0.0/16"
  },
  "security_groups": [
    {
      "name": "calclaw-http",
      "ports": [3000, 80, 443]
    },
    {
      "name": "calclaw-ssh",
      "ports": [22]
    }
  ],
  "backup": {
    "enabled": true,
    "frequency": "daily",
    "retention_days": 30
  },
  "monitoring": {
    "enabled": true,
    "alarms": ["cpu", "memory", "disk"]
  }
}
EOF
        
        echo "   ✅ קובץ קונפיגורציית ענן נוצר"
        ;;
        
    "k8s")
        echo "   🐳 סביבת Kubernetes - מכין manifests"
        
        # צור קובץ קונפיגורציה ל-Kubernetes
        cat > "$CONFIG_DIR/kubernetes_config.yaml" << EOF
apiVersion: v1
kind: Namespace
metadata:
  name: calclaw-enterprise
  labels:
    app: calclaw
    environment: production
---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: calclaw
  namespace: calclaw-enterprise
spec:
  replicas: 3
  selector:
    matchLabels:
      app: calclaw
  template:
    metadata:
      labels:
        app: calclaw
    spec:
      containers:
      - name: calclaw
        image: calclaw/enterprise:latest
        ports:
        - containerPort: 3000
        env:
        - name: USERS
          value: "$USERS"
        - name: SECURITY_LEVEL
          value: "$SECURITY_LEVEL"
        resources:
          requests:
            memory: "512Mi"
            cpu: "250m"
          limits:
            memory: "2Gi"
            cpu: "1"
---
apiVersion: v1
kind: Service
metadata:
  name: calclaw
  namespace: calclaw-enterprise
spec:
  selector:
    app: calclaw
  ports:
  - port: 3000
    targetPort: 3000
  type: LoadBalancer
---
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: calclaw-network-policy
  namespace: calclaw-enterprise
spec:
  podSelector:
    matchLabels:
      app: calclaw
  policyTypes:
  - Ingress
  - Egress
  ingress:
  - from:
    - ipBlock:
        cidr: 0.0.0.0/0
    ports:
    - protocol: TCP
      port: 3000
  egress:
  - to:
    - ipBlock:
        cidr: 0.0.0.0/0
    ports:
    - protocol: TCP
      port: 443
EOF
        
        echo "   ✅ Kubernetes manifests נוצרו"
        ;;
esac

# שלב 2: התקן Ollama ארגוני
echo ""
echo "2. 🤖 מתקין Ollama ארגוני..."

if [[ "$ENVIRONMENT" == "k8s" ]]; then
    # Ollama ב-Kubernetes
    cat > "$CONFIG_DIR/ollama_k8s.yaml" << EOF
apiVersion: apps/v1
kind: Deployment
metadata:
  name: ollama
  namespace: calclaw-enterprise
spec:
  replicas: 2
  selector:
    matchLabels:
      app: ollama
  template:
    metadata:
      labels:
        app: ollama
    spec:
      containers:
      - name: ollama
        image: ollama/ollama:latest
        ports:
        - containerPort: 11434
        volumeMounts:
        - name: models
          mountPath: /root/.ollama
        resources:
          requests:
            memory: "4Gi"
            cpu: "2"
          limits:
            memory: "8Gi"
            cpu: "4"
      volumes:
      - name: models
        persistentVolumeClaim:
          claimName: ollama-models-pvc
---
apiVersion: v1
kind: Service
metadata:
  name: ollama
  namespace: calclaw-enterprise
spec:
  selector:
    app: ollama
  ports:
  - port: 11434
    targetPort: 11434
---
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: ollama-models-pvc
  namespace: calclaw-enterprise
spec:
  accessModes:
    - ReadWriteOnce
  resources:
    requests:
      storage: 50Gi
EOF
    
    echo "   ✅ קובץ Ollama ל-Kubernetes נוצר"
else
    # Ollama עם Docker Compose
    cat > "$CONFIG_DIR/docker-compose.ollama.yml" << EOF
version: '3.8'

services:
  ollama:
    image: ollama/ollama:latest
    ports:
      - "11434:11434"
    volumes:
      - ollama_models:/root/.ollama
    environment:
      - OLLAMA_KEEP_ALIVE=24h
      - OLLAMA_HOST=0.0.0.0
    deploy:
      resources:
        limits:
          memory: 8G
        reservations:
          memory: 4G
    restart: unless-stopped

volumes:
  ollama_models:
    driver: local
EOF
    
    echo "   ✅ קובץ Docker Compose ל-Ollama נוצר"
fi

# שלב 3: התקן בקרות אבטחה לפי רמה
echo ""
echo "3. 🔒 מתקין בקרות אבטחה ($SECURITY_LEVEL)..."

case $SECURITY_LEVEL in
    "low")
        echo "   🟢 אבטחה בסיסית - הגבלות מינימליות"
        ;;
    "medium")
        echo "   🟡 אבטחה בינונית - אישורים לפעולות מסוכנות"
        
        # צור קובץ MCP Proxy עם אישורים
        cat > "$CONFIG_DIR/mcp_proxy_config.json" << EOF
{
  "requires_approval": true,
  "approval_channels": ["slack", "email"],
  "risk_levels": {
    "low": {
      "requires_approval": false,
      "tools": ["read_file", "list_directory", "get_status"]
    },
    "medium": {
      "requires_approval": true,
      "timeout": 300,
      "tools": ["write_file", "create_issue", "send_message"]
    },
    "high": {
      "requires_approval": true,
      "requires_multiple_approvers": true,
      "timeout": 600,
      "tools": ["execute_command", "delete_file", "merge_pr"]
    }
  },
  "audit_logging": {
    "enabled": true,
    "retention_days": 90,
    "alert_on_suspicious": true
  }
}
EOF
        
        echo "   ✅ קובץ MCP Proxy עם אישורים נוצר"
        ;;
    "high")
        echo "   🔴 אבטחה מתקדמת - network policies, audit logs, compliance"
        
        # צור קובץ אבטחה מתקדמת
        cat > "$CONFIG_DIR/security_advanced.json" << EOF
{
  "network_policies": {
    "enabled": true,
    "egress_rules": [
      {
        "destination": "slack.com",
        "ports": [443],
        "protocol": "TCP"
      },
      {
        "destination": "github.com",
        "ports": [443],
        "protocol": "TCP"
      },
      {
        "destination": "docker.io",
        "ports": [443],
        "protocol": "TCP"
      }
    ],
    "deny_all_other": true
  },
  "authentication": {
    "oauth2": true,
    "multi_factor": false,
    "session_timeout": 3600
  },
  "encryption": {
    "data_at_rest": true,
    "data_in_transit": true,
    "key_rotation_days": 90
  },
  "compliance": {
    "gdpr": true,
    "hipaa": false,
    "soc2": false
  },
  "monitoring": {
    "siem_integration": true,
    "alerting": {
      "suspicious_activity": true,
      "failed_logins": true,
      "resource_abuse": true
    }
  }
}
EOF
        
        echo "   ✅ קובץ