//! Response caching layer for LLM requests.
//!
//! This module provides a caching layer to reduce API calls and costs by storing
//! and reusing previous LLM responses.

use anyhow::Result;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// A cached response with metadata.
#[derive(Debug, Clone)]
pub struct CachedResponse {
    /// The cached response text
    pub response: String,
    /// When this entry was created
    pub created_at: Instant,
    /// How many times this cache entry has been used
    pub hit_count: usize,
    /// Time-to-live for this entry
    pub ttl: Duration,
}

impl CachedResponse {
    /// Creates a new cached response.
    pub fn new(response: String, ttl: Duration) -> Self {
        Self {
            response,
            created_at: Instant::now(),
            hit_count: 0,
            ttl,
        }
    }

    /// Checks if this cache entry has expired.
    pub fn is_expired(&self) -> bool {
        self.created_at.elapsed() > self.ttl
    }

    /// Increments the hit count.
    pub fn increment_hit_count(&mut self) {
        self.hit_count += 1;
    }
}

/// Configuration for the response cache.
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Maximum number of entries in the cache
    pub max_entries: usize,
    /// Default time-to-live for cache entries
    pub default_ttl: Duration,
    /// Whether caching is enabled
    pub enabled: bool,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_entries: 1000,
            default_ttl: Duration::from_secs(3600), // 1 hour
            enabled: true,
        }
    }
}

impl CacheConfig {
    /// Creates a new cache configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the maximum number of entries.
    pub fn with_max_entries(mut self, max_entries: usize) -> Self {
        self.max_entries = max_entries;
        self
    }

    /// Sets the default TTL.
    pub fn with_default_ttl(mut self, ttl: Duration) -> Self {
        self.default_ttl = ttl;
        self
    }

    /// Sets whether caching is enabled.
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

/// Statistics for cache performance.
#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    /// Total number of cache hits
    pub hits: usize,
    /// Total number of cache misses
    pub misses: usize,
    /// Total number of evictions
    pub evictions: usize,
    /// Current number of entries
    pub entries: usize,
}

impl CacheStats {
    /// Calculates the cache hit rate.
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            (self.hits as f64 / total as f64) * 100.0
        }
    }
}

/// A simple LRU cache for LLM responses.
#[derive(Clone)]
pub struct ResponseCache {
    cache: Arc<Mutex<HashMap<u64, CachedResponse>>>,
    config: CacheConfig,
    stats: Arc<Mutex<CacheStats>>,
}

impl ResponseCache {
    /// Creates a new response cache with default configuration.
    pub fn new() -> Self {
        Self::with_config(CacheConfig::default())
    }

