//! Legal trend analysis: time-series trend detection over legal events.
//!
//! [`TrendAnalyzer`] aggregates a corpus of [`LegalEvent`]s into a regular time
//! series (count- or value-based, bucketed by [`AnalyticsGranularity`]) and runs
//! a battery of classical trend statistics:
//!
//! * **Ordinary least squares** linear regression on the bucket series, with the
//!   coefficient of determination (`R^2`).
//! * The non-parametric **Mann-Kendall** trend test (with tie-corrected variance
//!   and a normal-approximation two-sided p-value) - robust to non-normal data.
//! * **Sen's slope** - the median of pairwise slopes, a robust trend magnitude.
//! * A trailing **moving average** smoother.
//! * **Change-point detection** via a normalised CUSUM scan.
//! * Per-period **seasonal averages** (e.g. month-of-year effects).
//!
//! Everything is deterministic and offline.

use super::{
    AnalyticsGranularity, LegalEvent, mean, percentile, population_std_dev, sample_std_dev,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// How a bucket value is computed from the events that fall into it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Aggregation {
    /// Number of events in the bucket.
    Count,
    /// Sum of event measures.
    Sum,
    /// Mean of event measures.
    Mean,
    /// Median of event measures.
    Median,
    /// Maximum event measure.
    Max,
    /// Minimum event measure.
    Min,
}

impl Aggregation {
    fn apply(&self, measures: &[f64]) -> f64 {
        if measures.is_empty() {
            return 0.0;
        }
        match self {
            Aggregation::Count => measures.len() as f64,
            Aggregation::Sum => measures.iter().sum(),
            Aggregation::Mean => mean(measures),
            Aggregation::Median => percentile(measures, 50.0),
            Aggregation::Max => measures.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
            Aggregation::Min => measures.iter().cloned().fold(f64::INFINITY, f64::min),
        }
    }
}

/// The qualitative direction of a detected trend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrendDirection {
    /// A statistically detectable upward trend.
    Increasing,
    /// A statistically detectable downward trend.
    Decreasing,
    /// No statistically detectable monotone trend.
    Stable,
}

/// A single point in an aggregated time series.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SeriesPoint {
    /// Stable, chronologically-sortable bucket key (e.g. `2024-Q1`).
    pub bucket: String,
    /// Representative calendar date for the bucket (its first day).
    pub date: NaiveDate,
    /// Aggregated value for the bucket.
    pub value: f64,
    /// Number of events that contributed to the bucket.
    pub count: usize,
}

/// The result of an ordinary-least-squares linear fit `y = slope * x + intercept`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LinearFit {
    /// Slope per bucket index.
    pub slope: f64,
    /// Intercept at bucket index 0.
    pub intercept: f64,
    /// Coefficient of determination in `[0, 1]`.
    pub r_squared: f64,
}

impl LinearFit {
    /// Predicts the fitted value at an arbitrary (possibly future) bucket index.
    pub fn predict(&self, index: f64) -> f64 {
        self.slope * index + self.intercept
    }
}

/// The result of the Mann-Kendall non-parametric trend test.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MannKendall {
    /// The Mann-Kendall S statistic (sum of pairwise signs).
    pub s: f64,
    /// Tie-corrected variance of S.
    pub variance: f64,
    /// The standardised Z statistic (with continuity correction).
    pub z: f64,
    /// Two-sided p-value from the normal approximation.
    pub p_value: f64,
    /// Kendall's tau rank correlation in `[-1, 1]`.
    pub tau: f64,
}

/// A detected change point in the series.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChangePoint {
    /// Index of the bucket at which the regime appears to change.
    pub index: usize,
    /// Bucket key at the change point.
    pub bucket: String,
    /// Mean of the segment before the change point.
    pub mean_before: f64,
    /// Mean of the segment from the change point onward.
    pub mean_after: f64,
    /// Magnitude of the normalised CUSUM excursion driving the detection.
    pub score: f64,
}

