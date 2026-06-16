//! Audit findings, severity model, and risk-based prioritization.
//!
//! A [`AuditFinding`] is the central unit of insight produced by the engine.
//! Each finding carries three orthogonal risk dimensions:
//!
//! - [`Severity`] — how damaging the finding is if it is real;
//! - [`Likelihood`] — how confident we are that it represents a genuine issue;
//! - [`BlastRadius`] — how widely the issue is spread across subjects/statutes.
//!
//! The [`FindingPrioritizer`] folds these three dimensions into a single
//! priority score using the classic multiplicative "severity x likelihood x
//! blast radius" risk model, expressed here as a configurable weighted
//! geometric mean so that a finding only ranks highly when *all* contributing
//! dimensions are elevated. Each scored finding is assigned a [`PriorityTier`]
//! so downstream consumers can triage without re-deriving thresholds.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// The category of an audit finding.
///
/// Variants map one-to-one onto the detectors that produce them and onto the
/// keys of the remediation catalogue, so adding a new detector means adding a
/// new variant here and a matching remediation template.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FindingKind {
    /// Decision volume in a time bucket is a statistically high outlier.
    VolumeSpike,
    /// Decision volume in a time bucket is a statistically low outlier.
    VolumeDrop,
    /// A single category's relative frequency spiked within a bucket.
    FrequencySpike,
    /// A rarely-occurring event category was observed.
    RareEvent,
    /// A baselined metric drifted away from its established centre.
    BaselineDrift,
    /// The observed outcome distribution drifted from the learned model.
    OutcomeDrift,
    /// A highly improbable outcome transition was observed in a sequence.
    ImprobableTransition,
    /// An elevated clustering of human overrides was detected.
    OverrideCluster,
    /// An elevated rate of void / error decisions was detected.
    ElevatedVoidRate,
    /// A hash-chain / integrity risk indicator was raised.
    IntegrityRisk,
    /// A bespoke finding kind keyed by a stable identifier.
    Custom(String),
}

impl FindingKind {
    /// Returns a stable, human-readable label for the kind.
    pub fn label(&self) -> String {
        match self {
            FindingKind::VolumeSpike => "volume_spike".to_string(),
            FindingKind::VolumeDrop => "volume_drop".to_string(),
            FindingKind::FrequencySpike => "frequency_spike".to_string(),
            FindingKind::RareEvent => "rare_event".to_string(),
            FindingKind::BaselineDrift => "baseline_drift".to_string(),
            FindingKind::OutcomeDrift => "outcome_drift".to_string(),
            FindingKind::ImprobableTransition => "improbable_transition".to_string(),
            FindingKind::OverrideCluster => "override_cluster".to_string(),
            FindingKind::ElevatedVoidRate => "elevated_void_rate".to_string(),
            FindingKind::IntegrityRisk => "integrity_risk".to_string(),
            FindingKind::Custom(name) => format!("custom:{name}"),
        }
    }
}

/// How damaging a finding is if it reflects a genuine problem.
///
/// Variants are declared in ascending order so the derived [`Ord`] orders
/// `Info < Low < Medium < High < Critical`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Severity {
    /// Informational; no action required.
    Info,
    /// Minor impact.
    Low,
    /// Moderate impact.
    Medium,
    /// Significant impact.
    High,
    /// Severe impact demanding immediate attention.
    Critical,
}

impl Severity {
    /// Returns the ordinal level in `1..=5`.
    pub fn level(self) -> u32 {
        match self {
            Severity::Info => 1,
            Severity::Low => 2,
            Severity::Medium => 3,
            Severity::High => 4,
            Severity::Critical => 5,
        }
    }

    /// Returns the level normalised onto `(0, 1]`.
    pub fn normalized(self) -> f64 {
        self.level() as f64 / 5.0
    }
}

