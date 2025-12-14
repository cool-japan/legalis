//! Resilience patterns for LLM providers.
//!
//! This module implements reliability patterns including:
//! - Retry with exponential backoff
//! - Provider fallback chain
//! - Circuit breaker
//! - Rate limiting

use crate::{LLMProvider, TextStream};
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use tokio::time::sleep;

/// Configuration for retry behavior.
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of retry attempts
    pub max_attempts: usize,
    /// Initial delay before first retry
    pub initial_delay: Duration,
    /// Maximum delay between retries
    pub max_delay: Duration,
    /// Backoff multiplier (e.g., 2.0 for doubling)
    pub backoff_multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(10),
            backoff_multiplier: 2.0,
        }
    }
}

impl RetryConfig {
    /// Creates a new retry configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the maximum number of retry attempts.
    pub fn with_max_attempts(mut self, max_attempts: usize) -> Self {
        self.max_attempts = max_attempts;
        self
    }

    /// Sets the initial delay.
    pub fn with_initial_delay(mut self, delay: Duration) -> Self {
        self.initial_delay = delay;
        self
    }

    /// Sets the maximum delay.
    pub fn with_max_delay(mut self, delay: Duration) -> Self {
        self.max_delay = delay;
        self
    }

    /// Sets the backoff multiplier.
    pub fn with_backoff_multiplier(mut self, multiplier: f64) -> Self {
        self.backoff_multiplier = multiplier;
        self
    }

    /// Calculates the delay for a given attempt number.
    pub fn delay_for_attempt(&self, attempt: usize) -> Duration {
        if attempt == 0 {
            return Duration::from_secs(0);
        }

        let delay_ms = self.initial_delay.as_millis() as f64
            * self.backoff_multiplier.powi((attempt - 1) as i32);
        let delay = Duration::from_millis(delay_ms as u64);

        delay.min(self.max_delay)
    }
}

/// Wraps an LLM provider with retry logic and exponential backoff.
pub struct RetryProvider<P> {
    provider: P,
    config: RetryConfig,
}

impl<P> RetryProvider<P> {
    /// Creates a new retry provider with default configuration.
    pub fn new(provider: P) -> Self {
        Self {
            provider,
            config: RetryConfig::default(),
        }
    }

    /// Creates a new retry provider with custom configuration.
    pub fn with_config(provider: P, config: RetryConfig) -> Self {
        Self { provider, config }
    }

    /// Gets a reference to the underlying provider.
    pub fn provider(&self) -> &P {
        &self.provider
    }
}

