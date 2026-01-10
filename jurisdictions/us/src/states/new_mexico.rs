//! New Mexico State Law
//!
//! New Mexico tort law features:
//! - Pure Comparative Negligence
//! - Tenth Circuit federal appeals jurisdiction
//! - Progressive tort principles
//! - Proportionate liability approach

use crate::states::types::{
    LegalTopic, LegalTradition, StateId, StateLawVariation, StateRule, StatuteReference,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// New Mexico state law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewMexicoLaw;

impl NewMexicoLaw {
    /// Get New Mexico state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::new("NM", "New Mexico", LegalTradition::CommonLaw)
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
            StatuteReference::new("N.M. Stat. Ann. § 41-3A-1")
                .with_title("New Mexico Comparative Negligence Act")
                .with_year(1981),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1981, 1, 1).unwrap())
        .with_notes(
            "New Mexico adopted pure comparative negligence in 1981. A plaintiff's \
             recovery is reduced by their percentage of fault, but not barred \
             entirely, regardless of fault percentage.",
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
            StatuteReference::new("N.M. Stat. Ann. § 41-3A-1")
                .with_title("New Mexico Several Liability")
                .with_year(1987),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1987, 1, 1).unwrap())
        .with_notes(
            "New Mexico abolished joint and several liability in 1987. Each defendant \
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
    fn test_new_mexico_state_id() {
        let nm = NewMexicoLaw::state_id();
        assert_eq!(nm.code, "NM");
        assert_eq!(nm.name, "New Mexico");
    }

    #[test]
    fn test_pure_comparative_negligence() {
        let comp_neg = NewMexicoLaw::comparative_negligence();
        assert_eq!(comp_neg.rule, StateRule::PureComparativeNegligence);
        assert_eq!(comp_neg.state.code, "NM");
        assert!(comp_neg.statutory_basis.is_some());
    }

    #[test]
    fn test_several_liability_only() {
        let joint_several = NewMexicoLaw::joint_and_several_liability();
        assert_eq!(joint_several.rule, StateRule::SeveralLiabilityOnly);
        assert!(joint_several.statutory_basis.is_some());
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = NewMexicoLaw::state_variations();
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
