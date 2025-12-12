//! Legalis-Viz: Visualization engine for legal statutes.
//!
//! This crate provides visualization capabilities for legal documents:
//! - Decision trees for eligibility determination
//! - Flowcharts for legal procedures
//! - Dependency graphs between statutes
//! - Highlighting of discretionary "gray zones"

use legalis_core::{Condition, Statute};
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::dot::{Config, Dot};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Errors during visualization.
#[derive(Debug, Error)]
pub enum VizError {
    #[error("Invalid statute structure: {0}")]
    InvalidStructure(String),

    #[error("Rendering error: {0}")]
    RenderError(String),

    #[error("Export error: {0}")]
    ExportError(String),
}

/// Result type for visualization operations.
pub type VizResult<T> = Result<T, VizError>;

/// Node types in a decision tree.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DecisionNode {
    /// Root node (statute entry point)
    Root { statute_id: String, title: String },
    /// Condition check node
    Condition { description: String, is_discretionary: bool },
    /// Outcome node (deterministic result)
    Outcome { description: String },
    /// Discretionary node (requires human judgment)
    Discretion { issue: String, hint: Option<String> },
}

/// Edge labels in decision graphs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EdgeLabel {
    Yes,
    No,
    Maybe,
    Proceeds,
}

impl std::fmt::Display for EdgeLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Yes => write!(f, "Yes"),
            Self::No => write!(f, "No"),
            Self::Maybe => write!(f, "Maybe"),
            Self::Proceeds => write!(f, "→"),
        }
    }
}

/// Decision tree representation of a statute.
pub struct DecisionTree {
    graph: DiGraph<DecisionNode, EdgeLabel>,
    root: Option<NodeIndex>,
    node_map: HashMap<String, NodeIndex>,
}

impl DecisionTree {
    /// Creates a new empty decision tree.
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            root: None,
            node_map: HashMap::new(),
        }
    }

    /// Builds a decision tree from a statute.
    pub fn from_statute(statute: &Statute) -> VizResult<Self> {
        let mut tree = Self::new();

        // Create root node
        let root = tree.graph.add_node(DecisionNode::Root {
            statute_id: statute.id.clone(),
            title: statute.title.clone(),
        });
        tree.root = Some(root);
        tree.node_map.insert(statute.id.clone(), root);

        let mut current = root;

        // Add condition nodes
        for (i, condition) in statute.preconditions.iter().enumerate() {
            let is_discretionary = matches!(condition, Condition::Custom { .. });
            let cond_node = tree.graph.add_node(DecisionNode::Condition {
                description: format_condition(condition),
                is_discretionary,
            });

            tree.graph.add_edge(current, cond_node, EdgeLabel::Proceeds);

            // Add "No" branch to void
            let void_node = tree.graph.add_node(DecisionNode::Outcome {
                description: format!("Condition {} not met", i + 1),
            });
            tree.graph.add_edge(cond_node, void_node, EdgeLabel::No);

            current = cond_node;
        }

        // Add final outcome
        if statute.discretion_logic.is_some() {
            let discretion_node = tree.graph.add_node(DecisionNode::Discretion {
                issue: "Discretionary review required".to_string(),
                hint: statute.discretion_logic.clone(),
            });
            tree.graph.add_edge(current, discretion_node, EdgeLabel::Yes);
        } else {
            let outcome = tree.graph.add_node(DecisionNode::Outcome {
                description: statute.effect.description.clone(),
            });
            tree.graph.add_edge(current, outcome, EdgeLabel::Yes);
        }

        Ok(tree)
    }

    /// Exports the tree to DOT format (GraphViz).
    pub fn to_dot(&self) -> String {
        format!("{:?}", Dot::with_config(&self.graph, &[Config::EdgeNoLabel]))
    }

    /// Exports the tree to Mermaid format.
    pub fn to_mermaid(&self) -> String {
        let mut output = String::from("flowchart TD\n");

        for node_idx in self.graph.node_indices() {
            let node = &self.graph[node_idx];
            let node_id = format!("N{}", node_idx.index());

            match node {
                DecisionNode::Root { title, .. } => {
                    output.push_str(&format!("    {}[\"{}\"]\n", node_id, title));
                }
                DecisionNode::Condition { description, is_discretionary } => {
                    if *is_discretionary {
                        output.push_str(&format!("    {}{{\"⚠️ {}\"}}:::discretion\n", node_id, description));
                    } else {
                        output.push_str(&format!("    {}{{\"{}\"}}:::condition\n", node_id, description));
                    }
                }
                DecisionNode::Outcome { description } => {
                    output.push_str(&format!("    {}([\"✓ {}\"]):::outcome\n", node_id, description));
                }
                DecisionNode::Discretion { issue, .. } => {
                    output.push_str(&format!("    {}[/\"🔴 {}\"/]:::discretion\n", node_id, issue));
                }
            }
        }

        output.push('\n');

        for edge in self.graph.edge_indices() {
            if let Some((source, target)) = self.graph.edge_endpoints(edge) {
                let label = &self.graph[edge];
                output.push_str(&format!(
                    "    N{} -->|{}| N{}\n",
                    source.index(),
                    label,
                    target.index()
                ));
            }
        }

        output.push_str("\n    classDef condition fill:#e1f5fe\n");
        output.push_str("    classDef outcome fill:#c8e6c9\n");
        output.push_str("    classDef discretion fill:#ffcdd2\n");

        output
    }

    /// Returns the number of nodes in the tree.
    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    /// Returns the number of discretionary nodes.
    pub fn discretionary_count(&self) -> usize {
        self.graph
            .node_indices()
            .filter(|&idx| {
                matches!(
                    &self.graph[idx],
                    DecisionNode::Discretion { .. }
                        | DecisionNode::Condition { is_discretionary: true, .. }
                )
            })
            .count()
    }
}

