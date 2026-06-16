//! VR/AR legal annotation format.
//!
//! Projects a legal corpus into a set of spatial annotations, each anchored to
//! a real- or virtual-world feature (a world point, an image marker, a detected
//! plane, a geo-location, ...) for overlay in augmented- and virtual-reality
//! viewers. Annotations are coloured by effect type, optionally billboarded to
//! face the viewer, and carry a Markdown body for in-headset reading.
//!
//! The scene also stores a structured provenance list so the original statutes
//! can be reconstructed losslessly on import.

use super::{
    AnchorKind, Color, SceneLayout, SpatialAnchor, Vec3, condition_salience, effect_color,
    layout_transform, round3,
};
use crate::formats_nextgen::{
    StructuredStatute, build_structured, effect_type_to_str, render_statute_markdown,
};
use crate::{
    ConversionReport, FormatExporter, FormatImporter, InteropError, InteropResult, LegalFormat,
};
use legalis_core::Statute;
use serde::{Deserialize, Serialize};

/// Schema identifier for the VR/AR annotation format.
pub const SCHEMA: &str = "legalis.vr-ar-annotation/v1";

/// Configuration for VR/AR scene generation.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct VrArConfig {
    /// Spatial arrangement of the annotation anchors.
    pub layout: SceneLayout,
    /// Nominal spacing (metres) between anchors.
    pub spacing: f64,
    /// Default anchor kind assigned to every annotation.
    pub anchor_kind: AnchorKind,
    /// Whether annotations rotate to always face the viewer.
    pub billboard: bool,
    /// Visibility cut-off distance (metres) for each annotation.
    pub visibility_range_m: f64,
}

impl Default for VrArConfig {
    fn default() -> Self {
        Self {
            layout: SceneLayout::Circle,
            spacing: 2.0,
            anchor_kind: AnchorKind::World,
            billboard: true,
            visibility_range_m: 25.0,
        }
    }
}

/// A VR/AR annotation scene.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VrArScene {
    /// Schema identifier ([`SCHEMA`]).
    pub schema: String,
    /// Coordinate space convention.
    pub coordinate_space: String,
    /// Whether annotations billboard toward the viewer.
    pub billboard: bool,
    /// The anchored annotations in document order.
    pub annotations: Vec<AnnotationAnchor>,
    /// Structured provenance enabling lossless reconstruction.
    pub provenance: Vec<StructuredStatute>,
}

/// A single spatially-anchored legal annotation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnotationAnchor {
    /// Stable annotation identifier.
    pub id: String,
    /// Source statute identifier.
    pub source_id: String,
    /// Short label (the statute title).
    pub label: String,
    /// Markdown annotation body.
    pub body: String,
    /// Spatial anchor (kind plus placement transform).
    pub anchor: SpatialAnchor,
    /// Display colour (derived from effect type).
    pub color: Color,
    /// Uniform display scale (derived from condition salience).
    pub scale: f64,
    /// Visibility cut-off distance in metres.
    pub visibility_range_m: f64,
}

impl VrArScene {
    /// Builds an annotation scene from statutes using the given configuration.
    pub fn build(statutes: &[Statute], config: VrArConfig) -> Self {
        let count = statutes.len();
        let annotations = statutes
            .iter()
            .enumerate()
            .map(|(index, statute)| {
                let transform = layout_transform(index, count, config.layout, config.spacing);
                let mut anchor = SpatialAnchor::new(config.anchor_kind, transform);
                if config.anchor_kind != AnchorKind::World {
                    anchor.reference = Some(format!("ref-{}", statute.id));
                }
                let effect = effect_type_to_str(&statute.effect.effect_type);
                AnnotationAnchor {
                    id: format!("anno-{index:04}-{}", statute.id),
                    source_id: statute.id.clone(),
                    label: statute.title.clone(),
                    body: render_statute_markdown(statute),
                    anchor,
                    color: effect_color(effect),
                    scale: round3(condition_salience(statute.preconditions.len())),
                    visibility_range_m: config.visibility_range_m,
                }
            })
            .collect();

        Self {
            schema: SCHEMA.to_string(),
            coordinate_space: "right_handed_y_up_meters".to_string(),
            billboard: config.billboard,
            annotations,
            provenance: build_structured(statutes),
        }
    }

    /// Number of annotations in the scene.
    pub fn annotation_count(&self) -> usize {
        self.annotations.len()
    }

    /// The centroid of all annotation anchor positions (origin when empty).
    pub fn centroid(&self) -> Vec3 {
        if self.annotations.is_empty() {
            return Vec3::zero();
        }
        let sum = self
            .annotations
            .iter()
            .fold(Vec3::zero(), |acc, annotation| {
                acc.plus(annotation.anchor.transform.position)
            });
        sum.scaled(1.0 / self.annotations.len() as f64)
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
            InteropError::SerializationError(format!("Failed to serialize VR/AR scene: {error}"))
        })
    }

    /// Parses a scene from JSON.
    pub fn from_json(source: &str) -> InteropResult<Self> {
        serde_json::from_str(source).map_err(|error| {
            InteropError::ParseError(format!("Failed to parse VR/AR JSON: {error}"))
        })
    }
}

/// Importer for the VR/AR annotation format.
#[derive(Debug, Default)]
pub struct VrArAnnotationImporter;

