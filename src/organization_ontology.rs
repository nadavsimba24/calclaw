// 🏢 Organization Ontology Module
// Defines the structure for understanding organizational data

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// 🏢 Organization Profile - Basic company information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationProfile {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub industry: String,
    pub size: OrganizationSize,
    pub location: String,
    pub timezone: String,
    pub language: String,
    pub website: Option<String>,
    pub founded_year: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 📊 Organization Size
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrganizationSize {
    Startup(usize),      // 1-50 employees
    SmallBusiness(usize), // 51-200 employees
    MediumBusiness(usize), // 201-1000 employees
    LargeEnterprise(usize), // 1001-5000 employees
    Enterprise(usize),   // 5000+ employees
}

/// 👥 Department/Team Structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Department {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub parent_department_id: Option<Uuid>,
    pub manager_id: Option<Uuid>,
    pub employee_count: usize,
    pub responsibilities: Vec<String>,
    pub key_processes: Vec<Process>,
}

/// 🔄 Business Process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Process {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub department_id: Uuid,
    pub steps: Vec<ProcessStep>,
    pub inputs: Vec<DataEntity>,
    pub outputs: Vec<DataEntity>,
    pub frequency: ProcessFrequency,
    pub automation_level: AutomationLevel,
}

/// 📝 Process Step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessStep {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub order: usize,
    pub role: String,
    pub tools: Vec<String>,
    pub estimated_time_minutes: u32,
    pub dependencies: Vec<Uuid>,
}

/// 📊 Data Entity - Represents a piece of data in the organization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataEntity {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub data_type: DataType,
    pub format: DataFormat,
    pub source: DataSource,
    pub sensitivity: DataSensitivity,
    pub retention_days: Option<u32>,
    pub owners: Vec<Uuid>, // User IDs
    pub relationships: Vec<DataRelationship>,
}

/// 🔗 Data Relationship
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRelationship {
    pub source_entity_id: Uuid,
    pub target_entity_id: Uuid,
    pub relationship_type: RelationshipType,
    pub cardinality: Cardinality,
    pub description: String,
}

/// 🏷️ Data Type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataType {
    CustomerData,
    ProductData,
    FinancialData,
    EmployeeData,
    OperationalData,
    MarketingData,
    SalesData,
    SupportData,
    TechnicalData,
    Custom(String),
}

/// 📁 Data Format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataFormat {
    DatabaseTable,
    CSVFile,
    JSONFile,
    ExcelFile,
    APIRest,
    GraphQL,
    RealTimeStream,
    Document,
    Email,
    Custom(String),
}

/// 📍 Data Source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataSource {
    InternalSystem(String),
    ExternalAPI(String),
    ManualEntry,
    IoTDevice,
    MobileApp,
    WebApp,
    LegacySystem,
    CloudService(String),
}

/// 🔒 Data Sensitivity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataSensitivity {
    Public,
    Internal,
    Confidential,
    Restricted,
    Secret,
}

/// 🔗 Relationship Type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationshipType {
    ParentChild,
    OneToOne,
    OneToMany,
    ManyToMany,
    Dependency,
    Reference,
    Aggregation,
    Composition,
    Association,
}

/// 🔢 Cardinality
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Cardinality {
    OneToOne,
    OneToMany,
    ManyToOne,
    ManyToMany,
    ZeroOrOne,
    ZeroOrMany,
    OneOrMore,
}

/// 🔄 Process Frequency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcessFrequency {
    RealTime,
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    Yearly,
    OnDemand,
    EventDriven,
}

/// 🤖 Automation Level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AutomationLevel {
    Manual,
    SemiAutomated,
    FullyAutomated,
    AIEnhanced,
}

/// 👤 User Role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRole {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub permissions: Vec<Permission>,
    pub data_access: Vec<DataAccess>,
    pub department_id: Uuid,
}

/// 🔐 Permission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub resource: String,
    pub action: Action,
    pub conditions: Vec<Condition>,
}

/// ⚡ Action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    Create,
    Read,
    Update,
    Delete,
    Execute,
    Approve,
    Delegate,
    Monitor,
}

