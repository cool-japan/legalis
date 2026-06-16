//! Stream anomaly detection over audit-event time series.
//!
//! This detector operates on *aggregated* event streams rather than on
//! individual record feature vectors (the per-record approach lives in
//! [`crate::ml_anomaly`]). Records are folded into fixed time buckets and the
//! resulting count series is analysed with robust statistics that tolerate the
//! heavy tails typical of audit data:
//!
//! - **Volume outliers** — buckets whose decision count has a large modified
//!   z-score (median / MAD based) or falls outside the Tukey IQR fences.
//! - **Frequency spikes** — individual statutes whose per-bucket volume jumps
//!   well above their own robust baseline.
//! - **Rare events** — statutes that are exercised so infrequently that they
//!   warrant review (rarely-tested rules are a latent compliance risk).
//! - **Baseline drift** — a regime change in which the recent half of the
//!   series settles around a materially different level than the earlier half.
//!
//! Robust estimators (median, median-absolute-deviation, quartiles) are used
//! throughout because a handful of extreme buckets would otherwise corrupt a
//! mean / standard-deviation baseline and mask the very anomalies we seek.

use crate::insights::finding::{AuditFinding, BlastRadius, FindingKind, Likelihood, Severity};
use crate::{AuditError, AuditRecord, AuditResult};
use chrono::{DateTime, Datelike, Duration, Timelike, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};
use uuid::Uuid;

/// The temporal resolution used to bucket the event stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeGranularity {
    /// One bucket per calendar hour.
    Hourly,
    /// One bucket per calendar day.
    Daily,
    /// One bucket per ISO week (Monday-anchored).
    Weekly,
}

impl TimeGranularity {
    /// Floors a timestamp to the start of its bucket.
    pub fn floor(self, ts: DateTime<Utc>) -> DateTime<Utc> {
        let day = ts
            .with_hour(0)
            .and_then(|t| t.with_minute(0))
            .and_then(|t| t.with_second(0))
            .and_then(|t| t.with_nanosecond(0))
            .unwrap_or(ts);
        match self {
            TimeGranularity::Hourly => ts
                .with_minute(0)
                .and_then(|t| t.with_second(0))
                .and_then(|t| t.with_nanosecond(0))
                .unwrap_or(ts),
            TimeGranularity::Daily => day,
            TimeGranularity::Weekly => {
                let offset = ts.weekday().num_days_from_monday() as i64;
                day - Duration::days(offset)
            }
        }
    }

    /// Returns the span of a single bucket.
    pub fn bucket_span(self) -> Duration {
        match self {
            TimeGranularity::Hourly => Duration::hours(1),
            TimeGranularity::Daily => Duration::days(1),
            TimeGranularity::Weekly => Duration::weeks(1),
        }
    }
}

/// Tuning parameters for the stream anomaly detector.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamAnomalyConfig {
    /// Bucketing resolution.
    pub granularity: TimeGranularity,
    /// Modified z-score above which a value is flagged (Iglewicz-Hoaglin
    /// suggest 3.5 as a conservative default).
    pub mad_threshold: f64,
    /// Multiplier applied to the IQR when forming Tukey outlier fences.
    pub iqr_multiplier: f64,
    /// Drift score above which the recent regime is flagged as drifted.
    pub drift_threshold: f64,
    /// Maximum relative share for a statute to count as a "rare" event.
    pub rare_event_max_share: f64,
    /// Maximum absolute occurrences for a statute to count as a "rare" event.
    pub rare_event_max_count: usize,
    /// Minimum bucket count for a category spike to be considered (noise gate).
    pub min_category_count: usize,
    /// Minimum number of buckets required before volume/drift baselining runs.
    pub min_buckets: usize,
    /// Maximum number of evidence record identifiers attached to a finding.
    pub max_evidence: usize,
}

impl Default for StreamAnomalyConfig {
    fn default() -> Self {
        Self {
            granularity: TimeGranularity::Daily,
            mad_threshold: 3.5,
            iqr_multiplier: 1.5,
            drift_threshold: 2.0,
            rare_event_max_share: 0.05,
            rare_event_max_count: 3,
            min_category_count: 3,
            min_buckets: 4,
            max_evidence: 100,
        }
    }
}

