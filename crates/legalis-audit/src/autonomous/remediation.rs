//! Rule-based auto-remediation.
//!
//! The [`RemediationEngine`] turns [`MonitorFinding`]s raised by the
//! [`crate::autonomous::monitor`] into concrete, auditable [`RemediationAction`]s
//! via a set of declarative [`RemediationRule`]s. It follows a strict
//! **dry-run / apply** model:
//!
//! - In [`ExecutionMode::DryRun`] (the default) every matched action is
//!   *planned* and recorded as [`ActionStatus::Planned`] — nothing is executed.
//! - In [`ExecutionMode::Apply`] each planned action is executed through a
//!   pluggable [`RemediationExecutor`] and the result (`Applied` / `Failed` /
//!   `Skipped`) is captured.
//!
//! Every remediation pass produces an immutable [`RemediationRecord`] (a small
//! hash-chained log of what was decided and done), so the act of remediating is
//! itself auditable — a hard requirement for autonomous compliance.

use crate::autonomous::monitor::{MonitorFinding, MonitorSeverity, MonitoredMetric};
use crate::quantum::{sha256, to_hex};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// How the engine treats matched actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ExecutionMode {
    /// Plan actions only; never execute. The safe default.
    #[default]
    DryRun,
    /// Execute matched actions through the configured executor.
    Apply,
}

/// The kind of remediation an action performs.
///
/// These are *abstract* effects; how (or whether) they are carried out is the
/// job of a [`RemediationExecutor`]. Keeping them abstract is what lets the
/// engine stay pure and fully testable while still modelling real remediations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RemediationKind {
    /// Tighten the threshold of an audit policy by an absolute delta.
    TightenPolicyThreshold {
        /// The metric whose policy threshold should be tightened.
        metric: MonitoredMetric,
        /// Absolute amount to tighten the threshold by.
        delta: f64,
    },
    /// Increase the sampling/verification rate to `new_rate` in `[0, 1]`.
    IncreaseSamplingRate {
        /// Target sampling rate.
        new_rate: f64,
    },
    /// Raise an alert / notification at the given severity.
    RaiseAlert {
        /// Severity to escalate to.
        severity: MonitorSeverity,
    },
    /// Quarantine a subject/statute pending human review.
    Quarantine {
        /// The scope identifier (subject id, statute id, ...).
        scope: String,
    },
    /// Trigger a fresh integrity re-verification of the trail.
    TriggerIntegrityCheck,
    /// Open a remediation ticket in an external system (modelled abstractly).
    OpenTicket {
        /// Free-form ticket subject.
        subject: String,
    },
    /// A no-op placeholder (useful for testing / explicit "observe only").
    NoOp,
}

impl RemediationKind {
    /// Stable lower-snake label.
    pub fn label(&self) -> &'static str {
        match self {
            RemediationKind::TightenPolicyThreshold { .. } => "tighten_policy_threshold",
            RemediationKind::IncreaseSamplingRate { .. } => "increase_sampling_rate",
            RemediationKind::RaiseAlert { .. } => "raise_alert",
            RemediationKind::Quarantine { .. } => "quarantine",
            RemediationKind::TriggerIntegrityCheck => "trigger_integrity_check",
            RemediationKind::OpenTicket { .. } => "open_ticket",
            RemediationKind::NoOp => "no_op",
        }
    }
}

/// The condition under which a [`RemediationRule`] fires for a finding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleTrigger {
    /// Match only findings for this invariant id (or `None` to match any).
    pub invariant_id: Option<String>,
    /// Match only findings on this metric (or `None` to match any).
    pub metric: Option<MonitoredMetric>,
    /// Match only findings of at least this severity.
    pub min_severity: MonitorSeverity,
}

impl RuleTrigger {
    /// Matches any finding at or above `min_severity`.
    pub fn any(min_severity: MonitorSeverity) -> Self {
        Self {
            invariant_id: None,
            metric: None,
            min_severity,
        }
    }

