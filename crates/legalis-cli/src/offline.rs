//! Offline-first capabilities for the Legalis CLI.
//!
//! This module implements the building blocks required to operate the CLI
//! without continuous connectivity and to reconcile changes once a connection
//! returns:
//!
//! - A **persisted command queue** ([`CommandQueue`]) that records mutating
//!   commands issued while offline, backed by JSON under a cache directory.
//! - A **versioned local cache** ([`LocalCache`]) with per-entry TTLs that holds
//!   resource snapshots for offline reads and as common ancestors for merges.
//! - **Connectivity probing** ([`ConnectivityProbe`]) including a real TCP probe.
//! - **Offline validation** ([`validate_command`]) of queued commands against the
//!   known command surface and structural rules.
//! - A **sync reconciler** ([`OfflineStore::sync`]) that replays queued commands
//!   through a [`CommandApplier`] when online, detecting version conflicts.
//! - **Conflict resolution** via last-writer-wins, remote-wins, or a recursive
//!   three-way JSON [`merge_three_way`], with explicit [`ConflictRecord`]s.
//!
//! Everything is file-backed under a configurable base directory. Production
//! code uses the platform cache directory; tests use [`std::env::temp_dir`].

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::net::{TcpStream, ToSocketAddrs};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Subcommands recognized by the CLI, used for offline validation.
const KNOWN_COMMANDS: &[&str] = &[
    "parse",
    "verify",
    "viz",
    "export",
    "init",
    "diff",
    "simulate",
    "audit",
    "complexity",
    "port",
    "import",
    "convert",
    "lod",
    "format",
    "lint",
    "test",
    "new",
    "search",
    "publish",
    "validate",
    "install",
    "list",
    "add",
    "update",
    "clean",
    "outdated",
    "uninstall",
    "explain",
    "trace",
    "benchmark",
    "migrate",
    "graph",
];

/// Commands that mutate registry/project state and therefore benefit from a
/// resource key and base version for conflict detection.
const MUTATING_COMMANDS: &[&str] = &[
    "publish",
    "install",
    "add",
    "update",
    "uninstall",
    "port",
    "migrate",
    "format",
    "init",
    "new",
    "clean",
];

/// Returns the current Unix timestamp in seconds (0 on a clock error).
fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|delta| delta.as_secs())
        .unwrap_or(0)
}

/// Returns the current time as an RFC 3339 string.
fn now_rfc3339() -> String {
    chrono::Utc::now().to_rfc3339()
}

// ---------------------------------------------------------------------------
// Command queue
// ---------------------------------------------------------------------------

/// Lifecycle state of a queued command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QueueStatus {
    /// Newly queued, not yet validated.
    Pending,
    /// Passed offline validation, ready to sync.
    Validated,
    /// Successfully applied to the authoritative store.
    Synced,
    /// Application failed; eligible for retry.
    Failed,
    /// Conflicts with the remote state and needs manual resolution.
    Conflicted,
}

impl QueueStatus {
    /// Whether a command in this state should be attempted during a sync.
    fn is_syncable(self) -> bool {
        matches!(self, Self::Pending | Self::Validated | Self::Failed)
    }
}

/// A command captured while offline for later reconciliation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QueuedCommand {
    /// Unique identifier.
    pub id: String,
    /// Subcommand name (e.g. `publish`).
    pub command: String,
    /// Command arguments.
    pub args: Vec<String>,
    /// Resource the command mutates, used for conflict detection.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_key: Option<String>,
    /// Serialized payload (typically JSON) representing the intended new state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
    /// Version of the resource the change was based on (the common ancestor).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_version: Option<u64>,
    /// RFC 3339 creation timestamp.
    pub created_at: String,
    /// Current lifecycle state.
    pub status: QueueStatus,
    /// Number of failed sync attempts.
    pub attempts: u32,
    /// Most recent error, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_error: Option<String>,
}

impl QueuedCommand {
    /// Creates a new pending command.
    pub fn new(command: impl Into<String>, args: Vec<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            command: command.into(),
            args,
            resource_key: None,
            payload: None,
            base_version: None,
            created_at: now_rfc3339(),
            status: QueueStatus::Pending,
            attempts: 0,
            last_error: None,
        }
    }

    /// Sets the resource key (builder style).
    pub fn with_resource(mut self, resource_key: impl Into<String>) -> Self {
        self.resource_key = Some(resource_key.into());
        self
    }

    /// Sets the payload and base version (builder style).
    pub fn with_payload(mut self, payload: impl Into<String>, base_version: u64) -> Self {
        self.payload = Some(payload.into());
        self.base_version = Some(base_version);
        self
    }

    /// Whether the command mutates authoritative state.
    pub fn is_mutating(&self) -> bool {
        MUTATING_COMMANDS.contains(&self.command.as_str())
    }
}

/// An ordered, persistable queue of offline commands.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommandQueue {
    /// Queued commands in insertion order.
    pub entries: Vec<QueuedCommand>,
}

impl CommandQueue {
    /// Number of queued commands.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns the commands matching `status`, or all when `None`.
    pub fn filter(&self, status: Option<QueueStatus>) -> Vec<&QueuedCommand> {
        self.entries
            .iter()
            .filter(|entry| status.is_none_or(|wanted| entry.status == wanted))
            .collect()
    }

    /// Removes a command by id, returning whether it was present.
    pub fn remove(&mut self, id: &str) -> bool {
        let before = self.entries.len();
        self.entries.retain(|entry| entry.id != id);
        before != self.entries.len()
    }
}

// ---------------------------------------------------------------------------
// Local cache
// ---------------------------------------------------------------------------

