//! Visualization support for simulation results.
//!
//! This module provides export capabilities for various visualization formats:
//! - GraphViz DOT format for relationship graphs
//! - D3.js compatible JSON for interactive visualizations
//! - Geographic data export
//! - Time-series data formatting

use crate::{RelationshipGraph, SimulationMetrics};
use legalis_core::LegalEntity;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Write as FmtWrite;

/// GraphViz export options
#[derive(Debug, Clone)]
pub struct GraphVizOptions {
    /// Include entity attributes in nodes
    pub include_attributes: bool,
    /// Maximum number of attributes to display per node
    pub max_attributes: usize,
    /// Graph direction (TB, LR, RL, BT)
    pub rankdir: String,
    /// Node shape
    pub node_shape: String,
    /// Include edge labels
    pub include_edge_labels: bool,
}

impl Default for GraphVizOptions {
    fn default() -> Self {
        Self {
            include_attributes: true,
            max_attributes: 5,
            rankdir: "TB".to_string(),
            node_shape: "box".to_string(),
            include_edge_labels: true,
        }
    }
}

/// Export a relationship graph to GraphViz DOT format
pub fn export_to_graphviz(
    graph: &RelationshipGraph,
    entities: &[Box<dyn LegalEntity>],
    options: &GraphVizOptions,
) -> String {
    let mut dot = String::new();

    writeln!(dot, "digraph Relationships {{").unwrap();
    writeln!(dot, "  rankdir={};", options.rankdir).unwrap();
    writeln!(dot, "  node [shape={}];", options.node_shape).unwrap();
    writeln!(dot, "").unwrap();

    // Add nodes
    for entity in entities {
        let id = entity.id();
        let mut label = format!("ID: {}", id);

        if options.include_attributes {
            // Add a few common attributes if they exist
            for attr_name in &["name", "age", "income", "region"] {
                if let Some(value) = entity.get_attribute(attr_name) {
                    label.push_str(&format!("\\n{}: {}", attr_name, value));
                }
            }
        }

        writeln!(dot, "  \"{}\" [label=\"{}\"];", id, label).unwrap();
    }

    writeln!(dot, "").unwrap();

    // Add edges from graph
    for rel in graph.all_relationships() {
        let edge_label = if options.include_edge_labels {
            format!(" [label=\"{:?}\"]", rel.relationship_type)
        } else {
            String::new()
        };

        writeln!(dot, "  \"{}\" -> \"{}\"{};", rel.from, rel.to, edge_label).unwrap();
    }

    writeln!(dot, "}}").unwrap();
    dot
}

/// D3.js compatible node data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct D3Node {
    pub id: String,
    pub group: i32,
    pub attributes: HashMap<String, serde_json::Value>,
}

/// D3.js compatible link data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct D3Link {
    pub source: String,
    pub target: String,
    #[serde(rename = "type")]
    pub link_type: String,
    pub value: f64,
}

/// D3.js force-directed graph data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct D3Graph {
    pub nodes: Vec<D3Node>,
    pub links: Vec<D3Link>,
}

/// Export relationship graph to D3.js compatible JSON
pub fn export_to_d3(graph: &RelationshipGraph, entities: &[Box<dyn LegalEntity>]) -> D3Graph {
    let nodes: Vec<D3Node> = entities
        .iter()
        .map(|e| {
            // Collect common attributes
            let mut attributes = HashMap::new();
            for attr_name in &["name", "age", "income", "region", "occupation"] {
                if let Some(value) = e.get_attribute(attr_name) {
                    attributes.insert(attr_name.to_string(), serde_json::json!(value));
                }
            }

            D3Node {
                id: e.id().to_string(),
                group: 1, // Default group, can be customized based on entity type
                attributes,
            }
        })
        .collect();

    let links: Vec<D3Link> = graph
        .all_relationships()
        .iter()
        .map(|rel| D3Link {
            source: rel.from.to_string(),
            target: rel.to.to_string(),
            link_type: format!("{:?}", rel.relationship_type),
            value: rel.strength,
        })
        .collect();

    D3Graph { nodes, links }
}

/// Time-series data point for D3 visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSeriesPoint {
    pub time: String,
    pub value: f64,
    pub series: String,
}

/// Export metrics to time-series format for D3
pub fn export_metrics_to_timeseries(
    metrics: &SimulationMetrics,
    metric_name: &str,
) -> Vec<TimeSeriesPoint> {
    let mut points = Vec::new();

    // Create basic time-series from metrics
    points.push(TimeSeriesPoint {
        time: "t0".to_string(),
        value: metrics.deterministic_count as f64,
        series: format!("{}_deterministic", metric_name),
    });

    points.push(TimeSeriesPoint {
        time: "t0".to_string(),
        value: metrics.discretion_count as f64,
        series: format!("{}_discretion", metric_name),
    });

    points.push(TimeSeriesPoint {
        time: "t0".to_string(),
        value: metrics.void_count as f64,
        series: format!("{}_void", metric_name),
    });

    points
}

/// Geographic data point for map visualizations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoDataPoint {
    pub latitude: f64,
    pub longitude: f64,
    pub value: f64,
    pub label: String,
    pub properties: HashMap<String, serde_json::Value>,
}

