//! Legal compliance and safety features.
//!
//! This module provides features for legal disclaimer injection, jurisdiction-aware
//! safety filters, unauthorized practice of law detection, confidentiality protection,
//! and audit logging for legal LLM operations.

use crate::{LLMProvider, legal::Jurisdiction};
use anyhow::{Context, Result, anyhow};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

/// Legal disclaimer to be added to LLM responses.
#[derive(Debug, Clone)]
pub struct LegalDisclaimer {
    /// Disclaimer text
    pub text: String,
    /// Whether to prepend or append the disclaimer
    pub position: DisclaimerPosition,
    /// Whether the disclaimer is required
    pub required: bool,
}

/// Position of the disclaimer in the response.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisclaimerPosition {
    /// Prepend to the beginning of the response
    Prepend,
    /// Append to the end of the response
    Append,
}

impl Default for LegalDisclaimer {
    fn default() -> Self {
        Self {
            text: "DISCLAIMER: This response is for informational purposes only and does not constitute legal advice. Please consult with a qualified attorney for legal advice specific to your situation.".to_string(),
            position: DisclaimerPosition::Append,
            required: true,
        }
    }
}

impl LegalDisclaimer {
    /// Creates a new legal disclaimer.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            position: DisclaimerPosition::Append,
            required: true,
        }
    }

    /// Sets the position of the disclaimer.
    pub fn with_position(mut self, position: DisclaimerPosition) -> Self {
        self.position = position;
        self
    }

    /// Sets whether the disclaimer is required.
    pub fn with_required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    /// Applies the disclaimer to text.
    pub fn apply(&self, text: &str) -> String {
        match self.position {
            DisclaimerPosition::Prepend => format!("{}\n\n{}", self.text, text),
            DisclaimerPosition::Append => format!("{}\n\n{}", text, self.text),
        }
    }
}

/// Audit log entry for LLM operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    /// Unique identifier for this entry
    pub id: String,
    /// Timestamp of the operation
    pub timestamp: DateTime<Utc>,
    /// User or system that made the request
    pub user: String,
    /// Operation type (e.g., "generate_text", "analyze_statute")
    pub operation: String,
    /// Input prompt or query (may be redacted)
    pub input: String,
    /// Output response (may be redacted)
    pub output: String,
    /// Jurisdiction context
    pub jurisdiction: Option<String>,
    /// Compliance flags or notes
    pub compliance_notes: Vec<String>,
    /// Whether the operation passed safety checks
    pub safety_passed: bool,
}

/// Audit logger for tracking LLM operations.
#[derive(Clone)]
pub struct AuditLogger {
    entries: Arc<Mutex<Vec<AuditLogEntry>>>,
    enabled: bool,
}

impl AuditLogger {
    /// Creates a new audit logger.
    pub fn new() -> Self {
        Self {
            entries: Arc::new(Mutex::new(Vec::new())),
            enabled: true,
        }
    }

    /// Enables or disables audit logging.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Logs an LLM operation.
    pub fn log(
        &self,
        user: impl Into<String>,
        operation: impl Into<String>,
        input: impl Into<String>,
        output: impl Into<String>,
        jurisdiction: Option<String>,
        safety_passed: bool,
    ) {
        if !self.enabled {
            return;
        }

        let entry = AuditLogEntry {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            user: user.into(),
            operation: operation.into(),
            input: input.into(),
            output: output.into(),
            jurisdiction,
            compliance_notes: Vec::new(),
            safety_passed,
        };

        if let Ok(mut entries) = self.entries.lock() {
            entries.push(entry);
        }
    }

    /// Retrieves all audit log entries.
    pub fn get_entries(&self) -> Vec<AuditLogEntry> {
        self.entries.lock().unwrap().clone()
    }

    /// Clears all audit log entries.
    pub fn clear(&self) {
        if let Ok(mut entries) = self.entries.lock() {
            entries.clear();
        }
    }

