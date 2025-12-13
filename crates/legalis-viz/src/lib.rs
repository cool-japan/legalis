//! Legalis-Viz: Visualization engine for legal statutes.
//!
//! This crate provides visualization capabilities for legal documents:
//! - Decision trees for eligibility determination
//! - Flowcharts for legal procedures
//! - Dependency graphs between statutes
//! - Highlighting of discretionary "gray zones"

use legalis_core::{Condition, Statute};
use petgraph::dot::{Config, Dot};
use petgraph::graph::{DiGraph, NodeIndex};
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
    Condition {
        description: String,
        is_discretionary: bool,
    },
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
            tree.graph
                .add_edge(current, discretion_node, EdgeLabel::Yes);
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
        format!(
            "{:?}",
            Dot::with_config(&self.graph, &[Config::EdgeNoLabel])
        )
    }

    /// Exports the tree to ASCII format for terminal display.
    pub fn to_ascii(&self) -> String {
        let mut output = String::new();
        let root_idx = match self.root {
            Some(idx) => idx,
            None => return output,
        };

        output.push_str(&self.ascii_node(root_idx, "", true));
        output
    }

    /// Helper to render a single node in ASCII format.
    fn ascii_node(&self, idx: NodeIndex, prefix: &str, is_last: bool) -> String {
        let mut output = String::new();
        let node = &self.graph[idx];

        let connector = if prefix.is_empty() {
            ""
        } else if is_last {
            "└── "
        } else {
            "├── "
        };
        let node_text = match node {
            DecisionNode::Root { title, statute_id } => {
                format!("📜 {} ({})", title, statute_id)
            }
            DecisionNode::Condition {
                description,
                is_discretionary,
            } => {
                if *is_discretionary {
                    format!("⚠️  {}", description)
                } else {
                    format!("❓ {}", description)
                }
            }
            DecisionNode::Outcome { description } => {
                format!("✓ {}", description)
            }
            DecisionNode::Discretion { issue, hint } => match hint {
                Some(h) => format!("🔴 {} (hint: {})", issue, h),
                None => format!("🔴 {}", issue),
            },
        };

        output.push_str(&format!("{}{}{}\n", prefix, connector, node_text));

        // Get children
        let children: Vec<_> = self.graph.neighbors(idx).collect();

        let child_prefix = if prefix.is_empty() {
            String::new()
        } else if is_last {
            format!("{}    ", prefix)
        } else {
            format!("{}│   ", prefix)
        };

        for (i, &child) in children.iter().enumerate() {
            let edge = self.graph.find_edge(idx, child);
            let label = edge.map(|e| &self.graph[e]);

            // Add edge label
            if let Some(label) = label {
                let edge_connector = if i == children.len() - 1 {
                    "└"
                } else {
                    "├"
                };
                output.push_str(&format!(
                    "{}{}─[{}]─┐\n",
                    child_prefix, edge_connector, label
                ));
            }

            let nested_prefix = format!("{}        ", child_prefix);
            output.push_str(&self.ascii_node(child, &nested_prefix, i == children.len() - 1));
        }

        output
    }

    /// Exports the tree to a compact box format.
    pub fn to_box(&self) -> String {
        let mut output = String::new();
        let root_idx = match self.root {
            Some(idx) => idx,
            None => return output,
        };

        // Collect all nodes at each level using BFS
        let mut levels: Vec<Vec<NodeIndex>> = Vec::new();
        let mut visited = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((root_idx, 0usize));
        visited.insert(root_idx);

        while let Some((node, level)) = queue.pop_front() {
            while levels.len() <= level {
                levels.push(Vec::new());
            }
            levels[level].push(node);

            for neighbor in self.graph.neighbors(node) {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back((neighbor, level + 1));
                }
            }
        }

        // Render each level
        for (level_idx, level_nodes) in levels.iter().enumerate() {
            if level_idx > 0 {
                // Add connector lines
                output.push_str("         │\n");
                output.push_str("         ▼\n");
            }

            for node_idx in level_nodes {
                let node = &self.graph[*node_idx];
                let (icon, text, style) = match node {
                    DecisionNode::Root { title, .. } => ("📜", title.clone(), "═"),
                    DecisionNode::Condition {
                        description,
                        is_discretionary,
                    } => {
                        if *is_discretionary {
                            ("⚠️", description.clone(), "~")
                        } else {
                            ("❓", description.clone(), "-")
                        }
                    }
                    DecisionNode::Outcome { description } => ("✓", description.clone(), "─"),
                    DecisionNode::Discretion { issue, .. } => ("🔴", issue.clone(), "═"),
                };

                let width = text.len().max(20) + 4;
                let border_top: String = style.repeat(width);
                let border_bot: String = style.repeat(width);
                let padding = width - text.len() - 2;
                let left_pad = padding / 2;
                let right_pad = padding - left_pad;

                output.push_str(&format!("┌{}┐\n", border_top));
                output.push_str(&format!(
                    "│ {}{} {}{}│\n",
                    " ".repeat(left_pad),
                    icon,
                    text,
                    " ".repeat(right_pad)
                ));
                output.push_str(&format!("└{}┘\n", border_bot));
            }
        }

        output
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
                DecisionNode::Condition {
                    description,
                    is_discretionary,
                } => {
                    if *is_discretionary {
                        output.push_str(&format!(
                            "    {}{{\"⚠️ {}\"}}:::discretion\n",
                            node_id, description
                        ));
                    } else {
                        output.push_str(&format!(
                            "    {}{{\"{}\"}}:::condition\n",
                            node_id, description
                        ));
                    }
                }
                DecisionNode::Outcome { description } => {
                    output.push_str(&format!(
                        "    {}([\"✓ {}\"]):::outcome\n",
                        node_id, description
                    ));
                }
                DecisionNode::Discretion { issue, .. } => {
                    output.push_str(&format!(
                        "    {}[/\"🔴 {}\"/]:::discretion\n",
                        node_id, issue
                    ));
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
                        | DecisionNode::Condition {
                            is_discretionary: true,
                            ..
                        }
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
        format!(
            "{:?}",
            Dot::with_config(&self.graph, &[Config::EdgeNoLabel])
        )
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
        Condition::DateRange { start, end } => match (start, end) {
            (Some(s), Some(e)) => format!("Date in [{}, {}]", s, e),
            (Some(s), None) => format!("Date ≥ {}", s),
            (None, Some(e)) => format!("Date ≤ {}", e),
            (None, None) => "Any date".to_string(),
        },
        Condition::Geographic {
            region_type,
            region_id,
        } => {
            format!("In {:?}({})", region_type, region_id)
        }
        Condition::EntityRelationship {
            relationship_type,
            target_entity_id,
        } => match target_entity_id {
            Some(id) => format!("{:?} with {}", relationship_type, id),
            None => format!("Has {:?}", relationship_type),
        },
        Condition::ResidencyDuration { operator, months } => {
            format!("Residency {} {} months", format_operator(operator), months)
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

    #[test]
    fn test_ascii_export() {
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
        let ascii = tree.to_ascii();

        assert!(ascii.contains("Adult Rights Act"));
        assert!(ascii.contains("Age"));
    }

    #[test]
    fn test_box_export() {
        let statute = Statute::new(
            "test-statute",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        );

        let tree = DecisionTree::from_statute(&statute).unwrap();
        let box_output = tree.to_box();

        assert!(box_output.contains("Test Statute"));
        assert!(box_output.contains("┌"));
        assert!(box_output.contains("└"));
    }

    #[test]
    fn test_ascii_with_discretion() {
        let statute = Statute::new(
            "discretionary",
            "Discretionary Statute",
            Effect::new(EffectType::Grant, "Some right"),
        )
        .with_discretion("Consider circumstances");

        let tree = DecisionTree::from_statute(&statute).unwrap();
        let ascii = tree.to_ascii();

        assert!(ascii.contains("Discretionary"));
        assert!(ascii.contains("🔴"));
    }
}
