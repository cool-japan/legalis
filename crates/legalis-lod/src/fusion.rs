//! Legal knowledge fusion for integrating data from multiple sources.
//!
//! This module provides tools for:
//! - Cross-ontology mapping
//! - Entity resolution across sources
//! - Knowledge graph merging
//! - Conflict detection and resolution
//! - Provenance tracking for fused data

use crate::{RdfValue, Triple};
use std::collections::{HashMap, HashSet};

/// Cross-ontology mapping between different vocabularies.
#[derive(Debug, Clone)]
pub struct OntologyMapping {
    /// Source ontology namespace
    pub source_namespace: String,
    /// Target ontology namespace
    pub target_namespace: String,
    /// Property mappings (source property -> target property)
    pub property_mappings: HashMap<String, String>,
    /// Class mappings (source class -> target class)
    pub class_mappings: HashMap<String, String>,
    /// Mapping confidence (0.0 to 1.0)
    pub confidence: f64,
}

impl OntologyMapping {
    /// Creates a new ontology mapping.
    pub fn new(source: impl Into<String>, target: impl Into<String>) -> Self {
        Self {
            source_namespace: source.into(),
            target_namespace: target.into(),
            property_mappings: HashMap::new(),
            class_mappings: HashMap::new(),
            confidence: 1.0,
        }
    }

    /// Adds a property mapping.
    pub fn add_property_mapping(
        &mut self,
        source_property: impl Into<String>,
        target_property: impl Into<String>,
    ) {
        self.property_mappings
            .insert(source_property.into(), target_property.into());
    }

    /// Adds a class mapping.
    pub fn add_class_mapping(
        &mut self,
        source_class: impl Into<String>,
        target_class: impl Into<String>,
    ) {
        self.class_mappings
            .insert(source_class.into(), target_class.into());
    }

    /// Sets the mapping confidence.
    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }

    /// Transforms a triple from source ontology to target ontology.
    pub fn transform_triple(&self, triple: &Triple) -> Option<Triple> {
        // Map predicate
        let new_predicate = self.property_mappings.get(&triple.predicate)?;

        Some(Triple {
            subject: triple.subject.clone(),
            predicate: new_predicate.clone(),
            object: triple.object.clone(),
        })
    }

    /// Transforms multiple triples.
    pub fn transform_triples(&self, triples: &[Triple]) -> Vec<Triple> {
        triples
            .iter()
            .filter_map(|t| self.transform_triple(t))
            .collect()
    }
}

/// Entity resolution matcher for identifying same entities across sources.
#[derive(Debug)]
pub struct EntityResolver {
    /// Similarity threshold for matching
    threshold: f64,
    /// Known equivalence links (entity1 -> entity2)
    equivalences: HashMap<String, String>,
}

impl EntityResolver {
    /// Creates a new entity resolver.
    pub fn new(threshold: f64) -> Self {
        Self {
            threshold: threshold.clamp(0.0, 1.0),
            equivalences: HashMap::new(),
        }
    }

    /// Adds a known equivalence link.
    pub fn add_equivalence(&mut self, entity1: impl Into<String>, entity2: impl Into<String>) {
        let e1 = entity1.into();
        let e2 = entity2.into();
        self.equivalences.insert(e1.clone(), e2.clone());
        self.equivalences.insert(e2, e1);
    }

    /// Finds potential matches based on string similarity.
    pub fn find_matches(&self, entity: &str, candidates: &[String]) -> Vec<(String, f64)> {
        let mut matches: Vec<(String, f64)> = candidates
            .iter()
            .map(|candidate| {
                let similarity = self.string_similarity(entity, candidate);
                (candidate.clone(), similarity)
            })
            .filter(|(_, sim)| *sim >= self.threshold)
            .collect();

        matches.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        matches
    }

    /// Resolves an entity to its canonical form.
    pub fn resolve(&self, entity: &str) -> String {
        self.equivalences
            .get(entity)
            .cloned()
            .unwrap_or_else(|| entity.to_string())
    }

