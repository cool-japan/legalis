//! Benchmark suite for comparing LLM providers and models.
//!
//! This module provides tools for benchmarking different LLM providers
//! and models across various metrics like latency, quality, and cost.

use crate::{LLMProvider, TokenUsage};
use serde::{Deserialize, Serialize};
use std::time::Instant;

/// A benchmark task to evaluate models.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkTask {
    /// Task name/description
    pub name: String,
    /// The prompt to use
    pub prompt: String,
    /// Expected output (for quality evaluation)
    pub expected_output: Option<String>,
    /// Task category (e.g., "reasoning", "coding", "summarization")
    pub category: String,
}

impl BenchmarkTask {
    /// Creates a new benchmark task.
    pub fn new(
        name: impl Into<String>,
        prompt: impl Into<String>,
        category: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            prompt: prompt.into(),
            expected_output: None,
            category: category.into(),
        }
    }

    /// Sets the expected output for quality comparison.
    pub fn with_expected_output(mut self, expected: impl Into<String>) -> Self {
        self.expected_output = Some(expected.into());
        self
    }
}

/// Result of running a benchmark on a single task.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    /// Task name
    pub task_name: String,
    /// Provider name
    pub provider_name: String,
    /// Model name
    pub model_name: String,
    /// Generated output
    pub output: String,
    /// Latency in milliseconds
    pub latency_ms: u128,
    /// Token usage
    pub tokens: Option<TokenUsage>,
    /// Estimated cost in USD
    pub cost_usd: Option<f64>,
    /// Quality score (0.0 - 1.0), if expected output provided
    pub quality_score: Option<f64>,
    /// Whether the task succeeded
    pub success: bool,
    /// Error message if failed
    pub error: Option<String>,
}

impl BenchmarkResult {
    /// Creates a new benchmark result.
    pub fn new(
        task_name: impl Into<String>,
        provider_name: impl Into<String>,
        model_name: impl Into<String>,
        output: impl Into<String>,
        latency_ms: u128,
        success: bool,
    ) -> Self {
        Self {
            task_name: task_name.into(),
            provider_name: provider_name.into(),
            model_name: model_name.into(),
            output: output.into(),
            latency_ms,
            tokens: None,
            cost_usd: None,
            quality_score: None,
            success,
            error: None,
        }
    }

    /// Adds token usage information.
    pub fn with_tokens(mut self, tokens: TokenUsage) -> Self {
        self.tokens = Some(tokens);
        self
    }

    /// Adds cost information.
    pub fn with_cost(mut self, cost: f64) -> Self {
        self.cost_usd = Some(cost);
        self
    }

    /// Adds quality score.
    pub fn with_quality_score(mut self, score: f64) -> Self {
        self.quality_score = Some(score.clamp(0.0, 1.0));
        self
    }

    /// Adds error information.
    pub fn with_error(mut self, error: impl Into<String>) -> Self {
        self.error = Some(error.into());
        self
    }
}

/// Aggregated benchmark statistics across multiple tasks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkStats {
    /// Provider name
    pub provider_name: String,
    /// Model name
    pub model_name: String,
    /// Number of tasks completed
    pub tasks_completed: usize,
    /// Number of tasks failed
    pub tasks_failed: usize,
    /// Success rate (0.0 - 1.0)
    pub success_rate: f64,
    /// Average latency in milliseconds
    pub avg_latency_ms: f64,
    /// Median latency in milliseconds
    pub median_latency_ms: u128,
    /// p95 latency in milliseconds
    pub p95_latency_ms: u128,
    /// p99 latency in milliseconds
    pub p99_latency_ms: u128,
    /// Average quality score (if available)
    pub avg_quality_score: Option<f64>,
    /// Total tokens used
    pub total_tokens: u64,
    /// Total cost in USD
    pub total_cost_usd: f64,
}

