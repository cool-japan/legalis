//! Typed attribute system for LegalEntity.
//!
//! This module provides a type-safe attribute system that replaces the
//! string-based attribute storage with strongly-typed values and validation.

use chrono::{DateTime, NaiveDate, Utc};
use std::collections::HashMap;
use std::fmt;
use thiserror::Error;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "schema")]
use schemars::JsonSchema;

/// Errors that can occur when working with typed attributes.
#[derive(Error, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum AttributeError {
    /// Attribute not found
    #[error("Attribute '{0}' not found")]
    NotFound(String),

    /// Type mismatch when retrieving attribute
    #[error("Type mismatch for attribute '{key}': expected {expected}, found {found}")]
    TypeMismatch {
        key: String,
        expected: String,
        found: String,
    },

    /// Invalid value for attribute
    #[error("Invalid value for attribute '{key}': {reason}")]
    InvalidValue { key: String, reason: String },

    /// Parse error
    #[error("Failed to parse attribute '{key}': {reason}")]
    ParseError { key: String, reason: String },
}

/// A typed attribute value that can hold various legal entity properties.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub enum AttributeValue {
    /// Unsigned 32-bit integer (age, counts, etc.)
    U32(u32),
    /// Unsigned 64-bit integer (income, large amounts)
    U64(u64),
    /// Signed 64-bit integer (for negative values if needed)
    I64(i64),
    /// Boolean flag
    Bool(bool),
    /// Text string
    String(String),
    /// Date value
    Date(NaiveDate),
    /// Floating point number (percentages, rates)
    F64(f64),
    /// List of strings (multiple values)
    StringList(Vec<String>),
}

