//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use legalis_core::Statute;
use std::collections::{HashMap, HashSet};

use super::functions::{
    check_accessibility, check_due_process, check_equal_protection, check_equality,
    check_freedom_of_expression, check_privacy_impact, check_procedural_due_process,
    check_property_rights, check_proportionality, check_retroactivity,
};
use super::functions_4::execute_scheduled_report;
use super::types::{
    MarkovTransition, ProofStep, ScheduledReportResult, StatuteEvolution, Strategy,
    VerificationProof, VerificationSummary,
};
use super::types_3::{
    AmbiguityType, DependencyNode, EvolutionMetrics, GapType, JurisdictionalRuleSet,
    PrincipleCheck, PrincipleCheckResult, PrivacyBudget, ReportOutputFormat,
};
use super::types_5::{
    ComplexityLevel, GameOutcome, MarkovState, PatternType, ReportTemplate, StatuteChange,
    VerificationError, VerificationResult,
};

/// Schedule configuration for automated report generation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReportSchedule {
    /// Schedule identifier
    pub id: String,
    /// Human-readable schedule name
    pub name: String,
    /// Report template to use
    pub template: ReportTemplate,
    /// Cron expression for scheduling (e.g., "0 0 * * *" for daily at midnight)
    pub cron_expression: String,
    /// Output directory for generated reports
    pub output_directory: String,
    /// Output format (markdown, html, json, pdf)
    pub output_format: ReportOutputFormat,
    /// Whether the schedule is active
    pub enabled: bool,
    /// Optional recipient email addresses
    pub recipients: Vec<String>,
    /// Last execution timestamp (RFC 3339)
    pub last_execution: Option<String>,
    /// Next scheduled execution timestamp (RFC 3339)
    pub next_execution: Option<String>,
}
impl ReportSchedule {
    /// Creates a new report schedule
    pub fn new(id: impl Into<String>, name: impl Into<String>, template: ReportTemplate) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            template,
            cron_expression: "0 0 * * *".to_string(),
            output_directory: "./reports".to_string(),
            output_format: ReportOutputFormat::Markdown,
            enabled: true,
            recipients: Vec::new(),
            last_execution: None,
            next_execution: None,
        }
    }
    /// Sets the cron expression for scheduling
    pub fn with_cron(mut self, cron_expression: impl Into<String>) -> Self {
        self.cron_expression = cron_expression.into();
        self
    }
    /// Sets the output directory
    pub fn with_output_directory(mut self, directory: impl Into<String>) -> Self {
        self.output_directory = directory.into();
        self
    }
    /// Sets the output format
    pub fn with_format(mut self, format: ReportOutputFormat) -> Self {
        self.output_format = format;
        self
    }
    /// Adds a recipient email address
    pub fn with_recipient(mut self, email: impl Into<String>) -> Self {
        self.recipients.push(email.into());
        self
    }
    /// Enables or disables the schedule
    pub fn set_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}
