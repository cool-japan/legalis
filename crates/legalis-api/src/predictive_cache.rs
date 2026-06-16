//! Predictive caching driven by access patterns.
//!
//! Learns sequential access patterns over resource keys and predicts which keys
//! are likely to be requested next, so they can be prefetched / warmed before
//! they are asked for. The predictor maintains:
//!
//! - A first-order Markov transition model: counts of `key_a -> key_b`
//!   transitions observed in the access stream, used to rank likely successors.
//! - Recency/frequency statistics per key (hit counts) used to seed cache
//!   warming for cold starts.
//!
//! The model is purely statistical and dependency-free. It exposes the top-N
//! predicted successors for a key and a combined warming set that blends
//! transition likelihood with overall popularity.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// A predicted next key with an associated confidence score in `[0, 1]`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Prediction {
    /// The predicted key.
    pub key: String,
    /// Confidence (transition probability) in `[0, 1]`.
    pub confidence: f64,
}

/// Aggregated statistics for diagnostics.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PredictorStats {
    /// Distinct keys observed.
    pub distinct_keys: usize,
    /// Total accesses recorded.
    pub total_accesses: u64,
    /// Distinct transitions observed.
    pub distinct_transitions: usize,
}

/// Per-key successor transition counts.
#[derive(Debug, Default, Clone)]
struct TransitionCounts {
    /// successor key -> count.
    successors: HashMap<String, u64>,
    /// total transitions out of this key.
    total: u64,
}

/// Access-pattern predictor / cache-warming planner.
#[derive(Clone, Default)]
pub struct PredictiveCache {
    inner: Arc<RwLock<PredictorState>>,
}

#[derive(Default)]
struct PredictorState {
    /// First-order Markov transitions: key -> successor counts.
    transitions: HashMap<String, TransitionCounts>,
    /// Overall access frequency per key.
    frequency: HashMap<String, u64>,
    /// The last key observed (to form the next transition).
    last_key: Option<String>,
    /// Total accesses recorded.
    total_accesses: u64,
}

impl PredictiveCache {
    /// Creates an empty predictor.
    pub fn new() -> Self {
        Self::default()
    }

    /// Records an access to `key`, updating frequency and the transition from the
    /// previously observed key (if any).
    pub async fn record_access(&self, key: impl Into<String>) {
        let key = key.into();
        let mut state = self.inner.write().await;
        state.total_accesses += 1;
        *state.frequency.entry(key.clone()).or_insert(0) += 1;

        if let Some(prev) = state.last_key.take() {
            // Avoid counting immediate self-repeats as transitions.
            if prev != key {
                let entry = state.transitions.entry(prev).or_default();
                *entry.successors.entry(key.clone()).or_insert(0) += 1;
                entry.total += 1;
            }
        }
        state.last_key = Some(key);
    }