impl AttributeValue {
    /// Returns the type name of this attribute value.
    pub fn type_name(&self) -> &'static str {
        match self {
            Self::U32(_) => "u32",
            Self::U64(_) => "u64",
            Self::I64(_) => "i64",
            Self::Bool(_) => "bool",
            Self::String(_) => "String",
            Self::Date(_) => "Date",
            Self::F64(_) => "f64",
            Self::StringList(_) => "Vec<String>",
        }
    }

    /// Attempts to extract a u32 value.
    pub fn as_u32(&self) -> Result<u32, AttributeError> {
        match self {
            Self::U32(v) => Ok(*v),
            other => Err(AttributeError::TypeMismatch {
                key: "".to_string(),
                expected: "u32".to_string(),
                found: other.type_name().to_string(),
            }),
        }
    }

    /// Attempts to extract a u64 value.
    pub fn as_u64(&self) -> Result<u64, AttributeError> {
        match self {
            Self::U64(v) => Ok(*v),
            Self::U32(v) => Ok(*v as u64), // Allow upcasting
            other => Err(AttributeError::TypeMismatch {
                key: "".to_string(),
                expected: "u64".to_string(),
                found: other.type_name().to_string(),
            }),
        }
    }

    /// Attempts to extract an i64 value.
    pub fn as_i64(&self) -> Result<i64, AttributeError> {
        match self {
            Self::I64(v) => Ok(*v),
            Self::U32(v) => Ok(*v as i64), // Allow upcasting
            Self::U64(v) if *v <= i64::MAX as u64 => Ok(*v as i64),
            other => Err(AttributeError::TypeMismatch {
                key: "".to_string(),
                expected: "i64".to_string(),
                found: other.type_name().to_string(),
            }),
        }
    }

    /// Attempts to extract a bool value.
    pub fn as_bool(&self) -> Result<bool, AttributeError> {
        match self {
            Self::Bool(v) => Ok(*v),
            other => Err(AttributeError::TypeMismatch {
                key: "".to_string(),
                expected: "bool".to_string(),
                found: other.type_name().to_string(),
            }),
        }
    }

    /// Attempts to extract a string reference.
    pub fn as_string(&self) -> Result<&str, AttributeError> {
        match self {
            Self::String(v) => Ok(v),
            other => Err(AttributeError::TypeMismatch {
                key: "".to_string(),
                expected: "String".to_string(),
                found: other.type_name().to_string(),
            }),
        }
    }

    /// Attempts to extract a date value.
    pub fn as_date(&self) -> Result<NaiveDate, AttributeError> {
        match self {
            Self::Date(v) => Ok(*v),
            other => Err(AttributeError::TypeMismatch {
                key: "".to_string(),
                expected: "Date".to_string(),
                found: other.type_name().to_string(),
            }),
        }
    }

    /// Attempts to extract an f64 value.
    pub fn as_f64(&self) -> Result<f64, AttributeError> {
        match self {
            Self::F64(v) => Ok(*v),
            Self::U32(v) => Ok(*v as f64),
            Self::I64(v) => Ok(*v as f64),
            other => Err(AttributeError::TypeMismatch {
                key: "".to_string(),
                expected: "f64".to_string(),
                found: other.type_name().to_string(),
            }),
        }
    }

    /// Attempts to extract a string list.
    pub fn as_string_list(&self) -> Result<&[String], AttributeError> {
        match self {
            Self::StringList(v) => Ok(v),
            other => Err(AttributeError::TypeMismatch {
                key: "".to_string(),
                expected: "Vec<String>".to_string(),
                found: other.type_name().to_string(),
            }),
        }
    }

    /// Parses a string into an AttributeValue based on heuristics.
    ///
    /// This is for backward compatibility with the string-based system.
    pub fn parse_from_string(s: &str) -> Self {
        // Try bool
        if s.eq_ignore_ascii_case("true") {
            return Self::Bool(true);
        }
        if s.eq_ignore_ascii_case("false") {
            return Self::Bool(false);
        }

        // Try u32
        if let Ok(v) = s.parse::<u32>() {
            return Self::U32(v);
        }

        // Try u64
        if let Ok(v) = s.parse::<u64>() {
            return Self::U64(v);
        }

        // Try i64
        if let Ok(v) = s.parse::<i64>() {
            return Self::I64(v);
        }

        // Try f64
        if let Ok(v) = s.parse::<f64>() {
            return Self::F64(v);
        }

        // Try date (YYYY-MM-DD format)
        if let Ok(v) = NaiveDate::parse_from_str(s, "%Y-%m-%d") {
            return Self::Date(v);
        }

        // Default to string
        Self::String(s.to_string())
    }

    /// Converts the attribute value to a string representation.
    pub fn to_string_value(&self) -> String {
        match self {
            Self::U32(v) => v.to_string(),
            Self::U64(v) => v.to_string(),
            Self::I64(v) => v.to_string(),
            Self::Bool(v) => v.to_string(),
            Self::String(v) => v.clone(),
            Self::Date(v) => v.format("%Y-%m-%d").to_string(),
            Self::F64(v) => v.to_string(),
            Self::StringList(v) => v.join(","),
        }
    }
}

impl fmt::Display for AttributeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string_value())
    }
}

/// Type-safe attribute storage for legal entities.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
pub struct TypedAttributes {
    /// Internal storage of typed attributes
    attributes: HashMap<String, AttributeValue>,
}

impl TypedAttributes {
    /// Creates a new empty TypedAttributes.
    pub fn new() -> Self {
        Self {
            attributes: HashMap::new(),
        }
    }

    /// Sets an attribute with a typed value.
    pub fn set(&mut self, key: impl Into<String>, value: AttributeValue) {
        self.attributes.insert(key.into(), value);
    }

    /// Sets a u32 attribute.
    pub fn set_u32(&mut self, key: impl Into<String>, value: u32) {
        self.set(key, AttributeValue::U32(value));
    }

    /// Sets a u64 attribute.
    pub fn set_u64(&mut self, key: impl Into<String>, value: u64) {
        self.set(key, AttributeValue::U64(value));
    }

    /// Sets a boolean attribute.
    pub fn set_bool(&mut self, key: impl Into<String>, value: bool) {
        self.set(key, AttributeValue::Bool(value));
    }

