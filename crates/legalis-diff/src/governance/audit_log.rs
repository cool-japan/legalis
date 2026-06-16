//! Enterprise audit logs with retention policies.
//!
//! An [`EnterpriseAuditLog`] is an append-only, **SHA-256 hash-chained** log:
//! each [`EnterpriseAuditEntry`] embeds the hash of the previous entry, so any
//! modification, reordering or deletion of an interior entry is detectable via
//! [`EnterpriseAuditLog::verify_integrity`]. On top of that it adds enterprise
//! retention features:
//!
//! - [`RetentionPolicy`] — purge by maximum age and/or maximum entry count,
//!   with a minimum-retention floor and a global freeze.
//! - **Legal holds** — pin specific resources so their entries are never purged.
//! - **Verifiable purge** — retention removes only a contiguous prefix of the
//!   oldest entries and records a *checkpoint hash*, so the remaining chain stays
//!   verifiable from the checkpoint onward.
//! - **Querying & export** — filter by actor/action/resource/severity/outcome/
//!   time range and export to CSV.
//!
//! This is intentionally distinct from [`crate::enterprise::AuditTrail`] (a
//! simple unchained list), [`crate::security::AuditTrail`] (a non-cryptographic
//! `DefaultHasher` chain) and [`crate::audit`] (change lifecycle/attribution).
//!
//! # Example
//!
//! ```
//! use legalis_diff::governance::audit_log::{
//!     AuditEvent, AuditOutcome, EnterpriseAuditLog,
//! };
//!
//! let mut log = EnterpriseAuditLog::new();
//! log.append(AuditEvent::new("alice", "diff:write", "statute:tax-2026", AuditOutcome::Success));
//! log.append(AuditEvent::new("bob", "diff:delete", "statute:tax-2026", AuditOutcome::Denied));
//! assert!(log.verify_integrity().ok);
//! assert_eq!(log.len(), 2);
//! ```

use crate::governance::{glob_match, sha256_parts};
use crate::{DiffError, DiffResult};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashSet};

/// The genesis previous-hash for the first entry (and empty checkpoint).
fn genesis_hash() -> String {
    "0".repeat(64)
}

/// Severity of an audit event (ordered: `Info` < `Notice` < `Warning` < `Critical`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum AuditSeverity {
    /// Routine informational event.
    Info,
    /// Noteworthy but not abnormal.
    Notice,
    /// Abnormal but non-fatal.
    Warning,
    /// Security-critical event.
    Critical,
}

impl AuditSeverity {
    /// A short label.
    pub fn label(self) -> &'static str {
        match self {
            Self::Info => "INFO",
            Self::Notice => "NOTICE",
            Self::Warning => "WARNING",
            Self::Critical => "CRITICAL",
        }
    }
}

/// The outcome of an audited operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuditOutcome {
    /// Operation succeeded.
    Success,
    /// Operation failed (error).
    Failure,
    /// Operation was denied by access control.
    Denied,
}

impl AuditOutcome {
    /// A short label.
    pub fn label(self) -> &'static str {
        match self {
            Self::Success => "SUCCESS",
            Self::Failure => "FAILURE",
            Self::Denied => "DENIED",
        }
    }
}

/// An event to be appended to an [`EnterpriseAuditLog`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditEvent {
    /// Who performed the action.
    pub actor: String,
    /// The action performed (free-form, e.g. `diff:write`).
    pub action: String,
    /// The resource acted upon (e.g. a statute id).
    pub resource: String,
    /// The outcome.
    pub outcome: AuditOutcome,
    /// Severity (defaults to [`AuditSeverity::Info`]).
    pub severity: AuditSeverity,
    /// Human-readable message.
    pub message: String,
    /// Structured metadata (ordered for deterministic hashing).
    pub metadata: BTreeMap<String, String>,
}

impl AuditEvent {
    /// Creates an event with [`AuditSeverity::Info`] and no message/metadata.
    pub fn new(
        actor: impl Into<String>,
        action: impl Into<String>,
        resource: impl Into<String>,
        outcome: AuditOutcome,
    ) -> Self {
        Self {
            actor: actor.into(),
            action: action.into(),
            resource: resource.into(),
            outcome,
            severity: AuditSeverity::Info,
            message: String::new(),
            metadata: BTreeMap::new(),
        }
    }

