//! The production-monitor orchestrator and its exportable snapshot.
//!
//! [`ProductionMonitor`] is the single entry point that ingests
//! [`ResponseObservation`]s, [`HealthProbe`]s and [`FeedbackSignal`]s and ties
//! together every other analytic in this module into one
//! [`MonitoringSnapshot`]. The snapshot reuses the crate's existing
//! [`crate::AggregatedMetrics`] for the latency/throughput rollup and renders to
//! the crate's existing [`crate::dashboard::Dashboard`] widget model.
//!
//! ## Live-transport boundary
//!
//! A snapshot is a *point-in-time* export. Building it is fully implemented and
//! offline; streaming successive snapshots to a browser dashboard over a
//! websocket / SSE / HTTP endpoint is an external transport concern. The
//! "real-time" loop is therefore: take a snapshot, serialise it with
//! [`ProductionMonitor::export_snapshot_json`], hand it to your transport, repeat.

use super::{
    AnomalyConfig, CostPerQueryStats, ErrorRateReport, FeedbackSignal, HealthProbe, QaEvaluator,
    QaReport, ResponseAnomaly, ResponseAnomalyDetector, ResponseObservation, SatisfactionStats,
    TrendGranularity, UptimeStats, cost_per_query_stats, cost_trend, error_rate_report, percentile,
    satisfaction_stats, uptime_stats,
};
use crate::dashboard::{Dashboard, WidgetType};
use crate::{AggregatedMetrics, AnomalySeverity};
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// A point-in-time, exportable snapshot of the whole monitoring surface.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringSnapshot {
    /// When the snapshot was generated.
    pub generated_at: DateTime<Utc>,
    /// Total observed requests.
    pub total_requests: usize,
    /// Latency / throughput rollup (reuses [`crate::AggregatedMetrics`]).
    pub performance: AggregatedMetrics,
    /// Cost-per-query statistics.
    pub cost_per_query: CostPerQueryStats,
    /// Error-rate report by category.
    pub errors: ErrorRateReport,
    /// Quality-assurance report.
    pub quality: QaReport,
    /// Detected response anomalies.
    pub anomalies: Vec<ResponseAnomaly>,
    /// Count of high-severity anomalies.
    pub anomalies_high: usize,
    /// Count of medium-severity anomalies.
    pub anomalies_medium: usize,
    /// Count of low-severity anomalies.
    pub anomalies_low: usize,
    /// Provider uptime statistics, keyed by provider.
    pub uptime: BTreeMap<String, UptimeStats>,
    /// User-satisfaction statistics.
    pub satisfaction: SatisfactionStats,
    /// Feedback rate (signals per request) in `[0, 1]`.
    pub feedback_rate: f64,
}

impl MonitoringSnapshot {
    /// Returns whether the snapshot indicates a healthy system.
    ///
    /// "Healthy" means: a high success rate, no high-severity anomalies and every
    /// monitored provider above a `99%` uptime floor.
    pub fn is_healthy(&self) -> bool {
        self.performance.success_rate >= 0.99
            && self.anomalies_high == 0
            && self.uptime.values().all(|stats| stats.uptime_pct >= 99.0)
    }
}

/// Builds an [`AggregatedMetrics`] rollup from raw observations.
///
/// Reuses the crate's metrics type rather than defining a parallel one; the
/// percentiles follow the same nearest-rank convention as the observability
/// metrics collector.
fn performance_summary(observations: &[ResponseObservation]) -> AggregatedMetrics {
    if observations.is_empty() {
        return AggregatedMetrics::default();
    }
    let total_requests = observations.len() as u64;
    let successful_requests = observations.iter().filter(|obs| obs.is_success()).count() as u64;
    let failed_requests = total_requests - successful_requests;
    let success_rate = successful_requests as f64 / total_requests as f64;
    let total_tokens: u64 = observations
        .iter()
        .map(|obs| obs.total_tokens() as u64)
        .sum();
    let total_cost_usd: f64 = observations.iter().map(|obs| obs.cost_or_zero()).sum();
    let avg_duration_ms = observations
        .iter()
        .map(|obs| obs.latency_ms as f64)
        .sum::<f64>()
        / total_requests as f64;

    let mut durations: Vec<u128> = observations
        .iter()
        .map(|obs| obs.latency_ms as u128)
        .collect();
    durations.sort_unstable();
    let pick = |p: f64| -> u128 {
        let idx = (durations.len() as f64 * p) as usize;
        durations.get(idx).copied().unwrap_or(0)
    };

    AggregatedMetrics {
        total_requests,
        successful_requests,
        failed_requests,
        success_rate,
        total_tokens,
        total_cost_usd,
        avg_duration_ms,
        p50_latency_ms: pick(0.50),
        p95_latency_ms: pick(0.95),
        p99_latency_ms: pick(0.99),
    }
}

