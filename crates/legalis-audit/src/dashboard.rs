//! Live audit dashboard for real-time monitoring.
//!
//! This module provides a real-time dashboard for monitoring audit trail activities,
//! decision patterns, and compliance metrics with live updates.

use crate::{AuditRecord, AuditResult, DecisionResult};
use chrono::{DateTime, Duration, Timelike, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

/// Configuration for the live dashboard.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardConfig {
    /// Update interval in seconds
    pub update_interval_seconds: u64,
    /// History window in hours
    pub history_window_hours: i64,
    /// Maximum events to store in memory
    pub max_events: usize,
    /// Enable performance metrics
    pub enable_performance_metrics: bool,
}

impl Default for DashboardConfig {
    fn default() -> Self {
        Self {
            update_interval_seconds: 5,
            history_window_hours: 24,
            max_events: 10000,
            enable_performance_metrics: true,
        }
    }
}

/// Real-time dashboard snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardSnapshot {
    /// Snapshot timestamp
    pub timestamp: DateTime<Utc>,
    /// Current metrics
    pub metrics: DashboardMetrics,
    /// Recent events
    pub recent_events: Vec<EventSummary>,
    /// Active alerts
    pub active_alerts: Vec<AlertInfo>,
    /// Performance statistics
    pub performance: Option<PerformanceStats>,
}

/// Dashboard metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardMetrics {
    /// Total decisions in time window
    pub total_decisions: usize,
    /// Decisions per minute (current rate)
    pub decisions_per_minute: f64,
    /// Override rate percentage
    pub override_rate: f64,
    /// Discretionary rate percentage
    pub discretionary_rate: f64,
    /// Void rate percentage
    pub void_rate: f64,
    /// Distribution by statute
    pub statute_distribution: HashMap<String, usize>,
    /// Distribution by actor
    pub actor_distribution: HashMap<String, usize>,
    /// Hourly trend data (last 24 hours)
    pub hourly_trends: Vec<HourlyMetric>,
}

/// Hourly metric data point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HourlyMetric {
    /// Hour timestamp
    pub hour: DateTime<Utc>,
    /// Decision count in this hour
    pub count: usize,
    /// Override count in this hour
    pub overrides: usize,
}

/// Event summary for dashboard.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSummary {
    /// Event ID
    pub id: Uuid,
    /// Event timestamp
    pub timestamp: DateTime<Utc>,
    /// Event type description
    pub event_type: String,
    /// Actor description
    pub actor: String,
    /// Statute ID
    pub statute_id: String,
    /// Result type
    pub result_type: String,
    /// Severity level
    pub severity: EventSeverity,
}

/// Event severity level.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum EventSeverity {
    Info,
    Warning,
    High,
    Critical,
}

/// Alert information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertInfo {
    /// Alert ID
    pub id: Uuid,
    /// Alert type
    pub alert_type: AlertType,
    /// Alert message
    pub message: String,
    /// Alert severity
    pub severity: EventSeverity,
    /// Triggered at
    pub triggered_at: DateTime<Utc>,
    /// Related record IDs
    pub related_records: Vec<Uuid>,
}

/// Alert type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AlertType {
    HighOverrideRate,
    VolumeSpike,
    UnusualTiming,
    IntegrityIssue,
    ComplianceViolation,
    Custom(String),
}

/// Performance statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceStats {
    /// Average processing time in milliseconds
    pub avg_processing_time_ms: f64,
    /// P95 processing time in milliseconds
    pub p95_processing_time_ms: f64,
    /// P99 processing time in milliseconds
    pub p99_processing_time_ms: f64,
    /// Memory usage in bytes
    pub memory_usage_bytes: usize,
    /// Cache hit rate
    pub cache_hit_rate: f64,
}

/// Live audit dashboard.
pub struct LiveDashboard {
    config: DashboardConfig,
    state: Arc<RwLock<DashboardState>>,
}

/// Internal dashboard state.
#[derive(Debug)]
struct DashboardState {
    recent_events: Vec<EventSummary>,
    active_alerts: Vec<AlertInfo>,
    metrics_history: Vec<(DateTime<Utc>, DashboardMetrics)>,
    performance_samples: Vec<f64>,
}

impl LiveDashboard {
    /// Creates a new live dashboard with default configuration.
    pub fn new() -> Self {
        Self::with_config(DashboardConfig::default())
    }

