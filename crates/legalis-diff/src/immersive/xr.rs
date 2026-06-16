//! Exporters that serialise a [`Scene3d`] into immersive scene-description
//! formats consumable by VR/AR runtimes.
//!
//! The pure-Rust part — turning the scene graph into a standards-shaped scene
//! document — lives here. Actually presenting the document in a head-set (the
//! WebXR session, GPU rasterisation) is the deferred external binding handled by
//! a future [`super::SceneRenderer`] implementation.

use super::{Camera, Scene3d, SceneEdge, SceneNode, Vec3};
use crate::{DiffError, DiffResult};
use serde::{Deserialize, Serialize};
use std::fmt::Write as _;

/// The immersive output format to emit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum XrFormat {
    /// [A-Frame](https://aframe.io) HTML — a WebXR-ready document of
    /// `<a-sphere>`/`<a-cylinder>` entities.
    AFrame,
    /// [X3D](https://www.web3d.org/x3d) XML — an ISO scene graph of
    /// `Transform`/`Shape` nodes.
    X3d,
    /// A glTF-like JSON manifest (nodes with translation/scale + colour extras).
    GltfJson,
}

impl XrFormat {
    /// The conventional file extension for this format.
    #[must_use]
    pub fn extension(&self) -> &'static str {
        match self {
            XrFormat::AFrame => "html",
            XrFormat::X3d => "x3d",
            XrFormat::GltfJson => "gltf.json",
        }
    }

    /// The MIME type for this format.
    #[must_use]
    pub fn mime_type(&self) -> &'static str {
        match self {
            XrFormat::AFrame => "text/html",
            XrFormat::X3d => "model/x3d+xml",
            XrFormat::GltfJson => "model/gltf+json",
        }
    }
}

/// Serialises `scene` (positioned by [`super::apply_layout`]) into the given
/// immersive `format`, using `camera` for the initial viewpoint.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::diff;
/// use legalis_diff::immersive::{scene_from_diff, apply_layout, export_scene, Camera, XrFormat,
///     LayoutAlgorithm, LayoutParams};
///
/// let old = Statute::new("law", "T", Effect::new(EffectType::Grant, "B"));
/// let mut new = old.clone();
/// new.effect = Effect::new(EffectType::Revoke, "R");
/// let d = diff(&old, &new).unwrap();
///
/// let mut scene = scene_from_diff(&d);
/// apply_layout(&mut scene, LayoutAlgorithm::FibonacciSphere, &LayoutParams::default()).unwrap();
/// let html = export_scene(&scene, &Camera::framing(&scene.bounds()), XrFormat::AFrame).unwrap();
/// assert!(html.contains("<a-scene"));
/// ```
///
/// # Errors
///
/// Returns [`DiffError::Visualization`] for an empty scene, or
/// [`DiffError::SerializationError`] if JSON serialisation fails.
pub fn export_scene(scene: &Scene3d, camera: &Camera, format: XrFormat) -> DiffResult<String> {
    if scene.is_empty() {
        return Err(DiffError::Visualization(
            "cannot export an empty scene".to_string(),
        ));
    }
    match format {
        XrFormat::AFrame => Ok(export_aframe(scene, camera)),
        XrFormat::X3d => Ok(export_x3d(scene, camera)),
        XrFormat::GltfJson => export_gltf_json(scene, camera),
    }
}

/// Escapes the five XML/HTML special characters.
fn xml_escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for ch in text.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(ch),
        }
    }
    out
}

/// Formats a vector as space-separated coordinates (the X3D/A-Frame convention).
fn fmt_xyz(v: Vec3) -> String {
    format!("{:.4} {:.4} {:.4}", v.x, v.y, v.z)
}

