//! Format Validation - Comprehensive validation for legal document formats.
//!
//! This module provides:
//! - Schema validation for XML and JSON formats
//! - Semantic validation rules for legal content
//! - Cross-format consistency checking
//! - Custom validation plugins
//! - Detailed validation report generation

use crate::LegalFormat;
use legalis_core::Statute;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Validation severity level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidationSeverity {
    /// Error - validation failed, document is invalid
    Error,
    /// Warning - potential issue detected
    Warning,
    /// Info - informational message
    Info,
}

/// Validation issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationIssue {
    /// Severity level
    pub severity: ValidationSeverity,
    /// Issue code
    pub code: String,
    /// Human-readable message
    pub message: String,
    /// Location in document (optional)
    pub location: Option<String>,
    /// Suggested fix (optional)
    pub suggestion: Option<String>,
}

impl ValidationIssue {
    /// Creates a new validation issue
    pub fn new(
        severity: ValidationSeverity,
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            severity,
            code: code.into(),
            message: message.into(),
            location: None,
            suggestion: None,
        }
    }

    /// Adds location to the issue
    pub fn with_location(mut self, location: impl Into<String>) -> Self {
        self.location = Some(location.into());
        self
    }

    /// Adds suggestion to the issue
    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }
}

/// Validation report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    /// Format being validated
    pub format: LegalFormat,
    /// All validation issues
    pub issues: Vec<ValidationIssue>,
    /// Number of errors
    pub error_count: usize,
    /// Number of warnings
    pub warning_count: usize,
    /// Number of info messages
    pub info_count: usize,
    /// Whether validation passed (no errors)
    pub passed: bool,
}

impl ValidationReport {
    /// Creates a new validation report
    pub fn new(format: LegalFormat) -> Self {
        Self {
            format,
            issues: Vec::new(),
            error_count: 0,
            warning_count: 0,
            info_count: 0,
            passed: true,
        }
    }

    /// Adds an issue to the report
    pub fn add_issue(&mut self, issue: ValidationIssue) {
        match issue.severity {
            ValidationSeverity::Error => {
                self.error_count += 1;
                self.passed = false;
            }
            ValidationSeverity::Warning => {
                self.warning_count += 1;
            }
            ValidationSeverity::Info => {
                self.info_count += 1;
            }
        }
        self.issues.push(issue);
    }

    /// Returns true if validation passed with no errors
    pub fn is_valid(&self) -> bool {
        self.passed
    }

    /// Returns true if there are any warnings
    pub fn has_warnings(&self) -> bool {
        self.warning_count > 0
    }

    /// Formats report as human-readable string
    pub fn format(&self) -> String {
        let mut output = format!("Validation Report for {:?}\n", self.format);
        output.push_str(&format!(
            "Status: {}\n",
            if self.passed { "PASSED" } else { "FAILED" }
        ));
        output.push_str(&format!(
            "Errors: {}, Warnings: {}, Info: {}\n\n",
            self.error_count, self.warning_count, self.info_count
        ));

        for issue in &self.issues {
            let severity_str = match issue.severity {
                ValidationSeverity::Error => "ERROR",
                ValidationSeverity::Warning => "WARNING",
                ValidationSeverity::Info => "INFO",
            };
            output.push_str(&format!(
                "[{}] {}: {}\n",
                severity_str, issue.code, issue.message
            ));
            if let Some(loc) = &issue.location {
                output.push_str(&format!("  Location: {}\n", loc));
            }
            if let Some(sug) = &issue.suggestion {
                output.push_str(&format!("  Suggestion: {}\n", sug));
            }
        }

        output
    }
}

/// Validation rule trait
pub trait ValidationRule: Send + Sync {
    /// Returns the rule name
    fn name(&self) -> &str;

    /// Validates source text
    fn validate(&self, source: &str, format: LegalFormat) -> Vec<ValidationIssue>;
}

/// Schema validator for XML/JSON formats
pub struct SchemaValidator;

impl SchemaValidator {
    /// Creates a new schema validator
    pub fn new(_format: LegalFormat) -> Self {
        Self
    }

    /// Validates XML structure
    fn validate_xml(&self, source: &str) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();

        // Check basic XML well-formedness
        if !source.trim().starts_with('<') {
            issues.push(ValidationIssue::new(
                ValidationSeverity::Error,
                "XML001",
                "Document does not start with XML tag",
            ));
        }

