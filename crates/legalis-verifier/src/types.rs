//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use legalis_core::Statute;
use std::collections::{HashMap, HashSet};

use super::functions::analyze_complexity;
use super::functions_3::compare_statutes;
#[cfg(feature = "smt-solver")]
use super::smt;
use super::types_3::{
    ConflictType, EvolutionMetrics, NotificationType, PrincipleCheck, ProofStepType, StatuteVersion,
};
use super::types_4::{ImpactLevel, IncrementalState, Severity};
use super::types_5::{
    ConstitutionalPrinciple, StatuteChange, VerificationError, VerificationResult,
};

/// Types of cross-reference errors.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CrossReferenceErrorType {
    /// Referenced statute does not exist
    NotFound,
    /// Reference creates a circular dependency
    CircularReference,
    /// Reference is ambiguous (multiple matches)
    Ambiguous,
}
/// Result of batch verification across multiple statutes.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BatchVerificationResult {
    /// Total number of statutes processed
    pub total_statutes: usize,
    /// Number of statutes that passed verification
    pub passed: usize,
    /// Number of statutes that failed verification
    pub failed: usize,
    /// Individual results for each statute
    pub individual_results: HashMap<String, VerificationResult>,
    /// Overall statistics
    pub error_counts: HashMap<Severity, usize>,
    /// Total verification time in milliseconds
    pub total_time_ms: u64,
}
impl BatchVerificationResult {
    /// Creates a new batch verification result.
    pub fn new() -> Self {
        Self {
            total_statutes: 0,
            passed: 0,
            failed: 0,
            individual_results: HashMap::new(),
            error_counts: HashMap::new(),
            total_time_ms: 0,
        }
    }
    /// Adds a result for a statute.
    pub fn add_result(&mut self, statute_id: String, result: VerificationResult) {
        self.total_statutes += 1;
        if result.passed {
            self.passed += 1;
        } else {
            self.failed += 1;
        }
        for error in &result.errors {
            *self.error_counts.entry(error.severity()).or_insert(0) += 1;
        }
        self.individual_results.insert(statute_id, result);
    }
    /// Returns the pass rate as a percentage.
    pub fn pass_rate(&self) -> f64 {
        if self.total_statutes == 0 {
            0.0
        } else {
            (self.passed as f64 / self.total_statutes as f64) * 100.0
        }
    }
}
/// Areas where statutes can overlap
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum OverlapArea {
    /// Jurisdiction overlap
    Jurisdiction,
    /// Subject matter overlap
    SubjectMatter,
    /// Temporal overlap
    Temporal,
    /// Population overlap (same target group)
    Population,
    /// Enforcement overlap
    Enforcement,
}
/// Overall quality score for a statute.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct QualityMetrics {
    /// Statute ID
    pub statute_id: String,
    /// Overall quality score (0.0 to 100.0)
    pub overall_score: f64,
    /// Complexity score (lower is better, 0-100)
    pub complexity_score: f64,
    /// Readability score (higher is better, 0-100)
    pub readability_score: f64,
    /// Consistency score (higher is better, 0-100)
    pub consistency_score: f64,
    /// Completeness score (higher is better, 0-100)
    pub completeness_score: f64,
    /// Legislative drafting quality score (0-100)
    pub drafting_quality_score: f64,
    /// Clarity index (0-100)
    pub clarity_index: f64,
    /// Testability assessment score (0-100)
    pub testability_score: f64,
    /// Maintainability score (0-100)
    pub maintainability_score: f64,
    /// List of quality issues
    pub issues: Vec<String>,
    /// List of quality strengths
    pub strengths: Vec<String>,
}
impl QualityMetrics {
    /// Creates a new quality metrics instance.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        statute_id: String,
        complexity_score: f64,
        readability_score: f64,
        consistency_score: f64,
        completeness_score: f64,
        drafting_quality_score: f64,
        clarity_index: f64,
        testability_score: f64,
        maintainability_score: f64,
    ) -> Self {
        let overall_score = (complexity_score
            + readability_score
            + consistency_score
            + completeness_score
            + drafting_quality_score
            + clarity_index
            + testability_score
            + maintainability_score)
            / 8.0;
        Self {
            statute_id,
            overall_score,
            complexity_score,
            readability_score,
            consistency_score,
            completeness_score,
            drafting_quality_score,
            clarity_index,
            testability_score,
            maintainability_score,
            issues: Vec::new(),
            strengths: Vec::new(),
        }
    }
    /// Adds a quality issue.
    pub fn with_issue(mut self, issue: impl Into<String>) -> Self {
        self.issues.push(issue.into());
        self
    }
    /// Adds a quality strength.
    pub fn with_strength(mut self, strength: impl Into<String>) -> Self {
        self.strengths.push(strength.into());
        self
    }
    /// Returns a quality grade (A, B, C, D, F).
    pub fn grade(&self) -> char {
        if self.overall_score >= 90.0 {
            'A'
        } else if self.overall_score >= 80.0 {
            'B'
        } else if self.overall_score >= 70.0 {
            'C'
        } else if self.overall_score >= 60.0 {
            'D'
        } else {
            'F'
        }
    }
}
/// Budget for verification operations.
#[derive(Debug, Clone, Copy)]
pub struct VerificationBudget {
    /// Maximum number of statutes to verify (None = unlimited)
    pub max_statutes: Option<usize>,
    /// Maximum number of checks to perform (None = unlimited)
    pub max_checks: Option<usize>,
    /// Maximum time in milliseconds (None = unlimited)
    pub max_time_ms: Option<u64>,
}
impl VerificationBudget {
    /// Creates an unlimited budget.
    pub fn unlimited() -> Self {
        Self {
            max_statutes: None,
            max_checks: None,
            max_time_ms: None,
        }
    }
    /// Creates a budget with maximum number of statutes.
    pub fn with_max_statutes(max: usize) -> Self {
        Self {
            max_statutes: Some(max),
            max_checks: None,
            max_time_ms: None,
        }
    }
    /// Creates a budget with maximum number of checks.
    pub fn with_max_checks(max: usize) -> Self {
        Self {
            max_statutes: None,
            max_checks: Some(max),
            max_time_ms: None,
        }
    }
    /// Creates a budget with maximum time.
    pub fn with_max_time_ms(max: u64) -> Self {
        Self {
            max_statutes: None,
            max_checks: None,
            max_time_ms: Some(max),
        }
    }
    /// Checks if the statute limit has been reached.
    pub fn statute_limit_reached(&self, count: usize) -> bool {
        self.max_statutes.is_some_and(|max| count >= max)
    }
    /// Checks if the check limit has been reached.
    pub fn check_limit_reached(&self, count: usize) -> bool {
        self.max_checks.is_some_and(|max| count >= max)
    }
    /// Checks if the time limit has been reached.
    pub fn time_limit_reached(&self, elapsed_ms: u64) -> bool {
        self.max_time_ms.is_some_and(|max| elapsed_ms >= max)
    }
}
/// Represents a conflict between two or more statutes.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StatuteConflict {
    /// Type of conflict
    pub conflict_type: ConflictType,
    /// IDs of statutes involved in the conflict
    pub statute_ids: Vec<String>,
    /// Description of the conflict
    pub description: String,
    /// Severity of the conflict
    pub severity: Severity,
    /// Suggestions for resolving the conflict
    pub resolution_suggestions: Vec<String>,
}
impl StatuteConflict {
    /// Creates a new statute conflict.
    pub fn new(
        conflict_type: ConflictType,
        statute_ids: Vec<String>,
        description: impl Into<String>,
    ) -> Self {
        let severity = match conflict_type {
            ConflictType::EffectConflict => Severity::Critical,
            ConflictType::HierarchyViolation => Severity::Critical,
            ConflictType::IdCollision => Severity::Error,
            ConflictType::JurisdictionalOverlap => Severity::Warning,
            ConflictType::TemporalConflict => Severity::Warning,
        };
        Self {
            conflict_type,
            statute_ids,
            description: description.into(),
            severity,
            resolution_suggestions: Vec::new(),
        }
    }
    /// Adds a resolution suggestion.
    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.resolution_suggestions.push(suggestion.into());
        self
    }
}
/// Verification path node for visualization
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VerificationPathNode {
    /// Node identifier
    pub id: String,
    /// Node type (statute, condition, effect, etc.)
    pub node_type: String,
    /// Display label
    pub label: String,
    /// Whether this node passed verification
    pub passed: bool,
    /// Child nodes
    pub children: Vec<VerificationPathNode>,
    /// Additional metadata
    pub metadata: std::collections::HashMap<String, String>,
}
impl VerificationPathNode {
    /// Creates a new path node
    pub fn new(
        id: impl Into<String>,
        node_type: impl Into<String>,
        label: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            node_type: node_type.into(),
            label: label.into(),
            passed: true,
            children: vec![],
            metadata: std::collections::HashMap::new(),
        }
    }
    /// Sets the pass/fail status
    pub fn with_status(mut self, passed: bool) -> Self {
        self.passed = passed;
        self
    }
    /// Adds a child node
    pub fn add_child(mut self, child: VerificationPathNode) -> Self {
        self.children.push(child);
        self
    }
    /// Adds metadata
    pub fn add_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
    /// Exports as DOT format for Graphviz
    pub fn to_dot(&self) -> String {
        let mut dot = String::from("digraph VerificationPath {\n");
        dot.push_str("  node [shape=box];\n");
        self.to_dot_recursive(&mut dot, None);
        dot.push_str("}\n");
        dot
    }
    fn to_dot_recursive(&self, dot: &mut String, parent_id: Option<&str>) {
        let color = if self.passed { "green" } else { "red" };
        let style = if self.passed { "solid" } else { "bold" };
        dot.push_str(&format!(
            "  \"{}\" [label=\"{}\\n({})\", color={}, style={}];\n",
            self.id, self.label, self.node_type, color, style
        ));
        if let Some(parent) = parent_id {
            dot.push_str(&format!("  \"{}\" -> \"{}\";\n", parent, self.id));
        }
        for child in &self.children {
            child.to_dot_recursive(dot, Some(&self.id));
        }
    }
}
/// What-if scenario for testing statute changes
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WhatIfScenario {
    /// Scenario description
    pub description: String,
    /// Original statute
    pub original_statute: Statute,
    /// Modified statute
    pub modified_statute: Statute,
    /// Changes made
    pub changes: Vec<String>,
    /// Original verification result
    pub original_result: VerificationResult,
    /// New verification result after changes
    pub new_result: VerificationResult,
}
impl WhatIfScenario {
    /// Creates a new what-if scenario
    pub fn new(
        description: impl Into<String>,
        original: Statute,
        modified: Statute,
        original_result: VerificationResult,
        new_result: VerificationResult,
    ) -> Self {
        let changes = Self::detect_changes(&original, &modified);
        Self {
            description: description.into(),
            original_statute: original,
            modified_statute: modified,
            changes,
            original_result,
            new_result,
        }
    }
    fn detect_changes(original: &Statute, modified: &Statute) -> Vec<String> {
        let mut changes = vec![];
        if original.title != modified.title {
            changes.push(format!(
                "Title changed from '{}' to '{}'",
                original.title, modified.title
            ));
        }
        if original.effect.effect_type != modified.effect.effect_type {
            changes.push(format!(
                "Effect type changed from {:?} to {:?}",
                original.effect.effect_type, modified.effect.effect_type
            ));
        }
        if original.preconditions != modified.preconditions {
            changes.push("Preconditions modified".to_string());
        }
        if original.jurisdiction != modified.jurisdiction {
            changes.push("Jurisdiction changed".to_string());
        }
        changes
    }
    /// Generates a comparison report
    pub fn report(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("# What-If Scenario: {}\n\n", self.description));
        output.push_str("## Changes Made\n");
        for change in &self.changes {
            output.push_str(&format!("- {}\n", change));
        }
        output.push('\n');
        output.push_str("## Impact Analysis\n");
        output.push_str(&format!(
            "**Before**: {} errors, {} warnings\n",
            self.original_result.errors.len(),
            self.original_result.warnings.len()
        ));
        output.push_str(&format!(
            "**After**: {} errors, {} warnings\n\n",
            self.new_result.errors.len(),
            self.new_result.warnings.len()
        ));
        let error_delta =
            self.new_result.errors.len() as i32 - self.original_result.errors.len() as i32;
        let warning_delta =
            self.new_result.warnings.len() as i32 - self.original_result.warnings.len() as i32;
        if error_delta < 0 {
            output.push_str(&format!("✓ Reduced errors by {}\n", error_delta.abs()));
        } else if error_delta > 0 {
            output.push_str(&format!("✗ Increased errors by {}\n", error_delta));
        }
        if warning_delta < 0 {
            output.push_str(&format!("✓ Reduced warnings by {}\n", warning_delta.abs()));
        } else if warning_delta > 0 {
            output.push_str(&format!("✗ Increased warnings by {}\n", warning_delta));
        }
        output.push('\n');
        if self.new_result.passed && !self.original_result.passed {
            output.push_str("**✓ This change fixes the statute!**\n\n");
        } else if !self.new_result.passed && self.original_result.passed {
            output.push_str("**✗ This change breaks the statute!**\n\n");
        }
        output
    }
}
/// Summary of verification results for certification
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VerificationSummary {
    /// Total statutes verified
    pub total_statutes: usize,
    /// Statutes passed
    pub passed_count: usize,
    /// Statutes failed
    pub failed_count: usize,
    /// Pass rate percentage
    pub pass_rate: f64,
    /// Critical errors found
    pub critical_errors: usize,
    /// Warnings found
    pub warnings: usize,
}
/// Represents a strategy in a game-theoretic model
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Strategy {
    /// Stakeholder who plays this strategy
    pub stakeholder_id: String,
    /// Name of the strategy
    pub name: String,
    /// Description of the strategy
    pub description: String,
    /// Statutes invoked or complied with
    pub statute_actions: Vec<String>,
}
impl Strategy {
    /// Creates a new strategy
    pub fn new(stakeholder_id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            stakeholder_id: stakeholder_id.into(),
            name: name.into(),
            description: String::new(),
            statute_actions: Vec::new(),
        }
    }
    /// Sets the description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }
    /// Adds a statute action
    pub fn with_statute_action(mut self, statute_id: impl Into<String>) -> Self {
        self.statute_actions.push(statute_id.into());
        self
    }
}
/// Evolution history for a statute
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StatuteEvolution {
    /// Statute ID
    pub statute_id: String,
    /// Chronological list of versions
    pub versions: Vec<StatuteVersion>,
}
impl StatuteEvolution {
    /// Creates a new evolution history starting with an initial statute
    pub fn new(statute: Statute) -> Self {
        Self {
            statute_id: statute.id.clone(),
            versions: vec![StatuteVersion {
                version: statute.version,
                statute,
                timestamp: None,
                change_description: None,
            }],
        }
    }
    /// Adds a new version to the evolution history
    pub fn add_version(&mut self, statute: Statute, description: Option<String>) {
        self.versions.push(StatuteVersion {
            version: statute.version,
            statute,
            timestamp: Some(chrono::Utc::now().naive_utc()),
            change_description: description,
        });
    }
    /// Gets the latest version
    pub fn latest_version(&self) -> Option<&StatuteVersion> {
        self.versions.last()
    }
    /// Gets a specific version by number
    pub fn get_version(&self, version: u32) -> Option<&StatuteVersion> {
        self.versions.iter().find(|v| v.version == version)
    }
    /// Analyzes the evolution metrics
    pub fn analyze_metrics(&self) -> EvolutionMetrics {
        let total_versions = self.versions.len();
        let mut major_changes = 0;
        let mut minor_changes = 0;
        for i in 1..self.versions.len() {
            let old = &self.versions[i - 1].statute;
            let new = &self.versions[i].statute;
            let changes = compare_statutes(old, new);
            for change in changes {
                match change {
                    StatuteChange::EffectChanged { .. }
                    | StatuteChange::PreconditionsChanged { .. } => {
                        major_changes += 1;
                    }
                    _ => {
                        minor_changes += 1;
                    }
                }
            }
        }
        let avg_days = if self.versions.len() > 1 {
            let mut total_days = 0.0;
            let mut count = 0;
            for i in 1..self.versions.len() {
                if let (Some(prev_ts), Some(curr_ts)) =
                    (&self.versions[i - 1].timestamp, &self.versions[i].timestamp)
                {
                    let duration = curr_ts.signed_duration_since(*prev_ts);
                    total_days += duration.num_days() as f64;
                    count += 1;
                }
            }
            if count > 0 {
                Some(total_days / count as f64)
            } else {
                None
            }
        } else {
            None
        };
        let total_changes = major_changes + minor_changes;
        let stability_score = if total_versions > 1 {
            1.0 - (total_changes as f64 / (total_versions - 1) as f64).min(1.0)
        } else {
            1.0
        };
        let complexity_trend = if self.versions.len() >= 3 {
            let first_complexity = analyze_complexity(&self.versions[0].statute);
            let last_complexity =
                analyze_complexity(&self.versions[self.versions.len() - 1].statute);
            let diff = (last_complexity.logical_operator_count as i32)
                - (first_complexity.logical_operator_count as i32);
            if diff > 2 {
                ComplexityTrend::Increasing
            } else if diff < -2 {
                ComplexityTrend::Decreasing
            } else {
                ComplexityTrend::Stable
            }
        } else {
            ComplexityTrend::Stable
        };
        EvolutionMetrics {
            statute_id: self.statute_id.clone(),
            total_versions,
            major_changes,
            minor_changes,
            avg_days_between_versions: avg_days,
            stability_score,
            complexity_trend,
        }
    }
}
/// Represents a step in a verification proof
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProofStep {
    /// Step number
    pub step_number: usize,
    /// Type of proof step
    pub step_type: ProofStepType,
    /// Description of what this step proves
    pub description: String,
    /// The formula or condition being proven
    pub formula: String,
    /// Justification for this step
    pub justification: String,
    /// References to previous steps this depends on
    pub depends_on: Vec<usize>,
}
/// Natural language explanation for a verification error
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NaturalLanguageExplanation {
    /// The original verification error
    pub error_type: String,
    /// Simple explanation (for laypersons)
    pub simple_explanation: String,
    /// Detailed technical explanation
    pub technical_explanation: String,
    /// Why this is a problem
    pub why_it_matters: String,
    /// Suggested fix in plain language
    pub how_to_fix: String,
    /// Example scenario illustrating the problem
    pub example_scenario: Option<String>,
}
impl NaturalLanguageExplanation {
    /// Creates a new explanation
    pub fn new(
        error_type: impl Into<String>,
        simple: impl Into<String>,
        technical: impl Into<String>,
        why: impl Into<String>,
        fix: impl Into<String>,
    ) -> Self {
        Self {
            error_type: error_type.into(),
            simple_explanation: simple.into(),
            technical_explanation: technical.into(),
            why_it_matters: why.into(),
            how_to_fix: fix.into(),
            example_scenario: None,
        }
    }
    /// Adds an example scenario
    pub fn with_example(mut self, example: impl Into<String>) -> Self {
        self.example_scenario = Some(example.into());
        self
    }
    /// Generates a formatted explanation
    pub fn format(&self, include_technical: bool) -> String {
        let mut output = String::new();
        output.push_str(&format!("# {}\n\n", self.error_type));
        output.push_str(&format!(
            "## What's Wrong?\n{}\n\n",
            self.simple_explanation
        ));
        if include_technical {
            output.push_str(&format!(
                "## Technical Details\n{}\n\n",
                self.technical_explanation
            ));
        }
        output.push_str(&format!("## Why This Matters\n{}\n\n", self.why_it_matters));
        output.push_str(&format!("## How to Fix It\n{}\n\n", self.how_to_fix));
        if let Some(example) = &self.example_scenario {
            output.push_str(&format!("## Example\n{}\n\n", example));
        }
        output
    }
}
/// Encrypted verification result
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EncryptedVerificationResult {
    /// Encrypted verification outcome
    pub encrypted_result: Vec<u8>,
    /// Encryption scheme
    pub scheme: String,
}
impl EncryptedVerificationResult {
    /// Generates a report (without decrypting)
    pub fn report(&self) -> String {
        format!(
            "Encrypted Verification Result\n\
             =============================\n\
             Scheme: {}\n\
             Result Size: {} bytes\n\
             (Result is encrypted and cannot be read without decryption key)\n",
            self.scheme,
            self.encrypted_result.len()
        )
    }
}
/// Impact assessment for a statute or set of statutes.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ImpactAssessment {
    /// Affected groups
    pub affected_groups: Vec<String>,
    /// Positive impacts
    pub positive_impacts: Vec<String>,
    /// Negative impacts
    pub negative_impacts: Vec<String>,
    /// Equity concerns
    pub equity_concerns: Vec<String>,
    /// Accessibility concerns
    pub accessibility_concerns: Vec<String>,
    /// Privacy concerns
    pub privacy_concerns: Vec<String>,
    /// Economic impact level (Low, Medium, High)
    pub economic_impact: ImpactLevel,
    /// Social impact level (Low, Medium, High)
    pub social_impact: ImpactLevel,
    /// Overall risk level (Low, Medium, High, Critical)
    pub overall_risk: RiskLevel,
}
impl ImpactAssessment {
    /// Creates a new impact assessment.
    pub fn new() -> Self {
        Self {
            affected_groups: Vec::new(),
            positive_impacts: Vec::new(),
            negative_impacts: Vec::new(),
            equity_concerns: Vec::new(),
            accessibility_concerns: Vec::new(),
            privacy_concerns: Vec::new(),
            economic_impact: ImpactLevel::Low,
            social_impact: ImpactLevel::Low,
            overall_risk: RiskLevel::Low,
        }
    }
    /// Generates a human-readable report.
    pub fn report(&self) -> String {
        let mut report = String::new();
        report.push_str("# Impact Assessment Report\n\n");
        report.push_str(&format!(
            "**Overall Risk Level**: {}\n\n",
            self.overall_risk
        ));
        if !self.affected_groups.is_empty() {
            report.push_str("## Affected Groups\n");
            for group in &self.affected_groups {
                report.push_str(&format!("- {}\n", group));
            }
            report.push('\n');
        }
        if !self.positive_impacts.is_empty() {
            report.push_str("## Positive Impacts\n");
            for impact in &self.positive_impacts {
                report.push_str(&format!("- {}\n", impact));
            }
            report.push('\n');
        }
        if !self.negative_impacts.is_empty() {
            report.push_str("## Negative Impacts\n");
            for impact in &self.negative_impacts {
                report.push_str(&format!("- {}\n", impact));
            }
            report.push('\n');
        }
        if !self.equity_concerns.is_empty() {
            report.push_str("## Equity Concerns\n");
            for concern in &self.equity_concerns {
                report.push_str(&format!("- {}\n", concern));
            }
            report.push('\n');
        }
        if !self.accessibility_concerns.is_empty() {
            report.push_str("## Accessibility Concerns\n");
            for concern in &self.accessibility_concerns {
                report.push_str(&format!("- {}\n", concern));
            }
            report.push('\n');
        }
        if !self.privacy_concerns.is_empty() {
            report.push_str("## Privacy Concerns\n");
            for concern in &self.privacy_concerns {
                report.push_str(&format!("- {}\n", concern));
            }
            report.push('\n');
        }
        report.push_str("## Impact Levels\n");
        report.push_str(&format!("- Economic Impact: {}\n", self.economic_impact));
        report.push_str(&format!("- Social Impact: {}\n", self.social_impact));
        report
    }
}
/// Sequence constraint specifying required event ordering.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SequenceConstraint {
    /// Identifier for this constraint
    pub id: String,
    /// Events that must occur in order
    pub events: Vec<String>,
    /// Whether the sequence must be immediate (no other events between)
    pub strict: bool,
    /// Description of the constraint
    pub description: String,
}
impl SequenceConstraint {
    /// Creates a new sequence constraint.
    pub fn new(id: impl Into<String>, events: Vec<String>) -> Self {
        Self {
            id: id.into(),
            events,
            strict: false,
            description: String::new(),
        }
    }
    /// Makes the sequence strict (events must be immediate).
    pub fn strict(mut self) -> Self {
        self.strict = true;
        self
    }
    /// Adds a description.
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }
}
/// Represents a transition between states with probability
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MarkovTransition {
    /// Source state ID
    pub from: String,
    /// Target state ID
    pub to: String,
    /// Transition probability (0.0 to 1.0)
    pub probability: f64,
    /// Optional action/event label
    pub action: Option<String>,
}
impl MarkovTransition {
    /// Creates a new transition
    pub fn new(from: impl Into<String>, to: impl Into<String>, probability: f64) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
            probability: probability.clamp(0.0, 1.0),
            action: None,
        }
    }
    /// Adds an action label
    pub fn with_action(mut self, action: impl Into<String>) -> Self {
        self.action = Some(action.into());
        self
    }
}
/// Code coverage information for condition analysis.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct CoverageInfo {
    /// Total number of conditions analyzed
    pub total_conditions: usize,
    /// Number of conditions that were evaluated as satisfiable
    pub satisfiable_conditions: usize,
    /// Number of conditions that were evaluated as unsatisfiable
    pub unsatisfiable_conditions: usize,
    /// Conditions covered (by statute ID and condition index)
    pub covered_conditions: HashMap<String, Vec<usize>>,
    /// Conditions not covered
    pub uncovered_conditions: HashMap<String, Vec<usize>>,
    /// Coverage percentage (0-100)
    pub coverage_percentage: f64,
}
impl CoverageInfo {
    /// Creates a new empty coverage info.
    pub fn new() -> Self {
        Self::default()
    }
    /// Computes coverage percentage.
    pub fn compute_percentage(&mut self) {
        if self.total_conditions > 0 {
            let covered = self
                .covered_conditions
                .values()
                .map(|v| v.len())
                .sum::<usize>();
            self.coverage_percentage = (covered as f64 / self.total_conditions as f64) * 100.0;
        } else {
            self.coverage_percentage = 0.0;
        }
    }
    /// Returns true if coverage is complete (100%).
    pub fn is_complete(&self) -> bool {
        self.coverage_percentage >= 100.0
    }
    /// Generates a human-readable coverage report.
    pub fn report(&self) -> String {
        let mut report = String::new();
        report.push_str("# Condition Coverage Report\n\n");
        report.push_str(&format!("Total Conditions: {}\n", self.total_conditions));
        report.push_str(&format!("Satisfiable: {}\n", self.satisfiable_conditions));
        report.push_str(&format!(
            "Unsatisfiable: {}\n",
            self.unsatisfiable_conditions
        ));
        report.push_str(&format!("Coverage: {:.2}%\n\n", self.coverage_percentage));
        if !self.covered_conditions.is_empty() {
            report.push_str("## Covered Conditions\n");
            for (statute_id, indices) in &self.covered_conditions {
                report.push_str(&format!("- {}: {:?}\n", statute_id, indices));
            }
            report.push('\n');
        }
        if !self.uncovered_conditions.is_empty() {
            report.push_str("## Uncovered Conditions\n");
            for (statute_id, indices) in &self.uncovered_conditions {
                report.push_str(&format!("- {}: {:?}\n", statute_id, indices));
            }
            report.push('\n');
        }
        report
    }
}
/// Trend in statute complexity over time
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ComplexityTrend {
    /// Complexity is increasing
    Increasing,
    /// Complexity is decreasing
    Decreasing,
    /// Complexity is stable
    Stable,
}
/// Notification message.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NotificationMessage {
    /// Notification type
    pub notification_type: NotificationType,
    /// Title/subject
    pub title: String,
    /// Message body
    pub message: String,
    /// Timestamp (RFC 3339 format)
    pub timestamp: String,
    /// Verification results (if include_details is true)
    pub results: Option<Vec<VerificationResult>>,
}
impl NotificationMessage {
    /// Creates a new notification message.
    pub fn new(
        notification_type: NotificationType,
        title: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            notification_type,
            title: title.into(),
            message: message.into(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            results: None,
        }
    }
    /// Adds verification results.
    pub fn with_results(mut self, results: Vec<VerificationResult>) -> Self {
        self.results = Some(results);
        self
    }
    /// Converts to JSON for webhook delivery.
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|_| "{}".to_string())
    }
}
/// Represents a conflict cascade - how conflicts propagate through statute dependencies
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConflictCascade {
    /// The original conflicting statutes
    pub origin_statutes: Vec<String>,
    /// Statutes affected by the cascade
    pub affected_statutes: Vec<String>,
    /// Cascade depth (levels of propagation)
    pub depth: usize,
    /// Description of the cascade
    pub description: String,
    /// Impact severity
    pub severity: Severity,
}
/// Risk level classification.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum RiskLevel {
    Minimal,
    Low,
    Medium,
    High,
    Critical,
}
impl RiskLevel {
    /// Classifies a risk score into a level
    pub fn from_score(score: f64) -> Self {
        if score < 0.25 {
            RiskLevel::Minimal
        } else if score < 0.50 {
            RiskLevel::Low
        } else if score < 0.75 {
            RiskLevel::Medium
        } else if score < 0.90 {
            RiskLevel::High
        } else {
            RiskLevel::Critical
        }
    }
}
/// Result of a scheduled report execution
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScheduledReportResult {
    /// Schedule ID that was executed
    pub schedule_id: String,
    /// Execution timestamp (RFC 3339)
    pub execution_time: String,
    /// Whether the report generation succeeded
    pub success: bool,
    /// Path to the generated report file
    pub output_path: Option<String>,
    /// Error message if generation failed
    pub error: Option<String>,
    /// Report file size in bytes
    pub file_size_bytes: Option<u64>,
}
/// Verifier for legal statutes.
pub struct StatuteVerifier {
    /// Constitutional principles to check against
    constitutional_principles: Vec<ConstitutionalPrinciple>,
    /// Cache for verification results
    cache: std::sync::Arc<std::sync::Mutex<HashMap<String, VerificationResult>>>,
    /// Whether caching is enabled
    caching_enabled: bool,
}
impl StatuteVerifier {
    /// Creates a new verifier.
    pub fn new() -> Self {
        Self {
            constitutional_principles: Self::default_principles(),
            cache: std::sync::Arc::new(std::sync::Mutex::new(HashMap::new())),
            caching_enabled: false,
        }
    }
    /// Creates a verifier with custom principles.
    pub fn with_principles(principles: Vec<ConstitutionalPrinciple>) -> Self {
        Self {
            constitutional_principles: principles,
            cache: std::sync::Arc::new(std::sync::Mutex::new(HashMap::new())),
            caching_enabled: false,
        }
    }
    /// Enables verification caching.
    pub fn with_caching(mut self) -> Self {
        self.caching_enabled = true;
        self
    }
    /// Clears the verification cache.
    pub fn clear_cache(&self) {
        if let Ok(mut cache) = self.cache.lock() {
            cache.clear();
        }
    }
    /// Returns the number of cached results.
    pub fn cache_size(&self) -> usize {
        self.cache.lock().map(|c| c.len()).unwrap_or(0)
    }
    /// Generates a cache key for a statute.
    fn cache_key(statute: &Statute) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        statute.id.hash(&mut hasher);
        statute.preconditions.len().hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
    /// Returns default constitutional principles.
    fn default_principles() -> Vec<ConstitutionalPrinciple> {
        vec![
            ConstitutionalPrinciple {
                id: "equality".to_string(),
                name: "Equal Protection".to_string(),
                description: "All persons are equal under the law".to_string(),
                check: PrincipleCheck::NoDiscrimination,
            },
            ConstitutionalPrinciple {
                id: "due-process".to_string(),
                name: "Due Process".to_string(),
                description: "Fair procedures must be followed".to_string(),
                check: PrincipleCheck::RequiresProcedure,
            },
        ]
    }
    /// Verifies a set of statutes.
    pub fn verify(&self, statutes: &[Statute]) -> VerificationResult {
        let mut result = VerificationResult::pass();
        result.merge(self.check_circular_references(statutes));
        result.merge(self.check_dead_statutes(statutes));
        for statute in statutes {
            result.merge(
                self.verify_statute_cached(statute, |s| self.check_constitutional_compliance(s)),
            );
        }
        result.merge(self.check_contradictions(statutes));
        for statute in statutes {
            result
                .merge(self.verify_statute_cached(statute, |s| self.check_redundant_conditions(s)));
        }
        for statute in statutes {
            result.merge(self.verify_statute_cached(statute, |s| self.check_unreachable_code(s)));
        }
        result
    }
    /// Performs incremental verification, only re-checking changed statutes.
    ///
    /// This method uses an IncrementalState to track which statutes have changed
    /// and only re-verifies those statutes, reusing previous results for unchanged ones.
    pub fn verify_incremental(
        &self,
        statutes: &[Statute],
        state: &mut IncrementalState,
    ) -> VerificationResult {
        let mut result = VerificationResult::pass();
        let mut changed_statutes = Vec::new();
        let mut unchanged_statutes = Vec::new();
        for statute in statutes {
            if state.has_changed(statute) {
                changed_statutes.push(statute);
            } else {
                unchanged_statutes.push(statute);
            }
        }
        for statute in &unchanged_statutes {
            if let Some(prev_result) = state.get_previous_result(&statute.id) {
                result.merge(prev_result.clone());
            }
        }
        for statute in &changed_statutes {
            let statute_result = self.verify_single_statute(statute);
            state.update(statute, statute_result.clone());
            result.merge(statute_result);
        }
        if !changed_statutes.is_empty() {
            result.merge(self.check_circular_references(statutes));
            result.merge(self.check_contradictions(statutes));
        }
        result
    }
    /// Verifies a single statute in isolation.
    fn verify_single_statute(&self, statute: &Statute) -> VerificationResult {
        let mut result = VerificationResult::pass();
        result.merge(self.check_constitutional_compliance(statute));
        result.merge(self.check_redundant_conditions(statute));
        result.merge(self.check_unreachable_code(statute));
        if self.is_dead_statute(statute) {
            result.merge(VerificationResult::fail(vec![
                VerificationError::DeadStatute {
                    statute_id: statute.id.clone(),
                },
            ]));
        }
        result
    }
    /// Verifies a single statute in isolation (public wrapper).
    ///
    /// Runs the per-statute checks (constitutional compliance, redundant
    /// conditions, unreachable code, dead-statute detection) without any
    /// cross-statute analysis. This is the building block used by the
    /// incremental/streaming verifier in
    /// [`crate::streaming_verification`].
    pub fn verify_single(&self, statute: &Statute) -> VerificationResult {
        self.verify_single_statute(statute)
    }
    /// Verifies statutes with a specified budget.
    ///
    /// This method respects the verification budget and stops early if limits are reached.
    /// Returns a tuple of (result, statutes_verified, checks_performed, budget_exceeded).
    pub fn verify_with_budget(
        &self,
        statutes: &[Statute],
        budget: VerificationBudget,
    ) -> (VerificationResult, usize, usize, bool) {
        use std::time::Instant;
        let start_time = Instant::now();
        let mut result = VerificationResult::pass();
        let mut statutes_verified = 0;
        let mut checks_performed = 0;
        let mut budget_exceeded = false;
        let check_budget = |verified: usize, checks: usize, start: Instant| -> bool {
            if budget.statute_limit_reached(verified) {
                return true;
            }
            if budget.check_limit_reached(checks) {
                return true;
            }
            let elapsed = start.elapsed().as_millis() as u64;
            if budget.time_limit_reached(elapsed) {
                return true;
            }
            false
        };
        if check_budget(statutes_verified, checks_performed, start_time) {
            budget_exceeded = true;
            return (result, statutes_verified, checks_performed, budget_exceeded);
        }
        result.merge(self.check_circular_references(statutes));
        checks_performed += 1;
        if check_budget(statutes_verified, checks_performed, start_time) {
            budget_exceeded = true;
            return (result, statutes_verified, checks_performed, budget_exceeded);
        }
        result.merge(self.check_dead_statutes(statutes));
        checks_performed += 1;
        for statute in statutes {
            if check_budget(statutes_verified, checks_performed, start_time) {
                budget_exceeded = true;
                break;
            }
            result.merge(self.verify_single_statute(statute));
            statutes_verified += 1;
            checks_performed += 3;
        }
        if !check_budget(statutes_verified, checks_performed, start_time) {
            result.merge(self.check_contradictions(statutes));
            checks_performed += 1;
        } else {
            budget_exceeded = true;
        }
        (result, statutes_verified, checks_performed, budget_exceeded)
    }
    /// Verifies a set of statutes in parallel (requires 'parallel' feature).
    ///
    /// This method processes independent verification checks concurrently,
    /// which can significantly speed up verification for large statute sets.
    #[cfg(feature = "parallel")]
    pub fn verify_parallel(&self, statutes: &[Statute]) -> VerificationResult {
        use rayon::prelude::*;
        let mut result = VerificationResult::pass();
        result.merge(self.check_circular_references(statutes));
        result.merge(self.check_dead_statutes(statutes));
        let constitutional_results: Vec<_> = statutes
            .par_iter()
            .map(|statute| {
                self.verify_statute_cached(statute, |s| self.check_constitutional_compliance(s))
            })
            .collect();
        for res in constitutional_results {
            result.merge(res);
        }
        result.merge(self.check_contradictions(statutes));
        let redundancy_results: Vec<_> = statutes
            .par_iter()
            .map(|statute| {
                self.verify_statute_cached(statute, |s| self.check_redundant_conditions(s))
            })
            .collect();
        for res in redundancy_results {
            result.merge(res);
        }
        let unreachable_results: Vec<_> = statutes
            .par_iter()
            .map(|statute| self.verify_statute_cached(statute, |s| self.check_unreachable_code(s)))
            .collect();
        for res in unreachable_results {
            result.merge(res);
        }
        result
    }
    /// Verifies a single statute with caching support.
    fn verify_statute_cached<F>(&self, statute: &Statute, verify_fn: F) -> VerificationResult
    where
        F: FnOnce(&Statute) -> VerificationResult,
    {
        if !self.caching_enabled {
            return verify_fn(statute);
        }
        let key = Self::cache_key(statute);
        if let Ok(cache) = self.cache.lock()
            && let Some(cached_result) = cache.get(&key)
        {
            return cached_result.clone();
        }
        let result = verify_fn(statute);
        if let Ok(mut cache) = self.cache.lock() {
            cache.insert(key, result.clone());
        }
        result
    }
    /// Checks for circular references between statutes.
    fn check_circular_references(&self, statutes: &[Statute]) -> VerificationResult {
        let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();
        for statute in statutes {
            let deps = self.extract_statute_references(&statute.preconditions);
            graph.insert(&statute.id, deps);
        }
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut errors = Vec::new();
        let mut cycles_found = HashSet::new();
        for statute in statutes {
            if !visited.contains(statute.id.as_str())
                && let Some(cycle) = Self::find_cycle_path(
                    &statute.id,
                    &graph,
                    &mut visited,
                    &mut rec_stack,
                    &mut Vec::new(),
                )
            {
                let mut cycle_sorted = cycle.clone();
                cycle_sorted.sort();
                let cycle_key = cycle_sorted.join("->");
                if cycles_found.insert(cycle_key) {
                    errors.push(VerificationError::CircularReference {
                        message: format!(
                            "Circular reference detected: {} -> {}",
                            cycle.join(" -> "),
                            cycle[0]
                        ),
                    });
                }
            }
        }
        if errors.is_empty() {
            VerificationResult::pass()
        } else {
            VerificationResult::fail(errors)
        }
    }
    /// Extracts statute references from conditions.
    fn extract_statute_references<'a>(
        &self,
        conditions: &'a [legalis_core::Condition],
    ) -> HashSet<&'a str> {
        let mut refs = HashSet::new();
        for condition in conditions {
            Self::extract_refs_from_condition(condition, &mut refs);
        }
        refs
    }
    /// Recursively extracts references from a single condition.
    fn extract_refs_from_condition<'a>(
        condition: &'a legalis_core::Condition,
        refs: &mut HashSet<&'a str>,
    ) {
        use legalis_core::Condition;
        match condition {
            Condition::Custom { description } => {
                if let Some(statute_ref) = description.strip_prefix("statute:") {
                    refs.insert(statute_ref.trim());
                }
            }
            Condition::And(left, right) | Condition::Or(left, right) => {
                Self::extract_refs_from_condition(left, refs);
                Self::extract_refs_from_condition(right, refs);
            }
            Condition::Not(inner) => {
                Self::extract_refs_from_condition(inner, refs);
            }
            _ => {}
        }
    }
    /// Finds a cycle path in the graph, if one exists.
    fn find_cycle_path<'a>(
        node: &'a str,
        graph: &HashMap<&'a str, HashSet<&'a str>>,
        visited: &mut HashSet<&'a str>,
        rec_stack: &mut HashSet<&'a str>,
        path: &mut Vec<String>,
    ) -> Option<Vec<String>> {
        visited.insert(node);
        rec_stack.insert(node);
        path.push(node.to_string());
        if let Some(deps) = graph.get(node) {
            for dep in deps {
                if !visited.contains(dep) {
                    if let Some(cycle) = Self::find_cycle_path(dep, graph, visited, rec_stack, path)
                    {
                        return Some(cycle);
                    }
                } else if rec_stack.contains(dep) {
                    let cycle_start_idx = path
                        .iter()
                        .position(|p| p == dep)
                        .expect("invariant: rec_stack.contains(dep) guarantees dep is in path");
                    return Some(path[cycle_start_idx..].to_vec());
                }
            }
        }
        rec_stack.remove(node);
        path.pop();
        None
    }
    /// Checks for "dead" statutes that can never be satisfied.
    fn check_dead_statutes(&self, statutes: &[Statute]) -> VerificationResult {
        let mut result = VerificationResult::pass();
        for statute in statutes {
            if self.is_dead_statute(statute) {
                result.merge(VerificationResult::fail(vec![
                    VerificationError::DeadStatute {
                        statute_id: statute.id.clone(),
                    },
                ]));
            }
        }
        result
    }
    fn is_dead_statute(&self, statute: &Statute) -> bool {
        if statute.preconditions.is_empty() {
            return false;
        }
        #[cfg(feature = "smt-solver")]
        {
            let mut smt_verifier = smt::SmtVerifier::new();
            let mut combined = statute.preconditions[0].clone();
            for condition in &statute.preconditions[1..] {
                combined =
                    legalis_core::Condition::And(Box::new(combined), Box::new(condition.clone()));
            }
            if let Ok(satisfiable) = smt_verifier.is_satisfiable(&combined) {
                return !satisfiable;
            }
        }
        for i in 0..statute.preconditions.len() {
            for j in (i + 1)..statute.preconditions.len() {
                if self.conditions_contradict(&statute.preconditions[i], &statute.preconditions[j])
                {
                    return true;
                }
            }
        }
        false
    }
    fn conditions_contradict(
        &self,
        cond1: &legalis_core::Condition,
        cond2: &legalis_core::Condition,
    ) -> bool {
        #[cfg(feature = "smt-solver")]
        {
            let mut smt_verifier = smt::SmtVerifier::new();
            if let Ok(contradicts) = smt_verifier.contradict(cond1, cond2) {
                return contradicts;
            }
        }
        #[cfg(not(feature = "smt-solver"))]
        {
            let _ = (cond1, cond2);
        }
        false
    }
    /// Checks constitutional compliance.
    fn check_constitutional_compliance(&self, statute: &Statute) -> VerificationResult {
        let mut result = VerificationResult::pass();
        for principle in &self.constitutional_principles {
            if !self.complies_with_principle(statute, principle) {
                result.merge(VerificationResult::fail(vec![
                    VerificationError::ConstitutionalConflict {
                        statute_id: statute.id.clone(),
                        principle: principle.name.clone(),
                    },
                ]));
            }
        }
        if statute.discretion_logic.is_some() {
            result = result.with_warning(format!(
                "Statute '{}' contains discretionary elements that require human review",
                statute.id
            ));
        }
        result
    }
    fn complies_with_principle(
        &self,
        _statute: &Statute,
        _principle: &ConstitutionalPrinciple,
    ) -> bool {
        true
    }
    /// Checks for logical contradictions between statutes.
    fn check_contradictions(&self, statutes: &[Statute]) -> VerificationResult {
        let mut result = VerificationResult::pass();
        for i in 0..statutes.len() {
            for j in (i + 1)..statutes.len() {
                if self.statutes_contradict(&statutes[i], &statutes[j]) {
                    result.merge(VerificationResult::fail(vec![
                        VerificationError::LogicalContradiction {
                            message: format!(
                                "Statutes '{}' and '{}' have contradictory effects",
                                statutes[i].id, statutes[j].id
                            ),
                        },
                    ]));
                }
            }
        }
        result
    }
    fn statutes_contradict(&self, statute1: &Statute, statute2: &Statute) -> bool {
        if statute1.preconditions.is_empty() || statute2.preconditions.is_empty() {
            return false;
        }
        #[cfg(feature = "smt-solver")]
        {
            let mut smt_verifier = smt::SmtVerifier::new();
            let mut combined1 = statute1.preconditions[0].clone();
            for condition in &statute1.preconditions[1..] {
                combined1 =
                    legalis_core::Condition::And(Box::new(combined1), Box::new(condition.clone()));
            }
            let mut combined2 = statute2.preconditions[0].clone();
            for condition in &statute2.preconditions[1..] {
                combined2 =
                    legalis_core::Condition::And(Box::new(combined2), Box::new(condition.clone()));
            }
            match smt_verifier.contradict(&combined1, &combined2) {
                Ok(true) => return false,
                Ok(false) => {
                    use legalis_core::EffectType;
                    return matches!(
                        (&statute1.effect.effect_type, &statute2.effect.effect_type),
                        (EffectType::Grant, EffectType::Revoke)
                            | (EffectType::Revoke, EffectType::Grant)
                            | (EffectType::Obligation, EffectType::Prohibition)
                            | (EffectType::Prohibition, EffectType::Obligation)
                    );
                }
                Err(_) => {}
            }
        }
        false
    }
    /// Checks for redundant conditions within a statute.
    ///
    /// A condition is redundant if it's always implied by another condition.
    /// For example, "age >= 21" makes "age >= 18" redundant.
    fn check_redundant_conditions(&self, statute: &Statute) -> VerificationResult {
        if statute.preconditions.len() < 2 {
            return VerificationResult::pass();
        }
        #[cfg(feature = "smt-solver")]
        {
            use crate::smt;
            let mut smt_verifier = smt::SmtVerifier::new();
            for i in 0..statute.preconditions.len() {
                for j in 0..statute.preconditions.len() {
                    if i == j {
                        continue;
                    }
                    if let Ok(implies) =
                        smt_verifier.implies(&statute.preconditions[i], &statute.preconditions[j])
                        && implies
                    {
                        let suggestion = format!(
                            "In statute '{}': condition '{}' is redundant (implied by '{}')",
                            statute.id,
                            format!("{:?}", statute.preconditions[j]),
                            format!("{:?}", statute.preconditions[i])
                        );
                        return VerificationResult::pass().with_suggestion(suggestion);
                    }
                }
            }
        }
        VerificationResult::pass()
    }
    /// Checks for unreachable code (dead branches) in conditions.
    ///
    /// Detects conditions that can never be satisfied, making parts of the logic unreachable.
    fn check_unreachable_code(&self, statute: &Statute) -> VerificationResult {
        let mut result = VerificationResult::pass();
        for (idx, condition) in statute.preconditions.iter().enumerate() {
            if let Some(message) = self.find_unreachable_branch(condition) {
                result = result.with_warning(format!(
                    "In statute '{}', precondition {}: {}",
                    statute.id,
                    idx + 1,
                    message
                ));
            }
        }
        result
    }
    /// Recursively finds unreachable branches in a condition tree.
    fn find_unreachable_branch(&self, condition: &legalis_core::Condition) -> Option<String> {
        #[cfg(feature = "smt-solver")]
        {
            use crate::smt;
            use legalis_core::Condition;
            let mut smt_verifier = smt::SmtVerifier::new();
            if let Ok(satisfiable) = smt_verifier.is_satisfiable(condition)
                && !satisfiable
            {
                return Some(format!(
                    "Unreachable branch: condition {:?} can never be satisfied",
                    condition
                ));
            }
            match condition {
                Condition::Or(left, right) => {
                    if let Ok(left_sat) = smt_verifier.is_satisfiable(left)
                        && !left_sat
                    {
                        return Some(
                            "Left branch of OR is always false, making it redundant".to_string(),
                        );
                    }
                    if let Ok(right_sat) = smt_verifier.is_satisfiable(right)
                        && !right_sat
                    {
                        return Some(
                            "Right branch of OR is always false, making it redundant".to_string(),
                        );
                    }
                    if let Some(msg) = self.find_unreachable_branch(left) {
                        return Some(msg);
                    }
                    if let Some(msg) = self.find_unreachable_branch(right) {
                        return Some(msg);
                    }
                }
                Condition::And(left, right) => {
                    if let Some(msg) = self.find_unreachable_branch(left) {
                        return Some(msg);
                    }
                    if let Some(msg) = self.find_unreachable_branch(right) {
                        return Some(msg);
                    }
                }
                Condition::Not(inner) => {
                    if let Ok(is_tautology) = smt_verifier.is_tautology(inner)
                        && is_tautology
                    {
                        return Some("NOT of a tautology is always false".to_string());
                    }
                    if let Some(msg) = self.find_unreachable_branch(inner) {
                        return Some(msg);
                    }
                }
                _ => {}
            }
        }
        #[cfg(not(feature = "smt-solver"))]
        {
            let _ = condition;
        }
        None
    }
}
/// A complete verification proof
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VerificationProof {
    /// Statute being verified
    pub statute_id: String,
    /// What is being proven
    pub claim: String,
    /// The proof steps
    pub steps: Vec<ProofStep>,
    /// Whether the proof is complete
    pub is_complete: bool,
    /// Timestamp when proof was generated
    pub generated_at: String,
}
impl VerificationProof {
    /// Creates a new empty proof
    pub fn new(statute_id: impl Into<String>, claim: impl Into<String>) -> Self {
        Self {
            statute_id: statute_id.into(),
            claim: claim.into(),
            steps: Vec::new(),
            is_complete: false,
            generated_at: chrono::Utc::now().to_rfc3339(),
        }
    }
    /// Adds a proof step
    pub fn add_step(&mut self, step: ProofStep) {
        self.steps.push(step);
    }
    /// Marks the proof as complete
    pub fn complete(mut self) -> Self {
        self.is_complete = true;
        self
    }
    /// Generates a human-readable proof text
    pub fn to_human_readable(&self) -> String {
        let mut output = String::new();
        output.push_str("# Verification Proof\n\n");
        output.push_str(&format!("**Statute**: {}\n", self.statute_id));
        output.push_str(&format!("**Claim**: {}\n", self.claim));
        output.push_str(&format!("**Generated**: {}\n", self.generated_at));
        output.push_str(&format!(
            "**Status**: {}\n\n",
            if self.is_complete {
                "Complete"
            } else {
                "Incomplete"
            }
        ));
        output.push_str("## Proof Steps\n\n");
        for step in &self.steps {
            output.push_str(&format!(
                "### Step {} - {}\n\n",
                step.step_number, step.step_type
            ));
            output.push_str(&format!("**Description**: {}\n\n", step.description));
            output.push_str(&format!("**Formula**: `{}`\n\n", step.formula));
            output.push_str(&format!("**Justification**: {}\n\n", step.justification));
            if !step.depends_on.is_empty() {
                output.push_str(&format!(
                    "**Depends on steps**: {}\n\n",
                    step.depends_on
                        .iter()
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            }
        }
        if self.is_complete {
            output.push_str(
                "## Conclusion\n\nThe proof is complete and the claim has been verified.\n",
            );
        }
        output
    }
}