impl BenchmarkStats {
    /// Computes statistics from a set of benchmark results.
    pub fn from_results(results: &[BenchmarkResult]) -> Self {
        if results.is_empty() {
            return Self::default();
        }

        let provider_name = results[0].provider_name.clone();
        let model_name = results[0].model_name.clone();

        let tasks_completed = results.iter().filter(|r| r.success).count();
        let tasks_failed = results.iter().filter(|r| !r.success).count();
        let success_rate = if !results.is_empty() {
            tasks_completed as f64 / results.len() as f64
        } else {
            0.0
        };

        let mut latencies: Vec<u128> = results.iter().map(|r| r.latency_ms).collect();
        latencies.sort_unstable();

        let avg_latency_ms = if !latencies.is_empty() {
            latencies.iter().sum::<u128>() as f64 / latencies.len() as f64
        } else {
            0.0
        };

        let median_latency_ms = if !latencies.is_empty() {
            latencies[latencies.len() / 2]
        } else {
            0
        };

        let p95_latency_ms = if !latencies.is_empty() {
            latencies[(latencies.len() as f64 * 0.95) as usize]
        } else {
            0
        };

        let p99_latency_ms = if !latencies.is_empty() {
            latencies[(latencies.len() as f64 * 0.99) as usize]
        } else {
            0
        };

        let quality_scores: Vec<f64> = results.iter().filter_map(|r| r.quality_score).collect();

        let avg_quality_score = if !quality_scores.is_empty() {
            Some(quality_scores.iter().sum::<f64>() / quality_scores.len() as f64)
        } else {
            None
        };

        let total_tokens = results
            .iter()
            .filter_map(|r| r.tokens.as_ref())
            .map(|t| t.total_tokens as u64)
            .sum();

        let total_cost_usd = results.iter().filter_map(|r| r.cost_usd).sum();

        Self {
            provider_name,
            model_name,
            tasks_completed,
            tasks_failed,
            success_rate,
            avg_latency_ms,
            median_latency_ms,
            p95_latency_ms,
            p99_latency_ms,
            avg_quality_score,
            total_tokens,
            total_cost_usd,
        }
    }

    /// Returns a score combining performance and quality (0.0 - 1.0).
    ///
    /// Lower latency and higher quality = higher score.
    pub fn overall_score(&self) -> f64 {
        let quality = self.avg_quality_score.unwrap_or(0.5);
        let speed = 1.0 / (1.0 + (self.avg_latency_ms / 1000.0)); // Normalize latency
        let reliability = self.success_rate;

        // Weighted combination
        0.4 * quality + 0.3 * speed + 0.3 * reliability
    }
}

impl Default for BenchmarkStats {
    fn default() -> Self {
        Self {
            provider_name: String::new(),
            model_name: String::new(),
            tasks_completed: 0,
            tasks_failed: 0,
            success_rate: 0.0,
            avg_latency_ms: 0.0,
            median_latency_ms: 0,
            p95_latency_ms: 0,
            p99_latency_ms: 0,
            avg_quality_score: None,
            total_tokens: 0,
            total_cost_usd: 0.0,
        }
    }
}

/// Benchmark suite for comparing models.
pub struct BenchmarkSuite {
    tasks: Vec<BenchmarkTask>,
}

impl BenchmarkSuite {
    /// Creates a new benchmark suite.
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    /// Adds a task to the suite.
    pub fn add_task(mut self, task: BenchmarkTask) -> Self {
        self.tasks.push(task);
        self
    }

    /// Adds multiple tasks to the suite.
    pub fn add_tasks(mut self, tasks: Vec<BenchmarkTask>) -> Self {
        self.tasks.extend(tasks);
        self
    }

    /// Runs the benchmark suite on a provider.
    pub async fn run<P: LLMProvider>(&self, provider: &P) -> Vec<BenchmarkResult> {
        let mut results = Vec::new();

        for task in &self.tasks {
            let start = Instant::now();
            let result = provider.generate_text(&task.prompt).await;
            let latency = start.elapsed().as_millis();

            match result {
                Ok(output) => {
                    let mut bench_result = BenchmarkResult::new(
                        task.name.clone(),
                        provider.provider_name(),
                        provider.model_name(),
                        output.clone(),
                        latency,
                        true,
                    );

                    // Compute quality score if expected output is provided
                    if let Some(expected) = &task.expected_output {
                        let score = compute_similarity(&output, expected);
                        bench_result = bench_result.with_quality_score(score);
                    }

                    results.push(bench_result);
                }
                Err(e) => {
                    results.push(
                        BenchmarkResult::new(
                            task.name.clone(),
                            provider.provider_name(),
                            provider.model_name(),
                            String::new(),
                            latency,
                            false,
                        )
                        .with_error(e.to_string()),
                    );
                }
            }
        }

        results
    }

    /// Creates a standard benchmark suite for general evaluation.
    pub fn standard() -> Self {
        Self::new()
            .add_task(
                BenchmarkTask::new(
                    "Simple Arithmetic",
                    "What is 15 * 23? Respond with just the number.",
                    "math",
                )
                .with_expected_output("345"),
            )
            .add_task(BenchmarkTask::new(
                "Code Generation",
                "Write a Python function to calculate fibonacci numbers.",
                "coding",
            ))
            .add_task(BenchmarkTask::new(
                "Summarization",
                "Summarize the following in one sentence: Artificial intelligence is transforming industries by automating tasks, improving decision-making, and enabling new capabilities.",
                "summarization",
            ))
            .add_task(BenchmarkTask::new(
                "Question Answering",
                "What is the capital of France?",
                "qa",
            ).with_expected_output("Paris"))
            .add_task(BenchmarkTask::new(
                "Reasoning",
                "If all roses are flowers and some flowers fade quickly, can we conclude that some roses fade quickly?",
                "reasoning",
            ))
    }