/// ⚙️ Condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub field: String,
    pub operator: Operator,
    pub value: serde_json::Value,
}

/// 🔧 Operator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    Contains,
    StartsWith,
    EndsWith,
    In,
    NotIn,
}

/// 📊 Data Access
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataAccess {
    pub data_entity_id: Uuid,
    pub access_level: AccessLevel,
    pub filters: Option<Vec<Condition>>,
}

/// 🔓 Access Level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessLevel {
    None,
    ReadOnly,
    ReadWrite,
    FullControl,
    Admin,
}

/// 🧠 Organization Ontology - Complete understanding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationOntology {
    pub profile: OrganizationProfile,
    pub departments: Vec<Department>,
    pub processes: Vec<Process>,
    pub data_entities: Vec<DataEntity>,
    pub user_roles: Vec<UserRole>,
    pub systems: Vec<System>,
    pub integrations: Vec<Integration>,
    pub goals: Vec<Goal>,
    pub metrics: Vec<Metric>,
}

/// 💻 System
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct System {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub system_type: SystemType,
    pub vendor: Option<String>,
    pub version: String,
    pub data_entities: Vec<Uuid>,
    pub integrations: Vec<Uuid>,
    pub status: SystemStatus,
}

/// 🖥️ System Type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemType {
    CRM,
    ERP,
    HRMS,
    Accounting,
    ProjectManagement,
    Communication,
    DocumentManagement,
    BusinessIntelligence,
    Custom(String),
}

/// 🟢 System Status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemStatus {
    Active,
    Inactive,
    Deprecated,
    Planned,
    UnderDevelopment,
}

/// 🔗 Integration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Integration {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub source_system_id: Uuid,
    pub target_system_id: Uuid,
    pub integration_type: IntegrationType,
    pub frequency: ProcessFrequency,
    pub data_flow: DataFlow,
    pub status: IntegrationStatus,
}

/// 🔄 Integration Type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntegrationType {
    APIRest,
    GraphQL,
    Webhook,
    FileTransfer,
    DatabaseSync,
    MessageQueue,
    EventBus,
    Custom(String),
}

/// 📊 Data Flow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataFlow {
    pub source_data_entities: Vec<Uuid>,
    pub target_data_entities: Vec<Uuid>,
    pub transformation_rules: Vec<TransformationRule>,
    pub error_handling: ErrorHandling,
}

/// 🔄 Transformation Rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformationRule {
    pub source_field: String,
    pub target_field: String,
    pub transformation: Transformation,
    pub validation: Option<ValidationRule>,
}

/// 🪄 Transformation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Transformation {
    DirectMapping,
    Concatenate(Vec<String>),
    Split(String),
    Format(String),
    Calculate(String),
    Lookup(Uuid),
    Custom(String),
}

/// ✅ Validation Rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    pub rule_type: ValidationType,
    pub parameters: serde_json::Value,
    pub error_message: String,
}

/// 📋 Validation Type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationType {
    Required,
    MinLength(usize),
    MaxLength(usize),
    Pattern(String),
    Range(f64, f64),
    InList(Vec<String>),
    Custom(String),
}

/// ❌ Error Handling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorHandling {
    pub retry_count: u32,
    pub retry_delay_seconds: u32,
    pub fallback_action: FallbackAction,
    pub notification_users: Vec<Uuid>,
}

/// 🆘 Fallback Action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FallbackAction {
    Skip,
    UseDefault,
    ManualReview,
    StopProcess,
    NotifyAndContinue,
}

/// 🎯 Goal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub department_id: Uuid,
    pub parent_goal_id: Option<Uuid>,
    pub target_date: Option<DateTime<Utc>>,
    pub metrics: Vec<Uuid>,
    pub status: GoalStatus,
    pub progress: f64,
}

/// 📈 Goal Status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GoalStatus {
    NotStarted,
    InProgress,
    AtRisk,
    Completed,
    Cancelled,
}

/// 📊 Metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metric {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub data_entity_id: Uuid,
    pub calculation: Calculation,
    pub frequency: ProcessFrequency,
    pub target_value: Option<f64>,
    pub unit: String,
    pub visualization: Visualization,
}

