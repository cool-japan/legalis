//! Continuous-improvement tracking across audit periods.
//!
//! Insight is only actionable if you can tell whether things are getting better
//! or worse. This module distils each audit period into a [`PeriodMetrics`]
//! snapshot — outcome rates plus a composite *health score* — and then derives
//! period-over-period [`TrendMetric`]s (slope, percentage change, and a
//! direction verdict) so an audit programme can demonstrate progress, or catch
//! regressions, over time.

use crate::insights::finding::PrioritizedFinding;
use crate::{AuditRecord, DecisionResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A named, half-open time window `[start, end)`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditPeriod {
    /// Human-readable label (e.g. `"2026-Q1"`).
    pub label: String,
    /// Inclusive start.
    pub start: DateTime<Utc>,
    /// Exclusive end.
    pub end: DateTime<Utc>,
}

impl AuditPeriod {
    /// Creates a new period.
    pub fn new(label: impl Into<String>, start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        Self {
            label: label.into(),
            start,
            end,
        }
    }

    /// Returns `true` if `ts` falls within `[start, end)`.
    pub fn contains(&self, ts: DateTime<Utc>) -> bool {
        ts >= self.start && ts < self.end
    }
}

/// A snapshot of audit health for a single period.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PeriodMetrics {
    /// The period described.
    pub period: AuditPeriod,
    /// Number of decisions recorded in the period.
    pub total_decisions: usize,
    /// Fraction of decisions overridden by a human.
    pub override_rate: f64,
    /// Fraction of decisions routed to human discretion.
    pub discretion_rate: f64,
    /// Fraction of decisions voided due to a logical error.
    pub void_rate: f64,
    /// Number of findings raised in the period.
    pub finding_count: usize,
    /// Sum of the priority scores of the period's findings.
    pub weighted_finding_score: f64,
    /// Findings risk per decision (weighted score / decisions).
    pub finding_density: f64,
    /// Composite health score in `[0, 100]` (higher is better).
    pub health_score: f64,
}

impl PeriodMetrics {
    /// Computes metrics for a period from its records and (already period-scoped)
    /// prioritized findings.
    ///
    /// Records are filtered to those whose timestamp falls within the period;
    /// the supplied findings are taken as-is (the caller is responsible for
    /// scoping them to the period).
    pub fn compute(
        period: AuditPeriod,
        records: &[AuditRecord],
        findings: &[PrioritizedFinding],
    ) -> Self {
        let in_period: Vec<&AuditRecord> = records
            .iter()
            .filter(|r| period.contains(r.timestamp))
            .collect();
        let total = in_period.len();

        let overrides = in_period
            .iter()
            .filter(|r| matches!(r.result, DecisionResult::Overridden { .. }))
            .count();
        let discretion = in_period
            .iter()
            .filter(|r| matches!(r.result, DecisionResult::RequiresDiscretion { .. }))
            .count();
        let voids = in_period
            .iter()
            .filter(|r| matches!(r.result, DecisionResult::Void { .. }))
            .count();

        let denom = total.max(1) as f64;
        let override_rate = overrides as f64 / denom;
        let discretion_rate = discretion as f64 / denom;
        let void_rate = voids as f64 / denom;

        let weighted_finding_score: f64 = findings.iter().map(|f| f.score.value).sum();
        let finding_density = weighted_finding_score / denom;

        let health_score = Self::health(override_rate, void_rate, finding_density);

        Self {
            period,
            total_decisions: total,
            override_rate,
            discretion_rate,
            void_rate,
            finding_count: findings.len(),
            weighted_finding_score,
            finding_density,
            health_score,
        }
    }

    /// Composite health score in `[0, 100]`.
    ///
    /// Health starts at a perfect 100 and is eroded by override pressure, voided
    /// decisions (weighted most heavily, since they are outright failures), and
    /// the density of outstanding findings.
    fn health(override_rate: f64, void_rate: f64, finding_density: f64) -> f64 {
        // Voids are weighted at 1.0 (outright failures), override pressure and
        // finding density at 0.5 each.
        let penalty = 0.5 * override_rate.clamp(0.0, 1.0)
            + void_rate.clamp(0.0, 1.0)
            + 0.5 * finding_density.clamp(0.0, 1.0);
        (100.0 * (1.0 - penalty.clamp(0.0, 1.0))).clamp(0.0, 100.0)
    }
}

/// The qualitative direction of a trend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImprovementDirection {
    /// The metric is moving in the desirable direction.
    Improving,
    /// The metric is moving in the undesirable direction.
    Worsening,
    /// The metric is essentially flat.
    Stable,
}

/// A single metric's trajectory across periods.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrendMetric {
    /// Metric name.
    pub name: String,
    /// Per-period values in chronological order.
    pub values: Vec<f64>,
    /// Ordinary-least-squares slope versus period index.
    pub slope: f64,
    /// Percentage change from the first to the last period.
    pub pct_change: f64,
    /// Whether a rising value is desirable.
    pub higher_is_better: bool,
    /// The verdict.
    pub direction: ImprovementDirection,
}

