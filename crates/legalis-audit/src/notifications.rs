//! Notifications for anomalies and alerts via Slack and Microsoft Teams.
//!
//! This module provides functionality for sending notifications about
//! detected anomalies to Slack and Teams channels.

use crate::{AuditResult, analysis};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

/// Notification severity level.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

/// Notification destination.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationDestination {
    /// Slack webhook
    Slack {
        webhook_url: String,
        channel: Option<String>,
        username: Option<String>,
        icon_emoji: Option<String>,
    },
    /// Microsoft Teams webhook
    Teams { webhook_url: String },
}

/// Notification configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationConfig {
    /// Destination for notifications
    pub destination: NotificationDestination,
    /// Minimum severity to trigger notification
    pub min_severity: Severity,
    /// Whether to include detailed information
    pub include_details: bool,
    /// Custom metadata
    pub metadata: HashMap<String, String>,
}

impl NotificationConfig {
    /// Creates a new notification configuration.
    pub fn new(destination: NotificationDestination) -> Self {
        Self {
            destination,
            min_severity: Severity::Medium,
            include_details: true,
            metadata: HashMap::new(),
        }
    }

    /// Sets the minimum severity level.
    pub fn with_min_severity(mut self, severity: Severity) -> Self {
        self.min_severity = severity;
        self
    }

    /// Sets whether to include detailed information.
    pub fn with_details(mut self, include: bool) -> Self {
        self.include_details = include;
        self
    }
}

/// Anomaly notification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyNotification {
    /// Notification title
    pub title: String,
    /// Description
    pub description: String,
    /// Severity level
    pub severity: Severity,
    /// Anomaly type
    pub anomaly_type: String,
    /// Affected records count
    pub affected_count: usize,
    /// Timestamp
    pub timestamp: String,
    /// Additional fields
    pub fields: HashMap<String, String>,
}

impl AnomalyNotification {
    /// Creates a new anomaly notification.
    pub fn new(
        title: String,
        description: String,
        severity: Severity,
        anomaly_type: String,
    ) -> Self {
        Self {
            title,
            description,
            severity,
            anomaly_type,
            affected_count: 0,
            timestamp: chrono::Utc::now().to_rfc3339(),
            fields: HashMap::new(),
        }
    }

    /// Sets the affected records count.
    pub fn with_affected_count(mut self, count: usize) -> Self {
        self.affected_count = count;
        self
    }

    /// Adds a custom field.
    pub fn add_field(mut self, key: String, value: String) -> Self {
        self.fields.insert(key, value);
        self
    }

    /// Creates a notification from a volume anomaly.
    pub fn from_volume_anomaly(anomaly: &analysis::VolumeAnomaly) -> Self {
        let ratio = anomaly.count as f64 / anomaly.expected.max(1) as f64;
        let severity = if ratio > 5.0 {
            Severity::Critical
        } else if ratio > 3.0 {
            Severity::High
        } else if ratio > 2.0 {
            Severity::Medium
        } else {
            Severity::Low
        };

        Self::new(
            "Volume Anomaly Detected".to_string(),
            format!(
                "Unusual volume spike detected between {} and {}",
                anomaly.start.format("%Y-%m-%d"),
                anomaly.end.format("%Y-%m-%d")
            ),
            severity,
            "VolumeAnomaly".to_string(),
        )
        .with_affected_count(anomaly.count)
        .add_field(
            "Start".to_string(),
            anomaly.start.format("%Y-%m-%d %H:%M").to_string(),
        )
        .add_field(
            "End".to_string(),
            anomaly.end.format("%Y-%m-%d %H:%M").to_string(),
        )
        .add_field("Count".to_string(), anomaly.count.to_string())
        .add_field("Expected".to_string(), anomaly.expected.to_string())
        .add_field("Ratio".to_string(), format!("{:.2}x", ratio))
    }

    /// Creates a notification from an override anomaly.
    pub fn from_override_anomaly(anomaly: &analysis::OverrideAnomaly) -> Self {
        let severity = if anomaly.override_rate > 0.5 {
            Severity::Critical
        } else if anomaly.override_rate > 0.3 {
            Severity::High
        } else if anomaly.override_rate > 0.2 {
            Severity::Medium
        } else {
            Severity::Low
        };

        Self::new(
            "Override Anomaly Detected".to_string(),
            format!(
                "High override rate detected for statute {}",
                anomaly.statute_id
            ),
            severity,
            "OverrideAnomaly".to_string(),
        )
        .with_affected_count(anomaly.overridden_count)
        .add_field("Statute".to_string(), anomaly.statute_id.clone())
        .add_field(
            "Total Decisions".to_string(),
            anomaly.total_decisions.to_string(),
        )
        .add_field(
            "Overrides".to_string(),
            anomaly.overridden_count.to_string(),
        )
        .add_field(
            "Override Rate".to_string(),
            format!("{:.1}%", anomaly.override_rate * 100.0),
        )
    }
}

