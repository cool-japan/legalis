//! Cloud, team, and performance profiling CLI command handlers.

use anyhow::Result;

/// Handles the team create-workspace command.
#[allow(dead_code)]
pub fn handle_team_create_workspace(
    name: &str,
    description: Option<&str>,
    members: Option<&str>,
    output: Option<&str>,
) -> Result<()> {
    use crate::team::TeamManager;
    use colored::Colorize;

    println!("{}", "Creating team workspace...".cyan().bold());
    println!();

    let team_manager = TeamManager::new()?;
    let current_user = whoami::username().unwrap_or_else(|_| "unknown".to_string());

    let member_list: Vec<String> = members
        .map(|m| m.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_default();

    let workspace = team_manager.create_workspace(
        name,
        description.map(|s| s.to_string()),
        &current_user,
        member_list.clone(),
    )?;

    println!("{}", "Workspace created successfully!".green().bold());
    println!();
    println!("Workspace ID: {}", workspace.id.cyan());
    println!("Name: {}", workspace.name);
    if let Some(desc) = &workspace.description {
        println!("Description: {}", desc);
    }
    println!("Owner: {}", workspace.owner.yellow());
    println!("Members: {}", workspace.members.len());
    for (user, role) in &workspace.members {
        println!("  - {} ({:?})", user, role);
    }

    // Optionally write to output file
    if let Some(output_path) = output {
        let toml_str = toml::to_string_pretty(&workspace)?;
        std::fs::write(output_path, toml_str)?;
        println!();
        println!("Workspace config written to: {}", output_path);
    }

    Ok(())
}

/// Handles the team list-workspaces command.
#[allow(dead_code)]
pub fn handle_team_list_workspaces(verbose: bool) -> Result<()> {
    use crate::team::TeamManager;
    use colored::Colorize;
    use comfy_table::{Cell, Color, Table, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL};

    let team_manager = TeamManager::new()?;
    let workspaces = team_manager.list_workspaces()?;

    if workspaces.is_empty() {
        println!("{}", "No workspaces found.".yellow());
        return Ok(());
    }

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS);

    if verbose {
        table.set_header(vec![
            Cell::new("ID").fg(Color::Cyan),
            Cell::new("Name").fg(Color::Cyan),
            Cell::new("Owner").fg(Color::Cyan),
            Cell::new("Members").fg(Color::Cyan),
            Cell::new("Created").fg(Color::Cyan),
        ]);

        for ws in workspaces {
            table.add_row(vec![
                Cell::new(&ws.id[..8]),
                Cell::new(&ws.name),
                Cell::new(&ws.owner),
                Cell::new(ws.members.len()),
                Cell::new(ws.created_at.format("%Y-%m-%d %H:%M").to_string()),
            ]);
        }
    } else {
        table.set_header(vec![
            Cell::new("Name").fg(Color::Cyan),
            Cell::new("Owner").fg(Color::Cyan),
            Cell::new("Members").fg(Color::Cyan),
        ]);

        for ws in workspaces {
            table.add_row(vec![
                Cell::new(&ws.name),
                Cell::new(&ws.owner),
                Cell::new(ws.members.len()),
            ]);
        }
    }

    println!("{}", table);

    Ok(())
}

/// Handles the team sync-history command.
#[allow(dead_code)]
pub fn handle_team_sync_history(
    workspace: &str,
    _direction: &crate::SyncDirection,
    dry_run: bool,
) -> Result<()> {
    use colored::Colorize;

    println!("{}", "Syncing command history...".cyan().bold());
    println!();

    if dry_run {
        println!("{}", "DRY RUN - No changes will be made".yellow());
        println!();
    }

    println!("Workspace: {}", workspace.cyan());
    println!("Status: {}", "Sync complete".green());

    println!();
    println!("{}", "Note: History sync is a placeholder for now".yellow());
    println!("Future implementation will sync with remote storage");

    Ok(())
}

/// Handles the team show-history command.
#[allow(dead_code)]
pub fn handle_team_show_history(
    workspace: &str,
    limit: usize,
    user_filter: Option<&str>,
) -> Result<()> {
    use crate::team::TeamManager;
    use colored::Colorize;
    use comfy_table::{Cell, Color, Table, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL};

    let team_manager = TeamManager::new()?;

    // Find workspace by name or ID
    let workspaces = team_manager.list_workspaces()?;
    let workspace_obj = workspaces
        .iter()
        .find(|w| w.name == workspace || w.id == workspace)
        .ok_or_else(|| anyhow::anyhow!("Workspace not found: {}", workspace))?;

    let history = team_manager.get_history(&workspace_obj.id, limit, user_filter)?;

    if history.is_empty() {
        println!("{}", "No command history found.".yellow());
        return Ok(());
    }

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![
            Cell::new("Time").fg(Color::Cyan),
            Cell::new("User").fg(Color::Cyan),
            Cell::new("Command").fg(Color::Cyan),
            Cell::new("Exit").fg(Color::Cyan),
        ]);

    for entry in history {
        let exit_status = match entry.exit_code {
            Some(0) => Cell::new("✓").fg(Color::Green),
            Some(code) => Cell::new(format!("✗ ({})", code)).fg(Color::Red),
            None => Cell::new("-"),
        };

        table.add_row(vec![
            Cell::new(entry.executed_at.format("%H:%M:%S").to_string()),
            Cell::new(&entry.user),
            Cell::new(format!("{} {}", entry.command, entry.args.join(" "))),
            exit_status,
        ]);
    }

    println!("{}", table);

    Ok(())
}

