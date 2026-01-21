//! Validation utilities for interoperability operations.
//!
//! This module provides tools to validate statutes, check conversion quality,
//! and ensure semantic preservation during format conversions.

use crate::{ConversionReport, InteropError, InteropResult};
use legalis_core::{Condition, EffectType, Statute};
use serde::{Deserialize, Serialize};

/// Validation result for a statute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatuteValidation {
    /// Whether the statute is valid
    pub is_valid: bool,
    /// Validation errors
    pub errors: Vec<String>,
    /// Validation warnings
    pub warnings: Vec<String>,
    /// Validation suggestions
    pub suggestions: Vec<String>,
}

impl StatuteValidation {
    /// Creates a new validation result.
    pub fn new() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            suggestions: Vec::new(),
        }
    }

    /// Adds an error and marks validation as failed.
    pub fn add_error(&mut self, error: impl Into<String>) {
        self.errors.push(error.into());
        self.is_valid = false;
    }

    /// Adds a warning.
    pub fn add_warning(&mut self, warning: impl Into<String>) {
        self.warnings.push(warning.into());
    }

    /// Adds a suggestion.
    pub fn add_suggestion(&mut self, suggestion: impl Into<String>) {
        self.suggestions.push(suggestion.into());
    }

    /// Returns true if there are any issues (errors or warnings).
    pub fn has_issues(&self) -> bool {
        !self.errors.is_empty() || !self.warnings.is_empty()
    }
}

impl Default for StatuteValidation {
    fn default() -> Self {
        Self::new()
    }
}

/// Validator for statutes and conversions.
pub struct Validator {
    /// Maximum allowed condition depth
    max_condition_depth: usize,
    /// Maximum allowed number of preconditions
    max_preconditions: usize,
    /// Whether to enforce non-empty IDs
    require_id: bool,
    /// Whether to enforce non-empty titles
    require_title: bool,
}

impl Validator {
    /// Creates a new validator with default settings.
    pub fn new() -> Self {
        Self {
            max_condition_depth: 10,
            max_preconditions: 50,
            require_id: true,
            require_title: false,
        }
    }

    /// Sets the maximum condition depth.
    pub fn with_max_condition_depth(mut self, depth: usize) -> Self {
        self.max_condition_depth = depth;
        self
    }

    /// Sets the maximum number of preconditions.
    pub fn with_max_preconditions(mut self, count: usize) -> Self {
        self.max_preconditions = count;
        self
    }

    /// Sets whether to require non-empty IDs.
    pub fn with_require_id(mut self, require: bool) -> Self {
        self.require_id = require;
        self
    }

    /// Sets whether to require non-empty titles.
    pub fn with_require_title(mut self, require: bool) -> Self {
        self.require_title = require;
        self
    }

    /// Validates a statute.
    pub fn validate_statute(&self, statute: &Statute) -> StatuteValidation {
        let mut validation = StatuteValidation::new();

        // Check ID
        if self.require_id && statute.id.is_empty() {
            validation.add_error("Statute ID cannot be empty");
        }

        // Check title
        if self.require_title && statute.title.is_empty() {
            validation.add_warning("Statute title is empty");
        }

        // Check effect description
        if statute.effect.description.is_empty() {
            validation.add_warning("Effect description is empty");
        }

        // Check preconditions
        if statute.preconditions.len() > self.max_preconditions {
            validation.add_error(format!(
                "Too many preconditions: {} (max {})",
                statute.preconditions.len(),
                self.max_preconditions
            ));
        }

        for (i, precondition) in statute.preconditions.iter().enumerate() {
            self.validate_condition(precondition, &mut validation, i);
        }

        // Check for tautologies
        if statute.preconditions.is_empty() && statute.effect.effect_type == EffectType::Grant {
            validation
                .add_suggestion("Consider adding preconditions to make the grant more specific");
        }

        // Check jurisdiction
        if statute.jurisdiction.is_none() {
            validation.add_suggestion("Consider specifying a jurisdiction for clarity");
        }

        validation
    }