    /// Matches findings for a specific invariant id.
    pub fn for_invariant(id: impl Into<String>, min_severity: MonitorSeverity) -> Self {
        Self {
            invariant_id: Some(id.into()),
            metric: None,
            min_severity,
        }
    }

    /// Matches findings for a specific metric.
    pub fn for_metric(metric: MonitoredMetric, min_severity: MonitorSeverity) -> Self {
        Self {
            invariant_id: None,
            metric: Some(metric),
            min_severity,
        }
    }

    /// Returns `true` when `finding` satisfies this trigger.
    pub fn matches(&self, finding: &MonitorFinding) -> bool {
        if finding.severity < self.min_severity {
            return false;
        }
        let invariant_ok = self
            .invariant_id
            .as_ref()
            .is_none_or(|id| &finding.invariant_id == id);
        let metric_ok = self.metric.is_none_or(|m| finding.metric == m);
        invariant_ok && metric_ok
    }
}

/// A declarative remediation rule: when [`RuleTrigger`] matches, emit a
/// [`RemediationKind`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemediationRule {
    /// Stable identifier.
    pub id: String,
    /// Human-readable name.
    pub name: String,
    /// The condition that fires this rule.
    pub trigger: RuleTrigger,
    /// The remediation to perform.
    pub action: RemediationKind,
    /// Whether the rule is currently enabled.
    pub enabled: bool,
}

impl RemediationRule {
    /// Builds an enabled rule.
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        trigger: RuleTrigger,
        action: RemediationKind,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            trigger,
            action,
            enabled: true,
        }
    }

    /// Disables the rule (builder style).
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }
}

/// The lifecycle status of a planned/executed action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionStatus {
    /// Planned in dry-run mode; not executed.
    Planned,
    /// Successfully executed.
    Applied,
    /// Execution attempted but failed.
    Failed,
    /// Deliberately skipped (e.g. de-duplicated or executor declined).
    Skipped,
}

/// A single concrete remediation derived from a finding via a rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemediationAction {
    /// Stable identifier.
    pub id: Uuid,
    /// The rule that produced this action.
    pub rule_id: String,
    /// The finding that triggered it.
    pub finding_id: Uuid,
    /// The id of the invariant the finding violated.
    pub invariant_id: String,
    /// The remediation effect.
    pub kind: RemediationKind,
    /// Lifecycle status.
    pub status: ActionStatus,
    /// Outcome / error detail.
    pub detail: String,
    /// When the action was planned.
    pub planned_at: DateTime<Utc>,
    /// When the action was executed (if it was).
    pub executed_at: Option<DateTime<Utc>>,
}

/// Pluggable executor that actually carries out [`RemediationAction`]s in
/// [`ExecutionMode::Apply`].
///
/// Implementations decide how an abstract [`RemediationKind`] maps onto real
/// side effects (mutating policies, calling APIs, ...). The default
/// [`RecordingExecutor`] simply records that it would have run, which keeps the
/// engine usable and testable with no external dependencies.
pub trait RemediationExecutor {
    /// Executes `action`, returning `Ok(detail)` on success or `Err(detail)` on
    /// failure. Implementations must not panic.
    fn execute(&mut self, action: &RemediationAction) -> Result<String, String>;
}

/// A default executor that records (rather than performs) each action and
/// always succeeds. Useful as a baseline and in tests.
#[derive(Debug, Default)]
pub struct RecordingExecutor {
    /// The kinds it was asked to execute, in order.
    pub executed: Vec<RemediationKind>,
}

impl RemediationExecutor for RecordingExecutor {
    fn execute(&mut self, action: &RemediationAction) -> Result<String, String> {
        self.executed.push(action.kind.clone());
        Ok(format!("recorded {}", action.kind.label()))
    }
}

