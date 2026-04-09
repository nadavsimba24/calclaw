}", entity.data_type);
            clusters.entry(type_str)
                .or_insert_with(Vec::new)
                .push(entity.id);
        }
        
        clusters.into_iter()
            .map(|(type_name, entity_ids)| {
                serde_json::json!({
                    "type": type_name,
                    "entities": entity_ids,
                    "count": entity_ids.len(),
                })
            })
            .collect()
    }
    
    /// Generate system landscape visualization
    fn generate_system_landscape_visualization(&self) -> serde_json::Value {
        let systems: Vec<serde_json::Value> = self.ontology.systems.iter()
            .map(|system| {
                serde_json::json!({
                    "id": system.id,
                    "name": system.name,
                    "type": format!("{:?}", system.system_type),
                    "vendor": system.vendor.clone().unwrap_or_default(),
                    "version": system.version.clone(),
                    "data_entities": system.data_entities.len(),
                    "status": format!("{:?}", system.status),
                })
            })
            .collect();
        
        serde_json::json!({
            "type": "system_landscape",
            "systems": systems,
            "integration_coverage": self.calculate_system_integration_score(),
        })
    }
    
    /// Generate integration network visualization
    fn generate_integration_network_visualization(&self) -> serde_json::Value {
        let integrations: Vec<serde_json::Value> = self.ontology.integrations.iter()
            .map(|integration| {
                let source_system = self.ontology.systems.iter()
                    .find(|s| s.id == integration.source_system_id)
                    .map(|s| s.name.clone())
                    .unwrap_or_default();
                
                let target_system = self.ontology.systems.iter()
                    .find(|s| s.id == integration.target_system_id)
                    .map(|s| s.name.clone())
                    .unwrap_or_default();
                
                serde_json::json!({
                    "id": integration.id,
                    "name": integration.name,
                    "source": source_system,
                    "target": target_system,
                    "type": format!("{:?}", integration.integration_type),
                    "frequency": format!("{:?}", integration.frequency),
                    "status": format!("{:?}", integration.status),
                })
            })
            .collect();
        
        serde_json::json!({
            "type": "integration_network",
            "integrations": integrations,
            "nodes": self.build_integration_nodes(),
            "edges": self.build_integration_edges(),
        })
    }
    
    /// Build integration nodes (systems)
    fn build_integration_nodes(&self) -> Vec<serde_json::Value> {
        self.ontology.systems.iter()
            .map(|system| {
                let integration_count = self.ontology.integrations.iter()
                    .filter(|i| i.source_system_id == system.id || i.target_system_id == system.id)
                    .count();
                
                serde_json::json!({
                    "id": system.id,
                    "name": system.name,
                    "type": format!("{:?}", system.system_type),
                    "integration_count": integration_count,
                    "size": system.data_entities.len(),
                })
            })
            .collect()
    }
    
    /// Build integration edges
    fn build_integration_edges(&self) -> Vec<serde_json::Value> {
        self.ontology.integrations.iter()
            .map(|integration| {
                serde_json::json!({
                    "source": integration.source_system_id,
                    "target": integration.target_system_id,
                    "id": integration.id,
                    "type": format!("{:?}", integration.integration_type),
                    "strength": integration.data_flow.source_data_entities.len(),
                })
            })
            .collect()
    }
    
    /// Generate goal tree visualization
    fn generate_goal_tree_visualization(&self) -> serde_json::Value {
        let goals: Vec<serde_json::Value> = self.ontology.goals.iter()
            .map(|goal| {
                let department = self.ontology.departments.iter()
                    .find(|dept| dept.id == goal.department_id)
                    .map(|dept| dept.name.clone())
                    .unwrap_or_default();
                
                serde_json::json!({
                    "id": goal.id,
                    "name": goal.name,
                    "department": department,
                    "progress": goal.progress,
                    "status": format!("{:?}", goal.status),
                    "metrics": goal.metrics.len(),
                    "has_parent": goal.parent_goal_id.is_some(),
                })
            })
            .collect();
        
        serde_json::json!({
            "type": "goal_tree",
            "goals": goals,
            "hierarchy": self.build_goal_hierarchy(),
            "overall_progress": self.calculate_overall_goal_progress(),
        })
    }
    
    /// Build goal hierarchy
    fn build_goal_hierarchy(&self) -> serde_json::Value {
        let mut hierarchy = Vec::new();
        
        // Find root goals (no parent)
        let root_goals: Vec<&Goal> = self.ontology.goals.iter()
            .filter(|goal| goal.parent_goal_id.is_none())
            .collect();
        
        for root in root_goals {
            hierarchy.push(self.build_goal_node(root));
        }
        
        serde_json::json!(hierarchy)
    }
    
    /// Build goal node with children
    fn build_goal_node(&self, goal: &Goal) -> serde_json::Value {
        let children: Vec<serde_json::Value> = self.ontology.goals.iter()
            .filter(|g| g.parent_goal_id == Some(goal.id))
            .map(|child| self.build_goal_node(child))
            .collect();
        
        serde_json::json!({
            "id": goal.id,
            "name": goal.name,
            "children": children,
            "progress": goal.progress,
            "status": format!("{:?}", goal.status),
        })
    }
    
    /// Calculate overall goal progress
    fn calculate_overall_goal_progress(&self) -> f64 {
        if self.ontology.goals.is_empty() {
            return 0.0;
        }
        
        self.ontology.goals.iter()
            .map(|goal| goal.progress)
            .sum::<f64>() / self.ontology.goals.len() as f64
    }
    
    /// Generate metric dashboard visualization
    fn generate_metric_dashboard_visualization(&self) -> serde_json::Value {
        let metrics: Vec<serde_json::Value> = self.ontology.metrics.iter()
            .map(|metric| {
                let data_entity = self.ontology.data_entities.iter()
                    .find(|de| de.id == metric.data_entity_id)
                    .map(|de| de.name.clone())
                    .unwrap_or_default();
                
                serde_json::json!({
                    "id": metric.id,
                    "name": metric.name,
                    "data_entity": data_entity,
                    "frequency": format!("{:?}", metric.frequency),
                    "target_value": metric.target_value,
                    "unit": metric.unit.clone(),
                    "visualization": format!("{:?}", metric.visualization.chart_type),
                })
            })
            .collect();
        
        serde_json::json!({
            "type": "metric_dashboard",
            "metrics": metrics,
            "categories": self.build_metric_categories(),
        })
    }
    
    /// Build metric categories
    fn build_metric_categories(&self) -> Vec<serde_json::Value> {
        use std::collections::HashMap;
        
        let mut categories: HashMap<String, Vec<Uuid>> = HashMap::new();
        
        for metric in &self.ontology.metrics {
            // Extract category from name or create one
            let category = if metric.name.to_lowercase().contains("sales") {
                "Sales".to_string()
            } else if metric.name.to_lowercase().contains("customer") {
                "Customer".to_string()
            } else if metric.name.to_lowercase().contains("financial") {
                "Financial".to_string()
            } else if metric.name.to_lowercase().contains("employee") {
                "Employee".to_string()
            } else if metric.name.to_lowercase().contains("operational") {
                "Operational".to_string()
            } else {
                "Other".to_string()
            };
            
            categories.entry(category)
                .or_insert_with(Vec::new)
                .push(metric.id);
        }
        
        categories.into_iter()
            .map(|(category, metric_ids)| {
                serde_json::json!({
                    "category": category,
                    "metrics": metric_ids,
                    "count": metric_ids.len(),
                })
            })
            .collect()
    }
    
    /// Record user interaction
    pub fn record_interaction(&mut self, user_id: Uuid, action: UIAction, target_element: Option<SelectedElement>, outcome: InteractionOutcome) {
        let interaction = UserInteraction {
            id: Uuid::new_v4(),
            user_id,
            action,
            target_element,
            timestamp: Utc::now(),
            duration_seconds: 0.0, // Would be calculated in real usage
            outcome,
        };
        
        self.user_interactions.push(interaction);
    }
    
    /// Get interaction analytics
    pub fn get_interaction_analytics(&self) -> serde_json::Value {
        let total_interactions = self.user_interactions.len();
        let successful_interactions = self.user_interactions.iter()
            .filter(|i| matches!(i.outcome, InteractionOutcome::Success))
            .count();
        
        let actions_by_type: std::collections::HashMap<String, usize> = 
            self.user_interactions.iter()
                .fold(std::collections::HashMap::new(), |mut map, interaction| {
                    *map.entry(format!("{:?}", interaction.action)).or_insert(0) += 1;
                    map
                });
        
        serde_json::json!({
            "total_interactions": total_interactions,
            "success_rate": if total_interactions > 0 {
                successful_interactions as f64 / total_interactions as f64 * 100.0
            } else {
                0.0
            },
            "actions_by_type": actions_by_type,
            "recent_interactions": self.user_interactions.iter()
                .rev()
                .take(10)
                .map(|i| serde_json::json!({
                    "action": format!("{:?}", i.action),
                    "timestamp": i.timestamp.to_rfc3339(),
                    "outcome": format!("{:?}", i.outcome),
                }))
                .collect::<Vec<_>>(),
        })
    }
}

