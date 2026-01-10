//! Nebraska State Law
//!
//! Nebraska tort law features:
//! - Modified Comparative Negligence (50% bar)
//! - Eighth Circuit federal appeals jurisdiction
//! - Great Plains state
//! - Traditional common law approach

use crate::states::types::{
    LegalTopic, LegalTradition, StateId, StateLawVariation, StateRule, StatuteReference,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Nebraska state law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NebraskaLaw;

impl NebraskaLaw {
    /// Get Nebraska state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::new("NE", "Nebraska", LegalTradition::CommonLaw)
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
            StatuteReference::new("Neb. Rev. Stat. § 25-21,185.09")
                .with_title("Nebraska Comparative Negligence")
                .with_year(1992),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1992, 1, 1).unwrap())
        .with_notes(
            "Nebraska adopted modified comparative negligence with 50% bar in 1992. \
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
            StatuteReference::new("Neb. Rev. Stat. § 25-21,185.10")
                .with_title("Nebraska Several Liability")
                .with_year(1992),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1992, 1, 1).unwrap())
        .with_notes(
            "Nebraska abolished joint and several liability in 1992. Each defendant \
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
    fn test_nebraska_state_id() {
        let ne = NebraskaLaw::state_id();
        assert_eq!(ne.code, "NE");
        assert_eq!(ne.name, "Nebraska");
    }

    #[test]
    fn test_modified_comparative_50() {
        let comp_neg = NebraskaLaw::comparative_negligence();
        assert_eq!(comp_neg.rule, StateRule::ModifiedComparative50);
        assert_eq!(comp_neg.state.code, "NE");
        assert!(comp_neg.statutory_basis.is_some());
    }

    #[test]
    fn test_several_liability_only() {
        let joint_several = NebraskaLaw::joint_and_several_liability();
        assert_eq!(joint_several.rule, StateRule::SeveralLiabilityOnly);
        assert!(joint_several.statutory_basis.is_some());
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = NebraskaLaw::state_variations();
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
