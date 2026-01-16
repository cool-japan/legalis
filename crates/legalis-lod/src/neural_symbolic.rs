//! Neural-symbolic reasoning integration for combining neural networks with symbolic reasoning.
//!
//! This module provides interfaces for integrating neural models with RDF knowledge graphs
//! for enhanced legal reasoning capabilities.

use crate::{RdfValue, Triple};
use std::collections::HashMap;

/// Neural-symbolic reasoner that combines embeddings with logical rules.
pub struct NeuralSymbolicReasoner {
    /// Rule templates learned from data
    rule_templates: Vec<RuleTemplate>,
    /// Confidence threshold for predictions
    confidence_threshold: f64,
}

/// A learned rule template (e.g., "X married-to Y => Y married-to X")
#[derive(Debug, Clone)]
pub struct RuleTemplate {
    pub head: TriplePattern,
    pub body: Vec<TriplePattern>,
    pub confidence: f64,
}

/// A pattern that can match triples
#[derive(Debug, Clone)]
pub struct TriplePattern {
    pub subject: PatternElement,
    pub predicate: PatternElement,
    pub object: PatternElement,
}

/// Elements of a pattern (variable or constant)
#[derive(Debug, Clone)]
pub enum PatternElement {
    Variable(String),
    Constant(String),
}

impl NeuralSymbolicReasoner {
    /// Creates a new neural-symbolic reasoner.
    pub fn new(confidence_threshold: f64) -> Self {
        Self {
            rule_templates: Vec::new(),
            confidence_threshold,
        }
    }

    /// Adds a learned rule template.
    pub fn add_rule(&mut self, rule: RuleTemplate) {
        if rule.confidence >= self.confidence_threshold {
            self.rule_templates.push(rule);
        }
    }

    /// Infers new triples from existing ones using learned rules.
    pub fn infer(&self, triples: &[Triple]) -> Vec<(Triple, f64)> {
        let mut inferred = Vec::new();

        for rule in &self.rule_templates {
            // Match body patterns against triples
            let bindings = self.match_patterns(&rule.body, triples);

            // For each binding, generate head triple
            for binding in bindings {
                if let Some(triple) = self.instantiate_pattern(&rule.head, &binding) {
                    inferred.push((triple, rule.confidence));
                }
            }
        }

        inferred
    }

    /// Returns the number of rules.
    pub fn num_rules(&self) -> usize {
        self.rule_templates.len()
    }

    /// Gets all rules with confidence above a threshold.
    pub fn get_rules_above(&self, threshold: f64) -> Vec<&RuleTemplate> {
        self.rule_templates
            .iter()
            .filter(|r| r.confidence >= threshold)
            .collect()
    }

    fn match_patterns(
        &self,
        patterns: &[TriplePattern],
        triples: &[Triple],
    ) -> Vec<HashMap<String, String>> {
        if patterns.is_empty() {
            return vec![HashMap::new()];
        }

        // Simple implementation: match first pattern, then recursively match rest
        let mut all_bindings = Vec::new();
        for triple in triples {
            if let Some(binding) = self.match_pattern(&patterns[0], triple, &HashMap::new()) {
                if patterns.len() == 1 {
                    all_bindings.push(binding);
                } else {
                    // Recursively match remaining patterns
                    for mut sub_binding in self.match_patterns(&patterns[1..], triples) {
                        // Merge bindings
                        let mut merged = binding.clone();
                        if self.compatible_bindings(&merged, &sub_binding) {
                            merged.extend(sub_binding.drain());
                            all_bindings.push(merged);
                        }
                    }
                }
            }
        }

        all_bindings
    }

    fn match_pattern(
        &self,
        pattern: &TriplePattern,
        triple: &Triple,
        binding: &HashMap<String, String>,
    ) -> Option<HashMap<String, String>> {
        let mut new_binding = binding.clone();

        // Match subject
        if !self.match_element(&pattern.subject, &triple.subject, &mut new_binding) {
            return None;
        }

        // Match predicate
        if !self.match_element(&pattern.predicate, &triple.predicate, &mut new_binding) {
            return None;
        }

        // Match object (extract URI from RdfValue)
        let obj_str = match &triple.object {
            RdfValue::Uri(uri) => uri.clone(),
            RdfValue::Literal(lit, _) => lit.clone(),
            RdfValue::TypedLiteral(lit, _) => lit.clone(),
            RdfValue::BlankNode(id) => format!("_:{}", id),
        };

        if !self.match_element(&pattern.object, &obj_str, &mut new_binding) {
            return None;
        }

        Some(new_binding)
    }