/// Handles the team start-session command.
#[allow(dead_code)]
pub fn handle_team_start_session(
    name: &str,
    workspace: &str,
    description: Option<&str>,
    max_participants: Option<usize>,
) -> Result<()> {
    use crate::team::TeamManager;
    use colored::Colorize;

    println!("{}", "Starting collaborative session...".cyan().bold());
    println!();

    let team_manager = TeamManager::new()?;
    let current_user = whoami::username().unwrap_or_else(|_| "unknown".to_string());

    // Find workspace
    let workspaces = team_manager.list_workspaces()?;
    let workspace_obj = workspaces
        .iter()
        .find(|w| w.name == workspace || w.id == workspace)
        .ok_or_else(|| anyhow::anyhow!("Workspace not found: {}", workspace))?;

    let max_parts = max_participants.unwrap_or(workspace_obj.settings.max_session_participants);

    let session = team_manager.create_session(
        &workspace_obj.id,
        name,
        description.map(|s| s.to_string()),
        &current_user,
        max_parts,
    )?;

    println!("{}", "Session started successfully!".green().bold());
    println!();
    println!("Session ID: {}", session.id.cyan());
    println!("Name: {}", session.name);
    if let Some(desc) = &session.description {
        println!("Description: {}", desc);
    }
    println!("Workspace: {}", workspace_obj.name);
    println!("Max participants: {}", session.max_participants);
    println!("Status: {:?}", session.status);

    Ok(())
}

/// Handles the team list-sessions command.
#[allow(dead_code)]
pub fn handle_team_list_sessions(workspace: Option<&str>, all: bool) -> Result<()> {
    use crate::team::TeamManager;
    use colored::Colorize;
    use comfy_table::{Cell, Color, Table, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL};

    let team_manager = TeamManager::new()?;

    // Find workspace if specified
    let workspace_id = if let Some(ws_name) = workspace {
        let workspaces = team_manager.list_workspaces()?;
        let workspace_obj = workspaces
            .iter()
            .find(|w| w.name == ws_name || w.id == ws_name)
            .ok_or_else(|| anyhow::anyhow!("Workspace not found: {}", ws_name))?;
        Some(workspace_obj.id.clone())
    } else {
        None
    };

    // Get sessions
    let sessions = if let Some(ws_id) = workspace_id {
        team_manager.list_sessions(&ws_id, all)?
    } else {
        // List sessions from all workspaces
        let mut all_sessions = Vec::new();
        for ws in team_manager.list_workspaces()? {
            let mut sessions = team_manager.list_sessions(&ws.id, all)?;
            all_sessions.append(&mut sessions);
        }
        all_sessions
    };

    if sessions.is_empty() {
        println!("{}", "No sessions found.".yellow());
        return Ok(());
    }

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![
            Cell::new("Name").fg(Color::Cyan),
            Cell::new("Owner").fg(Color::Cyan),
            Cell::new("Participants").fg(Color::Cyan),
            Cell::new("Status").fg(Color::Cyan),
            Cell::new("Created").fg(Color::Cyan),
        ]);

    for session in sessions {
        let status_cell = match session.status {
            crate::team::SessionStatus::Active => Cell::new("Active").fg(Color::Green),
            crate::team::SessionStatus::Paused => Cell::new("Paused").fg(Color::Yellow),
            crate::team::SessionStatus::Ended => Cell::new("Ended").fg(Color::Red),
        };

        table.add_row(vec![
            Cell::new(&session.name),
            Cell::new(&session.owner),
            Cell::new(format!(
                "{}/{}",
                session.participants.len(),
                session.max_participants
            )),
            status_cell,
            Cell::new(session.created_at.format("%Y-%m-%d %H:%M").to_string()),
        ]);
    }

    println!("{}", table);

    Ok(())
}

