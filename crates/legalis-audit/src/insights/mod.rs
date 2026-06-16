//! AI-powered audit insights.
//!
//! This module is the orchestration layer over a family of pure-Rust,
//! statistically-grounded analysers. It turns a stream of [`AuditRecord`]s into
//! prioritized, explained, and actionable intelligence:
//!
//! - [`anomaly`] — robust (MAD / IQR) stream anomaly detection, rare-event and
//!   frequency-spike detection, and baseline-drift detection.
//! - [`prediction`] — a first-order Markov model over decision outcomes that
//!   predicts the next outcome, flags improbable transitions, and detects
//!   outcome drift.
//! - [`root_cause`] — hash-chain backtracking and event correlation to explain
//!   *why* a finding occurred.
//! - [`finding`] — the [`AuditFinding`] model and risk-based prioritization
//!   (severity x likelihood x blast radius).
//! - [`remediation`] — a template catalogue that turns findings into concrete
//!   remediation suggestions.
//! - [`improvement`] — period-over-period trend tracking that shows whether the
//!   audit programme is getting better or worse.
//!
//! The [`InsightsEngine`] runs the whole pipeline and additionally synthesises
//! higher-level [`Recommendation`]s — the "AI-powered audit recommendations"
//! that aggregate many findings into a small set of prioritized actions.
//!
//! Everything here is heuristic / statistical and runs entirely in-process; no
//! external ML service is involved. It complements, and does not duplicate, the
//! per-record [`crate::ml_anomaly`] detector and the violation-forecasting
//! [`crate::predictive`] analyser.

pub mod anomaly;
pub mod finding;
pub mod improvement;
pub mod prediction;
pub mod remediation;
pub mod root_cause;

pub use anomaly::{
    BaselineModel, StreamAnomaly, StreamAnomalyConfig, StreamAnomalyDetector, TimeGranularity,
};
pub use finding::{
    AuditFinding, BlastRadius, FindingKind, FindingPrioritizer, ImpactScope, Likelihood,
    PrioritizationConfig, PrioritizedFinding, PriorityScore, PriorityTier, Severity,
};
pub use improvement::{
    AuditPeriod, ImprovementConfig, ImprovementDirection, ImprovementReport, ImprovementTracker,
    PeriodMetrics, TrendMetric,
};
pub use prediction::{
    OutcomeCategory, OutcomePrediction, OutcomePredictionConfig, OutcomePredictor, SequenceKey,
    TransitionModel,
};
pub use remediation::{
    RemediationCatalog, RemediationEffort, RemediationSuggestion, RemediationTemplate,
};
pub use root_cause::{
    CandidateCause, CausalLink, CorrelationKind, RootCauseAnalysis, RootCauseAnalyzer,
    RootCauseConfig,
};

use crate::{AuditRecord, AuditResult, DecisionResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Aggregate configuration for the whole insights pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsightsConfig {
    /// Stream anomaly detection configuration.
    pub anomaly: StreamAnomalyConfig,
    /// Outcome prediction configuration.
    pub prediction: OutcomePredictionConfig,
    /// Finding prioritization configuration.
    pub prioritization: PrioritizationConfig,
    /// Root-cause analysis configuration.
    pub root_cause: RootCauseConfig,
    /// Enable stream anomaly detection.
    pub enable_anomaly: bool,
    /// Enable outcome prediction (drift + improbable transitions + forecasts).
    pub enable_prediction: bool,
    /// Enable behavioural rate-cluster signals (override / void clustering).
    pub enable_behavioral: bool,
    /// Enable root-cause analysis of the top findings.
    pub enable_root_cause: bool,
    /// Number of top findings to run root-cause analysis on.
    pub root_cause_findings: usize,
    /// Evidence targets analysed per finding.
    pub root_cause_targets_per_finding: usize,
    /// Fraction of the most-recent records treated as the "recent" window.
    pub recent_window_fraction: f64,
    /// Maximum number of outcome forecasts retained in the report.
    pub max_forecasts: usize,
}

impl Default for InsightsConfig {
    fn default() -> Self {
        Self {
            anomaly: StreamAnomalyConfig::default(),
            prediction: OutcomePredictionConfig::default(),
            prioritization: PrioritizationConfig::default(),
            root_cause: RootCauseConfig::default(),
            enable_anomaly: true,
            enable_prediction: true,
            enable_behavioral: true,
            enable_root_cause: true,
            root_cause_findings: 5,
            root_cause_targets_per_finding: 1,
            recent_window_fraction: 0.25,
            max_forecasts: 20,
        }
    }
}

/// A synthesised, higher-level recommendation aggregating related findings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    /// Stable identifier.
    pub id: Uuid,
    /// Headline action.
    pub title: String,
    /// Why this is recommended.
    pub rationale: String,
    /// The triage tier of the recommendation.
    pub priority: PriorityTier,
    /// The finding kind addressed, or `None` for a cross-cutting recommendation.
    pub finding_kind: Option<String>,
    /// How many findings support this recommendation.
    pub supporting_findings: usize,
    /// Confidence in `[0, 1]`.
    pub confidence: f64,
    /// Concrete remediation steps.
    pub remediation: Vec<RemediationSuggestion>,
}