    /// Validates a condition.
    fn validate_condition(
        &self,
        condition: &Condition,
        validation: &mut StatuteValidation,
        index: usize,
    ) {
        let depth = condition.depth();
        if depth > self.max_condition_depth {
            validation.add_error(format!(
                "Condition {} is too deeply nested: {} levels (max {})",
                index, depth, self.max_condition_depth
            ));
        }

        // Check for potential issues
        if let Condition::Not(inner) = condition
            && matches!(**inner, Condition::Not(_))
        {
            validation.add_warning(format!(
                "Condition {} has double negation - consider simplifying",
                index
            ));
        }

        // Check for tautologies (A OR NOT A or NOT A OR A)
        if let Condition::Or(left, right) = condition {
            let is_tautology = match (left.as_ref(), right.as_ref()) {
                (Condition::Not(a), b) => **a == *b,
                (a, Condition::Not(b)) => *a == **b,
                _ => false,
            };
            if is_tautology {
                validation.add_warning(format!("Condition {} is a tautology: A OR NOT A", index));
            }
        }

        // Check for contradictions (A AND NOT A or NOT A AND A)
        if let Condition::And(left, right) = condition {
            let is_contradiction = match (left.as_ref(), right.as_ref()) {
                (Condition::Not(a), b) => **a == *b,
                (a, Condition::Not(b)) => *a == **b,
                _ => false,
            };
            if is_contradiction {
                validation.add_error(format!(
                    "Condition {} is a contradiction: A AND NOT A",
                    index
                ));
            }
        }
    }

    /// Validates a batch of statutes.
    pub fn validate_batch(&self, statutes: &[Statute]) -> Vec<(usize, StatuteValidation)> {
        statutes
            .iter()
            .enumerate()
            .map(|(i, statute)| (i, self.validate_statute(statute)))
            .collect()
    }

    /// Validates a conversion report's quality.
    pub fn validate_conversion_report(&self, report: &ConversionReport) -> InteropResult<()> {
        if !report.is_high_quality() && report.confidence < 0.5 {
            return Err(InteropError::ConversionError(format!(
                "Conversion quality too low: confidence = {:.2}",
                report.confidence
            )));
        }
        Ok(())
    }

    /// Checks if two statutes are semantically equivalent.
    pub fn are_equivalent(&self, a: &Statute, b: &Statute) -> bool {
        // Check effect types match
        if a.effect.effect_type != b.effect.effect_type {
            return false;
        }

        // Check same number of preconditions
        if a.preconditions.len() != b.preconditions.len() {
            return false;
        }

        // For a full semantic equivalence check, we'd need to compare
        // conditions structurally (handling commutative operations, etc.)
        // For now, we do a simple structural comparison
        for (cond_a, cond_b) in a.preconditions.iter().zip(b.preconditions.iter()) {
            if !Self::conditions_equivalent(cond_a, cond_b) {
                return false;
            }
        }

        true
    }

    /// Checks if two conditions are equivalent (simplified check).
    fn conditions_equivalent(a: &Condition, b: &Condition) -> bool {
        // This is a simplified check - a full implementation would handle
        // commutativity (AND/OR order doesn't matter), double negation, etc.
        match (a, b) {
            (
                Condition::Age {
                    operator: op1,
                    value: v1,
                },
                Condition::Age {
                    operator: op2,
                    value: v2,
                },
            ) => op1 == op2 && v1 == v2,
            (
                Condition::Income {
                    operator: op1,
                    value: v1,
                },
                Condition::Income {
                    operator: op2,
                    value: v2,
                },
            ) => op1 == op2 && v1 == v2,
            (Condition::And(l1, r1), Condition::And(l2, r2))
            | (Condition::Or(l1, r1), Condition::Or(l2, r2)) => {
                Self::conditions_equivalent(l1, l2) && Self::conditions_equivalent(r1, r2)
            }
            (Condition::Not(i1), Condition::Not(i2)) => Self::conditions_equivalent(i1, i2),
            (
                Condition::AttributeEquals { key: k1, value: v1 },
                Condition::AttributeEquals { key: k2, value: v2 },
            ) => k1 == k2 && v1 == v2,
            (Condition::HasAttribute { key: k1 }, Condition::HasAttribute { key: k2 }) => k1 == k2,
            (
                Condition::DateRange { start: s1, end: e1 },
                Condition::DateRange { start: s2, end: e2 },
            ) => s1 == s2 && e1 == e2,
            (
                Condition::Geographic {
                    region_type: rt1,
                    region_id: ri1,
                },
                Condition::Geographic {
                    region_type: rt2,
                    region_id: ri2,
                },
            ) => rt1 == rt2 && ri1 == ri2,
            (
                Condition::EntityRelationship {
                    relationship_type: rt1,
                    target_entity_id: te1,
                },
                Condition::EntityRelationship {
                    relationship_type: rt2,
                    target_entity_id: te2,
                },
            ) => rt1 == rt2 && te1 == te2,
            (
                Condition::ResidencyDuration {
                    operator: op1,
                    months: m1,
                },
                Condition::ResidencyDuration {
                    operator: op2,
                    months: m2,
                },
            ) => op1 == op2 && m1 == m2,
            (Condition::Custom { description: d1 }, Condition::Custom { description: d2 }) => {
                d1 == d2
            }
            _ => false,
        }
    }
}

