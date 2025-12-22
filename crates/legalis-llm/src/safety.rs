//! Safety and moderation tools for LLM content.
//!
//! This module provides content filtering, PII detection, toxicity scoring,
//! and custom safety guardrails for LLM inputs and outputs.

use crate::LLMProvider;
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Content moderation categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModerationCategory {
    /// Hate speech
    Hate,
    /// Harassment
    Harassment,
    /// Self-harm
    SelfHarm,
    /// Sexual content
    Sexual,
    /// Violence
    Violence,
    /// Illegal activity
    Illegal,
}

/// Moderation result for content.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModerationResult {
    /// Whether the content is flagged
    pub flagged: bool,
    /// Category scores (0.0 - 1.0)
    pub scores: std::collections::HashMap<ModerationCategory, f32>,
    /// Detected categories
    pub categories: Vec<ModerationCategory>,
}

impl ModerationResult {
    /// Creates a new moderation result.
    pub fn new(flagged: bool) -> Self {
        Self {
            flagged,
            scores: std::collections::HashMap::new(),
            categories: Vec::new(),
        }
    }

    /// Adds a category score.
    pub fn with_score(mut self, category: ModerationCategory, score: f32) -> Self {
        self.scores.insert(category, score);
        if score > 0.5 {
            self.categories.push(category);
            self.flagged = true;
        }
        self
    }

    /// Returns the maximum score across all categories.
    pub fn max_score(&self) -> f32 {
        self.scores.values().copied().fold(0.0f32, f32::max)
    }
}

/// Content moderation provider.
#[async_trait]
pub trait ModerationProvider: Send + Sync {
    /// Moderates text content.
    async fn moderate(&self, text: &str) -> Result<ModerationResult>;
}

/// Basic pattern-based moderator.
pub struct PatternModerator {
    patterns: Vec<(ModerationCategory, Regex)>,
}

impl PatternModerator {
    /// Creates a new pattern-based moderator with default patterns.
    pub fn new() -> Self {
        let mut patterns = Vec::new();

        // Hate speech patterns (simplified for demonstration)
        if let Ok(re) = Regex::new(r"(?i)\b(hate|racist|bigot)\b") {
            patterns.push((ModerationCategory::Hate, re));
        }

        // Harassment patterns
        if let Ok(re) = Regex::new(r"(?i)\b(harass|bully|threaten)\b") {
            patterns.push((ModerationCategory::Harassment, re));
        }

        // Violence patterns
        if let Ok(re) = Regex::new(r"(?i)\b(kill|murder|violence|attack)\b") {
            patterns.push((ModerationCategory::Violence, re));
        }

        // Self-harm patterns
        if let Ok(re) = Regex::new(r"(?i)\b(suicide|self.?harm|kill myself)\b") {
            patterns.push((ModerationCategory::SelfHarm, re));
        }

        Self { patterns }
    }

    /// Adds a custom pattern.
    pub fn add_pattern(&mut self, category: ModerationCategory, pattern: &str) -> Result<()> {
        let re = Regex::new(pattern)?;
        self.patterns.push((category, re));
        Ok(())
    }
}

impl Default for PatternModerator {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ModerationProvider for PatternModerator {
    async fn moderate(&self, text: &str) -> Result<ModerationResult> {
        let mut result = ModerationResult::new(false);

        for (category, pattern) in &self.patterns {
            if pattern.is_match(text) {
                result = result.with_score(*category, 0.8);
            }
        }

        Ok(result)
    }
}

/// PII (Personally Identifiable Information) type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PIIType {
    /// Email address
    Email,
    /// Phone number
    Phone,
    /// Social Security Number
    SSN,
    /// Credit card number
    CreditCard,
    /// IP address
    IPAddress,
}

/// Detected PII in text.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedPII {
    /// Type of PII
    pub pii_type: PIIType,
    /// The detected value
    pub value: String,
    /// Start position in text
    pub start: usize,
    /// End position in text
    pub end: usize,
}

/// PII detector using pattern matching.
pub struct PIIDetector {
    patterns: Vec<(PIIType, Regex)>,
}

impl PIIDetector {
    /// Creates a new PII detector with default patterns.
    pub fn new() -> Self {
        let mut patterns = Vec::new();

        // Email pattern
        if let Ok(re) = Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b") {
            patterns.push((PIIType::Email, re));
        }

        // Phone pattern (simple US format)
        if let Ok(re) = Regex::new(r"\b\d{3}[-.]?\d{3}[-.]?\d{4}\b") {
            patterns.push((PIIType::Phone, re));
        }

        // SSN pattern
        if let Ok(re) = Regex::new(r"\b\d{3}-\d{2}-\d{4}\b") {
            patterns.push((PIIType::SSN, re));
        }

        // Credit card pattern (basic)
        if let Ok(re) = Regex::new(r"\b\d{4}[- ]?\d{4}[- ]?\d{4}[- ]?\d{4}\b") {
            patterns.push((PIIType::CreditCard, re));
        }