    /// Sets the severity.
    #[must_use]
    pub fn with_severity(mut self, severity: AuditSeverity) -> Self {
        self.severity = severity;
        self
    }

    /// Sets the message.
    #[must_use]
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }

    /// Adds a metadata key/value.
    #[must_use]
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

/// A committed audit entry, linked into the hash chain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnterpriseAuditEntry {
    /// Monotonic sequence number (stable across purges).
    pub seq: u64,
    /// When the event occurred.
    pub timestamp: DateTime<Utc>,
    /// Who performed the action.
    pub actor: String,
    /// The action performed.
    pub action: String,
    /// The resource acted upon.
    pub resource: String,
    /// The outcome.
    pub outcome: AuditOutcome,
    /// Severity.
    pub severity: AuditSeverity,
    /// Human-readable message.
    pub message: String,
    /// Structured metadata.
    pub metadata: BTreeMap<String, String>,
    /// Hash of the previous entry (or the checkpoint/genesis hash).
    pub prev_hash: String,
    /// This entry's hash.
    pub entry_hash: String,
}

impl EnterpriseAuditEntry {
    fn digest(seq: u64, timestamp: DateTime<Utc>, event: &AuditEvent, prev_hash: &str) -> String {
        let mut meta = String::new();
        for (k, v) in &event.metadata {
            meta.push_str(k);
            meta.push('=');
            meta.push_str(v);
            meta.push('\u{1e}');
        }
        sha256_parts(&[
            &seq.to_le_bytes(),
            &timestamp.timestamp().to_le_bytes(),
            event.actor.as_bytes(),
            event.action.as_bytes(),
            event.resource.as_bytes(),
            event.outcome.label().as_bytes(),
            event.severity.label().as_bytes(),
            event.message.as_bytes(),
            meta.as_bytes(),
            prev_hash.as_bytes(),
        ])
    }

    fn recompute(&self) -> String {
        let event = AuditEvent {
            actor: self.actor.clone(),
            action: self.action.clone(),
            resource: self.resource.clone(),
            outcome: self.outcome,
            severity: self.severity,
            message: self.message.clone(),
            metadata: self.metadata.clone(),
        };
        Self::digest(self.seq, self.timestamp, &event, &self.prev_hash)
    }
}

/// A retention policy for an audit log.
///
/// Durations are stored as whole seconds so the policy is trivially
/// serializable; helper builders accept [`chrono::Duration`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetentionPolicy {
    /// Maximum age (seconds) before an entry becomes purge-eligible.
    pub max_age_seconds: Option<i64>,
    /// Maximum number of entries to retain (oldest beyond this are purgeable).
    pub max_entries: Option<usize>,
    /// Minimum age (seconds) below which an entry is never purged.
    pub min_retention_seconds: Option<i64>,
    /// When set, no entries are purged at all (global hold).
    pub frozen: bool,
}

impl RetentionPolicy {
    /// An empty policy that purges nothing.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the maximum retention age.
    #[must_use]
    pub fn with_max_age(mut self, age: Duration) -> Self {
        self.max_age_seconds = Some(age.num_seconds());
        self
    }

    /// Sets the maximum entry count.
    #[must_use]
    pub fn with_max_entries(mut self, count: usize) -> Self {
        self.max_entries = Some(count);
        self
    }

    /// Sets the minimum-retention floor.
    #[must_use]
    pub fn with_min_retention(mut self, age: Duration) -> Self {
        self.min_retention_seconds = Some(age.num_seconds());
        self
    }

    /// Freezes the log (purges nothing).
    #[must_use]
    pub fn frozen(mut self) -> Self {
        self.frozen = true;
        self
    }

    fn max_age(&self) -> Option<Duration> {
        self.max_age_seconds.map(Duration::seconds)
    }

    fn min_retention(&self) -> Option<Duration> {
        self.min_retention_seconds.map(Duration::seconds)
    }
}

/// The result of [`EnterpriseAuditLog::verify_integrity`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntegrityReport {
    /// `true` if the chain is intact.
    pub ok: bool,
    /// Number of entries checked.
    pub checked: usize,
    /// Sequence number of the first broken entry, if any.
    pub first_broken_seq: Option<u64>,
}