/// How confident we are that the finding is a genuine issue (not noise).
///
/// Declared ascending so the derived [`Ord`] orders
/// `Rare < Unlikely < Possible < Likely < AlmostCertain`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Likelihood {
    /// Almost certainly a false positive.
    Rare,
    /// Probably noise.
    Unlikely,
    /// Could go either way.
    Possible,
    /// Probably a genuine issue.
    Likely,
    /// Almost certainly a genuine issue.
    AlmostCertain,
}

impl Likelihood {
    /// Returns the ordinal level in `1..=5`.
    pub fn level(self) -> u32 {
        match self {
            Likelihood::Rare => 1,
            Likelihood::Unlikely => 2,
            Likelihood::Possible => 3,
            Likelihood::Likely => 4,
            Likelihood::AlmostCertain => 5,
        }
    }

    /// Returns the level normalised onto `(0, 1]`.
    pub fn normalized(self) -> f64 {
        self.level() as f64 / 5.0
    }

    /// Maps a continuous confidence score in `[0, 1]` onto a likelihood band.
    pub fn from_confidence(confidence: f64) -> Self {
        let clamped = confidence.clamp(0.0, 1.0);
        if clamped >= 0.85 {
            Likelihood::AlmostCertain
        } else if clamped >= 0.65 {
            Likelihood::Likely
        } else if clamped >= 0.4 {
            Likelihood::Possible
        } else if clamped >= 0.2 {
            Likelihood::Unlikely
        } else {
            Likelihood::Rare
        }
    }
}

/// The breadth of impact of a finding across the audit population.
///
/// Declared ascending so the derived [`Ord`] orders
/// `Isolated < Localized < Widespread < Systemic`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ImpactScope {
    /// A single (or near-single) subject is affected.
    Isolated,
    /// A small cluster of subjects is affected.
    Localized,
    /// Many subjects or multiple statutes are affected.
    Widespread,
    /// The issue spans the system broadly.
    Systemic,
}

impl ImpactScope {
    /// Returns the ordinal level in `1..=4`.
    pub fn level(self) -> u32 {
        match self {
            ImpactScope::Isolated => 1,
            ImpactScope::Localized => 2,
            ImpactScope::Widespread => 3,
            ImpactScope::Systemic => 4,
        }
    }

    /// Returns the level normalised onto `(0, 1]`.
    pub fn normalized(self) -> f64 {
        self.level() as f64 / 4.0
    }
}

/// Quantifies how widely a finding spreads across the audited population.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlastRadius {
    /// Number of audit records implicated.
    pub affected_records: usize,
    /// Number of distinct subjects implicated.
    pub affected_subjects: usize,
    /// Number of distinct statutes implicated.
    pub affected_statutes: usize,
    /// Derived qualitative scope.
    pub scope: ImpactScope,
}

impl BlastRadius {
    /// Builds a blast radius from raw counts, deriving the qualitative
    /// [`ImpactScope`] using fixed escalation thresholds.
    pub fn from_counts(
        affected_records: usize,
        affected_subjects: usize,
        affected_statutes: usize,
    ) -> Self {
        let scope = Self::classify(affected_subjects, affected_statutes);
        Self {
            affected_records,
            affected_subjects,
            affected_statutes,
            scope,
        }
    }

    /// Convenience constructor for a finding confined to a single subject.
    pub fn isolated(affected_records: usize) -> Self {
        Self {
            affected_records,
            affected_subjects: 1,
            affected_statutes: 1,
            scope: ImpactScope::Isolated,
        }
    }

    fn classify(affected_subjects: usize, affected_statutes: usize) -> ImpactScope {
        if affected_statutes >= 5 || affected_subjects >= 100 {
            ImpactScope::Systemic
        } else if affected_statutes >= 2 || affected_subjects >= 20 {
            ImpactScope::Widespread
        } else if affected_subjects >= 5 {
            ImpactScope::Localized
        } else {
            ImpactScope::Isolated
        }
    }
}

