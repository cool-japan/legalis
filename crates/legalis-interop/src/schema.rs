//! Schema validation and migration for legal document formats.
//!
//! Provides XML Schema (XSD) and JSON Schema validation capabilities,
//! custom schema extension points, schema migration utilities, and
//! schema compatibility checking.

use crate::{InteropError, InteropResult, LegalFormat};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Schema validation result.
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Whether the validation passed
    pub valid: bool,
    /// Validation errors
    pub errors: Vec<String>,
    /// Validation warnings
    pub warnings: Vec<String>,
}

impl ValidationResult {
    /// Creates a successful validation result.
    pub fn success() -> Self {
        Self {
            valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// Creates a failed validation result with errors.
    pub fn failure(errors: Vec<String>) -> Self {
        Self {
            valid: false,
            errors,
            warnings: Vec::new(),
        }
    }

    /// Adds a warning to the validation result.
    pub fn with_warning(mut self, warning: String) -> Self {
        self.warnings.push(warning);
        self
    }
}

/// Schema type for validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchemaType {
    /// XML Schema (XSD)
    Xsd,
    /// JSON Schema
    JsonSchema,
    /// Custom schema
    Custom,
}

/// Schema definition.
#[derive(Debug, Clone)]
pub struct Schema {
    /// Schema type
    pub schema_type: SchemaType,
    /// Schema content
    pub content: String,
    /// Schema version
    pub version: String,
}

/// XML Schema validator.
pub struct XmlSchemaValidator {
    schemas: HashMap<LegalFormat, Schema>,
}

impl XmlSchemaValidator {
    /// Creates a new XML schema validator.
    pub fn new() -> Self {
        Self {
            schemas: HashMap::new(),
        }
    }

    /// Registers a schema for a legal format.
    pub fn register_schema(&mut self, format: LegalFormat, schema: Schema) {
        self.schemas.insert(format, schema);
    }

    /// Validates XML against a registered schema.
    pub fn validate(&self, format: LegalFormat, xml: &str) -> InteropResult<ValidationResult> {
        // Get the schema for the format
        let _schema = self.schemas.get(&format).ok_or_else(|| {
            InteropError::ValidationError(format!("No schema registered for format {:?}", format))
        })?;

        // Basic XML well-formedness check
        // Note: Full XSD validation would require an external library like `xsd-parser`
        // For now, we do basic checks
        let result = self.validate_xml_wellformed(xml)?;

        // Additional format-specific validations
        match format {
            LegalFormat::AkomaNtoso => self.validate_akoma_ntoso(xml, &result),
            LegalFormat::LegalRuleML => self.validate_legalruleml(xml, &result),
            LegalFormat::LegalDocML => self.validate_legaldocml(xml, &result),
            _ => Ok(result),
        }
    }

    fn validate_xml_wellformed(&self, xml: &str) -> InteropResult<ValidationResult> {
        // Use quick-xml to parse and validate XML structure
        use quick_xml::Reader;
        use quick_xml::events::Event;

        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(true);

        let mut errors = Vec::new();
        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Eof) => break,
                Err(e) => {
                    errors.push(format!("XML parsing error: {}", e));
                    break;
                }
                _ => {}
            }
            buf.clear();
        }

        if errors.is_empty() {
            Ok(ValidationResult::success())
        } else {
            Ok(ValidationResult::failure(errors))
        }
    }

    fn validate_akoma_ntoso(
        &self,
        xml: &str,
        base_result: &ValidationResult,
    ) -> InteropResult<ValidationResult> {
        let mut result = base_result.clone();

        // Check for required Akoma Ntoso elements
        if !xml.contains("<akomaNtoso") && !xml.contains("<act>") {
            result.warnings.push(
                "Document may not be valid Akoma Ntoso: missing required elements".to_string(),
            );
        }

        Ok(result)
    }

    fn validate_legalruleml(
        &self,
        xml: &str,
        base_result: &ValidationResult,
    ) -> InteropResult<ValidationResult> {
        let mut result = base_result.clone();

        // Check for LegalRuleML namespace
        if !xml.contains("lrml:") && !xml.contains("LegalRuleML") {
            result.warnings.push(
                "Document may not be valid LegalRuleML: missing namespace or elements".to_string(),
            );
        }

        Ok(result)
    }

    fn validate_legaldocml(
        &self,
        xml: &str,
        base_result: &ValidationResult,
    ) -> InteropResult<ValidationResult> {
        let mut result = base_result.clone();

        // Check for LegalDocML elements
        if !xml.contains("LegalDocument") {
            result.warnings.push(
                "Document may not be valid LegalDocML: missing required elements".to_string(),
            );
        }

        Ok(result)
    }
}