impl VrArAnnotationImporter {
    /// Creates a new importer.
    pub fn new() -> Self {
        Self
    }
}

impl FormatImporter for VrArAnnotationImporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::VrArAnnotation
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let scene = VrArScene::from_json(source)?;
        let statutes = scene.to_statutes();
        let mut report = ConversionReport::new(LegalFormat::VrArAnnotation, LegalFormat::Legalis);
        report.statutes_converted = statutes.len();
        Ok((statutes, report))
    }

    fn validate(&self, source: &str) -> bool {
        schema_matches(source, SCHEMA)
    }
}

/// Exporter for the VR/AR annotation format.
#[derive(Debug, Clone, Copy)]
pub struct VrArAnnotationExporter {
    config: VrArConfig,
}

impl VrArAnnotationExporter {
    /// Creates an exporter with default configuration.
    pub fn new() -> Self {
        Self {
            config: VrArConfig::default(),
        }
    }

    /// Sets the scene generation configuration.
    pub fn with_config(mut self, config: VrArConfig) -> Self {
        self.config = config;
        self
    }
}

impl Default for VrArAnnotationExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for VrArAnnotationExporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::VrArAnnotation
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let scene = VrArScene::build(statutes, self.config);
        let json = scene.to_json()?;
        let mut report = ConversionReport::new(LegalFormat::Legalis, LegalFormat::VrArAnnotation);
        report.statutes_converted = statutes.len();
        Ok((json, report))
    }

    fn can_represent(&self, _statute: &Statute) -> Vec<String> {
        vec![]
    }
}

/// Returns true when `source` is JSON whose top-level `schema` equals `schema`.
pub(crate) fn schema_matches(source: &str, schema: &str) -> bool {
    serde_json::from_str::<serde_json::Value>(source)
        .ok()
        .and_then(|value| {
            value
                .get("schema")
                .and_then(|field| field.as_str())
                .map(|found| found == schema)
        })
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType};

    fn statutes() -> Vec<Statute> {
        vec![
            Statute::new(
                "voting-rights",
                "Voting Rights",
                Effect::new(EffectType::Grant, "Grant the right to vote"),
            )
            .with_jurisdiction("US")
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            }),
            Statute::new(
                "no-smoking",
                "No Smoking Indoors",
                Effect::new(EffectType::Prohibition, "Prohibit indoor smoking"),
            ),
            Statute::new(
                "tax-duty",
                "Tax Duty",
                Effect::new(EffectType::Obligation, "Pay annual tax"),
            )
            .with_precondition(Condition::Income {
                operator: ComparisonOp::GreaterThan,
                value: 10_000,
            })
            .with_precondition(Condition::HasAttribute {
                key: "resident".to_string(),
            }),
        ]
    }

    #[test]
    fn test_build_scene_anchors_and_colors() {
        let scene = VrArScene::build(&statutes(), VrArConfig::default());
        assert_eq!(scene.annotation_count(), 3);
        assert_eq!(scene.schema, SCHEMA);
        // Effect colours differ across grant/prohibition/obligation.
        assert_eq!(
            scene.annotations[0].color.to_hex(),
            effect_color("grant").to_hex()
        );
        assert_eq!(
            scene.annotations[1].color.to_hex(),
            effect_color("prohibition").to_hex()
        );
        // More-conditioned statutes get a larger display scale.
        assert!(scene.annotations[2].scale > scene.annotations[1].scale);
        assert!(scene.annotations[0].body.contains("Voting Rights"));
    }

    #[test]
    fn test_non_world_anchor_gets_reference() {
        let config = VrArConfig {
            anchor_kind: AnchorKind::ImageMarker,
            ..VrArConfig::default()
        };
        let scene = VrArScene::build(&statutes(), config);
        for annotation in &scene.annotations {
            assert_eq!(annotation.anchor.kind, AnchorKind::ImageMarker);
            assert!(annotation.anchor.reference.is_some());
        }
    }

    #[test]
    fn test_export_import_roundtrip() {
        let exporter = VrArAnnotationExporter::new();
        let importer = VrArAnnotationImporter::new();
        let (json, export_report) = exporter.export(&statutes()).expect("export");
        assert_eq!(export_report.statutes_converted, 3);

        let (imported, import_report) = importer.import(&json).expect("import");
        assert_eq!(import_report.statutes_converted, 3);
        assert_eq!(imported.len(), 3);
        assert_eq!(imported[0].id, "voting-rights");
        assert_eq!(imported[0].jurisdiction.as_deref(), Some("US"));
        assert_eq!(imported[2].preconditions.len(), 2);
    }

    #[test]
    fn test_validate_and_centroid() {
        let importer = VrArAnnotationImporter::new();
        let (json, _) = VrArAnnotationExporter::new()
            .export(&statutes())
            .expect("export");
        assert!(importer.validate(&json));
        assert!(!importer.validate("{\"schema\":\"legalis.semantic-chunk/v1\"}"));
        assert!(!importer.validate("not json"));

        let scene = VrArScene::from_json(&json).expect("parse");
        // Circle layout is centred on the origin, so the centroid is near it.
        let centroid = scene.centroid();
        assert!(centroid.length() < 1.0);
        assert_eq!(
            VrArScene::build(&[], VrArConfig::default()).centroid(),
            Vec3::zero()
        );
    }
}
