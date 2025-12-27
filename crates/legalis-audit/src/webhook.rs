//! Webhook notifications for audit events.
//!
//! This module provides webhook functionality to notify external systems
//! when audit records are created or specific events occur.

use crate::{AuditError, AuditRecord, AuditResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

/// Webhook configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookConfig {
    /// Webhook endpoint URL
    pub url: String,
    /// Secret for HMAC signature (optional)
    pub secret: Option<String>,
    /// Timeout for webhook requests (seconds)
    pub timeout_secs: u64,
    /// Maximum retry attempts
    pub max_retries: u32,
    /// Retry delay (seconds)
    pub retry_delay_secs: u64,
    /// Event filters (if empty, all events are sent)
    pub event_filters: Vec<String>,
}

impl WebhookConfig {
    /// Creates a new webhook configuration.
    pub fn new(url: String) -> Self {
        Self {
            url,
            secret: None,
            timeout_secs: 10,
            max_retries: 3,
            retry_delay_secs: 5,
            event_filters: Vec::new(),
        }
    }

    /// Sets the webhook secret.
    pub fn with_secret(mut self, secret: String) -> Self {
        self.secret = Some(secret);
        self
    }

    /// Sets the timeout.
    pub fn with_timeout(mut self, timeout_secs: u64) -> Self {
        self.timeout_secs = timeout_secs;
        self
    }

    /// Sets the maximum retries.
    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    /// Adds an event filter.
    pub fn with_event_filter(mut self, filter: String) -> Self {
        self.event_filters.push(filter);
        self
    }

    /// Checks if an event should be sent based on filters.
    pub fn should_send(&self, event_type: &str) -> bool {
        if self.event_filters.is_empty() {
            return true;
        }
        self.event_filters.iter().any(|f| f == event_type)
    }
}

/// Webhook payload for audit events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookPayload {
    /// Event type
    pub event: String,
    /// Timestamp of the webhook
    pub timestamp: DateTime<Utc>,
    /// Audit record
    pub record: AuditRecord,
    /// Additional metadata
    pub metadata: WebhookMetadata,
}

/// Webhook metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookMetadata {
    /// Webhook ID
    pub webhook_id: String,
    /// Source system
    pub source: String,
    /// Delivery attempt number
    pub attempt: u32,
}

/// Webhook delivery result.
#[derive(Debug, Clone)]
pub struct WebhookDeliveryResult {
    /// Whether delivery was successful
    pub success: bool,
    /// HTTP status code
    pub status_code: Option<u16>,
    /// Response body
    pub response_body: Option<String>,
    /// Error message if failed
    pub error: Option<String>,
    /// Number of attempts made
    pub attempts: u32,
}

/// Webhook notifier.
pub struct WebhookNotifier {
    configs: Vec<WebhookConfig>,
    sender: mpsc::Sender<WebhookTask>,
}

struct WebhookTask {
    payload: WebhookPayload,
    configs: Vec<WebhookConfig>,
}

impl WebhookNotifier {
    /// Creates a new webhook notifier.
    pub fn new(configs: Vec<WebhookConfig>) -> Self {
        let (sender, receiver) = mpsc::channel(100);

        // Spawn worker to handle webhook deliveries
        tokio::spawn(async move {
            WebhookWorker::new(receiver).run().await;
        });

        Self { configs, sender }
    }

    /// Sends a webhook notification for a new audit record.
    pub async fn notify(&self, record: AuditRecord) -> AuditResult<()> {
        let event_type = format!("{:?}", record.event_type);

        // Filter configs based on event filters
        let applicable_configs: Vec<_> = self
            .configs
            .iter()
            .filter(|c| c.should_send(&event_type))
            .cloned()
            .collect();

        if applicable_configs.is_empty() {
            return Ok(());
        }

        let payload = WebhookPayload {
            event: event_type,
            timestamp: Utc::now(),
            record,
            metadata: WebhookMetadata {
                webhook_id: uuid::Uuid::new_v4().to_string(),
                source: "legalis-audit".to_string(),
                attempt: 1,
            },
        };

        let task = WebhookTask {
            payload,
            configs: applicable_configs,
        };

        self.sender
            .send(task)
            .await
            .map_err(|_| AuditError::StorageError("Webhook channel closed".to_string()))?;

        Ok(())
    }

    /// Returns the number of configured webhooks.
    pub fn webhook_count(&self) -> usize {
        self.configs.len()
    }
}

/// Internal webhook worker.
struct WebhookWorker {
    receiver: mpsc::Receiver<WebhookTask>,
}

impl WebhookWorker {
    fn new(receiver: mpsc::Receiver<WebhookTask>) -> Self {
        Self { receiver }
    }

    async fn run(mut self) {
        while let Some(task) = self.receiver.recv().await {
            for config in &task.configs {
                self.deliver_webhook(&task.payload, config).await;
            }
        }
    }

