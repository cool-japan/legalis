//! Graph Database Backend Module (v0.2.3)
//!
//! This module provides graph database integration for statute registry:
//! - Neo4j storage backend for graph-native statute storage
//! - Relationship graph queries for analyzing statute connections
//! - Path-based dependency analysis for tracing statute relationships
//! - Graph-based impact analysis for change propagation
//! - Visual graph exploration API for interactive visualization

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use uuid::Uuid;

// =============================================================================
// Graph Node and Edge Types
// =============================================================================

/// Types of nodes in the statute graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GraphNodeType {
    /// Statute node
    Statute,
    /// Jurisdiction node
    Jurisdiction,
    /// Tag node
    Tag,
    /// Concept node
    Concept,
    /// Section node (part of a statute)
    Section,
}

impl GraphNodeType {
    /// Returns the node type name as a string.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Statute => "Statute",
            Self::Jurisdiction => "Jurisdiction",
            Self::Tag => "Tag",
            Self::Concept => "Concept",
            Self::Section => "Section",
        }
    }
}

/// Types of edges (relationships) in the statute graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GraphEdgeType {
    /// References another statute
    References,
    /// Amends another statute
    Amends,
    /// Supersedes another statute
    Supersedes,
    /// Depends on another statute
    DependsOn,
    /// Conflicts with another statute
    ConflictsWith,
    /// Belongs to jurisdiction
    BelongsTo,
    /// Tagged with
    TaggedWith,
    /// Related to concept
    RelatedToConcept,
    /// Contains section
    ContainsSection,
    /// Derived from (version history)
    DerivedFrom,
}

impl GraphEdgeType {
    /// Returns the edge type name as a string.
    pub fn name(&self) -> &'static str {
        match self {
            Self::References => "REFERENCES",
            Self::Amends => "AMENDS",
            Self::Supersedes => "SUPERSEDES",
            Self::DependsOn => "DEPENDS_ON",
            Self::ConflictsWith => "CONFLICTS_WITH",
            Self::BelongsTo => "BELONGS_TO",
            Self::TaggedWith => "TAGGED_WITH",
            Self::RelatedToConcept => "RELATED_TO_CONCEPT",
            Self::ContainsSection => "CONTAINS_SECTION",
            Self::DerivedFrom => "DERIVED_FROM",
        }
    }

    /// Returns whether this edge type is directional.
    pub fn is_directional(&self) -> bool {
        match self {
            Self::ConflictsWith => false, // Conflicts are bidirectional
            _ => true,
        }
    }
}

/// Represents a node in the statute graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    /// Unique node ID
    pub id: String,
    /// Node type
    pub node_type: GraphNodeType,
    /// Node properties (key-value pairs)
    pub properties: HashMap<String, String>,
    /// When the node was created
    pub created_at: DateTime<Utc>,
    /// When the node was last updated
    pub updated_at: DateTime<Utc>,
}

impl GraphNode {
    /// Creates a new graph node.
    pub fn new(id: impl Into<String>, node_type: GraphNodeType) -> Self {
        let now = Utc::now();
        Self {
            id: id.into(),
            node_type,
            properties: HashMap::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Adds a property to the node.
    pub fn add_property(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.properties.insert(key.into(), value.into());
        self.updated_at = Utc::now();
    }

    /// Gets a property value.
    pub fn get_property(&self, key: &str) -> Option<&str> {
        self.properties.get(key).map(|s| s.as_str())
    }
}

/// Represents an edge (relationship) in the statute graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphEdge {
    /// Unique edge ID
    pub id: Uuid,
    /// Source node ID
    pub from_id: String,
    /// Target node ID
    pub to_id: String,
    /// Edge type
    pub edge_type: GraphEdgeType,
    /// Edge properties (key-value pairs)
    pub properties: HashMap<String, String>,
    /// Edge weight (for weighted graph algorithms)
    pub weight: f64,
    /// When the edge was created
    pub created_at: DateTime<Utc>,
}

impl GraphEdge {
    /// Creates a new graph edge.
    pub fn new(
        from_id: impl Into<String>,
        to_id: impl Into<String>,
        edge_type: GraphEdgeType,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            from_id: from_id.into(),
            to_id: to_id.into(),
            edge_type,
            properties: HashMap::new(),
            weight: 1.0,
            created_at: Utc::now(),
        }
    }

    /// Sets the edge weight.
    pub fn with_weight(mut self, weight: f64) -> Self {
        self.weight = weight;
        self
    }

