//! Legal Practice / Workflow Automation.
//!
//! A self-contained, pure-Rust toolkit for the day-to-day automation tasks of a
//! legal practice. Every capability in this module works fully offline with no
//! LLM call: documents are assembled from a real template engine (variable
//! substitution, conditional sections, loops and a reusable clause library),
//! contracts are generated from those templates, due-diligence is tracked with
//! configurable checklists and gap detection, intake forms are described by a
//! typed field schema and validated, deadlines are computed with business-day
//! awareness and turned into reminder schedules, tasks are prioritised with a
//! weighted urgency/importance/dependency model, and matters move through a
//! guarded workflow state machine with full transition history.
//!
//! Where a [`crate::LLMProvider`] is available the assembled draft can be
//! *optionally* polished (see [`assembly::DocumentAssembler::augment`]), but the
//! suite is fully functional without any provider.
//!
//! ## Sub-modules
//!
//! * [`assembly`] - document assembly + contract generation template engine.
//! * [`due_diligence`] - configurable due-diligence checklists & gap detection.
//! * [`forms`] - typed legal form schema, mapping and validation.
//! * [`deadlines`] - business-day deadline tracking and reminder scheduling.
//! * [`prioritization`] - weighted task prioritisation with dependencies.
//! * [`workflow`] - guarded workflow state machine with history.

mod assembly;
mod deadlines;
mod due_diligence;
mod forms;
mod prioritization;
mod workflow;

pub use assembly::*;
pub use deadlines::*;
pub use due_diligence::*;
pub use forms::*;
pub use prioritization::*;
pub use workflow::*;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

// ============================================================================
// Shared value model
// ============================================================================

/// The static type of a practice data field.
///
/// Used both by document-assembly variable declarations and by form field
/// specifications so that the same [`FieldValue`] payloads can be validated and
/// rendered consistently across the suite.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FieldKind {
    /// Free text.
    Text,
    /// Whole number.
    Integer,
    /// Fractional number.
    Decimal,
    /// Boolean flag.
    Boolean,
    /// Calendar date (no time component).
    Date,
    /// An ordered list of values.
    List,
}

impl FieldKind {
    /// Returns a human-readable label.
    pub fn label(&self) -> &'static str {
        match self {
            FieldKind::Text => "text",
            FieldKind::Integer => "integer",
            FieldKind::Decimal => "decimal",
            FieldKind::Boolean => "boolean",
            FieldKind::Date => "date",
            FieldKind::List => "list",
        }
    }
}

/// A typed value used throughout the practice suite.
///
/// `FieldValue` is the common currency between the template engine, form
/// filling, workflow guards and (indirectly) the rest of the module. It can be
/// rendered to a string for document output, tested for truthiness in template
/// conditionals/guards, and parsed from raw strings coming from external data
/// sources.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FieldValue {
    /// Free text.
    Text(String),
    /// Whole number.
    Integer(i64),
    /// Fractional number.
    Decimal(f64),
    /// Boolean flag.
    Boolean(bool),
    /// Calendar date.
    Date(NaiveDate),
    /// An ordered list of values.
    List(Vec<FieldValue>),
}

impl FieldValue {
    /// Creates a text value.
    pub fn text(value: impl Into<String>) -> Self {
        FieldValue::Text(value.into())
    }

    /// Creates an integer value.
    pub fn integer(value: i64) -> Self {
        FieldValue::Integer(value)
    }

    /// Creates a decimal value.
    pub fn decimal(value: f64) -> Self {
        FieldValue::Decimal(value)
    }

    /// Creates a boolean value.
    pub fn boolean(value: bool) -> Self {
        FieldValue::Boolean(value)
    }

    /// Creates a date value.
    pub fn date(value: NaiveDate) -> Self {
        FieldValue::Date(value)
    }

    /// Creates a list value from an iterator of values.
    pub fn list<I: IntoIterator<Item = FieldValue>>(values: I) -> Self {
        FieldValue::List(values.into_iter().collect())
    }

    /// Returns the static kind of this value.
    pub fn kind(&self) -> FieldKind {
        match self {
            FieldValue::Text(_) => FieldKind::Text,
            FieldValue::Integer(_) => FieldKind::Integer,
            FieldValue::Decimal(_) => FieldKind::Decimal,
            FieldValue::Boolean(_) => FieldKind::Boolean,
            FieldValue::Date(_) => FieldKind::Date,
            FieldValue::List(_) => FieldKind::List,
        }
    }

