//! Oklahoma State Law
//!
//! Oklahoma tort law features:
//! - Modified Comparative Negligence (51% bar)
//! - Tenth Circuit federal appeals jurisdiction
//! - Tort reform state
//! - Business-friendly environment

use crate::states::types::{
    LegalTopic, LegalTradition, StateId, StateLawVariation, StateRule, StatuteReference,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Oklahoma state law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OklahomaLaw;

impl OklahomaLaw {
    /// Get Oklahoma state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::new("OK", "Oklahoma", LegalTradition::CommonLaw)
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
            StatuteReference::new("23 Okla. Stat. § 13")
                .with_title("Oklahoma Comparative Negligence")
                .with_year(1978),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1978, 1, 1).unwrap())
        .with_notes(
            "Oklahoma adopted modified comparative negligence with 51% bar in 1978. \
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
            StatuteReference::new("23 Okla. Stat. § 15")
                .with_title("Oklahoma Several Liability")
                .with_year(2004),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(2004, 11, 1).unwrap())
        .with_notes(
            "Oklahoma abolished joint and several liability in 2004. Each defendant \
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
    fn test_oklahoma_state_id() {
        let ok = OklahomaLaw::state_id();
        assert_eq!(ok.code, "OK");
        assert_eq!(ok.name, "Oklahoma");
    }

    #[test]
    fn test_modified_comparative_51() {
        let comp_neg = OklahomaLaw::comparative_negligence();
        assert_eq!(comp_neg.rule, StateRule::ModifiedComparative51);
        assert_eq!(comp_neg.state.code, "OK");
        assert!(comp_neg.statutory_basis.is_some());
    }

    #[test]
    fn test_several_liability_only() {
        let joint_several = OklahomaLaw::joint_and_several_liability();
        assert_eq!(joint_several.rule, StateRule::SeveralLiabilityOnly);
        assert!(joint_several.statutory_basis.is_some());
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = OklahomaLaw::state_variations();
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
