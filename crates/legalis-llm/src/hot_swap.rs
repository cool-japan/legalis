//! Hot model swapping for zero-downtime model updates.
//!
//! This module provides capabilities for swapping LLM models at runtime
//! without interrupting ongoing requests.

use crate::{LLMProvider, TextStream};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Model swap strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwapStrategy {
    /// Wait for all in-flight requests to complete before swapping
    Graceful,
    /// Swap immediately, ongoing requests use old model
    Immediate,
    /// Gradually drain requests to old model while routing new ones to new model
    GradualDrain,
}

/// Model swap status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwapStatus {
    /// No swap in progress
    Idle,
    /// Swap initiated, waiting for drain
    Draining,
    /// Swap in progress
    Swapping,
    /// Swap completed
    Completed,
    /// Swap failed
    Failed,
}

/// Statistics for model swapping.
#[derive(Debug, Clone)]
pub struct SwapStats {
    /// Number of swaps performed
    pub total_swaps: u64,
    /// Number of in-flight requests during last swap
    pub in_flight_at_swap: usize,
    /// Time taken for last swap in milliseconds
    pub last_swap_duration_ms: u64,
    /// Current swap status
    pub status: SwapStatus,
}

impl Default for SwapStats {
    fn default() -> Self {
        Self {
            total_swaps: 0,
            in_flight_at_swap: 0,
            last_swap_duration_ms: 0,
            status: SwapStatus::Idle,
        }
    }
}

/// Hot-swappable provider wrapper.
pub struct HotSwappableProvider<P> {
    current_provider: Arc<RwLock<P>>,
    in_flight_requests: Arc<RwLock<usize>>,
    strategy: SwapStrategy,
    stats: Arc<RwLock<SwapStats>>,
}

impl<P: Clone> HotSwappableProvider<P> {
    /// Creates a new hot-swappable provider.
    pub fn new(provider: P, strategy: SwapStrategy) -> Self {
        Self {
            current_provider: Arc::new(RwLock::new(provider)),
            in_flight_requests: Arc::new(RwLock::new(0)),
            strategy,
            stats: Arc::new(RwLock::new(SwapStats::default())),
        }
    }

    /// Gets the current number of in-flight requests.
    pub async fn in_flight_count(&self) -> usize {
        *self.in_flight_requests.read().await
    }

    /// Gets swap statistics.
    pub async fn swap_stats(&self) -> SwapStats {
        self.stats.read().await.clone()
    }

    /// Swaps the underlying provider with a new one.
    pub async fn swap_provider(&self, new_provider: P) -> Result<()> {
        let start = std::time::Instant::now();

        // Update status
        {
            let mut stats = self.stats.write().await;
            stats.status = SwapStatus::Draining;
        }

        match self.strategy {
            SwapStrategy::Graceful => {
                // Wait for all in-flight requests to complete
                self.wait_for_drain().await;
            }
            SwapStrategy::Immediate => {
                // Swap immediately, no waiting
            }
            SwapStrategy::GradualDrain => {
                // Wait with a timeout (e.g., 30 seconds)
                self.wait_for_drain_with_timeout(std::time::Duration::from_secs(30))
                    .await;
            }
        }

        // Update status
        {
            let mut stats = self.stats.write().await;
            stats.status = SwapStatus::Swapping;
        }

        // Perform the swap
        let in_flight = self.in_flight_count().await;
        {
            let mut provider = self.current_provider.write().await;
            *provider = new_provider;
        }

        // Update statistics
        {
            let mut stats = self.stats.write().await;
            stats.total_swaps += 1;
            stats.in_flight_at_swap = in_flight;
            stats.last_swap_duration_ms = start.elapsed().as_millis() as u64;
            stats.status = SwapStatus::Completed;
        }

        Ok(())
    }

