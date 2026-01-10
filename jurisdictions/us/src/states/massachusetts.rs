//! Massachusetts State Law
//!
//! Massachusetts tort law features:
//! - Modified Comparative Negligence (51% bar)
//! - New England legal leader
//! - First Circuit federal appeals jurisdiction
//! - Strong appellate court influence

use crate::states::types::{
    LegalTopic, LegalTradition, StateId, StateLawVariation, StateRule, StatuteReference,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Massachusetts state law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MassachusettsLaw;

impl MassachusettsLaw {
    /// Get Massachusetts state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::new("MA", "Massachusetts", LegalTradition::CommonLaw)
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
            StatuteReference::new("Mass. Gen. Laws ch. 231, § 85")
                .with_title("Massachusetts Comparative Negligence")
                .with_year(1986),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1986, 4, 1).unwrap())
        .with_notes(
            "Massachusetts adopted modified comparative negligence with 51% bar in 1986. \
             Plaintiff's recovery is barred if their fault is greater than the combined \
             fault of all defendants against whom recovery is sought.",
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
            StatuteReference::new("Mass. Gen. Laws ch. 231B, § 4")
                .with_title("Massachusetts Joint and Several Liability Reform")
                .with_year(1986),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1986, 4, 1).unwrap())
        .with_notes(
            "Massachusetts modified joint and several liability in 1986. Joint liability \
             applies if defendant is more than 50% at fault; otherwise several liability \
             applies for non-economic damages.",
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
    fn test_massachusetts_state_id() {
        let ma = MassachusettsLaw::state_id();
        assert_eq!(ma.code, "MA");
        assert_eq!(ma.name, "Massachusetts");
    }

    #[test]
    fn test_modified_comparative_51() {
        let comp_neg = MassachusettsLaw::comparative_negligence();
        assert_eq!(comp_neg.rule, StateRule::ModifiedComparative51);
        assert_eq!(comp_neg.state.code, "MA");
        assert!(comp_neg.statutory_basis.is_some());
        let statute = comp_neg.statutory_basis.as_ref().unwrap();
        assert_eq!(statute.citation, "Mass. Gen. Laws ch. 231, § 85");
    }

    #[test]
    fn test_modified_joint_and_several() {
        let joint_several = MassachusettsLaw::joint_and_several_liability();
        if let StateRule::ModifiedJointAndSeveral { threshold_percent } = joint_several.rule {
            assert_eq!(threshold_percent, 50);
        } else {
            panic!("Expected ModifiedJointAndSeveral");
        }
        assert!(joint_several.statutory_basis.is_some());
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = MassachusettsLaw::state_variations();
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