    /// Renders the value to a display string suitable for document output.
    ///
    /// Booleans render as `Yes`/`No`, dates as ISO-8601 (`YYYY-MM-DD`) and lists
    /// as a comma-separated rendering of their items.
    pub fn render(&self) -> String {
        match self {
            FieldValue::Text(value) => value.clone(),
            FieldValue::Integer(value) => value.to_string(),
            FieldValue::Decimal(value) => value.to_string(),
            FieldValue::Boolean(value) => {
                if *value {
                    "Yes".to_string()
                } else {
                    "No".to_string()
                }
            }
            FieldValue::Date(value) => value.format("%Y-%m-%d").to_string(),
            FieldValue::List(values) => values
                .iter()
                .map(FieldValue::render)
                .collect::<Vec<_>>()
                .join(", "),
        }
    }

    /// Returns whether the value is "truthy" for conditional rendering / guards.
    ///
    /// Empty text, zero numbers and empty lists are falsy; everything else
    /// (including any date) is truthy.
    pub fn is_truthy(&self) -> bool {
        match self {
            FieldValue::Text(value) => !value.trim().is_empty(),
            FieldValue::Integer(value) => *value != 0,
            FieldValue::Decimal(value) => value.abs() > f64::EPSILON,
            FieldValue::Boolean(value) => *value,
            FieldValue::Date(_) => true,
            FieldValue::List(values) => !values.is_empty(),
        }
    }

    /// Returns the text payload, if this is a [`FieldValue::Text`].
    pub fn as_text(&self) -> Option<&str> {
        match self {
            FieldValue::Text(value) => Some(value.as_str()),
            _ => None,
        }
    }

    /// Returns the integer payload, if this is a [`FieldValue::Integer`].
    pub fn as_integer(&self) -> Option<i64> {
        match self {
            FieldValue::Integer(value) => Some(*value),
            _ => None,
        }
    }

    /// Returns the list payload, if this is a [`FieldValue::List`].
    pub fn as_list(&self) -> Option<&[FieldValue]> {
        match self {
            FieldValue::List(values) => Some(values.as_slice()),
            _ => None,
        }
    }

    /// Returns the value coerced to a number, when it is numeric.
    ///
    /// Integers and decimals convert directly; booleans map to `1.0`/`0.0`.
    pub fn as_number(&self) -> Option<f64> {
        match self {
            FieldValue::Integer(value) => Some(*value as f64),
            FieldValue::Decimal(value) => Some(*value),
            FieldValue::Boolean(value) => Some(if *value { 1.0 } else { 0.0 }),
            _ => None,
        }
    }

    /// Converts the value to a "natural" JSON value (as opposed to the tagged
    /// representation produced by the `serde` derive).
    ///
    /// This is what form filling uses to export a filled instance for downstream
    /// systems: text and dates become JSON strings, numbers become JSON numbers,
    /// booleans become JSON booleans and lists become JSON arrays.
    pub fn to_json(&self) -> serde_json::Value {
        match self {
            FieldValue::Text(value) => serde_json::Value::String(value.clone()),
            FieldValue::Integer(value) => serde_json::Value::from(*value),
            FieldValue::Decimal(value) => serde_json::json!(*value),
            FieldValue::Boolean(value) => serde_json::Value::Bool(*value),
            FieldValue::Date(value) => {
                serde_json::Value::String(value.format("%Y-%m-%d").to_string())
            }
            FieldValue::List(values) => {
                serde_json::Value::Array(values.iter().map(FieldValue::to_json).collect())
            }
        }
    }

    /// Parses a raw string into a typed value of the requested kind.
    ///
    /// This is the bridge used by form filling to ingest data from external,
    /// stringly-typed sources (CSV exports, query parameters, CRM records).
    pub fn parse_as(kind: FieldKind, raw: &str) -> Result<FieldValue, FieldParseError> {
        let trimmed = raw.trim();
        match kind {
            FieldKind::Text => Ok(FieldValue::Text(raw.to_string())),
            FieldKind::Integer => trimmed
                .parse::<i64>()
                .map(FieldValue::Integer)
                .map_err(|_| FieldParseError::new(kind, raw)),
            FieldKind::Decimal => trimmed
                .parse::<f64>()
                .map(FieldValue::Decimal)
                .map_err(|_| FieldParseError::new(kind, raw)),
            FieldKind::Boolean => match trimmed.to_ascii_lowercase().as_str() {
                "true" | "yes" | "y" | "1" | "on" => Ok(FieldValue::Boolean(true)),
                "false" | "no" | "n" | "0" | "off" => Ok(FieldValue::Boolean(false)),
                _ => Err(FieldParseError::new(kind, raw)),
            },
            FieldKind::Date => NaiveDate::parse_from_str(trimmed, "%Y-%m-%d")
                .map(FieldValue::Date)
                .map_err(|_| FieldParseError::new(kind, raw)),
            FieldKind::List => Ok(FieldValue::List(
                trimmed
                    .split(',')
                    .map(|part| FieldValue::Text(part.trim().to_string()))
                    .filter(|value| value.is_truthy())
                    .collect(),
            )),
        }
    }
}

