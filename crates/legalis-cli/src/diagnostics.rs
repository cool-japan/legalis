//! Self-healing: self-diagnostics, configuration repair, and crash
//! recovery/resume.
//!
//! This module provides three cooperating capabilities:
//!
//! - **Self-diagnostics** ([`run_diagnostics`]): a structured health report over
//!   the environment, data directory, config, policy and audit log — richer and
//!   machine-readable compared to the textual `doctor` command.
//! - **Configuration repair** ([`repair_config`]): detect and fix invalid
//!   configuration (bad enum values, malformed booleans, missing directories),
//!   returning the set of repairs performed.
//! - **Crash recovery & resume** ([`CheckpointStore`]): checkpoint long-running
//!   operations to disk so they can be resumed after an interruption, complete
//!   with progress and an opaque payload.
//!
//! Everything is file-backed under the data directory; tests redirect via
//! `LEGALIS_DATA_DIR`.

use crate::paths;
use crate::policy::Policy;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

// ---------------------------------------------------------------------------
// Self-diagnostics
// ---------------------------------------------------------------------------

/// The status of a single diagnostic check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CheckStatus {
    /// The check passed.
    Ok,
    /// The check found a non-fatal problem.
    Warn,
    /// The check found a problem that likely breaks functionality.
    Fail,
}

impl CheckStatus {
    /// A short symbol for textual rendering.
    pub fn symbol(self) -> &'static str {
        match self {
            CheckStatus::Ok => "[OK]",
            CheckStatus::Warn => "[WARN]",
            CheckStatus::Fail => "[FAIL]",
        }
    }
}

/// A single diagnostic result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticCheck {
    /// The name of the check.
    pub name: String,
    /// Its status.
    pub status: CheckStatus,
    /// A human-readable message.
    pub message: String,
}

impl DiagnosticCheck {
    fn ok(name: &str, message: impl Into<String>) -> Self {
        Self {
            name: name.to_string(),
            status: CheckStatus::Ok,
            message: message.into(),
        }
    }
    fn warn(name: &str, message: impl Into<String>) -> Self {
        Self {
            name: name.to_string(),
            status: CheckStatus::Warn,
            message: message.into(),
        }
    }
    fn fail(name: &str, message: impl Into<String>) -> Self {
        Self {
            name: name.to_string(),
            status: CheckStatus::Fail,
            message: message.into(),
        }
    }
}

/// The full diagnostic report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticReport {
    /// The individual checks, in execution order.
    pub checks: Vec<DiagnosticCheck>,
}

impl DiagnosticReport {
    /// The number of failing checks.
    pub fn failures(&self) -> usize {
        self.checks
            .iter()
            .filter(|c| c.status == CheckStatus::Fail)
            .count()
    }

    /// The number of warning checks.
    pub fn warnings(&self) -> usize {
        self.checks
            .iter()
            .filter(|c| c.status == CheckStatus::Warn)
            .count()
    }

    /// Whether the system is healthy (no failures).
    pub fn is_healthy(&self) -> bool {
        self.failures() == 0
    }

    /// Renders the report as plain text.
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        for check in &self.checks {
            out.push_str(&format!(
                "{} {}: {}\n",
                check.status.symbol(),
                check.name,
                check.message
            ));
        }
        out.push_str(&format!(
            "\n{} check(s): {} ok, {} warning(s), {} failure(s)\n",
            self.checks.len(),
            self.checks.len() - self.warnings() - self.failures(),
            self.warnings(),
            self.failures()
        ));
        out
    }
}

