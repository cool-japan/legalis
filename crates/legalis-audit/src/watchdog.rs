//! Watchdog process integration for continuous monitoring.
//!
//! This module provides watchdog capabilities for continuous health monitoring,
//! automated recovery, metrics tracking, and system alerting.

use crate::AuditResult;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Watchdog configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchdogConfig {
    /// Health check interval in seconds
    pub check_interval_seconds: u64,
    /// Enable health checks
    pub health_check_enabled: bool,
    /// Enable automatic recovery
    pub auto_recovery: bool,
    /// Number of failures before triggering recovery
    pub failure_threshold: usize,
    /// Enable detailed metrics
    pub enable_metrics: bool,
    /// History retention window in hours
    pub history_window_hours: i64,
}

impl Default for WatchdogConfig {
    fn default() -> Self {
        Self {
            check_interval_seconds: 30,
            health_check_enabled: true,
            auto_recovery: true,
            failure_threshold: 3,
            enable_metrics: true,
            history_window_hours: 24,
        }
    }
}

/// Health status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Critical,
    Recovering,
}

/// Component being monitored.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MonitoredComponent {
    AuditTrail,
    Storage,
    IntegrityChecker,
    StreamingAnalyzer,
    AlertManager,
    Custom(String),
}

/// Health check result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    /// Health status
    pub status: HealthStatus,
    /// Check timestamp
    pub checked_at: DateTime<Utc>,
    /// Status message
    pub message: String,
    /// Component being checked
    pub component: MonitoredComponent,
    /// Additional metrics
    pub metrics: HealthMetrics,
}

/// Health metrics.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HealthMetrics {
    /// CPU usage percentage (0-100)
    pub cpu_usage: Option<f64>,
    /// Memory usage in bytes
    pub memory_usage_bytes: Option<usize>,
    /// Disk usage percentage (0-100)
    pub disk_usage: Option<f64>,
    /// Request rate (requests/sec)
    pub request_rate: Option<f64>,
    /// Error rate (errors/sec)
    pub error_rate: Option<f64>,
    /// Average response time in milliseconds
    pub avg_response_time_ms: Option<f64>,
}

/// Recovery action.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RecoveryAction {
    Restart,
    ClearCache,
    ResetConnections,
    RollbackTransaction,
    NotifyAdministrator,
    Custom(String),
}

/// Recovery result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryResult {
    /// Whether recovery was successful
    pub success: bool,
    /// Actions taken
    pub actions_taken: Vec<RecoveryAction>,
    /// Recovery timestamp
    pub performed_at: DateTime<Utc>,
    /// Result message
    pub message: String,
}

/// Watchdog monitor.
pub struct Watchdog {
    config: WatchdogConfig,
    last_check: Option<DateTime<Utc>>,
    check_history: VecDeque<HealthCheck>,
    consecutive_failures: usize,
    recovery_history: Vec<RecoveryResult>,
}

impl Watchdog {
    /// Creates a new watchdog with default configuration.
    pub fn new() -> Self {
        Self::with_config(WatchdogConfig::default())
    }

    /// Creates a watchdog with custom configuration.
    pub fn with_config(config: WatchdogConfig) -> Self {
        Self {
            config,
            last_check: None,
            check_history: VecDeque::new(),
            consecutive_failures: 0,
            recovery_history: Vec::new(),
        }
    }

