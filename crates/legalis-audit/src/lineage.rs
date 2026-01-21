//! Decision lineage visualization and tracking.
//!
//! This module provides functionality for tracking decision lineage,
//! showing relationships between decisions, and generating visual
//! representations of decision flows.

use crate::{AuditRecord, AuditResult};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

/// Represents a node in the decision lineage graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageNode {
    /// Record ID
    pub id: Uuid,
    /// Node type
    pub node_type: LineageNodeType,
    /// Statute ID
    pub statute_id: String,
    /// Subject ID
    pub subject_id: Uuid,
    /// Timestamp
    pub timestamp: String,
    /// Actor description
    pub actor: String,
    /// Result description
    pub result: String,
}

/// Type of lineage node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LineageNodeType {
    /// Original decision
    Original,
    /// Override of another decision
    Override,
    /// Appeal or review
    Appeal,
    /// Modification
    Modification,
}

/// Represents an edge connecting two nodes in the lineage graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageEdge {
    /// Source node ID
    pub from: Uuid,
    /// Target node ID
    pub to: Uuid,
    /// Relationship type
    pub relationship: EdgeRelationship,
    /// Description
    pub description: Option<String>,
}

/// Type of relationship between nodes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EdgeRelationship {
    /// One decision overrides another
    Overrides,
    /// One decision appeals another
    Appeals,
    /// One decision modifies another
    Modifies,
    /// Temporal precedence (happened after)
    Precedes,
    /// Related by same subject
    SameSubject,
    /// Related by same statute
    SameStatute,
}

/// Decision lineage graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageGraph {
    /// All nodes in the graph
    pub nodes: Vec<LineageNode>,
    /// All edges in the graph
    pub edges: Vec<LineageEdge>,
    /// Metadata
    pub metadata: HashMap<String, String>,
}

impl LineageGraph {
    /// Creates a new empty lineage graph.
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Adds a node to the graph.
    pub fn add_node(&mut self, node: LineageNode) {
        self.nodes.push(node);
    }

    /// Adds an edge to the graph.
    pub fn add_edge(&mut self, edge: LineageEdge) {
        self.edges.push(edge);
    }

    /// Finds all descendants of a given node.
    pub fn find_descendants(&self, node_id: Uuid) -> Vec<&LineageNode> {
        let mut descendants = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = vec![node_id];

        while let Some(current) = queue.pop() {
            if visited.contains(&current) {
                continue;
            }
            visited.insert(current);

            for edge in &self.edges {
                if edge.from == current
                    && let Some(node) = self.nodes.iter().find(|n| n.id == edge.to)
                {
                    descendants.push(node);
                    queue.push(edge.to);
                }
            }
        }

        descendants
    }

    /// Finds all ancestors of a given node.
    pub fn find_ancestors(&self, node_id: Uuid) -> Vec<&LineageNode> {
        let mut ancestors = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = vec![node_id];

        while let Some(current) = queue.pop() {
            if visited.contains(&current) {
                continue;
            }
            visited.insert(current);

            for edge in &self.edges {
                if edge.to == current
                    && let Some(node) = self.nodes.iter().find(|n| n.id == edge.from)
                {
                    ancestors.push(node);
                    queue.push(edge.from);
                }
            }
        }

        ancestors
    }

    /// Generates a Graphviz DOT representation of the lineage graph.
    pub fn to_dot(&self) -> String {
        let mut dot = String::from("digraph DecisionLineage {\n");
        dot.push_str("  rankdir=TB;\n");
        dot.push_str("  node [shape=box, style=rounded];\n\n");

        // Add nodes
        for node in &self.nodes {
            let color = match node.node_type {
                LineageNodeType::Original => "lightblue",
                LineageNodeType::Override => "orange",
                LineageNodeType::Appeal => "yellow",
                LineageNodeType::Modification => "lightgreen",
            };

            let label = format!(
                "{}\n{}\n{}\n{}",
                node.statute_id, node.actor, node.result, node.timestamp
            );

            dot.push_str(&format!(
                "  \"{}\" [label=\"{}\", fillcolor={}, style=filled];\n",
                node.id, label, color
            ));
        }

        dot.push('\n');

        // Add edges
        for edge in &self.edges {
            let (color, style) = match edge.relationship {
                EdgeRelationship::Overrides => ("red", "solid"),
                EdgeRelationship::Appeals => ("blue", "dashed"),
                EdgeRelationship::Modifies => ("green", "dotted"),
                EdgeRelationship::Precedes => ("gray", "solid"),
                EdgeRelationship::SameSubject => ("purple", "dotted"),
                EdgeRelationship::SameStatute => ("brown", "dotted"),
            };

            let label = edge.description.as_deref().unwrap_or("");

            dot.push_str(&format!(
                "  \"{}\" -> \"{}\" [label=\"{}\", color={}, style={}];\n",
                edge.from, edge.to, label, color, style
            ));
        }

        dot.push_str("}\n");
        dot
    }

