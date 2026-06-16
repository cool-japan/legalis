//! Cross-reality (immersive / spatial) legal document formats.
//!
//! This module groups a family of formats that project a legal corpus into
//! three-dimensional, immersive, and metaverse-native representations rather
//! than flat text or XML:
//!
//! - **VR/AR annotation** ([`vr_ar`]): legal annotations anchored to spatial
//!   anchors (world, image marker, plane, geo-location) for overlay in
//!   augmented- and virtual-reality scenes.
//! - **3D document** ([`document_3d`]): a 3D scene graph of statute panels with
//!   transforms and derivation edges, additionally renderable to an X3D-like
//!   XML projection.
//! - **Holographic display** ([`holographic`]): a depth-layered light-field
//!   representation with per-element parallax and luminance.
//! - **Spatial markup** ([`spatial_markup`]): a compact, human-readable,
//!   fully-parseable textual spatial markup language (`SLM`).
//! - **Metaverse-native** ([`metaverse`]): an interactive virtual-world scene
//!   graph with avatar interactions and portals between linked provisions.
//!
//! Every format embeds a [`StructuredStatute`] provenance record (reused from
//! [`crate::formats_nextgen`]) so that, in addition to its immersive view, it
//! losslessly round-trips the underlying [`Statute`] set through the standard
//! [`crate::FormatImporter`] / [`crate::FormatExporter`] pipeline.
//!
//! All geometry is computed by deterministic, dependency-free formulas (beyond
//! `serde`), keeping the workspace pure-Rust and reproducible.

pub mod document_3d;
pub mod holographic;
pub mod metaverse;
pub mod spatial_markup;
pub mod vr_ar;

use serde::{Deserialize, Serialize};
use std::f64::consts::{PI, TAU};

/// A 3D vector / point in a right-handed, Y-up coordinate space (metres).
///
/// The [`Default`] is the origin / zero vector (equivalently [`Vec3::zero`]).
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct Vec3 {
    /// X coordinate.
    pub x: f64,
    /// Y coordinate (up).
    pub y: f64,
    /// Z coordinate.
    pub z: f64,
}

impl Vec3 {
    /// Creates a new vector.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// The zero vector / origin.
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    /// A vector with all three components equal.
    pub fn splat(value: f64) -> Self {
        Self::new(value, value, value)
    }

    /// Component-wise addition.
    pub fn plus(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    /// Component-wise subtraction.
    pub fn minus(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    /// Uniform scaling by a scalar.
    pub fn scaled(self, factor: f64) -> Self {
        Self::new(self.x * factor, self.y * factor, self.z * factor)
    }

    /// Dot product.
    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Cross product.
    pub fn cross(self, other: Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    /// Squared Euclidean length.
    pub fn length_squared(self) -> f64 {
        self.dot(self)
    }

    /// Euclidean length (magnitude).
    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    /// Returns a unit-length copy; a zero vector is returned unchanged.
    pub fn normalized(self) -> Self {
        let length = self.length();
        if length > f64::EPSILON {
            self.scaled(1.0 / length)
        } else {
            self
        }
    }

    /// Euclidean distance to another point.
    pub fn distance(self, other: Self) -> f64 {
        self.minus(other).length()
    }

    /// Linear interpolation toward `other` by `t` (clamped to `[0, 1]`).
    pub fn lerp(self, other: Self, t: f64) -> Self {
        let t = t.clamp(0.0, 1.0);
        self.plus(other.minus(self).scaled(t))
    }
}

/// A unit quaternion describing an orientation (`x`, `y`, `z`, `w`).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Quaternion {
    /// Imaginary X component.
    pub x: f64,
    /// Imaginary Y component.
    pub y: f64,
    /// Imaginary Z component.
    pub z: f64,
    /// Real (scalar) component.
    pub w: f64,
}

impl Quaternion {
    /// Creates a new quaternion from raw components.
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    /// The identity (no rotation) quaternion.
    pub fn identity() -> Self {
        Self::new(0.0, 0.0, 0.0, 1.0)
    }

    /// Builds a rotation of `angle_rad` radians about the given (non-zero) axis.
    pub fn from_axis_angle(axis: Vec3, angle_rad: f64) -> Self {
        let axis = axis.normalized();
        let half = angle_rad * 0.5;
        let sin = half.sin();
        Self::new(axis.x * sin, axis.y * sin, axis.z * sin, half.cos()).normalized()
    }

    /// Builds a yaw rotation (about the +Y axis) of `angle_rad` radians.
    pub fn yaw(angle_rad: f64) -> Self {
        Self::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), angle_rad)
    }

