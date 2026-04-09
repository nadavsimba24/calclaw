                "tasks_completed": self.performance_metrics.tasks_completed,
                "tasks_failed": self.performance_metrics.tasks_failed,
                "learning_rate": self.performance_metrics.learning_rate
            },
            "knowledge_base": {
                "facts": self.knowledge_base.facts.len(),
                "rules": self.knowledge_base.rules.len(),
                "patterns": self.knowledge_base.patterns.len(),
                "decisions": self.knowledge_base.decisions.len(),
                "learnings": self.knowledge_base.learnings.len()
            },
            "timestamp": Utc::now().to_rfc3339()
        })
    }
    
    /// Learn from a decision outcome
    pub fn learn_from_decision(&mut self, decision_id: Uuid, outcome: DecisionOutcome) {
        let learning = Learning {
            id: Uuid::new_v4(),
            from_decision_id: decision_id,
            insight: outcome.lessons.join(", "),
            action: "Update decision model".to_string(),
            confidence_impact: 0.1, // Small positive impact
            applied: true,
        };
        
        self.knowledge_base.learnings.push(learning);
        self.performance_metrics.learning_rate += 0.1;
        self.update_performance_metrics();
    }
    
    /// Add a fact to knowledge base
    pub fn add_fact(&mut self, subject: String, predicate: String, object: String, source: String) {
        let fact = Fact {
            id: Uuid::new_v4(),
            subject,
            predicate,
            object,
            confidence: 1.0,
            source,
            created_at: Utc::now(),
            expires_at: None,
        };
        
        self.knowledge_base.facts.push(fact);
    }
    
    /// Query knowledge base
    pub fn query_knowledge(&self, query: &str) -> Vec<&Fact> {
        // Simple keyword matching for now
        // In production, this would use proper NLP and reasoning
        self.knowledge_base.facts.iter()
            .filter(|fact| {
                fact.subject.contains(query) ||
                fact.predicate.contains(query) ||
                fact.object.contains(query)
            })
            .collect()
    }
    
    /// Get recommendations based on ontology and current state
    pub fn get_recommendations(&self) -> Vec<Recommendation> {
        let mut recommendations = Vec::new();
        
        // Analyze ontology for recommendations
        if self.ontology.processes.len() > 5 {
            recommendations.push(Recommendation {
                id: Uuid::new_v4(),
                title: "Process Optimization".to_string(),
                description: "You have many processes. Consider automating some of them.".to_string(),
                category: RecommendationCategory::ProcessOptimization,
                priority: Priority::Medium,
                estimated_impact: 0.3,
                actions: vec![
                    "Identify repetitive processes".to_string(),
                    "Evaluate automation potential".to_string(),
                    "Implement automation for top 3 processes".to_string(),
                ],
            });
        }
        
        if self.ontology.data_entities.len() > 10 {
            recommendations.push(Recommendation {
                id: Uuid::new_v4(),
                title: "Data Governance".to_string(),
                description: "You have many data entities. Consider implementing data governance.".to_string(),
                category: RecommendationCategory::DataGovernance,
                priority: Priority::High,
                estimated_impact: 0.4,
                actions: vec![
                    "Define data ownership".to_string(),
                    "Establish data quality standards".to_string(),
                    "Implement data catalog".to_string(),
                ],
            });
        }
        
        if self.ontology.integrations.is_empty() && self.ontology.systems.len() > 1 {
            recommendations.push(Recommendation {
                id: Uuid::new_v4(),
                title: "System Integration".to_string(),
                description: "You have multiple systems but no integrations. Consider connecting them.".to_string(),
                category: RecommendationCategory::SystemIntegration,
                priority: Priority::High,
                estimated_impact: 0.5,
                actions: vec![
                    "Identify key data flows between systems".to_string(),
                    "Design integration architecture".to_string(),
                    "Implement critical integrations".to_string(),
                ],
            });
        }
        
        recommendations
    }
}

