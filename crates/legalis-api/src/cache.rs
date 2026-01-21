//! Response caching and ETag support with in-memory and Redis backends.

use async_trait::async_trait;
use axum::{
    body::Body,
    extract::Request,
    http::{StatusCode, header},
    middleware::Next,
    response::Response,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Cache entry.
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct CacheEntry {
    pub etag: String,
    pub content: Vec<u8>,
    pub content_type: String,
}

/// Cache backend trait for abstracting storage implementations.
#[async_trait]
pub trait CacheBackend: Send + Sync {
    /// Get a cache entry by key.
    async fn get(&self, key: &str) -> Option<CacheEntry>;

    /// Set a cache entry with optional TTL in seconds.
    async fn set(&self, key: String, entry: CacheEntry, ttl: Option<u64>);

    /// Invalidate (remove) a cache entry.
    async fn invalidate(&self, key: &str);

    /// Clear all cache entries.
    async fn clear(&self);

    /// Invalidate entries matching a pattern (e.g., "statutes:*").
    async fn invalidate_pattern(&self, pattern: &str);

    /// Warm the cache by preloading multiple entries at once.
    /// This is useful for preloading frequently accessed data on startup.
    async fn warm(&self, entries: Vec<(String, CacheEntry, Option<u64>)>) {
        for (key, entry, ttl) in entries {
            self.set(key, entry, ttl).await;
        }
    }

    /// Get cache statistics (if supported by the backend).
    async fn stats(&self) -> CacheStats {
        CacheStats::default()
    }
}

/// Cache statistics.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct CacheStats {
    /// Number of entries in the cache
    pub entry_count: usize,
    /// Cache hit rate (0.0 to 1.0)
    pub hit_rate: f64,
    /// Total hits
    pub hits: u64,
    /// Total misses
    pub misses: u64,
}

/// In-memory cache store implementation.
pub struct InMemoryCacheBackend {
    entries: RwLock<HashMap<String, CacheEntry>>,
    hits: Arc<RwLock<u64>>,
    misses: Arc<RwLock<u64>>,
}

impl InMemoryCacheBackend {
    pub fn new() -> Self {
        Self {
            entries: RwLock::new(HashMap::new()),
            hits: Arc::new(RwLock::new(0)),
            misses: Arc::new(RwLock::new(0)),
        }
    }
}

impl Default for InMemoryCacheBackend {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl CacheBackend for InMemoryCacheBackend {
    async fn get(&self, key: &str) -> Option<CacheEntry> {
        let result = self.entries.read().await.get(key).cloned();
        if result.is_some() {
            *self.hits.write().await += 1;
        } else {
            *self.misses.write().await += 1;
        }
        result
    }

    async fn set(&self, key: String, entry: CacheEntry, _ttl: Option<u64>) {
        self.entries.write().await.insert(key, entry);
    }

    async fn invalidate(&self, key: &str) {
        self.entries.write().await.remove(key);
    }

    async fn clear(&self) {
        self.entries.write().await.clear();
    }

    async fn invalidate_pattern(&self, pattern: &str) {
        // Simple pattern matching for in-memory cache
        let pattern_prefix = pattern.trim_end_matches('*');
        let mut entries = self.entries.write().await;
        entries.retain(|k, _| !k.starts_with(pattern_prefix));
    }

    async fn stats(&self) -> CacheStats {
        let entry_count = self.entries.read().await.len();
        let hits = *self.hits.read().await;
        let misses = *self.misses.read().await;
        let total = hits + misses;
        let hit_rate = if total > 0 {
            hits as f64 / total as f64
        } else {
            0.0
        };

        CacheStats {
            entry_count,
            hit_rate,
            hits,
            misses,
        }
    }
}

/// Redis cache backend implementation (requires "redis-cache" feature).
#[cfg(feature = "redis-cache")]
pub struct RedisCacheBackend {
    _client: redis::Client,
    connection_manager: Arc<RwLock<Option<redis::aio::ConnectionManager>>>,
}

#[cfg(feature = "redis-cache")]
impl RedisCacheBackend {
    /// Create a new Redis cache backend.
    pub async fn new(redis_url: &str) -> Result<Self, redis::RedisError> {
        let client = redis::Client::open(redis_url)?;
        let conn_manager = redis::aio::ConnectionManager::new(client.clone()).await?;

        Ok(Self {
            _client: client,
            connection_manager: Arc::new(RwLock::new(Some(conn_manager))),
        })
    }

