//! Token usage tracking and cost estimation for LLM requests.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Token usage information for a single request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    /// Prompt tokens used
    pub prompt_tokens: usize,
    /// Completion tokens generated
    pub completion_tokens: usize,
    /// Total tokens (prompt + completion)
    pub total_tokens: usize,
}

impl TokenUsage {
    /// Creates a new TokenUsage.
    pub fn new(prompt_tokens: usize, completion_tokens: usize) -> Self {
        Self {
            prompt_tokens,
            completion_tokens,
            total_tokens: prompt_tokens + completion_tokens,
        }
    }

    /// Adds usage from another TokenUsage.
    pub fn add(&mut self, other: &TokenUsage) {
        self.prompt_tokens += other.prompt_tokens;
        self.completion_tokens += other.completion_tokens;
        self.total_tokens += other.total_tokens;
    }
}

/// Cost estimation for different LLM providers and models.
#[derive(Debug, Clone)]
pub struct CostEstimator {
    /// Cost per 1k prompt tokens in USD
    pub prompt_cost_per_1k: f64,
    /// Cost per 1k completion tokens in USD
    pub completion_cost_per_1k: f64,
}

impl CostEstimator {
    /// Creates a new cost estimator.
    pub fn new(prompt_cost_per_1k: f64, completion_cost_per_1k: f64) -> Self {
        Self {
            prompt_cost_per_1k,
            completion_cost_per_1k,
        }
    }

    /// Estimates the cost for given token usage in USD.
    pub fn estimate_cost(&self, usage: &TokenUsage) -> f64 {
        let prompt_cost = (usage.prompt_tokens as f64 / 1000.0) * self.prompt_cost_per_1k;
        let completion_cost =
            (usage.completion_tokens as f64 / 1000.0) * self.completion_cost_per_1k;
        prompt_cost + completion_cost
    }

    /// Creates a cost estimator for OpenAI GPT-4.
    pub fn openai_gpt4() -> Self {
        Self::new(0.03, 0.06) // $0.03 per 1k prompt, $0.06 per 1k completion
    }

    /// Creates a cost estimator for OpenAI GPT-3.5-Turbo.
    pub fn openai_gpt35_turbo() -> Self {
        Self::new(0.0015, 0.002) // $0.0015 per 1k prompt, $0.002 per 1k completion
    }

    /// Creates a cost estimator for Anthropic Claude 3 Opus.
    pub fn anthropic_claude3_opus() -> Self {
        Self::new(0.015, 0.075) // $0.015 per 1k prompt, $0.075 per 1k completion
    }

    /// Creates a cost estimator for Anthropic Claude 3 Sonnet.
    pub fn anthropic_claude3_sonnet() -> Self {
        Self::new(0.003, 0.015) // $0.003 per 1k prompt, $0.015 per 1k completion
    }

    /// Creates a cost estimator for Anthropic Claude 3 Haiku.
    pub fn anthropic_claude3_haiku() -> Self {
        Self::new(0.00025, 0.00125) // $0.00025 per 1k prompt, $0.00125 per 1k completion
    }

    /// Creates a cost estimator for Google Gemini Pro.
    pub fn gemini_pro() -> Self {
        Self::new(0.00025, 0.0005) // $0.00025 per 1k prompt, $0.0005 per 1k completion
    }

    /// Creates a cost estimator for Google Gemini Pro 1.5.
    pub fn gemini_pro_15() -> Self {
        Self::new(0.00125, 0.005) // $0.00125 per 1k prompt, $0.005 per 1k completion
    }
}

/// Statistics for token usage across multiple requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenStats {
    /// Total number of requests
    pub request_count: usize,
    /// Total tokens used
    pub total_usage: TokenUsage,
    /// Average tokens per request
    pub avg_tokens_per_request: f64,
    /// Minimum tokens in a request
    pub min_tokens: usize,
    /// Maximum tokens in a request
    pub max_tokens: usize,
}