    fn match_element(
        &self,
        element: &PatternElement,
        value: &str,
        binding: &mut HashMap<String, String>,
    ) -> bool {
        match element {
            PatternElement::Variable(var) => {
                if let Some(bound_value) = binding.get(var) {
                    bound_value == value
                } else {
                    binding.insert(var.clone(), value.to_string());
                    true
                }
            }
            PatternElement::Constant(constant) => constant == value,
        }
    }

    fn compatible_bindings(
        &self,
        binding1: &HashMap<String, String>,
        binding2: &HashMap<String, String>,
    ) -> bool {
        for (var, val1) in binding1 {
            if let Some(val2) = binding2.get(var) {
                if val1 != val2 {
                    return false;
                }
            }
        }
        true
    }

    fn instantiate_pattern(
        &self,
        pattern: &TriplePattern,
        binding: &HashMap<String, String>,
    ) -> Option<Triple> {
        let subject = self.instantiate_element(&pattern.subject, binding)?;
        let predicate = self.instantiate_element(&pattern.predicate, binding)?;
        let object_str = self.instantiate_element(&pattern.object, binding)?;

        Some(Triple {
            subject,
            predicate,
            object: RdfValue::Uri(object_str),
        })
    }

    fn instantiate_element(
        &self,
        element: &PatternElement,
        binding: &HashMap<String, String>,
    ) -> Option<String> {
        match element {
            PatternElement::Variable(var) => binding.get(var).cloned(),
            PatternElement::Constant(constant) => Some(constant.clone()),
        }
    }
}

/// Neural link predictor for predicting missing links in knowledge graphs.
pub struct NeuralLinkPredictor {
    /// Predicted links with confidence scores
    predictions: Vec<(Triple, f64)>,
}

impl NeuralLinkPredictor {
    /// Creates a new link predictor.
    pub fn new() -> Self {
        Self {
            predictions: Vec::new(),
        }
    }

    /// Predicts missing links based on pattern analysis and co-occurrence.
    /// Uses heuristic-based prediction to find likely missing connections.
    /// In a production system, this would use trained neural embeddings.
    pub fn predict(&mut self, triples: &[Triple]) -> &[(Triple, f64)] {
        // Keep manually added predictions, only clear if empty or analyzing new data
        let manual_predictions = self.predictions.clone();
        self.predictions.clear();

        // Build co-occurrence statistics
        let mut subject_predicate_stats: HashMap<(String, String), Vec<String>> = HashMap::new();
        let mut predicate_object_stats: HashMap<(String, String), Vec<String>> = HashMap::new();

        for triple in triples {
            // Track subject-predicate pairs and their objects
            subject_predicate_stats
                .entry((triple.subject.clone(), triple.predicate.clone()))
                .or_default()
                .push(object_to_string(&triple.object));

            // Track predicate-object pairs and their subjects
            predicate_object_stats
                .entry((triple.predicate.clone(), object_to_string(&triple.object)))
                .or_default()
                .push(triple.subject.clone());
        }

        // Predict missing links based on similar patterns
        for triple in triples {
            // Look for symmetric patterns (e.g., if A relates-to B, predict B relates-to A)
            if let RdfValue::Uri(ref obj_uri) = triple.object {
                // Check if reverse relationship exists
                let reverse_exists = triples.iter().any(|t| {
                    t.subject == *obj_uri
                        && t.predicate == triple.predicate
                        && object_to_string(&t.object) == triple.subject
                });

                if !reverse_exists && is_symmetric_predicate(&triple.predicate) {
                    self.predictions.push((
                        Triple {
                            subject: obj_uri.clone(),
                            predicate: triple.predicate.clone(),
                            object: RdfValue::Uri(triple.subject.clone()),
                        },
                        0.75, // High confidence for symmetric predicates
                    ));
                }
            }

            // Predict based on common co-occurrence patterns
            if let Some(common_objects) =
                subject_predicate_stats.get(&(triple.subject.clone(), triple.predicate.clone()))
            {
                // If a subject-predicate pair commonly has multiple objects,
                // predict similar objects for similar subjects
                if common_objects.len() > 1 {
                    for obj in common_objects.iter().take(3) {
                        if obj != &object_to_string(&triple.object) {
                            // Check if this prediction already exists
                            let exists = triples.iter().any(|t| {
                                t.subject == triple.subject
                                    && t.predicate == triple.predicate
                                    && object_to_string(&t.object) == *obj
                            });

                            if !exists {
                                self.predictions.push((
                                    Triple {
                                        subject: triple.subject.clone(),
                                        predicate: triple.predicate.clone(),
                                        object: RdfValue::Uri(obj.clone()),
                                    },
                                    0.6, // Medium confidence for co-occurrence
                                ));
                            }
                        }
                    }
                }
            }
        }

        // Add back manual predictions
        self.predictions.extend(manual_predictions);

        // Deduplicate and sort by confidence
        self.predictions
            .sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        self.predictions.dedup_by(|a, b| {
            a.0.subject == b.0.subject
                && a.0.predicate == b.0.predicate
                && object_to_string(&a.0.object) == object_to_string(&b.0.object)
        });

        // Limit to top 100 predictions
        self.predictions.truncate(100);

        &self.predictions
    }