        // IP address pattern
        if let Ok(re) = Regex::new(r"\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\b") {
            patterns.push((PIIType::IPAddress, re));
        }

        Self { patterns }
    }

    /// Detects PII in text.
    pub fn detect(&self, text: &str) -> Vec<DetectedPII> {
        let mut detected = Vec::new();

        for (pii_type, pattern) in &self.patterns {
            for mat in pattern.find_iter(text) {
                detected.push(DetectedPII {
                    pii_type: *pii_type,
                    value: mat.as_str().to_string(),
                    start: mat.start(),
                    end: mat.end(),
                });
            }
        }

        detected.sort_by_key(|d| d.start);
        detected
    }

    /// Redacts PII from text.
    pub fn redact(&self, text: &str) -> String {
        let detected = self.detect(text);
        let mut result = text.to_string();
        let mut offset = 0i32;

        for pii in detected {
            let replacement = match pii.pii_type {
                PIIType::Email => "[EMAIL]",
                PIIType::Phone => "[PHONE]",
                PIIType::SSN => "[SSN]",
                PIIType::CreditCard => "[CARD]",
                PIIType::IPAddress => "[IP]",
            };

            let start = (pii.start as i32 + offset) as usize;
            let end = (pii.end as i32 + offset) as usize;
            result.replace_range(start..end, replacement);

            offset += replacement.len() as i32 - (pii.end - pii.start) as i32;
        }

        result
    }
}

impl Default for PIIDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Safety rule for custom content validation.
#[derive(Debug, Clone)]
pub struct SafetyRule {
    /// Rule name
    pub name: String,
    /// Rule description
    pub description: String,
    /// Pattern to match
    pub pattern: Regex,
    /// Whether this is a blocking rule
    pub blocking: bool,
}

impl SafetyRule {
    /// Creates a new safety rule.
    pub fn new(name: impl Into<String>, pattern: &str, blocking: bool) -> Result<Self> {
        Ok(Self {
            name: name.into(),
            description: String::new(),
            pattern: Regex::new(pattern)?,
            blocking,
        })
    }

    /// Sets the description.
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    /// Checks if text violates this rule.
    pub fn check(&self, text: &str) -> bool {
        self.pattern.is_match(text)
    }
}

/// Safety rules engine.
pub struct SafetyEngine {
    rules: Vec<SafetyRule>,
}

impl SafetyEngine {
    /// Creates a new safety engine.
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    /// Adds a safety rule.
    pub fn add_rule(&mut self, rule: SafetyRule) {
        self.rules.push(rule);
    }

    /// Validates text against all rules.
    pub fn validate(&self, text: &str) -> SafetyValidation {
        let mut validation = SafetyValidation {
            passed: true,
            violations: Vec::new(),
        };

        for rule in &self.rules {
            if rule.check(text) {
                validation.violations.push(rule.name.clone());
                if rule.blocking {
                    validation.passed = false;
                }
            }
        }

        validation
    }
}

impl Default for SafetyEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of safety validation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyValidation {
    /// Whether validation passed
    pub passed: bool,
    /// List of violated rules
    pub violations: Vec<String>,
}

impl SafetyValidation {
    /// Returns whether validation passed.
    pub fn is_safe(&self) -> bool {
        self.passed
    }

    /// Returns whether there are any violations.
    pub fn has_violations(&self) -> bool {
        !self.violations.is_empty()
    }
}

/// Guardrails configuration for LLM safety.
#[derive(Debug, Clone)]
pub struct GuardrailsConfig {
    /// Enable input moderation
    pub moderate_input: bool,
    /// Enable output moderation
    pub moderate_output: bool,
    /// Enable PII detection on input
    pub detect_pii_input: bool,
    /// Enable PII detection on output
    pub detect_pii_output: bool,
    /// Redact PII from input
    pub redact_pii_input: bool,
    /// Redact PII from output
    pub redact_pii_output: bool,
    /// Apply safety rules to input
    pub apply_rules_input: bool,
    /// Apply safety rules to output
    pub apply_rules_output: bool,
}

impl Default for GuardrailsConfig {
    fn default() -> Self {
        Self {
            moderate_input: true,
            moderate_output: true,
            detect_pii_input: true,
            detect_pii_output: true,
            redact_pii_input: true,
            redact_pii_output: true,
            apply_rules_input: true,
            apply_rules_output: true,
        }
    }
}

/// LLM provider with safety guardrails.
pub struct GuardedProvider<P, M> {
    provider: P,
    moderator: Arc<M>,
    pii_detector: Arc<PIIDetector>,
    safety_engine: Arc<SafetyEngine>,
    config: GuardrailsConfig,
}