    /// Creates a new response cache with custom configuration.
    pub fn with_config(config: CacheConfig) -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
            config,
            stats: Arc::new(Mutex::new(CacheStats::default())),
        }
    }

    /// Computes a hash key for a prompt.
    fn hash_prompt(prompt: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        let mut hasher = DefaultHasher::new();
        prompt.hash(&mut hasher);
        hasher.finish()
    }

    /// Gets a cached response if available and not expired.
    pub fn get(&self, prompt: &str) -> Option<String> {
        if !self.config.enabled {
            return None;
        }

        let key = Self::hash_prompt(prompt);
        let mut cache = self.cache.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();

        if let Some(entry) = cache.get_mut(&key) {
            if entry.is_expired() {
                // Remove expired entry
                cache.remove(&key);
                stats.misses += 1;
                None
            } else {
                // Cache hit
                entry.increment_hit_count();
                stats.hits += 1;
                Some(entry.response.clone())
            }
        } else {
            // Cache miss
            stats.misses += 1;
            None
        }
    }

    /// Stores a response in the cache.
    pub fn set(&self, prompt: &str, response: String) {
        if !self.config.enabled {
            return;
        }

        let key = Self::hash_prompt(prompt);
        let mut cache = self.cache.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();

        // Evict entries if we're at capacity
        if cache.len() >= self.config.max_entries && !cache.contains_key(&key) {
            self.evict_lru(&mut cache, &mut stats);
        }

        let entry = CachedResponse::new(response, self.config.default_ttl);
        cache.insert(key, entry);
        stats.entries = cache.len();
    }

    /// Stores a response with a custom TTL.
    pub fn set_with_ttl(&self, prompt: &str, response: String, ttl: Duration) {
        if !self.config.enabled {
            return;
        }

        let key = Self::hash_prompt(prompt);
        let mut cache = self.cache.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();

        // Evict entries if we're at capacity
        if cache.len() >= self.config.max_entries && !cache.contains_key(&key) {
            self.evict_lru(&mut cache, &mut stats);
        }

        let entry = CachedResponse::new(response, ttl);
        cache.insert(key, entry);
        stats.entries = cache.len();
    }

    /// Evicts the least recently used entry.
    fn evict_lru(&self, cache: &mut HashMap<u64, CachedResponse>, stats: &mut CacheStats) {
        // Find the entry with the oldest created_at time and lowest hit count
        let mut oldest_key = None;
        let mut oldest_time = Instant::now();
        let mut lowest_hits = usize::MAX;

        for (key, entry) in cache.iter() {
            if entry.hit_count < lowest_hits
                || (entry.hit_count == lowest_hits && entry.created_at < oldest_time)
            {
                oldest_key = Some(*key);
                oldest_time = entry.created_at;
                lowest_hits = entry.hit_count;
            }
        }

        if let Some(key) = oldest_key {
            cache.remove(&key);
            stats.evictions += 1;
        }
    }

    /// Clears all expired entries from the cache.
    pub fn clear_expired(&self) {
        let mut cache = self.cache.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();

        let before_count = cache.len();
        cache.retain(|_, entry| !entry.is_expired());
        let after_count = cache.len();

        stats.entries = after_count;
        stats.evictions += before_count - after_count;
    }

    /// Clears all entries from the cache.
    pub fn clear(&self) {
        let mut cache = self.cache.lock().unwrap();
        let mut stats = self.stats.lock().unwrap();

        cache.clear();
        stats.entries = 0;
    }

    /// Gets the current cache statistics.
    pub fn stats(&self) -> CacheStats {
        let stats = self.stats.lock().unwrap();
        stats.clone()
    }

    /// Resets the cache statistics.
    pub fn reset_stats(&self) {
        let mut stats = self.stats.lock().unwrap();
        *stats = CacheStats::default();
        stats.entries = self.cache.lock().unwrap().len();
    }

    /// Gets the current number of entries in the cache.
    pub fn len(&self) -> usize {
        self.cache.lock().unwrap().len()
    }

    /// Checks if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.cache.lock().unwrap().is_empty()
    }
}

impl Default for ResponseCache {
    fn default() -> Self {
        Self::new()
    }
}

/// A wrapper around an LLM provider that adds caching.
pub struct CachedProvider<P> {
    provider: P,
    cache: ResponseCache,
}

impl<P> CachedProvider<P> {
    /// Creates a new cached provider.
    pub fn new(provider: P) -> Self {
        Self {
            provider,
            cache: ResponseCache::new(),
        }
    }

    /// Creates a new cached provider with custom cache configuration.
    pub fn with_cache_config(provider: P, config: CacheConfig) -> Self {
        Self {
            provider,
            cache: ResponseCache::with_config(config),
        }
    }

    /// Gets the cache instance.
    pub fn cache(&self) -> &ResponseCache {
        &self.cache
    }

    /// Gets the underlying provider.
    pub fn provider(&self) -> &P {
        &self.provider
    }
}

#[async_trait::async_trait]
impl<P: crate::LLMProvider> crate::LLMProvider for CachedProvider<P> {
    async fn generate_text(&self, prompt: &str) -> Result<String> {
        // Check cache first
        if let Some(cached) = self.cache.get(prompt) {
            return Ok(cached);
        }

        // Cache miss - call provider
        let response = self.provider.generate_text(prompt).await?;

        // Store in cache
        self.cache.set(prompt, response.clone());

        Ok(response)
    }

    async fn generate_structured<T: serde::de::DeserializeOwned + Send>(
        &self,
        prompt: &str,
    ) -> Result<T> {
        // Structured responses are not cached because we can't serialize T
        // without adding Serialize bound which would break the trait contract.
        // Users can cache the text response instead if needed.
        self.provider.generate_structured::<T>(prompt).await
    }