    /// Generates an SVG representation of the lineage graph (simplified).
    pub fn to_svg(&self) -> String {
        let width = 800;
        let height = 600;
        let node_width = 120;
        let node_height = 60;

        let mut svg = format!(
            r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">"#,
            width, height
        );

        svg.push_str("<style>");
        svg.push_str("text { font-family: sans-serif; font-size: 10px; }");
        svg.push_str(".node { stroke: black; stroke-width: 2; }");
        svg.push_str(".edge { stroke: gray; stroke-width: 1; fill: none; }");
        svg.push_str("</style>");

        // Simple layout: arrange nodes in a grid
        let cols = (self.nodes.len() as f64).sqrt().ceil() as usize;
        let spacing_x = width / (cols + 1);
        let spacing_y = height / ((self.nodes.len() / cols) + 2);

        let mut node_positions = HashMap::new();

        // Draw nodes
        for (i, node) in self.nodes.iter().enumerate() {
            let x = ((i % cols) + 1) * spacing_x;
            let y = ((i / cols) + 1) * spacing_y;
            node_positions.insert(node.id, (x, y));

            let fill = match node.node_type {
                LineageNodeType::Original => "lightblue",
                LineageNodeType::Override => "orange",
                LineageNodeType::Appeal => "yellow",
                LineageNodeType::Modification => "lightgreen",
            };

            svg.push_str(&format!(
                r#"<rect class="node" x="{}" y="{}" width="{}" height="{}" fill="{}"/>"#,
                x - node_width / 2,
                y - node_height / 2,
                node_width,
                node_height,
                fill
            ));

            svg.push_str(&format!(
                r#"<text x="{}" y="{}" text-anchor="middle">{}</text>"#,
                x,
                y - 10,
                &node.statute_id[..node.statute_id.len().min(15)]
            ));
            svg.push_str(&format!(
                r#"<text x="{}" y="{}" text-anchor="middle">{}</text>"#,
                x, y, node.actor
            ));
            svg.push_str(&format!(
                r#"<text x="{}" y="{}" text-anchor="middle">{}</text>"#,
                x,
                y + 10,
                &node.result[..node.result.len().min(15)]
            ));
        }

        // Draw edges
        for edge in &self.edges {
            if let (Some(&(x1, y1)), Some(&(x2, y2))) =
                (node_positions.get(&edge.from), node_positions.get(&edge.to))
            {
                let color = match edge.relationship {
                    EdgeRelationship::Overrides => "red",
                    EdgeRelationship::Appeals => "blue",
                    EdgeRelationship::Modifies => "green",
                    EdgeRelationship::Precedes => "gray",
                    EdgeRelationship::SameSubject => "purple",
                    EdgeRelationship::SameStatute => "brown",
                };

                svg.push_str(&format!(
                    r#"<line class="edge" x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}"/>"#,
                    x1, y1, x2, y2, color
                ));

                // Add arrowhead
                let angle = ((y2 - y1) as f64).atan2((x2 - x1) as f64);
                let arrow_size = 10.0;
                let arrow_x = x2 - (arrow_size * angle.cos()) as usize;
                let arrow_y = y2 - (arrow_size * angle.sin()) as usize;

                svg.push_str(&format!(
                    r#"<polygon points="{},{} {},{} {},{}" fill="{}"/>"#,
                    x2,
                    y2,
                    arrow_x - (arrow_size / 2.0 * angle.sin()) as usize,
                    arrow_y + (arrow_size / 2.0 * angle.cos()) as usize,
                    arrow_x + (arrow_size / 2.0 * angle.sin()) as usize,
                    arrow_y - (arrow_size / 2.0 * angle.cos()) as usize,
                    color
                ));
            }
        }

        svg.push_str("</svg>");
        svg
    }
}

