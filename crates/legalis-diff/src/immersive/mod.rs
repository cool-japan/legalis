//! Immersive, 3-D and collaborative visualization of statute diffs (v0.5.5).
//!
//! Where [`crate::advanced_visual`] and [`crate::visual`] render *2-D* artifacts
//! (DOT graphs, SVG charts, HTML dashboards), this module models a diff as a
//! navigable **3-D scene graph** and layers immersive features on top of it.
//! Everything here is pure Rust and self-contained:
//!
//! - [`scene`] — a [`Scene3d`] graph (nodes with 3-D positions + typed edges)
//!   constructed from one or many [`crate::StatuteDiff`]s.
//! - [`layout`] — deterministic 3-D layout algorithms (force-directed,
//!   Fibonacci-sphere, BFS-layered, lattice grid) that position the nodes.
//! - [`xr`] — exporters that serialise a scene into immersive scene formats
//!   (A-Frame HTML for WebXR, X3D XML, and a glTF-like JSON manifest).
//! - [`navigation`] — an interactive graph navigator (focus, neighbour
//!   expansion, shortest-path, level-of-detail and a back/forward history).
//! - [`collab`] — real-time collaborative *visualization* sessions: a shared
//!   camera, per-participant 3-D cursors, annotations and a sequence-ordered
//!   event log with last-writer-wins conflict resolution.
//! - [`plugin`] — a [`plugin::VisualizationPlugin`] trait + registry for custom
//!   renderers (two built-ins are provided).
//!
//! # Deferred external binding
//!
//! Actually *rasterising* a scene on a GPU (WebGL / WebGPU) or driving a live
//! WebXR head-set requires a browser/graphics runtime this offline workspace
//! does not have. That binding is abstracted behind the [`SceneRenderer`] trait;
//! two pure-Rust backends are provided ([`JsonSceneRenderer`] producing a
//! verifiable draw-call manifest, and [`NullSceneRenderer`] for headless stats)
//! so the whole pipeline is exercisable, and a real GPU/WebXR backend can be
//! added later as another implementation without touching callers.
//!
//! # Example
//!
//! ```
//! use legalis_core::{Statute, Effect, EffectType};
//! use legalis_diff::diff;
//! use legalis_diff::immersive::{scene_from_diff, apply_layout, LayoutAlgorithm, LayoutParams};
//!
//! let old = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
//! let mut new = old.clone();
//! new.effect = Effect::new(EffectType::Revoke, "Revoked");
//! let d = diff(&old, &new).unwrap();
//!
//! let mut scene = scene_from_diff(&d);
//! apply_layout(&mut scene, LayoutAlgorithm::ForceDirected, &LayoutParams::default()).unwrap();
//! assert!(scene.node_count() >= 2);
//! ```

use crate::{ChangeType, DiffError, DiffResult, Severity};
use serde::{Deserialize, Serialize};

pub mod collab;
pub mod layout;
pub mod navigation;
pub mod plugin;
pub mod scene;
pub mod xr;

pub use collab::{
    CameraState, ParticipantRole, PresenceCursor, SceneAnnotation, VizEvent, VizParticipant,
    VizSession, VizSnapshot,
};
pub use layout::{LayoutAlgorithm, LayoutParams, apply_layout};
pub use navigation::{NavStep, SceneNavigator};
pub use plugin::{AsciiScatterPlugin, VisualizationPlugin, VizPluginRegistry, WireframeJsonPlugin};
pub use scene::{
    EdgeKind, NodeKind, Scene3d, SceneEdge, SceneNode, scene_from_diff, scene_from_diffs,
};
pub use xr::{XrFormat, export_scene};

/// A point or vector in 3-D space (right-handed, Y-up).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Vec3 {
    /// X component.
    pub x: f64,
    /// Y component.
    pub y: f64,
    /// Z component.
    pub z: f64,
}

impl Vec3 {
    /// Creates a vector from its components.
    #[must_use]
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// The zero vector / origin.
    #[must_use]
    pub const fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    /// Multiplies every component by `factor`.
    #[must_use]
    pub fn scale(self, factor: f64) -> Self {
        Self::new(self.x * factor, self.y * factor, self.z * factor)
    }

    /// Dot product.
    #[must_use]
    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Squared Euclidean length (cheaper than [`Vec3::length`]).
    #[must_use]
    pub fn length_squared(self) -> f64 {
        self.dot(self)
    }

