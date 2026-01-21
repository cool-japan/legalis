//! Core types for EU AI Act (Regulation EU 2024/1689)
//!
//! This module defines the fundamental types for EU AI regulation.

use chrono::{DateTime, Utc};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "schema")]
use schemars::JsonSchema;

/// AI System definition (Article 3)
///
/// A machine-based system designed to operate with varying levels of autonomy
/// and that may exhibit adaptiveness after deployment and that, for explicit or implicit objectives,
/// infers, from the input it receives, how to generate outputs such as predictions, content,
/// recommendations, or decisions that can influence physical or virtual environments.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct AiSystem {
    /// Unique identifier for the AI system
    pub system_id: String,
    /// Name of the AI system
    pub name: String,
    /// Description of the system's purpose and functionality
    pub description: String,
    /// Provider of the AI system
    pub provider: String,
    /// Deployer of the AI system (may be different from provider)
    pub deployer: Option<String>,
    /// Intended purpose
    pub intended_purpose: String,
    /// Risk level classification
    pub risk_level: RiskLevel,
    /// Whether system exhibits adaptiveness after deployment
    pub adaptive: bool,
    /// Date system was placed on the market
    pub market_placement_date: Option<DateTime<Utc>>,
    /// Conformity assessment status
    pub conformity_status: ConformityStatus,
}

/// Risk level classification under AI Act
///
/// AI systems are classified into different risk categories with corresponding obligations.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum RiskLevel {
    /// Unacceptable risk - Prohibited AI practices (Article 5)
    /// These systems pose a threat to safety, livelihoods, and rights
    Unacceptable {
        /// The prohibited practice
        prohibited_practice: ProhibitedPractice,
    },

    /// High-risk AI systems (Article 6, Annex III)
    /// Subject to strict requirements before market placement
    HighRisk {
        /// Category of high-risk system
        category: HighRiskCategory,
    },

    /// Limited risk - Transparency obligations (Article 52)
    /// Must inform users they are interacting with AI
    LimitedRisk {
        /// Type of limited risk system
        system_type: LimitedRiskType,
    },

    /// Minimal risk - No specific obligations
    /// General-purpose AI with minimal risks
    MinimalRisk,
}

/// Prohibited AI practices (Article 5)
///
/// AI systems that pose unacceptable risks are prohibited in the EU.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum ProhibitedPractice {
    /// Subliminal manipulation causing harm
    SubliminalManipulation,

    /// Exploitation of vulnerabilities (age, disability)
    VulnerabilityExploitation {
        /// Type of vulnerability exploited
        vulnerability_type: String,
    },

    /// Social scoring by public authorities
    SocialScoring,

    /// Real-time remote biometric identification in public spaces by law enforcement
    /// (subject to limited exceptions)
    RealTimeBiometricIdentification {
        /// Whether exception applies
        exception_applies: bool,
    },

    /// Biometric categorization based on sensitive attributes
    SensitiveBiometricCategorization,

    /// Emotion recognition in workplace or education
    EmotionRecognitionWorkplaceEducation {
        /// Context (workplace or education)
        context: String,
    },

    /// Indiscriminate scraping of facial images
    FacialImageScraping,

    /// Inferring emotions in law enforcement (except medical/safety)
    EmotionInferenceLawEnforcement,
}

/// High-risk AI system categories (Article 6, Annex III)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum HighRiskCategory {
    /// Biometric identification and categorization
    BiometricIdentification,

    /// Critical infrastructure management (transport, energy, water)
    CriticalInfrastructure {
        /// Type of infrastructure
        infrastructure_type: String,
    },

    /// Education and vocational training
    EducationVocationalTraining {
        /// Specific use case
        use_case: String,
    },

    /// Employment, worker management, self-employment
    Employment {
        /// Specific use case (recruitment, promotion, etc.)
        use_case: String,
    },

    /// Essential private and public services
    EssentialServices {
        /// Type of service
        service_type: String,
    },

    /// Law enforcement
    LawEnforcement {
        /// Specific use case
        use_case: String,
    },

    /// Migration, asylum, border control
    MigrationAsylumBorderControl {
        /// Specific use case
        use_case: String,
    },

    /// Administration of justice and democratic processes
    JusticeDemocraticProcesses {
        /// Specific use case
        use_case: String,
    },
}