impl Default for LineageGraph {
    fn default() -> Self {
        Self::new()
    }
}

/// Builds a lineage graph from audit records.
pub struct LineageBuilder;

impl LineageBuilder {
    /// Builds a lineage graph from audit records.
    pub fn build(records: &[AuditRecord]) -> AuditResult<LineageGraph> {
        let mut graph = LineageGraph::new();

        // Build nodes
        for record in records {
            let node_type = match &record.event_type {
                crate::EventType::AutomaticDecision | crate::EventType::DiscretionaryReview => {
                    LineageNodeType::Original
                }
                crate::EventType::HumanOverride => LineageNodeType::Override,
                crate::EventType::Appeal => LineageNodeType::Appeal,
                crate::EventType::StatuteModified => LineageNodeType::Modification,
                crate::EventType::SimulationRun => LineageNodeType::Original,
            };

            let actor = match &record.actor {
                crate::Actor::System { component } => format!("System: {}", component),
                crate::Actor::User { user_id, role } => format!("User: {} ({})", user_id, role),
                crate::Actor::External { system_id } => format!("External: {}", system_id),
            };

            let result = match &record.result {
                crate::DecisionResult::Deterministic { effect_applied, .. } => {
                    effect_applied.clone()
                }
                crate::DecisionResult::RequiresDiscretion { issue, .. } => issue.clone(),
                crate::DecisionResult::Void { reason } => format!("Void: {}", reason),
                crate::DecisionResult::Overridden { justification, .. } => {
                    format!("Override: {}", justification)
                }
            };

            graph.add_node(LineageNode {
                id: record.id,
                node_type,
                statute_id: record.statute_id.clone(),
                subject_id: record.subject_id,
                timestamp: record.timestamp.format("%Y-%m-%d %H:%M").to_string(),
                actor,
                result,
            });
        }

        // Build edges based on relationships
        for (i, record_i) in records.iter().enumerate() {
            for (j, record_j) in records.iter().enumerate() {
                if i >= j {
                    continue;
                }

                // Check for override relationship
                if matches!(record_j.event_type, crate::EventType::HumanOverride)
                    && record_i.subject_id == record_j.subject_id
                    && record_i.statute_id == record_j.statute_id
                {
                    graph.add_edge(LineageEdge {
                        from: record_i.id,
                        to: record_j.id,
                        relationship: EdgeRelationship::Overrides,
                        description: Some("Human override".to_string()),
                    });
                }

                // Check for appeal relationship
                if matches!(record_j.event_type, crate::EventType::Appeal)
                    && record_i.subject_id == record_j.subject_id
                {
                    graph.add_edge(LineageEdge {
                        from: record_i.id,
                        to: record_j.id,
                        relationship: EdgeRelationship::Appeals,
                        description: Some("Appeal".to_string()),
                    });
                }

                // Check for same subject relationship (if different statutes)
                if record_i.subject_id == record_j.subject_id
                    && record_i.statute_id != record_j.statute_id
                {
                    graph.add_edge(LineageEdge {
                        from: record_i.id,
                        to: record_j.id,
                        relationship: EdgeRelationship::SameSubject,
                        description: None,
                    });
                }
            }
        }

        graph
            .metadata
            .insert("total_nodes".to_string(), graph.nodes.len().to_string());
        graph
            .metadata
            .insert("total_edges".to_string(), graph.edges.len().to_string());

        Ok(graph)
    }

    /// Builds a lineage graph for a specific subject.
    pub fn build_for_subject(
        records: &[AuditRecord],
        subject_id: Uuid,
    ) -> AuditResult<LineageGraph> {
        let filtered: Vec<_> = records
            .iter()
            .filter(|r| r.subject_id == subject_id)
            .cloned()
            .collect();
        Self::build(&filtered)
    }

