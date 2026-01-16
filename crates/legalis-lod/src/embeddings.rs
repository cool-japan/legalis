//! Graph embedding generation for knowledge graphs.
//!
//! This module provides various graph embedding techniques to convert RDF triples
//! into dense vector representations for machine learning and similarity search.
//!
//! Supported embedding methods:
//! - TransE: Translation-based embeddings
//! - Node2Vec-style random walks
//! - Simple co-occurrence embeddings

use crate::{RdfValue, Triple};
use std::collections::{HashMap, HashSet};

/// Vector representation of an entity or relation.
pub type Embedding = Vec<f64>;

/// Configuration for embedding generation.
#[derive(Debug, Clone)]
pub struct EmbeddingConfig {
    /// Dimension of the embedding vectors
    pub dimension: usize,
    /// Learning rate for training
    pub learning_rate: f64,
    /// Number of training epochs
    pub epochs: usize,
    /// Random walk length (for random-walk based methods)
    pub walk_length: usize,
    /// Number of walks per node
    pub num_walks: usize,
}

impl Default for EmbeddingConfig {
    fn default() -> Self {
        Self {
            dimension: 100,
            learning_rate: 0.01,
            epochs: 100,
            walk_length: 10,
            num_walks: 10,
        }
    }
}

/// Graph embedding generator.
pub struct EmbeddingGenerator {
    config: EmbeddingConfig,
    entity_embeddings: HashMap<String, Embedding>,
    relation_embeddings: HashMap<String, Embedding>,
}

impl EmbeddingGenerator {
    /// Creates a new embedding generator with default config.
    pub fn new() -> Self {
        Self::with_config(EmbeddingConfig::default())
    }

    /// Creates a new embedding generator with custom config.
    pub fn with_config(config: EmbeddingConfig) -> Self {
        Self {
            config,
            entity_embeddings: HashMap::new(),
            relation_embeddings: HashMap::new(),
        }
    }

    /// Trains embeddings using TransE algorithm.
    /// TransE represents relations as translations in embedding space: h + r ≈ t
    pub fn train_transe(&mut self, triples: &[Triple]) {
        // Initialize embeddings randomly
        self.initialize_embeddings(triples);

        // Training loop
        for epoch in 0..self.config.epochs {
            let mut total_loss = 0.0;

            for triple in triples {
                // Get embeddings
                let h = self.get_or_init_entity(&triple.subject);
                let r = self.get_or_init_relation(&triple.predicate);
                let t = self.get_or_init_entity_from_value(&triple.object);

                // Calculate loss: ||h + r - t||
                let loss = self.transe_loss(&h, &r, &t);
                total_loss += loss;

                // Update embeddings via gradient descent
                self.update_transe_embeddings(&triple.subject, &triple.predicate, &triple.object);
            }

            if epoch % 10 == 0 {
                let _avg_loss = total_loss / triples.len() as f64;
                #[cfg(test)]
                println!("Epoch {}: avg loss = {:.4}", epoch, _avg_loss);
            }
        }
    }

    /// Generates embeddings using random walk sampling.
    pub fn train_random_walk(&mut self, triples: &[Triple]) {
        // Build adjacency list
        let graph = self.build_graph(triples);

        // Initialize embeddings
        self.initialize_embeddings(triples);

        // Generate random walks for each node
        let mut walks = Vec::new();
        for node in graph.keys() {
            for _ in 0..self.config.num_walks {
                walks.push(self.random_walk(node, &graph));
            }
        }

        // Train embeddings using skip-gram-like approach
        self.train_from_walks(&walks);
    }

