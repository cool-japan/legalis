//! Ontology learning from legal text.
//!
//! This module provides functionality for automatically learning ontological
//! structures (classes, properties, hierarchies) from legal text.

use crate::{RdfValue, Triple};
use std::collections::HashMap;

/// A learned ontology class.
#[derive(Debug, Clone, PartialEq)]
pub struct LearnedClass {
    /// Class URI
    pub uri: String,
    /// Class label
    pub label: String,
    /// Description/definition
    pub description: Option<String>,
    /// Parent classes
    pub parents: Vec<String>,
    /// Example instances
    pub examples: Vec<String>,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
}

/// A learned ontology property.
#[derive(Debug, Clone, PartialEq)]
pub struct LearnedProperty {
    /// Property URI
    pub uri: String,
    /// Property label
    pub label: String,
    /// Description
    pub description: Option<String>,
    /// Domain (subject class)
    pub domain: Option<String>,
    /// Range (object class)
    pub range: Option<String>,
    /// Property type (ObjectProperty, DatatypeProperty, etc.)
    pub property_type: PropertyType,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
}

/// Type of OWL property.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropertyType {
    /// Object property (relates two resources)
    ObjectProperty,
    /// Datatype property (relates resource to literal)
    DatatypeProperty,
    /// Annotation property
    AnnotationProperty,
}

/// Learned ontology structure.
#[derive(Debug, Clone)]
pub struct LearnedOntology {
    /// Base URI for the ontology
    pub base_uri: String,
    /// Ontology title
    pub title: String,
    /// Learned classes
    pub classes: Vec<LearnedClass>,
    /// Learned properties
    pub properties: Vec<LearnedProperty>,
    /// Class hierarchy (subclass -> superclass)
    pub class_hierarchy: HashMap<String, Vec<String>>,
}

