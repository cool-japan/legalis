//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use chrono::{DateTime, Datelike, NaiveDate, Utc};
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::types::{
    ConflictResolution, Contradiction, Effect, EntailmentResult, ErrorSeverity, ValidationError,
};
use super::types_3::{
    AttributeBasedContext, ConflictReason, ContradictionType, DiagnosticContext, EffectType,
};
use super::types_5::Condition;
use super::types_6::Statute;

/// Statute conflict analyzer.
///
/// Provides methods to detect and resolve conflicts between statutes
/// using established legal principles.
pub struct StatuteConflictAnalyzer;
impl StatuteConflictAnalyzer {
    /// Analyzes two statutes for conflicts and determines which should prevail.
    ///
    /// Uses the following hierarchy of resolution principles:
    /// 1. Explicit amendment relationships
    /// 2. Temporal precedence (newer laws)
    /// 3. Specificity (more specific laws)
    /// 4. Hierarchy (jurisdictional authority)
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType, StatuteConflictAnalyzer, TemporalValidity};
    /// use chrono::NaiveDate;
    ///
    /// let old_law = Statute::new("old-1", "Old Law", Effect::new(EffectType::Grant, "Old grant"))
    ///     .with_temporal_validity(
    ///         TemporalValidity::new()
    ///             .with_effective_date(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap())
    ///     )
    ///     .with_version(1);
    ///
    /// let new_law = Statute::new("new-1", "New Law", Effect::new(EffectType::Grant, "New grant"))
    ///     .with_temporal_validity(
    ///         TemporalValidity::new()
    ///             .with_effective_date(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())
    ///     )
    ///     .with_version(1);
    ///
    /// let resolution = StatuteConflictAnalyzer::resolve(&old_law, &new_law);
    /// // New law prevails by temporal precedence
    /// ```
    pub fn resolve(first: &Statute, second: &Statute) -> ConflictResolution {
        if !Self::has_conflict(first, second) {
            return ConflictResolution::NoConflict;
        }
        if let Some(resolution) = Self::check_temporal_precedence(first, second) {
            return resolution;
        }
        if let Some(resolution) = Self::check_specificity(first, second) {
            return resolution;
        }
        if let Some(resolution) = Self::check_hierarchy(first, second) {
            return resolution;
        }
        ConflictResolution::Unresolvable(
            "Statutes conflict but resolution requires human judgment".to_string(),
        )
    }
    /// Checks if two statutes have conflicting effects.
    fn has_conflict(first: &Statute, second: &Statute) -> bool {
        use EffectType::*;
        matches!(
            (&first.effect.effect_type, &second.effect.effect_type),
            (Grant, Prohibition)
                | (Grant, Revoke)
                | (Prohibition, Grant)
                | (Revoke, Grant)
                | (Obligation, Prohibition)
                | (Prohibition, Obligation)
        )
    }
    /// Applies lex posterior (later law prevails).
    fn check_temporal_precedence(first: &Statute, second: &Statute) -> Option<ConflictResolution> {
        let first_date = first.temporal_validity.effective_date?;
        let second_date = second.temporal_validity.effective_date?;
        if first_date > second_date {
            Some(ConflictResolution::FirstPrevails(
                ConflictReason::TemporalPrecedence,
            ))
        } else if second_date > first_date {
            Some(ConflictResolution::SecondPrevails(
                ConflictReason::TemporalPrecedence,
            ))
        } else {
            None
        }
    }
    /// Applies lex specialis (more specific law prevails).
    ///
    /// A statute is considered more specific if it has more preconditions.
    fn check_specificity(first: &Statute, second: &Statute) -> Option<ConflictResolution> {
        let first_specificity = Self::calculate_specificity(first);
        let second_specificity = Self::calculate_specificity(second);
        if first_specificity > second_specificity {
            Some(ConflictResolution::FirstPrevails(
                ConflictReason::Specificity,
            ))
        } else if second_specificity > first_specificity {
            Some(ConflictResolution::SecondPrevails(
                ConflictReason::Specificity,
            ))
        } else {
            None
        }
    }
    /// Calculates specificity score based on number and complexity of conditions.
    fn calculate_specificity(statute: &Statute) -> usize {
        statute
            .preconditions
            .iter()
            .map(|c| c.count_conditions())
            .sum()
    }
    /// Applies lex superior (higher authority prevails).
    ///
    /// Uses jurisdiction hierarchy: federal > state > local
    fn check_hierarchy(first: &Statute, second: &Statute) -> Option<ConflictResolution> {
        let first_level = Self::jurisdiction_level(&first.jurisdiction);
        let second_level = Self::jurisdiction_level(&second.jurisdiction);
        if first_level > second_level {
            Some(ConflictResolution::FirstPrevails(ConflictReason::Hierarchy))
        } else if second_level > first_level {
            Some(ConflictResolution::SecondPrevails(
                ConflictReason::Hierarchy,
            ))
        } else {
            None
        }
    }
    /// Determines jurisdiction hierarchy level.
    ///
    /// Higher number = higher authority
    pub fn jurisdiction_level(jurisdiction: &Option<String>) -> u32 {
        jurisdiction.as_ref().map_or(0, |j| {
            if j.to_lowercase().contains("federal") || j.to_lowercase().contains("national") {
                3
            } else if j.to_lowercase().contains("state") || j.to_lowercase().contains("provincial")
            {
                2
            } else if j.to_lowercase().contains("local") || j.to_lowercase().contains("municipal") {
                1
            } else {
                if j.len() <= 3 && j.chars().all(|c| c.is_ascii_uppercase()) {
                    3
                } else if j.contains('-') {
                    2
                } else {
                    0
                }
            }
        })
    }
    /// Checks if a statute is still in effect on a given date.
    pub fn is_in_effect(statute: &Statute, date: NaiveDate) -> bool {
        statute.temporal_validity.is_active(date)
    }
    /// Finds which statutes from a set apply to a given date and resolves conflicts.
    ///
    /// Returns statutes in order of precedence (highest priority first).
    pub fn resolve_conflicts_at_date(statutes: &[Statute], date: NaiveDate) -> Vec<&Statute> {
        let mut active: Vec<&Statute> = statutes
            .iter()
            .filter(|s| Self::is_in_effect(s, date))
            .collect();
        active.sort_by(|a, b| {
            let date_cmp = b
                .temporal_validity
                .effective_date
                .cmp(&a.temporal_validity.effective_date);
            if date_cmp != std::cmp::Ordering::Equal {
                return date_cmp;
            }
            let spec_cmp = Self::calculate_specificity(b).cmp(&Self::calculate_specificity(a));
            if spec_cmp != std::cmp::Ordering::Equal {
                return spec_cmp;
            }
            Self::jurisdiction_level(&b.jurisdiction)
                .cmp(&Self::jurisdiction_level(&a.jurisdiction))
        });
        active
    }
    /// Detects contradictions across a set of statutes.
    ///
    /// A contradiction occurs when:
    /// - Two statutes have conflicting effects (Grant vs Revoke) for the same thing
    /// - Two statutes have mutually exclusive preconditions but same effects
    /// - Statutes create logical inconsistencies in the legal system
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType, StatuteConflictAnalyzer, Condition, ComparisonOp};
    ///
    /// let grant = Statute::new("grant-1", "Grant Right", Effect::new(EffectType::Grant, "Voting"))
    ///     .with_precondition(Condition::age(ComparisonOp::GreaterOrEqual, 18));
    ///
    /// let revoke = Statute::new("revoke-1", "Revoke Right", Effect::new(EffectType::Revoke, "Voting"))
    ///     .with_precondition(Condition::age(ComparisonOp::GreaterOrEqual, 18));
    ///
    /// let statutes = vec![grant, revoke];
    /// let contradictions = StatuteConflictAnalyzer::detect_contradictions(&statutes);
    ///
    /// assert!(!contradictions.is_empty());
    /// ```
    #[must_use]
    pub fn detect_contradictions(statutes: &[Statute]) -> Vec<Contradiction> {
        let mut contradictions = Vec::new();
        for (i, statute_a) in statutes.iter().enumerate() {
            for statute_b in statutes.iter().skip(i + 1) {
                if Self::effects_contradict(&statute_a.effect, &statute_b.effect)
                    && Self::conditions_may_overlap(
                        &statute_a.preconditions,
                        &statute_b.preconditions,
                    )
                {
                    contradictions.push(Contradiction {
                        statute_a_id: statute_a.id.clone(),
                        statute_b_id: statute_b.id.clone(),
                        contradiction_type: ContradictionType::ConflictingEffects,
                        description: format!(
                            "Statute '{}' grants while '{}' revokes the same right",
                            statute_a.id, statute_b.id
                        ),
                        severity: ErrorSeverity::Critical,
                    });
                }
                if statute_a.preconditions == statute_b.preconditions
                    && statute_a.effect.effect_type != statute_b.effect.effect_type
                {
                    contradictions
                        .push(Contradiction {
                            statute_a_id: statute_a.id.clone(),
                            statute_b_id: statute_b.id.clone(),
                            contradiction_type: ContradictionType::IdenticalConditionsConflictingEffects,
                            description: format!(
                                "Statutes '{}' and '{}' have identical conditions but conflicting effects",
                                statute_a.id, statute_b.id
                            ),
                            severity: ErrorSeverity::Critical,
                        });
                }
            }
        }
        contradictions
    }
    /// Checks if two effects contradict each other.
    fn effects_contradict(effect_a: &Effect, effect_b: &Effect) -> bool {
        matches!(
            (&effect_a.effect_type, &effect_b.effect_type),
            (EffectType::Grant, EffectType::Revoke) | (EffectType::Revoke, EffectType::Grant)
        ) && effect_a.description == effect_b.description
    }
    /// Checks if two sets of conditions may overlap (both could be true).
    /// This is a simplified heuristic - full overlap detection requires SAT solving.
    #[allow(dead_code)]
    fn conditions_may_overlap(conds_a: &[Condition], conds_b: &[Condition]) -> bool {
        conds_a.is_empty() || conds_b.is_empty() || conds_a == conds_b
    }
}
/// Recurrence patterns for temporal effects.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum RecurrencePattern {
    /// Recurs daily.
    Daily,
    /// Recurs weekly (every N weeks).
    Weekly { interval: u32 },
    /// Recurs monthly (every N months, on same day).
    Monthly { interval: u32 },
    /// Recurs yearly (every N years, on same date).
    Yearly { interval: u32 },
    /// Recurs on specific days of week (0=Sunday, 6=Saturday).
    DaysOfWeek { days: Vec<u32> },
    /// Custom cron-like pattern (simplified).
    Custom { description: String },
}
impl RecurrencePattern {
    /// Checks if the pattern matches a given date.
    #[must_use]
    pub fn matches(&self, date: NaiveDate, start: NaiveDate) -> bool {
        match self {
            Self::Daily => true,
            Self::Weekly { interval } => {
                let days_diff = (date - start).num_days();
                days_diff >= 0 && days_diff % ((*interval as i64) * 7) == 0
            }
            Self::Monthly { interval } => {
                let months_diff = (date.year() - start.year()) * 12
                    + (date.month() as i32 - start.month() as i32);
                months_diff >= 0
                    && months_diff % (*interval as i32) == 0
                    && date.day() == start.day()
            }
            Self::Yearly { interval } => {
                let years_diff = date.year() - start.year();
                years_diff >= 0
                    && years_diff % (*interval as i32) == 0
                    && date.month() == start.month()
                    && date.day() == start.day()
            }
            Self::DaysOfWeek { days } => {
                let weekday = date.weekday().num_days_from_sunday();
                days.contains(&weekday)
            }
            Self::Custom { .. } => true,
        }
    }
    /// Finds the next occurrence after a given date.
    #[must_use]
    pub fn next_occurrence(
        &self,
        after: NaiveDate,
        start: NaiveDate,
        end: Option<NaiveDate>,
    ) -> Option<NaiveDate> {
        let mut candidate = after.succ_opt()?;
        for _ in 0..365 {
            if let Some(end_date) = end
                && candidate > end_date
            {
                return None;
            }
            if candidate >= start && self.matches(candidate, start) {
                return Some(candidate);
            }
            candidate = candidate.succ_opt()?;
        }
        None
    }
}
/// Enhanced validation error with diagnostic context.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DiagnosticValidationError {
    /// The base validation error
    pub error: ValidationError,
    /// Diagnostic context
    pub context: DiagnosticContext,
}
impl DiagnosticValidationError {
    /// Creates a new diagnostic validation error.
    #[must_use]
    pub fn new(error: ValidationError) -> Self {
        Self {
            error,
            context: DiagnosticContext::new(),
        }
    }
    /// Adds diagnostic context.
    #[must_use]
    pub fn with_context(mut self, context: DiagnosticContext) -> Self {
        self.context = context;
        self
    }
    /// Gets the error code.
    #[must_use]
    pub fn error_code(&self) -> &str {
        self.error.error_code()
    }
    /// Gets the error severity.
    #[must_use]
    pub fn severity(&self) -> ErrorSeverity {
        self.error.severity()
    }
    /// Gets a suggestion for fixing the error.
    #[must_use]
    pub fn suggestion(&self) -> Option<&str> {
        self.error.suggestion()
    }
}
/// Effect dependency graph for tracking and detecting cycles.
///
/// Tracks dependencies between effects to ensure proper ordering
/// and detect circular dependencies.
///
/// # Example
/// ```
/// # use legalis_core::{Effect, EffectDependencyGraph};
/// let mut graph = EffectDependencyGraph::new();
/// let e1 = Effect::grant("base access");
/// let e2 = Effect::grant("extended access");
/// let e3 = Effect::obligation("reporting");
///
/// graph.add_effect("e1".to_string(), e1);
/// graph.add_effect("e2".to_string(), e2);
/// graph.add_effect("e3".to_string(), e3);
///
/// graph.add_dependency("e2", "e1"); // e2 depends on e1
/// graph.add_dependency("e3", "e2"); // e3 depends on e2
///
/// assert!(!graph.has_cycle());
/// assert_eq!(graph.topological_sort().unwrap(), vec!["e1", "e2", "e3"]);
/// ```
#[derive(Debug, Clone)]
pub struct EffectDependencyGraph {
    /// Effects indexed by ID.
    pub(super) effects: std::collections::HashMap<String, Effect>,
    /// Dependencies: effect_id -> list of effect_ids it depends on.
    pub(super) dependencies: std::collections::HashMap<String, Vec<String>>,
}
impl EffectDependencyGraph {
    /// Creates a new empty dependency graph.
    #[must_use]
    pub fn new() -> Self {
        Self {
            effects: std::collections::HashMap::new(),
            dependencies: std::collections::HashMap::new(),
        }
    }
    /// Adds an effect to the graph.
    pub fn add_effect(&mut self, id: String, effect: Effect) {
        self.effects.insert(id.clone(), effect);
        self.dependencies.entry(id).or_default();
    }
    /// Adds a dependency: `from` depends on `to`.
    ///
    /// Returns an error if it would create a cycle.
    pub fn add_dependency(&mut self, from: &str, to: &str) -> Result<(), String> {
        if !self.effects.contains_key(from) {
            return Err(format!("Effect '{}' not found", from));
        }
        if !self.effects.contains_key(to) {
            return Err(format!("Effect '{}' not found", to));
        }
        self.dependencies
            .entry(from.to_string())
            .or_default()
            .push(to.to_string());
        if self.has_cycle() {
            if let Some(deps) = self.dependencies.get_mut(from) {
                deps.retain(|d| d != to);
            }
            return Err(format!(
                "Adding dependency {} -> {} would create a cycle",
                from, to
            ));
        }
        Ok(())
    }
    /// Checks if the graph contains a cycle.
    #[must_use]
    pub fn has_cycle(&self) -> bool {
        let mut visited = std::collections::HashSet::new();
        let mut rec_stack = std::collections::HashSet::new();
        for node in self.effects.keys() {
            if self.has_cycle_util(node, &mut visited, &mut rec_stack) {
                return true;
            }
        }
        false
    }
    /// Helper function for cycle detection (DFS).
    fn has_cycle_util(
        &self,
        node: &str,
        visited: &mut std::collections::HashSet<String>,
        rec_stack: &mut std::collections::HashSet<String>,
    ) -> bool {
        if rec_stack.contains(node) {
            return true;
        }
        if visited.contains(node) {
            return false;
        }
        visited.insert(node.to_string());
        rec_stack.insert(node.to_string());
        if let Some(deps) = self.dependencies.get(node) {
            for dep in deps {
                if self.has_cycle_util(dep, visited, rec_stack) {
                    return true;
                }
            }
        }
        rec_stack.remove(node);
        false
    }
    /// Returns a topological sort of the effects (dependency order).
    ///
    /// Returns None if there's a cycle.
    #[must_use]
    pub fn topological_sort(&self) -> Option<Vec<String>> {
        if self.has_cycle() {
            return None;
        }
        let mut visited = std::collections::HashSet::new();
        let mut stack = Vec::new();
        for node in self.effects.keys() {
            if !visited.contains(node) {
                self.topological_sort_util(node, &mut visited, &mut stack);
            }
        }
        Some(stack)
    }
    /// Helper for topological sort (DFS).
    fn topological_sort_util(
        &self,
        node: &str,
        visited: &mut std::collections::HashSet<String>,
        stack: &mut Vec<String>,
    ) {
        visited.insert(node.to_string());
        if let Some(deps) = self.dependencies.get(node) {
            for dep in deps {
                if !visited.contains(dep) {
                    self.topological_sort_util(dep, visited, stack);
                }
            }
        }
        stack.push(node.to_string());
    }
    /// Gets an effect by ID.
    #[must_use]
    pub fn get_effect(&self, id: &str) -> Option<&Effect> {
        self.effects.get(id)
    }
    /// Returns the number of effects in the graph.
    #[must_use]
    pub fn len(&self) -> usize {
        self.effects.len()
    }
    /// Checks if the graph is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.effects.is_empty()
    }
}
/// Comparison operators for conditions.
///
/// Used in conditions to compare numeric values (age, income, duration, etc.).
///
/// # Examples
///
/// ```
/// use legalis_core::ComparisonOp;
///
/// let op = ComparisonOp::GreaterOrEqual;
/// assert_eq!(format!("{}", op), ">=");
///
/// let eq = ComparisonOp::Equal;
/// assert_eq!(format!("{}", eq), "==");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum ComparisonOp {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterOrEqual,
    LessThan,
    LessOrEqual,
}
impl ComparisonOp {
    /// Returns the inverse of this comparison operator.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::ComparisonOp;
    ///
    /// assert_eq!(ComparisonOp::GreaterThan.inverse(), ComparisonOp::LessOrEqual);
    /// assert_eq!(ComparisonOp::Equal.inverse(), ComparisonOp::NotEqual);
    /// ```
    #[must_use]
    pub const fn inverse(&self) -> Self {
        match self {
            Self::Equal => Self::NotEqual,
            Self::NotEqual => Self::Equal,
            Self::GreaterThan => Self::LessOrEqual,
            Self::GreaterOrEqual => Self::LessThan,
            Self::LessThan => Self::GreaterOrEqual,
            Self::LessOrEqual => Self::GreaterThan,
        }
    }
    /// Returns true if this is an equality check (Equal or NotEqual).
    #[must_use]
    pub const fn is_equality(&self) -> bool {
        matches!(self, Self::Equal | Self::NotEqual)
    }
    /// Returns true if this is an ordering comparison.
    #[must_use]
    pub const fn is_ordering(&self) -> bool {
        !self.is_equality()
    }
    /// Compares two u32 values using this operator.
    #[must_use]
    pub const fn compare_u32(&self, left: u32, right: u32) -> bool {
        match self {
            Self::Equal => left == right,
            Self::NotEqual => left != right,
            Self::GreaterThan => left > right,
            Self::GreaterOrEqual => left >= right,
            Self::LessThan => left < right,
            Self::LessOrEqual => left <= right,
        }
    }
    /// Compares two u64 values using this operator.
    #[must_use]
    pub const fn compare_u64(&self, left: u64, right: u64) -> bool {
        match self {
            Self::Equal => left == right,
            Self::NotEqual => left != right,
            Self::GreaterThan => left > right,
            Self::GreaterOrEqual => left >= right,
            Self::LessThan => left < right,
            Self::LessOrEqual => left <= right,
        }
    }
    /// Compares two i64 values using this operator.
    #[must_use]
    pub const fn compare_i64(&self, left: i64, right: i64) -> bool {
        match self {
            Self::Equal => left == right,
            Self::NotEqual => left != right,
            Self::GreaterThan => left > right,
            Self::GreaterOrEqual => left >= right,
            Self::LessThan => left < right,
            Self::LessOrEqual => left <= right,
        }
    }
    /// Compares two f64 values using this operator.
    #[must_use]
    pub fn compare_f64(&self, left: f64, right: f64) -> bool {
        match self {
            Self::Equal => (left - right).abs() < f64::EPSILON,
            Self::NotEqual => (left - right).abs() >= f64::EPSILON,
            Self::GreaterThan => left > right,
            Self::GreaterOrEqual => left >= right,
            Self::LessThan => left < right,
            Self::LessOrEqual => left <= right,
        }
    }
}
/// Subsumption analyzer for determining if one statute subsumes another.
///
/// In legal reasoning, statute A subsumes statute B if:
/// - A and B have the same legal effect
/// - B's conditions are more specific than (or equal to) A's conditions
/// - Whenever B applies, A also applies (but not necessarily vice versa)
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, Condition, ComparisonOp, SubsumptionAnalyzer};
///
/// // General statute: anyone over 18 can vote
/// let general = Statute::new("vote-general", "Voting Rights", Effect::grant("vote"))
///     .with_precondition(Condition::age(ComparisonOp::GreaterOrEqual, 18));
///
/// // Specific statute: citizens over 18 can vote (more specific)
/// let specific = Statute::new("vote-citizen", "Citizen Voting", Effect::grant("vote"))
///     .with_precondition(
///         Condition::age(ComparisonOp::GreaterOrEqual, 18)
///             .and(Condition::has_attribute("citizenship"))
///     );
///
/// // General subsumes specific
/// assert!(SubsumptionAnalyzer::subsumes(&general, &specific));
/// assert!(!SubsumptionAnalyzer::subsumes(&specific, &general));
/// ```
pub struct SubsumptionAnalyzer;
impl SubsumptionAnalyzer {
    /// Checks if statute A subsumes statute B.
    ///
    /// Returns `true` if B is more specific than A (A subsumes B).
    #[must_use]
    pub fn subsumes(a: &Statute, b: &Statute) -> bool {
        if !Self::effects_compatible(&a.effect, &b.effect) {
            return false;
        }
        if a.preconditions.is_empty() {
            return true;
        }
        if b.preconditions.is_empty() {
            return false;
        }
        Self::conditions_subsume(&a.preconditions, &b.preconditions)
    }
    /// Checks if effects are compatible for subsumption.
    fn effects_compatible(a: &Effect, b: &Effect) -> bool {
        a.effect_type == b.effect_type && a.description == b.description
    }
    /// Checks if condition set A subsumes condition set B.
    ///
    /// Returns `true` if B is more specific (adds more constraints).
    fn conditions_subsume(a_conds: &[Condition], b_conds: &[Condition]) -> bool {
        for a_cond in a_conds {
            if !Self::condition_present_in(a_cond, b_conds) {
                return false;
            }
        }
        true
    }
    /// Checks if a single condition from A is present (or implied) in B's conditions.
    fn condition_present_in(a_cond: &Condition, b_conds: &[Condition]) -> bool {
        if b_conds
            .iter()
            .any(|b_cond| Self::conditions_equivalent(a_cond, b_cond))
        {
            return true;
        }
        if b_conds
            .iter()
            .any(|b_cond| Self::condition_subsumes_condition(a_cond, b_cond))
        {
            return true;
        }
        for b_cond in b_conds {
            if Self::condition_in_compound(a_cond, b_cond) {
                return true;
            }
        }
        false
    }
    /// Checks if condition A subsumes condition B (B is stricter than A).
    fn condition_subsumes_condition(a: &Condition, b: &Condition) -> bool {
        match (a, b) {
            (
                Condition::Age {
                    operator: op_a,
                    value: val_a,
                },
                Condition::Age {
                    operator: op_b,
                    value: val_b,
                },
            ) => match (op_a, op_b) {
                (ComparisonOp::GreaterOrEqual, ComparisonOp::GreaterOrEqual) => val_b >= val_a,
                (ComparisonOp::LessOrEqual, ComparisonOp::LessOrEqual) => val_b <= val_a,
                _ => false,
            },
            (
                Condition::Income {
                    operator: op_a,
                    value: val_a,
                },
                Condition::Income {
                    operator: op_b,
                    value: val_b,
                },
            ) => match (op_a, op_b) {
                (ComparisonOp::LessThan, ComparisonOp::LessThan) => val_b <= val_a,
                (ComparisonOp::GreaterThan, ComparisonOp::GreaterThan) => val_b >= val_a,
                _ => false,
            },
            (
                Condition::Percentage {
                    operator: op_a,
                    value: val_a,
                    context: ctx_a,
                },
                Condition::Percentage {
                    operator: op_b,
                    value: val_b,
                    context: ctx_b,
                },
            ) => {
                if ctx_a != ctx_b {
                    return false;
                }
                match (op_a, op_b) {
                    (ComparisonOp::GreaterOrEqual, ComparisonOp::GreaterOrEqual) => val_b >= val_a,
                    (ComparisonOp::LessOrEqual, ComparisonOp::LessOrEqual) => val_b <= val_a,
                    _ => false,
                }
            }
            (Condition::And(a_left, a_right), Condition::And(b_left, b_right)) => {
                Self::condition_subsumes_condition(a_left, b_left)
                    && Self::condition_subsumes_condition(a_right, b_right)
            }
            _ => false,
        }
    }
    /// Checks if two conditions are logically equivalent.
    fn conditions_equivalent(a: &Condition, b: &Condition) -> bool {
        match (a, b) {
            (
                Condition::Age {
                    operator: op_a,
                    value: val_a,
                },
                Condition::Age {
                    operator: op_b,
                    value: val_b,
                },
            ) => op_a == op_b && val_a == val_b,
            (
                Condition::Income {
                    operator: op_a,
                    value: val_a,
                },
                Condition::Income {
                    operator: op_b,
                    value: val_b,
                },
            ) => op_a == op_b && val_a == val_b,
            (Condition::HasAttribute { key: key_a }, Condition::HasAttribute { key: key_b }) => {
                key_a == key_b
            }
            (
                Condition::AttributeEquals {
                    key: key_a,
                    value: val_a,
                },
                Condition::AttributeEquals {
                    key: key_b,
                    value: val_b,
                },
            ) => key_a == key_b && val_a == val_b,
            (
                Condition::Geographic {
                    region_type: rt_a,
                    region_id: rid_a,
                },
                Condition::Geographic {
                    region_type: rt_b,
                    region_id: rid_b,
                },
            ) => rt_a == rt_b && rid_a == rid_b,
            _ => false,
        }
    }
    /// Checks if a condition appears within a compound condition.
    fn condition_in_compound(target: &Condition, compound: &Condition) -> bool {
        match compound {
            Condition::And(left, right) | Condition::Or(left, right) => {
                Self::conditions_equivalent(target, left)
                    || Self::conditions_equivalent(target, right)
                    || Self::condition_subsumes_condition(target, left)
                    || Self::condition_subsumes_condition(target, right)
                    || Self::condition_in_compound(target, left)
                    || Self::condition_in_compound(target, right)
            }
            Condition::Not(inner) => {
                Self::conditions_equivalent(target, inner)
                    || Self::condition_in_compound(target, inner)
            }
            _ => false,
        }
    }
    /// Finds all statutes that are subsumed by the given statute.
    ///
    /// Returns statutes that are more specific than the given statute.
    #[must_use]
    pub fn find_subsumed<'a>(statute: &Statute, candidates: &'a [Statute]) -> Vec<&'a Statute> {
        candidates
            .iter()
            .filter(|candidate| candidate.id != statute.id && Self::subsumes(statute, candidate))
            .collect()
    }
    /// Finds all statutes that subsume the given statute.
    ///
    /// Returns statutes that are more general than the given statute.
    #[must_use]
    pub fn find_subsuming<'a>(statute: &Statute, candidates: &'a [Statute]) -> Vec<&'a Statute> {
        candidates
            .iter()
            .filter(|candidate| candidate.id != statute.id && Self::subsumes(candidate, statute))
            .collect()
    }
}
/// Entity relationship types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum RelationshipType {
    /// Parent-child relationship
    ParentChild,
    /// Spousal relationship
    Spouse,
    /// Employment relationship
    Employment,
    /// Guardianship
    Guardian,
    /// Business ownership
    BusinessOwner,
    /// Contractual relationship
    Contractual,
}
/// Legal entailment engine that determines what conclusions follow from statutes and facts.
///
/// Given a set of statutes and an evaluation context, the entailment engine:
/// 1. Evaluates each statute's preconditions
/// 2. Applies statutes whose conditions are met
/// 3. Returns the resulting legal effects
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, Condition, ComparisonOp};
/// use legalis_core::{EntailmentEngine, AttributeBasedContext};
/// use std::collections::HashMap;
///
/// let voting_statute = Statute::new("vote", "Voting Rights", Effect::grant("vote"))
///     .with_precondition(Condition::age(ComparisonOp::GreaterOrEqual, 18));
///
/// let tax_credit = Statute::new("tax", "Tax Credit", Effect::grant("tax_credit"))
///     .with_precondition(Condition::income(ComparisonOp::LessThan, 50000));
///
/// let mut attributes = HashMap::new();
/// attributes.insert("age".to_string(), "25".to_string());
/// attributes.insert("income".to_string(), "45000".to_string());
/// let context = AttributeBasedContext::new(attributes);
///
/// let statutes = vec![voting_statute, tax_credit];
/// let engine = EntailmentEngine::new(statutes);
/// let results = engine.entail(&context);
///
/// // Both statutes apply
/// assert_eq!(results.len(), 2);
/// assert!(results.iter().all(|r| r.conditions_satisfied));
/// ```
#[derive(Debug, Clone)]
pub struct EntailmentEngine {
    pub(super) statutes: Vec<Statute>,
}
impl EntailmentEngine {
    /// Creates a new entailment engine with the given statutes.
    #[must_use]
    pub fn new(statutes: Vec<Statute>) -> Self {
        Self { statutes }
    }
    /// Determines what legal effects follow from the statutes given the context.
    ///
    /// Returns all applicable effects where preconditions are satisfied.
    pub fn entail(&self, context: &AttributeBasedContext) -> Vec<EntailmentResult> {
        self.statutes
            .iter()
            .map(|statute| self.apply_statute(statute, context))
            .collect()
    }
    /// Determines what legal effects follow, filtering to only satisfied statutes.
    ///
    /// Returns only the effects where all preconditions are met.
    pub fn entail_satisfied(&self, context: &AttributeBasedContext) -> Vec<EntailmentResult> {
        self.entail(context)
            .into_iter()
            .filter(|result| result.conditions_satisfied)
            .collect()
    }
    /// Applies a single statute and returns the result.
    fn apply_statute(
        &self,
        statute: &Statute,
        context: &AttributeBasedContext,
    ) -> EntailmentResult {
        let mut errors = Vec::new();
        let mut all_satisfied = true;
        if statute.preconditions.is_empty() {
            return EntailmentResult {
                statute_id: statute.id.clone(),
                effect: statute.effect.clone(),
                conditions_satisfied: true,
                errors: Vec::new(),
            };
        }
        for condition in &statute.preconditions {
            match condition.evaluate_simple(context) {
                Ok(true) => {}
                Ok(false) => {
                    all_satisfied = false;
                }
                Err(e) => {
                    all_satisfied = false;
                    errors.push(format!("{}", e));
                }
            }
        }
        EntailmentResult {
            statute_id: statute.id.clone(),
            effect: statute.effect.clone(),
            conditions_satisfied: all_satisfied,
            errors,
        }
    }
    /// Adds a statute to the engine.
    pub fn add_statute(&mut self, statute: Statute) {
        self.statutes.push(statute);
    }
    /// Removes a statute by ID.
    pub fn remove_statute(&mut self, statute_id: &str) -> bool {
        let original_len = self.statutes.len();
        self.statutes.retain(|s| s.id != statute_id);
        self.statutes.len() < original_len
    }
    /// Returns a reference to all statutes in the engine.
    #[must_use]
    pub fn statutes(&self) -> &[Statute] {
        &self.statutes
    }
    /// Returns the number of statutes in the engine.
    #[must_use]
    pub fn statute_count(&self) -> usize {
        self.statutes.len()
    }
    /// Checks if a statute exists in the engine by ID.
    #[must_use]
    pub fn has_statute(&self, statute_id: &str) -> bool {
        self.statutes.iter().any(|s| s.id == statute_id)
    }
}
/// Temporal validity for statutes.
///
/// Defines when a statute is in force, including effective dates, expiry dates (sunset clauses),
/// and amendment history.
///
/// # Examples
///
/// ```
/// use legalis_core::TemporalValidity;
/// use chrono::NaiveDate;
///
/// let validity = TemporalValidity::new()
///     .with_effective_date(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())
///     .with_expiry_date(NaiveDate::from_ymd_opt(2025, 12, 31).unwrap());
///
/// let today = NaiveDate::from_ymd_opt(2025, 6, 15).unwrap();
/// assert!(validity.is_active(today));
///
/// let before = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
/// assert!(!validity.is_active(before));
///
/// let after = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
/// assert!(!validity.is_active(after));
/// ```
#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct TemporalValidity {
    /// Effective date (when the statute comes into force)
    pub effective_date: Option<NaiveDate>,
    /// Expiry date (sunset clause)
    pub expiry_date: Option<NaiveDate>,
    /// Enactment timestamp
    pub enacted_at: Option<DateTime<Utc>>,
    /// Last amended timestamp
    pub amended_at: Option<DateTime<Utc>>,
}
impl TemporalValidity {
    /// Creates a new TemporalValidity with no dates set.
    pub fn new() -> Self {
        Self::default()
    }
    /// Sets the effective date.
    pub fn with_effective_date(mut self, date: NaiveDate) -> Self {
        self.effective_date = Some(date);
        self
    }
    /// Sets the expiry date.
    pub fn with_expiry_date(mut self, date: NaiveDate) -> Self {
        self.expiry_date = Some(date);
        self
    }
    /// Sets the enacted timestamp.
    pub fn with_enacted_at(mut self, timestamp: DateTime<Utc>) -> Self {
        self.enacted_at = Some(timestamp);
        self
    }
    /// Sets the amended timestamp.
    pub fn with_amended_at(mut self, timestamp: DateTime<Utc>) -> Self {
        self.amended_at = Some(timestamp);
        self
    }
    /// Checks if the statute is currently active.
    pub fn is_active(&self, as_of: NaiveDate) -> bool {
        let after_effective = self.effective_date.is_none_or(|d| as_of >= d);
        let before_expiry = self.expiry_date.is_none_or(|d| as_of <= d);
        after_effective && before_expiry
    }
    /// Returns whether this has an effective date set.
    #[must_use]
    pub fn has_effective_date(&self) -> bool {
        self.effective_date.is_some()
    }
    /// Returns whether this has an expiry date set.
    #[must_use]
    pub fn has_expiry_date(&self) -> bool {
        self.expiry_date.is_some()
    }
    /// Returns whether this has been enacted (has an enacted_at timestamp).
    #[must_use]
    pub fn is_enacted(&self) -> bool {
        self.enacted_at.is_some()
    }
    /// Returns whether this has been amended.
    #[must_use]
    pub fn is_amended(&self) -> bool {
        self.amended_at.is_some()
    }
    /// Returns whether the statute has expired as of the given date.
    #[must_use]
    pub fn has_expired(&self, as_of: NaiveDate) -> bool {
        self.expiry_date.is_some_and(|exp| as_of > exp)
    }
    /// Returns whether the statute is not yet effective as of the given date.
    #[must_use]
    pub fn is_pending(&self, as_of: NaiveDate) -> bool {
        self.effective_date.is_some_and(|eff| as_of < eff)
    }
}
/// Types of changes that can occur in a statute.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum StatuteChange {
    /// The statute ID was changed
    IdChanged { old: String, new: String },
    /// The statute title was changed
    TitleChanged { old: String, new: String },
    /// The effect was changed
    EffectChanged { old: String, new: String },
    /// Preconditions were modified
    PreconditionsChanged { added: usize, removed: usize },
    /// Temporal validity was changed
    TemporalValidityChanged,
    /// Version number was changed
    VersionChanged { old: u32, new: u32 },
    /// Jurisdiction was changed
    JurisdictionChanged {
        old: Option<String>,
        new: Option<String>,
    },
}