impl TokenStats {
    /// Creates empty statistics.
    pub fn new() -> Self {
        Self {
            request_count: 0,
            total_usage: TokenUsage::new(0, 0),
            avg_tokens_per_request: 0.0,
            min_tokens: usize::MAX,
            max_tokens: 0,
        }
    }

    /// Updates statistics with a new token usage.
    pub fn update(&mut self, usage: &TokenUsage) {
        self.request_count += 1;
        self.total_usage.add(usage);
        self.min_tokens = self.min_tokens.min(usage.total_tokens);
        self.max_tokens = self.max_tokens.max(usage.total_tokens);
        self.avg_tokens_per_request =
            self.total_usage.total_tokens as f64 / self.request_count as f64;
    }
}

impl Default for TokenStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Tracks token usage and provides cost estimation.
#[derive(Clone)]
pub struct TokenTracker {
    stats: Arc<Mutex<HashMap<String, TokenStats>>>,
    estimators: Arc<HashMap<String, CostEstimator>>,
}

impl TokenTracker {
    /// Creates a new token tracker.
    pub fn new() -> Self {
        let mut estimators = HashMap::new();

        // Register common model estimators
        estimators.insert("gpt-4".to_string(), CostEstimator::openai_gpt4());
        estimators.insert(
            "gpt-3.5-turbo".to_string(),
            CostEstimator::openai_gpt35_turbo(),
        );
        estimators.insert(
            "claude-3-opus".to_string(),
            CostEstimator::anthropic_claude3_opus(),
        );
        estimators.insert(
            "claude-3-sonnet".to_string(),
            CostEstimator::anthropic_claude3_sonnet(),
        );
        estimators.insert(
            "claude-3-haiku".to_string(),
            CostEstimator::anthropic_claude3_haiku(),
        );
        estimators.insert("gemini-pro".to_string(), CostEstimator::gemini_pro());
        estimators.insert("gemini-1.5-pro".to_string(), CostEstimator::gemini_pro_15());

        Self {
            stats: Arc::new(Mutex::new(HashMap::new())),
            estimators: Arc::new(estimators),
        }
    }

    /// Registers a custom cost estimator for a model.
    pub fn register_estimator(&mut self, model: impl Into<String>, estimator: CostEstimator) {
        Arc::get_mut(&mut self.estimators)
            .expect("Cannot modify estimators after sharing")
            .insert(model.into(), estimator);
    }

    /// Records token usage for a model.
    pub fn record_usage(&self, model: &str, usage: TokenUsage) {
        let mut stats = self.stats.lock().unwrap();
        stats.entry(model.to_string()).or_default().update(&usage);
    }

    /// Gets statistics for a specific model.
    pub fn get_stats(&self, model: &str) -> Option<TokenStats> {
        self.stats.lock().unwrap().get(model).cloned()
    }

    /// Gets statistics for all models.
    pub fn get_all_stats(&self) -> HashMap<String, TokenStats> {
        self.stats.lock().unwrap().clone()
    }

    /// Estimates cost for a specific model based on recorded usage.
    pub fn estimate_total_cost(&self, model: &str) -> Option<f64> {
        let stats = self.get_stats(model)?;
        let estimator = self.estimators.get(model)?;
        Some(estimator.estimate_cost(&stats.total_usage))
    }

    /// Estimates total cost across all models.
    pub fn estimate_total_cost_all(&self) -> f64 {
        let stats = self.stats.lock().unwrap();
        let mut total_cost = 0.0;

        for (model, model_stats) in stats.iter() {
            if let Some(estimator) = self.estimators.get(model) {
                total_cost += estimator.estimate_cost(&model_stats.total_usage);
            }
        }

        total_cost
    }

    /// Resets all statistics.
    pub fn reset(&self) {
        self.stats.lock().unwrap().clear();
    }

