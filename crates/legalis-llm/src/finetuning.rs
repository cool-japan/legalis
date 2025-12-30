//! Fine-tuning support for LLMs.
//!
//! This module provides:
//! - LoRA (Low-Rank Adaptation) adapter support
//! - Dataset preparation utilities for fine-tuning
//! - Training metrics tracking and visualization
//! - A/B testing for fine-tuned models

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// LoRA (Low-Rank Adaptation) configuration.
///
/// LoRA is a parameter-efficient fine-tuning technique that
/// adds trainable low-rank decomposition matrices to frozen
/// pre-trained model weights.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoRAConfig {
    /// Rank of the low-rank decomposition (typically 4-64)
    pub rank: usize,
    /// Alpha parameter for scaling (typically same as rank or 2x rank)
    pub alpha: f32,
    /// Dropout probability for LoRA layers
    pub dropout: f32,
    /// Target modules to apply LoRA (e.g., ["q_proj", "v_proj"])
    pub target_modules: Vec<String>,
    /// Whether to merge adapter weights into base model after training
    pub merge_weights: bool,
}

impl Default for LoRAConfig {
    fn default() -> Self {
        Self {
            rank: 8,
            alpha: 16.0,
            dropout: 0.05,
            target_modules: vec![
                "q_proj".to_string(),
                "k_proj".to_string(),
                "v_proj".to_string(),
                "o_proj".to_string(),
            ],
            merge_weights: false,
        }
    }
}

impl LoRAConfig {
    /// Creates a new LoRA configuration with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the rank.
    pub fn with_rank(mut self, rank: usize) -> Self {
        self.rank = rank;
        self
    }

    /// Sets the alpha parameter.
    pub fn with_alpha(mut self, alpha: f32) -> Self {
        self.alpha = alpha;
        self
    }

    /// Sets the dropout probability.
    pub fn with_dropout(mut self, dropout: f32) -> Self {
        self.dropout = dropout.clamp(0.0, 1.0);
        self
    }

    /// Sets the target modules.
    pub fn with_target_modules(mut self, modules: Vec<String>) -> Self {
        self.target_modules = modules;
        self
    }

    /// Sets whether to merge weights after training.
    pub fn with_merge_weights(mut self, merge: bool) -> Self {
        self.merge_weights = merge;
        self
    }

    /// Validates the configuration.
    pub fn validate(&self) -> Result<()> {
        if self.rank == 0 {
            return Err(anyhow!("LoRA rank must be greater than 0"));
        }
        if self.alpha <= 0.0 {
            return Err(anyhow!("LoRA alpha must be positive"));
        }
        if self.target_modules.is_empty() {
            return Err(anyhow!("At least one target module must be specified"));
        }
        Ok(())
    }
}

/// LoRA adapter metadata and weights.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoRAAdapter {
    /// Adapter name/identifier
    pub name: String,
    /// LoRA configuration
    pub config: LoRAConfig,
    /// Base model name
    pub base_model: String,
    /// Adapter weights (serialized)
    #[serde(skip)]
    pub weights: HashMap<String, Vec<f32>>,
    /// Adapter metadata
    pub metadata: HashMap<String, String>,
}

impl LoRAAdapter {
    /// Creates a new LoRA adapter.
    pub fn new(name: impl Into<String>, base_model: impl Into<String>, config: LoRAConfig) -> Self {
        Self {
            name: name.into(),
            config,
            base_model: base_model.into(),
            weights: HashMap::new(),
            metadata: HashMap::new(),
        }
    }

    /// Adds metadata.
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Saves the adapter to disk.
    pub async fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        tokio::fs::write(path, json).await?;
        Ok(())
    }

    /// Loads an adapter from disk.
    pub async fn load(path: impl AsRef<Path>) -> Result<Self> {
        let json = tokio::fs::read_to_string(path).await?;
        let adapter: Self = serde_json::from_str(&json)?;
        adapter.config.validate()?;
        Ok(adapter)
    }
}

/// Fine-tuning dataset format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FineTuningExample {
    /// Input prompt/context
    pub prompt: String,
    /// Expected completion/output
    pub completion: String,
    /// Optional metadata
    pub metadata: HashMap<String, String>,
}

