//! New Hampshire State Law
//!
//! New Hampshire tort law features:
//! - Modified Comparative Negligence (51% bar)
//! - First Circuit federal appeals jurisdiction
//! - New England legal tradition
//! - Conservative approach to liability

use crate::states::types::{
    LegalTopic, LegalTradition, StateId, StateLawVariation, StateRule, StatuteReference,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// New Hampshire state law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewHampshireLaw;

impl NewHampshireLaw {
    /// Get New Hampshire state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::new("NH", "New Hampshire", LegalTradition::CommonLaw)
    }

    /// Get comparative negligence variation.
    #[must_use]
    pub fn comparative_negligence() -> StateLawVariation {
        StateLawVariation::new(
            Self::state_id(),
            LegalTopic::ComparativeNegligence,
            StateRule::ModifiedComparative51,
        )
        .with_statute(
            StatuteReference::new("N.H. Rev. Stat. Ann. § 507:7-d")
                .with_title("New Hampshire Comparative Fault")
                .with_year(1969),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1969, 1, 1).unwrap())
        .with_notes(
            "New Hampshire adopted modified comparative negligence with 51% bar in 1969. \
             Plaintiff's recovery is barred if their negligence is greater than the \
             negligence of the defendant.",
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
            StatuteReference::new("N.H. Rev. Stat. Ann. § 507:7-e")
                .with_title("New Hampshire Several Liability")
                .with_year(1987),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1987, 1, 1).unwrap())
        .with_notes(
            "New Hampshire abolished joint and several liability in 1987. Each defendant \
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
    fn test_new_hampshire_state_id() {
        let nh = NewHampshireLaw::state_id();
        assert_eq!(nh.code, "NH");
        assert_eq!(nh.name, "New Hampshire");
    }

    #[test]
    fn test_modified_comparative_51() {
        let comp_neg = NewHampshireLaw::comparative_negligence();
        assert_eq!(comp_neg.rule, StateRule::ModifiedComparative51);
        assert_eq!(comp_neg.state.code, "NH");
        assert!(comp_neg.statutory_basis.is_some());
    }

    #[test]
    fn test_several_liability_only() {
        let joint_several = NewHampshireLaw::joint_and_several_liability();
        assert_eq!(joint_several.rule, StateRule::SeveralLiabilityOnly);
        assert!(joint_several.statutory_basis.is_some());
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = NewHampshireLaw::state_variations();
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