/// A compact roll-up of the report's findings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsightsSummary {
    /// Total findings produced.
    pub total_findings: usize,
    /// Count of [`PriorityTier::Urgent`] findings.
    pub urgent: usize,
    /// Count of [`PriorityTier::High`] findings.
    pub high: usize,
    /// Count of [`PriorityTier::Medium`] findings.
    pub medium: usize,
    /// Count of [`PriorityTier::Low`] findings.
    pub low: usize,
    /// Count of [`PriorityTier::Backlog`] findings.
    pub backlog: usize,
    /// Number of stream anomalies detected.
    pub anomaly_count: usize,
    /// The highest priority score observed, if any.
    pub top_priority: Option<f64>,
}

/// The complete output of an insights run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsightsReport {
    /// All findings, prioritized and sorted by descending priority.
    pub findings: Vec<PrioritizedFinding>,
    /// Raw stream anomalies (also surfaced as findings).
    pub anomalies: Vec<StreamAnomaly>,
    /// Outcome forecasts for in-flight sequences, adverse-first.
    pub forecasts: Vec<OutcomePrediction>,
    /// Root-cause analyses for the top findings.
    pub root_causes: Vec<RootCauseAnalysis>,
    /// Synthesised, prioritized recommendations.
    pub recommendations: Vec<Recommendation>,
    /// Roll-up summary.
    pub summary: InsightsSummary,
    /// When the report was generated.
    pub generated_at: DateTime<Utc>,
}

/// Runs the full insights pipeline.
#[derive(Debug, Clone)]
pub struct InsightsEngine {
    config: InsightsConfig,
    catalog: RemediationCatalog,
}

impl InsightsEngine {
    /// Creates an engine with the default configuration and remediation
    /// catalogue.
    pub fn new() -> Self {
        Self::with_config(InsightsConfig::default())
    }

    /// Creates an engine with a custom configuration and the default catalogue.
    pub fn with_config(config: InsightsConfig) -> Self {
        Self {
            config,
            catalog: RemediationCatalog::default(),
        }
    }

    /// Overrides the remediation catalogue (builder style).
    pub fn with_catalog(mut self, catalog: RemediationCatalog) -> Self {
        self.catalog = catalog;
        self
    }

    /// Returns the active configuration.
    pub fn config(&self) -> &InsightsConfig {
        &self.config
    }

    /// Returns the remediation catalogue.
    pub fn catalog(&self) -> &RemediationCatalog {
        &self.catalog
    }