/// 🚀 Main Calclaw Ontology Module
pub mod calclaw_ontology {
    use super::*;
    
    /// Initialize Calclaw with organization understanding
    pub async fn initialize_calclaw() -> (SuperAgent, OntologyUIManager) {
        // Create onboarding questionnaire
        let mut questionnaire = OnboardingQuestionnaire::new();
        
        // In a real scenario, these would come from user input
        // For now, we'll simulate answers
        questionnaire.answer_question(
            questionnaire.questions[0].id,
            serde_json::json!("TechCorp Inc.")
        ).unwrap();
        
        questionnaire.answer_question(
            questionnaire.questions[1].id,
            serde_json::json!("Technology")
        ).unwrap();
        
        questionnaire.answer_question(
            questionnaire.questions[2].id,
            serde_json::json!(150)
        ).unwrap();
        
        questionnaire.answer_question(
            questionnaire.questions[3].id,
            serde_json::json!("Engineering, Sales, Marketing, Support")
        ).unwrap();
        
        questionnaire.answer_question(
            questionnaire.questions[4].id,
            serde_json::json!("Software development, Customer onboarding, Technical support")
        ).unwrap();
        
        questionnaire.answer_question(
            questionnaire.questions[5].id,
            serde_json::json!(["Customer Data", "Product Data", "Sales Data"])
        ).unwrap();
        
        // Generate ontology from questionnaire
        let ontology = questionnaire.generate_ontology().unwrap();
        
        // Create SuperAgent and UI Manager
        let super_agent = SuperAgent::new(ontology.clone());
        let ui_manager = OntologyUIManager::new(ontology);
        
        (super_agent, ui_manager)
    }
    
