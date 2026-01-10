//! Montana State Law
//!
//! Montana tort law features:
//! - Modified Comparative Negligence (51% bar)
//! - Ninth Circuit federal appeals jurisdiction
//! - Mountain West region
//! - Progressive consumer protection

use crate::states::types::{
    LegalTopic, LegalTradition, StateId, StateLawVariation, StateRule, StatuteReference,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Montana state law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MontanaLaw;

impl MontanaLaw {
    /// Get Montana state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::new("MT", "Montana", LegalTradition::CommonLaw)
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
            StatuteReference::new("Mont. Code Ann. § 27-1-702")
                .with_title("Montana Comparative Negligence")
                .with_year(1975),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1975, 7, 1).unwrap())
        .with_notes(
            "Montana adopted modified comparative negligence with 51% bar in 1975. \
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
            StatuteReference::new("Mont. Code Ann. § 27-1-703")
                .with_title("Montana Several Liability")
                .with_year(1987),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1987, 7, 1).unwrap())
        .with_notes(
            "Montana abolished joint and several liability in 1987. Each defendant \
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
    fn test_montana_state_id() {
        let mt = MontanaLaw::state_id();
        assert_eq!(mt.code, "MT");
        assert_eq!(mt.name, "Montana");
    }

    #[test]
    fn test_modified_comparative_51() {
        let comp_neg = MontanaLaw::comparative_negligence();
        assert_eq!(comp_neg.rule, StateRule::ModifiedComparative51);
        assert_eq!(comp_neg.state.code, "MT");
        assert!(comp_neg.statutory_basis.is_some());
    }

    #[test]
    fn test_several_liability_only() {
        let joint_several = MontanaLaw::joint_and_several_liability();
        assert_eq!(joint_several.rule, StateRule::SeveralLiabilityOnly);
        assert!(joint_several.statutory_basis.is_some());
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = MontanaLaw::state_variations();
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