        // Check for balanced tags
        let open_tags = source.matches('<').count();
        let close_tags = source.matches('>').count();
        if open_tags != close_tags {
            issues.push(
                ValidationIssue::new(
                    ValidationSeverity::Error,
                    "XML002",
                    format!(
                        "Unbalanced tags: {} opening, {} closing",
                        open_tags, close_tags
                    ),
                )
                .with_suggestion("Check that all tags are properly closed"),
            );
        }

        issues
    }

    /// Validates JSON structure
    fn validate_json(&self, source: &str) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();

        // Try to parse as JSON
        match serde_json::from_str::<serde_json::Value>(source) {
            Ok(_) => {
                // Valid JSON
            }
            Err(e) => {
                issues.push(
                    ValidationIssue::new(
                        ValidationSeverity::Error,
                        "JSON001",
                        format!("Invalid JSON: {}", e),
                    )
                    .with_suggestion("Check JSON syntax"),
                );
            }
        }

        issues
    }
}

impl ValidationRule for SchemaValidator {
    fn name(&self) -> &str {
        "SchemaValidator"
    }

    fn validate(&self, source: &str, format: LegalFormat) -> Vec<ValidationIssue> {
        match format {
            LegalFormat::AkomaNtoso
            | LegalFormat::LegalRuleML
            | LegalFormat::LegalDocML
            | LegalFormat::LKIF
            | LegalFormat::LegalCite
            | LegalFormat::MetaLex
            | LegalFormat::Mpeg21Rel
            | LegalFormat::Formex
            | LegalFormat::Niem
            | LegalFormat::RegML => self.validate_xml(source),

            LegalFormat::CommonForm
            | LegalFormat::ClauseIo
            | LegalFormat::FinReg
            | LegalFormat::MiFID2
            | LegalFormat::Basel3 => self.validate_json(source),

            _ => Vec::new(), // No schema validation for text-based formats
        }
    }
}

/// Semantic validation rule
pub struct SemanticValidator;

impl SemanticValidator {
    /// Creates a new semantic validator
    pub fn new() -> Self {
        Self
    }

    /// Validates statute semantics
    fn validate_statute(&self, statute: &Statute) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();

        // Check ID is not empty
        if statute.id.is_empty() {
            issues.push(ValidationIssue::new(
                ValidationSeverity::Error,
                "SEM001",
                "Statute ID cannot be empty",
            ));
        }

        // Check title is not empty
        if statute.title.is_empty() {
            issues.push(
                ValidationIssue::new(
                    ValidationSeverity::Warning,
                    "SEM002",
                    format!("Statute '{}' has empty title", statute.id),
                )
                .with_suggestion("Add a descriptive title"),
            );
        }

        // Check for excessively long titles
        if statute.title.len() > 200 {
            issues.push(
                ValidationIssue::new(
                    ValidationSeverity::Warning,
                    "SEM003",
                    format!(
                        "Statute '{}' has very long title ({} chars)",
                        statute.id,
                        statute.title.len()
                    ),
                )
                .with_suggestion("Consider shortening the title"),
            );
        }

        // Check effect description
        if statute.effect.description.is_empty() {
            issues.push(ValidationIssue::new(
                ValidationSeverity::Warning,
                "SEM004",
                format!("Statute '{}' has empty effect description", statute.id),
            ));
        }

        // Check for reasonable number of preconditions
        if statute.preconditions.len() > 20 {
            issues.push(
                ValidationIssue::new(
                    ValidationSeverity::Info,
                    "SEM005",
                    format!(
                        "Statute '{}' has many preconditions ({})",
                        statute.id,
                        statute.preconditions.len()
                    ),
                )
                .with_suggestion("Consider simplifying or splitting the statute"),
            );
        }

        issues
    }
}

impl Default for SemanticValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidationRule for SemanticValidator {
    fn name(&self) -> &str {
        "SemanticValidator"
    }

    fn validate(&self, _source: &str, _format: LegalFormat) -> Vec<ValidationIssue> {
        // Semantic validation is done on parsed statutes, not source text
        Vec::new()
    }
}

/// Cross-format consistency checker
pub struct ConsistencyChecker {
    reference_statutes: Vec<Statute>,
}

