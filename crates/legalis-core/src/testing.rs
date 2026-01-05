//! Testing utilities for legalis-core.
//!
//! This module provides utilities for testing legal frameworks:
//! - `StatuteTestBuilder` - Generate test fixtures with realistic data
//! - `ConditionGenerator` - Random condition generation for property tests
//! - `EvaluationContextMock` - Mock context for testing condition evaluation
//! - `SnapshotTester` - Snapshot testing for statute serialization
//! - `MutationTester` - Mutation testing for condition logic
//!
//! # Examples
//!
//! ```
//! use legalis_core::testing::StatuteTestBuilder;
//!
//! let statute = StatuteTestBuilder::new()
//!     .with_random_id()
//!     .with_random_title()
//!     .with_grant_effect()
//!     .with_age_condition(18)
//!     .build();
//!
//! assert!(!statute.id.is_empty());
//! assert!(!statute.title.is_empty());
//! ```

use crate::{
    ComparisonOp, Condition, Effect, EvaluationContext, NaiveDate, RegionType, RelationshipType,
    Statute, TemporalValidity,
};
use std::collections::HashMap;

/// Test fixture builder for statutes.
///
/// Generates statutes with realistic test data for unit and integration tests.
///
/// # Examples
///
/// ```
/// use legalis_core::testing::StatuteTestBuilder;
///
/// let statute = StatuteTestBuilder::new()
///     .with_id("test-1")
///     .with_title("Test Statute")
///     .with_grant_effect()
///     .with_age_condition(21)
///     .with_jurisdiction("US")
///     .build();
///
/// assert_eq!(statute.id, "test-1");
/// assert_eq!(statute.preconditions.len(), 1);
/// ```
pub struct StatuteTestBuilder {
    id: Option<String>,
    title: Option<String>,
    effect: Option<Effect>,
    preconditions: Vec<Condition>,
    discretion_logic: Option<String>,
    temporal_validity: Option<TemporalValidity>,
    version: u32,
    jurisdiction: Option<String>,
    counter: usize,
}

impl StatuteTestBuilder {
    /// Create a new test builder.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::testing::StatuteTestBuilder;
    ///
    /// let builder = StatuteTestBuilder::new();
    /// let statute = builder
    ///     .with_id("test")
    ///     .with_title("Test")
    ///     .with_grant_effect()
    ///     .build();
    /// ```
    pub fn new() -> Self {
        Self {
            id: None,
            title: None,
            effect: None,
            preconditions: Vec::new(),
            discretion_logic: None,
            temporal_validity: None,
            version: 1,
            jurisdiction: None,
            counter: 0,
        }
    }

    /// Set the statute ID.
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Generate a random statute ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::testing::StatuteTestBuilder;
    ///
    /// let statute = StatuteTestBuilder::new()
    ///     .with_random_id()
    ///     .with_title("Test")
    ///     .with_grant_effect()
    ///     .build();
    ///
    /// assert!(!statute.id.is_empty());
    /// ```
    pub fn with_random_id(mut self) -> Self {
        self.counter += 1;
        self.id = Some(format!("test-statute-{}", self.counter));
        self
    }

    /// Set the statute title.
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Generate a random statute title.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::testing::StatuteTestBuilder;
    ///
    /// let statute = StatuteTestBuilder::new()
    ///     .with_id("test")
    ///     .with_random_title()
    ///     .with_grant_effect()
    ///     .build();
    ///
    /// assert!(!statute.title.is_empty());
    /// ```
    pub fn with_random_title(mut self) -> Self {
        self.counter += 1;
        self.title = Some(format!("Test Statute #{}", self.counter));
        self
    }

    /// Add a grant effect.
    pub fn with_grant_effect(mut self) -> Self {
        self.effect = Some(Effect::grant("Test grant"));
        self
    }

    /// Add a revoke effect.
    pub fn with_revoke_effect(mut self) -> Self {
        self.effect = Some(Effect::revoke("Test revoke"));
        self
    }

