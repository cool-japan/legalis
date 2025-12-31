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
    writeln!(dot).unwrap();

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

    writeln!(dot).unwrap();

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

/// Real-time dashboard update for streaming visualizations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardUpdate {
    /// Timestamp of the update
    pub timestamp: u64,
    /// Update type
    pub update_type: UpdateType,
    /// Changed metrics
    pub metrics: HashMap<String, f64>,
    /// Optional message
    pub message: Option<String>,
}

/// Type of dashboard update
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UpdateType {
    /// Incremental update (add to existing data)
    Incremental,
    /// Full refresh (replace all data)
    FullRefresh,
    /// Status update (no data changes)
    Status,
    /// Error notification
    Error,
}

impl DashboardUpdate {
    /// Creates a new dashboard update
    pub fn new(timestamp: u64, update_type: UpdateType) -> Self {
        Self {
            timestamp,
            update_type,
            metrics: HashMap::new(),
            message: None,
        }
    }

    /// Adds a metric to the update
    pub fn with_metric(mut self, key: String, value: f64) -> Self {
        self.metrics.insert(key, value);
        self
    }

    /// Adds a message to the update
    pub fn with_message(mut self, message: String) -> Self {
        self.message = Some(message);
        self
    }
}

/// Real-time dashboard stream handler
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealTimeDashboard {
    /// Stream of updates
    pub updates: Vec<DashboardUpdate>,
    /// Current state
    pub current_state: DashboardData,
    /// Update interval (in milliseconds)
    pub update_interval_ms: u64,
}

impl RealTimeDashboard {
    /// Creates a new real-time dashboard
    pub fn new(initial_state: DashboardData, update_interval_ms: u64) -> Self {
        Self {
            updates: Vec::new(),
            current_state: initial_state,
            update_interval_ms,
        }
    }

    /// Adds an update to the stream
    pub fn push_update(&mut self, update: DashboardUpdate) {
        // Apply update to current state
        for (key, value) in &update.metrics {
            self.current_state.summary.insert(key.clone(), *value);
        }
        self.updates.push(update);
    }

    /// Gets all updates since a given timestamp
    pub fn get_updates_since(&self, timestamp: u64) -> Vec<&DashboardUpdate> {
        self.updates
            .iter()
            .filter(|u| u.timestamp > timestamp)
            .collect()
    }

    /// Clears old updates (keeping only recent ones)
    pub fn prune_updates(&mut self, keep_count: usize) {
        if self.updates.len() > keep_count {
            self.updates.drain(0..self.updates.len() - keep_count);
        }
    }
}

/// Animated time-lapse frame for temporal simulations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeLapseFrame {
    /// Frame number / time step
    pub frame: usize,
    /// Timestamp or time label
    pub time_label: String,
    /// Snapshot of metrics at this time
    pub metrics: HashMap<String, f64>,
    /// Snapshot of entity states
    pub entity_states: Vec<EntitySnapshot>,
    /// Events that occurred in this frame
    pub events: Vec<String>,
}

/// Snapshot of entity state at a point in time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitySnapshot {
    /// Entity ID
    pub id: String,
    /// Entity attributes at this time
    pub attributes: HashMap<String, serde_json::Value>,
    /// Geographic position (if applicable)
    pub position: Option<(f64, f64)>,
}

/// Animated time-lapse visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeLapseVisualization {
    /// All frames in the animation
    pub frames: Vec<TimeLapseFrame>,
    /// Frame rate (frames per second for playback)
    pub frame_rate: f64,
    /// Total duration (in simulation time units)
    pub duration: f64,
    /// Metadata about the simulation
    pub metadata: HashMap<String, String>,
}

impl TimeLapseVisualization {
    /// Creates a new time-lapse visualization
    pub fn new(frame_rate: f64) -> Self {
        Self {
            frames: Vec::new(),
            frame_rate,
            duration: 0.0,
            metadata: HashMap::new(),
        }
    }

    /// Adds a frame to the time-lapse
    pub fn add_frame(&mut self, frame: TimeLapseFrame) {
        self.duration = frame.frame as f64;
        self.frames.push(frame);
    }