    /// Returns a unit-length copy; a degenerate quaternion collapses to identity.
    pub fn normalized(self) -> Self {
        let norm = (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt();
        if norm > f64::EPSILON {
            Self::new(self.x / norm, self.y / norm, self.z / norm, self.w / norm)
        } else {
            Self::identity()
        }
    }

    /// Hamilton product `self ∘ other` (apply `other` then `self`).
    pub fn compose(self, other: Self) -> Self {
        Self::new(
            self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y,
            self.w * other.y - self.x * other.z + self.y * other.w + self.z * other.x,
            self.w * other.z + self.x * other.y - self.y * other.x + self.z * other.w,
            self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z,
        )
        .normalized()
    }
}

impl Default for Quaternion {
    fn default() -> Self {
        Self::identity()
    }
}

/// A rigid (plus uniform/non-uniform scale) spatial transform.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Transform {
    /// Translation.
    pub position: Vec3,
    /// Orientation.
    pub rotation: Quaternion,
    /// Per-axis scale.
    pub scale: Vec3,
}

impl Transform {
    /// Creates a transform from its parts.
    pub fn new(position: Vec3, rotation: Quaternion, scale: Vec3) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }

    /// The identity transform (origin, no rotation, unit scale).
    pub fn identity() -> Self {
        Self::new(Vec3::zero(), Quaternion::identity(), Vec3::splat(1.0))
    }

    /// A transform placed at `position` with identity rotation and unit scale.
    pub fn at(position: Vec3) -> Self {
        Self::new(position, Quaternion::identity(), Vec3::splat(1.0))
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::identity()
    }
}

/// An RGBA colour with components in `[0, 1]`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Color {
    /// Red.
    pub r: f64,
    /// Green.
    pub g: f64,
    /// Blue.
    pub b: f64,
    /// Alpha (opacity).
    pub a: f64,
}

impl Color {
    /// Creates an opaque colour from RGB components.
    pub fn rgb(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    /// Creates a colour from RGBA components.
    pub fn rgba(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self { r, g, b, a }
    }

    /// Renders the colour as a `#rrggbb` hex string (alpha omitted).
    pub fn to_hex(self) -> String {
        let to_byte = |value: f64| (value.clamp(0.0, 1.0) * 255.0).round() as u8;
        format!(
            "#{:02x}{:02x}{:02x}",
            to_byte(self.r),
            to_byte(self.g),
            to_byte(self.b)
        )
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::rgb(1.0, 1.0, 1.0)
    }
}

/// An axis-aligned bounding box.
///
/// The [`Default`] is the degenerate origin box (equivalently [`Aabb::empty`]).
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct Aabb {
    /// Minimum corner.
    pub min: Vec3,
    /// Maximum corner.
    pub max: Vec3,
}

impl Aabb {
    /// Creates a box from explicit corners.
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    /// A degenerate (empty) box at the origin.
    pub fn empty() -> Self {
        Self::new(Vec3::zero(), Vec3::zero())
    }

    /// Computes the tight bounding box of a set of points; empty input yields
    /// an origin box.
    pub fn from_points(points: &[Vec3]) -> Self {
        let Some(first) = points.first() else {
            return Self::empty();
        };
        let mut min = *first;
        let mut max = *first;
        for point in &points[1..] {
            min = Vec3::new(min.x.min(point.x), min.y.min(point.y), min.z.min(point.z));
            max = Vec3::new(max.x.max(point.x), max.y.max(point.y), max.z.max(point.z));
        }
        Self::new(min, max)
    }

    /// The geometric centre of the box.
    pub fn center(self) -> Vec3 {
        self.min.plus(self.max).scaled(0.5)
    }

