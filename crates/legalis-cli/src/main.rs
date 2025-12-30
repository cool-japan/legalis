//! Legalis CLI entry point.

use anyhow::Result;
use clap::Parser;
use legalis_cli::{
    Cli, Commands, commands, generate_all_man_pages, generate_completions, generate_man_page,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration early to support alias expansion
    let config = legalis_cli::config::Config::load();

    // Expand aliases in arguments
    let args: Vec<String> = std::env::args().collect();
    let expanded_args = config.expand_aliases(args);

    // Parse CLI with potentially expanded arguments
    let cli = Cli::parse_from(expanded_args);

    // Reload configuration with potentially specified config file
    let _config = if let Some(config_path) = &cli.config {
        legalis_cli::config::Config::from_file(std::path::Path::new(config_path))?
    } else {
        config
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

    // Handle interactive mode for supported commands
    if cli.interactive {
        match &cli.command {
            Commands::New { .. } => {
                let (name, template_str, output) =
                    legalis_cli::interactive::interactive_new_statute()?;
                let template = match template_str.as_str() {
                    "basic" => legalis_cli::StatuteTemplate::Basic,
                    "income" => legalis_cli::StatuteTemplate::Income,
                    "geographic" => legalis_cli::StatuteTemplate::Geographic,
                    "temporal" => legalis_cli::StatuteTemplate::Temporal,
                    "complex" => legalis_cli::StatuteTemplate::Complex,
                    _ => anyhow::bail!("Unknown template: {}", template_str),
                };
                commands::handle_new(&name, &template, output.as_deref())?;
                return Ok(());
            }
            Commands::Verify { .. } => {
                let (files, strict) = legalis_cli::interactive::interactive_verify()?;
                commands::handle_verify(&files, strict, &cli.format)?;
                return Ok(());
            }
            Commands::Export { .. } => {
                let (input, output, format_str) = legalis_cli::interactive::interactive_export()?;
                let export_format = match format_str.as_str() {
                    "json" => legalis_cli::ExportFormat::Json,
                    "yaml" => legalis_cli::ExportFormat::Yaml,
                    "solidity" => legalis_cli::ExportFormat::Solidity,
                    _ => anyhow::bail!("Unknown format: {}", format_str),
                };
                commands::handle_export(&input, &output, &export_format)?;
                return Ok(());
            }
            _ => {
                eprintln!(
                    "Warning: Interactive mode not supported for this command. Using normal mode."
                );
            }
        }
    }

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
        Commands::ManPage { output } => {
            if let Some(output_dir) = output {
                generate_all_man_pages(std::path::Path::new(output_dir))?;
                println!("Man pages generated in: {}", output_dir);
            } else {
                generate_man_page()?;
            }
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
        Commands::Repl { load, no_color } => {
            commands::handle_repl(load.as_deref(), *no_color)?;
        }
        Commands::Search {
            registry,
            query,
            jurisdiction,
            tags,
            limit,
        } => {
            commands::handle_search(registry, query, jurisdiction.as_deref(), tags, *limit)?;
        }
        Commands::Publish {
            input,
            registry,
            tags,
            dry_run,
        } => {
            commands::handle_publish(input, registry, tags, *dry_run)?;
        }
        Commands::Validate {
            input,
            format,
            strict,
        } => {
            commands::handle_validate(input, format.as_ref(), *strict)?;
        }
        Commands::Install {
            statute_id,
            registry,
            output,
            force,
        } => {
            commands::handle_install(statute_id, registry, output, *force)?;
        }
        Commands::List { directory, verbose } => {
            commands::handle_list(directory, *verbose)?;
        }
        Commands::Add {
            statute_id,
            registry,
            config,
        } => {
            commands::handle_add(statute_id, registry, config)?;
        }
        Commands::Update {
            statute_id,
            registry,
            dry_run,
        } => {
            commands::handle_update(statute_id.as_deref(), registry, *dry_run)?;
        }
        Commands::Clean {
            all,
            cache,
            temp,
            dry_run,
        } => {
            commands::handle_clean(*all, *cache, *temp, *dry_run)?;
        }
        Commands::Outdated {
            directory,
            registry,
            all,
        } => {
            commands::handle_outdated(directory, registry, *all)?;
        }
        Commands::Uninstall {
            statute_id,
            directory,
            force,
            dry_run,
        } => {
            commands::handle_uninstall(statute_id, directory, *force, *dry_run)?;
        }
        Commands::Tutorial { topic } => {
            let tutorial_topic = topic.as_ref().map(|t| t.clone().into());
            legalis_cli::tutorial::run_tutorial(tutorial_topic)?;
        }
        Commands::Explain {
            input,
            detail,
            output,
        } => {
            commands::handle_explain(input, detail, output.as_deref())?;
        }
        Commands::Trace {
            input,
            test_case,
            trace_format,
            output,
        } => {
            commands::handle_trace(input, test_case, trace_format, output.as_deref())?;
        }
        Commands::Benchmark {
            input,
            bench_type,
            iterations,
            population,
            output,
        } => {
            commands::handle_benchmark(
                input,
                bench_type,
                *iterations,
                *population,
                output.as_deref(),
            )
            .await?;
        }
        Commands::Migrate {
            input,
            from_version,
            to_version,
            output,
            dry_run,
        } => {
            commands::handle_migrate(input, from_version, to_version, output.as_deref(), *dry_run)?;
        }
        Commands::Graph {
            input,
            graph_type,
            output,
            graph_format,
        } => {
            commands::handle_graph(input, graph_type, output, graph_format)?;
        }
        Commands::BuilderWizard { help_only } => {
            commands::handle_builder_wizard(*help_only)?;
        }
        Commands::DiffViewer { old, new } => {
            commands::handle_diff_viewer(old, new)?;
        }
        Commands::SimTune { input } => {
            commands::handle_sim_tune(input).await?;
        }
        Commands::ResolveConflicts { input } => {
            commands::handle_resolve_conflicts(input)?;
        }
        Commands::RegistryBrowser { registry, search } => {
            commands::handle_registry_browser(registry, *search)?;
        }
        Commands::Batch { operation } => {
            use legalis_cli::BatchOperation;
            match operation {
                BatchOperation::Verify {
                    input,
                    strict,
                    workers,
                    resume,
                    journal,
                } => {
                    commands::handle_batch_verify(input, *strict, *workers, *resume, journal)
                        .await?;
                }
                BatchOperation::Format {
                    input,
                    style,
                    inplace,
                    workers,
                    resume,
                    journal,
                } => {
                    commands::handle_batch_format(
                        input, style, *inplace, *workers, *resume, journal,
                    )
                    .await?;
                }
                BatchOperation::Lint {
                    input,
                    fix,
                    strict,
                    workers,
                    resume,
                    journal,
                } => {
                    commands::handle_batch_lint(input, *fix, *strict, *workers, *resume, journal)
                        .await?;
                }
                BatchOperation::Export {
                    input,
                    output,
                    export_format,
                    workers,
                    resume,
                    journal,
                } => {
                    commands::handle_batch_export(
                        input,
                        output,
                        export_format,
                        *workers,
                        *resume,
                        journal,
                    )
                    .await?;
                }
            }
        }
    }

    Ok(())
}