impl FineTuningExample {
    /// Creates a new fine-tuning example.
    pub fn new(prompt: impl Into<String>, completion: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            completion: completion.into(),
            metadata: HashMap::new(),
        }
    }

    /// Adds metadata.
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Validates the example.
    pub fn validate(&self) -> Result<()> {
        if self.prompt.trim().is_empty() {
            return Err(anyhow!("Prompt cannot be empty"));
        }
        if self.completion.trim().is_empty() {
            return Err(anyhow!("Completion cannot be empty"));
        }
        Ok(())
    }
}

/// Dataset preparation utilities.
pub struct DatasetBuilder {
    examples: Vec<FineTuningExample>,
    train_split: f32,
    validation_split: f32,
}

impl DatasetBuilder {
    /// Creates a new dataset builder.
    pub fn new() -> Self {
        Self {
            examples: Vec::new(),
            train_split: 0.8,
            validation_split: 0.2,
        }
    }

    /// Adds an example to the dataset.
    pub fn add_example(mut self, example: FineTuningExample) -> Self {
        self.examples.push(example);
        self
    }

    /// Adds multiple examples.
    pub fn add_examples(mut self, examples: Vec<FineTuningExample>) -> Self {
        self.examples.extend(examples);
        self
    }

    /// Sets the train/validation split ratio.
    pub fn with_split(mut self, train: f32, validation: f32) -> Result<Self> {
        if (train + validation - 1.0).abs() > 0.001 {
            return Err(anyhow!("Train and validation splits must sum to 1.0"));
        }
        if train <= 0.0 || validation <= 0.0 {
            return Err(anyhow!("Splits must be positive"));
        }
        self.train_split = train;
        self.validation_split = validation;
        Ok(self)
    }

    /// Validates all examples.
    pub fn validate(&self) -> Result<()> {
        if self.examples.is_empty() {
            return Err(anyhow!("Dataset must contain at least one example"));
        }
        for (i, example) in self.examples.iter().enumerate() {
            example
                .validate()
                .map_err(|e| anyhow!("Example {} is invalid: {}", i, e))?;
        }
        Ok(())
    }

    /// Builds the dataset and splits into train/validation.
    pub fn build(mut self) -> Result<FineTuningDataset> {
        self.validate()?;

        // Shuffle examples
        use rand::seq::SliceRandom;
        let mut rng = rand::rng();
        self.examples.shuffle(&mut rng);

        // Split into train/validation
        let train_size = (self.examples.len() as f32 * self.train_split) as usize;
        let train_examples = self.examples.drain(..train_size).collect();
        let validation_examples = self.examples;

        Ok(FineTuningDataset {
            train_examples,
            validation_examples,
        })
    }

    /// Saves the dataset to JSONL format.
    pub async fn save_jsonl(&self, path: impl AsRef<Path>) -> Result<()> {
        use tokio::io::AsyncWriteExt;

        let mut file = tokio::fs::File::create(path).await?;
        for example in &self.examples {
            let json = serde_json::to_string(example)?;
            file.write_all(json.as_bytes()).await?;
            file.write_all(b"\n").await?;
        }
        Ok(())
    }

    /// Loads a dataset from JSONL format.
    pub async fn load_jsonl(path: impl AsRef<Path>) -> Result<Self> {
        use tokio::io::AsyncBufReadExt;

        let file = tokio::fs::File::open(path).await?;
        let reader = tokio::io::BufReader::new(file);
        let mut lines = reader.lines();

        let mut builder = Self::new();
        while let Some(line) = lines.next_line().await? {
            if !line.trim().is_empty() {
                let example: FineTuningExample = serde_json::from_str(&line)?;
                builder = builder.add_example(example);
            }
        }

        Ok(builder)
    }
}

impl Default for DatasetBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Fine-tuning dataset with train/validation split.
#[derive(Debug, Clone)]
pub struct FineTuningDataset {
    pub train_examples: Vec<FineTuningExample>,
    pub validation_examples: Vec<FineTuningExample>,
}

impl FineTuningDataset {
    /// Returns the total number of examples.
    pub fn len(&self) -> usize {
        self.train_examples.len() + self.validation_examples.len()
    }

    /// Returns whether the dataset is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns statistics about the dataset.
    pub fn stats(&self) -> DatasetStats {
        DatasetStats {
            total_examples: self.len(),
            train_examples: self.train_examples.len(),
            validation_examples: self.validation_examples.len(),
            avg_prompt_length: self.avg_length(&self.train_examples, |e| e.prompt.len()),
            avg_completion_length: self.avg_length(&self.train_examples, |e| e.completion.len()),
        }
    }

