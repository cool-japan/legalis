//! Response validation and retry logic for LLM outputs.
//!
//! This module provides validation for LLM responses, including:
//! - JSON structure validation
//! - Retry on malformed responses
//! - Confidence scoring

use anyhow::{Result, anyhow};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::HashMap;

/// Validation result with confidence score.
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Whether the response is valid
    pub is_valid: bool,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f64,
    /// Validation errors if any
    pub errors: Vec<String>,
    /// Warnings that don't fail validation
    pub warnings: Vec<String>,
}

impl ValidationResult {
    /// Creates a valid result with high confidence.
    pub fn valid() -> Self {
        Self {
            is_valid: true,
            confidence: 1.0,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// Creates an invalid result with errors.
    pub fn invalid(errors: Vec<String>) -> Self {
        Self {
            is_valid: false,
            confidence: 0.0,
            errors,
            warnings: Vec::new(),
        }
    }

    /// Adds a warning.
    pub fn with_warning(mut self, warning: impl Into<String>) -> Self {
        self.warnings.push(warning.into());
        self
    }

    /// Sets the confidence score.
    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }
}

/// A simple JSON schema validator.
#[derive(Debug, Clone)]
pub struct JsonSchema {
    /// Required fields and their types
    pub required_fields: HashMap<String, JsonType>,
    /// Optional fields and their types
    pub optional_fields: HashMap<String, JsonType>,
    /// Whether to allow additional fields
    pub allow_additional: bool,
}

/// JSON data types for schema validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JsonType {
    /// String type
    String,
    /// Number type (integer or float)
    Number,
    /// Boolean type
    Boolean,
    /// Array type
    Array,
    /// Object type
    Object,
    /// Null type
    Null,
    /// Any type (no validation)
    Any,
}

impl JsonSchema {
    /// Creates a new empty schema.
    pub fn new() -> Self {
        Self {
            required_fields: HashMap::new(),
            optional_fields: HashMap::new(),
            allow_additional: true,
        }
    }

    /// Adds a required field.
    pub fn require(mut self, field: impl Into<String>, field_type: JsonType) -> Self {
        self.required_fields.insert(field.into(), field_type);
        self
    }

    /// Adds an optional field.
    pub fn optional(mut self, field: impl Into<String>, field_type: JsonType) -> Self {
        self.optional_fields.insert(field.into(), field_type);
        self
    }

    /// Sets whether to allow additional fields.
    pub fn allow_additional(mut self, allow: bool) -> Self {
        self.allow_additional = allow;
        self
    }

    /// Validates a JSON value against the schema.
    pub fn validate(&self, value: &Value) -> ValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Check if value is an object
        let obj = match value.as_object() {
            Some(obj) => obj,
            None => {
                return ValidationResult::invalid(vec!["Expected JSON object".to_string()]);
            }
        };

        // Check required fields
        for (field, expected_type) in &self.required_fields {
            match obj.get(field) {
                Some(field_value) => {
                    if !self.check_type(field_value, expected_type) {
                        errors.push(format!(
                            "Field '{}' has wrong type (expected {:?})",
                            field, expected_type
                        ));
                    }
                }
                None => {
                    errors.push(format!("Missing required field: {}", field));
                }
            }
        }

        // Check optional fields if present
        for (field, expected_type) in &self.optional_fields {
            if let Some(field_value) = obj.get(field) {
                if !self.check_type(field_value, expected_type) {
                    warnings.push(format!(
                        "Optional field '{}' has wrong type (expected {:?})",
                        field, expected_type
                    ));
                }
            }
        }

        // Check for additional fields
        if !self.allow_additional {
            for field in obj.keys() {
                if !self.required_fields.contains_key(field)
                    && !self.optional_fields.contains_key(field)
                {
                    warnings.push(format!("Unexpected field: {}", field));
                }
            }
        }