    /// Records a sequence of accesses in order.
    pub async fn record_sequence<I, S>(&self, keys: I)
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for key in keys {
            self.record_access(key).await;
        }
    }

    /// Returns the top-`n` predicted successors of `key`, ranked by transition
    /// probability (descending). Returns an empty vector for unseen keys.
    pub async fn predict_next(&self, key: &str, n: usize) -> Vec<Prediction> {
        let state = self.inner.read().await;
        let counts = match state.transitions.get(key) {
            Some(c) if c.total > 0 => c,
            _ => return Vec::new(),
        };
        let mut preds: Vec<Prediction> = counts
            .successors
            .iter()
            .map(|(succ, &count)| Prediction {
                key: succ.clone(),
                confidence: count as f64 / counts.total as f64,
            })
            .collect();
        preds.sort_by(|a, b| {
            b.confidence
                .partial_cmp(&a.confidence)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.key.cmp(&b.key))
        });
        preds.truncate(n);
        preds
    }

    /// Returns the `n` most popular keys overall, ranked by access frequency.
    pub async fn most_popular(&self, n: usize) -> Vec<Prediction> {
        let state = self.inner.read().await;
        if state.total_accesses == 0 {
            return Vec::new();
        }
        let mut items: Vec<Prediction> = state
            .frequency
            .iter()
            .map(|(key, &count)| Prediction {
                key: key.clone(),
                confidence: count as f64 / state.total_accesses as f64,
            })
            .collect();
        items.sort_by(|a, b| {
            b.confidence
                .partial_cmp(&a.confidence)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.key.cmp(&b.key))
        });
        items.truncate(n);
        items
    }

    /// Produces a cache-warming set for after `current_key` was accessed.
    ///
    /// Blends transition-based predictions (weight `transition_weight`) with
    /// global popularity (weight `1 - transition_weight`), returning up to `n`
    /// distinct keys ranked by the blended score. This handles both "hot path"
    /// prefetching and cold-start warming.
    pub async fn warming_set(
        &self,
        current_key: &str,
        n: usize,
        transition_weight: f64,
    ) -> Vec<Prediction> {
        let w = transition_weight.clamp(0.0, 1.0);
        // Gather a generous candidate pool from both signals.
        let pool = n.saturating_mul(4).max(n);
        let transitions = self.predict_next(current_key, pool).await;
        let popular = self.most_popular(pool).await;

        let mut blended: HashMap<String, f64> = HashMap::new();
        for p in transitions {
            *blended.entry(p.key).or_insert(0.0) += w * p.confidence;
        }
        for p in popular {
            // Don't re-warm the key we just served.
            if p.key == current_key {
                continue;
            }
            *blended.entry(p.key).or_insert(0.0) += (1.0 - w) * p.confidence;
        }
        blended.remove(current_key);

        let mut items: Vec<Prediction> = blended
            .into_iter()
            .map(|(key, confidence)| Prediction { key, confidence })
            .collect();
        items.sort_by(|a, b| {
            b.confidence
                .partial_cmp(&a.confidence)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.key.cmp(&b.key))
        });
        items.truncate(n);
        items
    }

    /// Returns aggregate predictor statistics.
    pub async fn stats(&self) -> PredictorStats {
        let state = self.inner.read().await;
        let distinct_transitions = state.transitions.values().map(|c| c.successors.len()).sum();
        PredictorStats {
            distinct_keys: state.frequency.len(),
            total_accesses: state.total_accesses,
            distinct_transitions,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_record_and_predict() {
        let cache = PredictiveCache::new();
        // A -> B observed three times, A -> C once.
        cache
            .record_sequence(["A", "B", "A", "B", "A", "B", "A", "C"])
            .await;
        let preds = cache.predict_next("A", 5).await;
        assert!(!preds.is_empty());
        // B should be the top successor of A.
        assert_eq!(preds[0].key, "B");
        assert!(preds[0].confidence > preds.get(1).map(|p| p.confidence).unwrap_or(0.0));
    }

    #[tokio::test]
    async fn test_predict_unseen_key() {
        let cache = PredictiveCache::new();
        cache.record_sequence(["A", "B"]).await;
        assert!(cache.predict_next("Z", 3).await.is_empty());
    }

    #[tokio::test]
    async fn test_confidence_sums_to_one() {
        let cache = PredictiveCache::new();
        cache.record_sequence(["X", "Y", "X", "Z"]).await;
        let preds = cache.predict_next("X", 10).await;
        let total: f64 = preds.iter().map(|p| p.confidence).sum();
        assert!((total - 1.0).abs() < 1e-9, "total={total}");
    }

    #[tokio::test]
    async fn test_self_repeat_not_counted_as_transition() {
        let cache = PredictiveCache::new();
        cache.record_sequence(["A", "A", "A", "B"]).await;
        // Only A -> B should be recorded (self-repeats skipped).
        let preds = cache.predict_next("A", 5).await;
        assert_eq!(preds.len(), 1);
        assert_eq!(preds[0].key, "B");
    }

    #[tokio::test]
    async fn test_most_popular() {
        let cache = PredictiveCache::new();
        cache.record_sequence(["A", "B", "A", "C", "A"]).await;
        let popular = cache.most_popular(2).await;
        assert_eq!(popular[0].key, "A");
        assert!(popular[0].confidence > popular[1].confidence);
    }

    #[tokio::test]
    async fn test_most_popular_empty() {
        let cache = PredictiveCache::new();
        assert!(cache.most_popular(3).await.is_empty());
    }

    #[tokio::test]
    async fn test_warming_set_blends() {
        let cache = PredictiveCache::new();
        // Strong A -> B transition plus generally popular D.
        cache
            .record_sequence(["A", "B", "A", "B", "D", "D", "D", "A", "B"])
            .await;
        // Pure transition weighting favours B.
        let transition_only = cache.warming_set("A", 3, 1.0).await;
        assert_eq!(transition_only[0].key, "B");

        // Pure popularity weighting should surface popular keys (excluding A).
        let popularity_only = cache.warming_set("A", 3, 0.0).await;
        assert!(popularity_only.iter().all(|p| p.key != "A"));
        assert!(!popularity_only.is_empty());
    }

    #[tokio::test]
    async fn test_warming_set_excludes_current() {
        let cache = PredictiveCache::new();
        cache.record_sequence(["A", "B", "A", "B"]).await;
        let set = cache.warming_set("A", 5, 0.5).await;
        assert!(set.iter().all(|p| p.key != "A"));
    }

    #[tokio::test]
    async fn test_stats() {
        let cache = PredictiveCache::new();
        cache.record_sequence(["A", "B", "C"]).await;
        let stats = cache.stats().await;
        assert_eq!(stats.distinct_keys, 3);
        assert_eq!(stats.total_accesses, 3);
        // Transitions: A->B, B->C.
        assert_eq!(stats.distinct_transitions, 2);
    }
}
