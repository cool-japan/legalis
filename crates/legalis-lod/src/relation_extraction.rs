//! Automatic relation extraction for legal knowledge graphs.
//!
//! This module provides functionality for automatically extracting semantic relations
//! from legal text and converting them into RDF triples.

use crate::{RdfValue, Triple};
use std::collections::HashMap;

/// Represents a candidate relation extracted from text.
#[derive(Debug, Clone, PartialEq)]
pub struct ExtractedRelation {
    /// Subject entity (e.g., "Article 5")
    pub subject: String,
    /// Relation type (e.g., "references", "amends", "repeals")
    pub relation: RelationType,
    /// Object entity (e.g., "Section 2 of Act X")
    pub object: String,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
    /// Source text span that supports this relation
    pub evidence: Option<String>,
}

/// Types of legal relations that can be extracted.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RelationType {
    /// References another legal document
    References,
    /// Amends another provision
    Amends,
    /// Repeals another provision
    Repeals,
    /// Defines a term or concept
    Defines,
    /// Depends on another provision
    DependsOn,
    /// Conflicts with another provision
    ConflictsWith,
    /// Implements another provision
    Implements,
    /// Derives from another provision
    DerivedFrom,
    /// Custom relation type
    Custom(String),
}

impl RelationType {
    /// Converts the relation type to a URI predicate.
    pub fn to_predicate(&self) -> String {
        match self {
            RelationType::References => "legalis:references".to_string(),
            RelationType::Amends => "legalis:amends".to_string(),
            RelationType::Repeals => "legalis:repeals".to_string(),
            RelationType::Defines => "legalis:defines".to_string(),
            RelationType::DependsOn => "legalis:dependsOn".to_string(),
            RelationType::ConflictsWith => "legalis:conflictsWith".to_string(),
            RelationType::Implements => "legalis:implements".to_string(),
            RelationType::DerivedFrom => "prov:wasDerivedFrom".to_string(),
            RelationType::Custom(s) => format!("legalis:{}", s.replace(' ', "")),
        }
    }

    /// Parses a relation type from a string pattern.
    pub fn from_pattern(pattern: &str) -> Option<Self> {
        let pattern_lower = pattern.to_lowercase();
        if pattern_lower.contains("reference") || pattern_lower.contains("refer to") {
            Some(RelationType::References)
        } else if pattern_lower.contains("amend") {
            Some(RelationType::Amends)
        } else if pattern_lower.contains("repeal") {
            Some(RelationType::Repeals)
        } else if pattern_lower.contains("define") {
            Some(RelationType::Defines)
        } else if pattern_lower.contains("depend") || pattern_lower.contains("subject to") {
            Some(RelationType::DependsOn)
        } else if pattern_lower.contains("conflict") {
            Some(RelationType::ConflictsWith)
        } else if pattern_lower.contains("implement") {
            Some(RelationType::Implements)
        } else if pattern_lower.contains("derive") {
            Some(RelationType::DerivedFrom)
        } else {
            None
        }
    }
}

/// Pattern-based relation extractor.
///
/// This extractor uses regular expressions and linguistic patterns
/// to identify relations in legal text.
pub struct PatternBasedExtractor {
    /// Minimum confidence threshold for accepting relations
    confidence_threshold: f64,
    /// Base URI for generated entities
    base_uri: String,
    /// Custom extraction patterns
    patterns: Vec<ExtractionPattern>,
}

/// A pattern for extracting relations from text.
#[derive(Debug, Clone)]
pub struct ExtractionPattern {
    /// Pattern identifier
    pub id: String,
    /// Regular expression pattern
    pub pattern: String,
    /// Relation type this pattern extracts
    pub relation_type: RelationType,
    /// Confidence score for matches (0.0 to 1.0)
    pub confidence: f64,
}

impl Default for PatternBasedExtractor {
    fn default() -> Self {
        Self::new()
    }
}

impl PatternBasedExtractor {
    /// Creates a new pattern-based extractor with default patterns.
    pub fn new() -> Self {
        let mut extractor = Self {
            confidence_threshold: 0.5,
            base_uri: "https://example.org/legalis/".to_string(),
            patterns: Vec::new(),
        };
        extractor.add_default_patterns();
        extractor
    }

    /// Sets the confidence threshold.
    pub fn with_threshold(mut self, threshold: f64) -> Self {
        self.confidence_threshold = threshold;
        self
    }

    /// Sets the base URI for entities.
    pub fn with_base_uri(mut self, base_uri: impl Into<String>) -> Self {
        self.base_uri = base_uri.into();
        self
    }