    /// Adds a property to the edge.
    pub fn add_property(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.properties.insert(key.into(), value.into());
    }
}

// =============================================================================
// Neo4j Backend Configuration
// =============================================================================

/// Neo4j connection configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Neo4jConfig {
    /// Neo4j server URI (e.g., "neo4j://localhost:7687")
    pub uri: String,
    /// Database name
    pub database: String,
    /// Username for authentication
    pub username: String,
    /// Password for authentication (not serialized for security)
    #[serde(skip)]
    pub password: String,
    /// Connection pool size
    pub max_connections: usize,
    /// Connection timeout in seconds
    pub timeout_secs: u64,
}

impl Neo4jConfig {
    /// Creates a new Neo4j configuration.
    pub fn new(
        uri: impl Into<String>,
        database: impl Into<String>,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> Self {
        Self {
            uri: uri.into(),
            database: database.into(),
            username: username.into(),
            password: password.into(),
            max_connections: 10,
            timeout_secs: 30,
        }
    }

    /// Creates a default local configuration.
    pub fn local() -> Self {
        Self::new("neo4j://localhost:7687", "neo4j", "neo4j", "password")
    }

    /// Sets the connection pool size.
    pub fn with_max_connections(mut self, max_connections: usize) -> Self {
        self.max_connections = max_connections;
        self
    }

    /// Sets the connection timeout.
    pub fn with_timeout(mut self, timeout_secs: u64) -> Self {
        self.timeout_secs = timeout_secs;
        self
    }
}

/// Neo4j storage backend for statute graph.
#[derive(Debug, Clone)]
pub struct Neo4jBackend {
    /// Configuration
    #[allow(dead_code)]
    config: Neo4jConfig,
    /// Connected status
    connected: bool,
    /// Statistics
    stats: Neo4jStats,
}

impl Neo4jBackend {
    /// Creates a new Neo4j backend.
    pub fn new(config: Neo4jConfig) -> Self {
        Self {
            config,
            connected: false,
            stats: Neo4jStats::default(),
        }
    }

    /// Connects to the Neo4j database.
    pub fn connect(&mut self) -> Result<(), GraphError> {
        // In a real implementation, this would establish a connection
        // For now, we'll simulate a successful connection
        self.connected = true;
        Ok(())
    }

    /// Disconnects from the Neo4j database.
    pub fn disconnect(&mut self) {
        self.connected = false;
    }

    /// Returns whether the backend is connected.
    pub fn is_connected(&self) -> bool {
        self.connected
    }

    /// Creates a node in the database.
    pub fn create_node(&mut self, _node: &GraphNode) -> Result<(), GraphError> {
        if !self.connected {
            return Err(GraphError::NotConnected);
        }
        // Simulate node creation
        self.stats.nodes_created += 1;
        Ok(())
    }

    /// Creates an edge in the database.
    pub fn create_edge(&mut self, _edge: &GraphEdge) -> Result<(), GraphError> {
        if !self.connected {
            return Err(GraphError::NotConnected);
        }
        // Simulate edge creation
        self.stats.edges_created += 1;
        Ok(())
    }

    /// Returns the backend statistics.
    pub fn stats(&self) -> &Neo4jStats {
        &self.stats
    }

    /// Executes a Cypher query.
    pub fn execute_cypher(
        &mut self,
        query: &str,
    ) -> Result<Vec<HashMap<String, String>>, GraphError> {
        if !self.connected {
            return Err(GraphError::NotConnected);
        }
        self.stats.queries_executed += 1;
        // In a real implementation, this would execute the query
        // For now, return empty results
        let _ = query; // Use the query parameter
        Ok(Vec::new())
    }
}

/// Statistics for Neo4j backend.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Neo4jStats {
    /// Total nodes created
    pub nodes_created: usize,
    /// Total edges created
    pub edges_created: usize,
    /// Total queries executed
    pub queries_executed: usize,
}

// =============================================================================
// Graph Query System
// =============================================================================

/// Graph query builder for statute relationships.
#[derive(Debug, Clone)]
pub struct GraphQuery {
    /// Starting node IDs
    start_nodes: Vec<String>,
    /// Edge types to traverse
    edge_types: Vec<GraphEdgeType>,
    /// Maximum depth for traversal
    max_depth: Option<usize>,
    /// Filter conditions
    filters: Vec<QueryFilter>,
}

