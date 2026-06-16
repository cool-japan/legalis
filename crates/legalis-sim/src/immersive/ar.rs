//! Augmented-reality *policy overlays*.
//!
//! An AR policy overlay pins simulation results to the real world: each statute
//! (or policy) becomes a world-anchored, colour-graded info card whose intensity
//! and hue encode its impact (e.g. how often it triggers judicial discretion).
//! Overlays are laid out deterministically in a ring around the viewer and can be
//! exported as a JSON anchor manifest or an AR.js / WebXR A-Frame document.
//!
//! As with the rest of [`super`], building the overlay document is the pure-Rust
//! part done here; presenting it through a phone/head-set AR runtime is the
//! deferred external binding.

use super::xr::xml_escape;
use super::{Color, Vec3};
use crate::SimResult;
use crate::metrics::{SimulationMetrics, StatuteMetrics};
use serde::{Deserialize, Serialize};
use std::fmt::Write as _;

/// How an [`ArAnchor`] is registered to the physical world.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArTrackingMode {
    /// A free world-space anchor (WebXR hit-test / SLAM).
    WorldAnchor,
    /// Anchored to a recognised fiducial image marker.
    ImageMarker,
    /// Anchored to a detected horizontal/vertical plane.
    PlaneDetection,
    /// Anchored to a geographic (lat/long) location.
    GeoAnchor,
}

impl ArTrackingMode {
    /// A short, stable string tag.
    #[must_use]
    pub fn tag(&self) -> &'static str {
        match self {
            ArTrackingMode::WorldAnchor => "world",
            ArTrackingMode::ImageMarker => "marker",
            ArTrackingMode::PlaneDetection => "plane",
            ArTrackingMode::GeoAnchor => "geo",
        }
    }
}

/// A world-registered anchor an overlay is attached to.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArAnchor {
    /// Stable anchor id.
    pub id: String,
    /// How the anchor is tracked.
    pub mode: ArTrackingMode,
    /// World-space position (metres), relative to the viewer's session origin.
    pub position: Vec3,
    /// Yaw the overlay faces, in degrees (0 = facing `-Z`).
    pub yaw_degrees: f64,
    /// Optional marker / geo reference (e.g. marker preset name or `lat,long`).
    pub reference: Option<String>,
}

impl ArAnchor {
    /// Creates a world anchor at `position`.
    #[must_use]
    pub fn world(id: impl Into<String>, position: Vec3) -> Self {
        Self {
            id: id.into(),
            mode: ArTrackingMode::WorldAnchor,
            position,
            yaw_degrees: 0.0,
            reference: None,
        }
    }

    /// Builder: sets the tracking mode.
    #[must_use]
    pub fn with_mode(mut self, mode: ArTrackingMode) -> Self {
        self.mode = mode;
        self
    }

    /// Builder: sets the facing yaw, in degrees.
    #[must_use]
    pub fn with_yaw(mut self, yaw_degrees: f64) -> Self {
        self.yaw_degrees = yaw_degrees;
        self
    }

    /// Builder: sets the marker / geo reference.
    #[must_use]
    pub fn with_reference(mut self, reference: impl Into<String>) -> Self {
        self.reference = Some(reference.into());
        self
    }
}

/// The visual form of a [`PolicyOverlay`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OverlayShape {
    /// A flat, text-bearing card facing the viewer.
    Card,
    /// A billboarded label that always faces the camera.
    Billboard,
    /// A vertical beacon column whose height scales with impact.
    Beacon,
    /// A ring whose radius scales with impact.
    Ring,
}

impl OverlayShape {
    /// A short, stable string tag.
    #[must_use]
    pub fn tag(&self) -> &'static str {
        match self {
            OverlayShape::Card => "card",
            OverlayShape::Billboard => "billboard",
            OverlayShape::Beacon => "beacon",
            OverlayShape::Ring => "ring",
        }
    }
}

/// A qualitative severity band derived from a `[0, 1]` intensity.
#[must_use]
pub fn severity_label(intensity: f64) -> &'static str {
    match intensity {
        t if t < 0.25 => "Low",
        t if t < 0.5 => "Moderate",
        t if t < 0.75 => "High",
        _ => "Critical",
    }
}