/// 🧮 Calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Calculation {
    pub formula: String,
    pub data_points: Vec<DataPoint>,
    pub filters: Vec<Condition>,
    pub time_range: Option<TimeRange>,
}

/// 📍 Data Point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPoint {
    pub data_entity_id: Uuid,
    pub field: String,
    pub aggregation: Aggregation,
}

/// 📊 Aggregation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Aggregation {
    Sum,
    Average,
    Count,
    Min,
    Max,
    DistinctCount,
    Median,
    StandardDeviation,
}

/// ⏰ Time Range
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub granularity: Granularity,
}

/// 📅 Granularity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Granularity {
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    Yearly,
}

/// 📊 Visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Visualization {
    pub chart_type: ChartType,
    pub dimensions: Vec<String>,
    pub measures: Vec<String>,
    pub filters: Vec<Condition>,
    pub options: serde_json::Value,
}

/// 📈 Chart Type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChartType {
    Line,
    Bar,
    Pie,
    Scatter,
    Area,
    Table,
    Gauge,
    Heatmap,
    TreeMap,
    Custom(String),
}

/// 🧠 Ontology Manager
pub struct OntologyManager {
    pub ontology: OrganizationOntology,
    pub version: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl OntologyManager {
    /// Create a new ontology manager
    pub fn new(profile: OrganizationProfile) -> Self {
        Self {
            ontology: OrganizationOntology {
                profile,
                departments: Vec::new(),
                processes: Vec::new(),
                data_entities: Vec::new(),
                user_roles: Vec::new(),
                systems: Vec::new(),
                integrations: Vec::new(),
                goals: Vec::new(),
                metrics: Vec::new(),
            },
            version: "1.0.0".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
    
    /// Add a department to the ontology
    pub fn add_department(&mut self, department: Department) {
        self.ontology.departments.push(department);
        self.updated_at = Utc::now();
    }
    
    /// Add a process to the ontology
    pub fn add_process(&mut self, process: Process) {
        self.ontology.processes.push(process);
        self.updated_at = Utc::now();
    }
    
    /// Add a data entity to the ontology
    pub fn add_data_entity(&mut self, data_entity: DataEntity) {
        self.ontology.data_entities.push(data_entity);
        self.updated_at = Utc::now();
    }
    
    /// Find data entity by name
    pub fn find_data_entity(&self, name: &str) -> Option<&DataEntity> {
        self.ontology.data_entities.iter().find(|de| de.name == name)
    }
    
    /// Get all processes for a department
    pub fn get_department_processes(&self, department_id: Uuid) -> Vec<&Process> {
        self.ontology.processes.iter()
            .filter(|p| p.department_id == department_id)
            .collect()
    }
    
    /// Export ontology to JSON
    pub fn export_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.ontology)
    }
    
    /// Import ontology from JSON
    pub fn import_json(json: &str) -> Result<Self, serde_json::Error> {
        let ontology: OrganizationOntology = serde_json::from_str(json)?;
        Ok(Self {
            ontology,
            version: "1.0.0".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }
    
    /// Generate a summary of the ontology
    pub fn generate_summary(&self) -> String {
        format!(
            "Organization: {}\n\
             Departments: {}\n\
             Processes: {}\n\
             Data Entities: {}\n\
             Systems: {}\n\
             Integrations: {}\n\
             Goals: {}\n\
             Metrics: {}",
            self.ontology.profile.name,
            self.ontology.departments.len(),
            self.ontology.processes.len(),
            self.ontology.data_entities.len(),
            self.ontology.systems.len(),
            self.ontology.integrations.len(),
            self.ontology.goals.len(),
            self.ontology.metrics.len()
        )
    }
}

/// 🎯 Question for organization onboarding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnboardingQuestion {
    pub id: Uuid,
    pub category: QuestionCategory,
    pub question: String,
    pub description: String,
    pub answer_type: AnswerType,
    pub required: bool,
    pub options: Option<Vec<String>>,
    pub depends_on: Option<Uuid>,
    pub validation: Option<ValidationRule>,
}

/// 📋 Question Category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuestionCategory {
    Organization