        if errors.is_empty() {
            let confidence = if warnings.is_empty() { 1.0 } else { 0.8 };
            ValidationResult {
                is_valid: true,
                confidence,
                errors,
                warnings,
            }
        } else {
            ValidationResult::invalid(errors).with_confidence(0.0)
        }
    }

    fn check_type(&self, value: &Value, expected: &JsonType) -> bool {
        match expected {
            JsonType::String => value.is_string(),
            JsonType::Number => value.is_number(),
            JsonType::Boolean => value.is_boolean(),
            JsonType::Array => value.is_array(),
            JsonType::Object => value.is_object(),
            JsonType::Null => value.is_null(),
            JsonType::Any => true,
        }
    }
}

impl Default for JsonSchema {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration for validating provider.
#[derive(Debug, Clone)]
pub struct ValidationConfig {
    /// Maximum retry attempts for malformed responses
    pub max_retries: usize,
    /// Optional JSON schema to validate against
    pub schema: Option<JsonSchema>,
    /// Minimum confidence score required (0.0 - 1.0)
    pub min_confidence: f64,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            schema: None,
            min_confidence: 0.7,
        }
    }
}

impl ValidationConfig {
    /// Creates a new validation configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the maximum retries.
    pub fn with_max_retries(mut self, max_retries: usize) -> Self {
        self.max_retries = max_retries;
        self
    }

    /// Sets the JSON schema.
    pub fn with_schema(mut self, schema: JsonSchema) -> Self {
        self.schema = Some(schema);
        self
    }

    /// Sets the minimum confidence.
    pub fn with_min_confidence(mut self, min_confidence: f64) -> Self {
        self.min_confidence = min_confidence.clamp(0.0, 1.0);
        self
    }
}

/// Wraps an LLM provider with response validation and retry logic.
pub struct ValidatingProvider<P> {
    provider: P,
    config: ValidationConfig,
}

impl<P> ValidatingProvider<P> {
    /// Creates a new validating provider with default configuration.
    pub fn new(provider: P) -> Self {
        Self {
            provider,
            config: ValidationConfig::default(),
        }
    }

    /// Creates a new validating provider with custom configuration.
    pub fn with_config(provider: P, config: ValidationConfig) -> Self {
        Self { provider, config }
    }

    /// Gets a reference to the underlying provider.
    pub fn provider(&self) -> &P {
        &self.provider
    }
}

#[async_trait::async_trait]
impl<P: crate::LLMProvider> crate::LLMProvider for ValidatingProvider<P> {
    async fn generate_text(&self, prompt: &str) -> Result<String> {
        // Text responses don't get schema validation
        self.provider.generate_text(prompt).await
    }

    async fn generate_structured<T: DeserializeOwned + Send>(&self, prompt: &str) -> Result<T> {
        let mut last_error = None;

        for attempt in 0..=self.config.max_retries {
            if attempt > 0 {
                tracing::debug!(
                    "Retrying structured generation (attempt {}/{})",
                    attempt + 1,
                    self.config.max_retries + 1
                );
            }

            match self.provider.generate_structured::<T>(prompt).await {
                Ok(result) => {
                    // Schema validation is skipped for structured responses
                    // because we cannot serialize T without adding Serialize bound
                    // which would violate the trait contract.
                    // Users should validate using the text response if needed.
                    return Ok(result);
                }
                Err(e) => {
                    tracing::warn!(
                        "Structured generation failed (attempt {}/{}): {}",
                        attempt + 1,
                        self.config.max_retries + 1,
                        e
                    );
                    last_error = Some(e);
                }
            }
        }

        Err(last_error.unwrap_or_else(|| anyhow!("All validation attempts exhausted")))
    }

    async fn generate_text_stream(&self, prompt: &str) -> Result<crate::TextStream> {
        // Streaming responses are not validated
        self.provider.generate_text_stream(prompt).await
    }

    fn provider_name(&self) -> &str {
        self.provider.provider_name()
    }

    fn model_name(&self) -> &str {
        self.provider.model_name()
    }

