//! Advanced error handling module.
//!
//! This module provides:
//! - Graceful degradation for unsupported features
//! - Partial conversion with warnings
//! - Error recovery strategies
//! - Interactive error resolution
//! - Error pattern analysis

use crate::{ConversionReport, InteropError, InteropResult, LegalConverter, LegalFormat, Statute};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Error recovery strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum RecoveryStrategy {
    /// Skip the problematic element and continue
    #[default]
    Skip,
    /// Use a default/fallback value
    UseDefault,
    /// Try an alternative conversion method
    TryAlternative,
    /// Ask user for resolution (interactive mode)
    AskUser,
    /// Abort the entire conversion
    Abort,
}

/// Configuration for error handling behavior.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorHandlingConfig {
    /// Default recovery strategy
    pub default_strategy: RecoveryStrategy,
    /// Per-error-type strategies
    pub strategy_map: HashMap<String, RecoveryStrategy>,
    /// Whether to collect detailed error information
    pub collect_diagnostics: bool,
    /// Maximum number of errors before aborting
    pub max_errors: Option<usize>,
    /// Whether to attempt partial conversion
    pub allow_partial: bool,
}

impl Default for ErrorHandlingConfig {
    fn default() -> Self {
        Self {
            default_strategy: RecoveryStrategy::Skip,
            strategy_map: HashMap::new(),
            collect_diagnostics: true,
            max_errors: None,
            allow_partial: true,
        }
    }
}

impl ErrorHandlingConfig {
    /// Creates a new configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the default recovery strategy.
    pub fn with_default_strategy(mut self, strategy: RecoveryStrategy) -> Self {
        self.default_strategy = strategy;
        self
    }

    /// Sets a recovery strategy for a specific error type.
    pub fn with_strategy(
        mut self,
        error_type: impl Into<String>,
        strategy: RecoveryStrategy,
    ) -> Self {
        self.strategy_map.insert(error_type.into(), strategy);
        self
    }

    /// Sets whether to collect diagnostics.
    pub fn with_diagnostics(mut self, collect: bool) -> Self {
        self.collect_diagnostics = collect;
        self
    }

    /// Sets the maximum number of errors before aborting.
    pub fn with_max_errors(mut self, max: usize) -> Self {
        self.max_errors = Some(max);
        self
    }

    /// Sets whether to allow partial conversions.
    pub fn with_partial(mut self, allow: bool) -> Self {
        self.allow_partial = allow;
        self
    }

    /// Returns the strategy for a given error type.
    pub fn strategy_for(&self, error_type: &str) -> RecoveryStrategy {
        self.strategy_map
            .get(error_type)
            .copied()
            .unwrap_or(self.default_strategy)
    }
}

/// Detailed error information with recovery context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedError {
    /// Error type/category
    pub error_type: String,
    /// Human-readable error message
    pub message: String,
    /// Context where the error occurred
    pub context: Option<String>,
    /// Source location (statute ID, line number, etc.)
    pub location: Option<String>,
    /// Suggested recovery strategy
    pub suggested_strategy: RecoveryStrategy,
    /// Possible fixes or alternatives
    pub suggestions: Vec<String>,
    /// Severity level (0-10, where 10 is critical)
    pub severity: u8,
}

impl DetailedError {
    /// Creates a new detailed error.
    pub fn new(error_type: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            error_type: error_type.into(),
            message: message.into(),
            context: None,
            location: None,
            suggested_strategy: RecoveryStrategy::Skip,
            suggestions: Vec::new(),
            severity: 5,
        }
    }

    /// Sets the context.
    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }

    /// Sets the location.
    pub fn with_location(mut self, location: impl Into<String>) -> Self {
        self.location = Some(location.into());
        self
    }

    /// Sets the suggested strategy.
    pub fn with_strategy(mut self, strategy: RecoveryStrategy) -> Self {
        self.suggested_strategy = strategy;
        self
    }

    /// Adds a suggestion.
    pub fn add_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestions.push(suggestion.into());
        self
    }

    /// Sets the severity.
    pub fn with_severity(mut self, severity: u8) -> Self {
        self.severity = severity.min(10);
        self
    }
}

/// Result of a partial conversion.
#[derive(Debug, Clone)]
pub struct PartialConversionResult {
    /// Successfully converted statutes
    pub converted_statutes: Vec<Statute>,
    /// Conversion report
    pub report: ConversionReport,
    /// Detailed errors encountered
    pub errors: Vec<DetailedError>,
    /// Whether the conversion was complete or partial
    pub is_complete: bool,
    /// Output (if any was generated)
    pub output: Option<String>,
}

