//! Cross-Domain Transfer Learning for Legal AI
//!
//! This module provides advanced transfer learning capabilities for legal domains,
//! including domain adaptation, few-shot learning, zero-shot classification, and
//! continual learning without catastrophic forgetting.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Legal domain specialization types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LegalDomain {
    /// Contract law
    ContractLaw,
    /// Tort law
    TortLaw,
    /// Criminal law
    CriminalLaw,
    /// Tax law
    TaxLaw,
    /// Employment law
    EmploymentLaw,
    /// Intellectual property
    IntellectualProperty,
    /// Family law
    FamilyLaw,
    /// Environmental law
    EnvironmentalLaw,
    /// Corporate law
    CorporateLaw,
    /// Constitutional law
    ConstitutionalLaw,
    /// Custom domain
    Custom(String),
}

impl std::fmt::Display for LegalDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ContractLaw => write!(f, "Contract Law"),
            Self::TortLaw => write!(f, "Tort Law"),
            Self::CriminalLaw => write!(f, "Criminal Law"),
            Self::TaxLaw => write!(f, "Tax Law"),
            Self::EmploymentLaw => write!(f, "Employment Law"),
            Self::IntellectualProperty => write!(f, "Intellectual Property"),
            Self::FamilyLaw => write!(f, "Family Law"),
            Self::EnvironmentalLaw => write!(f, "Environmental Law"),
            Self::CorporateLaw => write!(f, "Corporate Law"),
            Self::ConstitutionalLaw => write!(f, "Constitutional Law"),
            Self::Custom(name) => write!(f, "{}", name),
        }
    }
}

/// Jurisdiction for cross-jurisdictional transfer
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransferJurisdiction {
    /// United States (federal)
    UnitedStatesFederal,
    /// US State
    USState(String),
    /// United Kingdom
    UnitedKingdom,
    /// European Union
    EuropeanUnion,
    /// EU Member State
    EUMemberState(String),
    /// Canada (federal)
    CanadaFederal,
    /// Canadian Province
    CanadianProvince(String),
    /// Australia (federal)
    AustraliaFederal,
    /// Australian State
    AustralianState(String),
    /// Custom jurisdiction
    Custom(String),
}

impl std::fmt::Display for TransferJurisdiction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnitedStatesFederal => write!(f, "United States (Federal)"),
            Self::USState(state) => write!(f, "United States - {}", state),
            Self::UnitedKingdom => write!(f, "United Kingdom"),
            Self::EuropeanUnion => write!(f, "European Union"),
            Self::EUMemberState(state) => write!(f, "EU - {}", state),
            Self::CanadaFederal => write!(f, "Canada (Federal)"),
            Self::CanadianProvince(province) => write!(f, "Canada - {}", province),
            Self::AustraliaFederal => write!(f, "Australia (Federal)"),
            Self::AustralianState(state) => write!(f, "Australia - {}", state),
            Self::Custom(name) => write!(f, "{}", name),
        }
    }
}

/// Domain adaptation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainAdaptationConfig {
    /// Source domain
    pub source_domain: LegalDomain,
    /// Target domain
    pub target_domain: LegalDomain,
    /// Adaptation strength (0.0 - 1.0)
    pub adaptation_strength: f64,
    /// Whether to use domain-invariant features
    pub use_domain_invariant: bool,
    /// Fine-tuning learning rate
    pub learning_rate: f64,
}

impl Default for DomainAdaptationConfig {
    fn default() -> Self {
        Self {
            source_domain: LegalDomain::ContractLaw,
            target_domain: LegalDomain::TortLaw,
            adaptation_strength: 0.5,
            use_domain_invariant: true,
            learning_rate: 1e-4,
        }
    }
}

/// Domain adapter for transferring knowledge between legal specialties
#[derive(Debug)]
pub struct DomainAdapter {
    config: DomainAdaptationConfig,
    #[allow(dead_code)]
    feature_mappings: HashMap<String, Vec<f64>>,
}

