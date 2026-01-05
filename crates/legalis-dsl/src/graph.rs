//! Graph generation for statute dependencies and relationships.
//!
//! This module provides utilities to generate visual representations of
//! statute dependencies using GraphViz DOT format and Mermaid diagrams.

use crate::ast::LegalDocument;
use std::collections::{HashMap, HashSet};

/// Graph generation format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphFormat {
    /// GraphViz DOT format
    Dot,
    /// Mermaid diagram format
    Mermaid,
}

/// Options for graph generation.
#[derive(Debug, Clone)]
pub struct GraphOptions {
    /// Graph format
    pub format: GraphFormat,
    /// Include node labels with titles
    pub include_titles: bool,
    /// Highlight cycles in the dependency graph
    pub highlight_cycles: bool,
    /// Graph direction (LR = left-to-right, TB = top-to-bottom)
    pub direction: GraphDirection,
}

/// Graph layout direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphDirection {
    /// Left to right
    LeftToRight,
    /// Top to bottom
    TopToBottom,
}

impl Default for GraphOptions {
    fn default() -> Self {
        Self {
            format: GraphFormat::Dot,
            include_titles: true,
            highlight_cycles: true,
            direction: GraphDirection::TopToBottom,
        }
    }
}

/// Dependency graph generator.
pub struct DependencyGraph {
    /// Statute nodes (id -> title)
    nodes: HashMap<String, String>,
    /// Dependency edges (from -> to)
    dependencies: Vec<(String, String)>,
    /// Supersedes edges (from -> to)
    supersedes: Vec<(String, String)>,
    /// Detected cycles
    cycles: Vec<Vec<String>>,
}

impl DependencyGraph {
    /// Creates a dependency graph from a legal document.
    pub fn from_document(doc: &LegalDocument) -> Self {
        let mut nodes = HashMap::new();
        let mut dependencies = Vec::new();
        let mut supersedes = Vec::new();

        for statute in &doc.statutes {
            nodes.insert(statute.id.clone(), statute.title.clone());

            for req in &statute.requires {
                dependencies.push((statute.id.clone(), req.clone()));
            }

            for sup in &statute.supersedes {
                supersedes.push((statute.id.clone(), sup.clone()));
            }
        }

        let cycles = detect_cycles(&nodes, &dependencies);

        Self {
            nodes,
            dependencies,
            supersedes,
            cycles,
        }
    }

    /// Generates the graph in the specified format.
    pub fn generate(&self, options: &GraphOptions) -> String {
        match options.format {
            GraphFormat::Dot => self.generate_dot(options),
            GraphFormat::Mermaid => self.generate_mermaid(options),
        }
    }

    /// Generates GraphViz DOT format.
    fn generate_dot(&self, options: &GraphOptions) -> String {
        let mut output = String::new();

        output.push_str("digraph StatuteDependencies {\n");

        // Set graph attributes
        let rankdir = match options.direction {
            GraphDirection::LeftToRight => "LR",
            GraphDirection::TopToBottom => "TB",
        };
        output.push_str(&format!("    rankdir={};\n", rankdir));
        output.push_str("    node [shape=box, style=rounded];\n");
        output.push_str("    edge [fontsize=10];\n\n");

        // Add nodes
        for (id, title) in &self.nodes {
            let label = if options.include_titles {
                format!("{}\\n{}", id, escape_dot_label(title))
            } else {
                id.clone()
            };

            // Check if node is part of a cycle
            let is_in_cycle =
                options.highlight_cycles && self.cycles.iter().any(|cycle| cycle.contains(id));

            if is_in_cycle {
                output.push_str(&format!(
                    "    \"{}\" [label=\"{}\", color=red, penwidth=2];\n",
                    id, label
                ));
            } else {
                output.push_str(&format!("    \"{}\" [label=\"{}\"];\n", id, label));
            }
        }

        output.push('\n');

        // Add dependency edges
        for (from, to) in &self.dependencies {
            output.push_str(&format!(
                "    \"{}\" -> \"{}\" [label=\"requires\", color=blue];\n",
                from, to
            ));
        }

        // Add supersedes edges
        for (from, to) in &self.supersedes {
            output.push_str(&format!(
                "    \"{}\" -> \"{}\" [label=\"supersedes\", color=red, style=dashed];\n",
                from, to
            ));
        }

        output.push_str("}\n");
        output
    }

