//! Jurisdiction comparison analytics.
//!
//! [`JurisdictionComparator`] aggregates a corpus of [`LegalEvent`]s by
//! jurisdiction and produces descriptive statistics per jurisdiction (count,
//! sum, mean, median, standard deviation, min/max of the event measure), then
//! ranks jurisdictions on any chosen [`ComparisonMetric`], computes spread
//! statistics across jurisdictions (range, coefficient of variation), and
//! quantifies the dispersion of activity with a normalised Gini coefficient and
//! a Herfindahl-Hirschman concentration index.
//!
//! It can also build a *metric matrix* (jurisdiction x category) for
//! cross-tabulated comparison - e.g. average award by jurisdiction and claim
//! type - entirely offline.

use super::{LegalEvent, mean, median, percentile, population_std_dev};
use crate::Jurisdiction;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Which summary statistic to rank jurisdictions on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComparisonMetric {
    /// Number of events.
    Count,
    /// Sum of measures.
    Sum,
    /// Mean measure.
    Mean,
    /// Median measure.
    Median,
    /// Population standard deviation of measures.
    StdDev,
    /// Maximum measure.
    Max,
    /// Minimum measure.
    Min,
}

impl ComparisonMetric {
    /// Extracts this metric from a [`JurisdictionSummary`].
    pub fn extract(&self, summary: &JurisdictionSummary) -> f64 {
        match self {
            ComparisonMetric::Count => summary.count as f64,
            ComparisonMetric::Sum => summary.sum,
            ComparisonMetric::Mean => summary.mean,
            ComparisonMetric::Median => summary.median,
            ComparisonMetric::StdDev => summary.std_dev,
            ComparisonMetric::Max => summary.max,
            ComparisonMetric::Min => summary.min,
        }
    }
}

/// Per-jurisdiction descriptive statistics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JurisdictionSummary {
    /// The jurisdiction these statistics describe.
    pub jurisdiction: Jurisdiction,
    /// Number of events.
    pub count: usize,
    /// Sum of measures.
    pub sum: f64,
    /// Mean measure.
    pub mean: f64,
    /// Median measure.
    pub median: f64,
    /// 25th percentile.
    pub p25: f64,
    /// 75th percentile.
    pub p75: f64,
    /// Population standard deviation.
    pub std_dev: f64,
    /// Minimum measure.
    pub min: f64,
    /// Maximum measure.
    pub max: f64,
}

/// A single entry in a metric ranking.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RankedJurisdiction {
    /// 1-based rank position.
    pub rank: usize,
    /// The jurisdiction.
    pub jurisdiction: Jurisdiction,
    /// The metric value used for ranking.
    pub value: f64,
}

/// Cross-jurisdiction comparison output.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JurisdictionComparison {
    /// The metric the comparison ranks on.
    pub metric: ComparisonMetric,
    /// Per-jurisdiction summaries (ordered by jurisdiction description).
    pub summaries: Vec<JurisdictionSummary>,
    /// Jurisdictions ranked by the chosen metric (descending).
    pub ranking: Vec<RankedJurisdiction>,
    /// Minimum of the chosen metric across jurisdictions.
    pub metric_min: f64,
    /// Maximum of the chosen metric across jurisdictions.
    pub metric_max: f64,
    /// Mean of the chosen metric across jurisdictions.
    pub metric_mean: f64,
    /// Coefficient of variation of the chosen metric across jurisdictions.
    pub coefficient_of_variation: f64,
    /// Gini coefficient of the chosen metric (0 = equal, 1 = concentrated).
    pub gini: f64,
    /// Herfindahl-Hirschman index of activity share (sum of squared shares).
    pub hhi: f64,
}

/// Aggregates events by jurisdiction and compares them.
#[derive(Debug, Clone, Default)]
pub struct JurisdictionComparator;

impl JurisdictionComparator {
    /// Creates a new comparator.
    pub fn new() -> Self {
        Self
    }

    /// Computes the per-jurisdiction summary for a single jurisdiction's events.
    fn summarize(&self, jurisdiction: Jurisdiction, measures: &[f64]) -> JurisdictionSummary {
        JurisdictionSummary {
            jurisdiction,
            count: measures.len(),
            sum: measures.iter().sum(),
            mean: mean(measures),
            median: median(measures),
            p25: percentile(measures, 25.0),
            p75: percentile(measures, 75.0),
            std_dev: population_std_dev(measures),
            min: measures.iter().cloned().fold(f64::INFINITY, f64::min),
            max: measures.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
        }
    }