    /// Adds a custom extraction pattern.
    pub fn add_pattern(&mut self, pattern: ExtractionPattern) {
        self.patterns.push(pattern);
    }

    /// Adds default legal patterns.
    fn add_default_patterns(&mut self) {
        // Reference patterns
        self.patterns.push(ExtractionPattern {
            id: "ref1".to_string(),
            pattern:
                "(?i)(article|section|paragraph)\\s+(\\d+)\\s+(?:of\\s+)?(?:the\\s+)?(\\w+\\s+\\w+)"
                    .to_string(),
            relation_type: RelationType::References,
            confidence: 0.8,
        });

        // Amendment patterns
        self.patterns.push(ExtractionPattern {
            id: "amend1".to_string(),
            pattern: "(?i)amend(?:s|ed)?\\s+(article|section)\\s+(\\d+)".to_string(),
            relation_type: RelationType::Amends,
            confidence: 0.9,
        });

        // Repeal patterns
        self.patterns.push(ExtractionPattern {
            id: "repeal1".to_string(),
            pattern: "(?i)repeal(?:s|ed)?\\s+(article|section)\\s+(\\d+)".to_string(),
            relation_type: RelationType::Repeals,
            confidence: 0.95,
        });

        // Definition patterns
        self.patterns.push(ExtractionPattern {
            id: "define1".to_string(),
            pattern: "(?i)([\"']\\w+[\"'])\\s+means".to_string(),
            relation_type: RelationType::Defines,
            confidence: 0.85,
        });

        // Dependency patterns
        self.patterns.push(ExtractionPattern {
            id: "depend1".to_string(),
            pattern: "(?i)subject\\s+to\\s+(article|section)\\s+(\\d+)".to_string(),
            relation_type: RelationType::DependsOn,
            confidence: 0.8,
        });
    }

    /// Extracts relations from legal text.
    pub fn extract(&self, text: &str) -> Vec<ExtractedRelation> {
        let mut relations = Vec::new();

        // Simple pattern matching (in a real implementation, would use regex)
        // For demonstration, we'll use simple string matching

        // Extract references
        if let Some(rel) = self.extract_references(text) {
            relations.extend(rel);
        }

        // Extract amendments
        if let Some(rel) = self.extract_amendments(text) {
            relations.extend(rel);
        }

        // Extract repeals
        if let Some(rel) = self.extract_repeals(text) {
            relations.extend(rel);
        }

        // Extract definitions
        if let Some(rel) = self.extract_definitions(text) {
            relations.extend(rel);
        }

        // Filter by confidence threshold
        relations.retain(|r| r.confidence >= self.confidence_threshold);

        relations
    }

    fn extract_references(&self, text: &str) -> Option<Vec<ExtractedRelation>> {
        let mut relations = Vec::new();
        let text_lower = text.to_lowercase();

        // Simple pattern: "refers to Article X"
        if text_lower.contains("refer") {
            // Extract article numbers (simplified)
            for word in text.split_whitespace() {
                if word.to_lowercase().starts_with("article")
                    && let Some(evidence) = self.extract_context(text, word)
                {
                    relations.push(ExtractedRelation {
                        subject: "current-provision".to_string(),
                        relation: RelationType::References,
                        object: word.to_string(),
                        confidence: 0.7,
                        evidence: Some(evidence),
                    });
                }
            }
        }

        if relations.is_empty() {
            None
        } else {
            Some(relations)
        }
    }

    fn extract_amendments(&self, text: &str) -> Option<Vec<ExtractedRelation>> {
        let mut relations = Vec::new();
        let text_lower = text.to_lowercase();

        if text_lower.contains("amend") {
            // Look for patterns like "amends section X"
            let words: Vec<&str> = text.split_whitespace().collect();
            for (i, word) in words.iter().enumerate() {
                if word.to_lowercase().contains("amend") {
                    // Look ahead for section/article references
                    if i + 2 < words.len() {
                        let next = words[i + 1];
                        if next.to_lowercase().contains("section")
                            || next.to_lowercase().contains("article")
                        {
                            let target = format!("{} {}", next, words[i + 2]);
                            if let Some(evidence) = self.extract_context(text, word) {
                                relations.push(ExtractedRelation {
                                    subject: "current-provision".to_string(),
                                    relation: RelationType::Amends,
                                    object: target,
                                    confidence: 0.85,
                                    evidence: Some(evidence),
                                });
                            }
                        }
                    }
                }
            }
        }

        if relations.is_empty() {
            None
        } else {
            Some(relations)
        }
    }

