Basics,
    Departments,
    Processes,
    Data,
    Systems,
    Goals,
    Security,
    Compliance,
}

/// 📝 Answer Type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnswerType {
    Text,
    Number,
    Boolean,
    SingleSelect,
    MultiSelect,
    Date,
    DateTime,
    File,
    JSON,
}

/// 🧠 Onboarding Questionnaire
pub struct OnboardingQuestionnaire {
    pub questions: Vec<OnboardingQuestion>,
    pub organization_id: Option<Uuid>,
    pub completed: bool,
    pub answers: Vec<QuestionAnswer>,
}

/// ✅ Question Answer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionAnswer {
    pub question_id: Uuid,
    pub answer: serde_json::Value,
    pub answered_at: DateTime<Utc>,
    pub confidence: f64, // 0.0 to 1.0
    pub notes: Option<String>,
}

impl OnboardingQuestionnaire {
    /// Create a new onboarding questionnaire
    pub fn new() -> Self {
        let questions = Self::default_questions();
        Self {
            questions,
            organization_id: None,
            completed: false,
            answers: Vec::new(),
        }
    }
    
    /// Default questions for organization onboarding
    fn default_questions() -> Vec<OnboardingQuestion> {
        vec![
            // Organization Basics
            OnboardingQuestion {
                id: Uuid::new_v4(),
                category: QuestionCategory::OrganizationBasics,
                question: "What is your organization's name?".to_string(),
                description: "The official name of your company or organization".to_string(),
                answer_type: AnswerType::Text,
                required: true,
                options: None,
                depends_on: None,
                validation: Some(ValidationRule {
                    rule_type: ValidationType::Required,
                    parameters: serde_json::json!({}),
                    error_message: "Organization name is required".to_string(),
                }),
            },
            OnboardingQuestion {
                id: Uuid::new_v4(),
                category: QuestionCategory::OrganizationBasics,
                question: "What industry are you in?".to_string(),
                description: "Select your primary industry".to_string(),
                answer_type: AnswerType::SingleSelect,
                required: true,
                options: Some(vec![
                    "Technology".to_string(),
                    "Finance".to_string(),
                    "Healthcare".to_string(),
                    "Retail".to_string(),
                    "Manufacturing".to_string(),
                    "Education".to_string(),
                    "Government".to_string(),
                    "Non-Profit".to_string(),
                    "Other".to_string(),
                ]),
                depends_on: None,
                validation: None,
            },
            OnboardingQuestion {
                id: Uuid::new_v4(),
                category: QuestionCategory::OrganizationBasics,
                question: "How many employees do you have?".to_string(),
                description: "Approximate number of employees".to_string(),
                answer_type: AnswerType::Number,
                required: true,
                options: None,
                depends_on: None,
                validation: Some(ValidationRule {
                    rule_type: ValidationType::Range(1.0, 100000.0),
                    parameters: serde_json::json!({"min": 1, "max": 100000}),
                    error_message: "Employee count must be between 1 and 100,000".to_string(),
                }),
            },
            
            // Departments
            OnboardingQuestion {
                id: Uuid::new_v4(),
                category: QuestionCategory::Departments,
                question: "What are your main departments?".to_string(),
                description: "List your primary departments (comma separated)".to_string(),
                answer_type: AnswerType::Text,
                required: false,
                options: None,
                depends_on: None,
                validation: None,
            },
            
            // Key Processes
            OnboardingQuestion {
                id: Uuid::new_v4(),
                category: QuestionCategory::Processes,
                question: "What are your key business processes?".to_string(),
                description: "Describe your main workflows (e.g., sales, support, development)".to_string(),
                answer_type: AnswerType::Text,
                required: false,
                options: None,
                depends_on: None,
                validation: None,
            },
            
            // Data
            OnboardingQuestion {
                id: Uuid::new_v4(),
                category: QuestionCategory::Data,
                question: "What are your main data types?".to_string(),
                description: "What kind of data do you work with? (e.g., customer, product, financial)".to_string(),
                answer_type: AnswerType::MultiSelect,
                required: false,
                options: Some(vec![
                    "Customer Data".to_string(),
                    "Product Data".to_string(),
                    "Financial Data".to_string(),
                    "Employee Data".to_string(),
                    "Sales Data".to_string(),
                    "Marketing Data".to_string(),
                    "Operational Data".to_string(),
                    "Technical Data".to_string(),
                ]),
                depends_on: None,
                validation: None,
            },
            
            // Systems
            OnboardingQuestion {
                id: Uuid::new_v4(),
                category: QuestionCategory::Systems,
                question: "What systems do you use?".to_string(),
                description: "List your main software systems (CRM, ERP, etc.)".to_string(),
                answer_type: AnswerType::Text,
                required: false,
                options: None,
                depends_on: None,
                validation: None,
            },
            
            // Goals
            OnboardingQuestion {
                id: Uuid::new_v4(),
                category: QuestionCategory::Goals,
                question: "What are your main business goals?".to_string(),
                description: "Describe your primary objectives for the next year".to_string(),
                answer_type: AnswerType::Text,
                required: false,
                options: None,
                depends_on: None,
                validation: None,
            },
        ]
    }
    
