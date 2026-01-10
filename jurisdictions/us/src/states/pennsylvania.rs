//! Pennsylvania State Law
//!
//! Pennsylvania tort law features:
//! - Modified Comparative Negligence (51% bar)
//! - Fair Share Act (joint and several liability reform)
//! - Third Circuit federal appeals jurisdiction
//! - Major Northeast legal center (Philadelphia)

use crate::states::types::{
    LegalTopic, LegalTradition, StateId, StateLawVariation, StateRule, StatuteReference,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Pennsylvania state law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PennsylvaniaLaw;

impl PennsylvaniaLaw {
    /// Get Pennsylvania state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::new("PA", "Pennsylvania", LegalTradition::CommonLaw)
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
            StatuteReference::new("42 Pa.C.S. § 7102")
                .with_title("Pennsylvania Comparative Negligence Act")
                .with_year(1976),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1976, 7, 25).unwrap())
        .with_notes(
            "Pennsylvania adopted modified comparative negligence with 51% bar in 1976. \
             Contributory negligence does not bar recovery where plaintiff's negligence \
             was not greater than the causal negligence of the defendant.",
        )
    }

    /// Joint and several liability variation (Fair Share Act).
    #[must_use]
    pub fn joint_and_several_liability() -> StateLawVariation {
        StateLawVariation::new(
            Self::state_id(),
            LegalTopic::JointAndSeveralLiability,
            StateRule::ModifiedJointAndSeveral {
                threshold_percent: 60,
            },
        )
        .with_statute(
            StatuteReference::new("42 Pa.C.S. § 7102")
                .with_title("Pennsylvania Fair Share Act")
                .with_year(2011),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(2011, 11, 23).unwrap())
        .with_notes(
            "Fair Share Act (2011): Joint liability applies only if defendant is more than \
             60% at fault; otherwise several liability applies. Each defendant liable for \
             their proportionate share plus any uncollectible shares from other defendants.",
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
    fn test_pennsylvania_state_id() {
        let pa = PennsylvaniaLaw::state_id();
        assert_eq!(pa.code, "PA");
        assert_eq!(pa.name, "Pennsylvania");
    }

    #[test]
    fn test_modified_comparative_51() {
        let comp_neg = PennsylvaniaLaw::comparative_negligence();
        assert_eq!(comp_neg.rule, StateRule::ModifiedComparative51);
        assert_eq!(comp_neg.state.code, "PA");
        assert!(comp_neg.statutory_basis.is_some());
        let statute = comp_neg.statutory_basis.as_ref().unwrap();
        assert_eq!(statute.citation, "42 Pa.C.S. § 7102");
    }

    #[test]
    fn test_fair_share_act() {
        let joint_several = PennsylvaniaLaw::joint_and_several_liability();
        if let StateRule::ModifiedJointAndSeveral { threshold_percent } = joint_several.rule {
            assert_eq!(threshold_percent, 60);
        } else {
            panic!("Expected ModifiedJointAndSeveral");
        }
        assert!(joint_several.statutory_basis.is_some());
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = PennsylvaniaLaw::state_variations();
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