    /// Generates simple co-occurrence based embeddings.
    pub fn train_cooccurrence(&mut self, triples: &[Triple]) {
        // Build co-occurrence matrix
        let mut cooccurrence: HashMap<String, HashMap<String, f64>> = HashMap::new();

        for triple in triples {
            let subj = triple.subject.clone();
            let obj = self.extract_entity_from_value(&triple.object);

            if let Some(obj) = obj {
                *cooccurrence
                    .entry(subj.clone())
                    .or_default()
                    .entry(obj.clone())
                    .or_default() += 1.0;
                *cooccurrence
                    .entry(obj)
                    .or_default()
                    .entry(subj)
                    .or_default() += 1.0;
            }
        }

        // Convert co-occurrence to embeddings using simple dimensionality reduction
        self.cooccurrence_to_embeddings(cooccurrence);
    }

    /// Gets the embedding for an entity.
    pub fn get_entity_embedding(&self, entity: &str) -> Option<&Embedding> {
        self.entity_embeddings.get(entity)
    }

    /// Gets the embedding for a relation.
    pub fn get_relation_embedding(&self, relation: &str) -> Option<&Embedding> {
        self.relation_embeddings.get(relation)
    }

    /// Calculates cosine similarity between two entities.
    pub fn entity_similarity(&self, entity1: &str, entity2: &str) -> Option<f64> {
        let emb1 = self.get_entity_embedding(entity1)?;
        let emb2 = self.get_entity_embedding(entity2)?;
        Some(cosine_similarity(emb1, emb2))
    }

    /// Finds the k most similar entities to a given entity.
    pub fn find_similar_entities(&self, entity: &str, k: usize) -> Vec<(String, f64)> {
        let target_emb = match self.get_entity_embedding(entity) {
            Some(emb) => emb,
            None => return Vec::new(),
        };

        let mut similarities: Vec<(String, f64)> = self
            .entity_embeddings
            .iter()
            .filter(|(e, _)| *e != entity)
            .map(|(e, emb)| (e.clone(), cosine_similarity(target_emb, emb)))
            .collect();

        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        similarities.into_iter().take(k).collect()
    }

    /// Exports embeddings in a simple text format (entity/relation -> vector).
    pub fn export_embeddings(&self) -> String {
        let mut output = String::new();

        output.push_str("# Entity Embeddings\n");
        for (entity, emb) in &self.entity_embeddings {
            output.push_str(&format!(
                "{}\t{}\n",
                entity,
                emb.iter()
                    .map(|v| format!("{:.6}", v))
                    .collect::<Vec<_>>()
                    .join(" ")
            ));
        }

        output.push_str("\n# Relation Embeddings\n");
        for (relation, emb) in &self.relation_embeddings {
            output.push_str(&format!(
                "{}\t{}\n",
                relation,
                emb.iter()
                    .map(|v| format!("{:.6}", v))
                    .collect::<Vec<_>>()
                    .join(" ")
            ));
        }

        output
    }

    // Private helper methods

    fn initialize_embeddings(&mut self, triples: &[Triple]) {
        use rand::Rng;
        let mut rng = rand::rng();

        // Collect all entities and relations
        let mut entities = HashSet::new();
        let mut relations = HashSet::new();

        for triple in triples {
            entities.insert(triple.subject.clone());
            relations.insert(triple.predicate.clone());
            if let Some(entity) = self.extract_entity_from_value(&triple.object) {
                entities.insert(entity);
            }
        }

        // Initialize with random values in [-0.1, 0.1]
        for entity in entities {
            let emb: Vec<f64> = (0..self.config.dimension)
                .map(|_| rng.random_range(-0.1..0.1))
                .collect();
            self.entity_embeddings.insert(entity, emb);
        }

        for relation in relations {
            let emb: Vec<f64> = (0..self.config.dimension)
                .map(|_| rng.random_range(-0.1..0.1))
                .collect();
            self.relation_embeddings.insert(relation, emb);
        }
    }

    fn get_or_init_entity(&self, entity: &str) -> Embedding {
        self.entity_embeddings
            .get(entity)
            .cloned()
            .unwrap_or_else(|| vec![0.0; self.config.dimension])
    }

