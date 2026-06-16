//! Legal form filling assistance.
//!
//! A [`FormSchema`] describes a legal form as a list of typed
//! [`FormFieldSpec`]s, each carrying validation [`FieldConstraint`]s. A
//! [`FormFiller`] ingests data from an external, stringly-typed source (CSV
//! export, intake questionnaire, CRM record), optionally remapping source keys
//! to field names via a [`FieldMapping`], parses each value into the field's
//! [`FieldKind`], applies defaults and validates the result, yielding a
//! [`FormInstance`] with a [`FormValidationReport`].

use super::{FieldKind, FieldValue};
use crate::Jurisdiction;
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A validation constraint applied to a form field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FieldConstraint {
    /// A value must be present.
    Required,
    /// Minimum text length (characters).
    MinLength(usize),
    /// Maximum text length (characters).
    MaxLength(usize),
    /// Text must match this regular expression.
    Pattern(String),
    /// Numeric value must be at least this.
    Min(f64),
    /// Numeric value must be at most this.
    Max(f64),
    /// Value's rendered form must be one of these.
    OneOf(Vec<String>),
}

impl FieldConstraint {
    /// Validates a value against this constraint, returning an error message on
    /// failure (or `None` when satisfied).
    ///
    /// `Required` is handled at the schema level (it concerns presence rather
    /// than a present value), so it is treated as always satisfied here.
    pub fn check(&self, value: &FieldValue) -> Result<Option<String>> {
        let message = match self {
            FieldConstraint::Required => None,
            FieldConstraint::MinLength(min) => {
                let length = value.render().chars().count();
                (length < *min).then(|| format!("must be at least {} characters", min))
            }
            FieldConstraint::MaxLength(max) => {
                let length = value.render().chars().count();
                (length > *max).then(|| format!("must be at most {} characters", max))
            }
            FieldConstraint::Pattern(pattern) => {
                let regex = regex::Regex::new(pattern)
                    .map_err(|err| anyhow!("invalid pattern '{}': {}", pattern, err))?;
                let rendered = value.render();
                (!regex.is_match(&rendered))
                    .then(|| format!("does not match required pattern {}", pattern))
            }
            FieldConstraint::Min(min) => match value.as_number() {
                Some(number) => (number < *min).then(|| format!("must be >= {}", min)),
                None => Some("expected a numeric value".to_string()),
            },
            FieldConstraint::Max(max) => match value.as_number() {
                Some(number) => (number > *max).then(|| format!("must be <= {}", max)),
                None => Some("expected a numeric value".to_string()),
            },
            FieldConstraint::OneOf(allowed) => {
                let rendered = value.render();
                (!allowed.contains(&rendered))
                    .then(|| format!("must be one of: {}", allowed.join(", ")))
            }
        };
        Ok(message)
    }
}

/// The specification of a single form field.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FormFieldSpec {
    /// Field name (key in the produced instance).
    pub name: String,
    /// Human-readable label.
    pub label: String,
    /// Field kind.
    pub kind: FieldKind,
    /// Validation constraints.
    pub constraints: Vec<FieldConstraint>,
    /// Optional default value.
    pub default: Option<FieldValue>,
    /// Optional help text.
    pub help: Option<String>,
}

impl FormFieldSpec {
    /// Creates a new field specification.
    pub fn new(name: impl Into<String>, label: impl Into<String>, kind: FieldKind) -> Self {
        Self {
            name: name.into(),
            label: label.into(),
            kind,
            constraints: Vec::new(),
            default: None,
            help: None,
        }
    }

    /// Marks the field as required.
    pub fn required(mut self) -> Self {
        if !self.constraints.contains(&FieldConstraint::Required) {
            self.constraints.push(FieldConstraint::Required);
        }
        self
    }

    /// Adds a constraint.
    pub fn with_constraint(mut self, constraint: FieldConstraint) -> Self {
        self.constraints.push(constraint);
        self
    }

    /// Sets a default value.
    pub fn with_default(mut self, value: FieldValue) -> Self {
        self.default = Some(value);
        self
    }

    /// Sets help text.
    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }

    /// Returns whether the field is required.
    pub fn is_required(&self) -> bool {
        self.constraints.contains(&FieldConstraint::Required)
    }
}

