//! Anomaly detection for API usage patterns.
//!
//! This module provides real-time anomaly detection to identify unusual
//! patterns in API usage that might indicate security issues or bugs.

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Type of anomaly detected
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AnomalyType {
    /// Sudden spike in request rate
    RateSpike,
    /// Unusual error rate
    ErrorRateAnomaly,
    /// Slow response times
    LatencyAnomaly,
    /// Unusual request pattern
    PatternAnomaly,
    /// Suspicious user behavior
    SuspiciousActivity,
    /// Unusual geographic access
    GeoAnomaly,
}

/// Severity level of an anomaly
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum AnomalySeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Detected anomaly
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anomaly {
    /// Type of anomaly
    pub anomaly_type: AnomalyType,
    /// Severity level
    pub severity: AnomalySeverity,
    /// Description of the anomaly
    pub description: String,
    /// Affected endpoint or resource
    pub resource: String,
    /// Metric value that triggered the anomaly
    pub value: f64,
    /// Expected baseline value
    pub baseline: f64,
    /// Standard deviation from baseline
    pub std_deviations: f64,
    /// Timestamp when detected
    pub detected_at: DateTime<Utc>,
    /// Additional metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// Time series data point
#[derive(Debug, Clone)]
struct DataPoint {
    timestamp: DateTime<Utc>,
    value: f64,
}

/// Time series for anomaly detection
#[derive(Debug, Clone)]
struct TimeSeries {
    /// Data points
    points: VecDeque<DataPoint>,
    /// Maximum number of points to keep
    max_points: usize,
    /// Window duration
    window: Duration,
}

impl TimeSeries {
    fn new(max_points: usize, window: Duration) -> Self {
        Self {
            points: VecDeque::new(),
            max_points,
            window,
        }
    }

    fn add(&mut self, value: f64) {
        let now = Utc::now();

        // Add new point
        self.points.push_back(DataPoint {
            timestamp: now,
            value,
        });

        // Remove old points outside the window
        let cutoff = now - self.window;
        while let Some(point) = self.points.front() {
            if point.timestamp < cutoff {
                self.points.pop_front();
            } else {
                break;
            }
        }

        // Limit number of points
        while self.points.len() > self.max_points {
            self.points.pop_front();
        }
    }

    fn mean(&self) -> f64 {
        if self.points.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.points.iter().map(|p| p.value).sum();
        sum / self.points.len() as f64
    }

    fn std_dev(&self) -> f64 {
        if self.points.len() < 2 {
            return 0.0;
        }

        let mean = self.mean();
        let variance: f64 = self
            .points
            .iter()
            .map(|p| (p.value - mean).powi(2))
            .sum::<f64>()
            / (self.points.len() - 1) as f64;

        variance.sqrt()
    }

    fn is_anomaly(&self, value: f64, threshold: f64) -> bool {
        let mean = self.mean();
        let std_dev = self.std_dev();

        if std_dev == 0.0 {
            return false;
        }

        let z_score = (value - mean).abs() / std_dev;
        z_score > threshold
    }

    fn z_score(&self, value: f64) -> f64 {
        let mean = self.mean();
        let std_dev = self.std_dev();

        if std_dev == 0.0 {
            return 0.0;
        }

        (value - mean).abs() / std_dev
    }
}

/// Anomaly detection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyConfig {
    /// Enable anomaly detection
    pub enabled: bool,
    /// Z-score threshold for anomaly detection (default: 3.0)
    pub threshold: f64,
    /// Window size in seconds for baseline calculation
    pub window_seconds: i64,
    /// Maximum data points to keep
    pub max_data_points: usize,
    /// Minimum observations before detecting anomalies
    pub min_observations: usize,
}

impl Default for AnomalyConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            threshold: 3.0,       // 3 standard deviations
            window_seconds: 3600, // 1 hour
            max_data_points: 1000,
            min_observations: 30,
        }
    }
}

/// Anomaly detector
#[derive(Clone)]
pub struct AnomalyDetector {
    /// Configuration
    config: Arc<RwLock<AnomalyConfig>>,
    /// Time series data per metric
    metrics: Arc<RwLock<HashMap<String, TimeSeries>>>,
    /// Detected anomalies
    anomalies: Arc<RwLock<VecDeque<Anomaly>>>,
    /// Maximum anomalies to keep in history
    max_anomalies: usize,
}

impl AnomalyDetector {
    /// Creates a new anomaly detector
    pub fn new() -> Self {
        Self::with_config(AnomalyConfig::default())
    }

    /// Creates a new anomaly detector with custom config
    pub fn with_config(config: AnomalyConfig) -> Self {
        Self {
            config: Arc::new(RwLock::new(config)),
            metrics: Arc::new(RwLock::new(HashMap::new())),
            anomalies: Arc::new(RwLock::new(VecDeque::new())),
            max_anomalies: 1000,
        }
    }