/// The central production-monitoring orchestrator.
///
/// Ingest observations / probes / feedback as they happen, then call
/// [`ProductionMonitor::snapshot`] (or [`ProductionMonitor::dashboard`]) to
/// produce a coherent view across all analytics.
pub struct ProductionMonitor {
    observations: Arc<RwLock<Vec<ResponseObservation>>>,
    health_probes: Arc<RwLock<Vec<HealthProbe>>>,
    feedback: Arc<RwLock<Vec<FeedbackSignal>>>,
    qa_evaluator: QaEvaluator,
    anomaly_detector: ResponseAnomalyDetector,
    max_history: usize,
}

impl ProductionMonitor {
    /// Creates a monitor with sensible defaults (legal QA suite, robust anomaly
    /// detection, and a `100_000`-observation ring buffer).
    pub fn new() -> Self {
        Self {
            observations: Arc::new(RwLock::new(Vec::new())),
            health_probes: Arc::new(RwLock::new(Vec::new())),
            feedback: Arc::new(RwLock::new(Vec::new())),
            qa_evaluator: QaEvaluator::legal_default(),
            anomaly_detector: ResponseAnomalyDetector::new(),
            max_history: 100_000,
        }
    }

    /// Replaces the QA evaluator.
    pub fn with_qa_evaluator(mut self, evaluator: QaEvaluator) -> Self {
        self.qa_evaluator = evaluator;
        self
    }

    /// Replaces the anomaly-detection configuration.
    pub fn with_anomaly_config(mut self, config: AnomalyConfig) -> Self {
        self.anomaly_detector = ResponseAnomalyDetector::with_config(config);
        self
    }

    /// Sets the maximum number of observations retained.
    pub fn with_max_history(mut self, max_history: usize) -> Self {
        self.max_history = max_history.max(1);
        self
    }

    /// Records a response observation, trimming history to the configured cap.
    pub async fn record(&self, observation: ResponseObservation) {
        let mut observations = self.observations.write().await;
        observations.push(observation);
        if observations.len() > self.max_history {
            let excess = observations.len() - self.max_history;
            observations.drain(0..excess);
        }
    }

    /// Records a provider health probe.
    pub async fn record_health(&self, probe: HealthProbe) {
        self.health_probes.write().await.push(probe);
    }

    /// Records a user feedback signal.
    pub async fn record_feedback(&self, signal: FeedbackSignal) {
        self.feedback.write().await.push(signal);
    }

    /// Returns the number of stored observations.
    pub async fn observation_count(&self) -> usize {
        self.observations.read().await.len()
    }

    /// Returns a clone of all stored observations.
    pub async fn observations(&self) -> Vec<ResponseObservation> {
        self.observations.read().await.clone()
    }

    /// Computes a full monitoring snapshot.
    pub async fn snapshot(&self) -> MonitoringSnapshot {
        let observations = self.observations.read().await.clone();
        let probes = self.health_probes.read().await.clone();
        let feedback = self.feedback.read().await.clone();

        let performance = performance_summary(&observations);
        let cost_per_query = cost_per_query_stats(&observations);
        let errors = error_rate_report(&observations);
        let quality = self.qa_evaluator.evaluate(&observations);
        let anomalies = self.anomaly_detector.detect(&observations);

        let mut anomalies_high = 0;
        let mut anomalies_medium = 0;
        let mut anomalies_low = 0;
        for anomaly in &anomalies {
            match anomaly.severity {
                AnomalySeverity::High => anomalies_high += 1,
                AnomalySeverity::Medium => anomalies_medium += 1,
                AnomalySeverity::Low => anomalies_low += 1,
            }
        }

        let mut providers: Vec<String> = probes.iter().map(|p| p.provider.clone()).collect();
        providers.sort();
        providers.dedup();
        let uptime: BTreeMap<String, UptimeStats> = providers
            .into_iter()
            .map(|provider| {
                let stats = uptime_stats(&provider, &probes);
                (provider, stats)
            })
            .collect();

        let satisfaction = satisfaction_stats(&feedback);
        let feedback_rate = satisfaction.feedback_rate(observations.len());

        MonitoringSnapshot {
            generated_at: Utc::now(),
            total_requests: observations.len(),
            performance,
            cost_per_query,
            errors,
            quality,
            anomalies,
            anomalies_high,
            anomalies_medium,
            anomalies_low,
            uptime,
            satisfaction,
            feedback_rate,
        }
    }

    /// Renders a snapshot to the crate's existing dashboard widget model.
    pub async fn dashboard(&self, title: impl Into<String>) -> Dashboard {
        let snapshot = self.snapshot().await;
        Self::render_dashboard(title.into(), &snapshot, &self.observations().await)
    }

