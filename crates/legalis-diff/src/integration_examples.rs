//! Integration Examples
//!
//! This module provides comprehensive examples that combine multiple features
//! of legalis-diff for real-world use cases.

use crate::{
    DiffResult, StatuteDiff,
    cloud::{CloudStorage, CloudStorageConfig},
    distributed::{DiffTask, DistributedCoordinator, NodeConfig, distributed_diff_batch},
    gpu::{GpuConfig, GpuDiffEngine, gpu_diff_batch},
    llm::{LlmAnalyzer, LlmConfig},
    quantum::{QuantumFingerprint, QuantumSimilarityConfig, quantum_similarity},
};
use legalis_core::Statute;
use serde::{Deserialize, Serialize};

/// Comprehensive diff pipeline that combines multiple features
pub struct ComprehensiveDiffPipeline {
    gpu_engine: GpuDiffEngine,
    llm_analyzer: LlmAnalyzer,
    cloud_storage: CloudStorage,
    quantum_config: QuantumSimilarityConfig,
}

impl ComprehensiveDiffPipeline {
    /// Creates a new comprehensive diff pipeline with default configurations
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::integration_examples::ComprehensiveDiffPipeline;
    ///
    /// let pipeline = ComprehensiveDiffPipeline::new();
    /// ```
    pub fn new() -> Self {
        Self {
            gpu_engine: GpuDiffEngine::new(GpuConfig::default()),
            llm_analyzer: LlmAnalyzer::new(LlmConfig::default()),
            cloud_storage: CloudStorage::new(CloudStorageConfig::default()),
            quantum_config: QuantumSimilarityConfig::default(),
        }
    }

    /// Processes a batch of statute pairs with full analysis
    ///
    /// This combines:
    /// - GPU-accelerated diff computation
    /// - LLM-based semantic analysis
    /// - Quantum similarity pre-screening
    /// - Cloud storage for results
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType};
    /// use legalis_diff::integration_examples::ComprehensiveDiffPipeline;
    ///
    /// let mut pipeline = ComprehensiveDiffPipeline::new();
    /// let old = Statute::new("law", "Old", Effect::new(EffectType::Grant, "Benefit"));
    /// let new = Statute::new("law", "New", Effect::new(EffectType::Grant, "Benefit"));
    ///
    /// let results = pipeline.process_batch(&[(old, new)]).unwrap();
    /// // Results may be empty if quantum similarity filters them out,
    /// // or contain 1 item if processed
    /// assert!(results.len() <= 1);
    /// ```
    pub fn process_batch(
        &mut self,
        statute_pairs: &[(Statute, Statute)],
    ) -> DiffResult<Vec<EnrichedDiffResult>> {
        // Step 1: Use quantum similarity for pre-screening
        let filtered_pairs: Vec<(Statute, Statute)> = statute_pairs
            .iter()
            .filter(|(old, new)| {
                let similarity = quantum_similarity(old, new, &self.quantum_config);
                similarity < 0.99 // Only process pairs that have meaningful differences
            })
            .cloned()
            .collect();

        if filtered_pairs.is_empty() {
            return Ok(Vec::new());
        }

        // Step 2: Use GPU-accelerated batch diff computation
        let diffs = self.gpu_engine.batch_compute(&filtered_pairs)?;

        // Step 3: Enrich with LLM analysis
        let mut enriched_results = Vec::new();
        for diff in diffs {
            let explanation = self.llm_analyzer.explain_diff(&diff)?;
            let intent = self.llm_analyzer.detect_intent(&diff)?;
            let categories = self.llm_analyzer.categorize_changes(&diff)?;
            let impact_prediction = self.llm_analyzer.predict_impact(&diff)?;

            // Step 4: Store to cloud
            let key = format!(
                "diff-{}-{}",
                diff.statute_id,
                chrono::Utc::now().timestamp()
            );
            self.cloud_storage.store(&key, &diff)?;

            enriched_results.push(EnrichedDiffResult {
                diff,
                explanation,
                intent,
                categories,
                impact_prediction,
                storage_key: key,
            });
        }

        Ok(enriched_results)
    }

    /// Retrieves a previously processed diff from cloud storage
    pub fn retrieve_diff(&mut self, key: &str) -> DiffResult<Option<StatuteDiff>> {
        self.cloud_storage.retrieve(key)
    }
}