    /// Gets frame at a specific index
    pub fn get_frame(&self, index: usize) -> Option<&TimeLapseFrame> {
        self.frames.get(index)
    }

    /// Gets frame count
    pub fn frame_count(&self) -> usize {
        self.frames.len()
    }

    /// Adds metadata
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

/// Interactive parameter configuration for tuning UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterConfig {
    /// Parameter name
    pub name: String,
    /// Parameter type
    pub param_type: ParameterType,
    /// Current value
    pub current_value: f64,
    /// Minimum value
    pub min_value: f64,
    /// Maximum value
    pub max_value: f64,
    /// Step size for adjustments
    pub step_size: f64,
    /// Description
    pub description: String,
    /// Category (for grouping in UI)
    pub category: String,
}

/// Type of parameter
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParameterType {
    /// Continuous numeric value
    Continuous,
    /// Discrete integer value
    Discrete,
    /// Boolean toggle
    Boolean,
    /// Percentage (0.0 to 1.0)
    Percentage,
}

impl ParameterConfig {
    /// Creates a new parameter configuration
    pub fn new(
        name: String,
        param_type: ParameterType,
        current_value: f64,
        min_value: f64,
        max_value: f64,
        step_size: f64,
    ) -> Self {
        Self {
            name,
            param_type,
            current_value,
            min_value,
            max_value,
            step_size,
            description: String::new(),
            category: "General".to_string(),
        }
    }

    /// Sets description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    /// Sets category
    pub fn with_category(mut self, category: String) -> Self {
        self.category = category;
        self
    }

    /// Validates and adjusts a value to be within bounds
    pub fn validate_value(&self, value: f64) -> f64 {
        let clamped = value.clamp(self.min_value, self.max_value);
        match self.param_type {
            ParameterType::Discrete => clamped.round(),
            ParameterType::Boolean => {
                if clamped >= 0.5 {
                    1.0
                } else {
                    0.0
                }
            }
            _ => clamped,
        }
    }
}

/// Interactive parameter tuning UI data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterTuningUI {
    /// All configurable parameters
    pub parameters: Vec<ParameterConfig>,
    /// Current simulation state
    pub current_state: Option<DashboardData>,
    /// Comparison with baseline (if available)
    pub baseline_comparison: Option<HashMap<String, f64>>,
}

impl ParameterTuningUI {
    /// Creates a new parameter tuning UI
    pub fn new() -> Self {
        Self {
            parameters: Vec::new(),
            current_state: None,
            baseline_comparison: None,
        }
    }

    /// Adds a parameter
    pub fn add_parameter(&mut self, param: ParameterConfig) {
        self.parameters.push(param);
    }

    /// Updates a parameter value
    pub fn update_parameter(&mut self, name: &str, value: f64) -> Option<f64> {
        if let Some(param) = self.parameters.iter_mut().find(|p| p.name == name) {
            let validated_value = param.validate_value(value);
            param.current_value = validated_value;
            Some(validated_value)
        } else {
            None
        }
    }

    /// Gets parameter by name
    pub fn get_parameter(&self, name: &str) -> Option<&ParameterConfig> {
        self.parameters.iter().find(|p| p.name == name)
    }

    /// Gets all parameters in a category
    pub fn get_parameters_by_category(&self, category: &str) -> Vec<&ParameterConfig> {
        self.parameters
            .iter()
            .filter(|p| p.category == category)
            .collect()
    }

    /// Updates current state
    pub fn update_state(&mut self, state: DashboardData) {
        self.current_state = Some(state);
    }

    /// Sets baseline for comparison
    pub fn set_baseline(&mut self, baseline: HashMap<String, f64>) {
        self.baseline_comparison = Some(baseline);
    }
}

impl Default for ParameterTuningUI {
    fn default() -> Self {
        Self::new()
    }
}

/// Heatmap data for visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeatmapData {
    /// X-axis labels
    pub x_labels: Vec<String>,
    /// Y-axis labels
    pub y_labels: Vec<String>,
    /// Values (row-major order: y then x)
    pub values: Vec<Vec<f64>>,
    /// Minimum value (for color scaling)
    pub min_value: f64,
    /// Maximum value (for color scaling)
    pub max_value: f64,
    /// Title
    pub title: String,
}