impl GraphQuery {
    /// Creates a new graph query.
    pub fn new() -> Self {
        Self {
            start_nodes: Vec::new(),
            edge_types: Vec::new(),
            max_depth: None,
            filters: Vec::new(),
        }
    }

    /// Sets the starting nodes.
    pub fn start_from(mut self, node_ids: Vec<String>) -> Self {
        self.start_nodes = node_ids;
        self
    }

    /// Adds edge types to traverse.
    pub fn follow_edges(mut self, edge_types: Vec<GraphEdgeType>) -> Self {
        self.edge_types = edge_types;
        self
    }

    /// Sets the maximum traversal depth.
    pub fn max_depth(mut self, depth: usize) -> Self {
        self.max_depth = Some(depth);
        self
    }

    /// Adds a filter condition.
    pub fn filter(mut self, filter: QueryFilter) -> Self {
        self.filters.push(filter);
        self
    }

    /// Generates a Cypher query string.
    pub fn to_cypher(&self) -> String {
        let mut query = String::from("MATCH ");

        // Build the pattern
        if let Some(depth) = self.max_depth {
            query.push_str(&format!("(start)-[r*1..{}]->(end) ", depth));
        } else {
            query.push_str("(start)-[r]->(end) ");
        }

        // Add WHERE clause
        if !self.start_nodes.is_empty() || !self.filters.is_empty() {
            query.push_str("WHERE ");
            let mut conditions = Vec::new();

            if !self.start_nodes.is_empty() {
                let ids = self
                    .start_nodes
                    .iter()
                    .map(|id| format!("'{}'", id))
                    .collect::<Vec<_>>()
                    .join(", ");
                conditions.push(format!("start.id IN [{}]", ids));
            }

            for filter in &self.filters {
                conditions.push(filter.to_cypher());
            }

            query.push_str(&conditions.join(" AND "));
        }

        query.push_str(" RETURN start, r, end");
        query
    }
}

impl Default for GraphQuery {
    fn default() -> Self {
        Self::new()
    }
}

/// Query filter for graph traversal.
#[derive(Debug, Clone)]
pub enum QueryFilter {
    /// Node property equals value
    NodePropertyEquals { key: String, value: String },
    /// Edge property equals value
    EdgePropertyEquals { key: String, value: String },
    /// Node type matches
    NodeTypeIs(GraphNodeType),
    /// Edge type matches
    EdgeTypeIs(GraphEdgeType),
}

impl QueryFilter {
    /// Converts the filter to Cypher syntax.
    pub fn to_cypher(&self) -> String {
        match self {
            Self::NodePropertyEquals { key, value } => {
                format!("end.{} = '{}'", key, value)
            }
            Self::EdgePropertyEquals { key, value } => {
                format!("r.{} = '{}'", key, value)
            }
            Self::NodeTypeIs(node_type) => {
                format!("end:'{}'", node_type.name())
            }
            Self::EdgeTypeIs(edge_type) => {
                format!("type(r) = '{}'", edge_type.name())
            }
        }
    }
}

// =============================================================================
// Path-Based Dependency Analysis
// =============================================================================

/// Represents a path between two nodes in the graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphPath {
    /// Node IDs in the path (from start to end)
    pub nodes: Vec<String>,
    /// Edge types in the path
    pub edges: Vec<GraphEdgeType>,
    /// Total path length
    pub length: usize,
    /// Total path weight (sum of edge weights)
    pub weight: f64,
}

impl GraphPath {
    /// Creates a new graph path.
    pub fn new(nodes: Vec<String>, edges: Vec<GraphEdgeType>) -> Self {
        let length = edges.len();
        Self {
            nodes,
            edges,
            length,
            weight: length as f64,
        }
    }

    /// Returns whether this path contains a specific node.
    pub fn contains_node(&self, node_id: &str) -> bool {
        self.nodes.iter().any(|n| n == node_id)
    }

    /// Returns whether this path contains a cycle.
    pub fn has_cycle(&self) -> bool {
        let mut seen = HashSet::new();
        for node in &self.nodes {
            if !seen.insert(node) {
                return true;
            }
        }
        false
    }

    /// Returns the start node.
    pub fn start(&self) -> Option<&str> {
        self.nodes.first().map(|s| s.as_str())
    }

    /// Returns the end node.
    pub fn end(&self) -> Option<&str> {
        self.nodes.last().map(|s| s.as_str())
    }
}

/// Dependency analysis engine for statute relationships.
#[derive(Debug, Clone)]
pub struct DependencyAnalyzer {
    /// Graph data (adjacency list)
    graph: HashMap<String, Vec<(String, GraphEdgeType, f64)>>,
}

