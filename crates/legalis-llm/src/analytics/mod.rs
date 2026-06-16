//! Advanced Legal Analytics
//!
//! A self-contained, pure-Rust analytics toolkit that operates entirely over
//! *caller-supplied* data. None of the features in this module require a live
//! LLM call: legal trends are detected with classical time-series statistics
//! (ordinary-least-squares regression, the non-parametric Mann-Kendall trend
//! test, moving averages, change-point detection and seasonal decomposition),
//! jurisdictions are compared with descriptive statistics over a metric matrix,
//! repeatable behaviour (judge decisions, settlements, or any other categorical
//! outcome) is summarised with a generic frequency/association pattern engine,
//! risk is captured in a structured likelihood x impact heat-map (with CSV /
//! Markdown export) and reports are produced from a composable builder that
//! renders to a structured tree, Markdown or plain text.
//!
//! ## Sub-modules
//!
//! * [`trends`] - time-series trend detection over a corpus of [`LegalEvent`]s.
//! * [`jurisdiction`] - cross-jurisdiction metric comparison analytics.
//! * [`patterns`] - a generic categorical pattern-analysis engine (judge
//!   decisions, settlements, dispositions, ...).
//! * [`heatmap`] - a structured risk matrix and exports.
//! * [`report`] - a composable report builder.

mod heatmap;
mod jurisdiction;
mod patterns;
mod report;
mod trends;

pub use heatmap::*;
pub use jurisdiction::*;
pub use patterns::*;
pub use report::*;
pub use trends::*;

use crate::Jurisdiction;
use chrono::{DateTime, Datelike, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Shared data model
// ============================================================================

/// A single dated legal event in an analytics corpus.
///
/// An event is the atomic unit consumed by the analytics engines: it pairs a
/// timestamp with an optional numeric measurement (a damages award, a contract
/// value, a sentence length, a processing time, ...), a category label (a court,
/// a judge, a claim type, an outcome, ...) and a free-form attribute bag for
/// dimensional slicing. Trend analysis aggregates the numeric `value` (or counts
/// events when no value is supplied); pattern analysis works over the
/// categorical `category` and `attributes`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LegalEvent {
    /// Stable identifier for the event.
    pub id: String,
    /// When the event occurred.
    pub timestamp: DateTime<Utc>,
    /// Optional numeric measurement attached to the event.
    pub value: Option<f64>,
    /// Primary categorical label (e.g. outcome, court, claim type).
    pub category: String,
    /// Jurisdiction the event belongs to, if known.
    pub jurisdiction: Option<Jurisdiction>,
    /// Arbitrary key/value attributes for dimensional analysis.
    pub attributes: HashMap<String, String>,
}

impl LegalEvent {
    /// Creates a new event with the mandatory fields.
    pub fn new(
        id: impl Into<String>,
        timestamp: DateTime<Utc>,
        category: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            timestamp,
            value: None,
            category: category.into(),
            jurisdiction: None,
            attributes: HashMap::new(),
        }
    }

    /// Sets the numeric measurement.
    pub fn with_value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    /// Sets the jurisdiction.
    pub fn with_jurisdiction(mut self, jurisdiction: Jurisdiction) -> Self {
        self.jurisdiction = Some(jurisdiction);
        self
    }

    /// Adds an attribute.
    pub fn with_attribute(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }

    /// Returns the value used for numeric aggregation: the explicit `value` if
    /// present, otherwise `1.0` so the event contributes to a count.
    pub fn measure(&self) -> f64 {
        self.value.unwrap_or(1.0)
    }
}

/// Time-bucket granularity for analytics aggregation.
///
/// Distinct from `monitoring::TrendGranularity` (which only supports
/// hour/day buckets for operational metrics): legal analytics typically spans
/// years, so coarser buckets are provided.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnalyticsGranularity {
    /// Calendar day buckets.
    Daily,
    /// ISO-week buckets (Monday start).
    Weekly,
    /// Calendar month buckets.
    Monthly,
    /// Calendar quarter buckets.
    Quarterly,
    /// Calendar year buckets.
    Yearly,
}

