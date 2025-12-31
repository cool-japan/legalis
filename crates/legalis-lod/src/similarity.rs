//! Semantic similarity indexing for efficient similarity search.
//!
//! This module provides indexing structures for fast approximate nearest neighbor
//! search based on entity embeddings.

use crate::embeddings::{Embedding, cosine_similarity, euclidean_distance};
use std::collections::HashMap;

/// Similarity metric for comparing entities.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimilarityMetric {
    /// Cosine similarity (angle between vectors)
    Cosine,
    /// Euclidean distance (L2 norm)
    Euclidean,
    /// Manhattan distance (L1 norm)
    Manhattan,
}

/// Locality-Sensitive Hashing (LSH) index for approximate nearest neighbor search.
pub struct LSHIndex {
    /// Number of hash tables
    num_tables: usize,
    /// Number of hash functions per table
    #[allow(dead_code)]
    num_hashes: usize,
    /// Dimension of embeddings
    dimension: usize,
    /// Random projection vectors for each hash table
    projection_vectors: Vec<Vec<Vec<f64>>>,
    /// Hash tables mapping hash values to entity IDs
    hash_tables: Vec<HashMap<Vec<i32>, Vec<String>>>,
    /// Entity embeddings
    embeddings: HashMap<String, Embedding>,
}

impl LSHIndex {
    /// Creates a new LSH index.
    pub fn new(num_tables: usize, num_hashes: usize, dimension: usize) -> Self {
        let mut projection_vectors = Vec::new();
        use rand::Rng;
        let mut rng = rand::rng();

        // Generate random projection vectors for each table
        for _ in 0..num_tables {
            let mut table_vectors = Vec::new();
            for _ in 0..num_hashes {
                let vec: Vec<f64> = (0..dimension)
                    .map(|_| rng.random_range(-1.0..1.0))
                    .collect();
                table_vectors.push(vec);
            }
            projection_vectors.push(table_vectors);
        }

        Self {
            num_tables,
            num_hashes,
            dimension,
            projection_vectors,
            hash_tables: vec![HashMap::new(); num_tables],
            embeddings: HashMap::new(),
        }
    }

    /// Adds an entity with its embedding to the index.
    pub fn add(&mut self, entity: String, embedding: Embedding) {
        if embedding.len() != self.dimension {
            return;
        }

        // Compute all hashes first
        let hashes: Vec<Vec<i32>> = (0..self.num_tables)
            .map(|table_idx| self.hash_vector(&embedding, table_idx))
            .collect();

        // Insert into hash tables
        for (table_idx, hash) in hashes.into_iter().enumerate() {
            self.hash_tables[table_idx]
                .entry(hash)
                .or_default()
                .push(entity.clone());
        }

        self.embeddings.insert(entity, embedding);
    }

    /// Queries for similar entities using approximate nearest neighbor search.
    pub fn query(
        &self,
        embedding: &Embedding,
        k: usize,
        metric: SimilarityMetric,
    ) -> Vec<(String, f64)> {
        if embedding.len() != self.dimension {
            return Vec::new();
        }

        // Collect candidates from all hash tables
        let mut candidates = std::collections::HashSet::new();
        for (table_idx, table) in self.hash_tables.iter().enumerate() {
            let hash = self.hash_vector(embedding, table_idx);
            if let Some(entities) = table.get(&hash) {
                for entity in entities {
                    candidates.insert(entity.clone());
                }
            }
        }

        // Calculate exact similarity for candidates
        let mut similarities: Vec<(String, f64)> = candidates
            .into_iter()
            .filter_map(|entity| {
                let emb = self.embeddings.get(&entity)?;
                let score = match metric {
                    SimilarityMetric::Cosine => cosine_similarity(embedding, emb),
                    SimilarityMetric::Euclidean => -euclidean_distance(embedding, emb), // Negate for sorting
                    SimilarityMetric::Manhattan => -manhattan_distance(embedding, emb),
                };
                Some((entity, score))
            })
            .collect();

        // Sort by similarity (descending)
        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        similarities.into_iter().take(k).collect()
    }

