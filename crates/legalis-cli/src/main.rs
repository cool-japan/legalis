//! Legalis CLI entry point.

use anyhow::Result;
use clap::Parser;
use legalis::{
    Cli, Commands, commands, generate_all_man_pages, generate_completions, generate_man_page,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration early to support alias expansion
    let config = legalis::config::Config::load();

    // Expand aliases in arguments
    let args: Vec<String> = std::env::args().collect();
    let expanded_args = config.expand_aliases(args);

    // Parse CLI with potentially expanded arguments
    let cli = Cli::parse_from(expanded_args);

    // Reload configuration with potentially specified config file
    let _config = if let Some(config_path) = &cli.config {
        legalis::config::Config::from_file(std::path::Path::new(config_path))?
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
                let (name, template_str, output) = legalis::interactive::interactive_new_statute()?;
                let template = match template_str.as_str() {
                    "basic" => legalis::StatuteTemplate::Basic,
                    "income" => legalis::StatuteTemplate::Income,
                    "geographic" => legalis::StatuteTemplate::Geographic,
                    "temporal" => legalis::StatuteTemplate::Temporal,
                    "complex" => legalis::StatuteTemplate::Complex,
                    _ => anyhow::bail!("Unknown template: {}", template_str),
                };
                commands::handle_new(&name, &template, output.as_deref())?;
                return Ok(());
            }
            Commands::Verify { .. } => {
                let (files, strict) = legalis::interactive::interactive_verify()?;
                commands::handle_verify(&files, strict, &cli.format)?;
                return Ok(());
            }
            Commands::Export { .. } => {
                let (input, output, format_str) = legalis::interactive::interactive_export()?;
                let export_format = match format_str.as_str() {
                    "json" => legalis::ExportFormat::Json,
                    "yaml" => legalis::ExportFormat::Yaml,
                    "solidity" => legalis::ExportFormat::Solidity,
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
            legalis::tutorial::run_tutorial(tutorial_topic)?;
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
        Commands::Profile {
            input,
            profile_type,
            iterations,
            output,
            flamegraph,
            #[cfg(target_os = "linux")]
            flamegraph_dir,
        } => {
            commands::handle_profile(
                input,
                profile_type,
                *iterations,
                output.as_deref(),
                *flamegraph,
                #[cfg(target_os = "linux")]
                flamegraph_dir,
                &cli.format,
            )?;
        }
        Commands::Debug {
            input,
            test_case,
            interactive,
            show_memory,
            show_timing,
            output,
        } => {
            commands::handle_debug(
                input,
                test_case,
                *interactive,
                *show_memory,
                *show_timing,
                output.as_deref(),
                &cli.format,
            )?;
        }
        Commands::Registry { operation } => {
            use legalis::RegistryOperation;
            match operation {
                RegistryOperation::Push {
                    input,
                    registry,
                    tags,
                    visibility,
                    dry_run,
                    force,
                } => {
                    commands::handle_registry_push(
                        input,
                        registry.as_deref(),
                        tags,
                        visibility,
                        *dry_run,
                        *force,
                    )?;
                }
                RegistryOperation::Pull {
                    statute_id,
                    registry,
                    output,
                    version,
                    force,
                } => {
                    commands::handle_registry_pull(
                        statute_id,
                        registry.as_deref(),
                        output,
                        version.as_deref(),
                        *force,
                    )?;
                }
                RegistryOperation::Diff {
                    local,
                    statute_id,
                    registry,
                    diff_format,
                    output,
                } => {
                    commands::handle_registry_diff(
                        local,
                        statute_id.as_deref(),
                        registry.as_deref(),
                        diff_format,
                        output.as_deref(),
                    )?;
                }
                RegistryOperation::Sync {
                    directory,
                    registry,
                    direction,
                    conflict,
                    dry_run,
                } => {
                    commands::handle_registry_sync(
                        directory,
                        registry.as_deref(),
                        direction,
                        conflict,
                        *dry_run,
                    )?;
                }
                RegistryOperation::Login {
                    registry,
                    username,
                    password,
                    token,
                } => {
                    commands::handle_registry_login(
                        registry,
                        username.as_deref(),
                        password.as_deref(),
                        token.as_deref(),
                    )?;
                }
                RegistryOperation::Logout { registry, all } => {
                    commands::handle_registry_logout(registry.as_deref(), *all)?;
                }
            }
        }
        Commands::Batch { operation } => {
            use legalis::BatchOperation;
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
        Commands::Plugin { operation } => {
            use legalis::PluginOperation;
            match operation {
                PluginOperation::Install { source, force } => {
                    commands::handle_plugin_install(source, *force)?;
                }
                PluginOperation::Uninstall { name, yes } => {
                    commands::handle_plugin_uninstall(name, *yes)?;
                }
                PluginOperation::List {
                    verbose,
                    plugin_type,
                } => {
                    let ptype = plugin_type.as_ref().map(|pt| pt.clone().into());
                    commands::handle_plugin_list(*verbose, ptype.as_ref())?;
                }
                PluginOperation::Info { name } => {
                    commands::handle_plugin_info(name)?;
                }
                PluginOperation::Enable { name } => {
                    commands::handle_plugin_enable(name)?;
                }
                PluginOperation::Disable { name } => {
                    commands::handle_plugin_disable(name)?;
                }
                PluginOperation::Update { name } => {
                    commands::handle_plugin_update(name.as_deref())?;
                }
            }
        }
        Commands::Config { operation } => {
            use legalis::ConfigOperation;
            match operation {
                ConfigOperation::Validate { config, verbose } => {
                    commands::handle_config_validate(config.as_deref(), *verbose)?;
                }
                ConfigOperation::Diff {
                    config1,
                    config2,
                    profile,
                } => {
                    commands::handle_config_diff(config1, config2, *profile)?;
                }
                ConfigOperation::Profiles { config } => {
                    commands::handle_config_profiles(config.as_deref())?;
                }
                ConfigOperation::Activate { profile, config } => {
                    commands::handle_config_activate(profile, config.as_deref())?;
                }
                ConfigOperation::Show {
                    config,
                    profile,
                    format,
                } => {
                    commands::handle_config_show(config.as_deref(), profile.as_deref(), format)?;
                }
                ConfigOperation::Init { force } => {
                    commands::handle_config_init(*force)?;
                }
            }
        }
        Commands::Script { operation } => {
            use legalis::ScriptOperation;
            match operation {
                ScriptOperation::Run {
                    script,
                    args,
                    debug,
                } => {
                    commands::handle_script_run(script, args, *debug)?;
                }
                ScriptOperation::List { verbose } => {
                    commands::handle_script_list(*verbose)?;
                }
                ScriptOperation::Info { name } => {
                    commands::handle_script_info(name)?;
                }
                ScriptOperation::Install { source } => {
                    commands::handle_script_install(source)?;
                }
                ScriptOperation::Uninstall { name, yes } => {
                    commands::handle_script_uninstall(name, *yes)?;
                }
                ScriptOperation::New {
                    name,
                    template,
                    output,
                } => {
                    commands::handle_script_new(name, template, output.as_deref())?;
                }
                ScriptOperation::Builtin { show_code } => {
                    commands::handle_script_builtin(*show_code)?;
                }
                ScriptOperation::Validate { script } => {
                    commands::handle_script_validate(script)?;
                }
            }
        }
        Commands::Ai { operation } => {
            use legalis::AiOperation;
            match operation {
                AiOperation::Parse { input } => {
                    commands::handle_ai_parse(input)?;
                }
                AiOperation::Intent { query } => {
                    commands::handle_ai_intent(query)?;
                }
                AiOperation::Assist { query } => {
                    commands::handle_ai_help(query)?;
                }
                AiOperation::Suggest { previous } => {
                    commands::handle_ai_suggest(previous.as_deref())?;
                }
                AiOperation::Complete { input } => {
                    commands::handle_ai_complete(input)?;
                }
            }
        }
        Commands::Dashboard { vim_keys, no_mouse } => {
            commands::handle_dashboard(*vim_keys, *no_mouse)?;
        }
        Commands::Workflow { operation } => {
            use legalis::WorkflowOperation;
            match operation {
                WorkflowOperation::Run {
                    file,
                    vars,
                    dry_run,
                    continue_on_error,
                } => {
                    commands::handle_workflow_run(file, vars, *dry_run, *continue_on_error)?;
                }
                WorkflowOperation::ListTemplates { verbose } => {
                    commands::handle_workflow_list_templates(*verbose)?;
                }
                WorkflowOperation::New {
                    template,
                    output,
                    vars,
                } => {
                    commands::handle_workflow_new(template, output, vars)?;
                }
                WorkflowOperation::Validate { file, verbose } => {
                    commands::handle_workflow_validate(file, *verbose)?;
                }
                WorkflowOperation::Info { file } => {
                    commands::handle_workflow_info(file)?;
                }
            }
        }
        Commands::Cloud { operation } => {
            use legalis::CloudOperation;
            match operation {
                CloudOperation::Status => {
                    commands::handle_cloud_status()?;
                }
                CloudOperation::Aws {
                    args,
                    profile,
                    region,
                } => {
                    commands::handle_cloud_aws(args, profile.as_deref(), region.as_deref())?;
                }
                CloudOperation::Azure { args, subscription } => {
                    commands::handle_cloud_azure(args, subscription.as_deref())?;
                }
                CloudOperation::Gcp {
                    args,
                    project,
                    zone,
                } => {
                    commands::handle_cloud_gcp(args, project.as_deref(), zone.as_deref())?;
                }
                CloudOperation::Provision {
                    file,
                    provider,
                    dry_run,
                } => {
                    let cloud_provider = provider.clone().into();
                    commands::handle_cloud_provision(file, &cloud_provider, *dry_run)?;
                }
                CloudOperation::List {
                    provider,
                    resource_type,
                    profile,
                    region,
                    subscription,
                    project,
                } => {
                    let cloud_provider = provider.clone().into();
                    commands::handle_cloud_list(
                        &cloud_provider,
                        resource_type,
                        profile.as_deref(),
                        region.as_deref(),
                        subscription.as_deref(),
                        project.as_deref(),
                    )?;
                }
                CloudOperation::Configure { provider, config } => {
                    let cloud_provider = provider.clone().into();
                    commands::handle_cloud_configure(&cloud_provider, config)?;
                }
            }
        }
        Commands::Team { operation } => {
            use legalis::{AccessOperation, TeamOperation};
            match operation {
                TeamOperation::CreateWorkspace {
                    name,
                    description,
                    members,
                    output,
                } => {
                    commands::handle_team_create_workspace(
                        name,
                        description.as_deref(),
                        members.as_deref(),
                        output.as_deref(),
                    )?;
                }
                TeamOperation::ListWorkspaces { verbose } => {
                    commands::handle_team_list_workspaces(*verbose)?;
                }
                TeamOperation::JoinWorkspace { workspace, token } => {
                    println!("Joining workspace: {}", workspace);
                    if let Some(t) = token {
                        println!("Using token: {}", t);
                    }
                    println!("Note: Join workspace is a placeholder for now");
                }
                TeamOperation::LeaveWorkspace { workspace, yes } => {
                    println!("Leaving workspace: {}", workspace);
                    if !yes {
                        println!("Use --yes to confirm");
                    }
                    println!("Note: Leave workspace is a placeholder for now");
                }
                TeamOperation::SyncHistory {
                    workspace,
                    direction,
                    dry_run,
                } => {
                    commands::handle_team_sync_history(workspace, direction, *dry_run)?;
                }
                TeamOperation::ShowHistory {
                    workspace,
                    limit,
                    user,
                } => {
                    commands::handle_team_show_history(workspace, *limit, user.as_deref())?;
                }
                TeamOperation::StartSession {
                    name,
                    workspace,
                    description,
                    max_participants,
                } => {
                    commands::handle_team_start_session(
                        name,
                        workspace,
                        description.as_deref(),
                        *max_participants,
                    )?;
                }
                TeamOperation::JoinSession { session, readonly } => {
                    println!("Joining session: {}", session);
                    println!("Read-only: {}", readonly);
                    println!("Note: Join session is a placeholder for now");
                }
                TeamOperation::LeaveSession { session } => {
                    println!("Leaving session: {}", session);
                    println!("Note: Leave session is a placeholder for now");
                }
                TeamOperation::ListSessions { workspace, all } => {
                    commands::handle_team_list_sessions(workspace.as_deref(), *all)?;
                }
                TeamOperation::Notify {
                    workspace,
                    message,
                    users,
                    priority,
                } => {
                    let priority_enum: legalis::team::Priority = match priority {
                        legalis::NotificationPriority::Low => legalis::team::Priority::Low,
                        legalis::NotificationPriority::Normal => legalis::team::Priority::Normal,
                        legalis::NotificationPriority::High => legalis::team::Priority::High,
                    };
                    commands::handle_team_notify(
                        workspace,
                        message,
                        users.as_deref(),
                        &priority_enum,
                    )?;
                }
                TeamOperation::ListNotifications {
                    workspace,
                    unread,
                    limit,
                } => {
                    commands::handle_team_list_notifications(
                        workspace.as_deref(),
                        *unread,
                        *limit,
                    )?;
                }
                TeamOperation::MarkRead { ids } => {
                    commands::handle_team_mark_read(ids)?;
                }
                TeamOperation::ManageAccess { operation } => match operation {
                    AccessOperation::Grant {
                        workspace,
                        user,
                        role,
                    } => {
                        let role_enum: legalis::team::Role = match role {
                            legalis::TeamRole::Owner => legalis::team::Role::Owner,
                            legalis::TeamRole::Admin => legalis::team::Role::Admin,
                            legalis::TeamRole::Write => legalis::team::Role::Write,
                            legalis::TeamRole::Read => legalis::team::Role::Read,
                        };
                        commands::handle_team_access_grant(workspace, user, &role_enum)?;
                    }
                    AccessOperation::Revoke {
                        workspace,
                        user,
                        yes,
                    } => {
                        commands::handle_team_access_revoke(workspace, user, *yes)?;
                    }
                    AccessOperation::List { workspace, verbose } => {
                        commands::handle_team_access_list(workspace, *verbose)?;
                    }
                    AccessOperation::Update {
                        workspace,
                        user,
                        role,
                    } => {
                        let role_enum: legalis::team::Role = match role {
                            legalis::TeamRole::Owner => legalis::team::Role::Owner,
                            legalis::TeamRole::Admin => legalis::team::Role::Admin,
                            legalis::TeamRole::Write => legalis::team::Role::Write,
                            legalis::TeamRole::Read => legalis::team::Role::Read,
                        };
                        commands::handle_team_access_update(workspace, user, &role_enum)?;
                    }
                },
            }
        }
        Commands::Perf { operation } => {
            use legalis::PerfOperation;
            match operation {
                PerfOperation::Start { name } => {
                    commands::handle_perf_start(name.as_deref())?;
                }
                PerfOperation::Stop { report } => {
                    commands::handle_perf_stop(*report)?;
                }
                PerfOperation::Record {
                    command,
                    args,
                    duration,
                    memory,
                } => {
                    println!("Recording command: {}", command);
                    println!("Args: {:?}", args);
                    println!("Duration: {}ms", duration);
                    if let Some(mem) = memory {
                        println!("Memory: {} bytes", mem);
                    }
                    println!("Note: Record is a manual operation placeholder");
                }
                PerfOperation::List { verbose } => {
                    commands::handle_perf_list(*verbose)?;
                }
                PerfOperation::Report {
                    session,
                    output,
                    format,
                } => {
                    commands::handle_perf_report(session.as_deref(), output.as_deref(), format)?;
                }
                PerfOperation::Stats { session, command } => {
                    commands::handle_perf_stats(session.as_deref(), command.as_deref())?;
                }
                PerfOperation::Bottlenecks {
                    session,
                    min_severity,
                } => {
                    commands::handle_perf_bottlenecks(session.as_deref(), min_severity.as_ref())?;
                }
                PerfOperation::Optimize {
                    session,
                    min_impact,
                } => {
                    commands::handle_perf_optimize(session.as_deref(), min_impact.as_ref())?;
                }
                PerfOperation::Enable => {
                    commands::handle_perf_enable()?;
                }
                PerfOperation::Disable => {
                    commands::handle_perf_disable()?;
                }
                PerfOperation::Status => {
                    commands::handle_perf_status()?;
                }
            }
        }
    }

    Ok(())
}
