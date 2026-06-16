//! Holographic legal display format.
//!
//! Projects a legal corpus into a volumetric, depth-layered light-field display.
//! Statutes are distributed across discrete depth planes (nearest planes carry
//! the most salient provisions); each holographic element records its in-plane
//! position, a parallax factor derived from its depth, a luminance, and a
//! colour. The model mirrors how multi-plane and light-field displays composite
//! imagery at several focal depths.
//!
//! Provenance is embedded so the original statutes round-trip losslessly.

use super::{Color, condition_salience, depth_parallax, effect_color, round3};
use crate::cross_reality::vr_ar::schema_matches;
use crate::formats_nextgen::{
    StructuredStatute, build_structured, effect_type_to_str, render_statute_markdown,
};
use crate::{
    ConversionReport, FormatExporter, FormatImporter, InteropError, InteropResult, LegalFormat,
};
use legalis_core::Statute;
use serde::{Deserialize, Serialize};

/// Schema identifier for the holographic display format.
pub const SCHEMA: &str = "legalis.holographic-display/v1";

/// Configuration for holographic display generation.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct HolographicConfig {
    /// Number of discrete depth planes.
    pub layer_count: usize,
    /// Spacing (metres) between adjacent depth planes.
    pub layer_spacing: f64,
    /// Horizontal extent (metres) of each plane.
    pub plane_width: f64,
    /// Vertical extent (metres) of each plane.
    pub plane_height: f64,
}

impl Default for HolographicConfig {
    fn default() -> Self {
        Self {
            layer_count: 4,
            layer_spacing: 0.25,
            plane_width: 1.2,
            plane_height: 0.8,
        }
    }
}

impl HolographicConfig {
    fn sanitized(self) -> Self {
        Self {
            layer_count: self.layer_count.max(1),
            layer_spacing: if self.layer_spacing > f64::EPSILON {
                self.layer_spacing
            } else {
                0.25
            },
            plane_width: if self.plane_width > f64::EPSILON {
                self.plane_width
            } else {
                1.0
            },
            plane_height: if self.plane_height > f64::EPSILON {
                self.plane_height
            } else {
                1.0
            },
        }
    }
}

/// Light-field projection parameters.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LightFieldParams {
    /// Number of distinct angular views the display reconstructs.
    pub view_count: u32,
    /// Total horizontal angular range (degrees) of the viewing cone.
    pub angular_range_deg: f64,
    /// Reference illumination wavelength (nanometres).
    pub wavelength_nm: f64,
}

impl Default for LightFieldParams {
    fn default() -> Self {
        Self {
            view_count: 45,
            angular_range_deg: 40.0,
            wavelength_nm: 532.0,
        }
    }
}

/// A single depth plane of the holographic display.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepthLayer {
    /// Zero-based layer index (0 is nearest the viewer).
    pub index: usize,
    /// Depth of the plane (metres) from the display surface.
    pub depth_m: f64,
    /// Base opacity of the plane in `[0, 1]`.
    pub opacity: f64,
    /// Elements composited on this plane.
    pub elements: Vec<HologramElement>,
}

/// A holographic element projected on a depth plane.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HologramElement {
    /// Stable element identifier.
    pub id: String,
    /// Source statute identifier.
    pub source_id: String,
    /// Display label (statute title).
    pub label: String,
    /// In-plane position `(u, v)` in `[-1, 1]` normalised plane coordinates.
    pub plane_position: [f64; 2],
    /// Parallax factor in `[0, 1]` (nearer elements parallax more).
    pub parallax: f64,
    /// Relative luminance / emission intensity in `[0, 1]`.
    pub luminance: f64,
    /// Element colour (derived from effect type).
    pub color: Color,
    /// Markdown content shown when the element is focused.
    pub content: String,
}

/// A holographic display representation of a legal document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HologramDisplay {
    /// Schema identifier ([`SCHEMA`]).
    pub schema: String,
    /// Projection technique label.
    pub projection: String,
    /// Light-field parameters.
    pub light_field: LightFieldParams,
    /// Depth planes ordered from nearest to farthest.
    pub depth_layers: Vec<DepthLayer>,
    /// Structured provenance enabling lossless reconstruction.
    pub provenance: Vec<StructuredStatute>,
}

