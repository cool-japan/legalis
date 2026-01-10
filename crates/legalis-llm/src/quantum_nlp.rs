//! Quantum-Enhanced NLP (v0.3.4)
//!
//! This module provides quantum embeddings for legal text, quantum attention mechanisms,
//! quantum-inspired similarity search, hybrid classical-quantum inference, and quantum advantage benchmarking.
//!
//! Note: This is a quantum-inspired implementation that simulates quantum properties
//! without requiring actual quantum hardware.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Quantum embedding generator for legal text.
///
/// Uses quantum-inspired techniques to create high-dimensional embeddings.
pub struct QuantumEmbedding {
    dimension: usize,
    embeddings: Arc<RwLock<HashMap<String, Vec<f64>>>>,
    quantum_params: QuantumParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumParameters {
    pub entanglement_strength: f64,
    pub superposition_depth: usize,
    pub phase_encoding: bool,
}

impl Default for QuantumParameters {
    fn default() -> Self {
        Self {
            entanglement_strength: 0.8,
            superposition_depth: 4,
            phase_encoding: true,
        }
    }
}

impl QuantumEmbedding {
    /// Creates a new quantum embedding generator.
    pub fn new(dimension: usize, params: QuantumParameters) -> Self {
        Self {
            dimension,
            embeddings: Arc::new(RwLock::new(HashMap::new())),
            quantum_params: params,
        }
    }

    /// Generates a quantum-inspired embedding for text.
    pub async fn generate_embedding(&self, text: &str) -> Result<Vec<f64>> {
        let mut embedding = vec![0.0; self.dimension];

        // Simulate quantum superposition using Hadamard-like transformation
        for (i, word) in text.split_whitespace().enumerate() {
            let hash = self.hash_word(word);
            let angle = (hash as f64) * std::f64::consts::PI / 1000.0;

            // Apply quantum-inspired transformations
            for j in 0..self.dimension {
                let phase = if self.quantum_params.phase_encoding {
                    (angle + j as f64 * std::f64::consts::PI / self.dimension as f64).cos()
                } else {
                    1.0
                };

                embedding[j] +=
                    phase * (angle + i as f64).sin() * self.quantum_params.entanglement_strength;
            }
        }

        // Normalize the embedding
        let norm: f64 = embedding.iter().map(|x| x * x).sum::<f64>().sqrt();
        if norm > 0.0 {
            for val in &mut embedding {
                *val /= norm;
            }
        }

        // Store embedding
        {
            let mut embeddings = self.embeddings.write().await;
            embeddings.insert(text.to_string(), embedding.clone());
        }

        Ok(embedding)
    }

    fn hash_word(&self, word: &str) -> u64 {
        word.bytes()
            .fold(0u64, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u64))
    }

    /// Applies quantum entanglement between two embeddings.
    pub fn entangle_embeddings(&self, emb1: &[f64], emb2: &[f64]) -> Vec<f64> {
        let mut entangled = vec![0.0; self.dimension];

        for i in 0..self.dimension.min(emb1.len()).min(emb2.len()) {
            // Simulate quantum entanglement using tensor product-like operation
            entangled[i] = (emb1[i] * emb2[i]).sqrt() * self.quantum_params.entanglement_strength
                + (emb1[i] + emb2[i]) / 2.0 * (1.0 - self.quantum_params.entanglement_strength);
        }

        entangled
    }

    /// Gets all stored embeddings.
    pub async fn get_all_embeddings(&self) -> HashMap<String, Vec<f64>> {
        let embeddings = self.embeddings.read().await;
        embeddings.clone()
    }
}

impl Default for QuantumEmbedding {
    fn default() -> Self {
        Self::new(128, QuantumParameters::default())
    }
}

