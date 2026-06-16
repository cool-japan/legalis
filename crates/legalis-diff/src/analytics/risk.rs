//! Automated risk assessment for statute diffs.
//!
//! This module turns the qualitative impact signals already produced by the
//! diff engine ([`crate::ImpactAssessment`]) plus the raw change set into a
//! single, explainable risk score in `[0, 100]`. The score is a weighted sum of
//! independent risk *factors*, each of which is retained in the result so the
//! assessment can be audited (no opaque black box).
//!
//! The factor weights are deliberately conservative and additive:
//!
//! | Factor                         | Max contribution |
//! |--------------------------------|------------------|
//! | Severity of the change         | 40               |
//! | Outcome (effect) altered       | 20               |
//! | Eligibility altered            | 15               |
//! | Discretion requirement altered | 15               |
//! | Breadth (number of changes)    | 10               |
//!
//! The breadth factor saturates so that a diff touching many provisions cannot,
//! on volume alone, dominate a genuinely breaking change.
//!
//! # Example
//!
//! ```
//! use legalis_core::{Statute, Effect, EffectType};
//! use legalis_diff::diff;
//! use legalis_diff::analytics::{assess_risk, RiskLevel};
//!
//! let old = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
//! let mut new = old.clone();
//! // Flipping a grant to a revocation alters the outcome — high risk.
//! new.effect = Effect::new(EffectType::Revoke, "Benefit revoked");
//!
//! let diff = diff(&old, &new).unwrap();
//! let assessment = assess_risk(&diff);
//! assert!(assessment.score >= 50.0);
//! // Major severity + altered outcome lands in the Moderate band (or higher).
//! assert!(assessment.level >= RiskLevel::Moderate);
//! ```

use crate::{Severity, StatuteDiff};
use serde::{Deserialize, Serialize};

/// Maximum contribution of each risk factor (sums to 100).
const W_SEVERITY: f64 = 40.0;
const W_OUTCOME: f64 = 20.0;
const W_ELIGIBILITY: f64 = 15.0;
const W_DISCRETION: f64 = 15.0;
const W_BREADTH: f64 = 10.0;

/// Number of changes at which the breadth factor reaches its maximum.
const BREADTH_SATURATION: f64 = 10.0;

/// A coarse risk band derived from the numeric score.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RiskLevel {
    /// Score `< 15`: no material risk (cosmetic edits).
    Negligible,
    /// Score `[15, 35)`: low risk.
    Low,
    /// Score `[35, 60)`: moderate risk; review recommended.
    Moderate,
    /// Score `[60, 85)`: high risk; review required.
    High,
    /// Score `>= 85`: critical risk; likely breaking.
    Critical,
}

impl RiskLevel {
    /// Maps a numeric score in `[0, 100]` to a band.
    pub fn from_score(score: f64) -> Self {
        if score >= 85.0 {
            Self::Critical
        } else if score >= 60.0 {
            Self::High
        } else if score >= 35.0 {
            Self::Moderate
        } else if score >= 15.0 {
            Self::Low
        } else {
            Self::Negligible
        }
    }

    /// A short human-readable label.
    pub fn label(&self) -> &'static str {
        match self {
            Self::Negligible => "negligible",
            Self::Low => "low",
            Self::Moderate => "moderate",
            Self::High => "high",
            Self::Critical => "critical",
        }
    }
}

/// One named contribution to the overall risk score.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RiskFactor {
    /// Identifier of the factor (stable, machine-friendly).
    pub name: String,
    /// Points this factor contributed to the score.
    pub contribution: f64,
    /// Human-readable justification.
    pub rationale: String,
}

/// The outcome of a risk assessment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RiskAssessment {
    /// Overall risk score in `[0, 100]`.
    pub score: f64,
    /// Banded risk level.
    pub level: RiskLevel,
    /// The individual factors that produced the score.
    pub factors: Vec<RiskFactor>,
}

impl RiskAssessment {
    /// Whether the assessed risk warrants mandatory human review
    /// ([`RiskLevel::High`] or above).
    pub fn requires_review(&self) -> bool {
        self.level >= RiskLevel::High
    }
}

/// Numeric weight of a severity level, normalised to `[0, 1]`.
fn severity_weight(severity: Severity) -> f64 {
    match severity {
        Severity::None => 0.0,
        Severity::Minor => 0.25,
        Severity::Moderate => 0.5,
        Severity::Major => 0.8,
        Severity::Breaking => 1.0,
    }
}

