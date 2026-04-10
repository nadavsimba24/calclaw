# 🚀 Calclaw Enterprise Security - Quick Start

## 📖 מבוא

Calclaw Enterprise Security Module מספק שכבת אבטחה מתקדמת עם:
- 🎯 Security Policies מלאות
- 🔒 Access Control עם approval workflow  
- 📊 Audit & Compliance (GDPR, HIPAA, SOC2)
- 🌐 REST API מלא
- 🤖 Integration עם Calclaw הקיים

## 🚀 התחלה מהירה

### 1. הפעלת שרת האבטחה
```bash
# בנה והפעל
cargo run --bin calclaw-security-server

# או הרץ ישירות
./target/release/calclaw-security-server
```

### 2. בדיקת החיבור
```bash
# בדוק סטטוס
curl http://localhost:8081/api/security/status

# בדוק health
curl http://localhost:8081/api/security/health-check

# קבל policies
curl http://localhost:8081/api/security/policies
```

### 3. בדיקת גישה
```bash
# בדוק גישה לרשת
curl -X POST http://localhost:8081/api/security/check/network \
  -H "Content-Type: application/json" \
  -d '{
    "destination": "api.openai.com",
    "protocol": "tcp",
    "port": 443,
    "tenant_id": "00000000-0000-0000-0000-000000000000"
  }'

# בדוק גישה למערכת קבצים
curl -X POST http://localhost:8081/api/security/check/filesystem \
  -H "Content-Type: application/json" \
  -d '{
    "path": "/workspace/project",
    "operation": "write",
    "tenant_id": "00000000-0000-0000-0000-000000000000"
  }'

# בדוק גישה למודל
curl -X POST http://localhost:8081/api/security/check/inference \
  -H "Content-Type: application/json" \
  -d '{
    "provider": "openai",
    "model": "gpt-4",
    "tokens": 1000,
    "tenant_id": "00000000-0000-0000-0000-000000000000"
  }'
```

### 4. ניהול Policies
```bash
# קבל policy ספציפית
curl http://localhost:8081/api/security/policies/{policy_id}

# צור policy חדשה
curl -X POST http://localhost:8081/api/security/policies \
  -H "Content-Type: application/json" \
  -d @config/security/policies/default.yaml

# הפעל policy
curl -X POST http://localhost:8081/api/security/policies/{policy_id}/activate
```

### 5. Audit & Compliance
```bash
# קבל audit logs
curl http://localhost:8081/api/security/audit

# קבל סטטיסטיקות
curl http://localhost:8081/api/security/audit/stats

# קבל דוח GDPR compliance
curl http://localhost:8081/api/security/compliance/gdpr

# קבל דוח HIPAA compliance  
curl http://localhost:8081/api/security/compliance/hipaa

# קבל דוח SOC2 compliance
curl http://localhost:8081/api/security/compliance/soc2
```

### 6. Approval System
```bash
# קבל בקשות ממתינות
curl http://localhost:8081/api/security/approvals/pending

# צור בקשה לאישור
curl -X POST http://localhost:8081/api/security/approvals \
  -H "Content-Type: application/json" \
  -d '{
    "request_type": "network_access",
    "requester_id": "00000000-0000-0000-0000-000000000000",
    "tenant_id": "00000000-0000-0000-0000-000000000000",
    "resource": "api.openai.com:443",
    "justification": "Need access for data analysis",
    "timeout_hours": 24
  }'

# אשר בקשה
curl -X POST http://localhost:8081/api/security/approvals/{request_id}/approve \
  -H "Content-Type: application/json" \
  -d '{
    "approver_id": "00000000-0000-0000-0000-000000000000"
  }'

# דחה בקשה
curl -X POST http://localhost:8081/api/security/approvals/{request_id}/deny \
  -H "Content-Type: application/json" \
  -d '{
    "approver_id": "00000000-0000-0000-0000-000000000000",
    "reason": "Access not justified"
  }'
```

## 🔗 Integration עם Calclaw

### עם Orchestration Engine
```rust
use calclaw_security::{SecurityManager, SecurityPolicy};

// צור security manager
let mut security_manager = SecurityManager::new();

// הוסף policy
let policy = SecurityPolicy::from_yaml_file("config/security/policies/default.yaml")?;
security_manager.add_policy(policy);

// בדוק גישה לפני ביצוע task
match security_manager.check_inference_access(
    task.provider,
    task.model,
    task.estimated_tokens,
    task.tenant_id,
) {
    Ok(_) => execute_task(task),
    Err(err) => {
        log::warn!("Access denied: {}", err);
        // צור בקשה לאישור
        security_manager.create_approval_request(...);
    }
}
```

### עם Organizational Ontology
```rust
// צור policy מותאמת לארגון
fn create_organization_policy(ontology: &OrganizationOntology) -> SecurityPolicy {
    let mut policy = SecurityPolicy::default();
    
    // התאם network rules למערכות הארגון
    for system in &ontology.systems {
        policy.network_policy.add_rule_for_system(system);
    }
    
    // התאם filesystem rules ל-data entities
    for data_entity in &ontology.data_entities {
        policy.filesystem_policy.add_path_for_data_entity(data_entity);
    }
    
    policy
}
```

## 🎯 Default Policies

### Strict Policy
- רשת: רק localhost, אישור לכל דבר אחר
- מערכת קבצים: רק /tmp ו-/workspace לכתיבה
- תהליכים: capabilities מוגבלות, seccomp strict
- מודלים: רק מודלים מקומיים, אישור למודלים חיצוניים

### Balanced Policy (ברירת מחדל)
- רשת: APIs נפוצות מותרות, אישור ל-APIs פחות נפוצות
- מערכת קבצים: יותר נתיבים לכתיבה, הגבלות הגיוניות
- תהליכים: capabilities סבירות, seccomp מאוזן
- מודלים: מודלים נפוצים מותרים, אישור למודלים גדולים

### Permissive Policy (פיתוח)
- רשת: כל הגישה מותרת
- מערכת קבצים: כל הגישה מותרת
- תהליכים: מינימום הגבלות
- מודלים: כל המודלים מותרים

## 📊 Monitoring

### Prometheus Metrics
```bash
# Security metrics endpoint
curl http://localhost:8081/metrics
```

### Alert Rules
```yaml
# דוגמה ל-alert rules
- alert: HighAccessDenialRate
  expr: rate(security_access_denied_total[5m]) > 0.1
  for: 5m
  labels:
    severity: warning
  annotations:
    summary: "High access denial rate detected"
    description: "More than