    /// Runs the complete pipeline over `records`.
    pub fn analyze(&self, records: &[AuditRecord]) -> AuditResult<InsightsReport> {
        let generated_at = Utc::now();
        if records.is_empty() {
            return Ok(empty_report(generated_at));
        }

        let mut anomalies = Vec::new();
        let mut findings: Vec<AuditFinding> = Vec::new();
        let mut forecasts = Vec::new();

        // 1. Stream anomaly detection.
        if self.config.enable_anomaly {
            let detector = StreamAnomalyDetector::with_config(self.config.anomaly.clone());
            anomalies = detector.detect(records)?;
            findings.extend(anomalies.iter().map(StreamAnomaly::to_finding));
        }

        // 2. Outcome prediction: drift, improbable transitions, forecasts.
        if self.config.enable_prediction {
            let mut predictor = OutcomePredictor::with_config(self.config.prediction.clone());
            predictor.train(records)?;

            let recent = recent_window(records, self.config.recent_window_fraction);
            if let Some(drift) = predictor.detect_outcome_drift(&recent)? {
                findings.push(drift);
            }
            findings.extend(predictor.detect_improbable_transitions(records)?);

            let mut all = predictor.predict_all(records)?;
            all.truncate(self.config.max_forecasts);
            forecasts = all;
        }

        // 3. Behavioural rate-cluster signals.
        if self.config.enable_behavioral {
            findings.extend(self.detect_rate_clusters(records));
        }

        // 4. Prioritize.
        let prioritizer = FindingPrioritizer::with_config(self.config.prioritization.clone());
        let prioritized = prioritizer.prioritize(findings);

        // 5. Root-cause analysis for the top findings.
        let mut root_causes = Vec::new();
        if self.config.enable_root_cause {
            let analyzer = RootCauseAnalyzer::with_config(self.config.root_cause.clone());
            for pf in prioritized.iter().take(self.config.root_cause_findings) {
                let analyses = analyzer.analyze_finding(
                    records,
                    &pf.finding,
                    self.config.root_cause_targets_per_finding,
                )?;
                root_causes.extend(analyses);
            }
        }

        // 6. Synthesise recommendations and roll-up.
        let recommendations = self.synthesize_recommendations(&prioritized);
        let summary = summarize(&prioritized, anomalies.len());

        Ok(InsightsReport {
            findings: prioritized,
            anomalies,
            forecasts,
            root_causes,
            recommendations,
            summary,
            generated_at,
        })
    }

    /// Computes the [`PeriodMetrics`] for a single period, running the full
    /// finding pipeline against the period's records so the health score
    /// reflects detected issues.
    pub fn period_metrics(
        &self,
        records: &[AuditRecord],
        period: AuditPeriod,
    ) -> AuditResult<PeriodMetrics> {
        let scoped: Vec<AuditRecord> = records
            .iter()
            .filter(|r| period.contains(r.timestamp))
            .cloned()
            .collect();
        let report = self.analyze(&scoped)?;
        Ok(PeriodMetrics::compute(period, &scoped, &report.findings))
    }

    /// Tracks improvement across a set of periods, returning a trend report
    /// (or `None` when fewer than two periods are supplied).
    pub fn track_improvement(
        &self,
        records: &[AuditRecord],
        periods: Vec<AuditPeriod>,
    ) -> AuditResult<Option<ImprovementReport>> {
        let mut tracker = ImprovementTracker::new();
        for period in periods {
            let metrics = self.period_metrics(records, period)?;
            tracker.add_period(metrics);
        }
        Ok(tracker.report())
    }

    /// Detects elevated clustering of overrides / voids in the recent window
    /// relative to the long-run baseline.
    fn detect_rate_clusters(&self, records: &[AuditRecord]) -> Vec<AuditFinding> {
        let mut findings = Vec::new();
        let total = records.len();
        if total < self.config.anomaly.min_buckets {
            return findings;
        }
        let recent = recent_window(records, self.config.recent_window_fraction);
        if recent.len() < 3 {
            return findings;
        }

        if let Some(f) = self.rate_cluster(
            records,
            &recent,
            FindingKind::OverrideCluster,
            |r| matches!(r.result, DecisionResult::Overridden { .. }),
            "override",
        ) {
            findings.push(f);
        }
        if let Some(f) = self.rate_cluster(
            records,
            &recent,
            FindingKind::ElevatedVoidRate,
            |r| matches!(r.result, DecisionResult::Void { .. }),
            "void",
        ) {
            findings.push(f);
        }
        findings
    }

