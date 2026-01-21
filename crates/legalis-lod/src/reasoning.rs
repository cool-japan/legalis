//! OWL 2 RL reasoning and legal-specific inference rules.
//!
//! This module provides reasoning capabilities for RDF graphs, including
//! OWL 2 RL reasoning and custom legal domain inference rules.

use crate::{LodResult, RdfValue, Triple};
use std::collections::{HashMap, HashSet};

/// Reasoning engine for RDF triples.
#[derive(Debug)]
pub struct ReasoningEngine {
    /// Inference rules to apply
    rules: Vec<Box<dyn InferenceRule>>,
    /// Maximum iterations for fixed-point reasoning
    max_iterations: usize,
}

impl Default for ReasoningEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl ReasoningEngine {
    /// Creates a new reasoning engine with default rules.
    pub fn new() -> Self {
        let mut engine = Self {
            rules: Vec::new(),
            max_iterations: 10,
        };

        // Add OWL 2 RL rules
        engine.add_rule(Box::new(TransitivityRule));
        engine.add_rule(Box::new(SymmetricPropertyRule));
        engine.add_rule(Box::new(InversePropertyRule));
        engine.add_rule(Box::new(SubClassRule));
        engine.add_rule(Box::new(SubPropertyRule));

        // Add legal-specific rules
        engine.add_rule(Box::new(LegalInheritanceRule));
        engine.add_rule(Box::new(TemporalReasoningRule));
        engine.add_rule(Box::new(JurisdictionInheritanceRule));

        engine
    }

    /// Adds an inference rule to the engine.
    pub fn add_rule(&mut self, rule: Box<dyn InferenceRule>) {
        self.rules.push(rule);
    }

    /// Sets the maximum number of reasoning iterations.
    pub fn set_max_iterations(&mut self, max_iterations: usize) {
        self.max_iterations = max_iterations;
    }

    /// Applies reasoning to a set of triples, returning inferred triples.
    pub fn reason(&self, triples: &[Triple]) -> LodResult<Vec<Triple>> {
        let mut all_triples: HashSet<String> = triples.iter().map(Self::triple_key).collect();

        let mut inferred = Vec::new();
        let mut iteration = 0;

        loop {
            if iteration >= self.max_iterations {
                break;
            }

            let mut new_inferences = Vec::new();

            // Apply each rule
            for rule in &self.rules {
                let rule_inferences = rule.apply(triples, &inferred)?;
                for triple in rule_inferences {
                    let key = Self::triple_key(&triple);
                    if all_triples.insert(key) {
                        new_inferences.push(triple);
                    }
                }
            }

            if new_inferences.is_empty() {
                break;
            }

            inferred.extend(new_inferences);
            iteration += 1;
        }

        Ok(inferred)
    }

    /// Applies reasoning and returns all triples (original + inferred).
    pub fn reason_all(&self, triples: &[Triple]) -> LodResult<Vec<Triple>> {
        let mut all = triples.to_vec();
        let inferred = self.reason(triples)?;
        all.extend(inferred);
        Ok(all)
    }

    /// Generates explanations for inferred triples.
    pub fn explain(&self, triple: &Triple, original_triples: &[Triple]) -> Vec<Explanation> {
        let mut explanations = Vec::new();

        for rule in &self.rules {
            if let Some(explanation) = rule.explain(triple, original_triples) {
                explanations.push(explanation);
            }
        }

        explanations
    }

    fn triple_key(triple: &Triple) -> String {
        format!(
            "{} {} {:?}",
            triple.subject, triple.predicate, triple.object
        )
    }
}

/// Explanation for an inferred triple.
#[derive(Debug, Clone)]
pub struct Explanation {
    /// The rule that generated the inference
    pub rule_name: String,
    /// Description of how the inference was made
    pub description: String,
    /// Source triples that led to the inference
    pub source_triples: Vec<String>,
}

/// Trait for inference rules.
pub trait InferenceRule: std::fmt::Debug {
    /// Applies the rule to generate new triples.
    fn apply(&self, triples: &[Triple], inferred: &[Triple]) -> LodResult<Vec<Triple>>;

    /// Explains how a triple was inferred (if applicable).
    fn explain(&self, triple: &Triple, original_triples: &[Triple]) -> Option<Explanation>;

    /// Returns the name of the rule.
    fn name(&self) -> &str;
}

/// OWL 2 RL: Transitivity rule (e.g., if A subClassOf B and B subClassOf C, then A subClassOf C).
#[derive(Debug)]
struct TransitivityRule;