impl PartialConversionResult {
    /// Creates a new partial conversion result.
    pub fn new() -> Self {
        Self {
            converted_statutes: Vec::new(),
            report: ConversionReport::default(),
            errors: Vec::new(),
            is_complete: true,
            output: None,
        }
    }

    /// Returns the success rate (0.0 - 1.0).
    pub fn success_rate(&self) -> f64 {
        if self.is_complete {
            return 1.0;
        }
        if self.converted_statutes.is_empty() && !self.errors.is_empty() {
            return 0.0;
        }
        let total = self.converted_statutes.len() + self.errors.len();
        if total == 0 {
            return 1.0;
        }
        self.converted_statutes.len() as f64 / total as f64
    }

    /// Returns critical errors (severity >= 8).
    pub fn critical_errors(&self) -> Vec<&DetailedError> {
        self.errors.iter().filter(|e| e.severity >= 8).collect()
    }

    /// Returns warnings (severity < 5).
    pub fn warnings(&self) -> Vec<&DetailedError> {
        self.errors.iter().filter(|e| e.severity < 5).collect()
    }
}

impl Default for PartialConversionResult {
    fn default() -> Self {
        Self::new()
    }
}

/// Callback for interactive error resolution.
pub type ErrorResolutionCallback = Box<dyn Fn(&DetailedError) -> RecoveryStrategy + Send + Sync>;

/// Converter with advanced error handling.
pub struct ResilientConverter {
    converter: LegalConverter,
    config: ErrorHandlingConfig,
    error_callback: Option<ErrorResolutionCallback>,
}

impl ResilientConverter {
    /// Creates a new resilient converter.
    pub fn new(converter: LegalConverter, config: ErrorHandlingConfig) -> Self {
        Self {
            converter,
            config,
            error_callback: None,
        }
    }

    /// Creates a resilient converter with default configuration.
    pub fn with_defaults(converter: LegalConverter) -> Self {
        Self::new(converter, ErrorHandlingConfig::default())
    }

    /// Sets an error resolution callback for interactive mode.
    pub fn with_callback<F>(mut self, callback: F) -> Self
    where
        F: Fn(&DetailedError) -> RecoveryStrategy + Send + Sync + 'static,
    {
        self.error_callback = Some(Box::new(callback));
        self
    }

    /// Attempts a partial import with error recovery.
    pub fn import_partial(
        &mut self,
        source: &str,
        format: LegalFormat,
    ) -> InteropResult<PartialConversionResult> {
        let mut result = PartialConversionResult::new();

        // Try normal import first
        match self.converter.import(source, format) {
            Ok((statutes, report)) => {
                result.converted_statutes = statutes;
                result.report = report;
                result.is_complete = true;
                Ok(result)
            }
            Err(e) => {
                if !self.config.allow_partial {
                    return Err(e);
                }

                // Create detailed error
                let detailed_error = DetailedError::new("import_error", e.to_string())
                    .with_context(format!("Importing {:?} format", format))
                    .with_strategy(self.config.default_strategy)
                    .with_severity(8);

                // Try recovery strategies
                let strategy = self.resolve_error(&detailed_error);

                match strategy {
                    RecoveryStrategy::Abort => Err(e),
                    RecoveryStrategy::Skip => {
                        result.errors.push(detailed_error);
                        result.is_complete = false;
                        Ok(result)
                    }
                    RecoveryStrategy::TryAlternative => {
                        // Try auto-detection as alternative
                        match self.converter.auto_import(source) {
                            Ok((statutes, report)) => {
                                result.converted_statutes = statutes;
                                result.report = report;
                                result.is_complete = true;
                                Ok(result)
                            }
                            Err(_) => {
                                result.errors.push(detailed_error);
                                result.is_complete = false;
                                Ok(result)
                            }
                        }
                    }
                    RecoveryStrategy::UseDefault => {
                        // Return empty result with error
                        result.errors.push(detailed_error);
                        result.is_complete = false;
                        Ok(result)
                    }
                    RecoveryStrategy::AskUser => {
                        // Already handled by resolve_error
                        result.errors.push(detailed_error);
                        result.is_complete = false;
                        Ok(result)
                    }
                }
            }
        }
    }