impl ConsistencyChecker {
    /// Creates a new consistency checker
    pub fn new(reference: Vec<Statute>) -> Self {
        Self {
            reference_statutes: reference,
        }
    }

    /// Checks consistency between reference and target statutes
    pub fn check_consistency(&self, target: &[Statute]) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();

        // Check statute count
        if target.len() != self.reference_statutes.len() {
            issues.push(ValidationIssue::new(
                ValidationSeverity::Warning,
                "CONS001",
                format!(
                    "Statute count mismatch: expected {}, found {}",
                    self.reference_statutes.len(),
                    target.len()
                ),
            ));
        }

        // Check IDs match
        let reference_ids: Vec<_> = self.reference_statutes.iter().map(|s| &s.id).collect();
        let target_ids: Vec<_> = target.iter().map(|s| &s.id).collect();

        for ref_id in &reference_ids {
            if !target_ids.contains(ref_id) {
                issues.push(ValidationIssue::new(
                    ValidationSeverity::Error,
                    "CONS002",
                    format!("Missing statute: {}", ref_id),
                ));
            }
        }

        for target_id in &target_ids {
            if !reference_ids.contains(target_id) {
                issues.push(ValidationIssue::new(
                    ValidationSeverity::Warning,
                    "CONS003",
                    format!("Extra statute: {}", target_id),
                ));
            }
        }

        issues
    }
}

/// Custom validation plugin
pub trait ValidationPlugin: Send + Sync {
    /// Plugin name
    fn name(&self) -> &str;

    /// Validates and returns issues
    fn validate(
        &self,
        source: &str,
        format: LegalFormat,
        statutes: &[Statute],
    ) -> Vec<ValidationIssue>;
}

/// Format validator with plugin support
pub struct FormatValidator {
    rules: Vec<Box<dyn ValidationRule>>,
    plugins: Vec<Box<dyn ValidationPlugin>>,
}

impl FormatValidator {
    /// Creates a new format validator
    pub fn new() -> Self {
        Self {
            rules: vec![Box::new(SemanticValidator::new())],
            plugins: Vec::new(),
        }
    }

    /// Adds a validation rule
    pub fn add_rule(&mut self, rule: Box<dyn ValidationRule>) {
        self.rules.push(rule);
    }

    /// Adds a validation plugin
    pub fn add_plugin(&mut self, plugin: Box<dyn ValidationPlugin>) {
        self.plugins.push(plugin);
    }

    /// Validates source text for a specific format
    pub fn validate(&self, source: &str, format: LegalFormat) -> ValidationReport {
        let mut report = ValidationReport::new(format);

        // Add schema validator for this format
        let schema_validator = SchemaValidator::new(format);
        for issue in schema_validator.validate(source, format) {
            report.add_issue(issue);
        }

        // Run validation rules
        for rule in &self.rules {
            for issue in rule.validate(source, format) {
                report.add_issue(issue);
            }
        }

        report
    }

    /// Validates source text and parsed statutes
    pub fn validate_with_statutes(
        &self,
        source: &str,
        format: LegalFormat,
        statutes: &[Statute],
    ) -> ValidationReport {
        let mut report = self.validate(source, format);

        // Validate each statute semantically
        let semantic_validator = SemanticValidator::new();
        for statute in statutes {
            for issue in semantic_validator.validate_statute(statute) {
                report.add_issue(issue);
            }
        }

        // Run plugins
        for plugin in &self.plugins {
            for issue in plugin.validate(source, format, statutes) {
                report.add_issue(issue);
            }
        }

        report
    }
}

impl Default for FormatValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Validation statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationStats {
    /// Total validations performed
    pub total_validations: usize,
    /// Successful validations (no errors)
    pub successful: usize,
    /// Failed validations
    pub failed: usize,
    /// Total errors across all validations
    pub total_errors: usize,
    /// Total warnings across all validations
    pub total_warnings: usize,
    /// Validation results by format
    pub by_format: HashMap<LegalFormat, FormatStats>,
}

impl ValidationStats {
    /// Creates new validation statistics
    pub fn new() -> Self {
        Self {
            total_validations: 0,
            successful: 0,
            failed: 0,
            total_errors: 0,
            total_warnings: 0,
            by_format: HashMap::new(),
        }
    }