/// A versioned cache entry with a time-to-live.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CacheRecord {
    /// Resource key.
    pub key: String,
    /// Serialized value.
    pub value: String,
    /// Authoritative version of the value.
    pub version: u64,
    /// Unix timestamp (seconds) when stored.
    pub created_at: u64,
    /// Time-to-live in seconds; `0` means never expires.
    pub ttl_secs: u64,
}

impl CacheRecord {
    /// Whether this record has expired relative to `now` (Unix seconds).
    pub fn is_expired(&self, now: u64) -> bool {
        self.ttl_secs > 0 && now.saturating_sub(self.created_at) > self.ttl_secs
    }
}

/// An in-memory, file-backed versioned cache with per-entry TTLs.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LocalCache {
    /// Records keyed by resource key.
    pub records: HashMap<String, CacheRecord>,
}

impl LocalCache {
    /// Stores a value under `key`, replacing any prior record.
    pub fn put(&mut self, key: &str, value: impl Into<String>, version: u64, ttl_secs: u64) {
        self.records.insert(
            key.to_string(),
            CacheRecord {
                key: key.to_string(),
                value: value.into(),
                version,
                created_at: now_secs(),
                ttl_secs,
            },
        );
    }

    /// Fetches a non-expired record for `key`.
    pub fn get(&self, key: &str) -> Option<&CacheRecord> {
        self.records
            .get(key)
            .filter(|record| !record.is_expired(now_secs()))
    }

    /// Removes expired records, returning the number pruned.
    pub fn prune_expired(&mut self) -> usize {
        let now = now_secs();
        let before = self.records.len();
        self.records.retain(|_, record| !record.is_expired(now));
        before - self.records.len()
    }

    /// Number of cached records (including expired ones).
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Whether the cache holds no records.
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}

// ---------------------------------------------------------------------------
// Connectivity
// ---------------------------------------------------------------------------

/// Determines whether the CLI is currently online.
pub trait ConnectivityProbe {
    /// Returns `true` when connectivity is available.
    fn is_online(&self) -> bool;
}

/// A probe that always reports online (e.g. for `--force`).
pub struct AlwaysOnline;
impl ConnectivityProbe for AlwaysOnline {
    fn is_online(&self) -> bool {
        true
    }
}

/// A probe that always reports offline (used in tests).
pub struct AlwaysOffline;
impl ConnectivityProbe for AlwaysOffline {
    fn is_online(&self) -> bool {
        false
    }
}

/// A probe that attempts a TCP connection to a host within a timeout.
pub struct TcpProbe {
    /// Host name or address.
    pub host: String,
    /// Port number.
    pub port: u16,
    /// Connection timeout.
    pub timeout: Duration,
}

impl TcpProbe {
    /// Creates a TCP probe with the given target and timeout.
    pub fn new(host: impl Into<String>, port: u16, timeout: Duration) -> Self {
        Self {
            host: host.into(),
            port,
            timeout,
        }
    }
}

impl ConnectivityProbe for TcpProbe {
    fn is_online(&self) -> bool {
        let target = format!("{}:{}", self.host, self.port);
        match target.to_socket_addrs() {
            Ok(addresses) => addresses
                .into_iter()
                .any(|address| TcpStream::connect_timeout(&address, self.timeout).is_ok()),
            Err(_) => false,
        }
    }
}

// ---------------------------------------------------------------------------
// Validation
// ---------------------------------------------------------------------------

/// Severity of a validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidationSeverity {
    /// Non-fatal advisory.
    Warning,
    /// Fatal; the command cannot be synced as-is.
    Error,
}

/// A single validation finding for a queued command.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationIssue {
    /// Severity of the finding.
    pub severity: ValidationSeverity,
    /// Human-readable description.
    pub message: String,
}

/// The result of validating a queued command.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationOutcome {
    /// Whether the command passed (no error-level issues).
    pub valid: bool,
    /// All findings, ordered by discovery.
    pub issues: Vec<ValidationIssue>,
}

impl ValidationOutcome {
    /// Convenience constructor that derives `valid` from the issue list.
    fn from_issues(issues: Vec<ValidationIssue>) -> Self {
        let valid = !issues
            .iter()
            .any(|issue| issue.severity == ValidationSeverity::Error);
        Self { valid, issues }
    }
}

/// Validates a queued command structurally and against the known command set.
pub fn validate_command(command: &QueuedCommand) -> ValidationOutcome {
    let mut issues = Vec::new();

    if command.command.trim().is_empty() {
        issues.push(ValidationIssue {
            severity: ValidationSeverity::Error,
            message: "Command name is empty".to_string(),
        });
    } else if !KNOWN_COMMANDS.contains(&command.command.as_str()) {
        issues.push(ValidationIssue {
            severity: ValidationSeverity::Error,
            message: format!("Unknown command '{}'", command.command),
        });
    }

    // Mutating commands require a resource key to detect conflicts.
    if command.is_mutating() {
        if command
            .resource_key
            .as_deref()
            .unwrap_or("")
            .trim()
            .is_empty()
        {
            issues.push(ValidationIssue {
                severity: ValidationSeverity::Error,
                message: format!(
                    "Mutating command '{}' requires a resource key",
                    command.command
                ),
            });
        }
        if command.base_version.is_none() {
            issues.push(ValidationIssue {
                severity: ValidationSeverity::Warning,
                message: "No base version recorded; conflict detection will be limited".to_string(),
            });
        }
    }

    // Payloads, when present, must be valid JSON for merging.
    if let Some(payload) = &command.payload
        && serde_json::from_str::<Value>(payload).is_err()
    {
        issues.push(ValidationIssue {
            severity: ValidationSeverity::Error,
            message: "Payload is not valid JSON".to_string(),
        });
    }

    if command.args.iter().any(|arg| arg.trim().is_empty()) {
        issues.push(ValidationIssue {
            severity: ValidationSeverity::Warning,
            message: "One or more arguments are empty".to_string(),
        });
    }

    ValidationOutcome::from_issues(issues)
}

