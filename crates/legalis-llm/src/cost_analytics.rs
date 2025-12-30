//! Cost analytics and optimization for LLM usage.
//!
//! This module provides comprehensive cost tracking, analysis, and optimization
//! capabilities for LLM operations in production environments.

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Cost record for a single LLM request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostRecord {
    /// Timestamp of the request
    pub timestamp: DateTime<Utc>,
    /// Provider used
    pub provider: String,
    /// Model used
    pub model: String,
    /// Input tokens
    pub input_tokens: usize,
    /// Output tokens
    pub output_tokens: usize,
    /// Total cost in USD
    pub cost_usd: f64,
    /// Request latency in milliseconds
    pub latency_ms: u64,
    /// Whether the request succeeded
    pub success: bool,
    /// User or tenant ID (for multi-tenant scenarios)
    pub tenant_id: Option<String>,
    /// Request category (e.g., "analysis", "generation", "summarization")
    pub category: Option<String>,
}

impl CostRecord {
    /// Creates a new cost record.
    pub fn new(
        provider: impl Into<String>,
        model: impl Into<String>,
        input_tokens: usize,
        output_tokens: usize,
        cost_usd: f64,
    ) -> Self {
        Self {
            timestamp: Utc::now(),
            provider: provider.into(),
            model: model.into(),
            input_tokens,
            output_tokens,
            cost_usd,
            latency_ms: 0,
            success: true,
            tenant_id: None,
            category: None,
        }
    }

    /// Sets the latency.
    pub fn with_latency(mut self, latency_ms: u64) -> Self {
        self.latency_ms = latency_ms;
        self
    }

    /// Sets the success status.
    pub fn with_success(mut self, success: bool) -> Self {
        self.success = success;
        self
    }

    /// Sets the tenant ID.
    pub fn with_tenant(mut self, tenant_id: impl Into<String>) -> Self {
        self.tenant_id = Some(tenant_id.into());
        self
    }

    /// Sets the category.
    pub fn with_category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    /// Returns total tokens.
    pub fn total_tokens(&self) -> usize {
        self.input_tokens + self.output_tokens
    }
}

/// Analytics for cost data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostAnalytics {
    /// Total cost
    pub total_cost: f64,
    /// Total requests
    pub total_requests: usize,
    /// Successful requests
    pub successful_requests: usize,
    /// Failed requests
    pub failed_requests: usize,
    /// Total input tokens
    pub total_input_tokens: usize,
    /// Total output tokens
    pub total_output_tokens: usize,
    /// Average cost per request
    pub avg_cost_per_request: f64,
    /// Average latency
    pub avg_latency_ms: f64,
    /// Cost by provider
    pub cost_by_provider: HashMap<String, f64>,
    /// Cost by model
    pub cost_by_model: HashMap<String, f64>,
    /// Cost by category
    pub cost_by_category: HashMap<String, f64>,
    /// Cost by tenant
    pub cost_by_tenant: HashMap<String, f64>,
    /// Time range
    pub time_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
}

impl CostAnalytics {
    /// Creates empty analytics.
    pub fn new() -> Self {
        Self {
            total_cost: 0.0,
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            total_input_tokens: 0,
            total_output_tokens: 0,
            avg_cost_per_request: 0.0,
            avg_latency_ms: 0.0,
            cost_by_provider: HashMap::new(),
            cost_by_model: HashMap::new(),
            cost_by_category: HashMap::new(),
            cost_by_tenant: HashMap::new(),
            time_range: None,
        }
    }

    /// Returns total tokens.
    pub fn total_tokens(&self) -> usize {
        self.total_input_tokens + self.total_output_tokens
    }

    /// Returns success rate as a percentage.
    pub fn success_rate(&self) -> f64 {
        if self.total_requests == 0 {
            0.0
        } else {
            (self.successful_requests as f64 / self.total_requests as f64) * 100.0
        }
    }

    /// Returns the most expensive provider.
    pub fn most_expensive_provider(&self) -> Option<(&String, &f64)> {
        self.cost_by_provider
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
    }

    /// Returns the most used model.
    pub fn most_expensive_model(&self) -> Option<(&String, &f64)> {
        self.cost_by_model
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
    }
}

impl Default for CostAnalytics {
    fn default() -> Self {
        Self::new()
    }
}

/// Cost optimizer with recommendations.
pub struct CostOptimizer {
    /// Cost records
    records: Arc<RwLock<Vec<CostRecord>>>,
    /// Pricing information
    pricing: Arc<RwLock<HashMap<String, ModelPricing>>>,
}

