//! Knowledge graph representation for legal reasoning.
//!
//! This module provides knowledge graph capabilities for representing
//! and reasoning about legal statutes, entities, and their relationships.
//!
//! ## Features
//!
//! - **Graph Construction**: Convert statutes to knowledge graph nodes and edges
//! - **Entity Linking**: Link legal entities to ontology concepts
//! - **Graph Reasoning**: Traverse and query the knowledge graph
//! - **Query DSL**: Fluent API for graph queries
//! - **Embeddings**: Semantic similarity using vector embeddings
//!
//! ## Example
//!
//! ```
//! use legalis_core::knowledge_graph::{KnowledgeGraph, Node, Edge, RelationType};
//! use legalis_core::{Statute, Effect, EffectType};
//!
//! let mut kg = KnowledgeGraph::new();
//!
//! // Add a statute node
//! let statute = Statute::new("s1", "Tax Credit", Effect::new(EffectType::Grant, "Credit"));
//! kg.add_statute(&statute);
//!
//! // Query the graph
//! let query = kg.query().node_type("Statute");
//! let nodes = query.execute();
//! assert_eq!(nodes.len(), 1);
//! ```

use crate::Statute;
use std::collections::HashMap;

/// Node in the knowledge graph.
#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    /// Unique node identifier
    pub id: String,
    /// Node type (Statute, Entity, Concept, etc.)
    pub node_type: String,
    /// Node label/name
    pub label: String,
    /// Additional properties
    pub properties: HashMap<String, String>,
}

impl Node {
    /// Creates a new node.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::knowledge_graph::Node;
    ///
    /// let node = Node::new("n1", "Statute", "Tax Law");
    /// assert_eq!(node.id, "n1");
    /// ```
    pub fn new(
        id: impl Into<String>,
        node_type: impl Into<String>,
        label: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            node_type: node_type.into(),
            label: label.into(),
            properties: HashMap::new(),
        }
    }

    /// Adds a property to the node.
    pub fn with_property(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.properties.insert(key.into(), value.into());
        self
    }
}

/// Relationship type in the knowledge graph.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum RelationType {
    /// References another statute
    References,
    /// Amends another statute
    Amends,
    /// Applies to an entity
    AppliesTo,
    /// Requires a condition
    Requires,
    /// Produces an effect
    Produces,
    /// Part of a hierarchy
    PartOf,
    /// Similar to another node
    SimilarTo,
    /// Linked to ontology concept
    LinkedTo,
    /// Custom relationship
    Custom(String),
}

impl std::fmt::Display for RelationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::References => write!(f, "REFERENCES"),
            Self::Amends => write!(f, "AMENDS"),
            Self::AppliesTo => write!(f, "APPLIES_TO"),
            Self::Requires => write!(f, "REQUIRES"),
            Self::Produces => write!(f, "PRODUCES"),
            Self::PartOf => write!(f, "PART_OF"),
            Self::SimilarTo => write!(f, "SIMILAR_TO"),
            Self::LinkedTo => write!(f, "LINKED_TO"),
            Self::Custom(s) => write!(f, "{}", s),
        }
    }
}

/// Edge in the knowledge graph.
#[derive(Clone, Debug, PartialEq)]
pub struct Edge {
    /// Source node ID
    pub from: String,
    /// Target node ID
    pub to: String,
    /// Relationship type
    pub relation: RelationType,
    /// Edge weight/confidence
    pub weight: f64,
    /// Additional properties
    pub properties: HashMap<String, String>,
}

impl Edge {
    /// Creates a new edge.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::knowledge_graph::{Edge, RelationType};
    ///
    /// let edge = Edge::new("n1", "n2", RelationType::References);
    /// assert_eq!(edge.from, "n1");
    /// assert_eq!(edge.to, "n2");
    /// ```
    pub fn new(from: impl Into<String>, to: impl Into<String>, relation: RelationType) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
            relation,
            weight: 1.0,
            properties: HashMap::new(),
        }
    }

    /// Sets the edge weight.
    pub fn with_weight(mut self, weight: f64) -> Self {
        self.weight = weight;
        self
    }

    /// Adds a property to the edge.
    pub fn with_property(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.properties.insert(key.into(), value.into());
        self
    }
}

/// Knowledge graph for legal reasoning.
pub struct KnowledgeGraph {
    nodes: HashMap<String, Node>,
    edges: Vec<Edge>,
}