impl Default for DecisionTree {
    fn default() -> Self {
        Self::new()
    }
}

/// Statute dependency graph.
pub struct DependencyGraph {
    graph: DiGraph<String, String>,
    statute_map: HashMap<String, NodeIndex>,
}

impl DependencyGraph {
    /// Creates a new dependency graph.
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            statute_map: HashMap::new(),
        }
    }

    /// Adds a statute to the graph.
    pub fn add_statute(&mut self, statute_id: &str) -> NodeIndex {
        if let Some(&idx) = self.statute_map.get(statute_id) {
            idx
        } else {
            let idx = self.graph.add_node(statute_id.to_string());
            self.statute_map.insert(statute_id.to_string(), idx);
            idx
        }
    }

    /// Adds a dependency edge.
    pub fn add_dependency(&mut self, from: &str, to: &str, relation: &str) {
        let from_idx = self.add_statute(from);
        let to_idx = self.add_statute(to);
        self.graph.add_edge(from_idx, to_idx, relation.to_string());
    }

    /// Exports to DOT format.
    pub fn to_dot(&self) -> String {
        format!("{:?}", Dot::with_config(&self.graph, &[Config::EdgeNoLabel]))
    }

    /// Exports to Mermaid format.
    pub fn to_mermaid(&self) -> String {
        let mut output = String::from("flowchart LR\n");

        for node_idx in self.graph.node_indices() {
            let statute_id = &self.graph[node_idx];
            output.push_str(&format!("    N{}[\"{}\"]\n", node_idx.index(), statute_id));
        }

        output.push('\n');

        for edge in self.graph.edge_indices() {
            if let Some((source, target)) = self.graph.edge_endpoints(edge) {
                let label = &self.graph[edge];
                output.push_str(&format!(
                    "    N{} -->|{}| N{}\n",
                    source.index(),
                    label,
                    target.index()
                ));
            }
        }

        output
    }
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}

/// Formats a condition for display.
fn format_condition(condition: &Condition) -> String {
    match condition {
        Condition::Age { operator, value } => {
            format!("Age {} {}", format_operator(operator), value)
        }
        Condition::Income { operator, value } => {
            format!("Income {} {}", format_operator(operator), value)
        }
        Condition::HasAttribute { key } => {
            format!("Has '{}'", key)
        }
        Condition::AttributeEquals { key, value } => {
            format!("{} = {}", key, value)
        }
        Condition::And(_, _) => "AND condition".to_string(),
        Condition::Or(_, _) => "OR condition".to_string(),
        Condition::Not(_) => "NOT condition".to_string(),
        Condition::Custom { description } => description.clone(),
    }
}

fn format_operator(op: &legalis_core::ComparisonOp) -> &'static str {
    match op {
        legalis_core::ComparisonOp::Equal => "=",
        legalis_core::ComparisonOp::NotEqual => "≠",
        legalis_core::ComparisonOp::GreaterThan => ">",
        legalis_core::ComparisonOp::GreaterOrEqual => "≥",
        legalis_core::ComparisonOp::LessThan => "<",
        legalis_core::ComparisonOp::LessOrEqual => "≤",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{ComparisonOp, Effect, EffectType};

    #[test]
    fn test_decision_tree_from_statute() {
        let statute = Statute::new(
            "adult-rights",
            "Adult Rights Act",
            Effect::new(EffectType::Grant, "Full legal capacity"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let tree = DecisionTree::from_statute(&statute).unwrap();
        assert!(tree.node_count() > 0);
    }

    #[test]
    fn test_mermaid_export() {
        let statute = Statute::new(
            "test",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        );

        let tree = DecisionTree::from_statute(&statute).unwrap();
        let mermaid = tree.to_mermaid();
        assert!(mermaid.contains("flowchart TD"));
    }

    #[test]
    fn test_dependency_graph() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("statute-a", "statute-b", "references");
        graph.add_dependency("statute-b", "statute-c", "amends");

        let mermaid = graph.to_mermaid();
        assert!(mermaid.contains("statute-a"));
        assert!(mermaid.contains("statute-b"));
    }
}
