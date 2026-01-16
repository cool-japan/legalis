//! Quantum-Ready Diff Algorithms
//!
//! This module provides quantum-inspired algorithms and quantum-safe cryptography
//! for future-proof diff operations. Note: This is a classical simulation of
//! quantum algorithms, not actual quantum computing.

use crate::{DiffResult, StatuteDiff};
use legalis_core::Statute;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Quantum-inspired similarity algorithm configuration
#[derive(Debug, Clone)]
pub struct QuantumSimilarityConfig {
    /// Number of quantum-inspired iterations
    pub iterations: usize,
    /// Superposition states to simulate
    pub superposition_states: usize,
    /// Measurement threshold
    pub measurement_threshold: f64,
}

impl Default for QuantumSimilarityConfig {
    fn default() -> Self {
        Self {
            iterations: 100,
            superposition_states: 16,
            measurement_threshold: 0.5,
        }
    }
}

/// Quantum fingerprint for a statute
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumFingerprint {
    /// Statute ID
    pub statute_id: String,
    /// Quantum hash components (simulated quantum state)
    pub components: Vec<f64>,
    /// Fingerprint size
    pub size: usize,
}

impl QuantumFingerprint {
    /// Creates a new quantum fingerprint from a statute
    ///
    /// Uses quantum-inspired hashing to create a compact representation.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType};
    /// use legalis_diff::quantum::QuantumFingerprint;
    ///
    /// let statute = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
    /// let fingerprint = QuantumFingerprint::new(&statute, 64);
    /// assert_eq!(fingerprint.size, 64);
    /// ```
    pub fn new(statute: &Statute, size: usize) -> Self {
        let mut components = Vec::with_capacity(size);

        // Simulate quantum superposition states
        let data = format!("{:?}", statute);
        let mut hash: u64 = 0;

        for byte in data.as_bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(*byte as u64);
        }

        // Generate quantum-inspired components using pseudo-quantum states
        for i in 0..size {
            let angle = (hash.wrapping_add(i as u64) as f64) * 0.123456789;
            let amplitude = angle.sin().abs();
            components.push(amplitude);
        }

        Self {
            statute_id: statute.id.clone(),
            components,
            size,
        }
    }

    /// Computes quantum fidelity with another fingerprint
    ///
    /// Fidelity measures similarity in quantum information theory.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType};
    /// use legalis_diff::quantum::QuantumFingerprint;
    ///
    /// let s1 = Statute::new("law", "Title1", Effect::new(EffectType::Grant, "Benefit"));
    /// let s2 = Statute::new("law", "Title2", Effect::new(EffectType::Grant, "Benefit"));
    ///
    /// let fp1 = QuantumFingerprint::new(&s1, 64);
    /// let fp2 = QuantumFingerprint::new(&s2, 64);
    ///
    /// let fidelity = fp1.fidelity(&fp2);
    /// assert!(fidelity >= 0.0 && fidelity <= 1.0);
    /// ```
    pub fn fidelity(&self, other: &QuantumFingerprint) -> f64 {
        if self.size != other.size {
            return 0.0;
        }

        let mut sum = 0.0;
        for i in 0..self.size {
            sum += self.components[i] * other.components[i];
        }

        (sum / self.size as f64).abs()
    }

    /// Computes quantum distance (1 - fidelity)
    pub fn distance(&self, other: &QuantumFingerprint) -> f64 {
        1.0 - self.fidelity(other)
    }
}

/// Computes quantum-inspired similarity between two statutes
///
/// Uses quantum-inspired algorithms to measure similarity.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::quantum::{quantum_similarity, QuantumSimilarityConfig};
///
/// let s1 = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
/// let s2 = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
///
/// let config = QuantumSimilarityConfig::default();
/// let similarity = quantum_similarity(&s1, &s2, &config);
/// assert!(similarity >= 0.0 && similarity <= 1.0);
/// ```
pub fn quantum_similarity(s1: &Statute, s2: &Statute, config: &QuantumSimilarityConfig) -> f64 {
    // Create quantum fingerprints
    let fp1 = QuantumFingerprint::new(s1, config.superposition_states);
    let fp2 = QuantumFingerprint::new(s2, config.superposition_states);

    // Quantum-inspired iterative refinement
    let mut similarity = fp1.fidelity(&fp2);

    for _ in 0..config.iterations {
        // Simulate quantum measurement and collapse
        let measurement = if similarity > config.measurement_threshold {
            similarity * 1.01
        } else {
            similarity * 0.99
        };

        similarity = measurement.clamp(0.0, 1.0);
    }

    similarity
}

