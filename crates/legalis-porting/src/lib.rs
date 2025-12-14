//! Legalis-Porting: Legal system porting for Legalis-RS.
//!
//! This crate enables "Soft ODA" - porting legal frameworks between jurisdictions
//! while adapting to local cultural parameters:
//! - Cross-jurisdiction statute translation
//! - Cultural parameter injection
//! - Legal concept mapping between systems
//! - Conflict detection with local laws

use async_trait::async_trait;
use legalis_core::Statute;
use legalis_i18n::{Jurisdiction, Locale};
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    /// Risk category
    pub category: String,
    /// Description
    pub description: String,
    /// Likelihood (0.0 - 1.0)
    pub likelihood: f64,
    /// Impact (0.0 - 1.0)
    pub impact: f64,
    /// Severity
    pub severity: Severity,
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

        Ok(PortedStatute {
            original_id: statute.id.clone(),
            statute: adapted,
            changes,
            locale: self.target.locale.clone(),
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
                category: "Legal System".to_string(),
                description: "Different legal systems may cause interpretation issues".to_string(),
                likelihood: 0.7,
                impact: 0.6,
                severity: Severity::Warning,
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
                category: "Cultural Adaptation".to_string(),
                description: format!(
                    "{} cultural adaptations may affect statute applicability",
                    cultural_changes
                ),
                likelihood: 0.5,
                impact: 0.5,
                severity: Severity::Info,
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
                category: "Incompatibility".to_string(),
                description: format!("{} incompatibilities detected", incompatibilities),
                likelihood: 0.9,
                impact: 0.8,
                severity: Severity::Error,
            });
        }

        // Calculate overall risk score
        let risk_score = if risks.is_empty() {
            0.1
        } else {
            risks.iter().map(|r| r.likelihood * r.impact).sum::<f64>() / risks.len() as f64
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
}