/// Runs the full self-diagnostic suite and returns a structured report.
pub fn run_diagnostics() -> DiagnosticReport {
    let mut checks = Vec::new();

    // 1. Data directory writability.
    match paths::data_dir() {
        Ok(dir) => {
            let probe = dir.join(format!(".diag-probe-{}", uuid::Uuid::new_v4()));
            match std::fs::write(&probe, b"ok") {
                Ok(()) => {
                    let _ = std::fs::remove_file(&probe);
                    checks.push(DiagnosticCheck::ok(
                        "data-dir",
                        format!("writable: {}", dir.display()),
                    ));
                }
                Err(error) => checks.push(DiagnosticCheck::fail(
                    "data-dir",
                    format!("not writable ({}): {}", dir.display(), error),
                )),
            }
        }
        Err(error) => {
            checks.push(DiagnosticCheck::fail(
                "data-dir",
                format!("could not resolve: {error}"),
            ));
        }
    }

    // 2. Temp directory writability (used by many operations).
    let temp_probe = std::env::temp_dir().join(format!(".diag-tmp-{}", uuid::Uuid::new_v4()));
    match std::fs::write(&temp_probe, b"ok") {
        Ok(()) => {
            let _ = std::fs::remove_file(&temp_probe);
            checks.push(DiagnosticCheck::ok("temp-dir", "writable"));
        }
        Err(error) => checks.push(DiagnosticCheck::fail(
            "temp-dir",
            format!("not writable: {error}"),
        )),
    }

    // 3. Config load + validation.
    let config = crate::config::Config::load();
    match config.validate() {
        Ok(warnings) if warnings.is_empty() => {
            checks.push(DiagnosticCheck::ok("config", "loaded and valid"));
        }
        Ok(warnings) => checks.push(DiagnosticCheck::warn(
            "config",
            format!("{} warning(s): {}", warnings.len(), warnings.join("; ")),
        )),
        Err(error) => checks.push(DiagnosticCheck::warn(
            "config",
            format!("validation error: {error}"),
        )),
    }

    // 4. Central config validation.
    match crate::central_config::CentralConfig::discover() {
        Ok(central) => {
            let errors = central.validate();
            if errors.is_empty() {
                checks.push(DiagnosticCheck::ok("central-config", "valid"));
            } else {
                checks.push(DiagnosticCheck::fail(
                    "central-config",
                    format!("{} error(s): {}", errors.len(), errors.join("; ")),
                ));
            }
        }
        Err(error) => checks.push(DiagnosticCheck::warn(
            "central-config",
            format!("could not load: {error}"),
        )),
    }

    // 5. Policy load.
    match Policy::discover(None) {
        Ok(policy) => {
            let descriptor = if policy.is_restrictive() {
                format!("active and restrictive ('{}')", policy.name)
            } else {
                "permissive (no restrictions)".to_string()
            };
            checks.push(DiagnosticCheck::ok("policy", descriptor));
        }
        Err(error) => checks.push(DiagnosticCheck::fail(
            "policy",
            format!("failed to load: {error}"),
        )),
    }

    // 6. Audit log openable + integrity.
    match crate::audit_log::AuditLogger::open() {
        Ok(logger) => match logger.verify_integrity() {
            Ok(true) => checks.push(DiagnosticCheck::ok(
                "audit-log",
                format!("intact ({} record(s))", logger.count()),
            )),
            Ok(false) => checks.push(DiagnosticCheck::fail(
                "audit-log",
                "integrity check FAILED (possible tampering)",
            )),
            Err(error) => checks.push(DiagnosticCheck::warn(
                "audit-log",
                format!("could not verify: {error}"),
            )),
        },
        Err(error) => checks.push(DiagnosticCheck::warn(
            "audit-log",
            format!("could not open: {error}"),
        )),
    }

    // 7. Pending crash-recovery checkpoints.
    match CheckpointStore::open() {
        Ok(store) => {
            let pending = store.list().unwrap_or_default();
            if pending.is_empty() {
                checks.push(DiagnosticCheck::ok("checkpoints", "none pending"));
            } else {
                checks.push(DiagnosticCheck::warn(
                    "checkpoints",
                    format!(
                        "{} resumable operation(s) pending; run `legalis recover --list`",
                        pending.len()
                    ),
                ));
            }
        }
        Err(error) => checks.push(DiagnosticCheck::warn(
            "checkpoints",
            format!("could not access: {error}"),
        )),
    }

    DiagnosticReport { checks }
}

// ---------------------------------------------------------------------------
// Configuration repair
// ---------------------------------------------------------------------------

/// A single repair that was (or would be) applied.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfigRepair {
    /// The setting/area repaired.
    pub field: String,
    /// What was wrong.
    pub problem: String,
    /// What the value/state was changed to.
    pub fix: String,
}

/// The result of a configuration repair pass.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RepairReport {
    /// The repairs performed (or proposed, in dry-run mode).
    pub repairs: Vec<ConfigRepair>,
    /// Whether changes were actually written (false in dry-run).
    pub applied: bool,
}

impl RepairReport {
    /// Whether anything needed repairing.
    pub fn needed_repair(&self) -> bool {
        !self.repairs.is_empty()
    }
}

