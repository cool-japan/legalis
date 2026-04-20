//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use legalis_core::Statute;
use legalis_i18n::{Jurisdiction, LegalSystem};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::functions::{PortingResult, TextGenerator};
use super::types::{
    AgentSpecialization, AgentState, AudienceLevel, ResourceRequirements, RiskCategory,
};
use super::types_3::{
    ActionPriority, ApprovalStepStatus, ImplementationPhase, PortingEngine, PortingOptions,
    PublicComment,
};
use super::types_4::{
    AdaptationType, ConflictPrecedentDatabase, ConsultationResponse, FeedbackAnalysis, LegalTerm,
    Severity, SyncStatus,
};
use super::types_6::{
    ConflictResolutionWorkflow, DependencyType, Gap, ImpactType, IndigenousRightCategory,
    InsightType, ModelParameters, PortingChange, PortingError, RegulatorySandbox, RiskMatrix,
    SandboxTestResult, SourceType, TemplateParameter,
};
use super::types_7::{ChangeType, CommentDocument, SandboxStatus};
use super::types_8::{
    AgentCapability, CommentPeriodStatus, ConflictReport, CulturalIssueType,
    JurisdictionDependencyResolver, NegotiatedResolutionTemplate, NormCategory,
    ResolutionWorkflowState, Risk, RiskLevel, SemanticEquivalence, SynchronizationState,
    TestScenario,
};
use super::types_9::{
    BindingForce, ComplianceSeverity, IndustryAssociation, MitigationStrategy, PublicHearing,
};
use super::types_10::{ComplianceCapacity, LearningModel, TermTranslation, TrendLegalStatus};
use super::types_11::{
    AgentPerformance, ApprovalMode, ApprovalRecord, ConceptEquivalence, EntityType, PortedStatute,
    TemplateSection,
};
use super::types_12::{ConflictType, ModelType, MultiTargetPortingResult};