/// Pricing information for a model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelPricing {
    /// Provider name
    pub provider: String,
    /// Model name
    pub model: String,
    /// Cost per 1K input tokens (USD)
    pub input_cost_per_1k: f64,
    /// Cost per 1K output tokens (USD)
    pub output_cost_per_1k: f64,
    /// Average latency (ms)
    pub avg_latency_ms: Option<f64>,
    /// Quality score (0-100)
    pub quality_score: Option<f64>,
}

impl ModelPricing {
    /// Creates new model pricing.
    pub fn new(
        provider: impl Into<String>,
        model: impl Into<String>,
        input_cost_per_1k: f64,
        output_cost_per_1k: f64,
    ) -> Self {
        Self {
            provider: provider.into(),
            model: model.into(),
            input_cost_per_1k,
            output_cost_per_1k,
            avg_latency_ms: None,
            quality_score: None,
        }
    }

    /// Calculates cost for given token counts.
    pub fn calculate_cost(&self, input_tokens: usize, output_tokens: usize) -> f64 {
        let input_cost = (input_tokens as f64 / 1000.0) * self.input_cost_per_1k;
        let output_cost = (output_tokens as f64 / 1000.0) * self.output_cost_per_1k;
        input_cost + output_cost
    }
}

/// Optimization recommendation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationRecommendation {
    /// Recommendation type
    pub recommendation_type: RecommendationType,
    /// Description
    pub description: String,
    /// Potential savings (USD/month)
    pub potential_savings_monthly: f64,
    /// Priority (1-5, 5 being highest)
    pub priority: u8,
    /// Actionable steps
    pub action_items: Vec<String>,
}

/// Types of optimization recommendations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecommendationType {
    /// Switch to a cheaper model
    ModelSwitch { from: String, to: String },
    /// Use caching more effectively
    ImprovedCaching,
    /// Reduce prompt length
    PromptOptimization,
    /// Use batch processing
    BatchProcessing,
    /// Switch provider for specific use cases
    ProviderSwitch { from: String, to: String },
    /// Implement request deduplication
    Deduplication,
    /// Use streaming for long responses
    UseStreaming,
    /// Custom recommendation
    Custom { title: String },
}

impl CostOptimizer {
    /// Creates a new cost optimizer.
    pub fn new() -> Self {
        let mut pricing = HashMap::new();

        // Add common model pricing (as of 2025)
        pricing.insert(
            "openai/gpt-4".to_string(),
            ModelPricing::new("openai", "gpt-4", 0.03, 0.06),
        );
        pricing.insert(
            "openai/gpt-4-turbo".to_string(),
            ModelPricing::new("openai", "gpt-4-turbo", 0.01, 0.03),
        );
        pricing.insert(
            "openai/gpt-3.5-turbo".to_string(),
            ModelPricing::new("openai", "gpt-3.5-turbo", 0.0005, 0.0015),
        );
        pricing.insert(
            "anthropic/claude-3-opus".to_string(),
            ModelPricing::new("anthropic", "claude-3-opus", 0.015, 0.075),
        );
        pricing.insert(
            "anthropic/claude-3-sonnet".to_string(),
            ModelPricing::new("anthropic", "claude-3-sonnet", 0.003, 0.015),
        );
        pricing.insert(
            "anthropic/claude-3-haiku".to_string(),
            ModelPricing::new("anthropic", "claude-3-haiku", 0.00025, 0.00125),
        );

        Self {
            records: Arc::new(RwLock::new(Vec::new())),
            pricing: Arc::new(RwLock::new(pricing)),
        }
    }

    /// Records a cost event.
    pub async fn record(&self, record: CostRecord) {
        let mut records = self.records.write().await;
        records.push(record);
    }

    /// Computes analytics for all records.
    pub async fn compute_analytics(&self) -> Result<CostAnalytics> {
        let records = self.records.read().await;
        self.compute_analytics_for_records(&records)
    }

    /// Computes analytics for records within a time range.
    pub async fn compute_analytics_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<CostAnalytics> {
        let records = self.records.read().await;
        let filtered: Vec<_> = records
            .iter()
            .filter(|r| r.timestamp >= start && r.timestamp <= end)
            .cloned()
            .collect();

        let mut analytics = self.compute_analytics_for_records(&filtered)?;
        analytics.time_range = Some((start, end));
        Ok(analytics)
    }