/// Detects and (unless `dry_run`) fixes invalid configuration in a TOML config
/// file. Returns the repairs performed.
///
/// Repairs:
/// - `output.format` outside the known set -> reset to `text`.
/// - `output.directory` missing -> created (and noted).
/// - non-boolean `output.colored` -> reset to `true`.
/// - `active_profile` referencing an undefined profile -> cleared.
///
/// The function is conservative: it never discards unknown keys.
pub fn repair_config(path: &Path, dry_run: bool) -> Result<RepairReport> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read config: {}", path.display()))?;

    // Parse into a generic TOML table so we can repair individual keys without
    // losing unknown ones.
    let mut table: toml::Table = toml::from_str(&content)
        .with_context(|| format!("Config is not valid TOML: {}", path.display()))?;

    let mut repairs = Vec::new();
    const VALID_FORMATS: &[&str] = &["text", "json", "yaml", "toml", "table", "csv", "html"];

    // Repair [output] section.
    if let Some(toml::Value::Table(output)) = table.get_mut("output") {
        if let Some(toml::Value::String(format)) = output.get("format").cloned()
            && !VALID_FORMATS.contains(&format.as_str())
        {
            repairs.push(ConfigRepair {
                field: "output.format".to_string(),
                problem: format!("invalid format '{format}'"),
                fix: "reset to 'text'".to_string(),
            });
            output.insert("format".to_string(), toml::Value::String("text".into()));
        }
        if let Some(toml::Value::String(dir)) = output.get("directory").cloned()
            && !Path::new(&dir).exists()
        {
            repairs.push(ConfigRepair {
                field: "output.directory".to_string(),
                problem: format!("directory does not exist: {dir}"),
                fix: "created directory".to_string(),
            });
            if !dry_run {
                std::fs::create_dir_all(&dir)
                    .with_context(|| format!("Failed to create output directory: {dir}"))?;
            }
        }
        // Repair non-boolean `colored`.
        if let Some(value) = output.get("colored").cloned()
            && !value.is_bool()
        {
            repairs.push(ConfigRepair {
                field: "output.colored".to_string(),
                problem: format!("expected boolean, found {value}"),
                fix: "reset to true".to_string(),
            });
            output.insert("colored".to_string(), toml::Value::Boolean(true));
        }
    }

    // Repair dangling active_profile referencing a missing profile.
    let active = table
        .get("active_profile")
        .and_then(|v| v.as_str())
        .map(String::from);
    if let Some(active_name) = active {
        let has_profile = table
            .get("profiles")
            .and_then(|p| p.as_table())
            .map(|p| p.contains_key(&active_name))
            .unwrap_or(false);
        if !has_profile {
            repairs.push(ConfigRepair {
                field: "active_profile".to_string(),
                problem: format!("references undefined profile '{active_name}'"),
                fix: "cleared active_profile".to_string(),
            });
            table.remove("active_profile");
        }
    }

    let applied = !dry_run && !repairs.is_empty();
    if applied {
        let serialized =
            toml::to_string_pretty(&table).context("Failed to serialize repaired config")?;
        std::fs::write(path, serialized)
            .with_context(|| format!("Failed to write repaired config: {}", path.display()))?;
    }

    Ok(RepairReport { repairs, applied })
}

// ---------------------------------------------------------------------------
// Crash recovery & resume (checkpoints)
// ---------------------------------------------------------------------------

/// A resumable checkpoint for a long-running operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Checkpoint {
    /// Unique id.
    pub id: String,
    /// The operation name (e.g. `batch-verify`).
    pub operation: String,
    /// Total number of work items, if known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<u64>,
    /// Number of completed work items.
    #[serde(default)]
    pub completed: u64,
    /// Opaque, operation-specific resume state (arbitrary JSON).
    #[serde(default)]
    pub payload: BTreeMap<String, String>,
    /// RFC 3339 creation timestamp.
    pub created_at: String,
    /// RFC 3339 last-update timestamp.
    pub updated_at: String,
}

impl Checkpoint {
    /// Creates a new checkpoint for `operation`.
    pub fn new(operation: impl Into<String>) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            operation: operation.into(),
            total: None,
            completed: 0,
            payload: BTreeMap::new(),
            created_at: now.clone(),
            updated_at: now,
        }
    }

    /// Sets the total work count (builder style).
    pub fn with_total(mut self, total: u64) -> Self {
        self.total = Some(total);
        self
    }

    /// Records progress, updating the timestamp.
    pub fn advance(&mut self, completed: u64) {
        self.completed = completed;
        self.updated_at = chrono::Utc::now().to_rfc3339();
    }

    /// Stores an opaque payload entry.
    pub fn put(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.payload.insert(key.into(), value.into());
    }

    /// Fraction complete in `[0, 1]`, or `None` when total is unknown.
    pub fn fraction(&self) -> Option<f64> {
        self.total.map(|total| {
            if total == 0 {
                1.0
            } else {
                (self.completed as f64 / total as f64).clamp(0.0, 1.0)
            }
        })
    }

    /// Whether the operation has finished all known work.
    pub fn is_complete(&self) -> bool {
        matches!(self.total, Some(total) if self.completed >= total)
    }
}

