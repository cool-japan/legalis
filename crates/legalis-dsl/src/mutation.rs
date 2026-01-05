//! Mutation Testing Framework for Legalis DSL
//!
//! This module provides mutation testing capabilities to assess test quality
//! by introducing small changes to AST nodes and verifying tests catch them.

use crate::ast::*;
use std::fmt;

/// Type of mutation applied to an AST node
#[derive(Debug, Clone, PartialEq)]
pub enum MutationType {
    /// Change comparison operator (e.g., > to <, >= to <=)
    ChangeOperator { from: String, to: String },
    /// Negate boolean condition
    NegateCondition,
    /// Change numeric value
    ChangeNumericValue { from: i64, to: i64 },
    /// Change string value
    ChangeStringValue { from: String, to: String },
    /// Remove condition (replace with always true)
    RemoveCondition,
    /// Swap AND/OR operators
    SwapLogicalOperator,
    /// Change effect type (GRANT <-> REVOKE, OBLIGATION <-> PROHIBITION)
    ChangeEffectType { from: String, to: String },
    /// Remove effect
    RemoveEffect,
    /// Remove exception
    RemoveException,
    /// Swap condition order in AND/OR
    SwapConditionOrder,
}

impl fmt::Display for MutationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MutationType::ChangeOperator { from, to } => {
                write!(f, "Changed operator from '{}' to '{}'", from, to)
            }
            MutationType::NegateCondition => write!(f, "Negated condition"),
            MutationType::ChangeNumericValue { from, to } => {
                write!(f, "Changed numeric value from {} to {}", from, to)
            }
            MutationType::ChangeStringValue { from, to } => {
                write!(f, "Changed string value from '{}' to '{}'", from, to)
            }
            MutationType::RemoveCondition => write!(f, "Removed condition"),
            MutationType::SwapLogicalOperator => write!(f, "Swapped AND/OR operator"),
            MutationType::ChangeEffectType { from, to } => {
                write!(f, "Changed effect type from {} to {}", from, to)
            }
            MutationType::RemoveEffect => write!(f, "Removed effect"),
            MutationType::RemoveException => write!(f, "Removed exception"),
            MutationType::SwapConditionOrder => write!(f, "Swapped condition order"),
        }
    }
}

/// A mutation applied to an AST
#[derive(Debug, Clone, PartialEq)]
pub struct Mutation {
    /// Type of mutation
    pub mutation_type: MutationType,
    /// Location in the document (statute ID)
    pub location: String,
    /// Description of what was mutated
    pub description: String,
}

/// Result of mutation testing
#[derive(Debug, Clone, PartialEq)]
pub enum MutationResult {
    /// Mutation was killed (test detected it)
    Killed { test_name: String },
    /// Mutation survived (test did not detect it)
    Survived,
    /// Mutation caused a compile/parse error
    Error { message: String },
}

/// Mutation test report
#[derive(Debug, Clone)]
pub struct MutationReport {
    /// Total number of mutations attempted
    pub total_mutations: usize,
    /// Number of mutations killed by tests
    pub killed: usize,
    /// Number of mutations that survived
    pub survived: usize,
    /// Number of mutations that caused errors
    pub errors: usize,
    /// Mutation score (percentage of non-error mutations killed)
    pub mutation_score: f64,
    /// Individual mutation results
    pub results: Vec<(Mutation, MutationResult)>,
}

impl MutationReport {
    /// Create a new mutation report
    pub fn new() -> Self {
        Self {
            total_mutations: 0,
            killed: 0,
            survived: 0,
            errors: 0,
            mutation_score: 0.0,
            results: Vec::new(),
        }
    }

    /// Add a mutation result
    pub fn add_result(&mut self, mutation: Mutation, result: MutationResult) {
        self.total_mutations += 1;
        match result {
            MutationResult::Killed { .. } => self.killed += 1,
            MutationResult::Survived => self.survived += 1,
            MutationResult::Error { .. } => self.errors += 1,
        }
        self.results.push((mutation, result));
        self.calculate_score();
    }

