//! Legalis CLI entry point.

use anyhow::Result;
use clap::Parser;
use legalis_cli::{Cli, Commands, commands, generate_completions};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Load configuration
    let _config = if let Some(config_path) = &cli.config {
        legalis_cli::config::Config::from_file(std::path::Path::new(config_path))?
    } else {
        legalis_cli::config::Config::load()
    };

    // Initialize logging based on verbosity
    let log_level = if cli.quiet {
        "error"
    } else {
        match cli.verbose {
            0 => "warn",
            1 => "info",
            2 => "debug",
            _ => "trace",
        }
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
        Commands::Init { path, dry_run } => {
            commands::handle_init(path, *dry_run)?;
        }
        Commands::Diff {
            old,
            new,
            diff_format,
        } => {
            commands::handle_diff(old, new, diff_format)?;
        }
        Commands::Simulate {
            input,
            population,
            output,
        } => {
            commands::handle_simulate(input, *population, output.as_deref()).await?;
        }
        Commands::Audit {
            input,
            output,
            with_complexity,
        } => {
            commands::handle_audit(input, output, *with_complexity)?;
        }
        Commands::Complexity { input, output } => {
            commands::handle_complexity(input, output.as_deref())?;
        }
        Commands::Port {
            input,
            target,
            output,
            port_format,
        } => {
            commands::handle_port(input, target, output.as_deref(), port_format)?;
        }
        Commands::Import {
            input,
            from,
            output,
            import_output,
        } => {
            commands::handle_import(input, from.as_ref(), output.as_deref(), import_output)?;
        }
        Commands::Convert {
            input,
            from,
            to,
            output,
        } => {
            commands::handle_convert(input, from.as_ref(), to, output.as_deref())?;
        }
        Commands::Completions { shell } => {
            generate_completions(*shell);
        }
        Commands::Lod {
            input,
            output,
            rdf_format,
            base_uri,
        } => {
            commands::handle_lod(input, output.as_deref(), rdf_format, base_uri)?;
        }
        Commands::Format {
            input,
            output,
            inplace,
            style,
            dry_run,
        } => {
            commands::handle_format(input, output.as_deref(), *inplace, style, *dry_run)?;
        }
        Commands::Lint { input, fix, strict } => {
            commands::handle_lint(input, *fix, *strict)?;
        }
        Commands::Watch { input, command } => {
            commands::handle_watch(input, command).await?;
        }
        Commands::Test {
            input,
            tests,
            verbose,
        } => {
            commands::handle_test(input, tests, *verbose)?;
        }
        Commands::New {
            name,
            template,
            output,
        } => {
            commands::handle_new(name, template, output.as_deref())?;
        }
        Commands::Doctor { verbose } => {
            commands::handle_doctor(*verbose)?;
        }
    }

    Ok(())
}