/// A single AR overlay: an info card anchored in the world, graded by impact.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PolicyOverlay {
    /// Stable overlay id.
    pub id: String,
    /// The anchor this overlay is attached to.
    pub anchor: ArAnchor,
    /// Headline (e.g. statute id).
    pub title: String,
    /// A representative scalar value (e.g. application volume).
    pub value: f64,
    /// Normalised impact intensity in `[0, 1]` driving colour / size.
    pub intensity: f64,
    /// Colour (heat-graded from `intensity`).
    pub color: Color,
    /// Overlay shape.
    pub shape: OverlayShape,
    /// Body lines of text.
    pub lines: Vec<String>,
}

impl PolicyOverlay {
    /// Creates an overlay, clamping `intensity` to `[0, 1]` and heat-grading its
    /// colour.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        anchor: ArAnchor,
        title: impl Into<String>,
        value: f64,
        intensity: f64,
    ) -> Self {
        let intensity = intensity.clamp(0.0, 1.0);
        Self {
            id: id.into(),
            anchor,
            title: title.into(),
            value,
            intensity,
            color: Color::heat(intensity),
            shape: OverlayShape::Card,
            lines: Vec::new(),
        }
    }

    /// Builder: sets the overlay shape.
    #[must_use]
    pub fn with_shape(mut self, shape: OverlayShape) -> Self {
        self.shape = shape;
        self
    }

    /// Builder: appends a body line.
    #[must_use]
    pub fn with_line(mut self, line: impl Into<String>) -> Self {
        self.lines.push(line.into());
        self
    }

    /// The severity band for this overlay's intensity.
    #[must_use]
    pub fn severity(&self) -> &'static str {
        severity_label(self.intensity)
    }
}

/// A collection of policy overlays anchored in an AR session.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ArOverlayScene {
    overlays: Vec<PolicyOverlay>,
}

impl ArOverlayScene {
    /// Creates an empty overlay scene.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an overlay. Re-adding an id replaces it in place and returns `false`.
    pub fn add(&mut self, overlay: PolicyOverlay) -> bool {
        if let Some(existing) = self.overlays.iter_mut().find(|o| o.id == overlay.id) {
            *existing = overlay;
            false
        } else {
            self.overlays.push(overlay);
            true
        }
    }

    /// All overlays.
    #[must_use]
    pub fn overlays(&self) -> &[PolicyOverlay] {
        &self.overlays
    }

    /// Number of overlays.
    #[must_use]
    pub fn len(&self) -> usize {
        self.overlays.len()
    }

