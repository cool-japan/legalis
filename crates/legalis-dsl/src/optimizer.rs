//! AST optimization passes for Legalis DSL (v0.1.8).
//!
//! This module provides various optimization techniques to improve
//! statute performance and clarity:
//! - Condition hoisting (move invariant conditions up)
//! - Common subexpression elimination
//! - Dead condition elimination
//! - Condition reordering for short-circuit optimization
//! - Constant folding for static expressions

use crate::ast::{ConditionNode, ConditionValue, LegalDocument, StatuteNode};
use std::collections::HashMap;

/// Optimizer for legal documents and statutes
#[derive(Debug, Clone)]
pub struct Optimizer {
    /// Statistics about optimizations performed
    stats: OptimizationStats,
}

/// Statistics about performed optimizations
#[derive(Debug, Clone, Default)]
pub struct OptimizationStats {
    /// Number of conditions hoisted
    pub hoisted_conditions: usize,
    /// Number of common subexpressions eliminated
    pub eliminated_subexpressions: usize,
    /// Number of dead conditions removed
    pub removed_dead_conditions: usize,
    /// Number of conditions reordered
    pub reordered_conditions: usize,
    /// Number of constants folded
    pub folded_constants: usize,
}

impl Optimizer {
    /// Creates a new optimizer
    pub fn new() -> Self {
        Self {
            stats: OptimizationStats::default(),
        }
    }

    /// Optimizes a legal document with all optimization passes
    pub fn optimize(&mut self, document: LegalDocument) -> LegalDocument {
        let mut doc = document;

        // Run optimization passes in order
        doc = self.optimize_document_hoisting(doc);
        doc = self.optimize_document_cse(doc);
        doc = self.optimize_document_dead_code(doc);
        doc = self.optimize_document_reordering(doc);
        doc = self.optimize_document_constant_folding(doc);

        doc
    }

    /// Returns optimization statistics
    pub fn stats(&self) -> &OptimizationStats {
        &self.stats
    }

    /// Resets optimization statistics
    pub fn reset_stats(&mut self) {
        self.stats = OptimizationStats::default();
    }

    // ==================== CONDITION HOISTING ====================

    /// Hoists invariant conditions up in the AST (v0.1.8.1)
    fn optimize_document_hoisting(&mut self, mut document: LegalDocument) -> LegalDocument {
        for statute in &mut document.statutes {
            self.hoist_conditions(statute);
        }
        document
    }

    /// Hoists conditions within a statute
    fn hoist_conditions(&mut self, statute: &mut StatuteNode) {
        let mut hoisted = Vec::new();
        let mut remaining = Vec::new();

        for condition in &statute.conditions {
            if self.is_invariant(condition) {
                hoisted.push(condition.clone());
                self.stats.hoisted_conditions += 1;
            } else {
                remaining.push(condition.clone());
            }
        }

        // Hoisted conditions come first
        statute.conditions = hoisted;
        statute.conditions.extend(remaining);
    }

    /// Checks if a condition is invariant (doesn't depend on runtime state)
    fn is_invariant(&self, condition: &ConditionNode) -> bool {
        match condition {
            ConditionNode::Comparison { field, .. } => {
                // Constants and static fields are invariant
                matches!(field.as_str(), "VERSION" | "JURISDICTION")
            }
            ConditionNode::HasAttribute { .. } => false,
            ConditionNode::And(left, right) => self.is_invariant(left) && self.is_invariant(right),
            ConditionNode::Or(left, right) => self.is_invariant(left) && self.is_invariant(right),
            ConditionNode::Not(inner) => self.is_invariant(inner),
            _ => false,
        }
    }

    // ==================== COMMON SUBEXPRESSION ELIMINATION ====================

    /// Eliminates common subexpressions (v0.1.8.2)
    fn optimize_document_cse(&mut self, mut document: LegalDocument) -> LegalDocument {
        for statute in &mut document.statutes {
            self.eliminate_common_subexpressions(statute);
        }
        document
    }