impl InferenceRule for TransitivityRule {
    fn apply(&self, triples: &[Triple], _inferred: &[Triple]) -> LodResult<Vec<Triple>> {
        let mut inferred = Vec::new();
        let transitive_props = vec!["rdfs:subClassOf", "rdfs:subPropertyOf"];

        for prop in transitive_props {
            let mut graph: HashMap<String, Vec<String>> = HashMap::new();

            // Build graph
            for triple in triples {
                if triple.predicate == prop
                    && let RdfValue::Uri(obj) = &triple.object
                {
                    graph
                        .entry(triple.subject.clone())
                        .or_default()
                        .push(obj.clone());
                }
            }

            // Compute transitive closure
            for start in graph.keys() {
                let mut visited = HashSet::new();
                let mut stack = vec![start.clone()];

                while let Some(current) = stack.pop() {
                    if visited.insert(current.clone())
                        && let Some(neighbors) = graph.get(&current)
                    {
                        for neighbor in neighbors {
                            // Add transitive triple
                            if neighbor != start && !visited.contains(neighbor) {
                                inferred.push(Triple {
                                    subject: start.clone(),
                                    predicate: prop.to_string(),
                                    object: RdfValue::Uri(neighbor.clone()),
                                });
                                stack.push(neighbor.clone());
                            }
                        }
                    }
                }
            }
        }

        Ok(inferred)
    }

    fn explain(&self, triple: &Triple, _original_triples: &[Triple]) -> Option<Explanation> {
        if triple.predicate == "rdfs:subClassOf" || triple.predicate == "rdfs:subPropertyOf" {
            Some(Explanation {
                rule_name: self.name().to_string(),
                description: "Inferred through transitivity".to_string(),
                source_triples: Vec::new(),
            })
        } else {
            None
        }
    }

    fn name(&self) -> &str {
        "TransitivityRule"
    }
}

/// OWL 2 RL: Symmetric property rule.
#[derive(Debug)]
struct SymmetricPropertyRule;

impl InferenceRule for SymmetricPropertyRule {
    fn apply(&self, triples: &[Triple], _inferred: &[Triple]) -> LodResult<Vec<Triple>> {
        let mut inferred = Vec::new();
        let symmetric_props = ["owl:sameAs", "skos:related"];

        let symmetric_set: HashSet<&str> = symmetric_props.iter().copied().collect();

        for triple in triples {
            if symmetric_set.contains(triple.predicate.as_str())
                && let RdfValue::Uri(obj) = &triple.object
            {
                inferred.push(Triple {
                    subject: obj.clone(),
                    predicate: triple.predicate.clone(),
                    object: RdfValue::Uri(triple.subject.clone()),
                });
            }
        }

        Ok(inferred)
    }

    fn explain(&self, triple: &Triple, _original_triples: &[Triple]) -> Option<Explanation> {
        let symmetric_props = ["owl:sameAs", "skos:related"];
        if symmetric_props.contains(&triple.predicate.as_str()) {
            Some(Explanation {
                rule_name: self.name().to_string(),
                description: "Inferred from symmetric property".to_string(),
                source_triples: Vec::new(),
            })
        } else {
            None
        }
    }

    fn name(&self) -> &str {
        "SymmetricPropertyRule"
    }
}

/// OWL 2 RL: Inverse property rule.
#[derive(Debug)]
struct InversePropertyRule;

impl InferenceRule for InversePropertyRule {
    fn apply(&self, _triples: &[Triple], _inferred: &[Triple]) -> LodResult<Vec<Triple>> {
        // Simplified implementation - would need inverse property definitions
        Ok(Vec::new())
    }

    fn explain(&self, _triple: &Triple, _original_triples: &[Triple]) -> Option<Explanation> {
        None
    }

    fn name(&self) -> &str {
        "InversePropertyRule"
    }
}

/// OWL 2 RL: Subclass rule.
#[derive(Debug)]
struct SubClassRule;

impl InferenceRule for SubClassRule {
    fn apply(&self, triples: &[Triple], _inferred: &[Triple]) -> LodResult<Vec<Triple>> {
        let mut inferred = Vec::new();

        // Build subclass hierarchy
        let mut subclass_map: HashMap<String, Vec<String>> = HashMap::new();
        for triple in triples {
            if triple.predicate == "rdfs:subClassOf"
                && let RdfValue::Uri(superclass) = &triple.object
            {
                subclass_map
                    .entry(triple.subject.clone())
                    .or_default()
                    .push(superclass.clone());
            }
        }

        // Apply subclass inference
        for triple in triples {
            if triple.predicate == "rdf:type"
                && let RdfValue::Uri(class) = &triple.object
                && let Some(superclasses) = subclass_map.get(class)
            {
                for superclass in superclasses {
                    inferred.push(Triple {
                        subject: triple.subject.clone(),
                        predicate: "rdf:type".to_string(),
                        object: RdfValue::Uri(superclass.clone()),
                    });
                }
            }
        }

        Ok(inferred)
    }