impl DependencyAnalyzer {
    /// Creates a new dependency analyzer.
    pub fn new() -> Self {
        Self {
            graph: HashMap::new(),
        }
    }

    /// Adds an edge to the graph.
    pub fn add_edge(&mut self, from: String, to: String, edge_type: GraphEdgeType, weight: f64) {
        self.graph
            .entry(from)
            .or_default()
            .push((to, edge_type, weight));
    }

    /// Finds all paths between two nodes using BFS.
    pub fn find_all_paths(&self, start: &str, end: &str, max_depth: usize) -> Vec<GraphPath> {
        let mut paths = Vec::new();
        let mut queue: VecDeque<(String, Vec<String>, Vec<GraphEdgeType>)> = VecDeque::new();
        queue.push_back((start.to_string(), vec![start.to_string()], Vec::new()));

        while let Some((current, path, edges)) = queue.pop_front() {
            if path.len() > max_depth + 1 {
                continue;
            }

            if current == end && path.len() > 1 {
                paths.push(GraphPath::new(path, edges));
                continue;
            }

            if let Some(neighbors) = self.graph.get(&current) {
                for (next, edge_type, _weight) in neighbors {
                    if !path.contains(next) {
                        let mut new_path = path.clone();
                        new_path.push(next.clone());
                        let mut new_edges = edges.clone();
                        new_edges.push(*edge_type);
                        queue.push_back((next.clone(), new_path, new_edges));
                    }
                }
            }
        }

        paths
    }

    /// Finds the shortest path between two nodes using BFS.
    pub fn find_shortest_path(&self, start: &str, end: &str) -> Option<GraphPath> {
        let mut queue: VecDeque<(String, Vec<String>, Vec<GraphEdgeType>)> = VecDeque::new();
        let mut visited = HashSet::new();

        queue.push_back((start.to_string(), vec![start.to_string()], Vec::new()));
        visited.insert(start.to_string());

        while let Some((current, path, edges)) = queue.pop_front() {
            if current == end {
                return Some(GraphPath::new(path, edges));
            }

            if let Some(neighbors) = self.graph.get(&current) {
                for (next, edge_type, _weight) in neighbors {
                    if !visited.contains(next) {
                        visited.insert(next.clone());
                        let mut new_path = path.clone();
                        new_path.push(next.clone());
                        let mut new_edges = edges.clone();
                        new_edges.push(*edge_type);
                        queue.push_back((next.clone(), new_path, new_edges));
                    }
                }
            }
        }

        None
    }

    /// Finds all dependencies of a node (nodes it depends on).
    pub fn find_dependencies(&self, node_id: &str) -> HashSet<String> {
        let mut dependencies = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(node_id.to_string());

        while let Some(current) = queue.pop_front() {
            if let Some(neighbors) = self.graph.get(&current) {
                for (next, edge_type, _) in neighbors {
                    if matches!(
                        edge_type,
                        GraphEdgeType::DependsOn | GraphEdgeType::References
                    ) && dependencies.insert(next.clone())
                    {
                        queue.push_back(next.clone());
                    }
                }
            }
        }

        dependencies
    }

    /// Finds all dependents of a node (nodes that depend on it).
    pub fn find_dependents(&self, node_id: &str) -> HashSet<String> {
        let mut dependents = HashSet::new();

        for (from, edges) in &self.graph {
            for (to, edge_type, _) in edges {
                if to == node_id
                    && matches!(
                        edge_type,
                        GraphEdgeType::DependsOn | GraphEdgeType::References
                    )
                {
                    dependents.insert(from.clone());
                }
            }
        }

        dependents
    }

    /// Detects circular dependencies.
    pub fn detect_circular_dependencies(&self, start: &str) -> Vec<GraphPath> {
        let mut cycles = Vec::new();
        let mut visited = HashSet::new();
        let mut path = Vec::new();
        let mut edges = Vec::new();

        self.dfs_cycles(start, &mut visited, &mut path, &mut edges, &mut cycles);
        cycles
    }