    fn calculate_score(&mut self) {
        let testable = self.total_mutations - self.errors;
        if testable > 0 {
            self.mutation_score = (self.killed as f64 / testable as f64) * 100.0;
        }
    }

    /// Generate a summary report
    pub fn summary(&self) -> String {
        format!(
            "Mutation Testing Report\n\
             =======================\n\
             Total mutations: {}\n\
             Killed: {} ({:.1}%)\n\
             Survived: {} ({:.1}%)\n\
             Errors: {}\n\
             Mutation Score: {:.1}%",
            self.total_mutations,
            self.killed,
            (self.killed as f64 / self.total_mutations as f64) * 100.0,
            self.survived,
            (self.survived as f64 / self.total_mutations as f64) * 100.0,
            self.errors,
            self.mutation_score
        )
    }
}

impl Default for MutationReport {
    fn default() -> Self {
        Self::new()
    }
}

/// Mutation operator that generates mutations for AST nodes
pub struct MutationOperator;

impl MutationOperator {
    /// Generate all possible mutations for a condition
    pub fn mutate_condition(
        condition: &ConditionNode,
        statute_id: &str,
    ) -> Vec<(ConditionNode, Mutation)> {
        let mut mutations = Vec::new();

        match condition {
            ConditionNode::Comparison {
                field,
                operator,
                value,
            } => {
                // Mutate operator
                let operator_mutations = Self::get_operator_mutations(operator);
                for new_op in operator_mutations {
                    let mutated = ConditionNode::Comparison {
                        field: field.clone(),
                        operator: new_op.clone(),
                        value: value.clone(),
                    };
                    mutations.push((
                        mutated,
                        Mutation {
                            mutation_type: MutationType::ChangeOperator {
                                from: operator.clone(),
                                to: new_op,
                            },
                            location: statute_id.to_string(),
                            description: format!("Changed operator in field '{}'", field),
                        },
                    ));
                }

                // Mutate value
                if let ConditionValue::Number(n) = value {
                    for new_val in [n + 1, n - 1, 0] {
                        let mutated = ConditionNode::Comparison {
                            field: field.clone(),
                            operator: operator.clone(),
                            value: ConditionValue::Number(new_val),
                        };
                        mutations.push((
                            mutated,
                            Mutation {
                                mutation_type: MutationType::ChangeNumericValue {
                                    from: *n,
                                    to: new_val,
                                },
                                location: statute_id.to_string(),
                                description: format!("Changed numeric value in field '{}'", field),
                            },
                        ));
                    }
                }
            }
            ConditionNode::And(left, right) => {
                // Swap to OR
                let mutated = ConditionNode::Or(left.clone(), right.clone());
                mutations.push((
                    mutated,
                    Mutation {
                        mutation_type: MutationType::SwapLogicalOperator,
                        location: statute_id.to_string(),
                        description: "Changed AND to OR".to_string(),
                    },
                ));

                // Swap order
                let mutated = ConditionNode::And(right.clone(), left.clone());
                mutations.push((
                    mutated,
                    Mutation {
                        mutation_type: MutationType::SwapConditionOrder,
                        location: statute_id.to_string(),
                        description: "Swapped AND condition order".to_string(),
                    },
                ));

                // Recursively mutate children
                for (mutated_left, mutation) in Self::mutate_condition(left, statute_id) {
                    mutations.push((
                        ConditionNode::And(Box::new(mutated_left), right.clone()),
                        mutation,
                    ));
                }
                for (mutated_right, mutation) in Self::mutate_condition(right, statute_id) {
                    mutations.push((
                        ConditionNode::And(left.clone(), Box::new(mutated_right)),
                        mutation,
                    ));
                }
            }
            ConditionNode::Or(left, right) => {
                // Swap to AND
                let mutated = ConditionNode::And(left.clone(), right.clone());
                mutations.push((
                    mutated,
                    Mutation {
                        mutation_type: MutationType::SwapLogicalOperator,
                        location: statute_id.to_string(),
                        description: "Changed OR to AND".to_string(),
                    },
                ));

                // Recursively mutate children
                for (mutated_left, mutation) in Self::mutate_condition(left, statute_id) {
                    mutations.push((
                        ConditionNode::Or(Box::new(mutated_left), right.clone()),
                        mutation,
                    ));
                }
                for (mutated_right, mutation) in Self::mutate_condition(right, statute_id) {
                    mutations.push((
                        ConditionNode::Or(left.clone(), Box::new(mutated_right)),
                        mutation,
                    ));
                }
            }
            ConditionNode::Not(inner) => {
                // Remove negation
                mutations.push((
                    (**inner).clone(),
                    Mutation {
                        mutation_type: MutationType::NegateCondition,
                        location: statute_id.to_string(),
                        description: "Removed NOT operator".to_string(),
                    },
                ));

                // Recursively mutate inner
                for (mutated_inner, mutation) in Self::mutate_condition(inner, statute_id) {
                    mutations.push((ConditionNode::Not(Box::new(mutated_inner)), mutation));
                }
            }
            ConditionNode::Between { field, min, max } => {
                // Swap min and max
                let mutated = ConditionNode::Between {
                    field: field.clone(),
                    min: max.clone(),
                    max: min.clone(),
                };
                mutations.push((
                    mutated,
                    Mutation {
                        mutation_type: MutationType::SwapConditionOrder,
                        location: statute_id.to_string(),
                        description: format!("Swapped min/max in BETWEEN for field '{}'", field),
                    },
                ));
            }
            _ => {}
        }

        mutations
    }

