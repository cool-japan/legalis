//! Observability and metrics for LLM operations.
//!
//! This module provides metrics collection, performance tracking,
//! and monitoring capabilities for LLM providers.

use crate::{LLMProvider, TextStream, TokenUsage};
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// Metrics for a single LLM request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestMetrics {
    /// Request timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Provider name
    pub provider: String,
    /// Model name
    pub model: String,
    /// Request duration
    pub duration_ms: u128,
    /// Token usage
    pub tokens: Option<TokenUsage>,
    /// Estimated cost in USD
    pub cost_usd: Option<f64>,
    /// Whether the request succeeded
    pub success: bool,
    /// Error message if failed
    pub error: Option<String>,
}

impl RequestMetrics {
    /// Creates a new request metrics record.
    pub fn new(provider: String, model: String, duration_ms: u128, success: bool) -> Self {
        Self {
            timestamp: chrono::Utc::now(),
            provider,
            model,
            duration_ms,
            tokens: None,
            cost_usd: None,
            success,
            error: None,
        }
    }

    /// Adds token usage information.
    pub fn with_tokens(mut self, tokens: TokenUsage) -> Self {
        self.tokens = Some(tokens);
        self
    }

    /// Adds cost information.
    pub fn with_cost(mut self, cost: f64) -> Self {
        self.cost_usd = Some(cost);
        self
    }

    /// Adds error information.
    pub fn with_error(mut self, error: impl Into<String>) -> Self {
        self.error = Some(error.into());
        self
    }
}

/// Aggregated metrics across multiple requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedMetrics {
    /// Total number of requests
    pub total_requests: u64,
    /// Number of successful requests
    pub successful_requests: u64,
    /// Number of failed requests
    pub failed_requests: u64,
    /// Success rate (0.0 - 1.0)
    pub success_rate: f64,
    /// Total tokens used (input + output)
    pub total_tokens: u64,
    /// Total cost in USD
    pub total_cost_usd: f64,
    /// Average request duration in milliseconds
    pub avg_duration_ms: f64,
    /// p50 (median) latency in milliseconds
    pub p50_latency_ms: u128,
    /// p95 latency in milliseconds
    pub p95_latency_ms: u128,
    /// p99 latency in milliseconds
    pub p99_latency_ms: u128,
}

impl Default for AggregatedMetrics {
    fn default() -> Self {
        Self {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            success_rate: 0.0,
            total_tokens: 0,
            total_cost_usd: 0.0,
            avg_duration_ms: 0.0,
            p50_latency_ms: 0,
            p95_latency_ms: 0,
            p99_latency_ms: 0,
        }
    }
}

/// Metrics collector for LLM operations.
pub struct MetricsCollector {
    metrics: Arc<RwLock<Vec<RequestMetrics>>>,
    max_history: usize,
}

impl MetricsCollector {
    /// Creates a new metrics collector.
    pub fn new(max_history: usize) -> Self {
        Self {
            metrics: Arc::new(RwLock::new(Vec::new())),
            max_history,
        }
    }

    /// Records a request metric.
    pub async fn record(&self, metric: RequestMetrics) {
        let mut metrics = self.metrics.write().await;
        metrics.push(metric);

        // Trim to max history
        if metrics.len() > self.max_history {
            let excess = metrics.len() - self.max_history;
            metrics.drain(0..excess);
        }
    }

    /// Returns all recorded metrics.
    pub async fn get_all(&self) -> Vec<RequestMetrics> {
        self.metrics.read().await.clone()
    }

