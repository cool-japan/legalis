//! Adaptive audit policies.
//!
//! An [`AdaptiveAuditPolicy`] holds the *tunable knobs* of the audit programme —
//! how aggressively to sample/verify records and where the alerting thresholds
//! sit — and adjusts them automatically in response to an observed
//! [`RiskLevel`]. The core idea of autonomous compliance is a control loop:
//!
//! 1. The monitor + forecaster produce a [`RiskAssessment`].
//! 2. The policy [`adapt`](AdaptiveAuditPolicy::adapt)s: under elevated risk it
//!    raises the sampling rate (more scrutiny) and tightens thresholds (lower
//!    tolerance); as risk subsides it relaxes back toward baseline, but never
//!    past safety floors/ceilings.
//! 3. Every adaptation is captured as a [`PolicyAdjustment`] so the policy's
//!    evolution is itself auditable.
//!
//! The policy is *bounded* and *hysteretic*: it moves a fraction of the way
//! toward the target each step (avoiding oscillation) and is clamped to
//! `[min_sampling_rate, max_sampling_rate]` and threshold floors.

use crate::autonomous::monitor::MonitoredMetric;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// A coarse, ordered risk level driving policy adaptation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RiskLevel {
    /// Nominal operation.
    Low,
    /// Slightly elevated; tighten modestly.
    Moderate,
    /// Materially elevated; tighten and sample more.
    High,
    /// Severe; maximal scrutiny.
    Critical,
}

impl RiskLevel {
    /// Maps a continuous risk score in `[0, 1]` to a level.
    pub fn from_score(score: f64) -> Self {
        let s = score.clamp(0.0, 1.0);
        if s >= 0.8 {
            RiskLevel::Critical
        } else if s >= 0.55 {
            RiskLevel::High
        } else if s >= 0.3 {
            RiskLevel::Moderate
        } else {
            RiskLevel::Low
        }
    }

    /// A multiplier in `[0, 1]` indicating how far toward the strict end the
    /// policy should move for this level.
    pub fn intensity(self) -> f64 {
        match self {
            RiskLevel::Low => 0.0,
            RiskLevel::Moderate => 0.35,
            RiskLevel::High => 0.7,
            RiskLevel::Critical => 1.0,
        }
    }

    /// Stable lower-snake label.
    pub fn label(self) -> &'static str {
        match self {
            RiskLevel::Low => "low",
            RiskLevel::Moderate => "moderate",
            RiskLevel::High => "high",
            RiskLevel::Critical => "critical",
        }
    }
}

/// A continuous, explainable risk assessment feeding policy adaptation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    /// Composite risk score in `[0, 1]`.
    pub score: f64,
    /// The discretised level.
    pub level: RiskLevel,
    /// Named contributing factors and their `[0, 1]` contributions.
    pub factors: HashMap<String, f64>,
    /// When assessed.
    pub assessed_at: DateTime<Utc>,
}

impl RiskAssessment {
    /// Builds an assessment from a composite score and factor map.
    pub fn new(score: f64, factors: HashMap<String, f64>) -> Self {
        let score = score.clamp(0.0, 1.0);
        Self {
            level: RiskLevel::from_score(score),
            score,
            factors,
            assessed_at: Utc::now(),
        }
    }

    /// Convenience constructor from a single number of findings and their max
    /// normalised severity (both in sensible ranges), producing a blended
    /// score. `finding_pressure` is a `[0, 1]` density of findings;
    /// `severity_weight` is a `[0, 1]` peak-severity weight.
    pub fn from_signals(finding_pressure: f64, severity_weight: f64) -> Self {
        let fp = finding_pressure.clamp(0.0, 1.0);
        let sw = severity_weight.clamp(0.0, 1.0);
        let score = (fp * 0.5 + sw * 0.5).clamp(0.0, 1.0);
        let mut factors = HashMap::new();
        factors.insert("finding_pressure".to_string(), fp);
        factors.insert("severity_weight".to_string(), sw);
        Self::new(score, factors)
    }
}

/// One audit-threshold knob the policy controls (mirrors a
/// [`MonitoredMetric`] ceiling).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThresholdKnob {
    /// The metric this threshold governs.
    pub metric: MonitoredMetric,
    /// Baseline (relaxed) threshold.
    pub baseline: f64,
    /// The strictest the threshold may become.
    pub floor: f64,
    /// The current, possibly-tightened threshold.
    pub current: f64,
}