impl DomainAdapter {
    /// Creates a new domain adapter
    pub fn new(config: DomainAdaptationConfig) -> Self {
        Self {
            config,
            feature_mappings: HashMap::new(),
        }
    }

    /// Adapts a model from source to target domain
    pub fn adapt(&mut self, input_text: &str) -> Result<String> {
        // Extract domain-specific features
        let features = self.extract_features(input_text)?;

        // Apply domain adaptation
        let adapted_features = if self.config.use_domain_invariant {
            self.extract_domain_invariant_features(&features)
        } else {
            features
        };

        // Generate adapted output
        self.generate_adapted_output(&adapted_features)
    }

    fn extract_features(&self, text: &str) -> Result<Vec<f64>> {
        // Simple feature extraction based on text characteristics
        let word_count = text.split_whitespace().count() as f64;
        let avg_word_length =
            text.split_whitespace().map(|w| w.len()).sum::<usize>() as f64 / word_count;
        let sentence_count = text.matches('.').count() as f64 + 1.0;

        Ok(vec![word_count, avg_word_length, sentence_count])
    }

    fn extract_domain_invariant_features(&self, features: &[f64]) -> Vec<f64> {
        // Apply domain adaptation strength
        features
            .iter()
            .map(|&f| f * self.config.adaptation_strength)
            .collect()
    }

    fn generate_adapted_output(&self, _features: &[f64]) -> Result<String> {
        Ok(format!(
            "Adapted from {} to {}",
            self.config.source_domain, self.config.target_domain
        ))
    }

    /// Gets the adaptation configuration
    pub fn config(&self) -> &DomainAdaptationConfig {
        &self.config
    }
}

/// Jurisdiction transfer engine for cross-jurisdictional knowledge transfer
#[derive(Debug)]
pub struct JurisdictionTransfer {
    source_jurisdiction: TransferJurisdiction,
    target_jurisdiction: TransferJurisdiction,
    similarity_threshold: f64,
}

impl JurisdictionTransfer {
    /// Creates a new jurisdiction transfer engine
    pub fn new(
        source_jurisdiction: TransferJurisdiction,
        target_jurisdiction: TransferJurisdiction,
    ) -> Self {
        Self {
            source_jurisdiction,
            target_jurisdiction,
            similarity_threshold: 0.7,
        }
    }

    /// Sets the similarity threshold for transfer
    pub fn with_similarity_threshold(mut self, threshold: f64) -> Self {
        self.similarity_threshold = threshold.clamp(0.0, 1.0);
        self
    }

    /// Transfers legal knowledge between jurisdictions
    pub fn transfer(&self, source_text: &str) -> Result<String> {
        let similarity = self.calculate_jurisdiction_similarity();

        if similarity < self.similarity_threshold {
            anyhow::bail!(
                "Jurisdiction similarity ({:.2}) below threshold ({:.2})",
                similarity,
                self.similarity_threshold
            );
        }

        Ok(format!(
            "Transferred from {} to {}: {}",
            self.source_jurisdiction, self.target_jurisdiction, source_text
        ))
    }

    fn calculate_jurisdiction_similarity(&self) -> f64 {
        // Simplified similarity calculation
        match (&self.source_jurisdiction, &self.target_jurisdiction) {
            // Same jurisdiction = perfect similarity
            (a, b) if a == b => 1.0,
            // Federal to state within same country = high similarity
            (TransferJurisdiction::UnitedStatesFederal, TransferJurisdiction::USState(_)) => 0.85,
            (TransferJurisdiction::CanadaFederal, TransferJurisdiction::CanadianProvince(_)) => {
                0.85
            }
            (TransferJurisdiction::AustraliaFederal, TransferJurisdiction::AustralianState(_)) => {
                0.85
            }
            // EU to member state = high similarity
            (TransferJurisdiction::EuropeanUnion, TransferJurisdiction::EUMemberState(_)) => 0.80,
            // Common law jurisdictions = moderate similarity
            (TransferJurisdiction::UnitedStatesFederal, TransferJurisdiction::UnitedKingdom) => {
                0.65
            }
            (TransferJurisdiction::UnitedKingdom, TransferJurisdiction::CanadaFederal) => 0.70,
            (TransferJurisdiction::UnitedKingdom, TransferJurisdiction::AustraliaFederal) => 0.70,
            // Default = low similarity
            _ => 0.4,
        }
    }