// ---------------------------------------------------------------------------
// Conflict resolution
// ---------------------------------------------------------------------------

/// Strategy for resolving a detected conflict.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictStrategy {
    /// The locally queued change wins.
    LastWriterWins,
    /// The remote (authoritative) state wins; the local change is dropped.
    RemoteWins,
    /// Attempt a three-way merge, recording an unresolved conflict on failure.
    Merge,
}

/// How a conflict was ultimately resolved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictResolution {
    /// Resolved in favor of the local change.
    ResolvedLocal,
    /// Resolved in favor of the remote state.
    ResolvedRemote,
    /// Resolved by a clean three-way merge.
    Merged,
    /// Could not be resolved automatically.
    Unresolved,
}

/// A persisted record describing a detected (and possibly resolved) conflict.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConflictRecord {
    /// Unique identifier.
    pub id: String,
    /// The queued command that conflicted.
    pub command_id: String,
    /// Resource key in conflict.
    pub resource_key: String,
    /// Base version the local change assumed.
    pub base_version: u64,
    /// Authoritative remote version at detection time.
    pub remote_version: u64,
    /// Strategy applied.
    pub strategy: ConflictStrategy,
    /// Outcome of resolution.
    pub resolution: ConflictResolution,
    /// JSON paths that could not be auto-merged (for `Merge`).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conflicting_paths: Vec<String>,
    /// RFC 3339 detection timestamp.
    pub detected_at: String,
    /// Local payload at the time of conflict.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_payload: Option<String>,
    /// Remote payload at the time of conflict.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote_payload: Option<String>,
    /// Merged payload, if a merge produced one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merged_payload: Option<String>,
}

/// Result of a three-way merge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MergeResult {
    /// The merged value (best-effort even when conflicts exist).
    pub merged: Value,
    /// JSON paths where local and remote diverged irreconcilably.
    pub conflicts: Vec<String>,
}

impl MergeResult {
    /// Whether the merge succeeded without conflicts.
    pub fn is_clean(&self) -> bool {
        self.conflicts.is_empty()
    }
}

/// Performs a recursive three-way merge of JSON values.
///
/// For each position the algorithm applies standard three-way semantics:
/// - if local and remote agree, take that value;
/// - if only one side changed relative to `base`, take the changed side;
/// - if both sides changed differently and both are objects, merge per-key;
/// - otherwise record an unresolvable conflict at that path.
///
/// Absent object keys are treated as JSON `null`, so delete/modify divergence is
/// reported as a conflict.
pub fn merge_three_way(base: &Value, local: &Value, remote: &Value) -> MergeResult {
    let mut conflicts = Vec::new();
    let merged = merge_value("", base, local, remote, &mut conflicts);
    MergeResult { merged, conflicts }
}

/// Recursive worker for [`merge_three_way`].
fn merge_value(
    path: &str,
    base: &Value,
    local: &Value,
    remote: &Value,
    conflicts: &mut Vec<String>,
) -> Value {
    if local == remote {
        return local.clone();
    }
    if local == base {
        return remote.clone();
    }
    if remote == base {
        return local.clone();
    }

    match (local, remote) {
        (Value::Object(local_map), Value::Object(remote_map)) => {
            let empty = serde_json::Map::new();
            let base_map = base.as_object().unwrap_or(&empty);

            let mut keys: BTreeSet<&String> = BTreeSet::new();
            keys.extend(base_map.keys());
            keys.extend(local_map.keys());
            keys.extend(remote_map.keys());

            let mut merged = serde_json::Map::new();
            for key in keys {
                let base_child = base_map.get(key).cloned().unwrap_or(Value::Null);
                let local_child = local_map.get(key).cloned().unwrap_or(Value::Null);
                let remote_child = remote_map.get(key).cloned().unwrap_or(Value::Null);
                let child_path = if path.is_empty() {
                    key.to_string()
                } else {
                    format!("{path}/{key}")
                };
                let value = merge_value(
                    &child_path,
                    &base_child,
                    &local_child,
                    &remote_child,
                    conflicts,
                );
                if !value.is_null() {
                    merged.insert(key.clone(), value);
                }
            }
            Value::Object(merged)
        }
        _ => {
            conflicts.push(if path.is_empty() {
                "<root>".to_string()
            } else {
                path.to_string()
            });
            local.clone()
        }
    }
}

// ---------------------------------------------------------------------------
// Appliers
// ---------------------------------------------------------------------------

/// The authoritative state of a resource as seen by an applier.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteSnapshot {
    /// Current authoritative version.
    pub version: u64,
    /// Current serialized payload.
    pub payload: Option<String>,
}

/// The result of applying a command to the authoritative store.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppliedOutcome {
    /// New authoritative version after applying.
    pub new_version: u64,
    /// Payload now stored authoritatively.
    pub payload: Option<String>,
}

/// An error returned while applying a queued command.
#[derive(Debug, thiserror::Error)]
pub enum ApplyError {
    /// The authoritative store rejected the command.
    #[error("command rejected: {0}")]
    Rejected(String),
    /// The authoritative store was unreachable or failed.
    #[error("store unavailable: {0}")]
    Unavailable(String),
}

/// Applies queued commands to an authoritative store and reports remote state.
pub trait CommandApplier {
    /// Applies `command`, returning the new authoritative state on success.
    fn apply(&self, command: &QueuedCommand) -> std::result::Result<AppliedOutcome, ApplyError>;

    /// Returns the current authoritative state for `resource_key`, if known.
    fn remote_state(&self, resource_key: &str) -> Option<RemoteSnapshot>;
}