    /// Add an obligation effect.
    pub fn with_obligation_effect(mut self) -> Self {
        self.effect = Some(Effect::obligation("Test obligation"));
        self
    }

    /// Add a custom effect.
    pub fn with_effect(mut self, effect: Effect) -> Self {
        self.effect = Some(effect);
        self
    }

    /// Add an age condition.
    pub fn with_age_condition(mut self, age: u32) -> Self {
        self.preconditions
            .push(Condition::age(ComparisonOp::GreaterOrEqual, age));
        self
    }

    /// Add an income condition.
    pub fn with_income_condition(mut self, income: u64) -> Self {
        self.preconditions
            .push(Condition::income(ComparisonOp::LessThan, income));
        self
    }

    /// Add a custom condition.
    pub fn with_condition(mut self, condition: Condition) -> Self {
        self.preconditions.push(condition);
        self
    }

    /// Add discretion logic.
    pub fn with_discretion(mut self, logic: impl Into<String>) -> Self {
        self.discretion_logic = Some(logic.into());
        self
    }

    /// Set temporal validity.
    pub fn with_temporal_validity(mut self, validity: TemporalValidity) -> Self {
        self.temporal_validity = Some(validity);
        self
    }

    /// Set jurisdiction.
    pub fn with_jurisdiction(mut self, jurisdiction: impl Into<String>) -> Self {
        self.jurisdiction = Some(jurisdiction.into());
        self
    }

    /// Set version.
    pub fn with_version(mut self, version: u32) -> Self {
        self.version = version;
        self
    }

    /// Build the statute.
    ///
    /// # Panics
    ///
    /// Panics if ID, title, or effect are not set.
    pub fn build(self) -> Statute {
        let id = self.id.expect("ID must be set");
        let title = self.title.expect("Title must be set");
        let effect = self.effect.expect("Effect must be set");

        let mut statute = Statute::new(id, title, effect);

        for condition in self.preconditions {
            statute = statute.with_precondition(condition);
        }

        if let Some(discretion) = self.discretion_logic {
            statute = statute.with_discretion(discretion);
        }

        if let Some(temporal) = self.temporal_validity {
            statute = statute.with_temporal_validity(temporal);
        }

        if let Some(jurisdiction) = self.jurisdiction {
            statute = statute.with_jurisdiction(jurisdiction);
        }

        statute = statute.with_version(self.version);

        statute
    }
}

impl Default for StatuteTestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Random condition generator for property-based tests.
///
/// Generates random conditions with configurable complexity.
///
/// # Examples
///
/// ```
/// use legalis_core::testing::ConditionGenerator;
///
/// let generator = ConditionGenerator::new().with_max_depth(3);
/// let condition = generator.generate_simple();
///
/// // Condition is guaranteed to be non-compound
/// assert!(!condition.to_string().contains("AND"));
/// assert!(!condition.to_string().contains("OR"));
/// ```
pub struct ConditionGenerator {
    max_depth: usize,
    counter: usize,
}

impl ConditionGenerator {
    /// Create a new condition generator.
    pub fn new() -> Self {
        Self {
            max_depth: 5,
            counter: 0,
        }
    }

    /// Set maximum nesting depth for compound conditions.
    pub fn with_max_depth(mut self, depth: usize) -> Self {
        self.max_depth = depth;
        self
    }

    /// Generate a simple (non-compound) condition.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::testing::ConditionGenerator;
    ///
    /// let generator = ConditionGenerator::new();
    /// let condition = generator.generate_simple();
    ///
    /// // Will be one of: Age, Income, HasAttribute, etc.
    /// assert!(condition.is_simple());
    /// ```
    pub fn generate_simple(&self) -> Condition {
        let variants = [
            Condition::age(ComparisonOp::GreaterOrEqual, 18),
            Condition::income(ComparisonOp::LessThan, 50000),
            Condition::has_attribute("citizen"),
            Condition::attribute_equals("status", "active"),
        ];

        // Use counter as seed for deterministic selection
        variants[self.counter % variants.len()].clone()
    }

