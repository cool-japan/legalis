//! Embedding-based link prediction for legal knowledge graphs.
//!
//! This module provides functionality for predicting missing links in knowledge graphs
//! using embedding-based approaches.

use crate::{RdfValue, Triple};
use std::collections::HashMap;

/// An embedding vector.
pub type Embedding = Vec<f64>;

/// Predicted link with confidence score.
#[derive(Debug, Clone, PartialEq)]
pub struct PredictedLink {
    /// Subject entity
    pub subject: String,
    /// Relation
    pub relation: String,
    /// Object entity
    pub object: String,
    /// Confidence score (0.0 to 1.0)
    pub score: f64,
}

impl PredictedLink {
    /// Converts to an RDF triple.
    pub fn to_triple(&self) -> Triple {
        Triple {
            subject: self.subject.clone(),
            predicate: self.relation.clone(),
            object: RdfValue::Uri(self.object.clone()),
        }
    }
}

/// Embedding-based link predictor.
pub struct EmbeddingLinkPredictor {
    /// Entity embeddings
    entity_embeddings: HashMap<String, Embedding>,
    /// Relation embeddings
    relation_embeddings: HashMap<String, Embedding>,
    /// Embedding dimension
    embedding_dim: usize,
    /// Scoring function
    scoring_fn: ScoringFunction,
}

/// Scoring function for link prediction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScoringFunction {
    /// TransE: score = -||h + r - t||
    TransE,
    /// DistMult: score = <h, r, t> (element-wise product)
    DistMult,
    /// ComplEx: score based on complex embeddings
    ComplEx,
}

impl EmbeddingLinkPredictor {
    /// Creates a new embedding-based link predictor.
    pub fn new(embedding_dim: usize, scoring_fn: ScoringFunction) -> Self {
        Self {
            entity_embeddings: HashMap::new(),
            relation_embeddings: HashMap::new(),
            embedding_dim,
            scoring_fn,
        }
    }

    /// Initializes embeddings from existing knowledge graph.
    pub fn initialize_from_triples(&mut self, triples: &[Triple]) {
        // Collect all entities and relations
        let mut entities = std::collections::HashSet::new();
        let mut relations = std::collections::HashSet::new();

        for triple in triples {
            entities.insert(triple.subject.clone());
            relations.insert(triple.predicate.clone());

            if let RdfValue::Uri(ref obj) = triple.object {
                entities.insert(obj.clone());
            }
        }

        // Initialize random embeddings (in real implementation, would train these)
        for entity in entities {
            self.entity_embeddings
                .insert(entity, self.random_embedding());
        }

        for relation in relations {
            self.relation_embeddings
                .insert(relation, self.random_embedding());
        }
    }

    fn random_embedding(&self) -> Embedding {
        // Simple random initialization (in real implementation, would use proper initialization)
        (0..self.embedding_dim)
            .map(|i| (i as f64 * 0.1) % 1.0 - 0.5)
            .collect()
    }

    /// Sets embedding for an entity.
    pub fn set_entity_embedding(&mut self, entity: impl Into<String>, embedding: Embedding) {
        if embedding.len() != self.embedding_dim {
            panic!("Embedding dimension mismatch");
        }
        self.entity_embeddings.insert(entity.into(), embedding);
    }

    /// Sets embedding for a relation.
    pub fn set_relation_embedding(&mut self, relation: impl Into<String>, embedding: Embedding) {
        if embedding.len() != self.embedding_dim {
            panic!("Embedding dimension mismatch");
        }
        self.relation_embeddings.insert(relation.into(), embedding);
    }

    /// Predicts the top-k most likely objects for a given (subject, relation) pair.
    pub fn predict_objects(&self, subject: &str, relation: &str, k: usize) -> Vec<PredictedLink> {
        let subject_emb = match self.entity_embeddings.get(subject) {
            Some(emb) => emb,
            None => return Vec::new(),
        };

        let relation_emb = match self.relation_embeddings.get(relation) {
            Some(emb) => emb,
            None => return Vec::new(),
        };

        // Score all possible objects
        let mut scores: Vec<(String, f64)> = self
            .entity_embeddings
            .iter()
            .filter(|(entity, _)| *entity != subject)
            .map(|(entity, obj_emb)| {
                let score = self.score_triple(subject_emb, relation_emb, obj_emb);
                (entity.clone(), score)
            })
            .collect();

        // Sort by score (descending)
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // Return top-k
        scores
            .into_iter()
            .take(k)
            .map(|(object, score)| PredictedLink {
                subject: subject.to_string(),
                relation: relation.to_string(),
                object,
                score: self.normalize_score(score),
            })
            .collect()
    }

