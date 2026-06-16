//! Spatial-audio cues for multi-dimensional data.
//!
//! This module *sonifies* a simulation: it maps data dimensions to 3-D
//! positional [`AudioSource`] descriptors — pitch, gain, timbre and world
//! position — and mixes them at a listener with a distance-attenuation /
//! stereo-pan model. An analyst can then *hear* the structure of a population or
//! a metrics aggregate (e.g. ambiguous statutes buzzing high and to the right).
//!
//! Producing the descriptors and the mix is the pure-Rust part done here;
//! synthesising actual audio samples through a device is the deferred external
//! binding (see [module overview](super)).

use super::Vec3;
use super::scene::AttributeAxis;
use crate::SimResult;
use crate::metrics::{SimulationMetrics, StatuteMetrics};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// The timbre (oscillator shape) of an [`AudioSource`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Waveform {
    /// Pure sine tone.
    Sine,
    /// Bright square wave.
    Square,
    /// Buzzy sawtooth.
    Sawtooth,
    /// Mellow triangle.
    Triangle,
    /// Filtered noise (for unstructured / failure cues).
    Noise,
}

impl Waveform {
    /// A short, stable string tag.
    #[must_use]
    pub fn tag(&self) -> &'static str {
        match self {
            Waveform::Sine => "sine",
            Waveform::Square => "square",
            Waveform::Sawtooth => "sawtooth",
            Waveform::Triangle => "triangle",
            Waveform::Noise => "noise",
        }
    }
}

/// A 3-D positional audio source.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioSource {
    /// Stable source id.
    pub id: String,
    /// World-space position.
    pub position: Vec3,
    /// Fundamental frequency, in Hz.
    pub pitch_hz: f64,
    /// Source gain (loudness) in `[0, 1]` before spatial attenuation.
    pub gain: f64,
    /// Timbre.
    pub waveform: Waveform,
    /// Human-readable label.
    pub label: String,
}

impl AudioSource {
    /// Creates a source, clamping `gain` to `[0, 1]` and `pitch` to `≥ 0`.
    #[must_use]
    pub fn new(id: impl Into<String>, pitch_hz: f64, gain: f64) -> Self {
        Self {
            id: id.into(),
            position: Vec3::zero(),
            pitch_hz: pitch_hz.max(0.0),
            gain: gain.clamp(0.0, 1.0),
            waveform: Waveform::Sine,
            label: String::new(),
        }
    }

    /// Builder: sets the position.
    #[must_use]
    pub fn at(mut self, position: Vec3) -> Self {
        self.position = position;
        self
    }

    /// Builder: sets the waveform.
    #[must_use]
    pub fn with_waveform(mut self, waveform: Waveform) -> Self {
        self.waveform = waveform;
        self
    }

    /// Builder: sets the label.
    #[must_use]
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }
}

/// Which audio parameter a data dimension drives.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AudioParam {
    /// Maps to pitch (log-interpolated across the sonifier's pitch band).
    Pitch,
    /// Maps to source gain.
    Gain,
    /// Maps to the world X coordinate (left/right).
    PositionX,
    /// Maps to the world Y coordinate (down/up).
    PositionY,
    /// Maps to the world Z coordinate (front/back).
    PositionZ,
}

/// One mapping: a named data dimension drives an [`AudioParam`], normalised by an
/// [`AttributeAxis`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SonifiedField {
    /// The data dimension name.
    pub dimension: String,
    /// The audio parameter it drives.
    pub param: AudioParam,
    /// Normalisation of the dimension's value to `[0, 1]`.
    pub axis: AttributeAxis,
}

impl SonifiedField {
    /// Creates a field mapping.
    #[must_use]
    pub fn new(dimension: impl Into<String>, param: AudioParam, axis: AttributeAxis) -> Self {
        Self {
            dimension: dimension.into(),
            param,
            axis,
        }
    }
}

/// Maps multi-dimensional data records to [`AudioSource`]s.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sonifier {
    /// The field mappings applied to every record.
    pub fields: Vec<SonifiedField>,
    /// Lowest pitch (normalised value 0), in Hz.
    pub min_pitch_hz: f64,
    /// Highest pitch (normalised value 1), in Hz.
    pub max_pitch_hz: f64,
    /// World half-extent that position parameters map into.
    pub position_extent: f64,
    /// Default timbre for generated sources.
    pub default_waveform: Waveform,
}