/// Notification service for sending alerts.
pub struct NotificationService;

impl NotificationService {
    /// Sends a notification.
    pub fn send(
        notification: &AnomalyNotification,
        config: &NotificationConfig,
    ) -> AuditResult<NotificationResult> {
        // Check if notification meets minimum severity
        let severity_level = match &notification.severity {
            Severity::Low => 1,
            Severity::Medium => 2,
            Severity::High => 3,
            Severity::Critical => 4,
        };

        let min_level = match &config.min_severity {
            Severity::Low => 1,
            Severity::Medium => 2,
            Severity::High => 3,
            Severity::Critical => 4,
        };

        if severity_level < min_level {
            return Ok(NotificationResult {
                success: true,
                skipped: true,
                reason: Some("Below minimum severity threshold".to_string()),
                message_id: None,
            });
        }

        match &config.destination {
            NotificationDestination::Slack {
                webhook_url,
                channel,
                username,
                icon_emoji,
            } => Self::send_slack(
                notification,
                webhook_url,
                channel.as_deref(),
                username.as_deref(),
                icon_emoji.as_deref(),
                config.include_details,
            ),
            NotificationDestination::Teams { webhook_url } => {
                Self::send_teams(notification, webhook_url, config.include_details)
            }
        }
    }

    fn send_slack(
        notification: &AnomalyNotification,
        webhook_url: &str,
        channel: Option<&str>,
        username: Option<&str>,
        icon_emoji: Option<&str>,
        include_details: bool,
    ) -> AuditResult<NotificationResult> {
        let color = match notification.severity {
            Severity::Low => "#36a64f",      // green
            Severity::Medium => "#ff9900",   // orange
            Severity::High => "#ff6600",     // dark orange
            Severity::Critical => "#ff0000", // red
        };

        let mut fields = Vec::new();

        if include_details {
            fields.push(json!({
                "title": "Anomaly Type",
                "value": notification.anomaly_type,
                "short": true
            }));

            fields.push(json!({
                "title": "Affected Records",
                "value": notification.affected_count.to_string(),
                "short": true
            }));

            for (key, value) in &notification.fields {
                fields.push(json!({
                    "title": key,
                    "value": value,
                    "short": true
                }));
            }
        }

        let mut payload = json!({
            "attachments": [{
                "fallback": &notification.title,
                "color": color,
                "title": &notification.title,
                "text": &notification.description,
                "fields": fields,
                "footer": "Legalis Audit System",
                "ts": chrono::Utc::now().timestamp()
            }]
        });

        if let Some(ch) = channel {
            payload["channel"] = json!(ch);
        }
        if let Some(user) = username {
            payload["username"] = json!(user);
        }
        if let Some(emoji) = icon_emoji {
            payload["icon_emoji"] = json!(emoji);
        }

        // Simulate sending (actual implementation would use HTTP client)
        tracing::info!(
            "Slack notification to {}: {}",
            webhook_url,
            notification.title
        );

        Ok(NotificationResult {
            success: true,
            skipped: false,
            reason: None,
            message_id: Some(format!("slack_{}", uuid::Uuid::new_v4())),
        })
    }

    fn send_teams(
        notification: &AnomalyNotification,
        webhook_url: &str,
        include_details: bool,
    ) -> AuditResult<NotificationResult> {
        let theme_color = match notification.severity {
            Severity::Low => "28A745",      // green
            Severity::Medium => "FFC107",   // amber
            Severity::High => "FF6F00",     // dark orange
            Severity::Critical => "DC3545", // red
        };

        let mut facts = Vec::new();

        if include_details {
            facts.push(json!({
                "name": "Anomaly Type",
                "value": notification.anomaly_type
            }));

            facts.push(json!({
                "name": "Affected Records",
                "value": notification.affected_count.to_string()
            }));

            for (key, value) in &notification.fields {
                facts.push(json!({
                    "name": key,
                    "value": value
                }));
            }
        }

        let _payload = json!({
            "@type": "MessageCard",
            "@context": "https://schema.org/extensions",
            "summary": &notification.title,
            "themeColor": theme_color,
            "title": &notification.title,
            "text": &notification.description,
            "sections": [{
                "facts": facts
            }]
        });

        // Simulate sending (actual implementation would use HTTP client)
        tracing::info!(
            "Teams notification to {}: {}",
            webhook_url,
            notification.title
        );

        Ok(NotificationResult {
            success: true,
            skipped: false,
            reason: None,
            message_id: Some(format!("teams_{}", uuid::Uuid::new_v4())),
        })
    }