    /// Generate a compound condition with controlled nesting.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::testing::ConditionGenerator;
    ///
    /// let mut generator = ConditionGenerator::new().with_max_depth(2);
    /// let condition = generator.generate_compound(1);
    ///
    /// // May contain AND, OR, NOT operators
    /// assert!(condition.depth() <= 2);
    /// ```
    pub fn generate_compound(&mut self, depth: usize) -> Condition {
        if depth >= self.max_depth {
            return self.generate_simple();
        }

        self.counter += 1;
        let variant = self.counter % 3;

        match variant {
            0 => {
                // AND condition
                let left = self.generate_compound(depth + 1);
                let right = self.generate_compound(depth + 1);
                Condition::And(Box::new(left), Box::new(right))
            }
            1 => {
                // OR condition
                let left = self.generate_compound(depth + 1);
                let right = self.generate_compound(depth + 1);
                Condition::Or(Box::new(left), Box::new(right))
            }
            _ => {
                // NOT condition
                let inner = self.generate_compound(depth + 1);
                Condition::Not(Box::new(inner))
            }
        }
    }

    /// Generate an age condition with random value.
    pub fn age_condition(&mut self, min_age: u32, max_age: u32) -> Condition {
        self.counter += 1;
        let age = min_age + (self.counter as u32 % (max_age - min_age + 1));
        Condition::age(ComparisonOp::GreaterOrEqual, age)
    }

    /// Generate an income condition with random value.
    pub fn income_condition(&mut self, min_income: u64, max_income: u64) -> Condition {
        self.counter += 1;
        let income = min_income + (self.counter as u64 % (max_income - min_income + 1));
        Condition::income(ComparisonOp::LessThan, income)
    }
}

impl Default for ConditionGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Mock evaluation context for testing.
///
/// Provides a simple in-memory context for testing condition evaluation.
///
/// # Examples
///
/// ```
/// use legalis_core::testing::EvaluationContextMock;
/// use legalis_core::{Condition, ComparisonOp, EvaluationContext};
///
/// let mut mock = EvaluationContextMock::new()
///     .with_age(25)
///     .with_income(40000)
///     .with_attribute("citizen", "true");
///
/// let condition = Condition::age(ComparisonOp::GreaterOrEqual, 18);
/// assert_eq!(condition.evaluate(&mock).unwrap(), true);
/// ```
pub struct EvaluationContextMock {
    age: Option<u32>,
    income: Option<u64>,
    attributes: HashMap<String, String>,
    date: Option<NaiveDate>,
}

impl EvaluationContextMock {
    /// Create a new mock context.
    pub fn new() -> Self {
        Self {
            age: None,
            income: None,
            attributes: HashMap::new(),
            date: None,
        }
    }

    /// Set age for testing.
    pub fn with_age(mut self, age: u32) -> Self {
        self.age = Some(age);
        self
    }

    /// Set income for testing.
    pub fn with_income(mut self, income: u64) -> Self {
        self.income = Some(income);
        self
    }

    /// Add an attribute for testing.
    pub fn with_attribute(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }

    /// Set date for temporal conditions.
    pub fn with_date(mut self, date: NaiveDate) -> Self {
        self.date = Some(date);
        self
    }
}

impl Default for EvaluationContextMock {
    fn default() -> Self {
        Self::new()
    }
}

impl EvaluationContext for EvaluationContextMock {
    fn get_attribute(&self, key: &str) -> Option<String> {
        self.attributes.get(key).cloned()
    }

    fn get_age(&self) -> Option<u32> {
        self.age
    }

    fn get_income(&self) -> Option<u64> {
        self.income
    }

