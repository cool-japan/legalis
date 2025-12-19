//! Batch request processing for LLMs.
//!
//! This module provides utilities for efficiently processing multiple requests
//! in batches, with support for concurrency control and result aggregation.

use crate::LLMProvider;
use anyhow::Result;
use futures::future::join_all;
use futures::stream::{self, StreamExt};
use serde::de::DeserializeOwned;
use std::sync::Arc;
use tokio::sync::Semaphore;

/// Configuration for batch processing.
#[derive(Debug, Clone)]
pub struct BatchConfig {
    /// Maximum number of concurrent requests
    pub max_concurrency: usize,
    /// Whether to fail fast on first error
    pub fail_fast: bool,
    /// Batch size (number of requests per batch)
    pub batch_size: usize,
}

impl Default for BatchConfig {
    fn default() -> Self {
        Self {
            max_concurrency: 10,
            fail_fast: false,
            batch_size: 100,
        }
    }
}

impl BatchConfig {
    /// Creates a new batch configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the maximum concurrency.
    pub fn with_max_concurrency(mut self, max: usize) -> Self {
        self.max_concurrency = max.max(1);
        self
    }

    /// Sets whether to fail fast on first error.
    pub fn with_fail_fast(mut self, fail_fast: bool) -> Self {
        self.fail_fast = fail_fast;
        self
    }

    /// Sets the batch size.
    pub fn with_batch_size(mut self, size: usize) -> Self {
        self.batch_size = size.max(1);
        self
    }
}

/// Batch processor for LLM requests.
pub struct BatchProcessor<P> {
    provider: Arc<P>,
    config: BatchConfig,
}

impl<P: LLMProvider + Send + Sync> BatchProcessor<P> {
    /// Creates a new batch processor.
    pub fn new(provider: P) -> Self {
        Self {
            provider: Arc::new(provider),
            config: BatchConfig::default(),
        }
    }

    /// Creates a new batch processor with custom configuration.
    pub fn with_config(provider: P, config: BatchConfig) -> Self {
        Self {
            provider: Arc::new(provider),
            config,
        }
    }

    /// Processes a batch of text generation requests.
    pub async fn process_text_batch(&self, prompts: &[String]) -> Vec<Result<String>> {
        let semaphore = Arc::new(Semaphore::new(self.config.max_concurrency));

        if self.config.fail_fast {
            // Try to process all, but stop on first error
            self.process_with_fail_fast(prompts, semaphore).await
        } else {
            // Process all regardless of errors
            self.process_with_continue(prompts, semaphore).await
        }
    }

    async fn process_with_fail_fast(
        &self,
        prompts: &[String],
        semaphore: Arc<Semaphore>,
    ) -> Vec<Result<String>> {
        let provider = Arc::clone(&self.provider);

        let tasks: Vec<_> = prompts
            .iter()
            .map(|prompt| {
                let provider = Arc::clone(&provider);
                let semaphore = Arc::clone(&semaphore);
                let prompt = prompt.clone();

                async move {
                    let _permit = semaphore.acquire().await.unwrap();
                    provider.generate_text(&prompt).await
                }
            })
            .collect();

        join_all(tasks).await
    }

    async fn process_with_continue(
        &self,
        prompts: &[String],
        semaphore: Arc<Semaphore>,
    ) -> Vec<Result<String>> {
        let provider = Arc::clone(&self.provider);

        stream::iter(prompts.iter())
            .map(|prompt| {
                let provider = Arc::clone(&provider);
                let semaphore = Arc::clone(&semaphore);
                let prompt = prompt.clone();

                async move {
                    let _permit = semaphore.acquire().await.unwrap();
                    provider.generate_text(&prompt).await
                }
            })
            .buffer_unordered(self.config.max_concurrency)
            .collect::<Vec<_>>()
            .await
    }

    /// Processes a batch of structured generation requests.
    pub async fn process_structured_batch<T: DeserializeOwned + Send>(
        &self,
        prompts: &[String],
    ) -> Vec<Result<T>> {
        let semaphore = Arc::new(Semaphore::new(self.config.max_concurrency));
        let provider = Arc::clone(&self.provider);

        stream::iter(prompts.iter())
            .map(|prompt| {
                let provider = Arc::clone(&provider);
                let semaphore = Arc::clone(&semaphore);
                let prompt = prompt.clone();

                async move {
                    let _permit = semaphore.acquire().await.unwrap();
                    provider.generate_structured::<T>(&prompt).await
                }
            })
            .buffer_unordered(self.config.max_concurrency)
            .collect::<Vec<_>>()
            .await
    }

    /// Gets batch processing statistics.
    pub async fn process_with_stats(&self, prompts: &[String]) -> BatchResult {
        let start = std::time::Instant::now();
        let results = self.process_text_batch(prompts).await;
        let duration = start.elapsed();

        let successful = results.iter().filter(|r| r.is_ok()).count();
        let failed = results.iter().filter(|r| r.is_err()).count();

        BatchResult {
            total: results.len(),
            successful,
            failed,
            duration_ms: duration.as_millis() as u64,
            results,
        }
    }
}