    /// Get operator mutations (boundary conditions)
    fn get_operator_mutations(operator: &str) -> Vec<String> {
        match operator {
            ">" => vec![">=".to_string(), "<".to_string()],
            ">=" => vec![">".to_string(), "<=".to_string()],
            "<" => vec!["<=".to_string(), ">".to_string()],
            "<=" => vec!["<".to_string(), ">=".to_string()],
            "=" | "==" => vec!["!=".to_string()],
            "!=" | "<>" => vec!["=".to_string()],
            _ => vec![],
        }
    }

    /// Generate mutations for an effect
    pub fn mutate_effect(effect: &EffectNode, statute_id: &str) -> Vec<(EffectNode, Mutation)> {
        let mut mutations = Vec::new();

        let opposite_effect = match effect.effect_type.as_str() {
            "GRANT" => "REVOKE",
            "REVOKE" => "GRANT",
            "OBLIGATION" => "PROHIBITION",
            "PROHIBITION" => "OBLIGATION",
            _ => return mutations,
        };

        let mutated = EffectNode {
            effect_type: opposite_effect.to_string(),
            description: effect.description.clone(),
            parameters: effect.parameters.clone(),
        };

        mutations.push((
            mutated,
            Mutation {
                mutation_type: MutationType::ChangeEffectType {
                    from: effect.effect_type.clone(),
                    to: opposite_effect.to_string(),
                },
                location: statute_id.to_string(),
                description: format!(
                    "Changed effect type: {} -> {}",
                    effect.effect_type, opposite_effect
                ),
            },
        ));

        mutations
    }