    fn compute_analytics_for_records(&self, records: &[CostRecord]) -> Result<CostAnalytics> {
        let mut analytics = CostAnalytics::new();

        for record in records {
            analytics.total_cost += record.cost_usd;
            analytics.total_requests += 1;
            analytics.total_input_tokens += record.input_tokens;
            analytics.total_output_tokens += record.output_tokens;

            if record.success {
                analytics.successful_requests += 1;
            } else {
                analytics.failed_requests += 1;
            }

            // By provider
            *analytics
                .cost_by_provider
                .entry(record.provider.clone())
                .or_insert(0.0) += record.cost_usd;

            // By model
            *analytics
                .cost_by_model
                .entry(record.model.clone())
                .or_insert(0.0) += record.cost_usd;

            // By category
            if let Some(ref category) = record.category {
                *analytics
                    .cost_by_category
                    .entry(category.clone())
                    .or_insert(0.0) += record.cost_usd;
            }

            // By tenant
            if let Some(ref tenant_id) = record.tenant_id {
                *analytics
                    .cost_by_tenant
                    .entry(tenant_id.clone())
                    .or_insert(0.0) += record.cost_usd;
            }
        }

        // Calculate averages
        if analytics.total_requests > 0 {
            analytics.avg_cost_per_request = analytics.total_cost / analytics.total_requests as f64;

            let total_latency: u64 = records.iter().map(|r| r.latency_ms).sum();
            analytics.avg_latency_ms = total_latency as f64 / analytics.total_requests as f64;
        }

        Ok(analytics)
    }

    /// Generates optimization recommendations.
    pub async fn generate_recommendations(&self) -> Result<Vec<OptimizationRecommendation>> {
        let analytics = self.compute_analytics().await?;
        let mut recommendations = Vec::new();

        // Recommendation: Switch expensive models
        if let Some((model, cost)) = analytics.most_expensive_model() {
            if cost > &100.0 {
                // If spending more than $100 on a model
                recommendations.push(OptimizationRecommendation {
                    recommendation_type: RecommendationType::ModelSwitch {
                        from: model.clone(),
                        to: "Consider cheaper alternatives".to_string(),
                    },
                    description: format!(
                        "Model '{}' accounts for ${:.2} in costs. Consider using a cheaper model for non-critical tasks.",
                        model, cost
                    ),
                    potential_savings_monthly: cost * 0.3, // Estimate 30% savings
                    priority: 5,
                    action_items: vec![
                        "Identify tasks that don't require the most expensive model".to_string(),
                        "Test cheaper alternatives for quality".to_string(),
                        "Implement routing to use cheaper models when appropriate".to_string(),
                    ],
                });
            }
        }

        // Recommendation: Improve caching
        if analytics.total_requests > 1000 {
            let cache_hit_estimate = 0.3; // Assume 30% cache hit rate possible
            let potential_savings = analytics.total_cost * cache_hit_estimate;

            recommendations.push(OptimizationRecommendation {
                recommendation_type: RecommendationType::ImprovedCaching,
                description: "High request volume detected. Implementing semantic caching could reduce costs significantly.".to_string(),
                potential_savings_monthly: potential_savings,
                priority: 4,
                action_items: vec![
                    "Enable semantic caching for similar queries".to_string(),
                    "Set appropriate cache TTLs".to_string(),
                    "Monitor cache hit rates".to_string(),
                ],
            });
        }

        // Recommendation: Batch processing
        let records = self.records.read().await;
        let single_request_count = records.len();
        if single_request_count > 500 {
            recommendations.push(OptimizationRecommendation {
                recommendation_type: RecommendationType::BatchProcessing,
                description:
                    "Many individual requests detected. Batching could reduce overhead and costs."
                        .to_string(),
                potential_savings_monthly: analytics.total_cost * 0.15, // Estimate 15% savings
                priority: 3,
                action_items: vec![
                    "Identify requests that can be batched".to_string(),
                    "Implement batch processing for bulk operations".to_string(),
                    "Use provider batch APIs where available".to_string(),
                ],
            });
        }

        Ok(recommendations)
    }

    /// Forecasts costs for the next period.
    pub async fn forecast_costs(&self, days_ahead: u32) -> Result<f64> {
        let now = Utc::now();
        let days_ago = now - chrono::Duration::days(30);

        let analytics = self.compute_analytics_range(days_ago, now).await?;

        // Simple linear projection
        let daily_avg = analytics.total_cost / 30.0;
        Ok(daily_avg * days_ahead as f64)
    }

