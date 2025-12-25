//! Network effects modeling for legal compliance.
//!
//! This module provides data structures and configuration for modeling social influence,
//! information diffusion, and peer effects in legal compliance behavior.
//!
//! Note: This module provides reference data structures for network effects algorithms.
//! Full implementations require adaptation to work with the existing RelationshipGraph
//! API which uses UUIDs and specific relationship types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Social influence model configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfluenceConfig {
    /// Base influence strength (0.0 to 1.0).
    pub base_influence: f64,
    /// Decay factor for indirect influence.
    pub decay_factor: f64,
    /// Maximum influence propagation depth.
    pub max_depth: usize,
    /// Threshold for influence to take effect.
    pub influence_threshold: f64,
}

impl Default for InfluenceConfig {
    fn default() -> Self {
        Self {
            base_influence: 0.3,
            decay_factor: 0.5,
            max_depth: 3,
            influence_threshold: 0.1,
        }
    }
}

/// Information diffusion model.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DiffusionModel {
    /// Simple contagion (single exposure sufficient).
    SimpleContagion,
    /// Complex contagion (multiple exposures needed).
    ComplexContagion { threshold: f64 },
    /// Linear threshold model.
    LinearThreshold { threshold: f64 },
    /// Independent cascade.
    IndependentCascade { probability: f64 },
}

/// Network centrality metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CentralityMetrics {
    /// Degree centrality (number of connections).
    pub degree_centrality: HashMap<String, f64>,
    /// Betweenness centrality (bridge nodes).
    pub betweenness_centrality: HashMap<String, f64>,
    /// Closeness centrality (average distance to others).
    pub closeness_centrality: HashMap<String, f64>,
    /// Eigenvector centrality (influence score).
    pub eigenvector_centrality: HashMap<String, f64>,
}

/// Result of diffusion simulation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffusionResult {
    /// Entities that received information.
    pub informed_entities: Vec<String>,
    /// Total size of cascade.
    pub cascade_size: usize,
    /// Number of iterations.
    pub iterations: usize,
    /// History of cascade size over time.
    pub cascade_history: Vec<usize>,
}

/// Helper function for deterministic pseudo-random probability based on string hash.
pub fn hash_based_probability(s: &str) -> f64 {
    let mut hash = 0u64;
    for byte in s.bytes() {
        hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
    }
    (hash % 10000) as f64 / 10000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_influence_config() {
        let config = InfluenceConfig::default();
        assert!(config.base_influence > 0.0 && config.base_influence <= 1.0);
        assert!(config.decay_factor > 0.0 && config.decay_factor <= 1.0);
    }

    #[test]
    fn test_diffusion_model_types() {
        let model1 = DiffusionModel::SimpleContagion;
        let model2 = DiffusionModel::ComplexContagion { threshold: 0.5 };
        let model3 = DiffusionModel::LinearThreshold { threshold: 0.3 };
        let model4 = DiffusionModel::IndependentCascade { probability: 0.2 };

        // Just ensure different model types can be created
        assert!(matches!(model1, DiffusionModel::SimpleContagion));
        assert!(matches!(model2, DiffusionModel::ComplexContagion { .. }));
        assert!(matches!(model3, DiffusionModel::LinearThreshold { .. }));
        assert!(matches!(model4, DiffusionModel::IndependentCascade { .. }));
    }

    #[test]
    fn test_hash_based_probability() {
        let p1 = hash_based_probability("test1");
        let p2 = hash_based_probability("test2");
        let p3 = hash_based_probability("test1");

        // Should be deterministic
        assert_eq!(p1, p3);
        // Should be different for different inputs
        assert_ne!(p1, p2);
        // Should be in valid range
        assert!((0.0..=1.0).contains(&p1));
        assert!((0.0..=1.0).contains(&p2));
    }

    #[test]
    fn test_diffusion_result_creation() {
        let result = DiffusionResult {
            informed_entities: vec!["A".to_string(), "B".to_string()],
            cascade_size: 2,
            iterations: 3,
            cascade_history: vec![1, 2, 2],
        };

        assert_eq!(result.cascade_size, 2);
        assert_eq!(result.iterations, 3);
        assert_eq!(result.informed_entities.len(), 2);
    }

    #[test]
    fn test_centrality_metrics_creation() {
        let mut degree = HashMap::new();
        degree.insert("node1".to_string(), 0.5);

        let metrics = CentralityMetrics {
            degree_centrality: degree.clone(),
            betweenness_centrality: degree.clone(),
            closeness_centrality: degree.clone(),
            eigenvector_centrality: degree,
        };

        assert!(metrics.degree_centrality.contains_key("node1"));
    }
}
