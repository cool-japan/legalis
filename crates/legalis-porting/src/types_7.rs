//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use legalis_core::Statute;
use legalis_i18n::Jurisdiction;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::functions::{PortingResult, TextGenerator};
use super::types::{DriftCategory, PopulationSegment, RegulationEntry, ThreadComment};
use super::types_3::{DiscussionThread, PortingProject};
use super::types_4::{
    AlternativeMapping, CustomaryRecognition, DocumentType, Milestone, NotificationPreferences,
    ProjectStatus, ProjectTimeline,
};
use super::types_5::{
    ChangeJustification, CivilReligiousInteraction, ComplianceLevel, ElementImportance,
    MarketSector, TermTranslationMatrix,
};
use super::types_6::{
    AutoTermMapping, CompletenessCheckResult, CostTimeframe, CustomarySubject, EquityAssessment,
    HarmonizationLevel, ImpactType, PortingChange, PortingError, RegressionTestStatus,
    TrainingModule,
};
use super::types_8::{FeasibilityFactor, ReligiousLegalStatus, RiskLevel};
use super::types_9::{
    BarrierType, ComplianceSeverity, ConversionStepStatus, ExecutiveSummary, IterationChange,
    PopulationImpactType, ReviewStepStatus, ValidationComplianceIssue,
};
use super::types_10::{
    ApiStatus, CompetitivenessImpact, DemographicProjection, MarketChange, ReligiousSubject,
    ThreadStatus, TrainingAudience, WorkflowReview,
};
use super::types_11::{
    DriftIssue, ElementType, FieldDiff, MissingElement, PortedStatute,
    TargetJurisdictionComplianceCheck,
};
use super::types_12::{BenefitCategory, GeographicScope};

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
/// Type of conversion strategy.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConversionStrategyType {
    /// Direct incorporation
    DirectIncorporation,
    /// Adaptive incorporation
    AdaptiveIncorporation,
    /// Inspired legislation
    InspiredLegislation,
    /// Phased implementation
    PhasedImplementation,
    /// Pilot program first
    PilotProgram,
}
/// Population impact modeling.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopulationImpactModeling {
    /// Model ID
    pub id: String,
    /// Statute being modeled
    pub statute_id: String,
    /// Target jurisdiction
    pub jurisdiction: String,
    /// Population segments analyzed
    pub segments: Vec<PopulationSegment>,
    /// Overall impact score (0.0 - 1.0)
    pub overall_impact: f64,
    /// Equity assessment
    pub equity_assessment: EquityAssessment,
    /// Demographic projections
    pub projections: Vec<DemographicProjection>,
    /// Created at timestamp
    pub created_at: String,
}
impl PopulationImpactModeling {
    /// Creates a new population impact model.
    pub fn new(statute_id: String, jurisdiction: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            statute_id,
            jurisdiction,
            segments: Vec::new(),
            overall_impact: 0.0,
            equity_assessment: EquityAssessment {
                gini_coefficient: 0.0,
                disparate_impact: false,
                vulnerable_groups_affected: Vec::new(),
                equity_score: 1.0,
                equity_recommendations: Vec::new(),
            },
            projections: Vec::new(),
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }
    /// Adds a population segment.
    pub fn add_segment(&mut self, segment: PopulationSegment) {
        self.segments.push(segment);
        self.calculate_overall_impact();
        self.assess_equity();
    }
    /// Calculates overall impact across all segments.
    fn calculate_overall_impact(&mut self) {
        if self.segments.is_empty() {
            self.overall_impact = 0.0;
            return;
        }
        let weighted_impact: f64 = self
            .segments
            .iter()
            .map(|s| {
                let impact_value = match s.impact_type {
                    PopulationImpactType::HighlyBeneficial => s.impact_level,
                    PopulationImpactType::ModeratelyBeneficial => s.impact_level * 0.5,
                    PopulationImpactType::Neutral => 0.0,
                    PopulationImpactType::ModeratelyHarmful => -s.impact_level * 0.5,
                    PopulationImpactType::HighlyHarmful => -s.impact_level,
                };
                impact_value * (s.percentage / 100.0)
            })
            .sum();
        self.overall_impact = weighted_impact;
    }
    /// Assesses equity of statute impact.
    fn assess_equity(&mut self) {
        if self.segments.is_empty() {
            return;
        }
        let mut impacts: Vec<f64> = self.segments.iter().map(|s| s.impact_level).collect();
        impacts.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let n = impacts.len() as f64;
        let mut gini_sum = 0.0;
        for (i, impact) in impacts.iter().enumerate() {
            gini_sum += (2.0 * (i + 1) as f64 - n - 1.0) * impact;
        }
        let mean_impact = impacts.iter().sum::<f64>() / n;
        if mean_impact > 0.0 {
            self.equity_assessment.gini_coefficient = gini_sum / (n * n * mean_impact);
        }
        let max_impact = impacts.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let min_impact = impacts.iter().cloned().fold(f64::INFINITY, f64::min);
        if max_impact > 0.0 {
            self.equity_assessment.disparate_impact = (min_impact / max_impact) < 0.8;
        }
        self.equity_assessment.equity_score = 1.0 - self.equity_assessment.gini_coefficient;
    }
    /// Gets negatively impacted segments.
    pub fn negatively_impacted_segments(&self) -> Vec<&PopulationSegment> {
        self.segments
            .iter()
            .filter(|s| {
                matches!(
                    s.impact_type,
                    PopulationImpactType::ModeratelyHarmful | PopulationImpactType::HighlyHarmful
                )
            })
            .collect()
    }
}
/// Metrics for the model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetrics {
    /// Precision (0.0 - 1.0)
    pub precision: f64,
    /// Recall (0.0 - 1.0)
    pub recall: f64,
    /// F1 score (0.0 - 1.0)
    pub f1_score: f64,
    /// Accuracy (0.0 - 1.0)
    pub accuracy: f64,
    /// ROC AUC (0.0 - 1.0)
    pub roc_auc: f64,
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
/// Export format for reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    /// JSON format
    Json,
    /// Markdown format
    Markdown,
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
/// Target jurisdiction compliance checker.
#[derive(Debug, Clone)]
pub struct TargetJurisdictionChecker {
    /// Target jurisdiction
    #[allow(dead_code)]
    target_jurisdiction: Jurisdiction,
    /// Known regulations database
    regulations: HashMap<String, RegulationEntry>,
}
impl TargetJurisdictionChecker {
    /// Creates a new compliance checker.
    pub fn new(target_jurisdiction: Jurisdiction) -> Self {
        let mut regulations = HashMap::new();
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
        for (reg_id, regulation) in &self.regulations {
            checked_regulations.push(regulation.title.clone());
            if self.has_scope_overlap(statute, regulation) {
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
        true
    }
    fn meets_requirement(&self, _statute: &Statute, _requirement: &str) -> bool {
        false
    }
}
/// A benefit from porting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortingBenefit {
    /// Benefit category
    pub category: BenefitCategory,
    /// Description
    pub description: String,
    /// Monetized value (if quantifiable)
    pub monetary_value: Option<f64>,
    /// Qualitative value description
    pub qualitative_value: String,
    /// Timeframe
    pub timeframe: CostTimeframe,
    /// Certainty level (0.0 - 1.0)
    pub certainty: f64,
}
/// Parameters for statute simulation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationParameters {
    /// Population size to simulate
    pub population_size: usize,
    /// Time horizon in years
    pub time_horizon_years: u32,
    /// Number of simulation runs (for Monte Carlo)
    pub simulation_runs: usize,
    /// Confidence level (e.g., 0.95 for 95%)
    pub confidence_level: f64,
    /// Enforcement intensity (0.0 - 1.0)
    pub enforcement_intensity: f64,
    /// Compliance culture factor (0.0 - 1.0)
    pub compliance_culture: f64,
}
/// Harmonization requirement from a treaty.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HarmonizationRequirement {
    /// Requirement ID
    pub id: String,
    /// Description
    pub description: String,
    /// Required harmonization level
    pub harmonization_level: HarmonizationLevel,
    /// Affected legal areas
    pub affected_areas: Vec<String>,
    /// Deadline
    pub deadline: Option<String>,
    /// Compliance status per jurisdiction
    pub compliance_status: Vec<(String, ComplianceLevel)>,
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
/// Completeness checker for ported statutes.
pub struct CompletenessChecker {
    /// Whether to check for optional elements.
    pub check_optional: bool,
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
        self.check_required_elements(ported, &mut missing_elements);
        self.check_recommended_elements(ported, &mut missing_elements);
        if self.check_optional {
            self.check_optional_elements(ported, &mut optional_elements);
        }
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
        if ported.statute.id.is_empty() {
            missing.push(MissingElement {
                element_type: ElementType::Metadata,
                importance: ElementImportance::Required,
                description: "Statute ID is required".to_string(),
                expected_location: Some("statute.id".to_string()),
            });
        }
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
/// A religious law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReligiousLawSystem {
    /// System name
    pub name: String,
    /// Religion
    pub religion: Religion,
    /// Legal status in jurisdiction
    pub legal_status: ReligiousLegalStatus,
    /// Applicable population (percentage)
    pub population_percentage: f64,
    /// Subject matters covered
    pub subject_matters: Vec<ReligiousSubject>,
    /// Interaction with civil law
    pub civil_interaction: CivilReligiousInteraction,
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
/// Type of customary-statutory interaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InteractionType {
    /// Laws are harmonious
    Harmonious,
    /// Statutory law defers to customary
    StatutoryDefers,
    /// Customary law defers to statutory
    CustomaryDefers,
    /// Conflict requiring resolution
    Conflict,
    /// Parallel application
    Parallel,
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
/// Major religions with legal systems.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Religion {
    /// Islamic law (Sharia)
    Islam,
    /// Jewish law (Halakha)
    Judaism,
    /// Hindu law
    Hinduism,
    /// Canon law (Catholic)
    Catholicism,
    /// Buddhist law
    Buddhism,
    /// Other religious system
    Other,
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
/// Level of self-governance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GovernanceLevel {
    /// Full sovereignty
    Sovereign,
    /// Substantial autonomy
    Autonomous,
    /// Limited self-governance
    Limited,
    /// Consultation rights only
    Consultation,
    /// No self-governance
    None,
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
/// Legal status of a local practice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PracticeLegalStatus {
    /// Fully recognized in law
    Recognized,
    /// Permitted but not codified
    Permitted,
    /// Tolerated informally
    Tolerated,
    /// Legally ambiguous
    Ambiguous,
    /// Prohibited
    Prohibited,
}
/// Market impact assessment for porting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketImpactAssessment {
    /// Assessment ID
    pub id: String,
    /// Statute being assessed
    pub statute_id: String,
    /// Jurisdiction
    pub jurisdiction: String,
    /// Affected market sectors
    pub affected_sectors: Vec<MarketSector>,
    /// Competitiveness impact
    pub competitiveness_impact: CompetitivenessImpact,
    /// Market entry barriers
    pub entry_barriers: Vec<EntryBarrier>,
    /// Expected market changes
    pub market_changes: Vec<MarketChange>,
    /// Overall market impact score (-1.0 to 1.0)
    pub impact_score: f64,
}
impl MarketImpactAssessment {
    /// Creates a new market impact assessment.
    pub fn new(statute_id: String, jurisdiction: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            statute_id,
            jurisdiction,
            affected_sectors: Vec::new(),
            competitiveness_impact: CompetitivenessImpact {
                domestic_change: 0.0,
                international_change: 0.0,
                drivers: Vec::new(),
                advantages: Vec::new(),
            },
            entry_barriers: Vec::new(),
            market_changes: Vec::new(),
            impact_score: 0.0,
        }
    }
    /// Adds an affected sector.
    pub fn add_sector(&mut self, sector: MarketSector) {
        self.affected_sectors.push(sector);
        self.recalculate_impact();
    }
    /// Recalculates overall market impact score.
    fn recalculate_impact(&mut self) {
        if self.affected_sectors.is_empty() {
            self.impact_score = 0.0;
            return;
        }
        let weighted_impact: f64 = self
            .affected_sectors
            .iter()
            .map(|s| {
                let magnitude = s.impact_magnitude;
                let sign = match s.impact_type {
                    ImpactType::Positive => 1.0,
                    ImpactType::Negative => -1.0,
                    ImpactType::Neutral => 0.0,
                    ImpactType::Mixed => 0.0,
                };
                s.size_percentage * magnitude * sign
            })
            .sum();
        self.impact_score = weighted_impact.clamp(-1.0, 1.0);
    }
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
/// Implementation step for soft law conversion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionImplementationStep {
    /// Step number
    pub step_number: usize,
    /// Description
    pub description: String,
    /// Responsible party
    pub responsible_party: String,
    /// Deadline
    pub deadline: Option<String>,
    /// Status
    pub status: ConversionStepStatus,
    /// Dependencies
    pub dependencies: Vec<usize>,
}
/// A customary law rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomaryLaw {
    /// Rule name
    pub name: String,
    /// Description
    pub description: String,
    /// Subject matter
    pub subject: CustomarySubject,
    /// Age of the custom (years)
    pub age_years: usize,
    /// Geographic applicability
    pub geographic_scope: GeographicScope,
    /// Recognition status
    pub recognition: CustomaryRecognition,
    /// Binding force
    pub binding_force: f64,
    /// Consistency with modern values (0.0 - 1.0)
    pub modern_compatibility: f64,
}
/// Performance of a variant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantPerformance {
    /// Variant ID
    pub variant_id: String,
    /// Primary metric value
    pub primary_metric_value: f64,
    /// Secondary metric values
    pub secondary_metric_values: HashMap<String, f64>,
    /// Sample size
    pub sample_size: usize,
    /// Compliance rate
    pub compliance_rate: f64,
    /// User satisfaction (0.0 - 1.0)
    pub user_satisfaction: f64,
    /// Confidence interval (lower, upper)
    pub confidence_interval: (f64, f64),
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
/// Type of predicted challenge.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChallengeType {
    /// Cultural incompatibility
    CulturalIncompatibility,
    /// Legal system mismatch
    LegalSystemMismatch,
    /// Political resistance
    PoliticalResistance,
    /// Economic barriers
    EconomicBarriers,
    /// Technical implementation difficulty
    TechnicalDifficulty,
    /// Stakeholder opposition
    StakeholderOpposition,
}
/// Step status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StepStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
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
/// Market entry barrier.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryBarrier {
    /// Barrier type
    pub barrier_type: BarrierType,
    /// Description
    pub description: String,
    /// Severity (0.0 - 1.0)
    pub severity: f64,
    /// Affected parties
    pub affected_parties: Vec<String>,
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