/// File-backed store of resumable checkpoints (one JSON file per checkpoint).
#[derive(Debug, Clone)]
pub struct CheckpointStore {
    dir: PathBuf,
}

impl CheckpointStore {
    /// Opens the default checkpoint store under the data directory.
    pub fn open() -> Result<Self> {
        Ok(Self {
            dir: paths::checkpoint_dir()?,
        })
    }

    /// Opens a checkpoint store under a specific directory (for tests).
    pub fn open_at(dir: impl Into<PathBuf>) -> Result<Self> {
        let dir = dir.into();
        std::fs::create_dir_all(&dir)
            .with_context(|| format!("Failed to create checkpoint dir: {}", dir.display()))?;
        Ok(Self { dir })
    }

    fn path_for(&self, id: &str) -> PathBuf {
        self.dir.join(format!("{id}.json"))
    }

    /// Saves (creates or overwrites) a checkpoint.
    pub fn save(&self, checkpoint: &Checkpoint) -> Result<()> {
        let path = self.path_for(&checkpoint.id);
        let content =
            serde_json::to_string_pretty(checkpoint).context("Failed to serialize checkpoint")?;
        std::fs::write(&path, content)
            .with_context(|| format!("Failed to write checkpoint: {}", path.display()))?;
        Ok(())
    }