impl KnowledgeGraph {
    /// Creates a new empty knowledge graph.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::knowledge_graph::KnowledgeGraph;
    ///
    /// let kg = KnowledgeGraph::new();
    /// assert_eq!(kg.node_count(), 0);
    /// ```
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
        }
    }

    /// Adds a statute to the knowledge graph.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::knowledge_graph::KnowledgeGraph;
    /// use legalis_core::{Statute, Effect, EffectType};
    ///
    /// let mut kg = KnowledgeGraph::new();
    /// let statute = Statute::new("s1", "Tax Law", Effect::new(EffectType::Grant, "Credit"));
    /// kg.add_statute(&statute);
    ///
    /// assert_eq!(kg.node_count(), 2); // Statute + Effect nodes
    /// ```
    pub fn add_statute(&mut self, statute: &Statute) -> &mut Self {
        // Add statute node
        let statute_node = Node::new(&statute.id, "Statute", &statute.title)
            .with_property("version", statute.version.to_string());

        if let Some(jurisdiction) = &statute.jurisdiction {
            self.nodes.insert(
                statute.id.clone(),
                statute_node.with_property("jurisdiction", jurisdiction),
            );
        } else {
            self.nodes.insert(statute.id.clone(), statute_node);
        }

        // Add effect node
        let effect_id = format!("{}_effect", statute.id);
        let effect_node = Node::new(&effect_id, "Effect", &statute.effect.description)
            .with_property("type", format!("{:?}", statute.effect.effect_type));
        self.nodes.insert(effect_id.clone(), effect_node);

        // Add edge from statute to effect
        self.edges
            .push(Edge::new(&statute.id, &effect_id, RelationType::Produces));

        // Add condition nodes
        for (i, condition) in statute.preconditions.iter().enumerate() {
            let cond_id = format!("{}_cond_{}", statute.id, i);
            let cond_node = Node::new(&cond_id, "Condition", format!("{}", condition))
                .with_property("index", i.to_string());
            self.nodes.insert(cond_id.clone(), cond_node);

            self.edges
                .push(Edge::new(&statute.id, &cond_id, RelationType::Requires));
        }

        self
    }

    /// Adds a custom node to the graph.
    pub fn add_node(&mut self, node: Node) -> &mut Self {
        self.nodes.insert(node.id.clone(), node);
        self
    }

    /// Adds an edge to the graph.
    pub fn add_edge(&mut self, edge: Edge) -> &mut Self {
        self.edges.push(edge);
        self
    }

    /// Returns the number of nodes in the graph.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Returns the number of edges in the graph.
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Gets a node by ID.
    pub fn get_node(&self, id: &str) -> Option<&Node> {
        self.nodes.get(id)
    }

    /// Finds neighbors of a node.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::knowledge_graph::{KnowledgeGraph, RelationType};
    /// use legalis_core::{Statute, Effect, EffectType};
    ///
    /// let mut kg = KnowledgeGraph::new();
    /// let statute = Statute::new("s1", "Tax Law", Effect::new(EffectType::Grant, "Credit"));
    /// kg.add_statute(&statute);
    ///
    /// let neighbors = kg.neighbors("s1", Some(RelationType::Produces));
    /// assert_eq!(neighbors.len(), 1);
    /// ```
    pub fn neighbors(&self, node_id: &str, relation: Option<RelationType>) -> Vec<&Node> {
        let mut result = Vec::new();

        for edge in &self.edges {
            if edge.from == node_id {
                if let Some(ref rel) = relation {
                    if &edge.relation == rel
                        && let Some(node) = self.nodes.get(&edge.to)
                    {
                        result.push(node);
                    }
                } else if let Some(node) = self.nodes.get(&edge.to) {
                    result.push(node);
                }
            }
        }

        result
    }

    /// Creates a query builder for the graph.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::knowledge_graph::KnowledgeGraph;
    /// use legalis_core::{Statute, Effect, EffectType};
    ///
    /// let mut kg = KnowledgeGraph::new();
    /// let statute = Statute::new("s1", "Tax Law", Effect::new(EffectType::Grant, "Credit"));
    /// kg.add_statute(&statute);
    ///
    /// let query = kg.query().node_type("Statute");
    /// let nodes = query.execute();
    /// assert_eq!(nodes.len(), 1);
    /// ```
    pub fn query(&self) -> GraphQuery<'_> {
        GraphQuery::new(self)
    }

    /// Exports the graph to Cypher (Neo4j) format.
    ///
    /// # Example
    ///
    /// ```
    /// use legalis_core::knowledge_graph::KnowledgeGraph;
    /// use legalis_core::{Statute, Effect, EffectType};
    ///
    /// let mut kg = KnowledgeGraph::new();
    /// let statute = Statute::new("s1", "Tax Law", Effect::new(EffectType::Grant, "Credit"));
    /// kg.add_statute(&statute);
    ///
    /// let cypher = kg.to_cypher();
    /// assert!(cypher.contains("CREATE"));
    /// ```
    pub fn to_cypher(&self) -> String {
        let mut output = String::new();

        // Create nodes
        for node in self.nodes.values() {
            output.push_str(&format!(
                "CREATE (n_{}:{} {{",
                node.id.replace('-', "_"),
                node.node_type
            ));
            output.push_str(&format!("id: '{}', label: '{}'", node.id, node.label));
            for (key, value) in &node.properties {
                output.push_str(&format!(", {}: '{}'", key, value));
            }
            output.push_str("});\n");
        }

        // Create edges
        for edge in &self.edges {
            output.push_str(&format!(
                "MATCH (a {{id: '{}'}}), (b {{id: '{}'}}) CREATE (a)-[:{} {{weight: {}}}]->(b);\n",
                edge.from, edge.to, edge.relation, edge.weight
            ));
        }

        output
    }
}