    /// Generates owl:sameAs triples for equivalences.
    pub fn generate_sameas_triples(&self) -> Vec<Triple> {
        let mut triples = Vec::new();
        let mut seen = HashSet::new();

        for (e1, e2) in &self.equivalences {
            let key = if e1 < e2 {
                (e1.clone(), e2.clone())
            } else {
                (e2.clone(), e1.clone())
            };

            if seen.insert(key.clone()) {
                triples.push(Triple {
                    subject: key.0,
                    predicate: "owl:sameAs".to_string(),
                    object: RdfValue::Uri(key.1),
                });
            }
        }

        triples
    }

    fn string_similarity(&self, s1: &str, s2: &str) -> f64 {
        // Simple Levenshtein distance-based similarity
        let distance = levenshtein_distance(s1, s2);
        let max_len = s1.len().max(s2.len());
        if max_len == 0 {
            1.0
        } else {
            1.0 - (distance as f64 / max_len as f64)
        }
    }

    /// Returns the number of equivalences.
    pub fn num_equivalences(&self) -> usize {
        self.equivalences.len() / 2 // Divided by 2 because we store bidirectional
    }
}

/// Knowledge graph merger for combining multiple graphs.
#[derive(Debug)]
pub struct KnowledgeGraphMerger {
    /// Merged triples
    triples: Vec<Triple>,
    /// Entity resolver for deduplication
    resolver: EntityResolver,
    /// Source tracking (triple index -> source)
    sources: HashMap<usize, String>,
}

impl KnowledgeGraphMerger {
    /// Creates a new knowledge graph merger.
    pub fn new(resolver: EntityResolver) -> Self {
        Self {
            triples: Vec::new(),
            resolver,
            sources: HashMap::new(),
        }
    }

    /// Adds triples from a source.
    pub fn add_source(&mut self, source_name: impl Into<String>, triples: Vec<Triple>) {
        let source = source_name.into();
        let start_idx = self.triples.len();

        for (i, triple) in triples.into_iter().enumerate() {
            // Resolve entities
            let resolved_triple = Triple {
                subject: self.resolver.resolve(&triple.subject),
                predicate: triple.predicate,
                object: match triple.object {
                    RdfValue::Uri(uri) => RdfValue::Uri(self.resolver.resolve(&uri)),
                    other => other,
                },
            };

            self.triples.push(resolved_triple);
            self.sources.insert(start_idx + i, source.clone());
        }
    }

    /// Gets all merged triples.
    pub fn get_triples(&self) -> &[Triple] {
        &self.triples
    }

    /// Gets the source of a triple by index.
    pub fn get_source(&self, index: usize) -> Option<&String> {
        self.sources.get(&index)
    }

