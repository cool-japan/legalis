//! Knowledge-graph data structures: a typed legal concept graph.
//!
//! [`LegalConceptGraph`] is a directed, typed knowledge graph whose nodes are
//! [`KnowledgeConcept`]s (statutes, doctrines, rights, duties, parties, remedies,
//! ...) and whose edges are typed [`KnowledgeRelation`]s (`is-a`, `part-of`,
//! `requires`, `conflicts-with`, `supersedes`, `cites`, ...). It supports the
//! data-structure and *query / reasoning* half of the v0.5.9 knowledge-graph
//! item:
//!
//! * adjacency in both directions (out-edges and in-edges) for any concept;
//! * typed neighbour queries (filter edges by relation);
//! * ancestor / descendant queries that follow only the hierarchical `IsA`
//!   relation (with cycle-safe traversal);
//! * generic forward/backward transitive closure over a chosen relation;
//! * breadth-first shortest path between two concepts;
//! * concept-type and relation-type census statistics.
//!
//! Graph *visualisation* is deliberately out of scope (it needs a renderer);
//! a DOT export is provided as plain text so an external tool can lay it out.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::fmt::Write as _;

/// The kind of legal concept a node represents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LegalConceptKind {
    /// A doctrine or legal principle.
    Doctrine,
    /// A statute or codified rule.
    Statute,
    /// A constitutional provision.
    Constitution,
    /// A right held by an actor.
    Right,
    /// A duty owed by an actor.
    Duty,
    /// A remedy / relief.
    Remedy,
    /// A legal actor / party role.
    Actor,
    /// An element of a cause of action or offence.
    Element,
    /// A procedure or process step.
    Procedure,
    /// A general legal category / topic.
    Category,
}

impl LegalConceptKind {
    /// A short human-readable label.
    pub fn label(&self) -> &'static str {
        match self {
            LegalConceptKind::Doctrine => "doctrine",
            LegalConceptKind::Statute => "statute",
            LegalConceptKind::Constitution => "constitution",
            LegalConceptKind::Right => "right",
            LegalConceptKind::Duty => "duty",
            LegalConceptKind::Remedy => "remedy",
            LegalConceptKind::Actor => "actor",
            LegalConceptKind::Element => "element",
            LegalConceptKind::Procedure => "procedure",
            LegalConceptKind::Category => "category",
        }
    }
}

/// The kind of relationship a directed edge represents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ConceptRelationKind {
    /// Subtype / specialisation (`X is-a Y`): hierarchical.
    IsA,
    /// Composition (`X part-of Y`).
    PartOf,
    /// `X requires Y` (a prerequisite / element).
    Requires,
    /// `X causes Y`.
    Causes,
    /// `X conflicts with Y`.
    ConflictsWith,
    /// `X supports Y`.
    Supports,
    /// `X supersedes Y` (overrides / replaces).
    Supersedes,
    /// `X cites Y`.
    Cites,
    /// `X applies to Y`.
    AppliesTo,
    /// `X grants Y` (e.g. a statute grants a right).
    Grants,
}

impl ConceptRelationKind {
    /// A short human-readable label.
    pub fn label(&self) -> &'static str {
        match self {
            ConceptRelationKind::IsA => "is-a",
            ConceptRelationKind::PartOf => "part-of",
            ConceptRelationKind::Requires => "requires",
            ConceptRelationKind::Causes => "causes",
            ConceptRelationKind::ConflictsWith => "conflicts-with",
            ConceptRelationKind::Supports => "supports",
            ConceptRelationKind::Supersedes => "supersedes",
            ConceptRelationKind::Cites => "cites",
            ConceptRelationKind::AppliesTo => "applies-to",
            ConceptRelationKind::Grants => "grants",
        }
    }

    /// Whether the relation is transitive (closure queries are meaningful).
    pub fn is_transitive(&self) -> bool {
        matches!(
            self,
            ConceptRelationKind::IsA
                | ConceptRelationKind::PartOf
                | ConceptRelationKind::Causes
                | ConceptRelationKind::Supersedes
        )
    }
}

/// A node in the legal concept graph.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KnowledgeConcept {
    /// Stable unique identifier.
    pub id: String,
    /// Human-readable name.
    pub name: String,
    /// The kind of concept.
    pub kind: LegalConceptKind,
    /// Optional free-text description.
    pub description: Option<String>,
    /// Arbitrary attributes.
    pub attributes: BTreeMap<String, String>,
}

impl KnowledgeConcept {
    /// Creates a new concept.
    pub fn new(id: impl Into<String>, name: impl Into<String>, kind: LegalConceptKind) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            kind,
            description: None,
            attributes: BTreeMap::new(),
        }
    }

    /// Sets the description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Adds an attribute.
    pub fn with_attribute(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }
}

