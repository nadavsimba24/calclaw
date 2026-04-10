// 🛡️ Calclaw Enterprise Security Library
// Complete security foundation for enterprise deployments

pub mod manager;
pub mod policies;

pub use manager::*;
pub use policies::*;

/// 🚀 Security API Module
pub mod api {
    use axum::{
        extract::{Path, State},
        http::StatusCode,
        response::IntoResponse,
        routing::{get, post, put, delete},
        Json, Router,
    };
    use serde_json::json;
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use uuid::Uuid;
    
    use super::*;
    
    /// 🏢 Security App State
    #[derive(Clone)]
    pub struct SecurityAppState {
        pub security_manager: Arc<RwLock<SecurityManager>>,
    }
    
    /// 🚀 Create security API router
    pub fn create_security_api() -> Router<SecurityAppState> {
        Router::new()
            // Policy management
            .route("/api/security/policies", get(get_policies))
            .route("/api/security/policies", post(create_policy))
            .route("/api/security/policies/:id", get(get_policy))
            .route("/api/security/policies/:id", put(update_policy))
            .route("/api/security/policies/:id", delete(delete_policy))
            .route("/api/security/policies/:id/activate", post(activate_policy))
            .route("/api/security/policies/default/strict", get(get_strict_policy))
            .route("/api/security/policies/default/balanced", get(get_balanced_policy))
            .route("/api/security/policies/default/permissive", get(get_permissive_policy))
            
            // Access checks
            .route("/api/security/check/network", post(check_network_access))
            .route("/api/security/check/filesystem", post(check_filesystem_access))
            .route("/api/security/check/inference", post(check_inference_access))
            
            // Audit logs
            .route("/api/security/audit", get(get_audit_logs))
            .route("/api/security/audit/export", get(export_audit_logs))
            .route("/api/security/audit/stats", get(get_audit_stats))
            
            // Approval requests
            .route("/api/security/approvals", get(get_approval_requests))
            .route("/api/security/approvals/pending", get(get_pending_approvals))
            .route("/api/security/approvals", post(create_approval_request))
            .route("/api/security/approvals/:id/approve", post(approve_request))
            .route("/api/security/approvals/:id/deny", post(deny_request))
            
            // Security status
            .route("/api/security/status", get(get_security_status))
            .route("/api/security/health", get(get_security_health))
            
            // Compliance
            .route("/api/security/compliance/gdpr", get(get_gdpr_compliance))
            .route("/api/security/compliance/hipaa", get(get_hipaa_compliance))
            .route("/api/security/compliance/soc2", get(get_soc2_compliance))
            
            // Health check
            .route("/api/security/health-check", get(health_check))
    }
    
    /// 🩺 Health check
    async fn health_check() -> impl IntoResponse {
        (StatusCode::OK, Json(json!({
            "status": "healthy",
            "service": "security",
            "timestamp": chrono::Utc::now().to_rfc3339()
        })))
    }
    
    /// 📋 Get all security policies
    async fn get_policies(
        State(state): State<SecurityAppState>,
    ) -> impl IntoResponse {
        let manager = state.security_manager.read().await;
        let policies: Vec<serde_json::Value> = manager.policies.values()
            .map(|policy| {
                json!({
                    "id": policy.id,
                    "name": policy.name,
                    "description": policy.description,
                    "enabled": policy.enabled,
                    "created_at": policy.created_at.to_rfc3339(),
                    "updated_at": policy.updated_at.to_rfc3339(),
                })
            })
            .collect();
        
        Json(json!({
            "policies": policies,
            "count": policies.len(),
            "active_policy": manager.active_policy_id,
        }))
    }
    
