//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::types_3::Timeline;
use super::types_5::TimelineEvent;
use super::types_6::{ImpactSeverity, Panoramic360Config};
use super::types_7::JurisdictionalStatute;
use super::types_9::MarketImpact;
use super::types_10::{JurisdictionalDifference, LegislativeStep, Theme};

/// 360° panoramic case timeline visualizer.
pub struct Panoramic360Timeline {
    pub(crate) theme: Theme,
    pub(crate) config: Panoramic360Config,
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
        html.push_str(
            "<script src=\"https://cdnjs.cloudflare.com/ajax/libs/three.js/r128/three.min.js\"></script>\n",
        );
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
                serde_json::json!(
                    { "date" : date, "type" : event_type, "description" : description }
                )
            })
            .collect()
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
            Self::IsA => "#3498db",
            Self::PartOf => "#2ecc71",
            Self::Requires => "#e74c3c",
            Self::ConflictsWith => "#c0392b",
            Self::Enables => "#f39c12",
            Self::RelatedTo => "#95a5a6",
            Self::Supersedes => "#9b59b6",
            Self::Implements => "#16a085",
        }
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
/// Market impact visualization for legal changes.
pub struct MarketImpactVisualizer {
    /// Visualizer title
    pub(crate) title: String,
    /// WebSocket URL for updates
    pub(crate) ws_url: String,
    /// Theme
    pub(crate) theme: Theme,
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
        html.push_str(
            &format!(
                "        body {{ background-color: {}; color: {}; font-family: 'Segoe UI', Arial, sans-serif; margin: 0; padding: 0; }}\n",
                self.theme.background_color, self.theme.text_color
            ),
        );
        html.push_str(
            "        .header { background: linear-gradient(135deg, #1e3c72 0%, #2a5298 100%); color: white; padding: 30px; }\n",
        );
        html.push_str("        .header h1 { margin: 0; }\n");
        html.push_str("        .container { max-width: 1400px; margin: 0 auto; padding: 20px; }\n");
        html.push_str(
            "        .grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(350px, 1fr)); gap: 20px; margin-bottom: 30px; }\n",
        );
        html.push_str(
            "        .card { background-color: white; border-radius: 8px; padding: 20px; box-shadow: 0 2px 8px rgba(0,0,0,0.1); }\n",
        );
        html.push_str(
            "        .card-title { font-size: 1.2em; font-weight: bold; color: #2c3e50; margin-bottom: 15px; }\n",
        );
        html.push_str(
            "        .metric { display: flex; justify-content: space-between; padding: 10px 0; border-bottom: 1px solid #ecf0f1; }\n",
        );
        html.push_str("        .metric-label { color: #7f8c8d; }\n");
        html.push_str("        .metric-value { font-weight: bold; color: #2c3e50; }\n");
        html.push_str("        .positive { color: #27ae60; }\n");
        html.push_str("        .negative { color: #e74c3c; }\n");
        html.push_str("        .neutral { color: #95a5a6; }\n");
        html.push_str("        .chart-container { position: relative; height: 300px; }\n");
        html.push_str("        .impact-list { }\n");
        html.push_str(
            "        .impact-item { background-color: #f8f9fa; padding: 15px; margin: 10px 0; border-radius: 5px; border-left: 4px solid #3498db; }\n",
        );
        html.push_str(
            "        .impact-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 10px; }\n",
        );
        html.push_str("        .impact-legal { font-weight: bold; color: #2c3e50; }\n");
        html.push_str(
            "        .impact-badge { padding: 4px 10px; border-radius: 3px; font-size: 0.85em; font-weight: bold; }\n",
        );
        html.push_str("        .badge-high { background-color: #e74c3c; color: white; }\n");
        html.push_str("        .badge-medium { background-color: #f39c12; color: white; }\n");
        html.push_str("        .badge-low { background-color: #3498db; color: white; }\n");
        html.push_str("        .sectors { margin-top: 8px; }\n");
        html.push_str(
            "        .sector-badge { display: inline-block; background-color: #e8f4f8; padding: 3px 8px; margin: 2px; border-radius: 3px; font-size: 0.8em; }\n",
        );
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
        let avg_stock_change: f64 = impacts
            .iter()
            .filter_map(|i| i.stock_price_change)
            .sum::<f64>()
            / impacts.len().max(1) as f64;
        let total_affected = impacts.len();
        html.push_str(
            &format!(
                "                <div class=\"metric\"><span class=\"metric-label\">Avg. Stock Change</span><span class=\"metric-value {}\">{:.2}%</span></div>\n",
                if avg_stock_change > 0.0 { "positive" } else if avg_stock_change < 0.0 {
                "negative" } else { "neutral" }, avg_stock_change
            ),
        );
        html.push_str(
            &format!(
                "                <div class=\"metric\"><span class=\"metric-label\">Affected Items</span><span class=\"metric-value\">{}</span></div>\n",
                total_affected
            ),
        );
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
                html.push_str(
                    &format!(
                        "                    <div><strong>Stock Impact:</strong> <span class=\"{}\">{:.2}%</span></div>\n",
                        change_class, stock_change
                    ),
                );
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
        html.push_str(&format!("const ws = new WebSocket('{}');\n", self.ws_url));
        html.push_str("ws.onmessage = function(event) {\n");
        html.push_str("    const data = JSON.parse(event.data);\n");
        html.push_str("    const container = document.getElementById('impact-list');\n");
        html.push_str("    const item = document.createElement('div');\n");
        html.push_str("    item.className = 'impact-item';\n");
        html.push_str("    const severityClass = 'badge-' + data.severity.toLowerCase();\n");
        html.push_str(
            "    const changeClass = data.stock_price_change > 0 ? 'positive' : data.stock_price_change < 0 ? 'negative' : 'neutral';\n",
        );
        html.push_str("    item.innerHTML = `\n");
        html.push_str("        <div class=\"impact-header\">\n");
        html.push_str("            <div class=\"impact-legal\">${data.legal_event}</div>\n");
        html.push_str(
            "            <div class=\"impact-badge ${severityClass}\">${data.severity} Impact</div>\n",
        );
        html.push_str("        </div>\n");
        html.push_str("        <div><strong>Date:</strong> ${data.event_date}</div>\n");
        html.push_str(
            "        ${data.stock_price_change != null ? '<div><strong>Stock Impact:</strong> <span class=\"' + changeClass + '\">' + data.stock_price_change.toFixed(2) + '%</span></div>' : ''}\n",
        );
        html.push_str(
            "        ${data.affected_companies && data.affected_companies.length > 0 ? '<div><strong>Affected:</strong> ' + data.affected_companies.join(', ') + '</div>' : ''}\n",
        );
        html.push_str(
            "        ${data.sectors && data.sectors.length > 0 ? '<div class=\"sectors\">' + data.sectors.map(s => '<span class=\"sector-badge\">' + s + '</span>').join('') + '</div>' : ''}\n",
        );
        html.push_str("    `;\n");
        html.push_str("    container.insertBefore(item, container.firstChild);\n");
        html.push_str("};\n");
        html.push_str("    </script>\n</body>\n</html>");
        html
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
        html.push_str(&format!("    <h1>{}</h1>\n", self.title));
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
                    html.push_str(
                        &format!(
                            "                <div class=\"difference-value\"><strong>{}:</strong> {}</div>\n",
                            jurisdiction, value
                        ),
                    );
                }
                html.push_str("            </div>\n");
                html.push_str("        </div>\n");
            }
            html.push_str("    </div>\n");
        }
        if self.synchronized_nav {
            html.push_str("    <script>\n");
            html.push_str(
                "        const columns = document.querySelectorAll('.jurisdiction-column');\n",
            );
            html.push_str("        columns.forEach(col => {\n");
            html.push_str("            col.addEventListener('scroll', (e) => {\n");
            html.push_str(
                "                const scrollRatio = e.target.scrollTop / (e.target.scrollHeight - e.target.clientHeight);\n",
            );
            html.push_str("                columns.forEach(otherCol => {\n");
            html.push_str("                    if (otherCol !== e.target) {\n");
            html.push_str(
                "                        otherCol.scrollTop = scrollRatio * (otherCol.scrollHeight - otherCol.clientHeight);\n",
            );
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
        html.push_str("        <div class=\"heatmap-cell heatmap-header\">Aspect</div>\n");
        for statute in &self.statutes {
            html.push_str(&format!(
                "        <div class=\"heatmap-cell heatmap-header\">{}</div>\n",
                statute.jurisdiction
            ));
        }
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
/// Legislative process flowchart visualizer
#[derive(Debug, Clone)]
pub struct LegislativeProcessVisualizer {
    pub(crate) theme: Theme,
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
