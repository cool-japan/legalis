//! Map-based knowledge exploration for legal geographic data.
//!
//! This module provides tools for exploring legal knowledge graphs through
//! interactive maps, including GeoJSON export, tile generation, and spatial
//! visualization utilities.

use crate::geosparql::{Jurisdiction, LegalZone};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// GeoJSON Feature Collection for legal data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoJsonFeatureCollection {
    #[serde(rename = "type")]
    pub type_name: String,
    pub features: Vec<GeoJsonFeature>,
}

impl GeoJsonFeatureCollection {
    /// Creates a new empty feature collection
    pub fn new() -> Self {
        Self {
            type_name: "FeatureCollection".to_string(),
            features: Vec::new(),
        }
    }

    /// Adds a feature
    pub fn add_feature(&mut self, feature: GeoJsonFeature) {
        self.features.push(feature);
    }

    /// Converts to JSON string
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}

impl Default for GeoJsonFeatureCollection {
    fn default() -> Self {
        Self::new()
    }
}

/// GeoJSON Feature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoJsonFeature {
    #[serde(rename = "type")]
    pub type_name: String,
    pub id: Option<String>,
    pub geometry: GeoJsonGeometry,
    pub properties: HashMap<String, serde_json::Value>,
}

impl GeoJsonFeature {
    /// Creates a new feature
    pub fn new(geometry: GeoJsonGeometry) -> Self {
        Self {
            type_name: "Feature".to_string(),
            id: None,
            geometry,
            properties: HashMap::new(),
        }
    }

    /// Sets the feature ID
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Adds a property
    pub fn with_property(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.properties.insert(key.into(), value);
        self
    }
}

/// GeoJSON Geometry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoJsonGeometry {
    #[serde(rename = "type")]
    pub type_name: String,
    pub coordinates: serde_json::Value,
}

impl GeoJsonGeometry {
    /// Creates a Point geometry
    pub fn point(lon: f64, lat: f64) -> Self {
        Self {
            type_name: "Point".to_string(),
            coordinates: serde_json::json!([lon, lat]),
        }
    }

    /// Creates a Polygon geometry
    pub fn polygon(coords: Vec<Vec<[f64; 2]>>) -> Self {
        Self {
            type_name: "Polygon".to_string(),
            coordinates: serde_json::json!(coords),
        }
    }

    /// Creates a LineString geometry
    pub fn line_string(coords: Vec<[f64; 2]>) -> Self {
        Self {
            type_name: "LineString".to_string(),
            coordinates: serde_json::json!(coords),
        }
    }

    /// Creates a MultiPolygon geometry
    pub fn multi_polygon(coords: Vec<Vec<Vec<[f64; 2]>>>) -> Self {
        Self {
            type_name: "MultiPolygon".to_string(),
            coordinates: serde_json::json!(coords),
        }
    }
}

/// Converts a Jurisdiction to GeoJSON Feature
pub fn jurisdiction_to_geojson(jurisdiction: &Jurisdiction) -> GeoJsonFeature {
    let geometry = wkt_to_geojson_geometry(&jurisdiction.boundary.wkt);

    GeoJsonFeature::new(geometry)
        .with_id(&jurisdiction.uri)
        .with_property("name", serde_json::json!(&jurisdiction.name))
        .with_property("type", serde_json::json!(&jurisdiction.jurisdiction_type))
        .with_property("uri", serde_json::json!(&jurisdiction.uri))
}

/// Converts a LegalZone to GeoJSON Feature
pub fn legal_zone_to_geojson(zone: &LegalZone) -> GeoJsonFeature {
    let geometry = wkt_to_geojson_geometry(&zone.extent.wkt);

    let mut feature = GeoJsonFeature::new(geometry)
        .with_id(&zone.uri)
        .with_property("name", serde_json::json!(&zone.name))
        .with_property("zone_type", serde_json::json!(&zone.zone_type))
        .with_property("jurisdiction", serde_json::json!(&zone.jurisdiction))
        .with_property("uri", serde_json::json!(&zone.uri));

    if !zone.regulations.is_empty() {
        feature = feature.with_property("regulations", serde_json::json!(&zone.regulations));
    }

    feature
}

/// Converts WKT to GeoJSON geometry (simplified parser)
fn wkt_to_geojson_geometry(wkt: &str) -> GeoJsonGeometry {
    // Simple WKT parser for common types
    if wkt.starts_with("POINT") {
        parse_wkt_point(wkt)
    } else if wkt.starts_with("POLYGON") {
        parse_wkt_polygon(wkt)
    } else if wkt.starts_with("LINESTRING") {
        parse_wkt_linestring(wkt)
    } else if wkt.starts_with("MULTIPOLYGON") {
        parse_wkt_multipolygon(wkt)
    } else {
        // Fallback to point
        GeoJsonGeometry::point(0.0, 0.0)
    }
}