    /// Records a validation result
    pub fn record(&mut self, report: &ValidationReport) {
        self.total_validations += 1;

        if report.passed {
            self.successful += 1;
        } else {
            self.failed += 1;
        }

        self.total_errors += report.error_count;
        self.total_warnings += report.warning_count;

        let format_stats = self.by_format.entry(report.format).or_default();
        format_stats.record(report);
    }

    /// Returns success rate (0.0 - 1.0)
    pub fn success_rate(&self) -> f64 {
        if self.total_validations == 0 {
            return 1.0;
        }
        self.successful as f64 / self.total_validations as f64
    }
}

impl Default for ValidationStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Format-specific statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatStats {
    /// Number of validations
    pub count: usize,
    /// Number of successful validations
    pub successful: usize,
    /// Number of failed validations
    pub failed: usize,
    /// Total errors
    pub errors: usize,
    /// Total warnings
    pub warnings: usize,
}

impl FormatStats {
    /// Creates new format statistics
    pub fn new() -> Self {
        Self {
            count: 0,
            successful: 0,
            failed: 0,
            errors: 0,
            warnings: 0,
        }
    }

    /// Records a validation result
    pub fn record(&mut self, report: &ValidationReport) {
        self.count += 1;
        if report.passed {
            self.successful += 1;
        } else {
            self.failed += 1;
        }
        self.errors += report.error_count;
        self.warnings += report.warning_count;
    }
}

impl Default for FormatStats {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{Effect, EffectType};

    #[test]
    fn test_validation_issue_new() {
        let issue = ValidationIssue::new(ValidationSeverity::Error, "ERR001", "Test error");
        assert_eq!(issue.severity, ValidationSeverity::Error);
        assert_eq!(issue.code, "ERR001");
        assert_eq!(issue.message, "Test error");
    }

    #[test]
    fn test_validation_issue_with_location() {
        let issue = ValidationIssue::new(ValidationSeverity::Warning, "WARN001", "Test warning")
            .with_location("line 10");
        assert_eq!(issue.location, Some("line 10".to_string()));
    }

    #[test]
    fn test_validation_issue_with_suggestion() {
        let issue = ValidationIssue::new(ValidationSeverity::Info, "INFO001", "Test info")
            .with_suggestion("Fix this");
        assert_eq!(issue.suggestion, Some("Fix this".to_string()));
    }

    #[test]
    fn test_validation_report_new() {
        let report = ValidationReport::new(LegalFormat::Catala);
        assert_eq!(report.format, LegalFormat::Catala);
        assert!(report.passed);
        assert_eq!(report.error_count, 0);
    }

    #[test]
    fn test_validation_report_add_issue() {
        let mut report = ValidationReport::new(LegalFormat::L4);
        report.add_issue(ValidationIssue::new(
            ValidationSeverity::Error,
            "E1",
            "Error",
        ));

        assert!(!report.passed);
        assert_eq!(report.error_count, 1);
        assert_eq!(report.issues.len(), 1);
    }

    #[test]
    fn test_validation_report_warnings() {
        let mut report = ValidationReport::new(LegalFormat::AkomaNtoso);
        report.add_issue(ValidationIssue::new(
            ValidationSeverity::Warning,
            "W1",
            "Warning",
        ));

        assert!(report.passed); // Warnings don't fail validation
        assert_eq!(report.warning_count, 1);
        assert!(report.has_warnings());
    }

    #[test]
    fn test_schema_validator_xml() {
        let validator = SchemaValidator::new(LegalFormat::AkomaNtoso);
        let issues = validator.validate_xml("<test>content</test>");
        assert_eq!(issues.len(), 0); // Valid XML
    }

    #[test]
    fn test_schema_validator_xml_invalid() {
        let validator = SchemaValidator::new(LegalFormat::AkomaNtoso);
        let issues = validator.validate_xml("not xml");
        assert!(issues.len() > 0);
    }

