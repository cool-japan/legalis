//! Service Level Objective (SLO) and Service Level Indicator (SLI) tracking.
//!
//! This module provides comprehensive SLO/SLI tracking to monitor service
//! quality and ensure commitments are met.

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::RwLock;

/// SLI (Service Level Indicator) metric type
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum SliType {
    /// Availability (uptime)
    Availability,
    /// Latency (response time)
    Latency,
    /// Error rate
    ErrorRate,
    /// Throughput (requests per second)
    Throughput,
    /// Quality (successful operations)
    Quality,
}

/// Time window for SLO measurement
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TimeWindow {
    /// Last hour
    Hour,
    /// Last day
    Day,
    /// Last week
    Week,
    /// Last 30 days
    Month,
}

impl TimeWindow {
    /// Converts to a Duration
    pub fn to_duration(&self) -> Duration {
        match self {
            TimeWindow::Hour => Duration::hours(1),
            TimeWindow::Day => Duration::days(1),
            TimeWindow::Week => Duration::weeks(1),
            TimeWindow::Month => Duration::days(30),
        }
    }
}

/// SLO (Service Level Objective) definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Slo {
    /// Unique identifier
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Description
    pub description: String,
    /// SLI type being tracked
    pub sli_type: SliType,
    /// Target value (percentage for most metrics)
    pub target: f64,
    /// Time window for measurement
    pub window: TimeWindow,
    /// Service or endpoint this SLO applies to
    pub service: String,
    /// Whether this SLO is enabled
    pub enabled: bool,
}

impl Slo {
    /// Creates a new SLO
    pub fn new(
        id: String,
        name: String,
        description: String,
        sli_type: SliType,
        target: f64,
        window: TimeWindow,
        service: String,
    ) -> Self {
        Self {
            id,
            name,
            description,
            sli_type,
            target,
            window,
            service,
            enabled: true,
        }
    }

    /// Creates an availability SLO (e.g., 99.9% uptime)
    pub fn availability(service: String, target: f64, window: TimeWindow) -> Self {
        Self::new(
            format!("availability-{}", service),
            format!("{} Availability", service),
            format!("{}% availability over {}", target, window_name(&window)),
            SliType::Availability,
            target,
            window,
            service,
        )
    }

    /// Creates a latency SLO (e.g., 95% of requests < 200ms)
    pub fn latency(service: String, target_ms: f64, percentile: f64, window: TimeWindow) -> Self {
        Self::new(
            format!("latency-{}", service),
            format!("{} Latency", service),
            format!(
                "P{} latency < {}ms over {}",
                percentile * 100.0,
                target_ms,
                window_name(&window)
            ),
            SliType::Latency,
            target_ms,
            window,
            service,
        )
    }

    /// Creates an error rate SLO (e.g., < 0.1% error rate)
    pub fn error_rate(service: String, max_rate: f64, window: TimeWindow) -> Self {
        Self::new(
            format!("error-rate-{}", service),
            format!("{} Error Rate", service),
            format!("Error rate < {}% over {}", max_rate, window_name(&window)),
            SliType::ErrorRate,
            max_rate,
            window,
            service,
        )
    }
}

fn window_name(window: &TimeWindow) -> &'static str {
    match window {
        TimeWindow::Hour => "last hour",
        TimeWindow::Day => "last day",
        TimeWindow::Week => "last week",
        TimeWindow::Month => "last 30 days",
    }
}

/// SLI measurement data point
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SliDataPoint {
    timestamp: DateTime<Utc>,
    value: f64,
    success: bool,
}

/// SLO status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SloStatus {
    /// SLO is being met
    Met,
    /// SLO is at risk (close to threshold)
    AtRisk,
    /// SLO is violated
    Violated,
}

/// SLO report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SloReport {
    /// SLO being reported on
    pub slo: Slo,
    /// Current measured value
    pub current_value: f64,
    /// Target value
    pub target_value: f64,
    /// Current status
    pub status: SloStatus,
    /// Error budget remaining (percentage)
    pub error_budget_remaining: f64,
    /// Number of data points in window
    pub data_points: usize,
    /// Report timestamp
    pub timestamp: DateTime<Utc>,
}

