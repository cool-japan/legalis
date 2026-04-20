//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::types::{ChoroplethData, CursorPosition};
use super::types_4::DependencyGraph;
use super::types_5::TrendDataPoint;
use super::types_6::GeoCoordinate;
use super::types_7::{ChartType, CollaborativeUser, GeoJsonFeature};
use super::types_8::{ConceptRelationshipGraph, GeoPoint, TileProvider};
use super::types_9::{HeatMapPoint, SharedAnnotation};
use super::types_12::DecisionTree;

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
    pub(crate) annotations: Vec<SharedAnnotation>,
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
        html.push_str(
            "        .cursor { position: absolute; width: 20px; height: 20px; border-radius: 50%; pointer-events: none; transition: all 0.1s ease; }\n",
        );
        html.push_str(
            "        .cursor-label { position: absolute; background: rgba(0,0,0,0.8); color: white; padding: 2px 6px; border-radius: 3px; font-size: 12px; margin-left: 25px; white-space: nowrap; }\n",
        );
        html.push_str(
            "        .annotation { position: absolute; background: #fff; border: 2px solid #333; border-radius: 4px; padding: 10px; box-shadow: 0 2px 8px rgba(0,0,0,0.2); max-width: 300px; z-index: 100; }\n",
        );
        html.push_str(
            "        .annotation-header { font-weight: bold; margin-bottom: 5px; display: flex; justify-content: space-between; align-items: center; }\n",
        );
        html.push_str(
            "        .annotation-author { font-size: 11px; color: #666; margin-bottom: 5px; }\n",
        );
        html.push_str("        .annotation-content { font-size: 13px; }\n");
        html.push_str(
            "        .annotation-resolved { opacity: 0.6; text-decoration: line-through; }\n",
        );
        html.push_str(
            "        .users-panel { position: fixed; top: 10px; right: 10px; background: white; border-radius: 8px; padding: 15px; box-shadow: 0 2px 8px rgba(0,0,0,0.2); max-width: 200px; }\n",
        );
        html.push_str(
            "        .user-item { display: flex; align-items: center; margin: 5px 0; font-size: 13px; }\n",
        );
        html.push_str(
            "        .user-dot { width: 10px; height: 10px; border-radius: 50%; margin-right: 8px; }\n",
        );
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
        html.push_str(&self.generate_websocket_code());
        html.push_str(&self.generate_cursor_code());
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
/// Geographic visualization renderer.
#[derive(Debug, Clone)]
pub struct GeoVisualization {
    tile_provider: TileProvider,
    center: GeoCoordinate,
    zoom: u32,
    pub(crate) theme: Theme,
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
        html.push_str(
            "    <link rel=\"stylesheet\" href=\"https://unpkg.com/leaflet@1.9.4/dist/leaflet.css\" />\n",
        );
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
        html.push_str(
            "        .legend-color { display: inline-block; width: 20px; height: 20px; margin-right: 5px; }\n",
        );
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
        html.push_str("const choroplethData = {\n");
        for item in data {
            html.push_str(&format!("    '{}': {},\n", item.region_id, item.value));
        }
        html.push_str("};\n\n");
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
        html.push_str(
            "    <link rel=\"stylesheet\" href=\"https://unpkg.com/leaflet@1.9.4/dist/leaflet.css\" />\n",
        );
        html.push_str(
            "    <script src=\"https://unpkg.com/leaflet@1.9.4/dist/leaflet.js\"></script>\n",
        );
        html.push_str(
            "    <script src=\"https://unpkg.com/leaflet.heat@0.2.0/dist/leaflet-heat.js\"></script>\n",
        );
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
        html.push_str(
            "    <link rel=\"stylesheet\" href=\"https://unpkg.com/leaflet@1.9.4/dist/leaflet.css\" />\n",
        );
        html.push_str(
            "    <link rel=\"stylesheet\" href=\"https://unpkg.com/leaflet.markercluster@1.4.1/dist/MarkerCluster.css\" />\n",
        );
        html.push_str(
            "    <link rel=\"stylesheet\" href=\"https://unpkg.com/leaflet.markercluster@1.4.1/dist/MarkerCluster.Default.css\" />\n",
        );
        html.push_str(
            "    <script src=\"https://unpkg.com/leaflet@1.9.4/dist/leaflet.js\"></script>\n",
        );
        html.push_str(
            "    <script src=\"https://unpkg.com/leaflet.markercluster@1.4.1/dist/leaflet.markercluster.js\"></script>\n",
        );
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
/// Legislative trend chart visualizer.
#[derive(Debug, Clone)]
pub struct LegislativeTrendChart {
    /// Chart title
    pub title: String,
    /// Trend data points
    pub data_points: Vec<TrendDataPoint>,
    /// Theme
    pub theme: Theme,
    /// Chart type (line, bar, area)
    pub chart_type: ChartType,
}
impl LegislativeTrendChart {
    /// Creates a new legislative trend chart.
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            data_points: Vec::new(),
            theme: Theme::light(),
            chart_type: ChartType::Line,
        }
    }
    /// Adds a data point.
    pub fn add_data_point(&mut self, period: &str, category: &str, value: f64) {
        self.data_points.push(TrendDataPoint {
            period: period.to_string(),
            category: category.to_string(),
            value,
        });
    }
    /// Sets the theme.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
    /// Sets the chart type.
    pub fn with_chart_type(mut self, chart_type: ChartType) -> Self {
        self.chart_type = chart_type;
        self
    }
    /// Generates HTML trend chart using D3.js.
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
        html.push_str("        .legend { margin: 20px 0; }\n");
        html.push_str("        .legend-item { display: inline-block; margin-right: 20px; }\n");
        html.push_str(
            "        .legend-color { display: inline-block; width: 20px; height: 4px; margin-right: 5px; }\n",
        );
        html.push_str("    </style>\n");
        html.push_str("</head>\n<body>\n");
        html.push_str(&format!("    <h1>{}</h1>\n", self.title));
        html.push_str("    <div id=\"chart\" class=\"chart\"></div>\n");
        html.push_str("    <script>\n");
        html.push_str(&format!(
            "        const data = {};\n",
            serde_json::to_string(&self.data_points)
                .expect("invariant: data_points is serializable")
        ));
        html.push_str("        \n");
        html.push_str("        const margin = {top: 40, right: 40, bottom: 80, left: 60};\n");
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
        html.push_str("        // Group data by category\n");
        html.push_str(
            "        const categories = Array.from(new Set(data.map(d => d.category)));\n",
        );
        html.push_str("        const periods = Array.from(new Set(data.map(d => d.period)));\n");
        html.push_str("        \n");
        html.push_str("        // Scales\n");
        html.push_str("        const x = d3.scaleBand()\n");
        html.push_str("            .domain(periods)\n");
        html.push_str("            .range([0, width])\n");
        html.push_str("            .padding(0.1);\n");
        html.push_str("        \n");
        html.push_str("        const y = d3.scaleLinear()\n");
        html.push_str("            .domain([0, d3.max(data, d => d.value)])\n");
        html.push_str("            .range([height, 0]);\n");
        html.push_str("        \n");
        html.push_str("        const color = d3.scaleOrdinal(d3.schemeCategory10)\n");
        html.push_str("            .domain(categories);\n");
        html.push_str("        \n");
        html.push_str("        // Axes\n");
        html.push_str("        svg.append('g')\n");
        html.push_str("            .attr('transform', `translate(0,${height})`)\n");
        html.push_str("            .call(d3.axisBottom(x))\n");
        html.push_str("            .selectAll('text')\n");
        html.push_str("            .attr('transform', 'rotate(-45)')\n");
        html.push_str("            .style('text-anchor', 'end');\n");
        html.push_str("        \n");
        html.push_str("        svg.append('g')\n");
        html.push_str("            .call(d3.axisLeft(y));\n");
        html.push_str("        \n");
        html.push_str("        // Axis labels\n");
        html.push_str("        svg.append('text')\n");
        html.push_str("            .attr('class', 'axis-label')\n");
        html.push_str("            .attr('x', width / 2)\n");
        html.push_str("            .attr('y', height + 70)\n");
        html.push_str("            .attr('text-anchor', 'middle')\n");
        html.push_str("            .text('Period');\n");
        html.push_str("        \n");
        html.push_str("        svg.append('text')\n");
        html.push_str("            .attr('class', 'axis-label')\n");
        html.push_str("            .attr('transform', 'rotate(-90)')\n");
        html.push_str("            .attr('x', -height / 2)\n");
        html.push_str("            .attr('y', -50)\n");
        html.push_str("            .attr('text-anchor', 'middle')\n");
        html.push_str("            .text('Value');\n");
        html.push_str("        \n");
        match self.chart_type {
            ChartType::Line => {
                html.push_str("        // Line chart\n");
                html.push_str("        const line = d3.line()\n");
                html.push_str("            .x(d => x(d.period) + x.bandwidth() / 2)\n");
                html.push_str("            .y(d => y(d.value));\n");
                html.push_str("        \n");
                html.push_str("        categories.forEach(cat => {\n");
                html.push_str(
                    "            const catData = data.filter(d => d.category === cat);\n",
                );
                html.push_str("            svg.append('path')\n");
                html.push_str("                .datum(catData)\n");
                html.push_str("                .attr('fill', 'none')\n");
                html.push_str("                .attr('stroke', color(cat))\n");
                html.push_str("                .attr('stroke-width', 2)\n");
                html.push_str("                .attr('d', line);\n");
                html.push_str("            \n");
                html.push_str("            svg.selectAll(`.dot-${cat}`)\n");
                html.push_str("                .data(catData)\n");
                html.push_str("                .enter()\n");
                html.push_str("                .append('circle')\n");
                html.push_str(
                    "                .attr('cx', d => x(d.period) + x.bandwidth() / 2)\n",
                );
                html.push_str("                .attr('cy', d => y(d.value))\n");
                html.push_str("                .attr('r', 4)\n");
                html.push_str("                .attr('fill', color(cat));\n");
                html.push_str("        });\n");
            }
            ChartType::Bar => {
                html.push_str("        // Bar chart\n");
                html.push_str("        const subgroups = categories;\n");
                html.push_str("        const xSubgroup = d3.scaleBand()\n");
                html.push_str("            .domain(subgroups)\n");
                html.push_str("            .range([0, x.bandwidth()])\n");
                html.push_str("            .padding(0.05);\n");
                html.push_str("        \n");
                html.push_str("        svg.append('g')\n");
                html.push_str("            .selectAll('g')\n");
                html.push_str("            .data(periods)\n");
                html.push_str("            .enter()\n");
                html.push_str("            .append('g')\n");
                html.push_str("            .attr('transform', d => `translate(${x(d)},0)`)\n");
                html.push_str("            .selectAll('rect')\n");
                html.push_str(
                    "            .data(period => categories.map(cat => ({period, category: cat, value: (data.find(d => d.period === period && d.category === cat) || {value: 0}).value})))\n",
                );
                html.push_str("            .enter()\n");
                html.push_str("            .append('rect')\n");
                html.push_str("            .attr('x', d => xSubgroup(d.category))\n");
                html.push_str("            .attr('y', d => y(d.value))\n");
                html.push_str("            .attr('width', xSubgroup.bandwidth())\n");
                html.push_str("            .attr('height', d => height - y(d.value))\n");
                html.push_str("            .attr('fill', d => color(d.category));\n");
            }
            ChartType::Area => {
                html.push_str("        // Area chart\n");
                html.push_str("        const area = d3.area()\n");
                html.push_str("            .x(d => x(d.period) + x.bandwidth() / 2)\n");
                html.push_str("            .y0(height)\n");
                html.push_str("            .y1(d => y(d.value));\n");
                html.push_str("        \n");
                html.push_str("        categories.forEach(cat => {\n");
                html.push_str(
                    "            const catData = data.filter(d => d.category === cat);\n",
                );
                html.push_str("            svg.append('path')\n");
                html.push_str("                .datum(catData)\n");
                html.push_str("                .attr('fill', color(cat))\n");
                html.push_str("                .attr('fill-opacity', 0.5)\n");
                html.push_str("                .attr('stroke', color(cat))\n");
                html.push_str("                .attr('stroke-width', 2)\n");
                html.push_str("                .attr('d', area);\n");
                html.push_str("        });\n");
            }
        }
        html.push_str("        \n");
        html.push_str("        // Legend\n");
        html.push_str("        const legend = svg.append('g')\n");
        html.push_str("            .attr('transform', `translate(${width - 100}, 0)`);\n");
        html.push_str("        \n");
        html.push_str("        categories.forEach((cat, i) => {\n");
        html.push_str("            const legendRow = legend.append('g')\n");
        html.push_str("                .attr('transform', `translate(0, ${i * 20})`);\n");
        html.push_str("            \n");
        html.push_str("            legendRow.append('rect')\n");
        html.push_str("                .attr('width', 15)\n");
        html.push_str("                .attr('height', 15)\n");
        html.push_str("                .attr('fill', color(cat));\n");
        html.push_str("            \n");
        html.push_str("            legendRow.append('text')\n");
        html.push_str("                .attr('x', 20)\n");
        html.push_str("                .attr('y', 12)\n");
        html.push_str("                .text(cat);\n");
        html.push_str("        });\n");
        html.push_str("    </script>\n");
        html.push_str("</body>\n</html>");
        html
    }
}
/// Timeline event in a story.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineStoryEvent {
    /// Event date
    pub date: String,
    /// Event description
    pub description: String,
}
/// Key player in a case story.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyPlayer {
    /// Player name
    pub name: String,
    /// Player role
    pub role: String,
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
            serde_json::to_string(&self.matches).expect("invariant: matches is serializable"),
            self.highlight_color
        );
        base_html.replace("</body>", &format!("{}</body>", highlight_script))
    }
}
/// GraphML exporter for network analysis tools.
#[derive(Debug, Clone)]
pub struct GraphMLExporter {
    /// Include visual attributes
    pub include_visuals: bool,
    /// Theme
    pub theme: Theme,
}
impl GraphMLExporter {
    /// Creates a new GraphML exporter.
    pub fn new() -> Self {
        Self {
            include_visuals: true,
            theme: Theme::light(),
        }
    }
    /// Sets whether to include visual attributes.
    pub fn with_visuals(mut self, include: bool) -> Self {
        self.include_visuals = include;
        self
    }
    /// Sets the theme.
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.theme = theme;
        self
    }
    /// Exports a dependency graph to GraphML format.
    pub fn export_graph(&self, graph: &DependencyGraph) -> String {
        let mut xml = String::new();
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str("<graphml xmlns=\"http://graphml.graphdrawing.org/xmlns\"\n");
        xml.push_str("    xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\"\n");
        xml.push_str("    xsi:schemaLocation=\"http://graphml.graphdrawing.org/xmlns\n");
        xml.push_str("    http://graphml.graphdrawing.org/xmlns/1.0/graphml.xsd\">\n\n");
        xml.push_str("  <key id=\"d0\" for=\"node\" attr.name=\"label\" attr.type=\"string\"/>\n");
        xml.push_str("  <key id=\"d1\" for=\"node\" attr.name=\"type\" attr.type=\"string\"/>\n");
        xml.push_str(
            "  <key id=\"d2\" for=\"edge\" attr.name=\"relationship\" attr.type=\"string\"/>\n",
        );
        if self.include_visuals {
            xml.push_str(
                "  <key id=\"d3\" for=\"node\" attr.name=\"color\" attr.type=\"string\"/>\n",
            );
            xml.push_str(
                "  <key id=\"d4\" for=\"node\" attr.name=\"size\" attr.type=\"double\"/>\n",
            );
        }
        xml.push('\n');
        xml.push_str("  <graph id=\"G\" edgedefault=\"directed\">\n");
        for node_idx in graph.graph.node_indices() {
            let statute_id = &graph.graph[node_idx];
            xml.push_str(&format!("    <node id=\"n{}\">\n", node_idx.index()));
            xml.push_str(&format!("      <data key=\"d0\">{}</data>\n", statute_id));
            xml.push_str("      <data key=\"d1\">statute</data>\n");
            if self.include_visuals {
                xml.push_str(&format!(
                    "      <data key=\"d3\">{}</data>\n",
                    self.theme.condition_color
                ));
                xml.push_str("      <data key=\"d4\">30.0</data>\n");
            }
            xml.push_str("    </node>\n");
        }
        let mut edge_id = 0;
        for edge in graph.graph.edge_indices() {
            if let Some((source, target)) = graph.graph.edge_endpoints(edge) {
                xml.push_str(&format!(
                    "    <edge id=\"e{}\" source=\"n{}\" target=\"n{}\">\n",
                    edge_id,
                    source.index(),
                    target.index()
                ));
                xml.push_str("      <data key=\"d2\">depends_on</data>\n");
                xml.push_str("    </edge>\n");
                edge_id += 1;
            }
        }
        xml.push_str("  </graph>\n");
        xml.push_str("</graphml>\n");
        xml
    }
    /// Exports a decision tree to GraphML format.
    pub fn export_decision_tree(&self, _tree: &DecisionTree) -> String {
        let mut xml = String::new();
        xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        xml.push_str("<graphml xmlns=\"http://graphml.graphdrawing.org/xmlns\">\n");
        xml.push_str("  <key id=\"d0\" for=\"node\" attr.name=\"label\" attr.type=\"string\"/>\n");
        xml.push_str("  <key id=\"d1\" for=\"node\" attr.name=\"type\" attr.type=\"string\"/>\n");
        xml.push('\n');
        xml.push_str("  <graph id=\"G\" edgedefault=\"directed\">\n");
        xml.push_str("    <node id=\"n0\">\n");
        xml.push_str("      <data key=\"d0\">Decision Tree</data>\n");
        xml.push_str("      <data key=\"d1\">root</data>\n");
        xml.push_str("    </node>\n");
        xml.push_str("  </graph>\n");
        xml.push_str("</graphml>\n");
        xml
    }
}
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