    /// Builds a single rate-cluster finding if the recent rate materially
    /// exceeds the baseline rate.
    fn rate_cluster(
        &self,
        all: &[AuditRecord],
        recent: &[AuditRecord],
        kind: FindingKind,
        predicate: impl Fn(&AuditRecord) -> bool,
        label: &str,
    ) -> Option<AuditFinding> {
        let baseline_hits = all.iter().filter(|r| predicate(r)).count();
        let baseline_rate = baseline_hits as f64 / all.len() as f64;
        let recent_hits: Vec<&AuditRecord> = recent.iter().filter(|r| predicate(r)).collect();
        let recent_rate = recent_hits.len() as f64 / recent.len() as f64;

        // Require a genuine cluster: at least 3 recent hits and a rate at least
        // double the baseline (with an absolute floor to suppress noise).
        let threshold = (baseline_rate * 2.0).max(baseline_rate + 0.1);
        if recent_hits.len() < 3 || recent_rate <= threshold {
            return None;
        }

        let confidence = ((recent_rate - threshold) / (1.0 - threshold).max(1e-6)).clamp(0.0, 1.0);
        let subjects: std::collections::HashSet<Uuid> =
            recent_hits.iter().map(|r| r.subject_id).collect();
        let statutes: std::collections::HashSet<String> =
            recent_hits.iter().map(|r| r.statute_id.clone()).collect();
        let evidence: Vec<Uuid> = recent_hits.iter().take(100).map(|r| r.id).collect();
        let blast = BlastRadius::from_counts(recent_hits.len(), subjects.len(), statutes.len());
        let severity = if kind == FindingKind::ElevatedVoidRate {
            Severity::High
        } else {
            Severity::Medium
        };

        Some(
            AuditFinding::new(
                kind,
                format!("Elevated {label} rate in recent activity"),
                severity,
                Likelihood::from_confidence(confidence),
                blast,
            )
            .with_description(format!(
                "The recent {} rate is {:.1}% versus a baseline of {:.1}%, indicating clustering that warrants review.",
                label,
                recent_rate * 100.0,
                baseline_rate * 100.0
            ))
            .with_evidence(evidence)
            .with_metric("recent_rate", recent_rate)
            .with_metric("baseline_rate", baseline_rate),
        )
    }

    /// Aggregates findings by kind into a small set of prioritized
    /// recommendations, each carrying concrete remediation steps. A
    /// cross-cutting recommendation is prepended when urgent/high findings
    /// accumulate.
    fn synthesize_recommendations(
        &self,
        prioritized: &[PrioritizedFinding],
    ) -> Vec<Recommendation> {
        // Group indices by finding kind, preserving the (already sorted) order.
        let mut groups: HashMap<String, Vec<usize>> = HashMap::new();
        let mut order: Vec<String> = Vec::new();
        for (idx, pf) in prioritized.iter().enumerate() {
            let key = pf.finding.kind.label();
            groups.entry(key.clone()).or_insert_with(|| {
                order.push(key.clone());
                Vec::new()
            });
            if let Some(v) = groups.get_mut(&key) {
                v.push(idx);
            }
        }

        let mut recommendations = Vec::new();
        for key in order {
            let Some(indices) = groups.get(&key) else {
                continue;
            };
            let Some(&rep_idx) = indices.first() else {
                continue;
            };
            let representative = &prioritized[rep_idx];
            let remediation = self.catalog.suggest(&representative.finding);
            let supporting = indices.len();
            let rationale = format!(
                "{} finding(s) of kind '{}' detected; the most severe affects {} record(s) at priority {:?} (score {:.2}).",
                supporting,
                key,
                representative.finding.blast_radius.affected_records,
                representative.score.tier,
                representative.score.value
            );
            recommendations.push(Recommendation {
                id: Uuid::new_v4(),
                title: remediation
                    .first()
                    .map(|r| r.title.clone())
                    .unwrap_or_else(|| format!("Address {key} findings")),
                rationale,
                priority: representative.score.tier,
                finding_kind: Some(key),
                supporting_findings: supporting,
                confidence: representative.score.value,
                remediation,
            });
        }

        recommendations.sort_by(|a, b| {
            b.priority
                .cmp(&a.priority)
                .then(b.confidence.total_cmp(&a.confidence))
        });

        // Cross-cutting strategic recommendation.
        let pressing = prioritized
            .iter()
            .filter(|pf| pf.score.tier >= PriorityTier::High)
            .count();
        if pressing >= 3 {
            recommendations.insert(
                0,
                Recommendation {
                    id: Uuid::new_v4(),
                    title: "Convene a focused audit-remediation sprint".to_string(),
                    rationale: format!(
                        "{pressing} high-or-urgent findings are outstanding across multiple categories; a coordinated remediation effort will reduce risk faster than piecemeal fixes."
                    ),
                    priority: PriorityTier::Urgent,
                    finding_kind: None,
                    supporting_findings: pressing,
                    confidence: 0.9,
                    remediation: Vec::new(),
                },
            );
        }

        recommendations
    }
}

