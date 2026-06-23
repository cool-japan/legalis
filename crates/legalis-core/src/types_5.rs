//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use chrono::NaiveDate;
#[cfg(feature = "parallel")]
use rayon::prelude::*;
#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::functions::EvaluationContext;
use super::types::{Effect, ExplanationStep, InferenceStep};
use super::types_3::{
    AttributeBasedContext, ConditionError, DurationUnit, EvaluationError, EvaluationExplanation,
    PartialBool, RegionType,
};
use super::types_4::{ComparisonOp, RelationshipType, StatuteChange};
use super::types_6::{CompositionStrategy, Statute};

/// Condition type for statute preconditions.
///
/// Conditions represent the requirements that must be met for a statute to apply.
/// They can be simple (age checks, attribute checks) or complex (combinations using AND/OR/NOT).
///
/// # Examples
///
/// ## Simple Condition
///
/// ```
/// use legalis_core::{Condition, ComparisonOp};
///
/// let age_check = Condition::Age {
///     operator: ComparisonOp::GreaterOrEqual,
///     value: 18,
/// };
///
/// assert_eq!(format!("{}", age_check), "age >= 18");
/// ```
///
/// ## Complex Condition
///
/// ```
/// use legalis_core::{Condition, ComparisonOp};
///
/// let age_check = Condition::Age {
///     operator: ComparisonOp::GreaterOrEqual,
///     value: 65,
/// };
/// let income_check = Condition::Income {
///     operator: ComparisonOp::LessThan,
///     value: 30000,
/// };
/// let eligibility = Condition::And(
///     Box::new(age_check),
///     Box::new(income_check),
/// );
///
/// assert!(format!("{}", eligibility).contains("AND"));
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum Condition {
    /// Age comparison (e.g., age >= 18)
    Age { operator: ComparisonOp, value: u32 },
    /// Income comparison
    Income { operator: ComparisonOp, value: u64 },
    /// Attribute existence check
    HasAttribute { key: String },
    /// Attribute value check
    AttributeEquals { key: String, value: String },
    /// Date range check (effective within date range)
    DateRange {
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
    },
    /// Geographic region check
    Geographic {
        region_type: RegionType,
        region_id: String,
    },
    /// Entity relationship check
    EntityRelationship {
        relationship_type: RelationshipType,
        target_entity_id: Option<String>,
    },
    /// Residency duration check
    ResidencyDuration { operator: ComparisonOp, months: u32 },
    /// Duration check (time periods, e.g., employment duration >= 5 years)
    Duration {
        operator: ComparisonOp,
        value: u32,
        unit: DurationUnit,
    },
    /// Percentage check (e.g., ownership >= 25%)
    Percentage {
        operator: ComparisonOp,
        value: u32,
        context: String,
    },
    /// Set membership check (e.g., status in {active, pending})
    SetMembership {
        attribute: String,
        values: Vec<String>,
        negated: bool,
    },
    /// Pattern matching check (regex for identifiers, codes, etc.)
    Pattern {
        attribute: String,
        pattern: String,
        negated: bool,
    },
    /// Calculation check (derived values, formulas)
    /// Example: `tax_owed = income * 0.2` where operator compares tax_owed
    Calculation {
        formula: String,
        operator: ComparisonOp,
        value: f64,
    },
    /// Composite condition - combines multiple conditions with weighted scoring
    /// Useful for complex eligibility where multiple factors contribute to a decision
    Composite {
        /// List of weighted conditions (weight, condition)
        /// Weights should be positive, typically 0.0-1.0 but not enforced
        conditions: Vec<(f64, Box<Condition>)>,
        /// Minimum total score required (sum of weights for satisfied conditions)
        threshold: f64,
    },
    /// Threshold condition - aggregate scoring across multiple numeric attributes
    /// Example: Combined income/asset test where total must exceed threshold
    Threshold {
        /// Attributes to sum (with optional multipliers)
        attributes: Vec<(String, f64)>,
        /// Comparison operator
        operator: ComparisonOp,
        /// Threshold value
        value: f64,
    },
    /// Fuzzy logic condition - membership in fuzzy set
    /// Supports gradual transitions between true/false
    Fuzzy {
        /// Attribute to evaluate
        attribute: String,
        /// Fuzzy set definition (value -> membership degree 0.0-1.0)
        /// For simplicity, uses linear interpolation between points
        membership_points: Vec<(f64, f64)>,
        /// Minimum membership degree required (0.0-1.0)
        min_membership: f64,
    },
    /// Probabilistic condition - probability-based evaluation
    /// Useful for modeling uncertain conditions or risk assessment
    Probabilistic {
        /// Base condition to evaluate
        condition: Box<Condition>,
        /// Probability that this condition is relevant (0.0-1.0)
        /// If p < 1.0, condition might be randomly evaluated as uncertain
        probability: f64,
        /// Minimum probability to consider condition satisfied
        threshold: f64,
    },
    /// Temporal condition - time-sensitive condition with decay/growth
    /// Value changes over time according to a decay or growth function
    Temporal {
        /// Base value at reference time
        base_value: f64,
        /// Reference timestamp (when base_value applies)
        reference_time: i64,
        /// Decay/growth rate per time unit (negative for decay, positive for growth)
        /// Applied as: value = base_value * (1 + rate)^time_elapsed
        rate: f64,
        /// Comparison operator
        operator: ComparisonOp,
        /// Target value to compare against
        target_value: f64,
    },
    /// Logical AND of conditions
    And(Box<Condition>, Box<Condition>),
    /// Logical OR of conditions
    Or(Box<Condition>, Box<Condition>),
    /// Logical NOT
    Not(Box<Condition>),
    /// Custom condition with description
    Custom { description: String },
}
impl Condition {
    /// Returns true if this is a compound condition (AND/OR/NOT).
    #[must_use]
    pub const fn is_compound(&self) -> bool {
        matches!(self, Self::And(..) | Self::Or(..) | Self::Not(..))
    }
    /// Returns true if this is a simple (non-compound) condition.
    #[must_use]
    pub const fn is_simple(&self) -> bool {
        !self.is_compound()
    }
    /// Returns true if this is a logical negation.
    #[must_use]
    pub const fn is_negation(&self) -> bool {
        matches!(self, Self::Not(..))
    }
    /// Counts the total number of conditions (including nested ones).
    #[must_use]
    pub fn count_conditions(&self) -> usize {
        match self {
            Self::And(left, right) | Self::Or(left, right) => {
                1 + left.count_conditions() + right.count_conditions()
            }
            Self::Not(inner) => 1 + inner.count_conditions(),
            Self::Composite { conditions, .. } => {
                1 + conditions
                    .iter()
                    .map(|(_, c)| c.count_conditions())
                    .sum::<usize>()
            }
            Self::Probabilistic { condition, .. } => 1 + condition.count_conditions(),
            _ => 1,
        }
    }
    /// Returns the depth of nested conditions.
    #[must_use]
    pub fn depth(&self) -> usize {
        match self {
            Self::And(left, right) | Self::Or(left, right) => 1 + left.depth().max(right.depth()),
            Self::Not(inner) => 1 + inner.depth(),
            Self::Composite { conditions, .. } => {
                1 + conditions.iter().map(|(_, c)| c.depth()).max().unwrap_or(0)
            }
            Self::Probabilistic { condition, .. } => 1 + condition.depth(),
            _ => 1,
        }
    }
    /// Creates a new Age condition.
    pub fn age(operator: ComparisonOp, value: u32) -> Self {
        Self::Age { operator, value }
    }
    /// Creates a new Income condition.
    pub fn income(operator: ComparisonOp, value: u64) -> Self {
        Self::Income { operator, value }
    }
    /// Creates a new HasAttribute condition.
    pub fn has_attribute(key: impl Into<String>) -> Self {
        Self::HasAttribute { key: key.into() }
    }
    /// Creates a new AttributeEquals condition.
    pub fn attribute_equals(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self::AttributeEquals {
            key: key.into(),
            value: value.into(),
        }
    }
    /// Creates a new Custom condition.
    pub fn custom(description: impl Into<String>) -> Self {
        Self::Custom {
            description: description.into(),
        }
    }
    /// Creates a new Duration condition.
    pub fn duration(operator: ComparisonOp, value: u32, unit: DurationUnit) -> Self {
        Self::Duration {
            operator,
            value,
            unit,
        }
    }
    /// Creates a new Percentage condition.
    pub fn percentage(operator: ComparisonOp, value: u32, context: impl Into<String>) -> Self {
        Self::Percentage {
            operator,
            value,
            context: context.into(),
        }
    }
    /// Creates a new SetMembership condition (attribute must be in set).
    pub fn in_set(attribute: impl Into<String>, values: Vec<String>) -> Self {
        Self::SetMembership {
            attribute: attribute.into(),
            values,
            negated: false,
        }
    }
    /// Creates a new SetMembership condition (attribute must NOT be in set).
    pub fn not_in_set(attribute: impl Into<String>, values: Vec<String>) -> Self {
        Self::SetMembership {
            attribute: attribute.into(),
            values,
            negated: true,
        }
    }
    /// Creates a new Pattern condition (attribute matches regex).
    pub fn matches_pattern(attribute: impl Into<String>, pattern: impl Into<String>) -> Self {
        Self::Pattern {
            attribute: attribute.into(),
            pattern: pattern.into(),
            negated: false,
        }
    }
    /// Creates a new Pattern condition (attribute does NOT match regex).
    pub fn not_matches_pattern(attribute: impl Into<String>, pattern: impl Into<String>) -> Self {
        Self::Pattern {
            attribute: attribute.into(),
            pattern: pattern.into(),
            negated: true,
        }
    }
    /// Creates a new Calculation condition (formula-based check).
    ///
    /// # Examples
    /// ```
    /// use legalis_core::{Condition, ComparisonOp};
    ///
    /// let tax_check = Condition::calculation("income * 0.2", ComparisonOp::GreaterThan, 5000.0);
    /// ```
    pub fn calculation(formula: impl Into<String>, operator: ComparisonOp, value: f64) -> Self {
        Self::Calculation {
            formula: formula.into(),
            operator,
            value,
        }
    }
    /// Creates a new Composite condition with weighted sub-conditions.
    ///
    /// # Arguments
    /// * `conditions` - Vector of (weight, condition) pairs
    /// * `threshold` - Minimum total score required
    ///
    /// # Example
    /// ```
    /// # use legalis_core::{Condition, ComparisonOp};
    /// let cond = Condition::composite(
    ///     vec![
    ///         (0.5, Box::new(Condition::age(ComparisonOp::GreaterOrEqual, 18))),
    ///         (0.3, Box::new(Condition::income(ComparisonOp::GreaterOrEqual, 30000))),
    ///     ],
    ///     0.6
    /// );
    /// ```
    pub fn composite(conditions: Vec<(f64, Box<Condition>)>, threshold: f64) -> Self {
        Self::Composite {
            conditions,
            threshold,
        }
    }
    /// Creates a new Threshold condition for aggregate scoring.
    ///
    /// # Arguments
    /// * `attributes` - Vector of (attribute_name, multiplier) pairs
    /// * `operator` - Comparison operator
    /// * `value` - Threshold value
    ///
    /// # Example
    /// ```
    /// # use legalis_core::{Condition, ComparisonOp};
    /// // Total assets (income + 10*savings) must be >= 50000
    /// let cond = Condition::threshold(
    ///     vec![("income".to_string(), 1.0), ("savings".to_string(), 10.0)],
    ///     ComparisonOp::GreaterOrEqual,
    ///     50000.0
    /// );
    /// ```
    pub fn threshold(attributes: Vec<(String, f64)>, operator: ComparisonOp, value: f64) -> Self {
        Self::Threshold {
            attributes,
            operator,
            value,
        }
    }
    /// Creates a new Fuzzy condition for gradual membership.
    ///
    /// # Arguments
    /// * `attribute` - Attribute to evaluate
    /// * `membership_points` - Vector of (value, membership_degree) pairs for linear interpolation
    /// * `min_membership` - Minimum membership degree required (0.0-1.0)
    ///
    /// # Example
    /// ```
    /// # use legalis_core::Condition;
    /// // Age is "young" with fuzzy membership
    /// let cond = Condition::fuzzy(
    ///     "age".to_string(),
    ///     vec![(0.0, 1.0), (25.0, 0.5), (50.0, 0.0)],
    ///     0.5
    /// );
    /// ```
    pub fn fuzzy(
        attribute: String,
        membership_points: Vec<(f64, f64)>,
        min_membership: f64,
    ) -> Self {
        Self::Fuzzy {
            attribute,
            membership_points,
            min_membership,
        }
    }
    /// Creates a new Probabilistic condition.
    ///
    /// # Arguments
    /// * `condition` - Base condition to evaluate
    /// * `probability` - Probability that this condition is relevant (0.0-1.0)
    /// * `threshold` - Minimum probability to consider satisfied
    ///
    /// # Example
    /// ```
    /// # use legalis_core::{Condition, ComparisonOp};
    /// // 80% chance that age >= 18 is relevant
    /// let cond = Condition::probabilistic(
    ///     Box::new(Condition::age(ComparisonOp::GreaterOrEqual, 18)),
    ///     0.8,
    ///     0.5
    /// );
    /// ```
    pub fn probabilistic(condition: Box<Condition>, probability: f64, threshold: f64) -> Self {
        Self::Probabilistic {
            condition,
            probability,
            threshold,
        }
    }
    /// Creates a new Temporal condition with decay/growth over time.
    ///
    /// # Arguments
    /// * `base_value` - Value at reference time
    /// * `reference_time` - Reference timestamp
    /// * `rate` - Decay/growth rate (negative for decay, positive for growth)
    /// * `operator` - Comparison operator
    /// * `target_value` - Target value to compare against
    ///
    /// # Example
    /// ```
    /// # use legalis_core::{Condition, ComparisonOp};
    /// // Asset value decays 5% per year, must stay above 10000
    /// let cond = Condition::temporal(
    ///     100000.0,
    ///     1609459200, // Jan 1, 2021
    ///     -0.05,
    ///     ComparisonOp::GreaterOrEqual,
    ///     10000.0
    /// );
    /// ```
    pub fn temporal(
        base_value: f64,
        reference_time: i64,
        rate: f64,
        operator: ComparisonOp,
        target_value: f64,
    ) -> Self {
        Self::Temporal {
            base_value,
            reference_time,
            rate,
            operator,
            target_value,
        }
    }
    /// Combines this condition with another using AND.
    pub fn and(self, other: Condition) -> Self {
        Self::And(Box::new(self), Box::new(other))
    }
    /// Combines this condition with another using OR.
    pub fn or(self, other: Condition) -> Self {
        Self::Or(Box::new(self), Box::new(other))
    }
    /// Negates this condition.
    #[allow(clippy::should_implement_trait)]
    pub fn not(self) -> Self {
        Self::Not(Box::new(self))
    }
    /// Normalizes this condition by applying logical simplifications.
    ///
    /// This method optimizes conditions by:
    /// - Removing double negations: `NOT (NOT A)` → `A`
    /// - Applying De Morgan's laws: `NOT (A AND B)` → `(NOT A) OR (NOT B)`
    /// - Recursively normalizing sub-conditions
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Condition, ComparisonOp};
    ///
    /// // Double negation elimination
    /// let condition = Condition::age(ComparisonOp::GreaterOrEqual, 18).not().not();
    /// let normalized = condition.normalize();
    /// // normalized is equivalent to: age >= 18
    /// ```
    #[must_use]
    pub fn normalize(self) -> Self {
        match self {
            Self::Not(inner) => match *inner {
                Self::Not(double_inner) => double_inner.normalize(),
                Self::And(left, right) => Self::Or(
                    Box::new(Self::Not(left).normalize()),
                    Box::new(Self::Not(right).normalize()),
                ),
                Self::Or(left, right) => Self::And(
                    Box::new(Self::Not(left).normalize()),
                    Box::new(Self::Not(right).normalize()),
                ),
                other => Self::Not(Box::new(other.normalize())),
            },
            Self::And(left, right) => {
                Self::And(Box::new(left.normalize()), Box::new(right.normalize()))
            }
            Self::Or(left, right) => {
                Self::Or(Box::new(left.normalize()), Box::new(right.normalize()))
            }
            other => other,
        }
    }
    /// Checks if this condition is in normalized form.
    #[must_use]
    pub fn is_normalized(&self) -> bool {
        match self {
            Self::Not(inner) => !matches!(**inner, Self::Not(_)) && inner.is_normalized(),
            Self::And(left, right) | Self::Or(left, right) => {
                left.is_normalized() && right.is_normalized()
            }
            _ => true,
        }
    }
    /// Evaluates this condition with lazy evaluation and short-circuit logic.
    ///
    /// This method implements:
    /// - **Short-circuit AND**: Returns false as soon as any condition is false
    /// - **Short-circuit OR**: Returns true as soon as any condition is true
    /// - **Maximum depth protection**: Prevents stack overflow from deeply nested conditions
    ///
    /// # Arguments
    ///
    /// * `ctx` - Evaluation context containing entity data and settings
    ///
    /// # Errors
    ///
    /// Returns [`ConditionError`] if:
    /// - Required attributes are missing
    /// - Type mismatches occur
    /// - Formula evaluation fails
    /// - Maximum evaluation depth is exceeded
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Condition, ComparisonOp, AttributeBasedContext};
    /// use std::collections::HashMap;
    ///
    /// let mut attrs = HashMap::new();
    /// attrs.insert("age".to_string(), "25".to_string());
    /// attrs.insert("income".to_string(), "45000".to_string());
    ///
    /// let ctx = AttributeBasedContext::new(attrs);
    /// let condition = Condition::age(ComparisonOp::GreaterOrEqual, 18)
    ///     .and(Condition::income(ComparisonOp::LessThan, 50000));
    ///
    /// assert_eq!(condition.evaluate_simple(&ctx).unwrap(), true);
    /// ```
    pub fn evaluate_simple(&self, ctx: &AttributeBasedContext) -> Result<bool, ConditionError> {
        self.evaluate_simple_with_depth(ctx, 0)
    }
    /// Internal evaluation with depth tracking.
    fn evaluate_simple_with_depth(
        &self,
        ctx: &AttributeBasedContext,
        depth: usize,
    ) -> Result<bool, ConditionError> {
        if depth > ctx.max_depth {
            return Err(ConditionError::MaxDepthExceeded {
                max_depth: ctx.max_depth,
            });
        }
        match self {
            Self::And(left, right) => {
                let left_result = left.evaluate_simple_with_depth(ctx, depth + 1)?;
                if !left_result {
                    return Ok(false);
                }
                right.evaluate_simple_with_depth(ctx, depth + 1)
            }
            Self::Or(left, right) => {
                let left_result = left.evaluate_simple_with_depth(ctx, depth + 1)?;
                if left_result {
                    return Ok(true);
                }
                right.evaluate_simple_with_depth(ctx, depth + 1)
            }
            Self::Not(inner) => {
                let result = inner.evaluate_simple_with_depth(ctx, depth + 1)?;
                Ok(!result)
            }
            Self::Age { operator, value } => {
                let age_str =
                    ctx.attributes
                        .get("age")
                        .ok_or_else(|| ConditionError::MissingAttribute {
                            key: "age".to_string(),
                        })?;
                let age: u32 = age_str.parse().map_err(|_| ConditionError::TypeMismatch {
                    expected: "u32".to_string(),
                    actual: age_str.clone(),
                })?;
                Ok(operator.compare_u32(age, *value))
            }
            Self::Income { operator, value } => {
                let income_str = ctx.attributes.get("income").ok_or_else(|| {
                    ConditionError::MissingAttribute {
                        key: "income".to_string(),
                    }
                })?;
                let income: u64 = income_str
                    .parse()
                    .map_err(|_| ConditionError::TypeMismatch {
                        expected: "u64".to_string(),
                        actual: income_str.clone(),
                    })?;
                Ok(operator.compare_u64(income, *value))
            }
            Self::HasAttribute { key } => Ok(ctx.attributes.contains_key(key)),
            Self::AttributeEquals { key, value } => Ok(ctx.attributes.get(key) == Some(value)),
            Self::Calculation {
                formula,
                operator,
                value,
            } => {
                let result = Self::evaluate_formula(formula, ctx)?;
                Ok(operator.compare_f64(result, *value))
            }
            Self::Pattern {
                attribute,
                pattern,
                negated,
            } => {
                let attr_value = ctx.attributes.get(attribute).ok_or_else(|| {
                    ConditionError::MissingAttribute {
                        key: attribute.clone(),
                    }
                })?;
                let matches = attr_value.contains(pattern);
                Ok(if *negated { !matches } else { matches })
            }
            Self::ResidencyDuration { operator, months } => {
                let residency_str = ctx.attributes.get("residency_months").ok_or_else(|| {
                    ConditionError::MissingAttribute {
                        key: "residency_months".to_string(),
                    }
                })?;
                let residency: u32 =
                    residency_str
                        .parse()
                        .map_err(|_| ConditionError::TypeMismatch {
                            expected: "u32".to_string(),
                            actual: residency_str.clone(),
                        })?;
                Ok(operator.compare_u32(residency, *months))
            }
            _ => Ok(true),
        }
    }
    /// Evaluates a formula using the shared pure-Rust recursive-descent parser.
    ///
    /// Variables are resolved from `ctx.attributes` by parsing their string
    /// values as `f64`. Returns `Err(ConditionError::InvalidFormula)` on any
    /// parse or evaluation failure.
    fn evaluate_formula(formula: &str, ctx: &AttributeBasedContext) -> Result<f64, ConditionError> {
        let resolve = |name: &str| -> Option<f64> {
            ctx.attributes.get(name).and_then(|s| s.parse::<f64>().ok())
        };
        crate::oracle::formula_eval::eval(formula, &resolve).map_err(|reason| {
            ConditionError::InvalidFormula {
                formula: formula.to_string(),
                error: reason,
            }
        })
    }
    /// Evaluates this condition using the `EvaluationContext` trait.
    ///
    /// This is the trait-based evaluation method that allows custom context implementations.
    /// For a simpler attribute-based approach, see [`evaluate_simple`](Self::evaluate_simple).
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Condition, ComparisonOp, EvaluationContext, RegionType, RelationshipType, DurationUnit};
    /// use chrono::NaiveDate;
    ///
    /// struct MyContext {
    ///     age: u32,
    ///     income: u64,
    /// }
    ///
    /// impl EvaluationContext for MyContext {
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
    /// let ctx = MyContext { age: 25, income: 45000 };
    /// let condition = Condition::age(ComparisonOp::GreaterOrEqual, 18)
    ///     .and(Condition::income(ComparisonOp::LessThan, 50000));
    ///
    /// assert_eq!(condition.evaluate(&ctx).unwrap(), true);
    /// ```
    pub fn evaluate<C: EvaluationContext>(&self, context: &C) -> Result<bool, EvaluationError> {
        self.evaluate_with_depth(context, 0)
    }
    /// Internal evaluation with depth tracking using the `EvaluationContext` trait.
    fn evaluate_with_depth<C: EvaluationContext>(
        &self,
        context: &C,
        depth: usize,
    ) -> Result<bool, EvaluationError> {
        const MAX_DEPTH: usize = 100;
        if depth > MAX_DEPTH {
            return Err(EvaluationError::MaxDepthExceeded {
                max_depth: MAX_DEPTH,
            });
        }
        match self {
            Self::And(left, right) => {
                let left_result = left.evaluate_with_depth(context, depth + 1)?;
                if !left_result {
                    return Ok(false);
                }
                right.evaluate_with_depth(context, depth + 1)
            }
            Self::Or(left, right) => {
                let left_result = left.evaluate_with_depth(context, depth + 1)?;
                if left_result {
                    return Ok(true);
                }
                right.evaluate_with_depth(context, depth + 1)
            }
            Self::Not(inner) => {
                let result = inner.evaluate_with_depth(context, depth + 1)?;
                Ok(!result)
            }
            Self::Age { operator, value } => {
                let age = context
                    .get_age()
                    .ok_or_else(|| EvaluationError::MissingAttribute {
                        key: "age".to_string(),
                    })?;
                Ok(operator.compare_u32(age, *value))
            }
            Self::Income { operator, value } => {
                let income =
                    context
                        .get_income()
                        .ok_or_else(|| EvaluationError::MissingAttribute {
                            key: "income".to_string(),
                        })?;
                Ok(operator.compare_u64(income, *value))
            }
            Self::HasAttribute { key } => Ok(context.get_attribute(key).is_some()),
            Self::AttributeEquals { key, value } => {
                Ok(context.get_attribute(key).as_ref() == Some(value))
            }
            Self::Geographic {
                region_type,
                region_id,
            } => Ok(context.check_geographic(*region_type, region_id)),
            Self::EntityRelationship {
                relationship_type,
                target_entity_id,
            } => Ok(context.check_relationship(*relationship_type, target_entity_id.as_deref())),
            Self::ResidencyDuration { operator, months } => {
                let residency = context.get_residency_months().ok_or_else(|| {
                    EvaluationError::MissingContext {
                        description: "residency months".to_string(),
                    }
                })?;
                Ok(operator.compare_u32(residency, *months))
            }
            Self::Duration {
                operator,
                value,
                unit,
            } => {
                let duration =
                    context
                        .get_duration(*unit)
                        .ok_or_else(|| EvaluationError::MissingContext {
                            description: format!("duration for unit {:?}", unit),
                        })?;
                Ok(operator.compare_u32(duration, *value))
            }
            Self::Percentage {
                operator,
                value,
                context: pct_context,
            } => {
                let percentage = context.get_percentage(pct_context).ok_or_else(|| {
                    EvaluationError::MissingContext {
                        description: format!("percentage for context '{}'", pct_context),
                    }
                })?;
                Ok(operator.compare_u32(percentage, *value))
            }
            Self::Calculation {
                formula,
                operator,
                value,
            } => {
                let result = context.evaluate_formula(formula).ok_or_else(|| {
                    EvaluationError::InvalidFormula {
                        formula: formula.clone(),
                        reason: "Formula evaluation not supported".to_string(),
                    }
                })?;
                Ok(operator.compare_f64(result, *value))
            }
            Self::Pattern {
                attribute,
                pattern,
                negated,
            } => {
                let attr_value = context.get_attribute(attribute).ok_or_else(|| {
                    EvaluationError::MissingAttribute {
                        key: attribute.clone(),
                    }
                })?;
                let matches = attr_value.contains(pattern);
                Ok(if *negated { !matches } else { matches })
            }
            Self::SetMembership {
                attribute,
                values,
                negated,
            } => {
                let attr_value = context.get_attribute(attribute).ok_or_else(|| {
                    EvaluationError::MissingAttribute {
                        key: attribute.clone(),
                    }
                })?;
                let is_member = values.contains(&attr_value);
                Ok(if *negated { !is_member } else { is_member })
            }
            Self::DateRange { start, end } => {
                let current_date =
                    context
                        .get_current_date()
                        .ok_or_else(|| EvaluationError::MissingContext {
                            description: "current date".to_string(),
                        })?;
                let after_start = start.is_none_or(|s| current_date >= s);
                let before_end = end.is_none_or(|e| current_date <= e);
                Ok(after_start && before_end)
            }
            Self::Composite {
                conditions,
                threshold,
            } => {
                let mut total_score = 0.0;
                for (weight, condition) in conditions {
                    let satisfied = condition.evaluate_with_depth(context, depth + 1)?;
                    if satisfied {
                        total_score += weight;
                    }
                }
                Ok(total_score >= *threshold)
            }
            Self::Threshold {
                attributes,
                operator,
                value,
            } => {
                let mut total = 0.0;
                for (attr_name, multiplier) in attributes {
                    let attr_value = context
                        .get_attribute(attr_name)
                        .and_then(|s| s.parse::<f64>().ok())
                        .ok_or_else(|| EvaluationError::MissingAttribute {
                            key: attr_name.clone(),
                        })?;
                    total += attr_value * multiplier;
                }
                Ok(operator.compare_f64(total, *value))
            }
            Self::Fuzzy {
                attribute,
                membership_points,
                min_membership,
            } => {
                let attr_value = context
                    .get_attribute(attribute)
                    .and_then(|s| s.parse::<f64>().ok())
                    .ok_or_else(|| EvaluationError::MissingAttribute {
                        key: attribute.clone(),
                    })?;
                let membership = if membership_points.is_empty() {
                    0.0
                } else if membership_points.len() == 1 {
                    membership_points[0].1
                } else {
                    let mut sorted = membership_points.clone();
                    sorted
                        .sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
                    if attr_value <= sorted[0].0 {
                        sorted[0].1
                    } else if attr_value >= sorted[sorted.len() - 1].0 {
                        sorted[sorted.len() - 1].1
                    } else {
                        let mut result = 0.0;
                        for i in 0..sorted.len() - 1 {
                            if attr_value >= sorted[i].0 && attr_value <= sorted[i + 1].0 {
                                let (x0, y0) = sorted[i];
                                let (x1, y1) = sorted[i + 1];
                                let t = (attr_value - x0) / (x1 - x0);
                                result = y0 + t * (y1 - y0);
                                break;
                            }
                        }
                        result
                    }
                };
                Ok(membership >= *min_membership)
            }
            Self::Probabilistic {
                condition,
                probability,
                threshold,
            } => {
                let satisfied = condition.evaluate_with_depth(context, depth + 1)?;
                let effective_probability = if satisfied { *probability } else { 0.0 };
                Ok(effective_probability >= *threshold)
            }
            Self::Temporal {
                base_value,
                reference_time,
                rate,
                operator,
                target_value,
            } => {
                let current_time = context.get_current_timestamp().ok_or_else(|| {
                    EvaluationError::MissingContext {
                        description: "current timestamp".to_string(),
                    }
                })?;
                let time_elapsed =
                    (current_time - reference_time) as f64 / (365.25 * 24.0 * 3600.0);
                let current_value = base_value * (1.0 + rate).powf(time_elapsed);
                Ok(operator.compare_f64(current_value, *target_value))
            }
            Self::Custom { description } => Err(EvaluationError::Custom {
                message: format!("Cannot evaluate custom condition: {}", description),
            }),
        }
    }
    /// Evaluates the condition with detailed step-by-step explanation.
    ///
    /// This method provides a full trace of the evaluation process, useful for:
    /// - Debugging complex conditions
    /// - Explaining legal decisions to users
    /// - Auditing and compliance
    /// - Educational purposes
    ///
    /// # Example
    /// ```
    /// # use legalis_core::{Condition, ComparisonOp, AttributeBasedContext};
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
    /// assert!(explanation.steps.len() >= 3); // AND, Age, Income checks
    /// assert!(explanation.conclusion);
    /// ```
    pub fn evaluate_with_explanation<C: EvaluationContext>(
        &self,
        context: &C,
    ) -> Result<(bool, EvaluationExplanation), EvaluationError> {
        let mut steps = Vec::new();
        let result = self.evaluate_with_explanation_recursive(context, 0, &mut steps)?;
        let explanation = EvaluationExplanation {
            condition: format!("{}", self),
            conclusion: result,
            steps,
        };
        Ok((result, explanation))
    }
    /// Internal recursive helper for evaluation with explanation.
    fn evaluate_with_explanation_recursive<C: EvaluationContext>(
        &self,
        context: &C,
        depth: usize,
        steps: &mut Vec<ExplanationStep>,
    ) -> Result<bool, EvaluationError> {
        const MAX_DEPTH: usize = 100;
        if depth > MAX_DEPTH {
            return Err(EvaluationError::MaxDepthExceeded {
                max_depth: MAX_DEPTH,
            });
        }
        let start_time = std::time::Instant::now();
        let (result, details) = match self {
            Self::And(left, right) => {
                let left_result =
                    left.evaluate_with_explanation_recursive(context, depth + 1, steps)?;
                if !left_result {
                    (
                        false,
                        "AND operation short-circuited (left operand is false)".to_string(),
                    )
                } else {
                    let right_result =
                        right.evaluate_with_explanation_recursive(context, depth + 1, steps)?;
                    (
                        right_result,
                        format!("AND: left={}, right={}", left_result, right_result),
                    )
                }
            }
            Self::Or(left, right) => {
                let left_result =
                    left.evaluate_with_explanation_recursive(context, depth + 1, steps)?;
                if left_result {
                    (
                        true,
                        "OR operation short-circuited (left operand is true)".to_string(),
                    )
                } else {
                    let right_result =
                        right.evaluate_with_explanation_recursive(context, depth + 1, steps)?;
                    (
                        right_result,
                        format!("OR: left={}, right={}", left_result, right_result),
                    )
                }
            }
            Self::Not(inner) => {
                let inner_result =
                    inner.evaluate_with_explanation_recursive(context, depth + 1, steps)?;
                (
                    !inner_result,
                    format!("NOT: inner={} -> {}", inner_result, !inner_result),
                )
            }
            Self::Age { operator, value } => {
                let age = context
                    .get_age()
                    .ok_or_else(|| EvaluationError::MissingAttribute {
                        key: "age".to_string(),
                    })?;
                let result = operator.compare_u32(age, *value);
                (
                    result,
                    format!("Age check: {} {} {} = {}", age, operator, value, result),
                )
            }
            Self::Income { operator, value } => {
                let income =
                    context
                        .get_income()
                        .ok_or_else(|| EvaluationError::MissingAttribute {
                            key: "income".to_string(),
                        })?;
                let result = operator.compare_u64(income, *value);
                (
                    result,
                    format!(
                        "Income check: {} {} {} = {}",
                        income, operator, value, result
                    ),
                )
            }
            Self::HasAttribute { key } => {
                let has_it = context.get_attribute(key).is_some();
                (has_it, format!("HasAttribute '{}': {}", key, has_it))
            }
            Self::AttributeEquals { key, value } => {
                let actual = context.get_attribute(key);
                let equals = actual.as_ref() == Some(value);
                (
                    equals,
                    format!(
                        "AttributeEquals '{}': expected='{}', actual={:?}, result={}",
                        key, value, actual, equals
                    ),
                )
            }
            _ => {
                let result = self.evaluate_with_depth(context, depth)?;
                (
                    result,
                    format!("Condition '{}' evaluated to {}", self, result),
                )
            }
        };
        let elapsed = start_time.elapsed().as_micros() as u64;
        steps.push(ExplanationStep {
            condition: format!("{}", self),
            result,
            details,
            depth,
            duration_micros: elapsed,
        });
        Ok(result)
    }
    /// Performs partial evaluation, allowing unknown values.
    ///
    /// Unlike `evaluate()`, this method can handle cases where some attributes
    /// or context values are unknown. It returns a three-valued logic result:
    /// - `PartialBool::True` - Definitely true
    /// - `PartialBool::False` - Definitely false
    /// - `PartialBool::Unknown` - Cannot determine (missing data)
    ///
    /// This is useful for:
    /// - Pre-checking eligibility with incomplete data
    /// - Planning data collection (what's missing?)
    /// - Optimistic evaluation strategies
    ///
    /// # Example
    /// ```
    /// # use legalis_core::{Condition, ComparisonOp, PartialBool, AttributeBasedContext};
    /// # use std::collections::HashMap;
    /// let mut attributes = HashMap::new();
    /// attributes.insert("age".to_string(), "25".to_string());
    /// // income is missing
    /// let ctx = AttributeBasedContext::new(attributes);
    ///
    /// let age_check = Condition::age(ComparisonOp::GreaterOrEqual, 18);
    /// let income_check = Condition::income(ComparisonOp::GreaterOrEqual, 30000);
    ///
    /// // Age check has data -> True
    /// assert!(matches!(age_check.partial_evaluate(&ctx), PartialBool::True { .. }));
    ///
    /// // Income check is missing data -> Unknown
    /// assert!(matches!(income_check.partial_evaluate(&ctx), PartialBool::Unknown { .. }));
    ///
    /// // AND with unknown propagates uncertainty
    /// let condition = age_check.and(income_check);
    /// assert!(matches!(condition.partial_evaluate(&ctx), PartialBool::Unknown { .. }));
    /// ```
    pub fn partial_evaluate<C: EvaluationContext>(&self, context: &C) -> PartialBool {
        self.partial_evaluate_with_depth(context, 0)
    }
    /// Internal recursive helper for partial evaluation.
    fn partial_evaluate_with_depth<C: EvaluationContext>(
        &self,
        context: &C,
        depth: usize,
    ) -> PartialBool {
        const MAX_DEPTH: usize = 100;
        if depth > MAX_DEPTH {
            return PartialBool::unknown(0.0, "Maximum depth exceeded");
        }
        match self {
            Self::And(left, right) => {
                let left_result = left.partial_evaluate_with_depth(context, depth + 1);
                let right_result = right.partial_evaluate_with_depth(context, depth + 1);
                match (&left_result, &right_result) {
                    (PartialBool::False { .. }, _) => left_result,
                    (_, PartialBool::False { .. }) => right_result,
                    (
                        PartialBool::True { confidence: c1, .. },
                        PartialBool::True { confidence: c2, .. },
                    ) => PartialBool::true_with_confidence((*c1).min(*c2)),
                    (
                        PartialBool::Unknown {
                            confidence: c1,
                            reason: r1,
                        },
                        PartialBool::Unknown {
                            confidence: c2,
                            reason: r2,
                        },
                    ) => {
                        let combined_confidence = (*c1).min(*c2);
                        PartialBool::unknown(combined_confidence, &format!("{} AND {}", r1, r2))
                    }
                    _ => PartialBool::unknown(0.5, "AND with unknown operand"),
                }
            }
            Self::Or(left, right) => {
                let left_result = left.partial_evaluate_with_depth(context, depth + 1);
                let right_result = right.partial_evaluate_with_depth(context, depth + 1);
                match (&left_result, &right_result) {
                    (PartialBool::True { .. }, _) => left_result,
                    (_, PartialBool::True { .. }) => right_result,
                    (
                        PartialBool::False { confidence: c1, .. },
                        PartialBool::False { confidence: c2, .. },
                    ) => PartialBool::false_with_confidence((*c1).min(*c2)),
                    (
                        PartialBool::Unknown {
                            confidence: c1,
                            reason: r1,
                        },
                        PartialBool::Unknown {
                            confidence: c2,
                            reason: r2,
                        },
                    ) => {
                        let combined_confidence = (*c1).min(*c2);
                        PartialBool::unknown(combined_confidence, &format!("{} OR {}", r1, r2))
                    }
                    _ => PartialBool::unknown(0.5, "OR with unknown operand"),
                }
            }
            Self::Not(inner) => {
                let inner_result = inner.partial_evaluate_with_depth(context, depth + 1);
                match inner_result {
                    PartialBool::True { confidence, reason } => {
                        PartialBool::false_with_confidence_and_reason(
                            confidence,
                            &format!("NOT ({})", reason),
                        )
                    }
                    PartialBool::False { confidence, reason } => {
                        PartialBool::true_with_confidence_and_reason(
                            confidence,
                            &format!("NOT ({})", reason),
                        )
                    }
                    PartialBool::Unknown { confidence, reason } => {
                        PartialBool::unknown(confidence, &format!("NOT ({})", reason))
                    }
                }
            }
            Self::Age { operator, value } => match context.get_age() {
                Some(age) => {
                    let result = operator.compare_u32(age, *value);
                    if result {
                        PartialBool::true_with_confidence(1.0)
                    } else {
                        PartialBool::false_with_confidence(1.0)
                    }
                }
                None => PartialBool::unknown(0.0, "age attribute missing"),
            },
            Self::Income { operator, value } => match context.get_income() {
                Some(income) => {
                    let result = operator.compare_u64(income, *value);
                    if result {
                        PartialBool::true_with_confidence(1.0)
                    } else {
                        PartialBool::false_with_confidence(1.0)
                    }
                }
                None => PartialBool::unknown(0.0, "income attribute missing"),
            },
            Self::HasAttribute { key } => match context.get_attribute(key) {
                Some(_) => PartialBool::true_with_confidence(1.0),
                None => PartialBool::false_with_confidence(1.0),
            },
            Self::AttributeEquals { key, value } => match context.get_attribute(key) {
                Some(actual) => {
                    if &actual == value {
                        PartialBool::true_with_confidence(1.0)
                    } else {
                        PartialBool::false_with_confidence(1.0)
                    }
                }
                None => PartialBool::unknown(0.0, &format!("attribute '{}' missing", key)),
            },
            Self::DateRange { start, end } => match context.get_current_date() {
                Some(current_date) => {
                    let after_start = start.is_none_or(|s| current_date >= s);
                    let before_end = end.is_none_or(|e| current_date <= e);
                    let result = after_start && before_end;
                    if result {
                        PartialBool::true_with_confidence(1.0)
                    } else {
                        PartialBool::false_with_confidence(1.0)
                    }
                }
                None => PartialBool::unknown(0.0, "current date missing"),
            },
            _ => match self.evaluate(context) {
                Ok(result) => {
                    if result {
                        PartialBool::true_with_confidence(1.0)
                    } else {
                        PartialBool::false_with_confidence(1.0)
                    }
                }
                Err(_) => PartialBool::unknown(0.0, "evaluation failed or data missing"),
            },
        }
    }
}
impl Condition {
    /// Evaluates this condition with parallel processing for independent conditions.
    ///
    /// When the `parallel` feature is enabled, this method will evaluate independent
    /// And/Or branches in parallel for better performance on multi-core systems.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use legalis_core::{Condition, ComparisonOp, EvaluationContext, RegionType, RelationshipType, DurationUnit};
    /// use chrono::NaiveDate;
    ///
    /// struct MyContext { age: u32, income: u64 }
    ///
    /// impl EvaluationContext for MyContext {
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
    /// let ctx = MyContext { age: 25, income: 45000 };
    ///
    /// // Complex condition with multiple independent checks
    /// let condition = Condition::age(ComparisonOp::GreaterOrEqual, 18)
    ///     .and(Condition::income(ComparisonOp::LessThan, 50000));
    ///
    /// // Evaluates branches in parallel when possible
    /// let result = condition.evaluate_parallel(&ctx);
    /// ```
    #[cfg(feature = "parallel")]
    pub fn evaluate_parallel<C: EvaluationContext + Sync>(
        &self,
        context: &C,
    ) -> Result<bool, EvaluationError> {
        self.evaluate_parallel_with_depth(context, 0, 100)
    }
    #[cfg(feature = "parallel")]
    #[allow(clippy::too_many_lines)]
    fn evaluate_parallel_with_depth<C: EvaluationContext + Sync>(
        &self,
        context: &C,
        depth: usize,
        max_depth: usize,
    ) -> Result<bool, EvaluationError> {
        if depth > max_depth {
            return Err(EvaluationError::MaxDepthExceeded { max_depth });
        }
        match self {
            Self::Age { .. }
            | Self::Income { .. }
            | Self::HasAttribute { .. }
            | Self::AttributeEquals { .. }
            | Self::DateRange { .. }
            | Self::Geographic { .. }
            | Self::EntityRelationship { .. }
            | Self::ResidencyDuration { .. }
            | Self::Duration { .. }
            | Self::Percentage { .. }
            | Self::SetMembership { .. }
            | Self::Pattern { .. }
            | Self::Calculation { .. }
            | Self::Threshold { .. }
            | Self::Fuzzy { .. }
            | Self::Temporal { .. }
            | Self::Custom { .. } => self.evaluate(context),
            Self::Composite {
                conditions,
                threshold,
            } => {
                let results: Vec<_> = conditions
                    .par_iter()
                    .map(|(weight, cond)| {
                        cond.evaluate_parallel_with_depth(context, depth + 1, max_depth)
                            .map(|satisfied| if satisfied { *weight } else { 0.0 })
                    })
                    .collect();
                for result in &results {
                    if let Err(e) = result {
                        return Err(e.clone());
                    }
                }
                let total_score: f64 = results.iter().filter_map(|r| r.as_ref().ok()).sum();
                Ok(total_score >= *threshold)
            }
            Self::Probabilistic {
                condition,
                probability,
                threshold,
            } => {
                let satisfied =
                    condition.evaluate_parallel_with_depth(context, depth + 1, max_depth)?;
                let effective_probability = if satisfied { *probability } else { 0.0 };
                Ok(effective_probability >= *threshold)
            }
            Self::And(left, right) => {
                let (left_result, right_result) = rayon::join(
                    || left.evaluate_parallel_with_depth(context, depth + 1, max_depth),
                    || right.evaluate_parallel_with_depth(context, depth + 1, max_depth),
                );
                match (left_result, right_result) {
                    (Ok(true), Ok(true)) => Ok(true),
                    (Ok(false), _) | (_, Ok(false)) => Ok(false),
                    (Err(e), _) | (_, Err(e)) => Err(e),
                }
            }
            Self::Or(left, right) => {
                let (left_result, right_result) = rayon::join(
                    || left.evaluate_parallel_with_depth(context, depth + 1, max_depth),
                    || right.evaluate_parallel_with_depth(context, depth + 1, max_depth),
                );
                match (left_result, right_result) {
                    (Ok(true), _) | (_, Ok(true)) => Ok(true),
                    (Ok(false), Ok(false)) => Ok(false),
                    (Err(e), _) | (_, Err(e)) => Err(e),
                }
            }
            Self::Not(inner) => {
                let result = inner.evaluate_parallel_with_depth(context, depth + 1, max_depth)?;
                Ok(!result)
            }
        }
    }
    /// Evaluates a collection of conditions in parallel.
    ///
    /// This is useful when you have multiple independent conditions to evaluate
    /// and want to leverage parallel processing.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use legalis_core::{Condition, ComparisonOp, EvaluationContext, RegionType, RelationshipType, DurationUnit};
    /// use chrono::NaiveDate;
    ///
    /// struct MyContext { age: u32, income: u64 }
    ///
    /// impl EvaluationContext for MyContext {
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
    /// let ctx = MyContext { age: 25, income: 45000 };
    /// let conditions = vec![
    ///     Condition::age(ComparisonOp::GreaterOrEqual, 18),
    ///     Condition::income(ComparisonOp::LessThan, 50000),
    /// ];
    ///
    /// let results = Condition::evaluate_all_parallel(&conditions, &ctx);
    /// assert_eq!(results.len(), 2);
    /// ```
    #[cfg(feature = "parallel")]
    pub fn evaluate_all_parallel<C: EvaluationContext + Sync>(
        conditions: &[Condition],
        context: &C,
    ) -> Vec<Result<bool, EvaluationError>> {
        conditions
            .par_iter()
            .map(|cond| cond.evaluate_parallel(context))
            .collect()
    }
}
/// Composed effect combining multiple effects with priority ordering.
///
/// When multiple effects need to be applied together, a ComposedEffect
/// provides conflict resolution strategies and ordering guarantees.
///
/// # Example
/// ```
/// # use legalis_core::{Effect, ComposedEffect, CompositionStrategy};
/// let effects = vec![
///     Effect::grant("resource access"),
///     Effect::obligation("annual reporting"),
/// ];
/// let composed = ComposedEffect::new(effects)
///     .with_resolution_strategy(CompositionStrategy::MostSpecific);
///
/// assert_eq!(composed.effects.len(), 2);
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct ComposedEffect {
    /// The effects to be composed (applied in order).
    pub effects: Vec<Effect>,
    /// Strategy for resolving conflicts between effects.
    pub resolution_strategy: CompositionStrategy,
}
impl ComposedEffect {
    /// Creates a new composed effect with default conflict resolution (FirstWins).
    #[must_use]
    pub fn new(effects: Vec<Effect>) -> Self {
        Self {
            effects,
            resolution_strategy: CompositionStrategy::FirstWins,
        }
    }
    /// Sets the conflict resolution strategy.
    #[must_use]
    pub fn with_resolution_strategy(mut self, strategy: CompositionStrategy) -> Self {
        self.resolution_strategy = strategy;
        self
    }
    /// Adds an effect to the composition.
    pub fn add_effect(&mut self, effect: Effect) {
        self.effects.push(effect);
    }
    /// Returns the number of effects.
    #[must_use]
    pub fn len(&self) -> usize {
        self.effects.len()
    }
    /// Checks if there are no effects.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.effects.is_empty()
    }
    /// Resolves the composition to a single effective result.
    ///
    /// Applies the conflict resolution strategy to determine which effects take precedence.
    #[must_use]
    pub fn resolve(&self) -> Vec<&Effect> {
        match self.resolution_strategy {
            CompositionStrategy::FirstWins => {
                let mut seen_types = std::collections::HashSet::new();
                self.effects
                    .iter()
                    .filter(|e| seen_types.insert(e.effect_type.clone()))
                    .collect()
            }
            CompositionStrategy::LastWins => {
                let mut result = std::collections::HashMap::new();
                for effect in &self.effects {
                    result.insert(effect.effect_type.clone(), effect);
                }
                result.values().copied().collect()
            }
            CompositionStrategy::MostSpecific => {
                let mut result = std::collections::HashMap::new();
                for effect in &self.effects {
                    result
                        .entry(effect.effect_type.clone())
                        .and_modify(|e: &mut &Effect| {
                            if effect.parameter_count() > e.parameter_count() {
                                *e = effect;
                            }
                        })
                        .or_insert(effect);
                }
                result.values().copied().collect()
            }
            CompositionStrategy::AllApply => self.effects.iter().collect(),
        }
    }
}
/// Forward chaining entailment with multi-step inference.
///
/// This engine can perform multi-step legal reasoning, where the effects
/// of one statute can enable the conditions of another statute.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, Condition, ComparisonOp};
/// use legalis_core::{ForwardChainingEngine, AttributeBasedContext};
/// use std::collections::HashMap;
///
/// // Step 1: Being 18+ grants eligibility
/// let eligibility = Statute::new("eligibility", "Eligibility", Effect::grant("eligible"))
///     .with_precondition(Condition::age(ComparisonOp::GreaterOrEqual, 18));
///
/// // Step 2: Having eligibility grants voting rights
/// let voting = Statute::new("voting", "Voting", Effect::grant("vote"))
///     .with_precondition(Condition::has_attribute("eligible"));
///
/// let mut attributes = HashMap::new();
/// attributes.insert("age".to_string(), "25".to_string());
/// let context = AttributeBasedContext::new(attributes);
///
/// let statutes = vec![eligibility, voting];
/// let engine = ForwardChainingEngine::new(statutes);
/// let chain = engine.infer(&context, 5);
///
/// // Should derive both eligibility and voting rights
/// assert!(!chain.is_empty());
/// ```
#[derive(Debug, Clone)]
pub struct ForwardChainingEngine {
    statutes: Vec<Statute>,
}
impl ForwardChainingEngine {
    /// Creates a new forward chaining engine.
    #[must_use]
    pub fn new(statutes: Vec<Statute>) -> Self {
        Self { statutes }
    }
    /// Performs forward chaining inference up to max_steps.
    ///
    /// Returns the chain of inferences that can be derived.
    pub fn infer(&self, context: &AttributeBasedContext, max_steps: usize) -> Vec<InferenceStep> {
        let mut inferences = Vec::new();
        let mut changed = true;
        let mut steps = 0;
        while changed && steps < max_steps {
            changed = false;
            steps += 1;
            for statute in &self.statutes {
                if inferences
                    .iter()
                    .any(|inf: &InferenceStep| inf.statute_id == statute.id)
                {
                    continue;
                }
                if self.can_apply_statute(statute, context) {
                    let depends_on = self.find_dependencies(&inferences, statute);
                    inferences.push(InferenceStep {
                        statute_id: statute.id.clone(),
                        effect: statute.effect.clone(),
                        depends_on,
                    });
                    changed = true;
                }
            }
        }
        inferences
    }
    /// Checks if a statute's conditions can be applied given the current context.
    fn can_apply_statute(&self, statute: &Statute, context: &AttributeBasedContext) -> bool {
        if statute.preconditions.is_empty() {
            return true;
        }
        statute
            .preconditions
            .iter()
            .all(|cond| cond.evaluate_simple(context).unwrap_or(false))
    }
    /// Finds which previous inferences this statute depends on.
    fn find_dependencies(&self, _inferences: &[InferenceStep], _statute: &Statute) -> Vec<usize> {
        Vec::new()
    }
}
/// Represents differences between two versions of a statute.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct StatuteDiff {
    /// ID of the statute being compared
    pub statute_id: String,
    /// List of changes detected
    pub changes: Vec<StatuteChange>,
}
impl StatuteDiff {
    /// Returns true if there are no changes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.changes.is_empty()
    }
    /// Returns the number of changes.
    #[must_use]
    pub fn change_count(&self) -> usize {
        self.changes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_evaluate_formula_types5_basic() {
        // formula "x + 1" with x=5 should return 6.0
        let mut attrs = HashMap::new();
        attrs.insert("x".to_string(), "5".to_string());
        let ctx = AttributeBasedContext::new(attrs);
        let result = Condition::evaluate_formula("x + 1", &ctx);
        assert_eq!(result, Ok(6.0));
    }
}