/// A file-backed authoritative store used as a local stand-in for a server.
///
/// Each resource is versioned; applying a command increments its version. This
/// makes `offline sync` meaningful even without a configured remote: queued
/// changes are committed to a durable journal and conflicts are detected when the
/// journal has advanced past a command's base version.
pub struct JournalApplier {
    path: PathBuf,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct JournalState {
    resources: HashMap<String, JournalEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct JournalEntry {
    version: u64,
    payload: Option<String>,
}

impl JournalApplier {
    /// Creates an applier backed by `path` (a JSON file).
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    /// Loads the journal state, tolerating a missing or unreadable file.
    fn load(&self) -> JournalState {
        let Ok(content) = fs::read_to_string(&self.path) else {
            return JournalState::default();
        };
        serde_json::from_str(&content).unwrap_or_default()
    }

    /// Persists the journal state.
    fn store(&self, state: &JournalState) -> std::result::Result<(), ApplyError> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)
                .map_err(|error| ApplyError::Unavailable(error.to_string()))?;
        }
        let content = serde_json::to_string_pretty(state)
            .map_err(|error| ApplyError::Unavailable(error.to_string()))?;
        fs::write(&self.path, content).map_err(|error| ApplyError::Unavailable(error.to_string()))
    }
}

impl CommandApplier for JournalApplier {
    fn apply(&self, command: &QueuedCommand) -> std::result::Result<AppliedOutcome, ApplyError> {
        let Some(resource_key) = &command.resource_key else {
            // Non-resource commands are accepted without journaling.
            return Ok(AppliedOutcome {
                new_version: 0,
                payload: command.payload.clone(),
            });
        };

        let mut state = self.load();
        let next_version = state
            .resources
            .get(resource_key)
            .map(|entry| entry.version + 1)
            .unwrap_or(1);
        state.resources.insert(
            resource_key.clone(),
            JournalEntry {
                version: next_version,
                payload: command.payload.clone(),
            },
        );
        self.store(&state)?;
        Ok(AppliedOutcome {
            new_version: next_version,
            payload: command.payload.clone(),
        })
    }

    fn remote_state(&self, resource_key: &str) -> Option<RemoteSnapshot> {
        self.load()
            .resources
            .get(resource_key)
            .map(|entry| RemoteSnapshot {
                version: entry.version,
                payload: entry.payload.clone(),
            })
    }
}

// ---------------------------------------------------------------------------
// Sync report
// ---------------------------------------------------------------------------

/// Summary of a reconciliation pass.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncReport {
    /// `true` if sync was skipped because the probe reported offline.
    pub skipped_offline: bool,
    /// Number of commands attempted.
    pub attempted: usize,
    /// Number successfully synced.
    pub synced: usize,
    /// Number that failed and remain queued.
    pub failed: usize,
    /// Number of conflicts detected.
    pub conflicts_detected: usize,
    /// Number of conflicts that could not be resolved automatically.
    pub unresolved: usize,
}

impl SyncReport {
    /// A short human-readable summary line.
    pub fn summary(&self) -> String {
        if self.skipped_offline {
            return "Offline: sync skipped; commands remain queued".to_string();
        }
        format!(
            "Synced {}/{} commands ({} failed, {} conflicts, {} unresolved)",
            self.synced, self.attempted, self.failed, self.conflicts_detected, self.unresolved
        )
    }
}

// ---------------------------------------------------------------------------
// Offline store
// ---------------------------------------------------------------------------

/// The facade tying together the queue, cache, and conflict log on disk.
pub struct OfflineStore {
    base_dir: PathBuf,
    queue: CommandQueue,
    cache: LocalCache,
    conflicts: Vec<ConflictRecord>,
    default_ttl_secs: u64,
}

impl OfflineStore {
    /// Default cache TTL (24 hours).
    pub const DEFAULT_TTL_SECS: u64 = 86_400;

    /// Opens (or initializes) the store under the platform cache directory.
    pub fn open() -> Result<Self> {
        let base_dir = dirs::cache_dir()
            .context("Failed to determine cache directory")?
            .join("legalis")
            .join("offline");
        Self::open_at(base_dir)
    }

    /// Opens (or initializes) the store under a specific base directory.
    pub fn open_at(base_dir: impl Into<PathBuf>) -> Result<Self> {
        let base_dir = base_dir.into();
        fs::create_dir_all(&base_dir)
            .with_context(|| format!("Failed to create offline dir: {}", base_dir.display()))?;
        let mut store = Self {
            base_dir,
            queue: CommandQueue::default(),
            cache: LocalCache::default(),
            conflicts: Vec::new(),
            default_ttl_secs: Self::DEFAULT_TTL_SECS,
        };
        store.load()?;
        Ok(store)
    }

    /// Sets the default TTL applied to cache writes during sync.
    pub fn set_default_ttl(&mut self, ttl_secs: u64) {
        self.default_ttl_secs = ttl_secs;
    }

    /// Path to the queue file.
    fn queue_path(&self) -> PathBuf {
        self.base_dir.join("queue.json")
    }

    /// Path to the cache file.
    fn cache_path(&self) -> PathBuf {
        self.base_dir.join("cache.json")
    }

    /// Path to the conflict log file.
    fn conflicts_path(&self) -> PathBuf {
        self.base_dir.join("conflicts.json")
    }

    /// Path to the default local journal (authoritative stand-in).
    pub fn journal_path(&self) -> PathBuf {
        self.base_dir.join("journal.json")
    }

    /// Loads persisted state, tolerating missing files.
    fn load(&mut self) -> Result<()> {
        self.queue = read_json_or_default(&self.queue_path())?;
        self.cache = read_json_or_default(&self.cache_path())?;
        self.conflicts = read_json_or_default(&self.conflicts_path())?;
        Ok(())
    }

    /// Persists all state to disk.
    pub fn save(&self) -> Result<()> {
        write_json(&self.queue_path(), &self.queue)?;
        write_json(&self.cache_path(), &self.cache)?;
        write_json(&self.conflicts_path(), &self.conflicts)?;
        Ok(())
    }

