// Skills system for CalcLaw
// Organization-specific skill creator and management

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

// Skill types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SkillType {
    DataProcessor,      // Process data from integrations
    ReportGenerator,    // Generate reports
    Notification,       // Send notifications
    Workflow,          // Multi-step workflows
    Integration,       // Connect to external services
    Analysis,          // Data analysis
    Automation,        // Task automation
    Custom,            // Custom skill type
}

// Skill trigger conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TriggerCondition {
    Scheduled(String),           // Cron expression
    OnMessage(String),           // When message matches pattern
    OnDataUpdate(String),        // When data changes
    OnEvent(String),             // Custom event
    Manual,                      // Manual trigger
}

// Skill action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SkillAction {
    SendMessage {
        channel: String,
        template: String,
        variables: Vec<String>,
    },
    GenerateReport {
        data_source: String,
        template: String,
        format: String, // pdf, html, csv, etc.
    },
    UpdateData {
        target: String,
        operation: String, // create, update, delete
        data: serde_json::Value,
    },
    CallApi {
        url: String,
        method: String,
        headers: HashMap<String, String>,
        body: Option<serde_json::Value>,
    },
    ExecuteScript {
        language: String, // python, javascript, bash
        code: String,
    },
    RunWorkflow {
        workflow_id: String,
        inputs: HashMap<String, serde_json::Value>,
    },
}

// Skill definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillDefinition {
    pub id: String,
    pub name: String,
    pub description: String,
    pub skill_type: SkillType,
    pub version: String,
    pub author: String,
    pub organization_id: String,
    pub department: Option<String>,
    
    // Triggers
    pub triggers: Vec<TriggerCondition>,
    
    // Actions
    pub actions: Vec<SkillAction>,
    
    // Configuration
    pub config_schema: Option<serde_json::Value>, // JSON Schema for configuration
    pub default_config: Option<serde_json::Value>,
    
    // Permissions
    pub required_permissions: Vec<String>,
    pub allowed_roles: Vec<String>, // admin, superuser, user
    
    // Metadata
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub enabled: bool,
}

// Skill instance (with configuration)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillInstance {
    pub id: String,
    pub skill_id: String,
    pub organization_id: String,
    pub department: Option<String>,
    
    // Configuration
    pub config: serde_json::Value,
    
    // State
    pub enabled: bool,
    pub last_run: Option<chrono::DateTime<chrono::Utc>>,
    pub next_run: Option<chrono::DateTime<chrono::Utc>>,
    pub run_count: u32,
    pub error_count: u32,
    
    // Metadata
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

// Skill execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillExecutionResult {
    pub success: bool,
    pub skill_instance_id: String,
    pub skill_id: String,
    pub execution_time_ms: u64,
    pub output: Option<serde_json::Value>,
    pub error: Option<String>,
    pub logs: Vec<String>,
}

// Skill template for organization creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillTemplate {
    pub name: String,
    pub description: String,
    pub skill_type: SkillType,
    pub triggers: Vec<TriggerCondition>,
    pub actions: Vec<SkillAction>,
    pub config_schema: Option<serde_json::Value>,
    pub default_config: Option<serde_json::Value>,
    pub required_permissions: Vec<String>,
    pub allowed_roles: Vec<String>,
}

// Organization skill creator
pub struct SkillCreator {
    skills_dir: PathBuf,
    templates_dir: PathBuf,
    instances_dir: PathBuf,
}

impl SkillCreator {
    pub fn new(base_dir: &Path) -> Result<Self> {
        let skills_dir = base_dir.join("skills");
        let templates_dir = base_dir.join("skill_templates");
        let instances_dir = base_dir.join("skill_instances");
        
        // Create directories if they don't exist
        for dir in [&skills_dir, &templates_dir, &instances_dir] {
            if !dir.exists() {
                fs::create_dir_all(dir)
                    .map_err(|e| anyhow!("Failed to create directory {}: {}", dir.display(), e))?;
            }
        }
        
        Ok(Self {
            skills_dir,
            templates_dir,
            instances_dir,
        })
    }
    
