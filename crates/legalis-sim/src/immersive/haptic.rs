//! Haptic-feedback encoding for impact perception.
//!
//! This module turns *impact metrics* into a playable [`HapticPattern`]: a
//! time-lined sequence of [`HapticCue`]s on **force** and **vibration** channels.
//! A controller, glove or wearable can render the pattern so that an analyst
//! literally *feels* a simulation's character — a steady force for solid,
//! deterministic outcomes; a buzzing vibration whose pitch rises with judicial
//! ambiguity; a sharp jolt for void/failed applications.
//!
//! Building the pattern is pure Rust and done here; driving a physical actuator
//! is the deferred external binding (see [module overview](super)).

use super::Color;
use crate::SimResult;
use crate::metrics::SimulationMetrics;
use serde::{Deserialize, Serialize};

/// A physical actuator channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HapticChannel {
    /// A sustained force / pressure (e.g. a resistive trigger).
    Force,
    /// An oscillating vibration (e.g. an LRA / eccentric-rotating-mass motor).
    Vibration,
}

impl HapticChannel {
    /// A short, stable string tag.
    #[must_use]
    pub fn tag(&self) -> &'static str {
        match self {
            HapticChannel::Force => "force",
            HapticChannel::Vibration => "vibration",
        }
    }
}

/// The amplitude envelope of a [`HapticCue`] over its duration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HapticWaveform {
    /// Constant amplitude for the whole cue.
    Constant,
    /// A short rising-then-falling pulse.
    Pulse,
    /// A linear ramp up to peak amplitude.
    Ramp,
    /// A sinusoidal oscillation (natural for vibration).
    Sine,
    /// A single sharp transient click.
    Click,
}

impl HapticWaveform {
    /// A short, stable string tag.
    #[must_use]
    pub fn tag(&self) -> &'static str {
        match self {
            HapticWaveform::Constant => "constant",
            HapticWaveform::Pulse => "pulse",
            HapticWaveform::Ramp => "ramp",
            HapticWaveform::Sine => "sine",
            HapticWaveform::Click => "click",
        }
    }

    /// Samples the normalised envelope at fractional time `phase` in `[0, 1]`.
    ///
    /// Returns a value in `[0, 1]` describing the relative amplitude; multiply by
    /// a cue's `intensity` to get the instantaneous amplitude.
    #[must_use]
    pub fn envelope(&self, phase: f64) -> f64 {
        let p = phase.clamp(0.0, 1.0);
        match self {
            HapticWaveform::Constant => 1.0,
            HapticWaveform::Pulse => {
                // Triangular: 0 → 1 at the midpoint → 0.
                1.0 - (2.0 * p - 1.0).abs()
            }
            HapticWaveform::Ramp => p,
            HapticWaveform::Sine => (std::f64::consts::PI * p).sin(),
            HapticWaveform::Click => {
                if p < 0.1 {
                    1.0
                } else {
                    0.0
                }
            }
        }
    }
}

/// A single haptic cue: an amplitude envelope on one channel for a span of time.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HapticCue {
    /// Human-readable label describing what the cue represents.
    pub label: String,
    /// The actuator channel.
    pub channel: HapticChannel,
    /// Peak amplitude in `[0, 1]`.
    pub intensity: f64,
    /// Cue duration, in milliseconds.
    pub duration_ms: u32,
    /// Vibration frequency, in Hz (meaningful for [`HapticChannel::Vibration`]).
    pub frequency_hz: f64,
    /// Amplitude envelope.
    pub waveform: HapticWaveform,
    /// Start offset within the owning pattern, in milliseconds.
    pub start_ms: u32,
}

impl HapticCue {
    /// Creates a cue, clamping `intensity` to `[0, 1]` and `frequency` to `≥ 0`.
    #[must_use]
    pub fn new(
        label: impl Into<String>,
        channel: HapticChannel,
        intensity: f64,
        duration_ms: u32,
    ) -> Self {
        Self {
            label: label.into(),
            channel,
            intensity: intensity.clamp(0.0, 1.0),
            duration_ms,
            frequency_hz: 0.0,
            waveform: HapticWaveform::Constant,
            start_ms: 0,
        }
    }

    /// Builder: sets the vibration frequency (clamped to `≥ 0`).
    #[must_use]
    pub fn with_frequency(mut self, frequency_hz: f64) -> Self {
        self.frequency_hz = frequency_hz.max(0.0);
        self
    }