/// A comprehensive trend report over an aggregated series.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrendReport {
    /// The aggregated time series the report was computed from.
    pub series: Vec<SeriesPoint>,
    /// OLS linear fit.
    pub linear_fit: LinearFit,
    /// Mann-Kendall trend test.
    pub mann_kendall: MannKendall,
    /// Robust Sen's slope (median of pairwise slopes).
    pub sen_slope: f64,
    /// Overall qualitative direction (from Mann-Kendall significance).
    pub direction: TrendDirection,
    /// Percentage change from the first to the last bucket value.
    pub percent_change: f64,
    /// Mean of all bucket values.
    pub mean_value: f64,
    /// Standard deviation of bucket values.
    pub std_dev: f64,
}

/// Aggregates legal events into a time series and detects trends.
#[derive(Debug, Clone)]
pub struct TrendAnalyzer {
    granularity: AnalyticsGranularity,
    aggregation: Aggregation,
    significance_level: f64,
    fill_gaps: bool,
}

impl Default for TrendAnalyzer {
    fn default() -> Self {
        Self::new(AnalyticsGranularity::Monthly, Aggregation::Count)
    }
}

impl TrendAnalyzer {
    /// Creates a new analyzer with the given bucketing and aggregation.
    pub fn new(granularity: AnalyticsGranularity, aggregation: Aggregation) -> Self {
        Self {
            granularity,
            aggregation,
            significance_level: 0.05,
            fill_gaps: true,
        }
    }

    /// Sets the two-sided significance level used to label trend direction.
    pub fn with_significance_level(mut self, alpha: f64) -> Self {
        self.significance_level = alpha.clamp(1e-6, 0.5);
        self
    }

    /// Controls whether empty intermediate buckets are inserted as zero-valued
    /// points so the series is contiguous (default `true`).
    pub fn with_gap_filling(mut self, fill: bool) -> Self {
        self.fill_gaps = fill;
        self
    }

    /// Aggregates events into a chronologically-ordered time series.
    pub fn build_series(&self, events: &[LegalEvent]) -> Vec<SeriesPoint> {
        let mut buckets: BTreeMap<String, (NaiveDate, Vec<f64>)> = BTreeMap::new();
        for event in events {
            let key = self.granularity.bucket_key(event.timestamp);
            let date = self.granularity.bucket_start(event.timestamp);
            let entry = buckets.entry(key).or_insert_with(|| (date, Vec::new()));
            entry.1.push(event.measure());
        }

        let mut points: Vec<SeriesPoint> = buckets
            .into_iter()
            .map(|(bucket, (date, measures))| SeriesPoint {
                bucket,
                date,
                value: self.aggregation.apply(&measures),
                count: measures.len(),
            })
            .collect();

        if self.fill_gaps {
            points = self.fill_series_gaps(points);
        }
        points
    }

    /// Inserts zero-valued points for any missing buckets between the first and
    /// last observed bucket so the series is contiguous.
    fn fill_series_gaps(&self, points: Vec<SeriesPoint>) -> Vec<SeriesPoint> {
        if points.len() < 2 {
            return points;
        }
        let mut filled = Vec::with_capacity(points.len());
        let mut existing: BTreeMap<String, SeriesPoint> =
            points.into_iter().map(|p| (p.bucket.clone(), p)).collect();

        // Reconstruct the ordered list of buckets that *should* exist by walking
        // the calendar from the first to the last observed date.
        let first = existing
            .values()
            .map(|p| p.date)
            .min()
            .unwrap_or_else(|| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap_or_default());
        let last = existing.values().map(|p| p.date).max().unwrap_or(first);

        let mut cursor = first;
        let mut guard = 0usize;
        // Guard bounds the loop defensively against any calendar edge case.
        while cursor <= last && guard < 100_000 {
            guard += 1;
            let key = self.granularity.bucket_key(date_to_utc_noon(cursor));
            let start = self.granularity.bucket_start(date_to_utc_noon(cursor));
            if let Some(point) = existing.remove(&key) {
                filled.push(point);
            } else {
                filled.push(SeriesPoint {
                    bucket: key,
                    date: start,
                    value: 0.0,
                    count: 0,
                });
            }
            cursor = advance_one_bucket(start, self.granularity);
        }
        // Append any stragglers that the calendar walk missed (defensive).
        let mut leftovers: Vec<SeriesPoint> = existing.into_values().collect();
        leftovers.sort_by(|a, b| a.bucket.cmp(&b.bucket));
        filled.extend(leftovers);
        filled
    }