    // Create a new skill from template
    pub async fn create_skill_from_template(
        &self,
        template: &SkillTemplate,
        organization_id: &str,
        department: Option<&str>,
        author: &str,
    ) -> Result<SkillDefinition> {
        let skill_id = format!("{}_{}_{}", organization_id, template.name.to_lowercase().replace(" ", "_"), chrono::Utc::now().timestamp());
        
        let skill = SkillDefinition {
            id: skill_id.clone(),
            name: template.name.clone(),
            description: template.description.clone(),
            skill_type: template.skill_type.clone(),
            version: "1.0.0".to_string(),
            author: author.to_string(),
            organization_id: organization_id.to_string(),
            department: department.map(|d| d.to_string()),
            triggers: template.triggers.clone(),
            actions: template.actions.clone(),
            config_schema: template.config_schema.clone(),
            default_config: template.default_config.clone(),
            required_permissions: template.required_permissions.clone(),
            allowed_roles: template.allowed_roles.clone(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            enabled: true,
        };
        
        // Save skill definition
        self.save_skill_definition(&skill).await?;
        
        info!("Created new skill: {} for organization: {}", skill.name, organization_id);
        
        Ok(skill)
    }
    
    // Create skill instance
    pub async fn create_skill_instance(
        &self,
        skill_id: &str,
        organization_id: &str,
        department: Option<&str>,
        config: serde_json::Value,
    ) -> Result<SkillInstance> {
        let instance_id = format!("instance_{}_{}", skill_id, chrono::Utc::now().timestamp());
        
        let instance = SkillInstance {
            id: instance_id.clone(),
            skill_id: skill_id.to_string(),
            organization_id: organization_id.to_string(),
            department: department.map(|d| d.to_string()),
            config,
            enabled: true,
            last_run: None,
            next_run: None,
            run_count: 0,
            error_count: 0,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        
        // Save instance
        self.save_skill_instance(&instance).await?;
        
        info!("Created skill instance: {} for skill: {}", instance_id, skill_id);
        
        Ok(instance)
    }
    
    // Save skill definition to file
    async fn save_skill_definition(&self, skill: &SkillDefinition) -> Result<()> {
        let file_path = self.skills_dir.join(format!("{}.json", skill.id));
        let content = serde_json::to_string_pretty(skill)
            .map_err(|e| anyhow!("Failed to serialize skill: {}", e))?;
        
        tokio::fs::write(&file_path, content).await
            .map_err(|e| anyhow!("Failed to save skill to {}: {}", file_path.display(), e))?;
        
        Ok(())
    }
    
    // Save skill instance to file
    async fn save_skill_instance(&self, instance: &SkillInstance) -> Result<()> {
        let file_path = self.instances_dir.join(format!("{}.json", instance.id));
        let content = serde_json::to_string_pretty(instance)
            .map_err(|e| anyhow!("Failed to serialize skill instance: {}", e))?;
        
        tokio::fs::write(&file_path, content).await
            .map_err(|e| anyhow!("Failed to save skill instance to {}: {}", file_path.display(), e))?;
        
        Ok(())
    }
    
    // Load skill definition
    pub async fn load_skill_definition(&self, skill_id: &str) -> Result<SkillDefinition> {
        let file_path = self.skills_dir.join(format!("{}.json", skill_id));
        
        if !file_path.exists() {
            return Err(anyhow!("Skill not found: {}", skill_id));
        }
        
        let content = tokio::fs::read_to_string(&file_path).await
            .map_err(|e| anyhow!("Failed to read skill file {}: {}", file_path.display(), e))?;
        
        let skill: SkillDefinition = serde_json::from_str(&content)
            .map_err(|e| anyhow!("Failed to parse skill JSON: {}", e))?;
        
        Ok(skill)
    }
    
    // Load skill instance
    pub async fn load_skill_instance(&self, instance_id: &str) -> Result<SkillInstance> {
        let file_path = self.instances_dir.join(format!("{}.json", instance_id));
        
        if !file_path.exists() {
            return Err(anyhow!("Skill instance not found: {}", instance_id));
        }
        
        let content = tokio::fs::read_to_string(&file_path).await
            .map_err(|e| anyhow!("Failed to read skill instance file {}: {}", file_path.display(), e))?;
        
        let instance: SkillInstance = serde_json::from_str(&content)
            .map_err(|e| anyhow!("Failed to parse skill instance JSON: {}", e))?;
        
        Ok(instance)
    }
    
    // List skills for organization
    pub async fn list_organization_skills(&self, organization_id: &str) -> Result<Vec<SkillDefinition>> {
        let mut skills = Vec::new();
        
        let entries = fs::read_dir(&self.skills_dir)
            .map_err(|e| anyhow!("Failed to read skills directory: {}", e))?;
        
        for entry in entries {
            let entry = entry.map_err(|e| anyhow!("Failed to read directory entry: {}", e))?;
            let path = entry.path();
            
            if path.extension().and_then(|ext| ext.to_str()) == Some("json") {
                let content = fs::read_to_string(&path)
                    .map_err(|e| anyhow!("Failed to read skill file {}: {}", path.display(), e))?;
                
                let skill: SkillDefinition = serde_json::from_str(&content)
                    .map_err(|e| anyhow!("Failed to parse skill JSON from {}: {}", path.display(), e))?;
                
                if skill.organization_id == organization_id {
                    skills.push(skill);
                }
            }
        }
        
        Ok(skills)
    }
    
    // List skill instances for organization
    pub async fn list_organization_instances(&self, organization_id: &str) -> Result<Vec<SkillInstance>> {
        let mut instances = Vec::new();
        
        let entries = fs::read_dir(&self.instances_dir)
            .map_err(|e| anyhow!("Failed to read instances directory: {}", e))?;
        
        for entry in entries {
            let entry = entry.map_err(|e| anyhow!("Failed to read directory entry: {}", e))?;
            let path = entry.path();
            
            if path.extension().and_then(|ext| ext.to_str()) == Some("json") {
                let content = fs::read_to_string(&path)
                    .map_err(|e| anyhow!("Failed to read instance file {}: {}", path.display(), e))?;
                
                let instance: SkillInstance = serde_json::from_str(&content)
                    .map_err(|e| anyhow!("Failed to parse instance JSON from {}: {}", path.display(), e))?;
                
                if instance.organization_id == organization_id {
                    instances.push(instance);
                }
            }
        }
        
        Ok(instances)
    }
    
    // Update skill instance
    pub async fn update_skill_instance(&self, instance_id: &str, updates: SkillInstanceUpdate) -> Result<SkillInstance> {
        let mut instance = self.load_skill_instance(instance_id).await?;
        
        if let Some(config) = updates.config {
            instance.config = config;
        }
        
        if let Some(enabled) = updates.enabled {
            instance.enabled = enabled;
        }
        
        instance.updated_at = chrono::Utc::now();
        
        // Save updated instance
        self.save_skill_instance(&instance).await?;
        
        Ok(instance)
    }
    
    // Delete skill instance
    pub async fn delete_skill_instance(&self, instance_id: &str) -> Result<()> {
        let file_path = self.instances_dir.join(format!("{}.json", instance_id));
        
        if file_path.exists() {
            fs::remove_file(&file_path)
                .map_err(|e| anyhow!("Failed to delete skill instance file {}: {}", file_path.display(), e))?;
        }
        
        Ok(())
    }
    
    // Load skill templates
    pub async fn load_skill_templates(&self) -> Result<Vec<SkillTemplate>> {
        let mut templates = Vec::new();
        
        let entries = fs::read_dir(&self.templates_dir)
            .map_err(|e| anyhow!("Failed to read templates directory: {}", e))?;
        
        for entry in entries {
            let entry = entry.map_err(|e| anyhow!("Failed to read directory entry: {}", e))?;
            let path = entry.path();
            
            if path.extension().and_then(|ext| ext.to_str()) == Some("json") {
                let content = fs::read_to_string(&path)
                    .map_err(|e| anyhow!("Failed to read template file {}: {}", path.display(), e))?;
                
                let template: SkillTemplate = serde_json::from_str(&content)
                    .map_err(|e| anyhow!("Failed to parse template JSON from {}: {}", path.display(), e))?;
                
                templates.push(template);
            }
        }
        
        Ok(templates)
    }
    
    // Save skill template
    pub async fn save_skill_template(&self, template: &SkillTemplate) -> Result<()> {
        let filename = format!("{}.json", template.name.to_lowercase().replace(" ", "_"));
        let file_path = self.templates_dir.join(filename);
        
        let content = serde_json::to_string_pretty(template)
            .map_err(|e| anyhow!("Failed to serialize template: {}", e))?;
        
        tokio::fs::write(&file_path, content).await
            .map_err(|e| anyhow!("Failed to save template to {}: {}", file_path.display(), e))?;
        
        Ok(())
    }
}

// Skill instance update structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillInstanceUpdate {
    pub config: Option<serde_json::Value>,
    pub enabled: Option<bool>,
}

// Built-in skill templates
pub fn get_builtin_templates() -> Vec<SkillTemplate> {
    vec![
        // Daily Sales Report
        SkillTemplate {
            name: "Daily Sales Report".to_string(),
            description: "Generate daily sales report and send to managers".to_string(),
            skill_type: SkillType::ReportGenerator,
            triggers: vec![TriggerCondition::Scheduled("0 9 * * *".to_string())], // 9 AM daily
            actions: vec![
                SkillAction::GenerateReport {
                    data_source: "salesforce".to_string(),
                    template: "daily_sales".to_string(),
                    format: "pdf".to_string(),
                },
                SkillAction::SendMessage {
                    channel: "telegram".to_string(),
                    template: "Daily sales report for {date} is ready. Total sales: ${total}".to_string(),
                    variables: vec!["date".to_string(), "total".to_string()],
                },
            ],
            config_schema: Some(serde_json::json!({
                "type": "object",
                "properties": {
                    "recipients": {
                        "type": "array",
                        "items": {"type": "string"},
                        "description": "List of email recipients"
                    },
                    "sales_target": {
                        "type": "number",
                        "description": "Daily sales target"
                    }
                },
                "required": ["recipients"]
            })),
            default_config: Some(serde_json::json!({
                "recipients": ["sales@example.com"],
                "sales_target": 10000
            })),
            required_permissions: vec!["read_sales_data".to_string()],
            allowed_roles: vec!["admin".to_string(), "superuser".to_string()],
        },
        
        // Customer Support Notification
        SkillTemplate {
            name: "Support Ticket Alert".to_string(),
            description: "Send notification when new support ticket is created".to_string(),
            skill_type: SkillType::Notification,
            triggers: vec![TriggerCondition::OnDataUpdate("servicenow.ticket.created".to_string())],
            actions: vec![
                SkillAction::SendMessage {
                    channel: "whatsapp".to_string(),