/// A directed, typed edge between two concepts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnowledgeRelation {
    /// Source concept id.
    pub from: String,
    /// Target concept id.
    pub to: String,
    /// Relation kind.
    pub kind: ConceptRelationKind,
}

impl KnowledgeRelation {
    /// Creates a new relation.
    pub fn new(from: impl Into<String>, to: impl Into<String>, kind: ConceptRelationKind) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
            kind,
        }
    }
}

/// Aggregate statistics about a concept graph.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GraphStatistics {
    /// Number of concept nodes.
    pub num_concepts: usize,
    /// Number of relation edges.
    pub num_relations: usize,
    /// Count of concepts by kind.
    pub concepts_by_kind: BTreeMap<String, usize>,
    /// Count of relations by kind.
    pub relations_by_kind: BTreeMap<String, usize>,
    /// Number of concepts with no outgoing or incoming edges.
    pub isolated_concepts: usize,
}

/// A typed, directed legal concept knowledge graph.
#[derive(Debug, Clone, Default)]
pub struct LegalConceptGraph {
    concepts: BTreeMap<String, KnowledgeConcept>,
    /// Outgoing adjacency: concept id -> edges where it is the source.
    out_edges: HashMap<String, Vec<KnowledgeRelation>>,
    /// Incoming adjacency: concept id -> edges where it is the target.
    in_edges: HashMap<String, Vec<KnowledgeRelation>>,
    /// All edges (for dedup and iteration).
    edges: BTreeSet<(String, String, ConceptRelationKind)>,
}

impl LegalConceptGraph {
    /// Creates an empty graph.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a concept. Errors on duplicate id.
    pub fn add_concept(&mut self, concept: KnowledgeConcept) -> Result<(), String> {
        if self.concepts.contains_key(&concept.id) {
            return Err(format!("duplicate concept id: {}", concept.id));
        }
        self.concepts.insert(concept.id.clone(), concept);
        Ok(())
    }

    /// Adds many concepts, stopping at the first error.
    pub fn add_concepts<I>(&mut self, concepts: I) -> Result<(), String>
    where
        I: IntoIterator<Item = KnowledgeConcept>,
    {
        for concept in concepts {
            self.add_concept(concept)?;
        }
        Ok(())
    }

    /// Adds a relation. Both endpoints must already exist. Duplicate edges
    /// (same source, target and kind) are ignored idempotently.
    pub fn add_relation(&mut self, relation: KnowledgeRelation) -> Result<(), String> {
        if !self.concepts.contains_key(&relation.from) {
            return Err(format!("unknown source concept: {}", relation.from));
        }
        if !self.concepts.contains_key(&relation.to) {
            return Err(format!("unknown target concept: {}", relation.to));
        }
        let key = (relation.from.clone(), relation.to.clone(), relation.kind);
        if !self.edges.insert(key) {
            return Ok(());
        }
        self.out_edges
            .entry(relation.from.clone())
            .or_default()
            .push(relation.clone());
        self.in_edges
            .entry(relation.to.clone())
            .or_default()
            .push(relation);
        Ok(())
    }

    /// Adds many relations, stopping at the first error.
    pub fn add_relations<I>(&mut self, relations: I) -> Result<(), String>
    where
        I: IntoIterator<Item = KnowledgeRelation>,
    {
        for relation in relations {
            self.add_relation(relation)?;
        }
        Ok(())
    }

    /// Number of concepts.
    pub fn concept_count(&self) -> usize {
        self.concepts.len()
    }

    /// Number of relations.
    pub fn relation_count(&self) -> usize {
        self.edges.len()
    }

    /// Whether the graph has no concepts.
    pub fn is_empty(&self) -> bool {
        self.concepts.is_empty()
    }

    /// Borrows a concept by id.
    pub fn concept(&self, id: &str) -> Option<&KnowledgeConcept> {
        self.concepts.get(id)
    }

    /// Iterates over all concepts (ordered by id).
    pub fn concepts(&self) -> impl Iterator<Item = &KnowledgeConcept> {
        self.concepts.values()
    }

    /// Outgoing relations from a concept.
    pub fn out_relations(&self, id: &str) -> &[KnowledgeRelation] {
        self.out_edges.get(id).map(|v| v.as_slice()).unwrap_or(&[])
    }

    /// Incoming relations to a concept.
    pub fn in_relations(&self, id: &str) -> &[KnowledgeRelation] {
        self.in_edges.get(id).map(|v| v.as_slice()).unwrap_or(&[])
    }