    /// The extent (max minus min) of the box.
    pub fn size(self) -> Vec3 {
        self.max.minus(self.min)
    }

    /// Whether a point lies within the (inclusive) box.
    pub fn contains(self, point: Vec3) -> bool {
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
            && point.z >= self.min.z
            && point.z <= self.max.z
    }
}

/// The kind of real-world feature a spatial anchor is bound to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnchorKind {
    /// A fixed point in the persistent world coordinate space.
    World,
    /// Tracked to a recognised 2D image marker.
    ImageMarker,
    /// A detected horizontal plane (floor, table).
    PlaneHorizontal,
    /// A detected vertical plane (wall).
    PlaneVertical,
    /// A geographic (latitude/longitude/altitude) anchor.
    GeoLocation,
    /// Tracked to a detected face.
    Face,
    /// Tracked to a recognised physical object.
    ObjectTracked,
}

impl AnchorKind {
    /// The canonical lowercase token for the anchor kind.
    pub fn as_str(self) -> &'static str {
        match self {
            AnchorKind::World => "world",
            AnchorKind::ImageMarker => "image_marker",
            AnchorKind::PlaneHorizontal => "plane_horizontal",
            AnchorKind::PlaneVertical => "plane_vertical",
            AnchorKind::GeoLocation => "geo_location",
            AnchorKind::Face => "face",
            AnchorKind::ObjectTracked => "object_tracked",
        }
    }

    /// Parses a canonical token, defaulting to [`AnchorKind::World`].
    pub fn parse(token: &str) -> Self {
        match token.trim().to_lowercase().as_str() {
            "image_marker" | "image" | "marker" => AnchorKind::ImageMarker,
            "plane_horizontal" | "plane_h" | "floor" => AnchorKind::PlaneHorizontal,
            "plane_vertical" | "plane_v" | "wall" => AnchorKind::PlaneVertical,
            "geo_location" | "geo" => AnchorKind::GeoLocation,
            "face" => AnchorKind::Face,
            "object_tracked" | "object" => AnchorKind::ObjectTracked,
            _ => AnchorKind::World,
        }
    }
}

/// A spatial anchor: a kind plus the transform at which content is placed, and
/// an optional reference identifier (marker name, geo-hash, object id, ...).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpatialAnchor {
    /// The feature the anchor is bound to.
    pub kind: AnchorKind,
    /// The placement transform within the anchor's frame.
    pub transform: Transform,
    /// Optional reference (marker id, geo-hash, tracked object id).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

impl SpatialAnchor {
    /// Creates a world anchor at the given transform.
    pub fn world(transform: Transform) -> Self {
        Self {
            kind: AnchorKind::World,
            transform,
            reference: None,
        }
    }

    /// Creates an anchor of the given kind at the given transform.
    pub fn new(kind: AnchorKind, transform: Transform) -> Self {
        Self {
            kind,
            transform,
            reference: None,
        }
    }
}

/// A deterministic spatial arrangement of items in a scene.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SceneLayout {
    /// A roughly-square grid on the floor (XZ) plane.
    Grid,
    /// A ring on the XZ plane, items facing the centre.
    Circle,
    /// An ascending helix, items facing the central axis.
    Helix,
    /// A vertical stack along the Y axis.
    Stack,
}

impl SceneLayout {
    /// The canonical lowercase token for the layout.
    pub fn as_str(self) -> &'static str {
        match self {
            SceneLayout::Grid => "grid",
            SceneLayout::Circle => "circle",
            SceneLayout::Helix => "helix",
            SceneLayout::Stack => "stack",
        }
    }

    /// Parses a canonical token, defaulting to [`SceneLayout::Grid`].
    pub fn parse(token: &str) -> Self {
        match token.trim().to_lowercase().as_str() {
            "circle" | "ring" => SceneLayout::Circle,
            "helix" | "spiral" => SceneLayout::Helix,
            "stack" | "column" => SceneLayout::Stack,
            _ => SceneLayout::Grid,
        }
    }
}

