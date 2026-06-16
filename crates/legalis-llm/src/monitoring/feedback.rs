//! User feedback collection and satisfaction (CSAT) metrics.
//!
//! Where [`crate::human_feedback::FeedbackCollector`] gathers rich, prompt/
//! response-linked annotations for RLHF *training*, this module collects
//! lightweight, request-linked production feedback signals (a star rating and/or
//! a thumbs up/down tied to a [`ResponseObservation`] id) and turns them into
//! the satisfaction metrics a production monitor tracks: CSAT, average rating,
//! thumbs-up rate, rating distribution, feedback rate and a CSAT trend over time.
//! It reuses the crate's existing [`crate::human_feedback::Rating`] scale.

use super::{ResponseObservation, TrendGranularity, truncate_to_bucket};
use crate::human_feedback::Rating;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// A single user feedback signal tied to a served request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FeedbackSignal {
    /// Stable unique identifier for the signal itself.
    pub id: String,
    /// The id of the [`ResponseObservation`] this feedback is about.
    pub request_id: String,
    /// When the feedback was received.
    pub timestamp: DateTime<Utc>,
    /// An optional 1-5 star rating (reuses the crate's RLHF rating scale).
    pub rating: Option<Rating>,
    /// An optional binary thumbs up (`true`) / down (`false`).
    pub thumbs_up: Option<bool>,
    /// An optional free-text comment.
    pub comment: Option<String>,
    /// Optional request category for segmentation.
    pub category: Option<String>,
    /// Optional tenant / user identifier.
    pub tenant_id: Option<String>,
}

impl FeedbackSignal {
    /// Creates a new feedback signal for a given request id.
    pub fn new(request_id: impl Into<String>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            request_id: request_id.into(),
            timestamp: Utc::now(),
            rating: None,
            thumbs_up: None,
            comment: None,
            category: None,
            tenant_id: None,
        }
    }

    /// Sets the star rating.
    pub fn with_rating(mut self, rating: Rating) -> Self {
        self.rating = Some(rating);
        self
    }

    /// Sets the thumbs up/down.
    pub fn with_thumbs(mut self, up: bool) -> Self {
        self.thumbs_up = Some(up);
        self
    }

    /// Sets a free-text comment.
    pub fn with_comment(mut self, comment: impl Into<String>) -> Self {
        self.comment = Some(comment.into());
        self
    }

    /// Sets the category.
    pub fn with_category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    /// Sets the tenant identifier.
    pub fn with_tenant(mut self, tenant_id: impl Into<String>) -> Self {
        self.tenant_id = Some(tenant_id.into());
        self
    }

    /// Overrides the timestamp.
    pub fn with_timestamp(mut self, timestamp: DateTime<Utc>) -> Self {
        self.timestamp = timestamp;
        self
    }

    /// Returns whether this signal expresses positive sentiment.
    ///
    /// A signal is positive when it carries a thumbs-up, or a rating of
    /// [`Rating::Good`] or better.
    pub fn is_positive(&self) -> bool {
        if let Some(up) = self.thumbs_up {
            return up;
        }
        matches!(self.rating, Some(rating) if rating >= Rating::Good)
    }
}

/// Aggregated satisfaction statistics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SatisfactionStats {
    /// Total feedback signals.
    pub total_signals: usize,
    /// Number of signals carrying a star rating.
    pub rated_count: usize,
    /// Average star rating across rated signals (`0.0` if none).
    pub avg_rating: f64,
    /// CSAT: percent of rated signals scoring 4-5 stars, in `[0, 100]`.
    pub csat_pct: f64,
    /// Number of thumbs-up signals.
    pub thumbs_up: usize,
    /// Number of thumbs-down signals.
    pub thumbs_down: usize,
    /// Thumbs-up rate over signals carrying a thumb, in `[0, 100]`.
    pub thumbs_up_pct: f64,
    /// Star-rating distribution keyed by star value (1-5).
    pub rating_distribution: BTreeMap<u8, usize>,
}

impl SatisfactionStats {
    /// Computes the feedback rate (signals per request) given a request count.
    pub fn feedback_rate(&self, total_requests: usize) -> f64 {
        if total_requests == 0 {
            0.0
        } else {
            self.total_signals as f64 / total_requests as f64
        }
    }
}

