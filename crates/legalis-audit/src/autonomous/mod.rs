//! Autonomous compliance (v0.3.3).
//!
//! This module turns the audit trail from something that is *inspected* into
//! something that *governs itself*. It implements a closed-loop compliance
//! controller built entirely from pure-Rust, statistically-grounded components
//! that reuse the crate's existing record, integrity, and quantum primitives —
//! no external service is involved.
//!
//! The loop has five cooperating parts, each usable independently:
//!
//! 1. [`monitor`] — **self-monitoring**: a [`ComplianceMonitor`] evaluates
//!    declarative [`Invariant`]s over derived [`MonitorMetrics`] and raises
//!    [`MonitorFinding`]s.
//! 2. [`predictive`] — **predictive compliance**: a [`ComplianceForecaster`]
//!    fits trend lines to historical compliance rates and forecasts when a rate
//!    will breach its threshold ([`DriftForecast`]).
//! 3. [`policy`] — **adaptive audit policies**: an [`AdaptiveAuditPolicy`]
//!    raises sampling and tightens thresholds under elevated [`RiskLevel`], and
//!    relaxes them as risk subsides — each move recorded as a
//!    [`PolicyAdjustment`].
//! 4. [`remediation`] — **auto-remediation**: a [`RemediationEngine`] maps
//!    findings to [`RemediationAction`]s via [`RemediationRule`]s under a strict
//!    dry-run/apply model, emitting a hash-chained [`RemediationRecord`].
//! 5. [`attestation`] — **continuous compliance attestation**: an
//!    [`AttestationEngine`] emits signed, fingerprinted, hash-chained
//!    [`ComplianceAttestation`]s pinning the exact records covered by each
//!    window.
//!
//! [`AutonomousComplianceEngine`] wires all five together into a single
//! [`run`](AutonomousComplianceEngine::run) call that observes a record set,
//! assesses risk, adapts policy, plans/applies remediation, and attests — and
//! returns one [`AutonomousCycleReport`] documenting everything it did.

pub mod attestation;
pub mod monitor;
pub mod policy;
pub mod predictive;
pub mod remediation;

pub use attestation::{
    AttestationCheck, AttestationEngine, AttestationSignature, AttestationVerdict, CheckOutcome,
    ComplianceAttestation, coverage_digest, verify_attestation_chain,
};
pub use monitor::{
    Comparator, ComplianceMonitor, Invariant, MonitorFinding, MonitorMetrics, MonitorReport,
    MonitorSeverity, MonitoredMetric,
};
pub use policy::{
    AdaptiveAuditPolicy, AdaptivePolicyConfig, PolicyAdjustment, RiskAssessment, RiskLevel,
    ThresholdKnob,
};
pub use predictive::{
    ComplianceForecaster, DriftConfig, DriftDirection, DriftForecast, DriftReport, TrendFit,
};
pub use remediation::{
    ActionStatus, ExecutionMode, RecordingExecutor, RemediationAction, RemediationEngine,
    RemediationExecutor, RemediationKind, RemediationRecord, RemediationRule, RuleTrigger,
    verify_remediation_chain,
};

use crate::{AuditRecord, AuditResult};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

/// Configuration for the closed-loop [`AutonomousComplianceEngine`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutonomousConfig {
    /// Trailing window the monitor / attestation operate over.
    pub observation_window: Duration,
    /// Whether to actually apply remediations (otherwise dry-run only).
    pub apply_remediation: bool,
    /// Whether to produce an attestation each cycle.
    pub attest: bool,
    /// Whether to run the drift forecaster each cycle.
    pub forecast: bool,
}

impl Default for AutonomousConfig {
    fn default() -> Self {
        Self {
            observation_window: Duration::days(7),
            apply_remediation: false,
            attest: true,
            forecast: true,
        }
    }
}

/// The full record of one autonomous compliance cycle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutonomousCycleReport {
    /// The monitoring pass outcome.
    pub monitor_report: MonitorReport,
    /// The forecast, when enabled.
    pub drift_report: Option<DriftReport>,
    /// The risk assessment that drove policy adaptation.
    pub risk: RiskAssessment,
    /// The policy adjustment applied this cycle.
    pub policy_adjustment: PolicyAdjustment,
    /// The remediation record produced this cycle.
    pub remediation: RemediationRecord,
    /// The attestation produced, when enabled.
    pub attestation: Option<ComplianceAttestation>,
    /// When the cycle ran.
    pub generated_at: DateTime<Utc>,
}

impl AutonomousCycleReport {
    /// `true` when monitoring found no violations this cycle.
    pub fn is_healthy(&self) -> bool {
        self.monitor_report.is_healthy()
    }
}