    async fn get_connection(&self) -> Option<redis::aio::ConnectionManager> {
        self.connection_manager.read().await.clone()
    }
}

#[cfg(feature = "redis-cache")]
#[async_trait]
impl CacheBackend for RedisCacheBackend {
    async fn get(&self, key: &str) -> Option<CacheEntry> {
        use redis::AsyncCommands;

        let mut conn = self.get_connection().await?;
        let data: Option<Vec<u8>> = conn.get(key).await.ok()?;

        data.and_then(|bytes| serde_json::from_slice(&bytes).ok())
    }

    async fn set(&self, key: String, entry: CacheEntry, ttl: Option<u64>) {
        use redis::AsyncCommands;

        if let Some(mut conn) = self.get_connection().await
            && let Ok(serialized) = serde_json::to_vec(&entry)
        {
            let _: Result<(), redis::RedisError> = if let Some(ttl_secs) = ttl {
                conn.set_ex(key, serialized, ttl_secs).await
            } else {
                conn.set(key, serialized).await
            };
        }
    }

    async fn invalidate(&self, key: &str) {
        use redis::AsyncCommands;

        if let Some(mut conn) = self.get_connection().await {
            let _: Result<(), redis::RedisError> = conn.del(key).await;
        }
    }

    async fn clear(&self) {
        if let Some(mut conn) = self.get_connection().await {
            let _: Result<(), redis::RedisError> =
                redis::cmd("FLUSHDB").query_async(&mut conn).await;
        }
    }

    async fn invalidate_pattern(&self, pattern: &str) {
        use redis::AsyncCommands;

        if let Some(mut conn) = self.get_connection().await {
            // Use SCAN to find matching keys
            let keys: Vec<String> = redis::cmd("KEYS")
                .arg(pattern)
                .query_async(&mut conn)
                .await
                .unwrap_or_default();

            if !keys.is_empty() {
                let _: Result<(), redis::RedisError> = conn.del(keys).await;
            }
        }
    }
}

/// Cache store that wraps a backend implementation.
pub struct CacheStore {
    backend: Arc<dyn CacheBackend>,
}

impl CacheStore {
    /// Create a new cache store with in-memory backend.
    pub fn new() -> Self {
        Self {
            backend: Arc::new(InMemoryCacheBackend::new()),
        }
    }

    /// Create a cache store with a custom backend.
    pub fn with_backend(backend: Arc<dyn CacheBackend>) -> Self {
        Self { backend }
    }

    /// Create a cache store with Redis backend (requires "redis-cache" feature).
    #[cfg(feature = "redis-cache")]
    pub async fn with_redis(redis_url: &str) -> Result<Self, redis::RedisError> {
        let backend = RedisCacheBackend::new(redis_url).await?;
        Ok(Self {
            backend: Arc::new(backend),
        })
    }

    pub async fn get(&self, key: &str) -> Option<CacheEntry> {
        self.backend.get(key).await
    }

    pub async fn set(&self, key: String, entry: CacheEntry) {
        self.backend.set(key, entry, None).await;
    }

    pub async fn set_with_ttl(&self, key: String, entry: CacheEntry, ttl_secs: u64) {
        self.backend.set(key, entry, Some(ttl_secs)).await;
    }

    pub async fn invalidate(&self, key: &str) {
        self.backend.invalidate(key).await;
    }

    pub async fn clear(&self) {
        self.backend.clear().await;
    }

    /// Invalidate all cache entries matching a pattern.
    /// For example: "statutes:*" invalidates all statute-related entries.
    pub async fn invalidate_pattern(&self, pattern: &str) {
        self.backend.invalidate_pattern(pattern).await;
    }
}

impl Default for CacheStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Cache invalidation strategies.
pub enum InvalidationStrategy {
    /// Time-to-live based invalidation (in seconds).
    Ttl(u64),
    /// Invalidate on write operations.
    WriteThrough,
    /// Invalidate by pattern (e.g., "statutes:*").
    Pattern(String),
    /// Manual invalidation only.
    Manual,
}

/// Helper to generate cache keys for different resource types.
pub mod cache_keys {
    /// Generate cache key for a statute by ID.
    pub fn statute(id: &str) -> String {
        format!("statute:{}", id)
    }

    /// Generate cache key for statute list.
    pub fn statute_list(query: &str) -> String {
        format!("statutes:list:{}", query)
    }

    /// Generate cache key for verification result.
    pub fn verification(statute_ids: &[String]) -> String {
        let ids = statute_ids.join(",");
        format!("verification:{}", ids)
    }

    /// Generate cache key for simulation result.
    pub fn simulation(statute_ids: &[String], population_size: usize) -> String {
        let ids = statute_ids.join(",");
        format!("simulation:{}:{}", ids, population_size)
    }

