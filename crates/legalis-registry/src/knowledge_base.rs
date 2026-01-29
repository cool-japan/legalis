//! Legal Knowledge Base module for advanced legal research and statute analysis.
//!
//! This module provides comprehensive knowledge management capabilities:
//! - Statute-to-concept linking for semantic organization
//! - Legal ontology integration for standardized terminology
//! - Case law cross-references for judicial precedent tracking
//! - Knowledge graph visualization for relationship mapping
//! - AI-powered legal research with intelligent querying

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

use crate::StatuteEntry;

// ============================================================================
// Concept Linking System
// ============================================================================

/// Legal concept with semantic information.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LegalConcept {
    /// Unique concept identifier
    pub concept_id: Uuid,
    /// Human-readable concept name
    pub name: String,
    /// Detailed concept description
    pub description: String,
    /// Parent concept for hierarchical organization
    pub parent_concept_id: Option<Uuid>,
    /// Related concept IDs
    pub related_concepts: Vec<Uuid>,
    /// Concept category (e.g., "contract law", "criminal law")
    pub category: String,
    /// Synonyms and alternative names
    pub synonyms: Vec<String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last modified timestamp
    pub updated_at: DateTime<Utc>,
}

impl LegalConcept {
    /// Creates a new legal concept.
    pub fn new(name: String, description: String, category: String) -> Self {
        let now = Utc::now();
        Self {
            concept_id: Uuid::new_v4(),
            name,
            description,
            parent_concept_id: None,
            related_concepts: Vec::new(),
            category,
            synonyms: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Adds a parent concept.
    pub fn with_parent(mut self, parent_id: Uuid) -> Self {
        self.parent_concept_id = Some(parent_id);
        self.updated_at = Utc::now();
        self
    }

    /// Adds a related concept.
    pub fn add_related(&mut self, concept_id: Uuid) {
        if !self.related_concepts.contains(&concept_id) {
            self.related_concepts.push(concept_id);
            self.updated_at = Utc::now();
        }
    }

    /// Adds a synonym.
    pub fn add_synonym(&mut self, synonym: String) {
        if !self.synonyms.contains(&synonym) {
            self.synonyms.push(synonym);
            self.updated_at = Utc::now();
        }
    }
}

/// Link between statute and legal concept.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StatuteConceptLink {
    /// Link identifier
    pub link_id: Uuid,
    /// Statute ID
    pub statute_id: String,
    /// Concept ID
    pub concept_id: Uuid,
    /// Link strength/relevance (0.0-1.0)
    pub relevance_score: f64,
    /// Link type (e.g., "defines", "applies", "references")
    pub link_type: ConceptLinkType,
    /// Specific statute sections this applies to
    pub sections: Vec<String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
}

impl StatuteConceptLink {
    /// Creates a new statute-concept link.
    pub fn new(
        statute_id: String,
        concept_id: Uuid,
        link_type: ConceptLinkType,
        relevance_score: f64,
    ) -> Self {
        Self {
            link_id: Uuid::new_v4(),
            statute_id,
            concept_id,
            relevance_score: relevance_score.clamp(0.0, 1.0),
            link_type,
            sections: Vec::new(),
            created_at: Utc::now(),
        }
    }

    /// Adds sections to the link.
    pub fn with_sections(mut self, sections: Vec<String>) -> Self {
        self.sections = sections;
        self
    }
}

/// Type of concept link.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ConceptLinkType {
    /// Statute defines the concept
    Defines,
    /// Statute applies the concept
    Applies,
    /// Statute references the concept
    References,
    /// Statute modifies the concept
    Modifies,
    /// Statute exemplifies the concept
    Exemplifies,
}

// ============================================================================
// Legal Ontology Integration
// ============================================================================

/// Legal ontology for standardized terminology.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalOntology {
    /// Ontology identifier
    pub ontology_id: Uuid,
    /// Ontology name
    pub name: String,
    /// Ontology version
    pub version: String,
    /// Jurisdiction this ontology applies to
    pub jurisdiction: String,
    /// All concepts in the ontology
    pub concepts: HashMap<Uuid, LegalConcept>,
    /// Concept hierarchy (child -> parent)
    pub hierarchy: HashMap<Uuid, Uuid>,
    /// Concept relationships (concept -> related concepts)
    pub relationships: HashMap<Uuid, Vec<Uuid>>,
    /// Term index for fast lookups (term -> concept IDs)
    pub term_index: HashMap<String, Vec<Uuid>>,
}

impl LegalOntology {
    /// Creates a new legal ontology.
    pub fn new(name: String, version: String, jurisdiction: String) -> Self {
        Self {
            ontology_id: Uuid::new_v4(),
            name,
            version,
            jurisdiction,
            concepts: HashMap::new(),
            hierarchy: HashMap::new(),
            relationships: HashMap::new(),
            term_index: HashMap::new(),
        }
    }

    /// Adds a concept to the ontology.
    pub fn add_concept(&mut self, concept: LegalConcept) -> Uuid {
        let concept_id = concept.concept_id;

        // Update hierarchy
        if let Some(parent_id) = concept.parent_concept_id {
            self.hierarchy.insert(concept_id, parent_id);
        }

        // Update relationships
        if !concept.related_concepts.is_empty() {
            self.relationships
                .insert(concept_id, concept.related_concepts.clone());
        }

        // Update term index
        let terms = std::iter::once(&concept.name)
            .chain(concept.synonyms.iter())
            .map(|s| s.to_lowercase())
            .collect::<Vec<_>>();

        for term in terms {
            self.term_index.entry(term).or_default().push(concept_id);
        }

        // Add concept
        self.concepts.insert(concept_id, concept);

        concept_id
    }

