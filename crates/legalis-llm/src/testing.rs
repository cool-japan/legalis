//! Testing utilities and infrastructure for LLM providers.
//!
//! This module provides comprehensive testing support including:
//! - Mock servers for integration testing
//! - Response fixtures for reproducible tests
//! - Performance benchmarking utilities
//! - Error handling test helpers
//! - Chaos testing for resilience verification

use crate::{LLMProvider, StreamChunk, TextStream};
use anyhow::Result;
use async_trait::async_trait;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// A recorded LLM response for fixture-based testing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseFixture {
    /// The prompt that generated this response
    pub prompt: String,
    /// The response text
    pub response: String,
    /// Provider name
    pub provider: String,
    /// Model name
    pub model: String,
    /// Timestamp when recorded
    pub recorded_at: String,
    /// Metadata (token counts, latency, etc.)
    pub metadata: HashMap<String, String>,
}

impl ResponseFixture {
    /// Creates a new response fixture.
    pub fn new(
        prompt: impl Into<String>,
        response: impl Into<String>,
        provider: impl Into<String>,
        model: impl Into<String>,
    ) -> Self {
        Self {
            prompt: prompt.into(),
            response: response.into(),
            provider: provider.into(),
            model: model.into(),
            recorded_at: chrono::Utc::now().to_rfc3339(),
            metadata: HashMap::new(),
        }
    }

    /// Adds metadata.
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Saves the fixture to a JSON file.
    pub fn save_to_file(&self, path: &str) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Loads a fixture from a JSON file.
    pub fn load_from_file(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let fixture = serde_json::from_str(&json)?;
        Ok(fixture)
    }
}

/// A collection of response fixtures.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixtureSet {
    /// Name of this fixture set
    pub name: String,
    /// Description
    pub description: Option<String>,
    /// Fixtures in this set
    pub fixtures: Vec<ResponseFixture>,
}

impl FixtureSet {
    /// Creates a new fixture set.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
            fixtures: Vec::new(),
        }
    }

    /// Sets the description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Adds a fixture to the set.
    pub fn add_fixture(mut self, fixture: ResponseFixture) -> Self {
        self.fixtures.push(fixture);
        self
    }

    /// Saves the fixture set to a JSON file.
    pub fn save_to_file(&self, path: &str) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Loads a fixture set from a JSON file.
    pub fn load_from_file(path: &str) -> Result<Self> {
        let json = std::fs::read_to_string(path)?;
        let fixture_set = serde_json::from_str(&json)?;
        Ok(fixture_set)
    }

    /// Finds a fixture by prompt (exact match).
    pub fn find_by_prompt(&self, prompt: &str) -> Option<&ResponseFixture> {
        self.fixtures.iter().find(|f| f.prompt == prompt)
    }

    /// Finds a fixture by prompt pattern (contains).
    pub fn find_by_pattern(&self, pattern: &str) -> Option<&ResponseFixture> {
        self.fixtures.iter().find(|f| f.prompt.contains(pattern))
    }
}

/// Provider that plays back recorded fixtures.
pub struct FixtureProvider {
    fixture_set: FixtureSet,
    provider_name: String,
    model_name: String,
}

impl FixtureProvider {
    /// Creates a new fixture provider.
    pub fn new(fixture_set: FixtureSet) -> Self {
        let provider_name = fixture_set
            .fixtures
            .first()
            .map(|f| f.provider.clone())
            .unwrap_or_else(|| "Fixture".to_string());

        let model_name = fixture_set
            .fixtures
            .first()
            .map(|f| f.model.clone())
            .unwrap_or_else(|| "fixture-v1".to_string());

        Self {
            fixture_set,
            provider_name,
            model_name,
        }
    }

    /// Loads fixtures from a file.
    pub fn from_file(path: &str) -> Result<Self> {
        let fixture_set = FixtureSet::load_from_file(path)?;
        Ok(Self::new(fixture_set))
    }
}