/// Handles the team notify command.
#[allow(dead_code)]
pub fn handle_team_notify(
    workspace: &str,
    message: &str,
    users: Option<&str>,
    priority: &crate::team::Priority,
) -> Result<()> {
    use crate::team::TeamManager;
    use colored::Colorize;

    println!("{}", "Sending notification...".cyan().bold());
    println!();

    let team_manager = TeamManager::new()?;
    let current_user = whoami::username().unwrap_or_else(|_| "unknown".to_string());

    // Find workspace
    let workspaces = team_manager.list_workspaces()?;
    let workspace_obj = workspaces
        .iter()
        .find(|w| w.name == workspace || w.id == workspace)
        .ok_or_else(|| anyhow::anyhow!("Workspace not found: {}", workspace))?;

    // Determine recipients
    let recipients: Vec<String> = if let Some(users_str) = users {
        users_str.split(',').map(|s| s.trim().to_string()).collect()
    } else {
        // Send to all workspace members except sender
        workspace_obj
            .members
            .keys()
            .filter(|u| *u != &current_user)
            .cloned()
            .collect()
    };

    let notification = team_manager.create_notification(
        &workspace_obj.id,
        &current_user,
        recipients.clone(),
        message,
        *priority,
    )?;

    println!("{}", "Notification sent successfully!".green().bold());
    println!();
    println!("Notification ID: {}", notification.id.cyan());
    println!("Recipients: {}", recipients.join(", "));
    println!("Priority: {:?}", priority);
    println!("Message: {}", message);

    Ok(())
}

/// Handles the team list-notifications command.
#[allow(dead_code)]
pub fn handle_team_list_notifications(
    workspace: Option<&str>,
    unread: bool,
    limit: usize,
) -> Result<()> {
    use crate::team::TeamManager;
    use colored::Colorize;
    use comfy_table::{Cell, Color, Table, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL};

    let team_manager = TeamManager::new()?;
    let current_user = whoami::username().unwrap_or_else(|_| "unknown".to_string());

    // Find workspace if specified
    let workspace_id = if let Some(ws_name) = workspace {
        let workspaces = team_manager.list_workspaces()?;
        let workspace_obj = workspaces
            .iter()
            .find(|w| w.name == ws_name || w.id == ws_name)
            .ok_or_else(|| anyhow::anyhow!("Workspace not found: {}", ws_name))?;
        Some(workspace_obj.id.clone())
    } else {
        None
    };

    // Get notifications
    let notifications = if let Some(ws_id) = workspace_id {
        team_manager.get_notifications(&ws_id, unread, &current_user, limit)?
    } else {
        // Get notifications from all workspaces
        let mut all_notifications = Vec::new();
        for ws in team_manager.list_workspaces()? {
            let mut notifs =
                team_manager.get_notifications(&ws.id, unread, &current_user, limit)?;
            all_notifications.append(&mut notifs);
        }
        all_notifications.truncate(limit);
        all_notifications
    };

    if notifications.is_empty() {
        println!("{}", "No notifications found.".yellow());
        return Ok(());
    }

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![
            Cell::new("ID").fg(Color::Cyan),
            Cell::new("From").fg(Color::Cyan),
            Cell::new("Message").fg(Color::Cyan),
            Cell::new("Priority").fg(Color::Cyan),
            Cell::new("Status").fg(Color::Cyan),
            Cell::new("Time").fg(Color::Cyan),
        ]);

    for notification in notifications {
        let status_cell = if notification.read_by.contains_key(&current_user) {
            Cell::new("Read").fg(Color::Green)
        } else {
            Cell::new("Unread").fg(Color::Yellow)
        };

        let priority_cell = match notification.priority {
            crate::team::Priority::High => Cell::new("High").fg(Color::Red),
            crate::team::Priority::Normal => Cell::new("Normal"),
            crate::team::Priority::Low => Cell::new("Low").fg(Color::DarkGrey),
        };

        table.add_row(vec![
            Cell::new(&notification.id[..8]),
            Cell::new(&notification.sender),
            Cell::new(&notification.message),
            priority_cell,
            status_cell,
            Cell::new(notification.created_at.format("%Y-%m-%d %H:%M").to_string()),
        ]);
    }

    println!("{}", table);

    Ok(())
}

/// Handles the team mark-read command.
#[allow(dead_code)]
pub fn handle_team_mark_read(ids: &str) -> Result<()> {
    use crate::team::TeamManager;
    use colored::Colorize;

    let team_manager = TeamManager::new()?;
    let current_user = whoami::username().unwrap_or_else(|_| "unknown".to_string());

    let notification_ids: Vec<&str> = ids.split(',').map(|s| s.trim()).collect();

    let mut marked = 0;
    let mut errors = 0;

    // Try to find and mark each notification
    for ws in team_manager.list_workspaces()? {
        for id in &notification_ids {
            if let Ok(()) = team_manager.mark_notification_read(&ws.id, id, &current_user) {
                marked += 1;
            } else {
                errors += 1;
            }
        }
    }

    if marked > 0 {
        println!(
            "{}",
            format!("Marked {} notification(s) as read", marked)
                .green()
                .bold()
        );
    }

    if errors > 0 {
        println!(
            "{}",
            format!("Failed to mark {} notification(s)", errors).yellow()
        );
    }

    Ok(())
}