impl LearnedOntology {
    /// Creates a new learned ontology.
    pub fn new(base_uri: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            base_uri: base_uri.into(),
            title: title.into(),
            classes: Vec::new(),
            properties: Vec::new(),
            class_hierarchy: HashMap::new(),
        }
    }

    /// Adds a class to the ontology.
    pub fn add_class(&mut self, class: LearnedClass) {
        // Update hierarchy
        for parent in &class.parents {
            self.class_hierarchy
                .entry(class.uri.clone())
                .or_default()
                .push(parent.clone());
        }

        self.classes.push(class);
    }

    /// Adds a property to the ontology.
    pub fn add_property(&mut self, property: LearnedProperty) {
        self.properties.push(property);
    }

    /// Converts the learned ontology to RDF triples.
    pub fn to_triples(&self) -> Vec<Triple> {
        let mut triples = Vec::new();

        // Ontology metadata
        let ontology_uri = format!("{}ontology", self.base_uri);
        triples.push(Triple {
            subject: ontology_uri.clone(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("owl:Ontology".to_string()),
        });

        triples.push(Triple {
            subject: ontology_uri,
            predicate: "rdfs:label".to_string(),
            object: RdfValue::string(&self.title),
        });

        // Classes
        for class in &self.classes {
            let class_uri = format!("{}{}", self.base_uri, class.uri);

            triples.push(Triple {
                subject: class_uri.clone(),
                predicate: "rdf:type".to_string(),
                object: RdfValue::Uri("owl:Class".to_string()),
            });

            triples.push(Triple {
                subject: class_uri.clone(),
                predicate: "rdfs:label".to_string(),
                object: RdfValue::string(&class.label),
            });

            if let Some(ref desc) = class.description {
                triples.push(Triple {
                    subject: class_uri.clone(),
                    predicate: "rdfs:comment".to_string(),
                    object: RdfValue::string(desc),
                });
            }

            // Subclass relationships
            for parent in &class.parents {
                triples.push(Triple {
                    subject: class_uri.clone(),
                    predicate: "rdfs:subClassOf".to_string(),
                    object: RdfValue::Uri(format!("{}{}", self.base_uri, parent)),
                });
            }

            // Confidence as annotation
            triples.push(Triple {
                subject: class_uri,
                predicate: "legalis:confidence".to_string(),
                object: RdfValue::TypedLiteral(
                    class.confidence.to_string(),
                    "xsd:double".to_string(),
                ),
            });
        }

        // Properties
        for property in &self.properties {
            let prop_uri = format!("{}{}", self.base_uri, property.uri);

            let prop_type = match property.property_type {
                PropertyType::ObjectProperty => "owl:ObjectProperty",
                PropertyType::DatatypeProperty => "owl:DatatypeProperty",
                PropertyType::AnnotationProperty => "owl:AnnotationProperty",
            };

            triples.push(Triple {
                subject: prop_uri.clone(),
                predicate: "rdf:type".to_string(),
                object: RdfValue::Uri(prop_type.to_string()),
            });

            triples.push(Triple {
                subject: prop_uri.clone(),
                predicate: "rdfs:label".to_string(),
                object: RdfValue::string(&property.label),
            });

            if let Some(ref desc) = property.description {
                triples.push(Triple {
                    subject: prop_uri.clone(),
                    predicate: "rdfs:comment".to_string(),
                    object: RdfValue::string(desc),
                });
            }

            if let Some(ref domain) = property.domain {
                triples.push(Triple {
                    subject: prop_uri.clone(),
                    predicate: "rdfs:domain".to_string(),
                    object: RdfValue::Uri(format!("{}{}", self.base_uri, domain)),
                });
            }

            if let Some(ref range) = property.range {
                triples.push(Triple {
                    subject: prop_uri.clone(),
                    predicate: "rdfs:range".to_string(),
                    object: RdfValue::Uri(format!("{}{}", self.base_uri, range)),
                });
            }

            triples.push(Triple {
                subject: prop_uri,
                predicate: "legalis:confidence".to_string(),
                object: RdfValue::TypedLiteral(
                    property.confidence.to_string(),
                    "xsd:double".to_string(),
                ),
            });
        }

        triples
    }

    /// Gets statistics about the learned ontology.
    pub fn stats(&self) -> OntologyStats {
        OntologyStats {
            total_classes: self.classes.len(),
            total_properties: self.properties.len(),
            avg_class_confidence: if self.classes.is_empty() {
                0.0
            } else {
                self.classes.iter().map(|c| c.confidence).sum::<f64>() / self.classes.len() as f64
            },
            avg_property_confidence: if self.properties.is_empty() {
                0.0
            } else {
                self.properties.iter().map(|p| p.confidence).sum::<f64>()
                    / self.properties.len() as f64
            },
        }
    }
}

/// Statistics about learned ontology.
#[derive(Debug, Clone)]
pub struct OntologyStats {
    /// Total number of classes
    pub total_classes: usize,
    /// Total number of properties
    pub total_properties: usize,
    /// Average confidence of classes
    pub avg_class_confidence: f64,
    /// Average confidence of properties
    pub avg_property_confidence: f64,
}

/// Pattern-based ontology learner.
pub struct PatternBasedLearner {
    /// Base URI for generated ontology
    base_uri: String,
    /// Minimum confidence threshold
    confidence_threshold: f64,
    /// Learned classes
    classes: HashMap<String, LearnedClass>,
    /// Learned properties
    properties: HashMap<String, LearnedProperty>,
}

impl PatternBasedLearner {
    /// Creates a new pattern-based learner.
    pub fn new(base_uri: impl Into<String>) -> Self {
        Self {
            base_uri: base_uri.into(),
            confidence_threshold: 0.5,
            classes: HashMap::new(),
            properties: HashMap::new(),
        }
    }

    /// Sets the confidence threshold.
    pub fn with_threshold(mut self, threshold: f64) -> Self {
        self.confidence_threshold = threshold;
        self
    }

