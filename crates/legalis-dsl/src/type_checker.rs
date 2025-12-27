//! Type inference and checking for condition values.
//!
//! This module provides type checking and inference for legal document conditions,
//! ensuring type safety and catching common errors at compile time.

use crate::ast::{ConditionNode, ConditionValue, LegalDocument, StatuteNode};
use std::collections::HashMap;

/// Type information for a value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    /// Numeric type
    Number,
    /// String type
    String,
    /// Boolean type
    Boolean,
    /// Date type
    Date,
    /// Set expression type
    Set(Box<Type>),
    /// Unknown/inferred type
    Unknown,
    /// Union of multiple types
    Union(Vec<Type>),
}

impl Type {
    /// Returns true if this type is compatible with another type.
    pub fn is_compatible_with(&self, other: &Type) -> bool {
        match (self, other) {
            (Type::Unknown, _) | (_, Type::Unknown) => true,
            (Type::Union(types1), Type::Union(types2)) => types1
                .iter()
                .any(|t1| types2.iter().any(|t2| t1.is_compatible_with(t2))),
            (Type::Union(types), other) | (other, Type::Union(types)) => {
                types.iter().any(|t| t.is_compatible_with(other))
            }
            (Type::Set(inner1), Type::Set(inner2)) => inner1.is_compatible_with(inner2),
            (t1, t2) => t1 == t2,
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Number => write!(f, "Number"),
            Type::String => write!(f, "String"),
            Type::Boolean => write!(f, "Boolean"),
            Type::Date => write!(f, "Date"),
            Type::Set(inner) => write!(f, "Set<{}>", inner),
            Type::Unknown => write!(f, "Unknown"),
            Type::Union(types) => {
                write!(f, "(")?;
                for (i, t) in types.iter().enumerate() {
                    if i > 0 {
                        write!(f, " | ")?;
                    }
                    write!(f, "{}", t)?;
                }
                write!(f, ")")
            }
        }
    }
}

/// Type checking error.
#[derive(Debug, Clone, PartialEq)]
pub struct TypeError {
    /// Location description
    pub location: String,
    /// Error message
    pub message: String,
    /// Expected type
    pub expected: Type,
    /// Actual type
    pub actual: Type,
}

impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Type error at {}: {}. Expected {}, found {}",
            self.location, self.message, self.expected, self.actual
        )
    }
}

/// Type inference context.
#[derive(Debug, Clone)]
pub struct TypeContext {
    /// Field types (field_name -> type)
    field_types: HashMap<String, Type>,
    /// Attribute types (attribute_key -> type)
    #[allow(dead_code)]
    attribute_types: HashMap<String, Type>,
}

impl TypeContext {
    /// Creates a new empty type context.
    pub fn new() -> Self {
        Self {
            field_types: HashMap::new(),
            attribute_types: HashMap::new(),
        }
    }

    /// Registers a field with a specific type.
    pub fn register_field(&mut self, field: String, ty: Type) {
        self.field_types.insert(field, ty);
    }

    /// Gets the type of a field.
    pub fn get_field_type(&self, field: &str) -> Type {
        self.field_types
            .get(field)
            .cloned()
            .unwrap_or(Type::Unknown)
    }

    /// Infers the type of a condition value.
    pub fn infer_value_type(&self, value: &ConditionValue) -> Type {
        match value {
            ConditionValue::Number(_) => Type::Number,
            ConditionValue::String(_) => Type::String,
            ConditionValue::Boolean(_) => Type::Boolean,
            ConditionValue::Date(_) => Type::Date,
            ConditionValue::SetExpr(_) => Type::Set(Box::new(Type::Unknown)),
        }
    }

    /// Infers types from a condition and updates the context.
    pub fn infer_from_condition(&mut self, condition: &ConditionNode) {
        match condition {
            ConditionNode::Comparison { field, value, .. } => {
                let value_type = self.infer_value_type(value);
                let existing_type = self.get_field_type(field);

                if existing_type == Type::Unknown {
                    self.register_field(field.clone(), value_type);
                } else if !existing_type.is_compatible_with(&value_type) {
                    // Upgrade to union type
                    self.register_field(
                        field.clone(),
                        Type::Union(vec![existing_type, value_type]),
                    );
                }
            }
            ConditionNode::Between { field, min, max } => {
                let min_type = self.infer_value_type(min);
                let max_type = self.infer_value_type(max);

                // Both bounds should have the same type
                if min_type.is_compatible_with(&max_type) {
                    self.register_field(field.clone(), min_type);
                }
            }
            ConditionNode::In { field, values } => {
                if let Some(first) = values.first() {
                    let elem_type = self.infer_value_type(first);
                    self.register_field(field.clone(), elem_type);
                }
            }
            ConditionNode::InRange {
                field, min, max, ..
            }
            | ConditionNode::NotInRange {
                field, min, max, ..
            } => {
                let min_type = self.infer_value_type(min);
                let max_type = self.infer_value_type(max);

                if min_type.is_compatible_with(&max_type) {
                    self.register_field(field.clone(), min_type);
                }
            }
            ConditionNode::And(left, right) | ConditionNode::Or(left, right) => {
                self.infer_from_condition(left);
                self.infer_from_condition(right);
            }
            ConditionNode::Not(inner) => {
                self.infer_from_condition(inner);
            }
            _ => {}
        }
    }
}

