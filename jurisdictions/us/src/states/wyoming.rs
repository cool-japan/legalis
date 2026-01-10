//! Wyoming State Law
//!
//! Wyoming tort law features:
//! - Modified Comparative Negligence (51% bar)
//! - Tenth Circuit federal appeals jurisdiction
//! - Conservative tort approach
//! - Business-friendly environment

use crate::states::types::{
    LegalTopic, LegalTradition, StateId, StateLawVariation, StateRule, StatuteReference,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Wyoming state law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WyomingLaw;

impl WyomingLaw {
    /// Get Wyoming state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::new("WY", "Wyoming", LegalTradition::CommonLaw)
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
            StatuteReference::new("Wyo. Stat. Ann. § 1-1-109")
                .with_title("Wyoming Comparative Negligence")
                .with_year(1973),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1973, 1, 1).unwrap())
        .with_notes(
            "Wyoming adopted modified comparative negligence with 51% bar in 1973. \
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
            StatuteReference::new("Wyo. Stat. Ann. § 1-1-109")
                .with_title("Wyoming Several Liability")
                .with_year(1994),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1994, 7, 1).unwrap())
        .with_notes(
            "Wyoming abolished joint and several liability in 1994. Each defendant \
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
    fn test_wyoming_state_id() {
        let wy = WyomingLaw::state_id();
        assert_eq!(wy.code, "WY");
        assert_eq!(wy.name, "Wyoming");
    }

    #[test]
    fn test_modified_comparative_51() {
        let comp_neg = WyomingLaw::comparative_negligence();
        assert_eq!(comp_neg.rule, StateRule::ModifiedComparative51);
        assert_eq!(comp_neg.state.code, "WY");
        assert!(comp_neg.statutory_basis.is_some());
    }

    #[test]
    fn test_several_liability_only() {
        let joint_several = WyomingLaw::joint_and_several_liability();
        assert_eq!(joint_several.rule, StateRule::SeveralLiabilityOnly);
        assert!(joint_several.statutory_basis.is_some());
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = WyomingLaw::state_variations();
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
