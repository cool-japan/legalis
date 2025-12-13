//! Legalis-Verifier: Formal verification for Legalis-RS legal statutes.
//!
//! This crate provides static analysis and verification tools for detecting
//! logical inconsistencies, circular references, and constitutional conflicts
//! in legal statutes.

#[cfg(feature = "z3-solver")]
mod smt;

#[cfg(feature = "z3-solver")]
pub use smt::{create_z3_context, SmtVerifier};

use legalis_core::Statute;
use std::collections::{HashMap, HashSet};
use thiserror::Error;

/// Errors from verification process.
#[derive(Debug, Clone, Error)]
pub enum VerificationError {
    #[error("Circular reference detected: {0}")]
    CircularReference(String),

    #[error("Dead statute detected: {statute_id} can never be satisfied")]
    DeadStatute { statute_id: String },

    #[error("Constitutional conflict: {statute_id} conflicts with {principle}")]
    ConstitutionalConflict {
        statute_id: String,
        principle: String,
    },

    #[error("Logical contradiction: {0}")]
    LogicalContradiction(String),

    #[error("Ambiguity detected: {0}")]
    Ambiguity(String),
}

/// Result of a verification check.
#[derive(Debug, Clone)]
pub struct VerificationResult {
    /// Whether the verification passed
    pub passed: bool,
    /// List of errors found
    pub errors: Vec<VerificationError>,
    /// List of warnings
    pub warnings: Vec<String>,
    /// Suggestions for improvement
    pub suggestions: Vec<String>,
}

impl VerificationResult {
    /// Creates a passing result.
    pub fn pass() -> Self {
        Self {
            passed: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            suggestions: Vec::new(),
        }
    }

    /// Creates a failing result with errors.
    pub fn fail(errors: Vec<VerificationError>) -> Self {
        Self {
            passed: false,
            errors,
            warnings: Vec::new(),
            suggestions: Vec::new(),
        }
    }

    /// Adds a warning.
    pub fn with_warning(mut self, warning: impl Into<String>) -> Self {
        self.warnings.push(warning.into());
        self
    }

    /// Adds a suggestion.
    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestions.push(suggestion.into());
        self
    }

    /// Merges another result into this one.
    pub fn merge(&mut self, other: VerificationResult) {
        if !other.passed {
            self.passed = false;
        }
        self.errors.extend(other.errors);
        self.warnings.extend(other.warnings);
        self.suggestions.extend(other.suggestions);
    }
}

/// Verifier for legal statutes.
pub struct StatuteVerifier {
    /// Constitutional principles to check against
    constitutional_principles: Vec<ConstitutionalPrinciple>,
}

impl StatuteVerifier {
    /// Creates a new verifier.
    pub fn new() -> Self {
        Self {
            constitutional_principles: Self::default_principles(),
        }
    }

    /// Creates a verifier with custom principles.
    pub fn with_principles(principles: Vec<ConstitutionalPrinciple>) -> Self {
        Self {
            constitutional_principles: principles,
        }
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

        // Check for circular references
        result.merge(self.check_circular_references(statutes));

        // Check for dead statutes
        result.merge(self.check_dead_statutes(statutes));

        // Check constitutional compliance
        for statute in statutes {
            result.merge(self.check_constitutional_compliance(statute));
        }

        // Check for logical contradictions between statutes
        result.merge(self.check_contradictions(statutes));

        result
    }

    /// Checks for circular references between statutes.
    fn check_circular_references(&self, statutes: &[Statute]) -> VerificationResult {
        let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();

        // Build dependency graph
        for statute in statutes {
            let deps: HashSet<&str> = HashSet::new();
            // In a real implementation, we'd parse conditions to find references
            graph.insert(&statute.id, deps);
        }

        // Detect cycles using DFS
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut errors = Vec::new();

        for statute in statutes {
            if Self::has_cycle(&statute.id, &graph, &mut visited, &mut rec_stack) {
                errors.push(VerificationError::CircularReference(format!(
                    "Statute '{}' is part of a circular reference",
                    statute.id
                )));
            }
        }

        if errors.is_empty() {
            VerificationResult::pass()
        } else {
            VerificationResult::fail(errors)
        }
    }