impl Default for InsightsEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Returns the most-recent `fraction` of records (by timestamp), with a floor of
/// one record, as an owned slice.
fn recent_window(records: &[AuditRecord], fraction: f64) -> Vec<AuditRecord> {
    if records.is_empty() {
        return Vec::new();
    }
    let mut sorted: Vec<AuditRecord> = records.to_vec();
    sorted.sort_by_key(|r| r.timestamp);
    let frac = fraction.clamp(0.0, 1.0);
    let take = ((records.len() as f64 * frac).ceil() as usize).clamp(1, records.len());
    sorted.split_off(records.len() - take)
}

/// Tallies findings into a summary.
fn summarize(prioritized: &[PrioritizedFinding], anomaly_count: usize) -> InsightsSummary {
    let mut summary = InsightsSummary {
        total_findings: prioritized.len(),
        urgent: 0,
        high: 0,
        medium: 0,
        low: 0,
        backlog: 0,
        anomaly_count,
        top_priority: prioritized.first().map(|pf| pf.score.value),
    };
    for pf in prioritized {
        match pf.score.tier {
            PriorityTier::Urgent => summary.urgent += 1,
            PriorityTier::High => summary.high += 1,
            PriorityTier::Medium => summary.medium += 1,
            PriorityTier::Low => summary.low += 1,
            PriorityTier::Backlog => summary.backlog += 1,
        }
    }
    summary
}