/// Impact of an adaptation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptationImpact {
    /// Semantic preservation score (0.0 - 1.0)
    pub semantic_preservation: f64,
    /// Legal validity score (0.0 - 1.0)
    pub legal_validity: f64,
    /// Cultural appropriateness (0.0 - 1.0)
    pub cultural_appropriateness: f64,
    /// Implementation complexity (0.0 - 1.0)
    pub implementation_complexity: f64,
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
/// Manager for regulatory sandboxes.
#[derive(Debug, Clone)]
pub struct RegulatorySandboxManager {
    pub(super) sandboxes: HashMap<String, RegulatorySandbox>,
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
/// Learning insight derived from outcomes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningInsight {
    /// Insight ID
    pub id: String,
    /// Insight type
    pub insight_type: InsightType,
    /// Description
    pub description: String,
    /// Confidence (0.0 - 1.0)
    pub confidence: f64,
    /// Supporting evidence count
    pub evidence_count: usize,
    /// Actionable recommendation
    pub recommendation: String,
    /// Discovered at timestamp
    pub discovered_at: String,
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
        let ordered_jurisdictions = if request.resolve_dependencies {
            let deps = self
                .dependency_resolver
                .resolve_dependencies(&request.target_jurisdictions);
            dependency_log.push(format!("Resolved {} dependencies", deps.len()));
            deps
        } else {
            request.target_jurisdictions.clone()
        };
        for target_jurisdiction in ordered_jurisdictions {
            let engine = PortingEngine::new(
                request.source_jurisdiction.clone(),
                target_jurisdiction.clone(),
            );
            match engine.port_statute(&request.source_statute, &request.options) {
                Ok(ported) => {
                    jurisdiction_results.insert(target_jurisdiction.id.clone(), ported.clone());
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
/// Social norm in a jurisdiction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialNorm {
    /// Norm description
    pub description: String,
    /// Norm category
    pub category: NormCategory,
    /// Strength (0.0 - 1.0)
    pub strength: f64,
    /// Legal recognition
    pub legally_recognized: bool,
}
/// Contemporary cultural trend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CulturalTrend {
    /// Trend description
    pub description: String,
    /// Direction (positive = increasing, negative = decreasing)
    pub direction: f64,
    /// Velocity of change (0.0 - 1.0)
    pub velocity: f64,
    /// Legal adaptation status
    pub legal_status: TrendLegalStatus,
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
/// Recommended action in response to alert.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendedAction {
    /// Action ID
    pub id: String,
    /// Action description
    pub action: String,
    /// Action priority
    pub priority: ActionPriority,
    /// Estimated effort
    pub estimated_effort: String,
    /// Deadline
    pub deadline: Option<String>,
    /// Prerequisites
    pub prerequisites: Vec<String>,
}
/// Type of soft law.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SoftLawType {
    /// UN resolution
    UNResolution,
    /// Guidelines
    Guidelines,
    /// Recommendations
    Recommendations,
    /// Principles
    Principles,
    /// Codes of conduct
    CodeOfConduct,
    /// Declarations
    Declaration,
    /// Best practices
    BestPractices,
    /// Standards
    Standards,
}
/// Interaction between civil and religious law.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CivilReligiousInteraction {
    /// Religious law takes precedence
    ReligiousPrecedence,
    /// Civil law takes precedence
    CivilPrecedence,
    /// Equal authority in respective domains
    DualSystem,
    /// Individual choice
    OptIn,
    /// Complete separation
    Separated,
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
/// Conflict with religious law.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReligiousConflict {
    /// Conflict description
    pub description: String,
    /// Severity (0.0 - 1.0)
    pub severity: f64,
    /// Affected population percentage
    pub affected_population: f64,
    /// Possible resolution
    pub resolution_option: String,
}
/// Recommended timing for porting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendedTiming {
    /// Optimal start date
    pub optimal_start: String,
    /// Latest recommended start
    pub latest_start: String,
    /// Expected duration
    pub expected_duration: String,
    /// Timing rationale
    pub rationale: String,
    /// Window of opportunity factors
    pub opportunity_factors: Vec<String>,
}
/// Assessment of practice transferability.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferabilityAssessment {
    /// Overall transferability score (0.0 - 1.0)
    pub overall_score: f64,
    /// Legal system compatibility
    pub legal_system_compatibility: Vec<(String, f64)>,
    /// Cultural adaptability
    pub cultural_adaptability: f64,
    /// Economic feasibility
    pub economic_feasibility: f64,
    /// Prerequisites for adoption
    pub prerequisites: Vec<String>,
    /// Potential barriers
    pub potential_barriers: Vec<String>,
}
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
/// Resource allocation for enforcement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceAllocation {
    /// Personnel count
    pub personnel: usize,
    /// Budget
    pub budget: f64,
    /// Currency
    pub currency: String,
    /// Equipment
    pub equipment: Vec<String>,
    /// Training requirements
    pub training_hours: f64,
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
            state
                .pending_changes
                .entry(jurisdiction.to_string())
                .or_default()
                .extend(changes);
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
/// A proposed adaptation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposedAdaptation {
    /// Adaptation type
    pub adaptation_type: AdaptationType,
    /// Original text/value
    pub original: String,
    /// Proposed text/value
    pub proposed: String,
    /// Justification
    pub justification: String,
    /// Confidence in this adaptation (0.0 - 1.0)
    pub confidence: f64,
    /// Impact assessment
    pub impact: AdaptationImpact,
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
        severity_score += match conflict.conflict_type {
            ConflictType::Contradiction => 3,
            ConflictType::CulturalIncompatibility => 2,
            ConflictType::SystemMismatch => 2,
            ConflictType::Overlap => 1,
            ConflictType::Procedural => 1,
        };
        if source_jurisdiction.legal_system != target_jurisdiction.legal_system {
            severity_score += 1;
        }
        let precedents = self.precedent_db.find_relevant_precedents(
            &source_jurisdiction.id,
            &target_jurisdiction.id,
            &conflict.conflict_type,
        );
        if precedents.is_empty() {
            severity_score += 1;
        } else {
            let avg_effectiveness: f64 =
                precedents.iter().map(|p| p.effectiveness).sum::<f64>() / precedents.len() as f64;
            if avg_effectiveness < 0.5 {
                severity_score += 1;
            }
        }
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
/// Compliance level with treaty requirements.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ComplianceLevel {
    /// Full compliance
    FullCompliance,
    /// Partial compliance
    PartialCompliance,
    /// Non-compliance
    NonCompliance,
    /// Assessment pending
    Pending,
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
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }
}
/// Legal term translation matrix.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TermTranslationMatrix {
    /// Translations indexed by source jurisdiction->target jurisdiction
    translations: HashMap<String, Vec<TermTranslation>>,
    /// Terms indexed by jurisdiction
    pub(super) terms: HashMap<String, Vec<LegalTerm>>,
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
        if let Some(ctx) = context
            && let Some(trans) = translations.iter().find(|t| {
                t.valid_contexts.is_empty() || t.valid_contexts.iter().any(|c| c.contains(ctx))
            })
        {
            return Some(trans);
        }
        translations.into_iter().max_by(|a, b| {
            a.accuracy
                .partial_cmp(&b.accuracy)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
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
/// Type of regulatory change.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RegulatoryChangeType {
    /// New legislation enacted
    NewLegislation,
    /// Amendment to existing law
    Amendment,
    /// Repeal of law
    Repeal,
    /// New regulation issued
    NewRegulation,
    /// Court decision with precedential value
    CourtDecision,
    /// Administrative guidance
    AdministrativeGuidance,
    /// Emergency order
    EmergencyOrder,
    /// Sunset provision activation
    SunsetProvision,
}
/// Tracker status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TrackerStatus {
    /// Active and monitoring
    Active,
    /// Paused
    Paused,
    /// Error state
    Error,
    /// Maintenance mode
    Maintenance,
}
/// Market sector affected by porting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketSector {
    /// Sector name
    pub name: String,
    /// Sector size (GDP percentage)
    pub size_percentage: f64,
    /// Number of businesses affected
    pub businesses_affected: usize,
    /// Impact type
    pub impact_type: ImpactType,
    /// Impact magnitude (0.0 - 1.0)
    pub impact_magnitude: f64,
}
/// Industry consultation integration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndustryConsultation {
    /// Consultation ID
    pub id: String,
    /// Statute ID
    pub statute_id: String,
    /// Jurisdiction
    pub jurisdiction: String,
    /// Industry associations consulted
    pub associations: Vec<IndustryAssociation>,
    /// Consultation responses
    pub responses: Vec<ConsultationResponse>,
    /// Public hearing IDs
    pub hearing_ids: Vec<String>,
    /// Feedback analysis
    pub feedback_analysis: FeedbackAnalysis,
}
impl IndustryConsultation {
    /// Creates a new industry consultation.
    pub fn new(statute_id: String, jurisdiction: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            statute_id,
            jurisdiction,
            associations: Vec::new(),
            responses: Vec::new(),
            hearing_ids: Vec::new(),
            feedback_analysis: FeedbackAnalysis {
                response_count: 0,
                average_support: 0.0,
                common_concerns: Vec::new(),
                consensus_recommendations: Vec::new(),
                divided_issues: Vec::new(),
            },
        }
    }
    /// Adds an industry association.
    pub fn add_association(&mut self, association: IndustryAssociation) {
        self.associations.push(association);
    }
    /// Adds a consultation response.
    pub fn add_response(&mut self, response: ConsultationResponse) {
        self.responses.push(response);
        self.analyze_feedback();
    }
    /// Analyzes all feedback received.
    fn analyze_feedback(&mut self) {
        self.feedback_analysis.response_count = self.responses.len();
        if !self.responses.is_empty() {
            self.feedback_analysis.average_support =
                self.responses.iter().map(|r| r.support_level).sum::<f64>()
                    / self.responses.len() as f64;
            let mut concern_map: HashMap<String, usize> = HashMap::new();
            for response in &self.responses {
                for concern in &response.concerns {
                    *concern_map.entry(concern.clone()).or_insert(0) += 1;
                }
            }
            self.feedback_analysis.common_concerns = concern_map
                .into_iter()
                .filter(|(_, count)| *count >= 2)
                .map(|(concern, _)| concern)
                .collect();
        }
    }
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
                let similarity = 0.75;
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
                let similarity = self.calculate_basic_similarity(source_concept, target_concept);
                let explain = "Rule-based similarity analysis".to_string();
                let sims = vec!["Lexical similarity detected".to_string()];
                let diffs = vec!["Different legal systems may affect interpretation".to_string()];
                (similarity, explain, sims, diffs)
            };
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
        let equivalence_score =
            (similarity_score * 0.4) + (structural_score * 0.3) + (functional_score * 0.3);
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
        let distance = self.levenshtein_distance(s1, s2);
        let max_len = s1.len().max(s2.len()) as f64;
        if max_len == 0.0 {
            1.0
        } else {
            1.0 - (distance as f64 / max_len)
        }
    }
    /// Calculates Levenshtein distance.
    #[allow(clippy::needless_range_loop)]
    fn levenshtein_distance(&self, s1: &str, s2: &str) -> usize {
        let len1 = s1.len();
        let len2 = s2.len();
        let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];
        for i in 0..=len1 {
            matrix[i][0] = i;
        }
        for (j, cell) in matrix[0].iter_mut().enumerate().take(len2 + 1) {
            *cell = j;
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
/// Implementation status of adopted model law.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ImplementationStatus {
    /// Fully implemented
    Implemented,
    /// Partially implemented
    PartiallyImplemented,
    /// Enacted but not yet implemented
    Enacted,
    /// In legislative process
    InLegislativeProcess,
    /// Planned
    Planned,
}
/// Data source for emerging law analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSource {
    /// Source type
    pub source_type: SourceType,
    /// Source identifier
    pub source_id: String,
    /// Source description
    pub description: String,
    /// Reliability score (0.0 - 1.0)
    pub reliability: f64,
    /// Last accessed
    pub last_accessed: String,
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
/// An indigenous right.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndigenousRight {
    /// Right description
    pub description: String,
    /// Right category
    pub category: IndigenousRightCategory,
    /// Legal basis
    pub legal_basis: Vec<String>,
    /// Geographic scope
    pub geographic_scope: Option<Vec<String>>,
    /// Limitation/qualifications
    pub limitations: Vec<String>,
}
/// Entity affected by compliance requirements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AffectedEntity {
    /// Entity type
    pub entity_type: EntityType,
    /// Number of entities
    pub count: usize,
    /// Average compliance cost per entity
    pub average_cost: f64,
    /// Capacity to comply
    pub capacity: ComplianceCapacity,
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
/// Soft law source document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoftLawSource {
    /// Source ID
    pub id: String,
    /// Source name
    pub name: String,
    /// Source type
    pub source_type: SoftLawType,
    /// Issuing body
    pub issuing_body: String,
    /// Content
    pub content: String,
    /// Binding force (if any)
    pub binding_force: BindingForce,
    /// Adoption/endorsement status
    pub endorsements: Vec<String>,
}
/// Configuration for human-in-the-loop system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HitlConfiguration {
    /// Confidence threshold below which human review is required
    pub confidence_threshold: f64,
    /// Whether to require review for high-stakes decisions
    pub require_review_for_high_stakes: bool,
    /// Maximum time for review (seconds)
    pub max_review_time: f64,
    /// Escalation threshold (number of rejections before escalation)
    pub escalation_threshold: usize,
}
/// Effort level for resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EffortLevel {
    Low,
    Medium,
    High,
    VeryHigh,
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
/// AI agent for autonomous porting analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortingAgent {
    /// Agent ID
    pub id: String,
    /// Agent name
    pub name: String,
    /// Agent specialization
    pub specialization: AgentSpecialization,
    /// Learning model
    pub model: LearningModel,
    /// Agent performance metrics
    pub performance: AgentPerformance,
    /// Agent capabilities
    pub capabilities: Vec<AgentCapability>,
    /// Agent state
    pub state: AgentState,
    /// Created at timestamp
    pub created_at: String,
}
impl PortingAgent {
    /// Creates a new porting agent.
    pub fn new(name: String, specialization: AgentSpecialization) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            specialization,
            model: LearningModel {
                version: "1.0.0".to_string(),
                model_type: ModelType::Supervised,
                training_data_size: 0,
                accuracy: 0.5,
                last_trained: chrono::Utc::now().to_rfc3339(),
                parameters: ModelParameters {
                    learning_rate: 0.001,
                    batch_size: 32,
                    layers: 3,
                    hidden_units: 128,
                    dropout_rate: 0.2,
                },
            },
            performance: AgentPerformance {
                total_analyses: 0,
                successful_analyses: 0,
                average_accuracy: 0.0,
                average_time_seconds: 0.0,
                user_satisfaction: 0.0,
                improvement_rate: 0.0,
            },
            capabilities: Vec::new(),
            state: AgentState::Idle,
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }
    /// Adds a capability to the agent.
    pub fn add_capability(&mut self, capability: AgentCapability) {
        self.capabilities.push(capability);
    }
    /// Updates agent state.
    pub fn set_state(&mut self, state: AgentState) {
        self.state = state;
    }
    /// Records an analysis result for performance tracking.
    pub fn record_analysis(&mut self, success: bool, accuracy: f64, time_seconds: f64) {
        self.performance.total_analyses += 1;
        if success {
            self.performance.successful_analyses += 1;
        }
        let n = self.performance.total_analyses as f64;
        self.performance.average_accuracy =
            (self.performance.average_accuracy * (n - 1.0) + accuracy) / n;
        self.performance.average_time_seconds =
            (self.performance.average_time_seconds * (n - 1.0) + time_seconds) / n;
    }
    /// Gets the success rate of the agent.
    pub fn success_rate(&self) -> f64 {
        if self.performance.total_analyses == 0 {
            0.0
        } else {
            self.performance.successful_analyses as f64 / self.performance.total_analyses as f64
        }
    }
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
/// Reason for requiring human review.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReviewReason {
    /// Low confidence in agent proposal
    LowConfidence,
    /// High-stakes decision
    HighStakes,
    /// Novel situation not in training data
    NovelSituation,
    /// Conflicting recommendations
    ConflictingRecommendations,
    /// Legal complexity
    LegalComplexity,
    /// User requested review
    UserRequested,
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
