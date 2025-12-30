//! Knowledge graph construction and entity relationship extraction.
//!
//! This module provides:
//! - Statute knowledge graph construction
//! - Entity relationship extraction
//! - Temporal knowledge graph support
//! - Graph visualization export

use crate::{RdfValue, Triple};
use chrono::NaiveDate;
use legalis_core::{Condition, Statute};
use std::collections::{HashMap, HashSet};

/// Knowledge graph builder for statutes.
#[derive(Debug)]
pub struct KnowledgeGraphBuilder {
    base_uri: String,
    entities: HashMap<String, Entity>,
    relationships: Vec<Relationship>,
    temporal_snapshots: Vec<TemporalSnapshot>,
}

impl KnowledgeGraphBuilder {
    /// Creates a new knowledge graph builder.
    pub fn new(base_uri: impl Into<String>) -> Self {
        Self {
            base_uri: base_uri.into(),
            entities: HashMap::new(),
            relationships: Vec::new(),
            temporal_snapshots: Vec::new(),
        }
    }

    /// Adds a statute to the knowledge graph.
    pub fn add_statute(&mut self, statute: &Statute) {
        let statute_uri = format!("{}statute/{}", self.base_uri, statute.id);

        // Create statute entity
        let mut entity = Entity::new(&statute_uri, EntityType::Statute);
        entity.add_label(&statute.title);
        if let Some(ref jurisdiction) = statute.jurisdiction {
            entity.add_property("jurisdiction", jurisdiction);
        }
        self.entities.insert(statute_uri.clone(), entity);

        // Extract entities from conditions
        self.extract_entities_from_conditions(&statute_uri, &statute.preconditions);

        // Create temporal snapshot if temporal validity exists
        if statute.temporal_validity.effective_date.is_some()
            || statute.temporal_validity.expiry_date.is_some()
        {
            self.add_temporal_snapshot(
                &statute_uri,
                statute.temporal_validity.effective_date,
                statute.temporal_validity.expiry_date,
            );
        }
    }

    /// Extracts entities from conditions.
    fn extract_entities_from_conditions(&mut self, statute_uri: &str, conditions: &[Condition]) {
        for (i, condition) in conditions.iter().enumerate() {
            let condition_uri = format!("{}/condition/{}", statute_uri, i);

            // Create condition entity
            let mut entity = Entity::new(&condition_uri, EntityType::Condition);
            entity.add_label(&format!("{:?}", condition));
            self.entities.insert(condition_uri.clone(), entity);

            // Create relationship
            self.relationships.push(Relationship {
                subject: statute_uri.to_string(),
                predicate: RelationType::HasCondition,
                object: condition_uri.clone(),
            });

            // Recursively extract from nested conditions
            self.extract_nested_entities(condition, &condition_uri);
        }
    }

    /// Extracts entities from nested conditions.
    fn extract_nested_entities(&mut self, condition: &Condition, parent_uri: &str) {
        match condition {
            Condition::And(left, right) | Condition::Or(left, right) => {
                let left_uri = format!("{}/left", parent_uri);
                let right_uri = format!("{}/right", parent_uri);

                let mut left_entity = Entity::new(&left_uri, EntityType::Condition);
                left_entity.add_label(&format!("{:?}", left));
                self.entities.insert(left_uri.clone(), left_entity);

                let mut right_entity = Entity::new(&right_uri, EntityType::Condition);
                right_entity.add_label(&format!("{:?}", right));
                self.entities.insert(right_uri.clone(), right_entity);

                self.relationships.push(Relationship {
                    subject: parent_uri.to_string(),
                    predicate: RelationType::HasLeftOperand,
                    object: left_uri.clone(),
                });

                self.relationships.push(Relationship {
                    subject: parent_uri.to_string(),
                    predicate: RelationType::HasRightOperand,
                    object: right_uri.clone(),
                });

                self.extract_nested_entities(left, &left_uri);
                self.extract_nested_entities(right, &right_uri);
            }
            Condition::Not(inner) => {
                let inner_uri = format!("{}/inner", parent_uri);
                let mut inner_entity = Entity::new(&inner_uri, EntityType::Condition);
                inner_entity.add_label(&format!("{:?}", inner));
                self.entities.insert(inner_uri.clone(), inner_entity);

                self.relationships.push(Relationship {
                    subject: parent_uri.to_string(),
                    predicate: RelationType::HasOperand,
                    object: inner_uri.clone(),
                });

                self.extract_nested_entities(inner, &inner_uri);
            }
            _ => {}
        }
    }