/// The result of [`EnterpriseAuditLog::enforce_retention`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PurgeSummary {
    /// Number of entries removed.
    pub purged: usize,
    /// Number of entries retained.
    pub retained: usize,
    /// Number of entries that were purge-eligible by age but kept by a legal hold.
    pub held_back: usize,
}

/// A query over an audit log.
#[derive(Debug, Clone, Default)]
pub struct AuditQuery {
    /// Exact actor match.
    pub actor: Option<String>,
    /// Action glob pattern.
    pub action: Option<String>,
    /// Resource glob pattern.
    pub resource: Option<String>,
    /// Minimum severity.
    pub min_severity: Option<AuditSeverity>,
    /// Exact outcome match.
    pub outcome: Option<AuditOutcome>,
    /// Inclusive start of time range.
    pub start: Option<DateTime<Utc>>,
    /// Inclusive end of time range.
    pub end: Option<DateTime<Utc>>,
}

impl AuditQuery {
    /// A query matching everything.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filters by exact actor.
    #[must_use]
    pub fn actor(mut self, actor: impl Into<String>) -> Self {
        self.actor = Some(actor.into());
        self
    }

    /// Filters by action glob.
    #[must_use]
    pub fn action(mut self, pattern: impl Into<String>) -> Self {
        self.action = Some(pattern.into());
        self
    }

    /// Filters by resource glob.
    #[must_use]
    pub fn resource(mut self, pattern: impl Into<String>) -> Self {
        self.resource = Some(pattern.into());
        self
    }

    /// Filters by minimum severity.
    #[must_use]
    pub fn min_severity(mut self, severity: AuditSeverity) -> Self {
        self.min_severity = Some(severity);
        self
    }

    /// Filters by exact outcome.
    #[must_use]
    pub fn outcome(mut self, outcome: AuditOutcome) -> Self {
        self.outcome = Some(outcome);
        self
    }

    /// Filters by inclusive time range.
    #[must_use]
    pub fn between(mut self, start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        self.start = Some(start);
        self.end = Some(end);
        self
    }

    fn matches(&self, entry: &EnterpriseAuditEntry) -> bool {
        if self.actor.as_ref().is_some_and(|a| &entry.actor != a) {
            return false;
        }
        if self
            .action
            .as_ref()
            .is_some_and(|p| !glob_match(p, &entry.action))
        {
            return false;
        }
        if self
            .resource
            .as_ref()
            .is_some_and(|p| !glob_match(p, &entry.resource))
        {
            return false;
        }
        if self.min_severity.is_some_and(|min| entry.severity < min) {
            return false;
        }
        if self.outcome.is_some_and(|o| entry.outcome != o) {
            return false;
        }
        if self.start.is_some_and(|s| entry.timestamp < s) {
            return false;
        }
        if self.end.is_some_and(|e| entry.timestamp > e) {
            return false;
        }
        true
    }
}

/// A tamper-evident, hash-chained enterprise audit log.
#[derive(Debug, Clone)]
pub struct EnterpriseAuditLog {
    entries: Vec<EnterpriseAuditEntry>,
    next_seq: u64,
    last_hash: String,
    checkpoint_hash: String,
    retention: RetentionPolicy,
    legal_holds: HashSet<String>,
}

impl Default for EnterpriseAuditLog {
    fn default() -> Self {
        Self::new()
    }
}