    /// Adds a predicted link.
    pub fn add_prediction(&mut self, triple: Triple, confidence: f64) {
        self.predictions.push((triple, confidence));
    }

    /// Gets predictions above a confidence threshold.
    pub fn get_predictions_above(&self, threshold: f64) -> Vec<&(Triple, f64)> {
        self.predictions
            .iter()
            .filter(|(_, conf)| *conf >= threshold)
            .collect()
    }

    /// Returns the number of predictions.
    pub fn num_predictions(&self) -> usize {
        self.predictions.len()
    }
}

impl Default for NeuralLinkPredictor {
    fn default() -> Self {
        Self::new()
    }
}

/// Knowledge graph completion using neural-symbolic methods.
pub struct KnowledgeGraphCompletion {
    reasoner: NeuralSymbolicReasoner,
    predictor: NeuralLinkPredictor,
}

impl KnowledgeGraphCompletion {
    /// Creates a new knowledge graph completion system.
    pub fn new(confidence_threshold: f64) -> Self {
        Self {
            reasoner: NeuralSymbolicReasoner::new(confidence_threshold),
            predictor: NeuralLinkPredictor::new(),
        }
    }

    /// Completes a knowledge graph by inferring missing triples.
    pub fn complete(&mut self, triples: &[Triple]) -> Vec<(Triple, f64, CompletionMethod)> {
        let mut completions = Vec::new();

        // Symbolic reasoning
        for (triple, conf) in self.reasoner.infer(triples) {
            completions.push((triple, conf, CompletionMethod::SymbolicReasoning));
        }

        // Neural prediction
        for (triple, conf) in self.predictor.predict(triples).iter() {
            completions.push((triple.clone(), *conf, CompletionMethod::NeuralPrediction));
        }

        completions
    }

    /// Adds a reasoning rule.
    pub fn add_rule(&mut self, rule: RuleTemplate) {
        self.reasoner.add_rule(rule);
    }

    /// Adds a neural prediction.
    pub fn add_prediction(&mut self, triple: Triple, confidence: f64) {
        self.predictor.add_prediction(triple, confidence);
    }
}

/// Method used for knowledge graph completion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionMethod {
    /// Completed using symbolic reasoning rules
    SymbolicReasoning,
    /// Completed using neural link prediction
    NeuralPrediction,
}

/// Helper function to convert RdfValue to string for comparison
fn object_to_string(value: &RdfValue) -> String {
    match value {
        RdfValue::Uri(uri) => uri.clone(),
        RdfValue::Literal(lit, None) => lit.clone(),
        RdfValue::Literal(lit, Some(lang)) => format!("{}@{}", lit, lang),
        RdfValue::TypedLiteral(lit, dtype) => format!("{}^^{}", lit, dtype),
        RdfValue::BlankNode(id) => format!("_:{}", id),
    }
}