    /// Run Calclaw with organization understanding
    pub async fn run_calclaw_with_ontology() {
        println!("🚀 Initializing Calclaw with Organization Understanding...");
        
        let (mut super_agent, mut ui_manager) = initialize_calclaw().await;
        
        println!("✅ Calclaw initialized with ontology for: {}", 
                 super_agent.ontology.profile.name);
        
        println!("📊 Ontology Summary:");
        println!("  Departments: {}", super_agent.ontology.departments.len());
        println!("  Processes: {}", super_agent.ontology.processes.len());
        println!("  Data Entities: {}", super_agent.ontology.data_entities.len());
        println!("  Systems: {}", super_agent.ontology.systems.len());
        
        println!("🛠️  Agent Capabilities:");
        for capability in &super_agent.capabilities {
            println!("  • {}: {}", capability.name, capability.description);
        }
        
        println!("🎯 Getting recommendations...");
        let recommendations = super_agent.get_recommendations();
        for recommendation in recommendations {
            println!("  • {} (Priority: {:?})", recommendation.title, recommendation.priority);
            println!("    {}", recommendation.description);
        }
        
        println!("📈 Generating visualizations...");
        let overview_viz = ui_manager.generate_visualization();
        println!("  Overview visualization ready");
        
        // Switch to department view
        ui_manager.ui_state.current_view = ViewType::DepartmentView;
        let department_viz = ui_manager.generate_visualization();
        println!("  Department visualization ready");
        
        println!("🤖 Creating and executing tasks...");
        
        // Create a data analysis task
        let task_id = super_agent.create_task(
            "Analyze Customer Data".to_string(),
            "Perform analysis on customer data to identify trends".to_string(),
            "Data Analysis",
            Priority::High
        ).unwrap();
        
        println!("  Created task: Analyze Customer Data");
        
        // Execute the task
        match super_agent.execute_task(task_id).await {
            Ok(result) => {
                println!("  Task completed: {}", result.success);
                if let Some(output) = result.output.get("insights") {
                    println!("  Insights found: {}", output.as_array().unwrap().len());
                }
            }
            Err(err) => {
                println!("  Task failed: {}", err);
            }
        }
        
        println!("📊 Agent Status:");
        let status = super_agent.get_status();
        println!("  Success Rate: {}%", status["performance"]["success_rate"]);
        println!("  Tasks Completed: {}", status["performance"]["tasks_completed"]);
        println!("  Active Tasks: {}", status["active_tasks"]);
        
        println!("🎉 Calclaw is ready to orchestrate your organization!");
        println!("");
        println!("Next steps:");
        println!("  1. Explore the ontology UI");
        println!("  2. Upload more detailed data");
        println!("  3. Connect your systems");
        println!("  4. Define automation workflows");
        println!("  5. Monitor performance metrics");
    }
}