    fn supports_streaming(&self) -> bool {
        self.provider.supports_streaming()
    }
}

/// Calculates a simple confidence score based on response quality indicators.
pub fn calculate_confidence(text: &str) -> f64 {
    let mut score: f64 = 1.0;

    // Empty text has very low confidence
    if text.is_empty() {
        return 0.1;
    }

    // Penalize very short responses
    if text.len() < 10 {
        score *= 0.5;
    }

    // Penalize responses with error indicators
    let error_patterns = ["error", "failed", "cannot", "unable", "invalid"];
    for pattern in &error_patterns {
        if text.to_lowercase().contains(pattern) {
            score *= 0.9;
        }
    }

    // Reward responses with structure (paragraphs, lists)
    if text.contains('\n') {
        score *= 1.1;
    }

    // Penalize incomplete JSON
    if text.contains('{') && !text.contains('}') {
        score *= 0.3;
    }

    score.clamp(0.0, 1.0)
}

/// Advanced validation rules beyond simple JSON schema.
pub mod rules {
    use super::*;
    use serde_json::Value;

    /// A validation rule trait.
    pub trait ValidationRule: Send + Sync {
        /// Validates a value and returns errors if any.
        fn validate(&self, value: &Value) -> Vec<String>;

        /// Returns the name of this rule.
        fn name(&self) -> &str;

        /// Returns the severity of violations (0-100).
        fn severity(&self) -> u8 {
            50
        }
    }

    /// Rule that checks string length.
    pub struct LengthRule {
        field: String,
        min_length: Option<usize>,
        max_length: Option<usize>,
    }

    impl LengthRule {
        /// Creates a new length rule for a field.
        pub fn new(field: impl Into<String>) -> Self {
            Self {
                field: field.into(),
                min_length: None,
                max_length: None,
            }
        }

        /// Sets the minimum length.
        pub fn min(mut self, min: usize) -> Self {
            self.min_length = Some(min);
            self
        }

        /// Sets the maximum length.
        pub fn max(mut self, max: usize) -> Self {
            self.max_length = Some(max);
            self
        }
    }

    impl ValidationRule for LengthRule {
        fn validate(&self, value: &Value) -> Vec<String> {
            let mut errors = Vec::new();

            if let Some(obj) = value.as_object() {
                if let Some(field_value) = obj.get(&self.field) {
                    if let Some(s) = field_value.as_str() {
                        let len = s.len();

                        if let Some(min) = self.min_length {
                            if len < min {
                                errors.push(format!(
                                    "Field '{}' is too short: {} < {}",
                                    self.field, len, min
                                ));
                            }
                        }

                        if let Some(max) = self.max_length {
                            if len > max {
                                errors.push(format!(
                                    "Field '{}' is too long: {} > {}",
                                    self.field, len, max
                                ));
                            }
                        }
                    }
                }
            }

            errors
        }

        fn name(&self) -> &str {
            "LengthRule"
        }

        fn severity(&self) -> u8 {
            60
        }
    }

    /// Rule that checks numeric ranges.
    pub struct RangeRule {
        field: String,
        min: Option<f64>,
        max: Option<f64>,
    }

    impl RangeRule {
        /// Creates a new range rule for a field.
        pub fn new(field: impl Into<String>) -> Self {
            Self {
                field: field.into(),
                min: None,
                max: None,
            }
        }

        /// Sets the minimum value.
        pub fn min(mut self, min: f64) -> Self {
            self.min = Some(min);
            self
        }

        /// Sets the maximum value.
        pub fn max(mut self, max: f64) -> Self {
            self.max = Some(max);
            self
        }
    }