impl AnalyticsGranularity {
    /// Returns a stable, sortable bucket key for a timestamp.
    ///
    /// Keys are ISO-like strings chosen so that lexical ordering matches
    /// chronological ordering within a single granularity (e.g. `2024-Q1` <
    /// `2024-Q2`, `2024-03` < `2024-04`).
    pub fn bucket_key(&self, timestamp: DateTime<Utc>) -> String {
        let date = timestamp.date_naive();
        match self {
            AnalyticsGranularity::Daily => date.format("%Y-%m-%d").to_string(),
            AnalyticsGranularity::Weekly => {
                let iso = date.iso_week();
                format!("{:04}-W{:02}", iso.year(), iso.week())
            }
            AnalyticsGranularity::Monthly => date.format("%Y-%m").to_string(),
            AnalyticsGranularity::Quarterly => {
                let quarter = (date.month0() / 3) + 1;
                format!("{:04}-Q{}", date.year(), quarter)
            }
            AnalyticsGranularity::Yearly => format!("{:04}", date.year()),
        }
    }

    /// Returns a canonical representative date for a bucket containing the given
    /// timestamp (the first day of the bucket).
    pub fn bucket_start(&self, timestamp: DateTime<Utc>) -> NaiveDate {
        let date = timestamp.date_naive();
        match self {
            AnalyticsGranularity::Daily => date,
            AnalyticsGranularity::Weekly => {
                let weekday = date.weekday().num_days_from_monday() as i64;
                date - chrono::Duration::days(weekday)
            }
            AnalyticsGranularity::Monthly => {
                NaiveDate::from_ymd_opt(date.year(), date.month(), 1).unwrap_or(date)
            }
            AnalyticsGranularity::Quarterly => {
                let quarter_start_month = (date.month0() / 3) * 3 + 1;
                NaiveDate::from_ymd_opt(date.year(), quarter_start_month, 1).unwrap_or(date)
            }
            AnalyticsGranularity::Yearly => {
                NaiveDate::from_ymd_opt(date.year(), 1, 1).unwrap_or(date)
            }
        }
    }
}

// ============================================================================
// Shared descriptive-statistics helpers
//
// Re-implemented locally (rather than reaching into the `monitoring` module's
// private helpers) so the analytics suite is self-contained and the two
// suites can evolve independently.
// ============================================================================

/// Returns the arithmetic mean of a slice, or `0.0` when empty.
pub(crate) fn mean(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    values.iter().sum::<f64>() / values.len() as f64
}

/// Returns the population variance of a slice, or `0.0` when empty.
pub(crate) fn population_variance(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let avg = mean(values);
    values
        .iter()
        .map(|value| (value - avg).powi(2))
        .sum::<f64>()
        / values.len() as f64
}

/// Returns the population standard deviation of a slice.
pub(crate) fn population_std_dev(values: &[f64]) -> f64 {
    population_variance(values).sqrt()
}

/// Returns the sample standard deviation (Bessel-corrected) of a slice.
///
/// Returns `0.0` for fewer than two values where the correction is undefined.
pub(crate) fn sample_std_dev(values: &[f64]) -> f64 {
    if values.len() < 2 {
        return 0.0;
    }
    let avg = mean(values);
    let var = values
        .iter()
        .map(|value| (value - avg).powi(2))
        .sum::<f64>()
        / (values.len() as f64 - 1.0);
    var.sqrt()
}

/// Returns the median of a slice (sorts a copy internally), or `0.0` when empty.
pub(crate) fn median(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let len = sorted.len();
    if len % 2 == 1 {
        sorted[len / 2]
    } else {
        (sorted[len / 2 - 1] + sorted[len / 2]) / 2.0
    }
}