    /// Eliminates common subexpressions in a statute
    fn eliminate_common_subexpressions(&mut self, statute: &mut StatuteNode) {
        let mut seen = HashMap::new();
        let mut deduplicated = Vec::new();

        for condition in &statute.conditions {
            let key = self.condition_signature(condition);
            if let std::collections::hash_map::Entry::Vacant(e) = seen.entry(key) {
                e.insert(condition.clone());
                deduplicated.push(condition.clone());
            } else {
                self.stats.eliminated_subexpressions += 1;
                // Skip duplicate
            }
        }

        statute.conditions = deduplicated;
    }

    /// Creates a signature for a condition to detect duplicates
    fn condition_signature(&self, condition: &ConditionNode) -> String {
        match condition {
            ConditionNode::Comparison {
                field,
                operator,
                value,
            } => format!("CMP:{}:{}:{:?}", field, operator, value),
            ConditionNode::HasAttribute { key } => format!("HAS:{}", key),
            ConditionNode::And(left, right) => {
                format!(
                    "AND:{}:{}",
                    self.condition_signature(left),
                    self.condition_signature(right)
                )
            }
            ConditionNode::Or(left, right) => {
                format!(
                    "OR:{}:{}",
                    self.condition_signature(left),
                    self.condition_signature(right)
                )
            }
            ConditionNode::Not(inner) => format!("NOT:{}", self.condition_signature(inner)),
            ConditionNode::Between { field, min, max } => {
                format!("BETWEEN:{}:{:?}:{:?}", field, min, max)
            }
            ConditionNode::In { field, values } => {
                format!("IN:{}:{:?}", field, values)
            }
            ConditionNode::Like { field, pattern } => format!("LIKE:{}:{}", field, pattern),
            ConditionNode::Matches {
                field,
                regex_pattern,
            } => format!("MATCHES:{}:{}", field, regex_pattern),
            ConditionNode::InRange {
                field,
                min,
                max,
                inclusive_min,
                inclusive_max,
            } => format!(
                "INRANGE:{}:{:?}:{:?}:{}:{}",
                field, min, max, inclusive_min, inclusive_max
            ),
            ConditionNode::NotInRange {
                field,
                min,
                max,
                inclusive_min,
                inclusive_max,
            } => format!(
                "NOTINRANGE:{}:{:?}:{:?}:{}:{}",
                field, min, max, inclusive_min, inclusive_max
            ),
            ConditionNode::TemporalComparison {
                field,
                operator,
                value,
            } => format!("TEMPORAL:{:?}:{}:{:?}", field, operator, value),
        }
    }

    // ==================== DEAD CODE ELIMINATION ====================

    /// Eliminates dead (unreachable) conditions (v0.1.8.3)
    fn optimize_document_dead_code(&mut self, mut document: LegalDocument) -> LegalDocument {
        for statute in &mut document.statutes {
            self.eliminate_dead_conditions(statute);
        }
        document
    }

    /// Eliminates dead conditions in a statute
    fn eliminate_dead_conditions(&mut self, statute: &mut StatuteNode) {
        let mut live_conditions = Vec::new();

        for condition in &statute.conditions {
            if !self.is_dead_condition(condition) {
                live_conditions.push(condition.clone());
            } else {
                self.stats.removed_dead_conditions += 1;
            }
        }

        statute.conditions = live_conditions;
    }

    /// Checks if a condition is dead (always false or contradictory)
    fn is_dead_condition(&self, condition: &ConditionNode) -> bool {
        match condition {
            ConditionNode::Comparison {
                field,
                operator,
                value,
            } => {
                // Detect always-false conditions like age < 0
                if (field == "age" || field == "AGE") && operator == "<" {
                    if let ConditionValue::Number(n) = value {
                        return *n == 0;
                    }
                }
                false
            }
            ConditionNode::And(left, right) => {
                // If either side is dead, the AND is dead
                self.is_dead_condition(left) || self.is_dead_condition(right)
            }
            ConditionNode::Between { min, max, .. } => {
                // Detect impossible ranges where min > max
                if let (ConditionValue::Number(min_val), ConditionValue::Number(max_val)) =
                    (min, max)
                {
                    return min_val > max_val;
                }
                false
            }
            _ => false,
        }
    }