impl HeatmapData {
    /// Creates a new heatmap
    pub fn new(x_labels: Vec<String>, y_labels: Vec<String>, values: Vec<Vec<f64>>) -> Self {
        let min_value = values
            .iter()
            .flat_map(|row| row.iter())
            .cloned()
            .fold(f64::INFINITY, f64::min);
        let max_value = values
            .iter()
            .flat_map(|row| row.iter())
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);

        Self {
            x_labels,
            y_labels,
            values,
            min_value,
            max_value,
            title: String::new(),
        }
    }

    /// Sets title
    pub fn with_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    /// Gets value at position
    pub fn get_value(&self, x: usize, y: usize) -> Option<f64> {
        self.values.get(y).and_then(|row| row.get(x)).copied()
    }
}

/// Creates a heatmap from correlation matrix
pub fn create_correlation_heatmap(
    variable_names: Vec<String>,
    correlation_matrix: Vec<Vec<f64>>,
) -> HeatmapData {
    HeatmapData::new(variable_names.clone(), variable_names, correlation_matrix)
        .with_title("Correlation Matrix".to_string())
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

    // Real-time Dashboard Tests

    #[test]
    fn test_dashboard_update_creation() {
        let update = DashboardUpdate::new(1000, UpdateType::Incremental)
            .with_metric("test_metric".to_string(), 42.0)
            .with_message("Test update".to_string());

        assert_eq!(update.timestamp, 1000);
        assert_eq!(update.update_type, UpdateType::Incremental);
        assert_eq!(update.metrics.get("test_metric").unwrap(), &42.0);
        assert_eq!(update.message.as_ref().unwrap(), "Test update");
    }

    #[test]
    fn test_realtime_dashboard() {
        let initial_state = DashboardData {
            summary: HashMap::new(),
            timeseries: Vec::new(),
            geographic: Vec::new(),
            network: D3Graph {
                nodes: Vec::new(),
                links: Vec::new(),
            },
        };

        let mut dashboard = RealTimeDashboard::new(initial_state, 1000);

        let update1 = DashboardUpdate::new(1000, UpdateType::Incremental)
            .with_metric("count".to_string(), 10.0);
        dashboard.push_update(update1);

        let update2 = DashboardUpdate::new(2000, UpdateType::Incremental)
            .with_metric("count".to_string(), 20.0);
        dashboard.push_update(update2);

        assert_eq!(dashboard.updates.len(), 2);
        assert_eq!(dashboard.current_state.summary.get("count").unwrap(), &20.0);
    }

    #[test]
    fn test_realtime_dashboard_get_updates_since() {
        let initial_state = DashboardData {
            summary: HashMap::new(),
            timeseries: Vec::new(),
            geographic: Vec::new(),
            network: D3Graph {
                nodes: Vec::new(),
                links: Vec::new(),
            },
        };

        let mut dashboard = RealTimeDashboard::new(initial_state, 1000);

        dashboard.push_update(DashboardUpdate::new(1000, UpdateType::Incremental));
        dashboard.push_update(DashboardUpdate::new(2000, UpdateType::Incremental));
        dashboard.push_update(DashboardUpdate::new(3000, UpdateType::Incremental));

        let recent_updates = dashboard.get_updates_since(1500);
        assert_eq!(recent_updates.len(), 2);
        assert_eq!(recent_updates[0].timestamp, 2000);
        assert_eq!(recent_updates[1].timestamp, 3000);
    }

    #[test]
    fn test_realtime_dashboard_prune() {
        let initial_state = DashboardData {
            summary: HashMap::new(),
            timeseries: Vec::new(),
            geographic: Vec::new(),
            network: D3Graph {
                nodes: Vec::new(),
                links: Vec::new(),
            },
        };

        let mut dashboard = RealTimeDashboard::new(initial_state, 1000);

        for i in 0..10 {
            dashboard.push_update(DashboardUpdate::new(i * 1000, UpdateType::Incremental));
        }

        assert_eq!(dashboard.updates.len(), 10);

        dashboard.prune_updates(5);
        assert_eq!(dashboard.updates.len(), 5);
        assert_eq!(dashboard.updates[0].timestamp, 5000);
    }

    // Time-Lapse Visualization Tests

    #[test]
    fn test_timelapse_creation() {
        let timelapse = TimeLapseVisualization::new(30.0);
        assert_eq!(timelapse.frame_rate, 30.0);
        assert_eq!(timelapse.frame_count(), 0);
    }

    #[test]
    fn test_timelapse_add_frame() {
        let mut timelapse = TimeLapseVisualization::new(30.0);

        let frame = TimeLapseFrame {
            frame: 0,
            time_label: "t=0".to_string(),
            metrics: HashMap::new(),
            entity_states: Vec::new(),
            events: vec!["Simulation started".to_string()],
        };

        timelapse.add_frame(frame);
        assert_eq!(timelapse.frame_count(), 1);
        assert_eq!(timelapse.get_frame(0).unwrap().time_label, "t=0");
    }

    #[test]
    fn test_timelapse_with_metadata() {
        let timelapse = TimeLapseVisualization::new(30.0)
            .with_metadata("title".to_string(), "Test Simulation".to_string())
            .with_metadata("author".to_string(), "Test User".to_string());

        assert_eq!(timelapse.metadata.get("title").unwrap(), "Test Simulation");
        assert_eq!(timelapse.metadata.get("author").unwrap(), "Test User");
    }

    #[test]
    fn test_timelapse_multiple_frames() {
        let mut timelapse = TimeLapseVisualization::new(60.0);

        for i in 0..10 {
            let mut metrics = HashMap::new();
            metrics.insert("count".to_string(), i as f64 * 10.0);

            let frame = TimeLapseFrame {
                frame: i,
                time_label: format!("t={}", i),
                metrics,
                entity_states: Vec::new(),
                events: Vec::new(),
            };

            timelapse.add_frame(frame);
        }

        assert_eq!(timelapse.frame_count(), 10);
        assert_eq!(timelapse.duration, 9.0);
        assert_eq!(
            timelapse
                .get_frame(5)
                .unwrap()
                .metrics
                .get("count")
                .unwrap(),
            &50.0
        );
    }

    // Parameter Tuning UI Tests

    #[test]
    fn test_parameter_config_creation() {
        let param = ParameterConfig::new(
            "test_param".to_string(),
            ParameterType::Continuous,
            5.0,
            0.0,
            10.0,
            0.1,
        )
        .with_description("Test parameter".to_string())
        .with_category("Testing".to_string());

        assert_eq!(param.name, "test_param");
        assert_eq!(param.current_value, 5.0);
        assert_eq!(param.description, "Test parameter");
        assert_eq!(param.category, "Testing");
    }

    #[test]
    fn test_parameter_validate_continuous() {
        let param = ParameterConfig::new(
            "test".to_string(),
            ParameterType::Continuous,
            5.0,
            0.0,
            10.0,
            0.1,
        );

        assert_eq!(param.validate_value(5.5), 5.5);
        assert_eq!(param.validate_value(-1.0), 0.0); // Clamped to min
        assert_eq!(param.validate_value(15.0), 10.0); // Clamped to max
    }

    #[test]
    fn test_parameter_validate_discrete() {
        let param = ParameterConfig::new(
            "test".to_string(),
            ParameterType::Discrete,
            5.0,
            0.0,
            10.0,
            1.0,
        );

        assert_eq!(param.validate_value(5.7), 6.0); // Rounded
        assert_eq!(param.validate_value(5.3), 5.0); // Rounded
    }

    #[test]
    fn test_parameter_validate_boolean() {
        let param = ParameterConfig::new(
            "test".to_string(),
            ParameterType::Boolean,
            0.0,
            0.0,
            1.0,
            1.0,
        );

        assert_eq!(param.validate_value(0.6), 1.0); // >= 0.5 -> 1.0
        assert_eq!(param.validate_value(0.4), 0.0); // < 0.5 -> 0.0
    }

    #[test]
    fn test_parameter_tuning_ui() {
        let mut ui = ParameterTuningUI::new();

        let param1 = ParameterConfig::new(
            "param1".to_string(),
            ParameterType::Continuous,
            1.0,
            0.0,
            10.0,
            0.1,
        )
        .with_category("Category A".to_string());

        let param2 = ParameterConfig::new(
            "param2".to_string(),
            ParameterType::Discrete,
            5.0,
            0.0,
            10.0,
            1.0,
        )
        .with_category("Category B".to_string());

        ui.add_parameter(param1);
        ui.add_parameter(param2);

        assert_eq!(ui.parameters.len(), 2);
        assert!(ui.get_parameter("param1").is_some());
        assert!(ui.get_parameter("param3").is_none());
    }

    #[test]
    fn test_parameter_tuning_ui_update() {
        let mut ui = ParameterTuningUI::new();

        let param = ParameterConfig::new(
            "test".to_string(),
            ParameterType::Continuous,
            5.0,
            0.0,
            10.0,
            0.1,
        );

        ui.add_parameter(param);

        let updated_value = ui.update_parameter("test", 7.5);
        assert_eq!(updated_value, Some(7.5));
        assert_eq!(ui.get_parameter("test").unwrap().current_value, 7.5);
    }

    #[test]
    fn test_parameter_tuning_ui_by_category() {
        let mut ui = ParameterTuningUI::new();

        ui.add_parameter(
            ParameterConfig::new(
                "param1".to_string(),
                ParameterType::Continuous,
                1.0,
                0.0,
                10.0,
                0.1,
            )
            .with_category("Category A".to_string()),
        );

        ui.add_parameter(
            ParameterConfig::new(
                "param2".to_string(),
                ParameterType::Continuous,
                2.0,
                0.0,
                10.0,
                0.1,
            )
            .with_category("Category A".to_string()),
        );

        ui.add_parameter(
            ParameterConfig::new(
                "param3".to_string(),
                ParameterType::Continuous,
                3.0,
                0.0,
                10.0,
                0.1,
            )
            .with_category("Category B".to_string()),
        );

        let cat_a_params = ui.get_parameters_by_category("Category A");
        assert_eq!(cat_a_params.len(), 2);
    }

    // Heatmap Tests

    #[test]
    fn test_heatmap_creation() {
        let x_labels = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        let y_labels = vec!["1".to_string(), "2".to_string()];
        let values = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];

        let heatmap = HeatmapData::new(x_labels.clone(), y_labels.clone(), values.clone());

        assert_eq!(heatmap.x_labels.len(), 3);
        assert_eq!(heatmap.y_labels.len(), 2);
        assert_eq!(heatmap.min_value, 1.0);
        assert_eq!(heatmap.max_value, 6.0);
    }

    #[test]
    fn test_heatmap_get_value() {
        let x_labels = vec!["A".to_string(), "B".to_string()];
        let y_labels = vec!["1".to_string(), "2".to_string()];
        let values = vec![vec![10.0, 20.0], vec![30.0, 40.0]];

        let heatmap = HeatmapData::new(x_labels, y_labels, values);

        assert_eq!(heatmap.get_value(0, 0), Some(10.0));
        assert_eq!(heatmap.get_value(1, 0), Some(20.0));
        assert_eq!(heatmap.get_value(0, 1), Some(30.0));
        assert_eq!(heatmap.get_value(1, 1), Some(40.0));
        assert_eq!(heatmap.get_value(2, 0), None);
    }

    #[test]
    fn test_correlation_heatmap() {
        let variables = vec!["var1".to_string(), "var2".to_string(), "var3".to_string()];
        let correlation = vec![
            vec![1.0, 0.8, 0.3],
            vec![0.8, 1.0, 0.5],
            vec![0.3, 0.5, 1.0],
        ];

        let heatmap = create_correlation_heatmap(variables, correlation);

        assert_eq!(heatmap.title, "Correlation Matrix");
        assert_eq!(heatmap.x_labels.len(), 3);
        assert_eq!(heatmap.y_labels.len(), 3);
        assert_eq!(heatmap.get_value(0, 0), Some(1.0));
        assert_eq!(heatmap.get_value(1, 0), Some(0.8));
    }
}