/// Limited risk system types (Article 52)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum LimitedRiskType {
    /// Chatbots and conversational AI
    Chatbot,

    /// Emotion recognition systems
    EmotionRecognition,

    /// Biometric categorization systems
    BiometricCategorization,

    /// Deep fake generation
    DeepFake,
}

/// Conformity assessment status
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum ConformityStatus {
    /// Not yet assessed
    NotAssessed,

    /// Assessment in progress
    InProgress {
        /// Expected completion date
        expected_completion: DateTime<Utc>,
    },

    /// Conformity established
    Conformant {
        /// Assessment completion date
        assessment_date: DateTime<Utc>,
        /// Notified body ID (if external assessment)
        notified_body_id: Option<String>,
    },

    /// Non-conformant
    NonConformant {
        /// Issues identified
        issues: Vec<String>,
    },
}

/// Requirements for high-risk AI systems (Chapter 2)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct HighRiskRequirements {
    /// Risk management system (Article 9)
    pub risk_management: RiskManagementSystem,

    /// Data governance (Article 10)
    pub data_governance: DataGovernance,

    /// Technical documentation (Article 11)
    pub technical_documentation: TechnicalDocumentation,

    /// Record-keeping (Article 12)
    pub record_keeping: RecordKeeping,

    /// Transparency and user information (Article 13)
    pub transparency: TransparencyRequirements,

    /// Human oversight (Article 14)
    pub human_oversight: HumanOversight,

    /// Accuracy, robustness, cybersecurity (Article 15)
    pub accuracy_robustness: AccuracyRobustness,
}

/// Risk management system (Article 9)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct RiskManagementSystem {
    /// Continuous iterative process implemented
    pub continuous_process: bool,
    /// Risk identification and analysis
    pub identified_risks: Vec<IdentifiedRisk>,
    /// Risk mitigation measures
    pub mitigation_measures: Vec<String>,
    /// Residual risk evaluation
    pub residual_risk_acceptable: bool,
    /// Testing procedures
    pub testing_procedures: Vec<String>,
}

/// Identified risk in AI system
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct IdentifiedRisk {
    /// Risk description
    pub description: String,
    /// Risk severity (low, medium, high, critical)
    pub severity: RiskSeverity,
    /// Likelihood of occurrence
    pub likelihood: RiskLikelihood,
    /// Affected stakeholders
    pub affected_stakeholders: Vec<String>,
}

/// Risk severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum RiskSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Risk likelihood
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum RiskLikelihood {
    Rare,
    Unlikely,
    Possible,
    Likely,
    AlmostCertain,
}

/// Data governance requirements (Article 10)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct DataGovernance {
    /// Training data quality criteria
    pub training_data_quality: DataQuality,
    /// Validation data quality
    pub validation_data_quality: DataQuality,
    /// Testing data quality
    pub testing_data_quality: DataQuality,
    /// Data examination for biases
    pub bias_examination: BiasExamination,
    /// Data relevance and representativeness
    pub data_representativeness: String,
}

/// Data quality assessment
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct DataQuality {
    /// Appropriate statistical properties
    pub appropriate_statistical_properties: bool,
    /// Free from errors and complete
    pub complete_and_error_free: bool,
    /// Relevant, representative, free of bias
    pub representative: bool,
}

/// Bias examination (Article 10(2)(f))
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct BiasExamination {
    /// Whether bias examination conducted
    pub conducted: bool,
    /// Identified biases
    pub identified_biases: Vec<String>,
    /// Mitigation measures for biases
    pub bias_mitigation: Vec<String>,
}