    fn get_current_date(&self) -> Option<NaiveDate> {
        self.date
    }

    fn get_current_timestamp(&self) -> Option<i64> {
        Some(chrono::Utc::now().timestamp())
    }

    fn check_geographic(&self, _region_type: RegionType, _region_id: &str) -> bool {
        // Default implementation for testing
        true
    }

    fn check_relationship(
        &self,
        _relationship_type: RelationshipType,
        _target_id: Option<&str>,
    ) -> bool {
        // Default implementation for testing
        false
    }

    fn get_residency_months(&self) -> Option<u32> {
        // Default implementation for testing
        None
    }

    fn get_duration(&self, _unit: crate::DurationUnit) -> Option<u32> {
        // Default implementation for testing
        None
    }

    fn get_percentage(&self, _context: &str) -> Option<u32> {
        // Default implementation for testing
        None
    }

    fn evaluate_formula(&self, _formula: &str) -> Option<f64> {
        // Default implementation for testing
        None
    }
}

/// Snapshot testing utilities for statute serialization.
///
/// Helps verify that statute serialization remains stable across changes.
///
/// # Examples
///
/// ```
/// use legalis_core::testing::{SnapshotTester, StatuteTestBuilder};
///
/// let statute = StatuteTestBuilder::new()
///     .with_id("snapshot-test")
///     .with_title("Snapshot Test")
///     .with_grant_effect()
///     .build();
///
/// let tester = SnapshotTester::new();
/// let snapshot = tester.create_snapshot(&statute);
///
/// assert!(snapshot.contains("snapshot-test"));
/// assert!(snapshot.contains("Snapshot Test"));
/// ```
pub struct SnapshotTester {
    format: SnapshotFormat,
}

/// Snapshot format for serialization testing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapshotFormat {
    /// JSON format
    Json,
    /// YAML format (requires yaml feature)
    #[cfg(feature = "yaml")]
    Yaml,
    /// TOML format (requires toml feature)
    #[cfg(feature = "toml")]
    Toml,
}

impl SnapshotTester {
    /// Create a new snapshot tester.
    pub fn new() -> Self {
        Self {
            format: SnapshotFormat::Json,
        }
    }

    /// Set snapshot format.
    pub fn with_format(mut self, format: SnapshotFormat) -> Self {
        self.format = format;
        self
    }

    /// Create a snapshot of a statute.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::testing::{SnapshotTester, StatuteTestBuilder};
    ///
    /// let statute = StatuteTestBuilder::new()
    ///     .with_id("test")
    ///     .with_title("Test")
    ///     .with_grant_effect()
    ///     .build();
    ///
    /// let tester = SnapshotTester::new();
    /// let snapshot = tester.create_snapshot(&statute);
    ///
    /// assert!(!snapshot.is_empty());
    /// ```
    #[cfg(feature = "serde")]
    pub fn create_snapshot(&self, statute: &Statute) -> String {
        match self.format {
            SnapshotFormat::Json => {
                serde_json::to_string_pretty(statute).unwrap_or_else(|_| "{}".to_string())
            }
            #[cfg(feature = "yaml")]
            SnapshotFormat::Yaml => {
                serde_yaml::to_string(statute).unwrap_or_else(|_| "".to_string())
            }
            #[cfg(feature = "toml")]
            SnapshotFormat::Toml => toml::to_string(statute).unwrap_or_else(|_| "".to_string()),
        }
    }

    /// Compare a statute against a snapshot.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::testing::{SnapshotTester, StatuteTestBuilder};
    ///
    /// let statute = StatuteTestBuilder::new()
    ///     .with_id("test")
    ///     .with_title("Test")
    ///     .with_grant_effect()
    ///     .build();
    ///
    /// let tester = SnapshotTester::new();
    /// let snapshot = tester.create_snapshot(&statute);
    ///
    /// assert!(tester.matches_snapshot(&statute, &snapshot));
    /// ```
    #[cfg(feature = "serde")]
    pub fn matches_snapshot(&self, statute: &Statute, snapshot: &str) -> bool {
        let current = self.create_snapshot(statute);
        current == snapshot
    }
}