    /// Generates Mermaid diagram format.
    fn generate_mermaid(&self, options: &GraphOptions) -> String {
        let mut output = String::new();

        let direction = match options.direction {
            GraphDirection::LeftToRight => "LR",
            GraphDirection::TopToBottom => "TB",
        };

        output.push_str(&format!("graph {}\n", direction));

        // Add nodes with optional titles
        for (id, title) in &self.nodes {
            let label = if options.include_titles {
                format!("{}\\n{}", id, escape_mermaid_label(title))
            } else {
                id.clone()
            };

            // Check if node is part of a cycle
            let is_in_cycle =
                options.highlight_cycles && self.cycles.iter().any(|cycle| cycle.contains(id));

            if is_in_cycle {
                output.push_str(&format!("    {}[\"{}\"]\n", sanitize_id(id), label));
                output.push_str(&format!("    style {} fill:#ffcccc\n", sanitize_id(id)));
            } else {
                output.push_str(&format!("    {}[\"{}\"]\n", sanitize_id(id), label));
            }
        }

        output.push('\n');

        // Add dependency edges
        for (from, to) in &self.dependencies {
            output.push_str(&format!(
                "    {} -->|requires| {}\n",
                sanitize_id(from),
                sanitize_id(to)
            ));
        }

        // Add supersedes edges
        for (from, to) in &self.supersedes {
            output.push_str(&format!(
                "    {} -.->|supersedes| {}\n",
                sanitize_id(from),
                sanitize_id(to)
            ));
        }

        output
    }

    /// Returns detected cycles.
    pub fn cycles(&self) -> &[Vec<String>] {
        &self.cycles
    }

    /// Returns true if the graph has cycles.
    pub fn has_cycles(&self) -> bool {
        !self.cycles.is_empty()
    }
}

/// Detects cycles in the dependency graph using DFS.
fn detect_cycles(nodes: &HashMap<String, String>, edges: &[(String, String)]) -> Vec<Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    // Build adjacency list
    for id in nodes.keys() {
        graph.insert(id.clone(), Vec::new());
    }
    for (from, to) in edges {
        graph.entry(from.clone()).or_default().push(to.clone());
    }

    let mut cycles = Vec::new();
    let mut visited = HashSet::new();
    let mut rec_stack = HashSet::new();
    let mut path = Vec::new();

    for node in nodes.keys() {
        if !visited.contains(node) {
            dfs_cycle_detect(
                node,
                &graph,
                &mut visited,
                &mut rec_stack,
                &mut path,
                &mut cycles,
            );
        }
    }

    cycles
}

/// DFS helper for cycle detection.
fn dfs_cycle_detect(
    node: &str,
    graph: &HashMap<String, Vec<String>>,
    visited: &mut HashSet<String>,
    rec_stack: &mut HashSet<String>,
    path: &mut Vec<String>,
    cycles: &mut Vec<Vec<String>>,
) {
    visited.insert(node.to_string());
    rec_stack.insert(node.to_string());
    path.push(node.to_string());

    if let Some(neighbors) = graph.get(node) {
        for neighbor in neighbors {
            if !visited.contains(neighbor) {
                dfs_cycle_detect(neighbor, graph, visited, rec_stack, path, cycles);
            } else if rec_stack.contains(neighbor) {
                // Found a cycle
                if let Some(start_idx) = path.iter().position(|n| n == neighbor) {
                    let cycle = path[start_idx..].to_vec();
                    if !cycles.contains(&cycle) {
                        cycles.push(cycle);
                    }
                }
            }
        }
    }

    path.pop();
    rec_stack.remove(node);
}

/// Escapes special characters for DOT labels.
fn escape_dot_label(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}