    /// Finds concepts by term.
    pub fn find_concepts_by_term(&self, term: &str) -> Vec<&LegalConcept> {
        let normalized_term = term.to_lowercase();
        self.term_index
            .get(&normalized_term)
            .map(|ids| ids.iter().filter_map(|id| self.concepts.get(id)).collect())
            .unwrap_or_default()
    }

    /// Gets concept hierarchy path (from root to concept).
    pub fn get_concept_path(&self, concept_id: Uuid) -> Vec<Uuid> {
        let mut path = vec![concept_id];
        let mut current = concept_id;

        while let Some(&parent_id) = self.hierarchy.get(&current) {
            path.push(parent_id);
            current = parent_id;
        }

        path.reverse();
        path
    }

    /// Gets all descendant concepts.
    pub fn get_descendants(&self, concept_id: Uuid) -> Vec<Uuid> {
        let mut descendants = Vec::new();
        let mut to_visit = vec![concept_id];

        while let Some(current) = to_visit.pop() {
            for (&child, &parent) in &self.hierarchy {
                if parent == current && !descendants.contains(&child) {
                    descendants.push(child);
                    to_visit.push(child);
                }
            }
        }

        descendants
    }

    /// Gets related concepts (directly and transitively).
    pub fn get_related_concepts(&self, concept_id: Uuid, depth: usize) -> HashSet<Uuid> {
        let mut related = HashSet::new();
        let mut current_level = vec![concept_id];

        for _ in 0..depth {
            let mut next_level = Vec::new();

            for &current in &current_level {
                if let Some(relations) = self.relationships.get(&current) {
                    for &rel_id in relations {
                        if related.insert(rel_id) {
                            next_level.push(rel_id);
                        }
                    }
                }
            }

            if next_level.is_empty() {
                break;
            }

            current_level = next_level;
        }

        related
    }
}

// ============================================================================
// Case Law Cross-References
// ============================================================================

/// Reference to a legal case.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CaseLawReference {
    /// Case identifier (e.g., citation)
    pub case_id: String,
    /// Case name
    pub case_name: String,
    /// Court that decided the case
    pub court: String,
    /// Jurisdiction
    pub jurisdiction: String,
    /// Decision date
    pub decision_date: DateTime<Utc>,
    /// Citation (e.g., "123 F.3d 456")
    pub citation: String,
    /// Summary of the case
    pub summary: String,
    /// Key legal principles established
    pub principles: Vec<String>,
    /// Related concepts
    pub concepts: Vec<Uuid>,
}

impl CaseLawReference {
    /// Creates a new case law reference.
    pub fn new(
        case_id: String,
        case_name: String,
        court: String,
        jurisdiction: String,
        citation: String,
    ) -> Self {
        Self {
            case_id,
            case_name,
            court,
            jurisdiction,
            decision_date: Utc::now(),
            citation,
            summary: String::new(),
            principles: Vec::new(),
            concepts: Vec::new(),
        }
    }

    /// Sets the decision date.
    pub fn with_decision_date(mut self, date: DateTime<Utc>) -> Self {
        self.decision_date = date;
        self
    }

    /// Sets the summary.
    pub fn with_summary(mut self, summary: String) -> Self {
        self.summary = summary;
        self
    }

    /// Adds legal principles.
    pub fn with_principles(mut self, principles: Vec<String>) -> Self {
        self.principles = principles;
        self
    }

    /// Adds related concepts.
    pub fn with_concepts(mut self, concepts: Vec<Uuid>) -> Self {
        self.concepts = concepts;
        self
    }
}

/// Link between statute and case law.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StatuteCaseLink {
    /// Link identifier
    pub link_id: Uuid,
    /// Statute ID
    pub statute_id: String,
    /// Case ID
    pub case_id: String,
    /// Link type
    pub link_type: CaseLinkType,
    /// Relevance score (0.0-1.0)
    pub relevance_score: f64,
    /// Description of how the case relates to the statute
    pub description: String,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
}

impl StatuteCaseLink {
    /// Creates a new statute-case link.
    pub fn new(
        statute_id: String,
        case_id: String,
        link_type: CaseLinkType,
        relevance_score: f64,
    ) -> Self {
        Self {
            link_id: Uuid::new_v4(),
            statute_id,
            case_id,
            link_type,
            relevance_score: relevance_score.clamp(0.0, 1.0),
            description: String::new(),
            created_at: Utc::now(),
        }
    }

    /// Sets the description.
    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }
}

/// Type of case law link.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CaseLinkType {
    /// Case interprets the statute
    Interprets,
    /// Case applies the statute
    Applies,
    /// Case challenges the statute
    Challenges,
    /// Case upholds the statute
    Upholds,
    /// Case distinguishes the statute
    Distinguishes,
}

// ============================================================================
// Knowledge Graph
// ============================================================================

/// Node in the knowledge graph.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum KnowledgeNode {
    /// Statute node
    Statute {
        statute_id: String,
        title: String,
        jurisdiction: String,
    },
    /// Concept node
    Concept {
        concept_id: Uuid,
        name: String,
        category: String,
    },
    /// Case law node
    Case {
        case_id: String,
        case_name: String,
        citation: String,
    },
}