fn parse_wkt_point(wkt: &str) -> GeoJsonGeometry {
    // Extract coordinates from "POINT (lon lat)"
    let coords_str = wkt
        .trim_start_matches("POINT")
        .trim()
        .trim_matches(|c| c == '(' || c == ')');
    let parts: Vec<&str> = coords_str.split_whitespace().collect();

    if parts.len() >= 2 {
        if let (Ok(lon), Ok(lat)) = (parts[0].parse::<f64>(), parts[1].parse::<f64>()) {
            return GeoJsonGeometry::point(lon, lat);
        }
    }

    GeoJsonGeometry::point(0.0, 0.0)
}

fn parse_wkt_polygon(wkt: &str) -> GeoJsonGeometry {
    // Extract coordinates from "POLYGON ((lon lat, ...))"
    let coords_str = wkt
        .trim_start_matches("POLYGON")
        .trim()
        .trim_matches(|c| c == '(' || c == ')');

    let rings: Vec<Vec<[f64; 2]>> = coords_str
        .split("), (")
        .map(parse_coordinate_pairs)
        .collect();

    GeoJsonGeometry::polygon(rings)
}

fn parse_wkt_linestring(wkt: &str) -> GeoJsonGeometry {
    // Extract coordinates from "LINESTRING (lon lat, ...)"
    let coords_str = wkt
        .trim_start_matches("LINESTRING")
        .trim()
        .trim_matches(|c| c == '(' || c == ')');

    let coords = parse_coordinate_pairs(coords_str);
    GeoJsonGeometry::line_string(coords)
}

fn parse_wkt_multipolygon(wkt: &str) -> GeoJsonGeometry {
    // Extract coordinates from "MULTIPOLYGON (((lon lat, ...)))"
    let coords_str = wkt
        .trim_start_matches("MULTIPOLYGON")
        .trim()
        .trim_matches(|c| c == '(' || c == ')');

    // This is a simplified parser
    let polygons: Vec<Vec<Vec<[f64; 2]>>> = coords_str
        .split(")), ((")
        .map(|poly| poly.split("), (").map(parse_coordinate_pairs).collect())
        .collect();

    GeoJsonGeometry::multi_polygon(polygons)
}

fn parse_coordinate_pairs(s: &str) -> Vec<[f64; 2]> {
    s.split(", ")
        .filter_map(|pair| {
            let parts: Vec<&str> = pair.split_whitespace().collect();
            if parts.len() >= 2 {
                if let (Ok(lon), Ok(lat)) = (parts[0].parse::<f64>(), parts[1].parse::<f64>()) {
                    return Some([lon, lat]);
                }
            }
            None
        })
        .collect()
}

/// Map layer definition for organizing features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapLayer {
    /// Layer name
    pub name: String,
    /// Layer type (jurisdictions, zones, etc.)
    pub layer_type: String,
    /// Feature collection
    pub features: GeoJsonFeatureCollection,
    /// Layer styling (optional)
    pub style: Option<LayerStyle>,
}

impl MapLayer {
    /// Creates a new map layer
    pub fn new(name: impl Into<String>, layer_type: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            layer_type: layer_type.into(),
            features: GeoJsonFeatureCollection::new(),
            style: None,
        }
    }

    /// Adds a feature to the layer
    pub fn add_feature(&mut self, feature: GeoJsonFeature) {
        self.features.add_feature(feature);
    }

    /// Sets the layer style
    pub fn with_style(mut self, style: LayerStyle) -> Self {
        self.style = Some(style);
        self
    }

    /// Converts to JSON
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}

/// Layer styling information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayerStyle {
    /// Fill color (hex)
    pub fill_color: Option<String>,
    /// Stroke color (hex)
    pub stroke_color: Option<String>,
    /// Stroke width
    pub stroke_width: Option<f64>,
    /// Fill opacity (0.0 to 1.0)
    pub fill_opacity: Option<f64>,
}

impl LayerStyle {
    /// Creates a new layer style
    pub fn new() -> Self {
        Self {
            fill_color: None,
            stroke_color: None,
            stroke_width: None,
            fill_opacity: None,
        }
    }

    /// Sets fill color
    pub fn with_fill_color(mut self, color: impl Into<String>) -> Self {
        self.fill_color = Some(color.into());
        self
    }

    /// Sets stroke color
    pub fn with_stroke_color(mut self, color: impl Into<String>) -> Self {
        self.stroke_color = Some(color.into());
        self
    }
}

impl Default for LayerStyle {
    fn default() -> Self {
        Self::new()
    }
}

/// Interactive map configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapConfig {
    /// Map title
    pub title: String,
    /// Center coordinates [lon, lat]
    pub center: [f64; 2],
    /// Zoom level
    pub zoom: u8,
    /// Map layers
    pub layers: Vec<MapLayer>,
}

impl MapConfig {
    /// Creates a new map configuration
    pub fn new(title: impl Into<String>, center: [f64; 2], zoom: u8) -> Self {
        Self {
            title: title.into(),
            center,
            zoom,
            layers: Vec::new(),
        }
    }

    /// Adds a layer
    pub fn add_layer(&mut self, layer: MapLayer) {
        self.layers.push(layer);
    }

