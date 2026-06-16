//! CLI audit logging.
//!
//! Records each invoked CLI operation to a tamper-evident, hash-chained audit
//! trail backed by `legalis-audit`'s JSONL storage. Every entry captures *who*
//! ran *what* command with which arguments, *when*, and the *outcome*
//! (success / failure / blocked-by-policy / blocked-by-compliance).
//!
//! The trail is the same primitive used for legal decision provenance, so CLI
//! operations land in a verifiable append-only log that can be queried and
//! integrity-checked later (see [`AuditLogger::verify_integrity`]).

use crate::paths;
use anyhow::{Context, Result};
use legalis_audit::{
    Actor, AuditRecord, AuditTrail, DecisionContext, DecisionResult, EvaluatedCondition, EventType,
};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use uuid::Uuid;

/// Synthetic statute id used to tag CLI-operation audit records.
const CLI_STATUTE_ID: &str = "legalis-cli/operation";

/// The outcome of a CLI operation, recorded in the audit trail.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationOutcome {
    /// The command completed successfully.
    Success,
    /// The command ran but returned an error.
    Failure,
    /// The command was blocked by enterprise policy.
    BlockedByPolicy,
    /// The command was blocked by compliance mode.
    BlockedByCompliance,
}

impl OperationOutcome {
    fn label(self) -> &'static str {
        match self {
            OperationOutcome::Success => "success",
            OperationOutcome::Failure => "failure",
            OperationOutcome::BlockedByPolicy => "blocked_by_policy",
            OperationOutcome::BlockedByCompliance => "blocked_by_compliance",
        }
    }

    fn succeeded(self) -> bool {
        matches!(self, OperationOutcome::Success)
    }
}

/// A description of a CLI operation to be recorded.
#[derive(Debug, Clone)]
pub struct OperationEntry {
    /// The subcommand name, e.g. `publish`.
    pub command: String,
    /// The (sanitized) arguments.
    pub args: Vec<String>,
    /// Outcome of the operation.
    pub outcome: OperationOutcome,
    /// Optional free-form detail (e.g. an error message or policy rule).
    pub detail: Option<String>,
}

impl OperationEntry {
    /// Creates a new operation entry.
    pub fn new(command: impl Into<String>, args: Vec<String>, outcome: OperationOutcome) -> Self {
        Self {
            command: command.into(),
            args,
            outcome,
            detail: None,
        }
    }

    /// Attaches a detail string (builder style).
    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }
}

/// Audit logger that appends CLI operations to a hash-chained JSONL trail.
pub struct AuditLogger {
    trail: AuditTrail,
    path: PathBuf,
    actor_id: String,
}

impl AuditLogger {
    /// Opens the default CLI audit trail (JSONL under the data directory).
    pub fn open() -> Result<Self> {
        let path = paths::audit_log_path()?;
        Self::open_at(path)
    }

    /// Opens (or creates) an audit trail at a specific JSONL path.
    pub fn open_at(path: impl Into<PathBuf>) -> Result<Self> {
        let path = path.into();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).with_context(|| {
                format!("Failed to create audit log directory: {}", parent.display())
            })?;
        }
        let trail = AuditTrail::with_jsonl_file(&path)
            .with_context(|| format!("Failed to open audit log: {}", path.display()))?;
        let actor_id = current_user();
        Ok(Self {
            trail,
            path,
            actor_id,
        })
    }

    /// Overrides the actor id (defaults to the current OS user).
    pub fn with_actor(mut self, actor_id: impl Into<String>) -> Self {
        self.actor_id = actor_id.into();
        self
    }

    /// The on-disk path of the audit trail.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Records a CLI operation, returning the new record's id.
    pub fn record(&mut self, entry: &OperationEntry) -> Result<Uuid> {
        let context = self.build_context(entry);
        let result = self.build_result(entry);
        // `AuditTrail::record` re-links the record into the storage's hash chain,
        // so we pass `None` as the previous hash and let the trail finalize it.
        let record = AuditRecord::new(
            EventType::SimulationRun,
            Actor::User {
                user_id: self.actor_id.clone(),
                role: "cli-operator".to_string(),
            },
            CLI_STATUTE_ID.to_string(),
            self.subject_for(&entry.command),
            context,
            result,
            None,
        );
        let id = self
            .trail
            .record(record)
            .context("Failed to append audit record")?;
        Ok(id)
    }

    /// Convenience: record a successful operation.
    pub fn record_success(&mut self, command: &str, args: &[String]) -> Result<Uuid> {
        self.record(&OperationEntry::new(
            command,
            args.to_vec(),
            OperationOutcome::Success,
        ))
    }

    /// Convenience: record a failed operation with a message.
    pub fn record_failure(
        &mut self,
        command: &str,
        args: &[String],
        message: &str,
    ) -> Result<Uuid> {
        self.record(
            &OperationEntry::new(command, args.to_vec(), OperationOutcome::Failure)
                .with_detail(message),
        )
    }

    /// Returns the total number of recorded operations.
    pub fn count(&self) -> usize {
        self.trail.count()
    }

    /// Returns all CLI-operation records, most useful for inspection/tests.
    pub fn all_operations(&self) -> Result<Vec<AuditRecord>> {
        self.trail
            .query_by_statute(CLI_STATUTE_ID)
            .context("Failed to query audit records")
    }

    /// Verifies the integrity of every record's hash chain link.
    ///
    /// Returns `Ok(true)` when all records pass their individual hash check.
    pub fn verify_integrity(&self) -> Result<bool> {
        let records = self
            .trail
            .query_by_statute(CLI_STATUTE_ID)
            .context("Failed to query audit records")?;
        Ok(records.iter().all(|record| record.verify()))
    }

    /// Builds the decision context (attributes + evaluated outcome) for an entry.
    fn build_context(&self, entry: &OperationEntry) -> DecisionContext {
        let mut attributes = HashMap::new();
        attributes.insert("command".to_string(), entry.command.clone());
        attributes.insert("args".to_string(), entry.args.join(" "));
        attributes.insert("outcome".to_string(), entry.outcome.label().to_string());
        if let Some(ref detail) = entry.detail {
            attributes.insert("detail".to_string(), detail.clone());
        }

        let evaluated = vec![EvaluatedCondition {
            description: format!("cli command '{}' executed", entry.command),
            result: entry.outcome.succeeded(),
            input_value: Some(entry.args.join(" ")),
            threshold: None,
        }];

        DecisionContext {
            attributes,
            metadata: HashMap::new(),
            evaluated_conditions: evaluated,
        }
    }

    /// Builds the decision result describing the operation outcome.
    fn build_result(&self, entry: &OperationEntry) -> DecisionResult {
        match entry.outcome {
            OperationOutcome::Success => {
                let mut parameters = HashMap::new();
                parameters.insert("command".to_string(), entry.command.clone());
                DecisionResult::Deterministic {
                    effect_applied: format!("executed '{}'", entry.command),
                    parameters,
                }
            }
            OperationOutcome::Failure => DecisionResult::Void {
                reason: entry
                    .detail
                    .clone()
                    .unwrap_or_else(|| format!("command '{}' failed", entry.command)),
            },
            OperationOutcome::BlockedByPolicy => DecisionResult::Void {
                reason: format!(
                    "command '{}' blocked by enterprise policy: {}",
                    entry.command,
                    entry.detail.as_deref().unwrap_or("policy violation")
                ),
            },
            OperationOutcome::BlockedByCompliance => DecisionResult::Void {
                reason: format!(
                    "command '{}' blocked by compliance mode: {}",
                    entry.command,
                    entry.detail.as_deref().unwrap_or("sensitive operation")
                ),
            },
        }
    }

    /// Derives a stable subject id for a command name (deterministic UUIDv... not
    /// available without extra deps, so we hash the name into a v4-shaped UUID).
    fn subject_for(&self, command: &str) -> Uuid {
        // Stable per-command identifier so records about the same command share a
        // subject id. We fold the bytes into a 16-byte array deterministically.
        let mut bytes = [0u8; 16];
        for (index, byte) in command.bytes().enumerate() {
            bytes[index % 16] ^= byte.wrapping_add(index as u8);
        }
        Uuid::from_bytes(bytes)
    }
}

