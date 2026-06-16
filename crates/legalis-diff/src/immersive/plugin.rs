//! A plugin system for custom scene renderers.
//!
//! [`VisualizationPlugin`] lets callers register their own renderers that turn a
//! positioned [`Scene3d`] (plus a [`Camera`]) into a string artifact — SVG, an
//! ASCII projection, a bespoke JSON, etc. It reuses [`crate::plugins::PluginMetadata`]
//! for consistency with the crate's other plugin systems
//! ([`crate::plugins`], [`crate::export_plugins`]).

use super::{Camera, Scene3d, Vec3};
use crate::plugins::PluginMetadata;
use crate::{DiffError, DiffResult};
use std::any::Any;
use std::collections::BTreeMap;
use std::fmt::Write as _;

/// A pluggable renderer for [`Scene3d`] visualizations.
pub trait VisualizationPlugin: Send + Sync {
    /// Plugin metadata (name, version, author, …).
    fn metadata(&self) -> &PluginMetadata;

    /// A short tag for the artifact kind this plugin emits (e.g. `"svg"`,
    /// `"ascii"`, `"json"`).
    fn output_kind(&self) -> &str;

    /// Optional initialisation hook.
    ///
    /// # Errors
    ///
    /// Implementations may reject invalid configuration.
    fn initialize(&mut self, config: &BTreeMap<String, String>) -> DiffResult<()> {
        let _ = config;
        Ok(())
    }

    /// Renders `scene` (as seen from `camera`) into a string artifact.
    ///
    /// # Errors
    ///
    /// Returns a [`DiffError`] if rendering fails (e.g. an empty scene).
    fn render(&self, scene: &Scene3d, camera: &Camera) -> DiffResult<String>;

    /// Upcast for downcasting back to the concrete type.
    fn as_any(&self) -> &dyn Any;
}

/// A registry of visualization plugins keyed by metadata name.
#[derive(Default)]
pub struct VizPluginRegistry {
    plugins: BTreeMap<String, Box<dyn VisualizationPlugin>>,
}

impl VizPluginRegistry {
    /// Creates an empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a plugin.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::Visualization`] if a plugin with the same name is
    /// already registered.
    pub fn register(&mut self, plugin: Box<dyn VisualizationPlugin>) -> DiffResult<()> {
        let name = plugin.metadata().name.clone();
        if self.plugins.contains_key(&name) {
            return Err(DiffError::Visualization(format!(
                "visualization plugin '{name}' already registered"
            )));
        }
        self.plugins.insert(name, plugin);
        Ok(())
    }

    /// Looks up a plugin by name.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&dyn VisualizationPlugin> {
        self.plugins.get(name).map(AsRef::as_ref)
    }

    /// Registered plugin names, sorted.
    #[must_use]
    pub fn names(&self) -> Vec<String> {
        self.plugins.keys().cloned().collect()
    }

    /// Number of registered plugins.
    #[must_use]
    pub fn len(&self) -> usize {
        self.plugins.len()
    }

    /// Returns `true` if no plugins are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.plugins.is_empty()
    }

    /// Renders `scene` with the named plugin.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::Visualization`] if no such plugin exists, or
    /// propagates the plugin's own render error.
    pub fn render_with(&self, name: &str, scene: &Scene3d, camera: &Camera) -> DiffResult<String> {
        let plugin = self.get(name).ok_or_else(|| {
            DiffError::Visualization(format!("no visualization plugin named '{name}'"))
        })?;
        plugin.render(scene, camera)
    }
}

/// A built-in plugin that emits a compact JSON wireframe (nodes + edges only).
pub struct WireframeJsonPlugin {
    metadata: PluginMetadata,
}

impl Default for WireframeJsonPlugin {
    fn default() -> Self {
        Self {
            metadata: PluginMetadata::new(
                "wireframe-json",
                "1.0.0",
                "legalis-diff",
                "Compact JSON wireframe (node ids/positions + edge endpoints).",
            ),
        }
    }
}

