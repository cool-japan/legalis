//! Neural legal entailment for automated reasoning.
//!
//! This module provides neural network-based legal entailment checking, determining
//! whether one legal statement logically entails another. This enables automated
//! legal reasoning, statute implication detection, and fact-to-conclusion inference.
//!
//! ## Features
//!
//! - **Neural Backend Abstraction**: Trait-based design for any ML framework
//! - **Entailment Classification**: Entails, Contradicts, or Neutral
//! - **Confidence Scoring**: Probabilistic outputs for uncertainty quantification
//! - **Batch Processing**: Efficient batch entailment checking
//! - **Embedding Support**: Vector embeddings for legal text
//!
//! ## Example
//!
//! ```
//! use legalis_core::neural_entailment::{EntailmentChecker, EntailmentLabel};
//!
//! let checker = EntailmentChecker::new();
//!
//! // Check if premise entails hypothesis
//! let premise = "All persons over 18 may vote";
//! let hypothesis = "A 25-year-old may vote";
//!
//! // In a real implementation with a trained model, this would return Entails
//! // For now, without a backend, it returns an error
//! assert!(checker.check_entailment(premise, hypothesis).is_err());
//! ```

use std::collections::HashMap;

/// Neural entailment backend trait.
///
/// Implementors can use PyTorch, TensorFlow, ONNX, or other ML frameworks.
pub trait NeuralBackend: Send + Sync {
    /// Predicts entailment between premise and hypothesis.
    fn predict_entailment(
        &self,
        premise: &str,
        hypothesis: &str,
    ) -> Result<EntailmentPrediction, NeuralError>;

    /// Batch predicts entailment for multiple premise-hypothesis pairs.
    fn batch_predict(
        &self,
        pairs: &[(String, String)],
    ) -> Result<Vec<EntailmentPrediction>, NeuralError>;

    /// Generates embeddings for legal text.
    fn embed_text(&self, text: &str) -> Result<Vec<f32>, NeuralError>;

    /// Returns the name of the neural backend.
    fn backend_name(&self) -> &str;

    /// Returns whether the backend is loaded and ready.
    fn is_ready(&self) -> bool;
}

/// Entailment prediction result.
#[derive(Debug, Clone, PartialEq)]
pub struct EntailmentPrediction {
    /// The predicted entailment label
    pub label: EntailmentLabel,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
    /// Probability distribution over labels
    pub probabilities: HashMap<EntailmentLabel, f64>,
}

impl EntailmentPrediction {
    /// Creates a new entailment prediction.
    pub fn new(label: EntailmentLabel, confidence: f64) -> Self {
        let mut probabilities = HashMap::new();
        probabilities.insert(label, confidence);

        Self {
            label,
            confidence,
            probabilities,
        }
    }

    /// Returns whether the prediction has high confidence (>= 0.8).
    pub fn is_high_confidence(&self) -> bool {
        self.confidence >= 0.8
    }

    /// Returns the probability for a specific label.
    pub fn probability(&self, label: &EntailmentLabel) -> f64 {
        self.probabilities.get(label).copied().unwrap_or(0.0)
    }
}

/// Entailment labels following NLI (Natural Language Inference) standard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntailmentLabel {
    /// Premise entails the hypothesis
    Entails,
    /// Premise contradicts the hypothesis
    Contradicts,
    /// Neither entailment nor contradiction
    Neutral,
}

impl std::fmt::Display for EntailmentLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntailmentLabel::Entails => write!(f, "entails"),
            EntailmentLabel::Contradicts => write!(f, "contradicts"),
            EntailmentLabel::Neutral => write!(f, "neutral"),
        }
    }
}

/// Errors that can occur in neural entailment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NeuralError {
    /// Neural backend not initialized
    BackendNotReady,
    /// Model loading failed
    ModelLoadError(String),
    /// Inference failed
    InferenceError(String),
    /// Input validation failed
    InvalidInput(String),
    /// Embedding generation failed
    EmbeddingError(String),
}

impl std::fmt::Display for NeuralError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NeuralError::BackendNotReady => write!(f, "Neural backend not ready"),
            NeuralError::ModelLoadError(msg) => write!(f, "Model load error: {}", msg),
            NeuralError::InferenceError(msg) => write!(f, "Inference error: {}", msg),
            NeuralError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            NeuralError::EmbeddingError(msg) => write!(f, "Embedding error: {}", msg),
        }
    }
}

impl std::error::Error for NeuralError {}

/// Entailment checker using neural models.
///
/// # Example
///
/// ```
/// use legalis_core::neural_entailment::EntailmentChecker;
///
/// let checker = EntailmentChecker::new();
/// assert!(!checker.is_ready()); // No backend loaded
/// assert_eq!(checker.cache_size(), 0);
/// ```
pub struct EntailmentChecker {
    /// Optional neural backend
    #[allow(dead_code)]
    backend: Option<Box<dyn NeuralBackend>>,
    /// Prediction cache
    cache: HashMap<(String, String), EntailmentPrediction>,
    /// Statistics
    stats: EntailmentStats,
}

impl EntailmentChecker {
    /// Creates a new entailment checker.
    pub fn new() -> Self {
        Self {
            backend: None,
            cache: HashMap::new(),
            stats: EntailmentStats::new(),
        }
    }

    /// Sets the neural backend.
    #[allow(dead_code)]
    pub fn with_backend(mut self, backend: Box<dyn NeuralBackend>) -> Self {
        self.backend = Some(backend);
        self
    }