#[async_trait]
impl LLMProvider for FixtureProvider {
    async fn generate_text(&self, prompt: &str) -> Result<String> {
        // Try exact match first
        if let Some(fixture) = self.fixture_set.find_by_prompt(prompt) {
            return Ok(fixture.response.clone());
        }

        // Try pattern match
        for fixture in &self.fixture_set.fixtures {
            if prompt.contains(&fixture.prompt) || fixture.prompt.contains(prompt) {
                return Ok(fixture.response.clone());
            }
        }

        Err(anyhow::anyhow!("No fixture found for prompt: {}", prompt))
    }

    async fn generate_structured<T: serde::de::DeserializeOwned + Send>(
        &self,
        prompt: &str,
    ) -> Result<T> {
        let text = self.generate_text(prompt).await?;
        serde_json::from_str(&text)
            .map_err(|e| anyhow::anyhow!("Failed to parse fixture response: {}", e))
    }

    async fn generate_text_stream(&self, prompt: &str) -> Result<TextStream> {
        let text = self.generate_text(prompt).await?;

        // Split into word-based chunks for realistic streaming
        let words: Vec<String> = text.split_whitespace().map(|s| s.to_string()).collect();
        let chunks: Vec<Result<StreamChunk>> = words
            .into_iter()
            .enumerate()
            .map(|(i, word)| {
                let content = if i > 0 { format!(" {}", word) } else { word };
                Ok(StreamChunk::new(content))
            })
            .collect();

        Ok(Box::pin(futures::stream::iter(chunks)))
    }

    fn provider_name(&self) -> &str {
        &self.provider_name
    }

    fn model_name(&self) -> &str {
        &self.model_name
    }

    fn supports_streaming(&self) -> bool {
        true
    }
}

/// Performance metrics for LLM operations.
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    /// Request latency
    pub latency: Duration,
    /// Tokens per second (if available)
    pub tokens_per_second: Option<f64>,
    /// Total tokens processed
    pub total_tokens: Option<usize>,
    /// Time to first token (for streaming)
    pub time_to_first_token: Option<Duration>,
    /// Provider name
    pub provider: String,
    /// Model name
    pub model: String,
}

impl PerformanceMetrics {
    /// Creates a new metrics instance.
    pub fn new(provider: impl Into<String>, model: impl Into<String>, latency: Duration) -> Self {
        Self {
            latency,
            tokens_per_second: None,
            total_tokens: None,
            time_to_first_token: None,
            provider: provider.into(),
            model: model.into(),
        }
    }

    /// Sets tokens per second.
    pub fn with_tokens_per_second(mut self, tps: f64) -> Self {
        self.tokens_per_second = Some(tps);
        self
    }

    /// Sets total tokens.
    pub fn with_total_tokens(mut self, tokens: usize) -> Self {
        self.total_tokens = Some(tokens);
        self
    }

    /// Sets time to first token.
    pub fn with_time_to_first_token(mut self, ttft: Duration) -> Self {
        self.time_to_first_token = Some(ttft);
        self
    }
}

/// Wrapper that measures performance of LLM operations.
pub struct PerformanceMeasuringProvider<P> {
    provider: P,
    metrics: Arc<Mutex<Vec<PerformanceMetrics>>>,
}