    /// Hashes a vector using the projection vectors for a specific table.
    fn hash_vector(&self, vec: &[f64], table_idx: usize) -> Vec<i32> {
        self.projection_vectors[table_idx]
            .iter()
            .map(|proj| {
                let dot_product: f64 = vec.iter().zip(proj.iter()).map(|(a, b)| a * b).sum();
                if dot_product >= 0.0 { 1 } else { 0 }
            })
            .collect()
    }

    /// Returns the number of indexed entities.
    pub fn size(&self) -> usize {
        self.embeddings.len()
    }

    /// Clears the index.
    pub fn clear(&mut self) {
        for table in &mut self.hash_tables {
            table.clear();
        }
        self.embeddings.clear();
    }
}

/// Simple inverted index for exact similarity search.
pub struct SimilarityIndex {
    embeddings: HashMap<String, Embedding>,
    metric: SimilarityMetric,
}

impl SimilarityIndex {
    /// Creates a new similarity index.
    pub fn new(metric: SimilarityMetric) -> Self {
        Self {
            embeddings: HashMap::new(),
            metric,
        }
    }

    /// Adds an entity with its embedding.
    pub fn add(&mut self, entity: String, embedding: Embedding) {
        self.embeddings.insert(entity, embedding);
    }

    /// Queries for the k most similar entities.
    pub fn query(&self, embedding: &Embedding, k: usize) -> Vec<(String, f64)> {
        let mut similarities: Vec<(String, f64)> = self
            .embeddings
            .iter()
            .map(|(entity, emb)| {
                let score = match self.metric {
                    SimilarityMetric::Cosine => cosine_similarity(embedding, emb),
                    SimilarityMetric::Euclidean => -euclidean_distance(embedding, emb),
                    SimilarityMetric::Manhattan => -manhattan_distance(embedding, emb),
                };
                (entity.clone(), score)
            })
            .collect();

        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        similarities.into_iter().take(k).collect()
    }

    /// Returns the number of indexed entities.
    pub fn size(&self) -> usize {
        self.embeddings.len()
    }

    /// Gets an embedding for an entity.
    pub fn get(&self, entity: &str) -> Option<&Embedding> {
        self.embeddings.get(entity)
    }

    /// Removes an entity from the index.
    pub fn remove(&mut self, entity: &str) -> Option<Embedding> {
        self.embeddings.remove(entity)
    }

    /// Clears the index.
    pub fn clear(&mut self) {
        self.embeddings.clear();
    }
}

/// Hierarchical Navigable Small World (HNSW) index for fast approximate search.
/// This is a simplified implementation for demonstration.
pub struct HNSWIndex {
    embeddings: HashMap<String, Embedding>,
    graph: HashMap<String, Vec<String>>,
    max_connections: usize,
    metric: SimilarityMetric,
}

impl HNSWIndex {
    /// Creates a new HNSW index.
    pub fn new(max_connections: usize, metric: SimilarityMetric) -> Self {
        Self {
            embeddings: HashMap::new(),
            graph: HashMap::new(),
            max_connections,
            metric,
        }
    }

    /// Adds an entity to the index.
    pub fn add(&mut self, entity: String, embedding: Embedding) {
        // Find nearest neighbors
        let neighbors = self.find_nearest(&embedding, self.max_connections);

        // Add bidirectional connections
        let mut connections = Vec::new();
        for (neighbor, _) in neighbors {
            connections.push(neighbor.clone());
            self.graph
                .entry(neighbor.clone())
                .or_default()
                .push(entity.clone());

            // Prune if necessary
            let neighbor_connections = self.graph.get_mut(&neighbor).unwrap();
            if neighbor_connections.len() > self.max_connections {
                self.prune_connections(&neighbor);
            }
        }

        self.graph.insert(entity.clone(), connections);
        self.embeddings.insert(entity, embedding);
    }