/// Escapes special characters for Mermaid labels.
fn escape_mermaid_label(s: &str) -> String {
    s.replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// Sanitizes IDs for Mermaid (alphanumeric and underscores only).
fn sanitize_id(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

/// Generates a dependency graph in DOT format.
pub fn generate_dot_graph(doc: &LegalDocument) -> String {
    let graph = DependencyGraph::from_document(doc);
    graph.generate(&GraphOptions::default())
}

/// Generates a dependency graph in Mermaid format.
pub fn generate_mermaid_graph(doc: &LegalDocument) -> String {
    let graph = DependencyGraph::from_document(doc);
    let options = GraphOptions {
        format: GraphFormat::Mermaid,
        ..Default::default()
    };
    graph.generate(&options)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{EffectNode, StatuteNode};

    fn sample_document() -> LegalDocument {
        LegalDocument {
            namespace: None,
            exports: vec![],
            imports: vec![],
            statutes: vec![
                StatuteNode {
                    id: "statute-a".to_string(),
                    title: "Statute A".to_string(),
                    visibility: crate::module_system::Visibility::Private,
                    conditions: vec![],
                    effects: vec![EffectNode {
                        effect_type: "grant".to_string(),
                        description: "Rights".to_string(),
                        parameters: vec![],
                    }],
                    discretion: None,
                    exceptions: vec![],
                    amendments: vec![],
                    supersedes: vec![],
                    defaults: vec![],
                    requires: vec!["statute-b".to_string()],
                    delegates: vec![],
                    scope: None,
                    constraints: vec![],
                    priority: None,
                },
                StatuteNode {
                    id: "statute-b".to_string(),
                    title: "Statute B".to_string(),
                    visibility: crate::module_system::Visibility::Private,
                    conditions: vec![],
                    effects: vec![],
                    discretion: None,
                    exceptions: vec![],
                    amendments: vec![],
                    supersedes: vec!["statute-c".to_string()],
                    defaults: vec![],
                    requires: vec![],
                    delegates: vec![],
                    scope: None,
                    constraints: vec![],
                    priority: None,
                },
                StatuteNode {
                    id: "statute-c".to_string(),
                    title: "Statute C".to_string(),
                    visibility: crate::module_system::Visibility::Private,
                    conditions: vec![],
                    effects: vec![],
                    discretion: None,
                    exceptions: vec![],
                    amendments: vec![],
                    supersedes: vec![],
                    defaults: vec![],
                    requires: vec![],
                    delegates: vec![],
                    scope: None,
                    constraints: vec![],
                    priority: None,
                },
            ],
        }
    }

    #[test]
    fn test_dependency_graph_creation() {
        let doc = sample_document();
        let graph = DependencyGraph::from_document(&doc);

        assert_eq!(graph.nodes.len(), 3);
        assert_eq!(graph.dependencies.len(), 1);
        assert_eq!(graph.supersedes.len(), 1);
    }

    #[test]
    fn test_dot_generation() {
        let doc = sample_document();
        let dot = generate_dot_graph(&doc);

        assert!(dot.contains("digraph StatuteDependencies"));
        assert!(dot.contains("statute-a"));
        assert!(dot.contains("statute-b"));
        assert!(dot.contains("requires"));
        assert!(dot.contains("supersedes"));
    }

    #[test]
    fn test_mermaid_generation() {
        let doc = sample_document();
        let mermaid = generate_mermaid_graph(&doc);

        assert!(mermaid.contains("graph TB"));
        assert!(mermaid.contains("statute_a"));
        assert!(mermaid.contains("statute_b"));
        assert!(mermaid.contains("requires"));
        assert!(mermaid.contains("supersedes"));
    }

    #[test]
    fn test_cycle_detection() {
        let mut doc = sample_document();
        // Create a cycle: a -> b -> c -> a
        doc.statutes[1].requires.push("statute-c".to_string());
        doc.statutes[2].requires.push("statute-a".to_string());

        let graph = DependencyGraph::from_document(&doc);
        assert!(graph.has_cycles());
    }

    #[test]
    fn test_no_cycles() {
        let doc = sample_document();
        let graph = DependencyGraph::from_document(&doc);
        assert!(!graph.has_cycles());
    }
}