/// Assesses the risk of a single diff, returning an explainable score.
pub fn assess_risk(diff: &StatuteDiff) -> RiskAssessment {
    let impact = &diff.impact;
    let mut factors = Vec::new();

    // Severity factor.
    let sev_w = severity_weight(impact.severity);
    if sev_w > 0.0 {
        factors.push(RiskFactor {
            name: "severity".to_string(),
            contribution: sev_w * W_SEVERITY,
            rationale: format!("Change severity assessed as {:?}", impact.severity),
        });
    }

    // Outcome factor.
    if impact.affects_outcome {
        factors.push(RiskFactor {
            name: "outcome".to_string(),
            contribution: W_OUTCOME,
            rationale: "The legal effect (outcome) of the statute changed".to_string(),
        });
    }

    // Eligibility factor.
    if impact.affects_eligibility {
        factors.push(RiskFactor {
            name: "eligibility".to_string(),
            contribution: W_ELIGIBILITY,
            rationale: "Eligibility preconditions changed".to_string(),
        });
    }

    // Discretion factor.
    if impact.discretion_changed {
        factors.push(RiskFactor {
            name: "discretion".to_string(),
            contribution: W_DISCRETION,
            rationale: "Human-judgment (discretion) requirements changed".to_string(),
        });
    }

    // Breadth factor — saturating in the number of changes.
    let change_count = diff.changes.len() as f64;
    if change_count > 0.0 {
        let breadth = (change_count / BREADTH_SATURATION).min(1.0);
        factors.push(RiskFactor {
            name: "breadth".to_string(),
            contribution: breadth * W_BREADTH,
            rationale: format!(
                "{} change(s) across the statute ({}% of saturation)",
                diff.changes.len(),
                (breadth * 100.0).round() as u32
            ),
        });
    }

    let score = factors
        .iter()
        .map(|f| f.contribution)
        .sum::<f64>()
        .clamp(0.0, 100.0);

    RiskAssessment {
        score,
        level: RiskLevel::from_score(score),
        factors,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};

    #[test]
    fn test_identical_statutes_negligible() {
        let s = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
        let diff = crate::diff(&s, &s).expect("diff");
        let assessment = assess_risk(&diff);
        assert_eq!(assessment.level, RiskLevel::Negligible);
        assert!(assessment.score < 15.0);
        assert!(!assessment.requires_review());
    }

    #[test]
    fn test_title_only_low() {
        let old = Statute::new("law", "Old", Effect::new(EffectType::Grant, "Benefit"));
        let mut new = old.clone();
        new.title = "New".into();
        let diff = crate::diff(&old, &new).expect("diff");
        let assessment = assess_risk(&diff);
        // A minor title change should be low / negligible, never high.
        assert!(assessment.level < RiskLevel::High);
    }

    #[test]
    fn test_effect_change_scores_moderate_plus() {
        let old = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
        let mut new = old.clone();
        new.effect = Effect::new(EffectType::Revoke, "Benefit revoked");
        let diff = crate::diff(&old, &new).expect("diff");
        let assessment = assess_risk(&diff);
        // Major severity (32) + outcome (20) + breadth (1) ≈ 53 → Moderate band.
        assert!(assessment.score >= 50.0);
        assert!(assessment.level >= RiskLevel::Moderate);
        assert!(assessment.factors.iter().any(|f| f.name == "outcome"));
    }

    #[test]
    fn test_high_risk_requires_review() {
        // Effect change plus added eligibility plus discretion pushes into High.
        let old = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
        let mut new = old.clone();
        new.effect = Effect::new(EffectType::Revoke, "Benefit revoked");
        new.discretion_logic = Some("officer review".into());
        new = new.with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 65,
        });
        let diff = crate::diff(&old, &new).expect("diff");
        let assessment = assess_risk(&diff);
        assert!(assessment.requires_review());
        assert!(assessment.level >= RiskLevel::High);
    }

    #[test]
    fn test_score_bounded() {
        // Pile on every risk factor; score must still be clamped to 100.
        let old = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
        let mut new = old.clone();
        new.title = "Rewritten".into();
        new.effect = Effect::new(EffectType::Revoke, "Revoked");
        new.discretion_logic = Some("officer discretion".into());
        for i in 0..15 {
            new = new.with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: i,
            });
        }
        let diff = crate::diff(&old, &new).expect("diff");
        let assessment = assess_risk(&diff);
        assert!(assessment.score <= 100.0);
        assert!(assessment.score > 80.0);
    }

    #[test]
    fn test_factors_explainable() {
        let old = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
        let mut new = old.clone();
        new.effect = Effect::new(EffectType::Revoke, "Revoked");
        let diff = crate::diff(&old, &new).expect("diff");
        let assessment = assess_risk(&diff);
        // Every factor carries a non-empty rationale and positive contribution.
        for f in &assessment.factors {
            assert!(!f.rationale.is_empty());
            assert!(f.contribution > 0.0);
        }
        // Score equals the sum of contributions (clamped).
        let sum: f64 = assessment.factors.iter().map(|f| f.contribution).sum();
        assert!((assessment.score - sum.min(100.0)).abs() < 1e-9);
    }

    #[test]
    fn test_risk_level_thresholds() {
        assert_eq!(RiskLevel::from_score(0.0), RiskLevel::Negligible);
        assert_eq!(RiskLevel::from_score(14.9), RiskLevel::Negligible);
        assert_eq!(RiskLevel::from_score(15.0), RiskLevel::Low);
        assert_eq!(RiskLevel::from_score(35.0), RiskLevel::Moderate);
        assert_eq!(RiskLevel::from_score(60.0), RiskLevel::High);
        assert_eq!(RiskLevel::from_score(85.0), RiskLevel::Critical);
        assert_eq!(RiskLevel::from_score(100.0), RiskLevel::Critical);
    }

    #[test]
    fn test_level_ordering() {
        assert!(RiskLevel::Critical > RiskLevel::High);
        assert!(RiskLevel::High > RiskLevel::Moderate);
        assert!(RiskLevel::Negligible < RiskLevel::Low);
    }

    #[test]
    fn test_label() {
        assert_eq!(RiskLevel::Critical.label(), "critical");
        assert_eq!(RiskLevel::Negligible.label(), "negligible");
    }
}
