//! Iowa State Law
//!
//! Iowa tort law features:
//! - Modified Comparative Negligence (51% bar)
//! - Eighth Circuit federal appeals jurisdiction
//! - Midwest agricultural state
//! - Traditional common law approach

use crate::states::types::{
    LegalTopic, LegalTradition, StateId, StateLawVariation, StateRule, StatuteReference,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Iowa state law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IowaLaw;

impl IowaLaw {
    /// Get Iowa state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::new("IA", "Iowa", LegalTradition::CommonLaw)
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
            StatuteReference::new("Iowa Code § 668.3")
                .with_title("Iowa Comparative Fault")
                .with_year(1984),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1984, 7, 1).unwrap())
        .with_notes(
            "Iowa adopted modified comparative negligence with 51% bar in 1984. \
             Plaintiff's recovery is barred if their fault is greater than the \
             combined fault of all defendants.",
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
            StatuteReference::new("Iowa Code § 668.4")
                .with_title("Iowa Several Liability")
                .with_year(1984),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1984, 7, 1).unwrap())
        .with_notes(
            "Iowa abolished joint and several liability in 1984. Each defendant \
             is liable only for their proportionate share of damages based on their \
             percentage of fault.",
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
    fn test_iowa_state_id() {
        let ia = IowaLaw::state_id();
        assert_eq!(ia.code, "IA");
        assert_eq!(ia.name, "Iowa");
    }

    #[test]
    fn test_modified_comparative_51() {
        let comp_neg = IowaLaw::comparative_negligence();
        assert_eq!(comp_neg.rule, StateRule::ModifiedComparative51);
        assert_eq!(comp_neg.state.code, "IA");
        assert!(comp_neg.statutory_basis.is_some());
    }

    #[test]
    fn test_several_liability_only() {
        let joint_several = IowaLaw::joint_and_several_liability();
        assert_eq!(joint_several.rule, StateRule::SeveralLiabilityOnly);
        assert!(joint_several.statutory_basis.is_some());
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = IowaLaw::state_variations();
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