    // ==================== CONDITION REORDERING ====================

    /// Reorders conditions for short-circuit optimization (v0.1.8.4)
    fn optimize_document_reordering(&mut self, mut document: LegalDocument) -> LegalDocument {
        for statute in &mut document.statutes {
            self.reorder_conditions(statute);
        }
        document
    }

    /// Reorders conditions to put cheaper checks first
    fn reorder_conditions(&mut self, statute: &mut StatuteNode) {
        let original_len = statute.conditions.len();

        // Sort by estimated cost (cheap checks first)
        statute.conditions.sort_by_key(|c| self.condition_cost(c));

        // Count how many were reordered
        self.stats.reordered_conditions += original_len;
    }

    /// Estimates the cost of evaluating a condition
    fn condition_cost(&self, condition: &ConditionNode) -> u32 {
        match condition {
            ConditionNode::HasAttribute { .. } => 1, // Cheapest: simple attribute check
            ConditionNode::Comparison { .. } => 2,   // Cheap: single comparison
            ConditionNode::Between { .. } => 3,      // Medium: two comparisons
            ConditionNode::In { values, .. } => {
                // Cost grows with number of values
                5 + values.len() as u32
            }
            ConditionNode::Like { .. } => 10, // Expensive: pattern matching
            ConditionNode::Matches { .. } => 15, // Very expensive: regex
            ConditionNode::And(left, right) => {
                // Cost is sum of both sides
                self.condition_cost(left) + self.condition_cost(right)
            }
            ConditionNode::Or(left, right) => {
                // Cost is sum of both sides
                self.condition_cost(left) + self.condition_cost(right)
            }
            ConditionNode::Not(inner) => self.condition_cost(inner) + 1,
            _ => 5,
        }
    }

    // ==================== CONSTANT FOLDING ====================

    /// Folds constants in static expressions (v0.1.8.5)
    fn optimize_document_constant_folding(&mut self, mut document: LegalDocument) -> LegalDocument {
        for statute in &mut document.statutes {
            self.fold_constants(statute);
        }
        document
    }

    /// Folds constants in a statute's conditions
    fn fold_constants(&mut self, statute: &mut StatuteNode) {
        let mut folded_conditions = Vec::new();

        for condition in &statute.conditions {
            if let Some(folded) = self.fold_condition(condition) {
                folded_conditions.push(folded);
            } else {
                folded_conditions.push(condition.clone());
            }
        }

        statute.conditions = folded_conditions;
    }

    /// Attempts to fold constants in a condition
    fn fold_condition(&mut self, condition: &ConditionNode) -> Option<ConditionNode> {
        match condition {
            ConditionNode::And(left, right) => {
                let folded_left = self
                    .fold_condition(left)
                    .unwrap_or_else(|| (**left).clone());
                let folded_right = self
                    .fold_condition(right)
                    .unwrap_or_else(|| (**right).clone());

                // Simplify: true AND x => x
                if self.is_always_true(&folded_left) {
                    self.stats.folded_constants += 1;
                    return Some(folded_right);
                }
                // Simplify: x AND true => x
                if self.is_always_true(&folded_right) {
                    self.stats.folded_constants += 1;
                    return Some(folded_left);
                }

                Some(ConditionNode::And(
                    Box::new(folded_left),
                    Box::new(folded_right),
                ))
            }
            ConditionNode::Or(left, right) => {
                let folded_left = self
                    .fold_condition(left)
                    .unwrap_or_else(|| (**left).clone());
                let folded_right = self
                    .fold_condition(right)
                    .unwrap_or_else(|| (**right).clone());

                // Simplify: false OR x => x
                if self.is_always_false(&folded_left) {
                    self.stats.folded_constants += 1;
                    return Some(folded_right);
                }
                // Simplify: x OR false => x
                if self.is_always_false(&folded_right) {
                    self.stats.folded_constants += 1;
                    return Some(folded_left);
                }

                Some(ConditionNode::Or(
                    Box::new(folded_left),
                    Box::new(folded_right),
                ))
            }
            ConditionNode::Not(inner) => {
                let folded = self
                    .fold_condition(inner)
                    .unwrap_or_else(|| (**inner).clone());

                // Double negation elimination: NOT(NOT(x)) => x
                if let ConditionNode::Not(inner_inner) = &folded {
                    self.stats.folded_constants += 1;
                    return Some((**inner_inner).clone());
                }

                Some(ConditionNode::Not(Box::new(folded)))
            }
            _ => None,
        }
    }