impl<P: LLMProvider> PerformanceMeasuringProvider<P> {
    /// Creates a new performance measuring provider.
    pub fn new(provider: P) -> Self {
        Self {
            provider,
            metrics: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Gets all collected metrics.
    pub fn get_metrics(&self) -> Vec<PerformanceMetrics> {
        self.metrics.lock().unwrap().clone()
    }

    /// Gets average latency.
    pub fn average_latency(&self) -> Option<Duration> {
        let metrics = self.metrics.lock().unwrap();
        if metrics.is_empty() {
            return None;
        }

        let total: Duration = metrics.iter().map(|m| m.latency).sum();
        Some(total / metrics.len() as u32)
    }

    /// Gets p95 latency.
    pub fn p95_latency(&self) -> Option<Duration> {
        let mut metrics = self.metrics.lock().unwrap().clone();
        if metrics.is_empty() {
            return None;
        }

        metrics.sort_by_key(|m| m.latency);
        let idx = (metrics.len() as f64 * 0.95) as usize;
        Some(metrics[idx.min(metrics.len() - 1)].latency)
    }

    /// Clears all metrics.
    pub fn clear_metrics(&self) {
        self.metrics.lock().unwrap().clear();
    }
}

#[async_trait]
impl<P: LLMProvider> LLMProvider for PerformanceMeasuringProvider<P> {
    async fn generate_text(&self, prompt: &str) -> Result<String> {
        let start = Instant::now();
        let result = self.provider.generate_text(prompt).await;
        let latency = start.elapsed();

        let metrics = PerformanceMetrics::new(
            self.provider.provider_name(),
            self.provider.model_name(),
            latency,
        );

        self.metrics.lock().unwrap().push(metrics);

        result
    }

    async fn generate_structured<T: serde::de::DeserializeOwned + Send>(
        &self,
        prompt: &str,
    ) -> Result<T> {
        let start = Instant::now();
        let result = self.provider.generate_structured(prompt).await;
        let latency = start.elapsed();

        let metrics = PerformanceMetrics::new(
            self.provider.provider_name(),
            self.provider.model_name(),
            latency,
        );

        self.metrics.lock().unwrap().push(metrics);

        result
    }

    async fn generate_text_stream(&self, prompt: &str) -> Result<TextStream> {
        let start = Instant::now();
        let stream = self.provider.generate_text_stream(prompt).await?;

        let first_token_time = Arc::new(Mutex::new(None));
        let first_token_time_clone = first_token_time.clone();
        let metrics_clone = self.metrics.clone();
        let provider_name = self.provider.provider_name().to_string();
        let model_name = self.provider.model_name().to_string();

        let wrapped = stream.map(move |result| {
            if first_token_time_clone.lock().unwrap().is_none() {
                *first_token_time_clone.lock().unwrap() = Some(start.elapsed());
            }
            result
        });

        // Record metrics when stream completes
        let final_stream = wrapped.inspect(move |_| {
            if let Some(ttft) = *first_token_time.lock().unwrap() {
                let total_latency = start.elapsed();
                let metrics = PerformanceMetrics::new(&provider_name, &model_name, total_latency)
                    .with_time_to_first_token(ttft);
                metrics_clone.lock().unwrap().push(metrics);
            }
        });

        Ok(Box::pin(final_stream))
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

/// Chaos testing utilities for resilience verification.
pub mod chaos {
    use super::*;
    use rand::Rng;

    /// Chaos mode configuration.
    #[derive(Debug, Clone)]
    pub enum ChaosMode {
        /// Randomly fail requests
        RandomFailure { probability: f64 },
        /// Add random delays
        RandomDelay { min_ms: u64, max_ms: u64 },
        /// Corrupt responses
        CorruptResponse { probability: f64 },
        /// Timeout randomly
        RandomTimeout { probability: f64 },
        /// Combined chaos modes
        Combined(Vec<ChaosMode>),
    }

    /// Provider wrapper that injects chaos for testing resilience.
    pub struct ChaosProvider<P> {
        provider: P,
        mode: ChaosMode,
        failure_count: Arc<Mutex<usize>>,
    }

    impl<P: LLMProvider> ChaosProvider<P> {
        /// Creates a new chaos provider.
        pub fn new(provider: P, mode: ChaosMode) -> Self {
            Self {
                provider,
                mode,
                failure_count: Arc::new(Mutex::new(0)),
            }
        }

        /// Gets the number of induced failures.
        pub fn failure_count(&self) -> usize {
            *self.failure_count.lock().unwrap()
        }

        /// Resets the failure count.
        pub fn reset_failure_count(&self) {
            *self.failure_count.lock().unwrap() = 0;
        }

        async fn apply_chaos(&self) -> Result<()> {
            match &self.mode {
                ChaosMode::RandomFailure { probability } => {
                    if rand::rng().random_bool(*probability) {
                        *self.failure_count.lock().unwrap() += 1;
                        return Err(anyhow::anyhow!("Chaos-induced failure"));
                    }
                }
                ChaosMode::RandomDelay { min_ms, max_ms } => {
                    let delay_ms = rand::rng().random_range(*min_ms..=*max_ms);
                    tokio::time::sleep(Duration::from_millis(delay_ms)).await;
                }
                ChaosMode::RandomTimeout { probability } => {
                    if rand::rng().random_bool(*probability) {
                        *self.failure_count.lock().unwrap() += 1;
                        tokio::time::sleep(Duration::from_secs(30)).await;
                        return Err(anyhow::anyhow!("Chaos-induced timeout"));
                    }
                }
                ChaosMode::Combined(modes) => {
                    // Apply each mode sequentially
                    for mode in modes {
                        match mode {
                            ChaosMode::RandomFailure { probability } => {
                                if rand::rng().random_bool(*probability) {
                                    *self.failure_count.lock().unwrap() += 1;
                                    return Err(anyhow::anyhow!("Chaos-induced failure"));
                                }
                            }
                            ChaosMode::RandomDelay { min_ms, max_ms } => {
                                let delay_ms = rand::rng().random_range(*min_ms..=*max_ms);
                                tokio::time::sleep(Duration::from_millis(delay_ms)).await;
                            }
                            ChaosMode::RandomTimeout { probability } => {
                                if rand::rng().random_bool(*probability) {
                                    *self.failure_count.lock().unwrap() += 1;
                                    tokio::time::sleep(Duration::from_secs(30)).await;
                                    return Err(anyhow::anyhow!("Chaos-induced timeout"));
                                }
                            }
                            ChaosMode::CorruptResponse { .. } => {
                                // Skip corruption in apply_chaos, it's handled in corrupt_response
                            }
                            ChaosMode::Combined(_) => {
                                // Skip nested combined modes to avoid recursion
                            }
                        }
                    }
                }
                _ => {}
            }
            Ok(())
        }

        fn corrupt_response(&self, response: String) -> String {
            let should_corrupt = match &self.mode {
                ChaosMode::CorruptResponse { probability } => *probability,
                ChaosMode::Combined(modes) => {
                    // Check if any mode is CorruptResponse
                    modes
                        .iter()
                        .find_map(|mode| {
                            if let ChaosMode::CorruptResponse { probability } = mode {
                                Some(*probability)
                            } else {
                                None
                            }
                        })
                        .unwrap_or(0.0)
                }
                _ => 0.0,
            };

            if should_corrupt > 0.0 && rand::rng().random_bool(should_corrupt) {
                // Corrupt by removing random characters
                let mut chars: Vec<char> = response.chars().collect();
                if !chars.is_empty() {
                    let idx = rand::rng().random_range(0..chars.len());
                    chars.remove(idx);
                    return chars.into_iter().collect();
                }
            }
            response
        }
    }

    #[async_trait]
    impl<P: LLMProvider> LLMProvider for ChaosProvider<P> {
        async fn generate_text(&self, prompt: &str) -> Result<String> {
            self.apply_chaos().await?;
            let response = self.provider.generate_text(prompt).await?;
            Ok(self.corrupt_response(response))
        }

        async fn generate_structured<T: serde::de::DeserializeOwned + Send>(
            &self,
            prompt: &str,
        ) -> Result<T> {
            self.apply_chaos().await?;
            self.provider.generate_structured(prompt).await
        }

        async fn generate_text_stream(&self, prompt: &str) -> Result<TextStream> {
            self.apply_chaos().await?;
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
}

/// Error injection for testing error handling paths.
pub mod errors {
    use super::*;

    /// Types of errors to inject.
    #[derive(Debug, Clone)]
    pub enum ErrorType {
        /// Network timeout
        Timeout,
        /// Rate limit exceeded
        RateLimit,
        /// Invalid response format
        InvalidResponse,
        /// Authentication failure
        AuthFailure,
        /// Service unavailable
        ServiceUnavailable,
        /// Custom error
        Custom(String),
    }

    /// Provider that injects specific errors for testing.
    pub struct ErrorInjectingProvider<P> {
        provider: P,
        error_type: ErrorType,
        inject_on_call: usize,
        call_count: Arc<Mutex<usize>>,
    }

    impl<P: LLMProvider> ErrorInjectingProvider<P> {
        /// Creates a new error injecting provider.
        pub fn new(provider: P, error_type: ErrorType, inject_on_call: usize) -> Self {
            Self {
                provider,
                error_type,
                inject_on_call,
                call_count: Arc::new(Mutex::new(0)),
            }
        }

        fn check_and_inject_error(&self) -> Result<()> {
            let mut count = self.call_count.lock().unwrap();
            *count += 1;

            if *count == self.inject_on_call {
                return Err(match &self.error_type {
                    ErrorType::Timeout => anyhow::anyhow!("Request timeout"),
                    ErrorType::RateLimit => anyhow::anyhow!("Rate limit exceeded"),
                    ErrorType::InvalidResponse => anyhow::anyhow!("Invalid response format"),
                    ErrorType::AuthFailure => anyhow::anyhow!("Authentication failed"),
                    ErrorType::ServiceUnavailable => anyhow::anyhow!("Service unavailable"),
                    ErrorType::Custom(msg) => anyhow::anyhow!("{}", msg),
                });
            }

            Ok(())
        }

        /// Gets the current call count.
        pub fn call_count(&self) -> usize {
            *self.call_count.lock().unwrap()
        }
    }

    #[async_trait]
    impl<P: LLMProvider> LLMProvider for ErrorInjectingProvider<P> {
        async fn generate_text(&self, prompt: &str) -> Result<String> {
            self.check_and_inject_error()?;
            self.provider.generate_text(prompt).await
        }

        async fn generate_structured<T: serde::de::DeserializeOwned + Send>(
            &self,
            prompt: &str,
        ) -> Result<T> {
            self.check_and_inject_error()?;
            self.provider.generate_structured(prompt).await
        }

        async fn generate_text_stream(&self, prompt: &str) -> Result<TextStream> {
            self.check_and_inject_error()?;
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MockProvider;

    #[tokio::test]
    async fn test_fixture_provider() {
        let fixture_set = FixtureSet::new("test-set").add_fixture(ResponseFixture::new(
            "test prompt",
            r#"{"result": "success"}"#,
            "Test",
            "test-v1",
        ));

        let provider = FixtureProvider::new(fixture_set);
        let response = provider.generate_text("test prompt").await.unwrap();

        assert!(response.contains("success"));
    }

    #[tokio::test]
    async fn test_performance_measuring() {
        let mock = MockProvider::new().with_response("test", "response");
        let perf_provider = PerformanceMeasuringProvider::new(mock);

        perf_provider.generate_text("test").await.unwrap();
        perf_provider.generate_text("test").await.unwrap();

        let metrics = perf_provider.get_metrics();
        assert_eq!(metrics.len(), 2);

        let avg = perf_provider.average_latency();
        assert!(avg.is_some());
    }

    #[tokio::test]
    async fn test_chaos_random_failure() {
        let mock = MockProvider::new().with_response("test", "response");
        let chaos = chaos::ChaosProvider::new(
            mock,
            chaos::ChaosMode::RandomFailure { probability: 1.0 }, // Always fail
        );

        let result = chaos.generate_text("test").await;
        assert!(result.is_err());
        assert_eq!(chaos.failure_count(), 1);
    }

    #[tokio::test]
    async fn test_error_injection() {
        let mock = MockProvider::new().with_response("test", "response");
        let error_provider = errors::ErrorInjectingProvider::new(
            mock,
            errors::ErrorType::RateLimit,
            2, // Inject error on second call
        );

        // First call should succeed
        let result1 = error_provider.generate_text("test").await;
        assert!(result1.is_ok());

        // Second call should fail
        let result2 = error_provider.generate_text("test").await;
        assert!(result2.is_err());
        assert!(result2.unwrap_err().to_string().contains("Rate limit"));
    }
}