/// A single insight surfaced by the engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditFinding {
    /// Stable identifier for this finding.
    pub id: Uuid,
    /// The category of the finding.
    pub kind: FindingKind,
    /// Short, human-readable headline.
    pub title: String,
    /// Detailed explanation of what was detected and why it matters.
    pub description: String,
    /// How damaging the finding is if genuine.
    pub severity: Severity,
    /// How confident we are the finding is genuine.
    pub likelihood: Likelihood,
    /// How widely the finding spreads.
    pub blast_radius: BlastRadius,
    /// Identifiers of the audit records that constitute the evidence.
    pub evidence: Vec<Uuid>,
    /// Supporting numeric metrics (observed value, expected value, scores, ...).
    pub metrics: HashMap<String, f64>,
    /// When the finding was produced.
    pub detected_at: DateTime<Utc>,
}

impl AuditFinding {
    /// Creates a new finding with the three required risk dimensions.
    pub fn new(
        kind: FindingKind,
        title: impl Into<String>,
        severity: Severity,
        likelihood: Likelihood,
        blast_radius: BlastRadius,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            kind,
            title: title.into(),
            description: String::new(),
            severity,
            likelihood,
            blast_radius,
            evidence: Vec::new(),
            metrics: HashMap::new(),
            detected_at: Utc::now(),
        }
    }

    /// Sets the detailed description (builder style).
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Attaches evidence record identifiers (builder style).
    pub fn with_evidence(mut self, evidence: Vec<Uuid>) -> Self {
        self.evidence = evidence;
        self
    }

    /// Records a supporting metric (builder style).
    pub fn with_metric(mut self, name: impl Into<String>, value: f64) -> Self {
        self.metrics.insert(name.into(), value);
        self
    }
}

/// The triage tier assigned to a scored finding.
///
/// Declared descending in urgency so the derived [`Ord`] keeps
/// `Backlog < Low < Medium < High < Urgent`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum PriorityTier {
    /// Defer; revisit during routine review.
    Backlog,
    /// Address opportunistically.
    Low,
    /// Schedule for the current cycle.
    Medium,
    /// Address promptly.
    High,
    /// Address immediately.
    Urgent,
}

/// The full result of scoring a finding.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PriorityScore {
    /// Composite priority in `[0, 1]`.
    pub value: f64,
    /// The triage tier derived from `value`.
    pub tier: PriorityTier,
    /// Normalised severity contribution.
    pub severity_component: f64,
    /// Normalised likelihood contribution.
    pub likelihood_component: f64,
    /// Normalised blast-radius contribution.
    pub blast_component: f64,
}

/// Configuration for the [`FindingPrioritizer`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrioritizationConfig {
    /// Exponent weight applied to the severity factor.
    pub severity_weight: f64,
    /// Exponent weight applied to the likelihood factor.
    pub likelihood_weight: f64,
    /// Exponent weight applied to the blast-radius factor.
    pub blast_weight: f64,
    /// Lower bound (inclusive) of the [`PriorityTier::Urgent`] band.
    pub urgent_threshold: f64,
    /// Lower bound (inclusive) of the [`PriorityTier::High`] band.
    pub high_threshold: f64,
    /// Lower bound (inclusive) of the [`PriorityTier::Medium`] band.
    pub medium_threshold: f64,
    /// Lower bound (inclusive) of the [`PriorityTier::Low`] band.
    pub low_threshold: f64,
}

impl Default for PrioritizationConfig {
    fn default() -> Self {
        Self {
            severity_weight: 1.0,
            likelihood_weight: 1.0,
            blast_weight: 1.0,
            urgent_threshold: 0.72,
            high_threshold: 0.55,
            medium_threshold: 0.38,
            low_threshold: 0.22,
        }
    }
}

/// A finding paired with its computed priority score.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrioritizedFinding {
    /// The underlying finding.
    pub finding: AuditFinding,
    /// Its computed priority.
    pub score: PriorityScore,
}