impl WireframeJsonPlugin {
    /// Creates the plugin.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl VisualizationPlugin for WireframeJsonPlugin {
    fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }

    fn output_kind(&self) -> &str {
        "json"
    }

    fn render(&self, scene: &Scene3d, _camera: &Camera) -> DiffResult<String> {
        if scene.is_empty() {
            return Err(DiffError::Visualization(
                "wireframe-json: empty scene".to_string(),
            ));
        }
        let nodes: Vec<serde_json::Value> = scene
            .nodes()
            .iter()
            .map(|n| {
                serde_json::json!({
                    "id": n.id,
                    "kind": n.kind.tag(),
                    "position": [n.position.x, n.position.y, n.position.z],
                })
            })
            .collect();
        let edges: Vec<serde_json::Value> = scene
            .edges()
            .iter()
            .map(|e| {
                serde_json::json!({
                    "source": e.source,
                    "target": e.target,
                    "kind": e.kind.tag(),
                })
            })
            .collect();
        let doc = serde_json::json!({ "nodes": nodes, "edges": edges });
        serde_json::to_string(&doc).map_err(|e| DiffError::SerializationError(e.to_string()))
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// A built-in plugin that orthographically projects the scene onto the camera's
/// view plane and rasterises it as ASCII art — a dependency-free preview useful
/// in terminals, logs and tests.
pub struct AsciiScatterPlugin {
    metadata: PluginMetadata,
    width: usize,
    height: usize,
}

impl Default for AsciiScatterPlugin {
    fn default() -> Self {
        Self {
            metadata: PluginMetadata::new(
                "ascii-scatter",
                "1.0.0",
                "legalis-diff",
                "Orthographic ASCII projection of the scene's node cloud.",
            ),
            width: 60,
            height: 24,
        }
    }
}

impl AsciiScatterPlugin {
    /// Creates the plugin with the default canvas size (60×24).
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Builder: sets the canvas size (each dimension clamped to at least 8).
    #[must_use]
    pub fn with_size(mut self, width: usize, height: usize) -> Self {
        self.width = width.max(8);
        self.height = height.max(8);
        self
    }

    /// The glyph used for a node of the given kind tag.
    fn glyph(kind_tag: &str) -> char {
        match kind_tag {
            "statute" => '@',
            "forest" => '#',
            "change-added" => '+',
            "change-removed" => '-',
            "change-modified" => '~',
            "change-reordered" => '>',
            "impact" => '!',
            "target-group" => 'o',
            _ => '*',
        }
    }
}

impl VisualizationPlugin for AsciiScatterPlugin {
    fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }

    fn output_kind(&self) -> &str {
        "ascii"
    }

    fn render(&self, scene: &Scene3d, camera: &Camera) -> DiffResult<String> {
        if scene.is_empty() {
            return Err(DiffError::Visualization(
                "ascii-scatter: empty scene".to_string(),
            ));
        }

        // Build an orthonormal camera basis (right, up, forward).
        let forward = camera.forward();
        let mut right = cross(forward, camera.up).normalized();
        if right.length() <= f64::EPSILON {
            // Degenerate up/forward — fall back to world X.
            right = Vec3::new(1.0, 0.0, 0.0);
        }
        let up = cross(right, forward).normalized();

        // Project every node into (u, v) plane coordinates.
        let projected: Vec<(f64, f64)> = scene
            .nodes()
            .iter()
            .map(|n| {
                let rel = n.position - camera.target;
                (rel.dot(right), rel.dot(up))
            })
            .collect();

        // Compute extents (guard against a single point / zero range).
        let (mut min_u, mut max_u, mut min_v, mut max_v) = (
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::INFINITY,
            f64::NEG_INFINITY,
        );
        for &(u, v) in &projected {
            min_u = min_u.min(u);
            max_u = max_u.max(u);
            min_v = min_v.min(v);
            max_v = max_v.max(v);
        }
        let range_u = (max_u - min_u).max(1e-6);
        let range_v = (max_v - min_v).max(1e-6);

        let mut grid = vec![vec![' '; self.width]; self.height];
        for (node, &(u, v)) in scene.nodes().iter().zip(projected.iter()) {
            let fx = (u - min_u) / range_u;
            let fy = (v - min_v) / range_v;
            let cx = ((fx * (self.width as f64 - 1.0)).round() as i64)
                .clamp(0, self.width as i64 - 1) as usize;
            // Invert Y so larger v is nearer the top.
            let cy = (((1.0 - fy) * (self.height as f64 - 1.0)).round() as i64)
                .clamp(0, self.height as i64 - 1) as usize;
            grid[cy][cx] = Self::glyph(node.kind.tag());
        }

        let mut out = String::new();
        for row in &grid {
            let line: String = row.iter().collect();
            let _ = writeln!(out, "{}", line.trim_end());
        }
        Ok(out)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// 3-D cross product (kept local; only the ASCII projector needs it).
fn cross(a: Vec3, b: Vec3) -> Vec3 {
    Vec3::new(
        a.y * b.z - a.z * b.y,
        a.z * b.x - a.x * b.z,
        a.x * b.y - a.y * b.x,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::immersive::{LayoutAlgorithm, LayoutParams, apply_layout, scene_from_diff};
    use legalis_core::{Effect, EffectType, Statute};

    fn laid_out_scene() -> Scene3d {
        let old = Statute::new("law-p", "Old", Effect::new(EffectType::Grant, "Benefit"));
        let mut new = old.clone();
        new.title = "New".to_string();
        new.effect = Effect::new(EffectType::Revoke, "Revoked");
        let d = crate::diff(&old, &new).expect("diff");
        let mut scene = scene_from_diff(&d);
        apply_layout(&mut scene, LayoutAlgorithm::Grid, &LayoutParams::default()).expect("layout");
        scene
    }

    #[test]
    fn test_registry_register_and_lookup() {
        let mut reg = VizPluginRegistry::new();
        assert!(reg.is_empty());
        reg.register(Box::new(WireframeJsonPlugin::new())).unwrap();
        reg.register(Box::new(AsciiScatterPlugin::new())).unwrap();
        assert_eq!(reg.len(), 2);
        assert_eq!(reg.names(), vec!["ascii-scatter", "wireframe-json"]);
        assert!(reg.get("wireframe-json").is_some());
        assert!(reg.get("missing").is_none());
    }

    #[test]
    fn test_registry_rejects_duplicate() {
        let mut reg = VizPluginRegistry::new();
        reg.register(Box::new(WireframeJsonPlugin::new())).unwrap();
        assert!(reg.register(Box::new(WireframeJsonPlugin::new())).is_err());
    }

    #[test]
    fn test_render_with_unknown_plugin_errors() {
        let reg = VizPluginRegistry::new();
        let scene = laid_out_scene();
        assert!(reg.render_with("nope", &scene, &Camera::default()).is_err());
    }

    #[test]
    fn test_wireframe_json_output_is_valid_json() {
        let scene = laid_out_scene();
        let plugin = WireframeJsonPlugin::new();
        assert_eq!(plugin.output_kind(), "json");
        let out = plugin.render(&scene, &Camera::default()).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&out).expect("valid json");
        assert_eq!(
            parsed["nodes"].as_array().map(Vec::len),
            Some(scene.node_count())
        );
        assert_eq!(
            parsed["edges"].as_array().map(Vec::len),
            Some(scene.edge_count())
        );
    }

    #[test]
    fn test_ascii_scatter_dimensions_and_content() {
        let scene = laid_out_scene();
        let plugin = AsciiScatterPlugin::new().with_size(40, 12);
        let camera = Camera::framing(&scene.bounds());
        let out = plugin.render(&scene, &camera).unwrap();
        let lines: Vec<&str> = out.lines().collect();
        assert_eq!(lines.len(), 12);
        // The statute root glyph '@' must appear somewhere.
        assert!(out.contains('@'));
    }

    #[test]
    fn test_ascii_scatter_min_size_clamped() {
        let plugin = AsciiScatterPlugin::new().with_size(1, 1);
        assert_eq!(plugin.width, 8);
        assert_eq!(plugin.height, 8);
    }

    #[test]
    fn test_plugins_reject_empty_scene() {
        let empty = Scene3d::new();
        assert!(
            WireframeJsonPlugin::new()
                .render(&empty, &Camera::default())
                .is_err()
        );
        assert!(
            AsciiScatterPlugin::new()
                .render(&empty, &Camera::default())
                .is_err()
        );
    }

    #[test]
    fn test_render_with_registered_plugin() {
        let mut reg = VizPluginRegistry::new();
        reg.register(Box::new(WireframeJsonPlugin::new())).unwrap();
        let scene = laid_out_scene();
        let out = reg
            .render_with("wireframe-json", &scene, &Camera::default())
            .unwrap();
        assert!(out.contains("\"nodes\""));
    }

    #[test]
    fn test_downcast_via_as_any() {
        let plugin = WireframeJsonPlugin::new();
        let dyn_ref: &dyn VisualizationPlugin = &plugin;
        assert!(
            dyn_ref
                .as_any()
                .downcast_ref::<WireframeJsonPlugin>()
                .is_some()
        );
        assert!(
            dyn_ref
                .as_any()
                .downcast_ref::<AsciiScatterPlugin>()
                .is_none()
        );
    }

    #[test]
    fn test_initialize_default_ok() {
        let mut plugin = AsciiScatterPlugin::new();
        let cfg = BTreeMap::new();
        assert!(plugin.initialize(&cfg).is_ok());
    }
}
