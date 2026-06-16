//! Anomaly detection in diff patterns.
//!
//! Given a population of diffs (typically the revision history of one statute,
//! or a corpus spanning many statutes) this module flags the diffs whose change
//! metrics are statistical outliers. Two complementary, well-known univariate
//! detectors are provided:
//!
//! - **Z-score** (`mean` / `standard deviation`): sensitive, but the mean and
//!   standard deviation are themselves distorted by extreme values.
//! - **Modified Z-score** (`median` / `median absolute deviation`, MAD): a
//!   robust estimator that resists masking by the very outliers it is looking
//!   for, following Iglewicz & Hoaglin's recommendation.
//!
//! The metric under analysis is configurable via [`AnomalyMetric`] so the same
//! machinery detects unusually *large* diffs, unusually *severe* diffs, or
//! diffs that touch an unusual mix of targets.
//!
//! # Example
//!
//! ```
//! use legalis_core::{Statute, Effect, EffectType, Condition, ComparisonOp};
//! use legalis_diff::diff;
//! use legalis_diff::analytics::{detect_anomalies, AnomalyConfig, AnomalyMetric};
//!
//! // Build a population of small diffs plus one large outlier.
//! let base = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
//! let mut diffs = Vec::new();
//! for _ in 0..6 {
//!     let mut new = base.clone();
//!     new.title = "Tweaked".into();
//!     diffs.push(diff(&base, &new).unwrap());
//! }
//! let mut big = base.clone();
//! big.title = "Wholly rewritten".into();
//! for n in 0..8 {
//!     big = big.with_precondition(Condition::Age {
//!         operator: ComparisonOp::GreaterOrEqual,
//!         value: n,
//!     });
//! }
//! diffs.push(diff(&base, &big).unwrap());
//!
//! let report = detect_anomalies(&diffs, &AnomalyConfig::default());
//! // The last diff is the obvious outlier.
//! assert!(report.anomalies.iter().any(|a| a.index == diffs.len() - 1));
//! ```

use crate::{ChangeType, Severity, StatuteDiff};
use serde::{Deserialize, Serialize};

/// Which scalar metric of a diff to analyse for anomalies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnomalyMetric {
    /// Total number of changes in the diff.
    ChangeCount,
    /// Numeric severity rank (`None` = 0 … `Breaking` = 4).
    SeverityRank,
    /// Number of distinct change *kinds* (added / removed / modified / reordered).
    KindDiversity,
    /// Number of removals only (large removals are frequently noteworthy).
    RemovalCount,
}

impl AnomalyMetric {
    /// Extracts this metric from a single diff as an `f64`.
    pub fn extract(&self, diff: &StatuteDiff) -> f64 {
        match self {
            Self::ChangeCount => diff.changes.len() as f64,
            Self::SeverityRank => severity_rank(diff.impact.severity) as f64,
            Self::KindDiversity => {
                let mut seen = [false; 4];
                for change in &diff.changes {
                    seen[kind_index(change.change_type)] = true;
                }
                seen.iter().filter(|&&b| b).count() as f64
            }
            Self::RemovalCount => diff
                .changes
                .iter()
                .filter(|c| c.change_type == ChangeType::Removed)
                .count() as f64,
        }
    }
}

fn severity_rank(severity: Severity) -> u8 {
    match severity {
        Severity::None => 0,
        Severity::Minor => 1,
        Severity::Moderate => 2,
        Severity::Major => 3,
        Severity::Breaking => 4,
    }
}

fn kind_index(kind: ChangeType) -> usize {
    match kind {
        ChangeType::Added => 0,
        ChangeType::Removed => 1,
        ChangeType::Modified => 2,
        ChangeType::Reordered => 3,
    }
}

/// The statistical method used to score deviation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnomalyMethod {
    /// Classic z-score using the arithmetic mean and standard deviation.
    ZScore,
    /// Robust modified z-score using the median and MAD.
    ModifiedZScore,
}