impl KnowledgeNode {
    /// Gets the node identifier.
    pub fn id(&self) -> String {
        match self {
            KnowledgeNode::Statute { statute_id, .. } => format!("statute:{}", statute_id),
            KnowledgeNode::Concept { concept_id, .. } => format!("concept:{}", concept_id),
            KnowledgeNode::Case { case_id, .. } => format!("case:{}", case_id),
        }
    }

    /// Gets the node display name.
    pub fn display_name(&self) -> &str {
        match self {
            KnowledgeNode::Statute { title, .. } => title,
            KnowledgeNode::Concept { name, .. } => name,
            KnowledgeNode::Case { case_name, .. } => case_name,
        }
    }
}

/// Edge in the knowledge graph.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KnowledgeEdge {
    /// Edge identifier
    pub edge_id: Uuid,
    /// Source node
    pub source: KnowledgeNode,
    /// Target node
    pub target: KnowledgeNode,
    /// Edge type
    pub edge_type: EdgeType,
    /// Edge weight/strength (0.0-1.0)
    pub weight: f64,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl KnowledgeEdge {
    /// Creates a new knowledge graph edge.
    pub fn new(source: KnowledgeNode, target: KnowledgeNode, edge_type: EdgeType) -> Self {
        Self {
            edge_id: Uuid::new_v4(),
            source,
            target,
            edge_type,
            weight: 1.0,
            metadata: HashMap::new(),
        }
    }

    /// Sets the edge weight.
    pub fn with_weight(mut self, weight: f64) -> Self {
        self.weight = weight.clamp(0.0, 1.0);
        self
    }

    /// Adds metadata.
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// Type of edge in the knowledge graph.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EdgeType {
    /// References relationship
    References,
    /// Defines relationship
    Defines,
    /// Applies relationship
    Applies,
    /// Interprets relationship
    Interprets,
    /// Amends relationship
    Amends,
    /// Supersedes relationship
    Supersedes,
    /// Related to relationship
    RelatedTo,
}

/// Knowledge graph for visualizing legal relationships.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeGraph {
    /// Graph identifier
    pub graph_id: Uuid,
    /// Graph name
    pub name: String,
    /// All nodes in the graph
    pub nodes: HashMap<String, KnowledgeNode>,
    /// All edges in the graph
    pub edges: Vec<KnowledgeEdge>,
    /// Adjacency list for efficient traversal (node_id -> connected node_ids)
    pub adjacency: HashMap<String, Vec<String>>,
}

impl KnowledgeGraph {
    /// Creates a new knowledge graph.
    pub fn new(name: String) -> Self {
        Self {
            graph_id: Uuid::new_v4(),
            name,
            nodes: HashMap::new(),
            edges: Vec::new(),
            adjacency: HashMap::new(),
        }
    }

    /// Adds a node to the graph.
    pub fn add_node(&mut self, node: KnowledgeNode) {
        let node_id = node.id();
        self.nodes.insert(node_id.clone(), node);
        self.adjacency.entry(node_id).or_default();
    }

    /// Adds an edge to the graph.
    pub fn add_edge(&mut self, edge: KnowledgeEdge) {
        let source_id = edge.source.id();
        let target_id = edge.target.id();

        // Ensure nodes exist
        self.add_node(edge.source.clone());
        self.add_node(edge.target.clone());

        // Update adjacency list
        self.adjacency
            .entry(source_id.clone())
            .or_default()
            .push(target_id.clone());

        self.edges.push(edge);
    }

    /// Gets all neighbors of a node.
    pub fn get_neighbors(&self, node_id: &str) -> Vec<&KnowledgeNode> {
        self.adjacency
            .get(node_id)
            .map(|ids| ids.iter().filter_map(|id| self.nodes.get(id)).collect())
            .unwrap_or_default()
    }