/// Checks if a predicate is likely to be symmetric
fn is_symmetric_predicate(predicate: &str) -> bool {
    let symmetric_predicates = [
        "owl:sameAs",
        "skos:related",
        "legalis:relatedTo",
        "foaf:knows",
        "sibling",
        "colleague",
        "spouse",
        "partner",
    ];

    symmetric_predicates.iter().any(|&p| predicate.contains(p))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_triple(s: &str, p: &str, o: &str) -> Triple {
        Triple {
            subject: s.to_string(),
            predicate: p.to_string(),
            object: RdfValue::Uri(o.to_string()),
        }
    }

    #[test]
    fn test_neural_symbolic_reasoner_new() {
        let reasoner = NeuralSymbolicReasoner::new(0.8);
        assert_eq!(reasoner.confidence_threshold, 0.8);
        assert_eq!(reasoner.num_rules(), 0);
    }

    #[test]
    fn test_add_rule() {
        let mut reasoner = NeuralSymbolicReasoner::new(0.8);
        let rule = RuleTemplate {
            head: TriplePattern {
                subject: PatternElement::Variable("X".to_string()),
                predicate: PatternElement::Constant("ex:knows".to_string()),
                object: PatternElement::Variable("Y".to_string()),
            },
            body: vec![],
            confidence: 0.9,
        };

        reasoner.add_rule(rule);
        assert_eq!(reasoner.num_rules(), 1);
    }

    #[test]
    fn test_add_rule_below_threshold() {
        let mut reasoner = NeuralSymbolicReasoner::new(0.8);
        let rule = RuleTemplate {
            head: TriplePattern {
                subject: PatternElement::Variable("X".to_string()),
                predicate: PatternElement::Constant("ex:knows".to_string()),
                object: PatternElement::Variable("Y".to_string()),
            },
            body: vec![],
            confidence: 0.5,
        };

        reasoner.add_rule(rule);
        assert_eq!(reasoner.num_rules(), 0);
    }

    #[test]
    fn test_neural_link_predictor() {
        let mut predictor = NeuralLinkPredictor::new();
        assert_eq!(predictor.num_predictions(), 0);

        predictor.add_prediction(sample_triple("A", "knows", "B"), 0.9);
        assert_eq!(predictor.num_predictions(), 1);
    }

    #[test]
    fn test_get_predictions_above() {
        let mut predictor = NeuralLinkPredictor::new();
        predictor.add_prediction(sample_triple("A", "knows", "B"), 0.9);
        predictor.add_prediction(sample_triple("C", "knows", "D"), 0.5);

        let high_conf = predictor.get_predictions_above(0.7);
        assert_eq!(high_conf.len(), 1);
    }

    #[test]
    fn test_knowledge_graph_completion() {
        let mut completion = KnowledgeGraphCompletion::new(0.8);

        let rule = RuleTemplate {
            head: TriplePattern {
                subject: PatternElement::Variable("X".to_string()),
                predicate: PatternElement::Constant("ex:knows".to_string()),
                object: PatternElement::Variable("Y".to_string()),
            },
            body: vec![],
            confidence: 0.9,
        };

        completion.add_rule(rule);
        completion.add_prediction(sample_triple("A", "likes", "B"), 0.85);

        let triples = vec![sample_triple("Alice", "knows", "Bob")];
        let completions = completion.complete(&triples);

        assert!(!completions.is_empty());
    }

    #[test]
    fn test_get_rules_above() {
        let mut reasoner = NeuralSymbolicReasoner::new(0.5);

        let rule1 = RuleTemplate {
            head: TriplePattern {
                subject: PatternElement::Variable("X".to_string()),
                predicate: PatternElement::Constant("ex:knows".to_string()),
                object: PatternElement::Variable("Y".to_string()),
            },
            body: vec![],
            confidence: 0.9,
        };

        let rule2 = RuleTemplate {
            head: TriplePattern {
                subject: PatternElement::Variable("X".to_string()),
                predicate: PatternElement::Constant("ex:likes".to_string()),
                object: PatternElement::Variable("Y".to_string()),
            },
            body: vec![],
            confidence: 0.6,
        };

        reasoner.add_rule(rule1);
        reasoner.add_rule(rule2);

        let high_conf_rules = reasoner.get_rules_above(0.8);
        assert_eq!(high_conf_rules.len(), 1);
    }

    #[test]
    fn test_simple_inference() {
        let mut reasoner = NeuralSymbolicReasoner::new(0.5);

        // Rule: if X knows Y, then Y knows X (symmetric relation)
        let rule = RuleTemplate {
            head: TriplePattern {
                subject: PatternElement::Variable("Y".to_string()),
                predicate: PatternElement::Constant("ex:knows".to_string()),
                object: PatternElement::Variable("X".to_string()),
            },
            body: vec![TriplePattern {
                subject: PatternElement::Variable("X".to_string()),
                predicate: PatternElement::Constant("ex:knows".to_string()),
                object: PatternElement::Variable("Y".to_string()),
            }],
            confidence: 0.95,
        };

        reasoner.add_rule(rule);

        let triples = vec![sample_triple("Alice", "ex:knows", "Bob")];
        let inferred = reasoner.infer(&triples);

        assert!(!inferred.is_empty());
    }

    #[test]
    fn test_pattern_element() {
        let var = PatternElement::Variable("X".to_string());
        let const_elem = PatternElement::Constant("ex:knows".to_string());

        assert!(matches!(var, PatternElement::Variable(_)));
        assert!(matches!(const_elem, PatternElement::Constant(_)));
    }

    #[test]
    fn test_completion_method() {
        let method1 = CompletionMethod::SymbolicReasoning;
        let method2 = CompletionMethod::NeuralPrediction;

        assert_eq!(method1, CompletionMethod::SymbolicReasoning);
        assert_ne!(method1, method2);
    }
}