    fn get_or_init_relation(&self, relation: &str) -> Embedding {
        self.relation_embeddings
            .get(relation)
            .cloned()
            .unwrap_or_else(|| vec![0.0; self.config.dimension])
    }

    fn get_or_init_entity_from_value(&self, value: &RdfValue) -> Embedding {
        if let Some(entity) = self.extract_entity_from_value(value) {
            self.get_or_init_entity(&entity)
        } else {
            vec![0.0; self.config.dimension]
        }
    }

    fn extract_entity_from_value(&self, value: &RdfValue) -> Option<String> {
        match value {
            RdfValue::Uri(uri) => Some(uri.clone()),
            _ => None,
        }
    }

    fn transe_loss(&self, h: &Embedding, r: &Embedding, t: &Embedding) -> f64 {
        let mut sum = 0.0;
        for i in 0..h.len() {
            let diff = h[i] + r[i] - t[i];
            sum += diff * diff;
        }
        sum.sqrt()
    }

    fn update_transe_embeddings(&mut self, subj: &str, pred: &str, obj: &RdfValue) {
        let lr = self.config.learning_rate;

        let h = self.get_or_init_entity(subj);
        let r = self.get_or_init_relation(pred);
        let t = self.get_or_init_entity_from_value(obj);

        // Gradient descent update
        let mut new_h = h.clone();
        let mut new_r = r.clone();
        let mut new_t = t.clone();

        for i in 0..self.config.dimension {
            let diff = h[i] + r[i] - t[i];
            new_h[i] -= lr * diff;
            new_r[i] -= lr * diff;
            new_t[i] += lr * diff;
        }

        self.entity_embeddings.insert(subj.to_string(), new_h);
        self.relation_embeddings.insert(pred.to_string(), new_r);
        if let Some(obj_entity) = self.extract_entity_from_value(obj) {
            self.entity_embeddings.insert(obj_entity, new_t);
        }
    }

    fn build_graph(&self, triples: &[Triple]) -> HashMap<String, Vec<String>> {
        let mut graph: HashMap<String, Vec<String>> = HashMap::new();

        for triple in triples {
            if let Some(obj) = self.extract_entity_from_value(&triple.object) {
                graph
                    .entry(triple.subject.clone())
                    .or_default()
                    .push(obj.clone());
                graph.entry(obj).or_default().push(triple.subject.clone());
            }
        }

        graph
    }

    fn random_walk(&self, start: &str, graph: &HashMap<String, Vec<String>>) -> Vec<String> {
        use rand::prelude::IndexedRandom;
        let mut rng = rand::rng();
        let mut walk = vec![start.to_string()];
        let mut current = start;

        for _ in 1..self.config.walk_length {
            if let Some(neighbors) = graph.get(current) {
                if neighbors.is_empty() {
                    break;
                }
                current = neighbors.choose(&mut rng).unwrap();
                walk.push(current.to_string());
            } else {
                break;
            }
        }

        walk
    }

    fn train_from_walks(&mut self, walks: &[Vec<String>]) {
        // Simple skip-gram-like training
        let window_size = 2;

        for walk in walks {
            for (i, center) in walk.iter().enumerate() {
                let start = i.saturating_sub(window_size);
                let end = (i + window_size + 1).min(walk.len());

                for (idx, context) in walk.iter().enumerate().take(end).skip(start) {
                    if i != idx {
                        self.update_skipgram(center, context);
                    }
                }
            }
        }
    }

    fn update_skipgram(&mut self, center: &str, context: &str) {
        let lr = self.config.learning_rate;

        let center_emb = self.get_or_init_entity(center);
        let context_emb = self.get_or_init_entity(context);

        // Simple gradient update to bring embeddings closer
        let mut new_center = center_emb.clone();
        let mut new_context = context_emb.clone();

        for i in 0..self.config.dimension {
            let diff = center_emb[i] - context_emb[i];
            new_center[i] -= lr * diff;
            new_context[i] += lr * diff;
        }

        self.entity_embeddings
            .insert(center.to_string(), new_center);
        self.entity_embeddings
            .insert(context.to_string(), new_context);
    }

