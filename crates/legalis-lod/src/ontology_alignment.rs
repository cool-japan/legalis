//! Ontology alignment tools for mapping between different ontologies.
//!
//! This module provides tools for aligning and mapping entities across ontologies,
//! including:
//! - Entity matching based on labels, URIs, and properties
//! - Similarity computation
//! - Alignment generation and validation
//! - Mapping export in standard formats

use crate::{RdfValue, Triple};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Type of alignment relationship between entities.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlignmentRelation {
    /// Entities are equivalent (owl:equivalentClass, owl:equivalentProperty)
    Equivalent,
    /// Source entity is a subclass/subproperty of target
    SubsumedBy,
    /// Target entity is a subclass/subproperty of source
    Subsumes,
    /// Entities are related but not hierarchically
    Related,
    /// Entities are disjoint (mutually exclusive)
    Disjoint,
}

impl AlignmentRelation {
    /// Returns the OWL/RDFS property for this relation.
    pub fn to_property(&self) -> &'static str {
        match self {
            AlignmentRelation::Equivalent => "owl:equivalentClass",
            AlignmentRelation::SubsumedBy => "rdfs:subClassOf",
            AlignmentRelation::Subsumes => "^rdfs:subClassOf",
            AlignmentRelation::Related => "skos:related",
            AlignmentRelation::Disjoint => "owl:disjointWith",
        }
    }
}

/// Represents an alignment between two entities from different ontologies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityAlignment {
    /// URI of the source entity
    pub source_uri: String,
    /// URI of the target entity
    pub target_uri: String,
    /// Type of alignment relationship
    pub relation: AlignmentRelation,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
    /// Explanation/justification for the alignment
    pub justification: Option<String>,
}

impl EntityAlignment {
    /// Creates a new entity alignment.
    pub fn new(
        source_uri: impl Into<String>,
        target_uri: impl Into<String>,
        relation: AlignmentRelation,
        confidence: f64,
    ) -> Self {
        Self {
            source_uri: source_uri.into(),
            target_uri: target_uri.into(),
            relation,
            confidence: confidence.clamp(0.0, 1.0),
            justification: None,
        }
    }

    /// Sets the justification.
    pub fn with_justification(mut self, justification: impl Into<String>) -> Self {
        self.justification = Some(justification.into());
        self
    }

    /// Converts the alignment to RDF triples.
    pub fn to_triples(&self) -> Vec<Triple> {
        let mut triples = Vec::new();

        triples.push(Triple {
            subject: self.source_uri.clone(),
            predicate: self.relation.to_property().to_string(),
            object: RdfValue::Uri(self.target_uri.clone()),
        });

        // Add confidence as annotation if less than 1.0
        if self.confidence < 1.0 {
            // Use RDF reification to add confidence
            let statement_uri = format!("{}#alignment", self.source_uri);
            triples.push(Triple {
                subject: statement_uri.clone(),
                predicate: "rdf:type".to_string(),
                object: RdfValue::Uri("rdf:Statement".to_string()),
            });
            triples.push(Triple {
                subject: statement_uri.clone(),
                predicate: "rdf:subject".to_string(),
                object: RdfValue::Uri(self.source_uri.clone()),
            });
            triples.push(Triple {
                subject: statement_uri.clone(),
                predicate: "rdf:predicate".to_string(),
                object: RdfValue::Uri(self.relation.to_property().to_string()),
            });
            triples.push(Triple {
                subject: statement_uri.clone(),
                predicate: "rdf:object".to_string(),
                object: RdfValue::Uri(self.target_uri.clone()),
            });
            triples.push(Triple {
                subject: statement_uri,
                predicate: "legalis:confidence".to_string(),
                object: RdfValue::TypedLiteral(
                    self.confidence.to_string(),
                    "xsd:double".to_string(),
                ),
            });
        }

        triples
    }
}