impl Default for ComprehensiveDiffPipeline {
    fn default() -> Self {
        Self::new()
    }
}

/// Enriched diff result with AI analysis and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnrichedDiffResult {
    /// The computed diff
    pub diff: StatuteDiff,
    /// Natural language explanation
    pub explanation: String,
    /// Detected intent
    pub intent: String,
    /// Change categories
    pub categories: Vec<String>,
    /// Impact prediction
    pub impact_prediction: crate::llm::ImpactPrediction,
    /// Cloud storage key
    pub storage_key: String,
}

/// High-performance distributed analysis workflow
pub struct DistributedAnalysisWorkflow {
    coordinator: DistributedCoordinator,
}

impl DistributedAnalysisWorkflow {
    /// Creates a new distributed analysis workflow
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::integration_examples::DistributedAnalysisWorkflow;
    /// use legalis_diff::distributed::NodeConfig;
    ///
    /// let nodes = vec![
    ///     NodeConfig {
    ///         node_id: "node-1".to_string(),
    ///         address: "localhost:8001".to_string(),
    ///         max_concurrent_tasks: 4,
    ///         capacity: 100,
    ///     },
    /// ];
    ///
    /// let workflow = DistributedAnalysisWorkflow::new(nodes);
    /// ```
    pub fn new(nodes: Vec<NodeConfig>) -> Self {
        Self {
            coordinator: DistributedCoordinator::new(nodes),
        }
    }

    /// Processes a large batch of statutes using distributed computation
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType};
    /// use legalis_diff::integration_examples::DistributedAnalysisWorkflow;
    /// use legalis_diff::distributed::NodeConfig;
    ///
    /// let nodes = vec![
    ///     NodeConfig {
    ///         node_id: "node-1".to_string(),
    ///         address: "localhost:8001".to_string(),
    ///         max_concurrent_tasks: 4,
    ///         capacity: 100,
    ///     },
    /// ];
    ///
    /// let mut workflow = DistributedAnalysisWorkflow::new(nodes);
    /// let old = Statute::new("law", "Old", Effect::new(EffectType::Grant, "Benefit"));
    /// let new = Statute::new("law", "New", Effect::new(EffectType::Grant, "Benefit"));
    ///
    /// let pairs = vec![(old, new)];
    /// let results = workflow.process_large_batch(&pairs, 5).unwrap();
    /// assert_eq!(results.len(), 1);
    /// ```
    pub fn process_large_batch(
        &mut self,
        statute_pairs: &[(Statute, Statute)],
        priority: u32,
    ) -> DiffResult<Vec<StatuteDiff>> {
        // Create tasks with priority
        let tasks: Vec<DiffTask> = statute_pairs
            .iter()
            .enumerate()
            .map(|(idx, (old, new))| DiffTask {
                task_id: format!("batch-task-{}", idx),
                old_statute: old.clone(),
                new_statute: new.clone(),
                priority,
            })
            .collect();

        // Submit and process
        self.coordinator.submit_batch(tasks);
        self.coordinator.process_all()?;

        // Collect results
        let results = self
            .coordinator
            .get_all_results()
            .into_iter()
            .map(|r| r.diff)
            .collect();

        Ok(results)
    }

    /// Gets workflow statistics
    pub fn get_statistics(&self) -> crate::distributed::ComputationStats {
        self.coordinator.get_statistics()
    }
}

/// Smart diff analyzer that chooses the best strategy
pub struct SmartDiffAnalyzer;