/// Export entities with geographic attributes to GeoJSON-like format
pub fn export_geographic_data(entities: &[Box<dyn LegalEntity>]) -> Vec<GeoDataPoint> {
    let mut points = Vec::new();

    for entity in entities {
        // Try to get latitude and longitude
        let lat = entity
            .get_attribute("latitude")
            .and_then(|s| s.parse::<f64>().ok());
        let lon = entity
            .get_attribute("longitude")
            .and_then(|s| s.parse::<f64>().ok());

        if let (Some(latitude), Some(longitude)) = (lat, lon) {
            // Collect other properties
            let mut properties = HashMap::new();
            for attr_name in &["name", "region", "city", "population"] {
                if let Some(value) = entity.get_attribute(attr_name) {
                    properties.insert(attr_name.to_string(), serde_json::json!(value));
                }
            }

            points.push(GeoDataPoint {
                latitude,
                longitude,
                value: 1.0, // Default value
                label: entity.id().to_string(),
                properties,
            });
        }
    }

    points
}

/// Dashboard data structure for interactive visualizations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardData {
    pub summary: HashMap<String, f64>,
    pub timeseries: Vec<TimeSeriesPoint>,
    pub geographic: Vec<GeoDataPoint>,
    pub network: D3Graph,
}

/// Create comprehensive dashboard data from simulation results
pub fn create_dashboard_data(
    metrics: &SimulationMetrics,
    graph: &RelationshipGraph,
    entities: &[Box<dyn LegalEntity>],
) -> DashboardData {
    let mut summary = HashMap::new();
    summary.insert(
        "total_applications".to_string(),
        metrics.total_applications as f64,
    );
    summary.insert(
        "deterministic".to_string(),
        metrics.deterministic_count as f64,
    );
    summary.insert("discretion".to_string(), metrics.discretion_count as f64);
    summary.insert("void".to_string(), metrics.void_count as f64);
    summary.insert(
        "deterministic_ratio".to_string(),
        metrics.deterministic_ratio(),
    );
    summary.insert("discretion_ratio".to_string(), metrics.discretion_ratio());

    DashboardData {
        summary,
        timeseries: export_metrics_to_timeseries(metrics, "main"),
        geographic: export_geographic_data(entities),
        network: export_to_d3(graph, entities),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Relationship, RelationshipType};
    use legalis_core::BasicEntity;
    use uuid::Uuid;

    #[test]
    fn test_graphviz_export() {
        let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();

        let mut entity1 = BasicEntity::with_id(id1);
        entity1.set_attribute("name", "Alice".to_string());

        let mut entity2 = BasicEntity::with_id(id2);
        entity2.set_attribute("name", "Bob".to_string());

        let entities: Vec<Box<dyn LegalEntity>> = vec![Box::new(entity1), Box::new(entity2)];

        let mut graph = RelationshipGraph::new();
        graph.add_relationship(Relationship::new(id1, id2, RelationshipType::Spouse));

        let options = GraphVizOptions::default();
        let dot = export_to_graphviz(&graph, &entities, &options);

        assert!(dot.contains("digraph Relationships"));
        assert!(dot.contains("Alice"));
        assert!(dot.contains("Bob"));
        assert!(dot.contains("Spouse"));
    }

    #[test]
    fn test_d3_export() {
        let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();

        let mut entity1 = BasicEntity::with_id(id1);
        entity1.set_attribute("name", "Alice".to_string());

        let mut entity2 = BasicEntity::with_id(id2);
        entity2.set_attribute("name", "Bob".to_string());

        let entities: Vec<Box<dyn LegalEntity>> = vec![Box::new(entity1), Box::new(entity2)];

        let mut graph = RelationshipGraph::new();
        graph.add_relationship(
            Relationship::new(id1, id2, RelationshipType::Spouse).with_strength(0.8),
        );

        let d3_graph = export_to_d3(&graph, &entities);

        assert_eq!(d3_graph.nodes.len(), 2);
        // RelationshipGraph stores only one relationship, even for symmetric types
        assert_eq!(d3_graph.links.len(), 1);
        assert_eq!(d3_graph.links[0].value, 0.8);
    }

    #[test]
    fn test_geographic_export() {
        let id = Uuid::new_v4();
        let mut entity = BasicEntity::with_id(id);
        entity.set_attribute("latitude", "35.6762".to_string());
        entity.set_attribute("longitude", "139.6503".to_string());
        entity.set_attribute("city", "Tokyo".to_string());

        let entities: Vec<Box<dyn LegalEntity>> = vec![Box::new(entity)];
        let geo_data = export_geographic_data(&entities);

        assert_eq!(geo_data.len(), 1);
        assert_eq!(geo_data[0].latitude, 35.6762);
        assert_eq!(geo_data[0].longitude, 139.6503);
    }

    #[test]
    fn test_dashboard_creation() {
        let metrics = SimulationMetrics {
            total_applications: 100,
            deterministic_count: 60,
            discretion_count: 30,
            void_count: 10,
            statute_metrics: HashMap::new(),
            discretion_agents: Vec::new(),
        };

        let id = Uuid::new_v4();
        let mut entity = BasicEntity::with_id(id);
        entity.set_attribute("name", "Test".to_string());

        let entities: Vec<Box<dyn LegalEntity>> = vec![Box::new(entity)];
        let graph = RelationshipGraph::new();

        let dashboard = create_dashboard_data(&metrics, &graph, &entities);

        assert_eq!(dashboard.summary.get("total_applications").unwrap(), &100.0);
        assert_eq!(dashboard.summary.get("deterministic").unwrap(), &60.0);
        assert_eq!(dashboard.network.nodes.len(), 1);
    }
}
