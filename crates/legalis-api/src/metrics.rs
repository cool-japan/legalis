//! Prometheus metrics for the API.

use lazy_static::lazy_static;
use prometheus::{
    Encoder, HistogramOpts, HistogramVec, IntCounter, IntCounterVec, IntGauge, Registry,
    TextEncoder,
};

lazy_static! {
    /// Global metrics registry.
    pub static ref REGISTRY: Registry = Registry::new();

    /// Total HTTP requests counter.
    pub static ref HTTP_REQUESTS_TOTAL: IntCounterVec = IntCounterVec::new(
        prometheus::opts!("legalis_http_requests_total", "Total HTTP requests"),
        &["method", "path", "status"]
    )
    .expect("metric can be created");

    /// HTTP request duration histogram.
    pub static ref HTTP_REQUEST_DURATION: HistogramVec = HistogramVec::new(
        HistogramOpts::new(
            "legalis_http_request_duration_seconds",
            "HTTP request duration in seconds"
        ),
        &["method", "path"]
    )
    .expect("metric can be created");

    /// Total statutes counter.
    pub static ref STATUTES_TOTAL: IntGauge =
        IntGauge::new("legalis_statutes_total", "Total number of statutes")
            .expect("metric can be created");

    /// Total verification requests counter.
    pub static ref VERIFICATIONS_TOTAL: IntCounter =
        IntCounter::new("legalis_verifications_total", "Total verification requests")
            .expect("metric can be created");

    /// Total simulation requests counter.
    pub static ref SIMULATIONS_TOTAL: IntCounter =
        IntCounter::new("legalis_simulations_total", "Total simulation requests")
            .expect("metric can be created");
}

/// Initialize and register all metrics.
pub fn init() {
    REGISTRY
        .register(Box::new(HTTP_REQUESTS_TOTAL.clone()))
        .expect("metric can be registered");
    REGISTRY
        .register(Box::new(HTTP_REQUEST_DURATION.clone()))
        .expect("metric can be registered");
    REGISTRY
        .register(Box::new(STATUTES_TOTAL.clone()))
        .expect("metric can be registered");
    REGISTRY
        .register(Box::new(VERIFICATIONS_TOTAL.clone()))
        .expect("metric can be registered");
    REGISTRY
        .register(Box::new(SIMULATIONS_TOTAL.clone()))
        .expect("metric can be registered");
}

/// Encode metrics into Prometheus text format.
pub fn encode() -> Result<String, prometheus::Error> {
    let encoder = TextEncoder::new();
    let metric_families = REGISTRY.gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer)?;
    String::from_utf8(buffer)
        .map_err(|e| prometheus::Error::Msg(format!("Failed to convert metrics to UTF-8: {}", e)))
}