/// Scores and ranks findings by combined risk.
#[derive(Debug, Clone)]
pub struct FindingPrioritizer {
    config: PrioritizationConfig,
}

impl FindingPrioritizer {
    /// Creates a prioritizer with the default configuration.
    pub fn new() -> Self {
        Self::with_config(PrioritizationConfig::default())
    }

    /// Creates a prioritizer with a custom configuration.
    pub fn with_config(config: PrioritizationConfig) -> Self {
        Self { config }
    }

    /// Returns the active configuration.
    pub fn config(&self) -> &PrioritizationConfig {
        &self.config
    }

    /// Computes the priority score of a single finding.
    ///
    /// The composite is a weighted geometric mean of the three normalised
    /// dimensions. The geometric mean enforces an implicit logical "and":
    /// a finding ranks highly only when severity, likelihood, and blast radius
    /// are *jointly* elevated, which is the intended semantics of the classic
    /// "severity x likelihood x blast radius" risk product.
    pub fn score(&self, finding: &AuditFinding) -> PriorityScore {
        let severity = finding.severity.normalized();
        let likelihood = finding.likelihood.normalized();
        let blast = finding.blast_radius.scope.normalized();

        let w_sum =
            self.config.severity_weight + self.config.likelihood_weight + self.config.blast_weight;

        let value = if w_sum <= 0.0 {
            // Degenerate weighting falls back to an unweighted geometric mean.
            (severity * likelihood * blast).cbrt()
        } else {
            let log_mean = (self.config.severity_weight * severity.ln()
                + self.config.likelihood_weight * likelihood.ln()
                + self.config.blast_weight * blast.ln())
                / w_sum;
            log_mean.exp()
        };

        let value = value.clamp(0.0, 1.0);
        let tier = self.tier_for(value);

        PriorityScore {
            value,
            tier,
            severity_component: severity,
            likelihood_component: likelihood,
            blast_component: blast,
        }
    }

    fn tier_for(&self, value: f64) -> PriorityTier {
        if value >= self.config.urgent_threshold {
            PriorityTier::Urgent
        } else if value >= self.config.high_threshold {
            PriorityTier::High
        } else if value >= self.config.medium_threshold {
            PriorityTier::Medium
        } else if value >= self.config.low_threshold {
            PriorityTier::Low
        } else {
            PriorityTier::Backlog
        }
    }

    /// Scores every finding and returns them sorted by descending priority.
    ///
    /// Ties are broken by descending severity and then by a larger affected
    /// record count, giving deterministic ordering for equal scores.
    pub fn prioritize(&self, findings: Vec<AuditFinding>) -> Vec<PrioritizedFinding> {
        let mut scored: Vec<PrioritizedFinding> = findings
            .into_iter()
            .map(|finding| {
                let score = self.score(&finding);
                PrioritizedFinding { finding, score }
            })
            .collect();

        scored.sort_by(|a, b| {
            b.score
                .value
                .total_cmp(&a.score.value)
                .then(b.finding.severity.cmp(&a.finding.severity))
                .then(
                    b.finding
                        .blast_radius
                        .affected_records
                        .cmp(&a.finding.blast_radius.affected_records),
                )
        });

        scored
    }
}