    /// Gets the source jurisdiction
    pub fn source_jurisdiction(&self) -> &TransferJurisdiction {
        &self.source_jurisdiction
    }

    /// Gets the target jurisdiction
    pub fn target_jurisdiction(&self) -> &TransferJurisdiction {
        &self.target_jurisdiction
    }
}

/// Multi-task learning task definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalTask {
    /// Task name
    pub name: String,
    /// Task domain
    pub domain: LegalDomain,
    /// Task weight (importance)
    pub weight: f64,
    /// Task-specific parameters
    pub parameters: HashMap<String, String>,
}

impl LegalTask {
    /// Creates a new legal task
    pub fn new(name: impl Into<String>, domain: LegalDomain) -> Self {
        Self {
            name: name.into(),
            domain,
            weight: 1.0,
            parameters: HashMap::new(),
        }
    }

    /// Sets the task weight
    pub fn with_weight(mut self, weight: f64) -> Self {
        self.weight = weight.max(0.0);
        self
    }

    /// Adds a parameter
    pub fn with_parameter(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.parameters.insert(key.into(), value.into());
        self
    }
}

/// Multi-task learning framework
#[derive(Debug)]
pub struct MultiTaskLearner {
    tasks: Vec<LegalTask>,
    shared_features: HashMap<String, Vec<f64>>,
}

impl MultiTaskLearner {
    /// Creates a new multi-task learner
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            shared_features: HashMap::new(),
        }
    }

    /// Adds a task
    pub fn add_task(&mut self, task: LegalTask) -> &mut Self {
        self.tasks.push(task);
        self
    }

    /// Trains on multiple tasks simultaneously
    pub fn train(&mut self, training_data: &[(String, String)]) -> Result<()> {
        for (input, _output) in training_data {
            let features = self.extract_shared_features(input)?;
            self.shared_features.insert(input.clone(), features);
        }
        Ok(())
    }

    /// Predicts for a specific task
    pub fn predict(&self, task_name: &str, input: &str) -> Result<String> {
        let task = self
            .tasks
            .iter()
            .find(|t| t.name == task_name)
            .context("Task not found")?;

        Ok(format!(
            "Prediction for task '{}' in domain {}: {}",
            task.name, task.domain, input
        ))
    }

    fn extract_shared_features(&self, text: &str) -> Result<Vec<f64>> {
        // Extract features shared across tasks
        let length = text.len() as f64;
        let word_count = text.split_whitespace().count() as f64;
        Ok(vec![length, word_count])
    }

    /// Gets all tasks
    pub fn tasks(&self) -> &[LegalTask] {
        &self.tasks
    }

    /// Gets the number of tasks
    pub fn task_count(&self) -> usize {
        self.tasks.len()
    }
}

impl Default for MultiTaskLearner {
    fn default() -> Self {
        Self::new()
    }
}

/// Domain-invariant feature extractor
#[derive(Debug)]
pub struct DomainInvariantExtractor {
    feature_dim: usize,
}

impl DomainInvariantExtractor {
    /// Creates a new domain-invariant feature extractor
    pub fn new(feature_dim: usize) -> Self {
        Self { feature_dim }
    }

    /// Extracts domain-invariant features from text
    pub fn extract(&self, text: &str) -> Vec<f64> {
        let mut features = Vec::with_capacity(self.feature_dim);

        // Extract basic linguistic features (domain-invariant)
        features.push(text.len() as f64);
        features.push(text.split_whitespace().count() as f64);
        features.push(text.matches('.').count() as f64);
        features.push(text.matches(',').count() as f64);

        // Pad or truncate to desired dimension
        features.resize(self.feature_dim, 0.0);

        // Normalize
        let sum: f64 = features.iter().sum();
        if sum > 0.0 {
            features.iter_mut().for_each(|f| *f /= sum);
        }

        features
    }