impl Default for Sonifier {
    fn default() -> Self {
        Self {
            fields: Vec::new(),
            min_pitch_hz: 110.0,  // A2
            max_pitch_hz: 1760.0, // A6
            position_extent: 8.0,
            default_waveform: Waveform::Sine,
        }
    }
}

impl Sonifier {
    /// Creates a sonifier with no field mappings.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Builder: adds a field mapping.
    #[must_use]
    pub fn with_field(mut self, field: SonifiedField) -> Self {
        self.fields.push(field);
        self
    }

    /// Log-interpolates a normalised value `t` across the pitch band.
    #[must_use]
    pub fn pitch_for(&self, t: f64) -> f64 {
        let t = t.clamp(0.0, 1.0);
        if self.min_pitch_hz > 0.0 && self.max_pitch_hz > 0.0 {
            self.min_pitch_hz * (self.max_pitch_hz / self.min_pitch_hz).powf(t)
        } else {
            self.min_pitch_hz + (self.max_pitch_hz - self.min_pitch_hz) * t
        }
    }

    /// Sonifies one named data record into an [`AudioSource`].
    ///
    /// Dimensions absent from the record leave their target parameter at its
    /// default (mid-band pitch, half gain, centred position).
    #[must_use]
    pub fn sonify(&self, id: impl Into<String>, record: &BTreeMap<String, f64>) -> AudioSource {
        let mut pitch = self.pitch_for(0.5);
        let mut gain = 0.5;
        let mut position = Vec3::zero();

        for field in &self.fields {
            let value = match record.get(&field.dimension) {
                Some(v) => *v,
                None => continue,
            };
            let t = field.axis.normalize(value);
            match field.param {
                AudioParam::Pitch => pitch = self.pitch_for(t),
                AudioParam::Gain => gain = t,
                AudioParam::PositionX => position.x = (t - 0.5) * 2.0 * self.position_extent,
                AudioParam::PositionY => position.y = (t - 0.5) * 2.0 * self.position_extent,
                AudioParam::PositionZ => position.z = (t - 0.5) * 2.0 * self.position_extent,
            }
        }

        AudioSource::new(id, pitch, gain)
            .at(position)
            .with_waveform(self.default_waveform)
    }

    /// A sonifier preset for [`SimulationMetrics`]: pitch ← effectiveness, gain ←
    /// application volume, pan (X) ← effectiveness, depth (Z) ← ambiguity.
    #[must_use]
    pub fn for_metrics() -> Self {
        // A guaranteed-valid `[0, 1]` axis (constructed directly to avoid a
        // fallible call on a statically-valid range).
        let unit = || AttributeAxis {
            attribute: "_".to_string(),
            min: 0.0,
            max: 1.0,
        };
        Self::new()
            .with_field(SonifiedField::new(
                "effectiveness",
                AudioParam::Pitch,
                unit(),
            ))
            .with_field(SonifiedField::new("volume", AudioParam::Gain, unit()))
            .with_field(SonifiedField::new(
                "effectiveness",
                AudioParam::PositionX,
                unit(),
            ))
            .with_field(SonifiedField::new(
                "ambiguity",
                AudioParam::PositionZ,
                unit(),
            ))
    }
}

/// A spatial scene: a listener plus positional sources with a mixing model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpatialAudioScene {
    /// Listener position.
    pub listener: Vec3,
    /// Listener forward direction.
    pub forward: Vec3,
    /// Listener up direction.
    pub up: Vec3,
    /// Reference distance within which there is no attenuation.
    pub reference_distance: f64,
    /// Distance beyond which a source is inaudible.
    pub max_distance: f64,
    sources: Vec<AudioSource>,
}

impl Default for SpatialAudioScene {
    fn default() -> Self {
        Self {
            listener: Vec3::zero(),
            forward: Vec3::new(0.0, 0.0, -1.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            reference_distance: 1.0,
            max_distance: 50.0,
            sources: Vec::new(),
        }
    }
}

impl SpatialAudioScene {
    /// Creates a scene with the listener at the origin facing `-Z`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Builder: places the listener.
    #[must_use]
    pub fn with_listener(mut self, position: Vec3) -> Self {
        self.listener = position;
        self
    }