/// Error budget for an SLO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorBudget {
    /// SLO ID
    pub slo_id: String,
    /// Total budget (percentage of failures allowed)
    pub total_budget: f64,
    /// Budget consumed so far
    pub consumed: f64,
    /// Budget remaining
    pub remaining: f64,
    /// Budget burn rate (per hour)
    pub burn_rate: f64,
}

/// SLO tracker
#[derive(Clone)]
pub struct SloTracker {
    /// Defined SLOs
    slos: Arc<RwLock<HashMap<String, Slo>>>,
    /// SLI data points
    data: Arc<RwLock<HashMap<String, VecDeque<SliDataPoint>>>>,
    /// Maximum data points to keep per SLO
    max_data_points: usize,
}

impl SloTracker {
    /// Creates a new SLO tracker
    pub fn new() -> Self {
        Self {
            slos: Arc::new(RwLock::new(HashMap::new())),
            data: Arc::new(RwLock::new(HashMap::new())),
            max_data_points: 10000,
        }
    }

    /// Registers an SLO
    pub async fn register_slo(&self, slo: Slo) {
        let mut slos = self.slos.write().await;
        slos.insert(slo.id.clone(), slo);
    }

    /// Records an SLI measurement
    pub async fn record_sli(&self, slo_id: String, value: f64, success: bool) {
        let mut data = self.data.write().await;
        let points = data.entry(slo_id).or_insert_with(VecDeque::new);

        points.push_back(SliDataPoint {
            timestamp: Utc::now(),
            value,
            success,
        });

        // Limit data points
        while points.len() > self.max_data_points {
            points.pop_front();
        }
    }

    /// Gets SLO report
    pub async fn get_report(&self, slo_id: &str) -> Option<SloReport> {
        let slos = self.slos.read().await;
        let slo = slos.get(slo_id)?.clone();
        drop(slos);

        let data = self.data.read().await;
        let points = data.get(slo_id)?;

        // Filter points within the time window
        let cutoff = Utc::now() - slo.window.to_duration();
        let window_points: Vec<_> = points.iter().filter(|p| p.timestamp >= cutoff).collect();

        if window_points.is_empty() {
            return None;
        }

        let current_value = match slo.sli_type {
            SliType::Availability | SliType::Quality => {
                let successful = window_points.iter().filter(|p| p.success).count();
                (successful as f64 / window_points.len() as f64) * 100.0
            }
            SliType::ErrorRate => {
                let errors = window_points.iter().filter(|p| !p.success).count();
                (errors as f64 / window_points.len() as f64) * 100.0
            }
            SliType::Latency => {
                // Calculate percentile (simplified: using mean)
                let sum: f64 = window_points.iter().map(|p| p.value).sum();
                sum / window_points.len() as f64
            }
            SliType::Throughput => {
                // Requests per second
                let duration = slo.window.to_duration().num_seconds() as f64;
                window_points.len() as f64 / duration
            }
        };

        let status = self.calculate_status(&slo, current_value);
        let error_budget = self.calculate_error_budget(&slo, current_value);

        Some(SloReport {
            target_value: slo.target,
            slo,
            current_value,
            status,
            error_budget_remaining: error_budget,
            data_points: window_points.len(),
            timestamp: Utc::now(),
        })
    }

    /// Calculates SLO status
    fn calculate_status(&self, slo: &Slo, current_value: f64) -> SloStatus {
        match slo.sli_type {
            SliType::Availability | SliType::Quality => {
                if current_value >= slo.target {
                    SloStatus::Met
                } else if current_value >= slo.target * 0.95 {
                    SloStatus::AtRisk
                } else {
                    SloStatus::Violated
                }
            }
            SliType::ErrorRate => {
                if current_value <= slo.target {
                    SloStatus::Met
                } else if current_value <= slo.target * 1.5 {
                    SloStatus::AtRisk
                } else {
                    SloStatus::Violated
                }
            }
            SliType::Latency => {
                if current_value <= slo.target {
                    SloStatus::Met
                } else if current_value <= slo.target * 1.2 {
                    SloStatus::AtRisk
                } else {
                    SloStatus::Violated
                }
            }
            SliType::Throughput => {
                if current_value >= slo.target {
                    SloStatus::Met
                } else if current_value >= slo.target * 0.8 {
                    SloStatus::AtRisk
                } else {
                    SloStatus::Violated
                }
            }
        }
    }