/// A robust baseline model fitted to a numeric series.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaselineModel {
    /// Robust centre (median).
    pub center: f64,
    /// Median absolute deviation.
    pub mad: f64,
    /// Mean absolute deviation (fallback scale when MAD collapses to zero).
    pub mean_abs_dev: f64,
    /// First quartile.
    pub q1: f64,
    /// Third quartile.
    pub q3: f64,
    /// Lower Tukey fence.
    pub lower_fence: f64,
    /// Upper Tukey fence.
    pub upper_fence: f64,
    /// Arithmetic mean (reported for reference).
    pub mean: f64,
    /// Standard deviation (reported for reference).
    pub std_dev: f64,
    /// Number of observations used to fit the model.
    pub sample_size: usize,
}

impl BaselineModel {
    /// Fits a baseline to the given series, or returns `None` if it is empty.
    pub fn build(values: &[f64], iqr_multiplier: f64) -> Option<Self> {
        if values.is_empty() {
            return None;
        }
        let mut sorted = values.to_vec();
        sorted.sort_by(|a, b| a.total_cmp(b));

        let center = median_sorted(&sorted);
        let deviations: Vec<f64> = sorted.iter().map(|v| (v - center).abs()).collect();
        let mut sorted_dev = deviations.clone();
        sorted_dev.sort_by(|a, b| a.total_cmp(b));
        let mad = median_sorted(&sorted_dev);
        let mean_abs_dev = deviations.iter().sum::<f64>() / deviations.len() as f64;

        let q1 = quantile_sorted(&sorted, 0.25);
        let q3 = quantile_sorted(&sorted, 0.75);
        let iqr = q3 - q1;
        let lower_fence = q1 - iqr_multiplier * iqr;
        let upper_fence = q3 + iqr_multiplier * iqr;

        let mean = sorted.iter().sum::<f64>() / sorted.len() as f64;
        let variance = sorted.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / sorted.len() as f64;
        let std_dev = variance.sqrt();

        Some(Self {
            center,
            mad,
            mean_abs_dev,
            q1,
            q3,
            lower_fence,
            upper_fence,
            mean,
            std_dev,
            sample_size: sorted.len(),
        })
    }

    /// Returns the (signed) modified z-score of `value` against this baseline.
    ///
    /// Uses the MAD-based estimator and falls back to the mean-absolute
    /// deviation when the MAD is zero (a tie-heavy series), so the score is
    /// always finite.
    pub fn modified_z(&self, value: f64) -> f64 {
        if self.mad > 1e-12 {
            0.6745 * (value - self.center) / self.mad
        } else if self.mean_abs_dev > 1e-12 {
            (value - self.center) / (1.253_314 * self.mean_abs_dev)
        } else {
            0.0
        }
    }

    /// Returns `true` if `value` lies outside the Tukey IQR fences.
    pub fn is_iqr_outlier(&self, value: f64) -> bool {
        value < self.lower_fence || value > self.upper_fence
    }
}

/// An anomaly detected in the aggregated event stream.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamAnomaly {
    /// The category of anomaly (maps directly onto a [`FindingKind`]).
    pub kind: FindingKind,
    /// The dimension that was anomalous (e.g. `"volume"` or `"statute:foo"`).
    pub dimension: String,
    /// Start of the implicated bucket.
    pub bucket_start: DateTime<Utc>,
    /// End of the implicated bucket.
    pub bucket_end: DateTime<Utc>,
    /// The observed value.
    pub observed: f64,
    /// The baseline expectation.
    pub expected: f64,
    /// The robust deviation score driving the detection.
    pub deviation_score: f64,
    /// Confidence in `[0, 1]` that the deviation is genuine.
    pub confidence: f64,
    /// Number of records implicated.
    pub affected_records: usize,
    /// Number of distinct subjects implicated.
    pub affected_subjects: usize,
    /// Number of distinct statutes implicated.
    pub affected_statutes: usize,
    /// Evidence record identifiers (capped by configuration).
    pub record_ids: Vec<Uuid>,
    /// Human-readable explanation.
    pub explanation: String,
}