/// Error returned when a raw string cannot be parsed into a [`FieldValue`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldParseError {
    /// The kind that parsing was attempted as.
    pub expected: FieldKind,
    /// The offending raw input.
    pub raw: String,
}

impl FieldParseError {
    fn new(expected: FieldKind, raw: &str) -> Self {
        Self {
            expected,
            raw: raw.to_string(),
        }
    }
}

impl std::fmt::Display for FieldParseError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "cannot parse {:?} as {}",
            self.raw,
            self.expected.label()
        )
    }
}

impl std::error::Error for FieldParseError {}

// ============================================================================
// Shared priority model
// ============================================================================

/// A coarse criticality / priority band shared by checklists, deadlines and
/// tasks.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
pub enum Criticality {
    /// Low priority / nice to have.
    Low,
    /// Normal priority.
    #[default]
    Medium,
    /// High priority.
    High,
    /// Mission-critical / blocking.
    Critical,
}

impl Criticality {
    /// Returns a normalised weight in `[0, 1]` for scoring.
    pub fn weight(&self) -> f64 {
        match self {
            Criticality::Low => 0.25,
            Criticality::Medium => 0.5,
            Criticality::High => 0.75,
            Criticality::Critical => 1.0,
        }
    }

    /// Returns a human-readable label.
    pub fn label(&self) -> &'static str {
        match self {
            Criticality::Low => "low",
            Criticality::Medium => "medium",
            Criticality::High => "high",
            Criticality::Critical => "critical",
        }
    }
}

// ============================================================================
// Shared helpers
// ============================================================================

/// Returns today's date in UTC.
///
/// Used as the default reference point for deadline status, reminder scheduling
/// and task urgency when the caller does not supply an explicit "as of" date.
pub fn today() -> NaiveDate {
    chrono::Utc::now().date_naive()
}

/// Counts the words in a rendered document body.
pub(crate) fn word_count(text: &str) -> usize {
    text.split_whitespace().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_value_kind_and_render() {
        assert_eq!(FieldValue::text("hi").kind(), FieldKind::Text);
        assert_eq!(FieldValue::integer(3).render(), "3");
        assert_eq!(FieldValue::boolean(true).render(), "Yes");
        assert_eq!(FieldValue::boolean(false).render(), "No");
        let date = NaiveDate::from_ymd_opt(2026, 6, 14).expect("valid date");
        assert_eq!(FieldValue::date(date).render(), "2026-06-14");
        let list = FieldValue::list([FieldValue::text("a"), FieldValue::text("b")]);
        assert_eq!(list.render(), "a, b");
    }

    #[test]
    fn test_field_value_truthiness() {
        assert!(!FieldValue::text("  ").is_truthy());
        assert!(FieldValue::text("x").is_truthy());
        assert!(!FieldValue::integer(0).is_truthy());
        assert!(FieldValue::integer(1).is_truthy());
        assert!(!FieldValue::decimal(0.0).is_truthy());
        assert!(FieldValue::decimal(0.1).is_truthy());
        assert!(!FieldValue::list(std::iter::empty()).is_truthy());
    }

    #[test]
    fn test_field_value_parse_as() {
        assert_eq!(
            FieldValue::parse_as(FieldKind::Integer, " 42 ").expect("parses"),
            FieldValue::Integer(42)
        );
        assert_eq!(
            FieldValue::parse_as(FieldKind::Boolean, "YES").expect("parses"),
            FieldValue::Boolean(true)
        );
        let parsed = FieldValue::parse_as(FieldKind::Date, "2026-01-02").expect("parses");
        assert_eq!(
            parsed,
            FieldValue::Date(NaiveDate::from_ymd_opt(2026, 1, 2).expect("valid"))
        );
        let list = FieldValue::parse_as(FieldKind::List, "x, y ,z").expect("parses");
        assert_eq!(list.as_list().map(|values| values.len()), Some(3));

        let err = FieldValue::parse_as(FieldKind::Integer, "not-a-number").unwrap_err();
        assert_eq!(err.expected, FieldKind::Integer);
        assert!(err.to_string().contains("integer"));
    }

    #[test]
    fn test_criticality_ordering_and_weight() {
        assert!(Criticality::Critical > Criticality::High);
        assert!(Criticality::High > Criticality::Medium);
        assert!(Criticality::Medium > Criticality::Low);
        assert!(Criticality::Critical.weight() > Criticality::Low.weight());
        assert_eq!(Criticality::default(), Criticality::Medium);
    }
}