/// A typed schema describing a legal form.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FormSchema {
    /// Stable identifier.
    pub id: String,
    /// Human-readable name.
    pub name: String,
    /// Field specifications.
    pub fields: Vec<FormFieldSpec>,
    /// Optional jurisdiction.
    pub jurisdiction: Option<Jurisdiction>,
}

impl FormSchema {
    /// Creates an empty schema.
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            fields: Vec::new(),
            jurisdiction: None,
        }
    }

    /// Adds a field (builder style).
    pub fn with_field(mut self, field: FormFieldSpec) -> Self {
        self.fields.push(field);
        self
    }

    /// Sets the jurisdiction.
    pub fn with_jurisdiction(mut self, jurisdiction: Jurisdiction) -> Self {
        self.jurisdiction = Some(jurisdiction);
        self
    }

    /// Returns a field specification by name.
    pub fn field(&self, name: &str) -> Option<&FormFieldSpec> {
        self.fields.iter().find(|field| field.name == name)
    }

    /// Returns all field names.
    pub fn field_names(&self) -> Vec<String> {
        self.fields.iter().map(|field| field.name.clone()).collect()
    }

    /// Validates a set of typed values against the schema.
    pub fn validate(&self, values: &HashMap<String, FieldValue>) -> Result<FormValidationReport> {
        let mut report = FormValidationReport::default();

        for field in &self.fields {
            match values.get(&field.name) {
                None => {
                    if field.is_required() {
                        report.missing_required.push(field.name.clone());
                    }
                }
                Some(value) => {
                    if !kind_compatible(field.kind, value.kind()) {
                        report.errors.push(FieldError {
                            field: field.name.clone(),
                            message: format!(
                                "expected {} but got {}",
                                field.kind.label(),
                                value.kind().label()
                            ),
                        });
                        continue;
                    }
                    for constraint in &field.constraints {
                        if let Some(message) = constraint.check(value)? {
                            report.errors.push(FieldError {
                                field: field.name.clone(),
                                message,
                            });
                        }
                    }
                }
            }
        }

        report.missing_required.sort();
        report.errors.sort_by(|a, b| a.field.cmp(&b.field));
        Ok(report)
    }

    /// Builds a standard client-intake form schema.
    pub fn client_intake() -> Self {
        Self::new("client_intake", "New Client Intake")
            .with_field(
                FormFieldSpec::new("full_name", "Full Legal Name", FieldKind::Text)
                    .required()
                    .with_constraint(FieldConstraint::MinLength(2)),
            )
            .with_field(
                FormFieldSpec::new("email", "Email Address", FieldKind::Text)
                    .required()
                    .with_constraint(FieldConstraint::Pattern(
                        r"^[^@\s]+@[^@\s]+\.[^@\s]+$".to_string(),
                    )),
            )
            .with_field(
                FormFieldSpec::new("matter_type", "Matter Type", FieldKind::Text)
                    .required()
                    .with_constraint(FieldConstraint::OneOf(vec![
                        "litigation".to_string(),
                        "transactional".to_string(),
                        "advisory".to_string(),
                    ])),
            )
            .with_field(
                FormFieldSpec::new("retainer", "Retainer Amount", FieldKind::Decimal)
                    .with_constraint(FieldConstraint::Min(0.0)),
            )
            .with_field(FormFieldSpec::new(
                "engagement_date",
                "Engagement Date",
                FieldKind::Date,
            ))
            .with_field(
                FormFieldSpec::new(
                    "conflict_checked",
                    "Conflict Check Done",
                    FieldKind::Boolean,
                )
                .with_default(FieldValue::Boolean(false)),
            )
    }
}

/// Maps external source keys to schema field names.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FieldMapping {
    /// `source_key -> field_name` entries.
    mappings: HashMap<String, String>,
}

impl FieldMapping {
    /// Creates an empty mapping.
    pub fn new() -> Self {
        Self::default()
    }

    /// Maps a source key to a target field name (builder style).
    pub fn map(mut self, source_key: impl Into<String>, field_name: impl Into<String>) -> Self {
        self.mappings.insert(source_key.into(), field_name.into());
        self
    }