    /// Adds a source.
    pub fn add_source(&mut self, source: AudioSource) {
        self.sources.push(source);
    }

    /// All sources.
    #[must_use]
    pub fn sources(&self) -> &[AudioSource] {
        &self.sources
    }

    /// Number of sources.
    #[must_use]
    pub fn len(&self) -> usize {
        self.sources.len()
    }

    /// Returns `true` if there are no sources.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.sources.is_empty()
    }

    /// The listener's right-hand vector (`forward × up`, normalised).
    #[must_use]
    pub fn right(&self) -> Vec3 {
        cross(self.forward, self.up).normalized()
    }

    /// The gain of `source` perceived at the listener after inverse-distance
    /// attenuation. Sources beyond [`SpatialAudioScene::max_distance`] are silent.
    #[must_use]
    pub fn perceived_gain(&self, source: &AudioSource) -> f64 {
        let distance = self.listener.distance(source.position);
        if distance >= self.max_distance {
            return 0.0;
        }
        let reference = self.reference_distance.max(f64::EPSILON);
        let attenuation = reference / distance.max(reference);
        (source.gain * attenuation).clamp(0.0, 1.0)
    }

    /// The stereo pan of `source` in `[-1, 1]` (`-1` full-left, `+1` full-right),
    /// projecting the listener→source direction onto the listener's right vector.
    #[must_use]
    pub fn pan(&self, source: &AudioSource) -> f64 {
        let direction = (source.position - self.listener).normalized();
        direction.dot(self.right()).clamp(-1.0, 1.0)
    }

    /// The sum of perceived gains across all sources (the overall mix level).
    #[must_use]
    pub fn mix_level(&self) -> f64 {
        self.sources.iter().map(|s| self.perceived_gain(s)).sum()
    }

    /// Serialises the scene to pretty JSON.
    ///
    /// # Errors
    ///
    /// Returns [`crate::SimulationError::Serialization`] if serialisation fails.
    pub fn to_json(&self) -> SimResult<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }
}

/// Cross product of two vectors.
fn cross(a: Vec3, b: Vec3) -> Vec3 {
    Vec3::new(
        a.y * b.z - a.z * b.y,
        a.z * b.x - a.x * b.z,
        a.x * b.y - a.y * b.x,
    )
}

