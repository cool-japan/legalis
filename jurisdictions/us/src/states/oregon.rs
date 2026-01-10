//! Oregon State Law
//!
//! Oregon tort law features:
//! - Modified Comparative Negligence (51% bar)
//! - Ninth Circuit federal appeals jurisdiction
//! - Progressive tort principles
//! - Consumer protection focus

use crate::states::types::{
    LegalTopic, LegalTradition, StateId, StateLawVariation, StateRule, StatuteReference,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Oregon state law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OregonLaw;

impl OregonLaw {
    /// Get Oregon state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::new("OR", "Oregon", LegalTradition::CommonLaw)
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
            StatuteReference::new("Or. Rev. Stat. § 18.470")
                .with_title("Oregon Comparative Negligence")
                .with_year(1975),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1975, 1, 1).unwrap())
        .with_notes(
            "Oregon adopted modified comparative negligence with 51% bar in 1975. \
             Plaintiff's recovery is barred if their negligence is greater than the \
             combined negligence of all defendants.",
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
            StatuteReference::new("Or. Rev. Stat. § 18.485")
                .with_title("Oregon Several Liability")
                .with_year(1987),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1987, 1, 1).unwrap())
        .with_notes(
            "Oregon abolished joint and several liability in 1987. Each defendant \
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
    fn test_oregon_state_id() {
        let or = OregonLaw::state_id();
        assert_eq!(or.code, "OR");
        assert_eq!(or.name, "Oregon");
    }

    #[test]
    fn test_modified_comparative_51() {
        let comp_neg = OregonLaw::comparative_negligence();
        assert_eq!(comp_neg.rule, StateRule::ModifiedComparative51);
        assert_eq!(comp_neg.state.code, "OR");
        assert!(comp_neg.statutory_basis.is_some());
    }

    #[test]
    fn test_several_liability_only() {
        let joint_several = OregonLaw::joint_and_several_liability();
        assert_eq!(joint_several.rule, StateRule::SeveralLiabilityOnly);
        assert!(joint_several.statutory_basis.is_some());
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = OregonLaw::state_variations();
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
