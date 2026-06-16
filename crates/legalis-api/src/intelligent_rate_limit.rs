//! Intelligent (adaptive) rate limiting.
//!
//! Implements a per-client token-bucket rate limiter whose effective capacity
//! and refill rate adapt dynamically to observed system load. Under healthy load
//! the limiter grants generous limits (up to a configured maximum); as a load
//! signal (e.g. error rate, latency, queue depth — normalised to `[0, 1]`) rises,
//! the limiter contracts limits toward a configured floor using a smooth
//! exponential mapping. A per-client reputation factor further rewards
//! well-behaved clients and throttles abusive ones.
//!
//! The algorithm is fully deterministic and dependency-free (no RNG, no external
//! services), making it suitable for in-process adaptive throttling.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Configuration for the adaptive limiter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveConfig {
    /// Maximum sustained requests per second per client at zero load.
    pub max_rate_per_sec: f64,
    /// Minimum requests per second per client at full load.
    pub min_rate_per_sec: f64,
    /// Bucket capacity as a multiple of the effective per-second rate (burst).
    pub burst_multiplier: f64,
    /// Sensitivity of contraction to the load signal (higher = sharper cuts).
    pub load_sensitivity: f64,
}

impl Default for AdaptiveConfig {
    fn default() -> Self {
        Self {
            max_rate_per_sec: 50.0,
            min_rate_per_sec: 1.0,
            burst_multiplier: 2.0,
            load_sensitivity: 3.0,
        }
    }
}

impl AdaptiveConfig {
    /// Creates a config, clamping to sane bounds.
    pub fn new(max_rate_per_sec: f64, min_rate_per_sec: f64) -> Self {
        let max = max_rate_per_sec.max(0.001);
        let min = min_rate_per_sec.clamp(0.001, max);
        Self {
            max_rate_per_sec: max,
            min_rate_per_sec: min,
            burst_multiplier: 2.0,
            load_sensitivity: 3.0,
        }
    }

    /// Computes the effective per-second rate for a given load signal in `[0, 1]`
    /// and a client reputation factor in `[0, 1]` (1 = best reputation).
    ///
    /// The rate interpolates exponentially from `max` (load 0) toward `min`
    /// (load 1); reputation scales the result so trusted clients keep a larger
    /// share of capacity under contention.
    pub fn effective_rate(&self, load: f64, reputation: f64) -> f64 {
        let load = load.clamp(0.0, 1.0);
        let reputation = reputation.clamp(0.0, 1.0);
        // Exponential decay factor in [0, 1]: 1 at load 0, -> small at load 1.
        let decay = (-self.load_sensitivity * load).exp();
        let span = self.max_rate_per_sec - self.min_rate_per_sec;
        let base = self.min_rate_per_sec + span * decay;
        // Reputation scales between 50% and 100% of the load-adjusted base so
        // even low-reputation clients keep some allowance.
        let reputation_scale = 0.5 + 0.5 * reputation;
        (base * reputation_scale).max(self.min_rate_per_sec * 0.5)
    }
}

/// Per-client token bucket with adaptive parameters.
#[derive(Debug, Clone)]
struct ClientBucket {
    /// Current token count.
    tokens: f64,
    /// Last time tokens were refilled.
    last_refill: DateTime<Utc>,
    /// Reputation factor in `[0, 1]`.
    reputation: f64,
    /// Count of recent rejections (for reputation decay).
    recent_rejections: u32,
}

impl ClientBucket {
    fn new(now: DateTime<Utc>, initial_tokens: f64) -> Self {
        Self {
            tokens: initial_tokens,
            last_refill: now,
            reputation: 1.0,
            recent_rejections: 0,
        }
    }
}

/// The result of an admission check.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdmissionDecision {
    /// Whether the request is admitted.
    pub allowed: bool,
    /// Tokens remaining after the decision.
    pub remaining_tokens: f64,
    /// The effective per-second rate currently applied to the client.
    pub effective_rate: f64,
    /// Approximate seconds until the next token is available (when rejected).
    pub retry_after_secs: Option<f64>,
}