    /// Creates a new live dashboard with custom configuration.
    pub fn with_config(config: DashboardConfig) -> Self {
        Self {
            config,
            state: Arc::new(RwLock::new(DashboardState {
                recent_events: Vec::new(),
                active_alerts: Vec::new(),
                metrics_history: Vec::new(),
                performance_samples: Vec::new(),
            })),
        }
    }

    /// Updates the dashboard with new audit records.
    pub fn update(&self, records: &[AuditRecord]) -> AuditResult<()> {
        let cutoff = Utc::now() - Duration::hours(self.config.history_window_hours);
        let recent_records: Vec<_> = records.iter().filter(|r| r.timestamp >= cutoff).collect();

        // Generate event summaries
        let events: Vec<EventSummary> = recent_records
            .iter()
            .rev()
            .take(100)
            .map(|r| self.create_event_summary(r))
            .collect();

        // Calculate metrics
        let metrics = self.calculate_metrics(&recent_records);

        // Detect alerts
        let alerts = self.detect_alerts(&recent_records, &metrics);

        // Update state
        let mut state = self.state.write().unwrap();
        state.recent_events = events;
        state.active_alerts = alerts;
        state.metrics_history.push((Utc::now(), metrics));

        // Trim history
        let history_cutoff = Utc::now() - Duration::hours(24);
        state
            .metrics_history
            .retain(|(ts, _)| *ts >= history_cutoff);

        // Limit recent events
        if state.recent_events.len() > self.config.max_events {
            state.recent_events.truncate(self.config.max_events);
        }

        Ok(())
    }

    /// Gets the current dashboard snapshot.
    pub fn snapshot(&self) -> DashboardSnapshot {
        let state = self.state.read().unwrap();

        let metrics = if let Some((_, latest)) = state.metrics_history.last() {
            latest.clone()
        } else {
            DashboardMetrics {
                total_decisions: 0,
                decisions_per_minute: 0.0,
                override_rate: 0.0,
                discretionary_rate: 0.0,
                void_rate: 0.0,
                statute_distribution: HashMap::new(),
                actor_distribution: HashMap::new(),
                hourly_trends: Vec::new(),
            }
        };

        let performance = if self.config.enable_performance_metrics {
            Some(self.calculate_performance(&state))
        } else {
            None
        };

        DashboardSnapshot {
            timestamp: Utc::now(),
            metrics,
            recent_events: state.recent_events.clone(),
            active_alerts: state.active_alerts.clone(),
            performance,
        }
    }

    /// Adds a performance sample.
    pub fn record_performance(&self, processing_time_ms: f64) {
        let mut state = self.state.write().unwrap();
        state.performance_samples.push(processing_time_ms);

        // Keep only recent samples
        if state.performance_samples.len() > 1000 {
            state.performance_samples.drain(0..500);
        }
    }

    /// Clears all alerts.
    pub fn clear_alerts(&self) {
        let mut state = self.state.write().unwrap();
        state.active_alerts.clear();
    }

    /// Clears a specific alert.
    pub fn clear_alert(&self, alert_id: Uuid) {
        let mut state = self.state.write().unwrap();
        state.active_alerts.retain(|a| a.id != alert_id);
    }

    /// Creates an event summary from an audit record.
    fn create_event_summary(&self, record: &AuditRecord) -> EventSummary {
        let event_type = format!("{:?}", record.event_type);

        let actor = match &record.actor {
            crate::Actor::System { component } => format!("System: {}", component),
            crate::Actor::User { user_id, role } => format!("User: {} ({})", user_id, role),
            crate::Actor::External { system_id } => format!("External: {}", system_id),
        };

        let (result_type, severity) = match &record.result {
            DecisionResult::Deterministic { .. } => {
                ("Deterministic".to_string(), EventSeverity::Info)
            }
            DecisionResult::RequiresDiscretion { .. } => {
                ("Discretionary".to_string(), EventSeverity::Warning)
            }
            DecisionResult::Void { .. } => ("Void".to_string(), EventSeverity::High),
            DecisionResult::Overridden { .. } => ("Override".to_string(), EventSeverity::High),
        };

        EventSummary {
            id: record.id,
            timestamp: record.timestamp,
            event_type,
            actor,
            statute_id: record.statute_id.clone(),
            result_type,
            severity,
        }
    }

