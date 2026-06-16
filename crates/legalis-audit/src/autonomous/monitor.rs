//! Self-monitoring systems for the audit trail.
//!
//! A [`ComplianceMonitor`] continuously watches an audit trail against a set of
//! configured [`Invariant`]s — declarative compliance constraints over derived
//! [`MonitorMetrics`] — and raises a [`MonitorFinding`] for every violation. It
//! is the autonomous-compliance analogue of a human compliance officer's
//! periodic checklist: instead of being run by hand it is designed to be invoked
//! on every batch of new records (or on a schedule) so drift is caught the
//! moment it crosses a threshold.
//!
//! The monitor is deliberately *self-contained and explainable*: every finding
//! carries the invariant it violated, the observed value, the threshold, and a
//! human-readable message, so it composes with the crate's existing
//! [`crate::insights`] and [`crate::compliance`] machinery without depending on
//! any of it.

use crate::{Actor, AuditRecord, DecisionResult, EventType};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// The metric an [`Invariant`] constrains, computed over the observation window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MonitoredMetric {
    /// Fraction in `[0, 1]` of decisions that were human overrides.
    OverrideRate,
    /// Fraction in `[0, 1]` of decisions that were voided.
    VoidRate,
    /// Fraction in `[0, 1]` of decisions requiring human discretion.
    DiscretionRate,
    /// Number of decisions in the most recent bucket (volume).
    RecentVolume,
    /// Mean number of decisions per active hour over the window.
    MeanHourlyVolume,
    /// Fraction in `[0, 1]` of decisions taken by external actors.
    ExternalActorRate,
    /// Number of distinct subjects touched in the window.
    DistinctSubjects,
    /// Integer flag (`0`/`1`): whether the hash chain verified end-to-end.
    ChainIntegrity,
    /// Number of records whose individual hash failed verification.
    BrokenRecordHashes,
}

impl MonitoredMetric {
    /// Stable lower-snake label.
    pub fn label(self) -> &'static str {
        match self {
            MonitoredMetric::OverrideRate => "override_rate",
            MonitoredMetric::VoidRate => "void_rate",
            MonitoredMetric::DiscretionRate => "discretion_rate",
            MonitoredMetric::RecentVolume => "recent_volume",
            MonitoredMetric::MeanHourlyVolume => "mean_hourly_volume",
            MonitoredMetric::ExternalActorRate => "external_actor_rate",
            MonitoredMetric::DistinctSubjects => "distinct_subjects",
            MonitoredMetric::ChainIntegrity => "chain_integrity",
            MonitoredMetric::BrokenRecordHashes => "broken_record_hashes",
        }
    }
}

/// The comparison an [`Invariant`] applies between the observed metric and its
/// threshold.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Comparator {
    /// Violated when `observed > threshold`.
    GreaterThan,
    /// Violated when `observed >= threshold`.
    GreaterOrEqual,
    /// Violated when `observed < threshold`.
    LessThan,
    /// Violated when `observed <= threshold`.
    LessOrEqual,
    /// Violated when `observed == threshold` (within `EPSILON`).
    Equal,
    /// Violated when `observed != threshold` (within `EPSILON`).
    NotEqual,
}

/// Floating-point tolerance for [`Comparator::Equal`] / [`Comparator::NotEqual`].
const EPSILON: f64 = 1e-9;

impl Comparator {
    /// Returns `true` when `observed` violates the constraint relative to
    /// `threshold`.
    fn is_violation(self, observed: f64, threshold: f64) -> bool {
        match self {
            Comparator::GreaterThan => observed > threshold,
            Comparator::GreaterOrEqual => observed >= threshold,
            Comparator::LessThan => observed < threshold,
            Comparator::LessOrEqual => observed <= threshold,
            Comparator::Equal => (observed - threshold).abs() <= EPSILON,
            Comparator::NotEqual => (observed - threshold).abs() > EPSILON,
        }
    }

    /// Human-readable infix symbol.
    fn symbol(self) -> &'static str {
        match self {
            Comparator::GreaterThan => ">",
            Comparator::GreaterOrEqual => ">=",
            Comparator::LessThan => "<",
            Comparator::LessOrEqual => "<=",
            Comparator::Equal => "==",
            Comparator::NotEqual => "!=",
        }
    }
}

