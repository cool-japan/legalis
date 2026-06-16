//! Predictive compliance — forecasting compliance drift from history.
//!
//! Where [`crate::autonomous::monitor`] answers "are we compliant *now*?", this
//! module answers "where are we *heading*?". It buckets historical audit records
//! into equal time windows, computes a per-bucket compliance signal (override /
//! void / discretion rate), fits a simple ordinary-least-squares trend line to
//! each signal, and extrapolates it forward to estimate when — if ever — a
//! signal will cross a configured threshold (a predicted breach).
//!
//! The statistics are deliberately lightweight and fully explainable (slope,
//! intercept, R², residual standard error, and a forecast horizon), so the
//! output is auditable rather than a black box. It complements the per-record
//! [`crate::predictive`] violation forecaster and the [`crate::insights`]
//! period-over-period tracker by focusing on *threshold-crossing time* for the
//! core compliance rates.

use crate::autonomous::monitor::MonitoredMetric;
use crate::{Actor, AuditRecord, DecisionResult, EventType};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

/// Configuration for compliance-drift forecasting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftConfig {
    /// Width of each history bucket.
    pub bucket: Duration,
    /// Minimum number of non-empty buckets required to fit a trend.
    pub min_buckets: usize,
    /// Maximum forecast horizon. A breach predicted beyond this is reported as
    /// "no breach within horizon".
    pub horizon: Duration,
    /// Override-rate threshold considered a breach.
    pub override_threshold: f64,
    /// Void-rate threshold considered a breach.
    pub void_threshold: f64,
    /// Discretion-rate threshold considered a breach.
    pub discretion_threshold: f64,
}

impl Default for DriftConfig {
    fn default() -> Self {
        Self {
            bucket: Duration::days(1),
            min_buckets: 3,
            horizon: Duration::days(30),
            override_threshold: 0.25,
            void_threshold: 0.1,
            discretion_threshold: 0.4,
        }
    }
}

/// The fitted parameters of an ordinary-least-squares line `y = slope * x +
/// intercept` over bucket index `x`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendFit {
    /// Slope (change in rate per bucket).
    pub slope: f64,
    /// Intercept (fitted rate at bucket 0).
    pub intercept: f64,
    /// Coefficient of determination R² in `[0, 1]`.
    pub r_squared: f64,
    /// Residual standard error.
    pub residual_std_error: f64,
    /// Number of points used in the fit.
    pub points: usize,
    /// Mean of the observed values.
    pub mean: f64,
}

impl TrendFit {
    /// Fits a least-squares line to `(0, y0), (1, y1), ...`. Returns `None` for
    /// fewer than two points.
    pub fn fit(values: &[f64]) -> Option<Self> {
        let n = values.len();
        if n < 2 {
            return None;
        }
        let nf = n as f64;
        let mean_x = (n as f64 - 1.0) / 2.0;
        let mean_y = values.iter().sum::<f64>() / nf;

        let mut sxx = 0.0;
        let mut sxy = 0.0;
        let mut syy = 0.0;
        for (i, &y) in values.iter().enumerate() {
            let dx = i as f64 - mean_x;
            let dy = y - mean_y;
            sxx += dx * dx;
            sxy += dx * dy;
            syy += dy * dy;
        }
        let slope = if sxx.abs() < f64::EPSILON {
            0.0
        } else {
            sxy / sxx
        };
        let intercept = mean_y - slope * mean_x;

        // R² = 1 - SS_res / SS_tot.
        let mut ss_res = 0.0;
        for (i, &y) in values.iter().enumerate() {
            let pred = slope * i as f64 + intercept;
            let resid = y - pred;
            ss_res += resid * resid;
        }
        let r_squared = if syy.abs() < f64::EPSILON {
            // Perfectly flat data: a flat line fits perfectly.
            1.0
        } else {
            (1.0 - ss_res / syy).clamp(0.0, 1.0)
        };
        let residual_std_error = if n > 2 {
            (ss_res / (n as f64 - 2.0)).max(0.0).sqrt()
        } else {
            0.0
        };

        Some(Self {
            slope,
            intercept,
            r_squared,
            residual_std_error,
            points: n,
            mean: mean_y,
        })
    }