impl EnterpriseAuditLog {
    /// Creates an empty log with no retention policy.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            next_seq: 0,
            last_hash: genesis_hash(),
            checkpoint_hash: genesis_hash(),
            retention: RetentionPolicy::new(),
            legal_holds: HashSet::new(),
        }
    }

    /// Sets the retention policy.
    #[must_use]
    pub fn with_retention(mut self, retention: RetentionPolicy) -> Self {
        self.retention = retention;
        self
    }

    /// Replaces the retention policy in place.
    pub fn set_retention(&mut self, retention: RetentionPolicy) {
        self.retention = retention;
    }

    /// Appends an event timestamped now, returning the committed entry.
    pub fn append(&mut self, event: AuditEvent) -> &EnterpriseAuditEntry {
        self.append_at(event, Utc::now())
    }

    /// Appends an event with an explicit timestamp (for import/backfill/tests).
    pub fn append_at(
        &mut self,
        event: AuditEvent,
        timestamp: DateTime<Utc>,
    ) -> &EnterpriseAuditEntry {
        let seq = self.next_seq;
        let prev_hash = self.last_hash.clone();
        let entry_hash = EnterpriseAuditEntry::digest(seq, timestamp, &event, &prev_hash);
        let entry = EnterpriseAuditEntry {
            seq,
            timestamp,
            actor: event.actor,
            action: event.action,
            resource: event.resource,
            outcome: event.outcome,
            severity: event.severity,
            message: event.message,
            metadata: event.metadata,
            prev_hash,
            entry_hash: entry_hash.clone(),
        };
        self.next_seq += 1;
        self.last_hash = entry_hash;
        self.entries.push(entry);
        let idx = self.entries.len() - 1;
        &self.entries[idx]
    }

    /// The committed entries.
    pub fn entries(&self) -> &[EnterpriseAuditEntry] {
        &self.entries
    }

    /// The number of committed entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns `true` if the log is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Places a legal hold on a resource (its entries are never purged).
    pub fn place_legal_hold(&mut self, resource: impl Into<String>) {
        self.legal_holds.insert(resource.into());
    }

    /// Releases a legal hold on a resource.
    pub fn release_legal_hold(&mut self, resource: &str) {
        self.legal_holds.remove(resource);
    }

    /// Returns `true` if a resource is under legal hold.
    pub fn is_on_hold(&self, resource: &str) -> bool {
        self.legal_holds.contains(resource)
    }

    /// Verifies the integrity of the hash chain from the checkpoint forward.
    pub fn verify_integrity(&self) -> IntegrityReport {
        let mut prev = self.checkpoint_hash.clone();
        for entry in &self.entries {
            if entry.prev_hash != prev || entry.recompute() != entry.entry_hash {
                return IntegrityReport {
                    ok: false,
                    checked: self.entries.len(),
                    first_broken_seq: Some(entry.seq),
                };
            }
            prev = entry.entry_hash.clone();
        }
        IntegrityReport {
            ok: true,
            checked: self.entries.len(),
            first_broken_seq: None,
        }
    }

    /// Enforces the retention policy at time `now`, purging a contiguous prefix
    /// of the oldest purge-eligible entries.
    ///
    /// An entry is purge-eligible when it is older than `max_age` (or beyond the
    /// `max_entries` count), is not within the `min_retention` floor, and is not
    /// under a legal hold. Because only a contiguous prefix is removed (to keep
    /// the remaining chain verifiable), a held or too-young entry also pins
    /// every entry after it.
    pub fn enforce_retention(&mut self, now: DateTime<Utc>) -> PurgeSummary {
        if self.retention.frozen {
            return PurgeSummary {
                purged: 0,
                retained: self.entries.len(),
                held_back: self.count_held_expired(now),
            };
        }
        let len = self.entries.len();
        let max_age = self.retention.max_age();
        let min_retention = self.retention.min_retention();
        let max_entries = self.retention.max_entries;

        let mut drop_count = 0usize;
        while drop_count < len {
            let entry = &self.entries[drop_count];
            let age = now.signed_duration_since(entry.timestamp);
            // Floor: too young to ever purge.
            if min_retention.is_some_and(|min_ret| age < min_ret) {
                break;
            }
            // Legal hold pins this (and everything after it).
            if self.legal_holds.contains(&entry.resource) {
                break;
            }
            let age_expired = max_age.map(|m| age > m).unwrap_or(false);
            let over_count = max_entries.map(|m| (len - drop_count) > m).unwrap_or(false);
            if age_expired || over_count {
                drop_count += 1;
            } else {
                break;
            }
        }

        if drop_count > 0 {
            // Advance the checkpoint to the last purged entry's hash so the
            // remaining chain still verifies.
            self.checkpoint_hash = self.entries[drop_count - 1].entry_hash.clone();
            self.entries.drain(0..drop_count);
        }

        PurgeSummary {
            purged: drop_count,
            retained: self.entries.len(),
            held_back: self.count_held_expired(now),
        }
    }

    fn count_held_expired(&self, now: DateTime<Utc>) -> usize {
        let max_age = self.retention.max_age();
        self.entries
            .iter()
            .filter(|e| {
                self.legal_holds.contains(&e.resource)
                    && max_age
                        .map(|m| now.signed_duration_since(e.timestamp) > m)
                        .unwrap_or(false)
            })
            .count()
    }

    /// Returns entries matching a query (in chain order).
    pub fn query(&self, query: &AuditQuery) -> Vec<&EnterpriseAuditEntry> {
        self.entries.iter().filter(|e| query.matches(e)).collect()
    }

    /// Exports all entries to CSV (RFC 4180 quoting).
    pub fn export_csv(&self) -> String {
        let mut out =
            String::from("seq,timestamp,actor,action,resource,outcome,severity,message\n");
        for entry in &self.entries {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{}\n",
                entry.seq,
                csv_escape(&entry.timestamp.to_rfc3339()),
                csv_escape(&entry.actor),
                csv_escape(&entry.action),
                csv_escape(&entry.resource),
                entry.outcome.label(),
                entry.severity.label(),
                csv_escape(&entry.message),
            ));
        }
        out
    }

    /// Serialises the entries to pretty JSON.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::SerializationError`] if serialisation fails.
    pub fn to_json(&self) -> DiffResult<String> {
        serde_json::to_string_pretty(&self.entries)
            .map_err(|e| DiffError::SerializationError(e.to_string()))
    }
}