/// An immutable, hash-chained record of one remediation pass.
///
/// The `record_hash` binds the mode, all actions, and the previous record's
/// hash, so a sequence of [`RemediationRecord`]s forms a tamper-evident log of
/// autonomous interventions — mirroring the crate's primary audit chain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemediationRecord {
    /// Stable identifier.
    pub id: Uuid,
    /// Execution mode used for this pass.
    pub mode: ExecutionMode,
    /// All actions planned/executed in this pass.
    pub actions: Vec<RemediationAction>,
    /// Number of findings considered.
    pub findings_considered: usize,
    /// When the pass ran.
    pub generated_at: DateTime<Utc>,
    /// Hash of the previous remediation record (chain linkage).
    pub previous_hash: Option<String>,
    /// SHA-256 fingerprint of this record.
    pub record_hash: String,
}

impl RemediationRecord {
    /// Computes the SHA-256 fingerprint binding this record's content and its
    /// predecessor.
    fn compute_hash(&self) -> String {
        let mut buf = Vec::new();
        buf.extend_from_slice(self.id.as_bytes());
        buf.extend_from_slice(&self.generated_at.timestamp().to_le_bytes());
        buf.push(match self.mode {
            ExecutionMode::DryRun => 0,
            ExecutionMode::Apply => 1,
        });
        buf.extend_from_slice(&(self.findings_considered as u64).to_le_bytes());
        if let Some(prev) = &self.previous_hash {
            buf.extend_from_slice(prev.as_bytes());
        }
        for a in &self.actions {
            buf.extend_from_slice(a.id.as_bytes());
            buf.extend_from_slice(a.rule_id.as_bytes());
            buf.extend_from_slice(a.kind.label().as_bytes());
            buf.push(match a.status {
                ActionStatus::Planned => 0,
                ActionStatus::Applied => 1,
                ActionStatus::Failed => 2,
                ActionStatus::Skipped => 3,
            });
        }
        to_hex(&sha256(&buf))
    }

    /// Verifies this record's own fingerprint.
    pub fn verify(&self) -> bool {
        self.compute_hash() == self.record_hash
    }

    /// Count of actions in a given status.
    pub fn count_status(&self, status: ActionStatus) -> usize {
        self.actions.iter().filter(|a| a.status == status).count()
    }
}

/// Verifies a chain of remediation records: each fingerprint must be valid and
/// each `previous_hash` must equal the prior record's `record_hash`.
pub fn verify_remediation_chain(records: &[RemediationRecord]) -> bool {
    let mut expected_prev: Option<String> = None;
    for r in records {
        if !r.verify() {
            return false;
        }
        if r.previous_hash != expected_prev {
            return false;
        }
        expected_prev = Some(r.record_hash.clone());
    }
    true
}

/// Drives rule-based remediation over monitor findings.
pub struct RemediationEngine {
    rules: Vec<RemediationRule>,
    mode: ExecutionMode,
    last_hash: Option<String>,
    /// De-duplication: actions already applied (rule_id + invariant_id) this
    /// session are skipped to avoid repeatedly applying the same fix.
    applied_keys: std::collections::HashSet<(String, String)>,
}