    /// Learns ontology from text.
    pub fn learn(&mut self, text: &str) -> LearnedOntology {
        // Extract classes from definitions
        self.extract_classes(text);

        // Extract properties from relations
        self.extract_properties(text);

        // Build ontology
        let mut ontology = LearnedOntology::new(self.base_uri.clone(), "Learned Legal Ontology");

        for class in self.classes.values() {
            if class.confidence >= self.confidence_threshold {
                ontology.add_class(class.clone());
            }
        }

        for property in self.properties.values() {
            if property.confidence >= self.confidence_threshold {
                ontology.add_property(property.clone());
            }
        }

        ontology
    }

    fn extract_classes(&mut self, text: &str) {
        // Look for definition patterns: "X means", "X is defined as", "X shall mean"
        let sentences: Vec<&str> = text.split('.').collect();

        for sentence in sentences {
            if sentence.contains("means") || sentence.contains("defined as") {
                self.extract_class_from_definition(sentence);
            }

            // Look for class hierarchies: "X is a type of Y", "X is a Y"
            if sentence.contains("is a type of") || sentence.contains("is a") {
                self.extract_class_hierarchy(sentence);
            }
        }
    }

    fn extract_class_from_definition(&mut self, sentence: &str) {
        // Simple pattern: extract quoted terms or capitalized terms before "means"
        if let Some(pos) = sentence.find("means") {
            let before = &sentence[..pos];

            // Try to extract quoted term
            if let Some(class_name) = self.extract_quoted(before) {
                self.add_class(
                    class_name.clone(),
                    class_name,
                    Some(sentence.trim().to_string()),
                    0.8,
                );
            }
        }
    }

    fn extract_class_hierarchy(&mut self, sentence: &str) {
        // Pattern: "X is a type of Y" or "X is a Y"
        if let Some(pos) = sentence.find("is a type of") {
            let parts: Vec<&str> = sentence[..pos].split_whitespace().collect();
            if let Some(subclass) = parts.last() {
                let remaining = &sentence[pos + 12..]; // Skip "is a type of"
                if let Some(superclass) = remaining.split_whitespace().next() {
                    self.add_class_with_parent(
                        subclass.to_string(),
                        subclass.to_string(),
                        superclass.to_string(),
                        0.7,
                    );
                }
            }
        }
    }

    fn extract_properties(&mut self, text: &str) {
        // Look for property patterns: "X has Y", "X contains Y", "X references Y"
        let sentences: Vec<&str> = text.split('.').collect();

        for sentence in sentences {
            // "has" pattern
            if sentence.contains(" has ") {
                self.extract_property_pattern(sentence, " has ", "has", 0.7);
            }

            // "contains" pattern
            if sentence.contains(" contains ") {
                self.extract_property_pattern(sentence, " contains ", "contains", 0.7);
            }

            // "references" pattern
            if sentence.contains(" references ") {
                self.extract_property_pattern(sentence, " references ", "references", 0.8);
            }
        }
    }

    fn extract_property_pattern(
        &mut self,
        sentence: &str,
        pattern: &str,
        prop_name: &str,
        confidence: f64,
    ) {
        if let Some(pos) = sentence.find(pattern) {
            let before = &sentence[..pos];
            let after = &sentence[pos + pattern.len()..];

            // Extract subject and object
            if let Some(subject) = before.split_whitespace().last() {
                if let Some(object) = after.split_whitespace().next() {
                    self.add_property(
                        prop_name.to_string(),
                        prop_name.to_string(),
                        Some(subject.to_string()),
                        Some(object.to_string()),
                        PropertyType::ObjectProperty,
                        confidence,
                    );
                }
            }
        }
    }

    fn extract_quoted(&self, text: &str) -> Option<String> {
        if let Some(start) = text.find('"') {
            if let Some(end) = text[start + 1..].find('"') {
                return Some(text[start + 1..start + 1 + end].to_string());
            }
        }
        None
    }

    fn add_class(
        &mut self,
        uri: String,
        label: String,
        description: Option<String>,
        confidence: f64,
    ) {
        let class = LearnedClass {
            uri: uri.clone(),
            label,
            description,
            parents: Vec::new(),
            examples: Vec::new(),
            confidence,
        };
        self.classes.insert(uri, class);
    }