    /// Adds a temporal snapshot to the knowledge graph.
    pub fn add_temporal_snapshot(
        &mut self,
        entity_uri: &str,
        start: Option<NaiveDate>,
        end: Option<NaiveDate>,
    ) {
        self.temporal_snapshots.push(TemporalSnapshot {
            entity_uri: entity_uri.to_string(),
            valid_from: start,
            valid_until: end,
        });
    }

    /// Adds a custom relationship between entities.
    pub fn add_relationship(&mut self, subject: &str, predicate: RelationType, object: &str) {
        self.relationships.push(Relationship {
            subject: subject.to_string(),
            predicate,
            object: object.to_string(),
        });
    }

    /// Builds the knowledge graph as RDF triples.
    pub fn build(&self) -> Vec<Triple> {
        let mut triples = Vec::new();

        // Entity triples
        for entity in self.entities.values() {
            triples.extend(entity.to_triples());
        }

        // Relationship triples
        for rel in &self.relationships {
            triples.extend(rel.to_triples());
        }

        // Temporal triples
        for snapshot in &self.temporal_snapshots {
            triples.extend(snapshot.to_triples());
        }

        triples
    }

    /// Gets all entities of a specific type.
    pub fn get_entities_by_type(&self, entity_type: EntityType) -> Vec<&Entity> {
        self.entities
            .values()
            .filter(|e| e.entity_type == entity_type)
            .collect()
    }

    /// Finds related entities.
    pub fn find_related_entities(&self, entity_uri: &str) -> Vec<&Entity> {
        let mut related_uris: HashSet<String> = HashSet::new();

        for rel in &self.relationships {
            if rel.subject == entity_uri {
                related_uris.insert(rel.object.clone());
            } else if rel.object == entity_uri {
                related_uris.insert(rel.subject.clone());
            }
        }

        related_uris
            .iter()
            .filter_map(|uri| self.entities.get(uri))
            .collect()
    }

    /// Exports graph visualization data in DOT format.
    pub fn export_dot(&self) -> String {
        let mut dot = String::from("digraph KnowledgeGraph {\n");
        dot.push_str("  rankdir=LR;\n");
        dot.push_str("  node [shape=box];\n\n");

        // Nodes
        for (uri, entity) in &self.entities {
            let node_id = uri.replace([':', '/', '.', '-'], "_");
            let label = entity.label.as_deref().unwrap_or(uri);
            dot.push_str(&format!(
                "  {} [label=\"{}\"];\n",
                node_id,
                escape_dot_label(label)
            ));
        }

        dot.push('\n');

        // Edges
        for rel in &self.relationships {
            let subject_id = rel.subject.replace([':', '/', '.', '-'], "_");
            let object_id = rel.object.replace([':', '/', '.', '-'], "_");
            let label = rel.predicate.label();
            dot.push_str(&format!(
                "  {} -> {} [label=\"{}\"];\n",
                subject_id, object_id, label
            ));
        }

        dot.push_str("}\n");
        dot
    }

    /// Finds the shortest path between two entities using BFS.
    pub fn find_shortest_path(&self, start: &str, end: &str) -> Option<Vec<String>> {
        if start == end {
            return Some(vec![start.to_string()]);
        }

        let mut queue = std::collections::VecDeque::new();
        let mut visited = HashSet::new();
        let mut parent: HashMap<String, String> = HashMap::new();

        queue.push_back(start.to_string());
        visited.insert(start.to_string());

        while let Some(current) = queue.pop_front() {
            if current == end {
                // Reconstruct path
                let mut path = vec![end.to_string()];
                let mut node = end;
                while let Some(prev) = parent.get(node) {
                    path.push(prev.clone());
                    node = prev;
                }
                path.reverse();
                return Some(path);
            }

            // Find neighbors
            for rel in &self.relationships {
                if rel.subject == current && !visited.contains(&rel.object) {
                    visited.insert(rel.object.clone());
                    parent.insert(rel.object.clone(), current.clone());
                    queue.push_back(rel.object.clone());
                }
            }
        }

        None
    }

    /// Finds all paths from start to end with a maximum depth.
    pub fn find_all_paths(&self, start: &str, end: &str, max_depth: usize) -> Vec<Vec<String>> {
        let mut all_paths = Vec::new();
        let mut current_path = vec![start.to_string()];
        let mut visited = HashSet::new();
        visited.insert(start.to_string());

        self.find_paths_recursive(
            start,
            end,
            max_depth,
            &mut current_path,
            &mut visited,
            &mut all_paths,
        );

        all_paths
    }