    /// Gets the number of audit log entries.
    pub fn entry_count(&self) -> usize {
        self.entries.lock().unwrap().len()
    }
}

impl Default for AuditLogger {
    fn default() -> Self {
        Self::new()
    }
}

/// Unauthorized practice of law (UPL) detector.
pub struct UplDetector<P> {
    provider: P,
    strictness: UplStrictness,
}

/// Strictness level for UPL detection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UplStrictness {
    /// Low strictness - only flag obvious UPL violations
    Low,
    /// Medium strictness - flag likely UPL violations
    Medium,
    /// High strictness - flag any potential UPL violations
    High,
}

/// UPL detection result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UplDetectionResult {
    /// Whether UPL was detected
    pub detected: bool,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f64,
    /// Explanation of the detection
    pub explanation: String,
    /// Specific issues or violations found
    pub violations: Vec<String>,
}

impl<P: LLMProvider> UplDetector<P> {
    /// Creates a new UPL detector.
    pub fn new(provider: P) -> Self {
        Self {
            provider,
            strictness: UplStrictness::Medium,
        }
    }

    /// Sets the strictness level.
    pub fn with_strictness(mut self, strictness: UplStrictness) -> Self {
        self.strictness = strictness;
        self
    }

    /// Detects potential unauthorized practice of law in text.
    pub async fn detect(&self, text: &str) -> Result<UplDetectionResult> {
        let strictness_desc = match self.strictness {
            UplStrictness::Low => "obvious and clear-cut",
            UplStrictness::Medium => "likely and probable",
            UplStrictness::High => "any potential or possible",
        };

        let prompt = format!(
            r#"Analyze the following text for potential unauthorized practice of law (UPL).
Flag {} violations.

Unauthorized practice of law typically includes:
- Providing specific legal advice to a particular person or situation
- Drafting legal documents for a specific client
- Representing someone in legal proceedings
- Making legal determinations or judgments
- Promising specific legal outcomes

Text to analyze:
{text}

Provide your analysis in the following JSON format:
{{
    "detected": true/false,
    "confidence": 0.85,
    "explanation": "Explanation of why this may or may not be UPL",
    "violations": ["Specific violation 1", "Specific violation 2", ...]
}}

Be objective and consider that general legal information is not UPL."#,
            strictness_desc,
            text = text
        );

        self.provider
            .generate_structured(&prompt)
            .await
            .context("Failed to detect UPL")
    }
}

/// Confidentiality protector for filtering sensitive information.
pub struct ConfidentialityProtector {
    redact_patterns: Vec<regex::Regex>,
}

impl ConfidentialityProtector {
    /// Creates a new confidentiality protector.
    pub fn new() -> Self {
        // Default patterns for common sensitive information
        let patterns = vec![
            // Social Security Numbers
            regex::Regex::new(r"\b\d{3}-\d{2}-\d{4}\b").unwrap(),
            // Credit card numbers (simple pattern)
            regex::Regex::new(r"\b\d{4}[\s-]?\d{4}[\s-]?\d{4}[\s-]?\d{4}\b").unwrap(),
            // Email addresses
            regex::Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b").unwrap(),
            // Phone numbers
            regex::Regex::new(r"\b\d{3}[-.]?\d{3}[-.]?\d{4}\b").unwrap(),
        ];

        Self {
            redact_patterns: patterns,
        }
    }

    /// Adds a custom redaction pattern.
    pub fn add_pattern(&mut self, pattern: &str) -> Result<()> {
        let regex = regex::Regex::new(pattern)
            .with_context(|| format!("Invalid regex pattern: {}", pattern))?;
        self.redact_patterns.push(regex);
        Ok(())
    }

    /// Redacts sensitive information from text.
    pub fn redact(&self, text: &str) -> String {
        let mut redacted = text.to_string();

        for pattern in &self.redact_patterns {
            redacted = pattern.replace_all(&redacted, "[REDACTED]").to_string();
        }

        redacted
    }