    /// Builds a lineage graph for a specific statute.
    pub fn build_for_statute(
        records: &[AuditRecord],
        statute_id: &str,
    ) -> AuditResult<LineageGraph> {
        let filtered: Vec<_> = records
            .iter()
            .filter(|r| r.statute_id == statute_id)
            .cloned()
            .collect();
        Self::build(&filtered)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use std::collections::HashMap;

    #[test]
    fn test_lineage_graph() {
        let mut graph = LineageGraph::new();

        let node1 = LineageNode {
            id: Uuid::new_v4(),
            node_type: LineageNodeType::Original,
            statute_id: "statute-1".to_string(),
            subject_id: Uuid::new_v4(),
            timestamp: "2024-01-01 10:00".to_string(),
            actor: "System".to_string(),
            result: "Approved".to_string(),
        };

        let node2 = LineageNode {
            id: Uuid::new_v4(),
            node_type: LineageNodeType::Override,
            statute_id: "statute-1".to_string(),
            subject_id: node1.subject_id,
            timestamp: "2024-01-01 11:00".to_string(),
            actor: "User: admin".to_string(),
            result: "Rejected".to_string(),
        };

        graph.add_node(node1.clone());
        graph.add_node(node2.clone());

        graph.add_edge(LineageEdge {
            from: node1.id,
            to: node2.id,
            relationship: EdgeRelationship::Overrides,
            description: Some("Manual override".to_string()),
        });

        assert_eq!(graph.nodes.len(), 2);
        assert_eq!(graph.edges.len(), 1);

        let descendants = graph.find_descendants(node1.id);
        assert_eq!(descendants.len(), 1);
        assert_eq!(descendants[0].id, node2.id);

        let ancestors = graph.find_ancestors(node2.id);
        assert_eq!(ancestors.len(), 1);
        assert_eq!(ancestors[0].id, node1.id);
    }

    #[test]
    fn test_lineage_builder() {
        let subject_id = Uuid::new_v4();
        let records = vec![
            AuditRecord::new(
                EventType::AutomaticDecision,
                Actor::System {
                    component: "engine".to_string(),
                },
                "statute-1".to_string(),
                subject_id,
                DecisionContext::default(),
                DecisionResult::Deterministic {
                    effect_applied: "approved".to_string(),
                    parameters: HashMap::new(),
                },
                None,
            ),
            AuditRecord::new(
                EventType::HumanOverride,
                Actor::User {
                    user_id: "admin".to_string(),
                    role: "manager".to_string(),
                },
                "statute-1".to_string(),
                subject_id,
                DecisionContext::default(),
                DecisionResult::Deterministic {
                    effect_applied: "rejected".to_string(),
                    parameters: HashMap::new(),
                },
                None,
            ),
        ];

        let graph = LineageBuilder::build(&records).unwrap();
        assert_eq!(graph.nodes.len(), 2);
        assert!(!graph.edges.is_empty());
    }

    #[test]
    fn test_dot_generation() {
        let mut graph = LineageGraph::new();
        let node1 = LineageNode {
            id: Uuid::new_v4(),
            node_type: LineageNodeType::Original,
            statute_id: "statute-1".to_string(),
            subject_id: Uuid::new_v4(),
            timestamp: "2024-01-01".to_string(),
            actor: "System".to_string(),
            result: "Approved".to_string(),
        };
        graph.add_node(node1);

        let dot = graph.to_dot();
        assert!(dot.contains("digraph DecisionLineage"));
        assert!(dot.contains("statute-1"));
    }

    #[test]
    fn test_svg_generation() {
        let mut graph = LineageGraph::new();
        let node1 = LineageNode {
            id: Uuid::new_v4(),
            node_type: LineageNodeType::Original,
            statute_id: "statute-1".to_string(),
            subject_id: Uuid::new_v4(),
            timestamp: "2024-01-01".to_string(),
            actor: "System".to_string(),
            result: "Approved".to_string(),
        };
        graph.add_node(node1);

        let svg = graph.to_svg();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("statute-1"));
    }
}