impl TrendMetric {
    /// Fits a trend to a chronological series.
    ///
    /// `stable_band` is the magnitude (relative to the series mean) below which
    /// the slope is treated as flat.
    pub fn fit(
        name: impl Into<String>,
        values: Vec<f64>,
        higher_is_better: bool,
        stable_band: f64,
    ) -> Self {
        let slope = ols_slope(&values);
        let pct_change = percent_change(&values);
        let mean = if values.is_empty() {
            0.0
        } else {
            values.iter().sum::<f64>() / values.len() as f64
        };
        let normalized = slope / (mean.abs() + 1e-9);
        let direction = if normalized.abs() < stable_band {
            ImprovementDirection::Stable
        } else if (slope > 0.0) == higher_is_better {
            ImprovementDirection::Improving
        } else {
            ImprovementDirection::Worsening
        };

        Self {
            name: name.into(),
            values,
            slope,
            pct_change,
            higher_is_better,
            direction,
        }
    }
}

/// A continuous-improvement report across all tracked periods.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImprovementReport {
    /// Trend for each tracked metric.
    pub metrics: Vec<TrendMetric>,
    /// The headline health-score trend.
    pub health_trend: TrendMetric,
    /// Overall verdict (driven by the health-score trend).
    pub verdict: ImprovementDirection,
    /// Narrative summary.
    pub summary: String,
    /// Number of periods analysed.
    pub period_count: usize,
}

/// Configuration for [`ImprovementTracker`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImprovementConfig {
    /// Relative slope magnitude below which a trend is "stable".
    pub stable_band: f64,
}

impl Default for ImprovementConfig {
    fn default() -> Self {
        Self { stable_band: 0.05 }
    }
}

/// Accumulates period metrics and derives improvement trends.
#[derive(Debug, Clone)]
pub struct ImprovementTracker {
    config: ImprovementConfig,
    periods: Vec<PeriodMetrics>,
}

impl ImprovementTracker {
    /// Creates a tracker with the default configuration.
    pub fn new() -> Self {
        Self::with_config(ImprovementConfig::default())
    }

    /// Creates a tracker with a custom configuration.
    pub fn with_config(config: ImprovementConfig) -> Self {
        Self {
            config,
            periods: Vec::new(),
        }
    }

    /// Adds a period snapshot, keeping snapshots ordered by start time.
    pub fn add_period(&mut self, metrics: PeriodMetrics) {
        self.periods.push(metrics);
        self.periods.sort_by_key(|a| a.period.start);
    }

    /// Returns the tracked period snapshots, in chronological order.
    pub fn periods(&self) -> &[PeriodMetrics] {
        &self.periods
    }

    /// Builds an improvement report, or `None` if fewer than two periods are
    /// tracked (a trend needs at least two points).
    pub fn report(&self) -> Option<ImprovementReport> {
        if self.periods.len() < 2 {
            return None;
        }

        let band = self.config.stable_band;
        let series =
            |f: fn(&PeriodMetrics) -> f64| -> Vec<f64> { self.periods.iter().map(f).collect() };

        let health_trend = TrendMetric::fit("health_score", series(|m| m.health_score), true, band);

        let metrics = vec![
            health_trend.clone(),
            TrendMetric::fit("override_rate", series(|m| m.override_rate), false, band),
            TrendMetric::fit("void_rate", series(|m| m.void_rate), false, band),
            TrendMetric::fit(
                "discretion_rate",
                series(|m| m.discretion_rate),
                false,
                band,
            ),
            TrendMetric::fit(
                "finding_density",
                series(|m| m.finding_density),
                false,
                band,
            ),
        ];

        let verdict = health_trend.direction;
        let summary = format!(
            "Across {} periods the health score moved from {:.1} to {:.1} ({:+.1}%): {}.",
            self.periods.len(),
            self.periods.first().map(|m| m.health_score).unwrap_or(0.0),
            self.periods.last().map(|m| m.health_score).unwrap_or(0.0),
            health_trend.pct_change,
            match verdict {
                ImprovementDirection::Improving => "improving",
                ImprovementDirection::Worsening => "regressing",
                ImprovementDirection::Stable => "holding steady",
            }
        );

        Some(ImprovementReport {
            metrics,
            health_trend,
            verdict,
            summary,
            period_count: self.periods.len(),
        })
    }
}

impl Default for ImprovementTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Ordinary-least-squares slope of `values` against the index `0, 1, 2, ...`.
fn ols_slope(values: &[f64]) -> f64 {
    let n = values.len();
    if n < 2 {
        return 0.0;
    }
    let n_f = n as f64;
    let mean_x = (n_f - 1.0) / 2.0;
    let mean_y = values.iter().sum::<f64>() / n_f;
    let mut cov = 0.0;
    let mut var_x = 0.0;
    for (i, &y) in values.iter().enumerate() {
        let dx = i as f64 - mean_x;
        cov += dx * (y - mean_y);
        var_x += dx * dx;
    }
    if var_x.abs() < 1e-12 {
        0.0
    } else {
        cov / var_x
    }
}

