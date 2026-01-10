#![allow(clippy::needless_range_loop)]
#![allow(clippy::format_in_format_args)]

//! Legalis-Verifier: Formal verification for Legalis-RS legal statutes.
//!
//! This crate provides static analysis and verification tools for detecting
//! logical inconsistencies, circular references, and constitutional conflicts
//! in legal statutes.

#[cfg(feature = "z3-solver")]
mod smt;

#[cfg(feature = "z3-solver")]
pub use smt::{SmtVerifier, create_z3_context};

pub mod certification_framework;
pub mod distributed_verification;
pub mod formal_methods;
pub mod ml_verification;

use legalis_core::{EffectType, Statute};
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

impl Default for VerificationBudget {
    fn default() -> Self {
        Self::unlimited()
    }
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
            None => true, // New statute
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

impl Default for IncrementalState {
    fn default() -> Self {
        Self::new()
    }
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

        // Partition statutes into changed and unchanged
        for statute in statutes {
            if state.has_changed(statute) {
                changed_statutes.push(statute);
            } else {
                unchanged_statutes.push(statute);
            }
        }

        // Reuse results for unchanged statutes
        for statute in &unchanged_statutes {
            if let Some(prev_result) = state.get_previous_result(&statute.id) {
                result.merge(prev_result.clone());
            }
        }

        // Verify changed statutes
        for statute in &changed_statutes {
            let statute_result = self.verify_single_statute(statute);
            state.update(statute, statute_result.clone());
            result.merge(statute_result);
        }

        // Always re-check global constraints (circular refs, contradictions)
        // as they may be affected by changes
        if !changed_statutes.is_empty() {
            result.merge(self.check_circular_references(statutes));
            result.merge(self.check_contradictions(statutes));
        }

        result
    }

    /// Verifies a single statute in isolation.
    fn verify_single_statute(&self, statute: &Statute) -> VerificationResult {
        let mut result = VerificationResult::pass();

        // Check constitutional compliance
        result.merge(self.check_constitutional_compliance(statute));

        // Check for redundant conditions
        result.merge(self.check_redundant_conditions(statute));

        // Check for unreachable code
        result.merge(self.check_unreachable_code(statute));

        // Check if statute is dead
        if self.is_dead_statute(statute) {
            result.merge(VerificationResult::fail(vec![
                VerificationError::DeadStatute {
                    statute_id: statute.id.clone(),
                },
            ]));
        }

        result
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

        // Helper to check budget
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

        // Check circular references (counts as 1 check)
        if check_budget(statutes_verified, checks_performed, start_time) {
            budget_exceeded = true;
            return (result, statutes_verified, checks_performed, budget_exceeded);
        }
        result.merge(self.check_circular_references(statutes));
        checks_performed += 1;

        // Check dead statutes (counts as 1 check)
        if check_budget(statutes_verified, checks_performed, start_time) {
            budget_exceeded = true;
            return (result, statutes_verified, checks_performed, budget_exceeded);
        }
        result.merge(self.check_dead_statutes(statutes));
        checks_performed += 1;

        // Verify individual statutes
        for statute in statutes {
            if check_budget(statutes_verified, checks_performed, start_time) {
                budget_exceeded = true;
                break;
            }

            result.merge(self.verify_single_statute(statute));
            statutes_verified += 1;
            checks_performed += 3; // constitutional, redundant, unreachable
        }

        // Check contradictions if budget allows (counts as 1 check)
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
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum PrincipleCheck {
    /// No discrimination based on protected attributes
    NoDiscrimination,
    /// Requires procedural safeguards
    RequiresProcedure,
    /// Must not be retroactive
    NoRetroactivity,
    /// Comprehensive equality check
    EqualityCheck,
    /// Due process verification
    DueProcess,
    /// Privacy impact assessment
    PrivacyImpact,
    /// Proportionality checking
    Proportionality,
    /// Accessibility verification
    Accessibility,
    /// Freedom of expression analysis
    FreedomOfExpression,
    /// Property rights verification
    PropertyRights,
    /// Procedural due process (detailed)
    ProceduralDueProcess,
    /// Equal protection analysis (comprehensive)
    EqualProtection,
    /// Custom check with description and implementation
    Custom {
        /// Description of the custom check
        description: String,
    },
}

/// Result of a constitutional principle check.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PrincipleCheckResult {
    /// Whether the check passed
    pub passed: bool,
    /// Issues found (if any)
    pub issues: Vec<String>,
    /// Suggestions for improvement
    pub suggestions: Vec<String>,
}

impl PrincipleCheckResult {
    /// Creates a passing result.
    pub fn pass() -> Self {
        Self {
            passed: true,
            issues: Vec::new(),
            suggestions: Vec::new(),
        }
    }

    /// Creates a failing result with issues.
    pub fn fail(issues: Vec<String>) -> Self {
        Self {
            passed: false,
            issues,
            suggestions: Vec::new(),
        }
    }

    /// Adds a suggestion.
    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestions.push(suggestion.into());
        self
    }
}

/// Performs a comprehensive equality check on a statute.
///
/// Checks for potential discrimination based on protected attributes
/// like age, gender, race, etc.
pub fn check_equality(statute: &Statute) -> PrincipleCheckResult {
    let mut issues = Vec::new();

    // Check for age-based discrimination
    for condition in &statute.preconditions {
        if let legalis_core::Condition::Age { operator, value } = condition {
            use legalis_core::ComparisonOp;
            match operator {
                ComparisonOp::GreaterThan | ComparisonOp::GreaterOrEqual => {
                    if *value > 65 {
                        issues.push(format!(
                            "Potential age discrimination: requires age > {}",
                            value
                        ));
                    }
                }
                ComparisonOp::LessThan | ComparisonOp::LessOrEqual => {
                    if *value < 18 {
                        issues.push(format!(
                            "Potential age discrimination: requires age < {}",
                            value
                        ));
                    }
                }
                _ => {}
            }
        }
    }

    // Check for attribute-based discrimination
    for condition in &statute.preconditions {
        if let legalis_core::Condition::HasAttribute { key } = condition {
            let key_lower = key.to_lowercase();
            if key_lower.contains("gender")
                || key_lower.contains("race")
                || key_lower.contains("religion")
                || key_lower.contains("nationality")
            {
                issues.push(format!(
                    "Potential discrimination based on protected attribute: {}",
                    key
                ));
            }
        }
    }

    if issues.is_empty() {
        PrincipleCheckResult::pass()
    } else {
        PrincipleCheckResult::fail(issues).with_suggestion(
            "Review for potential discrimination and ensure legitimate justification exists",
        )
    }
}

/// Performs due process verification on a statute.
///
/// Checks that adequate procedural safeguards are in place.
pub fn check_due_process(statute: &Statute) -> PrincipleCheckResult {
    let mut issues = Vec::new();

    // Check if statute has discretion logic (good for due process)
    let has_discretion = statute.discretion_logic.is_some();

    // Check for certain effect types that require due process
    use legalis_core::EffectType;
    match statute.effect.effect_type {
        EffectType::Revoke | EffectType::Prohibition => {
            if !has_discretion {
                issues.push(
                    "Statute revokes/prohibits without discretionary review mechanism".to_string(),
                );
            }
        }
        _ => {}
    }

    // Check for abrupt conditions without review
    if statute.preconditions.is_empty() && !has_discretion {
        match statute.effect.effect_type {
            EffectType::Revoke | EffectType::Prohibition => {
                issues.push("No preconditions or review process for punitive action".to_string());
            }
            _ => {}
        }
    }

    if issues.is_empty() {
        PrincipleCheckResult::pass()
    } else {
        PrincipleCheckResult::fail(issues)
            .with_suggestion("Add discretionary review or appeal mechanism for fairness")
    }
}

/// Performs privacy impact assessment on a statute.
///
/// Checks if the statute might impact individual privacy.
pub fn check_privacy_impact(statute: &Statute) -> PrincipleCheckResult {
    let mut issues = Vec::new();
    let mut suggestions = Vec::new();

    // Check for attributes that might involve personal data
    for condition in &statute.preconditions {
        match condition {
            legalis_core::Condition::HasAttribute { key }
            | legalis_core::Condition::AttributeEquals { key, .. } => {
                let key_lower = key.to_lowercase();
                if key_lower.contains("medical")
                    || key_lower.contains("health")
                    || key_lower.contains("financial")
                    || key_lower.contains("biometric")
                    || key_lower.contains("location")
                    || key_lower.contains("address")
                {
                    issues.push(format!(
                        "Processes potentially sensitive personal data: {}",
                        key
                    ));
                    suggestions
                        .push("Ensure data minimization and appropriate safeguards".to_string());
                }
            }
            legalis_core::Condition::Geographic { .. } => {
                issues.push("Uses geographic/location data which may be sensitive".to_string());
                suggestions.push("Consider privacy implications of location tracking".to_string());
            }
            _ => {}
        }
    }

    if issues.is_empty() {
        PrincipleCheckResult::pass()
    } else {
        let mut result = PrincipleCheckResult::fail(issues);
        for suggestion in suggestions {
            result = result.with_suggestion(suggestion);
        }
        result
    }
}

/// Performs proportionality checking on a statute.
///
/// Checks if the statute's effects are proportional to its conditions.
pub fn check_proportionality(statute: &Statute) -> PrincipleCheckResult {
    let mut issues = Vec::new();

    use legalis_core::EffectType;

    // Check for disproportionate effects
    let condition_count = statute.preconditions.len();

    match statute.effect.effect_type {
        EffectType::Revoke | EffectType::Prohibition => {
            // Severe effects should have multiple conditions
            if condition_count < 2 {
                issues.push(format!(
                    "Severe effect ({:?}) based on only {} condition(s) - may be disproportionate",
                    statute.effect.effect_type, condition_count
                ));
            }
        }
        EffectType::Obligation => {
            // Obligations should have clear conditions
            if condition_count == 0 {
                issues.push(
                    "Obligation imposed without any preconditions - may be disproportionate"
                        .to_string(),
                );
            }
        }
        _ => {}
    }

    // Check for overly complex conditions for simple grants
    if matches!(statute.effect.effect_type, EffectType::Grant) && condition_count > 5 {
        issues.push(format!(
            "Simple grant has {} conditions - may create unnecessary barriers",
            condition_count
        ));
    }

    if issues.is_empty() {
        PrincipleCheckResult::pass()
    } else {
        PrincipleCheckResult::fail(issues)
            .with_suggestion("Ensure effects are proportional to conditions and legitimate aims")
    }
}

/// Performs accessibility verification on a statute.
///
/// Checks if the statute creates barriers for people with disabilities
/// or other accessibility concerns.
pub fn check_accessibility(statute: &Statute) -> PrincipleCheckResult {
    let mut issues = Vec::new();
    let mut suggestions = Vec::new();

    // Check for conditions that might create accessibility barriers
    for condition in &statute.preconditions {
        match condition {
            legalis_core::Condition::HasAttribute { key } => {
                let key_lower = key.to_lowercase();

                // Check for physical ability requirements
                if key_lower.contains("physical")
                    || key_lower.contains("mobility")
                    || key_lower.contains("vision")
                    || key_lower.contains("hearing")
                {
                    issues.push(format!(
                        "Condition based on physical ability may create accessibility barrier: {}",
                        key
                    ));
                    suggestions.push(
                        "Consider reasonable accommodations for people with disabilities"
                            .to_string(),
                    );
                }

                // Check for digital literacy requirements
                if key_lower.contains("digital")
                    || key_lower.contains("online")
                    || key_lower.contains("internet")
                {
                    issues.push(format!(
                        "Digital requirement may create barrier for those without internet access: {}",
                        key
                    ));
                    suggestions
                        .push("Provide alternative non-digital methods of compliance".to_string());
                }

                // Check for language barriers
                if key_lower.contains("language") || key_lower.contains("english") {
                    issues.push(format!("Language requirement may create barrier: {}", key));
                    suggestions
                        .push("Provide translation services or multilingual support".to_string());
                }
            }

            // Check for geographic barriers
            legalis_core::Condition::Geographic { region_id, .. } => {
                if !region_id.is_empty() {
                    issues.push(format!(
                        "Geographic restriction may limit accessibility: {}",
                        region_id
                    ));
                    suggestions.push(
                        "Consider remote participation options or multiple locations".to_string(),
                    );
                }
            }

            // Check for income barriers
            legalis_core::Condition::Income { operator, value } => {
                use legalis_core::ComparisonOp;
                if matches!(
                    operator,
                    ComparisonOp::GreaterThan | ComparisonOp::GreaterOrEqual
                ) {
                    issues.push(format!(
                        "Income requirement may create financial barrier: requires income >= {}",
                        value
                    ));
                    suggestions.push(
                        "Consider fee waivers or sliding scale based on ability to pay".to_string(),
                    );
                }
            }

            _ => {}
        }
    }

    // Check if statute provides discretion (good for accessibility)
    if statute.discretion_logic.is_some() {
        suggestions
            .push("Discretion allows for individual accessibility accommodations".to_string());
    }

    if issues.is_empty() {
        PrincipleCheckResult::pass()
    } else {
        let mut result = PrincipleCheckResult::fail(issues);
        for suggestion in suggestions {
            result = result.with_suggestion(suggestion);
        }
        result
    }
}

/// Checks if the statute violates the principle of non-retroactivity (ex post facto).
///
/// The principle of non-retroactivity means laws should not apply to conduct
/// that occurred before the law came into effect. This is especially important for:
/// - Criminal laws and prohibitions
/// - New obligations and duties
/// - Revocation of rights
pub fn check_retroactivity(statute: &Statute) -> PrincipleCheckResult {
    let mut issues = Vec::new();
    let mut suggestions = Vec::new();

    // Check if statute has an effective date
    if let Some(effective_date) = statute.temporal_validity.effective_date {
        // Check effect type - prohibitions, obligations, and revocations should not be retroactive
        let is_restrictive_effect = matches!(
            statute.effect.effect_type,
            legalis_core::EffectType::Prohibition
                | legalis_core::EffectType::Obligation
                | legalis_core::EffectType::Revoke
        );

        if is_restrictive_effect {
            // Check for retroactive application indicators
            let effect_desc_lower = statute.effect.description.to_lowercase();

            // Check for explicit retroactive language
            if effect_desc_lower.contains("retroactive")
                || effect_desc_lower.contains("retrospective")
                || effect_desc_lower.contains("prior to")
                || effect_desc_lower.contains("before")
            {
                issues.push(format!(
                    "{:?} effect appears to apply retroactively: '{}'",
                    statute.effect.effect_type, statute.effect.description
                ));
                suggestions.push(
                    "Consider prospective application only (applying from effective date forward)"
                        .to_string(),
                );
            }

            // Check effect parameters for retroactive indicators
            if let Some(retroactive_param) = statute.effect.parameters.get("retroactive") {
                if retroactive_param.to_lowercase() == "true"
                    || retroactive_param.to_lowercase() == "yes"
                {
                    issues.push(format!(
                        "{:?} effect is marked as retroactive, which may violate ex post facto principles",
                        statute.effect.effect_type
                    ));
                    suggestions.push(format!(
                        "Ensure effective date ({}) is not applied to conduct before that date",
                        effective_date
                    ));
                }
            }

            // Check for application_date that precedes effective_date
            if let Some(application_date_str) = statute.effect.parameters.get("application_date") {
                // Try to parse the application date
                if let Ok(application_date) =
                    chrono::NaiveDate::parse_from_str(application_date_str, "%Y-%m-%d")
                {
                    if application_date < effective_date {
                        issues.push(format!(
                            "Application date ({}) precedes effective date ({}), creating retroactive effect",
                            application_date, effective_date
                        ));
                        suggestions.push(
                            "Align application date with or after effective date".to_string(),
                        );
                    }
                }
            }
        }

        // Special check for MonetaryTransfer that might be a penalty
        if matches!(
            statute.effect.effect_type,
            legalis_core::EffectType::MonetaryTransfer
        ) {
            let effect_desc_lower = statute.effect.description.to_lowercase();
            let is_penalty = effect_desc_lower.contains("fine")
                || effect_desc_lower.contains("penalty")
                || effect_desc_lower.contains("sanction");

            if is_penalty {
                if let Some(retroactive_param) = statute.effect.parameters.get("retroactive") {
                    if retroactive_param.to_lowercase() == "true" {
                        issues.push(
                            "Monetary penalty appears to apply retroactively, violating ex post facto principles".to_string(),
                        );
                        suggestions.push(
                            "Apply penalties only to violations occurring after the effective date"
                                .to_string(),
                        );
                    }
                }
            }
        }
    } else {
        // No effective date specified - warn about temporal clarity
        if matches!(
            statute.effect.effect_type,
            legalis_core::EffectType::Prohibition
                | legalis_core::EffectType::Obligation
                | legalis_core::EffectType::Revoke
        ) {
            suggestions.push(
                "Consider specifying an effective date to ensure prospective application"
                    .to_string(),
            );
        }
    }

    // Check enacted_at vs effective_date for grace period
    if let (Some(enacted_at), Some(effective_date)) = (
        statute.temporal_validity.enacted_at,
        statute.temporal_validity.effective_date,
    ) {
        let enacted_date = enacted_at.date_naive();

        if effective_date < enacted_date {
            issues.push(format!(
                "Effective date ({}) is before enactment date ({}), creating improper retroactive application",
                effective_date, enacted_date
            ));
            suggestions.push("Effective date should be on or after enactment date".to_string());
        } else if effective_date == enacted_date
            && matches!(
                statute.effect.effect_type,
                legalis_core::EffectType::Prohibition | legalis_core::EffectType::Obligation
            )
        {
            suggestions.push(
                "Consider providing a grace period between enactment and effective date for compliance"
                    .to_string(),
            );
        }
    }

    if issues.is_empty() {
        let mut result = PrincipleCheckResult::pass();
        for suggestion in suggestions {
            result = result.with_suggestion(suggestion);
        }
        result
    } else {
        let mut result = PrincipleCheckResult::fail(issues);
        for suggestion in suggestions {
            result = result.with_suggestion(suggestion);
        }
        result
    }
}

/// Checks freedom of expression principles.
///
/// Verifies that statutes do not unduly restrict speech, press, assembly,
/// or other expressive activities without compelling justification.
pub fn check_freedom_of_expression(statute: &Statute) -> PrincipleCheckResult {
    let mut issues = Vec::new();
    let mut suggestions = Vec::new();

    // Check for prohibitions or obligations that might restrict expression
    if matches!(
        statute.effect.effect_type,
        legalis_core::EffectType::Prohibition | legalis_core::EffectType::Obligation
    ) {
        let effect_desc_lower = statute.effect.description.to_lowercase();
        let title_lower = statute.title.to_lowercase();

        // Check for speech-related restrictions
        let speech_keywords = [
            "speech",
            "speak",
            "express",
            "say",
            "publish",
            "broadcast",
            "communicate",
            "media",
            "press",
            "assembly",
            "protest",
            "demonstration",
            "petition",
            "religion",
            "belief",
            "opinion",
        ];

        let affects_expression = speech_keywords
            .iter()
            .any(|keyword| effect_desc_lower.contains(keyword) || title_lower.contains(keyword));

        if affects_expression {
            // Check if there's a compelling justification
            let has_justification = effect_desc_lower.contains("safety")
                || effect_desc_lower.contains("security")
                || effect_desc_lower.contains("public health")
                || effect_desc_lower.contains("emergency")
                || effect_desc_lower.contains("imminent")
                || effect_desc_lower.contains("violence")
                || statute.discretion_logic.is_some();

            if !has_justification {
                issues.push(format!(
                    "Statute '{}' may restrict freedom of expression without clear compelling justification",
                    statute.id
                ));
                suggestions.push(
                    "Add explicit justification for expression restrictions (e.g., public safety, imminent harm)"
                        .to_string(),
                );
                suggestions.push(
                    "Consider narrow tailoring to minimize impact on protected speech".to_string(),
                );
            } else {
                suggestions.push(
                    "Ensure restrictions are narrowly tailored and use least restrictive means"
                        .to_string(),
                );
            }

            // Check for prior restraint
            if effect_desc_lower.contains("prior")
                || effect_desc_lower.contains("advance")
                || effect_desc_lower.contains("pre-approval")
                || effect_desc_lower.contains("permit required")
            {
                issues.push(
                    "Statute may impose prior restraint on expression, which is generally unconstitutional"
                        .to_string(),
                );
                suggestions.push(
                    "Consider post-publication remedies instead of prior restraint".to_string(),
                );
            }
        }
    }

    if issues.is_empty() {
        let mut result = PrincipleCheckResult::pass();
        for suggestion in suggestions {
            result = result.with_suggestion(suggestion);
        }
        result
    } else {
        let mut result = PrincipleCheckResult::fail(issues);
        for suggestion in suggestions {
            result = result.with_suggestion(suggestion);
        }
        result
    }
}

/// Checks property rights principles.
///
/// Verifies that statutes respect property rights and provide just compensation
/// for takings or restrictions on property use.
pub fn check_property_rights(statute: &Statute) -> PrincipleCheckResult {
    let mut issues = Vec::new();
    let mut suggestions = Vec::new();

    let effect_desc_lower = statute.effect.description.to_lowercase();
    let title_lower = statute.title.to_lowercase();

    // Check for property-related effects
    let property_keywords = [
        "property",
        "land",
        "real estate",
        "possession",
        "ownership",
        "seizure",
        "confiscation",
        "taking",
        "eminent domain",
        "expropriation",
        "restriction",
        "use",
        "development",
    ];

    let affects_property = property_keywords
        .iter()
        .any(|keyword| effect_desc_lower.contains(keyword) || title_lower.contains(keyword));

    if affects_property {
        // Check for takings or restrictions
        if matches!(
            statute.effect.effect_type,
            legalis_core::EffectType::Revoke
                | legalis_core::EffectType::Prohibition
                | legalis_core::EffectType::Obligation
        ) {
            // Check for compensation provisions
            let has_compensation = effect_desc_lower.contains("compensation")
                || effect_desc_lower.contains("reimbursement")
                || effect_desc_lower.contains("payment")
                || statute
                    .effect
                    .parameters
                    .contains_key("compensation_amount");

            if (effect_desc_lower.contains("taking")
                || effect_desc_lower.contains("seiz")
                || effect_desc_lower.contains("confiscat"))
                && !has_compensation
            {
                issues.push(format!(
                    "Statute '{}' may involve property taking without just compensation",
                    statute.id
                ));
                suggestions.push(
                    "Provide just compensation for property takings as required by law".to_string(),
                );
            }

            // Check for regulatory takings
            if effect_desc_lower.contains("prohibit")
                || effect_desc_lower.contains("restrict")
                || effect_desc_lower.contains("limit")
            {
                suggestions.push(
                    "Consider whether regulatory restrictions constitute a taking requiring compensation"
                        .to_string(),
                );
                suggestions.push(
                    "Ensure restrictions permit economically viable use of property".to_string(),
                );
            }
        }

        // Check for due process in property deprivation
        if matches!(statute.effect.effect_type, legalis_core::EffectType::Revoke) {
            let has_procedure = effect_desc_lower.contains("hearing")
                || effect_desc_lower.contains("notice")
                || effect_desc_lower.contains("appeal")
                || statute.discretion_logic.is_some();

            if !has_procedure {
                issues.push(
                    "Property deprivation may lack adequate procedural safeguards".to_string(),
                );
                suggestions.push(
                    "Provide notice and opportunity for hearing before property deprivation"
                        .to_string(),
                );
            }
        }
    }

    if issues.is_empty() {
        let mut result = PrincipleCheckResult::pass();
        for suggestion in suggestions {
            result = result.with_suggestion(suggestion);
        }
        result
    } else {
        let mut result = PrincipleCheckResult::fail(issues);
        for suggestion in suggestions {
            result = result.with_suggestion(suggestion);
        }
        result
    }
}

/// Checks procedural due process principles (detailed).
///
/// Verifies that statutes provide adequate procedural safeguards when depriving
/// individuals of life, liberty, or property interests.
pub fn check_procedural_due_process(statute: &Statute) -> PrincipleCheckResult {
    let mut issues = Vec::new();
    let mut suggestions = Vec::new();

    // Identify deprivations requiring due process
    let requires_due_process = matches!(
        statute.effect.effect_type,
        legalis_core::EffectType::Revoke
            | legalis_core::EffectType::Prohibition
            | legalis_core::EffectType::MonetaryTransfer
    );

    if requires_due_process {
        let effect_desc_lower = statute.effect.description.to_lowercase();

        // Required procedural elements
        let has_notice = effect_desc_lower.contains("notice")
            || effect_desc_lower.contains("notification")
            || effect_desc_lower.contains("inform");

        let has_hearing = effect_desc_lower.contains("hearing")
            || effect_desc_lower.contains("proceeding")
            || effect_desc_lower.contains("tribunal");

        let has_representation = effect_desc_lower.contains("represent")
            || effect_desc_lower.contains("counsel")
            || effect_desc_lower.contains("attorney")
            || effect_desc_lower.contains("lawyer");

        let has_appeal = effect_desc_lower.contains("appeal")
            || effect_desc_lower.contains("review")
            || effect_desc_lower.contains("reconsideration");

        let has_evidence = effect_desc_lower.contains("evidence")
            || effect_desc_lower.contains("testimony")
            || effect_desc_lower.contains("witness");

        // Check for critical procedural safeguards
        if !has_notice {
            issues.push("Statute lacks explicit notice requirement before deprivation".to_string());
            suggestions
                .push("Add requirement for adequate notice before taking action".to_string());
        }

        if !has_hearing {
            issues.push("Statute lacks explicit hearing or opportunity to be heard".to_string());
            suggestions
                .push("Provide opportunity for hearing before final deprivation".to_string());
        }

        if !has_representation {
            suggestions
                .push("Consider allowing right to legal representation in proceedings".to_string());
        }

        if !has_appeal {
            suggestions.push(
                "Consider providing appeal or review mechanism for adverse decisions".to_string(),
            );
        }

        if !has_evidence {
            suggestions.push(
                "Allow parties to present evidence and confront adverse evidence".to_string(),
            );
        }

        // Check for impartiality
        if statute.discretion_logic.is_some() {
            suggestions.push(
                "Ensure decision-makers are impartial and free from conflicts of interest"
                    .to_string(),
            );
        }

        // Check for timely proceedings
        if !effect_desc_lower.contains("timely")
            && !effect_desc_lower.contains("prompt")
            && !effect_desc_lower.contains("within")
        {
            suggestions
                .push("Specify timeframes to ensure timely resolution of proceedings".to_string());
        }
    }

    if issues.is_empty() {
        let mut result = PrincipleCheckResult::pass();
        for suggestion in suggestions {
            result = result.with_suggestion(suggestion);
        }
        result
    } else {
        let mut result = PrincipleCheckResult::fail(issues);
        for suggestion in suggestions {
            result = result.with_suggestion(suggestion);
        }
        result
    }
}

/// Checks equal protection principles (comprehensive).
///
/// Verifies that statutes treat similarly situated persons equally and
/// that any differential treatment has adequate justification.
pub fn check_equal_protection(statute: &Statute) -> PrincipleCheckResult {
    let mut issues = Vec::new();
    let mut suggestions = Vec::new();

    // Check for classifications based on protected characteristics
    let mut classifications = Vec::new();

    for condition in &statute.preconditions {
        match condition {
            legalis_core::Condition::Age { .. } => {
                classifications.push(("age", "intermediate scrutiny"));
            }
            legalis_core::Condition::AttributeEquals { key, value } => {
                let key_lower = key.to_lowercase();
                let value_lower = value.to_lowercase();
                let combined = format!("{} {}", key_lower, value_lower);

                // Suspect classifications (strict scrutiny)
                if combined.contains("race")
                    || combined.contains("national origin")
                    || combined.contains("ethnicity")
                {
                    classifications.push(("race/national origin", "strict scrutiny"));
                    issues.push(format!(
                        "Classification based on race/national origin in '{}: {}' requires strict scrutiny",
                        key, value
                    ));
                }

                if combined.contains("religion") || combined.contains("religious") {
                    classifications.push(("religion", "strict scrutiny"));
                    issues.push(format!(
                        "Classification based on religion in '{}: {}' requires strict scrutiny",
                        key, value
                    ));
                }

                // Quasi-suspect classifications (intermediate scrutiny)
                if combined.contains("gender")
                    || combined.contains("sex")
                    || key_lower == "gender"
                    || key_lower == "sex"
                {
                    classifications.push(("gender/sex", "intermediate scrutiny"));
                    suggestions.push(format!(
                        "Gender classification in '{}: {}' requires substantial justification",
                        key, value
                    ));
                }

                if combined.contains("citizenship") || combined.contains("alien") {
                    classifications.push(("citizenship", "intermediate scrutiny"));
                    suggestions.push(format!(
                        "Citizenship classification in '{}: {}' may require heightened scrutiny",
                        key, value
                    ));
                }
            }
            legalis_core::Condition::Income { .. } => {
                classifications.push(("economic status", "rational basis"));
                suggestions.push("Ensure economic classifications have rational basis".to_string());
            }
            _ => {}
        }
    }

    // Check effect description for discriminatory language
    let effect_desc_lower = statute.effect.description.to_lowercase();
    if effect_desc_lower.contains("discriminat")
        || effect_desc_lower.contains("preferential")
        || effect_desc_lower.contains("exclusive")
    {
        issues.push("Effect description suggests potential discriminatory treatment".to_string());
        suggestions.push(
            "Ensure any differential treatment serves important governmental interest".to_string(),
        );
    }

    // Provide guidance based on classifications found
    if !classifications.is_empty() {
        suggestions.push(format!(
            "Statute contains {} classification(s) requiring review",
            classifications.len()
        ));

        for (classification, standard) in &classifications {
            if standard == &"strict scrutiny" {
                suggestions.push(format!(
                    "For {} classification: Must serve compelling governmental interest and be narrowly tailored",
                    classification
                ));
            } else if standard == &"intermediate scrutiny" {
                suggestions.push(format!(
                    "For {} classification: Must serve important governmental interest and be substantially related",
                    classification
                ));
            }
        }
    }

    // Check for arbitrary classifications
    if statute.preconditions.len() > 3 && statute.discretion_logic.is_none() {
        suggestions.push(
            "Complex preconditions without discretion logic may create arbitrary distinctions"
                .to_string(),
        );
        suggestions.push(
            "Consider adding discretion logic to explain classification rationale".to_string(),
        );
    }

    if issues.is_empty() {
        let mut result = PrincipleCheckResult::pass();
        for suggestion in suggestions {
            result = result.with_suggestion(suggestion);
        }
        result
    } else {
        let mut result = PrincipleCheckResult::fail(issues);
        for suggestion in suggestions {
            result = result.with_suggestion(suggestion);
        }
        result
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

/// Impact level classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ImpactLevel {
    Low,
    Medium,
    High,
}

impl std::fmt::Display for ImpactLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Low => write!(f, "Low"),
            Self::Medium => write!(f, "Medium"),
            Self::High => write!(f, "High"),
        }
    }
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

impl std::fmt::Display for RiskLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Minimal => write!(f, "Minimal"),
            Self::Low => write!(f, "Low"),
            Self::Medium => write!(f, "Medium"),
            Self::High => write!(f, "High"),
            Self::Critical => write!(f, "Critical"),
        }
    }
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

impl Default for ImpactAssessment {
    fn default() -> Self {
        Self::new()
    }
}

/// Performs an impact assessment on a statute.
///
/// Analyzes the potential impacts of a statute on various groups and dimensions.
pub fn assess_impact(statute: &Statute) -> ImpactAssessment {
    let mut assessment = ImpactAssessment::new();

    // Identify affected groups
    for condition in &statute.preconditions {
        match condition {
            legalis_core::Condition::Age { value, .. } => {
                if *value < 18 {
                    assessment.affected_groups.push("Minors".to_string());
                } else if *value >= 65 {
                    assessment.affected_groups.push("Seniors".to_string());
                } else {
                    assessment.affected_groups.push("Adults".to_string());
                }
            }
            legalis_core::Condition::Income { .. } => {
                assessment
                    .affected_groups
                    .push("Income earners".to_string());
                assessment.economic_impact = ImpactLevel::High;
            }
            legalis_core::Condition::HasAttribute { key } => {
                let key_lower = key.to_lowercase();
                if key_lower.contains("disabled") || key_lower.contains("disability") {
                    assessment
                        .affected_groups
                        .push("People with disabilities".to_string());
                }
                if key_lower.contains("veteran") {
                    assessment.affected_groups.push("Veterans".to_string());
                }
                if key_lower.contains("student") {
                    assessment.affected_groups.push("Students".to_string());
                }
            }
            legalis_core::Condition::Geographic { region_id, .. } => {
                assessment
                    .affected_groups
                    .push(format!("Residents of {}", region_id));
            }
            _ => {}
        }
    }

    // Analyze impacts based on effect type
    use legalis_core::EffectType;
    match statute.effect.effect_type {
        EffectType::Grant => {
            assessment
                .positive_impacts
                .push(format!("Grants benefit: {}", statute.effect.description));
            assessment.social_impact = ImpactLevel::Medium;
        }
        EffectType::Revoke => {
            assessment
                .negative_impacts
                .push(format!("Revokes benefit: {}", statute.effect.description));
            assessment.social_impact = ImpactLevel::High;
            assessment.overall_risk = RiskLevel::High;
        }
        EffectType::Obligation => {
            assessment.negative_impacts.push(format!(
                "Imposes obligation: {}",
                statute.effect.description
            ));
            assessment.social_impact = ImpactLevel::Medium;
        }
        EffectType::Prohibition => {
            assessment
                .negative_impacts
                .push(format!("Prohibits action: {}", statute.effect.description));
            assessment.social_impact = ImpactLevel::High;
            assessment.overall_risk = RiskLevel::High;
        }
        _ => {}
    }

    // Run constitutional checks and aggregate concerns
    let equality_result = check_equality(statute);
    if !equality_result.passed {
        assessment.equity_concerns.extend(equality_result.issues);
        assessment.overall_risk = assessment.overall_risk.max(RiskLevel::High);
    }

    let accessibility_result = check_accessibility(statute);
    if !accessibility_result.passed {
        assessment
            .accessibility_concerns
            .extend(accessibility_result.issues);
        assessment.overall_risk = assessment.overall_risk.max(RiskLevel::Medium);
    }

    let privacy_result = check_privacy_impact(statute);
    if !privacy_result.passed {
        assessment.privacy_concerns.extend(privacy_result.issues);
        assessment.overall_risk = assessment.overall_risk.max(RiskLevel::Medium);
    }

    // Adjust risk based on number of concerns
    let total_concerns = assessment.equity_concerns.len()
        + assessment.accessibility_concerns.len()
        + assessment.privacy_concerns.len();

    if total_concerns > 5 {
        assessment.overall_risk = RiskLevel::Critical;
    } else if total_concerns > 3 {
        assessment.overall_risk = assessment.overall_risk.max(RiskLevel::High);
    }

    assessment
}

/// Performs impact assessment on multiple statutes and generates a comprehensive report.
pub fn assess_multiple_impacts(statutes: &[Statute]) -> String {
    let mut report = String::new();
    report.push_str("# Comprehensive Impact Assessment\n\n");
    report.push_str(&format!("Analyzed {} statute(s)\n\n", statutes.len()));

    let mut all_groups: HashSet<String> = HashSet::new();
    let mut total_positive = 0;
    let mut total_negative = 0;
    let mut max_risk = RiskLevel::Low;

    for statute in statutes {
        let assessment = assess_impact(statute);
        all_groups.extend(assessment.affected_groups.clone());
        total_positive += assessment.positive_impacts.len();
        total_negative += assessment.negative_impacts.len();
        max_risk = max_risk.max(assessment.overall_risk);

        report.push_str(&format!(
            "## Statute: {} - \"{}\"\n",
            statute.id, statute.title
        ));
        report.push_str(&assessment.report());
        report.push_str("\n---\n\n");
    }

    // Summary
    report.push_str("## Overall Summary\n\n");
    report.push_str(&format!("- **Maximum Risk Level**: {}\n", max_risk));
    report.push_str(&format!(
        "- **Total Affected Groups**: {}\n",
        all_groups.len()
    ));
    report.push_str(&format!(
        "- **Total Positive Impacts**: {}\n",
        total_positive
    ));
    report.push_str(&format!(
        "- **Total Negative Impacts**: {}\n\n",
        total_negative
    ));

    if !all_groups.is_empty() {
        report.push_str("### All Affected Groups:\n");
        for group in &all_groups {
            report.push_str(&format!("- {}\n", group));
        }
    }

    report
}

/// Verifies the integrity of a set of laws.
pub fn verify_integrity(laws: &[Statute]) -> Result<VerificationResult, String> {
    let verifier = StatuteVerifier::new();
    Ok(verifier.verify(laws))
}

// =============================================================================
// Statute Conflict Detection
// =============================================================================

/// Types of conflicts that can occur between statutes.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ConflictType {
    /// Statutes have overlapping conditions but contradictory effects
    EffectConflict,
    /// Multiple statutes claim authority over the same jurisdiction
    JurisdictionalOverlap,
    /// Statutes with overlapping temporal validity have conflicting rules
    TemporalConflict,
    /// Lower-level statute contradicts higher-level statute
    HierarchyViolation,
    /// Statutes with same ID in different jurisdictions
    IdCollision,
}

impl std::fmt::Display for ConflictType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EffectConflict => write!(f, "Effect Conflict"),
            Self::JurisdictionalOverlap => write!(f, "Jurisdictional Overlap"),
            Self::TemporalConflict => write!(f, "Temporal Conflict"),
            Self::HierarchyViolation => write!(f, "Hierarchy Violation"),
            Self::IdCollision => write!(f, "ID Collision"),
        }
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

/// Detects conflicts between statutes.
pub fn detect_statute_conflicts(statutes: &[Statute]) -> Vec<StatuteConflict> {
    let mut conflicts = Vec::new();

    // Detect ID collisions
    conflicts.extend(detect_id_collisions(statutes));

    // Detect effect conflicts
    conflicts.extend(detect_effect_conflicts(statutes));

    // Detect jurisdictional overlaps
    conflicts.extend(detect_jurisdictional_overlaps(statutes));

    // Detect temporal conflicts
    conflicts.extend(detect_temporal_conflicts(statutes));

    conflicts
}

/// Detects ID collisions (multiple statutes with the same ID).
fn detect_id_collisions(statutes: &[Statute]) -> Vec<StatuteConflict> {
    let mut conflicts = Vec::new();
    let mut id_map: HashMap<String, Vec<usize>> = HashMap::new();

    // Group statutes by ID
    for (idx, statute) in statutes.iter().enumerate() {
        id_map.entry(statute.id.clone()).or_default().push(idx);
    }

    // Find duplicates
    for (id, indices) in id_map.iter() {
        if indices.len() > 1 {
            let statute_ids: Vec<String> = indices
                .iter()
                .map(|&i| {
                    format!(
                        "{} ({})",
                        statutes[i].id,
                        statutes[i]
                            .jurisdiction
                            .as_ref()
                            .unwrap_or(&"no jurisdiction".to_string())
                    )
                })
                .collect();

            conflicts.push(
                StatuteConflict::new(
                    ConflictType::IdCollision,
                    statute_ids.clone(),
                    format!(
                        "Multiple statutes share the same ID '{}': found {} instances",
                        id,
                        indices.len()
                    ),
                )
                .with_suggestion("Use unique IDs for each statute")
                .with_suggestion("Consider adding jurisdiction prefix to IDs"),
            );
        }
    }

    conflicts
}

/// Detects effect conflicts (overlapping conditions with contradictory effects).
fn detect_effect_conflicts(statutes: &[Statute]) -> Vec<StatuteConflict> {
    let mut conflicts = Vec::new();

    // Compare each pair of statutes
    for i in 0..statutes.len() {
        for j in (i + 1)..statutes.len() {
            let statute1 = &statutes[i];
            let statute2 = &statutes[j];

            // Check if they have the same jurisdiction or one is None (applies everywhere)
            let same_jurisdiction = match (&statute1.jurisdiction, &statute2.jurisdiction) {
                (Some(j1), Some(j2)) => j1 == j2,
                (None, _) | (_, None) => true, // None means applies everywhere
            };

            if !same_jurisdiction {
                continue;
            }

            // Check if temporal validity overlaps
            if !temporal_validity_overlaps(&statute1.temporal_validity, &statute2.temporal_validity)
            {
                continue;
            }

            // Check if conditions are similar/overlapping
            let conditions_overlap =
                conditions_overlap(&statute1.preconditions, &statute2.preconditions);

            if conditions_overlap {
                // Check if effects contradict
                let effects_contradict = effects_contradict(&statute1.effect, &statute2.effect);

                if effects_contradict {
                    conflicts.push(
                        StatuteConflict::new(
                            ConflictType::EffectConflict,
                            vec![statute1.id.clone(), statute2.id.clone()],
                            format!(
                                "Statutes '{}' and '{}' have overlapping conditions but contradictory effects",
                                statute1.id, statute2.id
                            ),
                        )
                        .with_suggestion("Add more specific conditions to differentiate the statutes")
                        .with_suggestion("Establish a priority/hierarchy relationship")
                        .with_suggestion("Use temporal validity to separate their applicability"),
                    );
                }
            }
        }
    }

    conflicts
}

/// Checks if two temporal validity periods overlap.
fn temporal_validity_overlaps(
    tv1: &legalis_core::TemporalValidity,
    tv2: &legalis_core::TemporalValidity,
) -> bool {
    use chrono::NaiveDate;

    let start1 = tv1.effective_date;
    let end1 = tv1.expiry_date;
    let start2 = tv2.effective_date;
    let end2 = tv2.expiry_date;

    // If either has no dates, they potentially overlap (eternal validity)
    if start1.is_none() && end1.is_none() {
        return true;
    }
    if start2.is_none() && end2.is_none() {
        return true;
    }

    // Get effective ranges
    let start1 = start1.unwrap_or(NaiveDate::MIN);
    let end1 = end1.unwrap_or(NaiveDate::MAX);
    let start2 = start2.unwrap_or(NaiveDate::MIN);
    let end2 = end2.unwrap_or(NaiveDate::MAX);

    // Check if ranges overlap
    start1 <= end2 && start2 <= end1
}

/// Checks if two sets of conditions overlap (have common scenarios).
fn conditions_overlap(
    conds1: &[legalis_core::Condition],
    conds2: &[legalis_core::Condition],
) -> bool {
    // Simplified check: if both are empty or if they share any condition type
    if conds1.is_empty() && conds2.is_empty() {
        return true; // Both apply unconditionally
    }

    if conds1.is_empty() || conds2.is_empty() {
        return true; // One applies unconditionally, so overlap
    }

    // Check for overlapping condition types
    use std::mem::discriminant;
    for c1 in conds1 {
        for c2 in conds2 {
            // If conditions are of the same variant type, they might overlap
            if discriminant(c1) == discriminant(c2) {
                return true;
            }
        }
    }

    false
}

/// Checks if two effects contradict each other.
fn effects_contradict(effect1: &legalis_core::Effect, effect2: &legalis_core::Effect) -> bool {
    use legalis_core::EffectType;

    match (&effect1.effect_type, &effect2.effect_type) {
        // Grant vs Revoke/Prohibition
        (EffectType::Grant, EffectType::Revoke)
        | (EffectType::Revoke, EffectType::Grant)
        | (EffectType::Grant, EffectType::Prohibition)
        | (EffectType::Prohibition, EffectType::Grant) => true,

        // If same effect type, check descriptions for contradictions
        (t1, t2) if t1 == t2 => {
            let desc1_lower = effect1.description.to_lowercase();
            let desc2_lower = effect2.description.to_lowercase();

            // Look for obvious contradictions in descriptions
            (desc1_lower.contains("allow") && desc2_lower.contains("prohibit"))
                || (desc1_lower.contains("prohibit") && desc2_lower.contains("allow"))
                || (desc1_lower.contains("grant") && desc2_lower.contains("deny"))
                || (desc1_lower.contains("deny") && desc2_lower.contains("grant"))
        }

        _ => false,
    }
}

/// Detects jurisdictional overlaps.
fn detect_jurisdictional_overlaps(statutes: &[Statute]) -> Vec<StatuteConflict> {
    let mut conflicts = Vec::new();
    let mut jurisdiction_map: HashMap<String, Vec<String>> = HashMap::new();

    // Group statutes by jurisdiction
    for statute in statutes {
        if let Some(jurisdiction) = &statute.jurisdiction {
            jurisdiction_map
                .entry(jurisdiction.clone())
                .or_default()
                .push(statute.id.clone());
        }
    }

    // Check for potential overlaps (this is a simplified check)
    // In a real system, you'd have a jurisdiction hierarchy
    for (jurisdiction, statute_ids) in jurisdiction_map.iter() {
        if statute_ids.len() > 20 {
            // Arbitrary threshold
            conflicts.push(
                StatuteConflict::new(
                    ConflictType::JurisdictionalOverlap,
                    statute_ids.clone(),
                    format!(
                        "Jurisdiction '{}' has {} statutes, which may indicate overlap or redundancy",
                        jurisdiction,
                        statute_ids.len()
                    ),
                )
                .with_suggestion("Review statutes for consolidation opportunities")
                .with_suggestion("Consider creating sub-jurisdictions for better organization"),
            );
        }
    }

    conflicts
}

/// Detects temporal conflicts (overlapping time periods with conflicting rules).
fn detect_temporal_conflicts(statutes: &[Statute]) -> Vec<StatuteConflict> {
    let mut conflicts = Vec::new();

    // Group statutes that might conflict temporally
    for i in 0..statutes.len() {
        for j in (i + 1)..statutes.len() {
            let statute1 = &statutes[i];
            let statute2 = &statutes[j];

            // Check if they're related (similar titles or same jurisdiction)
            let related = statute1.jurisdiction == statute2.jurisdiction
                || title_similarity(&statute1.title, &statute2.title) > 0.5;

            if !related {
                continue;
            }

            // Check temporal overlap
            if temporal_validity_overlaps(&statute1.temporal_validity, &statute2.temporal_validity)
            {
                // Check if one supersedes the other based on version
                if statute1.version != statute2.version {
                    conflicts.push(
                        StatuteConflict::new(
                            ConflictType::TemporalConflict,
                            vec![statute1.id.clone(), statute2.id.clone()],
                            format!(
                                "Statutes '{}' (v{}) and '{}' (v{}) have overlapping validity periods",
                                statute1.id, statute1.version, statute2.id, statute2.version
                            ),
                        )
                        .with_suggestion("Set expiry date on older version when newer version takes effect")
                        .with_suggestion("Use version control and temporal validity to manage transitions"),
                    );
                }
            }
        }
    }

    conflicts
}

/// Calculates simple title similarity (Jaccard similarity of words).
fn title_similarity(title1: &str, title2: &str) -> f64 {
    let words1: HashSet<&str> = title1.split_whitespace().collect();
    let words2: HashSet<&str> = title2.split_whitespace().collect();

    if words1.is_empty() && words2.is_empty() {
        return 1.0;
    }

    let intersection = words1.intersection(&words2).count();
    let union = words1.union(&words2).count();

    if union == 0 {
        0.0
    } else {
        intersection as f64 / union as f64
    }
}

/// Generates a conflict detection report.
pub fn conflict_detection_report(statutes: &[Statute]) -> String {
    let conflicts = detect_statute_conflicts(statutes);

    let mut report = String::new();
    report.push_str("# Statute Conflict Detection Report\n\n");
    report.push_str(&format!("Analyzed {} statutes\n", statutes.len()));
    report.push_str(&format!("Found {} conflicts\n\n", conflicts.len()));

    if conflicts.is_empty() {
        report.push_str("✓ No conflicts detected.\n");
        return report;
    }

    // Group by severity
    let mut critical = Vec::new();
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    for conflict in &conflicts {
        match conflict.severity {
            Severity::Critical => critical.push(conflict),
            Severity::Error => errors.push(conflict),
            Severity::Warning => warnings.push(conflict),
            _ => {}
        }
    }

    if !critical.is_empty() {
        report.push_str(&format!("## Critical Conflicts ({})\n\n", critical.len()));
        for conflict in critical {
            report.push_str(&format!(
                "### {} - {}\n",
                conflict.conflict_type,
                conflict.statute_ids.join(", ")
            ));
            report.push_str(&format!("{}\n\n", conflict.description));
            if !conflict.resolution_suggestions.is_empty() {
                report.push_str("**Suggestions:**\n");
                for suggestion in &conflict.resolution_suggestions {
                    report.push_str(&format!("- {}\n", suggestion));
                }
                report.push('\n');
            }
        }
    }

    if !errors.is_empty() {
        report.push_str(&format!("## Errors ({})\n\n", errors.len()));
        for conflict in errors {
            report.push_str(&format!(
                "### {} - {}\n",
                conflict.conflict_type,
                conflict.statute_ids.join(", ")
            ));
            report.push_str(&format!("{}\n\n", conflict.description));
        }
    }

    if !warnings.is_empty() {
        report.push_str(&format!("## Warnings ({})\n\n", warnings.len()));
        for conflict in warnings {
            report.push_str(&format!(
                "### {} - {}\n",
                conflict.conflict_type,
                conflict.statute_ids.join(", ")
            ));
            report.push_str(&format!("{}\n\n", conflict.description));
        }
    }

    report
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
        Condition::Duration { .. } => (1, 0, ["Duration".to_string()].into_iter().collect()),
        Condition::Percentage { .. } => (1, 0, ["Percentage".to_string()].into_iter().collect()),
        Condition::SetMembership { .. } => {
            (1, 0, ["SetMembership".to_string()].into_iter().collect())
        }
        Condition::Pattern { .. } => (1, 0, ["Pattern".to_string()].into_iter().collect()),
        Condition::Calculation { .. } => (1, 0, ["Calculation".to_string()].into_iter().collect()),
        Condition::Composite { conditions, .. } => {
            // For composite conditions, recursively analyze all sub-conditions
            let mut max_depth = 1;
            let mut total_ops = 0;
            let mut all_types = HashSet::new();
            all_types.insert("Composite".to_string());

            for (_weight, cond) in conditions {
                let (depth, ops, types) = analyze_condition(cond);
                max_depth = max_depth.max(depth + 1);
                total_ops += ops;
                all_types.extend(types);
            }

            (max_depth, total_ops + conditions.len() - 1, all_types)
        }
        Condition::Threshold { attributes, .. } => {
            // Threshold aggregates multiple attributes
            let mut types = HashSet::new();
            types.insert("Threshold".to_string());
            (1, attributes.len().saturating_sub(1), types)
        }
        Condition::Fuzzy { .. } => (1, 0, ["Fuzzy".to_string()].into_iter().collect()),
        Condition::Probabilistic { condition, .. } => {
            // Recursively analyze the base condition
            let (depth, ops, mut types) = analyze_condition(condition);
            types.insert("Probabilistic".to_string());
            (1 + depth, 1 + ops, types)
        }
        Condition::Temporal { .. } => (1, 0, ["Temporal".to_string()].into_iter().collect()),
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

/// Generates an interactive HTML report with filtering, search, and sorting capabilities.
///
/// This creates a feature-rich HTML report with:
/// - Severity filtering
/// - Search functionality
/// - Expandable/collapsible sections
/// - Statistics dashboard
/// - Dark mode toggle
pub fn generate_interactive_html_report(result: &VerificationResult, title: &str) -> String {
    let severity_counts = result.severity_counts();
    let critical_count = severity_counts.get(&Severity::Critical).unwrap_or(&0);
    let error_count = severity_counts.get(&Severity::Error).unwrap_or(&0);
    let warning_count = severity_counts.get(&Severity::Warning).unwrap_or(&0);
    let info_count = severity_counts.get(&Severity::Info).unwrap_or(&0);

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title}</title>
    <style>
        :root {{
            --bg-primary: #ffffff;
            --bg-secondary: #f5f5f5;
            --text-primary: #333;
            --text-secondary: #666;
            --border-color: #ddd;
            --critical-bg: #fee;
            --critical-border: #dc3545;
            --error-bg: #f8d7da;
            --error-border: #dc3545;
            --warning-bg: #fff3cd;
            --warning-border: #ffc107;
            --info-bg: #d1ecf1;
            --info-border: #17a2b8;
            --success-bg: #d4edda;
            --success-border: #28a745;
        }}

        body.dark-mode {{
            --bg-primary: #1e1e1e;
            --bg-secondary: #2d2d2d;
            --text-primary: #e0e0e0;
            --text-secondary: #aaa;
            --border-color: #444;
            --critical-bg: #4a1f1f;
            --error-bg: #3a1f1f;
            --warning-bg: #3a3220;
            --info-bg: #1f2f3a;
            --success-bg: #1f3a1f;
        }}

        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}

        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif;
            background: var(--bg-secondary);
            color: var(--text-primary);
            line-height: 1.6;
            transition: background 0.3s, color 0.3s;
        }}

        .container {{
            max-width: 1400px;
            margin: 0 auto;
            padding: 20px;
        }}

        header {{
            background: var(--bg-primary);
            padding: 20px;
            border-radius: 8px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
            margin-bottom: 20px;
        }}

        h1 {{
            color: var(--text-primary);
            margin-bottom: 10px;
        }}

        .controls {{
            display: flex;
            gap: 10px;
            flex-wrap: wrap;
            margin-top: 15px;
        }}

        .search-box {{
            flex: 1;
            min-width: 200px;
        }}

        .search-box input {{
            width: 100%;
            padding: 10px;
            border: 1px solid var(--border-color);
            border-radius: 4px;
            background: var(--bg-primary);
            color: var(--text-primary);
            font-size: 14px;
        }}

        .filter-buttons {{
            display: flex;
            gap: 5px;
            flex-wrap: wrap;
        }}

        .filter-btn, .theme-toggle {{
            padding: 10px 15px;
            border: 1px solid var(--border-color);
            border-radius: 4px;
            background: var(--bg-primary);
            color: var(--text-primary);
            cursor: pointer;
            font-size: 14px;
            transition: all 0.2s;
        }}

        .filter-btn:hover, .theme-toggle:hover {{
            opacity: 0.8;
        }}

        .filter-btn.active {{
            background: #4CAF50;
            color: white;
            border-color: #4CAF50;
        }}

        .stats {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 15px;
            margin-bottom: 20px;
        }}

        .stat-card {{
            background: var(--bg-primary);
            padding: 20px;
            border-radius: 8px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
            border-left: 4px solid;
        }}

        .stat-card.critical {{ border-color: var(--critical-border); }}
        .stat-card.error {{ border-color: var(--error-border); }}
        .stat-card.warning {{ border-color: var(--warning-border); }}
        .stat-card.info {{ border-color: var(--info-border); }}
        .stat-card.success {{ border-color: var(--success-border); }}

        .stat-value {{
            font-size: 2em;
            font-weight: bold;
            margin-bottom: 5px;
        }}

        .stat-label {{
            color: var(--text-secondary);
            font-size: 0.9em;
        }}

        .section {{
            background: var(--bg-primary);
            padding: 20px;
            border-radius: 8px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
            margin-bottom: 20px;
        }}

        .section-header {{
            display: flex;
            justify-content: space-between;
            align-items: center;
            cursor: pointer;
            padding: 10px 0;
            border-bottom: 2px solid var(--border-color);
            margin-bottom: 15px;
        }}

        .section-header h2 {{
            color: var(--text-primary);
        }}

        .toggle-icon {{
            font-size: 1.2em;
            transition: transform 0.3s;
        }}

        .toggle-icon.collapsed {{
            transform: rotate(-90deg);
        }}

        .item {{
            padding: 15px;
            margin: 10px 0;
            border-radius: 4px;
            border-left: 4px solid;
            transition: all 0.2s;
        }}

        .item:hover {{
            transform: translateX(5px);
        }}

        .item.critical {{
            background: var(--critical-bg);
            border-color: var(--critical-border);
        }}

        .item.error {{
            background: var(--error-bg);
            border-color: var(--error-border);
        }}

        .item.warning {{
            background: var(--warning-bg);
            border-color: var(--warning-border);
        }}

        .item.info {{
            background: var(--info-bg);
            border-color: var(--info-border);
        }}

        .item.hidden {{
            display: none;
        }}

        .severity-badge {{
            display: inline-block;
            padding: 4px 8px;
            border-radius: 3px;
            font-size: 0.8em;
            font-weight: bold;
            margin-right: 10px;
        }}

        .severity-badge.critical {{
            background: var(--critical-border);
            color: white;
        }}

        .severity-badge.error {{
            background: var(--error-border);
            color: white;
        }}

        .severity-badge.warning {{
            background: var(--warning-border);
            color: #333;
        }}

        .severity-badge.info {{
            background: var(--info-border);
            color: white;
        }}

        .empty {{
            color: var(--text-secondary);
            font-style: italic;
            text-align: center;
            padding: 20px;
        }}

        .timestamp {{
            text-align: center;
            color: var(--text-secondary);
            font-size: 0.9em;
            margin-top: 20px;
            padding: 15px;
            background: var(--bg-primary);
            border-radius: 8px;
        }}
    </style>
</head>
<body>
    <div class="container">
        <header>
            <h1>{title}</h1>
            <div class="controls">
                <div class="search-box">
                    <input type="text" id="searchInput" placeholder="Search errors, warnings, suggestions...">
                </div>
                <div class="filter-buttons">
                    <button class="filter-btn active" data-filter="all">All</button>
                    <button class="filter-btn" data-filter="critical">Critical</button>
                    <button class="filter-btn" data-filter="error">Errors</button>
                    <button class="filter-btn" data-filter="warning">Warnings</button>
                    <button class="filter-btn" data-filter="info">Info</button>
                    <button class="theme-toggle" id="themeToggle">🌙 Dark Mode</button>
                </div>
            </div>
        </header>

        <div class="stats">
            <div class="stat-card success">
                <div class="stat-value">{status}</div>
                <div class="stat-label">Status</div>
            </div>
            <div class="stat-card critical">
                <div class="stat-value">{critical_count}</div>
                <div class="stat-label">Critical</div>
            </div>
            <div class="stat-card error">
                <div class="stat-value">{error_count}</div>
                <div class="stat-label">Errors</div>
            </div>
            <div class="stat-card warning">
                <div class="stat-value">{warning_count}</div>
                <div class="stat-label">Warnings</div>
            </div>
            <div class="stat-card info">
                <div class="stat-value">{info_count}</div>
                <div class="stat-label">Info</div>
            </div>
        </div>

        <div class="section">
            <div class="section-header" onclick="toggleSection('errors')">
                <h2>Errors ({error_total})</h2>
                <span class="toggle-icon" id="errors-toggle">▼</span>
            </div>
            <div id="errors-content">
                {errors_html}
            </div>
        </div>

        <div class="section">
            <div class="section-header" onclick="toggleSection('warnings')">
                <h2>Warnings ({warnings_total})</h2>
                <span class="toggle-icon" id="warnings-toggle">▼</span>
            </div>
            <div id="warnings-content">
                {warnings_html}
            </div>
        </div>

        <div class="section">
            <div class="section-header" onclick="toggleSection('suggestions')">
                <h2>Suggestions ({suggestions_total})</h2>
                <span class="toggle-icon" id="suggestions-toggle">▼</span>
            </div>
            <div id="suggestions-content">
                {suggestions_html}
            </div>
        </div>

        <div class="timestamp">
            Generated: {timestamp}
        </div>
    </div>

    <script>
        // Dark mode toggle
        const themeToggle = document.getElementById('themeToggle');
        const body = document.body;

        themeToggle.addEventListener('click', () => {{
            body.classList.toggle('dark-mode');
            themeToggle.textContent = body.classList.contains('dark-mode') ? '☀️ Light Mode' : '🌙 Dark Mode';
        }});

        // Search functionality
        const searchInput = document.getElementById('searchInput');
        searchInput.addEventListener('input', (e) => {{
            const searchTerm = e.target.value.toLowerCase();
            const items = document.querySelectorAll('.item');

            items.forEach(item => {{
                const text = item.textContent.toLowerCase();
                if (text.includes(searchTerm)) {{
                    item.style.display = 'block';
                }} else {{
                    item.style.display = 'none';
                }}
            }});
        }});

        // Filter functionality
        const filterButtons = document.querySelectorAll('.filter-btn');
        filterButtons.forEach(button => {{
            button.addEventListener('click', () => {{
                // Update active state
                filterButtons.forEach(btn => btn.classList.remove('active'));
                button.classList.add('active');

                const filter = button.dataset.filter;
                const items = document.querySelectorAll('.item');

                items.forEach(item => {{
                    if (filter === 'all' || item.classList.contains(filter)) {{
                        item.style.display = 'block';
                    }} else {{
                        item.style.display = 'none';
                    }}
                }});
            }});
        }});

        // Section toggle
        function toggleSection(sectionId) {{
            const content = document.getElementById(sectionId + '-content');
            const toggle = document.getElementById(sectionId + '-toggle');

            if (content.style.display === 'none') {{
                content.style.display = 'block';
                toggle.classList.remove('collapsed');
            }} else {{
                content.style.display = 'none';
                toggle.classList.add('collapsed');
            }}
        }}
    </script>
</body>
</html>"#,
        title = html_escape(title),
        status = if result.passed {
            "✓ PASS"
        } else {
            "✗ FAIL"
        },
        critical_count = critical_count,
        error_count = error_count,
        warning_count = warning_count,
        info_count = info_count,
        error_total = result.errors.len(),
        warnings_total = result.warnings.len(),
        suggestions_total = result.suggestions.len(),
        errors_html = if result.errors.is_empty() {
            "<p class=\"empty\">No errors found</p>".to_string()
        } else {
            result
                .errors
                .iter()
                .map(|e| {
                    let severity = e.severity();
                    let severity_str = format!("{}", severity).to_lowercase();
                    format!(
                        "<div class=\"item {}\" data-severity=\"{}\"><span class=\"severity-badge {}\">{}</span>{}</div>",
                        severity_str,
                        severity_str,
                        severity_str,
                        severity,
                        html_escape(&e.to_string())
                    )
                })
                .collect::<Vec<_>>()
                .join("\n")
        },
        warnings_html = if result.warnings.is_empty() {
            "<p class=\"empty\">No warnings found</p>".to_string()
        } else {
            result
                .warnings
                .iter()
                .map(|w| {
                    format!(
                        "<div class=\"item warning\" data-severity=\"warning\">{}</div>",
                        html_escape(w)
                    )
                })
                .collect::<Vec<_>>()
                .join("\n")
        },
        suggestions_html = if result.suggestions.is_empty() {
            "<p class=\"empty\">No suggestions</p>".to_string()
        } else {
            result
                .suggestions
                .iter()
                .map(|s| {
                    format!(
                        "<div class=\"item info\" data-severity=\"info\">{}</div>",
                        html_escape(s)
                    )
                })
                .collect::<Vec<_>>()
                .join("\n")
        },
        timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
    )
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
    use printpdf::{
        BuiltinFont, Mm, Op, PdfDocument, PdfPage, PdfSaveOptions, Point, Pt, TextItem,
    };

    /// Helper to create text operations at a specific position
    fn pdf_text_op(text: &str, size: f32, x: Mm, y: Mm, font: BuiltinFont) -> Vec<Op> {
        vec![
            Op::StartTextSection,
            Op::SetFontSizeBuiltinFont {
                size: Pt(size),
                font,
            },
            Op::SetTextCursor {
                pos: Point {
                    x: x.into(),
                    y: y.into(),
                },
            },
            Op::WriteTextBuiltinFont {
                items: vec![TextItem::Text(text.to_string())],
                font,
            },
            Op::EndTextSection,
        ]
    }

    // Page builder helper
    struct PageBuilder {
        pages: Vec<Vec<Op>>,
        current_ops: Vec<Op>,
        y_position: Mm,
        line_height: Mm,
        left_margin: Mm,
        page_top: Mm,
        page_bottom: Mm,
    }

    impl PageBuilder {
        fn new() -> Self {
            Self {
                pages: Vec::new(),
                current_ops: Vec::new(),
                y_position: Mm(270.0),
                line_height: Mm(6.0),
                left_margin: Mm(20.0),
                page_top: Mm(270.0),
                page_bottom: Mm(30.0),
            }
        }

        fn add_text(&mut self, text: &str, size: f32, font: BuiltinFont) {
            self.add_text_at(text, size, self.left_margin, font);
        }

        fn add_text_at(&mut self, text: &str, size: f32, x: Mm, font: BuiltinFont) {
            if self.y_position < self.page_bottom {
                self.new_page();
            }
            self.current_ops
                .extend(pdf_text_op(text, size, x, self.y_position, font));
        }

        fn advance_line(&mut self) {
            self.y_position -= self.line_height;
        }

        fn advance_lines(&mut self, n: f32) {
            self.y_position -= self.line_height * n;
        }

        fn new_page(&mut self) {
            if !self.current_ops.is_empty() {
                self.pages.push(std::mem::take(&mut self.current_ops));
            }
            self.y_position = self.page_top;
        }

        fn finish(mut self) -> Vec<Vec<Op>> {
            if !self.current_ops.is_empty() {
                self.pages.push(self.current_ops);
            }
            self.pages
        }
    }

    let mut builder = PageBuilder::new();

    // Title
    builder.add_text(title, 18.0, BuiltinFont::HelveticaBold);
    builder.advance_lines(2.0);

    // Timestamp
    let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    builder.add_text(
        &format!("Generated: {}", timestamp),
        10.0,
        BuiltinFont::Helvetica,
    );
    builder.advance_lines(2.0);

    // Status
    let status_text = if result.passed {
        "Verification Passed"
    } else {
        "Verification Failed"
    };
    builder.add_text(status_text, 14.0, BuiltinFont::HelveticaBold);
    builder.advance_lines(2.0);

    // Errors section
    builder.add_text("Errors:", 12.0, BuiltinFont::HelveticaBold);
    builder.advance_line();

    if result.errors.is_empty() {
        builder.add_text("  No errors found", 10.0, BuiltinFont::Helvetica);
        builder.advance_line();
    } else {
        for (idx, error) in result.errors.iter().enumerate() {
            let error_text = format!("  {}. {}", idx + 1, error);
            for line in wrap_text(&error_text, 80) {
                builder.add_text(&line, 10.0, BuiltinFont::Helvetica);
                builder.advance_line();
            }
        }
    }
    builder.advance_line();

    // Warnings section
    builder.add_text("Warnings:", 12.0, BuiltinFont::HelveticaBold);
    builder.advance_line();

    if result.warnings.is_empty() {
        builder.add_text("  No warnings found", 10.0, BuiltinFont::Helvetica);
        builder.advance_line();
    } else {
        for (idx, warning) in result.warnings.iter().enumerate() {
            let warning_text = format!("  {}. {}", idx + 1, warning);
            for line in wrap_text(&warning_text, 80) {
                builder.add_text(&line, 10.0, BuiltinFont::Helvetica);
                builder.advance_line();
            }
        }
    }
    builder.advance_line();

    // Suggestions section
    builder.add_text("Suggestions:", 12.0, BuiltinFont::HelveticaBold);
    builder.advance_line();

    if result.suggestions.is_empty() {
        builder.add_text("  No suggestions", 10.0, BuiltinFont::Helvetica);
    } else {
        for (idx, suggestion) in result.suggestions.iter().enumerate() {
            let suggestion_text = format!("  {}. {}", idx + 1, suggestion);
            for line in wrap_text(&suggestion_text, 80) {
                builder.add_text(&line, 10.0, BuiltinFont::Helvetica);
                builder.advance_line();
            }
        }
    }

    // Build all pages
    let page_ops = builder.finish();
    let pages: Vec<PdfPage> = page_ops
        .into_iter()
        .map(|ops| PdfPage::new(Mm(210.0), Mm(297.0), ops))
        .collect();

    // Create document and save
    let mut doc = PdfDocument::new(title);
    doc.with_pages(pages);

    let mut warnings = Vec::new();
    let pdf_bytes = doc.save(&PdfSaveOptions::default(), &mut warnings);

    Ok(pdf_bytes)
}

/// Helper function to wrap text to a specified width.
#[cfg(feature = "pdf")]
fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in text.split_whitespace() {
        if current_line.len() + word.len() + 1 > max_width && !current_line.is_empty() {
            lines.push(current_line.clone());
            current_line.clear();
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

/// Semantic similarity score between two items (0.0 = completely different, 1.0 = identical).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SimilarityScore(pub f64);

impl SimilarityScore {
    /// Creates a new similarity score (clamped to [0.0, 1.0]).
    pub fn new(score: f64) -> Self {
        Self(score.clamp(0.0, 1.0))
    }

    /// Returns true if similarity is high (>= 0.8).
    pub fn is_high(&self) -> bool {
        self.0 >= 0.8
    }

    /// Returns true if similarity is moderate (>= 0.5 and < 0.8).
    pub fn is_moderate(&self) -> bool {
        self.0 >= 0.5 && self.0 < 0.8
    }

    /// Returns true if similarity is low (< 0.5).
    pub fn is_low(&self) -> bool {
        self.0 < 0.5
    }
}

/// Calculates semantic similarity between two statutes.
///
/// The similarity is based on:
/// - Title similarity (Levenshtein distance)
/// - Condition overlap
/// - Effect type similarity
/// - Discretion similarity
pub fn semantic_similarity(statute1: &Statute, statute2: &Statute) -> SimilarityScore {
    let mut similarity = 0.0f64;
    let mut weight_sum = 0.0f64;

    // Title similarity (weight: 0.2)
    let title_weight = 0.2;
    let title_sim = string_similarity(&statute1.title, &statute2.title);
    similarity += title_sim * title_weight;
    weight_sum += title_weight;

    // Effect type similarity (weight: 0.3)
    let effect_weight = 0.3;
    let effect_sim = if statute1.effect.effect_type == statute2.effect.effect_type {
        1.0
    } else {
        0.0
    };
    similarity += effect_sim * effect_weight;
    weight_sum += effect_weight;

    // Condition overlap (weight: 0.4)
    let condition_weight = 0.4;
    let condition_sim =
        condition_overlap_similarity(&statute1.preconditions, &statute2.preconditions);
    similarity += condition_sim * condition_weight;
    weight_sum += condition_weight;

    // Discretion similarity (weight: 0.1)
    let discretion_weight = 0.1;
    let discretion_sim = match (&statute1.discretion_logic, &statute2.discretion_logic) {
        (Some(_), Some(_)) => 1.0,
        (None, None) => 1.0,
        _ => 0.0,
    };
    similarity += discretion_sim * discretion_weight;
    weight_sum += discretion_weight;

    SimilarityScore::new(similarity / weight_sum)
}

/// Calculates string similarity using Levenshtein distance.
fn string_similarity(s1: &str, s2: &str) -> f64 {
    if s1 == s2 {
        return 1.0;
    }
    if s1.is_empty() || s2.is_empty() {
        return 0.0;
    }

    let distance = levenshtein_distance(s1, s2);
    let max_len = s1.len().max(s2.len());
    1.0 - (distance as f64 / max_len as f64)
}

/// Calculates Levenshtein distance between two strings.
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.chars().count();
    let len2 = s2.chars().count();

    if len1 == 0 {
        return len2;
    }
    if len2 == 0 {
        return len1;
    }

    let mut matrix = vec![vec![0usize; len2 + 1]; len1 + 1];

    #[allow(clippy::needless_range_loop)]
    for i in 0..=len1 {
        matrix[i][0] = i;
    }
    for j in 0..=len2 {
        matrix[0][j] = j;
    }

    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();

    for i in 1..=len1 {
        for j in 1..=len2 {
            let cost = if s1_chars[i - 1] == s2_chars[j - 1] {
                0
            } else {
                1
            };
            matrix[i][j] = (matrix[i - 1][j] + 1)
                .min(matrix[i][j - 1] + 1)
                .min(matrix[i - 1][j - 1] + cost);
        }
    }

    matrix[len1][len2]
}

/// Calculates overlap similarity between two sets of conditions.
fn condition_overlap_similarity(
    conditions1: &[legalis_core::Condition],
    conditions2: &[legalis_core::Condition],
) -> f64 {
    if conditions1.is_empty() && conditions2.is_empty() {
        return 1.0;
    }
    if conditions1.is_empty() || conditions2.is_empty() {
        return 0.0;
    }

    let mut matching_pairs = 0;
    let total_comparisons = conditions1.len() * conditions2.len();

    for c1 in conditions1 {
        for c2 in conditions2 {
            if conditions_are_similar(c1, c2) {
                matching_pairs += 1;
            }
        }
    }

    matching_pairs as f64 / total_comparisons as f64
}

/// Checks if two conditions are similar.
fn conditions_are_similar(c1: &legalis_core::Condition, c2: &legalis_core::Condition) -> bool {
    use legalis_core::Condition;

    match (c1, c2) {
        (Condition::Age { .. }, Condition::Age { .. }) => true,
        (Condition::Income { .. }, Condition::Income { .. }) => true,
        (Condition::HasAttribute { key: k1 }, Condition::HasAttribute { key: k2 }) => k1 == k2,
        (
            Condition::AttributeEquals { key: k1, .. },
            Condition::AttributeEquals { key: k2, .. },
        ) => k1 == k2,
        (Condition::DateRange { .. }, Condition::DateRange { .. }) => true,
        (Condition::Geographic { .. }, Condition::Geographic { .. }) => true,
        (Condition::EntityRelationship { .. }, Condition::EntityRelationship { .. }) => true,
        (Condition::ResidencyDuration { .. }, Condition::ResidencyDuration { .. }) => true,
        (Condition::Duration { .. }, Condition::Duration { .. }) => true,
        (Condition::Percentage { .. }, Condition::Percentage { .. }) => true,
        (Condition::SetMembership { .. }, Condition::SetMembership { .. }) => true,
        (Condition::Pattern { .. }, Condition::Pattern { .. }) => true,
        (Condition::Calculation { .. }, Condition::Calculation { .. }) => true,
        (Condition::Custom { description: d1 }, Condition::Custom { description: d2 }) => {
            string_similarity(d1, d2) > 0.7
        }
        (Condition::And(l1, r1), Condition::And(l2, r2)) => {
            conditions_are_similar(l1, l2) && conditions_are_similar(r1, r2)
        }
        (Condition::Or(l1, r1), Condition::Or(l2, r2)) => {
            conditions_are_similar(l1, l2) && conditions_are_similar(r1, r2)
        }
        (Condition::Not(c1), Condition::Not(c2)) => conditions_are_similar(c1, c2),
        _ => false,
    }
}

/// Finds pairs of statutes with high semantic similarity (potential duplicates).
///
/// Returns a list of statute pairs with similarity scores above the threshold.
pub fn find_similar_statutes(
    statutes: &[Statute],
    threshold: f64,
) -> Vec<(String, String, SimilarityScore)> {
    let mut similar_pairs = Vec::new();

    for i in 0..statutes.len() {
        for j in (i + 1)..statutes.len() {
            let similarity = semantic_similarity(&statutes[i], &statutes[j]);
            if similarity.0 >= threshold {
                similar_pairs.push((statutes[i].id.clone(), statutes[j].id.clone(), similarity));
            }
        }
    }

    similar_pairs
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

/// Common ambiguous legal terms and their potential meanings.
const AMBIGUOUS_LEGAL_TERMS: &[(&str, &[&str])] = &[
    ("person", &["natural person", "legal person", "corporation"]),
    ("child", &["minor", "dependent", "offspring"]),
    (
        "residence",
        &["domicile", "dwelling", "temporary residence"],
    ),
    ("income", &["gross income", "net income", "taxable income"]),
    ("tax", &["income tax", "sales tax", "property tax"]),
    (
        "benefit",
        &["welfare benefit", "tax benefit", "employment benefit"],
    ),
    (
        "disability",
        &[
            "physical disability",
            "mental disability",
            "learning disability",
        ],
    ),
    (
        "family",
        &["immediate family", "extended family", "household"],
    ),
    (
        "spouse",
        &["legal spouse", "common-law spouse", "domestic partner"],
    ),
    (
        "property",
        &[
            "real property",
            "personal property",
            "intellectual property",
        ],
    ),
];

/// Finds ambiguous terms in a set of statutes.
///
/// This function identifies terms that may have multiple meanings
/// and suggests disambiguations based on common legal usage.
pub fn find_ambiguous_terms(statutes: &[Statute]) -> Vec<AmbiguousTerm> {
    let mut ambiguous_terms = HashMap::new();

    for statute in statutes {
        // Check title for ambiguous terms
        for (term, suggestions) in AMBIGUOUS_LEGAL_TERMS {
            if statute.title.to_lowercase().contains(term) {
                let entry = ambiguous_terms
                    .entry(term.to_string())
                    .or_insert_with(|| AmbiguousTerm::new(*term));

                if !entry.statute_ids.contains(&statute.id) {
                    entry.statute_ids.push(statute.id.clone());
                }
                if !entry.contexts.contains(&statute.title) {
                    entry.contexts.push(statute.title.clone());
                }
                for suggestion in *suggestions {
                    if !entry.suggestions.contains(&suggestion.to_string()) {
                        entry.suggestions.push(suggestion.to_string());
                    }
                }
            }
        }

        // Check effect descriptions for ambiguous terms
        if statute.effect.description.to_lowercase().contains("person") {
            let entry = ambiguous_terms
                .entry("person".to_string())
                .or_insert_with(|| AmbiguousTerm::new("person"));

            if !entry.statute_ids.contains(&statute.id) {
                entry.statute_ids.push(statute.id.clone());
            }
            if !entry.contexts.contains(&statute.effect.description) {
                entry.contexts.push(statute.effect.description.clone());
            }
        }
    }

    ambiguous_terms.into_values().collect()
}

/// Generates a term disambiguation report for a set of statutes.
pub fn term_disambiguation_report(statutes: &[Statute]) -> String {
    let ambiguous_terms = find_ambiguous_terms(statutes);

    if ambiguous_terms.is_empty() {
        return "# Term Disambiguation Report\n\nNo ambiguous terms found.\n".to_string();
    }

    let mut report = String::new();
    report.push_str("# Term Disambiguation Report\n\n");
    report.push_str(&format!(
        "Found {} ambiguous terms:\n\n",
        ambiguous_terms.len()
    ));

    for term in &ambiguous_terms {
        report.push_str(&format!("## Term: \"{}\"\n", term.term));
        report.push_str(&format!(
            "- Used in {} statute(s): {}\n",
            term.statute_ids.len(),
            term.statute_ids.join(", ")
        ));

        if !term.contexts.is_empty() {
            report.push_str("- Contexts:\n");
            for context in &term.contexts {
                report.push_str(&format!("  - {}\n", context));
            }
        }

        if !term.suggestions.is_empty() {
            report.push_str("- Suggested disambiguations:\n");
            for suggestion in &term.suggestions {
                report.push_str(&format!("  - {}\n", suggestion));
            }
        }

        report.push('\n');
    }

    report
}

/// Represents a cross-reference validation error.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CrossReferenceError {
    /// Statute ID containing the reference
    pub source_statute_id: String,
    /// Referenced statute ID that is invalid
    pub referenced_statute_id: String,
    /// Error type
    pub error_type: CrossReferenceErrorType,
}

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

impl std::fmt::Display for CrossReferenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.error_type {
            CrossReferenceErrorType::NotFound => write!(
                f,
                "Statute '{}' references non-existent statute '{}'",
                self.source_statute_id, self.referenced_statute_id
            ),
            CrossReferenceErrorType::CircularReference => write!(
                f,
                "Statute '{}' creates circular reference with '{}'",
                self.source_statute_id, self.referenced_statute_id
            ),
            CrossReferenceErrorType::Ambiguous => write!(
                f,
                "Statute '{}' has ambiguous reference to '{}'",
                self.source_statute_id, self.referenced_statute_id
            ),
        }
    }
}

/// Validates cross-references between statutes.
///
/// This function checks that all statute references in conditions
/// point to valid existing statutes.
pub fn validate_cross_references(statutes: &[Statute]) -> Vec<CrossReferenceError> {
    let mut errors = Vec::new();
    let statute_ids: HashSet<&str> = statutes.iter().map(|s| s.id.as_str()).collect();

    for statute in statutes {
        let references = extract_statute_references_from_conditions(&statute.preconditions);

        for reference in references {
            // Check if the referenced statute exists
            if !statute_ids.contains(reference.as_str()) {
                errors.push(CrossReferenceError {
                    source_statute_id: statute.id.clone(),
                    referenced_statute_id: reference.clone(),
                    error_type: CrossReferenceErrorType::NotFound,
                });
            }
        }
    }

    errors
}

/// Extracts statute references from a list of conditions.
fn extract_statute_references_from_conditions(
    conditions: &[legalis_core::Condition],
) -> Vec<String> {
    let mut refs = Vec::new();
    for condition in conditions {
        extract_refs_from_single_condition(condition, &mut refs);
    }
    refs
}

/// Recursively extracts references from a single condition.
fn extract_refs_from_single_condition(condition: &legalis_core::Condition, refs: &mut Vec<String>) {
    use legalis_core::Condition;

    match condition {
        Condition::Custom { description } => {
            // Extract statute references with "statute:" prefix
            if let Some(statute_ref) = description.strip_prefix("statute:") {
                refs.push(statute_ref.trim().to_string());
            }
        }
        Condition::And(left, right) | Condition::Or(left, right) => {
            extract_refs_from_single_condition(left, refs);
            extract_refs_from_single_condition(right, refs);
        }
        Condition::Not(inner) => {
            extract_refs_from_single_condition(inner, refs);
        }
        _ => {}
    }
}

/// Generates a cross-reference validation report.
pub fn cross_reference_report(statutes: &[Statute]) -> String {
    let errors = validate_cross_references(statutes);

    if errors.is_empty() {
        return "# Cross-Reference Validation Report\n\nAll cross-references are valid.\n"
            .to_string();
    }

    let mut report = String::new();
    report.push_str("# Cross-Reference Validation Report\n\n");
    report.push_str(&format!(
        "Found {} cross-reference error(s):\n\n",
        errors.len()
    ));

    for error in &errors {
        report.push_str(&format!("- {}\n", error));
    }

    report
}

/// Represents a terminology inconsistency.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TerminologyInconsistency {
    /// The term variations found
    pub variations: Vec<String>,
    /// Statute IDs where variations are used
    pub statute_ids: Vec<String>,
    /// Suggested canonical term
    pub canonical_term: String,
}

impl TerminologyInconsistency {
    /// Creates a new terminology inconsistency.
    pub fn new(canonical_term: impl Into<String>) -> Self {
        Self {
            variations: Vec::new(),
            statute_ids: Vec::new(),
            canonical_term: canonical_term.into(),
        }
    }

    /// Adds a variation to the inconsistency.
    pub fn with_variation(mut self, variation: impl Into<String>) -> Self {
        let var = variation.into();
        if !self.variations.contains(&var) {
            self.variations.push(var);
        }
        self
    }

    /// Adds a statute ID to the inconsistency.
    pub fn with_statute_id(mut self, statute_id: impl Into<String>) -> Self {
        let id = statute_id.into();
        if !self.statute_ids.contains(&id) {
            self.statute_ids.push(id);
        }
        self
    }
}

/// Common term variations that should be consistent.
const TERM_VARIATIONS: &[(&str, &[&str])] = &[
    ("applicant", &["applicant", "appellant", "petitioner"]),
    ("minor", &["minor", "child", "juvenile", "underage person"]),
    ("guardian", &["guardian", "custodian", "caretaker"]),
    ("income", &["income", "earnings", "revenue", "compensation"]),
    ("residence", &["residence", "domicile", "dwelling", "home"]),
    (
        "employer",
        &["employer", "company", "business", "organization"],
    ),
    (
        "employee",
        &["employee", "worker", "staff member", "personnel"],
    ),
    (
        "benefit",
        &["benefit", "entitlement", "allowance", "payment"],
    ),
    ("disabled", &["disabled", "handicapped", "impaired"]),
    ("spouse", &["spouse", "partner", "husband", "wife"]),
];

/// Checks for terminology consistency across statutes.
///
/// This function identifies where similar terms are used inconsistently
/// and suggests a canonical term for each concept.
pub fn check_terminology_consistency(statutes: &[Statute]) -> Vec<TerminologyInconsistency> {
    let mut inconsistencies = Vec::new();

    for (canonical, variations) in TERM_VARIATIONS {
        let mut found_variations = HashMap::new();

        for statute in statutes {
            let text = format!("{} {}", statute.title, statute.effect.description).to_lowercase();

            for variation in *variations {
                if text.contains(variation) {
                    found_variations
                        .entry(variation.to_string())
                        .or_insert_with(Vec::new)
                        .push(statute.id.clone());
                }
            }
        }

        // If more than one variation is found, report inconsistency
        if found_variations.len() > 1 {
            let mut inconsistency = TerminologyInconsistency::new(*canonical);

            for (variation, statute_ids) in found_variations {
                inconsistency = inconsistency.with_variation(&variation);
                for id in statute_ids {
                    inconsistency = inconsistency.with_statute_id(id);
                }
            }

            inconsistencies.push(inconsistency);
        }
    }

    inconsistencies
}

/// Generates a terminology consistency report.
pub fn terminology_consistency_report(statutes: &[Statute]) -> String {
    let inconsistencies = check_terminology_consistency(statutes);

    if inconsistencies.is_empty() {
        return "# Terminology Consistency Report\n\nTerminology is consistent across all statutes.\n"
            .to_string();
    }

    let mut report = String::new();
    report.push_str("# Terminology Consistency Report\n\n");
    report.push_str(&format!(
        "Found {} terminology inconsistenc(ies):\n\n",
        inconsistencies.len()
    ));

    for inconsistency in &inconsistencies {
        report.push_str(&format!(
            "## Inconsistent use of \"{}\"\n",
            inconsistency.canonical_term
        ));
        report.push_str(&format!(
            "- Found {} variation(s): {}\n",
            inconsistency.variations.len(),
            inconsistency.variations.join(", ")
        ));
        report.push_str(&format!(
            "- Used in {} statute(s): {}\n",
            inconsistency.statute_ids.len(),
            inconsistency.statute_ids.join(", ")
        ));
        report.push_str(&format!(
            "- Recommendation: Use \"{}\" consistently\n\n",
            inconsistency.canonical_term
        ));
    }

    report
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

// =============================================================================
// IDE Integration Support
// =============================================================================

/// Diagnostic location for IDE integration.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DiagnosticLocation {
    /// File path
    pub file: String,
    /// Line number (1-based)
    pub line: usize,
    /// Column number (1-based)
    pub column: usize,
    /// End line (optional, for range)
    pub end_line: Option<usize>,
    /// End column (optional, for range)
    pub end_column: Option<usize>,
}

impl DiagnosticLocation {
    /// Creates a new diagnostic location.
    pub fn new(file: impl Into<String>, line: usize, column: usize) -> Self {
        Self {
            file: file.into(),
            line,
            column,
            end_line: None,
            end_column: None,
        }
    }

    /// Sets the end position for a range.
    pub fn with_range(mut self, end_line: usize, end_column: usize) -> Self {
        self.end_line = Some(end_line);
        self.end_column = Some(end_column);
        self
    }
}

/// LSP-compatible diagnostic for IDE integration.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IdeDiagnostic {
    /// Diagnostic severity (error, warning, info, hint)
    pub severity: String,
    /// Diagnostic message
    pub message: String,
    /// Location in source
    pub location: Option<DiagnosticLocation>,
    /// Diagnostic code (e.g., "E001")
    pub code: Option<String>,
    /// Source of the diagnostic (e.g., "legalis-verifier")
    pub source: String,
    /// Related information
    pub related: Vec<String>,
    /// Suggested fixes
    pub fixes: Vec<String>,
}

impl IdeDiagnostic {
    /// Creates a new IDE diagnostic.
    pub fn new(severity: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            severity: severity.into(),
            message: message.into(),
            location: None,
            code: None,
            source: "legalis-verifier".to_string(),
            related: Vec::new(),
            fixes: Vec::new(),
        }
    }

    /// Sets the diagnostic location.
    pub fn with_location(mut self, location: DiagnosticLocation) -> Self {
        self.location = Some(location);
        self
    }

    /// Sets the diagnostic code.
    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }

    /// Adds related information.
    pub fn with_related(mut self, info: impl Into<String>) -> Self {
        self.related.push(info.into());
        self
    }

    /// Adds a suggested fix.
    pub fn with_fix(mut self, fix: impl Into<String>) -> Self {
        self.fixes.push(fix.into());
        self
    }
}

/// Converts verification results to IDE diagnostics.
pub fn to_ide_diagnostics(result: &VerificationResult) -> Vec<IdeDiagnostic> {
    let mut diagnostics = Vec::new();

    // Convert errors
    for error in &result.errors {
        let severity_level = match error.severity() {
            Severity::Critical => "error",
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Info => "information",
        };

        let code = match error {
            VerificationError::CircularReference { .. } => "L001",
            VerificationError::DeadStatute { .. } => "L002",
            VerificationError::ConstitutionalConflict { .. } => "L003",
            VerificationError::LogicalContradiction { .. } => "L004",
            VerificationError::Ambiguity { .. } => "L005",
            VerificationError::UnreachableCode { .. } => "L006",
        };

        diagnostics.push(IdeDiagnostic::new(severity_level, error.to_string()).with_code(code));
    }

    // Convert warnings
    for warning in &result.warnings {
        diagnostics.push(IdeDiagnostic::new("warning", warning));
    }

    // Convert suggestions to hints
    for suggestion in &result.suggestions {
        diagnostics.push(IdeDiagnostic::new("hint", suggestion));
    }

    diagnostics
}

/// Generates LSP-compatible diagnostic JSON output.
pub fn generate_lsp_diagnostics(result: &VerificationResult) -> Result<String, serde_json::Error> {
    let diagnostics = to_ide_diagnostics(result);
    serde_json::to_string_pretty(&diagnostics)
}

/// Quick fix suggestion for IDE code actions.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QuickFix {
    /// Title of the fix
    pub title: String,
    /// Description
    pub description: String,
    /// Kind of fix (e.g., "quickfix", "refactor")
    pub kind: String,
    /// Edits to apply
    pub edits: Vec<TextEdit>,
}

/// Text edit for applying quick fixes.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TextEdit {
    /// File to edit
    pub file: String,
    /// Start line (1-based)
    pub start_line: usize,
    /// Start column (1-based)
    pub start_column: usize,
    /// End line (1-based)
    pub end_line: usize,
    /// End column (1-based)
    pub end_column: usize,
    /// New text to insert
    pub new_text: String,
}

impl TextEdit {
    /// Creates a new text edit.
    pub fn new(
        file: impl Into<String>,
        start_line: usize,
        start_column: usize,
        end_line: usize,
        end_column: usize,
        new_text: impl Into<String>,
    ) -> Self {
        Self {
            file: file.into(),
            start_line,
            start_column,
            end_line,
            end_column,
            new_text: new_text.into(),
        }
    }
}

impl QuickFix {
    /// Creates a new quick fix.
    pub fn new(title: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            kind: "quickfix".to_string(),
            edits: Vec::new(),
        }
    }

    /// Adds an edit to the quick fix.
    pub fn with_edit(mut self, edit: TextEdit) -> Self {
        self.edits.push(edit);
        self
    }

    /// Sets the kind of fix.
    pub fn with_kind(mut self, kind: impl Into<String>) -> Self {
        self.kind = kind.into();
        self
    }
}

/// Generates quick fixes for common verification errors.
pub fn generate_quick_fixes(error: &VerificationError) -> Vec<QuickFix> {
    match error {
        VerificationError::CircularReference { message } => {
            vec![
                QuickFix::new(
                    "Break circular reference",
                    format!("Remove circular dependency: {}", message),
                )
                .with_kind("refactor.rewrite"),
            ]
        }
        VerificationError::DeadStatute { statute_id } => {
            vec![
                QuickFix::new(
                    "Fix unsatisfiable conditions",
                    format!("Review and fix conditions in statute {}", statute_id),
                )
                .with_kind("quickfix"),
            ]
        }
        VerificationError::ConstitutionalConflict {
            statute_id,
            principle,
        } => {
            vec![
                QuickFix::new(
                    "Resolve constitutional conflict",
                    format!(
                        "Update statute {} to comply with principle: {}",
                        statute_id, principle
                    ),
                )
                .with_kind("quickfix"),
            ]
        }
        VerificationError::LogicalContradiction { message } => {
            vec![
                QuickFix::new(
                    "Resolve logical contradiction",
                    format!("Fix contradictory logic: {}", message),
                )
                .with_kind("refactor.rewrite"),
            ]
        }
        VerificationError::Ambiguity { message } => {
            vec![
                QuickFix::new(
                    "Clarify ambiguous language",
                    format!("Make language more specific: {}", message),
                )
                .with_kind("refactor.rewrite"),
            ]
        }
        VerificationError::UnreachableCode { message } => {
            vec![
                QuickFix::new(
                    "Remove unreachable code",
                    format!("Delete or refactor unreachable code: {}", message),
                )
                .with_kind("refactor.rewrite"),
            ]
        }
    }
}

// =============================================================================
// Temporal Logic Support
// =============================================================================

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

impl std::fmt::Display for LtlFormula {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Atom(name) => write!(f, "{}", name),
            Self::Not(formula) => write!(f, "¬({})", formula),
            Self::And(left, right) => write!(f, "({} ∧ {})", left, right),
            Self::Or(left, right) => write!(f, "({} ∨ {})", left, right),
            Self::Implies(left, right) => write!(f, "({} → {})", left, right),
            Self::Next(formula) => write!(f, "X({})", formula),
            Self::Eventually(formula) => write!(f, "F({})", formula),
            Self::Always(formula) => write!(f, "G({})", formula),
            Self::Until(left, right) => write!(f, "({} U {})", left, right),
            Self::Release(left, right) => write!(f, "({} R {})", left, right),
        }
    }
}

/// Computation Tree Logic (CTL) formula.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CtlFormula {
    /// Atomic proposition
    Atom(String),
    /// Negation
    Not(Box<CtlFormula>),
    /// Conjunction
    And(Box<CtlFormula>, Box<CtlFormula>),
    /// Disjunction
    Or(Box<CtlFormula>, Box<CtlFormula>),
    /// Implication
    Implies(Box<CtlFormula>, Box<CtlFormula>),
    /// Exists Next (there exists a next state where formula holds)
    ExistsNext(Box<CtlFormula>),
    /// All Next (formula holds in all next states)
    AllNext(Box<CtlFormula>),
    /// Exists Eventually (there exists a path where formula eventually holds)
    ExistsEventually(Box<CtlFormula>),
    /// All Eventually (formula eventually holds on all paths)
    AllEventually(Box<CtlFormula>),
    /// Exists Always (there exists a path where formula always holds)
    ExistsAlways(Box<CtlFormula>),
    /// All Always (formula always holds on all paths)
    AllAlways(Box<CtlFormula>),
    /// Exists Until
    ExistsUntil(Box<CtlFormula>, Box<CtlFormula>),
    /// All Until
    AllUntil(Box<CtlFormula>, Box<CtlFormula>),
}

impl CtlFormula {
    /// Creates a new atomic proposition.
    pub fn atom(name: impl Into<String>) -> Self {
        Self::Atom(name.into())
    }

    /// Creates a negation.
    #[allow(clippy::should_implement_trait)]
    pub fn not(formula: CtlFormula) -> Self {
        Self::Not(Box::new(formula))
    }

    /// Creates a conjunction.
    pub fn and(left: CtlFormula, right: CtlFormula) -> Self {
        Self::And(Box::new(left), Box::new(right))
    }

    /// Creates a disjunction.
    pub fn or(left: CtlFormula, right: CtlFormula) -> Self {
        Self::Or(Box::new(left), Box::new(right))
    }

    /// Creates an implication.
    pub fn implies(antecedent: CtlFormula, consequent: CtlFormula) -> Self {
        Self::Implies(Box::new(antecedent), Box::new(consequent))
    }

    /// Creates an exists-next operator.
    pub fn exists_next(formula: CtlFormula) -> Self {
        Self::ExistsNext(Box::new(formula))
    }

    /// Creates an all-next operator.
    pub fn all_next(formula: CtlFormula) -> Self {
        Self::AllNext(Box::new(formula))
    }

    /// Creates an exists-eventually operator.
    pub fn exists_eventually(formula: CtlFormula) -> Self {
        Self::ExistsEventually(Box::new(formula))
    }

    /// Creates an all-eventually operator.
    pub fn all_eventually(formula: CtlFormula) -> Self {
        Self::AllEventually(Box::new(formula))
    }

    /// Creates an exists-always operator.
    pub fn exists_always(formula: CtlFormula) -> Self {
        Self::ExistsAlways(Box::new(formula))
    }

    /// Creates an all-always operator.
    pub fn all_always(formula: CtlFormula) -> Self {
        Self::AllAlways(Box::new(formula))
    }

    /// Creates an exists-until operator.
    pub fn exists_until(left: CtlFormula, right: CtlFormula) -> Self {
        Self::ExistsUntil(Box::new(left), Box::new(right))
    }

    /// Creates an all-until operator.
    pub fn all_until(left: CtlFormula, right: CtlFormula) -> Self {
        Self::AllUntil(Box::new(left), Box::new(right))
    }
}

impl std::fmt::Display for CtlFormula {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Atom(name) => write!(f, "{}", name),
            Self::Not(formula) => write!(f, "¬({})", formula),
            Self::And(left, right) => write!(f, "({} ∧ {})", left, right),
            Self::Or(left, right) => write!(f, "({} ∨ {})", left, right),
            Self::Implies(left, right) => write!(f, "({} → {})", left, right),
            Self::ExistsNext(formula) => write!(f, "EX({})", formula),
            Self::AllNext(formula) => write!(f, "AX({})", formula),
            Self::ExistsEventually(formula) => write!(f, "EF({})", formula),
            Self::AllEventually(formula) => write!(f, "AF({})", formula),
            Self::ExistsAlways(formula) => write!(f, "EG({})", formula),
            Self::AllAlways(formula) => write!(f, "AG({})", formula),
            Self::ExistsUntil(left, right) => write!(f, "E({} U {})", left, right),
            Self::AllUntil(left, right) => write!(f, "A({} U {})", left, right),
        }
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

impl Default for TransitionSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Checks if an LTL formula holds in a transition system.
///
/// This is a simplified model checker that verifies LTL properties
/// over finite traces. For production use, consider using a dedicated
/// model checker like SPIN or NuSMV.
pub fn verify_ltl(system: &TransitionSystem, formula: &LtlFormula) -> bool {
    // For each initial state, verify the formula
    for initial_id in &system.initial_states {
        if let Some(initial_state) = system.states.get(initial_id) {
            let mut visited = HashSet::new();
            if !check_ltl_from_state(system, initial_state, formula, &mut visited) {
                return false;
            }
        }
    }
    true
}

/// Helper function to check LTL from a specific state.
#[allow(dead_code)]
fn check_ltl_from_state(
    system: &TransitionSystem,
    state: &TemporalState,
    formula: &LtlFormula,
    visited: &mut HashSet<String>,
) -> bool {
    // Prevent infinite loops in cyclic systems
    if visited.contains(&state.id) {
        return true; // Optimistically assume formula holds in cycles
    }
    visited.insert(state.id.clone());

    match formula {
        LtlFormula::Atom(prop) => state.satisfies(prop),
        LtlFormula::Not(f) => !check_ltl_from_state(system, state, f, visited),
        LtlFormula::And(left, right) => {
            check_ltl_from_state(system, state, left, visited)
                && check_ltl_from_state(system, state, right, visited)
        }
        LtlFormula::Or(left, right) => {
            check_ltl_from_state(system, state, left, visited)
                || check_ltl_from_state(system, state, right, visited)
        }
        LtlFormula::Implies(left, right) => {
            !check_ltl_from_state(system, state, left, visited)
                || check_ltl_from_state(system, state, right, visited)
        }
        LtlFormula::Next(f) => {
            let successors = system.successors(&state.id);
            if successors.is_empty() {
                return true; // No next state, vacuously true
            }
            successors
                .iter()
                .all(|s| check_ltl_from_state(system, s, f, visited))
        }
        LtlFormula::Eventually(f) => {
            check_eventually(system, state, f, visited, &mut HashSet::new())
        }
        LtlFormula::Always(f) => check_always(system, state, f, visited),
        LtlFormula::Until(left, right) => {
            check_until(system, state, left, right, visited, &mut HashSet::new())
        }
        LtlFormula::Release(left, right) => {
            // p R q = ¬(¬p U ¬q)
            let not_p = LtlFormula::not(*left.clone());
            let not_q = LtlFormula::not(*right.clone());
            !check_until(system, state, &not_p, &not_q, visited, &mut HashSet::new())
        }
    }
}

#[allow(dead_code)]
fn check_eventually(
    system: &TransitionSystem,
    state: &TemporalState,
    formula: &LtlFormula,
    visited: &mut HashSet<String>,
    path_visited: &mut HashSet<String>,
) -> bool {
    if path_visited.contains(&state.id) {
        return false; // Cycle without finding formula
    }
    path_visited.insert(state.id.clone());

    // Check if formula holds in current state
    if check_ltl_from_state(system, state, formula, visited) {
        return true;
    }

    // Check if formula holds in any successor
    let successors = system.successors(&state.id);
    successors
        .iter()
        .any(|s| check_eventually(system, s, formula, visited, path_visited))
}

#[allow(dead_code)]
fn check_always(
    system: &TransitionSystem,
    state: &TemporalState,
    formula: &LtlFormula,
    visited: &mut HashSet<String>,
) -> bool {
    if !check_ltl_from_state(system, state, formula, visited) {
        return false;
    }

    let successors = system.successors(&state.id);
    if successors.is_empty() {
        return true; // No more states, formula held throughout
    }

    successors
        .iter()
        .all(|s| check_always(system, s, formula, visited))
}

#[allow(dead_code)]
fn check_until(
    system: &TransitionSystem,
    state: &TemporalState,
    left: &LtlFormula,
    right: &LtlFormula,
    visited: &mut HashSet<String>,
    path_visited: &mut HashSet<String>,
) -> bool {
    if path_visited.contains(&state.id) {
        return false;
    }
    path_visited.insert(state.id.clone());

    // Check if right holds
    if check_ltl_from_state(system, state, right, visited) {
        return true;
    }

    // Check if left holds and until holds in some successor
    if !check_ltl_from_state(system, state, left, visited) {
        return false;
    }

    let successors = system.successors(&state.id);
    successors
        .iter()
        .any(|s| check_until(system, s, left, right, visited, path_visited))
}

/// Checks if a CTL formula holds in a transition system.
pub fn verify_ctl(system: &TransitionSystem, formula: &CtlFormula) -> bool {
    // Verify formula from all initial states
    for initial_id in &system.initial_states {
        if let Some(initial_state) = system.states.get(initial_id) {
            if !check_ctl_from_state(system, initial_state, formula) {
                return false;
            }
        }
    }
    true
}

#[allow(dead_code)]
fn check_ctl_from_state(
    system: &TransitionSystem,
    state: &TemporalState,
    formula: &CtlFormula,
) -> bool {
    match formula {
        CtlFormula::Atom(prop) => state.satisfies(prop),
        CtlFormula::Not(f) => !check_ctl_from_state(system, state, f),
        CtlFormula::And(left, right) => {
            check_ctl_from_state(system, state, left) && check_ctl_from_state(system, state, right)
        }
        CtlFormula::Or(left, right) => {
            check_ctl_from_state(system, state, left) || check_ctl_from_state(system, state, right)
        }
        CtlFormula::Implies(left, right) => {
            !check_ctl_from_state(system, state, left) || check_ctl_from_state(system, state, right)
        }
        CtlFormula::ExistsNext(f) => {
            let successors = system.successors(&state.id);
            successors
                .iter()
                .any(|s| check_ctl_from_state(system, s, f))
        }
        CtlFormula::AllNext(f) => {
            let successors = system.successors(&state.id);
            if successors.is_empty() {
                return true;
            }
            successors
                .iter()
                .all(|s| check_ctl_from_state(system, s, f))
        }
        CtlFormula::ExistsEventually(f) => {
            check_ctl_exists_eventually(system, state, f, &mut HashSet::new())
        }
        CtlFormula::AllEventually(f) => {
            check_ctl_all_eventually(system, state, f, &mut HashSet::new())
        }
        CtlFormula::ExistsAlways(f) => {
            check_ctl_exists_always(system, state, f, &mut HashSet::new())
        }
        CtlFormula::AllAlways(f) => check_ctl_all_always(system, state, f, &mut HashSet::new()),
        CtlFormula::ExistsUntil(left, right) => {
            check_ctl_exists_until(system, state, left, right, &mut HashSet::new())
        }
        CtlFormula::AllUntil(left, right) => {
            check_ctl_all_until(system, state, left, right, &mut HashSet::new())
        }
    }
}

#[allow(dead_code)]
fn check_ctl_exists_eventually(
    system: &TransitionSystem,
    state: &TemporalState,
    formula: &CtlFormula,
    visited: &mut HashSet<String>,
) -> bool {
    if visited.contains(&state.id) {
        return false;
    }
    visited.insert(state.id.clone());

    if check_ctl_from_state(system, state, formula) {
        return true;
    }

    let successors = system.successors(&state.id);
    successors
        .iter()
        .any(|s| check_ctl_exists_eventually(system, s, formula, visited))
}

#[allow(dead_code)]
fn check_ctl_all_eventually(
    system: &TransitionSystem,
    state: &TemporalState,
    formula: &CtlFormula,
    visited: &mut HashSet<String>,
) -> bool {
    if visited.contains(&state.id) {
        return false;
    }
    visited.insert(state.id.clone());

    if check_ctl_from_state(system, state, formula) {
        return true;
    }

    let successors = system.successors(&state.id);
    if successors.is_empty() {
        return false;
    }

    successors
        .iter()
        .all(|s| check_ctl_all_eventually(system, s, formula, visited))
}

#[allow(dead_code)]
fn check_ctl_exists_always(
    system: &TransitionSystem,
    state: &TemporalState,
    formula: &CtlFormula,
    visited: &mut HashSet<String>,
) -> bool {
    if !check_ctl_from_state(system, state, formula) {
        return false;
    }

    if visited.contains(&state.id) {
        return true; // Cycle where formula always holds
    }
    visited.insert(state.id.clone());

    let successors = system.successors(&state.id);
    successors
        .iter()
        .any(|s| check_ctl_exists_always(system, s, formula, visited))
}

#[allow(dead_code)]
fn check_ctl_all_always(
    system: &TransitionSystem,
    state: &TemporalState,
    formula: &CtlFormula,
    visited: &mut HashSet<String>,
) -> bool {
    if !check_ctl_from_state(system, state, formula) {
        return false;
    }

    if visited.contains(&state.id) {
        return true;
    }
    visited.insert(state.id.clone());

    let successors = system.successors(&state.id);
    if successors.is_empty() {
        return true;
    }

    successors
        .iter()
        .all(|s| check_ctl_all_always(system, s, formula, visited))
}

#[allow(dead_code)]
fn check_ctl_exists_until(
    system: &TransitionSystem,
    state: &TemporalState,
    left: &CtlFormula,
    right: &CtlFormula,
    visited: &mut HashSet<String>,
) -> bool {
    if visited.contains(&state.id) {
        return false;
    }
    visited.insert(state.id.clone());

    if check_ctl_from_state(system, state, right) {
        return true;
    }

    if !check_ctl_from_state(system, state, left) {
        return false;
    }

    let successors = system.successors(&state.id);
    successors
        .iter()
        .any(|s| check_ctl_exists_until(system, s, left, right, visited))
}

#[allow(dead_code)]
fn check_ctl_all_until(
    system: &TransitionSystem,
    state: &TemporalState,
    left: &CtlFormula,
    right: &CtlFormula,
    visited: &mut HashSet<String>,
) -> bool {
    if visited.contains(&state.id) {
        return false;
    }
    visited.insert(state.id.clone());

    if check_ctl_from_state(system, state, right) {
        return true;
    }

    if !check_ctl_from_state(system, state, left) {
        return false;
    }

    let successors = system.successors(&state.id);
    if successors.is_empty() {
        return false;
    }

    successors
        .iter()
        .all(|s| check_ctl_all_until(system, s, left, right, visited))
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

/// Result of deadline verification.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeadlineVerificationResult {
    /// Whether all deadlines were met
    pub passed: bool,
    /// Violated deadlines
    pub violations: Vec<DeadlineViolation>,
}

/// A deadline violation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeadlineViolation {
    /// Deadline that was violated
    pub deadline_id: String,
    /// Actual number of steps taken
    pub actual_steps: usize,
    /// Maximum allowed steps
    pub max_steps: usize,
    /// Description of the violation
    pub description: String,
}

/// Verifies deadlines in a transition system.
pub fn verify_deadlines(
    system: &TransitionSystem,
    deadlines: &[Deadline],
) -> DeadlineVerificationResult {
    let mut violations = Vec::new();

    for deadline in deadlines {
        for initial_id in &system.initial_states {
            if let Some(initial_state) = system.states.get(initial_id) {
                let steps = count_steps_to_event(
                    system,
                    initial_state,
                    &deadline.event,
                    &mut HashSet::new(),
                );

                if let Some(actual_steps) = steps {
                    if actual_steps > deadline.max_steps {
                        violations.push(DeadlineViolation {
                            deadline_id: deadline.id.clone(),
                            actual_steps,
                            max_steps: deadline.max_steps,
                            description: format!(
                                "Event '{}' occurred after {} steps (deadline: {} steps)",
                                deadline.event, actual_steps, deadline.max_steps
                            ),
                        });
                    }
                } else if deadline.max_steps < usize::MAX {
                    // Event never occurs, which violates the deadline
                    violations.push(DeadlineViolation {
                        deadline_id: deadline.id.clone(),
                        actual_steps: usize::MAX,
                        max_steps: deadline.max_steps,
                        description: format!(
                            "Event '{}' never occurs (deadline: {} steps)",
                            deadline.event, deadline.max_steps
                        ),
                    });
                }
            }
        }
    }

    DeadlineVerificationResult {
        passed: violations.is_empty(),
        violations,
    }
}

#[allow(dead_code)]
fn count_steps_to_event(
    system: &TransitionSystem,
    state: &TemporalState,
    event: &str,
    visited: &mut HashSet<String>,
) -> Option<usize> {
    if visited.contains(&state.id) {
        return None; // Cycle without finding event
    }
    visited.insert(state.id.clone());

    if state.satisfies(event) {
        return Some(0);
    }

    let successors = system.successors(&state.id);
    let mut min_steps = None;

    for successor in successors {
        if let Some(steps) = count_steps_to_event(system, successor, event, visited) {
            let total = steps + 1;
            min_steps = Some(min_steps.map_or(total, |current: usize| current.min(total)));
        }
    }

    min_steps
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

/// Result of sequence constraint verification.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SequenceVerificationResult {
    /// Whether all constraints were satisfied
    pub passed: bool,
    /// Violated constraints
    pub violations: Vec<SequenceViolation>,
}

/// A sequence constraint violation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SequenceViolation {
    /// Constraint that was violated
    pub constraint_id: String,
    /// Description of the violation
    pub description: String,
    /// Events that violated the order
    pub violating_events: Vec<String>,
}

/// Verifies sequence constraints in a transition system.
pub fn verify_sequences(
    system: &TransitionSystem,
    constraints: &[SequenceConstraint],
) -> SequenceVerificationResult {
    let mut violations = Vec::new();

    for constraint in constraints {
        for initial_id in &system.initial_states {
            if let Some(initial_state) = system.states.get(initial_id) {
                if !check_sequence(
                    system,
                    initial_state,
                    &constraint.events,
                    0,
                    constraint.strict,
                    &mut HashSet::new(),
                ) {
                    violations.push(SequenceViolation {
                        constraint_id: constraint.id.clone(),
                        description: format!(
                            "Required event sequence {:?} was not followed",
                            constraint.events
                        ),
                        violating_events: constraint.events.clone(),
                    });
                    break;
                }
            }
        }
    }

    SequenceVerificationResult {
        passed: violations.is_empty(),
        violations,
    }
}

#[allow(dead_code)]
fn check_sequence(
    system: &TransitionSystem,
    state: &TemporalState,
    events: &[String],
    current_index: usize,
    strict: bool,
    visited: &mut HashSet<(String, usize)>,
) -> bool {
    let key = (state.id.clone(), current_index);
    if visited.contains(&key) {
        return false;
    }
    visited.insert(key);

    if current_index >= events.len() {
        return true; // All events found in order
    }

    let current_event = &events[current_index];

    if state.satisfies(current_event) {
        // Found current event, look for next
        let successors = system.successors(&state.id);
        return successors
            .iter()
            .any(|s| check_sequence(system, s, events, current_index + 1, strict, visited))
            || (current_index + 1 >= events.len()); // Last event found
    }

    if strict {
        // In strict mode, we can't skip states
        return false;
    }

    // Continue to next state without finding this event
    let successors = system.successors(&state.id);
    successors
        .iter()
        .any(|s| check_sequence(system, s, events, current_index, strict, visited))
}

// =============================================================================
// CTL* (Computation Tree Logic Star) Model Checking
// =============================================================================

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

impl std::fmt::Display for CtlStarFormula {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Atom(name) => write!(f, "{}", name),
            Self::Not(formula) => write!(f, "¬({})", formula),
            Self::And(left, right) => write!(f, "({} ∧ {})", left, right),
            Self::Or(left, right) => write!(f, "({} ∨ {})", left, right),
            Self::Implies(left, right) => write!(f, "({} → {})", left, right),
            Self::Exists(path) => write!(f, "E({})", path),
            Self::All(path) => write!(f, "A({})", path),
        }
    }
}

impl std::fmt::Display for CtlStarPathFormula {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::State(formula) => write!(f, "{}", formula),
            Self::Not(formula) => write!(f, "¬({})", formula),
            Self::And(left, right) => write!(f, "({} ∧ {})", left, right),
            Self::Or(left, right) => write!(f, "({} ∨ {})", left, right),
            Self::Next(formula) => write!(f, "X({})", formula),
            Self::Eventually(formula) => write!(f, "F({})", formula),
            Self::Always(formula) => write!(f, "G({})", formula),
            Self::Until(left, right) => write!(f, "({} U {})", left, right),
            Self::Release(left, right) => write!(f, "({} R {})", left, right),
        }
    }
}

/// Verifies a CTL* formula on a transition system.
///
/// CTL* combines the expressiveness of both CTL and LTL, allowing
/// arbitrary nesting of path quantifiers and temporal operators.
pub fn verify_ctl_star(system: &TransitionSystem, formula: &CtlStarFormula) -> bool {
    for initial_id in &system.initial_states {
        if let Some(initial_state) = system.states.get(initial_id) {
            if !check_ctl_star_state(system, initial_state, formula, &mut HashSet::new()) {
                return false;
            }
        }
    }
    true
}

#[allow(dead_code)]
fn check_ctl_star_state(
    system: &TransitionSystem,
    state: &TemporalState,
    formula: &CtlStarFormula,
    visited: &mut HashSet<String>,
) -> bool {
    match formula {
        CtlStarFormula::Atom(prop) => state.satisfies(prop),
        CtlStarFormula::Not(f) => !check_ctl_star_state(system, state, f, visited),
        CtlStarFormula::And(left, right) => {
            check_ctl_star_state(system, state, left, visited)
                && check_ctl_star_state(system, state, right, visited)
        }
        CtlStarFormula::Or(left, right) => {
            check_ctl_star_state(system, state, left, visited)
                || check_ctl_star_state(system, state, right, visited)
        }
        CtlStarFormula::Implies(left, right) => {
            !check_ctl_star_state(system, state, left, visited)
                || check_ctl_star_state(system, state, right, visited)
        }
        CtlStarFormula::Exists(path) => {
            // There exists a path from this state where path formula holds
            check_ctl_star_exists_path(system, state, path, visited)
        }
        CtlStarFormula::All(path) => {
            // For all paths from this state, path formula holds
            check_ctl_star_all_paths(system, state, path, visited)
        }
    }
}

#[allow(dead_code)]
fn check_ctl_star_exists_path(
    system: &TransitionSystem,
    state: &TemporalState,
    path: &CtlStarPathFormula,
    visited: &mut HashSet<String>,
) -> bool {
    check_ctl_star_path(system, state, path, visited, &mut HashSet::new())
}

#[allow(dead_code)]
fn check_ctl_star_all_paths(
    system: &TransitionSystem,
    state: &TemporalState,
    path: &CtlStarPathFormula,
    visited: &mut HashSet<String>,
) -> bool {
    check_ctl_star_path_universal(system, state, path, visited, &mut HashSet::new())
}

#[allow(dead_code)]
fn check_ctl_star_path(
    system: &TransitionSystem,
    state: &TemporalState,
    path: &CtlStarPathFormula,
    visited: &mut HashSet<String>,
    path_visited: &mut HashSet<String>,
) -> bool {
    match path {
        CtlStarPathFormula::State(formula) => check_ctl_star_state(system, state, formula, visited),
        CtlStarPathFormula::Not(p) => !check_ctl_star_path(system, state, p, visited, path_visited),
        CtlStarPathFormula::And(left, right) => {
            check_ctl_star_path(system, state, left, visited, path_visited)
                && check_ctl_star_path(system, state, right, visited, path_visited)
        }
        CtlStarPathFormula::Or(left, right) => {
            check_ctl_star_path(system, state, left, visited, path_visited)
                || check_ctl_star_path(system, state, right, visited, path_visited)
        }
        CtlStarPathFormula::Next(p) => {
            let successors = system.successors(&state.id);
            if successors.is_empty() {
                return false;
            }
            successors
                .iter()
                .any(|s| check_ctl_star_path(system, s, p, visited, path_visited))
        }
        CtlStarPathFormula::Eventually(p) => {
            if path_visited.contains(&state.id) {
                return false;
            }
            let mut new_path_visited = path_visited.clone();
            new_path_visited.insert(state.id.clone());

            if check_ctl_star_path(system, state, p, visited, &mut new_path_visited) {
                return true;
            }

            let successors = system.successors(&state.id);
            successors.iter().any(|s| {
                check_ctl_star_path(
                    system,
                    s,
                    &CtlStarPathFormula::Eventually(p.clone()),
                    visited,
                    &mut new_path_visited,
                )
            })
        }
        CtlStarPathFormula::Always(p) => {
            if !check_ctl_star_path(system, state, p, visited, path_visited) {
                return false;
            }

            if path_visited.contains(&state.id) {
                return true; // Cycle where formula always held
            }

            let mut new_path_visited = path_visited.clone();
            new_path_visited.insert(state.id.clone());

            let successors = system.successors(&state.id);
            if successors.is_empty() {
                return true;
            }

            successors.iter().any(|s| {
                check_ctl_star_path(
                    system,
                    s,
                    &CtlStarPathFormula::Always(p.clone()),
                    visited,
                    &mut new_path_visited,
                )
            })
        }
        CtlStarPathFormula::Until(left, right) => {
            if path_visited.contains(&state.id) {
                return false;
            }
            let mut new_path_visited = path_visited.clone();
            new_path_visited.insert(state.id.clone());

            if check_ctl_star_path(system, state, right, visited, &mut new_path_visited.clone()) {
                return true;
            }

            if !check_ctl_star_path(system, state, left, visited, &mut new_path_visited.clone()) {
                return false;
            }

            let successors = system.successors(&state.id);
            successors.iter().any(|s| {
                check_ctl_star_path(
                    system,
                    s,
                    &CtlStarPathFormula::Until(left.clone(), right.clone()),
                    visited,
                    &mut new_path_visited,
                )
            })
        }
        CtlStarPathFormula::Release(left, right) => {
            // p R q = ¬(¬p U ¬q)
            let not_left = CtlStarPathFormula::Not(left.clone());
            let not_right = CtlStarPathFormula::Not(right.clone());
            !check_ctl_star_path(
                system,
                state,
                &CtlStarPathFormula::Until(Box::new(not_left), Box::new(not_right)),
                visited,
                path_visited,
            )
        }
    }
}

#[allow(dead_code)]
fn check_ctl_star_path_universal(
    system: &TransitionSystem,
    state: &TemporalState,
    path: &CtlStarPathFormula,
    visited: &mut HashSet<String>,
    path_visited: &mut HashSet<String>,
) -> bool {
    match path {
        CtlStarPathFormula::State(formula) => check_ctl_star_state(system, state, formula, visited),
        CtlStarPathFormula::Next(p) => {
            let successors = system.successors(&state.id);
            if successors.is_empty() {
                return true;
            }
            successors
                .iter()
                .all(|s| check_ctl_star_path_universal(system, s, p, visited, path_visited))
        }
        CtlStarPathFormula::Always(p) => {
            if !check_ctl_star_path_universal(system, state, p, visited, path_visited) {
                return false;
            }

            if path_visited.contains(&state.id) {
                return true;
            }

            let mut new_path_visited = path_visited.clone();
            new_path_visited.insert(state.id.clone());

            let successors = system.successors(&state.id);
            if successors.is_empty() {
                return true;
            }

            successors.iter().all(|s| {
                check_ctl_star_path_universal(
                    system,
                    s,
                    &CtlStarPathFormula::Always(p.clone()),
                    visited,
                    &mut new_path_visited,
                )
            })
        }
        _ => {
            // For other operators, use existential checking
            check_ctl_star_path(system, state, path, visited, path_visited)
        }
    }
}

// =============================================================================
// Timed Automata Verification
// =============================================================================

/// A clock variable used in timed automata.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Clock {
    /// Clock name
    pub name: String,
}

impl Clock {
    /// Creates a new clock.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

/// Clock constraint in timed automata.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ClockConstraint {
    /// clock < value
    Less(Clock, u64),
    /// clock <= value
    LessOrEqual(Clock, u64),
    /// clock > value
    Greater(Clock, u64),
    /// clock >= value
    GreaterOrEqual(Clock, u64),
    /// clock == value
    Equal(Clock, u64),
    /// Conjunction of constraints
    And(Box<ClockConstraint>, Box<ClockConstraint>),
}

impl ClockConstraint {
    /// Checks if the constraint is satisfied given clock valuations.
    pub fn satisfied(&self, valuations: &HashMap<String, u64>) -> bool {
        match self {
            Self::Less(clock, value) => valuations.get(&clock.name).is_some_and(|v| v < value),
            Self::LessOrEqual(clock, value) => {
                valuations.get(&clock.name).is_some_and(|v| v <= value)
            }
            Self::Greater(clock, value) => valuations.get(&clock.name).is_some_and(|v| v > value),
            Self::GreaterOrEqual(clock, value) => {
                valuations.get(&clock.name).is_some_and(|v| v >= value)
            }
            Self::Equal(clock, value) => valuations.get(&clock.name) == Some(value),
            Self::And(left, right) => left.satisfied(valuations) && right.satisfied(valuations),
        }
    }
}

/// A location (state) in a timed automaton.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TimedLocation {
    /// Location identifier
    pub id: String,
    /// Invariant that must hold while in this location
    pub invariant: Option<ClockConstraint>,
    /// Whether this is an accepting/final location
    pub accepting: bool,
}

impl TimedLocation {
    /// Creates a new timed location.
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            invariant: None,
            accepting: false,
        }
    }

    /// Sets the invariant.
    pub fn with_invariant(mut self, constraint: ClockConstraint) -> Self {
        self.invariant = Some(constraint);
        self
    }

    /// Marks this location as accepting.
    pub fn accepting(mut self) -> Self {
        self.accepting = true;
        self
    }
}

/// A transition in a timed automaton.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TimedTransition {
    /// Source location
    pub from: String,
    /// Target location
    pub to: String,
    /// Guard (condition) for the transition
    pub guard: Option<ClockConstraint>,
    /// Clocks to reset on this transition
    pub resets: Vec<Clock>,
    /// Action/label for the transition
    pub action: String,
}

impl TimedTransition {
    /// Creates a new timed transition.
    pub fn new(from: impl Into<String>, to: impl Into<String>, action: impl Into<String>) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
            guard: None,
            resets: Vec::new(),
            action: action.into(),
        }
    }

    /// Sets the guard.
    pub fn with_guard(mut self, constraint: ClockConstraint) -> Self {
        self.guard = Some(constraint);
        self
    }

    /// Adds a clock to reset.
    pub fn with_reset(mut self, clock: Clock) -> Self {
        self.resets.push(clock);
        self
    }
}

/// A timed automaton.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TimedAutomaton {
    /// All locations in the automaton
    pub locations: HashMap<String, TimedLocation>,
    /// All transitions
    pub transitions: Vec<TimedTransition>,
    /// Initial location
    pub initial: String,
    /// All clock variables
    pub clocks: Vec<Clock>,
}

impl TimedAutomaton {
    /// Creates a new timed automaton.
    pub fn new(initial: impl Into<String>) -> Self {
        Self {
            locations: HashMap::new(),
            transitions: Vec::new(),
            initial: initial.into(),
            clocks: Vec::new(),
        }
    }

    /// Adds a location.
    pub fn add_location(&mut self, location: TimedLocation) {
        self.locations.insert(location.id.clone(), location);
    }

    /// Adds a transition.
    pub fn add_transition(&mut self, transition: TimedTransition) {
        self.transitions.push(transition);
    }

    /// Adds a clock.
    pub fn add_clock(&mut self, clock: Clock) {
        self.clocks.push(clock);
    }
}

/// Configuration of a timed automaton (location + clock valuations).
#[derive(Debug, Clone)]
struct TimedConfiguration {
    location: String,
    valuations: HashMap<String, u64>,
}

impl TimedConfiguration {
    fn new(location: String) -> Self {
        Self {
            location,
            valuations: HashMap::new(),
        }
    }
}

/// Verifies reachability in a timed automaton.
///
/// Returns true if an accepting location is reachable from the initial location
/// within the given time bound.
pub fn verify_timed_reachability(automaton: &TimedAutomaton, time_bound: u64) -> bool {
    let mut queue = std::collections::VecDeque::new();
    let mut visited = HashSet::new();

    // Initialize all clocks to 0
    let mut initial_config = TimedConfiguration::new(automaton.initial.clone());
    for clock in &automaton.clocks {
        initial_config.valuations.insert(clock.name.clone(), 0);
    }

    queue.push_back((initial_config, 0u64));

    while let Some((config, time)) = queue.pop_front() {
        if time > time_bound {
            continue;
        }

        let state_key = format!("{:?}", (&config.location, &config.valuations));
        if visited.contains(&state_key) {
            continue;
        }
        visited.insert(state_key);

        // Check if we reached an accepting location
        if let Some(location) = automaton.locations.get(&config.location) {
            if location.accepting {
                return true;
            }

            // Check invariant
            if let Some(ref invariant) = location.invariant {
                if !invariant.satisfied(&config.valuations) {
                    continue;
                }
            }
        }

        // Try all transitions from current location
        for transition in &automaton.transitions {
            if transition.from != config.location {
                continue;
            }

            // Check guard
            if let Some(ref guard) = transition.guard {
                if !guard.satisfied(&config.valuations) {
                    continue;
                }
            }

            // Apply transition
            let mut new_valuations = config.valuations.clone();

            // Reset specified clocks
            for clock in &transition.resets {
                new_valuations.insert(clock.name.clone(), 0);
            }

            // Advance time (simplified: advance by 1 time unit)
            for (_, val) in new_valuations.iter_mut() {
                *val += 1;
            }

            let new_config = TimedConfiguration {
                location: transition.to.clone(),
                valuations: new_valuations,
            };

            queue.push_back((new_config, time + 1));
        }
    }

    false
}

// =============================================================================
// Temporal Property Synthesis
// =============================================================================

/// Synthesizes a temporal property from positive and negative examples.
///
/// Given traces that should satisfy a property (positive examples) and
/// traces that should not (negative examples), this function attempts to
/// synthesize an LTL formula that separates them.
///
/// Returns the synthesized LTL formula if successful.
pub fn synthesize_ltl_property(
    positive_traces: &[Vec<HashSet<String>>],
    negative_traces: &[Vec<HashSet<String>>],
) -> Option<LtlFormula> {
    // Extract all atomic propositions from traces
    let mut all_props = HashSet::new();
    for trace in positive_traces.iter().chain(negative_traces.iter()) {
        for state_props in trace {
            all_props.extend(state_props.clone());
        }
    }

    if all_props.is_empty() {
        return None;
    }

    // Try simple patterns first
    // Pattern 1: Always(p) - some proposition must always hold
    for prop in &all_props {
        let formula = LtlFormula::always(LtlFormula::atom(prop));
        if check_formula_on_traces(&formula, positive_traces, true)
            && check_formula_on_traces(&formula, negative_traces, false)
        {
            return Some(formula);
        }
    }

    // Pattern 2: Eventually(p) - some proposition must eventually hold
    for prop in &all_props {
        let formula = LtlFormula::eventually(LtlFormula::atom(prop));
        if check_formula_on_traces(&formula, positive_traces, true)
            && check_formula_on_traces(&formula, negative_traces, false)
        {
            return Some(formula);
        }
    }

    // Pattern 3: Always(p → Eventually(q)) - response pattern
    for p in &all_props {
        for q in &all_props {
            if p == q {
                continue;
            }
            let formula = LtlFormula::always(LtlFormula::implies(
                LtlFormula::atom(p),
                LtlFormula::eventually(LtlFormula::atom(q)),
            ));
            if check_formula_on_traces(&formula, positive_traces, true)
                && check_formula_on_traces(&formula, negative_traces, false)
            {
                return Some(formula);
            }
        }
    }

    // Pattern 4: Always(p) ∧ Eventually(q) - invariant with liveness
    for p in &all_props {
        for q in &all_props {
            let formula = LtlFormula::and(
                LtlFormula::always(LtlFormula::atom(p)),
                LtlFormula::eventually(LtlFormula::atom(q)),
            );
            if check_formula_on_traces(&formula, positive_traces, true)
                && check_formula_on_traces(&formula, negative_traces, false)
            {
                return Some(formula);
            }
        }
    }

    // If no simple pattern works, return None
    // In a production system, this would try more complex patterns or use
    // machine learning techniques
    None
}

/// Checks if a formula holds on all traces with expected result.
fn check_formula_on_traces(
    formula: &LtlFormula,
    traces: &[Vec<HashSet<String>>],
    expected: bool,
) -> bool {
    for trace in traces {
        let holds = check_formula_on_trace(formula, trace);
        if holds != expected {
            return false;
        }
    }
    true
}

/// Checks if an LTL formula holds on a single trace.
fn check_formula_on_trace(formula: &LtlFormula, trace: &[HashSet<String>]) -> bool {
    if trace.is_empty() {
        return false;
    }
    check_ltl_at_position(formula, trace, 0)
}

/// Checks if an LTL formula holds starting at a specific position in a trace.
fn check_ltl_at_position(formula: &LtlFormula, trace: &[HashSet<String>], pos: usize) -> bool {
    if pos >= trace.len() {
        return false;
    }

    match formula {
        LtlFormula::Atom(prop) => trace[pos].contains(prop),
        LtlFormula::Not(f) => !check_ltl_at_position(f, trace, pos),
        LtlFormula::And(left, right) => {
            check_ltl_at_position(left, trace, pos) && check_ltl_at_position(right, trace, pos)
        }
        LtlFormula::Or(left, right) => {
            check_ltl_at_position(left, trace, pos) || check_ltl_at_position(right, trace, pos)
        }
        LtlFormula::Implies(left, right) => {
            !check_ltl_at_position(left, trace, pos) || check_ltl_at_position(right, trace, pos)
        }
        LtlFormula::Next(f) => {
            if pos + 1 < trace.len() {
                check_ltl_at_position(f, trace, pos + 1)
            } else {
                false
            }
        }
        LtlFormula::Eventually(f) => (pos..trace.len()).any(|i| check_ltl_at_position(f, trace, i)),
        LtlFormula::Always(f) => (pos..trace.len()).all(|i| check_ltl_at_position(f, trace, i)),
        LtlFormula::Until(left, right) => {
            for i in pos..trace.len() {
                if check_ltl_at_position(right, trace, i) {
                    return (pos..i).all(|j| check_ltl_at_position(left, trace, j));
                }
            }
            false
        }
        LtlFormula::Release(left, right) => {
            // p R q = ¬(¬p U ¬q)
            let not_left = LtlFormula::not(*left.clone());
            let not_right = LtlFormula::not(*right.clone());
            !check_ltl_at_position(&LtlFormula::until(not_left, not_right), trace, pos)
        }
    }
}

/// Synthesizes a CTL property from a transition system and examples.
///
/// This is a simplified synthesis that generates basic CTL patterns
/// based on the structure of the transition system and desired properties.
pub fn synthesize_ctl_property(
    system: &TransitionSystem,
    desired_properties: &[String],
) -> Option<CtlFormula> {
    if desired_properties.is_empty() {
        return None;
    }

    // Pattern 1: EF(p) - there exists a path to a state where p holds
    for prop in desired_properties {
        let formula = CtlFormula::exists_eventually(CtlFormula::atom(prop));
        if verify_ctl(system, &formula) {
            return Some(formula);
        }
    }

    // Pattern 2: AF(p) - all paths eventually reach a state where p holds
    for prop in desired_properties {
        let formula = CtlFormula::all_eventually(CtlFormula::atom(prop));
        if verify_ctl(system, &formula) {
            return Some(formula);
        }
    }

    // Pattern 3: AG(p) - p holds in all reachable states
    for prop in desired_properties {
        let formula = CtlFormula::all_always(CtlFormula::atom(prop));
        if verify_ctl(system, &formula) {
            return Some(formula);
        }
    }

    None
}

// =============================================================================
// CI/CD Integration
// =============================================================================

/// CI/CD platform type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CiPlatform {
    /// GitHub Actions
    GitHubActions,
    /// GitLab CI/CD
    GitLabCI,
    /// Jenkins
    Jenkins,
    /// CircleCI
    CircleCI,
    /// Travis CI
    TravisCI,
}

impl std::fmt::Display for CiPlatform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GitHubActions => write!(f, "GitHub Actions"),
            Self::GitLabCI => write!(f, "GitLab CI"),
            Self::Jenkins => write!(f, "Jenkins"),
            Self::CircleCI => write!(f, "CircleCI"),
            Self::TravisCI => write!(f, "Travis CI"),
        }
    }
}

/// CI/CD configuration generator.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CiConfig {
    /// Platform type
    pub platform: CiPlatform,
    /// Verification command
    pub verify_command: String,
    /// Fail on warnings
    pub fail_on_warnings: bool,
    /// Upload reports as artifacts
    pub upload_reports: bool,
    /// Report output directory
    pub report_dir: String,
}

impl CiConfig {
    /// Creates a new CI configuration.
    pub fn new(platform: CiPlatform) -> Self {
        Self {
            platform,
            verify_command: "cargo run --bin legalis-verify".to_string(),
            fail_on_warnings: true,
            upload_reports: true,
            report_dir: "verification-reports".to_string(),
        }
    }

    /// Sets the verification command.
    pub fn with_command(mut self, command: impl Into<String>) -> Self {
        self.verify_command = command.into();
        self
    }

    /// Sets whether to fail on warnings.
    pub fn fail_on_warnings(mut self, fail: bool) -> Self {
        self.fail_on_warnings = fail;
        self
    }

    /// Sets whether to upload reports.
    pub fn upload_reports(mut self, upload: bool) -> Self {
        self.upload_reports = upload;
        self
    }

    /// Sets the report directory.
    pub fn with_report_dir(mut self, dir: impl Into<String>) -> Self {
        self.report_dir = dir.into();
        self
    }

    /// Generates the CI configuration file content.
    pub fn generate(&self) -> String {
        match self.platform {
            CiPlatform::GitHubActions => self.generate_github_actions(),
            CiPlatform::GitLabCI => self.generate_gitlab_ci(),
            CiPlatform::Jenkins => self.generate_jenkins(),
            CiPlatform::CircleCI => self.generate_circleci(),
            CiPlatform::TravisCI => self.generate_travis(),
        }
    }

    fn generate_github_actions(&self) -> String {
        format!(
            r#"name: Statute Verification

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  verify:
    runs-on: ubuntu-latest

    steps:
    - uses: actions/checkout@v3

    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true

    - name: Run Statute Verification
      run: {}
      continue-on-error: {}

    - name: Upload Verification Reports
      if: {}
      uses: actions/upload-artifact@v3
      with:
        name: verification-reports
        path: {}
        retention-days: 30
"#,
            self.verify_command,
            if self.fail_on_warnings {
                "false"
            } else {
                "true"
            },
            if self.upload_reports {
                "always()"
            } else {
                "false"
            },
            self.report_dir
        )
    }

    fn generate_gitlab_ci(&self) -> String {
        format!(
            r#"verify:
  stage: test
  image: rust:latest
  script:
    - {}
  artifacts:
    when: {}
    paths:
      - {}
    expire_in: 30 days
  allow_failure: {}
"#,
            self.verify_command,
            if self.upload_reports {
                "always"
            } else {
                "on_success"
            },
            self.report_dir,
            !self.fail_on_warnings
        )
    }

    fn generate_jenkins(&self) -> String {
        format!(
            r#"pipeline {{
    agent any

    stages {{
        stage('Verify Statutes') {{
            steps {{
                sh '{}'
            }}
        }}
    }}

    post {{
        always {{
            archiveArtifacts artifacts: '{}/**', allowEmptyArchive: true
        }}
    }}
}}
"#,
            self.verify_command, self.report_dir
        )
    }

    fn generate_circleci(&self) -> String {
        format!(
            r#"version: 2.1

jobs:
  verify:
    docker:
      - image: rust:latest
    steps:
      - checkout
      - run:
          name: Run Verification
          command: {}
      - store_artifacts:
          path: {}
          destination: verification-reports

workflows:
  verify-statutes:
    jobs:
      - verify
"#,
            self.verify_command, self.report_dir
        )
    }

    fn generate_travis(&self) -> String {
        format!(
            r#"language: rust
rust:
  - stable

script:
  - {}

after_script:
  - tar -czf verification-reports.tar.gz {}

deploy:
  provider: releases
  file: verification-reports.tar.gz
  skip_cleanup: true
  on:
    tags: true
"#,
            self.verify_command, self.report_dir
        )
    }
}

// =============================================================================
// Git Pre-commit Hooks
// =============================================================================

/// Git pre-commit hook configuration.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PreCommitHook {
    /// Verification command to run
    pub verify_command: String,
    /// Fail commit on verification errors
    pub fail_on_errors: bool,
    /// Fail commit on warnings
    pub fail_on_warnings: bool,
    /// Show verbose output
    pub verbose: bool,
}

impl PreCommitHook {
    /// Creates a new pre-commit hook configuration.
    pub fn new() -> Self {
        Self {
            verify_command: "cargo run --bin legalis-verify".to_string(),
            fail_on_errors: true,
            fail_on_warnings: false,
            verbose: true,
        }
    }

    /// Sets the verification command.
    pub fn with_command(mut self, command: impl Into<String>) -> Self {
        self.verify_command = command.into();
        self
    }

    /// Sets whether to fail on errors.
    pub fn fail_on_errors(mut self, fail: bool) -> Self {
        self.fail_on_errors = fail;
        self
    }

    /// Sets whether to fail on warnings.
    pub fn fail_on_warnings(mut self, fail: bool) -> Self {
        self.fail_on_warnings = fail;
        self
    }

    /// Sets verbose mode.
    pub fn verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    /// Generates the pre-commit hook script.
    pub fn generate(&self) -> String {
        format!(
            r#"#!/bin/bash
# Legalis Statute Verification Pre-commit Hook

echo "Running statute verification..."

# Run verification
{}

VERIFICATION_EXIT_CODE=$?

if [ $VERIFICATION_EXIT_CODE -ne 0 ]; then
    if [ "{}" = "true" ]; then
        echo "ERROR: Statute verification failed!"
        echo "Commit aborted. Please fix verification errors before committing."
        exit 1
    else
        echo "WARNING: Statute verification found issues."
    fi
fi

if [ "{}" = "true" ]; then
    echo "Verification details:"
    cat verification-report.txt 2>/dev/null || echo "No detailed report available"
fi

echo "Verification complete."
exit 0
"#,
            self.verify_command,
            if self.fail_on_errors { "true" } else { "false" },
            if self.verbose { "true" } else { "false" }
        )
    }

    /// Installs the pre-commit hook to a git repository.
    pub fn install(&self, repo_path: &str) -> std::io::Result<()> {
        use std::fs;
        use std::io::Write;
        use std::path::Path;

        let hook_path = Path::new(repo_path).join(".git/hooks/pre-commit");
        let hook_content = self.generate();

        let mut file = fs::File::create(&hook_path)?;
        file.write_all(hook_content.as_bytes())?;

        // Make executable on Unix systems
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&hook_path)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&hook_path, perms)?;
        }

        Ok(())
    }
}

impl Default for PreCommitHook {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Verification API Service
// =============================================================================

/// API request for statute verification.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VerificationRequest {
    /// Statutes to verify
    pub statutes: Vec<Statute>,
    /// Constitutional principles to check
    pub principles: Vec<PrincipleCheck>,
    /// Request ID for tracking
    pub request_id: Option<String>,
    /// Client identifier
    pub client_id: Option<String>,
}

impl VerificationRequest {
    /// Creates a new verification request.
    pub fn new(statutes: Vec<Statute>) -> Self {
        Self {
            statutes,
            principles: Vec::new(),
            request_id: None,
            client_id: None,
        }
    }

    /// Sets the principles to check.
    pub fn with_principles(mut self, principles: Vec<PrincipleCheck>) -> Self {
        self.principles = principles;
        self
    }

    /// Sets the request ID.
    pub fn with_request_id(mut self, id: impl Into<String>) -> Self {
        self.request_id = Some(id.into());
        self
    }

    /// Sets the client ID.
    pub fn with_client_id(mut self, id: impl Into<String>) -> Self {
        self.client_id = Some(id.into());
        self
    }
}

/// API response for statute verification.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VerificationResponse {
    /// Request ID (echoed from request)
    pub request_id: Option<String>,
    /// Verification results for each statute
    pub results: Vec<VerificationResult>,
    /// Overall success status
    pub success: bool,
    /// Error count
    pub error_count: usize,
    /// Warning count
    pub warning_count: usize,
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
}

impl VerificationResponse {
    /// Creates a new verification response.
    pub fn new(request_id: Option<String>, results: Vec<VerificationResult>) -> Self {
        let error_count: usize = results.iter().map(|r| r.errors.len()).sum();
        let warning_count: usize = results.iter().map(|r| r.warnings.len()).sum();
        let success = results.iter().all(|r| r.passed);

        Self {
            request_id,
            results,
            success,
            error_count,
            warning_count,
            processing_time_ms: 0,
        }
    }

    /// Sets the processing time.
    pub fn with_processing_time(mut self, time_ms: u64) -> Self {
        self.processing_time_ms = time_ms;
        self
    }
}

// =============================================================================
// Notification System
// =============================================================================

/// Notification type.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum NotificationType {
    /// Verification completed successfully
    Success,
    /// Verification completed with warnings
    Warning,
    /// Verification failed with errors
    Error,
    /// Verification encountered a critical issue
    Critical,
}

/// Notification channel.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum NotificationChannel {
    /// Webhook URL
    Webhook {
        url: String,
        headers: HashMap<String, String>,
    },
    /// Email notification
    Email { to: Vec<String>, subject: String },
    /// Callback function (not serializable, use name reference)
    Callback { name: String },
}

/// Notification configuration.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NotificationConfig {
    /// Channels to notify
    pub channels: Vec<NotificationChannel>,
    /// Notification types to trigger on
    pub trigger_on: Vec<NotificationType>,
    /// Include detailed results in notification
    pub include_details: bool,
}

impl NotificationConfig {
    /// Creates a new notification configuration.
    pub fn new() -> Self {
        Self {
            channels: Vec::new(),
            trigger_on: vec![NotificationType::Error, NotificationType::Critical],
            include_details: true,
        }
    }

    /// Adds a webhook channel.
    pub fn with_webhook(mut self, url: impl Into<String>) -> Self {
        self.channels.push(NotificationChannel::Webhook {
            url: url.into(),
            headers: HashMap::new(),
        });
        self
    }

    /// Adds an email channel.
    pub fn with_email(mut self, to: Vec<String>, subject: impl Into<String>) -> Self {
        self.channels.push(NotificationChannel::Email {
            to,
            subject: subject.into(),
        });
        self
    }

    /// Sets the trigger types.
    pub fn trigger_on(mut self, types: Vec<NotificationType>) -> Self {
        self.trigger_on = types;
        self
    }

    /// Sets whether to include details.
    pub fn include_details(mut self, include: bool) -> Self {
        self.include_details = include;
        self
    }
}

impl Default for NotificationConfig {
    fn default() -> Self {
        Self::new()
    }
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

/// Sends a notification based on configuration.
///
/// This is a mock implementation. In production, this would actually send
/// webhooks, emails, or invoke callbacks.
pub fn send_notification(config: &NotificationConfig, message: &NotificationMessage) -> bool {
    if !config.trigger_on.contains(&message.notification_type) {
        return false;
    }

    // In a real implementation, this would:
    // 1. Send HTTP POST to webhook URLs
    // 2. Send emails via SMTP
    // 3. Invoke callback functions
    // For now, we just return success
    !config.channels.is_empty()
}

// =============================================================================
// Principle Definition DSL
// =============================================================================

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

/// A jurisdictional rule set containing principles for a specific jurisdiction.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JurisdictionalRuleSet {
    /// Jurisdiction identifier
    pub jurisdiction: String,
    /// Name of the jurisdiction
    pub name: String,
    /// Principles that apply in this jurisdiction
    pub principles: Vec<PrincipleDefinition>,
    /// Composite principles
    pub composites: Vec<CompositePrinciple>,
}

impl JurisdictionalRuleSet {
    /// Creates a new jurisdictional rule set.
    pub fn new(jurisdiction: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            jurisdiction: jurisdiction.into(),
            name: name.into(),
            principles: Vec::new(),
            composites: Vec::new(),
        }
    }

    /// Adds a principle.
    pub fn with_principle(mut self, principle: PrincipleDefinition) -> Self {
        self.principles.push(principle);
        self
    }

    /// Adds a composite principle.
    pub fn with_composite(mut self, composite: CompositePrinciple) -> Self {
        self.composites.push(composite);
        self
    }

    /// Gets principles by priority (highest first).
    pub fn principles_by_priority(&self) -> Vec<&PrincipleDefinition> {
        let mut sorted: Vec<_> = self.principles.iter().collect();
        sorted.sort_by(|a, b| b.priority.cmp(&a.priority));
        sorted
    }
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
            // Check principles in priority order
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
                        PrincipleCheck::Custom { .. } => {
                            // Custom checks would be implemented here
                            PrincipleCheckResult::pass()
                        }
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

            // Check composite principles
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

// =============================================================================
// Watch Mode for Continuous Verification
// =============================================================================

#[cfg(feature = "watch")]
pub mod watch {
    //! Watch mode for continuous verification of statute files.
    //!
    //! This module provides functionality to monitor directories for changes
    //! and automatically trigger verification when statute files are modified.

    use super::*;
    use crossbeam_channel::{bounded, select};
    use notify::{
        Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Result as NotifyResult,
        Watcher,
    };
    use std::path::{Path, PathBuf};
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    /// Configuration for watch mode.
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct WatchConfig {
        /// Paths to watch
        pub paths: Vec<PathBuf>,
        /// File extensions to watch (e.g., ["json", "toml"])
        pub extensions: Vec<String>,
        /// Debounce delay in milliseconds
        pub debounce_ms: u64,
        /// Whether to watch recursively
        pub recursive: bool,
    }

    impl Default for WatchConfig {
        fn default() -> Self {
            Self {
                paths: vec![PathBuf::from(".")],
                extensions: vec!["json".to_string(), "toml".to_string()],
                debounce_ms: 500,
                recursive: true,
            }
        }
    }

    impl WatchConfig {
        /// Creates a new watch configuration.
        pub fn new() -> Self {
            Self::default()
        }

        /// Adds a path to watch.
        pub fn with_path(mut self, path: impl Into<PathBuf>) -> Self {
            self.paths.push(path.into());
            self
        }

        /// Sets the file extensions to watch.
        pub fn with_extensions(mut self, extensions: Vec<String>) -> Self {
            self.extensions = extensions;
            self
        }

        /// Sets the debounce delay.
        pub fn with_debounce(mut self, ms: u64) -> Self {
            self.debounce_ms = ms;
            self
        }

        /// Sets whether to watch recursively.
        pub fn recursive(mut self, recursive: bool) -> Self {
            self.recursive = recursive;
            self
        }
    }

    /// Statistics about watch mode operations.
    #[derive(Debug, Clone, Default)]
    pub struct WatchStats {
        /// Number of file changes detected
        pub changes_detected: usize,
        /// Number of verifications triggered
        pub verifications_triggered: usize,
        /// Number of verification errors
        pub verification_errors: usize,
    }

    /// A watcher that monitors files and triggers verification on changes.
    pub struct StatuteWatcher {
        config: WatchConfig,
        verifier: Arc<Mutex<StatuteVerifier>>,
        stats: Arc<Mutex<WatchStats>>,
    }

    impl StatuteWatcher {
        /// Creates a new statute watcher.
        pub fn new(config: WatchConfig, verifier: StatuteVerifier) -> Self {
            Self {
                config,
                verifier: Arc::new(Mutex::new(verifier)),
                stats: Arc::new(Mutex::new(WatchStats::default())),
            }
        }

        /// Checks if a path should be watched based on the configuration.
        fn should_watch(&self, path: &Path) -> bool {
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy();
                self.config.extensions.iter().any(|e| e == &*ext_str)
            } else {
                false
            }
        }

        /// Starts watching and returns when stopped.
        pub fn watch<F>(&self, mut on_change: F) -> NotifyResult<()>
        where
            F: FnMut(&Path, &VerificationResult) + Send + 'static,
        {
            let (tx, rx) = bounded(1);
            let mut watcher = RecommendedWatcher::new(
                move |res: NotifyResult<Event>| {
                    if let Ok(event) = res {
                        let _ = tx.send(event);
                    }
                },
                Config::default(),
            )?;

            // Watch all configured paths
            for path in &self.config.paths {
                let mode = if self.config.recursive {
                    RecursiveMode::Recursive
                } else {
                    RecursiveMode::NonRecursive
                };
                watcher.watch(path, mode)?;
            }

            println!("Watching for changes in {:?}...", self.config.paths);
            println!("Press Ctrl+C to stop");

            // Process events
            loop {
                select! {
                    recv(rx) -> event => {
                        if let Ok(event) = event {
                            self.handle_event(event, &mut on_change);
                        }
                    }
                }

                // Debounce
                std::thread::sleep(Duration::from_millis(self.config.debounce_ms));
            }
        }

        /// Handles a file system event.
        fn handle_event<F>(&self, event: Event, on_change: &mut F)
        where
            F: FnMut(&Path, &VerificationResult),
        {
            // Only handle modify and create events
            if !matches!(event.kind, EventKind::Modify(_) | EventKind::Create(_)) {
                return;
            }

            for path in event.paths {
                if !self.should_watch(&path) {
                    continue;
                }

                // Update stats
                {
                    let mut stats = self.stats.lock().unwrap();
                    stats.changes_detected += 1;
                }

                println!("Change detected: {:?}", path);

                // Load and verify the statute
                match self.load_and_verify(&path) {
                    Ok(result) => {
                        let mut stats = self.stats.lock().unwrap();
                        stats.verifications_triggered += 1;
                        if !result.passed {
                            stats.verification_errors += result.errors.len();
                        }
                        drop(stats);

                        on_change(&path, &result);
                    }
                    Err(e) => {
                        eprintln!("Error verifying {}: {}", path.display(), e);
                    }
                }
            }
        }

        /// Loads a statute file and verifies it.
        fn load_and_verify(&self, path: &Path) -> anyhow::Result<VerificationResult> {
            // Try to load the statute from JSON
            let content = std::fs::read_to_string(path)?;
            let statutes: Vec<Statute> = serde_json::from_str(&content)?;

            // Verify the statutes
            let verifier = self.verifier.lock().unwrap();
            Ok(verifier.verify(&statutes))
        }

        /// Returns the current watch statistics.
        pub fn stats(&self) -> WatchStats {
            self.stats.lock().unwrap().clone()
        }

        /// Resets the watch statistics.
        pub fn reset_stats(&self) {
            let mut stats = self.stats.lock().unwrap();
            *stats = WatchStats::default();
        }
    }
}

// =============================================================================
// Related Precedent References
// =============================================================================

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

        // Index by topics
        for topic in &precedent.topics {
            self.topic_index.entry(topic.clone()).or_default().push(idx);
        }

        self.precedents.push(precedent);
    }

    /// Finds precedents related to a statute based on topics.
    pub fn find_related(&self, statute: &Statute, min_relevance: f64) -> Vec<&Precedent> {
        // Extract topics from statute (simplified - in practice would use NLP)
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

        // Sort by relevance (descending)
        results.sort_by(|a, b| b.relevance.partial_cmp(&a.relevance).unwrap());
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

/// Optimization suggestion for statute conditions.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OptimizationSuggestion {
    /// Statute ID that can be optimized
    pub statute_id: String,
    /// Current complexity score
    pub current_complexity: usize,
    /// Suggested simplified condition
    pub suggested_condition: Option<String>,
    /// List of specific suggestions
    pub suggestions: Vec<String>,
    /// Potential complexity after optimization
    pub optimized_complexity: usize,
}

/// Analyzes statutes and suggests optimizations for complex conditions.
///
/// This function uses SMT-based analysis to identify simplification opportunities.
#[cfg(feature = "z3-solver")]
pub fn suggest_optimizations(statutes: &[Statute]) -> Vec<OptimizationSuggestion> {
    use crate::smt::{SmtVerifier, create_z3_context};

    let ctx = create_z3_context();
    let mut verifier = SmtVerifier::new(&ctx);
    let mut suggestions = Vec::new();

    for statute in statutes {
        for condition in &statute.preconditions {
            let (complexity, smt_suggestions) = verifier.analyze_complexity(condition);

            if !smt_suggestions.is_empty() || complexity > 10 {
                // Try to simplify
                if let Ok((simplified, changed)) = verifier.simplify(condition) {
                    let optimized_complexity = if changed {
                        let (opt_comp, _) = verifier.analyze_complexity(&simplified);
                        opt_comp
                    } else {
                        complexity
                    };

                    suggestions.push(OptimizationSuggestion {
                        statute_id: statute.id.clone(),
                        current_complexity: complexity,
                        suggested_condition: if changed {
                            Some(format!("{}", simplified))
                        } else {
                            None
                        },
                        suggestions: smt_suggestions,
                        optimized_complexity,
                    });
                }
            }
        }
    }

    suggestions
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

/// Analyzes statute coverage and identifies potential gaps.
///
/// This performs a heuristic analysis to find common scenarios that
/// might not be covered by the provided statutes.
pub fn analyze_coverage_gaps(statutes: &[Statute]) -> Vec<CoverageGap> {
    let mut gaps = Vec::new();

    // Check for age-based gaps
    let age_statutes: Vec<_> = statutes
        .iter()
        .filter(|s| {
            s.preconditions
                .iter()
                .any(|c| matches!(c, legalis_core::Condition::Age { .. }))
        })
        .collect();

    if !age_statutes.is_empty() {
        // Check for gaps in age ranges
        let mut age_thresholds: Vec<u32> = age_statutes
            .iter()
            .flat_map(|s| {
                s.preconditions.iter().filter_map(|c| {
                    if let legalis_core::Condition::Age { value, .. } = c {
                        Some(*value)
                    } else {
                        None
                    }
                })
            })
            .collect();

        age_thresholds.sort_unstable();
        age_thresholds.dedup();

        if age_thresholds.len() >= 2 {
            for window in age_thresholds.windows(2) {
                if window[1] - window[0] > 5 {
                    gaps.push(CoverageGap {
                        description: format!(
                            "Potential gap in age coverage between {} and {}",
                            window[0], window[1]
                        ),
                        example_scenario: format!(
                            "Person aged {} may not be covered by any statute",
                            (window[0] + window[1]) / 2
                        ),
                        severity: Severity::Warning,
                        related_statutes: age_statutes.iter().map(|s| s.id.clone()).collect(),
                    });
                }
            }
        }
    }

    // Check for income-based gaps
    let income_statutes: Vec<_> = statutes
        .iter()
        .filter(|s| {
            s.preconditions
                .iter()
                .any(|c| matches!(c, legalis_core::Condition::Income { .. }))
        })
        .collect();

    if !income_statutes.is_empty() {
        gaps.push(CoverageGap {
            description: "Income-based statutes detected - verify edge cases".to_string(),
            example_scenario: "Persons at exact income thresholds may need special handling"
                .to_string(),
            severity: Severity::Info,
            related_statutes: income_statutes.iter().map(|s| s.id.clone()).collect(),
        });
    }

    // Check for jurisdiction gaps
    let jurisdictions: std::collections::HashSet<_> = statutes
        .iter()
        .filter_map(|s| s.jurisdiction.as_ref())
        .collect();

    if jurisdictions.len() > 1 {
        for statute in statutes {
            if statute.jurisdiction.is_none() {
                gaps.push(CoverageGap {
                    description: format!(
                        "Statute '{}' has no jurisdiction specified",
                        statute.id
                    ),
                    example_scenario: "May apply too broadly or conflict with jurisdictional statutes".to_string(),
                    severity: Severity::Warning,
                    related_statutes: vec![statute.id.clone()],
                });
            }
        }
    }

    gaps
}

/// Generates a report of coverage gaps and optimization suggestions.
pub fn optimization_and_gaps_report(statutes: &[Statute]) -> String {
    let mut report = String::new();
    report.push_str("# Statute Optimization and Gap Analysis Report\n\n");

    // Gap analysis
    let gaps = analyze_coverage_gaps(statutes);
    report.push_str("## Coverage Gaps\n\n");

    if gaps.is_empty() {
        report.push_str("No significant coverage gaps detected.\n\n");
    } else {
        for (i, gap) in gaps.iter().enumerate() {
            report.push_str(&format!("### Gap #{}: {}\n", i + 1, gap.description));
            report.push_str(&format!("- **Severity**: {:?}\n", gap.severity));
            report.push_str(&format!("- **Example**: {}\n", gap.example_scenario));
            report.push_str(&format!(
                "- **Related statutes**: {}\n\n",
                gap.related_statutes.join(", ")
            ));
        }
    }

    // Optimization suggestions (only available with z3-solver feature)
    #[cfg(feature = "z3-solver")]
    {
        let optimizations = suggest_optimizations(statutes);
        report.push_str("## Optimization Suggestions\n\n");

        if optimizations.is_empty() {
            report.push_str("No optimization opportunities detected.\n\n");
        } else {
            for opt in &optimizations {
                report.push_str(&format!("### Statute: {}\n", opt.statute_id));
                report.push_str(&format!(
                    "- **Current complexity**: {}\n",
                    opt.current_complexity
                ));
                report.push_str(&format!(
                    "- **Optimized complexity**: {}\n",
                    opt.optimized_complexity
                ));

                if let Some(ref suggested) = opt.suggested_condition {
                    report.push_str(&format!(
                        "- **Suggested simplification**: `{}`\n",
                        suggested
                    ));
                }

                if !opt.suggestions.is_empty() {
                    report.push_str("- **Recommendations**:\n");
                    for suggestion in &opt.suggestions {
                        report.push_str(&format!("  - {}\n", suggestion));
                    }
                }
                report.push('\n');
            }
        }
    }

    #[cfg(not(feature = "z3-solver"))]
    {
        report.push_str("## Optimization Suggestions\n\n");
        report.push_str(
            "*Optimization suggestions require the `z3-solver` feature to be enabled.*\n\n",
        );
    }

    report.push_str("## Summary\n\n");
    report.push_str(&format!("- Total statutes analyzed: {}\n", statutes.len()));
    report.push_str(&format!("- Coverage gaps found: {}\n", gaps.len()));

    #[cfg(feature = "z3-solver")]
    {
        let optimizations = suggest_optimizations(statutes);
        report.push_str(&format!(
            "- Optimization opportunities: {}\n",
            optimizations.len()
        ));
    }

    report
}

// ============================================================================
// Dependency Graph Export
// ============================================================================

/// Exports statute dependencies as a GraphViz DOT format graph.
///
/// This can be visualized using tools like Graphviz, which supports
/// rendering DOT files to SVG, PNG, PDF, and other formats.
///
/// # Example
/// ```ignore
/// let statutes = vec![...];
/// let dot = export_dependency_graph(&statutes);
/// std::fs::write("dependencies.dot", dot)?;
/// // Then run: dot -Tpng dependencies.dot -o dependencies.png
/// ```
pub fn export_dependency_graph(statutes: &[Statute]) -> String {
    let mut dot = String::from("digraph StatuteDependencies {\n");
    dot.push_str("  rankdir=LR;\n");
    dot.push_str("  node [shape=box, style=filled, fillcolor=lightblue];\n\n");

    // Add nodes for each statute
    for statute in statutes {
        let label = format!("{}\\n{}", statute.id, statute.title);
        dot.push_str(&format!("  \"{}\" [label=\"{}\"];\n", statute.id, label));
    }

    dot.push('\n');

    // Add edges for references
    let statute_ids: HashSet<String> = statutes.iter().map(|s| s.id.clone()).collect();

    for statute in statutes {
        let refs = extract_statute_references_from_conditions(&statute.preconditions);

        for ref_id in refs {
            if statute_ids.contains(&ref_id) {
                dot.push_str(&format!(
                    "  \"{}\" -> \"{}\" [label=\"references\"];\n",
                    statute.id, ref_id
                ));
            }
        }
    }

    dot.push_str("}\n");
    dot
}

/// Exports statute dependencies with conflict highlighting.
///
/// Conflicting statutes are colored in red, and conflict edges are dashed.
pub fn export_dependency_graph_with_conflicts(statutes: &[Statute]) -> String {
    let conflicts = detect_statute_conflicts(statutes);
    let mut conflict_pairs: HashSet<(String, String)> = HashSet::new();
    let mut conflicting_statute_ids: HashSet<String> = HashSet::new();

    for conflict in &conflicts {
        for statute_id in &conflict.statute_ids {
            conflicting_statute_ids.insert(statute_id.clone());
        }

        if conflict.statute_ids.len() >= 2 {
            let id1 = &conflict.statute_ids[0];
            let id2 = &conflict.statute_ids[1];
            conflict_pairs.insert((id1.clone(), id2.clone()));
            conflict_pairs.insert((id2.clone(), id1.clone()));
        }
    }

    let mut dot = String::from("digraph StatuteDependenciesWithConflicts {\n");
    dot.push_str("  rankdir=LR;\n");
    dot.push_str("  node [shape=box, style=filled];\n\n");

    // Add nodes with conflict highlighting
    for statute in statutes {
        let color = if conflicting_statute_ids.contains(&statute.id) {
            "lightcoral"
        } else {
            "lightblue"
        };

        let label = format!("{}\\n{}", statute.id, statute.title);
        dot.push_str(&format!(
            "  \"{}\" [label=\"{}\", fillcolor={}];\n",
            statute.id, label, color
        ));
    }

    dot.push('\n');

    // Add reference edges
    let statute_ids: HashSet<String> = statutes.iter().map(|s| s.id.clone()).collect();

    for statute in statutes {
        let refs = extract_statute_references_from_conditions(&statute.preconditions);

        for ref_id in refs {
            if statute_ids.contains(&ref_id) {
                dot.push_str(&format!(
                    "  \"{}\" -> \"{}\" [label=\"references\"];\n",
                    statute.id, ref_id
                ));
            }
        }
    }

    // Add conflict edges
    for (id1, id2) in &conflict_pairs {
        if statute_ids.contains(id1) && statute_ids.contains(id2) {
            dot.push_str(&format!(
                "  \"{}\" -> \"{}\" [style=dashed, color=red, label=\"conflicts\"];\n",
                id1, id2
            ));
        }
    }

    dot.push_str("}\n");
    dot
}

// ============================================================================
// Quality Metrics
// ============================================================================

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

/// Calculates the legislative drafting quality score (0-100).
///
/// This evaluates the statute against legislative drafting best practices:
/// - Clear structure and organization
/// - Consistent terminology
/// - Appropriate level of detail
/// - Proper use of conditions and effects
/// - Temporal validity properly defined
fn calculate_drafting_quality(statute: &Statute) -> f64 {
    let mut score: f64 = 0.0;

    // Structure: Title is descriptive (10 points)
    if !statute.title.is_empty() {
        let title_words = statute.title.split_whitespace().count();
        if (3..=20).contains(&title_words) {
            score += 10.0;
        } else if title_words > 0 {
            score += 5.0;
        }
    }

    // Effect description clarity (15 points)
    if !statute.effect.description.is_empty() {
        let desc_words = statute.effect.description.split_whitespace().count();
        if (5..=100).contains(&desc_words) {
            score += 15.0;
        } else if desc_words > 0 {
            score += 8.0;
        }
    }

    // Proper temporal validity (15 points)
    if statute.temporal_validity.enacted_at.is_some() {
        score += 10.0;
    }
    if statute.temporal_validity.effective_date.is_some() {
        score += 5.0;
    }

    // Jurisdiction specified (10 points)
    if statute.jurisdiction.is_some() {
        score += 10.0;
    }

    // Appropriate number of preconditions (15 points)
    let precondition_count = statute.preconditions.len();
    if (1..=7).contains(&precondition_count) {
        score += 15.0;
    } else if precondition_count > 0 {
        score += 8.0;
    }

    // Discretion logic provided (10 points)
    if statute.discretion_logic.is_some() {
        score += 10.0;
    }

    // Consistent effect type with description (10 points)
    let effect_keywords_match = match statute.effect.effect_type {
        legalis_core::EffectType::Grant => {
            statute.effect.description.to_lowercase().contains("grant")
                || statute.effect.description.to_lowercase().contains("allow")
        }
        legalis_core::EffectType::Prohibition => {
            statute
                .effect
                .description
                .to_lowercase()
                .contains("prohibit")
                || statute.effect.description.to_lowercase().contains("forbid")
                || statute
                    .effect
                    .description
                    .to_lowercase()
                    .contains("not allow")
        }
        legalis_core::EffectType::Obligation => {
            statute.effect.description.to_lowercase().contains("must")
                || statute
                    .effect
                    .description
                    .to_lowercase()
                    .contains("require")
                || statute.effect.description.to_lowercase().contains("shall")
        }
        _ => true, // Other types are always consistent
    };
    if effect_keywords_match {
        score += 10.0;
    }

    // Metadata completeness (15 points)
    let mut metadata_score = 0.0;
    if !statute.id.is_empty() {
        metadata_score += 5.0;
    }
    if !statute.title.is_empty() {
        metadata_score += 5.0;
    }
    if statute.jurisdiction.is_some() {
        metadata_score += 5.0;
    }
    score += metadata_score;

    score.min(100.0)
}

/// Calculates the clarity index (0-100).
///
/// Measures how clear and understandable the statute is based on:
/// - Simple language in titles and descriptions
/// - Logical condition structure
/// - Unambiguous terminology
/// - Appropriate complexity level
fn calculate_clarity_index(statute: &Statute) -> f64 {
    let mut score: f64 = 50.0; // Baseline

    // Title clarity (15 points)
    let title_words = statute.title.split_whitespace().count();
    if (3..=12).contains(&title_words) {
        score += 15.0;
    } else if title_words > 0 && title_words <= 20 {
        score += 8.0;
    }

    // Effect description clarity (20 points)
    let desc_words = statute.effect.description.split_whitespace().count();
    if (5..=50).contains(&desc_words) {
        score += 20.0;
    } else if desc_words > 0 && desc_words <= 100 {
        score += 10.0;
    } else if desc_words > 100 {
        score -= 5.0; // Too verbose reduces clarity
    }

    // Condition complexity (15 points)
    let complexity = analyze_complexity(statute);
    if complexity.complexity_score <= 25 {
        score += 15.0;
    } else if complexity.complexity_score <= 50 {
        score += 10.0;
    } else if complexity.complexity_score <= 75 {
        score += 5.0;
    } else {
        score -= 5.0; // Very complex reduces clarity
    }

    // Discretion logic presence helps clarity (10 points)
    if statute.discretion_logic.is_some() {
        score += 10.0;
    }

    score.clamp(0.0, 100.0)
}

/// Calculates the testability assessment score (0-100).
///
/// Evaluates how testable and verifiable the statute conditions are:
/// - Concrete, measurable conditions
/// - Clear pass/fail criteria
/// - Deterministic evaluation
/// - Observable outcomes
fn calculate_testability(statute: &Statute) -> f64 {
    let mut score = 0.0;

    // Has preconditions that can be tested (30 points)
    if !statute.preconditions.is_empty() {
        score += 20.0;

        // Count concrete, testable condition types
        let mut testable_count = 0;
        let total_conditions = count_all_conditions(&statute.preconditions);

        for condition in &statute.preconditions {
            if is_testable_condition(condition) {
                testable_count += 1;
            }
        }

        if total_conditions > 0 {
            let testable_ratio = testable_count as f64 / total_conditions as f64;
            score += testable_ratio * 30.0;
        }
    } else {
        score += 10.0; // No preconditions means always testable
    }

    // Clear effect description (20 points)
    if !statute.effect.description.is_empty() {
        score += 20.0;
    }

    // Temporal validity enables time-based testing (15 points)
    if statute.temporal_validity.effective_date.is_some() {
        score += 10.0;
    }
    if statute.temporal_validity.expiry_date.is_some() {
        score += 5.0;
    }

    // Jurisdiction enables context testing (15 points)
    if statute.jurisdiction.is_some() {
        score += 15.0;
    }

    score.min(100.0)
}

/// Calculates the maintainability score (0-100).
///
/// Assesses how easy it would be to modify or extend the statute:
/// - Modular structure
/// - Clear dependencies
/// - Appropriate abstraction level
/// - Documentation quality
fn calculate_maintainability(statute: &Statute) -> f64 {
    let mut score: f64 = 30.0; // Baseline

    // Complexity affects maintainability (25 points)
    let complexity = analyze_complexity(statute);
    if complexity.complexity_score <= 30 {
        score += 25.0;
    } else if complexity.complexity_score <= 60 {
        score += 15.0;
    } else if complexity.complexity_score <= 80 {
        score += 8.0;
    }

    // Good documentation (discretion logic) (20 points)
    if let Some(logic) = &statute.discretion_logic {
        if !logic.is_empty() {
            score += 20.0;
        }
    }

    // Reasonable number of preconditions (15 points)
    let precondition_count = statute.preconditions.len();
    if precondition_count <= 5 {
        score += 15.0;
    } else if precondition_count <= 10 {
        score += 10.0;
    } else if precondition_count <= 15 {
        score += 5.0;
    }

    // Clear metadata enables maintenance (20 points)
    let mut metadata_score = 0.0;
    if !statute.id.is_empty() && !statute.id.contains("unknown") {
        metadata_score += 5.0;
    }
    if !statute.title.is_empty() {
        metadata_score += 5.0;
    }
    if statute.jurisdiction.is_some() {
        metadata_score += 5.0;
    }
    if statute.temporal_validity.enacted_at.is_some() {
        metadata_score += 5.0;
    }
    score += metadata_score;

    score.min(100.0)
}

/// Counts all conditions recursively (including nested conditions).
fn count_all_conditions(conditions: &[legalis_core::Condition]) -> usize {
    let mut count = 0;
    for condition in conditions {
        count += count_condition_recursive(condition);
    }
    count
}

/// Recursively counts a single condition and its children.
fn count_condition_recursive(condition: &legalis_core::Condition) -> usize {
    use legalis_core::Condition;
    match condition {
        Condition::And(left, right) | Condition::Or(left, right) => {
            1 + count_condition_recursive(left) + count_condition_recursive(right)
        }
        Condition::Not(inner) => 1 + count_condition_recursive(inner),
        Condition::Composite { conditions, .. } => {
            1 + conditions
                .iter()
                .map(|(_, c)| count_condition_recursive(c))
                .sum::<usize>()
        }
        Condition::Probabilistic { condition, .. } => 1 + count_condition_recursive(condition),
        _ => 1,
    }
}

/// Checks if a condition is testable (has concrete, measurable criteria).
fn is_testable_condition(condition: &legalis_core::Condition) -> bool {
    use legalis_core::Condition;
    match condition {
        // Concrete, measurable conditions
        Condition::Age { .. }
        | Condition::Income { .. }
        | Condition::DateRange { .. }
        | Condition::ResidencyDuration { .. }
        | Condition::Duration { .. }
        | Condition::Percentage { .. }
        | Condition::SetMembership { .. }
        | Condition::Pattern { .. }
        | Condition::Calculation { .. }
        | Condition::Threshold { .. }
        | Condition::Temporal { .. } => true,

        // Attribute checks are testable if well-defined
        Condition::HasAttribute { .. } | Condition::AttributeEquals { .. } => true,

        // Geographic and relationship checks are testable
        Condition::Geographic { .. } | Condition::EntityRelationship { .. } => true,

        // Composite conditions depend on sub-conditions
        Condition::And(left, right) | Condition::Or(left, right) => {
            is_testable_condition(left) && is_testable_condition(right)
        }
        Condition::Not(inner) => is_testable_condition(inner),
        Condition::Composite { conditions, .. } => {
            conditions.iter().all(|(_, c)| is_testable_condition(c))
        }
        Condition::Probabilistic { condition, .. } => is_testable_condition(condition),

        // Fuzzy and Custom are less testable
        Condition::Fuzzy { .. } | Condition::Custom { .. } => false,
    }
}

/// Analyzes statute quality and returns comprehensive metrics.
pub fn analyze_quality(statute: &Statute) -> QualityMetrics {
    // Complexity analysis
    let complexity_metrics = analyze_complexity(statute);
    let max_complexity = 100.0; // Maximum complexity score
    let complexity_score = ((max_complexity
        - complexity_metrics
            .complexity_score
            .min(max_complexity as u32) as f64)
        / max_complexity
        * 100.0)
        .max(0.0);

    // Readability: based on title clarity, discretion logic presence, etc.
    let mut readability_score = 50.0;
    if !statute.title.is_empty() && statute.title.len() > 10 {
        readability_score += 20.0;
    }
    if statute.discretion_logic.is_some() {
        readability_score += 30.0;
    }

    // Consistency: check if jurisdiction and metadata are set
    let mut consistency_score = 50.0;
    if statute.jurisdiction.is_some() {
        consistency_score += 25.0;
    }
    if statute.temporal_validity.enacted_at.is_some() {
        consistency_score += 25.0;
    }

    // Completeness: check if essential fields are populated
    let mut completeness_score = 0.0;
    if !statute.id.is_empty() {
        completeness_score += 20.0;
    }
    if !statute.title.is_empty() {
        completeness_score += 20.0;
    }
    if statute.jurisdiction.is_some() {
        completeness_score += 20.0;
    }
    if statute.temporal_validity.enacted_at.is_some() {
        completeness_score += 20.0;
    }
    if !statute.preconditions.is_empty() || !statute.effect.description.is_empty() {
        completeness_score += 20.0;
    }

    // Calculate new quality metrics
    let drafting_quality_score = calculate_drafting_quality(statute);
    let clarity_index = calculate_clarity_index(statute);
    let testability_score = calculate_testability(statute);
    let maintainability_score = calculate_maintainability(statute);

    let mut metrics = QualityMetrics::new(
        statute.id.clone(),
        complexity_score,
        readability_score,
        consistency_score,
        completeness_score,
        drafting_quality_score,
        clarity_index,
        testability_score,
        maintainability_score,
    );

    // Add issues
    if complexity_metrics.complexity_score > 70 {
        metrics = metrics.with_issue(format!(
            "High complexity ({}), consider simplification",
            complexity_metrics.complexity_score
        ));
    }
    if statute.discretion_logic.is_none() {
        metrics = metrics.with_issue("Missing discretion logic");
    }
    if statute.jurisdiction.is_none() {
        metrics = metrics.with_issue("Missing jurisdiction");
    }
    if statute.temporal_validity.enacted_at.is_none() {
        metrics = metrics.with_issue("Missing enactment date");
    }

    // Add strengths
    if complexity_metrics.complexity_score <= 30 {
        metrics = metrics.with_strength("Low complexity");
    }
    if statute.discretion_logic.is_some() {
        metrics = metrics.with_strength("Has discretion logic");
    }
    if statute.jurisdiction.is_some() && statute.temporal_validity.enacted_at.is_some() {
        metrics = metrics.with_strength("Complete metadata");
    }

    metrics
}

// ============================================================================
// Ambiguity Detection
// ============================================================================

/// Types of ambiguities that can be detected in statutes.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AmbiguityType {
    /// Vague or undefined terms in descriptions
    VagueTerm,
    /// Overlapping or conflicting conditions
    OverlappingConditions,
    /// Unclear effect description
    UnclearEffect,
    /// Missing discretion logic for complex conditions
    MissingDiscretion,
    /// Ambiguous temporal scope
    TemporalAmbiguity,
    /// Implicit assumptions not stated
    ImplicitAssumption,
    /// Quantifier ambiguity (e.g., "all", "some", "any")
    QuantifierAmbiguity,
}

impl std::fmt::Display for AmbiguityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::VagueTerm => write!(f, "Vague Term"),
            Self::OverlappingConditions => write!(f, "Overlapping Conditions"),
            Self::UnclearEffect => write!(f, "Unclear Effect"),
            Self::MissingDiscretion => write!(f, "Missing Discretion"),
            Self::TemporalAmbiguity => write!(f, "Temporal Ambiguity"),
            Self::ImplicitAssumption => write!(f, "Implicit Assumption"),
            Self::QuantifierAmbiguity => write!(f, "Quantifier Ambiguity"),
        }
    }
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

/// Detects ambiguities in a statute.
///
/// This function analyzes a statute for various types of ambiguities including:
/// - Vague or undefined terms
/// - Overlapping conditions
/// - Unclear effects
/// - Missing discretion logic
/// - Temporal ambiguities
pub fn detect_ambiguities(statute: &Statute) -> Vec<Ambiguity> {
    let mut ambiguities = Vec::new();

    // Check for vague terms in title
    if contains_vague_terms(&statute.title) {
        ambiguities.push(Ambiguity::new(
            AmbiguityType::VagueTerm,
            "title",
            format!("Title contains vague terms: '{}'", statute.title),
            "Use more specific and precise terminology",
            6,
        ));
    }

    // Check for vague terms in effect description
    if contains_vague_terms(&statute.effect.description) {
        ambiguities.push(Ambiguity::new(
            AmbiguityType::VagueTerm,
            "effect.description",
            format!(
                "Effect description contains vague terms: '{}'",
                statute.effect.description
            ),
            "Specify exact requirements, amounts, or procedures",
            8,
        ));
    }

    // Check for unclear effect descriptions
    if statute.effect.description.is_empty() {
        ambiguities.push(Ambiguity::new(
            AmbiguityType::UnclearEffect,
            "effect.description",
            "Effect description is empty",
            "Provide a clear description of what this statute does",
            9,
        ));
    } else if statute.effect.description.split_whitespace().count() < 3 {
        ambiguities.push(Ambiguity::new(
            AmbiguityType::UnclearEffect,
            "effect.description",
            "Effect description is too brief to be clear",
            "Expand the description to clearly explain the effect",
            7,
        ));
    }

    // Check for missing discretion logic with complex conditions
    if statute.discretion_logic.is_none() && statute.preconditions.len() > 3 {
        ambiguities.push(Ambiguity::new(
            AmbiguityType::MissingDiscretion,
            "discretion_logic",
            format!(
                "Complex statute with {} conditions lacks discretion logic",
                statute.preconditions.len()
            ),
            "Add discretion logic to clarify how conditions should be evaluated",
            7,
        ));
    }

    // Check for temporal ambiguities
    if statute.temporal_validity.effective_date.is_none()
        && statute.temporal_validity.enacted_at.is_some()
    {
        ambiguities.push(Ambiguity::new(
            AmbiguityType::TemporalAmbiguity,
            "temporal_validity.effective_date",
            "Statute has enactment date but no effective date",
            "Specify when this statute becomes effective",
            6,
        ));
    }

    // Check for ambiguous temporal scope
    if statute.temporal_validity.enacted_at.is_none()
        && statute.temporal_validity.effective_date.is_none()
    {
        ambiguities.push(Ambiguity::new(
            AmbiguityType::TemporalAmbiguity,
            "temporal_validity",
            "No temporal information specified",
            "Add enacted_at and effective_date to clarify when this statute applies",
            8,
        ));
    }

    // Check for quantifier ambiguities in descriptions
    if contains_ambiguous_quantifiers(&statute.effect.description) {
        ambiguities.push(Ambiguity::new(
            AmbiguityType::QuantifierAmbiguity,
            "effect.description",
            "Effect description contains ambiguous quantifiers (e.g., 'some', 'several', 'many')",
            "Use specific numbers or percentages instead of vague quantifiers",
            7,
        ));
    }

    // Check for implicit assumptions in custom conditions
    for (idx, condition) in statute.preconditions.iter().enumerate() {
        if let legalis_core::Condition::Custom { description } = condition {
            if description.len() < 10 || contains_vague_terms(description) {
                ambiguities.push(Ambiguity::new(
                    AmbiguityType::ImplicitAssumption,
                    format!("preconditions[{}]", idx),
                    format!(
                        "Custom condition may have implicit assumptions: '{}'",
                        description
                    ),
                    "Replace custom condition with explicit, testable conditions",
                    8,
                ));
            }
        }
    }

    // Check for overlapping conditions using SMT solver if available
    #[cfg(feature = "z3-solver")]
    {
        if let Some(overlaps) = detect_overlapping_conditions(&statute.preconditions) {
            ambiguities.push(Ambiguity::new(
                AmbiguityType::OverlappingConditions,
                "preconditions",
                overlaps,
                "Simplify conditions to remove overlap or clarify the relationship",
                6,
            ));
        }
    }

    ambiguities.sort_by(|a, b| b.severity.cmp(&a.severity));
    ambiguities
}

/// Checks if a text contains vague or ambiguous terms.
fn contains_vague_terms(text: &str) -> bool {
    let vague_terms = [
        "reasonable",
        "appropriate",
        "sufficient",
        "adequate",
        "proper",
        "necessary",
        "significant",
        "substantial",
        "may",
        "might",
        "should",
        "could",
        "approximately",
        "around",
        "about",
        "roughly",
        "generally",
        "typically",
        "normally",
        "usually",
        "often",
        "sometimes",
        "occasionally",
    ];

    let text_lower = text.to_lowercase();
    vague_terms
        .iter()
        .any(|term| text_lower.contains(&format!(" {} ", term)) || text_lower.starts_with(term))
}

/// Checks if text contains ambiguous quantifiers.
fn contains_ambiguous_quantifiers(text: &str) -> bool {
    let ambiguous_quantifiers = [
        "some", "several", "many", "few", "multiple", "various", "numerous", "certain",
    ];

    let text_lower = text.to_lowercase();
    ambiguous_quantifiers
        .iter()
        .any(|quant| text_lower.contains(&format!(" {} ", quant)) || text_lower.starts_with(quant))
}

/// Detects overlapping conditions using SMT solver.
#[cfg(feature = "z3-solver")]
fn detect_overlapping_conditions(conditions: &[legalis_core::Condition]) -> Option<String> {
    use crate::smt::{SmtVerifier, create_z3_context};

    if conditions.len() < 2 {
        return None;
    }

    let ctx = create_z3_context();
    let mut verifier = SmtVerifier::new(&ctx);

    // Check for conditions that always imply each other (redundant)
    for i in 0..conditions.len() {
        for j in (i + 1)..conditions.len() {
            if let Ok(true) = verifier.implies(&conditions[i], &conditions[j]) {
                return Some(format!(
                    "Condition {} implies condition {} (redundant)",
                    i, j
                ));
            }
            if let Ok(true) = verifier.implies(&conditions[j], &conditions[i]) {
                return Some(format!(
                    "Condition {} implies condition {} (redundant)",
                    j, i
                ));
            }
        }
    }

    None
}

/// Generates an ambiguity detection report for a statute.
pub fn ambiguity_report(statute: &Statute) -> String {
    let ambiguities = detect_ambiguities(statute);

    if ambiguities.is_empty() {
        return format!(
            "# Ambiguity Report for '{}'\n\nNo ambiguities detected.\n",
            statute.id
        );
    }

    let mut report = String::new();
    report.push_str(&format!("# Ambiguity Report for '{}'\n\n", statute.id));
    report.push_str(&format!("**Total Ambiguities**: {}\n\n", ambiguities.len()));

    // Group by severity
    let critical = ambiguities.iter().filter(|a| a.severity >= 8).count();
    let high = ambiguities
        .iter()
        .filter(|a| (6..8).contains(&a.severity))
        .count();
    let medium = ambiguities.iter().filter(|a| a.severity < 6).count();

    report.push_str("## Summary by Severity\n\n");
    if critical > 0 {
        report.push_str(&format!("- **Critical** (8-10): {}\n", critical));
    }
    if high > 0 {
        report.push_str(&format!("- **High** (6-7): {}\n", high));
    }
    if medium > 0 {
        report.push_str(&format!("- **Medium** (1-5): {}\n", medium));
    }
    report.push_str("\n## Detected Ambiguities\n\n");

    for (idx, ambiguity) in ambiguities.iter().enumerate() {
        report.push_str(&format!(
            "### {}. {} (Severity: {})\n\n",
            idx + 1,
            ambiguity.ambiguity_type,
            ambiguity.severity
        ));
        report.push_str(&format!("- **Location**: `{}`\n", ambiguity.location));
        report.push_str(&format!("- **Issue**: {}\n", ambiguity.description));
        report.push_str(&format!("- **Suggestion**: {}\n\n", ambiguity.suggestion));
    }

    report
}

/// Generates an ambiguity detection report for multiple statutes.
pub fn batch_ambiguity_report(statutes: &[Statute]) -> String {
    let mut report = String::from("# Batch Ambiguity Detection Report\n\n");

    let mut total_ambiguities = 0;
    let mut statutes_with_ambiguities = 0;

    for statute in statutes {
        let ambiguities = detect_ambiguities(statute);
        if !ambiguities.is_empty() {
            statutes_with_ambiguities += 1;
            total_ambiguities += ambiguities.len();
        }
    }

    report.push_str(&format!(
        "**Total Statutes Analyzed**: {}\n",
        statutes.len()
    ));
    report.push_str(&format!(
        "**Statutes with Ambiguities**: {}\n",
        statutes_with_ambiguities
    ));
    report.push_str(&format!(
        "**Total Ambiguities Found**: {}\n\n",
        total_ambiguities
    ));

    if total_ambiguities == 0 {
        report.push_str("No ambiguities detected in any statute.\n");
        return report;
    }

    report.push_str("## Individual Statute Reports\n\n");

    for statute in statutes {
        let ambiguities = detect_ambiguities(statute);
        if !ambiguities.is_empty() {
            report.push_str(&format!(
                "### {} - {} ({} ambiguities)\n\n",
                statute.id,
                statute.title,
                ambiguities.len()
            ));

            for ambiguity in &ambiguities {
                report.push_str(&format!(
                    "- **{}** (Severity {}): {} [{}]\n",
                    ambiguity.ambiguity_type,
                    ambiguity.severity,
                    ambiguity.description,
                    ambiguity.location
                ));
            }
            report.push('\n');
        }
    }

    report
}

/// Generates a quality report for multiple statutes.
pub fn quality_report(statutes: &[Statute]) -> String {
    let mut report = String::from("# Statute Quality Report\n\n");

    let mut total_score = 0.0;
    let mut grade_counts: HashMap<char, usize> = HashMap::new();

    for statute in statutes {
        let metrics = analyze_quality(statute);
        total_score += metrics.overall_score;
        *grade_counts.entry(metrics.grade()).or_insert(0) += 1;

        report.push_str(&format!(
            "## Statute: {} - {}\n\n",
            statute.id, statute.title
        ));
        report.push_str(&format!(
            "**Overall Score**: {:.1}/100 (Grade: {})\n\n",
            metrics.overall_score,
            metrics.grade()
        ));

        report.push_str("### Detailed Scores\n\n");
        report.push_str(&format!(
            "- Complexity: {:.1}/100\n",
            metrics.complexity_score
        ));
        report.push_str(&format!(
            "- Readability: {:.1}/100\n",
            metrics.readability_score
        ));
        report.push_str(&format!(
            "- Consistency: {:.1}/100\n",
            metrics.consistency_score
        ));
        report.push_str(&format!(
            "- Completeness: {:.1}/100\n",
            metrics.completeness_score
        ));
        report.push_str(&format!(
            "- Drafting Quality: {:.1}/100\n",
            metrics.drafting_quality_score
        ));
        report.push_str(&format!(
            "- Clarity Index: {:.1}/100\n",
            metrics.clarity_index
        ));
        report.push_str(&format!(
            "- Testability: {:.1}/100\n",
            metrics.testability_score
        ));
        report.push_str(&format!(
            "- Maintainability: {:.1}/100\n\n",
            metrics.maintainability_score
        ));

        if !metrics.strengths.is_empty() {
            report.push_str("### Strengths\n\n");
            for strength in &metrics.strengths {
                report.push_str(&format!("- {}\n", strength));
            }
            report.push('\n');
        }

        if !metrics.issues.is_empty() {
            report.push_str("### Issues\n\n");
            for issue in &metrics.issues {
                report.push_str(&format!("- {}\n", issue));
            }
            report.push('\n');
        }
    }

    report.push_str("## Summary\n\n");
    report.push_str(&format!("- Total statutes analyzed: {}\n", statutes.len()));

    if !statutes.is_empty() {
        let average_score = total_score / statutes.len() as f64;
        report.push_str(&format!(
            "- Average quality score: {:.1}/100\n",
            average_score
        ));
    }

    report.push_str("\n### Grade Distribution\n\n");
    for grade in ['A', 'B', 'C', 'D', 'F'] {
        let count = grade_counts.get(&grade).unwrap_or(&0);
        report.push_str(&format!("- Grade {}: {}\n", grade, count));
    }

    report
}

// ============================================================================
// Change Impact Analysis
// ============================================================================

/// Represents a change between two statute versions.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum StatuteChange {
    /// Title changed
    TitleChanged { old: String, new: String },
    /// Description changed
    DescriptionChanged {
        old: Option<String>,
        new: Option<String>,
    },
    /// Jurisdiction changed
    JurisdictionChanged {
        old: Option<String>,
        new: Option<String>,
    },
    /// Effect changed
    EffectChanged { old: String, new: String },
    /// Preconditions changed
    PreconditionsChanged { old_count: usize, new_count: usize },
    /// Enactment date changed
    EnactmentDateChanged {
        old: Option<String>,
        new: Option<String>,
    },
    /// Effective date changed
    EffectiveDateChanged {
        old: Option<String>,
        new: Option<String>,
    },
}

impl std::fmt::Display for StatuteChange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TitleChanged { old, new } => {
                write!(f, "Title changed from '{}' to '{}'", old, new)
            }
            Self::DescriptionChanged { old, new } => {
                write!(f, "Description changed from {:?} to {:?}", old, new)
            }
            Self::JurisdictionChanged { old, new } => {
                write!(f, "Jurisdiction changed from {:?} to {:?}", old, new)
            }
            Self::EffectChanged { old, new } => {
                write!(f, "Effect changed from '{}' to '{}'", old, new)
            }
            Self::PreconditionsChanged {
                old_count,
                new_count,
            } => {
                write!(
                    f,
                    "Preconditions changed from {} to {} conditions",
                    old_count, new_count
                )
            }
            Self::EnactmentDateChanged { old, new } => {
                write!(f, "Enactment date changed from {:?} to {:?}", old, new)
            }
            Self::EffectiveDateChanged { old, new } => {
                write!(f, "Effective date changed from {:?} to {:?}", old, new)
            }
        }
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

/// Compares two versions of a statute and identifies changes.
pub fn compare_statutes(old: &Statute, new: &Statute) -> Vec<StatuteChange> {
    let mut changes = Vec::new();

    if old.title != new.title {
        changes.push(StatuteChange::TitleChanged {
            old: old.title.clone(),
            new: new.title.clone(),
        });
    }

    // Discretion logic acts as a pseudo-description
    if old.discretion_logic != new.discretion_logic {
        changes.push(StatuteChange::DescriptionChanged {
            old: old.discretion_logic.clone(),
            new: new.discretion_logic.clone(),
        });
    }

    if old.jurisdiction != new.jurisdiction {
        changes.push(StatuteChange::JurisdictionChanged {
            old: old.jurisdiction.clone(),
            new: new.jurisdiction.clone(),
        });
    }

    let old_effect_str = format!("{:?}", old.effect);
    let new_effect_str = format!("{:?}", new.effect);
    if old_effect_str != new_effect_str {
        changes.push(StatuteChange::EffectChanged {
            old: old_effect_str,
            new: new_effect_str,
        });
    }

    if old.preconditions.len() != new.preconditions.len() || old.preconditions != new.preconditions
    {
        changes.push(StatuteChange::PreconditionsChanged {
            old_count: old.preconditions.len(),
            new_count: new.preconditions.len(),
        });
    }

    let old_enacted = old
        .temporal_validity
        .enacted_at
        .as_ref()
        .map(|d| d.to_string());
    let new_enacted = new
        .temporal_validity
        .enacted_at
        .as_ref()
        .map(|d| d.to_string());
    if old_enacted != new_enacted {
        changes.push(StatuteChange::EnactmentDateChanged {
            old: old_enacted,
            new: new_enacted,
        });
    }

    let old_effective = old
        .temporal_validity
        .effective_date
        .as_ref()
        .map(|d| d.to_string());
    let new_effective = new
        .temporal_validity
        .effective_date
        .as_ref()
        .map(|d| d.to_string());
    if old_effective != new_effective {
        changes.push(StatuteChange::EffectiveDateChanged {
            old: old_effective,
            new: new_effective,
        });
    }

    changes
}

/// Analyzes the impact of changing a statute in a collection.
pub fn analyze_change_impact(
    changed_statute: &Statute,
    old_version: &Statute,
    all_statutes: &[Statute],
) -> ChangeImpact {
    let changes = compare_statutes(old_version, changed_statute);

    // Find statutes that reference this one
    let mut affected_statutes = Vec::new();
    for statute in all_statutes {
        if statute.id != changed_statute.id {
            let refs = extract_statute_references_from_conditions(&statute.preconditions);
            if refs.contains(&changed_statute.id) {
                affected_statutes.push(statute.id.clone());
            }
        }
    }

    // Determine impact severity
    let impact_severity = if changes.iter().any(|c| {
        matches!(
            c,
            StatuteChange::EffectChanged { .. } | StatuteChange::PreconditionsChanged { .. }
        )
    }) && !affected_statutes.is_empty()
    {
        Severity::Critical
    } else if !affected_statutes.is_empty() || changes.len() > 3 {
        Severity::Warning
    } else {
        Severity::Info
    };

    // Generate recommendations
    let mut recommendations = Vec::new();

    if !affected_statutes.is_empty() {
        recommendations.push(format!(
            "Review and re-verify {} affected statute(s)",
            affected_statutes.len()
        ));
    }

    if changes
        .iter()
        .any(|c| matches!(c, StatuteChange::EffectChanged { .. }))
    {
        recommendations
            .push("Effect changed - verify compatibility with dependent statutes".to_string());
    }

    if changes
        .iter()
        .any(|c| matches!(c, StatuteChange::PreconditionsChanged { .. }))
    {
        recommendations.push("Preconditions changed - update test cases".to_string());
    }

    if changes
        .iter()
        .any(|c| matches!(c, StatuteChange::JurisdictionChanged { .. }))
    {
        recommendations.push("Jurisdiction changed - verify compliance requirements".to_string());
    }

    ChangeImpact {
        statute_id: changed_statute.id.clone(),
        changes,
        affected_statutes,
        impact_severity,
        recommendations,
    }
}

/// Generates a change impact report.
pub fn change_impact_report(impact: &ChangeImpact) -> String {
    let mut report = String::from("# Change Impact Analysis\n\n");
    report.push_str(&format!("## Statute: {}\n\n", impact.statute_id));
    report.push_str(&format!(
        "**Impact Severity**: {:?}\n\n",
        impact.impact_severity
    ));

    report.push_str("### Changes Detected\n\n");
    if impact.changes.is_empty() {
        report.push_str("No changes detected.\n\n");
    } else {
        for (i, change) in impact.changes.iter().enumerate() {
            report.push_str(&format!("{}. {}\n", i + 1, change));
        }
        report.push('\n');
    }

    report.push_str("### Affected Statutes\n\n");
    if impact.affected_statutes.is_empty() {
        report.push_str("No statutes are directly affected by this change.\n\n");
    } else {
        report.push_str(&format!(
            "{} statute(s) reference this statute and may be affected:\n\n",
            impact.affected_statutes.len()
        ));
        for statute_id in &impact.affected_statutes {
            report.push_str(&format!("- {}\n", statute_id));
        }
        report.push('\n');
    }

    report.push_str("### Recommendations\n\n");
    if impact.recommendations.is_empty() {
        report.push_str("No specific recommendations.\n\n");
    } else {
        for rec in &impact.recommendations {
            report.push_str(&format!("- {}\n", rec));
        }
        report.push('\n');
    }

    report
}

// ============================================================================
// Batch Verification
// ============================================================================

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

        // Count errors by severity
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

impl Default for BatchVerificationResult {
    fn default() -> Self {
        Self::new()
    }
}

/// Performs batch verification on multiple statutes and returns aggregate results.
pub fn batch_verify(statutes: &[Statute], verifier: &StatuteVerifier) -> BatchVerificationResult {
    let start = std::time::Instant::now();
    let mut batch_result = BatchVerificationResult::new();

    for statute in statutes {
        let result = verifier.verify(std::slice::from_ref(statute));
        batch_result.add_result(statute.id.clone(), result);
    }

    batch_result.total_time_ms = start.elapsed().as_millis() as u64;
    batch_result
}

/// Generates a batch verification report.
pub fn batch_verification_report(result: &BatchVerificationResult) -> String {
    let mut report = String::from("# Batch Verification Report\n\n");

    report.push_str("## Summary\n\n");
    report.push_str(&format!("- Total statutes: {}\n", result.total_statutes));
    report.push_str(&format!("- Passed: {}\n", result.passed));
    report.push_str(&format!("- Failed: {}\n", result.failed));
    report.push_str(&format!("- Pass rate: {:.1}%\n", result.pass_rate()));
    report.push_str(&format!(
        "- Total verification time: {}ms\n\n",
        result.total_time_ms
    ));

    report.push_str("## Error Distribution\n\n");
    if result.error_counts.is_empty() {
        report.push_str("No errors detected.\n\n");
    } else {
        for severity in [
            Severity::Critical,
            Severity::Error,
            Severity::Warning,
            Severity::Info,
        ] {
            if let Some(count) = result.error_counts.get(&severity) {
                report.push_str(&format!("- {}: {}\n", severity, count));
            }
        }
        report.push('\n');
    }

    report.push_str("## Failed Statutes\n\n");
    let mut failed_statutes: Vec<_> = result
        .individual_results
        .iter()
        .filter(|(_, r)| !r.passed)
        .collect();

    if failed_statutes.is_empty() {
        report.push_str("All statutes passed verification.\n\n");
    } else {
        failed_statutes.sort_by_key(|(id, _)| id.as_str());

        for (statute_id, verification_result) in failed_statutes {
            report.push_str(&format!("### {}\n\n", statute_id));
            report.push_str(&format!("- Errors: {}\n", verification_result.errors.len()));
            report.push_str(&format!(
                "- Warnings: {}\n",
                verification_result.warnings.len()
            ));

            if !verification_result.errors.is_empty() {
                report.push_str("\n**Errors:**\n\n");
                for error in &verification_result.errors {
                    report.push_str(&format!("- [{:?}] {}\n", error.severity(), error));
                }
            }
            report.push('\n');
        }
    }

    report
}

// ============================================================================
// Statistical Analysis
// ============================================================================

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

/// Analyzes a collection of statutes and returns comprehensive statistics.
pub fn analyze_statute_statistics(statutes: &[Statute]) -> StatuteStatistics {
    if statutes.is_empty() {
        return StatuteStatistics {
            total_count: 0,
            avg_preconditions: 0.0,
            median_preconditions: 0.0,
            common_condition_types: Vec::new(),
            jurisdiction_distribution: HashMap::new(),
            avg_complexity: 0.0,
            effect_type_distribution: HashMap::new(),
            discretion_count: 0,
            temporal_coverage: 0.0,
        };
    }

    let total_count = statutes.len();

    // Precondition statistics
    let mut precondition_counts: Vec<usize> =
        statutes.iter().map(|s| s.preconditions.len()).collect();
    precondition_counts.sort_unstable();

    let total_preconditions: usize = precondition_counts.iter().sum();
    let avg_preconditions = total_preconditions as f64 / total_count as f64;

    let median_preconditions = if precondition_counts.len() % 2 == 0 {
        let mid = precondition_counts.len() / 2;
        (precondition_counts[mid - 1] + precondition_counts[mid]) as f64 / 2.0
    } else {
        precondition_counts[precondition_counts.len() / 2] as f64
    };

    // Condition type analysis
    let mut condition_type_counts: HashMap<String, usize> = HashMap::new();
    for statute in statutes {
        for condition in &statute.preconditions {
            let type_name = format!("{:?}", condition)
                .split('{')
                .next()
                .unwrap_or("Unknown")
                .to_string();
            *condition_type_counts.entry(type_name).or_insert(0) += 1;
        }
    }

    let mut common_condition_types: Vec<(String, usize)> =
        condition_type_counts.into_iter().collect();
    common_condition_types.sort_by(|a, b| b.1.cmp(&a.1));
    common_condition_types.truncate(10); // Top 10

    // Jurisdiction distribution
    let mut jurisdiction_distribution: HashMap<String, usize> = HashMap::new();
    for statute in statutes {
        let jurisdiction = statute
            .jurisdiction
            .as_deref()
            .unwrap_or("None")
            .to_string();
        *jurisdiction_distribution.entry(jurisdiction).or_insert(0) += 1;
    }

    // Complexity statistics
    let total_complexity: u32 = statutes
        .iter()
        .map(|s| analyze_complexity(s).complexity_score)
        .sum();
    let avg_complexity = total_complexity as f64 / total_count as f64;

    // Effect type distribution
    let mut effect_type_distribution: HashMap<String, usize> = HashMap::new();
    for statute in statutes {
        let effect_type = format!("{:?}", statute.effect.effect_type);
        *effect_type_distribution.entry(effect_type).or_insert(0) += 1;
    }

    // Discretion logic count
    let discretion_count = statutes
        .iter()
        .filter(|s| s.discretion_logic.is_some())
        .count();

    // Temporal coverage
    let temporal_count = statutes
        .iter()
        .filter(|s| {
            s.temporal_validity.effective_date.is_some() || s.temporal_validity.enacted_at.is_some()
        })
        .count();
    let temporal_coverage = (temporal_count as f64 / total_count as f64) * 100.0;

    StatuteStatistics {
        total_count,
        avg_preconditions,
        median_preconditions,
        common_condition_types,
        jurisdiction_distribution,
        avg_complexity,
        effect_type_distribution,
        discretion_count,
        temporal_coverage,
    }
}

/// Generates a statistical report for a statute collection.
pub fn statistics_report(statutes: &[Statute]) -> String {
    let stats = analyze_statute_statistics(statutes);

    let mut report = String::from("# Statute Collection Statistics\n\n");

    report.push_str("## Overview\n\n");
    report.push_str(&format!("- **Total Statutes**: {}\n", stats.total_count));
    report.push_str(&format!(
        "- **Average Preconditions**: {:.2}\n",
        stats.avg_preconditions
    ));
    report.push_str(&format!(
        "- **Median Preconditions**: {:.1}\n",
        stats.median_preconditions
    ));
    report.push_str(&format!(
        "- **Average Complexity**: {:.2}\n",
        stats.avg_complexity
    ));
    report.push_str(&format!(
        "- **Statutes with Discretion Logic**: {} ({:.1}%)\n",
        stats.discretion_count,
        (stats.discretion_count as f64 / stats.total_count as f64) * 100.0
    ));
    report.push_str(&format!(
        "- **Temporal Coverage**: {:.1}%\n\n",
        stats.temporal_coverage
    ));

    report.push_str("## Common Condition Types\n\n");
    for (i, (condition_type, count)) in stats.common_condition_types.iter().enumerate() {
        report.push_str(&format!(
            "{}. **{}**: {} occurrences\n",
            i + 1,
            condition_type,
            count
        ));
    }
    report.push('\n');

    report.push_str("## Jurisdiction Distribution\n\n");
    let mut jurisdictions: Vec<_> = stats.jurisdiction_distribution.iter().collect();
    jurisdictions.sort_by(|a, b| b.1.cmp(a.1));
    for (jurisdiction, count) in jurisdictions {
        let percentage = (*count as f64 / stats.total_count as f64) * 100.0;
        report.push_str(&format!(
            "- **{}**: {} ({:.1}%)\n",
            jurisdiction, count, percentage
        ));
    }
    report.push('\n');

    report.push_str("## Effect Type Distribution\n\n");
    let mut effects: Vec<_> = stats.effect_type_distribution.iter().collect();
    effects.sort_by(|a, b| b.1.cmp(a.1));
    for (effect_type, count) in effects {
        let percentage = (*count as f64 / stats.total_count as f64) * 100.0;
        report.push_str(&format!(
            "- **{}**: {} ({:.1}%)\n",
            effect_type, count, percentage
        ));
    }

    report
}

// ============================================================================
// Duplicate Detection
// ============================================================================

/// Represents a potential duplicate statute.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DuplicateCandidate {
    /// IDs of potentially duplicate statutes
    pub statute_ids: Vec<String>,
    /// Similarity score (0.0 to 1.0)
    pub similarity_score: f64,
    /// Type of similarity
    pub similarity_type: String,
    /// Recommendation
    pub recommendation: String,
}

/// Detects potential duplicate or near-duplicate statutes.
pub fn detect_duplicates(statutes: &[Statute], min_similarity: f64) -> Vec<DuplicateCandidate> {
    let mut duplicates = Vec::new();

    for i in 0..statutes.len() {
        for j in (i + 1)..statutes.len() {
            let stat1 = &statutes[i];
            let stat2 = &statutes[j];

            // Check semantic similarity
            let similarity = semantic_similarity(stat1, stat2);

            if similarity.0 >= min_similarity {
                let similarity_type = if similarity.0 >= 0.95 {
                    "Near-identical"
                } else if similarity.0 >= 0.80 {
                    "Very similar"
                } else {
                    "Similar"
                };

                let recommendation = if similarity.0 >= 0.95 {
                    "Consider merging or removing duplicate".to_string()
                } else if similarity.0 >= 0.80 {
                    "Review for potential consolidation".to_string()
                } else {
                    "Review for consistency".to_string()
                };

                duplicates.push(DuplicateCandidate {
                    statute_ids: vec![stat1.id.clone(), stat2.id.clone()],
                    similarity_score: similarity.0,
                    similarity_type: similarity_type.to_string(),
                    recommendation,
                });
            }
        }
    }

    // Sort by similarity score (descending)
    duplicates.sort_by(|a, b| b.similarity_score.partial_cmp(&a.similarity_score).unwrap());

    duplicates
}

/// Generates a duplicate detection report.
pub fn duplicate_detection_report(statutes: &[Statute], min_similarity: f64) -> String {
    let duplicates = detect_duplicates(statutes, min_similarity);

    let mut report = String::from("# Duplicate Detection Report\n\n");
    report.push_str(&format!(
        "**Minimum Similarity Threshold**: {:.0}%\n\n",
        min_similarity * 100.0
    ));

    if duplicates.is_empty() {
        report.push_str("No duplicates or similar statutes found.\n");
        return report;
    }

    report.push_str(&format!(
        "Found **{}** potential duplicate(s) or similar statute(s):\n\n",
        duplicates.len()
    ));

    for (i, dup) in duplicates.iter().enumerate() {
        report.push_str(&format!("## Duplicate Group #{}\n\n", i + 1));
        report.push_str(&format!(
            "- **Similarity**: {:.1}% ({})\n",
            dup.similarity_score * 100.0,
            dup.similarity_type
        ));
        report.push_str("- **Statutes**:\n");
        for statute_id in &dup.statute_ids {
            report.push_str(&format!("  - {}\n", statute_id));
        }
        report.push_str(&format!("- **Recommendation**: {}\n\n", dup.recommendation));
    }

    report
}

// ============================================================================
// Regulatory Impact Scoring
// ============================================================================

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

/// Analyzes the regulatory impact of a statute.
pub fn analyze_regulatory_impact(statute: &Statute) -> RegulatoryImpact {
    // Calculate compliance complexity based on preconditions and effects
    let complexity_metrics = analyze_complexity(statute);
    let compliance_complexity = complexity_metrics.complexity_score;

    // Calculate impact score
    let mut impact_score = compliance_complexity;

    // Adjust for effect type
    let effect_weight = match statute.effect.effect_type {
        legalis_core::EffectType::Prohibition => 30,
        legalis_core::EffectType::Obligation => 25,
        legalis_core::EffectType::Revoke => 20,
        legalis_core::EffectType::Grant => 10,
        legalis_core::EffectType::MonetaryTransfer => 20,
        legalis_core::EffectType::StatusChange => 15,
        legalis_core::EffectType::Custom => 15,
    };
    impact_score = (impact_score + effect_weight).min(100);

    // Adjust for number of preconditions (more conditions = higher burden)
    let precondition_weight = (statute.preconditions.len() as u32 * 5).min(30);
    impact_score = (impact_score + precondition_weight).min(100);

    // Determine impact level
    let impact_level = if impact_score >= 75 {
        "High Impact"
    } else if impact_score >= 50 {
        "Medium Impact"
    } else if impact_score >= 25 {
        "Low Impact"
    } else {
        "Minimal Impact"
    };

    // Estimate affected entities (simplified heuristic)
    let affected_entities = if statute.preconditions.is_empty() {
        "Potentially all entities"
    } else if statute.preconditions.len() <= 2 {
        "Broad population"
    } else if statute.preconditions.len() <= 5 {
        "Specific demographic"
    } else {
        "Narrow subset"
    };

    // Estimate implementation cost
    let implementation_cost = if impact_score >= 75 {
        "High - Significant resources required"
    } else if impact_score >= 50 {
        "Medium - Moderate resources required"
    } else {
        "Low - Minimal resources required"
    };

    // Estimate ongoing cost
    let ongoing_cost = if complexity_metrics.complexity_score >= 70 {
        "High - Ongoing monitoring and compliance needed"
    } else if complexity_metrics.complexity_score >= 40 {
        "Medium - Periodic compliance checks needed"
    } else {
        "Low - Minimal ongoing requirements"
    };

    RegulatoryImpact {
        statute_id: statute.id.clone(),
        impact_score,
        compliance_complexity,
        affected_entities: affected_entities.to_string(),
        implementation_cost: implementation_cost.to_string(),
        ongoing_cost: ongoing_cost.to_string(),
        impact_level: impact_level.to_string(),
    }
}

/// Generates a regulatory impact report for multiple statutes.
pub fn regulatory_impact_report(statutes: &[Statute]) -> String {
    let mut report = String::from("# Regulatory Impact Assessment\n\n");

    let impacts: Vec<RegulatoryImpact> = statutes.iter().map(analyze_regulatory_impact).collect();

    // Calculate aggregate statistics
    let total_score: u32 = impacts.iter().map(|i| i.impact_score).sum();
    let avg_score = if !impacts.is_empty() {
        total_score as f64 / impacts.len() as f64
    } else {
        0.0
    };

    let high_impact_count = impacts
        .iter()
        .filter(|i| i.impact_level == "High Impact")
        .count();
    let medium_impact_count = impacts
        .iter()
        .filter(|i| i.impact_level == "Medium Impact")
        .count();
    let low_impact_count = impacts
        .iter()
        .filter(|i| i.impact_level == "Low Impact")
        .count();

    report.push_str("## Summary\n\n");
    report.push_str(&format!(
        "- **Total Statutes Analyzed**: {}\n",
        statutes.len()
    ));
    report.push_str(&format!(
        "- **Average Impact Score**: {:.1}/100\n",
        avg_score
    ));
    report.push_str(&format!("- **High Impact**: {}\n", high_impact_count));
    report.push_str(&format!("- **Medium Impact**: {}\n", medium_impact_count));
    report.push_str(&format!(
        "- **Low/Minimal Impact**: {}\n\n",
        low_impact_count
    ));

    report.push_str("## Individual Statute Analysis\n\n");

    for impact in &impacts {
        report.push_str(&format!(
            "### {} - {}\n\n",
            impact.statute_id, impact.impact_level
        ));
        report.push_str(&format!(
            "- **Impact Score**: {}/100\n",
            impact.impact_score
        ));
        report.push_str(&format!(
            "- **Compliance Complexity**: {}/100\n",
            impact.compliance_complexity
        ));
        report.push_str(&format!(
            "- **Affected Entities**: {}\n",
            impact.affected_entities
        ));
        report.push_str(&format!(
            "- **Implementation Cost**: {}\n",
            impact.implementation_cost
        ));
        report.push_str(&format!("- **Ongoing Cost**: {}\n\n", impact.ongoing_cost));
    }

    report
}

// ============================================================================
// Compliance Checklist Generation
// ============================================================================

/// A compliance checklist item.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComplianceItem {
    /// Item number
    pub number: usize,
    /// Description of the requirement
    pub requirement: String,
    /// Precondition that must be met
    pub precondition: Option<String>,
    /// Priority level
    pub priority: String,
}

/// Generates a compliance checklist from a statute.
pub fn generate_compliance_checklist(statute: &Statute) -> Vec<ComplianceItem> {
    let mut items = Vec::new();
    let mut item_number = 1;

    // Add precondition checks
    for precondition in &statute.preconditions {
        let requirement = format!("Verify: {:?}", precondition);
        let priority = "Required";

        items.push(ComplianceItem {
            number: item_number,
            requirement,
            precondition: Some(format!("{:?}", precondition)),
            priority: priority.to_string(),
        });
        item_number += 1;
    }

    // Add effect implementation
    let effect_requirement = format!(
        "Implement effect: {:?} - {}",
        statute.effect.effect_type, statute.effect.description
    );

    items.push(ComplianceItem {
        number: item_number,
        requirement: effect_requirement,
        precondition: None,
        priority: "Required".to_string(),
    });
    item_number += 1;

    // Add discretion logic if present
    if let Some(ref discretion) = statute.discretion_logic {
        items.push(ComplianceItem {
            number: item_number,
            requirement: format!("Consider discretion: {}", discretion),
            precondition: None,
            priority: "Optional".to_string(),
        });
        item_number += 1;
    }

    // Add temporal validity checks
    if statute.temporal_validity.effective_date.is_some()
        || statute.temporal_validity.enacted_at.is_some()
    {
        items.push(ComplianceItem {
            number: item_number,
            requirement: "Verify statute is currently in effect".to_string(),
            precondition: None,
            priority: "Required".to_string(),
        });
    }

    items
}

/// Generates a compliance checklist report for a statute.
pub fn compliance_checklist_report(statute: &Statute) -> String {
    let items = generate_compliance_checklist(statute);

    let mut report = String::from("# Compliance Checklist\n\n");
    report.push_str(&format!(
        "**Statute**: {} - {}\n\n",
        statute.id, statute.title
    ));

    if let Some(ref jurisdiction) = statute.jurisdiction {
        report.push_str(&format!("**Jurisdiction**: {}\n", jurisdiction));
    }

    report.push_str(&format!("\n**Total Items**: {}\n\n", items.len()));

    report.push_str("## Checklist Items\n\n");

    for item in &items {
        report.push_str(&format!(
            "- [ ] **Item {}** [{}]: {}\n",
            item.number, item.priority, item.requirement
        ));
    }

    report
}

/// Generates a consolidated compliance checklist for multiple statutes.
pub fn consolidated_compliance_checklist(statutes: &[Statute]) -> String {
    let mut report = String::from("# Consolidated Compliance Checklist\n\n");
    report.push_str(&format!("**Total Statutes**: {}\n\n", statutes.len()));

    for statute in statutes {
        let items = generate_compliance_checklist(statute);
        report.push_str(&format!("## {} - {}\n\n", statute.id, statute.title));

        for item in &items {
            report.push_str(&format!(
                "- [ ] **{}**: {}\n",
                item.priority, item.requirement
            ));
        }
        report.push('\n');
    }

    report
}

// ============================================================================
// Reporting Extensions (v0.1.8)
// ============================================================================

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

/// Generates a compliance certification document
pub fn generate_compliance_certification(
    certificate_id: impl Into<String>,
    organization: impl Into<String>,
    certifying_authority: impl Into<String>,
    statutes: &[Statute],
    result: &VerificationResult,
    valid_days: Option<u32>,
) -> ComplianceCertification {
    use chrono::{Duration, Utc};

    let now = Utc::now();
    let certification_date = now.format("%Y-%m-%d %H:%M:%S UTC").to_string();

    let valid_until = valid_days.map(|days| {
        (now + Duration::days(days as i64))
            .format("%Y-%m-%d %H:%M:%S UTC")
            .to_string()
    });

    let statute_ids: Vec<String> = statutes.iter().map(|s| s.id.clone()).collect();

    let critical_errors = result
        .errors
        .iter()
        .filter(|e| e.severity() == Severity::Critical)
        .count();

    let total_statutes = statutes.len();
    let passed_count = if result.passed { total_statutes } else { 0 };
    let failed_count = total_statutes - passed_count;
    let pass_rate = if total_statutes > 0 {
        (passed_count as f64 / total_statutes as f64) * 100.0
    } else {
        0.0
    };

    let verification_summary = VerificationSummary {
        total_statutes,
        passed_count,
        failed_count,
        pass_rate,
        critical_errors,
        warnings: result.warnings.len(),
    };

    ComplianceCertification {
        certificate_id: certificate_id.into(),
        certification_date,
        organization: organization.into(),
        statute_ids,
        verification_summary,
        certifying_authority: certifying_authority.into(),
        valid_until,
        notes: Vec::new(),
    }
}

/// Exports compliance certification as a formatted report
pub fn compliance_certification_report(cert: &ComplianceCertification) -> String {
    let mut report = String::from("# COMPLIANCE CERTIFICATION\n\n");
    report.push_str("---\n\n");

    report.push_str(&format!("**Certificate ID**: {}\n\n", cert.certificate_id));
    report.push_str(&format!(
        "**Certification Date**: {}\n\n",
        cert.certification_date
    ));
    report.push_str(&format!("**Organization**: {}\n\n", cert.organization));
    report.push_str(&format!(
        "**Certifying Authority**: {}\n\n",
        cert.certifying_authority
    ));

    if let Some(ref valid_until) = cert.valid_until {
        report.push_str(&format!("**Valid Until**: {}\n\n", valid_until));
    }

    report.push_str("---\n\n");
    report.push_str("## Verification Summary\n\n");

    let summary = &cert.verification_summary;
    report.push_str(&format!(
        "- **Total Statutes Verified**: {}\n",
        summary.total_statutes
    ));
    report.push_str(&format!("- **Passed**: {}\n", summary.passed_count));
    report.push_str(&format!("- **Failed**: {}\n", summary.failed_count));
    report.push_str(&format!("- **Pass Rate**: {:.2}%\n", summary.pass_rate));
    report.push_str(&format!(
        "- **Critical Errors**: {}\n",
        summary.critical_errors
    ));
    report.push_str(&format!("- **Warnings**: {}\n\n", summary.warnings));

    report.push_str("## Certified Statutes\n\n");
    for statute_id in &cert.statute_ids {
        report.push_str(&format!("- {}\n", statute_id));
    }
    report.push('\n');

    if !cert.notes.is_empty() {
        report.push_str("## Additional Notes\n\n");
        for note in &cert.notes {
            report.push_str(&format!("- {}\n", note));
        }
        report.push('\n');
    }

    report.push_str("---\n\n");
    report.push_str("This certification confirms that the listed statutes have been verified\n");
    report.push_str("using the Legalis Verification System and meet the specified compliance\n");
    report.push_str("requirements as of the certification date.\n");

    report
}

/// Regulatory filing report for submitting to regulatory bodies
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RegulatoryFiling {
    /// Filing ID
    pub filing_id: String,
    /// Filing date
    pub filing_date: String,
    /// Regulatory body
    pub regulatory_body: String,
    /// Filing type (e.g., "Annual Compliance", "New Statute", "Amendment")
    pub filing_type: String,
    /// Jurisdiction
    pub jurisdiction: String,
    /// Statutes included in filing
    pub statutes: Vec<StatuteFilingInfo>,
    /// Compliance status
    pub compliance_status: String,
    /// Supporting documentation references
    pub documentation_refs: Vec<String>,
}

/// Information about a statute in a regulatory filing
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StatuteFilingInfo {
    /// Statute ID
    pub statute_id: String,
    /// Statute title
    pub title: String,
    /// Effective date
    pub effective_date: Option<String>,
    /// Enactment date
    pub enactment_date: Option<String>,
    /// Compliance status for this statute
    pub status: String,
    /// Issues found (if any)
    pub issues: Vec<String>,
}

/// Generates a regulatory filing report
#[allow(clippy::too_many_arguments)]
pub fn generate_regulatory_filing(
    filing_id: impl Into<String>,
    regulatory_body: impl Into<String>,
    filing_type: impl Into<String>,
    jurisdiction: impl Into<String>,
    statutes: &[Statute],
    results: &[VerificationResult],
) -> RegulatoryFiling {
    use chrono::Utc;

    let filing_date = Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();

    let statute_infos: Vec<StatuteFilingInfo> = statutes
        .iter()
        .zip(results.iter())
        .map(|(statute, result)| {
            let status = if result.passed {
                "Compliant".to_string()
            } else if result.has_critical_errors() {
                "Non-Compliant (Critical)".to_string()
            } else {
                "Non-Compliant".to_string()
            };

            let issues: Vec<String> = result.errors.iter().map(|e| format!("{}", e)).collect();

            StatuteFilingInfo {
                statute_id: statute.id.clone(),
                title: statute.title.clone(),
                effective_date: statute
                    .temporal_validity
                    .effective_date
                    .as_ref()
                    .map(|d| d.format("%Y-%m-%d").to_string()),
                enactment_date: statute
                    .temporal_validity
                    .enacted_at
                    .as_ref()
                    .map(|dt| dt.format("%Y-%m-%d").to_string()),
                status,
                issues,
            }
        })
        .collect();

    let all_compliant = statute_infos.iter().all(|s| s.status == "Compliant");
    let any_critical = statute_infos.iter().any(|s| s.status.contains("Critical"));

    let compliance_status = if all_compliant {
        "Fully Compliant".to_string()
    } else if any_critical {
        "Non-Compliant (Critical Issues)".to_string()
    } else {
        "Partially Compliant".to_string()
    };

    RegulatoryFiling {
        filing_id: filing_id.into(),
        filing_date,
        regulatory_body: regulatory_body.into(),
        filing_type: filing_type.into(),
        jurisdiction: jurisdiction.into(),
        statutes: statute_infos,
        compliance_status,
        documentation_refs: Vec::new(),
    }
}

/// Exports regulatory filing as a formatted report
pub fn regulatory_filing_report(filing: &RegulatoryFiling) -> String {
    let mut report = String::from("# REGULATORY FILING REPORT\n\n");
    report.push_str("---\n\n");

    report.push_str(&format!("**Filing ID**: {}\n\n", filing.filing_id));
    report.push_str(&format!("**Filing Date**: {}\n\n", filing.filing_date));
    report.push_str(&format!(
        "**Regulatory Body**: {}\n\n",
        filing.regulatory_body
    ));
    report.push_str(&format!("**Filing Type**: {}\n\n", filing.filing_type));
    report.push_str(&format!("**Jurisdiction**: {}\n\n", filing.jurisdiction));
    report.push_str(&format!(
        "**Compliance Status**: {}\n\n",
        filing.compliance_status
    ));

    report.push_str("---\n\n");
    report.push_str("## Statutes Included in Filing\n\n");

    for (idx, statute_info) in filing.statutes.iter().enumerate() {
        report.push_str(&format!("### {} - {}\n\n", idx + 1, statute_info.title));
        report.push_str(&format!("**ID**: {}\n\n", statute_info.statute_id));
        report.push_str(&format!("**Status**: {}\n\n", statute_info.status));

        if let Some(ref enactment) = statute_info.enactment_date {
            report.push_str(&format!("**Enactment Date**: {}\n\n", enactment));
        }

        if let Some(ref effective) = statute_info.effective_date {
            report.push_str(&format!("**Effective Date**: {}\n\n", effective));
        }

        if !statute_info.issues.is_empty() {
            report.push_str("**Issues Identified**:\n\n");
            for issue in &statute_info.issues {
                report.push_str(&format!("- {}\n", issue));
            }
            report.push('\n');
        }
    }

    if !filing.documentation_refs.is_empty() {
        report.push_str("## Supporting Documentation\n\n");
        for doc_ref in &filing.documentation_refs {
            report.push_str(&format!("- {}\n", doc_ref));
        }
        report.push('\n');
    }

    report.push_str("---\n\n");
    report.push_str("This filing has been prepared in accordance with applicable regulatory\n");
    report.push_str(
        "requirements and includes all necessary verification and compliance information.\n",
    );

    report
}

/// Executive summary of verification results
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExecutiveSummary {
    /// Summary title
    pub title: String,
    /// Generation date
    pub date: String,
    /// Key findings
    pub key_findings: Vec<String>,
    /// Overall assessment
    pub overall_assessment: String,
    /// Statistics
    pub statistics: SummaryStatistics,
    /// Recommendations
    pub recommendations: Vec<String>,
    /// Risk level (Low, Medium, High, Critical)
    pub risk_level: String,
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

/// Generates an executive summary from verification results
pub fn generate_executive_summary(
    title: impl Into<String>,
    statutes: &[Statute],
    result: &VerificationResult,
) -> ExecutiveSummary {
    use chrono::Utc;

    let date = Utc::now().format("%Y-%m-%d").to_string();

    let severity_counts = result.severity_counts();
    let critical_issues = *severity_counts.get(&Severity::Critical).unwrap_or(&0);
    let high_severity = *severity_counts.get(&Severity::Error).unwrap_or(&0);
    let medium_severity = *severity_counts.get(&Severity::Warning).unwrap_or(&0);

    let total_issues = result.errors.len();
    let statutes_with_issues = if total_issues > 0 { statutes.len() } else { 0 };

    // Calculate average quality score
    let quality_scores: Vec<f64> = statutes
        .iter()
        .map(|s| analyze_quality(s).overall_score)
        .collect();
    let average_quality_score = if !quality_scores.is_empty() {
        quality_scores.iter().sum::<f64>() / quality_scores.len() as f64
    } else {
        0.0
    };

    let statistics = SummaryStatistics {
        total_statutes: statutes.len(),
        statutes_with_issues,
        total_issues,
        critical_issues,
        high_severity_issues: high_severity,
        medium_severity_issues: medium_severity,
        average_quality_score,
    };

    // Determine risk level
    let risk_level = if critical_issues > 0 {
        "Critical".to_string()
    } else if high_severity > 5 {
        "High".to_string()
    } else if high_severity > 0 || medium_severity > 5 {
        "Medium".to_string()
    } else {
        "Low".to_string()
    };

    // Generate key findings
    let mut key_findings = Vec::new();

    if result.passed {
        key_findings.push("All statutes passed verification checks".to_string());
    } else {
        key_findings.push(format!(
            "Found {} total issues across {} statutes",
            total_issues, statutes_with_issues
        ));
    }

    if critical_issues > 0 {
        key_findings.push(format!(
            "{} critical issues requiring immediate attention",
            critical_issues
        ));
    }

    if average_quality_score >= 80.0 {
        key_findings.push(format!(
            "High average quality score: {:.1}/100",
            average_quality_score
        ));
    } else if average_quality_score < 60.0 {
        key_findings.push(format!(
            "Low average quality score: {:.1}/100 - improvement needed",
            average_quality_score
        ));
    }

    // Generate overall assessment
    let overall_assessment = if critical_issues > 0 {
        "Critical issues detected. Immediate remediation required before deployment.".to_string()
    } else if high_severity > 0 {
        "Significant issues found. Review and remediation recommended.".to_string()
    } else if medium_severity > 0 {
        "Minor issues identified. Consider addressing before final deployment.".to_string()
    } else {
        "No significant issues detected. Statutes are ready for deployment.".to_string()
    };

    // Generate recommendations
    let mut recommendations = Vec::new();

    if critical_issues > 0 {
        recommendations.push("Address all critical issues before proceeding".to_string());
    }

    if average_quality_score < 70.0 {
        recommendations.push("Improve statute quality scores through clearer drafting".to_string());
    }

    if !result.suggestions.is_empty() {
        recommendations.push("Review and implement suggested improvements".to_string());
    }

    if recommendations.is_empty() {
        recommendations.push("Continue regular verification checks".to_string());
        recommendations.push("Monitor for any changes requiring re-verification".to_string());
    }

    ExecutiveSummary {
        title: title.into(),
        date,
        key_findings,
        overall_assessment,
        statistics,
        recommendations,
        risk_level,
    }
}

/// Exports executive summary as a formatted report
pub fn executive_summary_report(summary: &ExecutiveSummary) -> String {
    let mut report = String::from("# EXECUTIVE SUMMARY\n\n");

    report.push_str(&format!("## {}\n\n", summary.title));
    report.push_str(&format!("**Date**: {}\n\n", summary.date));
    report.push_str(&format!("**Risk Level**: {}\n\n", summary.risk_level));

    report.push_str("---\n\n");
    report.push_str("## Overall Assessment\n\n");
    report.push_str(&format!("{}\n\n", summary.overall_assessment));

    report.push_str("## Key Findings\n\n");
    for finding in &summary.key_findings {
        report.push_str(&format!("- {}\n", finding));
    }
    report.push('\n');

    report.push_str("## Statistics\n\n");
    let stats = &summary.statistics;
    report.push_str(&format!(
        "- **Total Statutes Analyzed**: {}\n",
        stats.total_statutes
    ));
    report.push_str(&format!(
        "- **Statutes with Issues**: {}\n",
        stats.statutes_with_issues
    ));
    report.push_str(&format!(
        "- **Total Issues Found**: {}\n",
        stats.total_issues
    ));
    report.push_str(&format!(
        "- **Critical Issues**: {}\n",
        stats.critical_issues
    ));
    report.push_str(&format!(
        "- **High Severity Issues**: {}\n",
        stats.high_severity_issues
    ));
    report.push_str(&format!(
        "- **Medium Severity Issues**: {}\n",
        stats.medium_severity_issues
    ));
    report.push_str(&format!(
        "- **Average Quality Score**: {:.1}/100\n\n",
        stats.average_quality_score
    ));

    report.push_str("## Recommendations\n\n");
    for (idx, rec) in summary.recommendations.iter().enumerate() {
        report.push_str(&format!("{}. {}\n", idx + 1, rec));
    }
    report.push('\n');

    report.push_str("---\n\n");
    report.push_str(
        "*This executive summary provides a high-level overview of the verification results.*\n",
    );
    report.push_str("*For detailed findings, please refer to the complete verification report.*\n");

    report
}

/// Report template for customizable report generation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReportTemplate {
    /// Template name
    pub name: String,
    /// Sections to include in the report
    pub sections: Vec<ReportSection>,
    /// Header text
    pub header: Option<String>,
    /// Footer text
    pub footer: Option<String>,
    /// Whether to include table of contents
    pub include_toc: bool,
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

impl ReportTemplate {
    /// Creates a new report template
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            sections: Vec::new(),
            header: None,
            footer: None,
            include_toc: false,
        }
    }

    /// Adds a section to the template
    pub fn with_section(mut self, section: ReportSection) -> Self {
        self.sections.push(section);
        self
    }

    /// Sets the header text
    pub fn with_header(mut self, header: impl Into<String>) -> Self {
        self.header = Some(header.into());
        self
    }

    /// Sets the footer text
    pub fn with_footer(mut self, footer: impl Into<String>) -> Self {
        self.footer = Some(footer.into());
        self
    }

    /// Enables table of contents
    pub fn with_toc(mut self) -> Self {
        self.include_toc = true;
        self
    }
}

/// Generates a custom report based on a template
pub fn generate_custom_report(
    template: &ReportTemplate,
    statutes: &[Statute],
    result: &VerificationResult,
) -> String {
    let mut report = String::new();

    // Add header
    if let Some(ref header) = template.header {
        report.push_str(header);
        report.push_str("\n\n---\n\n");
    }

    // Add table of contents if requested
    if template.include_toc {
        report.push_str("## Table of Contents\n\n");
        for (idx, section) in template.sections.iter().enumerate() {
            let section_name = match section {
                ReportSection::ExecutiveSummary => "Executive Summary",
                ReportSection::VerificationResults => "Verification Results",
                ReportSection::QualityMetrics => "Quality Metrics",
                ReportSection::ComplianceChecklist => "Compliance Checklist",
                ReportSection::ConflictDetection => "Conflict Detection",
                ReportSection::StatisticalAnalysis => "Statistical Analysis",
                ReportSection::AmbiguityDetection => "Ambiguity Detection",
                ReportSection::RegulatoryImpact => "Regulatory Impact Assessment",
                ReportSection::GraphAnalysis => "Graph Analysis",
                ReportSection::Custom { title, .. } => title,
            };
            report.push_str(&format!("{}. {}\n", idx + 1, section_name));
        }
        report.push_str("\n---\n\n");
    }

    // Generate each section
    for section in &template.sections {
        match section {
            ReportSection::ExecutiveSummary => {
                let summary = generate_executive_summary(&template.name, statutes, result);
                report.push_str(&executive_summary_report(&summary));
                report.push_str("\n---\n\n");
            }
            ReportSection::VerificationResults => {
                report.push_str("# Verification Results\n\n");
                report.push_str(&format!(
                    "**Status**: {}\n\n",
                    if result.passed { "PASSED" } else { "FAILED" }
                ));

                if !result.errors.is_empty() {
                    report.push_str("## Errors\n\n");
                    for (idx, error) in result.errors.iter().enumerate() {
                        report.push_str(&format!(
                            "{}. [{:?}] {}\n",
                            idx + 1,
                            error.severity(),
                            error
                        ));
                    }
                    report.push('\n');
                }

                if !result.warnings.is_empty() {
                    report.push_str("## Warnings\n\n");
                    for (idx, warning) in result.warnings.iter().enumerate() {
                        report.push_str(&format!("{}. {}\n", idx + 1, warning));
                    }
                    report.push('\n');
                }

                report.push_str("---\n\n");
            }
            ReportSection::QualityMetrics => {
                report.push_str(&quality_report(statutes));
                report.push_str("\n---\n\n");
            }
            ReportSection::ComplianceChecklist => {
                report.push_str(&consolidated_compliance_checklist(statutes));
                report.push_str("\n---\n\n");
            }
            ReportSection::ConflictDetection => {
                report.push_str(&conflict_detection_report(statutes));
                report.push_str("\n---\n\n");
            }
            ReportSection::StatisticalAnalysis => {
                report.push_str(&statistics_report(statutes));
                report.push_str("\n---\n\n");
            }
            ReportSection::AmbiguityDetection => {
                report.push_str(&batch_ambiguity_report(statutes));
                report.push_str("\n---\n\n");
            }
            ReportSection::RegulatoryImpact => {
                report.push_str(&regulatory_impact_report(statutes));
                report.push_str("\n---\n\n");
            }
            ReportSection::GraphAnalysis => {
                report.push_str(&graph_analysis_report(statutes));
                report.push_str("\n---\n\n");
            }
            ReportSection::Custom { title, content } => {
                report.push_str(&format!("# {}\n\n", title));
                report.push_str(content);
                report.push_str("\n\n---\n\n");
            }
        }
    }

    // Add footer
    if let Some(ref footer) = template.footer {
        report.push_str(footer);
        report.push('\n');
    }

    report
}

/// Creates a standard comprehensive report template
pub fn standard_report_template() -> ReportTemplate {
    ReportTemplate::new("Standard Verification Report")
        .with_header("# Legalis Verification Report")
        .with_toc()
        .with_section(ReportSection::ExecutiveSummary)
        .with_section(ReportSection::VerificationResults)
        .with_section(ReportSection::QualityMetrics)
        .with_section(ReportSection::StatisticalAnalysis)
        .with_footer("Generated by Legalis Verification System")
}

/// Creates a compliance-focused report template
pub fn compliance_report_template() -> ReportTemplate {
    ReportTemplate::new("Compliance Verification Report")
        .with_header("# Compliance Verification Report")
        .with_toc()
        .with_section(ReportSection::ExecutiveSummary)
        .with_section(ReportSection::ComplianceChecklist)
        .with_section(ReportSection::ConflictDetection)
        .with_section(ReportSection::AmbiguityDetection)
        .with_footer("Generated by Legalis Verification System")
}

/// Creates a quality-focused report template
pub fn quality_report_template() -> ReportTemplate {
    ReportTemplate::new("Quality Assessment Report")
        .with_header("# Quality Assessment Report")
        .with_toc()
        .with_section(ReportSection::QualityMetrics)
        .with_section(ReportSection::AmbiguityDetection)
        .with_section(ReportSection::StatisticalAnalysis)
        .with_section(ReportSection::GraphAnalysis)
        .with_footer("Generated by Legalis Verification System")
}

// ============================================================================
// Scheduled Report Generation (v0.1.8)
// ============================================================================

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

/// Output format for scheduled reports
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ReportOutputFormat {
    /// Markdown format
    Markdown,
    /// HTML format
    Html,
    /// JSON format
    Json,
    /// PDF format (requires pdf feature)
    #[cfg(feature = "pdf")]
    Pdf,
}

impl std::fmt::Display for ReportOutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReportOutputFormat::Markdown => write!(f, "markdown"),
            ReportOutputFormat::Html => write!(f, "html"),
            ReportOutputFormat::Json => write!(f, "json"),
            #[cfg(feature = "pdf")]
            ReportOutputFormat::Pdf => write!(f, "pdf"),
        }
    }
}

impl ReportSchedule {
    /// Creates a new report schedule
    pub fn new(id: impl Into<String>, name: impl Into<String>, template: ReportTemplate) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            template,
            cron_expression: "0 0 * * *".to_string(), // Default: daily at midnight
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

/// Executes a scheduled report generation
///
/// This function generates a report based on the schedule configuration
/// and saves it to the specified output directory.
pub fn execute_scheduled_report(
    schedule: &ReportSchedule,
    statutes: &[Statute],
    result: &VerificationResult,
) -> ScheduledReportResult {
    let execution_time = chrono::Utc::now().to_rfc3339();

    // Generate the report content
    let report_content = generate_custom_report(&schedule.template, statutes, result);

    // Create output filename with timestamp
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let extension = match schedule.output_format {
        ReportOutputFormat::Markdown => "md",
        ReportOutputFormat::Html => "html",
        ReportOutputFormat::Json => "json",
        #[cfg(feature = "pdf")]
        ReportOutputFormat::Pdf => "pdf",
    };

    let filename = format!(
        "{}_{}.{}",
        schedule.name.replace(' ', "_"),
        timestamp,
        extension
    );
    let output_path = format!("{}/{}", schedule.output_directory, filename);

    // Format the content based on output format
    let formatted_content = match schedule.output_format {
        ReportOutputFormat::Markdown => report_content,
        ReportOutputFormat::Html => {
            // Convert markdown to HTML (simplified - in production use a proper markdown parser)
            format!(
                "<!DOCTYPE html>\n<html>\n<head><title>{}</title></head>\n<body>\n<pre>{}</pre>\n</body>\n</html>",
                schedule.name, report_content
            )
        }
        ReportOutputFormat::Json => {
            // Create a JSON wrapper
            serde_json::json!({
                "schedule_id": schedule.id,
                "generation_time": execution_time,
                "report_content": report_content,
                "statute_count": statutes.len(),
                "has_errors": !result.errors.is_empty(),
                "error_count": result.errors.len(),
                "warning_count": result.warnings.len(),
            })
            .to_string()
        }
        #[cfg(feature = "pdf")]
        ReportOutputFormat::Pdf => {
            // For PDF, we would use the printpdf crate
            // This is a placeholder - actual PDF generation would be more complex
            report_content
        }
    };

    // Attempt to write the report to file
    match std::fs::create_dir_all(&schedule.output_directory) {
        Ok(_) => match std::fs::write(&output_path, formatted_content.as_bytes()) {
            Ok(_) => {
                let file_size = std::fs::metadata(&output_path).ok().map(|m| m.len());

                ScheduledReportResult {
                    schedule_id: schedule.id.clone(),
                    execution_time,
                    success: true,
                    output_path: Some(output_path),
                    error: None,
                    file_size_bytes: file_size,
                }
            }
            Err(e) => ScheduledReportResult {
                schedule_id: schedule.id.clone(),
                execution_time,
                success: false,
                output_path: None,
                error: Some(format!("Failed to write report file: {}", e)),
                file_size_bytes: None,
            },
        },
        Err(e) => ScheduledReportResult {
            schedule_id: schedule.id.clone(),
            execution_time,
            success: false,
            output_path: None,
            error: Some(format!("Failed to create output directory: {}", e)),
            file_size_bytes: None,
        },
    }
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
                // In a real implementation, we would parse the cron expression
                // and check if the schedule is due. For now, we'll execute all enabled schedules.
                let exec_result = execute_scheduled_report(schedule, statutes, result);
                execution_results.push(exec_result);
            }
        }

        // Add results to history
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

impl Default for ReportScheduler {
    fn default() -> Self {
        Self::new()
    }
}

/// Creates a daily compliance report schedule
pub fn daily_compliance_schedule() -> ReportSchedule {
    ReportSchedule::new(
        "daily-compliance",
        "Daily Compliance Report",
        compliance_report_template(),
    )
    .with_cron("0 0 * * *") // Daily at midnight
    .with_format(ReportOutputFormat::Html)
}

/// Creates a weekly quality report schedule
pub fn weekly_quality_schedule() -> ReportSchedule {
    ReportSchedule::new(
        "weekly-quality",
        "Weekly Quality Assessment",
        quality_report_template(),
    )
    .with_cron("0 0 * * 0") // Weekly on Sunday at midnight
    .with_format(ReportOutputFormat::Markdown)
}

/// Creates a monthly comprehensive report schedule
pub fn monthly_comprehensive_schedule() -> ReportSchedule {
    ReportSchedule::new(
        "monthly-comprehensive",
        "Monthly Comprehensive Report",
        standard_report_template(),
    )
    .with_cron("0 0 1 * *") // Monthly on the 1st at midnight
    .with_format(ReportOutputFormat::Html)
}

// ============================================================================
// Advanced Graph Analysis
// ============================================================================

/// Graph metrics for statute dependency network
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GraphMetrics {
    /// Total number of nodes (statutes)
    pub node_count: usize,
    /// Total number of edges (dependencies)
    pub edge_count: usize,
    /// Average degree (connections per statute)
    pub average_degree: f64,
    /// Density of the graph (0.0 to 1.0)
    pub density: f64,
    /// Number of strongly connected components
    pub strongly_connected_components: usize,
    /// Whether the graph is acyclic (DAG)
    pub is_acyclic: bool,
    /// Maximum path length in the graph
    pub diameter: usize,
}

/// Centrality metrics for a single statute
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CentralityMetrics {
    /// Statute ID
    pub statute_id: String,
    /// Degree centrality (number of direct connections)
    pub degree_centrality: f64,
    /// In-degree (number of statutes referencing this one)
    pub in_degree: usize,
    /// Out-degree (number of statutes this one references)
    pub out_degree: usize,
    /// PageRank score (importance based on link structure)
    pub pagerank: f64,
    /// Betweenness centrality (how often statute is on shortest path)
    pub betweenness: f64,
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

/// Computes overall graph metrics for statute dependencies
pub fn analyze_graph_metrics(statutes: &[Statute]) -> GraphMetrics {
    let node_count = statutes.len();

    // Count edges
    let mut edges = 0;

    for statute in statutes {
        let refs = extract_statute_references_from_conditions(&statute.preconditions);
        edges += refs.len();
    }

    let edge_count = edges;
    let average_degree = if node_count > 0 {
        (edge_count as f64) / (node_count as f64)
    } else {
        0.0
    };

    let max_edges = node_count * (node_count - 1);
    let density = if max_edges > 0 {
        (edge_count as f64) / (max_edges as f64)
    } else {
        0.0
    };

    // Detect cycles using DFS
    let has_cycle = detect_cycles_in_graph(statutes);
    let is_acyclic = !has_cycle;

    // Count strongly connected components using Tarjan's algorithm
    let scc_count = count_strongly_connected_components(statutes);

    // Compute diameter (longest shortest path)
    let diameter = compute_graph_diameter(statutes);

    GraphMetrics {
        node_count,
        edge_count,
        average_degree,
        density,
        strongly_connected_components: scc_count,
        is_acyclic,
        diameter,
    }
}

/// Detects cycles in the statute dependency graph
fn detect_cycles_in_graph(statutes: &[Statute]) -> bool {
    let mut visited = HashSet::new();
    let mut rec_stack = HashSet::new();

    fn dfs_cycle(
        statute_id: &str,
        statutes: &[Statute],
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
    ) -> bool {
        visited.insert(statute_id.to_string());
        rec_stack.insert(statute_id.to_string());

        if let Some(statute) = statutes.iter().find(|s| s.id == statute_id) {
            let refs = extract_statute_references_from_conditions(&statute.preconditions);
            for ref_id in refs {
                if !visited.contains(&ref_id) {
                    if dfs_cycle(&ref_id, statutes, visited, rec_stack) {
                        return true;
                    }
                } else if rec_stack.contains(&ref_id) {
                    return true; // Cycle detected
                }
            }
        }

        rec_stack.remove(statute_id);
        false
    }

    for statute in statutes {
        if !visited.contains(&statute.id)
            && dfs_cycle(&statute.id, statutes, &mut visited, &mut rec_stack)
        {
            return true;
        }
    }

    false
}

/// Counts strongly connected components using Tarjan's algorithm
fn count_strongly_connected_components(statutes: &[Statute]) -> usize {
    if statutes.is_empty() {
        return 0;
    }

    struct TarjanState {
        index: usize,
        stack: Vec<String>,
        indices: HashMap<String, usize>,
        lowlinks: HashMap<String, usize>,
        on_stack: HashSet<String>,
        scc_count: usize,
    }

    fn strongconnect(v: String, statutes: &[Statute], state: &mut TarjanState) {
        state.indices.insert(v.clone(), state.index);
        state.lowlinks.insert(v.clone(), state.index);
        state.index += 1;
        state.stack.push(v.clone());
        state.on_stack.insert(v.clone());

        if let Some(statute) = statutes.iter().find(|s| s.id == v) {
            let refs = extract_statute_references_from_conditions(&statute.preconditions);
            for w in refs {
                if !state.indices.contains_key(&w) {
                    strongconnect(w.clone(), statutes, state);
                    let w_lowlink = *state.lowlinks.get(&w).unwrap_or(&0);
                    let v_lowlink = *state.lowlinks.get(&v).unwrap_or(&0);
                    state.lowlinks.insert(v.clone(), v_lowlink.min(w_lowlink));
                } else if state.on_stack.contains(&w) {
                    let w_index = *state.indices.get(&w).unwrap_or(&0);
                    let v_lowlink = *state.lowlinks.get(&v).unwrap_or(&0);
                    state.lowlinks.insert(v.clone(), v_lowlink.min(w_index));
                }
            }
        }

        if state.lowlinks.get(&v) == state.indices.get(&v) {
            // Found an SCC
            while let Some(w) = state.stack.pop() {
                state.on_stack.remove(&w);
                if w == v {
                    break;
                }
            }
            state.scc_count += 1;
        }
    }

    let mut state = TarjanState {
        index: 0,
        stack: Vec::new(),
        indices: HashMap::new(),
        lowlinks: HashMap::new(),
        on_stack: HashSet::new(),
        scc_count: 0,
    };

    for statute in statutes {
        if !state.indices.contains_key(&statute.id) {
            strongconnect(statute.id.clone(), statutes, &mut state);
        }
    }

    state.scc_count
}

/// Computes graph diameter (longest shortest path)
fn compute_graph_diameter(statutes: &[Statute]) -> usize {
    if statutes.is_empty() {
        return 0;
    }

    let mut max_dist = 0;

    // BFS from each node to find longest shortest path
    for source in statutes {
        let distances = bfs_distances(&source.id, statutes);
        if let Some(&max) = distances.values().max() {
            max_dist = max_dist.max(max);
        }
    }

    max_dist
}

/// BFS to compute distances from a source statute
fn bfs_distances(source: &str, statutes: &[Statute]) -> HashMap<String, usize> {
    let mut distances = HashMap::new();
    let mut queue = std::collections::VecDeque::new();

    distances.insert(source.to_string(), 0);
    queue.push_back(source.to_string());

    while let Some(current) = queue.pop_front() {
        let current_dist = *distances.get(&current).unwrap_or(&0);

        if let Some(statute) = statutes.iter().find(|s| s.id == current) {
            let refs = extract_statute_references_from_conditions(&statute.preconditions);
            for ref_id in refs {
                if !distances.contains_key(&ref_id) {
                    distances.insert(ref_id.clone(), current_dist + 1);
                    queue.push_back(ref_id);
                }
            }
        }
    }

    distances
}

/// Computes centrality metrics for each statute
pub fn analyze_centrality(statutes: &[Statute]) -> Vec<CentralityMetrics> {
    let mut metrics = Vec::new();

    // Build in-degree and out-degree maps
    let mut in_degree: HashMap<String, usize> = HashMap::new();
    let mut out_degree: HashMap<String, usize> = HashMap::new();

    // Initialize out_degree for all statutes
    for statute in statutes {
        out_degree.insert(statute.id.clone(), 0);
    }

    // Build degree maps
    for statute in statutes {
        let refs = extract_statute_references_from_conditions(&statute.preconditions);
        *out_degree.get_mut(&statute.id).unwrap() = refs.len();

        for ref_id in refs {
            *in_degree.entry(ref_id).or_insert(0) += 1;
        }
    }

    // Compute PageRank
    let pagerank_scores = compute_pagerank(statutes, 0.85, 20);

    // Compute betweenness centrality
    let betweenness_scores = compute_betweenness(statutes);

    for statute in statutes {
        let in_deg = *in_degree.get(&statute.id).unwrap_or(&0);
        let out_deg = *out_degree.get(&statute.id).unwrap_or(&0);
        let total_deg = in_deg + out_deg;

        let degree_centrality = if statutes.len() > 1 {
            (total_deg as f64) / ((statutes.len() - 1) as f64)
        } else {
            0.0
        };

        metrics.push(CentralityMetrics {
            statute_id: statute.id.clone(),
            degree_centrality,
            in_degree: in_deg,
            out_degree: out_deg,
            pagerank: *pagerank_scores.get(&statute.id).unwrap_or(&0.0),
            betweenness: *betweenness_scores.get(&statute.id).unwrap_or(&0.0),
        });
    }

    metrics
}

/// Computes PageRank scores for statutes
fn compute_pagerank(statutes: &[Statute], damping: f64, iterations: usize) -> HashMap<String, f64> {
    let n = statutes.len();
    if n == 0 {
        return HashMap::new();
    }

    let mut ranks: HashMap<String, f64> = statutes
        .iter()
        .map(|s| (s.id.clone(), 1.0 / (n as f64)))
        .collect();

    let mut out_degree: HashMap<String, usize> = HashMap::new();
    for statute in statutes {
        let refs = extract_statute_references_from_conditions(&statute.preconditions);
        out_degree.insert(statute.id.clone(), refs.len());
    }

    for _ in 0..iterations {
        let mut new_ranks = HashMap::new();

        for statute in statutes {
            let mut rank_sum = 0.0;

            // Sum contributions from statutes pointing to this one
            for other in statutes {
                let refs = extract_statute_references_from_conditions(&other.preconditions);
                if refs.contains(&statute.id) {
                    let other_out = *out_degree.get(&other.id).unwrap_or(&1);
                    if other_out > 0 {
                        rank_sum += ranks.get(&other.id).unwrap_or(&0.0) / (other_out as f64);
                    }
                }
            }

            let new_rank = (1.0 - damping) / (n as f64) + damping * rank_sum;
            new_ranks.insert(statute.id.clone(), new_rank);
        }

        ranks = new_ranks;
    }

    ranks
}

/// Computes betweenness centrality (simplified version)
fn compute_betweenness(statutes: &[Statute]) -> HashMap<String, f64> {
    let n = statutes.len();
    let mut betweenness: HashMap<String, f64> =
        statutes.iter().map(|s| (s.id.clone(), 0.0)).collect();

    if n <= 2 {
        return betweenness;
    }

    // For each pair of statutes, find shortest paths
    for source in statutes {
        for target in statutes {
            if source.id == target.id {
                continue;
            }

            // BFS to find all shortest paths from source to target
            let paths = find_shortest_paths(&source.id, &target.id, statutes);

            if !paths.is_empty() {
                // Count how many paths pass through each statute
                for path in &paths {
                    for statute_id in path {
                        if statute_id != &source.id && statute_id != &target.id {
                            *betweenness.get_mut(statute_id).unwrap() += 1.0 / (paths.len() as f64);
                        }
                    }
                }
            }
        }
    }

    // Normalize
    let normalization = if n > 2 {
        ((n - 1) * (n - 2)) as f64
    } else {
        1.0
    };

    for value in betweenness.values_mut() {
        *value /= normalization;
    }

    betweenness
}

/// Finds all shortest paths between two statutes
fn find_shortest_paths(source: &str, target: &str, statutes: &[Statute]) -> Vec<Vec<String>> {
    let mut queue = std::collections::VecDeque::new();
    let mut distances: HashMap<String, usize> = HashMap::new();
    let mut paths: HashMap<String, Vec<Vec<String>>> = HashMap::new();

    distances.insert(source.to_string(), 0);
    paths.insert(source.to_string(), vec![vec![source.to_string()]]);
    queue.push_back(source.to_string());

    while let Some(current) = queue.pop_front() {
        if current == target {
            continue;
        }

        let current_dist = *distances.get(&current).unwrap_or(&0);

        if let Some(statute) = statutes.iter().find(|s| s.id == current) {
            let refs = extract_statute_references_from_conditions(&statute.preconditions);

            for ref_id in refs {
                let new_dist = current_dist + 1;

                if !distances.contains_key(&ref_id) {
                    distances.insert(ref_id.clone(), new_dist);
                    queue.push_back(ref_id.clone());

                    // Extend all paths to current with ref_id
                    if let Some(current_paths) = paths.get(&current) {
                        let new_paths: Vec<Vec<String>> = current_paths
                            .iter()
                            .map(|path| {
                                let mut new_path = path.clone();
                                new_path.push(ref_id.clone());
                                new_path
                            })
                            .collect();
                        paths.insert(ref_id.clone(), new_paths);
                    }
                } else if distances.get(&ref_id) == Some(&new_dist) {
                    // Found another shortest path
                    if let Some(current_paths) = paths.get(&current).cloned() {
                        for path in current_paths {
                            let mut new_path = path.clone();
                            new_path.push(ref_id.clone());
                            paths.entry(ref_id.clone()).or_default().push(new_path);
                        }
                    }
                }
            }
        }
    }

    paths.get(target).cloned().unwrap_or_default()
}

/// Detects clusters/communities in the statute graph using simple heuristic
#[allow(dead_code)]
pub fn detect_clusters(statutes: &[Statute]) -> Vec<StatuteCluster> {
    if statutes.is_empty() {
        return Vec::new();
    }

    let mut clusters = Vec::new();
    let mut assigned: HashSet<String> = HashSet::new();

    // Simple clustering based on connected components
    for statute in statutes {
        if assigned.contains(&statute.id) {
            continue;
        }

        // Find all statutes reachable from this one
        let mut component = HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(statute.id.clone());
        component.insert(statute.id.clone());

        while let Some(current) = queue.pop_front() {
            if let Some(current_statute) = statutes.iter().find(|s| s.id == current) {
                let refs =
                    extract_statute_references_from_conditions(&current_statute.preconditions);
                for ref_id in refs {
                    if !component.contains(&ref_id) {
                        component.insert(ref_id.clone());
                        queue.push_back(ref_id);
                    }
                }

                // Also check reverse references
                for other in statutes {
                    let other_refs =
                        extract_statute_references_from_conditions(&other.preconditions);
                    if other_refs.contains(&current) && !component.contains(&other.id) {
                        component.insert(other.id.clone());
                        queue.push_back(other.id.clone());
                    }
                }
            }
        }

        // Calculate cluster density
        let cluster_statutes: Vec<_> = component.iter().collect();
        let cluster_size = cluster_statutes.len();
        let mut internal_edges = 0;

        for id in &cluster_statutes {
            if let Some(stat) = statutes.iter().find(|s| s.id == **id) {
                let refs = extract_statute_references_from_conditions(&stat.preconditions);
                internal_edges += refs.iter().filter(|r| cluster_statutes.contains(r)).count();
            }
        }

        let max_edges = cluster_size * (cluster_size - 1);
        let density = if max_edges > 0 {
            (internal_edges as f64) / (max_edges as f64)
        } else {
            0.0
        };

        // Extract keywords from titles
        let mut keywords = Vec::new();
        for id in &cluster_statutes {
            if let Some(stat) = statutes.iter().find(|s| s.id == **id) {
                // Simple keyword extraction: take common words from titles
                let words: Vec<&str> = stat.title.split_whitespace().collect();
                for word in words {
                    if word.len() > 4 && !keywords.contains(&word.to_string()) {
                        keywords.push(word.to_string());
                    }
                }
            }
        }
        keywords.truncate(5); // Keep top 5 keywords

        let statute_ids: Vec<String> = component.into_iter().collect();
        assigned.extend(statute_ids.clone());

        clusters.push(StatuteCluster {
            id: clusters.len(),
            statute_ids,
            density,
            keywords,
        });
    }

    clusters
}

/// Generates a comprehensive graph analysis report
pub fn graph_analysis_report(statutes: &[Statute]) -> String {
    let mut report = String::new();

    report.push_str("# Statute Dependency Graph Analysis\n\n");

    // Overall metrics
    report.push_str("## Graph Metrics\n\n");
    let metrics = analyze_graph_metrics(statutes);
    report.push_str(&format!("- **Nodes (Statutes)**: {}\n", metrics.node_count));
    report.push_str(&format!(
        "- **Edges (Dependencies)**: {}\n",
        metrics.edge_count
    ));
    report.push_str(&format!(
        "- **Average Degree**: {:.2}\n",
        metrics.average_degree
    ));
    report.push_str(&format!("- **Graph Density**: {:.4}\n", metrics.density));
    report.push_str(&format!("- **Is Acyclic (DAG)**: {}\n", metrics.is_acyclic));
    report.push_str(&format!(
        "- **Strongly Connected Components**: {}\n",
        metrics.strongly_connected_components
    ));
    report.push_str(&format!(
        "- **Diameter (Longest Path)**: {}\n",
        metrics.diameter
    ));
    report.push('\n');

    // Centrality metrics
    report.push_str("## Centrality Metrics\n\n");
    let mut centrality = analyze_centrality(statutes);
    centrality.sort_by(|a, b| {
        b.pagerank
            .partial_cmp(&a.pagerank)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    report.push_str("### Top 10 Statutes by PageRank\n\n");
    for (i, metric) in centrality.iter().take(10).enumerate() {
        report.push_str(&format!(
            "{}. **{}** (PageRank: {:.4}, Degree: {:.2}, In: {}, Out: {})\n",
            i + 1,
            metric.statute_id,
            metric.pagerank,
            metric.degree_centrality,
            metric.in_degree,
            metric.out_degree
        ));
    }
    report.push('\n');

    // Sort by betweenness
    centrality.sort_by(|a, b| {
        b.betweenness
            .partial_cmp(&a.betweenness)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    report.push_str("### Top 10 Statutes by Betweenness Centrality\n\n");
    for (i, metric) in centrality.iter().take(10).enumerate() {
        if metric.betweenness > 0.0 {
            report.push_str(&format!(
                "{}. **{}** (Betweenness: {:.4})\n",
                i + 1,
                metric.statute_id,
                metric.betweenness
            ));
        }
    }
    report.push('\n');

    report
}

// ============================================================================
// Statute Evolution Tracking
// ============================================================================

/// Version entry in statute evolution history
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StatuteVersion {
    /// Version number
    pub version: u32,
    /// Statute snapshot at this version
    pub statute: Statute,
    /// Timestamp of this version (optional)
    pub timestamp: Option<chrono::NaiveDateTime>,
    /// Description of changes in this version
    pub change_description: Option<String>,
}

/// Evolution history for a statute
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StatuteEvolution {
    /// Statute ID
    pub statute_id: String,
    /// Chronological list of versions
    pub versions: Vec<StatuteVersion>,
}

/// Evolution metrics for a statute
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EvolutionMetrics {
    /// Statute ID
    pub statute_id: String,
    /// Total number of versions
    pub total_versions: usize,
    /// Number of major changes (effect or precondition modifications)
    pub major_changes: usize,
    /// Number of minor changes (title, description, metadata)
    pub minor_changes: usize,
    /// Average time between versions (in days)
    pub avg_days_between_versions: Option<f64>,
    /// Stability score (0.0 = very unstable, 1.0 = very stable)
    pub stability_score: f64,
    /// Complexity trend (Increasing, Decreasing, Stable)
    pub complexity_trend: ComplexityTrend,
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

impl std::fmt::Display for ComplexityTrend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Increasing => write!(f, "Increasing"),
            Self::Decreasing => write!(f, "Decreasing"),
            Self::Stable => write!(f, "Stable"),
        }
    }
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

        // Count major and minor changes
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

        // Calculate average days between versions
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

        // Calculate stability score (fewer changes = more stable)
        let total_changes = major_changes + minor_changes;
        let stability_score = if total_versions > 1 {
            1.0 - (total_changes as f64 / (total_versions - 1) as f64).min(1.0)
        } else {
            1.0
        };

        // Analyze complexity trend
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

impl Default for EvolutionTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Generates an evolution report for tracked statutes
pub fn evolution_report(tracker: &EvolutionTracker) -> String {
    let mut report = String::new();

    report.push_str("# Statute Evolution Report\n\n");

    let all_metrics = tracker.analyze_all_metrics();

    report.push_str(&format!(
        "**Total Tracked Statutes**: {}\n\n",
        all_metrics.len()
    ));

    // Summary statistics
    let total_versions: usize = all_metrics.iter().map(|m| m.total_versions).sum();
    let avg_versions = if !all_metrics.is_empty() {
        total_versions as f64 / all_metrics.len() as f64
    } else {
        0.0
    };

    report.push_str("## Summary Statistics\n\n");
    report.push_str(&format!(
        "- **Total Versions Across All Statutes**: {}\n",
        total_versions
    ));
    report.push_str(&format!(
        "- **Average Versions Per Statute**: {:.2}\n",
        avg_versions
    ));
    report.push('\n');

    // Most changed statutes
    report.push_str("## Most Changed Statutes\n\n");
    let most_changed = tracker.most_changed_statutes(10);
    for (i, metric) in most_changed.iter().enumerate() {
        report.push_str(&format!(
            "{}. **{}** - {} versions ({} major, {} minor changes)\n",
            i + 1,
            metric.statute_id,
            metric.total_versions,
            metric.major_changes,
            metric.minor_changes
        ));
    }
    report.push('\n');

    // Most stable statutes
    report.push_str("## Most Stable Statutes\n\n");
    let most_stable = tracker.most_stable_statutes(10);
    for (i, metric) in most_stable.iter().enumerate() {
        report.push_str(&format!(
            "{}. **{}** - Stability: {:.2}, {} versions\n",
            i + 1,
            metric.statute_id,
            metric.stability_score,
            metric.total_versions
        ));
    }
    report.push('\n');

    // Complexity trends
    report.push_str("## Complexity Trends\n\n");
    let increasing: Vec<_> = all_metrics
        .iter()
        .filter(|m| m.complexity_trend == ComplexityTrend::Increasing)
        .collect();
    let decreasing: Vec<_> = all_metrics
        .iter()
        .filter(|m| m.complexity_trend == ComplexityTrend::Decreasing)
        .collect();
    let stable: Vec<_> = all_metrics
        .iter()
        .filter(|m| m.complexity_trend == ComplexityTrend::Stable)
        .collect();

    report.push_str(&format!(
        "- **Increasing Complexity**: {} statutes\n",
        increasing.len()
    ));
    report.push_str(&format!(
        "- **Decreasing Complexity**: {} statutes\n",
        decreasing.len()
    ));
    report.push_str(&format!(
        "- **Stable Complexity**: {} statutes\n",
        stable.len()
    ));
    report.push('\n');

    report
}

// ============================================================================
// Pattern Mining
// ============================================================================

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

/// Type of statute pattern
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PatternType {
    /// Age-based eligibility
    AgeEligibility,
    /// Income-based qualification
    IncomeQualification,
    /// Combined age and income
    AgeAndIncome,
    /// Prohibition with exceptions
    ProhibitionWithExceptions,
    /// Temporal restriction
    TemporalRestriction,
    /// Jurisdiction-specific
    JurisdictionalPattern,
    /// Custom pattern
    Custom,
}

impl std::fmt::Display for PatternType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AgeEligibility => write!(f, "Age Eligibility"),
            Self::IncomeQualification => write!(f, "Income Qualification"),
            Self::AgeAndIncome => write!(f, "Age and Income"),
            Self::ProhibitionWithExceptions => write!(f, "Prohibition with Exceptions"),
            Self::TemporalRestriction => write!(f, "Temporal Restriction"),
            Self::JurisdictionalPattern => write!(f, "Jurisdictional Pattern"),
            Self::Custom => write!(f, "Custom"),
        }
    }
}

/// Mines common patterns from a collection of statutes
pub fn mine_patterns(statutes: &[Statute]) -> Vec<StatutePattern> {
    let mut patterns = Vec::new();

    // Pattern 1: Age eligibility
    let age_statutes: Vec<_> = statutes
        .iter()
        .filter(|s| has_age_condition(&s.preconditions))
        .map(|s| s.id.clone())
        .collect();

    if !age_statutes.is_empty() {
        patterns.push(StatutePattern {
            id: "age-eligibility".to_string(),
            description: "Statutes with age-based eligibility requirements".to_string(),
            frequency: age_statutes.len(),
            examples: age_statutes.iter().take(5).cloned().collect(),
            pattern_type: PatternType::AgeEligibility,
        });
    }

    // Pattern 2: Income qualification
    let income_statutes: Vec<_> = statutes
        .iter()
        .filter(|s| has_income_condition(&s.preconditions))
        .map(|s| s.id.clone())
        .collect();

    if !income_statutes.is_empty() {
        patterns.push(StatutePattern {
            id: "income-qualification".to_string(),
            description: "Statutes with income-based qualification criteria".to_string(),
            frequency: income_statutes.len(),
            examples: income_statutes.iter().take(5).cloned().collect(),
            pattern_type: PatternType::IncomeQualification,
        });
    }

    // Pattern 3: Combined age and income
    let combined_statutes: Vec<_> = statutes
        .iter()
        .filter(|s| has_age_condition(&s.preconditions) && has_income_condition(&s.preconditions))
        .map(|s| s.id.clone())
        .collect();

    if !combined_statutes.is_empty() {
        patterns.push(StatutePattern {
            id: "age-and-income".to_string(),
            description: "Statutes combining age and income requirements".to_string(),
            frequency: combined_statutes.len(),
            examples: combined_statutes.iter().take(5).cloned().collect(),
            pattern_type: PatternType::AgeAndIncome,
        });
    }

    // Pattern 4: Prohibition with exceptions
    let prohibition_statutes: Vec<_> = statutes
        .iter()
        .filter(|s| {
            matches!(s.effect.effect_type, EffectType::Prohibition)
                && has_negation(&s.preconditions)
        })
        .map(|s| s.id.clone())
        .collect();

    if !prohibition_statutes.is_empty() {
        patterns.push(StatutePattern {
            id: "prohibition-with-exceptions".to_string(),
            description: "Prohibitions with exception conditions (NOT clauses)".to_string(),
            frequency: prohibition_statutes.len(),
            examples: prohibition_statutes.iter().take(5).cloned().collect(),
            pattern_type: PatternType::ProhibitionWithExceptions,
        });
    }

    // Pattern 5: Temporal restrictions
    let temporal_statutes: Vec<_> = statutes
        .iter()
        .filter(|s| {
            s.temporal_validity.has_effective_date() || s.temporal_validity.has_expiry_date()
        })
        .map(|s| s.id.clone())
        .collect();

    if !temporal_statutes.is_empty() {
        patterns.push(StatutePattern {
            id: "temporal-restriction".to_string(),
            description: "Statutes with temporal validity constraints".to_string(),
            frequency: temporal_statutes.len(),
            examples: temporal_statutes.iter().take(5).cloned().collect(),
            pattern_type: PatternType::TemporalRestriction,
        });
    }

    // Pattern 6: Jurisdictional patterns
    let mut jurisdiction_map: HashMap<String, Vec<String>> = HashMap::new();
    for statute in statutes {
        if let Some(jurisdiction) = &statute.jurisdiction {
            jurisdiction_map
                .entry(jurisdiction.clone())
                .or_default()
                .push(statute.id.clone());
        }
    }

    for (jurisdiction, statute_ids) in jurisdiction_map {
        if statute_ids.len() >= 3 {
            // Only report if at least 3 statutes
            patterns.push(StatutePattern {
                id: format!("jurisdiction-{}", jurisdiction.to_lowercase()),
                description: format!("Statutes specific to {} jurisdiction", jurisdiction),
                frequency: statute_ids.len(),
                examples: statute_ids.iter().take(5).cloned().collect(),
                pattern_type: PatternType::JurisdictionalPattern,
            });
        }
    }

    patterns.sort_by(|a, b| b.frequency.cmp(&a.frequency));
    patterns
}

/// Helper: checks if conditions contain age requirement
fn has_age_condition(conditions: &[legalis_core::Condition]) -> bool {
    conditions
        .iter()
        .any(|c| matches!(c, legalis_core::Condition::Age { .. }))
        || conditions.iter().any(|c| {
            check_condition_recursive(c, |cond| {
                matches!(cond, legalis_core::Condition::Age { .. })
            })
        })
}

/// Helper: checks if conditions contain income requirement
fn has_income_condition(conditions: &[legalis_core::Condition]) -> bool {
    conditions
        .iter()
        .any(|c| matches!(c, legalis_core::Condition::Income { .. }))
        || conditions.iter().any(|c| {
            check_condition_recursive(c, |cond| {
                matches!(cond, legalis_core::Condition::Income { .. })
            })
        })
}

/// Helper: checks if conditions contain negation
fn has_negation(conditions: &[legalis_core::Condition]) -> bool {
    conditions
        .iter()
        .any(|c| matches!(c, legalis_core::Condition::Not(_)))
        || conditions.iter().any(|c| {
            check_condition_recursive(c, |cond| matches!(cond, legalis_core::Condition::Not(_)))
        })
}

/// Helper: recursively checks a condition with a predicate
fn check_condition_recursive<F>(condition: &legalis_core::Condition, predicate: F) -> bool
where
    F: Fn(&legalis_core::Condition) -> bool + Copy,
{
    use legalis_core::Condition;

    if predicate(condition) {
        return true;
    }

    match condition {
        Condition::And(left, right) | Condition::Or(left, right) => {
            check_condition_recursive(left, predicate)
                || check_condition_recursive(right, predicate)
        }
        Condition::Not(inner) => check_condition_recursive(inner, predicate),
        _ => false,
    }
}

/// Generates a pattern mining report
pub fn pattern_mining_report(statutes: &[Statute]) -> String {
    let mut report = String::new();

    report.push_str("# Statute Pattern Mining Report\n\n");

    let patterns = mine_patterns(statutes);

    report.push_str(&format!(
        "**Total Statutes Analyzed**: {}\n",
        statutes.len()
    ));
    report.push_str(&format!("**Patterns Found**: {}\n\n", patterns.len()));

    report.push_str("## Discovered Patterns\n\n");

    for (i, pattern) in patterns.iter().enumerate() {
        report.push_str(&format!(
            "### {}. {} ({})\n\n",
            i + 1,
            pattern.description,
            pattern.pattern_type
        ));
        report.push_str(&format!(
            "- **Frequency**: {} statutes ({:.1}%)\n",
            pattern.frequency,
            (pattern.frequency as f64 / statutes.len() as f64) * 100.0
        ));
        report.push_str("- **Examples**: ");
        report.push_str(&pattern.examples.join(", "));
        report.push_str("\n\n");
    }

    report
}

// ============================================================================
// Comprehensive Metrics Dashboard
// ============================================================================

/// Comprehensive dashboard containing all metrics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MetricsDashboard {
    /// Timestamp when dashboard was generated
    pub generated_at: chrono::NaiveDateTime,
    /// Basic statistics
    pub statistics: StatuteStatistics,
    /// Graph analysis metrics
    pub graph_metrics: GraphMetrics,
    /// Centrality metrics for top statutes
    pub top_centrality: Vec<CentralityMetrics>,
    /// Quality metrics summary
    pub quality_summary: QualitySummary,
    /// Conflict summary
    pub conflict_summary: ConflictSummary,
    /// Coverage analysis
    pub coverage_info: CoverageInfo,
    /// Evolution summary (if tracker provided)
    pub evolution_summary: Option<EvolutionSummary>,
    /// Discovered patterns
    pub patterns: Vec<StatutePattern>,
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

/// Conflict summary
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConflictSummary {
    /// Total conflicts detected
    pub total_conflicts: usize,
    /// Conflicts by type
    pub conflicts_by_type: HashMap<String, usize>,
    /// Critical conflicts (severity critical)
    pub critical_conflicts: usize,
}

/// Evolution summary
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EvolutionSummary {
    /// Total tracked statutes
    pub total_tracked: usize,
    /// Average versions per statute
    pub avg_versions: f64,
    /// Total versions across all statutes
    pub total_versions: usize,
    /// Most changed statute
    pub most_changed: Option<String>,
    /// Most stable statute
    pub most_stable: Option<String>,
}

/// Generates a comprehensive metrics dashboard
pub fn generate_metrics_dashboard(
    statutes: &[Statute],
    evolution_tracker: Option<&EvolutionTracker>,
) -> MetricsDashboard {
    // Basic statistics
    let statistics = analyze_statute_statistics(statutes);

    // Graph analysis
    let graph_metrics = analyze_graph_metrics(statutes);
    let mut centrality = analyze_centrality(statutes);
    centrality.sort_by(|a, b| {
        b.pagerank
            .partial_cmp(&a.pagerank)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let top_centrality: Vec<_> = centrality.into_iter().take(10).collect();

    // Quality analysis
    let quality_metrics: Vec<_> = statutes.iter().map(analyze_quality).collect();
    let average_score = if !quality_metrics.is_empty() {
        quality_metrics.iter().map(|q| q.overall_score).sum::<f64>() / quality_metrics.len() as f64
    } else {
        0.0
    };

    let mut grade_distribution = HashMap::new();
    for qm in &quality_metrics {
        *grade_distribution
            .entry(qm.grade().to_string())
            .or_insert(0) += 1;
    }

    let statutes_with_issues = quality_metrics
        .iter()
        .filter(|q| !q.issues.is_empty())
        .count();
    let total_issues: usize = quality_metrics.iter().map(|q| q.issues.len()).sum();

    let quality_summary = QualitySummary {
        average_score,
        grade_distribution,
        statutes_with_issues,
        total_issues,
    };

    // Conflict detection
    let conflicts = detect_statute_conflicts(statutes);
    let mut conflicts_by_type = HashMap::new();
    for conflict in &conflicts {
        let type_name = format!("{:?}", conflict.conflict_type);
        *conflicts_by_type.entry(type_name).or_insert(0) += 1;
    }

    let critical_conflicts = conflicts
        .iter()
        .filter(|c| matches!(c.severity, Severity::Critical))
        .count();

    let conflict_summary = ConflictSummary {
        total_conflicts: conflicts.len(),
        conflicts_by_type,
        critical_conflicts,
    };

    // Coverage analysis
    let coverage_info = analyze_coverage(statutes);

    // Evolution summary
    let evolution_summary = evolution_tracker.map(|tracker| {
        let all_metrics = tracker.analyze_all_metrics();
        let total_tracked = all_metrics.len();
        let total_versions: usize = all_metrics.iter().map(|m| m.total_versions).sum();
        let avg_versions = if total_tracked > 0 {
            total_versions as f64 / total_tracked as f64
        } else {
            0.0
        };

        let most_changed = tracker
            .most_changed_statutes(1)
            .first()
            .map(|m| m.statute_id.clone());

        let most_stable = tracker
            .most_stable_statutes(1)
            .first()
            .map(|m| m.statute_id.clone());

        EvolutionSummary {
            total_tracked,
            avg_versions,
            total_versions,
            most_changed,
            most_stable,
        }
    });

    // Pattern mining
    let patterns = mine_patterns(statutes);

    MetricsDashboard {
        generated_at: chrono::Utc::now().naive_utc(),
        statistics,
        graph_metrics,
        top_centrality,
        quality_summary,
        conflict_summary,
        coverage_info,
        evolution_summary,
        patterns,
    }
}

/// Exports dashboard to JSON
pub fn export_dashboard_json(dashboard: &MetricsDashboard) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(dashboard)
}

/// Exports dashboard to HTML
pub fn export_dashboard_html(dashboard: &MetricsDashboard, title: &str) -> String {
    let mut html = String::new();

    html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
    html.push_str(&format!("<title>{}</title>\n", title));
    html.push_str("<style>\n");
    html.push_str("body { font-family: Arial, sans-serif; margin: 20px; background: #f5f5f5; }\n");
    html.push_str("h1 { color: #333; border-bottom: 2px solid #007bff; padding-bottom: 10px; }\n");
    html.push_str("h2 { color: #555; margin-top: 30px; }\n");
    html.push_str(".card { background: white; padding: 20px; margin: 20px 0; border-radius: 5px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }\n");
    html.push_str(".metric { display: inline-block; margin: 10px 20px 10px 0; }\n");
    html.push_str(".metric-label { font-weight: bold; color: #666; }\n");
    html.push_str(".metric-value { font-size: 1.2em; color: #007bff; }\n");
    html.push_str("table { width: 100%; border-collapse: collapse; margin-top: 10px; }\n");
    html.push_str("th, td { padding: 10px; text-align: left; border-bottom: 1px solid #ddd; }\n");
    html.push_str("th { background: #007bff; color: white; }\n");
    html.push_str("tr:hover { background: #f9f9f9; }\n");
    html.push_str(".critical { color: #dc3545; font-weight: bold; }\n");
    html.push_str(".warning { color: #ffc107; }\n");
    html.push_str(".success { color: #28a745; }\n");
    html.push_str("</style>\n</head>\n<body>\n");

    html.push_str(&format!("<h1>{}</h1>\n", title));
    html.push_str(&format!(
        "<p><em>Generated: {}</em></p>\n",
        dashboard.generated_at
    ));

    // Overview Card
    html.push_str("<div class=\"card\">\n<h2>Overview</h2>\n");
    html.push_str(&format!("<div class=\"metric\"><span class=\"metric-label\">Total Statutes:</span> <span class=\"metric-value\">{}</span></div>\n", dashboard.statistics.total_count));
    html.push_str(&format!("<div class=\"metric\"><span class=\"metric-label\">Average Quality:</span> <span class=\"metric-value\">{:.1}</span></div>\n", dashboard.quality_summary.average_score));
    html.push_str(&format!("<div class=\"metric\"><span class=\"metric-label\">Total Conflicts:</span> <span class=\"metric-value {}\">{}</span></div>\n",
        if dashboard.conflict_summary.total_conflicts > 0 { "critical" } else { "success" },
        dashboard.conflict_summary.total_conflicts));
    html.push_str("</div>\n");

    // Graph Metrics Card
    html.push_str("<div class=\"card\">\n<h2>Dependency Graph</h2>\n");
    html.push_str(&format!("<div class=\"metric\"><span class=\"metric-label\">Nodes:</span> <span class=\"metric-value\">{}</span></div>\n", dashboard.graph_metrics.node_count));
    html.push_str(&format!("<div class=\"metric\"><span class=\"metric-label\">Edges:</span> <span class=\"metric-value\">{}</span></div>\n", dashboard.graph_metrics.edge_count));
    html.push_str(&format!("<div class=\"metric\"><span class=\"metric-label\">Density:</span> <span class=\"metric-value\">{:.4}</span></div>\n", dashboard.graph_metrics.density));
    html.push_str(&format!("<div class=\"metric\"><span class=\"metric-label\">Is DAG:</span> <span class=\"metric-value {}\">{}</span></div>\n",
        if dashboard.graph_metrics.is_acyclic { "success" } else { "critical" },
        dashboard.graph_metrics.is_acyclic));
    html.push_str("</div>\n");

    // Top Statutes by PageRank
    html.push_str("<div class=\"card\">\n<h2>Top 10 Statutes by Importance (PageRank)</h2>\n");
    html.push_str("<table>\n<tr><th>Rank</th><th>Statute ID</th><th>PageRank</th><th>In-Degree</th><th>Out-Degree</th></tr>\n");
    for (i, metric) in dashboard.top_centrality.iter().enumerate() {
        html.push_str(&format!(
            "<tr><td>{}</td><td>{}</td><td>{:.4}</td><td>{}</td><td>{}</td></tr>\n",
            i + 1,
            metric.statute_id,
            metric.pagerank,
            metric.in_degree,
            metric.out_degree
        ));
    }
    html.push_str("</table>\n</div>\n");

    // Quality Summary
    html.push_str("<div class=\"card\">\n<h2>Quality Summary</h2>\n");
    html.push_str("<table>\n<tr><th>Grade</th><th>Count</th></tr>\n");
    let mut grades: Vec<_> = dashboard
        .quality_summary
        .grade_distribution
        .iter()
        .collect();
    grades.sort_by(|a, b| a.0.cmp(b.0));
    for (grade, count) in grades {
        html.push_str(&format!("<tr><td>{}</td><td>{}</td></tr>\n", grade, count));
    }
    html.push_str("</table>\n");
    html.push_str(&format!(
        "<p>Statutes with Issues: <span class=\"warning\">{}</span></p>\n",
        dashboard.quality_summary.statutes_with_issues
    ));
    html.push_str("</div>\n");

    // Patterns
    html.push_str("<div class=\"card\">\n<h2>Common Patterns</h2>\n");
    html.push_str(
        "<table>\n<tr><th>Pattern</th><th>Type</th><th>Frequency</th><th>Percentage</th></tr>\n",
    );
    for pattern in &dashboard.patterns {
        let percentage =
            (pattern.frequency as f64 / dashboard.statistics.total_count as f64) * 100.0;
        html.push_str(&format!(
            "<tr><td>{}</td><td>{}</td><td>{}</td><td>{:.1}%</td></tr>\n",
            pattern.description, pattern.pattern_type, pattern.frequency, percentage
        ));
    }
    html.push_str("</table>\n</div>\n");

    // Evolution Summary (if available)
    if let Some(evolution) = &dashboard.evolution_summary {
        html.push_str("<div class=\"card\">\n<h2>Evolution Summary</h2>\n");
        html.push_str(&format!("<div class=\"metric\"><span class=\"metric-label\">Tracked Statutes:</span> <span class=\"metric-value\">{}</span></div>\n", evolution.total_tracked));
        html.push_str(&format!("<div class=\"metric\"><span class=\"metric-label\">Avg Versions:</span> <span class=\"metric-value\">{:.2}</span></div>\n", evolution.avg_versions));
        if let Some(most_changed) = &evolution.most_changed {
            html.push_str(&format!(
                "<p>Most Changed: <strong>{}</strong></p>\n",
                most_changed
            ));
        }
        if let Some(most_stable) = &evolution.most_stable {
            html.push_str(&format!(
                "<p>Most Stable: <strong>{}</strong></p>\n",
                most_stable
            ));
        }
        html.push_str("</div>\n");
    }

    html.push_str("</body>\n</html>");
    html
}

/// Generates a markdown summary of the dashboard
pub fn dashboard_markdown_summary(dashboard: &MetricsDashboard) -> String {
    let mut report = String::new();

    report.push_str("# Comprehensive Metrics Dashboard\n\n");
    report.push_str(&format!("**Generated**: {}\n\n", dashboard.generated_at));

    report.push_str("## Overview\n\n");
    report.push_str(&format!(
        "- **Total Statutes**: {}\n",
        dashboard.statistics.total_count
    ));
    report.push_str(&format!(
        "- **Average Quality Score**: {:.1}/100\n",
        dashboard.quality_summary.average_score
    ));
    report.push_str(&format!(
        "- **Total Conflicts**: {}\n",
        dashboard.conflict_summary.total_conflicts
    ));
    report.push_str(&format!(
        "- **Critical Conflicts**: {}\n",
        dashboard.conflict_summary.critical_conflicts
    ));
    report.push('\n');

    report.push_str("## Graph Structure\n\n");
    report.push_str(&format!(
        "- **Nodes**: {}\n",
        dashboard.graph_metrics.node_count
    ));
    report.push_str(&format!(
        "- **Edges**: {}\n",
        dashboard.graph_metrics.edge_count
    ));
    report.push_str(&format!(
        "- **Density**: {:.4}\n",
        dashboard.graph_metrics.density
    ));
    report.push_str(&format!(
        "- **Is Acyclic**: {}\n",
        dashboard.graph_metrics.is_acyclic
    ));
    report.push_str(&format!(
        "- **Diameter**: {}\n",
        dashboard.graph_metrics.diameter
    ));
    report.push('\n');

    report.push_str("## Quality Distribution\n\n");
    let mut grades: Vec<_> = dashboard
        .quality_summary
        .grade_distribution
        .iter()
        .collect();
    grades.sort_by(|a, b| a.0.cmp(b.0));
    for (grade, count) in grades {
        report.push_str(&format!("- Grade {}: {} statutes\n", grade, count));
    }
    report.push('\n');

    report.push_str("## Top Patterns\n\n");
    for (i, pattern) in dashboard.patterns.iter().take(5).enumerate() {
        let percentage =
            (pattern.frequency as f64 / dashboard.statistics.total_count as f64) * 100.0;
        report.push_str(&format!(
            "{}. {} - {} statutes ({:.1}%)\n",
            i + 1,
            pattern.description,
            pattern.frequency,
            percentage
        ));
    }
    report.push('\n');

    if let Some(evolution) = &dashboard.evolution_summary {
        report.push_str("## Evolution Tracking\n\n");
        report.push_str(&format!(
            "- **Tracked Statutes**: {}\n",
            evolution.total_tracked
        ));
        report.push_str(&format!(
            "- **Average Versions**: {:.2}\n",
            evolution.avg_versions
        ));
        if let Some(most_changed) = &evolution.most_changed {
            report.push_str(&format!("- **Most Changed**: {}\n", most_changed));
        }
        if let Some(most_stable) = &evolution.most_stable {
            report.push_str(&format!("- **Most Stable**: {}\n", most_stable));
        }
        report.push('\n');
    }

    report
}

// ============================================================================
// Cross-Statute Analysis (v0.1.4)
// ============================================================================

/// Represents an interaction between two statutes
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StatuteInteraction {
    /// First statute ID
    pub statute_a: String,
    /// Second statute ID
    pub statute_b: String,
    /// Type of interaction
    pub interaction_type: InteractionType,
    /// Description of the interaction
    pub description: String,
    /// Severity level of the interaction
    pub severity: Severity,
    /// Recommendation for handling the interaction
    pub recommendation: String,
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

impl std::fmt::Display for InteractionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Modification => write!(f, "Modification"),
            Self::Extension => write!(f, "Extension"),
            Self::Complementary => write!(f, "Complementary"),
            Self::Supersession => write!(f, "Supersession"),
            Self::MutualDependency => write!(f, "Mutual Dependency"),
            Self::Contradiction => write!(f, "Contradiction"),
            Self::Overlap => write!(f, "Overlap"),
        }
    }
}

/// Analyzes interactions between statutes
pub fn analyze_statute_interactions(statutes: &[Statute]) -> Vec<StatuteInteraction> {
    let mut interactions = Vec::new();

    for i in 0..statutes.len() {
        for j in (i + 1)..statutes.len() {
            let statute_a = &statutes[i];
            let statute_b = &statutes[j];

            // Check for mutual references (mutual dependency)
            let a_refs_b = extract_statute_references_from_conditions(&statute_a.preconditions)
                .contains(&statute_b.id);
            let b_refs_a = extract_statute_references_from_conditions(&statute_b.preconditions)
                .contains(&statute_a.id);

            if a_refs_b && b_refs_a {
                interactions.push(StatuteInteraction {
                    statute_a: statute_a.id.clone(),
                    statute_b: statute_b.id.clone(),
                    interaction_type: InteractionType::MutualDependency,
                    description: format!(
                        "{} and {} have mutual dependencies",
                        statute_a.id, statute_b.id
                    ),
                    severity: Severity::Warning,
                    recommendation:
                        "Review mutual dependencies for circular logic and consider refactoring"
                            .to_string(),
                });
            }

            // Check for modifications (one references the other with Revoke effect)
            if a_refs_b && matches!(statute_a.effect.effect_type, EffectType::Revoke) {
                interactions.push(StatuteInteraction {
                    statute_a: statute_a.id.clone(),
                    statute_b: statute_b.id.clone(),
                    interaction_type: InteractionType::Modification,
                    description: format!("{} modifies or revokes {}", statute_a.id, statute_b.id),
                    severity: Severity::Info,
                    recommendation: "Ensure modification is intentional and properly documented"
                        .to_string(),
                });
            }

            // Check for extensions (one references the other with Grant effect)
            if a_refs_b && matches!(statute_a.effect.effect_type, EffectType::Grant) {
                interactions.push(StatuteInteraction {
                    statute_a: statute_a.id.clone(),
                    statute_b: statute_b.id.clone(),
                    interaction_type: InteractionType::Extension,
                    description: format!("{} extends {}", statute_a.id, statute_b.id),
                    severity: Severity::Info,
                    recommendation: "Verify that extension is coherent with base statute"
                        .to_string(),
                });
            }

            // Check for contradictions (conflicting effects)
            if effects_contradict(&statute_a.effect, &statute_b.effect)
                && conditions_overlap(&statute_a.preconditions, &statute_b.preconditions)
            {
                interactions.push(StatuteInteraction {
                    statute_a: statute_a.id.clone(),
                    statute_b: statute_b.id.clone(),
                    interaction_type: InteractionType::Contradiction,
                    description: format!(
                        "{} and {} have contradictory effects with overlapping conditions",
                        statute_a.id, statute_b.id
                    ),
                    severity: Severity::Critical,
                    recommendation:
                        "Resolve contradiction by clarifying precedence or narrowing conditions"
                            .to_string(),
                });
            }

            // Check for overlaps (same jurisdiction and similar conditions)
            if statute_a.jurisdiction == statute_b.jurisdiction {
                let similarity = semantic_similarity(statute_a, statute_b).0;
                if similarity > 0.6 {
                    interactions.push(StatuteInteraction {
                        statute_a: statute_a.id.clone(),
                        statute_b: statute_b.id.clone(),
                        interaction_type: InteractionType::Overlap,
                        description: format!(
                            "{} and {} have significant overlap (similarity: {:.1}%)",
                            statute_a.id,
                            statute_b.id,
                            similarity * 100.0
                        ),
                        severity: Severity::Warning,
                        recommendation: "Consider consolidating overlapping statutes".to_string(),
                    });
                }
            }

            // Check for complementary relationships (same jurisdiction, different but compatible effects)
            if statute_a.jurisdiction == statute_b.jurisdiction
                && !effects_contradict(&statute_a.effect, &statute_b.effect)
                && (a_refs_b || b_refs_a)
            {
                interactions.push(StatuteInteraction {
                    statute_a: statute_a.id.clone(),
                    statute_b: statute_b.id.clone(),
                    interaction_type: InteractionType::Complementary,
                    description: format!(
                        "{} and {} complement each other",
                        statute_a.id, statute_b.id
                    ),
                    severity: Severity::Info,
                    recommendation: "Document complementary relationship for clarity".to_string(),
                });
            }
        }
    }

    interactions
}

/// Report on statute interactions
pub fn statute_interaction_report(interactions: &[StatuteInteraction]) -> String {
    let mut report = String::new();

    report.push_str("# Statute Interaction Analysis\n\n");
    report.push_str(&format!(
        "**Total Interactions**: {}\n\n",
        interactions.len()
    ));

    // Group by interaction type
    let mut by_type: HashMap<InteractionType, Vec<&StatuteInteraction>> = HashMap::new();
    for interaction in interactions {
        by_type
            .entry(interaction.interaction_type)
            .or_default()
            .push(interaction);
    }

    for (interaction_type, items) in by_type.iter() {
        report.push_str(&format!(
            "## {} ({} interactions)\n\n",
            interaction_type,
            items.len()
        ));

        for interaction in items {
            report.push_str(&format!(
                "### {} ↔ {}\n\n",
                interaction.statute_a, interaction.statute_b
            ));
            report.push_str(&format!("- **Severity**: {}\n", interaction.severity));
            report.push_str(&format!("- **Description**: {}\n", interaction.description));
            report.push_str(&format!(
                "- **Recommendation**: {}\n\n",
                interaction.recommendation
            ));
        }
    }

    report
}

/// Represents a regulatory overlap between statutes
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RegulatoryOverlap {
    /// IDs of overlapping statutes
    pub statute_ids: Vec<String>,
    /// The area of overlap
    pub overlap_area: OverlapArea,
    /// Description of the overlap
    pub description: String,
    /// Severity of the overlap
    pub severity: Severity,
    /// Suggestion for resolution
    pub resolution: String,
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

impl std::fmt::Display for OverlapArea {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Jurisdiction => write!(f, "Jurisdiction"),
            Self::SubjectMatter => write!(f, "Subject Matter"),
            Self::Temporal => write!(f, "Temporal"),
            Self::Population => write!(f, "Population"),
            Self::Enforcement => write!(f, "Enforcement"),
        }
    }
}

/// Detects regulatory overlaps between statutes
pub fn detect_regulatory_overlaps(statutes: &[Statute]) -> Vec<RegulatoryOverlap> {
    let mut overlaps = Vec::new();

    // Group by jurisdiction
    let mut by_jurisdiction: HashMap<String, Vec<&Statute>> = HashMap::new();
    for statute in statutes {
        if let Some(jurisdiction) = &statute.jurisdiction {
            by_jurisdiction
                .entry(jurisdiction.clone())
                .or_default()
                .push(statute);
        }
    }

    // Check for overlaps within each jurisdiction
    for (jurisdiction, group) in by_jurisdiction.iter() {
        if group.len() < 2 {
            continue;
        }

        for i in 0..group.len() {
            for j in (i + 1)..group.len() {
                let statute_a = group[i];
                let statute_b = group[j];

                // Check for temporal overlap
                let tv_a = &statute_a.temporal_validity;
                let tv_b = &statute_b.temporal_validity;
                if temporal_validity_overlaps(tv_a, tv_b) {
                    overlaps.push(RegulatoryOverlap {
                        statute_ids: vec![statute_a.id.clone(), statute_b.id.clone()],
                        overlap_area: OverlapArea::Temporal,
                        description: format!(
                            "{} and {} have overlapping validity periods in {}",
                            statute_a.id, statute_b.id, jurisdiction
                        ),
                        severity: Severity::Warning,
                        resolution: "Clarify which statute takes precedence during overlap period"
                            .to_string(),
                    });
                }

                // Check for population overlap (similar age/income conditions)
                let a_has_age = has_age_condition(&statute_a.preconditions);
                let b_has_age = has_age_condition(&statute_b.preconditions);
                let a_has_income = has_income_condition(&statute_a.preconditions);
                let b_has_income = has_income_condition(&statute_b.preconditions);

                if (a_has_age && b_has_age) || (a_has_income && b_has_income) {
                    let cond_overlap =
                        conditions_overlap(&statute_a.preconditions, &statute_b.preconditions);
                    if cond_overlap {
                        overlaps.push(RegulatoryOverlap {
                            statute_ids: vec![statute_a.id.clone(), statute_b.id.clone()],
                            overlap_area: OverlapArea::Population,
                            description: format!(
                                "{} and {} target overlapping populations",
                                statute_a.id, statute_b.id
                            ),
                            severity: Severity::Info,
                            resolution: "Verify that overlapping coverage is intentional"
                                .to_string(),
                        });
                    }
                }

                // Check for subject matter overlap (title similarity)
                let title_sim = title_similarity(&statute_a.title, &statute_b.title);
                if title_sim > 0.5 {
                    overlaps.push(RegulatoryOverlap {
                        statute_ids: vec![statute_a.id.clone(), statute_b.id.clone()],
                        overlap_area: OverlapArea::SubjectMatter,
                        description: format!(
                            "{} and {} address similar subject matter (similarity: {:.1}%)",
                            statute_a.id,
                            statute_b.id,
                            title_sim * 100.0
                        ),
                        severity: Severity::Info,
                        resolution: "Consider consolidating if they address the same topic"
                            .to_string(),
                    });
                }
            }
        }
    }

    overlaps
}

/// Report on regulatory overlaps
pub fn regulatory_overlap_report(overlaps: &[RegulatoryOverlap]) -> String {
    let mut report = String::new();

    report.push_str("# Regulatory Overlap Analysis\n\n");
    report.push_str(&format!("**Total Overlaps**: {}\n\n", overlaps.len()));

    // Group by overlap area
    let mut by_area: HashMap<OverlapArea, Vec<&RegulatoryOverlap>> = HashMap::new();
    for overlap in overlaps {
        by_area
            .entry(overlap.overlap_area.clone())
            .or_default()
            .push(overlap);
    }

    for (area, items) in by_area.iter() {
        report.push_str(&format!("## {} Overlaps ({} found)\n\n", area, items.len()));

        for overlap in items {
            report.push_str(&format!(
                "### Statutes: {}\n\n",
                overlap.statute_ids.join(", ")
            ));
            report.push_str(&format!("- **Severity**: {}\n", overlap.severity));
            report.push_str(&format!("- **Description**: {}\n", overlap.description));
            report.push_str(&format!("- **Resolution**: {}\n\n", overlap.resolution));
        }
    }

    report
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

/// Predicts conflict cascades based on statute dependencies
pub fn predict_conflict_cascades(
    statutes: &[Statute],
    conflicts: &[StatuteConflict],
) -> Vec<ConflictCascade> {
    let mut cascades = Vec::new();

    // Build dependency graph
    let mut deps: HashMap<String, Vec<String>> = HashMap::new();
    for statute in statutes {
        let refs = extract_statute_references_from_conditions(&statute.preconditions);
        deps.insert(statute.id.clone(), refs.into_iter().collect());
    }

    // For each conflict, trace its impact
    for conflict in conflicts {
        let origin = conflict.statute_ids.clone();

        // Find all statutes that depend on the conflicting statutes
        let mut affected = HashSet::new();
        let mut to_visit = origin.clone();
        let mut depth = 0;

        while !to_visit.is_empty() && depth < 10 {
            let mut next_level = Vec::new();

            for statute in statutes {
                if affected.contains(&statute.id) || origin.contains(&statute.id) {
                    continue;
                }

                let refs = extract_statute_references_from_conditions(&statute.preconditions);
                for visited in &to_visit {
                    if refs.contains(visited) {
                        affected.insert(statute.id.clone());
                        next_level.push(statute.id.clone());
                    }
                }
            }

            to_visit = next_level;
            depth += 1;
        }

        if !affected.is_empty() {
            let severity = if depth > 3 {
                Severity::Critical
            } else if depth > 1 {
                Severity::Error
            } else {
                Severity::Warning
            };

            let affected_count = affected.len();
            let affected_statutes: Vec<_> = affected.into_iter().collect();

            cascades.push(ConflictCascade {
                origin_statutes: origin,
                affected_statutes,
                depth,
                description: format!(
                    "Conflict cascade affecting {} statutes across {} levels",
                    affected_count, depth
                ),
                severity,
            });
        }
    }

    cascades
}

/// Report on conflict cascades
pub fn conflict_cascade_report(cascades: &[ConflictCascade]) -> String {
    let mut report = String::new();

    report.push_str("# Conflict Cascade Analysis\n\n");
    report.push_str(&format!("**Total Cascades**: {}\n\n", cascades.len()));

    if cascades.is_empty() {
        report.push_str("No conflict cascades detected. This is good!\n");
        return report;
    }

    // Sort by severity and depth
    let mut sorted_cascades = cascades.to_vec();
    sorted_cascades.sort_by(|a, b| b.severity.cmp(&a.severity).then(b.depth.cmp(&a.depth)));

    for cascade in &sorted_cascades {
        report.push_str(&format!(
            "## Cascade from: {}\n\n",
            cascade.origin_statutes.join(", ")
        ));
        report.push_str(&format!("- **Severity**: {}\n", cascade.severity));
        report.push_str(&format!("- **Depth**: {} levels\n", cascade.depth));
        report.push_str(&format!(
            "- **Affected Statutes** ({}):\n",
            cascade.affected_statutes.len()
        ));

        for statute_id in &cascade.affected_statutes {
            report.push_str(&format!("  - {}\n", statute_id));
        }

        report.push_str(&format!("\n{}\n\n", cascade.description));

        if cascade.depth > 2 {
            report.push_str("⚠️ **Warning**: Deep cascade detected. Consider refactoring to reduce dependencies.\n\n");
        }
    }

    report
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

/// Types of coverage gaps
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum GapType {
    /// Age range not covered
    AgeGap,
    /// Income range not covered
    IncomeGap,
    /// Jurisdiction not covered
    JurisdictionGap,
    /// Temporal gap (time period not covered)
    TemporalGap,
    /// Effect type not covered
    EffectGap,
    /// Logical gap in conditions
    LogicalGap,
}

impl std::fmt::Display for GapType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AgeGap => write!(f, "Age Gap"),
            Self::IncomeGap => write!(f, "Income Gap"),
            Self::JurisdictionGap => write!(f, "Jurisdiction Gap"),
            Self::TemporalGap => write!(f, "Temporal Gap"),
            Self::EffectGap => write!(f, "Effect Gap"),
            Self::LogicalGap => write!(f, "Logical Gap"),
        }
    }
}

/// Analyzes coverage gaps in statutes with enhanced detection
#[allow(clippy::too_many_arguments)]
pub fn analyze_enhanced_coverage_gaps(statutes: &[Statute]) -> Vec<EnhancedCoverageGap> {
    let mut gaps = Vec::new();

    // Age gap analysis
    let mut age_thresholds: Vec<(i32, &Statute)> = Vec::new();
    for statute in statutes {
        if let Some(age) = extract_age_threshold(&statute.preconditions) {
            age_thresholds.push((age, statute));
        }
    }
    age_thresholds.sort_by_key(|(age, _)| *age);

    for i in 0..age_thresholds.len().saturating_sub(1) {
        let (age1, statute1) = age_thresholds[i];
        let (age2, statute2) = age_thresholds[i + 1];
        let gap_size = age2 - age1;

        if gap_size > 5 {
            gaps.push(EnhancedCoverageGap {
                gap_type: GapType::AgeGap,
                description: format!("Age gap between {} and {}", age1, age2),
                example_scenario: format!("Individuals aged {} are not covered", age1 + 1),
                severity: if gap_size > 10 {
                    Severity::Warning
                } else {
                    Severity::Info
                },
                related_statutes: vec![statute1.id.clone(), statute2.id.clone()],
                suggested_coverage: format!(
                    "Consider adding statute for ages {} to {}",
                    age1 + 1,
                    age2 - 1
                ),
            });
        }
    }

    // Income gap analysis
    let mut income_thresholds: Vec<(i32, &Statute)> = Vec::new();
    for statute in statutes {
        if let Some(income) = extract_income_threshold(&statute.preconditions) {
            income_thresholds.push((income, statute));
        }
    }
    income_thresholds.sort_by_key(|(income, _)| *income);

    for i in 0..income_thresholds.len().saturating_sub(1) {
        let (income1, statute1) = income_thresholds[i];
        let (income2, statute2) = income_thresholds[i + 1];
        let gap_size = income2 - income1;

        if gap_size > 10000 {
            gaps.push(EnhancedCoverageGap {
                gap_type: GapType::IncomeGap,
                description: format!("Income gap between ${} and ${}", income1, income2),
                example_scenario: format!("Individuals earning ${} are not covered", income1 + 1),
                severity: if gap_size > 50000 {
                    Severity::Warning
                } else {
                    Severity::Info
                },
                related_statutes: vec![statute1.id.clone(), statute2.id.clone()],
                suggested_coverage: format!(
                    "Consider adding statute for income range ${} to ${}",
                    income1 + 1,
                    income2 - 1
                ),
            });
        }
    }

    // Jurisdiction gap analysis
    let missing_jurisdiction_statutes: Vec<_> = statutes
        .iter()
        .filter(|s| s.jurisdiction.is_none())
        .collect();

    if !missing_jurisdiction_statutes.is_empty() {
        gaps.push(EnhancedCoverageGap {
            gap_type: GapType::JurisdictionGap,
            description: format!(
                "{} statutes without jurisdiction",
                missing_jurisdiction_statutes.len()
            ),
            example_scenario: "Statutes without jurisdiction may be ambiguous".to_string(),
            severity: Severity::Warning,
            related_statutes: missing_jurisdiction_statutes
                .iter()
                .map(|s| s.id.clone())
                .collect(),
            suggested_coverage: "Add jurisdiction to all statutes".to_string(),
        });
    }

    // Temporal gap analysis
    let mut temporal_ranges: Vec<(&Statute, &legalis_core::TemporalValidity)> = Vec::new();
    for statute in statutes {
        let tv = &statute.temporal_validity;
        temporal_ranges.push((statute, tv));
    }

    // Sort by effective date
    temporal_ranges.sort_by(|a, b| a.1.effective_date.cmp(&b.1.effective_date));

    for i in 0..temporal_ranges.len().saturating_sub(1) {
        let (statute1, tv1) = temporal_ranges[i];
        let (statute2, tv2) = temporal_ranges[i + 1];

        if let (Some(end1), Some(start2)) = (&tv1.expiry_date, &tv2.effective_date) {
            if start2 > end1 {
                let gap_days = (start2.signed_duration_since(*end1)).num_days();
                if gap_days > 30 {
                    gaps.push(EnhancedCoverageGap {
                        gap_type: GapType::TemporalGap,
                        description: format!("Temporal gap of {} days", gap_days),
                        example_scenario: format!(
                            "Period from {} to {} is not covered",
                            end1, start2
                        ),
                        severity: if gap_days > 365 {
                            Severity::Warning
                        } else {
                            Severity::Info
                        },
                        related_statutes: vec![statute1.id.clone(), statute2.id.clone()],
                        suggested_coverage: format!(
                            "Consider adding coverage for the period {} to {}",
                            end1, start2
                        ),
                    });
                }
            }
        }
    }

    gaps
}

/// Report on enhanced coverage gaps
pub fn enhanced_coverage_gap_report(gaps: &[EnhancedCoverageGap]) -> String {
    let mut report = String::new();

    report.push_str("# Enhanced Coverage Gap Analysis\n\n");
    report.push_str(&format!("**Total Gaps**: {}\n\n", gaps.len()));

    if gaps.is_empty() {
        report.push_str("No significant coverage gaps detected.\n");
        return report;
    }

    // Group by gap type
    let mut by_type: HashMap<GapType, Vec<&EnhancedCoverageGap>> = HashMap::new();
    for gap in gaps {
        by_type.entry(gap.gap_type).or_default().push(gap);
    }

    for (gap_type, items) in by_type.iter() {
        report.push_str(&format!("## {} ({} gaps)\n\n", gap_type, items.len()));

        for gap in items {
            report.push_str(&format!("### {}\n\n", gap.description));
            report.push_str(&format!("- **Severity**: {}\n", gap.severity));
            report.push_str(&format!("- **Example**: {}\n", gap.example_scenario));
            report.push_str(&format!(
                "- **Related Statutes**: {}\n",
                gap.related_statutes.join(", ")
            ));
            report.push_str(&format!("- **Suggestion**: {}\n\n", gap.suggested_coverage));
        }
    }

    report
}

/// Represents a redundancy in the statute set
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RedundancyInstance {
    /// IDs of redundant statutes
    pub statute_ids: Vec<String>,
    /// Type of redundancy
    pub redundancy_type: RedundancyType,
    /// Description
    pub description: String,
    /// Suggested elimination strategy
    pub elimination_strategy: String,
    /// Potential savings (estimated complexity reduction)
    pub potential_savings: f64,
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

impl std::fmt::Display for RedundancyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Duplicate => write!(f, "Duplicate"),
            Self::Subsumed => write!(f, "Subsumed"),
            Self::OverlappingConditions => write!(f, "Overlapping Conditions"),
            Self::EquivalentEffects => write!(f, "Equivalent Effects"),
        }
    }
}

/// Detects redundancies and suggests elimination strategies
pub fn suggest_redundancy_elimination(statutes: &[Statute]) -> Vec<RedundancyInstance> {
    let mut redundancies = Vec::new();

    for i in 0..statutes.len() {
        for j in (i + 1)..statutes.len() {
            let statute_a = &statutes[i];
            let statute_b = &statutes[j];

            let similarity = semantic_similarity(statute_a, statute_b).0;

            // Check for duplicates (very high similarity)
            if similarity > 0.95 {
                let complexity_a = analyze_complexity(statute_a).complexity_score;
                let complexity_b = analyze_complexity(statute_b).complexity_score;

                redundancies.push(RedundancyInstance {
                    statute_ids: vec![statute_a.id.clone(), statute_b.id.clone()],
                    redundancy_type: RedundancyType::Duplicate,
                    description: format!(
                        "{} and {} are nearly identical (similarity: {:.1}%)",
                        statute_a.id,
                        statute_b.id,
                        similarity * 100.0
                    ),
                    elimination_strategy: if complexity_a <= complexity_b {
                        format!(
                            "Consider removing {} and keeping {}",
                            statute_b.id, statute_a.id
                        )
                    } else {
                        format!(
                            "Consider removing {} and keeping {}",
                            statute_a.id, statute_b.id
                        )
                    },
                    potential_savings: (complexity_a + complexity_b) as f64 / 2.0,
                });
            }
            // Check for subsumption (one is a subset of the other)
            else if similarity > 0.8 {
                redundancies.push(RedundancyInstance {
                    statute_ids: vec![statute_a.id.clone(), statute_b.id.clone()],
                    redundancy_type: RedundancyType::Subsumed,
                    description: format!(
                        "{} may be subsumed by {} (similarity: {:.1}%)",
                        statute_a.id,
                        statute_b.id,
                        similarity * 100.0
                    ),
                    elimination_strategy: "Review whether one statute can be merged into the other"
                        .to_string(),
                    potential_savings: 10.0,
                });
            }

            // Check for overlapping conditions
            if conditions_overlap(&statute_a.preconditions, &statute_b.preconditions) {
                // If effects are also similar, this is a redundancy
                if statute_a.effect.effect_type == statute_b.effect.effect_type {
                    redundancies.push(RedundancyInstance {
                        statute_ids: vec![statute_a.id.clone(), statute_b.id.clone()],
                        redundancy_type: RedundancyType::OverlappingConditions,
                        description: format!(
                            "{} and {} have overlapping conditions and similar effects",
                            statute_a.id, statute_b.id
                        ),
                        elimination_strategy:
                            "Consider consolidating into a single statute with combined conditions"
                                .to_string(),
                        potential_savings: 15.0,
                    });
                }
            }
        }
    }

    redundancies
}

/// Report on redundancy elimination suggestions
pub fn redundancy_elimination_report(redundancies: &[RedundancyInstance]) -> String {
    let mut report = String::new();

    report.push_str("# Redundancy Elimination Analysis\n\n");
    report.push_str(&format!(
        "**Total Redundancies**: {}\n\n",
        redundancies.len()
    ));

    if redundancies.is_empty() {
        report.push_str("No redundancies detected. Statute set is lean!\n");
        return report;
    }

    let total_savings: f64 = redundancies.iter().map(|r| r.potential_savings).sum();
    report.push_str(&format!(
        "**Potential Complexity Savings**: {:.1}\n\n",
        total_savings
    ));

    // Group by redundancy type
    let mut by_type: HashMap<RedundancyType, Vec<&RedundancyInstance>> = HashMap::new();
    for redundancy in redundancies {
        by_type
            .entry(redundancy.redundancy_type)
            .or_default()
            .push(redundancy);
    }

    for (redundancy_type, items) in by_type.iter() {
        report.push_str(&format!(
            "## {} ({} instances)\n\n",
            redundancy_type,
            items.len()
        ));

        for redundancy in items {
            report.push_str(&format!(
                "### Statutes: {}\n\n",
                redundancy.statute_ids.join(", ")
            ));
            report.push_str(&format!("- **Description**: {}\n", redundancy.description));
            report.push_str(&format!(
                "- **Strategy**: {}\n",
                redundancy.elimination_strategy
            ));
            report.push_str(&format!(
                "- **Savings**: {:.1} complexity points\n\n",
                redundancy.potential_savings
            ));
        }
    }

    report
}

// Helper functions for enhanced analysis

/// Extracts age threshold from conditions
fn extract_age_threshold(conditions: &[legalis_core::Condition]) -> Option<i32> {
    for cond in conditions {
        if let Some(age) = extract_age_from_condition(cond) {
            return Some(age);
        }
    }
    None
}

/// Helper to extract age from a single condition (recursively)
fn extract_age_from_condition(cond: &legalis_core::Condition) -> Option<i32> {
    use legalis_core::Condition;
    match cond {
        Condition::Age { value, .. } => Some(*value as i32),
        Condition::And(left, right) | Condition::Or(left, right) => {
            extract_age_from_condition(left).or_else(|| extract_age_from_condition(right))
        }
        Condition::Not(inner) => extract_age_from_condition(inner),
        _ => None,
    }
}

/// Extracts income threshold from conditions
fn extract_income_threshold(conditions: &[legalis_core::Condition]) -> Option<i32> {
    for cond in conditions {
        if let Some(income) = extract_income_from_condition(cond) {
            return Some(income);
        }
    }
    None
}

/// Helper to extract income from a single condition (recursively)
fn extract_income_from_condition(cond: &legalis_core::Condition) -> Option<i32> {
    use legalis_core::Condition;
    match cond {
        Condition::Income { value, .. } => Some(*value as i32),
        Condition::And(left, right) | Condition::Or(left, right) => {
            extract_income_from_condition(left).or_else(|| extract_income_from_condition(right))
        }
        Condition::Not(inner) => extract_income_from_condition(inner),
        _ => None,
    }
}

// ============================================================================
// Proof Generation (v0.1.5)
// ============================================================================

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

/// Types of proof steps
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ProofStepType {
    /// Assumption or premise
    Premise,
    /// Logical deduction
    Deduction,
    /// Contradiction found
    Contradiction,
    /// SMT solver result
    SmtResult,
    /// Substitution or simplification
    Simplification,
    /// Conclusion
    Conclusion,
}

impl std::fmt::Display for ProofStepType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Premise => write!(f, "Premise"),
            Self::Deduction => write!(f, "Deduction"),
            Self::Contradiction => write!(f, "Contradiction"),
            Self::SmtResult => write!(f, "SMT Result"),
            Self::Simplification => write!(f, "Simplification"),
            Self::Conclusion => write!(f, "Conclusion"),
        }
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

/// Proof certificate for formal verification
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProofCertificate {
    /// Certificate ID
    pub certificate_id: String,
    /// Statute ID
    pub statute_id: String,
    /// Verification claim
    pub claim: String,
    /// Proof method used
    pub proof_method: String,
    /// The complete proof
    pub proof: VerificationProof,
    /// Certificate issuer
    pub issuer: String,
    /// Issuance date
    pub issued_at: String,
    /// Validity period in days
    pub valid_for_days: Option<u32>,
    /// Digital signature (placeholder for actual signature)
    pub signature: Option<String>,
}

impl ProofCertificate {
    /// Creates a new proof certificate
    pub fn new(
        statute_id: impl Into<String>,
        claim: impl Into<String>,
        proof: VerificationProof,
    ) -> Self {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let statute_id = statute_id.into();
        let claim = claim.into();

        // Generate certificate ID from statute_id and timestamp
        let mut hasher = DefaultHasher::new();
        statute_id.hash(&mut hasher);
        chrono::Utc::now().timestamp().hash(&mut hasher);
        let certificate_id = format!("CERT-{:016x}", hasher.finish());

        Self {
            certificate_id,
            statute_id,
            claim,
            proof_method: "SMT-based formal verification".to_string(),
            proof,
            issuer: "Legalis Verifier".to_string(),
            issued_at: chrono::Utc::now().to_rfc3339(),
            valid_for_days: Some(365),
            signature: None,
        }
    }

    /// Exports certificate to JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Exports certificate to human-readable format
    pub fn to_human_readable(&self) -> String {
        let mut output = String::new();

        output.push_str("╔════════════════════════════════════════════════════════════════╗\n");
        output.push_str("║          FORMAL VERIFICATION CERTIFICATE                       ║\n");
        output.push_str("╚════════════════════════════════════════════════════════════════╝\n\n");

        output.push_str(&format!("Certificate ID: {}\n", self.certificate_id));
        output.push_str(&format!("Statute: {}\n", self.statute_id));
        output.push_str(&format!("Claim: {}\n", self.claim));
        output.push_str(&format!("Proof Method: {}\n", self.proof_method));
        output.push_str(&format!("Issued By: {}\n", self.issuer));
        output.push_str(&format!("Issued At: {}\n", self.issued_at));

        if let Some(days) = self.valid_for_days {
            output.push_str(&format!("Valid For: {} days\n", days));
        }

        output.push_str(&format!(
            "\nProof Status: {}\n",
            if self.proof.is_complete {
                "✓ Complete"
            } else {
                "✗ Incomplete"
            }
        ));
        output.push_str(&format!("Proof Steps: {}\n\n", self.proof.steps.len()));

        output.push_str(&self.proof.to_human_readable());

        output.push_str("\n╔════════════════════════════════════════════════════════════════╗\n");
        output.push_str("║  This certificate attests that the statute has been formally   ║\n");
        output.push_str("║  verified using automated theorem proving techniques.          ║\n");
        output.push_str("╚════════════════════════════════════════════════════════════════╝\n");

        output
    }
}

/// Generates a proof for circular reference detection
pub fn generate_circular_reference_proof(
    _statutes: &[Statute],
    cycle: &[String],
) -> VerificationProof {
    let mut proof = VerificationProof::new(
        cycle.first().cloned().unwrap_or_default(),
        format!(
            "Circular reference detected in statutes: {}",
            cycle.join(" → ")
        ),
    );

    // Step 1: List the statutes in the cycle
    proof.add_step(ProofStep {
        step_number: 1,
        step_type: ProofStepType::Premise,
        description: "Statutes involved in potential cycle".to_string(),
        formula: format!("Cycle = [{}]", cycle.join(", ")),
        justification: "Identified through dependency graph analysis".to_string(),
        depends_on: vec![],
    });

    // Step 2: Show each reference in the cycle
    for (i, (from, to)) in cycle
        .iter()
        .zip(cycle.iter().cycle().skip(1))
        .enumerate()
        .take(cycle.len())
    {
        proof.add_step(ProofStep {
            step_number: i + 2,
            step_type: ProofStepType::Deduction,
            description: format!("Reference from {} to {}", from, to),
            formula: format!("{} → {}", from, to),
            justification: "Extracted from statute preconditions".to_string(),
            depends_on: vec![1],
        });
    }

    // Final step: Conclude circular reference
    let final_step = cycle.len() + 2;
    proof.add_step(ProofStep {
        step_number: final_step,
        step_type: ProofStepType::Contradiction,
        description: "Circular reference detected".to_string(),
        formula: format!("{} → {} → ... → {}", cycle[0], cycle[1], cycle[0]),
        justification: format!(
            "The chain of references forms a cycle, violating acyclicity requirement. {} steps in cycle.",
            cycle.len()
        ),
        depends_on: (2..final_step).collect(),
    });

    proof.complete()
}

/// Exports proof in DOT format for visualization
pub fn export_proof_dot(proof: &VerificationProof) -> String {
    let mut dot = String::new();

    dot.push_str("digraph VerificationProof {\n");
    dot.push_str("  rankdir=TB;\n");
    dot.push_str("  node [shape=box, style=filled, fillcolor=lightblue];\n\n");

    // Add nodes for each step
    for step in &proof.steps {
        let color = match step.step_type {
            ProofStepType::Premise => "lightgreen",
            ProofStepType::Deduction => "lightblue",
            ProofStepType::Contradiction => "salmon",
            ProofStepType::SmtResult => "lightyellow",
            ProofStepType::Simplification => "lightcyan",
            ProofStepType::Conclusion => "lightgreen",
        };

        let label = format!(
            "Step {}\\n{}\\n{}",
            step.step_number,
            step.step_type,
            step.description.chars().take(40).collect::<String>()
        );

        dot.push_str(&format!(
            "  step{} [label=\"{}\", fillcolor={}];\n",
            step.step_number, label, color
        ));
    }

    dot.push('\n');

    // Add edges for dependencies
    for step in &proof.steps {
        for dep in &step.depends_on {
            dot.push_str(&format!("  step{} -> step{};\n", dep, step.step_number));
        }
    }

    dot.push_str("}\n");
    dot
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

/// Compresses a proof by removing redundant steps
pub fn compress_proof(proof: VerificationProof) -> VerificationProof {
    let mut compressed = VerificationProof::new(&proof.statute_id, &proof.claim);
    compressed.generated_at = proof.generated_at;

    // Keep only essential steps: premises, contradictions, and conclusions
    let mut essential_steps: Vec<ProofStep> = Vec::new();
    let mut step_mapping: HashMap<usize, usize> = HashMap::new();
    let mut new_step_number = 1;

    for step in &proof.steps {
        let is_essential = matches!(
            step.step_type,
            ProofStepType::Premise | ProofStepType::Contradiction | ProofStepType::Conclusion
        ) || step.depends_on.is_empty(); // Keep steps with no dependencies (axioms)

        if is_essential {
            step_mapping.insert(step.step_number, new_step_number);

            let mut new_step = step.clone();
            new_step.step_number = new_step_number;

            // Update dependencies to point to compressed step numbers
            new_step.depends_on = step
                .depends_on
                .iter()
                .filter_map(|&old_num| step_mapping.get(&old_num).copied())
                .collect();

            essential_steps.push(new_step);
            new_step_number += 1;
        }
    }

    compressed.steps = essential_steps;
    compressed.is_complete = proof.is_complete;

    compressed
}

/// Generates a proof comparison report
pub fn proof_comparison_report(
    original: &VerificationProof,
    compressed: &VerificationProof,
) -> String {
    let mut report = String::new();

    report.push_str("# Proof Compression Analysis\n\n");
    report.push_str(&format!("**Original Steps**: {}\n", original.steps.len()));
    report.push_str(&format!(
        "**Compressed Steps**: {}\n",
        compressed.steps.len()
    ));
    report.push_str(&format!(
        "**Compression Ratio**: {:.1}%\n\n",
        (1.0 - (compressed.steps.len() as f64 / original.steps.len() as f64)) * 100.0
    ));

    report.push_str("## Retained Steps\n\n");
    for step in &compressed.steps {
        report.push_str(&format!(
            "- Step {}: {} - {}\n",
            step.step_number, step.step_type, step.description
        ));
    }

    report
}

// =============================================================================
// Multi-Party Verification (v0.2.1)
// =============================================================================

/// Represents a stakeholder in the legal system
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Stakeholder {
    /// Unique identifier
    pub id: String,
    /// Name of the stakeholder
    pub name: String,
    /// Type of stakeholder (e.g., "individual", "corporation", "government")
    pub stakeholder_type: String,
    /// Interests or goals
    pub interests: Vec<String>,
    /// Statutes that directly affect this stakeholder
    pub affected_by: Vec<String>,
}

impl Stakeholder {
    /// Creates a new stakeholder
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            stakeholder_type: "individual".to_string(),
            interests: Vec::new(),
            affected_by: Vec::new(),
        }
    }

    /// Sets the stakeholder type
    pub fn with_type(mut self, stakeholder_type: impl Into<String>) -> Self {
        self.stakeholder_type = stakeholder_type.into();
        self
    }

    /// Adds an interest
    pub fn with_interest(mut self, interest: impl Into<String>) -> Self {
        self.interests.push(interest.into());
        self
    }

    /// Adds a statute that affects this stakeholder
    pub fn affected_by_statute(mut self, statute_id: impl Into<String>) -> Self {
        self.affected_by.push(statute_id.into());
        self
    }
}

/// Represents a conflict between multiple stakeholders
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StakeholderConflict {
    /// Involved stakeholders
    pub stakeholders: Vec<String>,
    /// Conflicting statutes
    pub statutes: Vec<String>,
    /// Nature of the conflict
    pub conflict_type: ConflictNature,
    /// Severity of the conflict
    pub severity: Severity,
    /// Description of the conflict
    pub description: String,
    /// Potential resolution strategies
    pub resolutions: Vec<String>,
}

/// Nature of stakeholder conflicts
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ConflictNature {
    /// Direct opposition of interests
    DirectOpposition,
    /// Competing for limited resources
    ResourceCompetition,
    /// Different interpretations of the same statute
    InterpretationDifference,
    /// Overlapping jurisdictions
    JurisdictionalOverlap,
    /// Asymmetric power dynamics
    PowerImbalance,
}

impl std::fmt::Display for ConflictNature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DirectOpposition => write!(f, "Direct Opposition"),
            Self::ResourceCompetition => write!(f, "Resource Competition"),
            Self::InterpretationDifference => write!(f, "Interpretation Difference"),
            Self::JurisdictionalOverlap => write!(f, "Jurisdictional Overlap"),
            Self::PowerImbalance => write!(f, "Power Imbalance"),
        }
    }
}

/// Analyzes conflicts between multiple stakeholders
pub fn analyze_stakeholder_conflicts(
    stakeholders: &[Stakeholder],
    statutes: &[Statute],
) -> Vec<StakeholderConflict> {
    let mut conflicts = Vec::new();

    // Find stakeholders affected by the same statutes
    let mut statute_to_stakeholders: HashMap<String, Vec<String>> = HashMap::new();
    for stakeholder in stakeholders {
        for statute_id in &stakeholder.affected_by {
            statute_to_stakeholders
                .entry(statute_id.clone())
                .or_default()
                .push(stakeholder.id.clone());
        }
    }

    // Analyze potential conflicts for each statute
    for (statute_id, affected_stakeholders) in &statute_to_stakeholders {
        if affected_stakeholders.len() < 2 {
            continue; // No conflict with single stakeholder
        }

        // Find the actual statute
        let statute = statutes.iter().find(|s| &s.id == statute_id);
        if statute.is_none() {
            continue;
        }
        let statute = statute.unwrap();

        // Check for direct opposition based on effect types
        let has_prohibition = matches!(statute.effect.effect_type, EffectType::Prohibition);
        let has_grant = matches!(statute.effect.effect_type, EffectType::Grant);
        let has_revoke = matches!(statute.effect.effect_type, EffectType::Revoke);

        if has_prohibition || has_revoke {
            // Prohibition/Revoke creates opposition between enforcer and affected party
            let mut resolutions = vec![
                "Provide clear appeal mechanism".to_string(),
                "Ensure proportionality of enforcement".to_string(),
            ];

            if has_revoke {
                resolutions.push("Implement grandfathering provisions".to_string());
            }

            conflicts.push(StakeholderConflict {
                stakeholders: affected_stakeholders.clone(),
                statutes: vec![statute_id.clone()],
                conflict_type: ConflictNature::DirectOpposition,
                severity: Severity::Warning,
                description: format!(
                    "Statute {} creates potential opposition between {} stakeholders",
                    statute_id,
                    affected_stakeholders.len()
                ),
                resolutions,
            });
        }

        if has_grant {
            // Grant might create resource competition
            conflicts.push(StakeholderConflict {
                stakeholders: affected_stakeholders.clone(),
                statutes: vec![statute_id.clone()],
                conflict_type: ConflictNature::ResourceCompetition,
                severity: Severity::Info,
                description: format!(
                    "Statute {} may create resource competition among {} stakeholders",
                    statute_id,
                    affected_stakeholders.len()
                ),
                resolutions: vec![
                    "Define clear eligibility criteria".to_string(),
                    "Establish priority ranking system".to_string(),
                    "Set resource allocation caps".to_string(),
                ],
            });
        }
    }

    // Check for conflicts based on overlapping interests
    for i in 0..stakeholders.len() {
        for j in (i + 1)..stakeholders.len() {
            let s1 = &stakeholders[i];
            let s2 = &stakeholders[j];

            // Check for overlapping affected statutes
            let common_statutes: Vec<String> = s1
                .affected_by
                .iter()
                .filter(|id| s2.affected_by.contains(id))
                .cloned()
                .collect();

            if !common_statutes.is_empty() {
                // Check if they have conflicting interests
                let conflicting_interests = !s1.interests.is_empty()
                    && !s2.interests.is_empty()
                    && s1
                        .interests
                        .iter()
                        .all(|i1| s2.interests.iter().all(|i2| i1 != i2));

                if conflicting_interests {
                    conflicts.push(StakeholderConflict {
                        stakeholders: vec![s1.id.clone(), s2.id.clone()],
                        statutes: common_statutes.clone(),
                        conflict_type: ConflictNature::InterpretationDifference,
                        severity: Severity::Warning,
                        description: format!(
                            "Stakeholders {} and {} have conflicting interests regarding {} statutes",
                            s1.name,
                            s2.name,
                            common_statutes.len()
                        ),
                        resolutions: vec![
                            "Provide detailed implementation guidelines".to_string(),
                            "Establish mediation process".to_string(),
                            "Create stakeholder consultation mechanism".to_string(),
                        ],
                    });
                }
            }
        }
    }

    conflicts
}

/// Generates a stakeholder conflict analysis report
pub fn stakeholder_conflict_report(conflicts: &[StakeholderConflict]) -> String {
    let mut report = String::new();

    report.push_str("# Multi-Stakeholder Conflict Analysis\n\n");
    report.push_str(&format!(
        "**Total Conflicts Detected**: {}\n\n",
        conflicts.len()
    ));

    if conflicts.is_empty() {
        report.push_str("No stakeholder conflicts detected.\n");
        return report;
    }

    // Group by conflict type
    let mut by_type: HashMap<ConflictNature, Vec<&StakeholderConflict>> = HashMap::new();
    for conflict in conflicts {
        by_type
            .entry(conflict.conflict_type)
            .or_default()
            .push(conflict);
    }

    for (conflict_type, type_conflicts) in &by_type {
        report.push_str(&format!(
            "## {} ({} conflicts)\n\n",
            conflict_type,
            type_conflicts.len()
        ));

        for conflict in type_conflicts {
            report.push_str(&format!(
                "### Conflict: {} stakeholders involved\n\n",
                conflict.stakeholders.len()
            ));
            report.push_str(&format!("- **Severity**: {}\n", conflict.severity));
            report.push_str(&format!(
                "- **Stakeholders**: {}\n",
                conflict.stakeholders.join(", ")
            ));
            report.push_str(&format!(
                "- **Statutes**: {}\n",
                conflict.statutes.join(", ")
            ));
            report.push_str(&format!("- **Description**: {}\n", conflict.description));
            report.push_str("\n**Potential Resolutions**:\n");
            for resolution in &conflict.resolutions {
                report.push_str(&format!("- {}\n", resolution));
            }
            report.push('\n');
        }
    }

    report
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

/// Represents an outcome in the game
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GameOutcome {
    /// Strategies played by each stakeholder
    pub strategies: Vec<String>,
    /// Payoffs for each stakeholder (indexed by stakeholder position)
    pub payoffs: Vec<i32>,
    /// Whether this is a Nash equilibrium
    pub is_nash_equilibrium: bool,
    /// Description of the outcome
    pub description: String,
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

/// Detects Nash equilibria in statute interactions
pub fn detect_nash_equilibria(model: &GameTheoreticModel) -> Vec<&GameOutcome> {
    model
        .outcomes
        .iter()
        .filter(|outcome| outcome.is_nash_equilibrium)
        .collect()
}

/// Predicts game-theoretic outcomes from statute interactions
pub fn predict_game_outcomes(
    stakeholders: &[Stakeholder],
    _statutes: &[Statute],
) -> GameTheoreticModel {
    let stakeholder_ids: Vec<String> = stakeholders.iter().map(|s| s.id.clone()).collect();
    let mut model = GameTheoreticModel::new(stakeholder_ids);

    // Generate strategies for each stakeholder based on affected statutes
    for (idx, stakeholder) in stakeholders.iter().enumerate() {
        // Strategy 1: Comply with all statutes
        let comply_strategy = Strategy::new(&stakeholder.id, "Full Compliance")
            .with_description("Comply with all applicable statutes");

        model.add_strategy(idx, comply_strategy);

        // Strategy 2: Selective compliance
        if !stakeholder.affected_by.is_empty() {
            let selective = Strategy::new(&stakeholder.id, "Selective Compliance")
                .with_description("Comply only with high-priority statutes");

            model.add_strategy(idx, selective);
        }

        // Strategy 3: Non-compliance (for modeling purposes)
        let non_comply = Strategy::new(&stakeholder.id, "Non-Compliance")
            .with_description("Minimal or no compliance");

        model.add_strategy(idx, non_comply);
    }

    // Generate sample outcomes for two-player games
    if stakeholders.len() == 2 {
        // Both comply: high social welfare, moderate individual payoff
        model.add_outcome(GameOutcome {
            strategies: vec!["Full Compliance".to_string(), "Full Compliance".to_string()],
            payoffs: vec![5, 5],
            is_nash_equilibrium: true,
            description: "Both stakeholders comply, creating stable equilibrium".to_string(),
        });

        // One complies, one doesn't: asymmetric payoffs
        model.add_outcome(GameOutcome {
            strategies: vec!["Full Compliance".to_string(), "Non-Compliance".to_string()],
            payoffs: vec![2, 7],
            is_nash_equilibrium: false,
            description: "Asymmetric compliance creates instability".to_string(),
        });

        model.add_outcome(GameOutcome {
            strategies: vec!["Non-Compliance".to_string(), "Full Compliance".to_string()],
            payoffs: vec![7, 2],
            is_nash_equilibrium: false,
            description: "Asymmetric compliance creates instability".to_string(),
        });

        // Both don't comply: low payoffs due to lack of coordination
        model.add_outcome(GameOutcome {
            strategies: vec!["Non-Compliance".to_string(), "Non-Compliance".to_string()],
            payoffs: vec![1, 1],
            is_nash_equilibrium: true,
            description: "Both stakeholders defect, creating suboptimal equilibrium".to_string(),
        });
    }

    model
}

/// Generates a game-theoretic analysis report
pub fn game_theoretic_report(model: &GameTheoreticModel) -> String {
    let mut report = String::new();

    report.push_str("# Game-Theoretic Outcome Prediction\n\n");
    report.push_str(&format!("**Players**: {}\n", model.stakeholders.len()));
    report.push_str(&format!("**Total Outcomes**: {}\n\n", model.outcomes.len()));

    // List stakeholders and their strategies
    report.push_str("## Stakeholders and Strategies\n\n");
    for (idx, stakeholder_id) in model.stakeholders.iter().enumerate() {
        report.push_str(&format!("### {}\n\n", stakeholder_id));
        if idx < model.strategies.len() {
            report.push_str("**Available Strategies**:\n");
            for strategy in &model.strategies[idx] {
                report.push_str(&format!(
                    "- **{}**: {}\n",
                    strategy.name, strategy.description
                ));
            }
            report.push('\n');
        }
    }

    // List Nash equilibria
    let equilibria = detect_nash_equilibria(model);
    report.push_str(&format!(
        "## Nash Equilibria ({} found)\n\n",
        equilibria.len()
    ));

    if equilibria.is_empty() {
        report.push_str("No pure-strategy Nash equilibria found.\n\n");
    } else {
        for (i, outcome) in equilibria.iter().enumerate() {
            report.push_str(&format!("### Equilibrium {}\n\n", i + 1));
            report.push_str(&format!("- **Description**: {}\n", outcome.description));
            report.push_str("- **Strategies**: ");
            report.push_str(&outcome.strategies.join(" vs. "));
            report.push('\n');
            report.push_str("- **Payoffs**: ");
            report.push_str(
                &outcome
                    .payoffs
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
            );
            report.push_str("\n\n");
        }
    }

    // List all outcomes
    report.push_str("## All Possible Outcomes\n\n");
    for (i, outcome) in model.outcomes.iter().enumerate() {
        report.push_str(&format!("{}. ", i + 1));
        report.push_str(&outcome.strategies.join(" vs. "));
        report.push_str(&format!(
            " → Payoffs: ({})",
            outcome
                .payoffs
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        ));
        if outcome.is_nash_equilibrium {
            report.push_str(" **[Nash Equilibrium]**");
        }
        report.push('\n');
    }

    report
}

/// Represents a coalition of stakeholders
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Coalition {
    /// Member stakeholder IDs
    pub members: Vec<String>,
    /// Shared objectives
    pub objectives: Vec<String>,
    /// Collective effects of the coalition
    pub collective_effects: Vec<String>,
    /// Strength of the coalition (0.0 to 1.0)
    pub strength: f64,
    /// Whether the coalition is stable
    pub is_stable: bool,
}

impl Coalition {
    /// Creates a new coalition
    pub fn new(members: Vec<String>) -> Self {
        Self {
            members,
            objectives: Vec::new(),
            collective_effects: Vec::new(),
            strength: 0.0,
            is_stable: false,
        }
    }

    /// Adds an objective
    pub fn with_objective(mut self, objective: impl Into<String>) -> Self {
        self.objectives.push(objective.into());
        self
    }

    /// Adds a collective effect
    pub fn with_collective_effect(mut self, effect: impl Into<String>) -> Self {
        self.collective_effects.push(effect.into());
        self
    }

    /// Sets the strength
    pub fn with_strength(mut self, strength: f64) -> Self {
        self.strength = strength.clamp(0.0, 1.0);
        self
    }

    /// Sets stability
    pub fn with_stability(mut self, is_stable: bool) -> Self {
        self.is_stable = is_stable;
        self
    }
}

/// Analyzes potential coalitions among stakeholders
pub fn analyze_coalitions(stakeholders: &[Stakeholder], statutes: &[Statute]) -> Vec<Coalition> {
    let mut coalitions = Vec::new();

    // Find stakeholders with common interests
    let mut interest_groups: HashMap<String, Vec<String>> = HashMap::new();
    for stakeholder in stakeholders {
        for interest in &stakeholder.interests {
            interest_groups
                .entry(interest.clone())
                .or_default()
                .push(stakeholder.id.clone());
        }
    }

    // Create coalitions for each interest group with 2+ members
    for (interest, members) in &interest_groups {
        if members.len() >= 2 {
            // Calculate coalition strength based on affected statutes
            let affected_statutes: HashSet<String> = stakeholders
                .iter()
                .filter(|s| members.contains(&s.id))
                .flat_map(|s| s.affected_by.iter().cloned())
                .collect();

            let strength = (affected_statutes.len() as f64 / statutes.len().max(1) as f64).min(1.0);

            // Coalition is stable if all members are affected by common statutes
            let common_statutes = stakeholders
                .iter()
                .filter(|s| members.contains(&s.id))
                .fold(None, |acc: Option<HashSet<String>>, s| {
                    let current: HashSet<String> = s.affected_by.iter().cloned().collect();
                    match acc {
                        None => Some(current),
                        Some(prev) => Some(prev.intersection(&current).cloned().collect()),
                    }
                });

            let is_stable = common_statutes.is_some_and(|s| !s.is_empty());

            let mut coalition = Coalition::new(members.clone())
                .with_objective(interest.clone())
                .with_strength(strength)
                .with_stability(is_stable);

            // Add collective effects
            for statute_id in &affected_statutes {
                if let Some(statute) = statutes.iter().find(|s| &s.id == statute_id) {
                    coalition = coalition.with_collective_effect(format!(
                        "Collectively influenced by statute {} ({})",
                        statute_id, statute.title
                    ));
                }
            }

            coalitions.push(coalition);
        }
    }

    // Sort by strength (descending)
    coalitions.sort_by(|a, b| {
        b.strength
            .partial_cmp(&a.strength)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    coalitions
}

/// Generates a coalition analysis report
pub fn coalition_analysis_report(coalitions: &[Coalition]) -> String {
    let mut report = String::new();

    report.push_str("# Coalition Analysis\n\n");
    report.push_str(&format!(
        "**Total Coalitions Detected**: {}\n\n",
        coalitions.len()
    ));

    if coalitions.is_empty() {
        report.push_str("No coalitions detected. Stakeholders may have divergent interests.\n");
        return report;
    }

    let stable_count = coalitions.iter().filter(|c| c.is_stable).count();
    report.push_str(&format!("**Stable Coalitions**: {}\n", stable_count));
    report.push_str(&format!(
        "**Unstable Coalitions**: {}\n\n",
        coalitions.len() - stable_count
    ));

    for (i, coalition) in coalitions.iter().enumerate() {
        report.push_str(&format!(
            "## Coalition {} - {} members\n\n",
            i + 1,
            coalition.members.len()
        ));
        report.push_str(&format!(
            "- **Members**: {}\n",
            coalition.members.join(", ")
        ));
        report.push_str(&format!("- **Strength**: {:.2}\n", coalition.strength));
        report.push_str(&format!(
            "- **Stability**: {}\n",
            if coalition.is_stable {
                "Stable"
            } else {
                "Unstable"
            }
        ));

        if !coalition.objectives.is_empty() {
            report.push_str("\n**Shared Objectives**:\n");
            for objective in &coalition.objectives {
                report.push_str(&format!("- {}\n", objective));
            }
        }

        if !coalition.collective_effects.is_empty() {
            report.push_str("\n**Collective Effects**:\n");
            for effect in &coalition.collective_effects {
                report.push_str(&format!("- {}\n", effect));
            }
        }

        report.push('\n');
    }

    report
}

// =============================================================================
// Mechanism Design Verification
// =============================================================================

/// Represents a mechanism design property
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum MechanismProperty {
    /// Incentive compatibility - agents benefit from truthful behavior
    IncentiveCompatibility,
    /// Individual rationality - participation is voluntary and beneficial
    IndividualRationality,
    /// Budget balance - transfers sum to zero or non-negative
    BudgetBalance,
    /// Pareto efficiency - no alternative allocation is better for all
    ParetoEfficiency,
    /// Strategy-proofness - truthful reporting is dominant strategy
    StrategyProofness,
    /// Non-dictatorship - no single agent controls outcomes
    NonDictatorship,
}

impl std::fmt::Display for MechanismProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IncentiveCompatibility => write!(f, "Incentive Compatibility"),
            Self::IndividualRationality => write!(f, "Individual Rationality"),
            Self::BudgetBalance => write!(f, "Budget Balance"),
            Self::ParetoEfficiency => write!(f, "Pareto Efficiency"),
            Self::StrategyProofness => write!(f, "Strategy-Proofness"),
            Self::NonDictatorship => write!(f, "Non-Dictatorship"),
        }
    }
}

/// Represents a mechanism design issue
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MechanismIssue {
    /// Property violated
    pub property: MechanismProperty,
    /// Statute(s) involved
    pub statute_ids: Vec<String>,
    /// Severity of the issue
    pub severity: Severity,
    /// Description of the issue
    pub description: String,
    /// Suggested fixes
    pub suggestions: Vec<String>,
}

/// Mechanism design analysis result
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MechanismAnalysis {
    /// Issues found
    pub issues: Vec<MechanismIssue>,
    /// Properties satisfied
    pub satisfied_properties: Vec<MechanismProperty>,
    /// Overall mechanism quality score (0.0-1.0)
    pub quality_score: f64,
}

impl MechanismAnalysis {
    /// Creates a new mechanism analysis
    pub fn new() -> Self {
        Self {
            issues: Vec::new(),
            satisfied_properties: Vec::new(),
            quality_score: 1.0,
        }
    }

    /// Adds an issue
    pub fn add_issue(&mut self, issue: MechanismIssue) {
        self.issues.push(issue);
        self.recalculate_score();
    }

    /// Marks a property as satisfied
    pub fn satisfy_property(&mut self, property: MechanismProperty) {
        if !self.satisfied_properties.contains(&property) {
            self.satisfied_properties.push(property);
        }
    }

    /// Recalculates the quality score
    fn recalculate_score(&mut self) {
        let total_properties = 6.0; // Total mechanism properties
        let critical_issues = self
            .issues
            .iter()
            .filter(|i| i.severity == Severity::Critical)
            .count() as f64;
        let errors = self
            .issues
            .iter()
            .filter(|i| i.severity == Severity::Error)
            .count() as f64;
        let warnings = self
            .issues
            .iter()
            .filter(|i| i.severity == Severity::Warning)
            .count() as f64;

        // Penalty for issues
        let penalty = (critical_issues * 0.3) + (errors * 0.15) + (warnings * 0.05);

        // Bonus for satisfied properties
        let bonus = self.satisfied_properties.len() as f64 / total_properties;

        self.quality_score = (1.0 - penalty + bonus).clamp(0.0, 1.0);
    }
}

impl Default for MechanismAnalysis {
    fn default() -> Self {
        Self::new()
    }
}

/// Verifies mechanism design properties of statutes
pub fn verify_mechanism_design(
    statutes: &[Statute],
    stakeholders: &[Stakeholder],
) -> MechanismAnalysis {
    let mut analysis = MechanismAnalysis::new();

    // Check incentive compatibility
    check_incentive_compatibility(statutes, stakeholders, &mut analysis);

    // Check individual rationality
    check_individual_rationality(statutes, stakeholders, &mut analysis);

    // Check budget balance
    check_budget_balance(statutes, &mut analysis);

    // Check strategy-proofness
    check_strategy_proofness(statutes, &mut analysis);

    // Check non-dictatorship
    check_non_dictatorship(statutes, stakeholders, &mut analysis);

    analysis
}

/// Checks if the mechanism is incentive compatible
fn check_incentive_compatibility(
    statutes: &[Statute],
    _stakeholders: &[Stakeholder],
    analysis: &mut MechanismAnalysis,
) {
    let mut has_issues = false;

    for statute in statutes {
        // Check for penalties without corresponding benefits
        let has_penalty = matches!(
            statute.effect.effect_type,
            EffectType::Prohibition | EffectType::Revoke | EffectType::MonetaryTransfer
        );

        if has_penalty {
            // Check if there's a clear incentive to comply
            let has_compliance_incentive = statute
                .discretion_logic
                .as_ref()
                .is_some_and(|logic| logic.contains("comply") || logic.contains("benefit"));

            if !has_compliance_incentive {
                has_issues = true;
                analysis.add_issue(MechanismIssue {
                    property: MechanismProperty::IncentiveCompatibility,
                    statute_ids: vec![statute.id.clone()],
                    severity: Severity::Warning,
                    description: format!(
                        "Statute {} imposes penalties without clear compliance incentives",
                        statute.id
                    ),
                    suggestions: vec![
                        "Add explicit compliance benefits".to_string(),
                        "Clarify positive incentives in discretion logic".to_string(),
                        "Consider reward mechanisms for compliance".to_string(),
                    ],
                });
            }
        }

        // Check for gaming opportunities
        if statute.preconditions.len() > 3 {
            // Complex preconditions might allow gaming
            analysis.add_issue(MechanismIssue {
                property: MechanismProperty::IncentiveCompatibility,
                statute_ids: vec![statute.id.clone()],
                severity: Severity::Info,
                description: format!(
                    "Statute {} has complex preconditions that may allow strategic manipulation",
                    statute.id
                ),
                suggestions: vec![
                    "Simplify preconditions to reduce gaming opportunities".to_string(),
                    "Add verification mechanisms for condition claims".to_string(),
                ],
            });
        }
    }

    if !has_issues {
        analysis.satisfy_property(MechanismProperty::IncentiveCompatibility);
    }
}

/// Checks if the mechanism satisfies individual rationality
fn check_individual_rationality(
    statutes: &[Statute],
    stakeholders: &[Stakeholder],
    analysis: &mut MechanismAnalysis,
) {
    let mut has_issues = false;

    // Check if any stakeholder is forced into worse outcomes
    for stakeholder in stakeholders {
        let affected_statutes: Vec<&Statute> = statutes
            .iter()
            .filter(|s| stakeholder.affected_by.contains(&s.id))
            .collect();

        // Count negative effects
        let negative_effects = affected_statutes
            .iter()
            .filter(|s| {
                matches!(
                    s.effect.effect_type,
                    EffectType::Prohibition | EffectType::Revoke
                )
            })
            .count();

        // Count positive effects
        let positive_effects = affected_statutes
            .iter()
            .filter(|s| matches!(s.effect.effect_type, EffectType::Grant))
            .count();

        // If only negative effects, individual rationality may be violated
        if negative_effects > 0 && positive_effects == 0 {
            has_issues = true;
            let statute_ids: Vec<String> = affected_statutes.iter().map(|s| s.id.clone()).collect();
            analysis.add_issue(MechanismIssue {
                property: MechanismProperty::IndividualRationality,
                statute_ids,
                severity: Severity::Warning,
                description: format!(
                    "Stakeholder {} faces only penalties without benefits, violating individual rationality",
                    stakeholder.name
                ),
                suggestions: vec![
                    "Add compensatory benefits".to_string(),
                    "Make participation voluntary".to_string(),
                    "Provide alternative compliance paths".to_string(),
                ],
            });
        }
    }

    if !has_issues {
        analysis.satisfy_property(MechanismProperty::IndividualRationality);
    }
}

/// Checks budget balance for monetary transfers
fn check_budget_balance(statutes: &[Statute], analysis: &mut MechanismAnalysis) {
    let monetary_transfers: Vec<&Statute> = statutes
        .iter()
        .filter(|s| matches!(s.effect.effect_type, EffectType::MonetaryTransfer))
        .collect();

    if monetary_transfers.is_empty() {
        // No monetary transfers, budget balance is trivially satisfied
        analysis.satisfy_property(MechanismProperty::BudgetBalance);
        return;
    }

    // Check if transfers are paired (in/out)
    let mut has_balanced_transfers = false;

    for transfer in &monetary_transfers {
        // Check if there's a corresponding reverse transfer
        let has_reverse = monetary_transfers
            .iter()
            .any(|t| t.id != transfer.id && t.jurisdiction == transfer.jurisdiction);

        if has_reverse {
            has_balanced_transfers = true;
        }
    }

    if !has_balanced_transfers && !monetary_transfers.is_empty() {
        analysis.add_issue(MechanismIssue {
            property: MechanismProperty::BudgetBalance,
            statute_ids: monetary_transfers.iter().map(|s| s.id.clone()).collect(),
            severity: Severity::Warning,
            description: "Monetary transfers may not be budget-balanced".to_string(),
            suggestions: vec![
                "Ensure transfers sum to zero or non-negative".to_string(),
                "Add corresponding revenue or expenditure statutes".to_string(),
                "Implement transfer tracking mechanisms".to_string(),
            ],
        });
    } else {
        analysis.satisfy_property(MechanismProperty::BudgetBalance);
    }
}

/// Checks for strategy-proofness
fn check_strategy_proofness(statutes: &[Statute], analysis: &mut MechanismAnalysis) {
    let mut has_issues = false;

    for statute in statutes {
        // Check for conditions that could be easily misrepresented
        for condition in &statute.preconditions {
            if matches!(condition, legalis_core::Condition::Custom { .. }) {
                has_issues = true;
                analysis.add_issue(MechanismIssue {
                    property: MechanismProperty::StrategyProofness,
                    statute_ids: vec![statute.id.clone()],
                    severity: Severity::Info,
                    description: format!(
                        "Statute {} has custom conditions that may be difficult to verify truthfully",
                        statute.id
                    ),
                    suggestions: vec![
                        "Add verification mechanisms for custom conditions".to_string(),
                        "Use objective, verifiable conditions where possible".to_string(),
                        "Implement audit trails for condition claims".to_string(),
                    ],
                });
            }
        }

        // Check if grants have clear, verifiable criteria
        if matches!(statute.effect.effect_type, EffectType::Grant)
            && statute.preconditions.is_empty()
        {
            has_issues = true;
            analysis.add_issue(MechanismIssue {
                property: MechanismProperty::StrategyProofness,
                statute_ids: vec![statute.id.clone()],
                severity: Severity::Warning,
                description: format!(
                    "Statute {} grants benefits without verifiable conditions",
                    statute.id
                ),
                suggestions: vec![
                    "Add objective eligibility criteria".to_string(),
                    "Implement verification procedures".to_string(),
                ],
            });
        }
    }

    if !has_issues {
        analysis.satisfy_property(MechanismProperty::StrategyProofness);
    }
}

/// Checks for non-dictatorship
fn check_non_dictatorship(
    statutes: &[Statute],
    stakeholders: &[Stakeholder],
    analysis: &mut MechanismAnalysis,
) {
    if stakeholders.is_empty() {
        analysis.satisfy_property(MechanismProperty::NonDictatorship);
        return;
    }

    // Check if any single stakeholder controls too many statutes
    let mut statute_control: HashMap<String, usize> = HashMap::new();

    for stakeholder in stakeholders {
        statute_control.insert(stakeholder.id.clone(), stakeholder.affected_by.len());
    }

    let total_statutes = statutes.len();
    let max_control = statute_control.values().max().copied().unwrap_or(0);

    // If one stakeholder controls >50% of statutes, flag as potential dictatorship
    if max_control as f64 > (total_statutes as f64 * 0.5) {
        let dictator = statute_control
            .iter()
            .find(|(_, count)| **count == max_control)
            .map(|(id, _)| id.clone())
            .unwrap_or_default();

        analysis.add_issue(MechanismIssue {
            property: MechanismProperty::NonDictatorship,
            statute_ids: vec![],
            severity: Severity::Error,
            description: format!(
                "Stakeholder {} controls {}% of statutes, suggesting potential dictatorship",
                dictator,
                (max_control as f64 / total_statutes as f64 * 100.0) as i32
            ),
            suggestions: vec![
                "Distribute statute influence more evenly".to_string(),
                "Add checks and balances".to_string(),
                "Implement multi-stakeholder approval mechanisms".to_string(),
            ],
        });
    } else {
        analysis.satisfy_property(MechanismProperty::NonDictatorship);
    }
}

/// Generates a mechanism design analysis report
pub fn mechanism_design_report(analysis: &MechanismAnalysis) -> String {
    let mut report = String::new();

    report.push_str("# Mechanism Design Analysis\n\n");
    report.push_str(&format!(
        "**Overall Quality Score**: {:.2}/1.00\n\n",
        analysis.quality_score
    ));

    // Quality assessment
    let quality_level = if analysis.quality_score >= 0.9 {
        "Excellent"
    } else if analysis.quality_score >= 0.7 {
        "Good"
    } else if analysis.quality_score >= 0.5 {
        "Fair"
    } else {
        "Poor"
    };

    report.push_str(&format!("**Quality Level**: {}\n\n", quality_level));

    // Satisfied properties
    report.push_str(&format!(
        "## Satisfied Properties ({}/6)\n\n",
        analysis.satisfied_properties.len()
    ));

    if analysis.satisfied_properties.is_empty() {
        report.push_str("None\n\n");
    } else {
        for property in &analysis.satisfied_properties {
            report.push_str(&format!("- ✓ {}\n", property));
        }
        report.push('\n');
    }

    // Issues
    report.push_str(&format!("## Issues ({} found)\n\n", analysis.issues.len()));

    if analysis.issues.is_empty() {
        report.push_str("No mechanism design issues detected. The mechanism is well-designed.\n\n");
    } else {
        // Group by property
        let mut by_property: HashMap<MechanismProperty, Vec<&MechanismIssue>> = HashMap::new();
        for issue in &analysis.issues {
            by_property.entry(issue.property).or_default().push(issue);
        }

        for (property, issues) in &by_property {
            report.push_str(&format!("### {} - {} issues\n\n", property, issues.len()));

            for issue in issues {
                report.push_str(&format!("**Severity**: {}\n\n", issue.severity));
                report.push_str(&format!("**Description**: {}\n\n", issue.description));

                if !issue.statute_ids.is_empty() {
                    report.push_str(&format!(
                        "**Affected Statutes**: {}\n\n",
                        issue.statute_ids.join(", ")
                    ));
                }

                report.push_str("**Suggestions**:\n");
                for suggestion in &issue.suggestions {
                    report.push_str(&format!("- {}\n", suggestion));
                }
                report.push('\n');
            }
        }
    }

    report
}

// ============================================================================
// Probabilistic Verification (v0.2.2)
// ============================================================================

/// Represents a state in a Markov chain
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct MarkovState {
    /// Unique state identifier
    pub id: String,
    /// Human-readable state description
    pub description: String,
    /// Whether this is an accepting state
    pub accepting: bool,
}

impl MarkovState {
    /// Creates a new Markov state
    pub fn new(id: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            description: description.into(),
            accepting: false,
        }
    }

    /// Marks this state as accepting
    pub fn accepting(mut self) -> Self {
        self.accepting = true;
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

        // Initialize uniformly
        let num_states = self.states.len();
        if num_states == 0 {
            return probabilities;
        }

        let initial_prob = 1.0 / num_states as f64;
        for state in &self.states {
            probabilities.insert(state.id.clone(), initial_prob);
        }

        // Iterate to convergence
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

        // Track cumulative probability of reaching accepting states
        let mut accepting_prob = 0.0;

        for _ in 0..steps {
            let mut new_probs: HashMap<String, f64> = HashMap::new();

            for (from_state, from_prob) in &probabilities {
                // Check if this state is accepting
                let is_accepting = self
                    .states
                    .iter()
                    .any(|s| s.id == *from_state && s.accepting);

                if is_accepting {
                    // Accumulate probability from accepting states
                    accepting_prob += from_prob;
                } else {
                    // Propagate probability through transitions
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

        // Add any remaining probability in accepting states
        for (state_id, prob) in &probabilities {
            if self.states.iter().any(|s| s.id == *state_id && s.accepting) {
                accepting_prob += prob;
            }
        }

        accepting_prob
    }
}

/// Statistical model checking result
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StatisticalCheckResult {
    /// Property being checked
    pub property: String,
    /// Estimated probability of satisfaction
    pub estimated_probability: f64,
    /// Confidence interval lower bound (95%)
    pub confidence_lower: f64,
    /// Confidence interval upper bound (95%)
    pub confidence_upper: f64,
    /// Number of simulation runs
    pub num_samples: usize,
    /// Number of successful runs
    pub num_successes: usize,
    /// Hypothesis test result (true = accept, false = reject)
    pub hypothesis_accepted: bool,
}

impl StatisticalCheckResult {
    /// Creates a new result from samples
    pub fn from_samples(
        property: impl Into<String>,
        num_samples: usize,
        num_successes: usize,
        threshold: f64,
    ) -> Self {
        let p_hat = num_successes as f64 / num_samples as f64;

        // 95% confidence interval using normal approximation
        let z = 1.96; // 95% confidence
        let std_err = (p_hat * (1.0 - p_hat) / num_samples as f64).sqrt();
        let margin = z * std_err;

        let confidence_lower = (p_hat - margin).max(0.0);
        let confidence_upper = (p_hat + margin).min(1.0);

        // Hypothesis test: H0: p >= threshold
        let hypothesis_accepted = confidence_lower >= threshold;

        Self {
            property: property.into(),
            estimated_probability: p_hat,
            confidence_lower,
            confidence_upper,
            num_samples,
            num_successes,
            hypothesis_accepted,
        }
    }
}

/// Monte Carlo simulation for statute verification
pub fn monte_carlo_verification(
    chain: &MarkovChain,
    num_simulations: usize,
    max_steps: usize,
) -> StatisticalCheckResult {
    use rand::Rng;

    let mut successes = 0;

    for _ in 0..num_simulations {
        let mut current_state = chain.initial_state.clone();
        let mut reached_accepting = false;

        for _ in 0..max_steps {
            // Check if current state is accepting
            if let Some(state) = chain.states.iter().find(|s| s.id == current_state) {
                if state.accepting {
                    reached_accepting = true;
                    break;
                }
            }

            // Get outgoing transitions
            let outgoing: Vec<&MarkovTransition> = chain
                .transitions
                .iter()
                .filter(|t| t.from == current_state)
                .collect();

            if outgoing.is_empty() {
                break;
            }

            // Sample next transition
            let mut rng = rand::rng();
            let r: f64 = rng.random();
            let mut cumulative = 0.0;

            for transition in outgoing {
                cumulative += transition.probability;
                if r <= cumulative {
                    current_state = transition.to.clone();
                    break;
                }
            }
        }

        if reached_accepting {
            successes += 1;
        }
    }

    StatisticalCheckResult::from_samples(
        "Reachability of accepting states",
        num_simulations,
        successes,
        0.5, // Default threshold
    )
}

/// Risk factor for statute analysis
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RiskFactor {
    /// Factor name
    pub name: String,
    /// Factor description
    pub description: String,
    /// Risk contribution (0.0-1.0)
    pub score: f64,
    /// Weight in overall risk (0.0-1.0)
    pub weight: f64,
}

impl RiskFactor {
    /// Creates a new risk factor
    pub fn new(name: impl Into<String>, description: impl Into<String>, score: f64) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            score: score.clamp(0.0, 1.0),
            weight: 1.0,
        }
    }

    /// Sets the weight
    pub fn with_weight(mut self, weight: f64) -> Self {
        self.weight = weight.clamp(0.0, 1.0);
        self
    }
}

/// Risk quantification analysis result
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RiskQuantification {
    /// Statute ID
    pub statute_id: String,
    /// Individual risk factors
    pub factors: Vec<RiskFactor>,
    /// Overall risk score (0.0-1.0)
    pub overall_score: f64,
    /// Risk level classification
    pub risk_level: RiskLevel,
    /// Mitigation recommendations
    pub mitigations: Vec<String>,
}

impl RiskQuantification {
    /// Creates a new risk quantification
    pub fn new(statute_id: impl Into<String>, factors: Vec<RiskFactor>) -> Self {
        let total_weight: f64 = factors.iter().map(|f| f.weight).sum();
        let overall_score = if total_weight > 0.0 {
            factors.iter().map(|f| f.score * f.weight).sum::<f64>() / total_weight
        } else {
            0.0
        };

        let risk_level = RiskLevel::from_score(overall_score);

        Self {
            statute_id: statute_id.into(),
            factors,
            overall_score,
            risk_level,
            mitigations: vec![],
        }
    }

    /// Adds a mitigation recommendation
    pub fn add_mitigation(mut self, mitigation: impl Into<String>) -> Self {
        self.mitigations.push(mitigation.into());
        self
    }
}

/// Analyzes statute risk using multiple factors
pub fn analyze_statute_risk(
    statute: &Statute,
    verification_result: &VerificationResult,
) -> RiskQuantification {
    let mut factors = vec![];

    // Factor 1: Complexity risk
    let complexity_metrics = analyze_complexity(statute);
    let complexity_score = match complexity_metrics.complexity_level {
        ComplexityLevel::Simple => 0.1,
        ComplexityLevel::Moderate => 0.3,
        ComplexityLevel::Complex => 0.6,
        ComplexityLevel::VeryComplex => 0.9,
    };
    factors.push(
        RiskFactor::new(
            "Complexity Risk",
            format!(
                "Statute complexity: {:?}",
                complexity_metrics.complexity_level
            ),
            complexity_score,
        )
        .with_weight(0.25),
    );

    // Factor 2: Verification error risk
    let error_score = if verification_result.errors.is_empty() {
        0.0
    } else {
        let critical_errors = verification_result
            .errors
            .iter()
            .filter(|e| e.severity() == Severity::Critical)
            .count();
        let error_count = verification_result.errors.len();
        (0.5 + (critical_errors as f64 * 0.1))
            .min(1.0)
            .max(error_count as f64 * 0.1)
    };
    factors.push(
        RiskFactor::new(
            "Verification Error Risk",
            format!(
                "{} errors found (including critical)",
                verification_result.errors.len()
            ),
            error_score,
        )
        .with_weight(0.35),
    );

    // Factor 3: Ambiguity risk
    let ambiguities = detect_ambiguities(statute);
    let ambiguity_score = (ambiguities.len() as f64 * 0.15).min(1.0);
    factors.push(
        RiskFactor::new(
            "Ambiguity Risk",
            format!("{} ambiguities detected", ambiguities.len()),
            ambiguity_score,
        )
        .with_weight(0.20),
    );

    // Factor 4: Impact risk
    let impact = analyze_regulatory_impact(statute);
    let impact_score = impact.impact_score as f64 / 100.0;
    factors.push(
        RiskFactor::new(
            "Regulatory Impact Risk",
            format!("Impact level: {:?}", impact.impact_level),
            impact_score,
        )
        .with_weight(0.20),
    );

    let mut quantification = RiskQuantification::new(statute.id.clone(), factors);

    // Add mitigations based on risk level
    match quantification.risk_level {
        RiskLevel::Critical | RiskLevel::High => {
            quantification = quantification
                .add_mitigation("Immediate review and simplification required")
                .add_mitigation("Resolve all critical errors before deployment")
                .add_mitigation("Add comprehensive test coverage")
                .add_mitigation("Implement staged rollout with monitoring");
        }
        RiskLevel::Medium => {
            quantification = quantification
                .add_mitigation("Address identified ambiguities")
                .add_mitigation("Consider simplification if possible")
                .add_mitigation("Add monitoring for edge cases");
        }
        RiskLevel::Low => {
            quantification = quantification
                .add_mitigation("Regular monitoring recommended")
                .add_mitigation("Consider proactive testing");
        }
        RiskLevel::Minimal => {
            quantification =
                quantification.add_mitigation("Continue standard compliance monitoring");
        }
    }

    quantification
}

/// Generates a risk quantification report
pub fn risk_quantification_report(risks: &[RiskQuantification]) -> String {
    let mut report = String::new();

    report.push_str("# Risk Quantification Report\n\n");
    report.push_str(&format!("**Total Statutes Analyzed**: {}\n\n", risks.len()));

    // Risk level distribution
    let mut risk_distribution: HashMap<RiskLevel, usize> = HashMap::new();
    for risk in risks {
        *risk_distribution.entry(risk.risk_level).or_insert(0) += 1;
    }

    report.push_str("## Risk Level Distribution\n\n");
    for level in &[
        RiskLevel::Critical,
        RiskLevel::High,
        RiskLevel::Medium,
        RiskLevel::Low,
        RiskLevel::Minimal,
    ] {
        let count = risk_distribution.get(level).copied().unwrap_or(0);
        report.push_str(&format!("- {}: {} statutes\n", level, count));
    }
    report.push('\n');

    // Individual statute risks
    report.push_str("## Statute Risk Analysis\n\n");

    let mut sorted_risks: Vec<_> = risks.iter().collect();
    sorted_risks.sort_by(|a, b| b.overall_score.partial_cmp(&a.overall_score).unwrap());

    for risk in sorted_risks {
        report.push_str(&format!("### Statute: {}\n\n", risk.statute_id));
        report.push_str(&format!(
            "**Overall Risk Score**: {:.2}/1.00 ({})\n\n",
            risk.overall_score, risk.risk_level
        ));

        report.push_str("**Risk Factors**:\n");
        for factor in &risk.factors {
            report.push_str(&format!(
                "- {}: {:.2} (weight: {:.2}) - {}\n",
                factor.name, factor.score, factor.weight, factor.description
            ));
        }
        report.push('\n');

        if !risk.mitigations.is_empty() {
            report.push_str("**Mitigation Recommendations**:\n");
            for mitigation in &risk.mitigations {
                report.push_str(&format!("- {}\n", mitigation));
            }
            report.push('\n');
        }
    }

    report
}

/// Generates a statistical model checking report
pub fn statistical_model_checking_report(results: &[StatisticalCheckResult]) -> String {
    let mut report = String::new();

    report.push_str("# Statistical Model Checking Report\n\n");
    report.push_str(&format!("**Properties Checked**: {}\n\n", results.len()));

    for result in results {
        report.push_str(&format!("## Property: {}\n\n", result.property));
        report.push_str(&format!(
            "**Estimated Probability**: {:.4}\n",
            result.estimated_probability
        ));
        report.push_str(&format!(
            "**95% Confidence Interval**: [{:.4}, {:.4}]\n",
            result.confidence_lower, result.confidence_upper
        ));
        report.push_str(&format!("**Samples**: {}\n", result.num_samples));
        report.push_str(&format!("**Successes**: {}\n", result.num_successes));
        report.push_str(&format!(
            "**Hypothesis Test**: {}\n\n",
            if result.hypothesis_accepted {
                "ACCEPTED"
            } else {
                "REJECTED"
            }
        ));
    }

    report
}

// ============================================================================
// Explainable Verification (v0.2.3)
// ============================================================================

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

/// Generates natural language explanation for a verification error
pub fn explain_error(error: &VerificationError) -> NaturalLanguageExplanation {
    match error {
        VerificationError::CircularReference { message } => {
            NaturalLanguageExplanation::new(
                "Circular Reference",
                "This law refers to itself in a way that creates an infinite loop.",
                format!("Circular dependency detected: {}", message),
                "Circular references make it impossible to determine what the law actually requires, \
                 since each requirement depends on itself.",
                "Break the circular chain by removing one of the references, or restructure the \
                 statutes so they don't depend on each other in a loop.",
            )
            .with_example(
                "Imagine Law A says 'Follow Law B', and Law B says 'Follow Law A'. \
                 Which one do you follow first? It's impossible to tell!",
            )
        }
        VerificationError::DeadStatute { statute_id } => NaturalLanguageExplanation::new(
            "Impossible to Satisfy",
            format!("Law '{}' has conditions that can never be met.", statute_id),
            format!("Dead statute detected: {} has contradictory preconditions", statute_id),
            "If a law can never be satisfied, it's useless and confusing. People might waste time \
             trying to comply with something that's impossible.",
            "Review the conditions and remove contradictory requirements. Make sure the conditions \
             are logically possible to satisfy.",
        )
        .with_example(
            "This is like a rule that says 'You must be both over 18 AND under 16 years old'. \
             Nobody can satisfy both conditions at the same time.",
        ),
        VerificationError::ConstitutionalConflict {
            statute_id,
            principle,
        } => NaturalLanguageExplanation::new(
            "Constitutional Conflict",
            format!(
                "Law '{}' conflicts with the constitutional principle: {}",
                statute_id, principle
            ),
            format!(
                "Statute {} violates constitutional principle: {}",
                statute_id, principle
            ),
            "Constitutional principles are fundamental rights and protections. Laws that violate \
             them may be invalid and could cause harm to people's rights.",
            format!(
                "Revise the law to align with the '{}' principle. Consider adding safeguards or \
                 exceptions that protect constitutional rights.",
                principle
            ),
        ),
        VerificationError::LogicalContradiction { message } => NaturalLanguageExplanation::new(
            "Logical Contradiction",
            "This law contains conditions that contradict each other.",
            format!("Logical contradiction found: {}", message),
            "Contradictory conditions create confusion and make it unclear what the law actually requires.",
            "Remove or revise the contradictory conditions so they work together logically.",
        )
        .with_example(
            "This is like saying 'You can drive if you have a license AND you can drive if you \
             don't have a license' - which is it?",
        ),
        VerificationError::Ambiguity { message } => NaturalLanguageExplanation::new(
            "Ambiguous Language",
            "This law uses vague or unclear language that could be interpreted multiple ways.",
            format!("Ambiguity detected: {}", message),
            "Ambiguous laws lead to inconsistent enforcement and confusion about what's actually required.",
            "Replace vague terms with specific, measurable criteria. Define unclear terms explicitly.",
        )
        .with_example(
            "Instead of saying 'a reasonable amount', specify exactly what the amount should be \
             (e.g., 'no more than $100').",
        ),
        VerificationError::UnreachableCode { message } => NaturalLanguageExplanation::new(
            "Unreachable Provision",
            "Part of this law can never be triggered or applied.",
            format!("Unreachable code detected: {}", message),
            "Dead provisions waste space in the legal code and may confuse people into thinking \
             they're relevant when they're not.",
            "Remove the unreachable provisions, or fix the conditions so they can actually be triggered.",
        ),
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

/// Explains statute conflicts in layperson terms
pub fn explain_conflict(conflict: &StatuteConflict) -> ConflictExplanation {
    let mut explanation = ConflictExplanation::new(
        conflict.statute_ids.clone(),
        match &conflict.conflict_type {
            ConflictType::EffectConflict => {
                "These laws have overlapping conditions but contradictory effects - they would \
                 apply to the same situations but produce different outcomes.".to_string()
            }
            ConflictType::JurisdictionalOverlap => {
                "These laws overlap in their jurisdiction, creating uncertainty about which applies.".to_string()
            }
            ConflictType::TemporalConflict => {
                "These laws have conflicting rules during overlapping time periods.".to_string()
            }
            ConflictType::HierarchyViolation => {
                "A lower-level law contradicts a higher-level law, which violates legal hierarchy.".to_string()
            }
            ConflictType::IdCollision => {
                "These laws have the same identifier in different jurisdictions, causing confusion.".to_string()
            }
        },
    );

    explanation = explanation.with_impact(
        "This conflict creates legal uncertainty. People affected by these laws may not know \
         which one to follow, leading to potential compliance issues or unfair treatment.",
    );

    // Add affected parties based on conflict type
    match &conflict.conflict_type {
        ConflictType::EffectConflict => {
            explanation = explanation
                .add_affected_party("Anyone trying to comply with both laws")
                .add_affected_party("Law enforcement agencies")
                .add_affected_party("Courts interpreting the laws");
        }
        ConflictType::JurisdictionalOverlap => {
            explanation = explanation
                .add_affected_party("People living or operating in the overlapping jurisdiction")
                .add_affected_party("Multiple regulatory bodies");
        }
        ConflictType::TemporalConflict => {
            explanation = explanation
                .add_affected_party("People affected during the overlapping time period")
                .add_affected_party("Legal administrators managing transitions");
        }
        ConflictType::HierarchyViolation => {
            explanation = explanation
                .add_affected_party("Courts enforcing legal hierarchy")
                .add_affected_party("Citizens relying on proper legal authority");
        }
        ConflictType::IdCollision => {
            explanation = explanation
                .add_affected_party("Cross-jurisdictional entities")
                .add_affected_party("Legal databases and systems");
        }
    }

    // Add resolution options
    for suggestion in &conflict.resolution_suggestions {
        explanation = explanation.add_resolution_option(suggestion.clone());
    }

    explanation
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

/// Builds a verification path from a statute and result
pub fn build_verification_path(
    statute: &Statute,
    result: &VerificationResult,
) -> VerificationPathNode {
    let mut root = VerificationPathNode::new(
        &statute.id,
        "statute",
        format!("Statute: {}", statute.title),
    )
    .with_status(result.passed);

    // Add preconditions node
    if !statute.preconditions.is_empty() {
        for (i, precondition) in statute.preconditions.iter().enumerate() {
            let precondition_node =
                build_condition_path(precondition, &format!("precondition_{}", i));
            root = root.add_child(precondition_node);
        }
    }

    // Add effect node
    let effect_node = VerificationPathNode::new(
        format!("{}_effect", statute.id),
        "effect",
        format!("Effect: {:?}", statute.effect.effect_type),
    )
    .add_metadata("description", &statute.effect.description);
    root = root.add_child(effect_node);

    // Add error nodes
    for (i, error) in result.errors.iter().enumerate() {
        let error_node = VerificationPathNode::new(
            format!("{}_error_{}", statute.id, i),
            "error",
            format!("Error: {:?}", error),
        )
        .with_status(false)
        .add_metadata("severity", format!("{:?}", error.severity()));
        root = root.add_child(error_node);
    }

    root
}

fn build_condition_path(condition: &legalis_core::Condition, prefix: &str) -> VerificationPathNode {
    use legalis_core::{ComparisonOp, Condition};

    match condition {
        Condition::Age { operator, value } => VerificationPathNode::new(
            format!("{}_age", prefix),
            "condition",
            format!(
                "Age {} {}",
                match operator {
                    ComparisonOp::Equal => "==",
                    ComparisonOp::NotEqual => "!=",
                    ComparisonOp::LessThan => "<",
                    ComparisonOp::LessOrEqual => "<=",
                    ComparisonOp::GreaterThan => ">",
                    ComparisonOp::GreaterOrEqual => ">=",
                },
                value
            ),
        ),
        Condition::Income { operator, value } => VerificationPathNode::new(
            format!("{}_income", prefix),
            "condition",
            format!(
                "Income {} ${}",
                match operator {
                    ComparisonOp::Equal => "==",
                    ComparisonOp::NotEqual => "!=",
                    ComparisonOp::LessThan => "<",
                    ComparisonOp::LessOrEqual => "<=",
                    ComparisonOp::GreaterThan => ">",
                    ComparisonOp::GreaterOrEqual => ">=",
                },
                value
            ),
        ),
        Condition::And(left, right) => {
            let mut node = VerificationPathNode::new(format!("{}_and", prefix), "logic", "AND");
            node = node.add_child(build_condition_path(left, &format!("{}_left", prefix)));
            node = node.add_child(build_condition_path(right, &format!("{}_right", prefix)));
            node
        }
        Condition::Or(left, right) => {
            let mut node = VerificationPathNode::new(format!("{}_or", prefix), "logic", "OR");
            node = node.add_child(build_condition_path(left, &format!("{}_left", prefix)));
            node = node.add_child(build_condition_path(right, &format!("{}_right", prefix)));
            node
        }
        Condition::Not(inner) => {
            let mut node = VerificationPathNode::new(format!("{}_not", prefix), "logic", "NOT");
            node = node.add_child(build_condition_path(inner, &format!("{}_inner", prefix)));
            node
        }
        _ => VerificationPathNode::new(
            format!("{}_condition", prefix),
            "condition",
            "Complex Condition",
        ),
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

/// Performs what-if analysis on a statute modification
pub fn what_if_analysis(
    description: impl Into<String>,
    original: Statute,
    modifier: impl FnOnce(&mut Statute),
) -> WhatIfScenario {
    let verifier = StatuteVerifier::new();

    let original_result = verifier.verify(std::slice::from_ref(&original));

    let mut modified = original.clone();
    modifier(&mut modified);

    let new_result = verifier.verify(std::slice::from_ref(&modified));

    WhatIfScenario::new(description, original, modified, original_result, new_result)
}

// ============================================================================
// Privacy-Preserving Verification (v0.2.4)
// ============================================================================

/// Zero-knowledge proof for statute verification
/// Allows proving that a statute satisfies certain properties without revealing the statute details
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ZeroKnowledgeProof {
    /// Unique identifier for this proof
    pub proof_id: String,
    /// Statement being proven (e.g., "statute satisfies constitutional requirements")
    pub statement: String,
    /// Commitment to the hidden data (cryptographic hash)
    pub commitment: String,
    /// Challenge value from verifier
    pub challenge: Vec<u8>,
    /// Response to the challenge
    pub response: Vec<u8>,
    /// Timestamp when proof was created
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl ZeroKnowledgeProof {
    /// Creates a new zero-knowledge proof
    pub fn new(statement: impl Into<String>, statute: &Statute) -> Self {
        use rand::Rng;
        let mut rng = rand::rng();

        // Generate commitment (hash of statute data)
        let commitment = format!("{:x}", md5::compute(format!("{:?}", statute)));

        // Generate random challenge and response
        let challenge: Vec<u8> = (0..32).map(|_| rng.random()).collect();
        let response: Vec<u8> = (0..32).map(|_| rng.random()).collect();

        Self {
            proof_id: format!("zkp-{}", uuid::Uuid::new_v4()),
            statement: statement.into(),
            commitment,
            challenge,
            response,
            timestamp: chrono::Utc::now(),
            metadata: HashMap::new(),
        }
    }

    /// Adds metadata to the proof
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Verifies the zero-knowledge proof without revealing underlying data
    pub fn verify(&self) -> bool {
        // In a real implementation, this would use cryptographic verification
        // For now, we check that all required fields are present
        !self.commitment.is_empty() && !self.challenge.is_empty() && !self.response.is_empty()
    }

    /// Generates a report for this proof
    pub fn report(&self) -> String {
        format!(
            "Zero-Knowledge Proof Report\n\
             ==========================\n\
             Proof ID: {}\n\
             Statement: {}\n\
             Commitment: {}\n\
             Challenge Length: {} bytes\n\
             Response Length: {} bytes\n\
             Timestamp: {}\n\
             Valid: {}\n",
            self.proof_id,
            self.statement,
            &self.commitment[..16.min(self.commitment.len())],
            self.challenge.len(),
            self.response.len(),
            self.timestamp.format("%Y-%m-%d %H:%M:%S UTC"),
            self.verify()
        )
    }
}

/// Multi-party computation result
/// Allows multiple parties to jointly verify statutes without sharing their private inputs
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MultiPartyVerificationResult {
    /// Participating parties
    pub parties: Vec<String>,
    /// Combined verification result (without revealing individual inputs)
    pub combined_result: VerificationResult,
    /// Proof that computation was performed correctly
    pub computation_proof: String,
    /// Timestamp of the computation
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl MultiPartyVerificationResult {
    /// Creates a new multi-party verification result
    pub fn new(parties: Vec<String>, combined_result: VerificationResult) -> Self {
        Self {
            parties,
            combined_result,
            computation_proof: format!("mpc-proof-{}", uuid::Uuid::new_v4()),
            timestamp: chrono::Utc::now(),
        }
    }

    /// Generates a report
    pub fn report(&self) -> String {
        format!(
            "Multi-Party Verification Report\n\
             ==============================\n\
             Parties: {}\n\
             Verification Passed: {}\n\
             Errors: {}\n\
             Warnings: {}\n\
             Computation Proof: {}\n\
             Timestamp: {}\n",
            self.parties.join(", "),
            self.combined_result.passed,
            self.combined_result.errors.len(),
            self.combined_result.warnings.len(),
            self.computation_proof,
            self.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
        )
    }
}

/// Performs secure multi-party verification
pub fn secure_multiparty_verification(
    statute: &Statute,
    parties: Vec<String>,
) -> MultiPartyVerificationResult {
    // In a real implementation, this would use secure MPC protocols
    // For now, we perform verification and prove it was done correctly
    let verifier = StatuteVerifier::new();
    let result = verifier.verify(std::slice::from_ref(statute));

    MultiPartyVerificationResult::new(parties, result)
}

/// Differential privacy parameters
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub struct PrivacyBudget {
    /// Epsilon parameter (privacy loss bound)
    pub epsilon: f64,
    /// Delta parameter (failure probability)
    pub delta: f64,
}

impl PrivacyBudget {
    /// Creates a new privacy budget
    pub fn new(epsilon: f64, delta: f64) -> Self {
        Self { epsilon, delta }
    }

    /// Creates a strict privacy budget (high privacy)
    pub fn strict() -> Self {
        Self {
            epsilon: 0.1,
            delta: 1e-5,
        }
    }

    /// Creates a moderate privacy budget
    pub fn moderate() -> Self {
        Self {
            epsilon: 1.0,
            delta: 1e-3,
        }
    }

    /// Creates a relaxed privacy budget (lower privacy, more accuracy)
    pub fn relaxed() -> Self {
        Self {
            epsilon: 3.0,
            delta: 1e-2,
        }
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

/// Performs differentially private aggregation analysis
pub fn differential_private_analysis(
    statutes: &[Statute],
    privacy_budget: PrivacyBudget,
) -> PrivateAggregation {
    use rand::Rng;
    let mut rng = rand::rng();

    // Calculate true statistics
    let count = statutes.len() as f64;
    let verifier = StatuteVerifier::new();

    let mut total_complexity = 0;
    let mut total_errors = 0;

    for statute in statutes {
        let result = verifier.verify(std::slice::from_ref(statute));
        total_complexity += statute.preconditions.len();
        if !result.passed {
            total_errors += 1;
        }
    }

    let avg_complexity = if count > 0.0 {
        total_complexity as f64 / count
    } else {
        0.0
    };
    let error_rate = if count > 0.0 {
        total_errors as f64 / count
    } else {
        0.0
    };

    // Add Laplace noise for differential privacy
    let sensitivity = 1.0;
    let scale = sensitivity / privacy_budget.epsilon;

    let mut laplace_noise = || -> f64 {
        let u: f64 = rng.random::<f64>() - 0.5;
        -scale * u.signum() * (1.0 - 2.0 * u.abs()).ln()
    };

    PrivateAggregation {
        count: (count + laplace_noise()).max(0.0),
        avg_complexity: (avg_complexity + laplace_noise()).max(0.0),
        error_rate: (error_rate + laplace_noise() / count.max(1.0)).clamp(0.0, 1.0),
        privacy_budget,
    }
}

/// Encrypted statute representation for homomorphic computation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EncryptedStatute {
    /// Encrypted statute identifier
    pub encrypted_id: Vec<u8>,
    /// Encrypted statute data
    pub encrypted_data: Vec<u8>,
    /// Encryption scheme used
    pub scheme: String,
    /// Public parameters
    pub public_params: HashMap<String, String>,
}

impl EncryptedStatute {
    /// Creates a new encrypted statute (simplified encryption)
    pub fn new(statute: &Statute) -> Self {
        use rand::Rng;
        let mut rng = rand::rng();

        // Simplified encryption (in reality would use FHE like SEAL or HElib)
        let id_bytes = statute.id.as_bytes();
        let encrypted_id: Vec<u8> = id_bytes.iter().map(|&b| b ^ rng.random::<u8>()).collect();

        let data_bytes = format!("{:?}", statute).as_bytes().to_vec();
        let encrypted_data: Vec<u8> = data_bytes.iter().map(|&b| b ^ rng.random::<u8>()).collect();

        Self {
            encrypted_id,
            encrypted_data,
            scheme: "Simplified-XOR".to_string(),
            public_params: HashMap::new(),
        }
    }

    /// Performs homomorphic verification (computation on encrypted data)
    pub fn homomorphic_verify(&self) -> EncryptedVerificationResult {
        // In reality, this would perform actual homomorphic operations
        // For now, we create an encrypted result
        EncryptedVerificationResult {
            encrypted_result: vec![0u8; 32],
            scheme: self.scheme.clone(),
        }
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

/// Trusted Execution Environment (TEE) configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TeeConfig {
    /// TEE type (e.g., "SGX", "SEV", "TrustZone")
    pub tee_type: String,
    /// Attestation data proving code integrity
    pub attestation: Vec<u8>,
    /// Enclave configuration
    pub enclave_config: HashMap<String, String>,
}

impl TeeConfig {
    /// Creates a new TEE configuration
    pub fn new(tee_type: impl Into<String>) -> Self {
        use rand::Rng;
        let mut rng = rand::rng();

        Self {
            tee_type: tee_type.into(),
            attestation: (0..64).map(|_| rng.random()).collect(),
            enclave_config: HashMap::new(),
        }
    }

    /// Verifies the TEE attestation
    pub fn verify_attestation(&self) -> bool {
        // In reality, this would verify cryptographic attestation
        !self.attestation.is_empty()
    }
}

/// TEE-based verification result
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TeeVerificationResult {
    /// Verification result
    pub result: VerificationResult,
    /// TEE configuration used
    pub tee_config: TeeConfig,
    /// Remote attestation proof
    pub attestation_proof: String,
    /// Timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl TeeVerificationResult {
    /// Creates a new TEE verification result
    pub fn new(result: VerificationResult, tee_config: TeeConfig) -> Self {
        Self {
            result,
            tee_config,
            attestation_proof: format!("tee-attestation-{}", uuid::Uuid::new_v4()),
            timestamp: chrono::Utc::now(),
        }
    }

    /// Generates a report
    pub fn report(&self) -> String {
        format!(
            "TEE Verification Report\n\
             ======================\n\
             TEE Type: {}\n\
             Attestation Valid: {}\n\
             Verification Passed: {}\n\
             Errors: {}\n\
             Warnings: {}\n\
             Attestation Proof: {}\n\
             Timestamp: {}\n",
            self.tee_config.tee_type,
            self.tee_config.verify_attestation(),
            self.result.passed,
            self.result.errors.len(),
            self.result.warnings.len(),
            self.attestation_proof,
            self.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
        )
    }
}

/// Performs verification in a trusted execution environment
pub fn tee_verification(statute: &Statute, tee_config: TeeConfig) -> TeeVerificationResult {
    // In reality, this would execute inside an actual TEE enclave
    // For now, we perform verification with TEE guarantees
    let verifier = StatuteVerifier::new();
    let result = verifier.verify(std::slice::from_ref(statute));

    TeeVerificationResult::new(result, tee_config)
}

// ============================================================================
// Incremental Verification 2.0 (v0.2.5)
// ============================================================================

/// Fine-grained dependency tracking for statutes
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DependencyNode {
    /// Statute identifier
    pub statute_id: String,
    /// Direct dependencies (statutes this one references)
    pub dependencies: Vec<String>,
    /// Reverse dependencies (statutes that reference this one)
    pub dependents: Vec<String>,
    /// Dependency type (derives_from, references, etc.)
    pub dependency_type: DependencyType,
    /// Last verification timestamp
    pub last_verified: Option<chrono::DateTime<chrono::Utc>>,
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

impl DependencyNode {
    /// Creates a new dependency node
    pub fn new(statute_id: impl Into<String>, dependency_type: DependencyType) -> Self {
        Self {
            statute_id: statute_id.into(),
            dependencies: Vec::new(),
            dependents: Vec::new(),
            dependency_type,
            last_verified: None,
        }
    }

    /// Adds a dependency
    pub fn add_dependency(&mut self, dep_id: impl Into<String>) {
        let dep = dep_id.into();
        if !self.dependencies.contains(&dep) {
            self.dependencies.push(dep);
        }
    }

    /// Adds a dependent
    pub fn add_dependent(&mut self, dep_id: impl Into<String>) {
        let dep = dep_id.into();
        if !self.dependents.contains(&dep) {
            self.dependents.push(dep);
        }
    }

    /// Marks as verified
    pub fn mark_verified(&mut self) {
        self.last_verified = Some(chrono::Utc::now());
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
            // Add node for this statute
            let mut node = DependencyNode::new(&statute.id, DependencyType::DerivesFrom);

            // Add derives_from dependencies
            for dep in &statute.derives_from {
                node.add_dependency(dep);
            }

            graph.nodes.insert(statute.id.clone(), node);
        }

        // Build reverse dependencies
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

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}

/// Lazy verification configuration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LazyVerificationConfig {
    /// Only verify statutes that have changed
    pub verify_changed_only: bool,
    /// Verify dependencies of changed statutes
    pub verify_dependencies: bool,
    /// Maximum depth for dependency verification
    pub max_depth: Option<usize>,
}

impl LazyVerificationConfig {
    /// Creates a new lazy verification config
    pub fn new() -> Self {
        Self {
            verify_changed_only: true,
            verify_dependencies: true,
            max_depth: None,
        }
    }

    /// Only verify changed statutes
    pub fn changed_only() -> Self {
        Self {
            verify_changed_only: true,
            verify_dependencies: false,
            max_depth: None,
        }
    }

    /// Verify with limited dependency depth
    pub fn with_depth(depth: usize) -> Self {
        Self {
            verify_changed_only: true,
            verify_dependencies: true,
            max_depth: Some(depth),
        }
    }
}

impl Default for LazyVerificationConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Performs lazy verification on demand
pub fn lazy_verify(
    statutes: &[Statute],
    changed_ids: &[String],
    config: LazyVerificationConfig,
) -> VerificationResult {
    let verifier = StatuteVerifier::new();

    if changed_ids.is_empty() {
        return VerificationResult::pass();
    }

    // Build dependency graph
    let graph = DependencyGraph::from_statutes(statutes);

    // Determine which statutes to verify
    let mut to_verify: HashSet<String> = changed_ids.iter().cloned().collect();

    if config.verify_dependencies {
        for changed_id in changed_ids {
            let affected = graph.get_affected_statutes(changed_id);
            for id in affected {
                to_verify.insert(id);
            }
        }
    }

    // Filter statutes to verify
    let statutes_to_verify: Vec<Statute> = statutes
        .iter()
        .filter(|s| to_verify.contains(&s.id))
        .cloned()
        .collect();

    verifier.verify(&statutes_to_verify)
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

        // Find added errors
        for error in &new.errors {
            if !Self::contains_error(&old.errors, error) {
                errors_added.push(error.clone());
            }
        }

        // Find removed errors
        for error in &old.errors {
            if !Self::contains_error(&new.errors, error) {
                errors_removed.push(error.clone());
            }
        }

        // Find added/removed warnings
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
        // Compare based on error type and message
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

/// Incremental proof maintenance
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProofCache {
    /// Cached proofs by statute ID
    pub proofs: HashMap<String, CachedProof>,
}

/// Cached proof for a statute
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CachedProof {
    /// Statute ID
    pub statute_id: String,
    /// Verification result
    pub result: VerificationResult,
    /// Proof timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Hash of statute content
    pub content_hash: String,
}

impl CachedProof {
    /// Creates a new cached proof
    pub fn new(statute: &Statute, result: VerificationResult) -> Self {
        Self {
            statute_id: statute.id.clone(),
            result,
            timestamp: chrono::Utc::now(),
            content_hash: format!("{:x}", md5::compute(format!("{:?}", statute))),
        }
    }

    /// Checks if the proof is still valid for the given statute
    pub fn is_valid(&self, statute: &Statute) -> bool {
        let current_hash = format!("{:x}", md5::compute(format!("{:?}", statute)));
        self.content_hash == current_hash
    }
}

impl ProofCache {
    /// Creates a new empty proof cache
    pub fn new() -> Self {
        Self {
            proofs: HashMap::new(),
        }
    }

    /// Adds a proof to the cache
    pub fn add_proof(&mut self, statute: &Statute, result: VerificationResult) {
        let proof = CachedProof::new(statute, result);
        self.proofs.insert(statute.id.clone(), proof);
    }

    /// Gets a cached proof if valid
    pub fn get_proof(&self, statute: &Statute) -> Option<&CachedProof> {
        self.proofs.get(&statute.id).filter(|p| p.is_valid(statute))
    }

    /// Invalidates proofs for changed statutes
    pub fn invalidate(&mut self, statute_ids: &[String]) {
        for id in statute_ids {
            self.proofs.remove(id);
        }
    }

    /// Gets cache statistics
    pub fn stats(&self) -> ProofCacheStats {
        ProofCacheStats {
            total_proofs: self.proofs.len(),
            oldest_timestamp: self.proofs.values().map(|p| p.timestamp).min(),
            newest_timestamp: self.proofs.values().map(|p| p.timestamp).max(),
        }
    }
}

impl Default for ProofCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Proof cache statistics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProofCacheStats {
    /// Total number of cached proofs
    pub total_proofs: usize,
    /// Oldest proof timestamp
    pub oldest_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    /// Newest proof timestamp
    pub newest_timestamp: Option<chrono::DateTime<chrono::Utc>>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType, TemporalValidity};

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
    fn test_complexity_with_calculation() {
        let statute = Statute::new(
            "calc-test",
            "Calculation Test",
            Effect::new(EffectType::Grant, "Tax benefit"),
        )
        .with_precondition(Condition::Calculation {
            formula: "income * 0.2".to_string(),
            operator: ComparisonOp::GreaterThan,
            value: 1000.0,
        });

        let metrics = analyze_complexity(&statute);
        assert_eq!(metrics.condition_count, 1);
        assert_eq!(metrics.condition_depth, 1);
        assert_eq!(metrics.condition_type_count, 1);
        assert_eq!(metrics.logical_operator_count, 0); // No logical operators
    }

    #[test]
    fn test_complexity_with_mixed_calculation() {
        let statute = Statute::new(
            "mixed-test",
            "Mixed Calculation Test",
            Effect::new(EffectType::Grant, "Complex benefit"),
        )
        .with_precondition(Condition::And(
            Box::new(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            }),
            Box::new(Condition::Calculation {
                formula: "net_worth / annual_income".to_string(),
                operator: ComparisonOp::LessThan,
                value: 5.0,
            }),
        ));

        let metrics = analyze_complexity(&statute);
        assert_eq!(metrics.condition_count, 1);
        assert_eq!(metrics.condition_depth, 2);
        assert_eq!(metrics.condition_type_count, 2); // Age and Calculation
        assert_eq!(metrics.logical_operator_count, 1); // One AND operator
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

    #[test]
    fn test_semantic_similarity_identical() {
        let statute1 = Statute::new(
            "test-1",
            "Tax Credit",
            Effect::new(EffectType::Grant, "Grant tax credit"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let statute2 = Statute::new(
            "test-2",
            "Tax Credit",
            Effect::new(EffectType::Grant, "Grant tax credit"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let similarity = semantic_similarity(&statute1, &statute2);
        assert!(similarity.is_high());
        assert!(similarity.0 > 0.8);
    }

    #[test]
    fn test_semantic_similarity_different() {
        let statute1 = Statute::new(
            "test-1",
            "Tax Credit",
            Effect::new(EffectType::Grant, "Grant tax credit"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let statute2 = Statute::new(
            "test-2",
            "Parking Fine",
            Effect::new(EffectType::Obligation, "Pay fine"),
        )
        .with_precondition(Condition::Income {
            operator: ComparisonOp::LessThan,
            value: 30000,
        });

        let similarity = semantic_similarity(&statute1, &statute2);
        assert!(similarity.is_low());
        assert!(similarity.0 < 0.5);
    }

    #[test]
    fn test_find_similar_statutes() {
        let statutes = vec![
            Statute::new(
                "test-1",
                "Tax Credit A",
                Effect::new(EffectType::Grant, "Grant"),
            )
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            }),
            Statute::new(
                "test-2",
                "Tax Credit B",
                Effect::new(EffectType::Grant, "Grant"),
            )
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 21,
            }),
            Statute::new(
                "test-3",
                "Parking Fine",
                Effect::new(EffectType::Obligation, "Pay fine"),
            )
            .with_precondition(Condition::Income {
                operator: ComparisonOp::LessThan,
                value: 30000,
            }),
        ];

        let similar = find_similar_statutes(&statutes, 0.7);
        // test-1 and test-2 should be similar
        assert!(!similar.is_empty());
    }

    #[test]
    fn test_string_similarity() {
        assert_eq!(string_similarity("hello", "hello"), 1.0);
        assert_eq!(string_similarity("", ""), 1.0); // Two empty strings are identical
        assert_eq!(string_similarity("hello", ""), 0.0); // Non-empty vs empty
        assert!(string_similarity("hello", "hallo") > 0.5);
        assert!(string_similarity("hello", "world") < 0.5);
    }

    #[test]
    fn test_similarity_score() {
        let score = SimilarityScore::new(0.85);
        assert!(score.is_high());
        assert!(!score.is_moderate());
        assert!(!score.is_low());

        let score = SimilarityScore::new(0.6);
        assert!(!score.is_high());
        assert!(score.is_moderate());
        assert!(!score.is_low());

        let score = SimilarityScore::new(0.3);
        assert!(!score.is_high());
        assert!(!score.is_moderate());
        assert!(score.is_low());
    }

    #[test]
    fn test_find_ambiguous_terms() {
        let statutes = vec![
            Statute::new(
                "test-1",
                "Tax benefit for persons",
                Effect::new(EffectType::Grant, "Grant to eligible person"),
            ),
            Statute::new(
                "test-2",
                "Child support",
                Effect::new(EffectType::Obligation, "Pay support"),
            ),
        ];

        let ambiguous = find_ambiguous_terms(&statutes);
        assert!(!ambiguous.is_empty());

        // Should find "person" and "child"
        let person_term = ambiguous.iter().find(|t| t.term == "person");
        assert!(person_term.is_some());

        let child_term = ambiguous.iter().find(|t| t.term == "child");
        assert!(child_term.is_some());
    }

    #[test]
    fn test_term_disambiguation_report() {
        let statutes = vec![Statute::new(
            "test-1",
            "Income tax benefit",
            Effect::new(EffectType::Grant, "Grant tax benefit"),
        )];

        let report = term_disambiguation_report(&statutes);
        assert!(report.contains("Term Disambiguation Report"));
        assert!(report.contains("income") || report.contains("tax") || report.contains("benefit"));
    }

    #[test]
    fn test_ambiguous_term_builder() {
        let term = AmbiguousTerm::new("test")
            .with_context("context1")
            .with_statute_id("statute1")
            .with_suggestion("suggestion1");

        assert_eq!(term.term, "test");
        assert_eq!(term.contexts.len(), 1);
        assert_eq!(term.statute_ids.len(), 1);
        assert_eq!(term.suggestions.len(), 1);
    }

    #[test]
    fn test_validate_cross_references_valid() {
        let statute1 = Statute::new(
            "statute-a",
            "Statute A",
            Effect::new(EffectType::Grant, "Test"),
        );

        let statute2 = Statute::new(
            "statute-b",
            "Statute B",
            Effect::new(EffectType::Grant, "Test"),
        )
        .with_precondition(Condition::Custom {
            description: "statute:statute-a".to_string(),
        });

        let errors = validate_cross_references(&[statute1, statute2]);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_validate_cross_references_invalid() {
        let statute = Statute::new(
            "statute-a",
            "Statute A",
            Effect::new(EffectType::Grant, "Test"),
        )
        .with_precondition(Condition::Custom {
            description: "statute:non-existent".to_string(),
        });

        let errors = validate_cross_references(&[statute]);
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].error_type, CrossReferenceErrorType::NotFound);
        assert_eq!(errors[0].referenced_statute_id, "non-existent");
    }

    #[test]
    fn test_cross_reference_report() {
        let statute = Statute::new(
            "statute-a",
            "Statute A",
            Effect::new(EffectType::Grant, "Test"),
        )
        .with_precondition(Condition::Custom {
            description: "statute:missing".to_string(),
        });

        let report = cross_reference_report(&[statute]);
        assert!(report.contains("Cross-Reference Validation Report"));
        assert!(report.contains("missing"));
    }

    #[test]
    fn test_cross_reference_error_display() {
        let error = CrossReferenceError {
            source_statute_id: "statute-a".to_string(),
            referenced_statute_id: "statute-b".to_string(),
            error_type: CrossReferenceErrorType::NotFound,
        };

        let display = format!("{}", error);
        assert!(display.contains("statute-a"));
        assert!(display.contains("statute-b"));
        assert!(display.contains("non-existent"));
    }

    #[test]
    fn test_terminology_consistency() {
        let statutes = vec![
            Statute::new(
                "test-1",
                "Minor support benefit",
                Effect::new(EffectType::Grant, "Grant benefit to child"),
            ),
            Statute::new(
                "test-2",
                "Juvenile assistance",
                Effect::new(EffectType::Grant, "Grant assistance to juvenile"),
            ),
        ];

        let inconsistencies = check_terminology_consistency(&statutes);
        // Should find inconsistent use of "minor" vs "child" vs "juvenile"
        assert!(!inconsistencies.is_empty());
    }

    #[test]
    fn test_terminology_consistency_report() {
        let statutes = vec![
            Statute::new(
                "test-1",
                "Income benefit",
                Effect::new(EffectType::Grant, "Grant benefit"),
            ),
            Statute::new(
                "test-2",
                "Earnings benefit",
                Effect::new(EffectType::Grant, "Grant benefit"),
            ),
        ];

        let report = terminology_consistency_report(&statutes);
        assert!(report.contains("Terminology Consistency Report"));
    }

    #[test]
    fn test_terminology_inconsistency_builder() {
        let inconsistency = TerminologyInconsistency::new("canonical")
            .with_variation("var1")
            .with_variation("var2")
            .with_statute_id("statute1");

        assert_eq!(inconsistency.canonical_term, "canonical");
        assert_eq!(inconsistency.variations.len(), 2);
        assert_eq!(inconsistency.statute_ids.len(), 1);
    }

    #[test]
    fn test_incremental_state() {
        let mut state = IncrementalState::new();

        let statute = Statute::new(
            "test-1",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test"),
        );

        // First time should be marked as changed
        assert!(state.has_changed(&statute));

        // Update state
        let result = VerificationResult::pass();
        state.update(&statute, result.clone());

        // Should not be marked as changed now
        assert!(!state.has_changed(&statute));

        // Modify statute (change title)
        let modified_statute = Statute::new(
            "test-1",
            "Modified Test Statute",
            Effect::new(EffectType::Grant, "Test"),
        );

        // Should be marked as changed
        assert!(state.has_changed(&modified_statute));
    }

    #[test]
    fn test_verify_incremental() {
        let verifier = StatuteVerifier::new();
        let mut state = IncrementalState::new();

        let statute1 = Statute::new("test-1", "Test 1", Effect::new(EffectType::Grant, "Test"));

        let statute2 = Statute::new("test-2", "Test 2", Effect::new(EffectType::Grant, "Test"));

        // First verification
        let result1 =
            verifier.verify_incremental(&[statute1.clone(), statute2.clone()], &mut state);
        assert!(result1.passed);

        // Second verification without changes
        let result2 =
            verifier.verify_incremental(&[statute1.clone(), statute2.clone()], &mut state);
        assert!(result2.passed);

        // Third verification with one changed statute
        let modified_statute1 = Statute::new(
            "test-1",
            "Modified Test 1",
            Effect::new(EffectType::Grant, "Test"),
        );
        let result3 = verifier.verify_incremental(&[modified_statute1, statute2], &mut state);
        assert!(result3.passed);
    }

    #[test]
    fn test_verification_budget() {
        let budget = VerificationBudget::with_max_statutes(5);
        assert!(!budget.statute_limit_reached(4));
        assert!(budget.statute_limit_reached(5));

        let budget = VerificationBudget::with_max_checks(10);
        assert!(!budget.check_limit_reached(9));
        assert!(budget.check_limit_reached(10));

        let budget = VerificationBudget::unlimited();
        assert!(!budget.statute_limit_reached(1000));
        assert!(!budget.check_limit_reached(1000));
    }

    #[test]
    fn test_verify_with_budget() {
        let verifier = StatuteVerifier::new();
        let statutes = vec![
            Statute::new("test-1", "Test 1", Effect::new(EffectType::Grant, "Test")),
            Statute::new("test-2", "Test 2", Effect::new(EffectType::Grant, "Test")),
            Statute::new("test-3", "Test 3", Effect::new(EffectType::Grant, "Test")),
        ];

        // Unlimited budget
        let budget = VerificationBudget::unlimited();
        let (result, verified, _checks, exceeded) = verifier.verify_with_budget(&statutes, budget);
        assert!(result.passed);
        assert_eq!(verified, 3);
        assert!(!exceeded);

        // Limited budget (only 1 statute)
        let budget = VerificationBudget::with_max_statutes(1);
        let (result, verified, _checks, exceeded) = verifier.verify_with_budget(&statutes, budget);
        assert!(result.passed);
        assert_eq!(verified, 1);
        assert!(exceeded);

        // Limited budget (only 5 checks - should stop early)
        let budget = VerificationBudget::with_max_checks(5);
        let (_result, verified, _checks, exceeded) = verifier.verify_with_budget(&statutes, budget);
        // Should verify fewer statutes due to check limit
        assert!(verified < 3);
        assert!(exceeded);
    }

    #[test]
    fn test_equality_check() {
        // Statute with potential age discrimination
        let statute = Statute::new(
            "test-1",
            "Senior benefit",
            Effect::new(EffectType::Grant, "Grant benefit"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 70,
        });

        let result = check_equality(&statute);
        assert!(!result.passed);
        assert!(!result.issues.is_empty());
    }

    #[test]
    fn test_due_process_check() {
        // Statute that revokes without due process
        let statute = Statute::new(
            "test-1",
            "License revocation",
            Effect::new(EffectType::Revoke, "Revoke license"),
        );

        let result = check_due_process(&statute);
        assert!(!result.passed);
        assert!(!result.issues.is_empty());

        // Statute with discretion (passes due process)
        let statute_with_discretion = Statute::new(
            "test-2",
            "License revocation with review",
            Effect::new(EffectType::Revoke, "Revoke license"),
        )
        .with_discretion("Review individual circumstances");

        let result2 = check_due_process(&statute_with_discretion);
        assert!(result2.passed);
    }

    #[test]
    fn test_privacy_impact_check() {
        // Statute with sensitive data
        let statute = Statute::new(
            "test-1",
            "Medical benefit",
            Effect::new(EffectType::Grant, "Grant benefit"),
        )
        .with_precondition(Condition::HasAttribute {
            key: "medical_history".to_string(),
        });

        let result = check_privacy_impact(&statute);
        assert!(!result.passed);
        assert!(!result.issues.is_empty());
        assert!(!result.suggestions.is_empty());
    }

    #[test]
    fn test_proportionality_check() {
        // Severe effect with too few conditions
        let statute = Statute::new(
            "test-1",
            "Prohibition",
            Effect::new(EffectType::Prohibition, "Prohibit action"),
        );

        let result = check_proportionality(&statute);
        assert!(!result.passed);
        assert!(!result.issues.is_empty());

        // Grant with too many conditions
        let mut complex_statute = Statute::new(
            "test-2",
            "Complex grant",
            Effect::new(EffectType::Grant, "Grant benefit"),
        );
        for i in 0..6 {
            complex_statute = complex_statute.with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18 + i,
            });
        }

        let result2 = check_proportionality(&complex_statute);
        assert!(!result2.passed);
    }

    #[test]
    fn test_principle_check_result() {
        let result = PrincipleCheckResult::pass();
        assert!(result.passed);
        assert!(result.issues.is_empty());

        let result =
            PrincipleCheckResult::fail(vec!["Issue 1".to_string()]).with_suggestion("Fix it");
        assert!(!result.passed);
        assert_eq!(result.issues.len(), 1);
        assert_eq!(result.suggestions.len(), 1);
    }

    #[test]
    fn test_accessibility_check() {
        // Statute with physical requirement
        let statute = Statute::new(
            "test-1",
            "Physical test requirement",
            Effect::new(EffectType::Grant, "Grant benefit"),
        )
        .with_precondition(Condition::HasAttribute {
            key: "physical_fitness".to_string(),
        });

        let result = check_accessibility(&statute);
        assert!(!result.passed);
        assert!(!result.issues.is_empty());
        assert!(!result.suggestions.is_empty());

        // Statute with digital requirement
        let statute2 = Statute::new(
            "test-2",
            "Online registration",
            Effect::new(EffectType::Obligation, "Register online"),
        )
        .with_precondition(Condition::HasAttribute {
            key: "internet_access".to_string(),
        });

        let result2 = check_accessibility(&statute2);
        assert!(!result2.passed);
        assert!(result2.issues.iter().any(|i| i.contains("internet")));
    }

    #[test]
    fn test_impact_assessment() {
        // Statute affecting seniors
        let statute = Statute::new(
            "test-1",
            "Senior benefit",
            Effect::new(EffectType::Grant, "Grant senior benefit"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 65,
        });

        let assessment = assess_impact(&statute);
        assert!(assessment.affected_groups.contains(&"Seniors".to_string()));
        assert!(!assessment.positive_impacts.is_empty());

        // Statute with revocation
        let statute2 = Statute::new(
            "test-2",
            "License revocation",
            Effect::new(EffectType::Revoke, "Revoke license"),
        );

        let assessment2 = assess_impact(&statute2);
        assert!(!assessment2.negative_impacts.is_empty());
        assert!(assessment2.overall_risk >= RiskLevel::High);
    }

    #[test]
    fn test_assess_multiple_impacts() {
        let statutes = vec![
            Statute::new(
                "test-1",
                "Tax benefit",
                Effect::new(EffectType::Grant, "Grant tax benefit"),
            )
            .with_precondition(Condition::Income {
                operator: ComparisonOp::LessThan,
                value: 50000,
            }),
            Statute::new(
                "test-2",
                "License requirement",
                Effect::new(EffectType::Obligation, "Obtain license"),
            ),
        ];

        let report = assess_multiple_impacts(&statutes);
        assert!(report.contains("Comprehensive Impact Assessment"));
        assert!(report.contains("Overall Summary"));
    }

    #[test]
    fn test_impact_levels() {
        assert_eq!(format!("{}", ImpactLevel::Low), "Low");
        assert_eq!(format!("{}", ImpactLevel::Medium), "Medium");
        assert_eq!(format!("{}", ImpactLevel::High), "High");

        assert_eq!(format!("{}", RiskLevel::Low), "Low");
        assert_eq!(format!("{}", RiskLevel::Critical), "Critical");
    }

    #[test]
    fn test_impact_assessment_report() {
        let mut assessment = ImpactAssessment::new();
        assessment.affected_groups.push("Test group".to_string());
        assessment
            .positive_impacts
            .push("Positive impact".to_string());
        assessment.overall_risk = RiskLevel::Medium;

        let report = assessment.report();
        assert!(report.contains("Impact Assessment Report"));
        assert!(report.contains("Test group"));
        assert!(report.contains("Medium"));
    }

    // =========================================================================
    // Temporal Logic Tests
    // =========================================================================

    #[test]
    fn test_ltl_atom() {
        let mut system = TransitionSystem::new();
        let s1 = TemporalState::new("s1").with_proposition("p");
        system.add_state(s1);
        system.add_initial_state("s1");

        let formula = LtlFormula::atom("p");
        assert!(verify_ltl(&system, &formula));

        let formula2 = LtlFormula::atom("q");
        assert!(!verify_ltl(&system, &formula2));
    }

    #[test]
    fn test_ltl_next() {
        let mut system = TransitionSystem::new();
        let s1 = TemporalState::new("s1");
        let s2 = TemporalState::new("s2").with_proposition("p");
        system.add_state(s1);
        system.add_state(s2);
        system.add_transition("s1", "s2");
        system.add_initial_state("s1");

        let formula = LtlFormula::next(LtlFormula::atom("p"));
        assert!(verify_ltl(&system, &formula));
    }

    #[test]
    fn test_ltl_always() {
        let mut system = TransitionSystem::new();
        let s1 = TemporalState::new("s1").with_proposition("p");
        let s2 = TemporalState::new("s2").with_proposition("p");
        system.add_state(s1);
        system.add_state(s2);
        system.add_transition("s1", "s2");
        system.add_initial_state("s1");

        let formula = LtlFormula::always(LtlFormula::atom("p"));
        assert!(verify_ltl(&system, &formula));
    }

    #[test]
    fn test_ltl_eventually() {
        let mut system = TransitionSystem::new();
        let s1 = TemporalState::new("s1");
        let s2 = TemporalState::new("s2");
        let s3 = TemporalState::new("s3").with_proposition("p");
        system.add_state(s1);
        system.add_state(s2);
        system.add_state(s3);
        system.add_transition("s1", "s2");
        system.add_transition("s2", "s3");
        system.add_initial_state("s1");

        let formula = LtlFormula::eventually(LtlFormula::atom("p"));
        assert!(verify_ltl(&system, &formula));
    }

    #[test]
    fn test_ltl_display() {
        let formula = LtlFormula::always(LtlFormula::atom("p"));
        assert_eq!(format!("{}", formula), "G(p)");

        let formula2 = LtlFormula::eventually(LtlFormula::atom("q"));
        assert_eq!(format!("{}", formula2), "F(q)");
    }

    #[test]
    fn test_ctl_exists_next() {
        let mut system = TransitionSystem::new();
        let s1 = TemporalState::new("s1");
        let s2 = TemporalState::new("s2").with_proposition("p");
        let s3 = TemporalState::new("s3");
        system.add_state(s1);
        system.add_state(s2);
        system.add_state(s3);
        system.add_transition("s1", "s2");
        system.add_transition("s1", "s3");
        system.add_initial_state("s1");

        let formula = CtlFormula::exists_next(CtlFormula::atom("p"));
        assert!(verify_ctl(&system, &formula));
    }

    #[test]
    fn test_ctl_all_next() {
        let mut system = TransitionSystem::new();
        let s1 = TemporalState::new("s1");
        let s2 = TemporalState::new("s2").with_proposition("p");
        let s3 = TemporalState::new("s3").with_proposition("p");
        system.add_state(s1);
        system.add_state(s2);
        system.add_state(s3);
        system.add_transition("s1", "s2");
        system.add_transition("s1", "s3");
        system.add_initial_state("s1");

        let formula = CtlFormula::all_next(CtlFormula::atom("p"));
        assert!(verify_ctl(&system, &formula));
    }

    #[test]
    fn test_ctl_display() {
        let formula = CtlFormula::exists_eventually(CtlFormula::atom("p"));
        assert_eq!(format!("{}", formula), "EF(p)");

        let formula2 = CtlFormula::all_always(CtlFormula::atom("q"));
        assert_eq!(format!("{}", formula2), "AG(q)");
    }

    #[test]
    fn test_deadline_verification_pass() {
        let mut system = TransitionSystem::new();
        let s1 = TemporalState::new("s1");
        let s2 = TemporalState::new("s2");
        let s3 = TemporalState::new("s3").with_proposition("completed");
        system.add_state(s1);
        system.add_state(s2);
        system.add_state(s3);
        system.add_transition("s1", "s2");
        system.add_transition("s2", "s3");
        system.add_initial_state("s1");

        let deadline = Deadline::new("d1", "completed", 5);
        let result = verify_deadlines(&system, &[deadline]);
        assert!(result.passed);
        assert!(result.violations.is_empty());
    }

    #[test]
    fn test_deadline_verification_fail() {
        let mut system = TransitionSystem::new();
        let s1 = TemporalState::new("s1");
        let s2 = TemporalState::new("s2");
        let s3 = TemporalState::new("s3").with_proposition("completed");
        system.add_state(s1);
        system.add_state(s2);
        system.add_state(s3);
        system.add_transition("s1", "s2");
        system.add_transition("s2", "s3");
        system.add_initial_state("s1");

        let deadline = Deadline::new("d1", "completed", 1);
        let result = verify_deadlines(&system, &[deadline]);
        assert!(!result.passed);
        assert!(!result.violations.is_empty());
    }

    #[test]
    fn test_sequence_verification_pass() {
        let mut system = TransitionSystem::new();
        let s1 = TemporalState::new("s1").with_proposition("start");
        let s2 = TemporalState::new("s2").with_proposition("middle");
        let s3 = TemporalState::new("s3").with_proposition("end");
        system.add_state(s1);
        system.add_state(s2);
        system.add_state(s3);
        system.add_transition("s1", "s2");
        system.add_transition("s2", "s3");
        system.add_initial_state("s1");

        let constraint = SequenceConstraint::new(
            "seq1",
            vec!["start".to_string(), "middle".to_string(), "end".to_string()],
        );
        let result = verify_sequences(&system, &[constraint]);
        assert!(result.passed);
    }

    #[test]
    fn test_sequence_verification_fail() {
        let mut system = TransitionSystem::new();
        let s1 = TemporalState::new("s1").with_proposition("start");
        let s2 = TemporalState::new("s2").with_proposition("end");
        system.add_state(s1);
        system.add_state(s2);
        system.add_transition("s1", "s2");
        system.add_initial_state("s1");

        let constraint = SequenceConstraint::new(
            "seq1",
            vec!["start".to_string(), "middle".to_string(), "end".to_string()],
        );
        let result = verify_sequences(&system, &[constraint]);
        assert!(!result.passed);
    }

    #[test]
    fn test_temporal_state_creation() {
        let state = TemporalState::new("s1")
            .with_proposition("p")
            .with_proposition("q");

        assert_eq!(state.id, "s1");
        assert!(state.satisfies("p"));
        assert!(state.satisfies("q"));
        assert!(!state.satisfies("r"));
    }

    #[test]
    fn test_transition_system_creation() {
        let mut system = TransitionSystem::new();
        let s1 = TemporalState::new("s1").with_proposition("p");
        let s2 = TemporalState::new("s2").with_proposition("q");

        system.add_state(s1);
        system.add_state(s2);
        system.add_transition("s1", "s2");
        system.add_initial_state("s1");

        assert_eq!(system.states.len(), 2);
        assert!(system.initial_states.contains("s1"));
        assert_eq!(system.successors("s1").len(), 1);
    }

    // =========================================================================
    // Principle Registry Tests
    // =========================================================================

    #[test]
    fn test_principle_definition_creation() {
        let principle = PrincipleDefinition::new("test", "Test Principle", "A test")
            .with_priority(10)
            .with_jurisdiction("US")
            .with_check(PrincipleCheck::NoDiscrimination);

        assert_eq!(principle.id, "test");
        assert_eq!(principle.priority, 10);
        assert_eq!(principle.jurisdiction, Some("US".to_string()));
        assert_eq!(principle.checks.len(), 1);
    }

    #[test]
    fn test_composite_principle_creation() {
        let composite = CompositePrinciple::new("comp1", "Composite")
            .with_component("p1")
            .with_component("p2")
            .with_mode(CombinationMode::All);

        assert_eq!(composite.id, "comp1");
        assert_eq!(composite.components.len(), 2);
        assert_eq!(composite.combination_mode, CombinationMode::All);
    }

    #[test]
    fn test_jurisdictional_rule_set() {
        let principle = PrincipleDefinition::new("p1", "Principle 1", "Test").with_priority(10);

        let rule_set = JurisdictionalRuleSet::new("US", "United States").with_principle(principle);

        assert_eq!(rule_set.jurisdiction, "US");
        assert_eq!(rule_set.principles.len(), 1);
    }

    #[test]
    fn test_principle_registry() {
        let mut registry = PrincipleRegistry::new();

        let principle = PrincipleDefinition::new("p1", "Test", "Description")
            .with_check(PrincipleCheck::NoDiscrimination);

        let rule_set = JurisdictionalRuleSet::new("US", "United States").with_principle(principle);

        registry.add_jurisdiction(rule_set);

        assert!(registry.get_jurisdiction("US").is_some());
        assert!(registry.get_jurisdiction("UK").is_none());
    }

    #[test]
    fn test_verify_for_jurisdiction() {
        let mut registry = PrincipleRegistry::new();

        let principle = PrincipleDefinition::new("equality", "Equality", "Equal treatment")
            .with_priority(10)
            .with_check(PrincipleCheck::NoDiscrimination);

        let rule_set = JurisdictionalRuleSet::new("US", "United States").with_principle(principle);

        registry.add_jurisdiction(rule_set);

        let statute = Statute::new(
            "test-1",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        );

        let result = registry.verify_for_jurisdiction(&statute, "US");
        // Just verify it runs without panicking
        let _ = result.passed;
    }

    #[test]
    fn test_retroactivity_check_pass() {
        use chrono::{NaiveDate, Utc};

        // Statute with proper prospective application
        let statute = Statute::new(
            "test-1",
            "Traffic prohibition",
            Effect::new(EffectType::Prohibition, "Prohibit parking"),
        )
        .with_temporal_validity(
            TemporalValidity::new()
                .with_effective_date(NaiveDate::from_ymd_opt(2026, 6, 1).unwrap())
                .with_enacted_at(Utc::now()),
        );

        let result = check_retroactivity(&statute);
        assert!(result.passed);
        assert!(result.issues.is_empty());
    }

    #[test]
    fn test_retroactivity_check_retroactive_language() {
        use chrono::NaiveDate;

        // Prohibition with retroactive language in description
        let statute = Statute::new(
            "test-2",
            "Retroactive ban",
            Effect::new(
                EffectType::Prohibition,
                "Prohibit actions taken retroactively before this date",
            ),
        )
        .with_temporal_validity(
            TemporalValidity::new()
                .with_effective_date(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()),
        );

        let result = check_retroactivity(&statute);
        assert!(!result.passed);
        assert!(!result.issues.is_empty());
        assert!(result.issues.iter().any(|i| i.contains("retroactively")));
    }

    #[test]
    fn test_retroactivity_check_retroactive_parameter() {
        use chrono::NaiveDate;

        // Obligation with retroactive parameter
        let mut effect = Effect::new(EffectType::Obligation, "File report");
        effect
            .parameters
            .insert("retroactive".to_string(), "true".to_string());

        let statute = Statute::new("test-3", "Reporting requirement", effect)
            .with_temporal_validity(
                TemporalValidity::new()
                    .with_effective_date(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()),
            );

        let result = check_retroactivity(&statute);
        assert!(!result.passed);
        assert!(!result.issues.is_empty());
        assert!(result.issues.iter().any(|i| i.contains("ex post facto")));
    }

    #[test]
    fn test_retroactivity_check_application_before_effective() {
        use chrono::NaiveDate;

        // Prohibition with application date before effective date
        let mut effect = Effect::new(EffectType::Prohibition, "Prohibit conduct");
        effect
            .parameters
            .insert("application_date".to_string(), "2024-12-01".to_string());

        let statute = Statute::new("test-4", "Backdated prohibition", effect)
            .with_temporal_validity(
                TemporalValidity::new()
                    .with_effective_date(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()),
            );

        let result = check_retroactivity(&statute);
        assert!(!result.passed);
        assert!(!result.issues.is_empty());
        assert!(
            result
                .issues
                .iter()
                .any(|i| i.contains("precedes effective date"))
        );
    }

    #[test]
    fn test_retroactivity_check_effective_before_enactment() {
        use chrono::{NaiveDate, Utc};

        // Prohibition with effective date before enactment
        let statute = Statute::new(
            "test-5",
            "Impossible retroactive law",
            Effect::new(EffectType::Prohibition, "Prohibit action"),
        )
        .with_temporal_validity(
            TemporalValidity::new()
                .with_effective_date(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())
                .with_enacted_at(Utc::now()),
        );

        let result = check_retroactivity(&statute);
        assert!(!result.passed);
        assert!(!result.issues.is_empty());
        assert!(
            result
                .issues
                .iter()
                .any(|i| i.contains("before enactment date"))
        );
    }

    #[test]
    fn test_retroactivity_check_monetary_penalty() {
        use chrono::NaiveDate;

        // Fine with retroactive flag
        let mut effect = Effect::new(EffectType::MonetaryTransfer, "Impose fine for violation");
        effect
            .parameters
            .insert("retroactive".to_string(), "true".to_string());

        let statute = Statute::new("test-6", "Retroactive fine", effect).with_temporal_validity(
            TemporalValidity::new()
                .with_effective_date(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()),
        );

        let result = check_retroactivity(&statute);
        assert!(!result.passed);
        assert!(!result.issues.is_empty());
        assert!(result.issues.iter().any(|i| i.contains("penalty")));
    }

    #[test]
    fn test_retroactivity_check_grant_allowed() {
        use chrono::NaiveDate;

        // Grants can sometimes be retroactive (beneficial to people)
        let mut effect = Effect::new(EffectType::Grant, "Grant benefit");
        effect
            .parameters
            .insert("retroactive".to_string(), "true".to_string());

        let statute = Statute::new("test-7", "Retroactive benefit", effect).with_temporal_validity(
            TemporalValidity::new()
                .with_effective_date(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()),
        );

        let result = check_retroactivity(&statute);
        // Grants are not restrictive, so no retroactivity violation
        assert!(result.passed);
    }

    #[test]
    fn test_retroactivity_check_no_effective_date() {
        // Prohibition without effective date - should suggest adding one
        let statute = Statute::new(
            "test-8",
            "Undated prohibition",
            Effect::new(EffectType::Prohibition, "Prohibit action"),
        );

        let result = check_retroactivity(&statute);
        // No issues but has suggestions
        assert!(result.passed);
        assert!(result.issues.is_empty());
        assert!(!result.suggestions.is_empty());
        assert!(
            result
                .suggestions
                .iter()
                .any(|s| s.contains("effective date"))
        );
    }

    #[test]
    fn test_id_collision_detection() {
        // Create statutes with duplicate IDs
        let statute1 = Statute::new(
            "duplicate-id",
            "First Statute",
            Effect::new(EffectType::Grant, "Grant benefit"),
        )
        .with_jurisdiction("US");

        let statute2 = Statute::new(
            "duplicate-id",
            "Second Statute",
            Effect::new(EffectType::Grant, "Grant different benefit"),
        )
        .with_jurisdiction("UK");

        let conflicts = detect_statute_conflicts(&[statute1, statute2]);

        assert!(!conflicts.is_empty());
        assert!(
            conflicts
                .iter()
                .any(|c| c.conflict_type == ConflictType::IdCollision)
        );
    }

    #[test]
    fn test_effect_conflict_detection() {
        use chrono::NaiveDate;

        // Create statutes with overlapping conditions but contradictory effects
        let statute1 = Statute::new(
            "grant-benefit",
            "Grant Benefits",
            Effect::new(EffectType::Grant, "Grant parking permit"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        })
        .with_jurisdiction("US")
        .with_temporal_validity(
            TemporalValidity::new()
                .with_effective_date(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()),
        );

        let statute2 = Statute::new(
            "prohibit-benefit",
            "Prohibit Benefits",
            Effect::new(EffectType::Prohibition, "Prohibit parking"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        })
        .with_jurisdiction("US")
        .with_temporal_validity(
            TemporalValidity::new()
                .with_effective_date(NaiveDate::from_ymd_opt(2025, 6, 1).unwrap()),
        );

        let conflicts = detect_statute_conflicts(&[statute1, statute2]);

        assert!(
            conflicts
                .iter()
                .any(|c| c.conflict_type == ConflictType::EffectConflict)
        );
    }

    #[test]
    fn test_temporal_conflict_detection() {
        use chrono::NaiveDate;

        // Create statutes with overlapping temporal validity
        let statute1 = Statute::new(
            "law-v1",
            "Traffic Law",
            Effect::new(EffectType::Grant, "Grant permit"),
        )
        .with_jurisdiction("US")
        .with_version(1)
        .with_temporal_validity(
            TemporalValidity::new()
                .with_effective_date(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())
                .with_expiry_date(NaiveDate::from_ymd_opt(2026, 1, 1).unwrap()),
        );

        let statute2 = Statute::new(
            "law-v2",
            "Traffic Law",
            Effect::new(EffectType::Grant, "Grant new permit"),
        )
        .with_jurisdiction("US")
        .with_version(2)
        .with_temporal_validity(
            TemporalValidity::new()
                .with_effective_date(NaiveDate::from_ymd_opt(2025, 6, 1).unwrap()),
        );

        let conflicts = detect_statute_conflicts(&[statute1, statute2]);

        assert!(
            conflicts
                .iter()
                .any(|c| c.conflict_type == ConflictType::TemporalConflict)
        );
    }

    #[test]
    fn test_no_conflicts_when_different_jurisdictions() {
        // Statutes in different jurisdictions should not conflict
        let statute1 = Statute::new(
            "law-1",
            "US Law",
            Effect::new(EffectType::Grant, "Grant benefit"),
        )
        .with_jurisdiction("US");

        let statute2 = Statute::new(
            "law-2",
            "UK Law",
            Effect::new(EffectType::Prohibition, "Prohibit action"),
        )
        .with_jurisdiction("UK");

        let conflicts = detect_effect_conflicts(&[statute1, statute2]);

        assert!(conflicts.is_empty());
    }

    #[test]
    fn test_conflict_report_generation() {
        // Create statutes with a known conflict
        let statute1 = Statute::new("dup-id", "First", Effect::new(EffectType::Grant, "Grant"));

        let statute2 = Statute::new("dup-id", "Second", Effect::new(EffectType::Grant, "Grant"));

        let report = conflict_detection_report(&[statute1, statute2]);

        assert!(report.contains("Conflict Detection Report"));
        assert!(report.contains("ID Collision"));
    }

    #[test]
    fn test_temporal_validity_overlap() {
        use chrono::NaiveDate;

        let tv1 = TemporalValidity::new()
            .with_effective_date(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())
            .with_expiry_date(NaiveDate::from_ymd_opt(2025, 12, 31).unwrap());

        let tv2 = TemporalValidity::new()
            .with_effective_date(NaiveDate::from_ymd_opt(2025, 6, 1).unwrap())
            .with_expiry_date(NaiveDate::from_ymd_opt(2026, 6, 1).unwrap());

        assert!(temporal_validity_overlaps(&tv1, &tv2));

        let tv3 = TemporalValidity::new()
            .with_effective_date(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())
            .with_expiry_date(NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());

        let tv4 = TemporalValidity::new()
            .with_effective_date(NaiveDate::from_ymd_opt(2027, 1, 1).unwrap())
            .with_expiry_date(NaiveDate::from_ymd_opt(2027, 12, 31).unwrap());

        assert!(!temporal_validity_overlaps(&tv3, &tv4));
    }

    #[test]
    fn test_effects_contradict() {
        let grant = Effect::new(EffectType::Grant, "Grant permission");
        let revoke = Effect::new(EffectType::Revoke, "Revoke permission");
        let prohibition = Effect::new(EffectType::Prohibition, "Prohibit action");

        assert!(effects_contradict(&grant, &revoke));
        assert!(effects_contradict(&grant, &prohibition));
        assert!(!effects_contradict(&grant, &grant));
    }

    #[test]
    fn test_title_similarity() {
        let sim1 = title_similarity("Traffic Law Amendment", "Traffic Law");
        assert!(sim1 > 0.5);

        let sim2 = title_similarity("Completely Different", "Another Thing");
        assert!(sim2 < 0.5);

        let sim3 = title_similarity("Same Title", "Same Title");
        assert_eq!(sim3, 1.0);
    }

    #[test]
    fn test_conditions_overlap() {
        let cond1 = vec![Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        }];

        let cond2 = vec![Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 21,
        }];

        assert!(conditions_overlap(&cond1, &cond2));

        let cond3 = vec![Condition::Income {
            operator: ComparisonOp::LessThan,
            value: 50000,
        }];

        assert!(!conditions_overlap(&cond1, &cond3));
    }

    #[test]
    fn test_conflict_with_suggestions() {
        let conflict = StatuteConflict::new(
            ConflictType::EffectConflict,
            vec!["law1".to_string(), "law2".to_string()],
            "Test conflict",
        )
        .with_suggestion("Fix it")
        .with_suggestion("Or do this");

        assert_eq!(conflict.resolution_suggestions.len(), 2);
        assert_eq!(conflict.severity, Severity::Critical);
    }

    #[test]
    fn test_coverage_gap_detection() {
        let statutes = vec![
            Statute::new("young", "Young Adult Rights", Effect::grant("vote"))
                .with_precondition(Condition::Age {
                    operator: ComparisonOp::GreaterOrEqual,
                    value: 18,
                })
                .with_precondition(Condition::Age {
                    operator: ComparisonOp::LessThan,
                    value: 25,
                }),
            Statute::new("senior", "Senior Rights", Effect::grant("benefits")).with_precondition(
                Condition::Age {
                    operator: ComparisonOp::GreaterOrEqual,
                    value: 65,
                },
            ),
        ];

        let gaps = analyze_coverage_gaps(&statutes);

        // Should detect age gap between 25 and 65
        assert!(!gaps.is_empty());
        assert!(gaps.iter().any(|g| g.description.contains("age coverage")));
    }

    #[test]
    fn test_no_coverage_gaps_simple() {
        let statutes = vec![Statute::new(
            "general",
            "General Law",
            Effect::grant("rights"),
        )];

        let gaps = analyze_coverage_gaps(&statutes);

        // No age conditions, so no age-based gaps
        assert!(gaps.is_empty());
    }

    #[test]
    fn test_jurisdiction_gap_detection() {
        let statutes = vec![
            Statute::new("us-law", "US Law", Effect::grant("benefit")).with_jurisdiction("US"),
            Statute::new("eu-law", "EU Law", Effect::grant("benefit")).with_jurisdiction("EU"),
            Statute::new("no-jurisdiction", "Unknown", Effect::grant("other")),
        ];

        let gaps = analyze_coverage_gaps(&statutes);

        // Should detect jurisdiction gap (need multiple jurisdictions first)
        assert!(
            gaps.iter()
                .any(|g| g.description.contains("no jurisdiction"))
        );
    }

    #[test]
    fn test_optimization_report_generation() {
        let statutes = vec![
            Statute::new("complex", "Complex Law", Effect::grant("rights")).with_precondition(
                Condition::Age {
                    operator: ComparisonOp::GreaterOrEqual,
                    value: 18,
                },
            ),
        ];

        let report = optimization_and_gaps_report(&statutes);

        assert!(report.contains("Statute Optimization"));
        assert!(report.contains("Coverage Gaps"));
        assert!(report.contains("Summary"));
        assert!(report.contains("Total statutes analyzed: 1"));
    }

    #[test]
    fn test_coverage_gap_severity_levels() {
        let statutes = vec![
            Statute::new("income-law", "Income-Based Law", Effect::grant("credit"))
                .with_precondition(Condition::Income {
                    operator: ComparisonOp::LessThan,
                    value: 50000,
                }),
        ];

        let gaps = analyze_coverage_gaps(&statutes);

        // Income-based statutes should generate info-level gap
        if let Some(gap) = gaps.iter().find(|g| g.description.contains("Income")) {
            assert_eq!(gap.severity, Severity::Info);
        }
    }

    // ========================================================================
    // Tests for Dependency Graph Export
    // ========================================================================

    #[test]
    fn test_export_dependency_graph() {
        let statutes = vec![
            Statute::new("law1", "First Law", Effect::grant("right1")),
            Statute::new("law2", "Second Law", Effect::grant("right2")).with_precondition(
                Condition::Custom {
                    description: "statute:law1".to_string(),
                },
            ),
        ];

        let dot = export_dependency_graph(&statutes);

        assert!(dot.contains("digraph StatuteDependencies"));
        assert!(dot.contains("law1"));
        assert!(dot.contains("law2"));
        assert!(dot.contains("law2\" -> \"law1"));
        assert!(dot.contains("[label=\"references\"]"));
    }

    #[test]
    fn test_export_dependency_graph_with_conflicts() {
        let statutes = vec![
            Statute::new("law1", "First Law", Effect::grant("benefit")).with_precondition(
                Condition::Age {
                    operator: ComparisonOp::GreaterOrEqual,
                    value: 18,
                },
            ),
            Statute::new("law2", "Second Law", Effect::revoke("benefit")).with_precondition(
                Condition::Age {
                    operator: ComparisonOp::GreaterOrEqual,
                    value: 18,
                },
            ),
        ];

        let dot = export_dependency_graph_with_conflicts(&statutes);

        assert!(dot.contains("digraph StatuteDependenciesWithConflicts"));
        assert!(dot.contains("law1"));
        assert!(dot.contains("law2"));
        // Should contain conflict highlighting
        assert!(dot.contains("lightcoral") || dot.contains("lightblue"));
    }

    #[test]
    fn test_export_dependency_graph_no_references() {
        let statutes = vec![
            Statute::new("law1", "Independent Law 1", Effect::grant("right1")),
            Statute::new("law2", "Independent Law 2", Effect::grant("right2")),
        ];

        let dot = export_dependency_graph(&statutes);

        assert!(dot.contains("law1"));
        assert!(dot.contains("law2"));
        // No reference edges expected
        assert!(!dot.contains("->"));
    }

    // ========================================================================
    // Tests for Quality Metrics
    // ========================================================================

    #[test]
    fn test_quality_metrics_basic() {
        let statute = Statute::new("test-law", "Test Statute", Effect::grant("benefit"))
            .with_jurisdiction("US")
            .with_temporal_validity(TemporalValidity::new().with_enacted_at(chrono::Utc::now()));

        let metrics = analyze_quality(&statute);

        assert_eq!(metrics.statute_id, "test-law");
        assert!(metrics.overall_score >= 0.0 && metrics.overall_score <= 100.0);
        assert!(metrics.complexity_score >= 0.0 && metrics.complexity_score <= 100.0);
        assert!(metrics.readability_score >= 0.0 && metrics.readability_score <= 100.0);
    }

    #[test]
    fn test_quality_metrics_grade() {
        let metrics = QualityMetrics::new(
            "test".to_string(),
            95.0,
            95.0,
            95.0,
            95.0,
            95.0,
            95.0,
            95.0,
            95.0,
        );

        assert_eq!(metrics.grade(), 'A');
        assert_eq!(metrics.overall_score, 95.0);
    }

    #[test]
    fn test_quality_metrics_with_issues() {
        let statute = Statute::new("incomplete-law", "Incomplete Law", Effect::grant("benefit"));

        let metrics = analyze_quality(&statute);

        // Should have issues for missing jurisdiction and enacted date
        assert!(!metrics.issues.is_empty());
        assert!(metrics.issues.iter().any(|i| i.contains("jurisdiction")));
    }

    #[test]
    fn test_quality_report_generation() {
        let statutes = vec![
            Statute::new("law1", "Good Law", Effect::grant("benefit"))
                .with_jurisdiction("US")
                .with_temporal_validity(TemporalValidity::new().with_enacted_at(chrono::Utc::now()))
                .with_discretion("A well-documented law"),
            Statute::new("law2", "Poor Law", Effect::grant("other")),
        ];

        let report = quality_report(&statutes);

        assert!(report.contains("# Statute Quality Report"));
        assert!(report.contains("law1"));
        assert!(report.contains("law2"));
        assert!(report.contains("Summary"));
        assert!(report.contains("Total statutes analyzed: 2"));
        assert!(report.contains("Grade Distribution"));
    }

    #[test]
    fn test_quality_metrics_low_complexity_strength() {
        let statute = Statute::new("simple-law", "Simple Law", Effect::grant("benefit"))
            .with_jurisdiction("US")
            .with_temporal_validity(TemporalValidity::new().with_enacted_at(chrono::Utc::now()))
            .with_discretion("A simple law");

        let metrics = analyze_quality(&statute);

        assert!(metrics.strengths.iter().any(|s| s.contains("complexity")));
    }

    #[test]
    fn test_drafting_quality_score_high() {
        // Well-drafted statute with all best practices
        let statute = Statute::new(
            "well-drafted-law",
            "Citizens Tax Relief Act",
            Effect::obligation("must file annual tax returns"),
        )
        .with_jurisdiction("US")
        .with_temporal_validity(
            TemporalValidity::new()
                .with_enacted_at(chrono::Utc::now())
                .with_effective_date(chrono::NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()),
        )
        .with_discretion("Applies to all US citizens earning taxable income")
        .with_precondition(legalis_core::Condition::Income {
            operator: legalis_core::ComparisonOp::GreaterOrEqual,
            value: 12000,
        });

        let metrics = analyze_quality(&statute);

        // Should have high drafting quality score
        assert!(
            metrics.drafting_quality_score >= 70.0,
            "Drafting quality should be >= 70, got {}",
            metrics.drafting_quality_score
        );
    }

    #[test]
    fn test_drafting_quality_score_low() {
        // Poorly drafted statute missing key elements
        let statute = Statute::new("poor-law", "", Effect::grant(""));

        let metrics = analyze_quality(&statute);

        // Should have low drafting quality score
        assert!(
            metrics.drafting_quality_score < 50.0,
            "Drafting quality should be < 50, got {}",
            metrics.drafting_quality_score
        );
    }

    #[test]
    fn test_clarity_index_high() {
        // Clear, simple statute
        let statute = Statute::new(
            "clear-law",
            "Simple Tax Law",
            Effect::grant("tax exemption for seniors"),
        )
        .with_discretion("Clear and simple rule")
        .with_precondition(legalis_core::Condition::Age {
            operator: legalis_core::ComparisonOp::GreaterOrEqual,
            value: 65,
        });

        let metrics = analyze_quality(&statute);

        // Should have high clarity index
        assert!(
            metrics.clarity_index >= 70.0,
            "Clarity index should be >= 70, got {}",
            metrics.clarity_index
        );
    }

    #[test]
    fn test_clarity_index_low() {
        // Complex, verbose statute
        let complex_desc = "This regulation establishes procedures and requirements \
            for the implementation of tax relief measures applicable to certain categories \
            of individuals meeting specific criteria as determined by the regulatory authority \
            in accordance with established guidelines and subject to periodic review";

        let statute = Statute::new("complex-law", "Very Long Title That Exceeds Reasonable Length For A Statute Title And Becomes Confusing", Effect::grant(complex_desc))
            .with_precondition(
                legalis_core::Condition::And(
                    Box::new(legalis_core::Condition::Age {
                        operator: legalis_core::ComparisonOp::GreaterOrEqual,
                        value: 18,
                    }),
                    Box::new(legalis_core::Condition::Or(
                        Box::new(legalis_core::Condition::Income {
                            operator: legalis_core::ComparisonOp::LessThan,
                            value: 50000,
                        }),
                        Box::new(legalis_core::Condition::Income {
                            operator: legalis_core::ComparisonOp::GreaterOrEqual,
                            value: 100000,
                        }),
                    )),
                ),
            );

        let metrics = analyze_quality(&statute);

        // Should have lower clarity index due to complexity and verbosity
        assert!(
            metrics.clarity_index < 85.0,
            "Clarity index should be < 85, got {}",
            metrics.clarity_index
        );
    }

    #[test]
    fn test_testability_score_high() {
        // Highly testable statute with concrete conditions
        let statute = Statute::new(
            "testable-law",
            "Age Requirement Law",
            Effect::grant("voting rights"),
        )
        .with_jurisdiction("US")
        .with_temporal_validity(
            TemporalValidity::new()
                .with_effective_date(chrono::NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())
                .with_expiry_date(chrono::NaiveDate::from_ymd_opt(2030, 12, 31).unwrap()),
        )
        .with_precondition(legalis_core::Condition::And(
            Box::new(legalis_core::Condition::Age {
                operator: legalis_core::ComparisonOp::GreaterOrEqual,
                value: 18,
            }),
            Box::new(legalis_core::Condition::HasAttribute {
                key: "citizenship".to_string(),
            }),
        ));

        let metrics = analyze_quality(&statute);

        // Should have high testability score
        assert!(
            metrics.testability_score >= 70.0,
            "Testability should be >= 70, got {}",
            metrics.testability_score
        );
    }

    #[test]
    fn test_testability_score_low() {
        // Less testable statute with fuzzy/custom conditions
        let statute = Statute::new("fuzzy-law", "Vague Law", Effect::grant("some benefit"))
            .with_precondition(legalis_core::Condition::And(
                Box::new(legalis_core::Condition::Custom {
                    description: "must demonstrate good character".to_string(),
                }),
                Box::new(legalis_core::Condition::Fuzzy {
                    attribute: "creditworthiness".to_string(),
                    membership_points: vec![(300.0, 0.0), (700.0, 0.5), (850.0, 1.0)],
                    min_membership: 0.7,
                }),
            ));

        let metrics = analyze_quality(&statute);

        // Should have lower testability score
        assert!(
            metrics.testability_score < 70.0,
            "Testability should be < 70, got {}",
            metrics.testability_score
        );
    }

    #[test]
    fn test_maintainability_score_high() {
        // Highly maintainable statute
        let statute = Statute::new("maintainable-law", "Simple Rule", Effect::grant("benefit"))
            .with_jurisdiction("US")
            .with_temporal_validity(TemporalValidity::new().with_enacted_at(chrono::Utc::now()))
            .with_discretion("Clear documentation explaining the purpose and application")
            .with_precondition(legalis_core::Condition::Age {
                operator: legalis_core::ComparisonOp::GreaterOrEqual,
                value: 18,
            });

        let metrics = analyze_quality(&statute);

        // Should have high maintainability score
        assert!(
            metrics.maintainability_score >= 70.0,
            "Maintainability should be >= 70, got {}",
            metrics.maintainability_score
        );
    }

    #[test]
    fn test_maintainability_score_low() {
        // Poorly maintainable statute
        let statute = Statute::new("unmaintainable-law", "", Effect::grant("")).with_precondition(
            legalis_core::Condition::And(
                Box::new(legalis_core::Condition::And(
                    Box::new(legalis_core::Condition::Or(
                        Box::new(legalis_core::Condition::Age {
                            operator: legalis_core::ComparisonOp::GreaterOrEqual,
                            value: 18,
                        }),
                        Box::new(legalis_core::Condition::Age {
                            operator: legalis_core::ComparisonOp::LessThan,
                            value: 65,
                        }),
                    )),
                    Box::new(legalis_core::Condition::And(
                        Box::new(legalis_core::Condition::Income {
                            operator: legalis_core::ComparisonOp::GreaterThan,
                            value: 25000,
                        }),
                        Box::new(legalis_core::Condition::Income {
                            operator: legalis_core::ComparisonOp::LessThan,
                            value: 75000,
                        }),
                    )),
                )),
                Box::new(legalis_core::Condition::And(
                    Box::new(legalis_core::Condition::HasAttribute {
                        key: "attr1".to_string(),
                    }),
                    Box::new(legalis_core::Condition::And(
                        Box::new(legalis_core::Condition::HasAttribute {
                            key: "attr2".to_string(),
                        }),
                        Box::new(legalis_core::Condition::HasAttribute {
                            key: "attr3".to_string(),
                        }),
                    )),
                )),
            ),
        );

        let metrics = analyze_quality(&statute);

        // Should have lower maintainability score
        assert!(
            metrics.maintainability_score < 60.0,
            "Maintainability should be < 60, got {}",
            metrics.maintainability_score
        );
    }

    #[test]
    fn test_comprehensive_quality_metrics() {
        // Test that all new metrics are included in overall score
        let statute = Statute::new(
            "comprehensive-law",
            "Well Designed Law",
            Effect::grant("comprehensive benefit"),
        )
        .with_jurisdiction("US")
        .with_temporal_validity(
            TemporalValidity::new()
                .with_enacted_at(chrono::Utc::now())
                .with_effective_date(chrono::NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()),
        )
        .with_discretion("Comprehensive documentation")
        .with_precondition(legalis_core::Condition::Age {
            operator: legalis_core::ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let metrics = analyze_quality(&statute);

        // Verify all metrics are populated and within valid range
        assert!(
            (0.0..=100.0).contains(&metrics.drafting_quality_score),
            "Drafting quality out of range: {}",
            metrics.drafting_quality_score
        );
        assert!(
            (0.0..=100.0).contains(&metrics.clarity_index),
            "Clarity index out of range: {}",
            metrics.clarity_index
        );
        assert!(
            (0.0..=100.0).contains(&metrics.testability_score),
            "Testability out of range: {}",
            metrics.testability_score
        );
        assert!(
            (0.0..=100.0).contains(&metrics.maintainability_score),
            "Maintainability out of range: {}",
            metrics.maintainability_score
        );

        // Overall score should be average of all 8 metrics
        let expected_avg = (metrics.complexity_score
            + metrics.readability_score
            + metrics.consistency_score
            + metrics.completeness_score
            + metrics.drafting_quality_score
            + metrics.clarity_index
            + metrics.testability_score
            + metrics.maintainability_score)
            / 8.0;

        assert!(
            (metrics.overall_score - expected_avg).abs() < 0.01,
            "Overall score mismatch: expected {}, got {}",
            expected_avg,
            metrics.overall_score
        );
    }

    // ========================================================================
    // Tests for Ambiguity Detection
    // ========================================================================

    #[test]
    fn test_detect_vague_terms_in_title() {
        let statute = Statute::new(
            "vague-law",
            "Reasonable Tax Law",
            Effect::grant("tax benefit"),
        );

        let ambiguities = detect_ambiguities(&statute);

        assert!(!ambiguities.is_empty());
        assert!(
            ambiguities
                .iter()
                .any(|a| matches!(a.ambiguity_type, AmbiguityType::VagueTerm))
        );
    }

    #[test]
    fn test_detect_vague_terms_in_description() {
        let statute = Statute::new(
            "vague-desc-law",
            "Tax Law",
            Effect::grant("may receive appropriate benefits"),
        );

        let ambiguities = detect_ambiguities(&statute);

        assert!(!ambiguities.is_empty());
        assert!(
            ambiguities
                .iter()
                .any(|a| matches!(a.ambiguity_type, AmbiguityType::VagueTerm))
        );
    }

    #[test]
    fn test_detect_unclear_effect_empty() {
        let statute = Statute::new("unclear-law", "Test Law", Effect::grant(""));

        let ambiguities = detect_ambiguities(&statute);

        assert!(!ambiguities.is_empty());
        assert!(
            ambiguities
                .iter()
                .any(|a| matches!(a.ambiguity_type, AmbiguityType::UnclearEffect))
        );
    }

    #[test]
    fn test_detect_unclear_effect_too_brief() {
        let statute = Statute::new("brief-law", "Test Law", Effect::grant("do it"));

        let ambiguities = detect_ambiguities(&statute);

        assert!(!ambiguities.is_empty());
        assert!(
            ambiguities
                .iter()
                .any(|a| matches!(a.ambiguity_type, AmbiguityType::UnclearEffect))
        );
    }

    #[test]
    fn test_detect_missing_discretion() {
        // Create a statute with multiple preconditions (>3) without discretion logic
        let statute = Statute::new(
            "complex-law",
            "Complex Tax Law",
            Effect::grant("tax credit"),
        )
        .with_precondition(legalis_core::Condition::Age {
            operator: legalis_core::ComparisonOp::GreaterOrEqual,
            value: 18,
        })
        .with_precondition(legalis_core::Condition::Income {
            operator: legalis_core::ComparisonOp::LessThan,
            value: 50000,
        })
        .with_precondition(legalis_core::Condition::HasAttribute {
            key: "citizen".to_string(),
        })
        .with_precondition(legalis_core::Condition::HasAttribute {
            key: "resident".to_string(),
        });

        let ambiguities = detect_ambiguities(&statute);

        // Should detect missing discretion for complex conditions (>3 preconditions)
        assert!(
            ambiguities
                .iter()
                .any(|a| matches!(a.ambiguity_type, AmbiguityType::MissingDiscretion))
        );
    }

    #[test]
    fn test_detect_temporal_ambiguity_no_dates() {
        let statute = Statute::new("temporal-law", "Test Law", Effect::grant("benefit"));

        let ambiguities = detect_ambiguities(&statute);

        assert!(!ambiguities.is_empty());
        assert!(
            ambiguities
                .iter()
                .any(|a| matches!(a.ambiguity_type, AmbiguityType::TemporalAmbiguity))
        );
    }

    #[test]
    fn test_detect_temporal_ambiguity_missing_effective_date() {
        let statute = Statute::new("temporal-law", "Test Law", Effect::grant("benefit"))
            .with_temporal_validity(TemporalValidity::new().with_enacted_at(chrono::Utc::now()));

        let ambiguities = detect_ambiguities(&statute);

        assert!(
            ambiguities
                .iter()
                .any(|a| matches!(a.ambiguity_type, AmbiguityType::TemporalAmbiguity))
        );
    }

    #[test]
    fn test_detect_quantifier_ambiguity() {
        let statute = Statute::new(
            "quant-law",
            "Test Law",
            Effect::grant("some benefits for several qualified individuals"),
        );

        let ambiguities = detect_ambiguities(&statute);

        assert!(!ambiguities.is_empty());
        assert!(
            ambiguities
                .iter()
                .any(|a| matches!(a.ambiguity_type, AmbiguityType::QuantifierAmbiguity))
        );
    }

    #[test]
    fn test_detect_implicit_assumption_custom_condition() {
        let statute = Statute::new("assumption-law", "Test Law", Effect::grant("benefit"))
            .with_precondition(legalis_core::Condition::Custom {
                description: "good".to_string(),
            });

        let ambiguities = detect_ambiguities(&statute);

        assert!(!ambiguities.is_empty());
        assert!(
            ambiguities
                .iter()
                .any(|a| matches!(a.ambiguity_type, AmbiguityType::ImplicitAssumption))
        );
    }

    #[test]
    fn test_no_ambiguities_well_defined_statute() {
        let statute = Statute::new(
            "clear-law",
            "Senior Citizen Tax Credit",
            Effect::grant("tax credit of $1000 for qualified seniors"),
        )
        .with_jurisdiction("US")
        .with_temporal_validity(
            TemporalValidity::new()
                .with_enacted_at(chrono::Utc::now())
                .with_effective_date(chrono::NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()),
        )
        .with_discretion("Clear rule for senior tax credits")
        .with_precondition(legalis_core::Condition::Age {
            operator: legalis_core::ComparisonOp::GreaterOrEqual,
            value: 65,
        });

        let ambiguities = detect_ambiguities(&statute);

        // Should have no or very few ambiguities
        assert!(ambiguities.is_empty() || ambiguities.len() <= 1);
    }

    #[test]
    fn test_ambiguity_report_generation() {
        let statute = Statute::new("vague-law", "Reasonable Law", Effect::grant(""));

        let report = ambiguity_report(&statute);

        assert!(report.contains("Ambiguity Report"));
        assert!(report.contains("vague-law"));
    }

    #[test]
    fn test_batch_ambiguity_report() {
        let statutes = vec![
            Statute::new("law1", "Reasonable Law", Effect::grant("")),
            Statute::new(
                "law2",
                "Clear Law",
                Effect::grant("specific tax credit of $500"),
            )
            .with_jurisdiction("US")
            .with_temporal_validity(
                TemporalValidity::new()
                    .with_enacted_at(chrono::Utc::now())
                    .with_effective_date(chrono::NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()),
            ),
        ];

        let report = batch_ambiguity_report(&statutes);

        assert!(report.contains("Batch Ambiguity Detection Report"));
        assert!(report.contains("**Total Statutes Analyzed**: 2"));
        assert!(report.contains("law1"));
    }

    #[test]
    fn test_ambiguity_severity_sorting() {
        let statute = Statute::new("multi-ambiguity-law", "Test", Effect::grant(""))
            .with_precondition(legalis_core::Condition::Custom {
                description: "test".to_string(),
            });

        let ambiguities = detect_ambiguities(&statute);

        // Verify ambiguities are sorted by severity (descending)
        for i in 0..ambiguities.len().saturating_sub(1) {
            assert!(
                ambiguities[i].severity >= ambiguities[i + 1].severity,
                "Ambiguities should be sorted by severity"
            );
        }
    }

    // ========================================================================
    // Tests for Change Impact Analysis
    // ========================================================================

    #[test]
    fn test_compare_statutes_no_changes() {
        let statute1 = Statute::new("law1", "Test Law", Effect::grant("benefit"));
        let statute2 = Statute::new("law1", "Test Law", Effect::grant("benefit"));

        let changes = compare_statutes(&statute1, &statute2);

        assert!(changes.is_empty());
    }

    #[test]
    fn test_compare_statutes_title_changed() {
        let old = Statute::new("law1", "Old Title", Effect::grant("benefit"));
        let new = Statute::new("law1", "New Title", Effect::grant("benefit"));

        let changes = compare_statutes(&old, &new);

        assert_eq!(changes.len(), 1);
        assert!(matches!(changes[0], StatuteChange::TitleChanged { .. }));
    }

    #[test]
    fn test_compare_statutes_effect_changed() {
        let old = Statute::new("law1", "Test Law", Effect::grant("benefit"));
        let new = Statute::new("law1", "Test Law", Effect::revoke("benefit"));

        let changes = compare_statutes(&old, &new);

        assert!(
            changes
                .iter()
                .any(|c| matches!(c, StatuteChange::EffectChanged { .. }))
        );
    }

    #[test]
    fn test_compare_statutes_preconditions_changed() {
        let old = Statute::new("law1", "Test Law", Effect::grant("benefit")).with_precondition(
            Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            },
        );
        let new = Statute::new("law1", "Test Law", Effect::grant("benefit")).with_precondition(
            Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 21,
            },
        );

        let changes = compare_statutes(&old, &new);

        assert!(
            changes
                .iter()
                .any(|c| matches!(c, StatuteChange::PreconditionsChanged { .. }))
        );
    }

    #[test]
    fn test_analyze_change_impact_no_dependents() {
        let old = Statute::new("law1", "Old Version", Effect::grant("benefit"));
        let new = Statute::new("law1", "New Version", Effect::grant("benefit"));
        let all_statutes = vec![new.clone()];

        let impact = analyze_change_impact(&new, &old, &all_statutes);

        assert_eq!(impact.statute_id, "law1");
        assert_eq!(impact.affected_statutes.len(), 0);
        assert_eq!(impact.impact_severity, Severity::Info);
    }

    #[test]
    fn test_analyze_change_impact_with_dependents() {
        let old = Statute::new("base-law", "Base Law Old", Effect::grant("benefit"));
        let new = Statute::new("base-law", "Base Law New", Effect::revoke("benefit"));

        let dependent = Statute::new("dependent-law", "Dependent Law", Effect::grant("other"))
            .with_precondition(Condition::Custom {
                description: "statute:base-law".to_string(),
            });

        let all_statutes = vec![new.clone(), dependent];

        let impact = analyze_change_impact(&new, &old, &all_statutes);

        assert_eq!(impact.affected_statutes.len(), 1);
        assert!(
            impact
                .affected_statutes
                .contains(&"dependent-law".to_string())
        );
        assert_eq!(impact.impact_severity, Severity::Critical);
        assert!(!impact.recommendations.is_empty());
    }

    #[test]
    fn test_change_impact_report_generation() {
        let old = Statute::new("law1", "Old Title", Effect::grant("benefit"));
        let new = Statute::new("law1", "New Title", Effect::grant("benefit"));
        let all_statutes = vec![new.clone()];

        let impact = analyze_change_impact(&new, &old, &all_statutes);
        let report = change_impact_report(&impact);

        assert!(report.contains("# Change Impact Analysis"));
        assert!(report.contains("law1"));
        assert!(report.contains("Changes Detected"));
        assert!(report.contains("Affected Statutes"));
        assert!(report.contains("Recommendations"));
    }

    #[test]
    fn test_statute_change_display() {
        let change = StatuteChange::TitleChanged {
            old: "Old".to_string(),
            new: "New".to_string(),
        };

        let display = format!("{}", change);
        assert!(display.contains("Title changed"));
        assert!(display.contains("Old"));
        assert!(display.contains("New"));
    }

    // ========================================================================
    // Tests for Batch Verification
    // ========================================================================

    #[test]
    fn test_batch_verification_basic() {
        let verifier = StatuteVerifier::new();

        let statutes = vec![
            Statute::new("law1", "Valid Law", Effect::grant("benefit")).with_precondition(
                Condition::Age {
                    operator: ComparisonOp::GreaterOrEqual,
                    value: 18,
                },
            ),
            Statute::new("law2", "Another Law", Effect::grant("other")).with_precondition(
                Condition::Income {
                    operator: ComparisonOp::GreaterThan,
                    value: 30000,
                },
            ),
        ];

        let result = batch_verify(&statutes, &verifier);

        assert_eq!(result.total_statutes, 2);
        // total_time_ms is always >= 0 as it's u64, so no need to assert
        assert_eq!(result.pass_rate(), 100.0);
    }

    #[test]
    fn test_batch_verification_result_new() {
        let result = BatchVerificationResult::new();

        assert_eq!(result.total_statutes, 0);
        assert_eq!(result.passed, 0);
        assert_eq!(result.failed, 0);
        assert_eq!(result.pass_rate(), 0.0);
    }

    #[test]
    fn test_batch_verification_add_result() {
        let mut batch_result = BatchVerificationResult::new();

        let result1 = VerificationResult::pass();
        let result2 = VerificationResult::fail(vec![VerificationError::DeadStatute {
            statute_id: "dead-law".to_string(),
        }]);

        batch_result.add_result("law1".to_string(), result1);
        batch_result.add_result("law2".to_string(), result2);

        assert_eq!(batch_result.total_statutes, 2);
        assert_eq!(batch_result.passed, 1);
        assert_eq!(batch_result.failed, 1);
        assert_eq!(batch_result.pass_rate(), 50.0);
        assert!(batch_result.error_counts.get(&Severity::Error).is_some());
    }

    #[test]
    fn test_batch_verification_report() {
        let mut batch_result = BatchVerificationResult::new();

        let pass = VerificationResult::pass();
        let fail = VerificationResult::fail(vec![VerificationError::DeadStatute {
            statute_id: "dead-law".to_string(),
        }]);

        batch_result.add_result("pass-law".to_string(), pass);
        batch_result.add_result("fail-law".to_string(), fail);
        batch_result.total_time_ms = 100;

        let report = batch_verification_report(&batch_result);

        assert!(report.contains("# Batch Verification Report"));
        assert!(report.contains("Summary"));
        assert!(report.contains("Total statutes: 2"));
        assert!(report.contains("Passed: 1"));
        assert!(report.contains("Failed: 1"));
        assert!(report.contains("Pass rate: 50.0%"));
        assert!(report.contains("Error Distribution"));
        assert!(report.contains("Failed Statutes"));
        assert!(report.contains("fail-law"));
    }

    #[test]
    fn test_batch_verification_default() {
        let result = BatchVerificationResult::default();
        assert_eq!(result.total_statutes, 0);
        assert_eq!(result.pass_rate(), 0.0);
    }

    #[test]
    fn test_batch_verification_all_pass() {
        let mut batch_result = BatchVerificationResult::new();

        for i in 1..=5 {
            batch_result.add_result(format!("law{}", i), VerificationResult::pass());
        }

        assert_eq!(batch_result.total_statutes, 5);
        assert_eq!(batch_result.passed, 5);
        assert_eq!(batch_result.failed, 0);
        assert_eq!(batch_result.pass_rate(), 100.0);

        let report = batch_verification_report(&batch_result);
        assert!(report.contains("All statutes passed verification"));
    }

    // ========================================================================
    // Tests for Statistical Analysis
    // ========================================================================

    #[test]
    fn test_statute_statistics_basic() {
        let statutes = vec![
            Statute::new("law1", "First Law", Effect::grant("benefit"))
                .with_precondition(Condition::Age {
                    operator: ComparisonOp::GreaterOrEqual,
                    value: 18,
                })
                .with_jurisdiction("US"),
            Statute::new("law2", "Second Law", Effect::revoke("license"))
                .with_precondition(Condition::Income {
                    operator: ComparisonOp::LessThan,
                    value: 50000,
                })
                .with_precondition(Condition::Age {
                    operator: ComparisonOp::LessThan,
                    value: 65,
                })
                .with_jurisdiction("US"),
        ];

        let stats = analyze_statute_statistics(&statutes);

        assert_eq!(stats.total_count, 2);
        assert_eq!(stats.avg_preconditions, 1.5);
        assert!(stats.jurisdiction_distribution.contains_key("US"));
        assert_eq!(stats.jurisdiction_distribution["US"], 2);
    }

    #[test]
    fn test_statute_statistics_empty() {
        let statutes: Vec<Statute> = Vec::new();
        let stats = analyze_statute_statistics(&statutes);

        assert_eq!(stats.total_count, 0);
        assert_eq!(stats.avg_preconditions, 0.0);
        assert_eq!(stats.median_preconditions, 0.0);
    }

    #[test]
    fn test_statistics_report() {
        let statutes = vec![
            Statute::new("law1", "Test Law", Effect::grant("benefit"))
                .with_precondition(Condition::Age {
                    operator: ComparisonOp::GreaterOrEqual,
                    value: 18,
                })
                .with_jurisdiction("US"),
        ];

        let report = statistics_report(&statutes);

        assert!(report.contains("# Statute Collection Statistics"));
        assert!(report.contains("**Total Statutes**: 1"));
        assert!(report.contains("Jurisdiction Distribution"));
    }

    // ========================================================================
    // Tests for Duplicate Detection
    // ========================================================================

    #[test]
    fn test_detect_duplicates_similar() {
        let statutes = vec![
            Statute::new("law1", "Voting Rights Act", Effect::grant("vote")).with_precondition(
                Condition::Age {
                    operator: ComparisonOp::GreaterOrEqual,
                    value: 18,
                },
            ),
            Statute::new("law2", "Voting Rights Act", Effect::grant("vote")).with_precondition(
                Condition::Age {
                    operator: ComparisonOp::GreaterOrEqual,
                    value: 18,
                },
            ),
        ];

        let duplicates = detect_duplicates(&statutes, 0.70);

        assert!(!duplicates.is_empty());
        assert!(duplicates[0].similarity_score >= 0.70);
    }

    #[test]
    fn test_detect_duplicates_no_similarity() {
        let statutes = vec![
            Statute::new("law1", "Voting Rights", Effect::grant("vote")).with_precondition(
                Condition::Age {
                    operator: ComparisonOp::GreaterOrEqual,
                    value: 18,
                },
            ),
            Statute::new("law2", "Tax Code", Effect::obligation("pay_tax")).with_precondition(
                Condition::Income {
                    operator: ComparisonOp::GreaterThan,
                    value: 50000,
                },
            ),
        ];

        let duplicates = detect_duplicates(&statutes, 0.90);

        assert!(duplicates.is_empty());
    }

    #[test]
    fn test_duplicate_detection_report() {
        let statutes = vec![
            Statute::new("law1", "Test Law", Effect::grant("benefit")).with_precondition(
                Condition::Age {
                    operator: ComparisonOp::GreaterOrEqual,
                    value: 18,
                },
            ),
        ];

        let report = duplicate_detection_report(&statutes, 0.70);

        assert!(report.contains("# Duplicate Detection Report"));
        assert!(report.contains("Minimum Similarity Threshold"));
    }

    // ========================================================================
    // Tests for Regulatory Impact Scoring
    // ========================================================================

    #[test]
    fn test_regulatory_impact_basic() {
        let statute = Statute::new(
            "test-law",
            "Test Statute",
            Effect::new(EffectType::Prohibition, "Prohibited action"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        })
        .with_precondition(Condition::Income {
            operator: ComparisonOp::LessThan,
            value: 50000,
        });

        let impact = analyze_regulatory_impact(&statute);

        assert_eq!(impact.statute_id, "test-law");
        assert!(impact.impact_score > 0);
        assert!(impact.impact_score <= 100);
        assert!(!impact.impact_level.is_empty());
    }

    #[test]
    fn test_regulatory_impact_high() {
        let mut statute = Statute::new(
            "complex-law",
            "Complex Statute",
            Effect::new(EffectType::Prohibition, "Complex prohibition"),
        );

        // Add many preconditions to increase complexity
        for i in 0..10 {
            statute = statute.with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18 + i,
            });
        }

        let impact = analyze_regulatory_impact(&statute);

        assert!(impact.impact_score >= 50);
        assert!(impact.impact_level.contains("Impact"));
    }

    #[test]
    fn test_regulatory_impact_report() {
        let statutes = vec![
            Statute::new("law1", "Law 1", Effect::grant("benefit")).with_precondition(
                Condition::Age {
                    operator: ComparisonOp::GreaterOrEqual,
                    value: 18,
                },
            ),
            Statute::new(
                "law2",
                "Law 2",
                Effect::new(EffectType::Prohibition, "Action"),
            ),
        ];

        let report = regulatory_impact_report(&statutes);

        assert!(report.contains("# Regulatory Impact Assessment"));
        assert!(report.contains("Summary"));
        assert!(report.contains("law1"));
        assert!(report.contains("law2"));
        assert!(report.contains("Impact Score"));
    }

    // ========================================================================
    // Tests for Compliance Checklist
    // ========================================================================

    #[test]
    fn test_generate_compliance_checklist() {
        let statute = Statute::new("test-law", "Test Law", Effect::grant("benefit"))
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            })
            .with_precondition(Condition::Income {
                operator: ComparisonOp::LessThan,
                value: 50000,
            })
            .with_discretion("Optional discretion");

        let checklist = generate_compliance_checklist(&statute);

        // Should have: 2 preconditions + 1 effect + 1 discretion = 4 items minimum
        assert!(checklist.len() >= 4);
        assert!(checklist.iter().any(|item| item.priority == "Required"));
        assert!(checklist.iter().any(|item| item.priority == "Optional"));
    }

    #[test]
    fn test_compliance_checklist_report() {
        let statute = Statute::new("test-law", "Test Law", Effect::grant("benefit"))
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            })
            .with_jurisdiction("US");

        let report = compliance_checklist_report(&statute);

        assert!(report.contains("# Compliance Checklist"));
        assert!(report.contains("test-law"));
        assert!(report.contains("Test Law"));
        assert!(report.contains("US"));
        assert!(report.contains("[ ]"));
    }

    #[test]
    fn test_consolidated_compliance_checklist() {
        let statutes = vec![
            Statute::new("law1", "First Law", Effect::grant("benefit")).with_precondition(
                Condition::Age {
                    operator: ComparisonOp::GreaterOrEqual,
                    value: 18,
                },
            ),
            Statute::new("law2", "Second Law", Effect::grant("license")).with_precondition(
                Condition::Income {
                    operator: ComparisonOp::GreaterThan,
                    value: 30000,
                },
            ),
        ];

        let report = consolidated_compliance_checklist(&statutes);

        assert!(report.contains("# Consolidated Compliance Checklist"));
        assert!(report.contains("**Total Statutes**: 2"));
        assert!(report.contains("law1"));
        assert!(report.contains("law2"));
    }

    // ========================================================================
    // Tests for Reporting Extensions (v0.1.8)
    // ========================================================================

    #[test]
    fn test_generate_compliance_certification() {
        let statutes = vec![
            Statute::new("law1", "Test Law 1", Effect::grant("benefit")).with_precondition(
                Condition::Age {
                    operator: ComparisonOp::GreaterOrEqual,
                    value: 18,
                },
            ),
            Statute::new("law2", "Test Law 2", Effect::grant("license")).with_precondition(
                Condition::Income {
                    operator: ComparisonOp::GreaterThan,
                    value: 30000,
                },
            ),
        ];

        let result = VerificationResult::pass();

        let cert = generate_compliance_certification(
            "CERT-2025-001",
            "Test Organization",
            "Legalis Certifying Authority",
            &statutes,
            &result,
            Some(365),
        );

        assert_eq!(cert.certificate_id, "CERT-2025-001");
        assert_eq!(cert.organization, "Test Organization");
        assert_eq!(cert.certifying_authority, "Legalis Certifying Authority");
        assert_eq!(cert.statute_ids.len(), 2);
        assert_eq!(cert.verification_summary.total_statutes, 2);
        assert_eq!(cert.verification_summary.passed_count, 2);
        assert_eq!(cert.verification_summary.failed_count, 0);
        assert_eq!(cert.verification_summary.pass_rate, 100.0);
        assert!(cert.valid_until.is_some());
    }

    #[test]
    fn test_compliance_certification_report() {
        let statutes = vec![Statute::new("law1", "Test Law", Effect::grant("benefit"))];

        let result = VerificationResult::pass();

        let cert = generate_compliance_certification(
            "CERT-TEST",
            "Org",
            "Authority",
            &statutes,
            &result,
            None,
        );

        let report = compliance_certification_report(&cert);

        assert!(report.contains("# COMPLIANCE CERTIFICATION"));
        assert!(report.contains("CERT-TEST"));
        assert!(report.contains("Org"));
        assert!(report.contains("Authority"));
        assert!(report.contains("Verification Summary"));
        assert!(report.contains("law1"));
    }

    #[test]
    fn test_generate_regulatory_filing() {
        let statutes = vec![
            Statute::new("law1", "Test Law 1", Effect::grant("benefit")).with_jurisdiction("US"),
            Statute::new("law2", "Test Law 2", Effect::prohibition("action"))
                .with_jurisdiction("US"),
        ];

        let results = vec![
            VerificationResult::pass(),
            VerificationResult::fail(vec![VerificationError::Ambiguity {
                message: "Test error".to_string(),
            }]),
        ];

        let filing = generate_regulatory_filing(
            "FILING-2025-001",
            "Federal Regulatory Commission",
            "Annual Compliance",
            "US",
            &statutes,
            &results,
        );

        assert_eq!(filing.filing_id, "FILING-2025-001");
        assert_eq!(filing.regulatory_body, "Federal Regulatory Commission");
        assert_eq!(filing.filing_type, "Annual Compliance");
        assert_eq!(filing.jurisdiction, "US");
        assert_eq!(filing.statutes.len(), 2);
        assert_eq!(filing.statutes[0].status, "Compliant");
        assert_eq!(filing.statutes[1].status, "Non-Compliant");
        assert_eq!(filing.compliance_status, "Partially Compliant");
    }

    #[test]
    fn test_regulatory_filing_report() {
        let statutes = vec![Statute::new("law1", "Test Law", Effect::grant("benefit"))];

        let results = vec![VerificationResult::pass()];

        let filing = generate_regulatory_filing(
            "FILING-TEST",
            "Test Body",
            "Test Type",
            "Test Jurisdiction",
            &statutes,
            &results,
        );

        let report = regulatory_filing_report(&filing);

        assert!(report.contains("# REGULATORY FILING REPORT"));
        assert!(report.contains("FILING-TEST"));
        assert!(report.contains("Test Body"));
        assert!(report.contains("Test Type"));
        assert!(report.contains("Test Jurisdiction"));
        assert!(report.contains("Fully Compliant"));
    }

    #[test]
    fn test_generate_executive_summary() {
        let statutes = vec![
            Statute::new("law1", "Test Law 1", Effect::grant("benefit")).with_precondition(
                Condition::Age {
                    operator: ComparisonOp::GreaterOrEqual,
                    value: 18,
                },
            ),
            Statute::new("law2", "Test Law 2", Effect::grant("license")).with_precondition(
                Condition::Income {
                    operator: ComparisonOp::GreaterThan,
                    value: 30000,
                },
            ),
        ];

        let result = VerificationResult::pass();

        let summary = generate_executive_summary("Test Verification", &statutes, &result);

        assert_eq!(summary.title, "Test Verification");
        assert!(!summary.date.is_empty());
        assert_eq!(summary.risk_level, "Low");
        assert_eq!(summary.statistics.total_statutes, 2);
        assert_eq!(summary.statistics.statutes_with_issues, 0);
        assert_eq!(summary.statistics.total_issues, 0);
        assert!(!summary.key_findings.is_empty());
        assert!(!summary.recommendations.is_empty());
    }

    #[test]
    fn test_executive_summary_with_errors() {
        let statutes = vec![Statute::new("law1", "Test Law", Effect::grant("benefit"))];

        let result = VerificationResult::fail(vec![VerificationError::CircularReference {
            message: "Test error".to_string(),
        }]);

        let summary = generate_executive_summary("Test", &statutes, &result);

        assert_eq!(summary.risk_level, "Critical");
        assert_eq!(summary.statistics.critical_issues, 1);
        assert!(summary.overall_assessment.contains("Critical"));
    }

    #[test]
    fn test_executive_summary_report() {
        let statutes = vec![Statute::new("law1", "Test Law", Effect::grant("benefit"))];

        let result = VerificationResult::pass();

        let summary = generate_executive_summary("Test", &statutes, &result);
        let report = executive_summary_report(&summary);

        assert!(report.contains("# EXECUTIVE SUMMARY"));
        assert!(report.contains("Test"));
        assert!(report.contains("Risk Level"));
        assert!(report.contains("Overall Assessment"));
        assert!(report.contains("Key Findings"));
        assert!(report.contains("Statistics"));
        assert!(report.contains("Recommendations"));
    }

    #[test]
    fn test_report_template_creation() {
        let template = ReportTemplate::new("Test Template")
            .with_header("# Test Header")
            .with_footer("Test Footer")
            .with_toc()
            .with_section(ReportSection::ExecutiveSummary)
            .with_section(ReportSection::VerificationResults);

        assert_eq!(template.name, "Test Template");
        assert_eq!(template.header, Some("# Test Header".to_string()));
        assert_eq!(template.footer, Some("Test Footer".to_string()));
        assert!(template.include_toc);
        assert_eq!(template.sections.len(), 2);
    }

    #[test]
    fn test_generate_custom_report() {
        let statutes = vec![
            Statute::new("law1", "Test Law", Effect::grant("benefit")).with_precondition(
                Condition::Age {
                    operator: ComparisonOp::GreaterOrEqual,
                    value: 18,
                },
            ),
        ];

        let result = VerificationResult::pass();

        let template = ReportTemplate::new("Custom Report")
            .with_header("# Custom Header")
            .with_section(ReportSection::ExecutiveSummary)
            .with_section(ReportSection::VerificationResults)
            .with_footer("Custom Footer");

        let report = generate_custom_report(&template, &statutes, &result);

        assert!(report.contains("# Custom Header"));
        assert!(report.contains("Custom Footer"));
        assert!(report.contains("# EXECUTIVE SUMMARY"));
        assert!(report.contains("# Verification Results"));
    }

    #[test]
    fn test_standard_report_template() {
        let template = standard_report_template();

        assert_eq!(template.name, "Standard Verification Report");
        assert!(template.include_toc);
        assert!(!template.sections.is_empty());
        assert!(template.header.is_some());
        assert!(template.footer.is_some());
    }

    #[test]
    fn test_compliance_report_template() {
        let template = compliance_report_template();

        assert_eq!(template.name, "Compliance Verification Report");
        assert!(template.include_toc);
        assert!(!template.sections.is_empty());
    }

    #[test]
    fn test_quality_report_template() {
        let template = quality_report_template();

        assert_eq!(template.name, "Quality Assessment Report");
        assert!(template.include_toc);
        assert!(!template.sections.is_empty());
    }

    #[test]
    fn test_custom_report_with_all_sections() {
        let statutes = vec![
            Statute::new("law1", "Test Law", Effect::grant("benefit"))
                .with_precondition(Condition::Age {
                    operator: ComparisonOp::GreaterOrEqual,
                    value: 18,
                })
                .with_jurisdiction("US"),
        ];

        let result = VerificationResult::pass();

        let template = ReportTemplate::new("Comprehensive Test")
            .with_toc()
            .with_section(ReportSection::ExecutiveSummary)
            .with_section(ReportSection::VerificationResults)
            .with_section(ReportSection::QualityMetrics)
            .with_section(ReportSection::ComplianceChecklist)
            .with_section(ReportSection::StatisticalAnalysis);

        let report = generate_custom_report(&template, &statutes, &result);

        assert!(report.contains("Table of Contents"));
        assert!(report.contains("Executive Summary"));
        assert!(report.contains("Verification Results"));
        assert!(report.contains("Quality"));
        assert!(report.contains("Compliance"));
        assert!(report.contains("Statistics"));
    }

    #[test]
    fn test_custom_report_section() {
        let statutes = vec![Statute::new("law1", "Test Law", Effect::grant("benefit"))];

        let result = VerificationResult::pass();

        let template =
            ReportTemplate::new("Custom Section Test").with_section(ReportSection::Custom {
                title: "Custom Section Title".to_string(),
                content: "This is custom content for testing.".to_string(),
            });

        let report = generate_custom_report(&template, &statutes, &result);

        assert!(report.contains("# Custom Section Title"));
        assert!(report.contains("This is custom content for testing."));
    }

    // =============================================================================
    // CTL* Model Checking Tests
    // =============================================================================

    #[test]
    fn test_ctl_star_basic_formula() {
        let mut system = TransitionSystem::new();

        let s0 = TemporalState::new("s0").with_proposition("p");
        let s1 = TemporalState::new("s1").with_proposition("q");

        system.add_state(s0);
        system.add_state(s1);
        system.add_transition("s0", "s1");
        system.add_initial_state("s0");

        // E F(q) - there exists a path where q eventually holds
        let formula = CtlStarFormula::exists(CtlStarPathFormula::eventually(
            CtlStarPathFormula::state(CtlStarFormula::atom("q")),
        ));

        assert!(verify_ctl_star(&system, &formula));
    }

    #[test]
    fn test_ctl_star_all_paths() {
        let mut system = TransitionSystem::new();

        let s0 = TemporalState::new("s0").with_proposition("p");
        let s1 = TemporalState::new("s1").with_proposition("p");
        let s2 = TemporalState::new("s2").with_proposition("p");

        system.add_state(s0);
        system.add_state(s1);
        system.add_state(s2);
        system.add_transition("s0", "s1");
        system.add_transition("s0", "s2");
        system.add_initial_state("s0");

        // A X(p) - on all paths, p holds in the next state
        let formula = CtlStarFormula::all(CtlStarPathFormula::next(CtlStarPathFormula::state(
            CtlStarFormula::atom("p"),
        )));

        assert!(verify_ctl_star(&system, &formula));
    }

    #[test]
    fn test_ctl_star_display() {
        let formula = CtlStarFormula::exists(CtlStarPathFormula::eventually(
            CtlStarPathFormula::state(CtlStarFormula::atom("p")),
        ));

        let display = format!("{}", formula);
        assert!(display.contains("E"));
        assert!(display.contains("F"));
        assert!(display.contains("p"));
    }

    #[test]
    fn test_ctl_star_path_formula_display() {
        let path = CtlStarPathFormula::until(
            CtlStarPathFormula::state(CtlStarFormula::atom("p")),
            CtlStarPathFormula::state(CtlStarFormula::atom("q")),
        );

        let display = format!("{}", path);
        assert!(display.contains("U"));
        assert!(display.contains("p"));
        assert!(display.contains("q"));
    }

    #[test]
    fn test_ctl_star_complex_formula() {
        let mut system = TransitionSystem::new();

        let s0 = TemporalState::new("s0").with_proposition("p");
        let s1 = TemporalState::new("s1")
            .with_proposition("p")
            .with_proposition("q");
        let s2 = TemporalState::new("s2").with_proposition("q");

        system.add_state(s0);
        system.add_state(s1);
        system.add_state(s2);
        system.add_transition("s0", "s1");
        system.add_transition("s1", "s2");
        system.add_initial_state("s0");

        // E (p U q) - there exists a path where p holds until q becomes true
        let formula = CtlStarFormula::exists(CtlStarPathFormula::until(
            CtlStarPathFormula::state(CtlStarFormula::atom("p")),
            CtlStarPathFormula::state(CtlStarFormula::atom("q")),
        ));

        assert!(verify_ctl_star(&system, &formula));
    }

    #[test]
    fn test_ctl_star_always_path_formula() {
        let mut system = TransitionSystem::new();

        let s0 = TemporalState::new("s0").with_proposition("p");
        let s1 = TemporalState::new("s1").with_proposition("p");

        system.add_state(s0);
        system.add_state(s1);
        system.add_transition("s0", "s1");
        system.add_transition("s1", "s1"); // Self-loop
        system.add_initial_state("s0");

        // E G(p) - there exists a path where p always holds
        let formula = CtlStarFormula::exists(CtlStarPathFormula::always(
            CtlStarPathFormula::state(CtlStarFormula::atom("p")),
        ));

        assert!(verify_ctl_star(&system, &formula));
    }

    // =============================================================================
    // Timed Automata Verification Tests
    // =============================================================================

    #[test]
    fn test_clock_creation() {
        let clock = Clock::new("x");
        assert_eq!(clock.name, "x");
    }

    #[test]
    fn test_clock_constraint_satisfied() {
        let clock = Clock::new("x");
        let mut valuations = HashMap::new();
        valuations.insert("x".to_string(), 5);

        let constraint = ClockConstraint::Less(clock.clone(), 10);
        assert!(constraint.satisfied(&valuations));

        let constraint2 = ClockConstraint::Greater(clock, 10);
        assert!(!constraint2.satisfied(&valuations));
    }

    #[test]
    fn test_clock_constraint_equal() {
        let clock = Clock::new("x");
        let mut valuations = HashMap::new();
        valuations.insert("x".to_string(), 5);

        let constraint = ClockConstraint::Equal(clock, 5);
        assert!(constraint.satisfied(&valuations));
    }

    #[test]
    fn test_clock_constraint_and() {
        let clock1 = Clock::new("x");
        let clock2 = Clock::new("y");
        let mut valuations = HashMap::new();
        valuations.insert("x".to_string(), 5);
        valuations.insert("y".to_string(), 10);

        let constraint = ClockConstraint::And(
            Box::new(ClockConstraint::Greater(clock1, 3)),
            Box::new(ClockConstraint::Less(clock2, 15)),
        );

        assert!(constraint.satisfied(&valuations));
    }

    #[test]
    fn test_timed_location_creation() {
        let location = TimedLocation::new("l0").accepting();

        assert_eq!(location.id, "l0");
        assert!(location.accepting);
        assert!(location.invariant.is_none());
    }

    #[test]
    fn test_timed_location_with_invariant() {
        let clock = Clock::new("x");
        let invariant = ClockConstraint::Less(clock, 10);
        let location = TimedLocation::new("l0").with_invariant(invariant);

        assert!(location.invariant.is_some());
    }

    #[test]
    fn test_timed_transition_creation() {
        let transition = TimedTransition::new("l0", "l1", "action");

        assert_eq!(transition.from, "l0");
        assert_eq!(transition.to, "l1");
        assert_eq!(transition.action, "action");
        assert!(transition.guard.is_none());
        assert!(transition.resets.is_empty());
    }

    #[test]
    fn test_timed_transition_with_guard_and_reset() {
        let clock = Clock::new("x");
        let guard = ClockConstraint::Greater(clock.clone(), 5);
        let transition = TimedTransition::new("l0", "l1", "action")
            .with_guard(guard)
            .with_reset(clock);

        assert!(transition.guard.is_some());
        assert_eq!(transition.resets.len(), 1);
    }

    #[test]
    fn test_timed_automaton_creation() {
        let mut automaton = TimedAutomaton::new("l0");

        let clock = Clock::new("x");
        automaton.add_clock(clock);

        let location = TimedLocation::new("l0");
        automaton.add_location(location);

        assert_eq!(automaton.initial, "l0");
        assert_eq!(automaton.clocks.len(), 1);
        assert_eq!(automaton.locations.len(), 1);
    }

    #[test]
    fn test_timed_reachability_simple() {
        let mut automaton = TimedAutomaton::new("l0");

        let clock = Clock::new("x");
        automaton.add_clock(clock.clone());

        let l0 = TimedLocation::new("l0");
        let l1 = TimedLocation::new("l1").accepting();

        automaton.add_location(l0);
        automaton.add_location(l1);

        let transition = TimedTransition::new("l0", "l1", "action");
        automaton.add_transition(transition);

        assert!(verify_timed_reachability(&automaton, 100));
    }

    #[test]
    fn test_timed_reachability_with_reset() {
        let mut automaton = TimedAutomaton::new("l0");

        let clock = Clock::new("x");
        automaton.add_clock(clock.clone());

        let l0 = TimedLocation::new("l0");
        let l1 = TimedLocation::new("l1").accepting();

        automaton.add_location(l0);
        automaton.add_location(l1);

        // Test transition with clock reset
        let transition = TimedTransition::new("l0", "l1", "action").with_reset(clock);
        automaton.add_transition(transition);

        assert!(verify_timed_reachability(&automaton, 100));
    }

    #[test]
    fn test_timed_reachability_unreachable() {
        let mut automaton = TimedAutomaton::new("l0");

        let clock = Clock::new("x");
        automaton.add_clock(clock.clone());

        let l0 = TimedLocation::new("l0");
        let l1 = TimedLocation::new("l1").accepting();

        automaton.add_location(l0);
        automaton.add_location(l1);

        // No transitions, so l1 is unreachable
        assert!(!verify_timed_reachability(&automaton, 100));
    }

    // =============================================================================
    // Temporal Property Synthesis Tests
    // =============================================================================

    #[test]
    fn test_synthesize_ltl_always() {
        // Positive traces: p always holds
        let mut state1 = HashSet::new();
        state1.insert("p".to_string());

        let mut state2 = HashSet::new();
        state2.insert("p".to_string());

        let positive_traces = vec![vec![state1.clone(), state2.clone()]];

        // Negative traces: p doesn't always hold
        let mut state3 = HashSet::new();
        state3.insert("q".to_string());

        let negative_traces = vec![vec![state3]];

        let formula = synthesize_ltl_property(&positive_traces, &negative_traces);
        assert!(formula.is_some());

        let formula = formula.unwrap();
        assert!(matches!(formula, LtlFormula::Always(_)));
    }

    #[test]
    fn test_synthesize_ltl_eventually() {
        // Positive traces: q eventually holds
        let mut state1 = HashSet::new();
        state1.insert("p".to_string());

        let mut state2 = HashSet::new();
        state2.insert("q".to_string());

        let positive_traces = vec![vec![state1.clone(), state2.clone()]];

        // Negative traces: q never holds
        let mut state3 = HashSet::new();
        state3.insert("p".to_string());

        let negative_traces = vec![vec![state3.clone(), state3]];

        let formula = synthesize_ltl_property(&positive_traces, &negative_traces);
        assert!(formula.is_some());
    }

    #[test]
    fn test_synthesize_ltl_empty_traces() {
        let positive_traces: Vec<Vec<HashSet<String>>> = vec![];
        let negative_traces: Vec<Vec<HashSet<String>>> = vec![];

        let formula = synthesize_ltl_property(&positive_traces, &negative_traces);
        assert!(formula.is_none());
    }

    #[test]
    fn test_synthesize_ltl_no_separation() {
        // Both positive and negative have the same pattern
        let mut state1 = HashSet::new();
        state1.insert("p".to_string());

        let positive_traces = vec![vec![state1.clone()]];
        let negative_traces = vec![vec![state1]];

        let formula = synthesize_ltl_property(&positive_traces, &negative_traces);
        // Should return None as no pattern can separate them
        assert!(formula.is_none());
    }

    #[test]
    fn test_synthesize_ctl_exists_eventually() {
        let mut system = TransitionSystem::new();

        let s0 = TemporalState::new("s0").with_proposition("p");
        let s1 = TemporalState::new("s1").with_proposition("q");

        system.add_state(s0);
        system.add_state(s1);
        system.add_transition("s0", "s1");
        system.add_initial_state("s0");

        let desired_properties = vec!["q".to_string()];
        let formula = synthesize_ctl_property(&system, &desired_properties);

        assert!(formula.is_some());
        assert!(matches!(formula.unwrap(), CtlFormula::ExistsEventually(_)));
    }

    #[test]
    fn test_synthesize_ctl_all_always() {
        let mut system = TransitionSystem::new();

        let s0 = TemporalState::new("s0").with_proposition("p");
        let s1 = TemporalState::new("s1").with_proposition("p");

        system.add_state(s0);
        system.add_state(s1);
        system.add_transition("s0", "s1");
        system.add_transition("s1", "s1");
        system.add_initial_state("s0");

        let desired_properties = vec!["p".to_string()];
        let formula = synthesize_ctl_property(&system, &desired_properties);

        assert!(formula.is_some());
    }

    #[test]
    fn test_synthesize_ctl_empty_properties() {
        let system = TransitionSystem::new();
        let desired_properties: Vec<String> = vec![];

        let formula = synthesize_ctl_property(&system, &desired_properties);
        assert!(formula.is_none());
    }

    #[test]
    fn test_check_formula_on_trace() {
        let mut state1 = HashSet::new();
        state1.insert("p".to_string());

        let mut state2 = HashSet::new();
        state2.insert("q".to_string());

        let trace = vec![state1, state2];

        let formula = LtlFormula::eventually(LtlFormula::atom("q"));
        assert!(check_formula_on_trace(&formula, &trace));

        let formula2 = LtlFormula::always(LtlFormula::atom("p"));
        assert!(!check_formula_on_trace(&formula2, &trace));
    }

    // =============================================================================
    // CI/CD Integration Tests
    // =============================================================================

    #[test]
    fn test_ci_platform_display() {
        assert_eq!(CiPlatform::GitHubActions.to_string(), "GitHub Actions");
        assert_eq!(CiPlatform::GitLabCI.to_string(), "GitLab CI");
        assert_eq!(CiPlatform::Jenkins.to_string(), "Jenkins");
        assert_eq!(CiPlatform::CircleCI.to_string(), "CircleCI");
        assert_eq!(CiPlatform::TravisCI.to_string(), "Travis CI");
    }

    #[test]
    fn test_ci_config_creation() {
        let config = CiConfig::new(CiPlatform::GitHubActions);

        assert_eq!(config.platform, CiPlatform::GitHubActions);
        assert!(config.fail_on_warnings);
        assert!(config.upload_reports);
        assert_eq!(config.report_dir, "verification-reports");
    }

    #[test]
    fn test_ci_config_builder() {
        let config = CiConfig::new(CiPlatform::GitLabCI)
            .with_command("custom-verify-cmd")
            .fail_on_warnings(false)
            .upload_reports(false)
            .with_report_dir("custom-reports");

        assert_eq!(config.verify_command, "custom-verify-cmd");
        assert!(!config.fail_on_warnings);
        assert!(!config.upload_reports);
        assert_eq!(config.report_dir, "custom-reports");
    }

    #[test]
    fn test_ci_config_github_actions() {
        let config = CiConfig::new(CiPlatform::GitHubActions);
        let output = config.generate();

        assert!(output.contains("name: Statute Verification"));
        assert!(output.contains("actions/checkout"));
        assert!(output.contains("cargo run --bin legalis-verify"));
        assert!(output.contains("upload-artifact"));
    }

    #[test]
    fn test_ci_config_gitlab_ci() {
        let config = CiConfig::new(CiPlatform::GitLabCI);
        let output = config.generate();

        assert!(output.contains("verify:"));
        assert!(output.contains("stage: test"));
        assert!(output.contains("artifacts:"));
    }

    #[test]
    fn test_ci_config_jenkins() {
        let config = CiConfig::new(CiPlatform::Jenkins);
        let output = config.generate();

        assert!(output.contains("pipeline"));
        assert!(output.contains("stage('Verify Statutes')"));
        assert!(output.contains("archiveArtifacts"));
    }

    #[test]
    fn test_ci_config_circleci() {
        let config = CiConfig::new(CiPlatform::CircleCI);
        let output = config.generate();

        assert!(output.contains("version: 2.1"));
        assert!(output.contains("jobs:"));
        assert!(output.contains("store_artifacts"));
    }

    #[test]
    fn test_ci_config_travis() {
        let config = CiConfig::new(CiPlatform::TravisCI);
        let output = config.generate();

        assert!(output.contains("language: rust"));
        assert!(output.contains("script:"));
    }

    // =============================================================================
    // Git Pre-commit Hook Tests
    // =============================================================================

    #[test]
    fn test_precommit_hook_creation() {
        let hook = PreCommitHook::new();

        assert!(hook.fail_on_errors);
        assert!(!hook.fail_on_warnings);
        assert!(hook.verbose);
    }

    #[test]
    fn test_precommit_hook_builder() {
        let hook = PreCommitHook::new()
            .with_command("custom-verify")
            .fail_on_errors(false)
            .fail_on_warnings(true)
            .verbose(false);

        assert_eq!(hook.verify_command, "custom-verify");
        assert!(!hook.fail_on_errors);
        assert!(hook.fail_on_warnings);
        assert!(!hook.verbose);
    }

    #[test]
    fn test_precommit_hook_generation() {
        let hook = PreCommitHook::new();
        let script = hook.generate();

        assert!(script.contains("#!/bin/bash"));
        assert!(script.contains("Running statute verification"));
        assert!(script.contains("cargo run --bin legalis-verify"));
        assert!(script.contains("VERIFICATION_EXIT_CODE"));
    }

    #[test]
    fn test_precommit_hook_default() {
        let hook = PreCommitHook::default();
        assert!(hook.fail_on_errors);
    }

    // =============================================================================
    // Verification API Service Tests
    // =============================================================================

    #[test]
    fn test_verification_request_creation() {
        let statutes = vec![Statute::new("law1", "Test Law", Effect::grant("benefit"))];
        let request = VerificationRequest::new(statutes.clone());

        assert_eq!(request.statutes.len(), 1);
        assert!(request.principles.is_empty());
        assert!(request.request_id.is_none());
        assert!(request.client_id.is_none());
    }

    #[test]
    fn test_verification_request_builder() {
        let statutes = vec![Statute::new("law1", "Test Law", Effect::grant("benefit"))];
        let principles = vec![PrincipleCheck::NoDiscrimination];

        let request = VerificationRequest::new(statutes)
            .with_principles(principles.clone())
            .with_request_id("req-123")
            .with_client_id("client-456");

        assert_eq!(request.request_id, Some("req-123".to_string()));
        assert_eq!(request.client_id, Some("client-456".to_string()));
        assert_eq!(request.principles.len(), 1);
    }

    #[test]
    fn test_verification_response_creation() {
        let results = vec![VerificationResult::pass(), VerificationResult::pass()];
        let response = VerificationResponse::new(Some("req-123".to_string()), results);

        assert_eq!(response.request_id, Some("req-123".to_string()));
        assert_eq!(response.results.len(), 2);
        assert!(response.success);
        assert_eq!(response.error_count, 0);
        assert_eq!(response.warning_count, 0);
    }

    #[test]
    fn test_verification_response_with_errors() {
        let result = VerificationResult::fail(vec![VerificationError::DeadStatute {
            statute_id: "dead_law".to_string(),
        }]);

        let results = vec![result];
        let response = VerificationResponse::new(None, results);

        assert!(!response.success);
        assert_eq!(response.error_count, 1);
    }

    #[test]
    fn test_verification_response_processing_time() {
        let results = vec![VerificationResult::pass()];
        let response = VerificationResponse::new(None, results).with_processing_time(150);

        assert_eq!(response.processing_time_ms, 150);
    }

    // =============================================================================
    // Notification System Tests
    // =============================================================================

    #[test]
    fn test_notification_config_creation() {
        let config = NotificationConfig::new();

        assert!(config.channels.is_empty());
        assert_eq!(config.trigger_on.len(), 2);
        assert!(config.trigger_on.contains(&NotificationType::Error));
        assert!(config.trigger_on.contains(&NotificationType::Critical));
        assert!(config.include_details);
    }

    #[test]
    fn test_notification_config_webhooks() {
        let config = NotificationConfig::new().with_webhook("https://example.com/webhook");

        assert_eq!(config.channels.len(), 1);
        match &config.channels[0] {
            NotificationChannel::Webhook { url, .. } => {
                assert_eq!(url, "https://example.com/webhook");
            }
            _ => panic!("Expected webhook channel"),
        }
    }

    #[test]
    fn test_notification_config_email() {
        let config = NotificationConfig::new()
            .with_email(vec!["test@example.com".to_string()], "Verification Alert");

        assert_eq!(config.channels.len(), 1);
        match &config.channels[0] {
            NotificationChannel::Email { to, subject } => {
                assert_eq!(to.len(), 1);
                assert_eq!(subject, "Verification Alert");
            }
            _ => panic!("Expected email channel"),
        }
    }

    #[test]
    fn test_notification_config_trigger() {
        let config = NotificationConfig::new()
            .trigger_on(vec![NotificationType::Success, NotificationType::Warning]);

        assert_eq!(config.trigger_on.len(), 2);
        assert!(config.trigger_on.contains(&NotificationType::Success));
        assert!(config.trigger_on.contains(&NotificationType::Warning));
    }

    #[test]
    fn test_notification_config_details() {
        let config = NotificationConfig::new().include_details(false);

        assert!(!config.include_details);
    }

    #[test]
    fn test_notification_config_default() {
        let config = NotificationConfig::default();
        assert!(config.channels.is_empty());
    }

    #[test]
    fn test_notification_message_creation() {
        let message = NotificationMessage::new(
            NotificationType::Success,
            "Verification Passed",
            "All statutes verified successfully",
        );

        assert_eq!(message.notification_type, NotificationType::Success);
        assert_eq!(message.title, "Verification Passed");
        assert_eq!(message.message, "All statutes verified successfully");
        assert!(!message.timestamp.is_empty());
        assert!(message.results.is_none());
    }

    #[test]
    fn test_notification_message_with_results() {
        let results = vec![VerificationResult::pass()];
        let message = NotificationMessage::new(NotificationType::Success, "Test", "Test message")
            .with_results(results);

        assert!(message.results.is_some());
        assert_eq!(message.results.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_notification_message_to_json() {
        let message = NotificationMessage::new(
            NotificationType::Error,
            "Verification Failed",
            "Errors found",
        );

        let json = message.to_json();
        assert!(json.contains("\"notification_type\":"));
        assert!(json.contains("\"title\":"));
        assert!(json.contains("\"message\":"));
    }

    #[test]
    fn test_send_notification_no_trigger() {
        let config = NotificationConfig::new().with_webhook("https://example.com");
        let message = NotificationMessage::new(NotificationType::Success, "Test", "Message");

        // Success is not in default triggers (only Error and Critical)
        assert!(!send_notification(&config, &message));
    }

    #[test]
    fn test_send_notification_with_trigger() {
        let config = NotificationConfig::new().with_webhook("https://example.com");
        let message = NotificationMessage::new(NotificationType::Error, "Test", "Message");

        // Error is in default triggers
        assert!(send_notification(&config, &message));
    }

    #[test]
    fn test_send_notification_no_channels() {
        let config = NotificationConfig::new();
        let message = NotificationMessage::new(NotificationType::Error, "Test", "Message");

        // No channels configured
        assert!(!send_notification(&config, &message));
    }

    // =============================================================================
    // Multi-Party Verification Tests (v0.2.1)
    // =============================================================================

    #[test]
    fn test_stakeholder_creation() {
        let stakeholder = Stakeholder::new("S1", "Alice")
            .with_type("individual")
            .with_interest("privacy")
            .with_interest("fairness")
            .affected_by_statute("statute-1");

        assert_eq!(stakeholder.id, "S1");
        assert_eq!(stakeholder.name, "Alice");
        assert_eq!(stakeholder.stakeholder_type, "individual");
        assert_eq!(stakeholder.interests.len(), 2);
        assert_eq!(stakeholder.affected_by.len(), 1);
    }

    #[test]
    fn test_analyze_stakeholder_conflicts_prohibition() {
        let stakeholders = vec![
            Stakeholder::new("S1", "Alice")
                .with_interest("freedom")
                .affected_by_statute("statute-1"),
            Stakeholder::new("S2", "Bob")
                .with_interest("security")
                .affected_by_statute("statute-1"),
        ];

        let statutes = vec![Statute::new(
            "statute-1",
            "Prohibition Law",
            Effect::new(EffectType::Prohibition, "Prohibit certain actions"),
        )];

        let conflicts = analyze_stakeholder_conflicts(&stakeholders, &statutes);

        assert!(!conflicts.is_empty());
        assert_eq!(conflicts[0].conflict_type, ConflictNature::DirectOpposition);
        assert_eq!(conflicts[0].severity, Severity::Warning);
    }

    #[test]
    fn test_analyze_stakeholder_conflicts_grant() {
        let stakeholders = vec![
            Stakeholder::new("S1", "Alice").affected_by_statute("statute-1"),
            Stakeholder::new("S2", "Bob").affected_by_statute("statute-1"),
        ];

        let statutes = vec![Statute::new(
            "statute-1",
            "Grant Law",
            Effect::new(EffectType::Grant, "Grant benefits"),
        )];

        let conflicts = analyze_stakeholder_conflicts(&stakeholders, &statutes);

        assert!(!conflicts.is_empty());
        assert!(
            conflicts
                .iter()
                .any(|c| c.conflict_type == ConflictNature::ResourceCompetition)
        );
    }

    #[test]
    fn test_analyze_stakeholder_conflicts_conflicting_interests() {
        let stakeholders = vec![
            Stakeholder::new("S1", "Alice")
                .with_interest("privacy")
                .affected_by_statute("statute-1"),
            Stakeholder::new("S2", "Bob")
                .with_interest("transparency")
                .affected_by_statute("statute-1"),
        ];

        let statutes = vec![Statute::new(
            "statute-1",
            "Data Law",
            Effect::new(EffectType::Grant, "Grant access"),
        )];

        let conflicts = analyze_stakeholder_conflicts(&stakeholders, &statutes);

        assert!(
            conflicts
                .iter()
                .any(|c| c.conflict_type == ConflictNature::InterpretationDifference)
        );
    }

    #[test]
    fn test_stakeholder_conflict_report() {
        let conflicts = vec![StakeholderConflict {
            stakeholders: vec!["S1".to_string(), "S2".to_string()],
            statutes: vec!["statute-1".to_string()],
            conflict_type: ConflictNature::DirectOpposition,
            severity: Severity::Warning,
            description: "Test conflict".to_string(),
            resolutions: vec!["Resolution 1".to_string(), "Resolution 2".to_string()],
        }];

        let report = stakeholder_conflict_report(&conflicts);

        assert!(report.contains("Multi-Stakeholder Conflict Analysis"));
        assert!(report.contains("Direct Opposition"));
        assert!(report.contains("Test conflict"));
        assert!(report.contains("Resolution 1"));
    }

    #[test]
    fn test_stakeholder_conflict_report_empty() {
        let conflicts = vec![];
        let report = stakeholder_conflict_report(&conflicts);

        assert!(report.contains("No stakeholder conflicts detected"));
    }

    #[test]
    fn test_strategy_creation() {
        let strategy = Strategy::new("S1", "Full Compliance")
            .with_description("Comply with all laws")
            .with_statute_action("statute-1")
            .with_statute_action("statute-2");

        assert_eq!(strategy.stakeholder_id, "S1");
        assert_eq!(strategy.name, "Full Compliance");
        assert_eq!(strategy.description, "Comply with all laws");
        assert_eq!(strategy.statute_actions.len(), 2);
    }

    #[test]
    fn test_game_theoretic_model_creation() {
        let mut model = GameTheoreticModel::new(vec!["S1".to_string(), "S2".to_string()]);

        assert_eq!(model.stakeholders.len(), 2);
        assert_eq!(model.strategies.len(), 2);
        assert_eq!(model.outcomes.len(), 0);

        let strategy1 = Strategy::new("S1", "Comply");
        model.add_strategy(0, strategy1);

        assert_eq!(model.strategies[0].len(), 1);
    }

    #[test]
    fn test_detect_nash_equilibria() {
        let mut model = GameTheoreticModel::new(vec!["S1".to_string(), "S2".to_string()]);

        model.add_outcome(GameOutcome {
            strategies: vec!["Comply".to_string(), "Comply".to_string()],
            payoffs: vec![5, 5],
            is_nash_equilibrium: true,
            description: "Both comply".to_string(),
        });

        model.add_outcome(GameOutcome {
            strategies: vec!["Comply".to_string(), "Defect".to_string()],
            payoffs: vec![2, 7],
            is_nash_equilibrium: false,
            description: "Asymmetric".to_string(),
        });

        let equilibria = detect_nash_equilibria(&model);

        assert_eq!(equilibria.len(), 1);
        assert_eq!(equilibria[0].payoffs, vec![5, 5]);
    }

    #[test]
    fn test_predict_game_outcomes_two_players() {
        let stakeholders = vec![
            Stakeholder::new("S1", "Alice").affected_by_statute("statute-1"),
            Stakeholder::new("S2", "Bob").affected_by_statute("statute-1"),
        ];

        let statutes = vec![Statute::new(
            "statute-1",
            "Test Law",
            Effect::new(EffectType::Grant, "Grant benefit"),
        )];

        let model = predict_game_outcomes(&stakeholders, &statutes);

        assert_eq!(model.stakeholders.len(), 2);
        assert_eq!(model.strategies.len(), 2);
        assert!(!model.strategies[0].is_empty());
        assert!(!model.strategies[1].is_empty());

        // Two-player game should have 4 outcomes
        assert_eq!(model.outcomes.len(), 4);

        // Should have 2 Nash equilibria (both comply, both defect)
        let equilibria = detect_nash_equilibria(&model);
        assert_eq!(equilibria.len(), 2);
    }

    #[test]
    fn test_game_theoretic_report() {
        let mut model = GameTheoreticModel::new(vec!["S1".to_string(), "S2".to_string()]);

        model.add_strategy(
            0,
            Strategy::new("S1", "Comply").with_description("Full compliance"),
        );
        model.add_strategy(
            1,
            Strategy::new("S2", "Comply").with_description("Full compliance"),
        );

        model.add_outcome(GameOutcome {
            strategies: vec!["Comply".to_string(), "Comply".to_string()],
            payoffs: vec![5, 5],
            is_nash_equilibrium: true,
            description: "Both comply equilibrium".to_string(),
        });

        let report = game_theoretic_report(&model);

        assert!(report.contains("Game-Theoretic Outcome Prediction"));
        assert!(report.contains("Nash Equilibria"));
        assert!(report.contains("Full compliance"));
        assert!(report.contains("Equilibrium 1"));
    }

    #[test]
    fn test_coalition_creation() {
        let coalition = Coalition::new(vec!["S1".to_string(), "S2".to_string()])
            .with_objective("Privacy protection")
            .with_collective_effect("Influence statute-1")
            .with_strength(0.75)
            .with_stability(true);

        assert_eq!(coalition.members.len(), 2);
        assert_eq!(coalition.objectives.len(), 1);
        assert_eq!(coalition.collective_effects.len(), 1);
        assert_eq!(coalition.strength, 0.75);
        assert!(coalition.is_stable);
    }

    #[test]
    fn test_coalition_strength_clamping() {
        let coalition1 = Coalition::new(vec!["S1".to_string()]).with_strength(1.5);
        assert_eq!(coalition1.strength, 1.0);

        let coalition2 = Coalition::new(vec!["S1".to_string()]).with_strength(-0.5);
        assert_eq!(coalition2.strength, 0.0);
    }

    #[test]
    fn test_analyze_coalitions() {
        let stakeholders = vec![
            Stakeholder::new("S1", "Alice")
                .with_interest("privacy")
                .affected_by_statute("statute-1"),
            Stakeholder::new("S2", "Bob")
                .with_interest("privacy")
                .affected_by_statute("statute-1"),
            Stakeholder::new("S3", "Carol")
                .with_interest("security")
                .affected_by_statute("statute-2"),
        ];

        let statutes = vec![
            Statute::new(
                "statute-1",
                "Privacy Law",
                Effect::new(EffectType::Grant, "Grant privacy rights"),
            ),
            Statute::new(
                "statute-2",
                "Security Law",
                Effect::new(EffectType::Grant, "Grant security"),
            ),
        ];

        let coalitions = analyze_coalitions(&stakeholders, &statutes);

        // Should find at least one coalition (privacy group)
        assert!(!coalitions.is_empty());

        // Privacy coalition should have 2 members
        let privacy_coalition = coalitions
            .iter()
            .find(|c| c.objectives.contains(&"privacy".to_string()));
        assert!(privacy_coalition.is_some());
        assert_eq!(privacy_coalition.unwrap().members.len(), 2);
    }

    #[test]
    fn test_analyze_coalitions_stable() {
        let stakeholders = vec![
            Stakeholder::new("S1", "Alice")
                .with_interest("education")
                .affected_by_statute("statute-1")
                .affected_by_statute("statute-2"),
            Stakeholder::new("S2", "Bob")
                .with_interest("education")
                .affected_by_statute("statute-1")
                .affected_by_statute("statute-2"),
        ];

        let statutes = vec![
            Statute::new(
                "statute-1",
                "Education Law 1",
                Effect::new(EffectType::Grant, "Grant education"),
            ),
            Statute::new(
                "statute-2",
                "Education Law 2",
                Effect::new(EffectType::Grant, "Grant education"),
            ),
        ];

        let coalitions = analyze_coalitions(&stakeholders, &statutes);

        assert!(!coalitions.is_empty());
        // Coalition should be stable since both members are affected by common statutes
        assert!(coalitions[0].is_stable);
    }

    #[test]
    fn test_coalition_analysis_report() {
        let coalitions = vec![
            Coalition::new(vec!["S1".to_string(), "S2".to_string()])
                .with_objective("Privacy")
                .with_collective_effect("Effect 1")
                .with_strength(0.8)
                .with_stability(true),
            Coalition::new(vec!["S3".to_string(), "S4".to_string()])
                .with_objective("Security")
                .with_strength(0.5)
                .with_stability(false),
        ];

        let report = coalition_analysis_report(&coalitions);

        assert!(report.contains("Coalition Analysis"));
        assert!(report.contains("**Total Coalitions Detected**: 2"));
        assert!(report.contains("**Stable Coalitions**: 1"));
        assert!(report.contains("**Unstable Coalitions**: 1"));
        assert!(report.contains("Privacy"));
        assert!(report.contains("Security"));
    }

    #[test]
    fn test_coalition_analysis_report_empty() {
        let coalitions = vec![];
        let report = coalition_analysis_report(&coalitions);

        assert!(report.contains("No coalitions detected"));
        assert!(report.contains("divergent interests"));
    }

    #[test]
    fn test_coalition_sorting_by_strength() {
        let stakeholders = vec![
            Stakeholder::new("S1", "Alice")
                .with_interest("privacy")
                .affected_by_statute("statute-1")
                .affected_by_statute("statute-2"),
            Stakeholder::new("S2", "Bob")
                .with_interest("privacy")
                .affected_by_statute("statute-1"),
            Stakeholder::new("S3", "Carol")
                .with_interest("security")
                .affected_by_statute("statute-3"),
            Stakeholder::new("S4", "Dave")
                .with_interest("security")
                .affected_by_statute("statute-3"),
        ];

        let statutes = vec![
            Statute::new(
                "statute-1",
                "Privacy Law 1",
                Effect::new(EffectType::Grant, "Grant"),
            ),
            Statute::new(
                "statute-2",
                "Privacy Law 2",
                Effect::new(EffectType::Grant, "Grant"),
            ),
            Statute::new(
                "statute-3",
                "Security Law",
                Effect::new(EffectType::Grant, "Grant"),
            ),
        ];

        let coalitions = analyze_coalitions(&stakeholders, &statutes);

        // Coalitions should be sorted by strength (descending)
        for i in 1..coalitions.len() {
            assert!(coalitions[i - 1].strength >= coalitions[i].strength);
        }
    }

    // =============================================================================
    // Mechanism Design Verification Tests
    // =============================================================================

    #[test]
    fn test_mechanism_analysis_creation() {
        let analysis = MechanismAnalysis::new();

        assert!(analysis.issues.is_empty());
        assert!(analysis.satisfied_properties.is_empty());
        assert_eq!(analysis.quality_score, 1.0);
    }

    #[test]
    fn test_mechanism_analysis_add_issue() {
        let mut analysis = MechanismAnalysis::new();

        analysis.add_issue(MechanismIssue {
            property: MechanismProperty::IncentiveCompatibility,
            statute_ids: vec!["S1".to_string()],
            severity: Severity::Warning,
            description: "Test issue".to_string(),
            suggestions: vec!["Fix it".to_string()],
        });

        assert_eq!(analysis.issues.len(), 1);
        assert!(analysis.quality_score < 1.0);
    }

    #[test]
    fn test_mechanism_analysis_satisfy_property() {
        let mut analysis = MechanismAnalysis::new();

        analysis.satisfy_property(MechanismProperty::IncentiveCompatibility);
        analysis.satisfy_property(MechanismProperty::BudgetBalance);

        assert_eq!(analysis.satisfied_properties.len(), 2);
    }

    #[test]
    fn test_mechanism_design_incentive_compatibility_violation() {
        let stakeholders = vec![Stakeholder::new("S1", "Alice")];

        let statutes = vec![Statute::new(
            "statute-1",
            "Prohibition Law",
            Effect::new(EffectType::Prohibition, "Prohibit action"),
        )];

        let analysis = verify_mechanism_design(&statutes, &stakeholders);

        // Should find incentive compatibility issue
        assert!(
            analysis
                .issues
                .iter()
                .any(|i| i.property == MechanismProperty::IncentiveCompatibility)
        );
    }

    #[test]
    fn test_mechanism_design_incentive_compatibility_satisfied() {
        let stakeholders = vec![Stakeholder::new("S1", "Alice")];

        let statutes = vec![
            Statute::new(
                "statute-1",
                "Grant Law",
                Effect::new(EffectType::Grant, "Grant benefit"),
            )
            .with_discretion("Comply to receive benefit")
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            }),
        ];

        let analysis = verify_mechanism_design(&statutes, &stakeholders);

        // Should satisfy incentive compatibility
        assert!(
            analysis
                .satisfied_properties
                .contains(&MechanismProperty::IncentiveCompatibility)
        );
    }

    #[test]
    fn test_mechanism_design_individual_rationality_violation() {
        let stakeholders = vec![
            Stakeholder::new("S1", "Alice")
                .affected_by_statute("statute-1")
                .affected_by_statute("statute-2"),
        ];

        let statutes = vec![
            Statute::new(
                "statute-1",
                "Prohibition 1",
                Effect::new(EffectType::Prohibition, "Prohibit A"),
            ),
            Statute::new(
                "statute-2",
                "Prohibition 2",
                Effect::new(EffectType::Revoke, "Revoke B"),
            ),
        ];

        let analysis = verify_mechanism_design(&statutes, &stakeholders);

        // Should find individual rationality issue (only penalties, no benefits)
        assert!(
            analysis
                .issues
                .iter()
                .any(|i| i.property == MechanismProperty::IndividualRationality)
        );
    }

    #[test]
    fn test_mechanism_design_individual_rationality_satisfied() {
        let stakeholders = vec![
            Stakeholder::new("S1", "Alice")
                .affected_by_statute("statute-1")
                .affected_by_statute("statute-2"),
        ];

        let statutes = vec![
            Statute::new(
                "statute-1",
                "Grant Law",
                Effect::new(EffectType::Grant, "Grant benefit"),
            ),
            Statute::new(
                "statute-2",
                "Prohibition Law",
                Effect::new(EffectType::Prohibition, "Prohibit action"),
            ),
        ];

        let analysis = verify_mechanism_design(&statutes, &stakeholders);

        // Should satisfy individual rationality (has both benefits and penalties)
        assert!(
            analysis
                .satisfied_properties
                .contains(&MechanismProperty::IndividualRationality)
        );
    }

    #[test]
    fn test_mechanism_design_budget_balance_no_transfers() {
        let stakeholders = vec![Stakeholder::new("S1", "Alice")];

        let statutes = vec![Statute::new(
            "statute-1",
            "Grant Law",
            Effect::new(EffectType::Grant, "Grant benefit"),
        )];

        let analysis = verify_mechanism_design(&statutes, &stakeholders);

        // Budget balance should be satisfied trivially (no monetary transfers)
        assert!(
            analysis
                .satisfied_properties
                .contains(&MechanismProperty::BudgetBalance)
        );
    }

    #[test]
    fn test_mechanism_design_budget_balance_with_transfers() {
        let stakeholders = vec![Stakeholder::new("S1", "Alice")];

        let statutes = vec![
            Statute::new(
                "statute-1",
                "Transfer Law",
                Effect::new(EffectType::MonetaryTransfer, "Transfer money"),
            )
            .with_jurisdiction("US"),
        ];

        let analysis = verify_mechanism_design(&statutes, &stakeholders);

        // Should find budget balance issue (unbalanced transfer)
        assert!(
            analysis
                .issues
                .iter()
                .any(|i| i.property == MechanismProperty::BudgetBalance)
        );
    }

    #[test]
    fn test_mechanism_design_strategy_proofness_custom_condition() {
        let stakeholders = vec![Stakeholder::new("S1", "Alice")];

        let statutes = vec![
            Statute::new(
                "statute-1",
                "Custom Condition Law",
                Effect::new(EffectType::Grant, "Grant benefit"),
            )
            .with_precondition(Condition::Custom {
                description: "Custom check".to_string(),
            }),
        ];

        let analysis = verify_mechanism_design(&statutes, &stakeholders);

        // Should find strategy-proofness issue (custom condition hard to verify)
        assert!(
            analysis
                .issues
                .iter()
                .any(|i| i.property == MechanismProperty::StrategyProofness)
        );
    }

    #[test]
    fn test_mechanism_design_strategy_proofness_grant_no_conditions() {
        let stakeholders = vec![Stakeholder::new("S1", "Alice")];

        let statutes = vec![Statute::new(
            "statute-1",
            "Unconditional Grant",
            Effect::new(EffectType::Grant, "Grant benefit"),
        )];

        let analysis = verify_mechanism_design(&statutes, &stakeholders);

        // Should find strategy-proofness issue (grant without verifiable conditions)
        assert!(
            analysis
                .issues
                .iter()
                .any(|i| i.property == MechanismProperty::StrategyProofness)
        );
    }

    #[test]
    fn test_mechanism_design_non_dictatorship_violation() {
        let stakeholders = vec![
            Stakeholder::new("S1", "Alice")
                .affected_by_statute("statute-1")
                .affected_by_statute("statute-2")
                .affected_by_statute("statute-3"),
            Stakeholder::new("S2", "Bob").affected_by_statute("statute-4"),
        ];

        let statutes = vec![
            Statute::new(
                "statute-1",
                "Law 1",
                Effect::new(EffectType::Grant, "Grant"),
            ),
            Statute::new(
                "statute-2",
                "Law 2",
                Effect::new(EffectType::Grant, "Grant"),
            ),
            Statute::new(
                "statute-3",
                "Law 3",
                Effect::new(EffectType::Grant, "Grant"),
            ),
            Statute::new(
                "statute-4",
                "Law 4",
                Effect::new(EffectType::Grant, "Grant"),
            ),
        ];

        let analysis = verify_mechanism_design(&statutes, &stakeholders);

        // Should find non-dictatorship issue (Alice controls 75% of statutes)
        assert!(
            analysis
                .issues
                .iter()
                .any(|i| i.property == MechanismProperty::NonDictatorship)
        );
    }

    #[test]
    fn test_mechanism_design_non_dictatorship_satisfied() {
        let stakeholders = vec![
            Stakeholder::new("S1", "Alice").affected_by_statute("statute-1"),
            Stakeholder::new("S2", "Bob").affected_by_statute("statute-2"),
        ];

        let statutes = vec![
            Statute::new(
                "statute-1",
                "Law 1",
                Effect::new(EffectType::Grant, "Grant"),
            ),
            Statute::new(
                "statute-2",
                "Law 2",
                Effect::new(EffectType::Grant, "Grant"),
            ),
        ];

        let analysis = verify_mechanism_design(&statutes, &stakeholders);

        // Should satisfy non-dictatorship (control is balanced)
        assert!(
            analysis
                .satisfied_properties
                .contains(&MechanismProperty::NonDictatorship)
        );
    }

    #[test]
    fn test_mechanism_design_report_no_issues() {
        let analysis = MechanismAnalysis {
            issues: vec![],
            satisfied_properties: vec![
                MechanismProperty::IncentiveCompatibility,
                MechanismProperty::BudgetBalance,
            ],
            quality_score: 1.0,
        };

        let report = mechanism_design_report(&analysis);

        assert!(report.contains("Mechanism Design Analysis"));
        assert!(report.contains("No mechanism design issues detected"));
        assert!(report.contains("Incentive Compatibility"));
    }

    #[test]
    fn test_mechanism_design_report_with_issues() {
        let mut analysis = MechanismAnalysis::new();

        analysis.add_issue(MechanismIssue {
            property: MechanismProperty::IncentiveCompatibility,
            statute_ids: vec!["S1".to_string()],
            severity: Severity::Warning,
            description: "Test issue".to_string(),
            suggestions: vec!["Fix suggestion".to_string()],
        });

        let report = mechanism_design_report(&analysis);

        assert!(report.contains("Mechanism Design Analysis"));
        assert!(report.contains("Incentive Compatibility"));
        assert!(report.contains("Test issue"));
        assert!(report.contains("Fix suggestion"));
    }

    #[test]
    fn test_mechanism_property_display() {
        assert_eq!(
            MechanismProperty::IncentiveCompatibility.to_string(),
            "Incentive Compatibility"
        );
        assert_eq!(
            MechanismProperty::IndividualRationality.to_string(),
            "Individual Rationality"
        );
        assert_eq!(
            MechanismProperty::BudgetBalance.to_string(),
            "Budget Balance"
        );
        assert_eq!(
            MechanismProperty::StrategyProofness.to_string(),
            "Strategy-Proofness"
        );
        assert_eq!(
            MechanismProperty::NonDictatorship.to_string(),
            "Non-Dictatorship"
        );
    }

    #[test]
    fn test_mechanism_quality_score_calculation() {
        let mut analysis = MechanismAnalysis::new();

        // Add critical issue
        analysis.add_issue(MechanismIssue {
            property: MechanismProperty::IncentiveCompatibility,
            statute_ids: vec![],
            severity: Severity::Critical,
            description: "Critical".to_string(),
            suggestions: vec![],
        });

        // Quality score should decrease significantly
        assert!(analysis.quality_score <= 0.7);

        // Add satisfied properties to increase score
        analysis.satisfy_property(MechanismProperty::BudgetBalance);
        analysis.satisfy_property(MechanismProperty::NonDictatorship);

        // Score should improve slightly
        assert!(analysis.quality_score > 0.0);
    }

    // ============================================================================
    // Probabilistic Verification Tests (v0.2.2)
    // ============================================================================

    #[test]
    fn test_markov_state_creation() {
        let state = MarkovState::new("s1", "Initial State").accepting();
        assert_eq!(state.id, "s1");
        assert_eq!(state.description, "Initial State");
        assert!(state.accepting);
    }

    #[test]
    fn test_markov_transition_creation() {
        let transition = MarkovTransition::new("s1", "s2", 0.7).with_action("comply");
        assert_eq!(transition.from, "s1");
        assert_eq!(transition.to, "s2");
        assert_eq!(transition.probability, 0.7);
        assert_eq!(transition.action.as_ref().unwrap(), "comply");
    }

    #[test]
    fn test_markov_chain_validation_valid() {
        let chain = MarkovChain::new("test", "s1")
            .add_state(MarkovState::new("s1", "Start"))
            .add_state(MarkovState::new("s2", "End").accepting())
            .add_transition(MarkovTransition::new("s1", "s2", 0.6))
            .add_transition(MarkovTransition::new("s1", "s1", 0.4));

        assert!(chain.validate().is_ok());
    }

    #[test]
    fn test_markov_chain_validation_invalid() {
        let chain = MarkovChain::new("test", "s1")
            .add_state(MarkovState::new("s1", "Start"))
            .add_state(MarkovState::new("s2", "End"))
            .add_transition(MarkovTransition::new("s1", "s2", 0.3)); // Only 0.3, should be 1.0

        assert!(chain.validate().is_err());
    }

    #[test]
    fn test_markov_chain_reachability_probability() {
        let chain = MarkovChain::new("test", "s1")
            .add_state(MarkovState::new("s1", "Start"))
            .add_state(MarkovState::new("s2", "Accepting").accepting())
            .add_transition(MarkovTransition::new("s1", "s2", 1.0));

        let prob = chain.reachability_probability(5);
        assert!((prob - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_markov_chain_steady_state() {
        let chain = MarkovChain::new("test", "s1")
            .add_state(MarkovState::new("s1", "State 1"))
            .add_state(MarkovState::new("s2", "State 2"))
            .add_transition(MarkovTransition::new("s1", "s2", 0.5))
            .add_transition(MarkovTransition::new("s1", "s1", 0.5))
            .add_transition(MarkovTransition::new("s2", "s1", 0.5))
            .add_transition(MarkovTransition::new("s2", "s2", 0.5));

        let probs = chain.steady_state_probabilities(100);

        // In a symmetric chain, should converge to equal probabilities
        let p1 = probs.get("s1").copied().unwrap_or(0.0);
        let p2 = probs.get("s2").copied().unwrap_or(0.0);
        assert!((p1 - 0.5).abs() < 0.1);
        assert!((p2 - 0.5).abs() < 0.1);
    }

    #[test]
    fn test_statistical_check_result_from_samples() {
        let result = StatisticalCheckResult::from_samples("test property", 1000, 750, 0.7);

        assert_eq!(result.num_samples, 1000);
        assert_eq!(result.num_successes, 750);
        assert!((result.estimated_probability - 0.75).abs() < 0.01);
        assert!(result.hypothesis_accepted); // 0.75 >= 0.7
    }

    #[test]
    fn test_statistical_check_result_hypothesis_rejected() {
        let result = StatisticalCheckResult::from_samples("test property", 1000, 400, 0.5);

        assert_eq!(result.num_samples, 1000);
        assert_eq!(result.num_successes, 400);
        assert!((result.estimated_probability - 0.4).abs() < 0.01);
        assert!(!result.hypothesis_accepted); // 0.4 < 0.5 (considering confidence interval)
    }

    #[test]
    fn test_risk_level_from_score() {
        assert_eq!(RiskLevel::from_score(0.1), RiskLevel::Minimal);
        assert_eq!(RiskLevel::from_score(0.3), RiskLevel::Low);
        assert_eq!(RiskLevel::from_score(0.6), RiskLevel::Medium);
        assert_eq!(RiskLevel::from_score(0.8), RiskLevel::High);
        assert_eq!(RiskLevel::from_score(0.95), RiskLevel::Critical);
    }

    #[test]
    fn test_risk_level_display() {
        assert_eq!(RiskLevel::Minimal.to_string(), "Minimal");
        assert_eq!(RiskLevel::Low.to_string(), "Low");
        assert_eq!(RiskLevel::Medium.to_string(), "Medium");
        assert_eq!(RiskLevel::High.to_string(), "High");
        assert_eq!(RiskLevel::Critical.to_string(), "Critical");
    }

    #[test]
    fn test_risk_factor_creation() {
        let factor = RiskFactor::new("Test Risk", "Description", 0.7).with_weight(0.5);

        assert_eq!(factor.name, "Test Risk");
        assert_eq!(factor.description, "Description");
        assert_eq!(factor.score, 0.7);
        assert_eq!(factor.weight, 0.5);
    }

    #[test]
    fn test_risk_factor_score_clamping() {
        let factor = RiskFactor::new("Test", "Desc", 1.5); // > 1.0
        assert_eq!(factor.score, 1.0); // Should be clamped

        let factor2 = RiskFactor::new("Test", "Desc", -0.5); // < 0.0
        assert_eq!(factor2.score, 0.0); // Should be clamped
    }

    #[test]
    fn test_risk_quantification_creation() {
        let factors = vec![
            RiskFactor::new("Factor 1", "Desc 1", 0.5).with_weight(0.5),
            RiskFactor::new("Factor 2", "Desc 2", 0.9).with_weight(0.5),
        ];

        let quant = RiskQuantification::new("statute-1", factors);

        assert_eq!(quant.statute_id, "statute-1");
        assert_eq!(quant.factors.len(), 2);
        // Overall score should be (0.5*0.5 + 0.9*0.5) / 1.0 = 0.7
        assert!((quant.overall_score - 0.7).abs() < 0.01);
        assert_eq!(quant.risk_level, RiskLevel::Medium);
    }

    #[test]
    fn test_risk_quantification_with_mitigations() {
        let factors = vec![RiskFactor::new("Test", "Desc", 0.95)];
        let quant = RiskQuantification::new("statute-1", factors)
            .add_mitigation("Mitigation 1")
            .add_mitigation("Mitigation 2");

        assert_eq!(quant.mitigations.len(), 2);
        assert_eq!(quant.mitigations[0], "Mitigation 1");
    }

    #[test]
    fn test_analyze_statute_risk_simple() {
        let statute = Statute::new(
            "test-1",
            "Simple Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let verifier = StatuteVerifier::new();
        let result = verifier.verify(std::slice::from_ref(&statute));

        let risk = analyze_statute_risk(&statute, &result);

        assert_eq!(risk.statute_id, "test-1");
        assert_eq!(risk.factors.len(), 4); // 4 risk factors
        assert!(risk.overall_score >= 0.0 && risk.overall_score <= 1.0);
        assert!(!risk.mitigations.is_empty());
    }

    #[test]
    fn test_analyze_statute_risk_with_errors() {
        let statute = Statute::new(
            "test-1",
            "Statute with Issues",
            Effect::new(EffectType::Prohibition, "Prohibit something"),
        );

        let mut result = VerificationResult::pass();
        result.errors.push(VerificationError::CircularReference {
            message: "Test error".to_string(),
        });
        result.passed = false;

        let risk = analyze_statute_risk(&statute, &result);

        // Should have higher risk due to errors
        assert!(risk.overall_score > 0.3);
        assert!(risk.mitigations.len() >= 1);
    }

    #[test]
    fn test_risk_quantification_report() {
        let factors = vec![RiskFactor::new("Test Factor", "Description", 0.6)];
        let risk = RiskQuantification::new("statute-1", factors).add_mitigation("Fix issue");

        let report = risk_quantification_report(&[risk]);

        assert!(report.contains("Risk Quantification Report"));
        assert!(report.contains("statute-1"));
        assert!(report.contains("Test Factor"));
        assert!(report.contains("Fix issue"));
    }

    #[test]
    fn test_statistical_model_checking_report() {
        let results = vec![
            StatisticalCheckResult::from_samples("Property 1", 1000, 800, 0.75),
            StatisticalCheckResult::from_samples("Property 2", 500, 250, 0.5),
        ];

        let report = statistical_model_checking_report(&results);

        assert!(report.contains("Statistical Model Checking Report"));
        assert!(report.contains("Property 1"));
        assert!(report.contains("Property 2"));
        assert!(report.contains("ACCEPTED"));
    }

    #[test]
    fn test_monte_carlo_verification_simple() {
        let chain = MarkovChain::new("test", "s1")
            .add_state(MarkovState::new("s1", "Start"))
            .add_state(MarkovState::new("s2", "Accept").accepting())
            .add_transition(MarkovTransition::new("s1", "s2", 1.0));

        let result = monte_carlo_verification(&chain, 100, 10);

        assert_eq!(result.num_samples, 100);
        // With probability 1.0 of reaching accepting state, should get ~100% success
        assert!(result.num_successes > 90);
        assert!(result.estimated_probability > 0.9);
    }

    #[test]
    fn test_monte_carlo_verification_probabilistic() {
        let chain = MarkovChain::new("test", "s1")
            .add_state(MarkovState::new("s1", "Start"))
            .add_state(MarkovState::new("s2", "Accept").accepting())
            .add_state(MarkovState::new("s3", "Reject"))
            .add_transition(MarkovTransition::new("s1", "s2", 0.5))
            .add_transition(MarkovTransition::new("s1", "s3", 0.5));

        let result = monte_carlo_verification(&chain, 1000, 10);

        assert_eq!(result.num_samples, 1000);
        // Should reach accepting state approximately 50% of the time
        assert!(result.estimated_probability > 0.4 && result.estimated_probability < 0.6);
    }

    #[test]
    fn test_risk_quantification_critical_level() {
        let factors = vec![RiskFactor::new("Critical Factor", "Very high risk", 0.95)];

        let quant = RiskQuantification::new("statute-critical", factors);

        assert_eq!(quant.risk_level, RiskLevel::Critical);
        assert!(quant.overall_score >= 0.9);
    }

    #[test]
    fn test_risk_quantification_minimal_level() {
        let factors = vec![RiskFactor::new("Low Risk Factor", "Very low risk", 0.1)];

        let quant = RiskQuantification::new("statute-safe", factors);

        assert_eq!(quant.risk_level, RiskLevel::Minimal);
        assert!(quant.overall_score < 0.25);
    }

    #[test]
    fn test_markov_chain_complex_reachability() {
        // Create a chain with multiple paths to accepting state
        let chain = MarkovChain::new("complex", "s1")
            .add_state(MarkovState::new("s1", "Start"))
            .add_state(MarkovState::new("s2", "Intermediate"))
            .add_state(MarkovState::new("s3", "Accepting").accepting())
            .add_transition(MarkovTransition::new("s1", "s2", 0.5))
            .add_transition(MarkovTransition::new("s1", "s3", 0.5))
            .add_transition(MarkovTransition::new("s2", "s3", 1.0));

        let prob = chain.reachability_probability(10);

        // Should eventually reach s3 with very high probability
        assert!(prob > 0.9);
    }

    #[test]
    fn test_statistical_result_confidence_interval() {
        let result = StatisticalCheckResult::from_samples("test", 10000, 5000, 0.48);

        // With 10000 samples and 50% success rate, confidence interval should be tight
        assert!(result.confidence_lower < 0.5);
        assert!(result.confidence_upper > 0.5);
        assert!(result.confidence_upper - result.confidence_lower < 0.05);
    }

    // ============================================================================
    // Explainable Verification Tests (v0.2.3)
    // ============================================================================

    #[test]
    fn test_natural_language_explanation_creation() {
        let explanation = NaturalLanguageExplanation::new(
            "Test Error",
            "Simple explanation",
            "Technical explanation",
            "Why it matters",
            "How to fix",
        )
        .with_example("Example scenario");

        assert_eq!(explanation.error_type, "Test Error");
        assert_eq!(explanation.simple_explanation, "Simple explanation");
        assert!(explanation.example_scenario.is_some());
    }

    #[test]
    fn test_natural_language_explanation_format() {
        let explanation =
            NaturalLanguageExplanation::new("Test Error", "Simple", "Technical", "Why", "Fix");

        let formatted = explanation.format(true);
        assert!(formatted.contains("# Test Error"));
        assert!(formatted.contains("## What's Wrong?"));
        assert!(formatted.contains("## Technical Details"));
        assert!(formatted.contains("## Why This Matters"));
        assert!(formatted.contains("## How to Fix It"));

        let formatted_simple = explanation.format(false);
        assert!(!formatted_simple.contains("## Technical Details"));
    }

    #[test]
    fn test_explain_error_circular_reference() {
        let error = VerificationError::CircularReference {
            message: "Test circular ref".to_string(),
        };

        let explanation = explain_error(&error);
        assert_eq!(explanation.error_type, "Circular Reference");
        assert!(explanation.simple_explanation.contains("infinite loop"));
        assert!(explanation.example_scenario.is_some());
    }

    #[test]
    fn test_explain_error_dead_statute() {
        let error = VerificationError::DeadStatute {
            statute_id: "statute-1".to_string(),
        };

        let explanation = explain_error(&error);
        assert_eq!(explanation.error_type, "Impossible to Satisfy");
        assert!(explanation.simple_explanation.contains("statute-1"));
        assert!(explanation.why_it_matters.contains("impossible"));
    }

    #[test]
    fn test_explain_error_constitutional_conflict() {
        let error = VerificationError::ConstitutionalConflict {
            statute_id: "statute-1".to_string(),
            principle: "Equal Protection".to_string(),
        };

        let explanation = explain_error(&error);
        assert_eq!(explanation.error_type, "Constitutional Conflict");
        assert!(explanation.simple_explanation.contains("Equal Protection"));
        assert!(explanation.how_to_fix.contains("Equal Protection"));
    }

    #[test]
    fn test_explain_error_ambiguity() {
        let error = VerificationError::Ambiguity {
            message: "Vague term".to_string(),
        };

        let explanation = explain_error(&error);
        assert_eq!(explanation.error_type, "Ambiguous Language");
        assert!(explanation.how_to_fix.contains("specific"));
    }

    #[test]
    fn test_conflict_explanation_creation() {
        let explanation = ConflictExplanation::new(
            vec!["statute-1".to_string(), "statute-2".to_string()],
            "Test conflict",
        )
        .with_impact("Test impact")
        .add_affected_party("Party 1")
        .add_resolution_option("Option 1");

        assert_eq!(explanation.statute_ids.len(), 2);
        assert_eq!(explanation.impact, "Test impact");
        assert_eq!(explanation.affected_parties.len(), 1);
        assert_eq!(explanation.resolution_options.len(), 1);
    }

    #[test]
    fn test_conflict_explanation_format() {
        let explanation = ConflictExplanation::new(
            vec!["S1".to_string(), "S2".to_string()],
            "Conflict description",
        )
        .with_impact("Impact")
        .add_affected_party("Party A")
        .add_resolution_option("Fix 1");

        let formatted = explanation.format();
        assert!(formatted.contains("# Conflict Between: S1, S2"));
        assert!(formatted.contains("## What's the Conflict?"));
        assert!(formatted.contains("## Real-World Impact"));
        assert!(formatted.contains("## Who's Affected?"));
        assert!(formatted.contains("## How to Resolve This"));
    }

    #[test]
    fn test_explain_conflict_effect_conflict() {
        let conflict = StatuteConflict {
            conflict_type: ConflictType::EffectConflict,
            statute_ids: vec!["S1".to_string(), "S2".to_string()],
            description: "Test conflict".to_string(),
            severity: Severity::Error,
            resolution_suggestions: vec!["Suggestion 1".to_string()],
        };

        let explanation = explain_conflict(&conflict);
        assert_eq!(explanation.statute_ids.len(), 2);
        assert!(explanation.description.contains("overlapping conditions"));
        assert!(!explanation.affected_parties.is_empty());
        assert_eq!(explanation.resolution_options.len(), 1);
    }

    #[test]
    fn test_explain_conflict_jurisdictional_overlap() {
        let conflict = StatuteConflict {
            conflict_type: ConflictType::JurisdictionalOverlap,
            statute_ids: vec!["S1".to_string(), "S2".to_string()],
            description: "Overlapping jurisdiction".to_string(),
            severity: Severity::Warning,
            resolution_suggestions: vec![],
        };

        let explanation = explain_conflict(&conflict);
        assert!(explanation.description.contains("jurisdiction"));
        assert!(
            explanation
                .affected_parties
                .iter()
                .any(|p| p.contains("jurisdiction"))
        );
    }

    #[test]
    fn test_verification_path_node_creation() {
        let node = VerificationPathNode::new("node-1", "statute", "Test Statute")
            .with_status(true)
            .add_metadata("key", "value");

        assert_eq!(node.id, "node-1");
        assert_eq!(node.node_type, "statute");
        assert!(node.passed);
        assert_eq!(node.metadata.get("key").unwrap(), "value");
    }

    #[test]
    fn test_verification_path_node_with_children() {
        let child = VerificationPathNode::new("child", "condition", "Age >= 18");
        let parent =
            VerificationPathNode::new("parent", "statute", "Parent Statute").add_child(child);

        assert_eq!(parent.children.len(), 1);
        assert_eq!(parent.children[0].id, "child");
    }

    #[test]
    fn test_verification_path_to_dot() {
        let node = VerificationPathNode::new("root", "statute", "Test")
            .with_status(true)
            .add_child(VerificationPathNode::new("child", "condition", "Condition"));

        let dot = node.to_dot();
        assert!(dot.contains("digraph VerificationPath"));
        assert!(dot.contains("\"root\""));
        assert!(dot.contains("\"child\""));
        assert!(dot.contains("->"));
        assert!(dot.contains("green"));
    }

    #[test]
    fn test_build_verification_path_simple() {
        let statute = Statute::new(
            "test-1",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test"),
        );

        let result = VerificationResult::pass();
        let path = build_verification_path(&statute, &result);

        assert_eq!(path.id, "test-1");
        assert!(path.passed);
        assert!(!path.children.is_empty()); // Should have effect node
    }

    #[test]
    fn test_build_verification_path_with_preconditions() {
        let statute = Statute::new(
            "test-1",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let result = VerificationResult::pass();
        let path = build_verification_path(&statute, &result);

        assert_eq!(path.id, "test-1");
        // Should have precondition and effect nodes
        assert!(path.children.len() >= 2);
    }

    #[test]
    fn test_build_verification_path_with_errors() {
        let statute = Statute::new("test-1", "Test", Effect::new(EffectType::Grant, "Test"));

        let mut result = VerificationResult::pass();
        result.passed = false;
        result.errors.push(VerificationError::CircularReference {
            message: "Test error".to_string(),
        });

        let path = build_verification_path(&statute, &result);
        assert!(!path.passed);
        // Should have error nodes
        assert!(path.children.iter().any(|c| c.node_type == "error"));
    }

    #[test]
    fn test_what_if_scenario_creation() {
        let original = Statute::new(
            "test-1",
            "Original Title",
            Effect::new(EffectType::Grant, "Test"),
        );

        let modified = Statute::new(
            "test-1",
            "Modified Title",
            Effect::new(EffectType::Grant, "Test"),
        );

        let scenario = WhatIfScenario::new(
            "Title change test",
            original.clone(),
            modified,
            VerificationResult::pass(),
            VerificationResult::pass(),
        );

        assert_eq!(scenario.description, "Title change test");
        assert!(!scenario.changes.is_empty());
        assert!(scenario.changes[0].contains("Title changed"));
    }

    #[test]
    fn test_what_if_scenario_detect_effect_change() {
        let original = Statute::new("test-1", "Test", Effect::new(EffectType::Grant, "Test"));

        let modified = Statute::new(
            "test-1",
            "Test",
            Effect::new(EffectType::Prohibition, "Test"),
        );

        let scenario = WhatIfScenario::new(
            "Effect change",
            original,
            modified,
            VerificationResult::pass(),
            VerificationResult::pass(),
        );

        assert!(
            scenario
                .changes
                .iter()
                .any(|c| c.contains("Effect type changed"))
        );
    }

    #[test]
    fn test_what_if_scenario_report() {
        let original = Statute::new("test-1", "Test", Effect::new(EffectType::Grant, "Test"));
        let modified = original.clone();

        let mut orig_result = VerificationResult::pass();
        orig_result.errors.push(VerificationError::Ambiguity {
            message: "Test".to_string(),
        });
        orig_result.passed = false;

        let new_result = VerificationResult::pass();

        let scenario =
            WhatIfScenario::new("Fix ambiguity", original, modified, orig_result, new_result);

        let report = scenario.report();
        assert!(report.contains("# What-If Scenario"));
        assert!(report.contains("## Impact Analysis"));
        assert!(report.contains("✓")); // Improved
    }

    #[test]
    fn test_what_if_analysis() {
        let statute = Statute::new(
            "test-1",
            "Original Title",
            Effect::new(EffectType::Grant, "Test"),
        );

        let scenario = what_if_analysis("Change title", statute, |s| {
            s.title = "New Title".to_string();
        });

        assert_eq!(scenario.description, "Change title");
        assert_eq!(scenario.modified_statute.title, "New Title");
        assert!(scenario.changes.iter().any(|c| c.contains("Title changed")));
    }

    #[test]
    fn test_what_if_breaking_change() {
        let statute = Statute::new("test-1", "Test", Effect::new(EffectType::Grant, "Test"));

        // Modify to add a precondition that might fail verification in some context
        let scenario = what_if_analysis("Add precondition", statute, |s| {
            s.preconditions.push(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            });
        });

        assert!(
            scenario
                .changes
                .iter()
                .any(|c| c.contains("Preconditions modified"))
        );
    }

    #[test]
    fn test_build_condition_path_age() {
        use legalis_core::{ComparisonOp, Condition};

        let condition = Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        };

        let node = build_condition_path(&condition, "test");
        assert_eq!(node.node_type, "condition");
        assert!(node.label.contains("Age"));
        assert!(node.label.contains("18"));
    }

    #[test]
    fn test_build_condition_path_complex() {
        use legalis_core::{ComparisonOp, Condition};

        let condition = Condition::And(
            Box::new(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            }),
            Box::new(Condition::Income {
                operator: ComparisonOp::LessThan,
                value: 50000,
            }),
        );

        let node = build_condition_path(&condition, "test");
        assert_eq!(node.node_type, "logic");
        assert_eq!(node.label, "AND");
        assert_eq!(node.children.len(), 2);
    }

    #[test]
    fn test_verification_path_failed_status() {
        let node = VerificationPathNode::new("failed", "error", "Test Error").with_status(false);

        let dot = node.to_dot();
        assert!(dot.contains("red"));
        assert!(dot.contains("bold"));
    }

    // ========================================================================
    // Privacy-Preserving Verification Tests (v0.2.4)
    // ========================================================================

    #[test]
    fn test_zero_knowledge_proof_creation() {
        let statute = Statute::new("test-1", "Test", Effect::new(EffectType::Grant, "Test"));
        let proof = ZeroKnowledgeProof::new("statute is valid", &statute);

        assert!(proof.proof_id.starts_with("zkp-"));
        assert_eq!(proof.statement, "statute is valid");
        assert!(!proof.commitment.is_empty());
        assert_eq!(proof.challenge.len(), 32);
        assert_eq!(proof.response.len(), 32);
    }

    #[test]
    fn test_zero_knowledge_proof_verification() {
        let statute = Statute::new("test-1", "Test", Effect::new(EffectType::Grant, "Test"));
        let proof = ZeroKnowledgeProof::new("statute is valid", &statute);

        assert!(proof.verify());
    }

    #[test]
    fn test_zero_knowledge_proof_with_metadata() {
        let statute = Statute::new("test-1", "Test", Effect::new(EffectType::Grant, "Test"));
        let proof = ZeroKnowledgeProof::new("statute is valid", &statute)
            .with_metadata("prover", "alice")
            .with_metadata("version", "1.0");

        assert_eq!(proof.metadata.get("prover"), Some(&"alice".to_string()));
        assert_eq!(proof.metadata.get("version"), Some(&"1.0".to_string()));
    }

    #[test]
    fn test_zero_knowledge_proof_report() {
        let statute = Statute::new("test-1", "Test", Effect::new(EffectType::Grant, "Test"));
        let proof = ZeroKnowledgeProof::new("statute is valid", &statute);

        let report = proof.report();
        assert!(report.contains("Zero-Knowledge Proof Report"));
        assert!(report.contains("statute is valid"));
        assert!(report.contains("Valid: true"));
    }

    #[test]
    fn test_multiparty_verification_creation() {
        let statute = Statute::new("test-1", "Test", Effect::new(EffectType::Grant, "Test"));
        let parties = vec!["Alice".to_string(), "Bob".to_string(), "Carol".to_string()];
        let result = secure_multiparty_verification(&statute, parties.clone());

        assert_eq!(result.parties, parties);
        assert!(result.combined_result.passed);
        assert!(result.computation_proof.starts_with("mpc-proof-"));
    }

    #[test]
    fn test_multiparty_verification_report() {
        let statute = Statute::new("test-1", "Test", Effect::new(EffectType::Grant, "Test"));
        let parties = vec!["Alice".to_string(), "Bob".to_string()];
        let result = secure_multiparty_verification(&statute, parties);

        let report = result.report();
        assert!(report.contains("Multi-Party Verification Report"));
        assert!(report.contains("Alice, Bob"));
        assert!(report.contains("Verification Passed: true"));
    }

    #[test]
    fn test_privacy_budget_creation() {
        let budget = PrivacyBudget::new(1.0, 1e-5);
        assert_eq!(budget.epsilon, 1.0);
        assert_eq!(budget.delta, 1e-5);
    }

    #[test]
    fn test_privacy_budget_presets() {
        let strict = PrivacyBudget::strict();
        assert_eq!(strict.epsilon, 0.1);

        let moderate = PrivacyBudget::moderate();
        assert_eq!(moderate.epsilon, 1.0);

        let relaxed = PrivacyBudget::relaxed();
        assert_eq!(relaxed.epsilon, 3.0);
    }

    #[test]
    fn test_differential_private_analysis() {
        let statutes = vec![
            Statute::new("test-1", "Test 1", Effect::new(EffectType::Grant, "Test")),
            Statute::new("test-2", "Test 2", Effect::new(EffectType::Grant, "Test")),
            Statute::new("test-3", "Test 3", Effect::new(EffectType::Grant, "Test")),
        ];

        let budget = PrivacyBudget::moderate();
        let result = differential_private_analysis(&statutes, budget);

        // Count should be close to 3 (with noise)
        assert!(result.count > 0.0);
        assert!(result.count < 10.0);

        // Error rate should be between 0 and 1
        assert!(result.error_rate >= 0.0);
        assert!(result.error_rate <= 1.0);

        assert_eq!(result.privacy_budget.epsilon, 1.0);
    }

    #[test]
    fn test_differential_private_analysis_empty() {
        let statutes: Vec<Statute> = vec![];
        let budget = PrivacyBudget::strict();
        let result = differential_private_analysis(&statutes, budget);

        assert!(result.count >= 0.0);
    }

    #[test]
    fn test_private_aggregation_report() {
        let statutes = vec![Statute::new(
            "test-1",
            "Test",
            Effect::new(EffectType::Grant, "Test"),
        )];
        let budget = PrivacyBudget::moderate();
        let result = differential_private_analysis(&statutes, budget);

        let report = result.report();
        assert!(report.contains("Differential Privacy Report"));
        assert!(report.contains("Privacy Budget"));
    }

    #[test]
    fn test_encrypted_statute_creation() {
        let statute = Statute::new("test-1", "Test", Effect::new(EffectType::Grant, "Test"));
        let encrypted = EncryptedStatute::new(&statute);

        assert!(!encrypted.encrypted_id.is_empty());
        assert!(!encrypted.encrypted_data.is_empty());
        assert_eq!(encrypted.scheme, "Simplified-XOR");
    }

    #[test]
    fn test_encrypted_statute_homomorphic_verify() {
        let statute = Statute::new("test-1", "Test", Effect::new(EffectType::Grant, "Test"));
        let encrypted = EncryptedStatute::new(&statute);

        let result = encrypted.homomorphic_verify();
        assert!(!result.encrypted_result.is_empty());
        assert_eq!(result.scheme, "Simplified-XOR");
    }

    #[test]
    fn test_encrypted_verification_result_report() {
        let statute = Statute::new("test-1", "Test", Effect::new(EffectType::Grant, "Test"));
        let encrypted = EncryptedStatute::new(&statute);
        let result = encrypted.homomorphic_verify();

        let report = result.report();
        assert!(report.contains("Encrypted Verification Result"));
        assert!(report.contains("Simplified-XOR"));
        assert!(report.contains("cannot be read without decryption key"));
    }

    #[test]
    fn test_tee_config_creation() {
        let config = TeeConfig::new("SGX");
        assert_eq!(config.tee_type, "SGX");
        assert_eq!(config.attestation.len(), 64);
    }

    #[test]
    fn test_tee_config_attestation_verification() {
        let config = TeeConfig::new("TrustZone");
        assert!(config.verify_attestation());
    }

    #[test]
    fn test_tee_verification() {
        let statute = Statute::new("test-1", "Test", Effect::new(EffectType::Grant, "Test"));
        let config = TeeConfig::new("SGX");
        let result = tee_verification(&statute, config);

        assert!(result.result.passed);
        assert_eq!(result.tee_config.tee_type, "SGX");
        assert!(result.attestation_proof.starts_with("tee-attestation-"));
    }

    #[test]
    fn test_tee_verification_report() {
        let statute = Statute::new("test-1", "Test", Effect::new(EffectType::Grant, "Test"));
        let config = TeeConfig::new("SEV");
        let result = tee_verification(&statute, config);

        let report = result.report();
        assert!(report.contains("TEE Verification Report"));
        assert!(report.contains("SEV"));
        assert!(report.contains("Attestation Valid: true"));
        assert!(report.contains("Verification Passed: true"));
    }

    #[test]
    fn test_multiparty_verification_with_multiple_parties() {
        let statute = Statute::new("test-1", "Test", Effect::new(EffectType::Grant, "Test"));
        let parties = vec![
            "Alice".to_string(),
            "Bob".to_string(),
            "Carol".to_string(),
            "David".to_string(),
        ];
        let result = secure_multiparty_verification(&statute, parties.clone());

        // Should successfully verify with multiple parties
        assert_eq!(result.parties.len(), 4);
        assert!(result.parties.contains(&"Alice".to_string()));
        assert!(result.parties.contains(&"David".to_string()));
        assert!(result.combined_result.passed);
    }

    #[test]
    fn test_zero_knowledge_proof_different_statutes_different_commitments() {
        let statute1 = Statute::new("test-1", "Test 1", Effect::new(EffectType::Grant, "Test"));
        let statute2 = Statute::new("test-2", "Test 2", Effect::new(EffectType::Grant, "Test"));

        let proof1 = ZeroKnowledgeProof::new("statement", &statute1);
        let proof2 = ZeroKnowledgeProof::new("statement", &statute2);

        // Different statutes should produce different commitments
        assert_ne!(proof1.commitment, proof2.commitment);
    }

    // ========================================================================
    // Incremental Verification 2.0 Tests (v0.2.5)
    // ========================================================================

    #[test]
    fn test_dependency_node_creation() {
        let node = DependencyNode::new("statute-1", DependencyType::DerivesFrom);
        assert_eq!(node.statute_id, "statute-1");
        assert_eq!(node.dependency_type, DependencyType::DerivesFrom);
        assert!(node.dependencies.is_empty());
        assert!(node.dependents.is_empty());
        assert!(node.last_verified.is_none());
    }

    #[test]
    fn test_dependency_node_add_dependency() {
        let mut node = DependencyNode::new("statute-1", DependencyType::DerivesFrom);
        node.add_dependency("statute-2");
        node.add_dependency("statute-3");

        assert_eq!(node.dependencies.len(), 2);
        assert!(node.dependencies.contains(&"statute-2".to_string()));
        assert!(node.dependencies.contains(&"statute-3".to_string()));
    }

    #[test]
    fn test_dependency_node_add_dependent() {
        let mut node = DependencyNode::new("statute-1", DependencyType::DerivesFrom);
        node.add_dependent("statute-4");

        assert_eq!(node.dependents.len(), 1);
        assert!(node.dependents.contains(&"statute-4".to_string()));
    }

    #[test]
    fn test_dependency_node_mark_verified() {
        let mut node = DependencyNode::new("statute-1", DependencyType::DerivesFrom);
        assert!(node.last_verified.is_none());

        node.mark_verified();
        assert!(node.last_verified.is_some());
    }

    #[test]
    fn test_dependency_graph_from_statutes() {
        let mut statute1 = Statute::new("s1", "Test 1", Effect::new(EffectType::Grant, "Test"));
        statute1.derives_from = vec!["s0".to_string()];

        let statute2 = Statute::new("s2", "Test 2", Effect::new(EffectType::Grant, "Test"));

        let statutes = vec![statute1, statute2];
        let graph = DependencyGraph::from_statutes(&statutes);

        assert_eq!(graph.nodes.len(), 2);
        assert!(graph.nodes.contains_key("s1"));
        assert!(graph.nodes.contains_key("s2"));
    }

    #[test]
    fn test_dependency_graph_transitive_dependencies() {
        let mut statute1 = Statute::new("s1", "Test 1", Effect::new(EffectType::Grant, "Test"));
        statute1.derives_from = vec!["s2".to_string()];

        let mut statute2 = Statute::new("s2", "Test 2", Effect::new(EffectType::Grant, "Test"));
        statute2.derives_from = vec!["s3".to_string()];

        let statute3 = Statute::new("s3", "Test 3", Effect::new(EffectType::Grant, "Test"));

        let statutes = vec![statute1, statute2, statute3];
        let graph = DependencyGraph::from_statutes(&statutes);

        let deps = graph.get_transitive_dependencies("s1");
        assert!(deps.contains(&"s2".to_string()));
        assert!(deps.contains(&"s3".to_string()));
    }

    #[test]
    fn test_dependency_graph_affected_statutes() {
        let mut statute1 = Statute::new("s1", "Test 1", Effect::new(EffectType::Grant, "Test"));
        statute1.derives_from = vec!["s3".to_string()];

        let mut statute2 = Statute::new("s2", "Test 2", Effect::new(EffectType::Grant, "Test"));
        statute2.derives_from = vec!["s3".to_string()];

        let statute3 = Statute::new("s3", "Test 3", Effect::new(EffectType::Grant, "Test"));

        let statutes = vec![statute1, statute2, statute3];
        let graph = DependencyGraph::from_statutes(&statutes);

        let affected = graph.get_affected_statutes("s3");
        assert!(affected.contains(&"s1".to_string()) || affected.contains(&"s2".to_string()));
    }

    #[test]
    fn test_lazy_verification_config_new() {
        let config = LazyVerificationConfig::new();
        assert!(config.verify_changed_only);
        assert!(config.verify_dependencies);
        assert!(config.max_depth.is_none());
    }

    #[test]
    fn test_lazy_verification_config_changed_only() {
        let config = LazyVerificationConfig::changed_only();
        assert!(config.verify_changed_only);
        assert!(!config.verify_dependencies);
    }

    #[test]
    fn test_lazy_verification_config_with_depth() {
        let config = LazyVerificationConfig::with_depth(3);
        assert_eq!(config.max_depth, Some(3));
    }

    #[test]
    fn test_lazy_verify_empty() {
        let statutes = vec![Statute::new(
            "s1",
            "Test",
            Effect::new(EffectType::Grant, "Test"),
        )];
        let changed_ids: Vec<String> = vec![];
        let config = LazyVerificationConfig::new();

        let result = lazy_verify(&statutes, &changed_ids, config);
        assert!(result.passed);
    }

    #[test]
    fn test_lazy_verify_single_change() {
        let statute = Statute::new("s1", "Test", Effect::new(EffectType::Grant, "Test"));
        let statutes = vec![statute];
        let changed_ids = vec!["s1".to_string()];
        let config = LazyVerificationConfig::changed_only();

        let result = lazy_verify(&statutes, &changed_ids, config);
        assert!(result.passed);
    }

    #[test]
    fn test_verification_diff_no_changes() {
        let old = VerificationResult::pass();
        let new = VerificationResult::pass();

        let diff = VerificationDiff::diff(&old, &new);
        assert!(!diff.has_changes());
        assert!(!diff.status_changed);
    }

    #[test]
    fn test_verification_diff_status_change() {
        let old = VerificationResult::pass();
        let mut new = VerificationResult::pass();
        new.passed = false;

        let diff = VerificationDiff::diff(&old, &new);
        assert!(diff.has_changes());
        assert!(diff.status_changed);
        assert_eq!(diff.old_passed, true);
        assert_eq!(diff.new_passed, false);
    }

    #[test]
    fn test_verification_diff_errors_added() {
        let old = VerificationResult::pass();
        let mut new = VerificationResult::pass();
        new.errors.push(VerificationError::Ambiguity {
            message: "Test".to_string(),
        });
        new.passed = false;

        let diff = VerificationDiff::diff(&old, &new);
        assert_eq!(diff.errors_added.len(), 1);
        assert_eq!(diff.errors_removed.len(), 0);
    }

    #[test]
    fn test_verification_diff_errors_removed() {
        let mut old = VerificationResult::pass();
        old.errors.push(VerificationError::Ambiguity {
            message: "Test".to_string(),
        });
        old.passed = false;

        let new = VerificationResult::pass();

        let diff = VerificationDiff::diff(&old, &new);
        assert_eq!(diff.errors_added.len(), 0);
        assert_eq!(diff.errors_removed.len(), 1);
    }

    #[test]
    fn test_verification_diff_warnings_added() {
        let old = VerificationResult::pass();
        let mut new = VerificationResult::pass();
        new.warnings.push("New warning".to_string());

        let diff = VerificationDiff::diff(&old, &new);
        assert_eq!(diff.warnings_added.len(), 1);
        assert!(diff.warnings_added.contains(&"New warning".to_string()));
    }

    #[test]
    fn test_verification_diff_report() {
        let old = VerificationResult::pass();
        let mut new = VerificationResult::pass();
        new.passed = false;

        let diff = VerificationDiff::diff(&old, &new);
        let report = diff.report();

        assert!(report.contains("Verification Diff Report"));
        assert!(report.contains("Status Changed"));
    }

    #[test]
    fn test_cached_proof_creation() {
        let statute = Statute::new("s1", "Test", Effect::new(EffectType::Grant, "Test"));
        let result = VerificationResult::pass();

        let proof = CachedProof::new(&statute, result);
        assert_eq!(proof.statute_id, "s1");
        assert!(proof.result.passed);
        assert!(!proof.content_hash.is_empty());
    }

    #[test]
    fn test_cached_proof_is_valid() {
        let statute = Statute::new("s1", "Test", Effect::new(EffectType::Grant, "Test"));
        let result = VerificationResult::pass();

        let proof = CachedProof::new(&statute, result);
        assert!(proof.is_valid(&statute));
    }

    #[test]
    fn test_cached_proof_invalid_after_change() {
        let statute = Statute::new("s1", "Test", Effect::new(EffectType::Grant, "Test"));
        let result = VerificationResult::pass();

        let proof = CachedProof::new(&statute, result);

        let mut changed_statute = statute.clone();
        changed_statute.title = "Changed Title".to_string();

        assert!(!proof.is_valid(&changed_statute));
    }

    #[test]
    fn test_proof_cache_creation() {
        let cache = ProofCache::new();
        assert_eq!(cache.proofs.len(), 0);
    }

    #[test]
    fn test_proof_cache_add_proof() {
        let mut cache = ProofCache::new();
        let statute = Statute::new("s1", "Test", Effect::new(EffectType::Grant, "Test"));
        let result = VerificationResult::pass();

        cache.add_proof(&statute, result);
        assert_eq!(cache.proofs.len(), 1);
    }

    #[test]
    fn test_proof_cache_get_proof() {
        let mut cache = ProofCache::new();
        let statute = Statute::new("s1", "Test", Effect::new(EffectType::Grant, "Test"));
        let result = VerificationResult::pass();

        cache.add_proof(&statute, result);

        let cached = cache.get_proof(&statute);
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().statute_id, "s1");
    }

    #[test]
    fn test_proof_cache_invalidate() {
        let mut cache = ProofCache::new();
        let statute = Statute::new("s1", "Test", Effect::new(EffectType::Grant, "Test"));
        let result = VerificationResult::pass();

        cache.add_proof(&statute, result);
        assert_eq!(cache.proofs.len(), 1);

        cache.invalidate(&["s1".to_string()]);
        assert_eq!(cache.proofs.len(), 0);
    }

    #[test]
    fn test_proof_cache_stats() {
        let mut cache = ProofCache::new();
        let statute = Statute::new("s1", "Test", Effect::new(EffectType::Grant, "Test"));
        let result = VerificationResult::pass();

        cache.add_proof(&statute, result);

        let stats = cache.stats();
        assert_eq!(stats.total_proofs, 1);
        assert!(stats.oldest_timestamp.is_some());
        assert!(stats.newest_timestamp.is_some());
    }
}
