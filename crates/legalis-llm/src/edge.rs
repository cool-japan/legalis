//! Edge deployment support for resource-constrained environments.
//!
//! This module provides optimizations and utilities for running LLM inference
//! on edge devices with limited compute, memory, and network resources.

use crate::{LLMProvider, TextStream};
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;

/// Edge device profile describing resource constraints.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeProfile {
    /// Maximum memory available in bytes
    pub max_memory: u64,
    /// CPU cores available
    pub cpu_cores: usize,
    /// Whether GPU acceleration is available
    pub has_gpu: bool,
    /// Network bandwidth in bytes per second (0 if offline)
    pub network_bandwidth: u64,
    /// Whether the device is battery-powered
    pub battery_powered: bool,
    /// Target inference latency in milliseconds
    pub target_latency_ms: u128,
}

impl EdgeProfile {
    /// Creates a minimal edge profile for very constrained devices (e.g., Raspberry Pi 3).
    pub fn minimal() -> Self {
        Self {
            max_memory: 512 * 1024 * 1024, // 512 MB
            cpu_cores: 1,
            has_gpu: false,
            network_bandwidth: 0, // Offline
            battery_powered: true,
            target_latency_ms: 5000, // 5 seconds
        }
    }

    /// Creates a standard edge profile for typical edge devices (e.g., Raspberry Pi 4).
    pub fn standard() -> Self {
        Self {
            max_memory: 2 * 1024 * 1024 * 1024, // 2 GB
            cpu_cores: 4,
            has_gpu: false,
            network_bandwidth: 1_000_000, // 1 MB/s
            battery_powered: false,
            target_latency_ms: 2000, // 2 seconds
        }
    }

    /// Creates a powerful edge profile for high-end edge devices (e.g., NVIDIA Jetson).
    pub fn powerful() -> Self {
        Self {
            max_memory: 8 * 1024 * 1024 * 1024, // 8 GB
            cpu_cores: 8,
            has_gpu: true,
            network_bandwidth: 10_000_000, // 10 MB/s
            battery_powered: false,
            target_latency_ms: 500, // 500 ms
        }
    }

    /// Checks if the profile supports offline operation.
    pub fn is_offline(&self) -> bool {
        self.network_bandwidth == 0
    }

    /// Estimates whether a model can run on this profile.
    pub fn can_run_model(&self, model_size_bytes: u64, required_memory_bytes: u64) -> bool {
        // Check if model fits in memory with some overhead
        let total_required = model_size_bytes + required_memory_bytes;
        total_required <= (self.max_memory as f64 * 0.8) as u64
    }
}

/// Edge optimization strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OptimizationStrategy {
    /// Minimize memory usage
    MinimizeMemory,
    /// Minimize latency
    MinimizeLatency,
    /// Minimize power consumption
    MinimizePower,
    /// Balance all factors
    Balanced,
}

/// Edge inference configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeConfig {
    /// Device profile
    pub profile: EdgeProfile,
    /// Optimization strategy
    pub strategy: OptimizationStrategy,
    /// Enable request caching
    pub enable_caching: bool,
    /// Cache size in number of entries
    pub cache_size: usize,
    /// Enable response compression
    pub enable_compression: bool,
    /// Enable prompt truncation to fit memory
    pub enable_truncation: bool,
    /// Maximum prompt length in characters
    pub max_prompt_length: usize,
    /// Enable batching for offline processing
    pub enable_batching: bool,
    /// Batch size for offline processing
    pub batch_size: usize,
}

impl Default for EdgeConfig {
    fn default() -> Self {
        Self {
            profile: EdgeProfile::standard(),
            strategy: OptimizationStrategy::Balanced,
            enable_caching: true,
            cache_size: 100,
            enable_compression: true,
            enable_truncation: true,
            max_prompt_length: 2048,
            enable_batching: false,
            batch_size: 1,
        }
    }
}

impl EdgeConfig {
    /// Creates a configuration for minimal edge devices.
    pub fn minimal() -> Self {
        Self {
            profile: EdgeProfile::minimal(),
            strategy: OptimizationStrategy::MinimizeMemory,
            enable_caching: true,
            cache_size: 10,
            enable_compression: true,
            enable_truncation: true,
            max_prompt_length: 512,
            enable_batching: true,
            batch_size: 1,
        }
    }

