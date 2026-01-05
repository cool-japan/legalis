//! Request sampling for high-volume endpoints.
//!
//! This module provides intelligent request sampling to reduce overhead
//! on high-traffic endpoints while maintaining observability.

use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Sampling strategy
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SamplingStrategy {
    /// Always sample (100%)
    Always,
    /// Never sample (0%)
    Never,
    /// Random sampling with a fixed rate
    Random,
    /// Adaptive sampling based on load
    Adaptive,
    /// Head-based sampling (sample first N requests)
    Head,
    /// Tail-based sampling (sample based on outcome)
    Tail,
}

/// Sampling configuration for an endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SamplingConfig {
    /// Sampling strategy
    pub strategy: SamplingStrategy,
    /// Sampling rate (0.0 to 1.0)
    pub rate: f64,
    /// Maximum samples per second (for rate limiting)
    pub max_samples_per_second: Option<u64>,
    /// Sample errors at a higher rate
    pub error_boost: f64,
}

impl Default for SamplingConfig {
    fn default() -> Self {
        Self {
            strategy: SamplingStrategy::Random,
            rate: 0.1, // 10% sampling rate
            max_samples_per_second: None,
            error_boost: 1.0, // No boost by default
        }
    }
}

impl SamplingConfig {
    /// Creates a config for always sampling
    pub fn always() -> Self {
        Self {
            strategy: SamplingStrategy::Always,
            rate: 1.0,
            max_samples_per_second: None,
            error_boost: 1.0,
        }
    }

    /// Creates a config for never sampling
    pub fn never() -> Self {
        Self {
            strategy: SamplingStrategy::Never,
            rate: 0.0,
            max_samples_per_second: None,
            error_boost: 1.0,
        }
    }

    /// Creates a config with a specific sampling rate
    pub fn with_rate(rate: f64) -> Self {
        Self {
            strategy: SamplingStrategy::Random,
            rate: rate.clamp(0.0, 1.0),
            max_samples_per_second: None,
            error_boost: 1.0,
        }
    }

    /// Creates a config for high-volume endpoints
    pub fn high_volume() -> Self {
        Self {
            strategy: SamplingStrategy::Adaptive,
            rate: 0.01, // 1% sampling rate
            max_samples_per_second: Some(100),
            error_boost: 10.0, // Boost error sampling by 10x
        }
    }
}

/// Request sampling decision
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SamplingDecision {
    /// Sample this request
    Sample,
    /// Skip this request
    Skip,
}

/// Sampling statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SamplingStats {
    /// Total number of requests
    pub total_requests: u64,
    /// Number of sampled requests
    pub sampled_requests: u64,
    /// Number of skipped requests
    pub skipped_requests: u64,
    /// Current sampling rate
    pub current_rate: f64,
}

impl SamplingStats {
    /// Updates stats with a sampling decision
    pub fn record(&mut self, decision: SamplingDecision) {
        self.total_requests += 1;
        match decision {
            SamplingDecision::Sample => self.sampled_requests += 1,
            SamplingDecision::Skip => self.skipped_requests += 1,
        }
        if self.total_requests > 0 {
            self.current_rate = self.sampled_requests as f64 / self.total_requests as f64;
        }
    }

    /// Resets statistics
    pub fn reset(&mut self) {
        self.total_requests = 0;
        self.sampled_requests = 0;
        self.skipped_requests = 0;
        self.current_rate = 0.0;
    }
}

/// Request sampler
#[derive(Clone)]
pub struct RequestSampler {
    /// Sampling configurations per endpoint pattern
    configs: Arc<RwLock<HashMap<String, SamplingConfig>>>,
    /// Sampling statistics per endpoint
    stats: Arc<RwLock<HashMap<String, SamplingStats>>>,
    /// Default configuration
    default_config: SamplingConfig,
}