    /// Euclidean length (magnitude).
    #[must_use]
    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    /// Euclidean distance to `other`.
    #[must_use]
    pub fn distance(self, other: Self) -> f64 {
        (self - other).length()
    }

    /// Returns a unit-length copy.
    ///
    /// For a (near) zero-length vector this returns the original vector
    /// unchanged rather than producing `NaN`s, keeping all downstream math
    /// finite.
    #[must_use]
    pub fn normalized(self) -> Self {
        let len = self.length();
        if len <= f64::EPSILON {
            self
        } else {
            self.scale(1.0 / len)
        }
    }

    /// Linear interpolation: `t = 0` yields `self`, `t = 1` yields `other`.
    #[must_use]
    pub fn lerp(self, other: Self, t: f64) -> Self {
        self + (other - self).scale(t)
    }

    /// The midpoint between two points.
    #[must_use]
    pub fn midpoint(self, other: Self) -> Self {
        self.lerp(other, 0.5)
    }

    /// Returns `true` if every component is finite (no `NaN`/`inf`).
    #[must_use]
    pub fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self::zero()
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl std::ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, factor: f64) -> Self {
        self.scale(factor)
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

/// An sRGB colour with 8-bit channels plus alpha.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Color {
    /// Red channel.
    pub r: u8,
    /// Green channel.
    pub g: u8,
    /// Blue channel.
    pub b: u8,
    /// Alpha channel (255 = opaque).
    pub a: u8,
}

impl Color {
    /// Creates an opaque colour.
    #[must_use]
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    /// Creates a colour with explicit alpha.
    #[must_use]
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Renders as a `#rrggbb` hex string (alpha omitted).
    #[must_use]
    pub fn to_hex(self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }

    /// Renders as a `#rrggbbaa` hex string (alpha included).
    #[must_use]
    pub fn to_hex_rgba(self) -> String {
        format!("#{:02x}{:02x}{:02x}{:02x}", self.r, self.g, self.b, self.a)
    }

    /// The canonical colour for a [`ChangeType`].
    #[must_use]
    pub fn for_change_type(change_type: ChangeType) -> Self {
        match change_type {
            ChangeType::Added => Self::rgb(0x28, 0xa7, 0x45),
            ChangeType::Removed => Self::rgb(0xdc, 0x35, 0x45),
            ChangeType::Modified => Self::rgb(0xff, 0xc1, 0x07),
            ChangeType::Reordered => Self::rgb(0x17, 0xa2, 0xb8),
        }
    }

    /// The canonical colour for a [`Severity`]; warmer hues for higher severity.
    #[must_use]
    pub fn for_severity(severity: Severity) -> Self {
        match severity {
            Severity::None => Self::rgb(0x6c, 0x75, 0x7d),
            Severity::Minor => Self::rgb(0x17, 0xa2, 0xb8),
            Severity::Moderate => Self::rgb(0xff, 0xc1, 0x07),
            Severity::Major => Self::rgb(0xfd, 0x7e, 0x14),
            Severity::Breaking => Self::rgb(0xdc, 0x35, 0x45),
        }
    }
}

/// A pin-hole camera looking from [`Camera::position`] toward
/// [`Camera::target`].
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Camera {
    /// Eye position.
    pub position: Vec3,
    /// Point the camera is aimed at.
    pub target: Vec3,
    /// World up direction.
    pub up: Vec3,
    /// Vertical field of view, in degrees.
    pub fov_degrees: f64,
}

impl Camera {
    /// Creates a camera at `position` looking at `target` with a default up
    /// vector (`+Y`) and a 60° field of view.
    #[must_use]
    pub fn looking_at(position: Vec3, target: Vec3) -> Self {
        Self {
            position,
            target,
            up: Vec3::new(0.0, 1.0, 0.0),
            fov_degrees: 60.0,
        }
    }

    /// The (normalised) forward direction the camera is facing.
    #[must_use]
    pub fn forward(&self) -> Vec3 {
        (self.target - self.position).normalized()
    }

    /// Frames `bounds` so the whole bounding box is comfortably in view,
    /// pulling the camera back along `+Z` proportional to the box size.
    #[must_use]
    pub fn framing(bounds: &BoundingBox) -> Self {
        let center = bounds.center();
        let radius = bounds.diagonal() * 0.5;
        let distance = (radius * 2.5).max(2.0);
        let position = center + Vec3::new(0.0, radius * 0.4, distance);
        Self::looking_at(position, center)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::looking_at(Vec3::new(0.0, 0.0, 8.0), Vec3::zero())
    }
}

