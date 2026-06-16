//! Metaverse-native legal format.
//!
//! Projects a legal corpus into an interactive virtual-world scene graph. Each
//! statute becomes a placeable, avatar-interactable entity (a kiosk, monolith,
//! ...) with a transform, an effect-derived colour, and a set of interactions
//! (inspect, accept, acknowledge, transfer, ...). Derivation relationships
//! become portals that teleport an avatar between linked provisions. World
//! metadata records the coordinate system, spawn point, and scale unit.
//!
//! Provenance is embedded so the original statutes round-trip losslessly.

use super::{
    Color, SceneLayout, Transform, Vec3, condition_salience, effect_color, layout_transform, round3,
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

/// Schema identifier for the metaverse-native format.
pub const SCHEMA: &str = "legalis.metaverse-legal/v1";

/// Configuration for metaverse scene generation.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MetaverseConfig {
    /// Spatial arrangement of entities.
    pub layout: SceneLayout,
    /// Nominal spacing (metres) between entities.
    pub spacing: f64,
    /// Primitive model assigned to each entity.
    pub entity_primitive: EntityPrimitive,
}

impl Default for MetaverseConfig {
    fn default() -> Self {
        Self {
            layout: SceneLayout::Circle,
            spacing: 4.0,
            entity_primitive: EntityPrimitive::Monolith,
        }
    }
}

/// A coarse visual primitive for a legal entity in-world.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityPrimitive {
    /// A tall standing slab.
    Monolith,
    /// An interactive information kiosk.
    Kiosk,
    /// A floating orb.
    Orb,
    /// A document pedestal.
    Pedestal,
}

impl EntityPrimitive {
    /// The canonical lowercase token.
    pub fn as_str(self) -> &'static str {
        match self {
            EntityPrimitive::Monolith => "monolith",
            EntityPrimitive::Kiosk => "kiosk",
            EntityPrimitive::Orb => "orb",
            EntityPrimitive::Pedestal => "pedestal",
        }
    }
}

/// An avatar interaction verb afforded by an entity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InteractionVerb {
    /// Read the full provision.
    Inspect,
    /// Acknowledge an obligation.
    Acknowledge,
    /// Accept a granted right.
    Accept,
    /// Relinquish a revoked right.
    Relinquish,
    /// Perform a monetary transfer.
    Transfer,
    /// Review a prohibition / warning.
    Review,
    /// Apply a status change.
    Update,
}

impl InteractionVerb {
    /// The canonical lowercase token.
    pub fn as_str(self) -> &'static str {
        match self {
            InteractionVerb::Inspect => "inspect",
            InteractionVerb::Acknowledge => "acknowledge",
            InteractionVerb::Accept => "accept",
            InteractionVerb::Relinquish => "relinquish",
            InteractionVerb::Transfer => "transfer",
            InteractionVerb::Review => "review",
            InteractionVerb::Update => "update",
        }
    }

    /// The primary interaction verb afforded by an effect type.
    pub fn for_effect(effect_type: &str) -> Self {
        match effect_type {
            "grant" => InteractionVerb::Accept,
            "revoke" => InteractionVerb::Relinquish,
            "obligation" => InteractionVerb::Acknowledge,
            "prohibition" => InteractionVerb::Review,
            "monetary_transfer" => InteractionVerb::Transfer,
            "status_change" => InteractionVerb::Update,
            _ => InteractionVerb::Inspect,
        }
    }
}

/// An interaction afforded by an entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interaction {
    /// The interaction verb.
    pub verb: InteractionVerb,
    /// Avatar-facing prompt text.
    pub prompt: String,
    /// Canonical condition string gating the interaction, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requires: Option<String>,
}

/// The visual model of an entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityModel {
    /// Visual primitive.
    pub primitive: EntityPrimitive,
    /// Bounding extents (metres).
    pub bounds: Vec3,
}

/// A placeable, interactable legal entity in the world.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaverseEntity {
    /// Stable entity identifier.
    pub id: String,
    /// Source statute identifier.
    pub source_id: String,
    /// Avatar-facing display name (statute title).
    pub display_name: String,
    /// Placement transform.
    pub transform: Transform,
    /// Visual model.
    pub model: EntityModel,
    /// Entity colour (derived from effect type).
    pub color: Color,
    /// Interactions afforded to avatars.
    pub interactions: Vec<Interaction>,
    /// Markdown detail shown on inspect.
    pub detail: String,
}

/// World-level metadata for a metaverse scene.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldMetadata {
    /// Coordinate system convention.
    pub coordinate_system: String,
    /// Avatar spawn point.
    pub spawn_point: Vec3,
    /// Scale unit (e.g. `meter`).
    pub scale_unit: String,
    /// Environment / skybox label.
    pub environment: String,
}

