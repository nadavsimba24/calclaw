            serde_json::json!({
                "destination": destination,
                "protocol": protocol,
                "port": port,
                "reason": "No matching rule, default deny",
            }),
            None,
        );
        
        Err("Network access denied - no matching rule".to_string())
    }
    
    /// Check filesystem access
    pub fn check_filesystem_access(
        &self,
        path: &str,
        operation: FilesystemOperation,
        tenant_id: Uuid,
    ) -> Result<(), String> {
        let policy = self.get_active_policy()
            .ok_or("No active security policy")?;
        
        // Check blocked paths first
        for blocked_path in &policy.filesystem_policy.blocked_paths {
            if self.matches_path(path, &blocked_path.path, blocked_path.recursive) {
                self.log_audit_event(
                    AuditEventType::FilesystemAccess,
                    None,
                    Some(tenant_id),
                    path,
                    &format!("{:?}", operation),
                    AuditOutcome::Denied,
                    serde_json::json!({
                        "path": path,
                        "operation": format!("{:?}", operation),
                        "reason": "Path is blocked by policy",
                        "blocked_path": blocked_path.path,
                    }),
                    None,
                );
                return Err("Filesystem access denied - path is blocked".to_string());
            }
        }
        
        // Check read-only paths for write operations
        if matches!(operation, FilesystemOperation::Write | FilesystemOperation::Delete) {
            for read_only_path in &policy.filesystem_policy.read_only_paths {
                if self.matches_path(path, &read_only_path.path, read_only_path.recursive) {
                    self.log_audit_event(
                        AuditEventType::FilesystemAccess,
                        None,
                        Some(tenant_id),
                        path,
                        &format!("{:?}", operation),
                        AuditOutcome::Denied,
                        serde_json::json!({
                            "path": path,
                            "operation": format!("{:?}", operation),
                            "reason": "Path is read-only",
                            "read_only_path": read_only_path.path,
                        }),
                        None,
                    );
                    return Err("Filesystem access denied - path is read-only".to_string());
                }
            }
        }
        
        // Check writable paths
        for writable_path in &policy.filesystem_policy.writable_paths {
            if self.matches_path(path, &writable_path.path, writable_path.recursive) {
                self.log_audit_event(
                    AuditEventType::FilesystemAccess,
                    None,
                    Some(tenant_id),
                    path,
                    &format!("{:?}", operation),
                    AuditOutcome::Success,
                    serde_json::json!({
                        "path": path,
                        "operation": format!("{:?}", operation),
                        "writable_path": writable_path.path,
                    }),
                    None,
                );
                return Ok(());
            }
        }
        
        // Default deny if no rule matches
        self.log_audit_event(
            AuditEventType::FilesystemAccess,
            None,
            Some(tenant_id),
            path,
            &format!("{:?}", operation),
            AuditOutcome::Denied,
            serde_json::json!({
                "path": path,
                "operation": format!("{:?}", operation),
                "reason": "No matching rule, default deny",
            }),
            None,
        );
        
        Err("Filesystem access denied - no matching rule".to_string())
    }
    
    /// Check inference access
    pub fn check_inference_access(
        &self,
        provider: &str,
        model: &str,
        tokens: u32,
        tenant_id: Uuid,
    ) -> Result<(), String> {
        let policy = self.get_active_policy()
            .ok_or("No active security policy")?;
        
        // Check blocked models first
        for blocked_model in &policy.inference_policy.blocked_models {
            if self.matches_model(provider, model, &blocked_model.provider, &blocked_model.model_pattern) {
                self.log_audit_event(
                    AuditEventType::InferenceRequest,
                    None,
                    Some(tenant_id),
                    model,
                    "request",
                    AuditOutcome::Denied,
                    serde_json::json!({
                        "provider": provider,
                        "model": model,
                        "tokens": tokens,
                        "reason": "Model is blocked by policy",
                        "blocked_pattern": blocked_model.model_pattern,
                    }),
                    None,
                );
                return Err("Inference access denied - model is blocked".to_string());
            }
        }
        
        // Check allowed models
        for allowed_model in &policy.inference_policy.allowed_models {
            if self.matches_model(provider, model, &allowed_model.provider, &allowed_model.model_pattern) {
                // Check token limit
                if tokens > policy.inference_policy.max_tokens_per_request {
                    self.log_audit_event(
                        AuditEventType::InferenceRequest,
                        None,
                        Some(tenant_id),
                        model,
                        "request",
                        AuditOutcome::Denied,
                        serde_json::json!({
                            "provider": provider,
                            "model": model,
                            "tokens": tokens,
                            "max_tokens": policy.inference_policy.max_tokens_per_request,
                            "reason": "Token limit exceeded",
                        }),
                        None,
                    );
                    return Err(format!(
                        "Token limit exceeded: {} > {}",
                        tokens, policy.inference_policy.max_tokens_per_request
                    ));
                }
                
                // Check if approval is required
                if allowed_model.require_approval {
                    let approval_request = ApprovalRequest {
                        id: Uuid::new_v4(),
                        request_type: ApprovalType::ModelAccess,
                        requester_id: Uuid::new_v4(),
                        tenant_id,
                        resource: format!("{}:{}", provider, model),
                        justification: "Model requires approval".to_string(),
                        created_at: Utc::now(),
                        status: ApprovalStatus::Pending,
                        approved_by: None,
                        approved_at: None,
                        expires_at: Utc::now() + chrono::Duration::hours(24),
                    };
                    
                    self.log_audit_event(
                        AuditEventType::InferenceRequest,
                        None,
                        Some(tenant_id),
                        model,
                        "request",
                        AuditOutcome::Pending,
                        serde_json::json!({
                            "provider": provider,
                            "model": model,
                            "tokens": tokens,
                            "reason": "Model requires approval",
                            "approval_request_id": approval_request.id,
                        }),
                        None,
                    );
                    
                    return Err("Inference access requires approval".to_string());
                }
                
                self.log_audit_event(
                    AuditEventType::InferenceRequest,
                    None,
                    Some(tenant_id),
                    model,
                    "request",
                    AuditOutcome::Success,
                    serde_json::json!({
                        "provider": provider,
                        "model": model,
                        "tokens": tokens,
                        "allowed_pattern": allowed_model.model_pattern,
                    }),
                    None,
                );
                return Ok(());
            }
        }
        
        // Default deny if no rule matches
        self.log_audit_event(
            AuditEventType::InferenceRequest,
            None,
            Some(tenant_id),
            model,
            "request",
            AuditOutcome::Denied,
            serde_json::json!({
                "provider": provider,
                "model": model,
                "tokens": tokens,
                "reason": "No matching rule, default deny",
            }),
            None,
        );
        
        Err("Inference access denied - no matching rule".to_string())
    }
    
    /// Log audit event
    pub fn log_audit_event(
        &mut self,
        event_type: AuditEventType,
        user_id: Option<Uuid>,
        tenant_id: Option<Uuid>,
        resource: &str,
        action: &str,
        outcome: AuditOutcome,
        details: serde_json::Value,
        ip_address: Option<String>,
    ) {
        let event = AuditEvent {
            id: Uuid::new_v4(),
            event_type,
            timestamp: Utc::now(),
            user_id,
            tenant_id,
            resource: resource.to_string(),
            action: action.to_string(),
            outcome,
            details,
            ip_address,
        };
        
        self.audit_log.push(event.clone());
        
        // In real implementation, this would also send to SIEM
        // For now, we just log to the vector
    }
    
    /// Get audit events
    pub fn get_audit_events(
        &self,
        limit: Option<usize>,
        event_type: Option<AuditEventType>,
        tenant_id: Option<Uuid>,
    ) -> Vec<&AuditEvent> {
        let mut events: Vec<&AuditEvent> = self.audit_log.iter()
            .filter(|event| {
                if let Some(ref filter_type) = event_type {
                    if event.event_type != *filter_type {
                        return false;
                    }
                }
                
                if let Some(filter_tenant) = tenant_id {
                    if event.tenant_id != Some(filter_tenant) {
                        return false;
                    }
                }
                
                true
            })
            .collect();
        
        // Sort by timestamp (newest first)
        events.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        
        if let Some(limit) = limit {
            events.truncate(limit);
        }
        
        events
    }
    
    /// Create approval request
    pub fn create_approval_request(
        &mut self,
        request_type: ApprovalType,
        requester_id: Uuid,
        tenant_id: Uuid,
        resource: String,
        justification: String,
        timeout_hours: u32,
    ) -> Uuid {
        let request = ApprovalRequest {
            id: Uuid::new_v4(),
            request_type,
            requester_id,
            tenant_id,
            resource,
            justification,
            created_at: Utc::now(),
            status: ApprovalStatus::Pending,
            approved_by: None,
            approved_at: None,
            expires_at: Utc::now() + chrono::Duration::hours(timeout_hours as i64),
        };
        
        self.approval_queue.push(request.clone());
        
        // Log the approval request
        self.log_audit_event(
            AuditEventType::ConfigurationChange,
            Some(requester_id),
            Some(tenant_id),
            "approval_request",
            "create",
            AuditOutcome::Success,
            serde_json::json!({
                "request_id": request.id,
                "request_type": format!("{:?}", request.request_type),
                "resource": request.resource,
            }),
            None,
        );
        
        request.id
    }
    
    /// Approve a request
    pub fn approve_request(
        &mut self,
        request_id: Uuid,
        approver_id: Uuid,
    ) -> Result<(), String> {
        let request = self.approval_queue.iter_mut()
            .find(|r| r.id == request_id)
            .ok_or("Approval request not found")?;
        
        if request.status != ApprovalStatus::Pending {
            return Err("Request is not pending".to_string());
        }
        
        if Utc::now() > request.expires_at {
            request.status = ApprovalStatus::Expired;
            return Err("Request has expired".to_string());
        }
        
        request.status = ApprovalStatus::Approved;
        request.approved_by = Some(approver_id);
        request.approved_at = Some(Utc::now());
        
        self.log_audit_event(
            AuditEventType::ConfigurationChange,
            Some(approver_id),
            Some(request.tenant_id),
            "approval_request",
            "approve",
            AuditOutcome::Success,
            serde_json::json!({
                "request_id": request.id,
                "resource": request.resource,
                "approver": approver_id,
            }),
            None,
        );
        
        Ok(())
    }
    
    /// Deny a request
    pub fn deny_request(
        &mut self,
        request_id: Uuid,
        approver_id: Uuid,
        reason: String,
    ) -> Result<(), String> {
        let request = self.approval_queue.iter_mut()
            .find(|r| r.id == request_id)
            .ok_or("Approval request not found")?;
        
        if request.status != ApprovalStatus::Pending {
            return Err("Request is not pending".to_string());
        }
        
        request.status = ApprovalStatus::Denied;
        request.approved_by = Some(approver_id);
        request.approved_at = Some(Utc::now());
        
        self.log_audit_event(
            AuditEventType::ConfigurationChange,
            Some(approver_id),
            Some(request.tenant_id),
            "approval_request",
            "deny",
            AuditOutcome::Success,
            serde_json::json!({
                "request_id": request.id,
                "resource": request.resource,
                "approver": approver_id,
                "reason": reason,
            }),
            None,
        );
        
        Ok(())
    }
    
    /// Get pending approval requests
    pub fn get_pending_requests(&self, tenant_id: Option<Uuid>) -> Vec<&ApprovalRequest> {
        self.approval_queue.iter()
            .filter(|request| {
                request.status == ApprovalStatus::Pending &&
                Utc::now() <= request.expires_at &&
                tenant_id.map_or(true, |id| request.tenant_id == id)
            })
            .collect()
    }
    
    // Helper methods
    
    fn matches_destination(&self, destination: &str, rule_destination: &Destination) -> bool {
        match rule_destination {
            Destination::Any => true,
            Destination::Host(host) => destination == host,
            Destination::Cidr(cidr) => {
                // In real implementation, this would check CIDR membership
                // For now, we'll do simple string matching
                destination.starts_with(cidr.trim_end_matches("/0"))
            }
            Destination::Domain(domain_pattern) => {
                self.matches_domain(destination, domain_pattern)
            }
        }
    }
    
    fn matches_domain(&self, domain: &str, pattern: &str) -> bool {
        if pattern == "*" {
            return true;
        }
        
        if pattern.starts_with("*.") {
            let suffix = &pattern[2..];
            return domain.ends_with(suffix) || domain == &suffix[1..];
        }
        
        domain == pattern
    }
    
    fn matches_protocol(&self, protocol: &str, rule_protocol: &Protocol) -> bool {
        match rule_protocol {
            Protocol::Any => true,
            Protocol::Tcp => protocol.to_lowercase() == "tcp",
            Protocol::Udp => protocol.to_lowercase() == "udp",
            Protocol::Icmp => protocol.to_lowercase() == "icmp",
        }
    }
    
    fn matches_port(&self, port: u16, port_ranges: &[PortRange]) -> bool {
        if port_ranges.is_empty() {
            return true; // No port restriction
        }
        
        port_ranges.iter().any(|range| port >= range.start && port <= range.end)
    }
    
    fn matches_path(&self, path: &str, rule_path: &str, recursive: bool) -> bool {
        if recursive {
            path.starts_with(rule_path)
        } else {
            path == rule_path
        }
    }
    
    fn matches_model(&self, provider: &str, model: &str, rule_provider: &str, pattern: &str) -> bool {
        if provider != rule_provider {
            return false;
        }
        
        if pattern == "*" {
            return true;
        }
        
        if pattern.ends_with("*") {
            let prefix = &pattern[..pattern.len() - 1];
            return model.starts_with(prefix);
        }
        
        model == pattern
    }
}