/// Builds an A-Frame WebXR HTML document.
fn export_aframe(scene: &Scene3d, camera: &Camera) -> String {
    let mut html = String::new();
    html.push_str("<!DOCTYPE html>\n<html>\n<head>\n  <meta charset=\"utf-8\">\n");
    html.push_str("  <title>Statute Diff — Immersive View</title>\n");
    html.push_str("  <script src=\"https://aframe.io/releases/1.5.0/aframe.min.js\"></script>\n");
    html.push_str("</head>\n<body>\n  <a-scene background=\"color: #101418\">\n");

    // Camera rig.
    let _ = writeln!(
        html,
        "    <a-entity camera look-controls wasd-controls position=\"{}\"></a-entity>",
        fmt_xyz(camera.position)
    );

    // Edges as thin cylinders between node centres.
    for edge in scene.edges() {
        if let (Some(a), Some(b)) = (scene.node(&edge.source), scene.node(&edge.target)) {
            let mid = a.position.midpoint(b.position);
            let len = a.position.distance(b.position).max(0.001);
            let _ = writeln!(
                html,
                "    <a-cylinder position=\"{}\" radius=\"0.03\" height=\"{:.4}\" \
                 color=\"#888888\" opacity=\"0.5\" data-edge=\"{}\"></a-cylinder>",
                fmt_xyz(mid),
                len,
                xml_escape(edge.kind.tag())
            );
        }
    }

    // Nodes as labelled spheres.
    for node in scene.nodes() {
        let _ = writeln!(
            html,
            "    <a-sphere position=\"{}\" radius=\"{:.4}\" color=\"{}\" data-id=\"{}\" \
             data-kind=\"{}\">",
            fmt_xyz(node.position),
            node.size * 0.4,
            node.color.to_hex(),
            xml_escape(&node.id),
            xml_escape(node.kind.tag())
        );
        let _ = writeln!(
            html,
            "      <a-text value=\"{}\" align=\"center\" position=\"0 {:.4} 0\" \
             scale=\"2 2 2\" color=\"#ffffff\"></a-text>",
            xml_escape(&node.label),
            node.size * 0.5 + 0.3
        );
        html.push_str("    </a-sphere>\n");
    }

    html.push_str("    <a-light type=\"ambient\" color=\"#bbbbbb\"></a-light>\n");
    html.push_str("    <a-light type=\"point\" intensity=\"0.8\" position=\"2 4 4\"></a-light>\n");
    html.push_str("  </a-scene>\n</body>\n</html>\n");
    html
}

/// Builds an X3D XML scene document.
fn export_x3d(scene: &Scene3d, camera: &Camera) -> String {
    let mut xml = String::new();
    xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml.push_str(
        "<X3D profile=\"Interchange\" version=\"4.0\" xmlns:xsd=\"http://www.w3.org/2001/XMLSchema-instance\">\n",
    );
    xml.push_str("  <Scene>\n");
    let _ = writeln!(
        xml,
        "    <Viewpoint position=\"{}\" description=\"diff-overview\"/>",
        fmt_xyz(camera.position)
    );

    // Edges as line segments via IndexedLineSet would need a shared coordinate
    // pool; emit one LineSet per edge for clarity/robustness.
    for edge in scene.edges() {
        if let (Some(a), Some(b)) = (scene.node(&edge.source), scene.node(&edge.target)) {
            line_segment_x3d(&mut xml, a, b);
        }
    }

    for node in scene.nodes() {
        node_sphere_x3d(&mut xml, node);
    }

    xml.push_str("  </Scene>\n</X3D>\n");
    xml
}

/// Appends a single line segment (an edge) to the X3D document.
fn line_segment_x3d(xml: &mut String, a: &SceneNode, b: &SceneNode) {
    xml.push_str("    <Shape>\n");
    xml.push_str("      <LineSet vertexCount=\"2\">\n");
    let _ = writeln!(
        xml,
        "        <Coordinate point=\"{} {}\"/>",
        fmt_xyz(a.position),
        fmt_xyz(b.position)
    );
    xml.push_str("      </LineSet>\n");
    xml.push_str("    </Shape>\n");
}

/// Appends a node sphere (a `Transform` wrapping a coloured `Sphere`) to X3D.
fn node_sphere_x3d(xml: &mut String, node: &SceneNode) {
    let _ = writeln!(
        xml,
        "    <Transform translation=\"{}\" DEF=\"{}\">",
        fmt_xyz(node.position),
        xml_escape(&node.id)
    );
    xml.push_str("      <Shape>\n");
    let _ = writeln!(
        xml,
        "        <Appearance><Material diffuseColor=\"{}\"/></Appearance>",
        rgb_floats(node)
    );
    let _ = writeln!(xml, "        <Sphere radius=\"{:.4}\"/>", node.size * 0.4);
    xml.push_str("      </Shape>\n");
    xml.push_str("    </Transform>\n");
}

/// X3D wants colours as space-separated floats in `[0,1]`.
fn rgb_floats(node: &SceneNode) -> String {
    format!(
        "{:.3} {:.3} {:.3}",
        f64::from(node.color.r) / 255.0,
        f64::from(node.color.g) / 255.0,
        f64::from(node.color.b) / 255.0,
    )
}

/// A glTF-like node in the JSON manifest.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct GltfNode {
    name: String,
    translation: [f64; 3],
    scale: [f64; 3],
    extras: GltfExtras,
}

/// glTF `extras` carrying diff-specific styling.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct GltfExtras {
    id: String,
    kind: String,
    color: String,
    label: String,
}

/// A glTF-like edge (custom extension).
#[derive(Debug, Clone, Serialize, Deserialize)]
struct GltfEdge {
    source: String,
    target: String,
    kind: String,
}

/// The top-level glTF-like manifest.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct GltfManifest {
    asset_version: String,
    generator: String,
    camera_position: [f64; 3],
    camera_target: [f64; 3],
    nodes: Vec<GltfNode>,
    #[serde(rename = "legalis_edges")]
    edges: Vec<GltfEdge>,
}