    #[allow(clippy::too_many_arguments)]
    fn find_paths_recursive(
        &self,
        current: &str,
        end: &str,
        remaining_depth: usize,
        path: &mut Vec<String>,
        visited: &mut HashSet<String>,
        all_paths: &mut Vec<Vec<String>>,
    ) {
        if current == end {
            all_paths.push(path.clone());
            return;
        }

        if remaining_depth == 0 {
            return;
        }

        for rel in &self.relationships {
            if rel.subject == current && !visited.contains(&rel.object) {
                visited.insert(rel.object.clone());
                path.push(rel.object.clone());

                self.find_paths_recursive(
                    &rel.object,
                    end,
                    remaining_depth - 1,
                    path,
                    visited,
                    all_paths,
                );

                path.pop();
                visited.remove(&rel.object);
            }
        }
    }

    /// Calculates the degree centrality for all entities.
    /// Returns a map of entity URI -> (in-degree, out-degree).
    pub fn calculate_degree_centrality(&self) -> HashMap<String, (usize, usize)> {
        let mut centrality: HashMap<String, (usize, usize)> = HashMap::new();

        // Initialize all entities with (0, 0)
        for entity_uri in self.entities.keys() {
            centrality.insert(entity_uri.clone(), (0, 0));
        }

        // Count in-degree and out-degree
        for rel in &self.relationships {
            centrality.entry(rel.subject.clone()).or_insert((0, 0)).1 += 1; // out-degree

            centrality.entry(rel.object.clone()).or_insert((0, 0)).0 += 1; // in-degree
        }

        centrality
    }

    /// Finds connected components in the graph (treating it as undirected).
    pub fn find_connected_components(&self) -> Vec<HashSet<String>> {
        let mut components = Vec::new();
        let mut visited = HashSet::new();

        for entity_uri in self.entities.keys() {
            if !visited.contains(entity_uri) {
                let mut component = HashSet::new();
                self.explore_component(entity_uri, &mut component, &mut visited);
                components.push(component);
            }
        }

        components
    }

    fn explore_component(
        &self,
        start: &str,
        component: &mut HashSet<String>,
        visited: &mut HashSet<String>,
    ) {
        if visited.contains(start) {
            return;
        }

        visited.insert(start.to_string());
        component.insert(start.to_string());

        // Explore neighbors (both directions)
        for rel in &self.relationships {
            if rel.subject == start {
                self.explore_component(&rel.object, component, visited);
            }
            if rel.object == start {
                self.explore_component(&rel.subject, component, visited);
            }
        }
    }

    /// Finds entities with the highest degree centrality.
    pub fn find_most_connected_entities(&self, limit: usize) -> Vec<(String, usize)> {
        let centrality = self.calculate_degree_centrality();
        let mut ranked: Vec<_> = centrality
            .into_iter()
            .map(|(uri, (in_deg, out_deg))| (uri, in_deg + out_deg))
            .collect();

        ranked.sort_by(|a, b| b.1.cmp(&a.1));
        ranked.truncate(limit);
        ranked
    }

    /// Finds entities that are highly referenced (high in-degree).
    pub fn find_most_referenced_entities(&self, limit: usize) -> Vec<(String, usize)> {
        let centrality = self.calculate_degree_centrality();
        let mut ranked: Vec<_> = centrality
            .into_iter()
            .map(|(uri, (in_deg, _))| (uri, in_deg))
            .filter(|(_, deg)| *deg > 0)
            .collect();

        ranked.sort_by(|a, b| b.1.cmp(&a.1));
        ranked.truncate(limit);
        ranked
    }
}

/// Entity in the knowledge graph.
#[derive(Debug, Clone)]
pub struct Entity {
    uri: String,
    entity_type: EntityType,
    label: Option<String>,
    properties: HashMap<String, String>,
}

impl Entity {
    /// Creates a new entity.
    pub fn new(uri: &str, entity_type: EntityType) -> Self {
        Self {
            uri: uri.to_string(),
            entity_type,
            label: None,
            properties: HashMap::new(),
        }
    }

    /// Adds a label to the entity.
    pub fn add_label(&mut self, label: &str) {
        self.label = Some(label.to_string());
    }

    /// Adds a property to the entity.
    pub fn add_property(&mut self, key: &str, value: &str) {
        self.properties.insert(key.to_string(), value.to_string());
    }

    /// Converts the entity to RDF triples.
    pub fn to_triples(&self) -> Vec<Triple> {
        let mut triples = Vec::new();

        // Type
        triples.push(Triple {
            subject: self.uri.clone(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri(self.entity_type.uri()),
        });

        // Label
        if let Some(ref label) = self.label {
            triples.push(Triple {
                subject: self.uri.clone(),
                predicate: "rdfs:label".to_string(),
                object: RdfValue::string(label),
            });
        }

        // Properties
        for (key, value) in &self.properties {
            triples.push(Triple {
                subject: self.uri.clone(),
                predicate: format!("legalis:{}", key),
                object: RdfValue::string(value),
            });
        }

        triples
    }
}

/// Entity types in the knowledge graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityType {
    /// Statute
    Statute,
    /// Condition
    Condition,
    /// Effect
    Effect,
    /// Agent (person or organization)
    Agent,
    /// Event
    Event,
    /// Concept
    Concept,
}