/// Returns the linearly-interpolated `p`-th percentile of a slice.
///
/// `p` is a percentage in `[0, 100]`; uses the "linear interpolation between
/// closest ranks" method (NumPy's default).
pub(crate) fn percentile(values: &[f64], p: f64) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    if sorted.len() == 1 {
        return sorted[0];
    }
    let clamped = p.clamp(0.0, 100.0);
    let rank = (clamped / 100.0) * (sorted.len() as f64 - 1.0);
    let lower = rank.floor() as usize;
    let upper = rank.ceil() as usize;
    if lower == upper {
        return sorted[lower];
    }
    let frac = rank - lower as f64;
    sorted[lower] + (sorted[upper] - sorted[lower]) * frac
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn ts(y: i32, m: u32, d: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(y, m, d, 12, 0, 0)
            .single()
            .expect("valid timestamp")
    }

    #[test]
    fn test_event_builder_and_measure() {
        let event = LegalEvent::new("e1", ts(2024, 3, 15), "settlement")
            .with_value(125_000.0)
            .with_jurisdiction(Jurisdiction::UsFederal)
            .with_attribute("judge", "Smith");
        assert_eq!(event.measure(), 125_000.0);
        assert_eq!(event.attributes.get("judge"), Some(&"Smith".to_string()));
        assert_eq!(event.jurisdiction, Some(Jurisdiction::UsFederal));

        // No value => measure counts as 1.
        let count_only = LegalEvent::new("e2", ts(2024, 3, 16), "filing");
        assert_eq!(count_only.measure(), 1.0);
    }

    #[test]
    fn test_bucket_keys_sort_chronologically() {
        let g = AnalyticsGranularity::Monthly;
        assert!(g.bucket_key(ts(2024, 3, 1)) < g.bucket_key(ts(2024, 4, 1)));
        assert_eq!(g.bucket_key(ts(2024, 3, 15)), "2024-03");

        let q = AnalyticsGranularity::Quarterly;
        assert_eq!(q.bucket_key(ts(2024, 2, 1)), "2024-Q1");
        assert_eq!(q.bucket_key(ts(2024, 5, 1)), "2024-Q2");
        assert_eq!(q.bucket_key(ts(2024, 11, 1)), "2024-Q4");
        assert!(q.bucket_key(ts(2024, 2, 1)) < q.bucket_key(ts(2024, 5, 1)));

        let y = AnalyticsGranularity::Yearly;
        assert_eq!(y.bucket_key(ts(2023, 6, 1)), "2023");
        assert!(y.bucket_key(ts(2023, 6, 1)) < y.bucket_key(ts(2024, 1, 1)));
    }

    #[test]
    fn test_bucket_start() {
        let m = AnalyticsGranularity::Monthly;
        assert_eq!(
            m.bucket_start(ts(2024, 3, 15)),
            NaiveDate::from_ymd_opt(2024, 3, 1).unwrap()
        );
        let q = AnalyticsGranularity::Quarterly;
        assert_eq!(
            q.bucket_start(ts(2024, 5, 20)),
            NaiveDate::from_ymd_opt(2024, 4, 1).unwrap()
        );
        let w = AnalyticsGranularity::Weekly;
        // 2024-03-15 is a Friday; Monday of that ISO week is 2024-03-11.
        assert_eq!(
            w.bucket_start(ts(2024, 3, 15)),
            NaiveDate::from_ymd_opt(2024, 3, 11).unwrap()
        );
    }

    #[test]
    fn test_descriptive_stats() {
        let values = [1.0, 2.0, 3.0, 4.0, 5.0];
        assert!((mean(&values) - 3.0).abs() < 1e-12);
        assert!((median(&values) - 3.0).abs() < 1e-12);
        assert!((percentile(&values, 50.0) - 3.0).abs() < 1e-12);
        assert!((percentile(&values, 0.0) - 1.0).abs() < 1e-12);
        assert!((percentile(&values, 100.0) - 5.0).abs() < 1e-12);
        // var of 1..5 (population) = 2.0; sample = 2.5.
        assert!((population_variance(&values) - 2.0).abs() < 1e-12);
        assert!((population_std_dev(&values) - 2.0_f64.sqrt()).abs() < 1e-12);
        assert!((sample_std_dev(&values) - 2.5_f64.sqrt()).abs() < 1e-12);
        // empty / degenerate guards.
        assert_eq!(mean(&[]), 0.0);
        assert_eq!(median(&[]), 0.0);
        assert_eq!(sample_std_dev(&[1.0]), 0.0);
    }
}