/// Severity assigned to a [`MonitorFinding`] when its [`Invariant`] is violated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MonitorSeverity {
    /// Informational; no immediate action required.
    Info,
    /// Worth attention but not urgent.
    Warning,
    /// A material compliance concern.
    Critical,
}

impl MonitorSeverity {
    /// Stable lower-snake label.
    pub fn label(self) -> &'static str {
        match self {
            MonitorSeverity::Info => "info",
            MonitorSeverity::Warning => "warning",
            MonitorSeverity::Critical => "critical",
        }
    }
}

/// A declarative compliance constraint over a single [`MonitoredMetric`].
///
/// An invariant reads as "`metric` `comparator` `threshold`"; when that holds it
/// is a *violation* and the monitor raises a [`MonitorFinding`] of the
/// configured [`MonitorSeverity`]. For example, an override-rate ceiling is
/// `Invariant::new(OverrideRate, GreaterThan, 0.2)`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invariant {
    /// Stable identifier (used to correlate findings to remediation rules).
    pub id: String,
    /// Human-readable name.
    pub name: String,
    /// The metric this invariant constrains.
    pub metric: MonitoredMetric,
    /// How the metric is compared to the threshold.
    pub comparator: Comparator,
    /// The threshold value.
    pub threshold: f64,
    /// Severity of a violation.
    pub severity: MonitorSeverity,
    /// Optional longer description / rationale.
    pub description: Option<String>,
}

impl Invariant {
    /// Builds a new invariant with a derived default name and `Warning`
    /// severity.
    pub fn new(metric: MonitoredMetric, comparator: Comparator, threshold: f64) -> Self {
        Self {
            id: format!("inv-{}-{}", metric.label(), comparator.symbol()),
            name: format!("{} {} {}", metric.label(), comparator.symbol(), threshold),
            metric,
            comparator,
            threshold,
            severity: MonitorSeverity::Warning,
            description: None,
        }
    }

    /// Overrides the identifier (builder style).
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    /// Overrides the human-readable name (builder style).
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Overrides the severity (builder style).
    pub fn with_severity(mut self, severity: MonitorSeverity) -> Self {
        self.severity = severity;
        self
    }

    /// Sets a description (builder style).
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Evaluates this invariant against the supplied metrics, returning a
    /// finding when violated.
    pub fn evaluate(&self, metrics: &MonitorMetrics) -> Option<MonitorFinding> {
        let observed = metrics.value(self.metric);
        if self.comparator.is_violation(observed, self.threshold) {
            Some(MonitorFinding {
                id: Uuid::new_v4(),
                invariant_id: self.id.clone(),
                invariant_name: self.name.clone(),
                metric: self.metric,
                comparator: self.comparator,
                observed,
                threshold: self.threshold,
                severity: self.severity,
                message: format!(
                    "Invariant '{}' violated: observed {} = {:.4} {} threshold {:.4}",
                    self.name,
                    self.metric.label(),
                    observed,
                    self.comparator.symbol(),
                    self.threshold
                ),
                detected_at: Utc::now(),
            })
        } else {
            None
        }
    }
}

/// A violation of an [`Invariant`] raised by the [`ComplianceMonitor`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorFinding {
    /// Stable identifier for this finding.
    pub id: Uuid,
    /// The id of the invariant that was violated.
    pub invariant_id: String,
    /// The name of the invariant that was violated.
    pub invariant_name: String,
    /// The metric involved.
    pub metric: MonitoredMetric,
    /// The comparator used.
    pub comparator: Comparator,
    /// The observed metric value.
    pub observed: f64,
    /// The threshold compared against.
    pub threshold: f64,
    /// Severity of the violation.
    pub severity: MonitorSeverity,
    /// Human-readable explanation.
    pub message: String,
    /// When the violation was detected.
    pub detected_at: DateTime<Utc>,
}

/// Derived metrics computed over an observation window of records.
///
/// Computed once via [`MonitorMetrics::compute`] and reused across all
/// invariants so a single monitor pass is `O(n)` in records plus `O(k)` in
/// invariants.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorMetrics {
    /// Total records in the window.
    pub total: usize,
    /// Override fraction in `[0, 1]`.
    pub override_rate: f64,
    /// Void fraction in `[0, 1]`.
    pub void_rate: f64,
    /// Discretion fraction in `[0, 1]`.
    pub discretion_rate: f64,
    /// External-actor fraction in `[0, 1]`.
    pub external_actor_rate: f64,
    /// Number of decisions in the most recent hour bucket.
    pub recent_volume: usize,
    /// Mean decisions per active hour.
    pub mean_hourly_volume: f64,
    /// Distinct subjects touched.
    pub distinct_subjects: usize,
    /// Whether the hash chain verified end-to-end.
    pub chain_integrity: bool,
    /// Number of records with an individually invalid hash.
    pub broken_record_hashes: usize,
    /// Window start (earliest record), if any.
    pub window_start: Option<DateTime<Utc>>,
    /// Window end (latest record), if any.
    pub window_end: Option<DateTime<Utc>>,
}

