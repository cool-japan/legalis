//! Real-time alerting system for audit trail monitoring.
//!
//! This module provides real-time alert generation, routing, and notification
//! for critical audit events and compliance violations.

use crate::{AuditError, AuditRecord, AuditResult};
use chrono::{DateTime, Duration, Timelike, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

/// Configuration for real-time alerting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertConfig {
    /// Enable alert throttling
    pub enable_throttling: bool,
    /// Throttle window in seconds
    pub throttle_window_seconds: u64,
    /// Maximum alerts per window
    pub max_alerts_per_window: usize,
    /// Alert channels
    pub channels: Vec<AlertChannel>,
}

impl Default for AlertConfig {
    fn default() -> Self {
        Self {
            enable_throttling: true,
            throttle_window_seconds: 300, // 5 minutes
            max_alerts_per_window: 10,
            channels: vec![AlertChannel::Log],
        }
    }
}

/// Alert channel for delivery.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AlertChannel {
    /// Log to system log
    Log,
    /// Email notification
    Email(String),
    /// Webhook notification
    Webhook(String),
    /// Slack notification
    Slack(String),
    /// Custom channel
    Custom(String),
}

/// Real-time alert.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    /// Alert ID
    pub id: Uuid,
    /// Alert type
    pub alert_type: AlertType,
    /// Alert severity
    pub severity: AlertSeverity,
    /// Alert title
    pub title: String,
    /// Alert message
    pub message: String,
    /// Triggered timestamp
    pub triggered_at: DateTime<Utc>,
    /// Related record IDs
    pub related_records: Vec<Uuid>,
    /// Alert metadata
    pub metadata: HashMap<String, String>,
    /// Acknowledgement status
    pub acknowledged: bool,
}

/// Alert type classification.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AlertType {
    /// Integrity violation
    IntegrityViolation,
    /// High override rate
    HighOverrideRate,
    /// Volume anomaly
    VolumeAnomaly,
    /// Unusual timing
    UnusualTiming,
    /// Compliance breach
    ComplianceBreach,
    /// Performance degradation
    PerformanceDegradation,
    /// Security incident
    SecurityIncident,
    /// Custom alert type
    Custom(String),
}

/// Alert severity level.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum AlertSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Alert rule definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRule {
    /// Rule ID
    pub id: String,
    /// Rule name
    pub name: String,
    /// Rule enabled status
    pub enabled: bool,
    /// Alert type to generate
    pub alert_type: AlertType,
    /// Alert severity
    pub severity: AlertSeverity,
    /// Rule condition
    pub condition: RuleCondition,
    /// Channels to notify
    pub channels: Vec<AlertChannel>,
}

/// Alert rule condition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleCondition {
    /// Override rate exceeds threshold
    OverrideRateThreshold { threshold: f64 },
    /// Decision volume exceeds threshold
    VolumeThreshold { decisions_per_minute: f64 },
    /// Percentage of night-time activity
    NightActivityThreshold { percentage: f64 },
    /// Discretionary rate threshold
    DiscretionaryRateThreshold { threshold: f64 },
    /// Custom condition
    Custom { description: String },
}

/// Alert history type.
type AlertHistory = Arc<RwLock<Vec<(DateTime<Utc>, AlertType)>>>;

/// Real-time alert manager.
pub struct AlertManager {
    config: AlertConfig,
    rules: Arc<RwLock<Vec<AlertRule>>>,
    active_alerts: Arc<RwLock<Vec<Alert>>>,
    alert_history: AlertHistory,
}

impl AlertManager {
    /// Creates a new alert manager with default configuration.
    pub fn new() -> Self {
        Self::with_config(AlertConfig::default())
    }