    /// Read-only access to the queue.
    pub fn queue(&self) -> &CommandQueue {
        &self.queue
    }

    /// Read-only access to the cache.
    pub fn cache(&self) -> &LocalCache {
        &self.cache
    }

    /// Read-only access to recorded conflicts.
    pub fn conflicts(&self) -> &[ConflictRecord] {
        &self.conflicts
    }

    /// Enqueues a command and persists the queue, returning its id.
    pub fn enqueue(&mut self, command: QueuedCommand) -> Result<String> {
        let id = command.id.clone();
        self.queue.entries.push(command);
        self.save()?;
        Ok(id)
    }

    /// Removes a command by id, returning whether it existed.
    pub fn dequeue(&mut self, id: &str) -> Result<bool> {
        let removed = self.queue.remove(id);
        if removed {
            self.save()?;
        }
        Ok(removed)
    }

    /// Clears the queue and persists.
    pub fn clear_queue(&mut self) -> Result<usize> {
        let count = self.queue.entries.len();
        self.queue.entries.clear();
        self.save()?;
        Ok(count)
    }

    /// Stores a value in the local cache and persists.
    pub fn cache_put(&mut self, key: &str, value: &str, version: u64, ttl_secs: u64) -> Result<()> {
        self.cache.put(key, value, version, ttl_secs);
        self.save()
    }

    /// Prunes expired cache records and persists.
    pub fn cache_prune(&mut self) -> Result<usize> {
        let pruned = self.cache.prune_expired();
        if pruned > 0 {
            self.save()?;
        }
        Ok(pruned)
    }

    /// Validates every queued command, updating statuses, and persists.
    ///
    /// Pending or previously failed commands become `Validated` when they pass
    /// and `Failed` (with a recorded reason) when they do not.
    pub fn validate_queue(&mut self) -> Result<Vec<(String, ValidationOutcome)>> {
        let mut results = Vec::with_capacity(self.queue.entries.len());
        for entry in &mut self.queue.entries {
            let outcome = validate_command(entry);
            if matches!(entry.status, QueueStatus::Pending | QueueStatus::Failed) {
                if outcome.valid {
                    entry.status = QueueStatus::Validated;
                    entry.last_error = None;
                } else {
                    entry.status = QueueStatus::Failed;
                    entry.last_error = outcome
                        .issues
                        .iter()
                        .find(|issue| issue.severity == ValidationSeverity::Error)
                        .map(|issue| issue.message.clone());
                }
            }
            results.push((entry.id.clone(), outcome));
        }
        self.save()?;
        Ok(results)
    }

    /// Resolves a recorded conflict by id with an explicit preference.
    ///
    /// Returns whether a matching unresolved conflict was found and resolved.
    pub fn resolve_conflict(&mut self, id: &str, prefer_local: bool) -> Result<bool> {
        let mut resolved = false;
        for record in &mut self.conflicts {
            if record.id == id && record.resolution == ConflictResolution::Unresolved {
                record.resolution = if prefer_local {
                    ConflictResolution::ResolvedLocal
                } else {
                    ConflictResolution::ResolvedRemote
                };
                // Reflect the decision in the authoritative cache snapshot.
                let chosen = if prefer_local {
                    record.local_payload.clone()
                } else {
                    record.remote_payload.clone()
                };
                if let Some(payload) = chosen {
                    let version = record.remote_version + 1;
                    self.cache.put(
                        &record.resource_key,
                        payload,
                        version,
                        self.default_ttl_secs,
                    );
                }
                // Clear any conflicted queue entry for the same command.
                for entry in &mut self.queue.entries {
                    if entry.id == record.command_id {
                        entry.status = QueueStatus::Synced;
                    }
                }
                resolved = true;
                break;
            }
        }
        if resolved {
            self.save()?;
        }
        Ok(resolved)
    }

    /// Reconciles the queue against an authoritative store when online.
    ///
    /// Offline (per `probe`): returns immediately with `skipped_offline = true`
    /// and leaves the queue untouched. Online: each syncable command is applied,
    /// with version-based conflict detection and resolution per `strategy`.
    pub fn sync(
        &mut self,
        applier: &dyn CommandApplier,
        probe: &dyn ConnectivityProbe,
        strategy: ConflictStrategy,
    ) -> Result<SyncReport> {
        let mut report = SyncReport::default();

        if !probe.is_online() {
            report.skipped_offline = true;
            return Ok(report);
        }

        // Process a detached copy so cache/conflict fields stay mutably borrowable.
        let mut entries = std::mem::take(&mut self.queue.entries);
        for entry in &mut entries {
            if !entry.status.is_syncable() {
                continue;
            }
            report.attempted += 1;
            self.sync_entry(entry, applier, strategy, &mut report);
        }
        self.queue.entries = entries;

        self.save()?;
        Ok(report)
    }

    /// Synchronizes a single entry, updating its status and the report.
    fn sync_entry(
        &mut self,
        entry: &mut QueuedCommand,
        applier: &dyn CommandApplier,
        strategy: ConflictStrategy,
        report: &mut SyncReport,
    ) {
        // Detect conflict via version comparison when we have enough context.
        if let (Some(resource_key), Some(base_version)) =
            (entry.resource_key.clone(), entry.base_version)
            && let Some(remote) = applier.remote_state(&resource_key)
            && remote.version != base_version
        {
            report.conflicts_detected += 1;
            self.resolve_sync_conflict(entry, applier, strategy, &resource_key, &remote, report);
            return;
        }

        // No conflict: apply directly.
        self.apply_entry(entry, applier, report);
    }

