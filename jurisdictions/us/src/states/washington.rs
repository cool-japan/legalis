//! Washington State Law
//!
//! Washington tort law features:
//! - Pure Comparative Negligence
//! - Pacific Northwest legal leader
//! - Ninth Circuit federal appeals jurisdiction
//! - Progressive consumer protection

use crate::states::types::{
    LegalTopic, LegalTradition, StateId, StateLawVariation, StateRule, StatuteReference,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Washington state law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WashingtonLaw;

impl WashingtonLaw {
    /// Get Washington state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::new("WA", "Washington", LegalTradition::CommonLaw)
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
            StatuteReference::new("Wash. Rev. Code § 4.22.005")
                .with_title("Washington Comparative Fault")
                .with_year(1981),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1981, 7, 26).unwrap())
        .with_notes(
            "Washington adopted pure comparative negligence in 1981. Plaintiff's damages \
             are reduced by their percentage of fault, with no bar to recovery regardless \
             of fault percentage.",
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
            StatuteReference::new("Wash. Rev. Code § 4.22.070")
                .with_title("Washington Joint and Several Liability Reform")
                .with_year(1986),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1986, 3, 12).unwrap())
        .with_notes(
            "Washington abolished joint and several liability in 1986. Each defendant is \
             liable only for their proportionate share of damages, except for \
             intentional torts and hazardous waste.",
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
    fn test_washington_state_id() {
        let wa = WashingtonLaw::state_id();
        assert_eq!(wa.code, "WA");
        assert_eq!(wa.name, "Washington");
    }

    #[test]
    fn test_pure_comparative_negligence() {
        let comp_neg = WashingtonLaw::comparative_negligence();
        assert_eq!(comp_neg.rule, StateRule::PureComparativeNegligence);
        assert_eq!(comp_neg.state.code, "WA");
        assert!(comp_neg.statutory_basis.is_some());
        let statute = comp_neg.statutory_basis.as_ref().unwrap();
        assert_eq!(statute.citation, "Wash. Rev. Code § 4.22.005");
    }

    #[test]
    fn test_several_liability_only() {
        let joint_several = WashingtonLaw::joint_and_several_liability();
        assert_eq!(joint_several.rule, StateRule::SeveralLiabilityOnly);
        assert!(joint_several.statutory_basis.is_some());
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = WashingtonLaw::state_variations();
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