    /// Exports to Leaflet HTML
    pub fn to_leaflet_html(&self) -> String {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str(&format!("  <title>{}</title>\n", self.title));
        html.push_str("  <meta charset=\"utf-8\" />\n");
        html.push_str(
            "  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n",
        );
        html.push_str("  <link rel=\"stylesheet\" href=\"https://unpkg.com/leaflet@1.9.4/dist/leaflet.css\" />\n");
        html.push_str(
            "  <script src=\"https://unpkg.com/leaflet@1.9.4/dist/leaflet.js\"></script>\n",
        );
        html.push_str("  <style>\n    #map { height: 600px; }\n  </style>\n");
        html.push_str("</head>\n<body>\n");
        html.push_str(&format!("  <h1>{}</h1>\n", self.title));
        html.push_str("  <div id=\"map\"></div>\n");
        html.push_str("  <script>\n");
        html.push_str(&format!(
            "    var map = L.map('map').setView([{}, {}], {});\n",
            self.center[1], self.center[0], self.zoom
        ));
        html.push_str("    L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {\n");
        html.push_str("      attribution: '&copy; OpenStreetMap contributors'\n");
        html.push_str("    }).addTo(map);\n\n");

        // Add layers
        for layer in &self.layers {
            if let Ok(geojson) = layer.features.to_json() {
                html.push_str(&format!(
                    "    var {} = {};\n",
                    layer.name.replace(' ', "_"),
                    geojson
                ));
                html.push_str(&format!(
                    "    L.geoJSON({}, {{\n",
                    layer.name.replace(' ', "_")
                ));
                html.push_str("      onEachFeature: function(feature, layer) {\n");
                html.push_str(
                    "        var popupContent = '<h3>' + feature.properties.name + '</h3>';\n",
                );
                html.push_str("        for (var key in feature.properties) {\n");
                html.push_str("          if (key !== 'name') {\n");
                html.push_str("            popupContent += '<b>' + key + ':</b> ' + feature.properties[key] + '<br>';\n");
                html.push_str("          }\n");
                html.push_str("        }\n");
                html.push_str("        layer.bindPopup(popupContent);\n");
                html.push_str("      }\n");
                html.push_str("    }).addTo(map);\n\n");
            }
        }

        html.push_str("  </script>\n");
        html.push_str("</body>\n</html>\n");

        html
    }

    /// Exports configuration to JSON
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geojson_point() {
        let geom = GeoJsonGeometry::point(139.6917, 35.6895);
        assert_eq!(geom.type_name, "Point");
    }

    #[test]
    fn test_geojson_feature() {
        let geom = GeoJsonGeometry::point(139.6917, 35.6895);
        let feature = GeoJsonFeature::new(geom)
            .with_id("tokyo-tower")
            .with_property("name", serde_json::json!("Tokyo Tower"));

        assert_eq!(feature.id, Some("tokyo-tower".to_string()));
        assert_eq!(
            feature.properties.get("name"),
            Some(&serde_json::json!("Tokyo Tower"))
        );
    }

    #[test]
    fn test_feature_collection() {
        let mut collection = GeoJsonFeatureCollection::new();
        let geom = GeoJsonGeometry::point(139.6917, 35.6895);
        let feature = GeoJsonFeature::new(geom).with_id("test");
        collection.add_feature(feature);

        assert_eq!(collection.features.len(), 1);
        assert_eq!(collection.type_name, "FeatureCollection");
    }

    #[test]
    fn test_wkt_point_parsing() {
        let wkt = "POINT (139.6917 35.6895)";
        let geom = parse_wkt_point(wkt);
        assert_eq!(geom.type_name, "Point");
    }

    #[test]
    fn test_wkt_polygon_parsing() {
        let wkt = "POLYGON ((0 0, 1 0, 1 1, 0 1, 0 0))";
        let geom = parse_wkt_polygon(wkt);
        assert_eq!(geom.type_name, "Polygon");
    }

    #[test]
    fn test_map_layer() {
        let mut layer = MapLayer::new("Jurisdictions", "jurisdiction");
        let geom = GeoJsonGeometry::point(139.6917, 35.6895);
        let feature = GeoJsonFeature::new(geom).with_id("tokyo");
        layer.add_feature(feature);

        assert_eq!(layer.features.features.len(), 1);
    }

    #[test]
    fn test_layer_style() {
        let style = LayerStyle::new()
            .with_fill_color("#FF0000")
            .with_stroke_color("#000000");

        assert_eq!(style.fill_color, Some("#FF0000".to_string()));
        assert_eq!(style.stroke_color, Some("#000000".to_string()));
    }

    #[test]
    fn test_map_config() {
        let mut config = MapConfig::new("Legal Zones Map", [139.6917, 35.6895], 10);
        let layer = MapLayer::new("Test Layer", "test");
        config.add_layer(layer);

        assert_eq!(config.layers.len(), 1);
        assert_eq!(config.title, "Legal Zones Map");
    }

    #[test]
    fn test_leaflet_html_generation() {
        let config = MapConfig::new("Test Map", [0.0, 0.0], 5);
        let html = config.to_leaflet_html();

        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("leaflet"));
        assert!(html.contains("Test Map"));
    }
}
