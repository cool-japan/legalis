//! Illinois State Law
//!
//! Illinois tort law features:
//! - Modified Comparative Negligence (51% bar)
//! - Joint and Several Liability (modified/abolished)
//! - Third largest state by population
//! - Major legal and financial center (Chicago)

use crate::states::types::{
    LegalTopic, LegalTradition, StateId, StateLawVariation, StateRule, StatuteReference,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Illinois state law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IllinoisLaw;

impl IllinoisLaw {
    /// Get Illinois state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::new("IL", "Illinois", LegalTradition::CommonLaw)
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
            StatuteReference::new("735 ILCS 5/2-1116")
                .with_title("Illinois Comparative Negligence")
                .with_year(1986),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1986, 1, 1).unwrap())
        .with_notes(
            "Illinois adopted modified comparative negligence with 51% bar in 1986. \
             Plaintiff is barred from recovery if their contributory fault is more than \
             50% of the proximate cause of the injury.",
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
            StatuteReference::new("735 ILCS 5/2-1117")
                .with_title("Illinois Joint and Several Liability Reform")
                .with_year(1995),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1995, 3, 9).unwrap())
        .with_notes(
            "Illinois Joint and Several Liability Reform Act (1995) abolished joint and several \
             liability. Each defendant is liable only for their proportionate share, except in \
             cases of medical malpractice and certain environmental violations.",
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
    fn test_illinois_state_id() {
        let il = IllinoisLaw::state_id();
        assert_eq!(il.code, "IL");
        assert_eq!(il.name, "Illinois");
    }

    #[test]
    fn test_modified_comparative_51() {
        let comp_neg = IllinoisLaw::comparative_negligence();
        assert_eq!(comp_neg.rule, StateRule::ModifiedComparative51);
        assert_eq!(comp_neg.state.code, "IL");
        assert!(comp_neg.statutory_basis.is_some());
        let statute = comp_neg.statutory_basis.as_ref().unwrap();
        assert_eq!(statute.citation, "735 ILCS 5/2-1116");
    }

    #[test]
    fn test_several_liability_only() {
        let joint_several = IllinoisLaw::joint_and_several_liability();
        assert_eq!(joint_several.rule, StateRule::SeveralLiabilityOnly);
        assert!(joint_several.statutory_basis.is_some());
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = IllinoisLaw::state_variations();
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
