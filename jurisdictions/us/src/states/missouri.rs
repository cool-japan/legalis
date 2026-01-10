//! Missouri State Law
//!
//! Missouri tort law features:
//! - Pure Comparative Negligence
//! - Eighth Circuit federal appeals jurisdiction
//! - Midwest legal center
//! - Regional tort law influence

use crate::states::types::{
    LegalTopic, LegalTradition, StateId, StateLawVariation, StateRule, StatuteReference,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Missouri state law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissouriLaw;

impl MissouriLaw {
    /// Get Missouri state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::new("MO", "Missouri", LegalTradition::CommonLaw)
    }

    /// Get comparative negligence variation.
    #[must_use]
    pub fn comparative_negligence() -> StateLawVariation {
        StateLawVariation::new(
            Self::state_id(),
            LegalTopic::ComparativeNegligence,
            StateRule::PureComparativeNegligence,
        )
        .with_statute(
            StatuteReference::new("Mo. Rev. Stat. § 537.765")
                .with_title("Missouri Comparative Fault")
                .with_year(1983),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1983, 1, 1).unwrap())
        .with_notes(
            "Missouri adopted pure comparative negligence in 1983. Plaintiff's damages \
             are reduced by their percentage of fault, with no bar to recovery \
             regardless of fault percentage.",
        )
    }

    /// Joint and several liability variation.
    #[must_use]
    pub fn joint_and_several_liability() -> StateLawVariation {
        StateLawVariation::new(
            Self::state_id(),
            LegalTopic::JointAndSeveralLiability,
            StateRule::ModifiedJointAndSeveral {
                threshold_percent: 51,
            },
        )
        .with_statute(
            StatuteReference::new("Mo. Rev. Stat. § 537.067")
                .with_title("Missouri Joint and Several Liability Reform")
                .with_year(2005),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(2005, 8, 28).unwrap())
        .with_notes(
            "Missouri tort reform (2005) modified joint and several liability. Joint \
             liability applies if defendant is more than 51% at fault; otherwise \
             several liability applies.",
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
    fn test_missouri_state_id() {
        let mo = MissouriLaw::state_id();
        assert_eq!(mo.code, "MO");
        assert_eq!(mo.name, "Missouri");
    }

    #[test]
    fn test_pure_comparative_negligence() {
        let comp_neg = MissouriLaw::comparative_negligence();
        assert_eq!(comp_neg.rule, StateRule::PureComparativeNegligence);
        assert_eq!(comp_neg.state.code, "MO");
        assert!(comp_neg.statutory_basis.is_some());
    }

    #[test]
    fn test_modified_joint_and_several() {
        let joint_several = MissouriLaw::joint_and_several_liability();
        if let StateRule::ModifiedJointAndSeveral { threshold_percent } = joint_several.rule {
            assert_eq!(threshold_percent, 51);
        } else {
            panic!("Expected ModifiedJointAndSeveral");
        }
        assert!(joint_several.statutory_basis.is_some());
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = MissouriLaw::state_variations();
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
