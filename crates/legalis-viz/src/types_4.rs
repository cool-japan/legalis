//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use petgraph::dot::{Config, Dot};
use petgraph::graph::{DiGraph, NodeIndex};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(feature = "png-export")]
use super::functions::VizResult;
#[cfg(feature = "png-export")]
use super::functions::svg_to_png;
use super::functions::{base64_encode, format_change_type};
use super::types::ComplianceStatus;
use super::types_3::OfflineConfig;
use super::types_5::ResponsiveScalingConfig;
use super::types_6::{QuizQuestion, TouchGestureConfig};
use super::types_7::AmendmentImpact;
use super::types_8::ConceptRelationshipGraph;
use super::types_10::{KeyPlayer, LayoutConfig, Theme, TimelineStoryEvent};
use super::types_11::DecisionNode;
use super::types_12::DecisionTree;

/// Document embedding support for various formats.
pub struct DocumentEmbedder {
    pub(crate) theme: Theme,
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
/// Visualizer for statute differences (comparing versions).
pub struct StatuteDiffVisualizer {
    pub(crate) theme: Theme,
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
    pub icons: Vec<(String, String, String)>,
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
/// GeoJSON geometry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoJsonGeometry {
    /// Geometry type (Polygon, MultiPolygon, Point, etc.)
    #[serde(rename = "type")]
    pub geometry_type: String,
    /// Coordinates (format depends on geometry type)
    pub coordinates: serde_json::Value,
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
        html.push_str(
            "    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no\">\n",
        );
        html.push_str(&self.pwa_config.to_html_meta_tags());
        html.push_str("    <title>Mobile Legal Visualization</title>\n");
        html.push_str("    <style>\n");
        html.push_str("        * { box-sizing: border-box; margin: 0; padding: 0; }\n");
        html.push_str(
            "        body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif; overflow-x: hidden; }\n",
        );
        html.push_str(
            "        .viz-container { width: 100%; height: 100vh; overflow: hidden; touch-action: none; }\n",
        );
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
/// Statute dependency graph.
pub struct DependencyGraph {
    pub(crate) graph: DiGraph<String, String>,
    pub(crate) statute_map: HashMap<String, NodeIndex>,
    pub(crate) layout_config: LayoutConfig,
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
        svg.push_str(
            &format!(
                "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" style=\"background-color: {}\">\n",
                width, height, theme.background_color
            ),
        );
        svg.push_str("  <defs>\n");
        svg.push_str(
            "    <marker id=\"arrow\" markerWidth=\"10\" markerHeight=\"10\" refX=\"9\" refY=\"3\" orient=\"auto\" markerUnits=\"strokeWidth\">\n",
        );
        svg.push_str(&format!(
            "      <path d=\"M0,0 L0,6 L9,3 z\" fill=\"{}\" />\n",
            theme.link_color
        ));
        svg.push_str("    </marker>\n");
        svg.push_str("  </defs>\n");
        let node_radius = 30;
        let cols = (self.node_count() as f64).sqrt().ceil() as usize;
        let spacing_x = width / (cols + 1);
        let spacing_y = height / ((self.node_count() / cols) + 2);
        let mut node_positions: std::collections::HashMap<NodeIndex, (usize, usize)> =
            std::collections::HashMap::new();
        for (i, node_idx) in self.graph.node_indices().enumerate() {
            let col = i % cols;
            let row = i / cols;
            let x = spacing_x * (col + 1);
            let y = spacing_y * (row + 1);
            node_positions.insert(node_idx, (x, y));
        }
        for edge in self.graph.edge_indices() {
            if let Some((source, target)) = self.graph.edge_endpoints(edge)
                && let (Some(&(x1, y1)), Some(&(x2, y2))) =
                    (node_positions.get(&source), node_positions.get(&target))
            {
                svg.push_str(
                    &format!(
                        "  <line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"2\" marker-end=\"url(#arrow)\"/>\n",
                        x1, y1, x2, y2, theme.link_color
                    ),
                );
                let label = &self.graph[edge];
                let mid_x = (x1 + x2) / 2;
                let mid_y = (y1 + y2) / 2;
                svg.push_str(
                    &format!(
                        "  <text x=\"{}\" y=\"{}\" font-size=\"10\" fill=\"{}\" text-anchor=\"middle\">{}</text>\n",
                        mid_x, mid_y.saturating_sub(5), theme.text_color, label
                    ),
                );
            }
        }
        for node_idx in self.graph.node_indices() {
            if let Some(&(x, y)) = node_positions.get(&node_idx) {
                let statute_id = &self.graph[node_idx];
                svg.push_str(
                    &format!(
                        "  <circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"2\"/>\n",
                        x, y, node_radius, theme.condition_color, theme.text_color
                    ),
                );
                let display_id = if statute_id.len() > 12 {
                    format!("{}...", &statute_id[..9])
                } else {
                    statute_id.clone()
                };
                svg.push_str(
                    &format!(
                        "  <text x=\"{}\" y=\"{}\" font-size=\"10\" fill=\"{}\" text-anchor=\"middle\">{}</text>\n",
                        x, y + 4, theme.text_color, display_id
                    ),
                );
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
        html.push_str(
            "const svg = d3.select(\"#graph\").append(\"svg\").attr(\"width\", width).attr(\"height\", height);\n",
        );
        html.push_str("const simulation = d3.forceSimulation(graphData.nodes)\n");
        html.push_str(
            &format!(
                "    .force(\"link\", d3.forceLink(graphData.links).id(function(d) {{ return d.id; }}).distance({}))\n",
                distance
            ),
        );
        html.push_str("    .force(\"charge\", d3.forceManyBody().strength(-300))\n");
        html.push_str("    .force(\"center\", d3.forceCenter(width / 2, height / 2));\n");
        html.push_str(
            "const link = svg.append(\"g\").attr(\"class\", \"links\").selectAll(\"line\").data(graphData.links).enter().append(\"line\").attr(\"stroke-width\", 2);\n",
        );
        html.push_str(
            "const linkLabel = svg.append(\"g\").attr(\"class\", \"link-labels\").selectAll(\"text\").data(graphData.links).enter().append(\"text\").attr(\"class\", \"link-label\").attr(\"dy\", -5).text(function(d) { return d.label; });\n",
        );
        html.push_str(
            "const node = svg.append(\"g\").attr(\"class\", \"nodes\").selectAll(\"circle\").data(graphData.nodes).enter().append(\"circle\").attr(\"r\", 10);\n",
        );
        html.push_str(
            "const label = svg.append(\"g\").selectAll(\"text\").data(graphData.nodes).enter().append(\"text\").text(function(d) { return d.id; }).attr(\"dx\", 12).attr(\"dy\", 4);\n",
        );
        html.push_str("simulation.on(\"tick\", function() {\n");
        html.push_str(
            "    link.attr(\"x1\", function(d) { return d.source.x; }).attr(\"y1\", function(d) { return d.source.y; }).attr(\"x2\", function(d) { return d.target.x; }).attr(\"y2\", function(d) { return d.target.y; });\n",
        );
        html.push_str(
            "    linkLabel.attr(\"x\", function(d) { return (d.source.x + d.target.x) / 2; }).attr(\"y\", function(d) { return (d.source.y + d.target.y) / 2; });\n",
        );
        html.push_str(
            "    node.attr(\"cx\", function(d) { return d.x; }).attr(\"cy\", function(d) { return d.y; });\n",
        );
        html.push_str(
            "    label.attr(\"x\", function(d) { return d.x; }).attr(\"y\", function(d) { return d.y; });\n",
        );
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
    pub(crate) theme: PropTypes.oneOf(['light', 'dark', 'high-contrast', 'colorblind-friendly']),\n\
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
/// Amendment impact analysis visualizer.
#[derive(Debug, Clone)]
pub struct AmendmentImpactAnalysis {
    /// Analysis title
    pub title: String,
    /// Amendment events with impact metrics
    pub amendments: Vec<AmendmentImpact>,
    /// Theme
    pub theme: Theme,
}
impl AmendmentImpactAnalysis {
    /// Creates a new amendment impact analysis.
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            amendments: Vec::new(),
            theme: Theme::light(),
        }
    }
    /// Adds an amendment impact.
    pub fn add_amendment(&mut self, amendment: AmendmentImpact) {
        self.amendments.push(amendment);
    }
    /// Sets the theme.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
    /// Generates HTML impact analysis dashboard.
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
        html.push_str(
            &format!(
                "        body {{ margin: 20px; background-color: {}; color: {}; font-family: 'Segoe UI', Arial, sans-serif; }}\n",
                self.theme.background_color, self.theme.text_color
            ),
        );
        html.push_str("        .dashboard { max-width: 1200px; margin: 0 auto; }\n");
        html.push_str(
            "        .metrics { display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 20px; margin: 30px 0; }\n",
        );
        html.push_str(
            "        .metric-card { background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.1); }\n",
        );
        html.push_str(
            "        .metric-value { font-size: 2.5em; font-weight: bold; color: #3498db; margin: 10px 0; }\n",
        );
        html.push_str("        .metric-label { color: #7f8c8d; font-size: 0.9em; }\n");
        html.push_str(
            "        .amendments-table { width: 100%; border-collapse: collapse; background: white; border-radius: 8px; overflow: hidden; box-shadow: 0 2px 8px rgba(0,0,0,0.1); }\n",
        );
        html.push_str(
            "        .amendments-table th, .amendments-table td { padding: 12px; text-align: left; border-bottom: 1px solid #ecf0f1; }\n",
        );
        html.push_str(
            "        .amendments-table th { background-color: #34495e; color: white; font-weight: bold; }\n",
        );
        html.push_str("        .amendments-table tr:hover { background-color: #ecf0f1; }\n");
        html.push_str(
            "        .severity-badge { padding: 4px 12px; border-radius: 4px; font-size: 0.85em; font-weight: bold; }\n",
        );
        html.push_str("        .severity-low { background-color: #27ae60; color: white; }\n");
        html.push_str("        .severity-medium { background-color: #f39c12; color: white; }\n");
        html.push_str("        .severity-high { background-color: #e74c3c; color: white; }\n");
        html.push_str("        #impact-chart { margin: 30px 0; }\n");
        html.push_str("    </style>\n");
        html.push_str("</head>\n<body>\n");
        html.push_str("    <div class=\"dashboard\">\n");
        html.push_str(&format!("        <h1>{}</h1>\n", self.title));
        let total_amendments = self.amendments.len();
        let total_sections: usize = self.amendments.iter().map(|a| a.sections_affected).sum();
        let total_downstream: usize = self.amendments.iter().map(|a| a.downstream_statutes).sum();
        let avg_severity = if total_amendments > 0 {
            self.amendments.iter().map(|a| a.severity).sum::<f64>() / total_amendments as f64
        } else {
            0.0
        };
        html.push_str("        <div class=\"metrics\">\n");
        html.push_str("            <div class=\"metric-card\">\n");
        html.push_str(&format!(
            "                <div class=\"metric-value\">{}</div>\n",
            total_amendments
        ));
        html.push_str("                <div class=\"metric-label\">Total Amendments</div>\n");
        html.push_str("            </div>\n");
        html.push_str("            <div class=\"metric-card\">\n");
        html.push_str(&format!(
            "                <div class=\"metric-value\">{}</div>\n",
            total_sections
        ));
        html.push_str("                <div class=\"metric-label\">Sections Affected</div>\n");
        html.push_str("            </div>\n");
        html.push_str("            <div class=\"metric-card\">\n");
        html.push_str(&format!(
            "                <div class=\"metric-value\">{}</div>\n",
            total_downstream
        ));
        html.push_str("                <div class=\"metric-label\">Downstream Statutes</div>\n");
        html.push_str("            </div>\n");
        html.push_str("            <div class=\"metric-card\">\n");
        html.push_str(&format!(
            "                <div class=\"metric-value\">{:.2}</div>\n",
            avg_severity
        ));
        html.push_str("                <div class=\"metric-label\">Avg Severity</div>\n");
        html.push_str("            </div>\n");
        html.push_str("        </div>\n");
        html.push_str("        <div id=\"impact-chart\"></div>\n");
        html.push_str("        <h2>Amendment Details</h2>\n");
        html.push_str("        <table class=\"amendments-table\">\n");
        html.push_str("            <thead>\n");
        html.push_str("                <tr>\n");
        html.push_str("                    <th>Date</th>\n");
        html.push_str("                    <th>Statute</th>\n");
        html.push_str("                    <th>Description</th>\n");
        html.push_str("                    <th>Sections</th>\n");
        html.push_str("                    <th>Downstream</th>\n");
        html.push_str("                    <th>Severity</th>\n");
        html.push_str("                </tr>\n");
        html.push_str("            </thead>\n");
        html.push_str("            <tbody>\n");
        for amendment in &self.amendments {
            let severity_class = if amendment.severity < 0.33 {
                "severity-low"
            } else if amendment.severity < 0.67 {
                "severity-medium"
            } else {
                "severity-high"
            };
            html.push_str("                <tr>\n");
            html.push_str(&format!(
                "                    <td>{}</td>\n",
                amendment.date.split('T').next().unwrap_or(&amendment.date)
            ));
            html.push_str(&format!(
                "                    <td>{}</td>\n",
                amendment.statute_name
            ));
            html.push_str(&format!(
                "                    <td>{}</td>\n",
                amendment.description
            ));
            html.push_str(&format!(
                "                    <td>{}</td>\n",
                amendment.sections_affected
            ));
            html.push_str(&format!(
                "                    <td>{}</td>\n",
                amendment.downstream_statutes
            ));
            html.push_str(&format!(
                "                    <td><span class=\"severity-badge {}\">{:.2}</span></td>\n",
                severity_class, amendment.severity
            ));
            html.push_str("                </tr>\n");
        }
        html.push_str("            </tbody>\n");
        html.push_str("        </table>\n");
        html.push_str("    </div>\n");
        html.push_str("    <script>\n");
        html.push_str(&format!(
            "        const data = {};\n",
            serde_json::to_string(&self.amendments).expect("invariant: amendments is serializable")
        ));
        html.push_str("        \n");
        html.push_str("        const margin = {top: 40, right: 40, bottom: 60, left: 60};\n");
        html.push_str("        const width = 1100 - margin.left - margin.right;\n");
        html.push_str("        const height = 400 - margin.top - margin.bottom;\n");
        html.push_str("        \n");
        html.push_str("        const svg = d3.select('#impact-chart')\n");
        html.push_str("            .append('svg')\n");
        html.push_str("            .attr('width', width + margin.left + margin.right)\n");
        html.push_str("            .attr('height', height + margin.top + margin.bottom)\n");
        html.push_str("            .append('g')\n");
        html.push_str(
            "            .attr('transform', `translate(${margin.left},${margin.top})`);\n",
        );
        html.push_str("        \n");
        html.push_str("        // Parse dates\n");
        html.push_str("        data.forEach(d => { d.parsed_date = new Date(d.date); });\n");
        html.push_str("        \n");
        html.push_str("        // Scales\n");
        html.push_str("        const x = d3.scaleTime()\n");
        html.push_str("            .domain(d3.extent(data, d => d.parsed_date))\n");
        html.push_str("            .range([0, width]);\n");
        html.push_str("        \n");
        html.push_str("        const y = d3.scaleLinear()\n");
        html.push_str("            .domain([0, d3.max(data, d => d.sections_affected)])\n");
        html.push_str("            .range([height, 0]);\n");
        html.push_str("        \n");
        html.push_str("        const colorScale = d3.scaleLinear()\n");
        html.push_str("            .domain([0, 0.5, 1])\n");
        html.push_str("            .range(['#27ae60', '#f39c12', '#e74c3c']);\n");
        html.push_str("        \n");
        html.push_str("        // Axes\n");
        html.push_str("        svg.append('g')\n");
        html.push_str("            .attr('transform', `translate(0,${height})`)\n");
        html.push_str("            .call(d3.axisBottom(x));\n");
        html.push_str("        \n");
        html.push_str("        svg.append('g')\n");
        html.push_str("            .call(d3.axisLeft(y));\n");
        html.push_str("        \n");
        html.push_str("        // Axis labels\n");
        html.push_str("        svg.append('text')\n");
        html.push_str("            .attr('x', width / 2)\n");
        html.push_str("            .attr('y', height + 50)\n");
        html.push_str("            .attr('text-anchor', 'middle')\n");
        html.push_str("            .text('Time');\n");
        html.push_str("        \n");
        html.push_str("        svg.append('text')\n");
        html.push_str("            .attr('transform', 'rotate(-90)')\n");
        html.push_str("            .attr('x', -height / 2)\n");
        html.push_str("            .attr('y', -50)\n");
        html.push_str("            .attr('text-anchor', 'middle')\n");
        html.push_str("            .text('Sections Affected');\n");
        html.push_str("        \n");
        html.push_str("        // Plot bars\n");
        html.push_str("        svg.selectAll('.bar')\n");
        html.push_str("            .data(data)\n");
        html.push_str("            .enter()\n");
        html.push_str("            .append('rect')\n");
        html.push_str("            .attr('class', 'bar')\n");
        html.push_str("            .attr('x', d => x(d.parsed_date) - 10)\n");
        html.push_str("            .attr('y', d => y(d.sections_affected))\n");
        html.push_str("            .attr('width', 20)\n");
        html.push_str("            .attr('height', d => height - y(d.sections_affected))\n");
        html.push_str("            .attr('fill', d => colorScale(d.severity));\n");
        html.push_str("    </script>\n");
        html.push_str("</body>\n</html>");
        html
    }
    /// Generates summary report in text format.
    pub fn to_text_report(&self) -> String {
        let mut report = String::new();
        report.push_str(&format!("{}\n", self.title));
        report.push_str(&"=".repeat(self.title.len()));
        report.push_str("\n\n");
        report.push_str(&format!("Total Amendments: {}\n", self.amendments.len()));
        report.push_str(&format!(
            "Total Sections Affected: {}\n",
            self.amendments
                .iter()
                .map(|a| a.sections_affected)
                .sum::<usize>()
        ));
        report.push_str(&format!(
            "Total Downstream Statutes: {}\n\n",
            self.amendments
                .iter()
                .map(|a| a.downstream_statutes)
                .sum::<usize>()
        ));
        report.push_str("Detailed Breakdown:\n");
        report.push_str("-".repeat(80).as_str());
        report.push('\n');
        for amendment in &self.amendments {
            report.push_str(&format!(
                "\n[{}] {}\n",
                amendment.date.split('T').next().unwrap_or(&amendment.date),
                amendment.statute_name
            ));
            report.push_str(&format!("  {}\n", amendment.description));
            report.push_str(&format!(
                "  Sections: {}, Downstream: {}, Severity: {:.2}\n",
                amendment.sections_affected, amendment.downstream_statutes, amendment.severity
            ));
        }
        report
    }
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
        let mut graph_clone = graph.clone();
        graph_clone.theme = self.theme.clone();
        let mut html = graph_clone.to_html();
        html = html
            .replace(
                "</style>",
                "        .ontology-layer { opacity: 0.9; }\n        .ontology-root { font-weight: bold; }\n    </style>",
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
/// Cypher query exporter for Neo4j graph database.
#[derive(Debug, Clone)]
pub struct CypherExporter {
    /// Include CREATE INDEX statements
    pub include_indexes: bool,
    /// Use MERGE instead of CREATE
    pub use_merge: bool,
}
impl CypherExporter {
    /// Creates a new Cypher exporter.
    pub fn new() -> Self {
        Self {
            include_indexes: true,
            use_merge: false,
        }
    }
    /// Sets whether to include index statements.
    pub fn with_indexes(mut self, include: bool) -> Self {
        self.include_indexes = include;
        self
    }
    /// Sets whether to use MERGE instead of CREATE.
    pub fn with_merge(mut self, use_merge: bool) -> Self {
        self.use_merge = use_merge;
        self
    }
    /// Exports a dependency graph to Cypher queries.
    pub fn export_graph(&self, graph: &DependencyGraph) -> String {
        let mut cypher = String::new();
        cypher.push_str("// Neo4j Cypher Query Export\n");
        cypher.push_str("// Generated by legalis-viz\n\n");
        if self.include_indexes {
            cypher.push_str("// Create indexes\n");
            cypher.push_str("CREATE INDEX statute_id IF NOT EXISTS FOR (s:Statute) ON (s.id);\n");
            cypher.push_str(
                "CREATE INDEX statute_name IF NOT EXISTS FOR (s:Statute) ON (s.name);\n\n",
            );
        }
        let node_cmd = if self.use_merge { "MERGE" } else { "CREATE" };
        cypher.push_str("// Create statute nodes\n");
        for node_idx in graph.graph.node_indices() {
            let statute_id = &graph.graph[node_idx];
            cypher.push_str(&format!(
                "{} (s_{}:Statute {{id: '{}', name: '{}', type: 'statute'}})\n",
                node_cmd,
                statute_id.replace('-', "_"),
                statute_id,
                statute_id
            ));
        }
        cypher.push_str("\n// Create relationships\n");
        for edge in graph.graph.edge_indices() {
            if let Some((source, target)) = graph.graph.edge_endpoints(edge) {
                let source_id = &graph.graph[source];
                let target_id = &graph.graph[target];
                cypher.push_str(&format!(
                    "{} (s_{})-[:DEPENDS_ON {{type: 'dependency'}}]->(s_{})\n",
                    node_cmd,
                    source_id.replace('-', "_"),
                    target_id.replace('-', "_")
                ));
            }
        }
        cypher.push_str("\n// Return created nodes\n");
        cypher.push_str("MATCH (s:Statute) RETURN s;\n");
        cypher
    }
    /// Exports a legal concept graph to Cypher queries.
    pub fn export_concept_graph(&self, graph: &ConceptRelationshipGraph) -> String {
        let mut cypher = String::new();
        cypher.push_str("// Neo4j Cypher Query Export - Legal Concepts\n");
        cypher.push_str("// Generated by legalis-viz\n\n");
        if self.include_indexes {
            cypher.push_str("// Create indexes\n");
            cypher.push_str("CREATE INDEX concept_id IF NOT EXISTS FOR (c:Concept) ON (c.id);\n");
            cypher.push_str(
                "CREATE INDEX concept_name IF NOT EXISTS FOR (c:Concept) ON (c.name);\n\n",
            );
        }
        let node_cmd = if self.use_merge { "MERGE" } else { "CREATE" };
        cypher.push_str("// Create concept nodes\n");
        for concept in &graph.concepts {
            cypher.push_str(&format!(
                "{} (c_{}:Concept {{id: '{}', name: '{}', description: '{}', category: '{}'}});\n",
                node_cmd,
                concept.id.replace('-', "_"),
                concept.id,
                concept.name,
                concept.description,
                concept.category
            ));
        }
        cypher.push_str("\n// Create relationships\n");
        for rel in &graph.relationships {
            let rel_type = format!("{:?}", rel.relation_type).to_uppercase();
            cypher.push_str(&format!(
                "{} (c_{})-[:{}{{strength: {}, description: '{}'}}]->(c_{});\n",
                node_cmd,
                rel.from_id.replace('-', "_"),
                rel_type,
                rel.strength,
                rel.description,
                rel.to_id.replace('-', "_")
            ));
        }
        cypher.push_str("\n// Return created concepts\n");
        cypher.push_str("MATCH (c:Concept) RETURN c;\n");
        cypher
    }
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
