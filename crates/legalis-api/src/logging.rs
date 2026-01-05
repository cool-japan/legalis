//! Structured logging middleware for API requests.

use axum::{extract::Request, middleware::Next, response::Response};
use std::time::Instant;
use tracing::{info, warn};

/// Middleware for structured request logging.
pub async fn log_request(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let version = req.version();

    let start = Instant::now();

    // Process request
    let response = next.run(req).await;

    let duration = start.elapsed();
    let status = response.status();

    // Log based on status code
    if status.is_server_error() {
        warn!(
            method = %method,
            uri = %uri,
            version = ?version,
            status = %status.as_u16(),
            duration_ms = %duration.as_millis(),
            "Request completed with server error"
        );
    } else if status.is_client_error() {
        warn!(
            method = %method,
            uri = %uri,
            version = ?version,
            status = %status.as_u16(),
            duration_ms = %duration.as_millis(),
            "Request completed with client error"
        );
    } else {
        info!(
            method = %method,
            uri = %uri,
            version = ?version,
            status = %status.as_u16(),
            duration_ms = %duration.as_millis(),
            "Request completed successfully"
        );
    }

    response
}

/// Initialize structured logging.
/// Set the RUST_LOG environment variable to control log levels.
/// Example: RUST_LOG=legalis_api=debug,tower_http=debug
pub fn init_logging() {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "legalis_api=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
