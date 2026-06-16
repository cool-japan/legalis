//! Immersive Simulation (v0.3.4).
//!
//! Where the crate's `visualization` module renders *2-D* artifacts (DOT graphs,
//! D3.js JSON, dashboards), this module models a running simulation as a
//! navigable **3-D scene** and layers immersive, multi-sensory features on top of
//! it. Everything here is pure Rust, deterministic and self-contained — no
//! external graphics/audio/haptics runtime is required to *build* the artifacts;
//! presenting them on a head-set or driving real actuators is the deferred
//! external binding (see "Deferred bindings" below).
//!
//! - [`scene`] — a [`SimScene`] graph (entities/statutes/clusters as 3-D nodes
//!   with typed edges) built from a simulation population or from
//!   [`crate::SimulationMetrics`].
//! - [`xr`] — exporters that serialise a scene into immersive VR/AR scene
//!   formats: A-Frame HTML (WebXR), X3D XML, and a glTF-like JSON manifest.
//! - [`ar`] — augmented-reality *policy overlays*: world-anchored info cards that
//!   map per-statute impact onto colour-graded AR anchors, exportable as an AR
//!   anchor manifest (JSON) or an AR.js marker scene.
//! - [`haptic`] — haptic-feedback encoding: force / vibration channels mapped
//!   from impact metrics into a time-lined [`haptic::HapticPattern`] a controller
//!   or wearable can play back.
//! - [`collab`] — collaborative VR exploration: a shared camera, per-participant
//!   3-D presence cursors, scene annotations and a sequence-ordered event log
//!   with deterministic last-writer-wins conflict resolution (replicas that
//!   accept the same operations converge to an identical state digest).
//! - [`audio`] — spatial-audio cues: 3-D positional audio descriptors that map
//!   data dimensions of a simulation to sound (pitch / gain / pan / timbre) with
//!   a distance-attenuation mixing model.
//!
//! # Deferred bindings
//!
//! Actually rasterising a scene on a GPU, presenting it inside a live WebXR
//! head-set, synthesising audio samples, or driving a physical haptic actuator
//! all require device/runtime access this offline workspace does not have. The
//! pure-Rust half — building standards-shaped scene/overlay documents, haptic
//! patterns and spatial-audio descriptors — lives here and is fully exercised by
//! the test-suite; the device binding is intentionally deferred and can be added
//! later as a thin adapter over these data structures without changing callers.
//!
//! # Example
//!
//! ```
//! use legalis_sim::immersive::{
//!     scene_from_metrics, export_scene, Camera, XrFormat,
//! };
//! use legalis_sim::SimulationMetrics;
//!
//! let metrics = SimulationMetrics::new();
//! let scene = scene_from_metrics(&metrics);
//! // An empty-metrics scene still has the origin node.
//! assert!(scene.node_count() >= 1);
//! let cam = Camera::framing(&scene.bounds());
//! let _html = export_scene(&scene, &cam, XrFormat::AFrame).unwrap();
//! ```

pub mod ar;
pub mod audio;
pub mod collab;
pub mod haptic;
pub mod scene;
pub mod xr;

pub use ar::{
    ArAnchor, ArOverlayScene, ArTrackingMode, OverlayShape, PolicyOverlay, overlay_from_metrics,
};
pub use audio::{AudioParam, AudioSource, SonifiedField, Sonifier, SpatialAudioScene, Waveform};
pub use collab::{
    CollabEvent, CollabParticipant, CollabSession, CollabSnapshot, ParticipantRole, PresenceCursor,
    SceneAnnotation, SharedCamera,
};
pub use haptic::{
    HapticChannel, HapticCue, HapticEncoder, HapticPattern, HapticWaveform, ImpactSignal,
};
pub use scene::{
    AttributeAxis, EdgeKind, NodeKind, PopulationMapping, SceneEdge, SceneNode, SimScene,
    scene_from_entities, scene_from_metrics,
};
pub use xr::{XrFormat, export_scene};

