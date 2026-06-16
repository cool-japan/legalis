//! Cost-per-query tracking.
//!
//! Builds on the crate's [`crate::TokenUsage`] and [`crate::CostEstimator`] to
//! answer the production question "what does a single query cost us?" - mean,
//! median and tail (p95/p99) cost per query, broken down by category, provider
//! and model, plus a bucketed cost trend over time. The stateful
//! [`CostPerQueryTracker`] mirrors the async-collector pattern used elsewhere in
//! the crate and reuses the existing cost estimators rather than redefining
//! pricing.

use super::{ResponseObservation, mean, median, percentile, truncate_to_bucket};
use crate::{CostEstimator, TokenUsage};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Aggregated cost-per-query statistics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CostPerQueryStats {
    /// Number of queries considered.
    pub total_queries: usize,
    /// Total cost across all queries (USD).
    pub total_cost_usd: f64,
    /// Mean cost per query (USD).
    pub mean_cost: f64,
    /// Median cost per query (USD).
    pub median_cost: f64,
    /// 95th-percentile cost per query (USD).
    pub p95_cost: f64,
    /// 99th-percentile cost per query (USD).
    pub p99_cost: f64,
    /// Minimum cost per query (USD).
    pub min_cost: f64,
    /// Maximum cost per query (USD).
    pub max_cost: f64,
    /// Mean tokens per query.
    pub mean_tokens: f64,
    /// Mean cost per 1k tokens across all queries (USD).
    pub mean_cost_per_1k_tokens: f64,
    /// Total cost grouped by request category.
    pub cost_by_category: BTreeMap<String, f64>,
    /// Total cost grouped by provider.
    pub cost_by_provider: BTreeMap<String, f64>,
    /// Total cost grouped by model.
    pub cost_by_model: BTreeMap<String, f64>,
    /// Query counts grouped by request category.
    pub queries_by_category: BTreeMap<String, usize>,
}

impl CostPerQueryStats {
    /// Returns the mean cost per query for a specific category, if present.
    pub fn category_mean_cost(&self, category: &str) -> Option<f64> {
        let cost = self.cost_by_category.get(category)?;
        let count = self.queries_by_category.get(category)?;
        if *count == 0 {
            None
        } else {
            Some(cost / *count as f64)
        }
    }

    /// Returns the most expensive category by total cost.
    pub fn top_category(&self) -> Option<(&String, &f64)> {
        self.cost_by_category
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
    }
}

/// Time-bucket granularity for cost trends.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrendGranularity {
    /// One bucket per hour.
    Hourly,
    /// One bucket per calendar day.
    Daily,
}

/// One point in a cost-over-time trend.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CostTrendPoint {
    /// Start of the time bucket (UTC).
    pub bucket_start: DateTime<Utc>,
    /// Number of queries in the bucket.
    pub query_count: usize,
    /// Total cost in the bucket (USD).
    pub total_cost: f64,
    /// Mean cost per query in the bucket (USD).
    pub mean_cost_per_query: f64,
}

/// Computes cost-per-query statistics over a batch of observations.
///
/// Every observation contributes its recorded [`ResponseObservation::cost_usd`]
/// (treated as `0.0` when unknown); a query's cost is counted whether or not the
/// request ultimately succeeded, matching how providers bill.
pub fn cost_per_query_stats(observations: &[ResponseObservation]) -> CostPerQueryStats {
    let mut stats = CostPerQueryStats {
        total_queries: observations.len(),
        total_cost_usd: 0.0,
        mean_cost: 0.0,
        median_cost: 0.0,
        p95_cost: 0.0,
        p99_cost: 0.0,
        min_cost: 0.0,
        max_cost: 0.0,
        mean_tokens: 0.0,
        mean_cost_per_1k_tokens: 0.0,
        cost_by_category: BTreeMap::new(),
        cost_by_provider: BTreeMap::new(),
        cost_by_model: BTreeMap::new(),
        queries_by_category: BTreeMap::new(),
    };
    if observations.is_empty() {
        return stats;
    }

    let costs: Vec<f64> = observations.iter().map(|obs| obs.cost_or_zero()).collect();
    let total_tokens: usize = observations.iter().map(|obs| obs.total_tokens()).sum();

    stats.total_cost_usd = costs.iter().sum();
    stats.mean_cost = mean(&costs);
    stats.median_cost = median(&costs);
    stats.p95_cost = percentile(&costs, 95.0);
    stats.p99_cost = percentile(&costs, 99.0);
    stats.min_cost = costs
        .iter()
        .copied()
        .fold(f64::INFINITY, f64::min)
        .min(stats.total_cost_usd);
    stats.max_cost = costs.iter().copied().fold(0.0_f64, f64::max);
    stats.mean_tokens = total_tokens as f64 / observations.len() as f64;
    stats.mean_cost_per_1k_tokens = if total_tokens > 0 {
        stats.total_cost_usd / (total_tokens as f64 / 1000.0)
    } else {
        0.0
    };

    for obs in observations {
        let cost = obs.cost_or_zero();
        *stats
            .cost_by_provider
            .entry(obs.provider.clone())
            .or_insert(0.0) += cost;
        *stats.cost_by_model.entry(obs.model.clone()).or_insert(0.0) += cost;
        if let Some(category) = &obs.category {
            *stats
                .cost_by_category
                .entry(category.clone())
                .or_insert(0.0) += cost;
            *stats
                .queries_by_category
                .entry(category.clone())
                .or_insert(0) += 1;
        }
    }

    stats
}

