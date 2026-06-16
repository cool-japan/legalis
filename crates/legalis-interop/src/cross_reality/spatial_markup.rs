//! Spatial legal markup (`SLM`).
//!
//! A compact, human-readable, fully-parseable textual markup that places each
//! statute as a spatially-transformed node in a scene. Unlike the JSON-based
//! immersive formats, `SLM` is a line-oriented text DSL: a header, a scene
//! directive, and one `@node` block per statute carrying its explicit
//! transform, anchor kind, and the complete statute payload (effect, conditions,
//! parameters, applicability, and derivations). Because every field is encoded,
//! the markup round-trips the underlying [`Statute`] set losslessly without a
//! separate provenance blob.

use super::{AnchorKind, Quaternion, SceneLayout, Transform, Vec3, layout_transform, round3};
use crate::formats_nextgen::StructuredStatute;
use crate::{
    ConversionReport, FormatExporter, FormatImporter, InteropError, InteropResult, LegalFormat,
};
use legalis_core::Statute;
use std::collections::BTreeMap;

/// Magic header prefix for spatial legal markup.
pub const HEADER: &str = "#SLM/v1";

/// Configuration for spatial markup generation.
#[derive(Debug, Clone, Copy)]
pub struct SpatialMarkupConfig {
    /// Spatial arrangement of nodes.
    pub layout: SceneLayout,
    /// Nominal spacing (metres) between nodes.
    pub spacing: f64,
    /// Anchor kind assigned to every node.
    pub anchor_kind: AnchorKind,
}

impl Default for SpatialMarkupConfig {
    fn default() -> Self {
        Self {
            layout: SceneLayout::Grid,
            spacing: 2.5,
            anchor_kind: AnchorKind::World,
        }
    }
}

/// A node in a spatial markup document.
#[derive(Debug, Clone)]
pub struct MarkupNode {
    /// Anchor kind for the node.
    pub anchor_kind: AnchorKind,
    /// Explicit placement transform.
    pub transform: Transform,
    /// Full structured statute payload.
    pub statute: StructuredStatute,
}

/// A parsed / buildable spatial markup document.
#[derive(Debug, Clone)]
pub struct SpatialMarkupDocument {
    /// Declared scene layout (advisory; transforms are explicit per node).
    pub layout: SceneLayout,
    /// Declared scene spacing (advisory).
    pub spacing: f64,
    /// Document nodes in order.
    pub nodes: Vec<MarkupNode>,
}

impl SpatialMarkupDocument {
    /// Builds a markup document from statutes using the given configuration.
    pub fn build(statutes: &[Statute], config: SpatialMarkupConfig) -> Self {
        let count = statutes.len();
        let nodes = statutes
            .iter()
            .enumerate()
            .map(|(index, statute)| MarkupNode {
                anchor_kind: config.anchor_kind,
                transform: layout_transform(index, count, config.layout, config.spacing),
                statute: StructuredStatute::from_statute(statute),
            })
            .collect();
        Self {
            layout: config.layout,
            spacing: config.spacing,
            nodes,
        }
    }

    /// Number of nodes.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Reconstructs the underlying statutes.
    pub fn to_statutes(&self) -> Vec<Statute> {
        self.nodes
            .iter()
            .map(|node| node.statute.to_statute())
            .collect()
    }

    /// Renders the document to `SLM` text.
    pub fn to_markup(&self) -> String {
        let mut out = String::new();
        out.push_str(HEADER);
        out.push('\n');
        out.push_str(&format!(
            "!scene layout={} spacing={} units=meters\n",
            self.layout.as_str(),
            round3(self.spacing)
        ));
        for node in &self.nodes {
            render_node(&mut out, node);
        }
        out
    }

    /// Parses an `SLM` document from text.
    pub fn from_markup(source: &str) -> InteropResult<Self> {
        let mut lines = source.lines();
        let header = lines.next().map(str::trim).unwrap_or("");
        if !header.starts_with("#SLM") {
            return Err(InteropError::ParseError(
                "spatial markup must start with an #SLM header".to_string(),
            ));
        }

        let mut layout = SceneLayout::Grid;
        let mut spacing = SpatialMarkupConfig::default().spacing;
        let mut nodes: Vec<MarkupNode> = Vec::new();
        let mut current: Option<NodeBuilder> = None;

        for raw in lines {
            let line = raw.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if let Some(rest) = line.strip_prefix("!scene") {
                let attributes = parse_attributes(rest);
                if let Some(value) = attributes.get("layout") {
                    layout = SceneLayout::parse(value);
                }
                if let Some(value) = attributes.get("spacing")
                    && let Ok(parsed) = value.parse::<f64>()
                {
                    spacing = parsed;
                }
                continue;
            }
            if let Some(rest) = line.strip_prefix("@node") {
                if let Some(builder) = current.take() {
                    nodes.push(builder.finish());
                }
                current = Some(NodeBuilder::start(rest));
                continue;
            }
            if let Some(builder) = current.as_mut() {
                builder.apply_field(line);
            }
        }
        if let Some(builder) = current.take() {
            nodes.push(builder.finish());
        }

        Ok(Self {
            layout,
            spacing,
            nodes,
        })
    }
}