    /// Applies an entry and records the outcome.
    fn apply_entry(
        &mut self,
        entry: &mut QueuedCommand,
        applier: &dyn CommandApplier,
        report: &mut SyncReport,
    ) {
        match applier.apply(entry) {
            Ok(outcome) => {
                entry.status = QueueStatus::Synced;
                entry.last_error = None;
                if let Some(resource_key) = &entry.resource_key {
                    let payload = outcome.payload.unwrap_or_default();
                    self.cache.put(
                        resource_key,
                        payload,
                        outcome.new_version,
                        self.default_ttl_secs,
                    );
                }
                report.synced += 1;
            }
            Err(error) => {
                entry.status = QueueStatus::Failed;
                entry.attempts += 1;
                entry.last_error = Some(error.to_string());
                report.failed += 1;
            }
        }
    }

    /// Resolves a conflict for `entry` according to `strategy`.
    fn resolve_sync_conflict(
        &mut self,
        entry: &mut QueuedCommand,
        applier: &dyn CommandApplier,
        strategy: ConflictStrategy,
        resource_key: &str,
        remote: &RemoteSnapshot,
        report: &mut SyncReport,
    ) {
        let base_version = entry.base_version.unwrap_or(0);
        let mut record = ConflictRecord {
            id: uuid::Uuid::new_v4().to_string(),
            command_id: entry.id.clone(),
            resource_key: resource_key.to_string(),
            base_version,
            remote_version: remote.version,
            strategy,
            resolution: ConflictResolution::Unresolved,
            conflicting_paths: Vec::new(),
            detected_at: now_rfc3339(),
            local_payload: entry.payload.clone(),
            remote_payload: remote.payload.clone(),
            merged_payload: None,
        };

        match strategy {
            ConflictStrategy::LastWriterWins => {
                record.resolution = ConflictResolution::ResolvedLocal;
                self.conflicts.push(record);
                self.apply_entry(entry, applier, report);
            }
            ConflictStrategy::RemoteWins => {
                record.resolution = ConflictResolution::ResolvedRemote;
                self.conflicts.push(record);
                // Accept remote: drop the local change but mark it handled.
                let payload = remote.payload.clone().unwrap_or_default();
                self.cache
                    .put(resource_key, payload, remote.version, self.default_ttl_secs);
                entry.status = QueueStatus::Synced;
                entry.last_error = None;
                report.synced += 1;
            }
            ConflictStrategy::Merge => {
                let base = self.merge_base(resource_key, base_version);
                let local = parse_json_or_null(entry.payload.as_deref());
                let remote_value = parse_json_or_null(remote.payload.as_deref());
                let merge = merge_three_way(&base, &local, &remote_value);

                if merge.is_clean() {
                    let merged_payload = serde_json::to_string(&merge.merged).unwrap_or_default();
                    record.resolution = ConflictResolution::Merged;
                    record.merged_payload = Some(merged_payload.clone());
                    self.conflicts.push(record);

                    let mut merged_command = entry.clone();
                    merged_command.payload = Some(merged_payload);
                    match applier.apply(&merged_command) {
                        Ok(outcome) => {
                            entry.status = QueueStatus::Synced;
                            entry.payload = merged_command.payload;
                            entry.last_error = None;
                            let payload = outcome.payload.unwrap_or_default();
                            self.cache.put(
                                resource_key,
                                payload,
                                outcome.new_version,
                                self.default_ttl_secs,
                            );
                            report.synced += 1;
                        }
                        Err(error) => {
                            entry.status = QueueStatus::Failed;
                            entry.attempts += 1;
                            entry.last_error = Some(error.to_string());
                            report.failed += 1;
                        }
                    }
                } else {
                    record.resolution = ConflictResolution::Unresolved;
                    record.conflicting_paths = merge.conflicts;
                    self.conflicts.push(record);
                    entry.status = QueueStatus::Conflicted;
                    entry.last_error = Some("Unresolved merge conflict".to_string());
                    report.unresolved += 1;
                }
            }
        }
    }

    /// Returns the common-ancestor value for a merge from the local cache.
    ///
    /// Uses the cached snapshot only when its version matches the command's base
    /// version; otherwise falls back to an empty object.
    fn merge_base(&self, resource_key: &str, base_version: u64) -> Value {
        if let Some(record) = self.cache.records.get(resource_key)
            && record.version == base_version
            && let Ok(value) = serde_json::from_str::<Value>(&record.value)
        {
            return value;
        }
        Value::Object(serde_json::Map::new())
    }
}

/// Parses an optional JSON string, defaulting to `null` on absence or error.
fn parse_json_or_null(payload: Option<&str>) -> Value {
    payload
        .and_then(|text| serde_json::from_str(text).ok())
        .unwrap_or(Value::Null)
}

/// Reads and deserializes JSON from `path`, returning the default on absence.
fn read_json_or_default<T: serde::de::DeserializeOwned + Default>(path: &Path) -> Result<T> {
    if !path.exists() {
        return Ok(T::default());
    }
    let content =
        fs::read_to_string(path).with_context(|| format!("Failed to read {}", path.display()))?;
    if content.trim().is_empty() {
        return Ok(T::default());
    }
    serde_json::from_str(&content).with_context(|| format!("Failed to parse {}", path.display()))
}

