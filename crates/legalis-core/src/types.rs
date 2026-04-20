//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::builder_states::*;
pub use crate::typed_attributes::{AttributeError, AttributeValue, TypedAttributes};
use chrono::NaiveDate;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use super::functions::EvaluationContext;
use super::types_3::{
    ConflictReason, ContradictionType, EffectType, EvaluationError, ReasoningStep, StepResult,
    TemporalEffect,
};
use super::types_4::{RecurrencePattern, TemporalValidity};
use super::types_5::{ComposedEffect, Condition};
use super::types_6::Statute;

/// Abductive reasoning engine for explaining legal outcomes.
///
/// This engine works backwards from an observed outcome to determine which
/// statutes and conditions led to that outcome.
///
/// # Examples
///
/// ```
/// use legalis_core::{AbductiveReasoner, Statute, Effect, EffectType, Condition, ComparisonOp};
/// use legalis_core::{EvaluationContext, RegionType, RelationshipType, DurationUnit};
/// use chrono::NaiveDate;
///
/// struct Person { age: u32, income: u64 }
///
/// impl EvaluationContext for Person {
///     fn get_attribute(&self, _key: &str) -> Option<String> { None }
///     fn get_age(&self) -> Option<u32> { Some(self.age) }
///     fn get_income(&self) -> Option<u64> { Some(self.income) }
///     fn get_current_date(&self) -> Option<NaiveDate> { None }
///     fn check_geographic(&self, _region_type: RegionType, _region_id: &str) -> bool { false }
///     fn check_relationship(&self, _relationship_type: RelationshipType, _target_id: Option<&str>) -> bool { false }
///     fn get_residency_months(&self) -> Option<u32> { None }
///     fn get_duration(&self, _unit: DurationUnit) -> Option<u32> { None }
///     fn get_percentage(&self, _context: &str) -> Option<u32> { None }
///     fn evaluate_formula(&self, _formula: &str) -> Option<f64> { None }
/// }
///
/// let voting_law = Statute::new("vote", "Voting Rights", Effect::grant("vote"))
///     .with_precondition(Condition::age(ComparisonOp::GreaterOrEqual, 18));
///
/// let statutes = vec![voting_law];
/// let person = Person { age: 25, income: 50000 };
///
/// let reasoner = AbductiveReasoner::new(statutes);
/// let explanations = reasoner.explain_effect(Effect::grant("vote"), &person);
///
/// assert!(!explanations.is_empty());
/// println!("{}", explanations[0]);
/// ```
#[derive(Debug, Clone)]
pub struct AbductiveReasoner {
    statutes: Vec<Statute>,
}
impl AbductiveReasoner {
    /// Creates a new abductive reasoner with the given statutes.
    #[must_use]
    pub fn new(statutes: Vec<Statute>) -> Self {
        Self { statutes }
    }
    /// Explains why a specific effect occurred.
    ///
    /// Returns all possible explanations ranked by confidence.
    pub fn explain_effect<C: EvaluationContext>(
        &self,
        target_effect: Effect,
        context: &C,
    ) -> Vec<LegalExplanation> {
        let mut explanations = Vec::new();
        for statute in &self.statutes {
            if self.effects_match(&statute.effect, &target_effect)
                && let Some(explanation) = self.explain_statute(statute, context)
            {
                explanations.push(explanation);
            }
        }
        explanations.sort_by(|a, b| {
            b.confidence
                .partial_cmp(&a.confidence)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        explanations
    }
    /// Explains why a specific statute was or was not applied.
    pub fn explain_statute<C: EvaluationContext>(
        &self,
        statute: &Statute,
        context: &C,
    ) -> Option<LegalExplanation> {
        let mut reasoning_chain = Vec::new();
        let mut satisfied_conditions = Vec::new();
        let mut unsatisfied_conditions = Vec::new();
        let mut step_num = 1;
        if statute.preconditions.is_empty() {
            reasoning_chain.push(ReasoningStep {
                step: step_num,
                description: format!("Statute '{}' has no preconditions", statute.id),
                statute_id: Some(statute.id.clone()),
                condition: None,
                result: StepResult::Applied,
            });
            return Some(LegalExplanation {
                outcome: statute.effect.clone(),
                applicable_statutes: vec![statute.id.clone()],
                satisfied_conditions: vec!["No preconditions".to_string()],
                unsatisfied_conditions: Vec::new(),
                confidence: 1.0,
                reasoning_chain,
            });
        }
        for condition in &statute.preconditions {
            let condition_str = format!("{}", condition);
            match condition.evaluate(context) {
                Ok(true) => {
                    satisfied_conditions.push(condition_str.clone());
                    reasoning_chain.push(ReasoningStep {
                        step: step_num,
                        description: format!("Condition satisfied: {}", condition_str),
                        statute_id: Some(statute.id.clone()),
                        condition: Some(condition_str),
                        result: StepResult::Satisfied,
                    });
                }
                Ok(false) => {
                    unsatisfied_conditions.push(condition_str.clone());
                    reasoning_chain.push(ReasoningStep {
                        step: step_num,
                        description: format!("Condition not satisfied: {}", condition_str),
                        statute_id: Some(statute.id.clone()),
                        condition: Some(condition_str),
                        result: StepResult::NotSatisfied,
                    });
                }
                Err(_) => {
                    unsatisfied_conditions.push(condition_str.clone());
                    reasoning_chain.push(ReasoningStep {
                        step: step_num,
                        description: format!("Condition evaluation failed: {}", condition_str),
                        statute_id: Some(statute.id.clone()),
                        condition: Some(condition_str),
                        result: StepResult::Uncertain,
                    });
                }
            }
            step_num += 1;
        }
        let total_conditions = statute.preconditions.len();
        let satisfied_count = satisfied_conditions.len();
        let confidence = if total_conditions > 0 {
            satisfied_count as f64 / total_conditions as f64
        } else {
            1.0
        };
        let all_satisfied = unsatisfied_conditions.is_empty();
        let applicable_statutes = if all_satisfied {
            vec![statute.id.clone()]
        } else {
            Vec::new()
        };
        reasoning_chain.push(ReasoningStep {
            step: step_num,
            description: if all_satisfied {
                format!("Statute '{}' applies", statute.id)
            } else {
                format!("Statute '{}' does not apply", statute.id)
            },
            statute_id: Some(statute.id.clone()),
            condition: None,
            result: if all_satisfied {
                StepResult::Applied
            } else {
                StepResult::NotApplicable
            },
        });
        Some(LegalExplanation {
            outcome: statute.effect.clone(),
            applicable_statutes,
            satisfied_conditions,
            unsatisfied_conditions,
            confidence,
            reasoning_chain,
        })
    }
    /// Explains why a specific outcome did NOT occur.
    ///
    /// This is useful for understanding what conditions would need to be satisfied
    /// for a desired outcome.
    pub fn explain_why_not<C: EvaluationContext>(
        &self,
        target_effect: Effect,
        context: &C,
    ) -> Vec<LegalExplanation> {
        let mut explanations = Vec::new();
        for statute in &self.statutes {
            if self.effects_match(&statute.effect, &target_effect)
                && let Some(explanation) = self.explain_statute(statute, context)
                && explanation.confidence < 1.0
            {
                explanations.push(explanation);
            }
        }
        explanations.sort_by(|a, b| {
            b.confidence
                .partial_cmp(&a.confidence)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        explanations
    }
    /// Checks if two effects match for explanation purposes.
    fn effects_match(&self, effect1: &Effect, effect2: &Effect) -> bool {
        effect1.effect_type == effect2.effect_type
            && effect1.description.contains(&effect2.description)
    }
    /// Finds alternative paths to achieve an outcome.
    ///
    /// Returns explanations for all statutes that could produce the target effect,
    /// showing which conditions need to be satisfied for each path.
    pub fn find_alternatives<C: EvaluationContext>(
        &self,
        target_effect: Effect,
        context: &C,
    ) -> Vec<LegalExplanation> {
        let mut alternatives = Vec::new();
        for statute in &self.statutes {
            if self.effects_match(&statute.effect, &target_effect)
                && let Some(explanation) = self.explain_statute(statute, context)
            {
                alternatives.push(explanation);
            }
        }
        alternatives
    }
}
/// New York (US-NY) jurisdiction marker.
#[derive(Debug, Clone, Copy)]
pub struct NewYork;
/// California (US-CA) jurisdiction marker.
#[derive(Debug, Clone, Copy)]
pub struct California;
/// Effect that depends on runtime conditions.
///
/// The effect is only applied if the condition evaluates to true.
/// This allows for dynamic, context-dependent effects.
///
/// # Example
/// ```
/// # use legalis_core::{Effect, ConditionalEffect, Condition, ComparisonOp, AttributeBasedContext};
/// # use std::collections::HashMap;
/// let effect = Effect::grant("senior discount");
/// let condition = Condition::age(ComparisonOp::GreaterOrEqual, 65);
/// let conditional = ConditionalEffect::new(effect, condition);
///
/// let mut attributes = HashMap::new();
/// attributes.insert("age".to_string(), "70".to_string());
/// let ctx = AttributeBasedContext::new(attributes);
///
/// assert!(conditional.should_apply(&ctx).unwrap());
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConditionalEffect {
    /// The effect to apply conditionally.
    pub effect: Effect,
    /// The condition that must be satisfied.
    pub condition: Condition,
}
impl ConditionalEffect {
    /// Creates a new conditional effect.
    #[must_use]
    pub fn new(effect: Effect, condition: Condition) -> Self {
        Self { effect, condition }
    }
    /// Checks if the effect should be applied given an evaluation context.
    pub fn should_apply<C: EvaluationContext>(&self, context: &C) -> Result<bool, EvaluationError> {
        self.condition.evaluate(context)
    }
    /// Applies the effect if the condition is met, returns the effect or None.
    pub fn apply_if<C: EvaluationContext>(
        &self,
        context: &C,
    ) -> Result<Option<&Effect>, EvaluationError> {
        if self.should_apply(context)? {
            Ok(Some(&self.effect))
        } else {
            Ok(None)
        }
    }
}
/// Inference step in legal reasoning chains.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct InferenceStep {
    /// The statute applied in this step
    pub statute_id: String,
    /// The effect produced
    pub effect: Effect,
    /// Previous steps this inference depends on
    pub depends_on: Vec<usize>,
}
/// A single step in the evaluation trace.
///
/// Records one condition evaluation including timing and context.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExplanationStep {
    /// The condition being evaluated (formatted as string).
    pub condition: String,
    /// The result of this evaluation step.
    pub result: bool,
    /// Additional details about how the result was determined.
    pub details: String,
    /// Nesting depth (for compound conditions).
    pub depth: usize,
    /// Time taken for this evaluation step (in microseconds).
    pub duration_micros: u64,
}
/// Validation errors for statutes with error codes and severity.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum ValidationError {
    /// Statute ID is empty.
    EmptyId,
    /// Statute ID contains invalid characters.
    InvalidId(String),
    /// Statute title is empty.
    EmptyTitle,
    /// Expiry date is before effective date.
    ExpiryBeforeEffective {
        effective: NaiveDate,
        expiry: NaiveDate,
    },
    /// A precondition is invalid.
    InvalidCondition { index: usize, message: String },
    /// Effect description is empty.
    EmptyEffectDescription,
    /// Version must be > 0.
    InvalidVersion,
}
impl ValidationError {
    /// Returns the error code for this validation error.
    #[must_use]
    pub const fn error_code(&self) -> &'static str {
        match self {
            Self::EmptyId => "E001",
            Self::InvalidId(_) => "E002",
            Self::EmptyTitle => "E003",
            Self::ExpiryBeforeEffective { .. } => "E004",
            Self::InvalidCondition { .. } => "E005",
            Self::EmptyEffectDescription => "E006",
            Self::InvalidVersion => "E007",
        }
    }
    /// Returns the severity level of this error.
    #[must_use]
    pub const fn severity(&self) -> ErrorSeverity {
        match self {
            Self::EmptyId | Self::EmptyTitle | Self::EmptyEffectDescription => {
                ErrorSeverity::Critical
            }
            Self::InvalidId(_) | Self::InvalidVersion => ErrorSeverity::Error,
            Self::ExpiryBeforeEffective { .. } | Self::InvalidCondition { .. } => {
                ErrorSeverity::Warning
            }
        }
    }
    /// Returns a suggestion for how to fix this error.
    #[must_use]
    pub fn suggestion(&self) -> Option<&str> {
        match self {
            Self::EmptyId => Some("Provide a non-empty ID for the statute"),
            Self::InvalidId(_) => {
                Some("Use only alphanumeric characters, hyphens, and underscores in IDs")
            }
            Self::EmptyTitle => Some("Provide a descriptive title for the statute"),
            Self::ExpiryBeforeEffective { .. } => {
                Some("Ensure the expiry date is after the effective date")
            }
            Self::InvalidCondition { .. } => {
                Some("Review and fix the condition, or remove it if not needed")
            }
            Self::EmptyEffectDescription => Some("Provide a description for the effect"),
            Self::InvalidVersion => Some("Version must be greater than 0"),
        }
    }
    /// Returns multiple recovery options for this error.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::ValidationError;
    ///
    /// let err = ValidationError::EmptyId;
    /// let options = err.recovery_options();
    /// assert!(!options.is_empty());
    /// ```
    #[must_use]
    pub fn recovery_options(&self) -> Vec<String> {
        match self {
            Self::EmptyId => {
                vec![
                    "Generate a unique ID based on title".to_string(),
                    "Use a UUID as the ID".to_string(),
                    "Derive ID from jurisdiction and statute number".to_string(),
                ]
            }
            Self::InvalidId(id) => {
                vec![
                    format!("Remove invalid characters from '{}'", id),
                    "Replace spaces with hyphens or underscores".to_string(),
                    "Start ID with a letter if it begins with a number".to_string(),
                ]
            }
            Self::EmptyTitle => {
                vec![
                    "Add a descriptive title summarizing the statute".to_string(),
                    "Use the statute ID as a temporary title".to_string(),
                ]
            }
            Self::ExpiryBeforeEffective { effective, expiry } => {
                vec![
                    format!("Change expiry date to be after {}", effective),
                    format!("Change effective date to be before {}", expiry),
                    "Remove the expiry date if statute doesn't expire".to_string(),
                ]
            }
            Self::InvalidCondition { index, message } => {
                vec![
                    format!("Fix condition at index {}: {}", index, message),
                    format!("Remove condition at index {}", index),
                    "Simplify the condition to avoid validation issues".to_string(),
                ]
            }
            Self::EmptyEffectDescription => {
                vec![
                    "Add a description explaining what the effect does".to_string(),
                    "Use the effect type as a default description".to_string(),
                ]
            }
            Self::InvalidVersion => {
                vec![
                    "Set version to 1 for new statutes".to_string(),
                    "Increment version number from previous version".to_string(),
                ]
            }
        }
    }
    /// Attempts to automatically fix this error if possible.
    ///
    /// Returns a description of the fix applied, or None if auto-fix is not available.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::ValidationError;
    ///
    /// let err = ValidationError::InvalidId("my statute!".to_string());
    /// let fixed = err.try_auto_fix();
    /// assert!(fixed.is_some());
    /// ```
    #[must_use]
    pub fn try_auto_fix(&self) -> Option<(String, String)> {
        match self {
            Self::InvalidId(id) => {
                let fixed = id
                    .chars()
                    .map(|c| {
                        if c.is_alphanumeric() || c == '-' || c == '_' {
                            c
                        } else if c.is_whitespace() {
                            '-'
                        } else {
                            '_'
                        }
                    })
                    .collect::<String>();
                Some((
                    fixed,
                    "Replaced invalid characters with hyphens/underscores".to_string(),
                ))
            }
            Self::InvalidVersion => Some((
                "1".to_string(),
                "Set version to 1 (default for new statutes)".to_string(),
            )),
            _ => None,
        }
    }
}
/// Result of applying a statute in the entailment process.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct EntailmentResult {
    /// The statute that was applied
    pub statute_id: String,
    /// The effect that was produced
    pub effect: Effect,
    /// Whether all preconditions were satisfied
    pub conditions_satisfied: bool,
    /// Evaluation errors if any
    pub errors: Vec<String>,
}
/// Cache for memoizing condition evaluation results.
///
/// This cache improves performance when the same conditions are evaluated repeatedly
/// with the same entity attributes.
#[derive(Debug, Clone)]
pub struct ConditionCache {
    /// Cache storage mapping condition strings to evaluation results.
    pub(super) cache: HashMap<String, bool>,
    /// Maximum number of entries to store (LRU eviction).
    max_capacity: usize,
    /// Access order for LRU eviction.
    pub(super) access_order: Vec<String>,
}
impl ConditionCache {
    /// Creates a new cache with default capacity (1000).
    #[must_use]
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            max_capacity: 1000,
            access_order: Vec::new(),
        }
    }
    /// Creates a new cache with custom capacity.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            cache: HashMap::with_capacity(capacity),
            max_capacity: capacity,
            access_order: Vec::with_capacity(capacity),
        }
    }
    /// Gets a cached evaluation result if available.
    pub fn get(&mut self, condition_key: &str) -> Option<bool> {
        if let Some(&result) = self.cache.get(condition_key) {
            if let Some(pos) = self.access_order.iter().position(|k| k == condition_key) {
                self.access_order.remove(pos);
            }
            self.access_order.push(condition_key.to_string());
            Some(result)
        } else {
            None
        }
    }
    /// Stores an evaluation result in the cache.
    pub fn insert(&mut self, condition_key: String, result: bool) {
        if self.cache.len() >= self.max_capacity
            && !self.cache.contains_key(&condition_key)
            && let Some(oldest_key) = self.access_order.first().cloned()
        {
            self.cache.remove(&oldest_key);
            self.access_order.remove(0);
        }
        self.cache.insert(condition_key.clone(), result);
        self.access_order.push(condition_key);
    }
    /// Clears all cached entries.
    pub fn clear(&mut self) {
        self.cache.clear();
        self.access_order.clear();
    }
    /// Returns the number of cached entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.cache.len()
    }
    /// Returns true if the cache is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }
    /// Returns cache hit rate (for performance monitoring).
    #[must_use]
    pub fn hit_rate(&self) -> f64 {
        0.0
    }
}
/// United Kingdom jurisdiction marker.
#[derive(Debug, Clone, Copy)]
pub struct UK;
/// Cross-jurisdiction statute equivalence detector.
///
/// This analyzer identifies statutes from different jurisdictions that serve
/// equivalent legal purposes, even if their exact wording differs.
///
/// # Examples
///
/// ```
/// use legalis_core::{CrossJurisdictionAnalyzer, Statute, Effect, Condition, ComparisonOp};
///
/// let us_law = Statute::new("us-voting", "Voting Rights", Effect::grant("Vote"))
///     .with_jurisdiction("US")
///     .with_precondition(Condition::age(ComparisonOp::GreaterOrEqual, 18));
///
/// let uk_law = Statute::new("uk-voting", "Electoral Rights", Effect::grant("Vote"))
///     .with_jurisdiction("UK")
///     .with_precondition(Condition::age(ComparisonOp::GreaterOrEqual, 18));
///
/// let analyzer = CrossJurisdictionAnalyzer::new();
/// let candidates = vec![uk_law.clone()];
/// let equiv = analyzer.find_equivalents(&us_law, &candidates);
///
/// assert_eq!(equiv.len(), 1);
/// ```
pub struct CrossJurisdictionAnalyzer {
    /// Similarity threshold (0.0 to 1.0)
    similarity_threshold: f64,
}
impl CrossJurisdictionAnalyzer {
    /// Creates a new cross-jurisdiction analyzer with default threshold (0.7).
    #[must_use]
    pub fn new() -> Self {
        Self {
            similarity_threshold: 0.7,
        }
    }
    /// Creates a new analyzer with a custom similarity threshold.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::CrossJurisdictionAnalyzer;
    ///
    /// let analyzer = CrossJurisdictionAnalyzer::with_threshold(0.8);
    /// ```
    #[must_use]
    pub fn with_threshold(threshold: f64) -> Self {
        Self {
            similarity_threshold: threshold.clamp(0.0, 1.0),
        }
    }
    /// Finds statutes from different jurisdictions that are equivalent to the given statute.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{CrossJurisdictionAnalyzer, Statute, Effect, Condition, ComparisonOp};
    ///
    /// let reference = Statute::new("ref", "Age Requirement", Effect::grant("Benefit"))
    ///     .with_jurisdiction("US")
    ///     .with_precondition(Condition::age(ComparisonOp::GreaterOrEqual, 21));
    ///
    /// let candidate = Statute::new("can", "Age Eligibility", Effect::grant("Benefit"))
    ///     .with_jurisdiction("CA")
    ///     .with_precondition(Condition::age(ComparisonOp::GreaterOrEqual, 21));
    ///
    /// let analyzer = CrossJurisdictionAnalyzer::new();
    /// let candidates = vec![candidate];
    /// let equivalents = analyzer.find_equivalents(&reference, &candidates);
    ///
    /// assert_eq!(equivalents.len(), 1);
    /// ```
    #[must_use]
    pub fn find_equivalents<'a>(
        &self,
        reference: &Statute,
        candidates: &'a [Statute],
    ) -> Vec<&'a Statute> {
        candidates
            .iter()
            .filter(|candidate| {
                if reference.jurisdiction == candidate.jurisdiction {
                    return false;
                }
                let similarity = self.calculate_similarity(reference, candidate);
                similarity >= self.similarity_threshold
            })
            .collect()
    }
    /// Calculates similarity score between two statutes (0.0 to 1.0).
    ///
    /// Higher scores indicate greater equivalence.
    #[must_use]
    pub fn calculate_similarity(&self, s1: &Statute, s2: &Statute) -> f64 {
        let mut score = 0.0;
        let mut weight_sum = 0.0;
        let effect_weight = 0.4;
        if s1.effect.effect_type == s2.effect.effect_type {
            score += effect_weight;
        }
        weight_sum += effect_weight;
        let precond_weight = 0.3;
        let precond_similarity = if s1.preconditions.is_empty() && s2.preconditions.is_empty() {
            1.0
        } else {
            let min_count = s1.preconditions.len().min(s2.preconditions.len()) as f64;
            let max_count = s1.preconditions.len().max(s2.preconditions.len()) as f64;
            if max_count == 0.0 {
                0.0
            } else {
                min_count / max_count
            }
        };
        score += precond_weight * precond_similarity;
        weight_sum += precond_weight;
        let entity_weight = 0.3;
        let entity_similarity = if s1.applies_to.is_empty() && s2.applies_to.is_empty() {
            1.0
        } else {
            let common = s1
                .applies_to
                .iter()
                .filter(|t| s2.applies_to.contains(t))
                .count() as f64;
            let total = (s1.applies_to.len() + s2.applies_to.len()) as f64;
            if total == 0.0 {
                0.0
            } else {
                2.0 * common / total
            }
        };
        score += entity_weight * entity_similarity;
        weight_sum += entity_weight;
        score / weight_sum
    }
}
/// Legal effect produced when statute conditions are met.
///
/// Effects represent the legal consequences that occur when a statute's conditions are satisfied.
/// They can include granting rights, imposing obligations, or changing legal status.
///
/// # Examples
///
/// ```
/// use legalis_core::{Effect, EffectType};
///
/// let grant = Effect::new(EffectType::Grant, "Right to vote")
///     .with_parameter("scope", "federal")
///     .with_parameter("duration", "permanent");
///
/// assert_eq!(grant.effect_type, EffectType::Grant);
/// assert_eq!(grant.parameters.get("scope"), Some(&"federal".to_string()));
/// ```
///
/// ```
/// use legalis_core::{Effect, EffectType};
///
/// let tax = Effect::new(EffectType::MonetaryTransfer, "Income tax")
///     .with_parameter("rate", "0.22")
///     .with_parameter("bracket", "middle");
///
/// assert!(format!("{}", tax).contains("Income tax"));
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Effect {
    /// Type of effect
    pub effect_type: EffectType,
    /// Description of the effect
    pub description: String,
    /// Parameters for the effect
    pub parameters: std::collections::HashMap<String, String>,
}
impl Effect {
    /// Creates a new Effect.
    pub fn new(effect_type: EffectType, description: impl Into<String>) -> Self {
        Self {
            effect_type,
            description: description.into(),
            parameters: std::collections::HashMap::new(),
        }
    }
    /// Adds a parameter to the effect.
    pub fn with_parameter(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.parameters.insert(key.into(), value.into());
        self
    }
    /// Gets a parameter value by key.
    #[must_use]
    pub fn get_parameter(&self, key: &str) -> Option<&String> {
        self.parameters.get(key)
    }
    /// Checks if a parameter exists.
    #[must_use]
    pub fn has_parameter(&self, key: &str) -> bool {
        self.parameters.contains_key(key)
    }
    /// Returns the number of parameters.
    #[must_use]
    pub fn parameter_count(&self) -> usize {
        self.parameters.len()
    }
    /// Removes a parameter by key.
    pub fn remove_parameter(&mut self, key: &str) -> Option<String> {
        self.parameters.remove(key)
    }
    /// Creates a Grant effect.
    pub fn grant(description: impl Into<String>) -> Self {
        Self::new(EffectType::Grant, description)
    }
    /// Creates a Revoke effect.
    pub fn revoke(description: impl Into<String>) -> Self {
        Self::new(EffectType::Revoke, description)
    }
    /// Creates an Obligation effect.
    pub fn obligation(description: impl Into<String>) -> Self {
        Self::new(EffectType::Obligation, description)
    }
    /// Creates a Prohibition effect.
    pub fn prohibition(description: impl Into<String>) -> Self {
        Self::new(EffectType::Prohibition, description)
    }
    /// Composes multiple effects with priority ordering.
    ///
    /// Creates a `ComposedEffect` that represents the combination of multiple effects.
    /// Effects are applied in the order specified, with earlier effects having higher priority
    /// for conflict resolution.
    ///
    /// # Example
    /// ```
    /// # use legalis_core::{Effect, EffectType, CompositionStrategy};
    /// let grant = Effect::grant("access to resource");
    /// let obligation = Effect::obligation("must report annually");
    /// let revoke = Effect::revoke("temporary access");
    ///
    /// let composed = Effect::compose(vec![grant, obligation, revoke]);
    /// assert_eq!(composed.effects.len(), 3);
    /// assert_eq!(composed.resolution_strategy, CompositionStrategy::FirstWins);
    /// ```
    pub fn compose(effects: Vec<Effect>) -> ComposedEffect {
        ComposedEffect::new(effects)
    }
    /// Computes the inverse effect for rollback operations.
    ///
    /// Returns the effect that would reverse this effect's action.
    /// For example, Grant ↔ Revoke, Obligation ↔ lifting of obligation.
    ///
    /// # Returns
    /// - `Some(Effect)` if an inverse exists
    /// - `None` if the effect cannot be inverted (e.g., Custom effects)
    ///
    /// # Example
    /// ```
    /// # use legalis_core::{Effect, EffectType};
    /// let grant = Effect::grant("access to resource");
    /// let inverse = grant.inverse().unwrap();
    /// assert_eq!(inverse.effect_type, EffectType::Revoke);
    /// assert_eq!(inverse.description, "access to resource");
    ///
    /// let obligation = Effect::obligation("must file taxes");
    /// let inverse_obligation = obligation.inverse().unwrap();
    /// assert_eq!(inverse_obligation.effect_type, EffectType::Grant);
    /// assert!(inverse_obligation.description.contains("relief from"));
    /// ```
    #[must_use]
    pub fn inverse(&self) -> Option<Effect> {
        let (inverse_type, inverse_description) = match self.effect_type {
            EffectType::Grant => (EffectType::Revoke, self.description.clone()),
            EffectType::Revoke => (EffectType::Grant, self.description.clone()),
            EffectType::Obligation => (
                EffectType::Grant,
                format!("relief from {}", self.description),
            ),
            EffectType::Prohibition => (
                EffectType::Grant,
                format!("permission for {}", self.description),
            ),
            EffectType::MonetaryTransfer => {
                let desc = if self.description.contains("tax") {
                    self.description.replace("tax", "refund")
                } else if self.description.contains("fine") {
                    self.description.replace("fine", "reimbursement")
                } else {
                    format!("reverse {}", self.description)
                };
                (EffectType::MonetaryTransfer, desc)
            }
            EffectType::StatusChange => (
                EffectType::StatusChange,
                format!("reverse {}", self.description),
            ),
            EffectType::Custom => return None,
        };
        let mut inverse = Effect::new(inverse_type, inverse_description);
        inverse.parameters = self.parameters.clone();
        inverse
            .parameters
            .insert("_is_inverse".to_string(), "true".to_string());
        inverse.parameters.insert(
            "_original_type".to_string(),
            format!("{:?}", self.effect_type),
        );
        Some(inverse)
    }
    /// Checks if this effect is an inverse of another effect.
    #[must_use]
    pub fn is_inverse_of(&self, other: &Effect) -> bool {
        if let Some(inv) = other.inverse() {
            self.effect_type == inv.effect_type
                && (self.description == inv.description
                    || self.description.contains(&other.description))
        } else {
            false
        }
    }
    /// Creates a temporal effect with start/end times and recurrence.
    ///
    /// Wraps this effect in a `TemporalEffect` that controls when the effect is active.
    ///
    /// # Example
    /// ```
    /// # use legalis_core::{Effect, RecurrencePattern};
    /// # use chrono::{NaiveDate, Utc};
    /// let grant = Effect::grant("seasonal parking permit");
    /// let start = NaiveDate::from_ymd_opt(2025, 6, 1).unwrap();
    /// let end = NaiveDate::from_ymd_opt(2025, 9, 1).unwrap();
    ///
    /// let temporal = grant.with_temporal_validity(start, Some(end), None);
    /// assert!(temporal.is_active_on(NaiveDate::from_ymd_opt(2025, 7, 15).unwrap()));
    /// assert!(!temporal.is_active_on(NaiveDate::from_ymd_opt(2025, 10, 1).unwrap()));
    /// ```
    #[must_use]
    pub fn with_temporal_validity(
        self,
        start: NaiveDate,
        end: Option<NaiveDate>,
        recurrence: Option<RecurrencePattern>,
    ) -> TemporalEffect {
        TemporalEffect::new(self, start, end, recurrence)
    }
    /// Creates a conditional effect that depends on runtime conditions.
    ///
    /// The effect will only be applied if the condition evaluates to true
    /// at the time of application.
    ///
    /// # Example
    /// ```
    /// # use legalis_core::{Effect, Condition, ComparisonOp};
    /// let grant = Effect::grant("bonus payment");
    /// let condition = Condition::income(ComparisonOp::GreaterOrEqual, 50000);
    ///
    /// let conditional = grant.when(condition);
    /// assert_eq!(conditional.effect.description, "bonus payment");
    /// ```
    #[must_use]
    pub fn when(self, condition: Condition) -> ConditionalEffect {
        ConditionalEffect::new(self, condition)
    }
}
/// Error severity levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum ErrorSeverity {
    /// Warning - potential issue but not critical.
    Warning,
    /// Error - significant problem that should be addressed.
    Error,
    /// Critical - fundamental issue that prevents operation.
    Critical,
}
/// Source location information for error diagnostics.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct SourceLocation {
    /// File path or source identifier
    pub file: Option<String>,
    /// Line number (1-indexed)
    pub line: Option<usize>,
    /// Column number (1-indexed)
    pub column: Option<usize>,
    /// Source snippet for context
    pub snippet: Option<String>,
}
impl SourceLocation {
    /// Creates a new source location.
    #[must_use]
    pub fn new() -> Self {
        Self {
            file: None,
            line: None,
            column: None,
            snippet: None,
        }
    }
    /// Sets the file path.
    #[must_use]
    pub fn with_file(mut self, file: impl Into<String>) -> Self {
        self.file = Some(file.into());
        self
    }
    /// Sets the line number.
    #[must_use]
    pub const fn with_line(mut self, line: usize) -> Self {
        self.line = Some(line);
        self
    }
    /// Sets the column number.
    #[must_use]
    pub const fn with_column(mut self, column: usize) -> Self {
        self.column = Some(column);
        self
    }
    /// Sets the source snippet.
    #[must_use]
    pub fn with_snippet(mut self, snippet: impl Into<String>) -> Self {
        self.snippet = Some(snippet.into());
        self
    }
}
/// Represents a logical contradiction between statutes.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct Contradiction {
    /// ID of the first statute involved
    pub statute_a_id: String,
    /// ID of the second statute involved
    pub statute_b_id: String,
    /// Type of contradiction
    pub contradiction_type: ContradictionType,
    /// Human-readable description
    pub description: String,
    /// Severity of the contradiction
    pub severity: ErrorSeverity,
}
#[derive(Debug, Clone)]
pub(crate) enum ConditionOperation {
    None,
    And,
    Or,
}
/// Represents the outcome of a conflict resolution between two statutes.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum ConflictResolution {
    /// First statute prevails
    FirstPrevails(ConflictReason),
    /// Second statute prevails
    SecondPrevails(ConflictReason),
    /// No conflict - statutes are compatible
    NoConflict,
    /// Statutes conflict but cannot be automatically resolved
    Unresolvable(String),
}
/// Type-safe builder for `Statute` using the typestate pattern.
///
/// This builder ensures at compile time that all required fields (id, title, effect)
/// are set before building a statute. The type parameters track which fields have been set.
///
/// # Type Parameters
///
/// - `I`: ID state (`NoId` or `HasId`)
/// - `T`: Title state (`NoTitle` or `HasTitle`)
/// - `E`: Effect state (`NoEffect` or `HasEffect`)
///
/// # Examples
///
/// ```
/// use legalis_core::{TypedStatuteBuilder, Effect, EffectType, Condition, ComparisonOp};
///
/// // This compiles - all required fields are set
/// let statute = TypedStatuteBuilder::new()
///     .id("tax-law-2025")
///     .title("Income Tax Credit")
///     .effect(Effect::new(EffectType::Grant, "Tax credit of $1000"))
///     .with_precondition(Condition::Income {
///         operator: ComparisonOp::LessThan,
///         value: 50000,
///     })
///     .build();
///
/// assert_eq!(statute.id, "tax-law-2025");
/// ```
///
/// ```compile_fail
/// use legalis_core::TypedStatuteBuilder;
///
/// // This won't compile - missing title and effect
/// let statute = TypedStatuteBuilder::new()
///     .id("tax-law-2025")
///     .build(); // ERROR: build() not available
/// ```
#[derive(Debug, Clone)]
pub struct TypedStatuteBuilder<I, T, E> {
    pub(super) id: Option<String>,
    pub(super) title: Option<String>,
    pub(super) effect: Option<Effect>,
    pub(super) preconditions: Vec<Condition>,
    pub(super) discretion_logic: Option<String>,
    pub(super) temporal_validity: TemporalValidity,
    pub(super) version: u32,
    pub(super) jurisdiction: Option<String>,
    _phantom: std::marker::PhantomData<(I, T, E)>,
}
impl TypedStatuteBuilder<NoId, NoTitle, NoEffect> {
    /// Creates a new builder with no fields set.
    #[must_use]
    pub fn new() -> Self {
        Self {
            id: None,
            title: None,
            effect: None,
            preconditions: Vec::new(),
            discretion_logic: None,
            temporal_validity: TemporalValidity::default(),
            version: 1,
            jurisdiction: None,
            _phantom: std::marker::PhantomData,
        }
    }
}
impl<T, E> TypedStatuteBuilder<NoId, T, E> {
    /// Sets the statute ID (required field).
    ///
    /// Transitions from `NoId` to `HasId` state.
    #[must_use]
    pub fn id(self, id: impl Into<String>) -> TypedStatuteBuilder<HasId, T, E> {
        TypedStatuteBuilder {
            id: Some(id.into()),
            title: self.title,
            effect: self.effect,
            preconditions: self.preconditions,
            discretion_logic: self.discretion_logic,
            temporal_validity: self.temporal_validity,
            version: self.version,
            jurisdiction: self.jurisdiction,
            _phantom: std::marker::PhantomData,
        }
    }
}
impl<I, E> TypedStatuteBuilder<I, NoTitle, E> {
    /// Sets the statute title (required field).
    ///
    /// Transitions from `NoTitle` to `HasTitle` state.
    #[must_use]
    pub fn title(self, title: impl Into<String>) -> TypedStatuteBuilder<I, HasTitle, E> {
        TypedStatuteBuilder {
            id: self.id,
            title: Some(title.into()),
            effect: self.effect,
            preconditions: self.preconditions,
            discretion_logic: self.discretion_logic,
            temporal_validity: self.temporal_validity,
            version: self.version,
            jurisdiction: self.jurisdiction,
            _phantom: std::marker::PhantomData,
        }
    }
}
impl<I, T> TypedStatuteBuilder<I, T, NoEffect> {
    /// Sets the statute effect (required field).
    ///
    /// Transitions from `NoEffect` to `HasEffect` state.
    #[must_use]
    pub fn effect(self, effect: Effect) -> TypedStatuteBuilder<I, T, HasEffect> {
        TypedStatuteBuilder {
            id: self.id,
            title: self.title,
            effect: Some(effect),
            preconditions: self.preconditions,
            discretion_logic: self.discretion_logic,
            temporal_validity: self.temporal_validity,
            version: self.version,
            jurisdiction: self.jurisdiction,
            _phantom: std::marker::PhantomData,
        }
    }
}
impl<I, T, E> TypedStatuteBuilder<I, T, E> {
    /// Adds a precondition (optional field).
    #[must_use]
    pub fn with_precondition(mut self, condition: Condition) -> Self {
        self.preconditions.push(condition);
        self
    }
    /// Sets the discretion logic (optional field).
    #[must_use]
    pub fn with_discretion(mut self, logic: impl Into<String>) -> Self {
        self.discretion_logic = Some(logic.into());
        self
    }
    /// Sets temporal validity (optional field).
    #[must_use]
    pub fn with_temporal_validity(mut self, validity: TemporalValidity) -> Self {
        self.temporal_validity = validity;
        self
    }
    /// Sets the version (optional field, defaults to 1).
    #[must_use]
    pub fn with_version(mut self, version: u32) -> Self {
        self.version = version;
        self
    }
    /// Sets the jurisdiction (optional field).
    #[must_use]
    pub fn with_jurisdiction(mut self, jurisdiction: impl Into<String>) -> Self {
        self.jurisdiction = Some(jurisdiction.into());
        self
    }
}
impl TypedStatuteBuilder<HasId, HasTitle, HasEffect> {
    /// Builds the `Statute` (only available when all required fields are set).
    ///
    /// This method is only callable when the builder has transitioned through
    /// all required states (HasId, HasTitle, HasEffect).
    #[must_use]
    pub fn build(self) -> Statute {
        Statute {
            id: self.id.expect("ID must be set"),
            title: self.title.expect("Title must be set"),
            effect: self.effect.expect("Effect must be set"),
            preconditions: self.preconditions,
            discretion_logic: self.discretion_logic,
            temporal_validity: self.temporal_validity,
            version: self.version,
            jurisdiction: self.jurisdiction,
            derives_from: Vec::new(),
            applies_to: Vec::new(),
            exceptions: Vec::new(),
        }
    }
}
/// European Union jurisdiction marker.
#[derive(Debug, Clone, Copy)]
pub struct EU;
/// Generic marker for any jurisdiction.
#[derive(Debug, Clone, Copy)]
pub struct AnyJurisdiction;
/// Statute (legal article) definition.
///
/// A statute represents a legal rule with preconditions, effects, and optional discretionary logic.
/// Statutes follow an "If-Then-Else If Maybe" pattern:
/// - **If**: Preconditions must be met
/// - **Then**: Legal effect occurs
/// - **Else If Maybe**: Discretionary logic for edge cases
///
/// # Examples
///
/// ## Simple Statute
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
///
/// let voting_rights = Statute::new(
///     "voting-rights-act",
///     "Right to Vote",
///     Effect::new(EffectType::Grant, "Right to participate in elections"),
/// )
/// .with_precondition(Condition::Age {
///     operator: ComparisonOp::GreaterOrEqual,
///     value: 18,
/// })
/// .with_jurisdiction("US");
///
/// assert_eq!(voting_rights.id, "voting-rights-act");
/// assert!(voting_rights.is_valid());
/// ```
///
/// ## Statute with Temporal Validity
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType, TemporalValidity};
/// use chrono::NaiveDate;
///
/// let temporary_law = Statute::new(
///     "covid-relief-2025",
///     "COVID-19 Relief Act",
///     Effect::new(EffectType::Grant, "Emergency assistance"),
/// )
/// .with_temporal_validity(
///     TemporalValidity::new()
///         .with_effective_date(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())
///         .with_expiry_date(NaiveDate::from_ymd_opt(2025, 12, 31).unwrap())
/// );
///
/// assert!(temporary_law.is_active(NaiveDate::from_ymd_opt(2025, 6, 1).unwrap()));
/// assert!(!temporary_law.is_active(NaiveDate::from_ymd_opt(2026, 1, 1).unwrap()));
/// ```
///
/// ## Statute with Discretion
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
///
/// let employment_termination = Statute::new(
///     "just-cause-termination",
///     "Employment Termination for Just Cause",
///     Effect::new(EffectType::Grant, "Right to terminate employment"),
/// )
/// .with_discretion("Determine if just cause exists based on circumstances");
///
/// assert!(employment_termination.discretion_logic.is_some());
/// ```
/// A structured exception to a statute's application.
///
/// Exceptions represent specific circumstances where a statute does not apply,
/// even when its preconditions would otherwise be satisfied.
///
/// # Examples
///
/// ```
/// use legalis_core::{StatuteException, Condition, ComparisonOp};
///
/// let exception = StatuteException::new(
///     "minor-exception",
///     "Exception for minors",
///     Condition::age(ComparisonOp::LessThan, 18)
/// );
///
/// assert_eq!(exception.id, "minor-exception");
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct StatuteException {
    /// Unique identifier for this exception
    pub id: String,
    /// Description of the exception
    pub description: String,
    /// Condition under which the exception applies
    pub condition: Condition,
}
impl StatuteException {
    /// Creates a new statute exception.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{StatuteException, Condition, ComparisonOp};
    ///
    /// let exception = StatuteException::new(
    ///     "medical-exception",
    ///     "Exception for medical emergencies",
    ///     Condition::has_attribute("medical_emergency")
    /// );
    /// ```
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        description: impl Into<String>,
        condition: Condition,
    ) -> Self {
        Self {
            id: id.into(),
            description: description.into(),
            condition,
        }
    }
}
/// Statute dependency graph for tracking relationships between statutes.
///
/// The `StatuteGraph` maintains a directed graph where nodes are statutes
/// and edges represent various relationships (derivation, amendments, cross-references).
///
/// # Examples
///
/// ```
/// use legalis_core::{StatuteGraph, Statute, Effect};
///
/// let mut graph = StatuteGraph::new();
///
/// let federal_law = Statute::new("federal-1", "Federal Law", Effect::grant("Benefit"));
/// let state_law = Statute::new("state-1", "State Law", Effect::grant("Benefit"))
///     .with_derives_from("federal-1");
///
/// graph.add_statute(federal_law);
/// graph.add_statute(state_law);
///
/// let derived = graph.find_derived_from("federal-1");
/// assert_eq!(derived.len(), 1);
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct StatuteGraph {
    /// All statutes in the graph
    pub(super) statutes: std::collections::HashMap<String, Statute>,
    /// Adjacency list: statute_id -> list of related statute IDs
    pub(super) derivation_edges: std::collections::HashMap<String, Vec<String>>,
}
impl StatuteGraph {
    /// Creates a new empty statute graph.
    #[must_use]
    pub fn new() -> Self {
        Self {
            statutes: std::collections::HashMap::new(),
            derivation_edges: std::collections::HashMap::new(),
        }
    }
    /// Adds a statute to the graph.
    ///
    /// This automatically builds derivation edges based on the statute's `derives_from` field.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{StatuteGraph, Statute, Effect};
    ///
    /// let mut graph = StatuteGraph::new();
    /// let statute = Statute::new("law-1", "Law", Effect::grant("Benefit"));
    /// graph.add_statute(statute);
    ///
    /// assert_eq!(graph.len(), 1);
    /// ```
    pub fn add_statute(&mut self, statute: Statute) {
        let id = statute.id.clone();
        for source_id in &statute.derives_from {
            self.derivation_edges
                .entry(source_id.clone())
                .or_default()
                .push(id.clone());
        }
        self.statutes.insert(id, statute);
    }
    /// Returns the number of statutes in the graph.
    #[must_use]
    pub fn len(&self) -> usize {
        self.statutes.len()
    }
    /// Returns whether the graph is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.statutes.is_empty()
    }
    /// Gets a statute by ID.
    #[must_use]
    pub fn get(&self, id: &str) -> Option<&Statute> {
        self.statutes.get(id)
    }
    /// Finds all statutes derived from a given statute.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{StatuteGraph, Statute, Effect};
    ///
    /// let mut graph = StatuteGraph::new();
    ///
    /// graph.add_statute(Statute::new("parent", "Parent", Effect::grant("Benefit")));
    /// graph.add_statute(Statute::new("child", "Child", Effect::grant("Benefit"))
    ///     .with_derives_from("parent"));
    ///
    /// let derived = graph.find_derived_from("parent");
    /// assert_eq!(derived.len(), 1);
    /// assert_eq!(derived[0].id, "child");
    /// ```
    #[must_use]
    pub fn find_derived_from(&self, source_id: &str) -> Vec<&Statute> {
        self.derivation_edges
            .get(source_id)
            .map(|ids| ids.iter().filter_map(|id| self.statutes.get(id)).collect())
            .unwrap_or_default()
    }
    /// Finds all statutes that a given statute is derived from (its sources).
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{StatuteGraph, Statute, Effect};
    ///
    /// let mut graph = StatuteGraph::new();
    ///
    /// graph.add_statute(Statute::new("source-1", "Source 1", Effect::grant("B1")));
    /// graph.add_statute(Statute::new("source-2", "Source 2", Effect::grant("B2")));
    /// graph.add_statute(Statute::new("derived", "Derived", Effect::grant("B"))
    ///     .with_derives_from("source-1")
    ///     .with_derives_from("source-2"));
    ///
    /// let sources = graph.find_sources("derived");
    /// assert_eq!(sources.len(), 2);
    /// ```
    #[must_use]
    pub fn find_sources(&self, statute_id: &str) -> Vec<&Statute> {
        self.statutes
            .get(statute_id)
            .map(|statute| {
                statute
                    .derives_from
                    .iter()
                    .filter_map(|id| self.statutes.get(id))
                    .collect()
            })
            .unwrap_or_default()
    }
    /// Finds the transitive closure of all statutes derived from a given statute.
    ///
    /// This includes direct derivatives and all their derivatives recursively.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{StatuteGraph, Statute, Effect};
    ///
    /// let mut graph = StatuteGraph::new();
    ///
    /// graph.add_statute(Statute::new("root", "Root", Effect::grant("B")));
    /// graph.add_statute(Statute::new("child", "Child", Effect::grant("B"))
    ///     .with_derives_from("root"));
    /// graph.add_statute(Statute::new("grandchild", "Grandchild", Effect::grant("B"))
    ///     .with_derives_from("child"));
    ///
    /// let all_derived = graph.find_all_derived_from("root");
    /// assert_eq!(all_derived.len(), 2); // child and grandchild
    /// ```
    #[must_use]
    pub fn find_all_derived_from(&self, source_id: &str) -> Vec<&Statute> {
        let mut result = Vec::new();
        let mut visited = std::collections::HashSet::new();
        let mut queue = vec![source_id];
        while let Some(current_id) = queue.pop() {
            if !visited.insert(current_id) {
                continue;
            }
            if let Some(derived_ids) = self.derivation_edges.get(current_id) {
                for derived_id in derived_ids {
                    if let Some(statute) = self.statutes.get(derived_id) {
                        result.push(statute);
                        queue.push(derived_id);
                    }
                }
            }
        }
        result
    }
    /// Detects cycles in the derivation graph.
    ///
    /// Returns statute IDs that form derivation cycles (circular dependencies).
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{StatuteGraph, Statute, Effect};
    ///
    /// let mut graph = StatuteGraph::new();
    ///
    /// // Normal case: no cycles
    /// graph.add_statute(Statute::new("a", "A", Effect::grant("B")));
    /// graph.add_statute(Statute::new("b", "B", Effect::grant("B"))
    ///     .with_derives_from("a"));
    ///
    /// assert!(graph.detect_cycles().is_empty());
    /// ```
    #[must_use]
    pub fn detect_cycles(&self) -> Vec<Vec<String>> {
        let mut cycles = Vec::new();
        let mut visited = std::collections::HashSet::new();
        let mut rec_stack = std::collections::HashSet::new();
        let mut path = Vec::new();
        for id in self.statutes.keys() {
            if !visited.contains(id.as_str()) {
                self.detect_cycles_dfs(id, &mut visited, &mut rec_stack, &mut path, &mut cycles);
            }
        }
        cycles
    }
    #[allow(clippy::too_many_arguments)]
    fn detect_cycles_dfs(
        &self,
        node: &str,
        visited: &mut std::collections::HashSet<String>,
        rec_stack: &mut std::collections::HashSet<String>,
        path: &mut Vec<String>,
        cycles: &mut Vec<Vec<String>>,
    ) {
        visited.insert(node.to_string());
        rec_stack.insert(node.to_string());
        path.push(node.to_string());
        if let Some(neighbors) = self.derivation_edges.get(node) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    self.detect_cycles_dfs(neighbor, visited, rec_stack, path, cycles);
                } else if rec_stack.contains(neighbor)
                    && let Some(pos) = path.iter().position(|x| x == neighbor)
                {
                    cycles.push(path[pos..].to_vec());
                }
            }
        }
        path.pop();
        rec_stack.remove(node);
    }
    /// Returns an iterator over all statutes in the graph.
    pub fn iter(&self) -> impl Iterator<Item = &Statute> {
        self.statutes.values()
    }
}
/// Explanation for why a legal outcome occurred.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct LegalExplanation {
    /// The observed outcome being explained
    pub outcome: Effect,
    /// Statutes that contributed to this outcome
    pub applicable_statutes: Vec<String>,
    /// Conditions that were satisfied
    pub satisfied_conditions: Vec<String>,
    /// Conditions that were not satisfied
    pub unsatisfied_conditions: Vec<String>,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
    /// Step-by-step reasoning chain
    pub reasoning_chain: Vec<ReasoningStep>,
}
/// A type-safe implementation of LegalEntity using strongly-typed attributes.
///
/// This provides compile-time type safety and runtime validation for entity attributes,
/// replacing error-prone string parsing with explicit type handling.
///
/// # Examples
///
/// ```
/// use legalis_core::TypedEntity;
/// use chrono::NaiveDate;
///
/// let mut person = TypedEntity::new();
/// person.set_string("name", "Bob");
/// person.set_u32("age", 25);
/// person.set_bool("is_citizen", true);
/// person.set_date("birth_date", NaiveDate::from_ymd_opt(1999, 1, 15).unwrap());
///
/// assert_eq!(person.get_string("name").unwrap(), "Bob");
/// assert_eq!(person.get_u32("age").unwrap(), 25);
/// assert!(person.get_bool("is_citizen").unwrap());
///
/// // Type safety: attempting to get a string as a number returns an error
/// assert!(person.get_u32("name").is_err());
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct TypedEntity {
    pub(super) id: Uuid,
    pub(super) attributes: TypedAttributes,
}
impl TypedEntity {
    /// Creates a new TypedEntity with a random UUID.
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            attributes: TypedAttributes::new(),
        }
    }
    /// Creates a new TypedEntity with a specific UUID.
    pub fn with_id(id: Uuid) -> Self {
        Self {
            id,
            attributes: TypedAttributes::new(),
        }
    }
    /// Gets the typed attributes storage.
    pub fn attributes(&self) -> &TypedAttributes {
        &self.attributes
    }
    /// Gets mutable access to the typed attributes storage.
    pub fn attributes_mut(&mut self) -> &mut TypedAttributes {
        &mut self.attributes
    }
    /// Sets a u32 attribute.
    pub fn set_u32(&mut self, key: impl Into<String>, value: u32) {
        self.attributes.set_u32(key, value);
    }
    /// Gets a u32 attribute.
    pub fn get_u32(&self, key: &str) -> Result<u32, AttributeError> {
        self.attributes.get_u32(key)
    }
    /// Sets a u64 attribute.
    pub fn set_u64(&mut self, key: impl Into<String>, value: u64) {
        self.attributes.set_u64(key, value);
    }
    /// Gets a u64 attribute.
    pub fn get_u64(&self, key: &str) -> Result<u64, AttributeError> {
        self.attributes.get_u64(key)
    }
    /// Sets a boolean attribute.
    pub fn set_bool(&mut self, key: impl Into<String>, value: bool) {
        self.attributes.set_bool(key, value);
    }
    /// Gets a boolean attribute.
    pub fn get_bool(&self, key: &str) -> Result<bool, AttributeError> {
        self.attributes.get_bool(key)
    }
    /// Sets a string attribute.
    pub fn set_string(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.attributes.set_string(key, value);
    }
    /// Gets a string attribute.
    pub fn get_string(&self, key: &str) -> Result<&str, AttributeError> {
        self.attributes.get_string(key)
    }
    /// Sets a date attribute.
    pub fn set_date(&mut self, key: impl Into<String>, value: NaiveDate) {
        self.attributes.set_date(key, value);
    }
    /// Gets a date attribute.
    pub fn get_date(&self, key: &str) -> Result<NaiveDate, AttributeError> {
        self.attributes.get_date(key)
    }
    /// Sets an f64 attribute.
    pub fn set_f64(&mut self, key: impl Into<String>, value: f64) {
        self.attributes.set_f64(key, value);
    }
    /// Gets an f64 attribute.
    pub fn get_f64(&self, key: &str) -> Result<f64, AttributeError> {
        self.attributes.get_f64(key)
    }
    /// Sets a typed attribute value.
    pub fn set_typed(&mut self, key: impl Into<String>, value: AttributeValue) {
        self.attributes.set(key, value);
    }
    /// Gets a typed attribute value.
    pub fn get_typed(&self, key: &str) -> Option<&AttributeValue> {
        self.attributes.get(key)
    }
    /// Checks if an attribute exists.
    pub fn has_attribute(&self, key: &str) -> bool {
        self.attributes.has(key)
    }
}