impl EntityType {
    /// Returns the URI for this entity type.
    pub fn uri(&self) -> String {
        let type_name = match self {
            Self::Statute => "Statute",
            Self::Condition => "Condition",
            Self::Effect => "Effect",
            Self::Agent => "Agent",
            Self::Event => "Event",
            Self::Concept => "Concept",
        };
        format!("https://legalis.dev/ontology#{}", type_name)
    }
}

/// Relationship between entities.
#[derive(Debug, Clone)]
pub struct Relationship {
    subject: String,
    predicate: RelationType,
    object: String,
}

impl Relationship {
    /// Converts the relationship to RDF triples.
    pub fn to_triples(&self) -> Vec<Triple> {
        vec![Triple {
            subject: self.subject.clone(),
            predicate: self.predicate.uri(),
            object: RdfValue::Uri(self.object.clone()),
        }]
    }
}

/// Relationship types in the knowledge graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RelationType {
    /// Has condition
    HasCondition,
    /// Has effect
    HasEffect,
    /// References
    References,
    /// Amends
    Amends,
    /// Replaces
    Replaces,
    /// Supersedes
    Supersedes,
    /// Implements
    Implements,
    /// Has left operand
    HasLeftOperand,
    /// Has right operand
    HasRightOperand,
    /// Has operand
    HasOperand,
}

impl RelationType {
    /// Returns the URI for this relation type.
    pub fn uri(&self) -> String {
        let relation_name = match self {
            Self::HasCondition => "hasPrecondition",
            Self::HasEffect => "hasEffect",
            Self::References => "references",
            Self::Amends => "amends",
            Self::Replaces => "replaces",
            Self::Supersedes => "supersedes",
            Self::Implements => "implements",
            Self::HasLeftOperand => "leftOperand",
            Self::HasRightOperand => "rightOperand",
            Self::HasOperand => "operand",
        };
        format!("legalis:{}", relation_name)
    }

    /// Returns a human-readable label.
    pub fn label(&self) -> &'static str {
        match self {
            Self::HasCondition => "has condition",
            Self::HasEffect => "has effect",
            Self::References => "references",
            Self::Amends => "amends",
            Self::Replaces => "replaces",
            Self::Supersedes => "supersedes",
            Self::Implements => "implements",
            Self::HasLeftOperand => "left",
            Self::HasRightOperand => "right",
            Self::HasOperand => "operand",
        }
    }
}

/// Temporal snapshot for time-varying entities.
#[derive(Debug, Clone)]
pub struct TemporalSnapshot {
    entity_uri: String,
    valid_from: Option<NaiveDate>,
    valid_until: Option<NaiveDate>,
}

impl TemporalSnapshot {
    /// Converts the temporal snapshot to RDF triples.
    pub fn to_triples(&self) -> Vec<Triple> {
        let mut triples = Vec::new();
        let snapshot_uri = format!("{}/temporal-snapshot", self.entity_uri);

        triples.push(Triple {
            subject: snapshot_uri.clone(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("https://legalis.dev/ontology#TemporalSnapshot".to_string()),
        });

        triples.push(Triple {
            subject: snapshot_uri.clone(),
            predicate: "legalis:snapshotOf".to_string(),
            object: RdfValue::Uri(self.entity_uri.clone()),
        });

        if let Some(from) = self.valid_from {
            triples.push(Triple {
                subject: snapshot_uri.clone(),
                predicate: "legalis:validFrom".to_string(),
                object: RdfValue::date(from),
            });
        }

        if let Some(until) = self.valid_until {
            triples.push(Triple {
                subject: snapshot_uri,
                predicate: "legalis:validUntil".to_string(),
                object: RdfValue::date(until),
            });
        }

        triples
    }
}

/// Escapes a label for DOT format.
fn escape_dot_label(s: &str) -> String {
    s.replace('"', "\\\"").replace('\n', "\\n")
}

/// Graph reasoning engine for inferring new relationships.
#[derive(Debug)]
pub struct ReasoningEngine {
    rules: Vec<InferenceRule>,
}

impl ReasoningEngine {
    /// Creates a new reasoning engine with default rules.
    pub fn new() -> Self {
        Self {
            rules: Self::default_rules(),
        }
    }