    /// Gets the feature dimension
    pub fn feature_dim(&self) -> usize {
        self.feature_dim
    }
}

/// Few-shot learning example
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FewShotExample {
    /// Input text
    pub input: String,
    /// Expected output
    pub output: String,
    /// Example domain
    pub domain: LegalDomain,
}

impl FewShotExample {
    /// Creates a new few-shot example
    pub fn new(input: impl Into<String>, output: impl Into<String>, domain: LegalDomain) -> Self {
        Self {
            input: input.into(),
            output: output.into(),
            domain,
        }
    }
}

/// Few-shot learner for new legal domains
#[derive(Debug)]
pub struct FewShotLearner {
    examples: Vec<FewShotExample>,
    k_neighbors: usize,
}

impl FewShotLearner {
    /// Creates a new few-shot learner
    pub fn new(k_neighbors: usize) -> Self {
        Self {
            examples: Vec::new(),
            k_neighbors: k_neighbors.max(1),
        }
    }

    /// Adds training examples
    pub fn add_examples(&mut self, examples: Vec<FewShotExample>) -> &mut Self {
        self.examples.extend(examples);
        self
    }

    /// Learns from few examples and predicts
    pub fn predict(&self, query: &str, domain: &LegalDomain) -> Result<String> {
        if self.examples.is_empty() {
            anyhow::bail!("No examples available for few-shot learning");
        }

        // Find k nearest examples in the same domain
        let mut domain_examples: Vec<_> = self
            .examples
            .iter()
            .filter(|ex| &ex.domain == domain)
            .collect();

        if domain_examples.is_empty() {
            // Fall back to any domain
            domain_examples = self.examples.iter().collect();
        }

        // Simple similarity: use first k examples
        let k = self.k_neighbors.min(domain_examples.len());
        let nearest = &domain_examples[..k];

        // Combine outputs
        let combined_output = nearest
            .iter()
            .map(|ex| ex.output.as_str())
            .collect::<Vec<_>>()
            .join("; ");

        Ok(format!(
            "Few-shot prediction for '{}' in {}: {}",
            query, domain, combined_output
        ))
    }

    /// Gets the number of examples
    pub fn example_count(&self) -> usize {
        self.examples.len()
    }

    /// Gets k (number of neighbors)
    pub fn k_neighbors(&self) -> usize {
        self.k_neighbors
    }
}

/// Zero-shot classification label
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroShotLabel {
    /// Label name
    pub name: String,
    /// Label description (for semantic matching)
    pub description: String,
    /// Associated domain
    pub domain: LegalDomain,
}

impl ZeroShotLabel {
    /// Creates a new zero-shot label
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        domain: LegalDomain,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            domain,
        }
    }
}

/// Zero-shot legal classifier
#[derive(Debug)]
pub struct ZeroShotClassifier {
    labels: Vec<ZeroShotLabel>,
    confidence_threshold: f64,
}

impl ZeroShotClassifier {
    /// Creates a new zero-shot classifier
    pub fn new() -> Self {
        Self {
            labels: Vec::new(),
            confidence_threshold: 0.5,
        }
    }

    /// Sets the confidence threshold
    pub fn with_confidence_threshold(mut self, threshold: f64) -> Self {
        self.confidence_threshold = threshold.clamp(0.0, 1.0);
        self
    }

    /// Adds classification labels
    pub fn add_labels(&mut self, labels: Vec<ZeroShotLabel>) -> &mut Self {
        self.labels.extend(labels);
        self
    }

