//! Command handlers for enterprise, UX, and self-healing features.
//!
//! These handlers are thin adapters over the substantive modules
//! ([`crate::audit_log`], [`crate::policy`], [`crate::compliance`],
//! [`crate::central_config`], [`crate::suggest`], [`crate::diagnostics`]);
//! the logic and tests live in those modules.

use crate::OutputFormat;
use crate::audit_log::AuditLogger;
use crate::central_config::CentralConfig;
use crate::diagnostics::{self, CheckpointStore};
use crate::policy::{LimitKind, Policy};
use crate::suggest::{self, UsageStats};
use anyhow::{Context, Result};
use colored::Colorize;
use comfy_table::{Cell, Table, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL};

/// Emits a serializable value in the requested format, falling back to `text`.
fn emit<T: serde::Serialize>(
    value: &T,
    format: &OutputFormat,
    text: impl FnOnce() -> String,
) -> Result<()> {
    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(value)?),
        OutputFormat::Yaml => println!("{}", serde_yaml::to_string(value)?),
        _ => println!("{}", text()),
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Audit log
// ---------------------------------------------------------------------------

/// Handles `legalis audit-log show`.
pub fn handle_audit_log_show(limit: usize, format: &OutputFormat) -> Result<()> {
    let logger = AuditLogger::open()?;
    let mut records = logger.all_operations()?;
    records.sort_by_key(|record| std::cmp::Reverse(record.timestamp));
    records.truncate(limit);

    if records.is_empty() {
        println!("{}", "No audit records.".yellow());
        return Ok(());
    }

    emit(&records, format, || {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_header(vec![
                Cell::new("Timestamp"),
                Cell::new("Actor"),
                Cell::new("Command"),
                Cell::new("Outcome"),
            ]);
        for record in &records {
            let actor = match &record.actor {
                legalis_audit::Actor::User { user_id, .. } => user_id.clone(),
                legalis_audit::Actor::System { component } => format!("system:{component}"),
                legalis_audit::Actor::External { system_id } => format!("ext:{system_id}"),
            };
            let command = record
                .context
                .attributes
                .get("command")
                .cloned()
                .unwrap_or_else(|| "?".to_string());
            let outcome = record
                .context
                .attributes
                .get("outcome")
                .cloned()
                .unwrap_or_else(|| "?".to_string());
            table.add_row(vec![
                Cell::new(record.timestamp.to_rfc3339()),
                Cell::new(actor),
                Cell::new(command),
                Cell::new(outcome),
            ]);
        }
        table.to_string()
    })
}

/// Handles `legalis audit-log verify`.
pub fn handle_audit_log_verify() -> Result<()> {
    let logger = AuditLogger::open()?;
    let count = logger.count();
    if logger.verify_integrity()? {
        println!(
            "{}",
            format!("Audit log intact: {count} record(s) verified")
                .green()
                .bold()
        );
        Ok(())
    } else {
        anyhow::bail!("Audit log integrity check FAILED — possible tampering detected");
    }
}

/// Handles `legalis audit-log path`.
pub fn handle_audit_log_path() -> Result<()> {
    let logger = AuditLogger::open()?;
    println!("{}", logger.path().display());
    Ok(())
}

// ---------------------------------------------------------------------------
// Policy
// ---------------------------------------------------------------------------