    /// Generates a usage report for all models.
    pub fn generate_report(&self) -> String {
        let stats = self.stats.lock().unwrap();
        let mut report = String::from("Token Usage Report\n");
        report.push_str("==================\n\n");

        let mut total_cost = 0.0;

        for (model, model_stats) in stats.iter() {
            report.push_str(&format!("Model: {}\n", model));
            report.push_str(&format!("  Requests: {}\n", model_stats.request_count));
            report.push_str(&format!(
                "  Total Tokens: {}\n",
                model_stats.total_usage.total_tokens
            ));
            report.push_str(&format!(
                "    - Prompt: {}\n",
                model_stats.total_usage.prompt_tokens
            ));
            report.push_str(&format!(
                "    - Completion: {}\n",
                model_stats.total_usage.completion_tokens
            ));
            report.push_str(&format!(
                "  Avg Tokens/Request: {:.2}\n",
                model_stats.avg_tokens_per_request
            ));
            report.push_str(&format!("  Min Tokens: {}\n", model_stats.min_tokens));
            report.push_str(&format!("  Max Tokens: {}\n", model_stats.max_tokens));

            if let Some(estimator) = self.estimators.get(model) {
                let cost = estimator.estimate_cost(&model_stats.total_usage);
                report.push_str(&format!("  Estimated Cost: ${:.4}\n", cost));
                total_cost += cost;
            }

            report.push('\n');
        }

        report.push_str(&format!("Total Estimated Cost: ${:.4}\n", total_cost));

        report
    }
}

impl Default for TokenTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Budget limits for token usage.
#[derive(Debug, Clone)]
pub struct BudgetLimit {
    /// Maximum cost in USD
    pub max_cost: f64,
    /// Maximum total tokens
    pub max_tokens: Option<usize>,
    /// Alert threshold as percentage of max_cost (0.0 - 1.0)
    pub alert_threshold: f64,
}

impl BudgetLimit {
    /// Creates a new budget limit.
    pub fn new(max_cost: f64) -> Self {
        Self {
            max_cost,
            max_tokens: None,
            alert_threshold: 0.8, // Alert at 80% of budget
        }
    }

    /// Sets the maximum tokens.
    pub fn with_max_tokens(mut self, max_tokens: usize) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    /// Sets the alert threshold (0.0 - 1.0).
    pub fn with_alert_threshold(mut self, threshold: f64) -> Self {
        self.alert_threshold = threshold.clamp(0.0, 1.0);
        self
    }
}

/// Budget alert triggered when thresholds are exceeded.
#[derive(Debug, Clone)]
pub enum BudgetAlert {
    /// Warning: approaching budget limit
    Warning {
        current_cost: f64,
        max_cost: f64,
        percentage: f64,
    },
    /// Critical: budget limit exceeded
    Exceeded {
        current_cost: f64,
        max_cost: f64,
        overage: f64,
    },
    /// Token limit exceeded
    TokenLimitExceeded {
        current_tokens: usize,
        max_tokens: usize,
    },
}

impl BudgetAlert {
    /// Returns whether this is a critical alert.
    pub fn is_critical(&self) -> bool {
        matches!(
            self,
            BudgetAlert::Exceeded { .. } | BudgetAlert::TokenLimitExceeded { .. }
        )
    }

    /// Returns a human-readable message.
    pub fn message(&self) -> String {
        match self {
            BudgetAlert::Warning {
                current_cost,
                max_cost,
                percentage,
            } => {
                format!(
                    "Budget warning: ${:.4} of ${:.4} used ({:.1}%)",
                    current_cost, max_cost, percentage
                )
            }
            BudgetAlert::Exceeded {
                current_cost,
                max_cost,
                overage,
            } => {
                format!(
                    "Budget exceeded: ${:.4} used, ${:.4} limit (${:.4} over)",
                    current_cost, max_cost, overage
                )
            }
            BudgetAlert::TokenLimitExceeded {
                current_tokens,
                max_tokens,
            } => {
                format!(
                    "Token limit exceeded: {} tokens used, {} limit",
                    current_tokens, max_tokens
                )
            }
        }
    }
}