    /// Checks if a condition is always true
    fn is_always_true(&self, _condition: &ConditionNode) -> bool {
        // Placeholder for more sophisticated analysis
        false
    }

    /// Checks if a condition is always false
    fn is_always_false(&self, condition: &ConditionNode) -> bool {
        self.is_dead_condition(condition)
    }
}

impl Default for Optimizer {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience functions for single-pass optimizations
impl Optimizer {
    /// Applies only condition hoisting
    pub fn hoist_only(&mut self, document: LegalDocument) -> LegalDocument {
        self.optimize_document_hoisting(document)
    }

    /// Applies only common subexpression elimination
    pub fn cse_only(&mut self, document: LegalDocument) -> LegalDocument {
        self.optimize_document_cse(document)
    }

    /// Applies only dead code elimination
    pub fn dead_code_only(&mut self, document: LegalDocument) -> LegalDocument {
        self.optimize_document_dead_code(document)
    }

    /// Applies only condition reordering
    pub fn reorder_only(&mut self, document: LegalDocument) -> LegalDocument {
        self.optimize_document_reordering(document)
    }

    /// Applies only constant folding
    pub fn fold_only(&mut self, document: LegalDocument) -> LegalDocument {
        self.optimize_document_constant_folding(document)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::module_system::Visibility;

    fn create_test_statute(id: &str, conditions: Vec<ConditionNode>) -> StatuteNode {
        StatuteNode {
            id: id.to_string(),
            visibility: Visibility::Private,
            title: "Test".to_string(),
            conditions,
            effects: vec![],
            discretion: None,
            exceptions: vec![],
            amendments: vec![],
            supersedes: vec![],
            defaults: vec![],
            requires: vec![],
            delegates: vec![],
            scope: None,
            constraints: vec![],
            priority: None,
        }
    }

    #[test]
    fn test_optimizer_new() {
        let optimizer = Optimizer::new();
        assert_eq!(optimizer.stats().hoisted_conditions, 0);
    }

    #[test]
    fn test_condition_hoisting() {
        let mut optimizer = Optimizer::new();

        let mut statute = create_test_statute(
            "test",
            vec![
                ConditionNode::Comparison {
                    field: "age".to_string(),
                    operator: ">".to_string(),
                    value: ConditionValue::Number(18),
                },
                ConditionNode::Comparison {
                    field: "VERSION".to_string(),
                    operator: "=".to_string(),
                    value: ConditionValue::Number(2),
                },
            ],
        );

        optimizer.hoist_conditions(&mut statute);

        // VERSION condition should be hoisted first
        if let ConditionNode::Comparison { field, .. } = &statute.conditions[0] {
            assert_eq!(field, "VERSION");
        } else {
            panic!("Expected VERSION condition first");
        }
    }

    #[test]
    fn test_common_subexpression_elimination() {
        let mut optimizer = Optimizer::new();

        let condition = ConditionNode::Comparison {
            field: "age".to_string(),
            operator: ">".to_string(),
            value: ConditionValue::Number(18),
        };

        let mut statute = create_test_statute("test", vec![condition.clone(), condition.clone()]);

        optimizer.eliminate_common_subexpressions(&mut statute);

        // Should have only one condition after deduplication
        assert_eq!(statute.conditions.len(), 1);
        assert_eq!(optimizer.stats().eliminated_subexpressions, 1);
    }

    #[test]
    fn test_dead_condition_detection() {
        let optimizer = Optimizer::new();

        // age < 0 is always false
        let dead = ConditionNode::Comparison {
            field: "age".to_string(),
            operator: "<".to_string(),
            value: ConditionValue::Number(0),
        };

        assert!(optimizer.is_dead_condition(&dead));
    }

    #[test]
    fn test_dead_condition_elimination() {
        let mut optimizer = Optimizer::new();

        let dead = ConditionNode::Comparison {
            field: "age".to_string(),
            operator: "<".to_string(),
            value: ConditionValue::Number(0),
        };

        let live = ConditionNode::Comparison {
            field: "age".to_string(),
            operator: ">".to_string(),
            value: ConditionValue::Number(18),
        };

        let mut statute = create_test_statute("test", vec![live, dead]);

        optimizer.eliminate_dead_conditions(&mut statute);

        assert_eq!(statute.conditions.len(), 1);
        assert_eq!(optimizer.stats().removed_dead_conditions, 1);
    }

    #[test]
    fn test_condition_cost() {
        let optimizer = Optimizer::new();

        let has = ConditionNode::HasAttribute {
            key: "citizen".to_string(),
        };
        let comparison = ConditionNode::Comparison {
            field: "age".to_string(),
            operator: ">".to_string(),
            value: ConditionValue::Number(18),
        };
        let regex = ConditionNode::Matches {
            field: "name".to_string(),
            regex_pattern: ".*".to_string(),
        };

        assert!(optimizer.condition_cost(&has) < optimizer.condition_cost(&comparison));
        assert!(optimizer.condition_cost(&comparison) < optimizer.condition_cost(&regex));
    }

    #[test]
    fn test_condition_reordering() {
        let mut optimizer = Optimizer::new();

        // Create conditions in expensive-first order
        let expensive = ConditionNode::Matches {
            field: "name".to_string(),
            regex_pattern: ".*".to_string(),
        };
        let cheap = ConditionNode::HasAttribute {
            key: "citizen".to_string(),
        };

        let mut statute = create_test_statute("test", vec![expensive, cheap]);

        optimizer.reorder_conditions(&mut statute);

        // Cheap condition should be first now
        if let ConditionNode::HasAttribute { .. } = &statute.conditions[0] {
            // Success
        } else {
            panic!("Expected HasAttribute condition first after reordering");
        }
    }

    #[test]
    fn test_constant_folding_double_negation() {
        let mut optimizer = Optimizer::new();

        let condition = ConditionNode::HasAttribute {
            key: "citizen".to_string(),
        };
        let double_neg =
            ConditionNode::Not(Box::new(ConditionNode::Not(Box::new(condition.clone()))));

        let folded = optimizer.fold_condition(&double_neg);
        assert!(folded.is_some());
        assert_eq!(optimizer.stats().folded_constants, 1);

        // Should be simplified to just the original condition
        if let Some(ConditionNode::HasAttribute { key }) = folded {
            assert_eq!(key, "citizen");
        }
    }

    #[test]
    fn test_full_optimization_pipeline() {
        let mut optimizer = Optimizer::new();

        let doc = LegalDocument {
            namespace: None,
            exports: vec![],
            imports: vec![],
            statutes: vec![create_test_statute(
                "test",
                vec![
                    ConditionNode::Comparison {
                        field: "age".to_string(),
                        operator: ">".to_string(),
                        value: ConditionValue::Number(18),
                    },
                    ConditionNode::Comparison {
                        field: "VERSION".to_string(),
                        operator: "=".to_string(),
                        value: ConditionValue::Number(2),
                    },
                ],
            )],
        };

        let optimized = optimizer.optimize(doc);
        assert_eq!(optimized.statutes.len(), 1);
    }

    #[test]
    fn test_impossible_range_detection() {
        let optimizer = Optimizer::new();

        let impossible = ConditionNode::Between {
            field: "age".to_string(),
            min: ConditionValue::Number(100),
            max: ConditionValue::Number(18),
        };

        assert!(optimizer.is_dead_condition(&impossible));
    }

    #[test]
    fn test_stats_reset() {
        let mut optimizer = Optimizer::new();
        optimizer.stats.hoisted_conditions = 5;
        optimizer.reset_stats();
        assert_eq!(optimizer.stats().hoisted_conditions, 0);
    }
}