    #[allow(dead_code)]
    fn dfs_cycles(
        &self,
        node: &str,
        visited: &mut HashSet<String>,
        path: &mut Vec<String>,
        edges: &mut Vec<GraphEdgeType>,
        cycles: &mut Vec<GraphPath>,
    ) {
        if path.contains(&node.to_string()) {
            // Found a cycle
            let cycle_start = path.iter().position(|n| n == node).unwrap();
            let cycle_nodes = path[cycle_start..].to_vec();
            let cycle_edges = edges[cycle_start..].to_vec();
            cycles.push(GraphPath::new(cycle_nodes, cycle_edges));
            return;
        }

        if visited.contains(node) {
            return;
        }

        visited.insert(node.to_string());
        path.push(node.to_string());

        if let Some(neighbors) = self.graph.get(node) {
            for (next, edge_type, _) in neighbors {
                edges.push(*edge_type);
                self.dfs_cycles(next, visited, path, edges, cycles);
                edges.pop();
            }
        }

        path.pop();
    }
}

impl Default for DependencyAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Graph-Based Impact Analysis
// =============================================================================

/// Impact analysis result for statute changes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactAnalysis {
    /// The statute that was changed
    pub changed_statute: String,
    /// Directly affected statutes (distance = 1)
    pub directly_affected: Vec<String>,
    /// Indirectly affected statutes (distance > 1)
    pub indirectly_affected: Vec<(String, usize)>, // (statute_id, distance)
    /// Total number of affected statutes
    pub total_affected: usize,
    /// Maximum impact depth
    pub max_depth: usize,
    /// Impact score (0.0 to 1.0)
    pub impact_score: f64,
}

impl ImpactAnalysis {
    /// Creates a new impact analysis.
    pub fn new(changed_statute: impl Into<String>) -> Self {
        Self {
            changed_statute: changed_statute.into(),
            directly_affected: Vec::new(),
            indirectly_affected: Vec::new(),
            total_affected: 0,
            max_depth: 0,
            impact_score: 0.0,
        }
    }

    /// Calculates the impact score based on affected statutes.
    pub fn calculate_score(&mut self) {
        let direct_weight = 1.0;
        let indirect_weight = 0.5;

        let direct_score = self.directly_affected.len() as f64 * direct_weight;
        let indirect_score = self.indirectly_affected.len() as f64 * indirect_weight;

        self.impact_score = (direct_score + indirect_score) / 100.0;
        self.impact_score = self.impact_score.min(1.0);
    }

    /// Returns whether the impact is high (score > 0.7).
    pub fn is_high_impact(&self) -> bool {
        self.impact_score > 0.7
    }

    /// Returns whether the impact is medium (0.3 < score <= 0.7).
    pub fn is_medium_impact(&self) -> bool {
        self.impact_score > 0.3 && self.impact_score <= 0.7
    }

    /// Returns whether the impact is low (score <= 0.3).
    pub fn is_low_impact(&self) -> bool {
        self.impact_score <= 0.3
    }
}

/// Impact analyzer for statute changes.
#[derive(Debug, Clone)]
pub struct ImpactAnalyzer {
    /// Dependency analyzer
    analyzer: DependencyAnalyzer,
}

impl ImpactAnalyzer {
    /// Creates a new impact analyzer.
    pub fn new(analyzer: DependencyAnalyzer) -> Self {
        Self { analyzer }
    }

    /// Analyzes the impact of changing a statute.
    pub fn analyze_impact(&self, statute_id: &str, max_depth: usize) -> ImpactAnalysis {
        let mut impact = ImpactAnalysis::new(statute_id);
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        // Start with the changed statute at depth 0
        queue.push_back((statute_id.to_string(), 0));
        visited.insert(statute_id.to_string());

        while let Some((current, depth)) = queue.pop_front() {
            if depth >= max_depth {
                continue;
            }

            // Find all dependents of the current statute
            let dependents = self.analyzer.find_dependents(&current);

            for dependent in dependents {
                if !visited.contains(&dependent) {
                    visited.insert(dependent.clone());

                    if depth == 0 {
                        // Direct dependency
                        impact.directly_affected.push(dependent.clone());
                    } else {
                        // Indirect dependency
                        impact
                            .indirectly_affected
                            .push((dependent.clone(), depth + 1));
                    }

                    impact.max_depth = impact.max_depth.max(depth + 1);
                    queue.push_back((dependent, depth + 1));
                }
            }
        }

        impact.total_affected = impact.directly_affected.len() + impact.indirectly_affected.len();
        impact.calculate_score();
        impact
    }

    /// Analyzes the ripple effect of multiple statute changes.
    pub fn analyze_ripple_effect(
        &self,
        statute_ids: &[String],
        max_depth: usize,
    ) -> Vec<ImpactAnalysis> {
        statute_ids
            .iter()
            .map(|id| self.analyze_impact(id, max_depth))
            .collect()
    }
}

