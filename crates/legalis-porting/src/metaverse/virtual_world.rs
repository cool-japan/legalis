//! Virtual-world jurisdiction porting.
//!
//! A [`VirtualJurisdiction`] models a virtual world as a tree of
//! [`VirtualSpace`]s. The tree mirrors how virtual worlds are actually carved
//! up: a top-level [`SpaceKind::Realm`] contains [`SpaceKind::Server`]s, which
//! contain [`SpaceKind::Shard`]s, down to individual [`SpaceKind::Parcel`]s of
//! land. Real-world *territorial* concepts (a country, a state, a city) do not
//! exist inside a virtual world, so porting a real statute in requires deciding
//! *where* in the space tree it applies. That decision is captured by a
//! [`TerritorialProjection`]: a mapping from a real-world region identifier to a
//! virtual space.
//!
//! Porting a real statute *into* a virtual world rewrites every
//! [`legalis_core::Condition::Geographic`] precondition whose region is covered
//! by the projection, replacing the real region id with the virtual space id and
//! recording the rewrite as a [`PortingChange`]. Porting *out of* a virtual world
//! performs the inverse, recovering the real region behind a virtual space.

use super::sha256_parts;
use crate::{ChangeType, PortedStatute, PortingChange, PortingError};
use legalis_core::{Condition, RegionType, Statute};
use legalis_i18n::Locale;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

type VirtualResult<T> = Result<T, PortingError>;

/// The granularity of a [`VirtualSpace`] within a virtual world, ordered from
/// the widest (a whole realm) to the narrowest (a single land parcel).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SpaceKind {
    /// A whole virtual world / realm — the widest scope.
    Realm = 1,
    /// A server / region cluster within a realm.
    Server = 2,
    /// A shard / instance within a server.
    Shard = 3,
    /// A zone / district within a shard.
    Zone = 4,
    /// A single parcel of virtual land — the narrowest scope.
    Parcel = 5,
}

impl SpaceKind {
    /// The conventional [`RegionType`] a real-world region of comparable breadth
    /// would carry, used when projecting real regions onto virtual spaces.
    pub fn analogous_region_type(self) -> RegionType {
        match self {
            SpaceKind::Realm => RegionType::Country,
            SpaceKind::Server => RegionType::State,
            SpaceKind::Shard | SpaceKind::Zone => RegionType::City,
            SpaceKind::Parcel => RegionType::Custom,
        }
    }
}

/// A node in a virtual world's space tree.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VirtualSpace {
    /// Stable identifier, unique within its jurisdiction.
    pub id: String,
    /// Human-readable name.
    pub name: String,
    /// Granularity of this space.
    pub kind: SpaceKind,
    /// Child spaces (the next level down the tree).
    pub children: Vec<VirtualSpace>,
}

impl VirtualSpace {
    /// Creates a leaf space (no children).
    pub fn new(id: impl Into<String>, name: impl Into<String>, kind: SpaceKind) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            kind,
            children: Vec::new(),
        }
    }

    /// Builder: attaches a child space.
    pub fn with_child(mut self, child: VirtualSpace) -> Self {
        self.children.push(child);
        self
    }

    /// Attaches a child space in place.
    pub fn add_child(&mut self, child: VirtualSpace) {
        self.children.push(child);
    }

    /// Total number of spaces in this subtree, including `self`.
    pub fn subtree_size(&self) -> usize {
        1 + self
            .children
            .iter()
            .map(VirtualSpace::subtree_size)
            .sum::<usize>()
    }

    /// Depth-first search for a descendant (or `self`) by id.
    pub fn find(&self, id: &str) -> Option<&VirtualSpace> {
        if self.id == id {
            return Some(self);
        }
        self.children.iter().find_map(|c| c.find(id))
    }

    /// Whether `descendant_id` names `self` or any space beneath it (containment).
    pub fn contains(&self, descendant_id: &str) -> bool {
        self.find(descendant_id).is_some()
    }

    /// Ids of every space in this subtree, in depth-first pre-order.
    pub fn descendant_ids(&self) -> Vec<String> {
        let mut out = vec![self.id.clone()];
        for child in &self.children {
            out.extend(child.descendant_ids());
        }
        out
    }
}