impl Default for Validator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{ComparisonOp, Effect};

    #[test]
    fn test_validate_valid_statute() {
        let validator = Validator::new();
        let statute = Statute::new(
            "test-statute",
            "Test Statute",
            Effect::new(EffectType::Grant, "Grant permission"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let validation = validator.validate_statute(&statute);
        assert!(validation.is_valid);
        assert!(validation.errors.is_empty());
    }

    #[test]
    fn test_validate_empty_id() {
        let validator = Validator::new().with_require_id(true);
        let statute = Statute::new("", "Test", Effect::new(EffectType::Grant, "Permission"));

        let validation = validator.validate_statute(&statute);
        assert!(!validation.is_valid);
        assert!(!validation.errors.is_empty());
    }

    #[test]
    fn test_validate_too_many_preconditions() {
        let validator = Validator::new().with_max_preconditions(2);
        let mut statute =
            Statute::new("test", "Test", Effect::new(EffectType::Grant, "Permission"));
        statute.preconditions.push(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });
        statute.preconditions.push(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 21,
        });
        statute.preconditions.push(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 25,
        });

        let validation = validator.validate_statute(&statute);
        assert!(!validation.is_valid);
    }

    #[test]
    fn test_validate_deep_nesting() {
        let validator = Validator::new().with_max_condition_depth(3);
        let deep_condition = Condition::And(
            Box::new(Condition::And(
                Box::new(Condition::And(
                    Box::new(Condition::Age {
                        operator: ComparisonOp::GreaterOrEqual,
                        value: 18,
                    }),
                    Box::new(Condition::Age {
                        operator: ComparisonOp::GreaterOrEqual,
                        value: 21,
                    }),
                )),
                Box::new(Condition::Age {
                    operator: ComparisonOp::GreaterOrEqual,
                    value: 25,
                }),
            )),
            Box::new(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 30,
            }),
        );

        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Permission"))
            .with_precondition(deep_condition);

        let validation = validator.validate_statute(&statute);
        assert!(!validation.is_valid);
        assert!(
            validation
                .errors
                .iter()
                .any(|e| e.contains("too deeply nested"))
        );
    }

    #[test]
    fn test_validate_double_negation() {
        let validator = Validator::new();
        let double_not = Condition::Not(Box::new(Condition::Not(Box::new(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        }))));

        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Permission"))
            .with_precondition(double_not);

        let validation = validator.validate_statute(&statute);
        assert!(!validation.warnings.is_empty());
        assert!(
            validation
                .warnings
                .iter()
                .any(|w| w.contains("double negation"))
        );
    }

    #[test]
    fn test_validate_contradiction() {
        let validator = Validator::new();
        let age_cond = Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        };
        let contradiction = Condition::And(
            Box::new(age_cond.clone()),
            Box::new(Condition::Not(Box::new(age_cond))),
        );

        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Permission"))
            .with_precondition(contradiction);

        let validation = validator.validate_statute(&statute);
        assert!(!validation.is_valid);
        assert!(
            validation
                .errors
                .iter()
                .any(|e| e.contains("contradiction"))
        );
    }

    #[test]
    fn test_validate_batch() {
        let validator = Validator::new();
        let statutes = vec![
            Statute::new(
                "test1",
                "Test 1",
                Effect::new(EffectType::Grant, "Permission"),
            ),
            Statute::new("", "Test 2", Effect::new(EffectType::Grant, "Permission")),
        ];

        let results = validator.validate_batch(&statutes);
        assert_eq!(results.len(), 2);
        assert!(results[0].1.is_valid);
        assert!(!results[1].1.is_valid);
    }

    #[test]
    fn test_are_equivalent() {
        let validator = Validator::new();
        let statute1 = Statute::new(
            "test1",
            "Test 1",
            Effect::new(EffectType::Grant, "Permission"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let statute2 = Statute::new(
            "test2",
            "Test 2",
            Effect::new(EffectType::Grant, "Permission"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        assert!(validator.are_equivalent(&statute1, &statute2));
    }

    #[test]
    fn test_are_not_equivalent() {
        let validator = Validator::new();
        let statute1 = Statute::new(
            "test1",
            "Test 1",
            Effect::new(EffectType::Grant, "Permission"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let statute2 = Statute::new(
            "test2",
            "Test 2",
            Effect::new(EffectType::Prohibition, "Permission"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        assert!(!validator.are_equivalent(&statute1, &statute2));
    }

    #[test]
    fn test_validate_conversion_report() {
        let validator = Validator::new();

        let good_report = ConversionReport::new(crate::LegalFormat::Catala, crate::LegalFormat::L4);
        assert!(validator.validate_conversion_report(&good_report).is_ok());

        let mut poor_report =
            ConversionReport::new(crate::LegalFormat::Catala, crate::LegalFormat::L4);
        poor_report.confidence = 0.3;
        assert!(validator.validate_conversion_report(&poor_report).is_err());
    }
}