    /// Computes an ordinary-least-squares linear fit of value against bucket
    /// index. Returns a flat zero-slope fit when fewer than two points exist.
    pub fn linear_fit(&self, series: &[SeriesPoint]) -> LinearFit {
        let n = series.len();
        if n < 2 {
            let intercept = series.first().map(|p| p.value).unwrap_or(0.0);
            return LinearFit {
                slope: 0.0,
                intercept,
                r_squared: 0.0,
            };
        }
        let xs: Vec<f64> = (0..n).map(|i| i as f64).collect();
        let ys: Vec<f64> = series.iter().map(|p| p.value).collect();
        let mean_x = mean(&xs);
        let mean_y = mean(&ys);

        let mut sxx = 0.0;
        let mut sxy = 0.0;
        let mut syy = 0.0;
        for i in 0..n {
            let dx = xs[i] - mean_x;
            let dy = ys[i] - mean_y;
            sxx += dx * dx;
            sxy += dx * dy;
            syy += dy * dy;
        }
        let slope = if sxx > 0.0 { sxy / sxx } else { 0.0 };
        let intercept = mean_y - slope * mean_x;
        let r_squared = if sxx > 0.0 && syy > 0.0 {
            (sxy * sxy) / (sxx * syy)
        } else {
            0.0
        };
        LinearFit {
            slope,
            intercept,
            r_squared: r_squared.clamp(0.0, 1.0),
        }
    }

    /// Runs the Mann-Kendall trend test with tie correction on bucket values.
    pub fn mann_kendall(&self, series: &[SeriesPoint]) -> MannKendall {
        let values: Vec<f64> = series.iter().map(|p| p.value).collect();
        let n = values.len();
        if n < 3 {
            return MannKendall {
                s: 0.0,
                variance: 0.0,
                z: 0.0,
                p_value: 1.0,
                tau: 0.0,
            };
        }

        let mut s = 0i64;
        for i in 0..n {
            for j in (i + 1)..n {
                let diff = values[j] - values[i];
                if diff > 0.0 {
                    s += 1;
                } else if diff < 0.0 {
                    s -= 1;
                }
            }
        }

        // Tie-corrected variance.
        let mut tie_groups: BTreeMap<u64, usize> = BTreeMap::new();
        for &v in &values {
            *tie_groups.entry(v.to_bits()).or_insert(0) += 1;
        }
        let n_f = n as f64;
        let mut tie_term = 0.0;
        for &count in tie_groups.values() {
            if count > 1 {
                let t = count as f64;
                tie_term += t * (t - 1.0) * (2.0 * t + 5.0);
            }
        }
        let variance = (n_f * (n_f - 1.0) * (2.0 * n_f + 5.0) - tie_term) / 18.0;

        let s_f = s as f64;
        let z = if variance > 0.0 {
            if s > 0 {
                (s_f - 1.0) / variance.sqrt()
            } else if s < 0 {
                (s_f + 1.0) / variance.sqrt()
            } else {
                0.0
            }
        } else {
            0.0
        };

        let p_value = two_sided_normal_p_value(z);
        // Kendall's tau with tie adjustment in the denominator.
        let n0 = n_f * (n_f - 1.0) / 2.0;
        let n1: f64 = tie_groups
            .values()
            .map(|&c| {
                let c = c as f64;
                c * (c - 1.0) / 2.0
            })
            .sum();
        let denom = ((n0 - n1) * n0).sqrt();
        let tau = if denom > 0.0 {
            (s_f / denom).clamp(-1.0, 1.0)
        } else {
            0.0
        };

        MannKendall {
            s: s_f,
            variance,
            z,
            p_value,
            tau,
        }
    }