/// Computes satisfaction statistics from a batch of feedback signals.
pub fn satisfaction_stats(signals: &[FeedbackSignal]) -> SatisfactionStats {
    let mut rating_distribution: BTreeMap<u8, usize> = BTreeMap::new();
    let mut rating_sum = 0.0;
    let mut rated_count = 0;
    let mut satisfied = 0;
    let mut thumbs_up = 0;
    let mut thumbs_down = 0;

    for signal in signals {
        if let Some(rating) = signal.rating {
            let value = rating.value();
            *rating_distribution.entry(value).or_insert(0) += 1;
            rating_sum += value as f64;
            rated_count += 1;
            if rating >= Rating::Good {
                satisfied += 1;
            }
        }
        match signal.thumbs_up {
            Some(true) => thumbs_up += 1,
            Some(false) => thumbs_down += 1,
            None => {}
        }
    }

    let avg_rating = if rated_count == 0 {
        0.0
    } else {
        rating_sum / rated_count as f64
    };
    let csat_pct = if rated_count == 0 {
        0.0
    } else {
        satisfied as f64 / rated_count as f64 * 100.0
    };
    let thumbs_total = thumbs_up + thumbs_down;
    let thumbs_up_pct = if thumbs_total == 0 {
        0.0
    } else {
        thumbs_up as f64 / thumbs_total as f64 * 100.0
    };

    SatisfactionStats {
        total_signals: signals.len(),
        rated_count,
        avg_rating,
        csat_pct,
        thumbs_up,
        thumbs_down,
        thumbs_up_pct,
        rating_distribution,
    }
}

/// One point in a CSAT-over-time trend.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CsatTrendPoint {
    /// Start of the time bucket (UTC).
    pub bucket_start: DateTime<Utc>,
    /// Number of signals in the bucket.
    pub signal_count: usize,
    /// CSAT for the bucket in `[0, 100]`.
    pub csat_pct: f64,
    /// Average rating for the bucket.
    pub avg_rating: f64,
}

/// Computes a CSAT trend bucketed at the given granularity (sorted by time).
pub fn csat_trend(
    signals: &[FeedbackSignal],
    granularity: TrendGranularity,
) -> Vec<CsatTrendPoint> {
    let mut buckets: BTreeMap<DateTime<Utc>, Vec<FeedbackSignal>> = BTreeMap::new();
    for signal in signals {
        let key = truncate_to_bucket(signal.timestamp, granularity);
        buckets.entry(key).or_default().push(signal.clone());
    }
    buckets
        .into_iter()
        .map(|(bucket_start, bucket_signals)| {
            let stats = satisfaction_stats(&bucket_signals);
            CsatTrendPoint {
                bucket_start,
                signal_count: bucket_signals.len(),
                csat_pct: stats.csat_pct,
                avg_rating: stats.avg_rating,
            }
        })
        .collect()
}

/// A stateful collector of production user feedback.
pub struct SatisfactionTracker {
    signals: Arc<RwLock<Vec<FeedbackSignal>>>,
}

