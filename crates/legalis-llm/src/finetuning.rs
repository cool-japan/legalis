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

/// Legal domain fine-tuning utilities.
pub mod legal_finetuning {
    use super::*;

    /// Legal task types for fine-tuning.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum LegalTaskType {
        /// Statute interpretation.
        StatuteInterpretation,
        /// Contract analysis.
        ContractAnalysis,
        /// Case law summarization.
        CaseLawSummarization,
        /// Legal argument generation.
        LegalArgumentGeneration,
        /// Compliance checking.
        ComplianceChecking,
        /// Clause extraction.
        ClauseExtraction,
        /// Citation extraction and formatting.
        CitationExtraction,
        /// Legal question answering.
        QuestionAnswering,
    }

    /// Legal fine-tuning pipeline configuration.
    #[derive(Debug, Clone)]
    pub struct LegalFineTuningConfig {
        /// Task type for this pipeline.
        pub task_type: LegalTaskType,
        /// Jurisdiction (e.g., "US", "UK", "EU").
        pub jurisdiction: Option<String>,
        /// LoRA configuration for efficient training.
        pub lora_config: Option<LoRAConfig>,
        /// Whether to apply constitutional AI alignment.
        pub constitutional_ai: bool,
        /// Whether to generate synthetic training data.
        pub synthetic_data: bool,
        /// Number of synthetic examples to generate.
        pub synthetic_count: usize,
        /// Training hyperparameters.
        pub hyperparameters: TrainingHyperparameters,
    }

    impl Default for LegalFineTuningConfig {
        fn default() -> Self {
            Self {
                task_type: LegalTaskType::StatuteInterpretation,
                jurisdiction: None,
                lora_config: Some(LoRAConfig::default()),
                constitutional_ai: true,
                synthetic_data: false,
                synthetic_count: 1000,
                hyperparameters: TrainingHyperparameters::default(),
            }
        }
    }

    impl LegalFineTuningConfig {
        /// Creates a new legal fine-tuning config.
        pub fn new(task_type: LegalTaskType) -> Self {
            Self {
                task_type,
                ..Default::default()
            }
        }

        /// Sets the jurisdiction.
        pub fn with_jurisdiction(mut self, jurisdiction: impl Into<String>) -> Self {
            self.jurisdiction = Some(jurisdiction.into());
            self
        }

        /// Sets the LoRA configuration.
        pub fn with_lora(mut self, config: LoRAConfig) -> Self {
            self.lora_config = Some(config);
            self
        }

        /// Enables or disables constitutional AI alignment.
        pub fn with_constitutional_ai(mut self, enable: bool) -> Self {
            self.constitutional_ai = enable;
            self
        }

        /// Enables synthetic data generation.
        pub fn with_synthetic_data(mut self, count: usize) -> Self {
            self.synthetic_data = true;
            self.synthetic_count = count;
            self
        }
    }

    /// Training hyperparameters.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct TrainingHyperparameters {
        /// Learning rate.
        pub learning_rate: f32,
        /// Number of epochs.
        pub epochs: usize,
        /// Batch size.
        pub batch_size: usize,
        /// Warmup steps.
        pub warmup_steps: usize,
        /// Weight decay.
        pub weight_decay: f32,
        /// Gradient accumulation steps.
        pub gradient_accumulation_steps: usize,
        /// Maximum gradient norm for clipping.
        pub max_grad_norm: f32,
    }

    impl Default for TrainingHyperparameters {
        fn default() -> Self {
            Self {
                learning_rate: 5e-5,
                epochs: 3,
                batch_size: 8,
                warmup_steps: 100,
                weight_decay: 0.01,
                gradient_accumulation_steps: 1,
                max_grad_norm: 1.0,
            }
        }
    }

    /// Legal domain fine-tuning pipeline.
    pub struct LegalFineTuningPipeline {
        config: LegalFineTuningConfig,
        base_dataset: Vec<FineTuningExample>,
        synthetic_dataset: Vec<FineTuningExample>,
    }

    impl LegalFineTuningPipeline {
        /// Creates a new legal fine-tuning pipeline.
        pub fn new(config: LegalFineTuningConfig) -> Self {
            Self {
                config,
                base_dataset: Vec::new(),
                synthetic_dataset: Vec::new(),
            }
        }

        /// Adds examples to the base dataset.
        pub fn add_examples(&mut self, examples: Vec<FineTuningExample>) {
            self.base_dataset.extend(examples);
        }

        /// Generates synthetic training data based on the task type.
        pub fn generate_synthetic_data(&mut self) -> Result<()> {
            if !self.config.synthetic_data {
                return Ok(());
            }

            let generator = SyntheticDataGenerator::new(self.config.task_type);
            self.synthetic_dataset = generator.generate(self.config.synthetic_count)?;

            Ok(())
        }

        /// Applies constitutional AI alignment to the dataset.
        pub fn apply_constitutional_alignment(&mut self) -> Result<()> {
            if !self.config.constitutional_ai {
                return Ok(());
            }

            let aligner = ConstitutionalAIAligner::new();

            for example in self.base_dataset.iter_mut() {
                aligner.align_example(example)?;
            }

            for example in self.synthetic_dataset.iter_mut() {
                aligner.align_example(example)?;
            }

            Ok(())
        }

        /// Builds the complete training dataset.
        pub fn build_dataset(self) -> Result<FineTuningDataset> {
            let mut all_examples = self.base_dataset;
            all_examples.extend(self.synthetic_dataset);

            DatasetBuilder::new().add_examples(all_examples).build()
        }

        /// Returns the total number of examples (base + synthetic).
        pub fn example_count(&self) -> usize {
            self.base_dataset.len() + self.synthetic_dataset.len()
        }
    }

    /// Synthetic data generator for legal tasks.
    pub struct SyntheticDataGenerator {
        task_type: LegalTaskType,
    }

    impl SyntheticDataGenerator {
        /// Creates a new synthetic data generator.
        pub fn new(task_type: LegalTaskType) -> Self {
            Self { task_type }
        }

        /// Generates synthetic training examples.
        pub fn generate(&self, count: usize) -> Result<Vec<FineTuningExample>> {
            let mut examples = Vec::with_capacity(count);

            for i in 0..count {
                let example = match self.task_type {
                    LegalTaskType::StatuteInterpretation => self.generate_statute_interpretation(i),
                    LegalTaskType::ContractAnalysis => self.generate_contract_analysis(i),
                    LegalTaskType::CaseLawSummarization => self.generate_case_law_summary(i),
                    LegalTaskType::LegalArgumentGeneration => self.generate_legal_argument(i),
                    LegalTaskType::ComplianceChecking => self.generate_compliance_check(i),
                    LegalTaskType::ClauseExtraction => self.generate_clause_extraction(i),
                    LegalTaskType::CitationExtraction => self.generate_citation_extraction(i),
                    LegalTaskType::QuestionAnswering => self.generate_qa(i),
                }?;

                examples.push(example);
            }

            Ok(examples)
        }

        fn generate_statute_interpretation(&self, _index: usize) -> Result<FineTuningExample> {
            Ok(FineTuningExample::new(
                "Interpret this statute: A person who operates a motor vehicle while intoxicated commits a misdemeanor.",
                "This statute creates a criminal offense for driving under the influence (DUI). Key elements: (1) Operation of a motor vehicle, (2) While intoxicated. The offense is classified as a misdemeanor, indicating a lesser criminal penalty than a felony."
            ).with_metadata("task", "statute_interpretation"))
        }

        fn generate_contract_analysis(&self, _index: usize) -> Result<FineTuningExample> {
            Ok(FineTuningExample::new(
                "Analyze this contract clause: Either party may terminate this agreement upon 30 days written notice.",
                "This is a termination clause granting mutual termination rights. Both parties have equal ability to end the agreement. Requirements: (1) 30 days advance notice, (2) Notice must be in writing. This provides flexibility but requires planning for transition."
            ).with_metadata("task", "contract_analysis"))
        }

        fn generate_case_law_summary(&self, _index: usize) -> Result<FineTuningExample> {
            Ok(FineTuningExample::new(
                "Summarize this case: In Smith v. Jones (2020), the court held that verbal agreements for real estate transactions are unenforceable under the statute of frauds.",
                "Case: Smith v. Jones (2020)\nHolding: Verbal real estate agreements are unenforceable.\nReasoning: Statute of frauds requires real estate contracts to be in writing.\nImplication: Real estate transactions must be documented in writing to be legally binding."
            ).with_metadata("task", "case_law_summarization"))
        }

        fn generate_legal_argument(&self, _index: usize) -> Result<FineTuningExample> {
            Ok(FineTuningExample::new(
                "Generate an argument for: The defendant's Fourth Amendment rights were violated by the warrantless search.",
                "The Fourth Amendment protects against unreasonable searches and seizures. A warrantless search is presumptively unreasonable unless it falls within a recognized exception. Here, no exception applies: (1) No exigent circumstances existed, (2) No consent was given, (3) The search exceeded the scope of any applicable exception. Therefore, the evidence obtained should be suppressed under the exclusionary rule as fruit of an unconstitutional search."
            ).with_metadata("task", "legal_argument"))
        }

        fn generate_compliance_check(&self, _index: usize) -> Result<FineTuningExample> {
            Ok(FineTuningExample::new(
                "Check compliance: Company stores customer data for 10 years. Regulation requires deletion after 5 years.",
                "COMPLIANCE VIOLATION: Data retention period exceeds regulatory requirements.\nRequired: 5 years maximum retention\nActual: 10 years retention\nRecommendation: Implement automated data deletion after 5 years to ensure compliance. Update data retention policy and notify data protection officer."
            ).with_metadata("task", "compliance_checking"))
        }

        fn generate_clause_extraction(&self, _index: usize) -> Result<FineTuningExample> {
            Ok(FineTuningExample::new(
                "Extract confidentiality clauses from: ...The Parties agree to keep all proprietary information confidential for 3 years after termination. Each party may use the information only for purposes of this agreement...",
                "Confidentiality Clause:\n- Obligation: Keep proprietary information confidential\n- Duration: 3 years after termination\n- Permitted Use: Only for purposes of this agreement\n- Scope: All proprietary information"
            ).with_metadata("task", "clause_extraction"))
        }

        fn generate_citation_extraction(&self, _index: usize) -> Result<FineTuningExample> {
            Ok(FineTuningExample::new(
                "Extract citations from: The court in Brown v. Board of Education, 347 U.S. 483 (1954) held that separate educational facilities are inherently unequal.",
                "Citations:\n1. Brown v. Board of Education, 347 U.S. 483 (1954)\n   - Court: U.S. Supreme Court\n   - Year: 1954\n   - Citation: 347 U.S. 483\n   - Holding: Separate educational facilities are inherently unequal"
            ).with_metadata("task", "citation_extraction"))
        }

        fn generate_qa(&self, _index: usize) -> Result<FineTuningExample> {
            Ok(FineTuningExample::new(
                "Question: What is the statute of limitations for breach of contract in most jurisdictions?\nContext: Contract law principles",
                "Answer: The statute of limitations for breach of contract varies by jurisdiction but typically ranges from 3 to 6 years from the date of breach. Some jurisdictions distinguish between written contracts (longer period, often 6 years) and oral contracts (shorter period, often 3 years). It's essential to check the specific statute in the relevant jurisdiction as timely filing is crucial to preserve the right to sue."
            ).with_metadata("task", "question_answering"))
        }
    }

    /// Constitutional AI alignment for legal outputs.
    pub struct ConstitutionalAIAligner {
        principles: Vec<ConstitutionalPrinciple>,
    }

    impl ConstitutionalAIAligner {
        /// Creates a new constitutional AI aligner with default legal principles.
        pub fn new() -> Self {
            Self {
                principles: Self::default_legal_principles(),
            }
        }

        /// Default constitutional principles for legal AI.
        fn default_legal_principles() -> Vec<ConstitutionalPrinciple> {
            vec![
                ConstitutionalPrinciple {
                    name: "Accuracy".to_string(),
                    description: "Ensure legal information is accurate and up-to-date".to_string(),
                    critique_prompt: "Is this legal information accurate? Does it cite reliable sources?".to_string(),
                },
                ConstitutionalPrinciple {
                    name: "Jurisdiction Awareness".to_string(),
                    description: "Clearly state which jurisdiction the advice applies to".to_string(),
                    critique_prompt: "Does this specify the relevant jurisdiction? Is it clear where this law applies?".to_string(),
                },
                ConstitutionalPrinciple {
                    name: "Disclaimer".to_string(),
                    description: "Include appropriate disclaimers about legal advice".to_string(),
                    critique_prompt: "Does this include a disclaimer that it's not legal advice and users should consult an attorney?".to_string(),
                },
                ConstitutionalPrinciple {
                    name: "Impartiality".to_string(),
                    description: "Present legal information objectively without bias".to_string(),
                    critique_prompt: "Is this information presented objectively? Does it avoid bias or advocacy for a particular position?".to_string(),
                },
                ConstitutionalPrinciple {
                    name: "Clarity".to_string(),
                    description: "Explain legal concepts in clear, accessible language".to_string(),
                    critique_prompt: "Is this explanation clear and understandable? Does it avoid unnecessary jargon?".to_string(),
                },
                ConstitutionalPrinciple {
                    name: "Completeness".to_string(),
                    description: "Provide comprehensive information including exceptions and limitations".to_string(),
                    critique_prompt: "Does this cover important exceptions or limitations? Is any critical information missing?".to_string(),
                },
            ]
        }

        /// Aligns a fine-tuning example with constitutional principles.
        pub fn align_example(&self, example: &mut FineTuningExample) -> Result<()> {
            // Add disclaimer if not present
            if !example.completion.contains("disclaimer")
                && !example.completion.contains("not legal advice")
            {
                example.completion.push_str("\n\nNote: This is general legal information, not legal advice. Consult a qualified attorney for advice specific to your situation.");
            }

            // Add constitutional metadata
            example
                .metadata
                .insert("constitutional_aligned".to_string(), "true".to_string());

            Ok(())
        }

        /// Adds a custom principle.
        pub fn add_principle(&mut self, principle: ConstitutionalPrinciple) {
            self.principles.push(principle);
        }

        /// Returns all principles.
        pub fn principles(&self) -> &[ConstitutionalPrinciple] {
            &self.principles
        }
    }

    impl Default for ConstitutionalAIAligner {
        fn default() -> Self {
            Self::new()
        }
    }

    /// A constitutional principle for AI alignment.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ConstitutionalPrinciple {
        /// Principle name.
        pub name: String,
        /// Description of the principle.
        pub description: String,
        /// Critique prompt to evaluate adherence.
        pub critique_prompt: String,
    }

    /// Legal task evaluation benchmark builder.
    pub struct LegalBenchmarkBuilder {
        task_type: LegalTaskType,
        cases: Vec<EvaluationCase>,
    }

    impl LegalBenchmarkBuilder {
        /// Creates a new legal benchmark builder.
        pub fn new(task_type: LegalTaskType) -> Self {
            Self {
                task_type,
                cases: Vec::new(),
            }
        }

        /// Adds predefined legal test cases for the task type.
        pub fn with_standard_cases(mut self) -> Self {
            let standard_cases = match self.task_type {
                LegalTaskType::StatuteInterpretation => self.statute_interpretation_cases(),
                LegalTaskType::ContractAnalysis => self.contract_analysis_cases(),
                LegalTaskType::CaseLawSummarization => self.case_law_cases(),
                LegalTaskType::LegalArgumentGeneration => self.legal_argument_cases(),
                LegalTaskType::ComplianceChecking => self.compliance_cases(),
                LegalTaskType::ClauseExtraction => self.clause_extraction_cases(),
                LegalTaskType::CitationExtraction => self.citation_cases(),
                LegalTaskType::QuestionAnswering => self.qa_cases(),
            };

            self.cases.extend(standard_cases);
            self
        }

        /// Builds the benchmark.
        pub fn build(self) -> EvaluationBenchmark {
            let name = format!("legal_{:?}_benchmark", self.task_type);
            EvaluationBenchmark::new(name)
                .add_cases(self.cases)
                .with_metadata("task_type", format!("{:?}", self.task_type))
                .with_metadata("domain", "legal")
        }

        fn statute_interpretation_cases(&self) -> Vec<EvaluationCase> {
            vec![
                EvaluationCase::new(
                    "Interpret: No vehicle shall enter the park.",
                    "This statute prohibits all vehicles from entering the park. 'Vehicle' likely includes cars, motorcycles, bicycles. 'Park' refers to the designated area. Questions may arise about wheelchairs, skateboards, or emergency vehicles."
                ).with_category("statute_interpretation"),
            ]
        }

        fn contract_analysis_cases(&self) -> Vec<EvaluationCase> {
            vec![
                EvaluationCase::new(
                    "Analyze: This agreement shall be governed by the laws of California.",
                    "Choice of law clause specifying California law governs the contract. This determines which state's laws apply to interpretation and disputes, regardless of where parties are located or where performance occurs."
                ).with_category("contract_analysis"),
            ]
        }

        fn case_law_cases(&self) -> Vec<EvaluationCase> {
            vec![
                EvaluationCase::new(
                    "Summarize: Miranda v. Arizona established that suspects must be informed of their rights before custodial interrogation.",
                    "Case: Miranda v. Arizona\nHolding: Suspects must receive Miranda warnings before custodial interrogation.\nRights: Right to remain silent, right to attorney.\nConsequence: Statements obtained without Miranda warnings are inadmissible."
                ).with_category("case_law"),
            ]
        }

        fn legal_argument_cases(&self) -> Vec<EvaluationCase> {
            vec![
                EvaluationCase::new(
                    "Argue for dismissal based on lack of personal jurisdiction.",
                    "The court lacks personal jurisdiction over defendant because: (1) Defendant has no minimum contacts with the forum state, (2) Defendant did not purposefully avail itself of forum benefits, (3) Exercise of jurisdiction would not comport with fair play and substantial justice per International Shoe. Therefore, the case should be dismissed."
                ).with_category("legal_argument"),
            ]
        }

        fn compliance_cases(&self) -> Vec<EvaluationCase> {
            vec![
                EvaluationCase::new(
                    "Check GDPR compliance: Website collects email addresses without consent checkbox.",
                    "VIOLATION: GDPR requires explicit consent for data collection. Missing: (1) Clear consent mechanism, (2) Opt-in checkbox, (3) Privacy policy link. Required action: Add consent checkbox and privacy policy before collecting emails."
                ).with_category("compliance"),
            ]
        }

        fn clause_extraction_cases(&self) -> Vec<EvaluationCase> {
            vec![
                EvaluationCase::new(
                    "Extract indemnification clause: Contractor shall indemnify Client against all claims arising from Contractor's negligence.",
                    "Indemnification Clause:\n- Indemnitor: Contractor\n- Indemnitee: Client\n- Scope: All claims arising from Contractor's negligence\n- Type: One-way indemnification"
                ).with_category("clause_extraction"),
            ]
        }

        fn citation_cases(&self) -> Vec<EvaluationCase> {
            vec![
                EvaluationCase::new(
                    "Extract: Marbury v. Madison, 5 U.S. 137 (1803)",
                    "Citation: Marbury v. Madison, 5 U.S. 137 (1803)\nCourt: U.S. Supreme Court\nYear: 1803\nVolume: 5\nReporter: U.S.\nPage: 137"
                ).with_category("citation"),
            ]
        }

        fn qa_cases(&self) -> Vec<EvaluationCase> {
            vec![
                EvaluationCase::new(
                    "What is consideration in contract law?",
                    "Consideration is something of value exchanged between parties to a contract. It can be a promise, performance, or forbearance. Both parties must provide consideration for a contract to be enforceable (the bargained-for exchange). Without consideration, an agreement is generally not legally binding."
                ).with_category("qa"),
            ]
        }
    }
}