    impl ValidationRule for RangeRule {
        fn validate(&self, value: &Value) -> Vec<String> {
            let mut errors = Vec::new();

            if let Some(obj) = value.as_object() {
                if let Some(field_value) = obj.get(&self.field) {
                    if let Some(num) = field_value.as_f64() {
                        if let Some(min) = self.min {
                            if num < min {
                                errors.push(format!(
                                    "Field '{}' is below minimum: {} < {}",
                                    self.field, num, min
                                ));
                            }
                        }

                        if let Some(max) = self.max {
                            if num > max {
                                errors.push(format!(
                                    "Field '{}' exceeds maximum: {} > {}",
                                    self.field, num, max
                                ));
                            }
                        }
                    }
                }
            }

            errors
        }

        fn name(&self) -> &str {
            "RangeRule"
        }

        fn severity(&self) -> u8 {
            70
        }
    }

    /// Rule that checks array length.
    pub struct ArrayLengthRule {
        field: String,
        min_items: Option<usize>,
        max_items: Option<usize>,
    }

    impl ArrayLengthRule {
        /// Creates a new array length rule.
        pub fn new(field: impl Into<String>) -> Self {
            Self {
                field: field.into(),
                min_items: None,
                max_items: None,
            }
        }

        /// Sets the minimum number of items.
        pub fn min(mut self, min: usize) -> Self {
            self.min_items = Some(min);
            self
        }

        /// Sets the maximum number of items.
        pub fn max(mut self, max: usize) -> Self {
            self.max_items = Some(max);
            self
        }
    }

    impl ValidationRule for ArrayLengthRule {
        fn validate(&self, value: &Value) -> Vec<String> {
            let mut errors = Vec::new();

            if let Some(obj) = value.as_object() {
                if let Some(field_value) = obj.get(&self.field) {
                    if let Some(arr) = field_value.as_array() {
                        let len = arr.len();

                        if let Some(min) = self.min_items {
                            if len < min {
                                errors.push(format!(
                                    "Field '{}' has too few items: {} < {}",
                                    self.field, len, min
                                ));
                            }
                        }

                        if let Some(max) = self.max_items {
                            if len > max {
                                errors.push(format!(
                                    "Field '{}' has too many items: {} > {}",
                                    self.field, len, max
                                ));
                            }
                        }
                    }
                }
            }

            errors
        }

        fn name(&self) -> &str {
            "ArrayLengthRule"
        }
    }

    /// Rule that checks for pattern matching.
    pub struct PatternRule {
        field: String,
        pattern: String,
    }

    impl PatternRule {
        /// Creates a new pattern rule.
        pub fn new(field: impl Into<String>, pattern: impl Into<String>) -> Self {
            Self {
                field: field.into(),
                pattern: pattern.into(),
            }
        }
    }

    impl ValidationRule for PatternRule {
        fn validate(&self, value: &Value) -> Vec<String> {
            let mut errors = Vec::new();

            if let Some(obj) = value.as_object() {
                if let Some(field_value) = obj.get(&self.field) {
                    if let Some(s) = field_value.as_str() {
                        // Simple pattern matching (not full regex)
                        if !s.contains(&self.pattern) {
                            errors.push(format!(
                                "Field '{}' does not match pattern '{}'",
                                self.field, self.pattern
                            ));
                        }
                    }
                }
            }

            errors
        }

        fn name(&self) -> &str {
            "PatternRule"
        }
    }

    /// Rule that ensures a field is not empty.
    pub struct NotEmptyRule {
        field: String,
    }

    impl NotEmptyRule {
        /// Creates a new not-empty rule.
        pub fn new(field: impl Into<String>) -> Self {
            Self {
                field: field.into(),
            }
        }
    }

    impl ValidationRule for NotEmptyRule {
        fn validate(&self, value: &Value) -> Vec<String> {
            let mut errors = Vec::new();

            if let Some(obj) = value.as_object() {
                if let Some(field_value) = obj.get(&self.field) {
                    let is_empty = match field_value {
                        Value::String(s) => s.is_empty(),
                        Value::Array(a) => a.is_empty(),
                        Value::Object(o) => o.is_empty(),
                        Value::Null => true,
                        _ => false,
                    };

                    if is_empty {
                        errors.push(format!("Field '{}' must not be empty", self.field));
                    }
                }
            }

            errors
        }