/// Percentage change from the first to the last value.
fn percent_change(values: &[f64]) -> f64 {
    match (values.first(), values.last()) {
        (Some(&first), Some(&last)) => (last - first) / (first.abs() + 1e-9) * 100.0,
        _ => 0.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::insights::finding::{
        AuditFinding, BlastRadius, FindingKind, FindingPrioritizer, Likelihood, Severity,
    };
    use crate::{Actor, DecisionContext, EventType};
    use chrono::Duration;
    use std::collections::HashMap as StdHashMap;
    use uuid::Uuid;

    fn period(label: &str, days_ago_start: i64, days_ago_end: i64) -> AuditPeriod {
        let now = Utc::now();
        AuditPeriod::new(
            label,
            now - Duration::days(days_ago_start),
            now - Duration::days(days_ago_end),
        )
    }

    fn record(ts: DateTime<Utc>, result: DecisionResult) -> AuditRecord {
        AuditRecord {
            id: Uuid::new_v4(),
            timestamp: ts,
            event_type: EventType::AutomaticDecision,
            actor: Actor::System {
                component: "t".to_string(),
            },
            statute_id: "s".to_string(),
            subject_id: Uuid::new_v4(),
            context: DecisionContext::default(),
            result,
            previous_hash: None,
            record_hash: String::new(),
        }
    }

    fn approved() -> DecisionResult {
        DecisionResult::Deterministic {
            effect_applied: "approved".to_string(),
            parameters: StdHashMap::new(),
        }
    }

    fn voided() -> DecisionResult {
        DecisionResult::Void {
            reason: "err".to_string(),
        }
    }

    #[test]
    fn test_period_metrics_basic() {
        let p = period("p", 10, 0);
        let mid = Utc::now() - Duration::days(5);
        let records = vec![record(mid, approved()), record(mid, voided())];
        let metrics = PeriodMetrics::compute(p, &records, &[]);
        assert_eq!(metrics.total_decisions, 2);
        assert!((metrics.void_rate - 0.5).abs() < 1e-9);
        // 50% voids drive a meaningful health penalty.
        assert!(metrics.health_score < 60.0);
    }

    #[test]
    fn test_period_metrics_filters_by_window() {
        let p = period("p", 10, 5);
        let inside = Utc::now() - Duration::days(7);
        let outside = Utc::now() - Duration::days(1);
        let records = vec![record(inside, approved()), record(outside, approved())];
        let metrics = PeriodMetrics::compute(p, &records, &[]);
        assert_eq!(metrics.total_decisions, 1);
    }

    #[test]
    fn test_findings_lower_health() {
        let p = period("p", 10, 0);
        let mid = Utc::now() - Duration::days(5);
        let records: Vec<AuditRecord> = (0..10).map(|_| record(mid, approved())).collect();

        let prioritizer = FindingPrioritizer::new();
        let findings = prioritizer.prioritize(vec![AuditFinding::new(
            FindingKind::OutcomeDrift,
            "drift",
            Severity::High,
            Likelihood::Likely,
            BlastRadius::from_counts(10, 10, 1),
        )]);

        let clean = PeriodMetrics::compute(p.clone(), &records, &[]);
        let dirty = PeriodMetrics::compute(p, &records, &findings);
        assert!(dirty.health_score <= clean.health_score);
        assert_eq!(dirty.finding_count, 1);
    }

    #[test]
    fn test_ols_slope_and_pct_change() {
        assert!((ols_slope(&[1.0, 2.0, 3.0, 4.0]) - 1.0).abs() < 1e-9);
        assert!(ols_slope(&[4.0, 3.0, 2.0, 1.0]) < 0.0);
        assert!((percent_change(&[10.0, 20.0]) - 100.0).abs() < 1e-6);
    }

    #[test]
    fn test_tracker_detects_improvement() {
        let mut tracker = ImprovementTracker::new();
        // Three periods with rising health scores.
        for (i, score) in [40.0_f64, 60.0, 85.0].iter().enumerate() {
            let p = period(&format!("p{i}"), (30 - i as i64 * 10).max(1), 0);
            let mut metrics = PeriodMetrics::compute(p, &[], &[]);
            metrics.health_score = *score;
            tracker.add_period(metrics);
        }
        let report = tracker.report().expect("two+ periods");
        assert_eq!(report.verdict, ImprovementDirection::Improving);
        assert_eq!(report.period_count, 3);
        assert!(report.health_trend.slope > 0.0);
    }

    #[test]
    fn test_tracker_detects_regression() {
        let mut tracker = ImprovementTracker::new();
        for (i, score) in [90.0_f64, 70.0, 45.0].iter().enumerate() {
            let p = period(&format!("p{i}"), (30 - i as i64 * 10).max(1), 0);
            let mut metrics = PeriodMetrics::compute(p, &[], &[]);
            metrics.health_score = *score;
            tracker.add_period(metrics);
        }
        let report = tracker.report().expect("two+ periods");
        assert_eq!(report.verdict, ImprovementDirection::Worsening);
    }

    #[test]
    fn test_tracker_needs_two_periods() {
        let mut tracker = ImprovementTracker::new();
        tracker.add_period(PeriodMetrics::compute(period("only", 5, 0), &[], &[]));
        assert!(tracker.report().is_none());
    }
}