impl MonitorMetrics {
    /// Computes metrics over `records`. Records may be unordered; they are
    /// inspected in chain order for integrity but bucketed by timestamp for
    /// volume.
    pub fn compute(records: &[AuditRecord]) -> Self {
        let total = records.len();
        if total == 0 {
            return Self::empty();
        }

        let mut overrides = 0usize;
        let mut voids = 0usize;
        let mut discretion = 0usize;
        let mut external = 0usize;
        let mut subjects: std::collections::HashSet<Uuid> = std::collections::HashSet::new();
        let mut min_ts = records[0].timestamp;
        let mut max_ts = records[0].timestamp;

        for r in records {
            match &r.result {
                DecisionResult::Overridden { .. } => overrides += 1,
                DecisionResult::Void { .. } => voids += 1,
                DecisionResult::RequiresDiscretion { .. } => discretion += 1,
                DecisionResult::Deterministic { .. } => {}
            }
            if matches!(r.actor, Actor::External { .. }) {
                external += 1;
            }
            // An override event also counts toward discretion-style human
            // involvement even when the *result* is deterministic.
            if matches!(r.event_type, EventType::HumanOverride) {
                // Avoid double-counting if the result is already Overridden.
                if !matches!(r.result, DecisionResult::Overridden { .. }) {
                    overrides += 1;
                }
            }
            subjects.insert(r.subject_id);
            if r.timestamp < min_ts {
                min_ts = r.timestamp;
            }
            if r.timestamp > max_ts {
                max_ts = r.timestamp;
            }
        }

        // Bucket by hour for volume metrics.
        let mut buckets: HashMap<i64, usize> = HashMap::new();
        for r in records {
            let hour = r.timestamp.timestamp().div_euclid(3600);
            *buckets.entry(hour).or_insert(0) += 1;
        }
        let active_hours = buckets.len().max(1);
        let mean_hourly_volume = total as f64 / active_hours as f64;
        let latest_hour = max_ts.timestamp().div_euclid(3600);
        let recent_volume = buckets.get(&latest_hour).copied().unwrap_or(0);

        let (chain_integrity, broken_record_hashes) = Self::verify_chain(records);

        let denom = total as f64;
        Self {
            total,
            override_rate: overrides as f64 / denom,
            void_rate: voids as f64 / denom,
            discretion_rate: discretion as f64 / denom,
            external_actor_rate: external as f64 / denom,
            recent_volume,
            mean_hourly_volume,
            distinct_subjects: subjects.len(),
            chain_integrity,
            broken_record_hashes,
            window_start: Some(min_ts),
            window_end: Some(max_ts),
        }
    }

    /// Verifies per-record hashes and the chain linkage in the given order.
    ///
    /// The records may be an arbitrary contiguous *window* of a larger chain, so
    /// the first record's `previous_hash` is accepted as an external anchor
    /// (only its own hash is checked); from the second record onward each
    /// `previous_hash` must equal the prior record's `record_hash`.
    fn verify_chain(records: &[AuditRecord]) -> (bool, usize) {
        let mut broken = 0usize;
        let mut chain_ok = true;
        let mut prev_hash: Option<String> = None;
        for (i, r) in records.iter().enumerate() {
            if !r.verify() {
                broken += 1;
                chain_ok = false;
            }
            // Skip the linkage check on the first record (it may anchor to a
            // record outside this window); enforce it thereafter.
            if i > 0 && r.previous_hash != prev_hash {
                chain_ok = false;
            }
            prev_hash = Some(r.record_hash.clone());
        }
        (chain_ok, broken)
    }

