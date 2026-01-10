//! Hawaii State Law
//!
//! Hawaii tort law features:
//! - Modified Comparative Negligence (51% bar)
//! - Ninth Circuit federal appeals jurisdiction
//! - Unique island jurisdiction considerations
//! - Progressive consumer protection

use crate::states::types::{
    LegalTopic, LegalTradition, StateId, StateLawVariation, StateRule, StatuteReference,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Hawaii state law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HawaiiLaw;

impl HawaiiLaw {
    /// Get Hawaii state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::new("HI", "Hawaii", LegalTradition::CommonLaw)
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
            StatuteReference::new("Haw. Rev. Stat. § 663-31")
                .with_title("Hawaii Comparative Negligence")
                .with_year(1969),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1969, 1, 1).unwrap())
        .with_notes(
            "Hawaii adopted modified comparative negligence with 51% bar in 1969. \
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
            StatuteReference::new("Haw. Rev. Stat. § 663-10.9")
                .with_title("Hawaii Several Liability")
                .with_year(1986),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1986, 7, 1).unwrap())
        .with_notes(
            "Hawaii abolished joint and several liability in 1986. Each defendant \
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
    fn test_hawaii_state_id() {
        let hi = HawaiiLaw::state_id();
        assert_eq!(hi.code, "HI");
        assert_eq!(hi.name, "Hawaii");
    }

    #[test]
    fn test_modified_comparative_51() {
        let comp_neg = HawaiiLaw::comparative_negligence();
        assert_eq!(comp_neg.rule, StateRule::ModifiedComparative51);
        assert_eq!(comp_neg.state.code, "HI");
        assert!(comp_neg.statutory_basis.is_some());
    }

    #[test]
    fn test_several_liability_only() {
        let joint_several = HawaiiLaw::joint_and_several_liability();
        assert_eq!(joint_several.rule, StateRule::SeveralLiabilityOnly);
        assert!(joint_several.statutory_basis.is_some());
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = HawaiiLaw::state_variations();
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
