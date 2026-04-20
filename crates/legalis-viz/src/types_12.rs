//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use legalis_core::{Condition, Statute};
use petgraph::dot::{Config, Dot};
use petgraph::graph::{DiGraph, NodeIndex};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(feature = "png-export")]
use super::functions::svg_to_png;
use super::functions::{VizResult, format_condition};
use super::types::AnomalyType;
use super::types_6::{EdgeLabel, EnforcementStatus};
use super::types_8::{Annotation, EnforcementActionType};
use super::types_10::Theme;
use super::types_11::DecisionNode;

/// Decision tree representation of a statute.
pub struct DecisionTree {
    pub(crate) graph: DiGraph<DecisionNode, EdgeLabel>,
    pub(crate) root: Option<NodeIndex>,
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
        let root = tree.graph.add_node(DecisionNode::Root {
            statute_id: statute.id.clone(),
            title: statute.title.clone(),
        });
        tree.root = Some(root);
        tree.node_map.insert(statute.id.clone(), root);
        let mut current = root;
        for (i, condition) in statute.preconditions.iter().enumerate() {
            let is_discretionary = matches!(condition, Condition::Custom { .. });
            let cond_node = tree.graph.add_node(DecisionNode::Condition {
                description: format_condition(condition),
                is_discretionary,
            });
            tree.graph.add_edge(current, cond_node, EdgeLabel::Proceeds);
            let void_node = tree.graph.add_node(DecisionNode::Outcome {
                description: format!("Condition {} not met", i + 1),
            });
            tree.graph.add_edge(cond_node, void_node, EdgeLabel::No);
            current = cond_node;
        }
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
            DecisionNode::Outcome { description } => format!("✓ {}", description),
            DecisionNode::Discretion { issue, hint } => match hint {
                Some(h) => format!("🔴 {} (hint: {})", issue, h),
                None => format!("🔴 {}", issue),
            },
        };
        output.push_str(&format!("{}{}{}\n", prefix, connector, node_text));
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
        for (level_idx, level_nodes) in levels.iter().enumerate() {
            if level_idx > 0 {
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
        svg.push_str(
            &format!(
                "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" style=\"background-color: {}\">\n",
                width, height, theme.background_color
            ),
        );
        svg.push_str("  <defs>\n");
        svg.push_str(
            "    <marker id=\"arrowhead\" markerWidth=\"10\" markerHeight=\"7\" refX=\"9\" refY=\"3.5\" orient=\"auto\">\n",
        );
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
        let rect_width = 180;
        let rect_height = 50;
        let rect_x = x.saturating_sub(rect_width / 2);
        svg.push_str(
            &format!(
                "  <rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"2\" rx=\"5\"/>\n",
                rect_x, y, rect_width, rect_height, color, theme.text_color
            ),
        );
        let display_text = if text.len() > 25 {
            format!("{}...", &text[..22])
        } else {
            text
        };
        svg.push_str(
            &format!(
                "  <text x=\"{}\" y=\"{}\" fill=\"{}\" text-anchor=\"middle\" font-size=\"12\">{}</text>\n",
                x, y + rect_height / 2 + 5, theme.text_color, display_text
            ),
        );
        let children: Vec<_> = self.graph.neighbors(idx).collect();
        if !children.is_empty() {
            *y_offset += 100;
            let child_spacing = rect_width * 2;
            let start_x = x.saturating_sub(child_spacing * (children.len().saturating_sub(1)) / 2);
            for (i, &child) in children.iter().enumerate() {
                let child_x = start_x + i * child_spacing;
                let child_y = *y_offset;
                svg.push_str(
                    &format!(
                        "  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"2\" marker-end=\"url(#arrowhead)\"/>\n",
                        x, y + rect_height, child_x, child_y, theme.link_color
                    ),
                );
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
        html.push_str(
            &format!(
                "        body {{ font-family: Arial, sans-serif; margin: 20px; background-color: {}; color: {}; }}\n",
                theme.background_color, theme.text_color
            ),
        );
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
        html.push_str(
            &format!(
                "        .link {{ fill: none; stroke: {}; stroke-width: 2px; transition: opacity 0.3s; }}\n",
                theme.link_color
            ),
        );
        html.push_str("        .link.hidden { opacity: 0.2; }\n");
        html.push_str("        .link-label { font-size: 10px; fill: #666; }\n");
        html.push_str(
            "        #details { position: fixed; top: 20px; right: 20px; background: rgba(255,255,255,0.95); padding: 15px; border-radius: 5px; box-shadow: 0 2px 10px rgba(0,0,0,0.2); max-width: 300px; display: none; }\n",
        );
        html.push_str("        #details.visible { display: block; }\n");
        html.push_str("        #details h3 { margin-top: 0; }\n");
        html.push_str("        .close-btn { float: right; cursor: pointer; font-size: 20px; }\n");
        html.push_str("    </style>\n</head>\n<body>\n");
        html.push_str("    <h1>Legal Decision Tree (Interactive)</h1>\n");
        html.push_str("    <p>Click on nodes to view details and drill down</p>\n");
        html.push_str("    <div id=\"tree\"></div>\n");
        html.push_str("    <div id=\"details\">\n");
        html.push_str(
            "        <span class=\"close-btn\" onclick=\"document.getElementById('details').classList.remove('visible')\">&times;</span>\n",
        );
        html.push_str("        <h3 id=\"detail-title\"></h3>\n");
        html.push_str("        <div id=\"detail-content\"></div>\n");
        html.push_str("    </div>\n");
        html.push_str("    <script>\n");
        html.push_str("const treeData = ");
        html.push_str(&self.to_d3_json());
        html.push_str(";\n");
        html.push_str("const width = 960;\nconst height = 600;\n");
        html.push_str(
            "const svg = d3.select(\"#tree\").append(\"svg\").attr(\"width\", width).attr(\"height\", height);\n",
        );
        html.push_str("const g = svg.append(\"g\").attr(\"transform\", \"translate(40,40)\");\n");
        html.push_str("const tree = d3.tree().size([height - 100, width - 200]);\n");
        html.push_str("const root = d3.hierarchy(treeData);\n");
        html.push_str("tree(root);\n");
        html.push_str(
            "const link = g.selectAll(\".link\").data(root.links()).enter().append(\"path\").attr(\"class\", \"link\").attr(\"d\", d3.linkHorizontal().x(function(d) { return d.y; }).y(function(d) { return d.x; }));\n",
        );
        html.push_str(
            "const node = g.selectAll(\".node\").data(root.descendants()).enter().append(\"g\").attr(\"class\", function(d) { return \"node \" + d.data.type; }).attr(\"transform\", function(d) { return \"translate(\" + d.y + \",\" + d.x + \")\"; });\n",
        );
        html.push_str("node.append(\"circle\").attr(\"r\", 6);\n");
        html.push_str(
            "node.append(\"text\").attr(\"dy\", 3).attr(\"x\", function(d) { return d.children ? -10 : 10; }).style(\"text-anchor\", function(d) { return d.children ? \"end\" : \"start\"; }).text(function(d) { return d.data.name; });\n",
        );
        html.push_str("node.on(\"click\", function(event, d) {\n");
        html.push_str("    const details = document.getElementById('details');\n");
        html.push_str("    const title = document.getElementById('detail-title');\n");
        html.push_str("    const content = document.getElementById('detail-content');\n");
        html.push_str("    title.textContent = d.data.name;\n");
        html.push_str(
            "    content.innerHTML = '<p><strong>Type:</strong> ' + d.data.type + '</p>';\n",
        );
        html.push_str("    if (d.children) {\n");
        html.push_str(
            "        content.innerHTML += '<p><strong>Children:</strong> ' + d.children.length + '</p>';\n",
        );
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