/// Escapes a CSV field, quoting when it contains a comma, quote or newline.
fn csv_escape(field: &str) -> String {
    if field.contains(',') || field.contains('"') || field.contains('\n') {
        format!("\"{}\"", field.replace('"', "\"\""))
    } else {
        field.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn event(actor: &str, action: &str, resource: &str, outcome: AuditOutcome) -> AuditEvent {
        AuditEvent::new(actor, action, resource, outcome)
    }

    #[test]
    fn test_append_and_chain_links() {
        let mut log = EnterpriseAuditLog::new();
        log.append(event("alice", "diff:read", "s1", AuditOutcome::Success));
        log.append(event("bob", "diff:write", "s1", AuditOutcome::Success));
        assert_eq!(log.len(), 2);
        let entries = log.entries();
        assert_eq!(entries[0].prev_hash, "0".repeat(64));
        assert_eq!(entries[1].prev_hash, entries[0].entry_hash);
        assert_eq!(entries[0].seq, 0);
        assert_eq!(entries[1].seq, 1);
    }

    #[test]
    fn test_integrity_ok() {
        let mut log = EnterpriseAuditLog::new();
        for i in 0..5 {
            log.append(event(
                "alice",
                "diff:read",
                &format!("s{i}"),
                AuditOutcome::Success,
            ));
        }
        let report = log.verify_integrity();
        assert!(report.ok);
        assert_eq!(report.checked, 5);
        assert!(report.first_broken_seq.is_none());
    }

    #[test]
    fn test_tamper_detection() {
        let mut log = EnterpriseAuditLog::new();
        log.append(event("alice", "diff:read", "s1", AuditOutcome::Success));
        log.append(event("bob", "diff:write", "s2", AuditOutcome::Success));
        log.append(event("carol", "diff:delete", "s3", AuditOutcome::Denied));
        // Tamper with the middle entry.
        log.entries[1].actor = "mallory".to_string();
        let report = log.verify_integrity();
        assert!(!report.ok);
        assert_eq!(report.first_broken_seq, Some(1));
    }

    #[test]
    fn test_retention_by_age() {
        let mut log = EnterpriseAuditLog::new()
            .with_retention(RetentionPolicy::new().with_max_age(Duration::days(30)));
        let now = Utc::now();
        log.append_at(
            event("a", "x", "old", AuditOutcome::Success),
            now - Duration::days(60),
        );
        log.append_at(
            event("a", "x", "recent", AuditOutcome::Success),
            now - Duration::days(5),
        );
        let summary = log.enforce_retention(now);
        assert_eq!(summary.purged, 1);
        assert_eq!(summary.retained, 1);
        assert_eq!(log.entries()[0].resource, "recent");
        // Chain still verifies from the advanced checkpoint.
        assert!(log.verify_integrity().ok);
    }

    #[test]
    fn test_retention_by_count() {
        let mut log =
            EnterpriseAuditLog::new().with_retention(RetentionPolicy::new().with_max_entries(3));
        let now = Utc::now();
        for i in 0..6 {
            log.append_at(
                event("a", "x", &format!("s{i}"), AuditOutcome::Success),
                now - Duration::minutes(60 - i as i64),
            );
        }
        let summary = log.enforce_retention(now);
        assert_eq!(summary.purged, 3);
        assert_eq!(log.len(), 3);
        assert_eq!(log.entries()[0].resource, "s3");
        assert!(log.verify_integrity().ok);
    }

    #[test]
    fn test_min_retention_floor() {
        let mut log = EnterpriseAuditLog::new().with_retention(
            RetentionPolicy::new()
                .with_max_entries(1)
                .with_min_retention(Duration::days(7)),
        );
        let now = Utc::now();
        // Both entries younger than the 7-day floor -> nothing purged despite count.
        log.append_at(
            event("a", "x", "s0", AuditOutcome::Success),
            now - Duration::days(1),
        );
        log.append_at(
            event("a", "x", "s1", AuditOutcome::Success),
            now - Duration::hours(1),
        );
        let summary = log.enforce_retention(now);
        assert_eq!(summary.purged, 0);
        assert_eq!(log.len(), 2);
    }

    #[test]
    fn test_legal_hold_pins_entries() {
        let mut log = EnterpriseAuditLog::new()
            .with_retention(RetentionPolicy::new().with_max_age(Duration::days(1)));
        let now = Utc::now();
        log.append_at(
            event("a", "x", "held", AuditOutcome::Success),
            now - Duration::days(10),
        );
        log.append_at(
            event("a", "x", "free", AuditOutcome::Success),
            now - Duration::days(10),
        );
        log.place_legal_hold("held");
        assert!(log.is_on_hold("held"));
        let summary = log.enforce_retention(now);
        // The held entry is first, so it pins the whole log.
        assert_eq!(summary.purged, 0);
        assert_eq!(summary.held_back, 1);

        // Release and re-run: now both old entries purge.
        log.release_legal_hold("held");
        let summary2 = log.enforce_retention(now);
        assert_eq!(summary2.purged, 2);
    }

    #[test]
    fn test_frozen_policy_purges_nothing() {
        let mut log = EnterpriseAuditLog::new().with_retention(
            RetentionPolicy::new()
                .with_max_age(Duration::days(1))
                .frozen(),
        );
        let now = Utc::now();
        log.append_at(
            event("a", "x", "old", AuditOutcome::Success),
            now - Duration::days(100),
        );
        let summary = log.enforce_retention(now);
        assert_eq!(summary.purged, 0);
        assert_eq!(log.len(), 1);
    }

    #[test]
    fn test_query_filters() {
        let mut log = EnterpriseAuditLog::new();
        log.append(
            event("alice", "diff:read", "statute:tax", AuditOutcome::Success)
                .with_severity(AuditSeverity::Info),
        );
        log.append(
            event("bob", "diff:delete", "statute:tax", AuditOutcome::Denied)
                .with_severity(AuditSeverity::Critical),
        );
        log.append(
            event(
                "alice",
                "diff:write",
                "statute:labour",
                AuditOutcome::Success,
            )
            .with_severity(AuditSeverity::Warning),
        );

        assert_eq!(log.query(&AuditQuery::new().actor("alice")).len(), 2);
        assert_eq!(log.query(&AuditQuery::new().action("diff:*")).len(), 3);
        assert_eq!(
            log.query(&AuditQuery::new().resource("statute:tax")).len(),
            2
        );
        assert_eq!(
            log.query(&AuditQuery::new().min_severity(AuditSeverity::Warning))
                .len(),
            2
        );
        assert_eq!(
            log.query(&AuditQuery::new().outcome(AuditOutcome::Denied))
                .len(),
            1
        );
    }

    #[test]
    fn test_csv_export_escaping() {
        let mut log = EnterpriseAuditLog::new();
        log.append(
            event("alice", "diff:write", "s1", AuditOutcome::Success)
                .with_message("changed, with comma"),
        );
        let csv = log.export_csv();
        assert!(csv.contains("\"changed, with comma\""));
        assert!(csv.starts_with("seq,timestamp,actor"));
        let json = log.to_json().unwrap();
        assert!(json.contains("\"actor\": \"alice\""));
    }
}