    /// Computes aggregated metrics.
    pub async fn aggregate(&self) -> AggregatedMetrics {
        let metrics = self.metrics.read().await;

        if metrics.is_empty() {
            return AggregatedMetrics::default();
        }

        let total_requests = metrics.len() as u64;
        let successful_requests = metrics.iter().filter(|m| m.success).count() as u64;
        let failed_requests = total_requests - successful_requests;
        let success_rate = successful_requests as f64 / total_requests as f64;

        let total_tokens: u64 = metrics
            .iter()
            .filter_map(|m| m.tokens.as_ref().map(|t| t.total_tokens as u64))
            .sum();

        let total_cost_usd: f64 = metrics.iter().filter_map(|m| m.cost_usd).sum();

        let avg_duration_ms =
            metrics.iter().map(|m| m.duration_ms as f64).sum::<f64>() / total_requests as f64;

        // Calculate percentiles
        let mut durations: Vec<u128> = metrics.iter().map(|m| m.duration_ms).collect();
        durations.sort_unstable();

        let p50_idx = (durations.len() as f64 * 0.50) as usize;
        let p95_idx = (durations.len() as f64 * 0.95) as usize;
        let p99_idx = (durations.len() as f64 * 0.99) as usize;

        let p50_latency_ms = durations.get(p50_idx).copied().unwrap_or(0);
        let p95_latency_ms = durations.get(p95_idx).copied().unwrap_or(0);
        let p99_latency_ms = durations.get(p99_idx).copied().unwrap_or(0);

        AggregatedMetrics {
            total_requests,
            successful_requests,
            failed_requests,
            success_rate,
            total_tokens,
            total_cost_usd,
            avg_duration_ms,
            p50_latency_ms,
            p95_latency_ms,
            p99_latency_ms,
        }
    }

    /// Clears all recorded metrics.
    pub async fn clear(&self) {
        self.metrics.write().await.clear();
    }

    /// Returns metrics for a specific time window.
    pub async fn get_since(&self, since: chrono::DateTime<chrono::Utc>) -> Vec<RequestMetrics> {
        let metrics = self.metrics.read().await;
        metrics
            .iter()
            .filter(|m| m.timestamp >= since)
            .cloned()
            .collect()
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new(1000)
    }
}

/// LLM provider with metrics collection.
pub struct ObservableProvider<P> {
    provider: P,
    collector: Arc<MetricsCollector>,
}

impl<P> ObservableProvider<P> {
    /// Creates a new observable provider.
    pub fn new(provider: P, collector: Arc<MetricsCollector>) -> Self {
        Self {
            provider,
            collector,
        }
    }

    /// Gets the metrics collector.
    pub fn collector(&self) -> Arc<MetricsCollector> {
        self.collector.clone()
    }

    /// Gets the underlying provider.
    pub fn provider(&self) -> &P {
        &self.provider
    }
}

#[async_trait]
impl<P: LLMProvider> LLMProvider for ObservableProvider<P> {
    async fn generate_text(&self, prompt: &str) -> Result<String> {
        let start = Instant::now();
        let result = self.provider.generate_text(prompt).await;
        let duration = start.elapsed();

        let metric = match &result {
            Ok(_) => RequestMetrics::new(
                self.provider.provider_name().to_string(),
                self.provider.model_name().to_string(),
                duration.as_millis(),
                true,
            ),
            Err(e) => RequestMetrics::new(
                self.provider.provider_name().to_string(),
                self.provider.model_name().to_string(),
                duration.as_millis(),
                false,
            )
            .with_error(e.to_string()),
        };

        self.collector.record(metric).await;
        result
    }

    async fn generate_structured<T: serde::de::DeserializeOwned + Send>(
        &self,
        prompt: &str,
    ) -> Result<T> {
        let start = Instant::now();
        let result = self.provider.generate_structured::<T>(prompt).await;
        let duration = start.elapsed();

        let metric = match &result {
            Ok(_) => RequestMetrics::new(
                self.provider.provider_name().to_string(),
                self.provider.model_name().to_string(),
                duration.as_millis(),
                true,
            ),
            Err(e) => RequestMetrics::new(
                self.provider.provider_name().to_string(),
                self.provider.model_name().to_string(),
                duration.as_millis(),
                false,
            )
            .with_error(e.to_string()),
        };

        self.collector.record(metric).await;
        result
    }

    async fn generate_text_stream(&self, prompt: &str) -> Result<TextStream> {
        let start = Instant::now();
        let result = self.provider.generate_text_stream(prompt).await;

        // Record initial metric (stream started)
        let metric = match &result {
            Ok(_) => RequestMetrics::new(
                self.provider.provider_name().to_string(),
                self.provider.model_name().to_string(),
                start.elapsed().as_millis(),
                true,
            ),
            Err(e) => RequestMetrics::new(
                self.provider.provider_name().to_string(),
                self.provider.model_name().to_string(),
                start.elapsed().as_millis(),
                false,
            )
            .with_error(e.to_string()),
        };

        self.collector.record(metric).await;
        result
    }