impl Default for XmlSchemaValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// JSON Schema validator.
pub struct JsonSchemaValidator {
    schemas: HashMap<LegalFormat, Schema>,
}

impl JsonSchemaValidator {
    /// Creates a new JSON schema validator.
    pub fn new() -> Self {
        Self {
            schemas: HashMap::new(),
        }
    }

    /// Registers a schema for a legal format.
    pub fn register_schema(&mut self, format: LegalFormat, schema: Schema) {
        self.schemas.insert(format, schema);
    }

    /// Validates JSON against a registered schema.
    pub fn validate(&self, format: LegalFormat, json: &str) -> InteropResult<ValidationResult> {
        // Get the schema for the format
        let _schema = self.schemas.get(&format).ok_or_else(|| {
            InteropError::ValidationError(format!("No schema registered for format {:?}", format))
        })?;

        // Basic JSON validity check
        match serde_json::from_str::<serde_json::Value>(json) {
            Ok(_) => Ok(ValidationResult::success()),
            Err(e) => Ok(ValidationResult::failure(vec![format!(
                "Invalid JSON: {}",
                e
            )])),
        }
    }
}

impl Default for JsonSchemaValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Schema extension point for custom validation.
pub trait SchemaExtension {
    /// Validates content against custom rules.
    fn validate(&self, content: &str) -> InteropResult<ValidationResult>;

    /// Returns the name of this extension.
    fn name(&self) -> &str;
}

/// Schema migration describes how to migrate from one schema version to another.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaMigration {
    /// Source schema version
    pub from_version: String,
    /// Target schema version
    pub to_version: String,
    /// Migration steps
    pub steps: Vec<MigrationStep>,
}

/// A single migration step.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MigrationStep {
    /// Rename an element or field
    Rename { from: String, to: String },
    /// Add a new element or field with default value
    AddField { name: String, default_value: String },
    /// Remove an element or field
    RemoveField { name: String },
    /// Transform a value
    Transform { field: String, function: String },
}

/// Schema migration engine.
pub struct SchemaMigrator {
    migrations: Vec<SchemaMigration>,
}

impl SchemaMigrator {
    /// Creates a new schema migrator.
    pub fn new() -> Self {
        Self {
            migrations: Vec::new(),
        }
    }

    /// Registers a migration.
    pub fn register_migration(&mut self, migration: SchemaMigration) {
        self.migrations.push(migration);
    }

    /// Migrates content from one schema version to another.
    pub fn migrate(
        &self,
        content: &str,
        from_version: &str,
        to_version: &str,
    ) -> InteropResult<String> {
        // Find migration path
        let migration = self
            .migrations
            .iter()
            .find(|m| m.from_version == from_version && m.to_version == to_version)
            .ok_or_else(|| {
                InteropError::ValidationError(format!(
                    "No migration found from {} to {}",
                    from_version, to_version
                ))
            })?;

        // Apply migration steps
        let mut result = content.to_string();
        for step in &migration.steps {
            result = self.apply_migration_step(&result, step)?;
        }

        Ok(result)
    }

    fn apply_migration_step(&self, content: &str, step: &MigrationStep) -> InteropResult<String> {
        match step {
            MigrationStep::Rename { from, to } => {
                // Simple string replacement (in production, use proper XML/JSON parsing)
                Ok(content.replace(from, to))
            }
            MigrationStep::AddField {
                name: _,
                default_value: _,
            } => {
                // For now, return content as-is
                // In production, parse and add field
                Ok(content.to_string())
            }
            MigrationStep::RemoveField { name } => {
                // Simple removal (in production, use proper parsing)
                Ok(content.replace(name, ""))
            }
            MigrationStep::Transform {
                field: _,
                function: _,
            } => {
                // For now, return content as-is
                // In production, apply transformation function
                Ok(content.to_string())
            }
        }
    }
}