    /// Finds shortest path between two nodes (BFS).
    pub fn shortest_path(&self, start_id: &str, end_id: &str) -> Option<Vec<String>> {
        use std::collections::VecDeque;

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut parent: HashMap<String, String> = HashMap::new();

        queue.push_back(start_id.to_string());
        visited.insert(start_id.to_string());

        while let Some(current) = queue.pop_front() {
            if current == end_id {
                // Reconstruct path
                let mut path = vec![current.clone()];
                let mut node = &current;

                while let Some(p) = parent.get(node) {
                    path.push(p.clone());
                    node = p;
                }

                path.reverse();
                return Some(path);
            }

            if let Some(neighbors) = self.adjacency.get(&current) {
                for neighbor in neighbors {
                    if visited.insert(neighbor.clone()) {
                        parent.insert(neighbor.clone(), current.clone());
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }

        None
    }

    /// Gets subgraph centered on a node with given depth.
    pub fn get_subgraph(&self, center_id: &str, depth: usize) -> KnowledgeGraph {
        let mut subgraph = KnowledgeGraph::new(format!("{} (subgraph)", self.name));
        let mut visited = HashSet::new();
        let mut current_level = vec![center_id.to_string()];

        for _ in 0..=depth {
            let mut next_level = Vec::new();

            for node_id in &current_level {
                if visited.insert(node_id.clone())
                    && let Some(node) = self.nodes.get(node_id)
                {
                    subgraph.add_node(node.clone());

                    // Add edges from this node
                    for edge in &self.edges {
                        if edge.source.id() == *node_id {
                            subgraph.add_edge(edge.clone());
                            next_level.push(edge.target.id());
                        }
                    }
                }
            }

            if next_level.is_empty() {
                break;
            }

            current_level = next_level;
        }

        subgraph
    }

    /// Exports graph to DOT format for visualization.
    pub fn to_dot(&self) -> String {
        let mut dot = String::from("digraph KnowledgeGraph {\n");
        dot.push_str("  rankdir=LR;\n");
        dot.push_str("  node [shape=box];\n\n");

        // Add nodes with styling based on type
        for (id, node) in &self.nodes {
            let (shape, color) = match node {
                KnowledgeNode::Statute { .. } => ("box", "lightblue"),
                KnowledgeNode::Concept { .. } => ("ellipse", "lightgreen"),
                KnowledgeNode::Case { .. } => ("diamond", "lightyellow"),
            };

            dot.push_str(&format!(
                "  \"{}\" [label=\"{}\", shape={}, fillcolor={}, style=filled];\n",
                id,
                node.display_name().replace('\"', "\\\""),
                shape,
                color
            ));
        }

        dot.push('\n');

        // Add edges with labels
        for edge in &self.edges {
            dot.push_str(&format!(
                "  \"{}\" -> \"{}\" [label=\"{:?}\", weight={}];\n",
                edge.source.id(),
                edge.target.id(),
                edge.edge_type,
                edge.weight
            ));
        }

        dot.push_str("}\n");
        dot
    }
}

// ============================================================================
// AI-Powered Legal Research
// ============================================================================

/// Query for AI-powered legal research.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalResearchQuery {
    /// Query text
    pub query: String,
    /// Jurisdictions to search
    pub jurisdictions: Vec<String>,
    /// Concept filters
    pub concepts: Vec<Uuid>,
    /// Include case law
    pub include_cases: bool,
    /// Maximum results
    pub max_results: usize,
    /// Minimum relevance score
    pub min_relevance: f64,
}

impl LegalResearchQuery {
    /// Creates a new legal research query.
    pub fn new(query: String) -> Self {
        Self {
            query,
            jurisdictions: Vec::new(),
            concepts: Vec::new(),
            include_cases: true,
            max_results: 20,
            min_relevance: 0.5,
        }
    }

    /// Adds jurisdictions to filter.
    pub fn with_jurisdictions(mut self, jurisdictions: Vec<String>) -> Self {
        self.jurisdictions = jurisdictions;
        self
    }

    /// Adds concept filters.
    pub fn with_concepts(mut self, concepts: Vec<Uuid>) -> Self {
        self.concepts = concepts;
        self
    }

    /// Sets maximum results.
    pub fn with_max_results(mut self, max_results: usize) -> Self {
        self.max_results = max_results;
        self
    }
}

/// Result from AI-powered legal research.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalResearchResult {
    /// Matched statutes with relevance scores
    pub statutes: Vec<(String, f64)>,
    /// Matched concepts with relevance scores
    pub concepts: Vec<(Uuid, f64)>,
    /// Matched cases with relevance scores
    pub cases: Vec<(String, f64)>,
    /// Suggested related queries
    pub related_queries: Vec<String>,
    /// Query understanding insights
    pub insights: Vec<String>,
}

impl LegalResearchResult {
    /// Creates an empty result.
    pub fn empty() -> Self {
        Self {
            statutes: Vec::new(),
            concepts: Vec::new(),
            cases: Vec::new(),
            related_queries: Vec::new(),
            insights: Vec::new(),
        }
    }

    /// Returns total number of results.
    pub fn total_results(&self) -> usize {
        self.statutes.len() + self.concepts.len() + self.cases.len()
    }

    /// Checks if the result has any matches.
    pub fn has_results(&self) -> bool {
        self.total_results() > 0
    }
}

// ============================================================================
// Knowledge Base Manager
// ============================================================================

/// Manager for the legal knowledge base.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeBaseManager {
    /// All ontologies
    pub ontologies: HashMap<Uuid, LegalOntology>,
    /// Statute-concept links
    pub statute_concept_links: Vec<StatuteConceptLink>,
    /// Case law references
    pub case_references: HashMap<String, CaseLawReference>,
    /// Statute-case links
    pub statute_case_links: Vec<StatuteCaseLink>,
    /// Knowledge graphs
    pub graphs: HashMap<Uuid, KnowledgeGraph>,
}

impl Default for KnowledgeBaseManager {
    fn default() -> Self {
        Self::new()
    }
}

impl KnowledgeBaseManager {
    /// Creates a new knowledge base manager.
    pub fn new() -> Self {
        Self {
            ontologies: HashMap::new(),
            statute_concept_links: Vec::new(),
            case_references: HashMap::new(),
            statute_case_links: Vec::new(),
            graphs: HashMap::new(),
        }
    }

    /// Adds an ontology.
    pub fn add_ontology(&mut self, ontology: LegalOntology) -> Uuid {
        let ontology_id = ontology.ontology_id;
        self.ontologies.insert(ontology_id, ontology);
        ontology_id
    }

    /// Links a statute to a concept.
    pub fn link_statute_to_concept(&mut self, link: StatuteConceptLink) {
        self.statute_concept_links.push(link);
    }

    /// Gets all concepts linked to a statute.
    pub fn get_statute_concepts(&self, statute_id: &str) -> Vec<&StatuteConceptLink> {
        self.statute_concept_links
            .iter()
            .filter(|link| link.statute_id == statute_id)
            .collect()
    }

    /// Adds a case law reference.
    pub fn add_case_reference(&mut self, case: CaseLawReference) {
        self.case_references.insert(case.case_id.clone(), case);
    }