    /// Empty metrics for an empty window.
    fn empty() -> Self {
        Self {
            total: 0,
            override_rate: 0.0,
            void_rate: 0.0,
            discretion_rate: 0.0,
            external_actor_rate: 0.0,
            recent_volume: 0,
            mean_hourly_volume: 0.0,
            distinct_subjects: 0,
            chain_integrity: true,
            broken_record_hashes: 0,
            window_start: None,
            window_end: None,
        }
    }

    /// Returns the numeric value for `metric` (booleans/counts widened to
    /// `f64`).
    pub fn value(&self, metric: MonitoredMetric) -> f64 {
        match metric {
            MonitoredMetric::OverrideRate => self.override_rate,
            MonitoredMetric::VoidRate => self.void_rate,
            MonitoredMetric::DiscretionRate => self.discretion_rate,
            MonitoredMetric::RecentVolume => self.recent_volume as f64,
            MonitoredMetric::MeanHourlyVolume => self.mean_hourly_volume,
            MonitoredMetric::ExternalActorRate => self.external_actor_rate,
            MonitoredMetric::DistinctSubjects => self.distinct_subjects as f64,
            MonitoredMetric::ChainIntegrity => {
                if self.chain_integrity {
                    1.0
                } else {
                    0.0
                }
            }
            MonitoredMetric::BrokenRecordHashes => self.broken_record_hashes as f64,
        }
    }
}

/// The outcome of a single monitoring pass.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorReport {
    /// Metrics computed over the window.
    pub metrics: MonitorMetrics,
    /// All findings, sorted by descending severity.
    pub findings: Vec<MonitorFinding>,
    /// Number of invariants evaluated.
    pub invariants_evaluated: usize,
    /// When the pass completed.
    pub generated_at: DateTime<Utc>,
}

impl MonitorReport {
    /// `true` when no invariant was violated.
    pub fn is_healthy(&self) -> bool {
        self.findings.is_empty()
    }

    /// Count of findings of at least the given severity.
    pub fn count_at_least(&self, severity: MonitorSeverity) -> usize {
        self.findings
            .iter()
            .filter(|f| f.severity >= severity)
            .count()
    }

    /// The single highest severity observed, if any.
    pub fn max_severity(&self) -> Option<MonitorSeverity> {
        self.findings.iter().map(|f| f.severity).max()
    }
}

/// Watches an audit trail against a set of [`Invariant`]s.
#[derive(Debug, Clone, Default)]
pub struct ComplianceMonitor {
    invariants: Vec<Invariant>,
}

impl ComplianceMonitor {
    /// Creates an empty monitor.
    pub fn new() -> Self {
        Self {
            invariants: Vec::new(),
        }
    }

    /// Creates a monitor seeded with a sensible default invariant set:
    /// override-rate and void-rate ceilings, a chain-integrity floor, and a
    /// broken-hash ceiling.
    pub fn with_defaults() -> Self {
        Self::new()
            .add_invariant(
                Invariant::new(MonitoredMetric::OverrideRate, Comparator::GreaterThan, 0.25)
                    .with_id("default-override-ceiling")
                    .with_name("Override rate ceiling")
                    .with_severity(MonitorSeverity::Warning)
                    .with_description(
                        "Sustained override rates above 25% suggest the automated \
                         decision logic is mis-calibrated or being routinely bypassed.",
                    ),
            )
            .add_invariant(
                Invariant::new(MonitoredMetric::VoidRate, Comparator::GreaterThan, 0.1)
                    .with_id("default-void-ceiling")
                    .with_name("Void rate ceiling")
                    .with_severity(MonitorSeverity::Critical)
                    .with_description(
                        "A void rate above 10% indicates frequent logical errors in \
                         statute evaluation.",
                    ),
            )
            .add_invariant(
                Invariant::new(MonitoredMetric::ChainIntegrity, Comparator::LessThan, 1.0)
                    .with_id("default-chain-integrity")
                    .with_name("Hash chain integrity")
                    .with_severity(MonitorSeverity::Critical)
                    .with_description("The audit hash chain failed end-to-end verification."),
            )
            .add_invariant(
                Invariant::new(
                    MonitoredMetric::BrokenRecordHashes,
                    Comparator::GreaterThan,
                    0.0,
                )
                .with_id("default-broken-hashes")
                .with_name("Broken record hashes")
                .with_severity(MonitorSeverity::Critical)
                .with_description("One or more records failed individual hash verification."),
            )
    }

    /// Adds an invariant (builder style).
    pub fn add_invariant(mut self, invariant: Invariant) -> Self {
        self.invariants.push(invariant);
        self
    }

