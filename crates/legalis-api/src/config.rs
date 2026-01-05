//! Configuration management for the API server.

use std::env;

/// API server configuration.
#[derive(Debug, Clone)]
pub struct Config {
    /// Host address to bind to
    pub host: String,
    /// Port to bind to
    pub port: u16,
    /// Enable structured logging
    pub structured_logging: bool,
    /// Log level (trace, debug, info, warn, error)
    pub log_level: String,
    /// CORS allowed origins (comma-separated)
    pub cors_origins: Option<String>,
    /// Maximum request body size in bytes
    pub max_body_size: usize,
    /// Request timeout in seconds
    pub request_timeout_secs: u64,
    /// Cache backend type (memory, redis)
    pub cache_backend: CacheBackend,
    /// Redis URL for cache (when using redis backend)
    pub redis_url: Option<String>,
    /// Default cache TTL in seconds
    pub cache_default_ttl: u64,
    /// Enable cache compression
    pub cache_compression: bool,
}

/// Cache backend type.
#[derive(Debug, Clone, PartialEq)]
pub enum CacheBackend {
    /// In-memory cache (default)
    Memory,
    /// Redis cache (requires redis-cache feature)
    Redis,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 3000,
            structured_logging: false,
            log_level: "info".to_string(),
            cors_origins: None,
            max_body_size: 10 * 1024 * 1024, // 10 MB
            request_timeout_secs: 30,
            cache_backend: CacheBackend::Memory,
            redis_url: None,
            cache_default_ttl: 300, // 5 minutes
            cache_compression: false,
        }
    }
}

impl Config {
    /// Load configuration from environment variables.
    pub fn from_env() -> Self {
        let mut config = Self::default();

        if let Ok(host) = env::var("LEGALIS_API_HOST") {
            config.host = host;
        }

        if let Ok(port) = env::var("LEGALIS_API_PORT") {
            if let Ok(p) = port.parse::<u16>() {
                config.port = p;
            }
        }

        if let Ok(structured) = env::var("LEGALIS_API_STRUCTURED_LOGGING") {
            config.structured_logging = structured.to_lowercase() == "true" || structured == "1";
        }

        if let Ok(level) = env::var("LEGALIS_API_LOG_LEVEL") {
            config.log_level = level;
        }

        if let Ok(origins) = env::var("LEGALIS_API_CORS_ORIGINS") {
            config.cors_origins = Some(origins);
        }

        if let Ok(size) = env::var("LEGALIS_API_MAX_BODY_SIZE") {
            if let Ok(s) = size.parse::<usize>() {
                config.max_body_size = s;
            }
        }

        if let Ok(timeout) = env::var("LEGALIS_API_REQUEST_TIMEOUT") {
            if let Ok(t) = timeout.parse::<u64>() {
                config.request_timeout_secs = t;
            }
        }

        // Cache configuration
        if let Ok(backend) = env::var("LEGALIS_API_CACHE_BACKEND") {
            config.cache_backend = match backend.to_lowercase().as_str() {
                "redis" => CacheBackend::Redis,
                "memory" => CacheBackend::Memory,
                _ => CacheBackend::Memory,
            };
        }

        if let Ok(url) = env::var("LEGALIS_API_REDIS_URL") {
            config.redis_url = Some(url);
        } else if let Ok(url) = env::var("REDIS_URL") {
            // Also check standard REDIS_URL env var
            config.redis_url = Some(url);
        }

        if let Ok(ttl) = env::var("LEGALIS_API_CACHE_TTL") {
            if let Ok(t) = ttl.parse::<u64>() {
                config.cache_default_ttl = t;
            }
        }

        if let Ok(compression) = env::var("LEGALIS_API_CACHE_COMPRESSION") {
            config.cache_compression = compression.to_lowercase() == "true" || compression == "1";
        }

        config
    }

    /// Returns the bind address.
    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