/// Computes a cost trend bucketed at the given granularity (sorted by time).
pub fn cost_trend(
    observations: &[ResponseObservation],
    granularity: TrendGranularity,
) -> Vec<CostTrendPoint> {
    let mut buckets: BTreeMap<DateTime<Utc>, (usize, f64)> = BTreeMap::new();
    for obs in observations {
        let key = truncate_to_bucket(obs.timestamp, granularity);
        let entry = buckets.entry(key).or_insert((0, 0.0));
        entry.0 += 1;
        entry.1 += obs.cost_or_zero();
    }
    buckets
        .into_iter()
        .map(|(bucket_start, (count, total))| CostTrendPoint {
            bucket_start,
            query_count: count,
            total_cost: total,
            mean_cost_per_query: if count > 0 { total / count as f64 } else { 0.0 },
        })
        .collect()
}

/// A stateful cost-per-query tracker with built-in pricing.
///
/// Reuses the crate's [`CostEstimator`] table to translate token usage into a
/// per-query cost, then aggregates the resulting observations. It can also be
/// fed pre-priced [`ResponseObservation`]s directly.
pub struct CostPerQueryTracker {
    observations: Arc<RwLock<Vec<ResponseObservation>>>,
    estimators: HashMap<String, CostEstimator>,
}

impl CostPerQueryTracker {
    /// Creates a tracker pre-populated with common model estimators.
    pub fn new() -> Self {
        let mut estimators = HashMap::new();
        estimators.insert("gpt-4".to_string(), CostEstimator::openai_gpt4());
        estimators.insert(
            "gpt-3.5-turbo".to_string(),
            CostEstimator::openai_gpt35_turbo(),
        );
        estimators.insert(
            "claude-3-opus".to_string(),
            CostEstimator::anthropic_claude3_opus(),
        );
        estimators.insert(
            "claude-3-sonnet".to_string(),
            CostEstimator::anthropic_claude3_sonnet(),
        );
        estimators.insert(
            "claude-3-haiku".to_string(),
            CostEstimator::anthropic_claude3_haiku(),
        );
        estimators.insert("gemini-pro".to_string(), CostEstimator::gemini_pro());
        Self {
            observations: Arc::new(RwLock::new(Vec::new())),
            estimators,
        }
    }

    /// Registers (or overrides) a cost estimator for a model.
    pub fn register_estimator(&mut self, model: impl Into<String>, estimator: CostEstimator) {
        self.estimators.insert(model.into(), estimator);
    }

    /// Estimates the cost of a usage record for a model, if pricing is known.
    pub fn estimate_cost(&self, model: &str, usage: &TokenUsage) -> Option<f64> {
        self.estimators
            .get(model)
            .map(|estimator| estimator.estimate_cost(usage))
    }

    /// Records a fully-formed observation as-is.
    pub async fn record(&self, observation: ResponseObservation) {
        self.observations.write().await.push(observation);
    }

    /// Records a query from its usage, pricing it via the registered estimators.
    ///
    /// Returns the priced [`ResponseObservation`] that was stored so the caller
    /// can inspect or further annotate the computed cost.
    pub async fn record_usage(
        &self,
        provider: impl Into<String>,
        model: impl Into<String>,
        usage: TokenUsage,
        latency_ms: u64,
        category: Option<String>,
    ) -> ResponseObservation {
        let model = model.into();
        let cost = self.estimate_cost(&model, &usage).unwrap_or(0.0);
        let mut observation = ResponseObservation::new(provider, model)
            .with_latency(latency_ms)
            .with_usage(usage)
            .with_cost(cost);
        if let Some(category) = category {
            observation = observation.with_category(category);
        }
        self.observations.write().await.push(observation.clone());
        observation
    }

    /// Computes cost-per-query statistics over everything recorded so far.
    pub async fn compute(&self) -> CostPerQueryStats {
        let observations = self.observations.read().await;
        cost_per_query_stats(&observations)
    }

    /// Computes a cost trend over everything recorded so far.
    pub async fn trend(&self, granularity: TrendGranularity) -> Vec<CostTrendPoint> {
        let observations = self.observations.read().await;
        cost_trend(&observations, granularity)
    }

    /// Returns the number of recorded queries.
    pub async fn len(&self) -> usize {
        self.observations.read().await.len()
    }