impl ThresholdKnob {
    /// Builds a knob, clamping `floor <= baseline` and starting at baseline.
    pub fn new(metric: MonitoredMetric, baseline: f64, floor: f64) -> Self {
        let floor = floor.min(baseline);
        Self {
            metric,
            baseline,
            floor,
            current: baseline,
        }
    }

    /// Adapts `current` toward a risk-scaled target between baseline and floor.
    /// Returns the previous value.
    fn adapt(&mut self, intensity: f64, responsiveness: f64) -> f64 {
        let prev = self.current;
        // Target tightens from baseline toward floor as intensity rises.
        let target = self.baseline - (self.baseline - self.floor) * intensity.clamp(0.0, 1.0);
        // Move a `responsiveness` fraction of the way (hysteresis).
        self.current += (target - self.current) * responsiveness.clamp(0.0, 1.0);
        // Clamp into bounds.
        if self.current < self.floor {
            self.current = self.floor;
        }
        if self.current > self.baseline {
            self.current = self.baseline;
        }
        prev
    }
}

/// A single recorded change to the policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyAdjustment {
    /// Stable identifier.
    pub id: Uuid,
    /// The risk level that drove the adjustment.
    pub risk_level: RiskLevel,
    /// The risk score that drove the adjustment.
    pub risk_score: f64,
    /// Previous sampling rate.
    pub previous_sampling_rate: f64,
    /// New sampling rate.
    pub new_sampling_rate: f64,
    /// Per-metric (previous, new) thresholds that changed.
    pub threshold_changes: HashMap<String, (f64, f64)>,
    /// When the adjustment occurred.
    pub adjusted_at: DateTime<Utc>,
}

impl PolicyAdjustment {
    /// `true` when this adjustment changed nothing materially.
    pub fn is_noop(&self) -> bool {
        (self.previous_sampling_rate - self.new_sampling_rate).abs() < 1e-9
            && self
                .threshold_changes
                .values()
                .all(|(p, n)| (p - n).abs() < 1e-9)
    }
}

/// Configuration for an [`AdaptiveAuditPolicy`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptivePolicyConfig {
    /// Baseline (low-risk) sampling/verification rate in `[0, 1]`.
    pub baseline_sampling_rate: f64,
    /// Hard minimum sampling rate (never go below).
    pub min_sampling_rate: f64,
    /// Hard maximum sampling rate (never go above).
    pub max_sampling_rate: f64,
    /// Fraction of the way to move toward a target each step (hysteresis).
    pub responsiveness: f64,
}

impl Default for AdaptivePolicyConfig {
    fn default() -> Self {
        Self {
            baseline_sampling_rate: 0.1,
            min_sampling_rate: 0.05,
            max_sampling_rate: 1.0,
            responsiveness: 0.5,
        }
    }
}

/// A self-adjusting audit policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveAuditPolicy {
    config: AdaptivePolicyConfig,
    /// Current sampling/verification rate.
    sampling_rate: f64,
    /// Tunable threshold knobs keyed by metric label.
    knobs: Vec<ThresholdKnob>,
    /// History of adjustments (most recent last).
    history: Vec<PolicyAdjustment>,
}

impl AdaptiveAuditPolicy {
    /// Creates a policy with the given configuration and no threshold knobs.
    pub fn new(config: AdaptivePolicyConfig) -> Self {
        let sampling_rate = config
            .baseline_sampling_rate
            .clamp(config.min_sampling_rate, config.max_sampling_rate);
        Self {
            config,
            sampling_rate,
            knobs: Vec::new(),
            history: Vec::new(),
        }
    }

    /// Creates a policy with default config and sensible override/void
    /// threshold knobs.
    pub fn with_defaults() -> Self {
        Self::new(AdaptivePolicyConfig::default())
            .add_knob(ThresholdKnob::new(MonitoredMetric::OverrideRate, 0.25, 0.1))
            .add_knob(ThresholdKnob::new(MonitoredMetric::VoidRate, 0.1, 0.02))
    }

    /// Adds a threshold knob (builder style).
    pub fn add_knob(mut self, knob: ThresholdKnob) -> Self {
        self.knobs.push(knob);
        self
    }