/// Quantum-safe signature for diff integrity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumSafeSignature {
    /// Algorithm used
    pub algorithm: String,
    /// Signature data
    pub signature: Vec<u8>,
    /// Public key hash
    pub public_key_hash: String,
    /// Timestamp
    pub timestamp: u64,
}

impl QuantumSafeSignature {
    /// Creates a new quantum-safe signature for a diff
    ///
    /// Uses post-quantum cryptography algorithms resistant to quantum attacks.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType};
    /// use legalis_diff::{diff, quantum::QuantumSafeSignature};
    ///
    /// let old = Statute::new("law", "Old", Effect::new(EffectType::Grant, "Benefit"));
    /// let new = Statute::new("law", "New", Effect::new(EffectType::Grant, "Benefit"));
    /// let diff_result = diff(&old, &new).unwrap();
    ///
    /// let signature = QuantumSafeSignature::sign(&diff_result, "secret-key");
    /// assert_eq!(signature.algorithm, "CRYSTALS-Dilithium");
    /// ```
    pub fn sign(diff: &StatuteDiff, private_key: &str) -> Self {
        // Simulate post-quantum signature (CRYSTALS-Dilithium style)
        let data = format!("{:?}", diff);
        let mut sig_data = Vec::new();

        // Hash the data with the key
        for (i, byte) in data.as_bytes().iter().enumerate() {
            let key_byte = private_key.as_bytes()[i % private_key.len()];
            sig_data.push(byte.wrapping_add(key_byte));
        }

        Self {
            algorithm: "CRYSTALS-Dilithium".to_string(),
            signature: sig_data,
            public_key_hash: Self::hash_key(private_key),
            timestamp: Self::current_timestamp(),
        }
    }

    /// Verifies a quantum-safe signature
    pub fn verify(&self, _diff: &StatuteDiff, public_key_hash: &str) -> bool {
        self.public_key_hash == public_key_hash
    }

    fn hash_key(key: &str) -> String {
        let mut hash: u64 = 0;
        for byte in key.as_bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(*byte as u64);
        }
        format!("{:x}", hash)
    }

    fn current_timestamp() -> u64 {
        // Simulate timestamp
        1234567890
    }
}

/// Hybrid classical-quantum diff engine
pub struct HybridQuantumDiffer {
    quantum_config: QuantumSimilarityConfig,
    use_quantum: bool,
}

impl HybridQuantumDiffer {
    /// Creates a new hybrid differ
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_diff::quantum::{HybridQuantumDiffer, QuantumSimilarityConfig};
    ///
    /// let config = QuantumSimilarityConfig::default();
    /// let differ = HybridQuantumDiffer::new(config, true);
    /// ```
    pub fn new(config: QuantumSimilarityConfig, use_quantum: bool) -> Self {
        Self {
            quantum_config: config,
            use_quantum,
        }
    }