    /// Builds the dashboard widgets from a snapshot and the raw observations
    /// (the latter only used for the cost time-series).
    fn render_dashboard(
        title: String,
        snapshot: &MonitoringSnapshot,
        observations: &[ResponseObservation],
    ) -> Dashboard {
        let mut dashboard = Dashboard::new(title)
            .with_description("Production monitoring overview")
            .with_refresh_interval(30);

        dashboard = dashboard.add_widget(WidgetType::Counter {
            title: "Total Requests".to_string(),
            value: snapshot.total_requests as f64,
            unit: Some("requests".to_string()),
        });
        dashboard = dashboard.add_widget(WidgetType::Counter {
            title: "Total Errors".to_string(),
            value: snapshot.errors.total_errors as f64,
            unit: Some("errors".to_string()),
        });
        dashboard = dashboard.add_widget(WidgetType::Counter {
            title: "Total Cost".to_string(),
            value: snapshot.cost_per_query.total_cost_usd,
            unit: Some("USD".to_string()),
        });
        dashboard = dashboard.add_widget(WidgetType::Gauge {
            title: "Success Rate".to_string(),
            value: snapshot.performance.success_rate * 100.0,
            min: 0.0,
            max: 100.0,
            unit: Some("%".to_string()),
        });
        dashboard = dashboard.add_widget(WidgetType::Gauge {
            title: "QA Pass Rate".to_string(),
            value: snapshot.quality.overall_pass_rate() * 100.0,
            min: 0.0,
            max: 100.0,
            unit: Some("%".to_string()),
        });
        dashboard = dashboard.add_widget(WidgetType::Gauge {
            title: "CSAT".to_string(),
            value: snapshot.satisfaction.csat_pct,
            min: 0.0,
            max: 100.0,
            unit: Some("%".to_string()),
        });
        dashboard = dashboard.add_widget(WidgetType::Counter {
            title: "p95 Latency".to_string(),
            value: snapshot.performance.p95_latency_ms as f64,
            unit: Some("ms".to_string()),
        });
        dashboard = dashboard.add_widget(WidgetType::Counter {
            title: "Mean Cost / Query".to_string(),
            value: snapshot.cost_per_query.mean_cost,
            unit: Some("USD".to_string()),
        });

        // Errors by category.
        let mut error_bars: Vec<(String, f64)> = snapshot
            .errors
            .errors_by_category
            .iter()
            .map(|(category, count)| (category.label().to_string(), *count as f64))
            .collect();
        error_bars.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        if !error_bars.is_empty() {
            dashboard = dashboard.add_widget(WidgetType::BarChart {
                title: "Errors by Category".to_string(),
                data: error_bars,
            });
        }

        // Cost by category.
        let cost_bars: Vec<(String, f64)> = snapshot
            .cost_per_query
            .cost_by_category
            .iter()
            .map(|(category, cost)| (category.clone(), *cost))
            .collect();
        if !cost_bars.is_empty() {
            dashboard = dashboard.add_widget(WidgetType::BarChart {
                title: "Cost by Category".to_string(),
                data: cost_bars,
            });
        }

        // Uptime table.
        if !snapshot.uptime.is_empty() {
            let rows: Vec<Vec<String>> = snapshot
                .uptime
                .values()
                .map(|stats| {
                    vec![
                        stats.provider.clone(),
                        format!("{:.2}%", stats.uptime_pct),
                        format!("{:?}", stats.current_status),
                        stats.incidents.len().to_string(),
                    ]
                })
                .collect();
            dashboard = dashboard.add_widget(WidgetType::Table {
                title: "Provider Uptime".to_string(),
                headers: vec![
                    "Provider".to_string(),
                    "Uptime".to_string(),
                    "Status".to_string(),
                    "Incidents".to_string(),
                ],
                rows,
            });
        }

        // Anomaly summary table.
        dashboard = dashboard.add_widget(WidgetType::Table {
            title: "Anomalies".to_string(),
            headers: vec!["Severity".to_string(), "Count".to_string()],
            rows: vec![
                vec!["High".to_string(), snapshot.anomalies_high.to_string()],
                vec!["Medium".to_string(), snapshot.anomalies_medium.to_string()],
                vec!["Low".to_string(), snapshot.anomalies_low.to_string()],
            ],
        });

        // Cost trend time-series (daily).
        let trend = cost_trend(observations, TrendGranularity::Daily);
        if !trend.is_empty() {
            let data: Vec<(DateTime<Utc>, f64)> = trend
                .iter()
                .map(|point| (point.bucket_start, point.total_cost))
                .collect();
            dashboard = dashboard.add_widget(WidgetType::TimeSeries {
                title: "Daily Cost".to_string(),
                data,
                label: "USD".to_string(),
            });
        }

        dashboard
    }