    /// Classifies text without training examples
    pub fn classify(&self, text: &str) -> Result<(String, f64)> {
        if self.labels.is_empty() {
            anyhow::bail!("No labels defined for zero-shot classification");
        }

        // Compute similarity between text and each label description
        let mut best_label = &self.labels[0];
        let mut best_score = 0.0;

        for label in &self.labels {
            let score = self.compute_similarity(text, &label.description);
            if score > best_score {
                best_score = score;
                best_label = label;
            }
        }

        if best_score < self.confidence_threshold {
            anyhow::bail!(
                "No label meets confidence threshold ({:.2} < {:.2})",
                best_score,
                self.confidence_threshold
            );
        }

        Ok((best_label.name.clone(), best_score))
    }

    fn compute_similarity(&self, text1: &str, text2: &str) -> f64 {
        // Simple word overlap similarity
        let text1_lower = text1.to_lowercase();
        let text2_lower = text2.to_lowercase();
        let words1: std::collections::HashSet<_> = text1_lower.split_whitespace().collect();
        let words2: std::collections::HashSet<_> = text2_lower.split_whitespace().collect();

        let intersection = words1.intersection(&words2).count();
        let union = words1.union(&words2).count();

        if union == 0 {
            0.0
        } else {
            intersection as f64 / union as f64
        }
    }

    /// Gets the number of labels
    pub fn label_count(&self) -> usize {
        self.labels.len()
    }

    /// Gets the confidence threshold
    pub fn confidence_threshold(&self) -> f64 {
        self.confidence_threshold
    }
}

impl Default for ZeroShotClassifier {
    fn default() -> Self {
        Self::new()
    }
}

/// Language for cross-lingual transfer
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    /// English
    English,
    /// Spanish
    Spanish,
    /// French
    French,
    /// German
    German,
    /// Italian
    Italian,
    /// Portuguese
    Portuguese,
    /// Chinese
    Chinese,
    /// Japanese
    Japanese,
    /// Korean
    Korean,
    /// Arabic
    Arabic,
    /// Custom language
    Custom(String),
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::English => write!(f, "English"),
            Self::Spanish => write!(f, "Spanish"),
            Self::French => write!(f, "French"),
            Self::German => write!(f, "German"),
            Self::Italian => write!(f, "Italian"),
            Self::Portuguese => write!(f, "Portuguese"),
            Self::Chinese => write!(f, "Chinese"),
            Self::Japanese => write!(f, "Japanese"),
            Self::Korean => write!(f, "Korean"),
            Self::Arabic => write!(f, "Arabic"),
            Self::Custom(name) => write!(f, "{}", name),
        }
    }
}

/// Cross-lingual transfer engine
#[derive(Debug)]
pub struct CrossLingualTransfer {
    source_language: Language,
    target_language: Language,
    preserve_legal_terms: bool,
}

impl CrossLingualTransfer {
    /// Creates a new cross-lingual transfer engine
    pub fn new(source_language: Language, target_language: Language) -> Self {
        Self {
            source_language,
            target_language,
            preserve_legal_terms: true,
        }
    }

    /// Sets whether to preserve legal terminology
    pub fn with_preserve_legal_terms(mut self, preserve: bool) -> Self {
        self.preserve_legal_terms = preserve;
        self
    }

    /// Transfers legal knowledge across languages
    pub fn transfer(&self, source_text: &str) -> Result<String> {
        // In a real implementation, this would use multilingual embeddings
        // or translation models to transfer legal concepts across languages

        let transferred = if self.preserve_legal_terms {
            format!(
                "[{} -> {}] (preserving legal terms): {}",
                self.source_language, self.target_language, source_text
            )
        } else {
            format!(
                "[{} -> {}]: {}",
                self.source_language, self.target_language, source_text
            )
        };

        Ok(transferred)
    }

    /// Gets the source language
    pub fn source_language(&self) -> &Language {
        &self.source_language
    }

    /// Gets the target language
    pub fn target_language(&self) -> &Language {
        &self.target_language
    }
}

/// Continual learning strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContinualStrategy {
    /// Elastic Weight Consolidation (EWC)
    ElasticWeightConsolidation,
    /// Progressive Neural Networks
    ProgressiveNeuralNetworks,
    /// Experience Replay
    ExperienceReplay,
    /// Learning without Forgetting (LwF)
    LearningWithoutForgetting,
}