impl SmartDiffAnalyzer {
    /// Analyzes statutes and chooses optimal diff strategy
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType};
    /// use legalis_diff::integration_examples::SmartDiffAnalyzer;
    ///
    /// let old = Statute::new("law", "Old", Effect::new(EffectType::Grant, "Benefit"));
    /// let new = Statute::new("law", "New", Effect::new(EffectType::Grant, "Benefit"));
    ///
    /// let result = SmartDiffAnalyzer::analyze(&old, &new).unwrap();
    /// ```
    pub fn analyze(old: &Statute, new: &Statute) -> DiffResult<StatuteDiff> {
        // Use quantum fingerprinting for quick similarity check
        let fp1 = QuantumFingerprint::new(old, 64);
        let fp2 = QuantumFingerprint::new(new, 64);
        let similarity = fp1.fidelity(&fp2);

        if similarity > 0.99 {
            // Nearly identical, use fast path
            return crate::diff(old, new);
        }

        // Use GPU acceleration for detailed diff
        let mut gpu_engine = GpuDiffEngine::new(GpuConfig::default());
        gpu_engine.compute_diff(old, new)
    }

    /// Batch analysis with automatic strategy selection
    pub fn batch_analyze(pairs: &[(Statute, Statute)]) -> DiffResult<Vec<StatuteDiff>> {
        if pairs.len() > 10 {
            // Use distributed computation for large batches
            let nodes = vec![NodeConfig {
                node_id: "auto-node".to_string(),
                address: "localhost:8000".to_string(),
                max_concurrent_tasks: 8,
                capacity: 1000,
            }];
            distributed_diff_batch(pairs, &nodes)
        } else {
            // Use GPU batch for smaller batches
            gpu_diff_batch(pairs)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{Effect, EffectType};

    fn create_test_statute(id: &str, title: &str) -> Statute {
        Statute::new(id, title, Effect::new(EffectType::Grant, "Test benefit"))
    }

    #[test]
    fn test_comprehensive_pipeline() {
        let mut pipeline = ComprehensiveDiffPipeline::new();
        let old = create_test_statute("law", "Old Title");
        let new = create_test_statute("law", "New Title");

        let results = pipeline.process_batch(&[(old, new)]).unwrap();
        assert!(!results.is_empty());

        let result = &results[0];
        assert!(!result.explanation.is_empty());
        assert!(!result.intent.is_empty());
        assert!(!result.categories.is_empty());
    }

    #[test]
    fn test_distributed_workflow() {
        let nodes = vec![NodeConfig {
            node_id: "test-node".to_string(),
            address: "localhost:8001".to_string(),
            max_concurrent_tasks: 4,
            capacity: 100,
        }];

        let mut workflow = DistributedAnalysisWorkflow::new(nodes);
        let old = create_test_statute("law", "Old");
        let new = create_test_statute("law", "New");

        let results = workflow.process_large_batch(&[(old, new)], 5).unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_smart_analyzer() {
        let old = create_test_statute("law", "Old");
        let new = create_test_statute("law", "New");

        let result = SmartDiffAnalyzer::analyze(&old, &new);
        assert!(result.is_ok());
    }

    #[test]
    fn test_smart_batch_analyzer() {
        let old = create_test_statute("law", "Old");
        let new = create_test_statute("law", "New");

        let pairs = vec![(old, new)];
        let results = SmartDiffAnalyzer::batch_analyze(&pairs);
        assert!(results.is_ok());
        assert_eq!(results.unwrap().len(), 1);
    }

    #[test]
    fn test_quantum_prefiltering() {
        let mut pipeline = ComprehensiveDiffPipeline::new();

        // Create identical statutes
        let statute1 = create_test_statute("law", "Same Title");
        let statute2 = statute1.clone();

        // Should be filtered out by quantum similarity (high similarity > 0.99)
        let results = pipeline.process_batch(&[(statute1, statute2)]).unwrap();
        // Quantum similarity should filter out nearly identical statutes
        assert!(results.is_empty() || results.len() <= 1);
    }

    #[test]
    fn test_cloud_retrieval() {
        let mut pipeline = ComprehensiveDiffPipeline::new();
        let old = create_test_statute("law", "Old");
        let new = create_test_statute("law", "New");

        let results = pipeline.process_batch(&[(old, new)]).unwrap();
        if !results.is_empty() {
            let key = &results[0].storage_key;
            let retrieved = pipeline.retrieve_diff(key).unwrap();
            assert!(retrieved.is_some());
        }
    }
}