    /// ➕ Create a new security policy
    async fn create_policy(
        State(state): State<SecurityAppState>,
        Json(payload): Json<serde_json::Value>,
    ) -> impl IntoResponse {
        let mut manager = state.security_manager.write().await;
        
        // Parse the policy from JSON
        let policy: SecurityPolicy = match serde_json::from_value(payload) {
            Ok(policy) => policy,
            Err(err) => {
                return (StatusCode::BAD_REQUEST, Json(json!({
                    "error": format!("Invalid policy: {}", err)
                })))
            }
        };
        
        manager.add_policy(policy.clone());
        
        Json(json!({
            "status": "created",
            "policy_id": policy.id,
            "policy_name": policy.name,
        }))
    }
    
    /// 👁️ Get a specific security policy
    async fn get_policy(
        State(state): State<SecurityAppState>,
        Path(policy_id): Path<Uuid>,
    ) -> impl IntoResponse {
        let manager = state.security_manager.read().await;
        
        match manager.policies.get(&policy_id) {
            Some(policy) => {
                Json(json!({
                    "policy": policy,
                    "is_active": manager.active_policy_id == Some(policy_id),
                }))
            }
            None => {
                (StatusCode::NOT_FOUND, Json(json!({
                    "error": "Policy not found"
                })))
            }
        }
    }
    
    /// ✏️ Update a security policy
    async fn update_policy(
        State(state): State<SecurityAppState>,
        Path(policy_id): Path<Uuid>,
        Json(payload): Json<serde_json::Value>,
    ) -> impl IntoResponse {
        let mut manager = state.security_manager.write().await;
        
        let policy = match manager.policies.get_mut(&policy_id) {
            Some(policy) => policy,
            None => {
                return (StatusCode::NOT_FOUND, Json(json!({
                    "error": "Policy not found"
                })))
            }
        };
        
        // Update policy fields
        if let Some(name) = payload.get("name").and_then(|v| v.as_str()) {
            policy.name = name.to_string();
        }
        
        if let Some(description) = payload.get("description").and_then(|v| v.as_str()) {
            policy.description = description.to_string();
        }
        
        if let Some(enabled) = payload.get("enabled").and_then(|v| v.as_bool()) {
            policy.enabled = enabled;
        }
        
        policy.updated_at = chrono::Utc::now();
        
        Json(json!({
            "status": "updated",
            "policy_id": policy_id,
            "updated_at": policy.updated_at.to_rfc3339(),
        }))
    }
    
    /// 🗑️ Delete a security policy
    async fn delete_policy(
        State(state): State<SecurityAppState>,
        Path(policy_id): Path<Uuid>,
    ) -> impl IntoResponse {
        let mut manager = state.security_manager.write().await;
        
        if manager.active_policy_id == Some(policy_id) {
            return (StatusCode::BAD_REQUEST, Json(json!({
                "error": "Cannot delete active policy"
            })));
        }
        
        if manager.policies.remove(&policy_id).is_some() {
            Json(json!({
                "status": "deleted",
                "policy_id": policy_id,
            }))
        } else {
            (StatusCode::NOT_FOUND, Json(json!({
                "error": "Policy not found"
            })))
        }
    }
    
    /// 🎯 Activate a security policy
    async fn activate_policy(
        State(state): State<SecurityAppState>,
        Path(policy_id): Path<Uuid>,
    ) -> impl IntoResponse {
        let mut manager = state.security_manager.write().await;
        
        match manager.set_active_policy(policy_id) {
            Ok(_) => {
                Json(json!({
                    "status": "activated",
                    "policy_id": policy_id,
                }))
            }
            Err(err) => {
                (StatusCode::BAD_REQUEST, Json(json!({
                    "error": err
                })))
            }
        }
    }
    
    /// 🛡️ Get strict default policy
    async fn get_strict_policy() -> impl IntoResponse {
        Json(json!({
            "policy": default_policies::strict_policy(),
            "description": "Maximum security with minimal access",
        }))
    }
    