    /// Predicts the top-k most likely subjects for a given (relation, object) pair.
    pub fn predict_subjects(&self, relation: &str, object: &str, k: usize) -> Vec<PredictedLink> {
        let object_emb = match self.entity_embeddings.get(object) {
            Some(emb) => emb,
            None => return Vec::new(),
        };

        let relation_emb = match self.relation_embeddings.get(relation) {
            Some(emb) => emb,
            None => return Vec::new(),
        };

        // Score all possible subjects
        let mut scores: Vec<(String, f64)> = self
            .entity_embeddings
            .iter()
            .filter(|(entity, _)| *entity != object)
            .map(|(entity, subj_emb)| {
                let score = self.score_triple(subj_emb, relation_emb, object_emb);
                (entity.clone(), score)
            })
            .collect();

        // Sort by score (descending)
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // Return top-k
        scores
            .into_iter()
            .take(k)
            .map(|(subject, score)| PredictedLink {
                subject,
                relation: relation.to_string(),
                object: object.to_string(),
                score: self.normalize_score(score),
            })
            .collect()
    }

    /// Scores a triple (subject, relation, object).
    fn score_triple(&self, subj_emb: &[f64], rel_emb: &[f64], obj_emb: &[f64]) -> f64 {
        match self.scoring_fn {
            ScoringFunction::TransE => self.score_transe(subj_emb, rel_emb, obj_emb),
            ScoringFunction::DistMult => self.score_distmult(subj_emb, rel_emb, obj_emb),
            ScoringFunction::ComplEx => self.score_complex(subj_emb, rel_emb, obj_emb),
        }
    }

    fn score_transe(&self, subj: &[f64], rel: &[f64], obj: &[f64]) -> f64 {
        // TransE: score = -||h + r - t||
        let mut sum = 0.0;
        for i in 0..self.embedding_dim {
            let diff = subj[i] + rel[i] - obj[i];
            sum += diff * diff;
        }
        -sum.sqrt()
    }

    fn score_distmult(&self, subj: &[f64], rel: &[f64], obj: &[f64]) -> f64 {
        // DistMult: score = sum(h * r * t)
        let mut sum = 0.0;
        for i in 0..self.embedding_dim {
            sum += subj[i] * rel[i] * obj[i];
        }
        sum
    }

    fn score_complex(&self, subj: &[f64], rel: &[f64], obj: &[f64]) -> f64 {
        // Simplified ComplEx (real part only for demo)
        self.score_distmult(subj, rel, obj)
    }

    fn normalize_score(&self, score: f64) -> f64 {
        // Normalize to [0, 1] range using sigmoid
        1.0 / (1.0 + (-score).exp())
    }

    /// Computes similarity between two entities based on their embeddings.
    pub fn entity_similarity(&self, entity1: &str, entity2: &str) -> Option<f64> {
        let emb1 = self.entity_embeddings.get(entity1)?;
        let emb2 = self.entity_embeddings.get(entity2)?;

        Some(self.cosine_similarity(emb1, emb2))
    }

    fn cosine_similarity(&self, a: &[f64], b: &[f64]) -> f64 {
        let dot: f64 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let norm_a: f64 = a.iter().map(|x| x * x).sum::<f64>().sqrt();
        let norm_b: f64 = b.iter().map(|x| x * x).sum::<f64>().sqrt();

        if norm_a == 0.0 || norm_b == 0.0 {
            0.0
        } else {
            dot / (norm_a * norm_b)
        }
    }

