//! Legal Knowledge Graphs (v0.3.1)
//!
//! This module provides automatic knowledge extraction, entity relationship mapping,
//! temporal knowledge evolution, legal concept ontology learning, and knowledge graph reasoning.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Represents an entity in the legal knowledge graph.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Entity {
    /// Unique identifier
    pub id: String,
    /// Entity type (e.g., "statute", "case", "person", "organization")
    pub entity_type: String,
    /// Display name
    pub name: String,
    /// Additional properties
    pub properties: HashMap<String, String>,
}

impl std::hash::Hash for Entity {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Entity {
    /// Creates a new entity.
    pub fn new(
        id: impl Into<String>,
        entity_type: impl Into<String>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            entity_type: entity_type.into(),
            name: name.into(),
            properties: HashMap::new(),
        }
    }

    /// Adds a property to the entity.
    pub fn with_property(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.properties.insert(key.into(), value.into());
        self
    }
}

/// Represents a relationship between two entities.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Relationship {
    /// Unique identifier
    pub id: String,
    /// Source entity ID
    pub from_entity: String,
    /// Target entity ID
    pub to_entity: String,
    /// Relationship type (e.g., "cites", "amends", "related_to")
    pub relation_type: String,
    /// Confidence score (0.0-1.0)
    pub confidence: f64,
    /// Temporal information
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,
    /// Additional properties
    pub properties: HashMap<String, String>,
}

impl Relationship {
    /// Creates a new relationship.
    pub fn new(
        from_entity: impl Into<String>,
        to_entity: impl Into<String>,
        relation_type: impl Into<String>,
    ) -> Self {
        let from = from_entity.into();
        let to = to_entity.into();
        let rel = relation_type.into();
        let id = format!("{}->{}:{}", from, to, rel);

        Self {
            id,
            from_entity: from,
            to_entity: to,
            relation_type: rel,
            confidence: 1.0,
            timestamp: Some(chrono::Utc::now()),
            properties: HashMap::new(),
        }
    }

    /// Sets the confidence score.
    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }

    /// Adds a property to the relationship.
    pub fn with_property(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.properties.insert(key.into(), value.into());
        self
    }
}

/// Automatic knowledge extraction from legal text.
pub struct KnowledgeExtractor {
    entity_patterns: Vec<EntityPattern>,
    relation_patterns: Vec<RelationPattern>,
}

#[derive(Debug, Clone)]
pub struct EntityPattern {
    pub pattern: regex::Regex,
    pub entity_type: String,
}

#[derive(Debug, Clone)]
pub struct RelationPattern {
    pub pattern: regex::Regex,
    pub relation_type: String,
}

impl KnowledgeExtractor {
    /// Creates a new knowledge extractor.
    pub fn new() -> Self {
        Self {
            entity_patterns: Vec::new(),
            relation_patterns: Vec::new(),
        }
    }

    /// Adds an entity pattern for extraction.
    pub fn add_entity_pattern(
        mut self,
        pattern: &str,
        entity_type: impl Into<String>,
    ) -> Result<Self> {
        let regex = regex::Regex::new(pattern)?;
        self.entity_patterns.push(EntityPattern {
            pattern: regex,
            entity_type: entity_type.into(),
        });
        Ok(self)
    }

    /// Adds a relation pattern for extraction.
    pub fn add_relation_pattern(
        mut self,
        pattern: &str,
        relation_type: impl Into<String>,
    ) -> Result<Self> {
        let regex = regex::Regex::new(pattern)?;
        self.relation_patterns.push(RelationPattern {
            pattern: regex,
            relation_type: relation_type.into(),
        });
        Ok(self)
    }

    /// Extracts entities from text.
    pub fn extract_entities(&self, text: &str) -> Vec<Entity> {
        let mut entities = Vec::new();
        let mut id_counter = 0;

        for pattern_def in &self.entity_patterns {
            for capture in pattern_def.pattern.captures_iter(text) {
                if let Some(matched) = capture.get(1) {
                    let entity = Entity::new(
                        format!("entity_{}", id_counter),
                        &pattern_def.entity_type,
                        matched.as_str(),
                    );
                    entities.push(entity);
                    id_counter += 1;
                }
            }
        }

        entities
    }

