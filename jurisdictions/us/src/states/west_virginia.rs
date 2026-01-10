//! West Virginia State Law
//!
//! West Virginia tort law features:
//! - Modified Comparative Negligence (51% bar)
//! - Fourth Circuit federal appeals jurisdiction
//! - Traditional common law approach
//! - Conservative liability standards

use crate::states::types::{
    LegalTopic, LegalTradition, StateId, StateLawVariation, StateRule, StatuteReference,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// West Virginia state law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WestVirginiaLaw;

impl WestVirginiaLaw {
    /// Get West Virginia state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::new("WV", "West Virginia", LegalTradition::CommonLaw)
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
            StatuteReference::new("W. Va. Code § 55-7-13")
                .with_title("West Virginia Comparative Negligence")
                .with_year(1986),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1986, 1, 1).unwrap())
        .with_notes(
            "West Virginia adopted modified comparative negligence with 51% bar in 1986. \
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
            StatuteReference::new("W. Va. Code § 55-7-24")
                .with_title("West Virginia Several Liability")
                .with_year(1986),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1986, 1, 1).unwrap())
        .with_notes(
            "West Virginia abolished joint and several liability in 1986. Each defendant \
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
    fn test_west_virginia_state_id() {
        let wv = WestVirginiaLaw::state_id();
        assert_eq!(wv.code, "WV");
        assert_eq!(wv.name, "West Virginia");
    }

    #[test]
    fn test_modified_comparative_51() {
        let comp_neg = WestVirginiaLaw::comparative_negligence();
        assert_eq!(comp_neg.rule, StateRule::ModifiedComparative51);
        assert_eq!(comp_neg.state.code, "WV");
        assert!(comp_neg.statutory_basis.is_some());
    }

    #[test]
    fn test_several_liability_only() {
        let joint_several = WestVirginiaLaw::joint_and_several_liability();
        assert_eq!(joint_several.rule, StateRule::SeveralLiabilityOnly);
        assert!(joint_several.statutory_basis.is_some());
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = WestVirginiaLaw::state_variations();
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
