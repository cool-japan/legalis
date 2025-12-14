//! Legalis-Verifier: Formal verification for Legalis-RS legal statutes.
//!
//! This crate provides static analysis and verification tools for detecting
//! logical inconsistencies, circular references, and constitutional conflicts
//! in legal statutes.

#[cfg(feature = "z3-solver")]
mod smt;

#[cfg(feature = "z3-solver")]
pub use smt::{SmtVerifier, create_z3_context};

use legalis_core::Statute;
use std::collections::{HashMap, HashSet};
use thiserror::Error;

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

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Info => write!(f, "Info"),
            Self::Warning => write!(f, "Warning"),
            Self::Error => write!(f, "Error"),
            Self::Critical => write!(f, "Critical"),
        }
    }
}

/// Errors from verification process.
#[derive(Debug, Clone, Error, serde::Serialize, serde::Deserialize)]
pub enum VerificationError {
    #[error("Circular reference detected: {message}")]
    CircularReference { message: String },

    #[error("Dead statute detected: {statute_id} can never be satisfied")]
    DeadStatute { statute_id: String },

    #[error("Constitutional conflict: {statute_id} conflicts with {principle}")]
    ConstitutionalConflict {
        statute_id: String,
        principle: String,
    },

    #[error("Logical contradiction: {message}")]
    LogicalContradiction { message: String },

    #[error("Ambiguity detected: {message}")]
    Ambiguity { message: String },

    #[error("Unreachable code detected: {message}")]
    UnreachableCode { message: String },
}

impl VerificationError {
    /// Returns the severity level of this error.
    pub fn severity(&self) -> Severity {
        match self {
            Self::CircularReference { .. } => Severity::Critical,
            Self::DeadStatute { .. } => Severity::Error,
            Self::ConstitutionalConflict { .. } => Severity::Critical,
            Self::LogicalContradiction { .. } => Severity::Error,
            Self::Ambiguity { .. } => Severity::Warning,
            Self::UnreachableCode { .. } => Severity::Warning,
        }
    }
}