// =============================================================================
// Visual Graph Exploration API
// =============================================================================

/// Graph visualization layout type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GraphLayout {
    /// Force-directed layout
    ForceDirected,
    /// Hierarchical layout (top-down)
    Hierarchical,
    /// Circular layout
    Circular,
    /// Grid layout
    Grid,
    /// Tree layout
    Tree,
}

impl GraphLayout {
    /// Returns the layout name as a string.
    pub fn name(&self) -> &'static str {
        match self {
            Self::ForceDirected => "force-directed",
            Self::Hierarchical => "hierarchical",
            Self::Circular => "circular",
            Self::Grid => "grid",
            Self::Tree => "tree",
        }
    }
}

/// Visual node for graph rendering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualNode {
    /// Node ID
    pub id: String,
    /// Node label (display text)
    pub label: String,
    /// Node type
    pub node_type: GraphNodeType,
    /// X coordinate
    pub x: f64,
    /// Y coordinate
    pub y: f64,
    /// Node size
    pub size: f64,
    /// Node color (hex color code)
    pub color: String,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl VisualNode {
    /// Creates a new visual node.
    pub fn new(id: impl Into<String>, label: impl Into<String>, node_type: GraphNodeType) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            node_type,
            x: 0.0,
            y: 0.0,
            size: 10.0,
            color: "#3b82f6".to_string(), // Default blue color
            metadata: HashMap::new(),
        }
    }

    /// Sets the position.
    pub fn with_position(mut self, x: f64, y: f64) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    /// Sets the size.
    pub fn with_size(mut self, size: f64) -> Self {
        self.size = size;
        self
    }

    /// Sets the color.
    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.color = color.into();
        self
    }
}

/// Visual edge for graph rendering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualEdge {
    /// Edge ID
    pub id: String,
    /// Source node ID
    pub source: String,
    /// Target node ID
    pub target: String,
    /// Edge type
    pub edge_type: GraphEdgeType,
    /// Edge label (display text)
    pub label: String,
    /// Edge color (hex color code)
    pub color: String,
    /// Edge width
    pub width: f64,
    /// Whether the edge is directed
    pub directed: bool,
}

impl VisualEdge {
    /// Creates a new visual edge.
    pub fn new(
        source: impl Into<String>,
        target: impl Into<String>,
        edge_type: GraphEdgeType,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            source: source.into(),
            target: target.into(),
            edge_type,
            label: edge_type.name().to_string(),
            color: "#6b7280".to_string(), // Default gray color
            width: 1.0,
            directed: edge_type.is_directional(),
        }
    }

    /// Sets the label.
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    /// Sets the color.
    pub fn with_color(mut self, color: impl Into<String>) -> Self {
        self.color = color.into();
        self
    }

    /// Sets the width.
    pub fn with_width(mut self, width: f64) -> Self {
        self.width = width;
        self
    }
}

/// Graph visualization data structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphVisualization {
    /// Visual nodes
    pub nodes: Vec<VisualNode>,
    /// Visual edges
    pub edges: Vec<VisualEdge>,
    /// Layout type
    pub layout: GraphLayout,
    /// Graph title
    pub title: String,
    /// Graph description
    pub description: Option<String>,
}

impl GraphVisualization {
    /// Creates a new graph visualization.
    pub fn new(layout: GraphLayout, title: impl Into<String>) -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            layout,
            title: title.into(),
            description: None,
        }
    }

    /// Adds a node to the visualization.
    pub fn add_node(&mut self, node: VisualNode) {
        self.nodes.push(node);
    }

    /// Adds an edge to the visualization.
    pub fn add_edge(&mut self, edge: VisualEdge) {
        self.edges.push(edge);
    }

    /// Sets the description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Returns the total number of elements (nodes + edges).
    pub fn total_elements(&self) -> usize {
        self.nodes.len() + self.edges.len()
    }

    /// Exports to JSON format.
    pub fn to_json(&self) -> Result<String, GraphError> {
        serde_json::to_string_pretty(self)
            .map_err(|e| GraphError::SerializationError(e.to_string()))
    }

    /// Exports to DOT format (Graphviz).
    pub fn to_dot(&self) -> String {
        let mut dot = String::from("digraph G {\n");
        dot.push_str("  rankdir=TB;\n");
        dot.push_str("  node [shape=box];\n\n");

        // Add nodes
        for node in &self.nodes {
            dot.push_str(&format!("  \"{}\" [label=\"{}\"];\n", node.id, node.label));
        }

        dot.push('\n');

        // Add edges
        for edge in &self.edges {
            let arrow = if edge.directed { "->" } else { "--" };
            dot.push_str(&format!(
                "  \"{}\" {} \"{}\" [label=\"{}\"];\n",
                edge.source, arrow, edge.target, edge.label
            ));
        }

        dot.push_str("}\n");
        dot
    }
}