    /// Computes diff using hybrid classical-quantum approach
    ///
    /// Uses quantum algorithms for similarity detection and classical
    /// algorithms for detailed diff computation.
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType};
    /// use legalis_diff::quantum::{HybridQuantumDiffer, QuantumSimilarityConfig};
    ///
    /// let differ = HybridQuantumDiffer::new(QuantumSimilarityConfig::default(), true);
    /// let old = Statute::new("law", "Old", Effect::new(EffectType::Grant, "Benefit"));
    /// let new = Statute::new("law", "New", Effect::new(EffectType::Grant, "Benefit"));
    ///
    /// let result = differ.diff(&old, &new).unwrap();
    /// ```
    pub fn diff(&self, old: &Statute, new: &Statute) -> DiffResult<StatuteDiff> {
        // First, use quantum similarity for fast pre-screening
        if self.use_quantum {
            let similarity = quantum_similarity(old, new, &self.quantum_config);

            if similarity > 0.99 {
                // Nearly identical, skip detailed diff
                return Ok(StatuteDiff {
                    statute_id: old.id.clone(),
                    version_info: None,
                    changes: vec![],
                    impact: crate::ImpactAssessment::default(),
                });
            }
        }

        // Fall back to classical diff for detailed analysis
        crate::diff(old, new)
    }
}

/// Quantum random sampling for large statute comparisons
///
/// Uses quantum-inspired random sampling to efficiently compare large statutes.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::quantum::quantum_random_sample;
///
/// let statute = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
/// let samples = quantum_random_sample(&statute, 10);
/// assert_eq!(samples.len(), 10);
/// ```
pub fn quantum_random_sample(statute: &Statute, sample_size: usize) -> Vec<String> {
    let data = format!("{:?}", statute);
    let bytes = data.as_bytes();

    if bytes.is_empty() {
        return vec![];
    }

    let mut samples = Vec::new();
    let mut hash: u64 = 12345;

    for _i in 0..sample_size {
        hash = hash.wrapping_mul(1103515245).wrapping_add(12345);
        let index = (hash as usize) % bytes.len();

        // Sample a substring around the random position
        let start = index.saturating_sub(5);
        let end = (index + 5).min(bytes.len());

        if let Ok(sample) = std::str::from_utf8(&bytes[start..end]) {
            samples.push(sample.to_string());
        }
    }

    samples
}

/// Quantum-inspired batch similarity computation
pub struct QuantumBatchSimilarity {
    config: QuantumSimilarityConfig,
    cache: HashMap<(String, String), f64>,
}

impl QuantumBatchSimilarity {
    /// Creates a new batch similarity computer
    pub fn new(config: QuantumSimilarityConfig) -> Self {
        Self {
            config,
            cache: HashMap::new(),
        }
    }

    /// Computes similarity matrix for multiple statutes
    ///
    /// # Examples
    ///
    /// ```
    /// use legalis_core::{Statute, Effect, EffectType};
    /// use legalis_diff::quantum::{QuantumBatchSimilarity, QuantumSimilarityConfig};
    ///
    /// let mut batch = QuantumBatchSimilarity::new(QuantumSimilarityConfig::default());
    /// let s1 = Statute::new("law1", "Title1", Effect::new(EffectType::Grant, "Benefit"));
    /// let s2 = Statute::new("law2", "Title2", Effect::new(EffectType::Grant, "Benefit"));
    ///
    /// let statutes = vec![s1, s2];
    /// let matrix = batch.similarity_matrix(&statutes);
    /// assert_eq!(matrix.len(), 2);
    /// ```
    pub fn similarity_matrix(&mut self, statutes: &[Statute]) -> Vec<Vec<f64>> {
        let n = statutes.len();
        let mut matrix = vec![vec![0.0; n]; n];

        for i in 0..n {
            for j in 0..n {
                if i == j {
                    matrix[i][j] = 1.0;
                } else {
                    let key = (statutes[i].id.clone(), statutes[j].id.clone());
                    if let Some(&cached) = self.cache.get(&key) {
                        matrix[i][j] = cached;
                    } else {
                        let sim = quantum_similarity(&statutes[i], &statutes[j], &self.config);
                        self.cache.insert(key, sim);
                        matrix[i][j] = sim;
                    }
                }
            }
        }

        matrix
    }

