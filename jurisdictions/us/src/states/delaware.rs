//! Delaware State Law
//!
//! Delaware tort law features:
//! - Modified Comparative Negligence (51% bar)
//! - Third Circuit federal appeals jurisdiction
//! - Corporate law leadership (Chancery Court)
//! - Progressive tort principles

use crate::states::types::{
    LegalTopic, LegalTradition, StateId, StateLawVariation, StateRule, StatuteReference,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Delaware state law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelawareLaw;

impl DelawareLaw {
    /// Get Delaware state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::new("DE", "Delaware", LegalTradition::CommonLaw)
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
            StatuteReference::new("10 Del. C. § 8132")
                .with_title("Delaware Comparative Negligence")
                .with_year(1995),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1995, 7, 11).unwrap())
        .with_notes(
            "Delaware adopted modified comparative negligence with 51% bar in 1995. \
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
            StatuteReference::new("10 Del. C. § 8133")
                .with_title("Delaware Several Liability")
                .with_year(1995),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1995, 7, 11).unwrap())
        .with_notes(
            "Delaware abolished joint and several liability in 1995. Each defendant \
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
    fn test_delaware_state_id() {
        let de = DelawareLaw::state_id();
        assert_eq!(de.code, "DE");
        assert_eq!(de.name, "Delaware");
    }

    #[test]
    fn test_modified_comparative_51() {
        let comp_neg = DelawareLaw::comparative_negligence();
        assert_eq!(comp_neg.rule, StateRule::ModifiedComparative51);
        assert_eq!(comp_neg.state.code, "DE");
        assert!(comp_neg.statutory_basis.is_some());
    }

    #[test]
    fn test_several_liability_only() {
        let joint_several = DelawareLaw::joint_and_several_liability();
        assert_eq!(joint_several.rule, StateRule::SeveralLiabilityOnly);
        assert!(joint_several.statutory_basis.is_some());
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = DelawareLaw::state_variations();
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