    /// Exports the current snapshot as pretty-printed JSON.
    pub async fn export_snapshot_json(&self) -> Result<String> {
        let snapshot = self.snapshot().await;
        Ok(serde_json::to_string_pretty(&snapshot)?)
    }

    /// Returns the p-th latency percentile (ms) across stored observations.
    pub async fn latency_percentile(&self, p: f64) -> f64 {
        let observations = self.observations.read().await;
        let latencies: Vec<f64> = observations
            .iter()
            .map(|obs| obs.latency_ms as f64)
            .collect();
        percentile(&latencies, p)
    }
}

impl Default for ProductionMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TokenUsage;
    use crate::human_feedback::Rating;
    use crate::monitoring::ErrorCategory;

    fn good(latency: u64) -> ResponseObservation {
        ResponseObservation::new("openai", "gpt-4")
            .with_category("analysis")
            .with_latency(latency)
            .with_cost(0.02)
            .with_usage(TokenUsage::new(500, 200))
            .with_response("This is a complete and adequately long legal analysis sentence.")
    }

    #[tokio::test]
    async fn test_snapshot_basic() {
        let monitor = ProductionMonitor::new();
        for index in 0..20 {
            monitor.record(good(100 + index)).await;
        }
        monitor
            .record(good(100).failed("503 service unavailable"))
            .await;
        monitor.record_health(HealthProbe::up("openai", 50)).await;
        monitor
            .record_feedback(FeedbackSignal::new("x").with_rating(Rating::Good))
            .await;

        let snapshot = monitor.snapshot().await;
        assert_eq!(snapshot.total_requests, 21);
        assert_eq!(snapshot.performance.total_requests, 21);
        assert_eq!(snapshot.errors.total_errors, 1);
        assert!(snapshot.cost_per_query.total_cost_usd > 0.0);
        assert!(snapshot.uptime.contains_key("openai"));
        assert_eq!(snapshot.satisfaction.total_signals, 1);
        assert!(snapshot.feedback_rate > 0.0);
    }

    #[tokio::test]
    async fn test_snapshot_health_assessment() {
        let monitor = ProductionMonitor::new();
        for _ in 0..50 {
            monitor.record(good(100)).await;
        }
        monitor.record_health(HealthProbe::up("openai", 50)).await;
        let snapshot = monitor.snapshot().await;
        assert!(snapshot.is_healthy());

        // Inject many failures - the system should no longer be healthy.
        for _ in 0..50 {
            monitor
                .record(good(100).with_error(ErrorCategory::ServiceUnavailable))
                .await;
        }
        let snapshot2 = monitor.snapshot().await;
        assert!(!snapshot2.is_healthy());
    }

    #[tokio::test]
    async fn test_dashboard_rendering() {
        let monitor = ProductionMonitor::new();
        for index in 0..15 {
            monitor.record(good(100 + index)).await;
        }
        monitor
            .record(good(100).failed("rate limit exceeded"))
            .await;
        monitor.record_health(HealthProbe::up("openai", 40)).await;

        let dashboard = monitor.dashboard("Legal LLM Monitor").await;
        assert_eq!(dashboard.title, "Legal LLM Monitor");
        assert!(dashboard.refresh_interval.is_some());
        // Counters + gauges + tables are always present.
        assert!(dashboard.widgets.len() >= 8);
        let has_success_gauge = dashboard
            .widgets
            .iter()
            .any(|w| matches!(w, WidgetType::Gauge { title, .. } if title == "Success Rate"));
        assert!(has_success_gauge);
    }

    #[tokio::test]
    async fn test_export_json_roundtrip() {
        let monitor = ProductionMonitor::new();
        monitor.record(good(120)).await;
        let json = monitor.export_snapshot_json().await.expect("serialises");
        assert!(json.contains("total_requests"));
        let parsed: MonitoringSnapshot = serde_json::from_str(&json).expect("deserialises");
        assert_eq!(parsed.total_requests, 1);
    }

    #[tokio::test]
    async fn test_max_history_ring_buffer() {
        let monitor = ProductionMonitor::new().with_max_history(10);
        for index in 0..25 {
            monitor.record(good(100 + index)).await;
        }
        assert_eq!(monitor.observation_count().await, 10);
    }

    #[tokio::test]
    async fn test_latency_percentile() {
        let monitor = ProductionMonitor::new();
        for index in 1..=100 {
            monitor.record(good(index * 10)).await;
        }
        let p95 = monitor.latency_percentile(95.0).await;
        assert!(p95 >= 900.0);
    }
}