    /// Returns default inference rules.
    fn default_rules() -> Vec<InferenceRule> {
        vec![
            // Transitive closure for References
            InferenceRule {
                name: "transitive_references".to_string(),
                description: "If A references B and B references C, then A references C"
                    .to_string(),
                pattern: InferencePattern::Transitive(RelationType::References),
            },
            // Transitive closure for Supersedes
            InferenceRule {
                name: "transitive_supersedes".to_string(),
                description: "If A supersedes B and B supersedes C, then A supersedes C"
                    .to_string(),
                pattern: InferencePattern::Transitive(RelationType::Supersedes),
            },
            // Implication: Replaces implies Supersedes
            InferenceRule {
                name: "replaces_implies_supersedes".to_string(),
                description: "If A replaces B, then A supersedes B".to_string(),
                pattern: InferencePattern::Implication {
                    antecedent: RelationType::Replaces,
                    consequent: RelationType::Supersedes,
                },
            },
            // Symmetry for certain relations
            InferenceRule {
                name: "symmetric_references".to_string(),
                description: "If A references B, infer B references A (for mutual references)"
                    .to_string(),
                pattern: InferencePattern::Symmetric(RelationType::References),
            },
        ]
    }

    /// Adds a custom inference rule.
    pub fn add_rule(&mut self, rule: InferenceRule) {
        self.rules.push(rule);
    }

    /// Applies all inference rules to infer new relationships.
    pub fn infer(&self, relationships: &[Relationship]) -> Vec<Relationship> {
        let mut inferred = Vec::new();
        let mut seen = HashSet::new();

        for rule in &self.rules {
            let new_rels = match &rule.pattern {
                InferencePattern::Transitive(rel_type) => {
                    self.apply_transitive(relationships, *rel_type)
                }
                InferencePattern::Implication {
                    antecedent,
                    consequent,
                } => self.apply_implication(relationships, *antecedent, *consequent),
                InferencePattern::Symmetric(rel_type) => {
                    self.apply_symmetric(relationships, *rel_type)
                }
            };

            // Deduplicate inferred relationships
            for rel in new_rels {
                let key = (rel.subject.clone(), rel.predicate, rel.object.clone());
                if seen.insert(key) {
                    inferred.push(rel);
                }
            }
        }

        inferred
    }

    /// Applies transitive closure for a relation type.
    fn apply_transitive(
        &self,
        relationships: &[Relationship],
        rel_type: RelationType,
    ) -> Vec<Relationship> {
        let mut inferred = Vec::new();

        // Find all relationships of this type
        let typed_rels: Vec<_> = relationships
            .iter()
            .filter(|r| r.predicate == rel_type)
            .collect();

        // For each pair, check if transitive closure applies
        for rel1 in &typed_rels {
            for rel2 in &typed_rels {
                if rel1.object == rel2.subject {
                    // Create inferred relationship: rel1.subject -> rel2.object
                    let new_rel = Relationship {
                        subject: rel1.subject.clone(),
                        predicate: rel_type,
                        object: rel2.object.clone(),
                    };

                    // Only add if not already in original relationships
                    if !relationships.iter().any(|r| {
                        r.subject == new_rel.subject
                            && r.predicate == new_rel.predicate
                            && r.object == new_rel.object
                    }) {
                        inferred.push(new_rel);
                    }
                }
            }
        }

        inferred
    }

    /// Applies implication rule.
    fn apply_implication(
        &self,
        relationships: &[Relationship],
        antecedent: RelationType,
        consequent: RelationType,
    ) -> Vec<Relationship> {
        let mut inferred = Vec::new();

        for rel in relationships {
            if rel.predicate == antecedent {
                let new_rel = Relationship {
                    subject: rel.subject.clone(),
                    predicate: consequent,
                    object: rel.object.clone(),
                };

                // Only add if not already in original relationships
                if !relationships.iter().any(|r| {
                    r.subject == new_rel.subject
                        && r.predicate == new_rel.predicate
                        && r.object == new_rel.object
                }) {
                    inferred.push(new_rel);
                }
            }
        }

        inferred
    }

    /// Applies symmetric rule.
    fn apply_symmetric(
        &self,
        relationships: &[Relationship],
        rel_type: RelationType,
    ) -> Vec<Relationship> {
        let mut inferred = Vec::new();

        for rel in relationships {
            if rel.predicate == rel_type {
                let new_rel = Relationship {
                    subject: rel.object.clone(),
                    predicate: rel_type,
                    object: rel.subject.clone(),
                };

                // Only add if not already in original relationships
                if !relationships.iter().any(|r| {
                    r.subject == new_rel.subject
                        && r.predicate == new_rel.predicate
                        && r.object == new_rel.object
                }) {
                    inferred.push(new_rel);
                }
            }
        }

        inferred
    }
}

impl Default for ReasoningEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Inference rule for graph reasoning.
#[derive(Debug, Clone)]
pub struct InferenceRule {
    /// Name of the rule
    pub name: String,
    /// Description of the rule
    pub description: String,
    /// Pattern to match
    pub pattern: InferencePattern,
}

/// Inference pattern types.
#[derive(Debug, Clone)]
pub enum InferencePattern {
    /// Transitive closure: if A->B and B->C, then A->C
    Transitive(RelationType),
    /// Implication: if A antecedent B, then A consequent B
    Implication {
        antecedent: RelationType,
        consequent: RelationType,
    },
    /// Symmetric: if A->B, then B->A
    Symmetric(RelationType),
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{ComparisonOp, Effect, EffectType};