/// A render surface measured in pixels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Viewport {
    /// Width in pixels.
    pub width: u32,
    /// Height in pixels.
    pub height: u32,
}

impl Viewport {
    /// Creates a viewport, clamping each dimension to a minimum of 1 pixel so
    /// the aspect ratio is always finite.
    #[must_use]
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width: width.max(1),
            height: height.max(1),
        }
    }

    /// Width divided by height.
    #[must_use]
    pub fn aspect_ratio(&self) -> f64 {
        f64::from(self.width) / f64::from(self.height)
    }
}

impl Default for Viewport {
    fn default() -> Self {
        Self::new(1280, 720)
    }
}

/// An axis-aligned bounding box.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct BoundingBox {
    /// Minimum corner.
    pub min: Vec3,
    /// Maximum corner.
    pub max: Vec3,
}

impl BoundingBox {
    /// An empty box whose `min`/`max` are deliberately inverted so the first
    /// [`BoundingBox::expand`] call snaps it onto a real point.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            min: Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY),
            max: Vec3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY),
        }
    }

    /// Returns `true` if the box contains no points (never expanded).
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.min.x > self.max.x || self.min.y > self.max.y || self.min.z > self.max.z
    }

    /// Grows the box to include `point`.
    pub fn expand(&mut self, point: Vec3) {
        self.min.x = self.min.x.min(point.x);
        self.min.y = self.min.y.min(point.y);
        self.min.z = self.min.z.min(point.z);
        self.max.x = self.max.x.max(point.x);
        self.max.y = self.max.y.max(point.y);
        self.max.z = self.max.z.max(point.z);
    }

    /// Builds the tightest box enclosing all `points`.
    #[must_use]
    pub fn from_points<I: IntoIterator<Item = Vec3>>(points: I) -> Self {
        let mut bb = Self::empty();
        for p in points {
            bb.expand(p);
        }
        bb
    }

    /// The geometric centre (origin if the box is empty).
    #[must_use]
    pub fn center(&self) -> Vec3 {
        if self.is_empty() {
            Vec3::zero()
        } else {
            self.min.midpoint(self.max)
        }
    }

    /// The per-axis size; zero for an empty box.
    #[must_use]
    pub fn size(&self) -> Vec3 {
        if self.is_empty() {
            Vec3::zero()
        } else {
            self.max - self.min
        }
    }

    /// Length of the box's space diagonal.
    #[must_use]
    pub fn diagonal(&self) -> f64 {
        self.size().length()
    }
}

impl Default for BoundingBox {
    fn default() -> Self {
        Self::empty()
    }
}

/// A single thing to draw: one node projected into the render queue.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DrawCall {
    /// Id of the scene node this call draws.
    pub node_id: String,
    /// World-space position.
    pub position: Vec3,
    /// Visual radius / size.
    pub size: f64,
    /// Fill colour.
    pub color: Color,
    /// Human-readable label.
    pub label: String,
    /// Distance from the camera at render time (used for depth sorting).
    pub depth: f64,
}

/// The output of a [`SceneRenderer`]: an ordered list of draw calls plus
/// summary statistics. This is the "draw list" a GPU/WebXR backend would
/// consume; producing it is the pure-Rust part of rendering, while rasterising
/// it is the deferred external binding.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RenderManifest {
    /// Name of the backend that produced the manifest.
    pub backend: String,
    /// Number of nodes in the source scene.
    pub node_count: usize,
    /// Number of edges in the source scene.
    pub edge_count: usize,
    /// Camera used for the render.
    pub camera: Camera,
    /// Viewport used for the render.
    pub viewport: Viewport,
    /// Ordered draw calls (back-to-front by depth).
    pub draw_calls: Vec<DrawCall>,
}

impl RenderManifest {
    /// Serialises the manifest to pretty JSON.
    ///
    /// # Errors
    ///
    /// Returns [`DiffError::SerializationError`] if serialisation fails.
    pub fn to_json(&self) -> DiffResult<String> {
        serde_json::to_string_pretty(self).map_err(|e| DiffError::SerializationError(e.to_string()))
    }
}

