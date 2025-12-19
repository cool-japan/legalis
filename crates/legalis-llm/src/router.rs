//! Model routing and automatic provider selection.
//!
//! This module provides intelligent routing between different LLM providers
//! based on criteria like cost, latency, task complexity, and availability.

use crate::{LLMProvider, TextStream};
use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use std::sync::Arc;

/// Routing strategy for selecting providers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoutingStrategy {
    /// Choose the cheapest provider that meets requirements
    CostOptimized,
    /// Choose the fastest provider (lowest latency)
    LatencyOptimized,
    /// Balance between cost and quality
    Balanced,
    /// Choose based on task complexity
    ComplexityBased,
    /// Round-robin between providers
    RoundRobin,
}

/// Task complexity level for routing decisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskComplexity {
    /// Simple tasks (classification, basic Q&A)
    Simple,
    /// Medium complexity (summarization, basic analysis)
    Medium,
    /// Complex tasks (deep reasoning, code generation)
    Complex,
}

/// Provider capability and cost information.
#[derive(Debug, Clone)]
pub struct ProviderInfo {
    /// Provider name
    pub name: String,
    /// Relative cost (0.0 to 1.0, where 1.0 is most expensive)
    pub cost: f32,
    /// Expected latency in milliseconds
    pub latency_ms: u64,
    /// Maximum task complexity this provider can handle
    pub max_complexity: TaskComplexity,
    /// Whether the provider is currently available
    pub available: bool,
}

impl ProviderInfo {
    /// Creates a new provider info.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            cost: 0.5,
            latency_ms: 1000,
            max_complexity: TaskComplexity::Complex,
            available: true,
        }
    }

    /// Sets the cost level.
    pub fn with_cost(mut self, cost: f32) -> Self {
        self.cost = cost.clamp(0.0, 1.0);
        self
    }

    /// Sets the expected latency.
    pub fn with_latency(mut self, latency_ms: u64) -> Self {
        self.latency_ms = latency_ms;
        self
    }

    /// Sets the maximum complexity.
    pub fn with_max_complexity(mut self, complexity: TaskComplexity) -> Self {
        self.max_complexity = complexity;
        self
    }

    /// Sets availability.
    pub fn with_availability(mut self, available: bool) -> Self {
        self.available = available;
        self
    }

    /// Computes a score for this provider based on strategy and complexity.
    fn score(&self, strategy: RoutingStrategy, complexity: TaskComplexity) -> f32 {
        if !self.available || complexity > self.max_complexity {
            return 0.0;
        }

        match strategy {
            RoutingStrategy::CostOptimized => 1.0 - self.cost,
            RoutingStrategy::LatencyOptimized => 1.0 / (1.0 + self.latency_ms as f32 / 1000.0),
            RoutingStrategy::Balanced => {
                let cost_score = 1.0 - self.cost;
                let latency_score = 1.0 / (1.0 + self.latency_ms as f32 / 1000.0);
                (cost_score + latency_score) / 2.0
            }
            RoutingStrategy::ComplexityBased => {
                // Prefer providers that match the complexity level
                if self.max_complexity == complexity {
                    1.0
                } else if self.max_complexity > complexity {
                    0.7 // Over-powered provider
                } else {
                    0.0 // Under-powered provider
                }
            }
            RoutingStrategy::RoundRobin => 1.0, // Handled separately
        }
    }
}

/// Model router that selects providers based on routing strategy.
pub struct ModelRouter<P: LLMProvider> {
    providers: Vec<(P, ProviderInfo)>,
    strategy: RoutingStrategy,
    round_robin_index: std::sync::atomic::AtomicUsize,
}

impl<P: LLMProvider> ModelRouter<P> {
    /// Creates a new model router with the given strategy.
    pub fn new(strategy: RoutingStrategy) -> Self {
        Self {
            providers: Vec::new(),
            strategy,
            round_robin_index: std::sync::atomic::AtomicUsize::new(0),
        }
    }

    /// Adds a provider to the router.
    pub fn add_provider(&mut self, provider: P, info: ProviderInfo) {
        self.providers.push((provider, info));
    }

    /// Adds a provider with builder pattern.
    pub fn with_provider(mut self, provider: P, info: ProviderInfo) -> Self {
        self.add_provider(provider, info);
        self
    }