    fn extract_repeals(&self, text: &str) -> Option<Vec<ExtractedRelation>> {
        let mut relations = Vec::new();
        let text_lower = text.to_lowercase();

        if text_lower.contains("repeal") {
            // Look for patterns like "repeals section X"
            let words: Vec<&str> = text.split_whitespace().collect();
            for (i, word) in words.iter().enumerate() {
                if word.to_lowercase().contains("repeal") {
                    // Look ahead for section/article references
                    if i + 2 < words.len() {
                        let next = words[i + 1];
                        if next.to_lowercase().contains("section")
                            || next.to_lowercase().contains("article")
                        {
                            let target = format!("{} {}", next, words[i + 2]);
                            if let Some(evidence) = self.extract_context(text, word) {
                                relations.push(ExtractedRelation {
                                    subject: "current-provision".to_string(),
                                    relation: RelationType::Repeals,
                                    object: target,
                                    confidence: 0.9,
                                    evidence: Some(evidence),
                                });
                            }
                        }
                    }
                }
            }
        }

        if relations.is_empty() {
            None
        } else {
            Some(relations)
        }
    }

    fn extract_definitions(&self, text: &str) -> Option<Vec<ExtractedRelation>> {
        let mut relations = Vec::new();
        let text_lower = text.to_lowercase();

        if text_lower.contains("means") || text_lower.contains("defined as") {
            // Look for quoted terms
            if text.contains('"') {
                let parts: Vec<&str> = text.split('"').collect();
                for i in (1..parts.len()).step_by(2) {
                    let term = parts[i];
                    if !term.is_empty()
                        && let Some(evidence) = self.extract_context(text, term)
                    {
                        relations.push(ExtractedRelation {
                            subject: "current-provision".to_string(),
                            relation: RelationType::Defines,
                            object: term.to_string(),
                            confidence: 0.8,
                            evidence: Some(evidence),
                        });
                    }
                }
            }
        }

        if relations.is_empty() {
            None
        } else {
            Some(relations)
        }
    }

    fn extract_context(&self, text: &str, keyword: &str) -> Option<String> {
        // Extract a sentence containing the keyword
        let sentences: Vec<&str> = text.split('.').collect();
        for sentence in sentences {
            if sentence.contains(keyword) {
                return Some(sentence.trim().to_string());
            }
        }
        None
    }

    /// Converts extracted relations to RDF triples.
    pub fn to_triples(&self, relations: &[ExtractedRelation]) -> Vec<Triple> {
        let mut triples = Vec::new();

        for relation in relations {
            let subject = format!("{}{}", self.base_uri, self.normalize_uri(&relation.subject));
            let object = format!("{}{}", self.base_uri, self.normalize_uri(&relation.object));

            // Main relation triple
            triples.push(Triple {
                subject: subject.clone(),
                predicate: relation.relation.to_predicate(),
                object: RdfValue::Uri(object.clone()),
            });

            // Add confidence as a property
            triples.push(Triple {
                subject: subject.clone(),
                predicate: "legalis:confidence".to_string(),
                object: RdfValue::TypedLiteral(
                    relation.confidence.to_string(),
                    "xsd:double".to_string(),
                ),
            });

            // Add evidence if available
            if let Some(ref evidence) = relation.evidence {
                triples.push(Triple {
                    subject: subject.clone(),
                    predicate: "legalis:evidence".to_string(),
                    object: RdfValue::string(evidence),
                });
            }
        }

        triples
    }

    fn normalize_uri(&self, text: &str) -> String {
        text.replace(' ', "_")
            .replace(['"', '\''], "")
            .to_lowercase()
    }
}

/// Relation graph for analyzing extracted relations.
pub struct RelationGraph {
    /// Relations indexed by subject
    by_subject: HashMap<String, Vec<ExtractedRelation>>,
    /// Relations indexed by object
    by_object: HashMap<String, Vec<ExtractedRelation>>,
    /// All relations
    relations: Vec<ExtractedRelation>,
}

impl RelationGraph {
    /// Creates a new relation graph from extracted relations.
    pub fn new(relations: Vec<ExtractedRelation>) -> Self {
        let mut by_subject: HashMap<String, Vec<ExtractedRelation>> = HashMap::new();
        let mut by_object: HashMap<String, Vec<ExtractedRelation>> = HashMap::new();

        for relation in &relations {
            by_subject
                .entry(relation.subject.clone())
                .or_default()
                .push(relation.clone());
            by_object
                .entry(relation.object.clone())
                .or_default()
                .push(relation.clone());
        }

        Self {
            by_subject,
            by_object,
            relations,
        }
    }