    /// Sets a string attribute.
    pub fn set_string(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.set(key, AttributeValue::String(value.into()));
    }

    /// Sets a date attribute.
    pub fn set_date(&mut self, key: impl Into<String>, value: NaiveDate) {
        self.set(key, AttributeValue::Date(value));
    }

    /// Sets an f64 attribute.
    pub fn set_f64(&mut self, key: impl Into<String>, value: f64) {
        self.set(key, AttributeValue::F64(value));
    }

    /// Gets an attribute value.
    pub fn get(&self, key: &str) -> Option<&AttributeValue> {
        self.attributes.get(key)
    }

    /// Gets a u32 attribute.
    pub fn get_u32(&self, key: &str) -> Result<u32, AttributeError> {
        self.get(key)
            .ok_or_else(|| AttributeError::NotFound(key.to_string()))?
            .as_u32()
            .map_err(|e| match e {
                AttributeError::TypeMismatch {
                    expected, found, ..
                } => AttributeError::TypeMismatch {
                    key: key.to_string(),
                    expected,
                    found,
                },
                other => other,
            })
    }

    /// Gets a u64 attribute.
    pub fn get_u64(&self, key: &str) -> Result<u64, AttributeError> {
        self.get(key)
            .ok_or_else(|| AttributeError::NotFound(key.to_string()))?
            .as_u64()
            .map_err(|e| match e {
                AttributeError::TypeMismatch {
                    expected, found, ..
                } => AttributeError::TypeMismatch {
                    key: key.to_string(),
                    expected,
                    found,
                },
                other => other,
            })
    }

    /// Gets a boolean attribute.
    pub fn get_bool(&self, key: &str) -> Result<bool, AttributeError> {
        self.get(key)
            .ok_or_else(|| AttributeError::NotFound(key.to_string()))?
            .as_bool()
            .map_err(|e| match e {
                AttributeError::TypeMismatch {
                    expected, found, ..
                } => AttributeError::TypeMismatch {
                    key: key.to_string(),
                    expected,
                    found,
                },
                other => other,
            })
    }

    /// Gets a string attribute.
    pub fn get_string(&self, key: &str) -> Result<&str, AttributeError> {
        self.get(key)
            .ok_or_else(|| AttributeError::NotFound(key.to_string()))?
            .as_string()
            .map_err(|e| match e {
                AttributeError::TypeMismatch {
                    expected, found, ..
                } => AttributeError::TypeMismatch {
                    key: key.to_string(),
                    expected,
                    found,
                },
                other => other,
            })
    }

    /// Gets a date attribute.
    pub fn get_date(&self, key: &str) -> Result<NaiveDate, AttributeError> {
        self.get(key)
            .ok_or_else(|| AttributeError::NotFound(key.to_string()))?
            .as_date()
            .map_err(|e| match e {
                AttributeError::TypeMismatch {
                    expected, found, ..
                } => AttributeError::TypeMismatch {
                    key: key.to_string(),
                    expected,
                    found,
                },
                other => other,
            })
    }

    /// Gets an f64 attribute.
    pub fn get_f64(&self, key: &str) -> Result<f64, AttributeError> {
        self.get(key)
            .ok_or_else(|| AttributeError::NotFound(key.to_string()))?
            .as_f64()
            .map_err(|e| match e {
                AttributeError::TypeMismatch {
                    expected, found, ..
                } => AttributeError::TypeMismatch {
                    key: key.to_string(),
                    expected,
                    found,
                },
                other => other,
            })
    }

    /// Checks if an attribute exists.
    pub fn has(&self, key: &str) -> bool {
        self.attributes.contains_key(key)
    }

    /// Removes an attribute.
    pub fn remove(&mut self, key: &str) -> Option<AttributeValue> {
        self.attributes.remove(key)
    }

    /// Returns the number of attributes.
    pub fn len(&self) -> usize {
        self.attributes.len()
    }

    /// Returns true if there are no attributes.
    pub fn is_empty(&self) -> bool {
        self.attributes.is_empty()
    }