fn render_node(out: &mut String, node: &MarkupNode) {
    let statute = &node.statute;
    out.push_str(&format!(
        "@node {} anchor={}\n",
        statute.id,
        node.anchor_kind.as_str()
    ));
    let position = node.transform.position;
    let rotation = node.transform.rotation;
    let scale = node.transform.scale;
    out.push_str(&format!(
        "  pos=({},{},{})\n",
        round3(position.x),
        round3(position.y),
        round3(position.z)
    ));
    out.push_str(&format!(
        "  rot=({},{},{},{})\n",
        round3(rotation.x),
        round3(rotation.y),
        round3(rotation.z),
        round3(rotation.w)
    ));
    out.push_str(&format!(
        "  scale=({},{},{})\n",
        round3(scale.x),
        round3(scale.y),
        round3(scale.z)
    ));
    out.push_str(&format!("  title: {}\n", statute.title));
    if let Some(jurisdiction) = &statute.jurisdiction {
        out.push_str(&format!("  jurisdiction: {}\n", jurisdiction));
    }
    out.push_str(&format!("  version: {}\n", statute.version));
    out.push_str(&format!(
        "  effect: {} -- {}\n",
        statute.effect_type, statute.effect_description
    ));
    for (key, value) in &statute.parameters {
        out.push_str(&format!("  param: {} = {}\n", key, value));
    }
    for condition in &statute.conditions {
        out.push_str(&format!("  cond: {}\n", condition));
    }
    for entity in &statute.applies_to {
        out.push_str(&format!("  applies: {}\n", entity));
    }
    for source in &statute.derives_from {
        out.push_str(&format!("  derives: {}\n", source));
    }
}

/// Mutable accumulator while parsing one `@node` block.
struct NodeBuilder {
    anchor_kind: AnchorKind,
    position: Vec3,
    rotation: Quaternion,
    scale: Vec3,
    id: String,
    title: String,
    jurisdiction: Option<String>,
    version: u32,
    effect_type: String,
    effect_description: String,
    parameters: BTreeMap<String, String>,
    conditions: Vec<String>,
    applies_to: Vec<String>,
    derives_from: Vec<String>,
}

impl NodeBuilder {
    fn start(header_rest: &str) -> Self {
        let attributes = parse_attributes(header_rest);
        // The node id is the first bare (non key=value) token after `@node`.
        let id = header_rest
            .split_whitespace()
            .find(|token| !token.contains('='))
            .unwrap_or("node")
            .to_string();
        let anchor_kind = attributes
            .get("anchor")
            .map(|value| AnchorKind::parse(value))
            .unwrap_or(AnchorKind::World);
        Self {
            anchor_kind,
            position: Vec3::zero(),
            rotation: Quaternion::identity(),
            scale: Vec3::splat(1.0),
            id,
            title: String::new(),
            jurisdiction: None,
            version: 1,
            effect_type: "custom".to_string(),
            effect_description: String::new(),
            parameters: BTreeMap::new(),
            conditions: Vec::new(),
            applies_to: Vec::new(),
            derives_from: Vec::new(),
        }
    }

    fn apply_field(&mut self, line: &str) {
        if let Some(rest) = line.strip_prefix("pos=") {
            if let Some(vector) = parse_vec3(rest) {
                self.position = vector;
            }
        } else if let Some(rest) = line.strip_prefix("rot=") {
            if let Some(quat) = parse_quat(rest) {
                self.rotation = quat;
            }
        } else if let Some(rest) = line.strip_prefix("scale=") {
            if let Some(vector) = parse_vec3(rest) {
                self.scale = vector;
            }
        } else if let Some(rest) = line.strip_prefix("param:") {
            if let Some((key, value)) = rest.split_once('=') {
                self.parameters
                    .insert(key.trim().to_string(), value.trim().to_string());
            }
        } else if let Some(rest) = line.strip_prefix("title:") {
            self.title = rest.trim().to_string();
        } else if let Some(rest) = line.strip_prefix("jurisdiction:") {
            self.jurisdiction = Some(rest.trim().to_string());
        } else if let Some(rest) = line.strip_prefix("version:") {
            self.version = rest.trim().parse::<u32>().unwrap_or(1);
        } else if let Some(rest) = line.strip_prefix("effect:") {
            let (effect_type, description) = rest.split_once(" -- ").unwrap_or((rest.trim(), ""));
            self.effect_type = effect_type.trim().to_string();
            self.effect_description = description.trim().to_string();
        } else if let Some(rest) = line.strip_prefix("cond:") {
            self.conditions.push(rest.trim().to_string());
        } else if let Some(rest) = line.strip_prefix("applies:") {
            self.applies_to.push(rest.trim().to_string());
        } else if let Some(rest) = line.strip_prefix("derives:") {
            self.derives_from.push(rest.trim().to_string());
        }
    }

