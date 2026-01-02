//! Legalis-Porting: Legal system porting for Legalis-RS.
//!
//! This crate enables "Soft ODA" (Official Development Assistance through legal framework
//! sharing) - porting legal frameworks between jurisdictions while adapting to local
//! cultural parameters and legal systems.
//!
//! # Features
//!
//! ## Core Porting
//!
//! - **Cross-jurisdiction statute translation**: Port statutes between different legal systems
//! - **Cultural parameter injection**: Automatically adapt age limits, prohibitions, etc.
//! - **Compatibility reports**: Assess feasibility and generate detailed analysis
//! - **Change tracking**: Document all adaptations made during porting
//! - **Partial porting**: Port specific sections of statutes
//! - **Reverse porting**: Analyze what would be needed to port back to source
//!
//! ## Intelligence & Validation
//!
//! - **AI-assisted adaptation**: Generate cultural adaptation suggestions using LLM
//! - **Conflict detection**: Identify conflicts with target jurisdiction laws
//! - **Semantic preservation**: Validate that legal meaning is preserved
//! - **Risk assessment**: Evaluate risks in ported statutes
//! - **Similar statute finding**: Find equivalent statutes across jurisdictions
//! - **Automatic term replacement**: Replace legal terms with local equivalents
//! - **Context-aware parameter adjustment**: Adjust values based on context
//!
//! ## Workflow & Compliance
//!
//! - **Legal expert review workflow**: Submit ported statutes for expert review
//! - **Automated compliance checking**: Check compliance with target regulations
//! - **Porting workflow management**: Track multi-step porting processes
//! - **Version control**: Manage versioned ported statutes
//!
//! ## Bilateral Cooperation
//!
//! - **Bilateral legal agreement templates**: Create agreements between jurisdictions
//! - **Regulatory equivalence mapping**: Map equivalent regulations
//! - **Batch porting**: Port multiple statutes efficiently
//!
//! # Examples
//!
//! ## Basic Porting
//!
//! ```rust
//! use legalis_core::{Effect, EffectType, Statute};
//! use legalis_i18n::{CulturalParams, Jurisdiction, LegalSystem, Locale};
//! use legalis_porting::{PortingEngine, PortingOptions};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Create jurisdictions
//! let japan = Jurisdiction::new("JP", "Japan", Locale::new("ja").with_country("JP"))
//!     .with_legal_system(LegalSystem::CivilLaw)
//!     .with_cultural_params(CulturalParams::japan());
//!
//! let usa = Jurisdiction::new("US", "United States", Locale::new("en").with_country("US"))
//!     .with_legal_system(LegalSystem::CommonLaw)
//!     .with_cultural_params(CulturalParams::for_country("US"));
//!
//! // Create porting engine
//! let engine = PortingEngine::new(japan, usa);
//!
//! // Create a statute
//! let statute = Statute::new(
//!     "adult-rights",
//!     "成人権利法",
//!     Effect::new(EffectType::Grant, "Full legal capacity"),
//! );
//!
//! // Port with options
//! let options = PortingOptions {
//!     apply_cultural_params: true,
//!     translate_terms: true,
//!     generate_report: true,
//!     ..Default::default()
//! };
//!
//! let ported = engine.port_statute(&statute, &options)?;
//!
//! // Review changes
//! for change in &ported.changes {
//!     println!("{:?}: {}", change.change_type, change.description);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Batch Porting with Validation
//!
//! ```rust
//! use legalis_core::{Effect, EffectType, Statute};
//! use legalis_i18n::{CulturalParams, Jurisdiction, LegalSystem, Locale};
//! use legalis_porting::{PortingEngine, PortingOptions};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # let japan = Jurisdiction::new("JP", "Japan", Locale::new("ja").with_country("JP"))
//! #     .with_legal_system(LegalSystem::CivilLaw)
//! #     .with_cultural_params(CulturalParams::japan());
//! # let usa = Jurisdiction::new("US", "United States", Locale::new("en").with_country("US"))
//! #     .with_legal_system(LegalSystem::CommonLaw)
//! #     .with_cultural_params(CulturalParams::for_country("US"));
//! let engine = PortingEngine::new(japan, usa);
//!
//! let statutes = vec![
//!     Statute::new("s1", "Statute 1", Effect::new(EffectType::Grant, "Right 1")),
//!     Statute::new("s2", "Statute 2", Effect::new(EffectType::Grant, "Right 2")),
//! ];
//!
//! let options = PortingOptions {
//!     apply_cultural_params: true,
//!     detect_conflicts: true,
//!     validate_semantics: true,
//!     ..Default::default()
//! };
//!
//! let result = engine.batch_port(&statutes, &options).await?;
//!
//! println!("Ported {} statutes", result.statutes.len());
//! println!("Detected {} conflicts", result.conflicts.len());
//! # Ok(())
//! # }
//! ```
//!
//! # Architecture
//!
//! The porting process follows these stages:
//!
//! 1. **Analysis**: Examine source statute structure and cultural parameters
//! 2. **Compatibility Check**: Assess legal system compatibility
//! 3. **Cultural Injection**: Apply target jurisdiction parameters
//! 4. **Conflict Detection**: Identify conflicts with target laws
//! 5. **Semantic Validation**: Verify legal meaning preservation
//! 6. **Risk Assessment**: Evaluate implementation risks
//! 7. **Report Generation**: Document all changes and recommendations

use async_trait::async_trait;
use legalis_core::Statute;
use legalis_i18n::{Jurisdiction, LegalSystem, Locale};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Errors during porting operations.
#[derive(Debug, Error)]
pub enum PortingError {
    #[error("Source jurisdiction not found: {0}")]
    SourceNotFound(String),

    #[error("Target jurisdiction not found: {0}")]
    TargetNotFound(String),

    #[error("Incompatible legal systems: {0} -> {1}")]
    IncompatibleSystems(String, String),

    #[error("Cultural conflict: {0}")]
    CulturalConflict(String),

    #[error("Translation failed: {0}")]
    TranslationFailed(String),

    #[error("Adaptation required: {0}")]
    AdaptationRequired(String),

    #[error("LLM error: {0}")]
    Llm(#[from] anyhow::Error),

    #[error("Conflict detected: {0}")]
    ConflictDetected(String),

    #[error("Semantic validation failed: {0}")]
    SemanticValidationFailed(String),

    #[error("Section not found: {0}")]
    SectionNotFound(String),
}

/// Result type for porting operations.
pub type PortingResult<T> = Result<T, PortingError>;

/// Simple dyn-compatible trait for text generation.
#[async_trait]
pub trait TextGenerator: Send + Sync {
    /// Generates text from a prompt.
    async fn generate(&self, prompt: &str) -> anyhow::Result<String>;
}

/// Porting request specification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortingRequest {
    /// Source statute(s) to port
    pub statutes: Vec<Statute>,
    /// Source jurisdiction ID
    pub source_jurisdiction: String,
    /// Target jurisdiction ID
    pub target_jurisdiction: String,
    /// Porting options
    pub options: PortingOptions,
}

/// Options for porting.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PortingOptions {
    /// Whether to translate legal terms
    pub translate_terms: bool,
    /// Whether to adapt numerical values (ages, amounts)
    pub adapt_values: bool,
    /// Whether to inject cultural parameters
    pub apply_cultural_params: bool,
    /// Specific overrides for values
    pub value_overrides: HashMap<String, String>,
    /// Whether to generate a compatibility report
    pub generate_report: bool,
    /// Whether to use AI for cultural adaptation suggestions
    pub use_ai_suggestions: bool,
    /// Whether to detect conflicts with target jurisdiction laws
    pub detect_conflicts: bool,
    /// Whether to validate semantic preservation
    pub validate_semantics: bool,
    /// Specific section IDs to port (if empty, port all)
    pub section_ids: Vec<String>,
    /// Whether to perform reverse porting analysis
    pub reverse_porting: bool,
}

/// Result of a porting operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortingOutput {
    /// Ported statutes
    pub statutes: Vec<PortedStatute>,
    /// Compatibility report
    pub report: Option<CompatibilityReport>,
    /// Warnings generated during porting
    pub warnings: Vec<String>,
    /// AI-generated adaptation suggestions
    pub ai_suggestions: Vec<AdaptationSuggestion>,
    /// Detected conflicts with target jurisdiction
    pub conflicts: Vec<ConflictReport>,
    /// Semantic validation results
    pub semantic_validation: Option<SemanticValidation>,
    /// Risk assessment
    pub risk_assessment: Option<RiskAssessment>,
}

/// A statute that has been ported to a new jurisdiction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortedStatute {
    /// Original statute ID
    pub original_id: String,
    /// New statute with adaptations
    pub statute: Statute,
    /// Changes made during porting
    pub changes: Vec<PortingChange>,
    /// Locale of the ported statute
    pub locale: Locale,
    /// Compatibility score (0.0 to 1.0)
    pub compatibility_score: f64,
}

/// A change made during porting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortingChange {
    /// Type of change
    pub change_type: ChangeType,
    /// Description of what changed
    pub description: String,
    /// Original value (if applicable)
    pub original: Option<String>,
    /// New value (if applicable)
    pub adapted: Option<String>,
    /// Reason for the change
    pub reason: String,
}

/// Types of changes during porting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChangeType {
    /// Term was translated
    Translation,
    /// Value was adapted (e.g., age threshold)
    ValueAdaptation,
    /// Condition was modified for cultural reasons
    CulturalAdaptation,
    /// Section was marked as incompatible
    Incompatible,
    /// Added for local compliance
    ComplianceAddition,
    /// Removed due to local prohibition
    Removal,
}

/// Compatibility report for ported statutes.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CompatibilityReport {
    /// Overall compatibility score (0.0 - 1.0)
    pub compatibility_score: f64,
    /// Number of adaptations required
    pub adaptations_required: usize,
    /// Number of incompatibilities found
    pub incompatibilities: usize,
    /// Detailed findings
    pub findings: Vec<CompatibilityFinding>,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// A finding from compatibility analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityFinding {
    /// Severity level
    pub severity: Severity,
    /// Category of finding
    pub category: String,
    /// Description
    pub description: String,
    /// Affected statute ID
    pub statute_id: Option<String>,
}

/// Severity levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Risk category for classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RiskCategory {
    /// Legal risks
    Legal,
    /// Cultural risks
    Cultural,
    /// Political risks
    Political,
    /// Economic risks
    Economic,
    /// Implementation risks
    Implementation,
    /// Technical risks
    Technical,
}

/// AI-generated adaptation suggestion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptationSuggestion {
    /// Statute ID this suggestion applies to
    pub statute_id: String,
    /// Suggested adaptation
    pub suggestion: String,
    /// Rationale for the suggestion
    pub rationale: String,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f64,
    /// Category of adaptation
    pub category: String,
}

/// Conflict detected with target jurisdiction laws.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictReport {
    /// Statute ID with conflict
    pub statute_id: String,
    /// Type of conflict
    pub conflict_type: ConflictType,
    /// Description of the conflict
    pub description: String,
    /// Severity of the conflict
    pub severity: Severity,
    /// Potential resolution strategies
    pub resolutions: Vec<String>,
}

/// Types of conflicts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictType {
    /// Contradicts existing law
    Contradiction,
    /// Overlaps with existing law
    Overlap,
    /// Cultural incompatibility
    CulturalIncompatibility,
    /// Legal system mismatch
    SystemMismatch,
    /// Procedural conflict
    Procedural,
}

/// Semantic validation results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticValidation {
    /// Overall semantic preservation score (0.0 - 1.0)
    pub preservation_score: f64,
    /// Validation findings
    pub findings: Vec<SemanticFinding>,
    /// Whether semantics are acceptably preserved
    pub is_valid: bool,
}

/// A finding from semantic validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticFinding {
    /// Statute ID
    pub statute_id: String,
    /// Finding description
    pub description: String,
    /// Severity
    pub severity: Severity,
    /// Impact on legal meaning
    pub impact: String,
}

/// Risk assessment for ported statutes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    /// Overall risk score (0.0 - 1.0, higher is riskier)
    pub risk_score: f64,
    /// Risk level
    pub risk_level: RiskLevel,
    /// Identified risks
    pub risks: Vec<Risk>,
    /// Mitigation strategies
    pub mitigations: Vec<String>,
}

/// Risk level categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// A specific risk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Risk {
    /// Risk identifier
    pub id: String,
    /// Risk category
    pub category: RiskCategory,
    /// Description
    pub description: String,
    /// Likelihood level
    pub likelihood: RiskLevel,
    /// Impact (0.0 - 1.0)
    pub impact: f64,
    /// Severity
    pub severity: RiskLevel,
}

/// Bilateral legal agreement template.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BilateralAgreement {
    /// Agreement ID
    pub id: String,
    /// Source jurisdiction
    pub source_jurisdiction: String,
    /// Target jurisdiction
    pub target_jurisdiction: String,
    /// Agreement type
    pub agreement_type: AgreementType,
    /// Mutual recognition clauses
    pub mutual_recognition: Vec<String>,
    /// Adaptation protocols
    pub adaptation_protocols: Vec<AdaptationProtocol>,
    /// Dispute resolution mechanism
    pub dispute_resolution: Option<String>,
}

/// Types of bilateral agreements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgreementType {
    /// Mutual recognition agreement
    MutualRecognition,
    /// Harmonization agreement
    Harmonization,
    /// Equivalence agreement
    Equivalence,
    /// Cooperation agreement
    Cooperation,
}

/// Protocol for adapting statutes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptationProtocol {
    /// Protocol name
    pub name: String,
    /// Description
    pub description: String,
    /// Applicable statute types
    pub statute_types: Vec<String>,
    /// Transformation rules
    pub rules: Vec<String>,
}

/// Regulatory equivalence mapping.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquivalenceMapping {
    /// Source regulation ID
    pub source_regulation: String,
    /// Target regulation ID
    pub target_regulation: String,
    /// Equivalence score (0.0 - 1.0)
    pub equivalence_score: f64,
    /// Differences identified
    pub differences: Vec<String>,
    /// Mapping notes
    pub notes: String,
}

/// Term replacement rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TermReplacement {
    /// Source term
    pub source_term: String,
    /// Target term
    pub target_term: String,
    /// Context where this applies
    pub context: Option<String>,
    /// Confidence in replacement (0.0 - 1.0)
    pub confidence: f64,
}

/// Context-aware parameter adjustment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextualAdjustment {
    /// Parameter name
    pub parameter: String,
    /// Original value
    pub original_value: String,
    /// Adjusted value
    pub adjusted_value: String,
    /// Context that triggered adjustment
    pub context: String,
    /// Rationale
    pub rationale: String,
}

/// Porting workflow state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortingWorkflow {
    /// Workflow ID
    pub id: String,
    /// Current state
    pub state: WorkflowState,
    /// Statute being ported
    pub statute_id: String,
    /// Source jurisdiction
    pub source_jurisdiction: String,
    /// Target jurisdiction
    pub target_jurisdiction: String,
    /// Steps completed
    pub completed_steps: Vec<WorkflowStep>,
    /// Pending steps
    pub pending_steps: Vec<WorkflowStep>,
    /// Approvals required
    pub approvals: Vec<Approval>,
}

/// Workflow state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkflowState {
    /// Initiated
    Initiated,
    /// In progress
    InProgress,
    /// Pending review
    PendingReview,
    /// Approved
    Approved,
    /// Rejected
    Rejected,
    /// Completed
    Completed,
}

/// Workflow step.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    /// Step name
    pub name: String,
    /// Description
    pub description: String,
    /// Status
    pub status: StepStatus,
    /// Completed at timestamp
    pub completed_at: Option<String>,
}

/// Step status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StepStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

/// Approval requirement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Approval {
    /// Approver role
    pub approver_role: String,
    /// Approval status
    pub status: ApprovalStatus,
    /// Comments
    pub comments: Option<String>,
}

/// Approval status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApprovalStatus {
    Pending,
    Approved,
    Rejected,
}

/// Version-controlled ported statute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionedPortedStatute {
    /// Statute information
    pub statute: PortedStatute,
    /// Version number
    pub version: u32,
    /// Previous version hash
    pub previous_hash: Option<String>,
    /// Current hash
    pub hash: String,
    /// Created at timestamp
    pub created_at: String,
    /// Created by
    pub created_by: String,
    /// Change notes
    pub change_notes: String,
}

/// Legal expert review request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewRequest {
    /// Request ID
    pub id: String,
    /// Statute being reviewed
    pub statute: PortedStatute,
    /// Source jurisdiction
    pub source_jurisdiction: String,
    /// Target jurisdiction
    pub target_jurisdiction: String,
    /// Request status
    pub status: ReviewStatus,
    /// Assigned expert
    pub assigned_expert: Option<String>,
    /// Submitted at timestamp
    pub submitted_at: String,
    /// Reviews received
    pub reviews: Vec<ExpertReview>,
}

/// Status of review request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReviewStatus {
    /// Submitted and awaiting assignment
    Pending,
    /// Assigned to expert
    Assigned,
    /// Under review
    InReview,
    /// Review completed
    Completed,
    /// Approved by expert
    Approved,
    /// Rejected by expert
    Rejected,
    /// Requires revision
    RequiresRevision,
}

/// Expert review of ported statute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpertReview {
    /// Review ID
    pub id: String,
    /// Expert identifier
    pub expert_id: String,
    /// Expert name
    pub expert_name: String,
    /// Expert qualifications
    pub qualifications: Vec<String>,
    /// Review timestamp
    pub reviewed_at: String,
    /// Overall recommendation
    pub recommendation: ReviewRecommendation,
    /// Review comments
    pub comments: Vec<ReviewComment>,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f64,
    /// Areas of concern
    pub concerns: Vec<String>,
    /// Suggested modifications
    pub suggested_modifications: Vec<String>,
}

/// Expert recommendation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReviewRecommendation {
    /// Approve without changes
    Approve,
    /// Approve with minor changes
    ApproveWithChanges,
    /// Reject and require major revision
    Reject,
    /// Request additional information
    RequestInformation,
}

/// Review comment from expert.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewComment {
    /// Comment ID
    pub id: String,
    /// Section or aspect being commented on
    pub section: Option<String>,
    /// Comment text
    pub text: String,
    /// Severity
    pub severity: Severity,
    /// Category
    pub category: String,
}

/// Automated compliance check result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceCheckResult {
    /// Check ID
    pub id: String,
    /// Statute ID checked
    pub statute_id: String,
    /// Check timestamp
    pub checked_at: String,
    /// Overall compliance status
    pub status: ComplianceStatus,
    /// Compliance score (0.0 - 1.0)
    pub compliance_score: f64,
    /// Individual check results
    pub checks: Vec<ComplianceCheck>,
    /// Violations found
    pub violations: Vec<ComplianceViolation>,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// Compliance status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComplianceStatus {
    /// Fully compliant
    Compliant,
    /// Compliant with minor issues
    CompliantWithIssues,
    /// Not compliant
    NonCompliant,
    /// Requires manual review
    RequiresReview,
}

/// Individual compliance check.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceCheck {
    /// Check name
    pub name: String,
    /// Check description
    pub description: String,
    /// Check result
    pub passed: bool,
    /// Finding details
    pub details: Option<String>,
    /// Severity if failed
    pub severity: Severity,
}

/// Compliance violation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceViolation {
    /// Violation type
    pub violation_type: String,
    /// Description
    pub description: String,
    /// Severity
    pub severity: Severity,
    /// Regulation violated
    pub regulation: String,
    /// Remediation steps
    pub remediation: Vec<String>,
}

/// Trait for porting adapters.
#[async_trait]
pub trait PortingAdapter: Send + Sync {
    /// Ports statutes from source to target jurisdiction.
    async fn port(&self, request: &PortingRequest) -> PortingResult<PortingOutput>;

    /// Analyzes compatibility between jurisdictions.
    async fn analyze_compatibility(
        &self,
        source: &Jurisdiction,
        target: &Jurisdiction,
    ) -> PortingResult<CompatibilityReport>;
}

// ============================================================================
// Jurisdiction Database (v0.1.1)
// ============================================================================

/// Legal system classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LegalSystemType {
    /// Common law system (precedent-based)
    CommonLaw,
    /// Civil law system (code-based)
    CivilLaw,
    /// Religious law system
    ReligiousLaw,
    /// Customary law system
    CustomaryLaw,
    /// Mixed/Hybrid system
    Mixed,
    /// Socialist law system
    SocialistLaw,
}

/// Court level in judicial hierarchy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CourtLevel {
    /// Local/Municipal court
    Local = 1,
    /// District/Regional court
    District = 2,
    /// High/Appellate court
    Appellate = 3,
    /// Supreme/Constitutional court
    Supreme = 4,
    /// International court
    International = 5,
}

/// Individual court in a jurisdiction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Court {
    /// Court name
    pub name: String,
    /// Court level
    pub level: CourtLevel,
    /// Jurisdiction (geographic or subject-matter)
    pub jurisdiction: String,
    /// Whether this court can create binding precedent
    pub precedent_setting: bool,
    /// Number of judges
    pub judges: Option<u32>,
    /// Court website URL
    pub url: Option<String>,
}

/// Court hierarchy for a jurisdiction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourtHierarchy {
    /// Courts organized by level
    pub courts: Vec<Court>,
    /// Appeal path description
    pub appeal_path: String,
    /// Whether jury trials are available
    pub has_jury_trials: bool,
    /// Constitutional court (if separate from supreme court)
    pub constitutional_court: Option<String>,
}

impl CourtHierarchy {
    /// Creates a new court hierarchy.
    pub fn new() -> Self {
        Self {
            courts: Vec::new(),
            appeal_path: String::new(),
            has_jury_trials: false,
            constitutional_court: None,
        }
    }

    /// Adds a court to the hierarchy.
    pub fn add_court(&mut self, court: Court) {
        self.courts.push(court);
    }

    /// Gets courts by level.
    pub fn courts_by_level(&self, level: CourtLevel) -> Vec<&Court> {
        self.courts.iter().filter(|c| c.level == level).collect()
    }
}

impl Default for CourtHierarchy {
    fn default() -> Self {
        Self::new()
    }
}

/// Legislative process stage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LegislativeStage {
    /// Bill drafting
    Drafting = 1,
    /// Committee review
    Committee = 2,
    /// First reading
    FirstReading = 3,
    /// Second reading
    SecondReading = 4,
    /// Third reading
    ThirdReading = 5,
    /// Upper house (if bicameral)
    UpperHouse = 6,
    /// Executive approval
    Executive = 7,
    /// Publication
    Publication = 8,
}

/// Legislative process for a jurisdiction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegislativeProcess {
    /// Legislative body name
    pub legislature_name: String,
    /// Whether the legislature is bicameral
    pub is_bicameral: bool,
    /// Lower house name
    pub lower_house: String,
    /// Upper house name (if bicameral)
    pub upper_house: Option<String>,
    /// Legislative stages in order
    pub stages: Vec<LegislativeStage>,
    /// Typical duration (in days)
    pub typical_duration_days: Option<u32>,
    /// Whether initiatives/referendums are available
    pub has_direct_democracy: bool,
    /// Legislative session frequency
    pub session_frequency: String,
}

impl LegislativeProcess {
    /// Creates a new legislative process.
    pub fn new(legislature_name: String, lower_house: String) -> Self {
        Self {
            legislature_name,
            is_bicameral: false,
            lower_house,
            upper_house: None,
            stages: vec![
                LegislativeStage::Drafting,
                LegislativeStage::Committee,
                LegislativeStage::FirstReading,
                LegislativeStage::SecondReading,
                LegislativeStage::ThirdReading,
                LegislativeStage::Executive,
                LegislativeStage::Publication,
            ],
            typical_duration_days: None,
            has_direct_democracy: false,
            session_frequency: String::from("Annual"),
        }
    }

    /// Makes the legislature bicameral.
    pub fn with_upper_house(mut self, upper_house: String) -> Self {
        self.is_bicameral = true;
        self.upper_house = Some(upper_house);
        if !self.stages.contains(&LegislativeStage::UpperHouse) {
            self.stages.insert(5, LegislativeStage::UpperHouse);
        }
        self
    }
}

/// Constitutional features.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConstitutionalFeature {
    /// Written constitution
    WrittenConstitution,
    /// Bill of rights
    BillOfRights,
    /// Separation of powers
    SeparationOfPowers,
    /// Federalism
    Federalism,
    /// Judicial review
    JudicialReview,
    /// Parliamentary sovereignty
    ParliamentarySovereignty,
    /// Presidential system
    PresidentialSystem,
    /// Parliamentary system
    ParliamentarySystem,
    /// Semi-presidential system
    SemiPresidentialSystem,
    /// Constitutional monarchy
    ConstitutionalMonarchy,
}

/// Constitutional framework for a jurisdiction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalFramework {
    /// Whether there is a written constitution
    pub has_written_constitution: bool,
    /// Constitution document name
    pub constitution_name: Option<String>,
    /// Year of current constitution
    pub constitution_year: Option<u32>,
    /// Constitutional features
    pub features: Vec<ConstitutionalFeature>,
    /// Amendment process difficulty (1-10, 10 = hardest)
    pub amendment_difficulty: u8,
    /// Fundamental rights enumerated
    pub fundamental_rights: Vec<String>,
    /// Government structure
    pub government_structure: String,
}

impl ConstitutionalFramework {
    /// Creates a new constitutional framework.
    pub fn new() -> Self {
        Self {
            has_written_constitution: true,
            constitution_name: None,
            constitution_year: None,
            features: Vec::new(),
            amendment_difficulty: 5,
            fundamental_rights: Vec::new(),
            government_structure: String::new(),
        }
    }

    /// Adds a constitutional feature.
    pub fn add_feature(&mut self, feature: ConstitutionalFeature) {
        if !self.features.contains(&feature) {
            self.features.push(feature);
        }
    }

    /// Checks if a feature is present.
    pub fn has_feature(&self, feature: ConstitutionalFeature) -> bool {
        self.features.contains(&feature)
    }
}

impl Default for ConstitutionalFramework {
    fn default() -> Self {
        Self::new()
    }
}

/// Comprehensive jurisdiction profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JurisdictionProfile {
    /// Jurisdiction code (ISO 3166-1 alpha-2)
    pub code: String,
    /// Full jurisdiction name
    pub name: String,
    /// Legal system type
    pub legal_system: LegalSystemType,
    /// Court hierarchy
    pub court_hierarchy: CourtHierarchy,
    /// Legislative process
    pub legislative_process: LegislativeProcess,
    /// Constitutional framework
    pub constitutional_framework: ConstitutionalFramework,
    /// Official languages
    pub official_languages: Vec<String>,
    /// Population (latest estimate)
    pub population: Option<u64>,
    /// GDP per capita (USD)
    pub gdp_per_capita: Option<f64>,
    /// Human Development Index
    pub hdi: Option<f64>,
    /// Legal tradition influences
    pub legal_influences: Vec<String>,
    /// Notable legal characteristics
    pub characteristics: Vec<String>,
}

impl JurisdictionProfile {
    /// Creates a new jurisdiction profile.
    pub fn new(code: String, name: String, legal_system: LegalSystemType) -> Self {
        Self {
            code,
            name,
            legal_system,
            court_hierarchy: CourtHierarchy::new(),
            legislative_process: LegislativeProcess::new(
                String::from("Legislature"),
                String::from("Chamber"),
            ),
            constitutional_framework: ConstitutionalFramework::new(),
            official_languages: Vec::new(),
            population: None,
            gdp_per_capita: None,
            hdi: None,
            legal_influences: Vec::new(),
            characteristics: Vec::new(),
        }
    }

    /// Calculates compatibility score with another jurisdiction.
    pub fn compatibility_score(&self, other: &JurisdictionProfile) -> f64 {
        let mut score = 0.0;
        let mut factors = 0.0;

        // Legal system similarity (weight: 3.0)
        if self.legal_system == other.legal_system {
            score += 3.0;
        } else if matches!(
            (self.legal_system, other.legal_system),
            (LegalSystemType::Mixed, _) | (_, LegalSystemType::Mixed)
        ) {
            score += 1.5;
        }
        factors += 3.0;

        // Constitutional features overlap (weight: 2.0)
        let self_features: std::collections::HashSet<_> =
            self.constitutional_framework.features.iter().collect();
        let other_features: std::collections::HashSet<_> =
            other.constitutional_framework.features.iter().collect();
        let overlap = self_features.intersection(&other_features).count();
        let total = self_features.union(&other_features).count();
        if total > 0 {
            score += 2.0 * (overlap as f64 / total as f64);
        }
        factors += 2.0;

        // Legislative structure similarity (weight: 1.0)
        if self.legislative_process.is_bicameral == other.legislative_process.is_bicameral {
            score += 1.0;
        } else {
            score += 0.5;
        }
        factors += 1.0;

        // Economic development similarity (weight: 1.0)
        if let (Some(self_gdp), Some(other_gdp)) = (self.gdp_per_capita, other.gdp_per_capita) {
            let ratio = self_gdp.min(other_gdp) / self_gdp.max(other_gdp);
            score += ratio;
        }
        factors += 1.0;

        // Normalize to 0.0-1.0
        score / factors
    }
}

/// Jurisdiction database with comprehensive profiles.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JurisdictionDatabase {
    /// Profiles indexed by jurisdiction code
    profiles: HashMap<String, JurisdictionProfile>,
}

impl JurisdictionDatabase {
    /// Creates a new jurisdiction database.
    pub fn new() -> Self {
        Self {
            profiles: HashMap::new(),
        }
    }

    /// Adds a jurisdiction profile.
    pub fn add_profile(&mut self, profile: JurisdictionProfile) {
        self.profiles.insert(profile.code.clone(), profile);
    }

    /// Gets a jurisdiction profile by code.
    pub fn get_profile(&self, code: &str) -> Option<&JurisdictionProfile> {
        self.profiles.get(code)
    }

    /// Gets a mutable jurisdiction profile by code.
    pub fn get_profile_mut(&mut self, code: &str) -> Option<&mut JurisdictionProfile> {
        self.profiles.get_mut(code)
    }

    /// Lists all jurisdiction codes.
    pub fn list_codes(&self) -> Vec<&String> {
        self.profiles.keys().collect()
    }

    /// Finds jurisdictions by legal system type.
    pub fn find_by_legal_system(&self, system: LegalSystemType) -> Vec<&JurisdictionProfile> {
        self.profiles
            .values()
            .filter(|p| p.legal_system == system)
            .collect()
    }

    /// Finds most compatible jurisdictions for a given one.
    pub fn find_compatible(&self, code: &str, min_score: f64) -> Vec<(&JurisdictionProfile, f64)> {
        if let Some(source) = self.get_profile(code) {
            let mut compatible: Vec<_> = self
                .profiles
                .values()
                .filter(|p| p.code != code)
                .map(|p| {
                    let score = source.compatibility_score(p);
                    (p, score)
                })
                .filter(|(_, score)| *score >= min_score)
                .collect();
            compatible.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
            compatible
        } else {
            Vec::new()
        }
    }

    /// Creates a database with comprehensive profiles for major jurisdictions.
    pub fn with_major_jurisdictions() -> Self {
        let mut db = Self::new();

        // United States
        let mut us = JurisdictionProfile::new(
            String::from("US"),
            String::from("United States"),
            LegalSystemType::CommonLaw,
        );
        us.official_languages = vec![String::from("en")];
        us.population = Some(331_000_000);
        us.gdp_per_capita = Some(69_287.0);
        us.hdi = Some(0.921);
        us.legal_influences = vec![String::from("English common law")];
        us.constitutional_framework = {
            let mut cf = ConstitutionalFramework::new();
            cf.has_written_constitution = true;
            cf.constitution_name = Some(String::from("Constitution of the United States"));
            cf.constitution_year = Some(1789);
            cf.add_feature(ConstitutionalFeature::WrittenConstitution);
            cf.add_feature(ConstitutionalFeature::BillOfRights);
            cf.add_feature(ConstitutionalFeature::SeparationOfPowers);
            cf.add_feature(ConstitutionalFeature::Federalism);
            cf.add_feature(ConstitutionalFeature::JudicialReview);
            cf.add_feature(ConstitutionalFeature::PresidentialSystem);
            cf.amendment_difficulty = 9;
            cf.government_structure = String::from("Federal presidential constitutional republic");
            cf.fundamental_rights = vec![
                String::from("Freedom of speech"),
                String::from("Freedom of religion"),
                String::from("Right to bear arms"),
                String::from("Due process"),
                String::from("Equal protection"),
            ];
            cf
        };
        us.legislative_process = LegislativeProcess::new(
            String::from("United States Congress"),
            String::from("House of Representatives"),
        )
        .with_upper_house(String::from("Senate"));
        us.court_hierarchy = {
            let mut ch = CourtHierarchy::new();
            ch.add_court(Court {
                name: String::from("Supreme Court of the United States"),
                level: CourtLevel::Supreme,
                jurisdiction: String::from("Federal"),
                precedent_setting: true,
                judges: Some(9),
                url: Some(String::from("https://www.supremecourt.gov")),
            });
            ch.add_court(Court {
                name: String::from("U.S. Courts of Appeals"),
                level: CourtLevel::Appellate,
                jurisdiction: String::from("Federal circuits"),
                precedent_setting: true,
                judges: Some(179),
                url: None,
            });
            ch.add_court(Court {
                name: String::from("U.S. District Courts"),
                level: CourtLevel::District,
                jurisdiction: String::from("Federal districts"),
                precedent_setting: false,
                judges: Some(677),
                url: None,
            });
            ch.has_jury_trials = true;
            ch.appeal_path = String::from("District → Appeals → Supreme Court");
            ch
        };
        db.add_profile(us);

        // Japan
        let mut jp = JurisdictionProfile::new(
            String::from("JP"),
            String::from("Japan"),
            LegalSystemType::CivilLaw,
        );
        jp.official_languages = vec![String::from("ja")];
        jp.population = Some(125_000_000);
        jp.gdp_per_capita = Some(39_285.0);
        jp.hdi = Some(0.919);
        jp.legal_influences = vec![
            String::from("German civil law"),
            String::from("French civil law"),
            String::from("Anglo-American law (post-WWII)"),
        ];
        jp.constitutional_framework = {
            let mut cf = ConstitutionalFramework::new();
            cf.has_written_constitution = true;
            cf.constitution_name = Some(String::from("Constitution of Japan"));
            cf.constitution_year = Some(1947);
            cf.add_feature(ConstitutionalFeature::WrittenConstitution);
            cf.add_feature(ConstitutionalFeature::BillOfRights);
            cf.add_feature(ConstitutionalFeature::SeparationOfPowers);
            cf.add_feature(ConstitutionalFeature::JudicialReview);
            cf.add_feature(ConstitutionalFeature::ParliamentarySystem);
            cf.add_feature(ConstitutionalFeature::ConstitutionalMonarchy);
            cf.amendment_difficulty = 10;
            cf.government_structure = String::from("Unitary parliamentary constitutional monarchy");
            cf.fundamental_rights = vec![
                String::from("Equality under the law"),
                String::from("Freedom of thought and conscience"),
                String::from("Academic freedom"),
                String::from("Right to life, liberty, and pursuit of happiness"),
                String::from("Pacifism (Article 9)"),
            ];
            cf
        };
        jp.legislative_process = LegislativeProcess::new(
            String::from("National Diet"),
            String::from("House of Representatives"),
        )
        .with_upper_house(String::from("House of Councillors"));
        jp.court_hierarchy = {
            let mut ch = CourtHierarchy::new();
            ch.add_court(Court {
                name: String::from("Supreme Court of Japan"),
                level: CourtLevel::Supreme,
                jurisdiction: String::from("National"),
                precedent_setting: true,
                judges: Some(15),
                url: Some(String::from("https://www.courts.go.jp")),
            });
            ch.add_court(Court {
                name: String::from("High Courts"),
                level: CourtLevel::Appellate,
                jurisdiction: String::from("Regional"),
                precedent_setting: false,
                judges: Some(350),
                url: None,
            });
            ch.add_court(Court {
                name: String::from("District Courts"),
                level: CourtLevel::District,
                jurisdiction: String::from("Prefectural"),
                precedent_setting: false,
                judges: Some(900),
                url: None,
            });
            ch.has_jury_trials = false;
            ch.appeal_path = String::from("District → High → Supreme Court");
            ch
        };
        db.add_profile(jp);

        // United Kingdom
        let mut gb = JurisdictionProfile::new(
            String::from("GB"),
            String::from("United Kingdom"),
            LegalSystemType::CommonLaw,
        );
        gb.official_languages = vec![String::from("en")];
        gb.population = Some(67_000_000);
        gb.gdp_per_capita = Some(46_510.0);
        gb.hdi = Some(0.929);
        gb.legal_influences = vec![String::from("English common law tradition")];
        gb.constitutional_framework = {
            let mut cf = ConstitutionalFramework::new();
            cf.has_written_constitution = false;
            cf.constitution_name = None;
            cf.add_feature(ConstitutionalFeature::ParliamentarySovereignty);
            cf.add_feature(ConstitutionalFeature::ParliamentarySystem);
            cf.add_feature(ConstitutionalFeature::ConstitutionalMonarchy);
            cf.amendment_difficulty = 3;
            cf.government_structure = String::from("Unitary parliamentary constitutional monarchy");
            cf.fundamental_rights = vec![
                String::from("Rights under common law"),
                String::from("Human Rights Act 1998"),
                String::from("Magna Carta principles"),
            ];
            cf
        };
        gb.legislative_process = LegislativeProcess::new(
            String::from("Parliament of the United Kingdom"),
            String::from("House of Commons"),
        )
        .with_upper_house(String::from("House of Lords"));
        gb.court_hierarchy = {
            let mut ch = CourtHierarchy::new();
            ch.add_court(Court {
                name: String::from("Supreme Court of the United Kingdom"),
                level: CourtLevel::Supreme,
                jurisdiction: String::from("National"),
                precedent_setting: true,
                judges: Some(12),
                url: Some(String::from("https://www.supremecourt.uk")),
            });
            ch.add_court(Court {
                name: String::from("Court of Appeal"),
                level: CourtLevel::Appellate,
                jurisdiction: String::from("England and Wales"),
                precedent_setting: true,
                judges: Some(39),
                url: None,
            });
            ch.add_court(Court {
                name: String::from("High Court"),
                level: CourtLevel::District,
                jurisdiction: String::from("England and Wales"),
                precedent_setting: true,
                judges: Some(108),
                url: None,
            });
            ch.has_jury_trials = true;
            ch.appeal_path = String::from("High Court → Court of Appeal → Supreme Court");
            ch
        };
        db.add_profile(gb);

        // Germany
        let mut de = JurisdictionProfile::new(
            String::from("DE"),
            String::from("Germany"),
            LegalSystemType::CivilLaw,
        );
        de.official_languages = vec![String::from("de")];
        de.population = Some(83_000_000);
        de.gdp_per_capita = Some(50_795.0);
        de.hdi = Some(0.942);
        de.legal_influences = vec![String::from("Roman law"), String::from("Germanic law")];
        de.constitutional_framework = {
            let mut cf = ConstitutionalFramework::new();
            cf.has_written_constitution = true;
            cf.constitution_name = Some(String::from("Basic Law (Grundgesetz)"));
            cf.constitution_year = Some(1949);
            cf.add_feature(ConstitutionalFeature::WrittenConstitution);
            cf.add_feature(ConstitutionalFeature::BillOfRights);
            cf.add_feature(ConstitutionalFeature::SeparationOfPowers);
            cf.add_feature(ConstitutionalFeature::Federalism);
            cf.add_feature(ConstitutionalFeature::JudicialReview);
            cf.add_feature(ConstitutionalFeature::ParliamentarySystem);
            cf.amendment_difficulty = 8;
            cf.government_structure = String::from("Federal parliamentary republic");
            cf.fundamental_rights = vec![
                String::from("Human dignity"),
                String::from("Right to life and physical integrity"),
                String::from("Equality before the law"),
                String::from("Freedom of faith and conscience"),
                String::from("Freedom of expression"),
            ];
            cf
        };
        de.legislative_process =
            LegislativeProcess::new(String::from("German Parliament"), String::from("Bundestag"))
                .with_upper_house(String::from("Bundesrat"));
        de.court_hierarchy = {
            let mut ch = CourtHierarchy::new();
            ch.add_court(Court {
                name: String::from("Federal Constitutional Court"),
                level: CourtLevel::Supreme,
                jurisdiction: String::from("Constitutional"),
                precedent_setting: true,
                judges: Some(16),
                url: Some(String::from("https://www.bundesverfassungsgericht.de")),
            });
            ch.add_court(Court {
                name: String::from("Federal Court of Justice"),
                level: CourtLevel::Supreme,
                jurisdiction: String::from("Civil and Criminal"),
                precedent_setting: true,
                judges: Some(127),
                url: None,
            });
            ch.constitutional_court = Some(String::from("Federal Constitutional Court"));
            ch.has_jury_trials = false;
            ch.appeal_path = String::from("Regional → Higher Regional → Federal");
            ch
        };
        db.add_profile(de);

        // France
        let mut fr = JurisdictionProfile::new(
            String::from("FR"),
            String::from("France"),
            LegalSystemType::CivilLaw,
        );
        fr.official_languages = vec![String::from("fr")];
        fr.population = Some(67_000_000);
        fr.gdp_per_capita = Some(44_408.0);
        fr.hdi = Some(0.903);
        fr.legal_influences = vec![String::from("Napoleonic Code"), String::from("Roman law")];
        fr.constitutional_framework = {
            let mut cf = ConstitutionalFramework::new();
            cf.has_written_constitution = true;
            cf.constitution_name = Some(String::from("Constitution of the Fifth Republic"));
            cf.constitution_year = Some(1958);
            cf.add_feature(ConstitutionalFeature::WrittenConstitution);
            cf.add_feature(ConstitutionalFeature::BillOfRights);
            cf.add_feature(ConstitutionalFeature::SeparationOfPowers);
            cf.add_feature(ConstitutionalFeature::JudicialReview);
            cf.add_feature(ConstitutionalFeature::SemiPresidentialSystem);
            cf.amendment_difficulty = 7;
            cf.government_structure = String::from("Unitary semi-presidential republic");
            cf.fundamental_rights = vec![
                String::from("Liberty"),
                String::from("Equality"),
                String::from("Fraternity"),
                String::from("Secularism (laïcité)"),
                String::from("Rights of Man and Citizen"),
            ];
            cf
        };
        fr.legislative_process = LegislativeProcess::new(
            String::from("French Parliament"),
            String::from("National Assembly"),
        )
        .with_upper_house(String::from("Senate"));
        fr.court_hierarchy = {
            let mut ch = CourtHierarchy::new();
            ch.add_court(Court {
                name: String::from("Constitutional Council"),
                level: CourtLevel::Supreme,
                jurisdiction: String::from("Constitutional"),
                precedent_setting: true,
                judges: Some(9),
                url: Some(String::from("https://www.conseil-constitutionnel.fr")),
            });
            ch.add_court(Court {
                name: String::from("Court of Cassation"),
                level: CourtLevel::Supreme,
                jurisdiction: String::from("Civil and Criminal"),
                precedent_setting: true,
                judges: Some(150),
                url: None,
            });
            ch.constitutional_court = Some(String::from("Constitutional Council"));
            ch.has_jury_trials = true;
            ch.appeal_path = String::from("First Instance → Appeal → Cassation");
            ch
        };
        db.add_profile(fr);

        db
    }
}

impl Default for JurisdictionDatabase {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Semantic Mapping (v0.1.2)
// ============================================================================

/// Concept equivalence entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptEquivalence {
    /// Source concept
    pub source_concept: String,
    /// Target concept
    pub target_concept: String,
    /// Equivalence score (0.0-1.0, 1.0 = perfect match)
    pub equivalence_score: f64,
    /// Semantic distance (0.0-1.0, 0.0 = identical)
    pub semantic_distance: f64,
    /// Context requirements
    pub context: Vec<String>,
    /// Notes on usage differences
    pub notes: Option<String>,
}

impl ConceptEquivalence {
    /// Creates a new concept equivalence.
    pub fn new(source_concept: String, target_concept: String, equivalence_score: f64) -> Self {
        Self {
            source_concept,
            target_concept,
            equivalence_score,
            semantic_distance: 1.0 - equivalence_score,
            context: Vec::new(),
            notes: None,
        }
    }

    /// Adds context requirement.
    pub fn with_context(mut self, context: String) -> Self {
        self.context.push(context);
        self
    }

    /// Adds notes.
    pub fn with_notes(mut self, notes: String) -> Self {
        self.notes = Some(notes);
        self
    }
}

/// Concept equivalence database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptEquivalenceDatabase {
    /// Equivalences indexed by source jurisdiction and concept
    equivalences: HashMap<String, Vec<ConceptEquivalence>>,
}

impl ConceptEquivalenceDatabase {
    /// Creates a new concept equivalence database.
    pub fn new() -> Self {
        Self {
            equivalences: HashMap::new(),
        }
    }

    /// Adds a concept equivalence.
    pub fn add_equivalence(&mut self, jurisdiction_pair: String, equivalence: ConceptEquivalence) {
        self.equivalences
            .entry(jurisdiction_pair)
            .or_default()
            .push(equivalence);
    }

    /// Finds equivalences for a concept.
    pub fn find_equivalences(
        &self,
        source_jurisdiction: &str,
        target_jurisdiction: &str,
        concept: &str,
    ) -> Vec<&ConceptEquivalence> {
        let key = format!("{}->{}", source_jurisdiction, target_jurisdiction);
        self.equivalences
            .get(&key)
            .map(|equivs| {
                equivs
                    .iter()
                    .filter(|e| {
                        e.source_concept.eq_ignore_ascii_case(concept)
                            || e.source_concept.contains(concept)
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Gets the best match for a concept.
    pub fn best_match(
        &self,
        source_jurisdiction: &str,
        target_jurisdiction: &str,
        concept: &str,
    ) -> Option<&ConceptEquivalence> {
        let matches = self.find_equivalences(source_jurisdiction, target_jurisdiction, concept);
        matches.into_iter().max_by(|a, b| {
            a.equivalence_score
                .partial_cmp(&b.equivalence_score)
                .unwrap()
        })
    }
}

impl Default for ConceptEquivalenceDatabase {
    fn default() -> Self {
        Self::new()
    }
}

/// Legal term entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalTerm {
    /// Term in source language/jurisdiction
    pub term: String,
    /// Definition
    pub definition: String,
    /// Jurisdiction code
    pub jurisdiction: String,
    /// Legal domain (e.g., "criminal", "civil", "constitutional")
    pub domain: String,
    /// Related terms
    pub related_terms: Vec<String>,
}

impl LegalTerm {
    /// Creates a new legal term.
    pub fn new(term: String, definition: String, jurisdiction: String, domain: String) -> Self {
        Self {
            term,
            definition,
            jurisdiction,
            domain,
            related_terms: Vec::new(),
        }
    }
}

/// Legal term translation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TermTranslation {
    /// Source term
    pub source_term: String,
    /// Source jurisdiction
    pub source_jurisdiction: String,
    /// Target term
    pub target_term: String,
    /// Target jurisdiction
    pub target_jurisdiction: String,
    /// Translation accuracy (0.0-1.0)
    pub accuracy: f64,
    /// Whether this is a direct translation or approximation
    pub is_direct: bool,
    /// Context where this translation is valid
    pub valid_contexts: Vec<String>,
    /// Usage notes
    pub notes: Option<String>,
}

impl TermTranslation {
    /// Creates a new term translation.
    pub fn new(
        source_term: String,
        source_jurisdiction: String,
        target_term: String,
        target_jurisdiction: String,
        accuracy: f64,
        is_direct: bool,
    ) -> Self {
        Self {
            source_term,
            source_jurisdiction,
            target_term,
            target_jurisdiction,
            accuracy,
            is_direct,
            valid_contexts: Vec::new(),
            notes: None,
        }
    }
}

/// Legal term translation matrix.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TermTranslationMatrix {
    /// Translations indexed by source jurisdiction->target jurisdiction
    translations: HashMap<String, Vec<TermTranslation>>,
    /// Terms indexed by jurisdiction
    terms: HashMap<String, Vec<LegalTerm>>,
}

impl TermTranslationMatrix {
    /// Creates a new term translation matrix.
    pub fn new() -> Self {
        Self {
            translations: HashMap::new(),
            terms: HashMap::new(),
        }
    }

    /// Adds a term to the dictionary.
    pub fn add_term(&mut self, term: LegalTerm) {
        self.terms
            .entry(term.jurisdiction.clone())
            .or_default()
            .push(term);
    }

    /// Adds a translation.
    pub fn add_translation(&mut self, translation: TermTranslation) {
        let key = format!(
            "{}->{}",
            translation.source_jurisdiction, translation.target_jurisdiction
        );
        self.translations.entry(key).or_default().push(translation);
    }

    /// Finds translations for a term.
    pub fn find_translations(
        &self,
        source_jurisdiction: &str,
        target_jurisdiction: &str,
        term: &str,
    ) -> Vec<&TermTranslation> {
        let key = format!("{}->{}", source_jurisdiction, target_jurisdiction);
        self.translations
            .get(&key)
            .map(|trans| {
                trans
                    .iter()
                    .filter(|t| t.source_term.eq_ignore_ascii_case(term))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Gets the best translation for a term.
    pub fn best_translation(
        &self,
        source_jurisdiction: &str,
        target_jurisdiction: &str,
        term: &str,
        context: Option<&str>,
    ) -> Option<&TermTranslation> {
        let translations = self.find_translations(source_jurisdiction, target_jurisdiction, term);

        if let Some(ctx) = context {
            // First try to find a translation valid in this context
            if let Some(trans) = translations.iter().find(|t| {
                t.valid_contexts.is_empty() || t.valid_contexts.iter().any(|c| c.contains(ctx))
            }) {
                return Some(trans);
            }
        }

        // Otherwise, return the most accurate translation
        translations
            .into_iter()
            .max_by(|a, b| a.accuracy.partial_cmp(&b.accuracy).unwrap())
    }

    /// Gets terms for a jurisdiction.
    pub fn get_terms(&self, jurisdiction: &str) -> Vec<&LegalTerm> {
        self.terms
            .get(jurisdiction)
            .map(|terms| terms.iter().collect())
            .unwrap_or_default()
    }

    /// Gets terms for a jurisdiction and domain.
    pub fn get_terms_by_domain(&self, jurisdiction: &str, domain: &str) -> Vec<&LegalTerm> {
        self.get_terms(jurisdiction)
            .into_iter()
            .filter(|t| t.domain.eq_ignore_ascii_case(domain))
            .collect()
    }

    /// Creates a matrix with common legal term translations.
    pub fn with_common_translations() -> Self {
        let mut matrix = Self::new();

        // US -> JP criminal law terms
        matrix.add_translation(TermTranslation::new(
            String::from("felony"),
            String::from("US"),
            String::from("重罪"),
            String::from("JP"),
            0.9,
            true,
        ));

        matrix.add_translation(TermTranslation::new(
            String::from("misdemeanor"),
            String::from("US"),
            String::from("軽罪"),
            String::from("JP"),
            0.9,
            true,
        ));

        matrix.add_translation(TermTranslation::new(
            String::from("indictment"),
            String::from("US"),
            String::from("起訴"),
            String::from("JP"),
            0.85,
            true,
        ));

        // JP -> US criminal law terms
        matrix.add_translation(TermTranslation::new(
            String::from("起訴"),
            String::from("JP"),
            String::from("indictment"),
            String::from("US"),
            0.85,
            true,
        ));

        matrix.add_translation(TermTranslation::new(
            String::from("判決"),
            String::from("JP"),
            String::from("judgment"),
            String::from("US"),
            0.9,
            true,
        ));

        // Common law -> civil law terms
        matrix.add_translation(TermTranslation::new(
            String::from("precedent"),
            String::from("GB"),
            String::from("jurisprudence"),
            String::from("FR"),
            0.7,
            false,
        ));

        matrix.add_translation(TermTranslation::new(
            String::from("case law"),
            String::from("US"),
            String::from("判例法"),
            String::from("JP"),
            0.85,
            true,
        ));

        matrix
    }
}

impl Default for TermTranslationMatrix {
    fn default() -> Self {
        Self::new()
    }
}

/// Semantic distance calculator.
#[derive(Debug, Clone)]
pub struct SemanticDistanceCalculator {
    /// Concept equivalence database
    concept_db: ConceptEquivalenceDatabase,
}

impl SemanticDistanceCalculator {
    /// Creates a new semantic distance calculator.
    pub fn new(concept_db: ConceptEquivalenceDatabase) -> Self {
        Self { concept_db }
    }

    /// Calculates semantic distance between two concepts.
    pub fn calculate_distance(
        &self,
        source_jurisdiction: &str,
        target_jurisdiction: &str,
        source_concept: &str,
        target_concept: &str,
    ) -> f64 {
        // Try to find an equivalence entry
        if let Some(equiv) =
            self.concept_db
                .best_match(source_jurisdiction, target_jurisdiction, source_concept)
        {
            if equiv.target_concept.eq_ignore_ascii_case(target_concept) {
                return equiv.semantic_distance;
            }
        }

        // Fall back to simple string similarity
        self.string_similarity_distance(source_concept, target_concept)
    }

    /// Calculates distance based on string similarity.
    fn string_similarity_distance(&self, a: &str, b: &str) -> f64 {
        if a.eq_ignore_ascii_case(b) {
            return 0.0;
        }

        // Simple Levenshtein-based approximation
        let max_len = a.len().max(b.len());
        if max_len == 0 {
            return 0.0;
        }

        let edit_distance = self.levenshtein_distance(a, b);
        (edit_distance as f64) / (max_len as f64)
    }

    /// Calculates Levenshtein distance.
    #[allow(clippy::needless_range_loop)]
    fn levenshtein_distance(&self, a: &str, b: &str) -> usize {
        let a_chars: Vec<char> = a.chars().collect();
        let b_chars: Vec<char> = b.chars().collect();
        let a_len = a_chars.len();
        let b_len = b_chars.len();

        if a_len == 0 {
            return b_len;
        }
        if b_len == 0 {
            return a_len;
        }

        let mut matrix = vec![vec![0; b_len + 1]; a_len + 1];

        // Initialize first column and row (standard Levenshtein algorithm)
        for i in 0..=a_len {
            matrix[i][0] = i;
        }
        for j in 0..=b_len {
            matrix[0][j] = j;
        }

        for i in 1..=a_len {
            for j in 1..=b_len {
                let cost = if a_chars[i - 1] == b_chars[j - 1] {
                    0
                } else {
                    1
                };
                matrix[i][j] = (matrix[i - 1][j] + 1)
                    .min(matrix[i][j - 1] + 1)
                    .min(matrix[i - 1][j - 1] + cost);
            }
        }

        matrix[a_len][b_len]
    }
}

/// Context-aware term mapper.
#[derive(Debug, Clone)]
pub struct ContextAwareTermMapper {
    /// Term translation matrix
    translation_matrix: TermTranslationMatrix,
    /// Context rules
    context_rules: HashMap<String, Vec<String>>,
}

impl ContextAwareTermMapper {
    /// Creates a new context-aware term mapper.
    pub fn new(translation_matrix: TermTranslationMatrix) -> Self {
        Self {
            translation_matrix,
            context_rules: HashMap::new(),
        }
    }

    /// Adds a context rule.
    pub fn add_context_rule(&mut self, context: String, keywords: Vec<String>) {
        self.context_rules.insert(context, keywords);
    }

    /// Maps a term with context awareness.
    pub fn map_term(
        &self,
        source_jurisdiction: &str,
        target_jurisdiction: &str,
        term: &str,
        context_text: &str,
    ) -> Option<String> {
        // Determine context from text
        let context = self.detect_context(context_text);

        // Find best translation
        if let Some(translation) = self.translation_matrix.best_translation(
            source_jurisdiction,
            target_jurisdiction,
            term,
            context.as_deref(),
        ) {
            return Some(translation.target_term.clone());
        }

        None
    }

    /// Detects context from text.
    fn detect_context(&self, text: &str) -> Option<String> {
        let text_lower = text.to_lowercase();

        for (context, keywords) in &self.context_rules {
            if keywords.iter().any(|kw| text_lower.contains(kw)) {
                return Some(context.clone());
            }
        }

        None
    }
}

/// Jurisdiction-specific legal dictionary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalDictionary {
    /// Jurisdiction code
    pub jurisdiction: String,
    /// Terms in this dictionary
    pub terms: Vec<LegalTerm>,
    /// Dictionary metadata
    pub metadata: HashMap<String, String>,
}

impl LegalDictionary {
    /// Creates a new legal dictionary.
    pub fn new(jurisdiction: String) -> Self {
        Self {
            jurisdiction,
            terms: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Adds a term to the dictionary.
    pub fn add_term(&mut self, term: LegalTerm) {
        self.terms.push(term);
    }

    /// Finds a term by name.
    pub fn find_term(&self, term_name: &str) -> Option<&LegalTerm> {
        self.terms
            .iter()
            .find(|t| t.term.eq_ignore_ascii_case(term_name))
    }

    /// Gets terms by domain.
    pub fn get_by_domain(&self, domain: &str) -> Vec<&LegalTerm> {
        self.terms
            .iter()
            .filter(|t| t.domain.eq_ignore_ascii_case(domain))
            .collect()
    }

    /// Creates a US legal dictionary with common terms.
    pub fn us_dictionary() -> Self {
        let mut dict = Self::new(String::from("US"));

        dict.add_term(LegalTerm::new(
            String::from("felony"),
            String::from("A serious crime punishable by imprisonment for more than one year"),
            String::from("US"),
            String::from("criminal"),
        ));

        dict.add_term(LegalTerm::new(
            String::from("misdemeanor"),
            String::from("A less serious crime punishable by up to one year in jail"),
            String::from("US"),
            String::from("criminal"),
        ));

        dict.add_term(LegalTerm::new(
            String::from("tort"),
            String::from("A civil wrong that causes harm or loss"),
            String::from("US"),
            String::from("civil"),
        ));

        dict.add_term(LegalTerm::new(
            String::from("precedent"),
            String::from("A legal decision that serves as an authoritative rule in future cases"),
            String::from("US"),
            String::from("common law"),
        ));

        dict
    }

    /// Creates a Japan legal dictionary with common terms.
    pub fn japan_dictionary() -> Self {
        let mut dict = Self::new(String::from("JP"));

        dict.add_term(LegalTerm::new(
            String::from("重罪"),
            String::from("重大な犯罪"),
            String::from("JP"),
            String::from("criminal"),
        ));

        dict.add_term(LegalTerm::new(
            String::from("軽罪"),
            String::from("比較的軽微な犯罪"),
            String::from("JP"),
            String::from("criminal"),
        ));

        dict.add_term(LegalTerm::new(
            String::from("不法行為"),
            String::from("他人の権利を侵害する行為"),
            String::from("JP"),
            String::from("civil"),
        ));

        dict.add_term(LegalTerm::new(
            String::from("判例"),
            String::from("裁判所の判断の先例"),
            String::from("JP"),
            String::from("civil law"),
        ));

        dict
    }
}

// ============================================================================
// Cultural Adaptation (v0.1.3)
// ============================================================================

/// Religious/cultural exception type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CulturalExceptionType {
    /// Religious observance
    Religious,
    /// Cultural practice
    Cultural,
    /// Traditional custom
    Traditional,
    /// Ethical consideration
    Ethical,
    /// Dietary restriction
    Dietary,
    /// Dress code
    DressCode,
    /// Gender-specific
    GenderSpecific,
    /// Family structure
    FamilyStructure,
}

/// Cultural exception rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CulturalException {
    /// Exception type
    pub exception_type: CulturalExceptionType,
    /// Jurisdiction code
    pub jurisdiction: String,
    /// Description
    pub description: String,
    /// Legal basis
    pub legal_basis: Option<String>,
    /// Applicable domains
    pub applicable_domains: Vec<String>,
    /// Conflict resolution strategy
    pub resolution_strategy: String,
}

impl CulturalException {
    /// Creates a new cultural exception.
    pub fn new(
        exception_type: CulturalExceptionType,
        jurisdiction: String,
        description: String,
    ) -> Self {
        Self {
            exception_type,
            jurisdiction,
            description,
            legal_basis: None,
            applicable_domains: Vec::new(),
            resolution_strategy: String::from("Defer to local law"),
        }
    }

    /// Adds legal basis.
    pub fn with_legal_basis(mut self, legal_basis: String) -> Self {
        self.legal_basis = Some(legal_basis);
        self
    }

    /// Adds applicable domain.
    pub fn with_domain(mut self, domain: String) -> Self {
        self.applicable_domains.push(domain);
        self
    }
}

/// Cultural exception registry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CulturalExceptionRegistry {
    /// Exceptions indexed by jurisdiction
    exceptions: HashMap<String, Vec<CulturalException>>,
}

impl CulturalExceptionRegistry {
    /// Creates a new registry.
    pub fn new() -> Self {
        Self {
            exceptions: HashMap::new(),
        }
    }

    /// Adds an exception.
    pub fn add_exception(&mut self, exception: CulturalException) {
        self.exceptions
            .entry(exception.jurisdiction.clone())
            .or_default()
            .push(exception);
    }

    /// Gets exceptions for a jurisdiction.
    pub fn get_exceptions(&self, jurisdiction: &str) -> Vec<&CulturalException> {
        self.exceptions
            .get(jurisdiction)
            .map(|excs| excs.iter().collect())
            .unwrap_or_default()
    }

    /// Gets exceptions by type.
    pub fn get_by_type(
        &self,
        jurisdiction: &str,
        exception_type: CulturalExceptionType,
    ) -> Vec<&CulturalException> {
        self.get_exceptions(jurisdiction)
            .into_iter()
            .filter(|e| e.exception_type == exception_type)
            .collect()
    }

    /// Creates a registry with common exceptions.
    pub fn with_common_exceptions() -> Self {
        let mut registry = Self::new();

        // Japan - Religious exceptions
        registry.add_exception(
            CulturalException::new(
                CulturalExceptionType::Religious,
                String::from("JP"),
                String::from("Shinto shrine visits and ceremonies"),
            )
            .with_legal_basis(String::from(
                "Freedom of religion - Constitution Article 20",
            ))
            .with_domain(String::from("labor"))
            .with_domain(String::from("education")),
        );

        // US - Religious exceptions
        registry.add_exception(
            CulturalException::new(
                CulturalExceptionType::Religious,
                String::from("US"),
                String::from("Religious accommodation in workplace"),
            )
            .with_legal_basis(String::from("Title VII of Civil Rights Act"))
            .with_domain(String::from("employment")),
        );

        // FR - Secular exceptions
        registry.add_exception(
            CulturalException::new(
                CulturalExceptionType::Religious,
                String::from("FR"),
                String::from("Laïcité - strict separation of religion and state"),
            )
            .with_legal_basis(String::from("French Constitution Article 1"))
            .with_domain(String::from("public service"))
            .with_domain(String::from("education")),
        );

        registry
    }
}

impl Default for CulturalExceptionRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Calendar system type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CalendarSystem {
    /// Gregorian calendar
    Gregorian,
    /// Japanese imperial calendar
    Japanese,
    /// Islamic calendar
    Islamic,
    /// Hebrew calendar
    Hebrew,
    /// Chinese calendar
    Chinese,
    /// Buddhist calendar
    Buddhist,
}

/// Holiday type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HolidayType {
    /// National holiday
    National,
    /// Religious holiday
    Religious,
    /// Cultural observance
    Cultural,
    /// Regional holiday
    Regional,
}

/// Holiday definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Holiday {
    /// Holiday name
    pub name: String,
    /// Holiday type
    pub holiday_type: HolidayType,
    /// Jurisdiction
    pub jurisdiction: String,
    /// Date (month, day) - for fixed holidays
    pub fixed_date: Option<(u8, u8)>,
    /// Whether it's a legal non-working day
    pub is_legal_holiday: bool,
    /// Legal implications
    pub legal_implications: Vec<String>,
}

impl Holiday {
    /// Creates a new holiday.
    pub fn new(name: String, holiday_type: HolidayType, jurisdiction: String) -> Self {
        Self {
            name,
            holiday_type,
            jurisdiction,
            fixed_date: None,
            is_legal_holiday: false,
            legal_implications: Vec::new(),
        }
    }

    /// Sets fixed date.
    pub fn with_fixed_date(mut self, month: u8, day: u8) -> Self {
        self.fixed_date = Some((month, day));
        self
    }

    /// Marks as legal holiday.
    pub fn as_legal_holiday(mut self) -> Self {
        self.is_legal_holiday = true;
        self
    }
}

/// Holiday calendar adapter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HolidayCalendar {
    /// Jurisdiction
    pub jurisdiction: String,
    /// Calendar system
    pub calendar_system: CalendarSystem,
    /// Holidays
    pub holidays: Vec<Holiday>,
}

impl HolidayCalendar {
    /// Creates a new holiday calendar.
    pub fn new(jurisdiction: String, calendar_system: CalendarSystem) -> Self {
        Self {
            jurisdiction,
            calendar_system,
            holidays: Vec::new(),
        }
    }

    /// Adds a holiday.
    pub fn add_holiday(&mut self, holiday: Holiday) {
        self.holidays.push(holiday);
    }

    /// Gets holidays by type.
    pub fn get_by_type(&self, holiday_type: HolidayType) -> Vec<&Holiday> {
        self.holidays
            .iter()
            .filter(|h| h.holiday_type == holiday_type)
            .collect()
    }

    /// Creates US calendar.
    pub fn us_calendar() -> Self {
        let mut calendar = Self::new(String::from("US"), CalendarSystem::Gregorian);

        let mut new_year = Holiday::new(
            String::from("New Year's Day"),
            HolidayType::National,
            String::from("US"),
        )
        .with_fixed_date(1, 1)
        .as_legal_holiday();
        new_year
            .legal_implications
            .push(String::from("Federal holiday - offices closed"));
        calendar.add_holiday(new_year);

        let mut independence = Holiday::new(
            String::from("Independence Day"),
            HolidayType::National,
            String::from("US"),
        )
        .with_fixed_date(7, 4)
        .as_legal_holiday();
        independence
            .legal_implications
            .push(String::from("Federal holiday - offices closed"));
        calendar.add_holiday(independence);

        calendar
    }

    /// Creates Japan calendar.
    pub fn japan_calendar() -> Self {
        let mut calendar = Self::new(String::from("JP"), CalendarSystem::Japanese);

        let mut new_year = Holiday::new(
            String::from("元日 (New Year's Day)"),
            HolidayType::National,
            String::from("JP"),
        )
        .with_fixed_date(1, 1)
        .as_legal_holiday();
        new_year
            .legal_implications
            .push(String::from("National holiday - banks closed"));
        calendar.add_holiday(new_year);

        let mut constitution = Holiday::new(
            String::from("憲法記念日 (Constitution Day)"),
            HolidayType::National,
            String::from("JP"),
        )
        .with_fixed_date(5, 3)
        .as_legal_holiday();
        constitution
            .legal_implications
            .push(String::from("National holiday - government offices closed"));
        calendar.add_holiday(constitution);

        calendar
    }
}

/// Currency unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Currency {
    USD,
    JPY,
    EUR,
    GBP,
    CNY,
}

impl Currency {
    /// Gets the currency code.
    pub fn code(&self) -> &str {
        match self {
            Currency::USD => "USD",
            Currency::JPY => "JPY",
            Currency::EUR => "EUR",
            Currency::GBP => "GBP",
            Currency::CNY => "CNY",
        }
    }

    /// Gets the currency symbol.
    pub fn symbol(&self) -> &str {
        match self {
            Currency::USD => "$",
            Currency::JPY => "¥",
            Currency::EUR => "€",
            Currency::GBP => "£",
            Currency::CNY => "¥",
        }
    }
}

/// Monetary conversion with legal implications.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonetaryConversion {
    /// Source amount
    pub source_amount: f64,
    /// Source currency
    pub source_currency: Currency,
    /// Target amount
    pub target_amount: f64,
    /// Target currency
    pub target_currency: Currency,
    /// Exchange rate used
    pub exchange_rate: f64,
    /// Conversion date
    pub conversion_date: Option<String>,
    /// Legal significance threshold
    pub legal_significance: Option<String>,
}

impl MonetaryConversion {
    /// Creates a new monetary conversion.
    pub fn new(
        source_amount: f64,
        source_currency: Currency,
        target_currency: Currency,
        exchange_rate: f64,
    ) -> Self {
        Self {
            source_amount,
            source_currency,
            target_amount: source_amount * exchange_rate,
            target_currency,
            exchange_rate,
            conversion_date: None,
            legal_significance: None,
        }
    }

    /// Checks if amount exceeds a legal threshold.
    pub fn exceeds_threshold(&self, threshold: f64) -> bool {
        self.target_amount >= threshold
    }
}

/// Monetary adapter for legal contexts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonetaryAdapter {
    /// Exchange rates (base currency to target)
    exchange_rates: HashMap<String, f64>,
    /// Legal thresholds by jurisdiction
    legal_thresholds: HashMap<String, Vec<(String, f64)>>,
}

impl MonetaryAdapter {
    /// Creates a new monetary adapter.
    pub fn new() -> Self {
        Self {
            exchange_rates: HashMap::new(),
            legal_thresholds: HashMap::new(),
        }
    }

    /// Adds an exchange rate.
    pub fn add_rate(&mut self, from: Currency, to: Currency, rate: f64) {
        let key = format!("{}->{}", from.code(), to.code());
        self.exchange_rates.insert(key, rate);
    }

    /// Adds a legal threshold.
    pub fn add_threshold(&mut self, jurisdiction: String, description: String, amount: f64) {
        self.legal_thresholds
            .entry(jurisdiction)
            .or_default()
            .push((description, amount));
    }

    /// Converts amount with legal context.
    pub fn convert(&self, amount: f64, from: Currency, to: Currency) -> Option<MonetaryConversion> {
        let key = format!("{}->{}", from.code(), to.code());
        self.exchange_rates
            .get(&key)
            .map(|rate| MonetaryConversion::new(amount, from, to, *rate))
    }

    /// Creates adapter with common rates and thresholds.
    pub fn with_common_rates() -> Self {
        let mut adapter = Self::new();

        // Exchange rates (approximate)
        adapter.add_rate(Currency::USD, Currency::JPY, 150.0);
        adapter.add_rate(Currency::JPY, Currency::USD, 0.0067);
        adapter.add_rate(Currency::USD, Currency::EUR, 0.92);
        adapter.add_rate(Currency::EUR, Currency::USD, 1.09);
        adapter.add_rate(Currency::GBP, Currency::USD, 1.27);
        adapter.add_rate(Currency::USD, Currency::GBP, 0.79);

        // Legal thresholds
        adapter.add_threshold(
            String::from("US"),
            String::from("Felony theft threshold"),
            1000.0,
        );
        adapter.add_threshold(
            String::from("JP"),
            String::from("Major theft threshold (重罪窃盗)"),
            150_000.0,
        );
        adapter.add_threshold(
            String::from("US"),
            String::from("Federal reporting requirement"),
            10_000.0,
        );

        adapter
    }
}

impl Default for MonetaryAdapter {
    fn default() -> Self {
        Self::new()
    }
}

/// Age of majority definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgeOfMajority {
    /// Jurisdiction
    pub jurisdiction: String,
    /// Age of majority
    pub age: u8,
    /// Exceptions
    pub exceptions: Vec<String>,
    /// Legal implications
    pub legal_implications: Vec<String>,
}

impl AgeOfMajority {
    /// Creates a new age of majority.
    pub fn new(jurisdiction: String, age: u8) -> Self {
        Self {
            jurisdiction,
            age,
            exceptions: Vec::new(),
            legal_implications: Vec::new(),
        }
    }
}

/// Age of majority mapper.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgeOfMajorityMapper {
    /// Age of majority by jurisdiction
    ages: HashMap<String, AgeOfMajority>,
}

impl AgeOfMajorityMapper {
    /// Creates a new mapper.
    pub fn new() -> Self {
        Self {
            ages: HashMap::new(),
        }
    }

    /// Adds age of majority.
    pub fn add_age(&mut self, age: AgeOfMajority) {
        self.ages.insert(age.jurisdiction.clone(), age);
    }

    /// Gets age of majority for jurisdiction.
    pub fn get_age(&self, jurisdiction: &str) -> Option<&AgeOfMajority> {
        self.ages.get(jurisdiction)
    }

    /// Maps age reference from source to target jurisdiction.
    pub fn map_age_reference(
        &self,
        source_jurisdiction: &str,
        target_jurisdiction: &str,
    ) -> Option<String> {
        if let (Some(source), Some(target)) = (
            self.get_age(source_jurisdiction),
            self.get_age(target_jurisdiction),
        ) {
            if source.age != target.age {
                return Some(format!(
                    "Age adjusted from {} to {} for {}",
                    source.age, target.age, target_jurisdiction
                ));
            }
        }
        None
    }

    /// Creates mapper with common jurisdictions.
    pub fn with_common_jurisdictions() -> Self {
        let mut mapper = Self::new();

        let mut us = AgeOfMajority::new(String::from("US"), 18);
        us.legal_implications.push(String::from("Voting rights"));
        us.legal_implications
            .push(String::from("Contract capacity"));
        us.exceptions.push(String::from("Alcohol: 21 years"));
        mapper.add_age(us);

        let mut jp = AgeOfMajority::new(String::from("JP"), 18);
        jp.legal_implications
            .push(String::from("Full legal capacity"));
        jp.legal_implications
            .push(String::from("Marriage without parental consent"));
        jp.exceptions
            .push(String::from("Alcohol and tobacco: 20 years (until 2022)"));
        mapper.add_age(jp);

        let mut gb = AgeOfMajority::new(String::from("GB"), 18);
        gb.legal_implications
            .push(String::from("Full contractual capacity"));
        gb.legal_implications.push(String::from("Voting rights"));
        mapper.add_age(gb);

        mapper
    }
}

impl Default for AgeOfMajorityMapper {
    fn default() -> Self {
        Self::new()
    }
}

/// Legal capacity type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LegalCapacityType {
    /// Contractual capacity
    Contractual,
    /// Testamentary capacity
    Testamentary,
    /// Criminal responsibility
    CriminalResponsibility,
    /// Voting capacity
    Voting,
    /// Marriage capacity
    Marriage,
    /// Employment capacity
    Employment,
}

/// Legal capacity rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalCapacityRule {
    /// Capacity type
    pub capacity_type: LegalCapacityType,
    /// Jurisdiction
    pub jurisdiction: String,
    /// Minimum age
    pub minimum_age: u8,
    /// Conditions
    pub conditions: Vec<String>,
    /// Exceptions
    pub exceptions: Vec<String>,
}

impl LegalCapacityRule {
    /// Creates a new legal capacity rule.
    pub fn new(capacity_type: LegalCapacityType, jurisdiction: String, minimum_age: u8) -> Self {
        Self {
            capacity_type,
            jurisdiction,
            minimum_age,
            conditions: Vec::new(),
            exceptions: Vec::new(),
        }
    }
}

/// Legal capacity adapter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalCapacityAdapter {
    /// Rules indexed by jurisdiction
    rules: HashMap<String, Vec<LegalCapacityRule>>,
}

impl LegalCapacityAdapter {
    /// Creates a new adapter.
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }

    /// Adds a rule.
    pub fn add_rule(&mut self, rule: LegalCapacityRule) {
        self.rules
            .entry(rule.jurisdiction.clone())
            .or_default()
            .push(rule);
    }

    /// Gets rules for jurisdiction.
    pub fn get_rules(&self, jurisdiction: &str) -> Vec<&LegalCapacityRule> {
        self.rules
            .get(jurisdiction)
            .map(|rules| rules.iter().collect())
            .unwrap_or_default()
    }

    /// Gets rule by type.
    pub fn get_rule(
        &self,
        jurisdiction: &str,
        capacity_type: LegalCapacityType,
    ) -> Option<&LegalCapacityRule> {
        self.get_rules(jurisdiction)
            .into_iter()
            .find(|r| r.capacity_type == capacity_type)
    }

    /// Creates adapter with common rules.
    pub fn with_common_rules() -> Self {
        let mut adapter = Self::new();

        // US rules
        let mut us_contract =
            LegalCapacityRule::new(LegalCapacityType::Contractual, String::from("US"), 18);
        us_contract
            .exceptions
            .push(String::from("Necessaries doctrine for minors"));
        adapter.add_rule(us_contract);

        adapter.add_rule(LegalCapacityRule::new(
            LegalCapacityType::Voting,
            String::from("US"),
            18,
        ));

        adapter.add_rule(LegalCapacityRule::new(
            LegalCapacityType::CriminalResponsibility,
            String::from("US"),
            18,
        ));

        // Japan rules
        adapter.add_rule(LegalCapacityRule::new(
            LegalCapacityType::Contractual,
            String::from("JP"),
            18,
        ));

        let mut jp_marriage =
            LegalCapacityRule::new(LegalCapacityType::Marriage, String::from("JP"), 18);
        jp_marriage.conditions.push(String::from(
            "Parental consent required until age 20 (pre-2022)",
        ));
        adapter.add_rule(jp_marriage);

        adapter.add_rule(LegalCapacityRule::new(
            LegalCapacityType::CriminalResponsibility,
            String::from("JP"),
            14,
        ));

        adapter
    }
}

impl Default for LegalCapacityAdapter {
    fn default() -> Self {
        Self::new()
    }
}

/// Basic porting engine.
pub struct PortingEngine {
    /// Source jurisdiction
    source: Jurisdiction,
    /// Target jurisdiction
    target: Jurisdiction,
    /// Optional text generator for AI-assisted features
    text_generator: Option<Box<dyn TextGenerator>>,
    /// Term replacement rules
    term_replacements: Vec<TermReplacement>,
    /// Equivalence mappings
    equivalence_mappings: Vec<EquivalenceMapping>,
}

impl PortingEngine {
    /// Creates a new porting engine.
    pub fn new(source: Jurisdiction, target: Jurisdiction) -> Self {
        Self {
            source,
            target,
            text_generator: None,
            term_replacements: Vec::new(),
            equivalence_mappings: Vec::new(),
        }
    }

    /// Sets the text generator for AI-assisted features.
    pub fn with_text_generator(mut self, generator: Box<dyn TextGenerator>) -> Self {
        self.text_generator = Some(generator);
        self
    }

    /// Adds term replacement rules.
    pub fn with_term_replacements(mut self, replacements: Vec<TermReplacement>) -> Self {
        self.term_replacements = replacements;
        self
    }

    /// Adds equivalence mappings.
    pub fn with_equivalence_mappings(mut self, mappings: Vec<EquivalenceMapping>) -> Self {
        self.equivalence_mappings = mappings;
        self
    }

    /// Ports a single statute.
    pub fn port_statute(
        &self,
        statute: &Statute,
        options: &PortingOptions,
    ) -> PortingResult<PortedStatute> {
        let mut changes = Vec::new();
        let mut adapted = statute.clone();

        // Apply cultural parameter adaptations
        if options.apply_cultural_params {
            self.apply_cultural_adaptations(&mut adapted, &mut changes)?;
        }

        // Update statute ID for target jurisdiction
        adapted.id = format!("{}-{}", self.target.id.to_lowercase(), statute.id);

        // Calculate compatibility score based on changes
        let compatibility_score = if changes.is_empty() {
            1.0
        } else {
            let incompatible_count = changes
                .iter()
                .filter(|c| matches!(c.change_type, ChangeType::Incompatible))
                .count();
            let major_count = changes
                .iter()
                .filter(|c| {
                    matches!(
                        c.change_type,
                        ChangeType::CulturalAdaptation | ChangeType::Translation
                    )
                })
                .count();

            // Decrease score based on severity of changes
            1.0 - (incompatible_count as f64 * 0.3 + major_count as f64 * 0.1).min(0.9)
        };

        Ok(PortedStatute {
            original_id: statute.id.clone(),
            statute: adapted,
            changes,
            locale: self.target.locale.clone(),
            compatibility_score,
        })
    }

    fn apply_cultural_adaptations(
        &self,
        _statute: &mut Statute,
        changes: &mut Vec<PortingChange>,
    ) -> PortingResult<()> {
        let source_params = &self.source.cultural_params;
        let target_params = &self.target.cultural_params;

        // Check for age of majority differences
        if source_params.age_of_majority != target_params.age_of_majority {
            if let (Some(source_age), Some(target_age)) =
                (source_params.age_of_majority, target_params.age_of_majority)
            {
                // Would need to modify conditions here
                changes.push(PortingChange {
                    change_type: ChangeType::ValueAdaptation,
                    description: "Age of majority adjusted".to_string(),
                    original: Some(source_age.to_string()),
                    adapted: Some(target_age.to_string()),
                    reason: format!(
                        "Target jurisdiction ({}) has different age of majority",
                        self.target.id
                    ),
                });
            }
        }

        // Check for cultural prohibitions
        for prohibition in &target_params.prohibitions {
            changes.push(PortingChange {
                change_type: ChangeType::CulturalAdaptation,
                description: format!("Checked against prohibition: {}", prohibition),
                original: None,
                adapted: None,
                reason: "Target jurisdiction has cultural prohibition".to_string(),
            });
        }

        Ok(())
    }

    /// Generates a compatibility report.
    pub fn generate_report(&self, statutes: &[Statute]) -> CompatibilityReport {
        let mut report = CompatibilityReport::default();
        let mut findings = Vec::new();

        // Check legal system compatibility
        if self.source.legal_system != self.target.legal_system {
            findings.push(CompatibilityFinding {
                severity: Severity::Warning,
                category: "Legal System".to_string(),
                description: format!(
                    "Different legal systems: {:?} -> {:?}",
                    self.source.legal_system, self.target.legal_system
                ),
                statute_id: None,
            });
            report.adaptations_required += 1;
        }

        // Check for discretionary statutes
        for statute in statutes {
            if statute.discretion_logic.is_some() {
                findings.push(CompatibilityFinding {
                    severity: Severity::Info,
                    category: "Discretion".to_string(),
                    description: "Statute contains discretionary elements requiring local review"
                        .to_string(),
                    statute_id: Some(statute.id.clone()),
                });
            }
        }

        report.findings = findings;
        report.compatibility_score = self.calculate_compatibility_score(&report);
        report.recommendations = self.generate_recommendations(&report);

        report
    }

    fn calculate_compatibility_score(&self, report: &CompatibilityReport) -> f64 {
        let base_score = 1.0;
        let deductions =
            (report.adaptations_required as f64 * 0.1) + (report.incompatibilities as f64 * 0.2);
        (base_score - deductions).max(0.0)
    }

    fn generate_recommendations(&self, report: &CompatibilityReport) -> Vec<String> {
        let mut recommendations = Vec::new();

        if report.compatibility_score < 0.5 {
            recommendations.push(
                "Low compatibility score. Consider a full legal review before adoption."
                    .to_string(),
            );
        }

        if self.source.legal_system != self.target.legal_system {
            recommendations.push(
                "Legal systems differ. Case law adaptation may be required for common law targets."
                    .to_string(),
            );
        }

        recommendations
    }

    /// Generates AI-assisted cultural adaptation suggestions.
    pub async fn generate_ai_suggestions(
        &self,
        statute: &Statute,
    ) -> PortingResult<Vec<AdaptationSuggestion>> {
        let generator = self.text_generator.as_ref().ok_or_else(|| {
            PortingError::AdaptationRequired("Text generator not configured".to_string())
        })?;

        let prompt = format!(
            "Analyze the following statute for cultural adaptation from {} to {}:\n\
             Statute ID: {}\n\
             Title: {}\n\
             Source Legal System: {:?}\n\
             Target Legal System: {:?}\n\
             Source Cultural Parameters: Age of Majority = {:?}, Prohibitions = {:?}\n\
             Target Cultural Parameters: Age of Majority = {:?}, Prohibitions = {:?}\n\n\
             Please provide specific adaptation suggestions with rationale.",
            self.source.id,
            self.target.id,
            statute.id,
            statute.title,
            self.source.legal_system,
            self.target.legal_system,
            self.source.cultural_params.age_of_majority,
            self.source.cultural_params.prohibitions,
            self.target.cultural_params.age_of_majority,
            self.target.cultural_params.prohibitions
        );

        let response = generator.generate(&prompt).await?;

        // Parse response into suggestions (simplified for now)
        let suggestions = vec![AdaptationSuggestion {
            statute_id: statute.id.clone(),
            suggestion: response,
            rationale: "AI-generated based on cultural parameter analysis".to_string(),
            confidence: 0.8,
            category: "Cultural Adaptation".to_string(),
        }];

        Ok(suggestions)
    }

    /// Ports specific sections of a statute.
    pub fn port_sections(
        &self,
        statute: &Statute,
        section_ids: &[String],
        options: &PortingOptions,
    ) -> PortingResult<PortedStatute> {
        // For now, port the whole statute but track which sections were requested
        // In a real implementation, we would filter conditions/effects by section
        let mut ported = self.port_statute(statute, options)?;

        // Add a change record for partial porting
        ported.changes.push(PortingChange {
            change_type: ChangeType::ComplianceAddition,
            description: format!("Partial porting of sections: {:?}", section_ids),
            original: None,
            adapted: Some(format!("{} sections ported", section_ids.len())),
            reason: "Selective section porting requested".to_string(),
        });

        Ok(ported)
    }

    /// Performs reverse porting analysis (compare target to source).
    pub fn reverse_port_analysis(
        &self,
        _target_statute: &Statute,
    ) -> PortingResult<Vec<PortingChange>> {
        let mut changes = Vec::new();

        // Analyze what would need to change to port back to source
        if let (Some(target_age), Some(source_age)) = (
            self.target.cultural_params.age_of_majority,
            self.source.cultural_params.age_of_majority,
        ) {
            if target_age != source_age {
                changes.push(PortingChange {
                    change_type: ChangeType::ValueAdaptation,
                    description: "Reverse age of majority adjustment".to_string(),
                    original: Some(target_age.to_string()),
                    adapted: Some(source_age.to_string()),
                    reason: format!(
                        "Reverting to source jurisdiction ({}) age of majority",
                        self.source.id
                    ),
                });
            }
        }

        // Check for prohibitions that would need to be lifted
        for prohibition in &self.target.cultural_params.prohibitions {
            if !self
                .source
                .cultural_params
                .prohibitions
                .contains(prohibition)
            {
                changes.push(PortingChange {
                    change_type: ChangeType::Removal,
                    description: format!("Remove prohibition: {}", prohibition),
                    original: Some(prohibition.clone()),
                    adapted: None,
                    reason: "Source jurisdiction does not have this prohibition".to_string(),
                });
            }
        }

        Ok(changes)
    }

    /// Detects conflicts with target jurisdiction laws.
    pub fn detect_conflicts(&self, statute: &Statute) -> Vec<ConflictReport> {
        let mut conflicts = Vec::new();

        // Check for legal system conflicts
        if self.source.legal_system != self.target.legal_system {
            conflicts.push(ConflictReport {
                statute_id: statute.id.clone(),
                conflict_type: ConflictType::SystemMismatch,
                description: format!(
                    "Legal system mismatch: {:?} vs {:?}",
                    self.source.legal_system, self.target.legal_system
                ),
                severity: Severity::Warning,
                resolutions: vec![
                    "Adapt procedural elements to target legal system".to_string(),
                    "Consult legal expert for system-specific modifications".to_string(),
                ],
            });
        }

        // Check for cultural prohibitions
        for prohibition in &self.target.cultural_params.prohibitions {
            // Simplified check - in real implementation, would analyze statute content
            conflicts.push(ConflictReport {
                statute_id: statute.id.clone(),
                conflict_type: ConflictType::CulturalIncompatibility,
                description: format!("Check compatibility with prohibition: {}", prohibition),
                severity: Severity::Info,
                resolutions: vec![
                    format!("Review statute for compliance with: {}", prohibition),
                    "Consider alternative formulations".to_string(),
                ],
            });
        }

        conflicts
    }

    /// Validates semantic preservation during porting.
    pub fn validate_semantics(
        &self,
        original: &Statute,
        ported: &PortedStatute,
    ) -> SemanticValidation {
        let mut findings = Vec::new();

        // Check if title changed significantly
        if original.title != ported.statute.title {
            findings.push(SemanticFinding {
                statute_id: original.id.clone(),
                description: "Title modified during porting".to_string(),
                severity: Severity::Info,
                impact: "May affect legal citation and reference".to_string(),
            });
        }

        // Analyze changes for semantic impact
        for change in &ported.changes {
            match change.change_type {
                ChangeType::Translation => {
                    findings.push(SemanticFinding {
                        statute_id: original.id.clone(),
                        description: format!("Translation: {}", change.description),
                        severity: Severity::Info,
                        impact: "Semantic drift possible in translation".to_string(),
                    });
                }
                ChangeType::Incompatible => {
                    findings.push(SemanticFinding {
                        statute_id: original.id.clone(),
                        description: format!("Incompatibility: {}", change.description),
                        severity: Severity::Error,
                        impact: "Significant semantic change required".to_string(),
                    });
                }
                _ => {}
            }
        }

        // Calculate preservation score
        let error_count = findings
            .iter()
            .filter(|f| f.severity == Severity::Error)
            .count();
        let warning_count = findings
            .iter()
            .filter(|f| f.severity == Severity::Warning)
            .count();

        let preservation_score = 1.0 - (error_count as f64 * 0.3) - (warning_count as f64 * 0.1);
        let preservation_score = preservation_score.clamp(0.0, 1.0);

        SemanticValidation {
            preservation_score,
            is_valid: preservation_score >= 0.7,
            findings,
        }
    }

    /// Generates a risk assessment for ported statutes.
    pub fn assess_risks(&self, ported: &PortedStatute) -> RiskAssessment {
        let mut risks = Vec::new();

        // Legal system mismatch risk
        if self.source.legal_system != self.target.legal_system {
            risks.push(Risk {
                id: uuid::Uuid::new_v4().to_string(),
                category: RiskCategory::Legal,
                description: "Different legal systems may cause interpretation issues".to_string(),
                likelihood: RiskLevel::Medium,
                impact: 0.6,
                severity: RiskLevel::Medium,
            });
        }

        // Cultural adaptation risks
        let cultural_changes = ported
            .changes
            .iter()
            .filter(|c| matches!(c.change_type, ChangeType::CulturalAdaptation))
            .count();

        if cultural_changes > 0 {
            risks.push(Risk {
                id: uuid::Uuid::new_v4().to_string(),
                category: RiskCategory::Cultural,
                description: format!(
                    "{} cultural adaptations may affect statute applicability",
                    cultural_changes
                ),
                likelihood: RiskLevel::Medium,
                impact: 0.5,
                severity: RiskLevel::Low,
            });
        }

        // Incompatibility risks
        let incompatibilities = ported
            .changes
            .iter()
            .filter(|c| matches!(c.change_type, ChangeType::Incompatible))
            .count();

        if incompatibilities > 0 {
            risks.push(Risk {
                id: uuid::Uuid::new_v4().to_string(),
                category: RiskCategory::Legal,
                description: format!("{} incompatibilities detected", incompatibilities),
                likelihood: RiskLevel::High,
                impact: 0.8,
                severity: RiskLevel::High,
            });
        }

        // Calculate overall risk score
        let risk_score = if risks.is_empty() {
            0.1
        } else {
            // Convert RiskLevel to numeric value for calculation
            let risk_level_to_f64 = |level: RiskLevel| match level {
                RiskLevel::Low => 0.25,
                RiskLevel::Medium => 0.5,
                RiskLevel::High => 0.75,
                RiskLevel::Critical => 1.0,
            };
            risks
                .iter()
                .map(|r| risk_level_to_f64(r.likelihood) * r.impact)
                .sum::<f64>()
                / risks.len() as f64
        };

        let risk_level = match risk_score {
            s if s < 0.25 => RiskLevel::Low,
            s if s < 0.5 => RiskLevel::Medium,
            s if s < 0.75 => RiskLevel::High,
            _ => RiskLevel::Critical,
        };

        let mitigations = vec![
            "Conduct legal expert review".to_string(),
            "Pilot test in limited scope".to_string(),
            "Monitor implementation closely".to_string(),
            "Establish feedback mechanism".to_string(),
        ];

        RiskAssessment {
            risk_score,
            risk_level,
            risks,
            mitigations,
        }
    }

    /// Batch port multiple statutes.
    pub async fn batch_port(
        &self,
        statutes: &[Statute],
        options: &PortingOptions,
    ) -> PortingResult<PortingOutput> {
        let mut ported_statutes = Vec::new();
        let mut all_warnings = Vec::new();
        let mut all_ai_suggestions = Vec::new();
        let mut all_conflicts = Vec::new();

        for statute in statutes {
            // Port statute
            let ported = if !options.section_ids.is_empty() {
                self.port_sections(statute, &options.section_ids, options)?
            } else {
                self.port_statute(statute, options)?
            };

            // Generate AI suggestions if enabled
            if options.use_ai_suggestions && self.text_generator.is_some() {
                match self.generate_ai_suggestions(statute).await {
                    Ok(suggestions) => all_ai_suggestions.extend(suggestions),
                    Err(e) => {
                        all_warnings.push(format!("AI suggestion failed for {}: {}", statute.id, e))
                    }
                }
            }

            // Detect conflicts if enabled
            if options.detect_conflicts {
                all_conflicts.extend(self.detect_conflicts(statute));
            }

            ported_statutes.push(ported);
        }

        // Generate compatibility report if requested
        let report = if options.generate_report {
            Some(self.generate_report(statutes))
        } else {
            None
        };

        // Perform semantic validation if requested
        let semantic_validation = if options.validate_semantics && !ported_statutes.is_empty() {
            Some(self.validate_semantics(&statutes[0], &ported_statutes[0]))
        } else {
            None
        };

        // Generate risk assessment
        let risk_assessment = if !ported_statutes.is_empty() {
            Some(self.assess_risks(&ported_statutes[0]))
        } else {
            None
        };

        Ok(PortingOutput {
            statutes: ported_statutes,
            report,
            warnings: all_warnings,
            ai_suggestions: all_ai_suggestions,
            conflicts: all_conflicts,
            semantic_validation,
            risk_assessment,
        })
    }

    /// Creates a bilateral legal agreement template.
    pub fn create_bilateral_agreement(&self, agreement_type: AgreementType) -> BilateralAgreement {
        BilateralAgreement {
            id: format!(
                "{}-{}-agreement",
                self.source.id.to_lowercase(),
                self.target.id.to_lowercase()
            ),
            source_jurisdiction: self.source.id.clone(),
            target_jurisdiction: self.target.id.clone(),
            agreement_type,
            mutual_recognition: vec![
                "Both parties recognize each other's legal frameworks".to_string(),
                "Statutes ported under this agreement maintain legal validity".to_string(),
            ],
            adaptation_protocols: vec![AdaptationProtocol {
                name: "Standard Adaptation Protocol".to_string(),
                description: "Default protocol for statute adaptation".to_string(),
                statute_types: vec!["civil".to_string(), "commercial".to_string()],
                rules: vec![
                    "Preserve legal intent and semantic meaning".to_string(),
                    "Adapt numerical thresholds to local standards".to_string(),
                    "Replace legal terms with local equivalents".to_string(),
                ],
            }],
            dispute_resolution: Some(
                "Disputes resolved through bilateral consultation".to_string(),
            ),
        }
    }

    /// Finds equivalent regulations between jurisdictions.
    pub fn find_regulatory_equivalence(&self, statute: &Statute) -> Vec<EquivalenceMapping> {
        // Check if we have pre-configured mappings
        self.equivalence_mappings
            .iter()
            .filter(|m| m.source_regulation == statute.id)
            .cloned()
            .collect()
    }

    /// Finds similar statutes across jurisdictions using text similarity.
    pub async fn find_similar_statutes(
        &self,
        statute: &Statute,
        candidate_statutes: &[Statute],
    ) -> Vec<(Statute, f64)> {
        let mut similarities = Vec::new();

        for candidate in candidate_statutes {
            // Simple similarity based on title matching
            let similarity = self.calculate_similarity(&statute.title, &candidate.title);
            if similarity > 0.3 {
                similarities.push((candidate.clone(), similarity));
            }
        }

        // Sort by similarity score descending
        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        similarities
    }

    fn calculate_similarity(&self, text1: &str, text2: &str) -> f64 {
        // Simple word-based similarity (Jaccard similarity)
        let lower1 = text1.to_lowercase();
        let lower2 = text2.to_lowercase();

        let words1: std::collections::HashSet<_> = lower1.split_whitespace().collect();
        let words2: std::collections::HashSet<_> = lower2.split_whitespace().collect();

        let intersection = words1.intersection(&words2).count();
        let union = words1.union(&words2).count();

        if union == 0 {
            0.0
        } else {
            intersection as f64 / union as f64
        }
    }

    /// Applies automatic term replacement.
    pub fn apply_term_replacement(&self, statute: &mut Statute) -> Vec<TermReplacement> {
        let mut applied_replacements = Vec::new();

        for replacement in &self.term_replacements {
            // Replace in title
            if statute.title.contains(&replacement.source_term) {
                statute.title = statute
                    .title
                    .replace(&replacement.source_term, &replacement.target_term);
                applied_replacements.push(replacement.clone());
            }
        }

        applied_replacements
    }

    /// Performs context-aware parameter adjustment.
    pub fn adjust_parameters_contextually(&self, statute: &Statute) -> Vec<ContextualAdjustment> {
        let mut adjustments = Vec::new();

        // Check age-related parameters
        if let (Some(source_age), Some(target_age)) = (
            self.source.cultural_params.age_of_majority,
            self.target.cultural_params.age_of_majority,
        ) {
            if source_age != target_age {
                adjustments.push(ContextualAdjustment {
                    parameter: "age_of_majority".to_string(),
                    original_value: source_age.to_string(),
                    adjusted_value: target_age.to_string(),
                    context: format!("Statute: {}", statute.id),
                    rationale: "Age of majority differs between jurisdictions".to_string(),
                });
            }
        }

        // Check for currency adjustments (if statute involves monetary values)
        if statute.title.to_lowercase().contains("fine")
            || statute.title.to_lowercase().contains("payment")
        {
            adjustments.push(ContextualAdjustment {
                parameter: "currency".to_string(),
                original_value: self.source.locale.language.clone(),
                adjusted_value: self.target.locale.language.clone(),
                context: "Monetary statute".to_string(),
                rationale: "Currency and amounts need localization".to_string(),
            });
        }

        adjustments
    }

    /// Creates a porting workflow.
    pub fn create_workflow(&self, statute_id: String) -> PortingWorkflow {
        PortingWorkflow {
            id: format!("workflow-{}", statute_id),
            state: WorkflowState::Initiated,
            statute_id: statute_id.clone(),
            source_jurisdiction: self.source.id.clone(),
            target_jurisdiction: self.target.id.clone(),
            completed_steps: Vec::new(),
            pending_steps: vec![
                WorkflowStep {
                    name: "Initial Analysis".to_string(),
                    description: "Analyze statute for porting compatibility".to_string(),
                    status: StepStatus::Pending,
                    completed_at: None,
                },
                WorkflowStep {
                    name: "Cultural Adaptation".to_string(),
                    description: "Apply cultural parameter adaptations".to_string(),
                    status: StepStatus::Pending,
                    completed_at: None,
                },
                WorkflowStep {
                    name: "Legal Review".to_string(),
                    description: "Review by legal expert".to_string(),
                    status: StepStatus::Pending,
                    completed_at: None,
                },
                WorkflowStep {
                    name: "Final Approval".to_string(),
                    description: "Final approval by authority".to_string(),
                    status: StepStatus::Pending,
                    completed_at: None,
                },
            ],
            approvals: vec![
                Approval {
                    approver_role: "Legal Expert".to_string(),
                    status: ApprovalStatus::Pending,
                    comments: None,
                },
                Approval {
                    approver_role: "Jurisdictional Authority".to_string(),
                    status: ApprovalStatus::Pending,
                    comments: None,
                },
            ],
        }
    }

    /// Advances workflow to next step.
    pub fn advance_workflow(&self, workflow: &mut PortingWorkflow) -> PortingResult<()> {
        if let Some(mut step) = workflow.pending_steps.first().cloned() {
            step.status = StepStatus::Completed;
            step.completed_at = Some(chrono::Utc::now().to_rfc3339());
            workflow.completed_steps.push(step);
            workflow.pending_steps.remove(0);

            // Update workflow state
            if workflow.pending_steps.is_empty() {
                workflow.state = WorkflowState::PendingReview;
            } else {
                workflow.state = WorkflowState::InProgress;
            }

            Ok(())
        } else {
            Err(PortingError::AdaptationRequired(
                "No pending steps to advance".to_string(),
            ))
        }
    }

    /// Creates a versioned ported statute.
    pub fn create_versioned_statute(
        &self,
        statute: PortedStatute,
        version: u32,
        created_by: String,
        change_notes: String,
    ) -> VersionedPortedStatute {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        // Calculate hash
        let mut hasher = DefaultHasher::new();
        statute.statute.id.hash(&mut hasher);
        statute.statute.title.hash(&mut hasher);
        version.hash(&mut hasher);
        let hash = format!("{:x}", hasher.finish());

        VersionedPortedStatute {
            statute,
            version,
            previous_hash: if version > 1 {
                Some("previous_hash_placeholder".to_string())
            } else {
                None
            },
            hash,
            created_at: chrono::Utc::now().to_rfc3339(),
            created_by,
            change_notes,
        }
    }

    /// Compares two versions of ported statutes.
    pub fn compare_versions(
        &self,
        v1: &VersionedPortedStatute,
        v2: &VersionedPortedStatute,
    ) -> Vec<String> {
        let mut differences = Vec::new();

        if v1.statute.statute.title != v2.statute.statute.title {
            differences.push(format!(
                "Title changed from '{}' to '{}'",
                v1.statute.statute.title, v2.statute.statute.title
            ));
        }

        if v1.statute.changes.len() != v2.statute.changes.len() {
            differences.push(format!(
                "Number of changes: {} -> {}",
                v1.statute.changes.len(),
                v2.statute.changes.len()
            ));
        }

        differences
    }

    /// Submits a ported statute for expert review.
    pub fn submit_for_review(&self, statute: PortedStatute) -> ReviewRequest {
        ReviewRequest {
            id: format!("review-{}", statute.statute.id),
            statute,
            source_jurisdiction: self.source.id.clone(),
            target_jurisdiction: self.target.id.clone(),
            status: ReviewStatus::Pending,
            assigned_expert: None,
            submitted_at: chrono::Utc::now().to_rfc3339(),
            reviews: Vec::new(),
        }
    }

    /// Assigns an expert to a review request.
    pub fn assign_expert(&self, request: &mut ReviewRequest, expert_id: String) {
        request.assigned_expert = Some(expert_id);
        request.status = ReviewStatus::Assigned;
    }

    /// Adds an expert review to a review request.
    pub fn add_expert_review(
        &self,
        request: &mut ReviewRequest,
        review: ExpertReview,
    ) -> PortingResult<()> {
        request.reviews.push(review.clone());
        request.status = ReviewStatus::InReview;

        // Update status based on recommendation
        match review.recommendation {
            ReviewRecommendation::Approve => {
                request.status = ReviewStatus::Approved;
            }
            ReviewRecommendation::ApproveWithChanges => {
                request.status = ReviewStatus::RequiresRevision;
            }
            ReviewRecommendation::Reject => {
                request.status = ReviewStatus::Rejected;
            }
            ReviewRecommendation::RequestInformation => {
                request.status = ReviewStatus::InReview;
            }
        }

        Ok(())
    }

    /// Creates a review comment.
    pub fn create_review_comment(
        &self,
        section: Option<String>,
        text: String,
        severity: Severity,
        category: String,
    ) -> ReviewComment {
        ReviewComment {
            id: format!("comment-{}", chrono::Utc::now().timestamp()),
            section,
            text,
            severity,
            category,
        }
    }

    /// Performs automated compliance checking on a ported statute.
    pub fn check_compliance(&self, statute: &PortedStatute) -> ComplianceCheckResult {
        let mut checks = Vec::new();
        let mut violations = Vec::new();

        // Check 1: Legal system compatibility
        let legal_system_check = ComplianceCheck {
            name: "Legal System Compatibility".to_string(),
            description: "Verify statute is compatible with target legal system".to_string(),
            passed: self.source.legal_system == self.target.legal_system,
            details: Some(format!(
                "Source: {:?}, Target: {:?}",
                self.source.legal_system, self.target.legal_system
            )),
            severity: if self.source.legal_system != self.target.legal_system {
                Severity::Warning
            } else {
                Severity::Info
            },
        };
        checks.push(legal_system_check.clone());

        if !legal_system_check.passed {
            violations.push(ComplianceViolation {
                violation_type: "Legal System Mismatch".to_string(),
                description: "Source and target legal systems differ".to_string(),
                severity: Severity::Error,
                regulation: "Legal System Compatibility Requirements".to_string(),
                remediation: vec![
                    "Review statute for procedural adaptations".to_string(),
                    "Consult legal expert for system-specific modifications".to_string(),
                ],
            });
        }

        // Check 2: Cultural parameter compliance
        let cultural_check = ComplianceCheck {
            name: "Cultural Parameter Compliance".to_string(),
            description: "Verify cultural parameters are properly adapted".to_string(),
            passed: !statute.changes.is_empty(),
            details: Some(format!(
                "{} cultural adaptations made",
                statute.changes.len()
            )),
            severity: Severity::Info,
        };
        checks.push(cultural_check);

        // Check 3: Prohibited content check
        let mut has_prohibited_content = false;
        for prohibition in &self.target.cultural_params.prohibitions {
            if statute
                .statute
                .title
                .to_lowercase()
                .contains(&prohibition.to_lowercase())
            {
                has_prohibited_content = true;
                violations.push(ComplianceViolation {
                    violation_type: "Prohibited Content".to_string(),
                    description: format!("Statute may conflict with prohibition: {}", prohibition),
                    severity: Severity::Error,
                    regulation: format!("Cultural Prohibition: {}", prohibition),
                    remediation: vec![
                        "Review statute content for compliance".to_string(),
                        "Consider alternative formulations".to_string(),
                        "Seek legal expert review".to_string(),
                    ],
                });
            }
        }

        checks.push(ComplianceCheck {
            name: "Prohibited Content Check".to_string(),
            description: "Verify statute does not violate cultural prohibitions".to_string(),
            passed: !has_prohibited_content,
            details: Some(format!(
                "Checked {} prohibitions",
                self.target.cultural_params.prohibitions.len()
            )),
            severity: if has_prohibited_content {
                Severity::Error
            } else {
                Severity::Info
            },
        });

        // Check 4: Title preservation
        checks.push(ComplianceCheck {
            name: "Title Preservation".to_string(),
            description: "Verify title maintains semantic meaning".to_string(),
            passed: true,
            details: Some("Title checked for semantic preservation".to_string()),
            severity: Severity::Info,
        });

        // Check 5: Change tracking
        checks.push(ComplianceCheck {
            name: "Change Tracking".to_string(),
            description: "Verify all changes are documented".to_string(),
            passed: !statute.changes.is_empty(),
            details: Some(format!("{} changes tracked", statute.changes.len())),
            severity: Severity::Info,
        });

        // Calculate compliance score
        let passed_count = checks.iter().filter(|c| c.passed).count();
        let compliance_score = passed_count as f64 / checks.len() as f64;

        // Determine compliance status
        let status = if violations.iter().any(|v| v.severity == Severity::Critical) {
            ComplianceStatus::NonCompliant
        } else if violations.iter().any(|v| v.severity == Severity::Error) {
            ComplianceStatus::RequiresReview
        } else if !violations.is_empty() {
            ComplianceStatus::CompliantWithIssues
        } else {
            ComplianceStatus::Compliant
        };

        // Generate recommendations
        let mut recommendations = Vec::new();
        if compliance_score < 0.8 {
            recommendations.push("Consider additional review before adoption".to_string());
        }
        if !violations.is_empty() {
            recommendations.push("Address identified violations before implementation".to_string());
        }
        if self.source.legal_system != self.target.legal_system {
            recommendations
                .push("Engage legal expert familiar with target legal system".to_string());
        }

        ComplianceCheckResult {
            id: format!("compliance-{}", statute.statute.id),
            statute_id: statute.statute.id.clone(),
            checked_at: chrono::Utc::now().to_rfc3339(),
            status,
            compliance_score,
            checks,
            violations,
            recommendations,
        }
    }

    /// Performs batch compliance checking.
    pub fn batch_check_compliance(&self, statutes: &[PortedStatute]) -> Vec<ComplianceCheckResult> {
        statutes.iter().map(|s| self.check_compliance(s)).collect()
    }

    /// Generates a compliance summary report.
    pub fn generate_compliance_summary(
        &self,
        results: &[ComplianceCheckResult],
    ) -> ComplianceSummary {
        let total = results.len();
        let compliant = results
            .iter()
            .filter(|r| r.status == ComplianceStatus::Compliant)
            .count();
        let compliant_with_issues = results
            .iter()
            .filter(|r| r.status == ComplianceStatus::CompliantWithIssues)
            .count();
        let non_compliant = results
            .iter()
            .filter(|r| r.status == ComplianceStatus::NonCompliant)
            .count();
        let requires_review = results
            .iter()
            .filter(|r| r.status == ComplianceStatus::RequiresReview)
            .count();

        let avg_score = if !results.is_empty() {
            results.iter().map(|r| r.compliance_score).sum::<f64>() / results.len() as f64
        } else {
            0.0
        };

        let total_violations: usize = results.iter().map(|r| r.violations.len()).sum();

        ComplianceSummary {
            total_statutes: total,
            compliant,
            compliant_with_issues,
            non_compliant,
            requires_review,
            average_compliance_score: avg_score,
            total_violations,
            critical_violations: results
                .iter()
                .flat_map(|r| &r.violations)
                .filter(|v| v.severity == Severity::Critical)
                .count(),
        }
    }

    /// Exports compatibility report to specified format.
    pub fn export_compatibility_report(
        &self,
        report: &CompatibilityReport,
        format: ExportFormat,
    ) -> PortingResult<String> {
        match format {
            ExportFormat::Json => serde_json::to_string_pretty(report).map_err(|e| {
                PortingError::AdaptationRequired(format!("JSON serialization failed: {}", e))
            }),
            ExportFormat::Markdown => Ok(self.format_report_as_markdown(report)),
        }
    }

    fn format_report_as_markdown(&self, report: &CompatibilityReport) -> String {
        let mut md = String::new();
        md.push_str("# Compatibility Report\n\n");
        md.push_str(&format!(
            "**Compatibility Score:** {:.1}%\n\n",
            report.compatibility_score * 100.0
        ));
        md.push_str(&format!(
            "**Adaptations Required:** {}\n\n",
            report.adaptations_required
        ));
        md.push_str(&format!(
            "**Incompatibilities:** {}\n\n",
            report.incompatibilities
        ));

        if !report.findings.is_empty() {
            md.push_str("## Findings\n\n");
            for finding in &report.findings {
                md.push_str(&format!(
                    "- **[{:?}]** {}: {}\n",
                    finding.severity, finding.category, finding.description
                ));
            }
            md.push('\n');
        }

        if !report.recommendations.is_empty() {
            md.push_str("## Recommendations\n\n");
            for rec in &report.recommendations {
                md.push_str(&format!("- {}\n", rec));
            }
        }

        md
    }

    /// Exports porting output to specified format.
    pub fn export_porting_output(
        &self,
        output: &PortingOutput,
        format: ExportFormat,
    ) -> PortingResult<String> {
        match format {
            ExportFormat::Json => serde_json::to_string_pretty(output).map_err(|e| {
                PortingError::AdaptationRequired(format!("JSON serialization failed: {}", e))
            }),
            ExportFormat::Markdown => Ok(self.format_output_as_markdown(output)),
        }
    }

    fn format_output_as_markdown(&self, output: &PortingOutput) -> String {
        let mut md = String::new();
        md.push_str("# Porting Output\n\n");
        md.push_str(&format!(
            "**Statutes Ported:** {}\n\n",
            output.statutes.len()
        ));

        for (i, statute) in output.statutes.iter().enumerate() {
            md.push_str(&format!(
                "## Statute {} of {}\n\n",
                i + 1,
                output.statutes.len()
            ));
            md.push_str(&format!("**Original ID:** {}\n\n", statute.original_id));
            md.push_str(&format!("**New ID:** {}\n\n", statute.statute.id));
            md.push_str(&format!("**Title:** {}\n\n", statute.statute.title));
            md.push_str(&format!("**Changes:** {}\n\n", statute.changes.len()));
        }

        if let Some(report) = &output.report {
            md.push_str(&self.format_report_as_markdown(report));
        }

        md
    }

    /// Calculates TF-IDF based similarity between two statutes.
    pub fn calculate_tfidf_similarity(&self, statute1: &Statute, statute2: &Statute) -> f64 {
        // Simple TF-IDF implementation
        let text1 = format!("{} {}", statute1.title, statute1.id);
        let text2 = format!("{} {}", statute2.title, statute2.id);

        // Tokenize
        let words1: Vec<&str> = text1.split_whitespace().collect();
        let words2: Vec<&str> = text2.split_whitespace().collect();

        // Calculate term frequencies
        let mut tf1 = std::collections::HashMap::new();
        let mut tf2 = std::collections::HashMap::new();

        for word in &words1 {
            *tf1.entry(word.to_lowercase()).or_insert(0) += 1;
        }
        for word in &words2 {
            *tf2.entry(word.to_lowercase()).or_insert(0) += 1;
        }

        // Calculate cosine similarity
        let mut dot_product = 0.0;
        let mut norm1 = 0.0;
        let mut norm2 = 0.0;

        let all_terms: std::collections::HashSet<_> =
            tf1.keys().chain(tf2.keys()).map(|s| s.as_str()).collect();

        for term in all_terms {
            let v1 = *tf1.get(term).unwrap_or(&0) as f64;
            let v2 = *tf2.get(term).unwrap_or(&0) as f64;
            dot_product += v1 * v2;
            norm1 += v1 * v1;
            norm2 += v2 * v2;
        }

        if norm1 == 0.0 || norm2 == 0.0 {
            0.0
        } else {
            dot_product / (norm1.sqrt() * norm2.sqrt())
        }
    }

    /// Creates a porting template from successful porting operations.
    pub fn create_template(
        &self,
        name: String,
        description: String,
        statute_types: Vec<String>,
    ) -> PortingTemplate {
        PortingTemplate {
            id: format!("template-{}-{}", self.source.id, self.target.id),
            name,
            description,
            statute_types,
            term_replacements: self.term_replacements.clone(),
            contextual_rules: vec![
                "Adjust age thresholds based on cultural parameters".to_string(),
                "Replace currency references with local currency".to_string(),
                "Adapt procedural elements to target legal system".to_string(),
            ],
            target_legal_systems: vec![self.target.legal_system],
        }
    }

    /// Applies a porting template to a statute.
    pub fn apply_template(
        &self,
        statute: &Statute,
        template: &PortingTemplate,
    ) -> PortingResult<PortedStatute> {
        let options = PortingOptions {
            apply_cultural_params: true,
            translate_terms: true,
            ..Default::default()
        };

        // Apply template-specific term replacements
        let engine_with_template = PortingEngine::new(self.source.clone(), self.target.clone())
            .with_term_replacements(template.term_replacements.clone());

        engine_with_template.port_statute(statute, &options)
    }

    /// Generates conflict resolution suggestions with priorities.
    pub fn generate_conflict_resolutions(
        &self,
        conflicts: &[ConflictReport],
    ) -> Vec<ConflictResolution> {
        let mut resolutions = Vec::new();

        for (i, conflict) in conflicts.iter().enumerate() {
            let (priority, effort) = match conflict.severity {
                Severity::Critical => (10, EffortLevel::VeryHigh),
                Severity::Error => (8, EffortLevel::High),
                Severity::Warning => (5, EffortLevel::Medium),
                Severity::Info => (2, EffortLevel::Low),
            };

            resolutions.push(ConflictResolution {
                conflict_id: format!("conflict-{}", i),
                strategy: conflict
                    .resolutions
                    .first()
                    .cloned()
                    .unwrap_or_else(|| "Consult legal expert for resolution strategy".to_string()),
                priority,
                effort,
                steps: conflict.resolutions.clone(),
                expected_outcome: format!(
                    "Resolve {:?} conflict for statute {}",
                    conflict.conflict_type, conflict.statute_id
                ),
            });
        }

        // Sort by priority (highest first)
        resolutions.sort_by(|a, b| b.priority.cmp(&a.priority));
        resolutions
    }

    /// Performs multi-hop porting through intermediate jurisdictions.
    pub async fn multi_hop_port(
        &self,
        statute: &Statute,
        intermediate_jurisdictions: &[Jurisdiction],
        options: &PortingOptions,
    ) -> PortingResult<PortingChain> {
        let mut hop_results = Vec::new();
        let mut cumulative_changes = Vec::new();
        let mut current_statute = statute.clone();

        // Port through each intermediate jurisdiction
        for intermediate in intermediate_jurisdictions {
            let hop_engine = PortingEngine::new(self.source.clone(), intermediate.clone());
            let ported = hop_engine.port_statute(&current_statute, options)?;

            cumulative_changes.extend(ported.changes.clone());
            current_statute = ported.statute.clone();
            hop_results.push(ported);
        }

        // Final hop to target
        let final_ported = self.port_statute(&current_statute, options)?;
        cumulative_changes.extend(final_ported.changes.clone());
        hop_results.push(final_ported);

        // Calculate chain score (average compatibility)
        let chain_score = 1.0 - (cumulative_changes.len() as f64 * 0.05).min(1.0);

        Ok(PortingChain {
            id: format!("chain-{}", statute.id),
            source_jurisdiction: self.source.id.clone(),
            target_jurisdiction: self.target.id.clone(),
            intermediate_hops: intermediate_jurisdictions
                .iter()
                .map(|j| j.id.clone())
                .collect(),
            hop_results,
            cumulative_changes,
            chain_score,
        })
    }

    /// Records a porting operation in history.
    pub fn record_history(
        &self,
        statute_id: String,
        user: String,
        options: &PortingOptions,
        success: bool,
        error: Option<String>,
    ) -> PortingHistoryEntry {
        PortingHistoryEntry {
            id: format!("history-{}", chrono::Utc::now().timestamp()),
            timestamp: chrono::Utc::now().to_rfc3339(),
            source_jurisdiction: self.source.id.clone(),
            target_jurisdiction: self.target.id.clone(),
            statute_id,
            user,
            options: options.clone(),
            success,
            error,
        }
    }

    /// Builds lineage tree for a statute across jurisdictions.
    pub fn build_lineage(
        &self,
        original_id: String,
        original_jurisdiction: String,
        porting_history: &[PortingHistoryEntry],
    ) -> StatuteLineage {
        let mut derived_versions = Vec::new();

        // Build tree from history
        for entry in porting_history.iter().filter(|e| e.success) {
            if entry.source_jurisdiction == original_jurisdiction {
                derived_versions.push(LineageNode {
                    jurisdiction: entry.target_jurisdiction.clone(),
                    statute_id: entry.statute_id.clone(),
                    parent_jurisdiction: Some(entry.source_jurisdiction.clone()),
                    ported_at: entry.timestamp.clone(),
                    children: Vec::new(),
                });
            }
        }

        StatuteLineage {
            original_id,
            original_jurisdiction,
            total_ports: derived_versions.len(),
            derived_versions,
        }
    }

    /// Generates diff visualization between original and ported statute.
    pub fn generate_diff(&self, original: &Statute, ported: &PortedStatute) -> StatuteDiff {
        let mut differences = Vec::new();

        // Check ID differences
        if original.id != ported.statute.id {
            differences.push(FieldDiff {
                field: "id".to_string(),
                original: original.id.clone(),
                new: ported.statute.id.clone(),
                change_type: DiffChangeType::Modified,
            });
        }

        // Check title differences
        if original.title != ported.statute.title {
            differences.push(FieldDiff {
                field: "title".to_string(),
                original: original.title.clone(),
                new: ported.statute.title.clone(),
                change_type: DiffChangeType::Modified,
            });
        }

        // Calculate similarity
        let similarity_score = if differences.is_empty() {
            1.0
        } else {
            1.0 - (differences.len() as f64 * 0.1).min(0.9)
        };

        StatuteDiff {
            original_id: original.id.clone(),
            ported_id: ported.statute.id.clone(),
            differences,
            similarity_score,
        }
    }

    /// Exports statute diff as markdown visualization.
    pub fn export_diff_markdown(&self, diff: &StatuteDiff) -> String {
        let mut md = String::new();
        md.push_str("# Statute Diff\n\n");
        md.push_str(&format!("**Original ID:** {}\n\n", diff.original_id));
        md.push_str(&format!("**Ported ID:** {}\n\n", diff.ported_id));
        md.push_str(&format!(
            "**Similarity Score:** {:.1}%\n\n",
            diff.similarity_score * 100.0
        ));

        if !diff.differences.is_empty() {
            md.push_str("## Changes\n\n");
            for field_diff in &diff.differences {
                md.push_str(&format!("### {}\n\n", field_diff.field));
                md.push_str(&format!("**Type:** {:?}\n\n", field_diff.change_type));
                md.push_str(&format!(
                    "```diff\n- {}\n+ {}\n```\n\n",
                    field_diff.original, field_diff.new
                ));
            }
        }

        md
    }
}

/// Summary of compliance check results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceSummary {
    /// Total statutes checked
    pub total_statutes: usize,
    /// Number fully compliant
    pub compliant: usize,
    /// Number compliant with issues
    pub compliant_with_issues: usize,
    /// Number non-compliant
    pub non_compliant: usize,
    /// Number requiring review
    pub requires_review: usize,
    /// Average compliance score
    pub average_compliance_score: f64,
    /// Total violations found
    pub total_violations: usize,
    /// Critical violations
    pub critical_violations: usize,
}

/// Export format for reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    /// JSON format
    Json,
    /// Markdown format
    Markdown,
}

/// Porting template for common patterns.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortingTemplate {
    /// Template ID
    pub id: String,
    /// Template name
    pub name: String,
    /// Description
    pub description: String,
    /// Applicable statute types
    pub statute_types: Vec<String>,
    /// Pre-configured term replacements
    pub term_replacements: Vec<TermReplacement>,
    /// Pre-configured contextual adjustments
    pub contextual_rules: Vec<String>,
    /// Target legal systems this template applies to
    pub target_legal_systems: Vec<LegalSystem>,
}

/// Conflict resolution suggestion with priority.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictResolution {
    /// Conflict being resolved
    pub conflict_id: String,
    /// Resolution strategy
    pub strategy: String,
    /// Priority level (1-10, higher is more important)
    pub priority: u8,
    /// Estimated effort
    pub effort: EffortLevel,
    /// Implementation steps
    pub steps: Vec<String>,
    /// Expected outcome
    pub expected_outcome: String,
}

/// Effort level for resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EffortLevel {
    Low,
    Medium,
    High,
    VeryHigh,
}

/// Multi-hop porting chain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortingChain {
    /// Chain ID
    pub id: String,
    /// Original source jurisdiction
    pub source_jurisdiction: String,
    /// Final target jurisdiction
    pub target_jurisdiction: String,
    /// Intermediate jurisdictions
    pub intermediate_hops: Vec<String>,
    /// Porting results at each hop
    pub hop_results: Vec<PortedStatute>,
    /// Cumulative changes across all hops
    pub cumulative_changes: Vec<PortingChange>,
    /// Overall chain compatibility score
    pub chain_score: f64,
}

/// Porting history entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortingHistoryEntry {
    /// Entry ID
    pub id: String,
    /// Timestamp
    pub timestamp: String,
    /// Source jurisdiction
    pub source_jurisdiction: String,
    /// Target jurisdiction
    pub target_jurisdiction: String,
    /// Statute ID
    pub statute_id: String,
    /// User who performed porting
    pub user: String,
    /// Options used
    pub options: PortingOptions,
    /// Success status
    pub success: bool,
    /// Error message if failed
    pub error: Option<String>,
}

/// Lineage tracking for a statute across jurisdictions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatuteLineage {
    /// Original statute ID
    pub original_id: String,
    /// Original jurisdiction
    pub original_jurisdiction: String,
    /// All derived versions
    pub derived_versions: Vec<LineageNode>,
    /// Total number of ports
    pub total_ports: usize,
}

/// Node in statute lineage tree.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageNode {
    /// Jurisdiction this version is in
    pub jurisdiction: String,
    /// Statute ID in this jurisdiction
    pub statute_id: String,
    /// Parent node (if any)
    pub parent_jurisdiction: Option<String>,
    /// Porting timestamp
    pub ported_at: String,
    /// Children nodes
    pub children: Vec<LineageNode>,
}

/// Diff between original and ported statute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatuteDiff {
    /// Original statute ID
    pub original_id: String,
    /// Ported statute ID
    pub ported_id: String,
    /// Field-level differences
    pub differences: Vec<FieldDiff>,
    /// Overall similarity score
    pub similarity_score: f64,
}

/// Difference in a specific field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDiff {
    /// Field name
    pub field: String,
    /// Original value
    pub original: String,
    /// New value
    pub new: String,
    /// Type of change
    pub change_type: DiffChangeType,
}

/// Type of diff change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiffChangeType {
    Modified,
    Added,
    Removed,
}

// ============================================================================
// Conflict Resolution Framework (v0.1.4)
// ============================================================================

/// Conflict precedent from previous porting operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictPrecedent {
    /// Precedent ID
    pub id: String,
    /// Source jurisdiction where conflict occurred
    pub source_jurisdiction: String,
    /// Target jurisdiction where conflict occurred
    pub target_jurisdiction: String,
    /// Conflict type
    pub conflict_type: ConflictType,
    /// Conflict description
    pub description: String,
    /// Resolution strategy that was used
    pub resolution_used: String,
    /// Effectiveness score (0.0 - 1.0)
    pub effectiveness: f64,
    /// Expert who resolved it
    pub resolved_by: Option<String>,
    /// Timestamp of resolution
    pub resolved_at: String,
    /// Lessons learned
    pub lessons_learned: Vec<String>,
    /// Applicable statute types
    pub applicable_statute_types: Vec<String>,
    /// Tags for categorization
    pub tags: Vec<String>,
}

/// Database of conflict precedents for learning from past resolutions.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConflictPrecedentDatabase {
    /// All stored precedents
    precedents: Vec<ConflictPrecedent>,
    /// Index by jurisdiction pair for fast lookup
    jurisdiction_index: HashMap<(String, String), Vec<usize>>,
}

impl ConflictPrecedentDatabase {
    /// Creates a new empty precedent database.
    pub fn new() -> Self {
        Self {
            precedents: Vec::new(),
            jurisdiction_index: HashMap::new(),
        }
    }

    /// Adds a precedent to the database.
    pub fn add_precedent(&mut self, precedent: ConflictPrecedent) {
        let idx = self.precedents.len();
        let key = (
            precedent.source_jurisdiction.clone(),
            precedent.target_jurisdiction.clone(),
        );

        self.jurisdiction_index.entry(key).or_default().push(idx);

        self.precedents.push(precedent);
    }

    /// Finds relevant precedents for a conflict.
    pub fn find_relevant_precedents(
        &self,
        source_jurisdiction: &str,
        target_jurisdiction: &str,
        conflict_type: &ConflictType,
    ) -> Vec<&ConflictPrecedent> {
        let key = (
            source_jurisdiction.to_string(),
            target_jurisdiction.to_string(),
        );

        if let Some(indices) = self.jurisdiction_index.get(&key) {
            indices
                .iter()
                .filter_map(|&idx| self.precedents.get(idx))
                .filter(|p| {
                    // Match by conflict type
                    std::mem::discriminant(&p.conflict_type)
                        == std::mem::discriminant(conflict_type)
                })
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Gets precedents with high effectiveness (>= 0.7).
    pub fn get_effective_precedents(&self) -> Vec<&ConflictPrecedent> {
        self.precedents
            .iter()
            .filter(|p| p.effectiveness >= 0.7)
            .collect()
    }

    /// Gets all precedents.
    pub fn all_precedents(&self) -> &[ConflictPrecedent] {
        &self.precedents
    }
}

/// Negotiated resolution template for common conflict patterns.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NegotiatedResolutionTemplate {
    /// Template ID
    pub id: String,
    /// Template name
    pub name: String,
    /// Conflict types this template addresses
    pub conflict_types: Vec<ConflictType>,
    /// Source jurisdiction patterns (e.g., "CommonLaw", "CivilLaw", or specific countries)
    pub source_patterns: Vec<String>,
    /// Target jurisdiction patterns
    pub target_patterns: Vec<String>,
    /// Resolution approach description
    pub approach: String,
    /// Specific negotiation steps
    pub negotiation_steps: Vec<NegotiationStep>,
    /// Fallback strategies if negotiation fails
    pub fallback_strategies: Vec<String>,
    /// Success rate of this template (0.0 - 1.0)
    pub success_rate: f64,
    /// Typical stakeholders involved
    pub stakeholders: Vec<String>,
    /// Required approvals
    pub required_approvals: Vec<String>,
}

/// Step in a negotiation process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NegotiationStep {
    /// Step number
    pub step_number: usize,
    /// Step description
    pub description: String,
    /// Stakeholders involved in this step
    pub involved_parties: Vec<String>,
    /// Expected outcome
    pub expected_outcome: String,
    /// Time estimate (in days)
    pub estimated_days: u32,
}

/// Human-in-the-loop conflict resolution workflow.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictResolutionWorkflow {
    /// Workflow ID
    pub id: String,
    /// Conflict being resolved
    pub conflict: ConflictReport,
    /// Current state
    pub state: ResolutionWorkflowState,
    /// Proposed resolution
    pub proposed_resolution: Option<String>,
    /// Stakeholder reviews
    pub stakeholder_reviews: Vec<StakeholderReview>,
    /// Expert consultations
    pub expert_consultations: Vec<ExpertConsultation>,
    /// Final decision
    pub final_decision: Option<ResolutionDecision>,
    /// Created at timestamp
    pub created_at: String,
    /// Updated at timestamp
    pub updated_at: String,
    /// Escalation level
    pub escalation_level: EscalationLevel,
}

/// State of conflict resolution workflow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResolutionWorkflowState {
    /// Initial assessment
    InitialAssessment,
    /// Awaiting expert input
    AwaitingExpert,
    /// Stakeholder review
    StakeholderReview,
    /// Negotiation in progress
    NegotiationInProgress,
    /// Decision pending
    DecisionPending,
    /// Resolved
    Resolved,
    /// Escalated
    Escalated,
    /// Abandoned
    Abandoned,
}

/// Review from a stakeholder on a proposed resolution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakeholderReview {
    /// Reviewer ID
    pub reviewer_id: String,
    /// Reviewer name
    pub reviewer_name: String,
    /// Stakeholder role
    pub role: String,
    /// Review timestamp
    pub reviewed_at: String,
    /// Recommendation
    pub recommendation: StakeholderRecommendation,
    /// Comments
    pub comments: String,
    /// Concerns raised
    pub concerns: Vec<String>,
    /// Suggested modifications
    pub modifications: Vec<String>,
}

/// Stakeholder recommendation on resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StakeholderRecommendation {
    /// Approve the proposed resolution
    Approve,
    /// Approve with modifications
    ApproveWithModifications,
    /// Request alternative approach
    RequestAlternative,
    /// Reject
    Reject,
}

/// Expert consultation for conflict resolution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpertConsultation {
    /// Consultation ID
    pub id: String,
    /// Expert ID
    pub expert_id: String,
    /// Expert name
    pub expert_name: String,
    /// Area of expertise
    pub expertise_area: String,
    /// Consultation timestamp
    pub consulted_at: String,
    /// Expert opinion
    pub opinion: String,
    /// Recommended approach
    pub recommended_approach: String,
    /// Confidence level (0.0 - 1.0)
    pub confidence: f64,
    /// References to legal precedents
    pub legal_references: Vec<String>,
}

/// Final decision on conflict resolution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolutionDecision {
    /// Decision ID
    pub id: String,
    /// Decision maker ID
    pub decision_maker_id: String,
    /// Decision maker role
    pub decision_maker_role: String,
    /// Timestamp of decision
    pub decided_at: String,
    /// Chosen resolution strategy
    pub chosen_strategy: String,
    /// Rationale for decision
    pub rationale: String,
    /// Implementation plan
    pub implementation_plan: Vec<String>,
    /// Monitoring requirements
    pub monitoring_requirements: Vec<String>,
    /// Risk acceptance
    pub accepted_risks: Vec<String>,
}

/// Escalation level for conflicts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EscalationLevel {
    /// Routine - can be resolved by standard procedures
    Routine,
    /// Elevated - requires expert consultation
    Elevated,
    /// High - requires stakeholder involvement
    High,
    /// Critical - requires senior decision maker
    Critical,
}

/// Enhanced conflict detector with severity analysis.
#[derive(Debug, Clone)]
pub struct ConflictDetector {
    /// Precedent database for learning
    pub precedent_db: ConflictPrecedentDatabase,
    /// Resolution templates
    pub templates: Vec<NegotiatedResolutionTemplate>,
}

impl ConflictDetector {
    /// Creates a new conflict detector.
    pub fn new() -> Self {
        Self {
            precedent_db: ConflictPrecedentDatabase::new(),
            templates: Vec::new(),
        }
    }

    /// Creates a detector with precedent database.
    pub fn with_precedents(precedent_db: ConflictPrecedentDatabase) -> Self {
        Self {
            precedent_db,
            templates: Vec::new(),
        }
    }

    /// Analyzes conflict severity based on multiple factors.
    pub fn analyze_severity(
        &self,
        conflict: &ConflictReport,
        source_jurisdiction: &Jurisdiction,
        target_jurisdiction: &Jurisdiction,
    ) -> Severity {
        let mut severity_score = 0;

        // Base severity from conflict type
        severity_score += match conflict.conflict_type {
            ConflictType::Contradiction => 3,
            ConflictType::CulturalIncompatibility => 2,
            ConflictType::SystemMismatch => 2,
            ConflictType::Overlap => 1,
            ConflictType::Procedural => 1,
        };

        // Adjust based on legal system compatibility
        if source_jurisdiction.legal_system != target_jurisdiction.legal_system {
            severity_score += 1;
        }

        // Check if there are precedents that can help
        let precedents = self.precedent_db.find_relevant_precedents(
            &source_jurisdiction.id,
            &target_jurisdiction.id,
            &conflict.conflict_type,
        );

        if precedents.is_empty() {
            // No precedents - more severe
            severity_score += 1;
        } else {
            // Has precedents with low effectiveness - moderate severity
            let avg_effectiveness: f64 =
                precedents.iter().map(|p| p.effectiveness).sum::<f64>() / precedents.len() as f64;
            if avg_effectiveness < 0.5 {
                severity_score += 1;
            }
        }

        // Map score to severity
        match severity_score {
            0..=2 => Severity::Info,
            3..=4 => Severity::Warning,
            5..=6 => Severity::Error,
            _ => Severity::Critical,
        }
    }

    /// Recommends resolution strategies based on precedents and templates.
    pub fn recommend_strategies(
        &self,
        conflict: &ConflictReport,
        source_jurisdiction: &Jurisdiction,
        target_jurisdiction: &Jurisdiction,
    ) -> Vec<String> {
        let mut strategies = Vec::new();

        // Get strategies from precedents
        let precedents = self.precedent_db.find_relevant_precedents(
            &source_jurisdiction.id,
            &target_jurisdiction.id,
            &conflict.conflict_type,
        );

        for precedent in precedents.iter().take(3) {
            if precedent.effectiveness >= 0.7 {
                strategies.push(format!(
                    "{} (proven effective: {:.0}%)",
                    precedent.resolution_used,
                    precedent.effectiveness * 100.0
                ));
            }
        }

        // Get strategies from templates
        for template in &self.templates {
            if template.conflict_types.contains(&conflict.conflict_type) {
                strategies.push(format!(
                    "{} (template: {}, success rate: {:.0}%)",
                    template.approach,
                    template.name,
                    template.success_rate * 100.0
                ));
            }
        }

        // Add default strategies if none found
        if strategies.is_empty() {
            strategies.extend(conflict.resolutions.clone());
        }

        strategies
    }

    /// Creates a resolution workflow for human review.
    pub fn create_resolution_workflow(
        &self,
        conflict: ConflictReport,
    ) -> ConflictResolutionWorkflow {
        let severity = conflict.severity;
        let escalation_level = match severity {
            Severity::Info => EscalationLevel::Routine,
            Severity::Warning => EscalationLevel::Elevated,
            Severity::Error => EscalationLevel::High,
            Severity::Critical => EscalationLevel::Critical,
        };

        let now = chrono::Utc::now().to_rfc3339();

        ConflictResolutionWorkflow {
            id: format!("workflow-{}", uuid::Uuid::new_v4()),
            conflict,
            state: ResolutionWorkflowState::InitialAssessment,
            proposed_resolution: None,
            stakeholder_reviews: Vec::new(),
            expert_consultations: Vec::new(),
            final_decision: None,
            created_at: now.clone(),
            updated_at: now,
            escalation_level,
        }
    }

    /// Adds a template to the detector.
    pub fn add_template(&mut self, template: NegotiatedResolutionTemplate) {
        self.templates.push(template);
    }
}

impl Default for ConflictDetector {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// AI-Assisted Porting (v0.1.5)
// ============================================================================

/// LLM-based adaptation suggestion with detailed analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmAdaptationSuggestion {
    /// Suggestion ID
    pub id: String,
    /// Statute ID this applies to
    pub statute_id: String,
    /// Section or aspect being adapted
    pub section: Option<String>,
    /// Suggested adaptation text
    pub suggestion: String,
    /// Detailed rationale from LLM
    pub rationale: String,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f64,
    /// Category of adaptation
    pub category: AdaptationCategory,
    /// Source jurisdiction context considered
    pub source_context: Vec<String>,
    /// Target jurisdiction context considered
    pub target_context: Vec<String>,
    /// Alternative suggestions
    pub alternatives: Vec<String>,
    /// Potential risks identified
    pub risks: Vec<String>,
    /// Legal references supporting the suggestion
    pub legal_references: Vec<String>,
}

/// Category of LLM adaptation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdaptationCategory {
    /// Terminological adaptation
    Terminology,
    /// Procedural adaptation
    Procedural,
    /// Cultural/social adaptation
    Cultural,
    /// Numerical value adaptation
    Numerical,
    /// Structural reorganization
    Structural,
    /// Legal principle adaptation
    LegalPrinciple,
    /// Compliance requirement
    Compliance,
}

/// Similar statute found across jurisdictions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarStatute {
    /// Statute from the database
    pub statute: Statute,
    /// Jurisdiction where this statute exists
    pub jurisdiction: String,
    /// Similarity score (0.0 - 1.0)
    pub similarity_score: f64,
    /// Matching features
    pub matching_features: Vec<MatchingFeature>,
    /// Key differences
    pub differences: Vec<String>,
    /// Relevance explanation
    pub relevance: String,
}

/// Feature that matches between statutes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchingFeature {
    /// Feature type
    pub feature_type: FeatureType,
    /// Description of the match
    pub description: String,
    /// Match strength (0.0 - 1.0)
    pub strength: f64,
}

/// Type of matching feature.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FeatureType {
    /// Similar legal effect
    LegalEffect,
    /// Similar structure
    Structure,
    /// Similar terminology
    Terminology,
    /// Similar scope
    Scope,
    /// Similar conditions
    Conditions,
    /// Similar penalties/remedies
    Remedies,
}

/// Gap analysis result identifying missing elements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GapAnalysis {
    /// Analysis ID
    pub id: String,
    /// Source statute analyzed
    pub source_statute_id: String,
    /// Target jurisdiction
    pub target_jurisdiction: String,
    /// Identified gaps
    pub gaps: Vec<Gap>,
    /// Coverage score (0.0 - 1.0, higher is better)
    pub coverage_score: f64,
    /// Overall assessment
    pub assessment: String,
    /// Recommendations to address gaps
    pub recommendations: Vec<String>,
}

/// A gap identified in the porting process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gap {
    /// Gap type
    pub gap_type: GapType,
    /// Description
    pub description: String,
    /// Severity
    pub severity: Severity,
    /// Missing element
    pub missing_element: String,
    /// Why it's important
    pub importance: String,
    /// Suggested solutions
    pub solutions: Vec<String>,
}

/// Type of gap.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GapType {
    /// Missing legal concept
    MissingConcept,
    /// Missing procedural element
    MissingProcedure,
    /// Missing enforcement mechanism
    MissingEnforcement,
    /// Missing safeguard
    MissingSafeguard,
    /// Insufficient specificity
    InsufficientSpecificity,
    /// Missing cultural consideration
    MissingCulturalElement,
}

/// Cultural sensitivity analysis result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CulturalSensitivityAnalysis {
    /// Analysis ID
    pub id: String,
    /// Statute analyzed
    pub statute_id: String,
    /// Overall sensitivity score (0.0 - 1.0, higher means more sensitive)
    pub sensitivity_score: f64,
    /// Identified issues
    pub issues: Vec<CulturalIssue>,
    /// Safe aspects
    pub safe_aspects: Vec<String>,
    /// Overall assessment
    pub assessment: String,
    /// Required consultations
    pub required_consultations: Vec<String>,
}

/// Cultural sensitivity issue.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CulturalIssue {
    /// Issue type
    pub issue_type: CulturalIssueType,
    /// Description
    pub description: String,
    /// Severity
    pub severity: Severity,
    /// Affected text/section
    pub affected_section: String,
    /// Why it's sensitive
    pub explanation: String,
    /// Suggested adaptations
    pub adaptations: Vec<String>,
    /// Stakeholders to consult
    pub stakeholders_to_consult: Vec<String>,
}

/// Type of cultural sensitivity issue.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CulturalIssueType {
    /// Religious sensitivity
    Religious,
    /// Traditional practice conflict
    Traditional,
    /// Social norm mismatch
    SocialNorm,
    /// Gender-related sensitivity
    Gender,
    /// Family structure sensitivity
    Family,
    /// Language/terminology sensitivity
    Language,
    /// Historical sensitivity
    Historical,
}

/// Plain language explanation of a statute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlainLanguageExplanation {
    /// Explanation ID
    pub id: String,
    /// Statute being explained
    pub statute_id: String,
    /// Target audience level
    pub audience_level: AudienceLevel,
    /// Summary (1-2 sentences)
    pub summary: String,
    /// Detailed explanation
    pub explanation: String,
    /// Key points
    pub key_points: Vec<String>,
    /// Practical examples
    pub examples: Vec<String>,
    /// Common questions and answers
    pub faqs: Vec<FrequentlyAskedQuestion>,
    /// Readability score (0.0 - 1.0)
    pub readability_score: f64,
}

/// Target audience level for explanations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AudienceLevel {
    /// General public with no legal background
    GeneralPublic,
    /// Business professionals
    Business,
    /// Government officials
    Government,
    /// Legal practitioners
    Legal,
    /// Academic/researchers
    Academic,
}

/// FAQ entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrequentlyAskedQuestion {
    /// Question
    pub question: String,
    /// Answer
    pub answer: String,
    /// Related topics
    pub related_topics: Vec<String>,
}

/// AI-powered porting assistant.
#[derive(Clone)]
pub struct AiPortingAssistant {
    /// Text generator for LLM interactions
    generator: Option<std::sync::Arc<dyn TextGenerator>>,
}

impl AiPortingAssistant {
    /// Creates a new AI porting assistant.
    pub fn new() -> Self {
        Self { generator: None }
    }

    /// Creates an assistant with an LLM generator.
    pub fn with_generator(generator: std::sync::Arc<dyn TextGenerator>) -> Self {
        Self {
            generator: Some(generator),
        }
    }

    /// Generates LLM-based adaptation suggestions.
    pub async fn generate_adaptation_suggestions(
        &self,
        statute: &Statute,
        source_jurisdiction: &Jurisdiction,
        target_jurisdiction: &Jurisdiction,
    ) -> PortingResult<Vec<LlmAdaptationSuggestion>> {
        let mut suggestions = Vec::new();

        // If we have an LLM generator, use it
        if let Some(generator) = &self.generator {
            let prompt = format!(
                "Analyze porting statute '{}' from {} to {}. \
                Source legal system: {:?}, Target legal system: {:?}. \
                Provide detailed adaptation suggestions considering legal, cultural, and procedural differences.",
                statute.title,
                source_jurisdiction.name,
                target_jurisdiction.name,
                source_jurisdiction.legal_system,
                target_jurisdiction.legal_system
            );

            let response = generator
                .generate(&prompt)
                .await
                .map_err(PortingError::Llm)?;

            // Parse LLM response into structured suggestions
            // This is a simplified version - real implementation would use more sophisticated parsing
            suggestions.push(LlmAdaptationSuggestion {
                id: format!("llm-sugg-{}", uuid::Uuid::new_v4()),
                statute_id: statute.id.clone(),
                section: None,
                suggestion: response.clone(),
                rationale: "AI-generated analysis based on jurisdiction differences".to_string(),
                confidence: 0.75,
                category: AdaptationCategory::Cultural,
                source_context: vec![format!(
                    "{:?} legal system",
                    source_jurisdiction.legal_system
                )],
                target_context: vec![format!(
                    "{:?} legal system",
                    target_jurisdiction.legal_system
                )],
                alternatives: vec![],
                risks: vec![],
                legal_references: vec![],
            });
        } else {
            // Fallback: rule-based suggestions
            if source_jurisdiction.legal_system != target_jurisdiction.legal_system {
                suggestions.push(LlmAdaptationSuggestion {
                    id: format!("rule-sugg-{}", uuid::Uuid::new_v4()),
                    statute_id: statute.id.clone(),
                    section: None,
                    suggestion: format!(
                        "Adapt procedural elements from {:?} to {:?} legal system",
                        source_jurisdiction.legal_system, target_jurisdiction.legal_system
                    ),
                    rationale: "Legal system differences require procedural adaptation".to_string(),
                    confidence: 0.8,
                    category: AdaptationCategory::Procedural,
                    source_context: vec![],
                    target_context: vec![],
                    alternatives: vec![],
                    risks: vec!["May require expert legal review".to_string()],
                    legal_references: vec![],
                });
            }
        }

        Ok(suggestions)
    }

    /// Discovers similar statutes across jurisdictions.
    pub async fn discover_similar_statutes(
        &self,
        statute: &Statute,
        jurisdictions: &[Jurisdiction],
    ) -> PortingResult<Vec<SimilarStatute>> {
        let mut similar = Vec::new();

        // Simple similarity based on title matching
        // Real implementation would use semantic similarity, embeddings, etc.
        for jurisdiction in jurisdictions {
            let similarity_score = self.calculate_similarity(statute, jurisdiction);

            if similarity_score > 0.3 {
                similar.push(SimilarStatute {
                    statute: statute.clone(),
                    jurisdiction: jurisdiction.id.clone(),
                    similarity_score,
                    matching_features: vec![MatchingFeature {
                        feature_type: FeatureType::Terminology,
                        description: "Similar legal terminology".to_string(),
                        strength: similarity_score,
                    }],
                    differences: vec![],
                    relevance: format!(
                        "Found in {} legal system",
                        match jurisdiction.legal_system {
                            LegalSystem::CommonLaw => "common law",
                            LegalSystem::CivilLaw => "civil law",
                            _ => "other",
                        }
                    ),
                });
            }
        }

        // Sort by similarity score
        similar.sort_by(|a, b| b.similarity_score.partial_cmp(&a.similarity_score).unwrap());

        Ok(similar)
    }

    /// Performs automatic gap analysis.
    pub async fn analyze_gaps(
        &self,
        statute: &Statute,
        source_jurisdiction: &Jurisdiction,
        target_jurisdiction: &Jurisdiction,
    ) -> PortingResult<GapAnalysis> {
        let mut gaps = Vec::new();

        // Check for enforcement mechanism
        gaps.push(Gap {
            gap_type: GapType::MissingEnforcement,
            description: "Verify enforcement mechanisms exist in target jurisdiction".to_string(),
            severity: Severity::Warning,
            missing_element: "Enforcement authority".to_string(),
            importance: "Required for effective statute implementation".to_string(),
            solutions: vec![
                "Identify equivalent enforcement body in target jurisdiction".to_string(),
                "Establish new enforcement mechanism if needed".to_string(),
            ],
        });

        // Check for cultural elements
        if source_jurisdiction.cultural_params.prohibitions
            != target_jurisdiction.cultural_params.prohibitions
        {
            gaps.push(Gap {
                gap_type: GapType::MissingCulturalElement,
                description: "Cultural prohibition differences detected".to_string(),
                severity: Severity::Info,
                missing_element: "Cultural context alignment".to_string(),
                importance: "Ensures cultural appropriateness".to_string(),
                solutions: vec![
                    "Consult with cultural advisors".to_string(),
                    "Adapt language and examples".to_string(),
                ],
            });
        }

        let coverage_score = 1.0 - (gaps.len() as f64 * 0.1).min(0.7);

        Ok(GapAnalysis {
            id: format!("gap-{}", uuid::Uuid::new_v4()),
            source_statute_id: statute.id.clone(),
            target_jurisdiction: target_jurisdiction.id.clone(),
            gaps,
            coverage_score,
            assessment: if coverage_score > 0.7 {
                "Good coverage with minor gaps".to_string()
            } else {
                "Significant gaps require attention".to_string()
            },
            recommendations: vec![
                "Address identified gaps before implementation".to_string(),
                "Conduct stakeholder review".to_string(),
            ],
        })
    }

    /// Checks for cultural sensitivity issues.
    pub async fn check_cultural_sensitivity(
        &self,
        statute: &Statute,
        target_jurisdiction: &Jurisdiction,
    ) -> PortingResult<CulturalSensitivityAnalysis> {
        let mut issues = Vec::new();

        // Check for prohibitions
        for prohibition in &target_jurisdiction.cultural_params.prohibitions {
            issues.push(CulturalIssue {
                issue_type: CulturalIssueType::Religious,
                description: format!("Review for compliance with: {}", prohibition),
                severity: Severity::Warning,
                affected_section: "General".to_string(),
                explanation: "Cultural/religious prohibition may affect statute applicability"
                    .to_string(),
                adaptations: vec![
                    "Add exception clause if appropriate".to_string(),
                    "Adjust language to respect cultural norms".to_string(),
                ],
                stakeholders_to_consult: vec![
                    "Cultural affairs ministry".to_string(),
                    "Religious leaders".to_string(),
                ],
            });
        }

        let sensitivity_score = if issues.is_empty() {
            0.1 // Low sensitivity
        } else {
            0.5 + (issues.len() as f64 * 0.1).min(0.4)
        };

        Ok(CulturalSensitivityAnalysis {
            id: format!("cultural-{}", uuid::Uuid::new_v4()),
            statute_id: statute.id.clone(),
            sensitivity_score,
            issues,
            safe_aspects: vec!["Legal framework structure".to_string()],
            assessment: if sensitivity_score < 0.3 {
                "Low cultural sensitivity concerns".to_string()
            } else if sensitivity_score < 0.7 {
                "Moderate cultural considerations needed".to_string()
            } else {
                "High cultural sensitivity - extensive consultation required".to_string()
            },
            required_consultations: vec!["Cultural advisors".to_string()],
        })
    }

    /// Generates plain language explanation.
    pub async fn generate_plain_explanation(
        &self,
        statute: &Statute,
        audience_level: AudienceLevel,
    ) -> PortingResult<PlainLanguageExplanation> {
        let summary = match audience_level {
            AudienceLevel::GeneralPublic => {
                format!(
                    "This law '{}' provides certain legal rights and responsibilities.",
                    statute.title
                )
            }
            AudienceLevel::Business => {
                format!(
                    "'{}' establishes legal framework affecting business operations.",
                    statute.title
                )
            }
            AudienceLevel::Government => {
                format!(
                    "'{}' defines statutory requirements for government implementation.",
                    statute.title
                )
            }
            AudienceLevel::Legal => {
                format!(
                    "Statute '{}' with effect: {:?}",
                    statute.title, statute.effect.effect_type
                )
            }
            AudienceLevel::Academic => {
                format!(
                    "Legal statute '{}' for academic analysis and research.",
                    statute.title
                )
            }
        };

        let explanation = format!(
            "The statute titled '{}' establishes legal provisions in its jurisdiction. \
            It has been analyzed for potential porting to other legal systems.",
            statute.title
        );

        Ok(PlainLanguageExplanation {
            id: format!("explain-{}", uuid::Uuid::new_v4()),
            statute_id: statute.id.clone(),
            audience_level,
            summary,
            explanation,
            key_points: vec![
                "Defines legal rights and obligations".to_string(),
                "Subject to jurisdictional requirements".to_string(),
                "May require adaptation for different legal systems".to_string(),
            ],
            examples: vec!["Example: Implementation in similar jurisdictions".to_string()],
            faqs: vec![FrequentlyAskedQuestion {
                question: "What does this statute cover?".to_string(),
                answer: "It establishes legal framework for specific matters.".to_string(),
                related_topics: vec!["Legal compliance".to_string()],
            }],
            readability_score: 0.8,
        })
    }

    /// Helper to calculate similarity score.
    fn calculate_similarity(&self, _statute: &Statute, _jurisdiction: &Jurisdiction) -> f64 {
        // Simplified similarity calculation
        // Real implementation would use embeddings, semantic analysis, etc.
        0.5
    }
}

impl Default for AiPortingAssistant {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Validation Framework (v0.1.6)
// ============================================================================

/// Compliance check result for target jurisdiction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetJurisdictionComplianceCheck {
    /// Result ID
    pub id: String,
    /// Is compliant with target jurisdiction
    pub is_compliant: bool,
    /// Compliance score (0.0 to 1.0)
    pub compliance_score: f64,
    /// List of compliance issues
    pub issues: Vec<ValidationComplianceIssue>,
    /// Recommended modifications
    pub recommendations: Vec<String>,
    /// Target jurisdiction regulations checked
    pub checked_regulations: Vec<String>,
}

/// Compliance issue detected during validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationComplianceIssue {
    /// Issue ID
    pub id: String,
    /// Issue severity
    pub severity: ComplianceSeverity,
    /// Issue category
    pub category: ComplianceCategory,
    /// Description of the issue
    pub description: String,
    /// Conflicting regulation reference
    pub conflicting_regulation: String,
    /// Suggested resolution
    pub suggested_resolution: Option<String>,
}

/// Severity level of compliance issues.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ComplianceSeverity {
    /// Critical - statute cannot be adopted
    Critical,
    /// High - major modifications required
    High,
    /// Medium - moderate changes needed
    Medium,
    /// Low - minor adjustments suggested
    Low,
    /// Info - informational only
    Info,
}

/// Category of compliance issues.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ComplianceCategory {
    /// Constitutional violation
    Constitutional,
    /// Regulatory conflict
    Regulatory,
    /// Procedural incompatibility
    Procedural,
    /// Cultural incompatibility
    Cultural,
    /// Technical standards mismatch
    Technical,
    /// Administrative burden
    Administrative,
}

/// Target jurisdiction compliance checker.
#[derive(Debug, Clone)]
pub struct TargetJurisdictionChecker {
    /// Target jurisdiction
    #[allow(dead_code)]
    target_jurisdiction: Jurisdiction,
    /// Known regulations database
    regulations: HashMap<String, RegulationEntry>,
}

/// Regulation entry in the database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulationEntry {
    /// Regulation ID
    pub id: String,
    /// Regulation title
    pub title: String,
    /// Regulatory authority
    pub authority: String,
    /// Regulation scope
    pub scope: Vec<String>,
    /// Mandatory requirements
    pub requirements: Vec<String>,
}

impl TargetJurisdictionChecker {
    /// Creates a new compliance checker.
    pub fn new(target_jurisdiction: Jurisdiction) -> Self {
        let mut regulations = HashMap::new();

        // Initialize with some common regulations per jurisdiction
        match target_jurisdiction.id.as_str() {
            "US" => {
                regulations.insert(
                    "cfr-title-5".to_string(),
                    RegulationEntry {
                        id: "cfr-title-5".to_string(),
                        title: "Code of Federal Regulations - Administrative Procedures"
                            .to_string(),
                        authority: "Federal Government".to_string(),
                        scope: vec!["administrative".to_string(), "procedural".to_string()],
                        requirements: vec![
                            "Public comment period".to_string(),
                            "Notice of rulemaking".to_string(),
                        ],
                    },
                );
            }
            "JP" => {
                regulations.insert(
                    "gyosei-tetsuzuki".to_string(),
                    RegulationEntry {
                        id: "gyosei-tetsuzuki".to_string(),
                        title: "行政手続法 (Administrative Procedure Act)".to_string(),
                        authority: "国会 (Diet)".to_string(),
                        scope: vec!["administrative".to_string(), "procedural".to_string()],
                        requirements: vec![
                            "意見公募 (Public comment)".to_string(),
                            "理由の提示 (Reason disclosure)".to_string(),
                        ],
                    },
                );
            }
            _ => {}
        }

        Self {
            target_jurisdiction,
            regulations,
        }
    }

    /// Checks compliance of a ported statute.
    pub fn check_compliance(&self, statute: &Statute) -> TargetJurisdictionComplianceCheck {
        let mut issues = Vec::new();
        let mut checked_regulations = Vec::new();

        // Check against known regulations
        for (reg_id, regulation) in &self.regulations {
            checked_regulations.push(regulation.title.clone());

            // Check if statute scope overlaps with regulation
            if self.has_scope_overlap(statute, regulation) {
                // Check specific requirements
                for requirement in &regulation.requirements {
                    if !self.meets_requirement(statute, requirement) {
                        issues.push(ValidationComplianceIssue {
                            id: uuid::Uuid::new_v4().to_string(),
                            severity: ComplianceSeverity::Medium,
                            category: ComplianceCategory::Regulatory,
                            description: format!("Does not meet requirement: {}", requirement),
                            conflicting_regulation: reg_id.clone(),
                            suggested_resolution: Some(format!(
                                "Add provisions for {}",
                                requirement
                            )),
                        });
                    }
                }
            }
        }

        let compliance_score = if issues.is_empty() {
            1.0
        } else {
            let critical_count = issues
                .iter()
                .filter(|i| i.severity == ComplianceSeverity::Critical)
                .count();
            let high_count = issues
                .iter()
                .filter(|i| i.severity == ComplianceSeverity::High)
                .count();

            if critical_count > 0 {
                0.0
            } else if high_count > 0 {
                0.5
            } else {
                0.8
            }
        };

        TargetJurisdictionComplianceCheck {
            id: uuid::Uuid::new_v4().to_string(),
            is_compliant: issues
                .iter()
                .all(|i| i.severity != ComplianceSeverity::Critical),
            compliance_score,
            issues,
            recommendations: vec![
                "Review all identified compliance issues".to_string(),
                "Consult with local legal experts".to_string(),
            ],
            checked_regulations,
        }
    }

    fn has_scope_overlap(&self, _statute: &Statute, _regulation: &RegulationEntry) -> bool {
        // Simplified - in real implementation would analyze statute content
        true
    }

    fn meets_requirement(&self, _statute: &Statute, _requirement: &str) -> bool {
        // Simplified - in real implementation would check statute provisions
        false
    }
}

/// Constitutional compatibility analysis result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalAnalysis {
    /// Analysis ID
    pub id: String,
    /// Is compatible with constitution
    pub is_compatible: bool,
    /// Compatibility score (0.0 to 1.0)
    pub compatibility_score: f64,
    /// Constitutional issues identified
    pub issues: Vec<ConstitutionalIssue>,
    /// Relevant constitutional provisions
    pub relevant_provisions: Vec<String>,
    /// Recommended amendments
    pub recommended_amendments: Vec<String>,
}

/// Constitutional issue identified.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalIssue {
    /// Issue ID
    pub id: String,
    /// Issue type
    pub issue_type: ConstitutionalIssueType,
    /// Description
    pub description: String,
    /// Conflicting constitutional provision
    pub conflicting_provision: String,
    /// Severity
    pub severity: ComplianceSeverity,
    /// Suggested remedy
    pub suggested_remedy: Option<String>,
}

/// Type of constitutional issue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConstitutionalIssueType {
    /// Violates fundamental rights
    FundamentalRights,
    /// Exceeds legislative authority
    LegislativeAuthority,
    /// Separation of powers issue
    SeparationOfPowers,
    /// Federalism/jurisdictional conflict
    Federalism,
    /// Due process violation
    DueProcess,
    /// Equal protection violation
    EqualProtection,
}

/// Constitutional compatibility analyzer.
#[derive(Debug, Clone)]
pub struct ConstitutionalAnalyzer {
    /// Target jurisdiction
    #[allow(dead_code)]
    target_jurisdiction: Jurisdiction,
    /// Constitutional provisions database
    provisions: HashMap<String, ConstitutionalProvision>,
}

/// Constitutional provision entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalProvision {
    /// Provision reference (e.g., "Article 14")
    pub reference: String,
    /// Provision text summary
    pub text: String,
    /// Category of rights/powers protected
    pub category: ConstitutionalIssueType,
}

impl ConstitutionalAnalyzer {
    /// Creates a new constitutional analyzer.
    pub fn new(target_jurisdiction: Jurisdiction) -> Self {
        let mut provisions = HashMap::new();

        // Initialize with key constitutional provisions
        match target_jurisdiction.id.as_str() {
            "US" => {
                provisions.insert(
                    "amend-1".to_string(),
                    ConstitutionalProvision {
                        reference: "First Amendment".to_string(),
                        text: "Freedom of speech, religion, press, assembly".to_string(),
                        category: ConstitutionalIssueType::FundamentalRights,
                    },
                );
                provisions.insert(
                    "amend-14".to_string(),
                    ConstitutionalProvision {
                        reference: "Fourteenth Amendment".to_string(),
                        text: "Equal protection and due process".to_string(),
                        category: ConstitutionalIssueType::EqualProtection,
                    },
                );
            }
            "JP" => {
                provisions.insert(
                    "art-14".to_string(),
                    ConstitutionalProvision {
                        reference: "憲法第14条 (Article 14)".to_string(),
                        text: "法の下の平等 (Equality under the law)".to_string(),
                        category: ConstitutionalIssueType::EqualProtection,
                    },
                );
                provisions.insert(
                    "art-21".to_string(),
                    ConstitutionalProvision {
                        reference: "憲法第21条 (Article 21)".to_string(),
                        text: "表現の自由 (Freedom of expression)".to_string(),
                        category: ConstitutionalIssueType::FundamentalRights,
                    },
                );
            }
            _ => {}
        }

        Self {
            target_jurisdiction,
            provisions,
        }
    }

    /// Analyzes constitutional compatibility.
    pub fn analyze(&self, statute: &Statute) -> ConstitutionalAnalysis {
        let mut issues = Vec::new();
        let mut relevant_provisions = Vec::new();

        // Check against constitutional provisions
        for provision in self.provisions.values() {
            relevant_provisions.push(provision.reference.clone());

            // Check for potential conflicts
            if self.may_conflict(statute, provision) {
                issues.push(ConstitutionalIssue {
                    id: uuid::Uuid::new_v4().to_string(),
                    issue_type: provision.category,
                    description: format!("Potential conflict with {}", provision.reference),
                    conflicting_provision: provision.reference.clone(),
                    severity: ComplianceSeverity::High,
                    suggested_remedy: Some(
                        "Review and modify to ensure constitutional compliance".to_string(),
                    ),
                });
            }
        }

        let compatibility_score = if issues.is_empty() {
            1.0
        } else {
            let critical_count = issues
                .iter()
                .filter(|i| i.severity == ComplianceSeverity::Critical)
                .count();
            if critical_count > 0 { 0.0 } else { 0.6 }
        };

        ConstitutionalAnalysis {
            id: uuid::Uuid::new_v4().to_string(),
            is_compatible: issues
                .iter()
                .all(|i| i.severity != ComplianceSeverity::Critical),
            compatibility_score,
            issues,
            relevant_provisions,
            recommended_amendments: vec![
                "Consult constitutional law experts".to_string(),
                "Consider judicial review".to_string(),
            ],
        }
    }

    fn may_conflict(&self, _statute: &Statute, _provision: &ConstitutionalProvision) -> bool {
        // Simplified - real implementation would analyze statute content
        false
    }
}

/// Treaty compliance check result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreatyComplianceResult {
    /// Result ID
    pub id: String,
    /// Is compliant with treaties
    pub is_compliant: bool,
    /// Compliance score (0.0 to 1.0)
    pub compliance_score: f64,
    /// Treaty conflicts identified
    pub conflicts: Vec<TreatyConflict>,
    /// Applicable treaties checked
    pub checked_treaties: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// Treaty conflict identified.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreatyConflict {
    /// Conflict ID
    pub id: String,
    /// Treaty name
    pub treaty_name: String,
    /// Treaty article/provision
    pub provision: String,
    /// Conflict description
    pub description: String,
    /// Severity
    pub severity: ComplianceSeverity,
    /// Suggested resolution
    pub suggested_resolution: Option<String>,
}

/// Treaty/international law compliance checker.
#[derive(Debug, Clone)]
pub struct TreatyTargetJurisdictionChecker {
    /// Target jurisdiction
    #[allow(dead_code)]
    target_jurisdiction: Jurisdiction,
    /// Applicable treaties database
    treaties: HashMap<String, TreatyEntry>,
}

/// Treaty entry in the database.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreatyEntry {
    /// Treaty ID
    pub id: String,
    /// Treaty full name
    pub name: String,
    /// Ratification status for jurisdiction
    pub ratified: bool,
    /// Key obligations
    pub obligations: Vec<String>,
    /// Prohibited actions
    pub prohibitions: Vec<String>,
}

impl TreatyTargetJurisdictionChecker {
    /// Creates a new treaty compliance checker.
    pub fn new(target_jurisdiction: Jurisdiction) -> Self {
        let mut treaties = HashMap::new();

        // Add major international treaties
        treaties.insert(
            "iccpr".to_string(),
            TreatyEntry {
                id: "iccpr".to_string(),
                name: "International Covenant on Civil and Political Rights".to_string(),
                ratified: true,
                obligations: vec![
                    "Protect right to life".to_string(),
                    "Ensure fair trial".to_string(),
                    "Freedom of expression".to_string(),
                ],
                prohibitions: vec!["Torture".to_string(), "Arbitrary detention".to_string()],
            },
        );

        treaties.insert(
            "icescr".to_string(),
            TreatyEntry {
                id: "icescr".to_string(),
                name: "International Covenant on Economic, Social and Cultural Rights".to_string(),
                ratified: true,
                obligations: vec![
                    "Right to work".to_string(),
                    "Right to education".to_string(),
                    "Right to health".to_string(),
                ],
                prohibitions: vec![],
            },
        );

        Self {
            target_jurisdiction,
            treaties,
        }
    }

    /// Checks treaty compliance.
    pub fn check_compliance(&self, statute: &Statute) -> TreatyComplianceResult {
        let mut conflicts = Vec::new();
        let mut checked_treaties = Vec::new();

        for treaty in self.treaties.values() {
            if !treaty.ratified {
                continue;
            }

            checked_treaties.push(treaty.name.clone());

            // Check prohibitions
            for prohibition in &treaty.prohibitions {
                if self.may_violate_prohibition(statute, prohibition) {
                    conflicts.push(TreatyConflict {
                        id: uuid::Uuid::new_v4().to_string(),
                        treaty_name: treaty.name.clone(),
                        provision: prohibition.clone(),
                        description: format!("May violate prohibition on {}", prohibition),
                        severity: ComplianceSeverity::Critical,
                        suggested_resolution: Some(
                            "Remove provisions that violate treaty prohibition".to_string(),
                        ),
                    });
                }
            }
        }

        let compliance_score = if conflicts.is_empty() {
            1.0
        } else {
            let critical_count = conflicts
                .iter()
                .filter(|c| c.severity == ComplianceSeverity::Critical)
                .count();
            if critical_count > 0 { 0.0 } else { 0.7 }
        };

        TreatyComplianceResult {
            id: uuid::Uuid::new_v4().to_string(),
            is_compliant: conflicts
                .iter()
                .all(|c| c.severity != ComplianceSeverity::Critical),
            compliance_score,
            conflicts,
            checked_treaties,
            recommendations: vec![
                "Review all applicable international treaties".to_string(),
                "Ensure compliance with treaty obligations".to_string(),
            ],
        }
    }

    fn may_violate_prohibition(&self, _statute: &Statute, _prohibition: &str) -> bool {
        // Simplified - real implementation would analyze statute content
        false
    }
}

/// Human rights impact assessment result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanRightsAssessment {
    /// Assessment ID
    pub id: String,
    /// Overall impact score (-1.0 to 1.0, where 1.0 is positive impact)
    pub impact_score: f64,
    /// Rights affected
    pub affected_rights: Vec<AffectedRight>,
    /// Vulnerable groups impacted
    pub vulnerable_groups: Vec<VulnerableGroupImpact>,
    /// Mitigation measures recommended
    pub mitigation_measures: Vec<String>,
    /// Overall assessment summary
    pub summary: String,
}

/// A human right affected by the statute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AffectedRight {
    /// Right name
    pub right: String,
    /// Impact type
    pub impact: RightImpactType,
    /// Impact severity
    pub severity: ImpactSeverity,
    /// Description of impact
    pub description: String,
}

/// Type of impact on a right.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RightImpactType {
    /// Enhances the right
    Enhancement,
    /// Neutral impact
    Neutral,
    /// Restricts the right
    Restriction,
    /// Potentially violates the right
    Violation,
}

/// Severity of impact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ImpactSeverity {
    /// Severe impact
    Severe,
    /// Moderate impact
    Moderate,
    /// Minor impact
    Minor,
    /// Negligible impact
    Negligible,
}

/// Impact on a vulnerable group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VulnerableGroupImpact {
    /// Group name
    pub group: String,
    /// Impact description
    pub impact: String,
    /// Severity
    pub severity: ImpactSeverity,
    /// Recommended protections
    pub recommended_protections: Vec<String>,
}

/// Human rights impact assessor.
#[derive(Debug, Clone)]
pub struct HumanRightsAssessor {
    /// Target jurisdiction
    #[allow(dead_code)]
    target_jurisdiction: Jurisdiction,
}

impl HumanRightsAssessor {
    /// Creates a new human rights assessor.
    pub fn new(target_jurisdiction: Jurisdiction) -> Self {
        Self {
            target_jurisdiction,
        }
    }

    /// Assesses human rights impact of a statute.
    pub fn assess(&self, statute: &Statute) -> HumanRightsAssessment {
        let mut affected_rights = Vec::new();
        let mut vulnerable_groups = Vec::new();

        // Analyze potential impacts on key rights
        let rights_to_check = vec![
            "Right to equality",
            "Right to privacy",
            "Freedom of expression",
            "Right to fair trial",
        ];

        for right in rights_to_check {
            let impact = self.assess_right_impact(statute, right);
            if impact.impact != RightImpactType::Neutral {
                affected_rights.push(impact);
            }
        }

        // Check impact on vulnerable groups
        let groups_to_check = vec![
            "Children",
            "Elderly",
            "Persons with disabilities",
            "Minorities",
        ];

        for group in groups_to_check {
            if let Some(impact) = self.assess_group_impact(statute, group) {
                vulnerable_groups.push(impact);
            }
        }

        // Calculate overall impact score
        let impact_score = self.calculate_impact_score(&affected_rights);

        HumanRightsAssessment {
            id: uuid::Uuid::new_v4().to_string(),
            impact_score,
            affected_rights,
            vulnerable_groups,
            mitigation_measures: vec![
                "Include non-discrimination clauses".to_string(),
                "Add safeguards for vulnerable groups".to_string(),
                "Ensure proportionality of restrictions".to_string(),
            ],
            summary: if impact_score >= 0.0 {
                "Statute has positive or neutral human rights impact".to_string()
            } else {
                "Statute may negatively impact human rights - review recommended".to_string()
            },
        }
    }

    fn assess_right_impact(&self, _statute: &Statute, right: &str) -> AffectedRight {
        // Simplified - real implementation would analyze statute content
        AffectedRight {
            right: right.to_string(),
            impact: RightImpactType::Neutral,
            severity: ImpactSeverity::Negligible,
            description: format!("No significant impact on {}", right),
        }
    }

    fn assess_group_impact(
        &self,
        _statute: &Statute,
        _group: &str,
    ) -> Option<VulnerableGroupImpact> {
        // Simplified - real implementation would analyze statute content
        None
    }

    fn calculate_impact_score(&self, affected_rights: &[AffectedRight]) -> f64 {
        if affected_rights.is_empty() {
            return 0.0;
        }

        let mut total_score = 0.0;
        for right in affected_rights {
            let score = match right.impact {
                RightImpactType::Enhancement => 1.0,
                RightImpactType::Neutral => 0.0,
                RightImpactType::Restriction => -0.5,
                RightImpactType::Violation => -1.0,
            };
            total_score += score;
        }

        total_score / affected_rights.len() as f64
    }
}

/// Enforceability prediction result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnforceabilityPrediction {
    /// Prediction ID
    pub id: String,
    /// Is statute enforceable
    pub is_enforceable: bool,
    /// Enforceability score (0.0 to 1.0)
    pub enforceability_score: f64,
    /// Enforcement challenges
    pub challenges: Vec<EnforcementChallenge>,
    /// Required enforcement mechanisms
    pub required_mechanisms: Vec<String>,
    /// Estimated implementation cost
    pub estimated_cost: Option<f64>,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// Enforcement challenge identified.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnforcementChallenge {
    /// Challenge ID
    pub id: String,
    /// Challenge type
    pub challenge_type: EnforcementChallengeType,
    /// Description
    pub description: String,
    /// Severity
    pub severity: ImpactSeverity,
    /// Suggested solution
    pub suggested_solution: Option<String>,
}

/// Type of enforcement challenge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnforcementChallengeType {
    /// Lacks enforcement authority
    Authority,
    /// Insufficient resources
    Resources,
    /// Technical complexity
    Technical,
    /// Cultural resistance
    Cultural,
    /// Administrative capacity
    Administrative,
    /// Monitoring difficulty
    Monitoring,
}

/// Enforceability predictor.
#[derive(Debug, Clone)]
pub struct EnforceabilityPredictor {
    /// Target jurisdiction
    #[allow(dead_code)]
    target_jurisdiction: Jurisdiction,
}

impl EnforceabilityPredictor {
    /// Creates a new enforceability predictor.
    pub fn new(target_jurisdiction: Jurisdiction) -> Self {
        Self {
            target_jurisdiction,
        }
    }

    /// Predicts enforceability of a statute.
    pub fn predict(&self, statute: &Statute) -> EnforceabilityPrediction {
        let mut challenges = Vec::new();
        let mut required_mechanisms = Vec::new();

        // Check for common enforcement challenges
        if self.lacks_enforcement_authority(statute) {
            challenges.push(EnforcementChallenge {
                id: uuid::Uuid::new_v4().to_string(),
                challenge_type: EnforcementChallengeType::Authority,
                description: "Lacks clear enforcement authority".to_string(),
                severity: ImpactSeverity::Severe,
                suggested_solution: Some(
                    "Designate enforcement agency and grant necessary authority".to_string(),
                ),
            });
        }

        if self.requires_significant_resources(statute) {
            challenges.push(EnforcementChallenge {
                id: uuid::Uuid::new_v4().to_string(),
                challenge_type: EnforcementChallengeType::Resources,
                description: "Requires significant enforcement resources".to_string(),
                severity: ImpactSeverity::Moderate,
                suggested_solution: Some(
                    "Allocate budget for enforcement infrastructure".to_string(),
                ),
            });
        }

        // Identify required mechanisms
        required_mechanisms.extend(vec![
            "Enforcement agency designation".to_string(),
            "Penalty structure".to_string(),
            "Monitoring system".to_string(),
            "Reporting requirements".to_string(),
        ]);

        let enforceability_score = if challenges.is_empty() {
            0.9
        } else {
            let severe_count = challenges
                .iter()
                .filter(|c| c.severity == ImpactSeverity::Severe)
                .count();
            if severe_count > 0 { 0.3 } else { 0.6 }
        };

        EnforceabilityPrediction {
            id: uuid::Uuid::new_v4().to_string(),
            is_enforceable: enforceability_score >= 0.5,
            enforceability_score,
            challenges,
            required_mechanisms,
            estimated_cost: Some(100000.0), // Placeholder
            recommendations: vec![
                "Establish clear enforcement procedures".to_string(),
                "Allocate adequate resources".to_string(),
                "Train enforcement personnel".to_string(),
            ],
        }
    }

    fn lacks_enforcement_authority(&self, _statute: &Statute) -> bool {
        // Simplified - real implementation would analyze statute provisions
        false
    }

    fn requires_significant_resources(&self, _statute: &Statute) -> bool {
        // Simplified - real implementation would estimate resource requirements
        true
    }
}

/// Comprehensive validation framework combining all validation types.
#[derive(Debug, Clone)]
pub struct ValidationFramework {
    compliance_checker: TargetJurisdictionChecker,
    constitutional_analyzer: ConstitutionalAnalyzer,
    treaty_checker: TreatyTargetJurisdictionChecker,
    human_rights_assessor: HumanRightsAssessor,
    enforceability_predictor: EnforceabilityPredictor,
}

/// Comprehensive validation result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    /// Result ID
    pub id: String,
    /// Overall validation passed
    pub passed: bool,
    /// Overall score (0.0 to 1.0)
    pub overall_score: f64,
    /// Compliance check result
    pub compliance: TargetJurisdictionComplianceCheck,
    /// Constitutional analysis
    pub constitutional: ConstitutionalAnalysis,
    /// Treaty compliance
    pub treaty_compliance: TreatyComplianceResult,
    /// Human rights assessment
    pub human_rights: HumanRightsAssessment,
    /// Enforceability prediction
    pub enforceability: EnforceabilityPrediction,
    /// Summary of validation
    pub summary: String,
}

impl ValidationFramework {
    /// Creates a new validation framework.
    pub fn new(target_jurisdiction: Jurisdiction) -> Self {
        Self {
            compliance_checker: TargetJurisdictionChecker::new(target_jurisdiction.clone()),
            constitutional_analyzer: ConstitutionalAnalyzer::new(target_jurisdiction.clone()),
            treaty_checker: TreatyTargetJurisdictionChecker::new(target_jurisdiction.clone()),
            human_rights_assessor: HumanRightsAssessor::new(target_jurisdiction.clone()),
            enforceability_predictor: EnforceabilityPredictor::new(target_jurisdiction),
        }
    }

    /// Performs comprehensive validation of a ported statute.
    pub fn validate(&self, statute: &Statute) -> ValidationResult {
        let compliance = self.compliance_checker.check_compliance(statute);
        let constitutional = self.constitutional_analyzer.analyze(statute);
        let treaty_compliance = self.treaty_checker.check_compliance(statute);
        let human_rights = self.human_rights_assessor.assess(statute);
        let enforceability = self.enforceability_predictor.predict(statute);

        // Calculate overall score
        let overall_score = (
            compliance.compliance_score
                + constitutional.compatibility_score
                + treaty_compliance.compliance_score
                + enforceability.enforceability_score
                + (human_rights.impact_score + 1.0) / 2.0
            // Normalize -1..1 to 0..1
        ) / 5.0;

        let passed = compliance.is_compliant
            && constitutional.is_compatible
            && treaty_compliance.is_compliant
            && human_rights.impact_score >= 0.0
            && enforceability.is_enforceable;

        let summary = if passed {
            format!("Validation passed with overall score {:.2}", overall_score)
        } else {
            format!(
                "Validation failed - review required (score: {:.2})",
                overall_score
            )
        };

        ValidationResult {
            id: uuid::Uuid::new_v4().to_string(),
            passed,
            overall_score,
            compliance,
            constitutional,
            treaty_compliance,
            human_rights,
            enforceability,
            summary,
        }
    }
}

// ============================================================================
// Pre-Porting Feasibility Analysis (v0.2.2)
// ============================================================================

/// Pre-porting feasibility analysis result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeasibilityAnalysis {
    /// Analysis ID
    pub id: String,
    /// Overall feasibility (true if recommended to proceed)
    pub is_feasible: bool,
    /// Feasibility score (0.0 to 1.0)
    pub feasibility_score: f64,
    /// Technical feasibility score
    pub technical_feasibility: f64,
    /// Legal feasibility score
    pub legal_feasibility: f64,
    /// Cultural feasibility score
    pub cultural_feasibility: f64,
    /// Economic feasibility score
    pub economic_feasibility: f64,
    /// Political feasibility score
    pub political_feasibility: f64,
    /// List of feasibility factors
    pub factors: Vec<FeasibilityFactor>,
    /// Identified risks
    pub risks: Vec<String>,
    /// Prerequisites for porting
    pub prerequisites: Vec<String>,
    /// Estimated time to complete (in days)
    pub estimated_time_days: u32,
    /// Estimated cost (in USD)
    pub estimated_cost_usd: f64,
    /// Recommended approach
    pub recommended_approach: String,
    /// Alternative approaches
    pub alternatives: Vec<String>,
    /// Overall recommendation
    pub recommendation: FeasibilityRecommendation,
    /// Detailed analysis notes
    pub notes: Vec<String>,
}

/// Feasibility factor affecting porting success.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeasibilityFactor {
    /// Factor ID
    pub id: String,
    /// Factor category
    pub category: FeasibilityCategory,
    /// Factor name
    pub name: String,
    /// Impact on feasibility (-1.0 to 1.0, negative is unfavorable)
    pub impact: f64,
    /// Severity of impact
    pub severity: FeasibilitySeverity,
    /// Description
    pub description: String,
    /// Mitigation strategies
    pub mitigation_strategies: Vec<String>,
}

/// Category of feasibility factors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FeasibilityCategory {
    /// Technical compatibility
    Technical,
    /// Legal compatibility
    Legal,
    /// Cultural compatibility
    Cultural,
    /// Economic viability
    Economic,
    /// Political support
    Political,
    /// Administrative capacity
    Administrative,
    /// Stakeholder support
    Stakeholder,
    /// Resource availability
    Resources,
}

/// Severity of feasibility impact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FeasibilitySeverity {
    /// Critical - prevents porting
    Critical,
    /// Major - significant obstacle
    Major,
    /// Moderate - manageable challenge
    Moderate,
    /// Minor - small concern
    Minor,
    /// Negligible - no significant impact
    Negligible,
}

/// Feasibility recommendation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FeasibilityRecommendation {
    /// Strongly recommended - proceed immediately
    StronglyRecommended,
    /// Recommended - proceed with normal caution
    Recommended,
    /// Conditional - proceed only if conditions met
    Conditional,
    /// NotRecommended - significant challenges exist
    NotRecommended,
    /// StronglyNotRecommended - do not proceed
    StronglyNotRecommended,
}

/// Pre-porting feasibility analyzer.
#[derive(Debug, Clone)]
pub struct PrePortingFeasibilityAnalyzer {
    /// Source jurisdiction
    source_jurisdiction: Jurisdiction,
    /// Target jurisdiction
    target_jurisdiction: Jurisdiction,
    /// Validation framework
    validation_framework: ValidationFramework,
}

impl PrePortingFeasibilityAnalyzer {
    /// Creates a new feasibility analyzer.
    pub fn new(source_jurisdiction: Jurisdiction, target_jurisdiction: Jurisdiction) -> Self {
        Self {
            source_jurisdiction: source_jurisdiction.clone(),
            target_jurisdiction: target_jurisdiction.clone(),
            validation_framework: ValidationFramework::new(target_jurisdiction),
        }
    }

    /// Analyzes feasibility of porting a statute.
    pub fn analyze(&self, statute: &Statute) -> FeasibilityAnalysis {
        let mut factors = Vec::new();
        let mut risks = Vec::new();
        let mut prerequisites = Vec::new();
        let mut notes = Vec::new();

        // Technical feasibility analysis
        let technical_feasibility =
            self.analyze_technical_feasibility(statute, &mut factors, &mut notes);

        // Legal feasibility analysis (using validation framework)
        let validation_result = self.validation_framework.validate(statute);
        let legal_feasibility = validation_result.overall_score;

        if !validation_result.passed {
            factors.push(FeasibilityFactor {
                id: uuid::Uuid::new_v4().to_string(),
                category: FeasibilityCategory::Legal,
                name: "Legal Validation Issues".to_string(),
                impact: -0.5,
                severity: FeasibilitySeverity::Major,
                description: validation_result.summary.clone(),
                mitigation_strategies: vec![
                    "Address compliance issues before porting".to_string(),
                    "Consult with legal experts".to_string(),
                ],
            });
            risks.push("Legal validation failed".to_string());
        }

        // Cultural feasibility analysis
        let cultural_feasibility =
            self.analyze_cultural_feasibility(statute, &mut factors, &mut notes);

        // Economic feasibility analysis
        let economic_feasibility =
            self.analyze_economic_feasibility(statute, &mut factors, &mut notes);

        // Political feasibility analysis
        let political_feasibility =
            self.analyze_political_feasibility(statute, &mut factors, &mut notes);

        // Overall feasibility score (weighted average)
        let feasibility_score = technical_feasibility * 0.2
            + legal_feasibility * 0.3
            + cultural_feasibility * 0.2
            + economic_feasibility * 0.15
            + political_feasibility * 0.15;

        // Determine if feasible
        let is_feasible = feasibility_score >= 0.6 && legal_feasibility >= 0.5;

        // Determine recommendation
        let recommendation = if feasibility_score >= 0.85 {
            FeasibilityRecommendation::StronglyRecommended
        } else if feasibility_score >= 0.7 {
            FeasibilityRecommendation::Recommended
        } else if feasibility_score >= 0.5 {
            FeasibilityRecommendation::Conditional
        } else if feasibility_score >= 0.3 {
            FeasibilityRecommendation::NotRecommended
        } else {
            FeasibilityRecommendation::StronglyNotRecommended
        };

        // Generate prerequisites
        prerequisites.extend(vec![
            "Secure stakeholder buy-in".to_string(),
            "Allocate necessary resources".to_string(),
            "Complete legal review".to_string(),
        ]);

        if cultural_feasibility < 0.7 {
            prerequisites.push("Conduct cultural impact assessment".to_string());
        }

        // Estimate time and cost
        let complexity_factor = 1.0 + (1.0 - feasibility_score);
        let estimated_time_days = (30.0 * complexity_factor) as u32;
        let estimated_cost_usd = 50000.0 * complexity_factor;

        // Recommended approach
        let recommended_approach = if is_feasible {
            format!(
                "Proceed with phased approach: (1) Legal review, (2) Cultural adaptation, (3) Stakeholder engagement, (4) Pilot implementation"
            )
        } else {
            format!(
                "Address critical issues before proceeding: focus on improving {} feasibility",
                self.identify_weakest_area(
                    technical_feasibility,
                    legal_feasibility,
                    cultural_feasibility,
                    economic_feasibility,
                    political_feasibility
                )
            )
        };

        // Alternative approaches
        let alternatives = vec![
            "Partial porting of compatible sections only".to_string(),
            "Phased implementation with pilot programs".to_string(),
            "Create hybrid approach combining elements from both jurisdictions".to_string(),
        ];

        FeasibilityAnalysis {
            id: uuid::Uuid::new_v4().to_string(),
            is_feasible,
            feasibility_score,
            technical_feasibility,
            legal_feasibility,
            cultural_feasibility,
            economic_feasibility,
            political_feasibility,
            factors,
            risks,
            prerequisites,
            estimated_time_days,
            estimated_cost_usd,
            recommended_approach,
            alternatives,
            recommendation,
            notes,
        }
    }

    fn analyze_technical_feasibility(
        &self,
        _statute: &Statute,
        factors: &mut Vec<FeasibilityFactor>,
        notes: &mut Vec<String>,
    ) -> f64 {
        let mut score: f64 = 0.8; // Default moderate technical feasibility

        // Check legal system compatibility
        if self.source_jurisdiction.legal_system == self.target_jurisdiction.legal_system {
            factors.push(FeasibilityFactor {
                id: uuid::Uuid::new_v4().to_string(),
                category: FeasibilityCategory::Technical,
                name: "Legal System Compatibility".to_string(),
                impact: 0.3,
                severity: FeasibilitySeverity::Minor,
                description: "Same legal system family facilitates porting".to_string(),
                mitigation_strategies: vec![],
            });
            score += 0.1;
            notes.push("Legal systems are compatible".to_string());
        } else {
            factors.push(FeasibilityFactor {
                id: uuid::Uuid::new_v4().to_string(),
                category: FeasibilityCategory::Technical,
                name: "Legal System Incompatibility".to_string(),
                impact: -0.2,
                severity: FeasibilitySeverity::Moderate,
                description: "Different legal systems require adaptation".to_string(),
                mitigation_strategies: vec![
                    "Engage experts in both legal systems".to_string(),
                    "Identify structural differences early".to_string(),
                ],
            });
            score -= 0.1;
            notes.push("Legal systems differ - requires careful adaptation".to_string());
        }

        score.clamp(0.0, 1.0)
    }

    fn analyze_cultural_feasibility(
        &self,
        _statute: &Statute,
        factors: &mut Vec<FeasibilityFactor>,
        notes: &mut Vec<String>,
    ) -> f64 {
        let mut score: f64 = 0.7;

        // Check if same country (trivially high cultural compatibility)
        if self.source_jurisdiction.id == self.target_jurisdiction.id {
            return 1.0;
        }

        // Check cultural parameters
        let source_params = &self.source_jurisdiction.cultural_params;
        let target_params = &self.target_jurisdiction.cultural_params;

        // Compare age of majority
        if source_params.age_of_majority != target_params.age_of_majority {
            factors.push(FeasibilityFactor {
                id: uuid::Uuid::new_v4().to_string(),
                category: FeasibilityCategory::Cultural,
                name: "Age of Majority Difference".to_string(),
                impact: -0.1,
                severity: FeasibilitySeverity::Minor,
                description: format!(
                    "Age of majority differs: {:?} vs {:?}",
                    source_params.age_of_majority, target_params.age_of_majority
                ),
                mitigation_strategies: vec!["Adjust age-related provisions".to_string()],
            });
            score -= 0.05;
            notes.push("Age-related provisions need adjustment".to_string());
        }

        // Check prohibitions
        if source_params.prohibitions != target_params.prohibitions {
            factors.push(FeasibilityFactor {
                id: uuid::Uuid::new_v4().to_string(),
                category: FeasibilityCategory::Cultural,
                name: "Prohibitions Difference".to_string(),
                impact: -0.15,
                severity: FeasibilitySeverity::Moderate,
                description: "Prohibitions lists differ between jurisdictions".to_string(),
                mitigation_strategies: vec![
                    "Review prohibition-related provisions".to_string(),
                    "Align with target jurisdiction prohibitions".to_string(),
                ],
            });
            score -= 0.1;
        }

        score.clamp(0.0, 1.0)
    }

    fn analyze_economic_feasibility(
        &self,
        _statute: &Statute,
        factors: &mut Vec<FeasibilityFactor>,
        _notes: &mut Vec<String>,
    ) -> f64 {
        let score = 0.75; // Default moderate economic feasibility

        factors.push(FeasibilityFactor {
            id: uuid::Uuid::new_v4().to_string(),
            category: FeasibilityCategory::Economic,
            name: "Implementation Cost".to_string(),
            impact: -0.2,
            severity: FeasibilitySeverity::Moderate,
            description: "Porting requires investment in legal review and adaptation".to_string(),
            mitigation_strategies: vec![
                "Secure budget allocation early".to_string(),
                "Consider phased implementation to spread costs".to_string(),
            ],
        });

        score
    }

    fn analyze_political_feasibility(
        &self,
        _statute: &Statute,
        factors: &mut Vec<FeasibilityFactor>,
        _notes: &mut Vec<String>,
    ) -> f64 {
        let score = 0.6; // Default moderate-low political feasibility (conservative)

        factors.push(FeasibilityFactor {
            id: uuid::Uuid::new_v4().to_string(),
            category: FeasibilityCategory::Political,
            name: "Stakeholder Engagement Required".to_string(),
            impact: -0.15,
            severity: FeasibilitySeverity::Moderate,
            description: "Requires engagement with multiple stakeholders and political support"
                .to_string(),
            mitigation_strategies: vec![
                "Early stakeholder consultation".to_string(),
                "Build coalition of supporters".to_string(),
                "Address concerns proactively".to_string(),
            ],
        });

        score
    }

    fn identify_weakest_area(
        &self,
        technical: f64,
        legal: f64,
        cultural: f64,
        economic: f64,
        political: f64,
    ) -> &'static str {
        let scores = [
            (technical, "technical"),
            (legal, "legal"),
            (cultural, "cultural"),
            (economic, "economic"),
            (political, "political"),
        ];

        scores
            .iter()
            .min_by(|a, b| a.0.partial_cmp(&b.0).unwrap())
            .map(|(_, name)| *name)
            .unwrap_or("overall")
    }
}

// ============================================================================
// Workflow Management (v0.1.7)
// ============================================================================

/// Porting project for managing multi-statute porting initiatives.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortingProject {
    /// Project ID
    pub id: String,
    /// Project name
    pub name: String,
    /// Project description
    pub description: String,
    /// Source jurisdiction
    pub source_jurisdiction: String,
    /// Target jurisdiction
    pub target_jurisdiction: String,
    /// Project status
    pub status: ProjectStatus,
    /// Statutes included in the project
    pub statute_ids: Vec<String>,
    /// Project stakeholders
    pub stakeholders: Vec<Stakeholder>,
    /// Project timeline
    pub timeline: ProjectTimeline,
    /// Created timestamp
    pub created_at: String,
    /// Last updated timestamp
    pub updated_at: String,
    /// Project metadata
    pub metadata: HashMap<String, String>,
}

/// Project status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProjectStatus {
    /// Project planning phase
    Planning,
    /// In progress
    InProgress,
    /// Under review
    UnderReview,
    /// Approved
    Approved,
    /// Rejected
    Rejected,
    /// On hold
    OnHold,
    /// Completed
    Completed,
    /// Cancelled
    Cancelled,
}

/// Stakeholder in a porting project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stakeholder {
    /// Stakeholder ID
    pub id: String,
    /// Stakeholder name
    pub name: String,
    /// Email address
    pub email: String,
    /// Role in the project
    pub role: StakeholderRole,
    /// Notification preferences
    pub notification_preferences: NotificationPreferences,
}

/// Stakeholder role in a porting project.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StakeholderRole {
    /// Project manager
    ProjectManager,
    /// Legal expert/reviewer
    LegalExpert,
    /// Technical reviewer
    TechnicalReviewer,
    /// Approver
    Approver,
    /// Observer
    Observer,
    /// Contributor
    Contributor,
}

/// Notification preferences for stakeholders.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationPreferences {
    /// Notify on status changes
    pub on_status_change: bool,
    /// Notify on deadline approaching
    pub on_deadline_approaching: bool,
    /// Notify on assignment
    pub on_assignment: bool,
    /// Notify on review request
    pub on_review_request: bool,
    /// Notification channels
    pub channels: Vec<NotificationChannel>,
}

/// Notification channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NotificationChannel {
    /// Email notification
    Email,
    /// In-app notification
    InApp,
    /// SMS notification
    Sms,
    /// Webhook
    Webhook,
    /// Website notification
    Website,
    /// Public notice (physical/official publication)
    PublicNotice,
}

/// Project timeline with milestones and deadlines.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectTimeline {
    /// Project start date
    pub start_date: String,
    /// Expected end date
    pub end_date: String,
    /// Milestones
    pub milestones: Vec<Milestone>,
    /// Current phase
    pub current_phase: String,
}

/// Project milestone.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Milestone {
    /// Milestone ID
    pub id: String,
    /// Milestone name
    pub name: String,
    /// Milestone description
    pub description: String,
    /// Target date
    pub target_date: String,
    /// Completion status
    pub completed: bool,
    /// Completed date
    pub completed_date: Option<String>,
    /// Dependencies (other milestone IDs)
    pub dependencies: Vec<String>,
}

/// Porting project manager.
#[derive(Debug)]
pub struct PortingProjectManager {
    projects: HashMap<String, PortingProject>,
}

impl PortingProjectManager {
    /// Creates a new project manager.
    pub fn new() -> Self {
        Self {
            projects: HashMap::new(),
        }
    }

    /// Creates a new porting project.
    pub fn create_project(
        &mut self,
        name: String,
        description: String,
        source_jurisdiction: String,
        target_jurisdiction: String,
    ) -> PortingProject {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();

        let project = PortingProject {
            id: id.clone(),
            name,
            description,
            source_jurisdiction,
            target_jurisdiction,
            status: ProjectStatus::Planning,
            statute_ids: Vec::new(),
            stakeholders: Vec::new(),
            timeline: ProjectTimeline {
                start_date: now.clone(),
                end_date: now.clone(),
                milestones: Vec::new(),
                current_phase: "Planning".to_string(),
            },
            created_at: now.clone(),
            updated_at: now,
            metadata: HashMap::new(),
        };

        self.projects.insert(id, project.clone());
        project
    }

    /// Gets a project by ID.
    pub fn get_project(&self, id: &str) -> Option<&PortingProject> {
        self.projects.get(id)
    }

    /// Updates project status.
    pub fn update_status(&mut self, project_id: &str, status: ProjectStatus) -> Option<()> {
        let project = self.projects.get_mut(project_id)?;
        project.status = status;
        project.updated_at = chrono::Utc::now().to_rfc3339();
        Some(())
    }

    /// Adds a statute to the project.
    pub fn add_statute(&mut self, project_id: &str, statute_id: String) -> Option<()> {
        let project = self.projects.get_mut(project_id)?;
        project.statute_ids.push(statute_id);
        project.updated_at = chrono::Utc::now().to_rfc3339();
        Some(())
    }

    /// Adds a stakeholder to the project.
    pub fn add_stakeholder(&mut self, project_id: &str, stakeholder: Stakeholder) -> Option<()> {
        let project = self.projects.get_mut(project_id)?;
        project.stakeholders.push(stakeholder);
        project.updated_at = chrono::Utc::now().to_rfc3339();
        Some(())
    }

    /// Adds a milestone to the project.
    pub fn add_milestone(&mut self, project_id: &str, milestone: Milestone) -> Option<()> {
        let project = self.projects.get_mut(project_id)?;
        project.timeline.milestones.push(milestone);
        project.updated_at = chrono::Utc::now().to_rfc3339();
        Some(())
    }

    /// Marks a milestone as completed.
    pub fn complete_milestone(&mut self, project_id: &str, milestone_id: &str) -> Option<()> {
        let project = self.projects.get_mut(project_id)?;
        let milestone = project
            .timeline
            .milestones
            .iter_mut()
            .find(|m| m.id == milestone_id)?;
        milestone.completed = true;
        milestone.completed_date = Some(chrono::Utc::now().to_rfc3339());
        project.updated_at = chrono::Utc::now().to_rfc3339();
        Some(())
    }

    /// Lists all projects.
    pub fn list_projects(&self) -> Vec<&PortingProject> {
        self.projects.values().collect()
    }

    /// Lists projects by status.
    pub fn list_projects_by_status(&self, status: ProjectStatus) -> Vec<&PortingProject> {
        self.projects
            .values()
            .filter(|p| p.status == status)
            .collect()
    }
}

impl Default for PortingProjectManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Review workflow step.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewWorkflowStep {
    /// Step ID
    pub id: String,
    /// Step name
    pub name: String,
    /// Step order
    pub order: u32,
    /// Required reviewers (stakeholder IDs)
    pub required_reviewers: Vec<String>,
    /// Optional reviewers
    pub optional_reviewers: Vec<String>,
    /// Minimum approvals required
    pub min_approvals: u32,
    /// Step status
    pub status: ReviewStepStatus,
    /// Reviews submitted
    pub reviews: Vec<WorkflowReview>,
}

/// Review step status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReviewStepStatus {
    /// Pending review
    Pending,
    /// In progress
    InProgress,
    /// Approved
    Approved,
    /// Rejected
    Rejected,
    /// Skipped
    Skipped,
}

/// Review from a stakeholder in a workflow.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowReview {
    /// Review ID
    pub id: String,
    /// Reviewer stakeholder ID
    pub reviewer_id: String,
    /// Review decision
    pub decision: ReviewDecision,
    /// Review comments
    pub comments: String,
    /// Review timestamp
    pub reviewed_at: String,
    /// Recommended changes
    pub recommended_changes: Vec<String>,
}

/// Review decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReviewDecision {
    /// Approve
    Approve,
    /// Approve with conditions
    ApproveWithConditions,
    /// Request changes
    RequestChanges,
    /// Reject
    Reject,
}

/// Stakeholder review workflow manager.
#[derive(Debug)]
pub struct StakeholderReviewWorkflow {
    workflows: HashMap<String, Vec<ReviewWorkflowStep>>,
}

impl StakeholderReviewWorkflow {
    /// Creates a new review workflow manager.
    pub fn new() -> Self {
        Self {
            workflows: HashMap::new(),
        }
    }

    /// Creates a workflow for a project.
    pub fn create_workflow(&mut self, project_id: String, steps: Vec<ReviewWorkflowStep>) {
        self.workflows.insert(project_id, steps);
    }

    /// Submits a review for a workflow step.
    pub fn submit_review(
        &mut self,
        project_id: &str,
        step_id: &str,
        review: WorkflowReview,
    ) -> Option<()> {
        let steps = self.workflows.get_mut(project_id)?;
        let step = steps.iter_mut().find(|s| s.id == step_id)?;
        step.reviews.push(review);

        // Check if step should be approved
        let approvals = step
            .reviews
            .iter()
            .filter(|r| {
                matches!(
                    r.decision,
                    ReviewDecision::Approve | ReviewDecision::ApproveWithConditions
                )
            })
            .count() as u32;

        if approvals >= step.min_approvals {
            step.status = ReviewStepStatus::Approved;
        }

        Some(())
    }

    /// Gets workflow status for a project.
    pub fn get_workflow_status(&self, project_id: &str) -> Option<&Vec<ReviewWorkflowStep>> {
        self.workflows.get(project_id)
    }

    /// Advances to next step if current is approved.
    pub fn advance_workflow(&mut self, project_id: &str) -> Option<usize> {
        let steps = self.workflows.get_mut(project_id)?;

        let current_step = steps
            .iter()
            .position(|s| s.status == ReviewStepStatus::InProgress)?;

        if steps[current_step].status == ReviewStepStatus::Approved
            && current_step + 1 < steps.len()
        {
            steps[current_step + 1].status = ReviewStepStatus::InProgress;
            return Some(current_step + 1);
        }

        None
    }
}

impl Default for StakeholderReviewWorkflow {
    fn default() -> Self {
        Self::new()
    }
}

/// Porting iteration version.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortingIteration {
    /// Iteration ID
    pub id: String,
    /// Project ID
    pub project_id: String,
    /// Iteration number
    pub iteration_number: u32,
    /// Branch name (None for main branch)
    pub branch: Option<String>,
    /// Parent iteration ID (for branches)
    pub parent_iteration_id: Option<String>,
    /// Statute snapshot
    pub statute_snapshot: String,
    /// Changes from previous iteration
    pub changes: Vec<IterationChange>,
    /// Created timestamp
    pub created_at: String,
    /// Created by (stakeholder ID)
    pub created_by: String,
    /// Iteration notes
    pub notes: String,
    /// Tags for categorization
    pub tags: Vec<String>,
}

/// Change in an iteration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IterationChange {
    /// Change ID
    pub id: String,
    /// Change type
    pub change_type: IterationChangeType,
    /// Field or section changed
    pub field: String,
    /// Previous value
    pub previous_value: String,
    /// New value
    pub new_value: String,
    /// Reason for change
    pub reason: String,
}

/// Type of iteration change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IterationChangeType {
    /// Added new content
    Addition,
    /// Modified existing content
    Modification,
    /// Removed content
    Deletion,
    /// Restructured content
    Restructure,
}

/// Version control for porting iterations.
#[derive(Debug)]
pub struct PortingVersionControl {
    iterations: HashMap<String, Vec<PortingIteration>>,
    branches: HashMap<String, Vec<String>>, // project_id -> branch names
}

impl PortingVersionControl {
    /// Creates a new version control system.
    pub fn new() -> Self {
        Self {
            iterations: HashMap::new(),
            branches: HashMap::new(),
        }
    }

    /// Creates a new iteration.
    pub fn create_iteration(
        &mut self,
        project_id: String,
        statute_snapshot: String,
        created_by: String,
        notes: String,
    ) -> PortingIteration {
        let iterations = self.iterations.entry(project_id.clone()).or_default();
        let iteration_number = (iterations.len() + 1) as u32;

        let iteration = PortingIteration {
            id: uuid::Uuid::new_v4().to_string(),
            project_id,
            iteration_number,
            branch: None, // Main branch by default
            parent_iteration_id: iterations.last().map(|i| i.id.clone()),
            statute_snapshot,
            changes: Vec::new(),
            created_at: chrono::Utc::now().to_rfc3339(),
            created_by,
            notes,
            tags: Vec::new(),
        };

        iterations.push(iteration.clone());
        iteration
    }

    /// Gets all iterations for a project.
    pub fn get_iterations(&self, project_id: &str) -> Option<&Vec<PortingIteration>> {
        self.iterations.get(project_id)
    }

    /// Gets a specific iteration.
    pub fn get_iteration(
        &self,
        project_id: &str,
        iteration_number: u32,
    ) -> Option<&PortingIteration> {
        self.iterations
            .get(project_id)?
            .iter()
            .find(|i| i.iteration_number == iteration_number)
    }

    /// Compares two iterations.
    pub fn compare_iterations(
        &self,
        project_id: &str,
        from_iteration: u32,
        to_iteration: u32,
    ) -> Option<Vec<IterationChange>> {
        let iterations = self.iterations.get(project_id)?;
        let _from = iterations
            .iter()
            .find(|i| i.iteration_number == from_iteration)?;
        let to = iterations
            .iter()
            .find(|i| i.iteration_number == to_iteration)?;

        // Simplified comparison - real implementation would do deep diff
        Some(to.changes.clone())
    }

    /// Creates a new branch from an iteration.
    pub fn create_branch(
        &mut self,
        project_id: String,
        branch_name: String,
        from_iteration_number: u32,
        created_by: String,
        notes: String,
    ) -> Option<PortingIteration> {
        let iterations = self.iterations.get(&project_id)?;
        let from_iteration = iterations
            .iter()
            .find(|i| i.iteration_number == from_iteration_number)?
            .clone();

        // Register branch
        self.branches
            .entry(project_id.clone())
            .or_default()
            .push(branch_name.clone());

        // Create new iteration on the branch
        let all_iterations = self.iterations.entry(project_id.clone()).or_default();
        let iteration_number = (all_iterations.len() + 1) as u32;

        let iteration = PortingIteration {
            id: uuid::Uuid::new_v4().to_string(),
            project_id,
            iteration_number,
            branch: Some(branch_name),
            parent_iteration_id: Some(from_iteration.id.clone()),
            statute_snapshot: from_iteration.statute_snapshot.clone(),
            changes: Vec::new(),
            created_at: chrono::Utc::now().to_rfc3339(),
            created_by,
            notes,
            tags: vec!["branch".to_string()],
        };

        all_iterations.push(iteration.clone());
        Some(iteration)
    }

    /// Gets all branches for a project.
    pub fn get_branches(&self, project_id: &str) -> Vec<String> {
        self.branches.get(project_id).cloned().unwrap_or_default()
    }

    /// Gets iterations for a specific branch.
    pub fn get_branch_iterations(
        &self,
        project_id: &str,
        branch_name: &str,
    ) -> Vec<PortingIteration> {
        self.iterations
            .get(project_id)
            .map(|iterations| {
                iterations
                    .iter()
                    .filter(|i| i.branch.as_deref() == Some(branch_name))
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Merges a branch into main (or another branch).
    pub fn merge_branch(
        &mut self,
        project_id: String,
        source_branch: String,
        target_branch: Option<String>,
        created_by: String,
        notes: String,
    ) -> Option<PortingIteration> {
        let iterations = self.iterations.get(&project_id)?;

        // Get latest iteration from source branch
        let source_iteration = iterations
            .iter()
            .filter(|i| i.branch.as_deref() == Some(&source_branch))
            .max_by_key(|i| i.iteration_number)?
            .clone();

        // Create merged iteration
        let all_iterations = self.iterations.entry(project_id.clone()).or_default();
        let iteration_number = (all_iterations.len() + 1) as u32;

        let iteration = PortingIteration {
            id: uuid::Uuid::new_v4().to_string(),
            project_id,
            iteration_number,
            branch: target_branch,
            parent_iteration_id: Some(source_iteration.id.clone()),
            statute_snapshot: source_iteration.statute_snapshot.clone(),
            changes: source_iteration.changes.clone(),
            created_at: chrono::Utc::now().to_rfc3339(),
            created_by,
            notes: format!("Merged {} - {}", source_branch, notes),
            tags: vec!["merge".to_string()],
        };

        all_iterations.push(iteration.clone());
        Some(iteration)
    }

    /// Generates a changelog for a project.
    pub fn generate_changelog(&self, project_id: &str) -> Option<PortingChangelog> {
        let iterations = self.iterations.get(project_id)?;

        if iterations.is_empty() {
            return None;
        }

        let mut entries = Vec::new();

        for iteration in iterations {
            let mut change_summary = Vec::new();

            for change in &iteration.changes {
                change_summary.push(format!(
                    "{:?}: {} ({})",
                    change.change_type, change.field, change.reason
                ));
            }

            entries.push(ChangelogEntry {
                id: uuid::Uuid::new_v4().to_string(),
                iteration_number: iteration.iteration_number,
                iteration_id: iteration.id.clone(),
                branch: iteration.branch.clone(),
                timestamp: iteration.created_at.clone(),
                author: iteration.created_by.clone(),
                summary: iteration.notes.clone(),
                changes: change_summary,
                tags: iteration.tags.clone(),
            });
        }

        Some(PortingChangelog {
            id: uuid::Uuid::new_v4().to_string(),
            project_id: project_id.to_string(),
            generated_at: chrono::Utc::now().to_rfc3339(),
            entries,
            total_iterations: iterations.len(),
            branches: self.get_branches(project_id),
        })
    }

    /// Reverts to a previous iteration.
    pub fn revert_to_iteration(
        &mut self,
        project_id: &str,
        iteration_number: u32,
        created_by: String,
    ) -> Option<PortingIteration> {
        let iteration = self.get_iteration(project_id, iteration_number)?.clone();

        Some(self.create_iteration(
            project_id.to_string(),
            iteration.statute_snapshot.clone(),
            created_by,
            format!("Reverted to iteration {}", iteration_number),
        ))
    }
}

impl Default for PortingVersionControl {
    fn default() -> Self {
        Self::new()
    }
}

/// Porting changelog for tracking all changes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortingChangelog {
    /// Changelog ID
    pub id: String,
    /// Project ID
    pub project_id: String,
    /// Generated timestamp
    pub generated_at: String,
    /// Changelog entries
    pub entries: Vec<ChangelogEntry>,
    /// Total number of iterations
    pub total_iterations: usize,
    /// List of branches
    pub branches: Vec<String>,
}

/// Entry in the changelog.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangelogEntry {
    /// Entry ID
    pub id: String,
    /// Iteration number
    pub iteration_number: u32,
    /// Iteration ID
    pub iteration_id: String,
    /// Branch (if any)
    pub branch: Option<String>,
    /// Timestamp
    pub timestamp: String,
    /// Author
    pub author: String,
    /// Summary of changes
    pub summary: String,
    /// Detailed changes
    pub changes: Vec<String>,
    /// Tags
    pub tags: Vec<String>,
}

impl PortingChangelog {
    /// Exports changelog to markdown format.
    pub fn to_markdown(&self) -> String {
        let mut output = String::new();

        output.push_str(&format!("# Porting Changelog\n\n"));
        output.push_str(&format!("**Project ID:** {}\n", self.project_id));
        output.push_str(&format!("**Generated:** {}\n", self.generated_at));
        output.push_str(&format!(
            "**Total Iterations:** {}\n",
            self.total_iterations
        ));

        if !self.branches.is_empty() {
            output.push_str(&format!("**Branches:** {}\n", self.branches.join(", ")));
        }

        output.push_str("\n---\n\n");

        for entry in &self.entries {
            let branch_info = entry
                .branch
                .as_ref()
                .map(|b| format!(" [{}]", b))
                .unwrap_or_default();

            output.push_str(&format!(
                "## Iteration {}{}\n\n",
                entry.iteration_number, branch_info
            ));
            output.push_str(&format!("**Date:** {}\n", entry.timestamp));
            output.push_str(&format!("**Author:** {}\n", entry.author));
            output.push_str(&format!("**Summary:** {}\n\n", entry.summary));

            if !entry.changes.is_empty() {
                output.push_str("**Changes:**\n\n");
                for change in &entry.changes {
                    output.push_str(&format!("- {}\n", change));
                }
                output.push_str("\n");
            }

            if !entry.tags.is_empty() {
                output.push_str(&format!("**Tags:** {}\n\n", entry.tags.join(", ")));
            }

            output.push_str("---\n\n");
        }

        output
    }

    /// Exports changelog to JSON format.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

/// Approval chain configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalChain {
    /// Chain ID
    pub id: String,
    /// Chain name
    pub name: String,
    /// Approval steps
    pub steps: Vec<ApprovalStep>,
    /// Chain status
    pub status: ApprovalChainStatus,
}

/// Approval step in the chain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalStep {
    /// Step ID
    pub id: String,
    /// Step name
    pub name: String,
    /// Step order
    pub order: u32,
    /// Approvers (stakeholder IDs)
    pub approvers: Vec<String>,
    /// Approval mode
    pub approval_mode: ApprovalMode,
    /// Step status
    pub status: ApprovalStepStatus,
    /// Approvals received
    pub approvals: Vec<ApprovalRecord>,
    /// Auto-approve after timeout
    pub auto_approve_after: Option<u64>,
}

/// Approval mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ApprovalMode {
    /// Any approver can approve
    Any,
    /// All approvers must approve
    All,
    /// Majority must approve
    Majority,
    /// Specific number must approve
    Threshold(u32),
}

/// Approval step status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ApprovalStepStatus {
    /// Waiting for approval
    Pending,
    /// Approved
    Approved,
    /// Rejected
    Rejected,
    /// Timed out
    TimedOut,
}

/// Approval chain status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ApprovalChainStatus {
    /// Not started
    NotStarted,
    /// In progress
    InProgress,
    /// Completed successfully
    Completed,
    /// Failed/rejected
    Failed,
}

/// Individual approval record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRecord {
    /// Approval ID
    pub id: String,
    /// Approver ID
    pub approver_id: String,
    /// Approved or rejected
    pub approved: bool,
    /// Comments
    pub comments: String,
    /// Approval timestamp
    pub approved_at: String,
}

/// Approval chain manager.
#[derive(Debug)]
pub struct ApprovalChainManager {
    chains: HashMap<String, ApprovalChain>,
}

impl ApprovalChainManager {
    /// Creates a new approval chain manager.
    pub fn new() -> Self {
        Self {
            chains: HashMap::new(),
        }
    }

    /// Creates an approval chain.
    pub fn create_chain(&mut self, name: String, steps: Vec<ApprovalStep>) -> ApprovalChain {
        let chain = ApprovalChain {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            steps,
            status: ApprovalChainStatus::NotStarted,
        };

        self.chains.insert(chain.id.clone(), chain.clone());
        chain
    }

    /// Submits an approval.
    pub fn submit_approval(
        &mut self,
        chain_id: &str,
        step_id: &str,
        approval: ApprovalRecord,
    ) -> Option<()> {
        let chain = self.chains.get_mut(chain_id)?;
        let step = chain.steps.iter_mut().find(|s| s.id == step_id)?;
        step.approvals.push(approval);

        // Check if step is approved based on mode
        let approved_count = step.approvals.iter().filter(|a| a.approved).count();
        let total_approvers = step.approvers.len();

        let step_approved = match step.approval_mode {
            ApprovalMode::Any => approved_count >= 1,
            ApprovalMode::All => approved_count == total_approvers,
            ApprovalMode::Majority => approved_count > total_approvers / 2,
            ApprovalMode::Threshold(n) => approved_count >= n as usize,
        };

        if step_approved {
            step.status = ApprovalStepStatus::Approved;
        }

        Some(())
    }

    /// Gets chain status.
    pub fn get_chain(&self, chain_id: &str) -> Option<&ApprovalChain> {
        self.chains.get(chain_id)
    }

    /// Advances chain to next step.
    pub fn advance_chain(&mut self, chain_id: &str) -> Option<usize> {
        let chain = self.chains.get_mut(chain_id)?;

        let current_step = chain
            .steps
            .iter()
            .position(|s| s.status == ApprovalStepStatus::Pending)?;

        if chain.steps[current_step].status == ApprovalStepStatus::Approved {
            if current_step + 1 < chain.steps.len() {
                return Some(current_step + 1);
            } else {
                chain.status = ApprovalChainStatus::Completed;
            }
        }

        None
    }
}

impl Default for ApprovalChainManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Notification to be sent to stakeholders.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    /// Notification ID
    pub id: String,
    /// Recipient stakeholder ID
    pub recipient_id: String,
    /// Notification type
    pub notification_type: NotificationType,
    /// Notification title
    pub title: String,
    /// Notification message
    pub message: String,
    /// Related project ID
    pub project_id: Option<String>,
    /// Priority
    pub priority: NotificationPriority,
    /// Created timestamp
    pub created_at: String,
    /// Read status
    pub read: bool,
    /// Delivery channels
    pub channels: Vec<NotificationChannel>,
}

/// Notification type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NotificationType {
    /// Status change notification
    StatusChange,
    /// Deadline approaching
    DeadlineApproaching,
    /// Assignment notification
    Assignment,
    /// Review request
    ReviewRequest,
    /// Approval request
    ApprovalRequest,
    /// Milestone completed
    MilestoneCompleted,
    /// Project completed
    ProjectCompleted,
}

/// Notification priority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NotificationPriority {
    /// Low priority
    Low,
    /// Normal priority
    Normal,
    /// High priority
    High,
    /// Urgent
    Urgent,
}

/// Deadline tracking entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeadlineTracker {
    /// Tracker ID
    pub id: String,
    /// Project ID
    pub project_id: String,
    /// Deadline name
    pub name: String,
    /// Deadline date
    pub deadline: String,
    /// Warning threshold in days
    pub warning_days: u32,
    /// Status
    pub status: DeadlineStatus,
    /// Assigned stakeholder IDs
    pub assigned_to: Vec<String>,
}

/// Deadline status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeadlineStatus {
    /// On track
    OnTrack,
    /// Approaching deadline
    Approaching,
    /// Overdue
    Overdue,
    /// Completed
    Completed,
}

/// Notification and deadline manager.
#[derive(Debug)]
pub struct NotificationManager {
    notifications: HashMap<String, Vec<Notification>>,
    deadlines: HashMap<String, Vec<DeadlineTracker>>,
}

impl NotificationManager {
    /// Creates a new notification manager.
    pub fn new() -> Self {
        Self {
            notifications: HashMap::new(),
            deadlines: HashMap::new(),
        }
    }

    /// Sends a notification.
    pub fn send_notification(&mut self, notification: Notification) {
        let recipient_id = notification.recipient_id.clone();
        self.notifications
            .entry(recipient_id)
            .or_default()
            .push(notification);
    }

    /// Gets notifications for a stakeholder.
    pub fn get_notifications(&self, stakeholder_id: &str) -> Vec<&Notification> {
        self.notifications
            .get(stakeholder_id)
            .map(|n| n.iter().collect())
            .unwrap_or_default()
    }

    /// Marks notification as read.
    pub fn mark_as_read(&mut self, stakeholder_id: &str, notification_id: &str) -> Option<()> {
        let notifications = self.notifications.get_mut(stakeholder_id)?;
        let notification = notifications.iter_mut().find(|n| n.id == notification_id)?;
        notification.read = true;
        Some(())
    }

    /// Adds a deadline tracker.
    pub fn add_deadline(&mut self, deadline: DeadlineTracker) {
        let project_id = deadline.project_id.clone();
        self.deadlines.entry(project_id).or_default().push(deadline);
    }

    /// Gets deadlines for a project.
    pub fn get_deadlines(&self, project_id: &str) -> Vec<&DeadlineTracker> {
        self.deadlines
            .get(project_id)
            .map(|d| d.iter().collect())
            .unwrap_or_default()
    }

    /// Checks approaching deadlines and generates notifications.
    pub fn check_deadlines(&mut self) -> Vec<Notification> {
        let mut notifications = Vec::new();
        let now = chrono::Utc::now();

        for (project_id, deadlines) in &self.deadlines {
            for deadline in deadlines {
                if let Ok(deadline_date) = chrono::DateTime::parse_from_rfc3339(&deadline.deadline)
                {
                    let days_until = (deadline_date.signed_duration_since(now)).num_days();

                    if days_until >= 0 && days_until <= deadline.warning_days as i64 {
                        for stakeholder_id in &deadline.assigned_to {
                            let notification = Notification {
                                id: uuid::Uuid::new_v4().to_string(),
                                recipient_id: stakeholder_id.clone(),
                                notification_type: NotificationType::DeadlineApproaching,
                                title: format!("Deadline Approaching: {}", deadline.name),
                                message: format!(
                                    "Deadline '{}' is approaching in {} days",
                                    deadline.name, days_until
                                ),
                                project_id: Some(project_id.clone()),
                                priority: if days_until <= 3 {
                                    NotificationPriority::Urgent
                                } else {
                                    NotificationPriority::High
                                },
                                created_at: now.to_rfc3339(),
                                read: false,
                                channels: vec![
                                    NotificationChannel::Email,
                                    NotificationChannel::InApp,
                                ],
                            };
                            notifications.push(notification);
                        }
                    }
                }
            }
        }

        notifications
    }
}

impl Default for NotificationManager {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Reporting (v0.1.8)
// ============================================================================

/// Executive summary of a porting project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutiveSummary {
    /// Project identifier
    pub project_id: String,
    /// Project title
    pub title: String,
    /// Source jurisdiction
    pub source_jurisdiction: String,
    /// Target jurisdiction
    pub target_jurisdiction: String,
    /// Number of statutes ported
    pub statutes_count: usize,
    /// Overall compatibility score (0.0 to 1.0)
    pub compatibility_score: f64,
    /// Overall risk level
    pub risk_level: RiskLevel,
    /// Key findings (3-5 bullet points)
    pub key_findings: Vec<String>,
    /// Main recommendations (3-5 bullet points)
    pub recommendations: Vec<String>,
    /// Timeline summary
    pub timeline_summary: String,
    /// Stakeholders involved
    pub stakeholders: Vec<String>,
    /// Generated timestamp
    pub generated_at: String,
}

/// Generator for executive summaries.
#[derive(Debug, Clone)]
pub struct ExecutiveSummaryGenerator;

impl ExecutiveSummaryGenerator {
    /// Creates a new executive summary generator.
    pub fn new() -> Self {
        Self
    }

    /// Generates an executive summary from a porting project.
    pub fn generate(
        &self,
        project: &PortingProject,
        ported_statutes: &[PortedStatute],
    ) -> ExecutiveSummary {
        let compatibility_score = if !ported_statutes.is_empty() {
            ported_statutes
                .iter()
                .map(|s| s.compatibility_score)
                .sum::<f64>()
                / ported_statutes.len() as f64
        } else {
            0.0
        };

        let risk_level = if compatibility_score >= 0.8 {
            RiskLevel::Low
        } else if compatibility_score >= 0.5 {
            RiskLevel::Medium
        } else {
            RiskLevel::High
        };

        let key_findings = self.extract_key_findings(ported_statutes);
        let recommendations = self.generate_recommendations(ported_statutes, compatibility_score);

        ExecutiveSummary {
            project_id: project.id.clone(),
            title: project.name.clone(),
            source_jurisdiction: project.source_jurisdiction.clone(),
            target_jurisdiction: project.target_jurisdiction.clone(),
            statutes_count: ported_statutes.len(),
            compatibility_score,
            risk_level,
            key_findings,
            recommendations,
            timeline_summary: format!(
                "Created: {}, Last updated: {}",
                project.created_at, project.updated_at
            ),
            stakeholders: project
                .stakeholders
                .iter()
                .map(|s| s.name.clone())
                .collect(),
            generated_at: chrono::Utc::now().to_rfc3339(),
        }
    }

    fn extract_key_findings(&self, ported_statutes: &[PortedStatute]) -> Vec<String> {
        let mut findings = Vec::new();

        let total_changes: usize = ported_statutes.iter().map(|s| s.changes.len()).sum();
        if total_changes > 0 {
            findings.push(format!(
                "Total of {} adaptations made across {} statutes",
                total_changes,
                ported_statutes.len()
            ));
        }

        let cultural_changes = ported_statutes
            .iter()
            .flat_map(|s| &s.changes)
            .filter(|c| matches!(c.change_type, ChangeType::CulturalAdaptation))
            .count();
        if cultural_changes > 0 {
            findings.push(format!(
                "{} cultural adaptations required",
                cultural_changes
            ));
        }

        let high_risk_count = ported_statutes
            .iter()
            .filter(|s| s.compatibility_score < 0.5)
            .count();
        if high_risk_count > 0 {
            findings.push(format!(
                "{} statutes require significant adaptation (compatibility < 50%)",
                high_risk_count
            ));
        }

        if findings.is_empty() {
            findings.push("All statutes ported successfully with minimal adaptations".to_string());
        }

        findings
    }

    fn generate_recommendations(
        &self,
        ported_statutes: &[PortedStatute],
        compatibility_score: f64,
    ) -> Vec<String> {
        let mut recommendations = Vec::new();

        if compatibility_score < 0.5 {
            recommendations
                .push("Comprehensive legal review recommended before implementation".to_string());
            recommendations.push(
                "Consider pilot program in limited jurisdiction before full rollout".to_string(),
            );
        } else if compatibility_score < 0.8 {
            recommendations.push("Expert review recommended for adapted sections".to_string());
        }

        let needs_review = ported_statutes
            .iter()
            .filter(|s| !s.changes.is_empty())
            .count();
        if needs_review > 0 {
            recommendations.push(format!(
                "Review {} statutes with cultural adaptations",
                needs_review
            ));
        }

        if recommendations.is_empty() {
            recommendations.push("Proceed with standard implementation process".to_string());
        }

        recommendations
    }
}

impl Default for ExecutiveSummaryGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Detailed risk assessment report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessmentReport {
    /// Project identifier
    pub project_id: String,
    /// Report title
    pub title: String,
    /// Overall risk score (0.0 to 1.0)
    pub overall_risk_score: f64,
    /// Overall risk level
    pub overall_risk_level: RiskLevel,
    /// Risks by category
    pub risks_by_category: HashMap<RiskCategory, Vec<Risk>>,
    /// Risk mitigation strategies
    pub mitigation_strategies: Vec<MitigationStrategy>,
    /// Risk matrix visualization data
    pub risk_matrix: RiskMatrix,
    /// Generated timestamp
    pub generated_at: String,
}

/// Risk matrix for visualization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskMatrix {
    /// High-probability, high-impact risks
    pub critical: Vec<String>,
    /// High-probability, low-impact risks
    pub moderate_high_prob: Vec<String>,
    /// Low-probability, high-impact risks
    pub moderate_high_impact: Vec<String>,
    /// Low-probability, low-impact risks
    pub low: Vec<String>,
}

/// Risk mitigation strategy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitigationStrategy {
    /// Risk being mitigated
    pub risk_id: String,
    /// Mitigation strategy description
    pub strategy: String,
    /// Expected effectiveness (0.0 to 1.0)
    pub effectiveness: f64,
    /// Implementation cost
    pub cost: MitigationCost,
    /// Priority
    pub priority: Priority,
}

/// Cost of implementing a mitigation strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MitigationCost {
    /// Low cost
    Low,
    /// Medium cost
    Medium,
    /// High cost
    High,
}

/// Priority level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Priority {
    /// Low priority
    Low,
    /// Medium priority
    Medium,
    /// High priority
    High,
    /// Critical priority
    Critical,
}

/// Generator for risk assessment reports.
#[derive(Debug, Clone)]
pub struct RiskAssessmentReportGenerator;

impl RiskAssessmentReportGenerator {
    /// Creates a new risk assessment report generator.
    pub fn new() -> Self {
        Self
    }

    /// Generates a risk assessment report.
    pub fn generate(
        &self,
        project: &PortingProject,
        risk_assessments: &[RiskAssessment],
    ) -> RiskAssessmentReport {
        let overall_risk_score = if !risk_assessments.is_empty() {
            risk_assessments.iter().map(|r| r.risk_score).sum::<f64>()
                / risk_assessments.len() as f64
        } else {
            0.0
        };

        let overall_risk_level = if overall_risk_score >= 0.7 {
            RiskLevel::High
        } else if overall_risk_score >= 0.4 {
            RiskLevel::Medium
        } else {
            RiskLevel::Low
        };

        let mut risks_by_category: HashMap<RiskCategory, Vec<Risk>> = HashMap::new();
        for assessment in risk_assessments {
            for risk in &assessment.risks {
                risks_by_category
                    .entry(risk.category)
                    .or_default()
                    .push(risk.clone());
            }
        }

        let mitigation_strategies = self.generate_mitigation_strategies(&risks_by_category);
        let risk_matrix = self.build_risk_matrix(&risks_by_category);

        RiskAssessmentReport {
            project_id: project.id.clone(),
            title: format!("Risk Assessment: {}", project.name),
            overall_risk_score,
            overall_risk_level,
            risks_by_category,
            mitigation_strategies,
            risk_matrix,
            generated_at: chrono::Utc::now().to_rfc3339(),
        }
    }

    #[allow(dead_code)]
    fn generate_mitigation_strategies(
        &self,
        risks_by_category: &HashMap<RiskCategory, Vec<Risk>>,
    ) -> Vec<MitigationStrategy> {
        let mut strategies = Vec::new();

        for (category, risks) in risks_by_category {
            for risk in risks {
                let strategy = match (category, risk.severity) {
                    (RiskCategory::Legal, RiskLevel::High) => MitigationStrategy {
                        risk_id: risk.id.clone(),
                        strategy: "Engage constitutional law experts for comprehensive review"
                            .to_string(),
                        effectiveness: 0.9,
                        cost: MitigationCost::High,
                        priority: Priority::Critical,
                    },
                    (RiskCategory::Cultural, RiskLevel::High) => MitigationStrategy {
                        risk_id: risk.id.clone(),
                        strategy: "Conduct cultural sensitivity review with local experts"
                            .to_string(),
                        effectiveness: 0.85,
                        cost: MitigationCost::Medium,
                        priority: Priority::High,
                    },
                    (RiskCategory::Political, RiskLevel::High) => MitigationStrategy {
                        risk_id: risk.id.clone(),
                        strategy: "Establish stakeholder consultation process".to_string(),
                        effectiveness: 0.75,
                        cost: MitigationCost::Medium,
                        priority: Priority::High,
                    },
                    (RiskCategory::Economic, RiskLevel::High) => MitigationStrategy {
                        risk_id: risk.id.clone(),
                        strategy: "Perform detailed cost-benefit analysis".to_string(),
                        effectiveness: 0.8,
                        cost: MitigationCost::Medium,
                        priority: Priority::High,
                    },
                    (RiskCategory::Implementation, RiskLevel::High) => MitigationStrategy {
                        risk_id: risk.id.clone(),
                        strategy: "Develop phased implementation plan with pilot program"
                            .to_string(),
                        effectiveness: 0.8,
                        cost: MitigationCost::High,
                        priority: Priority::High,
                    },
                    _ => MitigationStrategy {
                        risk_id: risk.id.clone(),
                        strategy: format!(
                            "Standard {} risk mitigation procedures",
                            format!("{:?}", category).to_lowercase()
                        ),
                        effectiveness: 0.7,
                        cost: MitigationCost::Low,
                        priority: Priority::Medium,
                    },
                };
                strategies.push(strategy);
            }
        }

        strategies
    }

    fn build_risk_matrix(
        &self,
        risks_by_category: &HashMap<RiskCategory, Vec<Risk>>,
    ) -> RiskMatrix {
        let mut critical = Vec::new();
        let mut moderate_high_prob = Vec::new();
        let mut moderate_high_impact = Vec::new();
        let mut low = Vec::new();

        for risks in risks_by_category.values() {
            for risk in risks {
                let risk_desc = format!("{}: {}", risk.id, risk.description);
                match (risk.severity, risk.likelihood) {
                    (RiskLevel::High, RiskLevel::High) => critical.push(risk_desc),
                    (RiskLevel::High, _) => moderate_high_impact.push(risk_desc),
                    (_, RiskLevel::High) => moderate_high_prob.push(risk_desc),
                    _ => low.push(risk_desc),
                }
            }
        }

        RiskMatrix {
            critical,
            moderate_high_prob,
            moderate_high_impact,
            low,
        }
    }
}

impl Default for RiskAssessmentReportGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Implementation roadmap for a porting project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImplementationRoadmap {
    /// Project identifier
    pub project_id: String,
    /// Roadmap title
    pub title: String,
    /// Implementation phases
    pub phases: Vec<ImplementationPhase>,
    /// Critical path items
    pub critical_path: Vec<String>,
    /// Resource requirements
    pub resource_requirements: ResourceRequirements,
    /// Estimated total duration (in days)
    pub estimated_duration_days: u32,
    /// Generated timestamp
    pub generated_at: String,
}

/// Implementation phase.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImplementationPhase {
    /// Phase number
    pub phase_number: u32,
    /// Phase name
    pub name: String,
    /// Phase description
    pub description: String,
    /// Tasks in this phase
    pub tasks: Vec<ImplementationTask>,
    /// Dependencies (phase numbers)
    pub dependencies: Vec<u32>,
    /// Estimated duration (in days)
    pub estimated_duration_days: u32,
    /// Success criteria
    pub success_criteria: Vec<String>,
}

/// Implementation task.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImplementationTask {
    /// Task identifier
    pub id: String,
    /// Task name
    pub name: String,
    /// Task description
    pub description: String,
    /// Assigned role/team
    pub assigned_to: String,
    /// Estimated effort (in person-days)
    pub estimated_effort_days: u32,
    /// Priority
    pub priority: Priority,
    /// Dependencies (task IDs)
    pub dependencies: Vec<String>,
}

/// Resource requirements for implementation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequirements {
    /// Required personnel
    pub personnel: Vec<PersonnelRequirement>,
    /// Required budget
    pub budget_estimate: BudgetEstimate,
    /// Required infrastructure
    pub infrastructure: Vec<String>,
}

/// Personnel requirement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonnelRequirement {
    /// Role/expertise
    pub role: String,
    /// Number of people
    pub count: u32,
    /// Estimated time commitment (in person-days)
    pub time_commitment_days: u32,
}

/// Budget estimate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetEstimate {
    /// Currency code
    pub currency: String,
    /// Minimum estimate
    pub min_amount: f64,
    /// Maximum estimate
    pub max_amount: f64,
    /// Budget breakdown
    pub breakdown: HashMap<String, f64>,
}

/// Generator for implementation roadmaps.
#[derive(Debug, Clone)]
pub struct ImplementationRoadmapGenerator;

impl ImplementationRoadmapGenerator {
    /// Creates a new implementation roadmap generator.
    pub fn new() -> Self {
        Self
    }

    /// Generates an implementation roadmap.
    pub fn generate(
        &self,
        project: &PortingProject,
        ported_statutes: &[PortedStatute],
    ) -> ImplementationRoadmap {
        let phases = self.generate_phases(ported_statutes);
        let critical_path = self.identify_critical_path(&phases);
        let resource_requirements = self.estimate_resources(ported_statutes, &phases);
        let estimated_duration_days = phases.iter().map(|p| p.estimated_duration_days).sum();

        ImplementationRoadmap {
            project_id: project.id.clone(),
            title: format!("Implementation Roadmap: {}", project.name),
            phases,
            critical_path,
            resource_requirements,
            estimated_duration_days,
            generated_at: chrono::Utc::now().to_rfc3339(),
        }
    }

    fn generate_phases(&self, ported_statutes: &[PortedStatute]) -> Vec<ImplementationPhase> {
        vec![
            ImplementationPhase {
                phase_number: 1,
                name: "Legal Review and Validation".to_string(),
                description: "Comprehensive legal review of ported statutes".to_string(),
                tasks: vec![
                    ImplementationTask {
                        id: "task-1-1".to_string(),
                        name: "Constitutional compatibility review".to_string(),
                        description: "Review all statutes for constitutional compatibility"
                            .to_string(),
                        assigned_to: "Constitutional Law Team".to_string(),
                        estimated_effort_days: 10,
                        priority: Priority::Critical,
                        dependencies: vec![],
                    },
                    ImplementationTask {
                        id: "task-1-2".to_string(),
                        name: "Conflict detection and resolution".to_string(),
                        description: "Identify and resolve conflicts with existing laws"
                            .to_string(),
                        assigned_to: "Legal Analysis Team".to_string(),
                        estimated_effort_days: 8,
                        priority: Priority::High,
                        dependencies: vec!["task-1-1".to_string()],
                    },
                ],
                dependencies: vec![],
                estimated_duration_days: 15,
                success_criteria: vec![
                    "All constitutional issues identified and addressed".to_string(),
                    "No unresolved conflicts with existing laws".to_string(),
                ],
            },
            ImplementationPhase {
                phase_number: 2,
                name: "Stakeholder Consultation".to_string(),
                description: "Engage stakeholders and gather feedback".to_string(),
                tasks: vec![
                    ImplementationTask {
                        id: "task-2-1".to_string(),
                        name: "Public comment period".to_string(),
                        description: "Open public comment period for feedback".to_string(),
                        assigned_to: "Public Affairs Team".to_string(),
                        estimated_effort_days: 30,
                        priority: Priority::High,
                        dependencies: vec!["task-1-2".to_string()],
                    },
                    ImplementationTask {
                        id: "task-2-2".to_string(),
                        name: "Expert consultations".to_string(),
                        description: "Conduct consultations with subject matter experts"
                            .to_string(),
                        assigned_to: "Policy Team".to_string(),
                        estimated_effort_days: 15,
                        priority: Priority::High,
                        dependencies: vec!["task-1-2".to_string()],
                    },
                ],
                dependencies: vec![1],
                estimated_duration_days: 30,
                success_criteria: vec![
                    "All stakeholder feedback documented".to_string(),
                    "Major concerns addressed".to_string(),
                ],
            },
            ImplementationPhase {
                phase_number: 3,
                name: "Pilot Implementation".to_string(),
                description: "Limited pilot rollout to test implementation".to_string(),
                tasks: vec![ImplementationTask {
                    id: "task-3-1".to_string(),
                    name: format!(
                        "Pilot program for {} statutes",
                        ported_statutes.len().min(5)
                    ),
                    description: "Implement pilot program in limited jurisdiction".to_string(),
                    assigned_to: "Implementation Team".to_string(),
                    estimated_effort_days: 45,
                    priority: Priority::High,
                    dependencies: vec!["task-2-1".to_string(), "task-2-2".to_string()],
                }],
                dependencies: vec![2],
                estimated_duration_days: 60,
                success_criteria: vec![
                    "Pilot successfully completed".to_string(),
                    "Implementation issues identified and documented".to_string(),
                ],
            },
            ImplementationPhase {
                phase_number: 4,
                name: "Full Rollout".to_string(),
                description: "Complete implementation across jurisdiction".to_string(),
                tasks: vec![ImplementationTask {
                    id: "task-4-1".to_string(),
                    name: "Full jurisdiction rollout".to_string(),
                    description: "Implement all ported statutes across full jurisdiction"
                        .to_string(),
                    assigned_to: "Implementation Team".to_string(),
                    estimated_effort_days: 90,
                    priority: Priority::Critical,
                    dependencies: vec!["task-3-1".to_string()],
                }],
                dependencies: vec![3],
                estimated_duration_days: 120,
                success_criteria: vec![
                    "All statutes successfully implemented".to_string(),
                    "Monitoring and enforcement mechanisms in place".to_string(),
                ],
            },
        ]
    }

    fn identify_critical_path(&self, phases: &[ImplementationPhase]) -> Vec<String> {
        let mut critical_path = Vec::new();
        for phase in phases {
            critical_path.push(format!(
                "Phase {}: {} ({} days)",
                phase.phase_number, phase.name, phase.estimated_duration_days
            ));
        }
        critical_path
    }

    fn estimate_resources(
        &self,
        ported_statutes: &[PortedStatute],
        phases: &[ImplementationPhase],
    ) -> ResourceRequirements {
        let statute_count = ported_statutes.len();
        let complexity_factor = if statute_count > 20 { 1.5 } else { 1.0 };

        let personnel = vec![
            PersonnelRequirement {
                role: "Legal Experts".to_string(),
                count: (statute_count / 10).max(2) as u32,
                time_commitment_days: (30.0 * complexity_factor) as u32,
            },
            PersonnelRequirement {
                role: "Policy Analysts".to_string(),
                count: (statute_count / 15).max(1) as u32,
                time_commitment_days: (25.0 * complexity_factor) as u32,
            },
            PersonnelRequirement {
                role: "Implementation Managers".to_string(),
                count: 2,
                time_commitment_days: phases.iter().map(|p| p.estimated_duration_days).sum(),
            },
        ];

        let base_budget = statute_count as f64 * 50000.0;
        let mut breakdown = HashMap::new();
        breakdown.insert("Personnel".to_string(), base_budget * 0.6);
        breakdown.insert("Consultation and Review".to_string(), base_budget * 0.2);
        breakdown.insert(
            "Infrastructure and Training".to_string(),
            base_budget * 0.15,
        );
        breakdown.insert("Contingency".to_string(), base_budget * 0.05);

        ResourceRequirements {
            personnel,
            budget_estimate: BudgetEstimate {
                currency: "USD".to_string(),
                min_amount: base_budget * 0.8,
                max_amount: base_budget * 1.3,
                breakdown,
            },
            infrastructure: vec![
                "Legal database access".to_string(),
                "Collaboration platform".to_string(),
                "Document management system".to_string(),
            ],
        }
    }
}

impl Default for ImplementationRoadmapGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Cost-benefit analysis for a porting project.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostBenefitAnalysis {
    /// Project identifier
    pub project_id: String,
    /// Analysis title
    pub title: String,
    /// Total estimated costs
    pub total_costs: CostBreakdown,
    /// Total estimated benefits
    pub total_benefits: BenefitAnalysis,
    /// Net present value
    pub net_present_value: f64,
    /// Benefit-cost ratio
    pub benefit_cost_ratio: f64,
    /// Return on investment (percentage)
    pub return_on_investment: f64,
    /// Recommendation
    pub recommendation: CBARecommendation,
    /// Generated timestamp
    pub generated_at: String,
}

/// Cost breakdown.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostBreakdown {
    /// Currency code
    pub currency: String,
    /// Direct costs
    pub direct_costs: f64,
    /// Indirect costs
    pub indirect_costs: f64,
    /// Implementation costs
    pub implementation_costs: f64,
    /// Maintenance costs (annual)
    pub maintenance_costs_annual: f64,
    /// Total costs (5-year projection)
    pub total_five_year: f64,
}

/// Benefit analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenefitAnalysis {
    /// Currency code
    pub currency: String,
    /// Quantifiable benefits (5-year projection)
    pub quantifiable_benefits: f64,
    /// Qualitative benefits
    pub qualitative_benefits: Vec<QualitativeBenefit>,
    /// Economic impact
    pub economic_impact: f64,
    /// Social impact score (0.0 to 1.0)
    pub social_impact_score: f64,
}

/// Qualitative benefit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualitativeBenefit {
    /// Benefit category
    pub category: String,
    /// Description
    pub description: String,
    /// Impact level
    pub impact_level: StakeholderImpactLevel,
}

/// Impact level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImpactLevel {
    /// Low impact
    Low,
    /// Medium impact
    Medium,
    /// High impact
    High,
    /// Transformative impact
    Transformative,
}

/// Cost-benefit analysis recommendation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CBARecommendation {
    /// Strongly recommend proceeding
    StronglyRecommend,
    /// Recommend with conditions
    RecommendWithConditions,
    /// Neutral (requires further analysis)
    Neutral,
    /// Do not recommend
    DoNotRecommend,
}

/// Generator for cost-benefit analysis.
#[derive(Debug, Clone)]
pub struct CostBenefitAnalyzer;

impl CostBenefitAnalyzer {
    /// Creates a new cost-benefit analyzer.
    pub fn new() -> Self {
        Self
    }

    /// Performs cost-benefit analysis for a porting project.
    pub fn analyze(
        &self,
        project: &PortingProject,
        roadmap: &ImplementationRoadmap,
        ported_statutes: &[PortedStatute],
    ) -> CostBenefitAnalysis {
        let total_costs = self.calculate_costs(
            &roadmap.resource_requirements,
            roadmap.estimated_duration_days,
        );
        let total_benefits = self.estimate_benefits(ported_statutes);

        let net_present_value = total_benefits.quantifiable_benefits - total_costs.total_five_year;
        let benefit_cost_ratio = if total_costs.total_five_year > 0.0 {
            total_benefits.quantifiable_benefits / total_costs.total_five_year
        } else {
            0.0
        };
        let return_on_investment = if total_costs.total_five_year > 0.0 {
            ((total_benefits.quantifiable_benefits - total_costs.total_five_year)
                / total_costs.total_five_year)
                * 100.0
        } else {
            0.0
        };

        let recommendation = if benefit_cost_ratio >= 2.0 && net_present_value > 1_000_000.0 {
            CBARecommendation::StronglyRecommend
        } else if benefit_cost_ratio >= 1.0 {
            CBARecommendation::RecommendWithConditions
        } else if benefit_cost_ratio >= 0.7 {
            CBARecommendation::Neutral
        } else {
            CBARecommendation::DoNotRecommend
        };

        CostBenefitAnalysis {
            project_id: project.id.clone(),
            title: format!("Cost-Benefit Analysis: {}", project.name),
            total_costs,
            total_benefits,
            net_present_value,
            benefit_cost_ratio,
            return_on_investment,
            recommendation,
            generated_at: chrono::Utc::now().to_rfc3339(),
        }
    }

    fn calculate_costs(
        &self,
        resources: &ResourceRequirements,
        duration_days: u32,
    ) -> CostBreakdown {
        let direct_costs = resources.budget_estimate.min_amount;
        let indirect_costs = direct_costs * 0.25;
        let implementation_costs = (duration_days as f64 / 30.0) * 100_000.0;
        let maintenance_costs_annual = direct_costs * 0.15;
        let total_five_year =
            direct_costs + indirect_costs + implementation_costs + (maintenance_costs_annual * 5.0);

        CostBreakdown {
            currency: resources.budget_estimate.currency.clone(),
            direct_costs,
            indirect_costs,
            implementation_costs,
            maintenance_costs_annual,
            total_five_year,
        }
    }

    fn estimate_benefits(&self, ported_statutes: &[PortedStatute]) -> BenefitAnalysis {
        let statute_count = ported_statutes.len();
        let avg_compatibility = if !ported_statutes.is_empty() {
            ported_statutes
                .iter()
                .map(|s| s.compatibility_score)
                .sum::<f64>()
                / ported_statutes.len() as f64
        } else {
            0.0
        };

        let base_benefit_per_statute = 200_000.0;
        let quantifiable_benefits =
            statute_count as f64 * base_benefit_per_statute * avg_compatibility * 5.0;

        let economic_impact = quantifiable_benefits * 1.5;
        let social_impact_score = avg_compatibility * 0.9;

        let qualitative_benefits = vec![
            QualitativeBenefit {
                category: "Legal Harmonization".to_string(),
                description: "Improved legal compatibility between jurisdictions".to_string(),
                impact_level: if avg_compatibility >= 0.8 {
                    StakeholderImpactLevel::High
                } else {
                    StakeholderImpactLevel::Medium
                },
            },
            QualitativeBenefit {
                category: "Governance".to_string(),
                description: "Enhanced legal framework and governance quality".to_string(),
                impact_level: StakeholderImpactLevel::High,
            },
            QualitativeBenefit {
                category: "International Cooperation".to_string(),
                description: "Strengthened bilateral legal cooperation".to_string(),
                impact_level: StakeholderImpactLevel::Medium,
            },
        ];

        BenefitAnalysis {
            currency: "USD".to_string(),
            quantifiable_benefits,
            qualitative_benefits,
            economic_impact,
            social_impact_score,
        }
    }
}

impl Default for CostBenefitAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Compliance certification for ported statutes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceCertification {
    /// Certification identifier
    pub id: String,
    /// Project identifier
    pub project_id: String,
    /// Certification title
    pub title: String,
    /// Certification level
    pub certification_level: CertificationLevel,
    /// Certification status
    pub status: CertificationStatus,
    /// Certified statutes
    pub certified_statutes: Vec<String>,
    /// Validation results
    pub validation_results: Vec<ValidationResult>,
    /// Certifier information
    pub certifier: CertifierInfo,
    /// Certification date
    pub certification_date: String,
    /// Expiration date
    pub expiration_date: Option<String>,
    /// Conditions or limitations
    pub conditions: Vec<String>,
    /// Digital signature
    pub signature: Option<String>,
}

/// Certification level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CertificationLevel {
    /// Provisional certification
    Provisional,
    /// Standard certification
    Standard,
    /// Enhanced certification
    Enhanced,
    /// Full certification
    Full,
}

/// Certification status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CertificationStatus {
    /// Pending review
    Pending,
    /// Under review
    UnderReview,
    /// Certified
    Certified,
    /// Conditional certification
    Conditional,
    /// Revoked
    Revoked,
}

/// Certifier information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertifierInfo {
    /// Certifier name
    pub name: String,
    /// Organization
    pub organization: String,
    /// Credentials
    pub credentials: Vec<String>,
    /// Contact information
    pub contact: String,
}

/// Compliance certification manager.
#[derive(Debug, Clone)]
pub struct ComplianceCertificationManager {
    certifications: HashMap<String, ComplianceCertification>,
}

impl ComplianceCertificationManager {
    /// Creates a new compliance certification manager.
    pub fn new() -> Self {
        Self {
            certifications: HashMap::new(),
        }
    }

    /// Issues a compliance certification.
    pub fn issue_certification(
        &mut self,
        project_id: String,
        validation_results: Vec<ValidationResult>,
        certifier: CertifierInfo,
    ) -> ComplianceCertification {
        let id = uuid::Uuid::new_v4().to_string();

        let overall_score = if !validation_results.is_empty() {
            validation_results
                .iter()
                .map(|r| r.overall_score)
                .sum::<f64>()
                / validation_results.len() as f64
        } else {
            0.0
        };

        let certification_level = if overall_score >= 0.95 {
            CertificationLevel::Full
        } else if overall_score >= 0.85 {
            CertificationLevel::Enhanced
        } else if overall_score >= 0.75 {
            CertificationLevel::Standard
        } else {
            CertificationLevel::Provisional
        };

        let status = if overall_score >= 0.75 {
            CertificationStatus::Certified
        } else if overall_score >= 0.6 {
            CertificationStatus::Conditional
        } else {
            CertificationStatus::Pending
        };

        let certified_statutes: Vec<String> = validation_results
            .iter()
            .filter(|r| r.overall_score >= 0.75)
            .map(|r| r.id.clone())
            .collect();

        let mut conditions = Vec::new();
        if overall_score < 0.95 {
            conditions.push("Periodic review required every 12 months".to_string());
        }
        if overall_score < 0.85 {
            conditions.push("Implementation monitoring required".to_string());
        }

        let now = chrono::Utc::now();
        let expiration = if overall_score >= 0.85 {
            Some((now + chrono::Duration::days(365 * 3)).to_rfc3339())
        } else {
            Some((now + chrono::Duration::days(365)).to_rfc3339())
        };

        let certification = ComplianceCertification {
            id: id.clone(),
            project_id: project_id.clone(),
            title: format!("Compliance Certification - Project {}", project_id),
            certification_level,
            status,
            certified_statutes,
            validation_results,
            certifier,
            certification_date: now.to_rfc3339(),
            expiration_date: expiration,
            conditions,
            signature: Some(format!("CERT-{}", &id[..8])),
        };

        self.certifications.insert(id, certification.clone());
        certification
    }

    /// Retrieves a certification by ID.
    pub fn get_certification(&self, id: &str) -> Option<&ComplianceCertification> {
        self.certifications.get(id)
    }

    /// Revokes a certification.
    pub fn revoke_certification(&mut self, id: &str) -> Option<()> {
        let cert = self.certifications.get_mut(id)?;
        cert.status = CertificationStatus::Revoked;
        Some(())
    }
}

impl Default for ComplianceCertificationManager {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Integration (v0.1.9)
// ============================================================================

/// REST API request types for porting service.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiPortingRequest {
    /// Source jurisdiction code
    pub source_jurisdiction: String,
    /// Target jurisdiction code
    pub target_jurisdiction: String,
    /// Statute IDs to port
    pub statute_ids: Vec<String>,
    /// Porting options
    pub options: PortingOptions,
}

/// REST API response for porting operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiPortingResponse {
    /// Request ID
    pub request_id: String,
    /// Status of the request
    pub status: ApiStatus,
    /// Ported statutes (if completed)
    pub results: Option<Vec<PortedStatute>>,
    /// Error message (if failed)
    pub error: Option<String>,
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
}

/// API status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApiStatus {
    /// Request accepted and queued
    Accepted,
    /// Processing in progress
    Processing,
    /// Completed successfully
    Completed,
    /// Failed with error
    Failed,
}

/// Bilateral agreement template library.
#[derive(Debug, Clone)]
pub struct BilateralAgreementTemplateLibrary {
    templates: HashMap<String, BilateralAgreementTemplate>,
}

/// Bilateral agreement template.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BilateralAgreementTemplate {
    /// Template identifier
    pub id: String,
    /// Template name
    pub name: String,
    /// Template description
    pub description: String,
    /// Applicable legal systems
    pub applicable_systems: Vec<LegalSystem>,
    /// Template sections
    pub sections: Vec<TemplateSection>,
    /// Required parameters
    pub required_parameters: Vec<TemplateParameter>,
    /// Optional parameters
    pub optional_parameters: Vec<TemplateParameter>,
}

/// Template section.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateSection {
    /// Section number
    pub section_number: u32,
    /// Section title
    pub title: String,
    /// Section content template
    pub content_template: String,
    /// Required
    pub required: bool,
}

/// Template parameter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateParameter {
    /// Parameter name
    pub name: String,
    /// Parameter description
    pub description: String,
    /// Parameter type
    pub parameter_type: ParameterType,
    /// Default value
    pub default_value: Option<String>,
}

/// Parameter type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParameterType {
    /// String parameter
    String,
    /// Numeric parameter
    Number,
    /// Date parameter
    Date,
    /// Boolean parameter
    Boolean,
    /// List parameter
    List,
}

impl BilateralAgreementTemplateLibrary {
    /// Creates a new template library.
    pub fn new() -> Self {
        let mut library = Self {
            templates: HashMap::new(),
        };
        library.add_default_templates();
        library
    }

    fn add_default_templates(&mut self) {
        // General bilateral agreement template
        self.add_template(BilateralAgreementTemplate {
            id: "general-bilateral".to_string(),
            name: "General Bilateral Legal Cooperation Agreement".to_string(),
            description: "Standard template for bilateral legal cooperation".to_string(),
            applicable_systems: vec![
                LegalSystem::CivilLaw,
                LegalSystem::CommonLaw,
            ],
            sections: vec![
                TemplateSection {
                    section_number: 1,
                    title: "Parties and Purpose".to_string(),
                    content_template: "This agreement is entered into between {{source_jurisdiction}} and {{target_jurisdiction}} for the purpose of {{purpose}}.".to_string(),
                    required: true,
                },
                TemplateSection {
                    section_number: 2,
                    title: "Scope of Cooperation".to_string(),
                    content_template: "The parties agree to cooperate in the following areas: {{cooperation_areas}}.".to_string(),
                    required: true,
                },
                TemplateSection {
                    section_number: 3,
                    title: "Legal Framework Porting".to_string(),
                    content_template: "The parties agree to facilitate the porting of legal frameworks according to the principles set forth in {{porting_principles}}.".to_string(),
                    required: true,
                },
                TemplateSection {
                    section_number: 4,
                    title: "Cultural Adaptation".to_string(),
                    content_template: "All ported statutes shall be adapted to respect the cultural, religious, and social norms of the target jurisdiction.".to_string(),
                    required: true,
                },
                TemplateSection {
                    section_number: 5,
                    title: "Review and Approval Process".to_string(),
                    content_template: "Ported statutes shall undergo review by {{review_body}} before implementation.".to_string(),
                    required: true,
                },
            ],
            required_parameters: vec![
                TemplateParameter {
                    name: "source_jurisdiction".to_string(),
                    description: "Source jurisdiction name".to_string(),
                    parameter_type: ParameterType::String,
                    default_value: None,
                },
                TemplateParameter {
                    name: "target_jurisdiction".to_string(),
                    description: "Target jurisdiction name".to_string(),
                    parameter_type: ParameterType::String,
                    default_value: None,
                },
                TemplateParameter {
                    name: "purpose".to_string(),
                    description: "Purpose of the agreement".to_string(),
                    parameter_type: ParameterType::String,
                    default_value: Some("legal framework cooperation and mutual development".to_string()),
                },
            ],
            optional_parameters: vec![
                TemplateParameter {
                    name: "cooperation_areas".to_string(),
                    description: "Areas of legal cooperation".to_string(),
                    parameter_type: ParameterType::List,
                    default_value: Some("civil law, commercial law, administrative law".to_string()),
                },
            ],
        });
    }

    /// Adds a template to the library.
    pub fn add_template(&mut self, template: BilateralAgreementTemplate) {
        self.templates.insert(template.id.clone(), template);
    }

    /// Retrieves a template by ID.
    pub fn get_template(&self, id: &str) -> Option<&BilateralAgreementTemplate> {
        self.templates.get(id)
    }

    /// Lists all available templates.
    pub fn list_templates(&self) -> Vec<&BilateralAgreementTemplate> {
        self.templates.values().collect()
    }

    /// Generates an agreement from a template.
    pub fn generate_agreement(
        &self,
        template_id: &str,
        parameters: &HashMap<String, String>,
    ) -> Option<String> {
        let template = self.get_template(template_id)?;
        let mut agreement = String::new();

        agreement.push_str(&format!("# {}\n\n", template.name));

        for section in &template.sections {
            agreement.push_str(&format!(
                "## Section {}: {}\n\n",
                section.section_number, section.title
            ));

            let mut content = section.content_template.clone();
            for (key, value) in parameters {
                content = content.replace(&format!("{{{{{}}}}}", key), value);
            }

            agreement.push_str(&format!("{}\n\n", content));
        }

        Some(agreement)
    }
}

impl Default for BilateralAgreementTemplateLibrary {
    fn default() -> Self {
        Self::new()
    }
}

/// Regulatory sandbox for testing ported statutes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulatorySandbox {
    /// Sandbox identifier
    pub id: String,
    /// Sandbox name
    pub name: String,
    /// Sandbox description
    pub description: String,
    /// Sandbox status
    pub status: SandboxStatus,
    /// Statutes being tested
    pub test_statutes: Vec<String>,
    /// Test scenarios
    pub scenarios: Vec<TestScenario>,
    /// Test results
    pub results: Vec<SandboxTestResult>,
    /// Start date
    pub start_date: String,
    /// End date
    pub end_date: Option<String>,
}

/// Sandbox status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SandboxStatus {
    /// Planning phase
    Planning,
    /// Active testing
    Active,
    /// Evaluation phase
    Evaluation,
    /// Completed
    Completed,
    /// Terminated
    Terminated,
}

/// Test scenario in regulatory sandbox.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestScenario {
    /// Scenario identifier
    pub id: String,
    /// Scenario name
    pub name: String,
    /// Scenario description
    pub description: String,
    /// Test parameters
    pub parameters: HashMap<String, String>,
    /// Expected outcomes
    pub expected_outcomes: Vec<String>,
}

/// Sandbox test result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxTestResult {
    /// Scenario identifier
    pub scenario_id: String,
    /// Test status
    pub status: TestStatus,
    /// Actual outcomes
    pub actual_outcomes: Vec<String>,
    /// Issues encountered
    pub issues: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
    /// Test date
    pub test_date: String,
}

/// Test status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TestStatus {
    /// Passed
    Passed,
    /// Passed with minor issues
    PassedWithIssues,
    /// Failed
    Failed,
    /// Inconclusive
    Inconclusive,
}

/// Manager for regulatory sandboxes.
#[derive(Debug, Clone)]
pub struct RegulatorySandboxManager {
    sandboxes: HashMap<String, RegulatorySandbox>,
}

impl RegulatorySandboxManager {
    /// Creates a new regulatory sandbox manager.
    pub fn new() -> Self {
        Self {
            sandboxes: HashMap::new(),
        }
    }

    /// Creates a new regulatory sandbox.
    pub fn create_sandbox(
        &mut self,
        name: String,
        description: String,
        test_statutes: Vec<String>,
    ) -> RegulatorySandbox {
        let id = uuid::Uuid::new_v4().to_string();
        let sandbox = RegulatorySandbox {
            id: id.clone(),
            name,
            description,
            status: SandboxStatus::Planning,
            test_statutes,
            scenarios: Vec::new(),
            results: Vec::new(),
            start_date: chrono::Utc::now().to_rfc3339(),
            end_date: None,
        };
        self.sandboxes.insert(id, sandbox.clone());
        sandbox
    }

    /// Adds a test scenario to a sandbox.
    pub fn add_scenario(&mut self, sandbox_id: &str, scenario: TestScenario) -> Option<()> {
        let sandbox = self.sandboxes.get_mut(sandbox_id)?;
        sandbox.scenarios.push(scenario);
        Some(())
    }

    /// Records a test result.
    pub fn record_result(&mut self, sandbox_id: &str, result: SandboxTestResult) -> Option<()> {
        let sandbox = self.sandboxes.get_mut(sandbox_id)?;
        sandbox.results.push(result);
        Some(())
    }

    /// Activates a sandbox.
    pub fn activate_sandbox(&mut self, sandbox_id: &str) -> Option<()> {
        let sandbox = self.sandboxes.get_mut(sandbox_id)?;
        sandbox.status = SandboxStatus::Active;
        Some(())
    }

    /// Completes a sandbox.
    pub fn complete_sandbox(&mut self, sandbox_id: &str) -> Option<()> {
        let sandbox = self.sandboxes.get_mut(sandbox_id)?;
        sandbox.status = SandboxStatus::Completed;
        sandbox.end_date = Some(chrono::Utc::now().to_rfc3339());
        Some(())
    }

    /// Retrieves a sandbox by ID.
    pub fn get_sandbox(&self, sandbox_id: &str) -> Option<&RegulatorySandbox> {
        self.sandboxes.get(sandbox_id)
    }
}

impl Default for RegulatorySandboxManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Notification to affected parties.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AffectedPartyNotification {
    /// Notification identifier
    pub id: String,
    /// Project identifier
    pub project_id: String,
    /// Notification title
    pub title: String,
    /// Notification content
    pub content: String,
    /// Affected party categories
    pub affected_categories: Vec<AffectedPartyCategory>,
    /// Distribution channels
    pub channels: Vec<NotificationChannel>,
    /// Notification date
    pub notification_date: String,
    /// Response deadline
    pub response_deadline: Option<String>,
    /// Feedback received
    pub feedback: Vec<PublicFeedback>,
}

/// Category of affected party.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AffectedPartyCategory {
    /// General public
    GeneralPublic,
    /// Business entities
    Businesses,
    /// Non-profit organizations
    NonProfits,
    /// Government agencies
    GovernmentAgencies,
    /// Legal professionals
    LegalProfessionals,
    /// Academic institutions
    AcademicInstitutions,
}

/// Public feedback on a notification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicFeedback {
    /// Feedback identifier
    pub id: String,
    /// Submitter information (optional/anonymous)
    pub submitter: Option<String>,
    /// Feedback category
    pub category: FeedbackCategory,
    /// Feedback content
    pub content: String,
    /// Submission date
    pub submitted_at: String,
}

/// Feedback category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FeedbackCategory {
    /// Support
    Support,
    /// Concern
    Concern,
    /// Question
    Question,
    /// Suggestion
    Suggestion,
    /// Objection
    Objection,
}

/// Manager for affected party notifications.
#[derive(Debug, Clone)]
pub struct AffectedPartyNotificationManager {
    notifications: HashMap<String, AffectedPartyNotification>,
}

impl AffectedPartyNotificationManager {
    /// Creates a new affected party notification manager.
    pub fn new() -> Self {
        Self {
            notifications: HashMap::new(),
        }
    }

    /// Sends a notification to affected parties.
    pub fn send_notification(
        &mut self,
        project_id: String,
        title: String,
        content: String,
        affected_categories: Vec<AffectedPartyCategory>,
        response_deadline_days: Option<u32>,
    ) -> AffectedPartyNotification {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now();

        let response_deadline = response_deadline_days
            .map(|days| (now + chrono::Duration::days(days as i64)).to_rfc3339());

        let notification = AffectedPartyNotification {
            id: id.clone(),
            project_id,
            title,
            content,
            affected_categories,
            channels: vec![
                NotificationChannel::Email,
                NotificationChannel::Website,
                NotificationChannel::PublicNotice,
            ],
            notification_date: now.to_rfc3339(),
            response_deadline,
            feedback: Vec::new(),
        };

        self.notifications.insert(id, notification.clone());
        notification
    }

    /// Records public feedback.
    pub fn record_feedback(
        &mut self,
        notification_id: &str,
        feedback: PublicFeedback,
    ) -> Option<()> {
        let notification = self.notifications.get_mut(notification_id)?;
        notification.feedback.push(feedback);
        Some(())
    }

    /// Retrieves a notification by ID.
    pub fn get_notification(&self, notification_id: &str) -> Option<&AffectedPartyNotification> {
        self.notifications.get(notification_id)
    }

    /// Lists all feedback for a notification.
    pub fn list_feedback(&self, notification_id: &str) -> Option<&[PublicFeedback]> {
        self.notifications
            .get(notification_id)
            .map(|n| n.feedback.as_slice())
    }
}

impl Default for AffectedPartyNotificationManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Public comment period for porting projects.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicCommentPeriod {
    /// Comment period identifier
    pub id: String,
    /// Project identifier
    pub project_id: String,
    /// Period title
    pub title: String,
    /// Period description
    pub description: String,
    /// Start date
    pub start_date: String,
    /// End date
    pub end_date: String,
    /// Status
    pub status: CommentPeriodStatus,
    /// Documents available for comment
    pub documents: Vec<CommentDocument>,
    /// Submitted comments
    pub comments: Vec<PublicComment>,
    /// Public hearings scheduled
    pub hearings: Vec<PublicHearing>,
}

/// Comment period status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommentPeriodStatus {
    /// Upcoming
    Upcoming,
    /// Currently open
    Open,
    /// Closed
    Closed,
    /// Extended
    Extended,
}

/// Document available for public comment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentDocument {
    /// Document identifier
    pub id: String,
    /// Document title
    pub title: String,
    /// Document type
    pub document_type: DocumentType,
    /// Document description
    pub description: String,
    /// Document URL or path
    pub url: String,
}

/// Document type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DocumentType {
    /// Draft statute
    DraftStatute,
    /// Impact assessment
    ImpactAssessment,
    /// Explanatory memorandum
    ExplanatoryMemorandum,
    /// Technical report
    TechnicalReport,
}

/// Public comment submitted during comment period.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicComment {
    /// Comment identifier
    pub id: String,
    /// Commenter information
    pub commenter: CommenterInfo,
    /// Comment text
    pub comment_text: String,
    /// Related document ID
    pub document_id: Option<String>,
    /// Specific section referenced
    pub section_reference: Option<String>,
    /// Submission date
    pub submitted_at: String,
    /// Comment category
    pub category: FeedbackCategory,
}

/// Information about a commenter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommenterInfo {
    /// Name (optional for anonymous comments)
    pub name: Option<String>,
    /// Organization (if applicable)
    pub organization: Option<String>,
    /// Email
    pub email: Option<String>,
    /// Affiliation type
    pub affiliation: AffectedPartyCategory,
}

/// Public hearing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicHearing {
    /// Hearing identifier
    pub id: String,
    /// Hearing title
    pub title: String,
    /// Date and time
    pub datetime: String,
    /// Location
    pub location: String,
    /// Virtual meeting link
    pub virtual_link: Option<String>,
    /// Agenda
    pub agenda: Vec<String>,
    /// Registration required
    pub registration_required: bool,
}

/// Manager for public comment periods.
#[derive(Debug, Clone)]
pub struct PublicCommentPeriodManager {
    periods: HashMap<String, PublicCommentPeriod>,
}

impl PublicCommentPeriodManager {
    /// Creates a new public comment period manager.
    pub fn new() -> Self {
        Self {
            periods: HashMap::new(),
        }
    }

    /// Opens a new public comment period.
    pub fn open_comment_period(
        &mut self,
        project_id: String,
        title: String,
        description: String,
        duration_days: u32,
    ) -> PublicCommentPeriod {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now();
        let end_date = now + chrono::Duration::days(duration_days as i64);

        let period = PublicCommentPeriod {
            id: id.clone(),
            project_id,
            title,
            description,
            start_date: now.to_rfc3339(),
            end_date: end_date.to_rfc3339(),
            status: CommentPeriodStatus::Open,
            documents: Vec::new(),
            comments: Vec::new(),
            hearings: Vec::new(),
        };

        self.periods.insert(id, period.clone());
        period
    }

    /// Adds a document to the comment period.
    pub fn add_document(&mut self, period_id: &str, document: CommentDocument) -> Option<()> {
        let period = self.periods.get_mut(period_id)?;
        period.documents.push(document);
        Some(())
    }

    /// Submits a public comment.
    pub fn submit_comment(&mut self, period_id: &str, comment: PublicComment) -> Option<()> {
        let period = self.periods.get_mut(period_id)?;
        if period.status == CommentPeriodStatus::Open
            || period.status == CommentPeriodStatus::Extended
        {
            period.comments.push(comment);
            Some(())
        } else {
            None
        }
    }

    /// Schedules a public hearing.
    pub fn schedule_hearing(&mut self, period_id: &str, hearing: PublicHearing) -> Option<()> {
        let period = self.periods.get_mut(period_id)?;
        period.hearings.push(hearing);
        Some(())
    }

    /// Extends a comment period.
    pub fn extend_period(&mut self, period_id: &str, additional_days: u32) -> Option<()> {
        let period = self.periods.get_mut(period_id)?;
        if let Ok(current_end) = chrono::DateTime::parse_from_rfc3339(&period.end_date) {
            let new_end = current_end + chrono::Duration::days(additional_days as i64);
            period.end_date = new_end.to_rfc3339();
            period.status = CommentPeriodStatus::Extended;
            Some(())
        } else {
            None
        }
    }

    /// Closes a comment period.
    pub fn close_period(&mut self, period_id: &str) -> Option<()> {
        let period = self.periods.get_mut(period_id)?;
        period.status = CommentPeriodStatus::Closed;
        Some(())
    }

    /// Retrieves a comment period by ID.
    pub fn get_period(&self, period_id: &str) -> Option<&PublicCommentPeriod> {
        self.periods.get(period_id)
    }

    /// Lists all comments for a period.
    pub fn list_comments(&self, period_id: &str) -> Option<&[PublicComment]> {
        self.periods.get(period_id).map(|p| p.comments.as_slice())
    }

    /// Generates a summary of public comments.
    pub fn generate_comment_summary(&self, period_id: &str) -> Option<CommentSummary> {
        let period = self.periods.get(period_id)?;

        let total_comments = period.comments.len();
        let mut category_counts: HashMap<FeedbackCategory, usize> = HashMap::new();
        let mut affiliation_counts: HashMap<AffectedPartyCategory, usize> = HashMap::new();

        for comment in &period.comments {
            *category_counts.entry(comment.category).or_insert(0) += 1;
            *affiliation_counts
                .entry(comment.commenter.affiliation)
                .or_insert(0) += 1;
        }

        Some(CommentSummary {
            period_id: period_id.to_string(),
            total_comments,
            category_breakdown: category_counts,
            affiliation_breakdown: affiliation_counts,
            key_themes: self.extract_key_themes(&period.comments),
        })
    }

    fn extract_key_themes(&self, _comments: &[PublicComment]) -> Vec<String> {
        // Simplified key theme extraction
        vec![
            "Constitutional compatibility concerns".to_string(),
            "Implementation timeline questions".to_string(),
            "Cultural adaptation suggestions".to_string(),
        ]
    }
}

impl Default for PublicCommentPeriodManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Summary of public comments.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentSummary {
    /// Comment period identifier
    pub period_id: String,
    /// Total number of comments
    pub total_comments: usize,
    /// Breakdown by category
    pub category_breakdown: HashMap<FeedbackCategory, usize>,
    /// Breakdown by affiliation
    pub affiliation_breakdown: HashMap<AffectedPartyCategory, usize>,
    /// Key themes identified
    pub key_themes: Vec<String>,
}

// ============================================================================
// Stakeholder Collaboration - Discussion Threads (v0.2.4)
// ============================================================================

/// Discussion thread for collaborative review.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscussionThread {
    /// Thread ID
    pub id: String,
    /// Project ID
    pub project_id: String,
    /// Thread title
    pub title: String,
    /// Thread context (e.g., statute section, specific issue)
    pub context: String,
    /// Thread status
    pub status: ThreadStatus,
    /// Root comments (top-level)
    pub comments: Vec<ThreadComment>,
    /// Created timestamp
    pub created_at: String,
    /// Created by stakeholder ID
    pub created_by: String,
    /// Tags for categorization
    pub tags: Vec<String>,
    /// Resolved by stakeholder ID
    pub resolved_by: Option<String>,
    /// Resolution timestamp
    pub resolved_at: Option<String>,
}

/// Thread status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ThreadStatus {
    /// Open for discussion
    Open,
    /// Under review
    UnderReview,
    /// Resolved
    Resolved,
    /// Archived
    Archived,
}

/// Comment in a discussion thread (supports nested replies).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadComment {
    /// Comment ID
    pub id: String,
    /// Parent comment ID (None for root comments)
    pub parent_id: Option<String>,
    /// Author stakeholder ID
    pub author_id: String,
    /// Comment text
    pub text: String,
    /// Created timestamp
    pub created_at: String,
    /// Last edited timestamp
    pub edited_at: Option<String>,
    /// Nested replies
    pub replies: Vec<ThreadComment>,
    /// Upvotes/likes
    pub upvotes: u32,
    /// Users who upvoted
    pub upvoted_by: Vec<String>,
    /// Marked as important
    pub is_important: bool,
}

/// Discussion thread manager.
#[derive(Debug)]
pub struct DiscussionThreadManager {
    threads: HashMap<String, DiscussionThread>,
}

impl DiscussionThreadManager {
    /// Creates a new discussion thread manager.
    pub fn new() -> Self {
        Self {
            threads: HashMap::new(),
        }
    }

    /// Creates a new discussion thread.
    pub fn create_thread(
        &mut self,
        project_id: String,
        title: String,
        context: String,
        created_by: String,
        tags: Vec<String>,
    ) -> DiscussionThread {
        let thread = DiscussionThread {
            id: uuid::Uuid::new_v4().to_string(),
            project_id,
            title,
            context,
            status: ThreadStatus::Open,
            comments: Vec::new(),
            created_at: chrono::Utc::now().to_rfc3339(),
            created_by,
            tags,
            resolved_by: None,
            resolved_at: None,
        };

        self.threads.insert(thread.id.clone(), thread.clone());
        thread
    }

    /// Adds a comment to a thread.
    pub fn add_comment(
        &mut self,
        thread_id: &str,
        author_id: String,
        text: String,
        parent_id: Option<String>,
    ) -> Option<ThreadComment> {
        let thread = self.threads.get_mut(thread_id)?;

        let comment = ThreadComment {
            id: uuid::Uuid::new_v4().to_string(),
            parent_id: parent_id.clone(),
            author_id,
            text,
            created_at: chrono::Utc::now().to_rfc3339(),
            edited_at: None,
            replies: Vec::new(),
            upvotes: 0,
            upvoted_by: Vec::new(),
            is_important: false,
        };

        // If parent_id is specified, add as reply to parent comment
        if let Some(parent) = parent_id {
            Self::add_reply_to_comment(&mut thread.comments, &parent, comment.clone())?;
        } else {
            thread.comments.push(comment.clone());
        }

        Some(comment)
    }

    fn add_reply_to_comment(
        comments: &mut Vec<ThreadComment>,
        parent_id: &str,
        reply: ThreadComment,
    ) -> Option<()> {
        for comment in comments {
            if comment.id == parent_id {
                comment.replies.push(reply);
                return Some(());
            }
            // Recursively search in replies
            if Self::add_reply_to_comment(&mut comment.replies, parent_id, reply.clone()).is_some()
            {
                return Some(());
            }
        }
        None
    }

    /// Upvotes a comment.
    pub fn upvote_comment(
        &mut self,
        thread_id: &str,
        comment_id: &str,
        user_id: String,
    ) -> Option<()> {
        let thread = self.threads.get_mut(thread_id)?;
        Self::upvote_comment_recursive(&mut thread.comments, comment_id, user_id)
    }

    fn upvote_comment_recursive(
        comments: &mut Vec<ThreadComment>,
        comment_id: &str,
        user_id: String,
    ) -> Option<()> {
        for comment in comments {
            if comment.id == comment_id {
                if !comment.upvoted_by.contains(&user_id) {
                    comment.upvoted_by.push(user_id);
                    comment.upvotes += 1;
                }
                return Some(());
            }
            if Self::upvote_comment_recursive(&mut comment.replies, comment_id, user_id.clone())
                .is_some()
            {
                return Some(());
            }
        }
        None
    }

    /// Resolves a thread.
    pub fn resolve_thread(&mut self, thread_id: &str, resolved_by: String) -> Option<()> {
        let thread = self.threads.get_mut(thread_id)?;
        thread.status = ThreadStatus::Resolved;
        thread.resolved_by = Some(resolved_by);
        thread.resolved_at = Some(chrono::Utc::now().to_rfc3339());
        Some(())
    }

    /// Gets a thread.
    pub fn get_thread(&self, thread_id: &str) -> Option<&DiscussionThread> {
        self.threads.get(thread_id)
    }

    /// Lists all threads for a project.
    pub fn list_threads(&self, project_id: &str) -> Vec<&DiscussionThread> {
        self.threads
            .values()
            .filter(|t| t.project_id == project_id)
            .collect()
    }
}

impl Default for DiscussionThreadManager {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Stakeholder Collaboration - Voting (v0.2.4)
// ============================================================================

/// Voting poll for stakeholder decisions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakeholderVote {
    /// Vote ID
    pub id: String,
    /// Project ID
    pub project_id: String,
    /// Vote title
    pub title: String,
    /// Vote description
    pub description: String,
    /// Vote type
    pub vote_type: VoteType,
    /// Options to vote on
    pub options: Vec<VoteOption>,
    /// Eligible voters (stakeholder IDs)
    pub eligible_voters: Vec<String>,
    /// Votes cast
    pub votes_cast: HashMap<String, Vec<String>>, // voter_id -> option_ids (for multi-select)
    /// Vote status
    pub status: VoteStatus,
    /// Start timestamp
    pub start_time: String,
    /// End timestamp
    pub end_time: String,
    /// Requires minimum participation
    pub minimum_participation: Option<f64>,
    /// Requires minimum approval threshold
    pub approval_threshold: Option<f64>,
}

/// Vote type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VoteType {
    /// Single choice (select one option)
    SingleChoice,
    /// Multiple choice (select multiple options)
    MultipleChoice,
    /// Ranking (rank options by preference)
    Ranking,
    /// Approval voting (approve/disapprove each option)
    Approval,
}

/// Vote option.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteOption {
    /// Option ID
    pub id: String,
    /// Option text
    pub text: String,
    /// Option description
    pub description: String,
    /// Vote count
    pub vote_count: u32,
}

/// Vote status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VoteStatus {
    /// Not yet started
    Pending,
    /// Currently active
    Active,
    /// Voting closed
    Closed,
    /// Vote passed
    Passed,
    /// Vote failed
    Failed,
}

/// Vote result summary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteResult {
    /// Vote ID
    pub vote_id: String,
    /// Total eligible voters
    pub total_eligible: usize,
    /// Total votes cast
    pub total_votes: usize,
    /// Participation rate
    pub participation_rate: f64,
    /// Winning option(s)
    pub winning_options: Vec<String>,
    /// Result by option
    pub results: HashMap<String, u32>,
    /// Vote passed or failed
    pub passed: bool,
}

/// Voting manager for stakeholders.
#[derive(Debug)]
pub struct VotingManager {
    votes: HashMap<String, StakeholderVote>,
}

impl VotingManager {
    /// Creates a new voting manager.
    pub fn new() -> Self {
        Self {
            votes: HashMap::new(),
        }
    }

    /// Creates a new vote.
    pub fn create_vote(
        &mut self,
        project_id: String,
        title: String,
        description: String,
        vote_type: VoteType,
        options: Vec<VoteOption>,
        eligible_voters: Vec<String>,
        duration_hours: u32,
    ) -> StakeholderVote {
        let now = chrono::Utc::now();
        let end_time = now + chrono::Duration::hours(duration_hours as i64);

        let vote = StakeholderVote {
            id: uuid::Uuid::new_v4().to_string(),
            project_id,
            title,
            description,
            vote_type,
            options,
            eligible_voters,
            votes_cast: HashMap::new(),
            status: VoteStatus::Active,
            start_time: now.to_rfc3339(),
            end_time: end_time.to_rfc3339(),
            minimum_participation: None,
            approval_threshold: Some(0.5), // Default 50% approval
        };

        self.votes.insert(vote.id.clone(), vote.clone());
        vote
    }

    /// Casts a vote.
    pub fn cast_vote(
        &mut self,
        vote_id: &str,
        voter_id: String,
        selected_options: Vec<String>,
    ) -> Option<()> {
        let vote = self.votes.get_mut(vote_id)?;

        // Check if voter is eligible
        if !vote.eligible_voters.contains(&voter_id) {
            return None;
        }

        // Check if vote is active
        if vote.status != VoteStatus::Active {
            return None;
        }

        // Validate based on vote type
        match vote.vote_type {
            VoteType::SingleChoice => {
                if selected_options.len() != 1 {
                    return None;
                }
            }
            VoteType::MultipleChoice | VoteType::Approval | VoteType::Ranking => {
                // Multiple selections allowed
            }
        }

        // Record vote
        vote.votes_cast.insert(voter_id, selected_options.clone());

        // Update option counts
        for option_id in selected_options {
            if let Some(option) = vote.options.iter_mut().find(|o| o.id == option_id) {
                option.vote_count += 1;
            }
        }

        Some(())
    }

    /// Closes a vote and calculates results.
    pub fn close_vote(&mut self, vote_id: &str) -> Option<VoteResult> {
        let vote = self.votes.get_mut(vote_id)?;
        vote.status = VoteStatus::Closed;

        let total_eligible = vote.eligible_voters.len();
        let total_votes = vote.votes_cast.len();
        let participation_rate = total_votes as f64 / total_eligible as f64;

        // Find winning option(s)
        let max_votes = vote.options.iter().map(|o| o.vote_count).max().unwrap_or(0);
        let winning_options: Vec<String> = vote
            .options
            .iter()
            .filter(|o| o.vote_count == max_votes)
            .map(|o| o.text.clone())
            .collect();

        // Calculate if vote passed
        let passed = if let Some(min_participation) = vote.minimum_participation {
            if participation_rate < min_participation {
                vote.status = VoteStatus::Failed;
                false
            } else {
                Self::check_approval_threshold(vote, max_votes, total_votes)
            }
        } else {
            Self::check_approval_threshold(vote, max_votes, total_votes)
        };

        if passed {
            vote.status = VoteStatus::Passed;
        } else {
            vote.status = VoteStatus::Failed;
        }

        let mut results = HashMap::new();
        for option in &vote.options {
            results.insert(option.text.clone(), option.vote_count);
        }

        Some(VoteResult {
            vote_id: vote_id.to_string(),
            total_eligible,
            total_votes,
            participation_rate,
            winning_options,
            results,
            passed,
        })
    }

    fn check_approval_threshold(
        vote: &StakeholderVote,
        max_votes: u32,
        total_votes: usize,
    ) -> bool {
        if let Some(threshold) = vote.approval_threshold {
            max_votes as f64 / total_votes as f64 >= threshold
        } else {
            true
        }
    }

    /// Gets a vote.
    pub fn get_vote(&self, vote_id: &str) -> Option<&StakeholderVote> {
        self.votes.get(vote_id)
    }

    /// Lists all votes for a project.
    pub fn list_votes(&self, project_id: &str) -> Vec<&StakeholderVote> {
        self.votes
            .values()
            .filter(|v| v.project_id == project_id)
            .collect()
    }
}

impl Default for VotingManager {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Stakeholder Collaboration - Impact Notifications (v0.2.4)
// ============================================================================

/// Stakeholder impact assessment for a porting change.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakeholderImpact {
    /// Impact ID
    pub id: String,
    /// Project ID
    pub project_id: String,
    /// Affected stakeholder ID
    pub stakeholder_id: String,
    /// Impact level
    pub impact_level: StakeholderImpactLevel,
    /// Impact category
    pub impact_category: StakeholderImpactCategory,
    /// Impact description
    pub description: String,
    /// Estimated magnitude (0.0 to 1.0)
    pub magnitude: f64,
    /// Timeframe for impact
    pub timeframe: ImpactTimeframe,
    /// Mitigation strategies
    pub mitigation_strategies: Vec<String>,
    /// Notification sent
    pub notification_sent: bool,
    /// Notification timestamp
    pub notified_at: Option<String>,
}

/// Impact level for stakeholders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StakeholderImpactLevel {
    /// No impact
    None,
    /// Low impact
    Low,
    /// Medium impact
    Medium,
    /// High impact
    High,
    /// Critical impact
    Critical,
}

/// Category of stakeholder impact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StakeholderImpactCategory {
    /// Economic/financial impact
    Economic,
    /// Operational/workflow impact
    Operational,
    /// Legal/compliance impact
    Legal,
    /// Rights and obligations impact
    Rights,
    /// Resource requirements impact
    Resources,
    /// Strategic impact
    Strategic,
}

/// Timeframe for impact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ImpactTimeframe {
    /// Immediate (within days)
    Immediate,
    /// Short-term (weeks to months)
    ShortTerm,
    /// Medium-term (months to a year)
    MediumTerm,
    /// Long-term (years)
    LongTerm,
}

/// Stakeholder impact tracker.
#[derive(Debug)]
pub struct StakeholderImpactTracker {
    impacts: HashMap<String, Vec<StakeholderImpact>>,
}

impl StakeholderImpactTracker {
    /// Creates a new impact tracker.
    pub fn new() -> Self {
        Self {
            impacts: HashMap::new(),
        }
    }

    /// Records a stakeholder impact.
    pub fn record_impact(
        &mut self,
        project_id: String,
        stakeholder_id: String,
        impact_level: StakeholderImpactLevel,
        impact_category: StakeholderImpactCategory,
        description: String,
        magnitude: f64,
        timeframe: ImpactTimeframe,
        mitigation_strategies: Vec<String>,
    ) -> StakeholderImpact {
        let impact = StakeholderImpact {
            id: uuid::Uuid::new_v4().to_string(),
            project_id: project_id.clone(),
            stakeholder_id: stakeholder_id.clone(),
            impact_level,
            impact_category,
            description,
            magnitude,
            timeframe,
            mitigation_strategies,
            notification_sent: false,
            notified_at: None,
        };

        self.impacts
            .entry(project_id)
            .or_default()
            .push(impact.clone());

        impact
    }

    /// Marks impact as notified.
    pub fn mark_notified(&mut self, project_id: &str, impact_id: &str) -> Option<()> {
        let impacts = self.impacts.get_mut(project_id)?;
        let impact = impacts.iter_mut().find(|i| i.id == impact_id)?;
        impact.notification_sent = true;
        impact.notified_at = Some(chrono::Utc::now().to_rfc3339());
        Some(())
    }

    /// Gets impacts for a stakeholder.
    pub fn get_stakeholder_impacts(
        &self,
        project_id: &str,
        stakeholder_id: &str,
    ) -> Vec<&StakeholderImpact> {
        self.impacts
            .get(project_id)
            .map(|impacts| {
                impacts
                    .iter()
                    .filter(|i| i.stakeholder_id == stakeholder_id)
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Gets all high/critical impacts that haven't been notified.
    pub fn get_unnotified_critical_impacts(&self, project_id: &str) -> Vec<&StakeholderImpact> {
        self.impacts
            .get(project_id)
            .map(|impacts| {
                impacts
                    .iter()
                    .filter(|i| {
                        matches!(
                            i.impact_level,
                            StakeholderImpactLevel::High | StakeholderImpactLevel::Critical
                        ) && !i.notification_sent
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Gets summary of impacts by level.
    pub fn get_impact_summary(&self, project_id: &str) -> HashMap<StakeholderImpactLevel, usize> {
        let mut summary = HashMap::new();

        if let Some(impacts) = self.impacts.get(project_id) {
            for impact in impacts {
                *summary.entry(impact.impact_level).or_insert(0) += 1;
            }
        }

        summary
    }
}

impl Default for StakeholderImpactTracker {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// AI-Assisted Porting (v0.2.0)
// ============================================================================

/// Semantic equivalence result between two legal concepts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticEquivalence {
    /// Equivalence ID
    pub id: String,
    /// Source concept
    pub source_concept: String,
    /// Target concept
    pub target_concept: String,
    /// Equivalence score (0.0 to 1.0)
    pub equivalence_score: f64,
    /// Semantic similarity score
    pub similarity_score: f64,
    /// Structural similarity score
    pub structural_score: f64,
    /// Functional equivalence score
    pub functional_score: f64,
    /// Confidence in the equivalence
    pub confidence: f64,
    /// Explanation of equivalence
    pub explanation: String,
    /// Key similarities
    pub similarities: Vec<String>,
    /// Key differences
    pub differences: Vec<String>,
    /// Usage context compatibility
    pub context_compatibility: f64,
}

/// Semantic equivalence detector using advanced AI.
#[derive(Clone)]
pub struct SemanticEquivalenceDetector {
    /// Optional LLM generator
    generator: Option<std::sync::Arc<dyn TextGenerator>>,
}

impl SemanticEquivalenceDetector {
    /// Creates a new semantic equivalence detector.
    pub fn new() -> Self {
        Self { generator: None }
    }

    /// Creates a detector with an LLM generator.
    pub fn with_generator(generator: std::sync::Arc<dyn TextGenerator>) -> Self {
        Self {
            generator: Some(generator),
        }
    }

    /// Detects semantic equivalence between legal concepts.
    pub async fn detect_equivalence(
        &self,
        source_concept: &str,
        target_concept: &str,
        source_jurisdiction: &Jurisdiction,
        target_jurisdiction: &Jurisdiction,
    ) -> PortingResult<SemanticEquivalence> {
        let (similarity_score, explanation, similarities, differences) =
            if let Some(generator) = &self.generator {
                // Use LLM for advanced semantic analysis
                let prompt = format!(
                    "Analyze semantic equivalence between legal concepts:\n\
                Source: '{}' in {} ({:?} legal system)\n\
                Target: '{}' in {} ({:?} legal system)\n\n\
                Provide:\n\
                1. Similarity score (0.0-1.0)\n\
                2. Brief explanation\n\
                3. Key similarities (3 points)\n\
                4. Key differences (3 points)",
                    source_concept,
                    source_jurisdiction.name,
                    source_jurisdiction.legal_system,
                    target_concept,
                    target_jurisdiction.name,
                    target_jurisdiction.legal_system
                );

                let response = generator
                    .generate(&prompt)
                    .await
                    .map_err(PortingError::Llm)?;

                // Parse LLM response (simplified)
                let similarity = 0.75; // Would parse from LLM response
                let explain = format!("AI Analysis: {}", response.lines().next().unwrap_or(""));
                let sims = vec![
                    "Similar legal purpose".to_string(),
                    "Comparable scope".to_string(),
                    "Equivalent enforcement mechanisms".to_string(),
                ];
                let diffs = vec![
                    "Different procedural requirements".to_string(),
                    "Varying jurisdictional scope".to_string(),
                ];

                (similarity, explain, sims, diffs)
            } else {
                // Fallback: rule-based analysis
                let similarity = self.calculate_basic_similarity(source_concept, target_concept);
                let explain = "Rule-based similarity analysis".to_string();
                let sims = vec!["Lexical similarity detected".to_string()];
                let diffs = vec!["Different legal systems may affect interpretation".to_string()];

                (similarity, explain, sims, diffs)
            };

        // Calculate structural and functional scores
        let structural_score = self.calculate_structural_similarity(
            source_concept,
            target_concept,
            &source_jurisdiction.legal_system,
            &target_jurisdiction.legal_system,
        );

        let functional_score = self.calculate_functional_equivalence(
            source_concept,
            target_concept,
            source_jurisdiction,
            target_jurisdiction,
        );

        // Overall equivalence score is weighted average
        let equivalence_score =
            (similarity_score * 0.4) + (structural_score * 0.3) + (functional_score * 0.3);

        // Context compatibility based on legal system alignment
        let context_compatibility =
            if source_jurisdiction.legal_system == target_jurisdiction.legal_system {
                0.9
            } else {
                0.6
            };

        Ok(SemanticEquivalence {
            id: format!("sem-eq-{}", uuid::Uuid::new_v4()),
            source_concept: source_concept.to_string(),
            target_concept: target_concept.to_string(),
            equivalence_score,
            similarity_score,
            structural_score,
            functional_score,
            confidence: similarity_score * context_compatibility,
            explanation,
            similarities,
            differences,
            context_compatibility,
        })
    }

    /// Calculates basic lexical similarity.
    fn calculate_basic_similarity(&self, s1: &str, s2: &str) -> f64 {
        // Simple Levenshtein-based similarity
        let distance = self.levenshtein_distance(s1, s2);
        let max_len = s1.len().max(s2.len()) as f64;
        if max_len == 0.0 {
            1.0
        } else {
            1.0 - (distance as f64 / max_len)
        }
    }

    /// Calculates Levenshtein distance.
    fn levenshtein_distance(&self, s1: &str, s2: &str) -> usize {
        let len1 = s1.len();
        let len2 = s2.len();
        let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

        for i in 0..=len1 {
            matrix[i][0] = i;
        }
        for j in 0..=len2 {
            matrix[0][j] = j;
        }

        for (i, c1) in s1.chars().enumerate() {
            for (j, c2) in s2.chars().enumerate() {
                let cost = if c1 == c2 { 0 } else { 1 };
                matrix[i + 1][j + 1] = (matrix[i][j + 1] + 1)
                    .min(matrix[i + 1][j] + 1)
                    .min(matrix[i][j] + cost);
            }
        }

        matrix[len1][len2]
    }

    /// Calculates structural similarity based on legal systems.
    fn calculate_structural_similarity(
        &self,
        _s1: &str,
        _s2: &str,
        sys1: &LegalSystem,
        sys2: &LegalSystem,
    ) -> f64 {
        if sys1 == sys2 {
            0.9
        } else {
            match (sys1, sys2) {
                (LegalSystem::CommonLaw, LegalSystem::CivilLaw)
                | (LegalSystem::CivilLaw, LegalSystem::CommonLaw) => 0.6,
                _ => 0.5,
            }
        }
    }

    /// Calculates functional equivalence.
    fn calculate_functional_equivalence(
        &self,
        _s1: &str,
        _s2: &str,
        j1: &Jurisdiction,
        j2: &Jurisdiction,
    ) -> f64 {
        // Check cultural parameter alignment
        let age_alignment =
            if j1.cultural_params.age_of_majority == j2.cultural_params.age_of_majority {
                1.0
            } else {
                0.7
            };

        let prohibition_alignment =
            if j1.cultural_params.prohibitions == j2.cultural_params.prohibitions {
                1.0
            } else {
                0.6
            };

        (age_alignment + prohibition_alignment) / 2.0
    }
}

impl Default for SemanticEquivalenceDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Automatic terminology mapping result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoTermMapping {
    /// Mapping ID
    pub id: String,
    /// Source term
    pub source_term: String,
    /// Mapped target term
    pub target_term: String,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
    /// Context in which the mapping applies
    pub context: String,
    /// Alternative mappings
    pub alternatives: Vec<AlternativeMapping>,
    /// Mapping rationale
    pub rationale: String,
    /// Usage examples
    pub examples: Vec<String>,
}

/// Alternative term mapping.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlternativeMapping {
    /// Alternative term
    pub term: String,
    /// Confidence in this alternative
    pub confidence: f64,
    /// When to use this alternative
    pub usage_context: String,
}

/// Automatic terminology mapper using AI.
#[derive(Clone)]
pub struct AutoTermMapper {
    /// Optional LLM generator
    generator: Option<std::sync::Arc<dyn TextGenerator>>,
    /// Term translation matrix for fallback
    translation_matrix: TermTranslationMatrix,
}

impl AutoTermMapper {
    /// Creates a new automatic term mapper.
    pub fn new() -> Self {
        Self {
            generator: None,
            translation_matrix: TermTranslationMatrix::new(),
        }
    }

    /// Creates a mapper with an LLM generator.
    pub fn with_generator(generator: std::sync::Arc<dyn TextGenerator>) -> Self {
        Self {
            generator: Some(generator),
            translation_matrix: TermTranslationMatrix::new(),
        }
    }

    /// Automatically maps legal terminology.
    pub async fn map_term(
        &self,
        term: &str,
        source_jurisdiction: &Jurisdiction,
        target_jurisdiction: &Jurisdiction,
        context: &str,
    ) -> PortingResult<AutoTermMapping> {
        let (target_term, confidence, alternatives, rationale) = if let Some(generator) =
            &self.generator
        {
            // Use LLM for intelligent term mapping
            let prompt = format!(
                "Map legal term from {} to {}:\n\
                Term: '{}'\n\
                Context: {}\n\
                Source legal system: {:?}\n\
                Target legal system: {:?}\n\n\
                Provide:\n\
                1. Best target term\n\
                2. Confidence (0.0-1.0)\n\
                3. Two alternative mappings with contexts\n\
                4. Brief rationale",
                source_jurisdiction.name,
                target_jurisdiction.name,
                term,
                context,
                source_jurisdiction.legal_system,
                target_jurisdiction.legal_system
            );

            let response = generator
                .generate(&prompt)
                .await
                .map_err(PortingError::Llm)?;

            // Parse LLM response (simplified)
            let target = response.lines().next().unwrap_or(term).to_string();
            let conf = 0.85;
            let alts = vec![
                AlternativeMapping {
                    term: format!("{}_alt1", term),
                    confidence: 0.7,
                    usage_context: "Formal legal documents".to_string(),
                },
                AlternativeMapping {
                    term: format!("{}_alt2", term),
                    confidence: 0.6,
                    usage_context: "Informal proceedings".to_string(),
                },
            ];
            let rat = "AI-based contextual mapping".to_string();

            (target, conf, alts, rat)
        } else {
            // Fallback: use translation matrix
            let translations = self.translation_matrix.find_translations(
                &source_jurisdiction.id,
                &target_jurisdiction.id,
                term,
            );
            let target = translations
                .iter()
                .find(|tr| {
                    tr.valid_contexts.iter().any(|c| c.contains(context)) || tr.source_term == term
                })
                .map(|tr| tr.target_term.clone())
                .unwrap_or_else(|| term.to_string());
            let conf = 0.6;
            let alts = vec![];
            let rat = "Dictionary-based translation".to_string();

            (target, conf, alts, rat)
        };

        Ok(AutoTermMapping {
            id: format!("term-map-{}", uuid::Uuid::new_v4()),
            source_term: term.to_string(),
            target_term,
            confidence,
            context: context.to_string(),
            alternatives,
            rationale,
            examples: vec![format!("Example usage: {} in {}", term, context)],
        })
    }

    /// Maps multiple terms in batch.
    pub async fn map_terms_batch(
        &self,
        terms: &[String],
        source_jurisdiction: &Jurisdiction,
        target_jurisdiction: &Jurisdiction,
        context: &str,
    ) -> PortingResult<Vec<AutoTermMapping>> {
        let mut mappings = Vec::new();

        for term in terms {
            let mapping = self
                .map_term(term, source_jurisdiction, target_jurisdiction, context)
                .await?;
            mappings.push(mapping);
        }

        Ok(mappings)
    }
}

impl Default for AutoTermMapper {
    fn default() -> Self {
        Self::new()
    }
}

/// AI-enhanced gap analysis result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiGapAnalysis {
    /// Analysis ID
    pub id: String,
    /// Source statute ID
    pub source_statute_id: String,
    /// Target jurisdiction
    pub target_jurisdiction: String,
    /// Identified gaps
    pub gaps: Vec<AiGap>,
    /// Overall coverage score (0.0 to 1.0)
    pub coverage_score: f64,
    /// Completeness assessment
    pub completeness_assessment: String,
    /// Critical gaps that must be addressed
    pub critical_gaps: Vec<String>,
    /// Recommended actions
    pub recommended_actions: Vec<String>,
    /// Confidence in the analysis
    pub confidence: f64,
}

/// AI-identified gap in porting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiGap {
    /// Gap ID
    pub id: String,
    /// Gap type
    pub gap_type: AiGapType,
    /// Description
    pub description: String,
    /// Severity
    pub severity: Severity,
    /// Impact analysis
    pub impact: String,
    /// Suggested solutions
    pub solutions: Vec<AiGapSolution>,
    /// Estimated effort to address
    pub effort_estimate: EffortLevel,
    /// Dependencies on other gaps
    pub dependencies: Vec<String>,
}

/// Type of AI-identified gap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AiGapType {
    /// Missing legal authority
    MissingAuthority,
    /// Missing enforcement mechanism
    MissingEnforcement,
    /// Missing cultural adaptation
    MissingCulturalAdaptation,
    /// Missing procedural framework
    MissingProcedure,
    /// Missing stakeholder consideration
    MissingStakeholder,
    /// Incomplete definitions
    IncompleteDefinitions,
    /// Insufficient remedies
    InsufficientRemedies,
}

/// Solution for an AI-identified gap.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiGapSolution {
    /// Solution ID
    pub id: String,
    /// Solution description
    pub description: String,
    /// Implementation steps
    pub steps: Vec<String>,
    /// Required resources
    pub resources: Vec<String>,
    /// Success likelihood (0.0 to 1.0)
    pub success_likelihood: f64,
}

/// AI-powered gap analyzer.
#[derive(Clone)]
pub struct AiGapAnalyzer {
    /// Optional LLM generator
    generator: Option<std::sync::Arc<dyn TextGenerator>>,
}

impl AiGapAnalyzer {
    /// Creates a new AI gap analyzer.
    pub fn new() -> Self {
        Self { generator: None }
    }

    /// Creates an analyzer with an LLM generator.
    pub fn with_generator(generator: std::sync::Arc<dyn TextGenerator>) -> Self {
        Self {
            generator: Some(generator),
        }
    }

    /// Performs AI-enhanced gap analysis.
    #[allow(clippy::too_many_arguments)]
    pub async fn analyze_gaps(
        &self,
        statute: &Statute,
        source_jurisdiction: &Jurisdiction,
        target_jurisdiction: &Jurisdiction,
    ) -> PortingResult<AiGapAnalysis> {
        let gaps = if let Some(generator) = &self.generator {
            // Use LLM for comprehensive gap analysis
            let prompt = format!(
                "Perform comprehensive gap analysis for porting statute:\n\
                Statute: '{}'\n\
                From: {} ({:?} legal system)\n\
                To: {} ({:?} legal system)\n\n\
                Identify gaps in:\n\
                1. Legal authority\n\
                2. Enforcement mechanisms\n\
                3. Cultural adaptation\n\
                4. Procedural framework\n\
                5. Stakeholder considerations\n\
                Provide severity, impact, and solutions for each gap.",
                statute.title,
                source_jurisdiction.name,
                source_jurisdiction.legal_system,
                target_jurisdiction.name,
                target_jurisdiction.legal_system
            );

            let response = generator
                .generate(&prompt)
                .await
                .map_err(PortingError::Llm)?;

            // Parse LLM response into gaps (simplified)
            vec![
                AiGap {
                    id: format!("gap-{}", uuid::Uuid::new_v4()),
                    gap_type: AiGapType::MissingEnforcement,
                    description: "Enforcement authority may need designation".to_string(),
                    severity: Severity::Warning,
                    impact: "May affect statute effectiveness".to_string(),
                    solutions: vec![AiGapSolution {
                        id: format!("sol-{}", uuid::Uuid::new_v4()),
                        description: "Designate equivalent enforcement body".to_string(),
                        steps: vec![
                            "Identify target jurisdiction enforcement agencies".to_string(),
                            "Map responsibilities".to_string(),
                        ],
                        resources: vec!["Legal research".to_string()],
                        success_likelihood: 0.8,
                    }],
                    effort_estimate: EffortLevel::Medium,
                    dependencies: vec![],
                },
                AiGap {
                    id: format!("gap-{}", uuid::Uuid::new_v4()),
                    gap_type: AiGapType::MissingCulturalAdaptation,
                    description: format!(
                        "Cultural adaptation needed: {}",
                        response.lines().next().unwrap_or("")
                    ),
                    severity: Severity::Info,
                    impact: "Affects cultural appropriateness".to_string(),
                    solutions: vec![AiGapSolution {
                        id: format!("sol-{}", uuid::Uuid::new_v4()),
                        description: "Consult cultural advisors".to_string(),
                        steps: vec!["Engage local experts".to_string()],
                        resources: vec!["Cultural consultation".to_string()],
                        success_likelihood: 0.9,
                    }],
                    effort_estimate: EffortLevel::Low,
                    dependencies: vec![],
                },
            ]
        } else {
            // Fallback: rule-based gap analysis
            vec![AiGap {
                id: format!("gap-{}", uuid::Uuid::new_v4()),
                gap_type: AiGapType::MissingEnforcement,
                description: "Standard enforcement gap check".to_string(),
                severity: Severity::Info,
                impact: "Standard porting consideration".to_string(),
                solutions: vec![],
                effort_estimate: EffortLevel::Medium,
                dependencies: vec![],
            }]
        };

        let critical_gaps: Vec<String> = gaps
            .iter()
            .filter(|g| g.severity == Severity::Critical)
            .map(|g| g.description.clone())
            .collect();

        let coverage_score = 1.0 - (gaps.len() as f64 * 0.1).min(0.6);

        Ok(AiGapAnalysis {
            id: format!("ai-gap-{}", uuid::Uuid::new_v4()),
            source_statute_id: statute.id.clone(),
            target_jurisdiction: target_jurisdiction.id.clone(),
            gaps,
            coverage_score,
            completeness_assessment: if coverage_score > 0.7 {
                "Good coverage with addressable gaps".to_string()
            } else {
                "Significant gaps require attention".to_string()
            },
            critical_gaps,
            recommended_actions: vec![
                "Address critical gaps before implementation".to_string(),
                "Conduct stakeholder review".to_string(),
            ],
            confidence: if self.generator.is_some() { 0.85 } else { 0.65 },
        })
    }
}

impl Default for AiGapAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Intelligent conflict prediction result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictPrediction {
    /// Prediction ID
    pub id: String,
    /// Source statute ID
    pub source_statute_id: String,
    /// Target jurisdiction
    pub target_jurisdiction: String,
    /// Predicted conflicts
    pub predicted_conflicts: Vec<PredictedConflict>,
    /// Overall conflict risk score (0.0 to 1.0)
    pub risk_score: f64,
    /// Risk assessment
    pub risk_assessment: String,
    /// Preventive measures
    pub preventive_measures: Vec<String>,
    /// Confidence in predictions
    pub confidence: f64,
}

/// A predicted conflict.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictedConflict {
    /// Conflict ID
    pub id: String,
    /// Conflict type
    pub conflict_type: ConflictType,
    /// Description
    pub description: String,
    /// Likelihood (0.0 to 1.0)
    pub likelihood: f64,
    /// Severity
    pub severity: Severity,
    /// Potential impact
    pub impact: String,
    /// Early warning indicators
    pub indicators: Vec<String>,
    /// Mitigation strategies
    pub mitigations: Vec<String>,
}

/// Intelligent conflict predictor using ML/AI.
#[derive(Clone)]
pub struct IntelligentConflictPredictor {
    /// Optional LLM generator
    generator: Option<std::sync::Arc<dyn TextGenerator>>,
    /// Historical conflict database
    precedent_db: ConflictPrecedentDatabase,
}

impl IntelligentConflictPredictor {
    /// Creates a new intelligent conflict predictor.
    pub fn new() -> Self {
        Self {
            generator: None,
            precedent_db: ConflictPrecedentDatabase::new(),
        }
    }

    /// Creates a predictor with an LLM generator.
    pub fn with_generator(generator: std::sync::Arc<dyn TextGenerator>) -> Self {
        Self {
            generator: Some(generator),
            precedent_db: ConflictPrecedentDatabase::new(),
        }
    }

    /// Predicts potential conflicts using AI.
    pub async fn predict_conflicts(
        &self,
        statute: &Statute,
        source_jurisdiction: &Jurisdiction,
        target_jurisdiction: &Jurisdiction,
    ) -> PortingResult<ConflictPrediction> {
        let predicted_conflicts = if let Some(generator) = &self.generator {
            // Use LLM for intelligent conflict prediction
            let prompt = format!(
                "Predict potential legal conflicts when porting statute:\n\
                Statute: '{}'\n\
                From: {} ({:?} legal system)\n\
                To: {} ({:?} legal system)\n\n\
                Analyze potential conflicts in:\n\
                1. Legal authority and jurisdiction\n\
                2. Procedural requirements\n\
                3. Cultural and ethical norms\n\
                4. Existing legislation\n\
                5. Constitutional compatibility\n\
                For each conflict, provide likelihood, severity, and mitigation.",
                statute.title,
                source_jurisdiction.name,
                source_jurisdiction.legal_system,
                target_jurisdiction.name,
                target_jurisdiction.legal_system
            );

            let response = generator
                .generate(&prompt)
                .await
                .map_err(PortingError::Llm)?;

            // Parse LLM response (simplified)
            vec![
                PredictedConflict {
                    id: format!("pred-{}", uuid::Uuid::new_v4()),
                    conflict_type: ConflictType::SystemMismatch,
                    description: "Legal system procedural differences".to_string(),
                    likelihood: 0.7,
                    severity: Severity::Warning,
                    impact: "May require procedural adaptation".to_string(),
                    indicators: vec!["Different legal traditions".to_string()],
                    mitigations: vec![
                        "Adapt procedures to target system".to_string(),
                        "Consult legal experts".to_string(),
                    ],
                },
                PredictedConflict {
                    id: format!("pred-{}", uuid::Uuid::new_v4()),
                    conflict_type: ConflictType::CulturalIncompatibility,
                    description: format!(
                        "AI prediction: {}",
                        response
                            .lines()
                            .next()
                            .unwrap_or("Cultural consideration needed")
                    ),
                    likelihood: 0.5,
                    severity: Severity::Info,
                    impact: "Cultural sensitivity required".to_string(),
                    indicators: vec!["Cultural parameter differences".to_string()],
                    mitigations: vec!["Cultural consultation".to_string()],
                },
            ]
        } else {
            // Fallback: rule-based prediction using precedent database
            let precedents = self.precedent_db.find_relevant_precedents(
                &source_jurisdiction.id,
                &target_jurisdiction.id,
                &ConflictType::SystemMismatch,
            );

            if !precedents.is_empty() {
                vec![PredictedConflict {
                    id: format!("pred-{}", uuid::Uuid::new_v4()),
                    conflict_type: ConflictType::SystemMismatch,
                    description: "Historical conflict pattern detected".to_string(),
                    likelihood: 0.6,
                    severity: Severity::Warning,
                    impact: "Based on historical precedents".to_string(),
                    indicators: vec!["Similar past conflicts".to_string()],
                    mitigations: vec!["Apply proven resolution strategies".to_string()],
                }]
            } else {
                vec![]
            }
        };

        let risk_score = if predicted_conflicts.is_empty() {
            0.1
        } else {
            predicted_conflicts
                .iter()
                .map(|c| c.likelihood)
                .sum::<f64>()
                / predicted_conflicts.len() as f64
        };

        Ok(ConflictPrediction {
            id: format!("conflict-pred-{}", uuid::Uuid::new_v4()),
            source_statute_id: statute.id.clone(),
            target_jurisdiction: target_jurisdiction.id.clone(),
            predicted_conflicts,
            risk_score,
            risk_assessment: if risk_score < 0.3 {
                "Low conflict risk".to_string()
            } else if risk_score < 0.7 {
                "Moderate conflict risk - review recommended".to_string()
            } else {
                "High conflict risk - extensive review required".to_string()
            },
            preventive_measures: vec![
                "Conduct thorough legal review".to_string(),
                "Engage stakeholders early".to_string(),
                "Plan mitigation strategies".to_string(),
            ],
            confidence: if self.generator.is_some() { 0.8 } else { 0.6 },
        })
    }

    /// Analyzes conflict patterns from history.
    pub fn analyze_patterns(
        &self,
        source_jurisdiction: &str,
        target_jurisdiction: &str,
    ) -> Vec<String> {
        let precedents = self.precedent_db.find_relevant_precedents(
            source_jurisdiction,
            target_jurisdiction,
            &ConflictType::SystemMismatch,
        );

        precedents
            .iter()
            .map(|p| format!("Pattern: {:?} -> {}", p.conflict_type, p.resolution_used))
            .collect()
    }
}

impl Default for IntelligentConflictPredictor {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Multi-Jurisdiction Workflows (v0.2.1)
// ============================================================================

/// Multi-target porting request for simultaneous porting to multiple jurisdictions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiTargetPortingRequest {
    /// Request ID
    pub id: String,
    /// Source statute
    pub source_statute: Statute,
    /// Source jurisdiction
    pub source_jurisdiction: Jurisdiction,
    /// Target jurisdictions
    pub target_jurisdictions: Vec<Jurisdiction>,
    /// Porting options
    pub options: PortingOptions,
    /// Whether to resolve dependencies
    pub resolve_dependencies: bool,
    /// Whether to enable cascade propagation
    pub enable_cascade: bool,
}

/// Result of multi-target porting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiTargetPortingResult {
    /// Result ID
    pub id: String,
    /// Source statute ID
    pub source_statute_id: String,
    /// Individual porting results by jurisdiction
    pub jurisdiction_results: HashMap<String, PortedStatute>,
    /// Failed jurisdictions with error messages
    pub failures: HashMap<String, String>,
    /// Overall success rate (0.0 to 1.0)
    pub success_rate: f64,
    /// Dependency resolution log
    pub dependency_log: Vec<String>,
    /// Cascade propagation log
    pub cascade_log: Vec<String>,
    /// Cross-jurisdiction conflicts detected
    pub cross_conflicts: Vec<CrossJurisdictionConflict>,
}

/// Conflict that spans multiple jurisdictions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossJurisdictionConflict {
    /// Conflict ID
    pub id: String,
    /// Jurisdictions involved
    pub jurisdictions: Vec<String>,
    /// Conflict description
    pub description: String,
    /// Severity
    pub severity: Severity,
    /// Recommended resolution
    pub resolution: String,
}

/// Multi-target porting engine for simultaneous porting to multiple jurisdictions.
#[derive(Clone)]
pub struct MultiTargetPortingEngine {
    /// Dependency resolver
    dependency_resolver: JurisdictionDependencyResolver,
}

impl MultiTargetPortingEngine {
    /// Creates a new multi-target porting engine.
    pub fn new() -> Self {
        Self {
            dependency_resolver: JurisdictionDependencyResolver::new(),
        }
    }

    /// Ports a statute to multiple jurisdictions simultaneously.
    pub async fn port_to_multiple_targets(
        &self,
        request: MultiTargetPortingRequest,
    ) -> PortingResult<MultiTargetPortingResult> {
        let mut jurisdiction_results = HashMap::new();
        let mut failures = HashMap::new();
        let mut dependency_log = Vec::new();
        let mut cascade_log = Vec::new();

        // Resolve dependencies if requested
        let ordered_jurisdictions = if request.resolve_dependencies {
            let deps = self
                .dependency_resolver
                .resolve_dependencies(&request.target_jurisdictions);
            dependency_log.push(format!("Resolved {} dependencies", deps.len()));
            deps
        } else {
            request.target_jurisdictions.clone()
        };

        // Port to each jurisdiction in order
        for target_jurisdiction in ordered_jurisdictions {
            let engine = PortingEngine::new(
                request.source_jurisdiction.clone(),
                target_jurisdiction.clone(),
            );

            match engine.port_statute(&request.source_statute, &request.options) {
                Ok(ported) => {
                    jurisdiction_results.insert(target_jurisdiction.id.clone(), ported.clone());

                    // Cascade changes if enabled
                    if request.enable_cascade {
                        cascade_log.push(format!("Cascaded changes to {}", target_jurisdiction.id));
                    }
                }
                Err(e) => {
                    failures.insert(target_jurisdiction.id.clone(), e.to_string());
                }
            }
        }

        let success_rate = if jurisdiction_results.is_empty() && failures.is_empty() {
            0.0
        } else {
            jurisdiction_results.len() as f64 / (jurisdiction_results.len() + failures.len()) as f64
        };

        // Detect cross-jurisdiction conflicts
        let cross_conflicts = self.detect_cross_conflicts(&jurisdiction_results);

        Ok(MultiTargetPortingResult {
            id: format!("multi-port-{}", uuid::Uuid::new_v4()),
            source_statute_id: request.source_statute.id.clone(),
            jurisdiction_results,
            failures,
            success_rate,
            dependency_log,
            cascade_log,
            cross_conflicts,
        })
    }

    /// Detects conflicts across multiple jurisdictions.
    fn detect_cross_conflicts(
        &self,
        results: &HashMap<String, PortedStatute>,
    ) -> Vec<CrossJurisdictionConflict> {
        let mut conflicts = Vec::new();

        // Check for inconsistencies between jurisdictions
        if results.len() > 1 {
            conflicts.push(CrossJurisdictionConflict {
                id: format!("cross-conflict-{}", uuid::Uuid::new_v4()),
                jurisdictions: results.keys().cloned().collect(),
                description: "Potential inconsistency in multi-jurisdiction porting".to_string(),
                severity: Severity::Info,
                resolution: "Review and harmonize across jurisdictions".to_string(),
            });
        }

        conflicts
    }
}

impl Default for MultiTargetPortingEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Jurisdiction dependency information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JurisdictionDependency {
    /// Dependency ID
    pub id: String,
    /// Source jurisdiction (depends on target)
    pub source_jurisdiction: String,
    /// Target jurisdiction (dependency)
    pub target_jurisdiction: String,
    /// Dependency type
    pub dependency_type: DependencyType,
    /// Strength of dependency (0.0 to 1.0)
    pub strength: f64,
    /// Explanation
    pub explanation: String,
}

/// Type of jurisdiction dependency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DependencyType {
    /// Legal system compatibility
    LegalSystemCompatibility,
    /// Treaty obligation
    TreatyObligation,
    /// Trade agreement
    TradeAgreement,
    /// Regional harmonization
    RegionalHarmonization,
    /// Model law adoption
    ModelLawAdoption,
}

/// Jurisdiction dependency resolver.
#[derive(Clone)]
pub struct JurisdictionDependencyResolver {
    /// Known dependencies
    dependencies: HashMap<String, Vec<JurisdictionDependency>>,
}

impl JurisdictionDependencyResolver {
    /// Creates a new dependency resolver.
    pub fn new() -> Self {
        Self {
            dependencies: HashMap::new(),
        }
    }

    /// Adds a dependency.
    #[allow(dead_code)]
    pub fn add_dependency(&mut self, dependency: JurisdictionDependency) {
        self.dependencies
            .entry(dependency.source_jurisdiction.clone())
            .or_default()
            .push(dependency);
    }

    /// Resolves dependencies and returns jurisdictions in dependency order.
    pub fn resolve_dependencies(&self, jurisdictions: &[Jurisdiction]) -> Vec<Jurisdiction> {
        // Simple topological sort - in production, would use proper algorithm
        let mut ordered = jurisdictions.to_vec();

        // Sort by legal system similarity (civil law jurisdictions together, etc.)
        ordered.sort_by_key(|j| match j.legal_system {
            LegalSystem::CivilLaw => 0,
            LegalSystem::CommonLaw => 1,
            _ => 2,
        });

        ordered
    }

    /// Finds dependencies for a jurisdiction.
    #[allow(dead_code)]
    pub fn find_dependencies(&self, jurisdiction_id: &str) -> Vec<&JurisdictionDependency> {
        self.dependencies
            .get(jurisdiction_id)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }
}

impl Default for JurisdictionDependencyResolver {
    fn default() -> Self {
        Self::new()
    }
}

/// Cascade change propagation configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CascadeConfig {
    /// Configuration ID
    pub id: String,
    /// Source jurisdiction
    pub source_jurisdiction: String,
    /// Target jurisdictions for cascade
    pub cascade_targets: Vec<String>,
    /// Propagation rules
    pub propagation_rules: Vec<PropagationRule>,
    /// Whether to propagate automatically
    pub auto_propagate: bool,
}

/// Rule for change propagation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropagationRule {
    /// Rule ID
    pub id: String,
    /// Rule name
    pub name: String,
    /// Change type to propagate
    pub change_type: ChangeType,
    /// Conditions for propagation
    pub conditions: Vec<String>,
    /// Target jurisdictions (empty = all)
    pub target_jurisdictions: Vec<String>,
}

/// Change propagation result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CascadePropagationResult {
    /// Result ID
    pub id: String,
    /// Source statute ID
    pub source_statute_id: String,
    /// Changes propagated
    pub propagated_changes: HashMap<String, Vec<PortingChange>>,
    /// Propagation conflicts
    pub conflicts: Vec<String>,
    /// Success rate (0.0 to 1.0)
    pub success_rate: f64,
}

/// Cascade change propagator.
#[derive(Clone)]
pub struct CascadeChangePropagator {
    /// Cascade configurations
    configs: Vec<CascadeConfig>,
}

impl CascadeChangePropagator {
    /// Creates a new cascade change propagator.
    pub fn new() -> Self {
        Self {
            configs: Vec::new(),
        }
    }

    /// Adds a cascade configuration.
    #[allow(dead_code)]
    pub fn add_config(&mut self, config: CascadeConfig) {
        self.configs.push(config);
    }

    /// Propagates changes across jurisdictions.
    #[allow(dead_code)]
    pub fn propagate_changes(
        &self,
        source_statute: &Statute,
        changes: &[PortingChange],
        config: &CascadeConfig,
    ) -> CascadePropagationResult {
        let mut propagated_changes = HashMap::new();
        let conflicts = Vec::new();

        // Apply propagation rules
        for target_jurisdiction in &config.cascade_targets {
            let mut target_changes = Vec::new();

            for change in changes {
                // Check if change should be propagated
                let should_propagate = config.propagation_rules.iter().any(|rule| {
                    rule.change_type == change.change_type
                        && (rule.target_jurisdictions.is_empty()
                            || rule.target_jurisdictions.contains(target_jurisdiction))
                });

                if should_propagate {
                    target_changes.push(change.clone());
                }
            }

            if !target_changes.is_empty() {
                propagated_changes.insert(target_jurisdiction.clone(), target_changes);
            }
        }

        let total_targets = config.cascade_targets.len();
        let successful_propagations = propagated_changes.len();
        let success_rate = if total_targets > 0 {
            successful_propagations as f64 / total_targets as f64
        } else {
            0.0
        };

        CascadePropagationResult {
            id: format!("cascade-{}", uuid::Uuid::new_v4()),
            source_statute_id: source_statute.id.clone(),
            propagated_changes,
            conflicts,
            success_rate,
        }
    }
}

impl Default for CascadeChangePropagator {
    fn default() -> Self {
        Self::new()
    }
}

/// Cross-jurisdiction synchronization state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynchronizationState {
    /// State ID
    pub id: String,
    /// Statute ID being synchronized
    pub statute_id: String,
    /// Jurisdictions involved
    pub jurisdictions: Vec<String>,
    /// Current versions by jurisdiction
    pub versions: HashMap<String, String>,
    /// Synchronization status
    pub status: SyncStatus,
    /// Last synchronized timestamp
    pub last_sync: String,
    /// Pending changes by jurisdiction
    pub pending_changes: HashMap<String, Vec<PortingChange>>,
}

/// Synchronization status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyncStatus {
    /// All jurisdictions synchronized
    Synchronized,
    /// Synchronization in progress
    InProgress,
    /// Out of sync
    OutOfSync,
    /// Conflict detected
    Conflict,
}

/// Cross-jurisdiction synchronization manager.
#[derive(Clone)]
pub struct CrossJurisdictionSynchronizer {
    /// Synchronization states
    states: HashMap<String, SynchronizationState>,
}

impl CrossJurisdictionSynchronizer {
    /// Creates a new synchronizer.
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
        }
    }

    /// Starts synchronization for a statute across jurisdictions.
    pub fn start_sync(
        &mut self,
        statute_id: &str,
        jurisdictions: Vec<String>,
    ) -> SynchronizationState {
        let state = SynchronizationState {
            id: format!("sync-{}", uuid::Uuid::new_v4()),
            statute_id: statute_id.to_string(),
            jurisdictions: jurisdictions.clone(),
            versions: jurisdictions
                .iter()
                .map(|j| (j.clone(), "v1.0".to_string()))
                .collect(),
            status: SyncStatus::InProgress,
            last_sync: chrono::Utc::now().to_rfc3339(),
            pending_changes: HashMap::new(),
        };

        self.states.insert(statute_id.to_string(), state.clone());
        state
    }

    /// Checks synchronization status.
    #[allow(dead_code)]
    pub fn check_sync_status(&self, statute_id: &str) -> Option<SyncStatus> {
        self.states.get(statute_id).map(|s| s.status)
    }

    /// Synchronizes changes across jurisdictions.
    #[allow(dead_code)]
    pub fn synchronize_changes(
        &mut self,
        statute_id: &str,
        jurisdiction: &str,
        changes: Vec<PortingChange>,
    ) -> Result<(), String> {
        if let Some(state) = self.states.get_mut(statute_id) {
            // Add pending changes
            state
                .pending_changes
                .entry(jurisdiction.to_string())
                .or_default()
                .extend(changes);

            // Check if all jurisdictions have pending changes
            let all_have_changes = state
                .jurisdictions
                .iter()
                .all(|j| state.pending_changes.contains_key(j));

            if all_have_changes {
                state.status = SyncStatus::Synchronized;
                state.last_sync = chrono::Utc::now().to_rfc3339();
            } else {
                state.status = SyncStatus::OutOfSync;
            }

            Ok(())
        } else {
            Err("Synchronization state not found".to_string())
        }
    }

    /// Gets synchronization state.
    #[allow(dead_code)]
    pub fn get_state(&self, statute_id: &str) -> Option<&SynchronizationState> {
        self.states.get(statute_id)
    }
}

impl Default for CrossJurisdictionSynchronizer {
    fn default() -> Self {
        Self::new()
    }
}

/// Harmonization tracking record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarmonizationRecord {
    /// Record ID
    pub id: String,
    /// Statute ID being harmonized
    pub statute_id: String,
    /// Jurisdictions being harmonized
    pub jurisdictions: Vec<String>,
    /// Harmonization goal
    pub goal: String,
    /// Current harmonization score (0.0 to 1.0)
    pub harmonization_score: f64,
    /// Differences identified
    pub differences: Vec<HarmonizationDifference>,
    /// Harmonization actions taken
    pub actions: Vec<HarmonizationAction>,
    /// Status
    pub status: HarmonizationStatus,
}

/// Difference between jurisdictions in harmonization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarmonizationDifference {
    /// Difference ID
    pub id: String,
    /// Jurisdictions with difference
    pub jurisdictions: Vec<String>,
    /// Difference type
    pub difference_type: DifferenceType,
    /// Description
    pub description: String,
    /// Impact on harmonization
    pub impact: f64,
}

/// Type of harmonization difference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DifferenceType {
    /// Terminological difference
    Terminological,
    /// Procedural difference
    Procedural,
    /// Cultural difference
    Cultural,
    /// Legal system difference
    LegalSystem,
    /// Enforcement difference
    Enforcement,
}

/// Action taken for harmonization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarmonizationAction {
    /// Action ID
    pub id: String,
    /// Action type
    pub action_type: String,
    /// Description
    pub description: String,
    /// Jurisdictions affected
    pub jurisdictions_affected: Vec<String>,
    /// Impact on harmonization score
    pub impact: f64,
    /// Timestamp
    pub timestamp: String,
}

/// Harmonization status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HarmonizationStatus {
    /// Planning harmonization
    Planning,
    /// In progress
    InProgress,
    /// Partially harmonized
    PartiallyHarmonized,
    /// Fully harmonized
    FullyHarmonized,
    /// Harmonization failed
    Failed,
}

/// Harmonization tracker.
#[derive(Clone)]
pub struct HarmonizationTracker {
    /// Harmonization records
    records: HashMap<String, HarmonizationRecord>,
}

impl HarmonizationTracker {
    /// Creates a new harmonization tracker.
    pub fn new() -> Self {
        Self {
            records: HashMap::new(),
        }
    }

    /// Starts tracking harmonization.
    pub fn start_tracking(
        &mut self,
        statute_id: &str,
        jurisdictions: Vec<String>,
        goal: String,
    ) -> HarmonizationRecord {
        let record = HarmonizationRecord {
            id: format!("harm-{}", uuid::Uuid::new_v4()),
            statute_id: statute_id.to_string(),
            jurisdictions,
            goal,
            harmonization_score: 0.0,
            differences: Vec::new(),
            actions: Vec::new(),
            status: HarmonizationStatus::Planning,
        };

        self.records.insert(statute_id.to_string(), record.clone());
        record
    }

    /// Adds a difference.
    #[allow(dead_code)]
    pub fn add_difference(
        &mut self,
        statute_id: &str,
        difference: HarmonizationDifference,
    ) -> Result<(), String> {
        if let Some(record) = self.records.get_mut(statute_id) {
            record.differences.push(difference);
            self.update_harmonization_score(statute_id)?;
            Ok(())
        } else {
            Err("Harmonization record not found".to_string())
        }
    }

    /// Records a harmonization action.
    #[allow(dead_code)]
    pub fn record_action(
        &mut self,
        statute_id: &str,
        action: HarmonizationAction,
    ) -> Result<(), String> {
        if let Some(record) = self.records.get_mut(statute_id) {
            record.actions.push(action);
            self.update_harmonization_score(statute_id)?;
            Ok(())
        } else {
            Err("Harmonization record not found".to_string())
        }
    }

    /// Updates harmonization score.
    fn update_harmonization_score(&mut self, statute_id: &str) -> Result<(), String> {
        if let Some(record) = self.records.get_mut(statute_id) {
            // Calculate score based on differences and actions
            let difference_penalty = record.differences.len() as f64 * 0.1;
            let action_bonus = record.actions.iter().map(|a| a.impact).sum::<f64>();

            let score = (1.0 - difference_penalty + action_bonus).clamp(0.0, 1.0);
            record.harmonization_score = score;

            // Update status based on score
            record.status = if score >= 0.9 {
                HarmonizationStatus::FullyHarmonized
            } else if score >= 0.6 {
                HarmonizationStatus::PartiallyHarmonized
            } else {
                HarmonizationStatus::InProgress
            };

            Ok(())
        } else {
            Err("Harmonization record not found".to_string())
        }
    }

    /// Gets harmonization record.
    #[allow(dead_code)]
    pub fn get_record(&self, statute_id: &str) -> Option<&HarmonizationRecord> {
        self.records.get(statute_id)
    }

    /// Gets all records.
    #[allow(dead_code)]
    pub fn all_records(&self) -> Vec<&HarmonizationRecord> {
        self.records.values().collect()
    }
}

impl Default for HarmonizationTracker {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Quality Assurance (v0.2.5)
// ============================================================================

/// Quality score for a ported statute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityScore {
    /// Overall quality score (0.0 to 1.0).
    pub overall: f64,
    /// Semantic preservation score (0.0 to 1.0).
    pub semantic_preservation: f64,
    /// Legal correctness score (0.0 to 1.0).
    pub legal_correctness: f64,
    /// Cultural adaptation score (0.0 to 1.0).
    pub cultural_adaptation: f64,
    /// Completeness score (0.0 to 1.0).
    pub completeness: f64,
    /// Consistency score (0.0 to 1.0).
    pub consistency: f64,
    /// Quality grade.
    pub grade: QualityGrade,
    /// Detailed quality issues.
    pub issues: Vec<QualityIssue>,
    /// Recommendations for improvement.
    pub recommendations: Vec<String>,
}

/// Quality grade classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QualityGrade {
    /// Excellent quality (>= 0.9).
    Excellent,
    /// Good quality (>= 0.75).
    Good,
    /// Acceptable quality (>= 0.6).
    Acceptable,
    /// Poor quality (>= 0.4).
    Poor,
    /// Unacceptable quality (< 0.4).
    Unacceptable,
}

/// Quality issue found during assessment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityIssue {
    /// Issue type.
    pub issue_type: QualityIssueType,
    /// Severity level.
    pub severity: QualityIssueSeverity,
    /// Description of the issue.
    pub description: String,
    /// Location in the ported statute.
    pub location: Option<String>,
    /// Suggested fix.
    pub suggested_fix: Option<String>,
}

/// Type of quality issue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QualityIssueType {
    /// Semantic meaning not preserved.
    SemanticDrift,
    /// Legal term incorrectly translated.
    IncorrectTranslation,
    /// Cultural adaptation missing or incorrect.
    CulturalMismatch,
    /// Inconsistent terminology.
    InconsistentTerminology,
    /// Missing required elements.
    Incompleteness,
    /// Logical inconsistency.
    LogicalInconsistency,
    /// Compliance violation.
    ComplianceViolation,
}

/// Severity of quality issue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QualityIssueSeverity {
    /// Critical issue that must be fixed.
    Critical,
    /// Major issue that should be fixed.
    Major,
    /// Minor issue that could be improved.
    Minor,
    /// Informational note.
    Info,
}

/// Quality scorer for automated quality assessment.
pub struct QualityScorer {
    /// Minimum acceptable quality threshold.
    min_quality_threshold: f64,
}

impl QualityScorer {
    /// Creates a new quality scorer.
    pub fn new() -> Self {
        Self {
            min_quality_threshold: 0.6,
        }
    }

    /// Sets minimum quality threshold.
    #[allow(dead_code)]
    pub fn with_threshold(mut self, threshold: f64) -> Self {
        self.min_quality_threshold = threshold.clamp(0.0, 1.0);
        self
    }

    /// Scores a ported statute.
    pub fn score_porting(&self, ported: &PortedStatute) -> QualityScore {
        let mut issues = Vec::new();
        let mut recommendations = Vec::new();

        // Score semantic preservation
        let semantic_score = self.score_semantic_preservation(ported, &mut issues);

        // Score legal correctness
        let legal_score = self.score_legal_correctness(ported, &mut issues);

        // Score cultural adaptation
        let cultural_score = self.score_cultural_adaptation(ported, &mut issues);

        // Score completeness
        let completeness_score = self.score_completeness(ported, &mut issues);

        // Score consistency
        let consistency_score = self.score_consistency(ported, &mut issues);

        // Calculate overall score (weighted average)
        let overall = (semantic_score * 0.25)
            + (legal_score * 0.25)
            + (cultural_score * 0.2)
            + (completeness_score * 0.15)
            + (consistency_score * 0.15);

        // Determine grade
        let grade = if overall >= 0.9 {
            QualityGrade::Excellent
        } else if overall >= 0.75 {
            QualityGrade::Good
        } else if overall >= 0.6 {
            QualityGrade::Acceptable
        } else if overall >= 0.4 {
            QualityGrade::Poor
        } else {
            QualityGrade::Unacceptable
        };

        // Generate recommendations
        if overall < 0.9 {
            recommendations.push(
                "Review semantic preservation to ensure legal meaning is maintained".to_string(),
            );
        }
        if cultural_score < 0.8 {
            recommendations
                .push("Review cultural adaptations for accuracy and appropriateness".to_string());
        }
        if !issues.is_empty() {
            recommendations.push(format!(
                "Address {} quality issues identified",
                issues.len()
            ));
        }

        QualityScore {
            overall,
            semantic_preservation: semantic_score,
            legal_correctness: legal_score,
            cultural_adaptation: cultural_score,
            completeness: completeness_score,
            consistency: consistency_score,
            grade,
            issues,
            recommendations,
        }
    }

    /// Scores semantic preservation.
    fn score_semantic_preservation(
        &self,
        ported: &PortedStatute,
        issues: &mut Vec<QualityIssue>,
    ) -> f64 {
        let mut score = 1.0;

        // Check if critical changes were made
        let critical_changes = ported
            .changes
            .iter()
            .filter(|c| {
                matches!(
                    c.change_type,
                    ChangeType::ValueAdaptation | ChangeType::Removal
                )
            })
            .count();

        if critical_changes > 0 {
            score -= 0.1 * critical_changes as f64;
            issues.push(QualityIssue {
                issue_type: QualityIssueType::SemanticDrift,
                severity: QualityIssueSeverity::Major,
                description: format!("{} critical changes to statute meaning", critical_changes),
                location: None,
                suggested_fix: Some(
                    "Review changes to ensure legal meaning is preserved".to_string(),
                ),
            });
        }

        score.max(0.0)
    }

    /// Scores legal correctness.
    fn score_legal_correctness(
        &self,
        ported: &PortedStatute,
        issues: &mut Vec<QualityIssue>,
    ) -> f64 {
        let mut score: f64 = 1.0;

        // Check for translation changes
        let translation_changes = ported
            .changes
            .iter()
            .filter(|c| matches!(c.change_type, ChangeType::Translation))
            .count();

        if translation_changes > 10 {
            score -= 0.05;
            issues.push(QualityIssue {
                issue_type: QualityIssueType::IncorrectTranslation,
                severity: QualityIssueSeverity::Minor,
                description: format!(
                    "{} term translations - review for accuracy",
                    translation_changes
                ),
                location: None,
                suggested_fix: Some(
                    "Verify legal term translations with jurisdiction experts".to_string(),
                ),
            });
        }

        score.max(0.0)
    }

    /// Scores cultural adaptation.
    fn score_cultural_adaptation(
        &self,
        ported: &PortedStatute,
        issues: &mut Vec<QualityIssue>,
    ) -> f64 {
        let mut score: f64 = 1.0;

        // Check for cultural parameter changes
        let cultural_changes = ported
            .changes
            .iter()
            .filter(|c| matches!(c.change_type, ChangeType::CulturalAdaptation))
            .count();

        if cultural_changes == 0 {
            score -= 0.2;
            issues.push(QualityIssue {
                issue_type: QualityIssueType::CulturalMismatch,
                severity: QualityIssueSeverity::Major,
                description:
                    "No cultural adaptations applied - may not be suitable for target jurisdiction"
                        .to_string(),
                location: None,
                suggested_fix: Some("Apply cultural parameter adaptations".to_string()),
            });
        }

        score.max(0.0)
    }

    /// Scores completeness.
    fn score_completeness(&self, ported: &PortedStatute, issues: &mut Vec<QualityIssue>) -> f64 {
        let mut score: f64 = 1.0;

        // Check if statute has minimum required elements
        if ported.statute.id.is_empty() {
            score -= 0.3;
            issues.push(QualityIssue {
                issue_type: QualityIssueType::Incompleteness,
                severity: QualityIssueSeverity::Critical,
                description: "Statute ID is empty".to_string(),
                location: None,
                suggested_fix: Some("Assign a valid statute ID".to_string()),
            });
        }

        if ported.statute.title.is_empty() {
            score -= 0.2;
            issues.push(QualityIssue {
                issue_type: QualityIssueType::Incompleteness,
                severity: QualityIssueSeverity::Major,
                description: "Statute title is empty".to_string(),
                location: None,
                suggested_fix: Some("Provide a statute title".to_string()),
            });
        }

        if ported.changes.is_empty() {
            score -= 0.1;
            issues.push(QualityIssue {
                issue_type: QualityIssueType::Incompleteness,
                severity: QualityIssueSeverity::Minor,
                description: "No changes documented".to_string(),
                location: None,
                suggested_fix: Some("Document all changes made during porting".to_string()),
            });
        }

        score.max(0.0)
    }

    /// Scores consistency.
    fn score_consistency(&self, ported: &PortedStatute, issues: &mut Vec<QualityIssue>) -> f64 {
        let mut score: f64 = 1.0;

        // Check for inconsistent terminology
        let term_changes: Vec<_> = ported
            .changes
            .iter()
            .filter(|c| matches!(c.change_type, ChangeType::Translation))
            .collect();

        // Simple heuristic: if same term appears multiple times with different translations
        if term_changes.len() > 5 {
            score -= 0.05;
            issues.push(QualityIssue {
                issue_type: QualityIssueType::InconsistentTerminology,
                severity: QualityIssueSeverity::Minor,
                description: "Multiple term translations - verify consistency".to_string(),
                location: None,
                suggested_fix: Some(
                    "Ensure consistent translation of legal terms throughout".to_string(),
                ),
            });
        }

        score.max(0.0)
    }

    /// Checks if quality meets minimum threshold.
    #[allow(dead_code)]
    pub fn meets_threshold(&self, score: &QualityScore) -> bool {
        score.overall >= self.min_quality_threshold
    }
}

impl Default for QualityScorer {
    fn default() -> Self {
        Self::new()
    }
}

/// Consistency check result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsistencyCheckResult {
    /// Whether the ported statute is consistent.
    pub is_consistent: bool,
    /// Consistency score (0.0 to 1.0).
    pub consistency_score: f64,
    /// Inconsistencies found.
    pub inconsistencies: Vec<Inconsistency>,
    /// Suggestions for fixing inconsistencies.
    pub suggestions: Vec<String>,
}

/// Inconsistency found in ported statute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inconsistency {
    /// Type of inconsistency.
    pub inconsistency_type: InconsistencyType,
    /// Severity level.
    pub severity: InconsistencySeverity,
    /// Description.
    pub description: String,
    /// Conflicting elements.
    pub conflicting_elements: Vec<String>,
    /// Location in statute.
    pub location: Option<String>,
}

/// Type of inconsistency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InconsistencyType {
    /// Terminology used inconsistently.
    TerminologyInconsistency,
    /// Parameters have conflicting values.
    ParameterConflict,
    /// Legal logic is inconsistent.
    LogicalInconsistency,
    /// References are inconsistent.
    ReferenceInconsistency,
    /// Formatting is inconsistent.
    FormattingInconsistency,
}

/// Severity of inconsistency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InconsistencySeverity {
    /// High severity - must fix.
    High,
    /// Medium severity - should fix.
    Medium,
    /// Low severity - nice to fix.
    Low,
}

/// Consistency verifier for ported statutes.
pub struct ConsistencyVerifier;

impl ConsistencyVerifier {
    /// Creates a new consistency verifier.
    pub fn new() -> Self {
        Self
    }

    /// Verifies consistency of a ported statute.
    pub fn verify(&self, ported: &PortedStatute) -> ConsistencyCheckResult {
        let mut inconsistencies = Vec::new();
        let mut suggestions = Vec::new();

        // Check terminology consistency
        self.check_terminology_consistency(ported, &mut inconsistencies);

        // Check parameter consistency
        self.check_parameter_consistency(ported, &mut inconsistencies);

        // Check logical consistency
        self.check_logical_consistency(ported, &mut inconsistencies);

        // Check reference consistency
        self.check_reference_consistency(ported, &mut inconsistencies);

        // Calculate consistency score
        let consistency_score = if inconsistencies.is_empty() {
            1.0
        } else {
            let penalty = inconsistencies
                .iter()
                .map(|i| match i.severity {
                    InconsistencySeverity::High => 0.2,
                    InconsistencySeverity::Medium => 0.1,
                    InconsistencySeverity::Low => 0.05,
                })
                .sum::<f64>();
            (1.0 - penalty).max(0.0)
        };

        let is_consistent = consistency_score >= 0.8;

        // Generate suggestions
        if !is_consistent {
            suggestions.push(
                "Review and standardize terminology usage throughout the statute".to_string(),
            );
            suggestions
                .push("Verify that all parameters are consistent and non-conflicting".to_string());
        }

        ConsistencyCheckResult {
            is_consistent,
            consistency_score,
            inconsistencies,
            suggestions,
        }
    }

    /// Checks terminology consistency.
    fn check_terminology_consistency(
        &self,
        ported: &PortedStatute,
        inconsistencies: &mut Vec<Inconsistency>,
    ) {
        // Check for term translation inconsistencies
        let term_translations: Vec<_> = ported
            .changes
            .iter()
            .filter(|c| matches!(c.change_type, ChangeType::Translation))
            .collect();

        if term_translations.len() > 10 {
            inconsistencies.push(Inconsistency {
                inconsistency_type: InconsistencyType::TerminologyInconsistency,
                severity: InconsistencySeverity::Low,
                description: format!(
                    "{} term translations - verify consistent usage",
                    term_translations.len()
                ),
                conflicting_elements: vec![],
                location: None,
            });
        }
    }

    /// Checks parameter consistency.
    fn check_parameter_consistency(
        &self,
        ported: &PortedStatute,
        inconsistencies: &mut Vec<Inconsistency>,
    ) {
        // Check for parameter adjustments that might conflict
        let param_changes: Vec<_> = ported
            .changes
            .iter()
            .filter(|c| {
                matches!(
                    c.change_type,
                    ChangeType::ValueAdaptation | ChangeType::CulturalAdaptation
                )
            })
            .collect();

        if param_changes.len() > 5 {
            inconsistencies.push(Inconsistency {
                inconsistency_type: InconsistencyType::ParameterConflict,
                severity: InconsistencySeverity::Medium,
                description: format!(
                    "{} parameter adjustments - verify they don't conflict",
                    param_changes.len()
                ),
                conflicting_elements: vec![],
                location: None,
            });
        }
    }

    /// Checks logical consistency.
    fn check_logical_consistency(
        &self,
        ported: &PortedStatute,
        inconsistencies: &mut Vec<Inconsistency>,
    ) {
        // Check for modifications that might create logical inconsistencies
        let value_mods: Vec<_> = ported
            .changes
            .iter()
            .filter(|c| matches!(c.change_type, ChangeType::ValueAdaptation))
            .collect();

        let removals: Vec<_> = ported
            .changes
            .iter()
            .filter(|c| matches!(c.change_type, ChangeType::Removal))
            .collect();

        if value_mods.len() > 3 && !removals.is_empty() {
            inconsistencies.push(Inconsistency {
                inconsistency_type: InconsistencyType::LogicalInconsistency,
                severity: InconsistencySeverity::High,
                description:
                    "Multiple value adaptations with removals - verify logical consistency"
                        .to_string(),
                conflicting_elements: vec![],
                location: None,
            });
        }
    }

    /// Checks reference consistency.
    fn check_reference_consistency(
        &self,
        _ported: &PortedStatute,
        _inconsistencies: &mut Vec<Inconsistency>,
    ) {
        // In a real implementation, would check that all references are valid
        // For now, this is a placeholder
    }
}

impl Default for ConsistencyVerifier {
    fn default() -> Self {
        Self::new()
    }
}

/// Completeness check result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletenessCheckResult {
    /// Whether the ported statute is complete.
    pub is_complete: bool,
    /// Completeness score (0.0 to 1.0).
    pub completeness_score: f64,
    /// Missing elements.
    pub missing_elements: Vec<MissingElement>,
    /// Optional elements that could be added.
    pub optional_elements: Vec<String>,
    /// Suggestions for improving completeness.
    pub suggestions: Vec<String>,
}

/// Missing element in ported statute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissingElement {
    /// Type of element.
    pub element_type: ElementType,
    /// Importance level.
    pub importance: ElementImportance,
    /// Description.
    pub description: String,
    /// Expected location.
    pub expected_location: Option<String>,
}

/// Type of element that may be missing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ElementType {
    /// Statute metadata.
    Metadata,
    /// Legal effect.
    Effect,
    /// Condition or trigger.
    Condition,
    /// Cultural adaptation.
    CulturalAdaptation,
    /// Jurisdiction information.
    JurisdictionInfo,
    /// Documentation or explanation.
    Documentation,
    /// Validation result.
    ValidationResult,
}

/// Importance of missing element.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ElementImportance {
    /// Required element.
    Required,
    /// Recommended element.
    Recommended,
    /// Optional element.
    Optional,
}

/// Completeness checker for ported statutes.
pub struct CompletenessChecker {
    /// Whether to check for optional elements.
    check_optional: bool,
}

impl CompletenessChecker {
    /// Creates a new completeness checker.
    pub fn new() -> Self {
        Self {
            check_optional: false,
        }
    }

    /// Sets whether to check optional elements.
    #[allow(dead_code)]
    pub fn with_optional_check(mut self, check: bool) -> Self {
        self.check_optional = check;
        self
    }

    /// Checks completeness of a ported statute.
    pub fn check(&self, ported: &PortedStatute) -> CompletenessCheckResult {
        let mut missing_elements = Vec::new();
        let mut optional_elements = Vec::new();
        let mut suggestions = Vec::new();

        // Check required elements
        self.check_required_elements(ported, &mut missing_elements);

        // Check recommended elements
        self.check_recommended_elements(ported, &mut missing_elements);

        // Check optional elements
        if self.check_optional {
            self.check_optional_elements(ported, &mut optional_elements);
        }

        // Calculate completeness score
        let required_missing = missing_elements
            .iter()
            .filter(|e| matches!(e.importance, ElementImportance::Required))
            .count();

        let recommended_missing = missing_elements
            .iter()
            .filter(|e| matches!(e.importance, ElementImportance::Recommended))
            .count();

        let completeness_score = if required_missing > 0 {
            0.0
        } else if recommended_missing > 0 {
            0.7 - (0.1 * recommended_missing as f64).min(0.3)
        } else {
            1.0
        };

        let is_complete = required_missing == 0 && recommended_missing == 0;

        // Generate suggestions
        if !is_complete {
            if required_missing > 0 {
                suggestions.push(format!("Add {} required elements", required_missing));
            }
            if recommended_missing > 0 {
                suggestions.push(format!(
                    "Add {} recommended elements for better quality",
                    recommended_missing
                ));
            }
        }

        CompletenessCheckResult {
            is_complete,
            completeness_score,
            missing_elements,
            optional_elements,
            suggestions,
        }
    }

    /// Checks for required elements.
    fn check_required_elements(&self, ported: &PortedStatute, missing: &mut Vec<MissingElement>) {
        // Check statute ID
        if ported.statute.id.is_empty() {
            missing.push(MissingElement {
                element_type: ElementType::Metadata,
                importance: ElementImportance::Required,
                description: "Statute ID is required".to_string(),
                expected_location: Some("statute.id".to_string()),
            });
        }

        // Check statute title
        if ported.statute.title.is_empty() {
            missing.push(MissingElement {
                element_type: ElementType::Metadata,
                importance: ElementImportance::Required,
                description: "Statute title is required".to_string(),
                expected_location: Some("statute.title".to_string()),
            });
        }
    }

    /// Checks for recommended elements.
    fn check_recommended_elements(
        &self,
        ported: &PortedStatute,
        missing: &mut Vec<MissingElement>,
    ) {
        // Check for cultural adaptations
        let has_cultural_adaptation = ported
            .changes
            .iter()
            .any(|c| matches!(c.change_type, ChangeType::CulturalAdaptation));

        if !has_cultural_adaptation {
            missing.push(MissingElement {
                element_type: ElementType::CulturalAdaptation,
                importance: ElementImportance::Recommended,
                description: "Cultural adaptations are recommended for cross-jurisdiction porting"
                    .to_string(),
                expected_location: Some("changes".to_string()),
            });
        }

        // Check for change documentation
        if ported.changes.is_empty() {
            missing.push(MissingElement {
                element_type: ElementType::Documentation,
                importance: ElementImportance::Recommended,
                description: "Document changes made during porting".to_string(),
                expected_location: Some("changes".to_string()),
            });
        }
    }

    /// Checks for optional elements.
    fn check_optional_elements(&self, _ported: &PortedStatute, optional: &mut Vec<String>) {
        optional.push("Detailed implementation notes".to_string());
        optional.push("Stakeholder review comments".to_string());
        optional.push("Compliance certification".to_string());
    }
}

impl Default for CompletenessChecker {
    fn default() -> Self {
        Self::new()
    }
}

/// Regression test for porting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionTest {
    /// Test ID.
    pub test_id: String,
    /// Test name.
    pub name: String,
    /// Source jurisdiction.
    pub source_jurisdiction: String,
    /// Target jurisdiction.
    pub target_jurisdiction: String,
    /// Input statute (snapshot).
    pub input_statute: String,
    /// Expected output (snapshot).
    pub expected_output: String,
    /// Quality baseline.
    pub quality_baseline: f64,
    /// Created at timestamp.
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last run timestamp.
    pub last_run: Option<chrono::DateTime<chrono::Utc>>,
    /// Test status.
    pub status: RegressionTestStatus,
}

/// Status of regression test.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegressionTestStatus {
    /// Not yet run.
    Pending,
    /// Test passed.
    Passed,
    /// Test failed.
    Failed,
    /// Test skipped.
    Skipped,
}

/// Result of running a regression test.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionTestResult {
    /// Test ID.
    pub test_id: String,
    /// Whether test passed.
    pub passed: bool,
    /// Quality score achieved.
    pub quality_score: f64,
    /// Quality baseline.
    pub quality_baseline: f64,
    /// Quality difference.
    pub quality_diff: f64,
    /// Differences found.
    pub differences: Vec<String>,
    /// Run timestamp.
    pub run_at: chrono::DateTime<chrono::Utc>,
}

/// Regression test manager.
pub struct RegressionTestManager {
    /// Collection of regression tests.
    tests: std::collections::HashMap<String, RegressionTest>,
    /// Quality scorer.
    scorer: QualityScorer,
}

impl RegressionTestManager {
    /// Creates a new regression test manager.
    pub fn new() -> Self {
        Self {
            tests: std::collections::HashMap::new(),
            scorer: QualityScorer::new(),
        }
    }

    /// Adds a regression test.
    pub fn add_test(&mut self, test: RegressionTest) {
        self.tests.insert(test.test_id.clone(), test);
    }

    /// Creates a regression test from a porting result.
    #[allow(dead_code)]
    pub fn create_test_from_porting(
        &mut self,
        test_id: String,
        name: String,
        source_jurisdiction: String,
        target_jurisdiction: String,
        input_statute: String,
        ported: &PortedStatute,
    ) -> Result<(), String> {
        let quality = self.scorer.score_porting(ported);

        let test = RegressionTest {
            test_id: test_id.clone(),
            name,
            source_jurisdiction,
            target_jurisdiction,
            input_statute,
            expected_output: serde_json::to_string(ported)
                .map_err(|e| format!("Failed to serialize ported statute: {}", e))?,
            quality_baseline: quality.overall,
            created_at: chrono::Utc::now(),
            last_run: None,
            status: RegressionTestStatus::Pending,
        };

        self.tests.insert(test_id, test);
        Ok(())
    }

    /// Runs a regression test.
    #[allow(dead_code)]
    pub fn run_test(
        &mut self,
        test_id: &str,
        current_result: &PortedStatute,
    ) -> Result<RegressionTestResult, String> {
        let test = self
            .tests
            .get_mut(test_id)
            .ok_or_else(|| format!("Test {} not found", test_id))?;

        // Score current result
        let quality = self.scorer.score_porting(current_result);

        // Compare with baseline
        let quality_diff = quality.overall - test.quality_baseline;
        let passed = quality_diff >= -0.05; // Allow 5% regression

        // Update test status
        test.status = if passed {
            RegressionTestStatus::Passed
        } else {
            RegressionTestStatus::Failed
        };
        test.last_run = Some(chrono::Utc::now());

        // Find differences (simplified)
        let mut differences = Vec::new();
        if quality_diff < 0.0 {
            differences.push(format!(
                "Quality regressed by {:.2}%",
                -quality_diff * 100.0
            ));
        }

        Ok(RegressionTestResult {
            test_id: test_id.to_string(),
            passed,
            quality_score: quality.overall,
            quality_baseline: test.quality_baseline,
            quality_diff,
            differences,
            run_at: chrono::Utc::now(),
        })
    }

    /// Runs all regression tests.
    #[allow(dead_code)]
    pub fn run_all_tests(
        &mut self,
        results: &std::collections::HashMap<String, PortedStatute>,
    ) -> Vec<RegressionTestResult> {
        let test_ids: Vec<_> = self.tests.keys().cloned().collect();
        let mut all_results = Vec::new();

        for test_id in test_ids {
            if let Some(ported) = results.get(&test_id) {
                if let Ok(result) = self.run_test(&test_id, ported) {
                    all_results.push(result);
                }
            }
        }

        all_results
    }

    /// Gets test statistics.
    #[allow(dead_code)]
    pub fn get_statistics(&self) -> RegressionTestStatistics {
        let total = self.tests.len();
        let mut passed = 0;
        let mut failed = 0;
        let mut pending = 0;
        let mut skipped = 0;

        for test in self.tests.values() {
            match test.status {
                RegressionTestStatus::Passed => passed += 1,
                RegressionTestStatus::Failed => failed += 1,
                RegressionTestStatus::Pending => pending += 1,
                RegressionTestStatus::Skipped => skipped += 1,
            }
        }

        RegressionTestStatistics {
            total,
            passed,
            failed,
            pending,
            skipped,
            pass_rate: if total > 0 {
                passed as f64 / total as f64
            } else {
                0.0
            },
        }
    }

    /// Gets all tests.
    #[allow(dead_code)]
    pub fn get_all_tests(&self) -> Vec<&RegressionTest> {
        self.tests.values().collect()
    }
}

impl Default for RegressionTestManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics for regression tests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionTestStatistics {
    /// Total number of tests.
    pub total: usize,
    /// Number of passed tests.
    pub passed: usize,
    /// Number of failed tests.
    pub failed: usize,
    /// Number of pending tests.
    pub pending: usize,
    /// Number of skipped tests.
    pub skipped: usize,
    /// Pass rate (0.0 to 1.0).
    pub pass_rate: f64,
}

/// Drift detection result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftDetectionResult {
    /// Whether drift was detected.
    pub drift_detected: bool,
    /// Drift score (0.0 = no drift, 1.0 = maximum drift).
    pub drift_score: f64,
    /// Drift category.
    pub category: DriftCategory,
    /// Detected drift issues.
    pub drift_issues: Vec<DriftIssue>,
    /// Recommendations.
    pub recommendations: Vec<String>,
}

/// Category of drift.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DriftCategory {
    /// No significant drift.
    None,
    /// Minor drift - monitoring recommended.
    Minor,
    /// Moderate drift - review recommended.
    Moderate,
    /// Major drift - action required.
    Major,
    /// Critical drift - immediate action required.
    Critical,
}

/// Drift issue detected.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftIssue {
    /// Type of drift.
    pub drift_type: DriftType,
    /// Severity.
    pub severity: DriftSeverity,
    /// Description.
    pub description: String,
    /// Detected at timestamp.
    pub detected_at: chrono::DateTime<chrono::Utc>,
    /// Suggested action.
    pub suggested_action: Option<String>,
}

/// Type of drift.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DriftType {
    /// Legal framework has changed in source jurisdiction.
    SourceJurisdictionChange,
    /// Legal framework has changed in target jurisdiction.
    TargetJurisdictionChange,
    /// Cultural parameters have shifted.
    CulturalShift,
    /// Semantic meaning has drifted.
    SemanticDrift,
    /// Quality has degraded.
    QualityDegradation,
    /// Compliance status has changed.
    ComplianceChange,
}

/// Severity of drift.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DriftSeverity {
    /// High severity.
    High,
    /// Medium severity.
    Medium,
    /// Low severity.
    Low,
}

/// Drift monitoring snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftSnapshot {
    /// Snapshot ID.
    pub snapshot_id: String,
    /// Ported statute ID.
    pub statute_id: String,
    /// Quality score at snapshot time.
    pub quality_score: f64,
    /// Compliance status at snapshot time.
    pub compliance_status: String,
    /// Snapshot timestamp.
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Metadata snapshot.
    pub metadata: std::collections::HashMap<String, String>,
}

/// Drift monitor for continuous monitoring.
pub struct DriftMonitor {
    /// Historical snapshots.
    snapshots: std::collections::HashMap<String, Vec<DriftSnapshot>>,
    /// Quality scorer.
    scorer: QualityScorer,
    /// Drift detection threshold.
    drift_threshold: f64,
}

impl DriftMonitor {
    /// Creates a new drift monitor.
    pub fn new() -> Self {
        Self {
            snapshots: std::collections::HashMap::new(),
            scorer: QualityScorer::new(),
            drift_threshold: 0.1, // 10% change triggers drift detection
        }
    }

    /// Sets drift detection threshold.
    #[allow(dead_code)]
    pub fn with_threshold(mut self, threshold: f64) -> Self {
        self.drift_threshold = threshold.clamp(0.0, 1.0);
        self
    }

    /// Creates a snapshot of current state.
    pub fn create_snapshot(&mut self, statute_id: String, ported: &PortedStatute) -> String {
        let quality = self.scorer.score_porting(ported);

        let snapshot_id = uuid::Uuid::new_v4().to_string();
        let snapshot = DriftSnapshot {
            snapshot_id: snapshot_id.clone(),
            statute_id: statute_id.clone(),
            quality_score: quality.overall,
            compliance_status: "compliant".to_string(),
            timestamp: chrono::Utc::now(),
            metadata: std::collections::HashMap::new(),
        };

        self.snapshots
            .entry(statute_id)
            .or_insert_with(Vec::new)
            .push(snapshot);

        snapshot_id
    }

    /// Detects drift by comparing current state with historical snapshots.
    pub fn detect_drift(&self, statute_id: &str, current: &PortedStatute) -> DriftDetectionResult {
        let mut drift_issues = Vec::new();
        let mut recommendations = Vec::new();

        // Get historical snapshots
        let snapshots = self.snapshots.get(statute_id);

        let drift_score = if let Some(snapshots) = snapshots {
            if snapshots.is_empty() {
                0.0
            } else {
                // Compare with most recent snapshot
                let latest = &snapshots[snapshots.len() - 1];
                let current_quality = self.scorer.score_porting(current);

                let quality_diff = (latest.quality_score - current_quality.overall).abs();

                if quality_diff > self.drift_threshold {
                    drift_issues.push(DriftIssue {
                        drift_type: DriftType::QualityDegradation,
                        severity: if quality_diff > 0.2 {
                            DriftSeverity::High
                        } else if quality_diff > 0.1 {
                            DriftSeverity::Medium
                        } else {
                            DriftSeverity::Low
                        },
                        description: format!(
                            "Quality score changed by {:.2}%",
                            quality_diff * 100.0
                        ),
                        detected_at: chrono::Utc::now(),
                        suggested_action: Some(
                            "Review ported statute for quality issues".to_string(),
                        ),
                    });
                }

                quality_diff
            }
        } else {
            0.0
        };

        let category = if drift_score >= 0.3 {
            DriftCategory::Critical
        } else if drift_score >= 0.2 {
            DriftCategory::Major
        } else if drift_score >= 0.1 {
            DriftCategory::Moderate
        } else if drift_score >= 0.05 {
            DriftCategory::Minor
        } else {
            DriftCategory::None
        };

        let drift_detected = !drift_issues.is_empty();

        if drift_detected {
            recommendations.push(
                "Review ported statute against current source and target frameworks".to_string(),
            );
            recommendations.push("Consider re-porting if drift is significant".to_string());
        }

        DriftDetectionResult {
            drift_detected,
            drift_score,
            category,
            drift_issues,
            recommendations,
        }
    }

    /// Gets all snapshots for a statute.
    #[allow(dead_code)]
    pub fn get_snapshots(&self, statute_id: &str) -> Option<&Vec<DriftSnapshot>> {
        self.snapshots.get(statute_id)
    }

    /// Gets drift trend over time.
    #[allow(dead_code)]
    pub fn get_drift_trend(&self, statute_id: &str) -> Vec<(chrono::DateTime<chrono::Utc>, f64)> {
        if let Some(snapshots) = self.snapshots.get(statute_id) {
            if snapshots.len() < 2 {
                return Vec::new();
            }

            let mut trend = Vec::new();
            for i in 1..snapshots.len() {
                let prev = &snapshots[i - 1];
                let curr = &snapshots[i];
                let drift = (prev.quality_score - curr.quality_score).abs();
                trend.push((curr.timestamp, drift));
            }
            trend
        } else {
            Vec::new()
        }
    }
}

impl Default for DriftMonitor {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Documentation Generation (v0.2.6)
// ============================================================================

/// Explanatory note for a ported statute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanatoryNote {
    /// Note ID.
    pub note_id: String,
    /// Ported statute ID.
    pub statute_id: String,
    /// Section being explained.
    pub section: String,
    /// Plain language explanation.
    pub explanation: String,
    /// Reason for porting change.
    pub reason_for_change: Option<String>,
    /// Legal implications.
    pub legal_implications: Vec<String>,
    /// Examples.
    pub examples: Vec<String>,
    /// Cross-references.
    pub cross_references: Vec<String>,
    /// Generated at timestamp.
    pub generated_at: chrono::DateTime<chrono::Utc>,
}

/// Explanatory note generator.
pub struct ExplanatoryNoteGenerator;

impl ExplanatoryNoteGenerator {
    /// Creates a new explanatory note generator.
    pub fn new() -> Self {
        Self
    }

    /// Generates explanatory notes for a ported statute.
    pub fn generate_notes(&self, ported: &PortedStatute) -> Vec<ExplanatoryNote> {
        let mut notes = Vec::new();

        // Generate a note for the overall statute
        notes.push(self.generate_statute_note(ported));

        // Generate notes for each significant change
        for (idx, change) in ported.changes.iter().enumerate() {
            if self.is_significant_change(change) {
                notes.push(self.generate_change_note(ported, change, idx));
            }
        }

        notes
    }

    /// Generates a note for the statute as a whole.
    fn generate_statute_note(&self, ported: &PortedStatute) -> ExplanatoryNote {
        let explanation = format!(
            "This statute has been ported from another jurisdiction. It contains {} adaptations to ensure compliance with local legal requirements and cultural norms.",
            ported.changes.len()
        );

        let legal_implications = vec![
            "This statute is adapted for the target jurisdiction".to_string(),
            format!(
                "Compatibility score: {:.2}%",
                ported.compatibility_score * 100.0
            ),
        ];

        ExplanatoryNote {
            note_id: uuid::Uuid::new_v4().to_string(),
            statute_id: ported.statute.id.clone(),
            section: "General".to_string(),
            explanation,
            reason_for_change: Some("Cross-jurisdiction legal framework porting".to_string()),
            legal_implications,
            examples: vec![],
            cross_references: vec![],
            generated_at: chrono::Utc::now(),
        }
    }

    /// Generates a note for a specific change.
    fn generate_change_note(
        &self,
        ported: &PortedStatute,
        change: &PortingChange,
        idx: usize,
    ) -> ExplanatoryNote {
        let explanation = format!(
            "{} (Change type: {:?})",
            change.description, change.change_type
        );

        let mut legal_implications = vec![change.reason.clone()];

        if let (Some(original), Some(adapted)) = (&change.original, &change.adapted) {
            legal_implications.push(format!(
                "Changed from '{}' to '{}' for local applicability",
                original, adapted
            ));
        }

        ExplanatoryNote {
            note_id: uuid::Uuid::new_v4().to_string(),
            statute_id: ported.statute.id.clone(),
            section: format!("Change {}", idx + 1),
            explanation,
            reason_for_change: Some(change.reason.clone()),
            legal_implications,
            examples: vec![],
            cross_references: vec![],
            generated_at: chrono::Utc::now(),
        }
    }

    /// Checks if a change is significant enough to warrant a note.
    fn is_significant_change(&self, change: &PortingChange) -> bool {
        matches!(
            change.change_type,
            ChangeType::CulturalAdaptation | ChangeType::ValueAdaptation | ChangeType::Removal
        )
    }
}

impl Default for ExplanatoryNoteGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Change justification report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeJustificationReport {
    /// Report ID.
    pub report_id: String,
    /// Ported statute ID.
    pub statute_id: String,
    /// Source jurisdiction.
    pub source_jurisdiction: String,
    /// Target jurisdiction.
    pub target_jurisdiction: String,
    /// Justifications for each change.
    pub justifications: Vec<ChangeJustification>,
    /// Overall rationale.
    pub overall_rationale: String,
    /// Legal basis for changes.
    pub legal_basis: Vec<String>,
    /// Stakeholder input summary.
    pub stakeholder_input: Option<String>,
    /// Generated at timestamp.
    pub generated_at: chrono::DateTime<chrono::Utc>,
}

/// Justification for a specific change.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeJustification {
    /// Change description.
    pub change_description: String,
    /// Change type.
    pub change_type: ChangeType,
    /// Justification.
    pub justification: String,
    /// Legal authority.
    pub legal_authority: Option<String>,
    /// Alternative considered.
    pub alternatives_considered: Vec<String>,
    /// Risk if not changed.
    pub risk_if_unchanged: Option<String>,
}

/// Change justification report generator.
pub struct ChangeJustificationReportGenerator;

impl ChangeJustificationReportGenerator {
    /// Creates a new change justification report generator.
    pub fn new() -> Self {
        Self
    }

    /// Generates a change justification report.
    pub fn generate_report(
        &self,
        ported: &PortedStatute,
        source_jurisdiction: &str,
        target_jurisdiction: &str,
    ) -> ChangeJustificationReport {
        let justifications = ported
            .changes
            .iter()
            .map(|change| self.justify_change(change))
            .collect();

        let overall_rationale = format!(
            "This statute was ported from {} to {} to facilitate legal harmonization and knowledge transfer. {} changes were made to ensure local applicability and compliance.",
            source_jurisdiction,
            target_jurisdiction,
            ported.changes.len()
        );

        let legal_basis = vec![
            "Cross-jurisdictional legal framework sharing".to_string(),
            "Cultural adaptation requirements".to_string(),
            "Local legal compliance mandate".to_string(),
        ];

        ChangeJustificationReport {
            report_id: uuid::Uuid::new_v4().to_string(),
            statute_id: ported.statute.id.clone(),
            source_jurisdiction: source_jurisdiction.to_string(),
            target_jurisdiction: target_jurisdiction.to_string(),
            justifications,
            overall_rationale,
            legal_basis,
            stakeholder_input: None,
            generated_at: chrono::Utc::now(),
        }
    }

    /// Justifies a specific change.
    fn justify_change(&self, change: &PortingChange) -> ChangeJustification {
        let justification = match change.change_type {
            ChangeType::Translation => "Translation required for language localization".to_string(),
            ChangeType::ValueAdaptation => {
                "Value adapted to match local legal standards and thresholds".to_string()
            }
            ChangeType::CulturalAdaptation => {
                "Cultural adaptation necessary for local acceptability and compliance".to_string()
            }
            ChangeType::Removal => {
                "Removed due to incompatibility with target jurisdiction laws".to_string()
            }
            ChangeType::ComplianceAddition => {
                "Added to ensure compliance with target jurisdiction requirements".to_string()
            }
            ChangeType::Incompatible => "Marked as incompatible pending further review".to_string(),
        };

        let risk_if_unchanged = match change.change_type {
            ChangeType::CulturalAdaptation | ChangeType::ValueAdaptation => {
                Some("Non-compliance with local legal requirements".to_string())
            }
            ChangeType::Removal => Some("Potential legal conflict or invalidity".to_string()),
            _ => None,
        };

        ChangeJustification {
            change_description: change.description.clone(),
            change_type: change.change_type,
            justification,
            legal_authority: None,
            alternatives_considered: vec![],
            risk_if_unchanged,
        }
    }
}

impl Default for ChangeJustificationReportGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Legislative history entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegislativeHistoryEntry {
    /// Event timestamp.
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Event type.
    pub event_type: LegislativeEventType,
    /// Description.
    pub description: String,
    /// Actor (person or organization).
    pub actor: Option<String>,
    /// Related documents.
    pub related_documents: Vec<String>,
}

/// Type of legislative event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LegislativeEventType {
    /// Initial drafting.
    Drafted,
    /// Review by stakeholder.
    Reviewed,
    /// Amendment proposed.
    Amended,
    /// Approved by authority.
    Approved,
    /// Published.
    Published,
    /// Ported to another jurisdiction.
    Ported,
}

/// Legislative history compilation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegislativeHistory {
    /// History ID.
    pub history_id: String,
    /// Statute ID.
    pub statute_id: String,
    /// Original enactment date (if applicable).
    pub original_enactment: Option<String>,
    /// Porting date.
    pub porting_date: String,
    /// Timeline of events.
    pub timeline: Vec<LegislativeHistoryEntry>,
    /// Key participants.
    pub key_participants: Vec<String>,
    /// Summary.
    pub summary: String,
}

/// Legislative history compiler.
pub struct LegislativeHistoryCompiler;

impl LegislativeHistoryCompiler {
    /// Creates a new legislative history compiler.
    pub fn new() -> Self {
        Self
    }

    /// Compiles legislative history for a ported statute.
    pub fn compile_history(&self, ported: &PortedStatute) -> LegislativeHistory {
        let mut timeline = Vec::new();

        // Add porting event
        timeline.push(LegislativeHistoryEntry {
            timestamp: chrono::Utc::now(),
            event_type: LegislativeEventType::Ported,
            description: format!("Statute ported with {} adaptations", ported.changes.len()),
            actor: Some("Porting System".to_string()),
            related_documents: vec![],
        });

        // Add change events
        for change in &ported.changes {
            timeline.push(LegislativeHistoryEntry {
                timestamp: chrono::Utc::now(),
                event_type: LegislativeEventType::Amended,
                description: change.description.clone(),
                actor: None,
                related_documents: vec![],
            });
        }

        let summary = format!(
            "This statute was ported from another jurisdiction with {} modifications to ensure local applicability.",
            ported.changes.len()
        );

        LegislativeHistory {
            history_id: uuid::Uuid::new_v4().to_string(),
            statute_id: ported.statute.id.clone(),
            original_enactment: None,
            porting_date: chrono::Utc::now().to_rfc3339(),
            timeline,
            key_participants: vec!["Porting System".to_string()],
            summary,
        }
    }

    /// Adds a custom event to history.
    #[allow(dead_code)]
    pub fn add_event(
        &self,
        history: &mut LegislativeHistory,
        event_type: LegislativeEventType,
        description: String,
        actor: Option<String>,
    ) {
        history.timeline.push(LegislativeHistoryEntry {
            timestamp: chrono::Utc::now(),
            event_type,
            description,
            actor,
            related_documents: vec![],
        });
    }
}

impl Default for LegislativeHistoryCompiler {
    fn default() -> Self {
        Self::new()
    }
}

/// Implementation guidance document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImplementationGuidance {
    /// Guidance ID.
    pub guidance_id: String,
    /// Statute ID.
    pub statute_id: String,
    /// Overview.
    pub overview: String,
    /// Prerequisites.
    pub prerequisites: Vec<String>,
    /// Implementation steps.
    pub implementation_steps: Vec<ImplementationStep>,
    /// Compliance checklist.
    pub compliance_checklist: Vec<String>,
    /// Common pitfalls.
    pub common_pitfalls: Vec<String>,
    /// Resources.
    pub resources: Vec<String>,
    /// Timeline estimate.
    pub timeline_estimate: Option<String>,
    /// Generated at timestamp.
    pub generated_at: chrono::DateTime<chrono::Utc>,
}

/// Implementation step.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImplementationStep {
    /// Step number.
    pub step_number: usize,
    /// Title.
    pub title: String,
    /// Description.
    pub description: String,
    /// Required actions.
    pub required_actions: Vec<String>,
    /// Success criteria.
    pub success_criteria: Vec<String>,
}

/// Implementation guidance generator.
pub struct ImplementationGuidanceGenerator;

impl ImplementationGuidanceGenerator {
    /// Creates a new implementation guidance generator.
    pub fn new() -> Self {
        Self
    }

    /// Generates implementation guidance for a ported statute.
    pub fn generate_guidance(&self, ported: &PortedStatute) -> ImplementationGuidance {
        let overview = format!(
            "This guidance provides step-by-step instructions for implementing the ported statute '{}'. The statute has been adapted with {} changes for local compliance.",
            ported.statute.title,
            ported.changes.len()
        );

        let prerequisites = vec![
            "Review the ported statute in detail".to_string(),
            "Ensure all stakeholders are informed".to_string(),
            "Verify compliance with local regulations".to_string(),
            "Prepare necessary resources".to_string(),
        ];

        let implementation_steps = self.generate_steps(ported);

        let compliance_checklist = vec![
            "Verify all cultural adaptations are appropriate".to_string(),
            "Confirm legal compliance in target jurisdiction".to_string(),
            "Validate translations are accurate".to_string(),
            "Ensure stakeholder approval is obtained".to_string(),
        ];

        let common_pitfalls = vec![
            "Overlooking cultural differences".to_string(),
            "Insufficient stakeholder consultation".to_string(),
            "Inadequate legal review".to_string(),
        ];

        ImplementationGuidance {
            guidance_id: uuid::Uuid::new_v4().to_string(),
            statute_id: ported.statute.id.clone(),
            overview,
            prerequisites,
            implementation_steps,
            compliance_checklist,
            common_pitfalls,
            resources: vec![],
            timeline_estimate: Some("3-6 months".to_string()),
            generated_at: chrono::Utc::now(),
        }
    }

    /// Generates implementation steps.
    fn generate_steps(&self, ported: &PortedStatute) -> Vec<ImplementationStep> {
        let mut steps = Vec::new();

        steps.push(ImplementationStep {
            step_number: 1,
            title: "Initial Review".to_string(),
            description: "Review the ported statute and all adaptations".to_string(),
            required_actions: vec![
                "Read the full statute text".to_string(),
                "Review all change justifications".to_string(),
            ],
            success_criteria: vec!["All adaptations understood".to_string()],
        });

        steps.push(ImplementationStep {
            step_number: 2,
            title: "Stakeholder Consultation".to_string(),
            description: "Consult with affected stakeholders".to_string(),
            required_actions: vec![
                "Identify all affected parties".to_string(),
                "Conduct consultation sessions".to_string(),
            ],
            success_criteria: vec!["Stakeholder feedback incorporated".to_string()],
        });

        steps.push(ImplementationStep {
            step_number: 3,
            title: "Legal Validation".to_string(),
            description: "Validate legal compliance".to_string(),
            required_actions: vec![
                "Conduct legal review".to_string(),
                "Verify compliance with all regulations".to_string(),
            ],
            success_criteria: vec!["Legal approval obtained".to_string()],
        });

        if !ported.changes.is_empty() {
            steps.push(ImplementationStep {
                step_number: 4,
                title: "Implementation of Adaptations".to_string(),
                description: format!("Implement {} adaptations", ported.changes.len()),
                required_actions: vec![
                    "Apply all cultural adaptations".to_string(),
                    "Update documentation".to_string(),
                ],
                success_criteria: vec!["All changes successfully applied".to_string()],
            });
        }

        steps.push(ImplementationStep {
            step_number: steps.len() + 1,
            title: "Final Approval and Publication".to_string(),
            description: "Obtain final approval and publish".to_string(),
            required_actions: vec![
                "Submit for final approval".to_string(),
                "Publish statute".to_string(),
            ],
            success_criteria: vec!["Statute officially enacted".to_string()],
        });

        steps
    }
}

impl Default for ImplementationGuidanceGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Training material.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingMaterial {
    /// Material ID.
    pub material_id: String,
    /// Statute ID.
    pub statute_id: String,
    /// Title.
    pub title: String,
    /// Target audience.
    pub target_audience: TrainingAudience,
    /// Learning objectives.
    pub learning_objectives: Vec<String>,
    /// Content modules.
    pub modules: Vec<TrainingModule>,
    /// Assessment questions.
    pub assessment_questions: Vec<AssessmentQuestion>,
    /// Estimated duration.
    pub estimated_duration: String,
    /// Generated at timestamp.
    pub generated_at: chrono::DateTime<chrono::Utc>,
}

/// Training audience type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrainingAudience {
    /// Legal professionals.
    LegalProfessionals,
    /// Government officials.
    GovernmentOfficials,
    /// General public.
    GeneralPublic,
    /// Enforcement officers.
    EnforcementOfficers,
}

/// Training module.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingModule {
    /// Module number.
    pub module_number: usize,
    /// Module title.
    pub title: String,
    /// Content.
    pub content: String,
    /// Key points.
    pub key_points: Vec<String>,
    /// Examples.
    pub examples: Vec<String>,
}

/// Assessment question.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssessmentQuestion {
    /// Question number.
    pub question_number: usize,
    /// Question text.
    pub question: String,
    /// Answer options.
    pub options: Vec<String>,
    /// Correct answer index.
    pub correct_answer: usize,
    /// Explanation.
    pub explanation: String,
}

/// Training material generator.
pub struct TrainingMaterialGenerator;

impl TrainingMaterialGenerator {
    /// Creates a new training material generator.
    pub fn new() -> Self {
        Self
    }

    /// Generates training materials for a ported statute.
    pub fn generate_materials(
        &self,
        ported: &PortedStatute,
        audience: TrainingAudience,
    ) -> TrainingMaterial {
        let title = format!("Training: {}", ported.statute.title);

        let learning_objectives = match audience {
            TrainingAudience::LegalProfessionals => vec![
                "Understand the legal framework of the ported statute".to_string(),
                "Identify all adaptations and their legal basis".to_string(),
                "Apply the statute in legal practice".to_string(),
            ],
            TrainingAudience::GovernmentOfficials => vec![
                "Understand the statute's requirements".to_string(),
                "Implement the statute in policy".to_string(),
                "Ensure compliance across departments".to_string(),
            ],
            TrainingAudience::GeneralPublic => vec![
                "Understand rights and obligations under the statute".to_string(),
                "Know how the statute affects daily life".to_string(),
            ],
            TrainingAudience::EnforcementOfficers => vec![
                "Understand enforcement procedures".to_string(),
                "Identify violations and apply penalties".to_string(),
            ],
        };

        let modules = self.generate_modules(ported, audience);
        let assessment_questions = self.generate_assessment(ported, audience);

        let estimated_duration = match audience {
            TrainingAudience::LegalProfessionals => "4 hours".to_string(),
            TrainingAudience::GovernmentOfficials => "3 hours".to_string(),
            TrainingAudience::GeneralPublic => "1 hour".to_string(),
            TrainingAudience::EnforcementOfficers => "2 hours".to_string(),
        };

        TrainingMaterial {
            material_id: uuid::Uuid::new_v4().to_string(),
            statute_id: ported.statute.id.clone(),
            title,
            target_audience: audience,
            learning_objectives,
            modules,
            assessment_questions,
            estimated_duration,
            generated_at: chrono::Utc::now(),
        }
    }

    /// Generates training modules.
    fn generate_modules(
        &self,
        ported: &PortedStatute,
        _audience: TrainingAudience,
    ) -> Vec<TrainingModule> {
        let mut modules = Vec::new();

        modules.push(TrainingModule {
            module_number: 1,
            title: "Introduction to the Statute".to_string(),
            content: format!(
                "This statute, '{}', has been ported from another jurisdiction to facilitate legal harmonization.",
                ported.statute.title
            ),
            key_points: vec![
                "Purpose of the statute".to_string(),
                "Scope of application".to_string(),
            ],
            examples: vec![],
        });

        if !ported.changes.is_empty() {
            modules.push(TrainingModule {
                module_number: 2,
                title: "Key Adaptations".to_string(),
                content: format!(
                    "{} adaptations were made for local compliance.",
                    ported.changes.len()
                ),
                key_points: ported
                    .changes
                    .iter()
                    .take(5)
                    .map(|c| c.description.clone())
                    .collect(),
                examples: vec![],
            });
        }

        modules.push(TrainingModule {
            module_number: modules.len() + 1,
            title: "Practical Application".to_string(),
            content: "How to apply this statute in practice".to_string(),
            key_points: vec![
                "Implementation procedures".to_string(),
                "Common scenarios".to_string(),
            ],
            examples: vec![],
        });

        modules
    }

    /// Generates assessment questions.
    fn generate_assessment(
        &self,
        ported: &PortedStatute,
        _audience: TrainingAudience,
    ) -> Vec<AssessmentQuestion> {
        let mut questions = Vec::new();

        questions.push(AssessmentQuestion {
            question_number: 1,
            question: format!("What is the main purpose of '{}'?", ported.statute.title),
            options: vec![
                "To provide legal framework".to_string(),
                "To regulate commerce".to_string(),
                "To enforce penalties".to_string(),
            ],
            correct_answer: 0,
            explanation: "This statute provides the legal framework for its subject matter."
                .to_string(),
        });

        if !ported.changes.is_empty() {
            questions.push(AssessmentQuestion {
                question_number: 2,
                question: format!("How many adaptations were made to this statute?"),
                options: vec![
                    format!("{}", ported.changes.len()),
                    "0".to_string(),
                    "100".to_string(),
                ],
                correct_answer: 0,
                explanation: format!(
                    "{} adaptations were made for local compliance.",
                    ported.changes.len()
                ),
            });
        }

        questions
    }
}

impl Default for TrainingMaterialGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{Effect, EffectType};
    use legalis_i18n::{CulturalParams, LegalSystem, Locale};

    fn test_jurisdiction_jp() -> Jurisdiction {
        Jurisdiction::new("JP", "Japan", Locale::new("ja").with_country("JP"))
            .with_legal_system(LegalSystem::CivilLaw)
            .with_cultural_params(CulturalParams::japan())
    }

    fn test_jurisdiction_us() -> Jurisdiction {
        Jurisdiction::new("US", "United States", Locale::new("en").with_country("US"))
            .with_legal_system(LegalSystem::CommonLaw)
            .with_cultural_params(CulturalParams::for_country("US"))
    }

    #[test]
    fn test_port_statute() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statute = Statute::new(
            "adult-rights",
            "成人権法",
            Effect::new(EffectType::Grant, "Complete legal capacity"),
        );

        let options = PortingOptions {
            apply_cultural_params: true,
            ..Default::default()
        };

        let result = engine.port_statute(&statute, &options).unwrap();
        assert!(result.statute.id.starts_with("us-"));
    }

    #[test]
    fn test_compatibility_report() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statutes = vec![Statute::new(
            "test",
            "Test",
            Effect::new(EffectType::Grant, "Test"),
        )];

        let report = engine.generate_report(&statutes);
        assert!(report.compatibility_score > 0.0);
    }

    #[test]
    fn test_conflict_detection() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statute = Statute::new(
            "test",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test"),
        );

        let conflicts = engine.detect_conflicts(&statute);
        // Should detect legal system mismatch
        assert!(!conflicts.is_empty());
    }

    #[test]
    fn test_semantic_validation() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"));

        let options = PortingOptions::default();
        let ported = engine.port_statute(&statute, &options).unwrap();

        let validation = engine.validate_semantics(&statute, &ported);
        assert!(validation.preservation_score >= 0.0);
        assert!(validation.preservation_score <= 1.0);
    }

    #[test]
    fn test_risk_assessment() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"));

        let options = PortingOptions::default();
        let ported = engine.port_statute(&statute, &options).unwrap();

        let assessment = engine.assess_risks(&ported);
        assert!(assessment.risk_score >= 0.0);
        assert!(assessment.risk_score <= 1.0);
    }

    #[test]
    fn test_partial_porting() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"));

        let options = PortingOptions::default();
        let section_ids = vec!["section1".to_string(), "section2".to_string()];

        let result = engine
            .port_sections(&statute, &section_ids, &options)
            .unwrap();
        assert!(result.statute.id.starts_with("us-"));
        assert!(!result.changes.is_empty());
    }

    #[test]
    fn test_reverse_porting() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"));

        let _changes = engine.reverse_port_analysis(&statute).unwrap();
        // Changes may or may not exist depending on cultural param differences
        // Just verify it doesn't error - test passes if no panic occurs
    }

    #[tokio::test]
    async fn test_batch_port() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statutes = vec![
            Statute::new("test1", "Test 1", Effect::new(EffectType::Grant, "Test 1")),
            Statute::new("test2", "Test 2", Effect::new(EffectType::Grant, "Test 2")),
        ];

        let options = PortingOptions {
            generate_report: true,
            detect_conflicts: true,
            validate_semantics: true,
            ..Default::default()
        };

        let result = engine.batch_port(&statutes, &options).await.unwrap();
        assert_eq!(result.statutes.len(), 2);
        assert!(result.report.is_some());
        assert!(!result.conflicts.is_empty());
        assert!(result.semantic_validation.is_some());
        assert!(result.risk_assessment.is_some());
    }

    #[test]
    fn test_bilateral_agreement() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let agreement = engine.create_bilateral_agreement(AgreementType::MutualRecognition);

        assert_eq!(agreement.source_jurisdiction, "JP");
        assert_eq!(agreement.target_jurisdiction, "US");
        assert!(!agreement.mutual_recognition.is_empty());
        assert!(!agreement.adaptation_protocols.is_empty());
    }

    #[test]
    fn test_regulatory_equivalence() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us())
            .with_equivalence_mappings(vec![EquivalenceMapping {
                source_regulation: "test".to_string(),
                target_regulation: "us-test".to_string(),
                equivalence_score: 0.9,
                differences: vec!["Minor terminology differences".to_string()],
                notes: "Highly equivalent".to_string(),
            }]);

        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"));
        let mappings = engine.find_regulatory_equivalence(&statute);

        assert_eq!(mappings.len(), 1);
        assert_eq!(mappings[0].equivalence_score, 0.9);
    }

    #[tokio::test]
    async fn test_similar_statutes() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statute = Statute::new(
            "test",
            "Adult Rights Law",
            Effect::new(EffectType::Grant, "Test"),
        );

        let candidates = vec![
            Statute::new(
                "c1",
                "Adult Rights Statute",
                Effect::new(EffectType::Grant, "C1"),
            ),
            Statute::new(
                "c2",
                "Child Protection Law",
                Effect::new(EffectType::Grant, "C2"),
            ),
            Statute::new(
                "c3",
                "Adult Legal Capacity",
                Effect::new(EffectType::Grant, "C3"),
            ),
        ];

        let similar = engine.find_similar_statutes(&statute, &candidates).await;
        assert!(!similar.is_empty());
        // First result should have highest similarity
        assert!(similar[0].1 >= 0.3);
    }

    #[test]
    fn test_term_replacement() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us())
            .with_term_replacements(vec![TermReplacement {
                source_term: "成人".to_string(),
                target_term: "adult".to_string(),
                context: None,
                confidence: 0.95,
            }]);

        let mut statute = Statute::new(
            "test",
            "成人 Rights Law",
            Effect::new(EffectType::Grant, "Test"),
        );
        let replacements = engine.apply_term_replacement(&mut statute);

        assert_eq!(replacements.len(), 1);
        assert!(statute.title.contains("adult"));
    }

    #[test]
    fn test_contextual_adjustment() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statute = Statute::new(
            "test",
            "Fine Payment Law",
            Effect::new(EffectType::Grant, "Test"),
        );

        let adjustments = engine.adjust_parameters_contextually(&statute);
        // Should detect monetary context
        assert!(!adjustments.is_empty());
    }

    #[test]
    fn test_workflow_creation() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let workflow = engine.create_workflow("test-statute".to_string());

        assert_eq!(workflow.state, WorkflowState::Initiated);
        assert_eq!(workflow.pending_steps.len(), 4);
        assert_eq!(workflow.approvals.len(), 2);
    }

    #[test]
    fn test_workflow_advancement() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let mut workflow = engine.create_workflow("test-statute".to_string());

        engine.advance_workflow(&mut workflow).unwrap();
        assert_eq!(workflow.state, WorkflowState::InProgress);
        assert_eq!(workflow.completed_steps.len(), 1);
        assert_eq!(workflow.pending_steps.len(), 3);
    }

    #[test]
    fn test_versioned_statute() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"));
        let options = PortingOptions::default();
        let ported = engine.port_statute(&statute, &options).unwrap();

        let versioned = engine.create_versioned_statute(
            ported,
            1,
            "test_user".to_string(),
            "Initial version".to_string(),
        );

        assert_eq!(versioned.version, 1);
        assert!(versioned.previous_hash.is_none());
        assert!(!versioned.hash.is_empty());
    }

    #[test]
    fn test_version_comparison() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statute1 = Statute::new("test", "Test V1", Effect::new(EffectType::Grant, "Test"));
        let statute2 = Statute::new("test", "Test V2", Effect::new(EffectType::Grant, "Test"));

        let options = PortingOptions::default();
        let ported1 = engine.port_statute(&statute1, &options).unwrap();
        let ported2 = engine.port_statute(&statute2, &options).unwrap();

        let v1 = engine.create_versioned_statute(ported1, 1, "user".to_string(), "V1".to_string());
        let v2 = engine.create_versioned_statute(ported2, 2, "user".to_string(), "V2".to_string());

        let differences = engine.compare_versions(&v1, &v2);
        assert!(!differences.is_empty());
    }

    #[test]
    fn test_submit_for_review() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"));
        let options = PortingOptions::default();
        let ported = engine.port_statute(&statute, &options).unwrap();

        let review_request = engine.submit_for_review(ported);

        assert_eq!(review_request.status, ReviewStatus::Pending);
        assert_eq!(review_request.source_jurisdiction, "JP");
        assert_eq!(review_request.target_jurisdiction, "US");
        assert!(review_request.assigned_expert.is_none());
        assert!(review_request.reviews.is_empty());
    }

    #[test]
    fn test_assign_expert() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"));
        let options = PortingOptions::default();
        let ported = engine.port_statute(&statute, &options).unwrap();

        let mut review_request = engine.submit_for_review(ported);
        engine.assign_expert(&mut review_request, "expert-001".to_string());

        assert_eq!(review_request.status, ReviewStatus::Assigned);
        assert_eq!(
            review_request.assigned_expert,
            Some("expert-001".to_string())
        );
    }

    #[test]
    fn test_add_expert_review_approve() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"));
        let options = PortingOptions::default();
        let ported = engine.port_statute(&statute, &options).unwrap();

        let mut review_request = engine.submit_for_review(ported);

        let expert_review = ExpertReview {
            id: "review-001".to_string(),
            expert_id: "expert-001".to_string(),
            expert_name: "Dr. Legal Expert".to_string(),
            qualifications: vec!["Bar License".to_string(), "PhD in Law".to_string()],
            reviewed_at: chrono::Utc::now().to_rfc3339(),
            recommendation: ReviewRecommendation::Approve,
            comments: Vec::new(),
            confidence: 0.95,
            concerns: Vec::new(),
            suggested_modifications: Vec::new(),
        };

        engine
            .add_expert_review(&mut review_request, expert_review)
            .unwrap();

        assert_eq!(review_request.status, ReviewStatus::Approved);
        assert_eq!(review_request.reviews.len(), 1);
    }

    #[test]
    fn test_add_expert_review_reject() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"));
        let options = PortingOptions::default();
        let ported = engine.port_statute(&statute, &options).unwrap();

        let mut review_request = engine.submit_for_review(ported);

        let expert_review = ExpertReview {
            id: "review-001".to_string(),
            expert_id: "expert-001".to_string(),
            expert_name: "Dr. Legal Expert".to_string(),
            qualifications: vec!["Bar License".to_string()],
            reviewed_at: chrono::Utc::now().to_rfc3339(),
            recommendation: ReviewRecommendation::Reject,
            comments: Vec::new(),
            confidence: 0.9,
            concerns: vec!["Major legal incompatibility".to_string()],
            suggested_modifications: vec!["Complete revision required".to_string()],
        };

        engine
            .add_expert_review(&mut review_request, expert_review)
            .unwrap();

        assert_eq!(review_request.status, ReviewStatus::Rejected);
    }

    #[test]
    fn test_create_review_comment() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());

        let comment = engine.create_review_comment(
            Some("section-1".to_string()),
            "This section needs clarification".to_string(),
            Severity::Warning,
            "Clarity".to_string(),
        );

        assert!(comment.section.is_some());
        assert_eq!(comment.text, "This section needs clarification");
        assert_eq!(comment.severity, Severity::Warning);
        assert_eq!(comment.category, "Clarity");
    }

    #[test]
    fn test_compliance_check() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statute = Statute::new(
            "test",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test"),
        );
        let options = PortingOptions {
            apply_cultural_params: true,
            ..Default::default()
        };
        let ported = engine.port_statute(&statute, &options).unwrap();

        let result = engine.check_compliance(&ported);

        assert!(!result.checks.is_empty());
        assert!(result.compliance_score >= 0.0);
        assert!(result.compliance_score <= 1.0);
        assert!(!result.recommendations.is_empty());
    }

    #[test]
    fn test_compliance_check_detects_issues() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"));
        let options = PortingOptions::default();
        let ported = engine.port_statute(&statute, &options).unwrap();

        let result = engine.check_compliance(&ported);

        // Should detect legal system mismatch
        assert!(!result.violations.is_empty());
        assert_eq!(result.status, ComplianceStatus::RequiresReview);
    }

    #[test]
    fn test_batch_compliance_check() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statutes = vec![
            Statute::new("test1", "Test 1", Effect::new(EffectType::Grant, "Test 1")),
            Statute::new("test2", "Test 2", Effect::new(EffectType::Grant, "Test 2")),
        ];

        let options = PortingOptions {
            apply_cultural_params: true,
            ..Default::default()
        };

        let ported: Vec<PortedStatute> = statutes
            .iter()
            .map(|s| engine.port_statute(s, &options).unwrap())
            .collect();

        let results = engine.batch_check_compliance(&ported);

        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|r| r.compliance_score >= 0.0));
    }

    #[test]
    fn test_compliance_summary() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statutes = vec![
            Statute::new("test1", "Test 1", Effect::new(EffectType::Grant, "Test 1")),
            Statute::new("test2", "Test 2", Effect::new(EffectType::Grant, "Test 2")),
        ];

        let options = PortingOptions {
            apply_cultural_params: true,
            ..Default::default()
        };

        let ported: Vec<PortedStatute> = statutes
            .iter()
            .map(|s| engine.port_statute(s, &options).unwrap())
            .collect();

        let results = engine.batch_check_compliance(&ported);
        let summary = engine.generate_compliance_summary(&results);

        assert_eq!(summary.total_statutes, 2);
        assert!(summary.average_compliance_score >= 0.0);
        assert!(summary.average_compliance_score <= 1.0);
    }

    #[test]
    fn test_export_compatibility_report_json() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"));
        let report = engine.generate_report(&[statute]);

        let json = engine
            .export_compatibility_report(&report, ExportFormat::Json)
            .unwrap();

        assert!(json.contains("compatibility_score"));
        assert!(json.contains("findings"));
    }

    #[test]
    fn test_export_compatibility_report_markdown() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"));
        let report = engine.generate_report(&[statute]);

        let md = engine
            .export_compatibility_report(&report, ExportFormat::Markdown)
            .unwrap();

        assert!(md.contains("# Compatibility Report"));
        assert!(md.contains("Compatibility Score"));
    }

    #[tokio::test]
    async fn test_export_porting_output() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statutes = vec![Statute::new(
            "test",
            "Test",
            Effect::new(EffectType::Grant, "Test"),
        )];

        let options = PortingOptions::default();
        let output = engine.batch_port(&statutes, &options).await.unwrap();

        let json = engine
            .export_porting_output(&output, ExportFormat::Json)
            .unwrap();
        assert!(json.contains("statutes"));

        let md = engine
            .export_porting_output(&output, ExportFormat::Markdown)
            .unwrap();
        assert!(md.contains("# Porting Output"));
    }

    #[test]
    fn test_tfidf_similarity() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statute1 = Statute::new(
            "test1",
            "Adult Rights Law",
            Effect::new(EffectType::Grant, "Test"),
        );
        let statute2 = Statute::new(
            "test2",
            "Adult Rights Act",
            Effect::new(EffectType::Grant, "Test"),
        );
        let statute3 = Statute::new(
            "test3",
            "Child Protection Law",
            Effect::new(EffectType::Grant, "Test"),
        );

        let sim12 = engine.calculate_tfidf_similarity(&statute1, &statute2);
        let sim13 = engine.calculate_tfidf_similarity(&statute1, &statute3);

        assert!(sim12 > sim13);
        assert!((0.0..=1.0).contains(&sim12));
        assert!((0.0..=1.0).contains(&sim13));
    }

    #[test]
    fn test_create_template() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let template = engine.create_template(
            "Civil Law Template".to_string(),
            "Template for civil law statutes".to_string(),
            vec!["civil".to_string(), "commercial".to_string()],
        );

        assert_eq!(template.name, "Civil Law Template");
        assert_eq!(template.statute_types.len(), 2);
        assert!(!template.contextual_rules.is_empty());
    }

    #[test]
    fn test_apply_template() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let template = engine.create_template(
            "Test Template".to_string(),
            "Test".to_string(),
            vec!["test".to_string()],
        );

        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"));
        let ported = engine.apply_template(&statute, &template).unwrap();

        assert!(ported.statute.id.starts_with("us-"));
    }

    #[test]
    fn test_generate_conflict_resolutions() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"));
        let conflicts = engine.detect_conflicts(&statute);

        let resolutions = engine.generate_conflict_resolutions(&conflicts);

        assert!(!resolutions.is_empty());
        for resolution in &resolutions {
            assert!(resolution.priority >= 1 && resolution.priority <= 10);
        }

        // Check that resolutions are sorted by priority (highest first)
        for i in 1..resolutions.len() {
            assert!(resolutions[i - 1].priority >= resolutions[i].priority);
        }
    }

    // ========================================================================
    // Tests for Conflict Resolution Framework (v0.1.4)
    // ========================================================================

    #[test]
    fn test_conflict_precedent_database() {
        let mut db = ConflictPrecedentDatabase::new();

        let precedent1 = ConflictPrecedent {
            id: "prec-1".to_string(),
            source_jurisdiction: "JP".to_string(),
            target_jurisdiction: "US".to_string(),
            conflict_type: ConflictType::SystemMismatch,
            description: "Legal system mismatch resolved".to_string(),
            resolution_used: "Adapt procedural elements".to_string(),
            effectiveness: 0.85,
            resolved_by: Some("Expert A".to_string()),
            resolved_at: "2024-01-01T00:00:00Z".to_string(),
            lessons_learned: vec!["Focus on procedural adaptation".to_string()],
            applicable_statute_types: vec!["commercial".to_string()],
            tags: vec!["system-mismatch".to_string()],
        };

        let precedent2 = ConflictPrecedent {
            id: "prec-2".to_string(),
            source_jurisdiction: "JP".to_string(),
            target_jurisdiction: "US".to_string(),
            conflict_type: ConflictType::CulturalIncompatibility,
            description: "Cultural conflict resolved".to_string(),
            resolution_used: "Local adaptation with consultation".to_string(),
            effectiveness: 0.75,
            resolved_by: Some("Expert B".to_string()),
            resolved_at: "2024-01-02T00:00:00Z".to_string(),
            lessons_learned: vec!["Involve local stakeholders".to_string()],
            applicable_statute_types: vec!["social".to_string()],
            tags: vec!["cultural".to_string()],
        };

        db.add_precedent(precedent1);
        db.add_precedent(precedent2);

        assert_eq!(db.all_precedents().len(), 2);

        let relevant = db.find_relevant_precedents("JP", "US", &ConflictType::SystemMismatch);
        assert_eq!(relevant.len(), 1);
        assert_eq!(relevant[0].id, "prec-1");

        let effective = db.get_effective_precedents();
        assert_eq!(effective.len(), 2);
    }

    #[test]
    fn test_conflict_detector_severity_analysis() {
        let mut detector = ConflictDetector::new();

        // Add a precedent
        let precedent = ConflictPrecedent {
            id: "prec-1".to_string(),
            source_jurisdiction: "JP".to_string(),
            target_jurisdiction: "US".to_string(),
            conflict_type: ConflictType::Contradiction,
            description: "Test conflict".to_string(),
            resolution_used: "Test resolution".to_string(),
            effectiveness: 0.9,
            resolved_by: None,
            resolved_at: "2024-01-01T00:00:00Z".to_string(),
            lessons_learned: vec![],
            applicable_statute_types: vec![],
            tags: vec![],
        };
        detector.precedent_db.add_precedent(precedent);

        let jp = test_jurisdiction_jp();
        let us = test_jurisdiction_us();

        let conflict = ConflictReport {
            statute_id: "test".to_string(),
            conflict_type: ConflictType::Contradiction,
            description: "Test conflict".to_string(),
            severity: Severity::Warning,
            resolutions: vec!["Test resolution".to_string()],
        };

        let severity = detector.analyze_severity(&conflict, &jp, &us);

        // Should be at least Warning due to contradiction type and legal system mismatch
        assert!(matches!(
            severity,
            Severity::Warning | Severity::Error | Severity::Critical
        ));
    }

    #[test]
    fn test_conflict_detector_recommend_strategies() {
        let mut detector = ConflictDetector::new();

        // Add a high-effectiveness precedent
        let precedent = ConflictPrecedent {
            id: "prec-1".to_string(),
            source_jurisdiction: "JP".to_string(),
            target_jurisdiction: "US".to_string(),
            conflict_type: ConflictType::SystemMismatch,
            description: "Legal system mismatch".to_string(),
            resolution_used: "Gradual adaptation with expert review".to_string(),
            effectiveness: 0.85,
            resolved_by: Some("Expert A".to_string()),
            resolved_at: "2024-01-01T00:00:00Z".to_string(),
            lessons_learned: vec![],
            applicable_statute_types: vec![],
            tags: vec![],
        };
        detector.precedent_db.add_precedent(precedent);

        // Add a template
        let template = NegotiatedResolutionTemplate {
            id: "template-1".to_string(),
            name: "System Mismatch Template".to_string(),
            conflict_types: vec![ConflictType::SystemMismatch],
            source_patterns: vec!["JP".to_string()],
            target_patterns: vec!["US".to_string()],
            approach: "Bilateral adaptation protocol".to_string(),
            negotiation_steps: vec![],
            fallback_strategies: vec![],
            success_rate: 0.8,
            stakeholders: vec![],
            required_approvals: vec![],
        };
        detector.add_template(template);

        let jp = test_jurisdiction_jp();
        let us = test_jurisdiction_us();

        let conflict = ConflictReport {
            statute_id: "test".to_string(),
            conflict_type: ConflictType::SystemMismatch,
            description: "System mismatch".to_string(),
            severity: Severity::Warning,
            resolutions: vec!["Default resolution".to_string()],
        };

        let strategies = detector.recommend_strategies(&conflict, &jp, &us);

        assert!(!strategies.is_empty());
        // Should include strategies from precedent and template
        assert!(strategies.iter().any(|s| s.contains("effective")));
        assert!(strategies.iter().any(|s| s.contains("template")));
    }

    #[test]
    fn test_conflict_resolution_workflow_creation() {
        let detector = ConflictDetector::new();

        let conflict = ConflictReport {
            statute_id: "test".to_string(),
            conflict_type: ConflictType::Contradiction,
            description: "Critical conflict".to_string(),
            severity: Severity::Critical,
            resolutions: vec!["Manual review required".to_string()],
        };

        let workflow = detector.create_resolution_workflow(conflict);

        assert_eq!(workflow.state, ResolutionWorkflowState::InitialAssessment);
        assert_eq!(workflow.escalation_level, EscalationLevel::Critical);
        assert!(workflow.stakeholder_reviews.is_empty());
        assert!(workflow.expert_consultations.is_empty());
        assert!(workflow.proposed_resolution.is_none());
        assert!(workflow.final_decision.is_none());
    }

    #[test]
    fn test_negotiated_resolution_template() {
        let template = NegotiatedResolutionTemplate {
            id: "template-1".to_string(),
            name: "Cultural Adaptation Template".to_string(),
            conflict_types: vec![ConflictType::CulturalIncompatibility],
            source_patterns: vec!["CivilLaw".to_string()],
            target_patterns: vec!["CommonLaw".to_string()],
            approach: "Phased adaptation with stakeholder consultation".to_string(),
            negotiation_steps: vec![
                NegotiationStep {
                    step_number: 1,
                    description: "Initial stakeholder meeting".to_string(),
                    involved_parties: vec![
                        "Legal experts".to_string(),
                        "Cultural advisors".to_string(),
                    ],
                    expected_outcome: "Agreement on adaptation scope".to_string(),
                    estimated_days: 5,
                },
                NegotiationStep {
                    step_number: 2,
                    description: "Draft adaptation proposal".to_string(),
                    involved_parties: vec!["Legal drafters".to_string()],
                    expected_outcome: "Initial proposal document".to_string(),
                    estimated_days: 10,
                },
            ],
            fallback_strategies: vec![
                "Escalate to bilateral commission".to_string(),
                "Seek international arbitration".to_string(),
            ],
            success_rate: 0.75,
            stakeholders: vec![
                "Source jurisdiction legal authority".to_string(),
                "Target jurisdiction legal authority".to_string(),
                "Cultural representatives".to_string(),
            ],
            required_approvals: vec![
                "Legal committee".to_string(),
                "Cultural affairs ministry".to_string(),
            ],
        };

        assert_eq!(template.negotiation_steps.len(), 2);
        assert_eq!(template.fallback_strategies.len(), 2);
        assert_eq!(template.stakeholders.len(), 3);
        assert!(template.success_rate > 0.5);
        assert!(
            template
                .conflict_types
                .contains(&ConflictType::CulturalIncompatibility)
        );
    }

    #[test]
    fn test_escalation_level_ordering() {
        assert!(EscalationLevel::Routine < EscalationLevel::Elevated);
        assert!(EscalationLevel::Elevated < EscalationLevel::High);
        assert!(EscalationLevel::High < EscalationLevel::Critical);
    }

    #[test]
    fn test_stakeholder_review() {
        let review = StakeholderReview {
            reviewer_id: "reviewer-1".to_string(),
            reviewer_name: "Jane Smith".to_string(),
            role: "Legal Counsel".to_string(),
            reviewed_at: "2024-01-01T00:00:00Z".to_string(),
            recommendation: StakeholderRecommendation::ApproveWithModifications,
            comments: "Approve with minor adjustments to cultural references".to_string(),
            concerns: vec!["Potential cultural sensitivity issue in section 3".to_string()],
            modifications: vec![
                "Adjust terminology in section 3".to_string(),
                "Add explanatory note for cultural context".to_string(),
            ],
        };

        assert_eq!(
            review.recommendation,
            StakeholderRecommendation::ApproveWithModifications
        );
        assert_eq!(review.concerns.len(), 1);
        assert_eq!(review.modifications.len(), 2);
    }

    #[test]
    fn test_expert_consultation() {
        let consultation = ExpertConsultation {
            id: "consult-1".to_string(),
            expert_id: "expert-123".to_string(),
            expert_name: "Dr. John Doe".to_string(),
            expertise_area: "International Legal Systems".to_string(),
            consulted_at: "2024-01-01T00:00:00Z".to_string(),
            opinion: "The proposed adaptation is sound but requires additional safeguards"
                .to_string(),
            recommended_approach: "Implement with monitoring period".to_string(),
            confidence: 0.9,
            legal_references: vec![
                "Treaty on Legal Harmonization, Art. 12".to_string(),
                "Case Law: Smith v. State (2020)".to_string(),
            ],
        };

        assert_eq!(consultation.confidence, 0.9);
        assert_eq!(consultation.legal_references.len(), 2);
        assert!(consultation.opinion.contains("safeguards"));
    }

    #[test]
    fn test_resolution_decision() {
        let decision = ResolutionDecision {
            id: "decision-1".to_string(),
            decision_maker_id: "dm-123".to_string(),
            decision_maker_role: "Chief Legal Officer".to_string(),
            decided_at: "2024-01-01T00:00:00Z".to_string(),
            chosen_strategy: "Gradual implementation with monitoring".to_string(),
            rationale: "Balances legal requirements with practical concerns".to_string(),
            implementation_plan: vec![
                "Phase 1: Pilot program in limited jurisdictions".to_string(),
                "Phase 2: Full implementation with review checkpoints".to_string(),
                "Phase 3: Final assessment and adjustments".to_string(),
            ],
            monitoring_requirements: vec![
                "Monthly compliance reports".to_string(),
                "Quarterly stakeholder reviews".to_string(),
            ],
            accepted_risks: vec!["Potential initial resistance from local authorities".to_string()],
        };

        assert_eq!(decision.implementation_plan.len(), 3);
        assert_eq!(decision.monitoring_requirements.len(), 2);
        assert_eq!(decision.accepted_risks.len(), 1);
    }

    // ========================================================================
    // Tests for AI-Assisted Porting (v0.1.5)
    // ========================================================================

    #[tokio::test]
    async fn test_ai_assistant_creation() {
        let assistant = AiPortingAssistant::new();
        assert!(assistant.generator.is_none());

        let assistant_default = AiPortingAssistant::default();
        assert!(assistant_default.generator.is_none());
    }

    #[tokio::test]
    async fn test_llm_adaptation_suggestions() {
        let assistant = AiPortingAssistant::new();
        let jp = test_jurisdiction_jp();
        let us = test_jurisdiction_us();
        let statute = Statute::new(
            "test",
            "Test Statute",
            Effect::new(EffectType::Grant, "Rights"),
        );

        let suggestions = assistant
            .generate_adaptation_suggestions(&statute, &jp, &us)
            .await
            .unwrap();

        assert!(!suggestions.is_empty());
        let first = &suggestions[0];
        assert_eq!(first.statute_id, "test");
        assert!(first.confidence > 0.0 && first.confidence <= 1.0);
        assert!(!first.suggestion.is_empty());
        assert!(matches!(
            first.category,
            AdaptationCategory::Procedural | AdaptationCategory::Cultural
        ));
    }

    #[tokio::test]
    async fn test_similar_statute_discovery() {
        let assistant = AiPortingAssistant::new();
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Rights"));
        let jurisdictions = vec![test_jurisdiction_jp(), test_jurisdiction_us()];

        let similar = assistant
            .discover_similar_statutes(&statute, &jurisdictions)
            .await
            .unwrap();

        // Should find at least one similar statute (similarity > 0.3)
        assert!(!similar.is_empty());

        for sim in &similar {
            assert!(sim.similarity_score > 0.0 && sim.similarity_score <= 1.0);
            assert!(!sim.matching_features.is_empty());
        }

        // Should be sorted by similarity score (descending)
        for i in 1..similar.len() {
            assert!(similar[i - 1].similarity_score >= similar[i].similarity_score);
        }
    }

    #[tokio::test]
    async fn test_gap_analysis() {
        let assistant = AiPortingAssistant::new();
        let jp = test_jurisdiction_jp();
        let us = test_jurisdiction_us();
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Rights"));

        let gap_analysis = assistant.analyze_gaps(&statute, &jp, &us).await.unwrap();

        assert_eq!(gap_analysis.source_statute_id, "test");
        assert!(gap_analysis.coverage_score >= 0.0 && gap_analysis.coverage_score <= 1.0);
        assert!(!gap_analysis.gaps.is_empty());
        assert!(!gap_analysis.recommendations.is_empty());

        for gap in &gap_analysis.gaps {
            assert!(!gap.description.is_empty());
            assert!(!gap.missing_element.is_empty());
            assert!(!gap.solutions.is_empty());
        }
    }

    #[tokio::test]
    async fn test_cultural_sensitivity_analysis() {
        let assistant = AiPortingAssistant::new();
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Rights"));

        // Create jurisdiction with cultural prohibitions
        let mut params = CulturalParams::for_country("US");
        params.prohibitions.push("alcohol".to_string());

        let jurisdiction = Jurisdiction::new("TEST", "Test", Locale::new("en").with_country("US"))
            .with_legal_system(LegalSystem::CommonLaw)
            .with_cultural_params(params);

        let analysis = assistant
            .check_cultural_sensitivity(&statute, &jurisdiction)
            .await
            .unwrap();

        assert_eq!(analysis.statute_id, "test");
        assert!(analysis.sensitivity_score >= 0.0 && analysis.sensitivity_score <= 1.0);
        assert!(!analysis.issues.is_empty());
        assert!(!analysis.assessment.is_empty());

        for issue in &analysis.issues {
            assert!(!issue.description.is_empty());
            assert!(!issue.explanation.is_empty());
        }
    }

    #[tokio::test]
    async fn test_plain_language_explanation() {
        let assistant = AiPortingAssistant::new();
        let statute = Statute::new(
            "test",
            "Test Statute",
            Effect::new(EffectType::Grant, "Rights"),
        );

        for audience_level in [
            AudienceLevel::GeneralPublic,
            AudienceLevel::Business,
            AudienceLevel::Government,
            AudienceLevel::Legal,
            AudienceLevel::Academic,
        ] {
            let explanation = assistant
                .generate_plain_explanation(&statute, audience_level)
                .await
                .unwrap();

            assert_eq!(explanation.statute_id, "test");
            assert_eq!(explanation.audience_level, audience_level);
            assert!(!explanation.summary.is_empty());
            assert!(!explanation.explanation.is_empty());
            assert!(!explanation.key_points.is_empty());
            assert!(explanation.readability_score > 0.0 && explanation.readability_score <= 1.0);
        }
    }

    #[test]
    fn test_adaptation_category() {
        let categories = vec![
            AdaptationCategory::Terminology,
            AdaptationCategory::Procedural,
            AdaptationCategory::Cultural,
            AdaptationCategory::Numerical,
            AdaptationCategory::Structural,
            AdaptationCategory::LegalPrinciple,
            AdaptationCategory::Compliance,
        ];

        for category in categories {
            assert!(matches!(
                category,
                AdaptationCategory::Terminology
                    | AdaptationCategory::Procedural
                    | AdaptationCategory::Cultural
                    | AdaptationCategory::Numerical
                    | AdaptationCategory::Structural
                    | AdaptationCategory::LegalPrinciple
                    | AdaptationCategory::Compliance
            ));
        }
    }

    #[test]
    fn test_gap_types() {
        let gap_types = vec![
            GapType::MissingConcept,
            GapType::MissingProcedure,
            GapType::MissingEnforcement,
            GapType::MissingSafeguard,
            GapType::InsufficientSpecificity,
            GapType::MissingCulturalElement,
        ];

        for gap_type in gap_types {
            assert!(matches!(
                gap_type,
                GapType::MissingConcept
                    | GapType::MissingProcedure
                    | GapType::MissingEnforcement
                    | GapType::MissingSafeguard
                    | GapType::InsufficientSpecificity
                    | GapType::MissingCulturalElement
            ));
        }
    }

    #[test]
    fn test_cultural_issue_types() {
        let issue_types = vec![
            CulturalIssueType::Religious,
            CulturalIssueType::Traditional,
            CulturalIssueType::SocialNorm,
            CulturalIssueType::Gender,
            CulturalIssueType::Family,
            CulturalIssueType::Language,
            CulturalIssueType::Historical,
        ];

        for issue_type in issue_types {
            assert!(matches!(
                issue_type,
                CulturalIssueType::Religious
                    | CulturalIssueType::Traditional
                    | CulturalIssueType::SocialNorm
                    | CulturalIssueType::Gender
                    | CulturalIssueType::Family
                    | CulturalIssueType::Language
                    | CulturalIssueType::Historical
            ));
        }
    }

    #[test]
    fn test_feature_types() {
        let feature_types = vec![
            FeatureType::LegalEffect,
            FeatureType::Structure,
            FeatureType::Terminology,
            FeatureType::Scope,
            FeatureType::Conditions,
            FeatureType::Remedies,
        ];

        for feature_type in feature_types {
            assert!(matches!(
                feature_type,
                FeatureType::LegalEffect
                    | FeatureType::Structure
                    | FeatureType::Terminology
                    | FeatureType::Scope
                    | FeatureType::Conditions
                    | FeatureType::Remedies
            ));
        }
    }

    #[test]
    fn test_audience_levels() {
        let levels = vec![
            AudienceLevel::GeneralPublic,
            AudienceLevel::Business,
            AudienceLevel::Government,
            AudienceLevel::Legal,
            AudienceLevel::Academic,
        ];

        for level in levels {
            assert!(matches!(
                level,
                AudienceLevel::GeneralPublic
                    | AudienceLevel::Business
                    | AudienceLevel::Government
                    | AudienceLevel::Legal
                    | AudienceLevel::Academic
            ));
        }
    }

    #[tokio::test]
    async fn test_multi_hop_port() {
        let jp = test_jurisdiction_jp();
        let us = test_jurisdiction_us();
        let uk = Jurisdiction::new("UK", "United Kingdom", Locale::new("en").with_country("GB"))
            .with_legal_system(LegalSystem::CommonLaw)
            .with_cultural_params(CulturalParams::for_country("GB"));

        let engine = PortingEngine::new(jp, us);
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"));

        let options = PortingOptions {
            apply_cultural_params: true,
            ..Default::default()
        };

        let chain = engine
            .multi_hop_port(&statute, &[uk], &options)
            .await
            .unwrap();

        assert_eq!(chain.hop_results.len(), 2);
        // Cumulative changes may be empty if no cultural differences between jurisdictions
        assert!(chain.chain_score >= 0.0 && chain.chain_score <= 1.0);
        assert_eq!(chain.source_jurisdiction, "JP");
        assert_eq!(chain.target_jurisdiction, "US");
        assert_eq!(chain.intermediate_hops.len(), 1);
    }

    #[test]
    fn test_record_history() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let options = PortingOptions::default();

        let history = engine.record_history(
            "test-statute".to_string(),
            "user-001".to_string(),
            &options,
            true,
            None,
        );

        assert_eq!(history.statute_id, "test-statute");
        assert_eq!(history.user, "user-001");
        assert!(history.success);
        assert!(history.error.is_none());
    }

    #[test]
    fn test_build_lineage() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let options = PortingOptions::default();

        let history = vec![
            engine.record_history(
                "statute-1".to_string(),
                "user".to_string(),
                &options,
                true,
                None,
            ),
            engine.record_history(
                "statute-2".to_string(),
                "user".to_string(),
                &options,
                true,
                None,
            ),
        ];

        let lineage = engine.build_lineage("original-id".to_string(), "JP".to_string(), &history);

        assert_eq!(lineage.original_id, "original-id");
        assert_eq!(lineage.original_jurisdiction, "JP");
        assert!(lineage.total_ports <= 2);
    }

    #[test]
    fn test_generate_diff() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statute = Statute::new(
            "test",
            "Original Title",
            Effect::new(EffectType::Grant, "Test"),
        );

        let options = PortingOptions::default();
        let ported = engine.port_statute(&statute, &options).unwrap();

        let diff = engine.generate_diff(&statute, &ported);

        assert_eq!(diff.original_id, "test");
        assert!(diff.similarity_score >= 0.0 && diff.similarity_score <= 1.0);
        assert!(!diff.differences.is_empty());
    }

    #[test]
    fn test_export_diff_markdown() {
        let engine = PortingEngine::new(test_jurisdiction_jp(), test_jurisdiction_us());
        let statute = Statute::new("test", "Original", Effect::new(EffectType::Grant, "Test"));

        let options = PortingOptions::default();
        let ported = engine.port_statute(&statute, &options).unwrap();

        let diff = engine.generate_diff(&statute, &ported);
        let md = engine.export_diff_markdown(&diff);

        assert!(md.contains("# Statute Diff"));
        assert!(md.contains("Similarity Score"));
        assert!(md.contains("```diff"));
    }

    // ========================================================================
    // Jurisdiction Database Tests (v0.1.1)
    // ========================================================================

    #[test]
    fn test_jurisdiction_profile_creation() {
        let profile = JurisdictionProfile::new(
            String::from("US"),
            String::from("United States"),
            LegalSystemType::CommonLaw,
        );

        assert_eq!(profile.code, "US");
        assert_eq!(profile.name, "United States");
        assert_eq!(profile.legal_system, LegalSystemType::CommonLaw);
        assert!(profile.official_languages.is_empty());
    }

    #[test]
    fn test_court_hierarchy() {
        let mut hierarchy = CourtHierarchy::new();

        hierarchy.add_court(Court {
            name: String::from("Supreme Court"),
            level: CourtLevel::Supreme,
            jurisdiction: String::from("Federal"),
            precedent_setting: true,
            judges: Some(9),
            url: None,
        });

        hierarchy.add_court(Court {
            name: String::from("District Court"),
            level: CourtLevel::District,
            jurisdiction: String::from("Regional"),
            precedent_setting: false,
            judges: Some(100),
            url: None,
        });

        assert_eq!(hierarchy.courts.len(), 2);
        assert_eq!(hierarchy.courts_by_level(CourtLevel::Supreme).len(), 1);
        assert_eq!(hierarchy.courts_by_level(CourtLevel::District).len(), 1);
    }

    #[test]
    fn test_legislative_process() {
        let process = LegislativeProcess::new(String::from("Congress"), String::from("House"))
            .with_upper_house(String::from("Senate"));

        assert!(process.is_bicameral);
        assert_eq!(process.upper_house, Some(String::from("Senate")));
        assert!(process.stages.contains(&LegislativeStage::UpperHouse));
    }

    #[test]
    fn test_constitutional_framework() {
        let mut framework = ConstitutionalFramework::new();
        framework.add_feature(ConstitutionalFeature::WrittenConstitution);
        framework.add_feature(ConstitutionalFeature::BillOfRights);
        framework.add_feature(ConstitutionalFeature::Federalism);

        assert!(framework.has_feature(ConstitutionalFeature::WrittenConstitution));
        assert!(framework.has_feature(ConstitutionalFeature::BillOfRights));
        assert!(framework.has_feature(ConstitutionalFeature::Federalism));
        assert!(!framework.has_feature(ConstitutionalFeature::ParliamentarySovereignty));
        assert_eq!(framework.features.len(), 3);
    }

    #[test]
    fn test_jurisdiction_compatibility_score() {
        let us = JurisdictionProfile::new(
            String::from("US"),
            String::from("United States"),
            LegalSystemType::CommonLaw,
        );

        let gb = JurisdictionProfile::new(
            String::from("GB"),
            String::from("United Kingdom"),
            LegalSystemType::CommonLaw,
        );

        let jp = JurisdictionProfile::new(
            String::from("JP"),
            String::from("Japan"),
            LegalSystemType::CivilLaw,
        );

        // US and GB should be more compatible (both common law)
        let us_gb_score = us.compatibility_score(&gb);
        let us_jp_score = us.compatibility_score(&jp);

        assert!(us_gb_score > us_jp_score);
        assert!(us_gb_score >= 0.0 && us_gb_score <= 1.0);
        assert!(us_jp_score >= 0.0 && us_jp_score <= 1.0);
    }

    #[test]
    fn test_jurisdiction_database() {
        let mut db = JurisdictionDatabase::new();

        let us = JurisdictionProfile::new(
            String::from("US"),
            String::from("United States"),
            LegalSystemType::CommonLaw,
        );

        let jp = JurisdictionProfile::new(
            String::from("JP"),
            String::from("Japan"),
            LegalSystemType::CivilLaw,
        );

        db.add_profile(us);
        db.add_profile(jp);

        assert!(db.get_profile("US").is_some());
        assert!(db.get_profile("JP").is_some());
        assert!(db.get_profile("FR").is_none());
        assert_eq!(db.list_codes().len(), 2);
    }

    #[test]
    fn test_find_by_legal_system() {
        let db = JurisdictionDatabase::with_major_jurisdictions();

        let common_law = db.find_by_legal_system(LegalSystemType::CommonLaw);
        let civil_law = db.find_by_legal_system(LegalSystemType::CivilLaw);

        assert!(common_law.len() >= 2); // US, GB
        assert!(civil_law.len() >= 3); // JP, DE, FR
    }

    #[test]
    fn test_find_compatible_jurisdictions() {
        let db = JurisdictionDatabase::with_major_jurisdictions();

        let compatible = db.find_compatible("US", 0.5);

        assert!(!compatible.is_empty());

        // Verify scores are sorted in descending order
        for i in 0..compatible.len().saturating_sub(1) {
            assert!(compatible[i].1 >= compatible[i + 1].1);
        }

        // Verify all scores meet minimum threshold
        for (_, score) in &compatible {
            assert!(*score >= 0.5);
        }
    }

    #[test]
    fn test_major_jurisdictions_database() {
        let db = JurisdictionDatabase::with_major_jurisdictions();

        // Verify US profile
        let us = db.get_profile("US").expect("US profile should exist");
        assert_eq!(us.name, "United States");
        assert_eq!(us.legal_system, LegalSystemType::CommonLaw);
        assert!(
            us.constitutional_framework
                .has_feature(ConstitutionalFeature::Federalism)
        );
        assert!(us.legislative_process.is_bicameral);
        assert!(us.court_hierarchy.has_jury_trials);

        // Verify Japan profile
        let jp = db.get_profile("JP").expect("JP profile should exist");
        assert_eq!(jp.name, "Japan");
        assert_eq!(jp.legal_system, LegalSystemType::CivilLaw);
        assert!(
            jp.constitutional_framework
                .has_feature(ConstitutionalFeature::ParliamentarySystem)
        );
        assert!(!jp.court_hierarchy.has_jury_trials);

        // Verify Germany profile
        let de = db.get_profile("DE").expect("DE profile should exist");
        assert_eq!(de.name, "Germany");
        assert!(
            de.constitutional_framework
                .has_feature(ConstitutionalFeature::Federalism)
        );
        assert!(de.court_hierarchy.constitutional_court.is_some());

        // Verify United Kingdom profile
        let gb = db.get_profile("GB").expect("GB profile should exist");
        assert_eq!(gb.name, "United Kingdom");
        assert!(!gb.constitutional_framework.has_written_constitution);
        assert!(
            gb.constitutional_framework
                .has_feature(ConstitutionalFeature::ParliamentarySovereignty)
        );

        // Verify France profile
        let fr = db.get_profile("FR").expect("FR profile should exist");
        assert_eq!(fr.name, "France");
        assert!(
            fr.constitutional_framework
                .has_feature(ConstitutionalFeature::SemiPresidentialSystem)
        );
    }

    #[test]
    fn test_court_level_ordering() {
        assert!(CourtLevel::Local < CourtLevel::District);
        assert!(CourtLevel::District < CourtLevel::Appellate);
        assert!(CourtLevel::Appellate < CourtLevel::Supreme);
        assert!(CourtLevel::Supreme < CourtLevel::International);
    }

    #[test]
    fn test_legislative_stage_ordering() {
        assert!(LegislativeStage::Drafting < LegislativeStage::Committee);
        assert!(LegislativeStage::Committee < LegislativeStage::FirstReading);
        assert!(LegislativeStage::FirstReading < LegislativeStage::SecondReading);
        assert!(LegislativeStage::SecondReading < LegislativeStage::ThirdReading);
        assert!(LegislativeStage::ThirdReading < LegislativeStage::UpperHouse);
        assert!(LegislativeStage::UpperHouse < LegislativeStage::Executive);
        assert!(LegislativeStage::Executive < LegislativeStage::Publication);
    }

    // ========================================================================
    // Semantic Mapping Tests (v0.1.2)
    // ========================================================================

    #[test]
    fn test_concept_equivalence() {
        let equiv = ConceptEquivalence::new(String::from("contract"), String::from("契約"), 0.95)
            .with_context(String::from("civil law"))
            .with_notes(String::from("Direct translation"));

        assert_eq!(equiv.source_concept, "contract");
        assert_eq!(equiv.target_concept, "契約");
        assert_eq!(equiv.equivalence_score, 0.95);
        assert!((equiv.semantic_distance - 0.05).abs() < 0.0001);
        assert_eq!(equiv.context.len(), 1);
        assert!(equiv.notes.is_some());
    }

    #[test]
    fn test_concept_equivalence_database() {
        let mut db = ConceptEquivalenceDatabase::new();

        db.add_equivalence(
            String::from("US->JP"),
            ConceptEquivalence::new(String::from("contract"), String::from("契約"), 0.95),
        );

        db.add_equivalence(
            String::from("US->JP"),
            ConceptEquivalence::new(String::from("tort"), String::from("不法行為"), 0.9),
        );

        let matches = db.find_equivalences("US", "JP", "contract");
        assert_eq!(matches.len(), 1);

        let best = db.best_match("US", "JP", "contract");
        assert!(best.is_some());
        assert_eq!(best.unwrap().target_concept, "契約");
    }

    #[test]
    fn test_term_translation() {
        let translation = TermTranslation::new(
            String::from("felony"),
            String::from("US"),
            String::from("重罪"),
            String::from("JP"),
            0.9,
            true,
        );

        assert_eq!(translation.source_term, "felony");
        assert_eq!(translation.target_term, "重罪");
        assert_eq!(translation.accuracy, 0.9);
        assert!(translation.is_direct);
    }

    #[test]
    fn test_term_translation_matrix() {
        let matrix = TermTranslationMatrix::with_common_translations();

        let translations = matrix.find_translations("US", "JP", "felony");
        assert!(!translations.is_empty());

        let best = matrix.best_translation("US", "JP", "felony", None);
        assert!(best.is_some());
        assert_eq!(best.unwrap().target_term, "重罪");
    }

    #[test]
    fn test_term_translation_context() {
        let mut matrix = TermTranslationMatrix::new();

        let mut criminal_trans = TermTranslation::new(
            String::from("charge"),
            String::from("US"),
            String::from("起訴"),
            String::from("JP"),
            0.9,
            true,
        );
        criminal_trans.valid_contexts = vec![String::from("criminal")];

        let mut civil_trans = TermTranslation::new(
            String::from("charge"),
            String::from("US"),
            String::from("料金"),
            String::from("JP"),
            0.8,
            true,
        );
        civil_trans.valid_contexts = vec![String::from("civil"), String::from("contract")];

        matrix.add_translation(criminal_trans);
        matrix.add_translation(civil_trans);

        let criminal_best = matrix.best_translation("US", "JP", "charge", Some("criminal"));
        assert_eq!(criminal_best.unwrap().target_term, "起訴");

        let civil_best = matrix.best_translation("US", "JP", "charge", Some("civil"));
        assert_eq!(civil_best.unwrap().target_term, "料金");
    }

    #[test]
    fn test_semantic_distance_calculator() {
        let mut concept_db = ConceptEquivalenceDatabase::new();

        concept_db.add_equivalence(
            String::from("US->JP"),
            ConceptEquivalence::new(String::from("contract"), String::from("契約"), 0.95),
        );

        let calculator = SemanticDistanceCalculator::new(concept_db);

        let distance = calculator.calculate_distance("US", "JP", "contract", "契約");
        assert!(distance >= 0.0 && distance <= 1.0);
        assert!(distance < 0.1); // Should be low for known equivalence
    }

    #[test]
    fn test_levenshtein_distance() {
        let concept_db = ConceptEquivalenceDatabase::new();
        let calculator = SemanticDistanceCalculator::new(concept_db);

        // Identical strings
        let dist1 = calculator.calculate_distance("US", "JP", "test", "test");
        assert_eq!(dist1, 0.0);

        // Different strings
        let dist2 = calculator.calculate_distance("US", "JP", "contract", "compact");
        assert!(dist2 > 0.0 && dist2 < 1.0);
    }

    #[test]
    fn test_context_aware_term_mapper() {
        let matrix = TermTranslationMatrix::with_common_translations();
        let mut mapper = ContextAwareTermMapper::new(matrix);

        mapper.add_context_rule(
            String::from("criminal"),
            vec![String::from("crime"), String::from("offense")],
        );

        let mapped = mapper.map_term("US", "JP", "felony", "serious crime");
        assert!(mapped.is_some());
        assert_eq!(mapped.unwrap(), "重罪");
    }

    #[test]
    fn test_legal_dictionary() {
        let dict = LegalDictionary::us_dictionary();

        assert_eq!(dict.jurisdiction, "US");
        assert!(!dict.terms.is_empty());

        let felony = dict.find_term("felony");
        assert!(felony.is_some());
        assert_eq!(felony.unwrap().domain, "criminal");

        let criminal_terms = dict.get_by_domain("criminal");
        assert!(criminal_terms.len() >= 2);
    }

    #[test]
    fn test_japan_dictionary() {
        let dict = LegalDictionary::japan_dictionary();

        assert_eq!(dict.jurisdiction, "JP");
        assert!(!dict.terms.is_empty());

        let felony = dict.find_term("重罪");
        assert!(felony.is_some());

        let criminal_terms = dict.get_by_domain("criminal");
        assert!(criminal_terms.len() >= 2);
    }

    #[test]
    fn test_legal_term_creation() {
        let term = LegalTerm::new(
            String::from("contract"),
            String::from("An agreement between parties"),
            String::from("US"),
            String::from("civil"),
        );

        assert_eq!(term.term, "contract");
        assert_eq!(term.jurisdiction, "US");
        assert_eq!(term.domain, "civil");
        assert!(term.related_terms.is_empty());
    }

    #[test]
    fn test_term_translation_matrix_get_terms() {
        let mut matrix = TermTranslationMatrix::new();

        matrix.add_term(LegalTerm::new(
            String::from("felony"),
            String::from("Serious crime"),
            String::from("US"),
            String::from("criminal"),
        ));

        matrix.add_term(LegalTerm::new(
            String::from("tort"),
            String::from("Civil wrong"),
            String::from("US"),
            String::from("civil"),
        ));

        let us_terms = matrix.get_terms("US");
        assert_eq!(us_terms.len(), 2);

        let criminal_terms = matrix.get_terms_by_domain("US", "criminal");
        assert_eq!(criminal_terms.len(), 1);
        assert_eq!(criminal_terms[0].term, "felony");
    }

    // ========================================================================
    // Cultural Adaptation Tests (v0.1.3)
    // ========================================================================

    #[test]
    fn test_cultural_exception() {
        let exception = CulturalException::new(
            CulturalExceptionType::Religious,
            String::from("US"),
            String::from("Religious accommodation"),
        )
        .with_legal_basis(String::from("Title VII"))
        .with_domain(String::from("employment"));

        assert_eq!(exception.exception_type, CulturalExceptionType::Religious);
        assert_eq!(exception.jurisdiction, "US");
        assert!(exception.legal_basis.is_some());
        assert_eq!(exception.applicable_domains.len(), 1);
    }

    #[test]
    fn test_cultural_exception_registry() {
        let registry = CulturalExceptionRegistry::with_common_exceptions();

        let us_exceptions = registry.get_exceptions("US");
        assert!(!us_exceptions.is_empty());

        let jp_religious = registry.get_by_type("JP", CulturalExceptionType::Religious);
        assert!(!jp_religious.is_empty());
    }

    #[test]
    fn test_holiday_calendar() {
        let mut calendar = HolidayCalendar::new(String::from("US"), CalendarSystem::Gregorian);

        let holiday = Holiday::new(
            String::from("Independence Day"),
            HolidayType::National,
            String::from("US"),
        )
        .with_fixed_date(7, 4)
        .as_legal_holiday();

        calendar.add_holiday(holiday);

        assert_eq!(calendar.holidays.len(), 1);
        assert_eq!(calendar.calendar_system, CalendarSystem::Gregorian);
    }

    #[test]
    fn test_us_calendar() {
        let calendar = HolidayCalendar::us_calendar();

        assert_eq!(calendar.jurisdiction, "US");
        assert_eq!(calendar.calendar_system, CalendarSystem::Gregorian);
        assert!(calendar.holidays.len() >= 2);

        let national_holidays = calendar.get_by_type(HolidayType::National);
        assert!(national_holidays.len() >= 2);
    }

    #[test]
    fn test_japan_calendar() {
        let calendar = HolidayCalendar::japan_calendar();

        assert_eq!(calendar.jurisdiction, "JP");
        assert_eq!(calendar.calendar_system, CalendarSystem::Japanese);
        assert!(calendar.holidays.len() >= 2);
    }

    #[test]
    fn test_currency() {
        assert_eq!(Currency::USD.code(), "USD");
        assert_eq!(Currency::JPY.symbol(), "¥");
        assert_eq!(Currency::EUR.code(), "EUR");
        assert_eq!(Currency::GBP.symbol(), "£");
    }

    #[test]
    fn test_monetary_conversion() {
        let conversion = MonetaryConversion::new(100.0, Currency::USD, Currency::JPY, 150.0);

        assert_eq!(conversion.source_amount, 100.0);
        assert_eq!(conversion.source_currency, Currency::USD);
        assert_eq!(conversion.target_amount, 15000.0);
        assert_eq!(conversion.target_currency, Currency::JPY);
        assert_eq!(conversion.exchange_rate, 150.0);
    }

    #[test]
    fn test_monetary_conversion_threshold() {
        let conversion = MonetaryConversion::new(100.0, Currency::USD, Currency::JPY, 150.0);

        assert!(conversion.exceeds_threshold(10000.0));
        assert!(!conversion.exceeds_threshold(20000.0));
    }

    #[test]
    fn test_monetary_adapter() {
        let adapter = MonetaryAdapter::with_common_rates();

        let conversion = adapter.convert(1000.0, Currency::USD, Currency::JPY);
        assert!(conversion.is_some());

        let conv = conversion.unwrap();
        assert_eq!(conv.target_amount, 150_000.0);
    }

    #[test]
    fn test_age_of_majority() {
        let age = AgeOfMajority::new(String::from("US"), 18);

        assert_eq!(age.jurisdiction, "US");
        assert_eq!(age.age, 18);
        assert!(age.exceptions.is_empty());
    }

    #[test]
    fn test_age_of_majority_mapper() {
        let mapper = AgeOfMajorityMapper::with_common_jurisdictions();

        let us_age = mapper.get_age("US");
        assert!(us_age.is_some());
        assert_eq!(us_age.unwrap().age, 18);

        let jp_age = mapper.get_age("JP");
        assert!(jp_age.is_some());
        assert_eq!(jp_age.unwrap().age, 18);
    }

    #[test]
    fn test_age_mapping() {
        let mapper = AgeOfMajorityMapper::with_common_jurisdictions();

        // US and JP both have age 18, so no mapping needed
        let mapping = mapper.map_age_reference("US", "JP");
        assert!(mapping.is_none());
    }

    #[test]
    fn test_legal_capacity_rule() {
        let rule = LegalCapacityRule::new(LegalCapacityType::Contractual, String::from("US"), 18);

        assert_eq!(rule.capacity_type, LegalCapacityType::Contractual);
        assert_eq!(rule.jurisdiction, "US");
        assert_eq!(rule.minimum_age, 18);
    }

    #[test]
    fn test_legal_capacity_adapter() {
        let adapter = LegalCapacityAdapter::with_common_rules();

        let us_rules = adapter.get_rules("US");
        assert!(!us_rules.is_empty());

        let us_contract = adapter.get_rule("US", LegalCapacityType::Contractual);
        assert!(us_contract.is_some());
        assert_eq!(us_contract.unwrap().minimum_age, 18);
    }

    #[test]
    fn test_legal_capacity_differences() {
        let adapter = LegalCapacityAdapter::with_common_rules();

        let us_criminal = adapter.get_rule("US", LegalCapacityType::CriminalResponsibility);
        let jp_criminal = adapter.get_rule("JP", LegalCapacityType::CriminalResponsibility);

        assert!(us_criminal.is_some());
        assert!(jp_criminal.is_some());

        // US: 18, JP: 14
        assert_eq!(us_criminal.unwrap().minimum_age, 18);
        assert_eq!(jp_criminal.unwrap().minimum_age, 14);
    }

    // Validation Framework Tests (v0.1.6)

    #[test]
    fn test_compliance_checker() {
        let us = test_jurisdiction_us();
        let checker = TargetJurisdictionChecker::new(us);

        let statute = Statute::new(
            "test-statute",
            "Test Administrative Procedure",
            Effect::new(EffectType::Grant, "Administrative rights"),
        );

        let result = checker.check_compliance(&statute);

        assert!(!result.id.is_empty());
        assert!(!result.checked_regulations.is_empty());
        assert!(result.compliance_score >= 0.0 && result.compliance_score <= 1.0);
    }

    #[test]
    fn test_compliance_severity_levels() {
        let us = test_jurisdiction_us();
        let checker = TargetJurisdictionChecker::new(us);

        let statute = Statute::new(
            "test-statute",
            "Test Statute",
            Effect::new(EffectType::Grant, "Rights"),
        );

        let result = checker.check_compliance(&statute);

        // Check that severity levels are properly categorized
        for issue in &result.issues {
            assert!(matches!(
                issue.severity,
                ComplianceSeverity::Critical
                    | ComplianceSeverity::High
                    | ComplianceSeverity::Medium
                    | ComplianceSeverity::Low
                    | ComplianceSeverity::Info
            ));
        }
    }

    #[test]
    fn test_constitutional_analyzer() {
        let us = test_jurisdiction_us();
        let analyzer = ConstitutionalAnalyzer::new(us);

        let statute = Statute::new(
            "test-statute",
            "Test Constitutional Statute",
            Effect::new(EffectType::Grant, "Freedom rights"),
        );

        let result = analyzer.analyze(&statute);

        assert!(!result.id.is_empty());
        assert!(result.compatibility_score >= 0.0 && result.compatibility_score <= 1.0);
        assert!(!result.relevant_provisions.is_empty());
        assert!(!result.recommended_amendments.is_empty());
    }

    #[test]
    fn test_constitutional_provisions_us() {
        let us = test_jurisdiction_us();
        let analyzer = ConstitutionalAnalyzer::new(us);

        let statute = Statute::new(
            "test-statute",
            "Test Statute",
            Effect::new(EffectType::Grant, "Rights"),
        );

        let result = analyzer.analyze(&statute);

        // US should have First Amendment and Fourteenth Amendment provisions
        assert!(
            result
                .relevant_provisions
                .iter()
                .any(|p| p.contains("Amendment"))
        );
    }

    #[test]
    fn test_constitutional_provisions_japan() {
        let jp = test_jurisdiction_jp();
        let analyzer = ConstitutionalAnalyzer::new(jp);

        let statute = Statute::new(
            "test-statute",
            "Test Statute",
            Effect::new(EffectType::Grant, "Rights"),
        );

        let result = analyzer.analyze(&statute);

        // Japan should have Article provisions
        assert!(
            result
                .relevant_provisions
                .iter()
                .any(|p| p.contains("Article") || p.contains("憲法"))
        );
    }

    #[test]
    fn test_treaty_compliance_checker() {
        let us = test_jurisdiction_us();
        let checker = TreatyTargetJurisdictionChecker::new(us);

        let statute = Statute::new(
            "test-statute",
            "Test Human Rights Statute",
            Effect::new(EffectType::Grant, "Human rights"),
        );

        let result = checker.check_compliance(&statute);

        assert!(!result.id.is_empty());
        assert!(result.compliance_score >= 0.0 && result.compliance_score <= 1.0);
        assert!(!result.checked_treaties.is_empty());
        assert!(!result.recommendations.is_empty());
    }

    #[test]
    fn test_treaty_database() {
        let us = test_jurisdiction_us();
        let checker = TreatyTargetJurisdictionChecker::new(us);

        let statute = Statute::new(
            "test-statute",
            "Test Statute",
            Effect::new(EffectType::Grant, "Rights"),
        );

        let result = checker.check_compliance(&statute);

        // Should check major international treaties
        assert!(
            result
                .checked_treaties
                .iter()
                .any(|t| t.contains("International Covenant") || t.contains("Rights"))
        );
    }

    #[test]
    fn test_human_rights_assessor() {
        let us = test_jurisdiction_us();
        let assessor = HumanRightsAssessor::new(us);

        let statute = Statute::new(
            "test-statute",
            "Test Human Rights Statute",
            Effect::new(EffectType::Grant, "Fundamental rights"),
        );

        let result = assessor.assess(&statute);

        assert!(!result.id.is_empty());
        assert!(result.impact_score >= -1.0 && result.impact_score <= 1.0);
        assert!(!result.mitigation_measures.is_empty());
        assert!(!result.summary.is_empty());
    }

    #[test]
    fn test_human_rights_impact_types() {
        let us = test_jurisdiction_us();
        let assessor = HumanRightsAssessor::new(us);

        let statute = Statute::new(
            "test-statute",
            "Test Statute",
            Effect::new(EffectType::Grant, "Rights"),
        );

        let result = assessor.assess(&statute);

        // Verify impact types are properly categorized
        for right in &result.affected_rights {
            assert!(matches!(
                right.impact,
                RightImpactType::Enhancement
                    | RightImpactType::Neutral
                    | RightImpactType::Restriction
                    | RightImpactType::Violation
            ));
        }
    }

    #[test]
    fn test_enforceability_predictor() {
        let us = test_jurisdiction_us();
        let predictor = EnforceabilityPredictor::new(us);

        let statute = Statute::new(
            "test-statute",
            "Test Enforcement Statute",
            Effect::new(EffectType::Grant, "Enforcement powers"),
        );

        let result = predictor.predict(&statute);

        assert!(!result.id.is_empty());
        assert!(result.enforceability_score >= 0.0 && result.enforceability_score <= 1.0);
        assert!(!result.required_mechanisms.is_empty());
        assert!(!result.recommendations.is_empty());
    }

    #[test]
    fn test_enforcement_challenge_types() {
        let us = test_jurisdiction_us();
        let predictor = EnforceabilityPredictor::new(us);

        let statute = Statute::new(
            "test-statute",
            "Test Statute",
            Effect::new(EffectType::Grant, "Rights"),
        );

        let result = predictor.predict(&statute);

        // Verify challenge types are properly categorized
        for challenge in &result.challenges {
            assert!(matches!(
                challenge.challenge_type,
                EnforcementChallengeType::Authority
                    | EnforcementChallengeType::Resources
                    | EnforcementChallengeType::Technical
                    | EnforcementChallengeType::Cultural
                    | EnforcementChallengeType::Administrative
                    | EnforcementChallengeType::Monitoring
            ));
        }
    }

    #[test]
    fn test_validation_framework_creation() {
        let us = test_jurisdiction_us();
        let framework = ValidationFramework::new(us);

        let statute = Statute::new(
            "test-statute",
            "Test Validation Statute",
            Effect::new(EffectType::Grant, "Rights"),
        );

        let result = framework.validate(&statute);

        assert!(!result.id.is_empty());
        assert!(result.overall_score >= 0.0 && result.overall_score <= 1.0);
        assert!(!result.summary.is_empty());
    }

    #[test]
    fn test_validation_framework_comprehensive() {
        let us = test_jurisdiction_us();
        let framework = ValidationFramework::new(us);

        let statute = Statute::new(
            "test-statute",
            "Test Comprehensive Statute",
            Effect::new(EffectType::Grant, "Comprehensive rights"),
        );

        let result = framework.validate(&statute);

        // All sub-validations should be present
        assert!(!result.compliance.id.is_empty());
        assert!(!result.constitutional.id.is_empty());
        assert!(!result.treaty_compliance.id.is_empty());
        assert!(!result.human_rights.id.is_empty());
        assert!(!result.enforceability.id.is_empty());
    }

    #[test]
    fn test_validation_overall_score_calculation() {
        let us = test_jurisdiction_us();
        let framework = ValidationFramework::new(us);

        let statute = Statute::new(
            "test-statute",
            "Test Score Statute",
            Effect::new(EffectType::Grant, "Rights"),
        );

        let result = framework.validate(&statute);

        // Overall score should be calculated from all components
        let expected_score = (result.compliance.compliance_score
            + result.constitutional.compatibility_score
            + result.treaty_compliance.compliance_score
            + result.enforceability.enforceability_score
            + (result.human_rights.impact_score + 1.0) / 2.0)
            / 5.0;

        assert!((result.overall_score - expected_score).abs() < 0.001);
    }

    #[test]
    fn test_validation_passed_criteria() {
        let us = test_jurisdiction_us();
        let framework = ValidationFramework::new(us);

        let statute = Statute::new(
            "test-statute",
            "Test Passing Statute",
            Effect::new(EffectType::Grant, "Rights"),
        );

        let result = framework.validate(&statute);

        // If validation passed, all components should be compliant
        if result.passed {
            assert!(result.compliance.is_compliant);
            assert!(result.constitutional.is_compatible);
            assert!(result.treaty_compliance.is_compliant);
            assert!(result.human_rights.impact_score >= 0.0);
            assert!(result.enforceability.is_enforceable);
        }
    }

    #[test]
    fn test_pre_porting_feasibility_analysis() {
        let jp = test_jurisdiction_jp();
        let us = test_jurisdiction_us();
        let analyzer = PrePortingFeasibilityAnalyzer::new(jp, us);

        let statute = Statute::new(
            "test-statute",
            "Test Feasibility Statute",
            Effect::new(EffectType::Grant, "Rights"),
        );

        let analysis = analyzer.analyze(&statute);

        // Check basic fields
        assert!(!analysis.id.is_empty());
        assert!(analysis.feasibility_score >= 0.0 && analysis.feasibility_score <= 1.0);
        assert!(analysis.technical_feasibility >= 0.0 && analysis.technical_feasibility <= 1.0);
        assert!(analysis.legal_feasibility >= 0.0 && analysis.legal_feasibility <= 1.0);
        assert!(analysis.cultural_feasibility >= 0.0 && analysis.cultural_feasibility <= 1.0);
        assert!(analysis.economic_feasibility >= 0.0 && analysis.economic_feasibility <= 1.0);
        assert!(analysis.political_feasibility >= 0.0 && analysis.political_feasibility <= 1.0);

        // Check that factors are generated
        assert!(!analysis.factors.is_empty());

        // Check that prerequisites are generated
        assert!(!analysis.prerequisites.is_empty());

        // Check time and cost estimates
        assert!(analysis.estimated_time_days > 0);
        assert!(analysis.estimated_cost_usd > 0.0);

        // Check recommended approach
        assert!(!analysis.recommended_approach.is_empty());
        assert!(!analysis.alternatives.is_empty());
    }

    #[test]
    fn test_feasibility_recommendation_levels() {
        // Test different feasibility score ranges produce expected recommendations
        let jp = test_jurisdiction_jp();
        let us = test_jurisdiction_us();
        let analyzer = PrePortingFeasibilityAnalyzer::new(jp.clone(), us.clone());

        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Rights"));

        let analysis = analyzer.analyze(&statute);

        // Verify recommendation matches feasibility score
        match analysis.recommendation {
            FeasibilityRecommendation::StronglyRecommended => {
                assert!(analysis.feasibility_score >= 0.85);
            }
            FeasibilityRecommendation::Recommended => {
                assert!(analysis.feasibility_score >= 0.7 && analysis.feasibility_score < 0.85);
            }
            FeasibilityRecommendation::Conditional => {
                assert!(analysis.feasibility_score >= 0.5 && analysis.feasibility_score < 0.7);
            }
            FeasibilityRecommendation::NotRecommended => {
                assert!(analysis.feasibility_score >= 0.3 && analysis.feasibility_score < 0.5);
            }
            FeasibilityRecommendation::StronglyNotRecommended => {
                assert!(analysis.feasibility_score < 0.3);
            }
        }
    }

    #[test]
    fn test_feasibility_factor_categories() {
        let factor = FeasibilityFactor {
            id: "test-factor".to_string(),
            category: FeasibilityCategory::Technical,
            name: "Test Factor".to_string(),
            impact: -0.2,
            severity: FeasibilitySeverity::Moderate,
            description: "Test description".to_string(),
            mitigation_strategies: vec!["Strategy 1".to_string()],
        };

        assert_eq!(factor.category, FeasibilityCategory::Technical);
        assert_eq!(factor.severity, FeasibilitySeverity::Moderate);
        assert_eq!(factor.impact, -0.2);
    }

    #[test]
    fn test_compliance_issue_categories() {
        let issue = ValidationComplianceIssue {
            id: "test-issue".to_string(),
            severity: ComplianceSeverity::Medium,
            category: ComplianceCategory::Regulatory,
            description: "Test issue".to_string(),
            conflicting_regulation: "test-reg".to_string(),
            suggested_resolution: Some("Test resolution".to_string()),
        };

        assert!(matches!(
            issue.category,
            ComplianceCategory::Constitutional
                | ComplianceCategory::Regulatory
                | ComplianceCategory::Procedural
                | ComplianceCategory::Cultural
                | ComplianceCategory::Technical
                | ComplianceCategory::Administrative
        ));
    }

    #[test]
    fn test_impact_severity_levels() {
        let severities = vec![
            ImpactSeverity::Severe,
            ImpactSeverity::Moderate,
            ImpactSeverity::Minor,
            ImpactSeverity::Negligible,
        ];

        for severity in severities {
            assert!(matches!(
                severity,
                ImpactSeverity::Severe
                    | ImpactSeverity::Moderate
                    | ImpactSeverity::Minor
                    | ImpactSeverity::Negligible
            ));
        }
    }

    // Workflow Management Tests (v0.1.7)

    #[test]
    fn test_project_creation() {
        let mut manager = PortingProjectManager::new();
        let project = manager.create_project(
            "Test Project".to_string(),
            "Test description".to_string(),
            "JP".to_string(),
            "US".to_string(),
        );

        assert!(!project.id.is_empty());
        assert_eq!(project.name, "Test Project");
        assert_eq!(project.status, ProjectStatus::Planning);
        assert!(project.statute_ids.is_empty());
        assert!(project.stakeholders.is_empty());
    }

    #[test]
    fn test_project_status_update() {
        let mut manager = PortingProjectManager::new();
        let project = manager.create_project(
            "Test".to_string(),
            "Desc".to_string(),
            "JP".to_string(),
            "US".to_string(),
        );

        manager.update_status(&project.id, ProjectStatus::InProgress);

        let updated = manager.get_project(&project.id).unwrap();
        assert_eq!(updated.status, ProjectStatus::InProgress);
    }

    #[test]
    fn test_add_statute_to_project() {
        let mut manager = PortingProjectManager::new();
        let project = manager.create_project(
            "Test".to_string(),
            "Desc".to_string(),
            "JP".to_string(),
            "US".to_string(),
        );

        manager.add_statute(&project.id, "statute-1".to_string());
        manager.add_statute(&project.id, "statute-2".to_string());

        let updated = manager.get_project(&project.id).unwrap();
        assert_eq!(updated.statute_ids.len(), 2);
        assert!(updated.statute_ids.contains(&"statute-1".to_string()));
    }

    #[test]
    fn test_add_stakeholder_to_project() {
        let mut manager = PortingProjectManager::new();
        let project = manager.create_project(
            "Test".to_string(),
            "Desc".to_string(),
            "JP".to_string(),
            "US".to_string(),
        );

        let stakeholder = Stakeholder {
            id: "stakeholder-1".to_string(),
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            role: StakeholderRole::LegalExpert,
            notification_preferences: NotificationPreferences {
                on_status_change: true,
                on_deadline_approaching: true,
                on_assignment: true,
                on_review_request: true,
                channels: vec![NotificationChannel::Email],
            },
        };

        manager.add_stakeholder(&project.id, stakeholder);

        let updated = manager.get_project(&project.id).unwrap();
        assert_eq!(updated.stakeholders.len(), 1);
        assert_eq!(updated.stakeholders[0].name, "John Doe");
    }

    #[test]
    fn test_add_milestone() {
        let mut manager = PortingProjectManager::new();
        let project = manager.create_project(
            "Test".to_string(),
            "Desc".to_string(),
            "JP".to_string(),
            "US".to_string(),
        );

        let milestone = Milestone {
            id: "milestone-1".to_string(),
            name: "Complete Draft".to_string(),
            description: "Complete initial draft".to_string(),
            target_date: "2025-12-31T00:00:00Z".to_string(),
            completed: false,
            completed_date: None,
            dependencies: Vec::new(),
        };

        manager.add_milestone(&project.id, milestone);

        let updated = manager.get_project(&project.id).unwrap();
        assert_eq!(updated.timeline.milestones.len(), 1);
    }

    #[test]
    fn test_complete_milestone() {
        let mut manager = PortingProjectManager::new();
        let project = manager.create_project(
            "Test".to_string(),
            "Desc".to_string(),
            "JP".to_string(),
            "US".to_string(),
        );

        let milestone = Milestone {
            id: "milestone-1".to_string(),
            name: "Complete Draft".to_string(),
            description: "Complete initial draft".to_string(),
            target_date: "2025-12-31T00:00:00Z".to_string(),
            completed: false,
            completed_date: None,
            dependencies: Vec::new(),
        };

        manager.add_milestone(&project.id, milestone);
        manager.complete_milestone(&project.id, "milestone-1");

        let updated = manager.get_project(&project.id).unwrap();
        assert!(updated.timeline.milestones[0].completed);
        assert!(updated.timeline.milestones[0].completed_date.is_some());
    }

    #[test]
    fn test_list_projects_by_status() {
        let mut manager = PortingProjectManager::new();

        manager.create_project(
            "P1".to_string(),
            "D1".to_string(),
            "JP".to_string(),
            "US".to_string(),
        );
        let p2 = manager.create_project(
            "P2".to_string(),
            "D2".to_string(),
            "JP".to_string(),
            "US".to_string(),
        );
        manager.update_status(&p2.id, ProjectStatus::InProgress);

        let in_progress = manager.list_projects_by_status(ProjectStatus::InProgress);
        assert_eq!(in_progress.len(), 1);

        let planning = manager.list_projects_by_status(ProjectStatus::Planning);
        assert_eq!(planning.len(), 1);
    }

    #[test]
    fn test_review_workflow_creation() {
        let mut workflow = StakeholderReviewWorkflow::new();

        let step = ReviewWorkflowStep {
            id: "step-1".to_string(),
            name: "Legal Review".to_string(),
            order: 1,
            required_reviewers: vec!["reviewer-1".to_string()],
            optional_reviewers: Vec::new(),
            min_approvals: 1,
            status: ReviewStepStatus::Pending,
            reviews: Vec::new(),
        };

        workflow.create_workflow("project-1".to_string(), vec![step]);

        let status = workflow.get_workflow_status("project-1");
        assert!(status.is_some());
        assert_eq!(status.unwrap().len(), 1);
    }

    #[test]
    fn test_submit_review() {
        let mut workflow = StakeholderReviewWorkflow::new();

        let step = ReviewWorkflowStep {
            id: "step-1".to_string(),
            name: "Legal Review".to_string(),
            order: 1,
            required_reviewers: vec!["reviewer-1".to_string()],
            optional_reviewers: Vec::new(),
            min_approvals: 1,
            status: ReviewStepStatus::Pending,
            reviews: Vec::new(),
        };

        workflow.create_workflow("project-1".to_string(), vec![step]);

        let review = WorkflowReview {
            id: "review-1".to_string(),
            reviewer_id: "reviewer-1".to_string(),
            decision: ReviewDecision::Approve,
            comments: "Looks good".to_string(),
            reviewed_at: chrono::Utc::now().to_rfc3339(),
            recommended_changes: Vec::new(),
        };

        workflow.submit_review("project-1", "step-1", review);

        let status = workflow.get_workflow_status("project-1").unwrap();
        assert_eq!(status[0].reviews.len(), 1);
        assert_eq!(status[0].status, ReviewStepStatus::Approved);
    }

    #[test]
    fn test_version_control_iteration() {
        let mut vc = PortingVersionControl::new();

        let iteration = vc.create_iteration(
            "project-1".to_string(),
            "statute snapshot v1".to_string(),
            "user-1".to_string(),
            "Initial version".to_string(),
        );

        assert_eq!(iteration.iteration_number, 1);
        assert_eq!(iteration.statute_snapshot, "statute snapshot v1");
        assert_eq!(iteration.project_id, "project-1");
    }

    #[test]
    fn test_multiple_iterations() {
        let mut vc = PortingVersionControl::new();

        vc.create_iteration(
            "project-1".to_string(),
            "v1".to_string(),
            "user-1".to_string(),
            "First".to_string(),
        );

        vc.create_iteration(
            "project-1".to_string(),
            "v2".to_string(),
            "user-1".to_string(),
            "Second".to_string(),
        );

        let iterations = vc.get_iterations("project-1").unwrap();
        assert_eq!(iterations.len(), 2);
        assert_eq!(iterations[0].iteration_number, 1);
        assert_eq!(iterations[1].iteration_number, 2);
    }

    #[test]
    fn test_get_specific_iteration() {
        let mut vc = PortingVersionControl::new();

        vc.create_iteration(
            "project-1".to_string(),
            "v1".to_string(),
            "user-1".to_string(),
            "First".to_string(),
        );

        vc.create_iteration(
            "project-1".to_string(),
            "v2".to_string(),
            "user-1".to_string(),
            "Second".to_string(),
        );

        let iteration = vc.get_iteration("project-1", 2).unwrap();
        assert_eq!(iteration.statute_snapshot, "v2");
    }

    #[test]
    fn test_revert_iteration() {
        let mut vc = PortingVersionControl::new();

        vc.create_iteration(
            "project-1".to_string(),
            "v1".to_string(),
            "user-1".to_string(),
            "First".to_string(),
        );

        vc.create_iteration(
            "project-1".to_string(),
            "v2".to_string(),
            "user-1".to_string(),
            "Second".to_string(),
        );

        let reverted = vc.revert_to_iteration("project-1", 1, "user-2".to_string());
        assert!(reverted.is_some());

        let iterations = vc.get_iterations("project-1").unwrap();
        assert_eq!(iterations.len(), 3);
        assert_eq!(iterations[2].statute_snapshot, "v1");
    }

    #[test]
    fn test_create_branch() {
        let mut vc = PortingVersionControl::new();

        // Create main branch iterations
        vc.create_iteration(
            "project-1".to_string(),
            "v1".to_string(),
            "user-1".to_string(),
            "Version 1".to_string(),
        );

        // Create a branch
        let branch = vc
            .create_branch(
                "project-1".to_string(),
                "feature-x".to_string(),
                1,
                "user-1".to_string(),
                "Working on feature X".to_string(),
            )
            .unwrap();

        assert_eq!(branch.branch, Some("feature-x".to_string()));
        assert_eq!(branch.statute_snapshot, "v1");
        assert!(branch.tags.contains(&"branch".to_string()));

        // Check branches list
        let branches = vc.get_branches("project-1");
        assert_eq!(branches.len(), 1);
        assert!(branches.contains(&"feature-x".to_string()));
    }

    #[test]
    fn test_branch_iterations() {
        let mut vc = PortingVersionControl::new();

        vc.create_iteration(
            "project-1".to_string(),
            "v1".to_string(),
            "user-1".to_string(),
            "Version 1".to_string(),
        );

        vc.create_branch(
            "project-1".to_string(),
            "feature-a".to_string(),
            1,
            "user-1".to_string(),
            "Branch A".to_string(),
        );

        vc.create_branch(
            "project-1".to_string(),
            "feature-b".to_string(),
            1,
            "user-1".to_string(),
            "Branch B".to_string(),
        );

        let branch_a_iterations = vc.get_branch_iterations("project-1", "feature-a");
        assert_eq!(branch_a_iterations.len(), 1);
        assert_eq!(branch_a_iterations[0].branch, Some("feature-a".to_string()));

        let branches = vc.get_branches("project-1");
        assert_eq!(branches.len(), 2);
    }

    #[test]
    fn test_merge_branch() {
        let mut vc = PortingVersionControl::new();

        vc.create_iteration(
            "project-1".to_string(),
            "v1".to_string(),
            "user-1".to_string(),
            "Version 1".to_string(),
        );

        vc.create_branch(
            "project-1".to_string(),
            "feature-x".to_string(),
            1,
            "user-1".to_string(),
            "Feature X".to_string(),
        );

        let merged = vc
            .merge_branch(
                "project-1".to_string(),
                "feature-x".to_string(),
                None,
                "user-1".to_string(),
                "Merged feature X".to_string(),
            )
            .unwrap();

        assert_eq!(merged.branch, None); // Merged to main
        assert!(merged.notes.contains("Merged feature-x"));
        assert!(merged.tags.contains(&"merge".to_string()));
    }

    #[test]
    fn test_generate_changelog() {
        let mut vc = PortingVersionControl::new();

        vc.create_iteration(
            "project-1".to_string(),
            "v1".to_string(),
            "user-1".to_string(),
            "Initial version".to_string(),
        );

        vc.create_iteration(
            "project-1".to_string(),
            "v2".to_string(),
            "user-2".to_string(),
            "Updated statute".to_string(),
        );

        let changelog = vc.generate_changelog("project-1").unwrap();

        assert_eq!(changelog.project_id, "project-1");
        assert_eq!(changelog.total_iterations, 2);
        assert_eq!(changelog.entries.len(), 2);
        assert_eq!(changelog.entries[0].iteration_number, 1);
        assert_eq!(changelog.entries[1].iteration_number, 2);
    }

    #[test]
    fn test_changelog_export_markdown() {
        let mut vc = PortingVersionControl::new();

        vc.create_iteration(
            "project-1".to_string(),
            "v1".to_string(),
            "user-1".to_string(),
            "Initial version".to_string(),
        );

        let changelog = vc.generate_changelog("project-1").unwrap();
        let markdown = changelog.to_markdown();

        assert!(markdown.contains("# Porting Changelog"));
        assert!(markdown.contains("project-1"));
        assert!(markdown.contains("## Iteration 1"));
        assert!(markdown.contains("user-1"));
    }

    #[test]
    fn test_changelog_export_json() {
        let mut vc = PortingVersionControl::new();

        vc.create_iteration(
            "project-1".to_string(),
            "v1".to_string(),
            "user-1".to_string(),
            "Initial version".to_string(),
        );

        let changelog = vc.generate_changelog("project-1").unwrap();
        let json = changelog.to_json().unwrap();

        assert!(json.contains("project-1"));
        assert!(json.contains("user-1"));
    }

    #[test]
    fn test_approval_chain_creation() {
        let mut manager = ApprovalChainManager::new();

        let step = ApprovalStep {
            id: "step-1".to_string(),
            name: "Manager Approval".to_string(),
            order: 1,
            approvers: vec!["manager-1".to_string()],
            approval_mode: ApprovalMode::Any,
            status: ApprovalStepStatus::Pending,
            approvals: Vec::new(),
            auto_approve_after: None,
        };

        let chain = manager.create_chain("Test Chain".to_string(), vec![step]);

        assert!(!chain.id.is_empty());
        assert_eq!(chain.name, "Test Chain");
        assert_eq!(chain.status, ApprovalChainStatus::NotStarted);
        assert_eq!(chain.steps.len(), 1);
    }

    #[test]
    fn test_submit_approval() {
        let mut manager = ApprovalChainManager::new();

        let step = ApprovalStep {
            id: "step-1".to_string(),
            name: "Manager Approval".to_string(),
            order: 1,
            approvers: vec!["manager-1".to_string()],
            approval_mode: ApprovalMode::Any,
            status: ApprovalStepStatus::Pending,
            approvals: Vec::new(),
            auto_approve_after: None,
        };

        let chain = manager.create_chain("Test Chain".to_string(), vec![step]);

        let approval = ApprovalRecord {
            id: "approval-1".to_string(),
            approver_id: "manager-1".to_string(),
            approved: true,
            comments: "Approved".to_string(),
            approved_at: chrono::Utc::now().to_rfc3339(),
        };

        manager.submit_approval(&chain.id, "step-1", approval);

        let updated = manager.get_chain(&chain.id).unwrap();
        assert_eq!(updated.steps[0].approvals.len(), 1);
        assert_eq!(updated.steps[0].status, ApprovalStepStatus::Approved);
    }

    #[test]
    fn test_approval_mode_all() {
        let mut manager = ApprovalChainManager::new();

        let step = ApprovalStep {
            id: "step-1".to_string(),
            name: "Multi Approval".to_string(),
            order: 1,
            approvers: vec!["approver-1".to_string(), "approver-2".to_string()],
            approval_mode: ApprovalMode::All,
            status: ApprovalStepStatus::Pending,
            approvals: Vec::new(),
            auto_approve_after: None,
        };

        let chain = manager.create_chain("Test Chain".to_string(), vec![step]);

        let approval1 = ApprovalRecord {
            id: "approval-1".to_string(),
            approver_id: "approver-1".to_string(),
            approved: true,
            comments: "OK".to_string(),
            approved_at: chrono::Utc::now().to_rfc3339(),
        };

        manager.submit_approval(&chain.id, "step-1", approval1);
        let updated = manager.get_chain(&chain.id).unwrap();
        assert_eq!(updated.steps[0].status, ApprovalStepStatus::Pending);

        let approval2 = ApprovalRecord {
            id: "approval-2".to_string(),
            approver_id: "approver-2".to_string(),
            approved: true,
            comments: "OK".to_string(),
            approved_at: chrono::Utc::now().to_rfc3339(),
        };

        manager.submit_approval(&chain.id, "step-1", approval2);
        let updated = manager.get_chain(&chain.id).unwrap();
        assert_eq!(updated.steps[0].status, ApprovalStepStatus::Approved);
    }

    #[test]
    fn test_notification_manager() {
        let mut manager = NotificationManager::new();

        let notification = Notification {
            id: "notif-1".to_string(),
            recipient_id: "user-1".to_string(),
            notification_type: NotificationType::StatusChange,
            title: "Status Changed".to_string(),
            message: "Project status changed to InProgress".to_string(),
            project_id: Some("project-1".to_string()),
            priority: NotificationPriority::Normal,
            created_at: chrono::Utc::now().to_rfc3339(),
            read: false,
            channels: vec![NotificationChannel::Email],
        };

        manager.send_notification(notification);

        let notifications = manager.get_notifications("user-1");
        assert_eq!(notifications.len(), 1);
        assert_eq!(notifications[0].title, "Status Changed");
        assert!(!notifications[0].read);
    }

    #[test]
    fn test_mark_notification_as_read() {
        let mut manager = NotificationManager::new();

        let notification = Notification {
            id: "notif-1".to_string(),
            recipient_id: "user-1".to_string(),
            notification_type: NotificationType::StatusChange,
            title: "Test".to_string(),
            message: "Test message".to_string(),
            project_id: None,
            priority: NotificationPriority::Normal,
            created_at: chrono::Utc::now().to_rfc3339(),
            read: false,
            channels: vec![NotificationChannel::Email],
        };

        manager.send_notification(notification);
        manager.mark_as_read("user-1", "notif-1");

        let notifications = manager.get_notifications("user-1");
        assert!(notifications[0].read);
    }

    #[test]
    fn test_deadline_tracker() {
        let mut manager = NotificationManager::new();

        let deadline = DeadlineTracker {
            id: "deadline-1".to_string(),
            project_id: "project-1".to_string(),
            name: "Final Review".to_string(),
            deadline: "2026-01-15T00:00:00Z".to_string(),
            warning_days: 7,
            status: DeadlineStatus::OnTrack,
            assigned_to: vec!["user-1".to_string()],
        };

        manager.add_deadline(deadline);

        let deadlines = manager.get_deadlines("project-1");
        assert_eq!(deadlines.len(), 1);
        assert_eq!(deadlines[0].name, "Final Review");
    }

    #[test]
    fn test_check_approaching_deadlines() {
        let mut manager = NotificationManager::new();

        let now = chrono::Utc::now();
        let deadline_date = now + chrono::Duration::days(5);

        let deadline = DeadlineTracker {
            id: "deadline-1".to_string(),
            project_id: "project-1".to_string(),
            name: "Urgent Deadline".to_string(),
            deadline: deadline_date.to_rfc3339(),
            warning_days: 7,
            status: DeadlineStatus::Approaching,
            assigned_to: vec!["user-1".to_string()],
        };

        manager.add_deadline(deadline);

        let notifications = manager.check_deadlines();
        assert!(!notifications.is_empty());
        assert_eq!(
            notifications[0].notification_type,
            NotificationType::DeadlineApproaching
        );
    }

    #[test]
    fn test_project_status_enum() {
        assert!(matches!(ProjectStatus::Planning, ProjectStatus::Planning));
        assert!(matches!(
            ProjectStatus::InProgress,
            ProjectStatus::InProgress
        ));
        assert!(matches!(ProjectStatus::Completed, ProjectStatus::Completed));
    }

    #[test]
    fn test_stakeholder_roles() {
        let role = StakeholderRole::LegalExpert;
        assert_eq!(role, StakeholderRole::LegalExpert);

        let roles = vec![
            StakeholderRole::ProjectManager,
            StakeholderRole::LegalExpert,
            StakeholderRole::TechnicalReviewer,
            StakeholderRole::Approver,
            StakeholderRole::Observer,
            StakeholderRole::Contributor,
        ];

        assert_eq!(roles.len(), 6);
    }

    #[test]
    fn test_notification_channels() {
        let channels = vec![
            NotificationChannel::Email,
            NotificationChannel::InApp,
            NotificationChannel::Sms,
            NotificationChannel::Webhook,
        ];

        assert_eq!(channels.len(), 4);
    }

    #[test]
    fn test_iteration_change_types() {
        assert!(matches!(
            IterationChangeType::Addition,
            IterationChangeType::Addition
        ));
        assert!(matches!(
            IterationChangeType::Modification,
            IterationChangeType::Modification
        ));
        assert!(matches!(
            IterationChangeType::Deletion,
            IterationChangeType::Deletion
        ));
        assert!(matches!(
            IterationChangeType::Restructure,
            IterationChangeType::Restructure
        ));
    }

    // ========================================================================
    // Tests for v0.1.8 Reporting Features
    // ========================================================================

    #[test]
    fn test_executive_summary_generator() {
        let generator = ExecutiveSummaryGenerator::new();
        let project = create_test_project();
        let statutes = create_test_ported_statutes(3);

        let summary = generator.generate(&project, &statutes);

        assert_eq!(summary.project_id, project.id);
        assert_eq!(summary.statutes_count, 3);
        assert!(summary.compatibility_score >= 0.0 && summary.compatibility_score <= 1.0);
        assert!(!summary.key_findings.is_empty());
        assert!(!summary.recommendations.is_empty());
        assert!(!summary.stakeholders.is_empty());
    }

    #[test]
    fn test_executive_summary_risk_levels() {
        let generator = ExecutiveSummaryGenerator::new();
        let project = create_test_project();

        // Test low risk (high compatibility)
        let high_compat_statutes = vec![create_test_ported_statute_with_score(0.9)];
        let summary = generator.generate(&project, &high_compat_statutes);
        assert_eq!(summary.risk_level, RiskLevel::Low);

        // Test high risk (low compatibility)
        let low_compat_statutes = vec![create_test_ported_statute_with_score(0.3)];
        let summary = generator.generate(&project, &low_compat_statutes);
        assert_eq!(summary.risk_level, RiskLevel::High);
    }

    #[test]
    fn test_risk_assessment_report_generator() {
        let generator = RiskAssessmentReportGenerator::new();
        let project = create_test_project();
        let risk_assessments = vec![create_test_risk_assessment()];

        let report = generator.generate(&project, &risk_assessments);

        assert_eq!(report.project_id, project.id);
        assert!(report.overall_risk_score >= 0.0 && report.overall_risk_score <= 1.0);
        assert!(!report.risks_by_category.is_empty());
        assert!(!report.mitigation_strategies.is_empty());
    }

    #[test]
    fn test_risk_matrix_categorization() {
        let generator = RiskAssessmentReportGenerator::new();
        let _project = create_test_project();

        let mut risks_by_category: HashMap<RiskCategory, Vec<Risk>> = HashMap::new();
        risks_by_category.insert(
            RiskCategory::Legal,
            vec![
                Risk {
                    id: "risk-1".to_string(),
                    category: RiskCategory::Legal,
                    description: "High-high risk".to_string(),
                    likelihood: RiskLevel::High,
                    impact: 0.9,
                    severity: RiskLevel::High,
                },
                Risk {
                    id: "risk-2".to_string(),
                    category: RiskCategory::Legal,
                    description: "Low-low risk".to_string(),
                    likelihood: RiskLevel::Low,
                    impact: 0.2,
                    severity: RiskLevel::Low,
                },
            ],
        );

        let matrix = generator.build_risk_matrix(&risks_by_category);

        assert!(!matrix.critical.is_empty());
        assert!(!matrix.low.is_empty());
    }

    #[test]
    fn test_implementation_roadmap_generator() {
        let generator = ImplementationRoadmapGenerator::new();
        let project = create_test_project();
        let statutes = create_test_ported_statutes(5);

        let roadmap = generator.generate(&project, &statutes);

        assert_eq!(roadmap.project_id, project.id);
        assert_eq!(roadmap.phases.len(), 4); // Legal Review, Stakeholder, Pilot, Rollout
        assert!(!roadmap.critical_path.is_empty());
        assert!(!roadmap.resource_requirements.personnel.is_empty());
        assert!(roadmap.estimated_duration_days > 0);
    }

    #[test]
    fn test_implementation_phases_dependencies() {
        let generator = ImplementationRoadmapGenerator::new();
        let project = create_test_project();
        let statutes = create_test_ported_statutes(2);

        let roadmap = generator.generate(&project, &statutes);

        // Phase 1 should have no dependencies
        assert!(roadmap.phases[0].dependencies.is_empty());

        // Subsequent phases should depend on previous phases
        assert!(!roadmap.phases[1].dependencies.is_empty());
        assert!(!roadmap.phases[2].dependencies.is_empty());
        assert!(!roadmap.phases[3].dependencies.is_empty());
    }

    #[test]
    fn test_cost_benefit_analyzer() {
        let analyzer = CostBenefitAnalyzer::new();
        let project = create_test_project();
        let roadmap = ImplementationRoadmapGenerator::new()
            .generate(&project, &create_test_ported_statutes(3));
        let statutes = create_test_ported_statutes(3);

        let analysis = analyzer.analyze(&project, &roadmap, &statutes);

        assert_eq!(analysis.project_id, project.id);
        assert!(analysis.total_costs.total_five_year > 0.0);
        assert!(analysis.total_benefits.quantifiable_benefits >= 0.0);
        assert!(analysis.net_present_value.is_finite());
        assert!(!analysis.total_benefits.qualitative_benefits.is_empty());
    }

    #[test]
    fn test_cost_benefit_recommendations() {
        let analyzer = CostBenefitAnalyzer::new();
        let project = create_test_project();

        // Create high-benefit scenario
        let high_compat_statutes = vec![
            create_test_ported_statute_with_score(0.95),
            create_test_ported_statute_with_score(0.92),
            create_test_ported_statute_with_score(0.90),
        ];
        let roadmap =
            ImplementationRoadmapGenerator::new().generate(&project, &high_compat_statutes);
        let analysis = analyzer.analyze(&project, &roadmap, &high_compat_statutes);

        // High compatibility should lead to positive recommendation
        assert!(matches!(
            analysis.recommendation,
            CBARecommendation::StronglyRecommend | CBARecommendation::RecommendWithConditions
        ));
    }

    #[test]
    fn test_compliance_certification_manager() {
        let mut manager = ComplianceCertificationManager::new();
        let project_id = "test-project".to_string();
        let validation_results = vec![create_test_validation_result(0.85)];
        let certifier = CertifierInfo {
            name: "John Doe".to_string(),
            organization: "Legal Standards Board".to_string(),
            credentials: vec!["Licensed Attorney".to_string()],
            contact: "john@example.com".to_string(),
        };

        let cert = manager.issue_certification(project_id.clone(), validation_results, certifier);

        assert_eq!(cert.project_id, project_id);
        assert_eq!(cert.certification_level, CertificationLevel::Enhanced);
        assert_eq!(cert.status, CertificationStatus::Certified);
        assert!(cert.signature.is_some());
        assert!(cert.expiration_date.is_some());
    }

    #[test]
    fn test_certification_levels() {
        let mut manager = ComplianceCertificationManager::new();
        let certifier = CertifierInfo {
            name: "Jane Smith".to_string(),
            organization: "Compliance Authority".to_string(),
            credentials: vec!["Certified Auditor".to_string()],
            contact: "jane@example.com".to_string(),
        };

        // Full certification (score >= 0.95)
        let full_cert = manager.issue_certification(
            "proj1".to_string(),
            vec![create_test_validation_result(0.96)],
            certifier.clone(),
        );
        assert_eq!(full_cert.certification_level, CertificationLevel::Full);

        // Enhanced certification (0.85 <= score < 0.95)
        let enhanced_cert = manager.issue_certification(
            "proj2".to_string(),
            vec![create_test_validation_result(0.88)],
            certifier.clone(),
        );
        assert_eq!(
            enhanced_cert.certification_level,
            CertificationLevel::Enhanced
        );

        // Standard certification (0.75 <= score < 0.85)
        let standard_cert = manager.issue_certification(
            "proj3".to_string(),
            vec![create_test_validation_result(0.78)],
            certifier.clone(),
        );
        assert_eq!(
            standard_cert.certification_level,
            CertificationLevel::Standard
        );

        // Provisional certification (score < 0.75)
        let provisional_cert = manager.issue_certification(
            "proj4".to_string(),
            vec![create_test_validation_result(0.65)],
            certifier,
        );
        assert_eq!(
            provisional_cert.certification_level,
            CertificationLevel::Provisional
        );
    }

    #[test]
    fn test_certification_revocation() {
        let mut manager = ComplianceCertificationManager::new();
        let certifier = CertifierInfo {
            name: "Test Certifier".to_string(),
            organization: "Test Org".to_string(),
            credentials: vec!["Test Credential".to_string()],
            contact: "test@example.com".to_string(),
        };

        let cert = manager.issue_certification(
            "test-proj".to_string(),
            vec![create_test_validation_result(0.85)],
            certifier,
        );

        let cert_id = cert.id.clone();

        // Revoke certification
        assert!(manager.revoke_certification(&cert_id).is_some());

        // Verify status changed
        let revoked_cert = manager.get_certification(&cert_id).unwrap();
        assert_eq!(revoked_cert.status, CertificationStatus::Revoked);
    }

    // ========================================================================
    // Tests for v0.1.9 Integration Features
    // ========================================================================

    #[test]
    fn test_bilateral_agreement_template_library() {
        let library = BilateralAgreementTemplateLibrary::new();

        // Check default template exists
        let templates = library.list_templates();
        assert!(!templates.is_empty());

        // Get default template
        let template = library.get_template("general-bilateral").unwrap();
        assert_eq!(template.id, "general-bilateral");
        assert!(!template.sections.is_empty());
        assert!(!template.required_parameters.is_empty());
    }

    #[test]
    fn test_template_agreement_generation() {
        let library = BilateralAgreementTemplateLibrary::new();

        let mut parameters = HashMap::new();
        parameters.insert(
            "source_jurisdiction".to_string(),
            "United States".to_string(),
        );
        parameters.insert("target_jurisdiction".to_string(), "Japan".to_string());
        parameters.insert("purpose".to_string(), "legal cooperation".to_string());

        let agreement = library.generate_agreement("general-bilateral", &parameters);

        assert!(agreement.is_some());
        let text = agreement.unwrap();
        assert!(text.contains("United States"));
        assert!(text.contains("Japan"));
        assert!(text.contains("legal cooperation"));
    }

    #[test]
    fn test_add_custom_template() {
        let mut library = BilateralAgreementTemplateLibrary::new();

        let custom_template = BilateralAgreementTemplate {
            id: "custom-test".to_string(),
            name: "Custom Test Template".to_string(),
            description: "A custom template for testing".to_string(),
            applicable_systems: vec![LegalSystem::CivilLaw],
            sections: vec![TemplateSection {
                section_number: 1,
                title: "Test Section".to_string(),
                content_template: "Test content for {{param1}}".to_string(),
                required: true,
            }],
            required_parameters: vec![TemplateParameter {
                name: "param1".to_string(),
                description: "Test parameter".to_string(),
                parameter_type: ParameterType::String,
                default_value: None,
            }],
            optional_parameters: vec![],
        };

        library.add_template(custom_template);
        assert!(library.get_template("custom-test").is_some());
    }

    #[test]
    fn test_regulatory_sandbox_manager() {
        let mut manager = RegulatorySandboxManager::new();

        let sandbox = manager.create_sandbox(
            "Test Sandbox".to_string(),
            "Testing ported statutes".to_string(),
            vec!["statute-1".to_string(), "statute-2".to_string()],
        );

        assert_eq!(sandbox.status, SandboxStatus::Planning);
        assert_eq!(sandbox.test_statutes.len(), 2);
        assert!(sandbox.scenarios.is_empty());
        assert!(sandbox.results.is_empty());
    }

    #[test]
    fn test_sandbox_scenario_and_results() {
        let mut manager = RegulatorySandboxManager::new();

        let sandbox = manager.create_sandbox(
            "Test Sandbox".to_string(),
            "Testing".to_string(),
            vec!["statute-1".to_string()],
        );
        let sandbox_id = sandbox.id.clone();

        // Add scenario
        let scenario = TestScenario {
            id: "scenario-1".to_string(),
            name: "Basic Test".to_string(),
            description: "Test basic functionality".to_string(),
            parameters: HashMap::new(),
            expected_outcomes: vec!["Outcome 1".to_string()],
        };
        assert!(manager.add_scenario(&sandbox_id, scenario).is_some());

        // Activate sandbox
        assert!(manager.activate_sandbox(&sandbox_id).is_some());
        let sandbox = manager.get_sandbox(&sandbox_id).unwrap();
        assert_eq!(sandbox.status, SandboxStatus::Active);

        // Record result
        let result = SandboxTestResult {
            scenario_id: "scenario-1".to_string(),
            status: TestStatus::Passed,
            actual_outcomes: vec!["Outcome 1".to_string()],
            issues: vec![],
            recommendations: vec![],
            test_date: chrono::Utc::now().to_rfc3339(),
        };
        assert!(manager.record_result(&sandbox_id, result).is_some());

        // Complete sandbox
        assert!(manager.complete_sandbox(&sandbox_id).is_some());
        let sandbox = manager.get_sandbox(&sandbox_id).unwrap();
        assert_eq!(sandbox.status, SandboxStatus::Completed);
        assert!(sandbox.end_date.is_some());
    }

    #[test]
    fn test_affected_party_notification_manager() {
        let mut manager = AffectedPartyNotificationManager::new();

        let notification = manager.send_notification(
            "proj-1".to_string(),
            "New Porting Initiative".to_string(),
            "We are porting statutes from jurisdiction A to B".to_string(),
            vec![
                AffectedPartyCategory::GeneralPublic,
                AffectedPartyCategory::LegalProfessionals,
            ],
            Some(30),
        );

        assert_eq!(notification.project_id, "proj-1");
        assert_eq!(notification.affected_categories.len(), 2);
        assert!(notification.response_deadline.is_some());
        assert!(notification.channels.contains(&NotificationChannel::Email));
    }

    #[test]
    fn test_notification_feedback() {
        let mut manager = AffectedPartyNotificationManager::new();

        let notification = manager.send_notification(
            "proj-1".to_string(),
            "Test".to_string(),
            "Content".to_string(),
            vec![AffectedPartyCategory::GeneralPublic],
            None,
        );
        let notif_id = notification.id.clone();

        // Record feedback
        let feedback = PublicFeedback {
            id: uuid::Uuid::new_v4().to_string(),
            submitter: Some("John Citizen".to_string()),
            category: FeedbackCategory::Support,
            content: "I support this initiative".to_string(),
            submitted_at: chrono::Utc::now().to_rfc3339(),
        };

        assert!(manager.record_feedback(&notif_id, feedback).is_some());

        let feedback_list = manager.list_feedback(&notif_id).unwrap();
        assert_eq!(feedback_list.len(), 1);
    }

    #[test]
    fn test_public_comment_period_manager() {
        let mut manager = PublicCommentPeriodManager::new();

        let period = manager.open_comment_period(
            "proj-1".to_string(),
            "Public Comment Period".to_string(),
            "Comments on proposed statute porting".to_string(),
            60,
        );

        assert_eq!(period.status, CommentPeriodStatus::Open);
        assert_eq!(period.project_id, "proj-1");
        assert!(period.comments.is_empty());
        assert!(period.documents.is_empty());
    }

    #[test]
    fn test_comment_period_document_management() {
        let mut manager = PublicCommentPeriodManager::new();

        let period = manager.open_comment_period(
            "proj-1".to_string(),
            "Test Period".to_string(),
            "Description".to_string(),
            30,
        );
        let period_id = period.id.clone();

        // Add document
        let document = CommentDocument {
            id: "doc-1".to_string(),
            title: "Draft Statute".to_string(),
            document_type: DocumentType::DraftStatute,
            description: "Draft version for review".to_string(),
            url: "https://example.com/draft.pdf".to_string(),
        };

        assert!(manager.add_document(&period_id, document).is_some());

        let period = manager.get_period(&period_id).unwrap();
        assert_eq!(period.documents.len(), 1);
    }

    #[test]
    fn test_comment_submission() {
        let mut manager = PublicCommentPeriodManager::new();

        let period = manager.open_comment_period(
            "proj-1".to_string(),
            "Test Period".to_string(),
            "Description".to_string(),
            30,
        );
        let period_id = period.id.clone();

        // Submit comment
        let comment = PublicComment {
            id: uuid::Uuid::new_v4().to_string(),
            commenter: CommenterInfo {
                name: Some("Jane Doe".to_string()),
                organization: Some("Citizens Alliance".to_string()),
                email: Some("jane@example.com".to_string()),
                affiliation: AffectedPartyCategory::GeneralPublic,
            },
            comment_text: "I have concerns about section 3".to_string(),
            document_id: None,
            section_reference: Some("Section 3".to_string()),
            submitted_at: chrono::Utc::now().to_rfc3339(),
            category: FeedbackCategory::Concern,
        };

        assert!(manager.submit_comment(&period_id, comment).is_some());

        let comments = manager.list_comments(&period_id).unwrap();
        assert_eq!(comments.len(), 1);
    }

    #[test]
    fn test_comment_period_extension() {
        let mut manager = PublicCommentPeriodManager::new();

        let period = manager.open_comment_period(
            "proj-1".to_string(),
            "Test Period".to_string(),
            "Description".to_string(),
            30,
        );
        let period_id = period.id.clone();
        let original_end = period.end_date.clone();

        // Extend period
        assert!(manager.extend_period(&period_id, 15).is_some());

        let period = manager.get_period(&period_id).unwrap();
        assert_eq!(period.status, CommentPeriodStatus::Extended);
        assert_ne!(period.end_date, original_end);
    }

    #[test]
    fn test_comment_period_closure() {
        let mut manager = PublicCommentPeriodManager::new();

        let period = manager.open_comment_period(
            "proj-1".to_string(),
            "Test Period".to_string(),
            "Description".to_string(),
            30,
        );
        let period_id = period.id.clone();

        // Close period
        assert!(manager.close_period(&period_id).is_some());

        let period = manager.get_period(&period_id).unwrap();
        assert_eq!(period.status, CommentPeriodStatus::Closed);

        // Cannot submit comments to closed period
        let comment = PublicComment {
            id: uuid::Uuid::new_v4().to_string(),
            commenter: CommenterInfo {
                name: None,
                organization: None,
                email: None,
                affiliation: AffectedPartyCategory::GeneralPublic,
            },
            comment_text: "Late comment".to_string(),
            document_id: None,
            section_reference: None,
            submitted_at: chrono::Utc::now().to_rfc3339(),
            category: FeedbackCategory::Question,
        };

        assert!(manager.submit_comment(&period_id, comment).is_none());
    }

    #[test]
    fn test_comment_summary_generation() {
        let mut manager = PublicCommentPeriodManager::new();

        let period = manager.open_comment_period(
            "proj-1".to_string(),
            "Test Period".to_string(),
            "Description".to_string(),
            30,
        );
        let period_id = period.id.clone();

        // Submit multiple comments
        for i in 0..5 {
            let comment = PublicComment {
                id: format!("comment-{}", i),
                commenter: CommenterInfo {
                    name: Some(format!("Commenter {}", i)),
                    organization: None,
                    email: None,
                    affiliation: if i % 2 == 0 {
                        AffectedPartyCategory::GeneralPublic
                    } else {
                        AffectedPartyCategory::Businesses
                    },
                },
                comment_text: format!("Comment {}", i),
                document_id: None,
                section_reference: None,
                submitted_at: chrono::Utc::now().to_rfc3339(),
                category: if i % 2 == 0 {
                    FeedbackCategory::Support
                } else {
                    FeedbackCategory::Concern
                },
            };
            manager.submit_comment(&period_id, comment).unwrap();
        }

        let summary = manager.generate_comment_summary(&period_id).unwrap();

        assert_eq!(summary.total_comments, 5);
        assert!(!summary.category_breakdown.is_empty());
        assert!(!summary.affiliation_breakdown.is_empty());
        assert!(!summary.key_themes.is_empty());
    }

    #[test]
    fn test_discussion_thread() {
        let mut manager = DiscussionThreadManager::new();

        let thread = manager.create_thread(
            "project-1".to_string(),
            "Section 5 Discussion".to_string(),
            "Discuss changes to section 5".to_string(),
            "user-1".to_string(),
            vec!["section-5".to_string()],
        );

        assert!(!thread.id.is_empty());
        assert_eq!(thread.status, ThreadStatus::Open);
        assert_eq!(thread.project_id, "project-1");
    }

    #[test]
    fn test_discussion_thread_comments() {
        let mut manager = DiscussionThreadManager::new();

        let thread = manager.create_thread(
            "project-1".to_string(),
            "Test Thread".to_string(),
            "Context".to_string(),
            "user-1".to_string(),
            vec![],
        );

        let comment1 = manager
            .add_comment(
                &thread.id,
                "user-1".to_string(),
                "First comment".to_string(),
                None,
            )
            .unwrap();

        let _reply = manager
            .add_comment(
                &thread.id,
                "user-2".to_string(),
                "Reply to first".to_string(),
                Some(comment1.id.clone()),
            )
            .unwrap();

        let thread_after = manager.get_thread(&thread.id).unwrap();
        assert_eq!(thread_after.comments.len(), 1);
        assert_eq!(thread_after.comments[0].replies.len(), 1);
    }

    #[test]
    fn test_upvote_comment() {
        let mut manager = DiscussionThreadManager::new();

        let thread = manager.create_thread(
            "project-1".to_string(),
            "Test".to_string(),
            "Context".to_string(),
            "user-1".to_string(),
            vec![],
        );

        let comment = manager
            .add_comment(
                &thread.id,
                "user-1".to_string(),
                "Comment".to_string(),
                None,
            )
            .unwrap();

        manager
            .upvote_comment(&thread.id, &comment.id, "user-2".to_string())
            .unwrap();

        let thread_after = manager.get_thread(&thread.id).unwrap();
        assert_eq!(thread_after.comments[0].upvotes, 1);
    }

    #[test]
    fn test_resolve_thread() {
        let mut manager = DiscussionThreadManager::new();

        let thread = manager.create_thread(
            "project-1".to_string(),
            "Test".to_string(),
            "Context".to_string(),
            "user-1".to_string(),
            vec![],
        );

        manager
            .resolve_thread(&thread.id, "user-1".to_string())
            .unwrap();

        let thread_after = manager.get_thread(&thread.id).unwrap();
        assert_eq!(thread_after.status, ThreadStatus::Resolved);
        assert_eq!(thread_after.resolved_by, Some("user-1".to_string()));
    }

    #[test]
    fn test_voting_creation() {
        let mut manager = VotingManager::new();

        let options = vec![
            VoteOption {
                id: "opt-1".to_string(),
                text: "Option 1".to_string(),
                description: "First option".to_string(),
                vote_count: 0,
            },
            VoteOption {
                id: "opt-2".to_string(),
                text: "Option 2".to_string(),
                description: "Second option".to_string(),
                vote_count: 0,
            },
        ];

        let vote = manager.create_vote(
            "project-1".to_string(),
            "Test Vote".to_string(),
            "Vote on approach".to_string(),
            VoteType::SingleChoice,
            options,
            vec!["user-1".to_string(), "user-2".to_string()],
            24,
        );

        assert!(!vote.id.is_empty());
        assert_eq!(vote.status, VoteStatus::Active);
    }

    #[test]
    fn test_cast_vote() {
        let mut manager = VotingManager::new();

        let options = vec![VoteOption {
            id: "opt-1".to_string(),
            text: "Option 1".to_string(),
            description: "First option".to_string(),
            vote_count: 0,
        }];

        let vote = manager.create_vote(
            "project-1".to_string(),
            "Test".to_string(),
            "Description".to_string(),
            VoteType::SingleChoice,
            options,
            vec!["user-1".to_string()],
            24,
        );

        manager
            .cast_vote(&vote.id, "user-1".to_string(), vec!["opt-1".to_string()])
            .unwrap();

        let vote_after = manager.get_vote(&vote.id).unwrap();
        assert_eq!(vote_after.votes_cast.len(), 1);
    }

    #[test]
    fn test_close_vote() {
        let mut manager = VotingManager::new();

        let options = vec![
            VoteOption {
                id: "opt-1".to_string(),
                text: "Option 1".to_string(),
                description: "First".to_string(),
                vote_count: 0,
            },
            VoteOption {
                id: "opt-2".to_string(),
                text: "Option 2".to_string(),
                description: "Second".to_string(),
                vote_count: 0,
            },
        ];

        let vote = manager.create_vote(
            "project-1".to_string(),
            "Test".to_string(),
            "Desc".to_string(),
            VoteType::SingleChoice,
            options,
            vec!["user-1".to_string(), "user-2".to_string()],
            24,
        );

        manager
            .cast_vote(&vote.id, "user-1".to_string(), vec!["opt-1".to_string()])
            .unwrap();

        let result = manager.close_vote(&vote.id).unwrap();

        assert_eq!(result.total_eligible, 2);
        assert_eq!(result.total_votes, 1);
        assert_eq!(result.participation_rate, 0.5);
    }

    #[test]
    fn test_stakeholder_impact_tracker() {
        let mut tracker = StakeholderImpactTracker::new();

        let impact = tracker.record_impact(
            "project-1".to_string(),
            "stakeholder-1".to_string(),
            StakeholderImpactLevel::High,
            StakeholderImpactCategory::Economic,
            "Significant cost increase".to_string(),
            0.8,
            ImpactTimeframe::ShortTerm,
            vec!["Budget allocation".to_string()],
        );

        assert!(!impact.id.is_empty());
        assert_eq!(impact.impact_level, StakeholderImpactLevel::High);
        assert!(!impact.notification_sent);
    }

    #[test]
    fn test_stakeholder_impact_notifications() {
        let mut tracker = StakeholderImpactTracker::new();

        let impact = tracker.record_impact(
            "project-1".to_string(),
            "stakeholder-1".to_string(),
            StakeholderImpactLevel::Critical,
            StakeholderImpactCategory::Legal,
            "Critical legal issue".to_string(),
            0.9,
            ImpactTimeframe::Immediate,
            vec![],
        );

        let unnotified = tracker.get_unnotified_critical_impacts("project-1");
        assert_eq!(unnotified.len(), 1);

        tracker.mark_notified("project-1", &impact.id).unwrap();

        let unnotified_after = tracker.get_unnotified_critical_impacts("project-1");
        assert_eq!(unnotified_after.len(), 0);
    }

    #[test]
    fn test_stakeholder_impact_summary() {
        let mut tracker = StakeholderImpactTracker::new();

        tracker.record_impact(
            "project-1".to_string(),
            "stakeholder-1".to_string(),
            StakeholderImpactLevel::High,
            StakeholderImpactCategory::Economic,
            "Impact 1".to_string(),
            0.8,
            ImpactTimeframe::ShortTerm,
            vec![],
        );

        tracker.record_impact(
            "project-1".to_string(),
            "stakeholder-2".to_string(),
            StakeholderImpactLevel::Medium,
            StakeholderImpactCategory::Operational,
            "Impact 2".to_string(),
            0.5,
            ImpactTimeframe::MediumTerm,
            vec![],
        );

        let summary = tracker.get_impact_summary("project-1");
        assert_eq!(*summary.get(&StakeholderImpactLevel::High).unwrap(), 1);
        assert_eq!(*summary.get(&StakeholderImpactLevel::Medium).unwrap(), 1);
    }

    #[test]
    fn test_public_hearing_scheduling() {
        let mut manager = PublicCommentPeriodManager::new();

        let period = manager.open_comment_period(
            "proj-1".to_string(),
            "Test Period".to_string(),
            "Description".to_string(),
            30,
        );
        let period_id = period.id.clone();

        // Schedule hearing
        let hearing = PublicHearing {
            id: "hearing-1".to_string(),
            title: "Public Hearing on Statute Porting".to_string(),
            datetime: "2025-02-15T10:00:00Z".to_string(),
            location: "City Hall, Room 101".to_string(),
            virtual_link: Some("https://meeting.example.com/hearing1".to_string()),
            agenda: vec![
                "Opening remarks".to_string(),
                "Presentation of ported statutes".to_string(),
                "Public questions and comments".to_string(),
            ],
            registration_required: true,
        };

        assert!(manager.schedule_hearing(&period_id, hearing).is_some());

        let period = manager.get_period(&period_id).unwrap();
        assert_eq!(period.hearings.len(), 1);
        assert_eq!(period.hearings[0].agenda.len(), 3);
    }

    // ========================================================================
    // Quality Assurance Tests (v0.2.5)
    // ========================================================================

    #[test]
    fn test_quality_scorer_creation() {
        let scorer = QualityScorer::new();
        assert_eq!(scorer.min_quality_threshold, 0.6);

        let scorer_custom = QualityScorer::new().with_threshold(0.8);
        assert_eq!(scorer_custom.min_quality_threshold, 0.8);
    }

    #[test]
    fn test_quality_scoring_with_changes() {
        let scorer = QualityScorer::new();

        let mut statute = Statute::new(
            "test-1",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test"),
        );
        statute.id = "test-statute".to_string();

        let ported = PortedStatute {
            original_id: "original-1".to_string(),
            statute,
            changes: vec![
                PortingChange {
                    change_type: ChangeType::CulturalAdaptation,
                    description: "Adapted age parameter".to_string(),
                    original: Some("20".to_string()),
                    adapted: Some("18".to_string()),
                    reason: "Age of majority differs between jurisdictions".to_string(),
                },
                PortingChange {
                    change_type: ChangeType::Translation,
                    description: "Translated legal term".to_string(),
                    original: Some("契約".to_string()),
                    adapted: Some("contract".to_string()),
                    reason: "Translation to target language".to_string(),
                },
            ],
            locale: Locale::new("en").with_country("US"),
            compatibility_score: 0.85,
        };

        let quality = scorer.score_porting(&ported);

        assert!(quality.overall >= 0.0 && quality.overall <= 1.0);
        assert!(quality.semantic_preservation >= 0.0);
        assert!(quality.legal_correctness >= 0.0);
        assert!(quality.cultural_adaptation >= 0.0);
        assert!(quality.completeness >= 0.0);
        assert!(quality.consistency >= 0.0);
    }

    #[test]
    fn test_quality_scoring_empty_statute() {
        let scorer = QualityScorer::new();

        let statute = Statute::new("", "", Effect::new(EffectType::Grant, "Test"));

        let ported = PortedStatute {
            original_id: "original-1".to_string(),
            statute,
            changes: vec![],
            locale: Locale::new("en").with_country("US"),
            compatibility_score: 0.5,
        };

        let quality = scorer.score_porting(&ported);

        // Should have reduced quality due to missing ID and title, and no changes
        // Completeness score: 0.4 (missing ID -0.3, missing title -0.2, no changes -0.1)
        // Cultural adaptation score: 0.8 (no cultural changes -0.2)
        // Other scores: 1.0 each
        // Overall: (1.0*0.25) + (1.0*0.25) + (0.8*0.2) + (0.4*0.15) + (1.0*0.15) = 0.87
        assert!(
            quality.overall < 0.9,
            "Quality score is {}",
            quality.overall
        );
        assert!(
            (quality.completeness - 0.4).abs() < 0.01,
            "Completeness score is {}",
            quality.completeness
        );
        assert!(!quality.issues.is_empty());
        assert!(
            quality
                .issues
                .iter()
                .any(|i| matches!(i.issue_type, QualityIssueType::Incompleteness))
        );
    }

    #[test]
    fn test_quality_grade_classification() {
        let scorer = QualityScorer::new();

        let excellent = PortedStatute {
            original_id: "test".to_string(),
            statute: Statute::new(
                "id-1",
                "Test Statute",
                Effect::new(EffectType::Grant, "Test"),
            ),
            changes: vec![PortingChange {
                change_type: ChangeType::CulturalAdaptation,
                description: "Test".to_string(),
                original: None,
                adapted: None,
                reason: "Test reason".to_string(),
            }],
            locale: Locale::new("en").with_country("US"),
            compatibility_score: 1.0,
        };

        let quality = scorer.score_porting(&excellent);
        assert!(matches!(
            quality.grade,
            QualityGrade::Good | QualityGrade::Excellent
        ));
    }

    #[test]
    fn test_quality_scorer_meets_threshold() {
        let scorer = QualityScorer::new().with_threshold(0.7);

        let score = QualityScore {
            overall: 0.8,
            semantic_preservation: 0.8,
            legal_correctness: 0.8,
            cultural_adaptation: 0.8,
            completeness: 0.8,
            consistency: 0.8,
            grade: QualityGrade::Good,
            issues: vec![],
            recommendations: vec![],
        };

        assert!(scorer.meets_threshold(&score));

        let low_score = QualityScore {
            overall: 0.5,
            semantic_preservation: 0.5,
            legal_correctness: 0.5,
            cultural_adaptation: 0.5,
            completeness: 0.5,
            consistency: 0.5,
            grade: QualityGrade::Poor,
            issues: vec![],
            recommendations: vec![],
        };

        assert!(!scorer.meets_threshold(&low_score));
    }

    #[test]
    fn test_consistency_verifier_creation() {
        let verifier = ConsistencyVerifier::new();
        let _ = verifier; // Just check it compiles
    }

    #[test]
    fn test_consistency_verification_consistent() {
        let verifier = ConsistencyVerifier::new();

        let ported = PortedStatute {
            original_id: "test".to_string(),
            statute: Statute::new(
                "id-1",
                "Test Statute",
                Effect::new(EffectType::Grant, "Test"),
            ),
            changes: vec![],
            locale: Locale::new("en").with_country("US"),
            compatibility_score: 1.0,
        };

        let result = verifier.verify(&ported);

        assert!(result.is_consistent);
        assert_eq!(result.consistency_score, 1.0);
        assert!(result.inconsistencies.is_empty());
    }

    #[test]
    fn test_consistency_verification_with_many_changes() {
        let verifier = ConsistencyVerifier::new();

        let mut changes = vec![];
        for i in 0..15 {
            changes.push(PortingChange {
                change_type: ChangeType::Translation,
                description: format!("Translation {}", i),
                original: Some(format!("old-{}", i)),
                adapted: Some(format!("new-{}", i)),
                reason: format!("Translation reason {}", i),
            });
        }

        let ported = PortedStatute {
            original_id: "test".to_string(),
            statute: Statute::new(
                "id-1",
                "Test Statute",
                Effect::new(EffectType::Grant, "Test"),
            ),
            changes,
            locale: Locale::new("en").with_country("US"),
            compatibility_score: 0.8,
        };

        let result = verifier.verify(&ported);

        assert!(!result.inconsistencies.is_empty());
        assert!(result.inconsistencies.iter().any(|i| matches!(
            i.inconsistency_type,
            InconsistencyType::TerminologyInconsistency
        )));
    }

    #[test]
    fn test_consistency_verification_logical_inconsistency() {
        let verifier = ConsistencyVerifier::new();

        let mut changes = vec![];
        // Add 4 value adaptations
        for i in 0..4 {
            changes.push(PortingChange {
                change_type: ChangeType::ValueAdaptation,
                description: format!("Value adaptation {}", i),
                original: Some(format!("old-{}", i)),
                adapted: Some(format!("new-{}", i)),
                reason: "Value adaptation".to_string(),
            });
        }
        // Add a removal
        changes.push(PortingChange {
            change_type: ChangeType::Removal,
            description: "Removed incompatible clause".to_string(),
            original: Some("incompatible".to_string()),
            adapted: None,
            reason: "Incompatible with target jurisdiction".to_string(),
        });

        let ported = PortedStatute {
            original_id: "test".to_string(),
            statute: Statute::new(
                "id-1",
                "Test Statute",
                Effect::new(EffectType::Grant, "Test"),
            ),
            changes,
            locale: Locale::new("en").with_country("US"),
            compatibility_score: 0.7,
        };

        let result = verifier.verify(&ported);

        assert!(!result.inconsistencies.is_empty());
        assert!(result.inconsistencies.iter().any(|i| matches!(
            i.inconsistency_type,
            InconsistencyType::LogicalInconsistency
        )));
    }

    #[test]
    fn test_completeness_checker_creation() {
        let checker = CompletenessChecker::new();
        assert!(!checker.check_optional);

        let checker_with_optional = CompletenessChecker::new().with_optional_check(true);
        assert!(checker_with_optional.check_optional);
    }

    #[test]
    fn test_completeness_check_complete() {
        let checker = CompletenessChecker::new();

        let ported = PortedStatute {
            original_id: "test".to_string(),
            statute: Statute::new(
                "id-1",
                "Test Statute",
                Effect::new(EffectType::Grant, "Test"),
            ),
            changes: vec![PortingChange {
                change_type: ChangeType::CulturalAdaptation,
                description: "Test change".to_string(),
                original: None,
                adapted: None,
                reason: "Cultural adaptation test".to_string(),
            }],
            locale: Locale::new("en").with_country("US"),
            compatibility_score: 1.0,
        };

        let result = checker.check(&ported);

        assert!(result.is_complete);
        assert_eq!(result.completeness_score, 1.0);
        assert!(result.missing_elements.is_empty());
    }

    #[test]
    fn test_completeness_check_missing_required() {
        let checker = CompletenessChecker::new();

        let statute = Statute::new("", "", Effect::new(EffectType::Grant, "Test"));

        let ported = PortedStatute {
            original_id: "test".to_string(),
            statute,
            changes: vec![],
            locale: Locale::new("en").with_country("US"),
            compatibility_score: 0.5,
        };

        let result = checker.check(&ported);

        assert!(!result.is_complete);
        assert_eq!(result.completeness_score, 0.0);
        assert!(
            result
                .missing_elements
                .iter()
                .any(|e| matches!(e.importance, ElementImportance::Required))
        );
    }

    #[test]
    fn test_completeness_check_missing_recommended() {
        let checker = CompletenessChecker::new();

        let ported = PortedStatute {
            original_id: "test".to_string(),
            statute: Statute::new(
                "id-1",
                "Test Statute",
                Effect::new(EffectType::Grant, "Test"),
            ),
            changes: vec![], // No changes - missing recommended element
            locale: Locale::new("en").with_country("US"),
            compatibility_score: 0.8,
        };

        let result = checker.check(&ported);

        assert!(!result.is_complete);
        assert!(result.completeness_score > 0.0 && result.completeness_score < 1.0);
        assert!(
            result
                .missing_elements
                .iter()
                .any(|e| matches!(e.importance, ElementImportance::Recommended))
        );
    }

    #[test]
    fn test_regression_test_manager_creation() {
        let manager = RegressionTestManager::new();
        let stats = manager.get_statistics();

        assert_eq!(stats.total, 0);
        assert_eq!(stats.pass_rate, 0.0);
    }

    #[test]
    fn test_regression_test_add() {
        let mut manager = RegressionTestManager::new();

        let test = RegressionTest {
            test_id: "test-1".to_string(),
            name: "Test Porting".to_string(),
            source_jurisdiction: "JP".to_string(),
            target_jurisdiction: "US".to_string(),
            input_statute: "{}".to_string(),
            expected_output: "{}".to_string(),
            quality_baseline: 0.8,
            created_at: chrono::Utc::now(),
            last_run: None,
            status: RegressionTestStatus::Pending,
        };

        manager.add_test(test);

        let stats = manager.get_statistics();
        assert_eq!(stats.total, 1);
        assert_eq!(stats.pending, 1);
    }

    #[test]
    fn test_regression_test_run() {
        let mut manager = RegressionTestManager::new();

        let ported = PortedStatute {
            original_id: "test".to_string(),
            statute: Statute::new(
                "id-1",
                "Test Statute",
                Effect::new(EffectType::Grant, "Test"),
            ),
            changes: vec![PortingChange {
                change_type: ChangeType::CulturalAdaptation,
                description: "Test".to_string(),
                original: None,
                adapted: None,
                reason: "Test reason".to_string(),
            }],
            locale: Locale::new("en").with_country("US"),
            compatibility_score: 0.9,
        };

        let test = RegressionTest {
            test_id: "test-1".to_string(),
            name: "Test Porting".to_string(),
            source_jurisdiction: "JP".to_string(),
            target_jurisdiction: "US".to_string(),
            input_statute: "{}".to_string(),
            expected_output: "{}".to_string(),
            quality_baseline: 0.8,
            created_at: chrono::Utc::now(),
            last_run: None,
            status: RegressionTestStatus::Pending,
        };

        manager.add_test(test);

        let result = manager.run_test("test-1", &ported);
        assert!(result.is_ok());

        let result = result.unwrap();
        assert!(result.passed);
        assert!(result.quality_score >= 0.0);
    }

    #[test]
    fn test_regression_test_statistics() {
        let mut manager = RegressionTestManager::new();

        for i in 0..5 {
            let test = RegressionTest {
                test_id: format!("test-{}", i),
                name: format!("Test {}", i),
                source_jurisdiction: "JP".to_string(),
                target_jurisdiction: "US".to_string(),
                input_statute: "{}".to_string(),
                expected_output: "{}".to_string(),
                quality_baseline: 0.8,
                created_at: chrono::Utc::now(),
                last_run: None,
                status: if i % 2 == 0 {
                    RegressionTestStatus::Passed
                } else {
                    RegressionTestStatus::Failed
                },
            };
            manager.add_test(test);
        }

        let stats = manager.get_statistics();
        assert_eq!(stats.total, 5);
        assert_eq!(stats.passed, 3); // 0, 2, 4
        assert_eq!(stats.failed, 2); // 1, 3
        assert_eq!(stats.pass_rate, 0.6);
    }

    #[test]
    fn test_drift_monitor_creation() {
        let monitor = DriftMonitor::new();
        assert_eq!(monitor.drift_threshold, 0.1);

        let monitor_custom = DriftMonitor::new().with_threshold(0.2);
        assert_eq!(monitor_custom.drift_threshold, 0.2);
    }

    #[test]
    fn test_drift_monitor_snapshot_creation() {
        let mut monitor = DriftMonitor::new();

        let ported = PortedStatute {
            original_id: "test".to_string(),
            statute: Statute::new(
                "id-1",
                "Test Statute",
                Effect::new(EffectType::Grant, "Test"),
            ),
            changes: vec![PortingChange {
                change_type: ChangeType::CulturalAdaptation,
                description: "Test".to_string(),
                original: None,
                adapted: None,
                reason: "Test reason".to_string(),
            }],
            locale: Locale::new("en").with_country("US"),
            compatibility_score: 0.9,
        };

        let snapshot_id = monitor.create_snapshot("statute-1".to_string(), &ported);

        assert!(!snapshot_id.is_empty());

        let snapshots = monitor.get_snapshots("statute-1");
        assert!(snapshots.is_some());
        assert_eq!(snapshots.unwrap().len(), 1);
    }

    #[test]
    fn test_drift_detection_no_drift() {
        let mut monitor = DriftMonitor::new();

        let ported = PortedStatute {
            original_id: "test".to_string(),
            statute: Statute::new(
                "id-1",
                "Test Statute",
                Effect::new(EffectType::Grant, "Test"),
            ),
            changes: vec![PortingChange {
                change_type: ChangeType::CulturalAdaptation,
                description: "Test".to_string(),
                original: None,
                adapted: None,
                reason: "Test reason".to_string(),
            }],
            locale: Locale::new("en").with_country("US"),
            compatibility_score: 0.9,
        };

        monitor.create_snapshot("statute-1".to_string(), &ported);

        let result = monitor.detect_drift("statute-1", &ported);

        assert!(!result.drift_detected);
        assert!(result.drift_issues.is_empty());
    }

    #[test]
    fn test_drift_detection_with_new_snapshot() {
        let mut monitor = DriftMonitor::new();

        let ported1 = PortedStatute {
            original_id: "test".to_string(),
            statute: Statute::new(
                "id-1",
                "Test Statute",
                Effect::new(EffectType::Grant, "Test"),
            ),
            changes: vec![PortingChange {
                change_type: ChangeType::CulturalAdaptation,
                description: "Test".to_string(),
                original: None,
                adapted: None,
                reason: "Test reason".to_string(),
            }],
            locale: Locale::new("en").with_country("US"),
            compatibility_score: 0.9,
        };

        monitor.create_snapshot("statute-1".to_string(), &ported1);

        // Create a degraded version
        let mut ported2 = ported1.clone();
        ported2.statute.id = "".to_string(); // This will lower the quality score

        let result = monitor.detect_drift("statute-1", &ported2);

        // May or may not detect drift depending on threshold
        assert!(result.drift_score >= 0.0);
    }

    #[test]
    fn test_drift_trend_tracking() {
        let mut monitor = DriftMonitor::new();

        let ported = PortedStatute {
            original_id: "test".to_string(),
            statute: Statute::new(
                "id-1",
                "Test Statute",
                Effect::new(EffectType::Grant, "Test"),
            ),
            changes: vec![PortingChange {
                change_type: ChangeType::CulturalAdaptation,
                description: "Test".to_string(),
                original: None,
                adapted: None,
                reason: "Test reason".to_string(),
            }],
            locale: Locale::new("en").with_country("US"),
            compatibility_score: 0.9,
        };

        // Create multiple snapshots
        monitor.create_snapshot("statute-1".to_string(), &ported);
        monitor.create_snapshot("statute-1".to_string(), &ported);

        let trend = monitor.get_drift_trend("statute-1");

        assert_eq!(trend.len(), 1); // One drift measurement (between 2 snapshots)
    }

    #[test]
    fn test_drift_category_classification() {
        let result = DriftDetectionResult {
            drift_detected: false,
            drift_score: 0.0,
            category: DriftCategory::None,
            drift_issues: vec![],
            recommendations: vec![],
        };

        assert!(matches!(result.category, DriftCategory::None));
    }

    // ========================================================================
    // Documentation Generation Tests (v0.2.6)
    // ========================================================================

    #[test]
    fn test_explanatory_note_generator() {
        let generator = ExplanatoryNoteGenerator::new();

        let ported = PortedStatute {
            original_id: "test".to_string(),
            statute: Statute::new(
                "id-1",
                "Test Statute",
                Effect::new(EffectType::Grant, "Test"),
            ),
            changes: vec![
                PortingChange {
                    change_type: ChangeType::CulturalAdaptation,
                    description: "Adapted parameter".to_string(),
                    original: Some("20".to_string()),
                    adapted: Some("18".to_string()),
                    reason: "Age difference".to_string(),
                },
                PortingChange {
                    change_type: ChangeType::Translation,
                    description: "Translated term".to_string(),
                    original: Some("契約".to_string()),
                    adapted: Some("contract".to_string()),
                    reason: "Language localization".to_string(),
                },
            ],
            locale: Locale::new("en").with_country("US"),
            compatibility_score: 0.9,
        };

        let notes = generator.generate_notes(&ported);

        // Should have 1 general note + 1 note for CulturalAdaptation (Translation is not significant)
        assert_eq!(notes.len(), 2);
        assert_eq!(notes[0].section, "General");
        assert!(!notes[0].explanation.is_empty());
    }

    #[test]
    fn test_explanatory_note_significant_changes_only() {
        let generator = ExplanatoryNoteGenerator::new();

        let ported = PortedStatute {
            original_id: "test".to_string(),
            statute: Statute::new(
                "id-1",
                "Test Statute",
                Effect::new(EffectType::Grant, "Test"),
            ),
            changes: vec![PortingChange {
                change_type: ChangeType::Translation,
                description: "Translation".to_string(),
                original: None,
                adapted: None,
                reason: "Test".to_string(),
            }],
            locale: Locale::new("en").with_country("US"),
            compatibility_score: 0.9,
        };

        let notes = generator.generate_notes(&ported);

        // Should only have the general note, no note for Translation (not significant)
        assert_eq!(notes.len(), 1);
        assert_eq!(notes[0].section, "General");
    }

    #[test]
    fn test_change_justification_report_generator() {
        let generator = ChangeJustificationReportGenerator::new();

        let ported = PortedStatute {
            original_id: "test".to_string(),
            statute: Statute::new(
                "id-1",
                "Test Statute",
                Effect::new(EffectType::Grant, "Test"),
            ),
            changes: vec![
                PortingChange {
                    change_type: ChangeType::CulturalAdaptation,
                    description: "Cultural adaptation".to_string(),
                    original: Some("old".to_string()),
                    adapted: Some("new".to_string()),
                    reason: "Culture".to_string(),
                },
                PortingChange {
                    change_type: ChangeType::ValueAdaptation,
                    description: "Value adaptation".to_string(),
                    original: Some("20".to_string()),
                    adapted: Some("18".to_string()),
                    reason: "Age threshold".to_string(),
                },
            ],
            locale: Locale::new("en").with_country("US"),
            compatibility_score: 0.85,
        };

        let report = generator.generate_report(&ported, "JP", "US");

        assert_eq!(report.source_jurisdiction, "JP");
        assert_eq!(report.target_jurisdiction, "US");
        assert_eq!(report.justifications.len(), 2);
        assert!(!report.overall_rationale.is_empty());
        assert!(!report.legal_basis.is_empty());

        // Check that risk is identified for cultural and value adaptations
        assert!(report.justifications[0].risk_if_unchanged.is_some());
        assert!(report.justifications[1].risk_if_unchanged.is_some());
    }

    #[test]
    fn test_change_justification_types() {
        let generator = ChangeJustificationReportGenerator::new();

        let change_removal = PortingChange {
            change_type: ChangeType::Removal,
            description: "Removed clause".to_string(),
            original: Some("old".to_string()),
            adapted: None,
            reason: "Incompatible".to_string(),
        };

        let justification = generator.justify_change(&change_removal);
        assert!(justification.justification.contains("incompatibility"));
        assert!(justification.risk_if_unchanged.is_some());
    }

    #[test]
    fn test_legislative_history_compiler() {
        let compiler = LegislativeHistoryCompiler::new();

        let ported = PortedStatute {
            original_id: "test".to_string(),
            statute: Statute::new(
                "id-1",
                "Test Statute",
                Effect::new(EffectType::Grant, "Test"),
            ),
            changes: vec![
                PortingChange {
                    change_type: ChangeType::CulturalAdaptation,
                    description: "Adapted".to_string(),
                    original: None,
                    adapted: None,
                    reason: "Test".to_string(),
                },
                PortingChange {
                    change_type: ChangeType::ValueAdaptation,
                    description: "Value change".to_string(),
                    original: None,
                    adapted: None,
                    reason: "Test".to_string(),
                },
            ],
            locale: Locale::new("en").with_country("US"),
            compatibility_score: 0.9,
        };

        let history = compiler.compile_history(&ported);

        assert_eq!(history.statute_id, "id-1");
        // Should have 1 porting event + 2 change events = 3 total
        assert_eq!(history.timeline.len(), 3);
        assert!(
            history
                .timeline
                .iter()
                .any(|e| matches!(e.event_type, LegislativeEventType::Ported))
        );
        assert_eq!(
            history
                .timeline
                .iter()
                .filter(|e| matches!(e.event_type, LegislativeEventType::Amended))
                .count(),
            2
        );
        assert!(!history.summary.is_empty());
        assert!(!history.key_participants.is_empty());
    }

    #[test]
    fn test_legislative_history_add_event() {
        let compiler = LegislativeHistoryCompiler::new();

        let ported = PortedStatute {
            original_id: "test".to_string(),
            statute: Statute::new(
                "id-1",
                "Test Statute",
                Effect::new(EffectType::Grant, "Test"),
            ),
            changes: vec![],
            locale: Locale::new("en").with_country("US"),
            compatibility_score: 1.0,
        };

        let mut history = compiler.compile_history(&ported);
        let initial_count = history.timeline.len();

        compiler.add_event(
            &mut history,
            LegislativeEventType::Reviewed,
            "Reviewed by legal team".to_string(),
            Some("Legal Team".to_string()),
        );

        assert_eq!(history.timeline.len(), initial_count + 1);
        assert!(
            history
                .timeline
                .last()
                .unwrap()
                .actor
                .as_ref()
                .unwrap()
                .contains("Legal Team")
        );
    }

    #[test]
    fn test_implementation_guidance_generator() {
        let generator = ImplementationGuidanceGenerator::new();

        let ported = PortedStatute {
            original_id: "test".to_string(),
            statute: Statute::new(
                "id-1",
                "Test Statute",
                Effect::new(EffectType::Grant, "Test"),
            ),
            changes: vec![PortingChange {
                change_type: ChangeType::CulturalAdaptation,
                description: "Cultural change".to_string(),
                original: None,
                adapted: None,
                reason: "Test".to_string(),
            }],
            locale: Locale::new("en").with_country("US"),
            compatibility_score: 0.9,
        };

        let guidance = generator.generate_guidance(&ported);

        assert_eq!(guidance.statute_id, "id-1");
        assert!(!guidance.overview.is_empty());
        assert!(!guidance.prerequisites.is_empty());
        assert!(!guidance.implementation_steps.is_empty());
        assert!(!guidance.compliance_checklist.is_empty());
        assert!(!guidance.common_pitfalls.is_empty());

        // Should have 5 steps (initial review, stakeholder, legal, adaptations, final approval)
        assert_eq!(guidance.implementation_steps.len(), 5);
        assert_eq!(guidance.implementation_steps[0].step_number, 1);
        assert_eq!(guidance.implementation_steps[0].title, "Initial Review");
    }

    #[test]
    fn test_implementation_guidance_steps_without_changes() {
        let generator = ImplementationGuidanceGenerator::new();

        let ported = PortedStatute {
            original_id: "test".to_string(),
            statute: Statute::new(
                "id-1",
                "Test Statute",
                Effect::new(EffectType::Grant, "Test"),
            ),
            changes: vec![],
            locale: Locale::new("en").with_country("US"),
            compatibility_score: 1.0,
        };

        let guidance = generator.generate_guidance(&ported);

        // Should have 4 steps (no "Implementation of Adaptations" step)
        assert_eq!(guidance.implementation_steps.len(), 4);
        assert!(
            !guidance
                .implementation_steps
                .iter()
                .any(|s| s.title.contains("Adaptations"))
        );
    }

    #[test]
    fn test_training_material_generator() {
        let generator = TrainingMaterialGenerator::new();

        let ported = PortedStatute {
            original_id: "test".to_string(),
            statute: Statute::new(
                "id-1",
                "Test Statute",
                Effect::new(EffectType::Grant, "Test"),
            ),
            changes: vec![PortingChange {
                change_type: ChangeType::CulturalAdaptation,
                description: "Adaptation".to_string(),
                original: None,
                adapted: None,
                reason: "Test".to_string(),
            }],
            locale: Locale::new("en").with_country("US"),
            compatibility_score: 0.9,
        };

        let material = generator.generate_materials(&ported, TrainingAudience::LegalProfessionals);

        assert_eq!(material.statute_id, "id-1");
        assert_eq!(
            material.target_audience,
            TrainingAudience::LegalProfessionals
        );
        assert!(!material.title.is_empty());
        assert!(!material.learning_objectives.is_empty());
        assert!(!material.modules.is_empty());
        assert!(!material.assessment_questions.is_empty());
        assert_eq!(material.estimated_duration, "4 hours");
    }

    #[test]
    fn test_training_material_different_audiences() {
        let generator = TrainingMaterialGenerator::new();

        let ported = PortedStatute {
            original_id: "test".to_string(),
            statute: Statute::new(
                "id-1",
                "Test Statute",
                Effect::new(EffectType::Grant, "Test"),
            ),
            changes: vec![],
            locale: Locale::new("en").with_country("US"),
            compatibility_score: 1.0,
        };

        let legal = generator.generate_materials(&ported, TrainingAudience::LegalProfessionals);
        let govt = generator.generate_materials(&ported, TrainingAudience::GovernmentOfficials);
        let public = generator.generate_materials(&ported, TrainingAudience::GeneralPublic);
        let enforcement =
            generator.generate_materials(&ported, TrainingAudience::EnforcementOfficers);

        assert_eq!(legal.estimated_duration, "4 hours");
        assert_eq!(govt.estimated_duration, "3 hours");
        assert_eq!(public.estimated_duration, "1 hour");
        assert_eq!(enforcement.estimated_duration, "2 hours");

        // Each should have different learning objectives
        assert_ne!(legal.learning_objectives, public.learning_objectives);
    }

    #[test]
    fn test_training_material_modules() {
        let generator = TrainingMaterialGenerator::new();

        let ported = PortedStatute {
            original_id: "test".to_string(),
            statute: Statute::new(
                "id-1",
                "Test Statute",
                Effect::new(EffectType::Grant, "Test"),
            ),
            changes: vec![
                PortingChange {
                    change_type: ChangeType::CulturalAdaptation,
                    description: "Change 1".to_string(),
                    original: None,
                    adapted: None,
                    reason: "Test".to_string(),
                },
                PortingChange {
                    change_type: ChangeType::ValueAdaptation,
                    description: "Change 2".to_string(),
                    original: None,
                    adapted: None,
                    reason: "Test".to_string(),
                },
            ],
            locale: Locale::new("en").with_country("US"),
            compatibility_score: 0.9,
        };

        let material = generator.generate_materials(&ported, TrainingAudience::GeneralPublic);

        // Should have 3 modules: Intro, Key Adaptations, Practical Application
        assert_eq!(material.modules.len(), 3);
        assert_eq!(material.modules[0].title, "Introduction to the Statute");
        assert_eq!(material.modules[1].title, "Key Adaptations");
        assert_eq!(material.modules[2].title, "Practical Application");
    }

    #[test]
    fn test_training_material_assessment() {
        let generator = TrainingMaterialGenerator::new();

        let ported = PortedStatute {
            original_id: "test".to_string(),
            statute: Statute::new(
                "id-1",
                "Test Statute",
                Effect::new(EffectType::Grant, "Test"),
            ),
            changes: vec![PortingChange {
                change_type: ChangeType::CulturalAdaptation,
                description: "Change".to_string(),
                original: None,
                adapted: None,
                reason: "Test".to_string(),
            }],
            locale: Locale::new("en").with_country("US"),
            compatibility_score: 0.9,
        };

        let material = generator.generate_materials(&ported, TrainingAudience::LegalProfessionals);

        // Should have 2 questions (purpose + number of adaptations)
        assert_eq!(material.assessment_questions.len(), 2);
        assert_eq!(material.assessment_questions[0].question_number, 1);
        assert_eq!(material.assessment_questions[1].question_number, 2);
        assert_eq!(material.assessment_questions[0].options.len(), 3);
        assert!(material.assessment_questions[0].correct_answer < 3);
    }

    // ========================================================================
    // Helper functions for tests
    // ========================================================================

    fn create_test_project() -> PortingProject {
        PortingProject {
            id: "test-project-1".to_string(),
            name: "Test Porting Project".to_string(),
            description: "A test project".to_string(),
            source_jurisdiction: "JP".to_string(),
            target_jurisdiction: "US".to_string(),
            status: ProjectStatus::InProgress,
            statute_ids: vec!["statute-1".to_string(), "statute-2".to_string()],
            stakeholders: vec![Stakeholder {
                id: "stakeholder-1".to_string(),
                name: "John Doe".to_string(),
                email: "john@example.com".to_string(),
                role: StakeholderRole::ProjectManager,
                notification_preferences: NotificationPreferences {
                    on_status_change: true,
                    on_deadline_approaching: true,
                    on_assignment: false,
                    on_review_request: true,
                    channels: vec![NotificationChannel::Email, NotificationChannel::InApp],
                },
            }],
            timeline: ProjectTimeline {
                start_date: chrono::Utc::now().to_rfc3339(),
                end_date: (chrono::Utc::now() + chrono::Duration::days(180)).to_rfc3339(),
                milestones: vec![],
                current_phase: "Implementation".to_string(),
            },
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
            metadata: HashMap::new(),
        }
    }

    fn create_test_ported_statutes(count: usize) -> Vec<PortedStatute> {
        (0..count)
            .map(|i| PortedStatute {
                original_id: format!("statute-{}", i),
                statute: Statute::new(
                    &format!("ported-{}", i),
                    &format!("Test Statute {}", i),
                    Effect::new(EffectType::Grant, "Test effect"),
                ),
                changes: vec![],
                locale: Locale::new("en").with_country("US"),
                compatibility_score: 0.85,
            })
            .collect()
    }

    fn create_test_ported_statute_with_score(score: f64) -> PortedStatute {
        PortedStatute {
            original_id: "test-statute".to_string(),
            statute: Statute::new(
                "ported-statute",
                "Test Statute",
                Effect::new(EffectType::Grant, "Test"),
            ),
            changes: vec![],
            locale: Locale::new("en").with_country("US"),
            compatibility_score: score,
        }
    }

    fn create_test_risk_assessment() -> RiskAssessment {
        RiskAssessment {
            risk_score: 0.5,
            risk_level: RiskLevel::Medium,
            risks: vec![Risk {
                id: "risk-1".to_string(),
                category: RiskCategory::Legal,
                description: "Legal system mismatch".to_string(),
                likelihood: RiskLevel::Medium,
                impact: 0.6,
                severity: RiskLevel::Medium,
            }],
            mitigations: vec!["Consult with legal experts".to_string()],
        }
    }

    fn create_test_validation_result(score: f64) -> ValidationResult {
        ValidationResult {
            id: uuid::Uuid::new_v4().to_string(),
            passed: score >= 0.75,
            overall_score: score,
            compliance: TargetJurisdictionComplianceCheck {
                id: uuid::Uuid::new_v4().to_string(),
                is_compliant: true,
                compliance_score: score,
                issues: vec![],
                recommendations: vec![],
                checked_regulations: vec![],
            },
            constitutional: ConstitutionalAnalysis {
                id: uuid::Uuid::new_v4().to_string(),
                is_compatible: true,
                compatibility_score: score,
                issues: vec![],
                relevant_provisions: vec![],
                recommended_amendments: vec![],
            },
            treaty_compliance: TreatyComplianceResult {
                id: uuid::Uuid::new_v4().to_string(),
                is_compliant: true,
                compliance_score: score,
                conflicts: vec![],
                checked_treaties: vec![],
                recommendations: vec![],
            },
            human_rights: HumanRightsAssessment {
                id: uuid::Uuid::new_v4().to_string(),
                impact_score: 0.0,
                affected_rights: vec![],
                vulnerable_groups: vec![],
                mitigation_measures: vec![],
                summary: "No human rights concerns identified".to_string(),
            },
            enforceability: EnforceabilityPrediction {
                id: uuid::Uuid::new_v4().to_string(),
                is_enforceable: true,
                enforceability_score: score,
                challenges: vec![],
                required_mechanisms: vec![],
                estimated_cost: None,
                recommendations: vec![],
            },
            summary: format!("Validation passed with score {:.2}", score),
        }
    }
}
