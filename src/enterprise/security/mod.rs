// 🛡️ Calclaw Enterprise Security Module
// Security foundation inspired by NemoClaw architecture

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// 🛡️ Security Policy - defines security controls for a tenant or system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPolicy {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub network_policy: NetworkPolicy,
    pub filesystem_policy: FilesystemPolicy,
    pub process_policy: ProcessPolicy,
    pub inference_policy: InferencePolicy,
    pub audit_policy: AuditPolicy,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub enabled: bool,
}

/// 🌐 Network Policy - controls network access
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkPolicy {
    pub egress_rules: Vec<EgressRule>,
    pub ingress_rules: Vec<IngressRule>,
    pub dns_policy: DnsPolicy,
    pub require_approval: bool,
    pub approval_timeout_seconds: u64,
    pub rate_limits: Vec<RateLimit>,
}

/// 🔗 Egress Rule - controls outbound traffic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EgressRule {
    pub id: Uuid,
    pub name: String,
    pub destination: Destination,
    pub protocol: Protocol,
    pub ports: Vec<PortRange>,
    pub action: RuleAction,
    pub require_approval: bool,
    pub description: String,
}

/// 🎯 Destination - network destination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Destination {
    Host(String),           // Specific host
    Cidr(String),          // CIDR range
    Domain(String),        // Domain with wildcards
    Any,                   // Any destination
}

/// 📡 Protocol - network protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Protocol {
    Tcp,
    Udp,
    Icmp,
    Any,
}

/// 🔢 Port Range
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortRange {
    pub start: u16,
    pub end: u16,
}

/// ✅ Rule Action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleAction {
    Allow,
    Deny,
    RequireApproval(String), // Reason for approval
}

/// 🔒 Ingress Rule - controls inbound traffic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngressRule {
    pub id: Uuid,
    pub name: String,
    pub source: Source,
    pub protocol: Protocol,
    pub ports: Vec<PortRange>,
    pub action: RuleAction,
    pub description: String,
}

/// 🌍 Source - network source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Source {
    Ip(String),            // Specific IP
    Cidr(String),         // CIDR range
    Any,                  // Any source
}

/// 🔤 DNS Policy - controls DNS resolution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsPolicy {
    pub allowed_domains: Vec<String>,
    pub blocked_domains: Vec<String>,
    pub use_system_dns: bool,
    pub dns_servers: Vec<String>,
}

/// ⚡ Rate Limit - traffic rate limiting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimit {
    pub destination: Destination,
    pub requests_per_minute: u32,
    pub burst_size: u32,
}

/// 📁 Filesystem Policy - controls filesystem access
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilesystemPolicy {
    pub read_only_paths: Vec<FilesystemPath>,
    pub writable_paths: Vec<FilesystemPath>,
    pub blocked_paths: Vec<FilesystemPath>,
    pub quota_limits: Vec<QuotaLimit>,
    pub backup_policy: BackupPolicy,
}

/// 🗺️ Filesystem Path
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilesystemPath {
    pub path: String,
    pub recursive: bool,
    pub description: String,
}

/// 📊 Quota Limit - storage limits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotaLimit {
    pub path: String,
    pub max_size_mb: u64,
    pub warning_threshold: f64, // Percentage (0.0-1.0)
}

/// 💾 Backup Policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupPolicy {
    pub enabled: bool,
    pub frequency: BackupFrequency,
    pub retention_days: u32,
    pub encryption: bool,
    pub compression: bool,
}

/// 🔄 Backup Frequency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BackupFrequency {
    Hourly,
    Daily,
    Weekly,
    Monthly,
}

/// ⚙️ Process Policy - controls process execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessPolicy {
    pub max_processes: u32,
    pub max_threads: u32,
    pub allowed_executables: Vec<String>,
    pub blocked_executables: Vec<String>,
    pub user_namespace: bool,
    pub capability_drops: Vec<Capability>,
    pub seccomp_profile: SeccompProfile,
}

/// 💪 Linux Capability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Capability {
    CapChown,
    CapDacOverride,
    CapDacReadSearch,
    CapFowner,
    CapFsetid,
    CapKill,
    CapSetgid,
    CapSetuid,
    CapSetpcap,
    CapLinuxImmutable,
    CapNetBindService,
    CapNetBroadcast,
    CapNetAdmin,
    CapNetRaw,
    CapIpcLock,
    CapIpcOwner,
    CapSysModule,
    CapSysRawio,
    CapSysChroot,
    CapSysPtrace,
    CapSysPacct,
    CapSysAdmin,
    CapSysBoot,
    CapSysNice,
    CapSysResource,
    CapSysTime,
    CapSysTtyConfig,
    CapMknod,
    CapLease,
    CapAuditWrite,
    CapAuditControl,
    CapSetfcap,
    CapMacOverride,
    CapMacAdmin,
    CapSyslog,
    CapWakeAlarm,
    CapBlockSuspend,
    CapAuditRead,
}