    /// Detects cost anomalies.
    pub async fn detect_anomalies(&self) -> Result<Vec<CostAnomaly>> {
        let mut anomalies = Vec::new();
        let records = self.records.read().await;

        if records.len() < 100 {
            return Ok(anomalies); // Not enough data
        }

        // Calculate daily costs for the last 30 days
        let mut daily_costs: HashMap<String, f64> = HashMap::new();
        let now = Utc::now();

        for record in records.iter() {
            let days_ago = (now - record.timestamp).num_days();
            if days_ago <= 30 {
                let date_key = record.timestamp.format("%Y-%m-%d").to_string();
                *daily_costs.entry(date_key).or_insert(0.0) += record.cost_usd;
            }
        }

        // Calculate mean and standard deviation
        let costs: Vec<f64> = daily_costs.values().copied().collect();
        let mean = costs.iter().sum::<f64>() / costs.len() as f64;
        let variance = costs.iter().map(|c| (c - mean).powi(2)).sum::<f64>() / costs.len() as f64;
        let std_dev = variance.sqrt();

        // Detect anomalies (costs > 2 standard deviations from mean)
        for (date, cost) in daily_costs.iter() {
            if (cost - mean).abs() > 2.0 * std_dev {
                anomalies.push(CostAnomaly {
                    date: date.clone(),
                    cost: *cost,
                    expected_cost: mean,
                    severity: if (cost - mean).abs() > 3.0 * std_dev {
                        AnomalySeverity::High
                    } else {
                        AnomalySeverity::Medium
                    },
                    description: format!(
                        "Unusual spending detected: ${:.2} vs expected ${:.2}",
                        cost, mean
                    ),
                });
            }
        }

        Ok(anomalies)
    }

    /// Returns the total number of cost records.
    pub async fn record_count(&self) -> usize {
        self.records.read().await.len()
    }

    /// Gets pricing information for a model.
    pub async fn get_pricing(&self, provider: &str, model: &str) -> Option<ModelPricing> {
        let key = format!("{}/{}", provider, model);
        let pricing = self.pricing.read().await;
        pricing.get(&key).cloned()
    }

    /// Adds or updates pricing information for a model.
    pub async fn set_pricing(&self, pricing: ModelPricing) {
        let key = format!("{}/{}", pricing.provider, pricing.model);
        let mut pricing_map = self.pricing.write().await;
        pricing_map.insert(key, pricing);
    }

    /// Compares cost between two models for given token usage.
    pub async fn compare_model_costs(
        &self,
        provider1: &str,
        model1: &str,
        provider2: &str,
        model2: &str,
        input_tokens: usize,
        output_tokens: usize,
    ) -> Option<(f64, f64)> {
        let pricing1 = self.get_pricing(provider1, model1).await?;
        let pricing2 = self.get_pricing(provider2, model2).await?;

        let cost1 = pricing1.calculate_cost(input_tokens, output_tokens);
        let cost2 = pricing2.calculate_cost(input_tokens, output_tokens);

        Some((cost1, cost2))
    }
}

impl Default for CostOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

/// Cost anomaly detection result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostAnomaly {
    /// Date of the anomaly
    pub date: String,
    /// Actual cost
    pub cost: f64,
    /// Expected cost
    pub expected_cost: f64,
    /// Severity level
    pub severity: AnomalySeverity,
    /// Description
    pub description: String,
}

