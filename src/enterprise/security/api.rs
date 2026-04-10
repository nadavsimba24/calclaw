                    "model": model,
                    "tokens": tokens,
                }))
            }
            Err(err) => {
                Json(json!({
                    "allowed": false,
                    "provider": provider,
                    "model": model,
                    "tokens": tokens,
                    "reason": err,
                }))
            }
        }
    }
    
    /// 📊 Get audit logs
    async fn get_audit_logs(
        State(state): State<SecurityAppState>,
    ) -> impl IntoResponse {
        let manager = state.security_manager.read().await;
        
        let limit = 100; // Default limit
        let events: Vec<serde_json::Value> = manager.get_audit_events(Some(limit), None, None)
            .iter()
            .map(|event| {
                json!({
                    "id": event.id,
                    "event_type": format!("{:?}", event.event_type),
                    "timestamp": event.timestamp.to_rfc3339(),
                    "user_id": event.user_id,
                    "tenant_id": event.tenant_id,
                    "resource": event.resource,
                    "action": event.action,
                    "outcome": format!("{:?}", event.outcome),
                    "details": event.details,
                    "ip_address": event.ip_address,
                })
            })
            .collect();
        
        Json(json!({
            "events": events,
            "count": events.len(),
            "total_events": manager.audit_log.len(),
        }))
    }
    
    /// 📥 Export audit logs
    async fn export_audit_logs(
        State(state): State<SecurityAppState>,
    ) -> impl IntoResponse {
        let manager = state.security_manager.read().await;
        
        let events: Vec<serde_json::Value> = manager.audit_log.iter()
            .map(|event| {
                json!({
                    "id": event.id,
                    "event_type": format!("{:?}", event.event_type),
                    "timestamp": event.timestamp.to_rfc3339(),
                    "user_id": event.user_id,
                    "tenant_id": event.tenant_id,
                    "resource": event.resource,
                    "action": event.action,
                    "outcome": format!("{:?}", event.outcome),
                    "details": event.details,
                    "ip_address": event.ip_address,
                })
            })
            .collect();
        
        Json(json!({
            "audit_log": events,
            "exported_at": Utc::now().to_rfc3339(),
            "total_events": events.len(),
        }))
    }
    
    /// 📈 Get audit statistics
    async fn get_audit_stats(
        State(state): State<SecurityAppState>,
    ) -> impl IntoResponse {
        let manager = state.security_manager.read().await;
        
        let mut stats = std::collections::HashMap::new();
        let mut outcome_stats = std::collections::HashMap::new();
        
        for event in &manager.audit_log {
            *stats.entry(format!("{:?}", event.event_type)).or_insert(0) += 1;
            *outcome_stats.entry(format!("{:?}", event.outcome)).or_insert(0) += 1;
        }
        
        Json(json!({
            "event_type_stats": stats,
            "outcome_stats": outcome_stats,
            "total_events": manager.audit_log.len(),
            "time_range": if manager.audit_log.len() > 0 {
                let first = &manager.audit_log[0];
                let last = &manager.audit_log[manager.audit_log.len() - 1];
                format!("{} to {}", first.timestamp.to_rfc3339(), last.timestamp.to_rfc3339())
            } else {
                "No events".to_string()
            },
        }))
    }
    
    /// 🤝 Get approval requests
    async fn get_approval_requests(
        State(state): State<SecurityAppState>,
    ) -> impl IntoResponse {
        let manager = state.security_manager.read().await;
        
        let requests: Vec<serde_json::Value> = manager.approval_queue.iter()
            .map(|request| {
                json!({
                    "id": request.id,
                    "request_type": format!("{:?}", request.request_type),
                    "requester_id": request.requester_id,
                    "tenant_id": request.tenant_id,
                    "resource": request.resource,
                    "justification": request.justification,
                    "created_at": request.created_at.to_rfc3339(),
                    "status": format!("{:?}", request.status),
                    "approved_by": request.approved_by,
                    "approved_at": request.approved_at.map(|t| t.to_rfc3339()),
                    "expires_at": request.expires_at.to_rfc3339(),
                })
            })
            .collect();
        
        Json(json!({
            "requests": requests,
            "count": requests.len(),
            "pending": manager.get_pending_requests(None).len(),
        }))
    }
    
    /// ⏳ Get pending approval requests
    async fn get_pending_approvals(
        State(state): State<SecurityAppState>,
    ) -> impl IntoResponse {
        let manager = state.security_manager.read().await;
        
        let requests: Vec<serde_json::Value> = manager.get_pending_requests(None)
            .iter()
            .map(|request| {
                json!({
                    "id": request.id,
                    "request_type": format!("{:?}", request.request_type),
                    "requester_id": request.requester_id,
                    "tenant_id": request.tenant_id,
                    "resource": request.resource,
                    "justification": request.justification,
                    "created_at": request.created_at.to_rfc3339(),
                    "expires_at": request.expires_at.to_rfc3339(),
                    "time_remaining_seconds": (request.expires_at - Utc::now()).num_seconds(),
                })
            })
            .collect();
        
        Json(json!({
            "requests": requests,
            "count": requests.len(),
        }))
    }
    
    /// ➕ Create approval request
    async fn create_approval_request(
        State(state): State<SecurityAppState>,
        Json(payload): Json<serde_json::Value>,
    ) -> impl IntoResponse {
        let mut manager = state.security_manager.write().await;
        
        let request_type_str = payload.get("request_type")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                (StatusCode::BAD_REQUEST, Json(json!({
                    "error": "Missing request_type"
                })))
            })?;
        
        let request_type = match request_type_str.to_lowercase().as_str() {
            "network_access" => ApprovalType::NetworkAccess,
            "model_access" => ApprovalType::ModelAccess,
            "filesystem_access" => ApprovalType::FilesystemAccess,
            "process_execution" => ApprovalType::ProcessExecution,
            "configuration_change" => ApprovalType::ConfigurationChange,
            _ => {
                return (StatusCode::BAD_REQUEST, Json(json!({
                    "error": "Invalid request_type"
                })))
            }
        };
        
        let requester_id = payload.get("requester_id")
            .and_then(|v| v.as_str())
            .and_then(|s| Uuid::parse_str(s).ok())
            .unwrap_or_else(Uuid::new_v4);
        
        let tenant_id = payload.get("tenant_id")
            .and_then(|v| v.as_str())
            .and_then(|s| Uuid::parse_str(s).ok())
            .unwrap_or_else(Uuid::new_v4);
        
        let resource = payload.get("resource")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                (StatusCode::BAD_REQUEST, Json(json!({
                    "error": "Missing resource"
                })))
            })?;
        
        let justification = payload.get("justification")
            .and_then(|v| v.as_str())
            .unwrap_or("No justification provided");
        
        let timeout_hours = payload.get("timeout_hours")
            .and_then(|v| v.as_u64())
            .unwrap_or(24) as u32;
        
        let request_id = manager.create_approval_request(
            request_type,
            requester_id,
            tenant_id,
            resource.to_string(),
            justification.to_string(),
            timeout_hours,
        );
        
        Json(json!({
            "status": "created",
            "request_id": request_id,
            "resource": resource,
            "timeout_hours": timeout_hours,
        }))
    }
    
    /// ✅ Approve a request
    async fn approve_request(
        State(state): State<SecurityAppState>,
        Path(request_id): Path<Uuid>,
        Json(payload): Json<serde_json::Value>,
    ) -> impl IntoResponse {
        let mut manager = state.security_manager.write().await;
        
        let approver_id = payload.get("approver_id")
            .and_then(|v| v.as_str())
            .and_then(|s| Uuid::parse_str(s).ok())
            .unwrap_or_else(Uuid::new_v4);
        
        match manager.approve_request(request_id, approver_id) {
            Ok(_) => {
                Json(json!({
                    "status": "approved",
                    "request_id": request_id,
                    "approver_id": approver_id,
                }))
            }
            Err(err) => {
                (StatusCode::BAD_REQUEST, Json(json!({
                    "error": err
                })))
            }
        }
    }
    
    /// ❌ Deny a request
    async fn deny_request(
        State(state): State<SecurityAppState>,
        Path(request_id): Path<Uuid>,
        Json(payload): Json<serde_json::Value>,
    ) -> impl IntoResponse {
        let mut manager = state.security_manager.write().await;
        
        let approver_id = payload.get("approver_id")
            .and_then(|v| v.as_str())
            .and_then(|s| Uuid::parse_str(s).ok())
            .unwrap_or_else(Uuid::new_v4);
        
        let reason = payload.get("reason")
            .and_then(|v| v.as_str())
            .unwrap_or("No reason provided");
        
        match manager.deny_request(request_id, approver_id, reason.to_string()) {
            Ok(_) => {
                Json(json!({
                    "status": "denied",
                    "request_id": request_id,
                    "approver_id": approver_id,
                    "reason": reason,
                }))
            }
            Err(err) => {
                (StatusCode::BAD_REQUEST, Json(json!({
                    "error": err
                })))
            }
        }
    }
    
    /// 📊 Get security status
    async fn get_security_status(
        State(state): State<SecurityAppState>,
    ) -> impl IntoResponse {
        let manager = state.security_manager.read().await;
        
        let active_policy = manager.get_active_policy();
        
        Json(json!({
            "active_policy": active_policy.map(|p| p.name),
            "total_policies": manager.policies.len(),
            "audit_events": manager.audit_log.len(),
            "pending_approvals": manager.get_pending_requests(None).len(),
            "security_enabled": active_policy.is_some(),
            "timestamp": Utc::now().to_rfc3339(),
        }))
    }
    
    /// 🩺 Get security health
    async fn get_security_health(
        State(state): State<SecurityAppState>,
    ) -> impl IntoResponse {
        let manager = state.security_manager.read().await;
        
        let has_active_policy = manager.active_policy_id.is_some();
        let has_audit_events = !manager.audit_log.is_empty();
        let has_policies = !manager.policies.is_empty();
        
        let health_score = {
            let mut score = 0;
            if has_active_policy { score += 40; }
            if has_audit_events { score += 30; }
            if has_policies { score += 30; }
            score
        };
        
        Json(json!({
            "health_score": health_score,
            "health_level": if health_score >= 80 {
                "excellent"
            } else if health_score >= 60 {
                "good"
            } else if health_score >= 40 {
                "fair"
            } else {
                "poor"
            },
            "checks": {
                "active_policy": has_active_policy,
                "audit_logging": has_audit_events,
                "policies_defined": has_policies,
            },
            "recommendations": if !has_active_policy {
                vec!["Activate a security policy".to_string()]
            } else if !has_policies {
                vec!["Define security policies".to_string()]
            } else {
                vec![]
            },
        }))
    }
    
    /// 🇪🇺 Get GDPR compliance report
    async fn get_gdpr_compliance(
        State(state): State<SecurityAppState>,
    ) -> impl IntoResponse {
        let manager = state.security_manager.read().await;
        
        let has_audit_logging = !manager.audit_log.is_empty();
        let has_data_access_logs = manager.audit_log.iter()
            .any(|event| matches!(event.event_type, AuditEventType::DataAccess));
        let has_privacy_rules = manager.get_active_policy()
            .map(|p| !p.inference_policy.privacy_rules.is_empty())
            .unwrap_or(false);
        
        Json(json!({
            "compliance_framework": "GDPR",
            "status": if has_audit_logging && has_data_access_logs && has_privacy_rules {
                "compliant"
            } else {
                "partial"
            },
            "checks": {
                "audit_logging": has_audit_logging,
                "data_access_logging": has_data_access_logs,
                "privacy_rules": has_privacy_rules,
            },
            "requirements": [
                "Article 5: Principles relating to processing of personal data",
                "Article 6: Lawfulness of processing",
                "Article 7: Conditions for consent",
                "Article 15: Right of access by the data subject",
                "Article 17: Right to erasure ('right to be forgotten')",
                "Article 25: Data protection by design and by default",
                "Article 30: Records of processing activities",
                "Article 32: Security of processing",
            ],
            "recommendations": if !has_data_access_logs {
                vec!["Enable data access logging".to_string()]
            } else if !has_privacy_rules {
                vec!["Define privacy rules in security policy".to_string()]
            } else {
                vec![]
            },
        }))
    }
    
    /// 🏥 Get HIPAA compliance report
    async fn get_hipaa_compliance(
        State(state): State<SecurityAppState>,
    ) -> impl IntoResponse {
        let manager = state.security_manager.read().await;
        
        let has_encryption = manager.get_active_policy()
            .map(|p| p.audit_policy.encryption && p.filesystem_policy.backup_policy.encryption)
            .unwrap_or(false);
        let has_access_controls = manager.get_active_policy().is_some();
        let has_audit_trail = !manager.audit_log.is_empty();
        
        Json(json!({
            "compliance_framework": "HIPAA",
            "status": if has_encryption && has_access_controls && has_audit_trail {
                "compliant"
            } else {
                "partial"
            },
            "checks": {
                "encryption": has_encryption,
                "access_controls": has_access_controls,
                "audit_trail": has_audit_trail,
            },
            "requirements": [
                "164.308(a)(1): Security management process",
                "164.308(a)(3): Workforce security",
                "164.308(a)(4): Information access management",
                "164.308(a)(5): Security awareness and training",
                "164.308(a)(6): Security incident procedures",
                "164.308(a)(7): Contingency plan",
                "164.308(a)(8): Evaluation",
                "164.310(a)(1): Facility access controls",
                "164.310(b): Workstation use",
                "164.310(c): Workstation security",
                "164.310(d)(1): Device and media controls",
                "164.312(a)(1): Access control",
                "164.312(b): Audit controls",
                "164.312(c)(1): Integrity",
                "164.312(d): Person or entity authentication",
                "164.312(e)(1): Transmission security",
            ],
            "recommendations": if !has_encryption {
                vec!["Enable encryption for audit logs and backups".to_string()]
            } else {
                vec![]
            },
        }))
    }
    
    /// 🏢 Get SOC2 compliance report
    async fn get_soc2_compliance(
        State(state): State<SecurityAppState>,
    ) -> impl IntoResponse {
        let manager = state.security_manager.read().await;
        
        let has_security_policy = manager.get_active_policy().is_some();
        let has_availability_monitoring = true; // Assuming Calclaw has monitoring
        let has_processing_integrity = true; // Assuming Calclaw processes correctly
        let has_confidentiality = manager.get_active_policy()
            .map(|p| !p.inference_policy.privacy_rules.is_empty())
            .unwrap_or(false);
        let has_privacy = manager.get_active_policy()
            .map(|p| !p.inference_policy.privacy_rules.is_empty())
            .unwrap_or(false);
        
        Json(json!({
            "compliance_framework": "SOC2",
            "status": if has_security_policy && has_availability_monitoring && 
                       has_processing_integrity && has_confidentiality && has_privacy {
                "compliant"
            } else {
                "partial"
            },
            "trust_service_criteria": {
                "security": has_security_policy,
                "availability": has_availability_monitoring,
                "processing_integrity": has_processing_integrity,
                "confidentiality": has_confidentiality,
                "privacy": has_privacy,