/// Differentially private aggregation result
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PrivateAggregation {
    /// Number of statutes analyzed (noised)
    pub count: f64,
    /// Average complexity (noised)
    pub avg_complexity: f64,
    /// Error rate (noised)
    pub error_rate: f64,
    /// Privacy budget used
    pub privacy_budget: PrivacyBudget,
}
impl PrivateAggregation {
    /// Generates a report
    pub fn report(&self) -> String {
        format!(
            "Differential Privacy Report\n\
             ==========================\n\
             Count: {:.2}\n\
             Average Complexity: {:.2}\n\
             Error Rate: {:.2}%\n\
             Privacy Budget: ε={:.3}, δ={:.6}\n",
            self.count,
            self.avg_complexity,
            self.error_rate * 100.0,
            self.privacy_budget.epsilon,
            self.privacy_budget.delta
        )
    }
}
/// Regulatory impact assessment for a statute.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RegulatoryImpact {
    /// Statute ID
    pub statute_id: String,
    /// Overall impact score (0-100, higher = more regulatory burden)
    pub impact_score: u32,
    /// Compliance complexity score (0-100)
    pub compliance_complexity: u32,
    /// Affected entities estimate
    pub affected_entities: String,
    /// Implementation cost estimate
    pub implementation_cost: String,
    /// Ongoing compliance cost estimate
    pub ongoing_cost: String,
    /// Impact level
    pub impact_level: String,
}
/// Represents a detected ambiguity in a statute.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Ambiguity {
    /// Type of ambiguity
    pub ambiguity_type: AmbiguityType,
    /// Location in the statute (field name)
    pub location: String,
    /// Description of the ambiguity
    pub description: String,
    /// Suggested clarification
    pub suggestion: String,
    /// Severity (1-10, higher is more severe)
    pub severity: u8,
}
impl Ambiguity {
    /// Creates a new ambiguity instance.
    pub fn new(
        ambiguity_type: AmbiguityType,
        location: impl Into<String>,
        description: impl Into<String>,
        suggestion: impl Into<String>,
        severity: u8,
    ) -> Self {
        Self {
            ambiguity_type,
            location: location.into(),
            description: description.into(),
            suggestion: suggestion.into(),
            severity: severity.min(10),
        }
    }
}
/// Cluster/community in the statute graph
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StatuteCluster {
    /// Cluster ID
    pub id: usize,
    /// Statute IDs in this cluster
    pub statute_ids: Vec<String>,
    /// Internal density of the cluster
    pub density: f64,
    /// Representative keywords/topics
    pub keywords: Vec<String>,
}
/// Types of redundancy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum RedundancyType {
    /// Duplicate statutes
    Duplicate,
    /// Subsumed (one statute is completely covered by another)
    Subsumed,
    /// Overlapping conditions
    OverlappingConditions,
    /// Equivalent effects
    EquivalentEffects,
}
/// Quality metrics summary
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QualitySummary {
    /// Average quality score
    pub average_score: f64,
    /// Grade distribution
    pub grade_distribution: HashMap<String, usize>,
    /// Number of statutes with issues
    pub statutes_with_issues: usize,
    /// Total issues found
    pub total_issues: usize,
}
/// Interactive proof explorer data structure
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InteractiveProof {
    /// The proof
    pub proof: VerificationProof,
    /// Current step being viewed
    pub current_step: usize,
    /// Whether to show all dependencies
    pub show_dependencies: bool,
    /// Navigation history
    pub history: Vec<usize>,
}
impl InteractiveProof {
    /// Creates a new interactive proof explorer
    pub fn new(proof: VerificationProof) -> Self {
        Self {
            proof,
            current_step: 0,
            show_dependencies: true,
            history: vec![0],
        }
    }
    /// Navigates to a specific step
    pub fn goto_step(&mut self, step_number: usize) -> Result<&ProofStep, String> {
        if step_number >= self.proof.steps.len() {
            return Err(format!("Step {} does not exist", step_number));
        }
        self.current_step = step_number;
        self.history.push(step_number);
        Ok(&self.proof.steps[step_number])
    }
    /// Goes to the next step
    pub fn next_step(&mut self) -> Option<&ProofStep> {
        if self.current_step + 1 < self.proof.steps.len() {
            self.current_step += 1;
            self.history.push(self.current_step);
            Some(&self.proof.steps[self.current_step])
        } else {
            None
        }
    }
    /// Goes to the previous step
    pub fn previous_step(&mut self) -> Option<&ProofStep> {
        if self.current_step > 0 {
            self.current_step -= 1;
            self.history.push(self.current_step);
            Some(&self.proof.steps[self.current_step])
        } else {
            None
        }
    }
    /// Gets the current step
    pub fn current(&self) -> Option<&ProofStep> {
        self.proof.steps.get(self.current_step)
    }
    /// Exports to JSON for web interface
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}
/// Difference between two verification results
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VerificationDiff {
    /// Errors added in new result
    pub errors_added: Vec<VerificationError>,
    /// Errors removed in new result
    pub errors_removed: Vec<VerificationError>,
    /// Warnings added
    pub warnings_added: Vec<String>,
    /// Warnings removed
    pub warnings_removed: Vec<String>,
    /// Overall status change
    pub status_changed: bool,
    /// Old status
    pub old_passed: bool,
    /// New status
    pub new_passed: bool,
}
impl VerificationDiff {
    /// Creates a diff between two verification results
    pub fn diff(old: &VerificationResult, new: &VerificationResult) -> Self {
        let mut errors_added = Vec::new();
        let mut errors_removed = Vec::new();
        for error in &new.errors {
            if !Self::contains_error(&old.errors, error) {
                errors_added.push(error.clone());
            }
        }
        for error in &old.errors {
            if !Self::contains_error(&new.errors, error) {
                errors_removed.push(error.clone());
            }
        }
        let mut warnings_added = Vec::new();
        let mut warnings_removed = Vec::new();
        for warning in &new.warnings {
            if !old.warnings.contains(warning) {
                warnings_added.push(warning.clone());
            }
        }
        for warning in &old.warnings {
            if !new.warnings.contains(warning) {
                warnings_removed.push(warning.clone());
            }
        }
        Self {
            errors_added,
            errors_removed,
            warnings_added,
            warnings_removed,
            status_changed: old.passed != new.passed,
            old_passed: old.passed,
            new_passed: new.passed,
        }
    }
    fn contains_error(errors: &[VerificationError], target: &VerificationError) -> bool {
        errors.iter().any(|e| Self::errors_equal(e, target))
    }
    fn errors_equal(a: &VerificationError, b: &VerificationError) -> bool {
        format!("{:?}", a) == format!("{:?}", b)
    }
    /// Checks if there are any changes
    pub fn has_changes(&self) -> bool {
        !self.errors_added.is_empty()
            || !self.errors_removed.is_empty()
            || !self.warnings_added.is_empty()
            || !self.warnings_removed.is_empty()
            || self.status_changed
    }
    /// Generates a report of the diff
    pub fn report(&self) -> String {
        let mut output = String::new();
        output.push_str("# Verification Diff Report\n\n");
        if self.status_changed {
            output.push_str(&format!(
                "## Status Changed: {} → {}\n\n",
                if self.old_passed { "PASS" } else { "FAIL" },
                if self.new_passed { "PASS" } else { "FAIL" }
            ));
        }
        if !self.errors_added.is_empty() {
            output.push_str("## Errors Added:\n");
            for error in &self.errors_added {
                output.push_str(&format!("- {:?}\n", error));
            }
            output.push('\n');
        }
        if !self.errors_removed.is_empty() {
            output.push_str("## Errors Removed:\n");
            for error in &self.errors_removed {
                output.push_str(&format!("- {:?}\n", error));
            }
            output.push('\n');
        }
        if !self.warnings_added.is_empty() {
            output.push_str("## Warnings Added:\n");
            for warning in &self.warnings_added {
                output.push_str(&format!("- {}\n", warning));
            }
            output.push('\n');
        }
        if !self.warnings_removed.is_empty() {
            output.push_str("## Warnings Removed:\n");
            for warning in &self.warnings_removed {
                output.push_str(&format!("- {}\n", warning));
            }
            output.push('\n');
        }
        if !self.has_changes() {
            output.push_str("No changes detected.\n");
        }
        output
    }
}
/// How to combine principle results.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CombinationMode {
    /// All component principles must pass
    All,
    /// At least one component principle must pass
    Any,
    /// Majority of component principles must pass
    Majority,
}
/// A principle definition in the DSL.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PrincipleDefinition {
    /// Unique identifier
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Description
    pub description: String,
    /// Priority (higher = more important)
    pub priority: u32,
    /// Jurisdiction where this principle applies
    pub jurisdiction: Option<String>,
    /// Conditions that must be checked
    pub checks: Vec<PrincipleCheck>,
}
impl PrincipleDefinition {
    /// Creates a new principle definition.
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: description.into(),
            priority: 0,
            jurisdiction: None,
            checks: Vec::new(),
        }
    }
    /// Sets the priority.
    pub fn with_priority(mut self, priority: u32) -> Self {
        self.priority = priority;
        self
    }
    /// Sets the jurisdiction.
    pub fn with_jurisdiction(mut self, jurisdiction: impl Into<String>) -> Self {
        self.jurisdiction = Some(jurisdiction.into());
        self
    }
    /// Adds a check.
    pub fn with_check(mut self, check: PrincipleCheck) -> Self {
        self.checks.push(check);
        self
    }
}
/// Deadline constraint for temporal verification.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Deadline {
    /// Identifier for this deadline
    pub id: String,
    /// Event that must occur
    pub event: String,
    /// Maximum time steps allowed
    pub max_steps: usize,
    /// Description of the deadline
    pub description: String,
}
impl Deadline {
    /// Creates a new deadline.
    pub fn new(id: impl Into<String>, event: impl Into<String>, max_steps: usize) -> Self {
        Self {
            id: id.into(),
            event: event.into(),
            max_steps,
            description: String::new(),
        }
    }
    /// Adds a description to the deadline.
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }
}
/// Impact of a statute change on the system.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChangeImpact {
    /// ID of the changed statute
    pub statute_id: String,
    /// List of changes detected
    pub changes: Vec<StatuteChange>,
    /// Statutes that reference this statute (potentially affected)
    pub affected_statutes: Vec<String>,
    /// Estimated impact severity
    pub impact_severity: Severity,
    /// Recommendations for handling the change
    pub recommendations: Vec<String>,
}
/// Impact level classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ImpactLevel {
    Low,
    Medium,
    High,
}
/// Principle registry managing multiple jurisdictions.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct PrincipleRegistry {
    /// Rule sets by jurisdiction
    pub jurisdictions: HashMap<String, JurisdictionalRuleSet>,
}
impl PrincipleRegistry {
    /// Creates a new empty registry.
    pub fn new() -> Self {
        Self {
            jurisdictions: HashMap::new(),
        }
    }
    /// Adds a jurisdictional rule set.
    pub fn add_jurisdiction(&mut self, rule_set: JurisdictionalRuleSet) {
        self.jurisdictions
            .insert(rule_set.jurisdiction.clone(), rule_set);
    }
    /// Gets a rule set for a jurisdiction.
    pub fn get_jurisdiction(&self, jurisdiction: &str) -> Option<&JurisdictionalRuleSet> {
        self.jurisdictions.get(jurisdiction)
    }
    /// Verifies a statute against a specific jurisdiction's rules.
    pub fn verify_for_jurisdiction(
        &self,
        statute: &Statute,
        jurisdiction: &str,
    ) -> VerificationResult {
        let mut result = VerificationResult::pass();
        if let Some(rule_set) = self.get_jurisdiction(jurisdiction) {
            for principle_def in rule_set.principles_by_priority() {
                for check in &principle_def.checks {
                    let check_result = match check {
                        PrincipleCheck::NoDiscrimination => check_equality(statute),
                        PrincipleCheck::RequiresProcedure => check_due_process(statute),
                        PrincipleCheck::NoRetroactivity => check_retroactivity(statute),
                        PrincipleCheck::EqualityCheck => check_equality(statute),
                        PrincipleCheck::DueProcess => check_due_process(statute),
                        PrincipleCheck::PrivacyImpact => check_privacy_impact(statute),
                        PrincipleCheck::Proportionality => check_proportionality(statute),
                        PrincipleCheck::Accessibility => check_accessibility(statute),
                        PrincipleCheck::FreedomOfExpression => check_freedom_of_expression(statute),
                        PrincipleCheck::PropertyRights => check_property_rights(statute),
                        PrincipleCheck::ProceduralDueProcess => {
                            check_procedural_due_process(statute)
                        }
                        PrincipleCheck::EqualProtection => check_equal_protection(statute),
                        PrincipleCheck::Custom { .. } => PrincipleCheckResult::pass(),
                    };
                    if !check_result.passed {
                        result.merge(VerificationResult::fail(vec![
                            VerificationError::ConstitutionalConflict {
                                statute_id: statute.id.clone(),
                                principle: principle_def.name.clone(),
                            },
                        ]));
                    }
                }
            }
            for composite in &rule_set.composites {
                let component_results: Vec<bool> = composite
                    .components
                    .iter()
                    .filter_map(|comp_id| rule_set.principles.iter().find(|p| &p.id == comp_id))
                    .map(|principle_def| {
                        principle_def.checks.iter().all(|check| match check {
                            PrincipleCheck::NoDiscrimination => check_equality(statute).passed,
                            PrincipleCheck::RequiresProcedure => check_due_process(statute).passed,
                            PrincipleCheck::NoRetroactivity => check_retroactivity(statute).passed,
                            PrincipleCheck::EqualityCheck => check_equality(statute).passed,
                            PrincipleCheck::DueProcess => check_due_process(statute).passed,
                            PrincipleCheck::PrivacyImpact => check_privacy_impact(statute).passed,
                            PrincipleCheck::Proportionality => {
                                check_proportionality(statute).passed
                            }
                            PrincipleCheck::Accessibility => check_accessibility(statute).passed,
                            PrincipleCheck::FreedomOfExpression => {
                                check_freedom_of_expression(statute).passed
                            }
                            PrincipleCheck::PropertyRights => check_property_rights(statute).passed,
                            PrincipleCheck::ProceduralDueProcess => {
                                check_procedural_due_process(statute).passed
                            }
                            PrincipleCheck::EqualProtection => {
                                check_equal_protection(statute).passed
                            }
                            PrincipleCheck::Custom { .. } => true,
                        })
                    })
                    .collect();
                let composite_passed = match composite.combination_mode {
                    CombinationMode::All => component_results.iter().all(|&x| x),
                    CombinationMode::Any => component_results.iter().any(|&x| x),
                    CombinationMode::Majority => {
                        let passed_count = component_results.iter().filter(|&&x| x).count();
                        passed_count * 2 > component_results.len()
                    }
                };
                if !composite_passed {
                    result.merge(VerificationResult::fail(vec![
                        VerificationError::ConstitutionalConflict {
                            statute_id: statute.id.clone(),
                            principle: composite.name.clone(),
                        },
                    ]));
                }
            }
        }
        result
    }
}
/// Represents a game-theoretic model
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GameTheoreticModel {
    /// Stakeholders (players)
    pub stakeholders: Vec<String>,
    /// Available strategies for each stakeholder
    pub strategies: Vec<Vec<Strategy>>,
    /// All possible outcomes
    pub outcomes: Vec<GameOutcome>,
}
impl GameTheoreticModel {
    /// Creates a new game-theoretic model
    pub fn new(stakeholders: Vec<String>) -> Self {
        let strategies = vec![Vec::new(); stakeholders.len()];
        Self {
            stakeholders,
            strategies,
            outcomes: Vec::new(),
        }
    }
    /// Adds a strategy for a stakeholder
    pub fn add_strategy(&mut self, stakeholder_idx: usize, strategy: Strategy) {
        if stakeholder_idx < self.strategies.len() {
            self.strategies[stakeholder_idx].push(strategy);
        }
    }
    /// Adds an outcome
    pub fn add_outcome(&mut self, outcome: GameOutcome) {
        self.outcomes.push(outcome);
    }
}
/// Fine-grained dependency graph
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DependencyGraph {
    /// All dependency nodes
    pub nodes: HashMap<String, DependencyNode>,
}
impl DependencyGraph {
    /// Creates a new empty dependency graph
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }
    /// Builds a dependency graph from statutes
    pub fn from_statutes(statutes: &[Statute]) -> Self {
        let mut graph = Self::new();
        for statute in statutes {
            let mut node = DependencyNode::new(&statute.id, DependencyType::DerivesFrom);
            for dep in &statute.derives_from {
                node.add_dependency(dep);
            }
            graph.nodes.insert(statute.id.clone(), node);
        }
        let statute_ids: Vec<String> = graph.nodes.keys().cloned().collect();
        for id in statute_ids {
            let deps: Vec<String> = graph.nodes[&id].dependencies.clone();
            for dep in deps {
                if let Some(dep_node) = graph.nodes.get_mut(&dep) {
                    dep_node.add_dependent(&id);
                }
            }
        }
        graph
    }
    /// Gets all transitive dependencies for a statute
    pub fn get_transitive_dependencies(&self, statute_id: &str) -> Vec<String> {
        let mut visited = HashSet::new();
        let mut result = Vec::new();
        self.collect_dependencies(statute_id, &mut visited, &mut result);
        result
    }
    fn collect_dependencies(
        &self,
        statute_id: &str,
        visited: &mut HashSet<String>,
        result: &mut Vec<String>,
    ) {
        if visited.contains(statute_id) {
            return;
        }
        visited.insert(statute_id.to_string());
        if let Some(node) = self.nodes.get(statute_id) {
            for dep in &node.dependencies {
                result.push(dep.clone());
                self.collect_dependencies(dep, visited, result);
            }
        }
    }
    /// Gets all statutes affected by a change to the given statute
    pub fn get_affected_statutes(&self, statute_id: &str) -> Vec<String> {
        let mut visited = HashSet::new();
        let mut result = Vec::new();
        self.collect_dependents(statute_id, &mut visited, &mut result);
        result
    }
    fn collect_dependents(
        &self,
        statute_id: &str,
        visited: &mut HashSet<String>,
        result: &mut Vec<String>,
    ) {
        if visited.contains(statute_id) {
            return;
        }
        visited.insert(statute_id.to_string());
        if let Some(node) = self.nodes.get(statute_id) {
            for dep in &node.dependents {
                result.push(dep.clone());
                self.collect_dependents(dep, visited, result);
            }
        }
    }
}
/// A transition system for temporal logic verification.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TransitionSystem {
    /// All states in the system
    pub states: HashMap<String, TemporalState>,
    /// Transitions between states (from -> to list)
    pub transitions: HashMap<String, Vec<String>>,
    /// Initial states
    pub initial_states: HashSet<String>,
}
impl TransitionSystem {
    /// Creates a new empty transition system.
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
            transitions: HashMap::new(),
            initial_states: HashSet::new(),
        }
    }
    /// Adds a state to the system.
    pub fn add_state(&mut self, state: TemporalState) {
        self.states.insert(state.id.clone(), state);
    }
    /// Adds a transition from one state to another.
    pub fn add_transition(&mut self, from: impl Into<String>, to: impl Into<String>) {
        self.transitions
            .entry(from.into())
            .or_default()
            .push(to.into());
    }
    /// Marks a state as initial.
    pub fn add_initial_state(&mut self, state_id: impl Into<String>) {
        self.initial_states.insert(state_id.into());
    }
    /// Gets the successors of a state.
    pub fn successors(&self, state_id: &str) -> Vec<&TemporalState> {
        self.transitions
            .get(state_id)
            .map(|ids| ids.iter().filter_map(|id| self.states.get(id)).collect())
            .unwrap_or_default()
    }
}
/// Common pattern found in statutes
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StatutePattern {
    /// Pattern ID
    pub id: String,
    /// Pattern description
    pub description: String,
    /// Frequency (number of statutes matching this pattern)
    pub frequency: usize,
    /// Example statute IDs
    pub examples: Vec<String>,
    /// Pattern type
    pub pattern_type: PatternType,
}
/// Section in a report template
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ReportSection {
    /// Executive summary section
    ExecutiveSummary,
    /// Verification results
    VerificationResults,
    /// Quality metrics
    QualityMetrics,
    /// Compliance checklist
    ComplianceChecklist,
    /// Conflict detection
    ConflictDetection,
    /// Statistical analysis
    StatisticalAnalysis,
    /// Ambiguity detection
    AmbiguityDetection,
    /// Regulatory impact
    RegulatoryImpact,
    /// Graph analysis
    GraphAnalysis,
    /// Custom section with markdown content
    Custom { title: String, content: String },
}
/// Discrete-Time Markov Chain (DTMC)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MarkovChain {
    /// Chain identifier
    pub id: String,
    /// All states in the chain
    pub states: Vec<MarkovState>,
    /// State transitions with probabilities
    pub transitions: Vec<MarkovTransition>,
    /// Initial state ID
    pub initial_state: String,
}
impl MarkovChain {
    /// Creates a new Markov chain
    pub fn new(id: impl Into<String>, initial_state: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            states: vec![],
            transitions: vec![],
            initial_state: initial_state.into(),
        }
    }
    /// Adds a state to the chain
    pub fn add_state(mut self, state: MarkovState) -> Self {
        self.states.push(state);
        self
    }
    /// Adds a transition to the chain
    pub fn add_transition(mut self, transition: MarkovTransition) -> Self {
        self.transitions.push(transition);
        self
    }
    /// Validates that transition probabilities from each state sum to 1.0
    pub fn validate(&self) -> Result<(), String> {
        use std::collections::HashMap;
        let mut outgoing: HashMap<&str, f64> = HashMap::new();
        for transition in &self.transitions {
            *outgoing.entry(&transition.from).or_insert(0.0) += transition.probability;
        }
        for (state, total_prob) in outgoing {
            if (total_prob - 1.0).abs() > 0.01 {
                return Err(format!(
                    "State '{}' has transitions summing to {:.3} (should be 1.0)",
                    state, total_prob
                ));
            }
        }
        Ok(())
    }
    /// Computes steady-state probabilities using iterative method
    pub fn steady_state_probabilities(&self, max_iterations: usize) -> HashMap<String, f64> {
        use std::collections::HashMap;
        let mut probabilities: HashMap<String, f64> = HashMap::new();
        let num_states = self.states.len();
        if num_states == 0 {
            return probabilities;
        }
        let initial_prob = 1.0 / num_states as f64;
        for state in &self.states {
            probabilities.insert(state.id.clone(), initial_prob);
        }
        for _ in 0..max_iterations {
            let mut new_probs: HashMap<String, f64> = HashMap::new();
            for state in &self.states {
                let mut incoming_prob = 0.0;
                for transition in &self.transitions {
                    if transition.to == state.id {
                        let from_prob = probabilities.get(&transition.from).copied().unwrap_or(0.0);
                        incoming_prob += from_prob * transition.probability;
                    }
                }
                new_probs.insert(state.id.clone(), incoming_prob);
            }
            probabilities = new_probs;
        }
        probabilities
    }
    /// Computes reachability probability to accepting states
    pub fn reachability_probability(&self, steps: usize) -> f64 {
        use std::collections::HashMap;
        let mut probabilities: HashMap<String, f64> = HashMap::new();
        probabilities.insert(self.initial_state.clone(), 1.0);
        let mut accepting_prob = 0.0;
        for _ in 0..steps {
            let mut new_probs: HashMap<String, f64> = HashMap::new();
            for (from_state, from_prob) in &probabilities {
                let is_accepting = self
                    .states
                    .iter()
                    .any(|s| s.id == *from_state && s.accepting);
                if is_accepting {
                    accepting_prob += from_prob;
                } else {
                    for transition in &self.transitions {
                        if &transition.from == from_state {
                            *new_probs.entry(transition.to.clone()).or_insert(0.0) +=
                                from_prob * transition.probability;
                        }
                    }
                }
            }
            probabilities = new_probs;
        }
        for (state_id, prob) in &probabilities {
            if self.states.iter().any(|s| s.id == *state_id && s.accepting) {
                accepting_prob += prob;
            }
        }
        accepting_prob
    }
}
/// Conflict explanation for laypersons
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConflictExplanation {
    /// The statutes in conflict
    pub statute_ids: Vec<String>,
    /// Simple description of the conflict
    pub description: String,
    /// Real-world impact
    pub impact: String,
    /// Who is affected
    pub affected_parties: Vec<String>,
    /// Resolution options
    pub resolution_options: Vec<String>,
}
impl ConflictExplanation {
    /// Creates a new conflict explanation
    pub fn new(statute_ids: Vec<String>, description: impl Into<String>) -> Self {
        Self {
            statute_ids,
            description: description.into(),
            impact: String::new(),
            affected_parties: vec![],
            resolution_options: vec![],
        }
    }
    /// Adds impact description
    pub fn with_impact(mut self, impact: impl Into<String>) -> Self {
        self.impact = impact.into();
        self
    }
    /// Adds affected party
    pub fn add_affected_party(mut self, party: impl Into<String>) -> Self {
        self.affected_parties.push(party.into());
        self
    }
    /// Adds resolution option
    pub fn add_resolution_option(mut self, option: impl Into<String>) -> Self {
        self.resolution_options.push(option.into());
        self
    }
    /// Formats the explanation
    pub fn format(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!(
            "# Conflict Between: {}\n\n",
            self.statute_ids.join(", ")
        ));
        output.push_str(&format!(
            "## What's the Conflict?\n{}\n\n",
            self.description
        ));
        if !self.impact.is_empty() {
            output.push_str(&format!("## Real-World Impact\n{}\n\n", self.impact));
        }
        if !self.affected_parties.is_empty() {
            output.push_str("## Who's Affected?\n");
            for party in &self.affected_parties {
                output.push_str(&format!("- {}\n", party));
            }
            output.push('\n');
        }
        if !self.resolution_options.is_empty() {
            output.push_str("## How to Resolve This\n");
            for (i, option) in self.resolution_options.iter().enumerate() {
                output.push_str(&format!("{}. {}\n", i + 1, option));
            }
            output.push('\n');
        }
        output
    }
}
/// Hot-reload verification watcher
#[cfg(feature = "watch")]
#[derive(Debug)]
pub struct HotReloadWatcher {
    /// Path being watched
    pub watch_path: std::path::PathBuf,
    /// Receiver for file change events
    pub receiver: crossbeam_channel::Receiver<notify::Result<notify::Event>>,
    /// File watcher
    _watcher: notify::RecommendedWatcher,
}
#[cfg(feature = "watch")]
impl HotReloadWatcher {
    /// Creates a new hot-reload watcher
    pub fn new(path: impl Into<std::path::PathBuf>) -> anyhow::Result<Self> {
        use notify::Watcher;
        let watch_path = path.into();
        let (tx, rx) = crossbeam_channel::unbounded();
        let mut watcher = notify::recommended_watcher(move |res| {
            let _ = tx.send(res);
        })?;
        watcher.watch(&watch_path, notify::RecursiveMode::Recursive)?;
        Ok(Self {
            watch_path,
            receiver: rx,
            _watcher: watcher,
        })
    }
    /// Checks for file changes (non-blocking)
    pub fn check_changes(&self) -> Vec<String> {
        let mut changed_files = Vec::new();
        while let Ok(Ok(event)) = self.receiver.try_recv() {
            for path in event.paths {
                if let Some(path_str) = path.to_str() {
                    changed_files.push(path_str.to_string());
                }
            }
        }
        changed_files
    }
}
/// Compliance certification document
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComplianceCertification {
    /// Certificate ID
    pub certificate_id: String,
    /// Certification date
    pub certification_date: String,
    /// Organization name
    pub organization: String,
    /// Statutes certified
    pub statute_ids: Vec<String>,
    /// Verification results summary
    pub verification_summary: VerificationSummary,
    /// Certifying authority
    pub certifying_authority: String,
    /// Certificate validity period
    pub valid_until: Option<String>,
    /// Additional notes
    pub notes: Vec<String>,
}
/// A composite principle combining multiple principles.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CompositePrinciple {
    /// Identifier
    pub id: String,
    /// Name
    pub name: String,
    /// Component principles
    pub components: Vec<String>,
    /// How to combine results (All must pass or Any must pass)
    pub combination_mode: CombinationMode,
}
impl CompositePrinciple {
    /// Creates a new composite principle.
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            components: Vec::new(),
            combination_mode: CombinationMode::All,
        }
    }
    /// Adds a component principle.
    pub fn with_component(mut self, principle_id: impl Into<String>) -> Self {
        self.components.push(principle_id.into());
        self
    }
    /// Sets the combination mode.
    pub fn with_mode(mut self, mode: CombinationMode) -> Self {
        self.combination_mode = mode;
        self
    }
}
/// Linear Temporal Logic (LTL) formula.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum LtlFormula {
    /// Atomic proposition (a condition)
    Atom(String),
    /// Negation
    Not(Box<LtlFormula>),
    /// Conjunction (and)
    And(Box<LtlFormula>, Box<LtlFormula>),
    /// Disjunction (or)
    Or(Box<LtlFormula>, Box<LtlFormula>),
    /// Implication
    Implies(Box<LtlFormula>, Box<LtlFormula>),
    /// Next (holds in the next state)
    Next(Box<LtlFormula>),
    /// Eventually (holds at some point in the future)
    Eventually(Box<LtlFormula>),
    /// Always (holds at all points in the future)
    Always(Box<LtlFormula>),
    /// Until (first holds until second becomes true)
    Until(Box<LtlFormula>, Box<LtlFormula>),
    /// Release (second holds until first becomes true, or forever)
    Release(Box<LtlFormula>, Box<LtlFormula>),
}
impl LtlFormula {
    /// Creates a new atomic proposition.
    pub fn atom(name: impl Into<String>) -> Self {
        Self::Atom(name.into())
    }
    /// Creates a negation.
    #[allow(clippy::should_implement_trait)]
    pub fn not(formula: LtlFormula) -> Self {
        Self::Not(Box::new(formula))
    }
    /// Creates a conjunction.
    pub fn and(left: LtlFormula, right: LtlFormula) -> Self {
        Self::And(Box::new(left), Box::new(right))
    }
    /// Creates a disjunction.
    pub fn or(left: LtlFormula, right: LtlFormula) -> Self {
        Self::Or(Box::new(left), Box::new(right))
    }
    /// Creates an implication.
    pub fn implies(antecedent: LtlFormula, consequent: LtlFormula) -> Self {
        Self::Implies(Box::new(antecedent), Box::new(consequent))
    }
    /// Creates a next operator.
    pub fn next(formula: LtlFormula) -> Self {
        Self::Next(Box::new(formula))
    }
    /// Creates an eventually operator.
    pub fn eventually(formula: LtlFormula) -> Self {
        Self::Eventually(Box::new(formula))
    }
    /// Creates an always operator.
    pub fn always(formula: LtlFormula) -> Self {
        Self::Always(Box::new(formula))
    }
    /// Creates an until operator.
    pub fn until(left: LtlFormula, right: LtlFormula) -> Self {
        Self::Until(Box::new(left), Box::new(right))
    }
    /// Creates a release operator.
    pub fn release(left: LtlFormula, right: LtlFormula) -> Self {
        Self::Release(Box::new(left), Box::new(right))
    }
}
/// Represents an ambiguous term found in statutes.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct AmbiguousTerm {
    /// The ambiguous term
    pub term: String,
    /// Context where the term appears
    pub contexts: Vec<String>,
    /// Statute IDs where the term is used
    pub statute_ids: Vec<String>,
    /// Suggested disambiguations
    pub suggestions: Vec<String>,
}
impl AmbiguousTerm {
    /// Creates a new ambiguous term.
    pub fn new(term: impl Into<String>) -> Self {
        Self {
            term: term.into(),
            contexts: Vec::new(),
            statute_ids: Vec::new(),
            suggestions: Vec::new(),
        }
    }
    /// Adds a context to the ambiguous term.
    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.contexts.push(context.into());
        self
    }
    /// Adds a statute ID to the ambiguous term.
    pub fn with_statute_id(mut self, statute_id: impl Into<String>) -> Self {
        self.statute_ids.push(statute_id.into());
        self
    }
    /// Adds a suggestion for disambiguation.
    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestions.push(suggestion.into());
        self
    }
}
/// Gap in statute coverage - a scenario not handled by any statute.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CoverageGap {
    /// Description of the gap
    pub description: String,
    /// Example scenario that falls into this gap
    pub example_scenario: String,
    /// Severity of the gap (Info, Warning, Error, Critical)
    pub severity: Severity,
    /// Suggested statutes that might be related
    pub related_statutes: Vec<String>,
}
/// Type of dependency between statutes
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DependencyType {
    /// Derived from another statute
    DerivesFrom,
    /// Applies to certain entities
    AppliesTo,
    /// Exception references
    Exception,
    /// Temporal dependency
    Temporal,
}
/// Severity level for verification errors.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum Severity {
    /// Informational message
    Info,
    /// Warning that should be addressed
    Warning,
    /// Error that must be fixed
    Error,
    /// Critical error that prevents execution
    Critical,
}
/// Incremental verification state for tracking statute changes.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IncrementalState {
    /// Hashes of previously verified statutes
    statute_hashes: HashMap<String, u64>,
    /// Previous verification results
    previous_results: HashMap<String, VerificationResult>,
}
impl IncrementalState {
    /// Creates a new incremental state.
    pub fn new() -> Self {
        Self {
            statute_hashes: HashMap::new(),
            previous_results: HashMap::new(),
        }
    }
    /// Computes a hash for a statute.
    fn compute_hash(statute: &Statute) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        statute.id.hash(&mut hasher);
        statute.title.hash(&mut hasher);
        statute.preconditions.len().hash(&mut hasher);
        hasher.finish()
    }
    /// Checks if a statute has changed since last verification.
    pub fn has_changed(&self, statute: &Statute) -> bool {
        let current_hash = Self::compute_hash(statute);
        match self.statute_hashes.get(&statute.id) {
            Some(&previous_hash) => previous_hash != current_hash,
            None => true,
        }
    }
    /// Updates the state with a verified statute.
    pub fn update(&mut self, statute: &Statute, result: VerificationResult) {
        let hash = Self::compute_hash(statute);
        self.statute_hashes.insert(statute.id.clone(), hash);
        self.previous_results.insert(statute.id.clone(), result);
    }
    /// Gets the previous result for a statute if available.
    pub fn get_previous_result(&self, statute_id: &str) -> Option<&VerificationResult> {
        self.previous_results.get(statute_id)
    }
}
/// CTL* formula combining LTL and CTL path quantifiers.
/// CTL* is a superset of both LTL and CTL, allowing arbitrary mixing of
/// path quantifiers (E, A) with linear temporal operators (X, F, G, U, R).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CtlStarFormula {
    /// Atomic proposition
    Atom(String),
    /// Negation
    Not(Box<CtlStarFormula>),
    /// Conjunction
    And(Box<CtlStarFormula>, Box<CtlStarFormula>),
    /// Disjunction
    Or(Box<CtlStarFormula>, Box<CtlStarFormula>),
    /// Implication
    Implies(Box<CtlStarFormula>, Box<CtlStarFormula>),
    /// Path quantifier: Exists (there exists a path)
    Exists(Box<CtlStarPathFormula>),
    /// Path quantifier: All (for all paths)
    All(Box<CtlStarPathFormula>),
}
impl CtlStarFormula {
    /// Creates an atomic proposition.
    pub fn atom(name: impl Into<String>) -> Self {
        Self::Atom(name.into())
    }
    /// Creates a negation.
    #[allow(clippy::should_implement_trait)]
    pub fn not(formula: CtlStarFormula) -> Self {
        Self::Not(Box::new(formula))
    }
    /// Creates a conjunction.
    pub fn and(left: CtlStarFormula, right: CtlStarFormula) -> Self {
        Self::And(Box::new(left), Box::new(right))
    }
    /// Creates a disjunction.
    pub fn or(left: CtlStarFormula, right: CtlStarFormula) -> Self {
        Self::Or(Box::new(left), Box::new(right))
    }
    /// Creates an implication.
    pub fn implies(antecedent: CtlStarFormula, consequent: CtlStarFormula) -> Self {
        Self::Implies(Box::new(antecedent), Box::new(consequent))
    }
    /// Creates an exists quantifier.
    pub fn exists(path_formula: CtlStarPathFormula) -> Self {
        Self::Exists(Box::new(path_formula))
    }
    /// Creates an all quantifier.
    pub fn all(path_formula: CtlStarPathFormula) -> Self {
        Self::All(Box::new(path_formula))
    }
}
/// Tracks evolution for multiple statutes
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EvolutionTracker {
    /// Map from statute ID to evolution history
    pub evolutions: HashMap<String, StatuteEvolution>,
}
impl EvolutionTracker {
    /// Creates a new evolution tracker
    pub fn new() -> Self {
        Self {
            evolutions: HashMap::new(),
        }
    }
    /// Adds a statute (creates new evolution or adds version to existing)
    pub fn track_statute(&mut self, statute: Statute, description: Option<String>) {
        if let Some(evolution) = self.evolutions.get_mut(&statute.id) {
            evolution.add_version(statute, description);
        } else {
            self.evolutions
                .insert(statute.id.clone(), StatuteEvolution::new(statute));
        }
    }
    /// Gets evolution history for a statute
    pub fn get_evolution(&self, statute_id: &str) -> Option<&StatuteEvolution> {
        self.evolutions.get(statute_id)
    }
    /// Analyzes metrics for all tracked statutes
    pub fn analyze_all_metrics(&self) -> Vec<EvolutionMetrics> {
        self.evolutions
            .values()
            .map(|e| e.analyze_metrics())
            .collect()
    }
    /// Finds statutes with most changes
    pub fn most_changed_statutes(&self, limit: usize) -> Vec<EvolutionMetrics> {
        let mut metrics = self.analyze_all_metrics();
        metrics.sort_by(|a, b| {
            (b.major_changes + b.minor_changes).cmp(&(a.major_changes + a.minor_changes))
        });
        metrics.truncate(limit);
        metrics
    }
    /// Finds most stable statutes
    pub fn most_stable_statutes(&self, limit: usize) -> Vec<EvolutionMetrics> {
        let mut metrics = self.analyze_all_metrics();
        metrics.sort_by(|a, b| {
            b.stability_score
                .partial_cmp(&a.stability_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        metrics.truncate(limit);
        metrics
    }
}
/// CTL* path formula (used after path quantifiers).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CtlStarPathFormula {
    /// State formula
    State(Box<CtlStarFormula>),
    /// Negation of path formula
    Not(Box<CtlStarPathFormula>),
    /// Conjunction of path formulas
    And(Box<CtlStarPathFormula>, Box<CtlStarPathFormula>),
    /// Disjunction of path formulas
    Or(Box<CtlStarPathFormula>, Box<CtlStarPathFormula>),
    /// Next operator (holds in next state)
    Next(Box<CtlStarPathFormula>),
    /// Eventually operator (holds at some future state)
    Eventually(Box<CtlStarPathFormula>),
    /// Always operator (holds in all future states)
    Always(Box<CtlStarPathFormula>),
    /// Until operator
    Until(Box<CtlStarPathFormula>, Box<CtlStarPathFormula>),
    /// Release operator
    Release(Box<CtlStarPathFormula>, Box<CtlStarPathFormula>),
}
impl CtlStarPathFormula {
    /// Creates a path formula from a state formula.
    pub fn state(formula: CtlStarFormula) -> Self {
        Self::State(Box::new(formula))
    }
    /// Creates a next operator.
    pub fn next(formula: CtlStarPathFormula) -> Self {
        Self::Next(Box::new(formula))
    }
    /// Creates an eventually operator.
    pub fn eventually(formula: CtlStarPathFormula) -> Self {
        Self::Eventually(Box::new(formula))
    }
    /// Creates an always operator.
    pub fn always(formula: CtlStarPathFormula) -> Self {
        Self::Always(Box::new(formula))
    }
    /// Creates an until operator.
    pub fn until(left: CtlStarPathFormula, right: CtlStarPathFormula) -> Self {
        Self::Until(Box::new(left), Box::new(right))
    }
    /// Creates a release operator.
    pub fn release(left: CtlStarPathFormula, right: CtlStarPathFormula) -> Self {
        Self::Release(Box::new(left), Box::new(right))
    }
}
/// Registry for managing precedents.
#[derive(Debug, Clone, Default)]
pub struct PrecedentRegistry {
    /// All precedents in the registry
    precedents: Vec<Precedent>,
    /// Index by topic for fast lookup
    topic_index: HashMap<String, Vec<usize>>,
}
impl PrecedentRegistry {
    /// Creates a new precedent registry.
    pub fn new() -> Self {
        Self::default()
    }
    /// Adds a precedent to the registry.
    pub fn add_precedent(&mut self, precedent: Precedent) {
        let idx = self.precedents.len();
        for topic in &precedent.topics {
            self.topic_index.entry(topic.clone()).or_default().push(idx);
        }
        self.precedents.push(precedent);
    }
    /// Finds precedents related to a statute based on topics.
    pub fn find_related(&self, statute: &Statute, min_relevance: f64) -> Vec<&Precedent> {
        let statute_text = format!("{} {}", statute.id, statute.title);
        let words: HashSet<String> = statute_text
            .split_whitespace()
            .map(|s| s.to_lowercase())
            .collect();
        let mut seen = HashSet::new();
        let mut results = Vec::new();
        for word in words {
            if let Some(indices) = self.topic_index.get(&word) {
                for &idx in indices {
                    if seen.insert(idx) {
                        let precedent = &self.precedents[idx];
                        if precedent.relevance >= min_relevance {
                            results.push(precedent);
                        }
                    }
                }
            }
        }
        results.sort_by(|a, b| {
            b.relevance
                .partial_cmp(&a.relevance)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        results
    }
    /// Returns all precedents for a specific jurisdiction.
    pub fn by_jurisdiction(&self, jurisdiction: &str) -> Vec<&Precedent> {
        self.precedents
            .iter()
            .filter(|p| p.jurisdiction == jurisdiction)
            .collect()
    }
    /// Returns all precedents with a specific topic.
    pub fn by_topic(&self, topic: &str) -> Vec<&Precedent> {
        if let Some(indices) = self.topic_index.get(topic) {
            indices.iter().map(|&idx| &self.precedents[idx]).collect()
        } else {
            Vec::new()
        }
    }
}
/// Statistics for executive summary
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SummaryStatistics {
    /// Total statutes analyzed
    pub total_statutes: usize,
    /// Statutes with issues
    pub statutes_with_issues: usize,
    /// Total issues found
    pub total_issues: usize,
    /// Critical issues
    pub critical_issues: usize,
    /// High severity issues
    pub high_severity_issues: usize,
    /// Medium severity issues
    pub medium_severity_issues: usize,
    /// Average quality score
    pub average_quality_score: f64,
}
/// Represents a legal precedent that may be relevant to a statute.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Precedent {
    /// Unique identifier for the precedent
    pub id: String,
    /// Citation (e.g., case name, statute reference)
    pub citation: String,
    /// Jurisdiction
    pub jurisdiction: String,
    /// Year decided/enacted
    pub year: u32,
    /// Brief description or holding
    pub description: String,
    /// Relevance score (0.0 to 1.0)
    pub relevance: f64,
    /// Topics/tags
    pub topics: Vec<String>,
}
impl Precedent {
    /// Creates a new precedent.
    pub fn new(
        id: impl Into<String>,
        citation: impl Into<String>,
        jurisdiction: impl Into<String>,
        year: u32,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            citation: citation.into(),
            jurisdiction: jurisdiction.into(),
            year,
            description: description.into(),
            relevance: 0.0,
            topics: Vec::new(),
        }
    }
    /// Sets the relevance score.
    pub fn with_relevance(mut self, relevance: f64) -> Self {
        self.relevance = relevance.clamp(0.0, 1.0);
        self
    }
    /// Adds a topic/tag.
    pub fn with_topic(mut self, topic: impl Into<String>) -> Self {
        self.topics.push(topic.into());
        self
    }
}
/// A state in a temporal model.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TemporalState {
    /// State identifier
    pub id: String,
    /// Atomic propositions that hold in this state
    pub propositions: HashSet<String>,
}
impl TemporalState {
    /// Creates a new temporal state.
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            propositions: HashSet::new(),
        }
    }
    /// Adds a proposition to this state.
    pub fn with_proposition(mut self, prop: impl Into<String>) -> Self {
        self.propositions.insert(prop.into());
        self
    }
    /// Checks if a proposition holds in this state.
    pub fn satisfies(&self, prop: &str) -> bool {
        self.propositions.contains(prop)
    }
}
/// Statistical summary of a statute collection.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StatuteStatistics {
    /// Total number of statutes
    pub total_count: usize,
    /// Average number of preconditions per statute
    pub avg_preconditions: f64,
    /// Median number of preconditions
    pub median_preconditions: f64,
    /// Most common condition types
    pub common_condition_types: Vec<(String, usize)>,
    /// Jurisdiction distribution
    pub jurisdiction_distribution: HashMap<String, usize>,
    /// Average complexity score
    pub avg_complexity: f64,
    /// Effect type distribution
    pub effect_type_distribution: HashMap<String, usize>,
    /// Statutes with discretion logic count
    pub discretion_count: usize,
    /// Temporal validity coverage (statutes with dates)
    pub temporal_coverage: f64,
}
/// Manages multiple report schedules
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReportScheduler {
    /// Active schedules
    pub schedules: Vec<ReportSchedule>,
    /// Execution history
    pub history: Vec<ScheduledReportResult>,
}
impl ReportScheduler {
    /// Creates a new report scheduler
    pub fn new() -> Self {
        Self {
            schedules: Vec::new(),
            history: Vec::new(),
        }
    }
    /// Adds a schedule to the scheduler
    pub fn add_schedule(&mut self, schedule: ReportSchedule) {
        self.schedules.push(schedule);
    }
    /// Removes a schedule by ID
    pub fn remove_schedule(&mut self, schedule_id: &str) -> bool {
        if let Some(pos) = self.schedules.iter().position(|s| s.id == schedule_id) {
            self.schedules.remove(pos);
            true
        } else {
            false
        }
    }
    /// Gets a schedule by ID
    pub fn get_schedule(&self, schedule_id: &str) -> Option<&ReportSchedule> {
        self.schedules.iter().find(|s| s.id == schedule_id)
    }
    /// Gets a mutable schedule by ID
    pub fn get_schedule_mut(&mut self, schedule_id: &str) -> Option<&mut ReportSchedule> {
        self.schedules.iter_mut().find(|s| s.id == schedule_id)
    }
    /// Lists all schedules
    pub fn list_schedules(&self) -> &[ReportSchedule] {
        &self.schedules
    }
    /// Lists only enabled schedules
    pub fn list_enabled_schedules(&self) -> Vec<&ReportSchedule> {
        self.schedules.iter().filter(|s| s.enabled).collect()
    }
    /// Executes all enabled schedules that are due
    ///
    /// This checks each enabled schedule and executes it if it's time.
    /// Returns the list of execution results.
    ///
    /// # Arguments
    /// * `statutes` - The statutes to include in the report
    /// * `result` - The verification result to include in the report
    pub fn execute_due_schedules(
        &mut self,
        statutes: &[Statute],
        result: &VerificationResult,
    ) -> Vec<ScheduledReportResult> {
        let mut execution_results = Vec::new();
        for schedule in &self.schedules {
            if schedule.enabled {
                let exec_result = execute_scheduled_report(schedule, statutes, result);
                execution_results.push(exec_result);
            }
        }
        self.history.extend(execution_results.clone());
        execution_results
    }
    /// Gets the execution history
    pub fn get_history(&self) -> &[ScheduledReportResult] {
        &self.history
    }
    /// Gets execution history for a specific schedule
    pub fn get_schedule_history(&self, schedule_id: &str) -> Vec<&ScheduledReportResult> {
        self.history
            .iter()
            .filter(|r| r.schedule_id == schedule_id)
            .collect()
    }
    /// Clears the execution history
    pub fn clear_history(&mut self) {
        self.history.clear();
    }
    /// Exports scheduler configuration to JSON
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|_| "{}".to_string())
    }
    /// Imports scheduler configuration from JSON
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}
/// Types of interactions between statutes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum InteractionType {
    /// One statute modifies another
    Modification,
    /// One statute extends another
    Extension,
    /// Statutes complement each other
    Complementary,
    /// One statute supersedes another
    Supersession,
    /// Statutes have mutual dependency
    MutualDependency,
    /// One statute contradicts another
    Contradiction,
    /// Statutes have overlapping scope
    Overlap,
}
/// Enhanced coverage gap with more detailed analysis
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EnhancedCoverageGap {
    /// Type of gap
    pub gap_type: GapType,
    /// Description of the gap
    pub description: String,
    /// Example scenario that falls in the gap
    pub example_scenario: String,
    /// Severity of the gap
    pub severity: Severity,
    /// Related statutes that create the gap
    pub related_statutes: Vec<String>,
    /// Suggested statute to fill the gap
    pub suggested_coverage: String,
}
/// Complexity metrics for a statute.
#[derive(Debug, Clone, Default)]
pub struct ComplexityMetrics {
    /// Number of preconditions
    pub condition_count: usize,
    /// Maximum nesting depth of conditions
    pub condition_depth: usize,
    /// Number of logical operators (AND, OR, NOT)
    pub logical_operator_count: usize,
    /// Number of distinct condition types
    pub condition_type_count: usize,
    /// Whether the statute has discretion logic
    pub has_discretion: bool,
    /// Cyclomatic complexity (simplified)
    pub cyclomatic_complexity: usize,
    /// Overall complexity score (0-100, higher = more complex)
    pub complexity_score: u32,
    /// Complexity level
    pub complexity_level: ComplexityLevel,
}