    fn avg_length<F>(&self, examples: &[FineTuningExample], f: F) -> f32
    where
        F: Fn(&FineTuningExample) -> usize,
    {
        if examples.is_empty() {
            return 0.0;
        }
        let total: usize = examples.iter().map(&f).sum();
        total as f32 / examples.len() as f32
    }
}

/// Dataset statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetStats {
    pub total_examples: usize,
    pub train_examples: usize,
    pub validation_examples: usize,
    pub avg_prompt_length: f32,
    pub avg_completion_length: f32,
}

/// Training metrics tracker.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TrainingMetrics {
    /// Current epoch number
    pub epoch: usize,
    /// Current step number
    pub step: usize,
    /// Training loss
    pub train_loss: f32,
    /// Validation loss
    pub validation_loss: Option<f32>,
    /// Learning rate
    pub learning_rate: f32,
    /// Gradient norm
    pub grad_norm: Option<f32>,
    /// Custom metrics
    pub custom_metrics: HashMap<String, f32>,
}

impl TrainingMetrics {
    /// Creates a new metrics instance.
    pub fn new(epoch: usize, step: usize) -> Self {
        Self {
            epoch,
            step,
            ..Default::default()
        }
    }

    /// Sets the training loss.
    pub fn with_train_loss(mut self, loss: f32) -> Self {
        self.train_loss = loss;
        self
    }

    /// Sets the validation loss.
    pub fn with_validation_loss(mut self, loss: f32) -> Self {
        self.validation_loss = Some(loss);
        self
    }

    /// Sets the learning rate.
    pub fn with_learning_rate(mut self, lr: f32) -> Self {
        self.learning_rate = lr;
        self
    }

    /// Sets the gradient norm.
    pub fn with_grad_norm(mut self, norm: f32) -> Self {
        self.grad_norm = Some(norm);
        self
    }

    /// Adds a custom metric.
    pub fn with_metric(mut self, name: impl Into<String>, value: f32) -> Self {
        self.custom_metrics.insert(name.into(), value);
        self
    }
}

/// Training metrics history.
#[derive(Debug, Clone, Default)]
pub struct MetricsHistory {
    metrics: Vec<TrainingMetrics>,
}

impl MetricsHistory {
    /// Creates a new metrics history.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds metrics for a step.
    pub fn record(&mut self, metrics: TrainingMetrics) {
        self.metrics.push(metrics);
    }

    /// Returns all recorded metrics.
    pub fn all(&self) -> &[TrainingMetrics] {
        &self.metrics
    }

    /// Returns the latest metrics.
    pub fn latest(&self) -> Option<&TrainingMetrics> {
        self.metrics.last()
    }

    /// Returns metrics for a specific epoch.
    pub fn epoch_metrics(&self, epoch: usize) -> Vec<&TrainingMetrics> {
        self.metrics.iter().filter(|m| m.epoch == epoch).collect()
    }

    /// Computes average loss for an epoch.
    pub fn avg_loss_for_epoch(&self, epoch: usize) -> f32 {
        let epoch_metrics = self.epoch_metrics(epoch);
        if epoch_metrics.is_empty() {
            return 0.0;
        }
        let total: f32 = epoch_metrics.iter().map(|m| m.train_loss).sum();
        total / epoch_metrics.len() as f32
    }

    /// Returns the best (lowest) validation loss.
    pub fn best_validation_loss(&self) -> Option<f32> {
        self.metrics
            .iter()
            .filter_map(|m| m.validation_loss)
            .min_by(|a, b| a.partial_cmp(b).unwrap())
    }

    /// Saves the history to a JSON file.
    pub async fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.metrics)?;
        tokio::fs::write(path, json).await?;
        Ok(())
    }

    /// Loads history from a JSON file.
    pub async fn load(path: impl AsRef<Path>) -> Result<Self> {
        let json = tokio::fs::read_to_string(path).await?;
        let metrics: Vec<TrainingMetrics> = serde_json::from_str(&json)?;
        Ok(Self { metrics })
    }
}

/// Model evaluation benchmark.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationBenchmark {
    /// Benchmark name
    pub name: String,
    /// Test cases
    pub test_cases: Vec<EvaluationCase>,
    /// Benchmark metadata
    pub metadata: HashMap<String, String>,
}

impl EvaluationBenchmark {
    /// Creates a new benchmark.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            test_cases: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Adds a test case.
    pub fn add_case(mut self, case: EvaluationCase) -> Self {
        self.test_cases.push(case);
        self
    }

