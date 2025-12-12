//! Legalis CLI entry point.

use anyhow::Result;
use clap::Parser;
use legalis_cli::{commands, Cli, Commands};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging based on verbosity
    let log_level = match cli.verbose {
        0 => "warn",
        1 => "info",
        2 => "debug",
        _ => "trace",
    };

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::new(log_level))
        .init();

    match &cli.command {
        Commands::Parse { input, output } => {
            commands::handle_parse(input, output.as_deref(), &cli.format)?;
        }
        Commands::Verify { input, strict } => {
            commands::handle_verify(input, *strict, &cli.format)?;
        }
        Commands::Viz {
            input,
            output,
            viz_format,
        } => {
            commands::handle_viz(input, output, viz_format)?;
        }
        Commands::Export {
            input,
            output,
            export_format,
        } => {
            commands::handle_export(input, output, export_format)?;
        }
        Commands::Serve { host, port } => {
            println!("Starting Legalis API server on {}:{}...", host, port);
            let state = std::sync::Arc::new(legalis_api::AppState::new());
            let app = legalis_api::create_router(state);
            let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port)).await?;
            println!("Server listening on http://{}:{}", host, port);
            axum::serve(listener, app).await?;
        }
        Commands::Init { path } => {
            commands::handle_init(path)?;
        }
    }

    Ok(())
}
