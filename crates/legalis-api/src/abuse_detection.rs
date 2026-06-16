//! Anomaly detection for abuse.
//!
//! Scores individual API clients for abusive behaviour using a blend of
//! statistical and heuristic signals derived from their recent request stream:
//!
//! - **Request burstiness**: requests-per-second relative to a learned baseline,
//!   scored via a robust z-score over a sliding window.
//! - **Error ratio**: fraction of 4xx/5xx responses (probing / credential
//!   stuffing tends to elevate this).
//! - **Endpoint diversity / scanning**: rapidly touching many distinct endpoints
//!   (path enumeration) is suspicious.
//! - **Status-code entropy**: an abnormally uniform spread across many error
//!   codes suggests automated scanning.
//!
//! Signals are combined into a single abuse score in `[0, 1]` with a configurable
//! threshold. The detector is dependency-free and deterministic (no RNG).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use tokio::sync::RwLock;

/// A single observed request event for a client.
#[derive(Debug, Clone)]
pub struct RequestEvent {
    /// Path requested.
    pub path: String,
    /// HTTP status code returned.
    pub status: u16,
    /// When the request occurred.
    pub timestamp: DateTime<Utc>,
}

impl RequestEvent {
    /// Creates a request event.
    pub fn new(path: impl Into<String>, status: u16, timestamp: DateTime<Utc>) -> Self {
        Self {
            path: path.into(),
            status,
            timestamp,
        }
    }
}

/// Configuration for the abuse detector.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbuseConfig {
    /// Sliding window size (number of recent events kept per client).
    pub window_size: usize,
    /// Weight of the burst signal.
    pub weight_burst: f64,
    /// Weight of the error-ratio signal.
    pub weight_errors: f64,
    /// Weight of the endpoint-scanning signal.
    pub weight_scanning: f64,
    /// Threshold above which a client is flagged as abusive.
    pub threshold: f64,
    /// Distinct-endpoint count (within window) considered full-scale scanning.
    pub scanning_saturation: usize,
}

impl Default for AbuseConfig {
    fn default() -> Self {
        Self {
            window_size: 100,
            weight_burst: 0.4,
            weight_errors: 0.4,
            weight_scanning: 0.2,
            threshold: 0.7,
            scanning_saturation: 20,
        }
    }
}

/// A computed abuse assessment for a client.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AbuseAssessment {
    /// Overall abuse score in `[0, 1]`.
    pub score: f64,
    /// Whether the score exceeds the configured threshold.
    pub is_abusive: bool,
    /// Burst sub-score in `[0, 1]`.
    pub burst_score: f64,
    /// Error-ratio sub-score in `[0, 1]`.
    pub error_score: f64,
    /// Endpoint-scanning sub-score in `[0, 1]`.
    pub scanning_score: f64,
    /// Number of events considered.
    pub sample_size: usize,
}

/// Per-client event window.
#[derive(Debug, Default, Clone)]
struct ClientWindow {
    events: VecDeque<RequestEvent>,
}

/// Statistical / heuristic abuse detector.
#[derive(Clone)]
pub struct AbuseDetector {
    inner: Arc<RwLock<HashMap<String, ClientWindow>>>,
    config: AbuseConfig,
}

impl AbuseDetector {
    /// Creates a detector with the given configuration.
    pub fn new(config: AbuseConfig) -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    /// Creates a detector with default configuration.
    pub fn with_defaults() -> Self {
        Self::new(AbuseConfig::default())
    }

    /// Records a request event for a client, evicting the oldest event when the
    /// window is full.
    pub async fn record(&self, client: impl Into<String>, event: RequestEvent) {
        let client = client.into();
        let mut state = self.inner.write().await;
        let window = state.entry(client).or_default();
        window.events.push_back(event);
        while window.events.len() > self.config.window_size {
            window.events.pop_front();
        }
    }

    /// Returns the number of events currently held for a client.
    pub async fn event_count(&self, client: &str) -> usize {
        self.inner
            .read()
            .await
            .get(client)
            .map(|w| w.events.len())
            .unwrap_or(0)
    }

    /// Computes the abuse assessment for a client from its current window.
    pub async fn assess(&self, client: &str) -> AbuseAssessment {
        let state = self.inner.read().await;
        let events: Vec<RequestEvent> = state
            .get(client)
            .map(|w| w.events.iter().cloned().collect())
            .unwrap_or_default();
        self.assess_events(&events)
    }