    /// Queries for k nearest neighbors using graph search.
    pub fn query(&self, embedding: &Embedding, k: usize) -> Vec<(String, f64)> {
        if self.embeddings.is_empty() {
            return Vec::new();
        }

        // Start from a random entry point
        let entry_point = self.embeddings.keys().next().unwrap().clone();

        // Greedy search
        let mut visited = std::collections::HashSet::new();
        let mut candidates = std::collections::BinaryHeap::new();
        let mut results = std::collections::BinaryHeap::new();

        let entry_score = self.compute_distance(embedding, &entry_point);
        candidates.push(std::cmp::Reverse((
            ordered_float::OrderedFloat(entry_score),
            entry_point.clone(),
        )));
        results.push((
            ordered_float::OrderedFloat(entry_score),
            entry_point.clone(),
        ));
        visited.insert(entry_point.clone());

        while let Some(std::cmp::Reverse((dist, current))) = candidates.pop() {
            if results.len() >= k {
                let worst = results.peek().unwrap();
                if dist.0 > worst.0.0 {
                    break;
                }
            }

            if let Some(neighbors) = self.graph.get(&current) {
                for neighbor in neighbors {
                    if visited.insert(neighbor.clone()) {
                        let neighbor_dist = self.compute_distance(embedding, neighbor);
                        candidates.push(std::cmp::Reverse((
                            ordered_float::OrderedFloat(neighbor_dist),
                            neighbor.clone(),
                        )));
                        results
                            .push((ordered_float::OrderedFloat(neighbor_dist), neighbor.clone()));
                    }
                }
            }
        }

        // Convert to sorted vector
        let mut result_vec: Vec<(String, f64)> = results
            .into_iter()
            .map(|(dist, entity)| (entity, -dist.0))
            .collect();
        result_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        result_vec.into_iter().take(k).collect()
    }

    /// Returns the number of indexed entities.
    pub fn size(&self) -> usize {
        self.embeddings.len()
    }

    fn find_nearest(&self, embedding: &Embedding, k: usize) -> Vec<(String, f64)> {
        let mut distances: Vec<(String, f64)> = self
            .embeddings
            .iter()
            .map(|(entity, emb)| {
                let dist = match self.metric {
                    SimilarityMetric::Cosine => -cosine_similarity(embedding, emb),
                    SimilarityMetric::Euclidean => euclidean_distance(embedding, emb),
                    SimilarityMetric::Manhattan => manhattan_distance(embedding, emb),
                };
                (entity.clone(), dist)
            })
            .collect();

        distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        distances.into_iter().take(k).collect()
    }

    fn compute_distance(&self, embedding: &Embedding, entity: &str) -> f64 {
        if let Some(emb) = self.embeddings.get(entity) {
            match self.metric {
                SimilarityMetric::Cosine => -cosine_similarity(embedding, emb),
                SimilarityMetric::Euclidean => euclidean_distance(embedding, emb),
                SimilarityMetric::Manhattan => manhattan_distance(embedding, emb),
            }
        } else {
            f64::INFINITY
        }
    }

    fn prune_connections(&mut self, entity: &str) {
        if let Some(connections) = self.graph.get(entity) {
            if connections.len() <= self.max_connections {
                return;
            }

            let entity_emb = self.embeddings.get(entity).unwrap();
            let mut conn_with_dist: Vec<(String, f64)> = connections
                .iter()
                .map(|conn| {
                    let dist = self.compute_distance(entity_emb, conn);
                    (conn.clone(), dist)
                })
                .collect();

            conn_with_dist.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            let pruned: Vec<String> = conn_with_dist
                .into_iter()
                .take(self.max_connections)
                .map(|(e, _)| e)
                .collect();

            self.graph.insert(entity.to_string(), pruned);
        }
    }
}