    fn finish(self) -> MarkupNode {
        MarkupNode {
            anchor_kind: self.anchor_kind,
            transform: Transform::new(self.position, self.rotation, self.scale),
            statute: StructuredStatute {
                id: self.id,
                title: self.title,
                jurisdiction: self.jurisdiction,
                version: self.version,
                effect_type: self.effect_type,
                effect_description: self.effect_description,
                parameters: self.parameters,
                conditions: self.conditions,
                applies_to: self.applies_to,
                derives_from: self.derives_from,
            },
        }
    }
}

/// Parses whitespace-separated `key=value` attributes from a directive tail.
fn parse_attributes(rest: &str) -> BTreeMap<String, String> {
    rest.split_whitespace()
        .filter_map(|token| token.split_once('='))
        .map(|(key, value)| (key.trim().to_string(), value.trim().to_string()))
        .collect()
}

/// Parses comma-separated floats from a `(a,b,c,...)` group.
fn parse_floats(group: &str) -> Vec<f64> {
    group
        .trim()
        .trim_start_matches('(')
        .trim_end_matches(')')
        .split(',')
        .filter_map(|part| part.trim().parse::<f64>().ok())
        .collect()
}

fn parse_vec3(group: &str) -> Option<Vec3> {
    let parts = parse_floats(group);
    if parts.len() >= 3 {
        Some(Vec3::new(parts[0], parts[1], parts[2]))
    } else {
        None
    }
}

fn parse_quat(group: &str) -> Option<Quaternion> {
    let parts = parse_floats(group);
    if parts.len() >= 4 {
        Some(Quaternion::new(parts[0], parts[1], parts[2], parts[3]))
    } else {
        None
    }
}

/// Importer for the spatial markup format.
#[derive(Debug, Default)]
pub struct SpatialMarkupImporter;

impl SpatialMarkupImporter {
    /// Creates a new importer.
    pub fn new() -> Self {
        Self
    }
}

impl FormatImporter for SpatialMarkupImporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::SpatialMarkup
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let document = SpatialMarkupDocument::from_markup(source)?;
        let statutes = document.to_statutes();
        let mut report = ConversionReport::new(LegalFormat::SpatialMarkup, LegalFormat::Legalis);
        report.statutes_converted = statutes.len();
        Ok((statutes, report))
    }

    fn validate(&self, source: &str) -> bool {
        source.trim_start().starts_with("#SLM")
    }
}

/// Exporter for the spatial markup format.
#[derive(Debug, Clone, Copy)]
pub struct SpatialMarkupExporter {
    config: SpatialMarkupConfig,
}

impl SpatialMarkupExporter {
    /// Creates an exporter with default configuration.
    pub fn new() -> Self {
        Self {
            config: SpatialMarkupConfig::default(),
        }
    }

    /// Sets the generation configuration.
    pub fn with_config(mut self, config: SpatialMarkupConfig) -> Self {
        self.config = config;
        self
    }
}