/// 📁 Filesystem Operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilesystemOperation {
    Read,
    Write,
    Delete,
    Execute,
}

/// 🛡️ Default Security Policies
pub mod default_policies {
    use super::*;
    
    /// Create a strict security policy
    pub fn strict_policy() -> SecurityPolicy {
        SecurityPolicy {
            id: Uuid::new_v4(),
            name: "Strict Security Policy".to_string(),
            description: "Maximum security with minimal access".to_string(),
            network_policy: NetworkPolicy {
                egress_rules: vec![
                    EgressRule {
                        id: Uuid::new_v4(),
                        name: "Allow local Ollama".to_string(),
                        destination: Destination::Host("localhost".to_string()),
                        protocol: Protocol::Tcp,
                        ports: vec![PortRange { start: 11434, end: 11434 }],
                        action: RuleAction::Allow,
                        require_approval: false,
                        description: "Allow access to local Ollama instance".to_string(),
                    },
                ],
                ingress_rules: vec![],
                dns_policy: DnsPolicy {
                    allowed_domains: vec![
                        "localhost".to_string(),
                        "127.0.0.1".to_string(),
                    ],
                    blocked_domains: vec![],
                    use_system_dns: false,
                    dns_servers: vec!["1.1.1.1".to_string(), "8.8.8.8".to_string()],
                },
                require_approval: true,
                approval_timeout_seconds: 3600, // 1 hour
                rate_limits: vec![
                    RateLimit {
                        destination: Destination::Any,
                        requests_per_minute: 60,
                        burst_size: 10,
                    },
                ],
            },
            filesystem_policy: FilesystemPolicy {
                read_only_paths: vec![
                    FilesystemPath {
                        path: "/etc".to_string(),
                        recursive: true,
                        description: "System configuration".to_string(),
                    },
                    FilesystemPath {
                        path: "/usr".to_string(),
                        recursive: true,
                        description: "System binaries".to_string(),
                    },
                ],
                writable_paths: vec![
                    FilesystemPath {
                        path: "/tmp".to_string(),
                        recursive: true,
                        description: "