    fn explain(&self, triple: &Triple, _original_triples: &[Triple]) -> Option<Explanation> {
        if triple.predicate == "rdf:type" {
            Some(Explanation {
                rule_name: self.name().to_string(),
                description: "Inferred from subclass relationship".to_string(),
                source_triples: Vec::new(),
            })
        } else {
            None
        }
    }

    fn name(&self) -> &str {
        "SubClassRule"
    }
}

/// OWL 2 RL: Subproperty rule.
#[derive(Debug)]
struct SubPropertyRule;

impl InferenceRule for SubPropertyRule {
    fn apply(&self, triples: &[Triple], _inferred: &[Triple]) -> LodResult<Vec<Triple>> {
        let mut inferred = Vec::new();

        // Build subproperty hierarchy
        let mut subprop_map: HashMap<String, Vec<String>> = HashMap::new();
        for triple in triples {
            if triple.predicate == "rdfs:subPropertyOf"
                && let RdfValue::Uri(superprop) = &triple.object
            {
                subprop_map
                    .entry(triple.subject.clone())
                    .or_default()
                    .push(superprop.clone());
            }
        }

        // Apply subproperty inference
        for triple in triples {
            if let Some(superprops) = subprop_map.get(&triple.predicate) {
                for superprop in superprops {
                    inferred.push(Triple {
                        subject: triple.subject.clone(),
                        predicate: superprop.clone(),
                        object: triple.object.clone(),
                    });
                }
            }
        }

        Ok(inferred)
    }

    fn explain(&self, _triple: &Triple, _original_triples: &[Triple]) -> Option<Explanation> {
        Some(Explanation {
            rule_name: self.name().to_string(),
            description: "Inferred from subproperty relationship".to_string(),
            source_triples: Vec::new(),
        })
    }

    fn name(&self) -> &str {
        "SubPropertyRule"
    }
}

/// Legal-specific: Inheritance of statute properties through references.
#[derive(Debug)]
struct LegalInheritanceRule;

impl InferenceRule for LegalInheritanceRule {
    fn apply(&self, triples: &[Triple], _inferred: &[Triple]) -> LodResult<Vec<Triple>> {
        let mut inferred = Vec::new();

        // Find statute references
        for triple in triples {
            if (triple.predicate == "legalis:references"
                || triple.predicate == "dcterms:references")
                && let RdfValue::Uri(referenced) = &triple.object
            {
                // Inherit jurisdiction
                for t in triples {
                    if t.subject == *referenced && t.predicate == "eli:jurisdiction" {
                        // Add inferred jurisdiction if not explicitly set
                        let has_jurisdiction = triples.iter().any(|existing| {
                            existing.subject == triple.subject
                                && existing.predicate == "eli:jurisdiction"
                        });

                        if !has_jurisdiction {
                            inferred.push(Triple {
                                subject: triple.subject.clone(),
                                predicate: "eli:jurisdiction".to_string(),
                                object: t.object.clone(),
                            });
                        }
                    }
                }
            }
        }

        Ok(inferred)
    }

    fn explain(&self, triple: &Triple, _original_triples: &[Triple]) -> Option<Explanation> {
        if triple.predicate == "eli:jurisdiction" {
            Some(Explanation {
                rule_name: self.name().to_string(),
                description: "Jurisdiction inherited from referenced statute".to_string(),
                source_triples: Vec::new(),
            })
        } else {
            None
        }
    }

    fn name(&self) -> &str {
        "LegalInheritanceRule"
    }
}

/// Legal-specific: Temporal reasoning for effective dates.
#[derive(Debug)]
struct TemporalReasoningRule;

impl InferenceRule for TemporalReasoningRule {
    fn apply(&self, _triples: &[Triple], _inferred: &[Triple]) -> LodResult<Vec<Triple>> {
        // Simplified - would check effective/expiry dates and infer eli:in_force
        Ok(Vec::new())
    }

    fn explain(&self, _triple: &Triple, _original_triples: &[Triple]) -> Option<Explanation> {
        None
    }

    fn name(&self) -> &str {
        "TemporalReasoningRule"
    }
}

/// Legal-specific: Jurisdiction hierarchy inference.
#[derive(Debug)]
struct JurisdictionInheritanceRule;

impl InferenceRule for JurisdictionInheritanceRule {
    fn apply(&self, _triples: &[Triple], _inferred: &[Triple]) -> LodResult<Vec<Triple>> {
        // Simplified - would infer jurisdiction relationships
        Ok(Vec::new())
    }

    fn explain(&self, _triple: &Triple, _original_triples: &[Triple]) -> Option<Explanation> {
        None
    }