    /// Returns an iterator over attribute keys.
    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.attributes.keys()
    }

    /// Returns an iterator over (key, value) pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &AttributeValue)> {
        self.attributes.iter()
    }

    /// Clears all attributes.
    pub fn clear(&mut self) {
        self.attributes.clear();
    }

    /// Creates TypedAttributes from a string-based HashMap (for backward compatibility).
    pub fn from_string_map(map: HashMap<String, String>) -> Self {
        let attributes = map
            .into_iter()
            .map(|(k, v)| (k, AttributeValue::parse_from_string(&v)))
            .collect();
        Self { attributes }
    }

    /// Converts to a string-based HashMap (for backward compatibility).
    pub fn to_string_map(&self) -> HashMap<String, String> {
        self.attributes
            .iter()
            .map(|(k, v)| (k.clone(), v.to_string_value()))
            .collect()
    }
}

/// Type alias for custom validation functions.
pub type ValidatorFn = fn(&AttributeValue) -> Result<(), String>;

/// Validation rule for attribute values.
///
/// These rules can be applied to validate attribute values before they are set
/// or after they are retrieved.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ValidationRule {
    /// Value must be within a range (inclusive)
    RangeU32 { min: u32, max: u32 },
    /// Value must be within a range (inclusive)
    RangeU64 { min: u64, max: u64 },
    /// Value must be within a range (inclusive)
    RangeI64 { min: i64, max: i64 },
    /// Value must be within a range (inclusive)
    RangeF64 { min: f64, max: f64 },
    /// Value must match a regex pattern
    Regex { pattern: String },
    /// Value must be one of the allowed values
    OneOf { values: Vec<String> },
    /// Value must not be empty (for strings)
    NotEmpty,
    /// Custom validation function (not serializable)
    #[cfg_attr(feature = "serde", serde(skip))]
    Custom {
        name: String,
        #[cfg_attr(feature = "serde", serde(skip))]
        validator: Option<ValidatorFn>,
    },
}

impl ValidationRule {
    /// Validates an attribute value against this rule.
    pub fn validate(&self, value: &AttributeValue) -> Result<(), AttributeError> {
        match self {
            Self::RangeU32 { min, max } => {
                let v = value.as_u32().map_err(|_| AttributeError::InvalidValue {
                    key: "".to_string(),
                    reason: "Expected u32 value".to_string(),
                })?;
                if v < *min || v > *max {
                    return Err(AttributeError::InvalidValue {
                        key: "".to_string(),
                        reason: format!("Value {} out of range [{}, {}]", v, min, max),
                    });
                }
                Ok(())
            }
            Self::RangeU64 { min, max } => {
                let v = value.as_u64().map_err(|_| AttributeError::InvalidValue {
                    key: "".to_string(),
                    reason: "Expected u64 value".to_string(),
                })?;
                if v < *min || v > *max {
                    return Err(AttributeError::InvalidValue {
                        key: "".to_string(),
                        reason: format!("Value {} out of range [{}, {}]", v, min, max),
                    });
                }
                Ok(())
            }
            Self::RangeI64 { min, max } => {
                let v = value.as_i64().map_err(|_| AttributeError::InvalidValue {
                    key: "".to_string(),
                    reason: "Expected i64 value".to_string(),
                })?;
                if v < *min || v > *max {
                    return Err(AttributeError::InvalidValue {
                        key: "".to_string(),
                        reason: format!("Value {} out of range [{}, {}]", v, min, max),
                    });
                }
                Ok(())
            }
            Self::RangeF64 { min, max } => {
                let v = value.as_f64().map_err(|_| AttributeError::InvalidValue {
                    key: "".to_string(),
                    reason: "Expected f64 value".to_string(),
                })?;
                if v < *min || v > *max {
                    return Err(AttributeError::InvalidValue {
                        key: "".to_string(),
                        reason: format!("Value {} out of range [{}, {}]", v, min, max),
                    });
                }
                Ok(())
            }
            Self::Regex { pattern } => {
                let v = value
                    .as_string()
                    .map_err(|_| AttributeError::InvalidValue {
                        key: "".to_string(),
                        reason: "Expected string value".to_string(),
                    })?;
                let re = regex::Regex::new(pattern).map_err(|e| AttributeError::InvalidValue {
                    key: "".to_string(),
                    reason: format!("Invalid regex pattern: {}", e),
                })?;
                if !re.is_match(v) {
                    return Err(AttributeError::InvalidValue {
                        key: "".to_string(),
                        reason: format!("Value '{}' does not match pattern '{}'", v, pattern),
                    });
                }
                Ok(())
            }
            Self::OneOf { values } => {
                let v = value.to_string_value();
                if !values.contains(&v) {
                    return Err(AttributeError::InvalidValue {
                        key: "".to_string(),
                        reason: format!("Value '{}' not in allowed values: {:?}", v, values),
                    });
                }
                Ok(())
            }
            Self::NotEmpty => {
                let v = value
                    .as_string()
                    .map_err(|_| AttributeError::InvalidValue {
                        key: "".to_string(),
                        reason: "Expected string value".to_string(),
                    })?;
                if v.is_empty() {
                    return Err(AttributeError::InvalidValue {
                        key: "".to_string(),
                        reason: "Value cannot be empty".to_string(),
                    });
                }
                Ok(())
            }
            Self::Custom { name, validator } => {
                if let Some(validator_fn) = validator {
                    validator_fn(value).map_err(|reason| AttributeError::InvalidValue {
                        key: "".to_string(),
                        reason: format!("{}: {}", name, reason),
                    })
                } else {
                    Ok(())
                }
            }
        }
    }
}