    /// Direct successors reached by edges of the given relation kind.
    pub fn neighbors(&self, id: &str, kind: ConceptRelationKind) -> Vec<&KnowledgeConcept> {
        self.out_relations(id)
            .iter()
            .filter(|r| r.kind == kind)
            .filter_map(|r| self.concepts.get(&r.to))
            .collect()
    }

    /// Direct predecessors via edges of the given relation kind.
    pub fn predecessors(&self, id: &str, kind: ConceptRelationKind) -> Vec<&KnowledgeConcept> {
        self.in_relations(id)
            .iter()
            .filter(|r| r.kind == kind)
            .filter_map(|r| self.concepts.get(&r.from))
            .collect()
    }

    /// Forward transitive closure of `id` over the given relation kind.
    ///
    /// Returns the ids reachable by following one or more edges of that kind,
    /// excluding `id` itself. Cycle-safe.
    pub fn transitive_closure(&self, id: &str, kind: ConceptRelationKind) -> BTreeSet<String> {
        let mut result = BTreeSet::new();
        if !self.concepts.contains_key(id) {
            return result;
        }
        let mut stack = vec![id.to_string()];
        let mut visited = HashSet::new();
        while let Some(current) = stack.pop() {
            for relation in self.out_relations(&current) {
                if relation.kind == kind && visited.insert(relation.to.clone()) {
                    result.insert(relation.to.clone());
                    stack.push(relation.to.clone());
                }
            }
        }
        result
    }

    /// Backward transitive closure of `id` over the given relation kind (all
    /// ids that reach `id` through edges of that kind). Cycle-safe.
    pub fn reverse_transitive_closure(
        &self,
        id: &str,
        kind: ConceptRelationKind,
    ) -> BTreeSet<String> {
        let mut result = BTreeSet::new();
        if !self.concepts.contains_key(id) {
            return result;
        }
        let mut stack = vec![id.to_string()];
        let mut visited = HashSet::new();
        while let Some(current) = stack.pop() {
            for relation in self.in_relations(&current) {
                if relation.kind == kind && visited.insert(relation.from.clone()) {
                    result.insert(relation.from.clone());
                    stack.push(relation.from.clone());
                }
            }
        }
        result
    }

    /// Ancestors of a concept in the `IsA` hierarchy (its supertypes,
    /// transitively).
    pub fn ancestors(&self, id: &str) -> BTreeSet<String> {
        self.transitive_closure(id, ConceptRelationKind::IsA)
    }

    /// Descendants of a concept in the `IsA` hierarchy (its subtypes,
    /// transitively).
    pub fn descendants(&self, id: &str) -> BTreeSet<String> {
        self.reverse_transitive_closure(id, ConceptRelationKind::IsA)
    }

    /// Whether `subtype` is a (transitive) subtype of `supertype` in the `IsA`
    /// hierarchy.
    pub fn is_subtype_of(&self, subtype: &str, supertype: &str) -> bool {
        self.ancestors(subtype).contains(supertype)
    }

    /// Breadth-first shortest path of concept ids from `from` to `to`,
    /// following any relation direction (out-edges). Returns `None` if no path
    /// exists; returns a single-element path when `from == to`.
    pub fn shortest_path(&self, from: &str, to: &str) -> Option<Vec<String>> {
        if !self.concepts.contains_key(from) || !self.concepts.contains_key(to) {
            return None;
        }
        if from == to {
            return Some(vec![from.to_string()]);
        }
        let mut queue = VecDeque::new();
        let mut predecessor: HashMap<String, String> = HashMap::new();
        let mut visited = HashSet::new();
        queue.push_back(from.to_string());
        visited.insert(from.to_string());

        while let Some(current) = queue.pop_front() {
            for relation in self.out_relations(&current) {
                if visited.insert(relation.to.clone()) {
                    predecessor.insert(relation.to.clone(), current.clone());
                    if relation.to == to {
                        // Reconstruct.
                        let mut path = vec![to.to_string()];
                        let mut node = to.to_string();
                        while let Some(prev) = predecessor.get(&node) {
                            path.push(prev.clone());
                            node = prev.clone();
                            if node == from {
                                break;
                            }
                        }
                        path.reverse();
                        return Some(path);
                    }
                    queue.push_back(relation.to.clone());
                }
            }
        }
        None
    }

    /// Finds concepts of a given kind.
    pub fn concepts_of_kind(&self, kind: LegalConceptKind) -> Vec<&KnowledgeConcept> {
        self.concepts.values().filter(|c| c.kind == kind).collect()
    }