    /// Builder: sets the waveform.
    #[must_use]
    pub fn with_waveform(mut self, waveform: HapticWaveform) -> Self {
        self.waveform = waveform;
        self
    }

    /// Builder: sets the start offset.
    #[must_use]
    pub fn starting_at(mut self, start_ms: u32) -> Self {
        self.start_ms = start_ms;
        self
    }

    /// The millisecond at which this cue ends (`start + duration`).
    #[must_use]
    pub fn end_ms(&self) -> u32 {
        self.start_ms.saturating_add(self.duration_ms)
    }
}

/// A time-lined sequence of haptic cues forming a single playable pattern.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HapticPattern {
    cues: Vec<HapticCue>,
}

impl HapticPattern {
    /// Creates an empty pattern.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Appends a cue.
    pub fn push(&mut self, cue: HapticCue) {
        self.cues.push(cue);
    }

    /// All cues, in insertion order.
    #[must_use]
    pub fn cues(&self) -> &[HapticCue] {
        &self.cues
    }

    /// Number of cues.
    #[must_use]
    pub fn len(&self) -> usize {
        self.cues.len()
    }

    /// Returns `true` if the pattern has no cues.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.cues.is_empty()
    }

    /// Total wall-clock duration, in milliseconds (the latest cue end).
    #[must_use]
    pub fn total_duration_ms(&self) -> u32 {
        self.cues.iter().map(HapticCue::end_ms).max().unwrap_or(0)
    }

    /// The maximum cue intensity in the pattern.
    #[must_use]
    pub fn peak_intensity(&self) -> f64 {
        self.cues
            .iter()
            .map(|c| c.intensity)
            .fold(0.0_f64, f64::max)
    }

    /// Cues on a given channel.
    #[must_use]
    pub fn channel_cues(&self, channel: HapticChannel) -> Vec<&HapticCue> {
        self.cues.iter().filter(|c| c.channel == channel).collect()
    }

    /// Serialises the pattern to pretty JSON.
    ///
    /// # Errors
    ///
    /// Returns [`crate::SimulationError::Serialization`] if serialisation fails.
    pub fn to_json(&self) -> SimResult<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }
}

/// A normalised, named impact signal in `[0, 1]` — the generic input to the
/// [`HapticEncoder`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImpactSignal {
    /// What the signal measures (e.g. "ambiguity").
    pub label: String,
    /// Normalised magnitude in `[0, 1]`.
    pub intensity: f64,
}

impl ImpactSignal {
    /// Creates a signal, clamping `intensity` to `[0, 1]`.
    #[must_use]
    pub fn new(label: impl Into<String>, intensity: f64) -> Self {
        Self {
            label: label.into(),
            intensity: intensity.clamp(0.0, 1.0),
        }
    }

    /// A heat colour for cross-referencing the signal with a visual cue.
    #[must_use]
    pub fn color(&self) -> Color {
        Color::heat(self.intensity)
    }

    /// Derives the canonical impact signals from a [`SimulationMetrics`]:
    /// `solidity` (deterministic ratio), `ambiguity` (discretion ratio) and
    /// `failure` (void ratio).
    #[must_use]
    pub fn from_metrics(metrics: &SimulationMetrics) -> Vec<Self> {
        let total = metrics.total_applications.max(1) as f64;
        let void_ratio = metrics.void_count as f64 / total;
        vec![
            Self::new("solidity", metrics.deterministic_ratio()),
            Self::new("ambiguity", metrics.discretion_ratio()),
            Self::new("failure", void_ratio),
        ]
    }
}

/// Configurable mapping from impact signals to haptic cues.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HapticEncoder {
    /// Base duration of each generated cue, in milliseconds.
    pub base_cue_ms: u32,
    /// Silent gap inserted between sequenced cues, in milliseconds.
    pub gap_ms: u32,
    /// Lowest vibration frequency (intensity 0), in Hz.
    pub min_frequency_hz: f64,
    /// Highest vibration frequency (intensity 1), in Hz.
    pub max_frequency_hz: f64,
    /// Perceptual exponent applied to force amplitude (`> 0`).
    pub force_gamma: f64,
}

impl Default for HapticEncoder {
    fn default() -> Self {
        Self {
            base_cue_ms: 200,
            gap_ms: 50,
            min_frequency_hz: 80.0,
            max_frequency_hz: 250.0,
            force_gamma: 1.0,
        }
    }
}

