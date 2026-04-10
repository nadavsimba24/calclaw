// 🛡️ Calclaw Enterprise Security Server
// REST API server for security management

use axum::Router;
use std::sync::Arc;
use tokio::sync::RwLock;

use calclaw_security::{SecurityManager, SecurityAppState, create_security_api};

#[tokio::main]
async fn main() {
    println!("🛡️ Calclaw Enterprise Security Server");
    println!("======================================");
    println!("");
    println!("🚀 מאתחל מערכת אבטחה עם:");
    println!("   • Security Policies מלאות");
    println!("   • Access Control עם approval workflow");
    println!("   • Audit & Compliance (GDPR, HIPAA, SOC2)");
    println!("   • REST API מלא");
    println!("");
    
    // צור security manager
    let security_manager = SecurityManager::new();
    
    // הוסף policies ברירת מחדל
    let strict_policy = default_policies::strict_policy();
    security_manager.add_policy(strict_policy.clone());
    security_manager.set_active_policy(strict_policy.id).unwrap();
    
    println!("✅ Security manager אותחל");
    println!("   • Policy: {}", strict_policy.name);
    println!("   • Description: {}", strict_policy.description);
    println!("");
    
    // צור את ה-app state
    let app_state = SecurityAppState {
        security_manager: Arc::new(RwLock::new(security_manager)),
    };
    
    // צור את ה-API router
    let app = Router::new()
        .nest("/api", create_security_api())
        .with_state(app_state);
    
    println!("🌐 שרת API מוכן!");
    println!("   • Base URL: http://localhost:8081/api/security");
    println!("   • Health check: http://localhost:8081/api/security/health-check");
    println!("   • Status: http://localhost:8081/api/security/status");
    println!("");
    println("📚 Endpoints זמינים:");
    println!("   • Policy management: /api/security/policies");
    println!("   • Access checks: /api/security/check/*");
    println!("   • Audit logs: /api/security/audit");
    println!("   • Compliance reports: /api/security/compliance/*");
    println!("   • Approval requests: /api/security/approvals");
    println!("");
    println!("💡 דוגמאות לשימוש:");
    println!("   curl http://localhost:8081/api/security/status");
    println!("   curl http://localhost:8081/api/security/policies");
    println!("   curl -X POST http://localhost:8081/api/security/check/network \\");
    println!("     -H 'Content-Type: application/json' \\");
    println!("     -d '{\"destination\":\"api.openai.com\",\"port\":443}'");
    println!("");
    
    // הפעל את השרת
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8081").await.unwrap();
    println!("🎯 שרת אבטחה רץ על: http://localhost:8081");
    axum::serve(listener, app).await.unwrap();
}