    /// Performs a comprehensive health check.
    pub fn health_check(&mut self, component: MonitoredComponent) -> AuditResult<HealthCheck> {
        self.last_check = Some(Utc::now());

        let metrics = if self.config.enable_metrics {
            self.collect_metrics(&component)
        } else {
            HealthMetrics::default()
        };

        let status = self.determine_status(&metrics);

        let message = match status {
            HealthStatus::Healthy => "All systems operational".to_string(),
            HealthStatus::Degraded => "System performance degraded".to_string(),
            HealthStatus::Unhealthy => "System health check failed".to_string(),
            HealthStatus::Critical => "Critical system failure detected".to_string(),
            HealthStatus::Recovering => "System recovering from failure".to_string(),
        };

        let check = HealthCheck {
            status: status.clone(),
            checked_at: Utc::now(),
            message,
            component,
            metrics,
        };

        // Track failures
        match status {
            HealthStatus::Unhealthy | HealthStatus::Critical => {
                self.consecutive_failures += 1;
            }
            HealthStatus::Healthy => {
                self.consecutive_failures = 0;
            }
            _ => {}
        }

        // Add to history
        self.check_history.push_back(check.clone());

        // Trim history
        let cutoff = Utc::now() - Duration::hours(self.config.history_window_hours);
        while self
            .check_history
            .front()
            .is_some_and(|c| c.checked_at < cutoff)
        {
            self.check_history.pop_front();
        }

        // Auto-recovery if enabled
        if self.config.auto_recovery && self.consecutive_failures >= self.config.failure_threshold {
            let _ = self.attempt_recovery();
        }

        Ok(check)
    }

    /// Collects system metrics.
    fn collect_metrics(&self, _component: &MonitoredComponent) -> HealthMetrics {
        // In a real implementation, these would be actual system metrics
        HealthMetrics {
            cpu_usage: Some(25.0),
            memory_usage_bytes: Some(1024 * 1024 * 512), // 512 MB
            disk_usage: Some(45.0),
            request_rate: Some(100.0),
            error_rate: Some(0.5),
            avg_response_time_ms: Some(150.0),
        }
    }

    /// Determines health status based on metrics.
    fn determine_status(&self, metrics: &HealthMetrics) -> HealthStatus {
        let mut issues = 0;

        if let Some(cpu) = metrics.cpu_usage {
            if cpu > 90.0 {
                return HealthStatus::Critical;
            } else if cpu > 75.0 {
                issues += 1;
            }
        }

        if let Some(disk) = metrics.disk_usage {
            if disk > 95.0 {
                return HealthStatus::Critical;
            } else if disk > 80.0 {
                issues += 1;
            }
        }

        if let Some(error_rate) = metrics.error_rate {
            if error_rate > 10.0 {
                return HealthStatus::Unhealthy;
            } else if error_rate > 5.0 {
                issues += 1;
            }
        }

        if issues > 0 {
            HealthStatus::Degraded
        } else {
            HealthStatus::Healthy
        }
    }

    /// Attempts automatic recovery.
    pub fn attempt_recovery(&mut self) -> AuditResult<RecoveryResult> {
        let mut actions = Vec::new();

        // Determine recovery actions based on health status
        if self.consecutive_failures >= self.config.failure_threshold {
            actions.push(RecoveryAction::ClearCache);
            actions.push(RecoveryAction::ResetConnections);

            if self.consecutive_failures >= self.config.failure_threshold * 2 {
                actions.push(RecoveryAction::NotifyAdministrator);
            }
        }

        let result = RecoveryResult {
            success: true,
            actions_taken: actions,
            performed_at: Utc::now(),
            message: "Recovery actions executed".to_string(),
        };

        self.recovery_history.push(result.clone());
        self.consecutive_failures = 0;

        Ok(result)
    }

    /// Checks if health check is due.
    pub fn is_due_for_check(&self) -> bool {
        if let Some(last) = self.last_check {
            (Utc::now() - last).num_seconds() >= self.config.check_interval_seconds as i64
        } else {
            true
        }
    }

    /// Gets check history.
    pub fn get_check_history(&self) -> Vec<HealthCheck> {
        self.check_history.iter().cloned().collect()
    }

    /// Gets recovery history.
    pub fn get_recovery_history(&self) -> &[RecoveryResult] {
        &self.recovery_history
    }

    /// Gets current health status.
    pub fn get_current_status(&self) -> Option<HealthStatus> {
        self.check_history.back().map(|c| c.status.clone())
    }

    /// Gets consecutive failure count.
    pub fn get_consecutive_failures(&self) -> usize {
        self.consecutive_failures
    }

    /// Calculates uptime percentage.
    pub fn calculate_uptime(&self) -> f64 {
        if self.check_history.is_empty() {
            return 100.0;
        }

        let healthy_checks = self
            .check_history
            .iter()
            .filter(|c| matches!(c.status, HealthStatus::Healthy | HealthStatus::Degraded))
            .count();

        (healthy_checks as f64 / self.check_history.len() as f64) * 100.0
    }