    fn sample_statute() -> Statute {
        Statute::new(
            "test-statute",
            "Test Statute",
            Effect::new(EffectType::Grant, "Grant test rights"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        })
    }

    #[test]
    fn test_knowledge_graph_builder() {
        let mut builder = KnowledgeGraphBuilder::new("http://example.org/");
        let statute = sample_statute();
        builder.add_statute(&statute);

        let triples = builder.build();
        assert!(!triples.is_empty());
        assert!(triples.iter().any(|t| t.predicate == "rdf:type"));
        assert!(triples.iter().any(|t| t.predicate == "rdfs:label"));
    }

    #[test]
    fn test_entity_creation() {
        let mut entity = Entity::new("http://example.org/test", EntityType::Statute);
        entity.add_label("Test Statute");
        entity.add_property("jurisdiction", "US");

        let triples = entity.to_triples();
        assert!(!triples.is_empty());
        assert!(triples.iter().any(|t| t.predicate == "rdfs:label"));
    }

    #[test]
    fn test_relationship_triples() {
        let rel = Relationship {
            subject: "http://example.org/statute/1".to_string(),
            predicate: RelationType::HasCondition,
            object: "http://example.org/condition/1".to_string(),
        };

        let triples = rel.to_triples();
        assert_eq!(triples.len(), 1);
        assert!(triples[0].predicate.contains("hasPrecondition"));
    }

    #[test]
    fn test_temporal_snapshot() {
        let snapshot = TemporalSnapshot {
            entity_uri: "http://example.org/statute/1".to_string(),
            valid_from: Some(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap()),
            valid_until: Some(NaiveDate::from_ymd_opt(2025, 12, 31).unwrap()),
        };

        let triples = snapshot.to_triples();
        assert!(!triples.is_empty());
        assert!(triples.iter().any(|t| t.predicate.contains("validFrom")));
        assert!(triples.iter().any(|t| t.predicate.contains("validUntil")));
    }

    #[test]
    fn test_entity_type_uri() {
        assert!(EntityType::Statute.uri().contains("Statute"));
        assert!(EntityType::Condition.uri().contains("Condition"));
    }

    #[test]
    fn test_relation_type_uri() {
        assert!(RelationType::HasCondition.uri().contains("hasPrecondition"));
        assert!(RelationType::Amends.uri().contains("amends"));
    }

    #[test]
    fn test_get_entities_by_type() {
        let mut builder = KnowledgeGraphBuilder::new("http://example.org/");
        let statute = sample_statute();
        builder.add_statute(&statute);

        let statutes = builder.get_entities_by_type(EntityType::Statute);
        assert!(!statutes.is_empty());

        let conditions = builder.get_entities_by_type(EntityType::Condition);
        assert!(!conditions.is_empty());
    }

    #[test]
    fn test_find_related_entities() {
        let mut builder = KnowledgeGraphBuilder::new("http://example.org/");
        let statute = sample_statute();
        builder.add_statute(&statute);

        let statute_uri = format!("{}statute/{}", "http://example.org/", statute.id);
        let related = builder.find_related_entities(&statute_uri);
        assert!(!related.is_empty());
    }

    #[test]
    fn test_export_dot() {
        let mut builder = KnowledgeGraphBuilder::new("http://example.org/");
        let statute = sample_statute();
        builder.add_statute(&statute);

        let dot = builder.export_dot();
        assert!(dot.contains("digraph KnowledgeGraph"));
        assert!(dot.contains("->"));
    }

    #[test]
    fn test_nested_conditions() {
        let statute = Statute::new(
            "complex-law",
            "Complex Law",
            Effect::new(EffectType::Grant, "Test"),
        )
        .with_precondition(Condition::And(
            Box::new(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            }),
            Box::new(Condition::Income {
                operator: ComparisonOp::LessThan,
                value: 50000,
            }),
        ));

        let mut builder = KnowledgeGraphBuilder::new("http://example.org/");
        builder.add_statute(&statute);

        let triples = builder.build();
        assert!(triples.iter().any(|t| t.predicate.contains("leftOperand")));
        assert!(triples.iter().any(|t| t.predicate.contains("rightOperand")));
    }

