//! Network effects modeling for legal compliance.
//!
//! This module provides algorithms and data structures for modeling social influence,
//! information diffusion, and peer effects in legal compliance behavior.
//!
//! Fully integrated with UUID-based RelationshipGraph API.

use crate::relationships::RelationshipGraph;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use uuid::Uuid;

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
    pub degree_centrality: HashMap<Uuid, f64>,
    /// Betweenness centrality (bridge nodes).
    pub betweenness_centrality: HashMap<Uuid, f64>,
    /// Closeness centrality (average distance to others).
    pub closeness_centrality: HashMap<Uuid, f64>,
    /// Eigenvector centrality (influence score).
    pub eigenvector_centrality: HashMap<Uuid, f64>,
}

/// Result of diffusion simulation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffusionResult {
    /// Entities that received information.
    pub informed_entities: Vec<Uuid>,
    /// Total size of cascade.
    pub cascade_size: usize,
    /// Number of iterations.
    pub iterations: usize,
    /// History of cascade size over time.
    pub cascade_history: Vec<usize>,
}

/// Helper function for deterministic pseudo-random probability based on UUID.
pub fn hash_based_probability(id: &Uuid) -> f64 {
    let bytes = id.as_bytes();
    let mut hash = 0u64;
    for &byte in bytes {
        hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
    }
    (hash % 10000) as f64 / 10000.0
}

/// Calculate all centrality metrics for a relationship graph.
pub fn calculate_centrality_metrics(
    graph: &RelationshipGraph,
    entities: &[Uuid],
) -> CentralityMetrics {
    let degree = calculate_degree_centrality(graph, entities);
    let betweenness = calculate_betweenness_centrality(graph, entities);
    let closeness = calculate_closeness_centrality(graph, entities);
    let eigenvector = calculate_eigenvector_centrality(graph, entities);

    CentralityMetrics {
        degree_centrality: degree,
        betweenness_centrality: betweenness,
        closeness_centrality: closeness,
        eigenvector_centrality: eigenvector,
    }
}

/// Calculate degree centrality for each entity.
/// Degree centrality is the number of connections an entity has.
pub fn calculate_degree_centrality(
    graph: &RelationshipGraph,
    entities: &[Uuid],
) -> HashMap<Uuid, f64> {
    let mut centrality = HashMap::new();
    let n = entities.len() as f64;

    for &entity_id in entities {
        let degree = graph.get_all_related(entity_id).len() as f64;
        // Normalize by (n-1) to get value between 0 and 1
        centrality.insert(entity_id, if n > 1.0 { degree / (n - 1.0) } else { 0.0 });
    }

    centrality
}