/// Abstraction over a backend that turns a [`Scene3d`] into a [`RenderManifest`]
/// for a given [`Camera`]/[`Viewport`].
///
/// Pure-Rust backends ([`JsonSceneRenderer`], [`NullSceneRenderer`]) build the
/// manifest locally. A production deployment can add a `WebGlSceneRenderer`
/// that rasterises the same manifest on the GPU without changing callers — that
/// networked/graphics binding is intentionally deferred.
pub trait SceneRenderer {
    /// Produces a render manifest for `scene`.
    ///
    /// # Errors
    ///
    /// Returns a [`DiffError`] if the scene cannot be rendered.
    fn render(
        &mut self,
        scene: &Scene3d,
        camera: &Camera,
        viewport: &Viewport,
    ) -> DiffResult<RenderManifest>;

    /// Human-readable backend label.
    fn backend_name(&self) -> &str;
}

/// A pure-Rust [`SceneRenderer`] that emits a full draw-call manifest.
///
/// Draw calls are sorted **back-to-front** by distance from the camera (the
/// painter's algorithm) so a downstream rasteriser can blend transparency
/// correctly.
#[derive(Debug, Clone, Default)]
pub struct JsonSceneRenderer;

impl JsonSceneRenderer {
    /// Creates a new renderer.
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl SceneRenderer for JsonSceneRenderer {
    fn render(
        &mut self,
        scene: &Scene3d,
        camera: &Camera,
        viewport: &Viewport,
    ) -> DiffResult<RenderManifest> {
        if scene.is_empty() {
            return Err(DiffError::Visualization(
                "cannot render an empty scene".to_string(),
            ));
        }
        let mut draw_calls: Vec<DrawCall> = scene
            .nodes()
            .iter()
            .map(|node| DrawCall {
                node_id: node.id.clone(),
                position: node.position,
                size: node.size,
                color: node.color,
                label: node.label.clone(),
                depth: node.position.distance(camera.position),
            })
            .collect();
        // Painter's algorithm: farthest first.
        draw_calls.sort_by(|a, b| {
            b.depth
                .partial_cmp(&a.depth)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.node_id.cmp(&b.node_id))
        });
        Ok(RenderManifest {
            backend: self.backend_name().to_string(),
            node_count: scene.node_count(),
            edge_count: scene.edge_count(),
            camera: *camera,
            viewport: *viewport,
            draw_calls,
        })
    }

    fn backend_name(&self) -> &str {
        "json"
    }
}

/// A headless [`SceneRenderer`] that validates the scene and reports counts but
/// emits no geometry. Useful for benchmarking the pipeline or smoke-testing
/// inputs without building a draw list.
#[derive(Debug, Clone, Default)]
pub struct NullSceneRenderer;

impl NullSceneRenderer {
    /// Creates a new headless renderer.
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl SceneRenderer for NullSceneRenderer {
    fn render(
        &mut self,
        scene: &Scene3d,
        camera: &Camera,
        viewport: &Viewport,
    ) -> DiffResult<RenderManifest> {
        if scene.is_empty() {
            return Err(DiffError::Visualization(
                "cannot render an empty scene".to_string(),
            ));
        }
        Ok(RenderManifest {
            backend: self.backend_name().to_string(),
            node_count: scene.node_count(),
            edge_count: scene.edge_count(),
            camera: *camera,
            viewport: *viewport,
            draw_calls: Vec::new(),
        })
    }

    fn backend_name(&self) -> &str {
        "null"
    }
}

/// Deterministic 64-bit FNV-1a hash of a string — the pure-Rust source of
/// reproducible pseudo-randomness used to seed layouts (no `rand` dependency,
/// honouring the workspace SciRS2 policy).
pub(crate) fn fnv1a(text: &str) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in text.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

/// Maps a `u64` hash to a deterministic `f64` in the half-open range
/// `[-1.0, 1.0)`.
pub(crate) fn unit_signed(hash: u64) -> f64 {
    // Use 53 bits for a uniform mantissa, then map [0,1) -> [-1,1).
    let unit = (hash >> 11) as f64 / (1u64 << 53) as f64;
    unit * 2.0 - 1.0
}

/// Derives a deterministic seed position for a node id inside a cube of the
/// given half-extent.
pub(crate) fn seed_position(id: &str, half_extent: f64) -> Vec3 {
    let hx = fnv1a(id);
    let hy = fnv1a(&format!("{id}#y"));
    let hz = fnv1a(&format!("{id}#z"));
    Vec3::new(
        unit_signed(hx) * half_extent,
        unit_signed(hy) * half_extent,
        unit_signed(hz) * half_extent,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-9
    }

    #[test]
    fn test_vec3_arithmetic() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(a + b, Vec3::new(5.0, 7.0, 9.0));
        assert_eq!(b - a, Vec3::new(3.0, 3.0, 3.0));
        assert_eq!(a.scale(2.0), Vec3::new(2.0, 4.0, 6.0));
        assert_eq!(a * 2.0, Vec3::new(2.0, 4.0, 6.0));
        assert_eq!(-a, Vec3::new(-1.0, -2.0, -3.0));
        assert!(approx(a.dot(b), 32.0));
    }