    /// Resolves the target field name for a source key (identity if unmapped).
    pub fn resolve<'a>(&'a self, source_key: &'a str) -> &'a str {
        self.mappings
            .get(source_key)
            .map(String::as_str)
            .unwrap_or(source_key)
    }

    /// Returns whether the mapping is empty.
    pub fn is_empty(&self) -> bool {
        self.mappings.is_empty()
    }
}

/// An error attached to a specific field.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldError {
    /// Field name.
    pub field: String,
    /// Failure message.
    pub message: String,
}

/// The result of validating a form instance.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FormValidationReport {
    /// Required fields with no value.
    pub missing_required: Vec<String>,
    /// Per-field validation errors.
    pub errors: Vec<FieldError>,
    /// Source keys that did not map to any schema field.
    pub unmapped_keys: Vec<String>,
}

impl FormValidationReport {
    /// Returns whether the form is valid (complete and error-free).
    pub fn is_valid(&self) -> bool {
        self.missing_required.is_empty() && self.errors.is_empty()
    }

    /// Returns a flat list of human-readable problems.
    pub fn problems(&self) -> Vec<String> {
        let mut problems = Vec::new();
        for name in &self.missing_required {
            problems.push(format!("missing required field '{}'", name));
        }
        for error in &self.errors {
            problems.push(format!("{}: {}", error.field, error.message));
        }
        problems
    }
}

/// A filled-in form instance.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FormInstance {
    /// Source schema id.
    pub schema_id: String,
    /// Resolved field values.
    pub values: HashMap<String, FieldValue>,
    /// Validation result.
    pub validation: FormValidationReport,
}

impl FormInstance {
    /// Returns a field value.
    pub fn get(&self, name: &str) -> Option<&FieldValue> {
        self.values.get(name)
    }

    /// Returns whether the instance passed validation.
    pub fn is_valid(&self) -> bool {
        self.validation.is_valid()
    }

    /// Exports the instance as a JSON object of "natural" values.
    pub fn to_json(&self) -> serde_json::Value {
        let mut object = serde_json::Map::new();
        for (name, value) in &self.values {
            object.insert(name.clone(), value.to_json());
        }
        serde_json::Value::Object(object)
    }
}

/// Fills a [`FormSchema`] from external data, applying mapping and validation.
#[derive(Debug, Clone)]
pub struct FormFiller {
    schema: FormSchema,
    mapping: FieldMapping,
}

impl FormFiller {
    /// Creates a filler for the supplied schema (identity mapping).
    pub fn new(schema: FormSchema) -> Self {
        Self {
            schema,
            mapping: FieldMapping::new(),
        }
    }

    /// Attaches a source-key mapping.
    pub fn with_mapping(mut self, mapping: FieldMapping) -> Self {
        self.mapping = mapping;
        self
    }

    /// Returns the schema.
    pub fn schema(&self) -> &FormSchema {
        &self.schema
    }

    /// Fills the form from raw string-keyed input.
    ///
    /// Each source key is remapped (if a mapping exists), parsed into the target
    /// field's [`FieldKind`], and collected. Parse failures and unmapped keys are
    /// recorded in the validation report; defaults fill in absent fields; the
    /// result is validated against the schema.
    pub fn fill(&self, source: &HashMap<String, String>) -> Result<FormInstance> {
        let mut values: HashMap<String, FieldValue> = HashMap::new();
        let mut report = FormValidationReport::default();

        for (source_key, raw) in source {
            let field_name = self.mapping.resolve(source_key);
            match self.schema.field(field_name) {
                Some(field) => match FieldValue::parse_as(field.kind, raw) {
                    Ok(value) => {
                        values.insert(field_name.to_string(), value);
                    }
                    Err(err) => report.errors.push(FieldError {
                        field: field_name.to_string(),
                        message: err.to_string(),
                    }),
                },
                None => report.unmapped_keys.push(source_key.clone()),
            }
        }

        // Apply defaults for any still-absent fields.
        for field in &self.schema.fields {
            if values.contains_key(&field.name) {
                continue;
            }
            if let Some(default) = &field.default {
                values.insert(field.name.clone(), default.clone());
            }
        }

        let schema_report = self.schema.validate(&values)?;
        report.missing_required = schema_report.missing_required;
        report.errors.extend(schema_report.errors);
        report.errors.sort_by(|a, b| a.field.cmp(&b.field));
        report.unmapped_keys.sort();

        Ok(FormInstance {
            schema_id: self.schema.id.clone(),
            values,
            validation: report,
        })
    }