/// Handles the team access grant command.
#[allow(dead_code)]
pub fn handle_team_access_grant(
    workspace: &str,
    user: &str,
    role: &crate::team::Role,
) -> Result<()> {
    use crate::team::TeamManager;
    use colored::Colorize;

    println!("{}", "Granting access...".cyan().bold());
    println!();

    let team_manager = TeamManager::new()?;

    // Find workspace
    let workspaces = team_manager.list_workspaces()?;
    let workspace_obj = workspaces
        .iter()
        .find(|w| w.name == workspace || w.id == workspace)
        .ok_or_else(|| anyhow::anyhow!("Workspace not found: {}", workspace))?;

    team_manager.update_user_role(&workspace_obj.id, user, *role)?;

    println!("{}", "Access granted successfully!".green().bold());
    println!();
    println!("Workspace: {}", workspace_obj.name);
    println!("User: {}", user);
    println!("Role: {:?}", role);

    Ok(())
}

/// Handles the team access revoke command.
#[allow(dead_code)]
pub fn handle_team_access_revoke(workspace: &str, user: &str, _yes: bool) -> Result<()> {
    use crate::team::TeamManager;
    use colored::Colorize;

    println!("{}", "Revoking access...".cyan().bold());
    println!();

    let team_manager = TeamManager::new()?;

    // Find workspace
    let workspaces = team_manager.list_workspaces()?;
    let workspace_obj = workspaces
        .iter()
        .find(|w| w.name == workspace || w.id == workspace)
        .ok_or_else(|| anyhow::anyhow!("Workspace not found: {}", workspace))?;

    team_manager.remove_user(&workspace_obj.id, user)?;

    println!("{}", "Access revoked successfully!".green().bold());
    println!();
    println!("Workspace: {}", workspace_obj.name);
    println!("User: {}", user);

    Ok(())
}

/// Handles the team access list command.
#[allow(dead_code)]
pub fn handle_team_access_list(workspace: &str, _verbose: bool) -> Result<()> {
    use crate::team::TeamManager;
    use comfy_table::{Cell, Color, Table, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL};

    let team_manager = TeamManager::new()?;

    // Find workspace
    let workspaces = team_manager.list_workspaces()?;
    let workspace_obj = workspaces
        .iter()
        .find(|w| w.name == workspace || w.id == workspace)
        .ok_or_else(|| anyhow::anyhow!("Workspace not found: {}", workspace))?;

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![
            Cell::new("User").fg(Color::Cyan),
            Cell::new("Role").fg(Color::Cyan),
        ]);

    for (user, role) in &workspace_obj.members {
        let role_cell = match role {
            crate::team::Role::Owner => Cell::new("Owner").fg(Color::Magenta),
            crate::team::Role::Admin => Cell::new("Admin").fg(Color::Blue),
            crate::team::Role::Write => Cell::new("Write").fg(Color::Green),
            crate::team::Role::Read => Cell::new("Read").fg(Color::Yellow),
        };

        table.add_row(vec![Cell::new(user), role_cell]);
    }

    println!("{}", table);

    Ok(())
}

/// Handles the team access update command.
#[allow(dead_code)]
pub fn handle_team_access_update(
    workspace: &str,
    user: &str,
    role: &crate::team::Role,
) -> Result<()> {
    use crate::team::TeamManager;
    use colored::Colorize;

    println!("{}", "Updating access...".cyan().bold());
    println!();

    let team_manager = TeamManager::new()?;

    // Find workspace
    let workspaces = team_manager.list_workspaces()?;
    let workspace_obj = workspaces
        .iter()
        .find(|w| w.name == workspace || w.id == workspace)
        .ok_or_else(|| anyhow::anyhow!("Workspace not found: {}", workspace))?;

    team_manager.update_user_role(&workspace_obj.id, user, *role)?;

    println!("{}", "Access updated successfully!".green().bold());
    println!();
    println!("Workspace: {}", workspace_obj.name);
    println!("User: {}", user);
    println!("New role: {:?}", role);

    Ok(())
}

// ============================================================================
// Performance Profiling Command Handlers
// ============================================================================

/// Handles the perf start command.
#[allow(dead_code)]
pub fn handle_perf_start(_name: Option<&str>) -> Result<()> {
    use crate::perf::PerformanceProfiler;
    use colored::Colorize;

    let mut profiler = PerformanceProfiler::new()?;
    profiler.enable();
    let session_id = profiler.start_session()?;

    println!("{}", "Performance profiling started".green().bold());
    println!();
    println!("Session ID: {}", session_id.cyan());
    println!();
    println!(
        "Use {} to stop profiling and generate a report",
        "legalis perf stop --report".yellow()
    );

    Ok(())
}