    /// Creates a new alert manager with custom configuration.
    pub fn with_config(config: AlertConfig) -> Self {
        Self {
            config,
            rules: Arc::new(RwLock::new(Vec::new())),
            active_alerts: Arc::new(RwLock::new(Vec::new())),
            alert_history: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Adds an alert rule.
    pub fn add_rule(&self, rule: AlertRule) {
        let mut rules = self.rules.write().unwrap();
        rules.push(rule);
    }

    /// Removes an alert rule.
    pub fn remove_rule(&self, rule_id: &str) {
        let mut rules = self.rules.write().unwrap();
        rules.retain(|r| r.id != rule_id);
    }

    /// Evaluates all rules against current audit records.
    pub fn evaluate(&self, records: &[AuditRecord]) -> AuditResult<Vec<Alert>> {
        let rules = self.rules.read().unwrap();
        let mut new_alerts = Vec::new();

        for rule in rules.iter().filter(|r| r.enabled) {
            if let Some(alert) = self.evaluate_rule(rule, records)? {
                // Check throttling
                if self.should_send_alert(&alert) {
                    new_alerts.push(alert.clone());
                    self.record_alert(&alert);
                }
            }
        }

        // Update active alerts
        let mut active = self.active_alerts.write().unwrap();
        active.extend(new_alerts.clone());

        // Cleanup old alerts
        let cutoff = Utc::now() - Duration::hours(24);
        active.retain(|a| a.triggered_at >= cutoff);

        Ok(new_alerts)
    }

    /// Evaluates a single rule.
    fn evaluate_rule(
        &self,
        rule: &AlertRule,
        records: &[AuditRecord],
    ) -> AuditResult<Option<Alert>> {
        let triggered = match &rule.condition {
            RuleCondition::OverrideRateThreshold { threshold } => {
                let override_count = records
                    .iter()
                    .filter(|r| matches!(r.result, crate::DecisionResult::Overridden { .. }))
                    .count();

                let rate = if !records.is_empty() {
                    (override_count as f64 / records.len() as f64) * 100.0
                } else {
                    0.0
                };

                rate > *threshold
            }
            RuleCondition::VolumeThreshold {
                decisions_per_minute,
            } => {
                if records.len() < 2 {
                    return Ok(None);
                }

                let time_span = if let (Some(first), Some(last)) = (records.first(), records.last())
                {
                    (last.timestamp - first.timestamp).num_minutes() as f64
                } else {
                    1.0
                };

                let current_rate = if time_span > 0.0 {
                    records.len() as f64 / time_span
                } else {
                    0.0
                };

                current_rate > *decisions_per_minute
            }
            RuleCondition::NightActivityThreshold { percentage } => {
                let night_count = records
                    .iter()
                    .filter(|r| {
                        let hour = r.timestamp.time().hour();
                        !(6..22).contains(&hour)
                    })
                    .count();

                let night_percentage = if !records.is_empty() {
                    (night_count as f64 / records.len() as f64) * 100.0
                } else {
                    0.0
                };

                night_percentage > *percentage
            }
            RuleCondition::DiscretionaryRateThreshold { threshold } => {
                let discretionary_count = records
                    .iter()
                    .filter(|r| {
                        matches!(r.result, crate::DecisionResult::RequiresDiscretion { .. })
                    })
                    .count();

                let rate = if !records.is_empty() {
                    (discretionary_count as f64 / records.len() as f64) * 100.0
                } else {
                    0.0
                };

                rate > *threshold
            }
            RuleCondition::Custom { .. } => false,
        };

        if triggered {
            let related_records = records.iter().map(|r| r.id).take(10).collect();

            let message = match &rule.condition {
                RuleCondition::OverrideRateThreshold { threshold } => {
                    format!("Override rate exceeded threshold of {}%", threshold)
                }
                RuleCondition::VolumeThreshold {
                    decisions_per_minute,
                } => {
                    format!(
                        "Decision volume exceeded {} decisions/minute",
                        decisions_per_minute
                    )
                }
                RuleCondition::NightActivityThreshold { percentage } => {
                    format!("Night-time activity exceeded {}% threshold", percentage)
                }
                RuleCondition::DiscretionaryRateThreshold { threshold } => {
                    format!("Discretionary rate exceeded threshold of {}%", threshold)
                }
                RuleCondition::Custom { description } => description.clone(),
            };

            Ok(Some(Alert {
                id: Uuid::new_v4(),
                alert_type: rule.alert_type.clone(),
                severity: rule.severity.clone(),
                title: rule.name.clone(),
                message,
                triggered_at: Utc::now(),
                related_records,
                metadata: HashMap::new(),
                acknowledged: false,
            }))
        } else {
            Ok(None)
        }
    }

    /// Checks if an alert should be sent based on throttling rules.
    fn should_send_alert(&self, alert: &Alert) -> bool {
        if !self.config.enable_throttling {
            return true;
        }

        let history = self.alert_history.read().unwrap();
        let cutoff = Utc::now() - Duration::seconds(self.config.throttle_window_seconds as i64);

        let recent_count = history
            .iter()
            .filter(|(ts, alert_type)| *ts >= cutoff && *alert_type == alert.alert_type)
            .count();

        recent_count < self.config.max_alerts_per_window
    }

    /// Records an alert in history.
    fn record_alert(&self, alert: &Alert) {
        let mut history = self.alert_history.write().unwrap();
        history.push((alert.triggered_at, alert.alert_type.clone()));

        // Cleanup old history
        let cutoff = Utc::now() - Duration::hours(1);
        history.retain(|(ts, _)| *ts >= cutoff);
    }

    /// Acknowledges an alert.
    pub fn acknowledge_alert(&self, alert_id: Uuid) -> AuditResult<()> {
        let mut active = self.active_alerts.write().unwrap();

        if let Some(alert) = active.iter_mut().find(|a| a.id == alert_id) {
            alert.acknowledged = true;
            Ok(())
        } else {
            Err(AuditError::RecordNotFound(alert_id))
        }
    }

    /// Gets all active alerts.
    pub fn get_active_alerts(&self) -> Vec<Alert> {
        self.active_alerts.read().unwrap().clone()
    }

    /// Gets unacknowledged alerts.
    pub fn get_unacknowledged_alerts(&self) -> Vec<Alert> {
        self.active_alerts
            .read()
            .unwrap()
            .iter()
            .filter(|a| !a.acknowledged)
            .cloned()
            .collect()
    }

    /// Clears all acknowledged alerts.
    pub fn clear_acknowledged(&self) {
        let mut active = self.active_alerts.write().unwrap();
        active.retain(|a| !a.acknowledged);
    }

    /// Returns the configuration.
    pub fn config(&self) -> &AlertConfig {
        &self.config
    }
}

impl Default for AlertManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use std::collections::HashMap;

    fn create_test_record(hours_ago: i64, is_override: bool) -> AuditRecord {
        let timestamp = Utc::now() - Duration::hours(hours_ago);

        let result = if is_override {
            DecisionResult::Overridden {
                original_result: Box::new(DecisionResult::Deterministic {
                    effect_applied: "approved".to_string(),
                    parameters: HashMap::new(),
                }),
                new_result: Box::new(DecisionResult::Deterministic {
                    effect_applied: "denied".to_string(),
                    parameters: HashMap::new(),
                }),
                justification: "test".to_string(),
            }
        } else {
            DecisionResult::Deterministic {
                effect_applied: "approved".to_string(),
                parameters: HashMap::new(),
            }
        };

        AuditRecord {
            id: Uuid::new_v4(),
            timestamp,
            event_type: EventType::AutomaticDecision,
            actor: Actor::System {
                component: "test".to_string(),
            },
            statute_id: "test-statute".to_string(),
            subject_id: Uuid::new_v4(),
            context: DecisionContext::default(),
            result,
            previous_hash: None,
            record_hash: String::new(),
        }
    }

    #[test]
    fn test_alert_manager_creation() {
        let manager = AlertManager::new();
        assert_eq!(manager.config().throttle_window_seconds, 300);
    }

    #[test]
    fn test_add_remove_rule() {
        let manager = AlertManager::new();

        let rule = AlertRule {
            id: "test-rule".to_string(),
            name: "Test Rule".to_string(),
            enabled: true,
            alert_type: AlertType::HighOverrideRate,
            severity: AlertSeverity::High,
            condition: RuleCondition::OverrideRateThreshold { threshold: 20.0 },
            channels: vec![AlertChannel::Log],
        };

        manager.add_rule(rule);

        let rules = manager.rules.read().unwrap();
        assert_eq!(rules.len(), 1);
        drop(rules);

        manager.remove_rule("test-rule");

        let rules = manager.rules.read().unwrap();
        assert_eq!(rules.len(), 0);
    }

    #[test]
    fn test_alert_evaluation() {
        let manager = AlertManager::new();

        let rule = AlertRule {
            id: "override-rule".to_string(),
            name: "High Override Rate".to_string(),
            enabled: true,
            alert_type: AlertType::HighOverrideRate,
            severity: AlertSeverity::High,
            condition: RuleCondition::OverrideRateThreshold { threshold: 20.0 },
            channels: vec![AlertChannel::Log],
        };

        manager.add_rule(rule);

        // Create records with high override rate
        let records: Vec<_> = (0..10).map(|i| create_test_record(i, i < 5)).collect();

        let alerts = manager.evaluate(&records).unwrap();
        assert!(!alerts.is_empty());
        assert_eq!(alerts[0].alert_type, AlertType::HighOverrideRate);
    }

    #[test]
    fn test_alert_throttling() {
        let mut config = AlertConfig::default();
        config.max_alerts_per_window = 2;

        let manager = AlertManager::with_config(config);

        let rule = AlertRule {
            id: "test-rule".to_string(),
            name: "Test Rule".to_string(),
            enabled: true,
            alert_type: AlertType::HighOverrideRate,
            severity: AlertSeverity::High,
            condition: RuleCondition::OverrideRateThreshold { threshold: 10.0 },
            channels: vec![AlertChannel::Log],
        };

        manager.add_rule(rule);

        let records: Vec<_> = (0..10).map(|i| create_test_record(i, true)).collect();

        // First evaluation should trigger alert
        let alerts1 = manager.evaluate(&records).unwrap();
        assert_eq!(alerts1.len(), 1);

        // Second evaluation should trigger alert
        let alerts2 = manager.evaluate(&records).unwrap();
        assert_eq!(alerts2.len(), 1);

        // Third evaluation should be throttled
        let alerts3 = manager.evaluate(&records).unwrap();
        assert_eq!(alerts3.len(), 0);
    }

    #[test]
    fn test_acknowledge_alert() {
        let manager = AlertManager::new();

        let rule = AlertRule {
            id: "test-rule".to_string(),
            name: "Test Rule".to_string(),
            enabled: true,
            alert_type: AlertType::HighOverrideRate,
            severity: AlertSeverity::High,
            condition: RuleCondition::OverrideRateThreshold { threshold: 20.0 },
            channels: vec![AlertChannel::Log],
        };

        manager.add_rule(rule);

        let records: Vec<_> = (0..10).map(|i| create_test_record(i, i < 5)).collect();

        let alerts = manager.evaluate(&records).unwrap();
        assert!(!alerts.is_empty());

        let alert_id = alerts[0].id;
        manager.acknowledge_alert(alert_id).unwrap();

        let unack = manager.get_unacknowledged_alerts();
        assert_eq!(unack.len(), 0);
    }

    #[test]
    fn test_get_active_alerts() {
        let manager = AlertManager::new();

        // Use OverrideRateThreshold instead of VolumeThreshold
        // since VolumeThreshold requires time spread between records
        let rule = AlertRule {
            id: "test-rule".to_string(),
            name: "Test Rule".to_string(),
            enabled: true,
            alert_type: AlertType::HighOverrideRate,
            severity: AlertSeverity::Medium,
            condition: RuleCondition::OverrideRateThreshold { threshold: 20.0 },
            channels: vec![AlertChannel::Log],
        };

        manager.add_rule(rule);

        // Create records with >20% override rate (50 overrides out of 100 = 50%)
        let records: Vec<_> = (0..100).map(|i| create_test_record(0, i < 50)).collect();

        manager.evaluate(&records).unwrap();

        let active = manager.get_active_alerts();
        assert!(!active.is_empty());
    }
}