    /// Computes Sen's slope: the median of all pairwise slopes
    /// `(y_j - y_i) / (j - i)`. A robust, outlier-resistant trend magnitude.
    pub fn sen_slope(&self, series: &[SeriesPoint]) -> f64 {
        let values: Vec<f64> = series.iter().map(|p| p.value).collect();
        let n = values.len();
        if n < 2 {
            return 0.0;
        }
        let mut slopes = Vec::with_capacity(n * (n - 1) / 2);
        for i in 0..n {
            for j in (i + 1)..n {
                slopes.push((values[j] - values[i]) / (j as f64 - i as f64));
            }
        }
        percentile(&slopes, 50.0)
    }

    /// Computes a trailing simple moving average with the given window.
    ///
    /// Each output point averages the up-to-`window` preceding values
    /// (inclusive), so the result has the same length as the input.
    pub fn moving_average(&self, series: &[SeriesPoint], window: usize) -> Vec<f64> {
        let window = window.max(1);
        let values: Vec<f64> = series.iter().map(|p| p.value).collect();
        let mut out = Vec::with_capacity(values.len());
        for i in 0..values.len() {
            let start = i.saturating_sub(window - 1);
            let slice = &values[start..=i];
            out.push(mean(slice));
        }
        out
    }

    /// Detects regime change points via a normalised CUSUM scan.
    ///
    /// Values are standardised (z-scored against the global mean / population
    /// standard deviation), a cumulative sum is accumulated, and the indices of
    /// the extreme excursions whose magnitude exceeds `threshold` standard-error
    /// units are returned as candidate change points, ordered by score.
    pub fn detect_change_points(&self, series: &[SeriesPoint], threshold: f64) -> Vec<ChangePoint> {
        let values: Vec<f64> = series.iter().map(|p| p.value).collect();
        let n = values.len();
        if n < 4 {
            return Vec::new();
        }
        let global_mean = mean(&values);
        let sd = population_std_dev(&values);
        if sd <= 0.0 {
            return Vec::new();
        }

        // Cumulative sum of standardised deviations.
        let mut cusum = vec![0.0; n];
        let mut running = 0.0;
        for i in 0..n {
            running += (values[i] - global_mean) / sd;
            cusum[i] = running;
        }

        // Candidate split = argmax |cusum|. Recurse into segments above
        // threshold to find multiple change points.
        let mut results = Vec::new();
        let mut stack = vec![(0usize, n)];
        while let Some((lo, hi)) = stack.pop() {
            if hi - lo < 4 {
                continue;
            }
            let mut best_idx = lo;
            let mut best_mag = 0.0;
            // Local CUSUM within the segment.
            let seg = &values[lo..hi];
            let seg_mean = mean(seg);
            let mut local = 0.0;
            for (offset, &v) in seg.iter().enumerate() {
                local += (v - seg_mean) / sd;
                if local.abs() > best_mag {
                    best_mag = local.abs();
                    best_idx = lo + offset;
                }
            }
            // Normalise by sqrt(segment length) to get a standard-error scale.
            let seg_len = (hi - lo) as f64;
            let score = best_mag / seg_len.sqrt();
            if score >= threshold && best_idx > lo && best_idx < hi - 1 {
                let split = best_idx + 1;
                let mean_before = mean(&values[lo..split]);
                let mean_after = mean(&values[split..hi]);
                results.push(ChangePoint {
                    index: split,
                    bucket: series[split].bucket.clone(),
                    mean_before,
                    mean_after,
                    score,
                });
                stack.push((lo, split));
                stack.push((split, hi));
            }
        }
        results.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.index.cmp(&b.index))
        });
        results
    }

    /// Computes per-period seasonal averages keyed by a calendar period label.
    ///
    /// The period is derived from each point's date according to the analyzer's
    /// granularity: month-of-year for daily/weekly/monthly series, quarter for
    /// quarterly series, and (degenerately) a single bucket for yearly series.
    pub fn seasonal_averages(&self, series: &[SeriesPoint]) -> BTreeMap<String, f64> {
        use chrono::Datelike;
        let mut groups: BTreeMap<String, Vec<f64>> = BTreeMap::new();
        for point in series {
            let label = match self.granularity {
                AnalyticsGranularity::Quarterly => {
                    format!("Q{}", (point.date.month0() / 3) + 1)
                }
                AnalyticsGranularity::Yearly => "year".to_string(),
                _ => format!("M{:02}", point.date.month()),
            };
            groups.entry(label).or_default().push(point.value);
        }
        groups
            .into_iter()
            .map(|(label, values)| (label, mean(&values)))
            .collect()
    }

    /// Runs the full trend battery and assembles a [`TrendReport`].
    pub fn analyze(&self, events: &[LegalEvent]) -> TrendReport {
        let series = self.build_series(events);
        self.analyze_series(series)
    }

    /// Runs the full trend battery on a pre-built series.
    pub fn analyze_series(&self, series: Vec<SeriesPoint>) -> TrendReport {
        let linear_fit = self.linear_fit(&series);
        let mann_kendall = self.mann_kendall(&series);
        let sen_slope = self.sen_slope(&series);
        let values: Vec<f64> = series.iter().map(|p| p.value).collect();
        let mean_value = mean(&values);
        let std_dev = sample_std_dev(&values);

        let direction = if mann_kendall.p_value <= self.significance_level && mann_kendall.s != 0.0
        {
            if mann_kendall.s > 0.0 {
                TrendDirection::Increasing
            } else {
                TrendDirection::Decreasing
            }
        } else {
            TrendDirection::Stable
        };

        let percent_change = match (series.first(), series.last()) {
            (Some(first), Some(last)) if first.value.abs() > f64::EPSILON => {
                (last.value - first.value) / first.value.abs() * 100.0
            }
            _ => 0.0,
        };

        TrendReport {
            series,
            linear_fit,
            mann_kendall,
            sen_slope,
            direction,
            percent_change,
            mean_value,
            std_dev,
        }
    }
}

