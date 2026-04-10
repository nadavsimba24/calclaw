# 🛡️ Calclaw Enterprise Security Module

## 📖 מבוא

מודול אבטחה מתקדם ל-Calclaw Enterprise Edition, בהשראת הארכיטקטורה של NemoClaw מ-NVIDIA. מספק שכבת אבטחה מלאה עם sandboxing, בקרת גישה, audit logging, ו-compliance.

## 🏗️ ארכיטקטורה

```
Calclaw Security Module
├── 🎯 Security Policies
│   ├── Network Policy (Egress/Ingress rules)
│   ├── Filesystem Policy (Read/Write/Blocked paths)
│   ├── Process Policy (Capabilities, seccomp)
│   ├── Inference Policy (Model access, rate limits)
│   └── Audit Policy (Logging, SIEM integration)
├── 🔒 Access Control
│   ├── Network Access Checks
│   ├── Filesystem Access Checks
│   ├── Inference Access Checks
│   └── Approval Workflow
├── 📊 Audit & Compliance
│   ├── Audit Event Logging
│   ├── Compliance Reports (GDPR, HIPAA, SOC2)
│   ├── SIEM Integration
│   └── Real-time Alerts
├── 🤝 Approval System
│   ├── Request Creation
│   ├── Approval Workflow
│   ├── Time-based Expiry
│   └── Audit Trail
└── 🌐 REST API
    ├── Policy Management
    ├── Access Checks
    ├── Audit Queries
    └── Compliance Reports
```

## 🚀 התחלה מהירה

### הוספת התלות
```toml
# Cargo.toml
[dependencies]
calclaw-security = { path = "./src/enterprise/security" }
```

### יצירת Security Manager
```rust
use calclaw_security::{SecurityManager, SecurityPolicy, default_policies};

// צור security manager חדש
let mut security_manager = SecurityManager::new();

// הוסף policy ברירת מחדל
let strict_policy = default_policies::strict_policy();
security_manager.add_policy(strict_policy.clone());

// הפעל את ה-policy
security_manager.set_active_policy(strict_policy.id).unwrap();
```

### בדיקת גישה לרשת
```rust
// בדוק אם מותר גישה ל-API חיצוני
match security_manager.check_network_access(
    "api.openai.com",
    "tcp",
    443,
    tenant_id,
) {
    Ok(_) => println!("✅ גישה מאושרת"),
    Err(err) => println!("❌ גישה נדחתה: {}", err),
}
```

## 📋 Security Policies

### Network Policy
```rust
let network_policy = NetworkPolicy {
    egress_rules: vec![
        EgressRule {
            destination: Destination::Host("localhost".to_string()),
            protocol: Protocol::Tcp,
            ports: vec![PortRange { start: 11434, end: 11434 }],
            action: RuleAction::Allow,
            require_approval: false,
            description: "Allow local Ollama".to_string(),
        },
        EgressRule {
            destination: Destination::Domain("*.openai.com".to_string()),
            protocol: Protocol::Tcp,
            ports: vec![PortRange { start: 443, end: 443 }],
            action: RuleAction::RequireApproval("External API access".to_string()),
            require_approval: true,
            description: "OpenAI API requires approval".to_string(),
        },
    ],
    // ... additional configuration
};
```

### Filesystem Policy
```rust
let filesystem_policy = FilesystemPolicy {
    read_only_paths: vec![
        FilesystemPath {
            path: "/etc".to_string(),
            recursive: true,
            description: "System configuration".to_string(),
        },
    ],
    writable_paths: vec![
        FilesystemPath {
            path: "/workspace".to_string(),
            recursive: true,
            description: "User workspace".to_string(),
        },
    ],
    blocked_paths: vec![
        FilesystemPath {
            path: "/root".to_string(),
            recursive: true,
            description: "Root directory".to_string(),
        },
    ],
    // ... additional configuration
};
```

### Inference Policy
```rust
let inference_policy = InferencePolicy {
    allowed_models: vec![
        ModelRule {
            provider: "ollama".to_string(),
            model_pattern: "gemma*".to_string(),
            max_context_length: Some(8192),
            require_approval: false,
        },
    ],
    blocked_models: vec![
        ModelRule {
            provider: "any".to_string(),
            model_pattern: "*large*".to_string(),
            max_context_length: None,
            require_approval: true,
        },
    ],
    max_tokens_per_request: 4096,
    // ... additional configuration
};
```

## 🌐 REST API Endpoints