/// Sonifies aggregate [`SimulationMetrics`] into a [`SpatialAudioScene`].
///
/// Each statute becomes an [`AudioSource`] via [`Sonifier::for_metrics`]; void-
/// heavy statutes get a [`Waveform::Noise`] timbre so failures sound gritty.
#[must_use]
pub fn sonify_metrics(metrics: &SimulationMetrics) -> SpatialAudioScene {
    let sonifier = Sonifier::for_metrics();
    let mut scene = SpatialAudioScene::new();

    let max_total = metrics
        .statute_metrics
        .values()
        .map(|m| m.total)
        .max()
        .unwrap_or(0)
        .max(1) as f64;

    let mut entries: Vec<(&String, &StatuteMetrics)> = metrics.statute_metrics.iter().collect();
    entries.sort_by(|a, b| a.0.cmp(b.0));

    for (statute_id, sm) in entries {
        let mut record = BTreeMap::new();
        record.insert("effectiveness".to_string(), sm.effectiveness());
        record.insert("ambiguity".to_string(), sm.ambiguity());
        record.insert("volume".to_string(), sm.total as f64 / max_total);
        let void_ratio = if sm.total == 0 {
            0.0
        } else {
            sm.void as f64 / sm.total as f64
        };
        let mut source = sonifier
            .sonify(format!("audio::{statute_id}"), &record)
            .with_label(statute_id.clone());
        if void_ratio > 0.5 {
            source.waveform = Waveform::Noise;
        }
        scene.add_source(source);
    }

    scene
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::LawApplicationResult;
    use legalis_core::{Effect, EffectType, LegalResult};

    fn record(m: &mut SimulationMetrics, statute: &str, result: LegalResult<Effect>) {
        m.record_result(&LawApplicationResult {
            agent_id: uuid::Uuid::new_v4(),
            statute_id: statute.to_string(),
            result,
        });
    }

    fn metrics() -> SimulationMetrics {
        let mut m = SimulationMetrics::new();
        for _ in 0..8 {
            record(
                &mut m,
                "effective",
                LegalResult::Deterministic(Effect::new(EffectType::Grant, "ok")),
            );
        }
        for _ in 0..3 {
            record(
                &mut m,
                "fuzzy",
                LegalResult::JudicialDiscretion {
                    issue: "x".to_string(),
                    context_id: uuid::Uuid::new_v4(),
                    narrative_hint: None,
                },
            );
        }
        m
    }

    #[test]
    fn test_pitch_log_interpolation() {
        let s = Sonifier::default();
        assert!((s.pitch_for(0.0) - 110.0).abs() < 1e-6);
        assert!((s.pitch_for(1.0) - 1760.0).abs() < 1e-6);
        // Log scale: midpoint is the geometric mean (one octave above min).
        assert!((s.pitch_for(0.5) - (110.0 * 1760.0_f64).sqrt()).abs() < 1e-6);
    }

    #[test]
    fn test_sonify_record_maps_params() {
        let s = Sonifier::for_metrics();
        let mut rec = BTreeMap::new();
        rec.insert("effectiveness".to_string(), 1.0);
        rec.insert("ambiguity".to_string(), 0.0);
        rec.insert("volume".to_string(), 0.5);
        let src = s.sonify("x", &rec);
        // Effectiveness 1.0 → top of pitch band and full-right X.
        assert!((src.pitch_hz - 1760.0).abs() < 1e-6);
        assert!((src.position.x - 8.0).abs() < 1e-6);
        assert!((src.gain - 0.5).abs() < 1e-9);
        // Ambiguity 0.0 → fully forward (Z = -extent).
        assert!((src.position.z + 8.0).abs() < 1e-6);
    }

    #[test]
    fn test_perceived_gain_attenuation() {
        let mut scene = SpatialAudioScene::new();
        scene.add_source(AudioSource::new("near", 440.0, 1.0).at(Vec3::new(0.0, 0.0, -1.0)));
        scene.add_source(AudioSource::new("far", 440.0, 1.0).at(Vec3::new(0.0, 0.0, -10.0)));
        scene.add_source(AudioSource::new("gone", 440.0, 1.0).at(Vec3::new(0.0, 0.0, -100.0)));
        let near = &scene.sources()[0].clone();
        let far = &scene.sources()[1].clone();
        let gone = &scene.sources()[2].clone();
        assert!(scene.perceived_gain(near) > scene.perceived_gain(far));
        assert!((scene.perceived_gain(gone)).abs() < 1e-9); // beyond max_distance
        assert!(scene.mix_level() > 0.0);
    }

    #[test]
    fn test_pan_left_right() {
        let scene = SpatialAudioScene::new(); // facing -Z, right = +X
        let right_src = AudioSource::new("r", 440.0, 1.0).at(Vec3::new(5.0, 0.0, -1.0));
        let left_src = AudioSource::new("l", 440.0, 1.0).at(Vec3::new(-5.0, 0.0, -1.0));
        assert!(scene.pan(&right_src) > 0.3);
        assert!(scene.pan(&left_src) < -0.3);
    }

    #[test]
    fn test_sonify_metrics_scene() {
        let scene = sonify_metrics(&metrics());
        assert_eq!(scene.len(), 2);
        let effective = scene
            .sources()
            .iter()
            .find(|s| s.label == "effective")
            .expect("effective source");
        let fuzzy = scene
            .sources()
            .iter()
            .find(|s| s.label == "fuzzy")
            .expect("fuzzy source");
        // Effective statute (effectiveness 1.0) is higher-pitched than the fuzzy one.
        assert!(effective.pitch_hz > fuzzy.pitch_hz);
    }

    #[test]
    fn test_void_heavy_statute_uses_noise_and_json() {
        let mut m = SimulationMetrics::new();
        for _ in 0..4 {
            record(
                &mut m,
                "broken",
                LegalResult::Void {
                    reason: "x".to_string(),
                },
            );
        }
        let scene = sonify_metrics(&m);
        assert_eq!(scene.sources()[0].waveform, Waveform::Noise);
        let json = scene.to_json().expect("json");
        let restored: SpatialAudioScene = serde_json::from_str(&json).expect("roundtrip");
        assert_eq!(restored.len(), scene.len());
    }
}
