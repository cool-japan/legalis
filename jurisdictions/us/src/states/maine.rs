//! Maine State Law
//!
//! Maine tort law features:
//! - Modified Comparative Negligence (50% bar)
//! - First Circuit federal appeals jurisdiction
//! - New England legal tradition
//! - Traditional common law approach

use crate::states::types::{
    LegalTopic, LegalTradition, StateId, StateLawVariation, StateRule, StatuteReference,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Maine state law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaineLaw;

impl MaineLaw {
    /// Get Maine state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::new("ME", "Maine", LegalTradition::CommonLaw)
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
            StatuteReference::new("14 Me. Rev. Stat. § 156")
                .with_title("Maine Comparative Negligence")
                .with_year(1965),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1965, 1, 1).unwrap())
        .with_notes(
            "Maine adopted modified comparative negligence with 50% bar in 1965. \
             Plaintiff's recovery is barred if their fault is as great as or greater \
             than the combined fault of all defendants (50% bar).",
        )
    }

    /// Joint and several liability variation.
    #[must_use]
    pub fn joint_and_several_liability() -> StateLawVariation {
        StateLawVariation::new(
            Self::state_id(),
            LegalTopic::JointAndSeveralLiability,
            StateRule::SeveralLiabilityOnly,
        )
        .with_statute(
            StatuteReference::new("14 Me. Rev. Stat. § 163")
                .with_title("Maine Several Liability")
                .with_year(1986),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1986, 1, 1).unwrap())
        .with_notes(
            "Maine abolished joint and several liability in 1986. Each defendant \
             is liable only for their proportionate share of damages.",
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
    fn test_maine_state_id() {
        let me = MaineLaw::state_id();
        assert_eq!(me.code, "ME");
        assert_eq!(me.name, "Maine");
    }

    #[test]
    fn test_modified_comparative_50() {
        let comp_neg = MaineLaw::comparative_negligence();
        assert_eq!(comp_neg.rule, StateRule::ModifiedComparative50);
        assert_eq!(comp_neg.state.code, "ME");
        assert!(comp_neg.statutory_basis.is_some());
    }

    #[test]
    fn test_several_liability_only() {
        let joint_several = MaineLaw::joint_and_several_liability();
        assert_eq!(joint_several.rule, StateRule::SeveralLiabilityOnly);
        assert!(joint_several.statutory_basis.is_some());
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = MaineLaw::state_variations();
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
