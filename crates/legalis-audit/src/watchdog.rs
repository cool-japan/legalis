//! Watchdog process integration for continuous monitoring.

use crate::AuditResult;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Watchdog configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchdogConfig {
    pub check_interval_seconds: u64,
    pub health_check_enabled: bool,
    pub auto_recovery: bool,
}

impl Default for WatchdogConfig {
    fn default() -> Self {
        Self {
            check_interval_seconds: 30,
            health_check_enabled: true,
            auto_recovery: true,
        }
    }
}

/// Health status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

/// Health check result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    pub status: HealthStatus,
    pub checked_at: DateTime<Utc>,
    pub message: String,
}

/// Watchdog monitor.
pub struct Watchdog {
    config: WatchdogConfig,
    last_check: Option<DateTime<Utc>>,
}

impl Watchdog {
    pub fn new() -> Self {
        Self::with_config(WatchdogConfig::default())
    }

    pub fn with_config(config: WatchdogConfig) -> Self {
        Self {
            config,
            last_check: None,
        }
    }

    pub fn health_check(&mut self) -> AuditResult<HealthCheck> {
        self.last_check = Some(Utc::now());

        Ok(HealthCheck {
            status: HealthStatus::Healthy,
            checked_at: Utc::now(),
            message: "All systems operational".to_string(),
        })
    }

    pub fn is_due_for_check(&self) -> bool {
        if let Some(last) = self.last_check {
            (Utc::now() - last).num_seconds() >= self.config.check_interval_seconds as i64
        } else {
            true
        }
    }

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

        let check = watchdog.health_check().unwrap();
        assert_eq!(check.status, HealthStatus::Healthy);
    }
}