    async fn generate_text_stream(&self, prompt: &str) -> Result<crate::TextStream> {
        // Streaming responses are not cached
        self.provider.generate_text_stream(prompt).await
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

/// Semantic cache for finding similar prompts.
///
/// Uses a simple similarity metric to match prompts that are semantically similar
/// but not identical.
pub struct SemanticCache {
    cache: ResponseCache,
    /// Minimum similarity threshold (0.0 - 1.0)
    similarity_threshold: f64,
    /// Store original prompts for similarity comparison
    prompts: Arc<Mutex<Vec<String>>>,
}

impl SemanticCache {
    /// Creates a new semantic cache.
    pub fn new() -> Self {
        Self {
            cache: ResponseCache::new(),
            similarity_threshold: 0.85,
            prompts: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Creates a new semantic cache with custom configuration.
    pub fn with_config(config: CacheConfig, similarity_threshold: f64) -> Self {
        Self {
            cache: ResponseCache::with_config(config),
            similarity_threshold: similarity_threshold.clamp(0.0, 1.0),
            prompts: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Sets the similarity threshold.
    pub fn with_threshold(mut self, threshold: f64) -> Self {
        self.similarity_threshold = threshold.clamp(0.0, 1.0);
        self
    }

    /// Calculates Jaccard similarity between two strings.
    fn jaccard_similarity(s1: &str, s2: &str) -> f64 {
        let words1: std::collections::HashSet<&str> = s1.split_whitespace().collect();
        let words2: std::collections::HashSet<&str> = s2.split_whitespace().collect();

        if words1.is_empty() && words2.is_empty() {
            return 1.0;
        }

        let intersection = words1.intersection(&words2).count();
        let union = words1.union(&words2).count();

        if union == 0 {
            0.0
        } else {
            intersection as f64 / union as f64
        }
    }

    /// Finds the most similar prompt in the cache.
    fn find_similar_prompt(&self, prompt: &str) -> Option<String> {
        let prompts = self.prompts.lock().unwrap();

        let mut best_match = None;
        let mut best_similarity = self.similarity_threshold;

        for cached_prompt in prompts.iter() {
            let similarity = Self::jaccard_similarity(prompt, cached_prompt);
            if similarity > best_similarity {
                best_similarity = similarity;
                best_match = Some(cached_prompt.clone());
            }
        }

        best_match
    }

    /// Gets a cached response, checking for semantic similarity.
    pub fn get(&self, prompt: &str) -> Option<String> {
        // Try exact match first
        if let Some(response) = self.cache.get(prompt) {
            return Some(response);
        }

        // Try semantic match
        if let Some(similar_prompt) = self.find_similar_prompt(prompt) {
            tracing::debug!("Found semantically similar prompt in cache");
            self.cache.get(&similar_prompt)
        } else {
            None
        }
    }

    /// Stores a response in the cache.
    pub fn set(&self, prompt: &str, response: String) {
        self.cache.set(prompt, response);

        let mut prompts = self.prompts.lock().unwrap();
        if !prompts.contains(&prompt.to_string()) {
            prompts.push(prompt.to_string());
        }
    }

    /// Stores a response with a custom TTL.
    pub fn set_with_ttl(&self, prompt: &str, response: String, ttl: Duration) {
        self.cache.set_with_ttl(prompt, response, ttl);

        let mut prompts = self.prompts.lock().unwrap();
        if !prompts.contains(&prompt.to_string()) {
            prompts.push(prompt.to_string());
        }
    }

    /// Clears the cache.
    pub fn clear(&self) {
        self.cache.clear();
        self.prompts.lock().unwrap().clear();
    }

    /// Gets cache statistics.
    pub fn stats(&self) -> CacheStats {
        self.cache.stats()
    }

    /// Gets the number of cached prompts.
    pub fn len(&self) -> usize {
        self.cache.len()
    }

    /// Checks if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }

    /// Clears expired entries.
    pub fn clear_expired(&self) {
        self.cache.clear_expired();
        // Note: This doesn't remove prompts from the prompts vec
        // as we don't track which prompts map to which cached entries
    }
}

impl Default for SemanticCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Cache invalidation strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvalidationStrategy {
    /// Time-based: invalidate after TTL expires
    TimeBased,
    /// Version-based: invalidate when version changes
    VersionBased,
    /// Pattern-based: invalidate entries matching a pattern
    PatternBased,
    /// Manual: only invalidate when explicitly requested
    Manual,
}

/// Cache entry with invalidation metadata.
#[derive(Debug, Clone)]
pub struct InvalidatableCacheEntry {
    pub response: String,
    pub created_at: Instant,
    pub ttl: Duration,
    pub version: Option<String>,
    pub tags: Vec<String>,
}

impl InvalidatableCacheEntry {
    pub fn new(response: String, ttl: Duration) -> Self {
        Self {
            response,
            created_at: Instant::now(),
            ttl,
            version: None,
            tags: Vec::new(),
        }
    }

    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    pub fn is_expired(&self) -> bool {
        self.created_at.elapsed() > self.ttl
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}

/// Advanced cache with invalidation strategies.
pub struct InvalidatableCache {
    cache: Arc<Mutex<HashMap<u64, InvalidatableCacheEntry>>>,
    strategy: InvalidationStrategy,
    current_version: Arc<Mutex<Option<String>>>,
}

impl InvalidatableCache {
    pub fn new(strategy: InvalidationStrategy) -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
            strategy,
            current_version: Arc::new(Mutex::new(None)),
        }
    }

    fn hash_prompt(prompt: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        prompt.hash(&mut hasher);
        hasher.finish()
    }

    pub fn get(&self, prompt: &str) -> Option<String> {
        let key = Self::hash_prompt(prompt);
        let mut cache = self.cache.lock().unwrap();

        if let Some(entry) = cache.get(&key) {
            // Check expiration
            if entry.is_expired() {
                cache.remove(&key);
                return None;
            }

            // Check version-based invalidation
            if self.strategy == InvalidationStrategy::VersionBased {
                let current_version = self.current_version.lock().unwrap();
                if let (Some(entry_version), Some(curr_version)) =
                    (&entry.version, &*current_version)
                    && entry_version != curr_version
                {
                    cache.remove(&key);
                    return None;
                }
            }

            Some(entry.response.clone())
        } else {
            None
        }
    }

    pub fn set(&self, prompt: &str, entry: InvalidatableCacheEntry) {
        let key = Self::hash_prompt(prompt);
        self.cache.lock().unwrap().insert(key, entry);
    }

    pub fn set_simple(&self, prompt: &str, response: String, ttl: Duration) {
        let entry = InvalidatableCacheEntry::new(response, ttl);
        self.set(prompt, entry);
    }

    pub fn set_version(&self, version: impl Into<String>) {
        *self.current_version.lock().unwrap() = Some(version.into());
    }

    pub fn invalidate_by_tag(&self, tag: &str) {
        let mut cache = self.cache.lock().unwrap();
        cache.retain(|_, entry| !entry.has_tag(tag));
    }

    pub fn invalidate_by_pattern(&self, pattern: &str) {
        // This is a simplified pattern match - in production you'd want regex
        let mut cache = self.cache.lock().unwrap();
        let keys_to_remove: Vec<u64> = cache
            .iter()
            .filter(|(_, entry)| entry.response.contains(pattern))
            .map(|(k, _)| *k)
            .collect();

        for key in keys_to_remove {
            cache.remove(&key);
        }
    }

    pub fn invalidate_all(&self) {
        self.cache.lock().unwrap().clear();
    }

    pub fn invalidate_expired(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.retain(|_, entry| !entry.is_expired());
    }

    pub fn len(&self) -> usize {
        self.cache.lock().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.cache.lock().unwrap().is_empty()
    }
}

/// Disk-based cache persistence.
pub struct DiskCache {
    cache_dir: std::path::PathBuf,
}

impl DiskCache {
    /// Creates a new disk cache.
    pub fn new(cache_dir: impl Into<std::path::PathBuf>) -> Self {
        Self {
            cache_dir: cache_dir.into(),
        }
    }

    /// Ensures the cache directory exists.
    fn ensure_cache_dir(&self) -> Result<()> {
        std::fs::create_dir_all(&self.cache_dir)?;
        Ok(())
    }

    /// Gets the cache file path for a prompt.
    fn cache_file_path(&self, prompt: &str) -> std::path::PathBuf {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        prompt.hash(&mut hasher);
        let hash = hasher.finish();

        self.cache_dir.join(format!("{:x}.cache", hash))
    }

    /// Gets a cached response from disk.
    pub fn get(&self, prompt: &str) -> Option<String> {
        let path = self.cache_file_path(prompt);

        if path.exists() {
            std::fs::read_to_string(&path).ok()
        } else {
            None
        }
    }

    /// Stores a response to disk.
    pub fn set(&self, prompt: &str, response: &str) -> Result<()> {
        self.ensure_cache_dir()?;
        let path = self.cache_file_path(prompt);
        std::fs::write(&path, response)?;
        Ok(())
    }

    /// Clears all cached files.
    pub fn clear(&self) -> Result<()> {
        if self.cache_dir.exists() {
            for entry in std::fs::read_dir(&self.cache_dir)? {
                let entry = entry?;
                if entry.path().extension() == Some(std::ffi::OsStr::new("cache")) {
                    std::fs::remove_file(entry.path())?;
                }
            }
        }
        Ok(())
    }

    /// Gets the number of cached files.
    pub fn len(&self) -> usize {
        if !self.cache_dir.exists() {
            return 0;
        }

        std::fs::read_dir(&self.cache_dir)
            .ok()
            .map(|entries| {
                entries
                    .filter_map(|e| e.ok())
                    .filter(|e| e.path().extension() == Some(std::ffi::OsStr::new("cache")))
                    .count()
            })
            .unwrap_or(0)
    }

    /// Checks if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_basic() {
        let cache = ResponseCache::new();

        // Cache miss
        assert!(cache.get("test prompt").is_none());

        // Store
        cache.set("test prompt", "test response".to_string());

        // Cache hit
        assert_eq!(cache.get("test prompt"), Some("test response".to_string()));

        let stats = cache.stats();
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
        assert_eq!(stats.entries, 1);
    }

    #[test]
    fn test_cache_expiry() {
        let config = CacheConfig::new().with_default_ttl(Duration::from_millis(100));
        let cache = ResponseCache::with_config(config);

        cache.set("test", "response".to_string());
        assert!(cache.get("test").is_some());

        // Wait for expiry
        std::thread::sleep(Duration::from_millis(150));

        // Should be expired
        assert!(cache.get("test").is_none());
    }

    #[test]
    fn test_cache_eviction() {
        let config = CacheConfig::new().with_max_entries(2);
        let cache = ResponseCache::with_config(config);

        cache.set("prompt1", "response1".to_string());
        cache.set("prompt2", "response2".to_string());
        cache.set("prompt3", "response3".to_string());

        // Should have evicted one entry
        assert_eq!(cache.len(), 2);

        let stats = cache.stats();
        assert_eq!(stats.evictions, 1);
    }

    #[test]
    fn test_cache_hit_rate() {
        let cache = ResponseCache::new();

        cache.set("test", "response".to_string());

        // 3 hits
        cache.get("test");
        cache.get("test");
        cache.get("test");

        // 1 miss
        cache.get("other");

        let stats = cache.stats();
        assert_eq!(stats.hits, 3);
        assert_eq!(stats.misses, 1);
        assert!((stats.hit_rate() - 75.0).abs() < 0.1);
    }

    #[test]
    fn test_cache_disabled() {
        let config = CacheConfig::new().with_enabled(false);
        let cache = ResponseCache::with_config(config);

        cache.set("test", "response".to_string());
        assert!(cache.get("test").is_none());
    }

    #[test]
    fn test_semantic_cache_exact_match() {
        let cache = SemanticCache::new();

        cache.set("What is the capital of France?", "Paris".to_string());

        let result = cache.get("What is the capital of France?");
        assert_eq!(result, Some("Paris".to_string()));
    }

    #[test]
    fn test_semantic_cache_similar_match() {
        let cache = SemanticCache::new().with_threshold(0.7);

        cache.set("What is the capital of France?", "Paris".to_string());

        // Similar but not identical
        let result = cache.get("What is the capital city of France?");
        assert!(result.is_some());
    }

    #[test]
    fn test_semantic_cache_no_match() {
        let cache = SemanticCache::new();

        cache.set("What is the capital of France?", "Paris".to_string());

        let result = cache.get("What is the weather today?");
        assert!(result.is_none());
    }

    #[test]
    fn test_semantic_cache_jaccard_similarity() {
        let sim1 = SemanticCache::jaccard_similarity("hello world", "hello world");
        assert!((sim1 - 1.0).abs() < 0.01);

        let sim2 = SemanticCache::jaccard_similarity("hello world", "world hello");
        assert!((sim2 - 1.0).abs() < 0.01);

        let sim3 = SemanticCache::jaccard_similarity("hello world", "goodbye world");
        assert!(sim3 > 0.0 && sim3 < 1.0);

        let sim4 = SemanticCache::jaccard_similarity("hello", "goodbye");
        assert!((sim4 - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_disk_cache_basic() {
        let temp_dir = std::env::temp_dir().join("legalis_llm_test_cache");
        let cache = DiskCache::new(&temp_dir);

        cache.set("test prompt", "test response").unwrap();

        let result = cache.get("test prompt");
        assert_eq!(result, Some("test response".to_string()));

        cache.clear().unwrap();
        assert!(cache.is_empty());

        // Clean up
        let _ = std::fs::remove_dir_all(temp_dir);
    }

    #[test]
    fn test_disk_cache_multiple_entries() {
        let temp_dir = std::env::temp_dir().join("legalis_llm_test_cache_multi");
        let cache = DiskCache::new(&temp_dir);

        cache.set("prompt1", "response1").unwrap();
        cache.set("prompt2", "response2").unwrap();
        cache.set("prompt3", "response3").unwrap();

        assert_eq!(cache.len(), 3);
        assert!(!cache.is_empty());

        assert_eq!(cache.get("prompt1"), Some("response1".to_string()));
        assert_eq!(cache.get("prompt2"), Some("response2".to_string()));
        assert_eq!(cache.get("prompt3"), Some("response3".to_string()));

        // Clean up
        cache.clear().unwrap();
        let _ = std::fs::remove_dir_all(temp_dir);
    }
}

/// Async-aware cache using tokio primitives for better async performance.
pub struct AsyncCache {
    cache: Arc<tokio::sync::RwLock<HashMap<u64, CachedResponse>>>,
    config: CacheConfig,
    stats: Arc<tokio::sync::RwLock<CacheStats>>,
}

impl AsyncCache {
    /// Creates a new async cache with default configuration.
    pub fn new() -> Self {
        Self::with_config(CacheConfig::default())
    }

    /// Creates a new async cache with custom configuration.
    pub fn with_config(config: CacheConfig) -> Self {
        Self {
            cache: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            config,
            stats: Arc::new(tokio::sync::RwLock::new(CacheStats::default())),
        }
    }

    /// Computes a hash key for a prompt.
    fn hash_prompt(prompt: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        let mut hasher = DefaultHasher::new();
        prompt.hash(&mut hasher);
        hasher.finish()
    }

    /// Gets a cached response if available and not expired.
    pub async fn get(&self, prompt: &str) -> Option<String> {
        if !self.config.enabled {
            return None;
        }

        let key = Self::hash_prompt(prompt);
        let mut cache = self.cache.write().await;
        let mut stats = self.stats.write().await;

        if let Some(entry) = cache.get_mut(&key) {
            if entry.is_expired() {
                cache.remove(&key);
                stats.misses += 1;
                None
            } else {
                entry.increment_hit_count();
                stats.hits += 1;
                Some(entry.response.clone())
            }
        } else {
            stats.misses += 1;
            None
        }
    }

    /// Stores a response in the cache.
    pub async fn set(&self, prompt: &str, response: String) {
        if !self.config.enabled {
            return;
        }

        let key = Self::hash_prompt(prompt);
        let mut cache = self.cache.write().await;

        // Evict oldest entry if cache is full
        if cache.len() >= self.config.max_entries
            && let Some(oldest_key) = cache
                .iter()
                .min_by_key(|(_, v)| v.created_at)
                .map(|(k, _)| *k)
        {
            cache.remove(&oldest_key);
            let mut stats = self.stats.write().await;
            stats.evictions += 1;
        }

        let cached = CachedResponse::new(response, self.config.default_ttl);
        cache.insert(key, cached);

        let mut stats = self.stats.write().await;
        stats.entries = cache.len();
    }

    /// Gets cache statistics.
    pub async fn stats(&self) -> CacheStats {
        self.stats.read().await.clone()
    }

    /// Clears all cached entries.
    pub async fn clear(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
        let mut stats = self.stats.write().await;
        stats.entries = 0;
    }

    /// Pre-warms the cache with a set of common prompts and responses.
    pub async fn warm(&self, entries: Vec<(String, String)>) {
        for (prompt, response) in entries {
            self.set(&prompt, response).await;
        }
    }

    /// Removes expired entries from the cache.
    pub async fn evict_expired(&self) -> usize {
        let mut cache = self.cache.write().await;
        let before_count = cache.len();

        cache.retain(|_, entry| !entry.is_expired());

        let evicted = before_count - cache.len();
        let mut stats = self.stats.write().await;
        stats.evictions += evicted;
        stats.entries = cache.len();

        evicted
    }
}

impl Default for AsyncCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Cache warming utility for preloading common prompts.
pub struct CacheWarmer {
    warming_prompts: Vec<(String, String)>,
}

impl CacheWarmer {
    /// Creates a new cache warmer.
    pub fn new() -> Self {
        Self {
            warming_prompts: Vec::new(),
        }
    }

    /// Adds a prompt-response pair to the warming set.
    pub fn add_entry(mut self, prompt: impl Into<String>, response: impl Into<String>) -> Self {
        self.warming_prompts.push((prompt.into(), response.into()));
        self
    }

    /// Loads common legal prompts for warming.
    pub fn with_legal_templates(mut self) -> Self {
        self.warming_prompts.extend(vec![
            (
                "What is consideration in contract law?".to_string(),
                "Consideration is something of value exchanged between parties...".to_string(),
            ),
            (
                "Define mens rea".to_string(),
                "Mens rea is the mental element of a crime...".to_string(),
            ),
            (
                "What is the statute of limitations?".to_string(),
                "The statute of limitations is a law that sets the maximum time...".to_string(),
            ),
        ]);
        self
    }

    /// Warms the given cache with the stored prompts.
    pub async fn warm_cache(&self, cache: &AsyncCache) {
        cache.warm(self.warming_prompts.clone()).await;
    }
}

impl Default for CacheWarmer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod async_tests {
    use super::*;

    #[tokio::test]
    async fn test_async_cache_basic() {
        let cache = AsyncCache::new();

        cache.set("test prompt", "test response".to_string()).await;
        let result = cache.get("test prompt").await;

        assert_eq!(result, Some("test response".to_string()));
    }

    #[tokio::test]
    async fn test_async_cache_miss() {
        let cache = AsyncCache::new();
        let result = cache.get("nonexistent").await;

        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_async_cache_stats() {
        let cache = AsyncCache::new();

        cache.set("prompt1", "response1".to_string()).await;
        let _ = cache.get("prompt1").await;
        let _ = cache.get("prompt2").await;

        let stats = cache.stats().await;
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
    }

    #[tokio::test]
    async fn test_async_cache_expiry() {
        let config = CacheConfig::new().with_default_ttl(Duration::from_millis(10));
        let cache = AsyncCache::with_config(config);

        cache.set("prompt", "response".to_string()).await;
        tokio::time::sleep(Duration::from_millis(20)).await;

        let result = cache.get("prompt").await;
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_cache_warmer() {
        let cache = AsyncCache::new();
        let warmer = CacheWarmer::new()
            .add_entry("q1", "a1")
            .add_entry("q2", "a2");

        warmer.warm_cache(&cache).await;

        assert_eq!(cache.get("q1").await, Some("a1".to_string()));
        assert_eq!(cache.get("q2").await, Some("a2".to_string()));
    }

    #[tokio::test]
    async fn test_evict_expired() {
        let config = CacheConfig::new().with_default_ttl(Duration::from_millis(10));
        let cache = AsyncCache::with_config(config);

        cache.set("prompt1", "response1".to_string()).await;
        cache.set("prompt2", "response2".to_string()).await;

        tokio::time::sleep(Duration::from_millis(20)).await;
        let evicted = cache.evict_expired().await;

        assert_eq!(evicted, 2);
        let stats = cache.stats().await;
        assert_eq!(stats.entries, 0);
    }
}
