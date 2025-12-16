//! Parser utilities for the legal DSL.

use crate::DslResult;
use crate::ast::*;

/// Trait for converting AST nodes to core types.
pub trait ToCore {
    type Output;
    fn to_core(&self) -> DslResult<Self::Output>;
}

impl ToCore for ConditionNode {
    type Output = legalis_core::Condition;

    fn to_core(&self) -> DslResult<Self::Output> {
        match self {
            ConditionNode::Comparison {
                field,
                operator,
                value,
            } => {
                let op = match operator.as_str() {
                    ">=" => legalis_core::ComparisonOp::GreaterOrEqual,
                    "<=" => legalis_core::ComparisonOp::LessOrEqual,
                    ">" => legalis_core::ComparisonOp::GreaterThan,
                    "<" => legalis_core::ComparisonOp::LessThan,
                    "==" | "=" => legalis_core::ComparisonOp::Equal,
                    "!=" => legalis_core::ComparisonOp::NotEqual,
                    _ => legalis_core::ComparisonOp::Equal,
                };

                match field.to_lowercase().as_str() {
                    "age" => {
                        if let ConditionValue::Number(n) = value {
                            Ok(legalis_core::Condition::Age {
                                operator: op,
                                value: *n as u32,
                            })
                        } else {
                            Ok(legalis_core::Condition::Custom {
                                description: "Age condition with non-numeric value".to_string(),
                            })
                        }
                    }
                    "income" => {
                        if let ConditionValue::Number(n) = value {
                            Ok(legalis_core::Condition::Income {
                                operator: op,
                                value: *n as u64,
                            })
                        } else {
                            Ok(legalis_core::Condition::Custom {
                                description: "Income condition with non-numeric value".to_string(),
                            })
                        }
                    }
                    _ => Ok(legalis_core::Condition::Custom {
                        description: format!("{field} {operator} {:?}", value),
                    }),
                }
            }
            ConditionNode::HasAttribute { key } => {
                Ok(legalis_core::Condition::HasAttribute { key: key.clone() })
            }
            ConditionNode::Between { field, min, max } => Ok(legalis_core::Condition::Custom {
                description: format!("{} BETWEEN {:?} AND {:?}", field, min, max),
            }),
            ConditionNode::In { field, values } => Ok(legalis_core::Condition::Custom {
                description: format!("{} IN {:?}", field, values),
            }),
            ConditionNode::Like { field, pattern } => Ok(legalis_core::Condition::Custom {
                description: format!("{} LIKE {}", field, pattern),
            }),
            ConditionNode::Matches {
                field,
                regex_pattern,
            } => Ok(legalis_core::Condition::Custom {
                description: format!("{} MATCHES {}", field, regex_pattern),
            }),
            ConditionNode::InRange {
                field,
                min,
                max,
                inclusive_min,
                inclusive_max,
            } => {
                let min_bracket = if *inclusive_min { "[" } else { "(" };
                let max_bracket = if *inclusive_max { "]" } else { ")" };
                Ok(legalis_core::Condition::Custom {
                    description: format!(
                        "{} IN_RANGE {}{:?}..{:?}{}",
                        field, min_bracket, min, max, max_bracket
                    ),
                })
            }
            ConditionNode::NotInRange {
                field,
                min,
                max,
                inclusive_min,
                inclusive_max,
            } => {
                let min_bracket = if *inclusive_min { "[" } else { "(" };
                let max_bracket = if *inclusive_max { "]" } else { ")" };
                Ok(legalis_core::Condition::Custom {
                    description: format!(
                        "{} NOT_IN_RANGE {}{:?}..{:?}{}",
                        field, min_bracket, min, max, max_bracket
                    ),
                })
            }
            ConditionNode::TemporalComparison {
                field,
                operator,
                value,
            } => {
                let field_desc = match field {
                    TemporalField::CurrentDate => "CURRENT_DATE".to_string(),
                    TemporalField::DateField(name) => format!("DATE_FIELD({})", name),
                };
                Ok(legalis_core::Condition::Custom {
                    description: format!("{} {} {:?}", field_desc, operator, value),
                })
            }
            ConditionNode::And(left, right) => Ok(legalis_core::Condition::And(
                Box::new(left.to_core()?),
                Box::new(right.to_core()?),
            )),
            ConditionNode::Or(left, right) => Ok(legalis_core::Condition::Or(
                Box::new(left.to_core()?),
                Box::new(right.to_core()?),
            )),
            ConditionNode::Not(inner) => {
                Ok(legalis_core::Condition::Not(Box::new(inner.to_core()?)))
            }
        }
    }
}

impl ToCore for EffectNode {
    type Output = legalis_core::Effect;

    fn to_core(&self) -> DslResult<Self::Output> {
        let effect_type = match self.effect_type.to_lowercase().as_str() {
            "grant" => legalis_core::EffectType::Grant,
            "revoke" => legalis_core::EffectType::Revoke,
            "obligation" => legalis_core::EffectType::Obligation,
            "prohibition" => legalis_core::EffectType::Prohibition,
            "monetary" | "transfer" => legalis_core::EffectType::MonetaryTransfer,
            "status" => legalis_core::EffectType::StatusChange,
            _ => legalis_core::EffectType::Custom,
        };

        let mut effect = legalis_core::Effect::new(effect_type, &self.description);
        for (key, value) in &self.parameters {
            effect = effect.with_parameter(key, value);
        }
        Ok(effect)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_condition_to_core() {
        let node = ConditionNode::Comparison {
            field: "age".to_string(),
            operator: ">=".to_string(),
            value: ConditionValue::Number(18),
        };

        let condition = node.to_core().unwrap();
        match condition {
            legalis_core::Condition::Age { operator, value } => {
                assert_eq!(operator, legalis_core::ComparisonOp::GreaterOrEqual);
                assert_eq!(value, 18);
            }
            _ => panic!("Expected Age condition"),
        }
    }

    #[test]
    fn test_effect_to_core() {
        let node = EffectNode {
            effect_type: "grant".to_string(),
            description: "Test permission".to_string(),
            parameters: vec![("scope".to_string(), "full".to_string())],
        };

        let effect = node.to_core().unwrap();
        assert_eq!(effect.effect_type, legalis_core::EffectType::Grant);
        assert_eq!(effect.parameters.get("scope"), Some(&"full".to_string()));
    }
}