/// Builds an empty report for an empty corpus.
fn empty_report(generated_at: DateTime<Utc>) -> InsightsReport {
    InsightsReport {
        findings: Vec::new(),
        anomalies: Vec::new(),
        forecasts: Vec::new(),
        root_causes: Vec::new(),
        recommendations: Vec::new(),
        summary: InsightsSummary {
            total_findings: 0,
            urgent: 0,
            high: 0,
            medium: 0,
            low: 0,
            backlog: 0,
            anomaly_count: 0,
            top_priority: None,
        },
        generated_at,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, DecisionContext, EventType};
    use chrono::Duration;
    use std::collections::HashMap as StdHashMap;

    fn record(ts: DateTime<Utc>, statute: &str, effect: &str) -> AuditRecord {
        AuditRecord {
            id: Uuid::new_v4(),
            timestamp: ts,
            event_type: EventType::AutomaticDecision,
            actor: Actor::System {
                component: "engine".to_string(),
            },
            statute_id: statute.to_string(),
            subject_id: Uuid::new_v4(),
            context: DecisionContext::default(),
            result: DecisionResult::Deterministic {
                effect_applied: effect.to_string(),
                parameters: StdHashMap::new(),
            },
            previous_hash: None,
            record_hash: String::new(),
        }
    }

    fn voided(ts: DateTime<Utc>, statute: &str) -> AuditRecord {
        let mut r = record(ts, statute, "");
        r.result = DecisionResult::Void {
            reason: "logic error".to_string(),
        };
        r
    }

    #[test]
    fn test_empty_report() {
        let engine = InsightsEngine::new();
        let report = engine.analyze(&[]).expect("ok");
        assert_eq!(report.summary.total_findings, 0);
        assert!(report.recommendations.is_empty());
    }

    #[test]
    fn test_analyze_produces_findings_and_recommendations() {
        let engine = InsightsEngine::new();
        let base = Utc::now();
        let mut records = Vec::new();
        // Quiet baseline.
        for day in 1..12 {
            for _ in 0..2 {
                records.push(record(base - Duration::days(day), "statute-a", "approved"));
            }
        }
        // A loud spike day to guarantee a volume anomaly.
        for _ in 0..50 {
            records.push(record(base - Duration::days(3), "statute-a", "approved"));
        }

        let report = engine.analyze(&records).expect("ok");
        assert!(!report.findings.is_empty());
        assert!(report.summary.anomaly_count > 0);
        assert!(!report.recommendations.is_empty());
        // Findings must be sorted by descending priority.
        for pair in report.findings.windows(2) {
            assert!(pair[0].score.value >= pair[1].score.value);
        }
    }

    #[test]
    fn test_recommendations_carry_remediation() {
        let engine = InsightsEngine::new();
        let base = Utc::now();
        let mut records = Vec::new();
        for day in 1..15 {
            for _ in 0..2 {
                records.push(record(base - Duration::days(day), "statute-a", "approved"));
            }
        }
        for _ in 0..60 {
            records.push(record(base - Duration::days(2), "statute-a", "approved"));
        }
        let report = engine.analyze(&records).expect("ok");
        // At least one kind-specific recommendation has remediation steps.
        assert!(
            report
                .recommendations
                .iter()
                .any(|r| r.finding_kind.is_some() && !r.remediation.is_empty())
        );
    }

    #[test]
    fn test_void_cluster_detected() {
        let engine = InsightsEngine::new();
        let base = Utc::now();
        let mut records = Vec::new();
        // Clean history.
        for day in 5..25 {
            records.push(record(base - Duration::days(day), "s", "approved"));
        }
        // Recent burst of voids.
        for h in 0..8 {
            records.push(voided(base - Duration::hours(h), "s"));
        }
        let report = engine.analyze(&records).expect("ok");
        assert!(
            report
                .findings
                .iter()
                .any(|pf| pf.finding.kind == FindingKind::ElevatedVoidRate)
        );
    }

    #[test]
    fn test_root_cause_attached_for_top_findings() {
        let engine = InsightsEngine::new();
        let base = Utc::now();
        let subject = Uuid::new_v4();
        let mut records = Vec::new();
        for day in 1..15 {
            for _ in 0..2 {
                let mut r = record(base - Duration::days(day), "statute-a", "approved");
                r.subject_id = subject;
                records.push(r);
            }
        }
        for _ in 0..50 {
            let mut r = record(base - Duration::days(2), "statute-a", "approved");
            r.subject_id = subject;
            records.push(r);
        }
        let report = engine.analyze(&records).expect("ok");
        // The volume spike's evidence records share a subject/statute, so
        // root-cause should find correlated antecedents.
        assert!(!report.root_causes.is_empty());
    }

    #[test]
    fn test_track_improvement_end_to_end() {
        let engine = InsightsEngine::new();
        let now = Utc::now();
        let mut records = Vec::new();
        // Period 1 (older): many voids -> poor health.
        for h in 0..20 {
            records.push(voided(now - Duration::days(40) + Duration::hours(h), "s"));
        }
        // Period 2 (recent): clean approvals -> better health.
        for h in 0..20 {
            records.push(record(
                now - Duration::days(5) + Duration::hours(h),
                "s",
                "approved",
            ));
        }

        let periods = vec![
            AuditPeriod::new("older", now - Duration::days(45), now - Duration::days(30)),
            AuditPeriod::new("recent", now - Duration::days(10), now + Duration::days(1)),
        ];
        let report = engine
            .track_improvement(&records, periods)
            .expect("ok")
            .expect("two periods");
        assert_eq!(report.period_count, 2);
        // Health should be trending up from the void-heavy period to the clean one.
        assert_eq!(report.verdict, ImprovementDirection::Improving);
    }

    #[test]
    fn test_report_serializes_to_json() {
        let engine = InsightsEngine::new();
        let base = Utc::now();
        let mut records = Vec::new();
        for day in 1..12 {
            records.push(record(base - Duration::days(day), "s", "approved"));
        }
        let report = engine.analyze(&records).expect("ok");
        // The whole report (incl. nested maps with enum keys) must serialize.
        let json = serde_json::to_string(&report).expect("serialize");
        assert!(json.contains("findings"));
    }

    #[test]
    fn test_disable_flags() {
        let config = InsightsConfig {
            enable_anomaly: false,
            enable_prediction: false,
            enable_behavioral: false,
            enable_root_cause: false,
            ..Default::default()
        };
        let engine = InsightsEngine::with_config(config);
        let base = Utc::now();
        let records: Vec<AuditRecord> = (1..20)
            .map(|d| record(base - Duration::days(d), "s", "approved"))
            .collect();
        let report = engine.analyze(&records).expect("ok");
        assert!(report.findings.is_empty());
        assert!(report.anomalies.is_empty());
    }
}