/// 🔐 Seccomp Profile - system call filtering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeccompProfile {
    pub default_action: SeccompAction,
    pub architectures: Vec<SeccompArch>,
    pub syscalls: Vec<SeccompRule>,
}

/// 🎭 Seccomp Action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SeccompAction {
    Allow,
    Errno(u32),
    Kill,
    Trace,
    Log,
}

/// 🏗️ Seccomp Architecture
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SeccompArch {
    X86,
    X86_64,
    X32,
    Arm,
    Arm64,
    Mips,
    Mips64,
    Mips64N32,
    Mipsel,
    Mipsel64,
    Mipsel64N32,
    Ppc,
    Ppc64,
    Ppc64Le,
    S390,
    S390X,
}

/// 📋 Seccomp Rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeccompRule {
    pub names: Vec<String>,
    pub action: SeccompAction,
    pub args: Vec<SeccompArg>,
}

/// 🔧 Seccomp Argument
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeccompArg {
    pub index: u32,
    pub value: u64,
    pub value_two: u64,
    pub op: SeccompOp,
}

/// ⚖️ Seccomp Operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SeccompOp {
    Ne,
    Lt,
    Le,
    Eq,
    Ge,
    Gt,
    MaskedEq(u64),
}

/// 🤖 Inference Policy - controls model access
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferencePolicy {
    pub allowed_models: Vec<ModelRule>,
    pub blocked_models: Vec<ModelRule>,
    pub max_tokens_per_request: u32,
    pub rate_limits: Vec<InferenceRateLimit>,
    pub cost_limits: Vec<CostLimit>,
    pub privacy_rules: Vec<PrivacyRule>,
}

/// 🧠 Model Rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelRule {
    pub provider: String,
    pub model_pattern: String, // e.g., "gpt-*", "claude-*"
    pub max_context_length: Option<u32>,
    pub require_approval: bool,
}

/// ⏱️ Inference Rate Limit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceRateLimit {
    pub model_pattern: String,
    pub requests_per_hour: u32,
    pub tokens_per_hour: u32,
}

/// 💰 Cost Limit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostLimit {
    pub provider: String,
    pub max_daily_cost: f64,
    pub currency: String,
}

/// 🔒 Privacy Rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyRule {
    pub data_type: DataType,
    pub action: PrivacyAction,
    pub justification: String,
}

/// 📊 Data Type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataType {
    PersonalInformation,
    FinancialData,
    HealthData,
    IntellectualProperty,
    Credentials,
    AnySensitive,
}

/// 🚫 Privacy Action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrivacyAction {
    Allow,
    Block,
    Anonymize,
    Encrypt,
    RequireApproval,
}

/// 📝 Audit Policy - controls auditing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditPolicy {
    pub enabled: bool,
    pub retention_days: u32,
    pub events_to_log: Vec<AuditEventType>,
    pub siem_integration: SiemIntegration,
    pub real_time_alerts: bool,
    pub encryption: bool,
}

/// 📋 Audit Event Type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditEventType {
    NetworkAccess,
    FilesystemAccess,
    ProcessExecution,
    InferenceRequest,
    PolicyChange,
    UserAuthentication,
    DataAccess,
    ConfigurationChange,
}

/// 🔗 SIEM Integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiemIntegration {
    pub enabled: bool,
    pub siem_type: SiemType,
    pub endpoint: String,
    pub api_key: Option<String>,
    pub format: LogFormat,
}

/// 🏢 SIEM Type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SiemType {
    Splunk,
    Elastic,
    Datadog,
    SumoLogic,
    Graylog,
    Custom(String),
}

/// 📄 Log Format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogFormat {
    Json,
    Cef, // Common Event Format
    Leef, // Log Event Extended Format
    Syslog,
    Custom(String),
}

/// 🛡️ Security Manager - manages security policies
pub struct SecurityManager {
    pub policies: HashMap<Uuid, SecurityPolicy>,
    pub active_policy_id: Option<Uuid>,
    pub audit_log: Vec<AuditEvent>,
    pub approval_queue: Vec<ApprovalRequest>,
}

/// 📊 Audit Event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub id: Uuid,
    pub event_type: AuditEventType,
    pub timestamp: DateTime<Utc>,
    pub user_id: Option<Uuid>,
    pub tenant_id: Option<Uuid>,
    pub resource: String,
    pub action: String,
    pub outcome: AuditOutcome,
    pub details: serde_json::Value,
    pub ip_address: Option<String>,
}

/// ✅ Audit Outcome
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditOutcome {
    Success,
    Failure,
    Denied,
    Pending,
}

