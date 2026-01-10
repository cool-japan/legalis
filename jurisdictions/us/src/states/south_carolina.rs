//! South Carolina State Law
//!
//! South Carolina tort law features:
//! - Modified Comparative Negligence (51% bar)
//! - Fourth Circuit federal appeals jurisdiction
//! - Conservative tort approach
//! - Traditional common law principles

use crate::states::types::{
    LegalTopic, LegalTradition, StateId, StateLawVariation, StateRule, StatuteReference,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// South Carolina state law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SouthCarolinaLaw;

impl SouthCarolinaLaw {
    /// Get South Carolina state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::new("SC", "South Carolina", LegalTradition::CommonLaw)
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
            StatuteReference::new("S.C. Code Ann. § 15-38-15")
                .with_title("South Carolina Comparative Negligence")
                .with_year(1991),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1991, 1, 1).unwrap())
        .with_notes(
            "South Carolina adopted modified comparative negligence with 51% bar in 1991. \
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
            StatuteReference::new("S.C. Code Ann. § 15-38-15")
                .with_title("South Carolina Several Liability")
                .with_year(2005),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(2005, 6, 7).unwrap())
        .with_notes(
            "South Carolina abolished joint and several liability in 2005. Each defendant \
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
    fn test_south_carolina_state_id() {
        let sc = SouthCarolinaLaw::state_id();
        assert_eq!(sc.code, "SC");
        assert_eq!(sc.name, "South Carolina");
    }

    #[test]
    fn test_modified_comparative_51() {
        let comp_neg = SouthCarolinaLaw::comparative_negligence();
        assert_eq!(comp_neg.rule, StateRule::ModifiedComparative51);
        assert_eq!(comp_neg.state.code, "SC");
        assert!(comp_neg.statutory_basis.is_some());
    }

    #[test]
    fn test_several_liability_only() {
        let joint_several = SouthCarolinaLaw::joint_and_several_liability();
        assert_eq!(joint_several.rule, StateRule::SeveralLiabilityOnly);
        assert!(joint_several.statutory_basis.is_some());
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = SouthCarolinaLaw::state_variations();
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