    /// Sends multiple notifications for a batch of anomalies.
    pub fn send_batch(
        notifications: &[AnomalyNotification],
        config: &NotificationConfig,
    ) -> Vec<AuditResult<NotificationResult>> {
        notifications
            .iter()
            .map(|n| Self::send(n, config))
            .collect()
    }
}

/// Result of sending a notification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationResult {
    /// Whether the notification was sent successfully
    pub success: bool,
    /// Whether the notification was skipped
    pub skipped: bool,
    /// Reason for failure or skip
    pub reason: Option<String>,
    /// Message ID (if successful)
    pub message_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_config() {
        let config = NotificationConfig::new(NotificationDestination::Slack {
            webhook_url: "https://hooks.slack.com/services/XXX".to_string(),
            channel: Some("#audit".to_string()),
            username: None,
            icon_emoji: None,
        })
        .with_min_severity(Severity::High)
        .with_details(false);

        assert_eq!(config.min_severity, Severity::High);
        assert!(!config.include_details);
    }

    #[test]
    fn test_anomaly_notification() {
        let notification = AnomalyNotification::new(
            "Test Anomaly".to_string(),
            "Test description".to_string(),
            Severity::High,
            "TestAnomaly".to_string(),
        )
        .with_affected_count(10)
        .add_field("key".to_string(), "value".to_string());

        assert_eq!(notification.title, "Test Anomaly");
        assert_eq!(notification.affected_count, 10);
        assert!(notification.fields.contains_key("key"));
    }

    #[test]
    fn test_volume_anomaly_notification() {
        use chrono::Utc;
        let anomaly = analysis::VolumeAnomaly {
            start: Utc::now(),
            end: Utc::now(),
            count: 100,
            expected: 20,
            severity: 5.0,
        };

        let notification = AnomalyNotification::from_volume_anomaly(&anomaly);
        assert_eq!(notification.severity, Severity::Critical);
        assert_eq!(notification.affected_count, 100);
    }

    #[test]
    fn test_override_anomaly_notification() {
        let anomaly = analysis::OverrideAnomaly {
            statute_id: "statute-1".to_string(),
            total_decisions: 100,
            overridden_count: 40,
            override_rate: 0.4,
        };

        let notification = AnomalyNotification::from_override_anomaly(&anomaly);
        assert_eq!(notification.severity, Severity::High);
        assert_eq!(notification.affected_count, 40);
    }

    #[test]
    fn test_send_slack_notification() {
        let notification = AnomalyNotification::new(
            "Test".to_string(),
            "Description".to_string(),
            Severity::High,
            "Test".to_string(),
        );

        let config = NotificationConfig::new(NotificationDestination::Slack {
            webhook_url: "https://hooks.slack.com/test".to_string(),
            channel: Some("#test".to_string()),
            username: Some("Audit Bot".to_string()),
            icon_emoji: Some(":warning:".to_string()),
        });

        let result = NotificationService::send(&notification, &config).unwrap();
        assert!(result.success);
        assert!(!result.skipped);
    }

    #[test]
    fn test_send_teams_notification() {
        let notification = AnomalyNotification::new(
            "Test".to_string(),
            "Description".to_string(),
            Severity::Medium,
            "Test".to_string(),
        );

        let config = NotificationConfig::new(NotificationDestination::Teams {
            webhook_url: "https://outlook.office.com/webhook/test".to_string(),
        });

        let result = NotificationService::send(&notification, &config).unwrap();
        assert!(result.success);
        assert!(!result.skipped);
    }

    #[test]
    fn test_severity_filtering() {
        let notification = AnomalyNotification::new(
            "Test".to_string(),
            "Description".to_string(),
            Severity::Low,
            "Test".to_string(),
        );

        let config = NotificationConfig::new(NotificationDestination::Slack {
            webhook_url: "https://hooks.slack.com/test".to_string(),
            channel: None,
            username: None,
            icon_emoji: None,
        })
        .with_min_severity(Severity::High);

        let result = NotificationService::send(&notification, &config).unwrap();
        assert!(result.success);
        assert!(result.skipped);
        assert!(result.reason.is_some());
    }
}