    /// Fills the form from already-typed values (no parsing), then validates.
    pub fn fill_typed(&self, mut values: HashMap<String, FieldValue>) -> Result<FormInstance> {
        for field in &self.schema.fields {
            if values.contains_key(&field.name) {
                continue;
            }
            if let Some(default) = &field.default {
                values.insert(field.name.clone(), default.clone());
            }
        }
        let validation = self.schema.validate(&values)?;
        Ok(FormInstance {
            schema_id: self.schema.id.clone(),
            values,
            validation,
        })
    }
}

/// Returns whether a supplied value kind satisfies the declared field kind.
fn kind_compatible(expected: FieldKind, actual: FieldKind) -> bool {
    expected == actual || (expected == FieldKind::Decimal && actual == FieldKind::Integer)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn raw(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect()
    }

    #[test]
    fn test_fill_and_parse_with_defaults() {
        let filler = FormFiller::new(FormSchema::client_intake());
        let instance = filler
            .fill(&raw(&[
                ("full_name", "Jane Roe"),
                ("email", "jane@example.com"),
                ("matter_type", "litigation"),
                ("retainer", "5000"),
                ("engagement_date", "2026-06-14"),
            ]))
            .expect("fills");
        assert!(instance.is_valid(), "{:?}", instance.validation.problems());
        // retainer parsed as decimal from an integer-looking string
        assert_eq!(
            instance.get("retainer").and_then(FieldValue::as_number),
            Some(5000.0)
        );
        // default applied for conflict_checked
        assert_eq!(
            instance.get("conflict_checked"),
            Some(&FieldValue::Boolean(false))
        );
        let json = instance.to_json();
        assert_eq!(json["full_name"], serde_json::json!("Jane Roe"));
    }

    #[test]
    fn test_validation_missing_and_pattern() {
        let filler = FormFiller::new(FormSchema::client_intake());
        let instance = filler
            .fill(&raw(&[
                ("email", "not-an-email"),
                ("matter_type", "litigation"),
            ]))
            .expect("fills");
        assert!(!instance.is_valid());
        assert!(
            instance
                .validation
                .missing_required
                .contains(&"full_name".to_string())
        );
        assert!(
            instance
                .validation
                .errors
                .iter()
                .any(|e| e.field == "email")
        );
    }

    #[test]
    fn test_oneof_constraint_rejects_unknown() {
        let filler = FormFiller::new(FormSchema::client_intake());
        let instance = filler
            .fill(&raw(&[
                ("full_name", "Al"),
                ("email", "al@x.io"),
                ("matter_type", "criminal"),
            ]))
            .expect("fills");
        assert!(
            instance
                .validation
                .errors
                .iter()
                .any(|e| e.field == "matter_type" && e.message.contains("one of"))
        );
    }

    #[test]
    fn test_field_mapping_and_unmapped_keys() {
        let mapping = FieldMapping::new()
            .map("client_name", "full_name")
            .map("client_email", "email");
        let filler = FormFiller::new(FormSchema::client_intake()).with_mapping(mapping);
        let instance = filler
            .fill(&raw(&[
                ("client_name", "Bob Builder"),
                ("client_email", "bob@build.io"),
                ("matter_type", "advisory"),
                ("unknown_col", "ignored"),
            ]))
            .expect("fills");
        assert_eq!(
            instance.get("full_name").and_then(FieldValue::as_text),
            Some("Bob Builder")
        );
        assert!(
            instance
                .validation
                .unmapped_keys
                .contains(&"unknown_col".to_string())
        );
    }

    #[test]
    fn test_parse_failure_recorded() {
        let filler = FormFiller::new(FormSchema::client_intake());
        let instance = filler
            .fill(&raw(&[
                ("full_name", "Jane"),
                ("email", "jane@x.io"),
                ("matter_type", "advisory"),
                ("retainer", "not-a-number"),
            ]))
            .expect("fills");
        assert!(
            instance
                .validation
                .errors
                .iter()
                .any(|e| e.field == "retainer")
        );
    }
}