    /// Extracts relationships from text using entities.
    pub fn extract_relationships(&self, text: &str, _entities: &[Entity]) -> Vec<Relationship> {
        let mut relationships = Vec::new();

        for pattern_def in &self.relation_patterns {
            for capture in pattern_def.pattern.captures_iter(text) {
                if let (Some(from), Some(to)) = (capture.get(1), capture.get(2)) {
                    let relationship =
                        Relationship::new(from.as_str(), to.as_str(), &pattern_def.relation_type);
                    relationships.push(relationship);
                }
            }
        }

        relationships
    }

    /// Extracts both entities and relationships from text.
    pub fn extract_knowledge(&self, text: &str) -> (Vec<Entity>, Vec<Relationship>) {
        let entities = self.extract_entities(text);
        let relationships = self.extract_relationships(text, &entities);
        (entities, relationships)
    }
}

impl Default for KnowledgeExtractor {
    fn default() -> Self {
        Self::new()
    }
}

/// Legal knowledge graph with entity-relationship mapping.
pub struct LegalKnowledgeGraph {
    entities: Arc<RwLock<HashMap<String, Entity>>>,
    relationships: Arc<RwLock<Vec<Relationship>>>,
    entity_index: Arc<RwLock<HashMap<String, Vec<String>>>>, // type -> entity IDs
}