/// Callback function for budget alerts.
pub type BudgetAlertCallback = Arc<dyn Fn(BudgetAlert) + Send + Sync>;

/// Token tracker with budget management.
pub struct BudgetTracker {
    tracker: TokenTracker,
    limit: Option<BudgetLimit>,
    alert_callback: Option<BudgetAlertCallback>,
    alert_triggered: Arc<std::sync::Mutex<bool>>,
}

impl BudgetTracker {
    /// Creates a new budget tracker.
    pub fn new() -> Self {
        Self {
            tracker: TokenTracker::new(),
            limit: None,
            alert_callback: None,
            alert_triggered: Arc::new(std::sync::Mutex::new(false)),
        }
    }

    /// Sets the budget limit.
    pub fn with_limit(mut self, limit: BudgetLimit) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Sets the alert callback.
    pub fn with_alert_callback<F>(mut self, callback: F) -> Self
    where
        F: Fn(BudgetAlert) + Send + Sync + 'static,
    {
        self.alert_callback = Some(Arc::new(callback));
        self
    }

    /// Registers a custom cost estimator for a model.
    pub fn register_estimator(&mut self, model: impl Into<String>, estimator: CostEstimator) {
        self.tracker.register_estimator(model, estimator);
    }

    /// Records token usage and checks budget.
    pub fn record_usage(&self, model: &str, usage: TokenUsage) -> Option<BudgetAlert> {
        self.tracker.record_usage(model, usage);

        if let Some(ref limit) = self.limit {
            // Check cost budget
            let current_cost = self.tracker.estimate_total_cost_all();
            let percentage = (current_cost / limit.max_cost) * 100.0;

            if current_cost > limit.max_cost {
                let alert = BudgetAlert::Exceeded {
                    current_cost,
                    max_cost: limit.max_cost,
                    overage: current_cost - limit.max_cost,
                };
                self.trigger_alert(alert.clone());
                return Some(alert);
            } else if percentage >= (limit.alert_threshold * 100.0) {
                // Only trigger warning once
                let mut triggered = self.alert_triggered.lock().unwrap();
                if !*triggered {
                    let alert = BudgetAlert::Warning {
                        current_cost,
                        max_cost: limit.max_cost,
                        percentage,
                    };
                    self.trigger_alert(alert.clone());
                    *triggered = true;
                    return Some(alert);
                }
            }

            // Check token limit
            if let Some(max_tokens) = limit.max_tokens {
                let total_tokens: usize = self
                    .tracker
                    .get_all_stats()
                    .values()
                    .map(|s| s.total_usage.total_tokens)
                    .sum();

                if total_tokens > max_tokens {
                    let alert = BudgetAlert::TokenLimitExceeded {
                        current_tokens: total_tokens,
                        max_tokens,
                    };
                    self.trigger_alert(alert.clone());
                    return Some(alert);
                }
            }
        }

        None
    }

    fn trigger_alert(&self, alert: BudgetAlert) {
        if let Some(ref callback) = self.alert_callback {
            callback(alert.clone());
        }
        tracing::warn!("{}", alert.message());
    }

    /// Gets the current budget status.
    pub fn budget_status(&self) -> BudgetStatus {
        let current_cost = self.tracker.estimate_total_cost_all();
        let total_tokens: usize = self
            .tracker
            .get_all_stats()
            .values()
            .map(|s| s.total_usage.total_tokens)
            .sum();

        let (remaining_cost, cost_percentage) = if let Some(ref limit) = self.limit {
            let remaining = (limit.max_cost - current_cost).max(0.0);
            let percentage = (current_cost / limit.max_cost) * 100.0;
            (Some(remaining), Some(percentage))
        } else {
            (None, None)
        };

        let (remaining_tokens, token_percentage) = if let Some(ref limit) = self.limit {
            if let Some(max_tokens) = limit.max_tokens {
                let remaining = max_tokens.saturating_sub(total_tokens);
                let percentage = (total_tokens as f64 / max_tokens as f64) * 100.0;
                (Some(remaining), Some(percentage))
            } else {
                (None, None)
            }
        } else {
            (None, None)
        };

        BudgetStatus {
            current_cost,
            remaining_cost,
            cost_percentage,
            total_tokens,
            remaining_tokens,
            token_percentage,
        }
    }