    /// Generate all mutations for a statute
    pub fn mutate_statute(statute: &StatuteNode) -> Vec<(StatuteNode, Mutation)> {
        let mut mutations = Vec::new();

        // Mutate conditions
        for (i, condition) in statute.conditions.iter().enumerate() {
            for (mutated_cond, mutation) in Self::mutate_condition(condition, &statute.id) {
                let mut mutated_statute = statute.clone();
                mutated_statute.conditions[i] = mutated_cond;
                mutations.push((mutated_statute, mutation));
            }
        }

        // Mutate effects
        for (i, effect) in statute.effects.iter().enumerate() {
            for (mutated_effect, mutation) in Self::mutate_effect(effect, &statute.id) {
                let mut mutated_statute = statute.clone();
                mutated_statute.effects[i] = mutated_effect;
                mutations.push((mutated_statute, mutation));
            }
        }

        // Remove exceptions
        if !statute.exceptions.is_empty() {
            for i in 0..statute.exceptions.len() {
                let mut mutated_statute = statute.clone();
                mutated_statute.exceptions.remove(i);
                mutations.push((
                    mutated_statute,
                    Mutation {
                        mutation_type: MutationType::RemoveException,
                        location: statute.id.clone(),
                        description: format!("Removed exception at index {}", i),
                    },
                ));
            }
        }

        mutations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operator_mutations() {
        let ops = MutationOperator::get_operator_mutations(">");
        assert!(ops.contains(&">=".to_string()));
        assert!(ops.contains(&"<".to_string()));
    }

    #[test]
    fn test_mutate_comparison_condition() {
        let condition = ConditionNode::Comparison {
            field: "age".to_string(),
            operator: ">".to_string(),
            value: ConditionValue::Number(18),
        };

        let mutations = MutationOperator::mutate_condition(&condition, "test-001");
        assert!(!mutations.is_empty());

        // Should have operator mutations and value mutations
        let has_operator_mutation = mutations
            .iter()
            .any(|(_, m)| matches!(m.mutation_type, MutationType::ChangeOperator { .. }));
        let has_value_mutation = mutations
            .iter()
            .any(|(_, m)| matches!(m.mutation_type, MutationType::ChangeNumericValue { .. }));

        assert!(has_operator_mutation);
        assert!(has_value_mutation);
    }

    #[test]
    fn test_mutate_and_condition() {
        let condition = ConditionNode::And(
            Box::new(ConditionNode::Comparison {
                field: "age".to_string(),
                operator: ">".to_string(),
                value: ConditionValue::Number(18),
            }),
            Box::new(ConditionNode::HasAttribute {
                key: "citizen".to_string(),
            }),
        );

        let mutations = MutationOperator::mutate_condition(&condition, "test-001");
        assert!(!mutations.is_empty());

        // Should have AND->OR mutation
        let has_swap = mutations
            .iter()
            .any(|(_, m)| matches!(m.mutation_type, MutationType::SwapLogicalOperator));
        assert!(has_swap);
    }

    #[test]
    fn test_mutate_effect() {
        let effect = EffectNode {
            effect_type: "GRANT".to_string(),
            description: "voting rights".to_string(),
            parameters: vec![],
        };

        let mutations = MutationOperator::mutate_effect(&effect, "test-001");
        assert_eq!(mutations.len(), 1);

        let (mutated, mutation) = &mutations[0];
        assert_eq!(mutated.effect_type, "REVOKE");
        assert!(matches!(
            mutation.mutation_type,
            MutationType::ChangeEffectType { .. }
        ));
    }

    #[test]
    fn test_mutation_report() {
        let mut report = MutationReport::new();

        report.add_result(
            Mutation {
                mutation_type: MutationType::ChangeOperator {
                    from: ">".to_string(),
                    to: ">=".to_string(),
                },
                location: "test-001".to_string(),
                description: "Test".to_string(),
            },
            MutationResult::Killed {
                test_name: "test_age_check".to_string(),
            },
        );

        report.add_result(
            Mutation {
                mutation_type: MutationType::NegateCondition,
                location: "test-002".to_string(),
                description: "Test".to_string(),
            },
            MutationResult::Survived,
        );

        assert_eq!(report.total_mutations, 2);
        assert_eq!(report.killed, 1);
        assert_eq!(report.survived, 1);
        assert_eq!(report.mutation_score, 50.0);
    }
}
