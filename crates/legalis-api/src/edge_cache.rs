//! Edge caching support for CDN-friendly responses.
//!
//! This module provides cache control headers and strategies for
//! distributing cached content through CDN and edge networks.

use axum::{
    body::Body,
    http::{HeaderValue, Request, Response, StatusCode, header},
    middleware::Next,
};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Cache control directives for different resource types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachePolicy {
    /// Maximum age in seconds
    pub max_age: u64,
    /// Whether the response can be cached by shared caches (CDN)
    pub public: bool,
    /// Whether the response must be revalidated
    pub must_revalidate: bool,
    /// S-maxage for shared caches (CDN)
    pub s_maxage: Option<u64>,
    /// Stale-while-revalidate duration
    pub stale_while_revalidate: Option<u64>,
    /// Stale-if-error duration
    pub stale_if_error: Option<u64>,
}

impl CachePolicy {
    /// Creates a cache policy for static resources
    pub fn static_resource() -> Self {
        Self {
            max_age: 86400, // 1 day
            public: true,
            must_revalidate: false,
            s_maxage: Some(604800), // 1 week for CDN
            stale_while_revalidate: Some(86400),
            stale_if_error: Some(259200), // 3 days
        }
    }

    /// Creates a cache policy for API responses
    pub fn api_response(max_age: u64) -> Self {
        Self {
            max_age,
            public: true,
            must_revalidate: true,
            s_maxage: Some(max_age),
            stale_while_revalidate: Some(60),
            stale_if_error: Some(300),
        }
    }

    /// Creates a cache policy for frequently changing data
    pub fn dynamic_content() -> Self {
        Self {
            max_age: 60, // 1 minute
            public: true,
            must_revalidate: true,
            s_maxage: Some(60),
            stale_while_revalidate: Some(30),
            stale_if_error: Some(120),
        }
    }

    /// Creates a no-cache policy
    pub fn no_cache() -> Self {
        Self {
            max_age: 0,
            public: false,
            must_revalidate: true,
            s_maxage: None,
            stale_while_revalidate: None,
            stale_if_error: None,
        }
    }

    /// Converts the policy to a Cache-Control header value
    pub fn to_header_value(&self) -> String {
        let mut directives = Vec::new();

        if self.public {
            directives.push("public".to_string());
        } else {
            directives.push("private".to_string());
        }

        if self.max_age > 0 {
            directives.push(format!("max-age={}", self.max_age));
        } else {
            directives.push("no-cache".to_string());
            directives.push("no-store".to_string());
        }

        if self.must_revalidate {
            directives.push("must-revalidate".to_string());
        }

        if let Some(s_maxage) = self.s_maxage {
            directives.push(format!("s-maxage={}", s_maxage));
        }

        if let Some(swr) = self.stale_while_revalidate {
            directives.push(format!("stale-while-revalidate={}", swr));
        }

        if let Some(sie) = self.stale_if_error {
            directives.push(format!("stale-if-error={}", sie));
        }

        directives.join(", ")
    }
}

/// Cache key generator for edge caching
pub struct CacheKeyGenerator;

impl CacheKeyGenerator {
    /// Generates a cache key from a request
    pub fn generate(req: &Request<Body>) -> String {
        let uri = req.uri();
        let query = uri.query().unwrap_or("");
        let path = uri.path();

        // Include relevant headers that affect response
        let mut key_parts = vec![path.to_string()];

        if !query.is_empty() {
            key_parts.push(query.to_string());
        }

        // Include Accept header for content negotiation
        if let Some(accept) = req.headers().get(header::ACCEPT) {
            if let Ok(accept_str) = accept.to_str() {
                key_parts.push(format!("accept:{}", accept_str));
            }
        }

        // Include Accept-Language for i18n
        if let Some(lang) = req.headers().get(header::ACCEPT_LANGUAGE) {
            if let Ok(lang_str) = lang.to_str() {
                key_parts.push(format!("lang:{}", lang_str));
            }
        }

        key_parts.join("|")
    }

    /// Generates a cache key with user context (for personalized content)
    pub fn generate_with_user(req: &Request<Body>, user_id: &str) -> String {
        let base_key = Self::generate(req);
        format!("{}|user:{}", base_key, user_id)
    }
}

/// Surrogate key header for cache invalidation
pub struct SurrogateKey {
    keys: Vec<String>,
}

impl SurrogateKey {
    /// Creates a new surrogate key
    pub fn new() -> Self {
        Self { keys: Vec::new() }
    }

    /// Adds a key
    pub fn add(&mut self, key: String) {
        self.keys.push(key);
    }

    /// Adds multiple keys
    pub fn add_all(&mut self, keys: Vec<String>) {
        self.keys.extend(keys);
    }

    /// Converts to header value
    pub fn to_header_value(&self) -> String {
        self.keys.join(" ")
    }
}

impl Default for SurrogateKey {
    fn default() -> Self {
        Self::new()
    }
}