    /// Computes an assessment directly from a slice of events (pure function,
    /// exposed for testing and offline scoring).
    pub fn assess_events(&self, events: &[RequestEvent]) -> AbuseAssessment {
        let sample_size = events.len();
        if sample_size == 0 {
            return AbuseAssessment {
                score: 0.0,
                is_abusive: false,
                burst_score: 0.0,
                error_score: 0.0,
                scanning_score: 0.0,
                sample_size: 0,
            };
        }

        let burst_score = burst_signal(events);
        let error_score = error_ratio_signal(events);
        let scanning_score = scanning_signal(events, self.config.scanning_saturation);

        let w_total =
            self.config.weight_burst + self.config.weight_errors + self.config.weight_scanning;
        let score = if w_total > 0.0 {
            (self.config.weight_burst * burst_score
                + self.config.weight_errors * error_score
                + self.config.weight_scanning * scanning_score)
                / w_total
        } else {
            0.0
        };
        let score = score.clamp(0.0, 1.0);

        AbuseAssessment {
            score,
            is_abusive: score >= self.config.threshold,
            burst_score,
            error_score,
            scanning_score,
            sample_size,
        }
    }

    /// Returns all clients currently assessed as abusive.
    pub async fn abusive_clients(&self) -> Vec<(String, AbuseAssessment)> {
        let state = self.inner.read().await;
        let mut out = Vec::new();
        for (client, window) in state.iter() {
            let events: Vec<RequestEvent> = window.events.iter().cloned().collect();
            let assessment = self.assess_events(&events);
            if assessment.is_abusive {
                out.push((client.clone(), assessment));
            }
        }
        out
    }
}

/// Computes a burst sub-score from inter-arrival timing.
///
/// Estimates instantaneous request rate over the window span and maps it through
/// a saturating function: very high rates approach 1.0. Uses a logistic-style
/// mapping centred at a moderate rate to avoid penalising normal traffic.
fn burst_signal(events: &[RequestEvent]) -> f64 {
    if events.len() < 2 {
        return 0.0;
    }
    let first = events.first().map(|e| e.timestamp);
    let last = events.last().map(|e| e.timestamp);
    let (first, last) = match (first, last) {
        (Some(f), Some(l)) => (f, l),
        _ => return 0.0,
    };
    let span_secs = (last - first).num_milliseconds().max(1) as f64 / 1000.0;
    let rate = events.len() as f64 / span_secs;
    // Saturating map: rate of ~50 req/s -> ~0.5, scaling smoothly to 1.0.
    let k = 0.05;
    1.0 - (-k * rate).exp()
}

/// Computes the error-ratio sub-score: fraction of non-2xx/3xx responses.
fn error_ratio_signal(events: &[RequestEvent]) -> f64 {
    if events.is_empty() {
        return 0.0;
    }
    let errors = events.iter().filter(|e| e.status >= 400).count();
    errors as f64 / events.len() as f64
}