impl RequestSampler {
    /// Creates a new request sampler
    pub fn new() -> Self {
        Self {
            configs: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(HashMap::new())),
            default_config: SamplingConfig::default(),
        }
    }

    /// Creates a new request sampler with a default config
    pub fn with_default(default_config: SamplingConfig) -> Self {
        Self {
            configs: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(HashMap::new())),
            default_config,
        }
    }

    /// Sets sampling configuration for an endpoint
    pub async fn set_config(&self, endpoint: String, config: SamplingConfig) {
        let mut configs = self.configs.write().await;
        configs.insert(endpoint, config);
    }

    /// Gets sampling configuration for an endpoint
    async fn get_config(&self, endpoint: &str) -> SamplingConfig {
        let configs = self.configs.read().await;
        configs
            .get(endpoint)
            .cloned()
            .unwrap_or_else(|| self.default_config.clone())
    }

    /// Decides whether to sample a request
    pub async fn should_sample(&self, endpoint: &str, is_error: bool) -> SamplingDecision {
        let config = self.get_config(endpoint).await;

        let decision = match config.strategy {
            SamplingStrategy::Always => SamplingDecision::Sample,
            SamplingStrategy::Never => SamplingDecision::Skip,
            SamplingStrategy::Random => {
                let mut rate = config.rate;

                // Apply error boost
                if is_error {
                    rate = (rate * config.error_boost).min(1.0);
                }

                let mut rng = rand::rng();
                if rng.random::<f64>() < rate {
                    SamplingDecision::Sample
                } else {
                    SamplingDecision::Skip
                }
            }
            SamplingStrategy::Adaptive => {
                // For adaptive sampling, check current load
                let stats = self.get_stats(endpoint).await;
                let mut rate = config.rate;

                // If we're seeing high volume, reduce sampling rate
                if stats.total_requests > 1000 {
                    rate = (rate * 0.5).max(0.001); // At least 0.1%
                }

                // Apply error boost
                if is_error {
                    rate = (rate * config.error_boost).min(1.0);
                }

                let mut rng = rand::rng();
                if rng.random::<f64>() < rate {
                    SamplingDecision::Sample
                } else {
                    SamplingDecision::Skip
                }
            }
            SamplingStrategy::Head => {
                // Sample first N requests
                let stats = self.get_stats(endpoint).await;
                if stats.sampled_requests < 100 {
                    SamplingDecision::Sample
                } else {
                    SamplingDecision::Skip
                }
            }
            SamplingStrategy::Tail => {
                // For tail-based, we need to buffer and decide later
                // For now, use random sampling
                let mut rng = rand::rng();
                if rng.random::<f64>() < config.rate {
                    SamplingDecision::Sample
                } else {
                    SamplingDecision::Skip
                }
            }
        };

        // Record the decision
        self.record_decision(endpoint, decision).await;

        decision
    }

    /// Records a sampling decision
    async fn record_decision(&self, endpoint: &str, decision: SamplingDecision) {
        let mut stats = self.stats.write().await;
        let endpoint_stats = stats
            .entry(endpoint.to_string())
            .or_insert_with(SamplingStats::default);
        endpoint_stats.record(decision);
    }

    /// Gets sampling statistics for an endpoint
    pub async fn get_stats(&self, endpoint: &str) -> SamplingStats {
        let stats = self.stats.read().await;
        stats.get(endpoint).cloned().unwrap_or_default()
    }

    /// Gets all sampling statistics
    pub async fn get_all_stats(&self) -> HashMap<String, SamplingStats> {
        let stats = self.stats.read().await;
        stats.clone()
    }

    /// Resets statistics for an endpoint
    pub async fn reset_stats(&self, endpoint: &str) {
        let mut stats = self.stats.write().await;
        if let Some(endpoint_stats) = stats.get_mut(endpoint) {
            endpoint_stats.reset();
        }
    }

    /// Resets all statistics
    pub async fn reset_all_stats(&self) {
        let mut stats = self.stats.write().await;
        for endpoint_stats in stats.values_mut() {
            endpoint_stats.reset();
        }
    }
}

impl Default for RequestSampler {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper to extract endpoint pattern from a path
pub fn extract_endpoint_pattern(path: &str) -> String {
    // Replace IDs and dynamic segments with patterns
    let mut pattern = path.to_string();

    // Replace UUID-like patterns
    pattern = regex::Regex::new(r"[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}")
        .unwrap()
        .replace_all(&pattern, "{id}")
        .to_string();

    // Replace numeric IDs
    pattern = regex::Regex::new(r"/\d+")
        .unwrap()
        .replace_all(&pattern, "/{id}")
        .to_string();

    pattern
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sampling_config_always() {
        let config = SamplingConfig::always();
        assert_eq!(config.strategy, SamplingStrategy::Always);
        assert_eq!(config.rate, 1.0);
    }

    #[test]
    fn test_sampling_config_never() {
        let config = SamplingConfig::never();
        assert_eq!(config.strategy, SamplingStrategy::Never);
        assert_eq!(config.rate, 0.0);
    }

    #[test]
    fn test_sampling_config_high_volume() {
        let config = SamplingConfig::high_volume();
        assert_eq!(config.strategy, SamplingStrategy::Adaptive);
        assert_eq!(config.rate, 0.01);
        assert_eq!(config.error_boost, 10.0);
    }

    #[test]
    fn test_sampling_stats() {
        let mut stats = SamplingStats::default();

        stats.record(SamplingDecision::Sample);
        stats.record(SamplingDecision::Sample);
        stats.record(SamplingDecision::Skip);

        assert_eq!(stats.total_requests, 3);
        assert_eq!(stats.sampled_requests, 2);
        assert_eq!(stats.skipped_requests, 1);
        assert!((stats.current_rate - 0.666).abs() < 0.01);
    }

    #[tokio::test]
    async fn test_request_sampler_always() {
        let sampler = RequestSampler::with_default(SamplingConfig::always());

        let decision = sampler.should_sample("/api/test", false).await;
        assert_eq!(decision, SamplingDecision::Sample);
    }

    #[tokio::test]
    async fn test_request_sampler_never() {
        let sampler = RequestSampler::with_default(SamplingConfig::never());

        let decision = sampler.should_sample("/api/test", false).await;
        assert_eq!(decision, SamplingDecision::Skip);
    }

    #[tokio::test]
    async fn test_request_sampler_stats() {
        let sampler = RequestSampler::new();

        sampler.should_sample("/api/test", false).await;
        sampler.should_sample("/api/test", false).await;
        sampler.should_sample("/api/test", false).await;

        let stats = sampler.get_stats("/api/test").await;
        assert_eq!(stats.total_requests, 3);
    }

    #[test]
    fn test_extract_endpoint_pattern() {
        let pattern = extract_endpoint_pattern("/api/v1/statutes/123");
        assert_eq!(pattern, "/api/v1/statutes/{id}");

        let pattern =
            extract_endpoint_pattern("/api/v1/statutes/550e8400-e29b-41d4-a716-446655440000");
        assert_eq!(pattern, "/api/v1/statutes/{id}");
    }
}
