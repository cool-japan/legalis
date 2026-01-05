//! Webhook notifications for conversion events.
//!
//! This module provides webhook notification capabilities for conversion operations.

use crate::{ConversionReport, LegalFormat};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Webhook event type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WebhookEvent {
    /// Conversion started
    ConversionStarted,
    /// Conversion completed successfully
    ConversionCompleted,
    /// Conversion failed
    ConversionFailed,
    /// Conversion warning
    ConversionWarning,
}

/// Webhook payload for conversion events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookPayload {
    /// Event type
    pub event: WebhookEvent,
    /// Conversion ID
    pub conversion_id: String,
    /// Timestamp (Unix timestamp in milliseconds)
    pub timestamp: u64,
    /// Source format
    pub source_format: Option<LegalFormat>,
    /// Target format
    pub target_format: Option<LegalFormat>,
    /// Conversion report (for completed conversions)
    pub report: Option<ConversionReport>,
    /// Error message (for failed conversions)
    pub error: Option<String>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl WebhookPayload {
    /// Creates a new webhook payload for a conversion start event.
    pub fn conversion_started(id: String, source: LegalFormat, target: LegalFormat) -> Self {
        Self {
            event: WebhookEvent::ConversionStarted,
            conversion_id: id,
            timestamp: Self::current_timestamp(),
            source_format: Some(source),
            target_format: Some(target),
            report: None,
            error: None,
            metadata: HashMap::new(),
        }
    }

    /// Creates a new webhook payload for a conversion completed event.
    pub fn conversion_completed(
        id: String,
        source: LegalFormat,
        target: LegalFormat,
        report: ConversionReport,
    ) -> Self {
        Self {
            event: WebhookEvent::ConversionCompleted,
            conversion_id: id,
            timestamp: Self::current_timestamp(),
            source_format: Some(source),
            target_format: Some(target),
            report: Some(report),
            error: None,
            metadata: HashMap::new(),
        }
    }

    /// Creates a new webhook payload for a conversion failed event.
    pub fn conversion_failed(
        id: String,
        source: Option<LegalFormat>,
        target: Option<LegalFormat>,
        error: String,
    ) -> Self {
        Self {
            event: WebhookEvent::ConversionFailed,
            conversion_id: id,
            timestamp: Self::current_timestamp(),
            source_format: source,
            target_format: target,
            report: None,
            error: Some(error),
            metadata: HashMap::new(),
        }
    }

    /// Creates a new webhook payload for a conversion warning event.
    pub fn conversion_warning(
        id: String,
        source: LegalFormat,
        target: LegalFormat,
        warning: String,
    ) -> Self {
        let mut metadata = HashMap::new();
        metadata.insert("warning".to_string(), warning);

        Self {
            event: WebhookEvent::ConversionWarning,
            conversion_id: id,
            timestamp: Self::current_timestamp(),
            source_format: Some(source),
            target_format: Some(target),
            report: None,
            error: None,
            metadata,
        }
    }

    /// Adds metadata to the payload.
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    /// Returns the current timestamp in milliseconds.
    fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64
    }

    /// Converts the payload to JSON.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Converts the payload to pretty JSON.
    pub fn to_json_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

/// Webhook delivery method.
pub trait WebhookDelivery: Send + Sync {
    /// Sends a webhook payload to the configured endpoint.
    fn send(&self, payload: &WebhookPayload) -> Result<(), String>;

    /// Returns the delivery method name.
    fn name(&self) -> &str;
}

/// HTTP webhook delivery implementation.
#[derive(Debug, Clone)]
pub struct HttpWebhook {
    /// Webhook URL
    pub url: String,
    /// HTTP headers to include
    pub headers: HashMap<String, String>,
    /// Timeout in milliseconds
    pub timeout_ms: u64,
}

impl HttpWebhook {
    /// Creates a new HTTP webhook.
    pub fn new(url: String) -> Self {
        Self {
            url,
            headers: HashMap::new(),
            timeout_ms: 5000,
        }
    }

    /// Adds an HTTP header.
    pub fn with_header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }

    /// Sets the timeout in milliseconds.
    pub fn with_timeout(mut self, timeout_ms: u64) -> Self {
        self.timeout_ms = timeout_ms;
        self
    }
}

impl WebhookDelivery for HttpWebhook {
    fn send(&self, payload: &WebhookPayload) -> Result<(), String> {
        // In a real implementation, this would use an HTTP client to send the payload
        // For now, we'll just simulate it
        let _json = payload
            .to_json()
            .map_err(|e| format!("Failed to serialize payload: {}", e))?;

        // Simulate HTTP POST (would use reqwest or similar in production)
        // reqwest::blocking::Client::new()
        //     .post(&self.url)
        //     .headers(...)
        //     .json(&payload)
        //     .send()

        Ok(())
    }

    fn name(&self) -> &str {
        "http"
    }
}

/// File-based webhook delivery (for testing/logging).
#[derive(Debug, Clone)]
pub struct FileWebhook {
    /// File path to write webhooks
    pub path: std::path::PathBuf,
}