    /// Sets the routing strategy.
    pub fn set_strategy(&mut self, strategy: RoutingStrategy) {
        self.strategy = strategy;
    }

    /// Selects the best provider for the given task complexity.
    fn select_provider(&self, complexity: TaskComplexity) -> Result<&P> {
        if self.providers.is_empty() {
            anyhow::bail!("No providers registered");
        }

        match self.strategy {
            RoutingStrategy::RoundRobin => {
                let index = self
                    .round_robin_index
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                let provider_index = index % self.providers.len();
                Ok(&self.providers[provider_index].0)
            }
            _ => {
                let mut best_score = 0.0;
                let mut best_provider: Option<&P> = None;

                for (provider, info) in &self.providers {
                    let score = info.score(self.strategy, complexity);
                    if score > best_score {
                        best_score = score;
                        best_provider = Some(provider);
                    }
                }

                best_provider.context("No suitable provider found for the given task complexity")
            }
        }
    }

    /// Estimates task complexity from the prompt.
    fn estimate_complexity(&self, prompt: &str) -> TaskComplexity {
        let len = prompt.len();
        let word_count = prompt.split_whitespace().count();

        // Simple heuristics - can be made more sophisticated
        if len < 50 && word_count < 10 {
            TaskComplexity::Simple
        } else if len < 300 && word_count < 50 {
            TaskComplexity::Medium
        } else {
            TaskComplexity::Complex
        }
    }

    /// Gets all registered providers.
    pub fn providers(&self) -> &[(P, ProviderInfo)] {
        &self.providers
    }

    /// Gets the current routing strategy.
    pub fn strategy(&self) -> RoutingStrategy {
        self.strategy
    }
}

#[async_trait]
impl<P: LLMProvider + Send + Sync> LLMProvider for ModelRouter<P> {
    async fn generate_text(&self, prompt: &str) -> Result<String> {
        let complexity = self.estimate_complexity(prompt);
        let provider = self.select_provider(complexity)?;
        provider.generate_text(prompt).await
    }

    async fn generate_structured<T: DeserializeOwned + Send>(&self, prompt: &str) -> Result<T> {
        let complexity = self.estimate_complexity(prompt);
        let provider = self.select_provider(complexity)?;
        provider.generate_structured::<T>(prompt).await
    }

    async fn generate_text_stream(&self, prompt: &str) -> Result<TextStream> {
        let complexity = self.estimate_complexity(prompt);
        let provider = self.select_provider(complexity)?;
        provider.generate_text_stream(prompt).await
    }

    fn provider_name(&self) -> &str {
        "ModelRouter"
    }

    fn model_name(&self) -> &str {
        "multi-model"
    }

    fn supports_streaming(&self) -> bool {
        self.providers.iter().any(|(p, _)| p.supports_streaming())
    }
}

/// Load balancer that distributes requests across multiple providers.
pub struct LoadBalancer<P: LLMProvider> {
    providers: Vec<Arc<P>>,
    index: std::sync::atomic::AtomicUsize,
}

impl<P: LLMProvider> LoadBalancer<P> {
    /// Creates a new load balancer.
    pub fn new() -> Self {
        Self {
            providers: Vec::new(),
            index: std::sync::atomic::AtomicUsize::new(0),
        }
    }

    /// Adds a provider to the load balancer.
    pub fn add_provider(&mut self, provider: P) {
        self.providers.push(Arc::new(provider));
    }

    /// Adds a provider with builder pattern.
    pub fn with_provider(mut self, provider: P) -> Self {
        self.add_provider(provider);
        self
    }

    /// Gets the next provider in round-robin order.
    fn next_provider(&self) -> Result<&Arc<P>> {
        if self.providers.is_empty() {
            anyhow::bail!("No providers registered");
        }

        let index = self
            .index
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let provider_index = index % self.providers.len();
        Ok(&self.providers[provider_index])
    }
}

impl<P: LLMProvider> Default for LoadBalancer<P> {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl<P: LLMProvider + Send + Sync> LLMProvider for LoadBalancer<P> {
    async fn generate_text(&self, prompt: &str) -> Result<String> {
        let provider = self.next_provider()?;
        provider.generate_text(prompt).await
    }

    async fn generate_structured<T: DeserializeOwned + Send>(&self, prompt: &str) -> Result<T> {
        let provider = self.next_provider()?;
        provider.generate_structured::<T>(prompt).await
    }