    /// Produces per-jurisdiction summaries from a corpus.
    ///
    /// Events without a jurisdiction are ignored. Summaries are returned ordered
    /// by jurisdiction description for stable output.
    pub fn summaries(&self, events: &[LegalEvent]) -> Vec<JurisdictionSummary> {
        let mut grouped: BTreeMap<String, (Jurisdiction, Vec<f64>)> = BTreeMap::new();
        for event in events {
            if let Some(jurisdiction) = &event.jurisdiction {
                let entry = grouped
                    .entry(jurisdiction.description())
                    .or_insert_with(|| (jurisdiction.clone(), Vec::new()));
                entry.1.push(event.measure());
            }
        }
        grouped
            .into_values()
            .map(|(jurisdiction, measures)| self.summarize(jurisdiction, &measures))
            .collect()
    }

    /// Compares jurisdictions on a chosen metric, producing a full comparison.
    pub fn compare(
        &self,
        events: &[LegalEvent],
        metric: ComparisonMetric,
    ) -> JurisdictionComparison {
        let summaries = self.summaries(events);
        let metric_values: Vec<f64> = summaries.iter().map(|s| metric.extract(s)).collect();

        let mut ranking: Vec<RankedJurisdiction> = summaries
            .iter()
            .map(|s| RankedJurisdiction {
                rank: 0,
                jurisdiction: s.jurisdiction.clone(),
                value: metric.extract(s),
            })
            .collect();
        ranking.sort_by(|a, b| {
            b.value
                .partial_cmp(&a.value)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| {
                    a.jurisdiction
                        .description()
                        .cmp(&b.jurisdiction.description())
                })
        });
        for (i, entry) in ranking.iter_mut().enumerate() {
            entry.rank = i + 1;
        }

        let metric_min = metric_values.iter().cloned().fold(f64::INFINITY, f64::min);
        let metric_max = metric_values
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);
        let metric_mean = mean(&metric_values);
        let metric_sd = population_std_dev(&metric_values);
        let coefficient_of_variation = if metric_mean.abs() > f64::EPSILON {
            metric_sd / metric_mean.abs()
        } else {
            0.0
        };

        let gini = gini_coefficient(&metric_values);

        // HHI over activity counts (share of total events per jurisdiction).
        let total_count: f64 = summaries.iter().map(|s| s.count as f64).sum();
        let hhi = if total_count > 0.0 {
            summaries
                .iter()
                .map(|s| {
                    let share = s.count as f64 / total_count;
                    share * share
                })
                .sum()
        } else {
            0.0
        };

        JurisdictionComparison {
            metric,
            summaries,
            ranking,
            metric_min: if metric_min.is_finite() {
                metric_min
            } else {
                0.0
            },
            metric_max: if metric_max.is_finite() {
                metric_max
            } else {
                0.0
            },
            metric_mean,
            coefficient_of_variation,
            gini,
            hhi,
        }
    }

    /// Builds a jurisdiction x category matrix of a chosen metric.
    ///
    /// The outer map is keyed by jurisdiction description, the inner by event
    /// category, with the value being the requested metric over the measures of
    /// the events in that cell. Useful for cross-tabulated comparison (e.g.
    /// average award by jurisdiction and claim type).
    pub fn metric_matrix(
        &self,
        events: &[LegalEvent],
        metric: ComparisonMetric,
    ) -> BTreeMap<String, BTreeMap<String, f64>> {
        let mut cells: BTreeMap<String, BTreeMap<String, Vec<f64>>> = BTreeMap::new();
        for event in events {
            if let Some(jurisdiction) = &event.jurisdiction {
                cells
                    .entry(jurisdiction.description())
                    .or_default()
                    .entry(event.category.clone())
                    .or_default()
                    .push(event.measure());
            }
        }
        cells
            .into_iter()
            .map(|(jurisdiction, categories)| {
                let row = categories
                    .into_iter()
                    .map(|(category, measures)| {
                        let summary =
                            self.summarize(Jurisdiction::Custom(jurisdiction.clone()), &measures);
                        (category, metric.extract(&summary))
                    })
                    .collect();
                (jurisdiction, row)
            })
            .collect()
    }
}

