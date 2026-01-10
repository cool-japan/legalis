//! Arkansas State Law
//!
//! Arkansas tort law features:
//! - Modified Comparative Negligence (50% bar)
//! - Eighth Circuit federal appeals jurisdiction
//! - Southern regional influence
//! - Tort reform measures

use crate::states::types::{
    LegalTopic, LegalTradition, StateId, StateLawVariation, StateRule, StatuteReference,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Arkansas state law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArkansasLaw;

impl ArkansasLaw {
    /// Get Arkansas state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::new("AR", "Arkansas", LegalTradition::CommonLaw)
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
            StatuteReference::new("Ark. Code Ann. § 16-64-122")
                .with_title("Arkansas Comparative Fault")
                .with_year(2003),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(2003, 4, 1).unwrap())
        .with_notes(
            "Arkansas adopted modified comparative negligence with 50% bar in 2003. \
             Plaintiff's recovery is barred if their fault is equal to or greater than \
             the fault of the defendant (50% bar).",
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
            StatuteReference::new("Ark. Code Ann. § 16-55-201")
                .with_title("Arkansas Several Liability")
                .with_year(2003),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(2003, 4, 1).unwrap())
        .with_notes(
            "Arkansas abolished joint and several liability in 2003. Each defendant \
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
    fn test_arkansas_state_id() {
        let ar = ArkansasLaw::state_id();
        assert_eq!(ar.code, "AR");
        assert_eq!(ar.name, "Arkansas");
    }

    #[test]
    fn test_modified_comparative_50() {
        let comp_neg = ArkansasLaw::comparative_negligence();
        assert_eq!(comp_neg.rule, StateRule::ModifiedComparative50);
        assert_eq!(comp_neg.state.code, "AR");
        assert!(comp_neg.statutory_basis.is_some());
    }

    #[test]
    fn test_several_liability_only() {
        let joint_several = ArkansasLaw::joint_and_several_liability();
        assert_eq!(joint_several.rule, StateRule::SeveralLiabilityOnly);
        assert!(joint_several.statutory_basis.is_some());
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = ArkansasLaw::state_variations();
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