#[async_trait]
impl<P: LLMProvider> LLMProvider for RetryProvider<P> {
    async fn generate_text(&self, prompt: &str) -> Result<String> {
        let mut last_error = None;

        for attempt in 0..self.config.max_attempts {
            if attempt > 0 {
                let delay = self.config.delay_for_attempt(attempt);
                tracing::debug!(
                    "Retrying after {:?} (attempt {}/{})",
                    delay,
                    attempt + 1,
                    self.config.max_attempts
                );
                sleep(delay).await;
            }

            match self.provider.generate_text(prompt).await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    tracing::warn!(
                        "Request failed (attempt {}/{}): {}",
                        attempt + 1,
                        self.config.max_attempts,
                        e
                    );
                    last_error = Some(e);
                }
            }
        }

        Err(last_error.unwrap_or_else(|| anyhow!("All retry attempts exhausted")))
    }

    async fn generate_structured<T: DeserializeOwned + Send>(&self, prompt: &str) -> Result<T> {
        let mut last_error = None;

        for attempt in 0..self.config.max_attempts {
            if attempt > 0 {
                let delay = self.config.delay_for_attempt(attempt);
                tracing::debug!(
                    "Retrying after {:?} (attempt {}/{})",
                    delay,
                    attempt + 1,
                    self.config.max_attempts
                );
                sleep(delay).await;
            }

            match self.provider.generate_structured::<T>(prompt).await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    tracing::warn!(
                        "Structured request failed (attempt {}/{}): {}",
                        attempt + 1,
                        self.config.max_attempts,
                        e
                    );
                    last_error = Some(e);
                }
            }
        }

        Err(last_error.unwrap_or_else(|| anyhow!("All retry attempts exhausted")))
    }

    async fn generate_text_stream(&self, prompt: &str) -> Result<TextStream> {
        // Streaming requests are not retried
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

/// A provider that tries two providers in sequence, falling back to the second if the first fails.
pub struct FallbackProvider<P1, P2> {
    primary: P1,
    fallback: P2,
}

impl<P1, P2> FallbackProvider<P1, P2> {
    /// Creates a new fallback provider with primary and fallback providers.
    pub fn new(primary: P1, fallback: P2) -> Self {
        Self { primary, fallback }
    }

    /// Gets a reference to the primary provider.
    pub fn primary(&self) -> &P1 {
        &self.primary
    }

    /// Gets a reference to the fallback provider.
    pub fn fallback(&self) -> &P2 {
        &self.fallback
    }
}

#[async_trait]
impl<P1: LLMProvider, P2: LLMProvider> LLMProvider for FallbackProvider<P1, P2> {
    async fn generate_text(&self, prompt: &str) -> Result<String> {
        match self.primary.generate_text(prompt).await {
            Ok(result) => Ok(result),
            Err(e) => {
                tracing::warn!("Primary provider failed: {}, trying fallback", e);
                self.fallback.generate_text(prompt).await
            }
        }
    }

    async fn generate_structured<T: DeserializeOwned + Send>(&self, prompt: &str) -> Result<T> {
        match self.primary.generate_structured::<T>(prompt).await {
            Ok(result) => Ok(result),
            Err(e) => {
                tracing::warn!("Primary provider failed: {}, trying fallback", e);
                self.fallback.generate_structured::<T>(prompt).await
            }
        }
    }

    async fn generate_text_stream(&self, prompt: &str) -> Result<TextStream> {
        match self.primary.generate_text_stream(prompt).await {
            Ok(result) => Ok(result),
            Err(e) => {
                tracing::warn!("Primary provider failed: {}, trying fallback", e);
                self.fallback.generate_text_stream(prompt).await
            }
        }
    }

    fn provider_name(&self) -> &str {
        "Fallback"
    }

    fn model_name(&self) -> &str {
        self.primary.model_name()
    }

    fn supports_streaming(&self) -> bool {
        self.primary.supports_streaming() || self.fallback.supports_streaming()
    }
}

/// Circuit breaker states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircuitState {
    /// Circuit is closed, requests pass through
    Closed,
    /// Circuit is open, requests fail fast
    Open,
    /// Circuit is half-open, testing if service recovered
    HalfOpen,
}

/// Configuration for circuit breaker.
#[derive(Debug, Clone)]
pub struct CircuitBreakerConfig {
    /// Number of failures before opening circuit
    pub failure_threshold: usize,
    /// Duration to wait before transitioning from Open to HalfOpen
    pub timeout: Duration,
    /// Number of successes to close circuit from HalfOpen
    pub success_threshold: usize,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            timeout: Duration::from_secs(60),
            success_threshold: 2,
        }
    }
}

impl CircuitBreakerConfig {
    /// Creates a new circuit breaker configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the failure threshold.
    pub fn with_failure_threshold(mut self, threshold: usize) -> Self {
        self.failure_threshold = threshold;
        self
    }

    /// Sets the timeout.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Sets the success threshold.
    pub fn with_success_threshold(mut self, threshold: usize) -> Self {
        self.success_threshold = threshold;
        self
    }
}

/// Circuit breaker for LLM providers.
pub struct CircuitBreaker<P> {
    provider: P,
    config: CircuitBreakerConfig,
    state: Arc<Mutex<CircuitBreakerState>>,
}

struct CircuitBreakerState {
    state: CircuitState,
    failure_count: usize,
    success_count: usize,
    last_failure_time: Option<Instant>,
}

impl<P> CircuitBreaker<P> {
    /// Creates a new circuit breaker with default configuration.
    pub fn new(provider: P) -> Self {
        Self::with_config(provider, CircuitBreakerConfig::default())
    }

    /// Creates a new circuit breaker with custom configuration.
    pub fn with_config(provider: P, config: CircuitBreakerConfig) -> Self {
        Self {
            provider,
            config,
            state: Arc::new(Mutex::new(CircuitBreakerState {
                state: CircuitState::Closed,
                failure_count: 0,
                success_count: 0,
                last_failure_time: None,
            })),
        }
    }

    /// Gets the current circuit state.
    pub async fn state(&self) -> CircuitState {
        self.state.lock().await.state
    }

    /// Resets the circuit breaker to closed state.
    pub async fn reset(&self) {
        let mut state = self.state.lock().await;
        state.state = CircuitState::Closed;
        state.failure_count = 0;
        state.success_count = 0;
        state.last_failure_time = None;
    }