/// The spatial scope a ported rule applies to inside a virtual world.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpaceScope {
    /// Applies across the entire virtual world (every space).
    WholeWorld,
    /// Applies to a named space and everything beneath it.
    Space(String),
}

/// A projection of a real-world territorial region onto a virtual space.
///
/// This is the bridge that gives a placeless virtual world a "where": it asserts
/// that, for porting purposes, real region `real_region_id` (of type
/// `real_region_type`) corresponds to virtual space `virtual_space_id`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerritorialProjection {
    /// The real-world region type being projected.
    pub real_region_type: RegionType,
    /// The real-world region identifier.
    pub real_region_id: String,
    /// The virtual space the real region maps onto.
    pub virtual_space_id: String,
}

impl TerritorialProjection {
    /// Creates a projection from a real region onto a virtual space.
    pub fn new(
        real_region_type: RegionType,
        real_region_id: impl Into<String>,
        virtual_space_id: impl Into<String>,
    ) -> Self {
        Self {
            real_region_type,
            real_region_id: real_region_id.into(),
            virtual_space_id: virtual_space_id.into(),
        }
    }
}

/// A virtual world modelled as a jurisdiction: a named space tree plus the
/// territorial projections that tie real regions to virtual spaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VirtualJurisdiction {
    /// Stable identifier for the virtual world.
    pub id: String,
    /// Human-readable name.
    pub name: String,
    /// Root of the space tree (e.g. the realm), if configured.
    pub root: Option<VirtualSpace>,
    /// Projections from real regions onto virtual spaces, keyed by
    /// `"<region_type>:<region_id>"`.
    projections: BTreeMap<String, TerritorialProjection>,
    /// Locale virtual rules are expressed in (defaults to a synthetic `xv`
    /// locale tagged with the world id's country slot).
    pub locale: Locale,
}