impl HologramDisplay {
    /// Builds a holographic display from statutes using the given configuration.
    pub fn build(statutes: &[Statute], config: HolographicConfig) -> Self {
        let config = config.sanitized();
        let max_depth = (config.layer_count.saturating_sub(1)) as f64 * config.layer_spacing;

        // Order statutes by salience so the most-conditioned land on the nearest
        // plane; ties keep document order for determinism.
        let mut order: Vec<usize> = (0..statutes.len()).collect();
        order.sort_by(|&a, &b| {
            let sa = statutes[a].preconditions.len();
            let sb = statutes[b].preconditions.len();
            sb.cmp(&sa).then(a.cmp(&b))
        });

        let mut layers: Vec<DepthLayer> = (0..config.layer_count)
            .map(|index| {
                let depth_m = round3(index as f64 * config.layer_spacing);
                DepthLayer {
                    index,
                    depth_m,
                    opacity: round3(1.0 - 0.15 * index as f64).max(0.2),
                    elements: Vec::new(),
                }
            })
            .collect();

        for (rank, &statute_index) in order.iter().enumerate() {
            let statute = &statutes[statute_index];
            let layer_index = rank % config.layer_count;
            let position_in_layer = layers[layer_index].elements.len();
            let depth_m = layers[layer_index].depth_m;
            let effect = effect_type_to_str(&statute.effect.effect_type);
            let salience = condition_salience(statute.preconditions.len());
            let element = HologramElement {
                id: format!("holo-{statute_index:04}-{}", statute.id),
                source_id: statute.id.clone(),
                label: statute.title.clone(),
                plane_position: plane_position(position_in_layer),
                parallax: round3(depth_parallax(depth_m, max_depth)),
                luminance: round3((salience - 1.0).clamp(0.0, 1.0) * 0.5 + 0.5),
                color: effect_color(effect),
                content: render_statute_markdown(statute),
            };
            layers[layer_index].elements.push(element);
        }

        Self {
            schema: SCHEMA.to_string(),
            projection: "multiplane_light_field".to_string(),
            light_field: LightFieldParams::default(),
            depth_layers: layers,
            provenance: build_structured(statutes),
        }
    }

    /// Number of depth layers.
    pub fn layer_count(&self) -> usize {
        self.depth_layers.len()
    }

    /// Total number of holographic elements across all layers.
    pub fn element_count(&self) -> usize {
        self.depth_layers
            .iter()
            .map(|layer| layer.elements.len())
            .sum()
    }

    /// Reconstructs the underlying statutes from provenance.
    pub fn to_statutes(&self) -> Vec<Statute> {
        self.provenance
            .iter()
            .map(StructuredStatute::to_statute)
            .collect()
    }

    /// Serialises the display to pretty JSON.
    pub fn to_json(&self) -> InteropResult<String> {
        serde_json::to_string_pretty(self).map_err(|error| {
            InteropError::SerializationError(format!("Failed to serialize hologram: {error}"))
        })
    }

    /// Parses a display from JSON.
    pub fn from_json(source: &str) -> InteropResult<Self> {
        serde_json::from_str(source).map_err(|error| {
            InteropError::ParseError(format!("Failed to parse hologram JSON: {error}"))
        })
    }
}

/// Places the `n`-th element of a plane on a deterministic outward spiral of
/// normalised `(u, v)` positions within `[-1, 1]`.
fn plane_position(n: usize) -> [f64; 2] {
    if n == 0 {
        return [0.0, 0.0];
    }
    // Ring-based placement: ring r holds up to 8*r slots.
    let mut ring = 1usize;
    let mut start = 1usize;
    while start + 8 * ring <= n {
        start += 8 * ring;
        ring += 1;
    }
    let slot = n - start;
    let slots_in_ring = 8 * ring;
    let angle = super::FULL_TURN * (slot as f64) / (slots_in_ring as f64);
    let radius = (ring as f64) / 6.0;
    let radius = radius.min(1.0);
    [round3(radius * angle.cos()), round3(radius * angle.sin())]
}

/// Importer for the holographic display format.
#[derive(Debug, Default)]
pub struct HolographicImporter;

impl HolographicImporter {
    /// Creates a new importer.
    pub fn new() -> Self {
        Self
    }
}