    /// Adds multiple test cases.
    pub fn add_cases(mut self, cases: Vec<EvaluationCase>) -> Self {
        self.test_cases.extend(cases);
        self
    }

    /// Adds metadata.
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Returns the number of test cases.
    pub fn len(&self) -> usize {
        self.test_cases.len()
    }

    /// Returns whether the benchmark is empty.
    pub fn is_empty(&self) -> bool {
        self.test_cases.is_empty()
    }

    /// Saves the benchmark to disk.
    pub async fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        tokio::fs::write(path, json).await?;
        Ok(())
    }

    /// Loads a benchmark from disk.
    pub async fn load(path: impl AsRef<Path>) -> Result<Self> {
        let json = tokio::fs::read_to_string(path).await?;
        let benchmark: Self = serde_json::from_str(&json)?;
        Ok(benchmark)
    }
}

/// Single evaluation test case.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationCase {
    /// Input prompt
    pub input: String,
    /// Expected output
    pub expected: String,
    /// Category/label
    pub category: Option<String>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl EvaluationCase {
    /// Creates a new evaluation case.
    pub fn new(input: impl Into<String>, expected: impl Into<String>) -> Self {
        Self {
            input: input.into(),
            expected: expected.into(),
            category: None,
            metadata: HashMap::new(),
        }
    }

    /// Sets the category.
    pub fn with_category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    /// Adds metadata.
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

/// Result of evaluating a model on a benchmark.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationResult {
    /// Model identifier
    pub model_id: String,
    /// Benchmark name
    pub benchmark_name: String,
    /// Individual case results
    pub case_results: Vec<CaseResult>,
    /// Overall metrics
    pub metrics: EvaluationMetrics,
    /// Evaluation timestamp
    pub timestamp: String,
}

impl EvaluationResult {
    /// Creates a new evaluation result.
    pub fn new(model_id: impl Into<String>, benchmark_name: impl Into<String>) -> Self {
        Self {
            model_id: model_id.into(),
            benchmark_name: benchmark_name.into(),
            case_results: Vec::new(),
            metrics: EvaluationMetrics::default(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Adds a case result.
    pub fn add_result(mut self, result: CaseResult) -> Self {
        self.case_results.push(result);
        self
    }

    /// Computes overall metrics from case results.
    pub fn compute_metrics(mut self) -> Self {
        if self.case_results.is_empty() {
            return self;
        }

        let total = self.case_results.len() as f32;
        let exact_matches = self.case_results.iter().filter(|r| r.exact_match).count() as f32;
        let avg_similarity = self
            .case_results
            .iter()
            .map(|r| r.similarity_score)
            .sum::<f32>()
            / total;
        let avg_latency = self.case_results.iter().map(|r| r.latency_ms).sum::<f32>() / total;

        self.metrics = EvaluationMetrics {
            accuracy: exact_matches / total,
            avg_similarity,
            avg_latency_ms: avg_latency,
            total_cases: self.case_results.len(),
            passed_cases: exact_matches as usize,
        };

        self
    }

    /// Saves the result to disk.
    pub async fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        tokio::fs::write(path, json).await?;
        Ok(())
    }

    /// Loads a result from disk.
    pub async fn load(path: impl AsRef<Path>) -> Result<Self> {
        let json = tokio::fs::read_to_string(path).await?;
        let result: Self = serde_json::from_str(&json)?;
        Ok(result)
    }
}

/// Result for a single case.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseResult {
    /// Input used
    pub input: String,
    /// Expected output
    pub expected: String,
    /// Actual output from model
    pub actual: String,
    /// Whether exact match
    pub exact_match: bool,
    /// Similarity score (0.0-1.0)
    pub similarity_score: f32,
    /// Latency in milliseconds
    pub latency_ms: f32,
}

impl CaseResult {
    /// Creates a new case result.
    pub fn new(input: String, expected: String, actual: String, latency_ms: f32) -> Self {
        let exact_match = expected.trim() == actual.trim();
        let similarity_score = Self::compute_similarity(&expected, &actual);

        Self {
            input,
            expected,
            actual,
            exact_match,
            similarity_score,
            latency_ms,
        }
    }