// =============================================================================
// Error Handling
// =============================================================================

/// Errors that can occur during graph operations.
#[derive(Debug, Clone, thiserror::Error)]
pub enum GraphError {
    #[error("Not connected to database")]
    NotConnected,

    #[error("Node not found: {0}")]
    NodeNotFound(String),

    #[error("Edge not found: {0}")]
    EdgeNotFound(String),

    #[error("Invalid query: {0}")]
    InvalidQuery(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Path not found between {0} and {1}")]
    PathNotFound(String, String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_node_type() {
        assert_eq!(GraphNodeType::Statute.name(), "Statute");
        assert_eq!(GraphNodeType::Jurisdiction.name(), "Jurisdiction");
    }

    #[test]
    fn test_graph_edge_type() {
        assert_eq!(GraphEdgeType::References.name(), "REFERENCES");
        assert!(GraphEdgeType::References.is_directional());
        assert!(!GraphEdgeType::ConflictsWith.is_directional());
    }

    #[test]
    fn test_graph_node() {
        let mut node = GraphNode::new("statute-001", GraphNodeType::Statute);
        node.add_property("title", "Test Statute");

        assert_eq!(node.id, "statute-001");
        assert_eq!(node.node_type, GraphNodeType::Statute);
        assert_eq!(node.get_property("title"), Some("Test Statute"));
    }

    #[test]
    fn test_graph_edge() {
        let edge = GraphEdge::new("statute-001", "statute-002", GraphEdgeType::References)
            .with_weight(2.0);

        assert_eq!(edge.from_id, "statute-001");
        assert_eq!(edge.to_id, "statute-002");
        assert_eq!(edge.edge_type, GraphEdgeType::References);
        assert_eq!(edge.weight, 2.0);
    }

    #[test]
    fn test_neo4j_config() {
        let config = Neo4jConfig::local()
            .with_max_connections(20)
            .with_timeout(60);

        assert_eq!(config.max_connections, 20);
        assert_eq!(config.timeout_secs, 60);
    }

    #[test]
    fn test_neo4j_backend() {
        let config = Neo4jConfig::local();
        let mut backend = Neo4jBackend::new(config);

        assert!(!backend.is_connected());
        assert!(backend.connect().is_ok());
        assert!(backend.is_connected());

        let node = GraphNode::new("test", GraphNodeType::Statute);
        assert!(backend.create_node(&node).is_ok());
        assert_eq!(backend.stats().nodes_created, 1);
    }

    #[test]
    fn test_graph_query() {
        let query = GraphQuery::new()
            .start_from(vec!["statute-001".to_string()])
            .follow_edges(vec![GraphEdgeType::References])
            .max_depth(3);

        let cypher = query.to_cypher();
        assert!(cypher.contains("MATCH"));
        assert!(cypher.contains("statute-001"));
    }

    #[test]
    fn test_query_filter() {
        let filter = QueryFilter::NodePropertyEquals {
            key: "status".to_string(),
            value: "active".to_string(),
        };

        let cypher = filter.to_cypher();
        assert!(cypher.contains("end.status = 'active'"));
    }

    #[test]
    fn test_graph_path() {
        let nodes = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        let edges = vec![GraphEdgeType::References, GraphEdgeType::DependsOn];
        let path = GraphPath::new(nodes, edges);

        assert_eq!(path.length, 2);
        assert_eq!(path.start(), Some("A"));
        assert_eq!(path.end(), Some("C"));
        assert!(path.contains_node("B"));
        assert!(!path.has_cycle());
    }

    #[test]
    fn test_dependency_analyzer() {
        let mut analyzer = DependencyAnalyzer::new();
        analyzer.add_edge(
            "A".to_string(),
            "B".to_string(),
            GraphEdgeType::DependsOn,
            1.0,
        );
        analyzer.add_edge(
            "B".to_string(),
            "C".to_string(),
            GraphEdgeType::DependsOn,
            1.0,
        );

        let deps = analyzer.find_dependencies("A");
        assert_eq!(deps.len(), 2);
        assert!(deps.contains("B"));
        assert!(deps.contains("C"));
    }

    #[test]
    fn test_shortest_path() {
        let mut analyzer = DependencyAnalyzer::new();
        analyzer.add_edge(
            "A".to_string(),
            "B".to_string(),
            GraphEdgeType::References,
            1.0,
        );
        analyzer.add_edge(
            "B".to_string(),
            "C".to_string(),
            GraphEdgeType::References,
            1.0,
        );

        let path = analyzer.find_shortest_path("A", "C");
        assert!(path.is_some());
        let path = path.unwrap();
        assert_eq!(path.length, 2);
        assert_eq!(path.nodes, vec!["A", "B", "C"]);
    }

    #[test]
    fn test_impact_analysis() {
        let mut impact = ImpactAnalysis::new("statute-001");
        impact.directly_affected = vec!["statute-002".to_string()];
        impact.indirectly_affected = vec![("statute-003".to_string(), 2)];
        impact.total_affected = 2;
        impact.calculate_score();

        assert!(impact.impact_score > 0.0);
        assert!(impact.is_low_impact());
    }

    #[test]
    fn test_impact_analyzer() {
        let mut dep_analyzer = DependencyAnalyzer::new();
        dep_analyzer.add_edge(
            "B".to_string(),
            "A".to_string(),
            GraphEdgeType::DependsOn,
            1.0,
        );
        dep_analyzer.add_edge(
            "C".to_string(),
            "B".to_string(),
            GraphEdgeType::DependsOn,
            1.0,
        );

        let impact_analyzer = ImpactAnalyzer::new(dep_analyzer);
        let impact = impact_analyzer.analyze_impact("A", 5);

        assert_eq!(impact.changed_statute, "A");
        assert!(impact.total_affected > 0);
    }

    #[test]
    fn test_graph_layout() {
        assert_eq!(GraphLayout::ForceDirected.name(), "force-directed");
        assert_eq!(GraphLayout::Hierarchical.name(), "hierarchical");
    }

    #[test]
    fn test_visual_node() {
        let node = VisualNode::new("node-1", "Test Node", GraphNodeType::Statute)
            .with_position(100.0, 200.0)
            .with_size(15.0)
            .with_color("#ff0000");

        assert_eq!(node.id, "node-1");
        assert_eq!(node.x, 100.0);
        assert_eq!(node.y, 200.0);
        assert_eq!(node.size, 15.0);
        assert_eq!(node.color, "#ff0000");
    }

    #[test]
    fn test_visual_edge() {
        let edge = VisualEdge::new("node-1", "node-2", GraphEdgeType::References)
            .with_label("References")
            .with_color("#00ff00")
            .with_width(2.0);

        assert_eq!(edge.source, "node-1");
        assert_eq!(edge.target, "node-2");
        assert_eq!(edge.width, 2.0);
        assert_eq!(edge.color, "#00ff00");
    }

    #[test]
    fn test_graph_visualization() {
        let mut viz = GraphVisualization::new(GraphLayout::ForceDirected, "Test Graph")
            .with_description("A test graph");

        let node1 = VisualNode::new("n1", "Node 1", GraphNodeType::Statute);
        let node2 = VisualNode::new("n2", "Node 2", GraphNodeType::Statute);
        let edge = VisualEdge::new("n1", "n2", GraphEdgeType::References);

        viz.add_node(node1);
        viz.add_node(node2);
        viz.add_edge(edge);

        assert_eq!(viz.nodes.len(), 2);
        assert_eq!(viz.edges.len(), 1);
        assert_eq!(viz.total_elements(), 3);
    }

    #[test]
    fn test_visualization_to_json() {
        let viz = GraphVisualization::new(GraphLayout::Hierarchical, "Test");
        let json = viz.to_json();
        assert!(json.is_ok());
    }

    #[test]
    fn test_visualization_to_dot() {
        let mut viz = GraphVisualization::new(GraphLayout::Tree, "Test");
        viz.add_node(VisualNode::new("A", "Node A", GraphNodeType::Statute));
        viz.add_node(VisualNode::new("B", "Node B", GraphNodeType::Statute));
        viz.add_edge(VisualEdge::new("A", "B", GraphEdgeType::References));

        let dot = viz.to_dot();
        assert!(dot.contains("digraph G"));
        assert!(dot.contains("Node A"));
        assert!(dot.contains("REFERENCES"));
    }
}