    /// Pushes an invariant in place.
    pub fn push_invariant(&mut self, invariant: Invariant) {
        self.invariants.push(invariant);
    }

    /// Returns the configured invariants.
    pub fn invariants(&self) -> &[Invariant] {
        &self.invariants
    }

    /// Runs all invariants over the whole record set.
    pub fn evaluate(&self, records: &[AuditRecord]) -> MonitorReport {
        let metrics = MonitorMetrics::compute(records);
        self.evaluate_metrics(metrics)
    }

    /// Runs all invariants over only records within the trailing `window`
    /// (relative to the latest record's timestamp).
    pub fn evaluate_window(&self, records: &[AuditRecord], window: Duration) -> MonitorReport {
        if records.is_empty() {
            return self.evaluate_metrics(MonitorMetrics::compute(&[]));
        }
        let latest = records
            .iter()
            .map(|r| r.timestamp)
            .max()
            .unwrap_or_else(Utc::now);
        let cutoff = latest - window;
        let scoped: Vec<AuditRecord> = records
            .iter()
            .filter(|r| r.timestamp >= cutoff)
            .cloned()
            .collect();
        self.evaluate(&scoped)
    }

    /// Runs all invariants over pre-computed metrics.
    pub fn evaluate_metrics(&self, metrics: MonitorMetrics) -> MonitorReport {
        let mut findings: Vec<MonitorFinding> = self
            .invariants
            .iter()
            .filter_map(|inv| inv.evaluate(&metrics))
            .collect();
        findings.sort_by(|a, b| {
            b.severity
                .cmp(&a.severity)
                .then(b.detected_at.cmp(&a.detected_at))
        });
        MonitorReport {
            metrics,
            invariants_evaluated: self.invariants.len(),
            findings,
            generated_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DecisionContext;
    use std::collections::HashMap as StdHashMap;

    fn det(statute: &str, ts: DateTime<Utc>) -> AuditRecord {
        let mut r = AuditRecord::new(
            EventType::AutomaticDecision,
            Actor::System {
                component: "engine".to_string(),
            },
            statute.to_string(),
            Uuid::new_v4(),
            DecisionContext::default(),
            DecisionResult::Deterministic {
                effect_applied: "approved".to_string(),
                parameters: StdHashMap::new(),
            },
            None,
        );
        r.timestamp = ts;
        r.record_hash = String::new();
        r
    }

    fn overridden(ts: DateTime<Utc>) -> AuditRecord {
        let mut r = det("s", ts);
        r.event_type = EventType::HumanOverride;
        r.result = DecisionResult::Overridden {
            original_result: Box::new(DecisionResult::Deterministic {
                effect_applied: "denied".to_string(),
                parameters: StdHashMap::new(),
            }),
            new_result: Box::new(DecisionResult::Deterministic {
                effect_applied: "approved".to_string(),
                parameters: StdHashMap::new(),
            }),
            justification: "manual".to_string(),
        };
        r
    }

    fn voided(ts: DateTime<Utc>) -> AuditRecord {
        let mut r = det("s", ts);
        r.result = DecisionResult::Void {
            reason: "logic error".to_string(),
        };
        r
    }

    /// Re-links a vector into a valid hash chain like `AuditTrail::record` does.
    fn chain(records: &mut [AuditRecord]) {
        let mut prev: Option<String> = None;
        for r in records.iter_mut() {
            r.relink(prev.clone());
            prev = Some(r.record_hash.clone());
        }
    }

    #[test]
    fn test_metrics_basic_rates() {
        let now = Utc::now();
        let mut records = vec![det("s", now), det("s", now), overridden(now), voided(now)];
        chain(&mut records);
        let m = MonitorMetrics::compute(&records);
        assert_eq!(m.total, 4);
        assert!((m.override_rate - 0.25).abs() < 1e-9);
        assert!((m.void_rate - 0.25).abs() < 1e-9);
        assert!(m.chain_integrity);
        assert_eq!(m.broken_record_hashes, 0);
    }

    #[test]
    fn test_invariant_override_ceiling_fires() {
        let now = Utc::now();
        let mut records = vec![overridden(now), overridden(now), det("s", now)];
        chain(&mut records);
        let monitor = ComplianceMonitor::new().add_invariant(
            Invariant::new(MonitoredMetric::OverrideRate, Comparator::GreaterThan, 0.5)
                .with_severity(MonitorSeverity::Critical),
        );
        let report = monitor.evaluate(&records);
        assert!(!report.is_healthy());
        assert_eq!(report.findings.len(), 1);
        assert_eq!(report.findings[0].severity, MonitorSeverity::Critical);
    }

    #[test]
    fn test_invariant_not_violated() {
        let now = Utc::now();
        let mut records = vec![det("s", now), det("s", now)];
        chain(&mut records);
        let monitor = ComplianceMonitor::new().add_invariant(Invariant::new(
            MonitoredMetric::OverrideRate,
            Comparator::GreaterThan,
            0.1,
        ));
        let report = monitor.evaluate(&records);
        assert!(report.is_healthy());
    }

    #[test]
    fn test_chain_integrity_detected() {
        let now = Utc::now();
        let mut records = vec![det("s", now), det("s", now), det("s", now)];
        chain(&mut records);
        // Tamper: break the middle record's hash.
        records[1].record_hash = "deadbeef".to_string();
        let monitor = ComplianceMonitor::with_defaults();
        let report = monitor.evaluate(&records);
        assert!(!report.metrics.chain_integrity);
        assert!(report.metrics.broken_record_hashes >= 1);
        assert!(
            report
                .findings
                .iter()
                .any(|f| f.invariant_id == "default-chain-integrity")
        );
        assert!(
            report
                .findings
                .iter()
                .any(|f| f.invariant_id == "default-broken-hashes")
        );
    }

    #[test]
    fn test_findings_sorted_by_severity() {
        let now = Utc::now();
        let mut records = vec![overridden(now), voided(now), voided(now)];
        chain(&mut records);
        let monitor = ComplianceMonitor::new()
            .add_invariant(
                Invariant::new(MonitoredMetric::OverrideRate, Comparator::GreaterThan, 0.0)
                    .with_severity(MonitorSeverity::Info),
            )
            .add_invariant(
                Invariant::new(MonitoredMetric::VoidRate, Comparator::GreaterThan, 0.0)
                    .with_severity(MonitorSeverity::Critical),
            );
        let report = monitor.evaluate(&records);
        assert_eq!(report.findings.len(), 2);
        assert_eq!(report.findings[0].severity, MonitorSeverity::Critical);
        assert_eq!(report.max_severity(), Some(MonitorSeverity::Critical));
        assert_eq!(report.count_at_least(MonitorSeverity::Warning), 1);
    }

    #[test]
    fn test_evaluate_window_scopes_records() {
        let now = Utc::now();
        let mut records = vec![
            voided(now - Duration::days(10)),
            voided(now - Duration::days(10)),
            det("s", now),
            det("s", now),
        ];
        chain(&mut records);
        let monitor = ComplianceMonitor::new().add_invariant(Invariant::new(
            MonitoredMetric::VoidRate,
            Comparator::GreaterThan,
            0.1,
        ));
        // Whole history: 50% voids -> violation.
        assert!(!monitor.evaluate(&records).is_healthy());
        // Last day only: 0% voids -> healthy.
        let windowed = monitor.evaluate_window(&records, Duration::days(1));
        assert!(windowed.is_healthy());
    }

    #[test]
    fn test_empty_records_healthy() {
        let monitor = ComplianceMonitor::with_defaults();
        let report = monitor.evaluate(&[]);
        assert!(report.is_healthy());
        assert_eq!(report.metrics.total, 0);
    }

    #[test]
    fn test_comparators() {
        assert!(Comparator::GreaterThan.is_violation(0.3, 0.2));
        assert!(!Comparator::GreaterThan.is_violation(0.2, 0.2));
        assert!(Comparator::GreaterOrEqual.is_violation(0.2, 0.2));
        assert!(Comparator::LessThan.is_violation(0.1, 0.2));
        assert!(Comparator::Equal.is_violation(1.0, 1.0));
        assert!(Comparator::NotEqual.is_violation(0.0, 1.0));
    }

    #[test]
    fn test_finding_serializes() {
        let now = Utc::now();
        let mut records = vec![voided(now), voided(now)];
        chain(&mut records);
        let monitor = ComplianceMonitor::with_defaults();
        let report = monitor.evaluate(&records);
        let json = serde_json::to_string(&report).expect("serialize");
        assert!(json.contains("void_rate"));
    }
}