    /// Creates a configuration optimized for low power.
    pub fn low_power() -> Self {
        Self {
            profile: EdgeProfile::standard(),
            strategy: OptimizationStrategy::MinimizePower,
            enable_caching: true,
            cache_size: 50,
            enable_compression: true,
            enable_truncation: true,
            max_prompt_length: 1024,
            enable_batching: true,
            batch_size: 5,
        }
    }
}

/// Simple LRU cache for edge inference results.
struct EdgeCache {
    cache: Arc<Mutex<lru::LruCache<String, String>>>,
    hits: Arc<Mutex<u64>>,
    misses: Arc<Mutex<u64>>,
}

impl EdgeCache {
    fn new(capacity: usize) -> Self {
        Self {
            cache: Arc::new(Mutex::new(lru::LruCache::new(
                std::num::NonZeroUsize::new(capacity).unwrap(),
            ))),
            hits: Arc::new(Mutex::new(0)),
            misses: Arc::new(Mutex::new(0)),
        }
    }

    async fn get(&self, key: &str) -> Option<String> {
        let mut cache = self.cache.lock().await;
        if let Some(value) = cache.get(key) {
            *self.hits.lock().await += 1;
            Some(value.clone())
        } else {
            *self.misses.lock().await += 1;
            None
        }
    }

    async fn put(&self, key: String, value: String) {
        let mut cache = self.cache.lock().await;
        cache.put(key, value);
    }

    #[allow(dead_code)]
    async fn stats(&self) -> (u64, u64) {
        let hits = *self.hits.lock().await;
        let misses = *self.misses.lock().await;
        (hits, misses)
    }

    #[allow(dead_code)]
    async fn hit_rate(&self) -> f64 {
        let (hits, misses) = self.stats().await;
        let total = hits + misses;
        if total == 0 {
            0.0
        } else {
            hits as f64 / total as f64
        }
    }
}

/// Edge-optimized LLM provider wrapper.
pub struct EdgeProvider<P> {
    provider: P,
    config: EdgeConfig,
    cache: Option<EdgeCache>,
    stats: Arc<Mutex<EdgeStats>>,
}

impl<P> EdgeProvider<P> {
    /// Creates a new edge provider.
    pub fn new(provider: P, config: EdgeConfig) -> Self {
        let cache = if config.enable_caching {
            Some(EdgeCache::new(config.cache_size))
        } else {
            None
        };

        Self {
            provider,
            config,
            cache,
            stats: Arc::new(Mutex::new(EdgeStats::default())),
        }
    }

    /// Gets the edge configuration.
    pub fn config(&self) -> &EdgeConfig {
        &self.config
    }

    /// Gets a reference to the underlying provider.
    pub fn provider(&self) -> &P {
        &self.provider
    }

    /// Returns edge inference statistics.
    pub async fn stats(&self) -> EdgeStats {
        self.stats.lock().await.clone()
    }

    /// Truncates a prompt to fit within memory constraints.
    fn truncate_prompt(&self, prompt: &str) -> String {
        if !self.config.enable_truncation {
            return prompt.to_string();
        }

        let max_len = self.config.max_prompt_length;
        if prompt.len() <= max_len {
            return prompt.to_string();
        }

        // Truncate with ellipsis
        let truncated = &prompt[..max_len.saturating_sub(3)];
        format!("{}...", truncated)
    }

    /// Compresses a response if compression is enabled.
    fn compress_response(&self, response: &str) -> String {
        if !self.config.enable_compression {
            return response.to_string();
        }

        // Simple whitespace compression
        response.split_whitespace().collect::<Vec<_>>().join(" ")
    }

    /// Creates a cache key from a prompt.
    fn cache_key(&self, prompt: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        prompt.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
}

#[async_trait]
impl<P: LLMProvider> LLMProvider for EdgeProvider<P> {
    async fn generate_text(&self, prompt: &str) -> Result<String> {
        let start = Instant::now();

        // Truncate prompt if needed
        let processed_prompt = self.truncate_prompt(prompt);

        // Check cache if enabled
        if let Some(cache) = &self.cache {
            let key = self.cache_key(&processed_prompt);
            if let Some(cached_response) = cache.get(&key).await {
                let mut stats = self.stats.lock().await;
                stats.total_requests += 1;
                stats.cache_hits += 1;
                stats.total_latency_ms += start.elapsed().as_millis();
                return Ok(cached_response);
            }
        }

        // Generate response
        let response = self.provider.generate_text(&processed_prompt).await?;

        // Compress response if enabled
        let processed_response = self.compress_response(&response);

        // Update cache
        if let Some(cache) = &self.cache {
            let key = self.cache_key(&processed_prompt);
            cache.put(key, processed_response.clone()).await;
        }

        // Update stats
        let mut stats = self.stats.lock().await;
        stats.total_requests += 1;
        stats.total_latency_ms += start.elapsed().as_millis();
        stats.bytes_processed += (processed_prompt.len() + processed_response.len()) as u64;

        Ok(processed_response)
    }