impl Default for SnapshotTester {
    fn default() -> Self {
        Self::new()
    }
}

/// Mutation testing support for condition logic.
///
/// Generates mutated versions of conditions to test condition evaluation robustness.
///
/// # Examples
///
/// ```
/// use legalis_core::testing::MutationTester;
/// use legalis_core::{Condition, ComparisonOp};
///
/// let original = Condition::age(ComparisonOp::GreaterOrEqual, 18);
/// let tester = MutationTester::new();
/// let mutations = tester.generate_mutations(&original);
///
/// // Should generate mutations like:
/// // - Age >= 17 (boundary mutation)
/// // - Age > 18 (operator mutation)
/// // - Age >= 19 (boundary mutation)
/// assert!(!mutations.is_empty());
/// ```
pub struct MutationTester {
    enable_boundary_mutations: bool,
    enable_operator_mutations: bool,
    enable_negation_mutations: bool,
}

impl MutationTester {
    /// Create a new mutation tester.
    pub fn new() -> Self {
        Self {
            enable_boundary_mutations: true,
            enable_operator_mutations: true,
            enable_negation_mutations: true,
        }
    }

    /// Enable or disable boundary mutations (e.g., 18 -> 17, 18 -> 19).
    pub fn with_boundary_mutations(mut self, enable: bool) -> Self {
        self.enable_boundary_mutations = enable;
        self
    }

    /// Enable or disable operator mutations (e.g., >= -> >).
    pub fn with_operator_mutations(mut self, enable: bool) -> Self {
        self.enable_operator_mutations = enable;
        self
    }

    /// Enable or disable negation mutations (e.g., A -> NOT A).
    pub fn with_negation_mutations(mut self, enable: bool) -> Self {
        self.enable_negation_mutations = enable;
        self
    }

    /// Generate mutations of a condition.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::testing::MutationTester;
    /// use legalis_core::{Condition, ComparisonOp};
    ///
    /// let condition = Condition::age(ComparisonOp::GreaterOrEqual, 18);
    /// let tester = MutationTester::new();
    /// let mutations = tester.generate_mutations(&condition);
    ///
    /// assert!(!mutations.is_empty());
    /// ```
    pub fn generate_mutations(&self, condition: &Condition) -> Vec<Condition> {
        let mut mutations = Vec::new();

        match condition {
            Condition::Age { operator, value } => {
                // Boundary mutations
                if self.enable_boundary_mutations {
                    if *value > 0 {
                        mutations.push(Condition::age(*operator, value - 1));
                    }
                    mutations.push(Condition::age(*operator, value + 1));
                }

                // Operator mutations
                if self.enable_operator_mutations {
                    let mutated_ops = self.mutate_operator(*operator);
                    for op in mutated_ops {
                        mutations.push(Condition::age(op, *value));
                    }
                }
            }
            Condition::Income { operator, value } => {
                // Boundary mutations
                if self.enable_boundary_mutations {
                    if *value > 0 {
                        mutations.push(Condition::income(*operator, value - 1));
                    }
                    mutations.push(Condition::income(*operator, value + 1));
                }

                // Operator mutations
                if self.enable_operator_mutations {
                    let mutated_ops = self.mutate_operator(*operator);
                    for op in mutated_ops {
                        mutations.push(Condition::income(op, *value));
                    }
                }
            }
            _ => {
                // For other conditions, just try negation
                if self.enable_negation_mutations {
                    mutations.push(Condition::Not(Box::new(condition.clone())));
                }
            }
        }

        mutations
    }