    #[test]
    fn test_schema_validator_json() {
        let validator = SchemaValidator::new(LegalFormat::CommonForm);
        let issues = validator.validate_json(r#"{"key": "value"}"#);
        assert_eq!(issues.len(), 0); // Valid JSON
    }

    #[test]
    fn test_schema_validator_json_invalid() {
        let validator = SchemaValidator::new(LegalFormat::CommonForm);
        let issues = validator.validate_json("{invalid json}");
        assert!(issues.len() > 0);
    }

    #[test]
    fn test_semantic_validator() {
        let validator = SemanticValidator::new();
        let statute = Statute::new("", "Title", Effect::new(EffectType::Grant, "test"));
        let issues = validator.validate_statute(&statute);

        // Should have error for empty ID
        assert!(issues.iter().any(|i| i.code == "SEM001"));
    }

    #[test]
    fn test_semantic_validator_long_title() {
        let validator = SemanticValidator::new();
        let long_title = "A".repeat(250);
        let statute = Statute::new("test", &long_title, Effect::new(EffectType::Grant, "test"));
        let issues = validator.validate_statute(&statute);

        // Should have warning for long title
        assert!(issues.iter().any(|i| i.code == "SEM003"));
    }

    #[test]
    fn test_consistency_checker() {
        let reference = vec![Statute::new(
            "s1",
            "Statute 1",
            Effect::new(EffectType::Grant, "test"),
        )];
        let target = vec![Statute::new(
            "s1",
            "Statute 1",
            Effect::new(EffectType::Grant, "test"),
        )];

        let checker = ConsistencyChecker::new(reference);
        let issues = checker.check_consistency(&target);
        assert_eq!(issues.len(), 0); // Should be consistent
    }

    #[test]
    fn test_consistency_checker_missing() {
        let reference = vec![Statute::new(
            "s1",
            "Statute 1",
            Effect::new(EffectType::Grant, "test"),
        )];
        let target = vec![];

        let checker = ConsistencyChecker::new(reference);
        let issues = checker.check_consistency(&target);
        assert!(issues.len() > 0); // Should have issues
    }

    #[test]
    fn test_format_validator_new() {
        let validator = FormatValidator::new();
        assert!(validator.rules.len() > 0);
    }

    #[test]
    fn test_format_validator_validate_xml() {
        let validator = FormatValidator::new();
        let report = validator.validate("<test>content</test>", LegalFormat::AkomaNtoso);
        assert!(report.passed);
    }

    #[test]
    fn test_format_validator_validate_json() {
        let validator = FormatValidator::new();
        let report = validator.validate(r#"{"key": "value"}"#, LegalFormat::CommonForm);
        assert!(report.passed);
    }

    #[test]
    fn test_format_validator_with_statutes() {
        let validator = FormatValidator::new();
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "test"));
        let report =
            validator.validate_with_statutes("test source", LegalFormat::Catala, &[statute]);
        // Should pass (valid statute)
        assert!(report.passed);
    }

    #[test]
    fn test_validation_stats_new() {
        let stats = ValidationStats::new();
        assert_eq!(stats.total_validations, 0);
        assert_eq!(stats.success_rate(), 1.0);
    }

    #[test]
    fn test_validation_stats_record() {
        let mut stats = ValidationStats::new();
        let report = ValidationReport::new(LegalFormat::Catala);

        stats.record(&report);
        assert_eq!(stats.total_validations, 1);
        assert_eq!(stats.successful, 1);
    }

    #[test]
    fn test_validation_stats_success_rate() {
        let mut stats = ValidationStats::new();

        let mut report1 = ValidationReport::new(LegalFormat::Catala);
        stats.record(&report1);

        report1.add_issue(ValidationIssue::new(
            ValidationSeverity::Error,
            "E1",
            "Error",
        ));
        report1.passed = false;
        stats.record(&report1);

        assert_eq!(stats.success_rate(), 0.5);
    }

    #[test]
    fn test_validation_report_format() {
        let mut report = ValidationReport::new(LegalFormat::L4);
        report.add_issue(ValidationIssue::new(
            ValidationSeverity::Error,
            "E1",
            "Test error",
        ));

        let formatted = report.format();
        assert!(formatted.contains("ERROR"));
        assert!(formatted.contains("E1"));
    }

    #[test]
    fn test_format_stats() {
        let mut stats = FormatStats::new();
        let report = ValidationReport::new(LegalFormat::Catala);

        stats.record(&report);
        assert_eq!(stats.count, 1);
        assert_eq!(stats.successful, 1);
    }

    #[test]
    fn test_validation_severity_serialization() {
        let severity = ValidationSeverity::Error;
        let json = serde_json::to_string(&severity).unwrap();
        assert!(json.contains("Error"));
    }
}