impl StreamAnomaly {
    /// Converts the anomaly into a fully-formed [`AuditFinding`], deriving the
    /// severity and likelihood bands from the anomaly kind and confidence.
    pub fn to_finding(&self) -> AuditFinding {
        let base_severity = match self.kind {
            FindingKind::BaselineDrift => Severity::High,
            FindingKind::RareEvent => Severity::Low,
            _ => Severity::Medium,
        };
        let severity = if self.confidence >= 0.85 {
            escalate(base_severity)
        } else {
            base_severity
        };
        let likelihood = Likelihood::from_confidence(self.confidence);
        let blast = BlastRadius::from_counts(
            self.affected_records,
            self.affected_subjects,
            self.affected_statutes,
        );

        AuditFinding::new(self.kind.clone(), self.title(), severity, likelihood, blast)
            .with_description(self.explanation.clone())
            .with_evidence(self.record_ids.clone())
            .with_metric("observed", self.observed)
            .with_metric("expected", self.expected)
            .with_metric("deviation_score", self.deviation_score)
            .with_metric("confidence", self.confidence)
    }

    fn title(&self) -> String {
        match self.kind {
            FindingKind::VolumeSpike => format!("Volume spike in {}", self.dimension),
            FindingKind::VolumeDrop => format!("Volume drop in {}", self.dimension),
            FindingKind::FrequencySpike => format!("Frequency spike in {}", self.dimension),
            FindingKind::RareEvent => format!("Rarely-exercised {}", self.dimension),
            FindingKind::BaselineDrift => format!("Baseline drift in {}", self.dimension),
            _ => format!("Anomaly in {}", self.dimension),
        }
    }
}

fn escalate(severity: Severity) -> Severity {
    match severity {
        Severity::Info => Severity::Low,
        Severity::Low => Severity::Medium,
        Severity::Medium => Severity::High,
        Severity::High | Severity::Critical => Severity::Critical,
    }
}

/// Per-bucket aggregation state.
struct Bucket {
    start: DateTime<Utc>,
    record_ids: Vec<Uuid>,
    subjects: HashSet<Uuid>,
    statutes: HashSet<String>,
    by_statute: HashMap<String, Vec<Uuid>>,
}

/// Detects anomalies across an aggregated audit-event stream.
#[derive(Debug, Clone)]
pub struct StreamAnomalyDetector {
    config: StreamAnomalyConfig,
}

impl StreamAnomalyDetector {
    /// Creates a detector with the default configuration.
    pub fn new() -> Self {
        Self::with_config(StreamAnomalyConfig::default())
    }

    /// Creates a detector with a custom configuration.
    pub fn with_config(config: StreamAnomalyConfig) -> Self {
        Self { config }
    }

    /// Returns the active configuration.
    pub fn config(&self) -> &StreamAnomalyConfig {
        &self.config
    }

    /// Runs every sub-detector over `records` and returns the merged result,
    /// sorted by descending confidence.
    pub fn detect(&self, records: &[AuditRecord]) -> AuditResult<Vec<StreamAnomaly>> {
        if self.config.mad_threshold <= 0.0 || self.config.iqr_multiplier <= 0.0 {
            return Err(AuditError::InvalidRecord(
                "stream anomaly thresholds must be positive".to_string(),
            ));
        }
        if records.is_empty() {
            return Ok(Vec::new());
        }

        let buckets = self.build_buckets(records);
        let mut anomalies = Vec::new();

        anomalies.extend(self.detect_volume(&buckets));
        anomalies.extend(self.detect_frequency_spikes(&buckets));
        anomalies.extend(self.detect_rare_events(records));
        anomalies.extend(self.detect_baseline_drift(&buckets));

        anomalies.sort_by(|a, b| b.confidence.total_cmp(&a.confidence));
        Ok(anomalies)
    }

    /// Builds the time-ordered bucket map.
    fn build_buckets(&self, records: &[AuditRecord]) -> BTreeMap<i64, Bucket> {
        let mut buckets: BTreeMap<i64, Bucket> = BTreeMap::new();
        for record in records {
            let start = self.config.granularity.floor(record.timestamp);
            let key = start.timestamp();
            let bucket = buckets.entry(key).or_insert_with(|| Bucket {
                start,
                record_ids: Vec::new(),
                subjects: HashSet::new(),
                statutes: HashSet::new(),
                by_statute: HashMap::new(),
            });
            bucket.record_ids.push(record.id);
            bucket.subjects.insert(record.subject_id);
            bucket.statutes.insert(record.statute_id.clone());
            bucket
                .by_statute
                .entry(record.statute_id.clone())
                .or_default()
                .push(record.id);
        }
        buckets
    }