/// Computes deterministic positions for `count` items under a layout, using
/// `spacing` (metres) as the nominal inter-item distance.
pub fn layout_positions(count: usize, layout: SceneLayout, spacing: f64) -> Vec<Vec3> {
    let spacing = if spacing > f64::EPSILON { spacing } else { 1.0 };
    if count == 0 {
        return Vec::new();
    }
    match layout {
        SceneLayout::Grid => grid_positions(count, spacing),
        SceneLayout::Circle => circle_positions(count, spacing, 0.0),
        SceneLayout::Helix => helix_positions(count, spacing),
        SceneLayout::Stack => (0..count)
            .map(|index| Vec3::new(0.0, index as f64 * spacing, 0.0))
            .collect(),
    }
}

fn grid_positions(count: usize, spacing: f64) -> Vec<Vec3> {
    let columns = (count as f64).sqrt().ceil().max(1.0) as usize;
    let rows = count.div_ceil(columns);
    let offset_x = (columns as f64 - 1.0) * spacing * 0.5;
    let offset_z = (rows as f64 - 1.0) * spacing * 0.5;
    (0..count)
        .map(|index| {
            let column = index % columns;
            let row = index / columns;
            Vec3::new(
                column as f64 * spacing - offset_x,
                0.0,
                row as f64 * spacing - offset_z,
            )
        })
        .collect()
}

fn circle_positions(count: usize, spacing: f64, base_y: f64) -> Vec<Vec3> {
    if count == 1 {
        return vec![Vec3::new(0.0, base_y, 0.0)];
    }
    // Choose a radius so adjacent items sit roughly `spacing` apart.
    let radius = (spacing * count as f64) / TAU;
    (0..count)
        .map(|index| {
            let angle = TAU * (index as f64) / (count as f64);
            Vec3::new(radius * angle.cos(), base_y, radius * angle.sin())
        })
        .collect()
}

fn helix_positions(count: usize, spacing: f64) -> Vec<Vec3> {
    let radius = spacing.max(1.0);
    let per_turn = 8usize;
    (0..count)
        .map(|index| {
            let angle = TAU * (index as f64) / (per_turn as f64);
            let height = (index as f64) * spacing * 0.5;
            Vec3::new(radius * angle.cos(), height, radius * angle.sin())
        })
        .collect()
}

/// A yaw-only rotation orienting an item at `position` to face `target` (about
/// the +Y axis). Items directly above/below the target keep identity yaw.
pub fn face_target_yaw(position: Vec3, target: Vec3) -> Quaternion {
    let delta = target.minus(position);
    if delta.x.abs() < f64::EPSILON && delta.z.abs() < f64::EPSILON {
        return Quaternion::identity();
    }
    // +Z is taken as "forward"; yaw measured from +Z toward +X.
    let yaw = delta.x.atan2(delta.z);
    Quaternion::yaw(yaw)
}

/// Builds the full placement transform for item `index` of `count` under a
/// layout, orienting circle/helix items toward the central axis.
pub fn layout_transform(
    index: usize,
    count: usize,
    layout: SceneLayout,
    spacing: f64,
) -> Transform {
    let positions = layout_positions(count, layout, spacing);
    let position = positions.get(index).copied().unwrap_or_else(Vec3::zero);
    let rotation = match layout {
        SceneLayout::Circle => face_target_yaw(position, Vec3::new(0.0, position.y, 0.0)),
        SceneLayout::Helix => face_target_yaw(position, Vec3::new(0.0, position.y, 0.0)),
        SceneLayout::Grid | SceneLayout::Stack => Quaternion::identity(),
    };
    Transform::new(position, rotation, Vec3::splat(1.0))
}

/// Maps a canonical effect-type token to a deterministic display colour for
/// immersive visualisation.
pub fn effect_color(effect_type: &str) -> Color {
    match effect_type {
        "grant" => Color::rgb(0.20, 0.70, 0.32),
        "revoke" => Color::rgb(0.82, 0.22, 0.22),
        "obligation" => Color::rgb(0.20, 0.42, 0.82),
        "prohibition" => Color::rgb(0.92, 0.52, 0.12),
        "monetary_transfer" => Color::rgb(0.85, 0.70, 0.20),
        "status_change" => Color::rgb(0.58, 0.32, 0.72),
        _ => Color::rgb(0.55, 0.55, 0.55),
    }
}