/// Returns the current OS username, or `"unknown"`.
fn current_user() -> String {
    whoami::username().unwrap_or_else(|_| "unknown".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_path() -> PathBuf {
        std::env::temp_dir().join(format!("legalis-audit-{}.jsonl", Uuid::new_v4()))
    }

    #[test]
    fn test_record_and_count() {
        let path = temp_path();
        {
            let mut logger = AuditLogger::open_at(&path)
                .expect("open")
                .with_actor("tester");
            logger
                .record_success("verify", &["--input".into(), "a.ldsl".into()])
                .expect("record success");
            logger
                .record_failure("publish", &["--input".into(), "b.ldsl".into()], "boom")
                .expect("record failure");
            assert_eq!(logger.count(), 2);
        }
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_persistence_across_open() {
        let path = temp_path();
        {
            let mut logger = AuditLogger::open_at(&path).expect("open");
            logger.record_success("init", &[]).expect("record");
        }
        {
            let logger = AuditLogger::open_at(&path).expect("reopen");
            let ops = logger.all_operations().expect("ops");
            assert_eq!(ops.len(), 1);
        }
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_integrity_holds() {
        let path = temp_path();
        {
            let mut logger = AuditLogger::open_at(&path).expect("open");
            for i in 0..5 {
                logger
                    .record_success("lint", &[format!("file{i}.ldsl")])
                    .expect("record");
            }
            assert!(logger.verify_integrity().expect("verify"));
        }
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_blocked_outcomes_recorded() {
        let path = temp_path();
        {
            let mut logger = AuditLogger::open_at(&path).expect("open");
            logger
                .record(
                    &OperationEntry::new(
                        "uninstall",
                        vec!["--statute-id".into(), "x".into()],
                        OperationOutcome::BlockedByCompliance,
                    )
                    .with_detail("destructive op disabled"),
                )
                .expect("record blocked");
            let ops = logger.all_operations().expect("ops");
            assert_eq!(ops.len(), 1);
            // The result should be a Void with the blocking reason.
            match &ops[0].result {
                DecisionResult::Void { reason } => {
                    assert!(reason.contains("compliance"));
                }
                other => panic!("expected Void, got {other:?}"),
            }
        }
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_subject_is_stable_per_command() {
        let path = temp_path();
        let logger = AuditLogger::open_at(&path).expect("open");
        assert_eq!(logger.subject_for("verify"), logger.subject_for("verify"));
        assert_ne!(logger.subject_for("verify"), logger.subject_for("publish"));
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_outcome_labels() {
        assert_eq!(OperationOutcome::Success.label(), "success");
        assert_eq!(OperationOutcome::Failure.label(), "failure");
        assert_eq!(
            OperationOutcome::BlockedByPolicy.label(),
            "blocked_by_policy"
        );
        assert_eq!(
            OperationOutcome::BlockedByCompliance.label(),
            "blocked_by_compliance"
        );
        assert!(OperationOutcome::Success.succeeded());
        assert!(!OperationOutcome::Failure.succeeded());
    }
}