    /// Checks if text contains sensitive information.
    pub fn contains_sensitive_info(&self, text: &str) -> bool {
        self.redact_patterns.iter().any(|p| p.is_match(text))
    }
}

impl Default for ConfidentialityProtector {
    fn default() -> Self {
        Self::new()
    }
}

/// Jurisdiction-aware safety filter.
pub struct JurisdictionFilter {
    jurisdiction: Jurisdiction,
    restricted_topics: Vec<String>,
}

impl JurisdictionFilter {
    /// Creates a new jurisdiction filter.
    pub fn new(jurisdiction: Jurisdiction) -> Self {
        Self {
            jurisdiction,
            restricted_topics: Vec::new(),
        }
    }

    /// Adds a restricted topic for this jurisdiction.
    pub fn add_restricted_topic(&mut self, topic: impl Into<String>) {
        self.restricted_topics.push(topic.into());
    }

    /// Checks if a query is allowed in this jurisdiction.
    pub fn is_allowed(&self, query: &str) -> Result<()> {
        // Check for restricted topics
        let query_lower = query.to_lowercase();
        for topic in &self.restricted_topics {
            if query_lower.contains(&topic.to_lowercase()) {
                return Err(anyhow!(
                    "This topic is restricted in {}",
                    self.jurisdiction.description()
                ));
            }
        }

        Ok(())
    }
}

/// Provider wrapper that adds compliance and safety features.
pub struct ComplianceProvider<P> {
    provider: P,
    disclaimer: Option<LegalDisclaimer>,
    audit_logger: Option<AuditLogger>,
    confidentiality_protector: Option<ConfidentialityProtector>,
    jurisdiction: Option<Jurisdiction>,
    user_id: String,
}

impl<P: LLMProvider> ComplianceProvider<P> {
    /// Creates a new compliance provider.
    pub fn new(provider: P) -> Self {
        Self {
            provider,
            disclaimer: Some(LegalDisclaimer::default()),
            audit_logger: Some(AuditLogger::new()),
            confidentiality_protector: Some(ConfidentialityProtector::new()),
            jurisdiction: None,
            user_id: "anonymous".to_string(),
        }
    }

    /// Sets the legal disclaimer.
    pub fn with_disclaimer(mut self, disclaimer: LegalDisclaimer) -> Self {
        self.disclaimer = Some(disclaimer);
        self
    }

    /// Disables the legal disclaimer.
    pub fn without_disclaimer(mut self) -> Self {
        self.disclaimer = None;
        self
    }

    /// Sets the audit logger.
    pub fn with_audit_logger(mut self, logger: AuditLogger) -> Self {
        self.audit_logger = Some(logger);
        self
    }

    /// Sets the confidentiality protector.
    pub fn with_confidentiality_protector(mut self, protector: ConfidentialityProtector) -> Self {
        self.confidentiality_protector = Some(protector);
        self
    }

    /// Sets the jurisdiction.
    pub fn with_jurisdiction(mut self, jurisdiction: Jurisdiction) -> Self {
        self.jurisdiction = Some(jurisdiction);
        self
    }

    /// Sets the user ID for audit logging.
    pub fn with_user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = user_id.into();
        self
    }

    /// Gets the audit logger.
    pub fn audit_logger(&self) -> Option<&AuditLogger> {
        self.audit_logger.as_ref()
    }
}

#[async_trait]
impl<P: LLMProvider> LLMProvider for ComplianceProvider<P> {
    async fn generate_text(&self, prompt: &str) -> Result<String> {
        // Check for sensitive information in input
        if let Some(ref protector) = self.confidentiality_protector {
            if protector.contains_sensitive_info(prompt) {
                return Err(anyhow!(
                    "Input contains sensitive information that must be redacted"
                ));
            }
        }

        // Generate response
        let mut response = self.provider.generate_text(prompt).await?;

        // Redact sensitive information from output
        if let Some(ref protector) = self.confidentiality_protector {
            response = protector.redact(&response);
        }

        // Apply disclaimer
        if let Some(ref disclaimer) = self.disclaimer {
            if disclaimer.required {
                response = disclaimer.apply(&response);
            }
        }

        // Audit log
        if let Some(ref logger) = self.audit_logger {
            logger.log(
                &self.user_id,
                "generate_text",
                prompt,
                &response,
                self.jurisdiction.as_ref().map(|j| j.description()),
                true,
            );
        }

        Ok(response)
    }