/// Attribute validator that stores validation rules for multiple attributes.
#[derive(Debug, Clone, Default)]
pub struct AttributeValidator {
    rules: HashMap<String, Vec<ValidationRule>>,
}

impl AttributeValidator {
    /// Creates a new empty validator.
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }

    /// Adds a validation rule for an attribute.
    pub fn add_rule(&mut self, key: impl Into<String>, rule: ValidationRule) {
        self.rules.entry(key.into()).or_default().push(rule);
    }

    /// Validates a single attribute value.
    pub fn validate(&self, key: &str, value: &AttributeValue) -> Result<(), AttributeError> {
        if let Some(rules) = self.rules.get(key) {
            for rule in rules {
                rule.validate(value).map_err(|e| match e {
                    AttributeError::InvalidValue { reason, .. } => AttributeError::InvalidValue {
                        key: key.to_string(),
                        reason,
                    },
                    other => other,
                })?;
            }
        }
        Ok(())
    }

    /// Validates all attributes in a TypedAttributes.
    pub fn validate_all(&self, attrs: &TypedAttributes) -> Result<(), AttributeError> {
        for (key, value) in attrs.iter() {
            self.validate(key, value)?;
        }
        Ok(())
    }

    /// Removes all rules for an attribute.
    pub fn remove_rules(&mut self, key: &str) {
        self.rules.remove(key);
    }

    /// Clears all validation rules.
    pub fn clear(&mut self) {
        self.rules.clear();
    }

    /// Returns true if there are any rules for the given attribute.
    pub fn has_rules(&self, key: &str) -> bool {
        self.rules.contains_key(key)
    }
}

/// A single change record for an attribute.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AttributeChange {
    /// The attribute key that was changed
    pub key: String,
    /// The old value (None if attribute was newly created)
    pub old_value: Option<AttributeValue>,
    /// The new value (None if attribute was deleted)
    pub new_value: Option<AttributeValue>,
    /// Timestamp of the change
    pub timestamp: DateTime<Utc>,
    /// Optional description or reason for the change
    pub reason: Option<String>,
}

impl AttributeChange {
    /// Creates a new attribute change record.
    pub fn new(
        key: String,
        old_value: Option<AttributeValue>,
        new_value: Option<AttributeValue>,
    ) -> Self {
        Self {
            key,
            old_value,
            new_value,
            timestamp: Utc::now(),
            reason: None,
        }
    }