    fn provider_name(&self) -> &str {
        self.provider.provider_name()
    }

    fn model_name(&self) -> &str {
        self.provider.model_name()
    }

    fn supports_streaming(&self) -> bool {
        self.provider.supports_streaming()
    }
}

/// Performance timer for tracking operation durations.
pub struct PerformanceTimer {
    start: Instant,
    label: String,
}

impl PerformanceTimer {
    /// Starts a new timer with a label.
    pub fn start(label: impl Into<String>) -> Self {
        Self {
            start: Instant::now(),
            label: label.into(),
        }
    }

    /// Returns the elapsed duration.
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    /// Stops the timer and returns the elapsed duration.
    pub fn stop(self) -> Duration {
        self.elapsed()
    }

    /// Returns the label.
    pub fn label(&self) -> &str {
        &self.label
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MockProvider;

    #[test]
    fn test_request_metrics() {
        let metric = RequestMetrics::new("test".to_string(), "model".to_string(), 100, true)
            .with_tokens(TokenUsage::new(10, 20))
            .with_cost(0.05);

        assert_eq!(metric.provider, "test");
        assert_eq!(metric.model, "model");
        assert_eq!(metric.duration_ms, 100);
        assert!(metric.success);
        assert_eq!(metric.tokens.unwrap().total_tokens, 30);
        assert_eq!(metric.cost_usd.unwrap(), 0.05);
    }

    #[tokio::test]
    async fn test_metrics_collector() {
        let collector = MetricsCollector::new(100);

        let metric1 = RequestMetrics::new("provider1".to_string(), "model1".to_string(), 50, true);
        let metric2 = RequestMetrics::new("provider2".to_string(), "model2".to_string(), 100, true);

        collector.record(metric1).await;
        collector.record(metric2).await;

        let all = collector.get_all().await;
        assert_eq!(all.len(), 2);

        collector.clear().await;
        let all = collector.get_all().await;
        assert_eq!(all.len(), 0);
    }

    #[tokio::test]
    async fn test_aggregated_metrics() {
        let collector = MetricsCollector::new(100);

        for i in 0..10 {
            let success = i < 8;
            let metric = RequestMetrics::new(
                "test".to_string(),
                "model".to_string(),
                (i + 1) * 10,
                success,
            );
            collector.record(metric).await;
        }

        let agg = collector.aggregate().await;
        assert_eq!(agg.total_requests, 10);
        assert_eq!(agg.successful_requests, 8);
        assert_eq!(agg.failed_requests, 2);
        assert!((agg.success_rate - 0.8).abs() < f64::EPSILON);
    }

    #[tokio::test]
    async fn test_observable_provider() {
        let provider = MockProvider::default();
        let collector = Arc::new(MetricsCollector::new(100));
        let observable = ObservableProvider::new(provider, collector.clone());

        let result = observable.generate_text("test").await;
        assert!(result.is_ok());

        let metrics = collector.get_all().await;
        assert_eq!(metrics.len(), 1);
        assert!(metrics[0].success);
    }

    #[test]
    fn test_performance_timer() {
        let timer = PerformanceTimer::start("test operation");
        std::thread::sleep(Duration::from_millis(10));
        let elapsed = timer.stop();

        assert!(elapsed.as_millis() >= 10);
    }

    #[tokio::test]
    async fn test_metrics_time_window() {
        let collector = MetricsCollector::new(100);

        let now = chrono::Utc::now();
        let past = now - chrono::Duration::hours(1);

        let metric1 = RequestMetrics::new("test".to_string(), "model".to_string(), 10, true);
        collector.record(metric1).await;

        // Get metrics from the past hour
        let recent = collector.get_since(past).await;
        assert_eq!(recent.len(), 1);

        // Get metrics from the future (should be empty)
        let future = collector.get_since(now + chrono::Duration::hours(1)).await;
        assert_eq!(future.len(), 0);
    }
}