/// 🤝 Approval Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRequest {
    pub id: Uuid,
    pub request_type: ApprovalType,
    pub requester_id: Uuid,
    pub tenant_id: Uuid,
    pub resource: String,
    pub justification: String,
    pub created_at: DateTime<Utc>,
    pub status: ApprovalStatus,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub expires_at: DateTime<Utc>,
}

/// 🎯 Approval Type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApprovalType {
    NetworkAccess,
    ModelAccess,
    FilesystemAccess,
    ProcessExecution,
    ConfigurationChange,
}

/// 📊 Approval Status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApprovalStatus {
    Pending,
    Approved,
    Denied,
    Expired,
}

impl SecurityManager {
    /// Create a new security manager
    pub fn new() -> Self {
        Self {
            policies: HashMap::new(),
            active_policy_id: None,
            audit_log: Vec::new(),
            approval_queue: Vec::new(),
        }
    }
    
    /// Add a security policy
    pub fn add_policy(&mut self, policy: SecurityPolicy) {
        self.policies.insert(policy.id, policy);
    }
    
    /// Set active policy
    pub fn set_active_policy(&mut self, policy_id: Uuid) -> Result<(), String> {
        if !self.policies.contains_key(&policy_id) {
            return Err("Policy not found".to_string());
        }
        
        self.active_policy_id = Some(policy_id);
        self.log_audit_event(
            AuditEventType::PolicyChange,
            None,
            None,
            "security_policy",
            "set_active",
            AuditOutcome::Success,
            serde_json::json!({"policy_id": policy_id}),
            None,
        );
        
        Ok(())
    }
    
    /// Get active policy
    pub fn get_active_policy(&self) -> Option<&SecurityPolicy> {
        self.active_policy_id
            .and_then(|id| self.policies.get(&id))
    }
    
    /// Check network access
    pub fn check_network_access(
        &self,
        destination: &str,
        protocol: &str,
        port: u16,
        tenant_id: Uuid,
    ) -> Result<(), String> {
        let policy = self.get_active_policy()
            .ok_or("No active security policy")?;
        
        // Check egress rules
        for rule in &policy.network_policy.egress_rules {
            if self.matches_destination(destination, &rule.destination) &&
               self.matches_protocol(protocol, &rule.protocol) &&
               self.matches_port(port, &rule.ports) {
                
                match &rule.action {
                    RuleAction::Allow => {
                        self.log_audit_event(
                            AuditEventType::NetworkAccess,
                            None,
                            Some(tenant_id),
                            destination,
                            "egress",
                            AuditOutcome::Success,
                            serde_json::json!({
                                "destination": destination,
                                "protocol": protocol,
                                "port": port,
                                "rule": rule.name,
                            }),
                            None,
                        );
                        return Ok(());
                    }
                    RuleAction::Deny => {
                        self.log_audit_event(
                            AuditEventType::NetworkAccess,
                            None,
                            Some(tenant_id),
                            destination,
                            "egress",
                            AuditOutcome::Denied,
                            serde_json::json!({
                                "destination": destination,
                                "protocol": protocol,
                                "port": port,
                                "rule": rule.name,
                                "reason": "Explicitly denied by policy",
                            }),
                            None,
                        );
                        return Err("Network access denied by policy".to_string());
                    }
                    RuleAction::RequireApproval(reason) => {
                        // Create approval request
                        let approval_request = ApprovalRequest {
                            id: Uuid::new_v4(),
                            request_type: ApprovalType::NetworkAccess,
                            requester_id: Uuid::new_v4(), // In real implementation, this would be the actual user
                            tenant_id,
                            resource: format!("{}:{} ({})", destination, port, protocol),
                            justification: reason.clone(),
                            created_at: Utc::now(),
                            status: ApprovalStatus::Pending,
                            approved_by: None,
                            approved_at: None,
                            expires_at: Utc::now() + chrono::Duration::seconds(
                                policy.network_policy.approval_timeout_seconds as i64
                            ),
                        };
                        
                        // In real implementation, this would add to approval queue
                        // For now, we'll simulate approval after timeout
                        
                        self.log_audit_event(
                            AuditEventType::NetworkAccess,
                            None,
                            Some(tenant_id),
                            destination,
                            "egress",
                            AuditOutcome::Pending,
                            serde_json::json!({
                                "destination": destination,
                                "protocol": protocol,
                                "port": port,
                                "rule": rule.name,
                                "reason": reason,
                                "approval_request_id": approval_request.id,
                            }),
                            None,
                        );
                        
                        return Err(format!("Network access requires approval: {}", reason));
                    }
                }
            }
        }
        
        // Default deny if no rule matches
        self.log_audit_event(
            AuditEventType::NetworkAccess,
            None,
            Some(tenant_id),
            destination,
            "egress",
            AuditOutcome::Denied,
            ser