impl Default for WorldMetadata {
    fn default() -> Self {
        Self {
            coordinate_system: "right_handed_y_up".to_string(),
            spawn_point: Vec3::new(0.0, 1.7, 0.0),
            scale_unit: "meter".to_string(),
            environment: "neutral_atrium".to_string(),
        }
    }
}

/// A portal teleporting an avatar between two linked entities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Portal {
    /// Source entity id.
    pub from_entity: String,
    /// Target entity id.
    pub to_entity: String,
    /// Portal kind (e.g. `lineage`).
    pub kind: String,
}

/// A metaverse-native scene representation of a legal document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaverseScene {
    /// Schema identifier ([`SCHEMA`]).
    pub schema: String,
    /// World metadata.
    pub world: WorldMetadata,
    /// Placed entities (one per statute).
    pub entities: Vec<MetaverseEntity>,
    /// Portals between linked entities.
    pub portals: Vec<Portal>,
    /// Structured provenance enabling lossless reconstruction.
    pub provenance: Vec<StructuredStatute>,
}

impl MetaverseScene {
    /// Builds a metaverse scene from statutes using the given configuration.
    pub fn build(statutes: &[Statute], config: MetaverseConfig) -> Self {
        let count = statutes.len();
        let entities: Vec<MetaverseEntity> = statutes
            .iter()
            .enumerate()
            .map(|(index, statute)| {
                let transform = layout_transform(index, count, config.layout, config.spacing);
                let effect = effect_type_to_str(&statute.effect.effect_type);
                let salience = condition_salience(statute.preconditions.len());
                MetaverseEntity {
                    id: entity_id(index, statute),
                    source_id: statute.id.clone(),
                    display_name: statute.title.clone(),
                    transform,
                    model: EntityModel {
                        primitive: config.entity_primitive,
                        bounds: Vec3::new(1.0, round3(2.0 * salience), 1.0),
                    },
                    color: effect_color(effect),
                    interactions: build_interactions(statute, effect),
                    detail: render_statute_markdown(statute),
                }
            })
            .collect();

        let portals = build_portals(statutes, &entities);

        Self {
            schema: SCHEMA.to_string(),
            world: WorldMetadata::default(),
            entities,
            portals,
            provenance: build_structured(statutes),
        }
    }

    /// Number of entities in the scene.
    pub fn entity_count(&self) -> usize {
        self.entities.len()
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
            InteropError::SerializationError(format!(
                "Failed to serialize metaverse scene: {error}"
            ))
        })
    }

    /// Parses a scene from JSON.
    pub fn from_json(source: &str) -> InteropResult<Self> {
        serde_json::from_str(source).map_err(|error| {
            InteropError::ParseError(format!("Failed to parse metaverse JSON: {error}"))
        })
    }
}

fn entity_id(index: usize, statute: &Statute) -> String {
    format!("entity-{index:04}-{}", statute.id)
}

fn build_interactions(statute: &Statute, effect: &str) -> Vec<Interaction> {
    let mut interactions = vec![Interaction {
        verb: InteractionVerb::Inspect,
        prompt: format!("Inspect \"{}\"", statute.title),
        requires: None,
    }];
    let verb = InteractionVerb::for_effect(effect);
    if verb != InteractionVerb::Inspect {
        // Gate the primary interaction on the first precondition, if present.
        let requires = statute
            .preconditions
            .first()
            .map(crate::formats_nextgen::render_condition);
        interactions.push(Interaction {
            verb,
            prompt: format!("{} ({})", verb.as_str(), statute.effect.description),
            requires,
        });
    }
    interactions
}

fn build_portals(statutes: &[Statute], entities: &[MetaverseEntity]) -> Vec<Portal> {
    let mut portals = Vec::new();
    for (statute, entity) in statutes.iter().zip(entities.iter()) {
        for source in &statute.derives_from {
            if let Some(target) = entities
                .iter()
                .find(|candidate| &candidate.source_id == source)
            {
                portals.push(Portal {
                    from_entity: entity.id.clone(),
                    to_entity: target.id.clone(),
                    kind: "lineage".to_string(),
                });
            }
        }
    }
    portals
}

/// Importer for the metaverse-native format.
#[derive(Debug, Default)]
pub struct MetaverseLegalImporter;

impl MetaverseLegalImporter {
    /// Creates a new importer.
    pub fn new() -> Self {
        Self
    }
}

impl FormatImporter for MetaverseLegalImporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::MetaverseLegal
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let scene = MetaverseScene::from_json(source)?;
        let statutes = scene.to_statutes();
        let mut report = ConversionReport::new(LegalFormat::MetaverseLegal, LegalFormat::Legalis);
        report.statutes_converted = statutes.len();
        Ok((statutes, report))
    }

    fn validate(&self, source: &str) -> bool {
        schema_matches(source, SCHEMA)
    }
}