    /// Links a statute to a case.
    pub fn link_statute_to_case(&mut self, link: StatuteCaseLink) {
        self.statute_case_links.push(link);
    }

    /// Gets all cases linked to a statute.
    pub fn get_statute_cases(&self, statute_id: &str) -> Vec<&StatuteCaseLink> {
        self.statute_case_links
            .iter()
            .filter(|link| link.statute_id == statute_id)
            .collect()
    }

    /// Creates a knowledge graph from statute and its relationships.
    pub fn build_statute_graph(
        &self,
        statute_id: &str,
        statute_entry: &StatuteEntry,
    ) -> KnowledgeGraph {
        let mut graph = KnowledgeGraph::new(format!("Graph for {}", statute_id));

        // Add statute node
        let statute_node = KnowledgeNode::Statute {
            statute_id: statute_id.to_string(),
            title: statute_entry.statute.title.clone(),
            jurisdiction: statute_entry
                .statute
                .jurisdiction
                .clone()
                .unwrap_or_default(),
        };
        graph.add_node(statute_node.clone());

        // Add concept links
        for link in self.get_statute_concepts(statute_id) {
            if let Some(ontology) = self.ontologies.values().next()
                && let Some(concept) = ontology.concepts.get(&link.concept_id)
            {
                let concept_node = KnowledgeNode::Concept {
                    concept_id: concept.concept_id,
                    name: concept.name.clone(),
                    category: concept.category.clone(),
                };

                let edge_type = match link.link_type {
                    ConceptLinkType::Defines => EdgeType::Defines,
                    ConceptLinkType::Applies => EdgeType::Applies,
                    ConceptLinkType::References => EdgeType::References,
                    ConceptLinkType::Modifies => EdgeType::RelatedTo,
                    ConceptLinkType::Exemplifies => EdgeType::RelatedTo,
                };

                let edge = KnowledgeEdge::new(statute_node.clone(), concept_node, edge_type)
                    .with_weight(link.relevance_score);

                graph.add_edge(edge);
            }
        }

        // Add case links
        for link in self.get_statute_cases(statute_id) {
            if let Some(case) = self.case_references.get(&link.case_id) {
                let case_node = KnowledgeNode::Case {
                    case_id: case.case_id.clone(),
                    case_name: case.case_name.clone(),
                    citation: case.citation.clone(),
                };

                let edge_type = match link.link_type {
                    CaseLinkType::Interprets => EdgeType::Interprets,
                    CaseLinkType::Applies => EdgeType::Applies,
                    _ => EdgeType::RelatedTo,
                };

                let edge = KnowledgeEdge::new(statute_node.clone(), case_node, edge_type)
                    .with_weight(link.relevance_score);

                graph.add_edge(edge);
            }
        }

        graph
    }