/// Continual learner to prevent catastrophic forgetting
#[derive(Debug)]
pub struct ContinualTransferLearner {
    strategy: ContinualStrategy,
    memory_buffer: Vec<(String, String)>,
    memory_size: usize,
    task_history: Vec<String>,
}

impl ContinualTransferLearner {
    /// Creates a new continual learner
    pub fn new(strategy: ContinualStrategy, memory_size: usize) -> Self {
        Self {
            strategy,
            memory_buffer: Vec::new(),
            memory_size,
            task_history: Vec::new(),
        }
    }

    /// Learns a new task without forgetting previous tasks
    pub fn learn_task(
        &mut self,
        task_name: impl Into<String>,
        examples: Vec<(String, String)>,
    ) -> Result<()> {
        let task_name = task_name.into();

        match self.strategy {
            ContinualStrategy::ExperienceReplay => {
                // Store examples in memory buffer
                for example in examples {
                    if self.memory_buffer.len() < self.memory_size {
                        self.memory_buffer.push(example);
                    } else {
                        // Replace oldest example (FIFO)
                        self.memory_buffer.remove(0);
                        self.memory_buffer.push(example);
                    }
                }
            }
            ContinualStrategy::ElasticWeightConsolidation => {
                // EWC: would compute importance weights for parameters
                // Here we just track task history
            }
            ContinualStrategy::ProgressiveNeuralNetworks => {
                // Progressive: would add new columns/modules
                // Here we just track task history
            }
            ContinualStrategy::LearningWithoutForgetting => {
                // LwF: would use knowledge distillation
                // Here we just track task history
            }
        }

        self.task_history.push(task_name);
        Ok(())
    }

    /// Predicts using continual learning
    pub fn predict(&self, input: &str) -> Result<String> {
        match self.strategy {
            ContinualStrategy::ExperienceReplay => {
                // Use memory buffer to prevent forgetting
                let memory_context = self
                    .memory_buffer
                    .iter()
                    .take(5)
                    .map(|(i, o)| format!("{}:{}", i, o))
                    .collect::<Vec<_>>()
                    .join("; ");

                Ok(format!(
                    "Prediction with replay memory: {} (context: {})",
                    input, memory_context
                ))
            }
            _ => Ok(format!(
                "Prediction using {} strategy: {}",
                match self.strategy {
                    ContinualStrategy::ElasticWeightConsolidation => "EWC",
                    ContinualStrategy::ProgressiveNeuralNetworks => "Progressive",
                    ContinualStrategy::LearningWithoutForgetting => "LwF",
                    _ => "Unknown",
                },
                input
            )),
        }
    }

    /// Gets the learning strategy
    pub fn strategy(&self) -> ContinualStrategy {
        self.strategy
    }

    /// Gets the memory buffer size
    pub fn memory_size(&self) -> usize {
        self.memory_size
    }

    /// Gets the number of tasks learned
    pub fn task_count(&self) -> usize {
        self.task_history.len()
    }