/// Rounds an `f64` to three decimal places for compact, stable text encoding.
pub fn round3(value: f64) -> f64 {
    (value * 1000.0).round() / 1000.0
}

/// Maps the number of preconditions to a salience scalar in `[1.0, 2.0]`, used
/// to emphasise more heavily-conditioned provisions in immersive layouts.
pub fn condition_salience(condition_count: usize) -> f64 {
    1.0 + (condition_count as f64 / (condition_count as f64 + 4.0))
}

/// Maps a depth (metres) to a parallax factor in `[0, 1]`: nearer layers move
/// more with viewpoint changes. `max_depth` of zero yields a flat field.
pub fn depth_parallax(depth: f64, max_depth: f64) -> f64 {
    if max_depth <= f64::EPSILON {
        return 0.0;
    }
    (1.0 - (depth / max_depth)).clamp(0.0, 1.0)
}

/// One full turn in radians (re-exported for downstream callers).
pub const FULL_TURN: f64 = TAU;

/// Half a turn in radians (re-exported for downstream callers).
pub const HALF_TURN: f64 = PI;

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    fn approx_vec(a: Vec3, b: Vec3) -> bool {
        approx(a.x, b.x) && approx(a.y, b.y) && approx(a.z, b.z)
    }

    #[test]
    fn test_vec3_arithmetic() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        assert!(approx_vec(a.plus(b), Vec3::new(5.0, 7.0, 9.0)));
        assert!(approx_vec(b.minus(a), Vec3::new(3.0, 3.0, 3.0)));
        assert!(approx_vec(a.scaled(2.0), Vec3::new(2.0, 4.0, 6.0)));
        assert!(approx(a.dot(b), 32.0));
        assert!(approx_vec(
            Vec3::new(1.0, 0.0, 0.0).cross(Vec3::new(0.0, 1.0, 0.0)),
            Vec3::new(0.0, 0.0, 1.0)
        ));
    }

    #[test]
    fn test_vec3_length_and_normalize() {
        let v = Vec3::new(3.0, 4.0, 0.0);
        assert!(approx(v.length(), 5.0));
        let unit = v.normalized();
        assert!(approx(unit.length(), 1.0));
        assert!(approx_vec(Vec3::zero().normalized(), Vec3::zero()));
        assert!(approx(
            Vec3::new(0.0, 0.0, 0.0).distance(Vec3::new(0.0, 3.0, 4.0)),
            5.0
        ));
    }

    #[test]
    fn test_vec3_lerp_clamps() {
        let a = Vec3::zero();
        let b = Vec3::new(10.0, 0.0, 0.0);
        assert!(approx_vec(a.lerp(b, 0.5), Vec3::new(5.0, 0.0, 0.0)));
        assert!(approx_vec(a.lerp(b, -1.0), a));
        assert!(approx_vec(a.lerp(b, 2.0), b));
    }

    #[test]
    fn test_quaternion_unit_and_identity_compose() {
        let q = Quaternion::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), HALF_TURN * 0.5);
        let norm = (q.x * q.x + q.y * q.y + q.z * q.z + q.w * q.w).sqrt();
        assert!(approx(norm, 1.0));
        let composed = q.compose(Quaternion::identity());
        assert!(approx(composed.x, q.x) && approx(composed.w, q.w));
        // Degenerate quaternion collapses to identity.
        assert_eq!(
            Quaternion::new(0.0, 0.0, 0.0, 0.0).normalized(),
            Quaternion::identity()
        );
    }

    #[test]
    fn test_transform_and_color_defaults() {
        let t = Transform::default();
        assert_eq!(t, Transform::identity());
        assert!(approx_vec(t.scale, Vec3::splat(1.0)));
        assert_eq!(Color::default().to_hex(), "#ffffff");
        assert_eq!(Color::rgb(1.0, 0.0, 0.0).to_hex(), "#ff0000");
        assert_eq!(
            effect_color("grant").to_hex(),
            Color::rgb(0.20, 0.70, 0.32).to_hex()
        );
    }

    #[test]
    fn test_aabb_bounds() {
        let points = vec![
            Vec3::new(-1.0, 0.0, 2.0),
            Vec3::new(3.0, 5.0, -4.0),
            Vec3::new(0.0, 1.0, 0.0),
        ];
        let box_ = Aabb::from_points(&points);
        assert!(approx_vec(box_.min, Vec3::new(-1.0, 0.0, -4.0)));
        assert!(approx_vec(box_.max, Vec3::new(3.0, 5.0, 2.0)));
        assert!(box_.contains(Vec3::new(0.0, 1.0, 0.0)));
        assert!(!box_.contains(Vec3::new(10.0, 10.0, 10.0)));
        assert_eq!(Aabb::from_points(&[]), Aabb::empty());
    }

    #[test]
    fn test_layout_positions_are_deterministic_and_sized() {
        for layout in [
            SceneLayout::Grid,
            SceneLayout::Circle,
            SceneLayout::Helix,
            SceneLayout::Stack,
        ] {
            let first = layout_positions(7, layout, 2.0);
            let second = layout_positions(7, layout, 2.0);
            assert_eq!(first.len(), 7);
            assert_eq!(
                first,
                second,
                "layout {} not deterministic",
                layout.as_str()
            );
        }
        assert!(layout_positions(0, SceneLayout::Grid, 1.0).is_empty());
        // A single circle item sits at the origin.
        assert!(approx_vec(
            layout_positions(1, SceneLayout::Circle, 2.0)[0],
            Vec3::zero()
        ));
    }

    #[test]
    fn test_circle_radius_tracks_spacing() {
        let positions = layout_positions(12, SceneLayout::Circle, 1.5);
        let radius = positions[0].length();
        // Adjacent chord length should be close to the requested spacing.
        let chord = positions[0].distance(positions[1]);
        assert!(radius > 0.0);
        assert!((chord - 1.5).abs() < 0.2, "chord {chord} far from spacing");
    }

    #[test]
    fn test_face_target_yaw_and_layout_transform() {
        // Item on +X facing the origin yaws by -90 degrees about Y.
        let yaw = face_target_yaw(Vec3::new(1.0, 0.0, 0.0), Vec3::zero());
        assert_ne!(yaw, Quaternion::identity());
        // Directly above the target keeps identity yaw.
        assert_eq!(
            face_target_yaw(Vec3::new(0.0, 5.0, 0.0), Vec3::zero()),
            Quaternion::identity()
        );
        let transform = layout_transform(0, 4, SceneLayout::Grid, 2.0);
        assert_eq!(transform.rotation, Quaternion::identity());
    }

    #[test]
    fn test_anchor_and_layout_codecs() {
        for kind in [
            AnchorKind::World,
            AnchorKind::ImageMarker,
            AnchorKind::PlaneHorizontal,
            AnchorKind::PlaneVertical,
            AnchorKind::GeoLocation,
            AnchorKind::Face,
            AnchorKind::ObjectTracked,
        ] {
            assert_eq!(AnchorKind::parse(kind.as_str()), kind);
        }
        for layout in [
            SceneLayout::Grid,
            SceneLayout::Circle,
            SceneLayout::Helix,
            SceneLayout::Stack,
        ] {
            assert_eq!(SceneLayout::parse(layout.as_str()), layout);
        }
        assert_eq!(AnchorKind::parse("unknown"), AnchorKind::World);
        assert_eq!(SceneLayout::parse("unknown"), SceneLayout::Grid);
    }

    #[test]
    fn test_scalar_helpers() {
        assert!(approx(round3(1.23456), 1.235));
        assert!(condition_salience(0) >= 1.0 && condition_salience(0) <= 2.0);
        assert!(condition_salience(10) > condition_salience(1));
        assert!(approx(depth_parallax(0.0, 4.0), 1.0));
        assert!(approx(depth_parallax(4.0, 4.0), 0.0));
        assert!(approx(depth_parallax(1.0, 0.0), 0.0));
    }
}