    /// Performs AI-powered legal research.
    pub fn research(&self, query: &LegalResearchQuery) -> LegalResearchResult {
        let mut result = LegalResearchResult::empty();

        // Simple keyword-based matching for statutes
        // In a real implementation, this would use embeddings and semantic search
        let query_lower = query.query.to_lowercase();

        // Search concepts
        for ontology in self.ontologies.values() {
            for concept in ontology.concepts.values() {
                let name_match = concept.name.to_lowercase().contains(&query_lower);
                let desc_match = concept.description.to_lowercase().contains(&query_lower);

                if name_match || desc_match {
                    let score = if name_match { 0.9 } else { 0.7 };
                    if score >= query.min_relevance {
                        result.concepts.push((concept.concept_id, score));
                    }
                }
            }
        }

        // Search cases if requested
        if query.include_cases {
            for case in self.case_references.values() {
                let name_match = case.case_name.to_lowercase().contains(&query_lower);
                let summary_match = case.summary.to_lowercase().contains(&query_lower);

                if name_match || summary_match {
                    let score = if name_match { 0.9 } else { 0.7 };
                    if score >= query.min_relevance {
                        result.cases.push((case.case_id.clone(), score));
                    }
                }
            }
        }

        // Generate insights
        if !result.concepts.is_empty() {
            result.insights.push(format!(
                "Found {} relevant legal concepts",
                result.concepts.len()
            ));
        }

        if !result.cases.is_empty() {
            result.insights.push(format!(
                "Found {} relevant case law references",
                result.cases.len()
            ));
        }

        // Generate related queries
        if !result.concepts.is_empty()
            && let Some(ontology) = self.ontologies.values().next()
        {
            for &(concept_id, _) in result.concepts.iter().take(3) {
                if let Some(concept) = ontology.concepts.get(&concept_id) {
                    result
                        .related_queries
                        .push(format!("Related to: {}", concept.name));
                }
            }
        }

        // Limit results
        result.statutes.truncate(query.max_results);
        result.concepts.truncate(query.max_results);
        result.cases.truncate(query.max_results);

        result
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legal_concept_creation() {
        let concept = LegalConcept::new(
            "Contract".to_string(),
            "A legally binding agreement".to_string(),
            "contract law".to_string(),
        );

        assert_eq!(concept.name, "Contract");
        assert_eq!(concept.description, "A legally binding agreement");
        assert_eq!(concept.category, "contract law");
        assert!(concept.parent_concept_id.is_none());
        assert!(concept.related_concepts.is_empty());
    }

    #[test]
    fn test_legal_concept_with_parent() {
        let parent_id = Uuid::new_v4();
        let concept = LegalConcept::new(
            "Sales Contract".to_string(),
            "A contract for the sale of goods".to_string(),
            "contract law".to_string(),
        )
        .with_parent(parent_id);

        assert_eq!(concept.parent_concept_id, Some(parent_id));
    }

    #[test]
    fn test_legal_concept_add_synonym() {
        let mut concept = LegalConcept::new(
            "Agreement".to_string(),
            "A mutual understanding".to_string(),
            "contract law".to_string(),
        );

        concept.add_synonym("Contract".to_string());
        concept.add_synonym("Pact".to_string());

        assert_eq!(concept.synonyms.len(), 2);
        assert!(concept.synonyms.contains(&"Contract".to_string()));
        assert!(concept.synonyms.contains(&"Pact".to_string()));
    }

    #[test]
    fn test_statute_concept_link_creation() {
        let link = StatuteConceptLink::new(
            "BGB-433".to_string(),
            Uuid::new_v4(),
            ConceptLinkType::Defines,
            0.95,
        );

        assert_eq!(link.statute_id, "BGB-433");
        assert_eq!(link.link_type, ConceptLinkType::Defines);
        assert_eq!(link.relevance_score, 0.95);
    }

    #[test]
    fn test_statute_concept_link_with_sections() {
        let link = StatuteConceptLink::new(
            "BGB-433".to_string(),
            Uuid::new_v4(),
            ConceptLinkType::Defines,
            0.95,
        )
        .with_sections(vec!["§1".to_string(), "§2".to_string()]);

        assert_eq!(link.sections.len(), 2);
    }

    #[test]
    fn test_legal_ontology_creation() {
        let ontology = LegalOntology::new(
            "German Civil Law".to_string(),
            "1.0.0".to_string(),
            "DE".to_string(),
        );

        assert_eq!(ontology.name, "German Civil Law");
        assert_eq!(ontology.version, "1.0.0");
        assert_eq!(ontology.jurisdiction, "DE");
        assert!(ontology.concepts.is_empty());
    }

    #[test]
    fn test_legal_ontology_add_concept() {
        let mut ontology = LegalOntology::new(
            "Test Ontology".to_string(),
            "1.0".to_string(),
            "US".to_string(),
        );

        let concept = LegalConcept::new(
            "Contract".to_string(),
            "A binding agreement".to_string(),
            "contract law".to_string(),
        );

        let concept_id = ontology.add_concept(concept);

        assert_eq!(ontology.concepts.len(), 1);
        assert!(ontology.concepts.contains_key(&concept_id));
    }

    #[test]
    fn test_legal_ontology_find_concepts_by_term() {
        let mut ontology = LegalOntology::new(
            "Test Ontology".to_string(),
            "1.0".to_string(),
            "US".to_string(),
        );

        let mut contract_concept = LegalConcept::new(
            "Contract".to_string(),
            "A binding agreement".to_string(),
            "contract law".to_string(),
        );
        contract_concept.add_synonym("Agreement".to_string());

        ontology.add_concept(contract_concept);

        let results = ontology.find_concepts_by_term("contract");
        assert_eq!(results.len(), 1);

        let results = ontology.find_concepts_by_term("agreement");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_legal_ontology_hierarchy() {
        let mut ontology = LegalOntology::new(
            "Test Ontology".to_string(),
            "1.0".to_string(),
            "US".to_string(),
        );

        let parent_concept = LegalConcept::new(
            "Contract".to_string(),
            "A binding agreement".to_string(),
            "contract law".to_string(),
        );
        let parent_id = ontology.add_concept(parent_concept);

        let child_concept = LegalConcept::new(
            "Sales Contract".to_string(),
            "A contract for sale of goods".to_string(),
            "contract law".to_string(),
        )
        .with_parent(parent_id);
        let child_id = ontology.add_concept(child_concept);

        let path = ontology.get_concept_path(child_id);
        assert_eq!(path.len(), 2);
        assert_eq!(path[0], parent_id);
        assert_eq!(path[1], child_id);
    }

    #[test]
    fn test_legal_ontology_descendants() {
        let mut ontology = LegalOntology::new(
            "Test Ontology".to_string(),
            "1.0".to_string(),
            "US".to_string(),
        );

        let parent_concept = LegalConcept::new(
            "Contract".to_string(),
            "A binding agreement".to_string(),
            "contract law".to_string(),
        );
        let parent_id = ontology.add_concept(parent_concept);

        let child1 = LegalConcept::new(
            "Sales Contract".to_string(),
            "Sale of goods".to_string(),
            "contract law".to_string(),
        )
        .with_parent(parent_id);
        ontology.add_concept(child1);

        let child2 = LegalConcept::new(
            "Service Contract".to_string(),
            "Service provision".to_string(),
            "contract law".to_string(),
        )
        .with_parent(parent_id);
        ontology.add_concept(child2);

        let descendants = ontology.get_descendants(parent_id);
        assert_eq!(descendants.len(), 2);
    }

    #[test]
    fn test_case_law_reference_creation() {
        let case_ref = CaseLawReference::new(
            "case-001".to_string(),
            "Smith v. Jones".to_string(),
            "Supreme Court".to_string(),
            "US".to_string(),
            "123 U.S. 456".to_string(),
        );

        assert_eq!(case_ref.case_id, "case-001");
        assert_eq!(case_ref.case_name, "Smith v. Jones");
        assert_eq!(case_ref.court, "Supreme Court");
        assert_eq!(case_ref.citation, "123 U.S. 456");
    }

    #[test]
    fn test_case_law_reference_with_details() {
        let case_ref = CaseLawReference::new(
            "case-001".to_string(),
            "Smith v. Jones".to_string(),
            "Supreme Court".to_string(),
            "US".to_string(),
            "123 U.S. 456".to_string(),
        )
        .with_summary("Important precedent".to_string())
        .with_principles(vec!["Freedom of contract".to_string()]);

        assert_eq!(case_ref.summary, "Important precedent");
        assert_eq!(case_ref.principles.len(), 1);
    }

    #[test]
    fn test_statute_case_link_creation() {
        let link = StatuteCaseLink::new(
            "BGB-433".to_string(),
            "case-001".to_string(),
            CaseLinkType::Interprets,
            0.9,
        );

        assert_eq!(link.statute_id, "BGB-433");
        assert_eq!(link.case_id, "case-001");
        assert_eq!(link.link_type, CaseLinkType::Interprets);
        assert_eq!(link.relevance_score, 0.9);
    }

    #[test]
    fn test_knowledge_node_creation() {
        let statute_node = KnowledgeNode::Statute {
            statute_id: "BGB-433".to_string(),
            title: "Purchase Contract".to_string(),
            jurisdiction: "DE".to_string(),
        };

        assert_eq!(statute_node.id(), "statute:BGB-433");
        assert_eq!(statute_node.display_name(), "Purchase Contract");
    }

    #[test]
    fn test_knowledge_edge_creation() {
        let source = KnowledgeNode::Statute {
            statute_id: "BGB-433".to_string(),
            title: "Purchase Contract".to_string(),
            jurisdiction: "DE".to_string(),
        };

        let target = KnowledgeNode::Concept {
            concept_id: Uuid::new_v4(),
            name: "Contract".to_string(),
            category: "contract law".to_string(),
        };

        let edge = KnowledgeEdge::new(source, target, EdgeType::Defines).with_weight(0.95);

        assert_eq!(edge.edge_type, EdgeType::Defines);
        assert_eq!(edge.weight, 0.95);
    }

    #[test]
    fn test_knowledge_graph_creation() {
        let mut graph = KnowledgeGraph::new("Test Graph".to_string());

        let node = KnowledgeNode::Statute {
            statute_id: "BGB-433".to_string(),
            title: "Purchase Contract".to_string(),
            jurisdiction: "DE".to_string(),
        };

        graph.add_node(node);

        assert_eq!(graph.nodes.len(), 1);
        assert!(graph.nodes.contains_key("statute:BGB-433"));
    }

    #[test]
    fn test_knowledge_graph_add_edge() {
        let mut graph = KnowledgeGraph::new("Test Graph".to_string());

        let statute_node = KnowledgeNode::Statute {
            statute_id: "BGB-433".to_string(),
            title: "Purchase Contract".to_string(),
            jurisdiction: "DE".to_string(),
        };

        let concept_node = KnowledgeNode::Concept {
            concept_id: Uuid::new_v4(),
            name: "Contract".to_string(),
            category: "contract law".to_string(),
        };

        let edge = KnowledgeEdge::new(statute_node, concept_node, EdgeType::Defines);

        graph.add_edge(edge);

        assert_eq!(graph.edges.len(), 1);
        assert_eq!(graph.nodes.len(), 2);
    }

    #[test]
    fn test_knowledge_graph_get_neighbors() {
        let mut graph = KnowledgeGraph::new("Test Graph".to_string());

        let statute_node = KnowledgeNode::Statute {
            statute_id: "BGB-433".to_string(),
            title: "Purchase Contract".to_string(),
            jurisdiction: "DE".to_string(),
        };

        let concept_node = KnowledgeNode::Concept {
            concept_id: Uuid::new_v4(),
            name: "Contract".to_string(),
            category: "contract law".to_string(),
        };

        let edge = KnowledgeEdge::new(statute_node.clone(), concept_node, EdgeType::Defines);

        graph.add_edge(edge);

        let neighbors = graph.get_neighbors(&statute_node.id());
        assert_eq!(neighbors.len(), 1);
    }

    #[test]
    fn test_knowledge_graph_shortest_path() {
        let mut graph = KnowledgeGraph::new("Test Graph".to_string());

        let node1 = KnowledgeNode::Statute {
            statute_id: "A".to_string(),
            title: "Statute A".to_string(),
            jurisdiction: "DE".to_string(),
        };

        let node2 = KnowledgeNode::Concept {
            concept_id: Uuid::new_v4(),
            name: "B".to_string(),
            category: "law".to_string(),
        };

        let node3 = KnowledgeNode::Case {
            case_id: "C".to_string(),
            case_name: "Case C".to_string(),
            citation: "123".to_string(),
        };

        graph.add_edge(KnowledgeEdge::new(
            node1.clone(),
            node2.clone(),
            EdgeType::Defines,
        ));
        graph.add_edge(KnowledgeEdge::new(node2, node3.clone(), EdgeType::Applies));

        let path = graph.shortest_path(&node1.id(), &node3.id());
        assert!(path.is_some());
        assert_eq!(path.unwrap().len(), 3);
    }

    #[test]
    fn test_knowledge_graph_to_dot() {
        let mut graph = KnowledgeGraph::new("Test Graph".to_string());

        let statute_node = KnowledgeNode::Statute {
            statute_id: "BGB-433".to_string(),
            title: "Purchase Contract".to_string(),
            jurisdiction: "DE".to_string(),
        };

        graph.add_node(statute_node);

        let dot = graph.to_dot();
        assert!(dot.contains("digraph KnowledgeGraph"));
        assert!(dot.contains("Purchase Contract"));
    }

    #[test]
    fn test_legal_research_query_creation() {
        let query = LegalResearchQuery::new("contract law".to_string())
            .with_jurisdictions(vec!["DE".to_string(), "US".to_string()])
            .with_max_results(10);

        assert_eq!(query.query, "contract law");
        assert_eq!(query.jurisdictions.len(), 2);
        assert_eq!(query.max_results, 10);
    }

    #[test]
    fn test_legal_research_result_empty() {
        let result = LegalResearchResult::empty();

        assert_eq!(result.total_results(), 0);
        assert!(!result.has_results());
    }

    #[test]
    fn test_knowledge_base_manager_creation() {
        let manager = KnowledgeBaseManager::new();

        assert!(manager.ontologies.is_empty());
        assert!(manager.statute_concept_links.is_empty());
        assert!(manager.case_references.is_empty());
        assert!(manager.statute_case_links.is_empty());
    }

    #[test]
    fn test_knowledge_base_manager_add_ontology() {
        let mut manager = KnowledgeBaseManager::new();

        let ontology = LegalOntology::new(
            "Test Ontology".to_string(),
            "1.0".to_string(),
            "US".to_string(),
        );

        let ontology_id = manager.add_ontology(ontology);

        assert_eq!(manager.ontologies.len(), 1);
        assert!(manager.ontologies.contains_key(&ontology_id));
    }

    #[test]
    fn test_knowledge_base_manager_link_statute_to_concept() {
        let mut manager = KnowledgeBaseManager::new();

        let link = StatuteConceptLink::new(
            "BGB-433".to_string(),
            Uuid::new_v4(),
            ConceptLinkType::Defines,
            0.95,
        );

        manager.link_statute_to_concept(link);

        assert_eq!(manager.statute_concept_links.len(), 1);
    }

    #[test]
    fn test_knowledge_base_manager_get_statute_concepts() {
        let mut manager = KnowledgeBaseManager::new();

        let link1 = StatuteConceptLink::new(
            "BGB-433".to_string(),
            Uuid::new_v4(),
            ConceptLinkType::Defines,
            0.95,
        );

        let link2 = StatuteConceptLink::new(
            "BGB-434".to_string(),
            Uuid::new_v4(),
            ConceptLinkType::Applies,
            0.85,
        );

        manager.link_statute_to_concept(link1);
        manager.link_statute_to_concept(link2);

        let concepts = manager.get_statute_concepts("BGB-433");
        assert_eq!(concepts.len(), 1);
    }

    #[test]
    fn test_knowledge_base_manager_add_case_reference() {
        let mut manager = KnowledgeBaseManager::new();

        let case_ref = CaseLawReference::new(
            "case-001".to_string(),
            "Smith v. Jones".to_string(),
            "Supreme Court".to_string(),
            "US".to_string(),
            "123 U.S. 456".to_string(),
        );

        manager.add_case_reference(case_ref);

        assert_eq!(manager.case_references.len(), 1);
        assert!(manager.case_references.contains_key("case-001"));
    }

    #[test]
    fn test_knowledge_base_manager_link_statute_to_case() {
        let mut manager = KnowledgeBaseManager::new();

        let link = StatuteCaseLink::new(
            "BGB-433".to_string(),
            "case-001".to_string(),
            CaseLinkType::Interprets,
            0.9,
        );

        manager.link_statute_to_case(link);

        assert_eq!(manager.statute_case_links.len(), 1);
    }

    #[test]
    fn test_knowledge_base_manager_get_statute_cases() {
        let mut manager = KnowledgeBaseManager::new();

        let link1 = StatuteCaseLink::new(
            "BGB-433".to_string(),
            "case-001".to_string(),
            CaseLinkType::Interprets,
            0.9,
        );

        let link2 = StatuteCaseLink::new(
            "BGB-434".to_string(),
            "case-002".to_string(),
            CaseLinkType::Applies,
            0.85,
        );

        manager.link_statute_to_case(link1);
        manager.link_statute_to_case(link2);

        let cases = manager.get_statute_cases("BGB-433");
        assert_eq!(cases.len(), 1);
    }

    #[test]
    fn test_knowledge_base_manager_research() {
        let mut manager = KnowledgeBaseManager::new();

        // Add ontology with concepts
        let mut ontology = LegalOntology::new(
            "Test Ontology".to_string(),
            "1.0".to_string(),
            "US".to_string(),
        );

        let concept = LegalConcept::new(
            "Contract Law".to_string(),
            "Legal rules governing contracts".to_string(),
            "law".to_string(),
        );

        ontology.add_concept(concept);
        manager.add_ontology(ontology);

        // Add case reference
        let case_ref = CaseLawReference::new(
            "case-001".to_string(),
            "Contract Dispute Case".to_string(),
            "Supreme Court".to_string(),
            "US".to_string(),
            "123 U.S. 456".to_string(),
        )
        .with_summary("Important contract law case".to_string());

        manager.add_case_reference(case_ref);

        // Perform research
        let query = LegalResearchQuery::new("contract".to_string()).with_max_results(10);

        let result = manager.research(&query);

        assert!(result.has_results());
        assert!(!result.concepts.is_empty() || !result.cases.is_empty());
    }
}