/// Calculate betweenness centrality for each entity.
/// Betweenness measures how often an entity appears on shortest paths.
pub fn calculate_betweenness_centrality(
    graph: &RelationshipGraph,
    entities: &[Uuid],
) -> HashMap<Uuid, f64> {
    let mut centrality: HashMap<Uuid, f64> = entities.iter().map(|&id| (id, 0.0)).collect();
    let n = entities.len();

    if n <= 2 {
        return centrality;
    }

    // For each pair of entities, find shortest paths using BFS
    for &source in entities {
        let mut distances: HashMap<Uuid, usize> = HashMap::new();
        let mut paths_count: HashMap<Uuid, usize> = HashMap::new();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        distances.insert(source, 0);
        paths_count.insert(source, 1);
        queue.push_back(source);
        visited.insert(source);

        while let Some(current) = queue.pop_front() {
            let current_dist = distances[&current];
            let current_paths = paths_count[&current];

            for (neighbor, _) in graph.get_all_related(current) {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    distances.insert(neighbor, current_dist + 1);
                    paths_count.insert(neighbor, current_paths);
                    queue.push_back(neighbor);
                } else if distances[&neighbor] == current_dist + 1 {
                    *paths_count.entry(neighbor).or_insert(0) += current_paths;
                }
            }
        }

        // Calculate dependency scores
        for &target in entities {
            if target != source {
                if let Some(&target_paths) = paths_count.get(&target) {
                    if target_paths > 0 {
                        for &intermediate in entities {
                            if intermediate != source && intermediate != target {
                                if let Some(&int_dist) = distances.get(&intermediate) {
                                    if let Some(&int_paths) = paths_count.get(&intermediate) {
                                        // Check if intermediate is on a shortest path
                                        if let Some(&target_dist) = distances.get(&target) {
                                            if int_dist + 1 == target_dist {
                                                let contribution =
                                                    int_paths as f64 / target_paths as f64;
                                                *centrality.entry(intermediate).or_insert(0.0) +=
                                                    contribution;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Normalize
    let normalizer = ((n - 1) * (n - 2)) as f64;
    if normalizer > 0.0 {
        for value in centrality.values_mut() {
            *value /= normalizer;
        }
    }

    centrality
}

/// Calculate closeness centrality for each entity.
/// Closeness is based on the average distance to all other entities.
pub fn calculate_closeness_centrality(
    graph: &RelationshipGraph,
    entities: &[Uuid],
) -> HashMap<Uuid, f64> {
    let mut centrality = HashMap::new();

    for &entity_id in entities {
        let distances = bfs_distances(graph, entity_id, entities);
        let total_distance: usize = distances.values().sum();

        let closeness = if total_distance > 0 {
            (entities.len() - 1) as f64 / total_distance as f64
        } else {
            0.0
        };

        centrality.insert(entity_id, closeness);
    }

    centrality
}

/// Calculate eigenvector centrality using power iteration.
/// Eigenvector centrality measures influence based on connections to influential entities.
pub fn calculate_eigenvector_centrality(
    graph: &RelationshipGraph,
    entities: &[Uuid],
) -> HashMap<Uuid, f64> {
    let n = entities.len();
    let mut centrality: HashMap<Uuid, f64> = entities.iter().map(|&id| (id, 1.0)).collect();

    if n == 0 {
        return centrality;
    }

    // Power iteration
    let max_iterations = 100;
    let tolerance = 1e-6;

    for _ in 0..max_iterations {
        let mut new_centrality: HashMap<Uuid, f64> = entities.iter().map(|&id| (id, 0.0)).collect();

        // For each entity, sum the centrality of neighbors
        for &entity_id in entities {
            let neighbors = graph.get_all_related(entity_id);
            for (neighbor, _) in neighbors {
                if let Some(&neighbor_score) = centrality.get(&neighbor) {
                    *new_centrality.entry(entity_id).or_insert(0.0) += neighbor_score;
                }
            }
        }

        // Normalize
        let norm: f64 = new_centrality.values().map(|x| x * x).sum::<f64>().sqrt();
        if norm > 0.0 {
            for value in new_centrality.values_mut() {
                *value /= norm;
            }
        }

        // Check for convergence
        let diff: f64 = entities
            .iter()
            .map(|id| {
                let old = centrality.get(id).unwrap_or(&0.0);
                let new = new_centrality.get(id).unwrap_or(&0.0);
                (old - new).abs()
            })
            .sum();

        centrality = new_centrality;

        if diff < tolerance {
            break;
        }
    }

    centrality
}

/// BFS to calculate distances from a source entity to all others.
fn bfs_distances(
    graph: &RelationshipGraph,
    source: Uuid,
    entities: &[Uuid],
) -> HashMap<Uuid, usize> {
    let mut distances = HashMap::new();
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    distances.insert(source, 0);
    queue.push_back(source);
    visited.insert(source);

    while let Some(current) = queue.pop_front() {
        let current_dist = distances[&current];

        for (neighbor, _) in graph.get_all_related(current) {
            if !visited.contains(&neighbor) && entities.contains(&neighbor) {
                visited.insert(neighbor);
                distances.insert(neighbor, current_dist + 1);
                queue.push_back(neighbor);
            }
        }
    }

    distances
}

/// Simulate influence propagation through the network.
pub fn simulate_influence_propagation(
    graph: &RelationshipGraph,
    seed_entities: &[Uuid],
    config: &InfluenceConfig,
) -> HashMap<Uuid, f64> {
    let mut influence_scores: HashMap<Uuid, f64> = HashMap::new();

    // Initialize seed entities with base influence
    for &seed in seed_entities {
        influence_scores.insert(seed, config.base_influence);
    }

    // Propagate influence through the network
    for depth in 0..config.max_depth {
        let current_scores = influence_scores.clone();
        let decay = config.decay_factor.powi(depth as i32);

        for (&entity_id, &influence) in &current_scores {
            if influence * decay < config.influence_threshold {
                continue;
            }

            // Propagate to neighbors
            for (neighbor, _) in graph.get_all_related(entity_id) {
                let propagated_influence = influence * decay;
                influence_scores
                    .entry(neighbor)
                    .and_modify(|e| *e = e.max(propagated_influence))
                    .or_insert(propagated_influence);
            }
        }
    }

    influence_scores
}

/// Simulate information diffusion through the network.
pub fn simulate_diffusion(
    graph: &RelationshipGraph,
    seed_entities: &[Uuid],
    model: DiffusionModel,
    max_iterations: usize,
) -> DiffusionResult {
    let mut informed = HashSet::new();
    let mut cascade_history = Vec::new();
    let mut iterations = 0;

    // Initialize with seed entities
    for &seed in seed_entities {
        informed.insert(seed);
    }
    cascade_history.push(informed.len());

    // Simulate diffusion
    for _ in 0..max_iterations {
        iterations += 1;
        let current_informed: Vec<Uuid> = informed.iter().copied().collect();
        let mut newly_informed = HashSet::new();

        for &entity_id in &current_informed {
            for (neighbor, _) in graph.get_all_related(entity_id) {
                if !informed.contains(&neighbor) {
                    let should_inform = match model {
                        DiffusionModel::SimpleContagion => true,
                        DiffusionModel::ComplexContagion { threshold } => {
                            // Count informed neighbors
                            let neighbor_neighbors = graph.get_all_related(neighbor);
                            let informed_neighbors = neighbor_neighbors
                                .iter()
                                .filter(|(n, _)| informed.contains(n))
                                .count();
                            let total_neighbors = neighbor_neighbors.len();
                            if total_neighbors > 0 {
                                informed_neighbors as f64 / total_neighbors as f64 >= threshold
                            } else {
                                false
                            }
                        }
                        DiffusionModel::LinearThreshold { threshold } => {
                            // Similar to complex contagion
                            let neighbor_neighbors = graph.get_all_related(neighbor);
                            let informed_neighbors = neighbor_neighbors
                                .iter()
                                .filter(|(n, _)| informed.contains(n))
                                .count();
                            let total_neighbors = neighbor_neighbors.len();
                            if total_neighbors > 0 {
                                informed_neighbors as f64 / total_neighbors as f64 >= threshold
                            } else {
                                false
                            }
                        }
                        DiffusionModel::IndependentCascade { probability } => {
                            hash_based_probability(&neighbor) < probability
                        }
                    };

                    if should_inform {
                        newly_informed.insert(neighbor);
                    }
                }
            }
        }

        // No new entities informed, cascade has stopped
        if newly_informed.is_empty() {
            break;
        }

        // Update informed set
        for entity in newly_informed {
            informed.insert(entity);
        }

        cascade_history.push(informed.len());
    }

    DiffusionResult {
        informed_entities: informed.into_iter().collect(),
        cascade_size: cascade_history.last().copied().unwrap_or(0),
        iterations,
        cascade_history,
    }
}

/// Community detection result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityDetectionResult {
    /// Community assignments (entity_id -> community_id).
    pub communities: HashMap<Uuid, usize>,
    /// Number of communities detected.
    pub num_communities: usize,
    /// Modularity score (quality metric).
    pub modularity: f64,
}

/// Detects communities using label propagation algorithm.
/// This is a fast, semi-supervised algorithm for community detection.
pub fn detect_communities_label_propagation(
    graph: &RelationshipGraph,
    entities: &[Uuid],
    max_iterations: usize,
) -> CommunityDetectionResult {
    let mut communities: HashMap<Uuid, usize> = HashMap::new();

    // Initialize each entity to its own community
    for (idx, entity) in entities.iter().enumerate() {
        communities.insert(*entity, idx);
    }

    // Label propagation: update each node's label to match majority of neighbors
    for _iteration in 0..max_iterations {
        let mut changed = false;

        for entity in entities {
            // Get all neighbors from the relationship graph
            let neighbors: Vec<Uuid> = graph
                .get_all_related(*entity)
                .into_iter()
                .map(|(id, _)| id)
                .collect();

            if neighbors.is_empty() {
                continue;
            }

            // Count labels of neighbors
            let mut label_counts: HashMap<usize, usize> = HashMap::new();
            for neighbor in &neighbors {
                if let Some(&label) = communities.get(neighbor) {
                    *label_counts.entry(label).or_insert(0) += 1;
                }
            }

            // Find most common label
            if let Some((&most_common_label, _)) =
                label_counts.iter().max_by_key(|&(_, &count)| count)
            {
                if communities[entity] != most_common_label {
                    communities.insert(*entity, most_common_label);
                    changed = true;
                }
            }
        }

        if !changed {
            break; // Convergence reached
        }
    }

    // Renumber communities to be consecutive
    let mut label_map: HashMap<usize, usize> = HashMap::new();
    let mut next_id = 0;

    for community_id in communities.values() {
        if !label_map.contains_key(community_id) {
            label_map.insert(*community_id, next_id);
            next_id += 1;
        }
    }

    for community_id in communities.values_mut() {
        *community_id = label_map[community_id];
    }

    let num_communities = label_map.len();
    let modularity = calculate_modularity(graph, entities, &communities);

    CommunityDetectionResult {
        communities,
        num_communities,
        modularity,
    }
}

/// Detects communities using simple connected components.
/// Treats each connected component as a separate community.
pub fn detect_communities_connected_components(
    graph: &RelationshipGraph,
    entities: &[Uuid],
) -> CommunityDetectionResult {
    let mut communities: HashMap<Uuid, usize> = HashMap::new();
    let mut visited: HashSet<Uuid> = HashSet::new();
    let mut community_id = 0;

    for entity in entities {
        if visited.contains(entity) {
            continue;
        }

        // BFS to find all connected entities
        let mut queue: VecDeque<Uuid> = VecDeque::new();
        queue.push_back(*entity);
        visited.insert(*entity);

        while let Some(current) = queue.pop_front() {
            communities.insert(current, community_id);

            let neighbors: Vec<Uuid> = graph
                .get_all_related(current)
                .into_iter()
                .map(|(id, _)| id)
                .collect();
            for neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back(neighbor);
                }
            }
        }

        community_id += 1;
    }

    let num_communities = community_id;
    let modularity = calculate_modularity(graph, entities, &communities);

    CommunityDetectionResult {
        communities,
        num_communities,
        modularity,
    }
}

/// Calculates modularity score for community structure.
/// Modularity ranges from -1 to 1, with higher values indicating stronger community structure.
fn calculate_modularity(
    graph: &RelationshipGraph,
    entities: &[Uuid],
    communities: &HashMap<Uuid, usize>,
) -> f64 {
    let m = count_total_edges(graph, entities) as f64;
    if m == 0.0 {
        return 0.0;
    }

    let mut modularity = 0.0;

    for i in entities {
        for j in entities {
            if i == j {
                continue;
            }

            // Check if in same community
            let same_community = communities.get(i) == communities.get(j);
            if !same_community {
                continue;
            }

            // A_ij: 1 if edge exists, 0 otherwise
            let neighbors_i: Vec<Uuid> = graph
                .get_all_related(*i)
                .into_iter()
                .map(|(id, _)| id)
                .collect();
            let a_ij = if neighbors_i.contains(j) { 1.0 } else { 0.0 };

            // k_i and k_j: degrees
            let k_i = neighbors_i.len() as f64;
            let k_j = graph.get_all_related(*j).len() as f64;

            // Expected number of edges
            let expected = (k_i * k_j) / (2.0 * m);

            modularity += a_ij - expected;
        }
    }

    modularity / (2.0 * m)
}

/// Counts total number of edges in the graph for given entities.
fn count_total_edges(graph: &RelationshipGraph, entities: &[Uuid]) -> usize {
    let entity_set: HashSet<Uuid> = entities.iter().copied().collect();
    let mut count = 0;

    for entity in entities {
        let neighbors: Vec<Uuid> = graph
            .get_all_related(*entity)
            .into_iter()
            .map(|(id, _)| id)
            .collect();
        for neighbor in neighbors {
            if entity_set.contains(&neighbor) {
                count += 1;
            }
        }
    }

    count / 2 // Each edge counted twice
}

/// Community statistics for analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityStats {
    /// Average community size.
    pub avg_size: f64,
    /// Largest community size.
    pub max_size: usize,
    /// Smallest community size.
    pub min_size: usize,
    /// Size distribution (community_id -> size).
    pub size_distribution: HashMap<usize, usize>,
}

impl CommunityStats {
    /// Calculates community statistics from detection result.
    pub fn from_result(result: &CommunityDetectionResult) -> Self {
        let mut size_distribution: HashMap<usize, usize> = HashMap::new();

        for &community_id in result.communities.values() {
            *size_distribution.entry(community_id).or_insert(0) += 1;
        }

        let sizes: Vec<usize> = size_distribution.values().copied().collect();
        let max_size = sizes.iter().copied().max().unwrap_or(0);
        let min_size = sizes.iter().copied().min().unwrap_or(0);
        let avg_size = if !sizes.is_empty() {
            sizes.iter().sum::<usize>() as f64 / sizes.len() as f64
        } else {
            0.0
        };

        Self {
            avg_size,
            max_size,
            min_size,
            size_distribution,
        }
    }
}

/// PageRank result for identifying influential entities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageRankResult {
    /// PageRank scores for each entity.
    pub scores: HashMap<Uuid, f64>,
    /// Number of iterations performed.
    pub iterations: usize,
    /// Whether the algorithm converged.
    pub converged: bool,
}

impl PageRankResult {
    /// Gets the top N entities by PageRank score.
    pub fn top_entities(&self, n: usize) -> Vec<(Uuid, f64)> {
        let mut sorted: Vec<_> = self
            .scores
            .iter()
            .map(|(&id, &score)| (id, score))
            .collect();
        sorted.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
        sorted.into_iter().take(n).collect()
    }

    /// Gets entities with scores above a threshold.
    pub fn above_threshold(&self, threshold: f64) -> Vec<(Uuid, f64)> {
        self.scores
            .iter()
            .filter(|&(_, &score)| score >= threshold)
            .map(|(&id, &score)| (id, score))
            .collect()
    }
}

/// Calculates PageRank scores for entities in the network.
/// PageRank identifies the most influential nodes based on network structure.
pub fn calculate_pagerank(
    graph: &RelationshipGraph,
    entities: &[Uuid],
    damping_factor: f64,
    max_iterations: usize,
    convergence_threshold: f64,
) -> PageRankResult {
    let n = entities.len() as f64;
    if n == 0.0 {
        return PageRankResult {
            scores: HashMap::new(),
            iterations: 0,
            converged: true,
        };
    }

    // Initialize all scores to 1/N
    let mut scores: HashMap<Uuid, f64> = entities.iter().map(|&id| (id, 1.0 / n)).collect();
    let mut new_scores: HashMap<Uuid, f64> = HashMap::new();

    let entity_set: HashSet<Uuid> = entities.iter().copied().collect();
    let mut converged = false;

    let mut iterations = 0;
    for iteration in 0..max_iterations {
        iterations = iteration + 1;
        let mut max_diff: f64 = 0.0;

        // Calculate new scores
        for entity in entities {
            // Get all entities that link to this entity (incoming links)
            let incoming = entities
                .iter()
                .filter(|&&other| {
                    if other == *entity {
                        return false;
                    }
                    let neighbors: Vec<Uuid> = graph
                        .get_all_related(other)
                        .into_iter()
                        .map(|(id, _)| id)
                        .collect();
                    neighbors.contains(entity)
                })
                .collect::<Vec<_>>();

            // Calculate new score
            let mut new_score = (1.0 - damping_factor) / n;
            for &&incoming_entity in &incoming {
                let outgoing_count = graph
                    .get_all_related(incoming_entity)
                    .into_iter()
                    .filter(|(id, _)| entity_set.contains(id))
                    .count() as f64;

                if outgoing_count > 0.0 {
                    new_score += damping_factor * scores[&incoming_entity] / outgoing_count;
                }
            }

            new_scores.insert(*entity, new_score);

            // Track maximum change for convergence
            let diff = (new_score - scores[entity]).abs();
            max_diff = max_diff.max(diff);
        }

        // Update scores
        scores = new_scores.clone();

        // Check for convergence
        if max_diff < convergence_threshold {
            converged = true;
            break;
        }
    }

    PageRankResult {
        scores,
        iterations,
        converged,
    }
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
        let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();

        let p1 = hash_based_probability(&id1);
        let p2 = hash_based_probability(&id2);
        let p3 = hash_based_probability(&id1);

        // Should be deterministic
        assert_eq!(p1, p3);
        // Should be different for different inputs (unless by chance they're equal)
        // Should be in valid range
        assert!((0.0..=1.0).contains(&p1));
        assert!((0.0..=1.0).contains(&p2));
    }

    #[test]
    fn test_diffusion_result_creation() {
        let id_a = Uuid::new_v4();
        let id_b = Uuid::new_v4();

        let result = DiffusionResult {
            informed_entities: vec![id_a, id_b],
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
        let node_id = Uuid::new_v4();
        let mut degree = HashMap::new();
        degree.insert(node_id, 0.5);

        let metrics = CentralityMetrics {
            degree_centrality: degree.clone(),
            betweenness_centrality: degree.clone(),
            closeness_centrality: degree.clone(),
            eigenvector_centrality: degree,
        };

        assert!(metrics.degree_centrality.contains_key(&node_id));
    }

    #[test]
    fn test_degree_centrality() {
        use crate::relationships::{Relationship, RelationshipType};

        let mut graph = RelationshipGraph::new();
        let n1 = Uuid::new_v4();
        let n2 = Uuid::new_v4();
        let n3 = Uuid::new_v4();

        graph.add_relationship(Relationship::new(n1, n2, RelationshipType::Parent));
        graph.add_relationship(Relationship::new(n1, n3, RelationshipType::Parent));

        let entities = vec![n1, n2, n3];
        let centrality = calculate_degree_centrality(&graph, &entities);

        // n1 has 2 connections out of 2 possible (n-1), so centrality = 1.0
        assert!((centrality[&n1] - 1.0).abs() < 1e-6);
        // n2 and n3 have 0 outgoing connections (Parent doesn't create inverse edge)
        assert!((centrality[&n2] - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_closeness_centrality() {
        use crate::relationships::{Relationship, RelationshipType};

        let mut graph = RelationshipGraph::new();
        let n1 = Uuid::new_v4();
        let n2 = Uuid::new_v4();
        let n3 = Uuid::new_v4();

        graph.add_relationship(Relationship::new(n1, n2, RelationshipType::Parent));
        graph.add_relationship(Relationship::new(n2, n3, RelationshipType::Parent));

        let entities = vec![n1, n2, n3];
        let centrality = calculate_closeness_centrality(&graph, &entities);

        // All entities should have some closeness value
        assert!(centrality.contains_key(&n1));
        assert!(centrality.contains_key(&n2));
        assert!(centrality.contains_key(&n3));
    }

    #[test]
    fn test_betweenness_centrality() {
        use crate::relationships::{Relationship, RelationshipType};

        let mut graph = RelationshipGraph::new();
        let n1 = Uuid::new_v4();
        let n2 = Uuid::new_v4();
        let n3 = Uuid::new_v4();

        graph.add_relationship(Relationship::new(n1, n2, RelationshipType::Parent));
        graph.add_relationship(Relationship::new(n2, n3, RelationshipType::Parent));

        let entities = vec![n1, n2, n3];
        let centrality = calculate_betweenness_centrality(&graph, &entities);

        // n2 is on the path from n1 to n3, so it should have higher betweenness
        assert!(centrality[&n2] >= centrality[&n1]);
        assert!(centrality[&n2] >= centrality[&n3]);
    }

    #[test]
    fn test_eigenvector_centrality() {
        use crate::relationships::{Relationship, RelationshipType};

        let mut graph = RelationshipGraph::new();
        let n1 = Uuid::new_v4();
        let n2 = Uuid::new_v4();
        let n3 = Uuid::new_v4();

        // Create a simple network
        graph.add_relationship(Relationship::new(n1, n2, RelationshipType::Spouse));
        graph.add_relationship(Relationship::new(n2, n3, RelationshipType::Spouse));

        let entities = vec![n1, n2, n3];
        let centrality = calculate_eigenvector_centrality(&graph, &entities);

        // All should have some value
        assert!(centrality.contains_key(&n1));
        assert!(centrality.contains_key(&n2));
        assert!(centrality.contains_key(&n3));
    }

    #[test]
    fn test_calculate_all_centrality_metrics() {
        use crate::relationships::{Relationship, RelationshipType};

        let mut graph = RelationshipGraph::new();
        let n1 = Uuid::new_v4();
        let n2 = Uuid::new_v4();
        let n3 = Uuid::new_v4();

        graph.add_relationship(Relationship::new(n1, n2, RelationshipType::Parent));
        graph.add_relationship(Relationship::new(n1, n3, RelationshipType::Parent));

        let entities = vec![n1, n2, n3];
        let metrics = calculate_centrality_metrics(&graph, &entities);

        assert_eq!(metrics.degree_centrality.len(), 3);
        assert_eq!(metrics.betweenness_centrality.len(), 3);
        assert_eq!(metrics.closeness_centrality.len(), 3);
        assert_eq!(metrics.eigenvector_centrality.len(), 3);
    }

    #[test]
    fn test_influence_propagation() {
        use crate::relationships::{Relationship, RelationshipType};

        let mut graph = RelationshipGraph::new();
        let n1 = Uuid::new_v4();
        let n2 = Uuid::new_v4();
        let n3 = Uuid::new_v4();

        graph.add_relationship(Relationship::new(n1, n2, RelationshipType::Parent));
        graph.add_relationship(Relationship::new(n2, n3, RelationshipType::Parent));

        let config = InfluenceConfig::default();
        let influence = simulate_influence_propagation(&graph, &[n1], &config);

        // n1 should have influence
        assert!(influence.contains_key(&n1));
        // Influence should propagate to n2
        assert!(influence.contains_key(&n2));
    }

    #[test]
    fn test_simple_contagion_diffusion() {
        use crate::relationships::{Relationship, RelationshipType};

        let mut graph = RelationshipGraph::new();
        let n1 = Uuid::new_v4();
        let n2 = Uuid::new_v4();
        let n3 = Uuid::new_v4();

        // Create a chain: n1 -> n2 -> n3
        graph.add_relationship(Relationship::new(n1, n2, RelationshipType::Parent));
        graph.add_relationship(Relationship::new(n2, n3, RelationshipType::Parent));

        let result = simulate_diffusion(&graph, &[n1], DiffusionModel::SimpleContagion, 10);

        // Should inform all connected entities
        assert!(result.informed_entities.contains(&n1));
        assert!(result.informed_entities.contains(&n2));
        assert!(result.informed_entities.contains(&n3));
        assert_eq!(result.cascade_size, 3);
    }

    #[test]
    fn test_complex_contagion_diffusion() {
        use crate::relationships::{Relationship, RelationshipType};

        let mut graph = RelationshipGraph::new();
        let n1 = Uuid::new_v4();
        let n2 = Uuid::new_v4();
        let n3 = Uuid::new_v4();

        // Create a network where n3 has two neighbors
        graph.add_relationship(Relationship::new(n1, n3, RelationshipType::Parent));
        graph.add_relationship(Relationship::new(n2, n3, RelationshipType::Parent));

        // With high threshold, n3 needs multiple informed neighbors
        let result = simulate_diffusion(
            &graph,
            &[n1],
            DiffusionModel::ComplexContagion { threshold: 0.6 },
            10,
        );

        // n1 is initially informed
        assert!(result.informed_entities.contains(&n1));
        // n3 won't be informed because only 1 of 2 neighbors is informed (50% < 60% threshold)
        assert!(!result.informed_entities.contains(&n3));
    }

    #[test]
    fn test_independent_cascade_diffusion() {
        use crate::relationships::{Relationship, RelationshipType};

        let mut graph = RelationshipGraph::new();
        let n1 = Uuid::new_v4();
        let n2 = Uuid::new_v4();

        graph.add_relationship(Relationship::new(n1, n2, RelationshipType::Parent));

        // With probability 0.0, nothing should propagate
        let result = simulate_diffusion(
            &graph,
            &[n1],
            DiffusionModel::IndependentCascade { probability: 0.0 },
            10,
        );

        assert_eq!(result.cascade_size, 1); // Only seed entity
        assert!(result.informed_entities.contains(&n1));
    }

    #[test]
    fn test_community_detection_connected_components() {
        use crate::relationships::{Relationship, RelationshipType};

        let mut graph = RelationshipGraph::new();
        let n1 = Uuid::new_v4();
        let n2 = Uuid::new_v4();
        let n3 = Uuid::new_v4();
        let n4 = Uuid::new_v4();

        // Create two separate components
        graph.add_relationship(Relationship::new(n1, n2, RelationshipType::Spouse));
        graph.add_relationship(Relationship::new(n3, n4, RelationshipType::Spouse));

        let entities = vec![n1, n2, n3, n4];
        let result = detect_communities_connected_components(&graph, &entities);

        // Should detect 2 communities
        assert_eq!(result.num_communities, 2);

        // n1 and n2 should be in the same community
        assert_eq!(result.communities[&n1], result.communities[&n2]);

        // n3 and n4 should be in the same community
        assert_eq!(result.communities[&n3], result.communities[&n4]);

        // But n1 and n3 should be in different communities
        assert_ne!(result.communities[&n1], result.communities[&n3]);
    }

    #[test]
    fn test_community_detection_label_propagation() {
        use crate::relationships::{Relationship, RelationshipType};

        let mut graph = RelationshipGraph::new();
        let n1 = Uuid::new_v4();
        let n2 = Uuid::new_v4();
        let n3 = Uuid::new_v4();

        // Create a simple network
        graph.add_relationship(Relationship::new(n1, n2, RelationshipType::Spouse));
        graph.add_relationship(Relationship::new(n2, n3, RelationshipType::Spouse));

        let entities = vec![n1, n2, n3];
        let result = detect_communities_label_propagation(&graph, &entities, 10);

        // Should converge to some community structure
        assert!(result.num_communities > 0);
        assert!(result.num_communities <= 3);
    }

    #[test]
    fn test_community_stats() {
        let mut communities = HashMap::new();
        let n1 = Uuid::new_v4();
        let n2 = Uuid::new_v4();
        let n3 = Uuid::new_v4();
        let n4 = Uuid::new_v4();

        communities.insert(n1, 0);
        communities.insert(n2, 0);
        communities.insert(n3, 1);
        communities.insert(n4, 1);

        let result = CommunityDetectionResult {
            communities,
            num_communities: 2,
            modularity: 0.5,
        };

        let stats = CommunityStats::from_result(&result);

        assert_eq!(stats.avg_size, 2.0);
        assert_eq!(stats.max_size, 2);
        assert_eq!(stats.min_size, 2);
        assert_eq!(stats.size_distribution.len(), 2);
    }

    #[test]
    fn test_community_detection_single_node() {
        let graph = RelationshipGraph::new();
        let n1 = Uuid::new_v4();

        let entities = vec![n1];
        let result = detect_communities_connected_components(&graph, &entities);

        // Single node should form one community
        assert_eq!(result.num_communities, 1);
        assert_eq!(result.communities[&n1], 0);
    }

    #[test]
    fn test_modularity_calculation() {
        use crate::relationships::{Relationship, RelationshipType};

        let mut graph = RelationshipGraph::new();
        let n1 = Uuid::new_v4();
        let n2 = Uuid::new_v4();
        let n3 = Uuid::new_v4();
        let n4 = Uuid::new_v4();

        // Create two tightly connected communities
        graph.add_relationship(Relationship::new(n1, n2, RelationshipType::Spouse));
        graph.add_relationship(Relationship::new(n3, n4, RelationshipType::Spouse));

        let entities = vec![n1, n2, n3, n4];
        let mut communities = HashMap::new();
        communities.insert(n1, 0);
        communities.insert(n2, 0);
        communities.insert(n3, 1);
        communities.insert(n4, 1);

        let modularity = calculate_modularity(&graph, &entities, &communities);

        // Modularity should be positive for good community structure
        // (though the value depends on the specific graph structure)
        assert!((-1.0..=1.0).contains(&modularity));
    }

    #[test]
    fn test_pagerank_basic() {
        use crate::relationships::{Relationship, RelationshipType};

        let mut graph = RelationshipGraph::new();
        let n1 = Uuid::new_v4();
        let n2 = Uuid::new_v4();
        let n3 = Uuid::new_v4();

        // Create a simple network where n2 is pointed to by both n1 and n3
        graph.add_relationship(Relationship::new(n1, n2, RelationshipType::Parent));
        graph.add_relationship(Relationship::new(n3, n2, RelationshipType::Parent));

        let entities = vec![n1, n2, n3];
        let result = calculate_pagerank(&graph, &entities, 0.85, 100, 0.0001);

        // n2 should have highest PageRank (it has 2 incoming links)
        assert!(result.scores[&n2] > result.scores[&n1]);
        assert!(result.scores[&n2] > result.scores[&n3]);
        assert!(result.converged);
    }

    #[test]
    fn test_pagerank_convergence() {
        use crate::relationships::{Relationship, RelationshipType};

        let mut graph = RelationshipGraph::new();
        let n1 = Uuid::new_v4();
        let n2 = Uuid::new_v4();

        graph.add_relationship(Relationship::new(n1, n2, RelationshipType::Spouse));

        let entities = vec![n1, n2];
        let result = calculate_pagerank(&graph, &entities, 0.85, 100, 0.0001);

        assert!(result.iterations > 0);
        assert!(result.converged);

        // Scores should sum to approximately 1.0
        let sum: f64 = result.scores.values().sum();
        assert!((sum - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_pagerank_empty() {
        let graph = RelationshipGraph::new();
        let entities: Vec<Uuid> = vec![];

        let result = calculate_pagerank(&graph, &entities, 0.85, 100, 0.0001);

        assert_eq!(result.scores.len(), 0);
        assert_eq!(result.iterations, 0);
        assert!(result.converged);
    }

    #[test]
    fn test_pagerank_top_entities() {
        let mut scores = HashMap::new();
        let n1 = Uuid::new_v4();
        let n2 = Uuid::new_v4();
        let n3 = Uuid::new_v4();

        scores.insert(n1, 0.5);
        scores.insert(n2, 0.3);
        scores.insert(n3, 0.2);

        let result = PageRankResult {
            scores,
            iterations: 10,
            converged: true,
        };

        let top2 = result.top_entities(2);
        assert_eq!(top2.len(), 2);
        assert_eq!(top2[0].0, n1); // Highest score
        assert_eq!(top2[1].0, n2); // Second highest
    }

    #[test]
    fn test_pagerank_above_threshold() {
        let mut scores = HashMap::new();
        let n1 = Uuid::new_v4();
        let n2 = Uuid::new_v4();
        let n3 = Uuid::new_v4();

        scores.insert(n1, 0.5);
        scores.insert(n2, 0.3);
        scores.insert(n3, 0.1);

        let result = PageRankResult {
            scores,
            iterations: 10,
            converged: true,
        };

        let above_threshold = result.above_threshold(0.25);
        assert_eq!(above_threshold.len(), 2); // n1 and n2
    }
}