    /// Resets the budget tracker.
    pub fn reset(&self) {
        self.tracker.reset();
        *self.alert_triggered.lock().unwrap() = false;
    }

    /// Gets the underlying token tracker.
    pub fn tracker(&self) -> &TokenTracker {
        &self.tracker
    }
}

impl Default for BudgetTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Current budget status.
#[derive(Debug, Clone)]
pub struct BudgetStatus {
    /// Current total cost
    pub current_cost: f64,
    /// Remaining budget (if limit set)
    pub remaining_cost: Option<f64>,
    /// Cost as percentage of budget (if limit set)
    pub cost_percentage: Option<f64>,
    /// Total tokens used
    pub total_tokens: usize,
    /// Remaining tokens (if limit set)
    pub remaining_tokens: Option<usize>,
    /// Tokens as percentage of limit (if limit set)
    pub token_percentage: Option<f64>,
}

impl BudgetStatus {
    /// Returns whether the budget is healthy.
    pub fn is_healthy(&self) -> bool {
        if let Some(percentage) = self.cost_percentage {
            percentage < 80.0
        } else {
            true
        }
    }

    /// Returns whether the budget is exceeded.
    pub fn is_exceeded(&self) -> bool {
        if let Some(percentage) = self.cost_percentage {
            percentage >= 100.0
        } else {
            false
        }
    }
}

/// Simple token estimator.
///
/// This provides a rough estimate of token count without needing tiktoken.
/// For production use with OpenAI, consider using the tiktoken library.
pub struct TokenEstimator;

impl TokenEstimator {
    /// Estimates token count for text using a simple heuristic.
    ///
    /// This uses the approximation: tokens ≈ words * 1.3
    /// This is based on the observation that on average, English text
    /// has about 0.75 words per token (or 1.3 tokens per word).
    pub fn estimate_tokens(text: &str) -> usize {
        let word_count = text.split_whitespace().count();
        // Rough approximation: 1 word ≈ 1.3 tokens
        ((word_count as f64) * 1.3).ceil() as usize
    }

    /// Estimates tokens for a prompt with system message.
    pub fn estimate_prompt_tokens(prompt: &str, system_prompt: Option<&str>) -> usize {
        let mut total = Self::estimate_tokens(prompt);
        if let Some(sys) = system_prompt {
            total += Self::estimate_tokens(sys);
        }
        // Add overhead for message formatting (role, structure, etc.)
        total + 10
    }

    /// Checks if a prompt would exceed the model's token limit.
    pub fn would_exceed_limit(
        prompt: &str,
        system_prompt: Option<&str>,
        max_tokens: usize,
        model_limit: usize,
    ) -> bool {
        let prompt_tokens = Self::estimate_prompt_tokens(prompt, system_prompt);
        prompt_tokens + max_tokens > model_limit
    }

    /// Estimates cost for a request before sending it.
    pub fn estimate_request_cost(
        prompt: &str,
        system_prompt: Option<&str>,
        expected_completion_tokens: usize,
        estimator: &CostEstimator,
    ) -> f64 {
        let prompt_tokens = Self::estimate_prompt_tokens(prompt, system_prompt);
        let usage = TokenUsage::new(prompt_tokens, expected_completion_tokens);
        estimator.estimate_cost(&usage)
    }

    /// Truncates text to fit within a token limit.
    pub fn truncate_to_tokens(text: &str, max_tokens: usize) -> String {
        let estimated_tokens = Self::estimate_tokens(text);

        if estimated_tokens <= max_tokens {
            return text.to_string();
        }

        // Calculate how many words we can keep
        let word_count = text.split_whitespace().count();
        let target_words = ((max_tokens as f64) / 1.3) as usize;

        if target_words >= word_count {
            return text.to_string();
        }

        // Truncate to target word count
        text.split_whitespace()
            .take(target_words)
            .collect::<Vec<_>>()
            .join(" ")
    }
}