    async fn generate_text_stream(&self, prompt: &str) -> Result<TextStream> {
        let provider = self.next_provider()?;
        provider.generate_text_stream(prompt).await
    }

    fn provider_name(&self) -> &str {
        "LoadBalancer"
    }

    fn model_name(&self) -> &str {
        "multi-model"
    }

    fn supports_streaming(&self) -> bool {
        self.providers.iter().any(|p| p.supports_streaming())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MockProvider;

    #[test]
    fn test_provider_info_creation() {
        let info = ProviderInfo::new("test")
            .with_cost(0.8)
            .with_latency(500)
            .with_max_complexity(TaskComplexity::Medium);

        assert_eq!(info.name, "test");
        assert!((info.cost - 0.8).abs() < f32::EPSILON);
        assert_eq!(info.latency_ms, 500);
        assert_eq!(info.max_complexity, TaskComplexity::Medium);
    }

    #[test]
    fn test_provider_scoring() {
        let cheap = ProviderInfo::new("cheap").with_cost(0.2);
        let expensive = ProviderInfo::new("expensive").with_cost(0.9);

        let cheap_score = cheap.score(RoutingStrategy::CostOptimized, TaskComplexity::Simple);
        let expensive_score =
            expensive.score(RoutingStrategy::CostOptimized, TaskComplexity::Simple);

        assert!(cheap_score > expensive_score);
    }

    #[test]
    fn test_task_complexity() {
        let simple = TaskComplexity::Simple;
        let medium = TaskComplexity::Medium;
        let complex = TaskComplexity::Complex;

        assert!(simple < medium);
        assert!(medium < complex);
    }

    #[tokio::test]
    async fn test_model_router() {
        let provider1 = MockProvider::new().with_response("test", "response1");
        let provider2 = MockProvider::new().with_response("test", "response2");

        let router = ModelRouter::new(RoutingStrategy::CostOptimized)
            .with_provider(
                provider1,
                ProviderInfo::new("provider1")
                    .with_cost(0.2)
                    .with_max_complexity(TaskComplexity::Simple),
            )
            .with_provider(
                provider2,
                ProviderInfo::new("provider2")
                    .with_cost(0.8)
                    .with_max_complexity(TaskComplexity::Complex),
            );

        // Simple task should use the cheap provider
        let result = router.generate_text("test").await.unwrap();
        assert!(result.contains("response"));
    }

    #[tokio::test]
    async fn test_load_balancer() {
        let provider1 = MockProvider::new().with_response("test", "response1");
        let provider2 = MockProvider::new().with_response("test", "response2");

        let balancer = LoadBalancer::new()
            .with_provider(provider1)
            .with_provider(provider2);

        // Both providers should be used
        let result1 = balancer.generate_text("test").await.unwrap();
        let result2 = balancer.generate_text("test").await.unwrap();

        assert!(result1.contains("response"));
        assert!(result2.contains("response"));
    }

    #[test]
    fn test_complexity_estimation() {
        let router = ModelRouter::<MockProvider>::new(RoutingStrategy::Balanced);

        assert_eq!(router.estimate_complexity("Hi"), TaskComplexity::Simple);

        let medium_prompt =
            "Explain the difference between synchronous and asynchronous programming in Rust.";
        assert_eq!(
            router.estimate_complexity(medium_prompt),
            TaskComplexity::Medium
        );

        let complex_prompt = "Write a detailed technical specification for implementing a distributed consensus algorithm using the Raft protocol. Include pseudocode for leader election, log replication, and safety guarantees. Explain the trade-offs between different configuration parameters and how they affect system behavior under various failure scenarios.";
        assert_eq!(
            router.estimate_complexity(complex_prompt),
            TaskComplexity::Complex
        );
    }

    #[test]
    fn test_round_robin_strategy() {
        let provider1 = MockProvider::new();
        let provider2 = MockProvider::new();
        let provider3 = MockProvider::new();

        let router = ModelRouter::new(RoutingStrategy::RoundRobin)
            .with_provider(provider1, ProviderInfo::new("p1"))
            .with_provider(provider2, ProviderInfo::new("p2"))
            .with_provider(provider3, ProviderInfo::new("p3"));

        // Should cycle through providers
        for _ in 0..10 {
            let provider = router.select_provider(TaskComplexity::Simple);
            assert!(provider.is_ok());
        }
    }
}