    async fn on_success(&self) {
        let mut state = self.state.lock().await;

        match state.state {
            CircuitState::HalfOpen => {
                state.success_count += 1;
                if state.success_count >= self.config.success_threshold {
                    tracing::info!("Circuit breaker transitioning to Closed");
                    state.state = CircuitState::Closed;
                    state.failure_count = 0;
                    state.success_count = 0;
                }
            }
            CircuitState::Closed => {
                // Reset failure count on success
                state.failure_count = 0;
            }
            CircuitState::Open => {}
        }
    }

    async fn on_failure(&self) {
        let mut state = self.state.lock().await;

        match state.state {
            CircuitState::Closed => {
                state.failure_count += 1;
                if state.failure_count >= self.config.failure_threshold {
                    tracing::warn!("Circuit breaker transitioning to Open");
                    state.state = CircuitState::Open;
                    state.last_failure_time = Some(Instant::now());
                }
            }
            CircuitState::HalfOpen => {
                tracing::warn!("Circuit breaker transitioning back to Open");
                state.state = CircuitState::Open;
                state.last_failure_time = Some(Instant::now());
                state.success_count = 0;
            }
            CircuitState::Open => {
                state.last_failure_time = Some(Instant::now());
            }
        }
    }

    async fn check_and_update_state(&self) -> Result<()> {
        let mut state = self.state.lock().await;

        if state.state == CircuitState::Open {
            if let Some(last_failure) = state.last_failure_time {
                if last_failure.elapsed() >= self.config.timeout {
                    tracing::info!("Circuit breaker transitioning to HalfOpen");
                    state.state = CircuitState::HalfOpen;
                    state.success_count = 0;
                } else {
                    return Err(anyhow!("Circuit breaker is open"));
                }
            }
        }

        Ok(())
    }

    /// Gets a reference to the underlying provider.
    pub fn provider(&self) -> &P {
        &self.provider
    }
}

#[async_trait]
impl<P: LLMProvider> LLMProvider for CircuitBreaker<P> {
    async fn generate_text(&self, prompt: &str) -> Result<String> {
        self.check_and_update_state().await?;

        match self.provider.generate_text(prompt).await {
            Ok(result) => {
                self.on_success().await;
                Ok(result)
            }
            Err(e) => {
                self.on_failure().await;
                Err(e)
            }
        }
    }

    async fn generate_structured<T: DeserializeOwned + Send>(&self, prompt: &str) -> Result<T> {
        self.check_and_update_state().await?;

        match self.provider.generate_structured::<T>(prompt).await {
            Ok(result) => {
                self.on_success().await;
                Ok(result)
            }
            Err(e) => {
                self.on_failure().await;
                Err(e)
            }
        }
    }

    async fn generate_text_stream(&self, prompt: &str) -> Result<TextStream> {
        self.check_and_update_state().await?;

        match self.provider.generate_text_stream(prompt).await {
            Ok(result) => {
                self.on_success().await;
                Ok(result)
            }
            Err(e) => {
                self.on_failure().await;
                Err(e)
            }
        }
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

/// Configuration for rate limiting.
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Maximum requests per time window
    pub max_requests: usize,
    /// Time window for rate limiting
    pub window: Duration,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_requests: 60,
            window: Duration::from_secs(60),
        }
    }
}

impl RateLimitConfig {
    /// Creates a new rate limit configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the maximum requests.
    pub fn with_max_requests(mut self, max_requests: usize) -> Self {
        self.max_requests = max_requests;
        self
    }

    /// Sets the time window.
    pub fn with_window(mut self, window: Duration) -> Self {
        self.window = window;
        self
    }
}

/// Rate limiter for LLM providers using a token bucket algorithm.
pub struct RateLimiter<P> {
    provider: P,
    config: RateLimitConfig,
    tokens: Arc<AtomicUsize>,
    last_refill: Arc<AtomicU64>,
}

impl<P> RateLimiter<P> {
    /// Creates a new rate limiter with default configuration.
    pub fn new(provider: P) -> Self {
        Self::with_config(provider, RateLimitConfig::default())
    }

    /// Creates a new rate limiter with custom configuration.
    pub fn with_config(provider: P, config: RateLimitConfig) -> Self {
        Self {
            provider,
            tokens: Arc::new(AtomicUsize::new(config.max_requests)),
            last_refill: Arc::new(AtomicU64::new(Instant::now().elapsed().as_millis() as u64)),
            config,
        }
    }

