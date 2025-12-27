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
}