    /// ⚖️ Get balanced default policy
    async fn get_balanced_policy() -> impl IntoResponse {
        // Similar to strict but with more allowances
        let mut policy = default_policies::strict_policy();
        policy.name = "Balanced Security Policy".to_string();
        policy.description = "Balanced security with reasonable access".to_string();
        
        // Add more network allowances
        policy.network_policy.egress_rules.push(EgressRule {
            id: Uuid::new_v4(),
            name: "Allow common APIs".to_string(),
            destination: Destination::Domain("api.*".to_string()),
            protocol: Protocol::Tcp,
            ports: vec![PortRange { start: 443, end: 443 }],
            action: RuleAction::Allow,
            require_approval: false,
            description: "Allow access to common APIs".to_string(),
        });
        
        Json(json!({
            "policy": policy,
            "description": "Balanced security with reasonable access",
        }))
    }
    
    /// 🔓 Get permissive default policy
    async fn get_permissive_policy() -> impl IntoResponse {
        // Very permissive policy for development
        let policy = SecurityPolicy {
            id: Uuid::new_v4(),
            name: "Permissive Security Policy".to_string(),
            description: "Development policy with maximum access".to_string(),
            network_policy: NetworkPolicy {
                egress_rules: vec![
                    EgressRule {
                        id: Uuid::new_v4(),
                        name: "Allow all outbound".to_string(),
                        destination: Destination::Any,
                        protocol: Protocol::Any,
                        ports: vec![],
                        action: RuleAction::Allow,
                        require_approval: false,
                        description: "Allow all outbound traffic".to_string(),
                    },
                ],
                ingress_rules: vec![],
                dns_policy: DnsPolicy {
                    allowed_domains: vec!["*".to_string()],
                    blocked_domains: vec![],
                    use_system_dns: true,
                    dns_servers: vec![],
                },
                require_approval: false,
                approval_timeout_seconds: 0,
                rate_limits: vec![],
            },
            filesystem_policy: FilesystemPolicy {
                read_only_paths: vec![],
                writable_paths: vec![
                    FilesystemPath {
                        path: "/".to_string(),
                        recursive: true,
                        description: "Full filesystem access".to_string(),
                    },
                ],
                blocked_paths: vec![],
                quota_limits: vec![],
                backup_policy: BackupPolicy {
                    enabled: false,
                    frequency: BackupFrequency::Daily,
                    retention_days: 0,
                    encryption: false,
                    compression: false,
                },
            },
            process_policy: ProcessPolicy {
                max_processes: 1000,
                max_threads: 5000,
                allowed_executables: vec!["*".to_string()],
                blocked_executables: vec![],
                user_namespace: false,
                capability_drops: vec![],
                seccomp_profile: SeccompProfile {
                    default_action: SeccompAction::Allow,
                    architectures: vec![SeccompArch::X86_64],
                    syscalls: vec![],
                },
            },
            inference_policy: InferencePolicy {
                allowed_models: vec![
                    ModelRule {
                        provider: "*".to_string(),
                        model_pattern: "*".to_string(),
                        max_context_length: None,
                        require_approval: false,
                    },
                ],
                blocked_models: vec![],
                max_tokens_per_request: 100000,
                rate_limits: vec![],
                cost_limits: vec![],
                privacy_rules: vec![],
            },
            audit_policy: AuditPolicy {
                enabled: true,
                retention_days: 7,
                events_to_log: vec![
                    AuditEventType::NetworkAccess,
                    AuditEventType::FilesystemAccess,
                    AuditEventType::ProcessExecution,
                    AuditEventType::InferenceRequest,
                ],
                siem_integration: SiemIntegration {
                    enabled: false,
                    siem_type: SiemType::Splunk,
                    endpoint: "".to_string(),
                    api_key: None,
                    format: LogFormat::Json,
                },
                real_time_alerts: false,
                encryption: false,
            },
            created_at: Utc::now(),
            updated_at: Utc::now(),
            enabled: true,
        };
        
        Json(json!({
            "policy": policy,
            "description": "Development policy with maximum access",
            "warning": "This policy provides minimal security - use only for development",
        }))
    }
    