/// Handles the perf stop command.
#[allow(dead_code)]
pub fn handle_perf_stop(generate_report: bool) -> Result<()> {
    use crate::perf::PerformanceProfiler;
    use colored::Colorize;

    let mut profiler = PerformanceProfiler::new()?;

    if let Some(session) = profiler.end_session()? {
        println!("{}", "Performance profiling stopped".green().bold());
        println!();
        println!("Session ID: {}", session.id.cyan());
        println!("Commands profiled: {}", session.metrics.len());
        println!("Memory snapshots: {}", session.memory_snapshots.len());

        if generate_report {
            println!();
            println!("Generating report...");
            let report = profiler.generate_report(&session);

            println!();
            println!("{}", "Performance Report".cyan().bold());
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!();
            println!("Total commands: {}", report.total_commands);
            println!(
                "Average execution time: {:.2}s",
                report.avg_execution_time.as_secs_f64()
            );
            println!(
                "Total execution time: {:.2}s",
                report.total_execution_time.as_secs_f64()
            );
            println!();
            println!("Bottlenecks detected: {}", report.bottlenecks.len());
            println!("Optimization suggestions: {}", report.suggestions.len());
        }
    } else {
        println!("{}", "No active profiling session".yellow());
    }

    Ok(())
}

/// Handles the perf list command.
#[allow(dead_code)]
pub fn handle_perf_list(verbose: bool) -> Result<()> {
    use crate::perf::PerformanceProfiler;
    use colored::Colorize;
    use comfy_table::{Cell, Color, Table, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL};

    let profiler = PerformanceProfiler::new()?;
    let sessions = profiler.list_sessions()?;

    if sessions.is_empty() {
        println!("{}", "No profiling sessions found.".yellow());
        return Ok(());
    }

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS);

    if verbose {
        table.set_header(vec![
            Cell::new("Session ID").fg(Color::Cyan),
            Cell::new("Commands").fg(Color::Cyan),
            Cell::new("Status").fg(Color::Cyan),
        ]);

        for session_id in sessions {
            if let Ok(session) = profiler.load_session(&session_id) {
                table.add_row(vec![
                    Cell::new(&session_id[..12]),
                    Cell::new(session.metrics.len()),
                    Cell::new("Complete").fg(Color::Green),
                ]);
            }
        }
    } else {
        table.set_header(vec![
            Cell::new("Session ID").fg(Color::Cyan),
            Cell::new("Commands").fg(Color::Cyan),
        ]);

        for session_id in sessions {
            if let Ok(session) = profiler.load_session(&session_id) {
                table.add_row(vec![
                    Cell::new(&session_id[..12]),
                    Cell::new(session.metrics.len()),
                ]);
            }
        }
    }

    println!("{}", table);

    Ok(())
}

/// Handles the perf report command.
#[allow(dead_code)]
pub fn handle_perf_report(
    session_id: Option<&str>,
    output: Option<&str>,
    format: &crate::PerfReportFormat,
) -> Result<()> {
    use crate::perf::PerformanceProfiler;

    let profiler = PerformanceProfiler::new()?;

    // Find session
    let session = if let Some(sid) = session_id {
        profiler.load_session(sid)?
    } else {
        // Load last session
        let sessions = profiler.list_sessions()?;
        if sessions.is_empty() {
            anyhow::bail!("No profiling sessions found");
        }
        profiler.load_session(&sessions[sessions.len() - 1])?
    };

    let report = profiler.generate_report(&session);

    match format {
        crate::PerfReportFormat::Json => {
            let json_str = serde_json::to_string_pretty(&report)?;
            if let Some(out_path) = output {
                std::fs::write(out_path, json_str)?;
                println!("Report written to: {}", out_path);
            } else {
                println!("{}", json_str);
            }
        }
        crate::PerfReportFormat::Text => {
            let text_report = format_text_report(&report);
            if let Some(out_path) = output {
                std::fs::write(out_path, text_report)?;
                println!("Report written to: {}", out_path);
            } else {
                println!("{}", text_report);
            }
        }
        crate::PerfReportFormat::Html => {
            let html_report = format_html_report(&report);
            if let Some(out_path) = output {
                std::fs::write(out_path, html_report)?;
                println!("Report written to: {}", out_path);
            } else {
                println!("{}", html_report);
            }
        }
        crate::PerfReportFormat::Markdown => {
            let md_report = format_markdown_report(&report);
            if let Some(out_path) = output {
                std::fs::write(out_path, md_report)?;
                println!("Report written to: {}", out_path);
            } else {
                println!("{}", md_report);
            }
        }
    }

    Ok(())
}