    /// Creates a new attribute change record with a reason.
    pub fn with_reason(
        key: String,
        old_value: Option<AttributeValue>,
        new_value: Option<AttributeValue>,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            key,
            old_value,
            new_value,
            timestamp: Utc::now(),
            reason: Some(reason.into()),
        }
    }

    /// Returns true if this change represents a creation (no old value).
    pub fn is_creation(&self) -> bool {
        self.old_value.is_none() && self.new_value.is_some()
    }

    /// Returns true if this change represents a deletion (no new value).
    pub fn is_deletion(&self) -> bool {
        self.old_value.is_some() && self.new_value.is_none()
    }

    /// Returns true if this change represents a modification.
    pub fn is_modification(&self) -> bool {
        self.old_value.is_some() && self.new_value.is_some()
    }
}

/// Attribute change history tracker.
///
/// This structure tracks all changes to attributes over time, providing
/// an audit trail for legal compliance and debugging.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AttributeHistory {
    /// All changes, stored in chronological order
    changes: Vec<AttributeChange>,
    /// Maximum number of changes to keep (None = unlimited)
    max_history: Option<usize>,
}

impl AttributeHistory {
    /// Creates a new empty attribute history.
    pub fn new() -> Self {
        Self {
            changes: Vec::new(),
            max_history: None,
        }
    }

    /// Creates a new attribute history with a maximum size limit.
    pub fn with_max_size(max_size: usize) -> Self {
        Self {
            changes: Vec::new(),
            max_history: Some(max_size),
        }
    }

    /// Records a new attribute change.
    pub fn record(&mut self, change: AttributeChange) {
        self.changes.push(change);

        // Trim history if needed
        if let Some(max) = self.max_history
            && self.changes.len() > max
        {
            self.changes.drain(0..(self.changes.len() - max));
        }
    }

    /// Returns all changes for a specific attribute.
    pub fn get_changes(&self, key: &str) -> Vec<&AttributeChange> {
        self.changes
            .iter()
            .filter(|change| change.key == key)
            .collect()
    }

    /// Returns the last N changes for a specific attribute.
    pub fn get_recent_changes(&self, key: &str, n: usize) -> Vec<&AttributeChange> {
        self.changes
            .iter()
            .filter(|change| change.key == key)
            .rev()
            .take(n)
            .collect()
    }

    /// Returns all changes in chronological order.
    pub fn all_changes(&self) -> &[AttributeChange] {
        &self.changes
    }

    /// Returns the total number of recorded changes.
    pub fn len(&self) -> usize {
        self.changes.len()
    }

    /// Returns true if no changes have been recorded.
    pub fn is_empty(&self) -> bool {
        self.changes.is_empty()
    }

    /// Clears all change history.
    pub fn clear(&mut self) {
        self.changes.clear();
    }

    /// Returns the last change for a specific attribute.
    pub fn last_change(&self, key: &str) -> Option<&AttributeChange> {
        self.changes.iter().rev().find(|change| change.key == key)
    }

    /// Returns all attribute keys that have been modified.
    pub fn modified_keys(&self) -> Vec<String> {
        let mut keys: Vec<String> = self
            .changes
            .iter()
            .map(|change| change.key.clone())
            .collect();
        keys.sort();
        keys.dedup();
        keys
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attribute_value_u32() {
        let val = AttributeValue::U32(25);
        assert_eq!(val.as_u32().unwrap(), 25);
        assert_eq!(val.as_u64().unwrap(), 25); // Allows upcasting
        assert_eq!(val.type_name(), "u32");
        assert_eq!(val.to_string_value(), "25");
    }

    #[test]
    fn test_attribute_value_bool() {
        let val = AttributeValue::Bool(true);
        assert!(val.as_bool().unwrap());
        assert_eq!(val.type_name(), "bool");
        assert_eq!(val.to_string_value(), "true");
    }

    #[test]
    fn test_attribute_value_string() {
        let val = AttributeValue::String("test".to_string());
        assert_eq!(val.as_string().unwrap(), "test");
        assert_eq!(val.type_name(), "String");
    }

    #[test]
    fn test_attribute_value_date() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
        let val = AttributeValue::Date(date);
        assert_eq!(val.as_date().unwrap(), date);
        assert_eq!(val.to_string_value(), "2024-01-15");
    }