/// Exporter for the metaverse-native format.
#[derive(Debug, Clone, Copy)]
pub struct MetaverseLegalExporter {
    config: MetaverseConfig,
}

impl MetaverseLegalExporter {
    /// Creates an exporter with default configuration.
    pub fn new() -> Self {
        Self {
            config: MetaverseConfig::default(),
        }
    }

    /// Sets the scene generation configuration.
    pub fn with_config(mut self, config: MetaverseConfig) -> Self {
        self.config = config;
        self
    }
}

impl Default for MetaverseLegalExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for MetaverseLegalExporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::MetaverseLegal
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let scene = MetaverseScene::build(statutes, self.config);
        let json = scene.to_json()?;
        let mut report = ConversionReport::new(LegalFormat::Legalis, LegalFormat::MetaverseLegal);
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
                "charter",
                "Founding Charter",
                Effect::new(EffectType::Grant, "Grant membership"),
            ),
            Statute::new(
                "dues",
                "Membership Dues",
                Effect::new(EffectType::MonetaryTransfer, "Pay annual dues"),
            )
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            })
            .with_derives_from("charter"),
            Statute::new(
                "ban",
                "Conduct Ban",
                Effect::new(EffectType::Prohibition, "No disruptive conduct"),
            ),
        ]
    }

    #[test]
    fn test_build_entities_and_world() {
        let scene = MetaverseScene::build(&statutes(), MetaverseConfig::default());
        assert_eq!(scene.entity_count(), 3);
        assert_eq!(scene.world.scale_unit, "meter");
        assert!(scene.world.spawn_point.y > 0.0);
        assert_eq!(scene.entities[0].model.primitive, EntityPrimitive::Monolith);
        assert_eq!(
            scene.entities[0].color.to_hex(),
            effect_color("grant").to_hex()
        );
    }

    #[test]
    fn test_interactions_by_effect() {
        let scene = MetaverseScene::build(&statutes(), MetaverseConfig::default());
        // Every entity affords Inspect.
        for entity in &scene.entities {
            assert!(
                entity
                    .interactions
                    .iter()
                    .any(|interaction| interaction.verb == InteractionVerb::Inspect)
            );
        }
        // Grant -> Accept, MonetaryTransfer -> Transfer, Prohibition -> Review.
        assert!(
            scene.entities[0]
                .interactions
                .iter()
                .any(|i| i.verb == InteractionVerb::Accept)
        );
        let transfer = &scene.entities[1];
        let primary = transfer
            .interactions
            .iter()
            .find(|i| i.verb == InteractionVerb::Transfer)
            .expect("transfer interaction");
        // Gated on the first precondition.
        assert_eq!(primary.requires.as_deref(), Some("age >= 18"));
        assert!(
            scene.entities[2]
                .interactions
                .iter()
                .any(|i| i.verb == InteractionVerb::Review)
        );
    }

    #[test]
    fn test_portals_from_derivations() {
        let scene = MetaverseScene::build(&statutes(), MetaverseConfig::default());
        assert_eq!(scene.portals.len(), 1);
        assert_eq!(scene.portals[0].kind, "lineage");
        assert_eq!(scene.portals[0].from_entity, scene.entities[1].id);
        assert_eq!(scene.portals[0].to_entity, scene.entities[0].id);
    }

    #[test]
    fn test_export_import_roundtrip() {
        let exporter = MetaverseLegalExporter::new();
        let importer = MetaverseLegalImporter::new();
        let (json, export_report) = exporter.export(&statutes()).expect("export");
        assert_eq!(export_report.statutes_converted, 3);

        let (imported, import_report) = importer.import(&json).expect("import");
        assert_eq!(import_report.statutes_converted, 3);
        assert_eq!(imported.len(), 3);
        assert_eq!(imported[1].derives_from, vec!["charter".to_string()]);
        assert_eq!(imported[1].effect.effect_type, EffectType::MonetaryTransfer);
        assert_eq!(imported[1].preconditions.len(), 1);
    }

    #[test]
    fn test_validate_and_verb_codecs() {
        let importer = MetaverseLegalImporter::new();
        let (json, _) = MetaverseLegalExporter::new()
            .export(&statutes())
            .expect("export");
        assert!(importer.validate(&json));
        assert!(!importer.validate("{\"schema\":\"legalis.holographic-display/v1\"}"));

        assert_eq!(
            InteractionVerb::for_effect("grant"),
            InteractionVerb::Accept
        );
        assert_eq!(
            InteractionVerb::for_effect("monetary_transfer"),
            InteractionVerb::Transfer
        );
        assert_eq!(
            InteractionVerb::for_effect("custom"),
            InteractionVerb::Inspect
        );
        assert_eq!(EntityPrimitive::Kiosk.as_str(), "kiosk");
    }
}
