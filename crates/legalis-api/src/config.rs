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

        config
    }

    /// Returns the bind address.
    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