    #[test]
    fn test_attribute_value_type_mismatch() {
        let val = AttributeValue::String("test".to_string());
        assert!(val.as_u32().is_err());
        assert!(val.as_bool().is_err());
    }

    #[test]
    fn test_parse_from_string() {
        assert_eq!(
            AttributeValue::parse_from_string("true"),
            AttributeValue::Bool(true)
        );
        assert_eq!(
            AttributeValue::parse_from_string("false"),
            AttributeValue::Bool(false)
        );
        assert_eq!(
            AttributeValue::parse_from_string("42"),
            AttributeValue::U32(42)
        );
        assert_eq!(
            AttributeValue::parse_from_string("12345678901"),
            AttributeValue::U64(12345678901)
        );
        assert_eq!(
            AttributeValue::parse_from_string("-123"),
            AttributeValue::I64(-123)
        );
        assert_eq!(
            AttributeValue::parse_from_string("2.71"),
            AttributeValue::F64(2.71)
        );

        let date = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
        assert_eq!(
            AttributeValue::parse_from_string("2024-01-15"),
            AttributeValue::Date(date)
        );

        assert_eq!(
            AttributeValue::parse_from_string("hello"),
            AttributeValue::String("hello".to_string())
        );
    }

    #[test]
    fn test_typed_attributes_set_get() {
        let mut attrs = TypedAttributes::new();

        attrs.set_u32("age", 25);
        attrs.set_bool("is_citizen", true);
        attrs.set_string("name", "Alice");

        assert_eq!(attrs.get_u32("age").unwrap(), 25);
        assert!(attrs.get_bool("is_citizen").unwrap());
        assert_eq!(attrs.get_string("name").unwrap(), "Alice");
    }

    #[test]
    fn test_typed_attributes_not_found() {
        let attrs = TypedAttributes::new();
        assert!(matches!(
            attrs.get_u32("nonexistent"),
            Err(AttributeError::NotFound(_))
        ));
    }

    #[test]
    fn test_typed_attributes_type_mismatch() {
        let mut attrs = TypedAttributes::new();
        attrs.set_string("name", "Alice");

        assert!(matches!(
            attrs.get_u32("name"),
            Err(AttributeError::TypeMismatch { .. })
        ));
    }

    #[test]
    fn test_from_string_map() {
        let mut map = HashMap::new();
        map.insert("age".to_string(), "25".to_string());
        map.insert("is_citizen".to_string(), "true".to_string());
        map.insert("name".to_string(), "Alice".to_string());

        let attrs = TypedAttributes::from_string_map(map);

        assert_eq!(attrs.get_u32("age").unwrap(), 25);
        assert!(attrs.get_bool("is_citizen").unwrap());
        assert_eq!(attrs.get_string("name").unwrap(), "Alice");
    }

    #[test]
    fn test_to_string_map() {
        let mut attrs = TypedAttributes::new();
        attrs.set_u32("age", 25);
        attrs.set_bool("is_citizen", true);

        let map = attrs.to_string_map();

        assert_eq!(map.get("age").unwrap(), "25");
        assert_eq!(map.get("is_citizen").unwrap(), "true");
    }

    #[test]
    fn test_attribute_operations() {
        let mut attrs = TypedAttributes::new();

        assert!(attrs.is_empty());
        assert_eq!(attrs.len(), 0);

        attrs.set_u32("age", 25);
        assert!(!attrs.is_empty());
        assert_eq!(attrs.len(), 1);
        assert!(attrs.has("age"));

        attrs.remove("age");
        assert!(attrs.is_empty());
        assert!(!attrs.has("age"));
    }
}