impl RemediationEngine {
    /// Creates an engine in the safe [`ExecutionMode::DryRun`] mode.
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            mode: ExecutionMode::DryRun,
            last_hash: None,
            applied_keys: std::collections::HashSet::new(),
        }
    }

    /// Creates an engine seeded with default rules for the default monitor
    /// invariant set.
    pub fn with_defaults() -> Self {
        Self::new()
            .add_rule(RemediationRule::new(
                "rule-override-tighten",
                "Tighten override policy on excessive overrides",
                RuleTrigger::for_invariant("default-override-ceiling", MonitorSeverity::Warning),
                RemediationKind::TightenPolicyThreshold {
                    metric: MonitoredMetric::OverrideRate,
                    delta: 0.05,
                },
            ))
            .add_rule(RemediationRule::new(
                "rule-void-integrity",
                "Re-verify integrity on elevated void rate",
                RuleTrigger::for_invariant("default-void-ceiling", MonitorSeverity::Critical),
                RemediationKind::TriggerIntegrityCheck,
            ))
            .add_rule(RemediationRule::new(
                "rule-chain-alert",
                "Escalate on chain integrity failure",
                RuleTrigger::for_invariant("default-chain-integrity", MonitorSeverity::Critical),
                RemediationKind::RaiseAlert {
                    severity: MonitorSeverity::Critical,
                },
            ))
            .add_rule(RemediationRule::new(
                "rule-broken-hash-ticket",
                "Open ticket on broken record hashes",
                RuleTrigger::for_invariant("default-broken-hashes", MonitorSeverity::Critical),
                RemediationKind::OpenTicket {
                    subject: "Investigate broken audit record hashes".to_string(),
                },
            ))
    }

    /// Sets the execution mode (builder style).
    pub fn with_mode(mut self, mode: ExecutionMode) -> Self {
        self.mode = mode;
        self
    }

    /// Sets the execution mode in place.
    pub fn set_mode(&mut self, mode: ExecutionMode) {
        self.mode = mode;
    }

    /// Returns the current execution mode.
    pub fn mode(&self) -> ExecutionMode {
        self.mode
    }

    /// Adds a rule (builder style).
    pub fn add_rule(mut self, rule: RemediationRule) -> Self {
        self.rules.push(rule);
        self
    }

    /// Pushes a rule in place.
    pub fn push_rule(&mut self, rule: RemediationRule) {
        self.rules.push(rule);
    }

    /// Returns the configured rules.
    pub fn rules(&self) -> &[RemediationRule] {
        &self.rules
    }

    /// The fingerprint of the last produced remediation record, if any.
    pub fn last_hash(&self) -> Option<&str> {
        self.last_hash.as_deref()
    }

    /// Plans (and, in [`ExecutionMode::Apply`], executes) remediations for the
    /// given findings, producing a hash-chained [`RemediationRecord`].
    ///
    /// The supplied `executor` is only consulted in `Apply` mode.
    pub fn remediate(
        &mut self,
        findings: &[MonitorFinding],
        executor: &mut dyn RemediationExecutor,
    ) -> RemediationRecord {
        let now = Utc::now();
        let mut actions: Vec<RemediationAction> = Vec::new();

        for finding in findings {
            for rule in self.rules.iter().filter(|r| r.enabled) {
                if !rule.trigger.matches(finding) {
                    continue;
                }
                let mut action = RemediationAction {
                    id: Uuid::new_v4(),
                    rule_id: rule.id.clone(),
                    finding_id: finding.id,
                    invariant_id: finding.invariant_id.clone(),
                    kind: rule.action.clone(),
                    status: ActionStatus::Planned,
                    detail: "planned (dry-run)".to_string(),
                    planned_at: now,
                    executed_at: None,
                };

                match self.mode {
                    ExecutionMode::DryRun => {
                        action.detail = format!(
                            "DRY-RUN: would {} for invariant '{}'",
                            rule.action.label(),
                            finding.invariant_id
                        );
                    }
                    ExecutionMode::Apply => {
                        let key = (rule.id.clone(), finding.invariant_id.clone());
                        if self.applied_keys.contains(&key) {
                            action.status = ActionStatus::Skipped;
                            action.detail = "skipped: already applied this session".to_string();
                        } else {
                            match executor.execute(&action) {
                                Ok(detail) => {
                                    action.status = ActionStatus::Applied;
                                    action.detail = detail;
                                    action.executed_at = Some(Utc::now());
                                    self.applied_keys.insert(key);
                                }
                                Err(err) => {
                                    action.status = ActionStatus::Failed;
                                    action.detail = format!("error: {err}");
                                    action.executed_at = Some(Utc::now());
                                }
                            }
                        }
                    }
                }
                actions.push(action);
            }
        }

        let mut record = RemediationRecord {
            id: Uuid::new_v4(),
            mode: self.mode,
            actions,
            findings_considered: findings.len(),
            generated_at: now,
            previous_hash: self.last_hash.clone(),
            record_hash: String::new(),
        };
        record.record_hash = record.compute_hash();
        self.last_hash = Some(record.record_hash.clone());
        record
    }

    /// Convenience: dry-run remediation against a default recording executor
    /// (which is never consulted in dry-run mode).
    pub fn dry_run(&mut self, findings: &[MonitorFinding]) -> RemediationRecord {
        let saved = self.mode;
        self.mode = ExecutionMode::DryRun;
        let mut exec = RecordingExecutor::default();
        let record = self.remediate(findings, &mut exec);
        self.mode = saved;
        record
    }

    /// Returns a summary of how many actions each rule has produced across the
    /// supplied records (useful for reporting).
    pub fn summarize(records: &[RemediationRecord]) -> HashMap<String, usize> {
        let mut map = HashMap::new();
        for rec in records {
            for action in &rec.actions {
                *map.entry(action.rule_id.clone()).or_insert(0) += 1;
            }
        }
        map
    }
}