    fn mutate_operator(&self, op: ComparisonOp) -> Vec<ComparisonOp> {
        match op {
            ComparisonOp::Equal => vec![ComparisonOp::NotEqual],
            ComparisonOp::NotEqual => vec![ComparisonOp::Equal],
            ComparisonOp::LessThan => vec![ComparisonOp::LessOrEqual, ComparisonOp::Equal],
            ComparisonOp::LessOrEqual => {
                vec![ComparisonOp::LessThan, ComparisonOp::GreaterOrEqual]
            }
            ComparisonOp::GreaterThan => {
                vec![ComparisonOp::GreaterOrEqual, ComparisonOp::Equal]
            }
            ComparisonOp::GreaterOrEqual => {
                vec![ComparisonOp::GreaterThan, ComparisonOp::LessOrEqual]
            }
        }
    }
}

impl Default for MutationTester {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_statute_test_builder() {
        let statute = StatuteTestBuilder::new()
            .with_id("test-1")
            .with_title("Test Statute")
            .with_grant_effect()
            .with_age_condition(18)
            .with_jurisdiction("US")
            .build();

        assert_eq!(statute.id, "test-1");
        assert_eq!(statute.title, "Test Statute");
        assert_eq!(statute.preconditions.len(), 1);
        assert_eq!(statute.jurisdiction, Some("US".to_string()));
    }

    #[test]
    fn test_statute_test_builder_random() {
        let statute = StatuteTestBuilder::new()
            .with_random_id()
            .with_random_title()
            .with_grant_effect()
            .build();

        assert!(!statute.id.is_empty());
        assert!(!statute.title.is_empty());
    }

    #[test]
    fn test_condition_generator_simple() {
        let generator = ConditionGenerator::new();
        let condition = generator.generate_simple();

        assert!(condition.is_simple());
    }

    #[test]
    fn test_condition_generator_compound() {
        let mut generator = ConditionGenerator::new().with_max_depth(3);
        let condition = generator.generate_compound(0);

        // Depth might be slightly higher due to how conditions are nested
        assert!(condition.depth() <= 5);
    }

    #[test]
    fn test_evaluation_context_mock() {
        let mock = EvaluationContextMock::new()
            .with_age(25)
            .with_income(40000)
            .with_attribute("citizen", "true");

        assert_eq!(mock.get_age().unwrap(), 25);
        assert_eq!(mock.get_income().unwrap(), 40000);
        assert_eq!(mock.get_attribute("citizen"), Some("true".to_string()));
    }

    #[test]
    fn test_evaluation_context_mock_condition() {
        let mock = EvaluationContextMock::new().with_age(25);
        let condition = Condition::age(ComparisonOp::GreaterOrEqual, 18);

        assert!(condition.evaluate(&mock).unwrap());
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_snapshot_tester() {
        let statute = StatuteTestBuilder::new()
            .with_id("snapshot-test")
            .with_title("Snapshot Test")
            .with_grant_effect()
            .build();

        let tester = SnapshotTester::new();
        let snapshot = tester.create_snapshot(&statute);

        assert!(snapshot.contains("snapshot-test"));
        assert!(snapshot.contains("Snapshot Test"));
        assert!(tester.matches_snapshot(&statute, &snapshot));
    }

    #[test]
    fn test_mutation_tester() {
        let condition = Condition::age(ComparisonOp::GreaterOrEqual, 18);
        let tester = MutationTester::new();
        let mutations = tester.generate_mutations(&condition);

        assert!(!mutations.is_empty());
        // Should have boundary mutations (17, 19) and operator mutations
        assert!(mutations.len() >= 2);
    }

    #[test]
    fn test_mutation_tester_boundary_only() {
        let condition = Condition::income(ComparisonOp::LessThan, 50000);
        let tester = MutationTester::new()
            .with_boundary_mutations(true)
            .with_operator_mutations(false)
            .with_negation_mutations(false);

        let mutations = tester.generate_mutations(&condition);

        // Should only have boundary mutations (49999, 50001)
        assert_eq!(mutations.len(), 2);
    }
}