impl HapticEncoder {
    /// Creates an encoder with default settings.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Maps an intensity to a vibration frequency within the configured band.
    #[must_use]
    pub fn frequency_for(&self, intensity: f64) -> f64 {
        let t = intensity.clamp(0.0, 1.0);
        self.min_frequency_hz + (self.max_frequency_hz - self.min_frequency_hz) * t
    }

    /// Applies the perceptual force curve to an intensity.
    #[must_use]
    pub fn force_amplitude(&self, intensity: f64) -> f64 {
        let gamma = if self.force_gamma > 0.0 {
            self.force_gamma
        } else {
            1.0
        };
        intensity.clamp(0.0, 1.0).powf(gamma)
    }

    /// Encodes a single signal onto a channel as one cue, starting at `start_ms`.
    ///
    /// Vibration cues get a frequency from [`HapticEncoder::frequency_for`] and a
    /// [`HapticWaveform::Sine`] envelope; force cues use the perceptual amplitude
    /// curve and a [`HapticWaveform::Ramp`] envelope.
    #[must_use]
    pub fn encode_signal(
        &self,
        signal: &ImpactSignal,
        channel: HapticChannel,
        start_ms: u32,
    ) -> HapticCue {
        match channel {
            HapticChannel::Vibration => HapticCue::new(
                signal.label.clone(),
                channel,
                signal.intensity,
                self.base_cue_ms,
            )
            .with_frequency(self.frequency_for(signal.intensity))
            .with_waveform(HapticWaveform::Sine)
            .starting_at(start_ms),
            HapticChannel::Force => HapticCue::new(
                signal.label.clone(),
                channel,
                self.force_amplitude(signal.intensity),
                self.base_cue_ms,
            )
            .with_waveform(HapticWaveform::Ramp)
            .starting_at(start_ms),
        }
    }

    /// Encodes a full [`HapticPattern`] from a [`SimulationMetrics`].
    ///
    /// The pattern sequences up to three cues (only non-zero signals are
    /// emitted): a **force** cue for deterministic *solidity*, a **vibration**
    /// cue whose pitch rises with *ambiguity*, and a sharp **force pulse** for
    /// the *failure* (void) rate.
    #[must_use]
    pub fn encode_metrics(&self, metrics: &SimulationMetrics) -> HapticPattern {
        let mut pattern = HapticPattern::new();
        let step = self.base_cue_ms.saturating_add(self.gap_ms);
        let mut slot: u32 = 0;

        let total = metrics.total_applications.max(1) as f64;
        let void_ratio = metrics.void_count as f64 / total;
        let solidity = metrics.deterministic_ratio();
        let ambiguity = metrics.discretion_ratio();

        if solidity > f64::EPSILON {
            pattern.push(
                self.encode_signal(
                    &ImpactSignal::new("solidity", solidity),
                    HapticChannel::Force,
                    slot,
                )
                .with_waveform(HapticWaveform::Constant),
            );
            slot = slot.saturating_add(step);
        }
        if ambiguity > f64::EPSILON {
            pattern.push(self.encode_signal(
                &ImpactSignal::new("ambiguity", ambiguity),
                HapticChannel::Vibration,
                slot,
            ));
            slot = slot.saturating_add(step);
        }
        if void_ratio > f64::EPSILON {
            pattern.push(
                HapticCue::new(
                    "failure",
                    HapticChannel::Force,
                    self.force_amplitude(void_ratio),
                    self.base_cue_ms,
                )
                .with_waveform(HapticWaveform::Pulse)
                .starting_at(slot),
            );
        }

        pattern
    }
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

    fn mixed_metrics() -> SimulationMetrics {
        let mut m = SimulationMetrics::new();
        for _ in 0..5 {
            record(
                &mut m,
                "s",
                LegalResult::Deterministic(Effect::new(EffectType::Grant, "ok")),
            );
        }
        for _ in 0..3 {
            record(
                &mut m,
                "s",
                LegalResult::JudicialDiscretion {
                    issue: "x".to_string(),
                    context_id: uuid::Uuid::new_v4(),
                    narrative_hint: None,
                },
            );
        }
        for _ in 0..2 {
            record(
                &mut m,
                "s",
                LegalResult::Void {
                    reason: "bad".to_string(),
                },
            );
        }
        m
    }