/// Calculates Manhattan distance between two vectors.
fn manhattan_distance(a: &[f64], b: &[f64]) -> f64 {
    if a.len() != b.len() {
        return f64::INFINITY;
    }
    a.iter().zip(b.iter()).map(|(x, y)| (x - y).abs()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_embeddings() -> Vec<(String, Embedding)> {
        vec![
            ("a".to_string(), vec![1.0, 0.0, 0.0]),
            ("b".to_string(), vec![0.9, 0.1, 0.0]),
            ("c".to_string(), vec![0.0, 1.0, 0.0]),
            ("d".to_string(), vec![0.0, 0.0, 1.0]),
        ]
    }

    #[test]
    fn test_similarity_index_add() {
        let mut index = SimilarityIndex::new(SimilarityMetric::Cosine);
        index.add("test".to_string(), vec![1.0, 0.0, 0.0]);
        assert_eq!(index.size(), 1);
    }

    #[test]
    fn test_similarity_index_query() {
        let mut index = SimilarityIndex::new(SimilarityMetric::Cosine);
        for (entity, emb) in sample_embeddings() {
            index.add(entity, emb);
        }

        let query_emb = vec![1.0, 0.0, 0.0];
        let results = index.query(&query_emb, 2);

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].0, "a");
    }

    #[test]
    fn test_similarity_index_remove() {
        let mut index = SimilarityIndex::new(SimilarityMetric::Cosine);
        index.add("test".to_string(), vec![1.0, 0.0, 0.0]);
        assert_eq!(index.size(), 1);

        index.remove("test");
        assert_eq!(index.size(), 0);
    }

    #[test]
    fn test_lsh_index_add() {
        let mut index = LSHIndex::new(4, 8, 3);
        index.add("test".to_string(), vec![1.0, 0.0, 0.0]);
        assert_eq!(index.size(), 1);
    }

    #[test]
    fn test_lsh_index_query() {
        let mut index = LSHIndex::new(4, 8, 3);
        for (entity, emb) in sample_embeddings() {
            index.add(entity, emb);
        }

        let query_emb = vec![1.0, 0.0, 0.0];
        let results = index.query(&query_emb, 2, SimilarityMetric::Cosine);

        assert!(results.len() <= 2);
    }

    #[test]
    fn test_lsh_index_clear() {
        let mut index = LSHIndex::new(4, 8, 3);
        index.add("test".to_string(), vec![1.0, 0.0, 0.0]);
        assert_eq!(index.size(), 1);

        index.clear();
        assert_eq!(index.size(), 0);
    }

    #[test]
    fn test_hnsw_index_add() {
        let mut index = HNSWIndex::new(10, SimilarityMetric::Cosine);
        index.add("test".to_string(), vec![1.0, 0.0, 0.0]);
        assert_eq!(index.size(), 1);
    }

    #[test]
    fn test_hnsw_index_query() {
        let mut index = HNSWIndex::new(10, SimilarityMetric::Cosine);
        for (entity, emb) in sample_embeddings() {
            index.add(entity, emb);
        }

        let query_emb = vec![1.0, 0.0, 0.0];
        let results = index.query(&query_emb, 2);

        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_manhattan_distance() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![0.0, 1.0, 0.0];
        assert_eq!(manhattan_distance(&a, &b), 2.0);

        let c = vec![0.0, 0.0, 0.0];
        let d = vec![1.0, 1.0, 1.0];
        assert_eq!(manhattan_distance(&c, &d), 3.0);
    }

    #[test]
    fn test_similarity_metrics() {
        // Test with each metric
        let index_cosine = SimilarityIndex::new(SimilarityMetric::Cosine);
        let index_euclidean = SimilarityIndex::new(SimilarityMetric::Euclidean);
        let index_manhattan = SimilarityIndex::new(SimilarityMetric::Manhattan);

        assert_eq!(index_cosine.metric, SimilarityMetric::Cosine);
        assert_eq!(index_euclidean.metric, SimilarityMetric::Euclidean);
        assert_eq!(index_manhattan.metric, SimilarityMetric::Manhattan);
    }

    #[test]
    fn test_empty_query() {
        let index = SimilarityIndex::new(SimilarityMetric::Cosine);
        let results = index.query(&vec![1.0, 0.0, 0.0], 5);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_query_more_than_available() {
        let mut index = SimilarityIndex::new(SimilarityMetric::Cosine);
        index.add("a".to_string(), vec![1.0, 0.0, 0.0]);
        index.add("b".to_string(), vec![0.0, 1.0, 0.0]);

        let results = index.query(&vec![1.0, 0.0, 0.0], 10);
        assert_eq!(results.len(), 2);
    }
}