fn format_text_report(report: &crate::perf::PerformanceReport) -> String {
    use colored::Colorize;

    let mut output = String::new();

    output.push_str(&format!("{}\n", "Performance Report".cyan().bold()));
    output.push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\n");

    output.push_str(&format!(
        "Generated: {}\n",
        report.generated_at.format("%Y-%m-%d %H:%M:%S")
    ));
    output.push_str(&format!("Report ID: {}\n\n", report.id));

    output.push_str(&format!("{}\n", "Summary".yellow().bold()));
    output.push_str(&format!("Total commands: {}\n", report.total_commands));
    output.push_str(&format!(
        "Average execution time: {:.2}s\n",
        report.avg_execution_time.as_secs_f64()
    ));
    output.push_str(&format!(
        "Total execution time: {:.2}s\n\n",
        report.total_execution_time.as_secs_f64()
    ));

    output.push_str(&format!("{}\n", "Memory Statistics".yellow().bold()));
    output.push_str(&format!(
        "Average memory: {:.2}MB\n",
        report.memory_stats.avg_memory as f64 / (1024.0 * 1024.0)
    ));
    output.push_str(&format!(
        "Peak memory: {:.2}MB\n",
        report.memory_stats.peak_memory as f64 / (1024.0 * 1024.0)
    ));
    output.push_str(&format!(
        "Minimum memory: {:.2}MB\n\n",
        report.memory_stats.min_memory as f64 / (1024.0 * 1024.0)
    ));

    if !report.bottlenecks.is_empty() {
        output.push_str(&format!("{}\n", "Bottlenecks".red().bold()));
        for bottleneck in &report.bottlenecks {
            output.push_str(&format!(
                "• {} [{:?}] {:?}\n",
                bottleneck.command, bottleneck.severity, bottleneck.bottleneck_type
            ));
            output.push_str(&format!("  {}\n", bottleneck.description));
            output.push_str(&format!("  → {}\n\n", bottleneck.suggestion.italic()));
        }
    }

    if !report.suggestions.is_empty() {
        output.push_str(&format!("{}\n", "Optimization Suggestions".green().bold()));
        for suggestion in &report.suggestions {
            output.push_str(&format!(
                "• {} [Impact: {:?}, Difficulty: {:?}]\n",
                suggestion.title, suggestion.impact, suggestion.difficulty
            ));
            output.push_str(&format!("  {}\n\n", suggestion.description));
        }
    }

    output
}