// --- internal helpers -------------------------------------------------------

fn date_to_utc_noon(date: NaiveDate) -> chrono::DateTime<chrono::Utc> {
    use chrono::TimeZone;
    chrono::Utc.from_utc_datetime(&date.and_hms_opt(12, 0, 0).unwrap_or_default())
}

fn advance_one_bucket(start: NaiveDate, granularity: AnalyticsGranularity) -> NaiveDate {
    use chrono::Datelike;
    match granularity {
        AnalyticsGranularity::Daily => start + chrono::Duration::days(1),
        AnalyticsGranularity::Weekly => start + chrono::Duration::weeks(1),
        AnalyticsGranularity::Monthly => add_months(start, 1),
        AnalyticsGranularity::Quarterly => add_months(start, 3),
        AnalyticsGranularity::Yearly => {
            NaiveDate::from_ymd_opt(start.year() + 1, 1, 1).unwrap_or(start)
        }
    }
}

fn add_months(date: NaiveDate, months: u32) -> NaiveDate {
    use chrono::Datelike;
    let total = date.month0() + months;
    let year = date.year() + (total / 12) as i32;
    let month = (total % 12) + 1;
    NaiveDate::from_ymd_opt(year, month, 1).unwrap_or(date)
}

/// Standard-normal CDF via the Abramowitz-Stegun error-function approximation.
fn normal_cdf(z: f64) -> f64 {
    0.5 * (1.0 + erf(z / std::f64::consts::SQRT_2))
}

/// Two-sided p-value for a standard-normal statistic.
fn two_sided_normal_p_value(z: f64) -> f64 {
    (2.0 * (1.0 - normal_cdf(z.abs()))).clamp(0.0, 1.0)
}

