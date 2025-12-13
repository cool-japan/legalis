//! Rate limiting middleware for API endpoints.
//!
//! Provides configurable rate limiting to prevent abuse and ensure fair API usage.
//! Uses the token bucket algorithm via the `governor` crate.

use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use governor::{
    Quota, RateLimiter,
    clock::DefaultClock,
    state::{InMemoryState, NotKeyed},
};
use std::{num::NonZeroU32, sync::Arc, time::Duration};

/// Rate limiter configuration.
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Maximum number of requests per period
    pub requests_per_period: u32,
    /// Time period for the rate limit
    pub period: Duration,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_period: 100,
            period: Duration::from_secs(60),
        }
    }
}

impl RateLimitConfig {
    /// Creates a new rate limit configuration.
    pub fn new(requests_per_period: u32, period: Duration) -> Self {
        Self {
            requests_per_period,
            period,
        }
    }

    /// Creates a configuration for N requests per second.
    pub fn per_second(requests: u32) -> Self {
        Self {
            requests_per_period: requests,
            period: Duration::from_secs(1),
        }
    }

    /// Creates a configuration for N requests per minute.
    pub fn per_minute(requests: u32) -> Self {
        Self {
            requests_per_period: requests,
            period: Duration::from_secs(60),
        }
    }

    /// Creates a configuration for N requests per hour.
    pub fn per_hour(requests: u32) -> Self {
        Self {
            requests_per_period: requests,
            period: Duration::from_secs(3600),
        }
    }
}

/// Shared rate limiter state.
pub type SharedRateLimiter = Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock>>;

/// Creates a rate limiter from configuration.
pub fn create_rate_limiter(config: &RateLimitConfig) -> SharedRateLimiter {
    let quota = Quota::with_period(config.period)
        .expect("Invalid rate limit period")
        .allow_burst(NonZeroU32::new(config.requests_per_period).expect("Invalid burst size"));

    Arc::new(RateLimiter::direct(quota))
}

/// Rate limit middleware for Axum.
pub async fn rate_limit_middleware(
    rate_limiter: SharedRateLimiter,
    request: Request,
    next: Next,
) -> Response {
    match rate_limiter.check() {
        Ok(_) => next.run(request).await,
        Err(_not_until) => (
            StatusCode::TOO_MANY_REQUESTS,
            "Rate limit exceeded. Please try again later.",
        )
            .into_response(),
    }
}

/// Extension trait for adding rate limiting to routers.
pub trait RateLimitExt {
    /// Adds rate limiting middleware to the router.
    fn with_rate_limit(self, config: RateLimitConfig) -> Self;
}

impl RateLimitExt for axum::Router {
    fn with_rate_limit(self, config: RateLimitConfig) -> Self {
        let limiter = create_rate_limiter(&config);
        self.layer(axum::middleware::from_fn(move |req, next| {
            rate_limit_middleware(limiter.clone(), req, next)
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limit_config_default() {
        let config = RateLimitConfig::default();
        assert_eq!(config.requests_per_period, 100);
        assert_eq!(config.period, Duration::from_secs(60));
    }

    #[test]
    fn test_rate_limit_config_per_second() {
        let config = RateLimitConfig::per_second(10);
        assert_eq!(config.requests_per_period, 10);
        assert_eq!(config.period, Duration::from_secs(1));
    }

    #[test]
    fn test_rate_limit_config_per_minute() {
        let config = RateLimitConfig::per_minute(100);
        assert_eq!(config.requests_per_period, 100);
        assert_eq!(config.period, Duration::from_secs(60));
    }

    #[test]
    fn test_rate_limit_config_per_hour() {
        let config = RateLimitConfig::per_hour(1000);
        assert_eq!(config.requests_per_period, 1000);
        assert_eq!(config.period, Duration::from_secs(3600));
    }

    #[test]
    fn test_create_rate_limiter() {
        let config = RateLimitConfig::per_second(10);
        let limiter = create_rate_limiter(&config);

        // First request should succeed
        assert!(limiter.check().is_ok());
    }

    #[tokio::test]
    async fn test_rate_limiter_allows_within_limit() {
        let config = RateLimitConfig::per_second(5);
        let limiter = create_rate_limiter(&config);

        // First 5 requests should succeed
        for _ in 0..5 {
            assert!(limiter.check().is_ok());
        }
    }

    #[tokio::test]
    async fn test_rate_limiter_blocks_over_limit() {
        let config = RateLimitConfig::per_second(2);
        let limiter = create_rate_limiter(&config);

        // First 2 should succeed
        assert!(limiter.check().is_ok());
        assert!(limiter.check().is_ok());

        // Third should fail
        assert!(limiter.check().is_err());
    }
}