    /// Flags buckets whose total volume is a robust outlier.
    fn detect_volume(&self, buckets: &BTreeMap<i64, Bucket>) -> Vec<StreamAnomaly> {
        let mut anomalies = Vec::new();
        if buckets.len() < self.config.min_buckets {
            return anomalies;
        }

        let counts: Vec<f64> = buckets
            .values()
            .map(|b| b.record_ids.len() as f64)
            .collect();
        let Some(model) = BaselineModel::build(&counts, self.config.iqr_multiplier) else {
            return anomalies;
        };

        let span = self.config.granularity.bucket_span();
        for bucket in buckets.values() {
            let observed = bucket.record_ids.len() as f64;
            let z = model.modified_z(observed);
            let iqr_outlier = model.is_iqr_outlier(observed);
            if z.abs() < self.config.mad_threshold && !iqr_outlier {
                continue;
            }

            let kind = if z >= 0.0 {
                FindingKind::VolumeSpike
            } else {
                FindingKind::VolumeDrop
            };
            let confidence = confidence_from_score(z.abs(), self.config.mad_threshold);
            let direction = if z >= 0.0 { "above" } else { "below" };
            let (ids, n_records, n_subjects, n_statutes) =
                summarize_bucket(bucket, self.config.max_evidence);

            anomalies.push(StreamAnomaly {
                kind,
                dimension: "volume".to_string(),
                bucket_start: bucket.start,
                bucket_end: bucket.start + span,
                observed,
                expected: model.center,
                deviation_score: z.abs(),
                confidence,
                affected_records: n_records,
                affected_subjects: n_subjects,
                affected_statutes: n_statutes,
                record_ids: ids,
                explanation: format!(
                    "Bucket starting {} recorded {} decisions, {} the robust baseline of {:.1} (modified z = {:.2}).",
                    bucket.start.to_rfc3339(),
                    bucket.record_ids.len(),
                    direction,
                    model.center,
                    z
                ),
            });
        }
        anomalies
    }

    /// Flags individual statutes whose per-bucket volume spikes versus their
    /// own robust baseline.
    fn detect_frequency_spikes(&self, buckets: &BTreeMap<i64, Bucket>) -> Vec<StreamAnomaly> {
        let mut anomalies = Vec::new();
        if buckets.len() < self.config.min_buckets {
            return anomalies;
        }

        // Collect the full statute universe.
        let mut statutes: HashSet<String> = HashSet::new();
        for bucket in buckets.values() {
            for statute in bucket.statutes.iter() {
                statutes.insert(statute.clone());
            }
        }

        let ordered: Vec<&Bucket> = buckets.values().collect();
        let span = self.config.granularity.bucket_span();

        for statute in statutes {
            // Per-bucket counts for this statute, including zero buckets so the
            // baseline reflects how sparse the statute usually is.
            let series: Vec<f64> = ordered
                .iter()
                .map(|b| b.by_statute.get(&statute).map_or(0, |v| v.len()) as f64)
                .collect();
            let Some(model) = BaselineModel::build(&series, self.config.iqr_multiplier) else {
                continue;
            };

            for bucket in ordered.iter() {
                let ids = match bucket.by_statute.get(&statute) {
                    Some(ids) => ids,
                    None => continue,
                };
                let observed = ids.len() as f64;
                if ids.len() < self.config.min_category_count {
                    continue;
                }
                let z = model.modified_z(observed);
                if z < self.config.mad_threshold {
                    continue;
                }
                let confidence = confidence_from_score(z, self.config.mad_threshold);
                let evidence: Vec<Uuid> =
                    ids.iter().take(self.config.max_evidence).copied().collect();
                let subjects = bucket
                    .record_ids
                    .iter()
                    .filter(|id| ids.contains(id))
                    .count()
                    .max(1);

                anomalies.push(StreamAnomaly {
                    kind: FindingKind::FrequencySpike,
                    dimension: format!("statute:{statute}"),
                    bucket_start: bucket.start,
                    bucket_end: bucket.start + span,
                    observed,
                    expected: model.center,
                    deviation_score: z,
                    confidence,
                    affected_records: ids.len(),
                    affected_subjects: subjects.min(ids.len()),
                    affected_statutes: 1,
                    record_ids: evidence,
                    explanation: format!(
                        "Statute '{}' was applied {} times in the bucket starting {}, far above its baseline of {:.1} (modified z = {:.2}).",
                        statute,
                        ids.len(),
                        bucket.start.to_rfc3339(),
                        model.center,
                        z
                    ),
                });
            }
        }
        anomalies
    }

