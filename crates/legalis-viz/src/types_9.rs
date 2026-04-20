//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use petgraph::graph::NodeIndex;
use serde::{Deserialize, Serialize};

use super::types::{LookingGlassConfig, StatuteChangeEvent};
use super::types_3::{RegulatoryChange, Timeline};
use super::types_4::DependencyGraph;
use super::types_5::TimelineEvent;
use super::types_6::{GeoCoordinate, ImpactSeverity};
use super::types_7::CollaborativeUser;
use super::types_8::InteractiveConfig;
use super::types_10::Theme;
use super::types_11::ThreeDConfig;
use super::types_12::DecisionTree;

/// 3D visualizer for dependency graphs and timelines using WebGL
pub struct ThreeDVisualizer {
    pub(crate) theme: Theme,
    pub(crate) config: ThreeDConfig,
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
        let mut nodes = Vec::new();
        for idx in graph.graph.node_indices() {
            if let Some(statute_id) = graph.graph.node_weight(idx) {
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
                return depth;
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
                    } => {
                        format!("{}: Amended {} - {}", date, statute_id, description)
                    }
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
        html.push_str("<div class=\"viz-3d-container\">\n");
        html.push_str("<div class=\"controls-panel\">\n");
        html.push_str(&format!("<h2>3D {} Visualization</h2>\n", title));
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
        html.push_str("<div class=\"info-panel\">\n");
        html.push_str("<div id=\"node-info\">Hover over nodes for details</div>\n");
        html.push_str(&format!("<div>Nodes: {}</div>\n", nodes.len()));
        html.push_str(&format!("<div>Edges: {}</div>\n", edges.len()));
        html.push_str("</div>\n");
        html.push_str("</div>\n");
        html.push_str("<div id=\"canvas-container\"></div>\n");
        html.push_str("</div>\n");
        html.push_str(
            "<script src=\"https://cdnjs.cloudflare.com/ajax/libs/three.js/r128/three.min.js\"></script>\n",
        );
        if self.config.enable_vr || self.config.enable_ar {
            html.push_str(
                "<script src=\"https://cdn.jsdelivr.net/npm/three@0.128.0/examples/js/webxr/VRButton.js\"></script>\n",
            );
        }
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
        js.push_str("const nodes = [\n");
        for (label, depth) in nodes {
            js.push_str(&format!(
                "    {{ label: '{}', depth: {} }},\n",
                label.replace('\'', "\\'"),
                depth
            ));
        }
        js.push_str("];\n\n");
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
/// Interactive visualizer for decision trees and dependency graphs
pub struct InteractiveVisualizer {
    pub(crate) theme: Theme,
    pub(crate) config: InteractiveConfig,
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
        html.push_str("<div class=\"viz-container\">\n");
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
        html.push_str("<div class=\"viz-main\">\n");
        html.push_str("<div id=\"svg-container\" class=\"svg-container\">\n");
        html.push_str(&svg);
        html.push_str("</div>\n");
        if self.config.enable_minimap {
            html.push_str(
                &format!(
                    "<div id=\"minimap\" class=\"minimap\" style=\"width: {}px; height: {}px;\"></div>\n",
                    self.config.minimap_size.0, self.config.minimap_size.1
                ),
            );
        }
        html.push_str("</div>\n");
        html.push_str("</div>\n");
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
            width: 610,
            height: 914,
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
/// Regulatory change monitoring visualizer.
pub struct RegulatoryChangeMonitor {
    /// Monitor title
    pub(crate) title: String,
    /// WebSocket URL for updates
    pub(crate) ws_url: String,
    /// Theme
    pub(crate) theme: Theme,
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
        html.push_str(
            &format!(
                "        body {{ background-color: {}; color: {}; font-family: Arial, sans-serif; margin: 0; padding: 0; }}\n",
                self.theme.background_color, self.theme.text_color
            ),
        );
        html.push_str(
            "        .header { background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 30px; }\n",
        );
        html.push_str("        .header h1 { margin: 0; }\n");
        html.push_str("        .container { max-width: 1200px; margin: 0 auto; padding: 20px; }\n");
        html.push_str(
            "        .filters { background-color: white; padding: 15px; margin-bottom: 20px; border-radius: 5px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }\n",
        );
        html.push_str(
            "        .filter-btn { padding: 8px 15px; margin: 5px; border: none; border-radius: 3px; cursor: pointer; background-color: #ecf0f1; }\n",
        );
        html.push_str("        .filter-btn.active { background-color: #3498db; color: white; }\n");
        html.push_str(
            "        .change-card { background-color: white; border-radius: 8px; padding: 20px; margin: 15px 0; box-shadow: 0 2px 8px rgba(0,0,0,0.1); }\n",
        );
        html.push_str(
            "        .change-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 15px; }\n",
        );
        html.push_str(
            "        .change-title { font-size: 1.3em; font-weight: bold; color: #2c3e50; }\n",
        );
        html.push_str(
            "        .change-badge { padding: 5px 12px; border-radius: 20px; font-size: 0.85em; font-weight: bold; }\n",
        );
        html.push_str("        .badge-proposed { background-color: #3498db; color: white; }\n");
        html.push_str("        .badge-enacted { background-color: #27ae60; color: white; }\n");
        html.push_str("        .badge-repealed { background-color: #e74c3c; color: white; }\n");
        html.push_str("        .badge-amended { background-color: #f39c12; color: white; }\n");
        html.push_str(
            "        .change-meta { color: #7f8c8d; font-size: 0.9em; margin-bottom: 10px; }\n",
        );
        html.push_str(
            "        .change-description { line-height: 1.6; color: #34495e; margin-bottom: 15px; }\n",
        );
        html.push_str(
            "        .change-impact { background-color: #fff3cd; border-left: 4px solid #f39c12; padding: 10px; margin-top: 10px; }\n",
        );
        html.push_str("        .change-impact-title { font-weight: bold; color: #856404; }\n");
        html.push_str("        .sectors { margin-top: 10px; }\n");
        html.push_str(
            "        .sector-tag { display: inline-block; background-color: #e8f4f8; color: #0366d6; padding: 4px 10px; margin: 3px; border-radius: 3px; font-size: 0.85em; }\n",
        );
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
        html.push_str(
            "        <div class=\"change-meta\">Agency: ${data.agency} | Effective: ${data.effective_date}</div>\n",
        );
        html.push_str("        <div class=\"change-description\">${data.description}</div>\n");
        html.push_str(
            "        ${data.impact_assessment ? '<div class=\"change-impact\"><div class=\"change-impact-title\">Impact Assessment</div><div>' + data.impact_assessment + '</div></div>' : ''}\n",
        );
        html.push_str(
            "        ${data.affected_sectors && data.affected_sectors.length > 0 ? '<div class=\"sectors\">' + data.affected_sectors.map(s => '<span class=\"sector-tag\">' + s + '</span>').join('') + '</div>' : ''}\n",
        );
        html.push_str("    `;\n");
        html.push_str("    container.insertBefore(card, container.firstChild);\n");
        html.push_str("};\n");
        html.push_str("// Filter functionality\n");
        html.push_str("document.querySelectorAll('.filter-btn').forEach(btn => {\n");
        html.push_str("    btn.addEventListener('click', function() {\n");
        html.push_str(
            "        document.querySelectorAll('.filter-btn').forEach(b => b.classList.remove('active'));\n",
        );
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
/// Time-series visualization for statute changes.
#[derive(Debug, Clone)]
pub struct StatuteTimeSeries {
    /// Title of the time series
    pub title: String,
    /// Change events
    pub events: Vec<StatuteChangeEvent>,
    /// Theme
    pub theme: Theme,
    /// Show metrics on chart
    pub show_metrics: bool,
}
impl StatuteTimeSeries {
    /// Creates a new statute time series.
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            events: Vec::new(),
            theme: Theme::light(),
            show_metrics: true,
        }
    }
    /// Adds a change event.
    pub fn add_event(&mut self, event: StatuteChangeEvent) {
        self.events.push(event);
    }
    /// Sets the theme.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
    /// Sets whether to show metrics.
    pub fn with_show_metrics(mut self, show: bool) -> Self {
        self.show_metrics = show;
        self
    }
    /// Generates HTML time-series chart using D3.js.
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
        html.push_str("        .chart { margin: 20px auto; max-width: 1200px; }\n");
        html.push_str("        .axis-label { font-size: 14px; font-weight: bold; }\n");
        html.push_str("        .event-dot { cursor: pointer; }\n");
        html.push_str("        .event-dot:hover { r: 8; }\n");
        html.push_str(
            "        .tooltip { position: absolute; padding: 10px; background: rgba(0,0,0,0.8); color: white; border-radius: 5px; pointer-events: none; opacity: 0; }\n",
        );
        html.push_str("        .legend { margin: 20px 0; }\n");
        html.push_str("        .legend-item { display: inline-block; margin-right: 20px; }\n");
        html.push_str(
            "        .legend-color { display: inline-block; width: 15px; height: 15px; margin-right: 5px; }\n",
        );
        html.push_str("    </style>\n");
        html.push_str("</head>\n<body>\n");
        html.push_str(&format!("    <h1>{}</h1>\n", self.title));
        html.push_str("    <div class=\"legend\">\n");
        html.push_str(
            "        <div class=\"legend-item\"><span class=\"legend-color\" style=\"background: #27ae60;\"></span>Enacted</div>\n",
        );
        html.push_str(
            "        <div class=\"legend-item\"><span class=\"legend-color\" style=\"background: #3498db;\"></span>Amended</div>\n",
        );
        html.push_str(
            "        <div class=\"legend-item\"><span class=\"legend-color\" style=\"background: #e74c3c;\"></span>Repealed</div>\n",
        );
        html.push_str(
            "        <div class=\"legend-item\"><span class=\"legend-color\" style=\"background: #f39c12;\"></span>Suspended</div>\n",
        );
        html.push_str(
            "        <div class=\"legend-item\"><span class=\"legend-color\" style=\"background: #9b59b6;\"></span>Reinstated</div>\n",
        );
        html.push_str("    </div>\n");
        html.push_str("    <div id=\"chart\" class=\"chart\"></div>\n");
        html.push_str("    <div id=\"tooltip\" class=\"tooltip\"></div>\n");
        html.push_str("    <script>\n");
        html.push_str(&format!(
            "        const data = {};\n",
            serde_json::to_string(&self.events).expect("invariant: events is serializable")
        ));
        html.push_str("        \n");
        html.push_str("        const margin = {top: 40, right: 40, bottom: 60, left: 60};\n");
        html.push_str("        const width = 1100 - margin.left - margin.right;\n");
        html.push_str("        const height = 500 - margin.top - margin.bottom;\n");
        html.push_str("        \n");
        html.push_str("        const svg = d3.select('#chart')\n");
        html.push_str("            .append('svg')\n");
        html.push_str("            .attr('width', width + margin.left + margin.right)\n");
        html.push_str("            .attr('height', height + margin.top + margin.bottom)\n");
        html.push_str("            .append('g')\n");
        html.push_str(
            "            .attr('transform', `translate(${margin.left},${margin.top})`);\n",
        );
        html.push_str("        \n");
        html.push_str("        // Parse dates\n");
        html.push_str("        data.forEach(d => { d.date = new Date(d.timestamp); });\n");
        html.push_str("        \n");
        html.push_str("        // Scales\n");
        html.push_str("        const x = d3.scaleTime()\n");
        html.push_str("            .domain(d3.extent(data, d => d.date))\n");
        html.push_str("            .range([0, width]);\n");
        html.push_str("        \n");
        if self.show_metrics {
            html.push_str("        const y = d3.scaleLinear()\n");
            html.push_str("            .domain([0, d3.max(data, d => d.metric_value || 0)])\n");
            html.push_str("            .range([height, 0]);\n");
        } else {
            html.push_str("        const y = d3.scaleBand()\n");
            html.push_str("            .domain(data.map(d => d.id))\n");
            html.push_str("            .range([0, height])\n");
            html.push_str("            .padding(0.1);\n");
        }
        html.push_str("        \n");
        html.push_str("        // Color scale\n");
        html.push_str("        const colorMap = {\n");
        html.push_str("            'enacted': '#27ae60',\n");
        html.push_str("            'amended': '#3498db',\n");
        html.push_str("            'repealed': '#e74c3c',\n");
        html.push_str("            'suspended': '#f39c12',\n");
        html.push_str("            'reinstated': '#9b59b6'\n");
        html.push_str("        };\n");
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
        html.push_str("            .attr('class', 'axis-label')\n");
        html.push_str("            .attr('x', width / 2)\n");
        html.push_str("            .attr('y', height + 50)\n");
        html.push_str("            .attr('text-anchor', 'middle')\n");
        html.push_str("            .text('Time');\n");
        html.push_str("        \n");
        html.push_str("        svg.append('text')\n");
        html.push_str("            .attr('class', 'axis-label')\n");
        html.push_str("            .attr('transform', 'rotate(-90)')\n");
        html.push_str("            .attr('x', -height / 2)\n");
        html.push_str("            .attr('y', -50)\n");
        html.push_str("            .attr('text-anchor', 'middle')\n");
        if self.show_metrics {
            html.push_str("            .text('Metric Value');\n");
        } else {
            html.push_str("            .text('Events');\n");
        }
        html.push_str("        \n");
        html.push_str("        // Tooltip\n");
        html.push_str("        const tooltip = d3.select('#tooltip');\n");
        html.push_str("        \n");
        html.push_str("        // Plot events\n");
        html.push_str("        svg.selectAll('.event-dot')\n");
        html.push_str("            .data(data)\n");
        html.push_str("            .enter()\n");
        html.push_str("            .append('circle')\n");
        html.push_str("            .attr('class', 'event-dot')\n");
        html.push_str("            .attr('cx', d => x(d.date))\n");
        if self.show_metrics {
            html.push_str("            .attr('cy', d => y(d.metric_value || 0))\n");
        } else {
            html.push_str("            .attr('cy', (d, i) => y(d.id) + y.bandwidth() / 2)\n");
        }
        html.push_str("            .attr('r', 5)\n");
        html.push_str(
            "            .attr('fill', d => colorMap[d.change_type.toLowerCase()] || '#95a5a6')\n",
        );
        html.push_str("            .on('mouseover', (event, d) => {\n");
        html.push_str("                tooltip.style('opacity', 1)\n");
        html.push_str(
            "                    .html(`<strong>${d.statute_name}</strong><br>Type: ${d.change_type}<br>Version: ${d.version}<br>Date: ${d.timestamp}<br>${d.description}`)\n",
        );
        html.push_str("                    .style('left', (event.pageX + 10) + 'px')\n");
        html.push_str("                    .style('top', (event.pageY - 10) + 'px');\n");
        html.push_str("            })\n");
        html.push_str("            .on('mouseout', () => {\n");
        html.push_str("                tooltip.style('opacity', 0);\n");
        html.push_str("            });\n");
        if self.show_metrics {
            html.push_str("        \n");
            html.push_str("        // Connect events with lines\n");
            html.push_str("        const line = d3.line()\n");
            html.push_str("            .x(d => x(d.date))\n");
            html.push_str("            .y(d => y(d.metric_value || 0))\n");
            html.push_str("            .curve(d3.curveMonotoneX);\n");
            html.push_str("        \n");
            html.push_str("        svg.append('path')\n");
            html.push_str("            .datum(data)\n");
            html.push_str("            .attr('fill', 'none')\n");
            html.push_str(&format!(
                "            .attr('stroke', '{}')\n",
                self.theme.link_color
            ));
            html.push_str("            .attr('stroke-width', 2)\n");
            html.push_str("            .attr('d', line);\n");
        }
        html.push_str("    </script>\n");
        html.push_str("</body>\n</html>");
        html
    }
    /// Generates Mermaid timeline diagram.
    pub fn to_mermaid(&self) -> String {
        let mut diagram = String::new();
        diagram.push_str("timeline\n");
        diagram.push_str(&format!("    title {}\n", self.title));
        for event in &self.events {
            diagram.push_str(&format!(
                "    {} : {} ({}) - {}\n",
                event.timestamp, event.statute_name, event.change_type, event.version
            ));
        }
        diagram
    }
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
/// Looking Glass holographic display visualizer.
pub struct LookingGlassVisualizer {
    pub(crate) title: String,
    pub(crate) config: LookingGlassConfig,
    pub(crate) theme: Theme,
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
        html.push_str(
            "    <script src=\"https://cdnjs.cloudflare.com/ajax/libs/three.js/r128/three.min.js\"></script>\n",
        );
        html.push_str(
            "    <script src=\"https://unpkg.com/holoplay-core@0.1.1/dist/holoplay-core.min.js\"></script>\n",
        );
        html.push_str("    <style>\n");
        html.push_str("        body { margin: 0; overflow: hidden; background: #000; }\n");
        html.push_str("        #canvas { width: 100%; height: 100%; }\n");
        html.push_str(
            "        #info { position: absolute; top: 10px; left: 10px; color: #fff; font-family: monospace; }\n",
        );
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
            serde_json::to_string(&self.config).expect("invariant: config is serializable")
        ));
        html.push_str("        const scene = new THREE.Scene();\n");
        html.push_str(
            &format!(
                "        const camera = new THREE.PerspectiveCamera({}, window.innerWidth / window.innerHeight, {}, {});\n",
                self.config.fov, self.config.depth_range.0, self.config.depth_range.1
            ),
        );
        html.push_str("        camera.position.set(0, 0, 10);\n");
        html.push_str(
            "        const renderer = new THREE.WebGLRenderer({ canvas: document.getElementById('canvas'), antialias: true });\n",
        );
        html.push_str("        renderer.setSize(window.innerWidth, window.innerHeight);\n");
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