/// Technical documentation (Article 11)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct TechnicalDocumentation {
    /// General description of AI system
    pub general_description: String,
    /// Detailed description of elements and development process
    pub detailed_description: String,
    /// Information about data used
    pub data_description: String,
    /// Monitoring, functioning, control mechanisms
    pub monitoring_mechanisms: String,
    /// Validation and testing procedures
    pub validation_testing: String,
    /// Modifications and updates
    pub modification_log: Vec<Modification>,
}

/// System modification record
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Modification {
    /// Modification date
    pub date: DateTime<Utc>,
    /// Description of modification
    pub description: String,
    /// Version number
    pub version: String,
    /// Whether requires new conformity assessment
    pub requires_new_assessment: bool,
}

/// Record-keeping requirements (Article 12)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct RecordKeeping {
    /// Automatic logging capability
    pub automatic_logging: bool,
    /// Log retention period
    pub retention_period_months: u32,
    /// Events logged
    pub logged_events: Vec<LoggedEvent>,
}

/// Event types logged by AI system
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum LoggedEvent {
    /// System activation/deactivation
    SystemOperation,
    /// Input data
    InputData,
    /// Output/decision
    Output,
    /// Human oversight actions
    HumanOversightAction,
    /// Anomalies and malfunctions
    Anomaly,
}

/// Transparency requirements (Article 13)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct TransparencyRequirements {
    /// Instructions for use provided
    pub instructions_for_use: String,
    /// Information concise, complete, correct, clear
    pub information_quality: bool,
    /// Reasonably foreseeable misuse identified
    pub foreseeable_misuse: Vec<String>,
    /// Level of accuracy disclosed
    pub accuracy_metrics: Vec<String>,
}

/// Human oversight requirements (Article 14)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct HumanOversight {
    /// Human oversight measures implemented
    pub measures: Vec<HumanOversightMeasure>,
    /// Whether humans can interpret system outputs
    pub output_interpretable: bool,
    /// Whether humans can override or stop system
    pub override_capability: bool,
}

/// Human oversight measure types
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum HumanOversightMeasure {
    /// Human-in-the-loop (HITL)
    HumanInTheLoop,
    /// Human-on-the-loop (HOTL)
    HumanOnTheLoop,
    /// Human-in-command (HIC)
    HumanInCommand,
}

/// Accuracy, robustness, cybersecurity (Article 15)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct AccuracyRobustness {
    /// Accuracy level achieved
    pub accuracy_level: f64,
    /// Accuracy metrics used
    pub accuracy_metrics: Vec<String>,
    /// Robustness to errors and inconsistencies
    pub robustness_measures: Vec<String>,
    /// Cybersecurity measures
    pub cybersecurity_measures: Vec<String>,
    /// Resilience to adversarial attacks
    pub adversarial_robustness: bool,
}

/// General-purpose AI model (Article 51)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct GeneralPurposeAiModel {
    /// Model name
    pub name: String,
    /// Provider
    pub provider: String,
    /// Training data summary
    pub training_data_summary: String,
    /// Capabilities and limitations
    pub capabilities: String,
    /// Whether model poses systemic risk
    pub systemic_risk: bool,
    /// Transparency documentation provided
    pub transparency_documentation: bool,
}

/// Transparency obligation for limited risk systems (Article 52)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct TransparencyObligation {
    /// System type
    pub system_type: LimitedRiskType,
    /// Whether users are informed of AI interaction
    pub users_informed: bool,
    /// Method of notification
    pub notification_method: String,
    /// For deep fakes: content marked as artificially generated
    pub content_marked: bool,
}

/// AI literacy requirements (Article 4)
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct AiLiteracy {
    /// Training programs for staff
    pub staff_training: Vec<String>,
    /// Understanding of AI capabilities and limitations
    pub capabilities_understanding: bool,
    /// Understanding of appropriate use
    pub appropriate_use_understanding: bool,
}