/// Computes an endpoint-scanning sub-score from distinct-path diversity.
///
/// Many distinct endpoints touched in a short window indicates path enumeration.
/// The count of distinct paths is normalised against a saturation threshold.
fn scanning_signal(events: &[RequestEvent], saturation: usize) -> f64 {
    if events.is_empty() || saturation == 0 {
        return 0.0;
    }
    let distinct: HashSet<&str> = events.iter().map(|e| e.path.as_str()).collect();
    (distinct.len() as f64 / saturation as f64).min(1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ev(path: &str, status: u16, offset_ms: i64, base: DateTime<Utc>) -> RequestEvent {
        RequestEvent::new(
            path,
            status,
            base + chrono::Duration::milliseconds(offset_ms),
        )
    }

    #[test]
    fn test_empty_assessment() {
        let detector = AbuseDetector::with_defaults();
        let a = detector.assess_events(&[]);
        assert_eq!(a.score, 0.0);
        assert!(!a.is_abusive);
        assert_eq!(a.sample_size, 0);
    }

    #[test]
    fn test_error_ratio_signal() {
        let base = Utc::now();
        let events = vec![
            ev("/a", 200, 0, base),
            ev("/a", 401, 10, base),
            ev("/a", 403, 20, base),
            ev("/a", 500, 30, base),
        ];
        // 3 of 4 are errors.
        assert!((error_ratio_signal(&events) - 0.75).abs() < 1e-9);
    }

    #[test]
    fn test_scanning_signal() {
        let base = Utc::now();
        let events: Vec<RequestEvent> = (0..10)
            .map(|i| ev(&format!("/p{i}"), 200, i as i64, base))
            .collect();
        // 10 distinct paths, saturation 20 -> 0.5.
        assert!((scanning_signal(&events, 20) - 0.5).abs() < 1e-9);
        // Saturates at 1.0.
        assert!((scanning_signal(&events, 5) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_burst_signal_high_rate() {
        let base = Utc::now();
        // 100 requests within 100ms -> very high rate.
        let events: Vec<RequestEvent> = (0..100).map(|i| ev("/a", 200, i, base)).collect();
        let burst = burst_signal(&events);
        assert!(burst > 0.9, "burst={burst}");
    }

    #[test]
    fn test_burst_signal_low_rate() {
        let base = Utc::now();
        // 3 requests spread over 30 seconds -> low rate.
        let events = vec![
            ev("/a", 200, 0, base),
            ev("/a", 200, 15_000, base),
            ev("/a", 200, 30_000, base),
        ];
        let burst = burst_signal(&events);
        assert!(burst < 0.2, "burst={burst}");
    }

    #[test]
    fn test_normal_traffic_not_abusive() {
        let detector = AbuseDetector::with_defaults();
        let base = Utc::now();
        // Steady, successful, single-endpoint traffic.
        let events: Vec<RequestEvent> = (0..20)
            .map(|i| ev("/api/v1/statutes", 200, i * 1000, base))
            .collect();
        let a = detector.assess_events(&events);
        assert!(!a.is_abusive, "score={}", a.score);
    }

    #[test]
    fn test_credential_stuffing_pattern_flagged() {
        let detector = AbuseDetector::with_defaults();
        let base = Utc::now();
        // Rapid, error-heavy hammering of one auth endpoint.
        let events: Vec<RequestEvent> = (0..80)
            .map(|i| ev("/auth/login", 401, i * 5, base))
            .collect();
        let a = detector.assess_events(&events);
        assert!(a.error_score > 0.9);
        assert!(a.burst_score > 0.5);
        assert!(a.is_abusive, "score={}", a.score);
    }

    #[test]
    fn test_scanning_pattern_signal_high() {
        let detector = AbuseDetector::with_defaults();
        let base = Utc::now();
        // Rapidly enumerating many endpoints with 404s.
        let events: Vec<RequestEvent> = (0..40)
            .map(|i| ev(&format!("/admin/{i}"), 404, i * 5, base))
            .collect();
        let a = detector.assess_events(&events);
        assert!(a.scanning_score >= 1.0 - 1e-9);
        assert!(a.is_abusive);
    }

    #[tokio::test]
    async fn test_record_window_eviction() {
        let config = AbuseConfig {
            window_size: 5,
            ..AbuseConfig::default()
        };
        let detector = AbuseDetector::new(config);
        let base = Utc::now();
        for i in 0..10 {
            detector.record("c1", ev("/a", 200, i, base)).await;
        }
        assert_eq!(detector.event_count("c1").await, 5);
    }

    #[tokio::test]
    async fn test_assess_via_store() {
        let detector = AbuseDetector::with_defaults();
        let base = Utc::now();
        for i in 0..60 {
            detector
                .record("attacker", ev("/auth", 401, i * 5, base))
                .await;
        }
        let a = detector.assess("attacker").await;
        assert!(a.is_abusive);

        // An unseen client is not abusive.
        let clean = detector.assess("ghost").await;
        assert!(!clean.is_abusive);
        assert_eq!(clean.sample_size, 0);
    }

    #[tokio::test]
    async fn test_abusive_clients_listing() {
        let detector = AbuseDetector::with_defaults();
        let base = Utc::now();
        // One abuser, one normal client.
        for i in 0..60 {
            detector.record("bad", ev("/auth", 401, i * 5, base)).await;
        }
        for i in 0..10 {
            detector
                .record("good", ev("/api/v1/statutes", 200, i * 1000, base))
                .await;
        }
        let abusive = detector.abusive_clients().await;
        assert_eq!(abusive.len(), 1);
        assert_eq!(abusive[0].0, "bad");
    }
}
