//! Response caching and ETag support.

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

/// Simple in-memory cache entry.
#[derive(Clone)]
pub struct CacheEntry {
    pub etag: String,
    pub content: Vec<u8>,
    pub content_type: String,
}

/// In-memory cache store.
pub struct CacheStore {
    entries: RwLock<HashMap<String, CacheEntry>>,
}

impl CacheStore {
    pub fn new() -> Self {
        Self {
            entries: RwLock::new(HashMap::new()),
        }
    }

    pub async fn get(&self, key: &str) -> Option<CacheEntry> {
        self.entries.read().await.get(key).cloned()
    }

    pub async fn set(&self, key: String, entry: CacheEntry) {
        self.entries.write().await.insert(key, entry);
    }

    pub async fn invalidate(&self, key: &str) {
        self.entries.write().await.remove(key);
    }

    pub async fn clear(&self) {
        self.entries.write().await.clear();
    }
}

impl Default for CacheStore {
    fn default() -> Self {
        Self::new()
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
    if let Some(if_none_match) = req.headers().get(header::IF_NONE_MATCH) {
        if let Some(cached) = cache_store.get(&cache_key).await {
            if if_none_match.to_str().ok() == Some(&cached.etag) {
                // ETag matches - return 304 Not Modified
                return Ok(Response::builder()
                    .status(StatusCode::NOT_MODIFIED)
                    .header(header::ETAG, cached.etag)
                    .body(Body::empty())
                    .unwrap());
            }
        }
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
}