/// A complete alignment between two ontologies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OntologyAlignment {
    /// URI of the source ontology
    pub source_ontology: String,
    /// URI of the target ontology
    pub target_ontology: String,
    /// All entity alignments
    pub alignments: Vec<EntityAlignment>,
    /// Metadata about the alignment process
    pub metadata: HashMap<String, String>,
}

impl OntologyAlignment {
    /// Creates a new ontology alignment.
    pub fn new(source_ontology: impl Into<String>, target_ontology: impl Into<String>) -> Self {
        Self {
            source_ontology: source_ontology.into(),
            target_ontology: target_ontology.into(),
            alignments: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Adds an alignment.
    pub fn add_alignment(&mut self, alignment: EntityAlignment) {
        self.alignments.push(alignment);
    }

    /// Adds metadata.
    pub fn add_metadata(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.metadata.insert(key.into(), value.into());
    }

    /// Filters alignments by minimum confidence.
    pub fn filter_by_confidence(&self, min_confidence: f64) -> Vec<&EntityAlignment> {
        self.alignments
            .iter()
            .filter(|a| a.confidence >= min_confidence)
            .collect()
    }

    /// Gets alignments for a specific source entity.
    pub fn get_alignments_for(&self, source_uri: &str) -> Vec<&EntityAlignment> {
        self.alignments
            .iter()
            .filter(|a| a.source_uri == source_uri)
            .collect()
    }

    /// Converts all alignments to RDF triples.
    pub fn to_triples(&self) -> Vec<Triple> {
        let mut triples = Vec::new();

        for alignment in &self.alignments {
            triples.extend(alignment.to_triples());
        }

        // Add ontology-level metadata
        let alignment_uri = format!(
            "{}#alignment-to-{}",
            self.source_ontology,
            self.target_ontology.replace(['/', ':'], "-")
        );

        triples.push(Triple {
            subject: alignment_uri.clone(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("owl:Ontology".to_string()),
        });

        triples.push(Triple {
            subject: alignment_uri.clone(),
            predicate: "owl:imports".to_string(),
            object: RdfValue::Uri(self.source_ontology.clone()),
        });

        triples.push(Triple {
            subject: alignment_uri,
            predicate: "owl:imports".to_string(),
            object: RdfValue::Uri(self.target_ontology.clone()),
        });

        triples
    }
}

/// Computes similarity between entity labels.
fn label_similarity(label1: &str, label2: &str) -> f64 {
    let label1_lower = label1.to_lowercase();
    let label2_lower = label2.to_lowercase();

    // Exact match
    if label1_lower == label2_lower {
        return 1.0;
    }

    // Contains match
    if label1_lower.contains(&label2_lower) || label2_lower.contains(&label1_lower) {
        return 0.8;
    }

    // Simple token-based similarity
    let tokens1: Vec<&str> = label1_lower.split_whitespace().collect();
    let tokens2: Vec<&str> = label2_lower.split_whitespace().collect();

    let common_tokens = tokens1.iter().filter(|t| tokens2.contains(t)).count();

    if tokens1.is_empty() || tokens2.is_empty() {
        return 0.0;
    }

    let max_tokens = tokens1.len().max(tokens2.len());
    common_tokens as f64 / max_tokens as f64
}

/// Computes similarity between entity URIs.
fn uri_similarity(uri1: &str, uri2: &str) -> f64 {
    // Extract local names
    let local1 = uri1.split(&['/', '#'][..]).next_back().unwrap_or(uri1);
    let local2 = uri2.split(&['/', '#'][..]).next_back().unwrap_or(uri2);

    label_similarity(local1, local2)
}

/// Entity matcher for finding alignments between ontologies.
pub struct EntityMatcher {
    /// Minimum confidence threshold for alignments
    pub min_confidence: f64,
}

impl EntityMatcher {
    /// Creates a new entity matcher.
    pub fn new() -> Self {
        Self {
            min_confidence: 0.5,
        }
    }

    /// Sets the minimum confidence threshold.
    pub fn with_min_confidence(mut self, min_confidence: f64) -> Self {
        self.min_confidence = min_confidence.clamp(0.0, 1.0);
        self
    }

    /// Finds alignments between entities from two ontologies.
    #[allow(clippy::too_many_arguments)]
    pub fn find_alignments(
        &self,
        source_entities: &[(String, String)], // (URI, label)
        target_entities: &[(String, String)],
    ) -> Vec<EntityAlignment> {
        let mut alignments = Vec::new();

        for (source_uri, source_label) in source_entities {
            for (target_uri, target_label) in target_entities {
                let label_sim = label_similarity(source_label, target_label);
                let uri_sim = uri_similarity(source_uri, target_uri);

                // Combine similarities (weighted average)
                let confidence = 0.7 * label_sim + 0.3 * uri_sim;

                if confidence >= self.min_confidence {
                    let relation = if confidence >= 0.9 {
                        AlignmentRelation::Equivalent
                    } else {
                        AlignmentRelation::Related
                    };

                    let alignment = EntityAlignment::new(
                        source_uri.clone(),
                        target_uri.clone(),
                        relation,
                        confidence,
                    )
                    .with_justification(format!(
                        "Label similarity: {:.2}, URI similarity: {:.2}",
                        label_sim, uri_sim
                    ));

                    alignments.push(alignment);
                }
            }
        }

        // Sort by confidence descending
        alignments.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());

        alignments
    }
}

impl Default for EntityMatcher {
    fn default() -> Self {
        Self::new()
    }
}

/// Validates an ontology alignment for consistency.
pub struct AlignmentValidator;

impl AlignmentValidator {
    /// Creates a new validator.
    pub fn new() -> Self {
        Self
    }

    /// Validates an alignment.
    pub fn validate(&self, alignment: &OntologyAlignment) -> ValidationResult {
        let mut issues = Vec::new();

        // Check for conflicting alignments
        let mut source_to_targets: HashMap<&str, Vec<(&str, &AlignmentRelation)>> = HashMap::new();

        for a in &alignment.alignments {
            source_to_targets
                .entry(&a.source_uri)
                .or_default()
                .push((&a.target_uri, &a.relation));
        }

        for (source, targets) in &source_to_targets {
            // Check for multiple equivalence alignments
            let equiv_count = targets
                .iter()
                .filter(|(_, rel)| **rel == AlignmentRelation::Equivalent)
                .count();

            if equiv_count > 1 {
                issues.push(format!(
                    "Entity {} has multiple equivalence alignments",
                    source
                ));
            }

            // Check for conflicting relations
            let has_equiv = targets
                .iter()
                .any(|(_, rel)| **rel == AlignmentRelation::Equivalent);
            let has_disjoint = targets
                .iter()
                .any(|(_, rel)| **rel == AlignmentRelation::Disjoint);

            if has_equiv && has_disjoint {
                issues.push(format!(
                    "Entity {} has conflicting equivalence and disjoint alignments",
                    source
                ));
            }
        }

        ValidationResult {
            is_valid: issues.is_empty(),
            issues,
        }
    }
}

impl Default for AlignmentValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of alignment validation.
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Whether the alignment is valid
    pub is_valid: bool,
    /// List of validation issues
    pub issues: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_label_similarity() {
        assert_eq!(label_similarity("Person", "Person"), 1.0);
        assert_eq!(label_similarity("Person", "Human"), 0.0); // Different words, no overlap
        assert!(label_similarity("Legal Person", "Person") > 0.5);
    }

    #[test]
    fn test_uri_similarity() {
        assert!(uri_similarity("http://example.org/Person", "http://other.org/Person") > 0.9);
    }

    #[test]
    fn test_entity_alignment() {
        let alignment = EntityAlignment::new(
            "http://example.org/Person",
            "http://other.org/Human",
            AlignmentRelation::Equivalent,
            0.95,
        );

        assert_eq!(alignment.source_uri, "http://example.org/Person");
        assert_eq!(alignment.confidence, 0.95);
        assert_eq!(alignment.relation, AlignmentRelation::Equivalent);
    }

    #[test]
    fn test_alignment_to_triples() {
        let alignment = EntityAlignment::new(
            "http://example.org/Person",
            "http://other.org/Human",
            AlignmentRelation::Equivalent,
            1.0,
        );

        let triples = alignment.to_triples();
        assert!(!triples.is_empty());
        assert!(triples.iter().any(|t| t.predicate == "owl:equivalentClass"));
    }

    #[test]
    fn test_ontology_alignment() {
        let mut alignment =
            OntologyAlignment::new("http://example.org/onto1", "http://example.org/onto2");

        let entity_alignment = EntityAlignment::new(
            "http://example.org/onto1#Person",
            "http://example.org/onto2#Human",
            AlignmentRelation::Equivalent,
            0.95,
        );

        alignment.add_alignment(entity_alignment);
        assert_eq!(alignment.alignments.len(), 1);
    }

    #[test]
    fn test_filter_by_confidence() {
        let mut alignment =
            OntologyAlignment::new("http://example.org/onto1", "http://example.org/onto2");

        alignment.add_alignment(EntityAlignment::new(
            "http://example.org/A",
            "http://example.org/B",
            AlignmentRelation::Equivalent,
            0.9,
        ));

        alignment.add_alignment(EntityAlignment::new(
            "http://example.org/C",
            "http://example.org/D",
            AlignmentRelation::Related,
            0.5,
        ));

        let filtered = alignment.filter_by_confidence(0.8);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].confidence, 0.9);
    }

    #[test]
    fn test_entity_matcher() {
        let matcher = EntityMatcher::new().with_min_confidence(0.5);

        let source = vec![
            (
                "http://example.org/Person".to_string(),
                "Person".to_string(),
            ),
            (
                "http://example.org/Company".to_string(),
                "Company".to_string(),
            ),
        ];

        let target = vec![
            ("http://other.org/Human".to_string(), "Human".to_string()),
            ("http://other.org/Person".to_string(), "Person".to_string()),
        ];

        let alignments = matcher.find_alignments(&source, &target);
        assert!(!alignments.is_empty());

        // Person should match Person with high confidence
        let person_alignment = alignments
            .iter()
            .find(|a| a.source_uri.contains("Person") && a.target_uri.contains("Person"));
        assert!(person_alignment.is_some());
        assert!(person_alignment.unwrap().confidence > 0.9);
    }

    #[test]
    fn test_alignment_validator() {
        let validator = AlignmentValidator::new();

        let mut alignment =
            OntologyAlignment::new("http://example.org/onto1", "http://example.org/onto2");

        // Valid alignment
        alignment.add_alignment(EntityAlignment::new(
            "http://example.org/A",
            "http://example.org/B",
            AlignmentRelation::Equivalent,
            0.95,
        ));

        let result = validator.validate(&alignment);
        assert!(result.is_valid);

        // Add conflicting alignment
        alignment.add_alignment(EntityAlignment::new(
            "http://example.org/A",
            "http://example.org/C",
            AlignmentRelation::Disjoint,
            0.8,
        ));

        let result2 = validator.validate(&alignment);
        assert!(!result2.is_valid);
        assert!(!result2.issues.is_empty());
    }

    #[test]
    fn test_alignment_relation_property() {
        assert_eq!(
            AlignmentRelation::Equivalent.to_property(),
            "owl:equivalentClass"
        );
        assert_eq!(
            AlignmentRelation::SubsumedBy.to_property(),
            "rdfs:subClassOf"
        );
        assert_eq!(AlignmentRelation::Related.to_property(), "skos:related");
    }
}