    /// Answer a question
    pub fn answer_question(&mut self, question_id: Uuid, answer: serde_json::Value) -> Result<(), String> {
        // Find the question
        let question = self.questions.iter()
            .find(|q| q.id == question_id)
            .ok_or_else(|| "Question not found".to_string())?;
        
        // Validate the answer
        if let Some(validation) = &question.validation {
            if !self.validate_answer(&answer, validation) {
                return Err(validation.error_message.clone());
            }
        }
        
        // Check if already answered
        if let Some(existing) = self.answers.iter_mut().find(|a| a.question_id == question_id) {
            existing.answer = answer;
            existing.answered_at = Utc::now();
            existing.confidence = 1.0;
        } else {
            self.answers.push(QuestionAnswer {
                question_id,
                answer,
                answered_at: Utc::now(),
                confidence: 1.0,
                notes: None,
            });
        }
        
        Ok(())
    }
    
    /// Validate an answer
    fn validate_answer(&self, answer: &serde_json::Value, validation: &ValidationRule) -> bool {
        match &validation.rule_type {
            ValidationType::Required => {
                !answer.is_null() && 
                !(answer.is_string() && answer.as_str().unwrap_or("").trim().is_empty())
            }
            ValidationType::MinLength(min) => {
                if let Some(s) = answer.as_str() {
                    s.len() >= *min
                } else {
                    false
                }
            }
            ValidationType::MaxLength(max) => {
                if let Some(s) = answer.as_str() {
                    s.len() <= *max
                } else {
                    false
                }
            }
            ValidationType::Pattern(pattern) => {
                if let Some(s) = answer.as_str() {
                    let re = regex::Regex::new(pattern).unwrap();
                    re.is_match(s)
                } else {
                    false
                }
            }
            ValidationType::Range(min, max) => {
                if let Some(n) = answer.as_f64() {
                    n >= *min && n <= *max
                } else {
                    false
                }
            }
            ValidationType::InList(list) => {
                if let Some(s) = answer.as_str() {
                    list.contains(&s.to_string())
                } else {
                    false
                }
            }
            ValidationType::Custom(_) => true, // Custom validation would be implemented separately
        }
    }
    
    /// Calculate completion percentage
    pub fn completion_percentage(&self) -> f64 {
        let required_questions = self.questions.iter()
            .filter(|q| q.required)
            .count();
        
        if required_questions == 0 {
            return 100.0;
        }
        
        let answered_required = self.answers.iter()
            .filter(|a| {
                self.questions.iter()
                    .find(|q| q.id == a.question_id)
                    .map(|q| q.required)
                    .unwrap_or(false)
            })
            .count();
        
        (answered_required as f64 / required_questions as f64) * 100.0
    }
    
