//! Decision clustering analysis using k-means algorithm.
//!
//! This module provides clustering analysis to identify patterns and group
//! similar decisions together. This can help identify:
//! - Common decision patterns
//! - Similar cases
//! - Anomalous decisions that don't fit clusters
//! - Decision complexity distribution

use crate::{AuditRecord, AuditResult};
use chrono::Timelike;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// A decision cluster.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionCluster {
    /// Cluster ID
    pub id: usize,
    /// Center of the cluster (feature vector)
    pub center: Vec<f64>,
    /// Records in this cluster
    pub records: Vec<Uuid>,
    /// Size of the cluster
    pub size: usize,
}

/// Features extracted from an audit record for clustering.
#[derive(Debug, Clone)]
struct RecordFeatures {
    record_id: Uuid,
    features: Vec<f64>,
}

impl RecordFeatures {
    /// Extracts features from an audit record.
    fn from_record(record: &AuditRecord) -> Self {
        let mut features = Vec::new();

        // Feature 1: Event type (encoded as number)
        let event_type_code = match record.event_type {
            crate::EventType::AutomaticDecision => 1.0,
            crate::EventType::DiscretionaryReview => 2.0,
            crate::EventType::HumanOverride => 3.0,
            crate::EventType::Appeal => 4.0,
            crate::EventType::StatuteModified => 5.0,
            crate::EventType::SimulationRun => 6.0,
        };
        features.push(event_type_code);

        // Feature 2: Actor type (encoded as number)
        let actor_type_code = match &record.actor {
            crate::Actor::System { .. } => 1.0,
            crate::Actor::User { .. } => 2.0,
            crate::Actor::External { .. } => 3.0,
        };
        features.push(actor_type_code);

        // Feature 3: Result type (encoded as number)
        let result_type_code = match &record.result {
            crate::DecisionResult::Deterministic { .. } => 1.0,
            crate::DecisionResult::RequiresDiscretion { .. } => 2.0,
            crate::DecisionResult::Void { .. } => 3.0,
            crate::DecisionResult::Overridden { .. } => 4.0,
        };
        features.push(result_type_code);

        // Feature 4: Number of evaluated conditions
        features.push(record.context.evaluated_conditions.len() as f64);

        // Feature 5: Number of attributes
        features.push(record.context.attributes.len() as f64);

        // Feature 6: Hour of day (for temporal patterns)
        features.push(record.timestamp.hour() as f64);

        Self {
            record_id: record.id,
            features,
        }
    }

    /// Computes Euclidean distance to another feature vector.
    fn distance_to(&self, other: &[f64]) -> f64 {
        self.features
            .iter()
            .zip(other.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f64>()
            .sqrt()
    }
}

/// K-means clustering analyzer for decisions.
pub struct DecisionClusterer {
    k: usize,
    max_iterations: usize,
}

impl DecisionClusterer {
    /// Creates a new decision clusterer.
    pub fn new(k: usize) -> Self {
        Self {
            k,
            max_iterations: 100,
        }
    }

    /// Sets the maximum number of iterations.
    pub fn with_max_iterations(mut self, max_iterations: usize) -> Self {
        self.max_iterations = max_iterations;
        self
    }

    /// Performs k-means clustering on audit records.
    pub fn cluster(&self, records: &[AuditRecord]) -> AuditResult<ClusteringResult> {
        if records.is_empty() {
            return Ok(ClusteringResult {
                clusters: Vec::new(),
                iterations: 0,
                total_records: 0,
            });
        }

        // Extract features
        let features: Vec<RecordFeatures> =
            records.iter().map(RecordFeatures::from_record).collect();

        // Initialize cluster centers randomly
        let mut centers = self.initialize_centers(&features);

        let mut assignments = vec![0; features.len()];
        let mut iterations = 0;

        // K-means iterations
        for _ in 0..self.max_iterations {
            let mut changed = false;
            iterations += 1;

            // Assignment step: assign each point to nearest center
            for (i, feature) in features.iter().enumerate() {
                let nearest = self.find_nearest_center(feature, &centers);
                if assignments[i] != nearest {
                    assignments[i] = nearest;
                    changed = true;
                }
            }

            if !changed {
                break; // Converged
            }

            // Update step: recompute centers
            centers = self.update_centers(&features, &assignments);
        }

        // Build cluster result
        let clusters = self.build_clusters(&features, &assignments, &centers);

        Ok(ClusteringResult {
            clusters,
            iterations,
            total_records: records.len(),
        })
    }