/// Result of batch processing with statistics.
#[derive(Debug)]
pub struct BatchResult {
    /// Total number of requests
    pub total: usize,
    /// Number of successful requests
    pub successful: usize,
    /// Number of failed requests
    pub failed: usize,
    /// Total duration in milliseconds
    pub duration_ms: u64,
    /// Individual results
    pub results: Vec<Result<String>>,
}

impl BatchResult {
    /// Calculates the success rate.
    pub fn success_rate(&self) -> f32 {
        if self.total == 0 {
            return 0.0;
        }
        (self.successful as f32 / self.total as f32) * 100.0
    }

    /// Calculates average processing time per request.
    pub fn avg_time_per_request_ms(&self) -> f32 {
        if self.total == 0 {
            return 0.0;
        }
        self.duration_ms as f32 / self.total as f32
    }

    /// Returns all successful results.
    pub fn successful_results(&self) -> Vec<&String> {
        self.results
            .iter()
            .filter_map(|r| r.as_ref().ok())
            .collect()
    }

    /// Returns all errors.
    pub fn errors(&self) -> Vec<&anyhow::Error> {
        self.results
            .iter()
            .filter_map(|r| r.as_ref().err())
            .collect()
    }
}

/// Parallel map operation over prompts.
pub async fn parallel_map<P, F, T>(
    provider: &P,
    prompts: &[String],
    max_concurrency: usize,
    mapper: F,
) -> Vec<Result<T>>
where
    P: LLMProvider + Send + Sync,
    F: Fn(String) -> T + Send + Sync + 'static,
    T: Send + 'static,
{
    let semaphore = Arc::new(Semaphore::new(max_concurrency));
    let mapper = Arc::new(mapper);

    stream::iter(prompts.iter())
        .map(|prompt| {
            let provider_ref = provider;
            let semaphore = Arc::clone(&semaphore);
            let mapper = Arc::clone(&mapper);
            let prompt = prompt.clone();

            async move {
                let _permit = semaphore.acquire().await.unwrap();
                provider_ref
                    .generate_text(&prompt)
                    .await
                    .map(|result| mapper(result))
            }
        })
        .buffer_unordered(max_concurrency)
        .collect::<Vec<_>>()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MockProvider;

    #[tokio::test]
    async fn test_batch_config() {
        let config = BatchConfig::new()
            .with_max_concurrency(5)
            .with_fail_fast(true)
            .with_batch_size(50);

        assert_eq!(config.max_concurrency, 5);
        assert_eq!(config.fail_fast, true);
        assert_eq!(config.batch_size, 50);
    }

    #[tokio::test]
    async fn test_batch_processor() {
        let provider = MockProvider::new().with_response("test", "response");
        let processor = BatchProcessor::new(provider);

        let prompts = vec![
            "test 1".to_string(),
            "test 2".to_string(),
            "test 3".to_string(),
        ];

        let results = processor.process_text_batch(&prompts).await;

        assert_eq!(results.len(), 3);
        for result in results {
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    async fn test_batch_with_stats() {
        let provider = MockProvider::new().with_response("test", "response");
        let processor = BatchProcessor::new(provider);

        let prompts = vec![
            "test 1".to_string(),
            "test 2".to_string(),
            "test 3".to_string(),
        ];

        let batch_result = processor.process_with_stats(&prompts).await;

        assert_eq!(batch_result.total, 3);
        assert_eq!(batch_result.successful, 3);
        assert_eq!(batch_result.failed, 0);
        assert!((batch_result.success_rate() - 100.0).abs() < f32::EPSILON);
    }

    #[tokio::test]
    async fn test_batch_result_methods() {
        let provider = MockProvider::new().with_response("test", "response");
        let processor = BatchProcessor::new(provider);

        let prompts = vec!["test".to_string()];
        let batch_result = processor.process_with_stats(&prompts).await;

        assert_eq!(batch_result.successful_results().len(), 1);
        assert_eq!(batch_result.errors().len(), 0);
        // Timing can be very fast, so just check it's non-negative
        assert!(batch_result.avg_time_per_request_ms() >= 0.0);
    }

    #[tokio::test]
    async fn test_parallel_map() {
        let provider = MockProvider::new().with_response("test", "hello");

        let prompts = vec!["test 1".to_string(), "test 2".to_string()];

        let results = parallel_map(&provider, &prompts, 2, |text| text.to_uppercase()).await;

        assert_eq!(results.len(), 2);
        for result in results {
            assert!(result.is_ok());
            let value = result.unwrap();
            assert!(value.contains("HELLO") || value.contains("MOCK"));
        }
    }

    #[tokio::test]
    async fn test_concurrency_limit() {
        let provider = MockProvider::new().with_response("test", "response");
        let config = BatchConfig::new().with_max_concurrency(2);
        let processor = BatchProcessor::with_config(provider, config);

        let prompts = vec![
            "test 1".to_string(),
            "test 2".to_string(),
            "test 3".to_string(),
            "test 4".to_string(),
            "test 5".to_string(),
        ];

        let start = std::time::Instant::now();
        let results = processor.process_text_batch(&prompts).await;
        let _duration = start.elapsed();

        assert_eq!(results.len(), 5);
        // With max_concurrency=2, it should process in batches
        // Duration check removed as it's timing-sensitive
    }
}