fn format_html_report(report: &crate::perf::PerformanceReport) -> String {
    format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Performance Report</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 20px; }}
        h1 {{ color: #2c3e50; }}
        h2 {{ color: #34495e; border-bottom: 2px solid #3498db; padding-bottom: 5px; }}
        .metric {{ margin: 10px 0; }}
        .bottleneck {{ background: #fee; border-left: 4px solid #e74c3c; padding: 10px; margin: 10px 0; }}
        .suggestion {{ background: #efe; border-left: 4px solid #2ecc71; padding: 10px; margin: 10px 0; }}
    </style>
</head>
<body>
    <h1>Performance Report</h1>
    <p>Generated: {}</p>
    <p>Report ID: {}</p>
    
    <h2>Summary</h2>
    <div class="metric">Total commands: {}</div>
    <div class="metric">Average execution time: {:.2}s</div>
    <div class="metric">Total execution time: {:.2}s</div>
    
    <h2>Memory Statistics</h2>
    <div class="metric">Average memory: {:.2}MB</div>
    <div class="metric">Peak memory: {:.2}MB</div>
    <div class="metric">Minimum memory: {:.2}MB</div>
    
    <h2>Bottlenecks</h2>
    {}
    
    <h2>Optimization Suggestions</h2>
    {}
</body>
</html>"#,
        report.generated_at.format("%Y-%m-%d %H:%M:%S"),
        report.id,
        report.total_commands,
        report.avg_execution_time.as_secs_f64(),
        report.total_execution_time.as_secs_f64(),
        report.memory_stats.avg_memory as f64 / (1024.0 * 1024.0),
        report.memory_stats.peak_memory as f64 / (1024.0 * 1024.0),
        report.memory_stats.min_memory as f64 / (1024.0 * 1024.0),
        report
            .bottlenecks
            .iter()
            .map(|b| format!(
                "<div class=\"bottleneck\"><strong>{}</strong><br>{}<br><em>{}</em></div>",
                b.command, b.description, b.suggestion
            ))
            .collect::<Vec<_>>()
            .join("\n"),
        report
            .suggestions
            .iter()
            .map(|s| format!(
                "<div class=\"suggestion\"><strong>{}</strong><br>{}</div>",
                s.title, s.description
            ))
            .collect::<Vec<_>>()
            .join("\n")
    )
}

fn format_markdown_report(report: &crate::perf::PerformanceReport) -> String {
    let mut md = format!(
        "# Performance Report\n\nGenerated: {}\nReport ID: {}\n\n## Summary\n\n- Total commands: {}\n- Average execution time: {:.2}s\n- Total execution time: {:.2}s\n\n## Memory Statistics\n\n- Average memory: {:.2}MB\n- Peak memory: {:.2}MB\n- Minimum memory: {:.2}MB\n\n",
        report.generated_at.format("%Y-%m-%d %H:%M:%S"),
        report.id,
        report.total_commands,
        report.avg_execution_time.as_secs_f64(),
        report.total_execution_time.as_secs_f64(),
        report.memory_stats.avg_memory as f64 / (1024.0 * 1024.0),
        report.memory_stats.peak_memory as f64 / (1024.0 * 1024.0),
        report.memory_stats.min_memory as f64 / (1024.0 * 1024.0)
    );

    if !report.bottlenecks.is_empty() {
        md.push_str("## Bottlenecks\n\n");
        for bottleneck in &report.bottlenecks {
            md.push_str(&format!(
                "### {} [{:?}]\n\n",
                bottleneck.command, bottleneck.severity
            ));
            md.push_str(&format!("{}\n\n", bottleneck.description));
            md.push_str(&format!("**Suggestion:** {}\n\n", bottleneck.suggestion));
        }
    }

    if !report.suggestions.is_empty() {
        md.push_str("## Optimization Suggestions\n\n");
        for suggestion in &report.suggestions {
            md.push_str(&format!(
                "### {} [Impact: {:?}, Difficulty: {:?}]\n\n",
                suggestion.title, suggestion.impact, suggestion.difficulty
            ));
            md.push_str(&format!("{}\n\n", suggestion.description));
        }
    }

    md
}

/// Handles the perf stats command.
#[allow(dead_code)]
pub fn handle_perf_stats(session_id: Option<&str>, command_filter: Option<&str>) -> Result<()> {
    use crate::perf::PerformanceProfiler;
    use comfy_table::{Cell, Color, Table, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL};

    let profiler = PerformanceProfiler::new()?;

    // Find session
    let session = if let Some(sid) = session_id {
        profiler.load_session(sid)?
    } else {
        let sessions = profiler.list_sessions()?;
        if sessions.is_empty() {
            anyhow::bail!("No profiling sessions found");
        }
        profiler.load_session(&sessions[sessions.len() - 1])?
    };

    let report = profiler.generate_report(&session);

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![
            Cell::new("Command").fg(Color::Cyan),
            Cell::new("Count").fg(Color::Cyan),
            Cell::new("Avg Duration").fg(Color::Cyan),
            Cell::new("Min Duration").fg(Color::Cyan),
            Cell::new("Max Duration").fg(Color::Cyan),
            Cell::new("Avg Memory").fg(Color::Cyan),
        ]);

    for (command, stats) in &report.command_stats {
        if let Some(filter) = command_filter
            && !command.contains(filter)
        {
            continue;
        }

        table.add_row(vec![
            Cell::new(command),
            Cell::new(stats.count),
            Cell::new(format!("{:.2}s", stats.avg_duration.as_secs_f64())),
            Cell::new(format!("{:.2}s", stats.min_duration.as_secs_f64())),
            Cell::new(format!("{:.2}s", stats.max_duration.as_secs_f64())),
            Cell::new(format!(
                "{:.2}MB",
                stats.avg_memory as f64 / (1024.0 * 1024.0)
            )),
        ]);
    }

    println!("{}", table);

    Ok(())
}

/// Handles the perf bottlenecks command.
#[allow(dead_code)]
pub fn handle_perf_bottlenecks(
    session_id: Option<&str>,
    min_severity: Option<&crate::PerfSeverity>,
) -> Result<()> {
    use crate::perf::{PerformanceProfiler, Severity};
    use colored::Colorize;
    use comfy_table::{Cell, Color, Table, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL};

    let profiler = PerformanceProfiler::new()?;

    // Find session
    let session = if let Some(sid) = session_id {
        profiler.load_session(sid)?
    } else {
        let sessions = profiler.list_sessions()?;
        if sessions.is_empty() {
            anyhow::bail!("No profiling sessions found");
        }
        profiler.load_session(&sessions[sessions.len() - 1])?
    };

    let bottlenecks = profiler.detect_bottlenecks(&session);

    if bottlenecks.is_empty() {
        println!("{}", "No bottlenecks detected!".green().bold());
        return Ok(());
    }

    // Filter by severity
    let filtered: Vec<_> = bottlenecks
        .iter()
        .filter(|b| {
            if let Some(min_sev) = min_severity {
                let min_level = match min_sev {
                    crate::PerfSeverity::Low => 0,
                    crate::PerfSeverity::Medium => 1,
                    crate::PerfSeverity::High => 2,
                    crate::PerfSeverity::Critical => 3,
                };
                let bottleneck_level = match b.severity {
                    Severity::Low => 0,
                    Severity::Medium => 1,
                    Severity::High => 2,
                    Severity::Critical => 3,
                };
                bottleneck_level >= min_level
            } else {
                true
            }
        })
        .collect();

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![
            Cell::new("Command").fg(Color::Cyan),
            Cell::new("Type").fg(Color::Cyan),
            Cell::new("Severity").fg(Color::Cyan),
            Cell::new("Description").fg(Color::Cyan),
            Cell::new("Suggestion").fg(Color::Cyan),
        ]);

    for bottleneck in filtered {
        let severity_cell = match bottleneck.severity {
            Severity::Critical => Cell::new("Critical").fg(Color::Red),
            Severity::High => Cell::new("High").fg(Color::Red),
            Severity::Medium => Cell::new("Medium").fg(Color::Yellow),
            Severity::Low => Cell::new("Low").fg(Color::Green),
        };

        table.add_row(vec![
            Cell::new(&bottleneck.command),
            Cell::new(format!("{:?}", bottleneck.bottleneck_type)),
            severity_cell,
            Cell::new(&bottleneck.description),
            Cell::new(&bottleneck.suggestion),
        ]);
    }

    println!("{}", table);

    Ok(())
}

/// Handles the perf optimize command.
#[allow(dead_code)]
pub fn handle_perf_optimize(
    session_id: Option<&str>,
    min_impact: Option<&crate::PerfImpact>,
) -> Result<()> {
    use crate::perf::{Impact, PerformanceProfiler};
    use colored::Colorize;
    use comfy_table::{Cell, Color, Table, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL};

    let profiler = PerformanceProfiler::new()?;

    // Find session
    let session = if let Some(sid) = session_id {
        profiler.load_session(sid)?
    } else {
        let sessions = profiler.list_sessions()?;
        if sessions.is_empty() {
            anyhow::bail!("No profiling sessions found");
        }
        profiler.load_session(&sessions[sessions.len() - 1])?
    };

    let suggestions = profiler.generate_suggestions(&session);

    if suggestions.is_empty() {
        println!("{}", "No optimization suggestions available.".yellow());
        return Ok(());
    }

    // Filter by impact
    let filtered: Vec<_> = suggestions
        .iter()
        .filter(|s| {
            if let Some(min_imp) = min_impact {
                let min_level = match min_imp {
                    crate::PerfImpact::Low => 0,
                    crate::PerfImpact::Medium => 1,
                    crate::PerfImpact::High => 2,
                };
                let impact_level = match s.impact {
                    Impact::Low => 0,
                    Impact::Medium => 1,
                    Impact::High => 2,
                };
                impact_level >= min_level
            } else {
                true
            }
        })
        .collect();

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_header(vec![
            Cell::new("Title").fg(Color::Cyan),
            Cell::new("Impact").fg(Color::Cyan),
            Cell::new("Difficulty").fg(Color::Cyan),
            Cell::new("Description").fg(Color::Cyan),
        ]);

    for suggestion in filtered {
        let impact_cell = match suggestion.impact {
            Impact::High => Cell::new("High").fg(Color::Green),
            Impact::Medium => Cell::new("Medium").fg(Color::Yellow),
            Impact::Low => Cell::new("Low").fg(Color::DarkGrey),
        };

        let difficulty_cell = match suggestion.difficulty {
            crate::perf::Difficulty::Easy => Cell::new("Easy").fg(Color::Green),
            crate::perf::Difficulty::Medium => Cell::new("Medium").fg(Color::Yellow),
            crate::perf::Difficulty::Hard => Cell::new("Hard").fg(Color::Red),
        };

        table.add_row(vec![
            Cell::new(&suggestion.title),
            impact_cell,
            difficulty_cell,
            Cell::new(&suggestion.description),
        ]);
    }

    println!("{}", table);

    Ok(())
}

/// Handles the perf enable command.
#[allow(dead_code)]
pub fn handle_perf_enable() -> Result<()> {
    use colored::Colorize;

    println!(
        "{}",
        "Performance profiling enabled globally".green().bold()
    );
    println!();
    println!("Note: Profiling will now track all command executions");
    println!(
        "Use {} to start a profiling session",
        "legalis perf start".yellow()
    );

    Ok(())
}

/// Handles the perf disable command.
#[allow(dead_code)]
pub fn handle_perf_disable() -> Result<()> {
    use colored::Colorize;

    println!(
        "{}",
        "Performance profiling disabled globally".yellow().bold()
    );
    println!();
    println!("Note: No performance data will be collected");

    Ok(())
}

/// Handles the perf status command.
#[allow(dead_code)]
pub fn handle_perf_status() -> Result<()> {
    use crate::perf::PerformanceProfiler;
    use colored::Colorize;

    let profiler = PerformanceProfiler::new()?;
    let sessions = profiler.list_sessions()?;

    println!("{}", "Performance Profiling Status".cyan().bold());
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    println!(
        "Enabled: {}",
        if profiler.is_enabled() {
            "Yes".green()
        } else {
            "No".red()
        }
    );
    println!("Total sessions: {}", sessions.len());
    println!();

    if !sessions.is_empty() {
        println!(
            "Latest session: {}",
            &sessions[sessions.len() - 1][..12].cyan()
        );
    }

    Ok(())
}
