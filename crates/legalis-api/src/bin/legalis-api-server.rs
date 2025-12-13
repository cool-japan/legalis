//! Legalis API Server Binary
//!
//! Standalone HTTP server for the Legalis REST API.

use legalis_api::{AppState, ServerConfig, create_router};
use std::sync::Arc;
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // Load configuration from environment
    let config = ServerConfig {
        host: std::env::var("LEGALIS_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
        port: std::env::var("LEGALIS_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(3000),
    };

    let bind_addr = config.bind_addr();
    info!("Starting Legalis API server on {}", bind_addr);

    // Create application state
    let state = Arc::new(AppState::new());

    // Create router
    let app = create_router(state);

    // Create TCP listener
    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    info!("Server listening on http://{}", bind_addr);
    info!("Health check: http://{}/health", bind_addr);
    info!("API docs: http://{}/api-docs/openapi.json", bind_addr);

    // Start server
    axum::serve(listener, app).await?;

    Ok(())
}