/// Model token limits for common models.
pub struct ModelLimits;

impl ModelLimits {
    pub const GPT_4: usize = 8192;
    pub const GPT_4_32K: usize = 32768;
    pub const GPT_4_TURBO: usize = 128000;
    pub const GPT_35_TURBO: usize = 4096;
    pub const GPT_35_TURBO_16K: usize = 16384;
    pub const CLAUDE_3_OPUS: usize = 200000;
    pub const CLAUDE_3_SONNET: usize = 200000;
    pub const CLAUDE_3_HAIKU: usize = 200000;
    pub const GEMINI_PRO: usize = 32760;
    pub const GEMINI_PRO_15: usize = 1048576;

    /// Gets the token limit for a model by name.
    pub fn get_limit(model: &str) -> Option<usize> {
        match model {
            "gpt-4" => Some(Self::GPT_4),
            "gpt-4-32k" => Some(Self::GPT_4_32K),
            "gpt-4-turbo" | "gpt-4-turbo-preview" => Some(Self::GPT_4_TURBO),
            "gpt-3.5-turbo" => Some(Self::GPT_35_TURBO),
            "gpt-3.5-turbo-16k" => Some(Self::GPT_35_TURBO_16K),
            "claude-3-opus" => Some(Self::CLAUDE_3_OPUS),
            "claude-3-sonnet" => Some(Self::CLAUDE_3_SONNET),
            "claude-3-haiku" => Some(Self::CLAUDE_3_HAIKU),
            "gemini-pro" => Some(Self::GEMINI_PRO),
            "gemini-1.5-pro" | "gemini-pro-1.5" => Some(Self::GEMINI_PRO_15),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_usage() {
        let usage = TokenUsage::new(100, 50);
        assert_eq!(usage.prompt_tokens, 100);
        assert_eq!(usage.completion_tokens, 50);
        assert_eq!(usage.total_tokens, 150);
    }

    #[test]
    fn test_token_usage_add() {
        let mut usage1 = TokenUsage::new(100, 50);
        let usage2 = TokenUsage::new(200, 100);
        usage1.add(&usage2);
        assert_eq!(usage1.prompt_tokens, 300);
        assert_eq!(usage1.completion_tokens, 150);
        assert_eq!(usage1.total_tokens, 450);
    }

    #[test]
    fn test_cost_estimator() {
        let estimator = CostEstimator::openai_gpt4();
        let usage = TokenUsage::new(1000, 500);
        let cost = estimator.estimate_cost(&usage);
        assert!((cost - 0.06).abs() < 0.001); // 0.03 + 0.03 = 0.06
    }

    #[test]
    fn test_token_stats() {
        let mut stats = TokenStats::new();
        stats.update(&TokenUsage::new(100, 50));
        stats.update(&TokenUsage::new(200, 100));

        assert_eq!(stats.request_count, 2);
        assert_eq!(stats.total_usage.total_tokens, 450);
        assert_eq!(stats.min_tokens, 150);
        assert_eq!(stats.max_tokens, 300);
        assert!((stats.avg_tokens_per_request - 225.0).abs() < 0.1);
    }

    #[test]
    fn test_token_tracker() {
        let tracker = TokenTracker::new();

        tracker.record_usage("gpt-4", TokenUsage::new(1000, 500));
        tracker.record_usage("gpt-4", TokenUsage::new(2000, 1000));

        let stats = tracker.get_stats("gpt-4").unwrap();
        assert_eq!(stats.request_count, 2);
        assert_eq!(stats.total_usage.total_tokens, 4500);

        let cost = tracker.estimate_total_cost("gpt-4").unwrap();
        assert!(cost > 0.0);
    }

    #[test]
    fn test_token_tracker_report() {
        let tracker = TokenTracker::new();
        tracker.record_usage("gpt-4", TokenUsage::new(1000, 500));

        let report = tracker.generate_report();
        assert!(report.contains("Token Usage Report"));
        assert!(report.contains("gpt-4"));
        assert!(report.contains("Total Estimated Cost"));
    }

    #[test]
    fn test_token_tracker_reset() {
        let tracker = TokenTracker::new();
        tracker.record_usage("gpt-4", TokenUsage::new(1000, 500));

        assert!(tracker.get_stats("gpt-4").is_some());

        tracker.reset();
        assert!(tracker.get_stats("gpt-4").is_none());
    }

    #[test]
    fn test_budget_tracker_basic() {
        let limit = BudgetLimit::new(1.0).with_max_tokens(10000);
        let tracker = BudgetTracker::new().with_limit(limit);

        tracker.record_usage("gpt-4", TokenUsage::new(1000, 500));

        let status = tracker.budget_status();
        assert!(status.current_cost > 0.0);
        assert!(status.remaining_cost.is_some());
        assert!(status.is_healthy());
    }

    #[test]
    fn test_budget_alert_threshold() {
        let limit = BudgetLimit::new(0.1).with_alert_threshold(0.5);
        let tracker = BudgetTracker::new().with_limit(limit);

        // Record usage that exceeds 50% of budget
        tracker.record_usage("gpt-4", TokenUsage::new(2000, 1000));

        let status = tracker.budget_status();
        assert!(status.cost_percentage.unwrap() > 50.0);
    }

    #[test]
    fn test_budget_alert_message() {
        let alert = BudgetAlert::Warning {
            current_cost: 8.0,
            max_cost: 10.0,
            percentage: 80.0,
        };
        assert!(alert.message().contains("80"));
        assert!(!alert.is_critical());

        let alert = BudgetAlert::Exceeded {
            current_cost: 12.0,
            max_cost: 10.0,
            overage: 2.0,
        };
        assert!(alert.message().contains("exceeded"));
        assert!(alert.is_critical());
    }

    #[test]
    fn test_budget_status() {
        let limit = BudgetLimit::new(10.0);
        let tracker = BudgetTracker::new().with_limit(limit);

        tracker.record_usage("gpt-4", TokenUsage::new(1000, 500));

        let status = tracker.budget_status();
        assert!(!status.is_exceeded());
        assert!(status.is_healthy());
    }

    #[test]
    fn test_token_estimator() {
        let text = "Hello world this is a test";
        let estimate = TokenEstimator::estimate_tokens(text);

        // Should be roughly 6 words * 1.3 = ~8 tokens
        assert!((6..=10).contains(&estimate));
    }

    #[test]
    fn test_token_estimator_with_system() {
        let prompt = "Hello world";
        let system = "You are a helpful assistant";

        let estimate = TokenEstimator::estimate_prompt_tokens(prompt, Some(system));

        // Should include both prompts + overhead
        assert!(estimate > 10);
    }

    #[test]
    fn test_token_truncation() {
        let text = "one two three four five six seven eight nine ten";
        let truncated = TokenEstimator::truncate_to_tokens(text, 5);

        // Should truncate to roughly 5 tokens (about 3-4 words)
        assert!(truncated.split_whitespace().count() <= 4);
    }

    #[test]
    fn test_model_limits() {
        assert_eq!(ModelLimits::get_limit("gpt-4"), Some(8192));
        assert_eq!(ModelLimits::get_limit("claude-3-opus"), Some(200000));
        assert_eq!(ModelLimits::get_limit("gemini-1.5-pro"), Some(1048576));
        assert_eq!(ModelLimits::get_limit("unknown-model"), None);
    }

    #[test]
    fn test_would_exceed_limit() {
        let prompt = "Hello world";
        let exceeds = TokenEstimator::would_exceed_limit(prompt, None, 1000, 100);
        assert!(exceeds);

        let not_exceeds = TokenEstimator::would_exceed_limit(prompt, None, 10, 100);
        assert!(!not_exceeds);
    }
}