    /// Loads a checkpoint by id.
    pub fn load(&self, id: &str) -> Result<Checkpoint> {
        let path = self.path_for(id);
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("Checkpoint not found: {}", path.display()))?;
        serde_json::from_str(&content).context("Failed to parse checkpoint")
    }

    /// Removes a checkpoint by id, returning whether it existed.
    pub fn remove(&self, id: &str) -> Result<bool> {
        let path = self.path_for(id);
        if path.exists() {
            std::fs::remove_file(&path)
                .with_context(|| format!("Failed to remove checkpoint: {}", path.display()))?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Lists all pending checkpoints, newest first.
    pub fn list(&self) -> Result<Vec<Checkpoint>> {
        let mut checkpoints = Vec::new();
        let entries = match std::fs::read_dir(&self.dir) {
            Ok(entries) => entries,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(checkpoints),
            Err(error) => {
                return Err(error).with_context(|| {
                    format!("Failed to read checkpoint dir: {}", self.dir.display())
                });
            }
        };
        for entry in entries {
            let entry = entry.context("Failed to read checkpoint entry")?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            // Skip unreadable/corrupt files rather than failing the whole listing.
            if let Ok(content) = std::fs::read_to_string(&path)
                && let Ok(checkpoint) = serde_json::from_str::<Checkpoint>(&content)
            {
                checkpoints.push(checkpoint);
            }
        }
        checkpoints.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        Ok(checkpoints)
    }

    /// Removes all completed checkpoints, returning how many were pruned.
    pub fn prune_completed(&self) -> Result<usize> {
        let mut pruned = 0;
        for checkpoint in self.list()? {
            if checkpoint.is_complete() && self.remove(&checkpoint.id)? {
                pruned += 1;
            }
        }
        Ok(pruned)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_dir() -> PathBuf {
        let dir = std::env::temp_dir().join(format!("legalis-diag-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&dir).expect("mkdir");
        dir
    }

    fn temp_file() -> PathBuf {
        std::env::temp_dir().join(format!("legalis-cfg-{}.toml", uuid::Uuid::new_v4()))
    }

    #[test]
    fn test_repair_invalid_format() {
        let path = temp_file();
        std::fs::write(&path, "[output]\nformat = \"bogus\"\ncolored = true\n").expect("write");
        let report = repair_config(&path, false).expect("repair");
        assert!(report.applied);
        assert!(report.repairs.iter().any(|r| r.field == "output.format"));
        // The file should now contain the corrected value.
        let fixed = std::fs::read_to_string(&path).expect("read");
        assert!(fixed.contains("format = \"text\""));
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_repair_dry_run_does_not_write() {
        let path = temp_file();
        std::fs::write(&path, "[output]\nformat = \"bogus\"\n").expect("write");
        let report = repair_config(&path, true).expect("repair dry");
        assert!(!report.applied);
        assert!(report.needed_repair());
        let unchanged = std::fs::read_to_string(&path).expect("read");
        assert!(unchanged.contains("bogus"));
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_repair_noop_on_valid_config() {
        let path = temp_file();
        std::fs::write(&path, "[output]\nformat = \"json\"\ncolored = true\n").expect("write");
        let report = repair_config(&path, false).expect("repair");
        assert!(!report.needed_repair());
        assert!(!report.applied);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_repair_dangling_active_profile() {
        let path = temp_file();
        std::fs::write(&path, "active_profile = \"ghost\"\n").expect("write");
        let report = repair_config(&path, false).expect("repair");
        assert!(report.repairs.iter().any(|r| r.field == "active_profile"));
        let fixed = std::fs::read_to_string(&path).expect("read");
        assert!(!fixed.contains("active_profile"));
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_repair_non_bool_colored() {
        let path = temp_file();
        std::fs::write(&path, "[output]\ncolored = \"yes\"\n").expect("write");
        let report = repair_config(&path, false).expect("repair");
        assert!(report.repairs.iter().any(|r| r.field == "output.colored"));
        let fixed = std::fs::read_to_string(&path).expect("read");
        assert!(fixed.contains("colored = true"));
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_checkpoint_lifecycle() {
        let dir = temp_dir();
        let store = CheckpointStore::open_at(&dir).expect("open");
        let mut cp = Checkpoint::new("batch-verify").with_total(10);
        cp.put("input", "statutes/*.ldsl");
        store.save(&cp).expect("save");

        cp.advance(5);
        store.save(&cp).expect("update");

        let loaded = store.load(&cp.id).expect("load");
        assert_eq!(loaded.completed, 5);
        assert_eq!(loaded.total, Some(10));
        assert_eq!(
            loaded.payload.get("input").map(String::as_str),
            Some("statutes/*.ldsl")
        );
        assert_eq!(loaded.fraction(), Some(0.5));
        assert!(!loaded.is_complete());

        let listed = store.list().expect("list");
        assert_eq!(listed.len(), 1);

        assert!(store.remove(&cp.id).expect("remove"));
        assert!(!store.remove(&cp.id).expect("remove again"));
        assert!(store.list().expect("list empty").is_empty());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_checkpoint_complete_and_prune() {
        let dir = temp_dir();
        let store = CheckpointStore::open_at(&dir).expect("open");
        let mut done = Checkpoint::new("export").with_total(3);
        done.advance(3);
        assert!(done.is_complete());
        store.save(&done).expect("save done");

        let pending = Checkpoint::new("simulate").with_total(100);
        store.save(&pending).expect("save pending");

        let pruned = store.prune_completed().expect("prune");
        assert_eq!(pruned, 1);
        let remaining = store.list().expect("list");
        assert_eq!(remaining.len(), 1);
        assert_eq!(remaining[0].operation, "simulate");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_checkpoint_list_skips_corrupt() {
        let dir = temp_dir();
        let store = CheckpointStore::open_at(&dir).expect("open");
        let cp = Checkpoint::new("op");
        store.save(&cp).expect("save");
        std::fs::write(dir.join("garbage.json"), "not json").expect("write garbage");
        let listed = store.list().expect("list");
        assert_eq!(listed.len(), 1);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_checkpoint_unknown_total_fraction() {
        let cp = Checkpoint::new("op");
        assert_eq!(cp.fraction(), None);
        assert!(!cp.is_complete());
    }

    #[test]
    fn test_diagnostic_report_helpers() {
        let report = DiagnosticReport {
            checks: vec![
                DiagnosticCheck::ok("a", "fine"),
                DiagnosticCheck::warn("b", "meh"),
                DiagnosticCheck::fail("c", "broken"),
            ],
        };
        assert_eq!(report.failures(), 1);
        assert_eq!(report.warnings(), 1);
        assert!(!report.is_healthy());
        let text = report.render_text();
        assert!(text.contains("[OK] a"));
        assert!(text.contains("[FAIL] c"));
    }

    #[test]
    fn test_run_diagnostics_in_temp_data_dir() {
        let dir = temp_dir();
        let saved = std::env::var(paths::DATA_DIR_ENV).ok();
        unsafe {
            std::env::set_var(paths::DATA_DIR_ENV, &dir);
        }
        let report = run_diagnostics();
        // The data-dir check must be present and the report non-empty.
        assert!(report.checks.iter().any(|c| c.name == "data-dir"));
        assert!(!report.checks.is_empty());
        unsafe {
            match saved {
                Some(v) => std::env::set_var(paths::DATA_DIR_ENV, v),
                None => std::env::remove_var(paths::DATA_DIR_ENV),
            }
        }
        let _ = std::fs::remove_dir_all(&dir);
    }
}