impl LegalKnowledgeGraph {
    /// Creates a new legal knowledge graph.
    pub fn new() -> Self {
        Self {
            entities: Arc::new(RwLock::new(HashMap::new())),
            relationships: Arc::new(RwLock::new(Vec::new())),
            entity_index: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Adds an entity to the graph.
    pub async fn add_entity(&self, entity: Entity) -> Result<()> {
        let entity_id = entity.id.clone();
        let entity_type = entity.entity_type.clone();

        // Add to entities
        {
            let mut entities = self.entities.write().await;
            entities.insert(entity_id.clone(), entity);
        }

        // Update index
        {
            let mut index = self.entity_index.write().await;
            index
                .entry(entity_type)
                .or_insert_with(Vec::new)
                .push(entity_id);
        }

        Ok(())
    }

    /// Adds a relationship to the graph.
    pub async fn add_relationship(&self, relationship: Relationship) -> Result<()> {
        let mut relationships = self.relationships.write().await;
        relationships.push(relationship);
        Ok(())
    }

    /// Gets an entity by ID.
    pub async fn get_entity(&self, id: &str) -> Option<Entity> {
        let entities = self.entities.read().await;
        entities.get(id).cloned()
    }

    /// Gets all entities of a specific type.
    pub async fn get_entities_by_type(&self, entity_type: &str) -> Vec<Entity> {
        let index = self.entity_index.read().await;
        let entities = self.entities.read().await;

        if let Some(ids) = index.get(entity_type) {
            ids.iter()
                .filter_map(|id| entities.get(id).cloned())
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Gets all relationships involving an entity.
    pub async fn get_relationships_for_entity(&self, entity_id: &str) -> Vec<Relationship> {
        let relationships = self.relationships.read().await;

        relationships
            .iter()
            .filter(|r| r.from_entity == entity_id || r.to_entity == entity_id)
            .cloned()
            .collect()
    }

    /// Gets all relationships of a specific type.
    pub async fn get_relationships_by_type(&self, relation_type: &str) -> Vec<Relationship> {
        let relationships = self.relationships.read().await;

        relationships
            .iter()
            .filter(|r| r.relation_type == relation_type)
            .cloned()
            .collect()
    }

    /// Finds connected entities (neighbors) of a given entity.
    pub async fn find_neighbors(&self, entity_id: &str) -> Vec<Entity> {
        let relationships = self.get_relationships_for_entity(entity_id).await;
        let entities = self.entities.read().await;

        let mut neighbors = Vec::new();
        for rel in relationships {
            let neighbor_id = if rel.from_entity == entity_id {
                &rel.to_entity
            } else {
                &rel.from_entity
            };

            if let Some(entity) = entities.get(neighbor_id) {
                neighbors.push(entity.clone());
            }
        }

        neighbors
    }

    /// Gets statistics about the knowledge graph.
    pub async fn get_statistics(&self) -> KnowledgeGraphStats {
        let entities = self.entities.read().await;
        let relationships = self.relationships.read().await;
        let index = self.entity_index.read().await;

        KnowledgeGraphStats {
            total_entities: entities.len(),
            total_relationships: relationships.len(),
            entity_types: index.len(),
            avg_relationships_per_entity: if !entities.is_empty() {
                relationships.len() as f64 / entities.len() as f64
            } else {
                0.0
            },
        }
    }
}

impl Default for LegalKnowledgeGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeGraphStats {
    pub total_entities: usize,
    pub total_relationships: usize,
    pub entity_types: usize,
    pub avg_relationships_per_entity: f64,
}

/// Temporal knowledge evolution tracking.
///
/// This component tracks how legal knowledge changes over time.
pub struct TemporalKnowledgeTracker {
    snapshots: Arc<RwLock<Vec<KnowledgeSnapshot>>>,
    changes: Arc<RwLock<Vec<KnowledgeChange>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeSnapshot {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub entity_count: usize,
    pub relationship_count: usize,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeChange {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub change_type: ChangeType,
    pub entity_id: Option<String>,
    pub relationship_id: Option<String>,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChangeType {
    EntityAdded,
    EntityModified,
    EntityRemoved,
    RelationshipAdded,
    RelationshipModified,
    RelationshipRemoved,
}

impl TemporalKnowledgeTracker {
    /// Creates a new temporal knowledge tracker.
    pub fn new() -> Self {
        Self {
            snapshots: Arc::new(RwLock::new(Vec::new())),
            changes: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Records a snapshot of the knowledge graph.
    pub async fn record_snapshot(&self, snapshot: KnowledgeSnapshot) -> Result<()> {
        let mut snapshots = self.snapshots.write().await;
        snapshots.push(snapshot);
        Ok(())
    }

    /// Records a knowledge change event.
    pub async fn record_change(&self, change: KnowledgeChange) -> Result<()> {
        let mut changes = self.changes.write().await;
        changes.push(change);
        Ok(())
    }

    /// Gets all snapshots within a time range.
    pub async fn get_snapshots_in_range(
        &self,
        start: chrono::DateTime<chrono::Utc>,
        end: chrono::DateTime<chrono::Utc>,
    ) -> Vec<KnowledgeSnapshot> {
        let snapshots = self.snapshots.read().await;

        snapshots
            .iter()
            .filter(|s| s.timestamp >= start && s.timestamp <= end)
            .cloned()
            .collect()
    }

    /// Gets all changes within a time range.
    pub async fn get_changes_in_range(
        &self,
        start: chrono::DateTime<chrono::Utc>,
        end: chrono::DateTime<chrono::Utc>,
    ) -> Vec<KnowledgeChange> {
        let changes = self.changes.read().await;

        changes
            .iter()
            .filter(|c| c.timestamp >= start && c.timestamp <= end)
            .cloned()
            .collect()
    }

    /// Gets the change rate (changes per day).
    pub async fn get_change_rate(&self) -> f64 {
        let changes = self.changes.read().await;

        if changes.is_empty() {
            return 0.0;
        }

        let first = &changes[0].timestamp;
        let last = &changes[changes.len() - 1].timestamp;
        let duration_days = (last.signed_duration_since(*first).num_seconds() as f64) / 86400.0;

        if duration_days > 0.0 {
            changes.len() as f64 / duration_days
        } else {
            0.0
        }
    }
}

impl Default for TemporalKnowledgeTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Legal concept ontology learner.
///
/// This component learns legal concept hierarchies and relationships.
pub struct OntologyLearner {
    concepts: Arc<RwLock<HashMap<String, Concept>>>,
    hierarchy: Arc<RwLock<HashMap<String, Vec<String>>>>, // parent -> children
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Concept {
    pub id: String,
    pub name: String,
    pub definition: String,
    pub parent: Option<String>,
    pub attributes: Vec<String>,
    pub examples: Vec<String>,
}

impl Concept {
    /// Creates a new concept.
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        definition: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            definition: definition.into(),
            parent: None,
            attributes: Vec::new(),
            examples: Vec::new(),
        }
    }

    /// Sets the parent concept.
    pub fn with_parent(mut self, parent: impl Into<String>) -> Self {
        self.parent = Some(parent.into());
        self
    }

    /// Adds an attribute to the concept.
    pub fn add_attribute(&mut self, attribute: impl Into<String>) {
        self.attributes.push(attribute.into());
    }

    /// Adds an example to the concept.
    pub fn add_example(&mut self, example: impl Into<String>) {
        self.examples.push(example.into());
    }
}

impl OntologyLearner {
    /// Creates a new ontology learner.
    pub fn new() -> Self {
        Self {
            concepts: Arc::new(RwLock::new(HashMap::new())),
            hierarchy: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Adds a concept to the ontology.
    pub async fn add_concept(&self, concept: Concept) -> Result<()> {
        let concept_id = concept.id.clone();
        let parent = concept.parent.clone();

        // Add to concepts
        {
            let mut concepts = self.concepts.write().await;
            concepts.insert(concept_id.clone(), concept);
        }

        // Update hierarchy
        if let Some(parent_id) = parent {
            let mut hierarchy = self.hierarchy.write().await;
            hierarchy
                .entry(parent_id)
                .or_insert_with(Vec::new)
                .push(concept_id);
        }

        Ok(())
    }

    /// Gets a concept by ID.
    pub async fn get_concept(&self, id: &str) -> Option<Concept> {
        let concepts = self.concepts.read().await;
        concepts.get(id).cloned()
    }

    /// Gets all children of a concept.
    pub async fn get_children(&self, parent_id: &str) -> Vec<Concept> {
        let hierarchy = self.hierarchy.read().await;
        let concepts = self.concepts.read().await;

        if let Some(child_ids) = hierarchy.get(parent_id) {
            child_ids
                .iter()
                .filter_map(|id| concepts.get(id).cloned())
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Gets all ancestors of a concept (parent, grandparent, etc.).
    pub async fn get_ancestors(&self, concept_id: &str) -> Vec<Concept> {
        let concepts = self.concepts.read().await;
        let mut ancestors = Vec::new();
        let mut current_id = concept_id.to_string();

        while let Some(concept) = concepts.get(&current_id) {
            if let Some(parent_id) = &concept.parent {
                if let Some(parent) = concepts.get(parent_id) {
                    ancestors.push(parent.clone());
                    current_id = parent_id.clone();
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        ancestors
    }

    /// Finds common ancestor of two concepts.
    pub async fn find_common_ancestor(
        &self,
        concept1_id: &str,
        concept2_id: &str,
    ) -> Option<Concept> {
        let ancestors1 = self.get_ancestors(concept1_id).await;
        let ancestors2 = self.get_ancestors(concept2_id).await;

        let set1: HashSet<_> = ancestors1.iter().map(|c| c.id.as_str()).collect();

        for ancestor in &ancestors2 {
            if set1.contains(ancestor.id.as_str()) {
                return Some(ancestor.clone());
            }
        }

        None
    }
}

impl Default for OntologyLearner {
    fn default() -> Self {
        Self::new()
    }
}

/// Knowledge graph reasoning engine.
///
/// This component performs reasoning over the knowledge graph.
pub struct KnowledgeGraphReasoner {
    graph: Arc<LegalKnowledgeGraph>,
}

impl KnowledgeGraphReasoner {
    /// Creates a new knowledge graph reasoner.
    pub fn new(graph: Arc<LegalKnowledgeGraph>) -> Self {
        Self { graph }
    }

    /// Finds the shortest path between two entities.
    pub async fn find_shortest_path(&self, from_id: &str, to_id: &str) -> Option<Vec<String>> {
        // Simple BFS implementation
        let mut queue = std::collections::VecDeque::new();
        let mut visited = HashSet::new();
        let mut parent: HashMap<String, String> = HashMap::new();

        queue.push_back(from_id.to_string());
        visited.insert(from_id.to_string());

        while let Some(current) = queue.pop_front() {
            if current == to_id {
                // Reconstruct path
                let mut path = vec![current.clone()];
                let mut node = current;

                while let Some(p) = parent.get(&node) {
                    path.push(p.clone());
                    node = p.clone();
                }

                path.reverse();
                return Some(path);
            }

            let neighbors = self.graph.find_neighbors(&current).await;
            for neighbor in neighbors {
                if !visited.contains(&neighbor.id) {
                    visited.insert(neighbor.id.clone());
                    parent.insert(neighbor.id.clone(), current.clone());
                    queue.push_back(neighbor.id);
                }
            }
        }

        None
    }

    /// Infers new relationships based on existing patterns.
    pub async fn infer_relationships(&self) -> Vec<Relationship> {
        let mut inferred = Vec::new();

        // Example: If A cites B and B cites C, infer that A is transitively related to C
        let all_entities_map = self.graph.entities.read().await;
        let all_entities: Vec<_> = all_entities_map.values().cloned().collect();
        drop(all_entities_map);

        for entity in &all_entities {
            let relationships = self.graph.get_relationships_for_entity(&entity.id).await;

            for rel in relationships {
                if rel.relation_type == "cites" {
                    let target_id = if rel.from_entity == entity.id {
                        &rel.to_entity
                    } else {
                        &rel.from_entity
                    };

                    let target_rels = self.graph.get_relationships_for_entity(target_id).await;

                    for target_rel in target_rels {
                        if target_rel.relation_type == "cites" {
                            let final_target = if target_rel.from_entity == *target_id {
                                &target_rel.to_entity
                            } else {
                                &target_rel.from_entity
                            };

                            // Create inferred relationship
                            if final_target != &entity.id {
                                let inferred_rel = Relationship::new(
                                    &entity.id,
                                    final_target,
                                    "transitively_cites",
                                )
                                .with_confidence(0.7);

                                inferred.push(inferred_rel);
                            }
                        }
                    }
                }
            }
        }

        inferred
    }

    /// Finds similar entities based on their relationships.
    pub async fn find_similar_entities(&self, entity_id: &str, top_k: usize) -> Vec<(String, f64)> {
        let entity_rels = self.graph.get_relationships_for_entity(entity_id).await;

        let entity_neighbors: HashSet<_> = entity_rels
            .iter()
            .map(|r| {
                if r.from_entity == entity_id {
                    &r.to_entity
                } else {
                    &r.from_entity
                }
            })
            .collect();

        let all_entities_map = self.graph.entities.read().await;
        let all_entities: Vec<_> = all_entities_map.keys().cloned().collect();
        drop(all_entities_map);

        let mut similarities = Vec::new();

        for other_id in all_entities {
            if other_id == entity_id {
                continue;
            }

            let other_rels = self.graph.get_relationships_for_entity(&other_id).await;
            let other_neighbors: HashSet<_> = other_rels
                .iter()
                .map(|r| {
                    if r.from_entity == other_id {
                        &r.to_entity
                    } else {
                        &r.from_entity
                    }
                })
                .collect();

            // Jaccard similarity
            let intersection = entity_neighbors.intersection(&other_neighbors).count();
            let union = entity_neighbors.union(&other_neighbors).count();

            if union > 0 {
                let similarity = intersection as f64 / union as f64;
                similarities.push((other_id, similarity));
            }
        }

        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        similarities.into_iter().take(top_k).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entity_creation() {
        let entity =
            Entity::new("e1", "statute", "Section 123").with_property("jurisdiction", "federal");

        assert_eq!(entity.id, "e1");
        assert_eq!(entity.entity_type, "statute");
        assert_eq!(entity.name, "Section 123");
        assert_eq!(
            entity.properties.get("jurisdiction"),
            Some(&"federal".to_string())
        );
    }

    #[test]
    fn test_relationship_creation() {
        let rel = Relationship::new("e1", "e2", "cites").with_confidence(0.95);

        assert_eq!(rel.from_entity, "e1");
        assert_eq!(rel.to_entity, "e2");
        assert_eq!(rel.relation_type, "cites");
        assert!((rel.confidence - 0.95).abs() < f64::EPSILON);
    }

    #[tokio::test]
    async fn test_knowledge_graph() {
        let graph = LegalKnowledgeGraph::new();

        let entity1 = Entity::new("e1", "statute", "Section 1");
        let entity2 = Entity::new("e2", "statute", "Section 2");

        graph.add_entity(entity1).await.unwrap();
        graph.add_entity(entity2).await.unwrap();

        let rel = Relationship::new("e1", "e2", "cites");
        graph.add_relationship(rel).await.unwrap();

        let stats = graph.get_statistics().await;
        assert_eq!(stats.total_entities, 2);
        assert_eq!(stats.total_relationships, 1);
    }

    #[tokio::test]
    async fn test_knowledge_extractor() {
        let extractor = KnowledgeExtractor::new()
            .add_entity_pattern(r"Section (\d+)", "statute")
            .unwrap();

        let text = "Section 123 applies to Section 456.";
        let entities = extractor.extract_entities(text);

        assert_eq!(entities.len(), 2);
        assert_eq!(entities[0].name, "123");
        assert_eq!(entities[1].name, "456");
    }

    #[tokio::test]
    async fn test_temporal_tracker() {
        let tracker = TemporalKnowledgeTracker::new();

        let snapshot = KnowledgeSnapshot {
            timestamp: chrono::Utc::now(),
            entity_count: 10,
            relationship_count: 20,
            metadata: HashMap::new(),
        };

        tracker.record_snapshot(snapshot).await.unwrap();

        let change = KnowledgeChange {
            timestamp: chrono::Utc::now(),
            change_type: ChangeType::EntityAdded,
            entity_id: Some("e1".to_string()),
            relationship_id: None,
            description: "Added new entity".to_string(),
        };

        tracker.record_change(change).await.unwrap();
    }

    #[tokio::test]
    async fn test_ontology_learner() {
        let learner = OntologyLearner::new();

        let mut contract = Concept::new("contract", "Contract", "A legal agreement");
        contract.add_attribute("parties");
        contract.add_attribute("terms");

        learner.add_concept(contract).await.unwrap();

        let sales_contract = Concept::new(
            "sales_contract",
            "Sales Contract",
            "A contract for sale of goods",
        )
        .with_parent("contract");

        learner.add_concept(sales_contract).await.unwrap();

        let children = learner.get_children("contract").await;
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].id, "sales_contract");

        let ancestors = learner.get_ancestors("sales_contract").await;
        assert_eq!(ancestors.len(), 1);
        assert_eq!(ancestors[0].id, "contract");
    }

    #[tokio::test]
    async fn test_knowledge_graph_reasoner() {
        let graph = Arc::new(LegalKnowledgeGraph::new());

        let e1 = Entity::new("e1", "statute", "S1");
        let e2 = Entity::new("e2", "statute", "S2");
        let e3 = Entity::new("e3", "statute", "S3");

        graph.add_entity(e1).await.unwrap();
        graph.add_entity(e2).await.unwrap();
        graph.add_entity(e3).await.unwrap();

        graph
            .add_relationship(Relationship::new("e1", "e2", "cites"))
            .await
            .unwrap();
        graph
            .add_relationship(Relationship::new("e2", "e3", "cites"))
            .await
            .unwrap();

        let reasoner = KnowledgeGraphReasoner::new(graph);

        let path = reasoner.find_shortest_path("e1", "e3").await;
        assert!(path.is_some());
        assert_eq!(path.unwrap().len(), 3);
    }
}