/// Configuration for an anomaly-detection pass.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnomalyConfig {
    /// Metric to analyse.
    pub metric: AnomalyMetric,
    /// Scoring method.
    pub method: AnomalyMethod,
    /// Absolute score above which a point is flagged as anomalous.
    ///
    /// A threshold of `3.0` for [`AnomalyMethod::ZScore`] flags points more than
    /// three standard deviations from the mean; `3.5` is the conventional cutoff
    /// for [`AnomalyMethod::ModifiedZScore`].
    pub threshold: f64,
}

impl Default for AnomalyConfig {
    fn default() -> Self {
        Self {
            metric: AnomalyMetric::ChangeCount,
            method: AnomalyMethod::ModifiedZScore,
            threshold: 3.5,
        }
    }
}

/// One diff flagged as anomalous.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Anomaly {
    /// Index of the diff within the analysed slice.
    pub index: usize,
    /// The raw metric value for this diff.
    pub value: f64,
    /// Signed deviation score (positive = above the centre, negative = below).
    pub score: f64,
}

/// Summary of an anomaly-detection pass.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnomalyReport {
    /// Configuration that produced this report.
    pub config: AnomalyConfig,
    /// Central tendency used by the detector (mean or median).
    pub center: f64,
    /// Dispersion used by the detector (standard deviation or scaled MAD).
    pub dispersion: f64,
    /// The flagged diffs, in ascending index order.
    pub anomalies: Vec<Anomaly>,
    /// Number of diffs that were analysed.
    pub population_size: usize,
}

impl AnomalyReport {
    /// Whether any anomaly was detected.
    pub fn has_anomalies(&self) -> bool {
        !self.anomalies.is_empty()
    }

    /// Fraction of the population that was flagged, in `[0, 1]`.
    pub fn anomaly_rate(&self) -> f64 {
        if self.population_size == 0 {
            0.0
        } else {
            self.anomalies.len() as f64 / self.population_size as f64
        }
    }
}

/// Detects anomalous diffs in a population according to `config`.
///
/// Returns an empty report (no anomalies) for populations too small to estimate
/// dispersion meaningfully (fewer than three points), or when every value is
/// identical (zero dispersion).
pub fn detect_anomalies(diffs: &[StatuteDiff], config: &AnomalyConfig) -> AnomalyReport {
    let values: Vec<f64> = diffs.iter().map(|d| config.metric.extract(d)).collect();
    let n = values.len();

    if n < 3 {
        return AnomalyReport {
            config: config.clone(),
            center: values.iter().sum::<f64>() / n.max(1) as f64,
            dispersion: 0.0,
            anomalies: Vec::new(),
            population_size: n,
        };
    }

    let (center, dispersion) = match config.method {
        AnomalyMethod::ZScore => {
            let mean = values.iter().sum::<f64>() / n as f64;
            let variance = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / n as f64;
            (mean, variance.sqrt())
        }
        AnomalyMethod::ModifiedZScore => {
            let med = median(&values);
            let abs_dev: Vec<f64> = values.iter().map(|v| (v - med).abs()).collect();
            let mad = median(&abs_dev);
            if mad.abs() >= f64::EPSILON {
                // 0.6745 makes the MAD a consistent estimator of the standard
                // deviation for normally distributed data.
                (med, mad / 0.6745)
            } else {
                // MAD collapses to zero whenever more than half the values are
                // identical (common in clean diff histories). Iglewicz & Hoaglin
                // prescribe falling back to the mean absolute deviation about the
                // median, scaled by 1.253314, which still flags genuine extremes.
                let mean_ad = abs_dev.iter().sum::<f64>() / n as f64;
                (med, mean_ad * 1.253314)
            }
        }
    };

    // No spread: nothing can be an outlier.
    if dispersion.abs() < f64::EPSILON {
        return AnomalyReport {
            config: config.clone(),
            center,
            dispersion,
            anomalies: Vec::new(),
            population_size: n,
        };
    }

    let anomalies = values
        .iter()
        .enumerate()
        .filter_map(|(index, &value)| {
            let score = (value - center) / dispersion;
            if score.abs() >= config.threshold {
                Some(Anomaly {
                    index,
                    value,
                    score,
                })
            } else {
                None
            }
        })
        .collect();

    AnomalyReport {
        config: config.clone(),
        center,
        dispersion,
        anomalies,
        population_size: n,
    }
}