    /// Attempts a partial export with error recovery.
    pub fn export_partial(
        &mut self,
        statutes: &[Statute],
        format: LegalFormat,
    ) -> InteropResult<PartialConversionResult> {
        let mut result = PartialConversionResult::new();
        let mut converted = Vec::new();
        let mut errors = Vec::new();

        if !self.config.allow_partial {
            // Try all-or-nothing conversion
            match self.converter.export(statutes, format) {
                Ok((output, report)) => {
                    result.converted_statutes = statutes.to_vec();
                    result.report = report;
                    result.output = Some(output);
                    result.is_complete = true;
                    return Ok(result);
                }
                Err(e) => return Err(e),
            }
        }

        // Try converting each statute individually
        for (idx, statute) in statutes.iter().enumerate() {
            match self.converter.export(std::slice::from_ref(statute), format) {
                Ok((output, report)) => {
                    converted.push(statute.clone());
                    // Merge output (simplified - in real implementation would be more sophisticated)
                    if result.output.is_none() {
                        result.output = Some(output);
                    } else {
                        // Append to existing output
                        result.output =
                            Some(format!("{}\n\n{}", result.output.as_ref().unwrap(), output));
                    }
                    // Merge report
                    result.report.statutes_converted += report.statutes_converted;
                    result.report.warnings.extend(report.warnings);
                    result
                        .report
                        .unsupported_features
                        .extend(report.unsupported_features);
                }
                Err(e) => {
                    let detailed_error = DetailedError::new(
                        "export_error",
                        format!("Failed to export statute: {}", e),
                    )
                    .with_context(format!("Exporting to {:?} format", format))
                    .with_location(format!("Statute #{}: {}", idx, statute.id))
                    .with_strategy(RecoveryStrategy::Skip)
                    .with_severity(6);

                    let strategy = self.resolve_error(&detailed_error);

                    match strategy {
                        RecoveryStrategy::Abort => {
                            return Err(InteropError::ConversionError(format!(
                                "Conversion aborted at statute #{}: {}",
                                idx, e
                            )));
                        }
                        RecoveryStrategy::Skip => {
                            errors.push(detailed_error);
                            result
                                .report
                                .add_warning(format!("Skipped statute #{}: {}", idx, statute.id));
                        }
                        _ => {
                            errors.push(detailed_error);
                        }
                    }
                }
            }

            // Check max errors
            if let Some(max) = self.config.max_errors {
                if errors.len() >= max {
                    return Err(InteropError::ConversionError(format!(
                        "Maximum error count ({}) exceeded",
                        max
                    )));
                }
            }
        }

        result.converted_statutes = converted;
        result.errors = errors;
        result.is_complete = result.errors.is_empty();

        Ok(result)
    }

    /// Converts with partial conversion support.
    pub fn convert_partial(
        &mut self,
        source: &str,
        from: LegalFormat,
        to: LegalFormat,
    ) -> InteropResult<PartialConversionResult> {
        // Import with error recovery
        let import_result = self.import_partial(source, from)?;

        if import_result.converted_statutes.is_empty() {
            return Ok(import_result);
        }

        // Export with error recovery
        let mut export_result = self.export_partial(&import_result.converted_statutes, to)?;

        // Merge errors
        export_result.errors.extend(import_result.errors);

        Ok(export_result)
    }

    /// Resolves an error using the configured strategy and optional callback.
    fn resolve_error(&self, error: &DetailedError) -> RecoveryStrategy {
        // First, check if we have an interactive callback
        if let Some(ref callback) = self.error_callback {
            return callback(error);
        }

        // Otherwise, use configured strategy
        self.config.strategy_for(&error.error_type)
    }

    /// Returns a reference to the inner converter.
    pub fn converter(&self) -> &LegalConverter {
        &self.converter
    }

    /// Returns a mutable reference to the inner converter.
    pub fn converter_mut(&mut self) -> &mut LegalConverter {
        &mut self.converter
    }
}

/// Error pattern analyzer for detecting common issues.
pub struct ErrorPatternAnalyzer {
    patterns: Vec<ErrorPattern>,
}

impl ErrorPatternAnalyzer {
    /// Creates a new analyzer with default patterns.
    pub fn new() -> Self {
        Self {
            patterns: vec![
                ErrorPattern::new(
                    "unsupported_format",
                    vec!["format", "not supported", "unknown"],
                    "Check if the file format is correct and supported",
                ),
                ErrorPattern::new(
                    "parse_error",
                    vec!["parse", "syntax", "unexpected"],
                    "Verify that the input file is well-formed and follows the format specification",
                ),
                ErrorPattern::new(
                    "missing_field",
                    vec!["missing", "required", "field"],
                    "Ensure all required fields are present in the source document",
                ),
                ErrorPattern::new(
                    "type_mismatch",
                    vec!["type", "expected", "got"],
                    "Check that field types match the expected schema",
                ),
                ErrorPattern::new(
                    "encoding_issue",
                    vec!["encoding", "utf", "invalid character"],
                    "Try converting the file to UTF-8 encoding",
                ),
            ],
        }
    }