impl VirtualJurisdiction {
    /// Creates a virtual jurisdiction with no spaces yet.
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            root: None,
            projections: BTreeMap::new(),
            locale: Locale::new("xv"),
        }
    }

    /// Builder: sets a locale for the virtual world's rules.
    pub fn with_locale(mut self, locale: Locale) -> Self {
        self.locale = locale;
        self
    }

    /// Sets the root space (the realm) of the world.
    pub fn set_root(&mut self, root: VirtualSpace) {
        self.root = Some(root);
    }

    /// Builder form of [`VirtualJurisdiction::set_root`].
    pub fn with_root(mut self, root: VirtualSpace) -> Self {
        self.root = Some(root);
        self
    }

    /// Total number of spaces in the world (0 if no root).
    pub fn space_count(&self) -> usize {
        self.root
            .as_ref()
            .map(VirtualSpace::subtree_size)
            .unwrap_or(0)
    }

    /// Finds a space by id anywhere in the tree.
    pub fn find_space(&self, id: &str) -> Option<&VirtualSpace> {
        self.root.as_ref().and_then(|r| r.find(id))
    }

    /// Builds the lookup key for a real region.
    fn projection_key(region_type: RegionType, region_id: &str) -> String {
        format!("{region_type:?}:{region_id}")
    }

    /// Registers a territorial projection.
    ///
    /// # Errors
    ///
    /// Returns [`PortingError::InvalidInput`] if the target virtual space does
    /// not exist in this world.
    pub fn add_projection(&mut self, projection: TerritorialProjection) -> VirtualResult<()> {
        if self.find_space(&projection.virtual_space_id).is_none() {
            return Err(PortingError::InvalidInput(format!(
                "virtual world '{}': projection targets unknown space '{}'",
                self.id, projection.virtual_space_id
            )));
        }
        let key = Self::projection_key(projection.real_region_type, &projection.real_region_id);
        self.projections.insert(key, projection);
        Ok(())
    }

    /// Looks up the virtual space a real region projects onto, if any.
    pub fn project_region(&self, region_type: RegionType, region_id: &str) -> Option<&str> {
        self.projections
            .get(&Self::projection_key(region_type, region_id))
            .map(|p| p.virtual_space_id.as_str())
    }

    /// Inverse lookup: the real region projecting onto a given virtual space.
    pub fn unproject_space(&self, virtual_space_id: &str) -> Option<&TerritorialProjection> {
        self.projections
            .values()
            .find(|p| p.virtual_space_id == virtual_space_id)
    }

    /// Number of registered projections.
    pub fn projection_count(&self) -> usize {
        self.projections.len()
    }

    /// A content hash binding the world's identity, space tree and projections.
    pub fn content_hash(&self) -> String {
        let spaces = self
            .root
            .as_ref()
            .map(|r| r.descendant_ids().join(","))
            .unwrap_or_default();
        let projections = self
            .projections
            .values()
            .map(|p| {
                format!(
                    "{:?}:{}->{}",
                    p.real_region_type, p.real_region_id, p.virtual_space_id
                )
            })
            .collect::<Vec<_>>()
            .join(";");
        sha256_parts(&[
            self.id.as_bytes(),
            self.name.as_bytes(),
            spaces.as_bytes(),
            projections.as_bytes(),
        ])
    }

    /// Ports a real-world statute *into* this virtual world.
    ///
    /// Every [`Condition::Geographic`] precondition whose region projects onto a
    /// virtual space is rewritten to target that space; the rewrite is recorded
    /// as a [`PortingChange`]. A statute with no geographic preconditions is
    /// scoped to the whole world. The returned [`PortedStatute`] carries the
    /// virtual world's locale and a compatibility score reflecting how many
    /// territorial references could be projected.
    ///
    /// # Errors
    ///
    /// Returns [`PortingError::InvalidInput`] if the world has no root space.
    pub fn port_in(&self, statute: &Statute) -> VirtualResult<VirtualWorldPort> {
        if self.root.is_none() {
            return Err(PortingError::InvalidInput(format!(
                "virtual world '{}': cannot port into a world with no spaces",
                self.id
            )));
        }
        let mut ported = statute.clone();
        ported.id = format!("{}::{}", self.id, statute.id);
        ported.jurisdiction = Some(self.id.clone());

        let mut changes = Vec::new();
        let mut geographic_total = 0usize;
        let mut geographic_projected = 0usize;
        let mut scope = SpaceScope::WholeWorld;

        for condition in ported.preconditions.iter_mut() {
            if let Condition::Geographic {
                region_type,
                region_id,
            } = condition
            {
                geographic_total += 1;
                if let Some(space_id) = self
                    .projections
                    .get(&Self::projection_key(*region_type, region_id))
                    .map(|p| p.virtual_space_id.clone())
                {
                    let original = format!("{region_type:?}:{region_id}");
                    changes.push(PortingChange {
                        change_type: ChangeType::ValueAdaptation,
                        description: format!(
                            "Projected real region '{region_id}' onto virtual space '{space_id}'"
                        ),
                        original: Some(original),
                        adapted: Some(format!("Custom:{space_id}")),
                        reason: "Territorial concept mapped into virtual world space tree"
                            .to_string(),
                    });
                    geographic_projected += 1;
                    if geographic_projected == 1 {
                        scope = SpaceScope::Space(space_id.clone());
                    }
                    *region_type = RegionType::Custom;
                    *region_id = space_id;
                } else {
                    changes.push(PortingChange {
                        change_type: ChangeType::Incompatible,
                        description: format!(
                            "Real region '{region_id}' has no projection into virtual world '{}'",
                            self.id
                        ),
                        original: Some(format!("{region_type:?}:{region_id}")),
                        adapted: None,
                        reason: "No territorial projection registered for this region".to_string(),
                    });
                }
            }
        }

        if geographic_total == 0 {
            changes.push(PortingChange {
                change_type: ChangeType::CulturalAdaptation,
                description: "Statute had no territorial scope; applied to the whole virtual world"
                    .to_string(),
                original: None,
                adapted: Some(self.id.clone()),
                reason: "Placeless virtual worlds default unscoped rules to world-wide".to_string(),
            });
        }

        let compatibility_score = if geographic_total == 0 {
            1.0
        } else {
            geographic_projected as f64 / geographic_total as f64
        };

        let ported_statute = PortedStatute {
            original_id: statute.id.clone(),
            statute: ported,
            changes,
            locale: self.locale.clone(),
            compatibility_score,
        };
        Ok(VirtualWorldPort {
            world_id: self.id.clone(),
            scope,
            ported: ported_statute,
            unprojected_regions: geographic_total - geographic_projected,
        })
    }

    /// Ports a virtual-world statute *out* to the real world, recovering real
    /// regions behind virtual spaces.
    ///
    /// Each [`Condition::Geographic`] targeting a virtual space that has an
    /// inverse projection is rewritten back to the real region; the inverse
    /// rewrite is recorded as a [`PortingChange`]. The result is expressed in
    /// `target_locale`.
    ///
    /// # Errors
    ///
    /// Returns [`PortingError::InvalidInput`] if `statute` carries no
    /// preconditions at all that reference this world's spaces (nothing to
    /// recover) — callers should instead treat such a statute as already real.
    pub fn port_out(
        &self,
        statute: &Statute,
        target_locale: Locale,
    ) -> VirtualResult<VirtualWorldPort> {
        let mut ported = statute.clone();
        ported.id = statute
            .id
            .strip_prefix(&format!("{}::", self.id))
            .map(str::to_string)
            .unwrap_or_else(|| format!("real::{}", statute.id));
        ported.jurisdiction = None;

        let mut changes = Vec::new();
        let mut recovered = 0usize;
        let mut referenced = 0usize;

        for condition in ported.preconditions.iter_mut() {
            if let Condition::Geographic {
                region_type,
                region_id,
            } = condition
                && let Some(projection) = self.unproject_space(region_id)
            {
                referenced += 1;
                changes.push(PortingChange {
                    change_type: ChangeType::ValueAdaptation,
                    description: format!(
                        "Recovered real region '{}' behind virtual space '{}'",
                        projection.real_region_id, region_id
                    ),
                    original: Some(format!("{region_type:?}:{region_id}")),
                    adapted: Some(format!(
                        "{:?}:{}",
                        projection.real_region_type, projection.real_region_id
                    )),
                    reason: "Virtual space mapped back to real territory".to_string(),
                });
                *region_type = projection.real_region_type;
                *region_id = projection.real_region_id.clone();
                recovered += 1;
            }
        }

        if referenced == 0 {
            return Err(PortingError::InvalidInput(format!(
                "virtual world '{}': statute references no projected spaces to port out",
                self.id
            )));
        }

        let compatibility_score = recovered as f64 / referenced as f64;
        let ported_statute = PortedStatute {
            original_id: statute.id.clone(),
            statute: ported,
            changes,
            locale: target_locale,
            compatibility_score,
        };
        Ok(VirtualWorldPort {
            world_id: self.id.clone(),
            scope: SpaceScope::WholeWorld,
            ported: ported_statute,
            unprojected_regions: referenced - recovered,
        })
    }
}

