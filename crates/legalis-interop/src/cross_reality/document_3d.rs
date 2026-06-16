//! 3D legal document format.
//!
//! Projects a legal corpus into a 3D scene graph in which every statute becomes
//! a panel ("card") positioned by a deterministic layout, coloured by effect
//! type, and connected to the provisions it derives from by typed edges. The
//! scene round-trips losslessly through embedded provenance and can also be
//! rendered to an X3D-like XML projection (the ISO standard for declarative 3D),
//! suitable for web and headset viewers.

use super::{
    Aabb, Color, SceneLayout, Transform, Vec3, condition_salience, effect_color, layout_transform,
    round3,
};
use crate::cross_reality::vr_ar::schema_matches;
use crate::formats_nextgen::{
    StructuredStatute, build_structured, effect_type_to_str, render_statute_markdown,
};
use crate::{
    ConversionReport, FormatExporter, FormatImporter, InteropError, InteropResult, LegalFormat,
};
use legalis_core::Statute;
use serde::{Deserialize, Serialize};

/// Schema identifier for the 3D document format.
pub const SCHEMA: &str = "legalis.spatial-document-3d/v1";

/// Configuration for 3D scene generation.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Document3DConfig {
    /// Spatial arrangement of statute panels.
    pub layout: SceneLayout,
    /// Nominal spacing (metres) between panels.
    pub spacing: f64,
    /// Base panel width (metres).
    pub panel_width: f64,
    /// Base panel height (metres).
    pub panel_height: f64,
    /// Panel thickness (metres).
    pub panel_depth: f64,
}

impl Default for Document3DConfig {
    fn default() -> Self {
        Self {
            layout: SceneLayout::Grid,
            spacing: 3.0,
            panel_width: 1.6,
            panel_height: 0.9,
            panel_depth: 0.05,
        }
    }
}

/// Box dimensions of a statute panel.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PanelGeometry {
    /// Width (metres).
    pub width: f64,
    /// Height (metres).
    pub height: f64,
    /// Depth / thickness (metres).
    pub depth: f64,
}

/// A node in the 3D scene graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node3D {
    /// Stable node identifier.
    pub id: String,
    /// Source statute identifier.
    pub source_id: String,
    /// Display label (statute title).
    pub label: String,
    /// Placement transform.
    pub transform: Transform,
    /// Panel geometry.
    pub panel: PanelGeometry,
    /// Markdown text rendered on / beside the panel.
    pub text: String,
    /// Panel colour (derived from effect type).
    pub color: Color,
}

/// A directed edge between two scene nodes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneEdge {
    /// Source node id.
    pub from: String,
    /// Target node id.
    pub to: String,
    /// Edge kind (e.g. `derives_from`).
    pub kind: String,
}

/// A 3D scene-graph representation of a legal document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scene3D {
    /// Schema identifier ([`SCHEMA`]).
    pub schema: String,
    /// Up-axis convention.
    pub up_axis: String,
    /// Scene nodes (one per statute).
    pub nodes: Vec<Node3D>,
    /// Derivation edges between nodes.
    pub edges: Vec<SceneEdge>,
    /// Tight bounding box of all node positions.
    pub bounds: Aabb,
    /// Structured provenance enabling lossless reconstruction.
    pub provenance: Vec<StructuredStatute>,
}

impl Scene3D {
    /// Builds a 3D scene from statutes using the given configuration.
    pub fn build(statutes: &[Statute], config: Document3DConfig) -> Self {
        let count = statutes.len();
        let nodes: Vec<Node3D> = statutes
            .iter()
            .enumerate()
            .map(|(index, statute)| {
                let mut transform = layout_transform(index, count, config.layout, config.spacing);
                let salience = condition_salience(statute.preconditions.len());
                transform.scale = Vec3::new(1.0, round3(salience), 1.0);
                let effect = effect_type_to_str(&statute.effect.effect_type);
                Node3D {
                    id: node_id(index, statute),
                    source_id: statute.id.clone(),
                    label: statute.title.clone(),
                    transform,
                    panel: PanelGeometry {
                        width: config.panel_width,
                        height: round3(config.panel_height * salience),
                        depth: config.panel_depth,
                    },
                    text: render_statute_markdown(statute),
                    color: effect_color(effect),
                }
            })
            .collect();

        let edges = derivation_edges(statutes, &nodes);
        let positions: Vec<Vec3> = nodes.iter().map(|node| node.transform.position).collect();

        Self {
            schema: SCHEMA.to_string(),
            up_axis: "y".to_string(),
            nodes,
            edges,
            bounds: Aabb::from_points(&positions),
            provenance: build_structured(statutes),
        }
    }