    #[test]
    fn test_escape_dot_label() {
        assert_eq!(escape_dot_label("test \"quote\""), "test \\\"quote\\\"");
        assert_eq!(escape_dot_label("line\nbreak"), "line\\nbreak");
    }

    #[test]
    fn test_custom_relationship() {
        let mut builder = KnowledgeGraphBuilder::new("http://example.org/");
        builder.add_relationship(
            "http://example.org/statute/1",
            RelationType::Amends,
            "http://example.org/statute/2",
        );

        let triples = builder.build();
        assert!(triples.iter().any(|t| t.predicate.contains("amends")));
    }

    #[test]
    fn test_reasoning_engine_transitive() {
        let relationships = vec![
            Relationship {
                subject: "A".to_string(),
                predicate: RelationType::References,
                object: "B".to_string(),
            },
            Relationship {
                subject: "B".to_string(),
                predicate: RelationType::References,
                object: "C".to_string(),
            },
        ];

        let engine = ReasoningEngine::new();
        let inferred = engine.infer(&relationships);

        // Should infer A -> C
        assert!(inferred.iter().any(|r| r.subject == "A" && r.object == "C"));
    }

    #[test]
    fn test_reasoning_engine_implication() {
        let relationships = vec![Relationship {
            subject: "A".to_string(),
            predicate: RelationType::Replaces,
            object: "B".to_string(),
        }];

        let engine = ReasoningEngine::new();
        let inferred = engine.infer(&relationships);

        // Should infer A supersedes B
        assert!(inferred.iter().any(|r| r.subject == "A"
            && r.predicate == RelationType::Supersedes
            && r.object == "B"));
    }

    #[test]
    fn test_reasoning_engine_symmetric() {
        let relationships = vec![Relationship {
            subject: "A".to_string(),
            predicate: RelationType::References,
            object: "B".to_string(),
        }];

        let engine = ReasoningEngine::new();
        let inferred = engine.infer(&relationships);

        // Should infer B -> A
        assert!(inferred.iter().any(|r| r.subject == "B"
            && r.predicate == RelationType::References
            && r.object == "A"));
    }

    #[test]
    fn test_reasoning_engine_custom_rule() {
        let mut engine = ReasoningEngine::new();
        engine.add_rule(InferenceRule {
            name: "test_rule".to_string(),
            description: "Test rule".to_string(),
            pattern: InferencePattern::Implication {
                antecedent: RelationType::Implements,
                consequent: RelationType::References,
            },
        });

        let relationships = vec![Relationship {
            subject: "A".to_string(),
            predicate: RelationType::Implements,
            object: "B".to_string(),
        }];

        let inferred = engine.infer(&relationships);

        // Should infer A references B
        assert!(inferred.iter().any(|r| r.subject == "A"
            && r.predicate == RelationType::References
            && r.object == "B"));
    }

    #[test]
    fn test_reasoning_engine_no_duplicates() {
        let relationships = vec![
            Relationship {
                subject: "A".to_string(),
                predicate: RelationType::References,
                object: "B".to_string(),
            },
            Relationship {
                subject: "A".to_string(),
                predicate: RelationType::References,
                object: "B".to_string(),
            },
        ];

        let engine = ReasoningEngine::new();
        let inferred = engine.infer(&relationships);

        // Should not create duplicate symmetric relationships
        let count = inferred
            .iter()
            .filter(|r| r.subject == "B" && r.object == "A")
            .count();
        assert_eq!(count, 1);
    }

    // Graph Algorithm Tests
    #[test]
    fn test_shortest_path() {
        let mut builder = KnowledgeGraphBuilder::new("http://example.org/");
        builder.add_relationship("A", RelationType::References, "B");
        builder.add_relationship("B", RelationType::References, "C");
        builder.add_relationship("A", RelationType::References, "D");
        builder.add_relationship("D", RelationType::References, "C");

        let path = builder.find_shortest_path("A", "C");
        assert!(path.is_some());
        let path = path.unwrap();
        assert_eq!(path.len(), 3); // A -> B -> C
        assert_eq!(path[0], "A");
        assert_eq!(path[2], "C");
    }

    #[test]
    fn test_shortest_path_same_node() {
        let builder = KnowledgeGraphBuilder::new("http://example.org/");
        let path = builder.find_shortest_path("A", "A");
        assert_eq!(path, Some(vec!["A".to_string()]));
    }

    #[test]
    fn test_shortest_path_no_path() {
        let mut builder = KnowledgeGraphBuilder::new("http://example.org/");
        builder.add_relationship("A", RelationType::References, "B");
        builder.add_relationship("C", RelationType::References, "D");

        let path = builder.find_shortest_path("A", "D");
        assert!(path.is_none());
    }