/// 💡 Recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub category: RecommendationCategory,
    pub priority: Priority,
    pub estimated_impact: f64, // 0.0 to 1.0
    pub actions: Vec<String>,
}

/// 📋 Recommendation Category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationCategory {
    ProcessOptimization,
    DataGovernance,
    SystemIntegration,
    SecurityImprovement,
    CostReduction,
    RevenueGrowth,
    CustomerExperience,
    EmployeeProductivity,
}

/// 🌐 Ontology UI Manager - Manages the visual ontology interface
pub struct OntologyUIManager {
    pub ontology: OrganizationOntology,
    pub ui_state: UIState,
    pub user_interactions: Vec<UserInteraction>,
    pub visualizations: Vec<Visualization>,
}

/// 🖥️ UI State
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIState {
    pub current_view: ViewType,
    pub selected_element: Option<SelectedElement>,
    pub filters: Vec<Filter>,
    pub zoom_level: f64,
    pub layout: Layout,
}

/// 👁️ View Type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ViewType {
    Overview,
    DepartmentView,
    ProcessFlow,
    DataMap,
    SystemLandscape,
    IntegrationNetwork,
    GoalTree,
    MetricDashboard,
}

/// 🎯 Selected Element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectedElement {
    pub element_type: ElementType,
    pub element_id: Uuid,
    pub properties: serde_json::Value,
}

/// 🔘 Element Type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ElementType {
    Department,
    Process,
    DataEntity,
    System,
    Integration,
    Goal,
    Metric,
    User,
}

/// 🔍 Filter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filter {
    pub field: String,
    pub operator: Operator,
    pub value: serde_json::Value,
}

/// 🗺️ Layout
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Layout {
    Tree,
    Graph,
    Grid,
    Timeline,
    Radial,
    Custom(String),
}

/// 👤 User Interaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInteraction {
    pub id: Uuid,
    pub user_id: Uuid,
    pub action: UIAction,
    pub target_element: Option<SelectedElement>,
    pub timestamp: DateTime<Utc>,
    pub duration_seconds: f64,
    pub outcome: InteractionOutcome,
}

/// 🎮 UI Action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UIAction {
    Click,
    DoubleClick,
    Drag,
    Drop,
    ZoomIn,
    ZoomOut,
    Pan,
    Select,
    Deselect,
    Filter,
    Search,
    Export,
    Import,
    Create,
    Update,
    Delete,
}

/// ✅ Interaction Outcome
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractionOutcome {
    Success,
    PartialSuccess,
    Failure,
    Cancelled,
    Timeout,
}

impl OntologyUIManager {
    /// Create a new UI manager
    pub fn new(ontology: OrganizationOntology) -> Self {
        Self {
            ontology,
            ui_state: UIState {
                current_view: ViewType::Overview,
                selected_element: None,
                filters: Vec::new(),
                zoom_level: 1.0,
                layout: Layout::Graph,
            },
            user_interactions: Vec::new(),
            visualizations: Vec::new(),
        }
    }
    
    /// Generate visualization data for current view
    pub fn generate_visualization(&self) -> serde_json::Value {
        match self.ui_state.current_view {
            ViewType::Overview => self.generate_overview_visualization(),
            ViewType::DepartmentView => self.generate_department_visualization(),
            ViewType::ProcessFlow => self.generate_process_flow_visualization(),
            ViewType::DataMap => self.generate_data_map_visualization(),
            ViewType::SystemLandscape => self.generate_system_landscape_visualization(),
            ViewType::IntegrationNetwork => self.generate_integration_network_visualization(),
            ViewType::GoalTree => self.generate_goal_tree_visualization(),
            ViewType::MetricDashboard => self.generate_metric_dashboard_visualization(),
        }
    }
    