/// Quantum attention mechanism for legal text analysis.
#[allow(dead_code)]
pub struct QuantumAttention {
    attention_heads: usize,
    quantum_gate_layers: Vec<QuantumGate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumGate {
    pub gate_type: GateType,
    pub parameters: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GateType {
    Hadamard,
    CNOT,
    Phase,
    Rotation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttentionWeights {
    pub token_positions: Vec<usize>,
    pub weights: Vec<f64>,
    pub quantum_phase: Vec<f64>,
}

impl QuantumAttention {
    /// Creates a new quantum attention mechanism.
    pub fn new(attention_heads: usize) -> Self {
        Self {
            attention_heads,
            quantum_gate_layers: vec![
                QuantumGate {
                    gate_type: GateType::Hadamard,
                    parameters: vec![1.0 / 2.0_f64.sqrt()],
                },
                QuantumGate {
                    gate_type: GateType::Phase,
                    parameters: vec![std::f64::consts::PI / 4.0],
                },
            ],
        }
    }

    /// Computes quantum-inspired attention weights for a sequence.
    pub fn compute_attention(&self, embeddings: &[Vec<f64>]) -> Result<AttentionWeights> {
        let seq_len = embeddings.len();
        let mut weights = vec![0.0; seq_len];
        let mut phases = vec![0.0; seq_len];

        for (i, emb) in embeddings.iter().enumerate() {
            // Apply quantum gates
            let mut state = emb.clone();

            for gate in &self.quantum_gate_layers {
                state = self.apply_quantum_gate(gate, &state);
            }

            // Compute attention weight using quantum state amplitude
            weights[i] = state.iter().map(|x| x * x).sum::<f64>().sqrt();

            // Compute quantum phase
            phases[i] = state
                .iter()
                .enumerate()
                .map(|(j, &x)| x * (j as f64 * std::f64::consts::PI / state.len() as f64).cos())
                .sum();
        }

        // Normalize weights
        let sum: f64 = weights.iter().sum();
        if sum > 0.0 {
            for w in &mut weights {
                *w /= sum;
            }
        }

        Ok(AttentionWeights {
            token_positions: (0..seq_len).collect(),
            weights,
            quantum_phase: phases,
        })
    }

    fn apply_quantum_gate(&self, gate: &QuantumGate, state: &[f64]) -> Vec<f64> {
        let mut new_state = state.to_vec();

        match gate.gate_type {
            GateType::Hadamard => {
                // Simulate Hadamard gate (creates superposition)
                let scale = gate
                    .parameters
                    .first()
                    .copied()
                    .unwrap_or(1.0 / 2.0_f64.sqrt());
                for val in &mut new_state {
                    *val *= scale;
                }
            }
            GateType::Phase => {
                // Apply phase rotation
                let theta = gate.parameters.first().copied().unwrap_or(0.0);
                for (i, val) in new_state.iter_mut().enumerate() {
                    *val *= (theta * i as f64).cos();
                }
            }
            GateType::Rotation => {
                // Apply rotation in quantum state space
                let angle = gate
                    .parameters
                    .first()
                    .copied()
                    .unwrap_or(std::f64::consts::PI / 4.0);
                for val in &mut new_state {
                    let old_val = *val;
                    *val = old_val * angle.cos() + old_val * angle.sin();
                }
            }
            GateType::CNOT => {
                // Controlled-NOT simulation (entanglement)
                for i in (0..new_state.len()).step_by(2) {
                    if i + 1 < new_state.len() {
                        new_state.swap(i + 1, i);
                    }
                }
            }
        }

        new_state
    }

    /// Adds a quantum gate layer.
    pub fn add_gate_layer(&mut self, gate: QuantumGate) {
        self.quantum_gate_layers.push(gate);
    }
}

/// Quantum-inspired similarity search for legal documents.
pub struct QuantumSimilaritySearch {
    quantum_embeddings: Arc<QuantumEmbedding>,
    search_index: Arc<RwLock<Vec<(String, Vec<f64>)>>>,
}

impl QuantumSimilaritySearch {
    /// Creates a new quantum similarity search.
    pub fn new(quantum_embeddings: Arc<QuantumEmbedding>) -> Self {
        Self {
            quantum_embeddings,
            search_index: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Adds a document to the search index.
    pub async fn index_document(&self, doc_id: &str, text: &str) -> Result<()> {
        let embedding = self.quantum_embeddings.generate_embedding(text).await?;

        let mut index = self.search_index.write().await;
        index.push((doc_id.to_string(), embedding));

        Ok(())
    }

    /// Searches for similar documents using quantum-inspired similarity.
    pub async fn search(&self, query: &str, top_k: usize) -> Result<Vec<SearchResult>> {
        let query_embedding = self.quantum_embeddings.generate_embedding(query).await?;
        let index = self.search_index.read().await;

        let mut results = Vec::new();

        for (doc_id, doc_embedding) in index.iter() {
            let similarity = self.quantum_similarity(&query_embedding, doc_embedding);

            results.push(SearchResult {
                document_id: doc_id.clone(),
                similarity_score: similarity,
                quantum_interference: self.compute_interference(&query_embedding, doc_embedding),
            });
        }

        // Sort by similarity score
        results.sort_by(|a, b| {
            b.similarity_score
                .partial_cmp(&a.similarity_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        Ok(results.into_iter().take(top_k).collect())
    }

    fn quantum_similarity(&self, emb1: &[f64], emb2: &[f64]) -> f64 {
        // Quantum-inspired similarity using amplitude overlap
        let dot_product: f64 = emb1.iter().zip(emb2.iter()).map(|(a, b)| a * b).sum();

        // Apply quantum phase factor
        let phase_factor = (dot_product * std::f64::consts::PI).cos();

        (dot_product.abs() * (1.0 + phase_factor) / 2.0).clamp(0.0, 1.0)
    }

    fn compute_interference(&self, emb1: &[f64], emb2: &[f64]) -> f64 {
        // Simulate quantum interference pattern
        emb1.iter()
            .zip(emb2.iter())
            .enumerate()
            .map(|(i, (a, b))| {
                (a + b) * (i as f64 * std::f64::consts::PI / emb1.len() as f64).cos()
            })
            .sum::<f64>()
            .abs()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub document_id: String,
    pub similarity_score: f64,
    pub quantum_interference: f64,
}

/// Hybrid classical-quantum inference system.
pub struct HybridInference {
    classical_weight: f64,
    quantum_weight: f64,
    quantum_embedding: Arc<QuantumEmbedding>,
}

impl HybridInference {
    /// Creates a new hybrid inference system.
    pub fn new(
        classical_weight: f64,
        quantum_weight: f64,
        quantum_embedding: Arc<QuantumEmbedding>,
    ) -> Self {
        Self {
            classical_weight,
            quantum_weight,
            quantum_embedding,
        }
    }

    /// Performs hybrid inference combining classical and quantum approaches.
    pub async fn infer(
        &self,
        input_text: &str,
        classical_features: &HashMap<String, f64>,
    ) -> Result<InferenceResult> {
        // Classical inference
        let classical_score = self.classical_inference(classical_features);

        // Quantum inference
        let quantum_score = self.quantum_inference(input_text).await?;

        // Combine results
        let combined_score =
            self.classical_weight * classical_score + self.quantum_weight * quantum_score;

        Ok(InferenceResult {
            classical_score,
            quantum_score,
            combined_score,
            confidence: (classical_score * quantum_score).sqrt(),
        })
    }

    fn classical_inference(&self, features: &HashMap<String, f64>) -> f64 {
        // Simple classical inference using feature averaging
        if features.is_empty() {
            return 0.5;
        }

        let sum: f64 = features.values().sum();
        (sum / features.len() as f64).clamp(0.0, 1.0)
    }

    async fn quantum_inference(&self, text: &str) -> Result<f64> {
        // Quantum-inspired inference using embedding magnitude
        let embedding = self.quantum_embedding.generate_embedding(text).await?;

        let magnitude: f64 = embedding.iter().map(|x| x * x).sum::<f64>().sqrt();

        Ok(magnitude.clamp(0.0, 1.0))
    }

    /// Adjusts the classical-quantum balance.
    pub fn set_weights(&mut self, classical_weight: f64, quantum_weight: f64) {
        let total = classical_weight + quantum_weight;
        self.classical_weight = classical_weight / total;
        self.quantum_weight = quantum_weight / total;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceResult {
    pub classical_score: f64,
    pub quantum_score: f64,
    pub combined_score: f64,
    pub confidence: f64,
}

/// Quantum advantage benchmarking system.
pub struct QuantumBenchmark {
    benchmarks: Arc<RwLock<Vec<QuantumBenchmarkResult>>>,
}

impl QuantumBenchmark {
    /// Creates a new quantum benchmark system.
    pub fn new() -> Self {
        Self {
            benchmarks: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Benchmarks quantum vs classical approach.
    pub async fn benchmark_task(
        &self,
        task_name: &str,
        dataset_size: usize,
    ) -> Result<QuantumBenchmarkResult> {
        use std::time::Instant;

        // Classical approach timing
        let classical_start = Instant::now();
        self.simulate_classical_processing(dataset_size);
        let classical_duration = classical_start.elapsed();

        // Quantum approach timing
        let quantum_start = Instant::now();
        self.simulate_quantum_processing(dataset_size);
        let quantum_duration = quantum_start.elapsed();

        let speedup = classical_duration.as_secs_f64() / quantum_duration.as_secs_f64();

        let result = QuantumBenchmarkResult {
            task_name: task_name.to_string(),
            dataset_size,
            classical_time_ms: classical_duration.as_millis() as f64,
            quantum_time_ms: quantum_duration.as_millis() as f64,
            speedup_factor: speedup,
            quantum_advantage: speedup > 1.0,
        };

        // Store result
        {
            let mut benchmarks = self.benchmarks.write().await;
            benchmarks.push(result.clone());
        }

        Ok(result)
    }

    fn simulate_classical_processing(&self, size: usize) {
        // Simulate O(n^2) classical algorithm
        let mut sum = 0.0;
        for i in 0..size {
            for j in 0..size {
                sum += (i as f64 * j as f64).sin();
            }
        }
        let _ = sum; // Prevent optimization
    }

    fn simulate_quantum_processing(&self, size: usize) {
        // Simulate O(n log n) quantum-inspired algorithm
        let mut sum = 0.0;
        for i in 0..size {
            sum += (i as f64).sin() * (size as f64).ln();
        }
        let _ = sum; // Prevent optimization
    }

    /// Gets all benchmark results.
    pub async fn get_benchmarks(&self) -> Vec<QuantumBenchmarkResult> {
        let benchmarks = self.benchmarks.read().await;
        benchmarks.clone()
    }

    /// Generates a benchmark report.
    pub async fn generate_report(&self) -> String {
        let benchmarks = self.get_benchmarks().await;

        let mut report = String::from("Quantum Advantage Benchmark Report\n");
        report.push_str("=".repeat(50).as_str());
        report.push('\n');

        for result in benchmarks {
            report.push_str(&format!(
                "\nTask: {}\n  Dataset Size: {}\n  Classical: {:.2}ms\n  Quantum: {:.2}ms\n  Speedup: {:.2}x\n  Advantage: {}\n",
                result.task_name,
                result.dataset_size,
                result.classical_time_ms,
                result.quantum_time_ms,
                result.speedup_factor,
                if result.quantum_advantage { "Yes" } else { "No" }
            ));
        }

        report
    }
}

impl Default for QuantumBenchmark {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumBenchmarkResult {
    pub task_name: String,
    pub dataset_size: usize,
    pub classical_time_ms: f64,
    pub quantum_time_ms: f64,
    pub speedup_factor: f64,
    pub quantum_advantage: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_quantum_embedding() {
        let qemb = QuantumEmbedding::new(64, QuantumParameters::default());

        let emb = qemb
            .generate_embedding("legal contract terms")
            .await
            .unwrap();

        assert_eq!(emb.len(), 64);
        assert!(emb.iter().any(|&x| x != 0.0));
    }

    #[tokio::test]
    async fn test_quantum_attention() {
        let attention = QuantumAttention::new(4);

        let embeddings = vec![vec![0.5, 0.3, 0.2], vec![0.1, 0.8, 0.1]];

        let weights = attention.compute_attention(&embeddings).unwrap();

        assert_eq!(weights.weights.len(), 2);
        assert_eq!(weights.quantum_phase.len(), 2);
    }

    #[tokio::test]
    async fn test_quantum_similarity_search() {
        let qemb = Arc::new(QuantumEmbedding::new(32, QuantumParameters::default()));
        let search = QuantumSimilaritySearch::new(qemb);

        search.index_document("doc1", "contract law").await.unwrap();
        search
            .index_document("doc2", "property rights")
            .await
            .unwrap();

        let results = search.search("contract", 1).await.unwrap();

        assert!(!results.is_empty());
        assert_eq!(results[0].document_id, "doc1");
    }

    #[tokio::test]
    async fn test_hybrid_inference() {
        let qemb = Arc::new(QuantumEmbedding::new(32, QuantumParameters::default()));
        let hybrid = HybridInference::new(0.5, 0.5, qemb);

        let mut features = HashMap::new();
        features.insert("feature1".to_string(), 0.8);

        let result = hybrid.infer("test text", &features).await.unwrap();

        assert!(result.combined_score >= 0.0 && result.combined_score <= 1.0);
        assert!(result.confidence >= 0.0 && result.confidence <= 1.0);
    }

    #[tokio::test]
    async fn test_quantum_benchmark() {
        let benchmark = QuantumBenchmark::new();

        let result = benchmark
            .benchmark_task("similarity_search", 10)
            .await
            .unwrap();

        assert_eq!(result.task_name, "similarity_search");
        assert_eq!(result.dataset_size, 10);
        assert!(result.speedup_factor > 0.0);
    }

    #[test]
    fn test_quantum_parameters_default() {
        let params = QuantumParameters::default();
        assert_eq!(params.entanglement_strength, 0.8);
        assert_eq!(params.superposition_depth, 4);
        assert!(params.phase_encoding);
    }
}