    #[test]
    fn test_vec3_length_and_distance() {
        let v = Vec3::new(3.0, 4.0, 0.0);
        assert!(approx(v.length(), 5.0));
        assert!(approx(v.length_squared(), 25.0));
        assert!(approx(Vec3::zero().distance(v), 5.0));
    }

    #[test]
    fn test_vec3_normalized_guards_zero() {
        let z = Vec3::zero().normalized();
        assert!(z.is_finite());
        let n = Vec3::new(0.0, 5.0, 0.0).normalized();
        assert!(approx(n.length(), 1.0));
    }

    #[test]
    fn test_vec3_lerp_and_midpoint() {
        let a = Vec3::zero();
        let b = Vec3::new(10.0, 0.0, 0.0);
        assert_eq!(a.lerp(b, 0.5), Vec3::new(5.0, 0.0, 0.0));
        assert_eq!(a.midpoint(b), Vec3::new(5.0, 0.0, 0.0));
    }

    #[test]
    fn test_color_hex_and_mappings() {
        assert_eq!(Color::rgb(0x28, 0xa7, 0x45).to_hex(), "#28a745");
        assert_eq!(Color::rgba(1, 2, 3, 4).to_hex_rgba(), "#01020304");
        assert_eq!(
            Color::for_change_type(ChangeType::Added),
            Color::rgb(0x28, 0xa7, 0x45)
        );
        assert_ne!(
            Color::for_severity(Severity::Breaking),
            Color::for_severity(Severity::None)
        );
    }

    #[test]
    fn test_bounding_box_expand_and_center() {
        let bb = BoundingBox::from_points([
            Vec3::new(-1.0, -2.0, -3.0),
            Vec3::new(1.0, 2.0, 3.0),
            Vec3::zero(),
        ]);
        assert_eq!(bb.center(), Vec3::zero());
        assert_eq!(bb.size(), Vec3::new(2.0, 4.0, 6.0));
        assert!(!bb.is_empty());
    }

    #[test]
    fn test_empty_bounding_box() {
        let bb = BoundingBox::empty();
        assert!(bb.is_empty());
        assert_eq!(bb.center(), Vec3::zero());
        assert_eq!(bb.size(), Vec3::zero());
    }

    #[test]
    fn test_camera_framing_pulls_back() {
        let bb = BoundingBox::from_points([Vec3::new(-5.0, -5.0, -5.0), Vec3::new(5.0, 5.0, 5.0)]);
        let cam = Camera::framing(&bb);
        assert!(cam.position.distance(cam.target) > 2.0);
        assert!(cam.forward().is_finite());
    }

    #[test]
    fn test_viewport_aspect_ratio_and_clamp() {
        let vp = Viewport::new(0, 0);
        assert_eq!(vp.width, 1);
        assert_eq!(vp.height, 1);
        let wide = Viewport::new(1600, 800);
        assert!(approx(wide.aspect_ratio(), 2.0));
    }

    #[test]
    fn test_seed_position_is_deterministic_and_bounded() {
        let a = seed_position("node-1", 10.0);
        let b = seed_position("node-1", 10.0);
        let c = seed_position("node-2", 10.0);
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert!(a.x.abs() <= 10.0 && a.y.abs() <= 10.0 && a.z.abs() <= 10.0);
    }

    #[test]
    fn test_fnv1a_distinct() {
        assert_ne!(fnv1a("alpha"), fnv1a("beta"));
        assert_eq!(fnv1a("same"), fnv1a("same"));
    }
}