    /// Flags statutes that are exercised so rarely they warrant review.
    fn detect_rare_events(&self, records: &[AuditRecord]) -> Vec<StreamAnomaly> {
        let mut anomalies = Vec::new();
        let total = records.len();
        if total < self.config.min_buckets {
            return anomalies;
        }

        let mut by_statute: HashMap<String, Vec<Uuid>> = HashMap::new();
        let mut subjects: HashMap<String, HashSet<Uuid>> = HashMap::new();
        for record in records {
            by_statute
                .entry(record.statute_id.clone())
                .or_default()
                .push(record.id);
            subjects
                .entry(record.statute_id.clone())
                .or_default()
                .insert(record.subject_id);
        }

        // A "rare" finding only makes sense when there is a richer population to
        // contrast against; skip degenerate single-statute corpora.
        if by_statute.len() < 2 {
            return anomalies;
        }

        for (statute, ids) in by_statute {
            let count = ids.len();
            let share = count as f64 / total as f64;
            if count > self.config.rare_event_max_count || share > self.config.rare_event_max_share
            {
                continue;
            }
            let n_subjects = subjects.get(&statute).map_or(0, |s| s.len());
            let evidence: Vec<Uuid> = ids.iter().take(self.config.max_evidence).copied().collect();
            anomalies.push(StreamAnomaly {
                kind: FindingKind::RareEvent,
                dimension: format!("statute:{statute}"),
                bucket_start: Utc::now(),
                bucket_end: Utc::now(),
                observed: count as f64,
                expected: self.config.rare_event_max_share * total as f64,
                deviation_score: 1.0 - share,
                confidence: 0.45,
                affected_records: count,
                affected_subjects: n_subjects,
                affected_statutes: 1,
                record_ids: evidence,
                explanation: format!(
                    "Statute '{}' was exercised only {} time(s) ({:.2}% of activity); rarely-tested rules carry latent risk.",
                    statute,
                    count,
                    share * 100.0
                ),
            });
        }
        anomalies
    }

    /// Detects a regime change between the earlier and recent halves of the
    /// volume series.
    fn detect_baseline_drift(&self, buckets: &BTreeMap<i64, Bucket>) -> Vec<StreamAnomaly> {
        let mut anomalies = Vec::new();
        let ordered: Vec<&Bucket> = buckets.values().collect();
        if ordered.len() < self.config.min_buckets {
            return anomalies;
        }

        let mid = ordered.len() / 2;
        let earlier: Vec<f64> = ordered[..mid]
            .iter()
            .map(|b| b.record_ids.len() as f64)
            .collect();
        let recent: Vec<f64> = ordered[mid..]
            .iter()
            .map(|b| b.record_ids.len() as f64)
            .collect();

        let (Some(earlier_model), Some(recent_model)) = (
            BaselineModel::build(&earlier, self.config.iqr_multiplier),
            BaselineModel::build(&recent, self.config.iqr_multiplier),
        ) else {
            return anomalies;
        };

        // 1.4826 * MAD is a consistent estimator of the standard deviation for
        // normally-distributed data; guard against a collapsed scale.
        let scale = (1.4826 * earlier_model.mad)
            .max(earlier_model.std_dev)
            .max(1.0);
        let drift_score = (recent_model.center - earlier_model.center).abs() / scale;
        if drift_score < self.config.drift_threshold {
            return anomalies;
        }

        let span = self.config.granularity.bucket_span();
        let mut ids = Vec::new();
        let mut subjects = HashSet::new();
        let mut statutes = HashSet::new();
        let mut total_records = 0usize;
        for bucket in ordered[mid..].iter() {
            total_records += bucket.record_ids.len();
            for id in bucket.record_ids.iter() {
                if ids.len() < self.config.max_evidence {
                    ids.push(*id);
                }
            }
            subjects.extend(bucket.subjects.iter().copied());
            statutes.extend(bucket.statutes.iter().cloned());
        }
        let direction = if recent_model.center > earlier_model.center {
            "upward"
        } else {
            "downward"
        };

        let last_start = ordered.last().map_or_else(Utc::now, |b| b.start);
        anomalies.push(StreamAnomaly {
            kind: FindingKind::BaselineDrift,
            dimension: "volume".to_string(),
            bucket_start: ordered[mid].start,
            bucket_end: last_start + span,
            observed: recent_model.center,
            expected: earlier_model.center,
            deviation_score: drift_score,
            confidence: confidence_from_score(drift_score, self.config.drift_threshold),
            affected_records: total_records,
            affected_subjects: subjects.len(),
            affected_statutes: statutes.len(),
            record_ids: ids,
            explanation: format!(
                "Decision volume drifted {} from a baseline median of {:.1} to {:.1} (drift score = {:.2}).",
                direction, earlier_model.center, recent_model.center, drift_score
            ),
        });
        anomalies
    }
}