/// Median of a slice, computed on a sorted copy. Returns `0.0` for an empty
/// slice. NaN values sort to the end and are therefore ignored as a centre.
fn median(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let mid = sorted.len() / 2;
    if sorted.len().is_multiple_of(2) {
        (sorted[mid - 1] + sorted[mid]) / 2.0
    } else {
        sorted[mid]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};

    fn diff_with_n_conditions(n: u64) -> StatuteDiff {
        let base = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
        let mut new = base.clone();
        for i in 0..n {
            new = new.with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: i as u32,
            });
        }
        crate::diff(&base, &new).expect("diff should succeed")
    }

    #[test]
    fn test_empty_population() {
        let report = detect_anomalies(&[], &AnomalyConfig::default());
        assert!(!report.has_anomalies());
        assert_eq!(report.population_size, 0);
        assert_eq!(report.anomaly_rate(), 0.0);
    }

    #[test]
    fn test_too_small_population() {
        let diffs = vec![diff_with_n_conditions(1), diff_with_n_conditions(2)];
        let report = detect_anomalies(&diffs, &AnomalyConfig::default());
        assert!(!report.has_anomalies());
    }

    #[test]
    fn test_no_anomaly_when_uniform() {
        let diffs = vec![
            diff_with_n_conditions(1),
            diff_with_n_conditions(1),
            diff_with_n_conditions(1),
            diff_with_n_conditions(1),
        ];
        let report = detect_anomalies(&diffs, &AnomalyConfig::default());
        assert!(!report.has_anomalies());
        assert!(report.dispersion.abs() < f64::EPSILON);
    }

    #[test]
    fn test_detects_large_outlier_modified_z() {
        let mut diffs: Vec<_> = (0..7).map(|_| diff_with_n_conditions(1)).collect();
        diffs.push(diff_with_n_conditions(20));
        let report = detect_anomalies(&diffs, &AnomalyConfig::default());
        assert!(report.has_anomalies());
        assert!(report.anomalies.iter().any(|a| a.index == 7));
        // The outlier sits well above the centre.
        let outlier = report.anomalies.iter().find(|a| a.index == 7).unwrap();
        assert!(outlier.score > 0.0);
    }

    #[test]
    fn test_detects_outlier_zscore() {
        let mut diffs: Vec<_> = (0..7).map(|_| diff_with_n_conditions(2)).collect();
        diffs.push(diff_with_n_conditions(30));
        let config = AnomalyConfig {
            metric: AnomalyMetric::ChangeCount,
            method: AnomalyMethod::ZScore,
            threshold: 2.0,
        };
        let report = detect_anomalies(&diffs, &config);
        assert!(report.has_anomalies());
        assert!(report.anomalies.iter().any(|a| a.index == 7));
    }

    #[test]
    fn test_anomaly_rate() {
        let mut diffs: Vec<_> = (0..9).map(|_| diff_with_n_conditions(1)).collect();
        diffs.push(diff_with_n_conditions(50));
        let report = detect_anomalies(&diffs, &AnomalyConfig::default());
        assert!(report.anomaly_rate() > 0.0);
        assert!(report.anomaly_rate() <= 1.0);
    }

    #[test]
    fn test_metric_extract_change_count() {
        let d = diff_with_n_conditions(3);
        assert_eq!(AnomalyMetric::ChangeCount.extract(&d), 3.0);
    }

    #[test]
    fn test_metric_removal_count() {
        // Removing conditions: old has 3, new has 0.
        let base = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"))
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 1,
            })
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 2,
            });
        let new = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
        let d = crate::diff(&base, &new).expect("diff");
        assert!(AnomalyMetric::RemovalCount.extract(&d) >= 1.0);
    }

    #[test]
    fn test_median_even_odd() {
        assert_eq!(median(&[1.0, 2.0, 3.0]), 2.0);
        assert_eq!(median(&[1.0, 2.0, 3.0, 4.0]), 2.5);
        assert_eq!(median(&[]), 0.0);
    }
}