    /// Checks if premise entails hypothesis.
    pub fn check_entailment(
        &self,
        premise: &str,
        hypothesis: &str,
    ) -> Result<EntailmentPrediction, NeuralError> {
        // Check cache first
        let key = (premise.to_string(), hypothesis.to_string());
        if let Some(cached) = self.cache.get(&key) {
            return Ok(cached.clone());
        }

        // In a real implementation, would call backend.predict_entailment()
        Err(NeuralError::BackendNotReady)
    }

    /// Returns whether the checker is ready (backend loaded).
    pub fn is_ready(&self) -> bool {
        self.backend.as_ref().is_some_and(|b| b.is_ready())
    }

    /// Returns the number of cached predictions.
    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }

    /// Clears the prediction cache.
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// Returns entailment statistics.
    pub fn stats(&self) -> &EntailmentStats {
        &self.stats
    }
}

impl Default for EntailmentChecker {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics for neural entailment checking.
#[derive(Debug, Clone, Default)]
pub struct EntailmentStats {
    /// Total number of entailment checks
    pub total_checks: u64,
    /// Number of entailment predictions
    pub entails_count: u64,
    /// Number of contradiction predictions
    pub contradicts_count: u64,
    /// Number of neutral predictions
    pub neutral_count: u64,
    /// Number of cache hits
    pub cache_hits: u64,
    /// Number of cache misses
    pub cache_misses: u64,
}

impl EntailmentStats {
    /// Creates new entailment statistics.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the cache hit rate (0.0 to 1.0).
    pub fn cache_hit_rate(&self) -> f64 {
        let total = self.cache_hits + self.cache_misses;
        if total == 0 {
            0.0
        } else {
            self.cache_hits as f64 / total as f64
        }
    }

    /// Returns the percentage of entailment predictions.
    pub fn entailment_rate(&self) -> f64 {
        if self.total_checks == 0 {
            0.0
        } else {
            self.entails_count as f64 / self.total_checks as f64
        }
    }

    /// Returns the percentage of contradiction predictions.
    pub fn contradiction_rate(&self) -> f64 {
        if self.total_checks == 0 {
            0.0
        } else {
            self.contradicts_count as f64 / self.total_checks as f64
        }
    }
}

/// Training example for fine-tuning entailment models.
#[derive(Debug, Clone)]
pub struct EntailmentExample {
    /// Premise text
    pub premise: String,
    /// Hypothesis text
    pub hypothesis: String,
    /// True label
    pub label: EntailmentLabel,
    /// Optional metadata
    pub metadata: HashMap<String, String>,
}

impl EntailmentExample {
    /// Creates a new training example.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::neural_entailment::{EntailmentExample, EntailmentLabel};
    ///
    /// let example = EntailmentExample::new(
    ///     "All citizens may vote".to_string(),
    ///     "John is a citizen, so he may vote".to_string(),
    ///     EntailmentLabel::Entails,
    /// );
    ///
    /// assert_eq!(example.label, EntailmentLabel::Entails);
    /// ```
    pub fn new(premise: String, hypothesis: String, label: EntailmentLabel) -> Self {
        Self {
            premise,
            hypothesis,
            label,
            metadata: HashMap::new(),
        }
    }

    /// Adds metadata to the example.
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entailment_prediction() {
        let pred = EntailmentPrediction::new(EntailmentLabel::Entails, 0.9);
        assert_eq!(pred.label, EntailmentLabel::Entails);
        assert_eq!(pred.confidence, 0.9);
        assert!(pred.is_high_confidence());
    }

    #[test]
    fn test_entailment_label_display() {
        assert_eq!(format!("{}", EntailmentLabel::Entails), "entails");
        assert_eq!(format!("{}", EntailmentLabel::Contradicts), "contradicts");
        assert_eq!(format!("{}", EntailmentLabel::Neutral), "neutral");
    }

    #[test]
    fn test_neural_error_display() {
        let err = NeuralError::BackendNotReady;
        assert_eq!(format!("{}", err), "Neural backend not ready");

        let err = NeuralError::ModelLoadError("Failed to load".to_string());
        assert_eq!(format!("{}", err), "Model load error: Failed to load");
    }

    #[test]
    fn test_entailment_checker() {
        let checker = EntailmentChecker::new();
        assert!(!checker.is_ready());
        assert_eq!(checker.cache_size(), 0);
    }

    #[test]
    fn test_entailment_stats() {
        let mut stats = EntailmentStats::new();
        stats.total_checks = 100;
        stats.entails_count = 40;
        stats.contradicts_count = 30;
        stats.neutral_count = 30;
        stats.cache_hits = 80;
        stats.cache_misses = 20;

        assert_eq!(stats.cache_hit_rate(), 0.8);
        assert_eq!(stats.entailment_rate(), 0.4);
        assert_eq!(stats.contradiction_rate(), 0.3);
    }

    #[test]
    fn test_entailment_example() {
        let example = EntailmentExample::new(
            "All dogs are animals".to_string(),
            "Fido is a dog, so Fido is an animal".to_string(),
            EntailmentLabel::Entails,
        );

        assert_eq!(example.label, EntailmentLabel::Entails);
        assert!(example.premise.contains("dogs"));
        assert!(example.hypothesis.contains("Fido"));
    }

    #[test]
    fn test_prediction_probability() {
        let mut pred = EntailmentPrediction::new(EntailmentLabel::Entails, 0.85);
        pred.probabilities.insert(EntailmentLabel::Neutral, 0.10);
        pred.probabilities
            .insert(EntailmentLabel::Contradicts, 0.05);

        assert_eq!(pred.probability(&EntailmentLabel::Entails), 0.85);
        assert_eq!(pred.probability(&EntailmentLabel::Neutral), 0.10);
    }
}
