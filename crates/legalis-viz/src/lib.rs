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
            "        .node circle { fill: #fff; stroke: steelblue; stroke-width: 3px; }\n",
        );
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
            "        .link {{ fill: none; stroke: {}; stroke-width: 2px; }}\n",
            theme.link_color
        ));
        html.push_str("        .link-label { font-size: 10px; fill: #666; }\n");
        html.push_str("    </style>\n</head>\n<body>\n");
        html.push_str("    <h1>Legal Decision Tree</h1>\n");
        html.push_str("    <div id=\"tree\"></div>\n");
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
        let mut svg = String::new();
        let width = self.layout_config.width;
        let height = self.layout_config.height;

        svg.push_str(&format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\">\n",
            width, height
        ));

        svg.push_str("  <defs>\n");
        svg.push_str("    <marker id=\"arrow\" markerWidth=\"10\" markerHeight=\"10\" refX=\"9\" refY=\"3\" orient=\"auto\" markerUnits=\"strokeWidth\">\n");
        svg.push_str("      <path d=\"M0,0 L0,6 L9,3 z\" fill=\"#666\" />\n");
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
                        "  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"#999\" stroke-width=\"2\" marker-end=\"url(#arrow)\"/>\n",
                        x1, y1, x2, y2
                    ));

                    // Add edge label
                    let label = &self.graph[edge];
                    let mid_x = (x1 + x2) / 2;
                    let mid_y = (y1 + y2) / 2;
                    svg.push_str(&format!(
                        "  <text x=\"{}\" y=\"{}\" font-size=\"10\" fill=\"#666\" text-anchor=\"middle\">{}</text>\n",
                        mid_x, mid_y.saturating_sub(5), label
                    ));
                }
            }
        }

        // Draw nodes
        for node_idx in self.graph.node_indices() {
            if let Some(&(x, y)) = node_positions.get(&node_idx) {
                let statute_id = &self.graph[node_idx];

                svg.push_str(&format!(
                    "  <circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"#69b3a2\" stroke=\"#fff\" stroke-width=\"2\"/>\n",
                    x, y, node_radius
                ));

                // Truncate long statute IDs
                let display_id = if statute_id.len() > 12 {
                    format!("{}...", &statute_id[..9])
                } else {
                    statute_id.clone()
                };

                svg.push_str(&format!(
                    "  <text x=\"{}\" y=\"{}\" font-size=\"10\" fill=\"#fff\" text-anchor=\"middle\">{}</text>\n",
                    x, y + 4, display_id
                ));
            }
        }

        svg.push_str("</svg>");
        svg
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
        Condition::Between {
            attribute,
            min,
            max,
        } => {
            format!("{} BETWEEN {} AND {}", attribute, min, max)
        }
        Condition::InSet { attribute, values } => {
            format!("{} IN ({})", attribute, values.join(", "))
        }
        Condition::TimeOfDay {
            start_hour,
            start_minute,
            end_hour,
            end_minute,
        } => {
            format!(
                "Time {:02}:{:02}-{:02}:{:02}",
                start_hour, start_minute, end_hour, end_minute
            )
        }
        Condition::Percentage {
            attribute,
            operator,
            percentage,
            of_value,
        } => {
            format!(
                "{} {} {}% of {}",
                attribute,
                format_operator(operator),
                percentage,
                of_value
            )
        }
        Condition::Formula {
            expression,
            operator,
            value,
        } => {
            format!("{} {} {}", expression, format_operator(operator), value)
        }
        Condition::Count {
            attribute,
            operator,
            count,
        } => {
            format!(
                "Count({}) {} {}",
                attribute,
                format_operator(operator),
                count
            )
        }
        Condition::Pattern {
            attribute,
            pattern,
            pattern_type,
        } => {
            format!("{} {:?} '{}'", attribute, pattern_type, pattern)
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
}