    /// Generate ontology from answers
    pub fn generate_ontology(&self) -> Result<OrganizationOntology, String> {
        if self.completion_percentage() < 100.0 {
            return Err("Questionnaire not complete".to_string());
        }
        
        // Extract answers
        let org_name = self.get_answer_text("What is your organization's name?")?;
        let industry = self.get_answer_text("What industry are you in?")?;
        let employee_count: usize = self.get_answer_number("How many employees do you have?")? as usize;
        
        // Create organization profile
        let profile = OrganizationProfile {
            id: Uuid::new_v4(),
            name: org_name,
            description: format!("{} company in {}", org_name, industry),
            industry,
            size: Self::determine_organization_size(employee_count),
            location: "Unknown".to_string(), // Would come from additional questions
            timezone: "UTC".to_string(),
            language: "English".to_string(),
            website: None,
            founded_year: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        // Create ontology
        let mut ontology = OrganizationOntology {
            profile,
            departments: Vec::new(),
            processes: Vec::new(),
            data_entities: Vec::new(),
            user_roles: Vec::new(),
            systems: Vec::new(),
            integrations: Vec::new(),
            goals: Vec::new(),
            metrics: Vec::new(),
        };
        
        // Add departments if answered
        if let Ok(departments_str) = self.get_answer_text("What are your main departments?") {
            for dept_name in departments_str.split(',') {
                let dept_name = dept_name.trim();
                if !dept_name.is_empty() {
                    ontology.departments.push(Department {
                        id: Uuid::new_v4(),
                        name: dept_name.to_string(),
                        description: format!("{} department", dept_name),
                        parent_department_id: None,
                        manager_id: None,
                        employee_count: 0,
                        responsibilities: Vec::new(),
                        key_processes: Vec::new(),
                    });
                }
            }
        }
        
        // Add data entities if answered
        if let Ok(data_types) = self.get_answer_array("What are your main data types?") {
            for data_type in data_types {
                ontology.data_entities.push(DataEntity {
                    id: Uuid::new_v4(),
                    name: data_type.clone(),
                    description: format!("{} data for {}", data_type, ontology.profile.name),
                    data_type: match data_type.as_str() {
                        "Customer Data" => DataType::CustomerData,
                        "Product Data" => DataType::ProductData,
                        "Financial Data" => DataType::FinancialData,
                        "Employee Data" => DataType::EmployeeData,
                        "Sales Data" => DataType::SalesData,
                        "Marketing Data" => DataType::MarketingData,
                        "Operational Data" => DataType::OperationalData,
                        "Technical Data" => DataType::TechnicalData,
                        _ => DataType::Custom(data_type.clone()),
                    },
                    format: DataFormat::DatabaseTable,
                    source: DataSource::InternalSystem("Unknown".to_string()),
                    sensitivity: DataSensitivity::Confidential,
                    retention_days: Some(365),
                    owners: Vec::new(),
                    relationships: Vec::new(),
                });
            }
        }
        
        Ok(ontology)
    }
    
    /// Helper to get text answer
    fn get_answer_text(&self, question: &str) -> Result<String, String> {
        let question_id = self.questions.iter()
            .find(|q| q.question == question)
            .map(|q| q.id)
            .ok_or_else(|| "Question not found".to_string())?;
        
        self.answers.iter()
            .find(|a| a.question_id == question_id)
            .and_then(|a| a.answer.as_str().map(|s| s.to_string()))
            .ok_or_else(|| "Answer not found".to_string())
    }
    
    /// Helper to get number answer
    fn get_answer_number(&self, question: &str) -> Result<f64, String> {
        let question_id = self.questions.iter()
            .find(|q| q.question == question)
            .map(|q| q.id)
            .ok_or_else(|| "Question not found".to_string())?;
        
        self.answers.iter()
            .find(|a| a.question_id == question_id)
            .and_then(|a| a.answer.as_f64())
            .ok_or_else(|| "Answer not found or not a number".to_string())
    }
    
    /// Helper to get array answer
    fn get_answer_array(&self, question: &str) -> Result<Vec<String>, String> {
        let question_id = self.questions.iter()
            .find(|q| q.question == question)
            .map(|q| q.id)
            .ok_or_else(|| "Question not found".to_string())?;
        
        self.answers.iter()
            .find(|a| a.question_id == question_id)
            .and_then(|a| {
                if let Some(arr) = a.answer.as_array() {
                    Some(arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect())
                } else if let Some(s) = a.answer.as_str() {
                    Some(vec![s.to_string()])
                } else {
                    None
                }
            })
            .ok_or_else(|| "Answer not found or not an array".to_string())
    }
    
    /// Determine organization size based on employee count
    fn determine_organization_size(employee_count: usize) -> OrganizationSize {
        match employee_count {
            1..=50 => OrganizationSize::Startup(employee_count),
            51..=200 => OrganizationSize::SmallBusiness(employee_count),
            201..=1000 => OrganizationSize::MediumBusiness(employee_count),
            1001..=5000 => OrganizationSize::LargeEnterprise(employee_count),
            _ => OrganizationSize::Enterprise(employee_count),
        }
    }
}

/// 🧠 SuperAgent - Orchestrates everything based on ontology
pub struct SuperAgent {
    pub ontology: OrganizationOntology,
    pub capabilities: Vec<AgentCapability>,
    pub current_tasks: Vec<AgentTask>,
    pub knowledge_base: KnowledgeBase,
    pub performance_metrics: PerformanceMetrics,
}

/// 🛠️ Agent Capability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCapability {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub category: CapabilityCategory,
    pub required_data: Vec<Uuid>, // Data entity IDs
    pub required_systems: Vec<Uuid>, // System IDs
    pub complexity: ComplexityLevel,
    pub execution_time_estimate: u32, // minutes
}

/// 📋 Capability Category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CapabilityCategory {
    DataAnalysis,
    ProcessAutomation,
    ReportGeneration,
    DecisionSupport,
    IntegrationOrchestration,
    Monitoring,
    Alerting,
    Optimization,
    Prediction,
    Recommendation,
}

/// 📊 Complexity Level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplexityLevel {
    Simple,
    Moderate,
    Complex,
    VeryComplex,
}

/// 📝 Agent Task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentTask {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub capability_id: Uuid,
    pub status: TaskStatus,
    pub priority: Priority,
    pub assigned_to: Option<Uuid>, // User ID
    pub deadline: Option<DateTime<Utc>>,
    pub dependencies: Vec<Uuid>,
    pub progress: f64,
    pub result: Option<TaskResult>,
}

/// 📈 Task Status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Blocked,
    Completed,
    Failed,
    Cancelled,
}

/// 🔝 Priority
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

/// ✅ Task Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    pub success: bool,
    pub output: serde_json::Value,
    pub error_message: Option<String>,
    pub execution_time_seconds: u32,
    pub resources_used: ResourcesUsed,
}

/// 💾 Resources Used
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourcesUsed {
    pub cpu_seconds: f64,
    pub memory_mb: f64,
    pub network_mb: f64,
    pub api_calls: u32,
}

/// 🧠 Knowledge Base
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeBase {
    pub facts: Vec<Fact>,
    pub rules: Vec<Rule>,
    pub patterns: Vec<Pattern>,
    pub decisions: Vec<Decision>,
    pub learnings: Vec<Learning>,
}

/// 📚 Fact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fact {