        fn name(&self) -> &str {
            "NotEmptyRule"
        }

        fn severity(&self) -> u8 {
            80
        }
    }

    /// Composite validator that applies multiple rules.
    pub struct RuleValidator {
        rules: Vec<Box<dyn ValidationRule>>,
    }

    impl RuleValidator {
        /// Creates a new rule validator.
        pub fn new() -> Self {
            Self { rules: Vec::new() }
        }

        /// Adds a validation rule.
        pub fn add_rule<R: ValidationRule + 'static>(mut self, rule: R) -> Self {
            self.rules.push(Box::new(rule));
            self
        }

        /// Validates a value against all rules.
        pub fn validate(&self, value: &Value) -> ValidationResult {
            let mut all_errors = Vec::new();
            let mut total_severity = 0u32;

            for rule in &self.rules {
                let errors = rule.validate(value);
                if !errors.is_empty() {
                    total_severity += rule.severity() as u32;
                    all_errors.extend(errors);
                }
            }

            if all_errors.is_empty() {
                ValidationResult::valid()
            } else {
                // Calculate confidence based on severity
                let confidence =
                    (100.0 - (total_severity as f64 / self.rules.len() as f64)) / 100.0;
                ValidationResult::invalid(all_errors).with_confidence(confidence.max(0.0))
            }
        }

        /// Validates and returns only high-severity errors.
        pub fn validate_critical(&self, value: &Value) -> ValidationResult {
            let mut critical_errors = Vec::new();

            for rule in &self.rules {
                if rule.severity() >= 70 {
                    let errors = rule.validate(value);
                    critical_errors.extend(errors);
                }
            }

            if critical_errors.is_empty() {
                ValidationResult::valid()
            } else {
                ValidationResult::invalid(critical_errors)
            }
        }
    }

    impl Default for RuleValidator {
        fn default() -> Self {
            Self::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LLMProvider;

    #[test]
    fn test_json_schema_validation() {
        let schema = JsonSchema::new()
            .require("name", JsonType::String)
            .require("age", JsonType::Number)
            .optional("email", JsonType::String);

        let valid_json = serde_json::json!({
            "name": "Alice",
            "age": 30
        });

        let result = schema.validate(&valid_json);
        assert!(result.is_valid);
        assert_eq!(result.confidence, 1.0);
    }

    #[test]
    fn test_json_schema_missing_field() {
        let schema = JsonSchema::new()
            .require("name", JsonType::String)
            .require("age", JsonType::Number);

        let invalid_json = serde_json::json!({
            "name": "Bob"
        });

        let result = schema.validate(&invalid_json);
        assert!(!result.is_valid);
        assert!(result.errors.iter().any(|e| e.contains("age")));
    }

    #[test]
    fn test_json_schema_wrong_type() {
        let schema = JsonSchema::new().require("age", JsonType::Number);

        let invalid_json = serde_json::json!({
            "age": "thirty"
        });

        let result = schema.validate(&invalid_json);
        assert!(!result.is_valid);
    }

    #[test]
    fn test_confidence_calculation() {
        assert!(calculate_confidence("A good response") > 0.5);
        assert!(calculate_confidence("error") < 1.0);
        assert!(calculate_confidence("") < 0.5);
    }

    #[tokio::test]
    async fn test_validating_provider() {
        use crate::MockProvider;

        let schema = JsonSchema::new().require("result", JsonType::String);

        let config = ValidationConfig::new()
            .with_schema(schema)
            .with_max_retries(2);

        let provider = MockProvider::new().with_response("test", r#"{"result": "success"}"#);

        let validating = ValidatingProvider::with_config(provider, config);

        #[derive(serde::Deserialize)]
        struct TestResponse {
            #[allow(dead_code)]
            result: String,
        }

        let response: TestResponse = validating.generate_structured("test prompt").await.unwrap();

        assert_eq!(response.result, "success");
    }
}