    /// Returns `true` if there are no overlays.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.overlays.is_empty()
    }

    /// Looks up an overlay by id.
    #[must_use]
    pub fn overlay(&self, id: &str) -> Option<&PolicyOverlay> {
        self.overlays.iter().find(|o| o.id == id)
    }

    /// The overlay with the highest intensity, if any (the most pressing policy).
    #[must_use]
    pub fn most_critical(&self) -> Option<&PolicyOverlay> {
        self.overlays.iter().max_by(|a, b| {
            a.intensity
                .partial_cmp(&b.intensity)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    /// Serialises the overlay scene to pretty JSON (the AR anchor manifest).
    ///
    /// # Errors
    ///
    /// Returns [`crate::SimulationError::Serialization`] if serialisation fails.
    pub fn to_json(&self) -> SimResult<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    /// Exports an AR.js / WebXR A-Frame document.
    ///
    /// Image-marker overlays are wrapped in `<a-marker>` blocks keyed by their
    /// anchor reference; all other overlays are world-locked `<a-entity>` nodes.
    #[must_use]
    pub fn to_aframe_ar(&self) -> String {
        let mut html = String::new();
        html.push_str("<!DOCTYPE html>\n<html>\n<head>\n  <meta charset=\"utf-8\">\n");
        html.push_str("  <title>Legalis AR Policy Overlay</title>\n");
        html.push_str(
            "  <script src=\"https://aframe.io/releases/1.5.0/aframe.min.js\"></script>\n",
        );
        html.push_str(
            "  <script src=\"https://cdn.jsdelivr.net/gh/AR-js-org/AR.js/aframe/build/aframe-ar.js\"></script>\n",
        );
        html.push_str("</head>\n<body style=\"margin:0;overflow:hidden\">\n");
        html.push_str("  <a-scene embedded arjs=\"sourceType: webcam;\" renderer=\"colorManagement: true\">\n");

        for overlay in &self.overlays {
            if overlay.anchor.mode == ArTrackingMode::ImageMarker {
                let preset = overlay
                    .anchor
                    .reference
                    .clone()
                    .unwrap_or_else(|| "hiro".to_string());
                let _ = writeln!(
                    html,
                    "    <a-marker type=\"pattern\" preset=\"{}\">",
                    xml_escape(&preset)
                );
                self.write_overlay_entity(&mut html, overlay, true);
                html.push_str("    </a-marker>\n");
            } else {
                self.write_overlay_entity(&mut html, overlay, false);
            }
        }

        html.push_str("    <a-entity camera></a-entity>\n");
        html.push_str("  </a-scene>\n</body>\n</html>\n");
        html
    }

    /// Writes a single overlay's geometry + label as an A-Frame entity.
    fn write_overlay_entity(&self, html: &mut String, overlay: &PolicyOverlay, marker_local: bool) {
        let pos = if marker_local {
            Vec3::new(0.0, 0.5, 0.0)
        } else {
            overlay.anchor.position
        };
        let height = 0.4 + overlay.intensity * 1.2;
        let _ = writeln!(
            html,
            "    <a-entity position=\"{:.3} {:.3} {:.3}\" rotation=\"0 {:.1} 0\" data-id=\"{}\" \
             data-severity=\"{}\">",
            pos.x,
            pos.y,
            pos.z,
            overlay.anchor.yaw_degrees,
            xml_escape(&overlay.id),
            xml_escape(overlay.severity()),
        );
        match overlay.shape {
            OverlayShape::Beacon => {
                let _ = writeln!(
                    html,
                    "      <a-cylinder radius=\"0.08\" height=\"{height:.3}\" color=\"{}\"></a-cylinder>",
                    overlay.color.to_hex()
                );
            }
            OverlayShape::Ring => {
                let _ = writeln!(
                    html,
                    "      <a-ring radius-inner=\"{:.3}\" radius-outer=\"{:.3}\" color=\"{}\"></a-ring>",
                    0.2 + overlay.intensity * 0.3,
                    0.3 + overlay.intensity * 0.4,
                    overlay.color.to_hex()
                );
            }
            OverlayShape::Card | OverlayShape::Billboard => {
                let _ = writeln!(
                    html,
                    "      <a-plane width=\"1.2\" height=\"0.7\" color=\"{}\" opacity=\"0.85\"></a-plane>",
                    overlay.color.to_hex()
                );
            }
        }
        let mut text = overlay.title.clone();
        for line in &overlay.lines {
            text.push('\n');
            text.push_str(line);
        }
        let _ = writeln!(
            html,
            "      <a-text value=\"{}\" align=\"center\" width=\"2\" position=\"0 0.05 0.01\" \
             color=\"#0d1117\"></a-text>",
            xml_escape(&text)
        );
        html.push_str("    </a-entity>\n");
    }
}

/// Builds an [`ArOverlayScene`] from aggregate [`SimulationMetrics`].
///
/// One [`PolicyOverlay`] is created per statute, laid out in a ring at eye level
/// around the viewer (deterministic ordering by statute id). Intensity is the
/// statute's ambiguity (discretion ratio); the body lines summarise the
/// deterministic / discretionary / void breakdown.
#[must_use]
pub fn overlay_from_metrics(metrics: &SimulationMetrics) -> ArOverlayScene {
    let mut scene = ArOverlayScene::new();
    let mut entries: Vec<(&String, &StatuteMetrics)> = metrics.statute_metrics.iter().collect();
    entries.sort_by(|a, b| a.0.cmp(b.0));
    let count = entries.len().max(1) as f64;
    let radius = 4.0;

    for (i, (statute_id, sm)) in entries.into_iter().enumerate() {
        let angle = std::f64::consts::TAU * (i as f64) / count;
        let position = Vec3::new(radius * angle.sin(), 1.5, -radius * angle.cos());
        let yaw = angle.to_degrees();
        let anchor = ArAnchor::world(format!("anchor::{statute_id}"), position).with_yaw(yaw);
        let intensity = sm.ambiguity();
        let overlay = PolicyOverlay::new(
            format!("overlay::{statute_id}"),
            anchor,
            statute_id.clone(),
            sm.total as f64,
            intensity,
        )
        .with_line(format!("Applications: {}", sm.total))
        .with_line(format!(
            "Deterministic: {} ({:.0}%)",
            sm.deterministic,
            sm.effectiveness() * 100.0
        ))
        .with_line(format!(
            "Discretion: {} ({:.0}%)",
            sm.discretion,
            sm.ambiguity() * 100.0
        ))
        .with_line(format!("Void: {}", sm.void));
        scene.add(overlay);
    }

    scene
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SimulationMetrics;
    use crate::engine::LawApplicationResult;
    use legalis_core::{Effect, EffectType, LegalResult};

    fn metrics() -> SimulationMetrics {
        let mut m = SimulationMetrics::new();
        for _ in 0..4 {
            m.record_result(&LawApplicationResult {
                agent_id: uuid::Uuid::new_v4(),
                statute_id: "clear-rule".to_string(),
                result: LegalResult::Deterministic(Effect::new(EffectType::Grant, "ok")),
            });
        }
        for _ in 0..3 {
            m.record_result(&LawApplicationResult {
                agent_id: uuid::Uuid::new_v4(),
                statute_id: "fuzzy-rule".to_string(),
                result: LegalResult::JudicialDiscretion {
                    issue: "x".to_string(),
                    context_id: uuid::Uuid::new_v4(),
                    narrative_hint: None,
                },
            });
        }
        m
    }

    #[test]
    fn test_overlay_from_metrics_one_per_statute() {
        let scene = overlay_from_metrics(&metrics());
        assert_eq!(scene.len(), 2);
        assert!(scene.overlay("overlay::clear-rule").is_some());
        let fuzzy = scene.overlay("overlay::fuzzy-rule").expect("fuzzy");
        // Fully discretionary → maximum intensity → Critical band.
        assert!((fuzzy.intensity - 1.0).abs() < 1e-9);
        assert_eq!(fuzzy.severity(), "Critical");
    }

    #[test]
    fn test_overlays_laid_out_in_ring_at_eye_level() {
        let scene = overlay_from_metrics(&metrics());
        for overlay in scene.overlays() {
            let p = overlay.anchor.position;
            assert!((p.y - 1.5).abs() < 1e-9);
            let radius = (p.x * p.x + p.z * p.z).sqrt();
            assert!((radius - 4.0).abs() < 1e-6);
        }
    }

    #[test]
    fn test_most_critical_picks_highest_intensity() {
        let scene = overlay_from_metrics(&metrics());
        let crit = scene.most_critical().expect("some overlay");
        assert_eq!(crit.title, "fuzzy-rule");
    }

    #[test]
    fn test_severity_label_bands() {
        assert_eq!(severity_label(0.1), "Low");
        assert_eq!(severity_label(0.3), "Moderate");
        assert_eq!(severity_label(0.6), "High");
        assert_eq!(severity_label(0.95), "Critical");
    }

    #[test]
    fn test_to_json_and_aframe_ar_export() {
        let mut scene = overlay_from_metrics(&metrics());
        // Add a marker-tracked beacon to exercise both branches.
        let anchor = ArAnchor::world("anchor::m", Vec3::new(0.0, 0.5, -2.0))
            .with_mode(ArTrackingMode::ImageMarker)
            .with_reference("hiro");
        scene.add(
            PolicyOverlay::new("overlay::m", anchor, "Marker Rule", 9.0, 0.8)
                .with_shape(OverlayShape::Beacon)
                .with_line("anchored to fiducial"),
        );
        let json = scene.to_json().expect("json");
        assert!(json.contains("\"overlay::fuzzy-rule\""));
        let restored: ArOverlayScene = serde_json::from_str(&json).expect("roundtrip");
        assert_eq!(restored.len(), scene.len());

        let html = scene.to_aframe_ar();
        assert!(html.contains("<a-scene"));
        assert!(html.contains("arjs="));
        assert!(html.contains("<a-marker"));
        assert!(html.contains("preset=\"hiro\""));
        assert!(html.contains("<a-cylinder")); // the beacon
    }

    #[test]
    fn test_add_replaces_by_id_and_clamps_intensity() {
        let mut scene = ArOverlayScene::new();
        let anchor = ArAnchor::world("a", Vec3::zero());
        assert!(scene.add(PolicyOverlay::new("o", anchor.clone(), "A", 1.0, 5.0)));
        // Intensity clamped to 1.0.
        assert!((scene.overlay("o").unwrap().intensity - 1.0).abs() < 1e-9);
        // Re-add same id replaces (returns false), no growth.
        assert!(!scene.add(PolicyOverlay::new("o", anchor, "B", 2.0, -1.0)));
        assert_eq!(scene.len(), 1);
        assert_eq!(scene.overlay("o").unwrap().title, "B");
        assert!((scene.overlay("o").unwrap().intensity - 0.0).abs() < 1e-9);
    }
}
