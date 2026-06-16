//! Error-rate tracking by category.
//!
//! Aggregates the classified [`crate::ErrorCategory`] failures carried by
//! [`ResponseObservation`]s into overall and per-category error rates, a
//! time-bucketed error-rate trend and a burst detector that flags time windows
//! where the error rate spikes well above the recent baseline.

use super::{ErrorCategory, ResponseObservation, TrendGranularity, median, truncate_to_bucket};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// An aggregated error-rate report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorRateReport {
    /// Total requests considered.
    pub total_requests: usize,
    /// Total failed requests.
    pub total_errors: usize,
    /// Overall error rate in `[0, 1]`.
    pub error_rate: f64,
    /// Number of errors per category.
    pub errors_by_category: BTreeMap<ErrorCategory, usize>,
    /// Error rate per category in `[0, 1]` (category errors / total requests).
    pub error_rate_by_category: BTreeMap<ErrorCategory, f64>,
    /// Number of errors that are retryable.
    pub retryable_errors: usize,
    /// Retryable errors as a fraction of all errors in `[0, 1]`.
    pub retryable_error_fraction: f64,
}

impl ErrorRateReport {
    /// Returns the success rate in `[0, 1]`.
    pub fn success_rate(&self) -> f64 {
        1.0 - self.error_rate
    }

    /// Returns the most common error category, if any errors occurred.
    pub fn dominant_category(&self) -> Option<ErrorCategory> {
        self.errors_by_category
            .iter()
            .max_by_key(|(_, count)| **count)
            .map(|(category, _)| *category)
    }
}

/// Computes an error-rate report over a batch of observations.
pub fn error_rate_report(observations: &[ResponseObservation]) -> ErrorRateReport {
    let total_requests = observations.len();
    let mut errors_by_category: BTreeMap<ErrorCategory, usize> = BTreeMap::new();
    let mut retryable_errors = 0;

    for obs in observations {
        if let Some(category) = obs.outcome.error_category() {
            *errors_by_category.entry(category).or_insert(0) += 1;
            if category.is_retryable() {
                retryable_errors += 1;
            }
        }
    }

    let total_errors: usize = errors_by_category.values().sum();
    let error_rate = if total_requests == 0 {
        0.0
    } else {
        total_errors as f64 / total_requests as f64
    };
    let error_rate_by_category = errors_by_category
        .iter()
        .map(|(category, count)| {
            let rate = if total_requests == 0 {
                0.0
            } else {
                *count as f64 / total_requests as f64
            };
            (*category, rate)
        })
        .collect();
    let retryable_error_fraction = if total_errors == 0 {
        0.0
    } else {
        retryable_errors as f64 / total_errors as f64
    };

    ErrorRateReport {
        total_requests,
        total_errors,
        error_rate,
        errors_by_category,
        error_rate_by_category,
        retryable_errors,
        retryable_error_fraction,
    }
}

/// One window of a time-bucketed error-rate trend.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorRateWindow {
    /// Start of the time bucket (UTC).
    pub bucket_start: DateTime<Utc>,
    /// Requests in the bucket.
    pub total_requests: usize,
    /// Errors in the bucket.
    pub total_errors: usize,
    /// Error rate in the bucket in `[0, 1]`.
    pub error_rate: f64,
}

/// Computes a time-bucketed error-rate trend (sorted by time).
pub fn error_rate_trend(
    observations: &[ResponseObservation],
    granularity: TrendGranularity,
) -> Vec<ErrorRateWindow> {
    // (total, errors) per bucket.
    let mut buckets: BTreeMap<DateTime<Utc>, (usize, usize)> = BTreeMap::new();
    for obs in observations {
        let key = truncate_to_bucket(obs.timestamp, granularity);
        let entry = buckets.entry(key).or_insert((0, 0));
        entry.0 += 1;
        if obs.outcome.is_error() {
            entry.1 += 1;
        }
    }
    buckets
        .into_iter()
        .map(|(bucket_start, (total, errors))| ErrorRateWindow {
            bucket_start,
            total_requests: total,
            total_errors: errors,
            error_rate: if total > 0 {
                errors as f64 / total as f64
            } else {
                0.0
            },
        })
        .collect()
}