impl Default for SpatialMarkupExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for SpatialMarkupExporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::SpatialMarkup
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let document = SpatialMarkupDocument::build(statutes, self.config);
        let markup = document.to_markup();
        let mut report = ConversionReport::new(LegalFormat::Legalis, LegalFormat::SpatialMarkup);
        report.statutes_converted = statutes.len();
        Ok((markup, report))
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
        let mut effect = Effect::new(EffectType::Grant, "Grant the right to vote");
        effect
            .parameters
            .insert("authority".to_string(), "federal".to_string());
        vec![
            Statute::new("voting-rights", "Voting Rights", effect)
                .with_jurisdiction("US")
                .with_version(2)
                .with_precondition(Condition::Age {
                    operator: ComparisonOp::GreaterOrEqual,
                    value: 18,
                })
                .with_precondition(Condition::AttributeEquals {
                    key: "status".to_string(),
                    value: "active".to_string(),
                })
                .with_applies_to("Citizen")
                .with_derives_from("constitution"),
            Statute::new(
                "duty",
                "Reporting Duty",
                Effect::new(EffectType::Obligation, "File annual report"),
            ),
        ]
    }

    #[test]
    fn test_markup_has_header_and_blocks() {
        let document = SpatialMarkupDocument::build(&statutes(), SpatialMarkupConfig::default());
        let markup = document.to_markup();
        assert!(markup.starts_with(HEADER));
        assert!(markup.contains("!scene layout=grid"));
        assert!(markup.contains("@node voting-rights anchor=world"));
        assert!(markup.contains("effect: grant -- Grant the right to vote"));
        assert!(markup.contains("param: authority = federal"));
        assert!(markup.contains("cond: age >= 18"));
        assert!(markup.contains("applies: Citizen"));
        assert!(markup.contains("derives: constitution"));
    }

    #[test]
    fn test_markup_parse_roundtrip_document() {
        let document = SpatialMarkupDocument::build(&statutes(), SpatialMarkupConfig::default());
        let markup = document.to_markup();
        let parsed = SpatialMarkupDocument::from_markup(&markup).expect("parse");
        assert_eq!(parsed.node_count(), 2);
        assert_eq!(parsed.layout, SceneLayout::Grid);
        assert_eq!(parsed.nodes[0].statute.id, "voting-rights");
        assert_eq!(parsed.nodes[0].statute.version, 2);
        assert_eq!(parsed.nodes[0].anchor_kind, AnchorKind::World);
        assert_eq!(parsed.nodes[0].transform, document.nodes[0].transform);
    }

    #[test]
    fn test_full_statute_roundtrip() {
        let exporter = SpatialMarkupExporter::new();
        let importer = SpatialMarkupImporter::new();
        let (markup, export_report) = exporter.export(&statutes()).expect("export");
        assert_eq!(export_report.statutes_converted, 2);

        let (imported, import_report) = importer.import(&markup).expect("import");
        assert_eq!(import_report.statutes_converted, 2);
        assert_eq!(imported.len(), 2);

        let first = &imported[0];
        assert_eq!(first.id, "voting-rights");
        assert_eq!(first.title, "Voting Rights");
        assert_eq!(first.jurisdiction.as_deref(), Some("US"));
        assert_eq!(first.version, 2);
        assert_eq!(first.effect.effect_type, EffectType::Grant);
        assert_eq!(
            first.effect.parameters.get("authority"),
            Some(&"federal".to_string())
        );
        assert_eq!(first.preconditions.len(), 2);
        assert_eq!(first.applies_to, vec!["Citizen".to_string()]);
        assert_eq!(first.derives_from, vec!["constitution".to_string()]);
    }

    #[test]
    fn test_custom_layout_and_anchor() {
        let config = SpatialMarkupConfig {
            layout: SceneLayout::Helix,
            spacing: 3.0,
            anchor_kind: AnchorKind::PlaneVertical,
        };
        let (markup, _) = SpatialMarkupExporter::new()
            .with_config(config)
            .export(&statutes())
            .expect("export");
        assert!(markup.contains("layout=helix"));
        assert!(markup.contains("anchor=plane_vertical"));
        let parsed = SpatialMarkupDocument::from_markup(&markup).expect("parse");
        assert_eq!(parsed.layout, SceneLayout::Helix);
        assert_eq!(parsed.nodes[0].anchor_kind, AnchorKind::PlaneVertical);
    }

    #[test]
    fn test_validate_and_header_error() {
        let importer = SpatialMarkupImporter::new();
        let (markup, _) = SpatialMarkupExporter::new()
            .export(&statutes())
            .expect("export");
        assert!(importer.validate(&markup));
        assert!(!importer.validate("{\"schema\":\"x\"}"));
        let error = SpatialMarkupDocument::from_markup("no header here");
        assert!(error.is_err());
    }

    #[test]
    fn test_float_group_parsers() {
        assert_eq!(
            parse_vec3("(1.0,2.5,-3.0)"),
            Some(Vec3::new(1.0, 2.5, -3.0))
        );
        assert_eq!(parse_vec3("(1.0,2.0)"), None);
        assert_eq!(
            parse_quat("(0.0,0.0,0.0,1.0)"),
            Some(Quaternion::new(0.0, 0.0, 0.0, 1.0))
        );
        assert_eq!(parse_quat("(0.0,0.0,1.0)"), None);
    }
}
