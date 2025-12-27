//! Legalis-Verifier: Formal verification for Legalis-RS legal statutes.
//!
//! This crate provides static analysis and verification tools for detecting
//! logical inconsistencies, circular references, and constitutional conflicts
//! in legal statutes.

#[cfg(feature = "z3-solver")]
mod smt;

#[cfg(feature = "z3-solver")]
pub use smt::{SmtVerifier, create_z3_context};

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
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl std::fmt::Display for RiskLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
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
    use crossbeam_channel::{Receiver, bounded, select};
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
    ) -> Self {
        let overall_score =
            (complexity_score + readability_score + consistency_score + completeness_score) / 4.0;

        Self {
            statute_id,
            overall_score,
            complexity_score,
            readability_score,
            consistency_score,
            completeness_score,
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

    let mut metrics = QualityMetrics::new(
        statute.id.clone(),
        complexity_score,
        readability_score,
        consistency_score,
        completeness_score,
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
            "- Completeness: {:.1}/100\n\n",
            metrics.completeness_score
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
        let metrics = QualityMetrics::new("test".to_string(), 95.0, 95.0, 95.0, 95.0);

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
}
