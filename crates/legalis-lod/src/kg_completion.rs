//! Knowledge graph completion for legal knowledge graphs.
//!
//! This module provides algorithms for predicting missing links and entities
//! in legal knowledge graphs.

use crate::{RdfValue, Triple};
use std::collections::{HashMap, HashSet};

/// Knowledge graph for completion tasks.
#[derive(Debug, Clone)]
pub struct KnowledgeGraph {
    /// All triples in the knowledge graph
    triples: Vec<Triple>,
    /// Entities in the graph
    entities: HashSet<String>,
    /// Relations in the graph
    relations: HashSet<String>,
    /// Entity-relation-entity index
    spo_index: HashMap<(String, String), Vec<String>>,
    /// Object-relation-subject index (reverse)
    ops_index: HashMap<(String, String), Vec<String>>,
}

impl KnowledgeGraph {
    /// Creates a new knowledge graph from triples.
    pub fn new(triples: Vec<Triple>) -> Self {
        let mut entities = HashSet::new();
        let mut relations = HashSet::new();
        let mut spo_index: HashMap<(String, String), Vec<String>> = HashMap::new();
        let mut ops_index: HashMap<(String, String), Vec<String>> = HashMap::new();

        for triple in &triples {
            entities.insert(triple.subject.clone());
            relations.insert(triple.predicate.clone());

            if let RdfValue::Uri(obj) = &triple.object {
                entities.insert(obj.clone());

                spo_index
                    .entry((triple.subject.clone(), triple.predicate.clone()))
                    .or_default()
                    .push(obj.clone());

                ops_index
                    .entry((obj.clone(), triple.predicate.clone()))
                    .or_default()
                    .push(triple.subject.clone());
            }
        }

        Self {
            triples,
            entities,
            relations,
            spo_index,
            ops_index,
        }
    }

    /// Gets all entities in the graph.
    pub fn entities(&self) -> &HashSet<String> {
        &self.entities
    }

    /// Gets all relations in the graph.
    pub fn relations(&self) -> &HashSet<String> {
        &self.relations
    }