    async fn acquire_token(&self) -> Result<()> {
        loop {
            // Refill tokens if window has passed
            let now = Instant::now().elapsed().as_millis() as u64;
            let last = self.last_refill.load(Ordering::Relaxed);
            let window_ms = self.config.window.as_millis() as u64;

            if now - last >= window_ms {
                // Try to refill
                if self
                    .last_refill
                    .compare_exchange(last, now, Ordering::SeqCst, Ordering::Relaxed)
                    .is_ok()
                {
                    self.tokens
                        .store(self.config.max_requests, Ordering::Relaxed);
                }
            }

            // Try to acquire a token
            let current = self.tokens.load(Ordering::Relaxed);
            if current > 0 {
                if self
                    .tokens
                    .compare_exchange(current, current - 1, Ordering::SeqCst, Ordering::Relaxed)
                    .is_ok()
                {
                    return Ok(());
                }
            } else {
                // No tokens available, wait
                let wait_time = window_ms - (now - self.last_refill.load(Ordering::Relaxed));
                tracing::debug!("Rate limit exceeded, waiting {}ms", wait_time);
                sleep(Duration::from_millis(wait_time)).await;
            }
        }
    }

    /// Gets a reference to the underlying provider.
    pub fn provider(&self) -> &P {
        &self.provider
    }
}

#[async_trait]
impl<P: LLMProvider> LLMProvider for RateLimiter<P> {
    async fn generate_text(&self, prompt: &str) -> Result<String> {
        self.acquire_token().await?;
        self.provider.generate_text(prompt).await
    }

    async fn generate_structured<T: DeserializeOwned + Send>(&self, prompt: &str) -> Result<T> {
        self.acquire_token().await?;
        self.provider.generate_structured::<T>(prompt).await
    }

    async fn generate_text_stream(&self, prompt: &str) -> Result<TextStream> {
        self.acquire_token().await?;
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

/// Health check result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HealthStatus {
    /// Provider is healthy
    Healthy,
    /// Provider is degraded but functional
    Degraded,
    /// Provider is unhealthy
    Unhealthy,
}

/// Health check statistics.
#[derive(Debug, Clone)]
pub struct HealthStats {
    /// Current health status
    pub status: HealthStatus,
    /// Total health checks performed
    pub total_checks: usize,
    /// Successful health checks
    pub successful_checks: usize,
    /// Failed health checks
    pub failed_checks: usize,
    /// Last check time
    pub last_check: Option<Instant>,
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,
}

impl HealthStats {
    /// Creates new health statistics.
    pub fn new() -> Self {
        Self {
            status: HealthStatus::Healthy,
            total_checks: 0,
            successful_checks: 0,
            failed_checks: 0,
            last_check: None,
            avg_response_time_ms: 0.0,
        }
    }

    /// Calculates the success rate.
    pub fn success_rate(&self) -> f64 {
        if self.total_checks == 0 {
            0.0
        } else {
            (self.successful_checks as f64 / self.total_checks as f64) * 100.0
        }
    }
}

impl Default for HealthStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Health checker for LLM providers.
pub struct HealthChecker<P> {
    provider: P,
    stats: Arc<Mutex<HealthStats>>,
    health_check_prompt: String,
}

impl<P> HealthChecker<P> {
    /// Creates a new health checker with default prompt.
    pub fn new(provider: P) -> Self {
        Self::with_prompt(provider, "Hello".to_string())
    }

    /// Creates a new health checker with custom health check prompt.
    pub fn with_prompt(provider: P, prompt: impl Into<String>) -> Self {
        Self {
            provider,
            stats: Arc::new(Mutex::new(HealthStats::new())),
            health_check_prompt: prompt.into(),
        }
    }

    /// Gets the current health statistics.
    pub async fn stats(&self) -> HealthStats {
        self.stats.lock().await.clone()
    }

    /// Resets the health statistics.
    pub async fn reset_stats(&self) {
        let mut stats = self.stats.lock().await;
        *stats = HealthStats::new();
    }

    /// Gets a reference to the underlying provider.
    pub fn provider(&self) -> &P {
        &self.provider
    }

