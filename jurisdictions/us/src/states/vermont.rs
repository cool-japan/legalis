//! Vermont State Law
//!
//! Vermont tort law features:
//! - Modified Comparative Negligence (51% bar)
//! - Second Circuit federal appeals jurisdiction
//! - New England legal tradition
//! - Modified Joint and Several Liability (50% threshold)

use crate::states::types::{
    LegalTopic, LegalTradition, StateId, StateLawVariation, StateRule, StatuteReference,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Vermont state law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VermontLaw;

impl VermontLaw {
    /// Get Vermont state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::new("VT", "Vermont", LegalTradition::CommonLaw)
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
            StatuteReference::new("12 Vt. Stat. Ann. § 1036")
                .with_title("Vermont Comparative Negligence")
                .with_year(1971),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1971, 1, 1).unwrap())
        .with_notes(
            "Vermont adopted modified comparative negligence with 51% bar in 1971. \
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
            StateRule::ModifiedJointAndSeveral {
                threshold_percent: 50,
            },
        )
        .with_statute(
            StatuteReference::new("12 Vt. Stat. Ann. § 1036")
                .with_title("Vermont Modified Joint and Several Liability")
                .with_year(1987),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1987, 1, 1).unwrap())
        .with_notes(
            "Vermont adopted modified joint and several liability in 1987. \
             Defendants with 50% or more fault are jointly and severally liable. \
             Defendants with less than 50% fault are only severally liable for \
             their proportionate share.",
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
    fn test_vermont_state_id() {
        let vt = VermontLaw::state_id();
        assert_eq!(vt.code, "VT");
        assert_eq!(vt.name, "Vermont");
    }

    #[test]
    fn test_modified_comparative_51() {
        let comp_neg = VermontLaw::comparative_negligence();
        assert_eq!(comp_neg.rule, StateRule::ModifiedComparative51);
        assert_eq!(comp_neg.state.code, "VT");
        assert!(comp_neg.statutory_basis.is_some());
    }

    #[test]
    fn test_modified_joint_and_several_liability() {
        let joint_several = VermontLaw::joint_and_several_liability();
        assert_eq!(
            joint_several.rule,
            StateRule::ModifiedJointAndSeveral {
                threshold_percent: 50
            }
        );
        assert!(joint_several.statutory_basis.is_some());
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = VermontLaw::state_variations();
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
