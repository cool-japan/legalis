//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use legalis_core::Statute;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::types::{HolographicModelConfig, VRExplorationConfig};
use super::types_3::{DashboardFilter, DashboardWidget};
use super::types_4::{DependencyGraph, GeoJsonGeometry};
use super::types_5::VizError;
use super::types_6::ScrollChapter;
use super::types_8::{ConceptRelationshipGraph, LegalConcept};
use super::types_10::Theme;
use super::types_11::{ConceptRelationType, DecisionNode};
use super::types_12::DecisionTree;

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
    pub layout: (u32, u32),
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
/// Legal history scrollytelling visualizer.
pub struct LegalHistoryScrollytelling {
    /// Title
    pub(crate) title: String,
    /// Configuration
    pub(crate) config: ScrollytellingConfig,
    /// Theme
    pub(crate) theme: Theme,
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
        html.push_str(
            &format!(
                "        body {{ margin: 0; padding: 0; font-family: 'Georgia', serif; background-color: {}; color: {}; }}\n",
                self.theme.background_color, self.theme.text_color
            ),
        );
        html.push_str(
            "        .chapter { min-height: 100vh; padding: 100px 20px; position: relative; }\n",
        );
        html.push_str(
            "        .chapter-content { max-width: 800px; margin: 0 auto; font-size: 1.2em; line-height: 1.8; }\n",
        );
        html.push_str(
            "        .chapter-title { font-size: 2.5em; font-weight: bold; margin-bottom: 30px; }\n",
        );
        html.push_str("        .chapter-text { margin-bottom: 20px; }\n");
        html.push_str(
            "        .visual-element { background-color: #f5f5f5; padding: 30px; margin: 40px 0; border-radius: 8px; text-align: center; }\n",
        );
        html.push_str(
            "        .progress-bar { position: fixed; top: 0; left: 0; height: 4px; background: linear-gradient(90deg, #3498db, #2ecc71); width: 0%; transition: width 0.3s; z-index: 1000; }\n",
        );
        html.push_str(
            "        .chapter-nav { position: fixed; right: 20px; top: 50%; transform: translateY(-50%); z-index: 100; }\n",
        );
        html.push_str(
            "        .nav-dot { width: 12px; height: 12px; border-radius: 50%; background-color: #ccc; margin: 10px 0; cursor: pointer; transition: all 0.3s; }\n",
        );
        html.push_str(
            "        .nav-dot.active { background-color: #3498db; transform: scale(1.5); }\n",
        );
        html.push_str(
            "        .fade-in { opacity: 0; transform: translateY(50px); transition: opacity 0.8s, transform 0.8s; }\n",
        );
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
            html.push_str(
                "    const scrolled = (window.scrollY / (document.body.scrollHeight - window.innerHeight)) * 100;\n",
            );
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
/// Visualizer for legal reasoning chains and explanations.
pub struct ReasoningChainVisualizer {
    pub(crate) theme: Theme,
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
        if !explanation.applicable_statutes.is_empty() {
            html.push_str("<h3>Applicable Statutes</h3><ul>");
            for statute in &explanation.applicable_statutes {
                html.push_str(&format!("<li>{}</li>", statute));
            }
            html.push_str("</ul>");
        }
        if !explanation.satisfied_conditions.is_empty() {
            html.push_str("<h3>Satisfied Conditions</h3><ul>");
            for condition in &explanation.satisfied_conditions {
                html.push_str(&format!("<li style='color: green;'>✓ {}</li>", condition));
            }
            html.push_str("</ul>");
        }
        if !explanation.unsatisfied_conditions.is_empty() {
            html.push_str("<h3>Unsatisfied Conditions</h3><ul>");
            for condition in &explanation.unsatisfied_conditions {
                html.push_str(&format!("<li style='color: red;'>✗ {}</li>", condition));
            }
            html.push_str("</ul>");
        }
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
        for (i, statute) in explanation.applicable_statutes.iter().enumerate() {
            mermaid.push_str(&format!("    Statutes --> S{}[\"{}\"]\n", i, statute));
        }
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
/// VR statute exploration visualizer.
pub struct VRStatuteExplorer {
    pub(crate) theme: Theme,
    pub(crate) config: VRExplorationConfig,
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
        html.push_str("<div id=\"vr-container\">\n");
        html.push_str("<div class=\"info-overlay\">\n");
        html.push_str("<h2>VR Statute Explorer</h2>\n");
        html.push_str("<p>Click 'Enter VR' to start the immersive experience</p>\n");
        html.push_str("<div id=\"status\">Status: Ready</div>\n");
        html.push_str("<div id=\"node-detail\">Point at nodes to see details</div>\n");
        html.push_str("</div>\n");
        html.push_str("</div>\n");
        html.push_str(
            "<script src=\"https://cdnjs.cloudflare.com/ajax/libs/three.js/r128/three.min.js\"></script>\n",
        );
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
                nodes.push(serde_json::json!(
                    { "label" : label, "type" : node_type, "depth" : 0,
                    "description" : description }
                ));
            }
        }
        nodes
    }
}
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
        for rel in &graph.relationships {
            if rel.to_id == root_id
                && rel.relation_type == ConceptRelationType::IsA
                && let Some(child_tree) = Self::from_graph(graph, &rel.from_id)
            {
                tree.add_child(child_tree);
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
/// Represents the impact of an amendment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmendmentImpact {
    /// Amendment ID
    pub id: String,
    /// Statute ID
    pub statute_id: String,
    /// Statute name
    pub statute_name: String,
    /// Amendment date
    pub date: String,
    /// Description
    pub description: String,
    /// Number of sections affected
    pub sections_affected: usize,
    /// Number of downstream statutes affected
    pub downstream_statutes: usize,
    /// Estimated affected population
    pub affected_population: Option<usize>,
    /// Impact severity (0.0 to 1.0)
    pub severity: f64,
}
impl AmendmentImpact {
    /// Creates a new amendment impact record.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: &str,
        statute_id: &str,
        statute_name: &str,
        date: &str,
        description: &str,
        sections_affected: usize,
        downstream_statutes: usize,
        severity: f64,
    ) -> Self {
        Self {
            id: id.to_string(),
            statute_id: statute_id.to_string(),
            statute_name: statute_name.to_string(),
            date: date.to_string(),
            description: description.to_string(),
            sections_affected,
            downstream_statutes,
            affected_population: None,
            severity: severity.clamp(0.0, 1.0),
        }
    }
    /// Sets the affected population.
    pub fn with_affected_population(mut self, population: usize) -> Self {
        self.affected_population = Some(population);
        self
    }
}
/// Chart type for trends.
#[derive(Debug, Clone)]
pub enum ChartType {
    Line,
    Bar,
    Area,
}
/// Holographic statute model visualizer.
pub struct HolographicStatuteModel {
    pub(crate) theme: Theme,
    pub(crate) config: HolographicModelConfig,
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
        html.push_str(
            "    <script src=\"https://cdnjs.cloudflare.com/ajax/libs/three.js/r128/three.min.js\"></script>\n",
        );
        html.push_str("    <style>\n");
        html.push_str("        body { margin: 0; background: #000; }\n");
        html.push_str("        #container { width: 100vw; height: 100vh; }\n");
        html.push_str(
            "        #info { position: absolute; top: 10px; left: 10px; color: #0f0; font-family: monospace; }\n",
        );
        html.push_str("    </style>\n");
        html.push_str("</head>\n<body>\n");
        html.push_str(&format!(
            "    <div id=\"info\">{}<br>Holographic Statute Model</div>\n",
            statute.title
        ));
        html.push_str("    <div id=\"container\"></div>\n");
        html.push_str("    <script>\n");
        html.push_str("        const scene = new THREE.Scene();\n");
        html.push_str(
            "        const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);\n",
        );
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
                html.push_str(
                    &format!(
                        "        const layer{}Material = new THREE.MeshBasicMaterial({{ color: '{}', transparent: true, opacity: 0.3, side: THREE.DoubleSide }});\n",
                        i, self.theme.condition_color
                    ),
                );
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
/// Represents a historical version of a statute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatuteVersion {
    /// Version ID
    pub version_id: String,
    /// Version number
    pub version: String,
    /// Effective date
    pub effective_date: String,
    /// Statute text or summary
    pub content: String,
    /// List of sections
    pub sections: Vec<String>,
    /// Metadata
    pub metadata: HashMap<String, String>,
}
impl StatuteVersion {
    /// Creates a new statute version.
    pub fn new(version_id: &str, version: &str, effective_date: &str, content: &str) -> Self {
        Self {
            version_id: version_id.to_string(),
            version: version.to_string(),
            effective_date: effective_date.to_string(),
            content: content.to_string(),
            sections: Vec::new(),
            metadata: HashMap::new(),
        }
    }
    /// Adds a section.
    pub fn add_section(&mut self, section: &str) {
        self.sections.push(section.to_string());
    }
    /// Adds metadata.
    pub fn add_metadata(&mut self, key: &str, value: &str) {
        self.metadata.insert(key.to_string(), value.to_string());
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
/// SPARQL export for semantic web/RDF.
#[derive(Debug, Clone)]
pub struct SparqlExporter {
    /// Base URI for resources
    pub base_uri: String,
    /// Include prefixes
    pub include_prefixes: bool,
}
impl SparqlExporter {
    /// Creates a new SPARQL exporter.
    pub fn new() -> Self {
        Self {
            base_uri: "http://example.org/legalis/".to_string(),
            include_prefixes: true,
        }
    }
    /// Sets the base URI.
    pub fn with_base_uri(mut self, uri: &str) -> Self {
        self.base_uri = uri.to_string();
        self
    }
    /// Sets whether to include prefixes.
    pub fn with_prefixes(mut self, include: bool) -> Self {
        self.include_prefixes = include;
        self
    }
    /// Exports a dependency graph to SPARQL INSERT queries.
    pub fn export_graph(&self, graph: &DependencyGraph) -> String {
        let mut sparql = String::new();
        if self.include_prefixes {
            sparql.push_str("# SPARQL INSERT Queries\n");
            sparql.push_str("# Generated by legalis-viz\n\n");
            sparql.push_str("PREFIX leg: <http://example.org/legalis#>\n");
            sparql.push_str("PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>\n");
            sparql.push_str("PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>\n");
            sparql.push_str("PREFIX dc: <http://purl.org/dc/elements/1.1/>\n\n");
        }
        sparql.push_str("INSERT DATA {\n");
        for node_idx in graph.graph.node_indices() {
            let statute_id = &graph.graph[node_idx];
            let uri = format!("{}{}", self.base_uri, statute_id);
            sparql.push_str(&format!("  <{}> rdf:type leg:Statute ;\n", uri));
            sparql.push_str(&format!("    leg:id \"{}\" ;\n", statute_id));
            sparql.push_str(&format!("    rdfs:label \"{}\" .\n\n", statute_id));
        }
        for edge in graph.graph.edge_indices() {
            if let Some((source, target)) = graph.graph.edge_endpoints(edge) {
                let source_id = &graph.graph[source];
                let target_id = &graph.graph[target];
                let source_uri = format!("{}{}", self.base_uri, source_id);
                let target_uri = format!("{}{}", self.base_uri, target_id);
                sparql.push_str(&format!(
                    "  <{}> leg:dependsOn <{}> .\n",
                    source_uri, target_uri
                ));
            }
        }
        sparql.push_str("}\n");
        sparql
    }
    /// Exports a concept graph to SPARQL INSERT queries.
    pub fn export_concept_graph(&self, graph: &ConceptRelationshipGraph) -> String {
        let mut sparql = String::new();
        if self.include_prefixes {
            sparql.push_str("# SPARQL INSERT Queries - Legal Concepts\n");
            sparql.push_str("# Generated by legalis-viz\n\n");
            sparql.push_str("PREFIX leg: <http://example.org/legalis#>\n");
            sparql.push_str("PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>\n");
            sparql.push_str("PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>\n");
            sparql.push_str("PREFIX skos: <http://www.w3.org/2004/02/skos/core#>\n\n");
        }
        sparql.push_str("INSERT DATA {\n");
        for concept in &graph.concepts {
            let uri = format!("{}{}", self.base_uri, concept.id);
            sparql.push_str(&format!("  <{}> rdf:type skos:Concept ;\n", uri));
            sparql.push_str(&format!("    leg:id \"{}\" ;\n", concept.id));
            sparql.push_str(&format!("    skos:prefLabel \"{}\" ;\n", concept.name));
            sparql.push_str(&format!(
                "    skos:definition \"{}\" ;\n",
                concept.description
            ));
            sparql.push_str(&format!("    leg:category \"{}\" .\n\n", concept.category));
        }
        for rel in &graph.relationships {
            let source_uri = format!("{}{}", self.base_uri, rel.from_id);
            let target_uri = format!("{}{}", self.base_uri, rel.to_id);
            let predicate = match rel.relation_type {
                ConceptRelationType::IsA => "skos:broader",
                ConceptRelationType::PartOf => "leg:partOf",
                ConceptRelationType::Requires => "leg:requires",
                ConceptRelationType::ConflictsWith => "leg:conflictsWith",
                ConceptRelationType::Enables => "leg:enables",
                ConceptRelationType::RelatedTo => "skos:related",
                ConceptRelationType::Supersedes => "leg:supersedes",
                ConceptRelationType::Implements => "leg:implements",
            };
            sparql.push_str(&format!(
                "  <{}> {} <{}> .\n",
                source_uri, predicate, target_uri
            ));
        }
        sparql.push_str("}\n");
        sparql
    }
    /// Exports to Turtle (TTL) format.
    pub fn export_to_turtle(&self, graph: &DependencyGraph) -> String {
        let mut turtle = String::new();
        if self.include_prefixes {
            turtle.push_str("@prefix leg: <http://example.org/legalis#> .\n");
            turtle.push_str("@prefix rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .\n");
            turtle.push_str("@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n\n");
        }
        for node_idx in graph.graph.node_indices() {
            let statute_id = &graph.graph[node_idx];
            let uri = format!("{}{}", self.base_uri, statute_id);
            turtle.push_str(&format!("<{}>\n", uri));
            turtle.push_str("  rdf:type leg:Statute ;\n");
            turtle.push_str(&format!("  leg:id \"{}\" ;\n", statute_id));
            turtle.push_str(&format!("  rdfs:label \"{}\" .\n\n", statute_id));
        }
        turtle
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
/// Custom theme builder for creating branded themes.
#[derive(Debug, Clone)]
pub struct CustomThemeBuilder {
    pub(crate) theme: Theme,
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
        self.dpi = 96;
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