/// Handles `legalis policy show`.
pub fn handle_policy_show(path: Option<&str>, format: &OutputFormat) -> Result<()> {
    let policy = Policy::discover(path)?;
    emit(&policy, format, || {
        let mut out = String::new();
        out.push_str(&format!("Policy: {}\n", policy.name.bold()));
        out.push_str(&format!("Restrictive: {}\n", policy.is_restrictive()));
        out.push_str(&format!(
            "Require compliance: {}\n",
            policy.require_compliance
        ));
        out.push_str(&format!(
            "Require audit log: {}\n",
            policy.require_audit_log
        ));
        if !policy.allowed_commands.is_empty() {
            out.push_str(&format!(
                "Allowed commands: {}\n",
                policy
                    .allowed_commands
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        if !policy.denied_commands.is_empty() {
            out.push_str(&format!(
                "Denied commands: {}\n",
                policy
                    .denied_commands
                    .iter()
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        if let Some(max) = policy.limits.max_population {
            out.push_str(&format!("Max population: {max}\n"));
        }
        if let Some(max) = policy.limits.max_workers {
            out.push_str(&format!("Max workers: {max}\n"));
        }
        if let Some(max) = policy.limits.max_iterations {
            out.push_str(&format!("Max iterations: {max}\n"));
        }
        if let Some(max) = policy.limits.max_input_files {
            out.push_str(&format!("Max input files: {max}\n"));
        }
        out
    })
}

/// Handles `legalis policy check`.
pub fn handle_policy_check(
    command: &str,
    path: Option<&str>,
    compliance_active: bool,
) -> Result<()> {
    let policy = Policy::discover(path)?;
    let decision = policy.evaluate(command, compliance_active);
    if decision.is_allowed() {
        println!(
            "{}",
            format!("Command '{command}' is ALLOWED by policy '{}'", policy.name)
                .green()
                .bold()
        );
        Ok(())
    } else {
        let reason = decision.reason().unwrap_or("denied");
        anyhow::bail!("Command '{command}' is DENIED: {reason}");
    }
}

/// Handles `legalis policy check-limit`.
pub fn handle_policy_check_limit(kind: LimitKind, value: usize, path: Option<&str>) -> Result<()> {
    let policy = Policy::discover(path)?;
    policy.check_limit(kind, value)?;
    println!(
        "{}",
        format!("Value {value} is within policy '{}' limits", policy.name)
            .green()
            .bold()
    );
    Ok(())
}

/// Handles `legalis policy init`.
pub fn handle_policy_init(output: Option<&str>, force: bool) -> Result<()> {
    let path = match output {
        Some(p) => std::path::PathBuf::from(p),
        None => Policy::default_path()?,
    };
    if path.exists() && !force {
        anyhow::bail!(
            "Policy file already exists: {} (use --force to overwrite)",
            path.display()
        );
    }
    // A sensible, restrictive-but-usable starter policy.
    let mut policy = Policy::permissive();
    policy.name = "enterprise".to_string();
    policy.require_audit_log = true;
    policy.denied_commands.insert("clean".to_string());
    policy.limits.max_population = Some(100_000);
    policy.save(&path)?;
    println!(
        "{}",
        format!("Starter policy written to: {}", path.display())
            .green()
            .bold()
    );
    Ok(())
}

// ---------------------------------------------------------------------------
// Centralized configuration
// ---------------------------------------------------------------------------

/// Handles `legalis central-config show`.
pub fn handle_central_config_show(format: &OutputFormat) -> Result<()> {
    let config = CentralConfig::discover()?;
    // Build a serializable view for machine formats.
    #[derive(serde::Serialize)]
    struct Entry {
        key: String,
        value: String,
        source: String,
    }
    let entries: Vec<Entry> = config
        .entries()
        .map(|(key, resolved)| Entry {
            key: key.clone(),
            value: resolved.value.clone(),
            source: resolved.source.to_string(),
        })
        .collect();

    emit(&entries, format, || {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_header(vec![
                Cell::new("Key"),
                Cell::new("Value"),
                Cell::new("Source"),
            ]);
        for entry in &entries {
            table.add_row(vec![
                Cell::new(&entry.key),
                Cell::new(&entry.value),
                Cell::new(&entry.source),
            ]);
        }
        table.to_string()
    })
}

/// Handles `legalis central-config validate`.
pub fn handle_central_config_validate() -> Result<()> {
    let config = CentralConfig::discover()?;
    let errors = config.validate();
    if errors.is_empty() {
        println!("{}", "Centralized configuration is valid".green().bold());
        Ok(())
    } else {
        for error in &errors {
            eprintln!("{} {}", "✗".red(), error);
        }
        anyhow::bail!("{} configuration error(s) found", errors.len());
    }
}

// ---------------------------------------------------------------------------
// Intelligent assistant
// ---------------------------------------------------------------------------

/// Handles `legalis assistant suggest`.
pub fn handle_assistant_suggest(
    previous: Option<&str>,
    limit: usize,
    format: &OutputFormat,
) -> Result<()> {
    let stats = UsageStats::load()?;
    let suggestions = stats.suggest_next(previous, limit);
    if suggestions.is_empty() {
        println!("{}", "No suggestions available yet.".yellow());
        return Ok(());
    }
    emit(&SuggestionView::from(&suggestions), format, || {
        let mut out = String::from("Suggested next commands:\n");
        for suggestion in &suggestions {
            out.push_str(&format!(
                "  {} {} ({})\n",
                "→".cyan(),
                suggestion.command.bold(),
                suggestion.reason.dimmed()
            ));
        }
        out
    })
}

/// Handles `legalis assistant recommend`.
pub fn handle_assistant_recommend(format: &OutputFormat) -> Result<()> {
    let stats = UsageStats::load()?;
    let in_project = std::path::Path::new("legalis.toml").exists()
        || std::path::Path::new("legalis.yaml").exists();
    let recommendations = suggest::recommendations(&stats, in_project);
    if recommendations.is_empty() {
        println!(
            "{}",
            "No recommendations right now — keep up the good work!".green()
        );
        return Ok(());
    }
    emit(&RecommendationView::from(&recommendations), format, || {
        let mut out = String::from("Proactive recommendations:\n");
        for rec in &recommendations {
            out.push_str(&format!("\n{} {}\n", "•".cyan(), rec.title.bold()));
            out.push_str(&format!("  {}\n", rec.detail));
        }
        out
    })
}

/// Handles `legalis assistant stats`.
pub fn handle_assistant_stats(limit: usize, format: &OutputFormat) -> Result<()> {
    let stats = UsageStats::load()?;
    let top = stats.top_commands(limit);
    #[derive(serde::Serialize)]
    struct StatsView {
        total: u64,
        distinct: usize,
        top: Vec<(String, u64)>,
        path: String,
    }
    let view = StatsView {
        total: stats.total,
        distinct: stats.distinct_commands(),
        top: top.clone(),
        path: suggest::stats_path()
            .map(|p| p.display().to_string())
            .unwrap_or_default(),
    };
    emit(&view, format, || {
        let mut out = format!(
            "Usage statistics ({} invocation(s), {} distinct command(s))\n",
            view.total, view.distinct
        );
        for (command, count) in &top {
            out.push_str(&format!("  {:>5}  {}\n", count, command));
        }
        // Only reveal the on-disk location at verbose verbosity.
        if crate::verbosity::global().shows_detail() {
            out.push_str(&format!("\nstats file: {}\n", view.path));
        }
        out
    })
}

/// Handles `legalis assistant record` (manually record an invocation, mostly
/// for testing/scripting; the dispatcher records automatically).
pub fn handle_assistant_record(command: &str) -> Result<()> {
    record_usage(command)?;
    println!("Recorded usage of '{command}'");
    Ok(())
}

// ---------------------------------------------------------------------------
// Self-diagnostics, repair, recovery
// ---------------------------------------------------------------------------

/// Handles `legalis diagnose`. Honors the global verbosity for text output:
/// quiet shows only problems; normal shows everything; verbose adds a footer.
pub fn handle_diagnose(format: &OutputFormat) -> Result<()> {
    use crate::diagnostics::CheckStatus;
    use crate::verbosity::Verbosity;

    let report = diagnostics::run_diagnostics();
    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&report)?),
        OutputFormat::Yaml => println!("{}", serde_yaml::to_string(&report)?),
        _ => {
            let verbosity = crate::verbosity::global();
            for check in &report.checks {
                // In quiet mode, only print warnings and failures.
                if verbosity <= Verbosity::Quiet && check.status == CheckStatus::Ok {
                    continue;
                }
                println!(
                    "{} {}: {}",
                    check.status.symbol(),
                    check.name,
                    check.message
                );
            }
            if verbosity.shows_status() {
                println!(
                    "\n{} check(s): {} warning(s), {} failure(s)",
                    report.checks.len(),
                    report.warnings(),
                    report.failures()
                );
            }
            if verbosity.shows_detail()
                && let Ok(dir) = crate::paths::data_dir()
            {
                println!("data directory: {}", dir.display());
            }
        }
    }
    if report.is_healthy() {
        Ok(())
    } else {
        anyhow::bail!("{} diagnostic check(s) failed", report.failures());
    }
}

/// Handles `legalis repair`.
pub fn handle_repair(config: Option<&str>, dry_run: bool) -> Result<()> {
    let path = resolve_config_path(config)?;
    let report = diagnostics::repair_config(&path, dry_run)?;
    if !report.needed_repair() {
        println!(
            "{}",
            format!("Configuration is healthy: {}", path.display()).green()
        );
        return Ok(());
    }
    let header = if dry_run {
        "Proposed repairs (dry run):".yellow().bold()
    } else {
        "Repairs applied:".green().bold()
    };
    println!("{header}");
    for repair in &report.repairs {
        println!(
            "  {} {}: {} -> {}",
            "•".cyan(),
            repair.field.bold(),
            repair.problem,
            repair.fix
        );
    }
    if dry_run {
        println!("\nRe-run without --dry-run to apply.");
    }
    Ok(())
}

/// Handles `legalis recover --list`.
pub fn handle_recover_list(format: &OutputFormat) -> Result<()> {
    let store = CheckpointStore::open()?;
    let checkpoints = store.list()?;
    if checkpoints.is_empty() {
        println!("{}", "No resumable operations.".green());
        return Ok(());
    }
    emit(&checkpoints, format, || {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_header(vec![
                Cell::new("ID"),
                Cell::new("Operation"),
                Cell::new("Progress"),
                Cell::new("Updated"),
            ]);
        for cp in &checkpoints {
            let short_id: String = cp.id.chars().take(8).collect();
            let progress = match (cp.fraction(), cp.total) {
                (Some(frac), Some(total)) => {
                    format!("{}/{} ({:.0}%)", cp.completed, total, frac * 100.0)
                }
                _ => format!("{} (unknown total)", cp.completed),
            };
            table.add_row(vec![
                Cell::new(short_id),
                Cell::new(&cp.operation),
                Cell::new(progress),
                Cell::new(&cp.updated_at),
            ]);
        }
        table.to_string()
    })
}

/// Handles `legalis recover --resume <id>`.
pub fn handle_recover_resume(id: &str) -> Result<()> {
    let store = CheckpointStore::open()?;
    let checkpoint = store.load(id).with_context(|| {
        format!("No checkpoint with id starting '{id}' (run `legalis recover --list`)")
    })?;
    println!(
        "{}",
        format!("Resuming '{}' from checkpoint {}", checkpoint.operation, id)
            .cyan()
            .bold()
    );
    if let Some(total) = checkpoint.total {
        println!(
            "Progress: {}/{} item(s) completed",
            checkpoint.completed, total
        );
    } else {
        println!("Progress: {} item(s) completed", checkpoint.completed);
    }
    if !checkpoint.payload.is_empty() {
        println!("Resume state:");
        for (key, value) in &checkpoint.payload {
            println!("  {key} = {value}");
        }
    }
    println!(
        "{}",
        "Re-run the original command; it will continue from the recorded state.".dimmed()
    );
    Ok(())
}

/// Handles `legalis recover --clean` (prune completed checkpoints).
pub fn handle_recover_clean() -> Result<()> {
    let store = CheckpointStore::open()?;
    let pruned = store.prune_completed()?;
    println!("Pruned {pruned} completed checkpoint(s)");
    Ok(())
}

// ---------------------------------------------------------------------------
// Shared helpers used by the dispatcher
// ---------------------------------------------------------------------------

/// Records a command invocation into the local usage-stats store, ignoring
/// failures (usage learning must never break the CLI).
pub fn record_usage(command: &str) -> Result<()> {
    let mut stats = UsageStats::load()?;
    stats.record(command);
    stats.save()
}

/// Best-effort usage recording that swallows all errors.
pub fn record_usage_best_effort(command: &str) {
    let _ = record_usage(command);
}

/// Records a CLI operation to the audit log, best-effort.
pub fn audit_operation_best_effort(
    command: &str,
    args: &[String],
    outcome: crate::audit_log::OperationOutcome,
    detail: Option<&str>,
) {
    if let Ok(mut logger) = AuditLogger::open() {
        let mut entry = crate::audit_log::OperationEntry::new(command, args.to_vec(), outcome);
        if let Some(detail) = detail {
            entry = entry.with_detail(detail);
        }
        let _ = logger.record(&entry);
    }
}

/// Resolves the config path to repair: explicit, else project, else user config.
fn resolve_config_path(explicit: Option<&str>) -> Result<std::path::PathBuf> {
    if let Some(path) = explicit {
        return Ok(std::path::PathBuf::from(path));
    }
    let project = std::path::PathBuf::from("legalis.toml");
    if project.exists() {
        return Ok(project);
    }
    if let Some(dir) = crate::config::Config::user_config_dir() {
        let user = dir.join("config.toml");
        if user.exists() {
            return Ok(user);
        }
    }
    anyhow::bail!(
        "No config file found to repair; pass --config <path> or create legalis.toml first"
    )
}

/// A serializable view of suggestions for JSON/YAML output.
#[derive(serde::Serialize)]
struct SuggestionView {
    suggestions: Vec<SuggestionItem>,
}

#[derive(serde::Serialize)]
struct SuggestionItem {
    command: String,
    score: u64,
    reason: String,
}

impl From<&Vec<suggest::Suggestion>> for SuggestionView {
    fn from(value: &Vec<suggest::Suggestion>) -> Self {
        Self {
            suggestions: value
                .iter()
                .map(|s| SuggestionItem {
                    command: s.command.clone(),
                    score: s.score,
                    reason: s.reason.clone(),
                })
                .collect(),
        }
    }
}

/// A serializable view of recommendations for JSON/YAML output.
#[derive(serde::Serialize)]
struct RecommendationView {
    recommendations: Vec<RecommendationItem>,
}

#[derive(serde::Serialize)]
struct RecommendationItem {
    title: String,
    detail: String,
}

impl From<&Vec<suggest::Recommendation>> for RecommendationView {
    fn from(value: &Vec<suggest::Recommendation>) -> Self {
        Self {
            recommendations: value
                .iter()
                .map(|r| RecommendationItem {
                    title: r.title.clone(),
                    detail: r.detail.clone(),
                })
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::paths;

    /// Runs `f` with the data dir redirected to a fresh temp directory.
    fn with_temp_data_dir<F: FnOnce()>(f: F) {
        let dir = std::env::temp_dir().join(format!("legalis-ent-{}", uuid::Uuid::new_v4()));
        let saved = std::env::var(paths::DATA_DIR_ENV).ok();
        unsafe {
            std::env::set_var(paths::DATA_DIR_ENV, &dir);
        }
        f();
        unsafe {
            match saved {
                Some(v) => std::env::set_var(paths::DATA_DIR_ENV, v),
                None => std::env::remove_var(paths::DATA_DIR_ENV),
            }
        }
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_record_usage_persists() {
        with_temp_data_dir(|| {
            record_usage("verify").expect("record");
            record_usage("verify").expect("record");
            let stats = UsageStats::load().expect("load");
            assert_eq!(stats.counts.get("verify"), Some(&2));
        });
    }

    #[test]
    fn test_audit_operation_best_effort_writes() {
        with_temp_data_dir(|| {
            audit_operation_best_effort(
                "verify",
                &["--input".into(), "x".into()],
                crate::audit_log::OperationOutcome::Success,
                None,
            );
            let logger = AuditLogger::open().expect("open");
            assert_eq!(logger.count(), 1);
        });
    }

    #[test]
    fn test_handle_diagnose_text_runs() {
        with_temp_data_dir(|| {
            // In a clean temp data dir this should at least run; healthiness may
            // vary, so we only assert the call does not panic and returns.
            let _ = handle_diagnose(&OutputFormat::Json);
        });
    }

    #[test]
    fn test_handle_audit_log_verify_empty_ok() {
        with_temp_data_dir(|| {
            handle_audit_log_verify().expect("empty log verifies");
        });
    }

    #[test]
    fn test_resolve_config_path_explicit() {
        let tmp = std::env::temp_dir().join("whatever.toml");
        let tmp_str = tmp.to_str().expect("valid path");
        let path = resolve_config_path(Some(tmp_str)).expect("explicit");
        assert_eq!(path, tmp);
    }

    #[test]
    fn test_policy_init_and_show() {
        with_temp_data_dir(|| {
            handle_policy_init(None, false).expect("init");
            // A second init without force should fail.
            assert!(handle_policy_init(None, false).is_err());
            // Show should succeed against the written policy.
            handle_policy_show(None, &OutputFormat::Json).expect("show");
        });
    }
}