    /// Analyzes an error and returns matching patterns.
    pub fn analyze(&self, error: &InteropError) -> Vec<&ErrorPattern> {
        let error_text = error.to_string().to_lowercase();
        self.patterns
            .iter()
            .filter(|p| p.matches(&error_text))
            .collect()
    }

    /// Analyzes a detailed error and enhances it with suggestions.
    pub fn enhance_error(&self, mut error: DetailedError) -> DetailedError {
        let error_text = format!("{} {}", error.error_type, error.message).to_lowercase();

        for pattern in &self.patterns {
            if pattern.matches(&error_text) {
                error.suggestions.push(pattern.suggestion.clone());
            }
        }

        error
    }

    /// Adds a custom error pattern.
    pub fn add_pattern(&mut self, pattern: ErrorPattern) {
        self.patterns.push(pattern);
    }
}

impl Default for ErrorPatternAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// A pattern for recognizing and suggesting fixes for common errors.
#[derive(Debug, Clone)]
pub struct ErrorPattern {
    /// Pattern identifier
    pub id: String,
    /// Keywords to match in error messages
    pub keywords: Vec<String>,
    /// Suggestion for fixing this error
    pub suggestion: String,
}

impl ErrorPattern {
    /// Creates a new error pattern.
    pub fn new(
        id: impl Into<String>,
        keywords: Vec<impl Into<String>>,
        suggestion: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            keywords: keywords.into_iter().map(|k| k.into()).collect(),
            suggestion: suggestion.into(),
        }
    }

    /// Returns true if this pattern matches the error text.
    pub fn matches(&self, error_text: &str) -> bool {
        let error_lower = error_text.to_lowercase();
        self.keywords
            .iter()
            .all(|k| error_lower.contains(&k.to_lowercase()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType};

    #[test]
    fn test_error_handling_config() {
        let config = ErrorHandlingConfig::new()
            .with_default_strategy(RecoveryStrategy::Skip)
            .with_strategy("critical_error", RecoveryStrategy::Abort)
            .with_max_errors(10)
            .with_partial(true);

        assert_eq!(config.default_strategy, RecoveryStrategy::Skip);
        assert_eq!(
            config.strategy_for("critical_error"),
            RecoveryStrategy::Abort
        );
        assert_eq!(config.strategy_for("unknown_error"), RecoveryStrategy::Skip);
        assert_eq!(config.max_errors, Some(10));
        assert!(config.allow_partial);
    }

    #[test]
    fn test_detailed_error() {
        let error = DetailedError::new("parse_error", "Invalid syntax")
            .with_context("Parsing Catala source")
            .with_location("line 42")
            .with_strategy(RecoveryStrategy::Skip)
            .add_suggestion("Check for missing semicolons")
            .with_severity(7);

        assert_eq!(error.error_type, "parse_error");
        assert_eq!(error.message, "Invalid syntax");
        assert_eq!(error.context, Some("Parsing Catala source".to_string()));
        assert_eq!(error.location, Some("line 42".to_string()));
        assert_eq!(error.suggested_strategy, RecoveryStrategy::Skip);
        assert_eq!(error.suggestions.len(), 1);
        assert_eq!(error.severity, 7);
    }

    #[test]
    fn test_partial_conversion_result() {
        let mut result = PartialConversionResult::new();
        assert!(result.is_complete);
        assert_eq!(result.success_rate(), 1.0);

        result.is_complete = false;
        result.converted_statutes.push(Statute::new(
            "test",
            "Test",
            Effect::new(EffectType::Grant, "test"),
        ));
        result
            .errors
            .push(DetailedError::new("error", "Failed").with_severity(9));

        assert_eq!(result.success_rate(), 0.5);
        assert_eq!(result.critical_errors().len(), 1);
        assert_eq!(result.warnings().len(), 0);
    }

    #[test]
    fn test_resilient_converter_import() {
        let converter = LegalConverter::new();
        let config = ErrorHandlingConfig::new().with_partial(true);
        let mut resilient = ResilientConverter::new(converter, config);

        // Valid import should work
        let catala_source = "declaration scope Test:\n  context input content integer";
        let result = resilient
            .import_partial(catala_source, LegalFormat::Catala)
            .unwrap();

        assert!(result.is_complete);
        assert!(!result.converted_statutes.is_empty());
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_resilient_converter_export() {
        let converter = LegalConverter::new();
        let config = ErrorHandlingConfig::new().with_partial(true);
        let mut resilient = ResilientConverter::new(converter, config);

        let statute = Statute::new(
            "test",
            "Test Statute",
            Effect::new(EffectType::Grant, "test"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let result = resilient
            .export_partial(&[statute], LegalFormat::L4)
            .unwrap();

        assert!(result.is_complete);
        assert_eq!(result.converted_statutes.len(), 1);
        assert!(result.output.is_some());
    }

    #[test]
    fn test_resilient_converter_partial_export() {
        let converter = LegalConverter::new();
        let config = ErrorHandlingConfig::new().with_partial(true);
        let mut resilient = ResilientConverter::new(converter, config);

        // Create multiple statutes
        let statutes = vec![
            Statute::new("test1", "Test 1", Effect::new(EffectType::Grant, "test1")),
            Statute::new("test2", "Test 2", Effect::new(EffectType::Grant, "test2")),
        ];

        let result = resilient
            .export_partial(&statutes, LegalFormat::L4)
            .unwrap();

        assert!(result.is_complete || !result.converted_statutes.is_empty());
        assert!(result.output.is_some());
    }

    #[test]
    fn test_error_pattern_matching() {
        let pattern = ErrorPattern::new("parse_error", vec!["parse", "syntax"], "Check syntax");

        assert!(pattern.matches("Parse error: unexpected syntax"));
        assert!(pattern.matches("parse syntax error"));
        assert!(!pattern.matches("Unknown error"));
        assert!(!pattern.matches("Only parse error")); // Missing "syntax"
        assert!(!pattern.matches("Only syntax error")); // Missing "parse"
    }

    #[test]
    fn test_error_pattern_analyzer() {
        let analyzer = ErrorPatternAnalyzer::new();

        let error = InteropError::ParseError("Parse error: unexpected syntax".to_string());
        let patterns = analyzer.analyze(&error);

        assert!(!patterns.is_empty());
        assert!(patterns.iter().any(|p| p.id == "parse_error"));
    }

    #[test]
    fn test_error_pattern_analyzer_enhance() {
        let analyzer = ErrorPatternAnalyzer::new();

        let error =
            DetailedError::new("parse_error", "Parse error: unexpected syntax").with_severity(7);

        let enhanced = analyzer.enhance_error(error);

        assert!(!enhanced.suggestions.is_empty());
        assert!(
            enhanced
                .suggestions
                .iter()
                .any(|s| s.contains("well-formed"))
        );
    }

    #[test]
    fn test_resilient_converter_with_callback() {
        let converter = LegalConverter::new();
        let config = ErrorHandlingConfig::new();

        let resilient = ResilientConverter::new(converter, config).with_callback(|error| {
            if error.severity >= 8 {
                RecoveryStrategy::Abort
            } else {
                RecoveryStrategy::Skip
            }
        });

        assert!(resilient.error_callback.is_some());
    }

    #[test]
    fn test_custom_error_pattern() {
        let mut analyzer = ErrorPatternAnalyzer::new();

        analyzer.add_pattern(ErrorPattern::new(
            "custom_error",
            vec!["custom", "specific"],
            "Try custom fix",
        ));

        let error = DetailedError::new("custom_error", "Custom specific error");
        let enhanced = analyzer.enhance_error(error);

        assert!(
            enhanced
                .suggestions
                .iter()
                .any(|s| s.contains("custom fix"))
        );
    }

    #[test]
    fn test_partial_result_critical_warnings() {
        let mut result = PartialConversionResult::new();

        result
            .errors
            .push(DetailedError::new("error1", "Critical").with_severity(9));
        result
            .errors
            .push(DetailedError::new("error2", "Warning").with_severity(3));
        result
            .errors
            .push(DetailedError::new("error3", "Normal").with_severity(5));

        assert_eq!(result.critical_errors().len(), 1);
        assert_eq!(result.warnings().len(), 1);
    }

    #[test]
    fn test_max_errors_limit() {
        let converter = LegalConverter::new();
        let config = ErrorHandlingConfig::new()
            .with_partial(true)
            .with_max_errors(2);

        let mut resilient = ResilientConverter::new(converter, config);

        // This should succeed with the existing statutes
        let statutes = vec![
            Statute::new("test1", "Test 1", Effect::new(EffectType::Grant, "test1")),
            Statute::new("test2", "Test 2", Effect::new(EffectType::Grant, "test2")),
        ];

        let result = resilient.export_partial(&statutes, LegalFormat::L4);
        assert!(result.is_ok());
    }
}