    /// Gets all relations where the given entity is the subject.
    pub fn get_outgoing(&self, entity: &str) -> Vec<&ExtractedRelation> {
        self.by_subject
            .get(entity)
            .map(|rels| rels.iter().collect())
            .unwrap_or_default()
    }

    /// Gets all relations where the given entity is the object.
    pub fn get_incoming(&self, entity: &str) -> Vec<&ExtractedRelation> {
        self.by_object
            .get(entity)
            .map(|rels| rels.iter().collect())
            .unwrap_or_default()
    }

    /// Gets all relations of a specific type.
    pub fn get_by_type(&self, relation_type: &RelationType) -> Vec<&ExtractedRelation> {
        self.relations
            .iter()
            .filter(|r| &r.relation == relation_type)
            .collect()
    }

    /// Finds transitive relations (e.g., A references B, B references C).
    pub fn find_transitive(&self, relation_type: &RelationType) -> Vec<(String, String)> {
        let mut transitive = Vec::new();

        for relation in &self.relations {
            if &relation.relation == relation_type {
                // Find relations from the object
                if let Some(next_rels) = self.by_subject.get(&relation.object) {
                    for next_rel in next_rels {
                        if &next_rel.relation == relation_type {
                            transitive.push((relation.subject.clone(), next_rel.object.clone()));
                        }
                    }
                }
            }
        }

        transitive
    }

    /// Gets statistics about the relation graph.
    pub fn stats(&self) -> RelationStats {
        let mut by_type: HashMap<RelationType, usize> = HashMap::new();
        for relation in &self.relations {
            *by_type.entry(relation.relation.clone()).or_insert(0) += 1;
        }

        RelationStats {
            total_relations: self.relations.len(),
            unique_subjects: self.by_subject.len(),
            unique_objects: self.by_object.len(),
            relations_by_type: by_type,
            avg_confidence: self.relations.iter().map(|r| r.confidence).sum::<f64>()
                / self.relations.len() as f64,
        }
    }
}

/// Statistics about extracted relations.
#[derive(Debug, Clone)]
pub struct RelationStats {
    /// Total number of relations
    pub total_relations: usize,
    /// Number of unique subjects
    pub unique_subjects: usize,
    /// Number of unique objects
    pub unique_objects: usize,
    /// Count of relations by type
    pub relations_by_type: HashMap<RelationType, usize>,
    /// Average confidence score
    pub avg_confidence: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relation_type_to_predicate() {
        assert_eq!(
            RelationType::References.to_predicate(),
            "legalis:references"
        );
        assert_eq!(RelationType::Amends.to_predicate(), "legalis:amends");
        assert_eq!(RelationType::Repeals.to_predicate(), "legalis:repeals");
    }

    #[test]
    fn test_relation_type_from_pattern() {
        assert_eq!(
            RelationType::from_pattern("this reference to"),
            Some(RelationType::References)
        );
        assert_eq!(
            RelationType::from_pattern("amends the provision"),
            Some(RelationType::Amends)
        );
        assert_eq!(
            RelationType::from_pattern("repeals section"),
            Some(RelationType::Repeals)
        );
    }

    #[test]
    fn test_pattern_extractor_creation() {
        let extractor = PatternBasedExtractor::new();
        assert!(!extractor.patterns.is_empty());
        assert_eq!(extractor.confidence_threshold, 0.5);
    }

    #[test]
    fn test_extract_references() {
        let extractor = PatternBasedExtractor::new();
        let text = "This provision refers to Article 5 of the Constitution.";
        let relations = extractor.extract(text);

        assert!(!relations.is_empty());
        assert!(
            relations
                .iter()
                .any(|r| r.relation == RelationType::References)
        );
    }

    #[test]
    fn test_extract_amendments() {
        let extractor = PatternBasedExtractor::new();
        let text = "This Act amends Section 10 of the previous law.";
        let relations = extractor.extract(text);

        assert!(!relations.is_empty());
        assert!(relations.iter().any(|r| r.relation == RelationType::Amends));
    }

    #[test]
    fn test_extract_repeals() {
        let extractor = PatternBasedExtractor::new();
        let text = "Section 3 repeals Article 7 of the old statute.";
        let relations = extractor.extract(text);

        assert!(!relations.is_empty());
        assert!(
            relations
                .iter()
                .any(|r| r.relation == RelationType::Repeals)
        );
    }