    /// Waits for all in-flight requests to drain.
    async fn wait_for_drain(&self) {
        loop {
            let count = self.in_flight_count().await;
            if count == 0 {
                break;
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    }

    /// Waits for requests to drain with a timeout.
    async fn wait_for_drain_with_timeout(&self, timeout: std::time::Duration) {
        let start = std::time::Instant::now();
        loop {
            if start.elapsed() >= timeout {
                break;
            }
            let count = self.in_flight_count().await;
            if count == 0 {
                break;
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    }

    /// Increments the in-flight request counter.
    async fn increment_in_flight(&self) {
        let mut count = self.in_flight_requests.write().await;
        *count += 1;
    }

    /// Decrements the in-flight request counter.
    async fn decrement_in_flight(&self) {
        let mut count = self.in_flight_requests.write().await;
        if *count > 0 {
            *count -= 1;
        }
    }
}

#[async_trait]
impl<P: LLMProvider + Clone> LLMProvider for HotSwappableProvider<P> {
    async fn generate_text(&self, prompt: &str) -> Result<String> {
        self.increment_in_flight().await;

        let result = {
            let provider = self.current_provider.read().await;
            provider.generate_text(prompt).await
        };

        self.decrement_in_flight().await;
        result
    }

    async fn generate_structured<T: serde::de::DeserializeOwned + Send>(
        &self,
        prompt: &str,
    ) -> Result<T> {
        self.increment_in_flight().await;

        let result = {
            let provider = self.current_provider.read().await;
            provider.generate_structured::<T>(prompt).await
        };

        self.decrement_in_flight().await;
        result
    }

    async fn generate_text_stream(&self, prompt: &str) -> Result<TextStream> {
        self.increment_in_flight().await;

        let result = {
            let provider = self.current_provider.read().await;
            provider.generate_text_stream(prompt).await
        };

        // Note: We decrement immediately, but in production you'd want to track
        // stream completion
        self.decrement_in_flight().await;

        result
    }

    fn provider_name(&self) -> &str {
        "HotSwappable"
    }

    fn model_name(&self) -> &str {
        "Dynamic"
    }

    fn supports_streaming(&self) -> bool {
        true
    }
}

/// Model version information.
#[derive(Debug, Clone)]
pub struct ModelVersion {
    /// Version identifier
    pub version: String,
    /// Model name
    pub model_name: String,
    /// Deployment timestamp
    pub deployed_at: chrono::DateTime<chrono::Utc>,
    /// Metadata
    pub metadata: std::collections::HashMap<String, String>,
}

impl ModelVersion {
    /// Creates a new model version.
    pub fn new(version: impl Into<String>, model_name: impl Into<String>) -> Self {
        Self {
            version: version.into(),
            model_name: model_name.into(),
            deployed_at: chrono::Utc::now(),
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Adds metadata.
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

/// Model version manager for tracking deployed models.
pub struct VersionManager {
    current_version: Arc<RwLock<Option<ModelVersion>>>,
    version_history: Arc<RwLock<Vec<ModelVersion>>>,
    max_history: usize,
}

impl VersionManager {
    /// Creates a new version manager.
    pub fn new(max_history: usize) -> Self {
        Self {
            current_version: Arc::new(RwLock::new(None)),
            version_history: Arc::new(RwLock::new(Vec::new())),
            max_history,
        }
    }

    /// Sets the current version.
    pub async fn set_current(&self, version: ModelVersion) {
        // Add old version to history if it exists
        if let Some(old_version) = self.current_version.read().await.clone() {
            let mut history = self.version_history.write().await;
            history.push(old_version);

            // Trim history if needed
            if history.len() > self.max_history {
                history.remove(0);
            }
        }

        // Set new current version
        let mut current = self.current_version.write().await;
        *current = Some(version);
    }

    /// Gets the current version.
    pub async fn current(&self) -> Option<ModelVersion> {
        self.current_version.read().await.clone()
    }

    /// Gets the version history.
    pub async fn history(&self) -> Vec<ModelVersion> {
        self.version_history.read().await.clone()
    }

    /// Rolls back to the previous version.
    pub async fn rollback(&self) -> Option<ModelVersion> {
        let mut history = self.version_history.write().await;
        if let Some(previous) = history.pop() {
            let mut current = self.current_version.write().await;
            *current = Some(previous.clone());
            Some(previous)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MockProvider;

    #[tokio::test]
    async fn test_hot_swap_graceful() {
        let provider1 = MockProvider::default();
        let swappable = HotSwappableProvider::new(provider1, SwapStrategy::Graceful);

        // Initial request
        let result = swappable.generate_text("test").await;
        assert!(result.is_ok());

        // Swap provider
        let provider2 = MockProvider::default();
        swappable.swap_provider(provider2).await.unwrap();

        // Request after swap
        let result = swappable.generate_text("test2").await;
        assert!(result.is_ok());

        // Check stats
        let stats = swappable.swap_stats().await;
        assert_eq!(stats.total_swaps, 1);
        assert_eq!(stats.status, SwapStatus::Completed);
    }

    #[tokio::test]
    async fn test_in_flight_tracking() {
        let provider = MockProvider::default();
        let swappable = HotSwappableProvider::new(provider, SwapStrategy::Immediate);

        assert_eq!(swappable.in_flight_count().await, 0);

        swappable.increment_in_flight().await;
        assert_eq!(swappable.in_flight_count().await, 1);

        swappable.decrement_in_flight().await;
        assert_eq!(swappable.in_flight_count().await, 0);
    }

    #[tokio::test]
    async fn test_version_manager() {
        let manager = VersionManager::new(5);

        let v1 = ModelVersion::new("1.0", "gpt-4");
        manager.set_current(v1.clone()).await;

        let current = manager.current().await;
        assert!(current.is_some());
        assert_eq!(current.unwrap().version, "1.0");

        let v2 = ModelVersion::new("2.0", "gpt-4");
        manager.set_current(v2).await;

        let history = manager.history().await;
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].version, "1.0");
    }

    #[tokio::test]
    async fn test_version_rollback() {
        let manager = VersionManager::new(5);

        let v1 = ModelVersion::new("1.0", "gpt-4");
        manager.set_current(v1).await;

        let v2 = ModelVersion::new("2.0", "gpt-4");
        manager.set_current(v2).await;

        // Rollback to v1
        let rolled_back = manager.rollback().await;
        assert!(rolled_back.is_some());
        assert_eq!(rolled_back.unwrap().version, "1.0");

        let current = manager.current().await;
        assert_eq!(current.unwrap().version, "1.0");
    }

    #[test]
    fn test_model_version_creation() {
        let version = ModelVersion::new("1.0", "gpt-4")
            .with_metadata("region", "us-east-1")
            .with_metadata("deployment", "blue");

        assert_eq!(version.version, "1.0");
        assert_eq!(version.model_name, "gpt-4");
        assert_eq!(
            version.metadata.get("region"),
            Some(&"us-east-1".to_string())
        );
    }
}