/// Adaptive, per-client rate limiter.
#[derive(Clone)]
pub struct IntelligentRateLimiter {
    inner: Arc<RwLock<LimiterState>>,
    config: AdaptiveConfig,
}

struct LimiterState {
    buckets: HashMap<String, ClientBucket>,
    /// Global load signal in `[0, 1]` (externally updated).
    load: f64,
}

impl IntelligentRateLimiter {
    /// Creates a limiter with the given configuration.
    pub fn new(config: AdaptiveConfig) -> Self {
        Self {
            inner: Arc::new(RwLock::new(LimiterState {
                buckets: HashMap::new(),
                load: 0.0,
            })),
            config,
        }
    }

    /// Creates a limiter with default configuration.
    pub fn with_defaults() -> Self {
        Self::new(AdaptiveConfig::default())
    }

    /// Updates the global load signal (clamped to `[0, 1]`).
    pub async fn set_load(&self, load: f64) {
        self.inner.write().await.load = load.clamp(0.0, 1.0);
    }

    /// Returns the current global load signal.
    pub async fn load(&self) -> f64 {
        self.inner.read().await.load
    }

    /// Returns the current reputation for a client (1.0 if unseen).
    pub async fn reputation(&self, client: &str) -> f64 {
        self.inner
            .read()
            .await
            .buckets
            .get(client)
            .map(|b| b.reputation)
            .unwrap_or(1.0)
    }

    /// Performs an admission check for `client` at `now`, consuming one token if
    /// allowed.
    pub async fn check(&self, client: &str, now: DateTime<Utc>) -> AdmissionDecision {
        let mut state = self.inner.write().await;
        let load = state.load;
        let config = self.config.clone();

        let bucket = state.buckets.entry(client.to_string()).or_insert_with(|| {
            let rate = config.effective_rate(load, 1.0);
            ClientBucket::new(now, rate * config.burst_multiplier)
        });

        let effective_rate = config.effective_rate(load, bucket.reputation);
        let capacity = (effective_rate * config.burst_multiplier).max(1.0);

        // Refill based on elapsed time at the effective rate.
        let elapsed = (now - bucket.last_refill).num_milliseconds().max(0) as f64 / 1000.0;
        bucket.tokens = (bucket.tokens + elapsed * effective_rate).min(capacity);
        bucket.last_refill = now;

        if bucket.tokens >= 1.0 {
            bucket.tokens -= 1.0;
            // Reward sustained good behaviour by recovering reputation slowly.
            if bucket.reputation < 1.0 {
                bucket.reputation = (bucket.reputation + 0.02).min(1.0);
            }
            bucket.recent_rejections = bucket.recent_rejections.saturating_sub(1);
            AdmissionDecision {
                allowed: true,
                remaining_tokens: bucket.tokens,
                effective_rate,
                retry_after_secs: None,
            }
        } else {
            // Penalise repeated rejections by lowering reputation.
            bucket.recent_rejections = bucket.recent_rejections.saturating_add(1);
            if bucket.recent_rejections >= 3 {
                bucket.reputation = (bucket.reputation - 0.1).max(0.1);
            }
            let deficit = 1.0 - bucket.tokens;
            let retry = if effective_rate > 0.0 {
                deficit / effective_rate
            } else {
                f64::INFINITY
            };
            AdmissionDecision {
                allowed: false,
                remaining_tokens: bucket.tokens,
                effective_rate,
                retry_after_secs: Some(retry),
            }
        }
    }