    #[test]
    fn test_extract_definitions() {
        let extractor = PatternBasedExtractor::new();
        let text = r#"The term "plaintiff" means a person who brings a case."#;
        let relations = extractor.extract(text);

        assert!(!relations.is_empty());
        assert!(
            relations
                .iter()
                .any(|r| r.relation == RelationType::Defines)
        );
    }

    #[test]
    fn test_confidence_threshold() {
        let extractor = PatternBasedExtractor::new().with_threshold(0.9);
        let text = "This refers to Article 5.";
        let relations = extractor.extract(text);

        // Low confidence relations should be filtered
        assert!(relations.iter().all(|r| r.confidence >= 0.9));
    }

    #[test]
    fn test_to_triples() {
        let extractor = PatternBasedExtractor::new();
        let relations = vec![ExtractedRelation {
            subject: "Article 1".to_string(),
            relation: RelationType::References,
            object: "Article 2".to_string(),
            confidence: 0.9,
            evidence: Some("Article 1 refers to Article 2".to_string()),
        }];

        let triples = extractor.to_triples(&relations);
        assert!(!triples.is_empty());
        assert!(triples.iter().any(|t| t.predicate == "legalis:references"));
        assert!(triples.iter().any(|t| t.predicate == "legalis:confidence"));
        assert!(triples.iter().any(|t| t.predicate == "legalis:evidence"));
    }

    #[test]
    fn test_relation_graph() {
        let relations = vec![
            ExtractedRelation {
                subject: "A".to_string(),
                relation: RelationType::References,
                object: "B".to_string(),
                confidence: 0.9,
                evidence: None,
            },
            ExtractedRelation {
                subject: "B".to_string(),
                relation: RelationType::References,
                object: "C".to_string(),
                confidence: 0.8,
                evidence: None,
            },
        ];

        let graph = RelationGraph::new(relations);
        assert_eq!(graph.get_outgoing("A").len(), 1);
        assert_eq!(graph.get_incoming("B").len(), 1);
    }

    #[test]
    fn test_transitive_relations() {
        let relations = vec![
            ExtractedRelation {
                subject: "A".to_string(),
                relation: RelationType::References,
                object: "B".to_string(),
                confidence: 0.9,
                evidence: None,
            },
            ExtractedRelation {
                subject: "B".to_string(),
                relation: RelationType::References,
                object: "C".to_string(),
                confidence: 0.8,
                evidence: None,
            },
        ];

        let graph = RelationGraph::new(relations);
        let transitive = graph.find_transitive(&RelationType::References);

        assert_eq!(transitive.len(), 1);
        assert_eq!(transitive[0], ("A".to_string(), "C".to_string()));
    }

    #[test]
    fn test_relation_stats() {
        let relations = vec![
            ExtractedRelation {
                subject: "A".to_string(),
                relation: RelationType::References,
                object: "B".to_string(),
                confidence: 0.9,
                evidence: None,
            },
            ExtractedRelation {
                subject: "C".to_string(),
                relation: RelationType::Amends,
                object: "D".to_string(),
                confidence: 0.8,
                evidence: None,
            },
        ];

        let graph = RelationGraph::new(relations);
        let stats = graph.stats();

        assert_eq!(stats.total_relations, 2);
        assert_eq!(stats.unique_subjects, 2);
        assert_eq!(stats.unique_objects, 2);
        assert!((stats.avg_confidence - 0.85).abs() < 0.001);
    }

    #[test]
    fn test_get_by_type() {
        let relations = vec![
            ExtractedRelation {
                subject: "A".to_string(),
                relation: RelationType::References,
                object: "B".to_string(),
                confidence: 0.9,
                evidence: None,
            },
            ExtractedRelation {
                subject: "C".to_string(),
                relation: RelationType::Amends,
                object: "D".to_string(),
                confidence: 0.8,
                evidence: None,
            },
        ];

        let graph = RelationGraph::new(relations);
        let refs = graph.get_by_type(&RelationType::References);

        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0].subject, "A");
    }

    #[test]
    fn test_custom_relation_type() {
        let custom = RelationType::Custom("supersedes".to_string());
        assert_eq!(custom.to_predicate(), "legalis:supersedes");
    }

    #[test]
    fn test_normalize_uri() {
        let extractor = PatternBasedExtractor::new();
        assert_eq!(extractor.normalize_uri("Article 5"), "article_5");
        assert_eq!(extractor.normalize_uri("\"term\""), "term");
    }
}
