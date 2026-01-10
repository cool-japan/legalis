//! Alaska State Law
//!
//! Alaska tort law features:
//! - Pure Comparative Negligence
//! - Ninth Circuit federal appeals jurisdiction
//! - Unique frontier state considerations
//! - Progressive tort reform

use crate::states::types::{
    LegalTopic, LegalTradition, StateId, StateLawVariation, StateRule, StatuteReference,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Alaska state law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlaskaLaw;

impl AlaskaLaw {
    /// Get Alaska state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::new("AK", "Alaska", LegalTradition::CommonLaw)
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
            StatuteReference::new("Alaska Stat. § 09.17.060")
                .with_title("Alaska Comparative Negligence")
                .with_year(1986),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1986, 7, 1).unwrap())
        .with_notes(
            "Alaska adopted pure comparative negligence in 1986. Plaintiff's damages \
             are reduced by their percentage of fault, with no bar to recovery \
             regardless of fault percentage.",
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
            StatuteReference::new("Alaska Stat. § 09.17.080")
                .with_title("Alaska Several Liability")
                .with_year(1986),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1986, 7, 1).unwrap())
        .with_notes(
            "Alaska abolished joint and several liability in 1986. Each defendant \
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
    fn test_alaska_state_id() {
        let ak = AlaskaLaw::state_id();
        assert_eq!(ak.code, "AK");
        assert_eq!(ak.name, "Alaska");
    }

    #[test]
    fn test_pure_comparative_negligence() {
        let comp_neg = AlaskaLaw::comparative_negligence();
        assert_eq!(comp_neg.rule, StateRule::PureComparativeNegligence);
        assert_eq!(comp_neg.state.code, "AK");
        assert!(comp_neg.statutory_basis.is_some());
    }

    #[test]
    fn test_several_liability_only() {
        let joint_several = AlaskaLaw::joint_and_several_liability();
        assert_eq!(joint_several.rule, StateRule::SeveralLiabilityOnly);
        assert!(joint_several.statutory_basis.is_some());
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = AlaskaLaw::state_variations();
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