    /// Returns the number of tasks in the suite.
    pub fn task_count(&self) -> usize {
        self.tasks.len()
    }
}

impl Default for BenchmarkSuite {
    fn default() -> Self {
        Self::new()
    }
}

/// Computes similarity between two strings (simple character-based).
fn compute_similarity(s1: &str, s2: &str) -> f64 {
    let s1_lower = s1.trim().to_lowercase();
    let s2_lower = s2.trim().to_lowercase();

    if s1_lower == s2_lower {
        return 1.0;
    }

    // Check if s1 contains s2 or vice versa
    if s1_lower.contains(&s2_lower) || s2_lower.contains(&s1_lower) {
        return 0.8;
    }

    // Simple word overlap score
    let words1: std::collections::HashSet<&str> = s1_lower.split_whitespace().collect();
    let words2: std::collections::HashSet<&str> = s2_lower.split_whitespace().collect();

    let intersection = words1.intersection(&words2).count();
    let union = words1.union(&words2).count();

    if union == 0 {
        0.0
    } else {
        intersection as f64 / union as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MockProvider;

    #[test]
    fn test_benchmark_task_creation() {
        let task =
            BenchmarkTask::new("Test Task", "What is 2+2?", "math").with_expected_output("4");

        assert_eq!(task.name, "Test Task");
        assert_eq!(task.prompt, "What is 2+2?");
        assert_eq!(task.category, "math");
        assert_eq!(task.expected_output, Some("4".to_string()));
    }

    #[test]
    fn test_benchmark_result_creation() {
        let result = BenchmarkResult::new("Test", "OpenAI", "gpt-4", "Output", 100, true)
            .with_cost(0.01)
            .with_quality_score(0.95);

        assert_eq!(result.task_name, "Test");
        assert_eq!(result.provider_name, "OpenAI");
        assert_eq!(result.latency_ms, 100);
        assert_eq!(result.cost_usd, Some(0.01));
        assert_eq!(result.quality_score, Some(0.95));
    }

    #[test]
    fn test_benchmark_stats_from_results() {
        let results = vec![
            BenchmarkResult::new("Task1", "Provider", "Model", "Output1", 100, true)
                .with_quality_score(0.9),
            BenchmarkResult::new("Task2", "Provider", "Model", "Output2", 200, true)
                .with_quality_score(0.8),
            BenchmarkResult::new("Task3", "Provider", "Model", "Output3", 150, false),
        ];

        let stats = BenchmarkStats::from_results(&results);

        assert_eq!(stats.tasks_completed, 2);
        assert_eq!(stats.tasks_failed, 1);
        assert!((stats.success_rate - 0.6666).abs() < 0.01);
        assert_eq!(stats.median_latency_ms, 150);
        assert!(stats.avg_quality_score.unwrap() > 0.84);
        assert!(stats.avg_quality_score.unwrap() < 0.86);
    }

    #[test]
    fn test_compute_similarity() {
        assert_eq!(compute_similarity("hello", "hello"), 1.0);
        assert_eq!(compute_similarity("Hello", "hello"), 1.0);
        assert!(compute_similarity("hello world", "world") > 0.5);
        assert!(compute_similarity("completely different", "unrelated words") < 0.5);
    }

    #[test]
    fn test_benchmark_suite_creation() {
        let suite = BenchmarkSuite::new()
            .add_task(BenchmarkTask::new("Task1", "Prompt1", "cat1"))
            .add_task(BenchmarkTask::new("Task2", "Prompt2", "cat2"));

        assert_eq!(suite.task_count(), 2);
    }

    #[test]
    fn test_standard_benchmark_suite() {
        let suite = BenchmarkSuite::standard();
        assert!(suite.task_count() >= 5);
    }

    #[tokio::test]
    async fn test_benchmark_run() {
        let provider = MockProvider::default();
        let suite = BenchmarkSuite::new().add_task(BenchmarkTask::new("Test", "Hello", "test"));

        let results = suite.run(&provider).await;

        assert_eq!(results.len(), 1);
        assert!(results[0].success);
        assert!(!results[0].output.is_empty());
    }

    #[test]
    fn test_overall_score() {
        let stats = BenchmarkStats {
            provider_name: "Test".to_string(),
            model_name: "test-model".to_string(),
            tasks_completed: 10,
            tasks_failed: 0,
            success_rate: 1.0,
            avg_latency_ms: 100.0,
            median_latency_ms: 100,
            p95_latency_ms: 150,
            p99_latency_ms: 200,
            avg_quality_score: Some(0.9),
            total_tokens: 1000,
            total_cost_usd: 0.1,
        };

        let score = stats.overall_score();
        assert!(score > 0.6);
        assert!(score <= 1.0);
    }
}