    fn add_class_with_parent(
        &mut self,
        uri: String,
        label: String,
        parent: String,
        confidence: f64,
    ) {
        let class = LearnedClass {
            uri: uri.clone(),
            label,
            description: None,
            parents: vec![parent],
            examples: Vec::new(),
            confidence,
        };
        self.classes.insert(uri, class);
    }

    fn add_property(
        &mut self,
        uri: String,
        label: String,
        domain: Option<String>,
        range: Option<String>,
        property_type: PropertyType,
        confidence: f64,
    ) {
        let property = LearnedProperty {
            uri: uri.clone(),
            label,
            description: None,
            domain,
            range,
            property_type,
            confidence,
        };
        self.properties.insert(uri, property);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_learned_class_creation() {
        let class = LearnedClass {
            uri: "LegalDocument".to_string(),
            label: "Legal Document".to_string(),
            description: Some("A formal legal document".to_string()),
            parents: vec!["Document".to_string()],
            examples: Vec::new(),
            confidence: 0.9,
        };

        assert_eq!(class.uri, "LegalDocument");
        assert_eq!(class.confidence, 0.9);
    }

    #[test]
    fn test_learned_property_creation() {
        let property = LearnedProperty {
            uri: "hasAuthor".to_string(),
            label: "has author".to_string(),
            description: None,
            domain: Some("Document".to_string()),
            range: Some("Person".to_string()),
            property_type: PropertyType::ObjectProperty,
            confidence: 0.8,
        };

        assert_eq!(property.uri, "hasAuthor");
        assert_eq!(property.property_type, PropertyType::ObjectProperty);
    }

    #[test]
    fn test_learned_ontology_creation() {
        let ontology = LearnedOntology::new("http://example.org/", "Test Ontology");

        assert_eq!(ontology.base_uri, "http://example.org/");
        assert_eq!(ontology.title, "Test Ontology");
        assert!(ontology.classes.is_empty());
        assert!(ontology.properties.is_empty());
    }

    #[test]
    fn test_add_class_to_ontology() {
        let mut ontology = LearnedOntology::new("http://example.org/", "Test Ontology");

        let class = LearnedClass {
            uri: "TestClass".to_string(),
            label: "Test Class".to_string(),
            description: None,
            parents: Vec::new(),
            examples: Vec::new(),
            confidence: 0.9,
        };

        ontology.add_class(class);
        assert_eq!(ontology.classes.len(), 1);
    }

    #[test]
    fn test_add_property_to_ontology() {
        let mut ontology = LearnedOntology::new("http://example.org/", "Test Ontology");

        let property = LearnedProperty {
            uri: "testProperty".to_string(),
            label: "test property".to_string(),
            description: None,
            domain: None,
            range: None,
            property_type: PropertyType::ObjectProperty,
            confidence: 0.8,
        };

        ontology.add_property(property);
        assert_eq!(ontology.properties.len(), 1);
    }

    #[test]
    fn test_ontology_to_triples() {
        let mut ontology = LearnedOntology::new("http://example.org/", "Test Ontology");

        ontology.add_class(LearnedClass {
            uri: "TestClass".to_string(),
            label: "Test Class".to_string(),
            description: Some("A test class".to_string()),
            parents: Vec::new(),
            examples: Vec::new(),
            confidence: 0.9,
        });

        let triples = ontology.to_triples();
        assert!(!triples.is_empty());
        assert!(triples.iter().any(|t| t.predicate == "rdf:type"));
        assert!(triples.iter().any(|t| t.predicate == "rdfs:label"));
    }

    #[test]
    fn test_pattern_based_learner() {
        let learner = PatternBasedLearner::new("http://example.org/");
        assert_eq!(learner.base_uri, "http://example.org/");
        assert_eq!(learner.confidence_threshold, 0.5);
    }

    #[test]
    fn test_extract_class_from_definition() {
        let mut learner = PatternBasedLearner::new("http://example.org/");
        let text = r#"The term "plaintiff" means a person who brings a case to court."#;

        learner.learn(text);
        assert!(learner.classes.contains_key("plaintiff"));
    }

    #[test]
    fn test_extract_property() {
        let mut learner = PatternBasedLearner::new("http://example.org/");
        let text = "The document references the statute.";

        learner.learn(text);
        assert!(learner.properties.contains_key("references"));
    }

    #[test]
    fn test_confidence_threshold() {
        let learner = PatternBasedLearner::new("http://example.org/").with_threshold(0.8);

        assert_eq!(learner.confidence_threshold, 0.8);
    }

    #[test]
    fn test_ontology_stats() {
        let mut ontology = LearnedOntology::new("http://example.org/", "Test");

        ontology.add_class(LearnedClass {
            uri: "C1".to_string(),
            label: "Class 1".to_string(),
            description: None,
            parents: Vec::new(),
            examples: Vec::new(),
            confidence: 0.8,
        });

        ontology.add_property(LearnedProperty {
            uri: "p1".to_string(),
            label: "property 1".to_string(),
            description: None,
            domain: None,
            range: None,
            property_type: PropertyType::ObjectProperty,
            confidence: 0.9,
        });

        let stats = ontology.stats();
        assert_eq!(stats.total_classes, 1);
        assert_eq!(stats.total_properties, 1);
        assert_eq!(stats.avg_class_confidence, 0.8);
        assert_eq!(stats.avg_property_confidence, 0.9);
    }

    #[test]
    fn test_class_hierarchy() {
        let mut ontology = LearnedOntology::new("http://example.org/", "Test");

        ontology.add_class(LearnedClass {
            uri: "Subclass".to_string(),
            label: "Subclass".to_string(),
            description: None,
            parents: vec!["Superclass".to_string()],
            examples: Vec::new(),
            confidence: 0.9,
        });

        assert_eq!(ontology.class_hierarchy.len(), 1);
        assert!(ontology.class_hierarchy.contains_key("Subclass"));
    }

    #[test]
    fn test_property_types() {
        assert_eq!(PropertyType::ObjectProperty, PropertyType::ObjectProperty);
        assert_ne!(PropertyType::ObjectProperty, PropertyType::DatatypeProperty);
    }

    #[test]
    fn test_extract_quoted() {
        let learner = PatternBasedLearner::new("http://example.org/");
        let text = r#"The "plaintiff" is defined"#;

        let quoted = learner.extract_quoted(text);
        assert_eq!(quoted, Some("plaintiff".to_string()));
    }

    #[test]
    fn test_multiple_classes() {
        let mut learner = PatternBasedLearner::new("http://example.org/");
        let text = r#"
            The term "plaintiff" means a person who brings a case.
            The term "defendant" means a person who defends a case.
        "#;

        learner.learn(text);
        assert!(learner.classes.len() >= 2);
    }

    #[test]
    fn test_multiple_properties() {
        let mut learner = PatternBasedLearner::new("http://example.org/");
        let text = "The statute references the law. The document contains provisions.";

        learner.learn(text);
        assert!(learner.properties.len() >= 2);
    }

    #[test]
    fn test_empty_text() {
        let mut learner = PatternBasedLearner::new("http://example.org/");
        let ontology = learner.learn("");

        assert_eq!(ontology.classes.len(), 0);
        assert_eq!(ontology.properties.len(), 0);
    }

    #[test]
    fn test_triples_contain_owl_types() {
        let mut ontology = LearnedOntology::new("http://example.org/", "Test");

        ontology.add_class(LearnedClass {
            uri: "C1".to_string(),
            label: "Class 1".to_string(),
            description: None,
            parents: Vec::new(),
            examples: Vec::new(),
            confidence: 0.9,
        });

        let triples = ontology.to_triples();
        assert!(
            triples
                .iter()
                .any(|t| matches!(&t.object, RdfValue::Uri(u) if u == "owl:Class"))
        );
        assert!(
            triples
                .iter()
                .any(|t| matches!(&t.object, RdfValue::Uri(u) if u == "owl:Ontology"))
        );
    }
}