    async fn generate_structured<T: serde::de::DeserializeOwned + Send>(
        &self,
        prompt: &str,
    ) -> Result<T> {
        let start = Instant::now();
        let processed_prompt = self.truncate_prompt(prompt);
        let result = self
            .provider
            .generate_structured::<T>(&processed_prompt)
            .await?;

        let mut stats = self.stats.lock().await;
        stats.total_requests += 1;
        stats.total_latency_ms += start.elapsed().as_millis();

        Ok(result)
    }

    async fn generate_text_stream(&self, prompt: &str) -> Result<TextStream> {
        let processed_prompt = self.truncate_prompt(prompt);
        self.provider.generate_text_stream(&processed_prompt).await
    }

    fn provider_name(&self) -> &str {
        self.provider.provider_name()
    }

    fn model_name(&self) -> &str {
        self.provider.model_name()
    }

    fn supports_streaming(&self) -> bool {
        self.provider.supports_streaming()
    }
}

/// Edge inference statistics.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EdgeStats {
    /// Total number of requests
    pub total_requests: u64,
    /// Number of cache hits
    pub cache_hits: u64,
    /// Total latency in milliseconds
    pub total_latency_ms: u128,
    /// Total bytes processed
    pub bytes_processed: u64,
}

impl EdgeStats {
    /// Returns the cache hit rate (0.0 to 1.0).
    pub fn cache_hit_rate(&self) -> f64 {
        if self.total_requests == 0 {
            0.0
        } else {
            self.cache_hits as f64 / self.total_requests as f64
        }
    }

    /// Returns the average latency in milliseconds.
    pub fn avg_latency_ms(&self) -> f64 {
        if self.total_requests == 0 {
            0.0
        } else {
            self.total_latency_ms as f64 / self.total_requests as f64
        }
    }
}

/// Model deployment manifest for edge devices.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeManifest {
    /// Model name
    pub model_name: String,
    /// Model version
    pub version: String,
    /// Model size in bytes
    pub size_bytes: u64,
    /// Required memory in bytes
    pub required_memory_bytes: u64,
    /// Supported quantization formats
    pub quantization_formats: Vec<String>,
    /// Minimum profile required
    pub min_profile: EdgeProfile,
    /// Recommended batch size
    pub recommended_batch_size: usize,
    /// Model checksum (SHA-256)
    pub checksum: String,
}

impl EdgeManifest {
    /// Creates a new edge deployment manifest.
    pub fn new(model_name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            model_name: model_name.into(),
            version: version.into(),
            size_bytes: 0,
            required_memory_bytes: 0,
            quantization_formats: vec!["gguf".to_string(), "awq".to_string()],
            min_profile: EdgeProfile::standard(),
            recommended_batch_size: 1,
            checksum: String::new(),
        }
    }

    /// Checks if this model is compatible with a given edge profile.
    pub fn is_compatible(&self, profile: &EdgeProfile) -> bool {
        profile.can_run_model(self.size_bytes, self.required_memory_bytes)
            && profile.cpu_cores >= self.min_profile.cpu_cores
    }

    /// Exports the manifest as JSON.
    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string_pretty(self)
            .map_err(|e| anyhow!("Failed to serialize manifest: {}", e))
    }

    /// Loads a manifest from JSON.
    pub fn from_json(json: &str) -> Result<Self> {
        serde_json::from_str(json).map_err(|e| anyhow!("Failed to deserialize manifest: {}", e))
    }
}

/// Edge deployment manager.
pub struct EdgeDeployment {
    manifests: Arc<Mutex<Vec<EdgeManifest>>>,
    profile: EdgeProfile,
}

impl EdgeDeployment {
    /// Creates a new edge deployment manager.
    pub fn new(profile: EdgeProfile) -> Self {
        Self {
            manifests: Arc::new(Mutex::new(Vec::new())),
            profile,
        }
    }

    /// Registers a model manifest.
    pub async fn register_manifest(&self, manifest: EdgeManifest) -> Result<()> {
        if !manifest.is_compatible(&self.profile) {
            return Err(anyhow!(
                "Model {} is not compatible with current edge profile",
                manifest.model_name
            ));
        }

        let mut manifests = self.manifests.lock().await;
        manifests.push(manifest);
        Ok(())
    }