    /// Predicts the value at bucket index `x`.
    pub fn predict(&self, x: f64) -> f64 {
        self.slope * x + self.intercept
    }

    /// Solves for the (possibly fractional) bucket index at which the line
    /// crosses `threshold`, or `None` when the slope is flat / moving away.
    fn crossing_index(&self, threshold: f64, last_index: usize) -> Option<f64> {
        if self.slope.abs() < f64::EPSILON {
            return None;
        }
        // x such that slope * x + intercept = threshold.
        let x = (threshold - self.intercept) / self.slope;
        if x < last_index as f64 {
            // Crossing is in the past (or already breached); not a forecast.
            None
        } else {
            Some(x)
        }
    }
}

/// The trend direction of a compliance signal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DriftDirection {
    /// Rate is climbing toward its threshold.
    Worsening,
    /// Rate is essentially flat.
    Stable,
    /// Rate is falling away from its threshold.
    Improving,
}

/// A forecast for a single compliance signal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftForecast {
    /// The metric being forecast.
    pub metric: MonitoredMetric,
    /// The fitted trend.
    pub trend: TrendFit,
    /// Qualitative direction.
    pub direction: DriftDirection,
    /// The most recent observed rate.
    pub current_rate: f64,
    /// The breach threshold for this signal.
    pub threshold: f64,
    /// Whether the most recent observation already breaches the threshold.
    pub already_breached: bool,
    /// Forecast value at the end of the horizon.
    pub projected_rate: f64,
    /// Estimated time until the threshold is crossed, when within the horizon.
    pub time_to_breach: Option<Duration>,
    /// Estimated timestamp of the breach, when within the horizon.
    pub estimated_breach_at: Option<DateTime<Utc>>,
    /// Confidence in the forecast in `[0, 1]` (driven by fit quality and
    /// sample size).
    pub confidence: f64,
    /// Human-readable summary.
    pub summary: String,
}

/// The complete output of a drift-forecasting run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftReport {
    /// One forecast per evaluated signal.
    pub forecasts: Vec<DriftForecast>,
    /// Number of buckets used.
    pub buckets: usize,
    /// The window covered (start, end), if any.
    pub window_start: Option<DateTime<Utc>>,
    /// End of the window.
    pub window_end: Option<DateTime<Utc>>,
    /// When the report was produced.
    pub generated_at: DateTime<Utc>,
}

impl DriftReport {
    /// All forecasts predicting a breach within the horizon, soonest first.
    pub fn impending_breaches(&self) -> Vec<&DriftForecast> {
        let mut v: Vec<&DriftForecast> = self
            .forecasts
            .iter()
            .filter(|f| f.time_to_breach.is_some() || f.already_breached)
            .collect();
        v.sort_by(|a, b| {
            let ax = a.time_to_breach.map(|d| d.num_seconds()).unwrap_or(0);
            let bx = b.time_to_breach.map(|d| d.num_seconds()).unwrap_or(0);
            ax.cmp(&bx)
        });
        v
    }
}

/// Forecasts compliance drift from historical records.
#[derive(Debug, Clone)]
pub struct ComplianceForecaster {
    config: DriftConfig,
}

impl ComplianceForecaster {
    /// Creates a forecaster with default configuration.
    pub fn new() -> Self {
        Self::with_config(DriftConfig::default())
    }

    /// Creates a forecaster with a custom configuration.
    pub fn with_config(config: DriftConfig) -> Self {
        Self { config }
    }

    /// Returns the active configuration.
    pub fn config(&self) -> &DriftConfig {
        &self.config
    }