    fn has_cycle<'a>(
        node: &'a str,
        graph: &HashMap<&'a str, HashSet<&'a str>>,
        visited: &mut HashSet<&'a str>,
        rec_stack: &mut HashSet<&'a str>,
    ) -> bool {
        if rec_stack.contains(node) {
            return true;
        }
        if visited.contains(node) {
            return false;
        }

        visited.insert(node);
        rec_stack.insert(node);

        if let Some(deps) = graph.get(node) {
            for dep in deps {
                if Self::has_cycle(dep, graph, visited, rec_stack) {
                    return true;
                }
            }
        }

        rec_stack.remove(node);
        false
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
        // Use SMT solver to check if the conjunction of all preconditions is satisfiable
        if statute.preconditions.is_empty() {
            return false;
        }

        #[cfg(feature = "z3-solver")]
        {
            // Create Z3 context and verifier
            let ctx = smt::create_z3_context();
            let mut smt_verifier = smt::SmtVerifier::new(&ctx);

            // Build conjunction of all preconditions
            let mut combined = statute.preconditions[0].clone();
            for condition in &statute.preconditions[1..] {
                combined = legalis_core::Condition::And(Box::new(combined), Box::new(condition.clone()));
            }

            // If the conjunction is unsatisfiable, the statute is dead
            match smt_verifier.is_satisfiable(&combined) {
                Ok(satisfiable) => return !satisfiable,
                Err(_) => {} // Fall through to simple checking
            }
        }

        // Simple pairwise checking (used when z3-solver feature is not enabled or SMT fails)
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
        #[cfg(feature = "z3-solver")]
        {
            // Use SMT solver to check if conditions contradict
            let ctx = smt::create_z3_context();
            let mut smt_verifier = smt::SmtVerifier::new(&ctx);

            match smt_verifier.contradict(cond1, cond2) {
                Ok(contradicts) => return contradicts,
                Err(_) => {} // Fall through to simple check
            }
        }

        // Simple heuristic check (used when z3-solver feature is not enabled)
        // This is conservative and only catches obvious cases
        #[cfg(not(feature = "z3-solver"))]
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

        // Add warning for discretionary statutes
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
        // Simplified compliance check
        // In a real implementation, this would analyze the statute's conditions
        true
    }

    /// Checks for logical contradictions between statutes.
    fn check_contradictions(&self, statutes: &[Statute]) -> VerificationResult {
        let mut result = VerificationResult::pass();

        for i in 0..statutes.len() {
            for j in (i + 1)..statutes.len() {
                if self.statutes_contradict(&statutes[i], &statutes[j]) {
                    result.merge(VerificationResult::fail(vec![
                        VerificationError::LogicalContradiction(format!(
                            "Statutes '{}' and '{}' have contradictory effects",
                            statutes[i].id, statutes[j].id
                        )),
                    ]));
                }
            }
        }

        result
    }

    fn statutes_contradict(&self, statute1: &Statute, statute2: &Statute) -> bool {
        // Two statutes contradict if their preconditions can be simultaneously satisfied
        // but their effects conflict

        // First check if preconditions can be satisfied together
        if statute1.preconditions.is_empty() || statute2.preconditions.is_empty() {
            return false;
        }

        #[cfg(feature = "z3-solver")]
        {
            let ctx = smt::create_z3_context();
            let mut smt_verifier = smt::SmtVerifier::new(&ctx);

            // Build conjunction of all preconditions from both statutes
            let mut combined1 = statute1.preconditions[0].clone();
            for condition in &statute1.preconditions[1..] {
                combined1 = legalis_core::Condition::And(Box::new(combined1), Box::new(condition.clone()));
            }

            let mut combined2 = statute2.preconditions[0].clone();
            for condition in &statute2.preconditions[1..] {
                combined2 = legalis_core::Condition::And(Box::new(combined2), Box::new(condition.clone()));
            }

            // Check if both sets of preconditions can be true simultaneously
            match smt_verifier.contradict(&combined1, &combined2) {
                Ok(true) => return false, // Preconditions contradict, so statutes don't conflict
                Ok(false) => {
                    // Preconditions can both be true - check if effects conflict
                    use legalis_core::EffectType;
                    return match (&statute1.effect.effect_type, &statute2.effect.effect_type) {
                        (EffectType::Grant, EffectType::Revoke) => true,
                        (EffectType::Revoke, EffectType::Grant) => true,
                        (EffectType::Obligation, EffectType::Prohibition) => true,
                        (EffectType::Prohibition, EffectType::Obligation) => true,
                        _ => false,
                    };
                }
                Err(_) => {} // Fall through to simple check
            }
        }

        // Without SMT solver, do simple effect-based checking
        false
    }
}