impl FormatImporter for HolographicImporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::Holographic
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let display = HologramDisplay::from_json(source)?;
        let statutes = display.to_statutes();
        let mut report = ConversionReport::new(LegalFormat::Holographic, LegalFormat::Legalis);
        report.statutes_converted = statutes.len();
        Ok((statutes, report))
    }

    fn validate(&self, source: &str) -> bool {
        schema_matches(source, SCHEMA)
    }
}

/// Exporter for the holographic display format.
#[derive(Debug, Clone, Copy)]
pub struct HolographicExporter {
    config: HolographicConfig,
}

impl HolographicExporter {
    /// Creates an exporter with default configuration.
    pub fn new() -> Self {
        Self {
            config: HolographicConfig::default(),
        }
    }

    /// Sets the display generation configuration.
    pub fn with_config(mut self, config: HolographicConfig) -> Self {
        self.config = config;
        self
    }
}

impl Default for HolographicExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for HolographicExporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::Holographic
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let display = HologramDisplay::build(statutes, self.config);
        let json = display.to_json()?;
        let mut report = ConversionReport::new(LegalFormat::Legalis, LegalFormat::Holographic);
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
        (0..6u32)
            .map(|index| {
                let mut statute = Statute::new(
                    format!("provision-{index}"),
                    format!("Provision {index}"),
                    Effect::new(EffectType::Obligation, format!("Duty {index}")),
                );
                for step in 0..index {
                    statute = statute.with_precondition(Condition::Age {
                        operator: ComparisonOp::GreaterOrEqual,
                        value: 18 + step,
                    });
                }
                statute
            })
            .collect()
    }

    #[test]
    fn test_build_layers_and_distribution() {
        let display = HologramDisplay::build(&statutes(), HolographicConfig::default());
        assert_eq!(display.layer_count(), 4);
        assert_eq!(display.element_count(), 6);
        // Nearest layer (index 0, depth 0) has full parallax.
        let nearest = &display.depth_layers[0];
        assert!(nearest.depth_m.abs() < 1e-9);
        for element in &nearest.elements {
            assert!((element.parallax - 1.0).abs() < 1e-6);
        }
    }

    #[test]
    fn test_most_conditioned_on_nearest_plane() {
        let display = HologramDisplay::build(&statutes(), HolographicConfig::default());
        // provision-5 has the most preconditions, so it ranks first -> layer 0.
        let nearest_ids: Vec<&str> = display.depth_layers[0]
            .elements
            .iter()
            .map(|element| element.source_id.as_str())
            .collect();
        assert!(nearest_ids.contains(&"provision-5"));
    }

    #[test]
    fn test_plane_positions_within_bounds() {
        let display = HologramDisplay::build(&statutes(), HolographicConfig::default());
        for layer in &display.depth_layers {
            for element in &layer.elements {
                assert!(element.plane_position[0].abs() <= 1.0);
                assert!(element.plane_position[1].abs() <= 1.0);
            }
        }
    }

    #[test]
    fn test_export_import_roundtrip() {
        let exporter = HolographicExporter::new();
        let importer = HolographicImporter::new();
        let (json, export_report) = exporter.export(&statutes()).expect("export");
        assert_eq!(export_report.statutes_converted, 6);

        let (imported, import_report) = importer.import(&json).expect("import");
        assert_eq!(import_report.statutes_converted, 6);
        assert_eq!(imported.len(), 6);
        // Provenance preserves original document order regardless of layer order.
        assert_eq!(imported[0].id, "provision-0");
        assert_eq!(imported[5].preconditions.len(), 5);
    }

    #[test]
    fn test_validate_and_single_layer_config() {
        let importer = HolographicImporter::new();
        let config = HolographicConfig {
            layer_count: 0,
            layer_spacing: 0.0,
            plane_width: 0.0,
            plane_height: 0.0,
        };
        // Degenerate config is sanitised to a single usable plane.
        let display = HologramDisplay::build(&statutes(), config);
        assert_eq!(display.layer_count(), 1);
        assert_eq!(display.element_count(), 6);

        let json = display.to_json().expect("json");
        assert!(importer.validate(&json));
        assert!(!importer.validate("{\"schema\":\"legalis.metaverse-legal/v1\"}"));
    }
}