    /// Performs a health check on the provider.
    pub async fn check_health(&self) -> HealthStatus
    where
        P: LLMProvider,
    {
        let start = Instant::now();
        let mut stats = self.stats.lock().await;

        stats.total_checks += 1;
        stats.last_check = Some(Instant::now());

        drop(stats); // Release lock before making the request

        let result = self.provider.generate_text(&self.health_check_prompt).await;
        let response_time = start.elapsed();

        let mut stats = self.stats.lock().await;

        match result {
            Ok(_) => {
                stats.successful_checks += 1;

                // Update average response time
                let total_time = stats.avg_response_time_ms * (stats.successful_checks - 1) as f64
                    + response_time.as_millis() as f64;
                stats.avg_response_time_ms = total_time / stats.successful_checks as f64;

                // Determine health based on response time and success rate
                if response_time.as_millis() < 1000 && stats.success_rate() > 95.0 {
                    stats.status = HealthStatus::Healthy;
                } else if response_time.as_millis() < 5000 && stats.success_rate() > 80.0 {
                    stats.status = HealthStatus::Degraded;
                } else {
                    stats.status = HealthStatus::Unhealthy;
                }
            }
            Err(e) => {
                stats.failed_checks += 1;
                tracing::warn!("Health check failed: {}", e);

                // Update status based on failure rate
                if stats.success_rate() < 50.0 {
                    stats.status = HealthStatus::Unhealthy;
                } else if stats.success_rate() < 80.0 {
                    stats.status = HealthStatus::Degraded;
                }
            }
        }

        stats.status.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MockProvider;

    #[test]
    fn test_retry_config() {
        let config = RetryConfig::new()
            .with_max_attempts(5)
            .with_initial_delay(Duration::from_millis(50))
            .with_backoff_multiplier(3.0);

        assert_eq!(config.max_attempts, 5);
        assert_eq!(config.delay_for_attempt(0), Duration::from_secs(0));
        assert_eq!(config.delay_for_attempt(1), Duration::from_millis(50));
        assert_eq!(config.delay_for_attempt(2), Duration::from_millis(150));
        assert_eq!(config.delay_for_attempt(3), Duration::from_millis(450));
    }

    #[tokio::test]
    async fn test_retry_provider_success() {
        let provider = MockProvider::new().with_response("test", "success");
        let retry_provider = RetryProvider::new(provider);

        let result = retry_provider.generate_text("test prompt").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_fallback_provider() {
        let primary = MockProvider::new().with_response("test", "primary response");
        let fallback_prov = MockProvider::new().with_response("test", "fallback response");

        let provider = FallbackProvider::new(primary, fallback_prov);

        let result = provider.generate_text("test prompt").await;
        assert!(result.is_ok());
        assert!(result.unwrap().contains("primary"));
    }

    #[tokio::test]
    async fn test_circuit_breaker_open() {
        let config = CircuitBreakerConfig::new()
            .with_failure_threshold(2)
            .with_timeout(Duration::from_millis(100));

        let provider = MockProvider::new();
        let breaker = CircuitBreaker::with_config(provider, config);

        // Initially closed
        assert_eq!(breaker.state().await, CircuitState::Closed);

        // Simulate failures
        let _ = breaker.generate_text("nonexistent").await;
        let _ = breaker.generate_text("nonexistent").await;

        // Should be open now
        assert_eq!(breaker.state().await, CircuitState::Open);

        // Wait for timeout
        sleep(Duration::from_millis(150)).await;

        // Should transition to half-open on next request
        let _ = breaker.generate_text("test").await;
        // After check_and_update_state, it should be HalfOpen
    }

    #[tokio::test]
    async fn test_rate_limiter() {
        let config = RateLimitConfig::new()
            .with_max_requests(2)
            .with_window(Duration::from_millis(100));

        let provider = MockProvider::new().with_response("test", "response");
        let limiter = RateLimiter::with_config(provider, config);

        // First two requests should succeed immediately
        assert!(limiter.generate_text("test").await.is_ok());
        assert!(limiter.generate_text("test").await.is_ok());

        // Third request should be delayed
        let start = Instant::now();
        assert!(limiter.generate_text("test").await.is_ok());
        let elapsed = start.elapsed();

        // Should have waited for the window to refill
        assert!(elapsed >= Duration::from_millis(90));
    }

    #[tokio::test]
    async fn test_health_checker() {
        let provider = MockProvider::new().with_response("Hello", "Hi there!");
        let checker = HealthChecker::new(provider);

        let status = checker.check_health().await;
        assert_eq!(status, HealthStatus::Healthy);

        let stats = checker.stats().await;
        assert_eq!(stats.total_checks, 1);
        assert_eq!(stats.successful_checks, 1);
        assert_eq!(stats.success_rate(), 100.0);
    }
}