/// Computes the Gini coefficient of a set of non-negative values.
///
/// Returns `0.0` for an empty input, a single value, or all-zero / negative
/// inputs (where the measure is undefined). The result lies in `[0, 1]`.
fn gini_coefficient(values: &[f64]) -> f64 {
    let positive: Vec<f64> = values.iter().cloned().filter(|v| *v >= 0.0).collect();
    let n = positive.len();
    if n < 2 {
        return 0.0;
    }
    let total: f64 = positive.iter().sum();
    if total <= 0.0 {
        return 0.0;
    }
    let mut sorted = positive;
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    // Gini = (2 * sum(i * x_i) / (n * sum) ) - (n + 1) / n, with i 1-based.
    let weighted: f64 = sorted
        .iter()
        .enumerate()
        .map(|(i, &x)| (i as f64 + 1.0) * x)
        .sum();
    let n_f = n as f64;
    let gini = (2.0 * weighted) / (n_f * total) - (n_f + 1.0) / n_f;
    gini.clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    fn event(id: &str, jurisdiction: Jurisdiction, category: &str, value: f64) -> LegalEvent {
        let ts = Utc
            .with_ymd_and_hms(2024, 1, 1, 0, 0, 0)
            .single()
            .expect("valid");
        LegalEvent::new(id, ts, category)
            .with_value(value)
            .with_jurisdiction(jurisdiction)
    }

    fn sample() -> Vec<LegalEvent> {
        vec![
            event("a", Jurisdiction::UsFederal, "tort", 100.0),
            event("b", Jurisdiction::UsFederal, "tort", 300.0),
            event("c", Jurisdiction::UsFederal, "contract", 200.0),
            event("d", Jurisdiction::Uk, "tort", 50.0),
            event("e", Jurisdiction::Uk, "contract", 150.0),
        ]
    }

    #[test]
    fn test_summaries() {
        let comparator = JurisdictionComparator::new();
        let summaries = comparator.summaries(&sample());
        assert_eq!(summaries.len(), 2);
        let us = summaries
            .iter()
            .find(|s| s.jurisdiction == Jurisdiction::UsFederal)
            .expect("us summary");
        assert_eq!(us.count, 3);
        assert!((us.sum - 600.0).abs() < 1e-9);
        assert!((us.mean - 200.0).abs() < 1e-9);
        assert!((us.median - 200.0).abs() < 1e-9);
        assert!((us.max - 300.0).abs() < 1e-9);
        assert!((us.min - 100.0).abs() < 1e-9);
    }

    #[test]
    fn test_compare_ranking() {
        let comparator = JurisdictionComparator::new();
        let comparison = comparator.compare(&sample(), ComparisonMetric::Sum);
        assert_eq!(comparison.ranking.len(), 2);
        // US sum 600 > UK sum 200.
        assert_eq!(comparison.ranking[0].rank, 1);
        assert_eq!(comparison.ranking[0].jurisdiction, Jurisdiction::UsFederal);
        assert!((comparison.ranking[0].value - 600.0).abs() < 1e-9);
        assert_eq!(comparison.ranking[1].jurisdiction, Jurisdiction::Uk);
        assert!((comparison.metric_max - 600.0).abs() < 1e-9);
        assert!((comparison.metric_min - 200.0).abs() < 1e-9);
        assert!(comparison.coefficient_of_variation > 0.0);
        // 3 US events vs 2 UK => shares 0.6, 0.4 => HHI = 0.52.
        assert!((comparison.hhi - 0.52).abs() < 1e-9);
    }

    #[test]
    fn test_compare_by_count() {
        let comparator = JurisdictionComparator::new();
        let comparison = comparator.compare(&sample(), ComparisonMetric::Count);
        assert_eq!(comparison.ranking[0].jurisdiction, Jurisdiction::UsFederal);
        assert!((comparison.ranking[0].value - 3.0).abs() < 1e-9);
    }

    #[test]
    fn test_metric_matrix() {
        let comparator = JurisdictionComparator::new();
        let matrix = comparator.metric_matrix(&sample(), ComparisonMetric::Mean);
        let us_row = matrix
            .get(&Jurisdiction::UsFederal.description())
            .expect("us row");
        // US tort mean = (100 + 300)/2 = 200.
        assert!((us_row.get("tort").copied().unwrap_or(0.0) - 200.0).abs() < 1e-9);
        assert!((us_row.get("contract").copied().unwrap_or(0.0) - 200.0).abs() < 1e-9);
        let uk_row = matrix.get(&Jurisdiction::Uk.description()).expect("uk row");
        assert!((uk_row.get("tort").copied().unwrap_or(0.0) - 50.0).abs() < 1e-9);
    }

    #[test]
    fn test_gini_coefficient() {
        // Perfectly equal => 0.
        assert!(gini_coefficient(&[5.0, 5.0, 5.0, 5.0]).abs() < 1e-9);
        // Maximally concentrated => approaches (n-1)/n.
        let g = gini_coefficient(&[0.0, 0.0, 0.0, 100.0]);
        assert!(g > 0.7);
        // Guards.
        assert_eq!(gini_coefficient(&[]), 0.0);
        assert_eq!(gini_coefficient(&[3.0]), 0.0);
        assert_eq!(gini_coefficient(&[0.0, 0.0]), 0.0);
    }

    #[test]
    fn test_events_without_jurisdiction_ignored() {
        let comparator = JurisdictionComparator::new();
        let ts = Utc
            .with_ymd_and_hms(2024, 1, 1, 0, 0, 0)
            .single()
            .expect("valid");
        let events = vec![
            LegalEvent::new("x", ts, "tort").with_value(10.0),
            event("y", Jurisdiction::Uk, "tort", 20.0),
        ];
        let summaries = comparator.summaries(&events);
        assert_eq!(summaries.len(), 1);
        assert_eq!(summaries[0].jurisdiction, Jurisdiction::Uk);
    }
}