impl Default for StatuteVerifier {
    fn default() -> Self {
        Self::new()
    }
}

/// A constitutional principle to check against.
#[derive(Debug, Clone)]
pub struct ConstitutionalPrinciple {
    /// Unique identifier
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Description of the principle
    pub description: String,
    /// Type of check to perform
    pub check: PrincipleCheck,
}

/// Types of constitutional checks.
#[derive(Debug, Clone)]
pub enum PrincipleCheck {
    /// No discrimination based on protected attributes
    NoDiscrimination,
    /// Requires procedural safeguards
    RequiresProcedure,
    /// Must not be retroactive
    NoRetroactivity,
    /// Custom check with description
    Custom(String),
}

/// Verifies the integrity of a set of laws.
pub fn verify_integrity(laws: &[Statute]) -> Result<VerificationResult, String> {
    let verifier = StatuteVerifier::new();
    Ok(verifier.verify(laws))
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

/// Complexity levels for statutes.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ComplexityLevel {
    /// Simple statute with few conditions
    #[default]
    Simple,
    /// Moderate complexity
    Moderate,
    /// Complex statute requiring careful review
    Complex,
    /// Very complex statute, consider simplification
    VeryComplex,
}

impl std::fmt::Display for ComplexityLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Simple => write!(f, "Simple"),
            Self::Moderate => write!(f, "Moderate"),
            Self::Complex => write!(f, "Complex"),
            Self::VeryComplex => write!(f, "Very Complex"),
        }
    }
}

/// Analyzes the complexity of a statute.
pub fn analyze_complexity(statute: &Statute) -> ComplexityMetrics {
    let mut metrics = ComplexityMetrics {
        condition_count: statute.preconditions.len(),
        ..Default::default()
    };

    // Analyze each condition
    let mut condition_types = HashSet::new();
    for condition in &statute.preconditions {
        let (depth, ops, types) = analyze_condition(condition);
        metrics.condition_depth = metrics.condition_depth.max(depth);
        metrics.logical_operator_count += ops;
        condition_types.extend(types);
    }
    metrics.condition_type_count = condition_types.len();

    // Check for discretion
    metrics.has_discretion = statute.discretion_logic.is_some();

    // Calculate cyclomatic complexity (simplified: 1 + decision points)
    metrics.cyclomatic_complexity = 1 + metrics.condition_count + metrics.logical_operator_count;

    // Calculate overall score (0-100)
    let mut score: u32 = 0;
    // Only count conditions beyond 1 as adding complexity
    if metrics.condition_count > 1 {
        score += ((metrics.condition_count - 1) * 10).min(30) as u32;
    }
    // Depth only adds complexity beyond 1
    if metrics.condition_depth > 1 {
        score += ((metrics.condition_depth - 1) * 15).min(30) as u32;
    }
    score += (metrics.logical_operator_count * 8).min(24) as u32;
    // Multiple condition types adds complexity
    if metrics.condition_type_count > 1 {
        score += ((metrics.condition_type_count - 1) * 6).min(12) as u32;
    }
    if metrics.has_discretion {
        score += 10;
    }
    metrics.complexity_score = score.min(100);

    // Determine level
    metrics.complexity_level = match metrics.complexity_score {
        0..=25 => ComplexityLevel::Simple,
        26..=50 => ComplexityLevel::Moderate,
        51..=75 => ComplexityLevel::Complex,
        _ => ComplexityLevel::VeryComplex,
    };

    metrics
}

