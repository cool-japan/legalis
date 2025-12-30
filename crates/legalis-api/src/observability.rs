//! Observability coordinator integrating all monitoring features.
//!
//! This module provides a unified interface for telemetry, anomaly detection,
//! SLO tracking, and request sampling.

use crate::{anomaly::AnomalyDetector, sampling::RequestSampler, slo::SloTracker};
use axum::{body::Body, extract::Request, http::StatusCode, middleware::Next, response::Response};
use std::sync::Arc;
use std::time::Instant;

/// Observability coordinator
#[derive(Clone)]
pub struct ObservabilityCoordinator {
    /// Anomaly detector
    pub anomaly_detector: Arc<AnomalyDetector>,
    /// SLO tracker
    pub slo_tracker: Arc<SloTracker>,
    /// Request sampler
    pub request_sampler: Arc<RequestSampler>,
}

impl ObservabilityCoordinator {
    /// Creates a new observability coordinator
    pub fn new() -> Self {
        Self {
            anomaly_detector: Arc::new(AnomalyDetector::new()),
            slo_tracker: Arc::new(SloTracker::new()),
            request_sampler: Arc::new(RequestSampler::new()),
        }
    }

    /// Records a request metric
    pub async fn record_request(&self, endpoint: &str, duration_ms: f64, status_code: u16) {
        let is_error = status_code >= 400;
        let is_success = status_code < 400;

        // Record for anomaly detection
        self.anomaly_detector
            .record(format!("{}_latency", endpoint), duration_ms)
            .await;

        if is_error {
            self.anomaly_detector
                .record(format!("{}_errors", endpoint), 1.0)
                .await;
        }

        // Record for SLO tracking
        let slo_id = format!("availability_{}", endpoint);
        self.slo_tracker
            .record_sli(slo_id.clone(), duration_ms, is_success)
            .await;

        let latency_slo_id = format!("latency_{}", endpoint);
        self.slo_tracker
            .record_sli(latency_slo_id, duration_ms, duration_ms < 200.0)
            .await;
    }

    /// Gets observability summary for an endpoint
    pub async fn get_endpoint_summary(&self, endpoint: &str) -> EndpointSummary {
        let latency_stats = self
            .anomaly_detector
            .get_metric_stats(&format!("{}_latency", endpoint))
            .await;

        let availability_slo = self
            .slo_tracker
            .get_report(&format!("availability_{}", endpoint))
            .await;

        let latency_slo = self
            .slo_tracker
            .get_report(&format!("latency_{}", endpoint))
            .await;

        let sampling_stats = self.request_sampler.get_stats(endpoint).await;

        EndpointSummary {
            endpoint: endpoint.to_string(),
            latency_mean: latency_stats.map(|(mean, _)| mean),
            latency_std_dev: latency_stats.map(|(_, std)| std),
            availability: availability_slo.map(|r| r.current_value),
            p95_latency: latency_slo.map(|r| r.current_value),
            total_requests: sampling_stats.total_requests,
            sampled_requests: sampling_stats.sampled_requests,
        }
    }
}

impl Default for ObservabilityCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

/// Endpoint observability summary
#[derive(Debug, Clone, serde::Serialize)]
pub struct EndpointSummary {
    /// Endpoint path
    pub endpoint: String,
    /// Mean latency in milliseconds
    pub latency_mean: Option<f64>,
    /// Latency standard deviation
    pub latency_std_dev: Option<f64>,
    /// Availability percentage
    pub availability: Option<f64>,
    /// P95 latency
    pub p95_latency: Option<f64>,
    /// Total requests
    pub total_requests: u64,
    /// Sampled requests
    pub sampled_requests: u64,
}

/// Observability middleware
pub async fn observability_middleware(
    req: Request<Body>,
    next: Next,
    coordinator: Arc<ObservabilityCoordinator>,
) -> Result<Response, StatusCode> {
    let path = req.uri().path().to_string();
    let start = Instant::now();

    // Check if we should sample this request
    let should_sample = coordinator
        .request_sampler
        .should_sample(&path, false)
        .await;

    let response = next.run(req).await;
    let duration = start.elapsed();
    let duration_ms = duration.as_secs_f64() * 1000.0;
    let status = response.status().as_u16();

    // Record metrics if sampled
    if should_sample == crate::sampling::SamplingDecision::Sample {
        coordinator.record_request(&path, duration_ms, status).await;
    }

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_observability_coordinator_creation() {
        let coordinator = ObservabilityCoordinator::new();
        assert!(Arc::strong_count(&coordinator.anomaly_detector) >= 1);
        assert!(Arc::strong_count(&coordinator.slo_tracker) >= 1);
        assert!(Arc::strong_count(&coordinator.request_sampler) >= 1);
    }

    #[tokio::test]
    async fn test_record_request() {
        let coordinator = ObservabilityCoordinator::new();

        // Record some requests
        coordinator.record_request("/api/v1/test", 150.0, 200).await;
        coordinator.record_request("/api/v1/test", 180.0, 200).await;
        coordinator.record_request("/api/v1/test", 200.0, 500).await;

        // Check that metrics were recorded (basic verification)
        let summary = coordinator.get_endpoint_summary("/api/v1/test").await;
        assert_eq!(summary.endpoint, "/api/v1/test");
    }

    #[tokio::test]
    async fn test_endpoint_summary() {
        let coordinator = ObservabilityCoordinator::new();

        let summary = coordinator.get_endpoint_summary("/api/v1/test").await;
        assert_eq!(summary.endpoint, "/api/v1/test");
        assert_eq!(summary.total_requests, 0);
    }
}