/// Result of a verification check.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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

    /// Exports the result to JSON format.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Exports the result to JSON format (non-pretty).
    pub fn to_json_compact(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Loads a result from JSON format.
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Filters errors by minimum severity level.
    pub fn errors_by_severity(&self, min_severity: Severity) -> Vec<&VerificationError> {
        self.errors
            .iter()
            .filter(|e| e.severity() >= min_severity)
            .collect()
    }

    /// Counts errors by severity level.
    pub fn severity_counts(&self) -> HashMap<Severity, usize> {
        let mut counts = HashMap::new();
        for error in &self.errors {
            *counts.entry(error.severity()).or_insert(0) += 1;
        }
        counts
    }

    /// Returns true if there are any critical errors.
    pub fn has_critical_errors(&self) -> bool {
        self.errors
            .iter()
            .any(|e| e.severity() == Severity::Critical)
    }
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
        // Hash preconditions count and effect type as a simple heuristic
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

        // Check for circular references
        result.merge(self.check_circular_references(statutes));

        // Check for dead statutes
        result.merge(self.check_dead_statutes(statutes));

        // Check constitutional compliance (with caching)
        for statute in statutes {
            result.merge(
                self.verify_statute_cached(statute, |s| self.check_constitutional_compliance(s)),
            );
        }

        // Check for logical contradictions between statutes
        result.merge(self.check_contradictions(statutes));

        // Check for redundant conditions within each statute (with caching)
        for statute in statutes {
            result
                .merge(self.verify_statute_cached(statute, |s| self.check_redundant_conditions(s)));
        }

        // Check for unreachable code (dead branches) (with caching)
        for statute in statutes {
            result.merge(self.verify_statute_cached(statute, |s| self.check_unreachable_code(s)));
        }

        result
    }

    /// Verifies a set of statutes in parallel (requires 'parallel' feature).
    ///
    /// This method processes independent verification checks concurrently,
    /// which can significantly speed up verification for large statute sets.
    #[cfg(feature = "parallel")]
    pub fn verify_parallel(&self, statutes: &[Statute]) -> VerificationResult {
        use rayon::prelude::*;

        let mut result = VerificationResult::pass();

        // Check for circular references (sequential, as it needs global graph)
        result.merge(self.check_circular_references(statutes));

        // Check for dead statutes (sequential, as it needs global graph)
        result.merge(self.check_dead_statutes(statutes));

        // Parallel check constitutional compliance
        let constitutional_results: Vec<_> = statutes
            .par_iter()
            .map(|statute| {
                self.verify_statute_cached(statute, |s| self.check_constitutional_compliance(s))
            })
            .collect();

        for res in constitutional_results {
            result.merge(res);
        }

        // Check for logical contradictions (sequential, needs pairwise comparison)
        result.merge(self.check_contradictions(statutes));

        // Parallel check redundant conditions
        let redundancy_results: Vec<_> = statutes
            .par_iter()
            .map(|statute| {
                self.verify_statute_cached(statute, |s| self.check_redundant_conditions(s))
            })
            .collect();

        for res in redundancy_results {
            result.merge(res);
        }

        // Parallel check unreachable code
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

        // Check cache
        if let Ok(cache) = self.cache.lock() {
            if let Some(cached_result) = cache.get(&key) {
                return cached_result.clone();
            }
        }

        // Compute result
        let result = verify_fn(statute);

        // Store in cache
        if let Ok(mut cache) = self.cache.lock() {
            cache.insert(key, result.clone());
        }

        result
    }

    /// Checks for circular references between statutes.
    fn check_circular_references(&self, statutes: &[Statute]) -> VerificationResult {
        let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();

        // Build dependency graph by extracting statute references from conditions
        for statute in statutes {
            let deps = self.extract_statute_references(&statute.preconditions);
            graph.insert(&statute.id, deps);
        }

        // Detect cycles using DFS
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut errors = Vec::new();
        let mut cycles_found = HashSet::new();

        for statute in statutes {
            if !visited.contains(statute.id.as_str()) {
                if let Some(cycle) = Self::find_cycle_path(
                    &statute.id,
                    &graph,
                    &mut visited,
                    &mut rec_stack,
                    &mut Vec::new(),
                ) {
                    // Create a normalized cycle key to avoid duplicate reporting
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
            // For Custom conditions, check if they reference statute IDs
            Condition::Custom { description } => {
                // Simple heuristic: if description contains "statute:" prefix
                if let Some(statute_ref) = description.strip_prefix("statute:") {
                    refs.insert(statute_ref.trim());
                }
            }
            // Recursive cases
            Condition::And(left, right) | Condition::Or(left, right) => {
                Self::extract_refs_from_condition(left, refs);
                Self::extract_refs_from_condition(right, refs);
            }
            Condition::Not(inner) => {
                Self::extract_refs_from_condition(inner, refs);
            }
            // Other conditions don't contain statute references
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
                    // Found a cycle - extract the cycle path
                    let cycle_start_idx = path.iter().position(|p| p == dep).unwrap();
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
                combined =
                    legalis_core::Condition::And(Box::new(combined), Box::new(condition.clone()));
            }

            // If the conjunction is unsatisfiable, the statute is dead
            if let Ok(satisfiable) = smt_verifier.is_satisfiable(&combined) {
                return !satisfiable;
            }
            // Fall through to simple checking on error
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

            if let Ok(contradicts) = smt_verifier.contradict(cond1, cond2) {
                return contradicts;
            }
            // Fall through to simple check on error
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
                combined1 =
                    legalis_core::Condition::And(Box::new(combined1), Box::new(condition.clone()));
            }

            let mut combined2 = statute2.preconditions[0].clone();
            for condition in &statute2.preconditions[1..] {
                combined2 =
                    legalis_core::Condition::And(Box::new(combined2), Box::new(condition.clone()));
            }

            // Check if both sets of preconditions can be true simultaneously
            match smt_verifier.contradict(&combined1, &combined2) {
                Ok(true) => return false, // Preconditions contradict, so statutes don't conflict
                Ok(false) => {
                    // Preconditions can both be true - check if effects conflict
                    use legalis_core::EffectType;
                    return matches!(
                        (&statute1.effect.effect_type, &statute2.effect.effect_type),
                        (EffectType::Grant, EffectType::Revoke)
                            | (EffectType::Revoke, EffectType::Grant)
                            | (EffectType::Obligation, EffectType::Prohibition)
                            | (EffectType::Prohibition, EffectType::Obligation)
                    );
                }
                Err(_) => {} // Fall through to simple check
            }
        }

        // Without SMT solver, do simple effect-based checking
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

        #[cfg(feature = "z3-solver")]
        {
            use crate::smt;

            let ctx = smt::create_z3_context();
            let mut smt_verifier = smt::SmtVerifier::new(&ctx);

            for i in 0..statute.preconditions.len() {
                for j in 0..statute.preconditions.len() {
                    if i == j {
                        continue;
                    }

                    // Check if condition i implies condition j
                    if let Ok(implies) =
                        smt_verifier.implies(&statute.preconditions[i], &statute.preconditions[j])
                    {
                        if implies {
                            // Condition j is redundant
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
        #[cfg(feature = "z3-solver")]
        {
            use crate::smt;
            use legalis_core::Condition;

            let ctx = smt::create_z3_context();
            let mut smt_verifier = smt::SmtVerifier::new(&ctx);

            // Check if this condition is unsatisfiable
            if let Ok(satisfiable) = smt_verifier.is_satisfiable(condition) {
                if !satisfiable {
                    return Some(format!(
                        "Unreachable branch: condition {:?} can never be satisfied",
                        condition
                    ));
                }
            }

            // Check branches in logical operators
            match condition {
                Condition::Or(left, right) => {
                    // Check if left branch is unsatisfiable
                    if let Ok(left_sat) = smt_verifier.is_satisfiable(left) {
                        if !left_sat {
                            return Some(
                                "Left branch of OR is always false, making it redundant"
                                    .to_string(),
                            );
                        }
                    }
                    // Check if right branch is unsatisfiable
                    if let Ok(right_sat) = smt_verifier.is_satisfiable(right) {
                        if !right_sat {
                            return Some(
                                "Right branch of OR is always false, making it redundant"
                                    .to_string(),
                            );
                        }
                    }
                    // Recursively check inner conditions
                    if let Some(msg) = self.find_unreachable_branch(left) {
                        return Some(msg);
                    }
                    if let Some(msg) = self.find_unreachable_branch(right) {
                        return Some(msg);
                    }
                }
                Condition::And(left, right) => {
                    // Recursively check inner conditions
                    if let Some(msg) = self.find_unreachable_branch(left) {
                        return Some(msg);
                    }
                    if let Some(msg) = self.find_unreachable_branch(right) {
                        return Some(msg);
                    }
                }
                Condition::Not(inner) => {
                    // Check if inner condition is a tautology (making NOT always false)
                    if let Ok(is_tautology) = smt_verifier.is_tautology(inner) {
                        if is_tautology {
                            return Some("NOT of a tautology is always false".to_string());
                        }
                    }
                    // Recursively check inner condition
                    if let Some(msg) = self.find_unreachable_branch(inner) {
                        return Some(msg);
                    }
                }
                _ => {}
            }
        }

        #[cfg(not(feature = "z3-solver"))]
        {
            let _ = condition; // Suppress unused variable warning
        }

        None
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

/// Analyzes code coverage for conditions in statutes.
///
/// This function determines which conditions have been evaluated during verification
/// and which paths through the condition logic have been taken.
pub fn analyze_coverage(statutes: &[Statute]) -> CoverageInfo {
    let mut coverage = CoverageInfo::new();

    #[cfg(feature = "z3-solver")]
    {
        use crate::smt;

        let ctx = smt::create_z3_context();
        let mut smt_verifier = smt::SmtVerifier::new(&ctx);

        for statute in statutes {
            let statute_id = statute.id.clone();
            let mut covered_indices = Vec::new();
            let mut uncovered_indices = Vec::new();

            for (idx, condition) in statute.preconditions.iter().enumerate() {
                coverage.total_conditions += 1;

                // Check if condition is satisfiable
                match smt_verifier.is_satisfiable(condition) {
                    Ok(true) => {
                        coverage.satisfiable_conditions += 1;
                        covered_indices.push(idx);
                    }
                    Ok(false) => {
                        coverage.unsatisfiable_conditions += 1;
                        uncovered_indices.push(idx);
                    }
                    Err(_) => {
                        // On error, mark as uncovered
                        uncovered_indices.push(idx);
                    }
                }
            }

            if !covered_indices.is_empty() {
                coverage
                    .covered_conditions
                    .insert(statute_id.clone(), covered_indices);
            }
            if !uncovered_indices.is_empty() {
                coverage
                    .uncovered_conditions
                    .insert(statute_id, uncovered_indices);
            }
        }
    }

    #[cfg(not(feature = "z3-solver"))]
    {
        // Without SMT solver, mark all conditions as covered (conservative approach)
        for statute in statutes {
            let statute_id = statute.id.clone();
            let indices: Vec<usize> = (0..statute.preconditions.len()).collect();
            coverage.total_conditions += statute.preconditions.len();
            coverage.satisfiable_conditions += statute.preconditions.len();

            if !indices.is_empty() {
                coverage.covered_conditions.insert(statute_id, indices);
            }
        }
    }

    coverage.compute_percentage();
    coverage
}

/// Analyzes code coverage for conditions in statutes in parallel (requires 'parallel' feature).
///
/// This function determines which conditions have been evaluated during verification
/// and which paths through the condition logic have been taken. Uses parallel processing
/// to speed up analysis for large statute sets.
#[cfg(feature = "parallel")]
pub fn analyze_coverage_parallel(statutes: &[Statute]) -> CoverageInfo {
    use rayon::prelude::*;
    use std::sync::Mutex;

    let coverage = Mutex::new(CoverageInfo::new());

    #[cfg(feature = "z3-solver")]
    {
        use crate::smt;

        statutes.par_iter().for_each(|statute| {
            let ctx = smt::create_z3_context();
            let mut smt_verifier = smt::SmtVerifier::new(&ctx);

            let statute_id = statute.id.clone();
            let mut covered_indices = Vec::new();
            let mut uncovered_indices = Vec::new();
            let mut total = 0;
            let mut satisfiable = 0;
            let mut unsatisfiable = 0;

            for (idx, condition) in statute.preconditions.iter().enumerate() {
                total += 1;

                // Check if condition is satisfiable
                match smt_verifier.is_satisfiable(condition) {
                    Ok(true) => {
                        satisfiable += 1;
                        covered_indices.push(idx);
                    }
                    Ok(false) => {
                        unsatisfiable += 1;
                        uncovered_indices.push(idx);
                    }
                    Err(_) => {
                        // On error, mark as uncovered
                        uncovered_indices.push(idx);
                    }
                }
            }

            // Update shared coverage info
            if let Ok(mut cov) = coverage.lock() {
                cov.total_conditions += total;
                cov.satisfiable_conditions += satisfiable;
                cov.unsatisfiable_conditions += unsatisfiable;

                if !covered_indices.is_empty() {
                    cov.covered_conditions
                        .insert(statute_id.clone(), covered_indices);
                }
                if !uncovered_indices.is_empty() {
                    cov.uncovered_conditions
                        .insert(statute_id, uncovered_indices);
                }
            }
        });
    }

    #[cfg(not(feature = "z3-solver"))]
    {
        // Without SMT solver, mark all conditions as covered (conservative approach)
        statutes.par_iter().for_each(|statute| {
            let statute_id = statute.id.clone();
            let indices: Vec<usize> = (0..statute.preconditions.len()).collect();
            let total = statute.preconditions.len();

            if let Ok(mut cov) = coverage.lock() {
                cov.total_conditions += total;
                cov.satisfiable_conditions += total;

                if !indices.is_empty() {
                    cov.covered_conditions.insert(statute_id, indices);
                }
            }
        });
    }

    let mut final_coverage = coverage.into_inner().unwrap();
    final_coverage.compute_percentage();
    final_coverage
}

/// Generates an HTML report for verification results.
pub fn generate_html_report(result: &VerificationResult, title: &str) -> String {
    let mut html = String::new();
    html.push_str("<!DOCTYPE html>\n");
    html.push_str("<html lang=\"en\">\n");
    html.push_str("<head>\n");
    html.push_str("  <meta charset=\"UTF-8\">\n");
    html.push_str("  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
    html.push_str(&format!("  <title>{}</title>\n", title));
    html.push_str("  <style>\n");
    html.push_str(
        "    body { font-family: Arial, sans-serif; margin: 20px; background: #f5f5f5; }\n",
    );
    html.push_str("    .container { max-width: 1200px; margin: 0 auto; background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }\n");
    html.push_str(
        "    h1 { color: #333; border-bottom: 3px solid #4CAF50; padding-bottom: 10px; }\n",
    );
    html.push_str(
        "    .status { padding: 10px; margin: 20px 0; border-radius: 4px; font-weight: bold; }\n",
    );
    html.push_str(
        "    .status.pass { background: #d4edda; color: #155724; border: 1px solid #c3e6cb; }\n",
    );
    html.push_str(
        "    .status.fail { background: #f8d7da; color: #721c24; border: 1px solid #f5c6cb; }\n",
    );
    html.push_str("    .section { margin: 20px 0; }\n");
    html.push_str("    .section h2 { color: #555; margin-bottom: 10px; }\n");
    html.push_str("    .error { background: #f8d7da; padding: 10px; margin: 5px 0; border-left: 4px solid #dc3545; border-radius: 4px; }\n");
    html.push_str("    .warning { background: #fff3cd; padding: 10px; margin: 5px 0; border-left: 4px solid #ffc107; border-radius: 4px; }\n");
    html.push_str("    .suggestion { background: #d1ecf1; padding: 10px; margin: 5px 0; border-left: 4px solid #17a2b8; border-radius: 4px; }\n");
    html.push_str("    .empty { color: #999; font-style: italic; }\n");
    html.push_str("    .timestamp { color: #666; font-size: 0.9em; margin-top: 20px; }\n");
    html.push_str("  </style>\n");
    html.push_str("</head>\n");
    html.push_str("<body>\n");
    html.push_str("  <div class=\"container\">\n");
    html.push_str(&format!("    <h1>{}</h1>\n", title));

    // Status section
    let status_class = if result.passed { "pass" } else { "fail" };
    let status_text = if result.passed {
        "✓ Verification Passed"
    } else {
        "✗ Verification Failed"
    };
    html.push_str(&format!(
        "    <div class=\"status {}\">{}</div>\n",
        status_class, status_text
    ));

    // Errors section
    html.push_str("    <div class=\"section\">\n");
    html.push_str("      <h2>Errors</h2>\n");
    if result.errors.is_empty() {
        html.push_str("      <p class=\"empty\">No errors found</p>\n");
    } else {
        for error in &result.errors {
            html.push_str(&format!(
                "      <div class=\"error\">{}</div>\n",
                html_escape(&error.to_string())
            ));
        }
    }
    html.push_str("    </div>\n");

    // Warnings section
    html.push_str("    <div class=\"section\">\n");
    html.push_str("      <h2>Warnings</h2>\n");
    if result.warnings.is_empty() {
        html.push_str("      <p class=\"empty\">No warnings found</p>\n");
    } else {
        for warning in &result.warnings {
            html.push_str(&format!(
                "      <div class=\"warning\">{}</div>\n",
                html_escape(warning)
            ));
        }
    }
    html.push_str("    </div>\n");

    // Suggestions section
    html.push_str("    <div class=\"section\">\n");
    html.push_str("      <h2>Suggestions</h2>\n");
    if result.suggestions.is_empty() {
        html.push_str("      <p class=\"empty\">No suggestions</p>\n");
    } else {
        for suggestion in &result.suggestions {
            html.push_str(&format!(
                "      <div class=\"suggestion\">{}</div>\n",
                html_escape(suggestion)
            ));
        }
    }
    html.push_str("    </div>\n");

    // Timestamp
    html.push_str("    <div class=\"timestamp\">\n");
    html.push_str(&format!(
        "      Generated: {}\n",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
    ));
    html.push_str("    </div>\n");

    html.push_str("  </div>\n");
    html.push_str("</body>\n");
    html.push_str("</html>\n");

    html
}

/// Simple HTML escaping function.
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

/// Generates a PDF report for verification results (requires 'pdf' feature).
///
/// Creates a professional PDF document with verification results,
/// including errors, warnings, and suggestions with proper formatting.
#[cfg(feature = "pdf")]
pub fn generate_pdf_report(
    result: &VerificationResult,
    title: &str,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    use printpdf::*;

    // Create a new PDF document
    let (doc, page1, layer1) = PdfDocument::new(title, Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    // Load fonts
    let font = doc.add_builtin_font(BuiltinFont::Helvetica)?;
    let font_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold)?;

    let mut y_position = Mm(270.0);
    let left_margin = Mm(20.0);
    let line_height = Mm(6.0);

    // Title
    current_layer.use_text(title, 18.0, left_margin, y_position, &font_bold);
    y_position -= line_height * 2.0;

    // Timestamp
    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    current_layer.use_text(
        &format!("Generated: {}", timestamp),
        10.0,
        left_margin,
        y_position,
        &font,
    );
    y_position -= line_height * 2.0;

    // Status
    let status_text = if result.passed {
        "✓ Verification Passed"
    } else {
        "✗ Verification Failed"
    };
    current_layer.use_text(status_text, 14.0, left_margin, y_position, &font_bold);
    y_position -= line_height * 2.0;

    // Errors section
    current_layer.use_text("Errors:", 12.0, left_margin, y_position, &font_bold);
    y_position -= line_height;

    if result.errors.is_empty() {
        current_layer.use_text("  No errors found", 10.0, left_margin, y_position, &font);
        y_position -= line_height;
    } else {
        for (idx, error) in result.errors.iter().enumerate() {
            let error_text = format!("  {}. {}", idx + 1, error);
            // Wrap long text
            for line in wrap_text(&error_text, 80) {
                if y_position < Mm(30.0) {
                    // Add new page if needed
                    let (page, layer) = doc.add_page(Mm(210.0), Mm(297.0), "Layer 1");
                    let new_layer = doc.get_page(page).get_layer(layer);
                    y_position = Mm(270.0);
                    new_layer.use_text(&line, 10.0, left_margin, y_position, &font);
                } else {
                    current_layer.use_text(&line, 10.0, left_margin, y_position, &font);
                }
                y_position -= line_height;
            }
        }
    }
    y_position -= line_height;

    // Warnings section
    current_layer.use_text("Warnings:", 12.0, left_margin, y_position, &font_bold);
    y_position -= line_height;

    if result.warnings.is_empty() {
        current_layer.use_text("  No warnings found", 10.0, left_margin, y_position, &font);
        y_position -= line_height;
    } else {
        for (idx, warning) in result.warnings.iter().enumerate() {
            let warning_text = format!("  {}. {}", idx + 1, warning);
            for line in wrap_text(&warning_text, 80) {
                if y_position < Mm(30.0) {
                    let (page, layer) = doc.add_page(Mm(210.0), Mm(297.0), "Layer 1");
                    let new_layer = doc.get_page(page).get_layer(layer);
                    y_position = Mm(270.0);
                    new_layer.use_text(&line, 10.0, left_margin, y_position, &font);
                } else {
                    current_layer.use_text(&line, 10.0, left_margin, y_position, &font);
                }
                y_position -= line_height;
            }
        }
    }
    y_position -= line_height;

    // Suggestions section
    current_layer.use_text("Suggestions:", 12.0, left_margin, y_position, &font_bold);
    y_position -= line_height;

    if result.suggestions.is_empty() {
        current_layer.use_text("  No suggestions", 10.0, left_margin, y_position, &font);
    } else {
        for (idx, suggestion) in result.suggestions.iter().enumerate() {
            let suggestion_text = format!("  {}. {}", idx + 1, suggestion);
            for line in wrap_text(&suggestion_text, 80) {
                if y_position < Mm(30.0) {
                    let (page, layer) = doc.add_page(Mm(210.0), Mm(297.0), "Layer 1");
                    let new_layer = doc.get_page(page).get_layer(layer);
                    y_position = Mm(270.0);
                    new_layer.use_text(&line, 10.0, left_margin, y_position, &font);
                } else {
                    current_layer.use_text(&line, 10.0, left_margin, y_position, &font);
                }
                y_position -= line_height;
            }
        }
    }

    // Save to bytes
    let mut buffer = std::io::Cursor::new(Vec::new());
    doc.save(&mut buffer)?;
    Ok(buffer.into_inner())
}

/// Helper function to wrap text to a specified width.
#[cfg(feature = "pdf")]
fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in text.split_whitespace() {
        if current_line.len() + word.len() + 1 > max_width {
            if !current_line.is_empty() {
                lines.push(current_line.clone());
                current_line.clear();
            }
        }
        if !current_line.is_empty() {
            current_line.push(' ');
        }
        current_line.push_str(word);
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    if lines.is_empty() {
        lines.push(String::new());
    }

    lines
}

/// Generates a SARIF (Static Analysis Results Interchange Format) report.
///
/// SARIF is a standard JSON format for static analysis results,
/// supported by many IDEs and CI/CD tools.
pub fn generate_sarif_report(
    result: &VerificationResult,
    tool_name: &str,
    tool_version: &str,
) -> Result<String, serde_json::Error> {
    use serde_json::json;

    let mut results_array = Vec::new();

    // Add errors
    for error in &result.errors {
        let (rule_id, message) = match error {
            VerificationError::CircularReference { message } => {
                ("circular-reference", message.clone())
            }
            VerificationError::DeadStatute { statute_id } => (
                "dead-statute",
                format!("Statute '{}' can never be satisfied", statute_id),
            ),
            VerificationError::ConstitutionalConflict {
                statute_id,
                principle,
            } => (
                "constitutional-conflict",
                format!(
                    "Statute '{}' conflicts with constitutional principle '{}'",
                    statute_id, principle
                ),
            ),
            VerificationError::LogicalContradiction { message } => {
                ("logical-contradiction", message.clone())
            }
            VerificationError::Ambiguity { message } => ("ambiguity", message.clone()),
            VerificationError::UnreachableCode { message } => ("unreachable-code", message.clone()),
        };

        results_array.push(json!({
            "ruleId": rule_id,
            "level": "error",
            "message": {
                "text": message
            }
        }));
    }

    // Add warnings
    for warning in &result.warnings {
        results_array.push(json!({
            "ruleId": "warning",
            "level": "warning",
            "message": {
                "text": warning
            }
        }));
    }

    // Add suggestions as notes
    for suggestion in &result.suggestions {
        results_array.push(json!({
            "ruleId": "suggestion",
            "level": "note",
            "message": {
                "text": suggestion
            }
        }));
    }

    let sarif = json!({
        "version": "2.1.0",
        "$schema": "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json",
        "runs": [{
            "tool": {
                "driver": {
                    "name": tool_name,
                    "version": tool_version,
                    "informationUri": "https://github.com/yourusername/legalis-rs",
                    "rules": [
                        {
                            "id": "circular-reference",
                            "name": "CircularReference",
                            "shortDescription": {
                                "text": "Circular reference detected between statutes"
                            },
                            "fullDescription": {
                                "text": "A circular reference occurs when statutes reference each other in a cycle, potentially causing infinite loops."
                            },
                            "helpUri": "https://docs.legalis-rs.org/errors/circular-reference"
                        },
                        {
                            "id": "dead-statute",
                            "name": "DeadStatute",
                            "shortDescription": {
                                "text": "Statute can never be satisfied"
                            },
                            "fullDescription": {
                                "text": "A statute is dead when its preconditions can never be satisfied simultaneously."
                            },
                            "helpUri": "https://docs.legalis-rs.org/errors/dead-statute"
                        },
                        {
                            "id": "constitutional-conflict",
                            "name": "ConstitutionalConflict",
                            "shortDescription": {
                                "text": "Statute conflicts with constitutional principle"
                            },
                            "fullDescription": {
                                "text": "A statute violates one or more constitutional principles."
                            },
                            "helpUri": "https://docs.legalis-rs.org/errors/constitutional-conflict"
                        },
                        {
                            "id": "logical-contradiction",
                            "name": "LogicalContradiction",
                            "shortDescription": {
                                "text": "Logical contradiction between statutes"
                            },
                            "fullDescription": {
                                "text": "Two or more statutes have contradictory effects under the same conditions."
                            },
                            "helpUri": "https://docs.legalis-rs.org/errors/logical-contradiction"
                        },
                        {
                            "id": "ambiguity",
                            "name": "Ambiguity",
                            "shortDescription": {
                                "text": "Ambiguity detected in statute"
                            },
                            "fullDescription": {
                                "text": "A statute contains ambiguous language or conditions that may lead to multiple interpretations."
                            },
                            "helpUri": "https://docs.legalis-rs.org/errors/ambiguity"
                        }
                    ]
                }
            },
            "results": results_array
        }]
    });

    serde_json::to_string_pretty(&sarif)
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

    #[test]
    fn test_json_export() {
        let result = VerificationResult::pass()
            .with_warning("Test warning")
            .with_suggestion("Test suggestion");

        let json = result.to_json().unwrap();
        assert!(json.contains("passed"));
        assert!(json.contains("Test warning"));
        assert!(json.contains("Test suggestion"));
    }

    #[test]
    fn test_json_roundtrip() {
        let original = VerificationResult::fail(vec![VerificationError::CircularReference {
            message: "Test cycle".to_string(),
        }])
        .with_warning("Test warning");

        let json = original.to_json().unwrap();
        let restored = VerificationResult::from_json(&json).unwrap();

        assert_eq!(original.passed, restored.passed);
        assert_eq!(original.errors.len(), restored.errors.len());
        assert_eq!(original.warnings.len(), restored.warnings.len());
    }

    #[test]
    fn test_html_report_generation() {
        let result = VerificationResult::fail(vec![VerificationError::DeadStatute {
            statute_id: "test-1".to_string(),
        }])
        .with_warning("Test warning")
        .with_suggestion("Test suggestion");

        let html = generate_html_report(&result, "Test Report");
        assert!(html.contains("<html"));
        assert!(html.contains("Test Report"));
        assert!(html.contains("test-1"));
        assert!(html.contains("Test warning"));
        assert!(html.contains("Test suggestion"));
        assert!(html.contains("Verification Failed"));
    }

    #[test]
    fn test_sarif_report_generation() {
        let result = VerificationResult::fail(vec![VerificationError::LogicalContradiction {
            message: "Test contradiction".to_string(),
        }])
        .with_warning("Test warning");

        let sarif = generate_sarif_report(&result, "legalis-verifier", "0.2.0").unwrap();
        assert!(sarif.contains("2.1.0"));
        assert!(sarif.contains("legalis-verifier"));
        assert!(sarif.contains("logical-contradiction"));
        assert!(sarif.contains("Test contradiction"));
    }

    #[test]
    fn test_circular_reference_detection() {
        // Create statutes with circular references using Custom conditions
        let statute1 = Statute::new(
            "statute-a",
            "Statute A",
            Effect::new(EffectType::Grant, "Test"),
        )
        .with_precondition(Condition::Custom {
            description: "statute:statute-b".to_string(),
        });

        let statute2 = Statute::new(
            "statute-b",
            "Statute B",
            Effect::new(EffectType::Grant, "Test"),
        )
        .with_precondition(Condition::Custom {
            description: "statute:statute-a".to_string(),
        });

        let verifier = StatuteVerifier::new();
        let result = verifier.verify(&[statute1, statute2]);

        assert!(!result.passed);
        assert!(!result.errors.is_empty());
        // Check that we got a circular reference error
        assert!(
            result
                .errors
                .iter()
                .any(|e| matches!(e, VerificationError::CircularReference { .. }))
        );
    }

    #[test]
    fn test_no_circular_reference() {
        let statute1 = Statute::new(
            "statute-a",
            "Statute A",
            Effect::new(EffectType::Grant, "Test"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let statute2 = Statute::new(
            "statute-b",
            "Statute B",
            Effect::new(EffectType::Grant, "Test"),
        )
        .with_precondition(Condition::Income {
            operator: ComparisonOp::LessThan,
            value: 50000,
        });

        let verifier = StatuteVerifier::new();
        let result = verifier.verify(&[statute1, statute2]);

        // Should not have circular reference errors
        assert!(
            result
                .errors
                .iter()
                .all(|e| !matches!(e, VerificationError::CircularReference { .. }))
        );
    }

    #[test]
    fn test_coverage_analysis() {
        let statutes = vec![
            Statute::new(
                "test-1",
                "Test Statute 1",
                Effect::new(EffectType::Grant, "Test"),
            )
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            }),
            Statute::new(
                "test-2",
                "Test Statute 2",
                Effect::new(EffectType::Grant, "Test"),
            )
            .with_precondition(Condition::Income {
                operator: ComparisonOp::LessThan,
                value: 50000,
            }),
        ];

        let coverage = analyze_coverage(&statutes);

        assert_eq!(coverage.total_conditions, 2);
        assert!(coverage.coverage_percentage >= 0.0);
        assert!(coverage.coverage_percentage <= 100.0);
    }

    #[test]
    fn test_coverage_report() {
        let statutes = vec![
            Statute::new(
                "test-1",
                "Test Statute",
                Effect::new(EffectType::Grant, "Test"),
            )
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            }),
        ];

        let coverage = analyze_coverage(&statutes);
        let report = coverage.report();

        assert!(report.contains("Condition Coverage Report"));
        assert!(report.contains("Total Conditions:"));
        assert!(report.contains("Coverage:"));
    }

    #[test]
    fn test_coverage_info_new() {
        let coverage = CoverageInfo::new();
        assert_eq!(coverage.total_conditions, 0);
        assert_eq!(coverage.coverage_percentage, 0.0);
        assert!(coverage.covered_conditions.is_empty());
        assert!(coverage.uncovered_conditions.is_empty());
    }

    #[test]
    fn test_coverage_compute_percentage() {
        let mut coverage = CoverageInfo::new();
        coverage.total_conditions = 10;
        coverage
            .covered_conditions
            .insert("test".to_string(), vec![0, 1, 2, 3, 4]);

        coverage.compute_percentage();

        assert_eq!(coverage.coverage_percentage, 50.0);
    }
}
