//! Utah State Law
//!
//! Utah tort law features:
//! - Modified Comparative Negligence (50% bar)
//! - Tenth Circuit federal appeals jurisdiction
//! - Conservative tort reform
//! - Business-friendly environment

use crate::states::types::{
    LegalTopic, LegalTradition, StateId, StateLawVariation, StateRule, StatuteReference,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Utah state law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UtahLaw;

impl UtahLaw {
    /// Get Utah state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::new("UT", "Utah", LegalTradition::CommonLaw)
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
            StatuteReference::new("Utah Code § 78B-5-818")
                .with_title("Utah Comparative Negligence")
                .with_year(1973),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1973, 1, 1).unwrap())
        .with_notes(
            "Utah adopted modified comparative negligence with 50% bar in 1973. \
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
            StatuteReference::new("Utah Code § 78B-5-823")
                .with_title("Utah Several Liability")
                .with_year(1986),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1986, 5, 5).unwrap())
        .with_notes(
            "Utah abolished joint and several liability in 1986. Each defendant \
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
    fn test_utah_state_id() {
        let ut = UtahLaw::state_id();
        assert_eq!(ut.code, "UT");
        assert_eq!(ut.name, "Utah");
    }

    #[test]
    fn test_modified_comparative_50() {
        let comp_neg = UtahLaw::comparative_negligence();
        assert_eq!(comp_neg.rule, StateRule::ModifiedComparative50);
        assert_eq!(comp_neg.state.code, "UT");
        assert!(comp_neg.statutory_basis.is_some());
    }

    #[test]
    fn test_several_liability_only() {
        let joint_several = UtahLaw::joint_and_several_liability();
        assert_eq!(joint_several.rule, StateRule::SeveralLiabilityOnly);
        assert!(joint_several.statutory_basis.is_some());
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = UtahLaw::state_variations();
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