    #[test]
    fn test_waveform_envelopes() {
        assert!((HapticWaveform::Constant.envelope(0.3) - 1.0).abs() < 1e-9);
        assert!((HapticWaveform::Pulse.envelope(0.5) - 1.0).abs() < 1e-9);
        assert!((HapticWaveform::Pulse.envelope(0.0)).abs() < 1e-9);
        assert!((HapticWaveform::Ramp.envelope(0.4) - 0.4).abs() < 1e-9);
        assert!(HapticWaveform::Sine.envelope(0.5) > 0.99);
        assert!((HapticWaveform::Click.envelope(0.5)).abs() < 1e-9);
    }

    #[test]
    fn test_frequency_and_force_mapping() {
        let enc = HapticEncoder::new();
        assert!((enc.frequency_for(0.0) - 80.0).abs() < 1e-9);
        assert!((enc.frequency_for(1.0) - 250.0).abs() < 1e-9);
        assert!(enc.frequency_for(0.5) > enc.frequency_for(0.0));
        // Perceptual curve with gamma=2 compresses low amplitudes.
        let enc2 = HapticEncoder {
            force_gamma: 2.0,
            ..HapticEncoder::default()
        };
        assert!((enc2.force_amplitude(0.5) - 0.25).abs() < 1e-9);
        // Degenerate gamma falls back to linear.
        let enc0 = HapticEncoder {
            force_gamma: 0.0,
            ..HapticEncoder::default()
        };
        assert!((enc0.force_amplitude(0.5) - 0.5).abs() < 1e-9);
    }

    #[test]
    fn test_encode_signal_per_channel() {
        let enc = HapticEncoder::new();
        let sig = ImpactSignal::new("ambiguity", 1.0);
        let vib = enc.encode_signal(&sig, HapticChannel::Vibration, 100);
        assert_eq!(vib.channel, HapticChannel::Vibration);
        assert_eq!(vib.waveform, HapticWaveform::Sine);
        assert!((vib.frequency_hz - 250.0).abs() < 1e-9);
        assert_eq!(vib.start_ms, 100);
        let force = enc.encode_signal(&sig, HapticChannel::Force, 0);
        assert_eq!(force.channel, HapticChannel::Force);
        assert_eq!(force.waveform, HapticWaveform::Ramp);
    }

    #[test]
    fn test_encode_metrics_full_timeline() {
        let enc = HapticEncoder::new();
        let pattern = enc.encode_metrics(&mixed_metrics());
        // solidity (force) + ambiguity (vibration) + failure (force pulse).
        assert_eq!(pattern.len(), 3);
        assert_eq!(pattern.channel_cues(HapticChannel::Force).len(), 2);
        assert_eq!(pattern.channel_cues(HapticChannel::Vibration).len(), 1);
        // Cues are sequenced (strictly increasing starts), pattern non-trivial.
        let starts: Vec<u32> = pattern.cues().iter().map(|c| c.start_ms).collect();
        assert!(starts.windows(2).all(|w| w[0] < w[1]));
        assert_eq!(pattern.total_duration_ms(), 200 + 2 * 250);
        assert!(pattern.peak_intensity() > 0.0);
    }

    #[test]
    fn test_encode_metrics_skips_zero_signals() {
        let mut m = SimulationMetrics::new();
        for _ in 0..4 {
            record(
                &mut m,
                "s",
                LegalResult::Deterministic(Effect::new(EffectType::Grant, "ok")),
            );
        }
        let pattern = HapticEncoder::new().encode_metrics(&m);
        // Only the deterministic "solidity" force cue should be present.
        assert_eq!(pattern.len(), 1);
        assert_eq!(pattern.cues()[0].channel, HapticChannel::Force);
        assert_eq!(pattern.cues()[0].label, "solidity");
    }

    #[test]
    fn test_signals_from_metrics_and_json() {
        let signals = ImpactSignal::from_metrics(&mixed_metrics());
        assert_eq!(signals.len(), 3);
        let ambiguity = signals.iter().find(|s| s.label == "ambiguity").unwrap();
        assert!((ambiguity.intensity - 0.3).abs() < 1e-9);
        // JSON round-trip of a pattern.
        let pattern = HapticEncoder::new().encode_metrics(&mixed_metrics());
        let json = pattern.to_json().expect("json");
        let restored: HapticPattern = serde_json::from_str(&json).expect("roundtrip");
        assert_eq!(restored.len(), pattern.len());
    }
}