    fn cooccurrence_to_embeddings(&mut self, cooccurrence: HashMap<String, HashMap<String, f64>>) {
        // Simple truncated SVD-like approach
        // For each entity, create embedding based on its co-occurrence pattern
        for (entity, neighbors) in cooccurrence {
            let mut emb = vec![0.0; self.config.dimension];

            // Hash-based projection (simple and fast)
            for (neighbor, count) in neighbors {
                let hash = self.simple_hash(&neighbor) % self.config.dimension;
                emb[hash] += count;
            }

            // Normalize
            let norm = emb.iter().map(|x| x * x).sum::<f64>().sqrt();
            if norm > 0.0 {
                for val in &mut emb {
                    *val /= norm;
                }
            }

            self.entity_embeddings.insert(entity, emb);
        }
    }

    fn simple_hash(&self, s: &str) -> usize {
        s.bytes().map(|b| b as usize).sum()
    }
}

impl Default for EmbeddingGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Calculates cosine similarity between two vectors.
pub fn cosine_similarity(a: &[f64], b: &[f64]) -> f64 {
    if a.len() != b.len() {
        return 0.0;
    }

    let dot_product: f64 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f64 = a.iter().map(|x| x * x).sum::<f64>().sqrt();
    let norm_b: f64 = b.iter().map(|x| x * x).sum::<f64>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        dot_product / (norm_a * norm_b)
    }
}