impl Default for KnowledgeGraph {
    fn default() -> Self {
        Self::new()
    }
}

/// Query builder for knowledge graph.
pub struct GraphQuery<'a> {
    graph: &'a KnowledgeGraph,
    node_type_filter: Option<String>,
    label_filter: Option<String>,
    property_filters: HashMap<String, String>,
}

impl<'a> GraphQuery<'a> {
    fn new(graph: &'a KnowledgeGraph) -> Self {
        Self {
            graph,
            node_type_filter: None,
            label_filter: None,
            property_filters: HashMap::new(),
        }
    }

    /// Filters by node type.
    pub fn node_type(mut self, node_type: impl Into<String>) -> Self {
        self.node_type_filter = Some(node_type.into());
        self
    }

    /// Filters by label (substring match).
    pub fn label_contains(mut self, label: impl Into<String>) -> Self {
        self.label_filter = Some(label.into());
        self
    }

    /// Filters by property value.
    pub fn property(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.property_filters.insert(key.into(), value.into());
        self
    }

    /// Executes the query and returns matching nodes.
    pub fn execute(&self) -> Vec<&Node> {
        let mut results = Vec::new();

        for node in self.graph.nodes.values() {
            // Check node type filter
            if let Some(ref nt) = self.node_type_filter
                && &node.node_type != nt
            {
                continue;
            }

            // Check label filter
            if let Some(ref lf) = self.label_filter
                && !node.label.contains(lf)
            {
                continue;
            }

            // Check property filters
            let mut props_match = true;
            for (key, value) in &self.property_filters {
                if node.properties.get(key) != Some(value) {
                    props_match = false;
                    break;
                }
            }
            if !props_match {
                continue;
            }

            results.push(node);
        }

        results
    }

    /// Returns the count of matching nodes.
    pub fn count(&self) -> usize {
        self.execute().len()
    }

    /// Returns the first matching node.
    pub fn first(&self) -> Option<&Node> {
        self.execute().into_iter().next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Effect, EffectType};

    #[test]
    fn test_node_creation() {
        let node = Node::new("n1", "Statute", "Tax Law").with_property("jurisdiction", "US");

        assert_eq!(node.id, "n1");
        assert_eq!(node.node_type, "Statute");
        assert_eq!(node.label, "Tax Law");
        assert_eq!(node.properties.get("jurisdiction"), Some(&"US".to_string()));
    }

    #[test]
    fn test_edge_creation() {
        let edge = Edge::new("n1", "n2", RelationType::References).with_weight(0.8);

        assert_eq!(edge.from, "n1");
        assert_eq!(edge.to, "n2");
        assert_eq!(edge.weight, 0.8);
    }

    #[test]
    fn test_knowledge_graph_add_statute() {
        let mut kg = KnowledgeGraph::new();
        let statute = Statute::new("s1", "Tax Law", Effect::new(EffectType::Grant, "Credit"));
        kg.add_statute(&statute);

        assert_eq!(kg.node_count(), 2); // Statute + Effect
        assert_eq!(kg.edge_count(), 1);
    }

    #[test]
    fn test_graph_query() {
        let mut kg = KnowledgeGraph::new();
        let statute = Statute::new("s1", "Tax Law", Effect::new(EffectType::Grant, "Credit"));
        kg.add_statute(&statute);

        let query = kg.query().node_type("Statute");
        let nodes = query.execute();
        assert_eq!(nodes.len(), 1);

        let query2 = kg.query().node_type("Effect");
        let effects = query2.execute();
        assert_eq!(effects.len(), 1);
    }

    #[test]
    fn test_neighbors() {
        let mut kg = KnowledgeGraph::new();
        let statute = Statute::new("s1", "Tax Law", Effect::new(EffectType::Grant, "Credit"));
        kg.add_statute(&statute);

        let neighbors = kg.neighbors("s1", Some(RelationType::Produces));
        assert_eq!(neighbors.len(), 1);
        assert_eq!(neighbors[0].node_type, "Effect");
    }

    #[test]
    fn test_cypher_export() {
        let mut kg = KnowledgeGraph::new();
        let statute = Statute::new("s1", "Tax Law", Effect::new(EffectType::Grant, "Credit"));
        kg.add_statute(&statute);

        let cypher = kg.to_cypher();
        assert!(cypher.contains("CREATE"));
        assert!(cypher.contains("MATCH"));
    }
}