/// Configuration for error-burst detection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorBurstConfig {
    /// Bucket granularity for the analysis.
    pub granularity: TrendGranularity,
    /// Minimum errors in a bucket before it can be a burst.
    pub min_errors: usize,
    /// Absolute error-rate floor a burst must exceed (in `[0, 1]`).
    pub absolute_threshold: f64,
    /// A burst's rate must exceed `spike_factor x baseline` (median of buckets).
    pub spike_factor: f64,
}

impl Default for ErrorBurstConfig {
    fn default() -> Self {
        Self {
            granularity: TrendGranularity::Hourly,
            min_errors: 3,
            absolute_threshold: 0.25,
            spike_factor: 3.0,
        }
    }
}

/// A detected error burst within one time bucket.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorBurst {
    /// Start of the bursting bucket (UTC).
    pub bucket_start: DateTime<Utc>,
    /// Error rate during the burst in `[0, 1]`.
    pub error_rate: f64,
    /// Baseline (median) error rate the burst is compared against.
    pub baseline_rate: f64,
    /// Number of errors during the burst.
    pub total_errors: usize,
    /// The dominant error category during the burst, if determinable.
    pub dominant_category: Option<ErrorCategory>,
}

/// Detects error bursts across a batch of observations.
///
/// A bucket is a burst when it carries at least `min_errors` failures and its
/// error rate exceeds both the absolute floor and `spike_factor` times the
/// median bucket rate. Comparing against the median (rather than the mean) keeps
/// the baseline robust to the very bursts being detected.
pub fn detect_error_bursts(
    observations: &[ResponseObservation],
    config: &ErrorBurstConfig,
) -> Vec<ErrorBurst> {
    let windows = error_rate_trend(observations, config.granularity);
    if windows.is_empty() {
        return Vec::new();
    }
    let rates: Vec<f64> = windows.iter().map(|window| window.error_rate).collect();
    let baseline = median(&rates);
    let dynamic_threshold = (baseline * config.spike_factor).max(config.absolute_threshold);

    windows
        .iter()
        .filter(|window| {
            window.total_errors >= config.min_errors && window.error_rate >= dynamic_threshold
        })
        .map(|window| ErrorBurst {
            bucket_start: window.bucket_start,
            error_rate: window.error_rate,
            baseline_rate: baseline,
            total_errors: window.total_errors,
            dominant_category: dominant_category_in_bucket(
                observations,
                window.bucket_start,
                config.granularity,
            ),
        })
        .collect()
}

/// Finds the most common error category among observations in one bucket.
fn dominant_category_in_bucket(
    observations: &[ResponseObservation],
    bucket_start: DateTime<Utc>,
    granularity: TrendGranularity,
) -> Option<ErrorCategory> {
    let mut counts: BTreeMap<ErrorCategory, usize> = BTreeMap::new();
    for obs in observations {
        if truncate_to_bucket(obs.timestamp, granularity) != bucket_start {
            continue;
        }
        if let Some(category) = obs.outcome.error_category() {
            *counts.entry(category).or_insert(0) += 1;
        }
    }
    counts
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(category, _)| category)
}

/// A stateful error-rate tracker.
pub struct ErrorRateTracker {
    observations: Arc<RwLock<Vec<ResponseObservation>>>,
}

