//! Georgia State Law
//!
//! Georgia tort law features:
//! - Modified Comparative Negligence (50% bar)
//! - Joint and several liability (modified)
//! - Southern growth state
//! - Eleventh Circuit federal appeals jurisdiction

use crate::states::types::{
    LegalTopic, LegalTradition, StateId, StateLawVariation, StateRule, StatuteReference,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Georgia state law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeorgiaLaw;

impl GeorgiaLaw {
    /// Get Georgia state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::new("GA", "Georgia", LegalTradition::CommonLaw)
    }

    /// Get comparative negligence variation.
    #[must_use]
    pub fn comparative_negligence() -> StateLawVariation {
        StateLawVariation::new(
            Self::state_id(),
            LegalTopic::ComparativeNegligence,
            StateRule::ModifiedComparative50,
        )
        .with_statute(
            StatuteReference::new("Ga. Code Ann. § 51-12-33")
                .with_title("Georgia Comparative Negligence")
                .with_year(1987),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1987, 7, 1).unwrap())
        .with_notes(
            "Georgia adopted modified comparative negligence with 50% bar in 1987. \
             Plaintiff's recovery is barred if their fault is equal to or greater than \
             50% (50% bar rule, not 51%).",
        )
    }

    /// Joint and several liability variation.
    #[must_use]
    pub fn joint_and_several_liability() -> StateLawVariation {
        StateLawVariation::new(
            Self::state_id(),
            LegalTopic::JointAndSeveralLiability,
            StateRule::JointAndSeveralLiability,
        )
        .with_statute(
            StatuteReference::new("Ga. Code Ann. § 51-12-31")
                .with_title("Georgia Joint and Several Liability")
                .with_year(1987),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1987, 7, 1).unwrap())
        .with_notes(
            "Georgia retains traditional joint and several liability. Multiple tortfeasors \
             who act in concert or contribute to an indivisible injury are jointly and \
             severally liable.",
        )
    }

    /// Get all state law variations.
    #[must_use]
    pub fn state_variations() -> Vec<StateLawVariation> {
        vec![
            Self::comparative_negligence(),
            Self::joint_and_several_liability(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_georgia_state_id() {
        let ga = GeorgiaLaw::state_id();
        assert_eq!(ga.code, "GA");
        assert_eq!(ga.name, "Georgia");
    }

    #[test]
    fn test_modified_comparative_50() {
        let comp_neg = GeorgiaLaw::comparative_negligence();
        assert_eq!(comp_neg.rule, StateRule::ModifiedComparative50);
        assert_eq!(comp_neg.state.code, "GA");
        assert!(comp_neg.statutory_basis.is_some());
        let statute = comp_neg.statutory_basis.as_ref().unwrap();
        assert_eq!(statute.citation, "Ga. Code Ann. § 51-12-33");
    }

    #[test]
    fn test_joint_and_several_retained() {
        let joint_several = GeorgiaLaw::joint_and_several_liability();
        assert_eq!(joint_several.rule, StateRule::JointAndSeveralLiability);
        assert!(joint_several.statutory_basis.is_some());
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = GeorgiaLaw::state_variations();
        assert_eq!(variations.len(), 2);
        assert!(
            variations
                .iter()
                .any(|v| v.topic == LegalTopic::ComparativeNegligence)
        );
        assert!(
            variations
                .iter()
                .any(|v| v.topic == LegalTopic::JointAndSeveralLiability)
        );
    }
}