    /// Gets triples from a specific source.
    pub fn get_triples_from_source(&self, source: &str) -> Vec<&Triple> {
        self.sources
            .iter()
            .filter_map(|(idx, src)| {
                if src == source {
                    self.triples.get(*idx)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Returns the number of sources.
    pub fn num_sources(&self) -> usize {
        self.sources.values().collect::<HashSet<_>>().len()
    }

    /// Returns the total number of triples.
    pub fn num_triples(&self) -> usize {
        self.triples.len()
    }
}

/// Conflict detector for identifying contradictions in merged data.
#[derive(Debug)]
pub struct ConflictDetector {
    /// Conflicts found (subject -> predicate -> [conflicting values])
    conflicts: HashMap<String, HashMap<String, Vec<RdfValue>>>,
}

impl ConflictDetector {
    /// Creates a new conflict detector.
    pub fn new() -> Self {
        Self {
            conflicts: HashMap::new(),
        }
    }

    /// Detects conflicts in a set of triples.
    pub fn detect(&mut self, triples: &[Triple]) {
        let mut value_map: HashMap<String, HashMap<String, Vec<RdfValue>>> = HashMap::new();

        // Group by subject and predicate
        for triple in triples {
            value_map
                .entry(triple.subject.clone())
                .or_default()
                .entry(triple.predicate.clone())
                .or_default()
                .push(triple.object.clone());
        }

        // Find conflicts (multiple different values for same subject-predicate)
        for (subject, predicates) in value_map {
            for (predicate, values) in predicates {
                if values.len() > 1 {
                    // Check if values are actually different
                    let unique_values: HashSet<String> =
                        values.iter().map(|v| format!("{:?}", v)).collect();

                    if unique_values.len() > 1 {
                        self.conflicts
                            .entry(subject.clone())
                            .or_default()
                            .insert(predicate, values);
                    }
                }
            }
        }
    }

    /// Gets all conflicts.
    pub fn get_conflicts(&self) -> &HashMap<String, HashMap<String, Vec<RdfValue>>> {
        &self.conflicts
    }

    /// Checks if there are any conflicts.
    pub fn has_conflicts(&self) -> bool {
        !self.conflicts.is_empty()
    }

    /// Returns the number of conflicting subjects.
    pub fn num_conflicts(&self) -> usize {
        self.conflicts.len()
    }

    /// Resolves conflicts using a strategy.
    pub fn resolve(&self, strategy: ConflictResolutionStrategy) -> Vec<Triple> {
        let mut resolved = Vec::new();

        for (subject, predicates) in &self.conflicts {
            for (predicate, values) in predicates {
                let selected_value = match strategy {
                    ConflictResolutionStrategy::TakeFirst => values.first(),
                    ConflictResolutionStrategy::TakeLast => values.last(),
                    ConflictResolutionStrategy::TakeMostCommon => {
                        // Find most common value
                        let mut counts: HashMap<String, usize> = HashMap::new();
                        for val in values {
                            *counts.entry(format!("{:?}", val)).or_default() += 1;
                        }
                        if let Some(most_common) = counts.iter().max_by_key(|(_, count)| *count) {
                            values.iter().find(|v| format!("{:?}", v) == *most_common.0)
                        } else {
                            None
                        }
                    }
                };

                if let Some(value) = selected_value {
                    resolved.push(Triple {
                        subject: subject.clone(),
                        predicate: predicate.clone(),
                        object: value.clone(),
                    });
                }
            }
        }

        resolved
    }
}

impl Default for ConflictDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Strategy for resolving conflicts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConflictResolutionStrategy {
    /// Take the first value encountered
    TakeFirst,
    /// Take the last value encountered
    TakeLast,
    /// Take the most common value
    TakeMostCommon,
}

/// Provenance tracker for fused data.
#[derive(Debug, Clone)]
pub struct FusionProvenance {
    /// Source of each triple
    pub source: String,
    /// Timestamp when added
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Confidence score
    pub confidence: f64,
    /// Transformation applied (if any)
    pub transformation: Option<String>,
}

impl FusionProvenance {
    /// Creates new provenance information.
    pub fn new(source: impl Into<String>, confidence: f64) -> Self {
        Self {
            source: source.into(),
            timestamp: chrono::Utc::now(),
            confidence: confidence.clamp(0.0, 1.0),
            transformation: None,
        }
    }

    /// Sets the transformation.
    pub fn with_transformation(mut self, transformation: impl Into<String>) -> Self {
        self.transformation = Some(transformation.into());
        self
    }

    /// Converts provenance to RDF triples.
    pub fn to_triples(&self, triple_uri: &str) -> Vec<Triple> {
        let mut triples = Vec::new();

        triples.push(Triple {
            subject: triple_uri.to_string(),
            predicate: "prov:wasDerivedFrom".to_string(),
            object: RdfValue::Uri(self.source.clone()),
        });

        triples.push(Triple {
            subject: triple_uri.to_string(),
            predicate: "prov:generatedAtTime".to_string(),
            object: RdfValue::datetime(self.timestamp),
        });

        triples.push(Triple {
            subject: triple_uri.to_string(),
            predicate: "legalis:confidence".to_string(),
            object: RdfValue::TypedLiteral(self.confidence.to_string(), "xsd:double".to_string()),
        });

        if let Some(ref trans) = self.transformation {
            triples.push(Triple {
                subject: triple_uri.to_string(),
                predicate: "legalis:transformation".to_string(),
                object: RdfValue::Literal(trans.clone(), None),
            });
        }

        triples
    }
}

/// Calculates Levenshtein distance between two strings.
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.len();
    let len2 = s2.len();

    if len1 == 0 {
        return len2;
    }
    if len2 == 0 {
        return len1;
    }

    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

    for (i, row) in matrix.iter_mut().enumerate().take(len1 + 1) {
        row[0] = i;
    }
    for j in 0..=len2 {
        matrix[0][j] = j;
    }

    for (i, c1) in s1.chars().enumerate() {
        for (j, c2) in s2.chars().enumerate() {
            let cost = if c1 == c2 { 0 } else { 1 };
            matrix[i + 1][j + 1] = (matrix[i][j + 1] + 1)
                .min(matrix[i + 1][j] + 1)
                .min(matrix[i][j] + cost);
        }
    }

    matrix[len1][len2]
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
    fn test_ontology_mapping_new() {
        let mapping = OntologyMapping::new("http://example.org/ont1#", "http://example.org/ont2#");
        assert_eq!(mapping.source_namespace, "http://example.org/ont1#");
        assert_eq!(mapping.confidence, 1.0);
    }

    #[test]
    fn test_add_property_mapping() {
        let mut mapping = OntologyMapping::new("ont1", "ont2");
        mapping.add_property_mapping("ont1:knows", "ont2:acquainted");
        assert_eq!(
            mapping.property_mappings.get("ont1:knows"),
            Some(&"ont2:acquainted".to_string())
        );
    }

    #[test]
    fn test_transform_triple() {
        let mut mapping = OntologyMapping::new("ont1", "ont2");
        mapping.add_property_mapping("ont1:knows", "ont2:acquainted");

        let triple = sample_triple("Alice", "ont1:knows", "Bob");
        let transformed = mapping.transform_triple(&triple).unwrap();

        assert_eq!(transformed.predicate, "ont2:acquainted");
        assert_eq!(transformed.subject, "Alice");
    }

    #[test]
    fn test_entity_resolver_new() {
        let resolver = EntityResolver::new(0.8);
        assert_eq!(resolver.threshold, 0.8);
    }

    #[test]
    fn test_add_equivalence() {
        let mut resolver = EntityResolver::new(0.8);
        resolver.add_equivalence("http://ex.org/Alice", "http://other.org/AliceSmith");

        assert_eq!(
            resolver.resolve("http://ex.org/Alice"),
            "http://other.org/AliceSmith"
        );
        assert_eq!(
            resolver.resolve("http://other.org/AliceSmith"),
            "http://ex.org/Alice"
        );
    }

    #[test]
    fn test_generate_sameas_triples() {
        let mut resolver = EntityResolver::new(0.8);
        resolver.add_equivalence("A", "B");
        resolver.add_equivalence("C", "D");

        let triples = resolver.generate_sameas_triples();
        assert_eq!(triples.len(), 2);
        assert!(triples.iter().all(|t| t.predicate == "owl:sameAs"));
    }

    #[test]
    fn test_string_similarity() {
        let resolver = EntityResolver::new(0.8);
        let sim1 = resolver.string_similarity("hello", "hello");
        assert!((sim1 - 1.0).abs() < 0.01);

        let sim2 = resolver.string_similarity("hello", "hallo");
        assert!(sim2 > 0.7 && sim2 < 1.0);
    }

    #[test]
    fn test_knowledge_graph_merger() {
        let resolver = EntityResolver::new(0.8);
        let mut merger = KnowledgeGraphMerger::new(resolver);

        merger.add_source("source1", vec![sample_triple("A", "knows", "B")]);
        merger.add_source("source2", vec![sample_triple("C", "likes", "D")]);

        assert_eq!(merger.num_triples(), 2);
        assert_eq!(merger.num_sources(), 2);
    }

    #[test]
    fn test_get_triples_from_source() {
        let resolver = EntityResolver::new(0.8);
        let mut merger = KnowledgeGraphMerger::new(resolver);

        merger.add_source("source1", vec![sample_triple("A", "knows", "B")]);
        merger.add_source("source2", vec![sample_triple("C", "likes", "D")]);

        let source1_triples = merger.get_triples_from_source("source1");
        assert_eq!(source1_triples.len(), 1);
    }

    #[test]
    fn test_conflict_detector_new() {
        let detector = ConflictDetector::new();
        assert!(!detector.has_conflicts());
    }

    #[test]
    fn test_detect_conflicts() {
        let mut detector = ConflictDetector::new();
        let triples = vec![
            sample_triple("Alice", "age", "30"),
            sample_triple("Alice", "age", "31"),
        ];

        detector.detect(&triples);
        assert!(detector.has_conflicts());
        assert_eq!(detector.num_conflicts(), 1);
    }

    #[test]
    fn test_no_conflict_same_values() {
        let mut detector = ConflictDetector::new();
        let triples = vec![
            sample_triple("Alice", "knows", "Bob"),
            sample_triple("Alice", "knows", "Bob"),
        ];

        detector.detect(&triples);
        assert!(!detector.has_conflicts());
    }

    #[test]
    fn test_resolve_conflicts() {
        let mut detector = ConflictDetector::new();
        let triples = vec![
            sample_triple("Alice", "age", "30"),
            sample_triple("Alice", "age", "31"),
        ];

        detector.detect(&triples);
        let resolved = detector.resolve(ConflictResolutionStrategy::TakeFirst);
        assert_eq!(resolved.len(), 1);
    }

    #[test]
    fn test_fusion_provenance() {
        let prov = FusionProvenance::new("http://source1.org", 0.9);
        assert_eq!(prov.source, "http://source1.org");
        assert_eq!(prov.confidence, 0.9);
    }

    #[test]
    fn test_provenance_to_triples() {
        let prov = FusionProvenance::new("http://source1.org", 0.9)
            .with_transformation("ontology_mapping");

        let triples = prov.to_triples("http://triple/1");
        assert!(triples.len() >= 3);
        assert!(triples.iter().any(|t| t.predicate == "prov:wasDerivedFrom"));
        assert!(
            triples
                .iter()
                .any(|t| t.predicate == "legalis:transformation")
        );
    }

    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(levenshtein_distance("", ""), 0);
        assert_eq!(levenshtein_distance("hello", "hello"), 0);
        assert_eq!(levenshtein_distance("hello", "hallo"), 1);
        assert_eq!(levenshtein_distance("hello", ""), 5);
    }

    #[test]
    fn test_find_matches() {
        let resolver = EntityResolver::new(0.7);
        let candidates = vec!["Alice".to_string(), "Alicia".to_string(), "Bob".to_string()];

        let matches = resolver.find_matches("Alice", &candidates);
        assert!(!matches.is_empty());
        assert_eq!(matches[0].0, "Alice");
    }

    #[test]
    fn test_conflict_resolution_strategies() {
        let mut detector = ConflictDetector::new();
        let triples = vec![
            sample_triple("Alice", "age", "30"),
            sample_triple("Alice", "age", "31"),
            sample_triple("Alice", "age", "30"),
        ];

        detector.detect(&triples);

        let first = detector.resolve(ConflictResolutionStrategy::TakeFirst);
        assert_eq!(first.len(), 1);

        let last = detector.resolve(ConflictResolutionStrategy::TakeLast);
        assert_eq!(last.len(), 1);

        let common = detector.resolve(ConflictResolutionStrategy::TakeMostCommon);
        assert_eq!(common.len(), 1);
    }

    #[test]
    fn test_transform_triples_batch() {
        let mut mapping = OntologyMapping::new("ont1", "ont2");
        mapping.add_property_mapping("ont1:knows", "ont2:acquainted");

        let triples = vec![
            sample_triple("Alice", "ont1:knows", "Bob"),
            sample_triple("Bob", "ont1:knows", "Charlie"),
        ];

        let transformed = mapping.transform_triples(&triples);
        assert_eq!(transformed.len(), 2);
        assert!(transformed.iter().all(|t| t.predicate == "ont2:acquainted"));
    }
}