    /// Produces a [`DriftReport`] forecasting the override, void, and discretion
    /// rates.
    pub fn forecast(&self, records: &[AuditRecord]) -> DriftReport {
        let now = Utc::now();
        let buckets = self.bucketize(records);
        let bucket_count = buckets.len();
        let (window_start, window_end) = if records.is_empty() {
            (None, None)
        } else {
            let min = records.iter().map(|r| r.timestamp).min();
            let max = records.iter().map(|r| r.timestamp).max();
            (min, max)
        };

        let mut forecasts = Vec::new();
        if bucket_count >= self.config.min_buckets {
            if let Some(f) = self.forecast_signal(
                &buckets,
                MonitoredMetric::OverrideRate,
                self.config.override_threshold,
                |b| b.override_rate,
                window_end,
                now,
            ) {
                forecasts.push(f);
            }
            if let Some(f) = self.forecast_signal(
                &buckets,
                MonitoredMetric::VoidRate,
                self.config.void_threshold,
                |b| b.void_rate,
                window_end,
                now,
            ) {
                forecasts.push(f);
            }
            if let Some(f) = self.forecast_signal(
                &buckets,
                MonitoredMetric::DiscretionRate,
                self.config.discretion_threshold,
                |b| b.discretion_rate,
                window_end,
                now,
            ) {
                forecasts.push(f);
            }
        }

        DriftReport {
            forecasts,
            buckets: bucket_count,
            window_start,
            window_end,
            generated_at: now,
        }
    }

    /// Forecasts a single signal extracted via `extract`.
    fn forecast_signal(
        &self,
        buckets: &[BucketStats],
        metric: MonitoredMetric,
        threshold: f64,
        extract: impl Fn(&BucketStats) -> f64,
        window_end: Option<DateTime<Utc>>,
        now: DateTime<Utc>,
    ) -> Option<DriftForecast> {
        let values: Vec<f64> = buckets.iter().map(&extract).collect();
        let trend = TrendFit::fit(&values)?;
        let last_index = values.len().saturating_sub(1);
        let current_rate = *values.last().unwrap_or(&0.0);
        let already_breached = current_rate >= threshold;

        // Project to horizon: how many buckets fit in the horizon.
        let bucket_secs = self.config.bucket.num_seconds().max(1);
        let horizon_buckets = self.config.horizon.num_seconds() as f64 / bucket_secs as f64;
        let projected_rate = trend
            .predict(last_index as f64 + horizon_buckets)
            .clamp(0.0, 1.0);

        let direction = if trend.slope > 1e-6 {
            DriftDirection::Worsening
        } else if trend.slope < -1e-6 {
            DriftDirection::Improving
        } else {
            DriftDirection::Stable
        };

        // Time to breach via the fitted crossing.
        let (time_to_breach, estimated_breach_at) = if already_breached {
            (None, None)
        } else if let Some(cross_x) = trend.crossing_index(threshold, last_index) {
            let buckets_ahead = cross_x - last_index as f64;
            if buckets_ahead <= horizon_buckets {
                let secs = (buckets_ahead * bucket_secs as f64).round() as i64;
                let ttb = Duration::seconds(secs.max(0));
                let at = window_end.unwrap_or(now) + ttb;
                (Some(ttb), Some(at))
            } else {
                (None, None)
            }
        } else {
            (None, None)
        };

        // Confidence blends fit quality with sample size.
        let size_factor = (trend.points as f64 / (trend.points as f64 + 4.0)).clamp(0.0, 1.0);
        let confidence = (trend.r_squared * 0.7 + size_factor * 0.3).clamp(0.0, 1.0);

        let summary = if already_breached {
            format!(
                "{} already breaches threshold ({:.1}% >= {:.1}%)",
                metric.label(),
                current_rate * 100.0,
                threshold * 100.0
            )
        } else if let Some(at) = estimated_breach_at {
            format!(
                "{} projected to breach {:.1}% around {} (trend {:?}, confidence {:.0}%)",
                metric.label(),
                threshold * 100.0,
                at.to_rfc3339(),
                direction,
                confidence * 100.0
            )
        } else {
            format!(
                "{} not projected to breach within horizon (trend {:?})",
                metric.label(),
                direction
            )
        };

        Some(DriftForecast {
            metric,
            trend,
            direction,
            current_rate,
            threshold,
            already_breached,
            projected_rate,
            time_to_breach,
            estimated_breach_at,
            confidence,
            summary,
        })
    }