    async fn deliver_webhook(&self, payload: &WebhookPayload, config: &WebhookConfig) {
        let mut attempt = 1;

        loop {
            let result = self.send_webhook(payload, config, attempt).await;

            if result.success || attempt >= config.max_retries {
                if result.success {
                    tracing::info!(
                        "Webhook delivered successfully to {} after {} attempts",
                        config.url,
                        attempt
                    );
                } else {
                    tracing::error!(
                        "Webhook delivery failed to {} after {} attempts: {:?}",
                        config.url,
                        attempt,
                        result.error
                    );
                }
                break;
            }

            attempt += 1;
            tokio::time::sleep(tokio::time::Duration::from_secs(config.retry_delay_secs)).await;
        }
    }

    async fn send_webhook(
        &self,
        payload: &WebhookPayload,
        config: &WebhookConfig,
        attempt: u32,
    ) -> WebhookDeliveryResult {
        // In a real implementation, this would use reqwest or similar HTTP client
        // For now, we'll simulate the webhook delivery

        let json_payload = match serde_json::to_string(payload) {
            Ok(json) => json,
            Err(e) => {
                return WebhookDeliveryResult {
                    success: false,
                    status_code: None,
                    response_body: None,
                    error: Some(format!("Failed to serialize payload: {}", e)),
                    attempts: attempt,
                };
            }
        };

        tracing::debug!(
            "Simulating webhook delivery to {} (attempt {}): {} bytes",
            config.url,
            attempt,
            json_payload.len()
        );

        // Simulate successful delivery
        WebhookDeliveryResult {
            success: true,
            status_code: Some(200),
            response_body: Some("OK".to_string()),
            error: None,
            attempts: attempt,
        }
    }
}

/// Webhook event types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebhookEventType {
    /// New audit record created
    AuditRecordCreated,
    /// Integrity verification failed
    IntegrityViolation,
    /// Retention policy applied
    RetentionPolicyApplied,
    /// Data subject access request
    DataSubjectAccessRequest,
}

impl WebhookEventType {
    /// Returns the string representation of the event type.
    pub fn as_str(&self) -> &str {
        match self {
            Self::AuditRecordCreated => "audit.record.created",
            Self::IntegrityViolation => "audit.integrity.violation",
            Self::RetentionPolicyApplied => "audit.retention.applied",
            Self::DataSubjectAccessRequest => "audit.dsar.received",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use std::collections::HashMap;
    use uuid::Uuid;

    #[test]
    fn test_webhook_config() {
        let config = WebhookConfig::new("https://example.com/webhook".to_string())
            .with_secret("secret123".to_string())
            .with_timeout(30)
            .with_max_retries(5)
            .with_event_filter("audit.record.created".to_string());

        assert_eq!(config.url, "https://example.com/webhook");
        assert_eq!(config.secret, Some("secret123".to_string()));
        assert_eq!(config.timeout_secs, 30);
        assert_eq!(config.max_retries, 5);
        assert!(config.should_send("audit.record.created"));
        assert!(!config.should_send("other.event"));
    }

    #[test]
    fn test_webhook_event_filter() {
        let config = WebhookConfig::new("https://example.com/webhook".to_string());

        // No filters means all events should be sent
        assert!(config.should_send("any.event"));

        let config_with_filter = config.with_event_filter("specific.event".to_string());
        assert!(config_with_filter.should_send("specific.event"));
        assert!(!config_with_filter.should_send("other.event"));
    }

    #[tokio::test]
    async fn test_webhook_notifier() {
        let config = WebhookConfig::new("https://example.com/webhook".to_string());
        let notifier = WebhookNotifier::new(vec![config]);

        let record = create_test_record();
        notifier.notify(record).await.unwrap();

        assert_eq!(notifier.webhook_count(), 1);

        // Give some time for the webhook to be processed
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    #[test]
    fn test_webhook_event_types() {
        assert_eq!(
            WebhookEventType::AuditRecordCreated.as_str(),
            "audit.record.created"
        );
        assert_eq!(
            WebhookEventType::IntegrityViolation.as_str(),
            "audit.integrity.violation"
        );
        assert_eq!(
            WebhookEventType::RetentionPolicyApplied.as_str(),
            "audit.retention.applied"
        );
        assert_eq!(
            WebhookEventType::DataSubjectAccessRequest.as_str(),
            "audit.dsar.received"
        );
    }

    fn create_test_record() -> AuditRecord {
        AuditRecord::new(
            EventType::AutomaticDecision,
            Actor::System {
                component: "test".to_string(),
            },
            "statute-1".to_string(),
            Uuid::new_v4(),
            DecisionContext::default(),
            DecisionResult::Deterministic {
                effect_applied: "test".to_string(),
                parameters: HashMap::new(),
            },
            None,
        )
    }
}