    /// Computes similarity between two strings (simple Jaccard similarity).
    fn compute_similarity(a: &str, b: &str) -> f32 {
        if a == b {
            return 1.0;
        }

        let words_a: std::collections::HashSet<_> = a.split_whitespace().collect();
        let words_b: std::collections::HashSet<_> = b.split_whitespace().collect();

        if words_a.is_empty() && words_b.is_empty() {
            return 1.0;
        }

        let intersection = words_a.intersection(&words_b).count();
        let union = words_a.union(&words_b).count();

        if union == 0 {
            return 0.0;
        }

        intersection as f32 / union as f32
    }
}

/// Overall evaluation metrics.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EvaluationMetrics {
    /// Accuracy (exact matches / total)
    pub accuracy: f32,
    /// Average similarity score
    pub avg_similarity: f32,
    /// Average latency in milliseconds
    pub avg_latency_ms: f32,
    /// Total number of test cases
    pub total_cases: usize,
    /// Number of passed cases
    pub passed_cases: usize,
}

/// A/B testing framework for comparing fine-tuned models.
#[derive(Debug, Clone)]
pub struct ABTest {
    /// Test name
    pub name: String,
    /// Model A identifier
    pub model_a: String,
    /// Model B identifier
    pub model_b: String,
    /// Test cases
    pub test_cases: Vec<EvaluationCase>,
    /// Results for model A
    pub results_a: Vec<CaseResult>,
    /// Results for model B
    pub results_b: Vec<CaseResult>,
}

impl ABTest {
    /// Creates a new A/B test.
    pub fn new(
        name: impl Into<String>,
        model_a: impl Into<String>,
        model_b: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            model_a: model_a.into(),
            model_b: model_b.into(),
            test_cases: Vec::new(),
            results_a: Vec::new(),
            results_b: Vec::new(),
        }
    }

    /// Adds test cases.
    pub fn add_test_cases(mut self, cases: Vec<EvaluationCase>) -> Self {
        self.test_cases.extend(cases);
        self
    }

    /// Adds a result for model A.
    pub fn add_result_a(&mut self, result: CaseResult) {
        self.results_a.push(result);
    }

    /// Adds a result for model B.
    pub fn add_result_b(&mut self, result: CaseResult) {
        self.results_b.push(result);
    }

    /// Analyzes the A/B test results.
    pub fn analyze(&self) -> ABTestAnalysis {
        let metrics_a = self.compute_metrics(&self.results_a);
        let metrics_b = self.compute_metrics(&self.results_b);

        let winner = if metrics_a.avg_similarity > metrics_b.avg_similarity {
            Some(self.model_a.clone())
        } else if metrics_b.avg_similarity > metrics_a.avg_similarity {
            Some(self.model_b.clone())
        } else {
            None
        };

        let improvement = if metrics_a.avg_similarity > 0.0 {
            ((metrics_b.avg_similarity - metrics_a.avg_similarity) / metrics_a.avg_similarity)
                * 100.0
        } else {
            0.0
        };

        ABTestAnalysis {
            test_name: self.name.clone(),
            model_a: self.model_a.clone(),
            model_b: self.model_b.clone(),
            metrics_a,
            metrics_b,
            winner,
            improvement_percent: improvement,
            total_cases: self.test_cases.len(),
        }
    }

    fn compute_metrics(&self, results: &[CaseResult]) -> EvaluationMetrics {
        if results.is_empty() {
            return EvaluationMetrics::default();
        }

        let total = results.len() as f32;
        let exact_matches = results.iter().filter(|r| r.exact_match).count() as f32;
        let avg_similarity = results.iter().map(|r| r.similarity_score).sum::<f32>() / total;
        let avg_latency = results.iter().map(|r| r.latency_ms).sum::<f32>() / total;

        EvaluationMetrics {
            accuracy: exact_matches / total,
            avg_similarity,
            avg_latency_ms: avg_latency,
            total_cases: results.len(),
            passed_cases: exact_matches as usize,
        }
    }
}

/// A/B test analysis results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ABTestAnalysis {
    /// Test name
    pub test_name: String,
    /// Model A identifier
    pub model_a: String,
    /// Model B identifier
    pub model_b: String,
    /// Metrics for model A
    pub metrics_a: EvaluationMetrics,
    /// Metrics for model B
    pub metrics_b: EvaluationMetrics,
    /// Winner (if any)
    pub winner: Option<String>,
    /// Improvement percentage (B vs A)
    pub improvement_percent: f32,
    /// Total test cases
    pub total_cases: usize,
}

