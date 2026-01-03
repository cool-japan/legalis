//! Autonomous Legal Intelligence (v0.3.0)
//!
//! This module provides self-improving legal reasoning capabilities,
//! meta-learning for legal domains, active learning with human feedback,
//! legal knowledge distillation, and continual learning without forgetting.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::RwLock;

/// A legal reasoning pattern that can be learned and improved over time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningPattern {
    /// Pattern identifier
    pub id: String,
    /// Pattern description
    pub description: String,
    /// Input features that trigger this pattern
    pub input_features: Vec<String>,
    /// Expected output structure
    pub output_template: String,
    /// Success rate (0.0-1.0)
    pub success_rate: f64,
    /// Number of times this pattern has been used
    pub usage_count: usize,
    /// Timestamp of last update
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

impl ReasoningPattern {
    /// Creates a new reasoning pattern.
    pub fn new(id: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            description: description.into(),
            input_features: Vec::new(),
            output_template: String::new(),
            success_rate: 0.5,
            usage_count: 0,
            last_updated: chrono::Utc::now(),
        }
    }

    /// Updates the success rate based on feedback.
    pub fn update_success_rate(&mut self, was_successful: bool) {
        let alpha = 0.1; // Learning rate
        let target = if was_successful { 1.0 } else { 0.0 };
        self.success_rate = self.success_rate * (1.0 - alpha) + target * alpha;
        self.usage_count += 1;
        self.last_updated = chrono::Utc::now();
    }

    /// Adds an input feature to this pattern.
    pub fn add_feature(&mut self, feature: impl Into<String>) {
        self.input_features.push(feature.into());
    }
}

/// Self-improving legal reasoning engine.
///
/// This component learns from past reasoning tasks and improves over time.
pub struct SelfImprovingReasoner {
    patterns: Arc<RwLock<HashMap<String, ReasoningPattern>>>,
    feedback_history: Arc<RwLock<VecDeque<ReasoningFeedback>>>,
    max_history_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningFeedback {
    pub pattern_id: String,
    pub was_successful: bool,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub context: String,
}

impl SelfImprovingReasoner {
    /// Creates a new self-improving reasoner.
    pub fn new() -> Self {
        Self {
            patterns: Arc::new(RwLock::new(HashMap::new())),
            feedback_history: Arc::new(RwLock::new(VecDeque::new())),
            max_history_size: 1000,
        }
    }

    /// Registers a new reasoning pattern.
    pub async fn register_pattern(&self, pattern: ReasoningPattern) -> Result<()> {
        let mut patterns = self.patterns.write().await;
        patterns.insert(pattern.id.clone(), pattern);
        Ok(())
    }