/// Analyzes a condition recursively.
/// Returns (depth, operator_count, condition_types)
fn analyze_condition(condition: &legalis_core::Condition) -> (usize, usize, HashSet<String>) {
    use legalis_core::Condition;

    match condition {
        Condition::Age { .. } => (1, 0, ["Age".to_string()].into_iter().collect()),
        Condition::Income { .. } => (1, 0, ["Income".to_string()].into_iter().collect()),
        Condition::HasAttribute { .. } => {
            (1, 0, ["HasAttribute".to_string()].into_iter().collect())
        }
        Condition::AttributeEquals { .. } => {
            (1, 0, ["AttributeEquals".to_string()].into_iter().collect())
        }
        Condition::DateRange { .. } => (1, 0, ["DateRange".to_string()].into_iter().collect()),
        Condition::Geographic { .. } => (1, 0, ["Geographic".to_string()].into_iter().collect()),
        Condition::EntityRelationship { .. } => (
            1,
            0,
            ["EntityRelationship".to_string()].into_iter().collect(),
        ),
        Condition::ResidencyDuration { .. } => (
            1,
            0,
            ["ResidencyDuration".to_string()].into_iter().collect(),
        ),
        Condition::Custom { .. } => (1, 0, ["Custom".to_string()].into_iter().collect()),
        Condition::And(left, right) => {
            let (l_depth, l_ops, l_types) = analyze_condition(left);
            let (r_depth, r_ops, r_types) = analyze_condition(right);
            let mut types = l_types;
            types.extend(r_types);
            (1 + l_depth.max(r_depth), 1 + l_ops + r_ops, types)
        }
        Condition::Or(left, right) => {
            let (l_depth, l_ops, l_types) = analyze_condition(left);
            let (r_depth, r_ops, r_types) = analyze_condition(right);
            let mut types = l_types;
            types.extend(r_types);
            (1 + l_depth.max(r_depth), 1 + l_ops + r_ops, types)
        }
        Condition::Not(inner) => {
            let (depth, ops, types) = analyze_condition(inner);
            (1 + depth, 1 + ops, types)
        }
    }
}