impl FileWebhook {
    /// Creates a new file webhook.
    pub fn new(path: std::path::PathBuf) -> Self {
        Self { path }
    }
}

impl WebhookDelivery for FileWebhook {
    fn send(&self, payload: &WebhookPayload) -> Result<(), String> {
        let json = payload
            .to_json_pretty()
            .map_err(|e| format!("Failed to serialize payload: {}", e))?;

        std::fs::write(&self.path, json)
            .map_err(|e| format!("Failed to write webhook to file: {}", e))?;

        Ok(())
    }

    fn name(&self) -> &str {
        "file"
    }
}

/// Webhook manager for handling multiple webhook endpoints.
pub struct WebhookManager {
    /// Registered webhooks
    webhooks: Vec<Box<dyn WebhookDelivery>>,
    /// Event filters (which events to send)
    event_filters: Vec<WebhookEvent>,
}

impl Default for WebhookManager {
    fn default() -> Self {
        Self::new()
    }
}

impl WebhookManager {
    /// Creates a new webhook manager.
    pub fn new() -> Self {
        Self {
            webhooks: Vec::new(),
            event_filters: vec![
                WebhookEvent::ConversionStarted,
                WebhookEvent::ConversionCompleted,
                WebhookEvent::ConversionFailed,
            ],
        }
    }

    /// Registers a webhook delivery method.
    pub fn register(&mut self, webhook: Box<dyn WebhookDelivery>) {
        self.webhooks.push(webhook);
    }

    /// Sets the event filters.
    pub fn set_event_filters(&mut self, events: Vec<WebhookEvent>) {
        self.event_filters = events;
    }

    /// Sends a webhook payload to all registered webhooks.
    pub fn send(&self, payload: &WebhookPayload) -> Vec<Result<(), String>> {
        // Check if this event should be sent
        if !self.event_filters.is_empty() && !self.event_filters.contains(&payload.event) {
            return Vec::new();
        }

        let mut results = Vec::new();
        for webhook in &self.webhooks {
            results.push(webhook.send(payload));
        }
        results
    }

    /// Sends a conversion started notification.
    pub fn notify_conversion_started(
        &self,
        id: String,
        source: LegalFormat,
        target: LegalFormat,
    ) -> Vec<Result<(), String>> {
        let payload = WebhookPayload::conversion_started(id, source, target);
        self.send(&payload)
    }

    /// Sends a conversion completed notification.
    pub fn notify_conversion_completed(
        &self,
        id: String,
        source: LegalFormat,
        target: LegalFormat,
        report: ConversionReport,
    ) -> Vec<Result<(), String>> {
        let payload = WebhookPayload::conversion_completed(id, source, target, report);
        self.send(&payload)
    }

    /// Sends a conversion failed notification.
    pub fn notify_conversion_failed(
        &self,
        id: String,
        source: Option<LegalFormat>,
        target: Option<LegalFormat>,
        error: String,
    ) -> Vec<Result<(), String>> {
        let payload = WebhookPayload::conversion_failed(id, source, target, error);
        self.send(&payload)
    }

    /// Sends a conversion warning notification.
    pub fn notify_conversion_warning(
        &self,
        id: String,
        source: LegalFormat,
        target: LegalFormat,
        warning: String,
    ) -> Vec<Result<(), String>> {
        let payload = WebhookPayload::conversion_warning(id, source, target, warning);
        self.send(&payload)
    }

    /// Returns the number of registered webhooks.
    pub fn webhook_count(&self) -> usize {
        self.webhooks.len()
    }

