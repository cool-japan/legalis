//! Mississippi State Law
//!
//! Mississippi tort law features:
//! - Pure Comparative Negligence
//! - Fifth Circuit federal appeals jurisdiction
//! - Deep South legal tradition
//! - Tort reform measures

use crate::states::types::{
    LegalTopic, LegalTradition, StateId, StateLawVariation, StateRule, StatuteReference,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Mississippi state law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MississippiLaw;

impl MississippiLaw {
    /// Get Mississippi state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::new("MS", "Mississippi", LegalTradition::CommonLaw)
    }

    /// Get comparative negligence variation.
    #[must_use]
    pub fn comparative_negligence() -> StateLawVariation {
        StateLawVariation::new(
            Self::state_id(),
            LegalTopic::ComparativeNegligence,
            StateRule::PureComparativeNegligence,
        )
        .with_statute(
            StatuteReference::new("Miss. Code Ann. § 11-7-15")
                .with_title("Mississippi Comparative Negligence")
                .with_year(1910),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1910, 1, 1).unwrap())
        .with_notes(
            "Mississippi adopted pure comparative negligence in 1910 (one of the earliest \
             states). Plaintiff's damages are reduced by their percentage of fault, with \
             no bar to recovery regardless of fault percentage.",
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
            StatuteReference::new("Miss. Code Ann. § 85-5-7")
                .with_title("Mississippi Several Liability")
                .with_year(1989),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1989, 7, 1).unwrap())
        .with_notes(
            "Mississippi abolished joint and several liability in 1989. Each defendant \
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
    fn test_mississippi_state_id() {
        let ms = MississippiLaw::state_id();
        assert_eq!(ms.code, "MS");
        assert_eq!(ms.name, "Mississippi");
    }

    #[test]
    fn test_pure_comparative_negligence() {
        let comp_neg = MississippiLaw::comparative_negligence();
        assert_eq!(comp_neg.rule, StateRule::PureComparativeNegligence);
        assert_eq!(comp_neg.state.code, "MS");
        assert!(comp_neg.statutory_basis.is_some());
    }

    #[test]
    fn test_several_liability_only() {
        let joint_several = MississippiLaw::joint_and_several_liability();
        assert_eq!(joint_several.rule, StateRule::SeveralLiabilityOnly);
        assert!(joint_several.statutory_basis.is_some());
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = MississippiLaw::state_variations();
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