// Example usage
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_ontology_creation() {
        let profile = OrganizationProfile {
            id: Uuid::new_v4(),
            name: "Test Corp".to_string(),
            description: "Test company".to_string(),
            industry: "Technology".to_string(),
            size: OrganizationSize::Startup(50),
            location: "San Francisco".to_string(),
            timezone: "America/Los_Angeles".to_string(),
            language: "English".to_string(),
            website: Some("https://testcorp.com".to_string()),
            founded_year: Some(2020),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        let mut ontology_manager = OntologyManager::new(profile);
        
        // Add a department
        let engineering_dept = Department {
            id: Uuid::new_v4(),
            name: "Engineering".to_string(),
            description: "Software development".to_string(),
            parent_department_id: None,
            manager_id: None,
            employee_count: 30,
            responsibilities: vec!["Develop software".to_string(), "Maintain systems".to_string()],
            key_processes: Vec::new(),
        };
        
        ontology_manager.add_department(engineering_dept);
        
        // Add a data entity
        let customer_data = DataEntity {
            id: Uuid::new_v4(),
            name: "Customer Data".to_string(),
            description: "Customer information and interactions".to_string(),
            data_type: DataType::CustomerData,
            format: DataFormat::DatabaseTable,
            source: DataSource::InternalSystem("CRM".to_string()),
            sensitivity: DataSensitivity::Confidential,
            retention_days: Some(365),
            owners: Vec::new(),
            relationships: Vec::new(),
        };
        
        ontology_manager.add_data_entity(customer_data);
        
        // Test summary
        let summary = ontology_manager.generate_summary();
        assert!(summary.contains("Test Corp"));
        assert!(summary.contains("Departments: 1"));
        assert!(summary.contains("Data Entities: 1"));
    }
    
    #[tokio::test]
    async fn test_super_agent() {
        let profile = OrganizationProfile {
            id: Uuid::new_v4(),
            name: "Test Corp".to_string(),
            description: "Test company".to_string(),
            industry: "Technology".to_string(),
            size: OrganizationSize::Startup(50),
            location: "San Francisco".to_string(),
            timezone: "America/Los_Angeles".to_string(),
            language: "English".to_string(),
            website: Some("https://testcorp.com".to_string()),
            founded_year: Some(2020),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        let ontology = Organization