    /// Returns the number of tracked clients.
    pub async fn tracked_clients(&self) -> usize {
        self.inner.read().await.buckets.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_effective_rate_decreases_with_load() {
        let config = AdaptiveConfig::new(100.0, 1.0);
        let zero = config.effective_rate(0.0, 1.0);
        let half = config.effective_rate(0.5, 1.0);
        let full = config.effective_rate(1.0, 1.0);
        assert!(zero > half);
        assert!(half > full);
        // At zero load the rate is near the maximum.
        assert!(zero > 90.0);
    }

    #[test]
    fn test_reputation_scales_rate() {
        let config = AdaptiveConfig::new(100.0, 1.0);
        let good = config.effective_rate(0.5, 1.0);
        let bad = config.effective_rate(0.5, 0.0);
        assert!(good > bad);
    }

    #[test]
    fn test_effective_rate_clamps_inputs() {
        let config = AdaptiveConfig::new(10.0, 1.0);
        // Out-of-range load/reputation are clamped, not panicking.
        let r1 = config.effective_rate(5.0, 5.0);
        let r2 = config.effective_rate(-1.0, -1.0);
        assert!(r1.is_finite());
        assert!(r2.is_finite());
    }

    #[tokio::test]
    async fn test_admission_basic() {
        let limiter = IntelligentRateLimiter::new(AdaptiveConfig::new(10.0, 1.0));
        let now = Utc::now();
        let d = limiter.check("c1", now).await;
        assert!(d.allowed);
        assert!(d.remaining_tokens >= 0.0);
        assert_eq!(limiter.tracked_clients().await, 1);
    }

    #[tokio::test]
    async fn test_burst_then_throttle() {
        // Low rate so the bucket drains quickly.
        let limiter = IntelligentRateLimiter::new(AdaptiveConfig::new(2.0, 1.0));
        let now = Utc::now();
        // Burst capacity = rate * burst_multiplier = 2 * 2 = 4 tokens.
        let mut allowed = 0;
        for _ in 0..10 {
            if limiter.check("c1", now).await.allowed {
                allowed += 1;
            }
        }
        // Should admit only the burst capacity at the same instant.
        assert!((3..=5).contains(&allowed), "allowed={allowed}");
        // The next request at the same instant is rejected.
        let d = limiter.check("c1", now).await;
        assert!(!d.allowed);
        assert!(d.retry_after_secs.is_some());
    }

    #[tokio::test]
    async fn test_refill_over_time() {
        let limiter = IntelligentRateLimiter::new(AdaptiveConfig::new(4.0, 1.0));
        let now = Utc::now();
        // Drain the bucket.
        for _ in 0..20 {
            limiter.check("c1", now).await;
        }
        assert!(!limiter.check("c1", now).await.allowed);
        // After 1 second at rate ~4/s, tokens are replenished.
        let later = now + chrono::Duration::seconds(1);
        assert!(limiter.check("c1", later).await.allowed);
    }

    #[tokio::test]
    async fn test_load_reduces_capacity() {
        let limiter = IntelligentRateLimiter::new(AdaptiveConfig::new(20.0, 1.0));
        let now = Utc::now();
        // Under high load, the effective rate drops markedly.
        limiter.set_load(1.0).await;
        let d = limiter.check("c1", now).await;
        assert!(d.effective_rate < 20.0);
        assert!(limiter.load().await == 1.0);
    }

    #[tokio::test]
    async fn test_reputation_drops_on_repeated_rejection() {
        let limiter = IntelligentRateLimiter::new(AdaptiveConfig::new(1.0, 1.0));
        let now = Utc::now();
        // Hammer at the same instant to force rejections.
        for _ in 0..10 {
            limiter.check("abuser", now).await;
        }
        let rep = limiter.reputation("abuser").await;
        assert!(rep < 1.0, "reputation should drop, got {rep}");
    }

    #[tokio::test]
    async fn test_separate_clients_independent() {
        let limiter = IntelligentRateLimiter::new(AdaptiveConfig::new(1.0, 1.0));
        let now = Utc::now();
        // Drain c1.
        for _ in 0..10 {
            limiter.check("c1", now).await;
        }
        // c2 still has a fresh bucket.
        assert!(limiter.check("c2", now).await.allowed);
    }
}