/// A self-governing compliance controller.
///
/// Owns the monitor, forecaster, adaptive policy, remediation engine, and
/// attestation engine. Each [`run`](Self::run) consumes one attestation leaf
/// when signed attestation is enabled (size the attestation key accordingly).
pub struct AutonomousComplianceEngine {
    config: AutonomousConfig,
    monitor: ComplianceMonitor,
    forecaster: ComplianceForecaster,
    policy: AdaptiveAuditPolicy,
    remediation: RemediationEngine,
    attestation: AttestationEngine,
}

impl AutonomousComplianceEngine {
    /// Builds a controller with the default monitor/forecaster/policy/
    /// remediation/attestation stack and the supplied config.
    pub fn new(config: AutonomousConfig) -> Self {
        let remediation = if config.apply_remediation {
            RemediationEngine::with_defaults().with_mode(ExecutionMode::Apply)
        } else {
            RemediationEngine::with_defaults()
        };
        Self {
            monitor: ComplianceMonitor::with_defaults(),
            forecaster: ComplianceForecaster::new(),
            policy: AdaptiveAuditPolicy::with_defaults(),
            remediation,
            attestation: AttestationEngine::with_defaults(),
            config,
        }
    }

    /// Builds a controller with default configuration.
    pub fn with_defaults() -> Self {
        Self::new(AutonomousConfig::default())
    }

    /// Replaces the monitor (builder style).
    pub fn with_monitor(mut self, monitor: ComplianceMonitor) -> Self {
        self.monitor = monitor;
        self
    }

    /// Replaces the adaptive policy (builder style).
    pub fn with_policy(mut self, policy: AdaptiveAuditPolicy) -> Self {
        self.policy = policy;
        self
    }

    /// Replaces the remediation engine (builder style).
    pub fn with_remediation(mut self, remediation: RemediationEngine) -> Self {
        self.remediation = remediation;
        self
    }

    /// Replaces the attestation engine (builder style); use this to enable
    /// signing via [`AttestationEngine::with_signing`].
    pub fn with_attestation(mut self, attestation: AttestationEngine) -> Self {
        self.attestation = attestation;
        self
    }

    /// Read-only access to the current adaptive policy (e.g. to observe the
    /// evolving sampling rate / thresholds).
    pub fn policy(&self) -> &AdaptiveAuditPolicy {
        &self.policy
    }

    /// Runs one full autonomous cycle over `records`.
    ///
    /// Pipeline: monitor → (forecast) → assess risk → adapt policy →
    /// remediate → (attest). A [`RecordingExecutor`] is used for apply-mode
    /// remediation; supply a custom executor via [`Self::run_with_executor`].
    pub fn run(&mut self, records: &[AuditRecord]) -> AuditResult<AutonomousCycleReport> {
        let mut exec = RecordingExecutor::default();
        self.run_with_executor(records, &mut exec)
    }

    /// Like [`Self::run`] but with a caller-supplied remediation executor
    /// (consulted only when remediation is in apply mode).
    pub fn run_with_executor(
        &mut self,
        records: &[AuditRecord],
        executor: &mut dyn RemediationExecutor,
    ) -> AuditResult<AutonomousCycleReport> {
        let now = Utc::now();

        // Scope to the observation window relative to the latest record.
        let scoped = scope_window(records, self.config.observation_window);

        // 1. Monitor.
        let monitor_report = self.monitor.evaluate(&scoped);

        // 2. Forecast (optional).
        let drift_report = if self.config.forecast {
            Some(self.forecaster.forecast(&scoped))
        } else {
            None
        };

        // 3. Assess risk from findings + forecasts.
        let risk = assess_risk(&monitor_report, drift_report.as_ref());

        // 4. Adapt policy.
        let policy_adjustment = self.policy.adapt(&risk);

        // 5. Remediate (dry-run or apply per config).
        let remediation = self
            .remediation
            .remediate(&monitor_report.findings, executor);

        // 6. Attest (optional).
        let attestation = if self.config.attest {
            let start = scoped
                .iter()
                .map(|r| r.timestamp)
                .min()
                .unwrap_or(now - self.config.observation_window);
            let end = scoped.iter().map(|r| r.timestamp).max().unwrap_or(now);
            Some(self.attestation.attest_records(&scoped, start, end)?)
        } else {
            None
        };

        Ok(AutonomousCycleReport {
            monitor_report,
            drift_report,
            risk,
            policy_adjustment,
            remediation,
            attestation,
            generated_at: now,
        })
    }
}