    fn name(&self) -> &str {
        "JurisdictionInheritanceRule"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reasoning_engine() {
        let engine = ReasoningEngine::new();
        assert!(!engine.rules.is_empty());
    }

    #[test]
    fn test_transitivity_rule() {
        let triples = vec![
            Triple {
                subject: "A".to_string(),
                predicate: "rdfs:subClassOf".to_string(),
                object: RdfValue::Uri("B".to_string()),
            },
            Triple {
                subject: "B".to_string(),
                predicate: "rdfs:subClassOf".to_string(),
                object: RdfValue::Uri("C".to_string()),
            },
        ];

        let engine = ReasoningEngine::new();
        let inferred = engine.reason(&triples).unwrap();

        // Should infer A subClassOf C
        assert!(inferred.iter().any(|t| {
            t.subject == "A"
                && t.predicate == "rdfs:subClassOf"
                && matches!(&t.object, RdfValue::Uri(u) if u == "C")
        }));
    }

    #[test]
    fn test_symmetric_property_rule() {
        let triples = vec![Triple {
            subject: "A".to_string(),
            predicate: "owl:sameAs".to_string(),
            object: RdfValue::Uri("B".to_string()),
        }];

        let rule = SymmetricPropertyRule;
        let inferred = rule.apply(&triples, &[]).unwrap();

        // Should infer B sameAs A
        assert!(inferred.iter().any(|t| {
            t.subject == "B"
                && t.predicate == "owl:sameAs"
                && matches!(&t.object, RdfValue::Uri(u) if u == "A")
        }));
    }

    #[test]
    fn test_subclass_rule() {
        let triples = vec![
            Triple {
                subject: "legalis:Statute".to_string(),
                predicate: "rdfs:subClassOf".to_string(),
                object: RdfValue::Uri("eli:LegalResource".to_string()),
            },
            Triple {
                subject: "statute:1".to_string(),
                predicate: "rdf:type".to_string(),
                object: RdfValue::Uri("legalis:Statute".to_string()),
            },
        ];

        let rule = SubClassRule;
        let inferred = rule.apply(&triples, &[]).unwrap();

        // Should infer statute:1 type LegalResource
        assert!(inferred.iter().any(|t| {
            t.subject == "statute:1"
                && t.predicate == "rdf:type"
                && matches!(&t.object, RdfValue::Uri(u) if u == "eli:LegalResource")
        }));
    }

    #[test]
    fn test_reasoning_fixed_point() {
        let triples = vec![
            Triple {
                subject: "A".to_string(),
                predicate: "rdfs:subClassOf".to_string(),
                object: RdfValue::Uri("B".to_string()),
            },
            Triple {
                subject: "B".to_string(),
                predicate: "rdfs:subClassOf".to_string(),
                object: RdfValue::Uri("C".to_string()),
            },
        ];

        let engine = ReasoningEngine::new();
        let inferred = engine.reason(&triples).unwrap();

        // Should reach fixed point and not generate duplicates
        let a_to_c_count = inferred
            .iter()
            .filter(|t| {
                t.subject == "A"
                    && t.predicate == "rdfs:subClassOf"
                    && matches!(&t.object, RdfValue::Uri(u) if u == "C")
            })
            .count();

        assert_eq!(a_to_c_count, 1);
    }

    #[test]
    fn test_explanation_generation() {
        let triples = vec![Triple {
            subject: "A".to_string(),
            predicate: "owl:sameAs".to_string(),
            object: RdfValue::Uri("B".to_string()),
        }];

        let engine = ReasoningEngine::new();

        let inferred_triple = Triple {
            subject: "B".to_string(),
            predicate: "owl:sameAs".to_string(),
            object: RdfValue::Uri("A".to_string()),
        };

        let explanations = engine.explain(&inferred_triple, &triples);
        assert!(!explanations.is_empty());
    }

    #[test]
    fn test_legal_inheritance_rule() {
        let triples = vec![
            Triple {
                subject: "statute:1".to_string(),
                predicate: "legalis:references".to_string(),
                object: RdfValue::Uri("statute:2".to_string()),
            },
            Triple {
                subject: "statute:2".to_string(),
                predicate: "eli:jurisdiction".to_string(),
                object: RdfValue::string("EU"),
            },
        ];

        let rule = LegalInheritanceRule;
        let inferred = rule.apply(&triples, &[]).unwrap();

        // Should inherit jurisdiction from referenced statute
        assert!(
            inferred
                .iter()
                .any(|t| { t.subject == "statute:1" && t.predicate == "eli:jurisdiction" })
        );
    }

    #[test]
    fn test_reason_all() {
        let triples = vec![Triple {
            subject: "A".to_string(),
            predicate: "owl:sameAs".to_string(),
            object: RdfValue::Uri("B".to_string()),
        }];

        let engine = ReasoningEngine::new();
        let all = engine.reason_all(&triples).unwrap();

        // Should include both original and inferred
        assert!(all.len() > triples.len());
    }
}