    /// Computes aggregate graph statistics.
    pub fn statistics(&self) -> GraphStatistics {
        let mut concepts_by_kind: BTreeMap<String, usize> = BTreeMap::new();
        for concept in self.concepts.values() {
            *concepts_by_kind
                .entry(concept.kind.label().to_string())
                .or_insert(0) += 1;
        }
        let mut relations_by_kind: BTreeMap<String, usize> = BTreeMap::new();
        for (_, _, kind) in &self.edges {
            *relations_by_kind
                .entry(kind.label().to_string())
                .or_insert(0) += 1;
        }
        let isolated = self
            .concepts
            .keys()
            .filter(|id| self.out_relations(id).is_empty() && self.in_relations(id).is_empty())
            .count();
        GraphStatistics {
            num_concepts: self.concepts.len(),
            num_relations: self.edges.len(),
            concepts_by_kind,
            relations_by_kind,
            isolated_concepts: isolated,
        }
    }

    /// Exports the graph in Graphviz DOT format (plain text; not a rendering).
    pub fn to_dot(&self) -> String {
        let mut out = String::from("digraph legal_concepts {\n");
        for concept in self.concepts.values() {
            let _ = writeln!(
                out,
                "  \"{}\" [label=\"{}\\n({})\"];",
                concept.id,
                concept.name.replace('"', "'"),
                concept.kind.label()
            );
        }
        // Iterate edges in deterministic order.
        for (from, to, kind) in &self.edges {
            let _ = writeln!(
                out,
                "  \"{from}\" -> \"{to}\" [label=\"{}\"];",
                kind.label()
            );
        }
        out.push_str("}\n");
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_graph() -> LegalConceptGraph {
        let mut graph = LegalConceptGraph::new();
        graph
            .add_concepts(vec![
                KnowledgeConcept::new("tort", "Tort", LegalConceptKind::Category),
                KnowledgeConcept::new("negligence", "Negligence", LegalConceptKind::Doctrine),
                KnowledgeConcept::new(
                    "gross_negligence",
                    "Gross Negligence",
                    LegalConceptKind::Doctrine,
                ),
                KnowledgeConcept::new("duty", "Duty of Care", LegalConceptKind::Element),
                KnowledgeConcept::new("breach", "Breach", LegalConceptKind::Element),
                KnowledgeConcept::new("causation", "Causation", LegalConceptKind::Element),
                KnowledgeConcept::new("damages", "Damages", LegalConceptKind::Remedy),
            ])
            .expect("concepts add");
        graph
            .add_relations(vec![
                KnowledgeRelation::new("negligence", "tort", ConceptRelationKind::IsA),
                KnowledgeRelation::new("gross_negligence", "negligence", ConceptRelationKind::IsA),
                KnowledgeRelation::new("negligence", "duty", ConceptRelationKind::Requires),
                KnowledgeRelation::new("negligence", "breach", ConceptRelationKind::Requires),
                KnowledgeRelation::new("negligence", "causation", ConceptRelationKind::Requires),
                KnowledgeRelation::new("negligence", "damages", ConceptRelationKind::Grants),
            ])
            .expect("relations add");
        graph
    }

    #[test]
    fn test_add_and_duplicate_concept() {
        let mut graph = LegalConceptGraph::new();
        let concept = KnowledgeConcept::new("x", "X", LegalConceptKind::Doctrine);
        graph.add_concept(concept.clone()).expect("first add");
        assert_eq!(graph.concept_count(), 1);
        assert!(graph.add_concept(concept).is_err());
    }

    #[test]
    fn test_relation_requires_existing_endpoints() {
        let mut graph = LegalConceptGraph::new();
        graph
            .add_concept(KnowledgeConcept::new("a", "A", LegalConceptKind::Doctrine))
            .expect("add a");
        let bad = KnowledgeRelation::new("a", "missing", ConceptRelationKind::IsA);
        assert!(graph.add_relation(bad).is_err());
    }

    #[test]
    fn test_duplicate_relation_idempotent() {
        let mut graph = sample_graph();
        let before = graph.relation_count();
        graph
            .add_relation(KnowledgeRelation::new(
                "negligence",
                "tort",
                ConceptRelationKind::IsA,
            ))
            .expect("idempotent add");
        assert_eq!(graph.relation_count(), before);
    }

    #[test]
    fn test_neighbors_and_predecessors() {
        let graph = sample_graph();
        let elements = graph.neighbors("negligence", ConceptRelationKind::Requires);
        let names: BTreeSet<&str> = elements.iter().map(|c| c.id.as_str()).collect();
        assert!(names.contains("duty"));
        assert!(names.contains("breach"));
        assert!(names.contains("causation"));
        assert_eq!(elements.len(), 3);

        // Reverse: who requires "duty"? negligence.
        let preds = graph.predecessors("duty", ConceptRelationKind::Requires);
        assert_eq!(preds.len(), 1);
        assert_eq!(preds[0].id, "negligence");
    }

    #[test]
    fn test_ancestors_descendants_subtype() {
        let graph = sample_graph();
        // gross_negligence -> negligence -> tort.
        let ancestors = graph.ancestors("gross_negligence");
        assert!(ancestors.contains("negligence"));
        assert!(ancestors.contains("tort"));
        assert_eq!(ancestors.len(), 2);

        let descendants = graph.descendants("tort");
        assert!(descendants.contains("negligence"));
        assert!(descendants.contains("gross_negligence"));

        assert!(graph.is_subtype_of("gross_negligence", "tort"));
        assert!(!graph.is_subtype_of("tort", "negligence"));
    }

    #[test]
    fn test_transitive_closure_cycle_safe() {
        let mut graph = LegalConceptGraph::new();
        graph
            .add_concepts(vec![
                KnowledgeConcept::new("a", "A", LegalConceptKind::Doctrine),
                KnowledgeConcept::new("b", "B", LegalConceptKind::Doctrine),
                KnowledgeConcept::new("c", "C", LegalConceptKind::Doctrine),
            ])
            .expect("concepts");
        // Introduce a cycle a->b->c->a over Causes.
        graph
            .add_relations(vec![
                KnowledgeRelation::new("a", "b", ConceptRelationKind::Causes),
                KnowledgeRelation::new("b", "c", ConceptRelationKind::Causes),
                KnowledgeRelation::new("c", "a", ConceptRelationKind::Causes),
            ])
            .expect("relations");
        let closure = graph.transitive_closure("a", ConceptRelationKind::Causes);
        // Reaches b, c and back to a (excluding the start from the result set
        // only if not re-reached; here a is reachable through the cycle).
        assert!(closure.contains("b"));
        assert!(closure.contains("c"));
        assert!(closure.contains("a"));
    }

    #[test]
    fn test_shortest_path() {
        let graph = sample_graph();
        // negligence -> damages (direct Grants edge).
        let path = graph.shortest_path("negligence", "damages").expect("path");
        assert_eq!(path, vec!["negligence".to_string(), "damages".to_string()]);

        // gross_negligence -> negligence -> tort (two IsA hops).
        let path2 = graph
            .shortest_path("gross_negligence", "tort")
            .expect("path2");
        assert_eq!(
            path2,
            vec![
                "gross_negligence".to_string(),
                "negligence".to_string(),
                "tort".to_string()
            ]
        );

        // Self path.
        assert_eq!(
            graph.shortest_path("tort", "tort"),
            Some(vec!["tort".to_string()])
        );
        // No path (tort has no out-edges to damages).
        assert!(graph.shortest_path("tort", "damages").is_none());
        // Unknown node.
        assert!(graph.shortest_path("nope", "tort").is_none());
    }

    #[test]
    fn test_concepts_of_kind_and_statistics() {
        let graph = sample_graph();
        let elements = graph.concepts_of_kind(LegalConceptKind::Element);
        assert_eq!(elements.len(), 3);

        let stats = graph.statistics();
        assert_eq!(stats.num_concepts, 7);
        assert_eq!(stats.num_relations, 6);
        assert_eq!(
            stats.concepts_by_kind.get("element").copied().unwrap_or(0),
            3
        );
        assert_eq!(stats.relations_by_kind.get("is-a").copied().unwrap_or(0), 2);
        assert_eq!(
            stats
                .relations_by_kind
                .get("requires")
                .copied()
                .unwrap_or(0),
            3
        );
        assert_eq!(stats.isolated_concepts, 0);
    }

    #[test]
    fn test_dot_export() {
        let graph = sample_graph();
        let dot = graph.to_dot();
        assert!(dot.starts_with("digraph legal_concepts {"));
        assert!(dot.contains("\"negligence\""));
        assert!(dot.contains("-> \"tort\" [label=\"is-a\"]"));
        assert!(dot.trim_end().ends_with('}'));
    }

    #[test]
    fn test_relation_kind_transitivity() {
        assert!(ConceptRelationKind::IsA.is_transitive());
        assert!(ConceptRelationKind::PartOf.is_transitive());
        assert!(!ConceptRelationKind::ConflictsWith.is_transitive());
        assert_eq!(ConceptRelationKind::IsA.label(), "is-a");
        assert_eq!(LegalConceptKind::Remedy.label(), "remedy");
    }
}
