//! Michigan State Law
//!
//! Michigan tort law features:
//! - Modified Comparative Negligence (51% bar)
//! - Automotive industry center
//! - Sixth Circuit federal appeals jurisdiction
//! - No-fault auto insurance system

use crate::states::types::{
    LegalTopic, LegalTradition, StateId, StateLawVariation, StateRule, StatuteReference,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Michigan state law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MichiganLaw;

impl MichiganLaw {
    /// Get Michigan state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::new("MI", "Michigan", LegalTradition::CommonLaw)
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
            StatuteReference::new("Mich. Comp. Laws § 600.2959")
                .with_title("Michigan Comparative Negligence")
                .with_year(1979),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1979, 7, 1).unwrap())
        .with_notes(
            "Michigan adopted modified comparative negligence with 51% bar in 1979. \
             Contributory negligence does not bar recovery if plaintiff's negligence \
             was not greater than the negligence of the defendant.",
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
            StatuteReference::new("Mich. Comp. Laws § 600.6304")
                .with_title("Michigan Several Liability")
                .with_year(1995),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1995, 3, 28).unwrap())
        .with_notes(
            "Michigan tort reform (1995) abolished joint and several liability for \
             non-economic damages. Each defendant is liable only for their percentage \
             of fault except for economic damages.",
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
    fn test_michigan_state_id() {
        let mi = MichiganLaw::state_id();
        assert_eq!(mi.code, "MI");
        assert_eq!(mi.name, "Michigan");
    }

    #[test]
    fn test_modified_comparative_51() {
        let comp_neg = MichiganLaw::comparative_negligence();
        assert_eq!(comp_neg.rule, StateRule::ModifiedComparative51);
        assert_eq!(comp_neg.state.code, "MI");
        assert!(comp_neg.statutory_basis.is_some());
        let statute = comp_neg.statutory_basis.as_ref().unwrap();
        assert_eq!(statute.citation, "Mich. Comp. Laws § 600.2959");
    }

    #[test]
    fn test_several_liability_only() {
        let joint_several = MichiganLaw::joint_and_several_liability();
        assert_eq!(joint_several.rule, StateRule::SeveralLiabilityOnly);
        assert!(joint_several.statutory_basis.is_some());
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = MichiganLaw::state_variations();
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