    /// Number of nodes in the scene.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Reconstructs the underlying statutes from provenance.
    pub fn to_statutes(&self) -> Vec<Statute> {
        self.provenance
            .iter()
            .map(StructuredStatute::to_statute)
            .collect()
    }

    /// Serialises the scene to pretty JSON.
    pub fn to_json(&self) -> InteropResult<String> {
        serde_json::to_string_pretty(self).map_err(|error| {
            InteropError::SerializationError(format!("Failed to serialize 3D scene: {error}"))
        })
    }

    /// Parses a scene from JSON.
    pub fn from_json(source: &str) -> InteropResult<Self> {
        serde_json::from_str(source)
            .map_err(|error| InteropError::ParseError(format!("Failed to parse 3D JSON: {error}")))
    }

    /// Renders the scene to an X3D-like XML projection.
    ///
    /// This is a lossy visualisation view (geometry and labels only, no
    /// provenance); the JSON form remains the canonical round-trippable format.
    pub fn to_x3d(&self) -> String {
        let mut out = String::new();
        out.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        out.push_str("<X3D profile=\"Interchange\" version=\"4.0\">\n");
        out.push_str("  <Scene>\n");
        for node in &self.nodes {
            let position = node.transform.position;
            let rotation = node.transform.rotation;
            out.push_str(&format!(
                "    <Transform DEF=\"{}\" translation=\"{} {} {}\" rotation=\"{} {} {} {}\" scale=\"{} {} {}\">\n",
                xml_escape(&node.id),
                round3(position.x),
                round3(position.y),
                round3(position.z),
                round3(rotation.x),
                round3(rotation.y),
                round3(rotation.z),
                round3(rotation.w),
                round3(node.transform.scale.x),
                round3(node.transform.scale.y),
                round3(node.transform.scale.z),
            ));
            out.push_str("      <Shape>\n");
            out.push_str(&format!(
                "        <Appearance><Material diffuseColor=\"{} {} {}\" transparency=\"{}\"/></Appearance>\n",
                round3(node.color.r),
                round3(node.color.g),
                round3(node.color.b),
                round3(1.0 - node.color.a),
            ));
            out.push_str(&format!(
                "        <Box size=\"{} {} {}\"/>\n",
                round3(node.panel.width),
                round3(node.panel.height),
                round3(node.panel.depth),
            ));
            out.push_str("      </Shape>\n");
            out.push_str(&format!(
                "      <Shape><Text string=\"{}\"><FontStyle size=\"0.1\"/></Text></Shape>\n",
                xml_escape(&node.label),
            ));
            out.push_str("    </Transform>\n");
        }
        out.push_str("  </Scene>\n");
        out.push_str("</X3D>\n");
        out
    }
}

fn node_id(index: usize, statute: &Statute) -> String {
    format!("node-{index:04}-{}", sanitize_def(&statute.id))
}

/// Sanitises a string into a valid X3D `DEF` token (alphanumerics, `-`, `_`).
fn sanitize_def(raw: &str) -> String {
    let sanitized: String = raw
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch
            } else {
                '_'
            }
        })
        .collect();
    if sanitized.is_empty() {
        "node".to_string()
    } else {
        sanitized
    }
}

fn derivation_edges(statutes: &[Statute], nodes: &[Node3D]) -> Vec<SceneEdge> {
    let mut edges = Vec::new();
    for (statute, node) in statutes.iter().zip(nodes.iter()) {
        for source in &statute.derives_from {
            if let Some(target) = nodes
                .iter()
                .find(|candidate| &candidate.source_id == source)
            {
                edges.push(SceneEdge {
                    from: node.id.clone(),
                    to: target.id.clone(),
                    kind: "derives_from".to_string(),
                });
            }
        }
    }
    edges
}

/// Escapes XML reserved characters in text and attribute values.
fn xml_escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for ch in text.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            '\n' => out.push(' '),
            other => out.push(other),
        }
    }
    out
}

/// Importer for the 3D document format.
#[derive(Debug, Default)]
pub struct Document3DImporter;

impl Document3DImporter {
    /// Creates a new importer.
    pub fn new() -> Self {
        Self
    }
}