impl Default for RemediationEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::autonomous::monitor::Comparator;

    fn finding(
        invariant_id: &str,
        metric: MonitoredMetric,
        sev: MonitorSeverity,
    ) -> MonitorFinding {
        MonitorFinding {
            id: Uuid::new_v4(),
            invariant_id: invariant_id.to_string(),
            invariant_name: invariant_id.to_string(),
            metric,
            comparator: Comparator::GreaterThan,
            observed: 0.5,
            threshold: 0.2,
            severity: sev,
            message: "test".to_string(),
            detected_at: Utc::now(),
        }
    }

    #[test]
    fn test_dry_run_plans_only() {
        let mut engine = RemediationEngine::with_defaults();
        let findings = vec![finding(
            "default-override-ceiling",
            MonitoredMetric::OverrideRate,
            MonitorSeverity::Warning,
        )];
        let mut exec = RecordingExecutor::default();
        let record = engine.remediate(&findings, &mut exec);
        assert_eq!(record.mode, ExecutionMode::DryRun);
        assert_eq!(record.actions.len(), 1);
        assert_eq!(record.actions[0].status, ActionStatus::Planned);
        // Executor must not have been called in dry-run.
        assert!(exec.executed.is_empty());
        assert!(record.verify());
    }

    #[test]
    fn test_apply_executes() {
        let mut engine = RemediationEngine::with_defaults().with_mode(ExecutionMode::Apply);
        let findings = vec![finding(
            "default-override-ceiling",
            MonitoredMetric::OverrideRate,
            MonitorSeverity::Warning,
        )];
        let mut exec = RecordingExecutor::default();
        let record = engine.remediate(&findings, &mut exec);
        assert_eq!(record.actions[0].status, ActionStatus::Applied);
        assert_eq!(exec.executed.len(), 1);
        assert!(record.actions[0].executed_at.is_some());
    }

    #[test]
    fn test_apply_dedup_skips_repeat() {
        let mut engine = RemediationEngine::with_defaults().with_mode(ExecutionMode::Apply);
        let f = finding(
            "default-override-ceiling",
            MonitoredMetric::OverrideRate,
            MonitorSeverity::Warning,
        );
        let mut exec = RecordingExecutor::default();
        let first = engine.remediate(std::slice::from_ref(&f), &mut exec);
        assert_eq!(first.actions[0].status, ActionStatus::Applied);
        // Second pass with the same invariant => skipped.
        let second = engine.remediate(std::slice::from_ref(&f), &mut exec);
        assert_eq!(second.actions[0].status, ActionStatus::Skipped);
        // Executor only called once.
        assert_eq!(exec.executed.len(), 1);
    }

    #[test]
    fn test_failing_executor_marks_failed() {
        struct FailingExec;
        impl RemediationExecutor for FailingExec {
            fn execute(&mut self, _action: &RemediationAction) -> Result<String, String> {
                Err("boom".to_string())
            }
        }
        let mut engine = RemediationEngine::with_defaults().with_mode(ExecutionMode::Apply);
        let findings = vec![finding(
            "default-chain-integrity",
            MonitoredMetric::ChainIntegrity,
            MonitorSeverity::Critical,
        )];
        let mut exec = FailingExec;
        let record = engine.remediate(&findings, &mut exec);
        assert_eq!(record.actions[0].status, ActionStatus::Failed);
        assert!(record.actions[0].detail.contains("boom"));
    }

    #[test]
    fn test_trigger_matching() {
        let f = finding("x", MonitoredMetric::VoidRate, MonitorSeverity::Warning);
        assert!(RuleTrigger::any(MonitorSeverity::Info).matches(&f));
        assert!(!RuleTrigger::any(MonitorSeverity::Critical).matches(&f));
        assert!(
            RuleTrigger::for_metric(MonitoredMetric::VoidRate, MonitorSeverity::Info).matches(&f)
        );
        assert!(
            !RuleTrigger::for_metric(MonitoredMetric::OverrideRate, MonitorSeverity::Info)
                .matches(&f)
        );
        assert!(RuleTrigger::for_invariant("x", MonitorSeverity::Info).matches(&f));
        assert!(!RuleTrigger::for_invariant("y", MonitorSeverity::Info).matches(&f));
    }

    #[test]
    fn test_remediation_chain_links_and_verifies() {
        let mut engine = RemediationEngine::with_defaults();
        let f1 = finding(
            "default-override-ceiling",
            MonitoredMetric::OverrideRate,
            MonitorSeverity::Warning,
        );
        let f2 = finding(
            "default-void-ceiling",
            MonitoredMetric::VoidRate,
            MonitorSeverity::Critical,
        );
        let r1 = engine.dry_run(std::slice::from_ref(&f1));
        let r2 = engine.dry_run(std::slice::from_ref(&f2));
        assert_eq!(r2.previous_hash.as_deref(), Some(r1.record_hash.as_str()));
        assert!(verify_remediation_chain(&[r1, r2]));
    }

    #[test]
    fn test_tampered_chain_fails_verification() {
        let mut engine = RemediationEngine::with_defaults();
        let f = finding(
            "default-override-ceiling",
            MonitoredMetric::OverrideRate,
            MonitorSeverity::Warning,
        );
        let mut r1 = engine.dry_run(std::slice::from_ref(&f));
        let r2 = engine.dry_run(std::slice::from_ref(&f));
        // Tamper with r1's findings count after the fact.
        r1.findings_considered = 999;
        assert!(!r1.verify());
        assert!(!verify_remediation_chain(&[r1, r2]));
    }

    #[test]
    fn test_disabled_rule_does_not_fire() {
        let mut engine = RemediationEngine::new().add_rule(
            RemediationRule::new(
                "r",
                "disabled",
                RuleTrigger::any(MonitorSeverity::Info),
                RemediationKind::NoOp,
            )
            .disabled(),
        );
        let f = finding("x", MonitoredMetric::VoidRate, MonitorSeverity::Critical);
        let record = engine.dry_run(&[f]);
        assert!(record.actions.is_empty());
    }

    #[test]
    fn test_summarize() {
        let mut engine = RemediationEngine::with_defaults();
        let f = finding(
            "default-override-ceiling",
            MonitoredMetric::OverrideRate,
            MonitorSeverity::Warning,
        );
        let r = engine.dry_run(&[f]);
        let summary = RemediationEngine::summarize(&[r]);
        assert_eq!(summary.get("rule-override-tighten"), Some(&1));
    }
}