    /// Returns true if any webhooks are registered.
    pub fn has_webhooks(&self) -> bool {
        !self.webhooks.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_webhook_payload_conversion_started() {
        let payload = WebhookPayload::conversion_started(
            "test-1".to_string(),
            LegalFormat::Catala,
            LegalFormat::L4,
        );

        assert_eq!(payload.event, WebhookEvent::ConversionStarted);
        assert_eq!(payload.conversion_id, "test-1");
        assert_eq!(payload.source_format, Some(LegalFormat::Catala));
        assert_eq!(payload.target_format, Some(LegalFormat::L4));
        assert!(payload.report.is_none());
        assert!(payload.error.is_none());
    }

    #[test]
    fn test_webhook_payload_conversion_completed() {
        let report = ConversionReport::new(LegalFormat::Catala, LegalFormat::L4);
        let payload = WebhookPayload::conversion_completed(
            "test-1".to_string(),
            LegalFormat::Catala,
            LegalFormat::L4,
            report,
        );

        assert_eq!(payload.event, WebhookEvent::ConversionCompleted);
        assert_eq!(payload.conversion_id, "test-1");
        assert!(payload.report.is_some());
        assert!(payload.error.is_none());
    }

    #[test]
    fn test_webhook_payload_conversion_failed() {
        let payload = WebhookPayload::conversion_failed(
            "test-1".to_string(),
            Some(LegalFormat::Catala),
            Some(LegalFormat::L4),
            "Parse error".to_string(),
        );

        assert_eq!(payload.event, WebhookEvent::ConversionFailed);
        assert_eq!(payload.conversion_id, "test-1");
        assert_eq!(payload.error, Some("Parse error".to_string()));
        assert!(payload.report.is_none());
    }

    #[test]
    fn test_webhook_payload_with_metadata() {
        let payload = WebhookPayload::conversion_started(
            "test-1".to_string(),
            LegalFormat::Catala,
            LegalFormat::L4,
        )
        .with_metadata("user".to_string(), "alice".to_string())
        .with_metadata("priority".to_string(), "high".to_string());

        assert_eq!(payload.metadata.get("user"), Some(&"alice".to_string()));
        assert_eq!(payload.metadata.get("priority"), Some(&"high".to_string()));
    }

    #[test]
    fn test_webhook_payload_to_json() {
        let payload = WebhookPayload::conversion_started(
            "test-1".to_string(),
            LegalFormat::Catala,
            LegalFormat::L4,
        );

        let json = payload.to_json().unwrap();
        assert!(json.contains("conversion_started"));
        assert!(json.contains("test-1"));
        assert!(json.contains("Catala"));
    }

    #[test]
    fn test_http_webhook() {
        let webhook = HttpWebhook::new("https://example.com/webhook".to_string())
            .with_header("Authorization".to_string(), "Bearer token".to_string())
            .with_timeout(10000);

        assert_eq!(webhook.url, "https://example.com/webhook");
        assert_eq!(webhook.timeout_ms, 10000);
        assert_eq!(
            webhook.headers.get("Authorization"),
            Some(&"Bearer token".to_string())
        );
    }

    #[test]
    fn test_file_webhook() {
        let temp_file = NamedTempFile::new().unwrap();
        let webhook = FileWebhook::new(temp_file.path().to_owned());

        let payload = WebhookPayload::conversion_started(
            "test-1".to_string(),
            LegalFormat::Catala,
            LegalFormat::L4,
        );

        let result = webhook.send(&payload);
        assert!(result.is_ok());

        let content = std::fs::read_to_string(temp_file.path()).unwrap();
        assert!(content.contains("conversion_started"));
        assert!(content.contains("test-1"));
    }

    #[test]
    fn test_webhook_manager() {
        let mut manager = WebhookManager::new();

        assert_eq!(manager.webhook_count(), 0);
        assert!(!manager.has_webhooks());

        let temp_file = NamedTempFile::new().unwrap();
        let webhook = Box::new(FileWebhook::new(temp_file.path().to_owned()));
        manager.register(webhook);

        assert_eq!(manager.webhook_count(), 1);
        assert!(manager.has_webhooks());

        let results = manager.notify_conversion_started(
            "test-1".to_string(),
            LegalFormat::Catala,
            LegalFormat::L4,
        );

        assert_eq!(results.len(), 1);
        assert!(results[0].is_ok());
    }

    #[test]
    fn test_webhook_manager_event_filters() {
        let mut manager = WebhookManager::new();

        let temp_file = NamedTempFile::new().unwrap();
        let webhook = Box::new(FileWebhook::new(temp_file.path().to_owned()));
        manager.register(webhook);

        // Only send completion events
        manager.set_event_filters(vec![WebhookEvent::ConversionCompleted]);

        // Start event should not be sent
        let results = manager.notify_conversion_started(
            "test-1".to_string(),
            LegalFormat::Catala,
            LegalFormat::L4,
        );
        assert_eq!(results.len(), 0);

        // Completion event should be sent
        let report = ConversionReport::new(LegalFormat::Catala, LegalFormat::L4);
        let results = manager.notify_conversion_completed(
            "test-1".to_string(),
            LegalFormat::Catala,
            LegalFormat::L4,
            report,
        );
        assert_eq!(results.len(), 1);
        assert!(results[0].is_ok());
    }

    #[test]
    fn test_webhook_manager_multiple_webhooks() {
        let mut manager = WebhookManager::new();

        let temp_file1 = NamedTempFile::new().unwrap();
        let temp_file2 = NamedTempFile::new().unwrap();

        manager.register(Box::new(FileWebhook::new(temp_file1.path().to_owned())));
        manager.register(Box::new(FileWebhook::new(temp_file2.path().to_owned())));

        assert_eq!(manager.webhook_count(), 2);

        let results = manager.notify_conversion_started(
            "test-1".to_string(),
            LegalFormat::Catala,
            LegalFormat::L4,
        );

        assert_eq!(results.len(), 2);
        assert!(results[0].is_ok());
        assert!(results[1].is_ok());

        // Both files should have been written
        let content1 = std::fs::read_to_string(temp_file1.path()).unwrap();
        let content2 = std::fs::read_to_string(temp_file2.path()).unwrap();

        assert!(content1.contains("conversion_started"));
        assert!(content2.contains("conversion_started"));
    }
}