impl Default for FindingPrioritizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn finding(severity: Severity, likelihood: Likelihood, scope: BlastRadius) -> AuditFinding {
        AuditFinding::new(
            FindingKind::VolumeSpike,
            "test",
            severity,
            likelihood,
            scope,
        )
    }

    #[test]
    fn test_severity_and_likelihood_ordering() {
        assert!(Severity::Critical > Severity::Info);
        assert!(Severity::High > Severity::Medium);
        assert!(Likelihood::AlmostCertain > Likelihood::Rare);
        assert_eq!(Severity::Medium.level(), 3);
        assert!((Severity::Critical.normalized() - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_likelihood_from_confidence() {
        assert_eq!(Likelihood::from_confidence(0.95), Likelihood::AlmostCertain);
        assert_eq!(Likelihood::from_confidence(0.7), Likelihood::Likely);
        assert_eq!(Likelihood::from_confidence(0.5), Likelihood::Possible);
        assert_eq!(Likelihood::from_confidence(0.25), Likelihood::Unlikely);
        assert_eq!(Likelihood::from_confidence(0.05), Likelihood::Rare);
        // Out-of-range inputs are clamped, never panic.
        assert_eq!(Likelihood::from_confidence(5.0), Likelihood::AlmostCertain);
        assert_eq!(Likelihood::from_confidence(-1.0), Likelihood::Rare);
    }

    #[test]
    fn test_blast_radius_classification() {
        assert_eq!(
            BlastRadius::from_counts(1, 1, 1).scope,
            ImpactScope::Isolated
        );
        assert_eq!(
            BlastRadius::from_counts(10, 8, 1).scope,
            ImpactScope::Localized
        );
        assert_eq!(
            BlastRadius::from_counts(50, 30, 1).scope,
            ImpactScope::Widespread
        );
        assert_eq!(
            BlastRadius::from_counts(50, 5, 3).scope,
            ImpactScope::Widespread
        );
        assert_eq!(
            BlastRadius::from_counts(500, 200, 9).scope,
            ImpactScope::Systemic
        );
    }

    #[test]
    fn test_priority_score_monotonicity() {
        let prioritizer = FindingPrioritizer::new();

        let low = finding(
            Severity::Low,
            Likelihood::Unlikely,
            BlastRadius::isolated(1),
        );
        let high = finding(
            Severity::Critical,
            Likelihood::AlmostCertain,
            BlastRadius::from_counts(500, 200, 9),
        );

        let low_score = prioritizer.score(&low);
        let high_score = prioritizer.score(&high);

        assert!(high_score.value > low_score.value);
        assert_eq!(high_score.tier, PriorityTier::Urgent);
        assert!(low_score.value >= 0.0 && low_score.value <= 1.0);
    }

    #[test]
    fn test_geometric_mean_requires_all_dimensions() {
        let prioritizer = FindingPrioritizer::new();

        // High severity but trivial likelihood/blast must NOT be urgent: the
        // geometric mean pulls the composite down toward the weakest factor.
        let lopsided = finding(
            Severity::Critical,
            Likelihood::Rare,
            BlastRadius::isolated(1),
        );
        let score = prioritizer.score(&lopsided);
        assert!(score.tier < PriorityTier::Urgent);
    }

    #[test]
    fn test_prioritize_sorts_descending() {
        let prioritizer = FindingPrioritizer::new();
        let findings = vec![
            finding(
                Severity::Low,
                Likelihood::Unlikely,
                BlastRadius::isolated(1),
            ),
            finding(
                Severity::Critical,
                Likelihood::AlmostCertain,
                BlastRadius::from_counts(300, 150, 7),
            ),
            finding(
                Severity::Medium,
                Likelihood::Possible,
                BlastRadius::from_counts(30, 22, 2),
            ),
        ];

        let ranked = prioritizer.prioritize(findings);
        assert_eq!(ranked.len(), 3);
        assert!(ranked[0].score.value >= ranked[1].score.value);
        assert!(ranked[1].score.value >= ranked[2].score.value);
        assert_eq!(ranked[0].finding.severity, Severity::Critical);
    }

    #[test]
    fn test_zero_weights_fall_back_to_unweighted() {
        let config = PrioritizationConfig {
            severity_weight: 0.0,
            likelihood_weight: 0.0,
            blast_weight: 0.0,
            ..Default::default()
        };
        let prioritizer = FindingPrioritizer::with_config(config);
        let f = finding(
            Severity::High,
            Likelihood::Likely,
            BlastRadius::from_counts(10, 8, 1),
        );
        let score = prioritizer.score(&f);
        assert!(score.value > 0.0 && score.value <= 1.0);
    }
}