    /// Calculates dashboard metrics.
    fn calculate_metrics(&self, records: &[&AuditRecord]) -> DashboardMetrics {
        let total = records.len();

        let override_count = records
            .iter()
            .filter(|r| matches!(r.result, DecisionResult::Overridden { .. }))
            .count();
        let discretionary_count = records
            .iter()
            .filter(|r| matches!(r.result, DecisionResult::RequiresDiscretion { .. }))
            .count();
        let void_count = records
            .iter()
            .filter(|r| matches!(r.result, DecisionResult::Void { .. }))
            .count();

        let override_rate = if total > 0 {
            (override_count as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        let discretionary_rate = if total > 0 {
            (discretionary_count as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        let void_rate = if total > 0 {
            (void_count as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        // Statute distribution
        let mut statute_distribution: HashMap<String, usize> = HashMap::new();
        for record in records {
            *statute_distribution
                .entry(record.statute_id.clone())
                .or_insert(0) += 1;
        }

        // Actor distribution
        let mut actor_distribution: HashMap<String, usize> = HashMap::new();
        for record in records {
            let actor_key = match &record.actor {
                crate::Actor::System { component } => format!("system:{}", component),
                crate::Actor::User { user_id, .. } => format!("user:{}", user_id),
                crate::Actor::External { system_id } => format!("external:{}", system_id),
            };
            *actor_distribution.entry(actor_key).or_insert(0) += 1;
        }

        // Calculate decisions per minute
        let decisions_per_minute = if !records.is_empty() {
            let time_span = if let (Some(first), Some(last)) = (records.first(), records.last()) {
                (last.timestamp - first.timestamp).num_minutes() as f64
            } else {
                1.0
            };

            if time_span > 0.0 {
                total as f64 / time_span
            } else {
                0.0
            }
        } else {
            0.0
        };

        // Hourly trends
        let hourly_trends = self.calculate_hourly_trends(records);

        DashboardMetrics {
            total_decisions: total,
            decisions_per_minute,
            override_rate,
            discretionary_rate,
            void_rate,
            statute_distribution,
            actor_distribution,
            hourly_trends,
        }
    }

    /// Calculates hourly trends.
    fn calculate_hourly_trends(&self, records: &[&AuditRecord]) -> Vec<HourlyMetric> {
        let mut hourly_data: HashMap<DateTime<Utc>, (usize, usize)> = HashMap::new();

        for record in records {
            // Round to hour
            let hour = record
                .timestamp
                .date_naive()
                .and_hms_opt(record.timestamp.time().hour(), 0, 0)
                .unwrap()
                .and_utc();

            let (count, overrides) = hourly_data.entry(hour).or_insert((0, 0));
            *count += 1;
            if matches!(record.result, DecisionResult::Overridden { .. }) {
                *overrides += 1;
            }
        }

        let mut trends: Vec<HourlyMetric> = hourly_data
            .into_iter()
            .map(|(hour, (count, overrides))| HourlyMetric {
                hour,
                count,
                overrides,
            })
            .collect();

        trends.sort_by_key(|m| m.hour);
        trends
    }

    /// Detects alerts from current metrics.
    fn detect_alerts(
        &self,
        records: &[&AuditRecord],
        metrics: &DashboardMetrics,
    ) -> Vec<AlertInfo> {
        let mut alerts = Vec::new();

        // High override rate alert
        if metrics.override_rate > 20.0 {
            alerts.push(AlertInfo {
                id: Uuid::new_v4(),
                alert_type: AlertType::HighOverrideRate,
                message: format!("High override rate detected: {:.1}%", metrics.override_rate),
                severity: if metrics.override_rate > 40.0 {
                    EventSeverity::Critical
                } else {
                    EventSeverity::High
                },
                triggered_at: Utc::now(),
                related_records: records
                    .iter()
                    .filter(|r| matches!(r.result, DecisionResult::Overridden { .. }))
                    .map(|r| r.id)
                    .take(10)
                    .collect(),
            });
        }

        // Volume spike alert
        if metrics.decisions_per_minute > 100.0 {
            alerts.push(AlertInfo {
                id: Uuid::new_v4(),
                alert_type: AlertType::VolumeSpike,
                message: format!(
                    "High decision volume: {:.0} decisions/minute",
                    metrics.decisions_per_minute
                ),
                severity: EventSeverity::Warning,
                triggered_at: Utc::now(),
                related_records: Vec::new(),
            });
        }

        // Unusual timing alert (night activity)
        let night_activity = records
            .iter()
            .filter(|r| {
                let hour = r.timestamp.time().hour();
                !(6..22).contains(&hour)
            })
            .count();

        let night_percentage = if !records.is_empty() {
            (night_activity as f64 / records.len() as f64) * 100.0
        } else {
            0.0
        };

        if night_percentage > 30.0 {
            alerts.push(AlertInfo {
                id: Uuid::new_v4(),
                alert_type: AlertType::UnusualTiming,
                message: format!(
                    "Significant night-time activity: {:.1}% of decisions",
                    night_percentage
                ),
                severity: EventSeverity::Warning,
                triggered_at: Utc::now(),
                related_records: Vec::new(),
            });
        }

        alerts
    }

    /// Calculates performance statistics.
    fn calculate_performance(&self, state: &DashboardState) -> PerformanceStats {
        let samples = &state.performance_samples;

        if samples.is_empty() {
            return PerformanceStats {
                avg_processing_time_ms: 0.0,
                p95_processing_time_ms: 0.0,
                p99_processing_time_ms: 0.0,
                memory_usage_bytes: 0,
                cache_hit_rate: 0.0,
            };
        }

        let avg = samples.iter().sum::<f64>() / samples.len() as f64;

        let mut sorted = samples.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let p95_idx = (sorted.len() as f64 * 0.95) as usize;
        let p99_idx = (sorted.len() as f64 * 0.99) as usize;

        let p95 = sorted.get(p95_idx).copied().unwrap_or(0.0);
        let p99 = sorted.get(p99_idx).copied().unwrap_or(0.0);

        PerformanceStats {
            avg_processing_time_ms: avg,
            p95_processing_time_ms: p95,
            p99_processing_time_ms: p99,
            memory_usage_bytes: std::mem::size_of_val(&*state),
            cache_hit_rate: 0.0, // Placeholder
        }
    }

    /// Returns the configuration.
    pub fn config(&self) -> &DashboardConfig {
        &self.config
    }

    /// Returns the number of active alerts.
    pub fn active_alert_count(&self) -> usize {
        self.state.read().unwrap().active_alerts.len()
    }
}

impl Default for LiveDashboard {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, DecisionContext, EventType};
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
    fn test_dashboard_creation() {
        let dashboard = LiveDashboard::new();
        assert_eq!(dashboard.config().update_interval_seconds, 5);
    }

    #[test]
    fn test_dashboard_update() {
        let dashboard = LiveDashboard::new();

        let records: Vec<_> = (0..20).map(|i| create_test_record(i, i % 5 == 0)).collect();

        let result = dashboard.update(&records);
        assert!(result.is_ok());

        let snapshot = dashboard.snapshot();
        assert_eq!(snapshot.metrics.total_decisions, 20);
    }

    #[test]
    fn test_dashboard_snapshot() {
        let dashboard = LiveDashboard::new();

        let records: Vec<_> = (0..10).map(|i| create_test_record(i, false)).collect();

        dashboard.update(&records).unwrap();

        let snapshot = dashboard.snapshot();
        assert!(!snapshot.recent_events.is_empty());
        assert_eq!(snapshot.metrics.total_decisions, 10);
    }

    #[test]
    fn test_alert_detection() {
        let dashboard = LiveDashboard::new();

        // Create many override records to trigger alert
        let records: Vec<_> = (0..10).map(|i| create_test_record(i, true)).collect();

        dashboard.update(&records).unwrap();

        let snapshot = dashboard.snapshot();
        assert!(!snapshot.active_alerts.is_empty());
        assert!(
            snapshot
                .active_alerts
                .iter()
                .any(|a| a.alert_type == AlertType::HighOverrideRate)
        );
    }

    #[test]
    fn test_performance_recording() {
        let dashboard = LiveDashboard::new();

        dashboard.record_performance(10.0);
        dashboard.record_performance(20.0);
        dashboard.record_performance(15.0);

        let snapshot = dashboard.snapshot();
        assert!(snapshot.performance.is_some());

        if let Some(perf) = snapshot.performance {
            assert!(perf.avg_processing_time_ms > 0.0);
        }
    }

    #[test]
    fn test_clear_alerts() {
        let dashboard = LiveDashboard::new();

        let records: Vec<_> = (0..10).map(|i| create_test_record(i, true)).collect();

        dashboard.update(&records).unwrap();
        assert!(dashboard.active_alert_count() > 0);

        dashboard.clear_alerts();
        assert_eq!(dashboard.active_alert_count(), 0);
    }

    #[test]
    fn test_hourly_trends() {
        let dashboard = LiveDashboard::new();

        let records: Vec<_> = (0..48).map(|i| create_test_record(i, false)).collect();

        dashboard.update(&records).unwrap();

        let snapshot = dashboard.snapshot();
        assert!(!snapshot.metrics.hourly_trends.is_empty());
    }
}