    /// Calculates error budget remaining
    fn calculate_error_budget(&self, slo: &Slo, current_value: f64) -> f64 {
        match slo.sli_type {
            SliType::Availability | SliType::Quality => {
                let budget = 100.0 - slo.target;
                let consumed = 100.0 - current_value;
                ((budget - consumed) / budget * 100.0).max(0.0)
            }
            SliType::ErrorRate => {
                let consumed = current_value;
                let budget = slo.target;
                ((budget - consumed) / budget * 100.0).max(0.0)
            }
            _ => 100.0, // Simplified for other types
        }
    }

    /// Gets all SLO reports
    pub async fn get_all_reports(&self) -> Vec<SloReport> {
        let slos = self.slos.read().await;
        let mut reports = Vec::new();

        for slo_id in slos.keys() {
            if let Some(report) = self.get_report(slo_id).await {
                reports.push(report);
            }
        }

        reports
    }

    /// Gets SLOs by status
    pub async fn get_slos_by_status(&self, status: SloStatus) -> Vec<SloReport> {
        let all_reports = self.get_all_reports().await;
        all_reports
            .into_iter()
            .filter(|r| r.status == status)
            .collect()
    }

    /// Gets error budget for an SLO
    pub async fn get_error_budget(&self, slo_id: &str) -> Option<ErrorBudget> {
        let report = self.get_report(slo_id).await?;

        Some(ErrorBudget {
            slo_id: slo_id.to_string(),
            total_budget: 100.0 - report.slo.target,
            consumed: 100.0 - report.current_value,
            remaining: report.error_budget_remaining,
            burn_rate: 0.0, // Simplified - would need historical data
        })
    }
}

impl Default for SloTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slo_creation() {
        let slo = Slo::availability("api".to_string(), 99.9, TimeWindow::Day);
        assert_eq!(slo.sli_type, SliType::Availability);
        assert_eq!(slo.target, 99.9);
        assert_eq!(slo.window, TimeWindow::Day);
    }

    #[tokio::test]
    async fn test_slo_tracker_register() {
        let tracker = SloTracker::new();
        let slo = Slo::availability("api".to_string(), 99.9, TimeWindow::Day);

        tracker.register_slo(slo.clone()).await;

        let report = tracker.get_report(&slo.id).await;
        assert!(report.is_none()); // No data yet
    }

    #[tokio::test]
    async fn test_slo_tracker_record() {
        let tracker = SloTracker::new();
        let slo = Slo::availability("api".to_string(), 99.9, TimeWindow::Hour);

        tracker.register_slo(slo.clone()).await;

        // Record successful requests
        for _ in 0..999 {
            tracker.record_sli(slo.id.clone(), 1.0, true).await;
        }
        // Record 1 failure
        tracker.record_sli(slo.id.clone(), 0.0, false).await;

        let report = tracker.get_report(&slo.id).await.unwrap();
        assert_eq!(report.status, SloStatus::Met);
        assert!((report.current_value - 99.9).abs() < 0.1);
    }

    #[tokio::test]
    async fn test_slo_status_calculation() {
        let tracker = SloTracker::new();
        let slo = Slo::availability("api".to_string(), 99.0, TimeWindow::Hour);

        tracker.register_slo(slo.clone()).await;

        // Record mostly successful requests
        for _ in 0..99 {
            tracker.record_sli(slo.id.clone(), 1.0, true).await;
        }
        tracker.record_sli(slo.id.clone(), 0.0, false).await;

        let report = tracker.get_report(&slo.id).await.unwrap();
        assert_eq!(report.status, SloStatus::Met);
    }

    #[tokio::test]
    async fn test_error_budget() {
        let tracker = SloTracker::new();
        let slo = Slo::availability("api".to_string(), 99.0, TimeWindow::Hour);

        tracker.register_slo(slo.clone()).await;

        // Record mostly successful requests (99.5% success rate)
        for _ in 0..199 {
            tracker.record_sli(slo.id.clone(), 1.0, true).await;
        }
        tracker.record_sli(slo.id.clone(), 0.0, false).await;

        let budget = tracker.get_error_budget(&slo.id).await.unwrap();
        assert!(budget.total_budget > 0.0);
        assert!(budget.consumed >= 0.0);
    }
}