/// The outcome of porting a statute into or out of a virtual world.
///
/// `PartialEq` is intentionally not derived because the embedded
/// [`PortedStatute`] (and its [`legalis_core::Statute`]) do not implement it;
/// compare via serde where structural equality is needed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualWorldPort {
    /// The virtual world involved.
    pub world_id: String,
    /// The spatial scope the ported rule applies to.
    pub scope: SpaceScope,
    /// The ported statute (with rewritten geographic scope and change log).
    pub ported: PortedStatute,
    /// Number of territorial references that could not be (un)projected.
    pub unprojected_regions: usize,
}

impl VirtualWorldPort {
    /// Whether every territorial reference was successfully (un)projected.
    pub fn is_fully_projected(&self) -> bool {
        self.unprojected_regions == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{ComparisonOp, Effect, EffectType};

    fn aurora() -> VirtualJurisdiction {
        let realm = VirtualSpace::new("realm-aurora", "Aurora", SpaceKind::Realm)
            .with_child(
                VirtualSpace::new("srv-eu", "EU Server", SpaceKind::Server).with_child(
                    VirtualSpace::new("shard-eu-1", "EU Shard 1", SpaceKind::Shard).with_child(
                        VirtualSpace::new("parcel-42", "Parcel 42", SpaceKind::Parcel),
                    ),
                ),
            )
            .with_child(VirtualSpace::new("srv-us", "US Server", SpaceKind::Server));
        let mut world = VirtualJurisdiction::new("mv-aurora", "Aurora Metaverse")
            .with_locale(Locale::new("xv").with_country("AU"));
        world.set_root(realm);
        world
    }

    fn geo_statute(region: &str) -> Statute {
        Statute::new(
            "zoning-1",
            "Zoning",
            Effect::new(EffectType::Grant, "Build"),
        )
        .with_precondition(Condition::Geographic {
            region_type: RegionType::Country,
            region_id: region.to_string(),
        })
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        })
    }

    #[test]
    fn test_space_tree_size_and_find() {
        let world = aurora();
        // realm + srv-eu + shard-eu-1 + parcel-42 + srv-us = 5 spaces.
        assert_eq!(world.space_count(), 5);
        assert!(world.find_space("parcel-42").is_some());
        assert!(world.find_space("nonexistent").is_none());
    }

    #[test]
    fn test_space_contains_and_descendants() {
        let world = aurora();
        let srv = world.find_space("srv-eu").expect("srv-eu");
        assert!(srv.contains("parcel-42"));
        assert!(!srv.contains("srv-us"));
        assert_eq!(srv.descendant_ids().len(), 3);
    }

    #[test]
    fn test_space_kind_ordering_and_region_type() {
        assert!(SpaceKind::Realm < SpaceKind::Parcel);
        assert_eq!(
            SpaceKind::Realm.analogous_region_type(),
            RegionType::Country
        );
        assert_eq!(SpaceKind::Server.analogous_region_type(), RegionType::State);
    }

    #[test]
    fn test_add_projection_rejects_unknown_space() {
        let mut world = aurora();
        let bad = TerritorialProjection::new(RegionType::Country, "DE", "no-such-space");
        assert!(world.add_projection(bad).is_err());
    }

    #[test]
    fn test_projection_roundtrip_lookup() {
        let mut world = aurora();
        world
            .add_projection(TerritorialProjection::new(
                RegionType::Country,
                "DE",
                "srv-eu",
            ))
            .expect("add");
        assert_eq!(
            world.project_region(RegionType::Country, "DE"),
            Some("srv-eu")
        );
        let inv = world.unproject_space("srv-eu").expect("inverse");
        assert_eq!(inv.real_region_id, "DE");
        assert_eq!(world.projection_count(), 1);
    }

    #[test]
    fn test_port_in_rewrites_geographic_scope() {
        let mut world = aurora();
        world
            .add_projection(TerritorialProjection::new(
                RegionType::Country,
                "DE",
                "srv-eu",
            ))
            .expect("add");
        let result = world.port_in(&geo_statute("DE")).expect("port in");
        assert!(result.is_fully_projected());
        assert_eq!(result.scope, SpaceScope::Space("srv-eu".to_string()));
        assert_eq!(
            result.ported.statute.jurisdiction.as_deref(),
            Some("mv-aurora")
        );
        assert_eq!(result.ported.statute.id, "mv-aurora::zoning-1");
        // The geographic precondition is now Custom -> srv-eu.
        let rewritten = result
            .ported
            .statute
            .preconditions
            .iter()
            .any(|c| matches!(c, Condition::Geographic { region_type: RegionType::Custom, region_id } if region_id == "srv-eu"));
        assert!(rewritten);
        assert!((result.ported.compatibility_score - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_port_in_unprojected_region_is_incompatible() {
        let world = aurora();
        let result = world.port_in(&geo_statute("DE")).expect("port in");
        assert!(!result.is_fully_projected());
        assert_eq!(result.unprojected_regions, 1);
        assert_eq!(result.ported.compatibility_score, 0.0);
        assert!(
            result
                .ported
                .changes
                .iter()
                .any(|c| c.change_type == ChangeType::Incompatible)
        );
    }

    #[test]
    fn test_port_in_no_geo_defaults_to_world() {
        let world = aurora();
        let statute = Statute::new("global-1", "Global", Effect::new(EffectType::Grant, "X"));
        let result = world.port_in(&statute).expect("port in");
        assert_eq!(result.scope, SpaceScope::WholeWorld);
        assert!((result.ported.compatibility_score - 1.0).abs() < f64::EPSILON);
        assert!(
            result
                .ported
                .changes
                .iter()
                .any(|c| c.change_type == ChangeType::CulturalAdaptation)
        );
    }

    #[test]
    fn test_port_in_empty_world_errors() {
        let world = VirtualJurisdiction::new("empty", "Empty");
        assert!(world.port_in(&geo_statute("DE")).is_err());
    }

    #[test]
    fn test_port_out_recovers_real_region() {
        let mut world = aurora();
        world
            .add_projection(TerritorialProjection::new(
                RegionType::Country,
                "DE",
                "srv-eu",
            ))
            .expect("add");
        let inward = world.port_in(&geo_statute("DE")).expect("port in");
        let outward = world
            .port_out(&inward.ported.statute, Locale::new("de").with_country("DE"))
            .expect("port out");
        assert!(outward.is_fully_projected());
        assert_eq!(outward.ported.statute.id, "zoning-1");
        assert!(outward.ported.statute.jurisdiction.is_none());
        let recovered = outward
            .ported
            .statute
            .preconditions
            .iter()
            .any(|c| matches!(c, Condition::Geographic { region_type: RegionType::Country, region_id } if region_id == "DE"));
        assert!(recovered);
    }

    #[test]
    fn test_port_out_without_projected_space_errors() {
        let world = aurora();
        let statute = geo_statute("DE"); // raw real region, never projected
        assert!(world.port_out(&statute, Locale::new("de")).is_err());
    }

    #[test]
    fn test_content_hash_changes_with_projection() {
        let mut world = aurora();
        let before = world.content_hash();
        world
            .add_projection(TerritorialProjection::new(
                RegionType::Country,
                "DE",
                "srv-eu",
            ))
            .expect("add");
        assert_ne!(before, world.content_hash());
    }

    #[test]
    fn test_virtual_world_serde_roundtrip() {
        let mut world = aurora();
        world
            .add_projection(TerritorialProjection::new(
                RegionType::Country,
                "DE",
                "srv-eu",
            ))
            .expect("add");
        let json = serde_json::to_string(&world).expect("ser");
        let back: VirtualJurisdiction = serde_json::from_str(&json).expect("de");
        assert_eq!(world, back);
    }

    #[test]
    fn test_port_result_serde_roundtrip() {
        let mut world = aurora();
        world
            .add_projection(TerritorialProjection::new(
                RegionType::Country,
                "DE",
                "srv-eu",
            ))
            .expect("add");
        let result = world.port_in(&geo_statute("DE")).expect("port in");
        let json = serde_json::to_string(&result).expect("ser");
        let back: VirtualWorldPort = serde_json::from_str(&json).expect("de");
        // PortedStatute lacks PartialEq, so compare via re-serialization.
        assert_eq!(json, serde_json::to_string(&back).expect("reser"));
        assert_eq!(back.world_id, result.world_id);
        assert_eq!(back.scope, result.scope);
    }
}