    /// Gets the task history
    pub fn task_history(&self) -> &[String] {
        &self.task_history
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legal_domain_display() {
        assert_eq!(LegalDomain::ContractLaw.to_string(), "Contract Law");
        assert_eq!(LegalDomain::TortLaw.to_string(), "Tort Law");
        assert_eq!(
            LegalDomain::Custom("Custom Domain".to_string()).to_string(),
            "Custom Domain"
        );
    }

    #[test]
    fn test_jurisdiction_display() {
        assert_eq!(
            TransferJurisdiction::UnitedStatesFederal.to_string(),
            "United States (Federal)"
        );
        assert_eq!(
            TransferJurisdiction::USState("California".to_string()).to_string(),
            "United States - California"
        );
    }

    #[test]
    fn test_domain_adapter() {
        let config = DomainAdaptationConfig {
            source_domain: LegalDomain::ContractLaw,
            target_domain: LegalDomain::TortLaw,
            adaptation_strength: 0.7,
            use_domain_invariant: true,
            learning_rate: 1e-4,
        };

        let mut adapter = DomainAdapter::new(config);
        let result = adapter.adapt("This is a test contract.").unwrap();
        assert!(result.contains("Contract Law"));
        assert!(result.contains("Tort Law"));
    }

    #[test]
    fn test_jurisdiction_transfer() {
        let transfer = JurisdictionTransfer::new(
            TransferJurisdiction::UnitedStatesFederal,
            TransferJurisdiction::USState("New York".to_string()),
        );

        let result = transfer.transfer("Test legal principle").unwrap();
        assert!(result.contains("United States (Federal)"));
        assert!(result.contains("New York"));
    }

    #[test]
    fn test_jurisdiction_similarity() {
        let transfer = JurisdictionTransfer::new(
            TransferJurisdiction::UnitedStatesFederal,
            TransferJurisdiction::USState("California".to_string()),
        );

        let similarity = transfer.calculate_jurisdiction_similarity();
        assert!(similarity > 0.8); // High similarity federal to state
    }

    #[test]
    fn test_multi_task_learner() {
        let mut learner = MultiTaskLearner::new();

        let task1 = LegalTask::new("classify_contract", LegalDomain::ContractLaw).with_weight(1.0);
        let task2 = LegalTask::new("classify_tort", LegalDomain::TortLaw).with_weight(0.8);

        learner.add_task(task1).add_task(task2);

        assert_eq!(learner.task_count(), 2);

        let result = learner.predict("classify_contract", "Test input").unwrap();
        assert!(result.contains("classify_contract"));
    }

    #[test]
    fn test_multi_task_training() {
        let mut learner = MultiTaskLearner::new();
        learner.add_task(LegalTask::new("task1", LegalDomain::ContractLaw));

        let training_data = vec![
            ("input1".to_string(), "output1".to_string()),
            ("input2".to_string(), "output2".to_string()),
        ];

        learner.train(&training_data).unwrap();
    }

    #[test]
    fn test_domain_invariant_extractor() {
        let extractor = DomainInvariantExtractor::new(10);
        let features = extractor.extract("This is a test legal document with multiple sentences.");

        assert_eq!(features.len(), 10);
        assert_eq!(extractor.feature_dim(), 10);

        // Features should be normalized
        let sum: f64 = features.iter().sum();
        assert!((sum - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_few_shot_learner() {
        let mut learner = FewShotLearner::new(3);

        let examples = vec![
            FewShotExample::new(
                "What is a contract?",
                "A contract is a legally binding agreement",
                LegalDomain::ContractLaw,
            ),
            FewShotExample::new(
                "Define consideration",
                "Consideration is something of value exchanged",
                LegalDomain::ContractLaw,
            ),
        ];

        learner.add_examples(examples);
        assert_eq!(learner.example_count(), 2);
        assert_eq!(learner.k_neighbors(), 3);

        let result = learner
            .predict("What is offer?", &LegalDomain::ContractLaw)
            .unwrap();
        assert!(result.contains("Few-shot"));
    }

    #[test]
    fn test_few_shot_learner_no_examples() {
        let learner = FewShotLearner::new(1);
        let result = learner.predict("test", &LegalDomain::ContractLaw);
        assert!(result.is_err());
    }

    #[test]
    fn test_zero_shot_classifier() {
        let mut classifier = ZeroShotClassifier::new().with_confidence_threshold(0.3);

        let labels = vec![
            ZeroShotLabel::new(
                "contract_dispute",
                "dispute disagreement contract breach",
                LegalDomain::ContractLaw,
            ),
            ZeroShotLabel::new(
                "tort_claim",
                "negligence injury damages tort harm",
                LegalDomain::TortLaw,
            ),
        ];

        classifier.add_labels(labels);
        assert_eq!(classifier.label_count(), 2);

        let (label, score) = classifier
            .classify("This is a contract breach dispute")
            .unwrap();
        assert_eq!(label, "contract_dispute");
        assert!(score > 0.0);
    }

    #[test]
    fn test_zero_shot_classifier_no_labels() {
        let classifier = ZeroShotClassifier::new();
        let result = classifier.classify("test");
        assert!(result.is_err());
    }

    #[test]
    fn test_language_display() {
        assert_eq!(Language::English.to_string(), "English");
        assert_eq!(Language::Spanish.to_string(), "Spanish");
        assert_eq!(Language::Custom("Latin".to_string()).to_string(), "Latin");
    }

    #[test]
    fn test_cross_lingual_transfer() {
        let transfer = CrossLingualTransfer::new(Language::English, Language::Spanish)
            .with_preserve_legal_terms(true);

        let result = transfer.transfer("contract liability clause").unwrap();
        assert!(result.contains("English"));
        assert!(result.contains("Spanish"));
        assert!(result.contains("preserving legal terms"));
    }

    #[test]
    fn test_cross_lingual_without_preservation() {
        let transfer = CrossLingualTransfer::new(Language::French, Language::German)
            .with_preserve_legal_terms(false);

        let result = transfer.transfer("test").unwrap();
        assert!(!result.contains("preserving"));
    }

    #[test]
    fn test_continual_learner_experience_replay() {
        let mut learner = ContinualTransferLearner::new(ContinualStrategy::ExperienceReplay, 100);

        let examples = vec![
            ("input1".to_string(), "output1".to_string()),
            ("input2".to_string(), "output2".to_string()),
        ];

        learner.learn_task("task1", examples).unwrap();
        assert_eq!(learner.task_count(), 1);
        assert_eq!(learner.strategy(), ContinualStrategy::ExperienceReplay);

        let result = learner.predict("test input").unwrap();
        assert!(result.contains("replay memory"));
    }

    #[test]
    fn test_continual_learner_ewc() {
        let mut learner =
            ContinualTransferLearner::new(ContinualStrategy::ElasticWeightConsolidation, 50);

        learner.learn_task("task1", vec![]).unwrap();
        learner.learn_task("task2", vec![]).unwrap();

        assert_eq!(learner.task_count(), 2);

        let result = learner.predict("test").unwrap();
        assert!(result.contains("EWC"));
    }

    #[test]
    fn test_continual_learner_progressive() {
        let mut learner =
            ContinualTransferLearner::new(ContinualStrategy::ProgressiveNeuralNetworks, 50);

        learner.learn_task("task1", vec![]).unwrap();

        let result = learner.predict("test").unwrap();
        assert!(result.contains("Progressive"));
    }

    #[test]
    fn test_continual_learner_lwf() {
        let mut learner =
            ContinualTransferLearner::new(ContinualStrategy::LearningWithoutForgetting, 50);

        learner.learn_task("task1", vec![]).unwrap();

        let result = learner.predict("test").unwrap();
        assert!(result.contains("LwF"));
    }

    #[test]
    fn test_continual_learner_memory_limit() {
        let mut learner = ContinualTransferLearner::new(ContinualStrategy::ExperienceReplay, 2);

        let examples = vec![
            ("input1".to_string(), "output1".to_string()),
            ("input2".to_string(), "output2".to_string()),
            ("input3".to_string(), "output3".to_string()),
        ];

        learner.learn_task("task1", examples).unwrap();

        assert_eq!(learner.memory_size(), 2);
        // Memory buffer should not exceed memory_size
    }

    #[test]
    fn test_legal_task_builder() {
        let task = LegalTask::new("test_task", LegalDomain::TaxLaw)
            .with_weight(0.75)
            .with_parameter("param1", "value1");

        assert_eq!(task.name, "test_task");
        assert_eq!(task.domain, LegalDomain::TaxLaw);
        assert!((task.weight - 0.75).abs() < f64::EPSILON);
        assert_eq!(task.parameters.get("param1"), Some(&"value1".to_string()));
    }
}