/// Calculates Euclidean distance between two vectors.
pub fn euclidean_distance(a: &[f64], b: &[f64]) -> f64 {
    if a.len() != b.len() {
        return f64::INFINITY;
    }

    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f64>()
        .sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_triples() -> Vec<Triple> {
        vec![
            Triple {
                subject: "ex:Alice".to_string(),
                predicate: "ex:knows".to_string(),
                object: RdfValue::Uri("ex:Bob".to_string()),
            },
            Triple {
                subject: "ex:Bob".to_string(),
                predicate: "ex:knows".to_string(),
                object: RdfValue::Uri("ex:Charlie".to_string()),
            },
            Triple {
                subject: "ex:Alice".to_string(),
                predicate: "ex:likes".to_string(),
                object: RdfValue::Uri("ex:Music".to_string()),
            },
        ]
    }

    #[test]
    fn test_embedding_config_default() {
        let config = EmbeddingConfig::default();
        assert_eq!(config.dimension, 100);
        assert_eq!(config.epochs, 100);
    }

    #[test]
    fn test_initialize_embeddings() {
        let mut generator = EmbeddingGenerator::new();
        let triples = sample_triples();
        generator.initialize_embeddings(&triples);

        assert!(generator.get_entity_embedding("ex:Alice").is_some());
        assert!(generator.get_entity_embedding("ex:Bob").is_some());
        assert!(generator.get_relation_embedding("ex:knows").is_some());
    }

    #[test]
    fn test_train_transe() {
        let mut generator = EmbeddingGenerator::with_config(EmbeddingConfig {
            dimension: 10,
            learning_rate: 0.01,
            epochs: 10,
            ..Default::default()
        });
        let triples = sample_triples();
        generator.train_transe(&triples);

        // Should have embeddings for all entities
        assert!(generator.get_entity_embedding("ex:Alice").is_some());
        assert_eq!(
            generator.get_entity_embedding("ex:Alice").unwrap().len(),
            10
        );
    }

    #[test]
    fn test_train_cooccurrence() {
        let mut generator = EmbeddingGenerator::with_config(EmbeddingConfig {
            dimension: 10,
            ..Default::default()
        });
        let triples = sample_triples();
        generator.train_cooccurrence(&triples);

        assert!(generator.get_entity_embedding("ex:Alice").is_some());
        assert!(generator.get_entity_embedding("ex:Bob").is_some());
    }

    #[test]
    fn test_entity_similarity() {
        let mut generator = EmbeddingGenerator::with_config(EmbeddingConfig {
            dimension: 10,
            learning_rate: 0.01,
            epochs: 20,
            ..Default::default()
        });
        let triples = sample_triples();
        generator.train_transe(&triples);

        let similarity = generator.entity_similarity("ex:Alice", "ex:Bob");
        assert!(similarity.is_some());
        let sim_value = similarity.unwrap();
        assert!((-1.0..=1.0).contains(&sim_value));
    }

    #[test]
    fn test_find_similar_entities() {
        let mut generator = EmbeddingGenerator::with_config(EmbeddingConfig {
            dimension: 10,
            epochs: 20,
            ..Default::default()
        });
        let triples = sample_triples();
        generator.train_transe(&triples);

        let similar = generator.find_similar_entities("ex:Alice", 2);
        assert!(similar.len() <= 2);
    }

    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        assert!((cosine_similarity(&a, &b) - 1.0).abs() < 1e-6);

        let c = vec![1.0, 0.0, 0.0];
        let d = vec![0.0, 1.0, 0.0];
        assert!((cosine_similarity(&c, &d) - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_euclidean_distance() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        assert!((euclidean_distance(&a, &b) - 0.0).abs() < 1e-6);

        let c = vec![0.0, 0.0, 0.0];
        let d = vec![1.0, 1.0, 1.0];
        let expected = 3.0_f64.sqrt();
        assert!((euclidean_distance(&c, &d) - expected).abs() < 1e-6);
    }

    #[test]
    fn test_export_embeddings() {
        let mut generator = EmbeddingGenerator::with_config(EmbeddingConfig {
            dimension: 5,
            epochs: 5,
            ..Default::default()
        });
        let triples = sample_triples();
        generator.train_transe(&triples);

        let export = generator.export_embeddings();
        assert!(export.contains("# Entity Embeddings"));
        assert!(export.contains("# Relation Embeddings"));
        assert!(export.contains("ex:Alice"));
    }

    #[test]
    fn test_random_walk() {
        let generator = EmbeddingGenerator::with_config(EmbeddingConfig {
            walk_length: 5,
            ..Default::default()
        });
        let triples = sample_triples();
        let graph = generator.build_graph(&triples);

        let walk = generator.random_walk("ex:Alice", &graph);
        assert!(walk.len() <= 5);
        assert_eq!(walk[0], "ex:Alice");
    }

    #[test]
    fn test_train_random_walk() {
        let mut generator = EmbeddingGenerator::with_config(EmbeddingConfig {
            dimension: 10,
            walk_length: 5,
            num_walks: 5,
            epochs: 10,
            ..Default::default()
        });
        let triples = sample_triples();
        generator.train_random_walk(&triples);

        assert!(generator.get_entity_embedding("ex:Alice").is_some());
        assert_eq!(
            generator.get_entity_embedding("ex:Alice").unwrap().len(),
            10
        );
    }

    #[test]
    fn test_empty_triples() {
        let mut generator = EmbeddingGenerator::new();
        let triples: Vec<Triple> = Vec::new();
        generator.train_transe(&triples);

        assert_eq!(generator.entity_embeddings.len(), 0);
        assert_eq!(generator.relation_embeddings.len(), 0);
    }

    #[test]
    fn test_single_triple() {
        let mut generator = EmbeddingGenerator::with_config(EmbeddingConfig {
            dimension: 10,
            epochs: 5,
            ..Default::default()
        });
        let triples = vec![Triple {
            subject: "ex:A".to_string(),
            predicate: "ex:rel".to_string(),
            object: RdfValue::Uri("ex:B".to_string()),
        }];
        generator.train_transe(&triples);

        assert_eq!(generator.entity_embeddings.len(), 2);
        assert_eq!(generator.relation_embeddings.len(), 1);
    }
}