use serde::{Deserialize, Serialize};

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
    /// For a (near) zero-length vector this returns the original vector unchanged
    /// rather than producing `NaN`s, keeping downstream math finite.
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

    /// The three components as an array (the glTF / X3D convention).
    #[must_use]
    pub fn to_array(self) -> [f64; 3] {
        [self.x, self.y, self.z]
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

    /// The colour as space-separated floats in `[0, 1]` (the X3D convention).
    #[must_use]
    pub fn to_rgb_floats(self) -> String {
        format!(
            "{:.3} {:.3} {:.3}",
            f64::from(self.r) / 255.0,
            f64::from(self.g) / 255.0,
            f64::from(self.b) / 255.0,
        )
    }

    /// A cool→warm heat colour for a normalised value `t`.
    ///
    /// `t` is clamped to `[0, 1]`: `0.0` maps to blue, `0.5` to amber and `1.0`
    /// to red, interpolating linearly per channel. Useful for grading nodes,
    /// AR overlays and audio cues by an intensity such as discretion ratio.
    #[must_use]
    pub fn heat(t: f64) -> Self {
        let t = t.clamp(0.0, 1.0);
        let blue = Self::rgb(0x21, 0x96, 0xf3);
        let amber = Self::rgb(0xff, 0xc1, 0x07);
        let red = Self::rgb(0xf4, 0x43, 0x36);
        if t <= 0.5 {
            lerp_color(blue, amber, t * 2.0)
        } else {
            lerp_color(amber, red, (t - 0.5) * 2.0)
        }
    }
}

/// Linearly interpolates one channel.
fn lerp_u8(a: u8, b: u8, t: f64) -> u8 {
    let a = f64::from(a);
    let b = f64::from(b);
    (a + (b - a) * t).round().clamp(0.0, 255.0) as u8
}

/// Linearly interpolates two colours (alpha included).
fn lerp_color(a: Color, b: Color, t: f64) -> Color {
    Color::rgba(
        lerp_u8(a.r, b.r, t),
        lerp_u8(a.g, b.g, t),
        lerp_u8(a.b, b.b, t),
        lerp_u8(a.a, b.a, t),
    )
}

/// A pin-hole camera looking from [`Camera::position`] toward [`Camera::target`].
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

    /// Frames `bounds` so the whole bounding box is comfortably in view, pulling
    /// the camera back along `+Z` proportional to the box size.
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
        Self::looking_at(Vec3::new(0.0, 0.0, 12.0), Vec3::zero())
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
    /// Creates a viewport, clamping each dimension to a minimum of 1 pixel so the
    /// aspect ratio is always finite.
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