impl ErrorRateTracker {
    /// Creates a new error-rate tracker.
    pub fn new() -> Self {
        Self {
            observations: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Records an observation.
    pub async fn record(&self, observation: ResponseObservation) {
        self.observations.write().await.push(observation);
    }

    /// Computes an error-rate report over everything recorded.
    pub async fn report(&self) -> ErrorRateReport {
        let observations = self.observations.read().await;
        error_rate_report(&observations)
    }

    /// Computes an error-rate report over observations at or after `since`.
    pub async fn report_since(&self, since: DateTime<Utc>) -> ErrorRateReport {
        let observations = self.observations.read().await;
        let filtered: Vec<ResponseObservation> = observations
            .iter()
            .filter(|obs| obs.timestamp >= since)
            .cloned()
            .collect();
        error_rate_report(&filtered)
    }

    /// Computes a time-bucketed error-rate trend.
    pub async fn trend(&self, granularity: TrendGranularity) -> Vec<ErrorRateWindow> {
        let observations = self.observations.read().await;
        error_rate_trend(&observations, granularity)
    }

    /// Detects error bursts over everything recorded.
    pub async fn bursts(&self, config: &ErrorBurstConfig) -> Vec<ErrorBurst> {
        let observations = self.observations.read().await;
        detect_error_bursts(&observations, config)
    }

    /// Returns the number of recorded observations.
    pub async fn len(&self) -> usize {
        self.observations.read().await.len()
    }

    /// Returns whether nothing has been recorded.
    pub async fn is_empty(&self) -> bool {
        self.observations.read().await.is_empty()
    }
}

impl Default for ErrorRateTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ok() -> ResponseObservation {
        ResponseObservation::new("openai", "gpt-4")
    }

    fn err(category: ErrorCategory) -> ResponseObservation {
        ResponseObservation::new("openai", "gpt-4").with_error(category)
    }

    #[test]
    fn test_error_rate_report() {
        let observations = vec![
            ok(),
            ok(),
            err(ErrorCategory::RateLimit),
            err(ErrorCategory::Timeout),
            err(ErrorCategory::Authentication),
        ];
        let report = error_rate_report(&observations);
        assert_eq!(report.total_requests, 5);
        assert_eq!(report.total_errors, 3);
        assert!((report.error_rate - 0.6).abs() < 1e-9);
        assert!((report.success_rate() - 0.4).abs() < 1e-9);
        assert_eq!(
            report.errors_by_category.get(&ErrorCategory::RateLimit),
            Some(&1)
        );
        // Timeout + RateLimit are retryable, Authentication is not.
        assert_eq!(report.retryable_errors, 2);
        assert!((report.retryable_error_fraction - 2.0 / 3.0).abs() < 1e-9);
    }

    #[test]
    fn test_dominant_category() {
        let observations = vec![
            err(ErrorCategory::Timeout),
            err(ErrorCategory::Timeout),
            err(ErrorCategory::RateLimit),
        ];
        let report = error_rate_report(&observations);
        assert_eq!(report.dominant_category(), Some(ErrorCategory::Timeout));
    }

    #[test]
    fn test_error_rate_by_category_normalised_to_requests() {
        let observations = vec![
            ok(),
            ok(),
            err(ErrorCategory::Network),
            err(ErrorCategory::Network),
        ];
        let report = error_rate_report(&observations);
        // 2 network errors out of 4 requests => 0.5.
        assert!(
            (report
                .error_rate_by_category
                .get(&ErrorCategory::Network)
                .copied()
                .unwrap_or(0.0)
                - 0.5)
                .abs()
                < 1e-9
        );
    }

    #[test]
    fn test_error_rate_trend() {
        let base = Utc::now();
        let observations = vec![
            ok().with_timestamp(base),
            err(ErrorCategory::Timeout).with_timestamp(base + chrono::Duration::minutes(5)),
            ok().with_timestamp(base + chrono::Duration::days(1)),
        ];
        let trend = error_rate_trend(&observations, TrendGranularity::Daily);
        assert_eq!(trend.len(), 2);
        assert!((trend[0].error_rate - 0.5).abs() < 1e-9);
        assert!((trend[1].error_rate - 0.0).abs() < 1e-9);
    }

    #[test]
    fn test_burst_detection() {
        let base = Utc::now();
        let mut observations = Vec::new();
        // Hour 0..3: healthy traffic (low error rate).
        for hour in 0..3 {
            for index in 0..20 {
                let ts = base + chrono::Duration::hours(hour) + chrono::Duration::seconds(index);
                if index == 0 {
                    observations.push(err(ErrorCategory::Network).with_timestamp(ts));
                } else {
                    observations.push(ok().with_timestamp(ts));
                }
            }
        }
        // Hour 5: a burst - almost all requests fail.
        for index in 0..20 {
            let ts = base + chrono::Duration::hours(5) + chrono::Duration::seconds(index);
            observations.push(err(ErrorCategory::ServiceUnavailable).with_timestamp(ts));
        }

        let bursts = detect_error_bursts(&observations, &ErrorBurstConfig::default());
        assert_eq!(bursts.len(), 1);
        assert!(bursts[0].error_rate > bursts[0].baseline_rate);
        assert_eq!(
            bursts[0].dominant_category,
            Some(ErrorCategory::ServiceUnavailable)
        );
    }

    #[tokio::test]
    async fn test_tracker() {
        let tracker = ErrorRateTracker::new();
        tracker.record(ok()).await;
        tracker.record(err(ErrorCategory::Timeout)).await;
        assert_eq!(tracker.len().await, 2);
        assert!(!tracker.is_empty().await);
        let report = tracker.report().await;
        assert!((report.error_rate - 0.5).abs() < 1e-9);
    }
}