    /// Buckets records into equal time windows aligned to the bucket width,
    /// returning per-bucket stats in chronological order (empty buckets within
    /// the span are included with zero rates so trend slope reflects gaps).
    fn bucketize(&self, records: &[AuditRecord]) -> Vec<BucketStats> {
        if records.is_empty() {
            return Vec::new();
        }
        let bucket_secs = self.config.bucket.num_seconds().max(1);
        let min_ts = records
            .iter()
            .map(|r| r.timestamp.timestamp())
            .min()
            .unwrap_or(0);
        let max_ts = records
            .iter()
            .map(|r| r.timestamp.timestamp())
            .max()
            .unwrap_or(0);
        let first_bucket = min_ts.div_euclid(bucket_secs);
        let last_bucket = max_ts.div_euclid(bucket_secs);
        let span = (last_bucket - first_bucket + 1).max(1) as usize;
        // Guard against pathological spans (e.g. tiny bucket over huge range).
        let span = span.min(100_000);

        let mut stats: Vec<BucketStats> = (0..span).map(|_| BucketStats::default()).collect();
        for r in records {
            let idx = (r.timestamp.timestamp().div_euclid(bucket_secs) - first_bucket) as usize;
            if let Some(b) = stats.get_mut(idx.min(span - 1)) {
                b.accumulate(r);
            }
        }
        for b in &mut stats {
            b.finalize();
        }
        stats
    }
}

impl Default for ComplianceForecaster {
    fn default() -> Self {
        Self::new()
    }
}

/// Mutable per-bucket accumulator finalised into rates.
#[derive(Debug, Default, Clone)]
struct BucketStats {
    total: usize,
    overrides: usize,
    voids: usize,
    discretion: usize,
    override_rate: f64,
    void_rate: f64,
    discretion_rate: f64,
}

impl BucketStats {
    fn accumulate(&mut self, r: &AuditRecord) {
        self.total += 1;
        match &r.result {
            DecisionResult::Overridden { .. } => self.overrides += 1,
            DecisionResult::Void { .. } => self.voids += 1,
            DecisionResult::RequiresDiscretion { .. } => self.discretion += 1,
            DecisionResult::Deterministic { .. } => {}
        }
        if matches!(r.event_type, EventType::HumanOverride)
            && !matches!(r.result, DecisionResult::Overridden { .. })
        {
            self.overrides += 1;
        }
        // External actors increase discretionary involvement signal weakly; not
        // counted here to keep signals orthogonal.
        let _ = matches!(r.actor, Actor::External { .. });
    }