/// Middleware to add cache headers to responses
pub async fn cache_headers_middleware(
    req: Request<Body>,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    let path = req.uri().path().to_string();
    let mut response = next.run(req).await;

    // Determine cache policy based on path
    let policy = if path.starts_with("/api/v1/statutes") && !path.contains("search") {
        // Statute data can be cached for a while
        CachePolicy::api_response(300) // 5 minutes
    } else if path.starts_with("/api/v1/verify") || path.starts_with("/api/v1/simulate") {
        // Verification and simulation results are dynamic
        CachePolicy::dynamic_content()
    } else if path.starts_with("/api/v1/health") || path.starts_with("/metrics") {
        // Health checks and metrics should not be cached
        CachePolicy::no_cache()
    } else if path.starts_with("/openapi") || path.starts_with("/docs") {
        // Documentation can be cached for a long time
        CachePolicy::static_resource()
    } else {
        // Default: short cache for API responses
        CachePolicy::api_response(60)
    };

    // Add Cache-Control header
    let cache_control = policy.to_header_value();
    if let Ok(header_value) = HeaderValue::from_str(&cache_control) {
        response
            .headers_mut()
            .insert(header::CACHE_CONTROL, header_value);
    }

    // Add Vary header to indicate which request headers affect the response
    if let Ok(vary_value) = HeaderValue::from_str("Accept, Accept-Language, Authorization") {
        response.headers_mut().insert(header::VARY, vary_value);
    }

    // Add CDN-specific headers
    // X-Cache-Status: for debugging cache behavior
    if let Ok(cache_status) = HeaderValue::from_str("MISS") {
        response
            .headers_mut()
            .insert("X-Cache-Status", cache_status);
    }

    Ok(response)
}

/// Edge cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeCacheConfig {
    /// Enable edge caching
    pub enabled: bool,
    /// Default TTL for cached responses
    pub default_ttl: Duration,
    /// Maximum cache size in bytes
    pub max_size: usize,
    /// Purge endpoint for cache invalidation
    pub purge_endpoint: Option<String>,
    /// CDN provider (cloudflare, fastly, cloudfront, etc.)
    pub cdn_provider: Option<String>,
}

impl Default for EdgeCacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            default_ttl: Duration::from_secs(300), // 5 minutes
            max_size: 1024 * 1024 * 1024,          // 1GB
            purge_endpoint: None,
            cdn_provider: None,
        }
    }
}

impl EdgeCacheConfig {
    /// Creates a new edge cache config from environment variables
    pub fn from_env() -> Self {
        Self {
            enabled: std::env::var("EDGE_CACHE_ENABLED")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
            default_ttl: Duration::from_secs(
                std::env::var("EDGE_CACHE_TTL")
                    .unwrap_or_else(|_| "300".to_string())
                    .parse()
                    .unwrap_or(300),
            ),
            max_size: std::env::var("EDGE_CACHE_MAX_SIZE")
                .unwrap_or_else(|_| "1073741824".to_string())
                .parse()
                .unwrap_or(1024 * 1024 * 1024),
            purge_endpoint: std::env::var("EDGE_CACHE_PURGE_ENDPOINT").ok(),
            cdn_provider: std::env::var("CDN_PROVIDER").ok(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_policy_static() {
        let policy = CachePolicy::static_resource();
        assert!(policy.public);
        assert_eq!(policy.max_age, 86400);
        assert!(policy.s_maxage.is_some());

        let header = policy.to_header_value();
        assert!(header.contains("public"));
        assert!(header.contains("max-age=86400"));
        assert!(header.contains("s-maxage=604800"));
    }

    #[test]
    fn test_cache_policy_dynamic() {
        let policy = CachePolicy::dynamic_content();
        assert!(policy.public);
        assert_eq!(policy.max_age, 60);
        assert!(policy.must_revalidate);

        let header = policy.to_header_value();
        assert!(header.contains("public"));
        assert!(header.contains("max-age=60"));
        assert!(header.contains("must-revalidate"));
    }

    #[test]
    fn test_cache_policy_no_cache() {
        let policy = CachePolicy::no_cache();
        assert!(!policy.public);
        assert_eq!(policy.max_age, 0);

        let header = policy.to_header_value();
        assert!(header.contains("private"));
        assert!(header.contains("no-cache"));
        assert!(header.contains("no-store"));
    }

    #[test]
    fn test_surrogate_key() {
        let mut key = SurrogateKey::new();
        key.add("statute-123".to_string());
        key.add("statute-456".to_string());

        let header = key.to_header_value();
        assert_eq!(header, "statute-123 statute-456");
    }

    #[test]
    fn test_edge_cache_config_default() {
        let config = EdgeCacheConfig::default();
        assert!(config.enabled);
        assert_eq!(config.default_ttl, Duration::from_secs(300));
        assert_eq!(config.max_size, 1024 * 1024 * 1024);
    }
}