    /// 🌐 Check network access
    async fn check_network_access(
        State(state): State<SecurityAppState>,
        Json(payload): Json<serde_json::Value>,
    ) -> impl IntoResponse {
        let manager = state.security_manager.read().await;
        
        let destination = payload.get("destination")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                (StatusCode::BAD_REQUEST, Json(json!({
                    "error": "Missing destination"
                })))
            })?;
        
        let protocol = payload.get("protocol")
            .and_then(|v| v.as_str())
            .unwrap_or("tcp");
        
        let port = payload.get("port")
            .and_then(|v| v.as_u64())
            .unwrap_or(443) as u16;
        
        let tenant_id = payload.get("tenant_id")
            .and_then(|v| v.as_str())
            .and_then(|s| Uuid::parse_str(s).ok())
            .unwrap_or_else(Uuid::new_v4);
        
        match manager.check_network_access(destination, protocol, port, tenant_id) {
            Ok(_) => {
                Json(json!({
                    "allowed": true,
                    "destination": destination,
                    "protocol": protocol,
                    "port": port,
                }))
            }
            Err(err) => {
                Json(json!({
                    "allowed": false,
                    "destination": destination,
                    "protocol": protocol,
                    "port": port,
                    "reason": err,
                }))
            }
        }
    }
    
    /// 📁 Check filesystem access
    async fn check_filesystem_access(
        State(state): State<SecurityAppState>,
        Json(payload): Json<serde_json::Value>,
    ) -> impl IntoResponse {
        let manager = state.security_manager.read().await;
        
        let path = payload.get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                (StatusCode::BAD_REQUEST, Json(json!({
                    "error": "Missing path"
                })))
            })?;
        
        let operation_str = payload.get("operation")
            .and_then(|v| v.as_str())
            .unwrap_or("read");
        
        let operation = match operation_str.to_lowercase().as_str() {
            "read" => FilesystemOperation::Read,
            "write" => FilesystemOperation::Write,
            "delete" => FilesystemOperation::Delete,
            "execute" => FilesystemOperation::Execute,
            _ => FilesystemOperation::Read,
        };
        
        let tenant_id = payload.get("tenant_id")
            .and_then(|v| v.as_str())
            .and_then(|s| Uuid::parse_str(s).ok())
            .unwrap_or_else(Uuid::new_v4);
        
        match manager.check_filesystem_access(path, operation, tenant_id) {
            Ok(_) => {
                Json(json!({
                    "allowed": true,
                    "path": path,
                    "operation": format!("{:?}", operation),
                }))
            }
            Err(err) => {
                Json(json!({
                    "allowed": false,
                    "path": path,
                    "operation": format!("{:?}", operation),
                    "reason": err,
                }))
            }
        }
    }
    
    /// 🤖 Check inference access
    async fn check_inference_access(
        State(state): State<SecurityAppState>,
        Json(payload): Json<serde_json::Value>,
    ) -> impl IntoResponse {
        let manager = state.security_manager.read().await;
        
        let provider = payload.get("provider")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                (StatusCode::BAD_REQUEST, Json(json!({
                    "error": "Missing provider"
                })))
            })?;
        
        let model = payload.get("model")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                (StatusCode::BAD_REQUEST, Json(json!({
                    "error": "Missing model"
                })))
            })?;
        
        let tokens = payload.get("tokens")
            .and_then(|v| v.as_u64())
            .unwrap_or(1024) as u32;
        
        let tenant_id = payload.get("tenant_id")
            .and_then(|v| v.as_str())
            .and_then(|s| Uuid::parse_str(s).ok())
            .unwrap_or_else(Uuid::new_v4);
        
        match manager.check_inference_access(provider, model, tokens, tenant_id) {
            Ok(_) => {
                Json(json!({
                    "allowed": true,
                    "provider": provider,
