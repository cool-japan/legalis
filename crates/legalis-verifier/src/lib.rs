//! Legalis-Verifier: Formal verification for Legalis-RS legal statutes.
//!
//! This crate provides static analysis and verification tools for detecting
//! logical inconsistencies, circular references, and constitutional conflicts
//! in legal statutes.

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
    ConstitutionalConflict { statute_id: String, principle: String },

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
            if self.has_cycle(&statute.id, &graph, &mut visited, &mut rec_stack) {
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
        &self,
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
                if self.has_cycle(dep, graph, visited, rec_stack) {
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
                result.merge(VerificationResult::fail(vec![VerificationError::DeadStatute {
                    statute_id: statute.id.clone(),
                }]));
            }
        }

        result
    }

    fn is_dead_statute(&self, statute: &Statute) -> bool {
        // Check for contradictory conditions
        // In a real implementation, this would use an SMT solver
        for i in 0..statute.preconditions.len() {
            for j in (i + 1)..statute.preconditions.len() {
                if self.conditions_contradict(&statute.preconditions[i], &statute.preconditions[j]) {
                    return true;
                }
            }
        }
        false
    }

    fn conditions_contradict(
        &self,
        _cond1: &legalis_core::Condition,
        _cond2: &legalis_core::Condition,
    ) -> bool {
        // Simplified contradiction check
        // In a real implementation, this would use proper logical analysis
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

    fn complies_with_principle(&self, _statute: &Statute, _principle: &ConstitutionalPrinciple) -> bool {
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

    fn statutes_contradict(&self, _statute1: &Statute, _statute2: &Statute) -> bool {
        // Simplified contradiction check
        // In a real implementation, this would compare effects and conditions
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
}