    /// Finds the most similar entities to a given entity.
    pub fn find_similar_entities(&self, entity: &str, k: usize) -> Vec<(String, f64)> {
        let emb = match self.entity_embeddings.get(entity) {
            Some(e) => e,
            None => return Vec::new(),
        };

        let mut similarities: Vec<(String, f64)> = self
            .entity_embeddings
            .iter()
            .filter(|(e, _)| *e != entity)
            .map(|(e, other_emb)| (e.clone(), self.cosine_similarity(emb, other_emb)))
            .collect();

        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        similarities.truncate(k);
        similarities
    }

    /// Gets statistics about the predictor.
    pub fn stats(&self) -> LinkPredictorStats {
        LinkPredictorStats {
            num_entities: self.entity_embeddings.len(),
            num_relations: self.relation_embeddings.len(),
            embedding_dim: self.embedding_dim,
            scoring_function: self.scoring_fn,
        }
    }
}

/// Statistics about the link predictor.
#[derive(Debug, Clone)]
pub struct LinkPredictorStats {
    /// Number of entities with embeddings
    pub num_entities: usize,
    /// Number of relations with embeddings
    pub num_relations: usize,
    /// Embedding dimension
    pub embedding_dim: usize,
    /// Scoring function used
    pub scoring_function: ScoringFunction,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_predictor() -> EmbeddingLinkPredictor {
        let mut predictor = EmbeddingLinkPredictor::new(10, ScoringFunction::TransE);

        // Add some test embeddings
        predictor.set_entity_embedding("A", vec![0.1; 10]);
        predictor.set_entity_embedding("B", vec![0.2; 10]);
        predictor.set_entity_embedding("C", vec![0.3; 10]);

        predictor.set_relation_embedding("r1", vec![0.15; 10]);

        predictor
    }

    #[test]
    fn test_predictor_creation() {
        let predictor = EmbeddingLinkPredictor::new(10, ScoringFunction::TransE);
        assert_eq!(predictor.embedding_dim, 10);
        assert_eq!(predictor.scoring_fn, ScoringFunction::TransE);
    }

    #[test]
    fn test_set_embeddings() {
        let mut predictor = EmbeddingLinkPredictor::new(5, ScoringFunction::TransE);
        predictor.set_entity_embedding("test", vec![0.1, 0.2, 0.3, 0.4, 0.5]);

        assert_eq!(predictor.entity_embeddings.len(), 1);
        assert!(predictor.entity_embeddings.contains_key("test"));
    }

    #[test]
    #[should_panic(expected = "Embedding dimension mismatch")]
    fn test_embedding_dimension_mismatch() {
        let mut predictor = EmbeddingLinkPredictor::new(5, ScoringFunction::TransE);
        predictor.set_entity_embedding("test", vec![0.1, 0.2]); // Wrong dimension
    }

    #[test]
    fn test_predict_objects() {
        let predictor = create_test_predictor();
        let predictions = predictor.predict_objects("A", "r1", 2);

        assert_eq!(predictions.len(), 2);
        assert!(predictions.iter().all(|p| p.subject == "A"));
        assert!(predictions.iter().all(|p| p.relation == "r1"));
    }

    #[test]
    fn test_predict_subjects() {
        let predictor = create_test_predictor();
        let predictions = predictor.predict_subjects("r1", "C", 2);

        assert_eq!(predictions.len(), 2);
        assert!(predictions.iter().all(|p| p.object == "C"));
        assert!(predictions.iter().all(|p| p.relation == "r1"));
    }

    #[test]
    fn test_score_normalization() {
        let predictor = EmbeddingLinkPredictor::new(10, ScoringFunction::TransE);

        let score1 = predictor.normalize_score(0.0);
        assert!((score1 - 0.5).abs() < 0.01);

        let score2 = predictor.normalize_score(10.0);
        assert!(score2 > 0.9);

        let score3 = predictor.normalize_score(-10.0);
        assert!(score3 < 0.1);
    }

    #[test]
    fn test_entity_similarity() {
        let predictor = create_test_predictor();
        let sim = predictor.entity_similarity("A", "B");

        assert!(sim.is_some());
        let sim_val = sim.unwrap();
        assert!((-1.0..=1.0).contains(&sim_val));
    }