    /// Current sampling/verification rate.
    pub fn sampling_rate(&self) -> f64 {
        self.sampling_rate
    }

    /// The current threshold for `metric`, if a knob governs it.
    pub fn threshold(&self, metric: MonitoredMetric) -> Option<f64> {
        self.knobs
            .iter()
            .find(|k| k.metric == metric)
            .map(|k| k.current)
    }

    /// All current knobs.
    pub fn knobs(&self) -> &[ThresholdKnob] {
        &self.knobs
    }

    /// The recorded adjustment history.
    pub fn history(&self) -> &[PolicyAdjustment] {
        &self.history
    }

    /// Adapts the policy to the supplied risk assessment, recording and
    /// returning the [`PolicyAdjustment`].
    pub fn adapt(&mut self, assessment: &RiskAssessment) -> PolicyAdjustment {
        let intensity = assessment.level.intensity().max(assessment.score);
        let responsiveness = self.config.responsiveness;

        // Sampling rate scales between baseline and max with intensity.
        let prev_rate = self.sampling_rate;
        let target_rate = self.config.baseline_sampling_rate
            + (self.config.max_sampling_rate - self.config.baseline_sampling_rate) * intensity;
        let mut new_rate = self.sampling_rate + (target_rate - self.sampling_rate) * responsiveness;
        new_rate = new_rate.clamp(self.config.min_sampling_rate, self.config.max_sampling_rate);
        self.sampling_rate = new_rate;

        // Adapt each threshold knob.
        let mut threshold_changes = HashMap::new();
        for knob in &mut self.knobs {
            let prev = knob.adapt(intensity, responsiveness);
            if (prev - knob.current).abs() > 1e-12 {
                threshold_changes.insert(knob.metric.label().to_string(), (prev, knob.current));
            }
        }

        let adjustment = PolicyAdjustment {
            id: Uuid::new_v4(),
            risk_level: assessment.level,
            risk_score: assessment.score,
            previous_sampling_rate: prev_rate,
            new_sampling_rate: new_rate,
            threshold_changes,
            adjusted_at: Utc::now(),
        };
        self.history.push(adjustment.clone());
        adjustment
    }

    /// Resets the policy back to baseline (sampling rate and all knobs),
    /// recording the reset as an adjustment at `RiskLevel::Low`.
    pub fn reset_to_baseline(&mut self) -> PolicyAdjustment {
        let prev_rate = self.sampling_rate;
        self.sampling_rate = self
            .config
            .baseline_sampling_rate
            .clamp(self.config.min_sampling_rate, self.config.max_sampling_rate);
        let mut threshold_changes = HashMap::new();
        for knob in &mut self.knobs {
            let prev = knob.current;
            knob.current = knob.baseline;
            if (prev - knob.current).abs() > 1e-12 {
                threshold_changes.insert(knob.metric.label().to_string(), (prev, knob.current));
            }
        }
        let adjustment = PolicyAdjustment {
            id: Uuid::new_v4(),
            risk_level: RiskLevel::Low,
            risk_score: 0.0,
            previous_sampling_rate: prev_rate,
            new_sampling_rate: self.sampling_rate,
            threshold_changes,
            adjusted_at: Utc::now(),
        };
        self.history.push(adjustment.clone());
        adjustment
    }
}