### Policy Management
- `GET /api/security/policies` - קבל כל ה-policies
- `POST /api/security/policies` - צור policy חדש
- `GET /api/security/policies/:id` - קבל policy ספציפי
- `PUT /api/security/policies/:id` - עדכן policy
- `DELETE /api/security/policies/:id` - מחק policy
- `POST /api/security/policies/:id/activate` - הפעל policy

### Access Checks
- `POST /api/security/check/network` - בדוק גישה לרשת
- `POST /api/security/check/filesystem` - בדוק גישה למערכת קבצים
- `POST /api/security/check/inference` - בדוק גישה למודלים

### Audit & Compliance
- `GET /api/security/audit` - קבל audit logs
- `GET /api/security/audit/export` - ייצא audit logs
- `GET /api/security/audit/stats` - סטטיסטיקות audit
- `GET /api/security/compliance/gdpr` - דוח GDPR compliance
- `GET /api/security/compliance/hipaa` - דוח HIPAA compliance
- `GET /api/security/compliance/soc2` - דוח SOC2 compliance

### Approval System
- `GET /api/security/approvals` - קבל כל הבקשות לאישור
- `GET /api/security/approvals/pending` - קבל בקשות ממתינות
- `POST /api/security/approvals` - צור בקשה לאישור
- `POST /api/security/approvals/:id/approve` - אשר בקשה
- `POST /api/security/approvals/:id/deny` - דחה בקשה

## 🔒 Security Features

### Network Isolation
- **Egress control** - בקרת יציאות לרשת
- **Ingress control** - בקרת כניסות לרשת
- **DNS policy** - בקרת רזולוציית DNS
- **Rate limiting** - הגבלת קצב בקשות
- **Operator approval** - אישור ידני לגישה

### Filesystem Protection
- **Read-only paths** - נתיבים לקריאה בלבד
- **Writable paths** - נתיבים לכתיבה
- **Blocked paths** - נתיבים חסומים
- **Quota limits** - הגבלות אחסון
- **Backup policies** - מדיניות גיבוי

### Process Security
- **Capability dropping** - הסרת יכולות לינוקס
- **Seccomp profiles** - סינון קריאות מערכת
- **User namespace** - בידוד משתמשים
- **Process limits** - הגבלות תהליכים
- **Executable control** - בקרת הרצת קבצים

### Model Access Control
- **Provider-based rules** - כללים לפי ספק
- **Model patterns** - תבניות מודלים (wildcards)
- **Token limits** - הגבלות טוקנים
- **Rate limits** - הגבלות קצב
- **Cost limits** - הגבלות עלות

## 📊 Compliance Frameworks

### GDPR (General Data Protection Regulation)
- **Data access logging** - רישום גישה לנתונים
- **Privacy rules** - כללי פרטיות
- **Audit trail** - מסלול audit
- **Right to erasure** - זכות למחיקה
- **Data protection by design** - הגנת נתונים בתכנון

### HIPAA (Health Insurance Portability and Accountability Act)
- **Encryption** - הצפנת נתונים
- **Access controls** - בקרות גישה
- **Audit controls** - בקרות audit
- **Transmission security** - אבטחת שידור
- **Workforce security** - אבטחת צוות

### SOC2 (Service Organization Control 2)
- **Security** - אבטחה
- **Availability** - זמינות
- **Processing integrity** - שלמות עיבוד
- **Confidentiality** - סודיות
- **Privacy** - פרטיות

## 🎯 Default Policies

### Strict Policy
- **Network**: רק localhost, אישור לכל דבר אחר
- **Filesystem**: רק /tmp ו-/workspace לכתיבה
- **Process**: capabilities מוגבלות, seccomp strict
- **Inference**: רק מודלים מקומיים, אישור למודלים חיצוניים

### Balanced Policy
- **Network**: APIs נפוצות מותרות, אישור ל-APIs פחות נפוצות
- **Filesystem**: יותר נתיבים לכתיבה, הגבלות הגיוניות
- **Process**: capabilities סבירות, seccomp מאוזן
- **Inference**: מודלים נפוצים מותרים, אישור למודלים גדולים

### Permissive Policy (Development)
- **Network**: כל הגישה מותרת
- **Filesystem**: כל הגישה מותרת
- **Process**: מינימום הגבלות
- **Inference**: כל המודלים מותרים

## 🔧 Integration עם Calclaw

### עם Orchestration Engine
```rust
// לפני ביצוע task, בדוק גישה
let security_manager = get_security_manager();
match security_manager.check_inference_access(
    task.provider,
    task.model,
    task.estimated_tokens,
    task.tenant_id,
) {
    Ok(_) => execute_task(task),
    Err(err) => log_denied_access(task, err),
}
```