    /// Lists all registered models.
    pub async fn list_models(&self) -> Vec<String> {
        let manifests = self.manifests.lock().await;
        manifests.iter().map(|m| m.model_name.clone()).collect()
    }

    /// Gets a model manifest by name.
    pub async fn get_manifest(&self, model_name: &str) -> Option<EdgeManifest> {
        let manifests = self.manifests.lock().await;
        manifests
            .iter()
            .find(|m| m.model_name == model_name)
            .cloned()
    }

    /// Returns the edge profile.
    pub fn profile(&self) -> &EdgeProfile {
        &self.profile
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MockProvider;

    #[test]
    fn test_edge_profile_minimal() {
        let profile = EdgeProfile::minimal();
        assert_eq!(profile.max_memory, 512 * 1024 * 1024);
        assert_eq!(profile.cpu_cores, 1);
        assert!(!profile.has_gpu);
        assert!(profile.is_offline());
        assert!(profile.battery_powered);
    }

    #[test]
    fn test_edge_profile_can_run_model() {
        let profile = EdgeProfile::standard();

        // Small model should fit
        assert!(profile.can_run_model(100 * 1024 * 1024, 200 * 1024 * 1024));

        // Large model should not fit
        assert!(!profile.can_run_model(1024 * 1024 * 1024, 1024 * 1024 * 1024));
    }

    #[test]
    fn test_edge_config_minimal() {
        let config = EdgeConfig::minimal();
        assert_eq!(config.cache_size, 10);
        assert_eq!(config.max_prompt_length, 512);
        assert!(config.enable_truncation);
    }

    #[tokio::test]
    async fn test_edge_cache() {
        let cache = EdgeCache::new(2);

        cache.put("key1".to_string(), "value1".to_string()).await;
        cache.put("key2".to_string(), "value2".to_string()).await;

        assert_eq!(cache.get("key1").await, Some("value1".to_string()));
        assert_eq!(cache.get("key2").await, Some("value2".to_string()));

        let (hits, misses) = cache.stats().await;
        assert_eq!(hits, 2);
        assert_eq!(misses, 0);
    }

    #[tokio::test]
    async fn test_edge_provider() {
        let provider = MockProvider::default();
        let config = EdgeConfig::default();
        let edge_provider = EdgeProvider::new(provider, config);

        let result = edge_provider.generate_text("test prompt").await;
        assert!(result.is_ok());

        let stats = edge_provider.stats().await;
        assert_eq!(stats.total_requests, 1);
    }

    #[test]
    fn test_edge_provider_truncate() {
        let provider = MockProvider::default();
        let mut config = EdgeConfig::default();
        config.max_prompt_length = 10;
        let edge_provider = EdgeProvider::new(provider, config);

        let long_prompt = "This is a very long prompt that should be truncated";
        let truncated = edge_provider.truncate_prompt(long_prompt);

        assert!(truncated.len() <= 10);
        assert!(truncated.ends_with("..."));
    }

    #[test]
    fn test_edge_manifest() {
        let manifest = EdgeManifest::new("tiny-llama", "1.0");
        let profile = EdgeProfile::standard();

        // Empty manifest should be compatible
        assert!(manifest.is_compatible(&profile));
    }

    #[test]
    fn test_edge_manifest_json() {
        let manifest = EdgeManifest::new("test-model", "1.0");
        let json = manifest.to_json().unwrap();
        let loaded = EdgeManifest::from_json(&json).unwrap();

        assert_eq!(loaded.model_name, "test-model");
        assert_eq!(loaded.version, "1.0");
    }

    #[tokio::test]
    async fn test_edge_deployment() {
        let profile = EdgeProfile::standard();
        let deployment = EdgeDeployment::new(profile);

        let manifest = EdgeManifest::new("test-model", "1.0");
        deployment.register_manifest(manifest).await.unwrap();

        let models = deployment.list_models().await;
        assert_eq!(models.len(), 1);
        assert_eq!(models[0], "test-model");
    }

    #[tokio::test]
    async fn test_edge_stats() {
        let mut stats = EdgeStats::default();
        stats.total_requests = 10;
        stats.cache_hits = 7;
        stats.total_latency_ms = 1000;

        assert_eq!(stats.cache_hit_rate(), 0.7);
        assert_eq!(stats.avg_latency_ms(), 100.0);
    }
}