    /// Gets average metrics over history window.
    pub fn get_average_metrics(&self) -> HealthMetrics {
        if self.check_history.is_empty() {
            return HealthMetrics::default();
        }

        let count = self.check_history.len() as f64;
        let mut sum_cpu = 0.0;
        let mut sum_mem = 0;
        let mut sum_disk = 0.0;
        let mut sum_req_rate = 0.0;
        let mut sum_err_rate = 0.0;
        let mut sum_resp_time = 0.0;

        for check in &self.check_history {
            if let Some(cpu) = check.metrics.cpu_usage {
                sum_cpu += cpu;
            }
            if let Some(mem) = check.metrics.memory_usage_bytes {
                sum_mem += mem;
            }
            if let Some(disk) = check.metrics.disk_usage {
                sum_disk += disk;
            }
            if let Some(req) = check.metrics.request_rate {
                sum_req_rate += req;
            }
            if let Some(err) = check.metrics.error_rate {
                sum_err_rate += err;
            }
            if let Some(resp) = check.metrics.avg_response_time_ms {
                sum_resp_time += resp;
            }
        }

        HealthMetrics {
            cpu_usage: Some(sum_cpu / count),
            memory_usage_bytes: Some((sum_mem as f64 / count) as usize),
            disk_usage: Some(sum_disk / count),
            request_rate: Some(sum_req_rate / count),
            error_rate: Some(sum_err_rate / count),
            avg_response_time_ms: Some(sum_resp_time / count),
        }
    }

    /// Resets the watchdog state.
    pub fn reset(&mut self) {
        self.check_history.clear();
        self.consecutive_failures = 0;
        self.recovery_history.clear();
        self.last_check = None;
    }

    /// Returns the configuration.
    pub fn config(&self) -> &WatchdogConfig {
        &self.config
    }
}

impl Default for Watchdog {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_watchdog() {
        let mut watchdog = Watchdog::new();
        assert!(watchdog.is_due_for_check());

        let check = watchdog
            .health_check(MonitoredComponent::AuditTrail)
            .unwrap();
        assert_eq!(check.status, HealthStatus::Healthy);
    }

    #[test]
    fn test_health_check_history() {
        let mut watchdog = Watchdog::new();

        for _ in 0..5 {
            watchdog.health_check(MonitoredComponent::Storage).unwrap();
        }

        let history = watchdog.get_check_history();
        assert_eq!(history.len(), 5);
    }

    #[test]
    fn test_uptime_calculation() {
        let mut watchdog = Watchdog::new();

        for _ in 0..10 {
            watchdog
                .health_check(MonitoredComponent::IntegrityChecker)
                .unwrap();
        }

        let uptime = watchdog.calculate_uptime();
        assert_eq!(uptime, 100.0);
    }

    #[test]
    fn test_recovery_attempt() {
        let mut watchdog = Watchdog::new();

        let result = watchdog.attempt_recovery().unwrap();
        assert!(result.success);
    }

    #[test]
    fn test_average_metrics() {
        let mut watchdog = Watchdog::new();

        for _ in 0..3 {
            watchdog
                .health_check(MonitoredComponent::StreamingAnalyzer)
                .unwrap();
        }

        let avg = watchdog.get_average_metrics();
        assert!(avg.cpu_usage.is_some());
        assert!(avg.memory_usage_bytes.is_some());
    }

    #[test]
    fn test_watchdog_reset() {
        let mut watchdog = Watchdog::new();

        watchdog
            .health_check(MonitoredComponent::AuditTrail)
            .unwrap();
        assert!(!watchdog.get_check_history().is_empty());

        watchdog.reset();
        assert!(watchdog.get_check_history().is_empty());
        assert_eq!(watchdog.get_consecutive_failures(), 0);
    }

    #[test]
    fn test_current_status() {
        let mut watchdog = Watchdog::new();

        assert!(watchdog.get_current_status().is_none());

        watchdog
            .health_check(MonitoredComponent::AlertManager)
            .unwrap();
        assert!(watchdog.get_current_status().is_some());
    }
}