/// The Gauss error function (A&S 7.1.26), max abs error < 1.5e-7.
fn erf(x: f64) -> f64 {
    const A1: f64 = 0.254_829_592;
    const A2: f64 = -0.284_496_736;
    const A3: f64 = 1.421_413_741;
    const A4: f64 = -1.453_152_027;
    const A5: f64 = 1.061_405_429;
    const P: f64 = 0.327_591_1;
    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x_abs = x.abs();
    let t = 1.0 / (1.0 + P * x_abs);
    let y = 1.0 - (((((A5 * t + A4) * t) + A3) * t + A2) * t + A1) * t * (-x_abs * x_abs).exp();
    sign * y
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Jurisdiction;
    use chrono::{TimeZone, Utc};

    fn event(id: &str, y: i32, m: u32, value: f64) -> LegalEvent {
        let ts = Utc
            .with_ymd_and_hms(y, m, 15, 12, 0, 0)
            .single()
            .expect("valid");
        LegalEvent::new(id, ts, "award")
            .with_value(value)
            .with_jurisdiction(Jurisdiction::UsFederal)
    }

    fn monotone_increasing_series() -> Vec<SeriesPoint> {
        (0..12)
            .map(|i| SeriesPoint {
                bucket: format!("2024-{:02}", i + 1),
                date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap() + chrono::Duration::days(30 * i),
                value: 100.0 + 10.0 * i as f64,
                count: 1,
            })
            .collect()
    }

    #[test]
    fn test_build_series_and_aggregation() {
        let analyzer = TrendAnalyzer::new(AnalyticsGranularity::Monthly, Aggregation::Sum);
        let events = vec![
            event("a", 2024, 1, 100.0),
            event("b", 2024, 1, 200.0),
            event("c", 2024, 3, 50.0),
        ];
        let series = analyzer.build_series(&events);
        // Jan + (filled Feb) + Mar.
        assert_eq!(series.len(), 3);
        assert_eq!(series[0].bucket, "2024-01");
        assert!((series[0].value - 300.0).abs() < 1e-9);
        assert_eq!(series[0].count, 2);
        assert_eq!(series[1].bucket, "2024-02");
        assert!((series[1].value).abs() < 1e-9);
        assert_eq!(series[1].count, 0);
        assert!((series[2].value - 50.0).abs() < 1e-9);
    }

    #[test]
    fn test_count_aggregation_no_value() {
        let analyzer = TrendAnalyzer::new(AnalyticsGranularity::Yearly, Aggregation::Count);
        let ts = |y| {
            Utc.with_ymd_and_hms(y, 6, 1, 0, 0, 0)
                .single()
                .expect("valid")
        };
        let events = vec![
            LegalEvent::new("a", ts(2022), "filing"),
            LegalEvent::new("b", ts(2022), "filing"),
            LegalEvent::new("c", ts(2023), "filing"),
        ];
        let series = analyzer.build_series(&events);
        assert_eq!(series.len(), 2);
        assert!((series[0].value - 2.0).abs() < 1e-9);
        assert!((series[1].value - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_linear_fit_perfect_line() {
        let analyzer = TrendAnalyzer::default();
        let series = monotone_increasing_series();
        let fit = analyzer.linear_fit(&series);
        assert!((fit.slope - 10.0).abs() < 1e-6);
        assert!((fit.intercept - 100.0).abs() < 1e-6);
        assert!((fit.r_squared - 1.0).abs() < 1e-9);
        assert!((fit.predict(12.0) - 220.0).abs() < 1e-6);
    }

    #[test]
    fn test_mann_kendall_detects_increase() {
        let analyzer = TrendAnalyzer::default();
        let series = monotone_increasing_series();
        let mk = analyzer.mann_kendall(&series);
        assert!(mk.s > 0.0);
        // Perfect monotone increase => tau = 1.
        assert!((mk.tau - 1.0).abs() < 1e-9);
        assert!(mk.p_value < 0.01);
    }

    #[test]
    fn test_mann_kendall_flat_series() {
        let analyzer = TrendAnalyzer::default();
        let series: Vec<SeriesPoint> = (0..10)
            .map(|i| SeriesPoint {
                bucket: format!("b{i}"),
                date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                value: 5.0,
                count: 1,
            })
            .collect();
        let mk = analyzer.mann_kendall(&series);
        assert_eq!(mk.s, 0.0);
        assert!(mk.p_value > 0.5);
        assert_eq!(mk.tau, 0.0);
    }

    #[test]
    fn test_sen_slope() {
        let analyzer = TrendAnalyzer::default();
        let series = monotone_increasing_series();
        let slope = analyzer.sen_slope(&series);
        assert!((slope - 10.0).abs() < 1e-9);
    }

    #[test]
    fn test_moving_average() {
        let analyzer = TrendAnalyzer::default();
        let series: Vec<SeriesPoint> = [2.0, 4.0, 6.0, 8.0]
            .iter()
            .enumerate()
            .map(|(i, &v)| SeriesPoint {
                bucket: format!("b{i}"),
                date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                value: v,
                count: 1,
            })
            .collect();
        let ma = analyzer.moving_average(&series, 2);
        assert_eq!(ma.len(), 4);
        assert!((ma[0] - 2.0).abs() < 1e-9);
        assert!((ma[1] - 3.0).abs() < 1e-9);
        assert!((ma[2] - 5.0).abs() < 1e-9);
        assert!((ma[3] - 7.0).abs() < 1e-9);
    }

    #[test]
    fn test_change_point_detection() {
        let analyzer = TrendAnalyzer::default();
        // Flat at 10 for 6 buckets, then jump to 50 for 6 buckets.
        let mut values = vec![10.0; 6];
        values.extend(vec![50.0; 6]);
        let series: Vec<SeriesPoint> = values
            .iter()
            .enumerate()
            .map(|(i, &v)| SeriesPoint {
                bucket: format!("2024-{:02}", i + 1),
                date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                value: v,
                count: 1,
            })
            .collect();
        let cps = analyzer.detect_change_points(&series, 0.5);
        assert!(!cps.is_empty());
        // The strongest change point should be at the regime boundary (index 6).
        assert_eq!(cps[0].index, 6);
        assert!(cps[0].mean_after > cps[0].mean_before);
    }

    #[test]
    fn test_change_point_none_on_flat() {
        let analyzer = TrendAnalyzer::default();
        let series: Vec<SeriesPoint> = (0..10)
            .map(|i| SeriesPoint {
                bucket: format!("b{i}"),
                date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                value: 7.0,
                count: 1,
            })
            .collect();
        assert!(analyzer.detect_change_points(&series, 1.0).is_empty());
    }

    #[test]
    fn test_seasonal_averages_quarterly() {
        let analyzer = TrendAnalyzer::new(AnalyticsGranularity::Quarterly, Aggregation::Sum);
        let ts = |y, m| {
            Utc.with_ymd_and_hms(y, m, 1, 0, 0, 0)
                .single()
                .expect("valid")
        };
        let events = vec![
            LegalEvent::new("a", ts(2023, 1), "x").with_value(10.0),
            LegalEvent::new("b", ts(2024, 2), "x").with_value(20.0),
            LegalEvent::new("c", ts(2023, 7), "x").with_value(100.0),
        ];
        let series = analyzer.build_series(&events);
        let seasonal = analyzer.seasonal_averages(&series);
        // Q1 buckets average (10 + 20)/2 = 15 (2023-Q1 and 2024-Q1).
        assert!((seasonal.get("Q1").copied().unwrap_or(0.0) - 15.0).abs() < 1e-9);
        assert!((seasonal.get("Q3").copied().unwrap_or(0.0) - 100.0).abs() < 1e-9);
    }

    #[test]
    fn test_full_analyze_report() {
        let analyzer = TrendAnalyzer::new(AnalyticsGranularity::Monthly, Aggregation::Mean);
        let events: Vec<LegalEvent> = (1..=12)
            .map(|m| event(&format!("e{m}"), 2024, m, 100.0 + 25.0 * m as f64))
            .collect();
        let report = analyzer.analyze(&events);
        assert_eq!(report.direction, TrendDirection::Increasing);
        assert!(report.linear_fit.slope > 0.0);
        assert!(report.percent_change > 0.0);
        assert!(report.mann_kendall.tau > 0.9);
        assert!(report.sen_slope > 0.0);
    }

    #[test]
    fn test_normal_cdf_sanity() {
        assert!((normal_cdf(0.0) - 0.5).abs() < 1e-7);
        assert!((normal_cdf(1.959_964) - 0.975).abs() < 1e-3);
        assert!(two_sided_normal_p_value(1.959_964) < 0.06);
        assert!(two_sided_normal_p_value(0.0) > 0.99);
    }
}
