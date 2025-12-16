//! Legalis API Server Binary
//!
//! Standalone HTTP server for the Legalis REST API.

use legalis_api::{AppState, config::Config, create_router, logging};
use std::sync::Arc;
use tokio::signal;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration from environment
    let config = Config::from_env();

    // Initialize logging
    logging::init_logging();

    let bind_addr = config.bind_addr();
    info!("Starting Legalis API server");
    info!("Configuration:");
    info!("  Host: {}", config.host);
    info!("  Port: {}", config.port);
    info!("  Log level: {}", config.log_level);
    info!("  Max body size: {} bytes", config.max_body_size);
    info!("  Request timeout: {}s", config.request_timeout_secs);

    // Create application state
    let state = Arc::new(AppState::new());

    // Create router
    let app = create_router(state);

    // Create TCP listener
    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    info!("Server listening on http://{}", bind_addr);
    info!("Health check: http://{}/health", bind_addr);
    info!("Metrics: http://{}/metrics", bind_addr);
    info!("API docs: http://{}/api-docs/openapi.json", bind_addr);

    // Start server with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    info!("Server shutdown complete");
    Ok(())
}

/// Wait for shutdown signal (Ctrl+C or SIGTERM).
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("Received Ctrl+C, shutting down gracefully...");
        }
        _ = terminate => {
            info!("Received SIGTERM, shutting down gracefully...");
        }
    }
}
