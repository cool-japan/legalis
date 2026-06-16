//! Command handlers for phase-level profiling and offline capabilities.
//!
//! These handlers are thin adapters over [`crate::profiling`] and
//! [`crate::offline`]; the substantive logic and tests live in those modules.

use crate::OutputFormat;
use crate::offline::{
    AlwaysOnline, CommandApplier, ConflictStrategy, ConnectivityProbe, JournalApplier,
    OfflineStore, QueueStatus, QueuedCommand, TcpProbe, validate_command,
};
use crate::profiling::Profiler;
use anyhow::{Context, Result};
use colored::Colorize;
use comfy_table::{Cell, Table, modifiers::UTF8_ROUND_CORNERS, presets::UTF8_FULL};
use legalis_dsl::LegalDslParser;
use legalis_verifier::StatuteVerifier;
use std::time::Duration;

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

/// Writes rendered output either to a file or stdout.
fn write_output(rendered: &str, output: Option<&str>) -> Result<()> {
    if let Some(path) = output {
        std::fs::write(path, rendered)
            .with_context(|| format!("Failed to write output file: {}", path))?;
        println!("Report written to: {}", path);
    } else {
        println!("{}", rendered);
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Profiling
// ---------------------------------------------------------------------------

/// Handles `legalis profiling`: per-phase latency/allocation profiling.
pub fn handle_profiling(
    inputs: &[String],
    iterations: usize,
    output: Option<&str>,
    format: &OutputFormat,
) -> Result<()> {
    if inputs.is_empty() {
        anyhow::bail!("No input files provided; pass at least one with --input");
    }
    if iterations == 0 {
        anyhow::bail!("Iterations must be at least 1");
    }

    let parser = LegalDslParser::new();
    let verifier = StatuteVerifier::new();
    let profiler = Profiler::new();

    for _ in 0..iterations {
        let mut statutes = Vec::with_capacity(inputs.len());
        for input in inputs {
            let content = profiler
                .try_measure("read", || std::fs::read_to_string(input))
                .with_context(|| format!("Failed to read input file: {}", input))?;
            let statute = profiler
                .measure("parse", || parser.parse_statute(&content))
                .map_err(|error| anyhow::anyhow!("Parse error in {}: {}", input, error))?;
            statutes.push(statute);
        }
        // The verifier result is intentionally discarded; we profile the work.
        let _ = profiler.measure("verify", || verifier.verify(&statutes));
    }

    let report = profiler.analyze();
    let rendered = report.render(format)?;
    write_output(&rendered, output)
}

// ---------------------------------------------------------------------------
// Offline
// ---------------------------------------------------------------------------

/// Handles `legalis offline queue`.
pub fn handle_offline_queue(
    command: &str,
    args: &[String],
    resource: Option<&str>,
    payload: Option<&str>,
    base_version: Option<u64>,
) -> Result<()> {
    let mut store = OfflineStore::open()?;

    let mut queued = QueuedCommand::new(command, args.to_vec());
    if let Some(resource_key) = resource {
        queued.resource_key = Some(resource_key.to_string());
    }
    if let Some(body) = payload {
        queued.payload = Some(body.to_string());
    }
    queued.base_version = base_version;

    // Surface validation findings immediately without blocking the enqueue.
    let outcome = validate_command(&queued);
    let id = store.enqueue(queued)?;

    println!("{}", "Command queued for offline execution".green().bold());
    println!("ID:      {}", id.cyan());
    println!("Command: {}", command);
    if !outcome.valid {
        println!("{}", "Validation warnings/errors:".yellow());
        for issue in &outcome.issues {
            println!("  [{:?}] {}", issue.severity, issue.message);
        }
    }
    Ok(())
}

/// Handles `legalis offline list`.
pub fn handle_offline_list(status: Option<QueueStatus>, format: &OutputFormat) -> Result<()> {
    let store = OfflineStore::open()?;
    let entries = store.queue().filter(status);

    if entries.is_empty() {
        println!("{}", "No queued commands.".yellow());
        return Ok(());
    }

    let owned: Vec<QueuedCommand> = entries.into_iter().cloned().collect();
    emit(&owned, format, || {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_header(vec![
                Cell::new("ID"),
                Cell::new("Command"),
                Cell::new("Resource"),
                Cell::new("Status"),
                Cell::new("Attempts"),
            ]);
        for entry in &owned {
            let short_id: String = entry.id.chars().take(8).collect();
            table.add_row(vec![
                Cell::new(short_id),
                Cell::new(&entry.command),
                Cell::new(entry.resource_key.as_deref().unwrap_or("-")),
                Cell::new(format!("{:?}", entry.status)),
                Cell::new(entry.attempts),
            ]);
        }
        table.to_string()
    })
}

/// Handles `legalis offline validate`.
pub fn handle_offline_validate(format: &OutputFormat) -> Result<()> {
    let mut store = OfflineStore::open()?;
    let results = store.validate_queue()?;

    if results.is_empty() {
        println!("{}", "No queued commands to validate.".yellow());
        return Ok(());
    }

    emit(&results, format, || {
        let mut out = String::new();
        for (id, outcome) in &results {
            let short_id: String = id.chars().take(8).collect();
            let status = if outcome.valid {
                "valid".green().to_string()
            } else {
                "invalid".red().to_string()
            };
            out.push_str(&format!("{}: {}\n", short_id, status));
            for issue in &outcome.issues {
                out.push_str(&format!("    [{:?}] {}\n", issue.severity, issue.message));
            }
        }
        out
    })
}

/// Handles `legalis offline sync`.
pub fn handle_offline_sync(
    host: Option<&str>,
    port: u16,
    force: bool,
    strategy: ConflictStrategy,
    format: &OutputFormat,
) -> Result<()> {
    let mut store = OfflineStore::open()?;
    let applier = JournalApplier::new(store.journal_path());

    // Choose a connectivity probe: forced/online by default, or a TCP probe.
    let tcp_probe;
    let probe: &dyn ConnectivityProbe = if force {
        &AlwaysOnline
    } else if let Some(host) = host {
        tcp_probe = TcpProbe::new(host, port, Duration::from_secs(3));
        &tcp_probe
    } else {
        &AlwaysOnline
    };

    // Checkpoint the sync so an interruption (e.g. the process being killed
    // mid-sync) leaves a resumable record discoverable via `legalis recover`.
    let pending = store.queue().filter(None).len();
    let checkpoint_store = crate::diagnostics::CheckpointStore::open()?;
    let mut checkpoint =
        crate::diagnostics::Checkpoint::new("offline-sync").with_total(pending as u64);
    checkpoint.put("strategy", format!("{strategy:?}"));
    checkpoint.put("force", force.to_string());
    checkpoint_store.save(&checkpoint)?;

    let report = store.sync(&applier as &dyn CommandApplier, probe, strategy)?;

    // Sync completed; record progress and drop the checkpoint.
    checkpoint.advance(report.synced as u64);
    let _ = checkpoint_store.remove(&checkpoint.id);

    emit(&report, format, || {
        let mut out = String::new();
        out.push_str(&report.summary());
        out.push('\n');
        if report.conflicts_detected > 0 {
            out.push_str(&format!(
                "Detected {} conflict(s); see `legalis offline conflicts`.\n",
                report.conflicts_detected
            ));
        }
        out
    })
}

/// Handles `legalis offline conflicts`.
pub fn handle_offline_conflicts(format: &OutputFormat) -> Result<()> {
    let store = OfflineStore::open()?;
    let conflicts = store.conflicts();

    if conflicts.is_empty() {
        println!("{}", "No recorded conflicts.".green());
        return Ok(());
    }

    emit(&conflicts.to_vec(), format, || {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_header(vec![
                Cell::new("ID"),
                Cell::new("Resource"),
                Cell::new("Base"),
                Cell::new("Remote"),
                Cell::new("Resolution"),
            ]);
        for record in conflicts {
            let short_id: String = record.id.chars().take(8).collect();
            table.add_row(vec![
                Cell::new(short_id),
                Cell::new(&record.resource_key),
                Cell::new(record.base_version),
                Cell::new(record.remote_version),
                Cell::new(format!("{:?}", record.resolution)),
            ]);
        }
        table.to_string()
    })
}

/// Handles `legalis offline resolve`.
pub fn handle_offline_resolve(id: &str, prefer_local: bool) -> Result<()> {
    let mut store = OfflineStore::open()?;
    if store.resolve_conflict(id, prefer_local)? {
        let side = if prefer_local { "local" } else { "remote" };
        println!(
            "{}",
            format!("Conflict {} resolved in favor of {}", id, side)
                .green()
                .bold()
        );
    } else {
        println!(
            "{}",
            format!("No unresolved conflict found with id {}", id).yellow()
        );
    }
    Ok(())
}

/// Handles `legalis offline cache-stats`.
pub fn handle_offline_cache_stats() -> Result<()> {
    let store = OfflineStore::open()?;
    let cache = store.cache();
    println!("{}", "Offline cache statistics".cyan().bold());
    println!("Records: {}", cache.len());
    Ok(())
}

/// Handles `legalis offline cache-prune`.
pub fn handle_offline_cache_prune() -> Result<()> {
    let mut store = OfflineStore::open()?;
    let pruned = store.cache_prune()?;
    println!("Pruned {} expired cache record(s)", pruned);
    Ok(())
}

/// Handles `legalis offline clear`.
pub fn handle_offline_clear() -> Result<()> {
    let mut store = OfflineStore::open()?;
    let cleared = store.clear_queue()?;
    println!("Cleared {} queued command(s)", cleared);
    Ok(())
}
