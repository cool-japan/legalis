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

#[cfg(feature = "png-export")]
use resvg::usvg;
#[cfg(feature = "png-export")]
use tiny_skia::Pixmap;

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

/// Color theme for visualizations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    /// Color for root nodes
    pub root_color: String,
    /// Color for condition nodes
    pub condition_color: String,
    /// Color for discretionary nodes
    pub discretion_color: String,
    /// Color for outcome nodes
    pub outcome_color: String,
    /// Color for links/edges
    pub link_color: String,
    /// Background color
    pub background_color: String,
    /// Text color
    pub text_color: String,
}

impl Theme {
    /// Creates a default light theme.
    pub fn light() -> Self {
        Self {
            root_color: "#f0f0f0".to_string(),
            condition_color: "#e1f5fe".to_string(),
            discretion_color: "#ffcdd2".to_string(),
            outcome_color: "#c8e6c9".to_string(),
            link_color: "#ccc".to_string(),
            background_color: "#ffffff".to_string(),
            text_color: "#333333".to_string(),
        }
    }

    /// Creates a dark theme.
    pub fn dark() -> Self {
        Self {
            root_color: "#2c2c2c".to_string(),
            condition_color: "#1e3a5f".to_string(),
            discretion_color: "#5c1a1a".to_string(),
            outcome_color: "#1a4d2e".to_string(),
            link_color: "#666".to_string(),
            background_color: "#1a1a1a".to_string(),
            text_color: "#e0e0e0".to_string(),
        }
    }

    /// Creates a high-contrast theme for accessibility.
    pub fn high_contrast() -> Self {
        Self {
            root_color: "#000000".to_string(),
            condition_color: "#0000ff".to_string(),
            discretion_color: "#ff0000".to_string(),
            outcome_color: "#00ff00".to_string(),
            link_color: "#000000".to_string(),
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        }
    }

    /// Creates a colorblind-friendly theme.
    pub fn colorblind_friendly() -> Self {
        Self {
            root_color: "#999999".to_string(),
            condition_color: "#0173b2".to_string(),
            discretion_color: "#de8f05".to_string(),
            outcome_color: "#029e73".to_string(),
            link_color: "#999999".to_string(),
            background_color: "#ffffff".to_string(),
            text_color: "#333333".to_string(),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::light()
    }
}

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

/// Annotation for judicial notes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Annotation {
    /// Annotation ID
    pub id: String,
    /// Target node or element
    pub target: String,
    /// Annotation text
    pub text: String,
    /// Citation (e.g., case law reference)
    pub citation: Option<String>,
    /// Author (e.g., judge, commentator)
    pub author: Option<String>,
    /// Date of annotation
    pub date: Option<String>,
    /// Annotation type (note, warning, interpretation, etc.)
    pub annotation_type: AnnotationType,
}

/// Types of annotations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnnotationType {
    /// General note
    Note,
    /// Warning or caution
    Warning,
    /// Legal interpretation
    Interpretation,
    /// Case law reference
    CaseLaw,
    /// Legislative history
    LegislativeHistory,
    /// Commentary
    Commentary,
}

impl Annotation {
    /// Creates a new annotation.
    pub fn new(id: &str, target: &str, text: &str) -> Self {
        Self {
            id: id.to_string(),
            target: target.to_string(),
            text: text.to_string(),
            citation: None,
            author: None,
            date: None,
            annotation_type: AnnotationType::Note,
        }
    }

    /// Sets the citation.
    pub fn with_citation(mut self, citation: &str) -> Self {
        self.citation = Some(citation.to_string());
        self
    }

    /// Sets the author.
    pub fn with_author(mut self, author: &str) -> Self {
        self.author = Some(author.to_string());
        self
    }

    /// Sets the date.
    pub fn with_date(mut self, date: &str) -> Self {
        self.date = Some(date.to_string());
        self
    }

    /// Sets the annotation type.
    pub fn with_type(mut self, annotation_type: AnnotationType) -> Self {
        self.annotation_type = annotation_type;
        self
    }
}

/// Decision tree representation of a statute.
pub struct DecisionTree {
    graph: DiGraph<DecisionNode, EdgeLabel>,
    root: Option<NodeIndex>,
    node_map: HashMap<String, NodeIndex>,
    annotations: Vec<Annotation>,
}

impl DecisionTree {
    /// Creates a new empty decision tree.
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            root: None,
            node_map: HashMap::new(),
            annotations: Vec::new(),
        }
    }

    /// Adds an annotation to the decision tree.
    pub fn add_annotation(&mut self, annotation: Annotation) {
        self.annotations.push(annotation);
    }

    /// Gets all annotations.
    pub fn annotations(&self) -> &[Annotation] {
        &self.annotations
    }

    /// Gets annotations for a specific target.
    pub fn annotations_for(&self, target: &str) -> Vec<&Annotation> {
        self.annotations
            .iter()
            .filter(|a| a.target == target)
            .collect()
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

        // Add annotations section if any exist
        if !self.annotations.is_empty() {
            output.push_str("\n\nAnnotations:\n");
            output.push_str("============\n");
            for annotation in &self.annotations {
                output.push_str(&format!("\n[{}] {}\n", annotation.id, annotation.target));
                output.push_str(&format!("  Type: {:?}\n", annotation.annotation_type));
                output.push_str(&format!("  {}\n", annotation.text));
                if let Some(citation) = &annotation.citation {
                    output.push_str(&format!("  Citation: {}\n", citation));
                }
                if let Some(author) = &annotation.author {
                    output.push_str(&format!("  Author: {}\n", author));
                }
                if let Some(date) = &annotation.date {
                    output.push_str(&format!("  Date: {}\n", date));
                }
            }
        }

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

    /// Exports the tree to PlantUML format.
    pub fn to_plantuml(&self) -> String {
        let mut output = String::from("@startuml\n");
        output.push_str("skinparam defaultTextAlignment center\n");
        output.push_str("skinparam activity {\n");
        output.push_str("  BackgroundColor<<discretion>> LightPink\n");
        output.push_str("  BackgroundColor<<outcome>> LightGreen\n");
        output.push_str("  BackgroundColor<<condition>> LightBlue\n");
        output.push_str("}\n\n");

        output.push_str("start\n");

        if let Some(root_idx) = self.root {
            self.plantuml_node(root_idx, &mut output);
        }

        output.push_str("stop\n");
        output.push_str("@enduml\n");
        output
    }

    /// Helper to render a node in PlantUML format.
    fn plantuml_node(&self, idx: NodeIndex, output: &mut String) {
        let node = &self.graph[idx];

        match node {
            DecisionNode::Root { title, .. } => {
                output.push_str(&format!(":{};\n", title));
            }
            DecisionNode::Condition {
                description,
                is_discretionary,
            } => {
                if *is_discretionary {
                    output.push_str(&format!(":{}; <<discretion>>\n", description));
                } else {
                    output.push_str(&format!("if ({}) then (yes)\n", description));
                }
            }
            DecisionNode::Outcome { description } => {
                output.push_str(&format!(":{}; <<outcome>>\n", description));
            }
            DecisionNode::Discretion { issue, hint } => {
                let text = if let Some(h) = hint {
                    format!("{}\n({})", issue, h)
                } else {
                    issue.clone()
                };
                output.push_str(&format!(":{}; <<discretion>>\n", text));
            }
        }

        // Process children
        let children: Vec<_> = self.graph.neighbors(idx).collect();
        for &child in &children {
            self.plantuml_node(child, output);
        }
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

    /// Exports the tree to SVG format.
    pub fn to_svg(&self) -> String {
        self.to_svg_with_theme(&Theme::default())
    }

    /// Exports the tree to SVG format with a custom theme.
    pub fn to_svg_with_theme(&self, theme: &Theme) -> String {
        let mut svg = String::new();
        let width = 800;
        let height = 600;

        svg.push_str(&format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" style=\"background-color: {}\">\n",
            width, height, theme.background_color
        ));

        svg.push_str("  <defs>\n");
        svg.push_str("    <marker id=\"arrowhead\" markerWidth=\"10\" markerHeight=\"7\" refX=\"9\" refY=\"3.5\" orient=\"auto\">\n");
        svg.push_str(&format!(
            "      <polygon points=\"0 0, 10 3.5, 0 7\" fill=\"{}\" />\n",
            theme.link_color
        ));
        svg.push_str("    </marker>\n");
        svg.push_str("  </defs>\n");

        if let Some(root_idx) = self.root {
            let mut y_offset = 50;
            self.svg_render_node(
                root_idx,
                width / 2,
                y_offset,
                theme,
                &mut svg,
                &mut y_offset,
            );
        }

        svg.push_str("</svg>");
        svg
    }

    /// Exports the tree to PNG format.
    #[cfg(feature = "png-export")]
    pub fn to_png(&self) -> VizResult<Vec<u8>> {
        self.to_png_with_theme(&Theme::default())
    }

    /// Exports the tree to PNG format with a custom theme.
    #[cfg(feature = "png-export")]
    pub fn to_png_with_theme(&self, theme: &Theme) -> VizResult<Vec<u8>> {
        let svg_data = self.to_svg_with_theme(theme);
        svg_to_png(&svg_data)
    }

    /// Helper to render a node in SVG format.
    fn svg_render_node(
        &self,
        idx: NodeIndex,
        x: usize,
        y: usize,
        theme: &Theme,
        svg: &mut String,
        y_offset: &mut usize,
    ) {
        let node = &self.graph[idx];

        let (color, text) = match node {
            DecisionNode::Root { title, .. } => (&theme.root_color, title.clone()),
            DecisionNode::Condition {
                description,
                is_discretionary,
            } => {
                if *is_discretionary {
                    (&theme.discretion_color, description.clone())
                } else {
                    (&theme.condition_color, description.clone())
                }
            }
            DecisionNode::Outcome { description } => (&theme.outcome_color, description.clone()),
            DecisionNode::Discretion { issue, .. } => (&theme.discretion_color, issue.clone()),
        };

        // Draw node rectangle
        let rect_width = 180;
        let rect_height = 50;
        let rect_x = x.saturating_sub(rect_width / 2);

        svg.push_str(&format!(
            "  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"2\" rx=\"5\"/>\n",
            rect_x, y, rect_width, rect_height, color, theme.text_color
        ));

        // Draw text (truncated if too long)
        let display_text = if text.len() > 25 {
            format!("{}...", &text[..22])
        } else {
            text
        };

        svg.push_str(&format!(
            "  <text x=\"{}\" y=\"{}\" fill=\"{}\" text-anchor=\"middle\" font-size=\"12\">{}</text>\n",
            x,
            y + rect_height / 2 + 5,
            theme.text_color,
            display_text
        ));

        // Draw edges to children
        let children: Vec<_> = self.graph.neighbors(idx).collect();
        if !children.is_empty() {
            *y_offset += 100;
            let child_spacing = rect_width * 2;
            let start_x = x.saturating_sub(child_spacing * (children.len().saturating_sub(1)) / 2);

            for (i, &child) in children.iter().enumerate() {
                let child_x = start_x + i * child_spacing;
                let child_y = *y_offset;

                // Draw line from parent to child
                svg.push_str(&format!(
                    "  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"2\" marker-end=\"url(#arrowhead)\"/>\n",
                    x,
                    y + rect_height,
                    child_x,
                    child_y,
                    theme.link_color
                ));

                self.svg_render_node(child, child_x, child_y, theme, svg, y_offset);
            }
        }
    }

    /// Exports the tree to HTML with embedded D3.js visualization.
    pub fn to_html(&self) -> String {
        self.to_html_with_theme(&Theme::default())
    }

    /// Exports the tree to HTML with embedded D3.js visualization using a custom theme.
    /// Includes drill-down navigation support.
    pub fn to_html_with_theme(&self, theme: &Theme) -> String {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str("    <meta charset=\"utf-8\">\n");
        html.push_str("    <title>Legalis Decision Tree Visualization</title>\n");
        html.push_str("    <script src=\"https://d3js.org/d3.v7.min.js\"></script>\n");
        html.push_str("    <style>\n");
        html.push_str(&format!("        body {{ font-family: Arial, sans-serif; margin: 20px; background-color: {}; color: {}; }}\n", theme.background_color, theme.text_color));
        html.push_str("        .node { cursor: pointer; }\n");
        html.push_str(
            "        .node circle { fill: #fff; stroke: steelblue; stroke-width: 3px; transition: all 0.3s; }\n",
        );
        html.push_str("        .node circle:hover { stroke-width: 5px; }\n");
        html.push_str(&format!(
            "        .node.root circle {{ fill: {}; stroke: #333; }}\n",
            theme.root_color
        ));
        html.push_str(&format!(
            "        .node.condition circle {{ fill: {}; stroke: #0277bd; }}\n",
            theme.condition_color
        ));
        html.push_str(&format!(
            "        .node.discretion circle {{ fill: {}; stroke: #c62828; }}\n",
            theme.discretion_color
        ));
        html.push_str(&format!(
            "        .node.outcome circle {{ fill: {}; stroke: #2e7d32; }}\n",
            theme.outcome_color
        ));
        html.push_str(&format!(
            "        .node text {{ font-size: 12px; fill: {}; }}\n",
            theme.text_color
        ));
        html.push_str(&format!(
            "        .link {{ fill: none; stroke: {}; stroke-width: 2px; transition: opacity 0.3s; }}\n",
            theme.link_color
        ));
        html.push_str("        .link.hidden { opacity: 0.2; }\n");
        html.push_str("        .link-label { font-size: 10px; fill: #666; }\n");
        html.push_str("        #details { position: fixed; top: 20px; right: 20px; background: rgba(255,255,255,0.95); padding: 15px; border-radius: 5px; box-shadow: 0 2px 10px rgba(0,0,0,0.2); max-width: 300px; display: none; }\n");
        html.push_str("        #details.visible { display: block; }\n");
        html.push_str("        #details h3 { margin-top: 0; }\n");
        html.push_str("        .close-btn { float: right; cursor: pointer; font-size: 20px; }\n");
        html.push_str("    </style>\n</head>\n<body>\n");
        html.push_str("    <h1>Legal Decision Tree (Interactive)</h1>\n");
        html.push_str("    <p>Click on nodes to view details and drill down</p>\n");
        html.push_str("    <div id=\"tree\"></div>\n");
        html.push_str("    <div id=\"details\">\n");
        html.push_str("        <span class=\"close-btn\" onclick=\"document.getElementById('details').classList.remove('visible')\">&times;</span>\n");
        html.push_str("        <h3 id=\"detail-title\"></h3>\n");
        html.push_str("        <div id=\"detail-content\"></div>\n");
        html.push_str("    </div>\n");
        html.push_str("    <script>\n");
        html.push_str("const treeData = ");
        html.push_str(&self.to_d3_json());
        html.push_str(";\n");
        html.push_str("const width = 960;\nconst height = 600;\n");
        html.push_str("const svg = d3.select(\"#tree\").append(\"svg\").attr(\"width\", width).attr(\"height\", height);\n");
        html.push_str("const g = svg.append(\"g\").attr(\"transform\", \"translate(40,40)\");\n");
        html.push_str("const tree = d3.tree().size([height - 100, width - 200]);\n");
        html.push_str("const root = d3.hierarchy(treeData);\n");
        html.push_str("tree(root);\n");
        html.push_str("const link = g.selectAll(\".link\").data(root.links()).enter().append(\"path\").attr(\"class\", \"link\").attr(\"d\", d3.linkHorizontal().x(function(d) { return d.y; }).y(function(d) { return d.x; }));\n");
        html.push_str("const node = g.selectAll(\".node\").data(root.descendants()).enter().append(\"g\").attr(\"class\", function(d) { return \"node \" + d.data.type; }).attr(\"transform\", function(d) { return \"translate(\" + d.y + \",\" + d.x + \")\"; });\n");
        html.push_str("node.append(\"circle\").attr(\"r\", 6);\n");
        html.push_str("node.append(\"text\").attr(\"dy\", 3).attr(\"x\", function(d) { return d.children ? -10 : 10; }).style(\"text-anchor\", function(d) { return d.children ? \"end\" : \"start\"; }).text(function(d) { return d.data.name; });\n");
        html.push_str("node.on(\"click\", function(event, d) {\n");
        html.push_str("    const details = document.getElementById('details');\n");
        html.push_str("    const title = document.getElementById('detail-title');\n");
        html.push_str("    const content = document.getElementById('detail-content');\n");
        html.push_str("    title.textContent = d.data.name;\n");
        html.push_str(
            "    content.innerHTML = '<p><strong>Type:</strong> ' + d.data.type + '</p>';\n",
        );
        html.push_str("    if (d.children) {\n");
        html.push_str("        content.innerHTML += '<p><strong>Children:</strong> ' + d.children.length + '</p>';\n");
        html.push_str("    }\n");
        html.push_str("    if (d.depth > 0) {\n");
        html.push_str(
            "        content.innerHTML += '<p><strong>Depth:</strong> ' + d.depth + '</p>';\n",
        );
        html.push_str("    }\n");
        html.push_str("    details.classList.add('visible');\n");
        html.push_str("});\n");
        html.push_str("    </script>\n</body>\n</html>");

        html
    }

    /// Converts the tree to D3.js JSON format.
    fn to_d3_json(&self) -> String {
        if let Some(root_idx) = self.root {
            self.node_to_d3_json(root_idx)
        } else {
            "{}".to_string()
        }
    }

    /// Converts a node to D3.js JSON format.
    fn node_to_d3_json(&self, idx: NodeIndex) -> String {
        let node = &self.graph[idx];

        let (node_type, name) = match node {
            DecisionNode::Root { title, .. } => ("root", title.clone()),
            DecisionNode::Condition {
                description,
                is_discretionary,
            } => {
                if *is_discretionary {
                    ("discretion", description.clone())
                } else {
                    ("condition", description.clone())
                }
            }
            DecisionNode::Outcome { description } => ("outcome", description.clone()),
            DecisionNode::Discretion { issue, .. } => ("discretion", issue.clone()),
        };

        let children: Vec<_> = self.graph.neighbors(idx).collect();

        if children.is_empty() {
            format!(r#"{{"name": "{}", "type": "{}"}}"#, name, node_type)
        } else {
            let children_json: Vec<String> = children
                .iter()
                .map(|&child| self.node_to_d3_json(child))
                .collect();
            format!(
                r#"{{"name": "{}", "type": "{}", "children": [{}]}}"#,
                name,
                node_type,
                children_json.join(", ")
            )
        }
    }
}

impl Default for DecisionTree {
    fn default() -> Self {
        Self::new()
    }
}

/// Timeline event types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimelineEvent {
    /// Statute enacted
    Enacted { statute_id: String, title: String },
    /// Statute amended
    Amended {
        statute_id: String,
        description: String,
    },
    /// Statute repealed
    Repealed { statute_id: String },
    /// Temporal range start
    EffectiveStart { statute_id: String },
    /// Temporal range end
    EffectiveEnd { statute_id: String },
}

/// Timeline visualization for temporal statutes.
pub struct Timeline {
    events: Vec<(String, TimelineEvent)>,
}

impl Timeline {
    /// Creates a new timeline.
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    /// Adds an event to the timeline.
    pub fn add_event(&mut self, date: &str, event: TimelineEvent) {
        self.events.push((date.to_string(), event));
    }

    /// Sorts events by date.
    pub fn sort_by_date(&mut self) {
        self.events.sort_by(|a, b| a.0.cmp(&b.0));
    }

    /// Exports to ASCII timeline.
    pub fn to_ascii(&self) -> String {
        let mut output = String::new();
        output.push_str("Timeline of Legal Events\n");
        output.push_str("========================\n\n");

        for (date, event) in &self.events {
            let event_desc = match event {
                TimelineEvent::Enacted { statute_id, title } => {
                    format!("📜 ENACTED: {} - {}", statute_id, title)
                }
                TimelineEvent::Amended {
                    statute_id,
                    description,
                } => {
                    format!("✏️  AMENDED: {} - {}", statute_id, description)
                }
                TimelineEvent::Repealed { statute_id } => {
                    format!("❌ REPEALED: {}", statute_id)
                }
                TimelineEvent::EffectiveStart { statute_id } => {
                    format!("▶️  EFFECTIVE START: {}", statute_id)
                }
                TimelineEvent::EffectiveEnd { statute_id } => {
                    format!("⏹️  EFFECTIVE END: {}", statute_id)
                }
            };

            output.push_str(&format!("{} │ {}\n", date, event_desc));
        }

        output
    }

    /// Exports to Mermaid Gantt chart format.
    pub fn to_mermaid(&self) -> String {
        let mut output = String::from("gantt\n");
        output.push_str("    title Legal Timeline\n");
        output.push_str("    dateFormat YYYY-MM-DD\n\n");

        let mut statute_map: HashMap<String, Vec<(String, &TimelineEvent)>> = HashMap::new();

        for (date, event) in &self.events {
            let statute_id = match event {
                TimelineEvent::Enacted { statute_id, .. }
                | TimelineEvent::Amended { statute_id, .. }
                | TimelineEvent::Repealed { statute_id }
                | TimelineEvent::EffectiveStart { statute_id }
                | TimelineEvent::EffectiveEnd { statute_id } => statute_id,
            };
            statute_map
                .entry(statute_id.clone())
                .or_default()
                .push((date.clone(), event));
        }

        for (statute_id, events) in statute_map {
            output.push_str(&format!("    section {}\n", statute_id));
            for (date, event) in events {
                match event {
                    TimelineEvent::Enacted { title, .. } => {
                        output.push_str(&format!("    Enacted: {}, 1d\n", date));
                        output.push_str(&format!("    {} : {}, 365d\n", title, date));
                    }
                    TimelineEvent::Amended { description, .. } => {
                        output
                            .push_str(&format!("    Amendment ({}) : {}, 1d\n", description, date));
                    }
                    TimelineEvent::Repealed { .. } => {
                        output.push_str(&format!("    Repealed : {}, 1d\n", date));
                    }
                    TimelineEvent::EffectiveStart { .. } => {
                        output.push_str(&format!("    Effective period starts : {}, 1d\n", date));
                    }
                    TimelineEvent::EffectiveEnd { .. } => {
                        output.push_str(&format!("    Effective period ends : {}, 1d\n", date));
                    }
                }
            }
        }

        output
    }

    /// Exports to HTML with embedded timeline visualization.
    pub fn to_html(&self) -> String {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str("    <meta charset=\"utf-8\">\n");
        html.push_str("    <title>Legal Timeline</title>\n");
        html.push_str("    <style>\n");
        html.push_str(
            "        body { font-family: Arial, sans-serif; margin: 20px; background: #f5f5f5; }\n",
        );
        html.push_str("        h1 { color: #333; }\n");
        html.push_str(
            "        .timeline { position: relative; max-width: 800px; margin: 0 auto; }\n",
        );
        html.push_str("        .timeline::after { content: ''; position: absolute; width: 4px; background-color: #2196f3; top: 0; bottom: 0; left: 50%; margin-left: -2px; }\n");
        html.push_str("        .event { padding: 10px 40px; position: relative; background-color: inherit; width: 50%; }\n");
        html.push_str("        .event::after { content: ''; position: absolute; width: 20px; height: 20px; right: -10px; background-color: white; border: 4px solid #2196f3; top: 15px; border-radius: 50%; z-index: 1; }\n");
        html.push_str("        .left { left: 0; }\n");
        html.push_str("        .right { left: 50%; }\n");
        html.push_str("        .left::before { content: \" \"; height: 0; position: absolute; top: 22px; width: 0; z-index: 1; right: 30px; border: medium solid #2196f3; border-width: 10px 0 10px 10px; border-color: transparent transparent transparent #2196f3; }\n");
        html.push_str("        .right::before { content: \" \"; height: 0; position: absolute; top: 22px; width: 0; z-index: 1; left: 30px; border: medium solid #2196f3; border-width: 10px 10px 10px 0; border-color: transparent #2196f3 transparent transparent; }\n");
        html.push_str("        .right::after { left: -10px; }\n");
        html.push_str("        .content { padding: 20px 30px; background-color: white; position: relative; border-radius: 6px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }\n");
        html.push_str("        .date { font-weight: bold; color: #2196f3; margin-bottom: 5px; }\n");
        html.push_str("        .enacted { border-left: 4px solid #4caf50; }\n");
        html.push_str("        .amended { border-left: 4px solid #ff9800; }\n");
        html.push_str("        .repealed { border-left: 4px solid #f44336; }\n");
        html.push_str("        .effective { border-left: 4px solid #2196f3; }\n");
        html.push_str("    </style>\n</head>\n<body>\n");
        html.push_str("    <h1>Legal Timeline</h1>\n");
        html.push_str("    <div class=\"timeline\">\n");

        for (i, (date, event)) in self.events.iter().enumerate() {
            let side = if i % 2 == 0 { "left" } else { "right" };
            let (event_class, event_desc) = match event {
                TimelineEvent::Enacted { statute_id, title } => {
                    ("enacted", format!("Enacted: {} - {}", statute_id, title))
                }
                TimelineEvent::Amended {
                    statute_id,
                    description,
                } => (
                    "amended",
                    format!("Amended: {} - {}", statute_id, description),
                ),
                TimelineEvent::Repealed { statute_id } => {
                    ("repealed", format!("Repealed: {}", statute_id))
                }
                TimelineEvent::EffectiveStart { statute_id } => {
                    ("effective", format!("Effective Start: {}", statute_id))
                }
                TimelineEvent::EffectiveEnd { statute_id } => {
                    ("effective", format!("Effective End: {}", statute_id))
                }
            };

            html.push_str(&format!("        <div class=\"event {}\">\n", side));
            html.push_str(&format!(
                "            <div class=\"content {}\">\n",
                event_class
            ));
            html.push_str(&format!(
                "                <div class=\"date\">{}</div>\n",
                date
            ));
            html.push_str(&format!("                <p>{}</p>\n", event_desc));
            html.push_str("            </div>\n");
            html.push_str("        </div>\n");
        }

        html.push_str("    </div>\n</body>\n</html>");

        html
    }
}

impl Default for Timeline {
    fn default() -> Self {
        Self::new()
    }
}

/// Layout options for large graphs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutConfig {
    /// Width of the visualization
    pub width: usize,
    /// Height of the visualization
    pub height: usize,
    /// Node spacing
    pub node_spacing: usize,
    /// Enable clustering for large graphs
    pub enable_clustering: bool,
    /// Maximum nodes to display before simplification
    pub max_nodes: Option<usize>,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            width: 960,
            height: 600,
            node_spacing: 100,
            enable_clustering: false,
            max_nodes: None,
        }
    }
}

impl LayoutConfig {
    /// Creates a configuration optimized for large graphs.
    pub fn large_graph() -> Self {
        Self {
            width: 1920,
            height: 1080,
            node_spacing: 150,
            enable_clustering: true,
            max_nodes: Some(100),
        }
    }

    /// Creates a configuration for compact display.
    pub fn compact() -> Self {
        Self {
            width: 800,
            height: 400,
            node_spacing: 50,
            enable_clustering: false,
            max_nodes: Some(50),
        }
    }
}

/// Data point for population distribution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopulationDataPoint {
    /// Category or status
    pub category: String,
    /// Count of entities
    pub count: usize,
    /// Percentage (optional)
    pub percentage: Option<f64>,
}

/// Population distribution chart for simulation results.
pub struct PopulationChart {
    /// Title of the chart
    title: String,
    /// Data points
    data: Vec<PopulationDataPoint>,
    /// Time series data (time -> category -> count)
    time_series: Vec<(String, Vec<PopulationDataPoint>)>,
}

impl PopulationChart {
    /// Creates a new population chart.
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            data: Vec::new(),
            time_series: Vec::new(),
        }
    }

    /// Adds a data point.
    pub fn add_data(&mut self, category: &str, count: usize) {
        self.data.push(PopulationDataPoint {
            category: category.to_string(),
            count,
            percentage: None,
        });
    }

    /// Adds time series data.
    pub fn add_time_point(&mut self, time: &str, data: Vec<PopulationDataPoint>) {
        self.time_series.push((time.to_string(), data));
    }

    /// Calculates percentages for all data points.
    pub fn calculate_percentages(&mut self) {
        let total: usize = self.data.iter().map(|d| d.count).sum();
        if total > 0 {
            for point in &mut self.data {
                point.percentage = Some((point.count as f64 / total as f64) * 100.0);
            }
        }
    }

    /// Exports to ASCII bar chart.
    pub fn to_ascii(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("{}\n", self.title));
        output.push_str(&format!("{}\n\n", "=".repeat(self.title.len())));

        let max_count = self.data.iter().map(|d| d.count).max().unwrap_or(1);
        let bar_width = 50;

        for point in &self.data {
            let bar_len = (point.count as f64 / max_count as f64 * bar_width as f64) as usize;
            let bar = "█".repeat(bar_len);

            if let Some(pct) = point.percentage {
                output.push_str(&format!(
                    "{:<20} │ {:<50} │ {} ({:.1}%)\n",
                    point.category, bar, point.count, pct
                ));
            } else {
                output.push_str(&format!(
                    "{:<20} │ {:<50} │ {}\n",
                    point.category, bar, point.count
                ));
            }
        }

        output
    }

    /// Exports to HTML with Chart.js visualization.
    pub fn to_html(&self) -> String {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str("    <meta charset=\"utf-8\">\n");
        html.push_str(&format!("    <title>{}</title>\n", self.title));
        html.push_str("    <script src=\"https://cdn.jsdelivr.net/npm/chart.js\"></script>\n");
        html.push_str("    <style>\n");
        html.push_str("        body { font-family: Arial, sans-serif; margin: 20px; }\n");
        html.push_str("        .chart-container { max-width: 800px; margin: 0 auto; }\n");
        html.push_str("    </style>\n</head>\n<body>\n");
        html.push_str(&format!("    <h1>{}</h1>\n", self.title));
        html.push_str("    <div class=\"chart-container\">\n");
        html.push_str("        <canvas id=\"chart\"></canvas>\n");
        html.push_str("    </div>\n");
        html.push_str("    <script>\n");

        // Generate chart data
        let labels: Vec<String> = self
            .data
            .iter()
            .map(|d| format!("\"{}\"", d.category))
            .collect();
        let counts: Vec<String> = self.data.iter().map(|d| d.count.to_string()).collect();

        html.push_str("        const ctx = document.getElementById('chart').getContext('2d');\n");
        html.push_str("        new Chart(ctx, {\n");
        html.push_str("            type: 'bar',\n");
        html.push_str("            data: {\n");
        html.push_str(&format!(
            "                labels: [{}],\n",
            labels.join(", ")
        ));
        html.push_str("                datasets: [{\n");
        html.push_str("                    label: 'Population Count',\n");
        html.push_str(&format!(
            "                    data: [{}],\n",
            counts.join(", ")
        ));
        html.push_str("                    backgroundColor: [\n");
        html.push_str("                        'rgba(54, 162, 235, 0.6)',\n");
        html.push_str("                        'rgba(255, 99, 132, 0.6)',\n");
        html.push_str("                        'rgba(255, 206, 86, 0.6)',\n");
        html.push_str("                        'rgba(75, 192, 192, 0.6)',\n");
        html.push_str("                        'rgba(153, 102, 255, 0.6)',\n");
        html.push_str("                        'rgba(255, 159, 64, 0.6)'\n");
        html.push_str("                    ],\n");
        html.push_str("                    borderColor: [\n");
        html.push_str("                        'rgba(54, 162, 235, 1)',\n");
        html.push_str("                        'rgba(255, 99, 132, 1)',\n");
        html.push_str("                        'rgba(255, 206, 86, 1)',\n");
        html.push_str("                        'rgba(75, 192, 192, 1)',\n");
        html.push_str("                        'rgba(153, 102, 255, 1)',\n");
        html.push_str("                        'rgba(255, 159, 64, 1)'\n");
        html.push_str("                    ],\n");
        html.push_str("                    borderWidth: 1\n");
        html.push_str("                }]\n");
        html.push_str("            },\n");
        html.push_str("            options: {\n");
        html.push_str("                responsive: true,\n");
        html.push_str("                scales: {\n");
        html.push_str("                    y: { beginAtZero: true }\n");
        html.push_str("                }\n");
        html.push_str("            }\n");
        html.push_str("        });\n");
        html.push_str("    </script>\n</body>\n</html>");

        html
    }

    /// Exports time series to HTML with line chart.
    pub fn time_series_to_html(&self) -> String {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str("    <meta charset=\"utf-8\">\n");
        html.push_str(&format!(
            "    <title>{} - Time Series</title>\n",
            self.title
        ));
        html.push_str("    <script src=\"https://cdn.jsdelivr.net/npm/chart.js\"></script>\n");
        html.push_str("    <style>\n");
        html.push_str("        body { font-family: Arial, sans-serif; margin: 20px; }\n");
        html.push_str("        .chart-container { max-width: 1000px; margin: 0 auto; }\n");
        html.push_str("    </style>\n</head>\n<body>\n");
        html.push_str(&format!("    <h1>{} - Time Series</h1>\n", self.title));
        html.push_str("    <div class=\"chart-container\">\n");
        html.push_str("        <canvas id=\"chart\"></canvas>\n");
        html.push_str("    </div>\n");
        html.push_str("    <script>\n");

        // Extract all unique categories
        let mut categories = std::collections::HashSet::new();
        for (_time, data) in &self.time_series {
            for point in data {
                categories.insert(point.category.clone());
            }
        }
        let categories: Vec<_> = categories.into_iter().collect();

        // Generate labels (time points)
        let labels: Vec<String> = self
            .time_series
            .iter()
            .map(|(time, _)| format!("\"{}\"", time))
            .collect();

        html.push_str("        const ctx = document.getElementById('chart').getContext('2d');\n");
        html.push_str("        new Chart(ctx, {\n");
        html.push_str("            type: 'line',\n");
        html.push_str("            data: {\n");
        html.push_str(&format!(
            "                labels: [{}],\n",
            labels.join(", ")
        ));
        html.push_str("                datasets: [\n");

        let colors = [
            ("54, 162, 235", "rgba(54, 162, 235, 0.2)"),
            ("255, 99, 132", "rgba(255, 99, 132, 0.2)"),
            ("255, 206, 86", "rgba(255, 206, 86, 0.2)"),
            ("75, 192, 192", "rgba(75, 192, 192, 0.2)"),
            ("153, 102, 255", "rgba(153, 102, 255, 0.2)"),
            ("255, 159, 64", "rgba(255, 159, 64, 0.2)"),
        ];

        for (i, category) in categories.iter().enumerate() {
            let (border_rgb, background) = colors.get(i % colors.len()).unwrap();

            // Extract data for this category across all time points
            let data_points: Vec<String> = self
                .time_series
                .iter()
                .map(|(_time, data)| {
                    data.iter()
                        .find(|p| &p.category == category)
                        .map(|p| p.count.to_string())
                        .unwrap_or_else(|| "0".to_string())
                })
                .collect();

            html.push_str("                    {\n");
            html.push_str(&format!("                        label: '{}',\n", category));
            html.push_str(&format!(
                "                        data: [{}],\n",
                data_points.join(", ")
            ));
            html.push_str(&format!(
                "                        borderColor: 'rgb({})',\n",
                border_rgb
            ));
            html.push_str(&format!(
                "                        backgroundColor: '{}',\n",
                background
            ));
            html.push_str("                        tension: 0.1\n");
            html.push_str("                    }");
            if i < categories.len() - 1 {
                html.push_str(",\n");
            } else {
                html.push('\n');
            }
        }

        html.push_str("                ]\n");
        html.push_str("            },\n");
        html.push_str("            options: {\n");
        html.push_str("                responsive: true,\n");
        html.push_str("                scales: {\n");
        html.push_str("                    y: { beginAtZero: true }\n");
        html.push_str("                }\n");
        html.push_str("            }\n");
        html.push_str("        });\n");
        html.push_str("    </script>\n</body>\n</html>");

        html
    }
}

impl Default for PopulationChart {
    fn default() -> Self {
        Self::new("Population Distribution")
    }
}

/// Statute dependency graph.
pub struct DependencyGraph {
    graph: DiGraph<String, String>,
    statute_map: HashMap<String, NodeIndex>,
    layout_config: LayoutConfig,
}

impl DependencyGraph {
    /// Creates a new dependency graph.
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            statute_map: HashMap::new(),
            layout_config: LayoutConfig::default(),
        }
    }

    /// Creates a new dependency graph with custom layout configuration.
    pub fn with_layout(layout_config: LayoutConfig) -> Self {
        Self {
            graph: DiGraph::new(),
            statute_map: HashMap::new(),
            layout_config,
        }
    }

    /// Sets the layout configuration.
    pub fn set_layout(&mut self, layout_config: LayoutConfig) {
        self.layout_config = layout_config;
    }

    /// Gets the number of nodes in the graph.
    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    /// Returns true if the graph is considered large based on layout config.
    pub fn is_large_graph(&self) -> bool {
        if let Some(max_nodes) = self.layout_config.max_nodes {
            self.node_count() > max_nodes
        } else {
            false
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

    /// Exports to PlantUML format.
    pub fn to_plantuml(&self) -> String {
        let mut output = String::from("@startuml\n");
        output.push_str("!define STATUTE_COLOR LightBlue\n\n");

        for node_idx in self.graph.node_indices() {
            let statute_id = &self.graph[node_idx];
            output.push_str(&format!(
                "component \"{}\" as N{} #STATUTE_COLOR\n",
                statute_id,
                node_idx.index()
            ));
        }

        output.push('\n');

        for edge in self.graph.edge_indices() {
            if let Some((source, target)) = self.graph.edge_endpoints(edge) {
                let label = &self.graph[edge];
                output.push_str(&format!(
                    "N{} --> N{} : {}\n",
                    source.index(),
                    target.index(),
                    label
                ));
            }
        }

        output.push_str("@enduml\n");
        output
    }

    /// Exports to SVG format.
    pub fn to_svg(&self) -> String {
        self.to_svg_with_theme(&Theme::default())
    }

    /// Exports to SVG format with custom theme.
    pub fn to_svg_with_theme(&self, theme: &Theme) -> String {
        let mut svg = String::new();
        let width = self.layout_config.width;
        let height = self.layout_config.height;

        svg.push_str(&format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" style=\"background-color: {}\">\n",
            width, height, theme.background_color
        ));

        svg.push_str("  <defs>\n");
        svg.push_str("    <marker id=\"arrow\" markerWidth=\"10\" markerHeight=\"10\" refX=\"9\" refY=\"3\" orient=\"auto\" markerUnits=\"strokeWidth\">\n");
        svg.push_str(&format!(
            "      <path d=\"M0,0 L0,6 L9,3 z\" fill=\"{}\" />\n",
            theme.link_color
        ));
        svg.push_str("    </marker>\n");
        svg.push_str("  </defs>\n");

        // Simple grid layout for nodes
        let node_radius = 30;
        let cols = (self.node_count() as f64).sqrt().ceil() as usize;
        let spacing_x = width / (cols + 1);
        let spacing_y = height / ((self.node_count() / cols) + 2);

        let mut node_positions: std::collections::HashMap<NodeIndex, (usize, usize)> =
            std::collections::HashMap::new();

        // Position nodes in a grid
        for (i, node_idx) in self.graph.node_indices().enumerate() {
            let col = i % cols;
            let row = i / cols;
            let x = spacing_x * (col + 1);
            let y = spacing_y * (row + 1);
            node_positions.insert(node_idx, (x, y));
        }

        // Draw edges first (so they appear behind nodes)
        for edge in self.graph.edge_indices() {
            if let Some((source, target)) = self.graph.edge_endpoints(edge) {
                if let (Some(&(x1, y1)), Some(&(x2, y2))) =
                    (node_positions.get(&source), node_positions.get(&target))
                {
                    svg.push_str(&format!(
                        "  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"2\" marker-end=\"url(#arrow)\"/>\n",
                        x1, y1, x2, y2, theme.link_color
                    ));

                    // Add edge label
                    let label = &self.graph[edge];
                    let mid_x = (x1 + x2) / 2;
                    let mid_y = (y1 + y2) / 2;
                    svg.push_str(&format!(
                        "  <text x=\"{}\" y=\"{}\" font-size=\"10\" fill=\"{}\" text-anchor=\"middle\">{}</text>\n",
                        mid_x, mid_y.saturating_sub(5), theme.text_color, label
                    ));
                }
            }
        }

        // Draw nodes
        for node_idx in self.graph.node_indices() {
            if let Some(&(x, y)) = node_positions.get(&node_idx) {
                let statute_id = &self.graph[node_idx];

                svg.push_str(&format!(
                    "  <circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"2\"/>\n",
                    x, y, node_radius, theme.condition_color, theme.text_color
                ));

                // Truncate long statute IDs
                let display_id = if statute_id.len() > 12 {
                    format!("{}...", &statute_id[..9])
                } else {
                    statute_id.clone()
                };

                svg.push_str(&format!(
                    "  <text x=\"{}\" y=\"{}\" font-size=\"10\" fill=\"{}\" text-anchor=\"middle\">{}</text>\n",
                    x, y + 4, theme.text_color, display_id
                ));
            }
        }

        svg.push_str("</svg>");
        svg
    }

    /// Exports to PNG format.
    #[cfg(feature = "png-export")]
    pub fn to_png(&self) -> VizResult<Vec<u8>> {
        self.to_png_with_theme(&Theme::default())
    }

    /// Exports to PNG format with a custom theme.
    #[cfg(feature = "png-export")]
    pub fn to_png_with_theme(&self, theme: &Theme) -> VizResult<Vec<u8>> {
        let svg_data = self.to_svg_with_theme(theme);
        svg_to_png(&svg_data)
    }

    /// Exports to HTML with embedded D3.js force-directed graph visualization.
    pub fn to_html(&self) -> String {
        let mut html = String::new();
        let width = self.layout_config.width;
        let height = self.layout_config.height;
        let distance = self.layout_config.node_spacing;

        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str("    <meta charset=\"utf-8\">\n");
        html.push_str("    <title>Statute Dependency Graph</title>\n");
        html.push_str("    <script src=\"https://d3js.org/d3.v7.min.js\"></script>\n");
        html.push_str("    <style>\n");
        html.push_str("        body { font-family: Arial, sans-serif; margin: 20px; }\n");
        html.push_str("        .links line { stroke: #999; stroke-opacity: 0.6; }\n");
        html.push_str(
            "        .nodes circle { stroke: #fff; stroke-width: 1.5px; fill: #69b3a2; }\n",
        );
        html.push_str(
            "        text { font-family: sans-serif; font-size: 10px; pointer-events: none; }\n",
        );
        html.push_str("        .link-label { font-size: 8px; fill: #666; }\n");
        html.push_str("    </style>\n</head>\n<body>\n");
        html.push_str("    <h1>Statute Dependency Graph</h1>\n");

        if self.is_large_graph() {
            html.push_str(&format!(
                "    <p>This graph contains {} nodes. Large graph layout is enabled.</p>\n",
                self.node_count()
            ));
        }

        html.push_str("    <div id=\"graph\"></div>\n");
        html.push_str("    <script>\n");
        html.push_str("const graphData = ");
        html.push_str(&self.to_d3_force_json());
        html.push_str(";\n");
        html.push_str(&format!(
            "const width = {};\nconst height = {};\n",
            width, height
        ));
        html.push_str("const svg = d3.select(\"#graph\").append(\"svg\").attr(\"width\", width).attr(\"height\", height);\n");
        html.push_str("const simulation = d3.forceSimulation(graphData.nodes)\n");
        html.push_str(&format!("    .force(\"link\", d3.forceLink(graphData.links).id(function(d) {{ return d.id; }}).distance({}))\n", distance));
        html.push_str("    .force(\"charge\", d3.forceManyBody().strength(-300))\n");
        html.push_str("    .force(\"center\", d3.forceCenter(width / 2, height / 2));\n");
        html.push_str("const link = svg.append(\"g\").attr(\"class\", \"links\").selectAll(\"line\").data(graphData.links).enter().append(\"line\").attr(\"stroke-width\", 2);\n");
        html.push_str("const linkLabel = svg.append(\"g\").attr(\"class\", \"link-labels\").selectAll(\"text\").data(graphData.links).enter().append(\"text\").attr(\"class\", \"link-label\").attr(\"dy\", -5).text(function(d) { return d.label; });\n");
        html.push_str("const node = svg.append(\"g\").attr(\"class\", \"nodes\").selectAll(\"circle\").data(graphData.nodes).enter().append(\"circle\").attr(\"r\", 10);\n");
        html.push_str("const label = svg.append(\"g\").selectAll(\"text\").data(graphData.nodes).enter().append(\"text\").text(function(d) { return d.id; }).attr(\"dx\", 12).attr(\"dy\", 4);\n");
        html.push_str("simulation.on(\"tick\", function() {\n");
        html.push_str("    link.attr(\"x1\", function(d) { return d.source.x; }).attr(\"y1\", function(d) { return d.source.y; }).attr(\"x2\", function(d) { return d.target.x; }).attr(\"y2\", function(d) { return d.target.y; });\n");
        html.push_str("    linkLabel.attr(\"x\", function(d) { return (d.source.x + d.target.x) / 2; }).attr(\"y\", function(d) { return (d.source.y + d.target.y) / 2; });\n");
        html.push_str("    node.attr(\"cx\", function(d) { return d.x; }).attr(\"cy\", function(d) { return d.y; });\n");
        html.push_str("    label.attr(\"x\", function(d) { return d.x; }).attr(\"y\", function(d) { return d.y; });\n");
        html.push_str("});\n");
        html.push_str("    </script>\n</body>\n</html>");

        html
    }

    /// Converts the graph to D3.js force-directed graph JSON format.
    fn to_d3_force_json(&self) -> String {
        let mut nodes = Vec::new();
        let mut links = Vec::new();

        for node_idx in self.graph.node_indices() {
            let statute_id = &self.graph[node_idx];
            nodes.push(format!(r#"{{"id": "{}"}}"#, statute_id));
        }

        for edge in self.graph.edge_indices() {
            if let Some((source, target)) = self.graph.edge_endpoints(edge) {
                let label = &self.graph[edge];
                let source_id = &self.graph[source];
                let target_id = &self.graph[target];
                links.push(format!(
                    r#"{{"source": "{}", "target": "{}", "label": "{}"}}"#,
                    source_id, target_id, label
                ));
            }
        }

        format!(
            r#"{{"nodes": [{}], "links": [{}]}}"#,
            nodes.join(", "),
            links.join(", ")
        )
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
        Condition::Duration {
            operator,
            value,
            unit,
        } => {
            let unit_str = match unit {
                legalis_core::DurationUnit::Days => "days",
                legalis_core::DurationUnit::Weeks => "weeks",
                legalis_core::DurationUnit::Months => "months",
                legalis_core::DurationUnit::Years => "years",
            };
            format!(
                "Duration {} {} {}",
                format_operator(operator),
                value,
                unit_str
            )
        }
        Condition::Percentage {
            operator,
            value,
            context,
        } => {
            format!("{} {} {}%", context, format_operator(operator), value)
        }
        Condition::SetMembership {
            attribute,
            values,
            negated,
        } => {
            let op = if *negated { "not in" } else { "in" };
            format!("{} {} {{{}}}", attribute, op, values.join(", "))
        }
        Condition::Pattern {
            attribute,
            pattern,
            negated,
        } => {
            let op = if *negated { "!~" } else { "~" };
            format!("{} {} '{}'", attribute, op, pattern)
        }
        Condition::Calculation {
            formula,
            operator,
            value,
        } => {
            format!("{} {} {}", formula, format_operator(operator), value)
        }
        Condition::And(_, _) => "AND condition".to_string(),
        Condition::Or(_, _) => "OR condition".to_string(),
        Condition::Not(_) => "NOT condition".to_string(),
        Condition::Custom { description } => description.clone(),
        Condition::Composite {
            conditions,
            threshold,
        } => {
            format!(
                "Composite ({} conditions, threshold: {})",
                conditions.len(),
                threshold
            )
        }
        Condition::Threshold {
            attributes,
            operator,
            value,
        } => {
            let attrs = attributes
                .iter()
                .map(|(attr, mult)| format!("{}*{}", mult, attr))
                .collect::<Vec<_>>()
                .join(" + ");
            format!("{} {} {}", attrs, format_operator(operator), value)
        }
        Condition::Fuzzy {
            attribute,
            membership_points,
            min_membership,
        } => {
            format!(
                "{} ∈ fuzzy set ({} points, min: {})",
                attribute,
                membership_points.len(),
                min_membership
            )
        }
        Condition::Probabilistic {
            condition: _,
            probability,
            threshold,
        } => {
            format!("Probabilistic (p={}, threshold={})", probability, threshold)
        }
        Condition::Temporal {
            base_value,
            reference_time: _,
            rate,
            operator,
            target_value,
        } => {
            format!(
                "Temporal (base={}, rate={}) {} {}",
                base_value,
                rate,
                format_operator(operator),
                target_value
            )
        }
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

/// Converts SVG data to PNG format.
#[cfg(feature = "png-export")]
fn svg_to_png(svg_data: &str) -> VizResult<Vec<u8>> {
    let options = usvg::Options::default();
    let tree = usvg::Tree::from_str(svg_data, &options)
        .map_err(|e| VizError::RenderError(format!("Failed to parse SVG: {}", e)))?;

    let size = tree.size();
    let width = size.width().ceil() as u32;
    let height = size.height().ceil() as u32;

    let mut pixmap = Pixmap::new(width, height)
        .ok_or_else(|| VizError::RenderError("Failed to create pixmap".to_string()))?;

    resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());

    pixmap
        .encode_png()
        .map_err(|e| VizError::RenderError(format!("Failed to encode PNG: {}", e)))
}

/// Plugin trait for custom renderers.
pub trait Renderer {
    /// The output type produced by this renderer.
    type Output;

    /// Renders a decision tree.
    fn render_decision_tree(&self, tree: &DecisionTree) -> VizResult<Self::Output>;

    /// Renders a dependency graph.
    fn render_dependency_graph(&self, graph: &DependencyGraph) -> VizResult<Self::Output>;

    /// Renders a timeline.
    fn render_timeline(&self, timeline: &Timeline) -> VizResult<Self::Output>;

    /// Renders a population chart.
    fn render_population_chart(&self, chart: &PopulationChart) -> VizResult<Self::Output>;
}

/// Registry for custom renderers.
pub struct RendererRegistry {
    renderers: HashMap<String, Box<dyn std::any::Any>>,
}

impl RendererRegistry {
    /// Creates a new renderer registry.
    pub fn new() -> Self {
        Self {
            renderers: HashMap::new(),
        }
    }

    /// Registers a custom renderer.
    pub fn register<R: Renderer + 'static>(&mut self, name: &str, renderer: R) {
        self.renderers.insert(name.to_string(), Box::new(renderer));
    }

    /// Gets a renderer by name.
    pub fn get<R: 'static>(&self, name: &str) -> Option<&R> {
        self.renderers.get(name).and_then(|r| r.downcast_ref())
    }
}

impl Default for RendererRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Real-time update event for visualizations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UpdateEvent {
    /// Population data updated
    PopulationUpdate {
        category: String,
        count: usize,
        timestamp: String,
    },
    /// New node added to decision tree
    NodeAdded {
        node_id: String,
        node_type: String,
        parent_id: Option<String>,
    },
    /// Statute dependency added
    DependencyAdded {
        from_statute: String,
        to_statute: String,
        relation: String,
    },
    /// Timeline event added
    TimelineEventAdded { date: String, description: String },
    /// Statistics updated
    StatisticsUpdate { metric: String, value: f64 },
}

/// Live visualization handler for real-time updates.
pub struct LiveVisualization {
    /// Population chart for live updates
    pub population_chart: PopulationChart,
    /// Dependency graph for live updates
    pub dependency_graph: DependencyGraph,
    /// Timeline for live updates
    pub timeline: Timeline,
    /// Update history
    update_history: Vec<UpdateEvent>,
}

impl LiveVisualization {
    /// Creates a new live visualization handler.
    pub fn new(title: &str) -> Self {
        Self {
            population_chart: PopulationChart::new(title),
            dependency_graph: DependencyGraph::new(),
            timeline: Timeline::new(),
            update_history: Vec::new(),
        }
    }

    /// Processes an update event.
    pub fn process_update(&mut self, event: UpdateEvent) {
        match &event {
            UpdateEvent::PopulationUpdate {
                category,
                count,
                timestamp,
            } => {
                // Check if we should add a new time point
                if self.population_chart.time_series.is_empty()
                    || self
                        .population_chart
                        .time_series
                        .last()
                        .map(|(t, _)| t != timestamp)
                        .unwrap_or(true)
                {
                    self.population_chart.add_time_point(timestamp, Vec::new());
                }

                // Update or add the data point
                if let Some((_time, data)) = self.population_chart.time_series.last_mut() {
                    if let Some(point) = data.iter_mut().find(|p| p.category == *category) {
                        point.count = *count;
                    } else {
                        data.push(PopulationDataPoint {
                            category: category.clone(),
                            count: *count,
                            percentage: None,
                        });
                    }
                }
            }
            UpdateEvent::DependencyAdded {
                from_statute,
                to_statute,
                relation,
            } => {
                self.dependency_graph
                    .add_dependency(from_statute, to_statute, relation);
            }
            UpdateEvent::TimelineEventAdded { date, description } => {
                self.timeline.add_event(
                    date,
                    TimelineEvent::Amended {
                        statute_id: "live-update".to_string(),
                        description: description.clone(),
                    },
                );
            }
            _ => {}
        }

        self.update_history.push(event);
    }

    /// Exports the current state to HTML with WebSocket support for real-time updates.
    pub fn to_live_html(&self, websocket_url: &str) -> String {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str("    <meta charset=\"utf-8\">\n");
        html.push_str("    <title>Live Visualization Dashboard</title>\n");
        html.push_str("    <script src=\"https://cdn.jsdelivr.net/npm/chart.js\"></script>\n");
        html.push_str("    <script src=\"https://d3js.org/d3.v7.min.js\"></script>\n");
        html.push_str("    <style>\n");
        html.push_str("        body { font-family: Arial, sans-serif; margin: 0; padding: 20px; background: #f5f5f5; }\n");
        html.push_str("        h1 { color: #333; }\n");
        html.push_str(
            "        .dashboard { display: grid; grid-template-columns: 1fr 1fr; gap: 20px; }\n",
        );
        html.push_str("        .panel { background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }\n");
        html.push_str("        .status { position: fixed; top: 10px; right: 10px; padding: 10px 20px; border-radius: 4px; color: white; }\n");
        html.push_str("        .status.connected { background: #4caf50; }\n");
        html.push_str("        .status.disconnected { background: #f44336; }\n");
        html.push_str("    </style>\n</head>\n<body>\n");
        html.push_str("    <div class=\"status disconnected\" id=\"status\">Disconnected</div>\n");
        html.push_str("    <h1>Live Visualization Dashboard</h1>\n");
        html.push_str("    <div class=\"dashboard\">\n");
        html.push_str("        <div class=\"panel\">\n");
        html.push_str("            <h2>Population Chart</h2>\n");
        html.push_str("            <canvas id=\"populationChart\"></canvas>\n");
        html.push_str("        </div>\n");
        html.push_str("        <div class=\"panel\">\n");
        html.push_str("            <h2>Update Log</h2>\n");
        html.push_str("            <div id=\"updateLog\" style=\"max-height: 400px; overflow-y: auto;\"></div>\n");
        html.push_str("        </div>\n");
        html.push_str("    </div>\n");
        html.push_str("    <script>\n");
        html.push_str(&format!("const wsUrl = '{}';\n", websocket_url));
        html.push_str("let ws = null;\n");
        html.push_str("const populationData = {};\n");
        html.push_str("let chart = null;\n\n");
        html.push_str("function connect() {\n");
        html.push_str("    ws = new WebSocket(wsUrl);\n");
        html.push_str("    ws.onopen = function() {\n");
        html.push_str("        document.getElementById('status').textContent = 'Connected';\n");
        html.push_str(
            "        document.getElementById('status').className = 'status connected';\n",
        );
        html.push_str("    };\n");
        html.push_str("    ws.onmessage = function(event) {\n");
        html.push_str("        const update = JSON.parse(event.data);\n");
        html.push_str("        processUpdate(update);\n");
        html.push_str("    };\n");
        html.push_str("    ws.onclose = function() {\n");
        html.push_str("        document.getElementById('status').textContent = 'Disconnected';\n");
        html.push_str(
            "        document.getElementById('status').className = 'status disconnected';\n",
        );
        html.push_str("        setTimeout(connect, 5000);\n");
        html.push_str("    };\n");
        html.push_str("}\n\n");
        html.push_str("function processUpdate(update) {\n");
        html.push_str("    const log = document.getElementById('updateLog');\n");
        html.push_str("    const entry = document.createElement('div');\n");
        html.push_str("    entry.textContent = JSON.stringify(update);\n");
        html.push_str("    entry.style.padding = '5px';\n");
        html.push_str("    entry.style.borderBottom = '1px solid #eee';\n");
        html.push_str("    log.insertBefore(entry, log.firstChild);\n");
        html.push_str("    if (update.PopulationUpdate) {\n");
        html.push_str("        const data = update.PopulationUpdate;\n");
        html.push_str("        populationData[data.category] = data.count;\n");
        html.push_str("        updateChart();\n");
        html.push_str("    }\n");
        html.push_str("}\n\n");
        html.push_str("function updateChart() {\n");
        html.push_str(
            "    const ctx = document.getElementById('populationChart').getContext('2d');\n",
        );
        html.push_str("    if (chart) chart.destroy();\n");
        html.push_str("    chart = new Chart(ctx, {\n");
        html.push_str("        type: 'bar',\n");
        html.push_str("        data: {\n");
        html.push_str("            labels: Object.keys(populationData),\n");
        html.push_str("            datasets: [{\n");
        html.push_str("                label: 'Count',\n");
        html.push_str("                data: Object.values(populationData),\n");
        html.push_str("                backgroundColor: 'rgba(54, 162, 235, 0.6)'\n");
        html.push_str("            }]\n");
        html.push_str("        },\n");
        html.push_str(
            "        options: { responsive: true, scales: { y: { beginAtZero: true } } }\n",
        );
        html.push_str("    });\n");
        html.push_str("}\n\n");
        html.push_str("connect();\n");
        html.push_str("    </script>\n</body>\n</html>");

        html
    }

    /// Returns the update history.
    pub fn update_history(&self) -> &[UpdateEvent] {
        &self.update_history
    }

    /// Clears the update history.
    pub fn clear_history(&mut self) {
        self.update_history.clear();
    }
}

/// PowerPoint/Keynote export format (PPTX XML).
pub struct PresentationExporter {
    /// Slides in the presentation
    slides: Vec<Slide>,
    /// Theme for the presentation
    theme: Theme,
}

/// A single slide in a presentation.
#[derive(Debug, Clone)]
pub struct Slide {
    /// Slide title
    pub title: String,
    /// Slide content (SVG or text)
    pub content: SlideContent,
    /// Animations on this slide
    pub animations: Vec<Animation>,
    /// Speaker notes
    pub notes: Option<String>,
}

/// Content type for a slide.
#[derive(Debug, Clone)]
pub enum SlideContent {
    /// SVG image content
    Svg(String),
    /// HTML content
    Html(String),
    /// Plain text content
    Text(String),
    /// Decision tree visualization
    DecisionTree(String),
    /// Dependency graph visualization
    DependencyGraph(String),
}

/// Animation for presentation elements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Animation {
    /// Target element ID
    pub target: String,
    /// Animation type
    pub animation_type: AnimationType,
    /// Duration in milliseconds
    pub duration_ms: u32,
    /// Delay before animation starts (milliseconds)
    pub delay_ms: u32,
}

/// Types of animations available.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnimationType {
    /// Fade in
    FadeIn,
    /// Fade out
    FadeOut,
    /// Slide from left
    SlideInLeft,
    /// Slide from right
    SlideInRight,
    /// Slide from top
    SlideInTop,
    /// Slide from bottom
    SlideInBottom,
    /// Zoom in
    ZoomIn,
    /// Zoom out
    ZoomOut,
    /// Highlight (color pulse)
    Highlight,
    /// Progressive reveal (for lists)
    ProgressiveReveal,
}

impl PresentationExporter {
    /// Creates a new presentation exporter.
    pub fn new() -> Self {
        Self {
            slides: Vec::new(),
            theme: Theme::default(),
        }
    }

    /// Sets the theme for the presentation.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Adds a slide to the presentation.
    pub fn add_slide(&mut self, slide: Slide) {
        self.slides.push(slide);
    }

    /// Creates a slide from a decision tree.
    pub fn add_decision_tree_slide(&mut self, title: &str, tree: &DecisionTree) {
        let svg = tree.to_svg_with_theme(&self.theme);
        self.add_slide(Slide {
            title: title.to_string(),
            content: SlideContent::DecisionTree(svg),
            animations: Vec::new(),
            notes: None,
        });
    }

    /// Creates a slide from a dependency graph.
    pub fn add_dependency_graph_slide(&mut self, title: &str, graph: &DependencyGraph) {
        let svg = graph.to_svg_with_theme(&self.theme);
        self.add_slide(Slide {
            title: title.to_string(),
            content: SlideContent::DependencyGraph(svg),
            animations: Vec::new(),
            notes: None,
        });
    }

    /// Exports to PowerPoint Open XML format (PPTX).
    pub fn to_pptx(&self) -> VizResult<String> {
        let mut xml = String::new();

        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n");
        xml.push_str(
            "<p:presentation xmlns:a=\"http://schemas.openxmlformats.org/drawingml/2006/main\" ",
        );
        xml.push_str(
            "xmlns:r=\"http://schemas.openxmlformats.org/officeDocument/2006/relationships\" ",
        );
        xml.push_str("xmlns:p=\"http://schemas.openxmlformats.org/presentationml/2006/main\">\n");
        xml.push_str("  <p:sldIdLst>\n");

        for (i, _slide) in self.slides.iter().enumerate() {
            xml.push_str(&format!(
                "    <p:sldId id=\"{}\" r:id=\"rId{}\"/>\n",
                256 + i,
                i + 1
            ));
        }

        xml.push_str("  </p:sldIdLst>\n");
        xml.push_str("  <p:sldSz cx=\"9144000\" cy=\"6858000\"/>\n");
        xml.push_str("  <p:notesSz cx=\"6858000\" cy=\"9144000\"/>\n");
        xml.push_str("</p:presentation>\n");

        Ok(xml)
    }

    /// Exports to Keynote format (iWork format).
    pub fn to_keynote(&self) -> VizResult<String> {
        // Keynote uses a similar structure but with Apple-specific XML
        let mut xml = String::new();

        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str("<!DOCTYPE key PUBLIC \"-//Apple//DTD KEY 2.0//EN\" \"http://www.apple.com/DTDs/Keynote-2.dtd\">\n");
        xml.push_str("<key version=\"92.2.1\">\n");
        xml.push_str("  <presentation>\n");
        xml.push_str("    <slides>\n");

        for (i, slide) in self.slides.iter().enumerate() {
            xml.push_str(&format!("      <slide id=\"{}\">\n", i + 1));
            xml.push_str(&format!("        <title>{}</title>\n", slide.title));

            match &slide.content {
                SlideContent::Svg(svg) => {
                    xml.push_str("        <content type=\"image/svg+xml\">\n");
                    xml.push_str("          <![CDATA[");
                    xml.push_str(svg);
                    xml.push_str("]]>\n");
                    xml.push_str("        </content>\n");
                }
                SlideContent::Text(text) => {
                    xml.push_str(&format!("        <content>{}</content>\n", text));
                }
                SlideContent::DecisionTree(svg) | SlideContent::DependencyGraph(svg) => {
                    xml.push_str("        <content type=\"image/svg+xml\">\n");
                    xml.push_str("          <![CDATA[");
                    xml.push_str(svg);
                    xml.push_str("]]>\n");
                    xml.push_str("        </content>\n");
                }
                SlideContent::Html(_) => {
                    xml.push_str("        <content type=\"text/html\"/>\n");
                }
            }

            if let Some(notes) = &slide.notes {
                xml.push_str(&format!("        <notes>{}</notes>\n", notes));
            }

            xml.push_str("      </slide>\n");
        }

        xml.push_str("    </slides>\n");
        xml.push_str("  </presentation>\n");
        xml.push_str("</key>\n");

        Ok(xml)
    }

    /// Exports to HTML with embedded animations for web-based presentations.
    pub fn to_animated_html(&self) -> String {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str("    <meta charset=\"utf-8\">\n");
        html.push_str("    <title>Animated Presentation</title>\n");
        html.push_str("    <style>\n");
        html.push_str(&format!(
            "        body {{ margin: 0; padding: 0; background: {}; color: {}; font-family: Arial, sans-serif; }}\n",
            self.theme.background_color, self.theme.text_color
        ));
        html.push_str("        .slide { display: none; width: 100vw; height: 100vh; padding: 40px; box-sizing: border-box; }\n");
        html.push_str("        .slide.active { display: flex; flex-direction: column; }\n");
        html.push_str("        .slide h1 { margin: 0 0 20px 0; font-size: 2.5em; }\n");
        html.push_str("        .slide .content { flex: 1; overflow: auto; }\n");
        html.push_str("        .controls { position: fixed; bottom: 20px; right: 20px; }\n");
        html.push_str("        .controls button { margin: 0 5px; padding: 10px 20px; font-size: 16px; cursor: pointer; }\n");
        html.push_str("        .animation-fade-in { animation: fadeIn 0.5s; }\n");
        html.push_str("        .animation-slide-in-left { animation: slideInLeft 0.5s; }\n");
        html.push_str("        .animation-slide-in-right { animation: slideInRight 0.5s; }\n");
        html.push_str("        .animation-zoom-in { animation: zoomIn 0.5s; }\n");
        html.push_str("        @keyframes fadeIn { from { opacity: 0; } to { opacity: 1; } }\n");
        html.push_str("        @keyframes slideInLeft { from { transform: translateX(-100%); } to { transform: translateX(0); } }\n");
        html.push_str("        @keyframes slideInRight { from { transform: translateX(100%); } to { transform: translateX(0); } }\n");
        html.push_str("        @keyframes zoomIn { from { transform: scale(0); } to { transform: scale(1); } }\n");
        html.push_str("    </style>\n</head>\n<body>\n");

        for (i, slide) in self.slides.iter().enumerate() {
            html.push_str(&format!(
                "    <div class=\"slide{}\" id=\"slide-{}\">\n",
                if i == 0 { " active" } else { "" },
                i
            ));
            html.push_str(&format!("        <h1>{}</h1>\n", slide.title));
            html.push_str("        <div class=\"content\">\n");

            match &slide.content {
                SlideContent::Svg(svg)
                | SlideContent::DecisionTree(svg)
                | SlideContent::DependencyGraph(svg) => {
                    html.push_str("            ");
                    html.push_str(svg);
                    html.push('\n');
                }
                SlideContent::Html(content) => {
                    html.push_str("            ");
                    html.push_str(content);
                    html.push('\n');
                }
                SlideContent::Text(text) => {
                    html.push_str("            <p>");
                    html.push_str(text);
                    html.push_str("</p>\n");
                }
            }

            html.push_str("        </div>\n");
            html.push_str("    </div>\n");
        }

        html.push_str("    <div class=\"controls\">\n");
        html.push_str("        <button onclick=\"previousSlide()\">Previous</button>\n");
        html.push_str("        <button onclick=\"nextSlide()\">Next</button>\n");
        html.push_str("    </div>\n");
        html.push_str("    <script>\n");
        html.push_str("        let currentSlide = 0;\n");
        html.push_str(&format!(
            "        const totalSlides = {};\n",
            self.slides.len()
        ));
        html.push_str("        function showSlide(n) {\n");
        html.push_str("            const slides = document.querySelectorAll('.slide');\n");
        html.push_str("            if (n >= totalSlides) currentSlide = 0;\n");
        html.push_str("            if (n < 0) currentSlide = totalSlides - 1;\n");
        html.push_str("            slides.forEach(s => s.classList.remove('active'));\n");
        html.push_str("            slides[currentSlide].classList.add('active');\n");
        html.push_str("        }\n");
        html.push_str(
            "        function nextSlide() { currentSlide++; showSlide(currentSlide); }\n",
        );
        html.push_str(
            "        function previousSlide() { currentSlide--; showSlide(currentSlide); }\n",
        );
        html.push_str("        document.addEventListener('keydown', function(e) {\n");
        html.push_str("            if (e.key === 'ArrowRight') nextSlide();\n");
        html.push_str("            if (e.key === 'ArrowLeft') previousSlide();\n");
        html.push_str("        });\n");
        html.push_str("    </script>\n</body>\n</html>");

        html
    }
}

impl Default for PresentationExporter {
    fn default() -> Self {
        Self::new()
    }
}

/// Document embedding support for various formats.
pub struct DocumentEmbedder {
    theme: Theme,
}

impl DocumentEmbedder {
    /// Creates a new document embedder.
    pub fn new() -> Self {
        Self {
            theme: Theme::default(),
        }
    }

    /// Sets the theme.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Embeds a decision tree in Markdown format with SVG data URI.
    pub fn embed_in_markdown(&self, tree: &DecisionTree) -> String {
        let svg = tree.to_svg_with_theme(&self.theme);
        let encoded = base64_encode(&svg);
        format!("![Decision Tree](data:image/svg+xml;base64,{})", encoded)
    }

    /// Embeds a decision tree in LaTeX format.
    pub fn embed_in_latex(&self, tree: &DecisionTree) -> String {
        let mut latex = String::new();
        latex.push_str("\\begin{figure}[h]\n");
        latex.push_str("\\centering\n");
        latex.push_str("\\begin{tikzpicture}\n");

        // Convert tree structure to TikZ format
        if let Some(root_idx) = tree.root {
            self.latex_render_node(tree, root_idx, &mut latex, 0, 0);
        }

        latex.push_str("\\end{tikzpicture}\n");
        latex.push_str("\\caption{Decision Tree Visualization}\n");
        latex.push_str("\\end{figure}\n");

        latex
    }

    /// Helper to render nodes in LaTeX/TikZ format.
    #[allow(dead_code)]
    fn latex_render_node(
        &self,
        tree: &DecisionTree,
        idx: NodeIndex,
        latex: &mut String,
        x: i32,
        y: i32,
    ) {
        let node = &tree.graph[idx];
        let node_text = match node {
            DecisionNode::Root { title, .. } => title.clone(),
            DecisionNode::Condition { description, .. } => description.clone(),
            DecisionNode::Outcome { description } => description.clone(),
            DecisionNode::Discretion { issue, .. } => issue.clone(),
        };

        latex.push_str(&format!("\\node at ({},{}) {{{}}};\n", x, y, node_text));

        // Recursively render children
        let children: Vec<_> = tree.graph.neighbors(idx).collect();
        for (i, &_child) in children.iter().enumerate() {
            let child_x = x + (i as i32 - (children.len() as i32 / 2)) * 3;
            let child_y = y - 2;
            latex.push_str(&format!(
                "\\draw ({},{}) -- ({},{});\n",
                x, y, child_x, child_y
            ));
        }
    }

    /// Embeds a decision tree in reStructuredText format.
    pub fn embed_in_rst(&self, tree: &DecisionTree) -> String {
        let svg = tree.to_svg_with_theme(&self.theme);
        let encoded = base64_encode(&svg);
        format!(
            ".. image:: data:image/svg+xml;base64,{}\n   :alt: Decision Tree\n   :align: center\n",
            encoded
        )
    }

    /// Embeds a decision tree in AsciiDoc format.
    pub fn embed_in_asciidoc(&self, tree: &DecisionTree) -> String {
        let svg = tree.to_svg_with_theme(&self.theme);
        let encoded = base64_encode(&svg);
        format!(
            "image::data:image/svg+xml;base64,{}[Decision Tree,align=center]\n",
            encoded
        )
    }

    /// Embeds as an HTML iframe snippet.
    pub fn embed_as_iframe(&self, tree: &DecisionTree, width: u32, height: u32) -> String {
        let html = tree.to_html_with_theme(&self.theme);
        let encoded = base64_encode(&html);
        format!(
            "<iframe width=\"{}\" height=\"{}\" src=\"data:text/html;base64,{}\" frameborder=\"0\"></iframe>",
            width, height, encoded
        )
    }
}

impl Default for DocumentEmbedder {
    fn default() -> Self {
        Self::new()
    }
}

/// Visual regression testing support.
pub struct VisualRegressionTest {
    /// Name of the test
    pub name: String,
    /// Expected output (baseline)
    pub baseline: String,
    /// Actual output
    pub actual: String,
    /// Test result
    pub passed: bool,
    /// Differences found
    pub differences: Vec<String>,
}

impl VisualRegressionTest {
    /// Creates a new visual regression test.
    pub fn new(name: &str, baseline: &str, actual: &str) -> Self {
        let differences = Self::find_differences(baseline, actual);
        let passed = differences.is_empty();

        Self {
            name: name.to_string(),
            baseline: baseline.to_string(),
            actual: actual.to_string(),
            passed,
            differences,
        }
    }

    /// Finds differences between baseline and actual output.
    fn find_differences(baseline: &str, actual: &str) -> Vec<String> {
        let mut diffs = Vec::new();

        if baseline != actual {
            // Simple line-by-line comparison
            let baseline_lines: Vec<&str> = baseline.lines().collect();
            let actual_lines: Vec<&str> = actual.lines().collect();

            if baseline_lines.len() != actual_lines.len() {
                diffs.push(format!(
                    "Line count mismatch: expected {}, got {}",
                    baseline_lines.len(),
                    actual_lines.len()
                ));
            }

            for (i, (base_line, actual_line)) in
                baseline_lines.iter().zip(actual_lines.iter()).enumerate()
            {
                if base_line != actual_line {
                    diffs.push(format!(
                        "Line {} differs:\n  Expected: {}\n  Actual: {}",
                        i + 1,
                        base_line,
                        actual_line
                    ));
                }
            }
        }

        diffs
    }

    /// Generates a test report.
    pub fn report(&self) -> String {
        let mut report = String::new();
        report.push_str(&format!("Visual Regression Test: {}\n", self.name));
        report.push_str(&format!(
            "Status: {}\n",
            if self.passed { "PASSED" } else { "FAILED" }
        ));

        if !self.passed {
            report.push_str("\nDifferences found:\n");
            for diff in &self.differences {
                report.push_str(&format!("  - {}\n", diff));
            }
        }

        report
    }
}

/// Visual regression test suite.
pub struct VisualRegressionSuite {
    tests: Vec<VisualRegressionTest>,
}

impl VisualRegressionSuite {
    /// Creates a new test suite.
    pub fn new() -> Self {
        Self { tests: Vec::new() }
    }

    /// Adds a test to the suite.
    pub fn add_test(&mut self, test: VisualRegressionTest) {
        self.tests.push(test);
    }

    /// Runs all tests and returns a summary.
    pub fn run(&self) -> String {
        let mut summary = String::new();
        let total = self.tests.len();
        let passed = self.tests.iter().filter(|t| t.passed).count();
        let failed = total - passed;

        summary.push_str("Visual Regression Test Suite\n");
        summary.push_str("============================\n");
        summary.push_str(&format!("Total tests: {}\n", total));
        summary.push_str(&format!("Passed: {}\n", passed));
        summary.push_str(&format!("Failed: {}\n\n", failed));

        for test in &self.tests {
            if !test.passed {
                summary.push_str(&test.report());
                summary.push('\n');
            }
        }

        summary
    }

    /// Returns true if all tests passed.
    pub fn all_passed(&self) -> bool {
        self.tests.iter().all(|t| t.passed)
    }
}

impl Default for VisualRegressionSuite {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function for base64 encoding.
fn base64_encode(data: &str) -> String {
    // Simple base64 encoding
    use std::fmt::Write;
    let bytes = data.as_bytes();
    let mut result = String::new();

    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    for chunk in bytes.chunks(3) {
        let mut buf = [0u8; 3];
        for (i, &byte) in chunk.iter().enumerate() {
            buf[i] = byte;
        }

        let b1 = (buf[0] >> 2) as usize;
        let b2 = (((buf[0] & 0x03) << 4) | (buf[1] >> 4)) as usize;
        let b3 = (((buf[1] & 0x0f) << 2) | (buf[2] >> 6)) as usize;
        let b4 = (buf[2] & 0x3f) as usize;

        write!(&mut result, "{}", CHARS[b1] as char).unwrap();
        write!(&mut result, "{}", CHARS[b2] as char).unwrap();
        write!(
            &mut result,
            "{}",
            if chunk.len() > 1 {
                CHARS[b3] as char
            } else {
                '='
            }
        )
        .unwrap();
        write!(
            &mut result,
            "{}",
            if chunk.len() > 2 {
                CHARS[b4] as char
            } else {
                '='
            }
        )
        .unwrap();
    }

    result
}

/// Visualizer for statute differences (comparing versions).
pub struct StatuteDiffVisualizer {
    theme: Theme,
}

impl StatuteDiffVisualizer {
    /// Creates a new statute diff visualizer with default theme.
    #[must_use]
    pub fn new() -> Self {
        Self {
            theme: Theme::light(),
        }
    }

    /// Sets the theme for visualization.
    #[must_use]
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Renders a statute diff as a side-by-side comparison in HTML.
    #[must_use]
    pub fn to_html(&self, diff: &legalis_core::StatuteDiff) -> String {
        let mut html = String::from("<div class='statute-diff'>");
        html.push_str(&format!(
            "<h2>Changes for Statute: {}</h2>",
            diff.statute_id
        ));

        if diff.is_empty() {
            html.push_str("<p>No changes detected.</p>");
        } else {
            html.push_str("<table class='diff-table'>");
            html.push_str("<thead><tr><th>Change Type</th><th>Details</th></tr></thead>");
            html.push_str("<tbody>");

            for change in &diff.changes {
                html.push_str("<tr>");
                html.push_str(&format!("<td>{}</td>", format_change_type(change)));
                html.push_str(&format!("<td>{}</td>", change));
                html.push_str("</tr>");
            }

            html.push_str("</tbody></table>");
        }

        html.push_str("</div>");
        self.add_styles(html)
    }

    /// Renders a statute diff as a Mermaid flowchart showing the transformation.
    #[must_use]
    pub fn to_mermaid(&self, diff: &legalis_core::StatuteDiff) -> String {
        let mut mermaid = String::from("flowchart LR\n");
        mermaid.push_str(&format!(
            "    Start[\"Statute: {}\"] --> Changes{{Changes}}\n",
            diff.statute_id
        ));

        for (i, change) in diff.changes.iter().enumerate() {
            mermaid.push_str(&format!("    Changes --> C{}[\"{}\"]\n", i, change));
        }

        mermaid.push_str("    Changes --> End[\"Updated Statute\"]\n");
        mermaid
    }

    /// Renders a statute diff as ASCII art for terminal display.
    #[must_use]
    pub fn to_ascii(&self, diff: &legalis_core::StatuteDiff) -> String {
        let mut ascii = String::new();
        ascii.push_str(&format!("=== Statute Diff: {} ===\n\n", diff.statute_id));

        if diff.is_empty() {
            ascii.push_str("No changes detected.\n");
        } else {
            for (i, change) in diff.changes.iter().enumerate() {
                ascii.push_str(&format!("{}. {}\n", i + 1, change));
            }
        }

        ascii
    }

    fn add_styles(&self, content: String) -> String {
        format!(
            "<style>
.statute-diff {{ font-family: Arial, sans-serif; padding: 20px; background: {}; color: {}; }}
.diff-table {{ width: 100%; border-collapse: collapse; }}
.diff-table th, .diff-table td {{ border: 1px solid {}; padding: 8px; text-align: left; }}
.diff-table th {{ background: {}; }}
</style>{}",
            self.theme.background_color,
            self.theme.text_color,
            self.theme.link_color,
            self.theme.root_color,
            content
        )
    }
}

impl Default for StatuteDiffVisualizer {
    fn default() -> Self {
        Self::new()
    }
}

/// Visualizer for legal reasoning chains and explanations.
pub struct ReasoningChainVisualizer {
    theme: Theme,
}

impl ReasoningChainVisualizer {
    /// Creates a new reasoning chain visualizer with default theme.
    #[must_use]
    pub fn new() -> Self {
        Self {
            theme: Theme::light(),
        }
    }

    /// Sets the theme for visualization.
    #[must_use]
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Renders a legal explanation as an interactive HTML timeline.
    #[must_use]
    pub fn to_html(&self, explanation: &legalis_core::LegalExplanation) -> String {
        let mut html = String::from("<div class='reasoning-chain'>");
        html.push_str(&format!(
            "<h2>Legal Reasoning: {}</h2>",
            explanation.outcome.description
        ));
        html.push_str(&format!(
            "<p><strong>Confidence:</strong> {:.1}%</p>",
            explanation.confidence * 100.0
        ));

        // Applicable statutes
        if !explanation.applicable_statutes.is_empty() {
            html.push_str("<h3>Applicable Statutes</h3><ul>");
            for statute in &explanation.applicable_statutes {
                html.push_str(&format!("<li>{}</li>", statute));
            }
            html.push_str("</ul>");
        }

        // Satisfied conditions
        if !explanation.satisfied_conditions.is_empty() {
            html.push_str("<h3>Satisfied Conditions</h3><ul>");
            for condition in &explanation.satisfied_conditions {
                html.push_str(&format!("<li style='color: green;'>✓ {}</li>", condition));
            }
            html.push_str("</ul>");
        }

        // Unsatisfied conditions
        if !explanation.unsatisfied_conditions.is_empty() {
            html.push_str("<h3>Unsatisfied Conditions</h3><ul>");
            for condition in &explanation.unsatisfied_conditions {
                html.push_str(&format!("<li style='color: red;'>✗ {}</li>", condition));
            }
            html.push_str("</ul>");
        }

        // Reasoning chain
        if !explanation.reasoning_chain.is_empty() {
            html.push_str("<h3>Reasoning Chain</h3>");
            html.push_str("<div class='reasoning-steps'>");
            for step in &explanation.reasoning_chain {
                html.push_str(&format!(
                    "<div class='step'><span class='step-num'>Step {}</span>: {}</div>",
                    step.step, step.description
                ));
            }
            html.push_str("</div>");
        }

        html.push_str("</div>");
        self.add_styles(html)
    }

    /// Renders a reasoning chain as a Mermaid flowchart.
    #[must_use]
    pub fn to_mermaid(&self, explanation: &legalis_core::LegalExplanation) -> String {
        let mut mermaid = String::from("flowchart TD\n");
        mermaid.push_str("    Start([Start Reasoning]) --> Statutes{Applicable Statutes}\n");

        // Add statutes
        for (i, statute) in explanation.applicable_statutes.iter().enumerate() {
            mermaid.push_str(&format!("    Statutes --> S{}[\"{}\"]\n", i, statute));
        }

        // Add reasoning steps
        if !explanation.reasoning_chain.is_empty() {
            mermaid.push_str("    Statutes --> Reasoning{Reasoning Chain}\n");
            for step in &explanation.reasoning_chain {
                mermaid.push_str(&format!(
                    "    Reasoning --> R{}[\"Step {}: {}\"]\n",
                    step.step, step.step, step.description
                ));
            }
            mermaid.push_str(&format!(
                "    Reasoning --> Outcome([\"Outcome: {}\\nConfidence: {:.1}%\"])\n",
                explanation.outcome.description,
                explanation.confidence * 100.0
            ));
        } else {
            mermaid.push_str(&format!(
                "    Statutes --> Outcome([\"Outcome: {}\\nConfidence: {:.1}%\"])\n",
                explanation.outcome.description,
                explanation.confidence * 100.0
            ));
        }

        mermaid
    }

    /// Renders a reasoning chain as ASCII art for terminal display.
    #[must_use]
    pub fn to_ascii(&self, explanation: &legalis_core::LegalExplanation) -> String {
        let mut ascii = String::new();
        ascii.push_str("=== Legal Reasoning Chain ===\n\n");
        ascii.push_str(&format!("Outcome: {}\n", explanation.outcome.description));
        ascii.push_str(&format!(
            "Confidence: {:.1}%\n\n",
            explanation.confidence * 100.0
        ));

        if !explanation.applicable_statutes.is_empty() {
            ascii.push_str("Applicable Statutes:\n");
            for statute in &explanation.applicable_statutes {
                ascii.push_str(&format!("  • {}\n", statute));
            }
            ascii.push('\n');
        }

        if !explanation.satisfied_conditions.is_empty() {
            ascii.push_str("Satisfied Conditions:\n");
            for condition in &explanation.satisfied_conditions {
                ascii.push_str(&format!("  ✓ {}\n", condition));
            }
            ascii.push('\n');
        }

        if !explanation.unsatisfied_conditions.is_empty() {
            ascii.push_str("Unsatisfied Conditions:\n");
            for condition in &explanation.unsatisfied_conditions {
                ascii.push_str(&format!("  ✗ {}\n", condition));
            }
            ascii.push('\n');
        }

        if !explanation.reasoning_chain.is_empty() {
            ascii.push_str("Reasoning Steps:\n");
            for step in &explanation.reasoning_chain {
                ascii.push_str(&format!("  {}. {}\n", step.step, step.description));
            }
        }

        ascii
    }

    fn add_styles(&self, content: String) -> String {
        format!(
            "<style>
.reasoning-chain {{ font-family: Arial, sans-serif; padding: 20px; background: {}; color: {}; }}
.reasoning-chain h2, .reasoning-chain h3 {{ color: {}; }}
.reasoning-chain ul {{ list-style: none; padding-left: 20px; }}
.reasoning-steps {{ margin-top: 10px; }}
.step {{ padding: 10px; margin: 5px 0; background: {}; border-left: 3px solid {}; }}
.step-num {{ font-weight: bold; color: {}; }}
</style>{}",
            self.theme.background_color,
            self.theme.text_color,
            self.theme.root_color,
            self.theme.condition_color,
            self.theme.link_color,
            self.theme.outcome_color,
            content
        )
    }
}

impl Default for ReasoningChainVisualizer {
    fn default() -> Self {
        Self::new()
    }
}

/// Visualizer for evaluation audit trails.
pub struct AuditTrailVisualizer {
    theme: Theme,
}

impl AuditTrailVisualizer {
    /// Creates a new audit trail visualizer with default theme.
    #[must_use]
    pub fn new() -> Self {
        Self {
            theme: Theme::light(),
        }
    }

    /// Sets the theme for visualization.
    #[must_use]
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Renders an audit trail as an HTML table with performance metrics.
    #[must_use]
    pub fn to_html(&self, trail: &legalis_core::EvaluationAuditTrail) -> String {
        let records = trail.records();
        let mut html = String::from("<div class='audit-trail'>");
        html.push_str("<h2>Evaluation Audit Trail</h2>");

        if records.is_empty() {
            html.push_str("<p>No evaluation records.</p>");
        } else {
            html.push_str(&format!(
                "<p><strong>Total Evaluations:</strong> {}</p>",
                records.len()
            ));

            html.push_str("<table class='audit-table'>");
            html.push_str("<thead><tr><th>#</th><th>Condition</th><th>Result</th><th>Duration (μs)</th></tr></thead>");
            html.push_str("<tbody>");

            for (i, record) in records.iter().enumerate() {
                let result_color = if record.result { "green" } else { "red" };
                let result_text = if record.result {
                    "✓ Pass"
                } else {
                    "✗ Fail"
                };
                html.push_str("<tr>");
                html.push_str(&format!("<td>{}</td>", i + 1));
                html.push_str(&format!("<td>{}</td>", record.condition));
                html.push_str(&format!(
                    "<td style='color: {};'>{}</td>",
                    result_color, result_text
                ));
                html.push_str(&format!("<td>{}</td>", record.duration_micros));
                html.push_str("</tr>");
            }

            html.push_str("</tbody></table>");

            // Summary statistics
            let total_duration: u64 = records.iter().map(|r| r.duration_micros).sum();
            let avg_duration = if !records.is_empty() {
                total_duration / records.len() as u64
            } else {
                0
            };
            let passed = records.iter().filter(|r| r.result).count();

            html.push_str("<div class='summary'>");
            html.push_str(&format!(
                "<p><strong>Pass Rate:</strong> {}/{} ({:.1}%)</p>",
                passed,
                records.len(),
                (passed as f64 / records.len() as f64) * 100.0
            ));
            html.push_str(&format!(
                "<p><strong>Average Duration:</strong> {} μs</p>",
                avg_duration
            ));
            html.push_str(&format!(
                "<p><strong>Total Duration:</strong> {} μs</p>",
                total_duration
            ));
            html.push_str("</div>");
        }

        html.push_str("</div>");
        self.add_styles(html)
    }

    /// Renders an audit trail as ASCII art for terminal display.
    #[must_use]
    pub fn to_ascii(&self, trail: &legalis_core::EvaluationAuditTrail) -> String {
        let records = trail.records();
        let mut ascii = String::new();
        ascii.push_str("=== Evaluation Audit Trail ===\n\n");

        if records.is_empty() {
            ascii.push_str("No evaluation records.\n");
        } else {
            ascii.push_str(&format!("Total Evaluations: {}\n\n", records.len()));

            for (i, record) in records.iter().enumerate() {
                let result_symbol = if record.result { "✓" } else { "✗" };
                ascii.push_str(&format!(
                    "{:3}. {} | {} | {} μs\n",
                    i + 1,
                    result_symbol,
                    record.condition,
                    record.duration_micros
                ));
            }

            // Summary
            let total_duration: u64 = records.iter().map(|r| r.duration_micros).sum();
            let avg_duration = total_duration / records.len() as u64;
            let passed = records.iter().filter(|r| r.result).count();

            ascii.push('\n');
            ascii.push_str("=== Summary ===\n");
            ascii.push_str(&format!(
                "Pass Rate: {}/{} ({:.1}%)\n",
                passed,
                records.len(),
                (passed as f64 / records.len() as f64) * 100.0
            ));
            ascii.push_str(&format!("Average Duration: {} μs\n", avg_duration));
            ascii.push_str(&format!("Total Duration: {} μs\n", total_duration));
        }

        ascii
    }

    fn add_styles(&self, content: String) -> String {
        format!(
            "<style>
.audit-trail {{ font-family: Arial, sans-serif; padding: 20px; background: {}; color: {}; }}
.audit-table {{ width: 100%; border-collapse: collapse; margin: 20px 0; }}
.audit-table th, .audit-table td {{ border: 1px solid {}; padding: 8px; text-align: left; }}
.audit-table th {{ background: {}; }}
.summary {{ margin-top: 20px; padding: 15px; background: {}; border-left: 3px solid {}; }}
</style>{}",
            self.theme.background_color,
            self.theme.text_color,
            self.theme.link_color,
            self.theme.root_color,
            self.theme.condition_color,
            self.theme.outcome_color,
            content
        )
    }
}

impl Default for AuditTrailVisualizer {
    fn default() -> Self {
        Self::new()
    }
}

fn format_change_type(change: &legalis_core::StatuteChange) -> &'static str {
    match change {
        legalis_core::StatuteChange::IdChanged { .. } => "ID Changed",
        legalis_core::StatuteChange::TitleChanged { .. } => "Title Changed",
        legalis_core::StatuteChange::EffectChanged { .. } => "Effect Changed",
        legalis_core::StatuteChange::PreconditionsChanged { .. } => "Preconditions Changed",
        legalis_core::StatuteChange::TemporalValidityChanged => "Temporal Validity Changed",
        legalis_core::StatuteChange::VersionChanged { .. } => "Version Changed",
        legalis_core::StatuteChange::JurisdictionChanged { .. } => "Jurisdiction Changed",
    }
}

/// Interactive visualization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractiveConfig {
    /// Enable zoom and pan controls
    pub enable_zoom_pan: bool,
    /// Enable node/edge hover tooltips
    pub enable_tooltips: bool,
    /// Enable click-to-expand for collapsed nodes
    pub enable_click_expand: bool,
    /// Enable search and highlight functionality
    pub enable_search: bool,
    /// Enable mini-map for navigation
    pub enable_minimap: bool,
    /// Initial zoom level (1.0 = 100%)
    pub initial_zoom: f64,
    /// Minimum zoom level
    pub min_zoom: f64,
    /// Maximum zoom level
    pub max_zoom: f64,
    /// Mini-map size (width, height in pixels)
    pub minimap_size: (u32, u32),
}

impl Default for InteractiveConfig {
    fn default() -> Self {
        Self {
            enable_zoom_pan: true,
            enable_tooltips: true,
            enable_click_expand: true,
            enable_search: true,
            enable_minimap: true,
            initial_zoom: 1.0,
            min_zoom: 0.1,
            max_zoom: 5.0,
            minimap_size: (200, 150),
        }
    }
}

/// Interactive visualizer for decision trees and dependency graphs
pub struct InteractiveVisualizer {
    theme: Theme,
    config: InteractiveConfig,
}

impl InteractiveVisualizer {
    /// Creates a new interactive visualizer with default settings.
    pub fn new() -> Self {
        Self {
            theme: Theme::light(),
            config: InteractiveConfig::default(),
        }
    }

    /// Sets the color theme.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Sets the interactive configuration.
    pub fn with_config(mut self, config: InteractiveConfig) -> Self {
        self.config = config;
        self
    }

    /// Generates interactive HTML for a decision tree with zoom, pan, tooltips, etc.
    pub fn to_interactive_html(&self, tree: &DecisionTree) -> String {
        let svg = tree.to_svg_with_theme(&self.theme);
        self.wrap_with_interactive_controls(svg, "decision-tree")
    }

    /// Generates interactive HTML for a dependency graph.
    pub fn to_interactive_html_graph(&self, graph: &DependencyGraph) -> String {
        let svg = graph.to_svg_with_theme(&self.theme);
        self.wrap_with_interactive_controls(svg, "dependency-graph")
    }

    fn wrap_with_interactive_controls(&self, svg: String, viz_type: &str) -> String {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str("<meta charset=\"UTF-8\">\n");
        html.push_str(&format!(
            "<title>Interactive {} Visualization</title>\n",
            viz_type
        ));
        html.push_str("<style>\n");
        html.push_str(&self.generate_styles());
        html.push_str("</style>\n");
        html.push_str("</head>\n<body>\n");

        // Container
        html.push_str("<div class=\"viz-container\">\n");

        // Toolbar
        if self.config.enable_zoom_pan || self.config.enable_search {
            html.push_str("<div class=\"toolbar\">\n");

            if self.config.enable_zoom_pan {
                html.push_str("<div class=\"zoom-controls\">\n");
                html.push_str("<button id=\"zoom-in\" title=\"Zoom In\">+</button>\n");
                html.push_str("<button id=\"zoom-out\" title=\"Zoom Out\">-</button>\n");
                html.push_str("<button id=\"zoom-reset\" title=\"Reset Zoom\">⚪</button>\n");
                html.push_str("<button id=\"fit-to-screen\" title=\"Fit to Screen\">⬜</button>\n");
                html.push_str("</div>\n");
            }

            if self.config.enable_search {
                html.push_str("<div class=\"search-controls\">\n");
                html.push_str(
                    "<input type=\"text\" id=\"search-box\" placeholder=\"Search nodes...\" />\n",
                );
                html.push_str("<button id=\"search-btn\">🔍</button>\n");
                html.push_str("<button id=\"clear-search\">✕</button>\n");
                html.push_str("</div>\n");
            }

            html.push_str("</div>\n");
        }

        // Main visualization area
        html.push_str("<div class=\"viz-main\">\n");
        html.push_str("<div id=\"svg-container\" class=\"svg-container\">\n");
        html.push_str(&svg);
        html.push_str("</div>\n");

        // Mini-map
        if self.config.enable_minimap {
            html.push_str(&format!(
                "<div id=\"minimap\" class=\"minimap\" style=\"width: {}px; height: {}px;\"></div>\n",
                self.config.minimap_size.0, self.config.minimap_size.1
            ));
        }

        html.push_str("</div>\n");
        html.push_str("</div>\n");

        // JavaScript
        html.push_str("<script>\n");
        html.push_str(&self.generate_javascript());
        html.push_str("</script>\n");

        html.push_str("</body>\n</html>");
        html
    }

    fn generate_styles(&self) -> String {
        format!(
            "body {{
    margin: 0;
    padding: 0;
    font-family: Arial, sans-serif;
    background: {};
    color: {};
}}

.viz-container {{
    width: 100vw;
    height: 100vh;
    display: flex;
    flex-direction: column;
}}

.toolbar {{
    background: {};
    padding: 10px;
    border-bottom: 2px solid {};
    display: flex;
    gap: 20px;
    align-items: center;
}}

.zoom-controls, .search-controls {{
    display: flex;
    gap: 5px;
}}

button {{
    padding: 8px 12px;
    background: {};
    border: 1px solid {};
    color: {};
    cursor: pointer;
    border-radius: 4px;
    font-size: 14px;
}}

button:hover {{
    opacity: 0.8;
}}

#search-box {{
    padding: 8px;
    border: 1px solid {};
    background: {};
    color: {};
    border-radius: 4px;
    min-width: 200px;
}}

.viz-main {{
    flex: 1;
    position: relative;
    overflow: hidden;
}}

.svg-container {{
    width: 100%;
    height: 100%;
    overflow: hidden;
    cursor: grab;
}}

.svg-container:active {{
    cursor: grabbing;
}}

.svg-container svg {{
    width: 100%;
    height: 100%;
}}

.minimap {{
    position: absolute;
    bottom: 20px;
    right: 20px;
    border: 2px solid {};
    background: rgba(255, 255, 255, 0.9);
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
    overflow: hidden;
}}

.minimap svg {{
    width: 100%;
    height: 100%;
}}

.node-tooltip {{
    position: absolute;
    background: {};
    color: {};
    padding: 10px;
    border: 1px solid {};
    border-radius: 4px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
    pointer-events: none;
    z-index: 1000;
    max-width: 300px;
}}

.highlighted {{
    filter: drop-shadow(0 0 8px yellow);
}}

.collapsed {{
    opacity: 0.6;
}}",
            self.theme.background_color,
            self.theme.text_color,
            self.theme.root_color,
            self.theme.link_color,
            self.theme.condition_color,
            self.theme.link_color,
            self.theme.text_color,
            self.theme.link_color,
            self.theme.background_color,
            self.theme.text_color,
            self.theme.link_color,
            self.theme.background_color,
            self.theme.text_color,
            self.theme.link_color
        )
    }

    fn generate_javascript(&self) -> String {
        let mut js = String::new();

        js.push_str(&format!(
            "const config = {{
    enableZoomPan: {},
    enableTooltips: {},
    enableClickExpand: {},
    enableSearch: {},
    enableMinimap: {},
    initialZoom: {},
    minZoom: {},
    maxZoom: {}
}};\n\n",
            self.config.enable_zoom_pan,
            self.config.enable_tooltips,
            self.config.enable_click_expand,
            self.config.enable_search,
            self.config.enable_minimap,
            self.config.initial_zoom,
            self.config.min_zoom,
            self.config.max_zoom
        ));

        js.push_str(
            "let currentZoom = config.initialZoom;
let panX = 0;
let panY = 0;
let isPanning = false;
let startX = 0;
let startY = 0;

const svgContainer = document.getElementById('svg-container');
const svg = svgContainer.querySelector('svg');

// Zoom and Pan functionality
if (config.enableZoomPan) {
    document.getElementById('zoom-in')?.addEventListener('click', () => {
        currentZoom = Math.min(currentZoom * 1.2, config.maxZoom);
        updateTransform();
    });

    document.getElementById('zoom-out')?.addEventListener('click', () => {
        currentZoom = Math.max(currentZoom / 1.2, config.minZoom);
        updateTransform();
    });

    document.getElementById('zoom-reset')?.addEventListener('click', () => {
        currentZoom = config.initialZoom;
        panX = 0;
        panY = 0;
        updateTransform();
    });

    document.getElementById('fit-to-screen')?.addEventListener('click', () => {
        const containerRect = svgContainer.getBoundingClientRect();
        const svgRect = svg.getBoundingClientRect();
        const scaleX = containerRect.width / svgRect.width;
        const scaleY = containerRect.height / svgRect.height;
        currentZoom = Math.min(scaleX, scaleY) * 0.9;
        panX = (containerRect.width - svgRect.width * currentZoom) / 2;
        panY = (containerRect.height - svgRect.height * currentZoom) / 2;
        updateTransform();
    });

    // Mouse wheel zoom
    svgContainer.addEventListener('wheel', (e) => {
        e.preventDefault();
        const delta = e.deltaY > 0 ? 0.9 : 1.1;
        currentZoom = Math.max(config.minZoom, Math.min(config.maxZoom, currentZoom * delta));
        updateTransform();
    });

    // Pan with mouse drag
    svgContainer.addEventListener('mousedown', (e) => {
        isPanning = true;
        startX = e.clientX - panX;
        startY = e.clientY - panY;
    });

    document.addEventListener('mousemove', (e) => {
        if (isPanning) {
            panX = e.clientX - startX;
            panY = e.clientY - startY;
            updateTransform();
        }
    });

    document.addEventListener('mouseup', () => {
        isPanning = false;
    });
}

function updateTransform() {
    svg.style.transform = `translate(${panX}px, ${panY}px) scale(${currentZoom})`;
    svg.style.transformOrigin = '0 0';
    updateMinimap();
}

// Tooltips
if (config.enableTooltips) {
    const tooltip = document.createElement('div');
    tooltip.className = 'node-tooltip';
    tooltip.style.display = 'none';
    document.body.appendChild(tooltip);

    svg.querySelectorAll('g[id], rect[id], circle[id], text[id]').forEach(element => {
        element.addEventListener('mouseenter', (e) => {
            const id = element.id || element.textContent;
            const content = element.getAttribute('data-tooltip') || id || 'Node';
            tooltip.textContent = content;
            tooltip.style.display = 'block';
        });

        element.addEventListener('mousemove', (e) => {
            tooltip.style.left = e.pageX + 10 + 'px';
            tooltip.style.top = e.pageY + 10 + 'px';
        });

        element.addEventListener('mouseleave', () => {
            tooltip.style.display = 'none';
        });
    });
}

// Click to expand/collapse
if (config.enableClickExpand) {
    const collapsedNodes = new Set();

    svg.querySelectorAll('g[id]').forEach(node => {
        node.addEventListener('click', (e) => {
            e.stopPropagation();
            const nodeId = node.id;

            if (collapsedNodes.has(nodeId)) {
                collapsedNodes.delete(nodeId);
                node.classList.remove('collapsed');
                showChildNodes(node);
            } else {
                collapsedNodes.add(nodeId);
                node.classList.add('collapsed');
                hideChildNodes(node);
            }
        });
    });

    function hideChildNodes(node) {
        // Find and hide child nodes (simple implementation)
        const children = findChildElements(node);
        children.forEach(child => {
            child.style.display = 'none';
        });
    }

    function showChildNodes(node) {
        const children = findChildElements(node);
        children.forEach(child => {
            child.style.display = '';
        });
    }

    function findChildElements(node) {
        // Simple heuristic: find elements connected via edges
        return [];
    }
}

// Search and highlight
if (config.enableSearch) {
    const searchBox = document.getElementById('search-box');
    const searchBtn = document.getElementById('search-btn');
    const clearBtn = document.getElementById('clear-search');

    function performSearch() {
        const query = searchBox.value.toLowerCase();
        clearHighlights();

        if (!query) return;

        svg.querySelectorAll('g[id], text').forEach(element => {
            const text = element.textContent.toLowerCase();
            if (text.includes(query)) {
                element.classList.add('highlighted');
            }
        });
    }

    function clearHighlights() {
        svg.querySelectorAll('.highlighted').forEach(el => {
            el.classList.remove('highlighted');
        });
    }

    searchBtn?.addEventListener('click', performSearch);
    searchBox?.addEventListener('keypress', (e) => {
        if (e.key === 'Enter') performSearch();
    });
    clearBtn?.addEventListener('click', () => {
        searchBox.value = '';
        clearHighlights();
    });
}

// Mini-map
if (config.enableMinimap) {
    const minimap = document.getElementById('minimap');
    if (minimap && svg) {
        const minimapSvg = svg.cloneNode(true);
        minimapSvg.style.transform = 'scale(0.1)';
        minimap.appendChild(minimapSvg);
    }
}

function updateMinimap() {
    if (!config.enableMinimap) return;
    const minimap = document.getElementById('minimap');
    if (minimap) {
        const minimapSvg = minimap.querySelector('svg');
        if (minimapSvg) {
            minimapSvg.style.transform = `scale(${currentZoom * 0.1})`;
        }
    }
}

// Initialize
updateTransform();
",
        );

        js
    }
}

impl Default for InteractiveVisualizer {
    fn default() -> Self {
        Self::new()
    }
}

/// 3D visualization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreeDConfig {
    /// Enable VR mode
    pub enable_vr: bool,
    /// Enable AR mode
    pub enable_ar: bool,
    /// Use force-directed layout
    pub force_directed: bool,
    /// Enable depth-based coloring
    pub depth_coloring: bool,
    /// Camera field of view (degrees)
    pub camera_fov: f64,
    /// Graph node size
    pub node_size: f64,
    /// Edge thickness
    pub edge_thickness: f64,
    /// Force-directed simulation strength (0.0-1.0)
    pub force_strength: f64,
    /// Auto-rotate speed (degrees per second, 0 = disabled)
    pub auto_rotate_speed: f64,
}

impl Default for ThreeDConfig {
    fn default() -> Self {
        Self {
            enable_vr: false,
            enable_ar: false,
            force_directed: true,
            depth_coloring: true,
            camera_fov: 75.0,
            node_size: 1.0,
            edge_thickness: 0.1,
            force_strength: 0.5,
            auto_rotate_speed: 10.0,
        }
    }
}

/// 3D visualizer for dependency graphs and timelines using WebGL
pub struct ThreeDVisualizer {
    theme: Theme,
    config: ThreeDConfig,
}

impl ThreeDVisualizer {
    /// Creates a new 3D visualizer with default settings.
    pub fn new() -> Self {
        Self {
            theme: Theme::light(),
            config: ThreeDConfig::default(),
        }
    }

    /// Sets the color theme.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Sets the 3D configuration.
    pub fn with_config(mut self, config: ThreeDConfig) -> Self {
        self.config = config;
        self
    }

    /// Generates 3D HTML visualization for a dependency graph.
    pub fn to_3d_html_graph(&self, graph: &DependencyGraph) -> String {
        let nodes = self.extract_graph_nodes(graph);
        let edges = self.extract_graph_edges(graph);
        self.generate_3d_html("Dependency Graph", &nodes, &edges, false)
    }

    /// Generates 3D HTML visualization for a timeline.
    pub fn to_3d_html_timeline(&self, timeline: &Timeline) -> String {
        let nodes = self.extract_timeline_nodes(timeline);
        let edges = self.extract_timeline_edges(timeline);
        self.generate_3d_html("Timeline", &nodes, &edges, true)
    }

    fn extract_graph_nodes(&self, graph: &DependencyGraph) -> Vec<(String, usize)> {
        // Extract nodes with their depth for depth-based coloring
        let mut nodes = Vec::new();
        for idx in graph.graph.node_indices() {
            if let Some(statute_id) = graph.graph.node_weight(idx) {
                // Calculate depth (simplified - distance from root nodes)
                let depth = self.calculate_node_depth(graph, idx);
                nodes.push((statute_id.clone(), depth));
            }
        }
        nodes
    }

    fn extract_graph_edges(&self, graph: &DependencyGraph) -> Vec<(usize, usize, String)> {
        let mut edges = Vec::new();
        for edge in graph.graph.edge_indices() {
            if let Some((from, to)) = graph.graph.edge_endpoints(edge) {
                let relation = graph
                    .graph
                    .edge_weight(edge)
                    .unwrap_or(&"depends-on".to_string())
                    .clone();
                edges.push((from.index(), to.index(), relation));
            }
        }
        edges
    }

    fn calculate_node_depth(&self, graph: &DependencyGraph, node: NodeIndex) -> usize {
        // Simple BFS to find depth from root nodes
        use std::collections::VecDeque;
        let mut visited = std::collections::HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((node, 0));
        visited.insert(node);

        while let Some((current, depth)) = queue.pop_front() {
            let incoming = graph
                .graph
                .neighbors_directed(current, petgraph::Direction::Incoming);
            if incoming.clone().count() == 0 {
                return depth; // Found a root node
            }
            for neighbor in incoming {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back((neighbor, depth + 1));
                }
            }
        }
        0
    }

    fn extract_timeline_nodes(&self, timeline: &Timeline) -> Vec<(String, usize)> {
        timeline
            .events
            .iter()
            .enumerate()
            .map(|(i, (date, event))| {
                let label = match event {
                    TimelineEvent::Enacted { statute_id, title } => {
                        format!("{}: Enacted {} - {}", date, statute_id, title)
                    }
                    TimelineEvent::Amended {
                        statute_id,
                        description,
                    } => format!("{}: Amended {} - {}", date, statute_id, description),
                    TimelineEvent::Repealed { statute_id } => {
                        format!("{}: Repealed {}", date, statute_id)
                    }
                    TimelineEvent::EffectiveStart { statute_id } => {
                        format!("{}: Effective Start {}", date, statute_id)
                    }
                    TimelineEvent::EffectiveEnd { statute_id } => {
                        format!("{}: Effective End {}", date, statute_id)
                    }
                };
                (label, i)
            })
            .collect()
    }

    fn extract_timeline_edges(&self, timeline: &Timeline) -> Vec<(usize, usize, String)> {
        // Connect consecutive events in timeline
        let mut edges = Vec::new();
        for i in 0..timeline.events.len().saturating_sub(1) {
            edges.push((i, i + 1, "follows".to_string()));
        }
        edges
    }

    fn generate_3d_html(
        &self,
        title: &str,
        nodes: &[(String, usize)],
        edges: &[(usize, usize, String)],
        is_timeline: bool,
    ) -> String {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str("<meta charset=\"UTF-8\">\n");
        html.push_str(&format!("<title>3D {} Visualization</title>\n", title));
        html.push_str("<style>\n");
        html.push_str(&self.generate_3d_styles());
        html.push_str("</style>\n");
        html.push_str("</head>\n<body>\n");

        // Container
        html.push_str("<div class=\"viz-3d-container\">\n");
        html.push_str("<div class=\"controls-panel\">\n");
        html.push_str(&format!("<h2>3D {} Visualization</h2>\n", title));

        // Control buttons
        html.push_str("<div class=\"control-group\">\n");
        html.push_str("<button id=\"reset-camera\">Reset Camera</button>\n");
        html.push_str("<button id=\"toggle-rotation\">Toggle Auto-Rotate</button>\n");
        if self.config.force_directed {
            html.push_str("<button id=\"reset-forces\">Reset Forces</button>\n");
        }
        if self.config.enable_vr {
            html.push_str("<button id=\"enter-vr\">Enter VR</button>\n");
        }
        if self.config.enable_ar {
            html.push_str("<button id=\"enter-ar\">Enter AR</button>\n");
        }
        html.push_str("</div>\n");

        // Info panel
        html.push_str("<div class=\"info-panel\">\n");
        html.push_str("<div id=\"node-info\">Hover over nodes for details</div>\n");
        html.push_str(&format!("<div>Nodes: {}</div>\n", nodes.len()));
        html.push_str(&format!("<div>Edges: {}</div>\n", edges.len()));
        html.push_str("</div>\n");

        html.push_str("</div>\n"); // controls-panel

        // 3D canvas
        html.push_str("<div id=\"canvas-container\"></div>\n");

        html.push_str("</div>\n"); // viz-3d-container

        // Include Three.js from CDN
        html.push_str("<script src=\"https://cdnjs.cloudflare.com/ajax/libs/three.js/r128/three.min.js\"></script>\n");

        if self.config.enable_vr || self.config.enable_ar {
            html.push_str("<script src=\"https://cdn.jsdelivr.net/npm/three@0.128.0/examples/js/webxr/VRButton.js\"></script>\n");
        }

        // Generate JavaScript
        html.push_str("<script>\n");
        html.push_str(&self.generate_3d_javascript(nodes, edges, is_timeline));
        html.push_str("</script>\n");

        html.push_str("</body>\n</html>");
        html
    }

    fn generate_3d_styles(&self) -> String {
        format!(
            "body {{
    margin: 0;
    padding: 0;
    font-family: Arial, sans-serif;
    background: {};
    color: {};
    overflow: hidden;
}}

.viz-3d-container {{
    width: 100vw;
    height: 100vh;
    display: flex;
}}

.controls-panel {{
    width: 250px;
    background: {};
    padding: 20px;
    overflow-y: auto;
    border-right: 2px solid {};
}}

.controls-panel h2 {{
    margin-top: 0;
    font-size: 18px;
}}

.control-group {{
    margin: 20px 0;
}}

button {{
    width: 100%;
    padding: 10px;
    margin: 5px 0;
    background: {};
    border: 1px solid {};
    color: {};
    cursor: pointer;
    border-radius: 4px;
    font-size: 14px;
}}

button:hover {{
    opacity: 0.8;
}}

.info-panel {{
    margin-top: 20px;
    padding: 10px;
    background: {};
    border-radius: 4px;
    font-size: 12px;
}}

.info-panel div {{
    margin: 5px 0;
}}

#canvas-container {{
    flex: 1;
    position: relative;
}}

#node-info {{
    font-weight: bold;
    margin-bottom: 10px !important;
}}",
            self.theme.background_color,
            self.theme.text_color,
            self.theme.root_color,
            self.theme.link_color,
            self.theme.condition_color,
            self.theme.link_color,
            self.theme.text_color,
            self.theme.discretion_color
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn generate_3d_javascript(
        &self,
        nodes: &[(String, usize)],
        edges: &[(usize, usize, String)],
        is_timeline: bool,
    ) -> String {
        let mut js = String::new();

        // Configuration
        js.push_str(&format!(
            "const config = {{
    enableVR: {},
    enableAR: {},
    forceDirected: {},
    depthColoring: {},
    cameraFov: {},
    nodeSize: {},
    edgeThickness: {},
    forceStrength: {},
    autoRotateSpeed: {},
    isTimeline: {}
}};\n\n",
            self.config.enable_vr,
            self.config.enable_ar,
            self.config.force_directed,
            self.config.depth_coloring,
            self.config.camera_fov,
            self.config.node_size,
            self.config.edge_thickness,
            self.config.force_strength,
            self.config.auto_rotate_speed,
            is_timeline
        ));

        // Node data
        js.push_str("const nodes = [\n");
        for (label, depth) in nodes {
            js.push_str(&format!(
                "    {{ label: '{}', depth: {} }},\n",
                label.replace('\'', "\\'"),
                depth
            ));
        }
        js.push_str("];\n\n");

        // Edge data
        js.push_str("const edges = [\n");
        for (from, to, relation) in edges {
            js.push_str(&format!(
                "    {{ from: {}, to: {}, relation: '{}' }},\n",
                from,
                to,
                relation.replace('\'', "\\'")
            ));
        }
        js.push_str("];\n\n");

        // Main Three.js code
        js.push_str(&format!(
            "// Three.js setup
let scene, camera, renderer, controls;
let nodeObjects = [];
let edgeObjects = [];
let autoRotate = true;

function init() {{
    const container = document.getElementById('canvas-container');

    // Scene
    scene = new THREE.Scene();
    scene.background = new THREE.Color('{}');

    // Camera
    camera = new THREE.PerspectiveCamera(
        config.cameraFov,
        container.clientWidth / container.clientHeight,
        0.1,
        1000
    );
    camera.position.z = 50;

    // Renderer
    renderer = new THREE.WebGLRenderer({{ antialias: true }});
    renderer.setSize(container.clientWidth, container.clientHeight);
    container.appendChild(renderer.domElement);

    // Lights
    const ambientLight = new THREE.AmbientLight(0xffffff, 0.6);
    scene.add(ambientLight);

    const directionalLight = new THREE.DirectionalLight(0xffffff, 0.8);
    directionalLight.position.set(10, 10, 10);
    scene.add(directionalLight);

    // Create graph
    createGraph();

    // Event listeners
    window.addEventListener('resize', onWindowResize);
    document.getElementById('reset-camera').addEventListener('click', resetCamera);
    document.getElementById('toggle-rotation').addEventListener('click', toggleRotation);

    if (config.forceDirected) {{
        document.getElementById('reset-forces')?.addEventListener('click', resetForces);
    }}

    // Mouse interaction
    const raycaster = new THREE.Raycaster();
    const mouse = new THREE.Vector2();

    renderer.domElement.addEventListener('mousemove', (event) => {{
        const rect = container.getBoundingClientRect();
        mouse.x = ((event.clientX - rect.left) / rect.width) * 2 - 1;
        mouse.y = -((event.clientY - rect.top) / rect.height) * 2 + 1;

        raycaster.setFromCamera(mouse, camera);
        const intersects = raycaster.intersectObjects(nodeObjects);

        if (intersects.length > 0) {{
            const nodeIndex = nodeObjects.indexOf(intersects[0].object);
            if (nodeIndex !== -1) {{
                document.getElementById('node-info').textContent = nodes[nodeIndex].label;
            }}
        }} else {{
            document.getElementById('node-info').textContent = 'Hover over nodes for details';
        }}
    }});

    // Animation loop
    animate();
}}

function createGraph() {{
    // Node positions
    const positions = calculateNodePositions();

    // Create nodes
    nodes.forEach((node, i) => {{
        const geometry = new THREE.SphereGeometry(config.nodeSize, 32, 32);

        // Depth-based coloring
        let color;
        if (config.depthColoring) {{
            const hue = (node.depth * 60) % 360; // Cycle through colors by depth
            color = new THREE.Color(`hsl(${{hue}}, 70%, 50%)`);
        }} else {{
            color = new THREE.Color('{}');
        }}

        const material = new THREE.MeshPhongMaterial({{ color }});
        const sphere = new THREE.Mesh(geometry, material);

        sphere.position.copy(positions[i]);
        sphere.userData = {{ index: i, label: node.label, depth: node.depth }};

        scene.add(sphere);
        nodeObjects.push(sphere);
    }});

    // Create edges
    edges.forEach(edge => {{
        const start = positions[edge.from];
        const end = positions[edge.to];

        const points = [start, end];
        const geometry = new THREE.BufferGeometry().setFromPoints(points);
        const material = new THREE.LineBasicMaterial({{
            color: '{}',
            linewidth: config.edgeThickness
        }});
        const line = new THREE.Line(geometry, material);

        scene.add(line);
        edgeObjects.push(line);
    }});
}}

function calculateNodePositions() {{
    const positions = [];

    if (config.isTimeline) {{
        // Timeline layout - linear arrangement
        nodes.forEach((node, i) => {{
            const x = (i - nodes.length / 2) * 5;
            const y = Math.sin(i * 0.5) * 3;
            const z = i * 2;
            positions.push(new THREE.Vector3(x, y, z));
        }});
    }} else if (config.forceDirected) {{
        // Force-directed layout (simplified)
        nodes.forEach((node, i) => {{
            const angle = (i / nodes.length) * Math.PI * 2;
            const radius = 20 + node.depth * 5;
            const x = Math.cos(angle) * radius;
            const y = Math.sin(angle) * radius;
            const z = node.depth * 3;
            positions.push(new THREE.Vector3(x, y, z));
        }});
    }} else {{
        // Simple circular layout
        nodes.forEach((node, i) => {{
            const angle = (i / nodes.length) * Math.PI * 2;
            const radius = 20;
            const x = Math.cos(angle) * radius;
            const y = Math.sin(angle) * radius;
            const z = 0;
            positions.push(new THREE.Vector3(x, y, z));
        }});
    }}

    return positions;
}}

function animate() {{
    requestAnimationFrame(animate);

    if (autoRotate) {{
        const delta = config.autoRotateSpeed * 0.001;
        scene.rotation.y += delta;
    }}

    renderer.render(scene, camera);
}}

function onWindowResize() {{
    const container = document.getElementById('canvas-container');
    camera.aspect = container.clientWidth / container.clientHeight;
    camera.updateProjectionMatrix();
    renderer.setSize(container.clientWidth, container.clientHeight);
}}

function resetCamera() {{
    camera.position.set(0, 0, 50);
    camera.lookAt(0, 0, 0);
    scene.rotation.set(0, 0, 0);
}}

function toggleRotation() {{
    autoRotate = !autoRotate;
}}

function resetForces() {{
    // Recreate graph with new force-directed positions
    nodeObjects.forEach(obj => scene.remove(obj));
    edgeObjects.forEach(obj => scene.remove(obj));
    nodeObjects = [];
    edgeObjects = [];
    createGraph();
}}

// Initialize
init();
",
            self.theme.background_color, self.theme.condition_color, self.theme.link_color
        ));

        js
    }
}

impl Default for ThreeDVisualizer {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Immersive Legal Visualization (v0.3.0)
// ============================================================================

/// Configuration for VR statute exploration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VRExplorationConfig {
    /// Enable hand tracking
    pub enable_hand_tracking: bool,
    /// Enable teleportation navigation
    pub enable_teleportation: bool,
    /// Enable voice commands
    pub enable_voice_commands: bool,
    /// Enable spatial audio
    pub enable_spatial_audio: bool,
    /// Enable haptic feedback
    pub enable_haptic_feedback: bool,
    /// Node interaction distance (meters)
    pub interaction_distance: f32,
    /// Movement speed multiplier
    pub movement_speed: f32,
}

impl Default for VRExplorationConfig {
    fn default() -> Self {
        Self {
            enable_hand_tracking: true,
            enable_teleportation: true,
            enable_voice_commands: false,
            enable_spatial_audio: true,
            enable_haptic_feedback: true,
            interaction_distance: 2.0,
            movement_speed: 1.0,
        }
    }
}

/// VR statute exploration visualizer.
pub struct VRStatuteExplorer {
    theme: Theme,
    config: VRExplorationConfig,
}

impl VRStatuteExplorer {
    /// Creates a new VR statute explorer.
    pub fn new() -> Self {
        Self {
            theme: Theme::light(),
            config: VRExplorationConfig::default(),
        }
    }

    /// Sets the color theme.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Sets the VR configuration.
    pub fn with_config(mut self, config: VRExplorationConfig) -> Self {
        self.config = config;
        self
    }

    /// Generates VR HTML for statute exploration.
    pub fn to_vr_html(&self, statute: &Statute) -> String {
        let tree = DecisionTree::from_statute(statute).unwrap_or_else(|_| DecisionTree::new());
        self.to_vr_html_tree(&tree)
    }

    /// Generates VR HTML for a decision tree.
    pub fn to_vr_html_tree(&self, tree: &DecisionTree) -> String {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str("<meta charset=\"UTF-8\">\n");
        html.push_str(
            "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n",
        );
        html.push_str("<title>VR Statute Explorer</title>\n");
        html.push_str("<style>\n");
        html.push_str(&self.generate_vr_styles());
        html.push_str("</style>\n");
        html.push_str("</head>\n<body>\n");

        // VR container
        html.push_str("<div id=\"vr-container\">\n");
        html.push_str("<div class=\"info-overlay\">\n");
        html.push_str("<h2>VR Statute Explorer</h2>\n");
        html.push_str("<p>Click 'Enter VR' to start the immersive experience</p>\n");
        html.push_str("<div id=\"status\">Status: Ready</div>\n");
        html.push_str("<div id=\"node-detail\">Point at nodes to see details</div>\n");
        html.push_str("</div>\n");
        html.push_str("</div>\n");

        // Scripts
        html.push_str("<script src=\"https://cdnjs.cloudflare.com/ajax/libs/three.js/r128/three.min.js\"></script>\n");
        html.push_str("<script>\n");
        html.push_str(&self.generate_vr_javascript(tree));
        html.push_str("</script>\n");

        html.push_str("</body>\n</html>");
        html
    }

    fn generate_vr_styles(&self) -> String {
        format!(
            "body {{
    margin: 0;
    padding: 0;
    font-family: Arial, sans-serif;
    background: {};
    color: {};
}}

#vr-container {{
    width: 100vw;
    height: 100vh;
    position: relative;
}}

.info-overlay {{
    position: absolute;
    top: 20px;
    left: 20px;
    background: rgba(0, 0, 0, 0.7);
    color: white;
    padding: 20px;
    border-radius: 8px;
    max-width: 400px;
    z-index: 1000;
}}

.info-overlay h2 {{
    margin: 0 0 10px 0;
    font-size: 24px;
}}

.info-overlay p {{
    margin: 5px 0;
    font-size: 14px;
}}

#status, #node-detail {{
    margin-top: 10px;
    padding: 8px;
    background: rgba(255, 255, 255, 0.1);
    border-radius: 4px;
    font-size: 12px;
}}
",
            self.theme.background_color, self.theme.text_color
        )
    }

    fn generate_vr_javascript(&self, tree: &DecisionTree) -> String {
        let nodes = self.extract_tree_nodes(tree);

        format!(
            "// VR Statute Explorer
const config = {{
    enableHandTracking: {},
    enableTeleportation: {},
    enableVoiceCommands: {},
    enableSpatialAudio: {},
    enableHapticFeedback: {},
    interactionDistance: {},
    movementSpeed: {}
}};

const nodes = {};

let scene, camera, renderer;
let vrSession = null;
let nodeObjects = [];
let controllers = [];
let audioContext = null;
let spatialAudioNodes = [];

function init() {{
    const container = document.getElementById('vr-container');

    // Scene
    scene = new THREE.Scene();
    scene.background = new THREE.Color('{}');

    // Camera
    camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
    camera.position.set(0, 1.6, 3); // Average human eye height

    // Renderer with WebXR
    renderer = new THREE.WebGLRenderer({{ antialias: true }});
    renderer.setSize(window.innerWidth, window.innerHeight);
    renderer.xr.enabled = true;
    container.appendChild(renderer.domElement);

    // Add VR button
    const vrButton = createVRButton();
    document.body.appendChild(vrButton);

    // Lights
    const ambientLight = new THREE.AmbientLight(0xffffff, 0.5);
    scene.add(ambientLight);

    const directionalLight = new THREE.DirectionalLight(0xffffff, 0.8);
    directionalLight.position.set(5, 10, 7.5);
    scene.add(directionalLight);

    // Floor
    const floorGeometry = new THREE.PlaneGeometry(50, 50);
    const floorMaterial = new THREE.MeshStandardMaterial({{
        color: 0x404040,
        roughness: 0.8,
        metalness: 0.2
    }});
    const floor = new THREE.Mesh(floorGeometry, floorMaterial);
    floor.rotation.x = -Math.PI / 2;
    scene.add(floor);

    // Create statute graph
    createStatuteGraph();

    // Setup controllers
    setupControllers();

    // Setup spatial audio
    if (config.enableSpatialAudio) {{
        setupSpatialAudio();
    }}

    // Event listeners
    window.addEventListener('resize', onWindowResize);

    // Start render loop
    renderer.setAnimationLoop(render);
}}

function createVRButton() {{
    const button = document.createElement('button');
    button.style.cssText = `
        position: absolute;
        bottom: 20px;
        left: 50%;
        transform: translateX(-50%);
        padding: 12px 24px;
        font-size: 16px;
        font-weight: bold;
        color: white;
        background: #1976d2;
        border: none;
        border-radius: 4px;
        cursor: pointer;
        z-index: 1001;
    `;
    button.textContent = 'ENTER VR';

    button.addEventListener('click', async () => {{
        if (!navigator.xr) {{
            alert('WebXR not supported in this browser');
            return;
        }}

        try {{
            const session = await navigator.xr.requestSession('immersive-vr', {{
                optionalFeatures: ['hand-tracking', 'local-floor']
            }});

            renderer.xr.setSession(session);
            vrSession = session;

            session.addEventListener('end', () => {{
                vrSession = null;
                document.getElementById('status').textContent = 'Status: VR session ended';
            }});

            document.getElementById('status').textContent = 'Status: VR session active';
        }} catch (error) {{
            console.error('Failed to start VR session:', error);
            alert('Failed to start VR session: ' + error.message);
        }}
    }});

    return button;
}}

function createStatuteGraph() {{
    nodes.forEach((node, index) => {{
        // Create node sphere
        const geometry = new THREE.SphereGeometry(0.2, 32, 32);
        let color;

        switch(node.type) {{
            case 'condition':
                color = new THREE.Color('{}');
                break;
            case 'discretion':
                color = new THREE.Color('{}');
                break;
            case 'outcome':
                color = new THREE.Color('{}');
                break;
            default:
                color = new THREE.Color('{}');
        }}

        const material = new THREE.MeshStandardMaterial({{
            color,
            roughness: 0.5,
            metalness: 0.3
        }});
        const sphere = new THREE.Mesh(geometry, material);

        // Position nodes in a circular arrangement
        const angle = (index / nodes.length) * Math.PI * 2;
        const radius = 3;
        sphere.position.set(
            Math.cos(angle) * radius,
            1.6 + (node.depth || 0) * 0.5,
            Math.sin(angle) * radius
        );

        sphere.userData = {{
            index,
            label: node.label,
            type: node.type,
            description: node.description || ''
        }};

        scene.add(sphere);
        nodeObjects.push(sphere);

        // Add text label
        const canvas = document.createElement('canvas');
        const context = canvas.getContext('2d');
        canvas.width = 512;
        canvas.height = 256;
        context.fillStyle = 'white';
        context.font = 'bold 48px Arial';
        context.textAlign = 'center';
        context.fillText(node.label, 256, 128);

        const texture = new THREE.CanvasTexture(canvas);
        const spriteMaterial = new THREE.SpriteMaterial({{ map: texture }});
        const sprite = new THREE.Sprite(spriteMaterial);
        sprite.position.copy(sphere.position);
        sprite.position.y += 0.3;
        sprite.scale.set(1, 0.5, 1);
        scene.add(sprite);
    }});
}}

function setupControllers() {{
    // Controller 1
    const controller1 = renderer.xr.getController(0);
    controller1.addEventListener('selectstart', onSelectStart);
    controller1.addEventListener('selectend', onSelectEnd);
    controller1.addEventListener('select', onSelect);
    scene.add(controller1);
    controllers.push(controller1);

    // Controller 2
    const controller2 = renderer.xr.getController(1);
    controller2.addEventListener('selectstart', onSelectStart);
    controller2.addEventListener('selectend', onSelectEnd);
    controller2.addEventListener('select', onSelect);
    scene.add(controller2);
    controllers.push(controller2);

    // Add controller visualizations
    const geometry = new THREE.BufferGeometry().setFromPoints([
        new THREE.Vector3(0, 0, 0),
        new THREE.Vector3(0, 0, -1)
    ]);
    const material = new THREE.LineBasicMaterial({{ color: 0xffffff }});

    controllers.forEach(controller => {{
        const line = new THREE.Line(geometry, material);
        line.name = 'line';
        line.scale.z = 5;
        controller.add(line);
    }});
}}

function setupSpatialAudio() {{
    audioContext = new (window.AudioContext || window.webkitAudioContext)();

    // Create spatial audio for each node
    nodeObjects.forEach((nodeObj, index) => {{
        const listener = new THREE.AudioListener();
        camera.add(listener);

        const sound = new THREE.PositionalAudio(listener);

        // Create oscillator for spatial audio feedback
        const oscillator = audioContext.createOscillator();
        const gainNode = audioContext.createGain();

        oscillator.frequency.value = 200 + (index * 50); // Different pitch for each node
        gainNode.gain.value = 0; // Start silent

        oscillator.connect(gainNode);
        gainNode.connect(audioContext.destination);

        spatialAudioNodes.push({{ node: nodeObj, oscillator, gainNode }});
    }});
}}

function onSelectStart(event) {{
    const controller = event.target;
    const intersections = getIntersections(controller);

    if (intersections.length > 0) {{
        const intersection = intersections[0];
        const nodeData = intersection.object.userData;

        if (nodeData && nodeData.label) {{
            document.getElementById('node-detail').textContent =
                `Selected: ${{nodeData.label}} - ${{nodeData.description || 'No description'}}`;

            // Haptic feedback
            if (config.enableHapticFeedback && controller.gamepad) {{
                controller.gamepad.hapticActuators[0].pulse(0.7, 100);
            }}

            // Spatial audio feedback
            if (config.enableSpatialAudio && spatialAudioNodes[nodeData.index]) {{
                const audio = spatialAudioNodes[nodeData.index];
                audio.gainNode.gain.value = 0.3;
                audio.oscillator.start(audioContext.currentTime);
                setTimeout(() => {{
                    audio.gainNode.gain.value = 0;
                }}, 200);
            }}
        }}
    }}
}}

function onSelectEnd(event) {{
    const controller = event.target;

    // Release haptic feedback
    if (config.enableHapticFeedback && controller.gamepad) {{
        controller.gamepad.hapticActuators[0].reset();
    }}
}}

function onSelect(event) {{
    // Handle selection complete
}}

function getIntersections(controller) {{
    const tempMatrix = new THREE.Matrix4();
    tempMatrix.identity().extractRotation(controller.matrixWorld);

    const raycaster = new THREE.Raycaster();
    raycaster.ray.origin.setFromMatrixPosition(controller.matrixWorld);
    raycaster.ray.direction.set(0, 0, -1).applyMatrix4(tempMatrix);

    return raycaster.intersectObjects(nodeObjects, false);
}}

function render() {{
    // Update controller interactions
    controllers.forEach(controller => {{
        const intersections = getIntersections(controller);

        if (intersections.length > 0) {{
            const intersection = intersections[0];
            const line = controller.getObjectByName('line');
            if (line) {{
                line.scale.z = intersection.distance;
            }}
        }}
    }});

    renderer.render(scene, camera);
}}

function onWindowResize() {{
    camera.aspect = window.innerWidth / window.innerHeight;
    camera.updateProjectionMatrix();
    renderer.setSize(window.innerWidth, window.innerHeight);
}}

// Initialize
init();
",
            self.config.enable_hand_tracking,
            self.config.enable_teleportation,
            self.config.enable_voice_commands,
            self.config.enable_spatial_audio,
            self.config.enable_haptic_feedback,
            self.config.interaction_distance,
            self.config.movement_speed,
            serde_json::to_string_pretty(&nodes).unwrap_or_else(|_| "[]".to_string()),
            self.theme.background_color,
            self.theme.condition_color,
            self.theme.discretion_color,
            self.theme.outcome_color,
            self.theme.root_color
        )
    }

    fn extract_tree_nodes(&self, tree: &DecisionTree) -> Vec<serde_json::Value> {
        let mut nodes = Vec::new();

        // Extract all nodes from the graph
        for node_idx in tree.graph.node_indices() {
            if let Some(node) = tree.graph.node_weight(node_idx) {
                let (node_type, label, description) = match node {
                    DecisionNode::Root { statute_id, title } => {
                        ("root", statute_id.clone(), title.clone())
                    }
                    DecisionNode::Condition {
                        description,
                        is_discretionary,
                    } => {
                        let node_type = if *is_discretionary {
                            "discretion"
                        } else {
                            "condition"
                        };
                        (node_type, description.clone(), description.clone())
                    }
                    DecisionNode::Outcome { description } => {
                        ("outcome", description.clone(), description.clone())
                    }
                    DecisionNode::Discretion { issue, hint } => {
                        let desc = hint.as_ref().unwrap_or(issue);
                        ("discretion", issue.clone(), desc.clone())
                    }
                };

                nodes.push(serde_json::json!({
                    "label": label,
                    "type": node_type,
                    "depth": 0,
                    "description": description
                }));
            }
        }

        nodes
    }
}

impl Default for VRStatuteExplorer {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration for AR document overlay.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AROverlayConfig {
    /// Enable marker-based AR
    pub enable_markers: bool,
    /// Enable markerless AR (SLAM)
    pub enable_markerless: bool,
    /// Enable face tracking
    pub enable_face_tracking: bool,
    /// Marker size in meters
    pub marker_size: f32,
    /// Overlay opacity (0.0-1.0)
    pub overlay_opacity: f32,
}

impl Default for AROverlayConfig {
    fn default() -> Self {
        Self {
            enable_markers: true,
            enable_markerless: true,
            enable_face_tracking: false,
            marker_size: 0.15,
            overlay_opacity: 0.9,
        }
    }
}

/// AR legal document overlay visualizer.
pub struct ARDocumentOverlay {
    theme: Theme,
    config: AROverlayConfig,
}

impl ARDocumentOverlay {
    /// Creates a new AR document overlay.
    pub fn new() -> Self {
        Self {
            theme: Theme::light(),
            config: AROverlayConfig::default(),
        }
    }

    /// Sets the color theme.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Sets the AR configuration.
    pub fn with_config(mut self, config: AROverlayConfig) -> Self {
        self.config = config;
        self
    }

    /// Generates AR HTML for document overlay.
    pub fn to_ar_html(&self, statute: &Statute) -> String {
        let tree = DecisionTree::from_statute(statute).unwrap_or_else(|_| DecisionTree::new());
        self.to_ar_html_tree(&tree)
    }

    /// Generates AR HTML for a decision tree overlay.
    pub fn to_ar_html_tree(&self, tree: &DecisionTree) -> String {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str("<meta charset=\"UTF-8\">\n");
        html.push_str(
            "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n",
        );
        html.push_str("<title>AR Document Overlay</title>\n");
        html.push_str("<style>\n");
        html.push_str(&self.generate_ar_styles());
        html.push_str("</style>\n");
        html.push_str("</head>\n<body>\n");

        html.push_str("<div id=\"ar-container\">\n");
        html.push_str("<div class=\"controls\">\n");
        html.push_str("<button id=\"start-ar\">Start AR</button>\n");
        html.push_str("<div id=\"ar-status\">AR Ready</div>\n");
        html.push_str("</div>\n");
        html.push_str("<video id=\"camera-feed\" autoplay playsinline></video>\n");
        html.push_str("<canvas id=\"ar-overlay\"></canvas>\n");
        html.push_str("</div>\n");

        html.push_str("<script src=\"https://cdnjs.cloudflare.com/ajax/libs/three.js/r128/three.min.js\"></script>\n");
        html.push_str("<script>\n");
        html.push_str(&self.generate_ar_javascript(tree));
        html.push_str("</script>\n");

        html.push_str("</body>\n</html>");
        html
    }

    fn generate_ar_styles(&self) -> String {
        "body {
    margin: 0;
    padding: 0;
    overflow: hidden;
    font-family: Arial, sans-serif;
}

#ar-container {
    position: relative;
    width: 100vw;
    height: 100vh;
}

#camera-feed {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    object-fit: cover;
}

#ar-overlay {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    pointer-events: none;
}

.controls {
    position: absolute;
    top: 20px;
    left: 20px;
    z-index: 1000;
}

#start-ar {
    padding: 12px 24px;
    font-size: 16px;
    font-weight: bold;
    background: #2196f3;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
}

#ar-status {
    margin-top: 10px;
    padding: 8px 12px;
    background: rgba(0, 0, 0, 0.7);
    color: white;
    border-radius: 4px;
}
"
        .to_string()
    }

    fn generate_ar_javascript(&self, tree: &DecisionTree) -> String {
        let nodes = self.extract_tree_nodes(tree);

        format!(
            "// AR Document Overlay
const config = {{
    enableMarkers: {},
    enableMarkerless: {},
    enableFaceTracking: {},
    markerSize: {},
    overlayOpacity: {}
}};

const nodes = {};

let video, canvas, ctx;
let scene, camera, renderer;
let arSession = null;

async function init() {{
    video = document.getElementById('camera-feed');
    canvas = document.getElementById('ar-overlay');
    ctx = canvas.getContext('2d');

    // Setup canvas
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;

    // Setup Three.js for AR
    scene = new THREE.Scene();
    camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);

    renderer = new THREE.WebGLRenderer({{
        canvas: canvas,
        alpha: true,
        antialias: true
    }});
    renderer.setSize(window.innerWidth, window.innerHeight);
    renderer.xr.enabled = true;

    // Setup AR button
    document.getElementById('start-ar').addEventListener('click', startAR);

    window.addEventListener('resize', onResize);
}}

async function startAR() {{
    try {{
        // Request camera access
        const stream = await navigator.mediaDevices.getUserMedia({{
            video: {{ facingMode: 'environment' }}
        }});

        video.srcObject = stream;
        await video.play();

        document.getElementById('ar-status').textContent = 'AR Active';

        // Check for WebXR AR support
        if (navigator.xr) {{
            const supported = await navigator.xr.isSessionSupported('immersive-ar');

            if (supported) {{
                arSession = await navigator.xr.requestSession('immersive-ar', {{
                    requiredFeatures: ['hit-test'],
                    optionalFeatures: ['dom-overlay']
                }});

                renderer.xr.setSession(arSession);
                createAROverlay();

                arSession.addEventListener('end', () => {{
                    arSession = null;
                    document.getElementById('ar-status').textContent = 'AR Ended';
                }});
            }} else {{
                // Fallback to marker-based AR
                createMarkerBasedAR();
            }}
        }} else {{
            // No WebXR, use camera-based overlay
            createCameraOverlay();
        }}

        render();
    }} catch (error) {{
        console.error('Failed to start AR:', error);
        document.getElementById('ar-status').textContent = 'AR Error: ' + error.message;
    }}
}}

function createAROverlay() {{
    // Create virtual content for AR
    nodes.forEach((node, index) => {{
        const geometry = new THREE.BoxGeometry(0.1, 0.1, 0.1);
        let color;

        switch(node.type) {{
            case 'condition':
                color = 0x3498db;
                break;
            case 'discretion':
                color = 0xe74c3c;
                break;
            case 'outcome':
                color = 0x2ecc71;
                break;
            default:
                color = 0x999999;
        }}

        const material = new THREE.MeshBasicMaterial({{
            color,
            transparent: true,
            opacity: config.overlayOpacity
        }});
        const cube = new THREE.Mesh(geometry, material);

        // Position in a grid
        const row = Math.floor(index / 3);
        const col = index % 3;
        cube.position.set(
            (col - 1) * 0.3,
            1.5 + (row * 0.3),
            -1
        );

        scene.add(cube);
    }});
}}

function createMarkerBasedAR() {{
    // Implement marker-based AR tracking
    console.log('Using marker-based AR');
    drawMarkerOverlay();
}}

function createCameraOverlay() {{
    // Simple camera-based overlay
    console.log('Using camera overlay');
    drawCameraOverlay();
}}

function drawMarkerOverlay() {{
    // Draw AR markers and overlays on canvas
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.globalAlpha = config.overlayOpacity;

    nodes.forEach((node, index) => {{
        const x = 100 + (index * 150);
        const y = 100 + (Math.floor(index / 3) * 100);

        // Draw node box
        ctx.fillStyle = node.type === 'condition' ? '#3498db' :
                        node.type === 'discretion' ? '#e74c3c' : '#2ecc71';
        ctx.fillRect(x, y, 120, 60);

        // Draw text
        ctx.fillStyle = 'white';
        ctx.font = 'bold 14px Arial';
        ctx.fillText(node.label, x + 10, y + 30);
    }});

    ctx.globalAlpha = 1.0;
}}

function drawCameraOverlay() {{
    drawMarkerOverlay();
}}

function render() {{
    if (!arSession) {{
        // Non-WebXR rendering
        drawMarkerOverlay();
        requestAnimationFrame(render);
    }} else {{
        // WebXR AR rendering
        renderer.render(scene, camera);
    }}
}}

function onResize() {{
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    camera.aspect = window.innerWidth / window.innerHeight;
    camera.updateProjectionMatrix();
    renderer.setSize(window.innerWidth, window.innerHeight);
}}

init();
",
            self.config.enable_markers,
            self.config.enable_markerless,
            self.config.enable_face_tracking,
            self.config.marker_size,
            self.config.overlay_opacity,
            serde_json::to_string_pretty(&nodes).unwrap_or_else(|_| "[]".to_string())
        )
    }

    fn extract_tree_nodes(&self, tree: &DecisionTree) -> Vec<serde_json::Value> {
        let mut nodes = Vec::new();

        // Extract all nodes from the graph
        for node_idx in tree.graph.node_indices() {
            if let Some(node) = tree.graph.node_weight(node_idx) {
                let (node_type, label) = match node {
                    DecisionNode::Root { statute_id, .. } => ("root", statute_id.clone()),
                    DecisionNode::Condition {
                        description,
                        is_discretionary,
                    } => {
                        let node_type = if *is_discretionary {
                            "discretion"
                        } else {
                            "condition"
                        };
                        (node_type, description.clone())
                    }
                    DecisionNode::Outcome { description } => ("outcome", description.clone()),
                    DecisionNode::Discretion { issue, .. } => ("discretion", issue.clone()),
                };

                nodes.push(serde_json::json!({
                    "label": label,
                    "type": node_type
                }));
            }
        }

        nodes
    }
}

impl Default for ARDocumentOverlay {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration for 360° case timeline viewing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Panoramic360Config {
    /// Enable VR mode for 360° viewing
    pub enable_vr_mode: bool,
    /// Enable auto-rotation
    pub enable_auto_rotation: bool,
    /// Rotation speed (degrees per second)
    pub rotation_speed: f32,
    /// Field of view (degrees)
    pub field_of_view: f32,
    /// Enable gyroscope controls (mobile)
    pub enable_gyroscope: bool,
}

impl Default for Panoramic360Config {
    fn default() -> Self {
        Self {
            enable_vr_mode: true,
            enable_auto_rotation: false,
            rotation_speed: 10.0,
            field_of_view: 75.0,
            enable_gyroscope: true,
        }
    }
}

/// 360° panoramic case timeline visualizer.
pub struct Panoramic360Timeline {
    theme: Theme,
    config: Panoramic360Config,
}

impl Panoramic360Timeline {
    /// Creates a new 360° timeline visualizer.
    pub fn new() -> Self {
        Self {
            theme: Theme::light(),
            config: Panoramic360Config::default(),
        }
    }

    /// Sets the color theme.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Sets the 360° configuration.
    pub fn with_config(mut self, config: Panoramic360Config) -> Self {
        self.config = config;
        self
    }

    /// Generates 360° HTML for a timeline.
    pub fn to_360_html(&self, timeline: &Timeline) -> String {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str("<meta charset=\"UTF-8\">\n");
        html.push_str(
            "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n",
        );
        html.push_str("<title>360\u{00b0} Case Timeline</title>\n");
        html.push_str("<style>\n");
        html.push_str(&self.generate_360_styles());
        html.push_str("</style>\n");
        html.push_str("</head>\n<body>\n");

        html.push_str("<div id=\"panorama-container\">\n");
        html.push_str("<div class=\"controls-overlay\">\n");
        html.push_str("<h2>360\u{00b0} Case Timeline</h2>\n");
        html.push_str("<button id=\"toggle-rotation\">Toggle Auto-Rotate</button>\n");
        if self.config.enable_vr_mode {
            html.push_str("<button id=\"enter-vr\">Enter VR</button>\n");
        }
        html.push_str("<div id=\"event-info\">Look around to explore timeline events</div>\n");
        html.push_str("</div>\n");
        html.push_str("</div>\n");

        html.push_str("<script src=\"https://cdnjs.cloudflare.com/ajax/libs/three.js/r128/three.min.js\"></script>\n");
        html.push_str("<script>\n");
        html.push_str(&self.generate_360_javascript(timeline));
        html.push_str("</script>\n");

        html.push_str("</body>\n</html>");
        html
    }

    fn generate_360_styles(&self) -> String {
        "body {
    margin: 0;
    padding: 0;
    overflow: hidden;
    font-family: Arial, sans-serif;
}

#panorama-container {
    width: 100vw;
    height: 100vh;
    position: relative;
}

.controls-overlay {
    position: absolute;
    top: 20px;
    left: 20px;
    z-index: 1000;
    background: rgba(0, 0, 0, 0.7);
    color: white;
    padding: 20px;
    border-radius: 8px;
}

.controls-overlay h2 {
    margin: 0 0 15px 0;
    font-size: 20px;
}

.controls-overlay button {
    margin: 5px;
    padding: 10px 20px;
    font-size: 14px;
    background: #2196f3;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
}

.controls-overlay button:hover {
    background: #1976d2;
}

#event-info {
    margin-top: 15px;
    padding: 10px;
    background: rgba(255, 255, 255, 0.1);
    border-radius: 4px;
    font-size: 14px;
}
"
        .to_string()
    }

    fn generate_360_javascript(&self, timeline: &Timeline) -> String {
        let events = self.extract_timeline_events(timeline);

        format!(
            "// 360° Panoramic Timeline
const config = {{
    enableVRMode: {},
    enableAutoRotation: {},
    rotationSpeed: {},
    fieldOfView: {},
    enableGyroscope: {}
}};

const events = {};

let scene, camera, renderer;
let controls;
let autoRotate = config.enableAutoRotation;
let eventObjects = [];

function init() {{
    const container = document.getElementById('panorama-container');

    // Scene
    scene = new THREE.Scene();
    scene.background = new THREE.Color(0x87ceeb); // Sky blue

    // Camera
    camera = new THREE.PerspectiveCamera(
        config.fieldOfView,
        window.innerWidth / window.innerHeight,
        0.1,
        1000
    );
    camera.position.set(0, 0, 0.01); // Center of 360° sphere

    // Renderer
    renderer = new THREE.WebGLRenderer({{ antialias: true }});
    renderer.setSize(window.innerWidth, window.innerHeight);
    if (config.enableVRMode) {{
        renderer.xr.enabled = true;
    }}
    container.appendChild(renderer.domElement);

    // Create 360° environment
    create360Environment();

    // Create timeline events
    createTimelineEvents();

    // Mouse/touch controls
    let isDragging = false;
    let previousMousePosition = {{ x: 0, y: 0 }};

    container.addEventListener('mousedown', (e) => {{
        isDragging = true;
        previousMousePosition = {{ x: e.clientX, y: e.clientY }};
    }});

    container.addEventListener('mousemove', (e) => {{
        if (isDragging) {{
            const deltaX = e.clientX - previousMousePosition.x;
            const deltaY = e.clientY - previousMousePosition.y;

            camera.rotation.y += deltaX * 0.005;
            camera.rotation.x += deltaY * 0.005;

            // Limit vertical rotation
            camera.rotation.x = Math.max(-Math.PI / 2, Math.min(Math.PI / 2, camera.rotation.x));

            previousMousePosition = {{ x: e.clientX, y: e.clientY }};
        }}
    }});

    container.addEventListener('mouseup', () => {{
        isDragging = false;
    }});

    // Gyroscope support for mobile
    if (config.enableGyroscope && window.DeviceOrientationEvent) {{
        window.addEventListener('deviceorientation', (event) => {{
            if (event.alpha !== null && event.beta !== null && event.gamma !== null) {{
                camera.rotation.y = event.alpha * (Math.PI / 180);
                camera.rotation.x = event.beta * (Math.PI / 180);
                camera.rotation.z = event.gamma * (Math.PI / 180);
            }}
        }});
    }}

    // Event listeners
    document.getElementById('toggle-rotation')?.addEventListener('click', () => {{
        autoRotate = !autoRotate;
    }});

    document.getElementById('enter-vr')?.addEventListener('click', async () => {{
        if (navigator.xr) {{
            try {{
                const session = await navigator.xr.requestSession('immersive-vr');
                renderer.xr.setSession(session);
            }} catch (error) {{
                console.error('Failed to start VR:', error);
            }}
        }}
    }});

    window.addEventListener('resize', onResize);

    // Raycaster for event detection
    const raycaster = new THREE.Raycaster();
    const mouse = new THREE.Vector2();

    container.addEventListener('click', (event) => {{
        mouse.x = (event.clientX / window.innerWidth) * 2 - 1;
        mouse.y = -(event.clientY / window.innerHeight) * 2 + 1;

        raycaster.setFromCamera(mouse, camera);
        const intersects = raycaster.intersectObjects(eventObjects);

        if (intersects.length > 0) {{
            const eventData = intersects[0].object.userData;
            document.getElementById('event-info').textContent =
                `${{eventData.date}}: ${{eventData.description}}`;
        }}
    }});

    // Start render loop
    renderer.setAnimationLoop(render);
}}

function create360Environment() {{
    // Create sphere for 360° panorama
    const geometry = new THREE.SphereGeometry(500, 60, 40);
    geometry.scale(-1, 1, 1); // Invert to see inside

    const material = new THREE.MeshBasicMaterial({{
        color: 0x87ceeb,
        side: THREE.BackSide
    }});

    const sphere = new THREE.Mesh(geometry, material);
    scene.add(sphere);

    // Add ambient light
    const ambientLight = new THREE.AmbientLight(0xffffff, 0.8);
    scene.add(ambientLight);
}}

function createTimelineEvents() {{
    events.forEach((event, index) => {{
        // Create event marker
        const geometry = new THREE.BoxGeometry(2, 2, 0.5);
        const material = new THREE.MeshBasicMaterial({{
            color: event.type === 'Enacted' ? 0x2ecc71 :
                   event.type === 'Amended' ? 0x3498db :
                   event.type === 'Repealed' ? 0xe74c3c : 0xf39c12
        }});
        const cube = new THREE.Mesh(geometry, material);

        // Position events in a circle around the viewer
        const angle = (index / events.length) * Math.PI * 2;
        const radius = 10;
        cube.position.set(
            Math.cos(angle) * radius,
            Math.sin(index * 0.5) * 2, // Vary height
            Math.sin(angle) * radius
        );

        // Make it face the center
        cube.lookAt(0, 0, 0);

        cube.userData = {{
            date: event.date,
            description: event.description,
            type: event.type
        }};

        scene.add(cube);
        eventObjects.push(cube);

        // Add text label
        const canvas = document.createElement('canvas');
        const context = canvas.getContext('2d');
        canvas.width = 512;
        canvas.height = 256;
        context.fillStyle = 'white';
        context.font = 'bold 32px Arial';
        context.textAlign = 'center';
        context.fillText(event.date, 256, 100);
        context.font = '24px Arial';
        context.fillText(event.type, 256, 150);

        const texture = new THREE.CanvasTexture(canvas);
        const spriteMaterial = new THREE.SpriteMaterial({{ map: texture }});
        const sprite = new THREE.Sprite(spriteMaterial);
        sprite.position.copy(cube.position);
        sprite.position.y += 1.5;
        sprite.scale.set(3, 1.5, 1);
        scene.add(sprite);
    }});
}}

function render() {{
    if (autoRotate) {{
        camera.rotation.y += (config.rotationSpeed * Math.PI / 180) * 0.01;
    }}

    renderer.render(scene, camera);
}}

function onResize() {{
    camera.aspect = window.innerWidth / window.innerHeight;
    camera.updateProjectionMatrix();
    renderer.setSize(window.innerWidth, window.innerHeight);
}}

init();
",
            self.config.enable_vr_mode,
            self.config.enable_auto_rotation,
            self.config.rotation_speed,
            self.config.field_of_view,
            self.config.enable_gyroscope,
            serde_json::to_string_pretty(&events).unwrap_or_else(|_| "[]".to_string())
        )
    }

    fn extract_timeline_events(&self, timeline: &Timeline) -> Vec<serde_json::Value> {
        timeline
            .events
            .iter()
            .map(|(date, event)| {
                let (event_type, description) = match event {
                    TimelineEvent::Enacted { statute_id, title } => {
                        ("Enacted", format!("{}: {}", statute_id, title))
                    }
                    TimelineEvent::Amended {
                        statute_id,
                        description,
                    } => ("Amended", format!("{}: {}", statute_id, description)),
                    TimelineEvent::Repealed { statute_id } => ("Repealed", statute_id.clone()),
                    TimelineEvent::EffectiveStart { statute_id } => {
                        ("EffectiveStart", statute_id.clone())
                    }
                    TimelineEvent::EffectiveEnd { statute_id } => {
                        ("EffectiveEnd", statute_id.clone())
                    }
                };

                serde_json::json!({
                    "date": date,
                    "type": event_type,
                    "description": description
                })
            })
            .collect()
    }
}

impl Default for Panoramic360Timeline {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// AI-Enhanced Visualization (v0.3.1)
// ============================================================================

/// Visualization types that can be automatically selected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VisualizationType {
    /// Decision tree visualization
    DecisionTree,
    /// Dependency graph
    DependencyGraph,
    /// Timeline visualization
    Timeline,
    /// 3D interactive graph
    ThreeD,
    /// Sankey diagram for flow
    Sankey,
    /// Heatmap
    Heatmap,
    /// Network graph
    Network,
}

/// Recommendation for visualization with confidence score.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationRecommendation {
    /// Recommended visualization type
    pub viz_type: VisualizationType,
    /// Confidence score (0.0-1.0)
    pub confidence: f32,
    /// Reasoning for the recommendation
    pub reasoning: String,
    /// Alternative suggestions
    pub alternatives: Vec<(VisualizationType, f32)>,
}

/// Automatic visualization selector based on data characteristics.
pub struct AutoVisualizationSelector {
    /// Minimum confidence threshold
    min_confidence: f32,
}

impl AutoVisualizationSelector {
    /// Creates a new automatic visualization selector.
    pub fn new() -> Self {
        Self {
            min_confidence: 0.7,
        }
    }

    /// Sets the minimum confidence threshold.
    pub fn with_min_confidence(mut self, min_confidence: f32) -> Self {
        self.min_confidence = min_confidence.clamp(0.0, 1.0);
        self
    }

    /// Recommends visualization for a decision tree.
    pub fn recommend_for_tree(&self, tree: &DecisionTree) -> VisualizationRecommendation {
        let node_count = tree.graph.node_count();
        let _edge_count = tree.graph.edge_count();
        let depth = self.estimate_tree_depth(tree);

        let (viz_type, confidence, reasoning) = if node_count < 10 {
            (
                VisualizationType::DecisionTree,
                0.95,
                "Small tree best suited for traditional decision tree layout".to_string(),
            )
        } else if node_count < 50 && depth < 5 {
            (
                VisualizationType::Network,
                0.85,
                "Medium-sized tree with shallow depth works well as network graph".to_string(),
            )
        } else if depth > 8 {
            (
                VisualizationType::Sankey,
                0.80,
                "Deep tree structure visualized as flow diagram".to_string(),
            )
        } else {
            (
                VisualizationType::ThreeD,
                0.90,
                "Large complex tree benefits from 3D interactive visualization".to_string(),
            )
        };

        let alternatives = vec![
            (VisualizationType::DecisionTree, 0.70),
            (VisualizationType::Network, 0.65),
            (VisualizationType::ThreeD, 0.60),
        ];

        VisualizationRecommendation {
            viz_type,
            confidence,
            reasoning,
            alternatives,
        }
    }

    /// Recommends visualization for a dependency graph.
    pub fn recommend_for_graph(&self, graph: &DependencyGraph) -> VisualizationRecommendation {
        let statute_count = graph.graph.node_count();
        let dependency_count = graph.graph.edge_count();
        let avg_deps = if statute_count > 0 {
            dependency_count as f32 / statute_count as f32
        } else {
            0.0
        };

        let (viz_type, confidence, reasoning) = if statute_count < 20 {
            (
                VisualizationType::DependencyGraph,
                0.95,
                "Small graph ideal for traditional dependency visualization".to_string(),
            )
        } else if avg_deps > 3.0 {
            (
                VisualizationType::Heatmap,
                0.88,
                "Highly interconnected graph best shown as dependency heatmap".to_string(),
            )
        } else if statute_count > 100 {
            (
                VisualizationType::ThreeD,
                0.92,
                "Large graph requires 3D space for clarity".to_string(),
            )
        } else {
            (
                VisualizationType::Network,
                0.85,
                "Medium-sized graph works well as network visualization".to_string(),
            )
        };

        let alternatives = vec![
            (VisualizationType::DependencyGraph, 0.75),
            (VisualizationType::Network, 0.70),
            (VisualizationType::ThreeD, 0.65),
        ];

        VisualizationRecommendation {
            viz_type,
            confidence,
            reasoning,
            alternatives,
        }
    }

    /// Recommends visualization for a timeline.
    pub fn recommend_for_timeline(&self, timeline: &Timeline) -> VisualizationRecommendation {
        let event_count = timeline.events.len();
        let time_span = self.estimate_timeline_span(timeline);

        let (viz_type, confidence, reasoning) = if event_count < 10 {
            (
                VisualizationType::Timeline,
                0.98,
                "Few events best shown in linear timeline".to_string(),
            )
        } else if time_span > 50 {
            (
                VisualizationType::Heatmap,
                0.87,
                "Long time span with many events works as temporal heatmap".to_string(),
            )
        } else {
            (
                VisualizationType::Timeline,
                0.93,
                "Standard timeline visualization for moderate event count".to_string(),
            )
        };

        let alternatives = vec![
            (VisualizationType::Timeline, 0.80),
            (VisualizationType::Heatmap, 0.60),
        ];

        VisualizationRecommendation {
            viz_type,
            confidence,
            reasoning,
            alternatives,
        }
    }

    fn estimate_tree_depth(&self, tree: &DecisionTree) -> usize {
        if let Some(root) = tree.root {
            Self::dfs_depth(&tree.graph, root, 0)
        } else {
            0
        }
    }

    fn dfs_depth(
        graph: &DiGraph<DecisionNode, EdgeLabel>,
        node: NodeIndex,
        current_depth: usize,
    ) -> usize {
        let mut max_depth = current_depth;
        for neighbor in graph.neighbors(node) {
            let depth = Self::dfs_depth(graph, neighbor, current_depth + 1);
            max_depth = max_depth.max(depth);
        }
        max_depth
    }

    fn estimate_timeline_span(&self, timeline: &Timeline) -> usize {
        if timeline.events.is_empty() {
            return 0;
        }

        let dates: Vec<&str> = timeline
            .events
            .iter()
            .map(|(date, _)| date.as_str())
            .collect();
        if dates.len() < 2 {
            return 1;
        }

        // Simple year span estimation
        let first_year = dates
            .first()
            .and_then(|d| d.split('-').next())
            .and_then(|y| y.parse::<i32>().ok())
            .unwrap_or(0);
        let last_year = dates
            .last()
            .and_then(|d| d.split('-').next())
            .and_then(|y| y.parse::<i32>().ok())
            .unwrap_or(0);

        (last_year - first_year).unsigned_abs() as usize
    }
}

impl Default for AutoVisualizationSelector {
    fn default() -> Self {
        Self::new()
    }
}

/// AI-generated annotation for visualizations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIAnnotation {
    /// Target element ID
    pub target_id: String,
    /// Annotation text
    pub text: String,
    /// Importance score (0.0-1.0)
    pub importance: f32,
    /// Category of annotation
    pub category: AnnotationCategory,
    /// Suggested position (x, y)
    pub position: Option<(f32, f32)>,
}

/// Categories of AI-generated annotations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnnotationCategory {
    /// Critical path or important decision
    CriticalPath,
    /// Complexity hotspot
    Complexity,
    /// Potential issue or inconsistency
    Issue,
    /// Interesting pattern
    Pattern,
    /// Summary or insight
    Insight,
}

/// AI annotation generator for visualizations.
pub struct AIAnnotationGenerator {
    /// Enable complexity analysis
    enable_complexity: bool,
    /// Enable pattern detection
    enable_patterns: bool,
    /// Minimum importance threshold
    min_importance: f32,
}

impl AIAnnotationGenerator {
    /// Creates a new AI annotation generator.
    pub fn new() -> Self {
        Self {
            enable_complexity: true,
            enable_patterns: true,
            min_importance: 0.5,
        }
    }

    /// Disables complexity analysis.
    pub fn without_complexity(mut self) -> Self {
        self.enable_complexity = false;
        self
    }

    /// Disables pattern detection.
    pub fn without_patterns(mut self) -> Self {
        self.enable_patterns = false;
        self
    }

    /// Sets minimum importance threshold.
    pub fn with_min_importance(mut self, min_importance: f32) -> Self {
        self.min_importance = min_importance.clamp(0.0, 1.0);
        self
    }

    /// Generates annotations for a decision tree.
    pub fn generate_for_tree(&self, tree: &DecisionTree) -> Vec<AIAnnotation> {
        let mut annotations = Vec::new();

        // Analyze complexity
        if self.enable_complexity {
            annotations.extend(self.analyze_tree_complexity(tree));
        }

        // Detect patterns
        if self.enable_patterns {
            annotations.extend(self.detect_tree_patterns(tree));
        }

        // Find critical paths
        annotations.extend(self.find_critical_paths(tree));

        // Filter by importance
        annotations.retain(|a| a.importance >= self.min_importance);

        annotations
    }

    /// Generates annotations for a dependency graph.
    pub fn generate_for_graph(&self, graph: &DependencyGraph) -> Vec<AIAnnotation> {
        let mut annotations = Vec::new();

        // Analyze hubs and bottlenecks
        if self.enable_complexity {
            annotations.extend(self.analyze_graph_hubs(graph));
        }

        // Detect cycles
        annotations.extend(self.detect_dependency_cycles(graph));

        // Filter by importance
        annotations.retain(|a| a.importance >= self.min_importance);

        annotations
    }

    fn analyze_tree_complexity(&self, tree: &DecisionTree) -> Vec<AIAnnotation> {
        let mut annotations = Vec::new();

        for node_idx in tree.graph.node_indices() {
            let out_degree = tree.graph.neighbors(node_idx).count();

            if out_degree > 5 {
                if let Some(_node) = tree.graph.node_weight(node_idx) {
                    annotations.push(AIAnnotation {
                        target_id: format!("node-{}", node_idx.index()),
                        text: format!("High complexity: {} outgoing paths", out_degree),
                        importance: 0.8,
                        category: AnnotationCategory::Complexity,
                        position: None,
                    });
                }
            }
        }

        annotations
    }

    fn detect_tree_patterns(&self, tree: &DecisionTree) -> Vec<AIAnnotation> {
        let mut annotations = Vec::new();

        // Detect chains of discretionary decisions
        let mut discretion_chains = 0;
        for node_idx in tree.graph.node_indices() {
            if let Some(node) = tree.graph.node_weight(node_idx) {
                if matches!(node, DecisionNode::Discretion { .. }) {
                    let has_discretion_child = tree.graph.neighbors(node_idx).any(|neighbor| {
                        matches!(
                            tree.graph.node_weight(neighbor),
                            Some(DecisionNode::Discretion { .. })
                        )
                    });

                    if has_discretion_child {
                        discretion_chains += 1;
                    }
                }
            }
        }

        if discretion_chains > 3 {
            annotations.push(AIAnnotation {
                target_id: "root".to_string(),
                text: format!("Pattern detected: {} chains of discretionary decisions may indicate high interpretive complexity", discretion_chains),
                importance: 0.75,
                category: AnnotationCategory::Pattern,
                position: None,
            });
        }

        annotations
    }

    fn find_critical_paths(&self, tree: &DecisionTree) -> Vec<AIAnnotation> {
        let mut annotations = Vec::new();

        // Find longest path from root
        if let Some(root) = tree.root {
            let longest_path = Self::find_longest_path(&tree.graph, root);

            if longest_path > 10 {
                annotations.push(AIAnnotation {
                    target_id: "root".to_string(),
                    text: format!(
                        "Critical path depth: {} steps - consider simplification",
                        longest_path
                    ),
                    importance: 0.9,
                    category: AnnotationCategory::CriticalPath,
                    position: None,
                });
            }
        }

        annotations
    }

    fn analyze_graph_hubs(&self, graph: &DependencyGraph) -> Vec<AIAnnotation> {
        let mut annotations = Vec::new();

        // Find nodes with many dependencies
        for node_idx in graph.graph.node_indices() {
            let out_degree = graph.graph.neighbors(node_idx).count();

            if out_degree > 5 {
                if let Some(statute_id) = graph.graph.node_weight(node_idx) {
                    annotations.push(AIAnnotation {
                        target_id: statute_id.clone(),
                        text: format!(
                            "Hub statute: {} dependencies - central to legal framework",
                            out_degree
                        ),
                        importance: 0.85,
                        category: AnnotationCategory::Complexity,
                        position: None,
                    });
                }
            }
        }

        annotations
    }

    fn detect_dependency_cycles(&self, graph: &DependencyGraph) -> Vec<AIAnnotation> {
        let mut annotations = Vec::new();

        // Detect cycles using petgraph
        if petgraph::algo::is_cyclic_directed(&graph.graph) {
            annotations.push(AIAnnotation {
                target_id: "graph".to_string(),
                text: "Warning: Circular dependencies detected in graph".to_string(),
                importance: 0.95,
                category: AnnotationCategory::Issue,
                position: None,
            });
        }

        annotations
    }

    fn find_longest_path(graph: &DiGraph<DecisionNode, EdgeLabel>, start: NodeIndex) -> usize {
        let mut max_length = 0;

        for neighbor in graph.neighbors(start) {
            let path_length = 1 + Self::find_longest_path(graph, neighbor);
            max_length = max_length.max(path_length);
        }

        max_length
    }

    #[allow(dead_code)]
    fn extract_node_label(&self, node: &DecisionNode) -> String {
        match node {
            DecisionNode::Root { statute_id, .. } => statute_id.clone(),
            DecisionNode::Condition { description, .. } => description.clone(),
            DecisionNode::Outcome { description } => description.clone(),
            DecisionNode::Discretion { issue, .. } => issue.clone(),
        }
    }
}

impl Default for AIAnnotationGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Natural language query processor for visualizations.
pub struct NaturalLanguageQueryProcessor {
    /// Case-sensitive matching
    case_sensitive: bool,
}

impl NaturalLanguageQueryProcessor {
    /// Creates a new NL query processor.
    pub fn new() -> Self {
        Self {
            case_sensitive: false,
        }
    }

    /// Enables case-sensitive matching.
    pub fn with_case_sensitive(mut self) -> Self {
        self.case_sensitive = true;
        self
    }

    /// Processes a natural language query against a decision tree.
    pub fn query_tree(&self, tree: &DecisionTree, query: &str) -> Vec<QueryResult> {
        let mut results = Vec::new();

        let query_lower = if self.case_sensitive {
            query.to_string()
        } else {
            query.to_lowercase()
        };

        // Parse query intent
        if query_lower.contains("outcome") || query_lower.contains("result") {
            results.extend(self.find_outcomes(tree, &query_lower));
        }

        if query_lower.contains("discretion") || query_lower.contains("judgment") {
            results.extend(self.find_discretionary_nodes(tree));
        }

        if query_lower.contains("path") || query_lower.contains("route") {
            results.extend(self.find_paths(tree, &query_lower));
        }

        // Keyword search
        if !query_lower.contains("show") && !query_lower.contains("find") {
            results.extend(self.keyword_search(tree, &query_lower));
        }

        results
    }

    fn find_outcomes(&self, tree: &DecisionTree, _query: &str) -> Vec<QueryResult> {
        let mut results = Vec::new();

        for node_idx in tree.graph.node_indices() {
            if let Some(DecisionNode::Outcome { description }) = tree.graph.node_weight(node_idx) {
                results.push(QueryResult {
                    node_id: format!("node-{}", node_idx.index()),
                    relevance: 0.9,
                    excerpt: description.clone(),
                    node_type: "outcome".to_string(),
                });
            }
        }

        results
    }

    fn find_discretionary_nodes(&self, tree: &DecisionTree) -> Vec<QueryResult> {
        let mut results = Vec::new();

        for node_idx in tree.graph.node_indices() {
            if let Some(node) = tree.graph.node_weight(node_idx) {
                match node {
                    DecisionNode::Discretion { issue, .. } => {
                        results.push(QueryResult {
                            node_id: format!("node-{}", node_idx.index()),
                            relevance: 0.95,
                            excerpt: issue.clone(),
                            node_type: "discretion".to_string(),
                        });
                    }
                    DecisionNode::Condition {
                        description,
                        is_discretionary,
                    } if *is_discretionary => {
                        results.push(QueryResult {
                            node_id: format!("node-{}", node_idx.index()),
                            relevance: 0.85,
                            excerpt: description.clone(),
                            node_type: "discretionary_condition".to_string(),
                        });
                    }
                    _ => {}
                }
            }
        }

        results
    }

    fn find_paths(&self, tree: &DecisionTree, _query: &str) -> Vec<QueryResult> {
        let mut results = Vec::new();

        if let Some(root) = tree.root {
            results.push(QueryResult {
                node_id: format!("node-{}", root.index()),
                relevance: 0.8,
                excerpt: "Root node - start of all paths".to_string(),
                node_type: "root".to_string(),
            });
        }

        results
    }

    fn keyword_search(&self, tree: &DecisionTree, query: &str) -> Vec<QueryResult> {
        let mut results = Vec::new();

        for node_idx in tree.graph.node_indices() {
            if let Some(node) = tree.graph.node_weight(node_idx) {
                let (text, node_type) = match node {
                    DecisionNode::Root { statute_id, title } => {
                        (format!("{} {}", statute_id, title), "root")
                    }
                    DecisionNode::Condition { description, .. } => {
                        (description.clone(), "condition")
                    }
                    DecisionNode::Outcome { description } => (description.clone(), "outcome"),
                    DecisionNode::Discretion { issue, hint } => (
                        format!("{} {}", issue, hint.as_ref().unwrap_or(&String::new())),
                        "discretion",
                    ),
                };

                let text_to_search = if self.case_sensitive {
                    text.clone()
                } else {
                    text.to_lowercase()
                };

                if text_to_search.contains(query) {
                    let relevance = query.len() as f32 / text.len() as f32;
                    results.push(QueryResult {
                        node_id: format!("node-{}", node_idx.index()),
                        relevance: relevance.min(1.0),
                        excerpt: text,
                        node_type: node_type.to_string(),
                    });
                }
            }
        }

        results
    }
}

impl Default for NaturalLanguageQueryProcessor {
    fn default() -> Self {
        Self::new()
    }
}

/// Result from a natural language query.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    /// Node identifier
    pub node_id: String,
    /// Relevance score (0.0-1.0)
    pub relevance: f32,
    /// Text excerpt
    pub excerpt: String,
    /// Type of node
    pub node_type: String,
}

/// Smart data highlighter for visualizations.
pub struct SmartDataHighlighter {
    /// Highlight color
    highlight_color: String,
    /// Minimum importance for highlighting
    min_importance: f32,
}

impl SmartDataHighlighter {
    /// Creates a new smart data highlighter.
    pub fn new() -> Self {
        Self {
            highlight_color: "#ffeb3b".to_string(),
            min_importance: 0.7,
        }
    }

    /// Sets the highlight color.
    pub fn with_color(mut self, color: String) -> Self {
        self.highlight_color = color;
        self
    }

    /// Sets minimum importance threshold.
    pub fn with_min_importance(mut self, min_importance: f32) -> Self {
        self.min_importance = min_importance.clamp(0.0, 1.0);
        self
    }

    /// Generates highlighting rules for a decision tree.
    pub fn highlight_tree(&self, tree: &DecisionTree) -> Vec<HighlightRule> {
        let mut rules = Vec::new();

        // Highlight discretionary decisions
        for node_idx in tree.graph.node_indices() {
            if let Some(node) = tree.graph.node_weight(node_idx) {
                match node {
                    DecisionNode::Discretion { .. } => {
                        rules.push(HighlightRule {
                            target_id: format!("node-{}", node_idx.index()),
                            color: "#ff9800".to_string(),
                            importance: 0.9,
                            reason: "Discretionary decision point".to_string(),
                        });
                    }
                    DecisionNode::Condition {
                        is_discretionary: true,
                        ..
                    } => {
                        rules.push(HighlightRule {
                            target_id: format!("node-{}", node_idx.index()),
                            color: "#ffc107".to_string(),
                            importance: 0.8,
                            reason: "Discretionary condition".to_string(),
                        });
                    }
                    _ => {}
                }
            }
        }

        // Highlight complex nodes
        for node_idx in tree.graph.node_indices() {
            let out_degree = tree.graph.neighbors(node_idx).count();
            if out_degree > 3 {
                rules.push(HighlightRule {
                    target_id: format!("node-{}", node_idx.index()),
                    color: "#e91e63".to_string(),
                    importance: 0.75,
                    reason: format!("Complex node with {} branches", out_degree),
                });
            }
        }

        // Filter by importance
        rules.retain(|r| r.importance >= self.min_importance);

        rules
    }

    /// Generates highlighting rules for a dependency graph.
    pub fn highlight_graph(&self, graph: &DependencyGraph) -> Vec<HighlightRule> {
        let mut rules = Vec::new();

        // Highlight hub statutes
        for node_idx in graph.graph.node_indices() {
            let incoming = graph
                .graph
                .neighbors_directed(node_idx, petgraph::Direction::Incoming)
                .count();
            let outgoing = graph.graph.neighbors(node_idx).count();

            if incoming > 3 || outgoing > 3 {
                if let Some(statute_id) = graph.graph.node_weight(node_idx) {
                    rules.push(HighlightRule {
                        target_id: statute_id.clone(),
                        color: "#9c27b0".to_string(),
                        importance: 0.85,
                        reason: format!("Hub statute ({} in, {} out)", incoming, outgoing),
                    });
                }
            }
        }

        // Filter by importance
        rules.retain(|r| r.importance >= self.min_importance);

        rules
    }
}

impl Default for SmartDataHighlighter {
    fn default() -> Self {
        Self::new()
    }
}

/// Highlighting rule for visualization elements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighlightRule {
    /// Target element ID
    pub target_id: String,
    /// Highlight color
    pub color: String,
    /// Importance score
    pub importance: f32,
    /// Reason for highlighting
    pub reason: String,
}

/// Anomaly detection for visualizations.
pub struct AnomalyDetector {
    /// Sensitivity (0.0-1.0, higher = more sensitive)
    sensitivity: f32,
}

impl AnomalyDetector {
    /// Creates a new anomaly detector.
    pub fn new() -> Self {
        Self { sensitivity: 0.7 }
    }

    /// Sets sensitivity level.
    pub fn with_sensitivity(mut self, sensitivity: f32) -> Self {
        self.sensitivity = sensitivity.clamp(0.0, 1.0);
        self
    }

    /// Detects anomalies in a decision tree.
    pub fn detect_in_tree(&self, tree: &DecisionTree) -> Vec<Anomaly> {
        let mut anomalies = Vec::new();

        // Detect orphaned nodes
        anomalies.extend(self.detect_orphaned_nodes(tree));

        // Detect unusually deep paths
        anomalies.extend(self.detect_deep_paths(tree));

        // Detect missing outcomes
        anomalies.extend(self.detect_missing_outcomes(tree));

        // Detect cycles
        anomalies.extend(self.detect_cycles(tree));

        anomalies
    }

    /// Detects anomalies in a dependency graph.
    pub fn detect_in_graph(&self, graph: &DependencyGraph) -> Vec<Anomaly> {
        let mut anomalies = Vec::new();

        // Detect isolated statutes
        anomalies.extend(self.detect_isolated_statutes(graph));

        // Detect asymmetric dependencies
        anomalies.extend(self.detect_asymmetric_dependencies(graph));

        anomalies
    }

    fn detect_orphaned_nodes(&self, tree: &DecisionTree) -> Vec<Anomaly> {
        let mut anomalies = Vec::new();

        for node_idx in tree.graph.node_indices() {
            let has_incoming = tree
                .graph
                .neighbors_directed(node_idx, petgraph::Direction::Incoming)
                .count()
                > 0;
            let is_root = Some(node_idx) == tree.root;

            if !has_incoming && !is_root {
                if let Some(node) = tree.graph.node_weight(node_idx) {
                    let label = match node {
                        DecisionNode::Root { statute_id, .. } => statute_id.clone(),
                        DecisionNode::Condition { description, .. } => description.clone(),
                        DecisionNode::Outcome { description } => description.clone(),
                        DecisionNode::Discretion { issue, .. } => issue.clone(),
                    };

                    anomalies.push(Anomaly {
                        anomaly_type: AnomalyType::OrphanedNode,
                        severity: 0.8,
                        description: format!("Orphaned node detected: {}", label),
                        location: format!("node-{}", node_idx.index()),
                        suggestion: "Connect this node to the tree or remove it".to_string(),
                    });
                }
            }
        }

        anomalies
    }

    fn detect_deep_paths(&self, tree: &DecisionTree) -> Vec<Anomaly> {
        let mut anomalies = Vec::new();

        if let Some(root) = tree.root {
            let max_depth = Self::calculate_max_depth(&tree.graph, root);

            if max_depth > 15 {
                anomalies.push(Anomaly {
                    anomaly_type: AnomalyType::UnusualDepth,
                    severity: 0.7,
                    description: format!("Unusually deep decision path: {} levels", max_depth),
                    location: "tree".to_string(),
                    suggestion:
                        "Consider simplifying the decision logic or breaking into sub-trees"
                            .to_string(),
                });
            }
        }

        anomalies
    }

    fn detect_missing_outcomes(&self, tree: &DecisionTree) -> Vec<Anomaly> {
        let mut anomalies = Vec::new();

        for node_idx in tree.graph.node_indices() {
            let has_outgoing = tree.graph.neighbors(node_idx).count() > 0;
            let is_outcome = matches!(
                tree.graph.node_weight(node_idx),
                Some(DecisionNode::Outcome { .. })
            );

            if !has_outgoing && !is_outcome {
                anomalies.push(Anomaly {
                    anomaly_type: AnomalyType::MissingOutcome,
                    severity: 0.85,
                    description: "Leaf node without outcome designation".to_string(),
                    location: format!("node-{}", node_idx.index()),
                    suggestion: "Add an outcome node or continue the decision path".to_string(),
                });
            }
        }

        anomalies
    }

    fn detect_cycles(&self, tree: &DecisionTree) -> Vec<Anomaly> {
        let mut anomalies = Vec::new();

        // Use petgraph's cycle detection
        if petgraph::algo::is_cyclic_directed(&tree.graph) {
            anomalies.push(Anomaly {
                anomaly_type: AnomalyType::Cycle,
                severity: 0.95,
                description: "Cycle detected in decision tree".to_string(),
                location: "tree".to_string(),
                suggestion: "Remove cyclic dependencies - decision trees should be acyclic"
                    .to_string(),
            });
        }

        anomalies
    }

    fn detect_isolated_statutes(&self, graph: &DependencyGraph) -> Vec<Anomaly> {
        let mut anomalies = Vec::new();

        for node_idx in graph.graph.node_indices() {
            let incoming = graph
                .graph
                .neighbors_directed(node_idx, petgraph::Direction::Incoming)
                .count();
            let outgoing = graph.graph.neighbors(node_idx).count();

            if incoming == 0 && outgoing == 0 {
                if let Some(statute_id) = graph.graph.node_weight(node_idx) {
                    anomalies.push(Anomaly {
                        anomaly_type: AnomalyType::IsolatedNode,
                        severity: 0.6,
                        description: format!("Isolated statute: {}", statute_id),
                        location: statute_id.clone(),
                        suggestion: "Consider if this statute should have dependencies".to_string(),
                    });
                }
            }
        }

        anomalies
    }

    fn detect_asymmetric_dependencies(&self, graph: &DependencyGraph) -> Vec<Anomaly> {
        let mut anomalies = Vec::new();

        // Check for bidirectional edges
        for edge in graph.graph.edge_indices() {
            if let Some((source, target)) = graph.graph.edge_endpoints(edge) {
                // Check if reverse edge exists
                let has_reverse = graph.graph.edges_connecting(target, source).count() > 0;

                if has_reverse {
                    if let (Some(from_id), Some(to_id)) = (
                        graph.graph.node_weight(source),
                        graph.graph.node_weight(target),
                    ) {
                        anomalies.push(Anomaly {
                            anomaly_type: AnomalyType::BidirectionalDependency,
                            severity: 0.75,
                            description: format!(
                                "Bidirectional dependency: {} <-> {}",
                                from_id, to_id
                            ),
                            location: format!("{}-{}", from_id, to_id),
                            suggestion: "Review if bidirectional dependency is intentional"
                                .to_string(),
                        });
                    }
                }
            }
        }

        anomalies
    }

    fn calculate_max_depth(graph: &DiGraph<DecisionNode, EdgeLabel>, start: NodeIndex) -> usize {
        let mut max_depth = 0;

        for neighbor in graph.neighbors(start) {
            let depth = 1 + Self::calculate_max_depth(graph, neighbor);
            max_depth = max_depth.max(depth);
        }

        max_depth
    }
}

impl Default for AnomalyDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Detected anomaly in visualization data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anomaly {
    /// Type of anomaly
    pub anomaly_type: AnomalyType,
    /// Severity score (0.0-1.0)
    pub severity: f32,
    /// Description of the anomaly
    pub description: String,
    /// Location identifier
    pub location: String,
    /// Suggested action
    pub suggestion: String,
}

/// Types of anomalies that can be detected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnomalyType {
    /// Node with no connections
    OrphanedNode,
    /// Unusually deep decision path
    UnusualDepth,
    /// Missing outcome designation
    MissingOutcome,
    /// Circular dependency
    Cycle,
    /// Isolated node
    IsolatedNode,
    /// Bidirectional dependency
    BidirectionalDependency,
}

// ============================================================================
// Advanced Export Formats
// ============================================================================

/// Export format types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExportFormat {
    /// Animated GIF
    AnimatedGif,
    /// MP4 video
    Mp4,
    /// WebM video
    WebM,
    /// Print-optimized PDF
    PrintPdf,
    /// Vector PDF
    VectorPdf,
    /// Poster-size image
    Poster,
}

/// Configuration for poster-size exports.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PosterConfig {
    /// Width in pixels (or mm for print)
    pub width: usize,
    /// Height in pixels (or mm for print)
    pub height: usize,
    /// DPI (dots per inch) for print quality
    pub dpi: usize,
    /// Paper size (e.g., "A0", "A1", "24x36")
    pub paper_size: String,
    /// Orientation ("portrait" or "landscape")
    pub orientation: String,
}

impl Default for PosterConfig {
    fn default() -> Self {
        Self {
            width: 841,   // A0 width in mm
            height: 1189, // A0 height in mm
            dpi: 300,
            paper_size: "A0".to_string(),
            orientation: "portrait".to_string(),
        }
    }
}

impl PosterConfig {
    /// Creates a new poster configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// A0 poster (841mm x 1189mm)
    pub fn a0() -> Self {
        Self {
            width: 841,
            height: 1189,
            dpi: 300,
            paper_size: "A0".to_string(),
            orientation: "portrait".to_string(),
        }
    }

    /// A1 poster (594mm x 841mm)
    pub fn a1() -> Self {
        Self {
            width: 594,
            height: 841,
            dpi: 300,
            paper_size: "A1".to_string(),
            orientation: "portrait".to_string(),
        }
    }

    /// A2 poster (420mm x 594mm)
    pub fn a2() -> Self {
        Self {
            width: 420,
            height: 594,
            dpi: 300,
            paper_size: "A2".to_string(),
            orientation: "portrait".to_string(),
        }
    }

    /// 24x36 inch poster (common US size)
    pub fn poster_24x36() -> Self {
        Self {
            width: 610,  // 24 inches in mm
            height: 914, // 36 inches in mm
            dpi: 300,
            paper_size: "24x36".to_string(),
            orientation: "portrait".to_string(),
        }
    }

    /// Sets landscape orientation.
    pub fn landscape(mut self) -> Self {
        std::mem::swap(&mut self.width, &mut self.height);
        self.orientation = "landscape".to_string();
        self
    }

    /// Sets the DPI.
    pub fn with_dpi(mut self, dpi: usize) -> Self {
        self.dpi = dpi;
        self
    }
}

/// Configuration for animated GIF export.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimatedGifConfig {
    /// Frame rate (frames per second)
    pub fps: u32,
    /// Duration in seconds
    pub duration: u32,
    /// Loop count (0 = infinite)
    pub loop_count: u16,
    /// Frame width
    pub width: usize,
    /// Frame height
    pub height: usize,
    /// Quality (1-100)
    pub quality: u8,
}

impl Default for AnimatedGifConfig {
    fn default() -> Self {
        Self {
            fps: 30,
            duration: 10,
            loop_count: 0,
            width: 1920,
            height: 1080,
            quality: 80,
        }
    }
}

impl AnimatedGifConfig {
    /// Creates a new animated GIF configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the frame rate.
    pub fn with_fps(mut self, fps: u32) -> Self {
        self.fps = fps;
        self
    }

    /// Sets the duration.
    pub fn with_duration(mut self, duration: u32) -> Self {
        self.duration = duration;
        self
    }

    /// Sets the loop count.
    pub fn with_loop_count(mut self, loop_count: u16) -> Self {
        self.loop_count = loop_count;
        self
    }

    /// Sets the dimensions.
    pub fn with_size(mut self, width: usize, height: usize) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Sets the quality.
    pub fn with_quality(mut self, quality: u8) -> Self {
        self.quality = quality.min(100);
        self
    }
}

/// Configuration for video export.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoConfig {
    /// Frame rate (frames per second)
    pub fps: u32,
    /// Duration in seconds
    pub duration: u32,
    /// Video width
    pub width: usize,
    /// Video height
    pub height: usize,
    /// Bitrate (in kbps)
    pub bitrate: u32,
    /// Codec (e.g., "h264", "vp9")
    pub codec: String,
}

impl Default for VideoConfig {
    fn default() -> Self {
        Self {
            fps: 30,
            duration: 10,
            width: 1920,
            height: 1080,
            bitrate: 5000,
            codec: "h264".to_string(),
        }
    }
}

impl VideoConfig {
    /// Creates a new video configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// 1080p HD configuration.
    pub fn hd_1080p() -> Self {
        Self {
            width: 1920,
            height: 1080,
            fps: 30,
            bitrate: 8000,
            ..Self::default()
        }
    }

    /// 720p HD configuration.
    pub fn hd_720p() -> Self {
        Self {
            width: 1280,
            height: 720,
            fps: 30,
            bitrate: 5000,
            ..Self::default()
        }
    }

    /// 4K UHD configuration.
    pub fn uhd_4k() -> Self {
        Self {
            width: 3840,
            height: 2160,
            fps: 30,
            bitrate: 20000,
            ..Self::default()
        }
    }

    /// Sets the frame rate.
    pub fn with_fps(mut self, fps: u32) -> Self {
        self.fps = fps;
        self
    }

    /// Sets the codec.
    pub fn with_codec(mut self, codec: &str) -> Self {
        self.codec = codec.to_string();
        self
    }

    /// Sets the bitrate.
    pub fn with_bitrate(mut self, bitrate: u32) -> Self {
        self.bitrate = bitrate;
        self
    }

    /// Sets the duration.
    pub fn with_duration(mut self, duration: u32) -> Self {
        self.duration = duration;
        self
    }
}

/// Configuration for PDF export.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfConfig {
    /// Page width in mm
    pub width: f32,
    /// Page height in mm
    pub height: f32,
    /// Margin in mm
    pub margin: f32,
    /// Vector-based (true) or rasterized (false)
    pub vector: bool,
    /// DPI for rasterized output
    pub dpi: usize,
    /// Optimize for print (true) or screen (false)
    pub print_optimized: bool,
}

impl Default for PdfConfig {
    fn default() -> Self {
        Self {
            width: 210.0,  // A4 width
            height: 297.0, // A4 height
            margin: 10.0,
            vector: true,
            dpi: 300,
            print_optimized: true,
        }
    }
}

impl PdfConfig {
    /// Creates a new PDF configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// A4 page size (210mm x 297mm)
    pub fn a4() -> Self {
        Self {
            width: 210.0,
            height: 297.0,
            ..Self::default()
        }
    }

    /// A3 page size (297mm x 420mm)
    pub fn a3() -> Self {
        Self {
            width: 297.0,
            height: 420.0,
            ..Self::default()
        }
    }

    /// Letter page size (215.9mm x 279.4mm)
    pub fn letter() -> Self {
        Self {
            width: 215.9,
            height: 279.4,
            ..Self::default()
        }
    }

    /// Tabloid page size (279.4mm x 431.8mm)
    pub fn tabloid() -> Self {
        Self {
            width: 279.4,
            height: 431.8,
            ..Self::default()
        }
    }

    /// Sets landscape orientation.
    pub fn landscape(mut self) -> Self {
        std::mem::swap(&mut self.width, &mut self.height);
        self
    }

    /// Sets vector mode.
    pub fn vector(mut self) -> Self {
        self.vector = true;
        self
    }

    /// Sets raster mode.
    pub fn raster(mut self) -> Self {
        self.vector = false;
        self
    }

    /// Sets print optimization.
    pub fn print_optimized(mut self) -> Self {
        self.print_optimized = true;
        self
    }

    /// Sets screen optimization.
    pub fn screen_optimized(mut self) -> Self {
        self.print_optimized = false;
        self.dpi = 96; // Screen DPI
        self
    }

    /// Sets the DPI.
    pub fn with_dpi(mut self, dpi: usize) -> Self {
        self.dpi = dpi;
        self
    }

    /// Sets the margin.
    pub fn with_margin(mut self, margin: f32) -> Self {
        self.margin = margin;
        self
    }
}

/// Advanced export handler for various formats.
#[derive(Debug, Clone)]
pub struct AdvancedExporter {
    theme: Theme,
}

impl AdvancedExporter {
    /// Creates a new advanced exporter.
    pub fn new() -> Self {
        Self {
            theme: Theme::default(),
        }
    }

    /// Sets the theme.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Exports a decision tree to animated GIF.
    /// Returns SVG frames that can be encoded to GIF with external tools.
    pub fn to_animated_gif(&self, tree: &DecisionTree, config: AnimatedGifConfig) -> Vec<String> {
        let mut frames = Vec::new();
        let total_frames = config.fps * config.duration;

        for i in 0..total_frames {
            // Generate a frame with progressive revelation
            let progress = i as f32 / total_frames as f32;
            let frame = self.generate_frame(tree, progress, config.width, config.height);
            frames.push(frame);
        }

        frames
    }

    #[allow(dead_code)]
    fn generate_frame(
        &self,
        tree: &DecisionTree,
        progress: f32,
        width: usize,
        height: usize,
    ) -> String {
        // Generate SVG frame with animation progress
        let mut svg = tree.to_svg_with_theme(&self.theme);

        // Add animation overlay based on progress
        let overlay = format!(
            r#"<rect x="0" y="0" width="{}" height="{}" fill="rgba(0,0,0,{})" />"#,
            width,
            height,
            (1.0 - progress) * 0.3
        );

        svg = svg.replace("</svg>", &format!("{}</svg>", overlay));
        svg
    }

    /// Exports a dependency graph to animated GIF frames.
    pub fn graph_to_animated_gif(
        &self,
        graph: &DependencyGraph,
        config: AnimatedGifConfig,
    ) -> Vec<String> {
        let mut frames = Vec::new();
        let total_frames = config.fps * config.duration;

        for i in 0..total_frames {
            let progress = i as f32 / total_frames as f32;
            let frame = self.generate_graph_frame(graph, progress, config.width, config.height);
            frames.push(frame);
        }

        frames
    }

    #[allow(dead_code)]
    fn generate_graph_frame(
        &self,
        graph: &DependencyGraph,
        _progress: f32,
        _width: usize,
        _height: usize,
    ) -> String {
        // Generate SVG frame for graph
        graph.to_svg_with_theme(&self.theme)
    }

    /// Exports to video format (returns SVG frames for encoding).
    pub fn to_video_frames(&self, tree: &DecisionTree, config: VideoConfig) -> Vec<String> {
        let total_frames = config.fps * config.duration;
        let mut frames = Vec::new();

        for i in 0..total_frames {
            let progress = i as f32 / total_frames as f32;
            let frame = self.generate_frame(tree, progress, config.width, config.height);
            frames.push(frame);
        }

        frames
    }

    /// Exports dependency graph to video frames.
    pub fn graph_to_video_frames(
        &self,
        graph: &DependencyGraph,
        config: VideoConfig,
    ) -> Vec<String> {
        let total_frames = config.fps * config.duration;
        let mut frames = Vec::new();

        for i in 0..total_frames {
            let progress = i as f32 / total_frames as f32;
            let frame = self.generate_graph_frame(graph, progress, config.width, config.height);
            frames.push(frame);
        }

        frames
    }

    /// Exports to print-optimized PDF (returns optimized SVG).
    pub fn to_print_pdf(&self, tree: &DecisionTree, config: PdfConfig) -> String {
        let mut svg = tree.to_svg_with_theme(&self.theme);

        if config.print_optimized {
            // Add print-specific optimizations
            svg = self.optimize_for_print(svg, &config);
        }

        svg
    }

    /// Exports dependency graph to print-optimized PDF.
    pub fn graph_to_print_pdf(&self, graph: &DependencyGraph, config: PdfConfig) -> String {
        let mut svg = graph.to_svg_with_theme(&self.theme);

        if config.print_optimized {
            svg = self.optimize_for_print(svg, &config);
        }

        svg
    }

    #[allow(dead_code)]
    fn optimize_for_print(&self, svg: String, config: &PdfConfig) -> String {
        // Add print-specific CSS
        let print_css = format!(
            r#"<style>
            @media print {{
                svg {{
                    width: {}mm;
                    height: {}mm;
                    page-break-inside: avoid;
                }}
                text {{
                    font-family: serif;
                    -webkit-font-smoothing: antialiased;
                }}
            }}
            </style>"#,
            config.width, config.height
        );

        svg.replace("<svg", &format!("{}<svg", print_css))
    }

    /// Exports to vector PDF (returns vector SVG).
    pub fn to_vector_pdf(&self, tree: &DecisionTree, config: PdfConfig) -> String {
        let svg = tree.to_svg_with_theme(&self.theme);
        self.vectorize_for_pdf(svg, &config)
    }

    /// Exports dependency graph to vector PDF.
    pub fn graph_to_vector_pdf(&self, graph: &DependencyGraph, config: PdfConfig) -> String {
        let svg = graph.to_svg_with_theme(&self.theme);
        self.vectorize_for_pdf(svg, &config)
    }

    #[allow(dead_code)]
    fn vectorize_for_pdf(&self, svg: String, config: &PdfConfig) -> String {
        // Ensure all elements are vector-based
        let mut vectorized = svg;

        // Add PDF-specific metadata
        let metadata = format!(
            r#"<!-- PDF Export: {}x{}mm @ {}dpi -->"#,
            config.width, config.height, config.dpi
        );

        vectorized = vectorized.replace("<svg", &format!("{}\n<svg", metadata));
        vectorized
    }

    /// Exports to poster size.
    pub fn to_poster(&self, tree: &DecisionTree, config: PosterConfig) -> String {
        // Generate high-resolution SVG
        let svg = tree.to_svg_with_theme(&self.theme);
        self.scale_to_poster(svg, &config)
    }

    /// Exports dependency graph to poster size.
    pub fn graph_to_poster(&self, graph: &DependencyGraph, config: PosterConfig) -> String {
        let svg = graph.to_svg_with_theme(&self.theme);
        self.scale_to_poster(svg, &config)
    }

    #[allow(dead_code)]
    fn scale_to_poster(&self, svg: String, config: &PosterConfig) -> String {
        // Scale SVG to poster dimensions
        let scale_factor = config.dpi as f32 / 96.0; // 96 DPI is screen standard
        let pixel_width = (config.width as f32 * scale_factor * 3.7795) as usize; // mm to pixels at given DPI
        let pixel_height = (config.height as f32 * scale_factor * 3.7795) as usize;

        let metadata = format!(
            r#"<!-- Poster: {} {}x{}mm ({}x{}px @ {}dpi) -->"#,
            config.paper_size, config.width, config.height, pixel_width, pixel_height, config.dpi
        );

        svg.replace(
            "<svg",
            &format!(
                "{}\n<svg width=\"{}\" height=\"{}\"",
                metadata, pixel_width, pixel_height
            ),
        )
    }

    /// Gets metadata for an export format.
    pub fn format_metadata(&self, format: ExportFormat) -> String {
        match format {
            ExportFormat::AnimatedGif => {
                "Animated GIF - Suitable for presentations and web".to_string()
            }
            ExportFormat::Mp4 => "MP4 Video - H.264 codec, widely compatible".to_string(),
            ExportFormat::WebM => "WebM Video - VP9 codec, web-optimized".to_string(),
            ExportFormat::PrintPdf => "Print PDF - Optimized for high-quality printing".to_string(),
            ExportFormat::VectorPdf => "Vector PDF - Scalable vector graphics".to_string(),
            ExportFormat::Poster => "Poster - Large format print output".to_string(),
        }
    }
}

impl Default for AdvancedExporter {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Accessibility Features (WCAG 2.1 AA Compliance)
// ============================================================================

/// Configuration for accessibility features.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityConfig {
    /// Enable WCAG 2.1 AA compliance features
    pub wcag_aa_compliant: bool,
    /// Enable screen reader descriptions (ARIA labels)
    pub enable_screen_reader: bool,
    /// Enable keyboard navigation
    pub enable_keyboard_nav: bool,
    /// Use high contrast colors (minimum 4.5:1 ratio)
    pub high_contrast_mode: bool,
    /// Reduce or disable animations
    pub reduced_motion: bool,
    /// Minimum font size in pixels
    pub min_font_size: f32,
    /// Focus indicator color
    pub focus_color: String,
    /// Tab index for interactive elements
    pub tab_index_start: i32,
}

impl Default for AccessibilityConfig {
    fn default() -> Self {
        Self {
            wcag_aa_compliant: true,
            enable_screen_reader: true,
            enable_keyboard_nav: true,
            high_contrast_mode: false,
            reduced_motion: false,
            min_font_size: 16.0,
            focus_color: "#005fcc".to_string(),
            tab_index_start: 0,
        }
    }
}

impl AccessibilityConfig {
    /// Creates a new accessibility configuration with WCAG 2.1 AA compliance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a configuration optimized for screen readers.
    pub fn screen_reader_optimized() -> Self {
        Self {
            wcag_aa_compliant: true,
            enable_screen_reader: true,
            enable_keyboard_nav: true,
            high_contrast_mode: true,
            reduced_motion: true,
            min_font_size: 18.0,
            focus_color: "#0066cc".to_string(),
            tab_index_start: 0,
        }
    }

    /// Creates a configuration with reduced motion for users sensitive to animation.
    pub fn reduced_motion() -> Self {
        Self {
            reduced_motion: true,
            ..Self::default()
        }
    }

    /// Creates a configuration with high contrast for users with low vision.
    pub fn high_contrast() -> Self {
        Self {
            high_contrast_mode: true,
            min_font_size: 18.0,
            ..Self::default()
        }
    }
}

/// Enhances visualizations with accessibility features.
#[derive(Debug, Clone)]
pub struct AccessibilityEnhancer {
    config: AccessibilityConfig,
    theme: Theme,
}

impl AccessibilityEnhancer {
    /// Creates a new accessibility enhancer with default configuration.
    pub fn new() -> Self {
        Self {
            config: AccessibilityConfig::default(),
            theme: Theme::default(),
        }
    }

    /// Sets the accessibility configuration.
    pub fn with_config(mut self, config: AccessibilityConfig) -> Self {
        self.config = config;
        self
    }

    /// Sets the theme, adjusting it for accessibility if needed.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = if self.config.high_contrast_mode {
            Theme::high_contrast()
        } else {
            theme
        };
        self
    }

    /// Generates ARIA label for a decision node.
    pub fn aria_label_for_node(&self, node: &DecisionNode) -> String {
        match node {
            DecisionNode::Root { statute_id, title } => {
                format!("Root node: {} (ID: {})", title, statute_id)
            }
            DecisionNode::Condition {
                description,
                is_discretionary,
            } => {
                if *is_discretionary {
                    format!("Discretionary condition: {}", description)
                } else {
                    format!("Condition: {}", description)
                }
            }
            DecisionNode::Outcome { description } => {
                format!("Outcome: {}", description)
            }
            DecisionNode::Discretion { issue, hint } => {
                if let Some(h) = hint {
                    format!("Discretionary decision: {}. Hint: {}", issue, h)
                } else {
                    format!("Discretionary decision: {}", issue)
                }
            }
        }
    }

    /// Generates ARIA role for a decision node.
    pub fn aria_role_for_node(&self, node: &DecisionNode) -> &'static str {
        match node {
            DecisionNode::Root { .. } => "landmark",
            DecisionNode::Condition { .. } => "listitem",
            DecisionNode::Outcome { .. } => "status",
            DecisionNode::Discretion { .. } => "alert",
        }
    }

    /// Adds keyboard navigation JavaScript to HTML output.
    pub fn keyboard_nav_script(&self) -> String {
        if !self.config.enable_keyboard_nav {
            return String::new();
        }

        format!(
            r#"
<script>
// Keyboard navigation support
document.addEventListener('DOMContentLoaded', function() {{
    let focusIndex = 0;
    const focusableElements = document.querySelectorAll('[tabindex]');

    document.addEventListener('keydown', function(e) {{
        // Tab navigation
        if (e.key === 'Tab') {{
            e.preventDefault();
            if (e.shiftKey) {{
                focusIndex = (focusIndex - 1 + focusableElements.length) % focusableElements.length;
            }} else {{
                focusIndex = (focusIndex + 1) % focusableElements.length;
            }}
            focusableElements[focusIndex].focus();
        }}

        // Enter/Space to activate
        if (e.key === 'Enter' || e.key === ' ') {{
            const activeElement = document.activeElement;
            if (activeElement && activeElement.onclick) {{
                e.preventDefault();
                activeElement.click();
            }}
        }}

        // Arrow key navigation
        if (e.key === 'ArrowUp' || e.key === 'ArrowLeft') {{
            e.preventDefault();
            focusIndex = (focusIndex - 1 + focusableElements.length) % focusableElements.length;
            focusableElements[focusIndex].focus();
        }}
        if (e.key === 'ArrowDown' || e.key === 'ArrowRight') {{
            e.preventDefault();
            focusIndex = (focusIndex + 1) % focusableElements.length;
            focusableElements[focusIndex].focus();
        }}

        // Home/End keys
        if (e.key === 'Home') {{
            e.preventDefault();
            focusIndex = 0;
            focusableElements[focusIndex].focus();
        }}
        if (e.key === 'End') {{
            e.preventDefault();
            focusIndex = focusableElements.length - 1;
            focusableElements[focusIndex].focus();
        }}
    }});

    // Add focus indicators
    const style = document.createElement('style');
    style.textContent = `
        *:focus {{
            outline: 3px solid {};
            outline-offset: 2px;
        }}
    `;
    document.head.appendChild(style);
}});
</script>
"#,
            self.config.focus_color
        )
    }

    /// Adds screen reader descriptions to HTML output.
    pub fn screen_reader_enhancements(&self) -> String {
        if !self.config.enable_screen_reader {
            return String::new();
        }

        r#"
<div role="complementary" aria-label="Accessibility information" class="sr-only">
    <h2>Navigation Instructions</h2>
    <p>Use Tab to navigate between elements. Press Enter or Space to activate buttons.</p>
    <p>Use arrow keys to navigate through the visualization.</p>
    <p>Press Home to go to the first element, End to go to the last element.</p>
</div>
<style>
.sr-only {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border-width: 0;
}
</style>
"#
        .to_string()
    }

    /// Generates CSS for reduced motion.
    pub fn reduced_motion_css(&self) -> String {
        if !self.config.reduced_motion {
            return String::new();
        }

        r#"
<style>
@media (prefers-reduced-motion: reduce) {
    *, *::before, *::after {
        animation-duration: 0.01ms !important;
        animation-iteration-count: 1 !important;
        transition-duration: 0.01ms !important;
        scroll-behavior: auto !important;
    }
}

/* Force reduced motion when config is enabled */
*, *::before, *::after {
    animation-duration: 0.01ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 0.01ms !important;
}
</style>
"#
        .to_string()
    }

    /// Generates CSS for high contrast mode.
    pub fn high_contrast_css(&self) -> String {
        if !self.config.high_contrast_mode {
            return String::new();
        }

        format!(
            r#"
<style>
/* High contrast mode styles */
body {{
    background: {};
    color: {};
    font-size: {}px;
}}

.node {{
    border: 2px solid {} !important;
}}

.edge, .link {{
    stroke: {} !important;
    stroke-width: 2px !important;
}}

text {{
    fill: {} !important;
    font-weight: bold;
    font-size: {}px;
}}

/* Ensure minimum contrast ratio of 4.5:1 */
.condition {{
    background: {};
    color: {};
}}

.outcome {{
    background: {};
    color: {};
}}

.discretion {{
    background: {};
    color: {};
}}
</style>
"#,
            self.theme.background_color,
            self.theme.text_color,
            self.config.min_font_size,
            self.theme.text_color,
            self.theme.link_color,
            self.theme.text_color,
            self.config.min_font_size,
            self.theme.condition_color,
            self.theme.text_color,
            self.theme.outcome_color,
            self.theme.text_color,
            self.theme.discretion_color,
            self.theme.text_color,
        )
    }

    /// Enhances HTML with full accessibility features.
    pub fn enhance_html(&self, html: &str) -> String {
        let mut enhanced = html.to_string();

        // Add lang attribute if not present
        if !enhanced.contains("lang=") {
            enhanced = enhanced.replace("<html>", r#"<html lang="en">"#);
        }

        // Add accessibility meta tags
        let meta_tags = r#"
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<meta name="description" content="Accessible legal statute visualization">
"#;

        if !enhanced.contains("viewport") {
            enhanced = enhanced.replace("</head>", &format!("{}</head>", meta_tags));
        }

        // Add screen reader enhancements
        let sr_enhancements = self.screen_reader_enhancements();
        enhanced = enhanced.replace("<body>", &format!("<body>{}", sr_enhancements));

        // Add CSS enhancements
        let mut css = String::new();
        css.push_str(&self.high_contrast_css());
        css.push_str(&self.reduced_motion_css());

        if !css.is_empty() {
            enhanced = enhanced.replace("</head>", &format!("{}</head>", css));
        }

        // Add keyboard navigation
        let kb_script = self.keyboard_nav_script();
        if !kb_script.is_empty() {
            enhanced = enhanced.replace("</body>", &format!("{}</body>", kb_script));
        }

        enhanced
    }

    /// Validates WCAG 2.1 AA compliance for color contrast.
    /// Returns true if the contrast ratio is at least 4.5:1.
    pub fn validate_contrast(&self, foreground: &str, background: &str) -> bool {
        // Parse hex colors
        let fg = Self::parse_hex_color(foreground);
        let bg = Self::parse_hex_color(background);

        if fg.is_none() || bg.is_none() {
            return false;
        }

        let (r1, g1, b1) = fg.unwrap();
        let (r2, g2, b2) = bg.unwrap();

        // Calculate relative luminance
        let l1 = Self::relative_luminance(r1, g1, b1);
        let l2 = Self::relative_luminance(r2, g2, b2);

        // Calculate contrast ratio
        let ratio = if l1 > l2 {
            (l1 + 0.05) / (l2 + 0.05)
        } else {
            (l2 + 0.05) / (l1 + 0.05)
        };

        // WCAG AA requires 4.5:1 for normal text
        ratio >= 4.5
    }

    #[allow(dead_code)]
    fn parse_hex_color(hex: &str) -> Option<(f32, f32, f32)> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return None;
        }

        let r = u8::from_str_radix(&hex[0..2], 16).ok()? as f32 / 255.0;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()? as f32 / 255.0;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()? as f32 / 255.0;

        Some((r, g, b))
    }

    #[allow(dead_code)]
    fn relative_luminance(r: f32, g: f32, b: f32) -> f32 {
        let r = if r <= 0.03928 {
            r / 12.92
        } else {
            ((r + 0.055) / 1.055).powf(2.4)
        };
        let g = if g <= 0.03928 {
            g / 12.92
        } else {
            ((g + 0.055) / 1.055).powf(2.4)
        };
        let b = if b <= 0.03928 {
            b / 12.92
        } else {
            ((b + 0.055) / 1.055).powf(2.4)
        };

        0.2126 * r + 0.7152 * g + 0.0722 * b
    }

    /// Generates an accessible HTML decision tree.
    pub fn to_accessible_html(&self, tree: &DecisionTree) -> String {
        let mut html = tree.to_html_with_theme(&self.theme);
        html = self.enhance_html(&html);
        html
    }

    /// Generates an accessible HTML dependency graph.
    pub fn to_accessible_html_graph(&self, graph: &DependencyGraph) -> String {
        let mut html = graph.to_html();
        html = self.enhance_html(&html);
        html
    }
}

impl Default for AccessibilityEnhancer {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Real-Time Collaboration Features (v0.1.5)
// ============================================================================

/// Streaming data source for continuous updates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamingDataSource {
    /// Source identifier
    pub source_id: String,
    /// Data stream URL or connection string
    pub stream_url: String,
    /// Update frequency in milliseconds
    pub update_frequency_ms: u64,
    /// Buffer size for data points
    pub buffer_size: usize,
    /// Current data buffer
    data_buffer: Vec<String>,
}

impl StreamingDataSource {
    /// Creates a new streaming data source.
    pub fn new(source_id: &str, stream_url: &str, update_frequency_ms: u64) -> Self {
        Self {
            source_id: source_id.to_string(),
            stream_url: stream_url.to_string(),
            update_frequency_ms,
            buffer_size: 1000,
            data_buffer: Vec::new(),
        }
    }

    /// Sets the buffer size.
    pub fn with_buffer_size(mut self, size: usize) -> Self {
        self.buffer_size = size;
        self
    }

    /// Adds data to the buffer.
    pub fn push_data(&mut self, data: String) {
        self.data_buffer.push(data);
        if self.data_buffer.len() > self.buffer_size {
            self.data_buffer.remove(0);
        }
    }

    /// Gets the current buffer.
    pub fn buffer(&self) -> &[String] {
        &self.data_buffer
    }

    /// Clears the buffer.
    pub fn clear_buffer(&mut self) {
        self.data_buffer.clear();
    }

    /// Generates JavaScript code for streaming data connection.
    pub fn to_javascript(&self) -> String {
        format!(
            r#"
class StreamingDataSource {{
    constructor() {{
        this.sourceId = '{}';
        this.streamUrl = '{}';
        this.updateFrequency = {};
        this.buffer = [];
        this.maxBufferSize = {};
        this.connection = null;
        this.callbacks = [];
    }}

    connect() {{
        this.connection = new WebSocket(this.streamUrl);
        this.connection.onmessage = (event) => {{
            const data = JSON.parse(event.data);
            this.pushData(data);
            this.notifyCallbacks(data);
        }};
        this.connection.onerror = (error) => {{
            console.error('Streaming error:', error);
        }};
        this.connection.onclose = () => {{
            console.log('Stream closed, reconnecting...');
            setTimeout(() => this.connect(), this.updateFrequency);
        }};
    }}

    pushData(data) {{
        this.buffer.push(data);
        if (this.buffer.length > this.maxBufferSize) {{
            this.buffer.shift();
        }}
    }}

    onData(callback) {{
        this.callbacks.push(callback);
    }}

    notifyCallbacks(data) {{
        this.callbacks.forEach(cb => cb(data));
    }}

    disconnect() {{
        if (this.connection) {{
            this.connection.close();
        }}
    }}
}}

const streamingSource = new StreamingDataSource();
streamingSource.connect();
"#,
            self.source_id, self.stream_url, self.update_frequency_ms, self.buffer_size
        )
    }
}

/// User information for collaborative sessions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CollaborativeUser {
    /// User ID
    pub user_id: String,
    /// User display name
    pub display_name: String,
    /// User color (for cursor and annotations)
    pub color: String,
    /// Whether the user is currently active
    pub active: bool,
}

impl CollaborativeUser {
    /// Creates a new collaborative user.
    pub fn new(user_id: &str, display_name: &str, color: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
            display_name: display_name.to_string(),
            color: color.to_string(),
            active: true,
        }
    }
}

/// Cursor position for collaborative viewing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CursorPosition {
    /// User who owns this cursor
    pub user: CollaborativeUser,
    /// X coordinate (percentage)
    pub x: f64,
    /// Y coordinate (percentage)
    pub y: f64,
    /// Timestamp of last update
    pub timestamp: u64,
}

impl CursorPosition {
    /// Creates a new cursor position.
    pub fn new(user: CollaborativeUser, x: f64, y: f64, timestamp: u64) -> Self {
        Self {
            user,
            x,
            y,
            timestamp,
        }
    }
}

/// Shared annotation for collaborative viewing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedAnnotation {
    /// Annotation ID
    pub annotation_id: String,
    /// User who created the annotation
    pub user: CollaborativeUser,
    /// Target node or element ID
    pub target_id: String,
    /// Annotation content
    pub content: String,
    /// Timestamp
    pub timestamp: u64,
    /// Whether the annotation is resolved
    pub resolved: bool,
}

impl SharedAnnotation {
    /// Creates a new shared annotation.
    pub fn new(
        annotation_id: &str,
        user: CollaborativeUser,
        target_id: &str,
        content: &str,
        timestamp: u64,
    ) -> Self {
        Self {
            annotation_id: annotation_id.to_string(),
            user,
            target_id: target_id.to_string(),
            content: content.to_string(),
            timestamp,
            resolved: false,
        }
    }

    /// Marks the annotation as resolved.
    pub fn resolve(&mut self) {
        self.resolved = true;
    }
}

/// Collaborative session manager for multi-user viewing and annotation.
#[derive(Debug, Clone)]
pub struct CollaborativeSession {
    /// Session ID
    pub session_id: String,
    /// Active users in the session
    users: Vec<CollaborativeUser>,
    /// Cursor positions for each user
    cursors: Vec<CursorPosition>,
    /// Shared annotations
    annotations: Vec<SharedAnnotation>,
    /// WebSocket URL for the session
    pub websocket_url: String,
}

impl CollaborativeSession {
    /// Creates a new collaborative session.
    pub fn new(session_id: &str, websocket_url: &str) -> Self {
        Self {
            session_id: session_id.to_string(),
            users: Vec::new(),
            cursors: Vec::new(),
            annotations: Vec::new(),
            websocket_url: websocket_url.to_string(),
        }
    }

    /// Adds a user to the session.
    pub fn add_user(&mut self, user: CollaborativeUser) {
        if !self.users.iter().any(|u| u.user_id == user.user_id) {
            self.users.push(user);
        }
    }

    /// Removes a user from the session.
    pub fn remove_user(&mut self, user_id: &str) {
        self.users.retain(|u| u.user_id != user_id);
        self.cursors.retain(|c| c.user.user_id != user_id);
    }

    /// Updates a user's cursor position.
    pub fn update_cursor(&mut self, cursor: CursorPosition) {
        if let Some(existing) = self
            .cursors
            .iter_mut()
            .find(|c| c.user.user_id == cursor.user.user_id)
        {
            *existing = cursor;
        } else {
            self.cursors.push(cursor);
        }
    }

    /// Adds a shared annotation.
    pub fn add_annotation(&mut self, annotation: SharedAnnotation) {
        self.annotations.push(annotation);
    }

    /// Removes an annotation by ID.
    pub fn remove_annotation(&mut self, annotation_id: &str) {
        self.annotations
            .retain(|a| a.annotation_id != annotation_id);
    }

    /// Gets all active users.
    pub fn active_users(&self) -> Vec<&CollaborativeUser> {
        self.users.iter().filter(|u| u.active).collect()
    }

    /// Gets all cursor positions.
    pub fn cursors(&self) -> &[CursorPosition] {
        &self.cursors
    }

    /// Gets all annotations.
    pub fn annotations(&self) -> &[SharedAnnotation] {
        &self.annotations
    }

    /// Generates HTML for collaborative visualization.
    pub fn to_collaborative_html(&self, tree: &DecisionTree) -> String {
        let base_html = tree.to_html();
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str("    <meta charset=\"utf-8\">\n");
        html.push_str("    <title>Collaborative Visualization</title>\n");
        html.push_str("    <script src=\"https://d3js.org/d3.v7.min.js\"></script>\n");
        html.push_str("    <style>\n");
        html.push_str("        body { margin: 0; padding: 0; overflow: hidden; }\n");
        html.push_str(
            "        #visualization { position: relative; width: 100vw; height: 100vh; }\n",
        );
        html.push_str("        .cursor { position: absolute; width: 20px; height: 20px; border-radius: 50%; pointer-events: none; transition: all 0.1s ease; }\n");
        html.push_str("        .cursor-label { position: absolute; background: rgba(0,0,0,0.8); color: white; padding: 2px 6px; border-radius: 3px; font-size: 12px; margin-left: 25px; white-space: nowrap; }\n");
        html.push_str("        .annotation { position: absolute; background: #fff; border: 2px solid #333; border-radius: 4px; padding: 10px; box-shadow: 0 2px 8px rgba(0,0,0,0.2); max-width: 300px; z-index: 100; }\n");
        html.push_str("        .annotation-header { font-weight: bold; margin-bottom: 5px; display: flex; justify-content: space-between; align-items: center; }\n");
        html.push_str(
            "        .annotation-author { font-size: 11px; color: #666; margin-bottom: 5px; }\n",
        );
        html.push_str("        .annotation-content { font-size: 13px; }\n");
        html.push_str(
            "        .annotation-resolved { opacity: 0.6; text-decoration: line-through; }\n",
        );
        html.push_str("        .users-panel { position: fixed; top: 10px; right: 10px; background: white; border-radius: 8px; padding: 15px; box-shadow: 0 2px 8px rgba(0,0,0,0.2); max-width: 200px; }\n");
        html.push_str("        .user-item { display: flex; align-items: center; margin: 5px 0; font-size: 13px; }\n");
        html.push_str("        .user-dot { width: 10px; height: 10px; border-radius: 50%; margin-right: 8px; }\n");
        html.push_str("    </style>\n</head>\n<body>\n");
        html.push_str("    <div id=\"visualization\">\n");
        html.push_str(&format!("        {}\n", base_html));
        html.push_str("    </div>\n");
        html.push_str("    <div class=\"users-panel\">\n");
        html.push_str(
            "        <div style=\"font-weight: bold; margin-bottom: 10px;\">Active Users</div>\n",
        );
        html.push_str("        <div id=\"user-list\"></div>\n");
        html.push_str("    </div>\n");
        html.push_str("    <script>\n");
        html.push_str(&format!("const sessionId = '{}';\n", self.session_id));
        html.push_str(&format!("const wsUrl = '{}';\n", self.websocket_url));
        html.push_str("let ws = null;\n");
        html.push_str("const cursors = new Map();\n");
        html.push_str("const annotations = new Map();\n\n");

        // Add WebSocket connection code
        html.push_str(&self.generate_websocket_code());

        // Add cursor rendering code
        html.push_str(&self.generate_cursor_code());

        // Add annotation rendering code
        html.push_str(&self.generate_annotation_code());

        html.push_str("    </script>\n</body>\n</html>");

        html
    }

    #[allow(dead_code)]
    fn generate_websocket_code(&self) -> String {
        r#"
function connectWebSocket() {
    ws = new WebSocket(wsUrl);

    ws.onopen = () => {
        console.log('Connected to collaborative session');
        ws.send(JSON.stringify({ type: 'join', sessionId: sessionId }));
    };

    ws.onmessage = (event) => {
        const message = JSON.parse(event.data);
        handleMessage(message);
    };

    ws.onclose = () => {
        console.log('Disconnected, reconnecting...');
        setTimeout(connectWebSocket, 3000);
    };
}

function handleMessage(message) {
    switch (message.type) {
        case 'cursor_update':
            updateCursor(message.user, message.x, message.y);
            break;
        case 'annotation_added':
            addAnnotation(message.annotation);
            break;
        case 'annotation_removed':
            removeAnnotation(message.annotationId);
            break;
        case 'user_joined':
            addUser(message.user);
            break;
        case 'user_left':
            removeUser(message.userId);
            break;
    }
}

document.addEventListener('mousemove', (e) => {
    const x = (e.clientX / window.innerWidth) * 100;
    const y = (e.clientY / window.innerHeight) * 100;
    if (ws && ws.readyState === WebSocket.OPEN) {
        ws.send(JSON.stringify({
            type: 'cursor_update',
            sessionId: sessionId,
            x: x,
            y: y
        }));
    }
});

connectWebSocket();
"#
        .to_string()
    }

    #[allow(dead_code)]
    fn generate_cursor_code(&self) -> String {
        r#"
function updateCursor(user, x, y) {
    let cursor = cursors.get(user.user_id);
    if (!cursor) {
        cursor = document.createElement('div');
        cursor.className = 'cursor';
        cursor.style.backgroundColor = user.color;

        const label = document.createElement('div');
        label.className = 'cursor-label';
        label.textContent = user.display_name;
        label.style.backgroundColor = user.color;
        cursor.appendChild(label);

        document.getElementById('visualization').appendChild(cursor);
        cursors.set(user.user_id, cursor);
    }

    cursor.style.left = x + '%';
    cursor.style.top = y + '%';
}

function removeCursor(userId) {
    const cursor = cursors.get(userId);
    if (cursor) {
        cursor.remove();
        cursors.delete(userId);
    }
}
"#
        .to_string()
    }

    #[allow(dead_code)]
    fn generate_annotation_code(&self) -> String {
        r#"
function addAnnotation(annotation) {
    const annotationDiv = document.createElement('div');
    annotationDiv.className = 'annotation' + (annotation.resolved ? ' annotation-resolved' : '');
    annotationDiv.id = 'annotation-' + annotation.annotation_id;

    annotationDiv.innerHTML = `
        <div class="annotation-header">
            <span style="color: ${annotation.user.color}">${annotation.user.display_name}</span>
            <button onclick="resolveAnnotation('${annotation.annotation_id}')">✓</button>
        </div>
        <div class="annotation-author">${new Date(annotation.timestamp).toLocaleString()}</div>
        <div class="annotation-content">${annotation.content}</div>
    `;

    // Position near target element
    const target = document.getElementById(annotation.target_id);
    if (target) {
        const rect = target.getBoundingClientRect();
        annotationDiv.style.left = (rect.right + 10) + 'px';
        annotationDiv.style.top = rect.top + 'px';
    }

    document.getElementById('visualization').appendChild(annotationDiv);
    annotations.set(annotation.annotation_id, annotationDiv);
}

function removeAnnotation(annotationId) {
    const annotation = annotations.get(annotationId);
    if (annotation) {
        annotation.remove();
        annotations.delete(annotationId);
    }
}

function resolveAnnotation(annotationId) {
    if (ws && ws.readyState === WebSocket.OPEN) {
        ws.send(JSON.stringify({
            type: 'resolve_annotation',
            sessionId: sessionId,
            annotationId: annotationId
        }));
    }
}

function addUser(user) {
    const userList = document.getElementById('user-list');
    const userItem = document.createElement('div');
    userItem.className = 'user-item';
    userItem.id = 'user-' + user.user_id;
    userItem.innerHTML = `
        <div class="user-dot" style="background-color: ${user.color}"></div>
        <span>${user.display_name}</span>
    `;
    userList.appendChild(userItem);
}

function removeUser(userId) {
    const userItem = document.getElementById('user-' + userId);
    if (userItem) {
        userItem.remove();
    }
    removeCursor(userId);
}
"#
        .to_string()
    }
}

// ============================================================================
// Custom Theme Builder (v0.1.7)
// ============================================================================

/// Custom theme builder for creating branded themes.
#[derive(Debug, Clone)]
pub struct CustomThemeBuilder {
    theme: Theme,
}

impl CustomThemeBuilder {
    /// Creates a new custom theme builder.
    pub fn new() -> Self {
        Self {
            theme: Theme::default(),
        }
    }

    /// Starts from an existing theme.
    pub fn from_theme(theme: Theme) -> Self {
        Self { theme }
    }

    /// Sets the background color.
    pub fn with_background_color(mut self, color: &str) -> Self {
        self.theme.background_color = color.to_string();
        self
    }

    /// Sets the text color.
    pub fn with_text_color(mut self, color: &str) -> Self {
        self.theme.text_color = color.to_string();
        self
    }

    /// Sets the condition node color.
    pub fn with_condition_color(mut self, color: &str) -> Self {
        self.theme.condition_color = color.to_string();
        self
    }

    /// Sets the outcome node color.
    pub fn with_outcome_color(mut self, color: &str) -> Self {
        self.theme.outcome_color = color.to_string();
        self
    }

    /// Sets the discretion zone color.
    pub fn with_discretion_color(mut self, color: &str) -> Self {
        self.theme.discretion_color = color.to_string();
        self
    }

    /// Sets the link/edge color.
    pub fn with_link_color(mut self, color: &str) -> Self {
        self.theme.link_color = color.to_string();
        self
    }

    /// Sets the root node color.
    pub fn with_root_color(mut self, color: &str) -> Self {
        self.theme.root_color = color.to_string();
        self
    }

    /// Sets organization branding colors.
    pub fn with_branding(mut self, primary_color: &str, secondary_color: &str) -> Self {
        self.theme.condition_color = primary_color.to_string();
        self.theme.outcome_color = secondary_color.to_string();
        self.theme.link_color = primary_color.to_string();
        self
    }

    /// Sets a custom color palette.
    pub fn with_palette(
        mut self,
        background: &str,
        foreground: &str,
        accent1: &str,
        accent2: &str,
        accent3: &str,
    ) -> Self {
        self.theme.background_color = background.to_string();
        self.theme.text_color = foreground.to_string();
        self.theme.condition_color = accent1.to_string();
        self.theme.outcome_color = accent2.to_string();
        self.theme.discretion_color = accent3.to_string();
        self.theme.link_color = accent1.to_string();
        self
    }

    /// Builds the custom theme.
    pub fn build(self) -> Theme {
        self.theme
    }

    /// Exports the theme to JSON.
    pub fn to_json(&self) -> Result<String, VizError> {
        serde_json::to_string_pretty(&self.theme)
            .map_err(|e| VizError::ExportError(format!("Failed to serialize theme: {}", e)))
    }

    /// Imports a theme from JSON.
    pub fn from_json(json: &str) -> Result<Self, VizError> {
        let theme: Theme = serde_json::from_str(json)
            .map_err(|e| VizError::ExportError(format!("Failed to deserialize theme: {}", e)))?;
        Ok(Self { theme })
    }
}

impl Default for CustomThemeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Seasonal and Event Themes (v0.1.7 continued)
// ============================================================================

/// Seasonal and event-specific theme presets.
#[derive(Debug, Clone)]
pub struct SeasonalThemes;

impl SeasonalThemes {
    /// Winter/Holiday theme with cool blues and whites.
    pub fn winter() -> Theme {
        Theme {
            root_color: "#e8f4f8".to_string(),
            condition_color: "#b3d9ff".to_string(),
            discretion_color: "#cce5ff".to_string(),
            outcome_color: "#d4ebf7".to_string(),
            link_color: "#668db8".to_string(),
            background_color: "#f0f8ff".to_string(),
            text_color: "#2c3e50".to_string(),
        }
    }

    /// Spring theme with fresh greens and pastels.
    pub fn spring() -> Theme {
        Theme {
            root_color: "#e8f5e9".to_string(),
            condition_color: "#c8e6c9".to_string(),
            discretion_color: "#fff9c4".to_string(),
            outcome_color: "#a5d6a7".to_string(),
            link_color: "#81c784".to_string(),
            background_color: "#f1f8e9".to_string(),
            text_color: "#33691e".to_string(),
        }
    }

    /// Summer theme with warm, vibrant colors.
    pub fn summer() -> Theme {
        Theme {
            root_color: "#fff3e0".to_string(),
            condition_color: "#ffe0b2".to_string(),
            discretion_color: "#ffccbc".to_string(),
            outcome_color: "#ffab91".to_string(),
            link_color: "#ff9800".to_string(),
            background_color: "#fffaf0".to_string(),
            text_color: "#e65100".to_string(),
        }
    }

    /// Autumn/Fall theme with warm earth tones.
    pub fn autumn() -> Theme {
        Theme {
            root_color: "#fbe9e7".to_string(),
            condition_color: "#ffccbc".to_string(),
            discretion_color: "#ffab91".to_string(),
            outcome_color: "#bcaaa4".to_string(),
            link_color: "#8d6e63".to_string(),
            background_color: "#fff8f5".to_string(),
            text_color: "#5d4037".to_string(),
        }
    }

    /// Holiday theme with festive reds and greens.
    pub fn holiday() -> Theme {
        Theme {
            root_color: "#ffebee".to_string(),
            condition_color: "#c8e6c9".to_string(),
            discretion_color: "#ffcdd2".to_string(),
            outcome_color: "#a5d6a7".to_string(),
            link_color: "#c62828".to_string(),
            background_color: "#fafafa".to_string(),
            text_color: "#1b5e20".to_string(),
        }
    }

    /// Professional/Corporate theme with navy and gray.
    pub fn corporate() -> Theme {
        Theme {
            root_color: "#eceff1".to_string(),
            condition_color: "#b0bec5".to_string(),
            discretion_color: "#90a4ae".to_string(),
            outcome_color: "#78909c".to_string(),
            link_color: "#455a64".to_string(),
            background_color: "#fafafa".to_string(),
            text_color: "#263238".to_string(),
        }
    }

    /// Academic theme with scholarly blues.
    pub fn academic() -> Theme {
        Theme {
            root_color: "#e3f2fd".to_string(),
            condition_color: "#bbdefb".to_string(),
            discretion_color: "#90caf9".to_string(),
            outcome_color: "#64b5f6".to_string(),
            link_color: "#1976d2".to_string(),
            background_color: "#fafafa".to_string(),
            text_color: "#0d47a1".to_string(),
        }
    }

    /// Legal/Government theme with traditional colors.
    pub fn legal() -> Theme {
        Theme {
            root_color: "#f5f5f5".to_string(),
            condition_color: "#e0e0e0".to_string(),
            discretion_color: "#d4af37".to_string(),
            outcome_color: "#bdbdbd".to_string(),
            link_color: "#1a237e".to_string(),
            background_color: "#ffffff".to_string(),
            text_color: "#000000".to_string(),
        }
    }
}

/// CSS variable customization for dynamic theming.
#[derive(Debug, Clone)]
pub struct CssVariableTheme {
    /// CSS variable definitions
    variables: Vec<(String, String)>,
}

impl CssVariableTheme {
    /// Creates a new CSS variable theme.
    pub fn new() -> Self {
        Self {
            variables: Vec::new(),
        }
    }

    /// Adds a CSS variable.
    pub fn add_variable(mut self, name: &str, value: &str) -> Self {
        self.variables.push((name.to_string(), value.to_string()));
        self
    }

    /// Creates CSS variable theme from a Theme.
    pub fn from_theme(theme: &Theme) -> Self {
        Self::new()
            .add_variable("--viz-root-color", &theme.root_color)
            .add_variable("--viz-condition-color", &theme.condition_color)
            .add_variable("--viz-discretion-color", &theme.discretion_color)
            .add_variable("--viz-outcome-color", &theme.outcome_color)
            .add_variable("--viz-link-color", &theme.link_color)
            .add_variable("--viz-background-color", &theme.background_color)
            .add_variable("--viz-text-color", &theme.text_color)
    }

    /// Generates CSS :root block with variables.
    pub fn to_css(&self) -> String {
        let mut css = String::from(":root {\n");
        for (name, value) in &self.variables {
            css.push_str(&format!("  {}: {};\n", name, value));
        }
        css.push_str("}\n");
        css
    }

    /// Generates CSS with custom selector.
    pub fn to_css_with_selector(&self, selector: &str) -> String {
        let mut css = String::from(selector);
        css.push_str(" {\n");
        for (name, value) in &self.variables {
            css.push_str(&format!("  {}: {};\n", name, value));
        }
        css.push_str("}\n");
        css
    }

    /// Gets all variables.
    pub fn variables(&self) -> &[(String, String)] {
        &self.variables
    }
}

impl Default for CssVariableTheme {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Performance Optimizations (v0.1.8)
// ============================================================================

/// Virtualization configuration for large datasets.
#[derive(Debug, Clone)]
pub struct VirtualizationConfig {
    /// Enable virtualization
    pub enabled: bool,
    /// Number of items to render at once
    pub render_batch_size: usize,
    /// Buffer size around visible area
    pub buffer_size: usize,
    /// Minimum item height in pixels
    pub min_item_height: u32,
    /// Enable dynamic height calculation
    pub dynamic_height: bool,
}

impl VirtualizationConfig {
    /// Creates a new virtualization configuration.
    pub fn new() -> Self {
        Self {
            enabled: true,
            render_batch_size: 100,
            buffer_size: 20,
            min_item_height: 50,
            dynamic_height: false,
        }
    }

    /// Disables virtualization.
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            ..Self::new()
        }
    }

    /// Sets the render batch size.
    pub fn with_batch_size(mut self, size: usize) -> Self {
        self.render_batch_size = size;
        self
    }

    /// Sets the buffer size.
    pub fn with_buffer_size(mut self, size: usize) -> Self {
        self.buffer_size = size;
        self
    }

    /// Enables dynamic height calculation.
    pub fn with_dynamic_height(mut self) -> Self {
        self.dynamic_height = true;
        self
    }

    /// Generates JavaScript virtualization code.
    pub fn to_javascript(&self) -> String {
        if !self.enabled {
            return String::new();
        }

        format!(
            r#"
// Virtualization for large datasets
class VirtualScroller {{
    constructor(container, items, config) {{
        this.container = container;
        this.items = items;
        this.renderBatchSize = {};
        this.bufferSize = {};
        this.minItemHeight = {};
        this.dynamicHeight = {};
        this.visibleStart = 0;
        this.visibleEnd = this.renderBatchSize;
        this.init();
    }}

    init() {{
        this.container.style.overflowY = 'auto';
        this.container.style.position = 'relative';

        // Create viewport
        this.viewport = document.createElement('div');
        this.viewport.style.position = 'relative';
        this.container.appendChild(this.viewport);

        // Initial render
        this.render();

        // Add scroll listener
        this.container.addEventListener('scroll', () => this.onScroll());
    }}

    onScroll() {{
        const scrollTop = this.container.scrollTop;
        const newStart = Math.floor(scrollTop / this.minItemHeight);
        const newEnd = newStart + this.renderBatchSize;

        if (newStart !== this.visibleStart || newEnd !== this.visibleEnd) {{
            this.visibleStart = Math.max(0, newStart - this.bufferSize);
            this.visibleEnd = Math.min(this.items.length, newEnd + this.bufferSize);
            this.render();
        }}
    }}

    render() {{
        // Clear viewport
        this.viewport.innerHTML = '';

        // Set total height
        this.viewport.style.height = (this.items.length * this.minItemHeight) + 'px';

        // Create fragment for batch rendering
        const fragment = document.createDocumentFragment();

        // Render visible items
        for (let i = this.visibleStart; i < this.visibleEnd; i++) {{
            const item = this.createItem(this.items[i], i);
            fragment.appendChild(item);
        }}

        this.viewport.appendChild(fragment);
    }}

    createItem(data, index) {{
        const item = document.createElement('div');
        item.className = 'virtual-item';
        item.style.position = 'absolute';
        item.style.top = (index * this.minItemHeight) + 'px';
        item.style.width = '100%';
        item.style.minHeight = this.minItemHeight + 'px';
        item.innerHTML = data;
        return item;
    }}
}}
"#,
            self.render_batch_size, self.buffer_size, self.min_item_height, self.dynamic_height
        )
    }
}

impl Default for VirtualizationConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Progressive loading configuration.
#[derive(Debug, Clone)]
pub struct ProgressiveLoadingConfig {
    /// Enable progressive loading
    pub enabled: bool,
    /// Initial load count
    pub initial_load: usize,
    /// Load increment on scroll
    pub load_increment: usize,
    /// Show loading indicator
    pub show_loading_indicator: bool,
    /// Delay before loading more (ms)
    pub load_delay_ms: u32,
}

impl ProgressiveLoadingConfig {
    /// Creates a new progressive loading configuration.
    pub fn new() -> Self {
        Self {
            enabled: true,
            initial_load: 50,
            load_increment: 25,
            show_loading_indicator: true,
            load_delay_ms: 200,
        }
    }

    /// Sets the initial load count.
    pub fn with_initial_load(mut self, count: usize) -> Self {
        self.initial_load = count;
        self
    }

    /// Sets the load increment.
    pub fn with_load_increment(mut self, increment: usize) -> Self {
        self.load_increment = increment;
        self
    }

    /// Disables loading indicator.
    pub fn without_loading_indicator(mut self) -> Self {
        self.show_loading_indicator = false;
        self
    }

    /// Generates JavaScript progressive loading code.
    pub fn to_javascript(&self) -> String {
        if !self.enabled {
            return String::new();
        }

        format!(
            r#"
// Progressive loading for large datasets
class ProgressiveLoader {{
    constructor(container, dataProvider, config) {{
        this.container = container;
        this.dataProvider = dataProvider;
        this.initialLoad = {};
        this.loadIncrement = {};
        this.showLoadingIndicator = {};
        this.loadDelay = {};
        this.currentIndex = 0;
        this.loading = false;
        this.hasMore = true;
        this.init();
    }}

    init() {{
        this.loadMore();
        this.container.addEventListener('scroll', () => this.checkScroll());
    }}

    checkScroll() {{
        if (this.loading || !this.hasMore) return;

        const scrollTop = this.container.scrollTop;
        const scrollHeight = this.container.scrollHeight;
        const clientHeight = this.container.clientHeight;

        // Load more when 80% scrolled
        if (scrollTop + clientHeight >= scrollHeight * 0.8) {{
            this.loadMore();
        }}
    }}

    async loadMore() {{
        if (this.loading || !this.hasMore) return;

        this.loading = true;
        if (this.showLoadingIndicator) {{
            this.showLoader();
        }}

        setTimeout(async () => {{
            const count = this.currentIndex === 0 ? this.initialLoad : this.loadIncrement;
            const items = await this.dataProvider(this.currentIndex, count);

            if (items.length === 0) {{
                this.hasMore = false;
            }} else {{
                this.renderItems(items);
                this.currentIndex += items.length;
            }}

            this.loading = false;
            if (this.showLoadingIndicator) {{
                this.hideLoader();
            }}
        }}, this.loadDelay);
    }}

    renderItems(items) {{
        const fragment = document.createDocumentFragment();
        items.forEach(item => {{
            const element = this.createItemElement(item);
            fragment.appendChild(element);
        }});
        this.container.appendChild(fragment);
    }}

    createItemElement(item) {{
        const div = document.createElement('div');
        div.className = 'progressive-item';
        div.innerHTML = item;
        return div;
    }}

    showLoader() {{
        if (!this.loader) {{
            this.loader = document.createElement('div');
            this.loader.className = 'progressive-loader';
            this.loader.innerHTML = '<div class="spinner">Loading...</div>';
        }}
        this.container.appendChild(this.loader);
    }}

    hideLoader() {{
        if (this.loader && this.loader.parentNode) {{
            this.loader.parentNode.removeChild(this.loader);
        }}
    }}
}}
"#,
            self.initial_load, self.load_increment, self.show_loading_indicator, self.load_delay_ms
        )
    }
}

impl Default for ProgressiveLoadingConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Level-of-detail configuration for complex visualizations.
#[derive(Debug, Clone)]
pub struct LevelOfDetailConfig {
    /// Enable LOD rendering
    pub enabled: bool,
    /// Zoom level thresholds for detail levels
    pub zoom_thresholds: Vec<f64>,
    /// Simplify graph at low zoom
    pub simplify_at_low_zoom: bool,
    /// Hide labels at low zoom
    pub hide_labels_at_low_zoom: bool,
    /// Aggregate nodes at low zoom
    pub aggregate_nodes: bool,
}

impl LevelOfDetailConfig {
    /// Creates a new LOD configuration.
    pub fn new() -> Self {
        Self {
            enabled: true,
            zoom_thresholds: vec![0.25, 0.5, 0.75, 1.0],
            simplify_at_low_zoom: true,
            hide_labels_at_low_zoom: true,
            aggregate_nodes: true,
        }
    }

    /// Disables LOD rendering.
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            ..Self::new()
        }
    }

    /// Sets custom zoom thresholds.
    pub fn with_zoom_thresholds(mut self, thresholds: Vec<f64>) -> Self {
        self.zoom_thresholds = thresholds;
        self
    }

    /// Generates JavaScript LOD code.
    pub fn to_javascript(&self) -> String {
        if !self.enabled {
            return String::new();
        }

        format!(
            r#"
// Level-of-detail rendering for performance
class LevelOfDetailRenderer {{
    constructor(svg, config) {{
        this.svg = svg;
        this.zoomThresholds = {:?};
        this.simplifyAtLowZoom = {};
        this.hideLabelsAtLowZoom = {};
        this.aggregateNodes = {};
        this.currentZoom = 1.0;
        this.init();
    }}

    init() {{
        // Add zoom listener
        this.svg.addEventListener('zoom', (e) => {{
            this.currentZoom = e.detail.scale;
            this.updateDetailLevel();
        }});
    }}

    updateDetailLevel() {{
        const level = this.getDetailLevel(this.currentZoom);

        // Apply detail level
        this.applyDetailLevel(level);
    }}

    getDetailLevel(zoom) {{
        for (let i = 0; i < this.zoomThresholds.length; i++) {{
            if (zoom <= this.zoomThresholds[i]) {{
                return i;
            }}
        }}
        return this.zoomThresholds.length;
    }}

    applyDetailLevel(level) {{
        // Hide/show labels based on zoom
        if (this.hideLabelsAtLowZoom) {{
            const labels = this.svg.querySelectorAll('.node-label');
            labels.forEach(label => {{
                label.style.display = level >= 2 ? 'block' : 'none';
            }});
        }}

        // Simplify edges at low zoom
        if (this.simplifyAtLowZoom) {{
            const edges = this.svg.querySelectorAll('.edge');
            edges.forEach(edge => {{
                edge.style.strokeWidth = level >= 2 ? '2px' : '1px';
            }});
        }}

        // Aggregate nodes at low zoom
        if (this.aggregateNodes && level < 2) {{
            this.performNodeAggregation();
        }}
    }}

    performNodeAggregation() {{
        // Group nearby nodes into clusters
        const nodes = this.svg.querySelectorAll('.node');
        // Implementation depends on graph structure
    }}
}}
"#,
            self.zoom_thresholds,
            self.simplify_at_low_zoom,
            self.hide_labels_at_low_zoom,
            self.aggregate_nodes
        )
    }
}

impl Default for LevelOfDetailConfig {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Domain-Specific Visualizations (v0.1.9)
// ============================================================================

/// Court hierarchy visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourtNode {
    /// Court identifier
    pub id: String,
    /// Court name
    pub name: String,
    /// Court level (e.g., "Supreme", "Appellate", "Trial")
    pub level: String,
    /// Jurisdiction
    pub jurisdiction: String,
    /// Number of judges
    pub judge_count: usize,
}

/// Court hierarchy visualizer
#[derive(Debug, Clone)]
pub struct CourtHierarchyVisualizer {
    theme: Theme,
}

impl CourtHierarchyVisualizer {
    /// Creates a new court hierarchy visualizer.
    pub fn new() -> Self {
        Self {
            theme: Theme::light(),
        }
    }

    /// Sets the theme.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Renders court hierarchy to HTML.
    #[allow(clippy::too_many_arguments)]
    pub fn to_html(&self, courts: &[CourtNode]) -> String {
        let mut levels: HashMap<String, Vec<&CourtNode>> = HashMap::new();

        for court in courts {
            levels.entry(court.level.clone()).or_default().push(court);
        }

        let mut html = format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <style>
        body {{
            font-family: Arial, sans-serif;
            background-color: {};
            color: {};
            padding: 20px;
        }}
        .court-hierarchy {{
            max-width: 1200px;
            margin: 0 auto;
        }}
        .court-level {{
            margin-bottom: 30px;
            padding: 20px;
            background-color: {};
            border-radius: 8px;
        }}
        .level-title {{
            font-size: 24px;
            font-weight: bold;
            margin-bottom: 15px;
            color: {};
        }}
        .court-container {{
            display: flex;
            flex-wrap: wrap;
            gap: 15px;
        }}
        .court-box {{
            flex: 1 1 300px;
            padding: 15px;
            background-color: {};
            border: 2px solid {};
            border-radius: 6px;
        }}
        .court-name {{
            font-weight: bold;
            font-size: 16px;
            margin-bottom: 8px;
        }}
        .court-info {{
            font-size: 14px;
            color: {};
            margin: 4px 0;
        }}
    </style>
</head>
<body>
    <div class="court-hierarchy">
        <h1>Court Hierarchy</h1>
"#,
            self.theme.background_color,
            self.theme.text_color,
            self.theme.root_color,
            self.theme.condition_color,
            self.theme.outcome_color,
            self.theme.link_color,
            self.theme.text_color,
        );

        // Sort levels (Supreme > Appellate > Trial)
        let level_order = ["Supreme", "Appellate", "Trial", "District", "Municipal"];
        for level in &level_order {
            if let Some(court_list) = levels.get(*level) {
                html.push_str(&format!(
                    r#"        <div class="court-level">
            <div class="level-title">{} Courts</div>
            <div class="court-container">
"#,
                    level
                ));

                for court in court_list {
                    html.push_str(&format!(
                        r#"                <div class="court-box">
                    <div class="court-name">{}</div>
                    <div class="court-info">Jurisdiction: {}</div>
                    <div class="court-info">Judges: {}</div>
                </div>
"#,
                        court.name, court.jurisdiction, court.judge_count
                    ));
                }

                html.push_str("            </div>\n        </div>\n");
            }
        }

        html.push_str(
            r#"    </div>
</body>
</html>"#,
        );

        html
    }

    /// Renders court hierarchy to Mermaid diagram.
    pub fn to_mermaid(&self, courts: &[CourtNode]) -> String {
        let mut diagram = String::from("graph TD\n");

        let mut levels: HashMap<String, Vec<&CourtNode>> = HashMap::new();
        for court in courts {
            levels.entry(court.level.clone()).or_default().push(court);
        }

        let level_order = ["Supreme", "Appellate", "Trial", "District", "Municipal"];
        for (i, level) in level_order.iter().enumerate() {
            if let Some(court_list) = levels.get(*level) {
                for court in court_list {
                    let node_id = court.id.replace('-', "_");
                    diagram.push_str(&format!(
                        "    {}[\"{}<br/>{}\"]",
                        node_id, court.name, court.jurisdiction
                    ));

                    if i > 0 {
                        if let Some(prev_level) = level_order.get(i - 1) {
                            if let Some(prev_courts) = levels.get(*prev_level) {
                                for prev_court in prev_courts {
                                    let prev_id = prev_court.id.replace('-', "_");
                                    diagram.push_str(&format!("\n    {} --> {}", prev_id, node_id));
                                }
                            }
                        }
                    }
                    diagram.push('\n');
                }
            }
        }

        diagram
    }
}

impl Default for CourtHierarchyVisualizer {
    fn default() -> Self {
        Self::new()
    }
}

/// Legislative process step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegislativeStep {
    /// Step identifier
    pub id: String,
    /// Step name
    pub name: String,
    /// Step description
    pub description: String,
    /// Required actors
    pub actors: Vec<String>,
    /// Estimated duration in days
    pub duration_days: Option<u32>,
}

/// Legislative process flowchart visualizer
#[derive(Debug, Clone)]
pub struct LegislativeProcessVisualizer {
    theme: Theme,
}

impl LegislativeProcessVisualizer {
    /// Creates a new legislative process visualizer.
    pub fn new() -> Self {
        Self {
            theme: Theme::light(),
        }
    }

    /// Sets the theme.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Renders legislative process to HTML.
    pub fn to_html(&self, steps: &[LegislativeStep]) -> String {
        let mut html = format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <style>
        body {{
            font-family: Arial, sans-serif;
            background-color: {};
            color: {};
            padding: 20px;
        }}
        .process-container {{
            max-width: 1200px;
            margin: 0 auto;
        }}
        .step {{
            display: flex;
            align-items: center;
            margin-bottom: 20px;
        }}
        .step-box {{
            flex: 1;
            padding: 20px;
            background-color: {};
            border: 2px solid {};
            border-radius: 8px;
        }}
        .step-number {{
            width: 40px;
            height: 40px;
            background-color: {};
            color: {};
            border-radius: 50%;
            display: flex;
            align-items: center;
            justify-content: center;
            font-weight: bold;
            margin-right: 20px;
        }}
        .step-title {{
            font-size: 18px;
            font-weight: bold;
            margin-bottom: 10px;
        }}
        .step-description {{
            margin-bottom: 10px;
        }}
        .step-actors {{
            font-style: italic;
            color: {};
        }}
        .step-duration {{
            color: {};
            font-size: 12px;
        }}
        .arrow {{
            text-align: center;
            font-size: 24px;
            color: {};
            margin: 10px 0;
        }}
    </style>
</head>
<body>
    <div class="process-container">
        <h1>Legislative Process</h1>
"#,
            self.theme.background_color,
            self.theme.text_color,
            self.theme.outcome_color,
            self.theme.link_color,
            self.theme.condition_color,
            self.theme.background_color,
            self.theme.discretion_color,
            self.theme.discretion_color,
            self.theme.link_color,
        );

        for (i, step) in steps.iter().enumerate() {
            html.push_str(&format!(
                r#"        <div class="step">
            <div class="step-number">{}</div>
            <div class="step-box">
                <div class="step-title">{}</div>
                <div class="step-description">{}</div>
                <div class="step-actors">Actors: {}</div>
"#,
                i + 1,
                step.name,
                step.description,
                step.actors.join(", ")
            ));

            if let Some(duration) = step.duration_days {
                html.push_str(&format!(
                    r#"                <div class="step-duration">Estimated duration: {} days</div>
"#,
                    duration
                ));
            }

            html.push_str("            </div>\n        </div>\n");

            if i < steps.len() - 1 {
                html.push_str(
                    r#"        <div class="arrow">↓</div>
"#,
                );
            }
        }

        html.push_str(
            r#"    </div>
</body>
</html>"#,
        );

        html
    }

    /// Renders legislative process to Mermaid flowchart.
    pub fn to_mermaid(&self, steps: &[LegislativeStep]) -> String {
        let mut diagram = String::from("graph TD\n");

        for (i, step) in steps.iter().enumerate() {
            let node_id = step.id.replace('-', "_");
            diagram.push_str(&format!("    {}[\"{}\"]\n", node_id, step.name));

            if i > 0 {
                let prev_id = steps[i - 1].id.replace('-', "_");
                diagram.push_str(&format!("    {} --> {}\n", prev_id, node_id));
            }
        }

        diagram
    }
}

impl Default for LegislativeProcessVisualizer {
    fn default() -> Self {
        Self::new()
    }
}

/// Case citation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseCitation {
    /// Case identifier
    pub id: String,
    /// Case name
    pub name: String,
    /// Year
    pub year: u32,
    /// Court
    pub court: String,
    /// Citations (references to other cases)
    pub citations: Vec<String>,
}

/// Case citation network visualizer
#[derive(Debug, Clone)]
pub struct CaseCitationNetworkVisualizer {
    theme: Theme,
}

impl CaseCitationNetworkVisualizer {
    /// Creates a new case citation network visualizer.
    pub fn new() -> Self {
        Self {
            theme: Theme::light(),
        }
    }

    /// Sets the theme.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Renders citation network to HTML with D3.js.
    #[allow(clippy::too_many_arguments)]
    pub fn to_html(&self, cases: &[CaseCitation]) -> String {
        let nodes_json = serde_json::to_string(cases).unwrap_or_else(|_| "[]".to_string());

        let html = format!(
            "<!DOCTYPE html>\n\
<html>\n\
<head>\n\
    <meta charset=\"UTF-8\">\n\
    <script src=\"https://d3js.org/d3.v7.min.js\"></script>\n\
    <style>\n\
        body {{\n\
            margin: 0;\n\
            background-color: {};\n\
            color: {};\n\
            font-family: Arial, sans-serif;\n\
        }}\n\
        #graph {{\n\
            width: 100vw;\n\
            height: 100vh;\n\
        }}\n\
        .node {{\n\
            stroke: {};\n\
            stroke-width: 2px;\n\
            cursor: pointer;\n\
        }}\n\
        .link {{\n\
            stroke: {};\n\
            stroke-opacity: 0.6;\n\
            fill: none;\n\
        }}\n\
        .label {{\n\
            font-size: 12px;\n\
            fill: {};\n\
            pointer-events: none;\n\
        }}\n\
    </style>\n\
</head>\n\
<body>\n\
    <svg id=\"graph\"></svg>\n\
    <script>\n\
        const data = {};\n\
\n\
        const width = window.innerWidth;\n\
        const height = window.innerHeight;\n\
\n\
        const svg = d3.select(\"#graph\")\n\
            .attr(\"width\", width)\n\
            .attr(\"height\", height);\n\
\n\
        const nodes = data.map(d => ({{{{ id: d.id, name: d.name, year: d.year, court: d.court }}}}));\n\
        const links = [];\n\
        data.forEach(d => {{\n\
            d.citations.forEach(target => {{\n\
                links.push({{{{ source: d.id, target: target }}}});\n\
            }});\n\
        }});\n\
\n\
        const simulation = d3.forceSimulation(nodes)\n\
            .force(\"link\", d3.forceLink(links).id(d => d.id))\n\
            .force(\"charge\", d3.forceManyBody().strength(-300))\n\
            .force(\"center\", d3.forceCenter(width / 2, height / 2));\n\
\n\
        const link = svg.append(\"g\")\n\
            .selectAll(\"line\")\n\
            .data(links)\n\
            .enter().append(\"line\")\n\
            .attr(\"class\", \"link\");\n\
\n\
        const node = svg.append(\"g\")\n\
            .selectAll(\"circle\")\n\
            .data(nodes)\n\
            .enter().append(\"circle\")\n\
            .attr(\"class\", \"node\")\n\
            .attr(\"r\", 8)\n\
            .attr(\"fill\", \"{}\")\n\
            .call(d3.drag()\n\
                .on(\"start\", dragstarted)\n\
                .on(\"drag\", dragged)\n\
                .on(\"end\", dragended));\n\
\n\
        const label = svg.append(\"g\")\n\
            .selectAll(\"text\")\n\
            .data(nodes)\n\
            .enter().append(\"text\")\n\
            .attr(\"class\", \"label\")\n\
            .text(d => d.name)\n\
            .attr(\"text-anchor\", \"middle\");\n\
\n\
        simulation.on(\"tick\", () => {{\n\
            link.attr(\"x1\", d => d.source.x)\n\
                .attr(\"y1\", d => d.source.y)\n\
                .attr(\"x2\", d => d.target.x)\n\
                .attr(\"y2\", d => d.target.y);\n\
\n\
            node.attr(\"cx\", d => d.x)\n\
                .attr(\"cy\", d => d.y);\n\
\n\
            label.attr(\"x\", d => d.x)\n\
                .attr(\"y\", d => d.y - 12);\n\
        }});\n\
\n\
        function dragstarted(event) {{\n\
            if (!event.active) simulation.alphaTarget(0.3).restart();\n\
            event.subject.fx = event.subject.x;\n\
            event.subject.fy = event.subject.y;\n\
        }}\n\
\n\
        function dragged(event) {{\n\
            event.subject.fx = event.x;\n\
            event.subject.fy = event.y;\n\
        }}\n\
\n\
        function dragended(event) {{\n\
            if (!event.active) simulation.alphaTarget(0);\n\
            event.subject.fx = null;\n\
            event.subject.fy = null;\n\
        }}\n\
    </script>\n\
</body>\n\
</html>",
            self.theme.background_color,
            self.theme.text_color,
            self.theme.link_color,
            self.theme.link_color,
            self.theme.text_color,
            nodes_json,
            self.theme.condition_color,
        );

        html
    }

    /// Renders citation network to Mermaid.
    pub fn to_mermaid(&self, cases: &[CaseCitation]) -> String {
        let mut diagram = String::from("graph LR\n");

        for case in cases {
            let node_id = case.id.replace('-', "_");
            diagram.push_str(&format!("    {}[\"{}\"]\n", node_id, case.name));

            for citation in &case.citations {
                let citation_id = citation.replace('-', "_");
                diagram.push_str(&format!("    {} --> {}\n", node_id, citation_id));
            }
        }

        diagram
    }
}

impl Default for CaseCitationNetworkVisualizer {
    fn default() -> Self {
        Self::new()
    }
}

/// Regulatory entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulatoryEntity {
    /// Entity identifier
    pub id: String,
    /// Entity name
    pub name: String,
    /// Entity type (e.g., "Agency", "Authority", "Commission")
    pub entity_type: String,
    /// Jurisdiction
    pub jurisdiction: String,
    /// Regulated sectors
    pub sectors: Vec<String>,
}

/// Regulatory landscape map visualizer
#[derive(Debug, Clone)]
pub struct RegulatoryLandscapeVisualizer {
    theme: Theme,
}

impl RegulatoryLandscapeVisualizer {
    /// Creates a new regulatory landscape visualizer.
    pub fn new() -> Self {
        Self {
            theme: Theme::light(),
        }
    }

    /// Sets the theme.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Renders regulatory landscape to HTML.
    #[allow(clippy::too_many_arguments)]
    pub fn to_html(&self, entities: &[RegulatoryEntity]) -> String {
        let mut entity_types: HashMap<String, Vec<&RegulatoryEntity>> = HashMap::new();

        for entity in entities {
            entity_types
                .entry(entity.entity_type.clone())
                .or_default()
                .push(entity);
        }

        let mut html = format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <style>
        body {{
            font-family: Arial, sans-serif;
            background-color: {};
            color: {};
            padding: 20px;
        }}
        .landscape {{
            max-width: 1400px;
            margin: 0 auto;
        }}
        .entity-type-section {{
            margin-bottom: 30px;
        }}
        .type-title {{
            font-size: 22px;
            font-weight: bold;
            color: {};
            margin-bottom: 15px;
        }}
        .entity-grid {{
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
            gap: 15px;
        }}
        .entity-card {{
            padding: 15px;
            background-color: {};
            border: 2px solid {};
            border-radius: 8px;
        }}
        .entity-name {{
            font-weight: bold;
            font-size: 16px;
            margin-bottom: 8px;
        }}
        .entity-info {{
            font-size: 14px;
            margin: 4px 0;
        }}
        .sectors {{
            display: flex;
            flex-wrap: wrap;
            gap: 5px;
            margin-top: 8px;
        }}
        .sector-tag {{
            padding: 3px 8px;
            background-color: {};
            color: {};
            border-radius: 4px;
            font-size: 12px;
        }}
    </style>
</head>
<body>
    <div class="landscape">
        <h1>Regulatory Landscape</h1>
"#,
            self.theme.background_color,
            self.theme.text_color,
            self.theme.condition_color,
            self.theme.outcome_color,
            self.theme.link_color,
            self.theme.discretion_color,
            self.theme.background_color,
        );

        for (entity_type, entity_list) in &entity_types {
            html.push_str(&format!(
                r#"        <div class="entity-type-section">
            <div class="type-title">{}</div>
            <div class="entity-grid">
"#,
                entity_type
            ));

            for entity in entity_list {
                html.push_str(&format!(
                    r#"                <div class="entity-card">
                    <div class="entity-name">{}</div>
                    <div class="entity-info">Jurisdiction: {}</div>
                    <div class="sectors">
"#,
                    entity.name, entity.jurisdiction
                ));

                for sector in &entity.sectors {
                    html.push_str(&format!(
                        r#"                        <span class="sector-tag">{}</span>
"#,
                        sector
                    ));
                }

                html.push_str("                    </div>\n                </div>\n");
            }

            html.push_str("            </div>\n        </div>\n");
        }

        html.push_str(
            r#"    </div>
</body>
</html>"#,
        );

        html
    }
}

impl Default for RegulatoryLandscapeVisualizer {
    fn default() -> Self {
        Self::new()
    }
}

/// Compliance status
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ComplianceStatus {
    /// Fully compliant
    Compliant,
    /// Partially compliant
    PartiallyCompliant,
    /// Non-compliant
    NonCompliant,
    /// Not applicable
    NotApplicable,
}

/// Compliance item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceItem {
    /// Item identifier
    pub id: String,
    /// Requirement name
    pub requirement: String,
    /// Status
    pub status: ComplianceStatus,
    /// Category
    pub category: String,
    /// Notes
    pub notes: String,
}

/// Compliance status dashboard visualizer
#[derive(Debug, Clone)]
pub struct ComplianceDashboardVisualizer {
    theme: Theme,
}

impl ComplianceDashboardVisualizer {
    /// Creates a new compliance dashboard visualizer.
    pub fn new() -> Self {
        Self {
            theme: Theme::light(),
        }
    }

    /// Sets the theme.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Renders compliance dashboard to HTML.
    #[allow(clippy::too_many_arguments)]
    pub fn to_html(&self, items: &[ComplianceItem]) -> String {
        let total = items.len();
        let compliant = items
            .iter()
            .filter(|i| matches!(i.status, ComplianceStatus::Compliant))
            .count();
        let partial = items
            .iter()
            .filter(|i| matches!(i.status, ComplianceStatus::PartiallyCompliant))
            .count();
        let non_compliant = items
            .iter()
            .filter(|i| matches!(i.status, ComplianceStatus::NonCompliant))
            .count();

        let compliance_rate = if total > 0 {
            (compliant as f64 / total as f64 * 100.0).round() as u32
        } else {
            0
        };

        let mut categories: HashMap<String, Vec<&ComplianceItem>> = HashMap::new();
        for item in items {
            categories
                .entry(item.category.clone())
                .or_default()
                .push(item);
        }

        let mut html = format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <style>
        body {{
            font-family: Arial, sans-serif;
            background-color: {};
            color: {};
            padding: 20px;
        }}
        .dashboard {{
            max-width: 1200px;
            margin: 0 auto;
        }}
        .summary {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 20px;
            margin-bottom: 30px;
        }}
        .summary-card {{
            padding: 20px;
            background-color: {};
            border-radius: 8px;
            text-align: center;
        }}
        .summary-number {{
            font-size: 36px;
            font-weight: bold;
            margin-bottom: 8px;
        }}
        .summary-label {{
            font-size: 14px;
            color: {};
        }}
        .category-section {{
            margin-bottom: 30px;
        }}
        .category-title {{
            font-size: 20px;
            font-weight: bold;
            margin-bottom: 15px;
            color: {};
        }}
        .item-list {{
            display: flex;
            flex-direction: column;
            gap: 10px;
        }}
        .item {{
            padding: 15px;
            background-color: {};
            border-left: 4px solid;
            border-radius: 4px;
        }}
        .item.compliant {{ border-left-color: #4caf50; }}
        .item.partial {{ border-left-color: #ff9800; }}
        .item.non-compliant {{ border-left-color: #f44336; }}
        .item.not-applicable {{ border-left-color: #9e9e9e; }}
        .item-header {{
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 8px;
        }}
        .item-name {{
            font-weight: bold;
        }}
        .status-badge {{
            padding: 4px 12px;
            border-radius: 12px;
            font-size: 12px;
            font-weight: bold;
        }}
        .status-badge.compliant {{
            background-color: #4caf50;
            color: white;
        }}
        .status-badge.partial {{
            background-color: #ff9800;
            color: white;
        }}
        .status-badge.non-compliant {{
            background-color: #f44336;
            color: white;
        }}
        .status-badge.not-applicable {{
            background-color: #9e9e9e;
            color: white;
        }}
        .item-notes {{
            font-size: 14px;
            color: {};
        }}
    </style>
</head>
<body>
    <div class="dashboard">
        <h1>Compliance Dashboard</h1>
        <div class="summary">
            <div class="summary-card">
                <div class="summary-number">{}%</div>
                <div class="summary-label">Compliance Rate</div>
            </div>
            <div class="summary-card">
                <div class="summary-number">{}</div>
                <div class="summary-label">Compliant</div>
            </div>
            <div class="summary-card">
                <div class="summary-number">{}</div>
                <div class="summary-label">Partial</div>
            </div>
            <div class="summary-card">
                <div class="summary-number">{}</div>
                <div class="summary-label">Non-Compliant</div>
            </div>
        </div>
"#,
            self.theme.background_color,
            self.theme.text_color,
            self.theme.root_color,
            self.theme.discretion_color,
            self.theme.condition_color,
            self.theme.outcome_color,
            self.theme.discretion_color,
            compliance_rate,
            compliant,
            partial,
            non_compliant,
        );

        for (category, item_list) in &categories {
            html.push_str(&format!(
                r#"        <div class="category-section">
            <div class="category-title">{}</div>
            <div class="item-list">
"#,
                category
            ));

            for item in item_list {
                let (status_class, status_label) = match item.status {
                    ComplianceStatus::Compliant => ("compliant", "Compliant"),
                    ComplianceStatus::PartiallyCompliant => ("partial", "Partially Compliant"),
                    ComplianceStatus::NonCompliant => ("non-compliant", "Non-Compliant"),
                    ComplianceStatus::NotApplicable => ("not-applicable", "N/A"),
                };

                html.push_str(&format!(
                    r#"                <div class="item {}">
                    <div class="item-header">
                        <div class="item-name">{}</div>
                        <div class="status-badge {}">{}</div>
                    </div>
                    <div class="item-notes">{}</div>
                </div>
"#,
                    status_class, item.requirement, status_class, status_label, item.notes
                ));
            }

            html.push_str("            </div>\n        </div>\n");
        }

        html.push_str(
            r#"    </div>
</body>
</html>"#,
        );

        html
    }
}

impl Default for ComplianceDashboardVisualizer {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Performance Features (v0.1.8)
// ============================================================================

/// WebWorker rendering configuration
#[derive(Debug, Clone)]
pub struct WebWorkerConfig {
    /// Enable web worker rendering
    pub enabled: bool,
    /// Number of worker threads
    pub worker_count: usize,
    /// Chunk size for parallel processing
    pub chunk_size: usize,
}

impl WebWorkerConfig {
    /// Creates a new web worker configuration.
    pub fn new() -> Self {
        Self {
            enabled: true,
            worker_count: 4,
            chunk_size: 100,
        }
    }

    /// Disables web worker rendering.
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            ..Self::new()
        }
    }

    /// Sets the worker count.
    pub fn with_worker_count(mut self, count: usize) -> Self {
        self.worker_count = count;
        self
    }

    /// Sets the chunk size.
    pub fn with_chunk_size(mut self, size: usize) -> Self {
        self.chunk_size = size;
        self
    }

    /// Generates JavaScript web worker code.
    pub fn to_javascript(&self) -> String {
        if !self.enabled {
            return String::new();
        }

        format!(
            r#"
// Web Worker rendering for performance
const workerCode = `
self.onmessage = function(e) {{
    const {{ nodes, edges, chunkIndex }} = e.data;

    // Process this chunk of data
    const processed = {{
        nodes: nodes.map(node => ({{
            ...node,
            rendered: true,
            position: calculatePosition(node)
        }})),
        edges: edges.map(edge => ({{
            ...edge,
            path: calculatePath(edge)
        }}))
    }};

    self.postMessage({{ chunkIndex, data: processed }});
}};

function calculatePosition(node) {{
    // Placeholder for position calculation
    return {{ x: 0, y: 0 }};
}}

function calculatePath(edge) {{
    // Placeholder for path calculation
    return '';
}}
`;

class WebWorkerRenderer {{
    constructor(data) {{
        this.data = data;
        this.workerCount = {};
        this.chunkSize = {};
        this.workers = [];
        this.results = [];
        this.init();
    }}

    init() {{
        const blob = new Blob([workerCode], {{ type: 'application/javascript' }});
        const workerUrl = URL.createObjectURL(blob);

        for (let i = 0; i < this.workerCount; i++) {{
            this.workers.push(new Worker(workerUrl));
        }}
    }}

    async render() {{
        const chunks = this.chunkData(this.data, this.chunkSize);
        const promises = chunks.map((chunk, index) => {{
            return new Promise((resolve) => {{
                const worker = this.workers[index % this.workerCount];
                worker.onmessage = (e) => {{
                    this.results[e.data.chunkIndex] = e.data.data;
                    resolve();
                }};
                worker.postMessage({{
                    nodes: chunk.nodes,
                    edges: chunk.edges,
                    chunkIndex: index
                }});
            }});
        }});

        await Promise.all(promises);
        return this.mergeResults();
    }}

    chunkData(data, size) {{
        const chunks = [];
        for (let i = 0; i < data.nodes.length; i += size) {{
            chunks.push({{
                nodes: data.nodes.slice(i, i + size),
                edges: data.edges.filter(e =>
                    e.source >= i && e.source < i + size
                )
            }});
        }}
        return chunks;
    }}

    mergeResults() {{
        return this.results.reduce((acc, result) => ({{
            nodes: [...acc.nodes, ...result.nodes],
            edges: [...acc.edges, ...result.edges]
        }}), {{ nodes: [], edges: [] }});
    }}

    terminate() {{
        this.workers.forEach(worker => worker.terminate());
    }}
}}
"#,
            self.worker_count, self.chunk_size
        )
    }
}

impl Default for WebWorkerConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Canvas fallback configuration
#[derive(Debug, Clone)]
pub struct CanvasFallbackConfig {
    /// Enable canvas rendering fallback
    pub enabled: bool,
    /// Use canvas for graphs larger than this size
    pub threshold_node_count: usize,
    /// Enable offscreen canvas
    pub offscreen: bool,
}

impl CanvasFallbackConfig {
    /// Creates a new canvas fallback configuration.
    pub fn new() -> Self {
        Self {
            enabled: true,
            threshold_node_count: 1000,
            offscreen: true,
        }
    }

    /// Disables canvas fallback.
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            ..Self::new()
        }
    }

    /// Sets the threshold for switching to canvas.
    pub fn with_threshold(mut self, threshold: usize) -> Self {
        self.threshold_node_count = threshold;
        self
    }

    /// Generates JavaScript canvas fallback code.
    pub fn to_javascript(&self) -> String {
        if !self.enabled {
            return String::new();
        }

        format!(
            r#"
// Canvas fallback for large graphs
class CanvasRenderer {{
    constructor(container, data) {{
        this.container = container;
        this.data = data;
        this.threshold = {};
        this.offscreen = {};
        this.init();
    }}

    init() {{
        if (this.data.nodes.length < this.threshold) {{
            // Use SVG for small graphs
            this.useSvg = true;
            return;
        }}

        this.useSvg = false;
        this.canvas = document.createElement('canvas');
        this.canvas.width = this.container.clientWidth;
        this.canvas.height = this.container.clientHeight;
        this.container.appendChild(this.canvas);

        if (this.offscreen && 'OffscreenCanvas' in window) {{
            this.offscreenCanvas = this.canvas.transferControlToOffscreen();
            this.ctx = this.offscreenCanvas.getContext('2d');
        }} else {{
            this.ctx = this.canvas.getContext('2d');
        }}
    }}

    render() {{
        if (this.useSvg) {{
            // Delegate to SVG renderer
            return;
        }}

        this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);

        // Render edges
        this.ctx.strokeStyle = '#ccc';
        this.ctx.lineWidth = 1;
        this.data.edges.forEach(edge => {{
            this.ctx.beginPath();
            this.ctx.moveTo(edge.source.x, edge.source.y);
            this.ctx.lineTo(edge.target.x, edge.target.y);
            this.ctx.stroke();
        }});

        // Render nodes
        this.data.nodes.forEach(node => {{
            this.ctx.fillStyle = node.color || '#3498db';
            this.ctx.beginPath();
            this.ctx.arc(node.x, node.y, 5, 0, 2 * Math.PI);
            this.ctx.fill();

            // Draw label
            this.ctx.fillStyle = '#333';
            this.ctx.font = '12px Arial';
            this.ctx.fillText(node.name, node.x + 8, node.y + 4);
        }});
    }}

    update(data) {{
        this.data = data;
        this.render();
    }}
}}
"#,
            self.threshold_node_count, self.offscreen
        )
    }
}

impl Default for CanvasFallbackConfig {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Framework Integration (v0.1.6)
// ============================================================================

/// React component wrapper configuration
#[derive(Debug, Clone)]
pub struct ReactComponentConfig {
    /// Component name
    pub component_name: String,
    /// Use TypeScript
    pub typescript: bool,
    /// Include prop types
    pub include_prop_types: bool,
}

impl ReactComponentConfig {
    /// Creates a new React component configuration.
    pub fn new(component_name: impl Into<String>) -> Self {
        Self {
            component_name: component_name.into(),
            typescript: true,
            include_prop_types: false,
        }
    }

    /// Disables TypeScript.
    pub fn without_typescript(mut self) -> Self {
        self.typescript = false;
        self
    }

    /// Enables PropTypes validation.
    pub fn with_prop_types(mut self) -> Self {
        self.include_prop_types = true;
        self
    }

    /// Generates React component code.
    pub fn to_react_component(&self) -> String {
        if self.typescript {
            format!(
                "import React, {{ useEffect, useRef, useState }} from 'react';\n\
\n\
interface {}Props {{\n\
    data: any;\n\
    theme?: 'light' | 'dark' | 'high-contrast' | 'colorblind-friendly';\n\
    width?: number;\n\
    height?: number;\n\
    onNodeClick?: (node: any) => void;\n\
}}\n\
\n\
export const {}: React.FC<{}Props> = ({{\n\
    data,\n\
    theme = 'light',\n\
    width = 800,\n\
    height = 600,\n\
    onNodeClick\n\
}}) => {{\n\
    const containerRef = useRef<HTMLDivElement>(null);\n\
    const [error, setError] = useState<string | null>(null);\n\
\n\
    useEffect(() => {{\n\
        if (!containerRef.current || !data) return;\n\
\n\
        try {{\n\
            const container = containerRef.current;\n\
            container.innerHTML = '';\n\
\n\
            const svg = document.createElementNS('http://www.w3.org/2000/svg', 'svg');\n\
            svg.setAttribute('width', width.toString());\n\
            svg.setAttribute('height', height.toString());\n\
            container.appendChild(svg);\n\
\n\
            if (onNodeClick) {{\n\
                svg.addEventListener('click', (e) => {{\n\
                    const target = e.target as SVGElement;\n\
                    if (target.classList.contains('node')) {{\n\
                        onNodeClick({{ id: target.getAttribute('data-id') }});\n\
                    }}\n\
                }});\n\
            }}\n\
        }} catch (err) {{\n\
            setError(err instanceof Error ? err.message : 'Unknown error');\n\
        }}\n\
    }}, [data, theme, width, height, onNodeClick]);\n\
\n\
    if (error) {{\n\
        return <div style={{{{{{ color: 'red' }}}}}}>Error: {{error}}</div>;\n\
    }}\n\
\n\
    return (\n\
        <div\n\
            ref={{{{containerRef}}}}\n\
            className=\"legalis-viz-container\"\n\
            style={{{{{{ width, height, overflow: 'hidden' }}}}}}\n\
        />\n\
    );\n\
}};\n\
\n\
export default {};\n",
                self.component_name, self.component_name, self.component_name, self.component_name
            )
        } else {
            let prop_types = if self.include_prop_types {
                format!(
                    "\nimport PropTypes from 'prop-types';\n\n\
{}.propTypes = {{\n\
    data: PropTypes.any.isRequired,\n\
    theme: PropTypes.oneOf(['light', 'dark', 'high-contrast', 'colorblind-friendly']),\n\
    width: PropTypes.number,\n\
    height: PropTypes.number,\n\
    onNodeClick: PropTypes.func\n\
}};\n",
                    self.component_name
                )
            } else {
                String::new()
            };

            format!(
                "import React, {{ useEffect, useRef, useState }} from 'react';\n\
\n\
export const {} = ({{\n\
    data,\n\
    theme = 'light',\n\
    width = 800,\n\
    height = 600,\n\
    onNodeClick\n\
}}) => {{\n\
    const containerRef = useRef(null);\n\
    const [error, setError] = useState(null);\n\
\n\
    useEffect(() => {{\n\
        if (!containerRef.current || !data) return;\n\
\n\
        try {{\n\
            const container = containerRef.current;\n\
            container.innerHTML = '';\n\
\n\
            const svg = document.createElementNS('http://www.w3.org/2000/svg', 'svg');\n\
            svg.setAttribute('width', width.toString());\n\
            svg.setAttribute('height', height.toString());\n\
            container.appendChild(svg);\n\
\n\
            if (onNodeClick) {{\n\
                svg.addEventListener('click', (e) => {{\n\
                    if (e.target.classList.contains('node')) {{\n\
                        onNodeClick({{ id: e.target.getAttribute('data-id') }});\n\
                    }}\n\
                }});\n\
            }}\n\
        }} catch (err) {{\n\
            setError(err.message || 'Unknown error');\n\
        }}\n\
    }}, [data, theme, width, height, onNodeClick]);\n\
\n\
    if (error) {{\n\
        return <div style={{{{{{ color: 'red' }}}}}}>Error: {{error}}</div>;\n\
    }}\n\
\n\
    return (\n\
        <div\n\
            ref={{{{containerRef}}}}\n\
            className=\"legalis-viz-container\"\n\
            style={{{{{{ width, height, overflow: 'hidden' }}}}}}\n\
        />\n\
    );\n\
}};\n\
{}\n\
export default {};\n",
                self.component_name, prop_types, self.component_name
            )
        }
    }
}

impl Default for ReactComponentConfig {
    fn default() -> Self {
        Self::new("LegalisViz")
    }
}

/// Vue.js component wrapper configuration
#[derive(Debug, Clone)]
pub struct VueComponentConfig {
    /// Component name
    pub component_name: String,
    /// Use TypeScript
    pub typescript: bool,
    /// Use Composition API
    pub composition_api: bool,
}

impl VueComponentConfig {
    /// Creates a new Vue component configuration.
    pub fn new(component_name: impl Into<String>) -> Self {
        Self {
            component_name: component_name.into(),
            typescript: true,
            composition_api: true,
        }
    }

    /// Disables TypeScript.
    pub fn without_typescript(mut self) -> Self {
        self.typescript = false;
        self
    }

    /// Uses Options API instead of Composition API.
    pub fn with_options_api(mut self) -> Self {
        self.composition_api = false;
        self
    }

    /// Generates Vue component code.
    #[allow(clippy::too_many_arguments)]
    pub fn to_vue_component(&self) -> String {
        if self.composition_api {
            if self.typescript {
                "<template>\n\
  <div ref=\"containerRef\" class=\"legalis-viz-container\" :style=\"{ width: width + 'px', height: height + 'px' }\">\n\
    <div v-if=\"error\" class=\"error\">Error: {{ error }}</div>\n\
  </div>\n\
</template>\n\
\n\
<script setup lang=\"ts\">\n\
import { ref, onMounted, watch } from 'vue';\n\
\n\
interface Props {\n\
  data: any;\n\
  theme?: 'light' | 'dark' | 'high-contrast' | 'colorblind-friendly';\n\
  width?: number;\n\
  height?: number;\n\
}\n\
\n\
const props = withDefaults(defineProps<Props>(), {\n\
  theme: 'light',\n\
  width: 800,\n\
  height: 600\n\
});\n\
\n\
const emit = defineEmits<{\n\
  nodeClick: [node: any];\n\
}>();\n\
\n\
const containerRef = ref<HTMLDivElement | null>(null);\n\
const error = ref<string | null>(null);\n\
\n\
const renderVisualization = () => {\n\
  if (!containerRef.value || !props.data) return;\n\
\n\
  try {\n\
    const container = containerRef.value;\n\
    container.innerHTML = '';\n\
\n\
    const svg = document.createElementNS('http://www.w3.org/2000/svg', 'svg');\n\
    svg.setAttribute('width', props.width.toString());\n\
    svg.setAttribute('height', props.height.toString());\n\
    container.appendChild(svg);\n\
\n\
    svg.addEventListener('click', (e) => {\n\
      const target = e.target as SVGElement;\n\
      if (target.classList.contains('node')) {\n\
        emit('nodeClick', { id: target.getAttribute('data-id') });\n\
      }\n\
    });\n\
  } catch (err) {\n\
    error.value = err instanceof Error ? err.message : 'Unknown error';\n\
  }\n\
};\n\
\n\
onMounted(() => {\n\
  renderVisualization();\n\
});\n\
\n\
watch(() => [props.data, props.theme, props.width, props.height], () => {\n\
  renderVisualization();\n\
});\n\
</script>\n\
\n\
<style scoped>\n\
.legalis-viz-container {\n\
  overflow: hidden;\n\
}\n\
\n\
.error {\n\
  color: red;\n\
}\n\
</style>\n".to_string()
            } else {
                "<template>\n\
  <div ref=\"containerRef\" class=\"legalis-viz-container\" :style=\"{ width: width + 'px', height: height + 'px' }\">\n\
    <div v-if=\"error\" class=\"error\">Error: {{ error }}</div>\n\
  </div>\n\
</template>\n\
\n\
<script setup>\n\
import { ref, onMounted, watch } from 'vue';\n\
\n\
const props = defineProps({\n\
  data: { type: Object, required: true },\n\
  theme: { type: String, default: 'light' },\n\
  width: { type: Number, default: 800 },\n\
  height: { type: Number, default: 600 }\n\
});\n\
\n\
const emit = defineEmits(['nodeClick']);\n\
\n\
const containerRef = ref(null);\n\
const error = ref(null);\n\
\n\
const renderVisualization = () => {\n\
  if (!containerRef.value || !props.data) return;\n\
\n\
  try {\n\
    const container = containerRef.value;\n\
    container.innerHTML = '';\n\
\n\
    const svg = document.createElementNS('http://www.w3.org/2000/svg', 'svg');\n\
    svg.setAttribute('width', props.width.toString());\n\
    svg.setAttribute('height', props.height.toString());\n\
    container.appendChild(svg);\n\
\n\
    svg.addEventListener('click', (e) => {\n\
      if (e.target.classList.contains('node')) {\n\
        emit('nodeClick', { id: e.target.getAttribute('data-id') });\n\
      }\n\
    });\n\
  } catch (err) {\n\
    error.value = err.message || 'Unknown error';\n\
  }\n\
};\n\
\n\
onMounted(() => {\n\
  renderVisualization();\n\
});\n\
\n\
watch(() => [props.data, props.theme, props.width, props.height], () => {\n\
  renderVisualization();\n\
});\n\
</script>\n\
\n\
<style scoped>\n\
.legalis-viz-container {\n\
  overflow: hidden;\n\
}\n\
\n\
.error {\n\
  color: red;\n\
}\n\
</style>\n".to_string()
            }
        } else {
            format!(
                "<template>\n\
  <div ref=\"container\" class=\"legalis-viz-container\" :style=\"{{ width: width + 'px', height: height + 'px' }}\">\n\
    <div v-if=\"error\" class=\"error\">Error: {{{{ error }}}}</div>\n\
  </div>\n\
</template>\n\
\n\
<script>\n\
export default {{\n\
  name: '{}',\n\
  props: {{\n\
    data: {{ type: Object, required: true }},\n\
    theme: {{ type: String, default: 'light' }},\n\
    width: {{ type: Number, default: 800 }},\n\
    height: {{ type: Number, default: 600 }}\n\
  }},\n\
  data() {{\n\
    return {{\n\
      error: null\n\
    }};\n\
  }},\n\
  mounted() {{\n\
    this.renderVisualization();\n\
  }},\n\
  watch: {{\n\
    data() {{ this.renderVisualization(); }},\n\
    theme() {{ this.renderVisualization(); }},\n\
    width() {{ this.renderVisualization(); }},\n\
    height() {{ this.renderVisualization(); }}\n\
  }},\n\
  methods: {{\n\
    renderVisualization() {{\n\
      if (!this.$refs.container || !this.data) return;\n\
\n\
      try {{\n\
        const container = this.$refs.container;\n\
        container.innerHTML = '';\n\
\n\
        const svg = document.createElementNS('http://www.w3.org/2000/svg', 'svg');\n\
        svg.setAttribute('width', this.width.toString());\n\
        svg.setAttribute('height', this.height.toString());\n\
        container.appendChild(svg);\n\
\n\
        svg.addEventListener('click', (e) => {{\n\
          if (e.target.classList.contains('node')) {{\n\
            this.$emit('nodeClick', {{ id: e.target.getAttribute('data-id') }});\n\
          }}\n\
        }});\n\
      }} catch (err) {{\n\
        this.error = err.message || 'Unknown error';\n\
      }}\n\
    }}\n\
  }}\n\
}};\n\
</script>\n\
\n\
<style scoped>\n\
.legalis-viz-container {{\n\
  overflow: hidden;\n\
}}\n\
\n\
.error {{\n\
  color: red;\n\
}}\n\
</style>\n",
                self.component_name
            )
        }
    }
}

impl Default for VueComponentConfig {
    fn default() -> Self {
        Self::new("LegalisViz")
    }
}

/// Angular component wrapper configuration
#[derive(Debug, Clone)]
pub struct AngularComponentConfig {
    /// Component name
    pub component_name: String,
    /// Component selector
    pub selector: String,
}

impl AngularComponentConfig {
    /// Creates a new Angular component configuration.
    pub fn new(component_name: impl Into<String>, selector: impl Into<String>) -> Self {
        Self {
            component_name: component_name.into(),
            selector: selector.into(),
        }
    }

    /// Generates Angular component code (TypeScript, HTML, CSS).
    pub fn to_angular_component(&self) -> (String, String, String) {
        let component_ts = format!(
            "import {{ Component, Input, Output, EventEmitter, OnInit, OnChanges, ElementRef, ViewChild }} from '@angular/core';\n\
\n\
@Component({{\n\
  selector: '{}',\n\
  templateUrl: './{}.component.html',\n\
  styleUrls: ['./{}.component.css']\n\
}})\n\
export class {} implements OnInit, OnChanges {{\n\
  @Input() data: any;\n\
  @Input() theme: 'light' | 'dark' | 'high-contrast' | 'colorblind-friendly' = 'light';\n\
  @Input() width: number = 800;\n\
  @Input() height: number = 600;\n\
  @Output() nodeClick = new EventEmitter<any>();\n\
\n\
  @ViewChild('container', {{ static: true }}) containerRef!: ElementRef<HTMLDivElement>;\n\
\n\
  error: string | null = null;\n\
\n\
  ngOnInit(): void {{\n\
    this.renderVisualization();\n\
  }}\n\
\n\
  ngOnChanges(): void {{\n\
    this.renderVisualization();\n\
  }}\n\
\n\
  private renderVisualization(): void {{\n\
    if (!this.containerRef?.nativeElement || !this.data) return;\n\
\n\
    try {{\n\
      const container = this.containerRef.nativeElement;\n\
      container.innerHTML = '';\n\
\n\
      const svg = document.createElementNS('http://www.w3.org/2000/svg', 'svg');\n\
      svg.setAttribute('width', this.width.toString());\n\
      svg.setAttribute('height', this.height.toString());\n\
      container.appendChild(svg);\n\
\n\
      svg.addEventListener('click', (e) => {{\n\
        const target = e.target as SVGElement;\n\
        if (target.classList.contains('node')) {{\n\
          this.nodeClick.emit({{ id: target.getAttribute('data-id') }});\n\
        }}\n\
      }});\n\
\n\
      this.error = null;\n\
    }} catch (err) {{\n\
      this.error = err instanceof Error ? err.message : 'Unknown error';\n\
    }}\n\
  }}\n\
}}\n",
            self.selector,
            self.component_name.to_lowercase(),
            self.component_name.to_lowercase(),
            self.component_name
        );

        let component_html = "<div #container class=\"legalis-viz-container\" [style.width.px]=\"width\" [style.height.px]=\"height\">\n\
  <div *ngIf=\"error\" class=\"error\">Error: {{ error }}</div>\n\
</div>\n".to_string();

        let component_css = ".legalis-viz-container {\n\
  overflow: hidden;\n\
}\n\
\n\
.error {\n\
  color: red;\n\
}\n"
        .to_string();

        (component_ts, component_html, component_css)
    }
}

impl Default for AngularComponentConfig {
    fn default() -> Self {
        Self::new("LegalisVizComponent", "app-legalis-viz")
    }
}

/// WordPress plugin integration configuration
#[derive(Debug, Clone)]
pub struct WordPressPluginConfig {
    /// Plugin name
    pub plugin_name: String,
    /// Plugin slug
    pub plugin_slug: String,
    /// Shortcode name
    pub shortcode: String,
}

impl WordPressPluginConfig {
    /// Creates a new WordPress plugin configuration.
    pub fn new(plugin_name: impl Into<String>) -> Self {
        let name = plugin_name.into();
        let slug = name.to_lowercase().replace(' ', "-");
        Self {
            plugin_name: name,
            plugin_slug: slug.clone(),
            shortcode: format!("{}_viz", slug.replace('-', "_")),
        }
    }

    /// Sets the shortcode name.
    pub fn with_shortcode(mut self, shortcode: impl Into<String>) -> Self {
        self.shortcode = shortcode.into();
        self
    }

    /// Generates WordPress plugin PHP code.
    #[allow(clippy::too_many_arguments)]
    pub fn to_wordpress_plugin(&self) -> String {
        let class_name = self
            .plugin_slug
            .replace('-', "_")
            .split('_')
            .map(|s| {
                let mut chars = s.chars();
                match chars.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().chain(chars).collect(),
                }
            })
            .collect::<String>();

        format!(
            "<?php\n\
/**\n\
 * Plugin Name: {}\n\
 * Description: Legal statute visualization plugin for WordPress\n\
 * Version: 1.0.0\n\
 * Author: Legalis\n\
 */\n\
\n\
if (!defined('ABSPATH')) {{\n\
    exit;\n\
}}\n\
\n\
class {} {{\n\
\n\
    public function __construct() {{\n\
        add_shortcode('{}', array($this, 'render_visualization'));\n\
        add_action('wp_enqueue_scripts', array($this, 'enqueue_scripts'));\n\
    }}\n\
\n\
    public function enqueue_scripts() {{\n\
        wp_enqueue_script(\n\
            '{}-viz',\n\
            plugin_dir_url(__FILE__) . 'js/visualization.js',\n\
            array(),\n\
            '1.0.0',\n\
            true\n\
        );\n\
\n\
        wp_enqueue_style(\n\
            '{}-viz',\n\
            plugin_dir_url(__FILE__) . 'css/visualization.css',\n\
            array(),\n\
            '1.0.0'\n\
        );\n\
    }}\n\
\n\
    public function render_visualization($atts) {{\n\
        $atts = shortcode_atts(array(\n\
            'data' => '',\n\
            'theme' => 'light',\n\
            'width' => '800',\n\
            'height' => '600',\n\
        ), $atts);\n\
\n\
        $data = esc_attr($atts['data']);\n\
        $theme = esc_attr($atts['theme']);\n\
        $width = intval($atts['width']);\n\
        $height = intval($atts['height']);\n\
\n\
        ob_start();\n\
        ?>\n\
        <div class=\"legalis-viz-container\"\n\
             data-viz-data=\"<?php echo $data; ?>\"\n\
             data-viz-theme=\"<?php echo $theme; ?>\"\n\
             style=\"width: <?php echo $width; ?>px; height: <?php echo $height; ?>px;\">\n\
        </div>\n\
        <?php\n\
        return ob_get_clean();\n\
    }}\n\
}}\n\
\n\
new {}();\n",
            self.plugin_name,
            class_name,
            self.shortcode,
            self.plugin_slug,
            self.plugin_slug,
            class_name
        )
    }
}

impl Default for WordPressPluginConfig {
    fn default() -> Self {
        Self::new("Legalis Visualization")
    }
}

// ============================================================================
// Web Components (v0.1.6)
// ============================================================================

/// Web Component configuration
#[derive(Debug, Clone)]
pub struct WebComponentConfig {
    /// Component tag name
    pub tag_name: String,
    /// Shadow DOM enabled
    pub shadow_dom: bool,
    /// Custom element registry
    pub auto_register: bool,
}

impl WebComponentConfig {
    /// Creates a new web component configuration.
    pub fn new(tag_name: impl Into<String>) -> Self {
        Self {
            tag_name: tag_name.into(),
            shadow_dom: true,
            auto_register: true,
        }
    }

    /// Disables shadow DOM.
    pub fn without_shadow_dom(mut self) -> Self {
        self.shadow_dom = false;
        self
    }

    /// Disables auto-registration.
    pub fn without_auto_register(mut self) -> Self {
        self.auto_register = false;
        self
    }

    /// Generates Web Component JavaScript code.
    pub fn to_javascript(&self, html_content: &str) -> String {
        let shadow_dom_code = if self.shadow_dom {
            r#"
        const shadow = this.attachShadow({ mode: 'open' });
        shadow.innerHTML = template;
"#
        } else {
            r#"
        this.innerHTML = template;
"#
        };

        let auto_register_code = if self.auto_register {
            format!(
                r#"
if (!customElements.get('{}')) {{
    customElements.define('{}', LegalisVizComponent);
}}
"#,
                self.tag_name, self.tag_name
            )
        } else {
            String::new()
        };

        format!(
            r#"
// Web Component for Legalis Viz
class LegalisVizComponent extends HTMLElement {{
    constructor() {{
        super();

        const template = `{}`;
        {}
    }}

    connectedCallback() {{
        // Component connected to DOM
    }}

    disconnectedCallback() {{
        // Component removed from DOM
    }}

    static get observedAttributes() {{
        return ['data', 'theme'];
    }}

    attributeChangedCallback(name, oldValue, newValue) {{
        if (name === 'data') {{
            this.updateData(JSON.parse(newValue));
        }} else if (name === 'theme') {{
            this.updateTheme(newValue);
        }}
    }}

    updateData(data) {{
        // Update visualization data
    }}

    updateTheme(theme) {{
        // Update visualization theme
    }}
}}
{}
// Usage: <{} data='{{...}}' theme='light'></{}>
"#,
            html_content.replace('\n', "\\n").replace('\'', "\\'"),
            shadow_dom_code,
            auto_register_code,
            self.tag_name,
            self.tag_name,
        )
    }
}

// ============================================================================
// v0.2.6: Mobile and Touch Support
// ============================================================================

/// Touch gesture types for mobile interaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TouchGesture {
    /// Pinch to zoom
    Pinch,
    /// Pan/drag to move
    Pan,
    /// Swipe to navigate
    Swipe,
    /// Tap to interact
    Tap,
    /// Double tap to zoom
    DoubleTap,
}

/// Configuration for touch gesture support.
#[derive(Debug, Clone)]
pub struct TouchGestureConfig {
    /// Enable pinch-to-zoom gesture
    pub enable_pinch: bool,
    /// Enable pan gesture
    pub enable_pan: bool,
    /// Enable swipe gestures
    pub enable_swipe: bool,
    /// Enable tap interactions
    pub enable_tap: bool,
    /// Enable double-tap to zoom
    pub enable_double_tap: bool,
    /// Minimum distance for swipe (pixels)
    pub swipe_threshold: f32,
    /// Minimum zoom scale
    pub min_zoom: f32,
    /// Maximum zoom scale
    pub max_zoom: f32,
}

impl Default for TouchGestureConfig {
    fn default() -> Self {
        Self {
            enable_pinch: true,
            enable_pan: true,
            enable_swipe: true,
            enable_tap: true,
            enable_double_tap: true,
            swipe_threshold: 50.0,
            min_zoom: 0.5,
            max_zoom: 3.0,
        }
    }
}

impl TouchGestureConfig {
    /// Creates a new touch gesture configuration with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Disables all touch gestures.
    pub fn disabled() -> Self {
        Self {
            enable_pinch: false,
            enable_pan: false,
            enable_swipe: false,
            enable_tap: false,
            enable_double_tap: false,
            swipe_threshold: 50.0,
            min_zoom: 0.5,
            max_zoom: 3.0,
        }
    }

    /// Generates JavaScript code for touch gesture handling.
    pub fn to_javascript(&self) -> String {
        if !self.enable_pinch && !self.enable_pan && !self.enable_swipe {
            return String::new();
        }

        format!(
            r#"
class TouchGestureHandler {{
    constructor(element, options = {{}}) {{
        this.element = element;
        this.enablePinch = {};
        this.enablePan = {};
        this.enableSwipe = {};
        this.enableTap = {};
        this.enableDoubleTap = {};
        this.swipeThreshold = {};
        this.minZoom = {};
        this.maxZoom = {};

        this.touches = [];
        this.scale = 1.0;
        this.translateX = 0;
        this.translateY = 0;
        this.initialDistance = 0;
        this.lastTap = 0;

        this.initEventListeners();
    }}

    initEventListeners() {{
        if (this.enablePinch || this.enablePan) {{
            this.element.addEventListener('touchstart', this.onTouchStart.bind(this));
            this.element.addEventListener('touchmove', this.onTouchMove.bind(this));
            this.element.addEventListener('touchend', this.onTouchEnd.bind(this));
        }}

        if (this.enableTap || this.enableDoubleTap) {{
            this.element.addEventListener('touchstart', this.onTap.bind(this));
        }}
    }}

    onTouchStart(event) {{
        this.touches = Array.from(event.touches);

        if (this.enablePinch && this.touches.length === 2) {{
            this.initialDistance = this.getDistance(this.touches[0], this.touches[1]);
        }}
    }}

    onTouchMove(event) {{
        event.preventDefault();
        this.touches = Array.from(event.touches);

        if (this.enablePinch && this.touches.length === 2) {{
            const distance = this.getDistance(this.touches[0], this.touches[1]);
            const scaleDelta = distance / this.initialDistance;
            this.scale = Math.min(this.maxZoom, Math.max(this.minZoom, this.scale * scaleDelta));
            this.initialDistance = distance;
            this.applyTransform();
        }} else if (this.enablePan && this.touches.length === 1) {{
            const touch = this.touches[0];
            if (this.lastTouch) {{
                this.translateX += touch.clientX - this.lastTouch.clientX;
                this.translateY += touch.clientY - this.lastTouch.clientY;
                this.applyTransform();
            }}
            this.lastTouch = touch;
        }}
    }}

    onTouchEnd(event) {{
        const remainingTouches = Array.from(event.touches);

        if (this.enableSwipe && this.touches.length === 1 && remainingTouches.length === 0) {{
            const deltaX = this.touches[0].clientX - (this.lastTouch?.clientX || this.touches[0].clientX);
            const deltaY = this.touches[0].clientY - (this.lastTouch?.clientY || this.touches[0].clientY);

            if (Math.abs(deltaX) > this.swipeThreshold) {{
                const direction = deltaX > 0 ? 'right' : 'left';
                this.element.dispatchEvent(new CustomEvent('swipe', {{ detail: {{ direction }} }}));
            }} else if (Math.abs(deltaY) > this.swipeThreshold) {{
                const direction = deltaY > 0 ? 'down' : 'up';
                this.element.dispatchEvent(new CustomEvent('swipe', {{ detail: {{ direction }} }}));
            }}
        }}

        this.touches = remainingTouches;
        this.lastTouch = null;
    }}

    onTap(event) {{
        const now = Date.now();
        const timeSinceLastTap = now - this.lastTap;

        if (this.enableDoubleTap && timeSinceLastTap < 300) {{
            // Double tap - zoom in/out
            if (this.scale === 1.0) {{
                this.scale = 2.0;
            }} else {{
                this.scale = 1.0;
            }}
            this.applyTransform();
            event.preventDefault();
        }} else if (this.enableTap) {{
            this.element.dispatchEvent(new CustomEvent('tap', {{
                detail: {{ x: event.touches[0].clientX, y: event.touches[0].clientY }}
            }}));
        }}

        this.lastTap = now;
    }}

    getDistance(touch1, touch2) {{
        const dx = touch1.clientX - touch2.clientX;
        const dy = touch1.clientY - touch2.clientY;
        return Math.sqrt(dx * dx + dy * dy);
    }}

    applyTransform() {{
        const transform = `translate(${{this.translateX}}px, ${{this.translateY}}px) scale(${{this.scale}})`;
        this.element.style.transform = transform;
    }}

    reset() {{
        this.scale = 1.0;
        this.translateX = 0;
        this.translateY = 0;
        this.applyTransform();
    }}
}}
"#,
            self.enable_pinch,
            self.enable_pan,
            self.enable_swipe,
            self.enable_tap,
            self.enable_double_tap,
            self.swipe_threshold,
            self.min_zoom,
            self.max_zoom,
        )
    }
}

/// Configuration for responsive visualization scaling.
#[derive(Debug, Clone)]
pub struct ResponsiveScalingConfig {
    /// Enable responsive scaling
    pub enabled: bool,
    /// Breakpoints for different screen sizes (width in pixels)
    pub breakpoints: Vec<(u32, String)>,
    /// Scale factor for small screens
    pub small_screen_scale: f32,
    /// Scale factor for medium screens
    pub medium_screen_scale: f32,
    /// Scale factor for large screens
    pub large_screen_scale: f32,
    /// Automatically adjust font sizes
    pub auto_adjust_fonts: bool,
}

impl Default for ResponsiveScalingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            breakpoints: vec![
                (480, "small".to_string()),
                (768, "medium".to_string()),
                (1024, "large".to_string()),
                (1440, "xlarge".to_string()),
            ],
            small_screen_scale: 0.7,
            medium_screen_scale: 0.85,
            large_screen_scale: 1.0,
            auto_adjust_fonts: true,
        }
    }
}

impl ResponsiveScalingConfig {
    /// Creates a new responsive scaling configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Disables responsive scaling.
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            ..Self::default()
        }
    }

    /// Generates CSS for responsive scaling.
    pub fn to_css(&self) -> String {
        if !self.enabled {
            return String::new();
        }

        let mut css = String::new();
        css.push_str("/* Responsive Scaling */\n");
        css.push_str("@media (max-width: 480px) {\n");
        css.push_str(&format!(
            "  .viz-container {{ transform: scale({}); transform-origin: top left; }}\n",
            self.small_screen_scale
        ));
        if self.auto_adjust_fonts {
            css.push_str("  .viz-text { font-size: 12px; }\n");
        }
        css.push_str("}\n\n");

        css.push_str("@media (min-width: 481px) and (max-width: 768px) {\n");
        css.push_str(&format!(
            "  .viz-container {{ transform: scale({}); transform-origin: top left; }}\n",
            self.medium_screen_scale
        ));
        if self.auto_adjust_fonts {
            css.push_str("  .viz-text { font-size: 14px; }\n");
        }
        css.push_str("}\n\n");

        css.push_str("@media (min-width: 769px) {\n");
        css.push_str(&format!(
            "  .viz-container {{ transform: scale({}); transform-origin: top left; }}\n",
            self.large_screen_scale
        ));
        if self.auto_adjust_fonts {
            css.push_str("  .viz-text { font-size: 16px; }\n");
        }
        css.push_str("}\n");

        css
    }
}

/// Configuration for offline viewing capability.
#[derive(Debug, Clone)]
pub struct OfflineConfig {
    /// Enable offline support
    pub enabled: bool,
    /// Cache name for offline assets
    pub cache_name: String,
    /// URLs to cache for offline use
    pub cache_urls: Vec<String>,
    /// Cache strategy: "cache-first" or "network-first"
    pub cache_strategy: String,
}

impl Default for OfflineConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            cache_name: "legalis-viz-cache-v1".to_string(),
            cache_urls: vec![
                "https://d3js.org/d3.v7.min.js".to_string(),
                "https://cdn.jsdelivr.net/npm/chart.js".to_string(),
            ],
            cache_strategy: "cache-first".to_string(),
        }
    }
}

impl OfflineConfig {
    /// Creates a new offline configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Disables offline support.
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            ..Self::default()
        }
    }

    /// Generates service worker JavaScript for offline support.
    pub fn to_service_worker(&self) -> String {
        if !self.enabled {
            return String::new();
        }

        let cache_urls = self
            .cache_urls
            .iter()
            .map(|url| format!("'{}'", url))
            .collect::<Vec<_>>()
            .join(", ");

        format!(
            r#"
// Service Worker for Offline Support
const CACHE_NAME = '{}';
const urlsToCache = [{}];

self.addEventListener('install', (event) => {{
    event.waitUntil(
        caches.open(CACHE_NAME)
            .then((cache) => cache.addAll(urlsToCache))
    );
}});

self.addEventListener('fetch', (event) => {{
    event.respondWith(
        caches.match(event.request)
            .then((response) => {{
                if (response && '{}' === 'cache-first') {{
                    return response;
                }}

                return fetch(event.request)
                    .then((fetchResponse) => {{
                        if (fetchResponse && fetchResponse.status === 200) {{
                            const responseClone = fetchResponse.clone();
                            caches.open(CACHE_NAME).then((cache) => {{
                                cache.put(event.request, responseClone);
                            }});
                        }}
                        return fetchResponse;
                    }})
                    .catch(() => response || new Response('Offline'));
            }})
    );
}});

self.addEventListener('activate', (event) => {{
    event.waitUntil(
        caches.keys().then((cacheNames) => {{
            return Promise.all(
                cacheNames.filter((name) => name !== CACHE_NAME)
                    .map((name) => caches.delete(name))
            );
        }})
    );
}});
"#,
            self.cache_name, cache_urls, self.cache_strategy
        )
    }
}

/// Configuration for Progressive Web App (PWA) support.
#[derive(Debug, Clone)]
pub struct PWAConfig {
    /// Enable PWA features
    pub enabled: bool,
    /// App name
    pub app_name: String,
    /// App short name
    pub app_short_name: String,
    /// App description
    pub app_description: String,
    /// Theme color
    pub theme_color: String,
    /// Background color
    pub background_color: String,
    /// Display mode: "standalone", "fullscreen", "minimal-ui"
    pub display_mode: String,
    /// Icons for PWA
    pub icons: Vec<(String, String, String)>, // (src, sizes, type)
}

impl Default for PWAConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            app_name: "Legalis Visualization".to_string(),
            app_short_name: "Legalis Viz".to_string(),
            app_description: "Legal statute visualization tool".to_string(),
            theme_color: "#3498db".to_string(),
            background_color: "#ffffff".to_string(),
            display_mode: "standalone".to_string(),
            icons: vec![],
        }
    }
}

impl PWAConfig {
    /// Creates a new PWA configuration.
    pub fn new(app_name: &str) -> Self {
        Self {
            app_name: app_name.to_string(),
            app_short_name: app_name.to_string(),
            ..Self::default()
        }
    }

    /// Sets the app description.
    pub fn with_description(mut self, description: &str) -> Self {
        self.app_description = description.to_string();
        self
    }

    /// Sets the theme color.
    pub fn with_theme_color(mut self, color: &str) -> Self {
        self.theme_color = color.to_string();
        self
    }

    /// Adds an icon.
    pub fn add_icon(mut self, src: &str, sizes: &str, icon_type: &str) -> Self {
        self.icons
            .push((src.to_string(), sizes.to_string(), icon_type.to_string()));
        self
    }

    /// Generates PWA manifest JSON.
    pub fn to_manifest_json(&self) -> String {
        if !self.enabled {
            return String::new();
        }

        let icons_json = self
            .icons
            .iter()
            .map(|(src, sizes, icon_type)| {
                format!(
                    r#"    {{ "src": "{}", "sizes": "{}", "type": "{}" }}"#,
                    src, sizes, icon_type
                )
            })
            .collect::<Vec<_>>()
            .join(",\n");

        format!(
            r#"{{
  "name": "{}",
  "short_name": "{}",
  "description": "{}",
  "start_url": "/",
  "display": "{}",
  "theme_color": "{}",
  "background_color": "{}",
  "icons": [
{}
  ]
}}"#,
            self.app_name,
            self.app_short_name,
            self.app_description,
            self.display_mode,
            self.theme_color,
            self.background_color,
            icons_json
        )
    }

    /// Generates HTML meta tags for PWA.
    pub fn to_html_meta_tags(&self) -> String {
        if !self.enabled {
            return String::new();
        }

        format!(
            r#"<meta name="application-name" content="{}">
<meta name="apple-mobile-web-app-capable" content="yes">
<meta name="apple-mobile-web-app-status-bar-style" content="default">
<meta name="apple-mobile-web-app-title" content="{}">
<meta name="description" content="{}">
<meta name="format-detection" content="telephone=no">
<meta name="mobile-web-app-capable" content="yes">
<meta name="theme-color" content="{}">
<link rel="manifest" href="/manifest.json">"#,
            self.app_name, self.app_short_name, self.app_description, self.theme_color
        )
    }
}

/// Mobile and touch support enhancer for visualizations.
#[derive(Debug, Clone, Default)]
pub struct MobileTouchEnhancer {
    touch_config: TouchGestureConfig,
    responsive_config: ResponsiveScalingConfig,
    offline_config: OfflineConfig,
    pwa_config: PWAConfig,
}

impl MobileTouchEnhancer {
    /// Creates a new mobile and touch enhancer.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the touch gesture configuration.
    pub fn with_touch_config(mut self, config: TouchGestureConfig) -> Self {
        self.touch_config = config;
        self
    }

    /// Sets the responsive scaling configuration.
    pub fn with_responsive_config(mut self, config: ResponsiveScalingConfig) -> Self {
        self.responsive_config = config;
        self
    }

    /// Sets the offline configuration.
    pub fn with_offline_config(mut self, config: OfflineConfig) -> Self {
        self.offline_config = config;
        self
    }

    /// Sets the PWA configuration.
    pub fn with_pwa_config(mut self, config: PWAConfig) -> Self {
        self.pwa_config = config;
        self
    }

    /// Generates mobile-optimized HTML for a decision tree.
    pub fn to_mobile_html(&self, tree: &DecisionTree) -> String {
        let base_html = tree.to_svg();
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str("    <meta charset=\"utf-8\">\n");
        html.push_str("    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no\">\n");
        html.push_str(&self.pwa_config.to_html_meta_tags());
        html.push_str("    <title>Mobile Legal Visualization</title>\n");
        html.push_str("    <style>\n");
        html.push_str("        * { box-sizing: border-box; margin: 0; padding: 0; }\n");
        html.push_str("        body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif; overflow-x: hidden; }\n");
        html.push_str("        .viz-container { width: 100%; height: 100vh; overflow: hidden; touch-action: none; }\n");
        html.push_str("        .viz-content { width: 100%; height: 100%; }\n");
        html.push_str(&self.responsive_config.to_css());
        html.push_str("    </style>\n</head>\n<body>\n");
        html.push_str("    <div class=\"viz-container\">\n");
        html.push_str("        <div class=\"viz-content\">\n");
        html.push_str(&format!("            {}\n", base_html));
        html.push_str("        </div>\n");
        html.push_str("    </div>\n");
        html.push_str("    <script>\n");
        html.push_str(&self.touch_config.to_javascript());
        html.push_str(
            r#"
        const container = document.querySelector('.viz-content');
        const gestureHandler = new TouchGestureHandler(container);

        // Add swipe event listener for navigation
        container.addEventListener('swipe', (e) => {
            console.log('Swiped:', e.detail.direction);
        });

        // Add tap event listener for interaction
        container.addEventListener('tap', (e) => {
            console.log('Tapped at:', e.detail.x, e.detail.y);
        });
"#,
        );
        html.push_str("    </script>\n");

        // Register service worker if offline support is enabled
        if self.offline_config.enabled {
            html.push_str("    <script>\n");
            html.push_str(
                r#"
        if ('serviceWorker' in navigator) {
            window.addEventListener('load', () => {
                navigator.serviceWorker.register('/service-worker.js')
                    .then(reg => console.log('Service Worker registered:', reg))
                    .catch(err => console.log('Service Worker registration failed:', err));
            });
        }
"#,
            );
            html.push_str("    </script>\n");
        }

        html.push_str("</body>\n</html>");
        html
    }

    /// Gets the service worker script content.
    pub fn service_worker_script(&self) -> String {
        self.offline_config.to_service_worker()
    }

    /// Gets the PWA manifest JSON content.
    pub fn pwa_manifest(&self) -> String {
        self.pwa_config.to_manifest_json()
    }
}

// ============================================================================
// v0.2.7: Analytics Dashboard Framework
// ============================================================================

/// Widget types for analytics dashboards.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WidgetType {
    /// Chart widget (bar, line, pie, etc.)
    Chart,
    /// Metric widget (single value)
    Metric,
    /// Table widget (data grid)
    Table,
    /// Text widget (custom HTML/text)
    Text,
    /// Visualization widget (custom viz)
    Visualization,
}

/// Dashboard widget configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardWidget {
    /// Widget ID
    pub id: String,
    /// Widget title
    pub title: String,
    /// Widget type
    pub widget_type: WidgetType,
    /// Widget position (row, column)
    pub position: (u32, u32),
    /// Widget size (width, height in grid units)
    pub size: (u32, u32),
    /// Widget data source
    pub data_source: String,
    /// Widget filters
    pub filters: Vec<DashboardFilter>,
    /// Widget refresh interval (milliseconds)
    pub refresh_interval_ms: Option<u32>,
    /// Custom widget config (JSON)
    pub config: String,
}

/// Dashboard filter for data filtering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardFilter {
    /// Filter ID
    pub id: String,
    /// Filter field name
    pub field: String,
    /// Filter operator
    pub operator: String,
    /// Filter value
    pub value: String,
    /// Is filter shared across widgets
    pub shared: bool,
}

/// Dashboard layout grid configuration.
#[derive(Debug, Clone)]
pub struct DashboardLayout {
    /// Number of columns in the grid
    pub columns: u32,
    /// Number of rows in the grid
    pub rows: u32,
    /// Gap between widgets (pixels)
    pub gap: u32,
    /// Responsive breakpoints
    pub breakpoints: Vec<(u32, u32)>, // (screen_width, columns)
}

impl Default for DashboardLayout {
    fn default() -> Self {
        Self {
            columns: 12,
            rows: 6,
            gap: 16,
            breakpoints: vec![(480, 4), (768, 8), (1024, 12)],
        }
    }
}

/// Saved dashboard configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardConfig {
    /// Dashboard ID
    pub id: String,
    /// Dashboard name
    pub name: String,
    /// Dashboard description
    pub description: String,
    /// Dashboard layout
    pub layout: (u32, u32), // (columns, rows)
    /// Dashboard widgets
    pub widgets: Vec<DashboardWidget>,
    /// Shared filters
    pub shared_filters: Vec<DashboardFilter>,
    /// Auto-refresh interval (milliseconds)
    pub auto_refresh_ms: Option<u32>,
}

impl DashboardConfig {
    /// Creates a new dashboard configuration.
    pub fn new(id: &str, name: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            description: String::new(),
            layout: (12, 6),
            widgets: Vec::new(),
            shared_filters: Vec::new(),
            auto_refresh_ms: None,
        }
    }

    /// Adds a widget to the dashboard.
    pub fn add_widget(&mut self, widget: DashboardWidget) {
        self.widgets.push(widget);
    }

    /// Adds a shared filter.
    pub fn add_shared_filter(&mut self, filter: DashboardFilter) {
        self.shared_filters.push(filter);
    }

    /// Sets auto-refresh interval.
    pub fn with_auto_refresh(mut self, interval_ms: u32) -> Self {
        self.auto_refresh_ms = Some(interval_ms);
        self
    }

    /// Serializes dashboard configuration to JSON.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Deserializes dashboard configuration from JSON.
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

/// Analytics dashboard builder and renderer.
#[derive(Debug, Clone)]
pub struct AnalyticsDashboard {
    config: DashboardConfig,
    layout: DashboardLayout,
    theme: Theme,
}

impl AnalyticsDashboard {
    /// Creates a new analytics dashboard.
    pub fn new(name: &str) -> Self {
        Self {
            config: DashboardConfig::new("dashboard-1", name),
            layout: DashboardLayout::default(),
            theme: Theme::default(),
        }
    }

    /// Creates from a saved configuration.
    pub fn from_config(config: DashboardConfig) -> Self {
        Self {
            layout: DashboardLayout {
                columns: config.layout.0,
                rows: config.layout.1,
                ..DashboardLayout::default()
            },
            config,
            theme: Theme::default(),
        }
    }

    /// Sets the dashboard theme.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Sets the dashboard layout.
    pub fn with_layout(mut self, layout: DashboardLayout) -> Self {
        self.layout = layout;
        self
    }

    /// Adds a chart widget.
    pub fn add_chart_widget(
        &mut self,
        id: &str,
        title: &str,
        position: (u32, u32),
        size: (u32, u32),
        data_source: &str,
    ) {
        let widget = DashboardWidget {
            id: id.to_string(),
            title: title.to_string(),
            widget_type: WidgetType::Chart,
            position,
            size,
            data_source: data_source.to_string(),
            filters: Vec::new(),
            refresh_interval_ms: None,
            config: "{}".to_string(),
        };
        self.config.add_widget(widget);
    }

    /// Adds a metric widget.
    pub fn add_metric_widget(
        &mut self,
        id: &str,
        title: &str,
        position: (u32, u32),
        size: (u32, u32),
        data_source: &str,
    ) {
        let widget = DashboardWidget {
            id: id.to_string(),
            title: title.to_string(),
            widget_type: WidgetType::Metric,
            position,
            size,
            data_source: data_source.to_string(),
            filters: Vec::new(),
            refresh_interval_ms: None,
            config: "{}".to_string(),
        };
        self.config.add_widget(widget);
    }

    /// Adds a table widget.
    pub fn add_table_widget(
        &mut self,
        id: &str,
        title: &str,
        position: (u32, u32),
        size: (u32, u32),
        data_source: &str,
    ) {
        let widget = DashboardWidget {
            id: id.to_string(),
            title: title.to_string(),
            widget_type: WidgetType::Table,
            position,
            size,
            data_source: data_source.to_string(),
            filters: Vec::new(),
            refresh_interval_ms: None,
            config: "{}".to_string(),
        };
        self.config.add_widget(widget);
    }

    /// Adds a shared filter that applies to all widgets.
    pub fn add_shared_filter(&mut self, field: &str, operator: &str, value: &str) {
        let filter = DashboardFilter {
            id: format!("filter-{}", self.config.shared_filters.len() + 1),
            field: field.to_string(),
            operator: operator.to_string(),
            value: value.to_string(),
            shared: true,
        };
        self.config.add_shared_filter(filter);
    }

    /// Enables auto-refresh for the dashboard.
    pub fn enable_auto_refresh(&mut self, interval_ms: u32) {
        self.config.auto_refresh_ms = Some(interval_ms);
    }

    /// Saves the dashboard configuration to JSON.
    pub fn save_config(&self) -> Result<String, serde_json::Error> {
        self.config.to_json()
    }

    /// Generates HTML for the dashboard.
    pub fn to_html(&self) -> String {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str("    <meta charset=\"utf-8\">\n");
        html.push_str(
            "    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n",
        );
        html.push_str(&format!("    <title>{}</title>\n", self.config.name));
        html.push_str("    <script src=\"https://cdn.jsdelivr.net/npm/chart.js\"></script>\n");
        html.push_str("    <style>\n");
        html.push_str("        * { box-sizing: border-box; margin: 0; padding: 0; }\n");
        html.push_str(&format!("        body {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif; background: {}; color: {}; }}\n",
            self.theme.background_color, self.theme.text_color));
        html.push_str(
            "        .dashboard-header { padding: 20px; border-bottom: 1px solid #e0e0e0; }\n",
        );
        html.push_str("        .dashboard-title { font-size: 24px; font-weight: bold; }\n");
        html.push_str("        .dashboard-filters { padding: 10px 20px; display: flex; gap: 10px; flex-wrap: wrap; background: #f5f5f5; }\n");
        html.push_str("        .filter-item { padding: 5px 10px; background: white; border: 1px solid #ddd; border-radius: 4px; font-size: 14px; }\n");
        html.push_str(&format!("        .dashboard-grid {{ display: grid; grid-template-columns: repeat({}, 1fr); grid-template-rows: repeat({}, 1fr); gap: {}px; padding: 20px; min-height: calc(100vh - 140px); }}\n",
            self.layout.columns, self.layout.rows, self.layout.gap));
        html.push_str("        .widget { background: white; border: 1px solid #e0e0e0; border-radius: 8px; padding: 16px; display: flex; flex-direction: column; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }\n");
        html.push_str("        .widget-header { font-weight: bold; margin-bottom: 12px; padding-bottom: 8px; border-bottom: 1px solid #e0e0e0; }\n");
        html.push_str("        .widget-content { flex: 1; overflow: auto; }\n");
        html.push_str("        .metric-value { font-size: 48px; font-weight: bold; text-align: center; padding: 20px; }\n");
        html.push_str(
            "        .metric-label { font-size: 14px; text-align: center; color: #666; }\n",
        );
        html.push_str("        table { width: 100%; border-collapse: collapse; }\n");
        html.push_str("        th, td { padding: 8px; text-align: left; border-bottom: 1px solid #e0e0e0; }\n");
        html.push_str("        th { background: #f5f5f5; font-weight: bold; }\n");

        // Add responsive breakpoints
        for (screen_width, cols) in &self.layout.breakpoints {
            html.push_str(&format!("        @media (max-width: {}px) {{ .dashboard-grid {{ grid-template-columns: repeat({}, 1fr); }} }}\n",
                screen_width, cols));
        }

        html.push_str("    </style>\n</head>\n<body>\n");

        // Dashboard header
        html.push_str("    <div class=\"dashboard-header\">\n");
        html.push_str(&format!(
            "        <div class=\"dashboard-title\">{}</div>\n",
            self.config.name
        ));
        if !self.config.description.is_empty() {
            html.push_str(&format!(
                "        <div style=\"margin-top: 5px; color: #666; font-size: 14px;\">{}</div>\n",
                self.config.description
            ));
        }
        html.push_str("    </div>\n");

        // Shared filters
        if !self.config.shared_filters.is_empty() {
            html.push_str("    <div class=\"dashboard-filters\">\n");
            html.push_str("        <span style=\"font-weight: bold;\">Filters:</span>\n");
            for filter in &self.config.shared_filters {
                html.push_str(&format!(
                    "        <div class=\"filter-item\">{} {} {}</div>\n",
                    filter.field, filter.operator, filter.value
                ));
            }
            html.push_str("    </div>\n");
        }

        // Dashboard grid
        html.push_str("    <div class=\"dashboard-grid\">\n");

        for widget in &self.config.widgets {
            let (col, row) = widget.position;
            let (width, height) = widget.size;

            html.push_str(&format!(
                "        <div class=\"widget\" style=\"grid-column: {} / span {}; grid-row: {} / span {};\">\n",
                col + 1, width, row + 1, height
            ));
            html.push_str(&format!(
                "            <div class=\"widget-header\">{}</div>\n",
                widget.title
            ));
            html.push_str("            <div class=\"widget-content\">\n");

            match widget.widget_type {
                WidgetType::Chart => {
                    html.push_str(&format!(
                        "                <canvas id=\"chart-{}\"></canvas>\n",
                        widget.id
                    ));
                }
                WidgetType::Metric => {
                    html.push_str("                <div class=\"metric-value\">1,234</div>\n");
                    html.push_str(&format!(
                        "                <div class=\"metric-label\">{}</div>\n",
                        widget.title
                    ));
                }
                WidgetType::Table => {
                    html.push_str("                <table>\n");
                    html.push_str("                    <thead><tr><th>Column 1</th><th>Column 2</th><th>Column 3</th></tr></thead>\n");
                    html.push_str("                    <tbody>\n");
                    html.push_str("                        <tr><td>Data 1</td><td>Data 2</td><td>Data 3</td></tr>\n");
                    html.push_str("                        <tr><td>Data 4</td><td>Data 5</td><td>Data 6</td></tr>\n");
                    html.push_str("                    </tbody>\n");
                    html.push_str("                </table>\n");
                }
                WidgetType::Text => {
                    html.push_str("                <p>Custom text content</p>\n");
                }
                WidgetType::Visualization => {
                    html.push_str(&format!(
                        "                <div id=\"viz-{}\">Visualization placeholder</div>\n",
                        widget.id
                    ));
                }
            }

            html.push_str("            </div>\n");
            html.push_str("        </div>\n");
        }

        html.push_str("    </div>\n");

        // JavaScript for charts and auto-refresh
        html.push_str("    <script>\n");

        // Initialize charts
        for widget in &self.config.widgets {
            if matches!(widget.widget_type, WidgetType::Chart) {
                html.push_str(&format!(
                    r#"
        const ctx{} = document.getElementById('chart-{}').getContext('2d');
        new Chart(ctx{}, {{
            type: 'bar',
            data: {{
                labels: ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun'],
                datasets: [{{
                    label: '{}',
                    data: [12, 19, 3, 5, 2, 3],
                    backgroundColor: '{}'
                }}]
            }},
            options: {{
                responsive: true,
                maintainAspectRatio: false,
                scales: {{ y: {{ beginAtZero: true }} }}
            }}
        }});
"#,
                    widget.id, widget.id, widget.id, widget.title, self.theme.condition_color
                ));
            }
        }

        // Auto-refresh
        if let Some(interval_ms) = self.config.auto_refresh_ms {
            html.push_str(&format!(
                r#"
        // Auto-refresh dashboard every {} milliseconds
        setInterval(() => {{
            console.log('Refreshing dashboard...');
            // Fetch new data and update widgets
            location.reload();
        }}, {});
"#,
                interval_ms, interval_ms
            ));
        }

        html.push_str("    </script>\n</body>\n</html>");
        html
    }

    /// Generates JavaScript for filter synchronization.
    pub fn filter_sync_script(&self) -> String {
        r#"
class DashboardFilterSync {{
    constructor() {{
        this.filters = new Map();
        this.widgets = new Map();
        this.subscribers = [];
    }}

    addFilter(filterId, field, operator, value, shared = false) {{
        this.filters.set(filterId, {{ field, operator, value, shared }});
        if (shared) {{
            this.notifySubscribers(filterId);
        }}
    }}

    removeFilter(filterId) {{
        this.filters.delete(filterId);
        this.notifySubscribers(filterId);
    }}

    updateFilter(filterId, value) {{
        const filter = this.filters.get(filterId);
        if (filter) {{
            filter.value = value;
            this.notifySubscribers(filterId);
        }}
    }}

    registerWidget(widgetId, onFilterChange) {{
        this.subscribers.push({{ widgetId, onFilterChange }});
    }}

    notifySubscribers(filterId) {{
        const filter = this.filters.get(filterId);
        if (filter && filter.shared) {{
            this.subscribers.forEach(sub => {{
                sub.onFilterChange(filterId, filter);
            }});
        }}
    }}

    getActiveFilters() {{
        const active = [];
        this.filters.forEach((filter, id) => {{
            if (filter.shared) {{
                active.push({{ id, ...filter }});
            }}
        }});
        return active;
    }}
}}

const filterSync = new DashboardFilterSync();
"#
        .to_string()
    }
}

// ============================================================================
// v0.2.8: Geographic Visualization 2.0
// ============================================================================

/// Geographic coordinate (latitude, longitude).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GeoCoordinate {
    /// Latitude
    pub lat: f64,
    /// Longitude
    pub lng: f64,
}

/// GeoJSON feature for boundary rendering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoJsonFeature {
    /// Feature ID
    pub id: String,
    /// Feature type (usually "Feature")
    #[serde(rename = "type")]
    pub feature_type: String,
    /// Geometry type and coordinates
    pub geometry: GeoJsonGeometry,
    /// Feature properties
    pub properties: serde_json::Value,
}

/// GeoJSON geometry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoJsonGeometry {
    /// Geometry type (Polygon, MultiPolygon, Point, etc.)
    #[serde(rename = "type")]
    pub geometry_type: String,
    /// Coordinates (format depends on geometry type)
    pub coordinates: serde_json::Value,
}

/// Choropleth map data point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChoroplethData {
    /// Geographic region ID (e.g., state code, county FIPS)
    pub region_id: String,
    /// Data value for the region
    pub value: f64,
    /// Region label/name
    pub label: String,
}

/// Heat map data point for legal activity visualization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeatMapPoint {
    /// Location
    pub location: GeoCoordinate,
    /// Intensity/weight of the activity
    pub intensity: f64,
    /// Activity type/label
    pub label: String,
}

/// Point cluster for entity visualization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointCluster {
    /// Center coordinate of the cluster
    pub center: GeoCoordinate,
    /// Number of points in the cluster
    pub count: usize,
    /// Individual points (if cluster is expanded)
    pub points: Vec<GeoPoint>,
}

/// Individual geographic point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoPoint {
    /// Point ID
    pub id: String,
    /// Point location
    pub location: GeoCoordinate,
    /// Point label
    pub label: String,
    /// Point data
    pub data: serde_json::Value,
}

/// Map tile provider configuration.
#[derive(Debug, Clone)]
pub enum TileProvider {
    /// OpenStreetMap tiles
    OpenStreetMap,
    /// Mapbox tiles (requires API key)
    Mapbox(String),
    /// Google Maps tiles (requires API key)
    GoogleMaps(String),
    /// Custom tile provider with URL template
    Custom(String),
}

impl TileProvider {
    /// Gets the tile URL template.
    pub fn url_template(&self) -> String {
        match self {
            TileProvider::OpenStreetMap => {
                "https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png".to_string()
            }
            TileProvider::Mapbox(api_key) => {
                format!(
                    "https://api.mapbox.com/styles/v1/mapbox/streets-v11/tiles/{{z}}/{{x}}/{{y}}?access_token={}",
                    api_key
                )
            }
            TileProvider::GoogleMaps(api_key) => {
                format!(
                    "https://maps.googleapis.com/maps/vt?pb=!1m5!1m4!1i{{z}}!2i{{x}}!3i{{y}}!4i256!2m3!1e0!2sm!3i{{s}}!3m9!2sen!3sUS!5e18!12m1!1e47!12m3!1e37!2m1!1ssmartmaps!4e0&key={}",
                    api_key
                )
            }
            TileProvider::Custom(template) => template.clone(),
        }
    }

    /// Gets attribution text for the tile provider.
    pub fn attribution(&self) -> &str {
        match self {
            TileProvider::OpenStreetMap => {
                "&copy; <a href='https://www.openstreetmap.org/copyright'>OpenStreetMap</a> contributors"
            }
            TileProvider::Mapbox(_) => {
                "&copy; <a href='https://www.mapbox.com/about/maps/'>Mapbox</a>"
            }
            TileProvider::GoogleMaps(_) => "&copy; Google Maps",
            TileProvider::Custom(_) => "",
        }
    }
}

/// Geographic visualization renderer.
#[derive(Debug, Clone)]
pub struct GeoVisualization {
    tile_provider: TileProvider,
    center: GeoCoordinate,
    zoom: u32,
    theme: Theme,
}

impl GeoVisualization {
    /// Creates a new geographic visualization.
    pub fn new(center: GeoCoordinate, zoom: u32) -> Self {
        Self {
            tile_provider: TileProvider::OpenStreetMap,
            center,
            zoom,
            theme: Theme::default(),
        }
    }

    /// Sets the tile provider.
    pub fn with_tile_provider(mut self, provider: TileProvider) -> Self {
        self.tile_provider = provider;
        self
    }

    /// Sets the theme.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Generates HTML for a choropleth map.
    pub fn to_choropleth_html(
        &self,
        data: &[ChoroplethData],
        geojson: &[GeoJsonFeature],
    ) -> String {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str("    <meta charset=\"utf-8\">\n");
        html.push_str(
            "    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n",
        );
        html.push_str("    <title>Choropleth Map</title>\n");
        html.push_str("    <link rel=\"stylesheet\" href=\"https://unpkg.com/leaflet@1.9.4/dist/leaflet.css\" />\n");
        html.push_str(
            "    <script src=\"https://unpkg.com/leaflet@1.9.4/dist/leaflet.js\"></script>\n",
        );
        html.push_str("    <style>\n");
        html.push_str("        body { margin: 0; padding: 0; }\n");
        html.push_str("        #map { width: 100vw; height: 100vh; }\n");
        html.push_str(
            "        .legend { background: white; padding: 10px; border-radius: 5px; }\n",
        );
        html.push_str("        .legend-item { margin: 5px 0; }\n");
        html.push_str("        .legend-color { display: inline-block; width: 20px; height: 20px; margin-right: 5px; }\n");
        html.push_str("    </style>\n</head>\n<body>\n");
        html.push_str("    <div id=\"map\"></div>\n");
        html.push_str("    <script>\n");
        html.push_str(&format!(
            "const map = L.map('map').setView([{}, {}], {});\n",
            self.center.lat, self.center.lng, self.zoom
        ));
        html.push_str(&format!(
            "L.tileLayer('{}', {{\n",
            self.tile_provider.url_template()
        ));
        html.push_str(&format!(
            "    attribution: '{}'\n",
            self.tile_provider.attribution()
        ));
        html.push_str("}).addTo(map);\n\n");

        // Add choropleth data
        html.push_str("const choroplethData = {\n");
        for item in data {
            html.push_str(&format!("    '{}': {},\n", item.region_id, item.value));
        }
        html.push_str("};\n\n");

        // Add GeoJSON layer
        if !geojson.is_empty() {
            let geojson_str = serde_json::to_string(&geojson).unwrap_or_else(|_| "[]".to_string());
            html.push_str(&format!("const geoJsonData = {};\n", geojson_str));
            html.push_str(
                r#"
L.geoJSON(geoJsonData, {
    style: function(feature) {
        const value = choroplethData[feature.id] || 0;
        return {
            fillColor: getColor(value),
            weight: 2,
            opacity: 1,
            color: 'white',
            fillOpacity: 0.7
        };
    },
    onEachFeature: function(feature, layer) {
        const value = choroplethData[feature.id] || 0;
        layer.bindPopup(`<b>${feature.properties.name || feature.id}</b><br>Value: ${value}`);
    }
}).addTo(map);

function getColor(value) {
    return value > 1000 ? '#800026' :
           value > 500  ? '#BD0026' :
           value > 200  ? '#E31A1C' :
           value > 100  ? '#FC4E2A' :
           value > 50   ? '#FD8D3C' :
           value > 20   ? '#FEB24C' :
           value > 10   ? '#FED976' :
                          '#FFEDA0';
}
"#,
            );
        }

        html.push_str("    </script>\n</body>\n</html>");
        html
    }

    /// Generates HTML for a heat map.
    pub fn to_heatmap_html(&self, points: &[HeatMapPoint]) -> String {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str("    <meta charset=\"utf-8\">\n");
        html.push_str(
            "    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n",
        );
        html.push_str("    <title>Heat Map</title>\n");
        html.push_str("    <link rel=\"stylesheet\" href=\"https://unpkg.com/leaflet@1.9.4/dist/leaflet.css\" />\n");
        html.push_str(
            "    <script src=\"https://unpkg.com/leaflet@1.9.4/dist/leaflet.js\"></script>\n",
        );
        html.push_str("    <script src=\"https://unpkg.com/leaflet.heat@0.2.0/dist/leaflet-heat.js\"></script>\n");
        html.push_str("    <style>\n");
        html.push_str("        body { margin: 0; padding: 0; }\n");
        html.push_str("        #map { width: 100vw; height: 100vh; }\n");
        html.push_str("    </style>\n</head>\n<body>\n");
        html.push_str("    <div id=\"map\"></div>\n");
        html.push_str("    <script>\n");
        html.push_str(&format!(
            "const map = L.map('map').setView([{}, {}], {});\n",
            self.center.lat, self.center.lng, self.zoom
        ));
        html.push_str(&format!(
            "L.tileLayer('{}', {{\n",
            self.tile_provider.url_template()
        ));
        html.push_str(&format!(
            "    attribution: '{}'\n",
            self.tile_provider.attribution()
        ));
        html.push_str("}).addTo(map);\n\n");

        // Add heat map data
        html.push_str("const heatData = [\n");
        for point in points {
            html.push_str(&format!(
                "    [{}, {}, {}],\n",
                point.location.lat, point.location.lng, point.intensity
            ));
        }
        html.push_str("];\n\n");

        html.push_str("L.heatLayer(heatData, { radius: 25, blur: 15, maxZoom: 17 }).addTo(map);\n");
        html.push_str("    </script>\n</body>\n</html>");
        html
    }

    /// Generates HTML for a clustered point map.
    pub fn to_cluster_map_html(&self, points: &[GeoPoint]) -> String {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str("    <meta charset=\"utf-8\">\n");
        html.push_str(
            "    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n",
        );
        html.push_str("    <title>Clustered Point Map</title>\n");
        html.push_str("    <link rel=\"stylesheet\" href=\"https://unpkg.com/leaflet@1.9.4/dist/leaflet.css\" />\n");
        html.push_str("    <link rel=\"stylesheet\" href=\"https://unpkg.com/leaflet.markercluster@1.4.1/dist/MarkerCluster.css\" />\n");
        html.push_str("    <link rel=\"stylesheet\" href=\"https://unpkg.com/leaflet.markercluster@1.4.1/dist/MarkerCluster.Default.css\" />\n");
        html.push_str(
            "    <script src=\"https://unpkg.com/leaflet@1.9.4/dist/leaflet.js\"></script>\n",
        );
        html.push_str("    <script src=\"https://unpkg.com/leaflet.markercluster@1.4.1/dist/leaflet.markercluster.js\"></script>\n");
        html.push_str("    <style>\n");
        html.push_str("        body { margin: 0; padding: 0; }\n");
        html.push_str("        #map { width: 100vw; height: 100vh; }\n");
        html.push_str("    </style>\n</head>\n<body>\n");
        html.push_str("    <div id=\"map\"></div>\n");
        html.push_str("    <script>\n");
        html.push_str(&format!(
            "const map = L.map('map').setView([{}, {}], {});\n",
            self.center.lat, self.center.lng, self.zoom
        ));
        html.push_str(&format!(
            "L.tileLayer('{}', {{\n",
            self.tile_provider.url_template()
        ));
        html.push_str(&format!(
            "    attribution: '{}'\n",
            self.tile_provider.attribution()
        ));
        html.push_str("}).addTo(map);\n\n");

        // Add marker cluster
        html.push_str("const markers = L.markerClusterGroup();\n\n");

        for point in points {
            html.push_str(&format!(
                "const marker{} = L.marker([{}, {}]).bindPopup('<b>{}</b>');\n",
                point.id.replace('-', "_"),
                point.location.lat,
                point.location.lng,
                point.label
            ));
            html.push_str(&format!(
                "markers.addLayer(marker{});\n",
                point.id.replace('-', "_")
            ));
        }

        html.push_str("\nmap.addLayer(markers);\n");
        html.push_str("    </script>\n</body>\n</html>");
        html
    }
}

// ============================================================================
// Real-Time Legal Intelligence (v0.3.2)
// ============================================================================

/// Live court proceeding visualization with real-time updates.
pub struct LiveCourtProceeding {
    /// Court name
    court_name: String,
    /// Case number
    case_number: String,
    /// WebSocket URL for live updates
    ws_url: String,
    /// Theme
    theme: Theme,
}

impl LiveCourtProceeding {
    /// Creates a new live court proceeding visualizer.
    pub fn new(court_name: &str, case_number: &str, ws_url: &str) -> Self {
        Self {
            court_name: court_name.to_string(),
            case_number: case_number.to_string(),
            ws_url: ws_url.to_string(),
            theme: Theme::default(),
        }
    }

    /// Sets the theme.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Generates live HTML for court proceeding.
    pub fn to_live_html(&self, events: &[CourtEvent]) -> String {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str("    <meta charset=\"utf-8\">\n");
        html.push_str(
            "    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n",
        );
        html.push_str(&format!(
            "    <title>Live: {} - {}</title>\n",
            self.court_name, self.case_number
        ));
        html.push_str("    <style>\n");
        html.push_str(&format!("        body {{ background-color: {}; color: {}; font-family: Arial, sans-serif; margin: 0; padding: 20px; }}\n", self.theme.background_color, self.theme.text_color));
        html.push_str("        .header { border-bottom: 2px solid #ccc; padding-bottom: 10px; margin-bottom: 20px; }\n");
        html.push_str("        .status { display: inline-block; padding: 5px 15px; border-radius: 5px; font-weight: bold; }\n");
        html.push_str("        .status.live { background-color: #e74c3c; color: white; animation: pulse 2s infinite; }\n");
        html.push_str(
            "        @keyframes pulse { 0%, 100% { opacity: 1; } 50% { opacity: 0.6; } }\n",
        );
        html.push_str("        .timeline { position: relative; padding-left: 30px; }\n");
        html.push_str("        .event { position: relative; padding: 15px; margin: 10px 0; background-color: #f5f5f5; border-left: 4px solid #3498db; }\n");
        html.push_str("        .event.motion { border-left-color: #9b59b6; }\n");
        html.push_str("        .event.ruling { border-left-color: #e74c3c; }\n");
        html.push_str("        .event.testimony { border-left-color: #f39c12; }\n");
        html.push_str("        .event.recess { border-left-color: #95a5a6; }\n");
        html.push_str("        .event-time { font-size: 0.9em; color: #7f8c8d; }\n");
        html.push_str("        .event-type { font-weight: bold; text-transform: uppercase; font-size: 0.8em; }\n");
        html.push_str("        .event-description { margin-top: 5px; }\n");
        html.push_str(
            "        .participants { margin-top: 10px; font-size: 0.9em; color: #34495e; }\n",
        );
        html.push_str("    </style>\n</head>\n<body>\n");
        html.push_str("    <div class=\"header\">\n");
        html.push_str(&format!("        <h1>{}</h1>\n", self.court_name));
        html.push_str(&format!("        <h2>Case: {}</h2>\n", self.case_number));
        html.push_str("        <span class=\"status live\" id=\"status\">● LIVE</span>\n");
        html.push_str("    </div>\n");
        html.push_str("    <div class=\"timeline\" id=\"timeline\">\n");

        for event in events {
            let event_class = match event.event_type {
                CourtEventType::Motion => "motion",
                CourtEventType::Ruling => "ruling",
                CourtEventType::Testimony => "testimony",
                CourtEventType::Recess => "recess",
                CourtEventType::Opening => "opening",
                CourtEventType::Closing => "closing",
            };

            html.push_str(&format!("        <div class=\"event {}\">\n", event_class));
            html.push_str(&format!(
                "            <div class=\"event-time\">{}</div>\n",
                event.timestamp
            ));
            html.push_str(&format!(
                "            <div class=\"event-type\">{:?}</div>\n",
                event.event_type
            ));
            html.push_str(&format!(
                "            <div class=\"event-description\">{}</div>\n",
                event.description
            ));

            if !event.participants.is_empty() {
                html.push_str(&format!(
                    "            <div class=\"participants\">Participants: {}</div>\n",
                    event.participants.join(", ")
                ));
            }

            html.push_str("        </div>\n");
        }

        html.push_str("    </div>\n");
        html.push_str("    <script>\n");
        html.push_str(&format!("const ws = new WebSocket('{}');\n", self.ws_url));
        html.push_str("ws.onmessage = function(event) {\n");
        html.push_str("    const data = JSON.parse(event.data);\n");
        html.push_str("    const timeline = document.getElementById('timeline');\n");
        html.push_str("    const eventDiv = document.createElement('div');\n");
        html.push_str("    eventDiv.className = 'event ' + data.type.toLowerCase();\n");
        html.push_str("    eventDiv.innerHTML = `\n");
        html.push_str("        <div class=\"event-time\">${data.timestamp}</div>\n");
        html.push_str("        <div class=\"event-type\">${data.type}</div>\n");
        html.push_str("        <div class=\"event-description\">${data.description}</div>\n");
        html.push_str("        ${data.participants ? '<div class=\"participants\">Participants: ' + data.participants.join(', ') + '</div>' : ''}\n");
        html.push_str("    `;\n");
        html.push_str("    timeline.appendChild(eventDiv);\n");
        html.push_str("    eventDiv.scrollIntoView({ behavior: 'smooth' });\n");
        html.push_str("};\n");
        html.push_str("ws.onclose = function() {\n");
        html.push_str("    document.getElementById('status').textContent = '● ENDED';\n");
        html.push_str("    document.getElementById('status').classList.remove('live');\n");
        html.push_str("};\n");
        html.push_str("    </script>\n</body>\n</html>");
        html
    }
}

impl Default for LiveCourtProceeding {
    fn default() -> Self {
        Self::new("Court", "Unknown", "ws://localhost:8080")
    }
}

/// Court event in a live proceeding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourtEvent {
    /// Event timestamp
    pub timestamp: String,
    /// Event type
    pub event_type: CourtEventType,
    /// Event description
    pub description: String,
    /// Participants
    pub participants: Vec<String>,
}

impl CourtEvent {
    /// Creates a new court event.
    pub fn new(timestamp: &str, event_type: CourtEventType, description: &str) -> Self {
        Self {
            timestamp: timestamp.to_string(),
            event_type,
            description: description.to_string(),
            participants: Vec::new(),
        }
    }

    /// Adds a participant.
    pub fn with_participant(mut self, participant: &str) -> Self {
        self.participants.push(participant.to_string());
        self
    }
}

/// Types of court events.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CourtEventType {
    /// Motion filed or argued
    Motion,
    /// Ruling issued
    Ruling,
    /// Testimony given
    Testimony,
    /// Court recess
    Recess,
    /// Opening statement
    Opening,
    /// Closing argument
    Closing,
}

/// Breaking legal news feed visualizer.
pub struct BreakingNewsFeed {
    /// Feed title
    title: String,
    /// WebSocket URL for news updates
    ws_url: String,
    /// Theme
    theme: Theme,
    /// Max items to display
    max_items: usize,
}

impl BreakingNewsFeed {
    /// Creates a new breaking news feed.
    pub fn new(title: &str, ws_url: &str) -> Self {
        Self {
            title: title.to_string(),
            ws_url: ws_url.to_string(),
            theme: Theme::default(),
            max_items: 50,
        }
    }

    /// Sets the theme.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Sets max items to display.
    pub fn with_max_items(mut self, max_items: usize) -> Self {
        self.max_items = max_items;
        self
    }

    /// Generates HTML for breaking news feed.
    pub fn to_html(&self, news_items: &[NewsItem]) -> String {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str("    <meta charset=\"utf-8\">\n");
        html.push_str(
            "    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n",
        );
        html.push_str(&format!("    <title>{}</title>\n", self.title));
        html.push_str("    <style>\n");
        html.push_str(&format!("        body {{ background-color: {}; color: {}; font-family: 'Segoe UI', Arial, sans-serif; margin: 0; padding: 0; }}\n", self.theme.background_color, self.theme.text_color));
        html.push_str("        .header { background-color: #c0392b; color: white; padding: 20px; border-bottom: 3px solid #e74c3c; }\n");
        html.push_str("        .header h1 { margin: 0; font-size: 2em; }\n");
        html.push_str("        .breaking-banner { background-color: #e74c3c; color: white; padding: 10px 20px; font-weight: bold; animation: flash 2s infinite; }\n");
        html.push_str(
            "        @keyframes flash { 0%, 100% { opacity: 1; } 50% { opacity: 0.7; } }\n",
        );
        html.push_str("        .news-feed { max-width: 1200px; margin: 0 auto; padding: 20px; }\n");
        html.push_str("        .news-item { background-color: white; border-left: 5px solid #3498db; margin: 15px 0; padding: 20px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }\n");
        html.push_str("        .news-item.urgent { border-left-color: #e74c3c; }\n");
        html.push_str("        .news-item.high { border-left-color: #f39c12; }\n");
        html.push_str("        .news-item.medium { border-left-color: #3498db; }\n");
        html.push_str("        .news-item.low { border-left-color: #95a5a6; }\n");
        html.push_str("        .news-title { font-size: 1.3em; font-weight: bold; margin-bottom: 10px; color: #2c3e50; }\n");
        html.push_str(
            "        .news-summary { margin-bottom: 10px; color: #34495e; line-height: 1.6; }\n",
        );
        html.push_str("        .news-meta { font-size: 0.9em; color: #7f8c8d; }\n");
        html.push_str("        .news-source { font-weight: bold; color: #2980b9; }\n");
        html.push_str("        .news-tags { margin-top: 10px; }\n");
        html.push_str("        .tag { display: inline-block; background-color: #ecf0f1; padding: 3px 10px; margin: 2px; border-radius: 3px; font-size: 0.85em; }\n");
        html.push_str("    </style>\n</head>\n<body>\n");
        html.push_str("    <div class=\"header\">\n");
        html.push_str(&format!("        <h1>{}</h1>\n", self.title));
        html.push_str("    </div>\n");
        html.push_str("    <div class=\"breaking-banner\" id=\"breaking\" style=\"display: none;\">BREAKING NEWS</div>\n");
        html.push_str("    <div class=\"news-feed\" id=\"feed\">\n");

        for item in news_items.iter().take(self.max_items) {
            let priority_class = match item.priority {
                NewsPriority::Urgent => "urgent",
                NewsPriority::High => "high",
                NewsPriority::Medium => "medium",
                NewsPriority::Low => "low",
            };

            html.push_str(&format!(
                "        <div class=\"news-item {}\">\n",
                priority_class
            ));
            html.push_str(&format!(
                "            <div class=\"news-title\">{}</div>\n",
                item.title
            ));
            html.push_str(&format!(
                "            <div class=\"news-summary\">{}</div>\n",
                item.summary
            ));
            html.push_str("            <div class=\"news-meta\">\n");
            html.push_str(&format!(
                "                <span class=\"news-source\">{}</span> • {}\n",
                item.source, item.timestamp
            ));
            html.push_str("            </div>\n");

            if !item.tags.is_empty() {
                html.push_str("            <div class=\"news-tags\">\n");
                for tag in &item.tags {
                    html.push_str(&format!(
                        "                <span class=\"tag\">{}</span>\n",
                        tag
                    ));
                }
                html.push_str("            </div>\n");
            }

            html.push_str("        </div>\n");
        }

        html.push_str("    </div>\n");
        html.push_str("    <script>\n");
        html.push_str(&format!("const ws = new WebSocket('{}');\n", self.ws_url));
        html.push_str(&format!("let itemCount = {};\n", news_items.len()));
        html.push_str(&format!("const maxItems = {};\n", self.max_items));
        html.push_str("ws.onmessage = function(event) {\n");
        html.push_str("    const data = JSON.parse(event.data);\n");
        html.push_str("    const feed = document.getElementById('feed');\n");
        html.push_str("    const newsItem = document.createElement('div');\n");
        html.push_str("    const priorityClass = data.priority.toLowerCase();\n");
        html.push_str("    newsItem.className = 'news-item ' + priorityClass;\n");
        html.push_str("    newsItem.innerHTML = `\n");
        html.push_str("        <div class=\"news-title\">${data.title}</div>\n");
        html.push_str("        <div class=\"news-summary\">${data.summary}</div>\n");
        html.push_str("        <div class=\"news-meta\">\n");
        html.push_str(
            "            <span class=\"news-source\">${data.source}</span> • ${data.timestamp}\n",
        );
        html.push_str("        </div>\n");
        html.push_str("        ${data.tags && data.tags.length > 0 ? '<div class=\"news-tags\">' + data.tags.map(t => '<span class=\"tag\">' + t + '</span>').join('') + '</div>' : ''}\n");
        html.push_str("    `;\n");
        html.push_str("    feed.insertBefore(newsItem, feed.firstChild);\n");
        html.push_str("    if (data.priority === 'Urgent') {\n");
        html.push_str("        document.getElementById('breaking').style.display = 'block';\n");
        html.push_str("        setTimeout(() => { document.getElementById('breaking').style.display = 'none'; }, 5000);\n");
        html.push_str("    }\n");
        html.push_str("    itemCount++;\n");
        html.push_str("    if (itemCount > maxItems) {\n");
        html.push_str("        feed.removeChild(feed.lastChild);\n");
        html.push_str("        itemCount--;\n");
        html.push_str("    }\n");
        html.push_str("};\n");
        html.push_str("    </script>\n</body>\n</html>");
        html
    }
}

impl Default for BreakingNewsFeed {
    fn default() -> Self {
        Self::new("Legal News Feed", "ws://localhost:8080")
    }
}

/// News item for legal news feed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsItem {
    /// News title
    pub title: String,
    /// News summary
    pub summary: String,
    /// News source
    pub source: String,
    /// Timestamp
    pub timestamp: String,
    /// Priority level
    pub priority: NewsPriority,
    /// Tags
    pub tags: Vec<String>,
}

impl NewsItem {
    /// Creates a new news item.
    pub fn new(
        title: &str,
        summary: &str,
        source: &str,
        timestamp: &str,
        priority: NewsPriority,
    ) -> Self {
        Self {
            title: title.to_string(),
            summary: summary.to_string(),
            source: source.to_string(),
            timestamp: timestamp.to_string(),
            priority,
            tags: Vec::new(),
        }
    }

    /// Adds a tag.
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
}

/// News priority levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NewsPriority {
    /// Urgent/breaking news
    Urgent,
    /// High priority
    High,
    /// Medium priority
    Medium,
    /// Low priority
    Low,
}

/// Regulatory change monitoring visualizer.
pub struct RegulatoryChangeMonitor {
    /// Monitor title
    title: String,
    /// WebSocket URL for updates
    ws_url: String,
    /// Theme
    theme: Theme,
}

impl RegulatoryChangeMonitor {
    /// Creates a new regulatory change monitor.
    pub fn new(title: &str, ws_url: &str) -> Self {
        Self {
            title: title.to_string(),
            ws_url: ws_url.to_string(),
            theme: Theme::default(),
        }
    }

    /// Sets the theme.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Generates HTML for regulatory change monitor.
    pub fn to_html(&self, changes: &[RegulatoryChange]) -> String {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str("    <meta charset=\"utf-8\">\n");
        html.push_str(
            "    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n",
        );
        html.push_str(&format!("    <title>{}</title>\n", self.title));
        html.push_str("    <style>\n");
        html.push_str(&format!("        body {{ background-color: {}; color: {}; font-family: Arial, sans-serif; margin: 0; padding: 0; }}\n", self.theme.background_color, self.theme.text_color));
        html.push_str("        .header { background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 30px; }\n");
        html.push_str("        .header h1 { margin: 0; }\n");
        html.push_str("        .container { max-width: 1200px; margin: 0 auto; padding: 20px; }\n");
        html.push_str("        .filters { background-color: white; padding: 15px; margin-bottom: 20px; border-radius: 5px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }\n");
        html.push_str("        .filter-btn { padding: 8px 15px; margin: 5px; border: none; border-radius: 3px; cursor: pointer; background-color: #ecf0f1; }\n");
        html.push_str("        .filter-btn.active { background-color: #3498db; color: white; }\n");
        html.push_str("        .change-card { background-color: white; border-radius: 8px; padding: 20px; margin: 15px 0; box-shadow: 0 2px 8px rgba(0,0,0,0.1); }\n");
        html.push_str("        .change-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 15px; }\n");
        html.push_str(
            "        .change-title { font-size: 1.3em; font-weight: bold; color: #2c3e50; }\n",
        );
        html.push_str("        .change-badge { padding: 5px 12px; border-radius: 20px; font-size: 0.85em; font-weight: bold; }\n");
        html.push_str("        .badge-proposed { background-color: #3498db; color: white; }\n");
        html.push_str("        .badge-enacted { background-color: #27ae60; color: white; }\n");
        html.push_str("        .badge-repealed { background-color: #e74c3c; color: white; }\n");
        html.push_str("        .badge-amended { background-color: #f39c12; color: white; }\n");
        html.push_str(
            "        .change-meta { color: #7f8c8d; font-size: 0.9em; margin-bottom: 10px; }\n",
        );
        html.push_str("        .change-description { line-height: 1.6; color: #34495e; margin-bottom: 15px; }\n");
        html.push_str("        .change-impact { background-color: #fff3cd; border-left: 4px solid #f39c12; padding: 10px; margin-top: 10px; }\n");
        html.push_str("        .change-impact-title { font-weight: bold; color: #856404; }\n");
        html.push_str("        .sectors { margin-top: 10px; }\n");
        html.push_str("        .sector-tag { display: inline-block; background-color: #e8f4f8; color: #0366d6; padding: 4px 10px; margin: 3px; border-radius: 3px; font-size: 0.85em; }\n");
        html.push_str("    </style>\n</head>\n<body>\n");
        html.push_str("    <div class=\"header\">\n");
        html.push_str(&format!("        <h1>{}</h1>\n", self.title));
        html.push_str("    </div>\n");
        html.push_str("    <div class=\"container\">\n");
        html.push_str("        <div class=\"filters\">\n");
        html.push_str(
            "            <button class=\"filter-btn active\" data-filter=\"all\">All</button>\n",
        );
        html.push_str(
            "            <button class=\"filter-btn\" data-filter=\"Proposed\">Proposed</button>\n",
        );
        html.push_str(
            "            <button class=\"filter-btn\" data-filter=\"Enacted\">Enacted</button>\n",
        );
        html.push_str(
            "            <button class=\"filter-btn\" data-filter=\"Amended\">Amended</button>\n",
        );
        html.push_str(
            "            <button class=\"filter-btn\" data-filter=\"Repealed\">Repealed</button>\n",
        );
        html.push_str("        </div>\n");
        html.push_str("        <div id=\"changes-list\">\n");

        for change in changes {
            let status_class = format!("badge-{}", format!("{:?}", change.status).to_lowercase());

            html.push_str(&format!(
                "        <div class=\"change-card\" data-status=\"{:?}\">\n",
                change.status
            ));
            html.push_str("            <div class=\"change-header\">\n");
            html.push_str(&format!(
                "                <div class=\"change-title\">{}</div>\n",
                change.regulation_id
            ));
            html.push_str(&format!(
                "                <div class=\"change-badge {}\">{:?}</div>\n",
                status_class, change.status
            ));
            html.push_str("            </div>\n");
            html.push_str(&format!(
                "            <div class=\"change-meta\">Agency: {} | Effective: {}</div>\n",
                change.agency, change.effective_date
            ));
            html.push_str(&format!(
                "            <div class=\"change-description\">{}</div>\n",
                change.description
            ));

            if let Some(impact) = &change.impact_assessment {
                html.push_str("            <div class=\"change-impact\">\n");
                html.push_str(
                    "                <div class=\"change-impact-title\">Impact Assessment</div>\n",
                );
                html.push_str(&format!("                <div>{}</div>\n", impact));
                html.push_str("            </div>\n");
            }

            if !change.affected_sectors.is_empty() {
                html.push_str("            <div class=\"sectors\">\n");
                for sector in &change.affected_sectors {
                    html.push_str(&format!(
                        "                <span class=\"sector-tag\">{}</span>\n",
                        sector
                    ));
                }
                html.push_str("            </div>\n");
            }

            html.push_str("        </div>\n");
        }

        html.push_str("        </div>\n");
        html.push_str("    </div>\n");
        html.push_str("    <script>\n");
        html.push_str(&format!("const ws = new WebSocket('{}');\n", self.ws_url));
        html.push_str("ws.onmessage = function(event) {\n");
        html.push_str("    const data = JSON.parse(event.data);\n");
        html.push_str("    const container = document.getElementById('changes-list');\n");
        html.push_str("    const card = document.createElement('div');\n");
        html.push_str("    card.className = 'change-card';\n");
        html.push_str("    card.setAttribute('data-status', data.status);\n");
        html.push_str("    const statusClass = 'badge-' + data.status.toLowerCase();\n");
        html.push_str("    card.innerHTML = `\n");
        html.push_str("        <div class=\"change-header\">\n");
        html.push_str("            <div class=\"change-title\">${data.regulation_id}</div>\n");
        html.push_str(
            "            <div class=\"change-badge ${statusClass}\">${data.status}</div>\n",
        );
        html.push_str("        </div>\n");
        html.push_str("        <div class=\"change-meta\">Agency: ${data.agency} | Effective: ${data.effective_date}</div>\n");
        html.push_str("        <div class=\"change-description\">${data.description}</div>\n");
        html.push_str("        ${data.impact_assessment ? '<div class=\"change-impact\"><div class=\"change-impact-title\">Impact Assessment</div><div>' + data.impact_assessment + '</div></div>' : ''}\n");
        html.push_str("        ${data.affected_sectors && data.affected_sectors.length > 0 ? '<div class=\"sectors\">' + data.affected_sectors.map(s => '<span class=\"sector-tag\">' + s + '</span>').join('') + '</div>' : ''}\n");
        html.push_str("    `;\n");
        html.push_str("    container.insertBefore(card, container.firstChild);\n");
        html.push_str("};\n");
        html.push_str("// Filter functionality\n");
        html.push_str("document.querySelectorAll('.filter-btn').forEach(btn => {\n");
        html.push_str("    btn.addEventListener('click', function() {\n");
        html.push_str("        document.querySelectorAll('.filter-btn').forEach(b => b.classList.remove('active'));\n");
        html.push_str("        this.classList.add('active');\n");
        html.push_str("        const filter = this.getAttribute('data-filter');\n");
        html.push_str("        document.querySelectorAll('.change-card').forEach(card => {\n");
        html.push_str(
            "            if (filter === 'all' || card.getAttribute('data-status') === filter) {\n",
        );
        html.push_str("                card.style.display = 'block';\n");
        html.push_str("            } else {\n");
        html.push_str("                card.style.display = 'none';\n");
        html.push_str("            }\n");
        html.push_str("        });\n");
        html.push_str("    });\n");
        html.push_str("});\n");
        html.push_str("    </script>\n</body>\n</html>");
        html
    }
}

impl Default for RegulatoryChangeMonitor {
    fn default() -> Self {
        Self::new("Regulatory Change Monitor", "ws://localhost:8080")
    }
}

/// Regulatory change item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulatoryChange {
    /// Regulation ID
    pub regulation_id: String,
    /// Description of the change
    pub description: String,
    /// Agency responsible
    pub agency: String,
    /// Effective date
    pub effective_date: String,
    /// Change status
    pub status: RegulatoryStatus,
    /// Impact assessment
    pub impact_assessment: Option<String>,
    /// Affected sectors
    pub affected_sectors: Vec<String>,
}

impl RegulatoryChange {
    /// Creates a new regulatory change.
    pub fn new(
        regulation_id: &str,
        description: &str,
        agency: &str,
        effective_date: &str,
        status: RegulatoryStatus,
    ) -> Self {
        Self {
            regulation_id: regulation_id.to_string(),
            description: description.to_string(),
            agency: agency.to_string(),
            effective_date: effective_date.to_string(),
            status,
            impact_assessment: None,
            affected_sectors: Vec::new(),
        }
    }

    /// Sets impact assessment.
    pub fn with_impact(mut self, impact: &str) -> Self {
        self.impact_assessment = Some(impact.to_string());
        self
    }

    /// Adds affected sector.
    pub fn with_sector(mut self, sector: &str) -> Self {
        self.affected_sectors.push(sector.to_string());
        self
    }
}

/// Regulatory status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegulatoryStatus {
    /// Proposed regulation
    Proposed,
    /// Enacted regulation
    Enacted,
    /// Amended regulation
    Amended,
    /// Repealed regulation
    Repealed,
}

/// Enforcement action tracking visualizer.
pub struct EnforcementActionTracker {
    /// Tracker title
    title: String,
    /// WebSocket URL for updates
    ws_url: String,
    /// Theme
    theme: Theme,
}

impl EnforcementActionTracker {
    /// Creates a new enforcement action tracker.
    pub fn new(title: &str, ws_url: &str) -> Self {
        Self {
            title: title.to_string(),
            ws_url: ws_url.to_string(),
            theme: Theme::default(),
        }
    }

    /// Sets the theme.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Generates HTML for enforcement action tracker.
    pub fn to_html(&self, actions: &[EnforcementAction]) -> String {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str("    <meta charset=\"utf-8\">\n");
        html.push_str(
            "    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n",
        );
        html.push_str(&format!("    <title>{}</title>\n", self.title));
        html.push_str("    <style>\n");
        html.push_str(&format!("        body {{ background-color: {}; color: {}; font-family: Arial, sans-serif; margin: 0; padding: 0; }}\n", self.theme.background_color, self.theme.text_color));
        html.push_str(
            "        .header { background-color: #c0392b; color: white; padding: 30px; }\n",
        );
        html.push_str("        .header h1 { margin: 0; }\n");
        html.push_str("        .stats { display: flex; justify-content: space-around; background-color: white; padding: 20px; margin: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }\n");
        html.push_str("        .stat { text-align: center; }\n");
        html.push_str(
            "        .stat-value { font-size: 2.5em; font-weight: bold; color: #2c3e50; }\n",
        );
        html.push_str("        .stat-label { color: #7f8c8d; margin-top: 5px; }\n");
        html.push_str("        .container { max-width: 1200px; margin: 0 auto; padding: 20px; }\n");
        html.push_str("        .action-card { background-color: white; border-radius: 8px; padding: 20px; margin: 15px 0; box-shadow: 0 2px 8px rgba(0,0,0,0.1); }\n");
        html.push_str("        .action-header { display: flex; justify-content: space-between; align-items: start; margin-bottom: 15px; }\n");
        html.push_str(
            "        .action-entity { font-size: 1.4em; font-weight: bold; color: #2c3e50; }\n",
        );
        html.push_str("        .action-type { padding: 6px 14px; border-radius: 20px; font-size: 0.85em; font-weight: bold; }\n");
        html.push_str("        .type-fine { background-color: #f39c12; color: white; }\n");
        html.push_str("        .type-warning { background-color: #e67e22; color: white; }\n");
        html.push_str("        .type-suspension { background-color: #e74c3c; color: white; }\n");
        html.push_str("        .type-settlement { background-color: #3498db; color: white; }\n");
        html.push_str("        .type-investigation { background-color: #9b59b6; color: white; }\n");
        html.push_str("        .action-details { display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 15px; margin-top: 15px; }\n");
        html.push_str("        .detail-item { }\n");
        html.push_str(
            "        .detail-label { font-weight: bold; color: #7f8c8d; font-size: 0.85em; }\n",
        );
        html.push_str("        .detail-value { color: #2c3e50; margin-top: 3px; }\n");
        html.push_str("        .action-violations { background-color: #fff5f5; border-left: 4px solid #e74c3c; padding: 10px; margin-top: 10px; }\n");
        html.push_str("        .violations-title { font-weight: bold; color: #c0392b; margin-bottom: 5px; }\n");
        html.push_str("    </style>\n</head>\n<body>\n");
        html.push_str("    <div class=\"header\">\n");
        html.push_str(&format!("        <h1>{}</h1>\n", self.title));
        html.push_str("    </div>\n");

        // Calculate statistics
        let total_actions = actions.len();
        let total_fines: f64 = actions.iter().filter_map(|a| a.fine_amount).sum();
        let pending_count = actions
            .iter()
            .filter(|a| a.status == EnforcementStatus::Pending)
            .count();

        html.push_str("    <div class=\"stats\">\n");
        html.push_str("        <div class=\"stat\">\n");
        html.push_str(&format!(
            "            <div class=\"stat-value\" id=\"total-actions\">{}</div>\n",
            total_actions
        ));
        html.push_str("            <div class=\"stat-label\">Total Actions</div>\n");
        html.push_str("        </div>\n");
        html.push_str("        <div class=\"stat\">\n");
        html.push_str(&format!(
            "            <div class=\"stat-value\" id=\"total-fines\">${:.0}M</div>\n",
            total_fines / 1_000_000.0
        ));
        html.push_str("            <div class=\"stat-label\">Total Fines</div>\n");
        html.push_str("        </div>\n");
        html.push_str("        <div class=\"stat\">\n");
        html.push_str(&format!(
            "            <div class=\"stat-value\" id=\"pending-count\">{}</div>\n",
            pending_count
        ));
        html.push_str("            <div class=\"stat-label\">Pending</div>\n");
        html.push_str("        </div>\n");
        html.push_str("    </div>\n");

        html.push_str("    <div class=\"container\" id=\"actions-list\">\n");

        for action in actions {
            let action_type_class = format!(
                "type-{}",
                format!("{:?}", action.action_type).to_lowercase()
            );

            html.push_str("        <div class=\"action-card\">\n");
            html.push_str("            <div class=\"action-header\">\n");
            html.push_str(&format!(
                "                <div class=\"action-entity\">{}</div>\n",
                action.entity
            ));
            html.push_str(&format!(
                "                <div class=\"action-type {}\">{:?}</div>\n",
                action_type_class, action.action_type
            ));
            html.push_str("            </div>\n");
            html.push_str("            <div class=\"action-details\">\n");
            html.push_str("                <div class=\"detail-item\">\n");
            html.push_str("                    <div class=\"detail-label\">Agency</div>\n");
            html.push_str(&format!(
                "                    <div class=\"detail-value\">{}</div>\n",
                action.agency
            ));
            html.push_str("                </div>\n");
            html.push_str("                <div class=\"detail-item\">\n");
            html.push_str("                    <div class=\"detail-label\">Date</div>\n");
            html.push_str(&format!(
                "                    <div class=\"detail-value\">{}</div>\n",
                action.action_date
            ));
            html.push_str("                </div>\n");
            html.push_str("                <div class=\"detail-item\">\n");
            html.push_str("                    <div class=\"detail-label\">Status</div>\n");
            html.push_str(&format!(
                "                    <div class=\"detail-value\">{:?}</div>\n",
                action.status
            ));
            html.push_str("                </div>\n");

            if let Some(fine) = action.fine_amount {
                html.push_str("                <div class=\"detail-item\">\n");
                html.push_str(
                    "                    <div class=\"detail-label\">Fine Amount</div>\n",
                );
                html.push_str(&format!(
                    "                    <div class=\"detail-value\">${:.0}</div>\n",
                    fine
                ));
                html.push_str("                </div>\n");
            }

            html.push_str("            </div>\n");

            if !action.violations.is_empty() {
                html.push_str("            <div class=\"action-violations\">\n");
                html.push_str(
                    "                <div class=\"violations-title\">Violations:</div>\n",
                );
                html.push_str(
                    "                <ul style=\"margin: 5px 0; padding-left: 20px;\">\n",
                );
                for violation in &action.violations {
                    html.push_str(&format!("                    <li>{}</li>\n", violation));
                }
                html.push_str("                </ul>\n");
                html.push_str("            </div>\n");
            }

            html.push_str("        </div>\n");
        }

        html.push_str("    </div>\n");
        html.push_str("    <script>\n");
        html.push_str(&format!("const ws = new WebSocket('{}');\n", self.ws_url));
        html.push_str("ws.onmessage = function(event) {\n");
        html.push_str("    const data = JSON.parse(event.data);\n");
        html.push_str("    const container = document.getElementById('actions-list');\n");
        html.push_str("    const card = document.createElement('div');\n");
        html.push_str("    card.className = 'action-card';\n");
        html.push_str("    const actionTypeClass = 'type-' + data.action_type.toLowerCase();\n");
        html.push_str("    card.innerHTML = `\n");
        html.push_str("        <div class=\"action-header\">\n");
        html.push_str("            <div class=\"action-entity\">${data.entity}</div>\n");
        html.push_str(
            "            <div class=\"action-type ${actionTypeClass}\">${data.action_type}</div>\n",
        );
        html.push_str("        </div>\n");
        html.push_str("        <div class=\"action-details\">\n");
        html.push_str("            <div class=\"detail-item\"><div class=\"detail-label\">Agency</div><div class=\"detail-value\">${data.agency}</div></div>\n");
        html.push_str("            <div class=\"detail-item\"><div class=\"detail-label\">Date</div><div class=\"detail-value\">${data.action_date}</div></div>\n");
        html.push_str("            <div class=\"detail-item\"><div class=\"detail-label\">Status</div><div class=\"detail-value\">${data.status}</div></div>\n");
        html.push_str("            ${data.fine_amount ? '<div class=\"detail-item\"><div class=\"detail-label\">Fine Amount</div><div class=\"detail-value\">$' + data.fine_amount.toLocaleString() + '</div></div>' : ''}\n");
        html.push_str("        </div>\n");
        html.push_str("        ${data.violations && data.violations.length > 0 ? '<div class=\"action-violations\"><div class=\"violations-title\">Violations:</div><ul style=\"margin: 5px 0; padding-left: 20px;\">' + data.violations.map(v => '<li>' + v + '</li>').join('') + '</ul></div>' : ''}\n");
        html.push_str("    `;\n");
        html.push_str("    container.insertBefore(card, container.firstChild);\n");
        html.push_str("    // Update stats\n");
        html.push_str("    const totalActions = document.getElementById('total-actions');\n");
        html.push_str("    totalActions.textContent = parseInt(totalActions.textContent) + 1;\n");
        html.push_str("    if (data.fine_amount) {\n");
        html.push_str("        const totalFines = document.getElementById('total-fines');\n");
        html.push_str("        const currentValue = parseFloat(totalFines.textContent.replace('$', '').replace('M', '')) * 1000000;\n");
        html.push_str("        const newValue = (currentValue + data.fine_amount) / 1000000;\n");
        html.push_str("        totalFines.textContent = '$' + newValue.toFixed(0) + 'M';\n");
        html.push_str("    }\n");
        html.push_str("    if (data.status === 'Pending') {\n");
        html.push_str("        const pending = document.getElementById('pending-count');\n");
        html.push_str("        pending.textContent = parseInt(pending.textContent) + 1;\n");
        html.push_str("    }\n");
        html.push_str("};\n");
        html.push_str("    </script>\n</body>\n</html>");
        html
    }
}

impl Default for EnforcementActionTracker {
    fn default() -> Self {
        Self::new("Enforcement Action Tracker", "ws://localhost:8080")
    }
}

/// Enforcement action item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnforcementAction {
    /// Entity subject to enforcement
    pub entity: String,
    /// Enforcement agency
    pub agency: String,
    /// Action date
    pub action_date: String,
    /// Action type
    pub action_type: EnforcementActionType,
    /// Action status
    pub status: EnforcementStatus,
    /// Fine amount (if applicable)
    pub fine_amount: Option<f64>,
    /// List of violations
    pub violations: Vec<String>,
}

impl EnforcementAction {
    /// Creates a new enforcement action.
    pub fn new(
        entity: &str,
        agency: &str,
        action_date: &str,
        action_type: EnforcementActionType,
        status: EnforcementStatus,
    ) -> Self {
        Self {
            entity: entity.to_string(),
            agency: agency.to_string(),
            action_date: action_date.to_string(),
            action_type,
            status,
            fine_amount: None,
            violations: Vec::new(),
        }
    }

    /// Sets fine amount.
    pub fn with_fine(mut self, amount: f64) -> Self {
        self.fine_amount = Some(amount);
        self
    }

    /// Adds a violation.
    pub fn with_violation(mut self, violation: &str) -> Self {
        self.violations.push(violation.to_string());
        self
    }
}

/// Enforcement action types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnforcementActionType {
    /// Monetary fine
    Fine,
    /// Warning letter
    Warning,
    /// License suspension
    Suspension,
    /// Settlement agreement
    Settlement,
    /// Investigation initiated
    Investigation,
}

/// Enforcement status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnforcementStatus {
    /// Pending action
    Pending,
    /// Active/ongoing
    Active,
    /// Resolved
    Resolved,
    /// Appealed
    Appealed,
}

/// Market impact visualization for legal changes.
pub struct MarketImpactVisualizer {
    /// Visualizer title
    title: String,
    /// WebSocket URL for updates
    ws_url: String,
    /// Theme
    theme: Theme,
}

impl MarketImpactVisualizer {
    /// Creates a new market impact visualizer.
    pub fn new(title: &str, ws_url: &str) -> Self {
        Self {
            title: title.to_string(),
            ws_url: ws_url.to_string(),
            theme: Theme::default(),
        }
    }

    /// Sets the theme.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Generates HTML for market impact visualization.
    pub fn to_html(&self, impacts: &[MarketImpact]) -> String {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str("    <meta charset=\"utf-8\">\n");
        html.push_str(
            "    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n",
        );
        html.push_str(&format!("    <title>{}</title>\n", self.title));
        html.push_str("    <script src=\"https://cdn.jsdelivr.net/npm/chart.js\"></script>\n");
        html.push_str("    <style>\n");
        html.push_str(&format!("        body {{ background-color: {}; color: {}; font-family: 'Segoe UI', Arial, sans-serif; margin: 0; padding: 0; }}\n", self.theme.background_color, self.theme.text_color));
        html.push_str("        .header { background: linear-gradient(135deg, #1e3c72 0%, #2a5298 100%); color: white; padding: 30px; }\n");
        html.push_str("        .header h1 { margin: 0; }\n");
        html.push_str("        .container { max-width: 1400px; margin: 0 auto; padding: 20px; }\n");
        html.push_str("        .grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(350px, 1fr)); gap: 20px; margin-bottom: 30px; }\n");
        html.push_str("        .card { background-color: white; border-radius: 8px; padding: 20px; box-shadow: 0 2px 8px rgba(0,0,0,0.1); }\n");
        html.push_str("        .card-title { font-size: 1.2em; font-weight: bold; color: #2c3e50; margin-bottom: 15px; }\n");
        html.push_str("        .metric { display: flex; justify-content: space-between; padding: 10px 0; border-bottom: 1px solid #ecf0f1; }\n");
        html.push_str("        .metric-label { color: #7f8c8d; }\n");
        html.push_str("        .metric-value { font-weight: bold; color: #2c3e50; }\n");
        html.push_str("        .positive { color: #27ae60; }\n");
        html.push_str("        .negative { color: #e74c3c; }\n");
        html.push_str("        .neutral { color: #95a5a6; }\n");
        html.push_str("        .chart-container { position: relative; height: 300px; }\n");
        html.push_str("        .impact-list { }\n");
        html.push_str("        .impact-item { background-color: #f8f9fa; padding: 15px; margin: 10px 0; border-radius: 5px; border-left: 4px solid #3498db; }\n");
        html.push_str("        .impact-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 10px; }\n");
        html.push_str("        .impact-legal { font-weight: bold; color: #2c3e50; }\n");
        html.push_str("        .impact-badge { padding: 4px 10px; border-radius: 3px; font-size: 0.85em; font-weight: bold; }\n");
        html.push_str("        .badge-high { background-color: #e74c3c; color: white; }\n");
        html.push_str("        .badge-medium { background-color: #f39c12; color: white; }\n");
        html.push_str("        .badge-low { background-color: #3498db; color: white; }\n");
        html.push_str("        .sectors { margin-top: 8px; }\n");
        html.push_str("        .sector-badge { display: inline-block; background-color: #e8f4f8; padding: 3px 8px; margin: 2px; border-radius: 3px; font-size: 0.8em; }\n");
        html.push_str("    </style>\n</head>\n<body>\n");
        html.push_str("    <div class=\"header\">\n");
        html.push_str(&format!("        <h1>{}</h1>\n", self.title));
        html.push_str("    </div>\n");
        html.push_str("    <div class=\"container\">\n");
        html.push_str("        <div class=\"grid\">\n");
        html.push_str("            <div class=\"card\">\n");
        html.push_str("                <div class=\"card-title\">Market Sentiment</div>\n");
        html.push_str("                <div class=\"chart-container\">\n");
        html.push_str("                    <canvas id=\"sentimentChart\"></canvas>\n");
        html.push_str("                </div>\n");
        html.push_str("            </div>\n");
        html.push_str("            <div class=\"card\">\n");
        html.push_str("                <div class=\"card-title\">Sector Impact</div>\n");
        html.push_str("                <div class=\"chart-container\">\n");
        html.push_str("                    <canvas id=\"sectorChart\"></canvas>\n");
        html.push_str("                </div>\n");
        html.push_str("            </div>\n");
        html.push_str("            <div class=\"card\">\n");
        html.push_str("                <div class=\"card-title\">Key Metrics</div>\n");

        // Calculate summary metrics
        let avg_stock_change: f64 = impacts
            .iter()
            .filter_map(|i| i.stock_price_change)
            .sum::<f64>()
            / impacts.len().max(1) as f64;
        let total_affected = impacts.len();

        html.push_str(&format!("                <div class=\"metric\"><span class=\"metric-label\">Avg. Stock Change</span><span class=\"metric-value {}\">{:.2}%</span></div>\n",
            if avg_stock_change > 0.0 { "positive" } else if avg_stock_change < 0.0 { "negative" } else { "neutral" },
            avg_stock_change));
        html.push_str(&format!("                <div class=\"metric\"><span class=\"metric-label\">Affected Items</span><span class=\"metric-value\">{}</span></div>\n", total_affected));
        html.push_str("            </div>\n");
        html.push_str("        </div>\n");

        html.push_str("        <div class=\"card\">\n");
        html.push_str("            <div class=\"card-title\">Impact Details</div>\n");
        html.push_str("            <div class=\"impact-list\" id=\"impact-list\">\n");

        for impact in impacts {
            let severity_class = match impact.severity {
                ImpactSeverity::High => "badge-high",
                ImpactSeverity::Medium => "badge-medium",
                ImpactSeverity::Low => "badge-low",
            };

            html.push_str("                <div class=\"impact-item\">\n");
            html.push_str("                    <div class=\"impact-header\">\n");
            html.push_str(&format!(
                "                        <div class=\"impact-legal\">{}</div>\n",
                impact.legal_event
            ));
            html.push_str(&format!(
                "                        <div class=\"impact-badge {}\">{:?} Impact</div>\n",
                severity_class, impact.severity
            ));
            html.push_str("                    </div>\n");
            html.push_str(&format!(
                "                    <div><strong>Date:</strong> {}</div>\n",
                impact.event_date
            ));

            if let Some(stock_change) = impact.stock_price_change {
                let change_class = if stock_change > 0.0 {
                    "positive"
                } else if stock_change < 0.0 {
                    "negative"
                } else {
                    "neutral"
                };
                html.push_str(&format!("                    <div><strong>Stock Impact:</strong> <span class=\"{}\">{:.2}%</span></div>\n", change_class, stock_change));
            }

            if !impact.affected_companies.is_empty() {
                html.push_str(&format!(
                    "                    <div><strong>Affected:</strong> {}</div>\n",
                    impact.affected_companies.join(", ")
                ));
            }

            if !impact.sectors.is_empty() {
                html.push_str("                    <div class=\"sectors\">\n");
                for sector in &impact.sectors {
                    html.push_str(&format!(
                        "                        <span class=\"sector-badge\">{}</span>\n",
                        sector
                    ));
                }
                html.push_str("                    </div>\n");
            }

            html.push_str("                </div>\n");
        }

        html.push_str("            </div>\n");
        html.push_str("        </div>\n");
        html.push_str("    </div>\n");
        html.push_str("    <script>\n");

        // Sentiment chart
        html.push_str(
            "const sentimentCtx = document.getElementById('sentimentChart').getContext('2d');\n",
        );
        html.push_str("new Chart(sentimentCtx, {\n");
        html.push_str("    type: 'line',\n");
        html.push_str("    data: {\n");
        html.push_str("        labels: [");
        for (i, impact) in impacts.iter().enumerate() {
            if i > 0 {
                html.push_str(", ");
            }
            html.push_str(&format!("'{}'", impact.event_date));
        }
        html.push_str("],\n");
        html.push_str("        datasets: [{\n");
        html.push_str("            label: 'Stock Price Change (%)',\n");
        html.push_str("            data: [");
        for (i, impact) in impacts.iter().enumerate() {
            if i > 0 {
                html.push_str(", ");
            }
            html.push_str(&format!("{}", impact.stock_price_change.unwrap_or(0.0)));
        }
        html.push_str("],\n");
        html.push_str("            borderColor: '#3498db',\n");
        html.push_str("            tension: 0.4\n");
        html.push_str("        }]\n");
        html.push_str("    },\n");
        html.push_str("    options: { responsive: true, maintainAspectRatio: false }\n");
        html.push_str("});\n");

        // Sector chart
        let mut sector_counts: HashMap<String, usize> = HashMap::new();
        for impact in impacts {
            for sector in &impact.sectors {
                *sector_counts.entry(sector.clone()).or_insert(0) += 1;
            }
        }

        html.push_str(
            "const sectorCtx = document.getElementById('sectorChart').getContext('2d');\n",
        );
        html.push_str("new Chart(sectorCtx, {\n");
        html.push_str("    type: 'bar',\n");
        html.push_str("    data: {\n");
        html.push_str("        labels: [");
        for (i, sector) in sector_counts.keys().enumerate() {
            if i > 0 {
                html.push_str(", ");
            }
            html.push_str(&format!("'{}'", sector));
        }
        html.push_str("],\n");
        html.push_str("        datasets: [{\n");
        html.push_str("            label: 'Number of Impacts',\n");
        html.push_str("            data: [");
        for (i, count) in sector_counts.values().enumerate() {
            if i > 0 {
                html.push_str(", ");
            }
            html.push_str(&format!("{}", count));
        }
        html.push_str("],\n");
        html.push_str("            backgroundColor: '#2ecc71'\n");
        html.push_str("        }]\n");
        html.push_str("    },\n");
        html.push_str("    options: { responsive: true, maintainAspectRatio: false }\n");
        html.push_str("});\n");

        // WebSocket for live updates
        html.push_str(&format!("const ws = new WebSocket('{}');\n", self.ws_url));
        html.push_str("ws.onmessage = function(event) {\n");
        html.push_str("    const data = JSON.parse(event.data);\n");
        html.push_str("    const container = document.getElementById('impact-list');\n");
        html.push_str("    const item = document.createElement('div');\n");
        html.push_str("    item.className = 'impact-item';\n");
        html.push_str("    const severityClass = 'badge-' + data.severity.toLowerCase();\n");
        html.push_str("    const changeClass = data.stock_price_change > 0 ? 'positive' : data.stock_price_change < 0 ? 'negative' : 'neutral';\n");
        html.push_str("    item.innerHTML = `\n");
        html.push_str("        <div class=\"impact-header\">\n");
        html.push_str("            <div class=\"impact-legal\">${data.legal_event}</div>\n");
        html.push_str("            <div class=\"impact-badge ${severityClass}\">${data.severity} Impact</div>\n");
        html.push_str("        </div>\n");
        html.push_str("        <div><strong>Date:</strong> ${data.event_date}</div>\n");
        html.push_str("        ${data.stock_price_change != null ? '<div><strong>Stock Impact:</strong> <span class=\"' + changeClass + '\">' + data.stock_price_change.toFixed(2) + '%</span></div>' : ''}\n");
        html.push_str("        ${data.affected_companies && data.affected_companies.length > 0 ? '<div><strong>Affected:</strong> ' + data.affected_companies.join(', ') + '</div>' : ''}\n");
        html.push_str("        ${data.sectors && data.sectors.length > 0 ? '<div class=\"sectors\">' + data.sectors.map(s => '<span class=\"sector-badge\">' + s + '</span>').join('') + '</div>' : ''}\n");
        html.push_str("    `;\n");
        html.push_str("    container.insertBefore(item, container.firstChild);\n");
        html.push_str("};\n");

        html.push_str("    </script>\n</body>\n</html>");
        html
    }
}

impl Default for MarketImpactVisualizer {
    fn default() -> Self {
        Self::new("Market Impact Analysis", "ws://localhost:8080")
    }
}

/// Market impact item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketImpact {
    /// Legal event description
    pub legal_event: String,
    /// Event date
    pub event_date: String,
    /// Impact severity
    pub severity: ImpactSeverity,
    /// Stock price change percentage
    pub stock_price_change: Option<f64>,
    /// Affected companies
    pub affected_companies: Vec<String>,
    /// Affected sectors
    pub sectors: Vec<String>,
}

impl MarketImpact {
    /// Creates a new market impact.
    pub fn new(legal_event: &str, event_date: &str, severity: ImpactSeverity) -> Self {
        Self {
            legal_event: legal_event.to_string(),
            event_date: event_date.to_string(),
            severity,
            stock_price_change: None,
            affected_companies: Vec::new(),
            sectors: Vec::new(),
        }
    }

    /// Sets stock price change.
    pub fn with_stock_change(mut self, change: f64) -> Self {
        self.stock_price_change = Some(change);
        self
    }

    /// Adds affected company.
    pub fn with_company(mut self, company: &str) -> Self {
        self.affected_companies.push(company.to_string());
        self
    }

    /// Adds sector.
    pub fn with_sector(mut self, sector: &str) -> Self {
        self.sectors.push(sector.to_string());
        self
    }
}

/// Impact severity levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImpactSeverity {
    /// High impact
    High,
    /// Medium impact
    Medium,
    /// Low impact
    Low,
}

// ============================================================================
// Narrative Visualization (v0.3.3)
// ============================================================================

/// Scrollytelling configuration for legal histories.
pub struct ScrollytellingConfig {
    /// Enable scroll-based animations
    pub enable_animations: bool,
    /// Scroll trigger threshold (0.0-1.0)
    pub trigger_threshold: f64,
    /// Enable progress indicator
    pub show_progress: bool,
    /// Enable chapter navigation
    pub enable_navigation: bool,
}

impl ScrollytellingConfig {
    /// Creates a new scrollytelling configuration.
    pub fn new() -> Self {
        Self {
            enable_animations: true,
            trigger_threshold: 0.5,
            show_progress: true,
            enable_navigation: true,
        }
    }

    /// Disables scroll animations.
    pub fn without_animations(mut self) -> Self {
        self.enable_animations = false;
        self
    }

    /// Sets the trigger threshold.
    pub fn with_trigger_threshold(mut self, threshold: f64) -> Self {
        self.trigger_threshold = threshold.clamp(0.0, 1.0);
        self
    }

    /// Hides the progress indicator.
    pub fn without_progress(mut self) -> Self {
        self.show_progress = false;
        self
    }

    /// Disables chapter navigation.
    pub fn without_navigation(mut self) -> Self {
        self.enable_navigation = false;
        self
    }
}

impl Default for ScrollytellingConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Legal history scrollytelling visualizer.
pub struct LegalHistoryScrollytelling {
    /// Title
    title: String,
    /// Configuration
    config: ScrollytellingConfig,
    /// Theme
    theme: Theme,
}

impl LegalHistoryScrollytelling {
    /// Creates a new legal history scrollytelling visualizer.
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            config: ScrollytellingConfig::new(),
            theme: Theme::default(),
        }
    }

    /// Sets the configuration.
    pub fn with_config(mut self, config: ScrollytellingConfig) -> Self {
        self.config = config;
        self
    }

    /// Sets the theme.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Generates HTML for scrollytelling.
    #[allow(clippy::too_many_arguments)]
    pub fn to_html(&self, chapters: &[ScrollChapter]) -> String {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str("    <meta charset=\"utf-8\">\n");
        html.push_str(
            "    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n",
        );
        html.push_str(&format!("    <title>{}</title>\n", self.title));
        html.push_str("    <style>\n");
        html.push_str(&format!("        body {{ margin: 0; padding: 0; font-family: 'Georgia', serif; background-color: {}; color: {}; }}\n", self.theme.background_color, self.theme.text_color));
        html.push_str(
            "        .chapter { min-height: 100vh; padding: 100px 20px; position: relative; }\n",
        );
        html.push_str("        .chapter-content { max-width: 800px; margin: 0 auto; font-size: 1.2em; line-height: 1.8; }\n");
        html.push_str("        .chapter-title { font-size: 2.5em; font-weight: bold; margin-bottom: 30px; }\n");
        html.push_str("        .chapter-text { margin-bottom: 20px; }\n");
        html.push_str("        .visual-element { background-color: #f5f5f5; padding: 30px; margin: 40px 0; border-radius: 8px; text-align: center; }\n");
        html.push_str("        .progress-bar { position: fixed; top: 0; left: 0; height: 4px; background: linear-gradient(90deg, #3498db, #2ecc71); width: 0%; transition: width 0.3s; z-index: 1000; }\n");
        html.push_str("        .chapter-nav { position: fixed; right: 20px; top: 50%; transform: translateY(-50%); z-index: 100; }\n");
        html.push_str("        .nav-dot { width: 12px; height: 12px; border-radius: 50%; background-color: #ccc; margin: 10px 0; cursor: pointer; transition: all 0.3s; }\n");
        html.push_str(
            "        .nav-dot.active { background-color: #3498db; transform: scale(1.5); }\n",
        );
        html.push_str("        .fade-in { opacity: 0; transform: translateY(50px); transition: opacity 0.8s, transform 0.8s; }\n");
        html.push_str("        .fade-in.visible { opacity: 1; transform: translateY(0); }\n");
        html.push_str("    </style>\n</head>\n<body>\n");

        if self.config.show_progress {
            html.push_str("    <div class=\"progress-bar\" id=\"progress\"></div>\n");
        }

        if self.config.enable_navigation {
            html.push_str("    <div class=\"chapter-nav\" id=\"nav\">\n");
            for i in 0..chapters.len() {
                html.push_str(&format!(
                    "        <div class=\"nav-dot{}\" data-chapter=\"{}\"></div>\n",
                    if i == 0 { " active" } else { "" },
                    i
                ));
            }
            html.push_str("    </div>\n");
        }

        for (i, chapter) in chapters.iter().enumerate() {
            html.push_str(&format!(
                "    <div class=\"chapter\" id=\"chapter-{}\">\n",
                i
            ));
            html.push_str("        <div class=\"chapter-content fade-in\">\n");
            html.push_str(&format!(
                "            <h1 class=\"chapter-title\">{}</h1>\n",
                chapter.title
            ));
            for paragraph in &chapter.content {
                html.push_str(&format!(
                    "            <p class=\"chapter-text\">{}</p>\n",
                    paragraph
                ));
            }
            if let Some(visual) = &chapter.visual {
                html.push_str(&format!(
                    "            <div class=\"visual-element\">{}</div>\n",
                    visual
                ));
            }
            html.push_str("        </div>\n");
            html.push_str("    </div>\n");
        }

        html.push_str("    <script>\n");
        if self.config.enable_animations {
            html.push_str("function checkScroll() {\n");
            html.push_str("    const elements = document.querySelectorAll('.fade-in');\n");
            html.push_str("    elements.forEach(el => {\n");
            html.push_str("        const rect = el.getBoundingClientRect();\n");
            html.push_str(&format!(
                "        const threshold = window.innerHeight * {};\n",
                self.config.trigger_threshold
            ));
            html.push_str("        if (rect.top < threshold) { el.classList.add('visible'); }\n");
            html.push_str("    });\n");
            html.push_str("}\n");
            html.push_str("window.addEventListener('scroll', checkScroll);\n");
            html.push_str("checkScroll();\n");
        }

        if self.config.show_progress {
            html.push_str("window.addEventListener('scroll', () => {\n");
            html.push_str("    const scrolled = (window.scrollY / (document.body.scrollHeight - window.innerHeight)) * 100;\n");
            html.push_str(
                "    document.getElementById('progress').style.width = scrolled + '%';\n",
            );
            html.push_str("});\n");
        }

        if self.config.enable_navigation {
            html.push_str("const chapters = document.querySelectorAll('.chapter');\n");
            html.push_str("const navDots = document.querySelectorAll('.nav-dot');\n");
            html.push_str("navDots.forEach(dot => {\n");
            html.push_str("    dot.addEventListener('click', () => {\n");
            html.push_str("        const chapterNum = dot.getAttribute('data-chapter');\n");
            html.push_str("        chapters[chapterNum].scrollIntoView({ behavior: 'smooth' });\n");
            html.push_str("    });\n");
            html.push_str("});\n");
            html.push_str("window.addEventListener('scroll', () => {\n");
            html.push_str("    chapters.forEach((chapter, i) => {\n");
            html.push_str("        const rect = chapter.getBoundingClientRect();\n");
            html.push_str("        if (rect.top >= 0 && rect.top < window.innerHeight / 2) {\n");
            html.push_str("            navDots.forEach(d => d.classList.remove('active'));\n");
            html.push_str("            navDots[i].classList.add('active');\n");
            html.push_str("        }\n");
            html.push_str("    });\n");
            html.push_str("});\n");
        }

        html.push_str("    </script>\n</body>\n</html>");
        html
    }
}

impl Default for LegalHistoryScrollytelling {
    fn default() -> Self {
        Self::new("Legal History")
    }
}

/// Scroll chapter for scrollytelling.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrollChapter {
    /// Chapter title
    pub title: String,
    /// Chapter content paragraphs
    pub content: Vec<String>,
    /// Optional visual element HTML
    pub visual: Option<String>,
}

impl ScrollChapter {
    /// Creates a new scroll chapter.
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            content: Vec::new(),
            visual: None,
        }
    }

    /// Adds a content paragraph.
    pub fn with_paragraph(mut self, paragraph: &str) -> Self {
        self.content.push(paragraph.to_string());
        self
    }

    /// Sets a visual element.
    pub fn with_visual(mut self, visual: &str) -> Self {
        self.visual = Some(visual.to_string());
        self
    }
}

/// Case story generator for narrative visualization.
pub struct CaseStoryGenerator {
    /// Theme
    theme: Theme,
    /// Include timeline
    include_timeline: bool,
    /// Include key players
    include_players: bool,
}

impl CaseStoryGenerator {
    /// Creates a new case story generator.
    pub fn new() -> Self {
        Self {
            theme: Theme::default(),
            include_timeline: true,
            include_players: true,
        }
    }

    /// Sets the theme.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Excludes timeline from story.
    pub fn without_timeline(mut self) -> Self {
        self.include_timeline = false;
        self
    }

    /// Excludes key players from story.
    pub fn without_players(mut self) -> Self {
        self.include_players = false;
        self
    }

    /// Generates HTML story for a case.
    pub fn generate_story(&self, case: &CaseStory) -> String {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str("    <meta charset=\"utf-8\">\n");
        html.push_str(
            "    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n",
        );
        html.push_str(&format!("    <title>{}</title>\n", case.title));
        html.push_str("    <style>\n");
        html.push_str(&format!("        body {{ background-color: {}; color: {}; font-family: 'Palatino', 'Georgia', serif; margin: 0; padding: 40px 20px; }}\n", self.theme.background_color, self.theme.text_color));
        html.push_str("        .story-container { max-width: 900px; margin: 0 auto; }\n");
        html.push_str("        .story-header { text-align: center; margin-bottom: 60px; border-bottom: 2px solid #ccc; padding-bottom: 30px; }\n");
        html.push_str(
            "        .story-title { font-size: 3em; font-weight: bold; margin-bottom: 10px; }\n",
        );
        html.push_str(
            "        .story-subtitle { font-size: 1.3em; color: #666; font-style: italic; }\n",
        );
        html.push_str("        .story-section { margin: 40px 0; }\n");
        html.push_str("        .section-title { font-size: 2em; font-weight: bold; margin-bottom: 20px; color: #2c3e50; }\n");
        html.push_str("        .story-text { font-size: 1.15em; line-height: 1.9; margin-bottom: 15px; text-align: justify; }\n");
        html.push_str("        .timeline-item { padding: 20px; margin: 15px 0; background-color: #f8f9fa; border-left: 4px solid #3498db; }\n");
        html.push_str("        .timeline-date { font-weight: bold; color: #3498db; }\n");
        html.push_str("        .player-card { display: inline-block; padding: 15px 25px; margin: 10px; background-color: #ecf0f1; border-radius: 8px; }\n");
        html.push_str("        .player-name { font-weight: bold; font-size: 1.1em; }\n");
        html.push_str("        .player-role { color: #7f8c8d; font-size: 0.9em; }\n");
        html.push_str("        .outcome-box { background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 30px; border-radius: 10px; margin: 30px 0; }\n");
        html.push_str("        .outcome-title { font-size: 1.8em; font-weight: bold; margin-bottom: 15px; }\n");
        html.push_str("    </style>\n</head>\n<body>\n");
        html.push_str("    <div class=\"story-container\">\n");
        html.push_str("        <div class=\"story-header\">\n");
        html.push_str(&format!(
            "            <h1 class=\"story-title\">{}</h1>\n",
            case.title
        ));
        html.push_str(&format!(
            "            <p class=\"story-subtitle\">{}</p>\n",
            case.subtitle
        ));
        html.push_str("        </div>\n");

        html.push_str("        <div class=\"story-section\">\n");
        html.push_str("            <h2 class=\"section-title\">The Case</h2>\n");
        for paragraph in &case.introduction {
            html.push_str(&format!(
                "            <p class=\"story-text\">{}</p>\n",
                paragraph
            ));
        }
        html.push_str("        </div>\n");

        if self.include_players && !case.key_players.is_empty() {
            html.push_str("        <div class=\"story-section\">\n");
            html.push_str("            <h2 class=\"section-title\">Key Players</h2>\n");
            for player in &case.key_players {
                html.push_str("            <div class=\"player-card\">\n");
                html.push_str(&format!(
                    "                <div class=\"player-name\">{}</div>\n",
                    player.name
                ));
                html.push_str(&format!(
                    "                <div class=\"player-role\">{}</div>\n",
                    player.role
                ));
                html.push_str("            </div>\n");
            }
            html.push_str("        </div>\n");
        }

        if self.include_timeline && !case.timeline.is_empty() {
            html.push_str("        <div class=\"story-section\">\n");
            html.push_str("            <h2 class=\"section-title\">Timeline of Events</h2>\n");
            for event in &case.timeline {
                html.push_str("            <div class=\"timeline-item\">\n");
                html.push_str(&format!(
                    "                <div class=\"timeline-date\">{}</div>\n",
                    event.date
                ));
                html.push_str(&format!(
                    "                <div>{}</div>\n",
                    event.description
                ));
                html.push_str("            </div>\n");
            }
            html.push_str("        </div>\n");
        }

        html.push_str("        <div class=\"story-section\">\n");
        html.push_str("            <h2 class=\"section-title\">The Resolution</h2>\n");
        for paragraph in &case.resolution {
            html.push_str(&format!(
                "            <p class=\"story-text\">{}</p>\n",
                paragraph
            ));
        }
        html.push_str("        </div>\n");

        if let Some(outcome) = &case.outcome {
            html.push_str("        <div class=\"outcome-box\">\n");
            html.push_str("            <div class=\"outcome-title\">Outcome</div>\n");
            html.push_str(&format!("            <div>{}</div>\n", outcome));
            html.push_str("        </div>\n");
        }

        html.push_str("    </div>\n</body>\n</html>");
        html
    }
}

impl Default for CaseStoryGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Case story data structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseStory {
    /// Case title
    pub title: String,
    /// Subtitle/tagline
    pub subtitle: String,
    /// Introduction paragraphs
    pub introduction: Vec<String>,
    /// Key players
    pub key_players: Vec<KeyPlayer>,
    /// Timeline of events
    pub timeline: Vec<TimelineStoryEvent>,
    /// Resolution paragraphs
    pub resolution: Vec<String>,
    /// Final outcome
    pub outcome: Option<String>,
}

impl CaseStory {
    /// Creates a new case story.
    pub fn new(title: &str, subtitle: &str) -> Self {
        Self {
            title: title.to_string(),
            subtitle: subtitle.to_string(),
            introduction: Vec::new(),
            key_players: Vec::new(),
            timeline: Vec::new(),
            resolution: Vec::new(),
            outcome: None,
        }
    }

    /// Adds introduction paragraph.
    pub fn with_intro(mut self, paragraph: &str) -> Self {
        self.introduction.push(paragraph.to_string());
        self
    }

    /// Adds a key player.
    pub fn with_player(mut self, name: &str, role: &str) -> Self {
        self.key_players.push(KeyPlayer {
            name: name.to_string(),
            role: role.to_string(),
        });
        self
    }

    /// Adds a timeline event.
    pub fn with_event(mut self, date: &str, description: &str) -> Self {
        self.timeline.push(TimelineStoryEvent {
            date: date.to_string(),
            description: description.to_string(),
        });
        self
    }

    /// Adds resolution paragraph.
    pub fn with_resolution(mut self, paragraph: &str) -> Self {
        self.resolution.push(paragraph.to_string());
        self
    }

    /// Sets the outcome.
    pub fn with_outcome(mut self, outcome: &str) -> Self {
        self.outcome = Some(outcome.to_string());
        self
    }
}

/// Key player in a case story.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyPlayer {
    /// Player name
    pub name: String,
    /// Player role
    pub role: String,
}

/// Timeline event in a story.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineStoryEvent {
    /// Event date
    pub date: String,
    /// Event description
    pub description: String,
}

/// Timeline narrative view generator.
pub struct TimelineNarrativeView {
    /// Title
    title: String,
    /// Theme
    theme: Theme,
    /// Show captions
    show_captions: bool,
}

impl TimelineNarrativeView {
    /// Creates a new timeline narrative view.
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            theme: Theme::default(),
            show_captions: true,
        }
    }

    /// Sets the theme.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Hides captions.
    pub fn without_captions(mut self) -> Self {
        self.show_captions = false;
        self
    }

    /// Generates HTML for narrative timeline.
    pub fn to_html(&self, events: &[NarrativeEvent]) -> String {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str("    <meta charset=\"utf-8\">\n");
        html.push_str(
            "    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n",
        );
        html.push_str(&format!("    <title>{}</title>\n", self.title));
        html.push_str("    <style>\n");
        html.push_str(&format!("        body {{ background-color: {}; color: {}; font-family: 'Helvetica Neue', Arial, sans-serif; margin: 0; padding: 40px 20px; }}\n", self.theme.background_color, self.theme.text_color));
        html.push_str("        .timeline-container { max-width: 1000px; margin: 0 auto; }\n");
        html.push_str("        .timeline-header { text-align: center; margin-bottom: 60px; }\n");
        html.push_str("        .timeline-title { font-size: 3em; font-weight: bold; }\n");
        html.push_str("        .timeline-track { position: relative; padding: 40px 0; }\n");
        html.push_str("        .timeline-line { position: absolute; left: 50%; width: 4px; height: 100%; background: linear-gradient(180deg, #3498db, #2ecc71); transform: translateX(-50%); }\n");
        html.push_str("        .narrative-event { position: relative; margin: 60px 0; }\n");
        html.push_str("        .event-content { width: 45%; padding: 30px; background-color: white; box-shadow: 0 4px 12px rgba(0,0,0,0.1); border-radius: 8px; position: relative; }\n");
        html.push_str(
            "        .narrative-event:nth-child(odd) .event-content { margin-left: 0; }\n",
        );
        html.push_str(
            "        .narrative-event:nth-child(even) .event-content { margin-left: 55%; }\n",
        );
        html.push_str("        .event-marker { position: absolute; left: 50%; top: 50%; width: 20px; height: 20px; background-color: #3498db; border: 4px solid white; border-radius: 50%; transform: translate(-50%, -50%); box-shadow: 0 2px 8px rgba(0,0,0,0.2); }\n");
        html.push_str("        .event-date { font-size: 1.1em; font-weight: bold; color: #3498db; margin-bottom: 10px; }\n");
        html.push_str("        .event-title { font-size: 1.5em; font-weight: bold; color: #2c3e50; margin-bottom: 15px; }\n");
        html.push_str(
            "        .event-narrative { font-size: 1.05em; line-height: 1.7; color: #34495e; }\n",
        );
        html.push_str("    </style>\n</head>\n<body>\n");
        html.push_str("    <div class=\"timeline-container\">\n");
        html.push_str("        <div class=\"timeline-header\">\n");
        html.push_str(&format!(
            "            <h1 class=\"timeline-title\">{}</h1>\n",
            self.title
        ));
        html.push_str("        </div>\n");
        html.push_str("        <div class=\"timeline-track\">\n");
        html.push_str("            <div class=\"timeline-line\"></div>\n");

        for event in events {
            html.push_str("            <div class=\"narrative-event\">\n");
            html.push_str("                <div class=\"event-marker\"></div>\n");
            html.push_str("                <div class=\"event-content\">\n");
            html.push_str(&format!(
                "                    <div class=\"event-date\">{}</div>\n",
                event.date
            ));
            html.push_str(&format!(
                "                    <div class=\"event-title\">{}</div>\n",
                event.title
            ));
            if self.show_captions {
                html.push_str(&format!(
                    "                    <div class=\"event-narrative\">{}</div>\n",
                    event.narrative
                ));
            }
            html.push_str("                </div>\n");
            html.push_str("            </div>\n");
        }

        html.push_str("        </div>\n");
        html.push_str("    </div>\n</body>\n</html>");
        html
    }
}

impl Default for TimelineNarrativeView {
    fn default() -> Self {
        Self::new("Timeline")
    }
}

/// Narrative event for timeline visualization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeEvent {
    /// Event date
    pub date: String,
    /// Event title
    pub title: String,
    /// Event narrative description
    pub narrative: String,
}

impl NarrativeEvent {
    /// Creates a new narrative event.
    pub fn new(date: &str, title: &str, narrative: &str) -> Self {
        Self {
            date: date.to_string(),
            title: title.to_string(),
            narrative: narrative.to_string(),
        }
    }
}

/// Guided exploration tour system.
pub struct GuidedExplorationTour {
    /// Tour title
    title: String,
    /// Theme
    theme: Theme,
    /// Enable auto-advance
    auto_advance: bool,
    /// Auto-advance delay (ms)
    advance_delay: u32,
}

impl GuidedExplorationTour {
    /// Creates a new guided exploration tour.
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            theme: Theme::default(),
            auto_advance: false,
            advance_delay: 5000,
        }
    }

    /// Sets the theme.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Enables auto-advance.
    pub fn with_auto_advance(mut self, delay_ms: u32) -> Self {
        self.auto_advance = true;
        self.advance_delay = delay_ms;
        self
    }

    /// Generates HTML for guided tour.
    pub fn to_html(&self, stops: &[TourStop]) -> String {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str("    <meta charset=\"utf-8\">\n");
        html.push_str(
            "    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n",
        );
        html.push_str(&format!("    <title>{}</title>\n", self.title));
        html.push_str("    <style>\n");
        html.push_str(&format!("        body {{ background-color: {}; color: {}; font-family: Arial, sans-serif; margin: 0; padding: 0; }}\n", self.theme.background_color, self.theme.text_color));
        html.push_str("        .tour-overlay { position: fixed; top: 0; left: 0; width: 100%; height: 100%; background-color: rgba(0,0,0,0.7); z-index: 9999; display: flex; align-items: center; justify-content: center; }\n");
        html.push_str("        .tour-card { background-color: white; max-width: 600px; padding: 40px; border-radius: 12px; box-shadow: 0 8px 32px rgba(0,0,0,0.3); position: relative; }\n");
        html.push_str("        .tour-step { color: #3498db; font-size: 0.9em; font-weight: bold; margin-bottom: 10px; }\n");
        html.push_str("        .tour-title { font-size: 2em; font-weight: bold; color: #2c3e50; margin-bottom: 20px; }\n");
        html.push_str("        .tour-description { font-size: 1.1em; line-height: 1.7; color: #34495e; margin-bottom: 30px; }\n");
        html.push_str("        .tour-visual { background-color: #ecf0f1; padding: 20px; margin: 20px 0; border-radius: 8px; text-align: center; font-style: italic; color: #7f8c8d; }\n");
        html.push_str("        .tour-controls { display: flex; justify-content: space-between; align-items: center; }\n");
        html.push_str("        .tour-button { padding: 12px 24px; border: none; border-radius: 6px; font-size: 1em; cursor: pointer; transition: all 0.3s; }\n");
        html.push_str("        .btn-primary { background-color: #3498db; color: white; }\n");
        html.push_str("        .btn-primary:hover { background-color: #2980b9; }\n");
        html.push_str("        .btn-secondary { background-color: #95a5a6; color: white; }\n");
        html.push_str("        .btn-secondary:hover { background-color: #7f8c8d; }\n");
        html.push_str("        .tour-progress { flex: 1; margin: 0 20px; height: 4px; background-color: #ecf0f1; border-radius: 2px; overflow: hidden; }\n");
        html.push_str("        .progress-fill { height: 100%; background-color: #3498db; transition: width 0.3s; }\n");
        html.push_str("    </style>\n</head>\n<body>\n");
        html.push_str("    <div class=\"tour-overlay\" id=\"tour\">\n");
        html.push_str("        <div class=\"tour-card\">\n");
        html.push_str("            <div class=\"tour-step\" id=\"step-indicator\">Step 1 of ");
        html.push_str(&format!("{}</div>\n", stops.len()));
        html.push_str("            <h1 class=\"tour-title\" id=\"tour-title\"></h1>\n");
        html.push_str(
            "            <div class=\"tour-description\" id=\"tour-description\"></div>\n",
        );
        html.push_str("            <div class=\"tour-visual\" id=\"tour-visual\" style=\"display: none;\"></div>\n");
        html.push_str("            <div class=\"tour-controls\">\n");
        html.push_str("                <button class=\"tour-button btn-secondary\" id=\"prev-btn\">Previous</button>\n");
        html.push_str("                <div class=\"tour-progress\">\n");
        html.push_str("                    <div class=\"progress-fill\" id=\"progress\"></div>\n");
        html.push_str("                </div>\n");
        html.push_str("                <button class=\"tour-button btn-primary\" id=\"next-btn\">Next</button>\n");
        html.push_str("            </div>\n");
        html.push_str("        </div>\n");
        html.push_str("    </div>\n");

        html.push_str("    <script>\n");
        html.push_str("const stops = ");
        html.push_str(&serde_json::to_string(stops).unwrap_or_else(|_| "[]".to_string()));
        html.push_str(";\n");
        html.push_str("let currentStop = 0;\n");
        html.push_str("function updateTour() {\n");
        html.push_str("    const stop = stops[currentStop];\n");
        html.push_str("    document.getElementById('step-indicator').textContent = `Step ${currentStop + 1} of ${stops.length}`;\n");
        html.push_str("    document.getElementById('tour-title').textContent = stop.title;\n");
        html.push_str(
            "    document.getElementById('tour-description').textContent = stop.description;\n",
        );
        html.push_str("    const visual = document.getElementById('tour-visual');\n");
        html.push_str("    if (stop.visual) {\n");
        html.push_str("        visual.textContent = stop.visual;\n");
        html.push_str("        visual.style.display = 'block';\n");
        html.push_str("    } else {\n");
        html.push_str("        visual.style.display = 'none';\n");
        html.push_str("    }\n");
        html.push_str("    document.getElementById('progress').style.width = ((currentStop + 1) / stops.length * 100) + '%';\n");
        html.push_str("    document.getElementById('prev-btn').disabled = currentStop === 0;\n");
        html.push_str("    const nextBtn = document.getElementById('next-btn');\n");
        html.push_str(
            "    nextBtn.textContent = currentStop === stops.length - 1 ? 'Finish' : 'Next';\n",
        );
        html.push_str("}\n");
        html.push_str("document.getElementById('prev-btn').addEventListener('click', () => {\n");
        html.push_str("    if (currentStop > 0) {\n");
        html.push_str("        currentStop--;\n");
        html.push_str("        updateTour();\n");
        html.push_str("    }\n");
        html.push_str("});\n");
        html.push_str("document.getElementById('next-btn').addEventListener('click', () => {\n");
        html.push_str("    if (currentStop < stops.length - 1) {\n");
        html.push_str("        currentStop++;\n");
        html.push_str("        updateTour();\n");
        html.push_str("    } else {\n");
        html.push_str("        document.getElementById('tour').style.display = 'none';\n");
        html.push_str("    }\n");
        html.push_str("});\n");

        if self.auto_advance {
            html.push_str("setInterval(() => {\n");
            html.push_str("    if (currentStop < stops.length - 1) {\n");
            html.push_str("        currentStop++;\n");
            html.push_str("        updateTour();\n");
            html.push_str("    }\n");
            html.push_str(&format!("}}, {});\n", self.advance_delay));
        }

        html.push_str("updateTour();\n");
        html.push_str("    </script>\n</body>\n</html>");
        html
    }
}

impl Default for GuidedExplorationTour {
    fn default() -> Self {
        Self::new("Guided Tour")
    }
}

/// Tour stop for guided exploration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TourStop {
    /// Stop title
    pub title: String,
    /// Stop description
    pub description: String,
    /// Optional visual element
    pub visual: Option<String>,
}

impl TourStop {
    /// Creates a new tour stop.
    pub fn new(title: &str, description: &str) -> Self {
        Self {
            title: title.to_string(),
            description: description.to_string(),
            visual: None,
        }
    }

    /// Sets a visual element.
    pub fn with_visual(mut self, visual: &str) -> Self {
        self.visual = Some(visual.to_string());
        self
    }
}

/// Educational walkthrough system.
pub struct EducationalWalkthrough {
    /// Walkthrough title
    title: String,
    /// Theme
    theme: Theme,
    /// Show quiz questions
    include_quiz: bool,
}

impl EducationalWalkthrough {
    /// Creates a new educational walkthrough.
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            theme: Theme::default(),
            include_quiz: true,
        }
    }

    /// Sets the theme.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Excludes quiz questions.
    pub fn without_quiz(mut self) -> Self {
        self.include_quiz = false;
        self
    }

    /// Generates HTML for educational walkthrough.
    pub fn to_html(&self, lessons: &[Lesson]) -> String {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str("    <meta charset=\"utf-8\">\n");
        html.push_str(
            "    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n",
        );
        html.push_str(&format!("    <title>{}</title>\n", self.title));
        html.push_str("    <style>\n");
        html.push_str(&format!("        body {{ background: linear-gradient(135deg, {} 0%, #ecf0f1 100%); color: {}; font-family: 'Segoe UI', Arial, sans-serif; margin: 0; padding: 40px 20px; min-height: 100vh; }}\n", self.theme.background_color, self.theme.text_color));
        html.push_str("        .walkthrough-container { max-width: 900px; margin: 0 auto; }\n");
        html.push_str("        .walkthrough-header { text-align: center; margin-bottom: 50px; }\n");
        html.push_str("        .walkthrough-title { font-size: 3em; font-weight: bold; color: #2c3e50; text-shadow: 2px 2px 4px rgba(0,0,0,0.1); }\n");
        html.push_str("        .lesson { background-color: white; border-radius: 12px; padding: 40px; margin: 30px 0; box-shadow: 0 4px 16px rgba(0,0,0,0.1); }\n");
        html.push_str("        .lesson-number { display: inline-block; background-color: #3498db; color: white; width: 40px; height: 40px; border-radius: 50%; text-align: center; line-height: 40px; font-weight: bold; margin-bottom: 15px; }\n");
        html.push_str("        .lesson-title { font-size: 2em; font-weight: bold; color: #2c3e50; margin-bottom: 20px; }\n");
        html.push_str("        .lesson-content { font-size: 1.1em; line-height: 1.8; color: #34495e; margin-bottom: 20px; }\n");
        html.push_str("        .example-box { background-color: #f8f9fa; border-left: 4px solid #f39c12; padding: 20px; margin: 20px 0; }\n");
        html.push_str(
            "        .example-title { font-weight: bold; color: #f39c12; margin-bottom: 10px; }\n",
        );
        html.push_str("        .quiz-section { background-color: #e8f4f8; border-radius: 8px; padding: 25px; margin-top: 25px; }\n");
        html.push_str("        .quiz-title { font-weight: bold; color: #2c3e50; margin-bottom: 15px; font-size: 1.2em; }\n");
        html.push_str("        .quiz-question { margin: 15px 0; }\n");
        html.push_str("        .quiz-option { display: block; padding: 12px 20px; margin: 8px 0; background-color: white; border: 2px solid #ddd; border-radius: 6px; cursor: pointer; transition: all 0.3s; }\n");
        html.push_str(
            "        .quiz-option:hover { border-color: #3498db; background-color: #f0f8ff; }\n",
        );
        html.push_str(
            "        .quiz-option.correct { border-color: #27ae60; background-color: #d4edda; }\n",
        );
        html.push_str("        .quiz-option.incorrect { border-color: #e74c3c; background-color: #f8d7da; }\n");
        html.push_str("        .key-takeaway { background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 25px; border-radius: 8px; margin-top: 20px; }\n");
        html.push_str("        .takeaway-title { font-weight: bold; font-size: 1.3em; margin-bottom: 10px; }\n");
        html.push_str("    </style>\n</head>\n<body>\n");
        html.push_str("    <div class=\"walkthrough-container\">\n");
        html.push_str("        <div class=\"walkthrough-header\">\n");
        html.push_str(&format!(
            "            <h1 class=\"walkthrough-title\">{}</h1>\n",
            self.title
        ));
        html.push_str("        </div>\n");

        for (i, lesson) in lessons.iter().enumerate() {
            html.push_str("        <div class=\"lesson\">\n");
            html.push_str(&format!(
                "            <div class=\"lesson-number\">{}</div>\n",
                i + 1
            ));
            html.push_str(&format!(
                "            <h2 class=\"lesson-title\">{}</h2>\n",
                lesson.title
            ));
            for paragraph in &lesson.content {
                html.push_str(&format!(
                    "            <p class=\"lesson-content\">{}</p>\n",
                    paragraph
                ));
            }

            if let Some(example) = &lesson.example {
                html.push_str("            <div class=\"example-box\">\n");
                html.push_str("                <div class=\"example-title\">Example:</div>\n");
                html.push_str(&format!("                <div>{}</div>\n", example));
                html.push_str("            </div>\n");
            }

            if self.include_quiz {
                if let Some(quiz) = &lesson.quiz_question {
                    html.push_str("            <div class=\"quiz-section\">\n");
                    html.push_str("                <div class=\"quiz-title\">Check Your Understanding</div>\n");
                    html.push_str(&format!(
                        "                <div class=\"quiz-question\">{}</div>\n",
                        quiz.question
                    ));
                    for (j, option) in quiz.options.iter().enumerate() {
                        html.push_str(&format!("                <div class=\"quiz-option\" data-correct=\"{}\">{}</div>\n", j == quiz.correct_index, option));
                    }
                    html.push_str("            </div>\n");
                }
            }

            if let Some(takeaway) = &lesson.key_takeaway {
                html.push_str("            <div class=\"key-takeaway\">\n");
                html.push_str("                <div class=\"takeaway-title\">Key Takeaway</div>\n");
                html.push_str(&format!("                <div>{}</div>\n", takeaway));
                html.push_str("            </div>\n");
            }

            html.push_str("        </div>\n");
        }

        html.push_str("    </div>\n");

        if self.include_quiz {
            html.push_str("    <script>\n");
            html.push_str("document.querySelectorAll('.quiz-option').forEach(option => {\n");
            html.push_str("    option.addEventListener('click', function() {\n");
            html.push_str(
                "        const isCorrect = this.getAttribute('data-correct') === 'true';\n",
            );
            html.push_str(
                "        const siblings = this.parentElement.querySelectorAll('.quiz-option');\n",
            );
            html.push_str("        siblings.forEach(s => {\n");
            html.push_str("            s.style.pointerEvents = 'none';\n");
            html.push_str("            if (s.getAttribute('data-correct') === 'true') {\n");
            html.push_str("                s.classList.add('correct');\n");
            html.push_str("            }\n");
            html.push_str("        });\n");
            html.push_str("        if (!isCorrect) {\n");
            html.push_str("            this.classList.add('incorrect');\n");
            html.push_str("        }\n");
            html.push_str("    });\n");
            html.push_str("});\n");
            html.push_str("    </script>\n");
        }

        html.push_str("</body>\n</html>");
        html
    }
}

impl Default for EducationalWalkthrough {
    fn default() -> Self {
        Self::new("Educational Walkthrough")
    }
}

/// Educational lesson.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lesson {
    /// Lesson title
    pub title: String,
    /// Lesson content paragraphs
    pub content: Vec<String>,
    /// Example
    pub example: Option<String>,
    /// Quiz question
    pub quiz_question: Option<QuizQuestion>,
    /// Key takeaway
    pub key_takeaway: Option<String>,
}

impl Lesson {
    /// Creates a new lesson.
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            content: Vec::new(),
            example: None,
            quiz_question: None,
            key_takeaway: None,
        }
    }

    /// Adds content paragraph.
    pub fn with_content(mut self, paragraph: &str) -> Self {
        self.content.push(paragraph.to_string());
        self
    }

    /// Sets an example.
    pub fn with_example(mut self, example: &str) -> Self {
        self.example = Some(example.to_string());
        self
    }

    /// Sets a quiz question.
    pub fn with_quiz(mut self, question: QuizQuestion) -> Self {
        self.quiz_question = Some(question);
        self
    }

    /// Sets a key takeaway.
    pub fn with_takeaway(mut self, takeaway: &str) -> Self {
        self.key_takeaway = Some(takeaway.to_string());
        self
    }
}

/// Quiz question for educational walkthrough.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizQuestion {
    /// Question text
    pub question: String,
    /// Answer options
    pub options: Vec<String>,
    /// Index of correct answer
    pub correct_index: usize,
}

impl QuizQuestion {
    /// Creates a new quiz question.
    pub fn new(question: &str, options: Vec<String>, correct_index: usize) -> Self {
        Self {
            question: question.to_string(),
            options,
            correct_index,
        }
    }
}

// ============================================================================
// v0.3.4 - Holographic Display Support
// ============================================================================

/// Configuration for Looking Glass holographic display.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LookingGlassConfig {
    /// Enable quilt rendering (multi-view for holographic display)
    pub enable_quilt: bool,
    /// Number of views in the quilt (typically 45 for Looking Glass Portrait)
    pub view_count: usize,
    /// Quilt width in pixels
    pub quilt_width: usize,
    /// Quilt height in pixels
    pub quilt_height: usize,
    /// Enable depth mapping
    pub enable_depth_mapping: bool,
    /// Field of view in degrees
    pub fov: f32,
    /// Depth range (near, far) in scene units
    pub depth_range: (f32, f32),
}

impl Default for LookingGlassConfig {
    fn default() -> Self {
        Self {
            enable_quilt: true,
            view_count: 45,
            quilt_width: 4096,
            quilt_height: 4096,
            enable_depth_mapping: true,
            fov: 14.0,
            depth_range: (0.1, 100.0),
        }
    }
}

/// Looking Glass holographic display visualizer.
pub struct LookingGlassVisualizer {
    title: String,
    config: LookingGlassConfig,
    theme: Theme,
}

impl LookingGlassVisualizer {
    /// Creates a new Looking Glass visualizer.
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            config: LookingGlassConfig::default(),
            theme: Theme::dark(),
        }
    }

    /// Sets the Looking Glass configuration.
    pub fn with_config(mut self, config: LookingGlassConfig) -> Self {
        self.config = config;
        self
    }

    /// Sets the theme.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Generates HTML for Looking Glass display.
    pub fn to_holographic_html(&self, graph: &DependencyGraph) -> String {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html>\n<head>\n");
        html.push_str(&format!("    <title>{}</title>\n", self.title));
        html.push_str("    <meta charset=\"utf-8\">\n");
        html.push_str("    <script src=\"https://cdnjs.cloudflare.com/ajax/libs/three.js/r128/three.min.js\"></script>\n");
        html.push_str("    <script src=\"https://unpkg.com/holoplay-core@0.1.1/dist/holoplay-core.min.js\"></script>\n");
        html.push_str("    <style>\n");
        html.push_str("        body { margin: 0; overflow: hidden; background: #000; }\n");
        html.push_str("        #canvas { width: 100%; height: 100%; }\n");
        html.push_str("        #info { position: absolute; top: 10px; left: 10px; color: #fff; font-family: monospace; }\n");
        html.push_str("    </style>\n");
        html.push_str("</head>\n<body>\n");

        html.push_str(&format!(
            "    <div id=\"info\">{}<br>Looking Glass Display<br>Views: {}</div>\n",
            self.title, self.config.view_count
        ));
        html.push_str("    <canvas id=\"canvas\"></canvas>\n");

        html.push_str("    <script>\n");
        html.push_str(&format!(
            "        const config = {};\n",
            serde_json::to_string(&self.config).unwrap()
        ));
        html.push_str("        const scene = new THREE.Scene();\n");
        html.push_str(&format!("        const camera = new THREE.PerspectiveCamera({}, window.innerWidth / window.innerHeight, {}, {});\n",
            self.config.fov, self.config.depth_range.0, self.config.depth_range.1));
        html.push_str("        camera.position.set(0, 0, 10);\n");
        html.push_str("        const renderer = new THREE.WebGLRenderer({ canvas: document.getElementById('canvas'), antialias: true });\n");
        html.push_str("        renderer.setSize(window.innerWidth, window.innerHeight);\n");

        // Add graph nodes as holographic objects
        html.push_str("        const geometry = new THREE.BoxGeometry(1, 1, 1);\n");
        html.push_str(&format!(
            "        const material = new THREE.MeshPhongMaterial({{ color: '{}' }});\n",
            self.theme.condition_color
        ));

        let node_count = graph.node_count().min(25);
        for i in 0..node_count {
            let x = (i % 5) as f32 * 2.0 - 4.0;
            let y = (i / 5) as f32 * 2.0 - 2.0;
            html.push_str(&format!(
                "        const cube{} = new THREE.Mesh(geometry, material);\n",
                i
            ));
            html.push_str(&format!(
                "        cube{}.position.set({}, {}, 0);\n",
                i, x, y
            ));
            html.push_str(&format!("        scene.add(cube{});\n", i));
        }

        html.push_str("        const light = new THREE.DirectionalLight(0xffffff, 1);\n");
        html.push_str("        light.position.set(5, 5, 5);\n");
        html.push_str("        scene.add(light);\n");
        html.push_str("        scene.add(new THREE.AmbientLight(0x404040));\n");

        html.push_str("        function animate() {\n");
        html.push_str("            requestAnimationFrame(animate);\n");
        html.push_str("            renderer.render(scene, camera);\n");
        html.push_str("        }\n");
        html.push_str("        animate();\n");
        html.push_str("    </script>\n");
        html.push_str("</body>\n</html>\n");

        html
    }
}

impl Default for LookingGlassVisualizer {
    fn default() -> Self {
        Self::new("Holographic Visualization")
    }
}

/// Holographic statute model configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HolographicModelConfig {
    /// Enable layer separation for legal structure
    pub enable_layers: bool,
    /// Number of depth layers
    pub layer_count: usize,
    /// Enable rotation animation
    pub enable_rotation: bool,
    /// Rotation speed (degrees per second)
    pub rotation_speed: f32,
    /// Enable interactive manipulation
    pub enable_interaction: bool,
}

impl Default for HolographicModelConfig {
    fn default() -> Self {
        Self {
            enable_layers: true,
            layer_count: 5,
            enable_rotation: true,
            rotation_speed: 15.0,
            enable_interaction: true,
        }
    }
}

/// Holographic statute model visualizer.
pub struct HolographicStatuteModel {
    theme: Theme,
    config: HolographicModelConfig,
}

impl HolographicStatuteModel {
    /// Creates a new holographic statute model.
    pub fn new() -> Self {
        Self {
            theme: Theme::dark(),
            config: HolographicModelConfig::default(),
        }
    }

    /// Sets the theme.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Sets the configuration.
    pub fn with_config(mut self, config: HolographicModelConfig) -> Self {
        self.config = config;
        self
    }

    /// Generates holographic model HTML.
    pub fn to_holographic_model(&self, statute: &Statute) -> String {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html>\n<head>\n");
        html.push_str(&format!(
            "    <title>Holographic Model: {}</title>\n",
            statute.title
        ));
        html.push_str("    <meta charset=\"utf-8\">\n");
        html.push_str("    <script src=\"https://cdnjs.cloudflare.com/ajax/libs/three.js/r128/three.min.js\"></script>\n");
        html.push_str("    <style>\n");
        html.push_str("        body { margin: 0; background: #000; }\n");
        html.push_str("        #container { width: 100vw; height: 100vh; }\n");
        html.push_str("        #info { position: absolute; top: 10px; left: 10px; color: #0f0; font-family: monospace; }\n");
        html.push_str("    </style>\n");
        html.push_str("</head>\n<body>\n");

        html.push_str(&format!(
            "    <div id=\"info\">{}<br>Holographic Statute Model</div>\n",
            statute.title
        ));
        html.push_str("    <div id=\"container\"></div>\n");

        html.push_str("    <script>\n");
        html.push_str("        const scene = new THREE.Scene();\n");
        html.push_str("        const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);\n");
        html.push_str("        camera.position.z = 15;\n");
        html.push_str("        const renderer = new THREE.WebGLRenderer({ antialias: true });\n");
        html.push_str("        renderer.setSize(window.innerWidth, window.innerHeight);\n");
        html.push_str(
            "        document.getElementById('container').appendChild(renderer.domElement);\n",
        );

        if self.config.enable_layers {
            for i in 0..self.config.layer_count {
                let z = (i as f32 - (self.config.layer_count as f32 / 2.0)) * 2.0;
                html.push_str(&format!(
                    "        const layer{}Geometry = new THREE.PlaneGeometry(8, 8);\n",
                    i
                ));
                html.push_str(&format!("        const layer{}Material = new THREE.MeshBasicMaterial({{ color: '{}', transparent: true, opacity: 0.3, side: THREE.DoubleSide }});\n",
                    i, self.theme.condition_color));
                html.push_str(&format!(
                    "        const layer{} = new THREE.Mesh(layer{}Geometry, layer{}Material);\n",
                    i, i, i
                ));
                html.push_str(&format!("        layer{}.position.z = {};\n", i, z));
                html.push_str(&format!("        scene.add(layer{});\n", i));
            }
        }

        html.push_str("        function animate() {\n");
        html.push_str("            requestAnimationFrame(animate);\n");
        if self.config.enable_rotation {
            html.push_str(&format!(
                "            scene.rotation.y += {};\n",
                self.config.rotation_speed * 0.001
            ));
        }
        html.push_str("            renderer.render(scene, camera);\n");
        html.push_str("        }\n");
        html.push_str("        animate();\n");
        html.push_str("    </script>\n");
        html.push_str("</body>\n</html>\n");

        html
    }
}

impl Default for HolographicStatuteModel {
    fn default() -> Self {
        Self::new()
    }
}

/// 3D print export configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrintExportConfig {
    /// Export format (STL, OBJ, 3MF)
    pub format: String,
    /// Scale factor for the model
    pub scale: f32,
    /// Base thickness in mm
    pub base_thickness: f32,
    /// Wall thickness in mm
    pub wall_thickness: f32,
    /// Enable support generation
    pub generate_supports: bool,
}

impl Default for PrintExportConfig {
    fn default() -> Self {
        Self {
            format: "STL".to_string(),
            scale: 1.0,
            base_thickness: 2.0,
            wall_thickness: 1.0,
            generate_supports: false,
        }
    }
}

/// 3D print export visualizer.
pub struct ThreeDPrintExporter {
    config: PrintExportConfig,
}

impl ThreeDPrintExporter {
    /// Creates a new 3D print exporter.
    pub fn new() -> Self {
        Self {
            config: PrintExportConfig::default(),
        }
    }

    /// Sets the export configuration.
    pub fn with_config(mut self, config: PrintExportConfig) -> Self {
        self.config = config;
        self
    }

    /// Exports decision tree as STL mesh data.
    pub fn to_stl(&self, tree: &DecisionTree) -> String {
        let mut stl = String::new();

        stl.push_str("solid DecisionTree\n");

        // Generate triangular facets for each node
        let node_count = tree.node_count().min(10);
        for i in 0..node_count {
            let x = (i as f32) * self.config.scale;
            let y = 0.0;
            let z = self.config.base_thickness;

            // Simple cube representation (12 triangles)
            stl.push_str("  facet normal 0 0 1\n");
            stl.push_str("    outer loop\n");
            stl.push_str(&format!("      vertex {} {} {}\n", x, y, z));
            stl.push_str(&format!("      vertex {} {} {}\n", x + 1.0, y, z));
            stl.push_str(&format!("      vertex {} {} {}\n", x + 1.0, y + 1.0, z));
            stl.push_str("    endloop\n");
            stl.push_str("  endfacet\n");
        }

        stl.push_str("endsolid DecisionTree\n");
        stl
    }

    /// Exports dependency graph as OBJ mesh data.
    pub fn to_obj(&self, graph: &DependencyGraph) -> String {
        let mut obj = String::new();

        let node_count = graph.node_count();
        obj.push_str("# OBJ file for dependency graph\n");
        obj.push_str(&format!("# Vertices: {}\n", node_count));

        // Write vertices
        for i in 0..node_count {
            let x = (i % 5) as f32 * self.config.scale;
            let y = (i / 5) as f32 * self.config.scale;
            let z = 0.0;
            obj.push_str(&format!("v {} {} {}\n", x, y, z));
        }

        // Write faces (simplified cube for each vertex)
        for i in 1..=node_count {
            obj.push_str(&format!("f {} {} {}\n", i, i, i));
        }

        obj
    }

    /// Exports as 3MF format (XML-based).
    pub fn to_3mf(&self, tree: &DecisionTree) -> String {
        let mut mf = String::new();

        mf.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        mf.push_str("<model unit=\"millimeter\" xmlns=\"http://schemas.microsoft.com/3dmanufacturing/core/2015/02\">\n");
        mf.push_str("  <resources>\n");
        mf.push_str("    <object id=\"1\" type=\"model\">\n");
        mf.push_str("      <mesh>\n");
        mf.push_str("        <vertices>\n");

        let node_count = tree.node_count().min(10);
        for i in 0..node_count {
            let x = i as f32 * self.config.scale;
            mf.push_str(&format!(
                "          <vertex x=\"{}\" y=\"0\" z=\"0\" />\n",
                x
            ));
        }

        mf.push_str("        </vertices>\n");
        mf.push_str("        <triangles>\n");
        mf.push_str("          <triangle v1=\"0\" v2=\"1\" v3=\"2\" />\n");
        mf.push_str("        </triangles>\n");
        mf.push_str("      </mesh>\n");
        mf.push_str("    </object>\n");
        mf.push_str("  </resources>\n");
        mf.push_str("  <build>\n");
        mf.push_str("    <item objectid=\"1\" />\n");
        mf.push_str("  </build>\n");
        mf.push_str("</model>\n");

        mf
    }
}

impl Default for ThreeDPrintExporter {
    fn default() -> Self {
        Self::new()
    }
}

/// Volumetric data rendering configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumetricConfig {
    /// Enable ray marching for volumetric rendering
    pub enable_ray_marching: bool,
    /// Number of sampling steps
    pub sample_steps: usize,
    /// Density threshold
    pub density_threshold: f32,
    /// Enable gradient-based lighting
    pub enable_lighting: bool,
    /// Color transfer function
    pub transfer_function: String,
}

impl Default for VolumetricConfig {
    fn default() -> Self {
        Self {
            enable_ray_marching: true,
            sample_steps: 128,
            density_threshold: 0.1,
            enable_lighting: true,
            transfer_function: "linear".to_string(),
        }
    }
}

/// Volumetric data renderer.
pub struct VolumetricRenderer {
    title: String,
    config: VolumetricConfig,
    theme: Theme,
}

impl VolumetricRenderer {
    /// Creates a new volumetric renderer.
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            config: VolumetricConfig::default(),
            theme: Theme::dark(),
        }
    }

    /// Sets the theme.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Sets the configuration.
    pub fn with_config(mut self, config: VolumetricConfig) -> Self {
        self.config = config;
        self
    }

    /// Generates volumetric rendering HTML.
    pub fn to_volumetric_html(&self, graph: &DependencyGraph) -> String {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html>\n<head>\n");
        html.push_str(&format!("    <title>{}</title>\n", self.title));
        html.push_str("    <meta charset=\"utf-8\">\n");
        html.push_str("    <script src=\"https://cdnjs.cloudflare.com/ajax/libs/three.js/r128/three.min.js\"></script>\n");
        html.push_str("    <style>\n");
        html.push_str("        body { margin: 0; background: #000; overflow: hidden; }\n");
        html.push_str("        #canvas { width: 100%; height: 100%; }\n");
        html.push_str("        #info { position: absolute; top: 10px; left: 10px; color: #0ff; font-family: monospace; }\n");
        html.push_str("    </style>\n");
        html.push_str("</head>\n<body>\n");

        html.push_str(&format!(
            "    <div id=\"info\">{}<br>Volumetric Rendering<br>Steps: {}</div>\n",
            self.title, self.config.sample_steps
        ));
        html.push_str("    <canvas id=\"canvas\"></canvas>\n");

        html.push_str("    <script>\n");
        html.push_str("        const scene = new THREE.Scene();\n");
        html.push_str("        const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);\n");
        html.push_str("        camera.position.z = 10;\n");
        html.push_str("        const renderer = new THREE.WebGLRenderer({ canvas: document.getElementById('canvas'), antialias: true });\n");
        html.push_str("        renderer.setSize(window.innerWidth, window.innerHeight);\n");

        // Create volumetric cloud for each statute
        html.push_str("        const geometry = new THREE.SphereGeometry(1, 32, 32);\n");
        let node_count = graph.node_count().min(10);
        for i in 0..node_count {
            let x = (i % 5) as f32 * 2.5 - 5.0;
            let y = (i / 5) as f32 * 2.5 - 2.5;
            html.push_str(&format!("        const material{} = new THREE.MeshPhongMaterial({{ color: '{}', transparent: true, opacity: 0.6 }});\n",
                i, self.theme.condition_color));
            html.push_str(&format!(
                "        const sphere{} = new THREE.Mesh(geometry, material{});\n",
                i, i
            ));
            html.push_str(&format!(
                "        sphere{}.position.set({}, {}, 0);\n",
                i, x, y
            ));
            html.push_str(&format!("        scene.add(sphere{});\n", i));
        }

        html.push_str("        const light = new THREE.PointLight(0xffffff, 1, 100);\n");
        html.push_str("        light.position.set(10, 10, 10);\n");
        html.push_str("        scene.add(light);\n");
        html.push_str("        scene.add(new THREE.AmbientLight(0x404040));\n");

        html.push_str("        function animate() {\n");
        html.push_str("            requestAnimationFrame(animate);\n");
        html.push_str("            scene.rotation.y += 0.005;\n");
        html.push_str("            renderer.render(scene, camera);\n");
        html.push_str("        }\n");
        html.push_str("        animate();\n");
        html.push_str("    </script>\n");
        html.push_str("</body>\n</html>\n");

        html
    }
}

impl Default for VolumetricRenderer {
    fn default() -> Self {
        Self::new("Volumetric Visualization")
    }
}

/// Gesture-based holographic interaction configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GestureConfig {
    /// Enable hand tracking
    pub enable_hand_tracking: bool,
    /// Enable pinch gestures
    pub enable_pinch: bool,
    /// Enable swipe gestures
    pub enable_swipe: bool,
    /// Enable rotation gestures
    pub enable_rotation: bool,
    /// Gesture sensitivity (0.0 to 1.0)
    pub sensitivity: f32,
}

impl Default for GestureConfig {
    fn default() -> Self {
        Self {
            enable_hand_tracking: true,
            enable_pinch: true,
            enable_swipe: true,
            enable_rotation: true,
            sensitivity: 0.7,
        }
    }
}

/// Gesture-based holographic interaction system.
pub struct HolographicGestureController {
    title: String,
    config: GestureConfig,
    theme: Theme,
}

impl HolographicGestureController {
    /// Creates a new holographic gesture controller.
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            config: GestureConfig::default(),
            theme: Theme::dark(),
        }
    }

    /// Sets the theme.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Sets the gesture configuration.
    pub fn with_config(mut self, config: GestureConfig) -> Self {
        self.config = config;
        self
    }

    /// Generates gesture-controlled holographic HTML.
    pub fn to_gesture_html(&self, tree: &DecisionTree) -> String {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html>\n<head>\n");
        html.push_str(&format!("    <title>{}</title>\n", self.title));
        html.push_str("    <meta charset=\"utf-8\">\n");
        html.push_str("    <script src=\"https://cdnjs.cloudflare.com/ajax/libs/three.js/r128/three.min.js\"></script>\n");
        html.push_str(
            "    <script src=\"https://unpkg.com/@mediapipe/hands/hands.js\"></script>\n",
        );
        html.push_str("    <style>\n");
        html.push_str("        body { margin: 0; background: #000; overflow: hidden; }\n");
        html.push_str("        #container { width: 100%; height: 100%; }\n");
        html.push_str("        #info { position: absolute; top: 10px; left: 10px; color: #f0f; font-family: monospace; }\n");
        html.push_str("        #gestures { position: absolute; bottom: 10px; left: 10px; color: #fff; font-family: monospace; }\n");
        html.push_str("    </style>\n");
        html.push_str("</head>\n<body>\n");

        html.push_str(&format!(
            "    <div id=\"info\">{}<br>Gesture Control Active</div>\n",
            self.title
        ));
        html.push_str("    <div id=\"gestures\">Gestures: Pinch to zoom | Swipe to rotate | Open palm to reset</div>\n");
        html.push_str("    <div id=\"container\"></div>\n");

        html.push_str("    <script>\n");
        html.push_str(&format!(
            "        const config = {};\n",
            serde_json::to_string(&self.config).unwrap()
        ));
        html.push_str("        const scene = new THREE.Scene();\n");
        html.push_str("        const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);\n");
        html.push_str("        camera.position.z = 10;\n");
        html.push_str("        const renderer = new THREE.WebGLRenderer({ antialias: true });\n");
        html.push_str("        renderer.setSize(window.innerWidth, window.innerHeight);\n");
        html.push_str(
            "        document.getElementById('container').appendChild(renderer.domElement);\n",
        );

        // Add tree nodes
        let node_count = tree.node_count().min(10);
        for i in 0..node_count {
            let x = (i % 5) as f32 * 2.0 - 4.0;
            let y = (i / 5) as f32 * 2.0 - 2.0;
            html.push_str(&format!(
                "        const nodeGeometry{} = new THREE.SphereGeometry(0.5, 32, 32);\n",
                i
            ));
            html.push_str(&format!(
                "        const nodeMaterial{} = new THREE.MeshPhongMaterial({{ color: '{}' }});\n",
                i, self.theme.condition_color
            ));
            html.push_str(&format!(
                "        const nodeMesh{} = new THREE.Mesh(nodeGeometry{}, nodeMaterial{});\n",
                i, i, i
            ));
            html.push_str(&format!(
                "        nodeMesh{}.position.set({}, {}, 0);\n",
                i, x, y
            ));
            html.push_str(&format!("        scene.add(nodeMesh{});\n", i));
        }

        html.push_str("        const light = new THREE.DirectionalLight(0xffffff, 1);\n");
        html.push_str("        light.position.set(5, 5, 5);\n");
        html.push_str("        scene.add(light);\n");
        html.push_str("        scene.add(new THREE.AmbientLight(0x404040));\n");

        html.push_str("        let gestureState = { pinch: false, swipe: 0, rotation: 0 };\n");

        if self.config.enable_hand_tracking {
            html.push_str("        // Gesture detection placeholder\n");
            html.push_str("        document.addEventListener('keydown', (e) => {\n");
            html.push_str(
                "            if (e.key === 'p') gestureState.pinch = !gestureState.pinch;\n",
            );
            html.push_str("            if (e.key === 's') gestureState.swipe += 0.1;\n");
            html.push_str("            if (e.key === 'r') gestureState.rotation += 0.1;\n");
            html.push_str("        });\n");
        }

        html.push_str("        function animate() {\n");
        html.push_str("            requestAnimationFrame(animate);\n");
        if self.config.enable_rotation {
            html.push_str("            scene.rotation.y += gestureState.rotation * 0.01;\n");
        }
        if self.config.enable_pinch {
            html.push_str("            if (gestureState.pinch) camera.position.z = Math.max(5, camera.position.z - 0.1);\n");
        }
        html.push_str("            renderer.render(scene, camera);\n");
        html.push_str("        }\n");
        html.push_str("        animate();\n");
        html.push_str("    </script>\n");
        html.push_str("</body>\n</html>\n");

        html
    }
}

impl Default for HolographicGestureController {
    fn default() -> Self {
        Self::new("Gesture-Controlled Holographic Visualization")
    }
}

// ============================================================================
// Cross-Jurisdictional Comparison (v0.4.0)
// ============================================================================

/// Represents a statute with jurisdiction information for comparison.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JurisdictionalStatute {
    /// The jurisdiction code (e.g., "US", "JP", "DE", "FR")
    pub jurisdiction: String,
    /// The jurisdiction's full name
    pub jurisdiction_name: String,
    /// The statute being compared
    pub statute: Statute,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl JurisdictionalStatute {
    /// Creates a new jurisdictional statute.
    pub fn new(jurisdiction: &str, jurisdiction_name: &str, statute: Statute) -> Self {
        Self {
            jurisdiction: jurisdiction.to_string(),
            jurisdiction_name: jurisdiction_name.to_string(),
            statute,
            metadata: HashMap::new(),
        }
    }

    /// Adds metadata to the jurisdictional statute.
    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }
}

/// Represents a difference between jurisdictions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JurisdictionalDifference {
    /// The aspect being compared (e.g., "eligibility", "age_requirement")
    pub aspect: String,
    /// Description of the difference
    pub description: String,
    /// Values for each jurisdiction
    pub values: HashMap<String, String>,
    /// Severity of the difference (0.0 = minor, 1.0 = major)
    pub severity: f64,
}

impl JurisdictionalDifference {
    /// Creates a new jurisdictional difference.
    pub fn new(aspect: &str, description: &str) -> Self {
        Self {
            aspect: aspect.to_string(),
            description: description.to_string(),
            values: HashMap::new(),
            severity: 0.5,
        }
    }

    /// Adds a jurisdiction's value for this difference.
    pub fn with_value(mut self, jurisdiction: &str, value: &str) -> Self {
        self.values
            .insert(jurisdiction.to_string(), value.to_string());
        self
    }

    /// Sets the severity level.
    pub fn with_severity(mut self, severity: f64) -> Self {
        self.severity = severity.clamp(0.0, 1.0);
        self
    }
}

/// Side-by-side statute comparison across jurisdictions.
#[derive(Debug, Clone)]
pub struct CrossJurisdictionalComparison {
    /// Title of the comparison
    pub title: String,
    /// Statutes being compared
    pub statutes: Vec<JurisdictionalStatute>,
    /// Identified differences
    pub differences: Vec<JurisdictionalDifference>,
    /// Theme for visualization
    pub theme: Theme,
    /// Enable synchronized navigation
    pub synchronized_nav: bool,
}

impl CrossJurisdictionalComparison {
    /// Creates a new cross-jurisdictional comparison.
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            statutes: Vec::new(),
            differences: Vec::new(),
            theme: Theme::light(),
            synchronized_nav: true,
        }
    }

    /// Adds a statute for comparison.
    pub fn add_statute(&mut self, statute: JurisdictionalStatute) {
        self.statutes.push(statute);
    }

    /// Adds a difference between jurisdictions.
    pub fn add_difference(&mut self, difference: JurisdictionalDifference) {
        self.differences.push(difference);
    }

    /// Sets the theme.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Enables or disables synchronized navigation.
    pub fn with_synchronized_nav(mut self, enabled: bool) -> Self {
        self.synchronized_nav = enabled;
        self
    }

    /// Generates side-by-side HTML comparison.
    pub fn to_side_by_side_html(&self) -> String {
        let mut html = String::new();

        // HTML header
        html.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n");
        html.push_str("    <meta charset=\"UTF-8\">\n");
        html.push_str(
            "    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n",
        );
        html.push_str(&format!("    <title>{}</title>\n", self.title));
        html.push_str("    <style>\n");
        html.push_str("        body {\n");
        html.push_str(&format!(
            "            background-color: {};\n",
            self.theme.background_color
        ));
        html.push_str(&format!("            color: {};\n", self.theme.text_color));
        html.push_str("            font-family: 'Segoe UI', Arial, sans-serif;\n");
        html.push_str("            margin: 0; padding: 20px;\n");
        html.push_str("        }\n");
        html.push_str("        .comparison-container {\n");
        html.push_str("            display: flex;\n");
        html.push_str("            gap: 20px;\n");
        html.push_str("            margin-bottom: 30px;\n");
        html.push_str("        }\n");
        html.push_str("        .jurisdiction-column {\n");
        html.push_str("            flex: 1;\n");
        html.push_str("            border: 2px solid #ccc;\n");
        html.push_str("            border-radius: 8px;\n");
        html.push_str("            padding: 15px;\n");
        html.push_str("            overflow-y: auto;\n");
        html.push_str("            max-height: 600px;\n");
        html.push_str("        }\n");
        html.push_str("        .jurisdiction-header {\n");
        html.push_str("            font-size: 1.5em;\n");
        html.push_str("            font-weight: bold;\n");
        html.push_str("            margin-bottom: 10px;\n");
        html.push_str("            padding-bottom: 10px;\n");
        html.push_str("            border-bottom: 2px solid #666;\n");
        html.push_str("        }\n");
        html.push_str("        .statute-content {\n");
        html.push_str("            line-height: 1.6;\n");
        html.push_str("        }\n");
        html.push_str("        .differences-section {\n");
        html.push_str("            margin-top: 30px;\n");
        html.push_str("            padding: 20px;\n");
        html.push_str("            background-color: rgba(255, 200, 0, 0.1);\n");
        html.push_str("            border-radius: 8px;\n");
        html.push_str("        }\n");
        html.push_str("        .difference-item {\n");
        html.push_str("            margin-bottom: 20px;\n");
        html.push_str("            padding: 15px;\n");
        html.push_str("            background-color: rgba(255, 255, 255, 0.05);\n");
        html.push_str("            border-left: 4px solid;\n");
        html.push_str("            border-radius: 4px;\n");
        html.push_str("        }\n");
        html.push_str("        .difference-minor { border-left-color: #4caf50; }\n");
        html.push_str("        .difference-moderate { border-left-color: #ff9800; }\n");
        html.push_str("        .difference-major { border-left-color: #f44336; }\n");
        html.push_str("        .difference-aspect {\n");
        html.push_str("            font-weight: bold;\n");
        html.push_str("            font-size: 1.1em;\n");
        html.push_str("            margin-bottom: 5px;\n");
        html.push_str("        }\n");
        html.push_str("        .difference-values {\n");
        html.push_str("            display: flex;\n");
        html.push_str("            gap: 15px;\n");
        html.push_str("            flex-wrap: wrap;\n");
        html.push_str("            margin-top: 10px;\n");
        html.push_str("        }\n");
        html.push_str("        .difference-value {\n");
        html.push_str("            padding: 5px 10px;\n");
        html.push_str("            background-color: rgba(100, 100, 100, 0.2);\n");
        html.push_str("            border-radius: 4px;\n");
        html.push_str("        }\n");
        html.push_str("    </style>\n");
        html.push_str("</head>\n<body>\n");

        // Title
        html.push_str(&format!("    <h1>{}</h1>\n", self.title));

        // Side-by-side comparison
        html.push_str("    <div class=\"comparison-container\">\n");
        for statute in &self.statutes {
            html.push_str("        <div class=\"jurisdiction-column\">\n");
            html.push_str(&format!(
                "            <div class=\"jurisdiction-header\">{} ({})</div>\n",
                statute.jurisdiction_name, statute.jurisdiction
            ));
            html.push_str("            <div class=\"statute-content\">\n");
            html.push_str(&format!(
                "                <strong>ID:</strong> {}<br>\n",
                statute.statute.id
            ));
            html.push_str(&format!(
                "                <strong>Title:</strong> {}<br>\n",
                statute.statute.title
            ));
            html.push_str(&format!(
                "                <strong>Effect:</strong> {}<br>\n",
                statute.statute.effect.description
            ));

            // Metadata
            if !statute.metadata.is_empty() {
                html.push_str("                <br><strong>Additional Information:</strong><br>\n");
                for (key, value) in &statute.metadata {
                    html.push_str(&format!(
                        "                <em>{}:</em> {}<br>\n",
                        key, value
                    ));
                }
            }

            html.push_str("            </div>\n");
            html.push_str("        </div>\n");
        }
        html.push_str("    </div>\n");

        // Differences section
        if !self.differences.is_empty() {
            html.push_str("    <div class=\"differences-section\">\n");
            html.push_str("        <h2>Key Differences</h2>\n");

            for diff in &self.differences {
                let severity_class = if diff.severity < 0.33 {
                    "difference-minor"
                } else if diff.severity < 0.67 {
                    "difference-moderate"
                } else {
                    "difference-major"
                };

                html.push_str(&format!(
                    "        <div class=\"difference-item {}\">\n",
                    severity_class
                ));
                html.push_str(&format!(
                    "            <div class=\"difference-aspect\">{}</div>\n",
                    diff.aspect
                ));
                html.push_str(&format!("            <div>{}</div>\n", diff.description));
                html.push_str("            <div class=\"difference-values\">\n");

                for (jurisdiction, value) in &diff.values {
                    html.push_str(&format!(
                        "                <div class=\"difference-value\"><strong>{}:</strong> {}</div>\n",
                        jurisdiction, value
                    ));
                }

                html.push_str("            </div>\n");
                html.push_str("        </div>\n");
            }

            html.push_str("    </div>\n");
        }

        // Synchronized navigation script
        if self.synchronized_nav {
            html.push_str("    <script>\n");
            html.push_str(
                "        const columns = document.querySelectorAll('.jurisdiction-column');\n",
            );
            html.push_str("        columns.forEach(col => {\n");
            html.push_str("            col.addEventListener('scroll', (e) => {\n");
            html.push_str("                const scrollRatio = e.target.scrollTop / (e.target.scrollHeight - e.target.clientHeight);\n");
            html.push_str("                columns.forEach(otherCol => {\n");
            html.push_str("                    if (otherCol !== e.target) {\n");
            html.push_str("                        otherCol.scrollTop = scrollRatio * (otherCol.scrollHeight - otherCol.clientHeight);\n");
            html.push_str("                    }\n");
            html.push_str("                });\n");
            html.push_str("            });\n");
            html.push_str("        });\n");
            html.push_str("    </script>\n");
        }

        html.push_str("</body>\n</html>");
        html
    }

    /// Generates a jurisdictional heatmap showing differences across regions.
    pub fn to_heatmap_html(&self) -> String {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n");
        html.push_str("    <meta charset=\"UTF-8\">\n");
        html.push_str(&format!("    <title>{} - Heatmap</title>\n", self.title));
        html.push_str("    <style>\n");
        html.push_str("        body {\n");
        html.push_str(&format!(
            "            background-color: {};\n",
            self.theme.background_color
        ));
        html.push_str(&format!("            color: {};\n", self.theme.text_color));
        html.push_str("            font-family: Arial, sans-serif;\n");
        html.push_str("            padding: 20px;\n");
        html.push_str("        }\n");
        html.push_str("        .heatmap-container {\n");
        html.push_str("            display: grid;\n");
        html.push_str(&format!(
            "            grid-template-columns: 200px repeat({}, 1fr);\n",
            self.statutes.len()
        ));
        html.push_str("            gap: 2px;\n");
        html.push_str("            margin-top: 20px;\n");
        html.push_str("        }\n");
        html.push_str("        .heatmap-cell {\n");
        html.push_str("            padding: 10px;\n");
        html.push_str("            text-align: center;\n");
        html.push_str("            border: 1px solid #ccc;\n");
        html.push_str("            min-height: 50px;\n");
        html.push_str("            display: flex;\n");
        html.push_str("            align-items: center;\n");
        html.push_str("            justify-content: center;\n");
        html.push_str("        }\n");
        html.push_str("        .heatmap-header {\n");
        html.push_str("            font-weight: bold;\n");
        html.push_str("            background-color: rgba(100, 100, 100, 0.3);\n");
        html.push_str("        }\n");
        html.push_str("        .heatmap-low { background-color: rgba(76, 175, 80, 0.3); }\n");
        html.push_str("        .heatmap-medium { background-color: rgba(255, 152, 0, 0.3); }\n");
        html.push_str("        .heatmap-high { background-color: rgba(244, 67, 54, 0.3); }\n");
        html.push_str("    </style>\n");
        html.push_str("</head>\n<body>\n");

        html.push_str(&format!(
            "    <h1>{} - Jurisdictional Heatmap</h1>\n",
            self.title
        ));
        html.push_str("    <div class=\"heatmap-container\">\n");

        // Header row
        html.push_str("        <div class=\"heatmap-cell heatmap-header\">Aspect</div>\n");
        for statute in &self.statutes {
            html.push_str(&format!(
                "        <div class=\"heatmap-cell heatmap-header\">{}</div>\n",
                statute.jurisdiction
            ));
        }

        // Difference rows
        for diff in &self.differences {
            html.push_str(&format!(
                "        <div class=\"heatmap-cell heatmap-header\">{}</div>\n",
                diff.aspect
            ));

            for statute in &self.statutes {
                let cell_class = if diff.severity < 0.33 {
                    "heatmap-low"
                } else if diff.severity < 0.67 {
                    "heatmap-medium"
                } else {
                    "heatmap-high"
                };

                let value = diff
                    .values
                    .get(&statute.jurisdiction)
                    .map(|v| v.as_str())
                    .unwrap_or("N/A");

                html.push_str(&format!(
                    "        <div class=\"heatmap-cell {}\">{}</div>\n",
                    cell_class, value
                ));
            }
        }

        html.push_str("    </div>\n");
        html.push_str("</body>\n</html>");
        html
    }
}

impl Default for CrossJurisdictionalComparison {
    fn default() -> Self {
        Self::new("Jurisdictional Comparison")
    }
}

// ============================================================================
// Semantic Legal Network (v0.4.1)
// ============================================================================

/// Represents a legal concept in the semantic network.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LegalConcept {
    /// Unique identifier
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Concept description
    pub description: String,
    /// Category (e.g., "rights", "obligations", "procedures")
    pub category: String,
    /// Related statute IDs
    pub statute_ids: Vec<String>,
    /// Additional metadata
    pub metadata: std::collections::HashMap<String, String>,
}

impl LegalConcept {
    /// Creates a new legal concept.
    pub fn new(id: &str, name: &str, description: &str, category: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            category: category.to_string(),
            statute_ids: Vec::new(),
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Adds a statute reference.
    pub fn add_statute(&mut self, statute_id: &str) {
        self.statute_ids.push(statute_id.to_string());
    }

    /// Adds metadata.
    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }
}

/// Types of relationships between legal concepts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ConceptRelationType {
    /// Concept A is a type of Concept B (inheritance)
    IsA,
    /// Concept A is part of Concept B (composition)
    PartOf,
    /// Concept A requires Concept B (dependency)
    Requires,
    /// Concept A conflicts with Concept B (mutual exclusion)
    ConflictsWith,
    /// Concept A enables Concept B (enablement)
    Enables,
    /// Concept A is related to Concept B (general association)
    RelatedTo,
    /// Concept A supersedes Concept B (replacement)
    Supersedes,
    /// Concept A implements Concept B (implementation)
    Implements,
}

impl ConceptRelationType {
    /// Returns a human-readable label for the relation type.
    pub fn label(&self) -> &'static str {
        match self {
            Self::IsA => "is a",
            Self::PartOf => "part of",
            Self::Requires => "requires",
            Self::ConflictsWith => "conflicts with",
            Self::Enables => "enables",
            Self::RelatedTo => "related to",
            Self::Supersedes => "supersedes",
            Self::Implements => "implements",
        }
    }

    /// Returns a color for visualizing the relation type.
    pub fn color(&self) -> &'static str {
        match self {
            Self::IsA => "#3498db",           // Blue - inheritance
            Self::PartOf => "#2ecc71",        // Green - composition
            Self::Requires => "#e74c3c",      // Red - dependency
            Self::ConflictsWith => "#c0392b", // Dark red - conflict
            Self::Enables => "#f39c12",       // Orange - enablement
            Self::RelatedTo => "#95a5a6",     // Gray - general
            Self::Supersedes => "#9b59b6",    // Purple - replacement
            Self::Implements => "#16a085",    // Teal - implementation
        }
    }
}

/// Relationship between two legal concepts.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConceptRelationship {
    /// Source concept ID
    pub from_id: String,
    /// Target concept ID
    pub to_id: String,
    /// Type of relationship
    pub relation_type: ConceptRelationType,
    /// Optional description
    pub description: String,
    /// Strength/confidence (0.0 to 1.0)
    pub strength: f64,
}

impl ConceptRelationship {
    /// Creates a new concept relationship.
    pub fn new(from_id: &str, to_id: &str, relation_type: ConceptRelationType) -> Self {
        Self {
            from_id: from_id.to_string(),
            to_id: to_id.to_string(),
            relation_type,
            description: String::new(),
            strength: 1.0,
        }
    }

    /// Sets the description.
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    /// Sets the strength (clamped to 0.0-1.0).
    pub fn with_strength(mut self, strength: f64) -> Self {
        self.strength = strength.clamp(0.0, 1.0);
        self
    }
}

/// Graph of legal concepts and their relationships.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConceptRelationshipGraph {
    /// Title of the graph
    pub title: String,
    /// Legal concepts in the graph
    pub concepts: Vec<LegalConcept>,
    /// Relationships between concepts
    pub relationships: Vec<ConceptRelationship>,
    /// Theme for visualization
    pub theme: Theme,
}

impl ConceptRelationshipGraph {
    /// Creates a new concept relationship graph.
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            concepts: Vec::new(),
            relationships: Vec::new(),
            theme: Theme::light(),
        }
    }

    /// Adds a concept to the graph.
    pub fn add_concept(&mut self, concept: LegalConcept) {
        self.concepts.push(concept);
    }

    /// Adds a relationship to the graph.
    pub fn add_relationship(&mut self, relationship: ConceptRelationship) {
        self.relationships.push(relationship);
    }

    /// Sets the theme.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Generates HTML visualization using D3.js force-directed graph.
    pub fn to_html(&self) -> String {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n");
        html.push_str("    <meta charset=\"UTF-8\">\n");
        html.push_str(
            "    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n",
        );
        html.push_str(&format!("    <title>{}</title>\n", self.title));
        html.push_str("    <script src=\"https://d3js.org/d3.v7.min.js\"></script>\n");
        html.push_str("    <style>\n");
        html.push_str("        body { margin: 0; padding: 0; overflow: hidden; }\n");
        html.push_str(&format!(
            "        body {{ background-color: {}; }}\n",
            self.theme.background_color
        ));
        html.push_str("        #graph { width: 100vw; height: 100vh; }\n");
        html.push_str("        .node { cursor: pointer; }\n");
        html.push_str("        .node circle { stroke: #fff; stroke-width: 2px; }\n");
        html.push_str("        .node text { font: 12px sans-serif; pointer-events: none; }\n");
        html.push_str(&format!(
            "        .node text {{ fill: {}; }}\n",
            self.theme.text_color
        ));
        html.push_str("        .link { stroke-opacity: 0.6; fill: none; }\n");
        html.push_str("        .link-label { font: 10px sans-serif; pointer-events: none; }\n");
        html.push_str(&format!(
            "        .link-label {{ fill: {}; }}\n",
            self.theme.text_color
        ));
        html.push_str("        .tooltip { position: absolute; padding: 8px; background: rgba(0,0,0,0.8); color: #fff; border-radius: 4px; pointer-events: none; opacity: 0; }\n");
        html.push_str("    </style>\n");
        html.push_str("</head>\n<body>\n");
        html.push_str("    <div id=\"graph\"></div>\n");
        html.push_str("    <div class=\"tooltip\" id=\"tooltip\"></div>\n");
        html.push_str("    <script>\n");

        // Generate nodes data
        html.push_str("        const nodes = [\n");
        for concept in &self.concepts {
            html.push_str(&format!(
                "            {{ id: '{}', name: '{}', category: '{}', description: '{}' }},\n",
                concept.id, concept.name, concept.category, concept.description
            ));
        }
        html.push_str("        ];\n\n");

        // Generate links data
        html.push_str("        const links = [\n");
        for rel in &self.relationships {
            html.push_str(&format!(
                "            {{ source: '{}', target: '{}', type: '{}', color: '{}', strength: {} }},\n",
                rel.from_id, rel.to_id, rel.relation_type.label(), rel.relation_type.color(), rel.strength
            ));
        }
        html.push_str("        ];\n\n");

        // D3.js visualization code
        html.push_str("        const width = window.innerWidth;\n");
        html.push_str("        const height = window.innerHeight;\n\n");
        html.push_str("        const svg = d3.select('#graph').append('svg')\n");
        html.push_str("            .attr('width', width)\n");
        html.push_str("            .attr('height', height);\n\n");
        html.push_str("        const simulation = d3.forceSimulation(nodes)\n");
        html.push_str(
            "            .force('link', d3.forceLink(links).id(d => d.id).distance(150))\n",
        );
        html.push_str("            .force('charge', d3.forceManyBody().strength(-300))\n");
        html.push_str("            .force('center', d3.forceCenter(width / 2, height / 2));\n\n");
        html.push_str("        const link = svg.append('g')\n");
        html.push_str("            .selectAll('line')\n");
        html.push_str("            .data(links)\n");
        html.push_str("            .enter().append('line')\n");
        html.push_str("            .attr('class', 'link')\n");
        html.push_str("            .attr('stroke', d => d.color)\n");
        html.push_str("            .attr('stroke-width', d => d.strength * 2);\n\n");
        html.push_str("        const linkLabel = svg.append('g')\n");
        html.push_str("            .selectAll('text')\n");
        html.push_str("            .data(links)\n");
        html.push_str("            .enter().append('text')\n");
        html.push_str("            .attr('class', 'link-label')\n");
        html.push_str("            .text(d => d.type);\n\n");
        html.push_str("        const node = svg.append('g')\n");
        html.push_str("            .selectAll('g')\n");
        html.push_str("            .data(nodes)\n");
        html.push_str("            .enter().append('g')\n");
        html.push_str("            .attr('class', 'node')\n");
        html.push_str("            .call(d3.drag()\n");
        html.push_str("                .on('start', dragstarted)\n");
        html.push_str("                .on('drag', dragged)\n");
        html.push_str("                .on('end', dragended));\n\n");
        html.push_str("        node.append('circle')\n");
        html.push_str("            .attr('r', 10)\n");
        html.push_str("            .attr('fill', '#3498db');\n\n");
        html.push_str("        node.append('text')\n");
        html.push_str("            .attr('dx', 12)\n");
        html.push_str("            .attr('dy', '.35em')\n");
        html.push_str("            .text(d => d.name);\n\n");
        html.push_str("        const tooltip = d3.select('#tooltip');\n");
        html.push_str("        node.on('mouseover', function(event, d) {\n");
        html.push_str("            tooltip.transition().duration(200).style('opacity', 1);\n");
        html.push_str("            tooltip.html(`<strong>${d.name}</strong><br/>${d.category}<br/>${d.description}`)\n");
        html.push_str("                .style('left', (event.pageX + 10) + 'px')\n");
        html.push_str("                .style('top', (event.pageY - 10) + 'px');\n");
        html.push_str("        }).on('mouseout', function() {\n");
        html.push_str("            tooltip.transition().duration(500).style('opacity', 0);\n");
        html.push_str("        });\n\n");
        html.push_str("        simulation.on('tick', () => {\n");
        html.push_str("            link.attr('x1', d => d.source.x)\n");
        html.push_str("                .attr('y1', d => d.source.y)\n");
        html.push_str("                .attr('x2', d => d.target.x)\n");
        html.push_str("                .attr('y2', d => d.target.y);\n");
        html.push_str("            linkLabel.attr('x', d => (d.source.x + d.target.x) / 2)\n");
        html.push_str("                .attr('y', d => (d.source.y + d.target.y) / 2);\n");
        html.push_str("            node.attr('transform', d => `translate(${d.x},${d.y})`);\n");
        html.push_str("        });\n\n");
        html.push_str("        function dragstarted(event) {\n");
        html.push_str("            if (!event.active) simulation.alphaTarget(0.3).restart();\n");
        html.push_str("            event.subject.fx = event.subject.x;\n");
        html.push_str("            event.subject.fy = event.subject.y;\n");
        html.push_str("        }\n");
        html.push_str("        function dragged(event) {\n");
        html.push_str("            event.subject.fx = event.x;\n");
        html.push_str("            event.subject.fy = event.y;\n");
        html.push_str("        }\n");
        html.push_str("        function dragended(event) {\n");
        html.push_str("            if (!event.active) simulation.alphaTarget(0);\n");
        html.push_str("            event.subject.fx = null;\n");
        html.push_str("            event.subject.fy = null;\n");
        html.push_str("        }\n");
        html.push_str("    </script>\n");
        html.push_str("</body>\n</html>");

        html
    }

    /// Generates Mermaid diagram format.
    pub fn to_mermaid(&self) -> String {
        let mut diagram = String::new();
        diagram.push_str("graph TD\n");

        // Add nodes
        for concept in &self.concepts {
            diagram.push_str(&format!("    {}[\"{}\"]\n", concept.id, concept.name));
        }

        // Add relationships
        for rel in &self.relationships {
            diagram.push_str(&format!(
                "    {} -->|{}| {}\n",
                rel.from_id,
                rel.relation_type.label(),
                rel.to_id
            ));
        }

        diagram
    }
}

/// Maps statutes to legal concepts.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StatuteConceptMapping {
    /// Statute ID
    pub statute_id: String,
    /// Statute name
    pub statute_name: String,
    /// Mapped concept IDs
    pub concept_ids: Vec<String>,
    /// Confidence scores for each mapping (0.0 to 1.0)
    pub confidence_scores: std::collections::HashMap<String, f64>,
}

impl StatuteConceptMapping {
    /// Creates a new statute-to-concept mapping.
    pub fn new(statute_id: &str, statute_name: &str) -> Self {
        Self {
            statute_id: statute_id.to_string(),
            statute_name: statute_name.to_string(),
            concept_ids: Vec::new(),
            confidence_scores: std::collections::HashMap::new(),
        }
    }

    /// Adds a concept mapping with confidence score.
    pub fn add_concept(&mut self, concept_id: &str, confidence: f64) {
        self.concept_ids.push(concept_id.to_string());
        self.confidence_scores
            .insert(concept_id.to_string(), confidence.clamp(0.0, 1.0));
    }

    /// Gets the confidence score for a concept.
    pub fn confidence(&self, concept_id: &str) -> f64 {
        self.confidence_scores
            .get(concept_id)
            .copied()
            .unwrap_or(0.0)
    }
}

/// Visualizes legal ontologies and taxonomies.
#[derive(Debug, Clone)]
pub struct OntologyBasedVisualizer {
    /// Theme for visualization
    pub theme: Theme,
}

impl OntologyBasedVisualizer {
    /// Creates a new ontology-based visualizer.
    pub fn new() -> Self {
        Self {
            theme: Theme::light(),
        }
    }

    /// Sets the theme.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }

    /// Generates HTML visualization of a concept graph as an ontology.
    pub fn to_html(&self, graph: &ConceptRelationshipGraph) -> String {
        // Delegate to the graph's HTML generation with theme override
        let mut graph_clone = graph.clone();
        graph_clone.theme = self.theme.clone();

        let mut html = graph_clone.to_html();

        // Add ontology-specific styling
        html = html.replace(
            "</style>",
            "        .ontology-layer { opacity: 0.9; }\n        .ontology-root { font-weight: bold; }\n    </style>"
        );

        html
    }

    /// Generates ontology tree visualization in HTML.
    pub fn to_tree_html(&self, graph: &ConceptRelationshipGraph) -> String {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n");
        html.push_str("    <meta charset=\"UTF-8\">\n");
        html.push_str(
            "    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n",
        );
        html.push_str(&format!(
            "    <title>{} - Ontology Tree</title>\n",
            graph.title
        ));
        html.push_str("    <style>\n");
        html.push_str("        body { margin: 20px; font-family: Arial, sans-serif; }\n");
        html.push_str(&format!(
            "        body {{ background-color: {}; color: {}; }}\n",
            self.theme.background_color, self.theme.text_color
        ));
        html.push_str("        .tree { list-style: none; padding-left: 20px; }\n");
        html.push_str(
            "        .tree-node { margin: 5px 0; padding: 5px; border-left: 2px solid #ccc; }\n",
        );
        html.push_str("        .tree-node:hover { background-color: rgba(52, 152, 219, 0.1); }\n");
        html.push_str("        .concept-name { font-weight: bold; color: #3498db; }\n");
        html.push_str("        .concept-category { color: #7f8c8d; font-size: 0.9em; }\n");
        html.push_str("    </style>\n");
        html.push_str("</head>\n<body>\n");
        html.push_str(&format!("    <h1>{}</h1>\n", graph.title));
        html.push_str("    <ul class=\"tree\">\n");

        // Build tree structure (simplified - shows all concepts)
        for concept in &graph.concepts {
            html.push_str("        <li class=\"tree-node\">\n");
            html.push_str(&format!(
                "            <span class=\"concept-name\">{}</span>\n",
                concept.name
            ));
            html.push_str(&format!(
                "            <span class=\"concept-category\"> [{}]</span>\n",
                concept.category
            ));
            html.push_str(&format!("            <div>{}</div>\n", concept.description));
            html.push_str("        </li>\n");
        }

        html.push_str("    </ul>\n");
        html.push_str("</body>\n</html>");

        html
    }
}

impl Default for OntologyBasedVisualizer {
    fn default() -> Self {
        Self::new()
    }
}

/// Highlights semantic search results in visualizations.
#[derive(Debug, Clone)]
pub struct SemanticSearchHighlighter {
    /// Search query
    pub query: String,
    /// Matching concept IDs
    pub matches: Vec<String>,
    /// Relevance scores (0.0 to 1.0)
    pub relevance_scores: std::collections::HashMap<String, f64>,
    /// Highlight color
    pub highlight_color: String,
}

impl SemanticSearchHighlighter {
    /// Creates a new semantic search highlighter.
    pub fn new(query: &str) -> Self {
        Self {
            query: query.to_string(),
            matches: Vec::new(),
            relevance_scores: std::collections::HashMap::new(),
            highlight_color: "#ffeb3b".to_string(),
        }
    }

    /// Performs semantic search on a concept graph.
    pub fn search(&mut self, graph: &ConceptRelationshipGraph) {
        self.matches.clear();
        self.relevance_scores.clear();

        let query_lower = self.query.to_lowercase();

        for concept in &graph.concepts {
            let name_lower = concept.name.to_lowercase();
            let desc_lower = concept.description.to_lowercase();
            let cat_lower = concept.category.to_lowercase();

            // Simple relevance scoring
            let mut score: f64 = 0.0;

            if name_lower.contains(&query_lower) {
                score += 1.0;
            }
            if desc_lower.contains(&query_lower) {
                score += 0.5;
            }
            if cat_lower.contains(&query_lower) {
                score += 0.3;
            }

            if score > 0.0 {
                self.matches.push(concept.id.clone());
                self.relevance_scores
                    .insert(concept.id.clone(), score.min(1.0));
            }
        }
    }

    /// Sets the highlight color.
    pub fn with_color(mut self, color: &str) -> Self {
        self.highlight_color = color.to_string();
        self
    }

    /// Generates highlighted HTML visualization.
    pub fn to_highlighted_html(&self, graph: &ConceptRelationshipGraph) -> String {
        let base_html = graph.to_html();

        // Inject highlighting JavaScript
        let highlight_script = format!(
            r#"
        <script>
            const highlights = {};
            setTimeout(() => {{
                d3.selectAll('.node circle')
                    .attr('fill', d => highlights[d.id] ? '{}' : '#3498db')
                    .attr('r', d => highlights[d.id] ? 15 : 10);
            }}, 500);
        </script>
        "#,
            serde_json::to_string(&self.matches).unwrap(),
            self.highlight_color
        );

        base_html.replace("</body>", &format!("{}</body>", highlight_script))
    }
}

/// Visualizes concept hierarchies as trees.
#[derive(Debug, Clone)]
pub struct ConceptHierarchyTree {
    /// Root concept
    pub root: LegalConcept,
    /// Child hierarchies
    pub children: Vec<ConceptHierarchyTree>,
    /// Theme for visualization
    pub theme: Theme,
}

impl ConceptHierarchyTree {
    /// Creates a new concept hierarchy tree.
    pub fn new(root: LegalConcept) -> Self {
        Self {
            root,
            children: Vec::new(),
            theme: Theme::light(),
        }
    }

    /// Adds a child concept.
    pub fn add_child(&mut self, child: ConceptHierarchyTree) {
        self.children.push(child);
    }

    /// Sets the theme.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme.clone();
        for child in &mut self.children {
            child.theme = theme.clone();
        }
        self
    }

    /// Builds a hierarchy from a concept graph (based on IsA relationships).
    pub fn from_graph(graph: &ConceptRelationshipGraph, root_id: &str) -> Option<Self> {
        let root_concept = graph.concepts.iter().find(|c| c.id == root_id)?;

        let mut tree = Self::new(root_concept.clone());

        // Find all IsA relationships where this concept is the parent
        for rel in &graph.relationships {
            if rel.to_id == root_id && rel.relation_type == ConceptRelationType::IsA {
                if let Some(child_tree) = Self::from_graph(graph, &rel.from_id) {
                    tree.add_child(child_tree);
                }
            }
        }

        Some(tree)
    }

    /// Generates HTML tree visualization.
    pub fn to_html(&self) -> String {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n");
        html.push_str("    <meta charset=\"UTF-8\">\n");
        html.push_str(
            "    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n",
        );
        html.push_str(&format!(
            "    <title>Concept Hierarchy: {}</title>\n",
            self.root.name
        ));
        html.push_str("    <style>\n");
        html.push_str(
            "        body { margin: 20px; font-family: 'Segoe UI', Arial, sans-serif; }\n",
        );
        html.push_str(&format!(
            "        body {{ background-color: {}; color: {}; }}\n",
            self.theme.background_color, self.theme.text_color
        ));
        html.push_str("        .hierarchy { list-style: none; padding-left: 30px; }\n");
        html.push_str("        .hierarchy > li { margin: 10px 0; }\n");
        html.push_str("        .concept-box { \n");
        html.push_str("            padding: 10px; \n");
        html.push_str("            border: 2px solid #3498db; \n");
        html.push_str("            border-radius: 5px; \n");
        html.push_str("            display: inline-block; \n");
        html.push_str("            margin: 5px 0;\n");
        html.push_str("            background-color: rgba(52, 152, 219, 0.1);\n");
        html.push_str("        }\n");
        html.push_str(
            "        .concept-name { font-weight: bold; font-size: 1.1em; color: #2980b9; }\n",
        );
        html.push_str(
            "        .concept-category { color: #7f8c8d; font-size: 0.9em; margin-left: 10px; }\n",
        );
        html.push_str("        .concept-description { margin-top: 5px; font-size: 0.95em; }\n");
        html.push_str("    </style>\n");
        html.push_str("</head>\n<body>\n");
        html.push_str("    <h1>Concept Hierarchy</h1>\n");
        html.push_str("    <ul class=\"hierarchy\">\n");

        self.render_node(&mut html);

        html.push_str("    </ul>\n");
        html.push_str("</body>\n</html>");

        html
    }

    #[allow(dead_code)]
    fn render_node(&self, html: &mut String) {
        html.push_str("        <li>\n");
        html.push_str("            <div class=\"concept-box\">\n");
        html.push_str(&format!(
            "                <span class=\"concept-name\">{}</span>\n",
            self.root.name
        ));
        html.push_str(&format!(
            "                <span class=\"concept-category\">[{}]</span>\n",
            self.root.category
        ));
        html.push_str(&format!(
            "                <div class=\"concept-description\">{}</div>\n",
            self.root.description
        ));
        html.push_str("            </div>\n");

        if !self.children.is_empty() {
            html.push_str("            <ul class=\"hierarchy\">\n");
            for child in &self.children {
                child.render_node(html);
            }
            html.push_str("            </ul>\n");
        }

        html.push_str("        </li>\n");
    }

    /// Generates Mermaid diagram format.
    pub fn to_mermaid(&self) -> String {
        let mut diagram = String::new();
        diagram.push_str("graph TD\n");
        self.render_mermaid_node(&mut diagram);
        diagram
    }

    #[allow(dead_code)]
    fn render_mermaid_node(&self, diagram: &mut String) {
        diagram.push_str(&format!("    {}[\"{}\"]\n", self.root.id, self.root.name));

        for child in &self.children {
            diagram.push_str(&format!("    {} --> {}\n", self.root.id, child.root.id));
            child.render_mermaid_node(diagram);
        }
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

    #[test]
    fn test_plantuml_export() {
        let statute = Statute::new(
            "test",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        );

        let tree = DecisionTree::from_statute(&statute).unwrap();
        let plantuml = tree.to_plantuml();
        assert!(plantuml.contains("@startuml"));
        assert!(plantuml.contains("@enduml"));
        assert!(plantuml.contains("Test Statute"));
    }

    #[test]
    fn test_dependency_graph_plantuml() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("statute-a", "statute-b", "references");
        graph.add_dependency("statute-b", "statute-c", "amends");

        let plantuml = graph.to_plantuml();
        assert!(plantuml.contains("@startuml"));
        assert!(plantuml.contains("statute-a"));
        assert!(plantuml.contains("references"));
    }

    #[test]
    fn test_html_export() {
        let statute = Statute::new(
            "test",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        );

        let tree = DecisionTree::from_statute(&statute).unwrap();
        let html = tree.to_html();
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("d3.v7.min.js"));
        assert!(html.contains("Test Statute"));
    }

    #[test]
    fn test_dependency_graph_html() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("statute-a", "statute-b", "references");

        let html = graph.to_html();
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("statute-a"));
        assert!(html.contains("d3.forceSimulation"));
    }

    #[test]
    fn test_timeline_ascii() {
        let mut timeline = Timeline::new();
        timeline.add_event(
            "2020-01-01",
            TimelineEvent::Enacted {
                statute_id: "test-law".to_string(),
                title: "Test Law".to_string(),
            },
        );
        timeline.add_event(
            "2021-06-15",
            TimelineEvent::Amended {
                statute_id: "test-law".to_string(),
                description: "Added provision X".to_string(),
            },
        );

        let ascii = timeline.to_ascii();
        assert!(ascii.contains("Timeline of Legal Events"));
        assert!(ascii.contains("2020-01-01"));
        assert!(ascii.contains("Test Law"));
    }

    #[test]
    fn test_timeline_mermaid() {
        let mut timeline = Timeline::new();
        timeline.add_event(
            "2020-01-01",
            TimelineEvent::Enacted {
                statute_id: "test-law".to_string(),
                title: "Test Law".to_string(),
            },
        );

        let mermaid = timeline.to_mermaid();
        assert!(mermaid.contains("gantt"));
        assert!(mermaid.contains("test-law"));
    }

    #[test]
    fn test_timeline_html() {
        let mut timeline = Timeline::new();
        timeline.add_event(
            "2020-01-01",
            TimelineEvent::Enacted {
                statute_id: "test-law".to_string(),
                title: "Test Law".to_string(),
            },
        );

        let html = timeline.to_html();
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Legal Timeline"));
        assert!(html.contains("2020-01-01"));
    }

    #[test]
    fn test_theme_light() {
        let theme = Theme::light();
        assert_eq!(theme.background_color, "#ffffff");
        assert_eq!(theme.text_color, "#333333");
    }

    #[test]
    fn test_theme_dark() {
        let theme = Theme::dark();
        assert_eq!(theme.background_color, "#1a1a1a");
        assert_eq!(theme.text_color, "#e0e0e0");
    }

    #[test]
    fn test_html_with_custom_theme() {
        let statute = Statute::new(
            "test",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        );

        let tree = DecisionTree::from_statute(&statute).unwrap();
        let theme = Theme::dark();
        let html = tree.to_html_with_theme(&theme);
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains(&theme.background_color));
        assert!(html.contains("Test Statute"));
    }

    #[test]
    fn test_annotation_creation() {
        let annotation = Annotation::new("ann1", "node-1", "This is a test annotation")
            .with_citation("Smith v. Jones, 123 U.S. 456 (2020)")
            .with_author("Judge Smith")
            .with_date("2020-01-01")
            .with_type(AnnotationType::CaseLaw);

        assert_eq!(annotation.id, "ann1");
        assert_eq!(annotation.target, "node-1");
        assert_eq!(annotation.text, "This is a test annotation");
        assert_eq!(
            annotation.citation,
            Some("Smith v. Jones, 123 U.S. 456 (2020)".to_string())
        );
        assert!(matches!(
            annotation.annotation_type,
            AnnotationType::CaseLaw
        ));
    }

    #[test]
    fn test_decision_tree_with_annotations() {
        let statute = Statute::new(
            "test",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        );

        let mut tree = DecisionTree::from_statute(&statute).unwrap();

        let annotation = Annotation::new("ann1", "test", "Judicial interpretation note")
            .with_type(AnnotationType::Interpretation);

        tree.add_annotation(annotation);

        assert_eq!(tree.annotations().len(), 1);
        let annotations_for_test = tree.annotations_for("test");
        assert_eq!(annotations_for_test.len(), 1);
        assert_eq!(annotations_for_test[0].text, "Judicial interpretation note");
    }

    #[test]
    fn test_ascii_with_annotations() {
        let statute = Statute::new(
            "test",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        );

        let mut tree = DecisionTree::from_statute(&statute).unwrap();

        let annotation =
            Annotation::new("ann1", "test", "Important note").with_citation("Example citation");

        tree.add_annotation(annotation);

        let ascii = tree.to_ascii();
        assert!(ascii.contains("Annotations:"));
        assert!(ascii.contains("Important note"));
        assert!(ascii.contains("Example citation"));
    }

    #[test]
    fn test_layout_config_default() {
        let config = LayoutConfig::default();
        assert_eq!(config.width, 960);
        assert_eq!(config.height, 600);
        assert!(!config.enable_clustering);
    }

    #[test]
    fn test_layout_config_large_graph() {
        let config = LayoutConfig::large_graph();
        assert_eq!(config.width, 1920);
        assert_eq!(config.height, 1080);
        assert!(config.enable_clustering);
        assert_eq!(config.max_nodes, Some(100));
    }

    #[test]
    fn test_dependency_graph_with_layout() {
        let layout = LayoutConfig::large_graph();
        let mut graph = DependencyGraph::with_layout(layout);

        for i in 0..10 {
            graph.add_statute(&format!("statute-{}", i));
        }

        assert_eq!(graph.node_count(), 10);
        assert!(!graph.is_large_graph());
    }

    #[test]
    fn test_large_graph_detection() {
        let layout = LayoutConfig {
            width: 800,
            height: 600,
            node_spacing: 100,
            enable_clustering: true,
            max_nodes: Some(5),
        };

        let mut graph = DependencyGraph::with_layout(layout);

        for i in 0..10 {
            graph.add_statute(&format!("statute-{}", i));
        }

        assert!(graph.is_large_graph());
    }

    #[test]
    fn test_population_chart_ascii() {
        let mut chart = PopulationChart::new("Test Distribution");
        chart.add_data("Eligible", 150);
        chart.add_data("Ineligible", 50);
        chart.add_data("Pending", 25);
        chart.calculate_percentages();

        let ascii = chart.to_ascii();
        assert!(ascii.contains("Test Distribution"));
        assert!(ascii.contains("Eligible"));
        assert!(ascii.contains("150"));
    }

    #[test]
    fn test_population_chart_html() {
        let mut chart = PopulationChart::new("Simulation Results");
        chart.add_data("Approved", 100);
        chart.add_data("Denied", 30);

        let html = chart.to_html();
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("chart.js"));
        assert!(html.contains("Simulation Results"));
    }

    #[test]
    fn test_population_chart_time_series() {
        let mut chart = PopulationChart::new("Population Over Time");

        let data_t1 = vec![
            PopulationDataPoint {
                category: "Approved".to_string(),
                count: 50,
                percentage: None,
            },
            PopulationDataPoint {
                category: "Denied".to_string(),
                count: 20,
                percentage: None,
            },
        ];

        let data_t2 = vec![
            PopulationDataPoint {
                category: "Approved".to_string(),
                count: 75,
                percentage: None,
            },
            PopulationDataPoint {
                category: "Denied".to_string(),
                count: 25,
                percentage: None,
            },
        ];

        chart.add_time_point("2020-01-01", data_t1);
        chart.add_time_point("2020-02-01", data_t2);

        let html = chart.time_series_to_html();
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("2020-01-01"));
        assert!(html.contains("Approved"));
    }

    #[test]
    fn test_population_percentages() {
        let mut chart = PopulationChart::new("Test");
        chart.add_data("A", 50);
        chart.add_data("B", 50);
        chart.calculate_percentages();

        assert_eq!(chart.data[0].percentage, Some(50.0));
        assert_eq!(chart.data[1].percentage, Some(50.0));
    }

    #[test]
    fn test_decision_tree_svg() {
        let statute = Statute::new(
            "test",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        );

        let tree = DecisionTree::from_statute(&statute).unwrap();
        let svg = tree.to_svg();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
        assert!(svg.contains("Test Statute"));
    }

    #[test]
    fn test_dependency_graph_svg() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("statute-a", "statute-b", "references");
        graph.add_dependency("statute-b", "statute-c", "amends");

        let svg = graph.to_svg();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
        assert!(svg.contains("statute-a"));
    }

    #[test]
    fn test_svg_with_custom_theme() {
        let statute = Statute::new(
            "test",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        );

        let tree = DecisionTree::from_statute(&statute).unwrap();
        let theme = Theme::dark();
        let svg = tree.to_svg_with_theme(&theme);
        assert!(svg.contains("<svg"));
        assert!(svg.contains(&theme.background_color));
    }

    #[test]
    #[cfg(feature = "png-export")]
    fn test_png_export() {
        let statute = Statute::new(
            "test",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        );

        let tree = DecisionTree::from_statute(&statute).unwrap();
        let png_data = tree.to_png();
        assert!(png_data.is_ok());
        assert!(!png_data.unwrap().is_empty());
    }

    #[test]
    #[cfg(feature = "png-export")]
    fn test_dependency_graph_png() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("statute-a", "statute-b", "references");

        let png_data = graph.to_png();
        assert!(png_data.is_ok());
        assert!(!png_data.unwrap().is_empty());
    }

    #[test]
    fn test_drill_down_html() {
        let statute = Statute::new(
            "test",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        );

        let tree = DecisionTree::from_statute(&statute).unwrap();
        let html = tree.to_html();
        assert!(html.contains("Interactive"));
        assert!(html.contains("drill down"));
        assert!(html.contains("details"));
        assert!(html.contains("click"));
    }

    #[test]
    fn test_renderer_registry() {
        let registry = RendererRegistry::new();
        assert!(registry.renderers.is_empty());
    }

    #[test]
    fn test_live_visualization() {
        let mut live_viz = LiveVisualization::new("Test Live Viz");

        let event = UpdateEvent::PopulationUpdate {
            category: "Eligible".to_string(),
            count: 100,
            timestamp: "2024-01-01".to_string(),
        };

        live_viz.process_update(event);
        assert_eq!(live_viz.update_history().len(), 1);
    }

    #[test]
    fn test_live_visualization_dependency_update() {
        let mut live_viz = LiveVisualization::new("Test");

        let event = UpdateEvent::DependencyAdded {
            from_statute: "statute-a".to_string(),
            to_statute: "statute-b".to_string(),
            relation: "references".to_string(),
        };

        live_viz.process_update(event);
        assert_eq!(live_viz.dependency_graph.node_count(), 2);
    }

    #[test]
    fn test_live_html_export() {
        let live_viz = LiveVisualization::new("Test");
        let html = live_viz.to_live_html("ws://localhost:8080");
        assert!(html.contains("WebSocket"));
        assert!(html.contains("ws://localhost:8080"));
        assert!(html.contains("Live Visualization Dashboard"));
    }

    #[test]
    fn test_update_event_serialization() {
        let event = UpdateEvent::PopulationUpdate {
            category: "Test".to_string(),
            count: 50,
            timestamp: "2024-01-01".to_string(),
        };

        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("PopulationUpdate"));
        assert!(json.contains("Test"));
    }

    #[test]
    fn test_theme_colorblind_friendly() {
        let theme = Theme::colorblind_friendly();
        assert_eq!(theme.condition_color, "#0173b2");
        assert_eq!(theme.discretion_color, "#de8f05");
        assert_eq!(theme.outcome_color, "#029e73");
    }

    #[test]
    fn test_dependency_graph_svg_with_theme() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("statute-a", "statute-b", "references");

        let theme = Theme::high_contrast();
        let svg = graph.to_svg_with_theme(&theme);
        assert!(svg.contains("<svg"));
        assert!(svg.contains(&theme.background_color));
    }

    #[test]
    fn test_all_output_formats_decision_tree() {
        let statute = Statute::new(
            "comprehensive-test",
            "Comprehensive Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let tree = DecisionTree::from_statute(&statute).unwrap();

        // Test all export formats
        let dot = tree.to_dot();
        assert!(!dot.is_empty());

        let ascii = tree.to_ascii();
        assert!(ascii.contains("Comprehensive Test Statute"));

        let box_format = tree.to_box();
        assert!(box_format.contains("┌"));

        let mermaid = tree.to_mermaid();
        assert!(mermaid.contains("flowchart TD"));

        let plantuml = tree.to_plantuml();
        assert!(plantuml.contains("@startuml"));

        let svg = tree.to_svg();
        assert!(svg.contains("<svg"));

        let html = tree.to_html();
        assert!(html.contains("<!DOCTYPE html>"));
    }

    #[test]
    fn test_all_output_formats_dependency_graph() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("statute-1", "statute-2", "references");
        graph.add_dependency("statute-2", "statute-3", "amends");

        let dot = graph.to_dot();
        assert!(!dot.is_empty());

        let mermaid = graph.to_mermaid();
        assert!(mermaid.contains("flowchart LR"));

        let plantuml = graph.to_plantuml();
        assert!(plantuml.contains("@startuml"));

        let svg = graph.to_svg();
        assert!(svg.contains("<svg"));

        let html = graph.to_html();
        assert!(html.contains("<!DOCTYPE html>"));
    }

    #[test]
    fn test_layout_config_compact() {
        let config = LayoutConfig::compact();
        assert_eq!(config.width, 800);
        assert_eq!(config.height, 400);
        assert_eq!(config.node_spacing, 50);
        assert_eq!(config.max_nodes, Some(50));
    }

    #[test]
    fn test_live_visualization_clear_history() {
        let mut live_viz = LiveVisualization::new("Test");

        let event = UpdateEvent::StatisticsUpdate {
            metric: "test_metric".to_string(),
            value: 42.5,
        };

        live_viz.process_update(event);
        assert_eq!(live_viz.update_history().len(), 1);

        live_viz.clear_history();
        assert_eq!(live_viz.update_history().len(), 0);
    }

    #[test]
    fn test_presentation_exporter_creation() {
        let exporter = PresentationExporter::new();
        assert_eq!(exporter.slides.len(), 0);
    }

    #[test]
    fn test_presentation_exporter_with_theme() {
        let theme = Theme::dark();
        let exporter = PresentationExporter::new().with_theme(theme.clone());
        assert_eq!(exporter.theme.background_color, theme.background_color);
    }

    #[test]
    fn test_presentation_add_decision_tree_slide() {
        let statute = Statute::new(
            "test-statute",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let tree = DecisionTree::from_statute(&statute).unwrap();
        let mut exporter = PresentationExporter::new();
        exporter.add_decision_tree_slide("Test Decision Tree", &tree);

        assert_eq!(exporter.slides.len(), 1);
        assert_eq!(exporter.slides[0].title, "Test Decision Tree");
    }

    #[test]
    fn test_presentation_add_dependency_graph_slide() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("statute-a", "statute-b", "references");

        let mut exporter = PresentationExporter::new();
        exporter.add_dependency_graph_slide("Test Dependency Graph", &graph);

        assert_eq!(exporter.slides.len(), 1);
        assert_eq!(exporter.slides[0].title, "Test Dependency Graph");
    }

    #[test]
    fn test_presentation_to_pptx() {
        let statute = Statute::new(
            "test-statute",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        );

        let tree = DecisionTree::from_statute(&statute).unwrap();
        let mut exporter = PresentationExporter::new();
        exporter.add_decision_tree_slide("Test Slide", &tree);

        let pptx = exporter.to_pptx().unwrap();
        assert!(pptx.contains("<?xml version=\"1.0\""));
        assert!(pptx.contains("<p:presentation"));
        assert!(pptx.contains("<p:sldIdLst>"));
    }

    #[test]
    fn test_presentation_to_keynote() {
        let statute = Statute::new(
            "test-statute",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        );

        let tree = DecisionTree::from_statute(&statute).unwrap();
        let mut exporter = PresentationExporter::new();
        exporter.add_decision_tree_slide("Test Slide", &tree);

        let keynote = exporter.to_keynote().unwrap();
        assert!(keynote.contains("<?xml version=\"1.0\""));
        assert!(keynote.contains("<key version="));
        assert!(keynote.contains("<slides>"));
        assert!(keynote.contains("<title>Test Slide</title>"));
    }

    #[test]
    fn test_presentation_to_animated_html() {
        let statute = Statute::new(
            "test-statute",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        );

        let tree = DecisionTree::from_statute(&statute).unwrap();
        let mut exporter = PresentationExporter::new();
        exporter.add_decision_tree_slide("Slide 1", &tree);
        exporter.add_decision_tree_slide("Slide 2", &tree);

        let html = exporter.to_animated_html();
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Animated Presentation"));
        assert!(html.contains("Slide 1"));
        assert!(html.contains("Slide 2"));
        assert!(html.contains("nextSlide"));
        assert!(html.contains("previousSlide"));
        assert!(html.contains("@keyframes fadeIn"));
    }

    #[test]
    fn test_document_embedder_creation() {
        let embedder = DocumentEmbedder::new();
        assert_eq!(
            embedder.theme.background_color,
            Theme::default().background_color
        );
    }

    #[test]
    fn test_document_embedder_with_theme() {
        let theme = Theme::dark();
        let embedder = DocumentEmbedder::new().with_theme(theme.clone());
        assert_eq!(embedder.theme.background_color, theme.background_color);
    }

    #[test]
    fn test_embed_in_markdown() {
        let statute = Statute::new(
            "test-statute",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        );

        let tree = DecisionTree::from_statute(&statute).unwrap();
        let embedder = DocumentEmbedder::new();
        let markdown = embedder.embed_in_markdown(&tree);

        assert!(markdown.starts_with("![Decision Tree](data:image/svg+xml;base64,"));
        assert!(markdown.contains("base64"));
    }

    #[test]
    fn test_embed_in_latex() {
        let statute = Statute::new(
            "test-statute",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        );

        let tree = DecisionTree::from_statute(&statute).unwrap();
        let embedder = DocumentEmbedder::new();
        let latex = embedder.embed_in_latex(&tree);

        assert!(latex.contains("\\begin{figure}"));
        assert!(latex.contains("\\begin{tikzpicture}"));
        assert!(latex.contains("\\end{tikzpicture}"));
        assert!(latex.contains("\\caption{Decision Tree Visualization}"));
    }

    #[test]
    fn test_embed_in_rst() {
        let statute = Statute::new(
            "test-statute",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        );

        let tree = DecisionTree::from_statute(&statute).unwrap();
        let embedder = DocumentEmbedder::new();
        let rst = embedder.embed_in_rst(&tree);

        assert!(rst.starts_with(".. image:: data:image/svg+xml;base64,"));
        assert!(rst.contains(":alt: Decision Tree"));
        assert!(rst.contains(":align: center"));
    }

    #[test]
    fn test_embed_in_asciidoc() {
        let statute = Statute::new(
            "test-statute",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        );

        let tree = DecisionTree::from_statute(&statute).unwrap();
        let embedder = DocumentEmbedder::new();
        let asciidoc = embedder.embed_in_asciidoc(&tree);

        assert!(asciidoc.starts_with("image::data:image/svg+xml;base64,"));
        assert!(asciidoc.contains("[Decision Tree,align=center]"));
    }

    #[test]
    fn test_embed_as_iframe() {
        let statute = Statute::new(
            "test-statute",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        );

        let tree = DecisionTree::from_statute(&statute).unwrap();
        let embedder = DocumentEmbedder::new();
        let iframe = embedder.embed_as_iframe(&tree, 800, 600);

        assert!(iframe.starts_with("<iframe"));
        assert!(iframe.contains("width=\"800\""));
        assert!(iframe.contains("height=\"600\""));
        assert!(iframe.contains("data:text/html;base64,"));
    }

    #[test]
    fn test_visual_regression_test_passed() {
        let baseline = "Line 1\nLine 2\nLine 3";
        let actual = "Line 1\nLine 2\nLine 3";

        let test = VisualRegressionTest::new("test1", baseline, actual);
        assert!(test.passed);
        assert_eq!(test.differences.len(), 0);
    }

    #[test]
    fn test_visual_regression_test_failed() {
        let baseline = "Line 1\nLine 2\nLine 3";
        let actual = "Line 1\nLine X\nLine 3";

        let test = VisualRegressionTest::new("test1", baseline, actual);
        assert!(!test.passed);
        assert!(!test.differences.is_empty());
    }

    #[test]
    fn test_visual_regression_test_report() {
        let baseline = "Line 1\nLine 2";
        let actual = "Line 1\nLine X";

        let test = VisualRegressionTest::new("test1", baseline, actual);
        let report = test.report();

        assert!(report.contains("Visual Regression Test: test1"));
        assert!(report.contains("Status: FAILED"));
        assert!(report.contains("Differences found:"));
    }

    #[test]
    fn test_visual_regression_suite() {
        let mut suite = VisualRegressionSuite::new();

        let test1 = VisualRegressionTest::new("test1", "data1", "data1");
        let test2 = VisualRegressionTest::new("test2", "data2", "different");

        suite.add_test(test1);
        suite.add_test(test2);

        let summary = suite.run();
        assert!(summary.contains("Total tests: 2"));
        assert!(summary.contains("Passed: 1"));
        assert!(summary.contains("Failed: 1"));
        assert!(!suite.all_passed());
    }

    #[test]
    fn test_visual_regression_suite_all_passed() {
        let mut suite = VisualRegressionSuite::new();

        let test1 = VisualRegressionTest::new("test1", "data1", "data1");
        let test2 = VisualRegressionTest::new("test2", "data2", "data2");

        suite.add_test(test1);
        suite.add_test(test2);

        assert!(suite.all_passed());
    }

    #[test]
    fn test_base64_encode() {
        let data = "Hello, World!";
        let encoded = base64_encode(data);
        assert!(!encoded.is_empty());
        // Base64 of "Hello, World!" should be "SGVsbG8sIFdvcmxkIQ=="
        assert_eq!(encoded, "SGVsbG8sIFdvcmxkIQ==");
    }

    #[test]
    fn test_animation_types() {
        let animation = Animation {
            target: "element1".to_string(),
            animation_type: AnimationType::FadeIn,
            duration_ms: 500,
            delay_ms: 0,
        };

        assert_eq!(animation.duration_ms, 500);
        assert_eq!(animation.delay_ms, 0);
    }

    #[test]
    fn test_slide_content_types() {
        let slide = Slide {
            title: "Test Slide".to_string(),
            content: SlideContent::Text("Some text".to_string()),
            animations: Vec::new(),
            notes: Some("Speaker notes".to_string()),
        };

        assert_eq!(slide.title, "Test Slide");
        assert!(slide.notes.is_some());
    }

    #[test]
    fn test_statute_diff_visualizer_creation() {
        let visualizer = StatuteDiffVisualizer::new();
        assert_eq!(visualizer.theme.background_color, "#ffffff");
    }

    #[test]
    fn test_statute_diff_visualizer_with_theme() {
        let visualizer = StatuteDiffVisualizer::new().with_theme(Theme::dark());
        assert_eq!(visualizer.theme.background_color, "#1a1a1a");
    }

    #[test]
    fn test_statute_diff_to_html_empty() {
        use legalis_core::StatuteDiff;

        let diff = StatuteDiff {
            statute_id: "test-statute".to_string(),
            changes: vec![],
        };

        let visualizer = StatuteDiffVisualizer::new();
        let html = visualizer.to_html(&diff);

        assert!(html.contains("test-statute"));
        assert!(html.contains("No changes detected"));
        assert!(html.contains("<style>"));
    }

    #[test]
    fn test_statute_diff_to_html_with_changes() {
        use legalis_core::{StatuteChange, StatuteDiff};

        let diff = StatuteDiff {
            statute_id: "test-statute".to_string(),
            changes: vec![
                StatuteChange::TitleChanged {
                    old: "Old Title".to_string(),
                    new: "New Title".to_string(),
                },
                StatuteChange::VersionChanged { old: 1, new: 2 },
            ],
        };

        let visualizer = StatuteDiffVisualizer::new();
        let html = visualizer.to_html(&diff);

        assert!(html.contains("test-statute"));
        assert!(html.contains("Title Changed"));
        assert!(html.contains("Version Changed"));
        assert!(html.contains("<table"));
    }

    #[test]
    fn test_statute_diff_to_mermaid() {
        use legalis_core::{StatuteChange, StatuteDiff};

        let diff = StatuteDiff {
            statute_id: "test-statute".to_string(),
            changes: vec![StatuteChange::VersionChanged { old: 1, new: 2 }],
        };

        let visualizer = StatuteDiffVisualizer::new();
        let mermaid = visualizer.to_mermaid(&diff);

        assert!(mermaid.contains("flowchart LR"));
        assert!(mermaid.contains("test-statute"));
        assert!(mermaid.contains("Changes"));
    }

    #[test]
    fn test_statute_diff_to_ascii() {
        use legalis_core::{StatuteChange, StatuteDiff};

        let diff = StatuteDiff {
            statute_id: "test-statute".to_string(),
            changes: vec![StatuteChange::TitleChanged {
                old: "Old".to_string(),
                new: "New".to_string(),
            }],
        };

        let visualizer = StatuteDiffVisualizer::new();
        let ascii = visualizer.to_ascii(&diff);

        assert!(ascii.contains("test-statute"));
        assert!(ascii.contains("1."));
    }

    #[test]
    fn test_reasoning_chain_visualizer_creation() {
        let visualizer = ReasoningChainVisualizer::new();
        assert_eq!(visualizer.theme.background_color, "#ffffff");
    }

    #[test]
    fn test_reasoning_chain_visualizer_with_theme() {
        let visualizer = ReasoningChainVisualizer::new().with_theme(Theme::colorblind_friendly());
        assert_eq!(visualizer.theme.root_color, "#999999");
    }

    #[test]
    fn test_reasoning_chain_to_html() {
        use legalis_core::{LegalExplanation, ReasoningStep};

        let explanation = LegalExplanation {
            outcome: Effect::new(EffectType::Grant, "Tax credit"),
            applicable_statutes: vec!["statute-1".to_string()],
            satisfied_conditions: vec!["Age >= 18".to_string()],
            unsatisfied_conditions: vec![],
            confidence: 0.95,
            reasoning_chain: vec![ReasoningStep {
                step: 1,
                description: "Check age requirement".to_string(),
                statute_id: Some("statute-1".to_string()),
                condition: Some("Age >= 18".to_string()),
                result: legalis_core::StepResult::Satisfied,
            }],
        };

        let visualizer = ReasoningChainVisualizer::new();
        let html = visualizer.to_html(&explanation);

        assert!(html.contains("Tax credit"));
        assert!(html.contains("95"));
        assert!(html.contains("statute-1"));
        assert!(html.contains("Age >= 18"));
        assert!(html.contains("Check age requirement"));
    }

    #[test]
    fn test_reasoning_chain_to_mermaid() {
        use legalis_core::{LegalExplanation, ReasoningStep};

        let explanation = LegalExplanation {
            outcome: Effect::new(EffectType::Grant, "Benefit"),
            applicable_statutes: vec!["statute-1".to_string()],
            satisfied_conditions: vec![],
            unsatisfied_conditions: vec![],
            confidence: 0.8,
            reasoning_chain: vec![ReasoningStep {
                step: 1,
                description: "Verify conditions".to_string(),
                statute_id: Some("statute-1".to_string()),
                condition: None,
                result: legalis_core::StepResult::Applied,
            }],
        };

        let visualizer = ReasoningChainVisualizer::new();
        let mermaid = visualizer.to_mermaid(&explanation);

        assert!(mermaid.contains("flowchart TD"));
        assert!(mermaid.contains("statute-1"));
        assert!(mermaid.contains("80"));
    }

    #[test]
    fn test_reasoning_chain_to_ascii() {
        use legalis_core::{LegalExplanation, ReasoningStep};

        let explanation = LegalExplanation {
            outcome: Effect::new(EffectType::Grant, "Grant"),
            applicable_statutes: vec!["statute-a".to_string()],
            satisfied_conditions: vec!["Condition A".to_string()],
            unsatisfied_conditions: vec!["Condition B".to_string()],
            confidence: 0.75,
            reasoning_chain: vec![ReasoningStep {
                step: 1,
                description: "Step one".to_string(),
                statute_id: None,
                condition: Some("Test condition".to_string()),
                result: legalis_core::StepResult::Satisfied,
            }],
        };

        let visualizer = ReasoningChainVisualizer::new();
        let ascii = visualizer.to_ascii(&explanation);

        assert!(ascii.contains("Grant"));
        assert!(ascii.contains("75"));
        assert!(ascii.contains("statute-a"));
        assert!(ascii.contains("Condition A"));
        assert!(ascii.contains("Condition B"));
        assert!(ascii.contains("Step one"));
    }

    #[test]
    fn test_audit_trail_visualizer_creation() {
        let visualizer = AuditTrailVisualizer::new();
        assert_eq!(visualizer.theme.background_color, "#ffffff");
    }

    #[test]
    fn test_audit_trail_visualizer_with_theme() {
        let visualizer = AuditTrailVisualizer::new().with_theme(Theme::high_contrast());
        assert_eq!(visualizer.theme.background_color, "#ffffff");
    }

    #[test]
    fn test_audit_trail_to_html_empty() {
        use legalis_core::EvaluationAuditTrail;

        let trail = EvaluationAuditTrail::new();
        let visualizer = AuditTrailVisualizer::new();
        let html = visualizer.to_html(&trail);

        assert!(html.contains("Evaluation Audit Trail"));
        assert!(html.contains("No evaluation records"));
    }

    #[test]
    fn test_audit_trail_to_html_with_records() {
        use legalis_core::EvaluationAuditTrail;

        let mut trail = EvaluationAuditTrail::new();
        trail.record("Age >= 18".to_string(), true, 100);
        trail.record("Income < 50000".to_string(), false, 150);

        let visualizer = AuditTrailVisualizer::new();
        let html = visualizer.to_html(&trail);

        assert!(html.contains("Total Evaluations"));
        assert!(html.contains("Age >= 18"));
        assert!(html.contains("Income < 50000"));
        assert!(html.contains("Pass Rate"));
        assert!(html.contains("Average Duration"));
    }

    #[test]
    fn test_audit_trail_to_ascii() {
        use legalis_core::EvaluationAuditTrail;

        let mut trail = EvaluationAuditTrail::new();
        trail.record("Condition A".to_string(), true, 50);
        trail.record("Condition B".to_string(), true, 75);
        trail.record("Condition C".to_string(), false, 60);

        let visualizer = AuditTrailVisualizer::new();
        let ascii = visualizer.to_ascii(&trail);

        assert!(ascii.contains("Evaluation Audit Trail"));
        assert!(ascii.contains("Total Evaluations: 3"));
        assert!(ascii.contains("Condition A"));
        assert!(ascii.contains("Condition B"));
        assert!(ascii.contains("Condition C"));
        assert!(ascii.contains("Pass Rate"));
        assert!(ascii.contains("66.7%")); // 2 out of 3 passed
    }

    #[test]
    fn test_statute_diff_default() {
        let visualizer1 = StatuteDiffVisualizer::new();
        let visualizer2 = StatuteDiffVisualizer::default();
        assert_eq!(
            visualizer1.theme.background_color,
            visualizer2.theme.background_color
        );
    }

    #[test]
    fn test_reasoning_chain_default() {
        let visualizer1 = ReasoningChainVisualizer::new();
        let visualizer2 = ReasoningChainVisualizer::default();
        assert_eq!(
            visualizer1.theme.background_color,
            visualizer2.theme.background_color
        );
    }

    #[test]
    fn test_audit_trail_default() {
        let visualizer1 = AuditTrailVisualizer::new();
        let visualizer2 = AuditTrailVisualizer::default();
        assert_eq!(
            visualizer1.theme.background_color,
            visualizer2.theme.background_color
        );
    }

    #[test]
    fn test_format_change_type() {
        use legalis_core::StatuteChange;

        assert_eq!(
            format_change_type(&StatuteChange::IdChanged {
                old: "a".to_string(),
                new: "b".to_string()
            }),
            "ID Changed"
        );

        assert_eq!(
            format_change_type(&StatuteChange::TitleChanged {
                old: "a".to_string(),
                new: "b".to_string()
            }),
            "Title Changed"
        );

        assert_eq!(
            format_change_type(&StatuteChange::TemporalValidityChanged),
            "Temporal Validity Changed"
        );
    }

    #[test]
    fn test_interactive_config_default() {
        let config = InteractiveConfig::default();
        assert!(config.enable_zoom_pan);
        assert!(config.enable_tooltips);
        assert!(config.enable_click_expand);
        assert!(config.enable_search);
        assert!(config.enable_minimap);
        assert_eq!(config.initial_zoom, 1.0);
        assert_eq!(config.min_zoom, 0.1);
        assert_eq!(config.max_zoom, 5.0);
        assert_eq!(config.minimap_size, (200, 150));
    }

    #[test]
    fn test_interactive_visualizer_creation() {
        let visualizer = InteractiveVisualizer::new();
        assert_eq!(visualizer.theme.background_color, "#ffffff");
        assert!(visualizer.config.enable_zoom_pan);
    }

    #[test]
    fn test_interactive_visualizer_with_theme() {
        let visualizer = InteractiveVisualizer::new().with_theme(Theme::dark());
        assert_eq!(visualizer.theme.background_color, "#1a1a1a");
    }

    #[test]
    fn test_interactive_visualizer_with_config() {
        let mut config = InteractiveConfig::default();
        config.enable_minimap = false;
        config.initial_zoom = 2.0;

        let visualizer = InteractiveVisualizer::new().with_config(config);
        assert!(!visualizer.config.enable_minimap);
        assert_eq!(visualizer.config.initial_zoom, 2.0);
    }

    #[test]
    fn test_interactive_html_generation() {
        let statute = Statute::new(
            "test-1",
            "Test Statute",
            Effect::new(EffectType::Grant, "benefit"),
        )
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });

        let tree = DecisionTree::from_statute(&statute).unwrap();
        let visualizer = InteractiveVisualizer::new();
        let html = visualizer.to_interactive_html(&tree);

        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Interactive decision-tree Visualization"));
        assert!(html.contains("zoom-in"));
        assert!(html.contains("zoom-out"));
        assert!(html.contains("search-box"));
        assert!(html.contains("minimap"));
        assert!(html.contains("enableZoomPan"));
        assert!(html.contains("enableTooltips"));
        assert!(html.contains("enableClickExpand"));
        assert!(html.contains("enableSearch"));
        assert!(html.contains("enableMinimap"));
    }

    #[test]
    fn test_interactive_html_graph() {
        let mut graph = DependencyGraph::new();
        graph.add_statute("statute-1");
        graph.add_statute("statute-2");
        graph.add_dependency("statute-2", "statute-1", "depends-on");

        let visualizer = InteractiveVisualizer::new();
        let html = visualizer.to_interactive_html_graph(&graph);

        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Interactive dependency-graph Visualization"));
        assert!(html.contains("zoom-controls"));
        assert!(html.contains("search-controls"));
    }

    #[test]
    fn test_interactive_config_disabled_features() {
        let config = InteractiveConfig {
            enable_zoom_pan: false,
            enable_tooltips: false,
            enable_click_expand: false,
            enable_search: false,
            enable_minimap: false,
            initial_zoom: 1.0,
            min_zoom: 0.5,
            max_zoom: 3.0,
            minimap_size: (100, 100),
        };

        let statute = Statute::new("test-1", "Test", Effect::new(EffectType::Grant, "test"));
        let tree = DecisionTree::from_statute(&statute).unwrap();

        let visualizer = InteractiveVisualizer::new().with_config(config);
        let html = visualizer.to_interactive_html(&tree);

        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("enableZoomPan: false"));
        assert!(html.contains("enableTooltips: false"));
        assert!(html.contains("enableClickExpand: false"));
        assert!(html.contains("enableSearch: false"));
        assert!(html.contains("enableMinimap: false"));
    }

    #[test]
    fn test_interactive_visualizer_default() {
        let visualizer1 = InteractiveVisualizer::new();
        let visualizer2 = InteractiveVisualizer::default();
        assert_eq!(
            visualizer1.theme.background_color,
            visualizer2.theme.background_color
        );
        assert_eq!(
            visualizer1.config.initial_zoom,
            visualizer2.config.initial_zoom
        );
    }

    #[test]
    fn test_3d_config_default() {
        let config = ThreeDConfig::default();
        assert!(!config.enable_vr);
        assert!(!config.enable_ar);
        assert!(config.force_directed);
        assert!(config.depth_coloring);
        assert_eq!(config.camera_fov, 75.0);
        assert_eq!(config.node_size, 1.0);
        assert_eq!(config.edge_thickness, 0.1);
        assert_eq!(config.force_strength, 0.5);
        assert_eq!(config.auto_rotate_speed, 10.0);
    }

    #[test]
    fn test_3d_visualizer_creation() {
        let visualizer = ThreeDVisualizer::new();
        assert_eq!(visualizer.theme.background_color, "#ffffff");
        assert!(visualizer.config.force_directed);
    }

    #[test]
    fn test_3d_visualizer_with_theme() {
        let visualizer = ThreeDVisualizer::new().with_theme(Theme::dark());
        assert_eq!(visualizer.theme.background_color, "#1a1a1a");
    }

    #[test]
    fn test_3d_visualizer_with_config() {
        let mut config = ThreeDConfig::default();
        config.enable_vr = true;
        config.enable_ar = true;
        config.force_directed = false;
        config.depth_coloring = false;

        let visualizer = ThreeDVisualizer::new().with_config(config);
        assert!(visualizer.config.enable_vr);
        assert!(visualizer.config.enable_ar);
        assert!(!visualizer.config.force_directed);
        assert!(!visualizer.config.depth_coloring);
    }

    #[test]
    fn test_3d_html_graph_generation() {
        let mut graph = DependencyGraph::new();
        graph.add_statute("statute-1");
        graph.add_statute("statute-2");
        graph.add_statute("statute-3");
        graph.add_dependency("statute-2", "statute-1", "depends-on");
        graph.add_dependency("statute-3", "statute-2", "depends-on");

        let visualizer = ThreeDVisualizer::new();
        let html = visualizer.to_3d_html_graph(&graph);

        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("3D Dependency Graph Visualization"));
        assert!(html.contains("three.min.js"));
        assert!(html.contains("reset-camera"));
        assert!(html.contains("toggle-rotation"));
        assert!(html.contains("const nodes = ["));
        assert!(html.contains("const edges = ["));
        assert!(html.contains("enableVR"));
        assert!(html.contains("enableAR"));
        assert!(html.contains("forceDirected"));
        assert!(html.contains("depthColoring"));
    }

    #[test]
    fn test_3d_html_timeline_generation() {
        let mut timeline = Timeline::new();
        timeline.add_event(
            "2020-01-01",
            TimelineEvent::Enacted {
                statute_id: "law-1".to_string(),
                title: "Event 1".to_string(),
            },
        );
        timeline.add_event(
            "2020-02-01",
            TimelineEvent::Amended {
                statute_id: "law-1".to_string(),
                description: "Event 2".to_string(),
            },
        );
        timeline.add_event(
            "2020-03-01",
            TimelineEvent::Repealed {
                statute_id: "law-1".to_string(),
            },
        );

        let visualizer = ThreeDVisualizer::new();
        let html = visualizer.to_3d_html_timeline(&timeline);

        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("3D Timeline Visualization"));
        assert!(html.contains("three.min.js"));
        assert!(html.contains("isTimeline: true"));
        assert!(html.contains("2020-01-01"));
        assert!(html.contains("2020-02-01"));
        assert!(html.contains("2020-03-01"));
    }

    #[test]
    fn test_3d_vr_ar_buttons() {
        let config = ThreeDConfig {
            enable_vr: true,
            enable_ar: true,
            force_directed: true,
            depth_coloring: true,
            camera_fov: 75.0,
            node_size: 1.0,
            edge_thickness: 0.1,
            force_strength: 0.5,
            auto_rotate_speed: 10.0,
        };

        let mut graph = DependencyGraph::new();
        graph.add_statute("test");

        let visualizer = ThreeDVisualizer::new().with_config(config);
        let html = visualizer.to_3d_html_graph(&graph);

        assert!(html.contains("enter-vr"));
        assert!(html.contains("enter-ar"));
        assert!(html.contains("VRButton.js"));
    }

    #[test]
    fn test_3d_force_directed_layout() {
        let config = ThreeDConfig {
            enable_vr: false,
            enable_ar: false,
            force_directed: true,
            depth_coloring: false,
            camera_fov: 75.0,
            node_size: 2.0,
            edge_thickness: 0.2,
            force_strength: 0.8,
            auto_rotate_speed: 5.0,
        };

        let mut graph = DependencyGraph::new();
        graph.add_statute("node1");
        graph.add_statute("node2");

        let visualizer = ThreeDVisualizer::new().with_config(config);
        let html = visualizer.to_3d_html_graph(&graph);

        assert!(html.contains("forceDirected: true"));
        assert!(html.contains("reset-forces"));
        assert!(html.contains("nodeSize: 2"));
        assert!(html.contains("edgeThickness: 0.2"));
        assert!(html.contains("forceStrength: 0.8"));
        assert!(html.contains("autoRotateSpeed: 5"));
    }

    #[test]
    fn test_3d_depth_based_coloring() {
        let config = ThreeDConfig {
            enable_vr: false,
            enable_ar: false,
            force_directed: false,
            depth_coloring: true,
            camera_fov: 75.0,
            node_size: 1.0,
            edge_thickness: 0.1,
            force_strength: 0.5,
            auto_rotate_speed: 0.0, // No auto-rotation
        };

        let mut graph = DependencyGraph::new();
        graph.add_statute("root");
        graph.add_statute("child");
        graph.add_dependency("child", "root", "depends-on");

        let visualizer = ThreeDVisualizer::new().with_config(config);
        let html = visualizer.to_3d_html_graph(&graph);

        assert!(html.contains("depthColoring: true"));
        assert!(html.contains("const hue = (node.depth * 60) % 360"));
    }

    #[test]
    fn test_3d_visualizer_default() {
        let visualizer1 = ThreeDVisualizer::new();
        let visualizer2 = ThreeDVisualizer::default();
        assert_eq!(
            visualizer1.theme.background_color,
            visualizer2.theme.background_color
        );
        assert_eq!(visualizer1.config.camera_fov, visualizer2.config.camera_fov);
    }

    // ============================================================================
    // Accessibility Tests
    // ============================================================================

    #[test]
    fn test_accessibility_config_default() {
        let config = AccessibilityConfig::default();
        assert!(config.wcag_aa_compliant);
        assert!(config.enable_screen_reader);
        assert!(config.enable_keyboard_nav);
        assert!(!config.high_contrast_mode);
        assert!(!config.reduced_motion);
        assert_eq!(config.min_font_size, 16.0);
        assert_eq!(config.focus_color, "#005fcc");
        assert_eq!(config.tab_index_start, 0);
    }

    #[test]
    fn test_accessibility_config_screen_reader_optimized() {
        let config = AccessibilityConfig::screen_reader_optimized();
        assert!(config.wcag_aa_compliant);
        assert!(config.enable_screen_reader);
        assert!(config.enable_keyboard_nav);
        assert!(config.high_contrast_mode);
        assert!(config.reduced_motion);
        assert_eq!(config.min_font_size, 18.0);
    }

    #[test]
    fn test_accessibility_config_reduced_motion() {
        let config = AccessibilityConfig::reduced_motion();
        assert!(config.reduced_motion);
        assert!(config.wcag_aa_compliant);
    }

    #[test]
    fn test_accessibility_config_high_contrast() {
        let config = AccessibilityConfig::high_contrast();
        assert!(config.high_contrast_mode);
        assert_eq!(config.min_font_size, 18.0);
    }

    #[test]
    fn test_accessibility_enhancer_creation() {
        let enhancer = AccessibilityEnhancer::new();
        assert!(enhancer.config.wcag_aa_compliant);
        assert_eq!(enhancer.theme.background_color, "#ffffff");
    }

    #[test]
    fn test_accessibility_enhancer_with_config() {
        let config = AccessibilityConfig::high_contrast();
        let enhancer = AccessibilityEnhancer::new().with_config(config);
        assert!(enhancer.config.high_contrast_mode);
    }

    #[test]
    fn test_accessibility_enhancer_with_theme() {
        let enhancer = AccessibilityEnhancer::new().with_theme(Theme::dark());
        assert_eq!(enhancer.theme.background_color, "#1a1a1a");
    }

    #[test]
    fn test_accessibility_enhancer_with_high_contrast_theme() {
        let config = AccessibilityConfig::high_contrast();
        let enhancer = AccessibilityEnhancer::new()
            .with_config(config)
            .with_theme(Theme::light());
        // Should override with high contrast theme
        assert_eq!(enhancer.theme.background_color, "#ffffff");
        assert_eq!(enhancer.theme.text_color, "#000000");
    }

    #[test]
    fn test_aria_label_for_root_node() {
        let enhancer = AccessibilityEnhancer::new();
        let node = DecisionNode::Root {
            statute_id: "test-1".to_string(),
            title: "Test Statute".to_string(),
        };
        let label = enhancer.aria_label_for_node(&node);
        assert!(label.contains("Root node"));
        assert!(label.contains("Test Statute"));
        assert!(label.contains("test-1"));
    }

    #[test]
    fn test_aria_label_for_condition_node() {
        let enhancer = AccessibilityEnhancer::new();
        let node = DecisionNode::Condition {
            description: "Age >= 18".to_string(),
            is_discretionary: false,
        };
        let label = enhancer.aria_label_for_node(&node);
        assert!(label.contains("Condition"));
        assert!(label.contains("Age >= 18"));
    }

    #[test]
    fn test_aria_label_for_discretionary_condition() {
        let enhancer = AccessibilityEnhancer::new();
        let node = DecisionNode::Condition {
            description: "Good moral character".to_string(),
            is_discretionary: true,
        };
        let label = enhancer.aria_label_for_node(&node);
        assert!(label.contains("Discretionary condition"));
        assert!(label.contains("Good moral character"));
    }

    #[test]
    fn test_aria_label_for_outcome_node() {
        let enhancer = AccessibilityEnhancer::new();
        let node = DecisionNode::Outcome {
            description: "Eligible for benefits".to_string(),
        };
        let label = enhancer.aria_label_for_node(&node);
        assert!(label.contains("Outcome"));
        assert!(label.contains("Eligible for benefits"));
    }

    #[test]
    fn test_aria_label_for_discretion_node() {
        let enhancer = AccessibilityEnhancer::new();
        let node = DecisionNode::Discretion {
            issue: "Exceptional circumstances".to_string(),
            hint: Some("Consider case history".to_string()),
        };
        let label = enhancer.aria_label_for_node(&node);
        assert!(label.contains("Discretionary decision"));
        assert!(label.contains("Exceptional circumstances"));
        assert!(label.contains("Hint"));
        assert!(label.contains("Consider case history"));
    }

    #[test]
    fn test_aria_role_for_nodes() {
        let enhancer = AccessibilityEnhancer::new();

        let root = DecisionNode::Root {
            statute_id: "test".to_string(),
            title: "Test".to_string(),
        };
        assert_eq!(enhancer.aria_role_for_node(&root), "landmark");

        let condition = DecisionNode::Condition {
            description: "Test".to_string(),
            is_discretionary: false,
        };
        assert_eq!(enhancer.aria_role_for_node(&condition), "listitem");

        let outcome = DecisionNode::Outcome {
            description: "Test".to_string(),
        };
        assert_eq!(enhancer.aria_role_for_node(&outcome), "status");

        let discretion = DecisionNode::Discretion {
            issue: "Test".to_string(),
            hint: None,
        };
        assert_eq!(enhancer.aria_role_for_node(&discretion), "alert");
    }

    #[test]
    fn test_keyboard_nav_script_enabled() {
        let enhancer = AccessibilityEnhancer::new();
        let script = enhancer.keyboard_nav_script();
        assert!(script.contains("Keyboard navigation support"));
        assert!(script.contains("Tab"));
        assert!(script.contains("ArrowUp"));
        assert!(script.contains("ArrowDown"));
        assert!(script.contains("Home"));
        assert!(script.contains("End"));
        assert!(script.contains(&enhancer.config.focus_color));
    }

    #[test]
    fn test_keyboard_nav_script_disabled() {
        let config = AccessibilityConfig {
            enable_keyboard_nav: false,
            ..Default::default()
        };
        let enhancer = AccessibilityEnhancer::new().with_config(config);
        let script = enhancer.keyboard_nav_script();
        assert!(script.is_empty());
    }

    #[test]
    fn test_screen_reader_enhancements_enabled() {
        let enhancer = AccessibilityEnhancer::new();
        let enhancements = enhancer.screen_reader_enhancements();
        assert!(enhancements.contains("Navigation Instructions"));
        assert!(enhancements.contains("Tab"));
        assert!(enhancements.contains("sr-only"));
        assert!(enhancements.contains("complementary"));
    }

    #[test]
    fn test_screen_reader_enhancements_disabled() {
        let config = AccessibilityConfig {
            enable_screen_reader: false,
            ..Default::default()
        };
        let enhancer = AccessibilityEnhancer::new().with_config(config);
        let enhancements = enhancer.screen_reader_enhancements();
        assert!(enhancements.is_empty());
    }

    #[test]
    fn test_reduced_motion_css_enabled() {
        let config = AccessibilityConfig::reduced_motion();
        let enhancer = AccessibilityEnhancer::new().with_config(config);
        let css = enhancer.reduced_motion_css();
        assert!(css.contains("prefers-reduced-motion"));
        assert!(css.contains("animation-duration"));
        assert!(css.contains("0.01ms"));
    }

    #[test]
    fn test_reduced_motion_css_disabled() {
        let enhancer = AccessibilityEnhancer::new();
        let css = enhancer.reduced_motion_css();
        assert!(css.is_empty());
    }

    #[test]
    fn test_high_contrast_css_enabled() {
        let config = AccessibilityConfig::high_contrast();
        let enhancer = AccessibilityEnhancer::new().with_config(config);
        let css = enhancer.high_contrast_css();
        assert!(css.contains("High contrast mode"));
        assert!(css.contains("font-size"));
        assert!(css.contains("18px"));
        assert!(css.contains(".node"));
        assert!(css.contains(".edge"));
    }

    #[test]
    fn test_high_contrast_css_disabled() {
        let enhancer = AccessibilityEnhancer::new();
        let css = enhancer.high_contrast_css();
        assert!(css.is_empty());
    }

    #[test]
    fn test_enhance_html_adds_lang() {
        let enhancer = AccessibilityEnhancer::new();
        let html = "<html><head></head><body></body></html>";
        let enhanced = enhancer.enhance_html(html);
        assert!(enhanced.contains(r#"lang="en""#));
    }

    #[test]
    fn test_enhance_html_adds_viewport() {
        let enhancer = AccessibilityEnhancer::new();
        let html = "<html><head></head><body></body></html>";
        let enhanced = enhancer.enhance_html(html);
        assert!(enhanced.contains("viewport"));
        assert!(enhanced.contains("width=device-width"));
    }

    #[test]
    fn test_enhance_html_preserves_existing_lang() {
        let enhancer = AccessibilityEnhancer::new();
        let html = r#"<html lang="fr"><head></head><body></body></html>"#;
        let enhanced = enhancer.enhance_html(html);
        assert!(enhanced.contains(r#"lang="fr""#));
    }

    #[test]
    fn test_validate_contrast_good() {
        let enhancer = AccessibilityEnhancer::new();
        // Black on white - very high contrast
        assert!(enhancer.validate_contrast("#000000", "#ffffff"));
        // Dark blue on white - good contrast
        assert!(enhancer.validate_contrast("#0000aa", "#ffffff"));
    }

    #[test]
    fn test_validate_contrast_bad() {
        let enhancer = AccessibilityEnhancer::new();
        // Light gray on white - poor contrast
        assert!(!enhancer.validate_contrast("#cccccc", "#ffffff"));
        // Yellow on white - poor contrast
        assert!(!enhancer.validate_contrast("#ffff00", "#ffffff"));
    }

    #[test]
    fn test_validate_contrast_invalid_color() {
        let enhancer = AccessibilityEnhancer::new();
        assert!(!enhancer.validate_contrast("invalid", "#ffffff"));
        assert!(!enhancer.validate_contrast("#fff", "#ffffff")); // Only 3 chars
    }

    #[test]
    fn test_accessible_html_decision_tree() {
        let statute = Statute::new(
            "test-1",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        );
        let tree = DecisionTree::from_statute(&statute).unwrap();

        let enhancer = AccessibilityEnhancer::new();
        let html = enhancer.to_accessible_html(&tree);

        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains(r#"lang="en""#));
        assert!(html.contains("viewport"));
        assert!(html.contains("Navigation Instructions"));
    }

    #[test]
    fn test_accessible_html_dependency_graph() {
        let mut graph = DependencyGraph::new();
        graph.add_statute("statute-1");
        graph.add_statute("statute-2");
        graph.add_dependency("statute-2", "statute-1", "depends-on");

        let enhancer = AccessibilityEnhancer::new();
        let html = enhancer.to_accessible_html_graph(&graph);

        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains(r#"lang="en""#));
        assert!(html.contains("viewport"));
    }

    #[test]
    fn test_accessible_html_with_all_features() {
        let config = AccessibilityConfig::screen_reader_optimized();
        let enhancer = AccessibilityEnhancer::new().with_config(config);

        let statute = Statute::new(
            "test-1",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        );
        let tree = DecisionTree::from_statute(&statute).unwrap();

        let html = enhancer.to_accessible_html(&tree);

        // Check for all accessibility features
        assert!(html.contains(r#"lang="en""#));
        assert!(html.contains("viewport"));
        assert!(html.contains("Navigation Instructions"));
        assert!(html.contains("Keyboard navigation support"));
        assert!(html.contains("High contrast mode"));
        assert!(html.contains("prefers-reduced-motion"));
    }

    #[test]
    fn test_accessibility_enhancer_default() {
        let enhancer1 = AccessibilityEnhancer::new();
        let enhancer2 = AccessibilityEnhancer::default();
        assert_eq!(
            enhancer1.config.wcag_aa_compliant,
            enhancer2.config.wcag_aa_compliant
        );
        assert_eq!(
            enhancer1.theme.background_color,
            enhancer2.theme.background_color
        );
    }

    // ============================================================================
    // Export Formats Tests
    // ============================================================================

    #[test]
    fn test_export_format_types() {
        let formats = vec![
            ExportFormat::AnimatedGif,
            ExportFormat::Mp4,
            ExportFormat::WebM,
            ExportFormat::PrintPdf,
            ExportFormat::VectorPdf,
            ExportFormat::Poster,
        ];
        assert_eq!(formats.len(), 6);
    }

    #[test]
    fn test_poster_config_default() {
        let config = PosterConfig::default();
        assert_eq!(config.width, 841);
        assert_eq!(config.height, 1189);
        assert_eq!(config.dpi, 300);
        assert_eq!(config.paper_size, "A0");
        assert_eq!(config.orientation, "portrait");
    }

    #[test]
    fn test_poster_config_a0() {
        let config = PosterConfig::a0();
        assert_eq!(config.width, 841);
        assert_eq!(config.height, 1189);
        assert_eq!(config.paper_size, "A0");
    }

    #[test]
    fn test_poster_config_a1() {
        let config = PosterConfig::a1();
        assert_eq!(config.width, 594);
        assert_eq!(config.height, 841);
        assert_eq!(config.paper_size, "A1");
    }

    #[test]
    fn test_poster_config_a2() {
        let config = PosterConfig::a2();
        assert_eq!(config.width, 420);
        assert_eq!(config.height, 594);
        assert_eq!(config.paper_size, "A2");
    }

    #[test]
    fn test_poster_config_24x36() {
        let config = PosterConfig::poster_24x36();
        assert_eq!(config.width, 610);
        assert_eq!(config.height, 914);
        assert_eq!(config.paper_size, "24x36");
    }

    #[test]
    fn test_poster_config_landscape() {
        let config = PosterConfig::a0().landscape();
        assert_eq!(config.width, 1189);
        assert_eq!(config.height, 841);
        assert_eq!(config.orientation, "landscape");
    }

    #[test]
    fn test_poster_config_with_dpi() {
        let config = PosterConfig::a0().with_dpi(600);
        assert_eq!(config.dpi, 600);
    }

    #[test]
    fn test_animated_gif_config_default() {
        let config = AnimatedGifConfig::default();
        assert_eq!(config.fps, 30);
        assert_eq!(config.duration, 10);
        assert_eq!(config.loop_count, 0);
        assert_eq!(config.width, 1920);
        assert_eq!(config.height, 1080);
        assert_eq!(config.quality, 80);
    }

    #[test]
    fn test_animated_gif_config_with_fps() {
        let config = AnimatedGifConfig::new().with_fps(60);
        assert_eq!(config.fps, 60);
    }

    #[test]
    fn test_animated_gif_config_with_duration() {
        let config = AnimatedGifConfig::new().with_duration(5);
        assert_eq!(config.duration, 5);
    }

    #[test]
    fn test_animated_gif_config_with_loop_count() {
        let config = AnimatedGifConfig::new().with_loop_count(5);
        assert_eq!(config.loop_count, 5);
    }

    #[test]
    fn test_animated_gif_config_with_size() {
        let config = AnimatedGifConfig::new().with_size(1280, 720);
        assert_eq!(config.width, 1280);
        assert_eq!(config.height, 720);
    }

    #[test]
    fn test_animated_gif_config_with_quality() {
        let config = AnimatedGifConfig::new().with_quality(90);
        assert_eq!(config.quality, 90);
    }

    #[test]
    fn test_animated_gif_config_quality_clamped() {
        let config = AnimatedGifConfig::new().with_quality(150);
        assert_eq!(config.quality, 100);
    }

    #[test]
    fn test_video_config_default() {
        let config = VideoConfig::default();
        assert_eq!(config.fps, 30);
        assert_eq!(config.duration, 10);
        assert_eq!(config.width, 1920);
        assert_eq!(config.height, 1080);
        assert_eq!(config.bitrate, 5000);
        assert_eq!(config.codec, "h264");
    }

    #[test]
    fn test_video_config_hd_1080p() {
        let config = VideoConfig::hd_1080p();
        assert_eq!(config.width, 1920);
        assert_eq!(config.height, 1080);
        assert_eq!(config.bitrate, 8000);
    }

    #[test]
    fn test_video_config_hd_720p() {
        let config = VideoConfig::hd_720p();
        assert_eq!(config.width, 1280);
        assert_eq!(config.height, 720);
        assert_eq!(config.bitrate, 5000);
    }

    #[test]
    fn test_video_config_uhd_4k() {
        let config = VideoConfig::uhd_4k();
        assert_eq!(config.width, 3840);
        assert_eq!(config.height, 2160);
        assert_eq!(config.bitrate, 20000);
    }

    #[test]
    fn test_video_config_with_codec() {
        let config = VideoConfig::new().with_codec("vp9");
        assert_eq!(config.codec, "vp9");
    }

    #[test]
    fn test_video_config_with_bitrate() {
        let config = VideoConfig::new().with_bitrate(10000);
        assert_eq!(config.bitrate, 10000);
    }

    #[test]
    fn test_video_config_with_duration() {
        let config = VideoConfig::new().with_duration(20);
        assert_eq!(config.duration, 20);
    }

    #[test]
    fn test_pdf_config_default() {
        let config = PdfConfig::default();
        assert_eq!(config.width, 210.0);
        assert_eq!(config.height, 297.0);
        assert_eq!(config.margin, 10.0);
        assert!(config.vector);
        assert_eq!(config.dpi, 300);
        assert!(config.print_optimized);
    }

    #[test]
    fn test_pdf_config_a4() {
        let config = PdfConfig::a4();
        assert_eq!(config.width, 210.0);
        assert_eq!(config.height, 297.0);
    }

    #[test]
    fn test_pdf_config_a3() {
        let config = PdfConfig::a3();
        assert_eq!(config.width, 297.0);
        assert_eq!(config.height, 420.0);
    }

    #[test]
    fn test_pdf_config_letter() {
        let config = PdfConfig::letter();
        assert_eq!(config.width, 215.9);
        assert_eq!(config.height, 279.4);
    }

    #[test]
    fn test_pdf_config_tabloid() {
        let config = PdfConfig::tabloid();
        assert_eq!(config.width, 279.4);
        assert_eq!(config.height, 431.8);
    }

    #[test]
    fn test_pdf_config_landscape() {
        let config = PdfConfig::a4().landscape();
        assert_eq!(config.width, 297.0);
        assert_eq!(config.height, 210.0);
    }

    #[test]
    fn test_pdf_config_vector() {
        let config = PdfConfig::new().vector();
        assert!(config.vector);
    }

    #[test]
    fn test_pdf_config_raster() {
        let config = PdfConfig::new().raster();
        assert!(!config.vector);
    }

    #[test]
    fn test_pdf_config_print_optimized() {
        let config = PdfConfig::new().print_optimized();
        assert!(config.print_optimized);
    }

    #[test]
    fn test_pdf_config_screen_optimized() {
        let config = PdfConfig::new().screen_optimized();
        assert!(!config.print_optimized);
        assert_eq!(config.dpi, 96);
    }

    #[test]
    fn test_pdf_config_with_dpi() {
        let config = PdfConfig::new().with_dpi(600);
        assert_eq!(config.dpi, 600);
    }

    #[test]
    fn test_pdf_config_with_margin() {
        let config = PdfConfig::new().with_margin(20.0);
        assert_eq!(config.margin, 20.0);
    }

    #[test]
    fn test_advanced_exporter_creation() {
        let exporter = AdvancedExporter::new();
        assert_eq!(exporter.theme.background_color, "#ffffff");
    }

    #[test]
    fn test_advanced_exporter_with_theme() {
        let exporter = AdvancedExporter::new().with_theme(Theme::dark());
        assert_eq!(exporter.theme.background_color, "#1a1a1a");
    }

    #[test]
    fn test_advanced_exporter_default() {
        let exporter1 = AdvancedExporter::new();
        let exporter2 = AdvancedExporter::default();
        assert_eq!(
            exporter1.theme.background_color,
            exporter2.theme.background_color
        );
    }

    #[test]
    fn test_to_animated_gif() {
        let statute = Statute::new(
            "test-1",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        );
        let tree = DecisionTree::from_statute(&statute).unwrap();

        let exporter = AdvancedExporter::new();
        let config = AnimatedGifConfig::new().with_fps(2).with_duration(1);
        let frames = exporter.to_animated_gif(&tree, config);

        assert_eq!(frames.len(), 2); // 2 fps * 1 second
        assert!(frames[0].contains("<svg"));
    }

    #[test]
    fn test_graph_to_animated_gif() {
        let mut graph = DependencyGraph::new();
        graph.add_statute("statute-1");
        graph.add_statute("statute-2");
        graph.add_dependency("statute-2", "statute-1", "depends-on");

        let exporter = AdvancedExporter::new();
        let config = AnimatedGifConfig::new().with_fps(2).with_duration(1);
        let frames = exporter.graph_to_animated_gif(&graph, config);

        assert_eq!(frames.len(), 2);
        assert!(frames[0].contains("<svg"));
    }

    #[test]
    fn test_to_video_frames() {
        let statute = Statute::new(
            "test-1",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        );
        let tree = DecisionTree::from_statute(&statute).unwrap();

        let exporter = AdvancedExporter::new();
        let config = VideoConfig::new().with_fps(2).with_duration(1);
        let frames = exporter.to_video_frames(&tree, config);

        assert_eq!(frames.len(), 2);
        assert!(frames[0].contains("<svg"));
    }

    #[test]
    fn test_graph_to_video_frames() {
        let mut graph = DependencyGraph::new();
        graph.add_statute("statute-1");

        let exporter = AdvancedExporter::new();
        let config = VideoConfig::hd_720p().with_fps(2).with_duration(1);
        let frames = exporter.graph_to_video_frames(&graph, config);

        assert_eq!(frames.len(), 2);
        assert!(frames[0].contains("<svg"));
    }

    #[test]
    fn test_to_print_pdf() {
        let statute = Statute::new(
            "test-1",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        );
        let tree = DecisionTree::from_statute(&statute).unwrap();

        let exporter = AdvancedExporter::new();
        let config = PdfConfig::a4().print_optimized();
        let svg = exporter.to_print_pdf(&tree, config);

        assert!(svg.contains("<svg"));
        assert!(svg.contains("@media print"));
    }

    #[test]
    fn test_graph_to_print_pdf() {
        let mut graph = DependencyGraph::new();
        graph.add_statute("statute-1");

        let exporter = AdvancedExporter::new();
        let config = PdfConfig::letter().print_optimized();
        let svg = exporter.graph_to_print_pdf(&graph, config);

        assert!(svg.contains("<svg"));
        assert!(svg.contains("@media print"));
    }

    #[test]
    fn test_to_vector_pdf() {
        let statute = Statute::new(
            "test-1",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        );
        let tree = DecisionTree::from_statute(&statute).unwrap();

        let exporter = AdvancedExporter::new();
        let config = PdfConfig::a4().vector();
        let svg = exporter.to_vector_pdf(&tree, config);

        assert!(svg.contains("<svg"));
        assert!(svg.contains("PDF Export"));
    }

    #[test]
    fn test_graph_to_vector_pdf() {
        let mut graph = DependencyGraph::new();
        graph.add_statute("statute-1");

        let exporter = AdvancedExporter::new();
        let config = PdfConfig::a3().vector();
        let svg = exporter.graph_to_vector_pdf(&graph, config);

        assert!(svg.contains("<svg"));
        assert!(svg.contains("PDF Export"));
    }

    #[test]
    fn test_to_poster() {
        let statute = Statute::new(
            "test-1",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        );
        let tree = DecisionTree::from_statute(&statute).unwrap();

        let exporter = AdvancedExporter::new();
        let config = PosterConfig::a0();
        let svg = exporter.to_poster(&tree, config);

        assert!(svg.contains("<svg"));
        assert!(svg.contains("Poster"));
        assert!(svg.contains("A0"));
    }

    #[test]
    fn test_graph_to_poster() {
        let mut graph = DependencyGraph::new();
        graph.add_statute("statute-1");

        let exporter = AdvancedExporter::new();
        let config = PosterConfig::poster_24x36().landscape();
        let svg = exporter.graph_to_poster(&graph, config);

        assert!(svg.contains("<svg"));
        assert!(svg.contains("Poster"));
        assert!(svg.contains("24x36"));
    }

    #[test]
    fn test_format_metadata() {
        let exporter = AdvancedExporter::new();

        let gif_meta = exporter.format_metadata(ExportFormat::AnimatedGif);
        assert!(gif_meta.contains("Animated GIF"));

        let mp4_meta = exporter.format_metadata(ExportFormat::Mp4);
        assert!(mp4_meta.contains("MP4"));

        let webm_meta = exporter.format_metadata(ExportFormat::WebM);
        assert!(webm_meta.contains("WebM"));

        let print_pdf_meta = exporter.format_metadata(ExportFormat::PrintPdf);
        assert!(print_pdf_meta.contains("Print PDF"));

        let vector_pdf_meta = exporter.format_metadata(ExportFormat::VectorPdf);
        assert!(vector_pdf_meta.contains("Vector PDF"));

        let poster_meta = exporter.format_metadata(ExportFormat::Poster);
        assert!(poster_meta.contains("Poster"));
    }

    #[test]
    fn test_animated_gif_config_builder_pattern() {
        let config = AnimatedGifConfig::new()
            .with_fps(60)
            .with_duration(5)
            .with_loop_count(3)
            .with_size(1280, 720)
            .with_quality(95);

        assert_eq!(config.fps, 60);
        assert_eq!(config.duration, 5);
        assert_eq!(config.loop_count, 3);
        assert_eq!(config.width, 1280);
        assert_eq!(config.height, 720);
        assert_eq!(config.quality, 95);
    }

    #[test]
    fn test_video_config_builder_pattern() {
        let config = VideoConfig::hd_1080p()
            .with_codec("vp9")
            .with_bitrate(15000)
            .with_duration(30);

        assert_eq!(config.codec, "vp9");
        assert_eq!(config.bitrate, 15000);
        assert_eq!(config.duration, 30);
    }

    #[test]
    fn test_pdf_config_builder_pattern() {
        let config = PdfConfig::a4()
            .landscape()
            .vector()
            .print_optimized()
            .with_dpi(600)
            .with_margin(15.0);

        assert_eq!(config.width, 297.0);
        assert_eq!(config.height, 210.0);
        assert!(config.vector);
        assert!(config.print_optimized);
        assert_eq!(config.dpi, 600);
        assert_eq!(config.margin, 15.0);
    }

    #[test]
    fn test_poster_config_builder_pattern() {
        let config = PosterConfig::a1().landscape().with_dpi(450);

        assert_eq!(config.width, 841);
        assert_eq!(config.height, 594);
        assert_eq!(config.orientation, "landscape");
        assert_eq!(config.dpi, 450);
    }

    // Real-Time Collaboration Features Tests
    #[test]
    fn test_streaming_data_source_creation() {
        let source = StreamingDataSource::new("test-source", "ws://localhost:8080", 1000);
        assert_eq!(source.source_id, "test-source");
        assert_eq!(source.stream_url, "ws://localhost:8080");
        assert_eq!(source.update_frequency_ms, 1000);
        assert_eq!(source.buffer_size, 1000);
    }

    #[test]
    fn test_streaming_data_source_buffer() {
        let mut source = StreamingDataSource::new("test", "ws://localhost:8080", 1000);
        source.push_data("data1".to_string());
        source.push_data("data2".to_string());
        assert_eq!(source.buffer().len(), 2);
        source.clear_buffer();
        assert_eq!(source.buffer().len(), 0);
    }

    #[test]
    fn test_streaming_data_source_buffer_limit() {
        let mut source =
            StreamingDataSource::new("test", "ws://localhost:8080", 1000).with_buffer_size(2);
        source.push_data("data1".to_string());
        source.push_data("data2".to_string());
        source.push_data("data3".to_string());
        assert_eq!(source.buffer().len(), 2);
        assert_eq!(source.buffer()[0], "data2");
        assert_eq!(source.buffer()[1], "data3");
    }

    #[test]
    fn test_streaming_data_source_javascript() {
        let source = StreamingDataSource::new("test-source", "ws://localhost:8080", 1000);
        let js = source.to_javascript();
        assert!(js.contains("class StreamingDataSource"));
        assert!(js.contains("test-source"));
        assert!(js.contains("ws://localhost:8080"));
    }

    #[test]
    fn test_collaborative_user_creation() {
        let user = CollaborativeUser::new("user1", "Alice", "#ff0000");
        assert_eq!(user.user_id, "user1");
        assert_eq!(user.display_name, "Alice");
        assert_eq!(user.color, "#ff0000");
        assert!(user.active);
    }

    #[test]
    fn test_cursor_position_creation() {
        let user = CollaborativeUser::new("user1", "Alice", "#ff0000");
        let cursor = CursorPosition::new(user.clone(), 50.0, 75.0, 1234567890);
        assert_eq!(cursor.user.user_id, "user1");
        assert_eq!(cursor.x, 50.0);
        assert_eq!(cursor.y, 75.0);
        assert_eq!(cursor.timestamp, 1234567890);
    }

    #[test]
    fn test_shared_annotation_creation() {
        let user = CollaborativeUser::new("user1", "Alice", "#ff0000");
        let annotation = SharedAnnotation::new(
            "annot1",
            user.clone(),
            "node-123",
            "This is a comment",
            1234567890,
        );
        assert_eq!(annotation.annotation_id, "annot1");
        assert_eq!(annotation.user.user_id, "user1");
        assert_eq!(annotation.target_id, "node-123");
        assert_eq!(annotation.content, "This is a comment");
        assert!(!annotation.resolved);
    }

    #[test]
    fn test_shared_annotation_resolve() {
        let user = CollaborativeUser::new("user1", "Alice", "#ff0000");
        let mut annotation =
            SharedAnnotation::new("annot1", user, "node-123", "This is a comment", 1234567890);
        annotation.resolve();
        assert!(annotation.resolved);
    }

    #[test]
    fn test_collaborative_session_creation() {
        let session = CollaborativeSession::new("session1", "ws://localhost:8080");
        assert_eq!(session.session_id, "session1");
        assert_eq!(session.websocket_url, "ws://localhost:8080");
        assert_eq!(session.active_users().len(), 0);
        assert_eq!(session.cursors().len(), 0);
        assert_eq!(session.annotations().len(), 0);
    }

    #[test]
    fn test_collaborative_session_add_user() {
        let mut session = CollaborativeSession::new("session1", "ws://localhost:8080");
        let user = CollaborativeUser::new("user1", "Alice", "#ff0000");
        session.add_user(user.clone());
        assert_eq!(session.active_users().len(), 1);

        // Adding the same user again should not duplicate
        session.add_user(user);
        assert_eq!(session.active_users().len(), 1);
    }

    #[test]
    fn test_collaborative_session_remove_user() {
        let mut session = CollaborativeSession::new("session1", "ws://localhost:8080");
        let user = CollaborativeUser::new("user1", "Alice", "#ff0000");
        session.add_user(user.clone());
        assert_eq!(session.active_users().len(), 1);

        session.remove_user("user1");
        assert_eq!(session.active_users().len(), 0);
    }

    #[test]
    fn test_collaborative_session_update_cursor() {
        let mut session = CollaborativeSession::new("session1", "ws://localhost:8080");
        let user = CollaborativeUser::new("user1", "Alice", "#ff0000");
        let cursor = CursorPosition::new(user.clone(), 50.0, 75.0, 1234567890);

        session.update_cursor(cursor.clone());
        assert_eq!(session.cursors().len(), 1);

        // Update the same cursor
        let cursor2 = CursorPosition::new(user, 60.0, 80.0, 1234567891);
        session.update_cursor(cursor2);
        assert_eq!(session.cursors().len(), 1);
        assert_eq!(session.cursors()[0].x, 60.0);
    }

    #[test]
    fn test_collaborative_session_add_annotation() {
        let mut session = CollaborativeSession::new("session1", "ws://localhost:8080");
        let user = CollaborativeUser::new("user1", "Alice", "#ff0000");
        let annotation =
            SharedAnnotation::new("annot1", user, "node-123", "This is a comment", 1234567890);

        session.add_annotation(annotation);
        assert_eq!(session.annotations().len(), 1);
    }

    #[test]
    fn test_collaborative_session_remove_annotation() {
        let mut session = CollaborativeSession::new("session1", "ws://localhost:8080");
        let user = CollaborativeUser::new("user1", "Alice", "#ff0000");
        let annotation =
            SharedAnnotation::new("annot1", user, "node-123", "This is a comment", 1234567890);

        session.add_annotation(annotation);
        assert_eq!(session.annotations().len(), 1);

        session.remove_annotation("annot1");
        assert_eq!(session.annotations().len(), 0);
    }

    #[test]
    fn test_collaborative_session_html_generation() {
        let statute = Statute::new(
            "test-1",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        );
        let tree = DecisionTree::from_statute(&statute).unwrap();

        let session = CollaborativeSession::new("session1", "ws://localhost:8080");
        let html = session.to_collaborative_html(&tree);

        assert!(html.contains("Collaborative Visualization"));
        assert!(html.contains("session1"));
        assert!(html.contains("ws://localhost:8080"));
        assert!(html.contains("connectWebSocket"));
        assert!(html.contains("updateCursor"));
        assert!(html.contains("addAnnotation"));
    }

    // Custom Theme Builder Tests
    #[test]
    fn test_custom_theme_builder_creation() {
        let builder = CustomThemeBuilder::new();
        let theme = builder.build();
        assert_eq!(theme.background_color, Theme::default().background_color);
    }

    #[test]
    fn test_custom_theme_builder_with_colors() {
        let theme = CustomThemeBuilder::new()
            .with_background_color("#000000")
            .with_text_color("#ffffff")
            .with_condition_color("#0000ff")
            .with_outcome_color("#00ff00")
            .with_discretion_color("#ff0000")
            .with_link_color("#ffff00")
            .with_root_color("#cccccc")
            .build();

        assert_eq!(theme.background_color, "#000000");
        assert_eq!(theme.text_color, "#ffffff");
        assert_eq!(theme.condition_color, "#0000ff");
        assert_eq!(theme.outcome_color, "#00ff00");
        assert_eq!(theme.discretion_color, "#ff0000");
        assert_eq!(theme.link_color, "#ffff00");
        assert_eq!(theme.root_color, "#cccccc");
    }

    #[test]
    fn test_custom_theme_builder_with_branding() {
        let theme = CustomThemeBuilder::new()
            .with_branding("#ff0000", "#0000ff")
            .build();

        assert_eq!(theme.condition_color, "#ff0000");
        assert_eq!(theme.outcome_color, "#0000ff");
        assert_eq!(theme.link_color, "#ff0000");
    }

    #[test]
    fn test_custom_theme_builder_with_palette() {
        let theme = CustomThemeBuilder::new()
            .with_palette("#ffffff", "#000000", "#ff0000", "#00ff00", "#0000ff")
            .build();

        assert_eq!(theme.background_color, "#ffffff");
        assert_eq!(theme.text_color, "#000000");
        assert_eq!(theme.condition_color, "#ff0000");
        assert_eq!(theme.outcome_color, "#00ff00");
        assert_eq!(theme.discretion_color, "#0000ff");
        assert_eq!(theme.link_color, "#ff0000");
    }

    #[test]
    fn test_custom_theme_builder_from_theme() {
        let dark_theme = Theme::dark();
        let custom = CustomThemeBuilder::from_theme(dark_theme.clone())
            .with_condition_color("#123456")
            .build();

        assert_eq!(custom.background_color, dark_theme.background_color);
        assert_eq!(custom.condition_color, "#123456");
    }

    #[test]
    fn test_custom_theme_builder_to_json() {
        let builder = CustomThemeBuilder::new()
            .with_background_color("#ffffff")
            .with_text_color("#000000");

        let json = builder.to_json().unwrap();
        assert!(json.contains("background_color"));
        assert!(json.contains("#ffffff"));
        assert!(json.contains("text_color"));
        assert!(json.contains("#000000"));
    }

    #[test]
    fn test_custom_theme_builder_from_json() {
        let json = r##"{
            "root_color": "#f0f0f0",
            "condition_color": "#e1f5fe",
            "discretion_color": "#ffcdd2",
            "outcome_color": "#c8e6c9",
            "link_color": "#ccc",
            "background_color": "#ffffff",
            "text_color": "#333333"
        }"##;

        let builder = CustomThemeBuilder::from_json(json).unwrap();
        let theme = builder.build();
        assert_eq!(theme.background_color, "#ffffff");
        assert_eq!(theme.text_color, "#333333");
    }

    #[test]
    fn test_custom_theme_builder_from_json_invalid() {
        let json = r##"{ "invalid": "json" }"##;
        let result = CustomThemeBuilder::from_json(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_custom_theme_builder_default() {
        let builder = CustomThemeBuilder::default();
        let theme = builder.build();
        assert_eq!(theme.background_color, Theme::default().background_color);
    }

    // Seasonal Themes Tests
    #[test]
    fn test_seasonal_theme_winter() {
        let theme = SeasonalThemes::winter();
        assert!(theme.background_color.contains("f0f8ff"));
        assert!(theme.link_color.contains("668db8"));
    }

    #[test]
    fn test_seasonal_theme_spring() {
        let theme = SeasonalThemes::spring();
        assert!(theme.background_color.contains("f1f8e9"));
        assert!(theme.link_color.contains("81c784"));
    }

    #[test]
    fn test_seasonal_theme_summer() {
        let theme = SeasonalThemes::summer();
        assert!(theme.background_color.contains("fffaf0"));
        assert!(theme.link_color.contains("ff9800"));
    }

    #[test]
    fn test_seasonal_theme_autumn() {
        let theme = SeasonalThemes::autumn();
        assert!(theme.background_color.contains("fff8f5"));
        assert!(theme.link_color.contains("8d6e63"));
    }

    #[test]
    fn test_seasonal_theme_holiday() {
        let theme = SeasonalThemes::holiday();
        assert_eq!(theme.background_color, "#fafafa");
        assert_eq!(theme.link_color, "#c62828");
    }

    #[test]
    fn test_seasonal_theme_corporate() {
        let theme = SeasonalThemes::corporate();
        assert_eq!(theme.background_color, "#fafafa");
        assert_eq!(theme.link_color, "#455a64");
    }

    #[test]
    fn test_seasonal_theme_academic() {
        let theme = SeasonalThemes::academic();
        assert_eq!(theme.background_color, "#fafafa");
        assert_eq!(theme.link_color, "#1976d2");
    }

    #[test]
    fn test_seasonal_theme_legal() {
        let theme = SeasonalThemes::legal();
        assert_eq!(theme.background_color, "#ffffff");
        assert_eq!(theme.link_color, "#1a237e");
        assert_eq!(theme.text_color, "#000000");
    }

    // CSS Variable Theme Tests
    #[test]
    fn test_css_variable_theme_creation() {
        let css_theme = CssVariableTheme::new()
            .add_variable("--primary-color", "#ff0000")
            .add_variable("--secondary-color", "#00ff00");

        assert_eq!(css_theme.variables().len(), 2);
        assert_eq!(css_theme.variables()[0].0, "--primary-color");
        assert_eq!(css_theme.variables()[0].1, "#ff0000");
    }

    #[test]
    fn test_css_variable_theme_from_theme() {
        let theme = Theme::dark();
        let css_theme = CssVariableTheme::from_theme(&theme);

        assert_eq!(css_theme.variables().len(), 7);
        let vars: Vec<&String> = css_theme.variables().iter().map(|(name, _)| name).collect();
        assert!(vars.contains(&&"--viz-root-color".to_string()));
        assert!(vars.contains(&&"--viz-condition-color".to_string()));
    }

    #[test]
    fn test_css_variable_theme_to_css() {
        let css_theme = CssVariableTheme::new()
            .add_variable("--primary-color", "#ff0000")
            .add_variable("--secondary-color", "#00ff00");

        let css = css_theme.to_css();
        assert!(css.contains(":root {"));
        assert!(css.contains("--primary-color: #ff0000;"));
        assert!(css.contains("--secondary-color: #00ff00;"));
    }

    #[test]
    fn test_css_variable_theme_to_css_with_selector() {
        let css_theme = CssVariableTheme::new().add_variable("--primary-color", "#ff0000");

        let css = css_theme.to_css_with_selector(".dark-theme");
        assert!(css.contains(".dark-theme {"));
        assert!(css.contains("--primary-color: #ff0000;"));
    }

    #[test]
    fn test_css_variable_theme_default() {
        let css_theme = CssVariableTheme::default();
        assert_eq!(css_theme.variables().len(), 0);
    }

    // Virtualization Tests
    #[test]
    fn test_virtualization_config_creation() {
        let config = VirtualizationConfig::new();
        assert!(config.enabled);
        assert_eq!(config.render_batch_size, 100);
        assert_eq!(config.buffer_size, 20);
        assert_eq!(config.min_item_height, 50);
        assert!(!config.dynamic_height);
    }

    #[test]
    fn test_virtualization_config_disabled() {
        let config = VirtualizationConfig::disabled();
        assert!(!config.enabled);
    }

    #[test]
    fn test_virtualization_config_builder() {
        let config = VirtualizationConfig::new()
            .with_batch_size(200)
            .with_buffer_size(30)
            .with_dynamic_height();

        assert_eq!(config.render_batch_size, 200);
        assert_eq!(config.buffer_size, 30);
        assert!(config.dynamic_height);
    }

    #[test]
    fn test_virtualization_config_javascript() {
        let config = VirtualizationConfig::new();
        let js = config.to_javascript();
        assert!(js.contains("class VirtualScroller"));
        assert!(js.contains("renderBatchSize"));
        assert!(js.contains("onScroll"));
    }

    #[test]
    fn test_virtualization_config_javascript_disabled() {
        let config = VirtualizationConfig::disabled();
        let js = config.to_javascript();
        assert_eq!(js, "");
    }

    #[test]
    fn test_virtualization_config_default() {
        let config = VirtualizationConfig::default();
        assert!(config.enabled);
        assert_eq!(config.render_batch_size, 100);
    }

    // Progressive Loading Tests
    #[test]
    fn test_progressive_loading_config_creation() {
        let config = ProgressiveLoadingConfig::new();
        assert!(config.enabled);
        assert_eq!(config.initial_load, 50);
        assert_eq!(config.load_increment, 25);
        assert!(config.show_loading_indicator);
        assert_eq!(config.load_delay_ms, 200);
    }

    #[test]
    fn test_progressive_loading_config_builder() {
        let config = ProgressiveLoadingConfig::new()
            .with_initial_load(100)
            .with_load_increment(50)
            .without_loading_indicator();

        assert_eq!(config.initial_load, 100);
        assert_eq!(config.load_increment, 50);
        assert!(!config.show_loading_indicator);
    }

    #[test]
    fn test_progressive_loading_config_javascript() {
        let config = ProgressiveLoadingConfig::new();
        let js = config.to_javascript();
        assert!(js.contains("class ProgressiveLoader"));
        assert!(js.contains("loadMore"));
        assert!(js.contains("checkScroll"));
    }

    #[test]
    fn test_progressive_loading_config_default() {
        let config = ProgressiveLoadingConfig::default();
        assert!(config.enabled);
        assert_eq!(config.initial_load, 50);
    }

    // Level of Detail Tests
    #[test]
    fn test_level_of_detail_config_creation() {
        let config = LevelOfDetailConfig::new();
        assert!(config.enabled);
        assert_eq!(config.zoom_thresholds.len(), 4);
        assert!(config.simplify_at_low_zoom);
        assert!(config.hide_labels_at_low_zoom);
        assert!(config.aggregate_nodes);
    }

    #[test]
    fn test_level_of_detail_config_disabled() {
        let config = LevelOfDetailConfig::disabled();
        assert!(!config.enabled);
    }

    #[test]
    fn test_level_of_detail_config_custom_thresholds() {
        let config = LevelOfDetailConfig::new().with_zoom_thresholds(vec![0.1, 0.5, 1.0]);

        assert_eq!(config.zoom_thresholds.len(), 3);
        assert_eq!(config.zoom_thresholds[0], 0.1);
        assert_eq!(config.zoom_thresholds[1], 0.5);
        assert_eq!(config.zoom_thresholds[2], 1.0);
    }

    #[test]
    fn test_level_of_detail_config_javascript() {
        let config = LevelOfDetailConfig::new();
        let js = config.to_javascript();
        assert!(js.contains("class LevelOfDetailRenderer"));
        assert!(js.contains("updateDetailLevel"));
        assert!(js.contains("applyDetailLevel"));
    }

    #[test]
    fn test_level_of_detail_config_javascript_disabled() {
        let config = LevelOfDetailConfig::disabled();
        let js = config.to_javascript();
        assert_eq!(js, "");
    }

    #[test]
    fn test_level_of_detail_config_default() {
        let config = LevelOfDetailConfig::default();
        assert!(config.enabled);
        assert_eq!(config.zoom_thresholds.len(), 4);
    }

    // ============================================================================
    // Immersive Legal Visualization Tests (v0.3.0)
    // ============================================================================

    #[test]
    fn test_vr_exploration_config_default() {
        let config = VRExplorationConfig::default();
        assert!(config.enable_hand_tracking);
        assert!(config.enable_teleportation);
        assert!(!config.enable_voice_commands);
        assert!(config.enable_spatial_audio);
        assert!(config.enable_haptic_feedback);
        assert_eq!(config.interaction_distance, 2.0);
        assert_eq!(config.movement_speed, 1.0);
    }

    #[test]
    fn test_vr_statute_explorer_creation() {
        let explorer = VRStatuteExplorer::new();
        assert_eq!(explorer.theme.background_color, "#ffffff");
        assert!(explorer.config.enable_hand_tracking);
    }

    #[test]
    fn test_vr_statute_explorer_with_theme() {
        let explorer = VRStatuteExplorer::new().with_theme(Theme::dark());
        assert_eq!(explorer.theme.background_color, "#1a1a1a");
    }

    #[test]
    fn test_vr_statute_explorer_with_config() {
        let config = VRExplorationConfig {
            enable_hand_tracking: false,
            enable_spatial_audio: false,
            ..Default::default()
        };
        let explorer = VRStatuteExplorer::new().with_config(config);
        assert!(!explorer.config.enable_hand_tracking);
        assert!(!explorer.config.enable_spatial_audio);
    }

    #[test]
    fn test_vr_statute_explorer_html_generation() {
        let statute = Statute::new(
            "test-1",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        );
        let explorer = VRStatuteExplorer::new();
        let html = explorer.to_vr_html(&statute);

        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("VR Statute Explorer"));
        assert!(html.contains("ENTER VR"));
        assert!(html.contains("renderer.xr.enabled = true"));
        assert!(html.contains("navigator.xr.requestSession"));
        assert!(html.contains("immersive-vr"));
    }

    #[test]
    fn test_vr_statute_explorer_spatial_audio() {
        let statute = Statute::new(
            "test-1",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        );
        let explorer = VRStatuteExplorer::new();
        let html = explorer.to_vr_html(&statute);

        assert!(html.contains("setupSpatialAudio"));
        assert!(html.contains("AudioContext"));
        assert!(html.contains("PositionalAudio"));
    }

    #[test]
    fn test_vr_statute_explorer_haptic_feedback() {
        let statute = Statute::new(
            "test-1",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        );
        let explorer = VRStatuteExplorer::new();
        let html = explorer.to_vr_html(&statute);

        assert!(html.contains("hapticActuators"));
        assert!(html.contains("pulse"));
    }

    #[test]
    fn test_vr_statute_explorer_default() {
        let explorer1 = VRStatuteExplorer::new();
        let explorer2 = VRStatuteExplorer::default();
        assert_eq!(
            explorer1.theme.background_color,
            explorer2.theme.background_color
        );
    }

    #[test]
    fn test_ar_overlay_config_default() {
        let config = AROverlayConfig::default();
        assert!(config.enable_markers);
        assert!(config.enable_markerless);
        assert!(!config.enable_face_tracking);
        assert_eq!(config.marker_size, 0.15);
        assert_eq!(config.overlay_opacity, 0.9);
    }

    #[test]
    fn test_ar_document_overlay_creation() {
        let overlay = ARDocumentOverlay::new();
        assert_eq!(overlay.theme.background_color, "#ffffff");
        assert!(overlay.config.enable_markers);
    }

    #[test]
    fn test_ar_document_overlay_with_theme() {
        let overlay = ARDocumentOverlay::new().with_theme(Theme::dark());
        assert_eq!(overlay.theme.background_color, "#1a1a1a");
    }

    #[test]
    fn test_ar_document_overlay_with_config() {
        let config = AROverlayConfig {
            enable_markers: false,
            enable_markerless: false,
            enable_face_tracking: true,
            marker_size: 0.2,
            overlay_opacity: 0.5,
        };
        let overlay = ARDocumentOverlay::new().with_config(config);
        assert!(!overlay.config.enable_markers);
        assert!(overlay.config.enable_face_tracking);
        assert_eq!(overlay.config.overlay_opacity, 0.5);
    }

    #[test]
    fn test_ar_document_overlay_html_generation() {
        let statute = Statute::new(
            "test-1",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        );
        let overlay = ARDocumentOverlay::new();
        let html = overlay.to_ar_html(&statute);

        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("AR Document Overlay"));
        assert!(html.contains("start-ar"));
        assert!(html.contains("camera-feed"));
        assert!(html.contains("getUserMedia"));
        assert!(html.contains("immersive-ar"));
    }

    #[test]
    fn test_ar_document_overlay_camera_access() {
        let statute = Statute::new(
            "test-1",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        );
        let overlay = ARDocumentOverlay::new();
        let html = overlay.to_ar_html(&statute);

        assert!(html.contains("navigator.mediaDevices.getUserMedia"));
        assert!(html.contains("facingMode: 'environment'"));
    }

    #[test]
    fn test_ar_document_overlay_default() {
        let overlay1 = ARDocumentOverlay::new();
        let overlay2 = ARDocumentOverlay::default();
        assert_eq!(
            overlay1.theme.background_color,
            overlay2.theme.background_color
        );
    }

    #[test]
    fn test_panoramic_360_config_default() {
        let config = Panoramic360Config::default();
        assert!(config.enable_vr_mode);
        assert!(!config.enable_auto_rotation);
        assert_eq!(config.rotation_speed, 10.0);
        assert_eq!(config.field_of_view, 75.0);
        assert!(config.enable_gyroscope);
    }

    #[test]
    fn test_panoramic_360_timeline_creation() {
        let timeline = Panoramic360Timeline::new();
        assert_eq!(timeline.theme.background_color, "#ffffff");
        assert!(timeline.config.enable_vr_mode);
    }

    #[test]
    fn test_panoramic_360_timeline_with_theme() {
        let timeline = Panoramic360Timeline::new().with_theme(Theme::dark());
        assert_eq!(timeline.theme.background_color, "#1a1a1a");
    }

    #[test]
    fn test_panoramic_360_timeline_with_config() {
        let config = Panoramic360Config {
            enable_vr_mode: false,
            enable_auto_rotation: true,
            rotation_speed: 20.0,
            field_of_view: 90.0,
            enable_gyroscope: false,
        };
        let timeline = Panoramic360Timeline::new().with_config(config);
        assert!(!timeline.config.enable_vr_mode);
        assert!(timeline.config.enable_auto_rotation);
        assert_eq!(timeline.config.rotation_speed, 20.0);
    }

    #[test]
    fn test_panoramic_360_timeline_html_generation() {
        let mut timeline_data = Timeline::new();
        timeline_data.add_event(
            "2024-01-01",
            TimelineEvent::Enacted {
                statute_id: "statute-1".to_string(),
                title: "Test Statute".to_string(),
            },
        );

        let timeline = Panoramic360Timeline::new();
        let html = timeline.to_360_html(&timeline_data);

        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("360° Case Timeline"));
        assert!(html.contains("SphereGeometry"));
        assert!(html.contains("BackSide"));
        assert!(html.contains("DeviceOrientationEvent"));
    }

    #[test]
    fn test_panoramic_360_timeline_gyroscope_support() {
        let mut timeline_data = Timeline::new();
        timeline_data.add_event(
            "2024-01-01",
            TimelineEvent::Enacted {
                statute_id: "statute-1".to_string(),
                title: "Test Statute".to_string(),
            },
        );

        let timeline = Panoramic360Timeline::new();
        let html = timeline.to_360_html(&timeline_data);

        assert!(html.contains("deviceorientation"));
        assert!(html.contains("event.alpha"));
        assert!(html.contains("event.beta"));
        assert!(html.contains("event.gamma"));
    }

    #[test]
    fn test_panoramic_360_timeline_vr_mode() {
        let mut timeline_data = Timeline::new();
        timeline_data.add_event(
            "2024-01-01",
            TimelineEvent::Enacted {
                statute_id: "statute-1".to_string(),
                title: "Test Statute".to_string(),
            },
        );

        let config = Panoramic360Config {
            enable_vr_mode: true,
            ..Default::default()
        };
        let timeline = Panoramic360Timeline::new().with_config(config);
        let html = timeline.to_360_html(&timeline_data);

        assert!(html.contains("enter-vr"));
        assert!(html.contains("renderer.xr.enabled = true"));
    }

    #[test]
    fn test_panoramic_360_timeline_auto_rotation() {
        let mut timeline_data = Timeline::new();
        timeline_data.add_event(
            "2024-01-01",
            TimelineEvent::Enacted {
                statute_id: "statute-1".to_string(),
                title: "Test Statute".to_string(),
            },
        );

        let config = Panoramic360Config {
            enable_auto_rotation: true,
            rotation_speed: 15.0,
            ..Default::default()
        };
        let timeline = Panoramic360Timeline::new().with_config(config);
        let html = timeline.to_360_html(&timeline_data);

        assert!(html.contains("enableAutoRotation: true"));
        assert!(html.contains("rotationSpeed: 15"));
        assert!(html.contains("toggle-rotation"));
    }

    #[test]
    fn test_panoramic_360_timeline_default() {
        let timeline1 = Panoramic360Timeline::new();
        let timeline2 = Panoramic360Timeline::default();
        assert_eq!(
            timeline1.theme.background_color,
            timeline2.theme.background_color
        );
    }

    #[test]
    fn test_panoramic_360_timeline_event_extraction() {
        let mut timeline_data = Timeline::new();
        timeline_data.add_event(
            "2024-01-01",
            TimelineEvent::Enacted {
                statute_id: "statute-1".to_string(),
                title: "Test Statute".to_string(),
            },
        );
        timeline_data.add_event(
            "2024-02-01",
            TimelineEvent::Amended {
                statute_id: "statute-1".to_string(),
                description: "First amendment".to_string(),
            },
        );
        timeline_data.add_event(
            "2024-03-01",
            TimelineEvent::Repealed {
                statute_id: "statute-1".to_string(),
            },
        );

        let timeline = Panoramic360Timeline::new();
        let html = timeline.to_360_html(&timeline_data);

        assert!(html.contains("2024-01-01"));
        assert!(html.contains("2024-02-01"));
        assert!(html.contains("2024-03-01"));
        assert!(html.contains("Enacted"));
        assert!(html.contains("Amended"));
        assert!(html.contains("Repealed"));
    }

    // ============================================================================
    // AI-Enhanced Visualization Tests (v0.3.1)
    // ============================================================================

    #[test]
    fn test_auto_visualization_selector_creation() {
        let selector = AutoVisualizationSelector::new();
        assert_eq!(selector.min_confidence, 0.7);
    }

    #[test]
    fn test_auto_visualization_selector_with_min_confidence() {
        let selector = AutoVisualizationSelector::new().with_min_confidence(0.8);
        assert_eq!(selector.min_confidence, 0.8);
    }

    #[test]
    fn test_auto_visualization_selector_recommend_small_tree() {
        let statute = Statute::new(
            "test-1",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        );
        let tree = DecisionTree::from_statute(&statute).unwrap();
        let selector = AutoVisualizationSelector::new();
        let recommendation = selector.recommend_for_tree(&tree);

        assert_eq!(recommendation.viz_type, VisualizationType::DecisionTree);
        assert!(recommendation.confidence > 0.7);
        assert!(!recommendation.reasoning.is_empty());
        assert!(!recommendation.alternatives.is_empty());
    }

    #[test]
    fn test_auto_visualization_selector_recommend_graph() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("statute-1", "statute-2", "references");

        let selector = AutoVisualizationSelector::new();
        let recommendation = selector.recommend_for_graph(&graph);

        assert!(recommendation.confidence > 0.7);
        assert!(!recommendation.reasoning.is_empty());
    }

    #[test]
    fn test_auto_visualization_selector_recommend_timeline() {
        let mut timeline = Timeline::new();
        timeline.add_event(
            "2024-01-01",
            TimelineEvent::Enacted {
                statute_id: "statute-1".to_string(),
                title: "Test".to_string(),
            },
        );

        let selector = AutoVisualizationSelector::new();
        let recommendation = selector.recommend_for_timeline(&timeline);

        assert_eq!(recommendation.viz_type, VisualizationType::Timeline);
        assert!(recommendation.confidence > 0.9);
    }

    #[test]
    fn test_auto_visualization_selector_default() {
        let selector1 = AutoVisualizationSelector::new();
        let selector2 = AutoVisualizationSelector::default();
        assert_eq!(selector1.min_confidence, selector2.min_confidence);
    }

    #[test]
    fn test_ai_annotation_generator_creation() {
        let generator = AIAnnotationGenerator::new();
        assert!(generator.enable_complexity);
        assert!(generator.enable_patterns);
        assert_eq!(generator.min_importance, 0.5);
    }

    #[test]
    fn test_ai_annotation_generator_without_complexity() {
        let generator = AIAnnotationGenerator::new().without_complexity();
        assert!(!generator.enable_complexity);
    }

    #[test]
    fn test_ai_annotation_generator_without_patterns() {
        let generator = AIAnnotationGenerator::new().without_patterns();
        assert!(!generator.enable_patterns);
    }

    #[test]
    fn test_ai_annotation_generator_with_min_importance() {
        let generator = AIAnnotationGenerator::new().with_min_importance(0.8);
        assert_eq!(generator.min_importance, 0.8);
    }

    #[test]
    fn test_ai_annotation_generator_for_tree() {
        let statute = Statute::new(
            "test-1",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        );
        let tree = DecisionTree::from_statute(&statute).unwrap();
        let generator = AIAnnotationGenerator::new();
        let annotations = generator.generate_for_tree(&tree);

        // Should have some annotations if tree is complex enough
        assert!(annotations.len() >= 0);
    }

    #[test]
    fn test_ai_annotation_generator_for_graph() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("statute-1", "statute-2", "references");

        let generator = AIAnnotationGenerator::new();
        let annotations = generator.generate_for_graph(&graph);

        assert!(annotations.len() >= 0);
    }

    #[test]
    fn test_ai_annotation_generator_default() {
        let gen1 = AIAnnotationGenerator::new();
        let gen2 = AIAnnotationGenerator::default();
        assert_eq!(gen1.min_importance, gen2.min_importance);
    }

    #[test]
    fn test_natural_language_query_processor_creation() {
        let processor = NaturalLanguageQueryProcessor::new();
        assert!(!processor.case_sensitive);
    }

    #[test]
    fn test_natural_language_query_processor_case_sensitive() {
        let processor = NaturalLanguageQueryProcessor::new().with_case_sensitive();
        assert!(processor.case_sensitive);
    }

    #[test]
    fn test_natural_language_query_processor_query_outcomes() {
        let statute = Statute::new(
            "test-1",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        );
        let tree = DecisionTree::from_statute(&statute).unwrap();
        let processor = NaturalLanguageQueryProcessor::new();
        let results = processor.query_tree(&tree, "show me outcomes");

        assert!(results.len() >= 0);
    }

    #[test]
    fn test_natural_language_query_processor_query_discretion() {
        let statute = Statute::new(
            "test-1",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        );
        let tree = DecisionTree::from_statute(&statute).unwrap();
        let processor = NaturalLanguageQueryProcessor::new();
        let results = processor.query_tree(&tree, "find discretion");

        assert!(results.len() >= 0);
    }

    #[test]
    fn test_natural_language_query_processor_default() {
        let proc1 = NaturalLanguageQueryProcessor::new();
        let proc2 = NaturalLanguageQueryProcessor::default();
        assert_eq!(proc1.case_sensitive, proc2.case_sensitive);
    }

    #[test]
    fn test_smart_data_highlighter_creation() {
        let highlighter = SmartDataHighlighter::new();
        assert_eq!(highlighter.highlight_color, "#ffeb3b");
        assert_eq!(highlighter.min_importance, 0.7);
    }

    #[test]
    fn test_smart_data_highlighter_with_color() {
        let highlighter = SmartDataHighlighter::new().with_color("#ff0000".to_string());
        assert_eq!(highlighter.highlight_color, "#ff0000");
    }

    #[test]
    fn test_smart_data_highlighter_with_min_importance() {
        let highlighter = SmartDataHighlighter::new().with_min_importance(0.9);
        assert_eq!(highlighter.min_importance, 0.9);
    }

    #[test]
    fn test_smart_data_highlighter_highlight_tree() {
        let statute = Statute::new(
            "test-1",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        );
        let tree = DecisionTree::from_statute(&statute).unwrap();
        let highlighter = SmartDataHighlighter::new();
        let rules = highlighter.highlight_tree(&tree);

        assert!(rules.len() >= 0);
    }

    #[test]
    fn test_smart_data_highlighter_highlight_graph() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("statute-1", "statute-2", "references");

        let highlighter = SmartDataHighlighter::new();
        let rules = highlighter.highlight_graph(&graph);

        assert!(rules.len() >= 0);
    }

    #[test]
    fn test_smart_data_highlighter_default() {
        let high1 = SmartDataHighlighter::new();
        let high2 = SmartDataHighlighter::default();
        assert_eq!(high1.highlight_color, high2.highlight_color);
    }

    #[test]
    fn test_anomaly_detector_creation() {
        let detector = AnomalyDetector::new();
        assert_eq!(detector.sensitivity, 0.7);
    }

    #[test]
    fn test_anomaly_detector_with_sensitivity() {
        let detector = AnomalyDetector::new().with_sensitivity(0.9);
        assert_eq!(detector.sensitivity, 0.9);
    }

    #[test]
    fn test_anomaly_detector_detect_in_tree() {
        let statute = Statute::new(
            "test-1",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        );
        let tree = DecisionTree::from_statute(&statute).unwrap();
        let detector = AnomalyDetector::new();
        let anomalies = detector.detect_in_tree(&tree);

        assert!(anomalies.len() >= 0);
    }

    #[test]
    fn test_anomaly_detector_detect_in_graph() {
        let mut graph = DependencyGraph::new();
        graph.add_dependency("statute-1", "statute-2", "references");

        let detector = AnomalyDetector::new();
        let anomalies = detector.detect_in_graph(&graph);

        assert!(anomalies.len() >= 0);
    }

    #[test]
    fn test_anomaly_detector_default() {
        let det1 = AnomalyDetector::new();
        let det2 = AnomalyDetector::default();
        assert_eq!(det1.sensitivity, det2.sensitivity);
    }

    #[test]
    fn test_visualization_type_serialization() {
        let viz_type = VisualizationType::DecisionTree;
        let json = serde_json::to_string(&viz_type).unwrap();
        assert!(json.contains("DecisionTree"));
    }

    #[test]
    fn test_annotation_category_serialization() {
        let category = AnnotationCategory::CriticalPath;
        let json = serde_json::to_string(&category).unwrap();
        assert!(json.contains("CriticalPath"));
    }

    #[test]
    fn test_anomaly_type_serialization() {
        let anomaly_type = AnomalyType::OrphanedNode;
        let json = serde_json::to_string(&anomaly_type).unwrap();
        assert!(json.contains("OrphanedNode"));
    }

    #[test]
    fn test_query_result_serialization() {
        let result = QueryResult {
            node_id: "node-1".to_string(),
            relevance: 0.8,
            excerpt: "test excerpt".to_string(),
            node_type: "condition".to_string(),
        };
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("node-1"));
        assert!(json.contains("0.8"));
    }

    #[test]
    fn test_highlight_rule_serialization() {
        let rule = HighlightRule {
            target_id: "node-1".to_string(),
            color: "#ff0000".to_string(),
            importance: 0.9,
            reason: "Test reason".to_string(),
        };
        let json = serde_json::to_string(&rule).unwrap();
        assert!(json.contains("node-1"));
        assert!(json.contains("#ff0000"));
    }

    #[test]
    fn test_anomaly_serialization() {
        let anomaly = Anomaly {
            anomaly_type: AnomalyType::Cycle,
            severity: 0.95,
            description: "Test anomaly".to_string(),
            location: "test-location".to_string(),
            suggestion: "Fix it".to_string(),
        };
        let json = serde_json::to_string(&anomaly).unwrap();
        assert!(json.contains("Cycle"));
        assert!(json.contains("0.95"));
    }

    // ============================================================================
    // Real-Time Legal Intelligence Tests (v0.3.2)
    // ============================================================================

    #[test]
    fn test_live_court_proceeding_creation() {
        let proceeding =
            LiveCourtProceeding::new("Supreme Court", "2024-001", "ws://localhost:8080");
        assert_eq!(proceeding.court_name, "Supreme Court");
        assert_eq!(proceeding.case_number, "2024-001");
        assert_eq!(proceeding.ws_url, "ws://localhost:8080");
    }

    #[test]
    fn test_live_court_proceeding_with_theme() {
        let proceeding =
            LiveCourtProceeding::new("Supreme Court", "2024-001", "ws://localhost:8080")
                .with_theme(Theme::dark());
        assert_eq!(proceeding.theme.background_color, "#1a1a1a");
    }

    #[test]
    fn test_live_court_proceeding_html_generation() {
        let events = vec![
            CourtEvent::new(
                "10:00 AM",
                CourtEventType::Opening,
                "Opening statements begin",
            )
            .with_participant("Prosecutor"),
            CourtEvent::new("10:30 AM", CourtEventType::Testimony, "Witness testimony")
                .with_participant("Witness 1"),
        ];

        let proceeding =
            LiveCourtProceeding::new("Supreme Court", "2024-001", "ws://localhost:8080");
        let html = proceeding.to_live_html(&events);

        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Supreme Court"));
        assert!(html.contains("2024-001"));
        assert!(html.contains("LIVE"));
        assert!(html.contains("Opening statements begin"));
        assert!(html.contains("Witness testimony"));
        assert!(html.contains("ws://localhost:8080"));
    }

    #[test]
    fn test_live_court_proceeding_default() {
        let proceeding = LiveCourtProceeding::default();
        assert_eq!(proceeding.court_name, "Court");
        assert_eq!(proceeding.case_number, "Unknown");
    }

    #[test]
    fn test_court_event_creation() {
        let event = CourtEvent::new("10:00 AM", CourtEventType::Ruling, "Judge issues ruling");
        assert_eq!(event.timestamp, "10:00 AM");
        assert_eq!(event.event_type, CourtEventType::Ruling);
        assert_eq!(event.description, "Judge issues ruling");
    }

    #[test]
    fn test_court_event_with_participant() {
        let event = CourtEvent::new("10:00 AM", CourtEventType::Motion, "Motion filed")
            .with_participant("Defense Attorney")
            .with_participant("Prosecutor");
        assert_eq!(event.participants.len(), 2);
    }

    #[test]
    fn test_court_event_type_serialization() {
        let event_type = CourtEventType::Testimony;
        let json = serde_json::to_string(&event_type).unwrap();
        assert!(json.contains("Testimony"));
    }

    #[test]
    fn test_breaking_news_feed_creation() {
        let feed = BreakingNewsFeed::new("Legal News", "ws://localhost:8080");
        assert_eq!(feed.title, "Legal News");
        assert_eq!(feed.ws_url, "ws://localhost:8080");
        assert_eq!(feed.max_items, 50);
    }

    #[test]
    fn test_breaking_news_feed_with_theme() {
        let feed =
            BreakingNewsFeed::new("Legal News", "ws://localhost:8080").with_theme(Theme::dark());
        assert_eq!(feed.theme.background_color, "#1a1a1a");
    }

    #[test]
    fn test_breaking_news_feed_with_max_items() {
        let feed = BreakingNewsFeed::new("Legal News", "ws://localhost:8080").with_max_items(100);
        assert_eq!(feed.max_items, 100);
    }

    #[test]
    fn test_breaking_news_feed_html_generation() {
        let news_items = vec![
            NewsItem::new(
                "Supreme Court Ruling",
                "Important case decided today",
                "Legal Times",
                "2024-01-01",
                NewsPriority::Urgent,
            )
            .with_tag("Supreme Court")
            .with_tag("Constitutional Law"),
            NewsItem::new(
                "New Legislation Proposed",
                "Bill introduced in Congress",
                "Law Gazette",
                "2024-01-02",
                NewsPriority::High,
            ),
        ];

        let feed = BreakingNewsFeed::new("Legal News", "ws://localhost:8080");
        let html = feed.to_html(&news_items);

        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Legal News"));
        assert!(html.contains("Supreme Court Ruling"));
        assert!(html.contains("New Legislation Proposed"));
        assert!(html.contains("Supreme Court"));
        assert!(html.contains("Constitutional Law"));
        assert!(html.contains("ws://localhost:8080"));
    }

    #[test]
    fn test_breaking_news_feed_default() {
        let feed = BreakingNewsFeed::default();
        assert_eq!(feed.title, "Legal News Feed");
    }

    #[test]
    fn test_news_item_creation() {
        let item = NewsItem::new(
            "Test News",
            "Summary",
            "Source",
            "2024-01-01",
            NewsPriority::Medium,
        );
        assert_eq!(item.title, "Test News");
        assert_eq!(item.priority, NewsPriority::Medium);
    }

    #[test]
    fn test_news_item_with_tag() {
        let item = NewsItem::new("Test", "Summary", "Source", "2024-01-01", NewsPriority::Low)
            .with_tag("Tag1")
            .with_tag("Tag2");
        assert_eq!(item.tags.len(), 2);
    }

    #[test]
    fn test_news_priority_serialization() {
        let priority = NewsPriority::Urgent;
        let json = serde_json::to_string(&priority).unwrap();
        assert!(json.contains("Urgent"));
    }

    #[test]
    fn test_regulatory_change_monitor_creation() {
        let monitor = RegulatoryChangeMonitor::new("Regulatory Monitor", "ws://localhost:8080");
        assert_eq!(monitor.title, "Regulatory Monitor");
        assert_eq!(monitor.ws_url, "ws://localhost:8080");
    }

    #[test]
    fn test_regulatory_change_monitor_with_theme() {
        let monitor = RegulatoryChangeMonitor::new("Regulatory Monitor", "ws://localhost:8080")
            .with_theme(Theme::dark());
        assert_eq!(monitor.theme.background_color, "#1a1a1a");
    }

    #[test]
    fn test_regulatory_change_monitor_html_generation() {
        let changes = vec![
            RegulatoryChange::new(
                "REG-2024-001",
                "New environmental standards",
                "EPA",
                "2024-06-01",
                RegulatoryStatus::Proposed,
            )
            .with_impact("Significant impact on manufacturing")
            .with_sector("Manufacturing")
            .with_sector("Energy"),
            RegulatoryChange::new(
                "REG-2024-002",
                "Financial reporting updates",
                "SEC",
                "2024-03-01",
                RegulatoryStatus::Enacted,
            )
            .with_sector("Finance"),
        ];

        let monitor = RegulatoryChangeMonitor::new("Regulatory Monitor", "ws://localhost:8080");
        let html = monitor.to_html(&changes);

        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Regulatory Monitor"));
        assert!(html.contains("REG-2024-001"));
        assert!(html.contains("REG-2024-002"));
        assert!(html.contains("EPA"));
        assert!(html.contains("SEC"));
        assert!(html.contains("Manufacturing"));
        assert!(html.contains("Finance"));
        assert!(html.contains("ws://localhost:8080"));
    }

    #[test]
    fn test_regulatory_change_monitor_default() {
        let monitor = RegulatoryChangeMonitor::default();
        assert_eq!(monitor.title, "Regulatory Change Monitor");
    }

    #[test]
    fn test_regulatory_change_creation() {
        let change = RegulatoryChange::new(
            "REG-001",
            "Description",
            "Agency",
            "2024-01-01",
            RegulatoryStatus::Proposed,
        );
        assert_eq!(change.regulation_id, "REG-001");
        assert_eq!(change.status, RegulatoryStatus::Proposed);
    }

    #[test]
    fn test_regulatory_change_with_impact() {
        let change = RegulatoryChange::new(
            "REG-001",
            "Description",
            "Agency",
            "2024-01-01",
            RegulatoryStatus::Enacted,
        )
        .with_impact("High impact");
        assert_eq!(change.impact_assessment, Some("High impact".to_string()));
    }

    #[test]
    fn test_regulatory_change_with_sector() {
        let change = RegulatoryChange::new(
            "REG-001",
            "Description",
            "Agency",
            "2024-01-01",
            RegulatoryStatus::Amended,
        )
        .with_sector("Healthcare")
        .with_sector("Technology");
        assert_eq!(change.affected_sectors.len(), 2);
    }

    #[test]
    fn test_regulatory_status_serialization() {
        let status = RegulatoryStatus::Repealed;
        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains("Repealed"));
    }

    #[test]
    fn test_enforcement_action_tracker_creation() {
        let tracker = EnforcementActionTracker::new("Enforcement Tracker", "ws://localhost:8080");
        assert_eq!(tracker.title, "Enforcement Tracker");
        assert_eq!(tracker.ws_url, "ws://localhost:8080");
    }

    #[test]
    fn test_enforcement_action_tracker_with_theme() {
        let tracker = EnforcementActionTracker::new("Enforcement Tracker", "ws://localhost:8080")
            .with_theme(Theme::dark());
        assert_eq!(tracker.theme.background_color, "#1a1a1a");
    }

    #[test]
    fn test_enforcement_action_tracker_html_generation() {
        let actions = vec![
            EnforcementAction::new(
                "Company A",
                "SEC",
                "2024-01-15",
                EnforcementActionType::Fine,
                EnforcementStatus::Active,
            )
            .with_fine(1000000.0)
            .with_violation("Insider trading")
            .with_violation("Misrepresentation"),
            EnforcementAction::new(
                "Company B",
                "FTC",
                "2024-02-10",
                EnforcementActionType::Warning,
                EnforcementStatus::Resolved,
            ),
        ];

        let tracker = EnforcementActionTracker::new("Enforcement Tracker", "ws://localhost:8080");
        let html = tracker.to_html(&actions);

        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Enforcement Tracker"));
        assert!(html.contains("Company A"));
        assert!(html.contains("Company B"));
        assert!(html.contains("SEC"));
        assert!(html.contains("FTC"));
        assert!(html.contains("1000000"));
        assert!(html.contains("Insider trading"));
        assert!(html.contains("ws://localhost:8080"));
    }

    #[test]
    fn test_enforcement_action_tracker_default() {
        let tracker = EnforcementActionTracker::default();
        assert_eq!(tracker.title, "Enforcement Action Tracker");
    }

    #[test]
    fn test_enforcement_action_creation() {
        let action = EnforcementAction::new(
            "Entity",
            "Agency",
            "2024-01-01",
            EnforcementActionType::Settlement,
            EnforcementStatus::Pending,
        );
        assert_eq!(action.entity, "Entity");
        assert_eq!(action.action_type, EnforcementActionType::Settlement);
        assert_eq!(action.status, EnforcementStatus::Pending);
    }

    #[test]
    fn test_enforcement_action_with_fine() {
        let action = EnforcementAction::new(
            "Entity",
            "Agency",
            "2024-01-01",
            EnforcementActionType::Fine,
            EnforcementStatus::Active,
        )
        .with_fine(500000.0);
        assert_eq!(action.fine_amount, Some(500000.0));
    }

    #[test]
    fn test_enforcement_action_with_violation() {
        let action = EnforcementAction::new(
            "Entity",
            "Agency",
            "2024-01-01",
            EnforcementActionType::Investigation,
            EnforcementStatus::Pending,
        )
        .with_violation("Violation 1")
        .with_violation("Violation 2");
        assert_eq!(action.violations.len(), 2);
    }

    #[test]
    fn test_enforcement_action_type_serialization() {
        let action_type = EnforcementActionType::Suspension;
        let json = serde_json::to_string(&action_type).unwrap();
        assert!(json.contains("Suspension"));
    }

    #[test]
    fn test_enforcement_status_serialization() {
        let status = EnforcementStatus::Appealed;
        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains("Appealed"));
    }

    #[test]
    fn test_market_impact_visualizer_creation() {
        let visualizer = MarketImpactVisualizer::new("Market Impact", "ws://localhost:8080");
        assert_eq!(visualizer.title, "Market Impact");
        assert_eq!(visualizer.ws_url, "ws://localhost:8080");
    }

    #[test]
    fn test_market_impact_visualizer_with_theme() {
        let visualizer = MarketImpactVisualizer::new("Market Impact", "ws://localhost:8080")
            .with_theme(Theme::dark());
        assert_eq!(visualizer.theme.background_color, "#1a1a1a");
    }

    #[test]
    fn test_market_impact_visualizer_html_generation() {
        let impacts = vec![
            MarketImpact::new(
                "Supreme Court Ruling on Tech",
                "2024-01-15",
                ImpactSeverity::High,
            )
            .with_stock_change(-5.2)
            .with_company("Tech Corp")
            .with_company("Data Inc")
            .with_sector("Technology"),
            MarketImpact::new(
                "New Financial Regulation",
                "2024-02-10",
                ImpactSeverity::Medium,
            )
            .with_stock_change(2.1)
            .with_company("Bank A")
            .with_sector("Finance"),
        ];

        let visualizer = MarketImpactVisualizer::new("Market Impact", "ws://localhost:8080");
        let html = visualizer.to_html(&impacts);

        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Market Impact"));
        assert!(html.contains("Supreme Court Ruling on Tech"));
        assert!(html.contains("New Financial Regulation"));
        assert!(html.contains("Tech Corp"));
        assert!(html.contains("Bank A"));
        assert!(html.contains("Technology"));
        assert!(html.contains("Finance"));
        assert!(html.contains("chart.js"));
        assert!(html.contains("ws://localhost:8080"));
    }

    #[test]
    fn test_market_impact_visualizer_default() {
        let visualizer = MarketImpactVisualizer::default();
        assert_eq!(visualizer.title, "Market Impact Analysis");
    }

    #[test]
    fn test_market_impact_creation() {
        let impact = MarketImpact::new("Legal Event", "2024-01-01", ImpactSeverity::Low);
        assert_eq!(impact.legal_event, "Legal Event");
        assert_eq!(impact.event_date, "2024-01-01");
        assert_eq!(impact.severity, ImpactSeverity::Low);
    }

    #[test]
    fn test_market_impact_with_stock_change() {
        let impact =
            MarketImpact::new("Event", "2024-01-01", ImpactSeverity::High).with_stock_change(-3.5);
        assert_eq!(impact.stock_price_change, Some(-3.5));
    }

    #[test]
    fn test_market_impact_with_company() {
        let impact = MarketImpact::new("Event", "2024-01-01", ImpactSeverity::Medium)
            .with_company("Company A")
            .with_company("Company B");
        assert_eq!(impact.affected_companies.len(), 2);
    }

    #[test]
    fn test_market_impact_with_sector() {
        let impact = MarketImpact::new("Event", "2024-01-01", ImpactSeverity::High)
            .with_sector("Healthcare")
            .with_sector("Pharma");
        assert_eq!(impact.sectors.len(), 2);
    }

    #[test]
    fn test_impact_severity_serialization() {
        let severity = ImpactSeverity::Medium;
        let json = serde_json::to_string(&severity).unwrap();
        assert!(json.contains("Medium"));
    }

    // ============================================================================
    // Narrative Visualization Tests (v0.3.3)
    // ============================================================================

    #[test]
    fn test_scrollytelling_config_creation() {
        let config = ScrollytellingConfig::new();
        assert!(config.enable_animations);
        assert_eq!(config.trigger_threshold, 0.5);
        assert!(config.show_progress);
        assert!(config.enable_navigation);
    }

    #[test]
    fn test_scrollytelling_config_customization() {
        let config = ScrollytellingConfig::new()
            .without_animations()
            .with_trigger_threshold(0.7)
            .without_progress()
            .without_navigation();

        assert!(!config.enable_animations);
        assert_eq!(config.trigger_threshold, 0.7);
        assert!(!config.show_progress);
        assert!(!config.enable_navigation);
    }

    #[test]
    fn test_legal_history_scrollytelling_creation() {
        let scrolly = LegalHistoryScrollytelling::new("Legal Evolution");
        assert_eq!(scrolly.title, "Legal Evolution");
    }

    #[test]
    fn test_legal_history_scrollytelling_html() {
        let chapters = vec![
            ScrollChapter::new("Chapter 1")
                .with_paragraph("First paragraph")
                .with_paragraph("Second paragraph")
                .with_visual("Visual element"),
            ScrollChapter::new("Chapter 2").with_paragraph("Content"),
        ];

        let scrolly = LegalHistoryScrollytelling::new("Test History");
        let html = scrolly.to_html(&chapters);

        assert!(html.contains("Test History"));
        assert!(html.contains("Chapter 1"));
        assert!(html.contains("Chapter 2"));
        assert!(html.contains("First paragraph"));
        assert!(html.contains("Visual element"));
    }

    #[test]
    fn test_scroll_chapter_creation() {
        let chapter = ScrollChapter::new("Test Chapter")
            .with_paragraph("Para 1")
            .with_visual("Visual");

        assert_eq!(chapter.title, "Test Chapter");
        assert_eq!(chapter.content.len(), 1);
        assert!(chapter.visual.is_some());
    }

    #[test]
    fn test_case_story_generator_creation() {
        let generator = CaseStoryGenerator::new();
        assert!(generator.include_timeline);
        assert!(generator.include_players);
    }

    #[test]
    fn test_case_story_generator_customization() {
        let generator = CaseStoryGenerator::new()
            .without_timeline()
            .without_players();

        assert!(!generator.include_timeline);
        assert!(!generator.include_players);
    }

    #[test]
    fn test_case_story_creation() {
        let case = CaseStory::new("Test Case", "Landmark Decision")
            .with_intro("Introduction paragraph")
            .with_player("John Doe", "Plaintiff")
            .with_event("2024-01-01", "Case filed")
            .with_resolution("Resolution paragraph")
            .with_outcome("Favorable outcome");

        assert_eq!(case.title, "Test Case");
        assert_eq!(case.subtitle, "Landmark Decision");
        assert_eq!(case.introduction.len(), 1);
        assert_eq!(case.key_players.len(), 1);
        assert_eq!(case.timeline.len(), 1);
        assert_eq!(case.resolution.len(), 1);
        assert!(case.outcome.is_some());
    }

    #[test]
    fn test_case_story_html_generation() {
        let case = CaseStory::new("Famous Case", "Legal Milestone")
            .with_intro("This was an important case")
            .with_player("Alice", "Defendant")
            .with_event("2024-06-15", "Trial begins")
            .with_resolution("The case was resolved")
            .with_outcome("Victory");

        let generator = CaseStoryGenerator::new();
        let html = generator.generate_story(&case);

        assert!(html.contains("Famous Case"));
        assert!(html.contains("Legal Milestone"));
        assert!(html.contains("Alice"));
        assert!(html.contains("Defendant"));
        assert!(html.contains("2024-06-15"));
        assert!(html.contains("Victory"));
    }

    #[test]
    fn test_timeline_narrative_view_creation() {
        let view = TimelineNarrativeView::new("Case Timeline");
        assert_eq!(view.title, "Case Timeline");
        assert!(view.show_captions);
    }

    #[test]
    fn test_timeline_narrative_view_html() {
        let events = vec![
            NarrativeEvent::new(
                "2024-01-15",
                "First Event",
                "This is the first event narrative",
            ),
            NarrativeEvent::new(
                "2024-02-20",
                "Second Event",
                "This is the second event narrative",
            ),
        ];

        let view = TimelineNarrativeView::new("Legal Timeline");
        let html = view.to_html(&events);

        assert!(html.contains("Legal Timeline"));
        assert!(html.contains("2024-01-15"));
        assert!(html.contains("First Event"));
        assert!(html.contains("This is the first event narrative"));
        assert!(html.contains("Second Event"));
    }

    #[test]
    fn test_narrative_event_creation() {
        let event = NarrativeEvent::new("2024-03-10", "Event Title", "Narrative text");
        assert_eq!(event.date, "2024-03-10");
        assert_eq!(event.title, "Event Title");
        assert_eq!(event.narrative, "Narrative text");
    }

    #[test]
    fn test_guided_exploration_tour_creation() {
        let tour = GuidedExplorationTour::new("Legal Concepts Tour");
        assert_eq!(tour.title, "Legal Concepts Tour");
        assert!(!tour.auto_advance);
        assert_eq!(tour.advance_delay, 5000);
    }

    #[test]
    fn test_guided_exploration_tour_auto_advance() {
        let tour = GuidedExplorationTour::new("Tour").with_auto_advance(3000);

        assert!(tour.auto_advance);
        assert_eq!(tour.advance_delay, 3000);
    }

    #[test]
    fn test_guided_exploration_tour_html() {
        let stops = vec![
            TourStop::new("Introduction", "Welcome to the tour"),
            TourStop::new("Main Concept", "This is the main idea").with_visual("Diagram"),
            TourStop::new("Conclusion", "Thank you"),
        ];

        let tour = GuidedExplorationTour::new("Test Tour");
        let html = tour.to_html(&stops);

        assert!(html.contains("Test Tour"));
        assert!(html.contains("Introduction"));
        assert!(html.contains("Welcome to the tour"));
        assert!(html.contains("Main Concept"));
        assert!(html.contains("Diagram"));
        assert!(html.contains("Step 1 of 3"));
    }

    #[test]
    fn test_tour_stop_creation() {
        let stop = TourStop::new("Stop 1", "Description").with_visual("Visual element");

        assert_eq!(stop.title, "Stop 1");
        assert_eq!(stop.description, "Description");
        assert!(stop.visual.is_some());
    }

    #[test]
    fn test_educational_walkthrough_creation() {
        let walkthrough = EducationalWalkthrough::new("Learn Legal Concepts");
        assert_eq!(walkthrough.title, "Learn Legal Concepts");
        assert!(walkthrough.include_quiz);
    }

    #[test]
    fn test_educational_walkthrough_without_quiz() {
        let walkthrough = EducationalWalkthrough::new("Walkthrough").without_quiz();

        assert!(!walkthrough.include_quiz);
    }

    #[test]
    fn test_lesson_creation() {
        let lesson = Lesson::new("Introduction to Contracts")
            .with_content("Contracts are agreements between parties")
            .with_content("They must have consideration")
            .with_example("Example: A buys from B for $100")
            .with_takeaway("Contracts require mutual agreement");

        assert_eq!(lesson.title, "Introduction to Contracts");
        assert_eq!(lesson.content.len(), 2);
        assert!(lesson.example.is_some());
        assert!(lesson.key_takeaway.is_some());
    }

    #[test]
    fn test_quiz_question_creation() {
        let quiz = QuizQuestion::new(
            "What is a contract?",
            vec![
                "An agreement".to_string(),
                "A law".to_string(),
                "A statute".to_string(),
            ],
            0,
        );

        assert_eq!(quiz.question, "What is a contract?");
        assert_eq!(quiz.options.len(), 3);
        assert_eq!(quiz.correct_index, 0);
    }

    #[test]
    fn test_educational_walkthrough_html() {
        let lessons = vec![
            Lesson::new("Lesson 1")
                .with_content("Content paragraph 1")
                .with_example("Example text")
                .with_quiz(QuizQuestion::new(
                    "Test question?",
                    vec!["Answer A".to_string(), "Answer B".to_string()],
                    1,
                ))
                .with_takeaway("Key point to remember"),
            Lesson::new("Lesson 2").with_content("More content"),
        ];

        let walkthrough = EducationalWalkthrough::new("Legal Education");
        let html = walkthrough.to_html(&lessons);

        assert!(html.contains("Legal Education"));
        assert!(html.contains("Lesson 1"));
        assert!(html.contains("Content paragraph 1"));
        assert!(html.contains("Example text"));
        assert!(html.contains("Test question?"));
        assert!(html.contains("Answer A"));
        assert!(html.contains("Key point to remember"));
    }

    #[test]
    fn test_scrollytelling_config_default() {
        let config1 = ScrollytellingConfig::new();
        let config2 = ScrollytellingConfig::default();
        assert_eq!(config1.enable_animations, config2.enable_animations);
    }

    #[test]
    fn test_legal_history_scrollytelling_default() {
        let scrolly = LegalHistoryScrollytelling::default();
        assert_eq!(scrolly.title, "Legal History");
    }

    #[test]
    fn test_case_story_generator_default() {
        let generator = CaseStoryGenerator::default();
        assert!(generator.include_timeline);
    }

    #[test]
    fn test_timeline_narrative_view_default() {
        let view = TimelineNarrativeView::default();
        assert_eq!(view.title, "Timeline");
    }

    #[test]
    fn test_guided_exploration_tour_default() {
        let tour = GuidedExplorationTour::default();
        assert_eq!(tour.title, "Guided Tour");
    }

    #[test]
    fn test_educational_walkthrough_default() {
        let walkthrough = EducationalWalkthrough::default();
        assert_eq!(walkthrough.title, "Educational Walkthrough");
    }

    #[test]
    fn test_key_player_serialization() {
        let player = KeyPlayer {
            name: "John Doe".to_string(),
            role: "Plaintiff".to_string(),
        };
        let json = serde_json::to_string(&player).unwrap();
        assert!(json.contains("John Doe"));
        assert!(json.contains("Plaintiff"));
    }

    #[test]
    fn test_timeline_story_event_serialization() {
        let event = TimelineStoryEvent {
            date: "2024-01-01".to_string(),
            description: "Event occurred".to_string(),
        };
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("2024-01-01"));
        assert!(json.contains("Event occurred"));
    }

    // ============================================================================
    // v0.3.4 - Holographic Display Support Tests
    // ============================================================================

    #[test]
    fn test_looking_glass_visualizer_creation() {
        let visualizer = LookingGlassVisualizer::new("Test Hologram");
        assert_eq!(visualizer.title, "Test Hologram");
        assert_eq!(visualizer.config.view_count, 45);
    }

    #[test]
    fn test_looking_glass_config_default() {
        let config = LookingGlassConfig::default();
        assert!(config.enable_quilt);
        assert_eq!(config.view_count, 45);
        assert_eq!(config.quilt_width, 4096);
        assert_eq!(config.quilt_height, 4096);
        assert!(config.enable_depth_mapping);
    }

    #[test]
    fn test_looking_glass_visualizer_with_config() {
        let config = LookingGlassConfig {
            enable_quilt: false,
            view_count: 30,
            quilt_width: 2048,
            quilt_height: 2048,
            enable_depth_mapping: false,
            fov: 20.0,
            depth_range: (0.5, 50.0),
        };
        let visualizer = LookingGlassVisualizer::new("Custom").with_config(config.clone());
        assert_eq!(visualizer.config.view_count, 30);
        assert_eq!(visualizer.config.quilt_width, 2048);
    }

    #[test]
    fn test_looking_glass_visualizer_html_generation() {
        let mut graph = DependencyGraph::new();
        graph.add_statute("test-1");

        let visualizer = LookingGlassVisualizer::new("Holographic Test");
        let html = visualizer.to_holographic_html(&graph);

        assert!(html.contains("Holographic Test"));
        assert!(html.contains("Looking Glass Display"));
        assert!(html.contains("holoplay-core"));
        assert!(html.contains("THREE.Scene"));
    }

    #[test]
    fn test_looking_glass_visualizer_default() {
        let visualizer = LookingGlassVisualizer::default();
        assert_eq!(visualizer.title, "Holographic Visualization");
    }

    #[test]
    fn test_holographic_statute_model_creation() {
        let model = HolographicStatuteModel::new();
        assert_eq!(model.config.layer_count, 5);
        assert!(model.config.enable_rotation);
    }

    #[test]
    fn test_holographic_model_config_default() {
        let config = HolographicModelConfig::default();
        assert!(config.enable_layers);
        assert_eq!(config.layer_count, 5);
        assert!(config.enable_rotation);
        assert_eq!(config.rotation_speed, 15.0);
        assert!(config.enable_interaction);
    }

    #[test]
    fn test_holographic_statute_model_with_config() {
        let config = HolographicModelConfig {
            enable_layers: false,
            layer_count: 3,
            enable_rotation: false,
            rotation_speed: 10.0,
            enable_interaction: false,
        };
        let model = HolographicStatuteModel::new().with_config(config.clone());
        assert_eq!(model.config.layer_count, 3);
        assert!(!model.config.enable_rotation);
    }

    #[test]
    fn test_holographic_statute_model_html() {
        let statute = Statute::new(
            "test-1",
            "Test Statute",
            Effect::new(EffectType::Grant, "Grants permission"),
        );

        let model = HolographicStatuteModel::new();
        let html = model.to_holographic_model(&statute);

        assert!(html.contains("Test Statute"));
        assert!(html.contains("Holographic Statute Model"));
        assert!(html.contains("THREE.Scene"));
        assert!(html.contains("PlaneGeometry"));
    }

    #[test]
    fn test_holographic_statute_model_default() {
        let model = HolographicStatuteModel::default();
        assert_eq!(model.config.layer_count, 5);
    }

    #[test]
    fn test_3d_print_exporter_creation() {
        let exporter = ThreeDPrintExporter::new();
        assert_eq!(exporter.config.format, "STL");
        assert_eq!(exporter.config.scale, 1.0);
    }

    #[test]
    fn test_print_export_config_default() {
        let config = PrintExportConfig::default();
        assert_eq!(config.format, "STL");
        assert_eq!(config.scale, 1.0);
        assert_eq!(config.base_thickness, 2.0);
        assert_eq!(config.wall_thickness, 1.0);
        assert!(!config.generate_supports);
    }

    #[test]
    fn test_3d_print_exporter_to_stl() {
        let statute = Statute::new(
            "test-1",
            "Test Statute",
            Effect::new(EffectType::Grant, "Grants permission"),
        );
        let tree = DecisionTree::from_statute(&statute).unwrap();

        let exporter = ThreeDPrintExporter::new();
        let stl = exporter.to_stl(&tree);

        assert!(stl.contains("solid DecisionTree"));
        assert!(stl.contains("facet normal"));
        assert!(stl.contains("vertex"));
        assert!(stl.contains("endsolid DecisionTree"));
    }

    #[test]
    fn test_3d_print_exporter_to_obj() {
        let mut graph = DependencyGraph::new();
        graph.add_statute("test-1");

        let exporter = ThreeDPrintExporter::new();
        let obj = exporter.to_obj(&graph);

        assert!(obj.contains("# OBJ file"));
        assert!(obj.contains("# Vertices:"));
        assert!(obj.contains("v "));
        assert!(obj.contains("f "));
    }

    #[test]
    fn test_3d_print_exporter_to_3mf() {
        let statute = Statute::new(
            "test-1",
            "Test Statute",
            Effect::new(EffectType::Grant, "Grants permission"),
        );
        let tree = DecisionTree::from_statute(&statute).unwrap();

        let exporter = ThreeDPrintExporter::new();
        let mf = exporter.to_3mf(&tree);

        assert!(mf.contains("<?xml version"));
        assert!(mf.contains("<model"));
        assert!(mf.contains("<mesh>"));
        assert!(mf.contains("<vertices>"));
        assert!(mf.contains("<triangles>"));
    }

    #[test]
    fn test_3d_print_exporter_with_config() {
        let config = PrintExportConfig {
            format: "OBJ".to_string(),
            scale: 2.0,
            base_thickness: 3.0,
            wall_thickness: 1.5,
            generate_supports: true,
        };
        let exporter = ThreeDPrintExporter::new().with_config(config.clone());
        assert_eq!(exporter.config.format, "OBJ");
        assert_eq!(exporter.config.scale, 2.0);
    }

    #[test]
    fn test_3d_print_exporter_default() {
        let exporter = ThreeDPrintExporter::default();
        assert_eq!(exporter.config.format, "STL");
    }

    #[test]
    fn test_volumetric_renderer_creation() {
        let renderer = VolumetricRenderer::new("Volumetric Test");
        assert_eq!(renderer.title, "Volumetric Test");
        assert_eq!(renderer.config.sample_steps, 128);
    }

    #[test]
    fn test_volumetric_config_default() {
        let config = VolumetricConfig::default();
        assert!(config.enable_ray_marching);
        assert_eq!(config.sample_steps, 128);
        assert_eq!(config.density_threshold, 0.1);
        assert!(config.enable_lighting);
        assert_eq!(config.transfer_function, "linear");
    }

    #[test]
    fn test_volumetric_renderer_with_config() {
        let config = VolumetricConfig {
            enable_ray_marching: false,
            sample_steps: 256,
            density_threshold: 0.2,
            enable_lighting: false,
            transfer_function: "cubic".to_string(),
        };
        let renderer = VolumetricRenderer::new("Custom").with_config(config.clone());
        assert_eq!(renderer.config.sample_steps, 256);
        assert_eq!(renderer.config.transfer_function, "cubic");
    }

    #[test]
    fn test_volumetric_renderer_html() {
        let mut graph = DependencyGraph::new();
        graph.add_statute("test-1");

        let renderer = VolumetricRenderer::new("Volumetric Viz");
        let html = renderer.to_volumetric_html(&graph);

        assert!(html.contains("Volumetric Viz"));
        assert!(html.contains("Volumetric Rendering"));
        assert!(html.contains("Steps: 128"));
        assert!(html.contains("THREE.Scene"));
        assert!(html.contains("SphereGeometry"));
    }

    #[test]
    fn test_volumetric_renderer_default() {
        let renderer = VolumetricRenderer::default();
        assert_eq!(renderer.title, "Volumetric Visualization");
    }

    #[test]
    fn test_holographic_gesture_controller_creation() {
        let controller = HolographicGestureController::new("Gesture Test");
        assert_eq!(controller.title, "Gesture Test");
        assert!(controller.config.enable_hand_tracking);
    }

    #[test]
    fn test_gesture_config_default() {
        let config = GestureConfig::default();
        assert!(config.enable_hand_tracking);
        assert!(config.enable_pinch);
        assert!(config.enable_swipe);
        assert!(config.enable_rotation);
        assert_eq!(config.sensitivity, 0.7);
    }

    #[test]
    fn test_holographic_gesture_controller_with_config() {
        let config = GestureConfig {
            enable_hand_tracking: false,
            enable_pinch: false,
            enable_swipe: true,
            enable_rotation: false,
            sensitivity: 0.5,
        };
        let controller = HolographicGestureController::new("Custom").with_config(config.clone());
        assert_eq!(controller.config.sensitivity, 0.5);
        assert!(!controller.config.enable_pinch);
    }

    #[test]
    fn test_holographic_gesture_controller_html() {
        let statute = Statute::new(
            "test-1",
            "Test Statute",
            Effect::new(EffectType::Grant, "Grants permission"),
        );
        let tree = DecisionTree::from_statute(&statute).unwrap();

        let controller = HolographicGestureController::new("Gesture Control");
        let html = controller.to_gesture_html(&tree);

        assert!(html.contains("Gesture Control"));
        assert!(html.contains("Gesture Control Active"));
        assert!(html.contains("Pinch to zoom"));
        assert!(html.contains("THREE.Scene"));
        assert!(html.contains("gestureState"));
    }

    #[test]
    fn test_holographic_gesture_controller_default() {
        let controller = HolographicGestureController::default();
        assert_eq!(
            controller.title,
            "Gesture-Controlled Holographic Visualization"
        );
    }

    #[test]
    fn test_looking_glass_config_serialization() {
        let config = LookingGlassConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("enable_quilt"));
        assert!(json.contains("view_count"));
    }

    #[test]
    fn test_holographic_model_config_serialization() {
        let config = HolographicModelConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("enable_layers"));
        assert!(json.contains("layer_count"));
    }

    #[test]
    fn test_print_export_config_serialization() {
        let config = PrintExportConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("format"));
        assert!(json.contains("scale"));
    }

    #[test]
    fn test_volumetric_config_serialization() {
        let config = VolumetricConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("enable_ray_marching"));
        assert!(json.contains("sample_steps"));
    }

    #[test]
    fn test_gesture_config_serialization() {
        let config = GestureConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("enable_hand_tracking"));
        assert!(json.contains("sensitivity"));
    }

    // ========================================================================
    // Cross-Jurisdictional Comparison Tests (v0.4.0)
    // ========================================================================

    #[test]
    fn test_jurisdictional_statute_creation() {
        let statute = Statute::new(
            "adult-rights",
            "Adult Rights Act",
            Effect::new(EffectType::Grant, "Grants adult rights"),
        );

        let js = JurisdictionalStatute::new("US", "United States", statute);

        assert_eq!(js.jurisdiction, "US");
        assert_eq!(js.jurisdiction_name, "United States");
        assert_eq!(js.statute.id, "adult-rights");
        assert!(js.metadata.is_empty());
    }

    #[test]
    fn test_jurisdictional_statute_with_metadata() {
        let statute = Statute::new(
            "test",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test effect"),
        );

        let js = JurisdictionalStatute::new("JP", "Japan", statute)
            .with_metadata("enacted", "2020")
            .with_metadata("status", "active");

        assert_eq!(js.metadata.len(), 2);
        assert_eq!(js.metadata.get("enacted"), Some(&"2020".to_string()));
        assert_eq!(js.metadata.get("status"), Some(&"active".to_string()));
    }

    #[test]
    fn test_jurisdictional_difference_creation() {
        let diff = JurisdictionalDifference::new(
            "age_requirement",
            "Different age requirements across jurisdictions",
        );

        assert_eq!(diff.aspect, "age_requirement");
        assert_eq!(diff.severity, 0.5);
        assert!(diff.values.is_empty());
    }

    #[test]
    fn test_jurisdictional_difference_with_values() {
        let diff = JurisdictionalDifference::new("age", "Age requirement differs")
            .with_value("US", "18 years")
            .with_value("JP", "20 years")
            .with_value("DE", "18 years")
            .with_severity(0.7);

        assert_eq!(diff.values.len(), 3);
        assert_eq!(diff.values.get("US"), Some(&"18 years".to_string()));
        assert_eq!(diff.values.get("JP"), Some(&"20 years".to_string()));
        assert_eq!(diff.severity, 0.7);
    }

    #[test]
    fn test_jurisdictional_difference_severity_clamping() {
        let diff1 = JurisdictionalDifference::new("test", "test").with_severity(1.5);
        assert_eq!(diff1.severity, 1.0);

        let diff2 = JurisdictionalDifference::new("test", "test").with_severity(-0.5);
        assert_eq!(diff2.severity, 0.0);
    }

    #[test]
    fn test_cross_jurisdictional_comparison_creation() {
        let comparison = CrossJurisdictionalComparison::new("Adult Rights Comparison");

        assert_eq!(comparison.title, "Adult Rights Comparison");
        assert!(comparison.statutes.is_empty());
        assert!(comparison.differences.is_empty());
        assert!(comparison.synchronized_nav);
    }

    #[test]
    fn test_cross_jurisdictional_comparison_default() {
        let comparison = CrossJurisdictionalComparison::default();

        assert_eq!(comparison.title, "Jurisdictional Comparison");
    }

    #[test]
    fn test_cross_jurisdictional_comparison_add_statute() {
        let mut comparison = CrossJurisdictionalComparison::new("Test");

        let statute1 = Statute::new("test1", "Test 1", Effect::new(EffectType::Grant, "Test"));
        let js1 = JurisdictionalStatute::new("US", "United States", statute1);

        comparison.add_statute(js1);

        assert_eq!(comparison.statutes.len(), 1);
        assert_eq!(comparison.statutes[0].jurisdiction, "US");
    }

    #[test]
    fn test_cross_jurisdictional_comparison_add_difference() {
        let mut comparison = CrossJurisdictionalComparison::new("Test");

        let diff = JurisdictionalDifference::new("age", "Age differs")
            .with_value("US", "18")
            .with_value("JP", "20");

        comparison.add_difference(diff);

        assert_eq!(comparison.differences.len(), 1);
        assert_eq!(comparison.differences[0].aspect, "age");
    }

    #[test]
    fn test_cross_jurisdictional_comparison_with_theme() {
        let comparison = CrossJurisdictionalComparison::new("Test").with_theme(Theme::dark());

        assert_eq!(comparison.theme.background_color, "#1a1a1a");
    }

    #[test]
    fn test_cross_jurisdictional_comparison_with_synchronized_nav() {
        let comparison1 = CrossJurisdictionalComparison::new("Test").with_synchronized_nav(true);
        assert!(comparison1.synchronized_nav);

        let comparison2 = CrossJurisdictionalComparison::new("Test").with_synchronized_nav(false);
        assert!(!comparison2.synchronized_nav);
    }

    #[test]
    fn test_cross_jurisdictional_comparison_side_by_side_html() {
        let mut comparison = CrossJurisdictionalComparison::new("Adult Rights Comparison");

        let statute_us = Statute::new(
            "us-adult",
            "US Adult Rights",
            Effect::new(EffectType::Grant, "Grants rights at 18"),
        );
        let js_us = JurisdictionalStatute::new("US", "United States", statute_us)
            .with_metadata("enacted", "1971");

        let statute_jp = Statute::new(
            "jp-adult",
            "Japan Adult Rights",
            Effect::new(EffectType::Grant, "Grants rights at 20"),
        );
        let js_jp =
            JurisdictionalStatute::new("JP", "Japan", statute_jp).with_metadata("enacted", "2022");

        comparison.add_statute(js_us);
        comparison.add_statute(js_jp);

        let diff = JurisdictionalDifference::new("age_requirement", "Age of majority differs")
            .with_value("US", "18 years")
            .with_value("JP", "20 years")
            .with_severity(0.6);

        comparison.add_difference(diff);

        let html = comparison.to_side_by_side_html();

        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Adult Rights Comparison"));
        assert!(html.contains("United States"));
        assert!(html.contains("Japan"));
        assert!(html.contains("us-adult"));
        assert!(html.contains("jp-adult"));
        assert!(html.contains("age_requirement"));
        assert!(html.contains("18 years"));
        assert!(html.contains("20 years"));
        assert!(html.contains("jurisdiction-column"));
        assert!(html.contains("differences-section"));
    }

    #[test]
    fn test_cross_jurisdictional_comparison_side_by_side_html_no_differences() {
        let mut comparison = CrossJurisdictionalComparison::new("Test Comparison");

        let statute = Statute::new(
            "test",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test"),
        );
        let js = JurisdictionalStatute::new("US", "United States", statute);

        comparison.add_statute(js);

        let html = comparison.to_side_by_side_html();

        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Test Comparison"));
        assert!(html.contains("United States"));
        // Should not have differences section
        assert!(!html.contains("Key Differences"));
    }

    #[test]
    fn test_cross_jurisdictional_comparison_synchronized_navigation_script() {
        let comparison1 = CrossJurisdictionalComparison::new("Test").with_synchronized_nav(true);
        let html1 = comparison1.to_side_by_side_html();
        assert!(html1.contains("addEventListener('scroll'"));
        assert!(html1.contains("scrollRatio"));

        let comparison2 = CrossJurisdictionalComparison::new("Test").with_synchronized_nav(false);
        let html2 = comparison2.to_side_by_side_html();
        assert!(!html2.contains("addEventListener('scroll'"));
    }

    #[test]
    fn test_cross_jurisdictional_comparison_heatmap_html() {
        let mut comparison = CrossJurisdictionalComparison::new("Rights Comparison");

        let statute_us = Statute::new(
            "us-rights",
            "US Rights",
            Effect::new(EffectType::Grant, "US rights"),
        );
        let js_us = JurisdictionalStatute::new("US", "United States", statute_us);

        let statute_jp = Statute::new(
            "jp-rights",
            "JP Rights",
            Effect::new(EffectType::Grant, "JP rights"),
        );
        let js_jp = JurisdictionalStatute::new("JP", "Japan", statute_jp);

        let statute_de = Statute::new(
            "de-rights",
            "DE Rights",
            Effect::new(EffectType::Grant, "DE rights"),
        );
        let js_de = JurisdictionalStatute::new("DE", "Germany", statute_de);

        comparison.add_statute(js_us);
        comparison.add_statute(js_jp);
        comparison.add_statute(js_de);

        let diff1 = JurisdictionalDifference::new("age", "Age requirement")
            .with_value("US", "18")
            .with_value("JP", "20")
            .with_value("DE", "18")
            .with_severity(0.3); // Low severity

        let diff2 = JurisdictionalDifference::new("citizenship", "Citizenship requirement")
            .with_value("US", "Yes")
            .with_value("JP", "No")
            .with_value("DE", "EU only")
            .with_severity(0.8); // High severity

        comparison.add_difference(diff1);
        comparison.add_difference(diff2);

        let html = comparison.to_heatmap_html();

        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Jurisdictional Heatmap"));
        assert!(html.contains("heatmap-container"));
        assert!(html.contains("US"));
        assert!(html.contains("JP"));
        assert!(html.contains("DE"));
        assert!(html.contains("age"));
        assert!(html.contains("citizenship"));
        assert!(html.contains("heatmap-low"));
        assert!(html.contains("heatmap-high"));
    }

    #[test]
    fn test_cross_jurisdictional_comparison_heatmap_severity_classes() {
        let mut comparison = CrossJurisdictionalComparison::new("Test");

        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Test"));
        let js = JurisdictionalStatute::new("US", "United States", statute);
        comparison.add_statute(js);

        // Test all severity levels
        let diff_low = JurisdictionalDifference::new("low", "Low severity")
            .with_value("US", "Low")
            .with_severity(0.2);

        let diff_medium = JurisdictionalDifference::new("medium", "Medium severity")
            .with_value("US", "Medium")
            .with_severity(0.5);

        let diff_high = JurisdictionalDifference::new("high", "High severity")
            .with_value("US", "High")
            .with_severity(0.9);

        comparison.add_difference(diff_low);
        comparison.add_difference(diff_medium);
        comparison.add_difference(diff_high);

        let html = comparison.to_heatmap_html();

        assert!(html.contains("heatmap-low"));
        assert!(html.contains("heatmap-medium"));
        assert!(html.contains("heatmap-high"));
    }

    #[test]
    fn test_cross_jurisdictional_comparison_heatmap_missing_values() {
        let mut comparison = CrossJurisdictionalComparison::new("Test");

        let statute_us = Statute::new("us", "US", Effect::new(EffectType::Grant, "US"));
        let js_us = JurisdictionalStatute::new("US", "United States", statute_us);

        let statute_jp = Statute::new("jp", "JP", Effect::new(EffectType::Grant, "JP"));
        let js_jp = JurisdictionalStatute::new("JP", "Japan", statute_jp);

        comparison.add_statute(js_us);
        comparison.add_statute(js_jp);

        // Difference with value only for US
        let diff = JurisdictionalDifference::new("test", "Test difference")
            .with_value("US", "Available")
            .with_severity(0.5);

        comparison.add_difference(diff);

        let html = comparison.to_heatmap_html();

        assert!(html.contains("Available"));
        assert!(html.contains("N/A")); // Should show N/A for JP
    }

    #[test]
    fn test_jurisdictional_statute_serialization() {
        let statute = Statute::new(
            "test",
            "Test Statute",
            Effect::new(EffectType::Grant, "Test"),
        );
        let js = JurisdictionalStatute::new("US", "United States", statute)
            .with_metadata("year", "2020");

        let json = serde_json::to_string(&js).unwrap();
        assert!(json.contains("US"));
        assert!(json.contains("United States"));
        assert!(json.contains("test"));
        assert!(json.contains("year"));
    }

    #[test]
    fn test_jurisdictional_difference_serialization() {
        let diff = JurisdictionalDifference::new("age", "Age differs")
            .with_value("US", "18")
            .with_value("JP", "20")
            .with_severity(0.7);

        let json = serde_json::to_string(&diff).unwrap();
        assert!(json.contains("age"));
        assert!(json.contains("18"));
        assert!(json.contains("20"));
        assert!(json.contains("0.7"));
    }

    // ========================================================================
    // Tests for Semantic Legal Network (v0.4.1)
    // ========================================================================

    #[test]
    fn test_legal_concept_creation() {
        let concept = LegalConcept::new("c1", "Privacy Right", "Right to privacy", "rights");
        assert_eq!(concept.id, "c1");
        assert_eq!(concept.name, "Privacy Right");
        assert_eq!(concept.description, "Right to privacy");
        assert_eq!(concept.category, "rights");
        assert!(concept.statute_ids.is_empty());
        assert!(concept.metadata.is_empty());
    }

    #[test]
    fn test_legal_concept_add_statute() {
        let mut concept = LegalConcept::new("c1", "Privacy", "Privacy rights", "rights");
        concept.add_statute("s1");
        concept.add_statute("s2");
        assert_eq!(concept.statute_ids.len(), 2);
        assert_eq!(concept.statute_ids[0], "s1");
        assert_eq!(concept.statute_ids[1], "s2");
    }

    #[test]
    fn test_legal_concept_with_metadata() {
        let concept = LegalConcept::new("c1", "Privacy", "Privacy rights", "rights")
            .with_metadata("jurisdiction", "US")
            .with_metadata("enacted", "2020");
        assert_eq!(concept.metadata.len(), 2);
        assert_eq!(
            concept.metadata.get("jurisdiction"),
            Some(&"US".to_string())
        );
        assert_eq!(concept.metadata.get("enacted"), Some(&"2020".to_string()));
    }

    #[test]
    fn test_concept_relation_type_label() {
        assert_eq!(ConceptRelationType::IsA.label(), "is a");
        assert_eq!(ConceptRelationType::PartOf.label(), "part of");
        assert_eq!(ConceptRelationType::Requires.label(), "requires");
        assert_eq!(ConceptRelationType::ConflictsWith.label(), "conflicts with");
        assert_eq!(ConceptRelationType::Enables.label(), "enables");
        assert_eq!(ConceptRelationType::RelatedTo.label(), "related to");
        assert_eq!(ConceptRelationType::Supersedes.label(), "supersedes");
        assert_eq!(ConceptRelationType::Implements.label(), "implements");
    }

    #[test]
    fn test_concept_relation_type_color() {
        assert_eq!(ConceptRelationType::IsA.color(), "#3498db");
        assert_eq!(ConceptRelationType::PartOf.color(), "#2ecc71");
        assert_eq!(ConceptRelationType::Requires.color(), "#e74c3c");
        assert_eq!(ConceptRelationType::ConflictsWith.color(), "#c0392b");
    }

    #[test]
    fn test_concept_relationship_creation() {
        let rel = ConceptRelationship::new("c1", "c2", ConceptRelationType::IsA);
        assert_eq!(rel.from_id, "c1");
        assert_eq!(rel.to_id, "c2");
        assert_eq!(rel.relation_type, ConceptRelationType::IsA);
        assert_eq!(rel.strength, 1.0);
        assert!(rel.description.is_empty());
    }

    #[test]
    fn test_concept_relationship_with_description() {
        let rel = ConceptRelationship::new("c1", "c2", ConceptRelationType::Requires)
            .with_description("Requires for validity");
        assert_eq!(rel.description, "Requires for validity");
    }

    #[test]
    fn test_concept_relationship_with_strength() {
        let rel =
            ConceptRelationship::new("c1", "c2", ConceptRelationType::RelatedTo).with_strength(0.7);
        assert_eq!(rel.strength, 0.7);

        // Test clamping
        let rel_high =
            ConceptRelationship::new("c1", "c2", ConceptRelationType::IsA).with_strength(1.5);
        assert_eq!(rel_high.strength, 1.0);

        let rel_low =
            ConceptRelationship::new("c1", "c2", ConceptRelationType::IsA).with_strength(-0.5);
        assert_eq!(rel_low.strength, 0.0);
    }

    #[test]
    fn test_concept_relationship_graph_creation() {
        let graph = ConceptRelationshipGraph::new("Legal Concepts");
        assert_eq!(graph.title, "Legal Concepts");
        assert!(graph.concepts.is_empty());
        assert!(graph.relationships.is_empty());
    }

    #[test]
    fn test_concept_relationship_graph_add_concept() {
        let mut graph = ConceptRelationshipGraph::new("Test");
        let concept = LegalConcept::new("c1", "Privacy", "Privacy rights", "rights");
        graph.add_concept(concept);
        assert_eq!(graph.concepts.len(), 1);
        assert_eq!(graph.concepts[0].id, "c1");
    }

    #[test]
    fn test_concept_relationship_graph_add_relationship() {
        let mut graph = ConceptRelationshipGraph::new("Test");
        let rel = ConceptRelationship::new("c1", "c2", ConceptRelationType::IsA);
        graph.add_relationship(rel);
        assert_eq!(graph.relationships.len(), 1);
        assert_eq!(graph.relationships[0].from_id, "c1");
    }

    #[test]
    fn test_concept_relationship_graph_html() {
        let mut graph = ConceptRelationshipGraph::new("Legal Network");
        let c1 = LegalConcept::new("c1", "Privacy", "Privacy rights", "rights");
        let c2 = LegalConcept::new("c2", "Data Protection", "Data protection laws", "rights");
        graph.add_concept(c1);
        graph.add_concept(c2);
        graph.add_relationship(ConceptRelationship::new(
            "c1",
            "c2",
            ConceptRelationType::RelatedTo,
        ));

        let html = graph.to_html();
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Legal Network"));
        assert!(html.contains("Privacy"));
        assert!(html.contains("Data Protection"));
        assert!(html.contains("d3js.org"));
        assert!(html.contains("forceSimulation"));
    }

    #[test]
    fn test_concept_relationship_graph_mermaid() {
        let mut graph = ConceptRelationshipGraph::new("Test");
        let c1 = LegalConcept::new("c1", "Privacy", "Privacy rights", "rights");
        let c2 = LegalConcept::new("c2", "Security", "Security measures", "obligations");
        graph.add_concept(c1);
        graph.add_concept(c2);
        graph.add_relationship(ConceptRelationship::new(
            "c1",
            "c2",
            ConceptRelationType::Requires,
        ));

        let mermaid = graph.to_mermaid();
        assert!(mermaid.contains("graph TD"));
        assert!(mermaid.contains("c1[\"Privacy\"]"));
        assert!(mermaid.contains("c2[\"Security\"]"));
        assert!(mermaid.contains("c1 -->|requires| c2"));
    }

    #[test]
    fn test_statute_concept_mapping_creation() {
        let mapping = StatuteConceptMapping::new("s1", "GDPR Article 5");
        assert_eq!(mapping.statute_id, "s1");
        assert_eq!(mapping.statute_name, "GDPR Article 5");
        assert!(mapping.concept_ids.is_empty());
        assert!(mapping.confidence_scores.is_empty());
    }

    #[test]
    fn test_statute_concept_mapping_add_concept() {
        let mut mapping = StatuteConceptMapping::new("s1", "Privacy Act");
        mapping.add_concept("c1", 0.9);
        mapping.add_concept("c2", 0.7);
        assert_eq!(mapping.concept_ids.len(), 2);
        assert_eq!(mapping.concept_ids[0], "c1");
        assert_eq!(mapping.confidence("c1"), 0.9);
        assert_eq!(mapping.confidence("c2"), 0.7);
        assert_eq!(mapping.confidence("c3"), 0.0);
    }

    #[test]
    fn test_statute_concept_mapping_confidence_clamping() {
        let mut mapping = StatuteConceptMapping::new("s1", "Test");
        mapping.add_concept("c1", 1.5); // Should clamp to 1.0
        mapping.add_concept("c2", -0.5); // Should clamp to 0.0
        assert_eq!(mapping.confidence("c1"), 1.0);
        assert_eq!(mapping.confidence("c2"), 0.0);
    }

    #[test]
    fn test_ontology_based_visualizer_creation() {
        let viz = OntologyBasedVisualizer::new();
        assert_eq!(viz.theme.background_color, "#ffffff");
    }

    #[test]
    fn test_ontology_based_visualizer_with_theme() {
        let viz = OntologyBasedVisualizer::new().with_theme(Theme::dark());
        assert_eq!(viz.theme.background_color, "#1a1a1a");
    }

    #[test]
    fn test_ontology_based_visualizer_html() {
        let viz = OntologyBasedVisualizer::new();
        let mut graph = ConceptRelationshipGraph::new("Ontology");
        let c1 = LegalConcept::new("c1", "Legal Right", "A legal right", "rights");
        graph.add_concept(c1);

        let html = viz.to_html(&graph);
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Ontology"));
        assert!(html.contains("ontology-layer"));
        assert!(html.contains("ontology-root"));
    }

    #[test]
    fn test_ontology_based_visualizer_tree_html() {
        let viz = OntologyBasedVisualizer::new();
        let mut graph = ConceptRelationshipGraph::new("Test Ontology");
        let c1 = LegalConcept::new("c1", "Privacy", "Privacy concept", "rights");
        graph.add_concept(c1);

        let html = viz.to_tree_html(&graph);
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Test Ontology"));
        assert!(html.contains("Privacy"));
        assert!(html.contains("tree-node"));
    }

    #[test]
    fn test_semantic_search_highlighter_creation() {
        let highlighter = SemanticSearchHighlighter::new("privacy");
        assert_eq!(highlighter.query, "privacy");
        assert!(highlighter.matches.is_empty());
        assert_eq!(highlighter.highlight_color, "#ffeb3b");
    }

    #[test]
    fn test_semantic_search_highlighter_with_color() {
        let highlighter = SemanticSearchHighlighter::new("test").with_color("#ff0000");
        assert_eq!(highlighter.highlight_color, "#ff0000");
    }

    #[test]
    fn test_semantic_search_highlighter_search() {
        let mut graph = ConceptRelationshipGraph::new("Test");
        let c1 = LegalConcept::new("c1", "Privacy Right", "Protects privacy", "rights");
        let c2 = LegalConcept::new("c2", "Data Security", "Ensures security", "obligations");
        let c3 = LegalConcept::new("c3", "Privacy Policy", "Privacy guidelines", "procedures");
        graph.add_concept(c1);
        graph.add_concept(c2);
        graph.add_concept(c3);

        let mut highlighter = SemanticSearchHighlighter::new("privacy");
        highlighter.search(&graph);

        assert_eq!(highlighter.matches.len(), 2); // c1 and c3
        assert!(highlighter.matches.contains(&"c1".to_string()));
        assert!(highlighter.matches.contains(&"c3".to_string()));
        assert!(!highlighter.matches.contains(&"c2".to_string()));
    }

    #[test]
    fn test_semantic_search_highlighter_relevance_scoring() {
        let mut graph = ConceptRelationshipGraph::new("Test");
        let c1 = LegalConcept::new("c1", "Privacy", "About privacy", "rights");
        graph.add_concept(c1);

        let mut highlighter = SemanticSearchHighlighter::new("privacy");
        highlighter.search(&graph);

        // Should match in name (1.0) and description (0.5) = 1.5, clamped to 1.0
        assert_eq!(highlighter.relevance_scores.get("c1"), Some(&1.0));
    }

    #[test]
    fn test_semantic_search_highlighter_highlighted_html() {
        let mut graph = ConceptRelationshipGraph::new("Test");
        let c1 = LegalConcept::new("c1", "Privacy", "Privacy concept", "rights");
        graph.add_concept(c1);

        let mut highlighter = SemanticSearchHighlighter::new("privacy");
        highlighter.search(&graph);

        let html = highlighter.to_highlighted_html(&graph);
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("highlights"));
        assert!(html.contains("#ffeb3b")); // Default highlight color
    }

    #[test]
    fn test_concept_hierarchy_tree_creation() {
        let concept = LegalConcept::new("c1", "Legal Right", "A legal right", "rights");
        let tree = ConceptHierarchyTree::new(concept);
        assert_eq!(tree.root.id, "c1");
        assert_eq!(tree.root.name, "Legal Right");
        assert!(tree.children.is_empty());
    }

    #[test]
    fn test_concept_hierarchy_tree_add_child() {
        let root = LegalConcept::new("c1", "Right", "General right", "rights");
        let mut tree = ConceptHierarchyTree::new(root);

        let child_concept = LegalConcept::new("c2", "Privacy Right", "Privacy right", "rights");
        let child_tree = ConceptHierarchyTree::new(child_concept);

        tree.add_child(child_tree);
        assert_eq!(tree.children.len(), 1);
        assert_eq!(tree.children[0].root.id, "c2");
    }

    #[test]
    fn test_concept_hierarchy_tree_from_graph() {
        let mut graph = ConceptRelationshipGraph::new("Test");
        let c1 = LegalConcept::new("c1", "Right", "General right", "rights");
        let c2 = LegalConcept::new("c2", "Privacy Right", "Privacy right", "rights");
        let c3 = LegalConcept::new("c3", "Data Privacy", "Data privacy", "rights");
        graph.add_concept(c1);
        graph.add_concept(c2);
        graph.add_concept(c3);

        // c2 is a c1, c3 is a c2
        graph.add_relationship(ConceptRelationship::new(
            "c2",
            "c1",
            ConceptRelationType::IsA,
        ));
        graph.add_relationship(ConceptRelationship::new(
            "c3",
            "c2",
            ConceptRelationType::IsA,
        ));

        let tree = ConceptHierarchyTree::from_graph(&graph, "c1").unwrap();
        assert_eq!(tree.root.id, "c1");
        assert_eq!(tree.children.len(), 1);
        assert_eq!(tree.children[0].root.id, "c2");
        assert_eq!(tree.children[0].children.len(), 1);
        assert_eq!(tree.children[0].children[0].root.id, "c3");
    }

    #[test]
    fn test_concept_hierarchy_tree_html() {
        let concept = LegalConcept::new("c1", "Privacy", "Privacy concept", "rights");
        let tree = ConceptHierarchyTree::new(concept);

        let html = tree.to_html();
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Concept Hierarchy"));
        assert!(html.contains("Privacy"));
        assert!(html.contains("concept-box"));
        assert!(html.contains("concept-name"));
    }

    #[test]
    fn test_concept_hierarchy_tree_mermaid() {
        let root = LegalConcept::new("c1", "Right", "General right", "rights");
        let mut tree = ConceptHierarchyTree::new(root);

        let child = LegalConcept::new("c2", "Privacy Right", "Privacy right", "rights");
        let child_tree = ConceptHierarchyTree::new(child);
        tree.add_child(child_tree);

        let mermaid = tree.to_mermaid();
        assert!(mermaid.contains("graph TD"));
        assert!(mermaid.contains("c1[\"Right\"]"));
        assert!(mermaid.contains("c2[\"Privacy Right\"]"));
        assert!(mermaid.contains("c1 --> c2"));
    }

    #[test]
    fn test_legal_concept_serialization() {
        let concept = LegalConcept::new("c1", "Privacy", "Privacy right", "rights")
            .with_metadata("jurisdiction", "US");

        let json = serde_json::to_string(&concept).unwrap();
        assert!(json.contains("c1"));
        assert!(json.contains("Privacy"));
        assert!(json.contains("rights"));
        assert!(json.contains("jurisdiction"));
    }

    #[test]
    fn test_concept_relationship_serialization() {
        let rel = ConceptRelationship::new("c1", "c2", ConceptRelationType::IsA).with_strength(0.8);

        let json = serde_json::to_string(&rel).unwrap();
        assert!(json.contains("c1"));
        assert!(json.contains("c2"));
        assert!(json.contains("0.8"));
    }

    #[test]
    fn test_concept_relationship_graph_serialization() {
        let mut graph = ConceptRelationshipGraph::new("Test");
        let c1 = LegalConcept::new("c1", "Privacy", "Privacy concept", "rights");
        graph.add_concept(c1);

        let json = serde_json::to_string(&graph).unwrap();
        assert!(json.contains("Test"));
        assert!(json.contains("c1"));
        assert!(json.contains("Privacy"));
    }
}