    fn finalize(&mut self) {
        if self.total == 0 {
            return;
        }
        let denom = self.total as f64;
        self.override_rate = self.overrides as f64 / denom;
        self.void_rate = self.voids as f64 / denom;
        self.discretion_rate = self.discretion as f64 / denom;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DecisionContext;
    use std::collections::HashMap as StdHashMap;

    fn det(ts: DateTime<Utc>) -> AuditRecord {
        AuditRecord {
            id: uuid::Uuid::new_v4(),
            timestamp: ts,
            event_type: EventType::AutomaticDecision,
            actor: Actor::System {
                component: "engine".to_string(),
            },
            statute_id: "s".to_string(),
            subject_id: uuid::Uuid::new_v4(),
            context: DecisionContext::default(),
            result: DecisionResult::Deterministic {
                effect_applied: "approved".to_string(),
                parameters: StdHashMap::new(),
            },
            previous_hash: None,
            record_hash: String::new(),
        }
    }

    fn voided(ts: DateTime<Utc>) -> AuditRecord {
        let mut r = det(ts);
        r.result = DecisionResult::Void {
            reason: "logic error".to_string(),
        };
        r
    }

    #[test]
    fn test_trend_fit_perfect_line() {
        let fit = TrendFit::fit(&[0.0, 1.0, 2.0, 3.0]).expect("fit");
        assert!((fit.slope - 1.0).abs() < 1e-9);
        assert!((fit.intercept - 0.0).abs() < 1e-9);
        assert!((fit.r_squared - 1.0).abs() < 1e-9);
        assert!((fit.predict(4.0) - 4.0).abs() < 1e-9);
    }

    #[test]
    fn test_trend_fit_flat() {
        let fit = TrendFit::fit(&[0.5, 0.5, 0.5]).expect("fit");
        assert!(fit.slope.abs() < 1e-9);
        assert!((fit.r_squared - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_trend_fit_too_few_points() {
        assert!(TrendFit::fit(&[1.0]).is_none());
        assert!(TrendFit::fit(&[]).is_none());
    }

    #[test]
    fn test_forecast_rising_void_rate_predicts_breach() {
        // Build a clearly rising void rate over several daily buckets.
        let now = Utc::now();
        let mut records = Vec::new();
        // 6 days back to today; each day, void fraction increases.
        for day_back in (0i64..6).rev() {
            let day = now - Duration::days(day_back);
            // 10 records/day; voids increase from 0 to ~5.
            let voids = 5 - day_back; // day_back 5 -> 0 voids ... day_back 0 -> 5 voids
            for i in 0i64..10 {
                if i < voids {
                    records.push(voided(day + Duration::minutes(i)));
                } else {
                    records.push(det(day + Duration::minutes(i)));
                }
            }
        }
        let forecaster = ComplianceForecaster::new();
        let report = forecaster.forecast(&records);
        let void_fc = report
            .forecasts
            .iter()
            .find(|f| f.metric == MonitoredMetric::VoidRate)
            .expect("void forecast");
        assert_eq!(void_fc.direction, DriftDirection::Worsening);
        // Either already breached or projected to breach within horizon.
        assert!(void_fc.already_breached || void_fc.time_to_breach.is_some());
    }

    #[test]
    fn test_forecast_stable_no_breach() {
        let now = Utc::now();
        let mut records = Vec::new();
        for day_back in (0i64..6).rev() {
            let day = now - Duration::days(day_back);
            for i in 0i64..10 {
                records.push(det(day + Duration::minutes(i)));
            }
        }
        let forecaster = ComplianceForecaster::new();
        let report = forecaster.forecast(&records);
        let void_fc = report
            .forecasts
            .iter()
            .find(|f| f.metric == MonitoredMetric::VoidRate)
            .expect("void forecast");
        assert_eq!(void_fc.direction, DriftDirection::Stable);
        assert!(void_fc.time_to_breach.is_none());
        assert!(!void_fc.already_breached);
    }

    #[test]
    fn test_insufficient_buckets_yields_empty() {
        let now = Utc::now();
        // All in one bucket -> only 1 bucket < min_buckets.
        let records: Vec<AuditRecord> = (0..5).map(|i| det(now + Duration::minutes(i))).collect();
        let forecaster = ComplianceForecaster::new();
        let report = forecaster.forecast(&records);
        assert!(report.forecasts.is_empty());
        assert!(report.buckets <= 1);
    }

    #[test]
    fn test_impending_breaches_sorted() {
        let now = Utc::now();
        let mut records = Vec::new();
        for day_back in (0i64..8).rev() {
            let day = now - Duration::days(day_back);
            let voids = (7 - day_back).min(10);
            for i in 0i64..10 {
                if i < voids {
                    records.push(voided(day + Duration::minutes(i)));
                } else {
                    records.push(det(day + Duration::minutes(i)));
                }
            }
        }
        let report = ComplianceForecaster::new().forecast(&records);
        let impending = report.impending_breaches();
        // Sorted ascending by time-to-breach where present.
        for pair in impending.windows(2) {
            let a = pair[0].time_to_breach.map(|d| d.num_seconds()).unwrap_or(0);
            let b = pair[1].time_to_breach.map(|d| d.num_seconds()).unwrap_or(0);
            assert!(a <= b);
        }
    }

    #[test]
    fn test_report_serializes() {
        let now = Utc::now();
        let mut records = Vec::new();
        for day_back in (0i64..5).rev() {
            let day = now - Duration::days(day_back);
            for i in 0i64..5 {
                records.push(det(day + Duration::minutes(i)));
            }
        }
        let report = ComplianceForecaster::new().forecast(&records);
        let json = serde_json::to_string(&report).expect("serialize");
        assert!(json.contains("forecasts"));
    }
}