    /// Returns whether no queries have been recorded.
    pub async fn is_empty(&self) -> bool {
        self.observations.read().await.is_empty()
    }
}

impl Default for CostPerQueryTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn priced(category: &str, cost: f64, tokens: usize) -> ResponseObservation {
        ResponseObservation::new("openai", "gpt-4")
            .with_category(category)
            .with_cost(cost)
            .with_usage(TokenUsage::new(tokens / 2, tokens / 2))
    }

    #[test]
    fn test_cost_per_query_stats() {
        let observations = vec![
            priced("analysis", 0.02, 1000),
            priced("analysis", 0.04, 2000),
            priced("summary", 0.06, 3000),
        ];
        let stats = cost_per_query_stats(&observations);
        assert_eq!(stats.total_queries, 3);
        assert!((stats.total_cost_usd - 0.12).abs() < 1e-9);
        assert!((stats.mean_cost - 0.04).abs() < 1e-9);
        assert!((stats.median_cost - 0.04).abs() < 1e-9);
        assert!((stats.max_cost - 0.06).abs() < 1e-9);
        assert!((stats.min_cost - 0.02).abs() < 1e-9);
        assert!(
            (stats
                .cost_by_category
                .get("analysis")
                .copied()
                .unwrap_or(0.0)
                - 0.06)
                .abs()
                < 1e-9
        );
        assert_eq!(stats.queries_by_category.get("analysis"), Some(&2));
    }

    #[test]
    fn test_category_mean_and_top() {
        let observations = vec![
            priced("a", 0.10, 1000),
            priced("a", 0.30, 1000),
            priced("b", 0.05, 1000),
        ];
        let stats = cost_per_query_stats(&observations);
        assert!((stats.category_mean_cost("a").unwrap_or(0.0) - 0.20).abs() < 1e-9);
        let (top, _) = stats.top_category().expect("has top");
        assert_eq!(top, "a");
    }

    #[test]
    fn test_cost_per_1k_tokens() {
        let observations = vec![priced("x", 0.10, 1000), priced("x", 0.10, 1000)];
        let stats = cost_per_query_stats(&observations);
        // total cost 0.20 over 2000 tokens => 0.10 per 1k.
        assert!((stats.mean_cost_per_1k_tokens - 0.10).abs() < 1e-9);
        assert!((stats.mean_tokens - 1000.0).abs() < 1e-9);
    }

    #[test]
    fn test_empty_stats() {
        let stats = cost_per_query_stats(&[]);
        assert_eq!(stats.total_queries, 0);
        assert_eq!(stats.total_cost_usd, 0.0);
        assert!(stats.top_category().is_none());
    }

    #[test]
    fn test_cost_trend_buckets() {
        let base = Utc::now();
        let observations = vec![
            priced("a", 0.01, 100).with_timestamp(base),
            priced("a", 0.02, 100).with_timestamp(base + chrono::Duration::minutes(10)),
            priced("a", 0.03, 100).with_timestamp(base + chrono::Duration::days(1)),
        ];
        let trend = cost_trend(&observations, TrendGranularity::Daily);
        assert_eq!(trend.len(), 2);
        assert_eq!(trend[0].query_count, 2);
        assert!((trend[0].total_cost - 0.03).abs() < 1e-9);
        assert_eq!(trend[1].query_count, 1);
    }

    #[tokio::test]
    async fn test_tracker_record_usage_prices_via_estimator() {
        let tracker = CostPerQueryTracker::new();
        // gpt-4: 0.03/1k prompt, 0.06/1k completion => 1000+500 = 0.03 + 0.03 = 0.06.
        let obs = tracker
            .record_usage(
                "openai",
                "gpt-4",
                TokenUsage::new(1000, 500),
                900,
                Some("analysis".to_string()),
            )
            .await;
        assert!((obs.cost_or_zero() - 0.06).abs() < 1e-6);
        assert_eq!(tracker.len().await, 1);
        assert!(!tracker.is_empty().await);

        let stats = tracker.compute().await;
        assert_eq!(stats.total_queries, 1);
        assert!((stats.mean_cost - 0.06).abs() < 1e-6);
    }

    #[tokio::test]
    async fn test_tracker_custom_estimator_and_trend() {
        let mut tracker = CostPerQueryTracker::new();
        tracker.register_estimator("my-model", CostEstimator::new(1.0, 1.0));
        assert!(
            (tracker
                .estimate_cost("my-model", &TokenUsage::new(1000, 1000))
                .unwrap_or(0.0)
                - 2.0)
                .abs()
                < 1e-9
        );

        tracker
            .record_usage("custom", "my-model", TokenUsage::new(1000, 0), 100, None)
            .await;
        let trend = tracker.trend(TrendGranularity::Hourly).await;
        assert_eq!(trend.len(), 1);
        assert_eq!(trend[0].query_count, 1);
    }
}
