//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::{EvaluationContext, Jurisdiction};
use super::types::{
    ConditionCache, ConditionOperation, Effect, ErrorSeverity, ExplanationStep, SourceLocation,
    ValidationError,
};
use super::types_4::{ComparisonOp, DiagnosticValidationError, RecurrencePattern};
use super::types_5::Condition;
use super::types_6::{Statute, StatuteQuery};
use chrono::{DateTime, NaiveDate, Utc};
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Statute registry for managing collections of statutes with query capabilities.
///
/// # Examples
///
/// ```
/// use legalis_core::{StatuteRegistry, Statute, Effect};
///
/// let mut registry = StatuteRegistry::new();
/// registry.add(Statute::new("law1", "Example Law", Effect::grant("right")));
/// registry.add(Statute::new("law2", "Another Law", Effect::revoke("privilege")));
///
/// assert_eq!(registry.len(), 2);
///
/// // Query the registry
/// let grants = registry.query().effect_type(legalis_core::EffectType::Grant).execute();
/// assert_eq!(grants.len(), 1);
/// ```
#[derive(Debug, Clone, Default)]
pub struct StatuteRegistry {
    pub(super) statutes: Vec<Statute>,
}
impl StatuteRegistry {
    /// Creates a new empty statute registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Creates a statute registry from a vector of statutes.
    #[must_use]
    pub fn from_statutes(statutes: Vec<Statute>) -> Self {
        Self { statutes }
    }
    /// Adds a statute to the registry.
    pub fn add(&mut self, statute: Statute) {
        self.statutes.push(statute);
    }
    /// Removes a statute by ID.
    ///
    /// Returns `true` if a statute was removed.
    pub fn remove(&mut self, id: &str) -> bool {
        let original_len = self.statutes.len();
        self.statutes.retain(|s| s.id != id);
        self.statutes.len() < original_len
    }
    /// Gets a statute by ID.
    #[must_use]
    pub fn get(&self, id: &str) -> Option<&Statute> {
        self.statutes.iter().find(|s| s.id == id)
    }
    /// Gets a mutable reference to a statute by ID.
    pub fn get_mut(&mut self, id: &str) -> Option<&mut Statute> {
        self.statutes.iter_mut().find(|s| s.id == id)
    }
    /// Returns the number of statutes in the registry.
    #[must_use]
    pub fn len(&self) -> usize {
        self.statutes.len()
    }
    /// Returns `true` if the registry is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.statutes.is_empty()
    }
    /// Returns an iterator over all statutes.
    pub fn iter(&self) -> impl Iterator<Item = &Statute> {
        self.statutes.iter()
    }
    /// Returns a mutable iterator over all statutes.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Statute> {
        self.statutes.iter_mut()
    }
    /// Creates a new query over the statutes in this registry.
    #[must_use]
    pub fn query(&self) -> StatuteQuery<'_> {
        StatuteQuery::new(&self.statutes)
    }
    /// Clears all statutes from the registry.
    pub fn clear(&mut self) {
        self.statutes.clear();
    }
    /// Returns all statutes as a vector.
    #[must_use]
    pub fn all(&self) -> &[Statute] {
        &self.statutes
    }
    /// Finds statutes that conflict with each other at a given date.
    #[must_use]
    pub fn find_conflicts(&self, date: NaiveDate) -> Vec<(&Statute, &Statute)> {
        let mut conflicts = Vec::new();
        let effective: Vec<_> = self
            .statutes
            .iter()
            .filter(|s| s.temporal_validity.is_active(date))
            .collect();
        for i in 0..effective.len() {
            for j in (i + 1)..effective.len() {
                let a = effective[i];
                let b = effective[j];
                if a.effect.effect_type == b.effect.effect_type
                    && a.effect.description != b.effect.description
                    && !a.preconditions.is_empty()
                    && !b.preconditions.is_empty()
                {
                    conflicts.push((a, b));
                }
            }
        }
        conflicts
    }
    /// Merges another registry into this one.
    pub fn merge(&mut self, other: StatuteRegistry) {
        self.statutes.extend(other.statutes);
    }
}
/// Builder for constructing `Effect` objects with a fluent API.
///
/// Provides a convenient way to construct effects with parameters.
///
/// # Examples
///
/// ```
/// use legalis_core::{EffectBuilder, EffectType};
///
/// let effect = EffectBuilder::new()
///     .effect_type(EffectType::Grant)
///     .description("Tax credit")
///     .parameter("amount", "1000")
///     .parameter("currency", "USD")
///     .build();
///
/// assert_eq!(effect.effect_type, EffectType::Grant);
/// assert_eq!(effect.parameters.get("amount"), Some(&"1000".to_string()));
/// ```
#[derive(Debug, Clone)]
pub struct EffectBuilder {
    pub(super) effect_type: Option<EffectType>,
    pub(super) description: Option<String>,
    pub(super) parameters: std::collections::HashMap<String, String>,
}
impl EffectBuilder {
    /// Creates a new effect builder.
    #[must_use]
    pub fn new() -> Self {
        Self {
            effect_type: None,
            description: None,
            parameters: std::collections::HashMap::new(),
        }
    }
    /// Creates a builder initialized with an effect type and description.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{EffectBuilder, EffectType};
    ///
    /// let effect = EffectBuilder::grant("Tax credit")
    ///     .parameter("amount", "1000")
    ///     .build();
    ///
    /// assert_eq!(effect.effect_type, EffectType::Grant);
    /// ```
    #[must_use]
    pub fn grant(description: impl Into<String>) -> Self {
        Self {
            effect_type: Some(EffectType::Grant),
            description: Some(description.into()),
            parameters: std::collections::HashMap::new(),
        }
    }
    /// Creates a builder for a revoke effect.
    #[must_use]
    pub fn revoke(description: impl Into<String>) -> Self {
        Self {
            effect_type: Some(EffectType::Revoke),
            description: Some(description.into()),
            parameters: std::collections::HashMap::new(),
        }
    }
    /// Creates a builder for an obligation effect.
    #[must_use]
    pub fn obligation(description: impl Into<String>) -> Self {
        Self {
            effect_type: Some(EffectType::Obligation),
            description: Some(description.into()),
            parameters: std::collections::HashMap::new(),
        }
    }
    /// Sets the effect type.
    #[must_use]
    pub fn effect_type(mut self, effect_type: EffectType) -> Self {
        self.effect_type = Some(effect_type);
        self
    }
    /// Sets the description.
    #[must_use]
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }
    /// Adds a parameter to the effect.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{EffectBuilder, EffectType};
    ///
    /// let effect = EffectBuilder::new()
    ///     .effect_type(EffectType::MonetaryTransfer)
    ///     .description("Tax payment")
    ///     .parameter("amount", "5000")
    ///     .parameter("currency", "USD")
    ///     .build();
    ///
    /// assert_eq!(effect.parameters.len(), 2);
    /// ```
    #[must_use]
    pub fn parameter(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.parameters.insert(key.into(), value.into());
        self
    }
    /// Builds the final effect.
    ///
    /// # Panics
    ///
    /// Panics if effect_type or description is not set.
    #[must_use]
    pub fn build(self) -> Effect {
        Effect {
            effect_type: self.effect_type.expect("Effect type must be set"),
            description: self.description.expect("Description must be set"),
            parameters: self.parameters,
        }
    }
    /// Builds the effect, returning an error if required fields are missing.
    pub fn try_build(self) -> Result<Effect, String> {
        let effect_type = self.effect_type.ok_or("Effect type not set")?;
        let description = self.description.ok_or("Description not set")?;
        Ok(Effect {
            effect_type,
            description,
            parameters: self.parameters,
        })
    }
}
/// Three-valued logic result with uncertainty propagation.
///
/// Represents the result of partial evaluation where some data may be unknown.
/// Each value includes a confidence score (0.0 to 1.0) representing certainty.
///
/// # Uncertainty Propagation
///
/// - **AND**: Confidence is minimum of operands; False propagates immediately
/// - **OR**: Confidence is minimum of operands; True propagates immediately
/// - **NOT**: Confidence is preserved; value is inverted
///
/// # Examples
///
/// ```
/// # use legalis_core::PartialBool;
/// let definite_true = PartialBool::true_with_confidence(1.0);
/// let uncertain = PartialBool::unknown(0.5, "missing data");
/// let definite_false = PartialBool::false_with_confidence(1.0);
///
/// assert!(matches!(definite_true, PartialBool::True { confidence, .. } if confidence == 1.0));
/// assert!(matches!(uncertain, PartialBool::Unknown { confidence, .. } if confidence == 0.5));
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum PartialBool {
    /// Definitely true with confidence score.
    True {
        /// Confidence in this result (0.0 to 1.0).
        confidence: f64,
        /// Optional reason or explanation.
        reason: String,
    },
    /// Definitely false with confidence score.
    False {
        /// Confidence in this result (0.0 to 1.0).
        confidence: f64,
        /// Optional reason or explanation.
        reason: String,
    },
    /// Unknown (insufficient data) with confidence score.
    Unknown {
        /// Confidence in knowing it's unknown (0.0 to 1.0).
        confidence: f64,
        /// Reason why value is unknown.
        reason: String,
    },
}
impl PartialBool {
    /// Creates a True value with the given confidence.
    #[must_use]
    pub fn true_with_confidence(confidence: f64) -> Self {
        Self::True {
            confidence,
            reason: String::new(),
        }
    }
    /// Creates a True value with confidence and reason.
    #[must_use]
    pub fn true_with_confidence_and_reason(confidence: f64, reason: &str) -> Self {
        Self::True {
            confidence,
            reason: reason.to_string(),
        }
    }
    /// Creates a False value with the given confidence.
    #[must_use]
    pub fn false_with_confidence(confidence: f64) -> Self {
        Self::False {
            confidence,
            reason: String::new(),
        }
    }
    /// Creates a False value with confidence and reason.
    #[must_use]
    pub fn false_with_confidence_and_reason(confidence: f64, reason: &str) -> Self {
        Self::False {
            confidence,
            reason: reason.to_string(),
        }
    }
    /// Creates an Unknown value with confidence and reason.
    #[must_use]
    pub fn unknown(confidence: f64, reason: &str) -> Self {
        Self::Unknown {
            confidence,
            reason: reason.to_string(),
        }
    }
    /// Returns the confidence score.
    #[must_use]
    pub fn confidence(&self) -> f64 {
        match self {
            Self::True { confidence, .. }
            | Self::False { confidence, .. }
            | Self::Unknown { confidence, .. } => *confidence,
        }
    }
    /// Returns the reason or explanation.
    #[must_use]
    pub fn reason(&self) -> &str {
        match self {
            Self::True { reason, .. }
            | Self::False { reason, .. }
            | Self::Unknown { reason, .. } => reason,
        }
    }
    /// Checks if the result is definitely true.
    #[must_use]
    pub fn is_true(&self) -> bool {
        matches!(self, Self::True { .. })
    }
    /// Checks if the result is definitely false.
    #[must_use]
    pub fn is_false(&self) -> bool {
        matches!(self, Self::False { .. })
    }
    /// Checks if the result is unknown.
    #[must_use]
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown { .. })
    }
}
/// Memoization cache for condition evaluation results.
///
/// Caches evaluation results to avoid re-evaluating the same conditions.
///
/// # Examples
///
/// ```
/// use legalis_core::{Condition, ComparisonOp, ConditionEvaluator};
/// use legalis_core::{EvaluationContext, RegionType, RelationshipType, DurationUnit};
/// use chrono::NaiveDate;
///
/// struct MyContext { age: u32 }
///
/// impl EvaluationContext for MyContext {
///     fn get_attribute(&self, _key: &str) -> Option<String> { None }
///     fn get_age(&self) -> Option<u32> { Some(self.age) }
///     fn get_income(&self) -> Option<u64> { None }
///     fn get_current_date(&self) -> Option<NaiveDate> { None }
///     fn check_geographic(&self, _region_type: RegionType, _region_id: &str) -> bool { false }
///     fn check_relationship(&self, _relationship_type: RelationshipType, _target_id: Option<&str>) -> bool { false }
///     fn get_residency_months(&self) -> Option<u32> { None }
///     fn get_duration(&self, _unit: DurationUnit) -> Option<u32> { None }
///     fn get_percentage(&self, _context: &str) -> Option<u32> { None }
///     fn evaluate_formula(&self, _formula: &str) -> Option<f64> { None }
/// }
///
/// let mut evaluator = ConditionEvaluator::new();
/// let ctx = MyContext { age: 25 };
/// let condition = Condition::age(ComparisonOp::GreaterOrEqual, 18);
///
/// // First evaluation - not cached
/// assert_eq!(evaluator.evaluate(&condition, &ctx).ok(), Some(true));
///
/// // Second evaluation - retrieved from cache
/// assert_eq!(evaluator.evaluate(&condition, &ctx).ok(), Some(true));
/// assert_eq!(evaluator.cache_hits(), 1);
/// ```
#[derive(Debug, Default)]
pub struct ConditionEvaluator {
    pub(super) cache: std::collections::HashMap<String, bool>,
    pub(super) cache_hits: usize,
    pub(super) cache_misses: usize,
}
impl ConditionEvaluator {
    /// Creates a new condition evaluator with an empty cache.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Evaluates a condition with memoization.
    ///
    /// Results are cached based on the condition's string representation.
    pub fn evaluate<C: EvaluationContext>(
        &mut self,
        condition: &Condition,
        context: &C,
    ) -> Result<bool, EvaluationError> {
        let cache_key = format!("{}", condition);
        if let Some(&result) = self.cache.get(&cache_key) {
            self.cache_hits += 1;
            return Ok(result);
        }
        self.cache_misses += 1;
        let result = condition.evaluate(context)?;
        self.cache.insert(cache_key, result);
        Ok(result)
    }
    /// Clears the evaluation cache.
    pub fn clear_cache(&mut self) {
        self.cache.clear();
        self.cache_hits = 0;
        self.cache_misses = 0;
    }
    /// Returns the number of cache hits.
    #[must_use]
    pub const fn cache_hits(&self) -> usize {
        self.cache_hits
    }
    /// Returns the number of cache misses.
    #[must_use]
    pub const fn cache_misses(&self) -> usize {
        self.cache_misses
    }
    /// Returns the cache hit ratio (0.0 to 1.0).
    #[must_use]
    pub fn hit_ratio(&self) -> f64 {
        let total = self.cache_hits + self.cache_misses;
        if total == 0 {
            0.0
        } else {
            self.cache_hits as f64 / total as f64
        }
    }
}
/// Parallel evaluation support for ConditionEvaluator.
#[cfg(feature = "parallel")]
impl ConditionEvaluator {
    /// Evaluates a condition with memoization and parallel processing.
    ///
    /// Note: The cache is not thread-safe, so this method requires mutable access.
    /// For truly concurrent evaluation, use separate evaluators per thread.
    pub fn evaluate_parallel<C: EvaluationContext + Sync>(
        &mut self,
        condition: &Condition,
        context: &C,
    ) -> Result<bool, EvaluationError> {
        let cache_key = format!("{}", condition);
        if let Some(&result) = self.cache.get(&cache_key) {
            self.cache_hits += 1;
            return Ok(result);
        }
        self.cache_misses += 1;
        let result = condition.evaluate_parallel(context)?;
        self.cache.insert(cache_key, result);
        Ok(result)
    }
}
/// A single evaluation record in the audit trail.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct EvaluationRecord {
    /// When the evaluation occurred
    pub timestamp: DateTime<Utc>,
    /// The condition that was evaluated (as string)
    pub condition: String,
    /// The result of the evaluation
    pub result: bool,
    /// How long the evaluation took (microseconds)
    pub duration_micros: u64,
}
/// Collection of jurisdiction-specific statutes.
///
/// This type ensures all statutes in the collection belong to the same jurisdiction.
///
/// # Examples
///
/// ```
/// use legalis_core::{JurisdictionStatuteRegistry, US, JurisdictionStatute, Statute, Effect, EffectType};
///
/// let mut registry = JurisdictionStatuteRegistry::<US>::new();
///
/// let statute1 = Statute::new("law-1", "Law 1", Effect::new(EffectType::Grant, "Benefit 1"));
/// let statute2 = Statute::new("law-2", "Law 2", Effect::new(EffectType::Grant, "Benefit 2"));
///
/// registry.add(JurisdictionStatute::new(statute1));
/// registry.add(JurisdictionStatute::new(statute2));
///
/// assert_eq!(registry.len(), 2);
/// assert_eq!(registry.jurisdiction_code(), "US");
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct JurisdictionStatuteRegistry<J: Jurisdiction> {
    pub(super) statutes: Vec<JurisdictionStatute<J>>,
    _phantom: std::marker::PhantomData<J>,
}
impl<J: Jurisdiction> JurisdictionStatuteRegistry<J> {
    /// Creates a new empty registry for a specific jurisdiction.
    #[must_use]
    pub fn new() -> Self {
        Self {
            statutes: Vec::new(),
            _phantom: std::marker::PhantomData,
        }
    }
    /// Adds a statute to the registry.
    pub fn add(&mut self, statute: JurisdictionStatute<J>) {
        self.statutes.push(statute);
    }
    /// Returns the number of statutes in the registry.
    #[must_use]
    pub fn len(&self) -> usize {
        self.statutes.len()
    }
    /// Returns true if the registry is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.statutes.is_empty()
    }
    /// Returns the jurisdiction code for this registry.
    #[must_use]
    pub fn jurisdiction_code(&self) -> &'static str {
        J::code()
    }
    /// Returns an iterator over the statutes.
    pub fn iter(&self) -> impl Iterator<Item = &JurisdictionStatute<J>> {
        self.statutes.iter()
    }
    /// Finds a statute by ID.
    #[must_use]
    pub fn find(&self, id: &str) -> Option<&JurisdictionStatute<J>> {
        self.statutes.iter().find(|s| s.statute().id == id)
    }
}
/// Context wrapper that provides default values for missing attributes.
///
/// This is useful for handling optional attributes with sensible defaults.
///
/// # Example
/// ```
/// # use legalis_core::{Condition, ComparisonOp, AttributeBasedContext, DefaultValueContext};
/// # use std::collections::HashMap;
/// let mut attributes = HashMap::new();
/// attributes.insert("name".to_string(), "Alice".to_string());
/// // age is missing
/// let entity = AttributeBasedContext::new(attributes);
///
/// let mut defaults = HashMap::new();
/// defaults.insert("age".to_string(), "18".to_string());
///
/// let ctx_with_defaults = DefaultValueContext::new(&entity, defaults);
///
/// // Will use default age of 18
/// let condition = Condition::age(ComparisonOp::GreaterOrEqual, 18);
/// assert!(condition.evaluate(&ctx_with_defaults).unwrap());
/// ```
#[derive(Debug)]
pub struct DefaultValueContext<'a, C: EvaluationContext> {
    pub(super) inner: &'a C,
    pub(super) defaults: HashMap<String, String>,
}
impl<'a, C: EvaluationContext> DefaultValueContext<'a, C> {
    /// Creates a new context with default values.
    pub fn new(inner: &'a C, defaults: HashMap<String, String>) -> Self {
        Self { inner, defaults }
    }
    /// Adds a default value for an attribute.
    pub fn with_default(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.defaults.insert(key.into(), value.into());
        self
    }
}
/// A simple implementation of LegalEntity for testing and basic use cases.
///
/// This struct provides a straightforward key-value string storage for entity attributes.
/// For type-safe attribute handling, consider using [`crate::TypedEntity`] instead.
///
/// # Examples
///
/// ```
/// use legalis_core::{BasicEntity, LegalEntity};
///
/// let mut person = BasicEntity::new();
/// person.set_attribute("name", "Alice".to_string());
/// person.set_attribute("age", "30".to_string());
///
/// assert_eq!(person.get_attribute("name"), Some("Alice".to_string()));
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct BasicEntity {
    pub(super) id: Uuid,
    pub(super) attributes: std::collections::HashMap<String, String>,
}
impl BasicEntity {
    /// Creates a new BasicEntity with a random UUID.
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            attributes: std::collections::HashMap::new(),
        }
    }
    /// Creates a new BasicEntity with a specific UUID.
    pub fn with_id(id: Uuid) -> Self {
        Self {
            id,
            attributes: std::collections::HashMap::new(),
        }
    }
}
/// Audit trail for tracking condition evaluations.
///
/// Records each evaluation with timestamp, condition, result, and duration.
/// Useful for debugging, compliance, and performance analysis.
#[derive(Debug, Clone)]
pub struct EvaluationAuditTrail {
    /// List of evaluation records
    pub(super) records: Vec<EvaluationRecord>,
    /// Maximum number of records to keep
    max_records: usize,
}
impl EvaluationAuditTrail {
    /// Creates a new audit trail with default capacity (1000 records).
    #[must_use]
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
            max_records: 1000,
        }
    }
    /// Creates a new audit trail with custom capacity.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            records: Vec::with_capacity(capacity),
            max_records: capacity,
        }
    }
    /// Records an evaluation.
    pub fn record(&mut self, condition: String, result: bool, duration_micros: u64) {
        if self.records.len() >= self.max_records {
            self.records.remove(0);
        }
        self.records.push(EvaluationRecord {
            timestamp: Utc::now(),
            condition,
            result,
            duration_micros,
        });
    }
    /// Returns all evaluation records.
    #[must_use]
    pub fn records(&self) -> &[EvaluationRecord] {
        &self.records
    }
    /// Returns the number of records.
    #[must_use]
    pub fn len(&self) -> usize {
        self.records.len()
    }
    /// Returns true if there are no records.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
    /// Clears all records.
    pub fn clear(&mut self) {
        self.records.clear();
    }
    /// Returns average evaluation duration in microseconds.
    #[must_use]
    pub fn average_duration(&self) -> f64 {
        if self.records.is_empty() {
            return 0.0;
        }
        let total: u64 = self.records.iter().map(|r| r.duration_micros).sum();
        total as f64 / self.records.len() as f64
    }
    /// Returns the slowest evaluation record.
    #[must_use]
    pub fn slowest_evaluation(&self) -> Option<&EvaluationRecord> {
        self.records.iter().max_by_key(|r| r.duration_micros)
    }
    /// Returns records where evaluation took longer than threshold (microseconds).
    #[must_use]
    pub fn slow_evaluations(&self, threshold_micros: u64) -> Vec<&EvaluationRecord> {
        self.records
            .iter()
            .filter(|r| r.duration_micros > threshold_micros)
            .collect()
    }
}
/// Jurisdiction-specific statute wrapper using phantom types.
///
/// This type enforces at compile time that statutes are used in the correct
/// jurisdiction context. The type parameter `J` ensures that you can't mix
/// statutes from different jurisdictions without explicit conversion.
///
/// # Type Parameters
///
/// - `J`: Jurisdiction marker type implementing the `Jurisdiction` trait
///
/// # Examples
///
/// ```
/// use legalis_core::{JurisdictionStatute, US, UK, Statute, Effect, EffectType};
///
/// // Create a US-specific statute
/// let us_statute = Statute::new("tax-law", "Tax Law", Effect::new(EffectType::Grant, "Tax credit"));
/// let us_law = JurisdictionStatute::<US>::new(us_statute);
///
/// // Create a UK-specific statute
/// let uk_statute = Statute::new("uk-law", "UK Law", Effect::new(EffectType::Grant, "Benefit"));
/// let uk_law = JurisdictionStatute::<UK>::new(uk_statute);
///
/// // These types are different and can't be mixed
/// assert_eq!(us_law.jurisdiction_code(), "US");
/// assert_eq!(uk_law.jurisdiction_code(), "UK");
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct JurisdictionStatute<J: Jurisdiction> {
    statute: Statute,
    _phantom: std::marker::PhantomData<J>,
}
impl<J: Jurisdiction> JurisdictionStatute<J> {
    /// Creates a new jurisdiction-specific statute.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{JurisdictionStatute, US, Statute, Effect, EffectType};
    ///
    /// let statute = Statute::new("law-1", "Law", Effect::new(EffectType::Grant, "Benefit"));
    /// let us_law = JurisdictionStatute::<US>::new(statute);
    /// ```
    #[must_use]
    pub fn new(mut statute: Statute) -> Self {
        if statute.jurisdiction.is_none() {
            statute.jurisdiction = Some(J::code().to_string());
        }
        Self {
            statute,
            _phantom: std::marker::PhantomData,
        }
    }
    /// Returns the jurisdiction code for this statute.
    #[must_use]
    pub fn jurisdiction_code(&self) -> &'static str {
        J::code()
    }
    /// Returns a reference to the underlying statute.
    #[must_use]
    pub fn statute(&self) -> &Statute {
        &self.statute
    }
    /// Consumes self and returns the underlying statute.
    #[must_use]
    pub fn into_statute(self) -> Statute {
        self.statute
    }
    /// Converts this statute to a different jurisdiction.
    ///
    /// This is an explicit operation that requires the caller to acknowledge
    /// they are changing jurisdictions.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{JurisdictionStatute, US, UK, Statute, Effect, EffectType};
    ///
    /// let statute = Statute::new("law", "Law", Effect::new(EffectType::Grant, "Benefit"));
    /// let us_law = JurisdictionStatute::<US>::new(statute);
    /// let uk_law = us_law.convert_to::<UK>();
    /// assert_eq!(uk_law.jurisdiction_code(), "UK");
    /// ```
    #[must_use]
    pub fn convert_to<K: Jurisdiction>(mut self) -> JurisdictionStatute<K> {
        self.statute.jurisdiction = Some(K::code().to_string());
        JurisdictionStatute {
            statute: self.statute,
            _phantom: std::marker::PhantomData,
        }
    }
}
/// Diagnostic context for detailed error reporting.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct DiagnosticContext {
    /// Source location where the error occurred
    pub location: Option<SourceLocation>,
    /// Related statute ID
    pub statute_id: Option<String>,
    /// Related condition description
    pub condition: Option<String>,
    /// Stack trace or call chain
    pub stack: Vec<String>,
    /// Additional contextual notes
    pub notes: Vec<String>,
    /// Suggested fixes
    pub suggestions: Vec<String>,
}
impl DiagnosticContext {
    /// Creates a new empty diagnostic context.
    #[must_use]
    pub fn new() -> Self {
        Self {
            location: None,
            statute_id: None,
            condition: None,
            stack: Vec::new(),
            notes: Vec::new(),
            suggestions: Vec::new(),
        }
    }
    /// Sets the source location.
    #[must_use]
    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.location = Some(location);
        self
    }
    /// Sets the statute ID.
    #[must_use]
    pub fn with_statute_id(mut self, id: impl Into<String>) -> Self {
        self.statute_id = Some(id.into());
        self
    }
    /// Sets the condition description.
    #[must_use]
    pub fn with_condition(mut self, condition: impl Into<String>) -> Self {
        self.condition = Some(condition.into());
        self
    }
    /// Adds a stack frame.
    pub fn add_stack_frame(&mut self, frame: impl Into<String>) {
        self.stack.push(frame.into());
    }
    /// Adds a note.
    pub fn add_note(&mut self, note: impl Into<String>) {
        self.notes.push(note.into());
    }
    /// Adds a suggestion.
    pub fn add_suggestion(&mut self, suggestion: impl Into<String>) {
        self.suggestions.push(suggestion.into());
    }
    /// Builder method to add a stack frame.
    #[must_use]
    pub fn with_stack_frame(mut self, frame: impl Into<String>) -> Self {
        self.stack.push(frame.into());
        self
    }
    /// Builder method to add a note.
    #[must_use]
    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }
    /// Builder method to add a suggestion.
    #[must_use]
    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestions.push(suggestion.into());
        self
    }
}
/// Geographic region types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum RegionType {
    /// Country level
    Country,
    /// State/Province level
    State,
    /// City/Municipality level
    City,
    /// District/Ward level
    District,
    /// Postal/ZIP code area
    PostalCode,
    /// Custom region
    Custom,
}
/// United States jurisdiction marker.
#[derive(Debug, Clone, Copy)]
pub struct US;
/// Time unit for duration conditions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum DurationUnit {
    /// Days
    Days,
    /// Weeks
    Weeks,
    /// Months
    Months,
    /// Years
    Years,
}
/// Errors that can occur during condition evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum EvaluationError {
    /// Missing required attribute for evaluation
    MissingAttribute { key: String },
    /// Missing required context data
    MissingContext { description: String },
    /// Invalid formula or calculation
    InvalidFormula { formula: String, reason: String },
    /// Pattern matching error
    PatternError { pattern: String, reason: String },
    /// Maximum evaluation depth exceeded (prevents infinite recursion)
    MaxDepthExceeded { max_depth: usize },
    /// Custom error
    Custom { message: String },
}
/// Reason why one statute prevails over another.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum ConflictReason {
    /// Later law prevails (lex posterior derogat legi priori)
    TemporalPrecedence,
    /// More specific law prevails (lex specialis derogat legi generali)
    Specificity,
    /// Higher authority prevails (lex superior derogat legi inferiori)
    Hierarchy,
    /// Explicit amendment/repeal relationship
    ExplicitAmendment,
}
/// Diagnostic error reporter for collecting and formatting errors.
///
/// # Examples
///
/// ```
/// use legalis_core::{DiagnosticReporter, ValidationError, SourceLocation, DiagnosticContext};
///
/// let mut reporter = DiagnosticReporter::new();
///
/// reporter.add_error(
///     ValidationError::EmptyTitle,
///     DiagnosticContext::new()
///         .with_statute_id("law-123")
///         .with_location(SourceLocation::new().with_file("statutes.json").with_line(45))
///         .with_suggestion("Add a 'title' field to the statute definition")
/// );
///
/// reporter.add_error(
///     ValidationError::InvalidVersion,
///     DiagnosticContext::new()
///         .with_statute_id("law-456")
///         .with_note("Version must be greater than 0")
/// );
///
/// // Print all errors with diagnostic context
/// println!("{}", reporter.report());
/// assert_eq!(reporter.error_count(), 2);
/// ```
#[derive(Debug, Default)]
pub struct DiagnosticReporter {
    pub(super) errors: Vec<DiagnosticValidationError>,
}
impl DiagnosticReporter {
    /// Creates a new diagnostic reporter.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    /// Adds an error with diagnostic context.
    pub fn add_error(&mut self, error: ValidationError, context: DiagnosticContext) {
        self.errors
            .push(DiagnosticValidationError { error, context });
    }
    /// Adds an error without context.
    pub fn add_simple_error(&mut self, error: ValidationError) {
        self.errors.push(DiagnosticValidationError::new(error));
    }
    /// Returns the number of errors.
    #[must_use]
    pub fn error_count(&self) -> usize {
        self.errors.len()
    }
    /// Returns `true` if there are no errors.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }
    /// Returns `true` if there are errors.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    /// Gets all errors.
    #[must_use]
    pub fn errors(&self) -> &[DiagnosticValidationError] {
        &self.errors
    }
    /// Filters errors by severity.
    #[must_use]
    pub fn errors_with_severity(&self, severity: ErrorSeverity) -> Vec<&DiagnosticValidationError> {
        self.errors
            .iter()
            .filter(|e| e.severity() == severity)
            .collect()
    }
    /// Returns only critical errors.
    #[must_use]
    pub fn critical_errors(&self) -> Vec<&DiagnosticValidationError> {
        self.errors_with_severity(ErrorSeverity::Critical)
    }
    /// Clears all errors.
    pub fn clear(&mut self) {
        self.errors.clear();
    }
    /// Generates a formatted error report.
    #[must_use]
    pub fn report(&self) -> String {
        if self.errors.is_empty() {
            return "No errors".to_string();
        }
        let mut output = String::new();
        output.push_str(&format!("\n{} error(s) found:\n\n", self.errors.len()));
        for (i, error) in self.errors.iter().enumerate() {
            output.push_str(&format!("{}. {}\n", i + 1, error));
        }
        output
    }
    /// Generates a summary of errors by type.
    #[must_use]
    pub fn summary(&self) -> String {
        let critical = self.critical_errors().len();
        let errors = self.errors_with_severity(ErrorSeverity::Error).len();
        let warnings = self.errors_with_severity(ErrorSeverity::Warning).len();
        format!(
            "{} total ({} critical, {} errors, {} warnings)",
            self.error_count(),
            critical,
            errors,
            warnings
        )
    }
}
/// Context for evaluating conditions (simple attribute-based implementation).
///
/// Contains entity attributes and evaluation settings.
/// For a more flexible trait-based approach, see the `EvaluationContext` trait.
#[derive(Debug, Clone)]
pub struct AttributeBasedContext {
    /// Entity attributes as key-value pairs.
    pub attributes: HashMap<String, String>,
    /// Maximum evaluation depth to prevent stack overflow.
    pub max_depth: usize,
    /// Optional cache for memoizing condition evaluation results.
    pub cache: Option<ConditionCache>,
    /// Optional audit trail for tracking evaluation history.
    pub audit_trail: Option<EvaluationAuditTrail>,
}
impl AttributeBasedContext {
    /// Creates a new evaluation context with default max depth (100).
    #[must_use]
    pub fn new(attributes: HashMap<String, String>) -> Self {
        Self {
            attributes,
            max_depth: 100,
            cache: None,
            audit_trail: None,
        }
    }
    /// Creates a new evaluation context with custom max depth.
    #[must_use]
    pub fn with_max_depth(attributes: HashMap<String, String>, max_depth: usize) -> Self {
        Self {
            attributes,
            max_depth,
            cache: None,
            audit_trail: None,
        }
    }
    /// Creates a new evaluation context with caching enabled.
    #[must_use]
    pub fn with_cache(attributes: HashMap<String, String>) -> Self {
        Self {
            attributes,
            max_depth: 100,
            cache: Some(ConditionCache::new()),
            audit_trail: None,
        }
    }
    /// Creates a new evaluation context with custom max depth and cache capacity.
    #[must_use]
    pub fn with_cache_capacity(
        attributes: HashMap<String, String>,
        max_depth: usize,
        cache_capacity: usize,
    ) -> Self {
        Self {
            attributes,
            max_depth,
            cache: Some(ConditionCache::with_capacity(cache_capacity)),
            audit_trail: None,
        }
    }
    /// Creates a new evaluation context with audit trail enabled.
    #[must_use]
    pub fn with_audit_trail(attributes: HashMap<String, String>) -> Self {
        Self {
            attributes,
            max_depth: 100,
            cache: None,
            audit_trail: Some(EvaluationAuditTrail::new()),
        }
    }
    /// Records an evaluation in the audit trail if enabled.
    pub fn record_evaluation(&mut self, condition: &str, result: bool, duration_micros: u64) {
        if let Some(trail) = &mut self.audit_trail {
            trail.record(condition.to_string(), result, duration_micros);
        }
    }
}
/// A single step in the reasoning chain.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ReasoningStep {
    /// Step number
    pub step: usize,
    /// Description of this reasoning step
    pub description: String,
    /// Statute ID involved in this step
    pub statute_id: Option<String>,
    /// Condition evaluated in this step
    pub condition: Option<String>,
    /// Result of this step
    pub result: StepResult,
}
/// Types of legal effects.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum EffectType {
    /// Grant a right or permission
    Grant,
    /// Revoke a right or permission
    Revoke,
    /// Impose an obligation
    Obligation,
    /// Impose a prohibition
    Prohibition,
    /// Monetary transfer (subsidy, tax, fine, etc.)
    MonetaryTransfer,
    /// Status change
    StatusChange,
    /// Custom effect
    Custom,
}
/// Types of contradictions that can occur between statutes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum ContradictionType {
    /// Statutes have conflicting effects (grant vs revoke)
    ConflictingEffects,
    /// Identical conditions but conflicting effects
    IdenticalConditionsConflictingEffects,
    /// Circular dependency between statutes
    CircularDependency,
    /// Logical inconsistency in rule set
    LogicalInconsistency,
}
/// Builder for constructing `Condition` objects with a fluent API.
///
/// Provides a convenient way to construct complex conditions with chaining.
///
/// # Examples
///
/// ```
/// use legalis_core::{ConditionBuilder, ComparisonOp};
///
/// let condition = ConditionBuilder::new()
///     .age(ComparisonOp::GreaterOrEqual, 18)
///     .and()
///     .income(ComparisonOp::LessThan, 50000)
///     .build();
///
/// assert!(!condition.to_string().is_empty());
/// ```
#[derive(Debug, Clone)]
pub struct ConditionBuilder {
    pub(super) conditions: Vec<Condition>,
    pub(super) operation: ConditionOperation,
}
impl ConditionBuilder {
    /// Creates a new condition builder.
    #[must_use]
    pub fn new() -> Self {
        Self {
            conditions: Vec::new(),
            operation: ConditionOperation::None,
        }
    }
    /// Adds an age condition.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{ConditionBuilder, ComparisonOp};
    ///
    /// let cond = ConditionBuilder::new()
    ///     .age(ComparisonOp::GreaterOrEqual, 21)
    ///     .build();
    /// ```
    #[must_use]
    pub fn age(mut self, operator: ComparisonOp, value: u32) -> Self {
        self.conditions.push(Condition::Age { operator, value });
        self
    }
    /// Adds an income condition.
    #[must_use]
    pub fn income(mut self, operator: ComparisonOp, value: u64) -> Self {
        self.conditions.push(Condition::Income { operator, value });
        self
    }
    /// Adds a has-attribute condition.
    #[must_use]
    pub fn has_attribute(mut self, attr: impl Into<String>) -> Self {
        self.conditions
            .push(Condition::HasAttribute { key: attr.into() });
        self
    }
    /// Adds an attribute-equals condition.
    #[must_use]
    pub fn attribute_equals(mut self, attr: impl Into<String>, value: impl Into<String>) -> Self {
        self.conditions.push(Condition::AttributeEquals {
            key: attr.into(),
            value: value.into(),
        });
        self
    }
    /// Adds a custom condition.
    #[must_use]
    pub fn custom(mut self, description: impl Into<String>) -> Self {
        self.conditions.push(Condition::Custom {
            description: description.into(),
        });
        self
    }
    /// Combines the next condition with AND logic.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{ConditionBuilder, ComparisonOp};
    ///
    /// let cond = ConditionBuilder::new()
    ///     .age(ComparisonOp::GreaterOrEqual, 18)
    ///     .and()
    ///     .income(ComparisonOp::LessThan, 50000)
    ///     .build();
    /// ```
    #[must_use]
    pub fn and(mut self) -> Self {
        self.operation = ConditionOperation::And;
        self
    }
    /// Combines the next condition with OR logic.
    #[must_use]
    pub fn or(mut self) -> Self {
        self.operation = ConditionOperation::Or;
        self
    }
    /// Builds the final condition.
    ///
    /// If multiple conditions were added, they are combined according to the
    /// specified operations (AND/OR).
    #[must_use]
    pub fn build(self) -> Condition {
        if self.conditions.is_empty() {
            Condition::Custom {
                description: "true".to_string(),
            }
        } else if self.conditions.len() == 1 {
            self.conditions
                .into_iter()
                .next()
                .expect("invariant: conditions.len() == 1 checked above")
        } else {
            let mut result = self.conditions[0].clone();
            for cond in self.conditions.into_iter().skip(1) {
                result = match self.operation {
                    ConditionOperation::And => Condition::And(Box::new(result), Box::new(cond)),
                    ConditionOperation::Or => Condition::Or(Box::new(result), Box::new(cond)),
                    ConditionOperation::None => Condition::And(Box::new(result), Box::new(cond)),
                };
            }
            result
        }
    }
}
/// Condition evaluation errors.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum ConditionError {
    /// Missing attribute in entity.
    MissingAttribute { key: String },
    /// Type mismatch when evaluating condition.
    TypeMismatch { expected: String, actual: String },
    /// Invalid calculation formula.
    InvalidFormula { formula: String, error: String },
    /// Pattern matching error.
    PatternError { pattern: String, error: String },
    /// Evaluation exceeded maximum depth (possible infinite recursion).
    MaxDepthExceeded { max_depth: usize },
    /// Custom evaluation error.
    Custom { message: String },
}
impl ConditionError {
    /// Returns the error code for this condition error.
    #[must_use]
    pub const fn error_code(&self) -> &'static str {
        match self {
            Self::MissingAttribute { .. } => "C001",
            Self::TypeMismatch { .. } => "C002",
            Self::InvalidFormula { .. } => "C003",
            Self::PatternError { .. } => "C004",
            Self::MaxDepthExceeded { .. } => "C005",
            Self::Custom { .. } => "C999",
        }
    }
    /// Returns the severity level of this error.
    #[must_use]
    pub const fn severity(&self) -> ErrorSeverity {
        match self {
            Self::MissingAttribute { .. } | Self::TypeMismatch { .. } => ErrorSeverity::Error,
            Self::InvalidFormula { .. } | Self::PatternError { .. } => ErrorSeverity::Critical,
            Self::MaxDepthExceeded { .. } => ErrorSeverity::Critical,
            Self::Custom { .. } => ErrorSeverity::Error,
        }
    }
    /// Returns a suggestion for how to fix this error.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::ConditionError;
    ///
    /// let err = ConditionError::MissingAttribute {
    ///     key: "age".to_string(),
    /// };
    /// assert!(err.suggestion().is_some());
    /// ```
    #[must_use]
    pub fn suggestion(&self) -> Option<String> {
        match self {
            Self::MissingAttribute { key } => Some(format!(
                "Add the '{}' attribute to the entity before evaluation",
                key
            )),
            Self::TypeMismatch { expected, actual } => Some(format!(
                "Convert the value from {} to {} or adjust the condition type",
                actual, expected
            )),
            Self::InvalidFormula { formula, error } => Some(format!(
                "Fix the formula '{}': {}. Check syntax and ensure all variables are defined.",
                formula, error
            )),
            Self::PatternError { pattern, error } => Some(format!(
                "Fix the regex pattern '{}': {}. Ensure the pattern is valid regex syntax.",
                pattern, error
            )),
            Self::MaxDepthExceeded { max_depth } => Some(format!(
                "Simplify the condition structure to reduce nesting below {} levels, or check for circular references",
                max_depth
            )),
            Self::Custom { .. } => None,
        }
    }
    /// Returns multiple recovery options for this error.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::ConditionError;
    ///
    /// let err = ConditionError::TypeMismatch {
    ///     expected: "u32".to_string(),
    ///     actual: "String".to_string(),
    /// };
    /// let options = err.recovery_options();
    /// assert!(!options.is_empty());
    /// ```
    #[must_use]
    pub fn recovery_options(&self) -> Vec<String> {
        match self {
            Self::MissingAttribute { key } => {
                vec![
                    format!("Add '{}' to entity attributes", key),
                    "Use default value for missing attribute".to_string(),
                    "Make this condition optional".to_string(),
                ]
            }
            Self::TypeMismatch { expected, actual } => {
                vec![
                    format!("Convert {} to {}", actual, expected),
                    "Change condition to accept current type".to_string(),
                    "Add type conversion in evaluation context".to_string(),
                ]
            }
            Self::InvalidFormula { .. } => {
                vec![
                    "Fix formula syntax".to_string(),
                    "Use simpler condition type instead of calculation".to_string(),
                    "Define missing variables in context".to_string(),
                ]
            }
            Self::PatternError { .. } => {
                vec![
                    "Fix regex syntax".to_string(),
                    "Escape special regex characters".to_string(),
                    "Use simpler string comparison instead".to_string(),
                ]
            }
            Self::MaxDepthExceeded { .. } => {
                vec![
                    "Flatten nested conditions using normalization".to_string(),
                    "Break complex condition into multiple simpler ones".to_string(),
                    "Check for and remove circular condition references".to_string(),
                ]
            }
            Self::Custom { .. } => vec![],
        }
    }
}
/// Detailed explanation of a condition evaluation.
///
/// Contains the evaluation result, the condition evaluated, and a trace
/// of all sub-evaluations that led to the final result.
///
/// # Examples
///
/// ```
/// # use legalis_core::{Condition, ComparisonOp, AttributeBasedContext, EvaluationExplanation};
/// # use std::collections::HashMap;
/// let mut attributes = HashMap::new();
/// attributes.insert("age".to_string(), "25".to_string());
/// attributes.insert("income".to_string(), "50000".to_string());
/// let ctx = AttributeBasedContext::new(attributes);
///
/// let age_check = Condition::age(ComparisonOp::GreaterOrEqual, 18);
/// let income_check = Condition::income(ComparisonOp::GreaterOrEqual, 30000);
/// let condition = age_check.and(income_check);
///
/// let (result, explanation) = condition.evaluate_with_explanation(&ctx).unwrap();
/// assert!(result);
/// println!("Explanation:\n{}", explanation);
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EvaluationExplanation {
    /// The condition that was evaluated (formatted as string).
    pub condition: String,
    /// The final evaluation result.
    pub conclusion: bool,
    /// Step-by-step trace of the evaluation.
    pub steps: Vec<ExplanationStep>,
}
/// Result of a reasoning step.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum StepResult {
    /// Condition was satisfied
    Satisfied,
    /// Condition was not satisfied
    NotSatisfied,
    /// Statute was applied
    Applied,
    /// Statute was not applicable
    NotApplicable,
    /// Uncertain result
    Uncertain,
}
/// Effect with temporal validity constraints.
///
/// Wraps an effect with start/end dates and optional recurrence pattern.
/// The effect is only active during specified time periods.
///
/// # Example
/// ```
/// # use legalis_core::{Effect, TemporalEffect, RecurrencePattern};
/// # use chrono::NaiveDate;
/// let effect = Effect::grant("summer internship");
/// let start = NaiveDate::from_ymd_opt(2025, 6, 1).unwrap();
/// let end = NaiveDate::from_ymd_opt(2025, 8, 31).unwrap();
///
/// let temporal = TemporalEffect::new(effect, start, Some(end), None);
/// assert!(temporal.is_active_on(NaiveDate::from_ymd_opt(2025, 7, 15).unwrap()));
/// assert!(!temporal.is_active_on(NaiveDate::from_ymd_opt(2025, 9, 1).unwrap()));
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct TemporalEffect {
    /// The underlying effect.
    pub effect: Effect,
    /// Start date (inclusive).
    pub start_date: NaiveDate,
    /// End date (inclusive), if any.
    pub end_date: Option<NaiveDate>,
    /// Recurrence pattern, if any.
    pub recurrence: Option<RecurrencePattern>,
}
impl TemporalEffect {
    /// Creates a new temporal effect.
    #[must_use]
    pub fn new(
        effect: Effect,
        start_date: NaiveDate,
        end_date: Option<NaiveDate>,
        recurrence: Option<RecurrencePattern>,
    ) -> Self {
        Self {
            effect,
            start_date,
            end_date,
            recurrence,
        }
    }
    /// Checks if the effect is active on a given date.
    #[must_use]
    pub fn is_active_on(&self, date: NaiveDate) -> bool {
        if date < self.start_date {
            return false;
        }
        if let Some(end) = self.end_date
            && date > end
        {
            return false;
        }
        if let Some(ref pattern) = self.recurrence {
            pattern.matches(date, self.start_date)
        } else {
            true
        }
    }
    /// Returns the next activation date after the given date.
    #[must_use]
    pub fn next_activation(&self, after: NaiveDate) -> Option<NaiveDate> {
        if let Some(ref pattern) = self.recurrence {
            pattern.next_occurrence(after, self.start_date, self.end_date)
        } else if after < self.start_date {
            Some(self.start_date)
        } else {
            None
        }
    }
}