    /// Records a metric value
    pub async fn record(&self, metric_name: String, value: f64) {
        let config = self.config.read().await;

        if !config.enabled {
            return;
        }

        let window = Duration::seconds(config.window_seconds);
        let max_points = config.max_data_points;

        drop(config);

        let mut metrics = self.metrics.write().await;
        let time_series = metrics
            .entry(metric_name.clone())
            .or_insert_with(|| TimeSeries::new(max_points, window));

        time_series.add(value);

        // Check for anomalies
        let config = self.config.read().await;
        if time_series.points.len() >= config.min_observations {
            if time_series.is_anomaly(value, config.threshold) {
                let z_score = time_series.z_score(value);
                let baseline = time_series.mean();

                let severity = if z_score > 5.0 {
                    AnomalySeverity::Critical
                } else if z_score > 4.0 {
                    AnomalySeverity::High
                } else if z_score > 3.5 {
                    AnomalySeverity::Medium
                } else {
                    AnomalySeverity::Low
                };

                let anomaly = Anomaly {
                    anomaly_type: AnomalyType::PatternAnomaly,
                    severity,
                    description: format!(
                        "Unusual value for {}: {:.2} (baseline: {:.2}, {:.1}σ)",
                        metric_name, value, baseline, z_score
                    ),
                    resource: metric_name,
                    value,
                    baseline,
                    std_deviations: z_score,
                    detected_at: Utc::now(),
                    metadata: None,
                };

                drop(config);
                self.record_anomaly(anomaly).await;
            }
        }
    }

    /// Records an anomaly
    async fn record_anomaly(&self, anomaly: Anomaly) {
        let mut anomalies = self.anomalies.write().await;
        anomalies.push_back(anomaly);

        // Limit the number of stored anomalies
        while anomalies.len() > self.max_anomalies {
            anomalies.pop_front();
        }
    }

    /// Gets recent anomalies
    pub async fn get_anomalies(&self, limit: usize) -> Vec<Anomaly> {
        let anomalies = self.anomalies.read().await;
        anomalies.iter().rev().take(limit).cloned().collect()
    }

    /// Gets anomalies by severity
    pub async fn get_anomalies_by_severity(
        &self,
        severity: AnomalySeverity,
        limit: usize,
    ) -> Vec<Anomaly> {
        let anomalies = self.anomalies.read().await;
        anomalies
            .iter()
            .rev()
            .filter(|a| a.severity >= severity)
            .take(limit)
            .cloned()
            .collect()
    }

    /// Gets anomalies by type
    pub async fn get_anomalies_by_type(
        &self,
        anomaly_type: AnomalyType,
        limit: usize,
    ) -> Vec<Anomaly> {
        let anomalies = self.anomalies.read().await;
        anomalies
            .iter()
            .rev()
            .filter(|a| a.anomaly_type == anomaly_type)
            .take(limit)
            .cloned()
            .collect()
    }

    /// Clears all anomalies
    pub async fn clear_anomalies(&self) {
        let mut anomalies = self.anomalies.write().await;
        anomalies.clear();
    }

    /// Gets current statistics for a metric
    pub async fn get_metric_stats(&self, metric_name: &str) -> Option<(f64, f64)> {
        let metrics = self.metrics.read().await;
        metrics.get(metric_name).map(|ts| (ts.mean(), ts.std_dev()))
    }

    /// Updates configuration
    pub async fn update_config(&self, config: AnomalyConfig) {
        let mut current_config = self.config.write().await;
        *current_config = config;
    }

    /// Gets current configuration
    pub async fn get_config(&self) -> AnomalyConfig {
        let config = self.config.read().await;
        config.clone()
    }
}

impl Default for AnomalyDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_series_mean() {
        let mut ts = TimeSeries::new(100, Duration::hours(1));
        ts.add(10.0);
        ts.add(20.0);
        ts.add(30.0);

        assert_eq!(ts.mean(), 20.0);
    }

    #[test]
    fn test_time_series_std_dev() {
        let mut ts = TimeSeries::new(100, Duration::hours(1));
        ts.add(10.0);
        ts.add(20.0);
        ts.add(30.0);

        let std_dev = ts.std_dev();
        assert!(std_dev > 0.0);
        assert!((std_dev - 10.0).abs() < 0.1);
    }

    #[test]
    fn test_time_series_anomaly_detection() {
        let mut ts = TimeSeries::new(100, Duration::hours(1));

        // Add baseline values with some variation
        for i in 0..30 {
            ts.add(10.0 + (i % 3) as f64);
        }

        // Normal value should not be anomaly
        assert!(!ts.is_anomaly(10.0, 3.0));

        // Value far from baseline should be anomaly
        assert!(ts.is_anomaly(100.0, 3.0));
    }

    #[tokio::test]
    async fn test_anomaly_detector_record() {
        let detector = AnomalyDetector::new();

        // Add baseline values
        for _ in 0..30 {
            detector.record("test_metric".to_string(), 10.0).await;
        }

        // Add anomalous value
        detector.record("test_metric".to_string(), 100.0).await;

        // Check if anomaly was detected
        let anomalies = detector.get_anomalies(10).await;
        assert!(!anomalies.is_empty());
    }

    #[tokio::test]
    async fn test_anomaly_detector_severity() {
        let detector = AnomalyDetector::new();

        // Add baseline
        for _ in 0..30 {
            detector.record("test_metric".to_string(), 10.0).await;
        }

        // Add critical anomaly
        detector.record("test_metric".to_string(), 200.0).await;

        let critical_anomalies = detector
            .get_anomalies_by_severity(AnomalySeverity::Critical, 10)
            .await;
        assert!(!critical_anomalies.is_empty());
    }

    #[tokio::test]
    async fn test_anomaly_detector_stats() {
        let detector = AnomalyDetector::new();

        detector.record("test_metric".to_string(), 10.0).await;
        detector.record("test_metric".to_string(), 20.0).await;
        detector.record("test_metric".to_string(), 30.0).await;

        let stats = detector.get_metric_stats("test_metric").await;
        assert!(stats.is_some());

        let (mean, _std_dev) = stats.unwrap();
        assert_eq!(mean, 20.0);
    }
}