impl SatisfactionTracker {
    /// Creates a new satisfaction tracker.
    pub fn new() -> Self {
        Self {
            signals: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Records a complete feedback signal.
    pub async fn record(&self, signal: FeedbackSignal) {
        self.signals.write().await.push(signal);
    }

    /// Records a star rating for a request.
    pub async fn record_rating(&self, request_id: impl Into<String>, rating: Rating) {
        self.record(FeedbackSignal::new(request_id).with_rating(rating))
            .await;
    }

    /// Records a thumbs up/down for a request.
    pub async fn record_thumbs(&self, request_id: impl Into<String>, up: bool) {
        self.record(FeedbackSignal::new(request_id).with_thumbs(up))
            .await;
    }

    /// Computes satisfaction statistics over everything recorded.
    pub async fn stats(&self) -> SatisfactionStats {
        let signals = self.signals.read().await;
        satisfaction_stats(&signals)
    }

    /// Computes satisfaction statistics restricted to one category.
    pub async fn stats_for_category(&self, category: &str) -> SatisfactionStats {
        let signals = self.signals.read().await;
        let filtered: Vec<FeedbackSignal> = signals
            .iter()
            .filter(|signal| signal.category.as_deref() == Some(category))
            .cloned()
            .collect();
        satisfaction_stats(&filtered)
    }

    /// Computes a CSAT trend over everything recorded.
    pub async fn trend(&self, granularity: TrendGranularity) -> Vec<CsatTrendPoint> {
        let signals = self.signals.read().await;
        csat_trend(&signals, granularity)
    }

    /// Returns all comments left on feedback signals.
    pub async fn comments(&self) -> Vec<String> {
        self.signals
            .read()
            .await
            .iter()
            .filter_map(|signal| signal.comment.clone())
            .collect()
    }

    /// Returns the feedback rate given a request total.
    pub async fn feedback_rate(&self, total_requests: usize) -> f64 {
        self.stats().await.feedback_rate(total_requests)
    }

    /// Returns the number of recorded signals.
    pub async fn len(&self) -> usize {
        self.signals.read().await.len()
    }

    /// Returns whether no feedback has been recorded.
    pub async fn is_empty(&self) -> bool {
        self.signals.read().await.is_empty()
    }
}

impl Default for SatisfactionTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Links feedback signals back to their observations by request id.
///
/// Returns, for each signal that matches an observation, a pair of references so
/// callers can correlate satisfaction with latency, cost or category without
/// re-implementing the join.
pub fn join_feedback<'a>(
    observations: &'a [ResponseObservation],
    signals: &'a [FeedbackSignal],
) -> Vec<(&'a ResponseObservation, &'a FeedbackSignal)> {
    let index: BTreeMap<&str, &ResponseObservation> = observations
        .iter()
        .map(|obs| (obs.id.as_str(), obs))
        .collect();
    signals
        .iter()
        .filter_map(|signal| {
            index
                .get(signal.request_id.as_str())
                .map(|obs| (*obs, signal))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_satisfaction_stats() {
        let signals = vec![
            FeedbackSignal::new("r1").with_rating(Rating::Excellent),
            FeedbackSignal::new("r2").with_rating(Rating::Good),
            FeedbackSignal::new("r3").with_rating(Rating::Poor),
            FeedbackSignal::new("r4").with_thumbs(true),
            FeedbackSignal::new("r5").with_thumbs(false),
        ];
        let stats = satisfaction_stats(&signals);
        assert_eq!(stats.total_signals, 5);
        assert_eq!(stats.rated_count, 3);
        // 2 of 3 ratings are >= Good.
        assert!((stats.csat_pct - 2.0 / 3.0 * 100.0).abs() < 1e-9);
        // avg of 5, 4, 2 = 3.667.
        assert!((stats.avg_rating - 11.0 / 3.0).abs() < 1e-9);
        assert_eq!(stats.thumbs_up, 1);
        assert_eq!(stats.thumbs_down, 1);
        assert!((stats.thumbs_up_pct - 50.0).abs() < 1e-9);
        assert_eq!(stats.rating_distribution.get(&5), Some(&1));
    }

    #[test]
    fn test_signal_is_positive() {
        assert!(FeedbackSignal::new("r").with_thumbs(true).is_positive());
        assert!(!FeedbackSignal::new("r").with_thumbs(false).is_positive());
        assert!(
            FeedbackSignal::new("r")
                .with_rating(Rating::Good)
                .is_positive()
        );
        assert!(
            !FeedbackSignal::new("r")
                .with_rating(Rating::Poor)
                .is_positive()
        );
        assert!(!FeedbackSignal::new("r").is_positive());
    }

    #[test]
    fn test_feedback_rate() {
        let signals = vec![
            FeedbackSignal::new("r1").with_thumbs(true),
            FeedbackSignal::new("r2").with_thumbs(true),
        ];
        let stats = satisfaction_stats(&signals);
        assert!((stats.feedback_rate(10) - 0.2).abs() < 1e-9);
        assert_eq!(stats.feedback_rate(0), 0.0);
    }

    #[test]
    fn test_csat_trend() {
        let base = Utc::now();
        let signals = vec![
            FeedbackSignal::new("r1")
                .with_rating(Rating::Excellent)
                .with_timestamp(base),
            FeedbackSignal::new("r2")
                .with_rating(Rating::Poor)
                .with_timestamp(base + chrono::Duration::days(1)),
        ];
        let trend = csat_trend(&signals, TrendGranularity::Daily);
        assert_eq!(trend.len(), 2);
        assert!((trend[0].csat_pct - 100.0).abs() < 1e-9);
        assert!((trend[1].csat_pct - 0.0).abs() < 1e-9);
    }

    #[test]
    fn test_join_feedback() {
        let obs = ResponseObservation::new("openai", "gpt-4").with_id("req-1");
        let observations = vec![obs];
        let signals = vec![
            FeedbackSignal::new("req-1").with_thumbs(true),
            FeedbackSignal::new("missing").with_thumbs(false),
        ];
        let joined = join_feedback(&observations, &signals);
        assert_eq!(joined.len(), 1);
        assert_eq!(joined[0].0.id, "req-1");
    }

    #[tokio::test]
    async fn test_tracker() {
        let tracker = SatisfactionTracker::new();
        tracker.record_rating("r1", Rating::Excellent).await;
        tracker.record_thumbs("r2", true).await;
        tracker
            .record(
                FeedbackSignal::new("r3")
                    .with_rating(Rating::Poor)
                    .with_category("billing")
                    .with_comment("too slow"),
            )
            .await;

        assert_eq!(tracker.len().await, 3);
        assert!(!tracker.is_empty().await);

        let stats = tracker.stats().await;
        assert_eq!(stats.total_signals, 3);

        let billing = tracker.stats_for_category("billing").await;
        assert_eq!(billing.total_signals, 1);

        let comments = tracker.comments().await;
        assert_eq!(comments, vec!["too slow".to_string()]);
        assert!((tracker.feedback_rate(6).await - 0.5).abs() < 1e-9);
    }
}