impl Default for AutonomousComplianceEngine {
    fn default() -> Self {
        Self::with_defaults()
    }
}

/// Scopes records to the trailing `window` relative to the latest timestamp.
fn scope_window(records: &[AuditRecord], window: Duration) -> Vec<AuditRecord> {
    if records.is_empty() {
        return Vec::new();
    }
    let latest = records
        .iter()
        .map(|r| r.timestamp)
        .max()
        .unwrap_or_else(Utc::now);
    let cutoff = latest - window;
    records
        .iter()
        .filter(|r| r.timestamp >= cutoff)
        .cloned()
        .collect()
}

/// Derives a [`RiskAssessment`] from the monitor report and (optional) forecast.
///
/// Risk blends three signals: the *density* of findings (per invariant), the
/// peak *severity* observed, and (when forecasting) whether a breach is
/// already-or-imminently predicted.
fn assess_risk(report: &MonitorReport, drift: Option<&DriftReport>) -> RiskAssessment {
    let mut factors = std::collections::HashMap::new();

    // Finding pressure: findings relative to invariants evaluated.
    let pressure = if report.invariants_evaluated == 0 {
        0.0
    } else {
        (report.findings.len() as f64 / report.invariants_evaluated as f64).clamp(0.0, 1.0)
    };
    factors.insert("finding_pressure".to_string(), pressure);

    // Peak severity weight.
    let severity_weight = match report.max_severity() {
        Some(MonitorSeverity::Critical) => 1.0,
        Some(MonitorSeverity::Warning) => 0.5,
        Some(MonitorSeverity::Info) => 0.2,
        None => 0.0,
    };
    factors.insert("severity_weight".to_string(), severity_weight);

    // Forecast pressure: any already-or-imminent breach raises risk.
    let forecast_weight = drift
        .map(|d| {
            let impending = d.impending_breaches();
            if impending.is_empty() {
                0.0
            } else if impending.iter().any(|f| f.already_breached) {
                1.0
            } else {
                0.6
            }
        })
        .unwrap_or(0.0);
    factors.insert("forecast_weight".to_string(), forecast_weight);

    // Composite: weighted toward severity and forecast.
    let mut score =
        (pressure * 0.3 + severity_weight * 0.4 + forecast_weight * 0.3).clamp(0.0, 1.0);

    // Severity floors: a single critical violation is intrinsically high risk
    // regardless of how few invariants fired, and an already-breached forecast
    // is likewise high. This prevents a lone-but-severe finding from being
    // diluted by a large invariant universe.
    match report.max_severity() {
        Some(MonitorSeverity::Critical) => score = score.max(0.6),
        Some(MonitorSeverity::Warning) => score = score.max(0.3),
        _ => {}
    }
    if forecast_weight >= 1.0 {
        score = score.max(0.6);
    }

    RiskAssessment::new(score.clamp(0.0, 1.0), factors)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use std::collections::HashMap as StdHashMap;
    use uuid::Uuid;

    fn det(ts: DateTime<Utc>) -> AuditRecord {
        let mut r = AuditRecord::new(
            EventType::AutomaticDecision,
            Actor::System {
                component: "engine".to_string(),
            },
            "s".to_string(),
            Uuid::new_v4(),
            DecisionContext::default(),
            DecisionResult::Deterministic {
                effect_applied: "approved".to_string(),
                parameters: StdHashMap::new(),
            },
            None,
        );
        r.timestamp = ts;
        r.relink(None);
        r
    }

    fn voided(ts: DateTime<Utc>) -> AuditRecord {
        let mut r = det(ts);
        r.result = DecisionResult::Void {
            reason: "logic error".to_string(),
        };
        r.relink(None);
        r
    }

    fn chain(records: &mut [AuditRecord]) {
        let mut prev: Option<String> = None;
        for r in records.iter_mut() {
            r.relink(prev.clone());
            prev = Some(r.record_hash.clone());
        }
    }

    #[test]
    fn test_cycle_clean_is_healthy_low_risk() {
        let now = Utc::now();
        let mut records: Vec<AuditRecord> =
            (0..10).map(|i| det(now - Duration::hours(i))).collect();
        chain(&mut records);
        let mut engine = AutonomousComplianceEngine::with_defaults();
        let report = engine.run(&records).expect("run");
        assert!(report.is_healthy());
        assert_eq!(report.risk.level, RiskLevel::Low);
        assert!(report.attestation.is_some());
        assert_eq!(
            report.attestation.as_ref().unwrap().verdict,
            AttestationVerdict::Compliant
        );
        // Dry-run by default: remediation actions, if any, are planned not applied.
        assert_eq!(report.remediation.mode, ExecutionMode::DryRun);
    }

    #[test]
    fn test_cycle_voids_raise_risk_and_tighten_policy() {
        let now = Utc::now();
        let mut records = Vec::new();
        for i in 0..10 {
            records.push(voided(now - Duration::hours(i)));
        }
        for i in 10..14 {
            records.push(det(now - Duration::hours(i)));
        }
        chain(&mut records);

        let mut engine = AutonomousComplianceEngine::with_defaults();
        let base_rate = engine.policy().sampling_rate();
        let report = engine.run(&records).expect("run");

        assert!(!report.is_healthy());
        assert!(report.risk.level >= RiskLevel::High);
        // Policy should have tightened: sampling raised.
        assert!(engine.policy().sampling_rate() > base_rate);
        // Attestation should be non-compliant.
        assert_eq!(
            report.attestation.as_ref().unwrap().verdict,
            AttestationVerdict::NonCompliant
        );
    }

    #[test]
    fn test_apply_mode_executes_remediation() {
        let now = Utc::now();
        let mut records: Vec<AuditRecord> =
            (0..10).map(|i| voided(now - Duration::hours(i))).collect();
        chain(&mut records);
        let config = AutonomousConfig {
            apply_remediation: true,
            ..Default::default()
        };
        let mut engine = AutonomousComplianceEngine::new(config);
        let mut exec = RecordingExecutor::default();
        let report = engine.run_with_executor(&records, &mut exec).expect("run");
        assert_eq!(report.remediation.mode, ExecutionMode::Apply);
        // Void ceiling is critical -> integrity-check remediation should apply.
        assert!(
            report
                .remediation
                .actions
                .iter()
                .any(|a| a.status == ActionStatus::Applied)
        );
        assert!(!exec.executed.is_empty());
    }

    #[test]
    fn test_signed_attestation_cycle() {
        let now = Utc::now();
        let mut records: Vec<AuditRecord> = (0..5).map(|i| det(now - Duration::hours(i))).collect();
        chain(&mut records);
        let att_engine = AttestationEngine::with_defaults()
            .with_signing(4)
            .expect("sign");
        let mut engine = AutonomousComplianceEngine::with_defaults().with_attestation(att_engine);
        let report = engine.run(&records).expect("run");
        let att = report.attestation.expect("attestation");
        assert!(att.signature.is_some());
        assert!(att.verify_signature().expect("verify"));
    }

    #[test]
    fn test_multiple_cycles_chain_attestations() {
        let now = Utc::now();
        let mut records: Vec<AuditRecord> = (0..5).map(|i| det(now - Duration::hours(i))).collect();
        chain(&mut records);
        let att_engine = AttestationEngine::with_defaults()
            .with_signing(4)
            .expect("sign");
        let mut engine = AutonomousComplianceEngine::with_defaults().with_attestation(att_engine);

        let r1 = engine.run(&records).expect("run1");
        let r2 = engine.run(&records).expect("run2");
        let a1 = r1.attestation.expect("a1");
        let a2 = r2.attestation.expect("a2");
        assert_eq!(
            a2.previous_hash.as_deref(),
            Some(a1.attestation_hash.as_str())
        );
        assert!(verify_attestation_chain(&[a1, a2]).expect("chain"));
    }

    #[test]
    fn test_empty_records_cycle() {
        let mut engine = AutonomousComplianceEngine::with_defaults();
        let report = engine.run(&[]).expect("run");
        assert!(report.is_healthy());
        assert_eq!(report.risk.level, RiskLevel::Low);
    }

    #[test]
    fn test_assess_risk_blends_signals() {
        let now = Utc::now();
        let mut records: Vec<AuditRecord> =
            (0..6).map(|i| voided(now - Duration::hours(i))).collect();
        chain(&mut records);
        let monitor = ComplianceMonitor::with_defaults();
        let report = monitor.evaluate(&records);
        let risk = assess_risk(&report, None);
        assert!(risk.score > 0.0);
        assert!(risk.factors.contains_key("severity_weight"));
    }

    #[test]
    fn test_cycle_report_serializes() {
        let now = Utc::now();
        let mut records: Vec<AuditRecord> = (0..8).map(|i| det(now - Duration::hours(i))).collect();
        chain(&mut records);
        let mut engine = AutonomousComplianceEngine::with_defaults();
        let report = engine.run(&records).expect("run");
        let json = serde_json::to_string(&report).expect("serialize");
        assert!(json.contains("monitor_report"));
    }
}