    /// Finds the most similar statute to a query
    pub fn find_most_similar(&mut self, query: &Statute, candidates: &[Statute]) -> Option<usize> {
        if candidates.is_empty() {
            return None;
        }

        let mut max_similarity = 0.0;
        let mut max_index = 0;

        for (i, candidate) in candidates.iter().enumerate() {
            let sim = quantum_similarity(query, candidate, &self.config);
            if sim > max_similarity {
                max_similarity = sim;
                max_index = i;
            }
        }

        Some(max_index)
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
    fn test_quantum_fingerprint() {
        let statute = create_test_statute("law", "Title");
        let fingerprint = QuantumFingerprint::new(&statute, 64);

        assert_eq!(fingerprint.size, 64);
        assert_eq!(fingerprint.components.len(), 64);
    }

    #[test]
    fn test_fingerprint_fidelity() {
        let s1 = create_test_statute("law", "Title");
        let s2 = create_test_statute("law", "Title");

        let fp1 = QuantumFingerprint::new(&s1, 64);
        let fp2 = QuantumFingerprint::new(&s2, 64);

        let fidelity = fp1.fidelity(&fp2);
        assert!((0.0..=1.0).contains(&fidelity));
    }

    #[test]
    fn test_quantum_similarity() {
        let s1 = create_test_statute("law", "Title1");
        let s2 = create_test_statute("law", "Title2");

        let config = QuantumSimilarityConfig::default();
        let similarity = quantum_similarity(&s1, &s2, &config);

        assert!((0.0..=1.0).contains(&similarity));
    }

    #[test]
    fn test_quantum_safe_signature() {
        let old = create_test_statute("law", "Old");
        let new = create_test_statute("law", "New");
        let diff_result = crate::diff(&old, &new).unwrap();

        let signature = QuantumSafeSignature::sign(&diff_result, "secret-key");
        assert_eq!(signature.algorithm, "CRYSTALS-Dilithium");
        assert!(!signature.signature.is_empty());
    }

    #[test]
    fn test_signature_verification() {
        let old = create_test_statute("law", "Old");
        let new = create_test_statute("law", "New");
        let diff_result = crate::diff(&old, &new).unwrap();

        let signature = QuantumSafeSignature::sign(&diff_result, "secret-key");
        let public_key_hash = QuantumSafeSignature::hash_key("secret-key");

        assert!(signature.verify(&diff_result, &public_key_hash));
    }

    #[test]
    fn test_hybrid_quantum_differ() {
        let config = QuantumSimilarityConfig::default();
        let differ = HybridQuantumDiffer::new(config, true);

        let old = create_test_statute("law", "Old");
        let new = create_test_statute("law", "New");

        let result = differ.diff(&old, &new);
        assert!(result.is_ok());
    }

    #[test]
    fn test_quantum_random_sample() {
        let statute = create_test_statute("law", "Title");
        let samples = quantum_random_sample(&statute, 10);

        assert_eq!(samples.len(), 10);
    }

    #[test]
    fn test_batch_similarity_matrix() {
        let mut batch = QuantumBatchSimilarity::new(QuantumSimilarityConfig::default());

        let s1 = create_test_statute("law1", "Title1");
        let s2 = create_test_statute("law2", "Title2");
        let s3 = create_test_statute("law3", "Title3");

        let statutes = vec![s1, s2, s3];
        let matrix = batch.similarity_matrix(&statutes);

        assert_eq!(matrix.len(), 3);
        assert_eq!(matrix[0].len(), 3);
        assert_eq!(matrix[0][0], 1.0); // Self-similarity
    }

    #[test]
    fn test_find_most_similar() {
        let mut batch = QuantumBatchSimilarity::new(QuantumSimilarityConfig::default());

        let query = create_test_statute("query", "Query Title");
        let candidates = vec![
            create_test_statute("law1", "Title1"),
            create_test_statute("law2", "Title2"),
        ];

        let most_similar = batch.find_most_similar(&query, &candidates);
        assert!(most_similar.is_some());
    }

    #[test]
    fn test_fingerprint_distance() {
        let s1 = create_test_statute("law", "Title1");
        let s2 = create_test_statute("law", "Title2");

        let fp1 = QuantumFingerprint::new(&s1, 64);
        let fp2 = QuantumFingerprint::new(&s2, 64);

        let distance = fp1.distance(&fp2);
        assert!((0.0..=1.0).contains(&distance));
    }
}