impl<P, M> GuardedProvider<P, M>
where
    P: LLMProvider,
    M: ModerationProvider,
{
    /// Creates a new guarded provider.
    pub fn new(
        provider: P,
        moderator: M,
        pii_detector: PIIDetector,
        safety_engine: SafetyEngine,
        config: GuardrailsConfig,
    ) -> Self {
        Self {
            provider,
            moderator: Arc::new(moderator),
            pii_detector: Arc::new(pii_detector),
            safety_engine: Arc::new(safety_engine),
            config,
        }
    }

    /// Validates input before sending to LLM.
    async fn validate_input(&self, text: &str) -> Result<String> {
        let mut processed = text.to_string();

        // Apply moderation
        if self.config.moderate_input {
            let moderation = self.moderator.moderate(&processed).await?;
            if moderation.flagged {
                return Err(anyhow!(
                    "Input content flagged for moderation: {:?}",
                    moderation.categories
                ));
            }
        }

        // Apply safety rules
        if self.config.apply_rules_input {
            let validation = self.safety_engine.validate(&processed);
            if !validation.is_safe() {
                return Err(anyhow!(
                    "Input violates safety rules: {:?}",
                    validation.violations
                ));
            }
        }

        // Redact PII
        if self.config.redact_pii_input {
            processed = self.pii_detector.redact(&processed);
        }

        Ok(processed)
    }

    /// Validates output from LLM.
    async fn validate_output(&self, text: &str) -> Result<String> {
        let mut processed = text.to_string();

        // Apply moderation
        if self.config.moderate_output {
            let moderation = self.moderator.moderate(&processed).await?;
            if moderation.flagged {
                return Err(anyhow!(
                    "Output content flagged for moderation: {:?}",
                    moderation.categories
                ));
            }
        }

        // Apply safety rules
        if self.config.apply_rules_output {
            let validation = self.safety_engine.validate(&processed);
            if !validation.is_safe() {
                return Err(anyhow!(
                    "Output violates safety rules: {:?}",
                    validation.violations
                ));
            }
        }

        // Redact PII
        if self.config.redact_pii_output {
            processed = self.pii_detector.redact(&processed);
        }

        Ok(processed)
    }
}

#[async_trait]
impl<P, M> LLMProvider for GuardedProvider<P, M>
where
    P: LLMProvider,
    M: ModerationProvider,
{
    async fn generate_text(&self, prompt: &str) -> Result<String> {
        let validated_input = self.validate_input(prompt).await?;
        let response = self.provider.generate_text(&validated_input).await?;
        self.validate_output(&response).await
    }

    async fn generate_structured<T: serde::de::DeserializeOwned + Send>(
        &self,
        prompt: &str,
    ) -> Result<T> {
        let validated_input = self.validate_input(prompt).await?;
        self.provider.generate_structured(&validated_input).await
    }

    async fn generate_text_stream(&self, prompt: &str) -> Result<crate::TextStream> {
        let validated_input = self.validate_input(prompt).await?;
        self.provider.generate_text_stream(&validated_input).await
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

    #[tokio::test]
    async fn test_pattern_moderator() {
        let moderator = PatternModerator::new();

        let result = moderator.moderate("I hate this").await.unwrap();
        assert!(result.flagged);
        assert!(result.categories.contains(&ModerationCategory::Hate));

        let result = moderator.moderate("Hello world").await.unwrap();
        assert!(!result.flagged);
    }

    #[test]
    fn test_pii_detector_email() {
        let detector = PIIDetector::new();
        let detected = detector.detect("Contact me at test@example.com");

        assert_eq!(detected.len(), 1);
        assert_eq!(detected[0].pii_type, PIIType::Email);
        assert_eq!(detected[0].value, "test@example.com");
    }

    #[test]
    fn test_pii_detector_phone() {
        let detector = PIIDetector::new();
        let detected = detector.detect("Call me at 555-123-4567");

        assert_eq!(detected.len(), 1);
        assert_eq!(detected[0].pii_type, PIIType::Phone);
    }

    #[test]
    fn test_pii_redaction() {
        let detector = PIIDetector::new();
        let text = "Email: test@example.com, Phone: 555-123-4567";
        let redacted = detector.redact(text);

        assert!(redacted.contains("[EMAIL]"));
        assert!(redacted.contains("[PHONE]"));
        assert!(!redacted.contains("test@example.com"));
        assert!(!redacted.contains("555-123-4567"));
    }

    #[test]
    fn test_safety_rule() {
        let rule = SafetyRule::new("no_passwords", r"(?i)\bpassword\b", true)
            .unwrap()
            .with_description("Blocks requests for passwords");

        assert!(rule.check("What is your password?"));
        assert!(!rule.check("Hello world"));
    }

    #[test]
    fn test_safety_engine() {
        let mut engine = SafetyEngine::new();
        engine.add_rule(SafetyRule::new("no_passwords", r"(?i)\bpassword\b", true).unwrap());

        let validation = engine.validate("What is your password?");
        assert!(!validation.is_safe());
        assert!(validation.has_violations());

        let validation = engine.validate("Hello world");
        assert!(validation.is_safe());
        assert!(!validation.has_violations());
    }

    #[test]
    fn test_moderation_result() {
        let result = ModerationResult::new(false)
            .with_score(ModerationCategory::Hate, 0.2)
            .with_score(ModerationCategory::Violence, 0.8);

        assert!(result.flagged);
        assert_eq!(result.max_score(), 0.8);
    }
}
