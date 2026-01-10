//! Rhode Island State Law
//!
//! Rhode Island tort law features:
//! - Pure Comparative Negligence
//! - First Circuit federal appeals jurisdiction
//! - New England legal tradition
//! - Retains Joint and Several Liability (notable exception)

use crate::states::types::{
    LegalTopic, LegalTradition, StateId, StateLawVariation, StateRule, StatuteReference,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Rhode Island state law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RhodeIslandLaw;

impl RhodeIslandLaw {
    /// Get Rhode Island state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::new("RI", "Rhode Island", LegalTradition::CommonLaw)
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
            StatuteReference::new("R.I. Gen. Laws § 9-20-4")
                .with_title("Rhode Island Comparative Negligence")
                .with_year(1969),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1969, 1, 1).unwrap())
        .with_notes(
            "Rhode Island adopted pure comparative negligence in 1969. A plaintiff's \
             recovery is reduced by their percentage of fault, but not barred \
             entirely, regardless of fault percentage.",
        )
    }

    /// Joint and several liability variation.
    #[must_use]
    pub fn joint_and_several_liability() -> StateLawVariation {
        StateLawVariation::new(
            Self::state_id(),
            LegalTopic::JointAndSeveralLiability,
            StateRule::JointAndSeveralLiability,
        )
        .with_statute(
            StatuteReference::new("R.I. Gen. Laws § 10-6-1")
                .with_title("Rhode Island Joint and Several Liability")
                .with_year(1986),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1986, 1, 1).unwrap())
        .with_notes(
            "Rhode Island retains traditional joint and several liability. \
             Multiple defendants can be held jointly liable for the entire amount \
             of damages. This is a notable exception to the modern trend of \
             abolishing joint and several liability.",
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
    fn test_rhode_island_state_id() {
        let ri = RhodeIslandLaw::state_id();
        assert_eq!(ri.code, "RI");
        assert_eq!(ri.name, "Rhode Island");
    }

    #[test]
    fn test_pure_comparative_negligence() {
        let comp_neg = RhodeIslandLaw::comparative_negligence();
        assert_eq!(comp_neg.rule, StateRule::PureComparativeNegligence);
        assert_eq!(comp_neg.state.code, "RI");
        assert!(comp_neg.statutory_basis.is_some());
    }

    #[test]
    fn test_joint_and_several_liability_retained() {
        let joint_several = RhodeIslandLaw::joint_and_several_liability();
        assert_eq!(joint_several.rule, StateRule::JointAndSeveralLiability);
        assert!(joint_several.statutory_basis.is_some());
        assert!(joint_several.notes.contains("notable exception"));
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = RhodeIslandLaw::state_variations();
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