### עם Organizational Ontology
```rust
// התאם security policy ל-organizational structure
fn create_policy_for_organization(ontology: &OrganizationOntology) -> SecurityPolicy {
    let mut policy = default_policies::balanced_policy();
    
    // התאם network rules לפי מערכות הארגון
    for system in &ontology.systems {
        policy.network_policy.egress_rules.push(
            create_rule_for_system(system)
        );
    }
    
    // התאם filesystem rules לפי data entities
    for data_entity in &ontology.data_entities {
        policy.filesystem_policy.writable_paths.push(
            create_path_for_data_entity(data_entity)
        );
    }
    
    policy
}
```

### עם Task Management
```rust
// הוסף security checks ל-task execution pipeline
struct SecureTaskExecutor {
    security_manager: SecurityManager,
    task_executor: TaskExecutor,
}

impl SecureTaskExecutor {
    async fn execute_task(&self, task: Task) -> Result<TaskResult, String> {
        // בדוק גישה לפני ביצוע
        self.security_manager.check_access(&task)?;
        
        // בצע את ה-task
        self.task_executor.execute_task(task).await
    }
}
```

## 🚀 Deployment

### Single-Node Deployment
```bash
# הפעל את שרת האבטחה
cargo run --bin calclaw-security-server
```

### Kubernetes Deployment
```yaml
# calclaw-security.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: calclaw-security
spec:
  replicas: 3
  template:
    spec:
      containers:
      - name: security
        image: calclaw/security:latest
        ports:
        - containerPort: 8080
        env:
        - name: SECURITY_MODE
          value: "enterprise"
```

### Docker Compose
```yaml
# docker-compose.security.yml
version: '3.8'
services:
  calclaw-security:
    image: calclaw/security:latest
    ports:
      - "8080:8080"
    volumes:
      - ./security-policies:/policies
      - ./audit-logs:/logs
    environment:
      - SECURITY_POLICY_PATH=/policies/default.yaml
      - AUDIT_LOG_PATH=/logs/audit.log
```

## 📈 Monitoring & Alerting

### Prometheus Metrics
```rust
// Expose security metrics
let metrics = SecurityMetrics {
    access_checks_total: Counter::new("security_access_checks_total", "Total access checks"),
    access_denied_total: Counter::new("security_access_denied_total", "Total access denials"),
    approval_requests_total: Counter::new("security_approval_requests_total", "Total approval requests"),
    audit_events_total: Counter::new("security_audit_events_total", "Total audit events"),
};
```

### Alert Rules
```yaml
# prometheus/alerts.yaml
groups:
  - name: security
    rules:
      - alert: HighAccessDenialRate
        expr: rate(security_access_denied_total[5m]) > 0.1
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "High access denial rate detected"
          description: "More than 10% of access checks are being denied"
      
      - alert: PendingApprovalBacklog
        expr: security_approval_requests_pending > 10
        for: 10m
        labels:
          severity: warning
        annotations:
          summary: "Approval request backlog detected"
          description: "More than 10 approval requests are pending"
```

## 🎯 Best Practices

### Policy Design
1. **Principle of least privilege** - הענק רק את ההרשאות הנדרשות
2. **Default deny** - דחה גישה כברירת מחדל
3. **Explicit approval** - דרוש אישור מפורש לגישה רגישה
4. **Regular review** - סקור policies באופן קבוע
5. **Version control** - שמור policies ב-version control

### Access Control
1. **Role-based access** - הרשאות לפי תפקידים
2. **Attribute-based access** - הרשאות לפי מאפיינים
3. **Time-based access** - הרשאות לפי זמן
4. **Location-based access** - הרשאות לפי מיקום
5. **Multi-factor authentication** - אימות מרובה גורמים

### Audit & Compliance
1. **Comprehensive logging** - רישום מלא של כל האירועים
2. **Tamper-proof logs** - logs מוגנים מפני שינוי
3. **Regular compliance checks** - בדיקות compliance תקופתיות
4. **Automated reporting** - דוחות אוטומטיים
5. **Incident response** - תגובה לאירועי אבטחה

## 🔗 קישורים

- **GitHub Repository**: https://github.com/nadavsimba24/calclaw
- **Security Documentation**: https://calclaw.ai/docs/security
- **Compliance Guide**: https://calclaw.ai/docs/compliance
- **API Reference**: https://calclaw.ai/docs/api/security

## 🎉 סיכום

**Calclaw Enterprise Security Module** מספק שכבת אבטחה מתקדמת לארגונים, עם בקרת גישה מלאה, audit logging, ו-compliance עם תקנים בינלאומיים.

**המערכת מוכנה להגן על הארגון שלך!** 🛡️