    /// Generate cache key for visualization.
    pub fn visualization(id: &str, format: &str, theme: &str) -> String {
        format!("viz:{}:{}:{}", id, format, theme)
    }
}

/// Generate an ETag from content.
pub fn generate_etag(content: &[u8]) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    content.hash(&mut hasher);
    format!("\"{}\"", hasher.finish())
}

/// Middleware for ETag support.
/// Checks If-None-Match header and returns 304 if ETag matches.
pub async fn etag_middleware(
    cache_store: Arc<CacheStore>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let cache_key = format!("{}:{}", req.method(), req.uri());

    // Check if client sent If-None-Match header
    if let Some(if_none_match) = req.headers().get(header::IF_NONE_MATCH)
        && let Some(cached) = cache_store.get(&cache_key).await
        && if_none_match.to_str().ok() == Some(&cached.etag)
    {
        // ETag matches - return 304 Not Modified
        return Ok(Response::builder()
            .status(StatusCode::NOT_MODIFIED)
            .header(header::ETAG, cached.etag)
            .body(Body::empty())
            .unwrap());
    }

    // Process request
    let response = next.run(req).await;

    // For GET requests, try to add ETag to response
    // Note: This is a simplified version. In production, you'd want to:
    // 1. Only cache certain endpoints
    // 2. Set appropriate cache-control headers
    // 3. Handle cache invalidation on mutations

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_etag() {
        let content = b"Hello, World!";
        let etag = generate_etag(content);
        assert!(etag.starts_with('"'));
        assert!(etag.ends_with('"'));

        // Same content should generate same ETag
        let etag2 = generate_etag(content);
        assert_eq!(etag, etag2);
    }

    #[tokio::test]
    async fn test_cache_store() {
        let store = CacheStore::new();

        let entry = CacheEntry {
            etag: "\"12345\"".to_string(),
            content: b"test".to_vec(),
            content_type: "application/json".to_string(),
        };

        store.set("test-key".to_string(), entry.clone()).await;

        let retrieved = store.get("test-key").await;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().etag, "\"12345\"");

        store.invalidate("test-key").await;
        assert!(store.get("test-key").await.is_none());
    }

    #[tokio::test]
    async fn test_cache_clear() {
        let store = CacheStore::new();

        let entry = CacheEntry {
            etag: "\"12345\"".to_string(),
            content: b"test".to_vec(),
            content_type: "application/json".to_string(),
        };

        store.set("key1".to_string(), entry.clone()).await;
        store.set("key2".to_string(), entry).await;

        assert!(store.get("key1").await.is_some());
        assert!(store.get("key2").await.is_some());

        store.clear().await;

        assert!(store.get("key1").await.is_none());
        assert!(store.get("key2").await.is_none());
    }

    #[tokio::test]
    async fn test_cache_invalidate_pattern() {
        let store = CacheStore::new();

        let entry = CacheEntry {
            etag: "\"12345\"".to_string(),
            content: b"test".to_vec(),
            content_type: "application/json".to_string(),
        };

        store.set("statutes:1".to_string(), entry.clone()).await;
        store.set("statutes:2".to_string(), entry.clone()).await;
        store.set("verification:1".to_string(), entry).await;

        assert!(store.get("statutes:1").await.is_some());
        assert!(store.get("statutes:2").await.is_some());
        assert!(store.get("verification:1").await.is_some());

        // Invalidate all statute-related entries
        store.invalidate_pattern("statutes:*").await;

        assert!(store.get("statutes:1").await.is_none());
        assert!(store.get("statutes:2").await.is_none());
        assert!(store.get("verification:1").await.is_some());
    }

    #[tokio::test]
    async fn test_cache_with_ttl() {
        let store = CacheStore::new();

        let entry = CacheEntry {
            etag: "\"12345\"".to_string(),
            content: b"test".to_vec(),
            content_type: "application/json".to_string(),
        };

        // In-memory backend ignores TTL, but this tests the API
        store
            .set_with_ttl("test-key".to_string(), entry.clone(), 60)
            .await;

        let retrieved = store.get("test-key").await;
        assert!(retrieved.is_some());
    }

    #[test]
    fn test_cache_keys() {
        use super::cache_keys;

        assert_eq!(cache_keys::statute("test-id"), "statute:test-id");
        assert_eq!(cache_keys::statute_list("query"), "statutes:list:query");
        assert_eq!(
            cache_keys::verification(&["id1".to_string(), "id2".to_string()]),
            "verification:id1,id2"
        );
        assert_eq!(
            cache_keys::simulation(&["id1".to_string()], 1000),
            "simulation:id1:1000"
        );
        assert_eq!(
            cache_keys::visualization("id", "svg", "dark"),
            "viz:id:svg:dark"
        );
    }
}