    /// Generate overview visualization
    fn generate_overview_visualization(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "overview",
            "organization": {
                "name": self.ontology.profile.name,
                "industry": self.ontology.profile.industry,
                "size": self.ontology.profile.size,
            },
            "stats": {
                "departments": self.ontology.departments.len(),
                "processes": self.ontology.processes.len(),
                "data_entities": self.ontology.data_entities.len(),
                "systems": self.ontology.systems.len(),
                "integrations": self.ontology.integrations.len(),
                "goals": self.ontology.goals.len(),
                "metrics": self.ontology.metrics.len(),
            },
            "health": {
                "process_coverage": self.calculate_process_coverage(),
                "data_governance": self.calculate_data_governance_score(),
                "system_integration": self.calculate_system_integration_score(),
                "goal_alignment": self.calculate_goal_alignment_score(),
            }
        })
    }
    
    /// Calculate process coverage score
    fn calculate_process_coverage(&self) -> f64 {
        if self.ontology.departments.is_empty() {
            return 0.0;
        }
        
        let departments_with_processes = self.ontology.departments.iter()
            .filter(|dept| {
                self.ontology.processes.iter()
                    .any(|process| process.department_id == dept.id)
            })
            .count();
        
        departments_with_processes as f64 / self.ontology.departments.len() as f64
    }
    
    /// Calculate data governance score
    fn calculate_data_governance_score(&self) -> f64 {
        if self.ontology.data_entities.is_empty() {
            return 0.0;
        }
        
        let entities_with_owners = self.ontology.data_entities.iter()
            .filter(|entity| !entity.owners.is_empty())
            .count();
        
        let entities_with_retention = self.ontology.data_entities.iter()
            .filter(|entity| entity.retention_days.is_some())
            .count();
        
        ((entities_with_owners + entities_with_retention) as f64) / 
        (self.ontology.data_entities.len() as f64 * 2.0)
    }
    
    /// Calculate system integration score
    fn calculate_system_integration_score(&self) -> f64 {
        if self.ontology.systems.len() < 2 {
            return 1.0; // Only one system, fully "integrated"
        }
        
        let possible_integrations = self.ontology.systems.len() * (self.ontology.systems.len() - 1) / 2;
        if possible_integrations == 0 {
            return 0.0;
        }
        
        self.ontology.integrations.len() as f64 / possible_integrations as f64
    }
    
    /// Calculate goal alignment score
    fn calculate_goal_alignment_score(&self) -> f64 {
        if self.ontology.goals.is_empty() {
            return 0.0;
        }
        
        let goals_with_metrics = self.ontology.goals.iter()
            .filter(|goal| !goal.metrics.is_empty())
            .count();
        
        goals_with_metrics as f64 / self.ontology.goals.len() as f64
    }
    
    /// Generate department visualization
    fn generate_department_visualization(&self) -> serde_json::Value {
        let departments: Vec<serde_json::Value> = self.ontology.departments.iter()
            .map(|dept| {
                let processes = self.ontology.processes.iter()
                    .filter(|p| p.department_id == dept.id)
                    .count();
                
                serde_json::json!({
                    "id": dept.id,
                    "name": dept.name,
                    "employee_count": dept.employee_count,
                    "process_count": processes,
                    "responsibilities": dept.responsibilities,
                })
            })
            .collect();
        
        serde_json::json!({
            "type": "department_view",
            "departments": departments,
            "total_employees": self.ontology.departments.iter().map(|d| d.employee_count).sum::<usize>(),
            "hierarchy": self.build_department_hierarchy(),
        })
    }
    
    /// Build department hierarchy
    fn build_department_hierarchy(&self) -> serde_json::Value {
        let mut hierarchy = Vec::new();
        
        // Find root departments (no parent)
        let root_departments: Vec<&Department> = self.ontology.departments.iter()
            .filter(|dept| dept.parent_department_id.is_none())
            .collect();
        
        for root in root_departments {
            hierarchy.push(self.build_department_node(root));
        }
        
        serde_json::json!(hierarchy)
    }
    
    /// Build department node with children
    fn build_department_node(&self, department: &Department) -> serde_json::Value {
        let children: Vec<serde_json::Value> = self.ontology.departments.iter()
            .filter(|dept| dept.parent_department_id == Some(department.id))
            .map(|child| self.build_department_node(child))
            .collect();
        
        serde_json::json!({
            "id": department.id,
            "name": department.name,
            "children": children,
            "size": department.employee_count,
        })
    }
    
    /// Generate process flow visualization
    fn generate_process_flow_visualization(&self) -> serde_json::Value {
        let processes: Vec<serde_json::Value> = self.ontology.processes.iter()
            .map(|process| {
                let department = self.ontology.departments.iter()
                    .find(|dept| dept.id == process.department_id)
                    .map(|dept| dept.name.clone())
                    .unwrap_or_default();
                
                serde_json::json!({
                    "id": process.id,
                    "name": process.name,
                    "department": department,
                    "steps": process.steps.len(),
                    "frequency": format!("{:?}", process.frequency),
                    "automation": format!("{:?}", process.automation_level),
                    "inputs": process.inputs.len(),
                    "outputs": process.outputs.len(),
                })
            })
            .collect();
        
        serde_json::json!({
            "type": "process_flow",
            "processes": processes,
            "connections": self.build_process_connections(),
        })
    }
    
    /// Build process connections based on data flow
    fn build_process_connections(&self) -> Vec<serde_json::Value> {
        let mut connections = Vec::new();
        
        for process in &self.ontology.processes {
            // Connect processes that share data entities
            for other_process in &self.ontology.processes {
                if process.id == other_process.id {
                    continue;
                }
                
                let shared_inputs = process.inputs.iter()
                    .filter(|input| other_process.outputs.contains(input))
                    .count();
                
                let shared_outputs = process.outputs.iter()
                    .filter(|output| other_process.inputs.contains(output))
                    .count();
                
                if shared_inputs > 0 || shared_outputs > 0 {
                    connections.push(serde_json::json!({
                        "source": process.id,
                        "target": other_process.id,
                        "strength": shared_inputs + shared_outputs,
                        "type": "data_flow",
                    }));
                }
            }
        }
        
        connections
    }
    
    /// Generate data map visualization
    fn generate_data_map_visualization(&self) -> serde_json::Value {
        let data_entities: Vec<serde_json::Value> = self.ontology.data_entities.iter()
            .map(|entity| {
                serde_json::json!({
                    "id": entity.id,
                    "name": entity.name,
                    "type": format!("{:?}", entity.data_type),
                    "format": format!("{:?}", entity.format),
                    "sensitivity": format!("{:?}", entity.sensitivity),
                    "relationships": entity.relationships.len(),
                    "owners": entity.owners.len(),
                })
            })
            .collect();
        
        serde_json::json!({
            "type": "data_map",
            "data_entities": data_entities,
            "relationships": self.build_data_relationships(),
            "clusters": self.build_data_clusters(),
        })
    }
    
    /// Build data relationships
    fn build_data_relationships(&self) -> Vec<serde_json::Value> {
        let mut relationships = Vec::new();
        
        for entity in &self.ontology.data_entities {
            for relationship in &entity.relationships {
                relationships.push(serde_json::json!({
                    "source": entity.id,
                    "target": relationship.target_entity_id,
                    "type": format!("{:?}", relationship.relationship_type),
                    "cardinality": format!("{:?}", relationship.cardinality),
                    "description": relationship.description,
                }));
            }
        }
        
        relationships
    }
    
    /// Build data clusters by type
    fn build_data_clusters(&self) -> Vec<serde_json::Value> {
        use std::collections::HashMap;
        
        let mut clusters: HashMap<String, Vec<Uuid>> = HashMap::new();
        
        for entity in &self.ontology.data_entities {
            let type_str = format!("{:?