/// Deterministic 64-bit FNV-1a hash of a byte slice — the pure-Rust source of
/// reproducible pseudo-randomness used to seed layouts and of state digests for
/// collaborative sessions (no `rand`/`sha2` dependency required).
#[must_use]
pub fn fnv1a_bytes(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

/// Deterministic 64-bit FNV-1a hash of a string.
#[must_use]
pub fn fnv1a(text: &str) -> u64 {
    fnv1a_bytes(text.as_bytes())
}

/// A 128-bit hex digest of `bytes` (two FNV-1a passes with distinct seeds),
/// giving a compact, dependency-free, collision-resistant-enough fingerprint for
/// replica state comparison.
#[must_use]
pub fn digest_hex(bytes: &[u8]) -> String {
    let lo = fnv1a_bytes(bytes);
    // Second pass over a salted copy decorrelates the two halves.
    let mut salted = Vec::with_capacity(bytes.len() + 1);
    salted.push(0x5a);
    salted.extend_from_slice(bytes);
    let hi = fnv1a_bytes(&salted);
    format!("{hi:016x}{lo:016x}")
}

/// Maps a `u64` hash to a deterministic `f64` in the half-open range `[-1, 1)`.
#[must_use]
pub fn unit_signed(hash: u64) -> f64 {
    let unit = (hash >> 11) as f64 / (1u64 << 53) as f64;
    unit * 2.0 - 1.0
}

/// Derives a deterministic seed position for an id inside a cube of the given
/// half-extent.
#[must_use]
pub fn seed_position(id: &str, half_extent: f64) -> Vec3 {
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
    fn test_vec3_arithmetic_and_geometry() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(a + b, Vec3::new(5.0, 7.0, 9.0));
        assert_eq!(b - a, Vec3::new(3.0, 3.0, 3.0));
        assert_eq!(a * 2.0, Vec3::new(2.0, 4.0, 6.0));
        assert_eq!(-a, Vec3::new(-1.0, -2.0, -3.0));
        assert!(approx(a.dot(b), 32.0));
        let v = Vec3::new(3.0, 4.0, 0.0);
        assert!(approx(v.length(), 5.0));
        assert!(approx(Vec3::zero().distance(v), 5.0));
        assert_eq!(a.midpoint(b), Vec3::new(2.5, 3.5, 4.5));
        assert_eq!(a.to_array(), [1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_vec3_normalized_guards_zero() {
        let z = Vec3::zero().normalized();
        assert!(z.is_finite());
        let n = Vec3::new(0.0, 5.0, 0.0).normalized();
        assert!(approx(n.length(), 1.0));
    }

    #[test]
    fn test_color_hex_and_heat_gradient() {
        assert_eq!(Color::rgb(0x21, 0x96, 0xf3).to_hex(), "#2196f3");
        assert_eq!(Color::rgba(1, 2, 3, 4).to_hex_rgba(), "#01020304");
        // Heat endpoints and monotone redward shift.
        assert_eq!(Color::heat(0.0), Color::rgb(0x21, 0x96, 0xf3));
        assert_eq!(Color::heat(1.0), Color::rgb(0xf4, 0x43, 0x36));
        assert!(Color::heat(1.0).r > Color::heat(0.0).r);
        // Clamped outside [0,1].
        assert_eq!(Color::heat(-3.0), Color::heat(0.0));
        assert_eq!(Color::heat(9.0), Color::heat(1.0));
    }

    #[test]
    fn test_bounding_box_and_camera_framing() {
        let bb = BoundingBox::from_points([
            Vec3::new(-5.0, -5.0, -5.0),
            Vec3::new(5.0, 5.0, 5.0),
            Vec3::zero(),
        ]);
        assert!(!bb.is_empty());
        assert_eq!(bb.center(), Vec3::zero());
        assert_eq!(bb.size(), Vec3::new(10.0, 10.0, 10.0));
        let cam = Camera::framing(&bb);
        assert!(cam.position.distance(cam.target) > 2.0);
        assert!(cam.forward().is_finite());
        // Empty box collapses to origin.
        let e = BoundingBox::empty();
        assert!(e.is_empty());
        assert_eq!(e.center(), Vec3::zero());
        assert_eq!(e.size(), Vec3::zero());
    }

    #[test]
    fn test_viewport_clamps_and_aspect() {
        let vp = Viewport::new(0, 0);
        assert_eq!((vp.width, vp.height), (1, 1));
        assert!(approx(Viewport::new(1600, 800).aspect_ratio(), 2.0));
    }

    #[test]
    fn test_hashing_is_deterministic_and_seed_bounded() {
        assert_eq!(fnv1a("alpha"), fnv1a("alpha"));
        assert_ne!(fnv1a("alpha"), fnv1a("beta"));
        let a = seed_position("node-1", 10.0);
        assert_eq!(a, seed_position("node-1", 10.0));
        assert_ne!(a, seed_position("node-2", 10.0));
        assert!(a.x.abs() <= 10.0 && a.y.abs() <= 10.0 && a.z.abs() <= 10.0);
        // digest stable and 32 hex chars.
        let d = digest_hex(b"hello");
        assert_eq!(d, digest_hex(b"hello"));
        assert_ne!(d, digest_hex(b"world"));
        assert_eq!(d.len(), 32);
        assert!(d.chars().all(|c| c.is_ascii_hexdigit()));
    }
}