    #[test]
    fn test_find_all_paths() {
        let mut builder = KnowledgeGraphBuilder::new("http://example.org/");
        builder.add_relationship("A", RelationType::References, "B");
        builder.add_relationship("B", RelationType::References, "C");
        builder.add_relationship("A", RelationType::References, "D");
        builder.add_relationship("D", RelationType::References, "C");

        let paths = builder.find_all_paths("A", "C", 5);
        assert_eq!(paths.len(), 2); // A->B->C and A->D->C
    }

    #[test]
    fn test_find_all_paths_max_depth() {
        let mut builder = KnowledgeGraphBuilder::new("http://example.org/");
        builder.add_relationship("A", RelationType::References, "B");
        builder.add_relationship("B", RelationType::References, "C");

        let paths = builder.find_all_paths("A", "C", 1); // Max depth 1, can't reach C
        assert_eq!(paths.len(), 0);

        let paths = builder.find_all_paths("A", "C", 2); // Max depth 2, can reach C
        assert_eq!(paths.len(), 1);
    }

    #[test]
    fn test_calculate_degree_centrality() {
        let mut builder = KnowledgeGraphBuilder::new("http://example.org/");

        // Create entities first
        let mut entity_a = Entity::new("A", EntityType::Statute);
        entity_a.add_label("Statute A");
        let mut entity_b = Entity::new("B", EntityType::Statute);
        entity_b.add_label("Statute B");
        let mut entity_c = Entity::new("C", EntityType::Statute);
        entity_c.add_label("Statute C");

        builder.entities.insert("A".to_string(), entity_a);
        builder.entities.insert("B".to_string(), entity_b);
        builder.entities.insert("C".to_string(), entity_c);

        builder.add_relationship("A", RelationType::References, "B");
        builder.add_relationship("B", RelationType::References, "C");
        builder.add_relationship("A", RelationType::References, "C");

        let centrality = builder.calculate_degree_centrality();

        // A: in=0, out=2
        assert_eq!(centrality.get("A"), Some(&(0, 2)));
        // B: in=1, out=1
        assert_eq!(centrality.get("B"), Some(&(1, 1)));
        // C: in=2, out=0
        assert_eq!(centrality.get("C"), Some(&(2, 0)));
    }

    #[test]
    fn test_find_connected_components() {
        let mut builder = KnowledgeGraphBuilder::new("http://example.org/");

        // Create entities
        for id in &["A", "B", "C", "D", "E", "F"] {
            let mut entity = Entity::new(id, EntityType::Statute);
            entity.add_label(&format!("Statute {}", id));
            builder.entities.insert(id.to_string(), entity);
        }

        // Component 1: A-B-C
        builder.add_relationship("A", RelationType::References, "B");
        builder.add_relationship("B", RelationType::References, "C");

        // Component 2: D-E
        builder.add_relationship("D", RelationType::References, "E");

        // Component 3: F (isolated)

        let components = builder.find_connected_components();
        assert_eq!(components.len(), 3);

        // Check component sizes
        let mut sizes: Vec<usize> = components.iter().map(|c| c.len()).collect();
        sizes.sort();
        assert_eq!(sizes, vec![1, 2, 3]);
    }

    #[test]
    fn test_find_most_connected_entities() {
        let mut builder = KnowledgeGraphBuilder::new("http://example.org/");

        for id in &["A", "B", "C", "D"] {
            let mut entity = Entity::new(id, EntityType::Statute);
            entity.add_label(&format!("Statute {}", id));
            builder.entities.insert(id.to_string(), entity);
        }

        builder.add_relationship("A", RelationType::References, "B");
        builder.add_relationship("A", RelationType::References, "C");
        builder.add_relationship("A", RelationType::References, "D");
        builder.add_relationship("B", RelationType::References, "C");

        let most_connected = builder.find_most_connected_entities(2);
        assert_eq!(most_connected.len(), 2);
        // A should be first (3 out-edges)
        assert_eq!(most_connected[0].0, "A");
        assert_eq!(most_connected[0].1, 3);
    }

    #[test]
    fn test_find_most_referenced_entities() {
        let mut builder = KnowledgeGraphBuilder::new("http://example.org/");

        for id in &["A", "B", "C"] {
            let mut entity = Entity::new(id, EntityType::Statute);
            entity.add_label(&format!("Statute {}", id));
            builder.entities.insert(id.to_string(), entity);
        }

        builder.add_relationship("A", RelationType::References, "C");
        builder.add_relationship("B", RelationType::References, "C");

        let most_referenced = builder.find_most_referenced_entities(1);
        assert_eq!(most_referenced.len(), 1);
        // C should be most referenced (2 in-edges)
        assert_eq!(most_referenced[0].0, "C");
        assert_eq!(most_referenced[0].1, 2);
    }
}