impl Default for TypeContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Type checker for legal documents.
pub struct TypeChecker {
    context: TypeContext,
}

impl TypeChecker {
    /// Creates a new type checker.
    pub fn new() -> Self {
        Self {
            context: TypeContext::new(),
        }
    }

    /// Type checks a legal document.
    pub fn check_document(&mut self, doc: &LegalDocument) -> Vec<TypeError> {
        let mut errors = Vec::new();

        for statute in &doc.statutes {
            errors.extend(self.check_statute(statute));
        }

        errors
    }

    /// Type checks a statute.
    pub fn check_statute(&mut self, statute: &StatuteNode) -> Vec<TypeError> {
        let mut errors = Vec::new();

        // First pass: infer types
        for condition in &statute.conditions {
            self.context.infer_from_condition(condition);
        }

        // Second pass: check types
        for condition in &statute.conditions {
            errors.extend(self.check_condition(condition, &statute.id));
        }

        errors
    }

    /// Type checks a condition.
    fn check_condition(&self, condition: &ConditionNode, statute_id: &str) -> Vec<TypeError> {
        let mut errors = Vec::new();

        match condition {
            ConditionNode::Comparison {
                field,
                operator,
                value,
            } => {
                let field_type = self.context.get_field_type(field);
                let value_type = self.context.infer_value_type(value);

                if !field_type.is_compatible_with(&value_type) {
                    errors.push(TypeError {
                        location: format!("statute {} field {}", statute_id, field),
                        message: format!("Type mismatch in comparison with {}", operator),
                        expected: field_type,
                        actual: value_type,
                    });
                }
            }
            ConditionNode::Between { field, min, max } => {
                let field_type = self.context.get_field_type(field);
                let min_type = self.context.infer_value_type(min);
                let max_type = self.context.infer_value_type(max);

                if !min_type.is_compatible_with(&max_type) {
                    errors.push(TypeError {
                        location: format!("statute {} field {} BETWEEN", statute_id, field),
                        message: "Min and max must have the same type".to_string(),
                        expected: min_type.clone(),
                        actual: max_type,
                    });
                }

                if !field_type.is_compatible_with(&min_type) {
                    errors.push(TypeError {
                        location: format!("statute {} field {} BETWEEN", statute_id, field),
                        message: "Field type incompatible with range bounds".to_string(),
                        expected: field_type,
                        actual: min_type,
                    });
                }
            }
            ConditionNode::In { field, values } => {
                let field_type = self.context.get_field_type(field);

                for (i, value) in values.iter().enumerate() {
                    let value_type = self.context.infer_value_type(value);
                    if !field_type.is_compatible_with(&value_type) {
                        errors.push(TypeError {
                            location: format!(
                                "statute {} field {} IN value {}",
                                statute_id, field, i
                            ),
                            message: "Value type incompatible with field".to_string(),
                            expected: field_type.clone(),
                            actual: value_type,
                        });
                    }
                }
            }
            ConditionNode::And(left, right) | ConditionNode::Or(left, right) => {
                errors.extend(self.check_condition(left, statute_id));
                errors.extend(self.check_condition(right, statute_id));
            }
            ConditionNode::Not(inner) => {
                errors.extend(self.check_condition(inner, statute_id));
            }
            _ => {}
        }

        errors
    }

    /// Returns the inferred type context.
    pub fn context(&self) -> &TypeContext {
        &self.context
    }
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_compatibility() {
        assert!(Type::Number.is_compatible_with(&Type::Number));
        assert!(Type::String.is_compatible_with(&Type::String));
        assert!(!Type::Number.is_compatible_with(&Type::String));
        assert!(Type::Unknown.is_compatible_with(&Type::Number));
        assert!(Type::Number.is_compatible_with(&Type::Unknown));
    }

    #[test]
    fn test_type_inference() {
        let mut ctx = TypeContext::new();

        let condition = ConditionNode::Comparison {
            field: "age".to_string(),
            operator: ">=".to_string(),
            value: ConditionValue::Number(18),
        };

        ctx.infer_from_condition(&condition);

        assert_eq!(ctx.get_field_type("age"), Type::Number);
    }

    #[test]
    fn test_type_checking() {
        // First infer types from conditions
        let mut ctx = TypeContext::new();

        ctx.infer_from_condition(&ConditionNode::Comparison {
            field: "age".to_string(),
            operator: ">=".to_string(),
            value: ConditionValue::Number(18),
        });

        // Verify age is inferred as Number
        assert_eq!(ctx.get_field_type("age"), Type::Number);

        // Now check a mismatched condition
        let mut checker = TypeChecker::new();
        checker
            .context
            .register_field("age".to_string(), Type::Number);

        let errors = checker.check_condition(
            &ConditionNode::Comparison {
                field: "age".to_string(),
                operator: "==".to_string(),
                value: ConditionValue::String("adult".to_string()),
            },
            "test",
        );

        // Should detect type mismatch: age is Number but compared to String
        assert!(!errors.is_empty());
        assert_eq!(errors[0].expected, Type::Number);
        assert_eq!(errors[0].actual, Type::String);
    }
}