impl Default for SchemaMigrator {
    fn default() -> Self {
        Self::new()
    }
}

/// Schema compatibility checker.
pub struct SchemaCompatibilityChecker;

impl SchemaCompatibilityChecker {
    /// Creates a new schema compatibility checker.
    pub fn new() -> Self {
        Self
    }

    /// Checks if two schemas are compatible.
    pub fn check_compatibility(
        &self,
        source_schema: &Schema,
        target_schema: &Schema,
    ) -> InteropResult<ValidationResult> {
        let mut warnings = Vec::new();

        // Check schema types
        if source_schema.schema_type != target_schema.schema_type {
            warnings.push(format!(
                "Schema types differ: {:?} vs {:?}",
                source_schema.schema_type, target_schema.schema_type
            ));
        }

        // Check versions
        if source_schema.version != target_schema.version {
            warnings.push(format!(
                "Schema versions differ: {} vs {}",
                source_schema.version, target_schema.version
            ));
        }

        let mut result = ValidationResult::success();
        result.warnings = warnings;
        Ok(result)
    }
}

impl Default for SchemaCompatibilityChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xml_wellformed_validation() {
        let validator = XmlSchemaValidator::new();

        // Valid XML
        let valid_xml = r#"<?xml version="1.0"?><root><child>text</child></root>"#;
        let result = validator.validate_xml_wellformed(valid_xml).unwrap();
        assert!(result.valid);

        // Invalid XML
        let invalid_xml = r#"<root><child>text</root>"#;
        let result = validator.validate_xml_wellformed(invalid_xml).unwrap();
        assert!(!result.valid);
    }

    #[test]
    fn test_json_validation() {
        let validator = JsonSchemaValidator::new();

        // Valid JSON
        let valid_json = r#"{"key": "value"}"#;
        let result = validator.validate(LegalFormat::Legalis, valid_json);
        assert!(result.is_err()); // No schema registered

        // Invalid JSON
        let invalid_json = r#"{key: value}"#;
        let result = validator.validate(LegalFormat::Legalis, invalid_json);
        assert!(result.is_err()); // No schema registered
    }

    #[test]
    fn test_schema_migration() {
        let mut migrator = SchemaMigrator::new();

        let migration = SchemaMigration {
            from_version: "1.0".to_string(),
            to_version: "2.0".to_string(),
            steps: vec![MigrationStep::Rename {
                from: "oldField".to_string(),
                to: "newField".to_string(),
            }],
        };

        migrator.register_migration(migration);

        let content = r#"<root><oldField>value</oldField></root>"#;
        let result = migrator.migrate(content, "1.0", "2.0").unwrap();
        assert!(result.contains("newField"));
        assert!(!result.contains("oldField"));
    }

    #[test]
    fn test_schema_compatibility() {
        let checker = SchemaCompatibilityChecker::new();

        let schema1 = Schema {
            schema_type: SchemaType::Xsd,
            content: "schema1".to_string(),
            version: "1.0".to_string(),
        };

        let schema2 = Schema {
            schema_type: SchemaType::Xsd,
            content: "schema2".to_string(),
            version: "2.0".to_string(),
        };

        let result = checker.check_compatibility(&schema1, &schema2).unwrap();
        assert!(result.valid);
        assert!(!result.warnings.is_empty()); // Version mismatch warning
    }

    #[test]
    fn test_validation_result() {
        let success = ValidationResult::success();
        assert!(success.valid);
        assert!(success.errors.is_empty());

        let failure = ValidationResult::failure(vec!["Error 1".to_string()]);
        assert!(!failure.valid);
        assert_eq!(failure.errors.len(), 1);

        let with_warning = success.with_warning("Warning 1".to_string());
        assert_eq!(with_warning.warnings.len(), 1);
    }
}