/// Severity level for anomalies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnomalySeverity {
    /// Low severity
    Low,
    /// Medium severity
    Medium,
    /// High severity
    High,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cost_record_creation() {
        let record = CostRecord::new("openai", "gpt-4", 1000, 500, 0.045)
            .with_latency(150)
            .with_tenant("tenant-1")
            .with_category("analysis");

        assert_eq!(record.provider, "openai");
        assert_eq!(record.model, "gpt-4");
        assert_eq!(record.input_tokens, 1000);
        assert_eq!(record.output_tokens, 500);
        assert_eq!(record.total_tokens(), 1500);
        assert_eq!(record.latency_ms, 150);
        assert_eq!(record.tenant_id, Some("tenant-1".to_string()));
    }

    #[test]
    fn test_model_pricing_calculation() {
        let pricing = ModelPricing::new("openai", "gpt-4", 0.03, 0.06);
        let cost = pricing.calculate_cost(1000, 500);

        // 1000 input tokens = 1 * 0.03 = 0.03
        // 500 output tokens = 0.5 * 0.06 = 0.03
        // Total = 0.06
        assert!((cost - 0.06).abs() < f64::EPSILON);
    }

    #[tokio::test]
    async fn test_cost_optimizer_recording() {
        let optimizer = CostOptimizer::new();

        optimizer
            .record(CostRecord::new("openai", "gpt-4", 1000, 500, 0.045))
            .await;
        optimizer
            .record(CostRecord::new(
                "anthropic",
                "claude-3-sonnet",
                800,
                600,
                0.021,
            ))
            .await;

        assert_eq!(optimizer.record_count().await, 2);
    }

    #[tokio::test]
    async fn test_analytics_computation() {
        let optimizer = CostOptimizer::new();

        optimizer
            .record(CostRecord::new("openai", "gpt-4", 1000, 500, 0.045).with_success(true))
            .await;
        optimizer
            .record(
                CostRecord::new("anthropic", "claude-3-sonnet", 800, 600, 0.021).with_success(true),
            )
            .await;
        optimizer
            .record(CostRecord::new("openai", "gpt-4", 500, 300, 0.033).with_success(false))
            .await;

        let analytics = optimizer.compute_analytics().await.unwrap();

        assert_eq!(analytics.total_requests, 3);
        assert_eq!(analytics.successful_requests, 2);
        assert_eq!(analytics.failed_requests, 1);
        assert!((analytics.total_cost - 0.099).abs() < 0.001);
        assert_eq!(analytics.total_input_tokens, 2300);
        assert_eq!(analytics.total_output_tokens, 1400);
    }

    #[tokio::test]
    async fn test_cost_by_provider() {
        let optimizer = CostOptimizer::new();

        optimizer
            .record(CostRecord::new("openai", "gpt-4", 1000, 500, 0.045))
            .await;
        optimizer
            .record(CostRecord::new("openai", "gpt-3.5-turbo", 1000, 500, 0.001))
            .await;
        optimizer
            .record(CostRecord::new(
                "anthropic",
                "claude-3-sonnet",
                800,
                600,
                0.021,
            ))
            .await;

        let analytics = optimizer.compute_analytics().await.unwrap();

        let openai_cost = analytics.cost_by_provider.get("openai").unwrap();
        assert!((openai_cost - 0.046).abs() < 0.001);

        let anthropic_cost = analytics.cost_by_provider.get("anthropic").unwrap();
        assert!((anthropic_cost - 0.021).abs() < 0.001);
    }

    #[tokio::test]
    async fn test_recommendations_generation() {
        let optimizer = CostOptimizer::new();

        // Add many expensive requests
        for _ in 0..2000 {
            optimizer
                .record(CostRecord::new("openai", "gpt-4", 1000, 500, 0.045))
                .await;
        }

        let recommendations = optimizer.generate_recommendations().await.unwrap();
        assert!(!recommendations.is_empty());
    }

    #[tokio::test]
    async fn test_cost_forecasting() {
        let optimizer = CostOptimizer::new();

        // Add records over time
        for _ in 0..100 {
            optimizer
                .record(CostRecord::new("openai", "gpt-4", 1000, 500, 0.045))
                .await;
        }

        let forecast = optimizer.forecast_costs(30).await.unwrap();
        assert!(forecast > 0.0);
    }

    #[tokio::test]
    async fn test_get_pricing() {
        let optimizer = CostOptimizer::new();

        let pricing = optimizer.get_pricing("openai", "gpt-4").await;
        assert!(pricing.is_some());

        let pricing = pricing.unwrap();
        assert_eq!(pricing.provider, "openai");
        assert_eq!(pricing.model, "gpt-4");
        assert!((pricing.input_cost_per_1k - 0.03).abs() < f64::EPSILON);
    }

    #[tokio::test]
    async fn test_set_pricing() {
        let optimizer = CostOptimizer::new();

        let custom_pricing = ModelPricing::new("custom", "my-model", 0.01, 0.02);
        optimizer.set_pricing(custom_pricing).await;

        let retrieved = optimizer.get_pricing("custom", "my-model").await;
        assert!(retrieved.is_some());

        let retrieved = retrieved.unwrap();
        assert_eq!(retrieved.provider, "custom");
        assert_eq!(retrieved.model, "my-model");
    }

    #[tokio::test]
    async fn test_compare_model_costs() {
        let optimizer = CostOptimizer::new();

        let comparison = optimizer
            .compare_model_costs("openai", "gpt-4", "openai", "gpt-3.5-turbo", 1000, 500)
            .await;

        assert!(comparison.is_some());
        let (cost1, cost2) = comparison.unwrap();

        // GPT-4 should be more expensive than GPT-3.5-turbo
        assert!(cost1 > cost2);
    }
}