impl Default for AdaptiveAuditPolicy {
    fn default() -> Self {
        Self::with_defaults()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_risk_level_from_score() {
        assert_eq!(RiskLevel::from_score(0.0), RiskLevel::Low);
        assert_eq!(RiskLevel::from_score(0.4), RiskLevel::Moderate);
        assert_eq!(RiskLevel::from_score(0.6), RiskLevel::High);
        assert_eq!(RiskLevel::from_score(0.95), RiskLevel::Critical);
        assert!(RiskLevel::Critical > RiskLevel::Low);
    }

    #[test]
    fn test_high_risk_raises_sampling_and_tightens() {
        let mut policy = AdaptiveAuditPolicy::with_defaults();
        let base_rate = policy.sampling_rate();
        let base_override = policy.threshold(MonitoredMetric::OverrideRate).unwrap();

        let assessment = RiskAssessment::new(0.9, HashMap::new());
        let adj = policy.adapt(&assessment);

        assert!(policy.sampling_rate() > base_rate);
        assert!(policy.threshold(MonitoredMetric::OverrideRate).unwrap() < base_override);
        assert_eq!(adj.risk_level, RiskLevel::Critical);
        assert!(!adj.is_noop());
    }

    #[test]
    fn test_low_risk_no_tightening() {
        let mut policy = AdaptiveAuditPolicy::with_defaults();
        let base_override = policy.threshold(MonitoredMetric::OverrideRate).unwrap();
        let assessment = RiskAssessment::new(0.0, HashMap::new());
        policy.adapt(&assessment);
        // At zero intensity from baseline, threshold stays at baseline.
        assert!(
            (policy.threshold(MonitoredMetric::OverrideRate).unwrap() - base_override).abs() < 1e-9
        );
    }

    #[test]
    fn test_sampling_clamped_to_bounds() {
        let config = AdaptivePolicyConfig {
            baseline_sampling_rate: 0.2,
            min_sampling_rate: 0.1,
            max_sampling_rate: 0.5,
            responsiveness: 1.0,
        };
        let mut policy = AdaptiveAuditPolicy::new(config);
        let assessment = RiskAssessment::new(1.0, HashMap::new());
        policy.adapt(&assessment);
        assert!(policy.sampling_rate() <= 0.5 + 1e-9);
        // Even repeated max-risk adaptation never exceeds the ceiling.
        policy.adapt(&assessment);
        assert!(policy.sampling_rate() <= 0.5 + 1e-9);
    }

    #[test]
    fn test_threshold_never_below_floor() {
        let mut policy = AdaptiveAuditPolicy::new(AdaptivePolicyConfig::default())
            .add_knob(ThresholdKnob::new(MonitoredMetric::VoidRate, 0.1, 0.05));
        let assessment = RiskAssessment::new(1.0, HashMap::new());
        // Many high-risk steps drive it down but never below the floor.
        for _ in 0..50 {
            policy.adapt(&assessment);
        }
        let t = policy.threshold(MonitoredMetric::VoidRate).unwrap();
        assert!(t >= 0.05 - 1e-9);
        assert!(t <= 0.06);
    }

    #[test]
    fn test_relaxation_after_risk_subsides() {
        let mut policy = AdaptiveAuditPolicy::with_defaults();
        // Spike then calm.
        policy.adapt(&RiskAssessment::new(0.9, HashMap::new()));
        let tight = policy.threshold(MonitoredMetric::OverrideRate).unwrap();
        for _ in 0..10 {
            policy.adapt(&RiskAssessment::new(0.0, HashMap::new()));
        }
        let relaxed = policy.threshold(MonitoredMetric::OverrideRate).unwrap();
        assert!(relaxed > tight);
    }

    #[test]
    fn test_reset_to_baseline() {
        let mut policy = AdaptiveAuditPolicy::with_defaults();
        policy.adapt(&RiskAssessment::new(0.9, HashMap::new()));
        let adj = policy.reset_to_baseline();
        assert_eq!(adj.risk_level, RiskLevel::Low);
        assert!((policy.sampling_rate() - 0.1).abs() < 1e-9);
        assert!((policy.threshold(MonitoredMetric::OverrideRate).unwrap() - 0.25).abs() < 1e-9);
    }

    #[test]
    fn test_history_records_each_adaptation() {
        let mut policy = AdaptiveAuditPolicy::with_defaults();
        policy.adapt(&RiskAssessment::new(0.9, HashMap::new()));
        policy.adapt(&RiskAssessment::new(0.5, HashMap::new()));
        assert_eq!(policy.history().len(), 2);
    }

    #[test]
    fn test_assessment_from_signals() {
        let a = RiskAssessment::from_signals(1.0, 1.0);
        assert_eq!(a.level, RiskLevel::Critical);
        assert!(a.factors.contains_key("finding_pressure"));
        let b = RiskAssessment::from_signals(0.0, 0.0);
        assert_eq!(b.level, RiskLevel::Low);
    }

    #[test]
    fn test_policy_serializes() {
        let mut policy = AdaptiveAuditPolicy::with_defaults();
        policy.adapt(&RiskAssessment::new(0.7, HashMap::new()));
        let json = serde_json::to_string(&policy).expect("serialize");
        assert!(json.contains("sampling_rate"));
    }
}