impl FormatImporter for Document3DImporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::SpatialDocument3D
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let scene = Scene3D::from_json(source)?;
        let statutes = scene.to_statutes();
        let mut report =
            ConversionReport::new(LegalFormat::SpatialDocument3D, LegalFormat::Legalis);
        report.statutes_converted = statutes.len();
        Ok((statutes, report))
    }

    fn validate(&self, source: &str) -> bool {
        schema_matches(source, SCHEMA)
    }
}

/// Exporter for the 3D document format.
#[derive(Debug, Clone, Copy)]
pub struct Document3DExporter {
    config: Document3DConfig,
}

impl Document3DExporter {
    /// Creates an exporter with default configuration.
    pub fn new() -> Self {
        Self {
            config: Document3DConfig::default(),
        }
    }

    /// Sets the scene generation configuration.
    pub fn with_config(mut self, config: Document3DConfig) -> Self {
        self.config = config;
        self
    }
}

impl Default for Document3DExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for Document3DExporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::SpatialDocument3D
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let scene = Scene3D::build(statutes, self.config);
        let json = scene.to_json()?;
        let mut report =
            ConversionReport::new(LegalFormat::Legalis, LegalFormat::SpatialDocument3D);
        report.statutes_converted = statutes.len();
        Ok((json, report))
    }

    fn can_represent(&self, _statute: &Statute) -> Vec<String> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType};

    fn statutes() -> Vec<Statute> {
        vec![
            Statute::new(
                "base-act",
                "Base Act",
                Effect::new(EffectType::Grant, "Establish base rights"),
            ),
            Statute::new(
                "amend/one",
                "Amendment One",
                Effect::new(EffectType::Obligation, "Add reporting duty"),
            )
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            })
            .with_derives_from("base-act"),
        ]
    }

    #[test]
    fn test_build_scene_nodes_edges_bounds() {
        let scene = Scene3D::build(&statutes(), Document3DConfig::default());
        assert_eq!(scene.node_count(), 2);
        assert_eq!(scene.edges.len(), 1);
        assert_eq!(scene.edges[0].kind, "derives_from");
        assert_eq!(scene.edges[0].to, scene.nodes[0].id);
        // Bounds enclose all node positions.
        for node in &scene.nodes {
            assert!(scene.bounds.contains(node.transform.position));
        }
        // The conditioned node is taller (higher Y scale).
        assert!(scene.nodes[1].transform.scale.y >= scene.nodes[0].transform.scale.y);
    }

    #[test]
    fn test_node_ids_sanitized() {
        let scene = Scene3D::build(&statutes(), Document3DConfig::default());
        // "amend/one" -> the '/' is replaced for a valid X3D DEF token.
        assert!(scene.nodes[1].id.contains("amend_one"));
        assert!(!scene.nodes[1].id.contains('/'));
    }

    #[test]
    fn test_export_import_roundtrip() {
        let exporter = Document3DExporter::new();
        let importer = Document3DImporter::new();
        let (json, export_report) = exporter.export(&statutes()).expect("export");
        assert_eq!(export_report.statutes_converted, 2);

        let (imported, import_report) = importer.import(&json).expect("import");
        assert_eq!(import_report.statutes_converted, 2);
        assert_eq!(imported.len(), 2);
        assert_eq!(imported[1].derives_from, vec!["base-act".to_string()]);
        assert_eq!(imported[1].preconditions.len(), 1);
    }

    #[test]
    fn test_x3d_rendering_is_well_formed_and_escaped() {
        let mut tricky = statutes();
        tricky.push(Statute::new(
            "x&y",
            "Rights <of> \"all\"",
            Effect::new(EffectType::Custom, "Special & sundry"),
        ));
        let scene = Scene3D::build(&tricky, Document3DConfig::default());
        let x3d = scene.to_x3d();
        assert!(x3d.starts_with("<?xml"));
        assert!(x3d.contains("<X3D"));
        assert!(x3d.contains("<Box size="));
        assert!(x3d.contains("<Transform"));
        // Reserved characters in the label are escaped.
        assert!(x3d.contains("Rights &lt;of&gt; &quot;all&quot;"));
        assert!(!x3d.contains("Rights <of>"));
    }

    #[test]
    fn test_validate() {
        let importer = Document3DImporter::new();
        let (json, _) = Document3DExporter::new()
            .export(&statutes())
            .expect("export");
        assert!(importer.validate(&json));
        assert!(!importer.validate("{\"schema\":\"legalis.vr-ar-annotation/v1\"}"));
        assert!(!importer.validate("<X3D></X3D>"));
    }
}