    /// Initializes cluster centers using k-means++ algorithm.
    fn initialize_centers(&self, features: &[RecordFeatures]) -> Vec<Vec<f64>> {
        use rand::Rng;
        use rand::prelude::IndexedRandom;

        let mut rng = rand::rng();
        let mut centers = Vec::new();

        // Choose first center randomly
        if let Some(first) = features.choose(&mut rng) {
            centers.push(first.features.clone());
        }

        // Choose remaining centers with probability proportional to distance
        while centers.len() < self.k && centers.len() < features.len() {
            let mut distances: Vec<f64> = features
                .iter()
                .map(|f| {
                    centers
                        .iter()
                        .map(|c| f.distance_to(c))
                        .min_by(|a, b| a.partial_cmp(b).unwrap())
                        .unwrap_or(0.0)
                        .powi(2)
                })
                .collect();

            // Normalize to probabilities
            let sum: f64 = distances.iter().sum();
            if sum > 0.0 {
                for d in &mut distances {
                    *d /= sum;
                }
            }

            // Choose next center
            let r: f64 = rng.random();
            let mut cumsum = 0.0;
            for (i, &p) in distances.iter().enumerate() {
                cumsum += p;
                if r <= cumsum {
                    centers.push(features[i].features.clone());
                    break;
                }
            }
        }

        centers
    }

    /// Finds the nearest center for a feature vector.
    fn find_nearest_center(&self, feature: &RecordFeatures, centers: &[Vec<f64>]) -> usize {
        centers
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| {
                let dist_a = feature.distance_to(a);
                let dist_b = feature.distance_to(b);
                dist_a.partial_cmp(&dist_b).unwrap()
            })
            .map(|(i, _)| i)
            .unwrap_or(0)
    }

    /// Updates cluster centers based on current assignments.
    fn update_centers(&self, features: &[RecordFeatures], assignments: &[usize]) -> Vec<Vec<f64>> {
        let mut centers = vec![vec![0.0; features[0].features.len()]; self.k];
        let mut counts = vec![0; self.k];

        // Sum features for each cluster
        for (feature, &cluster) in features.iter().zip(assignments.iter()) {
            for (i, &val) in feature.features.iter().enumerate() {
                centers[cluster][i] += val;
            }
            counts[cluster] += 1;
        }

        // Compute means
        for (center, &count) in centers.iter_mut().zip(counts.iter()) {
            if count > 0 {
                for val in center.iter_mut() {
                    *val /= count as f64;
                }
            }
        }

        centers
    }

    /// Builds cluster result from assignments.
    fn build_clusters(
        &self,
        features: &[RecordFeatures],
        assignments: &[usize],
        centers: &[Vec<f64>],
    ) -> Vec<DecisionCluster> {
        let mut clusters = Vec::new();

        for (id, center) in centers.iter().enumerate() {
            let records: Vec<Uuid> = features
                .iter()
                .zip(assignments.iter())
                .filter(|(_, cluster)| **cluster == id)
                .map(|(f, _)| f.record_id)
                .collect();

            clusters.push(DecisionCluster {
                id,
                center: center.clone(),
                size: records.len(),
                records,
            });
        }

        clusters
    }
}

/// Result of clustering analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusteringResult {
    /// The identified clusters
    pub clusters: Vec<DecisionCluster>,
    /// Number of iterations to converge
    pub iterations: usize,
    /// Total number of records clustered
    pub total_records: usize,
}

impl ClusteringResult {
    /// Gets summary statistics about the clustering.
    pub fn summary(&self) -> ClusteringSummary {
        let cluster_sizes: Vec<usize> = self.clusters.iter().map(|c| c.size).collect();
        let avg_size = if !cluster_sizes.is_empty() {
            cluster_sizes.iter().sum::<usize>() as f64 / cluster_sizes.len() as f64
        } else {
            0.0
        };

        let max_size = cluster_sizes.iter().max().copied().unwrap_or(0);
        let min_size = cluster_sizes.iter().min().copied().unwrap_or(0);

        ClusteringSummary {
            num_clusters: self.clusters.len(),
            total_records: self.total_records,
            iterations: self.iterations,
            avg_cluster_size: avg_size,
            max_cluster_size: max_size,
            min_cluster_size: min_size,
        }
    }

    /// Finds anomalous records (records in very small clusters).
    pub fn find_anomalies(&self, threshold_size: usize) -> Vec<Uuid> {
        self.clusters
            .iter()
            .filter(|c| c.size <= threshold_size)
            .flat_map(|c| c.records.iter())
            .copied()
            .collect()
    }