    #[test]
    fn test_cosine_similarity() {
        let predictor = EmbeddingLinkPredictor::new(3, ScoringFunction::TransE);

        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        let sim = predictor.cosine_similarity(&a, &b);
        assert!((sim - 1.0).abs() < 0.001);

        let c = vec![0.0, 1.0, 0.0];
        let sim2 = predictor.cosine_similarity(&a, &c);
        assert!(sim2.abs() < 0.001);
    }

    #[test]
    fn test_find_similar_entities() {
        let predictor = create_test_predictor();
        let similar = predictor.find_similar_entities("A", 2);

        assert_eq!(similar.len(), 2);
        assert!(similar.iter().all(|(e, _)| e != "A"));
    }

    #[test]
    fn test_initialize_from_triples() {
        let mut predictor = EmbeddingLinkPredictor::new(10, ScoringFunction::TransE);

        let triples = vec![
            Triple {
                subject: "E1".to_string(),
                predicate: "r1".to_string(),
                object: RdfValue::Uri("E2".to_string()),
            },
            Triple {
                subject: "E2".to_string(),
                predicate: "r2".to_string(),
                object: RdfValue::Uri("E3".to_string()),
            },
        ];

        predictor.initialize_from_triples(&triples);

        assert_eq!(predictor.entity_embeddings.len(), 3);
        assert_eq!(predictor.relation_embeddings.len(), 2);
    }

    #[test]
    fn test_predicted_link_to_triple() {
        let link = PredictedLink {
            subject: "A".to_string(),
            relation: "r1".to_string(),
            object: "B".to_string(),
            score: 0.9,
        };

        let triple = link.to_triple();
        assert_eq!(triple.subject, "A");
        assert_eq!(triple.predicate, "r1");
        assert!(matches!(triple.object, RdfValue::Uri(ref s) if s == "B"));
    }

    #[test]
    fn test_scoring_functions() {
        let predictor_transe = EmbeddingLinkPredictor::new(5, ScoringFunction::TransE);
        let predictor_distmult = EmbeddingLinkPredictor::new(5, ScoringFunction::DistMult);
        let predictor_complex = EmbeddingLinkPredictor::new(5, ScoringFunction::ComplEx);

        let subj = vec![0.1; 5];
        let rel = vec![0.2; 5];
        let obj = vec![0.3; 5];

        let score_transe = predictor_transe.score_triple(&subj, &rel, &obj);
        let score_distmult = predictor_distmult.score_triple(&subj, &rel, &obj);
        let score_complex = predictor_complex.score_triple(&subj, &rel, &obj);

        // Scores should be different (except ComplEx and DistMult which we simplified)
        assert_ne!(score_transe, score_distmult);
        assert_eq!(score_distmult, score_complex); // Simplified ComplEx = DistMult in our impl
    }

    #[test]
    fn test_stats() {
        let predictor = create_test_predictor();
        let stats = predictor.stats();

        assert_eq!(stats.num_entities, 3);
        assert_eq!(stats.num_relations, 1);
        assert_eq!(stats.embedding_dim, 10);
        assert_eq!(stats.scoring_function, ScoringFunction::TransE);
    }

    #[test]
    fn test_empty_predictions() {
        let predictor = EmbeddingLinkPredictor::new(10, ScoringFunction::TransE);

        // No embeddings, should return empty
        let predictions = predictor.predict_objects("unknown", "r1", 5);
        assert!(predictions.is_empty());
    }

    #[test]
    fn test_prediction_scores_normalized() {
        let predictor = create_test_predictor();
        let predictions = predictor.predict_objects("A", "r1", 2);

        for pred in &predictions {
            assert!(pred.score >= 0.0 && pred.score <= 1.0);
        }
    }

    #[test]
    fn test_predictions_sorted() {
        let predictor = create_test_predictor();
        let predictions = predictor.predict_objects("A", "r1", 2);

        // Scores should be in descending order
        for i in 0..predictions.len() - 1 {
            assert!(predictions[i].score >= predictions[i + 1].score);
        }
    }

    #[test]
    fn test_random_embedding() {
        let predictor = EmbeddingLinkPredictor::new(10, ScoringFunction::TransE);
        let emb = predictor.random_embedding();

        assert_eq!(emb.len(), 10);
        assert!(emb.iter().all(|x| *x >= -1.0 && *x <= 1.0));
    }
}