    async fn generate_structured<T: serde::de::DeserializeOwned + Send>(
        &self,
        prompt: &str,
    ) -> Result<T> {
        // Similar checks as generate_text
        if let Some(ref protector) = self.confidentiality_protector {
            if protector.contains_sensitive_info(prompt) {
                return Err(anyhow!(
                    "Input contains sensitive information that must be redacted"
                ));
            }
        }

        let response = self.provider.generate_structured(prompt).await?;

        // Audit log
        if let Some(ref logger) = self.audit_logger {
            logger.log(
                &self.user_id,
                "generate_structured",
                prompt,
                "structured_output",
                self.jurisdiction.as_ref().map(|j| j.description()),
                true,
            );
        }

        Ok(response)
    }

    async fn generate_text_stream(&self, prompt: &str) -> Result<crate::TextStream> {
        // Check for sensitive information
        if let Some(ref protector) = self.confidentiality_protector {
            if protector.contains_sensitive_info(prompt) {
                return Err(anyhow!(
                    "Input contains sensitive information that must be redacted"
                ));
            }
        }

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::providers::MockProvider;

    #[test]
    fn test_legal_disclaimer() {
        let disclaimer = LegalDisclaimer::default();
        let text = "This is a response";
        let result = disclaimer.apply(text);

        assert!(result.contains("DISCLAIMER"));
        assert!(result.contains(text));
    }

    #[test]
    fn test_audit_logger() {
        let logger = AuditLogger::new();

        logger.log("user1", "test_op", "input", "output", None, true);
        assert_eq!(logger.entry_count(), 1);

        let entries = logger.get_entries();
        assert_eq!(entries[0].user, "user1");
        assert_eq!(entries[0].operation, "test_op");

        logger.clear();
        assert_eq!(logger.entry_count(), 0);
    }

    #[test]
    fn test_confidentiality_protector() {
        let protector = ConfidentialityProtector::new();

        let text = "My email is test@example.com";
        assert!(protector.contains_sensitive_info(text));

        let redacted = protector.redact(text);
        assert!(redacted.contains("[REDACTED]"));
        assert!(!redacted.contains("test@example.com"));
    }

    #[test]
    fn test_jurisdiction_filter() {
        let mut filter = JurisdictionFilter::new(Jurisdiction::UsFederal);
        filter.add_restricted_topic("gambling");

        assert!(filter.is_allowed("general legal question").is_ok());
        assert!(filter.is_allowed("online gambling laws").is_err());
    }

    #[tokio::test]
    async fn test_compliance_provider() {
        let mock_provider = MockProvider::new().with_response("test", "Mock response");

        let compliance = ComplianceProvider::new(mock_provider)
            .with_user_id("test_user")
            .with_jurisdiction(Jurisdiction::UsFederal);

        let response = compliance.generate_text("test prompt").await.unwrap();

        assert!(response.contains("DISCLAIMER"));

        if let Some(logger) = compliance.audit_logger() {
            assert_eq!(logger.entry_count(), 1);
        }
    }

    #[test]
    fn test_disclaimer_position() {
        let disclaimer = LegalDisclaimer::new("NOTICE").with_position(DisclaimerPosition::Prepend);

        let result = disclaimer.apply("Content");
        assert!(result.starts_with("NOTICE"));

        let disclaimer = LegalDisclaimer::new("NOTICE").with_position(DisclaimerPosition::Append);

        let result = disclaimer.apply("Content");
        assert!(result.ends_with("NOTICE"));
    }
}