impl Default for StreamAnomalyDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Maps a robust deviation score onto a confidence in `[0, 1]`.
///
/// At exactly the threshold the confidence is `0.5`; it saturates to `1.0` at
/// twice the threshold.
fn confidence_from_score(score: f64, threshold: f64) -> f64 {
    if threshold <= 0.0 {
        return 0.0;
    }
    (score / (2.0 * threshold)).clamp(0.0, 1.0)
}

/// Summarises a bucket into capped evidence and distinct-entity counts.
fn summarize_bucket(bucket: &Bucket, max_evidence: usize) -> (Vec<Uuid>, usize, usize, usize) {
    let ids: Vec<Uuid> = bucket
        .record_ids
        .iter()
        .take(max_evidence)
        .copied()
        .collect();
    (
        ids,
        bucket.record_ids.len(),
        bucket.subjects.len(),
        bucket.statutes.len(),
    )
}

/// Returns the median of a slice that is already sorted ascending.
fn median_sorted(sorted: &[f64]) -> f64 {
    let n = sorted.len();
    if n == 0 {
        return 0.0;
    }
    if n % 2 == 1 {
        sorted[n / 2]
    } else {
        (sorted[n / 2 - 1] + sorted[n / 2]) / 2.0
    }
}

/// Linear-interpolation quantile (the "type 7" / NumPy default) over a slice
/// already sorted ascending.
fn quantile_sorted(sorted: &[f64], q: f64) -> f64 {
    let n = sorted.len();
    if n == 0 {
        return 0.0;
    }
    if n == 1 {
        return sorted[0];
    }
    let pos = q.clamp(0.0, 1.0) * (n - 1) as f64;
    let lower = pos.floor() as usize;
    let upper = pos.ceil() as usize;
    if lower == upper {
        sorted[lower]
    } else {
        let frac = pos - lower as f64;
        sorted[lower] * (1.0 - frac) + sorted[upper] * frac
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use std::collections::HashMap as StdHashMap;

    fn record_at(ts: DateTime<Utc>, statute: &str) -> AuditRecord {
        AuditRecord {
            id: Uuid::new_v4(),
            timestamp: ts,
            event_type: EventType::AutomaticDecision,
            actor: Actor::System {
                component: "test".to_string(),
            },
            statute_id: statute.to_string(),
            subject_id: Uuid::new_v4(),
            context: DecisionContext::default(),
            result: DecisionResult::Deterministic {
                effect_applied: "approved".to_string(),
                parameters: StdHashMap::new(),
            },
            previous_hash: None,
            record_hash: String::new(),
        }
    }

    #[test]
    fn test_robust_stats_helpers() {
        let mut v: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 100.0];
        v.sort_by(|a, b| a.total_cmp(b));
        assert!((median_sorted(&v) - 3.0).abs() < 1e-9);
        // 100 is a clear outlier; the median is unmoved by it.
        assert!((quantile_sorted(&v, 0.25) - 2.0).abs() < 1e-9);
        assert!((quantile_sorted(&v, 0.75) - 4.0).abs() < 1e-9);
    }

    #[test]
    fn test_baseline_model_robustness() {
        let series = vec![10.0, 11.0, 9.0, 10.0, 12.0, 8.0, 80.0];
        let model = BaselineModel::build(&series, 1.5).expect("non-empty");
        // The robust centre tracks the bulk of the data, not the 80 outlier.
        assert!(model.center >= 9.0 && model.center <= 12.0);
        assert!(model.modified_z(80.0) > 3.5);
        assert!(model.is_iqr_outlier(80.0));
        assert!(!model.is_iqr_outlier(10.0));
    }

    #[test]
    fn test_baseline_model_zero_mad_fallback() {
        // All-equal except one: MAD is zero, so the mean-abs-dev fallback runs.
        let series = vec![5.0, 5.0, 5.0, 5.0, 5.0, 25.0];
        let model = BaselineModel::build(&series, 1.5).expect("non-empty");
        assert!(model.modified_z(25.0).abs() > 0.0);
        assert!(model.modified_z(5.0).abs() < 1.0);
    }

    #[test]
    fn test_detect_volume_spike() {
        let detector = StreamAnomalyDetector::new();
        let base = Utc::now()
            .with_hour(12)
            .and_then(|t| t.with_minute(0))
            .and_then(|t| t.with_second(0))
            .and_then(|t| t.with_nanosecond(0))
            .unwrap_or_else(Utc::now);

        let mut records = Vec::new();
        // Ten quiet days with two decisions each.
        for day in 0..10 {
            let ts = base - Duration::days(day);
            for _ in 0..2 {
                records.push(record_at(ts, "statute-a"));
            }
        }
        // One loud day with a large spike.
        let spike_day = base - Duration::days(3);
        for _ in 0..40 {
            records.push(record_at(spike_day, "statute-a"));
        }

        let anomalies = detector.detect(&records).expect("detect ok");
        assert!(
            anomalies
                .iter()
                .any(|a| a.kind == FindingKind::VolumeSpike && a.dimension == "volume")
        );
    }

    #[test]
    fn test_detect_rare_event() {
        let detector = StreamAnomalyDetector::new();
        let base = Utc::now();
        let mut records = Vec::new();
        // A dominant statute.
        for day in 0..20 {
            records.push(record_at(base - Duration::days(day), "statute-common"));
        }
        // A single rarely-used statute.
        records.push(record_at(base, "statute-rare"));

        let anomalies = detector.detect(&records).expect("detect ok");
        assert!(
            anomalies
                .iter()
                .any(|a| a.kind == FindingKind::RareEvent && a.dimension == "statute:statute-rare")
        );
    }

    #[test]
    fn test_detect_baseline_drift() {
        let config = StreamAnomalyConfig {
            mad_threshold: 100.0, // suppress volume spikes to isolate drift
            ..Default::default()
        };
        let detector = StreamAnomalyDetector::with_config(config);
        let base = Utc::now();
        let mut records = Vec::new();
        // Earlier regime: ~2/day for 10 days.
        for day in 10..20 {
            for _ in 0..2 {
                records.push(record_at(base - Duration::days(day), "statute-a"));
            }
        }
        // Recent regime: ~10/day for 10 days.
        for day in 0..10 {
            for _ in 0..10 {
                records.push(record_at(base - Duration::days(day), "statute-a"));
            }
        }

        let anomalies = detector.detect(&records).expect("detect ok");
        assert!(
            anomalies
                .iter()
                .any(|a| a.kind == FindingKind::BaselineDrift)
        );
    }

    #[test]
    fn test_empty_and_invalid_config() {
        let detector = StreamAnomalyDetector::new();
        assert!(detector.detect(&[]).expect("empty ok").is_empty());

        let bad = StreamAnomalyDetector::with_config(StreamAnomalyConfig {
            mad_threshold: 0.0,
            ..Default::default()
        });
        assert!(bad.detect(&[record_at(Utc::now(), "s")]).is_err());
    }

    #[test]
    fn test_stream_anomaly_to_finding() {
        let anomaly = StreamAnomaly {
            kind: FindingKind::VolumeSpike,
            dimension: "volume".to_string(),
            bucket_start: Utc::now(),
            bucket_end: Utc::now(),
            observed: 40.0,
            expected: 2.0,
            deviation_score: 9.0,
            confidence: 0.95,
            affected_records: 40,
            affected_subjects: 40,
            affected_statutes: 1,
            record_ids: vec![Uuid::new_v4()],
            explanation: "spike".to_string(),
        };
        let finding = anomaly.to_finding();
        assert_eq!(finding.kind, FindingKind::VolumeSpike);
        // High confidence escalates Medium -> High.
        assert_eq!(finding.severity, Severity::High);
        assert_eq!(finding.likelihood, Likelihood::AlmostCertain);
        assert!(finding.metrics.contains_key("observed"));
    }

    #[test]
    fn test_granularity_floor() {
        let ts = Utc::now();
        let daily = TimeGranularity::Daily.floor(ts);
        assert_eq!(daily.hour(), 0);
        assert_eq!(daily.minute(), 0);
        let weekly = TimeGranularity::Weekly.floor(ts);
        assert_eq!(weekly.weekday().num_days_from_monday(), 0);
    }
}