    /// Gets cluster distribution.
    pub fn cluster_distribution(&self) -> HashMap<usize, usize> {
        self.clusters.iter().map(|c| (c.id, c.size)).collect()
    }
}

/// Summary statistics for clustering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusteringSummary {
    /// Number of clusters
    pub num_clusters: usize,
    /// Total records clustered
    pub total_records: usize,
    /// Iterations to converge
    pub iterations: usize,
    /// Average cluster size
    pub avg_cluster_size: f64,
    /// Maximum cluster size
    pub max_cluster_size: usize,
    /// Minimum cluster size
    pub min_cluster_size: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use std::collections::HashMap as StdHashMap;

    fn create_test_record(event_type: EventType) -> AuditRecord {
        AuditRecord::new(
            event_type,
            Actor::System {
                component: "test".to_string(),
            },
            "statute-1".to_string(),
            Uuid::new_v4(),
            DecisionContext::default(),
            DecisionResult::Deterministic {
                effect_applied: "test".to_string(),
                parameters: StdHashMap::new(),
            },
            None,
        )
    }

    #[test]
    fn test_record_features() {
        let record = create_test_record(EventType::AutomaticDecision);
        let features = RecordFeatures::from_record(&record);
        assert_eq!(features.features.len(), 6); // 6 features
        assert_eq!(features.record_id, record.id);
    }

    #[test]
    fn test_feature_distance() {
        let record = create_test_record(EventType::AutomaticDecision);
        let features = RecordFeatures::from_record(&record);
        let center = vec![1.0, 1.0, 1.0, 0.0, 0.0, 0.0];
        let distance = features.distance_to(&center);
        assert!(distance >= 0.0);
    }

    #[test]
    fn test_clustering_empty() {
        let clusterer = DecisionClusterer::new(3);
        let result = clusterer.cluster(&[]).unwrap();
        assert_eq!(result.clusters.len(), 0);
        assert_eq!(result.total_records, 0);
    }

    #[test]
    fn test_clustering_basic() {
        let records = vec![
            create_test_record(EventType::AutomaticDecision),
            create_test_record(EventType::AutomaticDecision),
            create_test_record(EventType::HumanOverride),
            create_test_record(EventType::HumanOverride),
        ];

        let clusterer = DecisionClusterer::new(2);
        let result = clusterer.cluster(&records).unwrap();

        assert_eq!(result.total_records, 4);
        assert!(result.iterations > 0);
        assert!(!result.clusters.is_empty());
    }

    #[test]
    fn test_clustering_summary() {
        let records = vec![
            create_test_record(EventType::AutomaticDecision),
            create_test_record(EventType::AutomaticDecision),
            create_test_record(EventType::HumanOverride),
        ];

        let clusterer = DecisionClusterer::new(2);
        let result = clusterer.cluster(&records).unwrap();
        let summary = result.summary();

        assert_eq!(summary.total_records, 3);
        assert!(summary.num_clusters > 0);
        assert!(summary.avg_cluster_size > 0.0);
    }

    #[test]
    fn test_find_anomalies() {
        let records = vec![
            create_test_record(EventType::AutomaticDecision),
            create_test_record(EventType::AutomaticDecision),
            create_test_record(EventType::HumanOverride),
        ];

        let clusterer = DecisionClusterer::new(3);
        let result = clusterer.cluster(&records).unwrap();

        // Small clusters (size 1) are anomalies
        let anomalies = result.find_anomalies(1);
        assert!(!anomalies.is_empty());
    }

    #[test]
    fn test_cluster_distribution() {
        let records = vec![
            create_test_record(EventType::AutomaticDecision),
            create_test_record(EventType::AutomaticDecision),
            create_test_record(EventType::HumanOverride),
        ];

        let clusterer = DecisionClusterer::new(2);
        let result = clusterer.cluster(&records).unwrap();
        let distribution = result.cluster_distribution();

        assert!(!distribution.is_empty());
        let total_in_clusters: usize = distribution.values().sum();
        assert_eq!(total_in_clusters, 3);
    }

    #[test]
    fn test_max_iterations() {
        let records = vec![
            create_test_record(EventType::AutomaticDecision),
            create_test_record(EventType::HumanOverride),
        ];

        let clusterer = DecisionClusterer::new(2).with_max_iterations(5);
        let result = clusterer.cluster(&records).unwrap();

        assert!(result.iterations <= 5);
    }
}