    /// Gets all triples with the given subject and predicate.
    pub fn get_objects(&self, subject: &str, predicate: &str) -> Vec<&str> {
        self.spo_index
            .get(&(subject.to_string(), predicate.to_string()))
            .map(|objs| objs.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default()
    }

    /// Gets all triples with the given object and predicate (reverse lookup).
    pub fn get_subjects(&self, object: &str, predicate: &str) -> Vec<&str> {
        self.ops_index
            .get(&(object.to_string(), predicate.to_string()))
            .map(|subjs| subjs.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default()
    }

    /// Adds a triple to the graph.
    pub fn add_triple(&mut self, triple: Triple) {
        self.entities.insert(triple.subject.clone());
        self.relations.insert(triple.predicate.clone());

        if let RdfValue::Uri(ref obj) = triple.object {
            self.entities.insert(obj.clone());

            self.spo_index
                .entry((triple.subject.clone(), triple.predicate.clone()))
                .or_default()
                .push(obj.clone());

            self.ops_index
                .entry((obj.clone(), triple.predicate.clone()))
                .or_default()
                .push(triple.subject.clone());
        }

        self.triples.push(triple);
    }
}

/// Predicted triple with confidence score.
#[derive(Debug, Clone, PartialEq)]
pub struct PredictedTriple {
    /// Subject entity
    pub subject: String,
    /// Relation
    pub relation: String,
    /// Object entity
    pub object: String,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
    /// Explanation for the prediction
    pub explanation: String,
}

impl PredictedTriple {
    /// Converts to an RDF triple.
    pub fn to_triple(&self) -> Triple {
        Triple {
            subject: self.subject.clone(),
            predicate: self.relation.clone(),
            object: RdfValue::Uri(self.object.clone()),
        }
    }
}

/// Rule-based completion strategy.
///
/// Uses logical rules to infer missing triples.
#[derive(Debug)]
pub struct RuleBasedCompletion {
    /// Knowledge graph
    kg: KnowledgeGraph,
    /// Inference rules
    rules: Vec<InferenceRule>,
}

/// An inference rule for completion.
#[derive(Debug, Clone)]
pub struct InferenceRule {
    /// Rule name
    pub name: String,
    /// Rule pattern (if A and B then C)
    pub pattern: RulePattern,
    /// Confidence of this rule
    pub confidence: f64,
}

/// Pattern for inference rules.
#[derive(Debug, Clone)]
pub enum RulePattern {
    /// Transitivity: (A, r, B) ∧ (B, r, C) → (A, r, C)
    Transitive { relation: String },
    /// Symmetry: (A, r, B) → (B, r, A)
    Symmetric { relation: String },
    /// Inverse: (A, r1, B) → (B, r2, A)
    Inverse {
        relation1: String,
        relation2: String,
    },
    /// Composition: (A, r1, B) ∧ (B, r2, C) → (A, r3, C)
    Composition {
        relation1: String,
        relation2: String,
        result_relation: String,
    },
}

impl RuleBasedCompletion {
    /// Creates a new rule-based completion system.
    pub fn new(kg: KnowledgeGraph) -> Self {
        let mut completion = Self {
            kg,
            rules: Vec::new(),
        };
        completion.add_default_rules();
        completion
    }

    /// Adds default legal rules.
    fn add_default_rules(&mut self) {
        // Transitivity for references
        self.add_rule(InferenceRule {
            name: "transitive-references".to_string(),
            pattern: RulePattern::Transitive {
                relation: "legalis:references".to_string(),
            },
            confidence: 0.7,
        });

        // Symmetry for related-to
        self.add_rule(InferenceRule {
            name: "symmetric-related".to_string(),
            pattern: RulePattern::Symmetric {
                relation: "legalis:relatedTo".to_string(),
            },
            confidence: 0.9,
        });

        // Inverse: amends / amendedBy
        self.add_rule(InferenceRule {
            name: "inverse-amends".to_string(),
            pattern: RulePattern::Inverse {
                relation1: "legalis:amends".to_string(),
                relation2: "legalis:amendedBy".to_string(),
            },
            confidence: 1.0,
        });

        // Inverse: repeals / repealedBy
        self.add_rule(InferenceRule {
            name: "inverse-repeals".to_string(),
            pattern: RulePattern::Inverse {
                relation1: "legalis:repeals".to_string(),
                relation2: "legalis:repealedBy".to_string(),
            },
            confidence: 1.0,
        });

        // Composition: dependsOn + partOf -> dependsOn
        self.add_rule(InferenceRule {
            name: "compose-depends-part".to_string(),
            pattern: RulePattern::Composition {
                relation1: "legalis:dependsOn".to_string(),
                relation2: "legalis:partOf".to_string(),
                result_relation: "legalis:dependsOn".to_string(),
            },
            confidence: 0.8,
        });
    }

    /// Adds a custom rule.
    pub fn add_rule(&mut self, rule: InferenceRule) {
        self.rules.push(rule);
    }

    /// Predicts missing triples using rules.
    pub fn predict(&self) -> Vec<PredictedTriple> {
        let mut predictions = Vec::new();

        for rule in &self.rules {
            match &rule.pattern {
                RulePattern::Transitive { relation } => {
                    predictions.extend(self.apply_transitive(
                        relation,
                        rule.confidence,
                        &rule.name,
                    ));
                }
                RulePattern::Symmetric { relation } => {
                    predictions.extend(self.apply_symmetric(relation, rule.confidence, &rule.name));
                }
                RulePattern::Inverse {
                    relation1,
                    relation2,
                } => {
                    predictions.extend(self.apply_inverse(
                        relation1,
                        relation2,
                        rule.confidence,
                        &rule.name,
                    ));
                }
                RulePattern::Composition {
                    relation1,
                    relation2,
                    result_relation,
                } => {
                    predictions.extend(self.apply_composition(
                        relation1,
                        relation2,
                        result_relation,
                        rule.confidence,
                        &rule.name,
                    ));
                }
            }
        }

        // Remove duplicates
        predictions.sort_by(|a, b| {
            (&a.subject, &a.relation, &a.object).cmp(&(&b.subject, &b.relation, &b.object))
        });
        predictions.dedup_by(|a, b| {
            a.subject == b.subject && a.relation == b.relation && a.object == b.object
        });

        predictions
    }

    fn apply_transitive(
        &self,
        relation: &str,
        confidence: f64,
        rule_name: &str,
    ) -> Vec<PredictedTriple> {
        let mut predictions = Vec::new();

        for triple in &self.kg.triples {
            if triple.predicate == relation {
                if let RdfValue::Uri(ref b) = triple.object {
                    // Find (B, r, C)
                    for c in self.kg.get_objects(b, relation) {
                        // Skip if already exists
                        if !self.kg.get_objects(&triple.subject, relation).contains(&c) {
                            predictions.push(PredictedTriple {
                                subject: triple.subject.clone(),
                                relation: relation.to_string(),
                                object: c.to_string(),
                                confidence,
                                explanation: format!("Transitive inference via {}", rule_name),
                            });
                        }
                    }
                }
            }
        }

        predictions
    }

    fn apply_symmetric(
        &self,
        relation: &str,
        confidence: f64,
        rule_name: &str,
    ) -> Vec<PredictedTriple> {
        let mut predictions = Vec::new();

        for triple in &self.kg.triples {
            if triple.predicate == relation {
                if let RdfValue::Uri(ref obj) = triple.object {
                    // Check if reverse doesn't exist
                    if !self
                        .kg
                        .get_objects(obj, relation)
                        .contains(&triple.subject.as_str())
                    {
                        predictions.push(PredictedTriple {
                            subject: obj.clone(),
                            relation: relation.to_string(),
                            object: triple.subject.clone(),
                            confidence,
                            explanation: format!("Symmetric inference via {}", rule_name),
                        });
                    }
                }
            }
        }

        predictions
    }

    fn apply_inverse(
        &self,
        relation1: &str,
        relation2: &str,
        confidence: f64,
        rule_name: &str,
    ) -> Vec<PredictedTriple> {
        let mut predictions = Vec::new();

        for triple in &self.kg.triples {
            if triple.predicate == relation1 {
                if let RdfValue::Uri(ref obj) = triple.object {
                    // Check if inverse doesn't exist
                    if !self
                        .kg
                        .get_objects(obj, relation2)
                        .contains(&triple.subject.as_str())
                    {
                        predictions.push(PredictedTriple {
                            subject: obj.clone(),
                            relation: relation2.to_string(),
                            object: triple.subject.clone(),
                            confidence,
                            explanation: format!("Inverse inference via {}", rule_name),
                        });
                    }
                }
            }
        }

        predictions
    }

    fn apply_composition(
        &self,
        relation1: &str,
        relation2: &str,
        result_relation: &str,
        confidence: f64,
        rule_name: &str,
    ) -> Vec<PredictedTriple> {
        let mut predictions = Vec::new();

        for triple in &self.kg.triples {
            if triple.predicate == relation1 {
                if let RdfValue::Uri(ref b) = triple.object {
                    // Find (B, r2, C)
                    for c in self.kg.get_objects(b, relation2) {
                        // Check if result doesn't exist
                        if !self
                            .kg
                            .get_objects(&triple.subject, result_relation)
                            .contains(&c)
                        {
                            predictions.push(PredictedTriple {
                                subject: triple.subject.clone(),
                                relation: result_relation.to_string(),
                                object: c.to_string(),
                                confidence,
                                explanation: format!("Composition inference via {}", rule_name),
                            });
                        }
                    }
                }
            }
        }

        predictions
    }

    /// Gets the underlying knowledge graph.
    pub fn knowledge_graph(&self) -> &KnowledgeGraph {
        &self.kg
    }

    /// Gets the rules.
    pub fn rules(&self) -> &[InferenceRule] {
        &self.rules
    }
}

/// Statistics about completion results.
#[derive(Debug, Clone)]
pub struct CompletionStats {
    /// Number of predictions made
    pub total_predictions: usize,
    /// Average confidence of predictions
    pub avg_confidence: f64,
    /// Predictions by relation type
    pub by_relation: HashMap<String, usize>,
}

impl CompletionStats {
    /// Computes statistics from predictions.
    pub fn from_predictions(predictions: &[PredictedTriple]) -> Self {
        let mut by_relation: HashMap<String, usize> = HashMap::new();
        let mut total_confidence = 0.0;

        for pred in predictions {
            *by_relation.entry(pred.relation.clone()).or_insert(0) += 1;
            total_confidence += pred.confidence;
        }

        let avg_confidence = if predictions.is_empty() {
            0.0
        } else {
            total_confidence / predictions.len() as f64
        };

        Self {
            total_predictions: predictions.len(),
            avg_confidence,
            by_relation,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_graph() -> KnowledgeGraph {
        let triples = vec![
            Triple {
                subject: "A".to_string(),
                predicate: "legalis:references".to_string(),
                object: RdfValue::Uri("B".to_string()),
            },
            Triple {
                subject: "B".to_string(),
                predicate: "legalis:references".to_string(),
                object: RdfValue::Uri("C".to_string()),
            },
            Triple {
                subject: "X".to_string(),
                predicate: "legalis:amends".to_string(),
                object: RdfValue::Uri("Y".to_string()),
            },
        ];

        KnowledgeGraph::new(triples)
    }

    #[test]
    fn test_knowledge_graph_creation() {
        let kg = create_test_graph();
        assert_eq!(kg.entities().len(), 5); // A, B, C, X, Y
        assert!(kg.entities().contains("A"));
        assert!(kg.entities().contains("B"));
        assert!(kg.entities().contains("C"));
    }

    #[test]
    fn test_get_objects() {
        let kg = create_test_graph();
        let objs = kg.get_objects("A", "legalis:references");
        assert_eq!(objs.len(), 1);
        assert_eq!(objs[0], "B");
    }

    #[test]
    fn test_get_subjects() {
        let kg = create_test_graph();
        let subjs = kg.get_subjects("B", "legalis:references");
        assert_eq!(subjs.len(), 1);
        assert_eq!(subjs[0], "A");
    }

    #[test]
    fn test_transitive_completion() {
        let kg = create_test_graph();
        let completion = RuleBasedCompletion::new(kg);
        let predictions = completion.predict();

        // Should predict A -> C via transitivity
        assert!(
            predictions
                .iter()
                .any(|p| p.subject == "A" && p.object == "C" && p.relation == "legalis:references")
        );
    }

    #[test]
    fn test_inverse_completion() {
        let kg = create_test_graph();
        let completion = RuleBasedCompletion::new(kg);
        let predictions = completion.predict();

        // Should predict Y amendedBy X
        assert!(
            predictions
                .iter()
                .any(|p| p.subject == "Y" && p.object == "X" && p.relation == "legalis:amendedBy")
        );
    }

    #[test]
    fn test_symmetric_completion() {
        let triples = vec![Triple {
            subject: "A".to_string(),
            predicate: "legalis:relatedTo".to_string(),
            object: RdfValue::Uri("B".to_string()),
        }];

        let kg = KnowledgeGraph::new(triples);
        let completion = RuleBasedCompletion::new(kg);
        let predictions = completion.predict();

        // Should predict B relatedTo A
        assert!(
            predictions
                .iter()
                .any(|p| p.subject == "B" && p.object == "A" && p.relation == "legalis:relatedTo")
        );
    }

    #[test]
    fn test_composition_completion() {
        let triples = vec![
            Triple {
                subject: "A".to_string(),
                predicate: "legalis:dependsOn".to_string(),
                object: RdfValue::Uri("B".to_string()),
            },
            Triple {
                subject: "B".to_string(),
                predicate: "legalis:partOf".to_string(),
                object: RdfValue::Uri("C".to_string()),
            },
        ];

        let kg = KnowledgeGraph::new(triples);
        let completion = RuleBasedCompletion::new(kg);
        let predictions = completion.predict();

        // Should predict A dependsOn C via composition
        assert!(
            predictions
                .iter()
                .any(|p| p.subject == "A" && p.object == "C" && p.relation == "legalis:dependsOn")
        );
    }

    #[test]
    fn test_predicted_triple_to_triple() {
        let pred = PredictedTriple {
            subject: "A".to_string(),
            relation: "legalis:references".to_string(),
            object: "B".to_string(),
            confidence: 0.9,
            explanation: "Test".to_string(),
        };

        let triple = pred.to_triple();
        assert_eq!(triple.subject, "A");
        assert_eq!(triple.predicate, "legalis:references");
        assert!(matches!(triple.object, RdfValue::Uri(ref s) if s == "B"));
    }

    #[test]
    fn test_completion_stats() {
        let predictions = vec![
            PredictedTriple {
                subject: "A".to_string(),
                relation: "legalis:references".to_string(),
                object: "B".to_string(),
                confidence: 0.8,
                explanation: "Test".to_string(),
            },
            PredictedTriple {
                subject: "C".to_string(),
                relation: "legalis:amends".to_string(),
                object: "D".to_string(),
                confidence: 0.9,
                explanation: "Test".to_string(),
            },
        ];

        let stats = CompletionStats::from_predictions(&predictions);
        assert_eq!(stats.total_predictions, 2);
        assert!((stats.avg_confidence - 0.85).abs() < 0.001);
        assert_eq!(stats.by_relation.len(), 2);
    }

    #[test]
    fn test_add_triple() {
        let mut kg = create_test_graph();
        let initial_count = kg.triples.len();

        kg.add_triple(Triple {
            subject: "D".to_string(),
            predicate: "legalis:references".to_string(),
            object: RdfValue::Uri("E".to_string()),
        });

        assert_eq!(kg.triples.len(), initial_count + 1);
        assert!(kg.entities().contains("D"));
        assert!(kg.entities().contains("E"));
    }

    #[test]
    fn test_custom_rule() {
        let kg = create_test_graph();
        let mut completion = RuleBasedCompletion::new(kg);

        completion.add_rule(InferenceRule {
            name: "custom-rule".to_string(),
            pattern: RulePattern::Transitive {
                relation: "legalis:customRel".to_string(),
            },
            confidence: 0.75,
        });

        assert!(completion.rules().len() > 5); // Default rules + custom
    }

    #[test]
    fn test_no_duplicate_predictions() {
        let kg = create_test_graph();
        let completion = RuleBasedCompletion::new(kg);
        let predictions = completion.predict();

        // Check for duplicates
        for i in 0..predictions.len() {
            for j in (i + 1)..predictions.len() {
                assert!(
                    !(predictions[i].subject == predictions[j].subject
                        && predictions[i].relation == predictions[j].relation
                        && predictions[i].object == predictions[j].object),
                    "Found duplicate predictions"
                );
            }
        }
    }

    #[test]
    fn test_empty_graph() {
        let kg = KnowledgeGraph::new(Vec::new());
        let completion = RuleBasedCompletion::new(kg);
        let predictions = completion.predict();

        assert!(predictions.is_empty());
    }

    #[test]
    fn test_confidence_scores() {
        let kg = create_test_graph();
        let completion = RuleBasedCompletion::new(kg);
        let predictions = completion.predict();

        // All predictions should have valid confidence scores
        for pred in &predictions {
            assert!(pred.confidence >= 0.0 && pred.confidence <= 1.0);
        }
    }
}