/// Serializes `value` as pretty JSON and writes it to `path`.
fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let content = serde_json::to_string_pretty(value)?;
    fs::write(path, content).with_context(|| format!("Failed to write {}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_store() -> OfflineStore {
        let dir = std::env::temp_dir().join(format!("legalis-offline-{}", uuid::Uuid::new_v4()));
        OfflineStore::open_at(dir).expect("open store")
    }

    #[test]
    fn enqueue_and_filter() {
        let mut store = temp_store();
        let id = store
            .enqueue(QueuedCommand::new("verify", vec!["a.ldsl".to_string()]))
            .expect("enqueue");
        assert_eq!(store.queue().len(), 1);
        let pending = store.queue().filter(Some(QueueStatus::Pending));
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].id, id);
        assert!(store.dequeue(&id).expect("dequeue"));
        assert!(store.queue().is_empty());
    }

    #[test]
    fn queue_persists_across_reopen() {
        let dir = std::env::temp_dir().join(format!("legalis-offline-{}", uuid::Uuid::new_v4()));
        {
            let mut store = OfflineStore::open_at(&dir).expect("open");
            store
                .enqueue(QueuedCommand::new("publish", vec![]).with_resource("statute:x"))
                .expect("enqueue");
        }
        let reopened = OfflineStore::open_at(&dir).expect("reopen");
        assert_eq!(reopened.queue().len(), 1);
        assert_eq!(reopened.queue().entries[0].command, "publish");
    }

    #[test]
    fn cache_put_get_and_ttl() {
        let mut store = temp_store();
        store.cache_put("statute:1", "{}", 1, 0).expect("put");
        assert!(store.cache().get("statute:1").is_some());

        // An expired record (created in the past) is not returned.
        store.cache.records.insert(
            "statute:2".to_string(),
            CacheRecord {
                key: "statute:2".to_string(),
                value: "{}".to_string(),
                version: 1,
                created_at: now_secs().saturating_sub(100),
                ttl_secs: 10,
            },
        );
        assert!(store.cache().get("statute:2").is_none());
        assert_eq!(store.cache_prune().expect("prune"), 1);
    }

    #[test]
    fn validate_rejects_unknown_and_requires_resource() {
        let unknown = QueuedCommand::new("frobnicate", vec![]);
        let outcome = validate_command(&unknown);
        assert!(!outcome.valid);

        let publish_no_key = QueuedCommand::new("publish", vec![]);
        let outcome = validate_command(&publish_no_key);
        assert!(!outcome.valid, "mutating command needs a resource key");

        let valid = QueuedCommand::new("publish", vec![]).with_payload("{}", 3);
        let valid = valid.with_resource("statute:x");
        let outcome = validate_command(&valid);
        assert!(outcome.valid, "issues: {:?}", outcome.issues);
    }

    #[test]
    fn validate_rejects_bad_payload_json() {
        let bad = QueuedCommand::new("verify", vec![]);
        let bad = QueuedCommand {
            payload: Some("{not json".to_string()),
            ..bad
        };
        let outcome = validate_command(&bad);
        assert!(!outcome.valid);
    }

    #[test]
    fn validate_queue_updates_statuses() {
        let mut store = temp_store();
        store
            .enqueue(QueuedCommand::new("verify", vec!["a.ldsl".to_string()]))
            .expect("enqueue good");
        store
            .enqueue(QueuedCommand::new("nope", vec![]))
            .expect("enqueue bad");
        store.validate_queue().expect("validate");

        let validated = store.queue().filter(Some(QueueStatus::Validated));
        let failed = store.queue().filter(Some(QueueStatus::Failed));
        assert_eq!(validated.len(), 1);
        assert_eq!(failed.len(), 1);
        assert!(failed[0].last_error.is_some());
    }

    #[test]
    fn probes_report_expected_state() {
        assert!(AlwaysOnline.is_online());
        assert!(!AlwaysOffline.is_online());
        // An unroutable port should not connect within a short timeout.
        let probe = TcpProbe::new("127.0.0.1", 1, Duration::from_millis(50));
        assert!(!probe.is_online());
    }

    #[test]
    fn sync_skipped_when_offline() {
        let mut store = temp_store();
        store
            .enqueue(QueuedCommand::new("verify", vec![]))
            .expect("enqueue");
        let applier = JournalApplier::new(store.journal_path());
        let report = store
            .sync(&applier, &AlwaysOffline, ConflictStrategy::LastWriterWins)
            .expect("sync");
        assert!(report.skipped_offline);
        assert_eq!(store.queue().filter(Some(QueueStatus::Pending)).len(), 1);
    }

    #[test]
    fn sync_applies_pending_when_online() {
        let mut store = temp_store();
        store
            .enqueue(
                QueuedCommand::new("publish", vec![])
                    .with_resource("statute:apply")
                    .with_payload("{\"a\":1}", 0),
            )
            .expect("enqueue");
        let applier = JournalApplier::new(store.journal_path());
        let report = store
            .sync(&applier, &AlwaysOnline, ConflictStrategy::LastWriterWins)
            .expect("sync");
        assert_eq!(report.synced, 1);
        assert_eq!(report.failed, 0);
        assert_eq!(store.queue().filter(Some(QueueStatus::Synced)).len(), 1);
        assert!(store.cache().get("statute:apply").is_some());
    }

    #[test]
    fn sync_detects_conflict_and_applies_last_writer_wins() {
        let mut store = temp_store();
        let applier = JournalApplier::new(store.journal_path());
        // Pre-advance the remote so base_version (0) is stale.
        applier
            .apply(&QueuedCommand::new("publish", vec![]).with_resource("statute:c"))
            .expect("seed remote");

        store
            .enqueue(
                QueuedCommand::new("publish", vec![])
                    .with_resource("statute:c")
                    .with_payload("{\"local\":true}", 0),
            )
            .expect("enqueue");

        let report = store
            .sync(&applier, &AlwaysOnline, ConflictStrategy::LastWriterWins)
            .expect("sync");
        assert_eq!(report.conflicts_detected, 1);
        assert_eq!(report.synced, 1);
        assert_eq!(store.conflicts().len(), 1);
        assert_eq!(
            store.conflicts()[0].resolution,
            ConflictResolution::ResolvedLocal
        );
    }

    #[test]
    fn sync_remote_wins_drops_local() {
        let mut store = temp_store();
        let applier = JournalApplier::new(store.journal_path());
        applier
            .apply(
                &QueuedCommand::new("publish", vec![])
                    .with_resource("statute:r")
                    .with_payload("{\"remote\":true}", 0),
            )
            .expect("seed remote");

        store
            .enqueue(
                QueuedCommand::new("publish", vec![])
                    .with_resource("statute:r")
                    .with_payload("{\"local\":true}", 0),
            )
            .expect("enqueue");

        let report = store
            .sync(&applier, &AlwaysOnline, ConflictStrategy::RemoteWins)
            .expect("sync");
        assert_eq!(report.conflicts_detected, 1);
        assert_eq!(
            store.conflicts()[0].resolution,
            ConflictResolution::ResolvedRemote
        );
        let cached = store.cache().get("statute:r").expect("cached");
        assert!(cached.value.contains("remote"));
    }

    #[test]
    fn three_way_merge_combines_disjoint_changes() {
        let base = serde_json::json!({"title": "Act", "rate": 10});
        let local = serde_json::json!({"title": "Act", "rate": 10, "note": "local"});
        let remote = serde_json::json!({"title": "Revised Act", "rate": 10});
        let result = merge_three_way(&base, &local, &remote);
        assert!(result.is_clean(), "conflicts: {:?}", result.conflicts);
        assert_eq!(result.merged["title"], serde_json::json!("Revised Act"));
        assert_eq!(result.merged["note"], serde_json::json!("local"));
    }

    #[test]
    fn three_way_merge_reports_divergent_conflict() {
        let base = serde_json::json!({"rate": 10});
        let local = serde_json::json!({"rate": 15});
        let remote = serde_json::json!({"rate": 20});
        let result = merge_three_way(&base, &local, &remote);
        assert!(!result.is_clean());
        assert_eq!(result.conflicts, vec!["rate".to_string()]);
    }

    #[test]
    fn three_way_merge_prefers_single_changed_side() {
        let base = serde_json::json!({"x": 1});
        let local = serde_json::json!({"x": 2});
        let remote = serde_json::json!({"x": 1});
        let result = merge_three_way(&base, &local, &remote);
        assert!(result.is_clean());
        assert_eq!(result.merged["x"], serde_json::json!(2));
    }

    #[test]
    fn sync_merge_strategy_records_unresolved_conflict() {
        let mut store = temp_store();
        let applier = JournalApplier::new(store.journal_path());
        // Seed remote with a divergent value and version 1.
        applier
            .apply(
                &QueuedCommand::new("update", vec![])
                    .with_resource("statute:m")
                    .with_payload("{\"rate\":20}", 0),
            )
            .expect("seed remote");
        // Cache the common ancestor at base version 0.
        store
            .cache_put("statute:m", "{\"rate\":10}", 0, 0)
            .expect("seed base");

        store
            .enqueue(
                QueuedCommand::new("update", vec![])
                    .with_resource("statute:m")
                    .with_payload("{\"rate\":15}", 0),
            )
            .expect("enqueue");

        let report = store
            .sync(&applier, &AlwaysOnline, ConflictStrategy::Merge)
            .expect("sync");
        assert_eq!(report.conflicts_detected, 1);
        assert_eq!(report.unresolved, 1);
        assert_eq!(store.queue().filter(Some(QueueStatus::Conflicted)).len(), 1);
        assert!(!store.conflicts()[0].conflicting_paths.is_empty());
    }

    #[test]
    fn manual_conflict_resolution_clears_entry() {
        let mut store = temp_store();
        let applier = JournalApplier::new(store.journal_path());
        applier
            .apply(
                &QueuedCommand::new("update", vec![])
                    .with_resource("statute:res")
                    .with_payload("{\"rate\":20}", 0),
            )
            .expect("seed remote");
        store
            .cache_put("statute:res", "{\"rate\":10}", 0, 0)
            .expect("seed base");
        store
            .enqueue(
                QueuedCommand::new("update", vec![])
                    .with_resource("statute:res")
                    .with_payload("{\"rate\":15}", 0),
            )
            .expect("enqueue");
        store
            .sync(&applier, &AlwaysOnline, ConflictStrategy::Merge)
            .expect("sync");

        let conflict_id = store.conflicts()[0].id.clone();
        assert!(store.resolve_conflict(&conflict_id, true).expect("resolve"));
        assert_eq!(store.queue().filter(Some(QueueStatus::Conflicted)).len(), 0);
        assert_eq!(
            store.conflicts()[0].resolution,
            ConflictResolution::ResolvedLocal
        );
    }

    #[test]
    fn sync_records_apply_failure() {
        struct FailingApplier;
        impl CommandApplier for FailingApplier {
            fn apply(
                &self,
                _command: &QueuedCommand,
            ) -> std::result::Result<AppliedOutcome, ApplyError> {
                Err(ApplyError::Rejected("nope".to_string()))
            }
            fn remote_state(&self, _resource_key: &str) -> Option<RemoteSnapshot> {
                None
            }
        }

        let mut store = temp_store();
        store
            .enqueue(QueuedCommand::new("verify", vec![]))
            .expect("enqueue");
        let report = store
            .sync(
                &FailingApplier,
                &AlwaysOnline,
                ConflictStrategy::LastWriterWins,
            )
            .expect("sync");
        assert_eq!(report.failed, 1);
        let failed = store.queue().filter(Some(QueueStatus::Failed));
        assert_eq!(failed.len(), 1);
        assert_eq!(failed[0].attempts, 1);
        assert!(failed[0].last_error.is_some());
    }

    #[test]
    fn sync_report_summary_text() {
        let offline = SyncReport {
            skipped_offline: true,
            ..SyncReport::default()
        };
        assert!(offline.summary().contains("Offline"));
        let online = SyncReport {
            attempted: 2,
            synced: 2,
            ..SyncReport::default()
        };
        assert!(online.summary().contains("Synced 2/2"));
    }
}