/// Builds the glTF-like JSON manifest.
fn export_gltf_json(scene: &Scene3d, camera: &Camera) -> DiffResult<String> {
    let nodes: Vec<GltfNode> = scene
        .nodes()
        .iter()
        .map(|node| GltfNode {
            name: node.label.clone(),
            translation: [node.position.x, node.position.y, node.position.z],
            scale: [node.size * 0.4; 3],
            extras: GltfExtras {
                id: node.id.clone(),
                kind: node.kind.tag().to_string(),
                color: node.color.to_hex(),
                label: node.label.clone(),
            },
        })
        .collect();
    let edges: Vec<GltfEdge> = scene
        .edges()
        .iter()
        .map(|edge: &SceneEdge| GltfEdge {
            source: edge.source.clone(),
            target: edge.target.clone(),
            kind: edge.kind.tag().to_string(),
        })
        .collect();
    let manifest = GltfManifest {
        asset_version: "2.0".to_string(),
        generator: "legalis-diff/immersive".to_string(),
        camera_position: [camera.position.x, camera.position.y, camera.position.z],
        camera_target: [camera.target.x, camera.target.y, camera.target.z],
        nodes,
        edges,
    };
    serde_json::to_string_pretty(&manifest)
        .map_err(|e| DiffError::SerializationError(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::immersive::{
        LayoutAlgorithm, LayoutParams, NodeKind, SceneNode, apply_layout, scene_from_diff,
    };
    use legalis_core::{Effect, EffectType, Statute};

    fn laid_out_scene() -> Scene3d {
        let old = Statute::new("law-x", "Old", Effect::new(EffectType::Grant, "Benefit"));
        let mut new = old.clone();
        new.title = "New Title".to_string();
        new.effect = Effect::new(EffectType::Revoke, "Revoked");
        let d = crate::diff(&old, &new).expect("diff");
        let mut scene = scene_from_diff(&d);
        apply_layout(
            &mut scene,
            LayoutAlgorithm::FibonacciSphere,
            &LayoutParams::default(),
        )
        .expect("layout");
        scene
    }

    #[test]
    fn test_aframe_export_has_scene_and_nodes() {
        let scene = laid_out_scene();
        let html = export_scene(&scene, &Camera::default(), XrFormat::AFrame).unwrap();
        assert!(html.contains("<a-scene"));
        assert!(html.contains("<a-sphere"));
        assert!(html.contains("data-id=\"law-x\""));
    }

    #[test]
    fn test_aframe_escapes_special_chars() {
        let mut scene = laid_out_scene();
        // A node whose label/id carry XML-significant characters.
        scene.add_node(SceneNode::new(
            "danger<&>",
            "Tag <x> & \"y\"",
            NodeKind::Impact,
        ));
        let html = export_scene(&scene, &Camera::default(), XrFormat::AFrame).unwrap();
        // The label must be escaped, never emitted raw.
        assert!(html.contains("Tag &lt;x&gt; &amp; &quot;y&quot;"));
        assert!(!html.contains("Tag <x> & \"y\""));
        // The id used in the data-id attribute must be escaped too.
        assert!(html.contains("data-id=\"danger&lt;&amp;&gt;\""));
    }

    #[test]
    fn test_x3d_export_is_xml() {
        let scene = laid_out_scene();
        let xml = export_scene(&scene, &Camera::default(), XrFormat::X3d).unwrap();
        assert!(xml.starts_with("<?xml"));
        assert!(xml.contains("<X3D"));
        assert!(xml.contains("<Sphere"));
        assert!(xml.contains("<Viewpoint"));
    }

    #[test]
    fn test_gltf_json_roundtrips() {
        let scene = laid_out_scene();
        let json = export_scene(&scene, &Camera::default(), XrFormat::GltfJson).unwrap();
        let parsed: GltfManifest = serde_json::from_str(&json).expect("valid json");
        assert_eq!(parsed.nodes.len(), scene.node_count());
        assert_eq!(parsed.edges.len(), scene.edge_count());
        assert_eq!(parsed.asset_version, "2.0");
    }

    #[test]
    fn test_export_empty_scene_errors() {
        let scene = Scene3d::new();
        assert!(export_scene(&scene, &Camera::default(), XrFormat::AFrame).is_err());
    }

    #[test]
    fn test_format_extensions_and_mime() {
        assert_eq!(XrFormat::AFrame.extension(), "html");
        assert_eq!(XrFormat::X3d.mime_type(), "model/x3d+xml");
        assert_eq!(XrFormat::GltfJson.extension(), "gltf.json");
    }

    #[test]
    fn test_xml_escape_helper() {
        assert_eq!(xml_escape("a<b>&\"'"), "a&lt;b&gt;&amp;&quot;&apos;");
    }
}