    /// Finds the best pattern for a given input.
    pub async fn find_best_pattern(&self, input_features: &[String]) -> Option<ReasoningPattern> {
        let patterns = self.patterns.read().await;

        patterns
            .values()
            .filter(|p| {
                // Check if pattern features match input
                input_features.iter().any(|f| p.input_features.contains(f))
            })
            .max_by(|a, b| {
                // Rank by success rate and usage count
                let score_a = a.success_rate * (1.0 + (a.usage_count as f64).ln());
                let score_b = b.success_rate * (1.0 + (b.usage_count as f64).ln());
                score_a
                    .partial_cmp(&score_b)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .cloned()
    }

    /// Records feedback for a reasoning attempt.
    pub async fn record_feedback(&self, feedback: ReasoningFeedback) -> Result<()> {
        // Update pattern
        {
            let mut patterns = self.patterns.write().await;
            if let Some(pattern) = patterns.get_mut(&feedback.pattern_id) {
                pattern.update_success_rate(feedback.was_successful);
            }
        }

        // Store feedback history
        {
            let mut history = self.feedback_history.write().await;
            history.push_back(feedback);
            if history.len() > self.max_history_size {
                history.pop_front();
            }
        }

        Ok(())
    }

    /// Gets statistics about the reasoner's performance.
    pub async fn get_statistics(&self) -> ReasonerStats {
        let patterns = self.patterns.read().await;
        let history = self.feedback_history.read().await;

        let total_patterns = patterns.len();
        let avg_success_rate = if !patterns.is_empty() {
            patterns.values().map(|p| p.success_rate).sum::<f64>() / total_patterns as f64
        } else {
            0.0
        };

        let recent_success_rate = if !history.is_empty() {
            let recent_successful = history.iter().filter(|f| f.was_successful).count();
            recent_successful as f64 / history.len() as f64
        } else {
            0.0
        };

        ReasonerStats {
            total_patterns,
            avg_success_rate,
            recent_success_rate,
            total_feedback_count: history.len(),
        }
    }
}

impl Default for SelfImprovingReasoner {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasonerStats {
    pub total_patterns: usize,
    pub avg_success_rate: f64,
    pub recent_success_rate: f64,
    pub total_feedback_count: usize,
}

/// Meta-learning system for legal domains.
///
/// This component learns how to learn legal concepts more efficiently.
pub struct MetaLearner {
    domain_models: Arc<RwLock<HashMap<String, DomainModel>>>,
    learning_strategies: Arc<RwLock<Vec<LearningStrategy>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainModel {
    pub domain: String,
    pub concepts: Vec<String>,
    pub relationships: Vec<ConceptRelation>,
    pub learning_rate: f64,
    pub performance_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptRelation {
    pub from_concept: String,
    pub to_concept: String,
    pub relation_type: String,
    pub strength: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningStrategy {
    pub name: String,
    pub description: String,
    pub success_rate: f64,
    pub applicable_domains: Vec<String>,
}

impl MetaLearner {
    /// Creates a new meta-learner.
    pub fn new() -> Self {
        Self {
            domain_models: Arc::new(RwLock::new(HashMap::new())),
            learning_strategies: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Adds a new domain model.
    pub async fn add_domain_model(&self, model: DomainModel) -> Result<()> {
        let mut models = self.domain_models.write().await;
        models.insert(model.domain.clone(), model);
        Ok(())
    }

    /// Finds similar domains to leverage transfer learning.
    pub async fn find_similar_domains(&self, target_domain: &str) -> Vec<String> {
        let models = self.domain_models.read().await;

        let target_model = match models.get(target_domain) {
            Some(m) => m,
            None => return Vec::new(),
        };

        models
            .iter()
            .filter(|(domain, _)| *domain != target_domain)
            .map(|(domain, model)| {
                let similarity = self.calculate_domain_similarity(target_model, model);
                (domain.clone(), similarity)
            })
            .filter(|(_, sim)| *sim > 0.5)
            .map(|(domain, _)| domain)
            .collect()
    }

    fn calculate_domain_similarity(&self, model1: &DomainModel, model2: &DomainModel) -> f64 {
        // Simple Jaccard similarity based on concepts
        let concepts1: std::collections::HashSet<_> = model1.concepts.iter().collect();
        let concepts2: std::collections::HashSet<_> = model2.concepts.iter().collect();

        let intersection = concepts1.intersection(&concepts2).count();
        let union = concepts1.union(&concepts2).count();

        if union == 0 {
            0.0
        } else {
            intersection as f64 / union as f64
        }
    }

    /// Suggests the best learning strategy for a domain.
    pub async fn suggest_strategy(&self, domain: &str) -> Option<LearningStrategy> {
        let strategies = self.learning_strategies.read().await;

        strategies
            .iter()
            .filter(|s| s.applicable_domains.contains(&domain.to_string()))
            .max_by(|a, b| {
                a.success_rate
                    .partial_cmp(&b.success_rate)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .cloned()
    }

    /// Registers a new learning strategy.
    pub async fn register_strategy(&self, strategy: LearningStrategy) -> Result<()> {
        let mut strategies = self.learning_strategies.write().await;
        strategies.push(strategy);
        Ok(())
    }
}

impl Default for MetaLearner {
    fn default() -> Self {
        Self::new()
    }
}

/// Active learning system with human feedback.
///
/// This component identifies uncertain predictions and requests human feedback
/// to improve the model.
pub struct ActiveLearner {
    uncertainty_queue: Arc<RwLock<Vec<UncertainPrediction>>>,
    feedback_buffer: Arc<RwLock<Vec<HumanFeedback>>>,
    uncertainty_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UncertainPrediction {
    pub id: String,
    pub input: String,
    pub predictions: Vec<PredictionCandidate>,
    pub uncertainty_score: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionCandidate {
    pub output: String,
    pub confidence: f64,
    pub reasoning: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanFeedback {
    pub prediction_id: String,
    pub correct_output: String,
    pub feedback_notes: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl ActiveLearner {
    /// Creates a new active learner with a default uncertainty threshold.
    pub fn new() -> Self {
        Self {
            uncertainty_queue: Arc::new(RwLock::new(Vec::new())),
            feedback_buffer: Arc::new(RwLock::new(Vec::new())),
            uncertainty_threshold: 0.7,
        }
    }

    /// Sets the uncertainty threshold for requesting feedback.
    pub fn with_threshold(mut self, threshold: f64) -> Self {
        self.uncertainty_threshold = threshold.clamp(0.0, 1.0);
        self
    }

    /// Records a prediction for potential human review.
    pub async fn record_prediction(&self, prediction: UncertainPrediction) -> Result<()> {
        if prediction.uncertainty_score > self.uncertainty_threshold {
            let mut queue = self.uncertainty_queue.write().await;
            queue.push(prediction);
        }
        Ok(())
    }

    /// Gets the next uncertain prediction that needs human feedback.
    pub async fn get_next_uncertain(&self) -> Option<UncertainPrediction> {
        let mut queue = self.uncertainty_queue.write().await;

        // Sort by uncertainty (highest first)
        queue.sort_by(|a, b| {
            b.uncertainty_score
                .partial_cmp(&a.uncertainty_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        queue.pop()
    }

    /// Submits human feedback for a prediction.
    pub async fn submit_feedback(&self, feedback: HumanFeedback) -> Result<()> {
        let mut buffer = self.feedback_buffer.write().await;
        buffer.push(feedback);
        Ok(())
    }

    /// Gets all pending feedback for training.
    pub async fn get_pending_feedback(&self) -> Vec<HumanFeedback> {
        let buffer = self.feedback_buffer.read().await;
        buffer.clone()
    }

    /// Clears the feedback buffer after training.
    pub async fn clear_feedback_buffer(&self) {
        let mut buffer = self.feedback_buffer.write().await;
        buffer.clear();
    }

    /// Gets statistics about active learning.
    pub async fn get_statistics(&self) -> ActiveLearningStats {
        let queue = self.uncertainty_queue.read().await;
        let feedback = self.feedback_buffer.read().await;

        ActiveLearningStats {
            pending_reviews: queue.len(),
            feedback_collected: feedback.len(),
            avg_uncertainty: if !queue.is_empty() {
                queue.iter().map(|p| p.uncertainty_score).sum::<f64>() / queue.len() as f64
            } else {
                0.0
            },
        }
    }
}

impl Default for ActiveLearner {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveLearningStats {
    pub pending_reviews: usize,
    pub feedback_collected: usize,
    pub avg_uncertainty: f64,
}

/// Legal knowledge distillation system.
///
/// This component compresses large legal models into smaller, more efficient ones
/// while preserving accuracy.
pub struct KnowledgeDistiller {
    teacher_patterns: Arc<RwLock<Vec<TeacherPattern>>>,
    student_patterns: Arc<RwLock<Vec<StudentPattern>>>,
    distillation_config: DistillationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeacherPattern {
    pub id: String,
    pub input: String,
    pub output: String,
    pub confidence: f64,
    pub complexity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudentPattern {
    pub id: String,
    pub input: String,
    pub output: String,
    pub confidence: f64,
    pub teacher_id: String,
}

#[derive(Debug, Clone)]
pub struct DistillationConfig {
    pub temperature: f64,
    pub min_confidence: f64,
    pub compression_ratio: f64,
}

impl Default for DistillationConfig {
    fn default() -> Self {
        Self {
            temperature: 2.0,
            min_confidence: 0.7,
            compression_ratio: 0.1,
        }
    }
}

impl KnowledgeDistiller {
    /// Creates a new knowledge distiller.
    pub fn new(config: DistillationConfig) -> Self {
        Self {
            teacher_patterns: Arc::new(RwLock::new(Vec::new())),
            student_patterns: Arc::new(RwLock::new(Vec::new())),
            distillation_config: config,
        }
    }

    /// Adds a teacher pattern (from a large, accurate model).
    pub async fn add_teacher_pattern(&self, pattern: TeacherPattern) -> Result<()> {
        let mut patterns = self.teacher_patterns.write().await;
        patterns.push(pattern);
        Ok(())
    }

    /// Distills knowledge from teacher to student patterns.
    pub async fn distill_knowledge(&self) -> Result<Vec<StudentPattern>> {
        let teacher = self.teacher_patterns.read().await;
        let mut student = self.student_patterns.write().await;

        // Select high-confidence teacher patterns
        let selected_patterns: Vec<_> = teacher
            .iter()
            .filter(|p| p.confidence >= self.distillation_config.min_confidence)
            .collect();

        // Sample based on compression ratio
        let sample_size =
            (selected_patterns.len() as f64 * self.distillation_config.compression_ratio) as usize;
        let sample_size = sample_size.max(1);

        for (idx, teacher_pattern) in selected_patterns.iter().take(sample_size).enumerate() {
            let student_pattern = StudentPattern {
                id: format!("student_{}", idx),
                input: teacher_pattern.input.clone(),
                output: teacher_pattern.output.clone(),
                confidence: teacher_pattern.confidence,
                teacher_id: teacher_pattern.id.clone(),
            };
            student.push(student_pattern);
        }

        Ok(student.clone())
    }

    /// Gets distillation statistics.
    pub async fn get_statistics(&self) -> DistillationStats {
        let teacher = self.teacher_patterns.read().await;
        let student = self.student_patterns.read().await;

        DistillationStats {
            teacher_patterns: teacher.len(),
            student_patterns: student.len(),
            compression_ratio: if !teacher.is_empty() {
                student.len() as f64 / teacher.len() as f64
            } else {
                0.0
            },
            avg_teacher_confidence: if !teacher.is_empty() {
                teacher.iter().map(|p| p.confidence).sum::<f64>() / teacher.len() as f64
            } else {
                0.0
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistillationStats {
    pub teacher_patterns: usize,
    pub student_patterns: usize,
    pub compression_ratio: f64,
    pub avg_teacher_confidence: f64,
}

/// Continual learning system without catastrophic forgetting.
///
/// This component allows the model to learn new legal domains while
/// preserving knowledge from previous domains.
pub struct ContinualLearner {
    memory_buffer: Arc<RwLock<Vec<MemoryItem>>>,
    domain_importance: Arc<RwLock<HashMap<String, f64>>>,
    replay_config: ReplayConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryItem {
    pub id: String,
    pub domain: String,
    pub input: String,
    pub output: String,
    pub importance: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub struct ReplayConfig {
    pub buffer_size: usize,
    pub replay_ratio: f64,
    pub importance_decay: f64,
}

impl Default for ReplayConfig {
    fn default() -> Self {
        Self {
            buffer_size: 10000,
            replay_ratio: 0.2,
            importance_decay: 0.99,
        }
    }
}

impl ContinualLearner {
    /// Creates a new continual learner.
    pub fn new(config: ReplayConfig) -> Self {
        Self {
            memory_buffer: Arc::new(RwLock::new(Vec::new())),
            domain_importance: Arc::new(RwLock::new(HashMap::new())),
            replay_config: config,
        }
    }

    /// Adds a new memory item to the buffer.
    pub async fn add_memory(&self, item: MemoryItem) -> Result<()> {
        let mut buffer = self.memory_buffer.write().await;

        // Add new item
        buffer.push(item.clone());

        // Maintain buffer size by removing least important items
        if buffer.len() > self.replay_config.buffer_size {
            buffer.sort_by(|a, b| {
                b.importance
                    .partial_cmp(&a.importance)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
            buffer.truncate(self.replay_config.buffer_size);
        }

        // Update domain importance
        let mut importance = self.domain_importance.write().await;
        *importance.entry(item.domain).or_insert(0.0) += item.importance;

        Ok(())
    }

    /// Samples memories for replay during training.
    pub async fn sample_replay_memories(&self, domain: &str, count: usize) -> Vec<MemoryItem> {
        let buffer = self.memory_buffer.read().await;

        // Prioritize memories from the current domain and important past memories
        let mut relevant: Vec<_> = buffer
            .iter()
            .filter(|m| m.domain == domain || m.importance > 0.5)
            .cloned()
            .collect();

        // Sort by importance
        relevant.sort_by(|a, b| {
            b.importance
                .partial_cmp(&a.importance)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        relevant.into_iter().take(count).collect()
    }

    /// Decays importance of old memories.
    pub async fn decay_importance(&self) -> Result<()> {
        let mut buffer = self.memory_buffer.write().await;

        for item in buffer.iter_mut() {
            item.importance *= self.replay_config.importance_decay;
        }

        Ok(())
    }

    /// Gets continual learning statistics.
    pub async fn get_statistics(&self) -> ContinualLearningStats {
        let buffer = self.memory_buffer.read().await;
        let importance = self.domain_importance.read().await;

        let total_domains = importance.len();
        let avg_importance = if !buffer.is_empty() {
            buffer.iter().map(|m| m.importance).sum::<f64>() / buffer.len() as f64
        } else {
            0.0
        };

        ContinualLearningStats {
            memory_buffer_size: buffer.len(),
            total_domains,
            avg_importance,
            buffer_utilization: buffer.len() as f64 / self.replay_config.buffer_size as f64,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContinualLearningStats {
    pub memory_buffer_size: usize,
    pub total_domains: usize,
    pub avg_importance: f64,
    pub buffer_utilization: f64,
}

/// Autonomous legal intelligence orchestrator.
///
/// This component coordinates all autonomous learning capabilities.
pub struct AutonomousOrchestrator {
    reasoner: Arc<SelfImprovingReasoner>,
    meta_learner: Arc<MetaLearner>,
    active_learner: Arc<ActiveLearner>,
    distiller: Arc<KnowledgeDistiller>,
    continual_learner: Arc<ContinualLearner>,
}

impl AutonomousOrchestrator {
    /// Creates a new autonomous orchestrator.
    pub fn new() -> Self {
        Self {
            reasoner: Arc::new(SelfImprovingReasoner::new()),
            meta_learner: Arc::new(MetaLearner::new()),
            active_learner: Arc::new(ActiveLearner::new()),
            distiller: Arc::new(KnowledgeDistiller::new(DistillationConfig::default())),
            continual_learner: Arc::new(ContinualLearner::new(ReplayConfig::default())),
        }
    }

    /// Gets the self-improving reasoner.
    pub fn reasoner(&self) -> Arc<SelfImprovingReasoner> {
        self.reasoner.clone()
    }

    /// Gets the meta-learner.
    pub fn meta_learner(&self) -> Arc<MetaLearner> {
        self.meta_learner.clone()
    }

    /// Gets the active learner.
    pub fn active_learner(&self) -> Arc<ActiveLearner> {
        self.active_learner.clone()
    }

    /// Gets the knowledge distiller.
    pub fn distiller(&self) -> Arc<KnowledgeDistiller> {
        self.distiller.clone()
    }

    /// Gets the continual learner.
    pub fn continual_learner(&self) -> Arc<ContinualLearner> {
        self.continual_learner.clone()
    }

    /// Gets comprehensive statistics from all components.
    pub async fn get_comprehensive_stats(&self) -> AutonomousStats {
        AutonomousStats {
            reasoner: self.reasoner.get_statistics().await,
            active_learning: self.active_learner.get_statistics().await,
            distillation: self.distiller.get_statistics().await,
            continual_learning: self.continual_learner.get_statistics().await,
        }
    }
}

impl Default for AutonomousOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutonomousStats {
    pub reasoner: ReasonerStats,
    pub active_learning: ActiveLearningStats,
    pub distillation: DistillationStats,
    pub continual_learning: ContinualLearningStats,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reasoning_pattern_creation() {
        let pattern = ReasoningPattern::new("pattern1", "Test pattern");
        assert_eq!(pattern.id, "pattern1");
        assert_eq!(pattern.description, "Test pattern");
        assert_eq!(pattern.success_rate, 0.5);
        assert_eq!(pattern.usage_count, 0);
    }

    #[test]
    fn test_reasoning_pattern_update() {
        let mut pattern = ReasoningPattern::new("pattern1", "Test pattern");
        pattern.update_success_rate(true);
        assert!(pattern.success_rate > 0.5);
        assert_eq!(pattern.usage_count, 1);

        pattern.update_success_rate(false);
        assert_eq!(pattern.usage_count, 2);
    }

    #[tokio::test]
    async fn test_self_improving_reasoner() {
        let reasoner = SelfImprovingReasoner::new();

        let mut pattern = ReasoningPattern::new("test", "Test pattern");
        pattern.add_feature("contract_analysis");

        reasoner.register_pattern(pattern).await.unwrap();

        let found = reasoner
            .find_best_pattern(&["contract_analysis".to_string()])
            .await;
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, "test");
    }

    #[tokio::test]
    async fn test_meta_learner() {
        let learner = MetaLearner::new();

        let model = DomainModel {
            domain: "contract_law".to_string(),
            concepts: vec!["contract".to_string(), "agreement".to_string()],
            relationships: Vec::new(),
            learning_rate: 0.01,
            performance_score: 0.85,
        };

        learner.add_domain_model(model).await.unwrap();

        let strategy = LearningStrategy {
            name: "few_shot".to_string(),
            description: "Few-shot learning".to_string(),
            success_rate: 0.9,
            applicable_domains: vec!["contract_law".to_string()],
        };

        learner.register_strategy(strategy).await.unwrap();

        let suggested = learner.suggest_strategy("contract_law").await;
        assert!(suggested.is_some());
        assert_eq!(suggested.unwrap().name, "few_shot");
    }

    #[tokio::test]
    async fn test_active_learner() {
        let learner = ActiveLearner::new().with_threshold(0.6);

        let prediction = UncertainPrediction {
            id: "pred1".to_string(),
            input: "test input".to_string(),
            predictions: vec![],
            uncertainty_score: 0.8,
            timestamp: chrono::Utc::now(),
        };

        learner.record_prediction(prediction).await.unwrap();

        let stats = learner.get_statistics().await;
        assert_eq!(stats.pending_reviews, 1);
    }

    #[tokio::test]
    async fn test_knowledge_distiller() {
        let distiller = KnowledgeDistiller::new(DistillationConfig::default());

        let teacher = TeacherPattern {
            id: "teacher1".to_string(),
            input: "input".to_string(),
            output: "output".to_string(),
            confidence: 0.95,
            complexity: 0.7,
        };

        distiller.add_teacher_pattern(teacher).await.unwrap();

        let student_patterns = distiller.distill_knowledge().await.unwrap();
        assert!(!student_patterns.is_empty());
    }

    #[tokio::test]
    async fn test_continual_learner() {
        let learner = ContinualLearner::new(ReplayConfig::default());

        let memory = MemoryItem {
            id: "mem1".to_string(),
            domain: "contract_law".to_string(),
            input: "input".to_string(),
            output: "output".to_string(),
            importance: 0.8,
            timestamp: chrono::Utc::now(),
        };

        learner.add_memory(memory).await.unwrap();

        let stats = learner.get_statistics().await;
        assert_eq!(stats.memory_buffer_size, 1);
        assert_eq!(stats.total_domains, 1);
    }

    #[tokio::test]
    async fn test_autonomous_orchestrator() {
        let orchestrator = AutonomousOrchestrator::new();

        let stats = orchestrator.get_comprehensive_stats().await;
        assert_eq!(stats.reasoner.total_patterns, 0);
        assert_eq!(stats.active_learning.pending_reviews, 0);
    }
}
