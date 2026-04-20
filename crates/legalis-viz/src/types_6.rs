//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use petgraph::graph::{DiGraph, NodeIndex};
use serde::{Deserialize, Serialize};

use super::types::{AnnotationCategory, AnomalyType, VideoConfig, VisualizationRecommendation};
use super::types_3::{AnimatedGifConfig, Timeline, VisualRegressionTest, VisualizationType};
use super::types_4::DependencyGraph;
use super::types_7::{ExportFormat, PdfConfig};
use super::types_8::GeoPoint;
use super::types_9::PosterConfig;
use super::types_10::Theme;
use super::types_11::DecisionNode;
use super::types_12::{Anomaly, DecisionTree, EnforcementAction};

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
/// Automatic visualization selector based on data characteristics.
pub struct AutoVisualizationSelector {
    /// Minimum confidence threshold
    pub(crate) min_confidence: f32,
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
/// Anomaly detection for visualizations.
pub struct AnomalyDetector {
    /// Sensitivity (0.0-1.0, higher = more sensitive)
    pub(crate) sensitivity: f32,
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
        anomalies.extend(self.detect_orphaned_nodes(tree));
        anomalies.extend(self.detect_deep_paths(tree));
        anomalies.extend(self.detect_missing_outcomes(tree));
        anomalies.extend(self.detect_cycles(tree));
        anomalies
    }
    /// Detects anomalies in a dependency graph.
    pub fn detect_in_graph(&self, graph: &DependencyGraph) -> Vec<Anomaly> {
        let mut anomalies = Vec::new();
        anomalies.extend(self.detect_isolated_statutes(graph));
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
            if !has_incoming
                && !is_root
                && let Some(node) = tree.graph.node_weight(node_idx)
            {
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
            if incoming == 0
                && outgoing == 0
                && let Some(statute_id) = graph.graph.node_weight(node_idx)
            {
                anomalies.push(Anomaly {
                    anomaly_type: AnomalyType::IsolatedNode,
                    severity: 0.6,
                    description: format!("Isolated statute: {}", statute_id),
                    location: statute_id.clone(),
                    suggestion: "Consider if this statute should have dependencies".to_string(),
                });
            }
        }
        anomalies
    }
    fn detect_asymmetric_dependencies(&self, graph: &DependencyGraph) -> Vec<Anomaly> {
        let mut anomalies = Vec::new();
        for edge in graph.graph.edge_indices() {
            if let Some((source, target)) = graph.graph.edge_endpoints(edge) {
                let has_reverse = graph.graph.edges_connecting(target, source).count() > 0;
                if has_reverse
                    && let (Some(from_id), Some(to_id)) = (
                        graph.graph.node_weight(source),
                        graph.graph.node_weight(target),
                    )
                {
                    anomalies.push(Anomaly {
                        anomaly_type: AnomalyType::BidirectionalDependency,
                        severity: 0.75,
                        description: format!("Bidirectional dependency: {} <-> {}", from_id, to_id),
                        location: format!("{}-{}", from_id, to_id),
                        suggestion: "Review if bidirectional dependency is intentional".to_string(),
                    });
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
/// Geographic coordinate (latitude, longitude).
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GeoCoordinate {
    /// Latitude
    pub lat: f64,
    /// Longitude
    pub lng: f64,
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
/// Enforcement action tracking visualizer.
pub struct EnforcementActionTracker {
    /// Tracker title
    pub(crate) title: String,
    /// WebSocket URL for updates
    pub(crate) ws_url: String,
    /// Theme
    pub(crate) theme: Theme,
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
        html.push_str(
            &format!(
                "        body {{ background-color: {}; color: {}; font-family: Arial, sans-serif; margin: 0; padding: 0; }}\n",
                self.theme.background_color, self.theme.text_color
            ),
        );
        html.push_str(
            "        .header { background-color: #c0392b; color: white; padding: 30px; }\n",
        );
        html.push_str("        .header h1 { margin: 0; }\n");
        html.push_str(
            "        .stats { display: flex; justify-content: space-around; background-color: white; padding: 20px; margin: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }\n",
        );
        html.push_str("        .stat { text-align: center; }\n");
        html.push_str(
            "        .stat-value { font-size: 2.5em; font-weight: bold; color: #2c3e50; }\n",
        );
        html.push_str("        .stat-label { color: #7f8c8d; margin-top: 5px; }\n");
        html.push_str("        .container { max-width: 1200px; margin: 0 auto; padding: 20px; }\n");
        html.push_str(
            "        .action-card { background-color: white; border-radius: 8px; padding: 20px; margin: 15px 0; box-shadow: 0 2px 8px rgba(0,0,0,0.1); }\n",
        );
        html.push_str(
            "        .action-header { display: flex; justify-content: space-between; align-items: start; margin-bottom: 15px; }\n",
        );
        html.push_str(
            "        .action-entity { font-size: 1.4em; font-weight: bold; color: #2c3e50; }\n",
        );
        html.push_str(
            "        .action-type { padding: 6px 14px; border-radius: 20px; font-size: 0.85em; font-weight: bold; }\n",
        );
        html.push_str("        .type-fine { background-color: #f39c12; color: white; }\n");
        html.push_str("        .type-warning { background-color: #e67e22; color: white; }\n");
        html.push_str("        .type-suspension { background-color: #e74c3c; color: white; }\n");
        html.push_str("        .type-settlement { background-color: #3498db; color: white; }\n");
        html.push_str("        .type-investigation { background-color: #9b59b6; color: white; }\n");
        html.push_str(
            "        .action-details { display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 15px; margin-top: 15px; }\n",
        );
        html.push_str("        .detail-item { }\n");
        html.push_str(
            "        .detail-label { font-weight: bold; color: #7f8c8d; font-size: 0.85em; }\n",
        );
        html.push_str("        .detail-value { color: #2c3e50; margin-top: 3px; }\n");
        html.push_str(
            "        .action-violations { background-color: #fff5f5; border-left: 4px solid #e74c3c; padding: 10px; margin-top: 10px; }\n",
        );
        html.push_str(
            "        .violations-title { font-weight: bold; color: #c0392b; margin-bottom: 5px; }\n",
        );
        html.push_str("    </style>\n</head>\n<body>\n");
        html.push_str("    <div class=\"header\">\n");
        html.push_str(&format!("        <h1>{}</h1>\n", self.title));
        html.push_str("    </div>\n");
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
        html.push_str(
            "            <div class=\"detail-item\"><div class=\"detail-label\">Agency</div><div class=\"detail-value\">${data.agency}</div></div>\n",
        );
        html.push_str(
            "            <div class=\"detail-item\"><div class=\"detail-label\">Date</div><div class=\"detail-value\">${data.action_date}</div></div>\n",
        );
        html.push_str(
            "            <div class=\"detail-item\"><div class=\"detail-label\">Status</div><div class=\"detail-value\">${data.status}</div></div>\n",
        );
        html.push_str(
            "            ${data.fine_amount ? '<div class=\"detail-item\"><div class=\"detail-label\">Fine Amount</div><div class=\"detail-value\">$' + data.fine_amount.toLocaleString() + '</div></div>' : ''}\n",
        );
        html.push_str("        </div>\n");
        html.push_str(
            "        ${data.violations && data.violations.length > 0 ? '<div class=\"action-violations\"><div class=\"violations-title\">Violations:</div><ul style=\"margin: 5px 0; padding-left: 20px;\">' + data.violations.map(v => '<li>' + v + '</li>').join('') + '</ul></div>' : ''}\n",
        );
        html.push_str("    `;\n");
        html.push_str("    container.insertBefore(card, container.firstChild);\n");
        html.push_str("    // Update stats\n");
        html.push_str("    const totalActions = document.getElementById('total-actions');\n");
        html.push_str("    totalActions.textContent = parseInt(totalActions.textContent) + 1;\n");
        html.push_str("    if (data.fine_amount) {\n");
        html.push_str("        const totalFines = document.getElementById('total-fines');\n");
        html.push_str(
            "        const currentValue = parseFloat(totalFines.textContent.replace('$', '').replace('M', '')) * 1000000;\n",
        );
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
/// Advanced export handler for various formats.
#[derive(Debug, Clone)]
pub struct AdvancedExporter {
    pub(crate) theme: Theme,
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
        let mut svg = tree.to_svg_with_theme(&self.theme);
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
        let mut vectorized = svg;
        let metadata = format!(
            r#"<!-- PDF Export: {}x{}mm @ {}dpi -->"#,
            config.width, config.height, config.dpi
        );
        vectorized = vectorized.replace("<svg", &format!("{}\n<svg", metadata));
        vectorized
    }
    /// Exports to poster size.
    pub fn to_poster(&self, tree: &DecisionTree, config: PosterConfig) -> String {
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
        let scale_factor = config.dpi as f32 / 96.0;
        let pixel_width = (config.width as f32 * scale_factor * 3.7795) as usize;
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
/// Edge labels in decision graphs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EdgeLabel {
    Yes,
    No,
    Maybe,
    Proceeds,
}
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
/// AI annotation generator for visualizations.
pub struct AIAnnotationGenerator {
    /// Enable complexity analysis
    pub(crate) enable_complexity: bool,
    /// Enable pattern detection
    pub(crate) enable_patterns: bool,
    /// Minimum importance threshold
    pub(crate) min_importance: f32,
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
        if self.enable_complexity {
            annotations.extend(self.analyze_tree_complexity(tree));
        }
        if self.enable_patterns {
            annotations.extend(self.detect_tree_patterns(tree));
        }
        annotations.extend(self.find_critical_paths(tree));
        annotations.retain(|a| a.importance >= self.min_importance);
        annotations
    }
    /// Generates annotations for a dependency graph.
    pub fn generate_for_graph(&self, graph: &DependencyGraph) -> Vec<AIAnnotation> {
        let mut annotations = Vec::new();
        if self.enable_complexity {
            annotations.extend(self.analyze_graph_hubs(graph));
        }
        annotations.extend(self.detect_dependency_cycles(graph));
        annotations.retain(|a| a.importance >= self.min_importance);
        annotations
    }
    fn analyze_tree_complexity(&self, tree: &DecisionTree) -> Vec<AIAnnotation> {
        let mut annotations = Vec::new();
        for node_idx in tree.graph.node_indices() {
            let out_degree = tree.graph.neighbors(node_idx).count();
            if out_degree > 5
                && let Some(_node) = tree.graph.node_weight(node_idx)
            {
                annotations.push(AIAnnotation {
                    target_id: format!("node-{}", node_idx.index()),
                    text: format!("High complexity: {} outgoing paths", out_degree),
                    importance: 0.8,
                    category: AnnotationCategory::Complexity,
                    position: None,
                });
            }
        }
        annotations
    }
    fn detect_tree_patterns(&self, tree: &DecisionTree) -> Vec<AIAnnotation> {
        let mut annotations = Vec::new();
        let mut discretion_chains = 0;
        for node_idx in tree.graph.node_indices() {
            if let Some(node) = tree.graph.node_weight(node_idx)
                && matches!(node, DecisionNode::Discretion { .. })
            {
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
        if discretion_chains > 3 {
            annotations
                .push(AIAnnotation {
                    target_id: "root".to_string(),
                    text: format!(
                        "Pattern detected: {} chains of discretionary decisions may indicate high interpretive complexity",
                        discretion_chains
                    ),
                    importance: 0.75,
                    category: AnnotationCategory::Pattern,
                    position: None,
                });
        }
        annotations
    }
    fn find_critical_paths(&self, tree: &DecisionTree) -> Vec<AIAnnotation> {
        let mut annotations = Vec::new();
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
        for node_idx in graph.graph.node_indices() {
            let out_degree = graph.graph.neighbors(node_idx).count();
            if out_degree > 5
                && let Some(statute_id) = graph.graph.node_weight(node_idx)
            {
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
        annotations
    }
    fn detect_dependency_cycles(&self, graph: &DependencyGraph) -> Vec<AIAnnotation> {
        let mut annotations = Vec::new();
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
/// Visualizer for evaluation audit trails.
pub struct AuditTrailVisualizer {
    pub(crate) theme: Theme,
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
            html.push_str(
                "<thead><tr><th>#</th><th>Condition</th><th>Result</th><th>Duration (μs)</th></tr></thead>",
            );
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
