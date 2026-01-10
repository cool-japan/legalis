//! Kansas State Law
//!
//! Kansas tort law features:
//! - Modified Comparative Negligence (50% bar)
//! - Tenth Circuit federal appeals jurisdiction
//! - Great Plains state
//! - Tort reform measures

use crate::states::types::{
    LegalTopic, LegalTradition, StateId, StateLawVariation, StateRule, StatuteReference,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Kansas state law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KansasLaw;

impl KansasLaw {
    /// Get Kansas state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::new("KS", "Kansas", LegalTradition::CommonLaw)
    }

    /// Get comparative negligence variation.
    #[must_use]
    pub fn comparative_negligence() -> StateLawVariation {
        StateLawVariation::new(
            Self::state_id(),
            LegalTopic::ComparativeNegligence,
            StateRule::ModifiedComparative50,
        )
        .with_statute(
            StatuteReference::new("Kan. Stat. Ann. § 60-258a")
                .with_title("Kansas Comparative Negligence")
                .with_year(1974),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1974, 7, 1).unwrap())
        .with_notes(
            "Kansas adopted modified comparative negligence with 50% bar in 1974. \
             Plaintiff's recovery is barred if their fault is as great as or greater \
             than the fault of the defendant (50% bar).",
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
            StatuteReference::new("Kan. Stat. Ann. § 60-258a(d)")
                .with_title("Kansas Several Liability")
                .with_year(1988),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1988, 7, 1).unwrap())
        .with_notes(
            "Kansas abolished joint and several liability in 1988. Each defendant \
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
    fn test_kansas_state_id() {
        let ks = KansasLaw::state_id();
        assert_eq!(ks.code, "KS");
        assert_eq!(ks.name, "Kansas");
    }

    #[test]
    fn test_modified_comparative_50() {
        let comp_neg = KansasLaw::comparative_negligence();
        assert_eq!(comp_neg.rule, StateRule::ModifiedComparative50);
        assert_eq!(comp_neg.state.code, "KS");
        assert!(comp_neg.statutory_basis.is_some());
    }

    #[test]
    fn test_several_liability_only() {
        let joint_several = KansasLaw::joint_and_several_liability();
        assert_eq!(joint_several.rule, StateRule::SeveralLiabilityOnly);
        assert!(joint_several.statutory_basis.is_some());
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = KansasLaw::state_variations();
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
