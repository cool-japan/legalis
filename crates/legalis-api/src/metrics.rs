//! Prometheus metrics for the API.

use lazy_static::lazy_static;
use prometheus::{
    Encoder, HistogramOpts, HistogramVec, IntCounter, IntCounterVec, IntGauge, Registry,
    TextEncoder,
};
use std::sync::Once;

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

    /// Statute operations counter (by operation type).
    pub static ref STATUTE_OPERATIONS: IntCounterVec = IntCounterVec::new(
        prometheus::opts!("legalis_statute_operations_total", "Total statute operations"),
        &["operation"]  // create, update, delete, version
    )
    .expect("metric can be created");

    /// Verification results counter (by result type).
    pub static ref VERIFICATION_RESULTS: IntCounterVec = IntCounterVec::new(
        prometheus::opts!("legalis_verification_results_total", "Total verification results"),
        &["result"]  // passed, failed
    )
    .expect("metric can be created");

    /// Simulation outcome distribution (by outcome type).
    pub static ref SIMULATION_OUTCOMES: IntCounterVec = IntCounterVec::new(
        prometheus::opts!("legalis_simulation_outcomes_total", "Total simulation outcomes"),
        &["outcome"]  // deterministic, discretionary, void
    )
    .expect("metric can be created");

    /// Audit log entries counter (by event type).
    pub static ref AUDIT_LOG_ENTRIES: IntCounterVec = IntCounterVec::new(
        prometheus::opts!("legalis_audit_log_entries_total", "Total audit log entries"),
        &["event_type"]
    )
    .expect("metric can be created");

    /// Permission operations counter (by operation type).
    pub static ref PERMISSION_OPERATIONS: IntCounterVec = IntCounterVec::new(
        prometheus::opts!("legalis_permission_operations_total", "Total permission operations"),
        &["operation"]  // grant, revoke
    )
    .expect("metric can be created");

    /// Active WebSocket connections gauge.
    pub static ref WEBSOCKET_CONNECTIONS: IntGauge =
        IntGauge::new("legalis_websocket_connections", "Active WebSocket connections")
            .expect("metric can be created");

    /// GraphQL query duration histogram.
    pub static ref GRAPHQL_QUERY_DURATION: HistogramVec = HistogramVec::new(
        HistogramOpts::new(
            "legalis_graphql_query_duration_seconds",
            "GraphQL query duration in seconds"
        ),
        &["query_name"]
    )
    .expect("metric can be created");
}

static INIT: Once = Once::new();

/// Initialize and register all metrics.
/// This function is idempotent and safe to call multiple times.
pub fn init() {
    INIT.call_once(|| {
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
        REGISTRY
            .register(Box::new(STATUTE_OPERATIONS.clone()))
            .expect("metric can be registered");
        REGISTRY
            .register(Box::new(VERIFICATION_RESULTS.clone()))
            .expect("metric can be registered");
        REGISTRY
            .register(Box::new(SIMULATION_OUTCOMES.clone()))
            .expect("metric can be registered");
        REGISTRY
            .register(Box::new(AUDIT_LOG_ENTRIES.clone()))
            .expect("metric can be registered");
        REGISTRY
            .register(Box::new(PERMISSION_OPERATIONS.clone()))
            .expect("metric can be registered");
        REGISTRY
            .register(Box::new(WEBSOCKET_CONNECTIONS.clone()))
            .expect("metric can be registered");
        REGISTRY
            .register(Box::new(GRAPHQL_QUERY_DURATION.clone()))
            .expect("metric can be registered");
    });
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