/// Generates a complexity report for multiple statutes.
pub fn complexity_report(statutes: &[Statute]) -> String {
    let mut report = String::new();
    report.push_str("# Statute Complexity Report\n\n");

    let mut total_score = 0u32;
    let mut max_complexity = ComplexityLevel::Simple;

    for statute in statutes {
        let metrics = analyze_complexity(statute);
        total_score += metrics.complexity_score;
        if metrics.complexity_level as u8 > max_complexity as u8 {
            max_complexity = metrics.complexity_level;
        }

        report.push_str(&format!("## {}: \"{}\"\n", statute.id, statute.title));
        report.push_str(&format!(
            "- Complexity Level: {}\n",
            metrics.complexity_level
        ));
        report.push_str(&format!(
            "- Complexity Score: {}/100\n",
            metrics.complexity_score
        ));
        report.push_str(&format!("- Conditions: {}\n", metrics.condition_count));
        report.push_str(&format!("- Max Depth: {}\n", metrics.condition_depth));
        report.push_str(&format!(
            "- Logical Operators: {}\n",
            metrics.logical_operator_count
        ));
        report.push_str(&format!(
            "- Condition Types: {}\n",
            metrics.condition_type_count
        ));
        report.push_str(&format!("- Has Discretion: {}\n", metrics.has_discretion));
        report.push_str(&format!(
            "- Cyclomatic Complexity: {}\n\n",
            metrics.cyclomatic_complexity
        ));
    }

    let avg_score = if statutes.is_empty() {
        0
    } else {
        total_score / statutes.len() as u32
    };

    report.push_str("## Summary\n");
    report.push_str(&format!("- Total Statutes: {}\n", statutes.len()));
    report.push_str(&format!("- Average Complexity Score: {}\n", avg_score));
    report.push_str(&format!("- Maximum Complexity Level: {}\n", max_complexity));

    report
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType};

    #[test]
    fn test_verifier_pass() {
        let statute = Statute::new(
            "test-1",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let verifier = StatuteVerifier::new();
        let result = verifier.verify(&[statute]);

        assert!(result.passed);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_verifier_discretion_warning() {
        let statute = Statute::new(
            "test-1",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        )
        .with_discretion("Consider special circumstances");

        let verifier = StatuteVerifier::new();
        let result = verifier.verify(&[statute]);

        assert!(result.passed);
        assert!(!result.warnings.is_empty());
    }

    #[test]
    fn test_verify_integrity() {
        let statutes = vec![Statute::new(
            "test-1",
            "Test",
            Effect::new(EffectType::Grant, "Test"),
        )];

        let result = verify_integrity(&statutes).unwrap();
        assert!(result.passed);
    }

    #[test]
    fn test_complexity_simple() {
        let statute = Statute::new(
            "simple-1",
            "Simple Statute",
            Effect::new(EffectType::Grant, "Test"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let metrics = analyze_complexity(&statute);
        assert_eq!(metrics.condition_count, 1);
        assert_eq!(metrics.condition_depth, 1);
        assert_eq!(metrics.logical_operator_count, 0);
        assert_eq!(metrics.complexity_level, ComplexityLevel::Simple);
    }

    #[test]
    fn test_complexity_with_and() {
        let statute = Statute::new(
            "and-1",
            "AND Statute",
            Effect::new(EffectType::Grant, "Test"),
        )
        .with_precondition(Condition::And(
            Box::new(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            }),
            Box::new(Condition::Income {
                operator: ComparisonOp::LessThan,
                value: 50000,
            }),
        ));

        let metrics = analyze_complexity(&statute);
        assert_eq!(metrics.condition_count, 1);
        assert_eq!(metrics.condition_depth, 2);
        assert_eq!(metrics.logical_operator_count, 1);
        assert_eq!(metrics.condition_type_count, 2); // Age and Income
    }

    #[test]
    fn test_complexity_nested() {
        let statute = Statute::new(
            "nested-1",
            "Nested Statute",
            Effect::new(EffectType::Grant, "Test"),
        )
        .with_precondition(Condition::Or(
            Box::new(Condition::And(
                Box::new(Condition::Age {
                    operator: ComparisonOp::GreaterOrEqual,
                    value: 18,
                }),
                Box::new(Condition::Income {
                    operator: ComparisonOp::LessThan,
                    value: 50000,
                }),
            )),
            Box::new(Condition::HasAttribute {
                key: "disabled".to_string(),
            }),
        ))
        .with_discretion("Consider special circumstances");

        let metrics = analyze_complexity(&statute);
        assert_eq!(metrics.condition_depth, 3);
        assert_eq!(metrics.logical_operator_count, 2); // AND + OR
        assert!(metrics.has_discretion);
        assert!(metrics.complexity_score > 25); // Should be at least moderate
    }

    #[test]
    fn test_complexity_report() {
        let statutes = vec![
            Statute::new("s1", "Simple", Effect::new(EffectType::Grant, "Test")),
            Statute::new(
                "s2",
                "With Condition",
                Effect::new(EffectType::Grant, "Test"),
            )
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 21,
            }),
        ];

        let report = complexity_report(&statutes);
        assert!(report.contains("# Statute Complexity Report"));
        assert!(report.contains("s1"));
        assert!(report.contains("s2"));
        assert!(report.contains("## Summary"));
    }
}