impl ABTestAnalysis {
    /// Generates a summary report.
    pub fn summary(&self) -> String {
        let winner_str = self
            .winner
            .as_ref()
            .map(|w| format!("Winner: {}", w))
            .unwrap_or_else(|| "Result: Tie".to_string());

        format!(
            "A/B Test: {}\n\
             Model A: {} (accuracy: {:.2}%, avg_similarity: {:.3})\n\
             Model B: {} (accuracy: {:.2}%, avg_similarity: {:.3})\n\
             {}\n\
             Improvement: {:.2}%\n\
             Total cases: {}",
            self.test_name,
            self.model_a,
            self.metrics_a.accuracy * 100.0,
            self.metrics_a.avg_similarity,
            self.model_b,
            self.metrics_b.accuracy * 100.0,
            self.metrics_b.avg_similarity,
            winner_str,
            self.improvement_percent,
            self.total_cases
        )
    }

    /// Saves the analysis to disk.
    pub async fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        tokio::fs::write(path, json).await?;
        Ok(())
    }

    /// Loads an analysis from disk.
    pub async fn load(path: impl AsRef<Path>) -> Result<Self> {
        let json = tokio::fs::read_to_string(path).await?;
        let analysis: Self = serde_json::from_str(&json)?;
        Ok(analysis)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lora_config_defaults() {
        let config = LoRAConfig::default();
        assert_eq!(config.rank, 8);
        assert!((config.alpha - 16.0).abs() < f32::EPSILON);
        assert!((config.dropout - 0.05).abs() < f32::EPSILON);
        assert_eq!(config.target_modules.len(), 4);
    }

    #[test]
    fn test_lora_config_builder() {
        let config = LoRAConfig::new()
            .with_rank(16)
            .with_alpha(32.0)
            .with_dropout(0.1)
            .with_merge_weights(true);

        assert_eq!(config.rank, 16);
        assert!((config.alpha - 32.0).abs() < f32::EPSILON);
        assert!((config.dropout - 0.1).abs() < f32::EPSILON);
        assert!(config.merge_weights);
    }

    #[test]
    fn test_lora_config_validation() {
        let config = LoRAConfig::new().with_rank(0);
        assert!(config.validate().is_err());

        let config = LoRAConfig::new().with_alpha(-1.0);
        assert!(config.validate().is_err());

        let config = LoRAConfig::new().with_target_modules(vec![]);
        assert!(config.validate().is_err());

        let config = LoRAConfig::new();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_lora_adapter_creation() {
        let config = LoRAConfig::new();
        let adapter = LoRAAdapter::new("test-adapter", "gpt-3.5-turbo", config.clone())
            .with_metadata("version", "1.0")
            .with_metadata("author", "test");

        assert_eq!(adapter.name, "test-adapter");
        assert_eq!(adapter.base_model, "gpt-3.5-turbo");
        assert_eq!(adapter.metadata.get("version"), Some(&"1.0".to_string()));
    }

    #[test]
    fn test_finetuning_example() {
        let example =
            FineTuningExample::new("What is Rust?", "Rust is a systems programming language.")
                .with_metadata("category", "programming");

        assert_eq!(example.prompt, "What is Rust?");
        assert_eq!(
            example.completion,
            "Rust is a systems programming language."
        );
        assert!(example.validate().is_ok());
    }

    #[test]
    fn test_finetuning_example_validation() {
        let example = FineTuningExample::new("", "completion");
        assert!(example.validate().is_err());

        let example = FineTuningExample::new("prompt", "");
        assert!(example.validate().is_err());

        let example = FineTuningExample::new("prompt", "completion");
        assert!(example.validate().is_ok());
    }

    #[test]
    fn test_dataset_builder() {
        let builder = DatasetBuilder::new()
            .add_example(FineTuningExample::new("prompt1", "completion1"))
            .add_example(FineTuningExample::new("prompt2", "completion2"))
            .add_example(FineTuningExample::new("prompt3", "completion3"));

        let dataset = builder.build().unwrap();
        assert_eq!(dataset.len(), 3);
        assert!(!dataset.is_empty());
    }

    #[test]
    fn test_dataset_split() {
        let builder = DatasetBuilder::new()
            .add_examples(vec![
                FineTuningExample::new("p1", "c1"),
                FineTuningExample::new("p2", "c2"),
                FineTuningExample::new("p3", "c3"),
                FineTuningExample::new("p4", "c4"),
                FineTuningExample::new("p5", "c5"),
            ])
            .with_split(0.8, 0.2)
            .unwrap();

        let dataset = builder.build().unwrap();
        assert_eq!(dataset.train_examples.len(), 4);
        assert_eq!(dataset.validation_examples.len(), 1);
    }

    #[test]
    fn test_dataset_stats() {
        let dataset = DatasetBuilder::new()
            .add_example(FineTuningExample::new("short", "short"))
            .add_example(FineTuningExample::new("longer prompt", "longer completion"))
            .build()
            .unwrap();

        let stats = dataset.stats();
        assert_eq!(stats.total_examples, 2);
        assert!(stats.avg_prompt_length > 0.0);
        assert!(stats.avg_completion_length > 0.0);
    }

    #[test]
    fn test_training_metrics() {
        let metrics = TrainingMetrics::new(1, 100)
            .with_train_loss(0.5)
            .with_validation_loss(0.6)
            .with_learning_rate(0.001)
            .with_grad_norm(1.5)
            .with_metric("accuracy", 0.95);

        assert_eq!(metrics.epoch, 1);
        assert_eq!(metrics.step, 100);
        assert!((metrics.train_loss - 0.5).abs() < f32::EPSILON);
        assert_eq!(metrics.validation_loss, Some(0.6));
        assert_eq!(metrics.custom_metrics.get("accuracy"), Some(&0.95));
    }

    #[test]
    fn test_metrics_history() {
        let mut history = MetricsHistory::new();

        history.record(TrainingMetrics::new(1, 10).with_train_loss(1.0));
        history.record(TrainingMetrics::new(1, 20).with_train_loss(0.9));
        history.record(
            TrainingMetrics::new(2, 30)
                .with_train_loss(0.8)
                .with_validation_loss(0.85),
        );

        assert_eq!(history.all().len(), 3);
        assert_eq!(history.latest().unwrap().step, 30);
        assert_eq!(history.epoch_metrics(1).len(), 2);

        let avg_loss = history.avg_loss_for_epoch(1);
        assert!((avg_loss - 0.95).abs() < f32::EPSILON);

        let best_val = history.best_validation_loss();
        assert_eq!(best_val, Some(0.85));
    }

    #[test]
    fn test_evaluation_benchmark() {
        let benchmark = EvaluationBenchmark::new("test-benchmark")
            .add_case(EvaluationCase::new("input1", "output1"))
            .add_case(EvaluationCase::new("input2", "output2"))
            .with_metadata("version", "1.0");

        assert_eq!(benchmark.name, "test-benchmark");
        assert_eq!(benchmark.len(), 2);
        assert!(!benchmark.is_empty());
        assert_eq!(benchmark.metadata.get("version"), Some(&"1.0".to_string()));
    }

    #[test]
    fn test_evaluation_case() {
        let case = EvaluationCase::new("test input", "test output")
            .with_category("testing")
            .with_metadata("priority", "high");

        assert_eq!(case.input, "test input");
        assert_eq!(case.expected, "test output");
        assert_eq!(case.category, Some("testing".to_string()));
        assert_eq!(case.metadata.get("priority"), Some(&"high".to_string()));
    }

    #[test]
    fn test_case_result_exact_match() {
        let result = CaseResult::new(
            "input".to_string(),
            "expected output".to_string(),
            "expected output".to_string(),
            100.0,
        );

        assert!(result.exact_match);
        assert!((result.similarity_score - 1.0).abs() < f32::EPSILON);
        assert_eq!(result.latency_ms, 100.0);
    }

    #[test]
    fn test_case_result_partial_match() {
        let result = CaseResult::new(
            "input".to_string(),
            "the quick brown fox".to_string(),
            "the quick red fox".to_string(),
            100.0,
        );

        assert!(!result.exact_match);
        assert!(result.similarity_score > 0.5);
        assert!(result.similarity_score < 1.0);
    }

    #[test]
    fn test_case_result_no_match() {
        let result = CaseResult::new(
            "input".to_string(),
            "completely different".to_string(),
            "totally unrelated words".to_string(),
            100.0,
        );

        assert!(!result.exact_match);
        assert!(result.similarity_score < 0.5);
    }

    #[test]
    fn test_evaluation_result() {
        let result = EvaluationResult::new("model-v1", "benchmark-1")
            .add_result(CaseResult::new(
                "i1".to_string(),
                "o1".to_string(),
                "o1".to_string(),
                50.0,
            ))
            .add_result(CaseResult::new(
                "i2".to_string(),
                "o2".to_string(),
                "o2".to_string(),
                60.0,
            ))
            .add_result(CaseResult::new(
                "i3".to_string(),
                "o3".to_string(),
                "different".to_string(),
                70.0,
            ))
            .compute_metrics();

        assert_eq!(result.model_id, "model-v1");
        assert_eq!(result.benchmark_name, "benchmark-1");
        assert_eq!(result.metrics.total_cases, 3);
        assert_eq!(result.metrics.passed_cases, 2);
        assert!((result.metrics.accuracy - 0.666667).abs() < 0.001);
        assert!((result.metrics.avg_latency_ms - 60.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_ab_test() {
        let mut ab_test = ABTest::new("test", "model-a", "model-b").add_test_cases(vec![
            EvaluationCase::new("input1", "output1"),
            EvaluationCase::new("input2", "output2"),
        ]);

        ab_test.add_result_a(CaseResult::new(
            "i1".to_string(),
            "o1".to_string(),
            "o1".to_string(),
            100.0,
        ));
        ab_test.add_result_a(CaseResult::new(
            "i2".to_string(),
            "o2".to_string(),
            "wrong".to_string(),
            100.0,
        ));

        ab_test.add_result_b(CaseResult::new(
            "i1".to_string(),
            "o1".to_string(),
            "o1".to_string(),
            90.0,
        ));
        ab_test.add_result_b(CaseResult::new(
            "i2".to_string(),
            "o2".to_string(),
            "o2".to_string(),
            90.0,
        ));

        assert_eq!(ab_test.results_a.len(), 2);
        assert_eq!(ab_test.results_b.len(), 2);
    }

    #[test]
    fn test_ab_test_analysis() {
        let mut ab_test = ABTest::new("comparison", "baseline", "finetuned");

        ab_test.add_result_a(CaseResult::new(
            "i1".to_string(),
            "o1".to_string(),
            "o1".to_string(),
            100.0,
        ));
        ab_test.add_result_a(CaseResult::new(
            "i2".to_string(),
            "o2".to_string(),
            "wrong".to_string(),
            100.0,
        ));

        ab_test.add_result_b(CaseResult::new(
            "i1".to_string(),
            "o1".to_string(),
            "o1".to_string(),
            90.0,
        ));
        ab_test.add_result_b(CaseResult::new(
            "i2".to_string(),
            "o2".to_string(),
            "o2".to_string(),
            90.0,
        ));

        let analysis = ab_test.analyze();

        assert_eq!(analysis.test_name, "comparison");
        assert_eq!(analysis.model_a, "baseline");
        assert_eq!(analysis.model_b, "finetuned");
        assert_eq!(analysis.winner, Some("finetuned".to_string()));
        assert!(analysis.improvement_percent > 0.0);
    }

    #[test]
    fn test_ab_test_analysis_tie() {
        let mut ab_test = ABTest::new("tie-test", "model-a", "model-b");

        ab_test.add_result_a(CaseResult::new(
            "i1".to_string(),
            "o1".to_string(),
            "o1".to_string(),
            100.0,
        ));
        ab_test.add_result_b(CaseResult::new(
            "i1".to_string(),
            "o1".to_string(),
            "o1".to_string(),
            100.0,
        ));

        let analysis = ab_test.analyze();
        assert_eq!(analysis.winner, None);
        assert!((analysis.improvement_percent - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_ab_test_analysis_summary() {
        let mut ab_test = ABTest::new("summary-test", "old-model", "new-model");

        ab_test.add_result_a(CaseResult::new(
            "i1".to_string(),
            "expected".to_string(),
            "expected".to_string(),
            100.0,
        ));
        ab_test.add_result_b(CaseResult::new(
            "i1".to_string(),
            "expected".to_string(),
            "expected".to_string(),
            50.0,
        ));

        let analysis = ab_test.analyze();
        let summary = analysis.summary();

        assert!(summary.contains("summary-test"));
        assert!(summary.contains("old-model"));
        assert!(summary.contains("new-model"));
    }

    #[test]
    fn test_evaluation_metrics_defaults() {
        let metrics = EvaluationMetrics::default();
        assert_eq!(metrics.accuracy, 0.0);
        assert_eq!(metrics.avg_similarity, 0.0);
        assert_eq!(metrics.avg_latency_ms, 0.0);
        assert_eq!(metrics.total_cases, 0);
        assert_eq!(metrics.passed_cases, 0);
    }
}
