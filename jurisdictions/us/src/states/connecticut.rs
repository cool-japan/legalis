//! Connecticut State Law
//!
//! Connecticut tort law features:
//! - Modified Comparative Negligence (51% bar)
//! - Second Circuit federal appeals jurisdiction
//! - New England legal tradition
//! - Products liability leadership

use crate::states::types::{
    LegalTopic, LegalTradition, StateId, StateLawVariation, StateRule, StatuteReference,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Connecticut state law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnecticutLaw;

impl ConnecticutLaw {
    /// Get Connecticut state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::new("CT", "Connecticut", LegalTradition::CommonLaw)
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
            StatuteReference::new("Conn. Gen. Stat. § 52-572h")
                .with_title("Connecticut Comparative Negligence")
                .with_year(1973),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1973, 10, 1).unwrap())
        .with_notes(
            "Connecticut adopted modified comparative negligence with 51% bar in 1973. \
             Plaintiff's recovery is barred if their negligence is greater than the \
             combined negligence of defendants.",
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
            StatuteReference::new("Conn. Gen. Stat. § 52-572h(o)")
                .with_title("Connecticut Joint and Several Liability")
                .with_year(1995),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1995, 1, 1).unwrap())
        .with_notes(
            "Connecticut retains modified joint and several liability for economic \
             damages, with several liability for non-economic damages. Defendants \
             remain jointly and severally liable for economic losses.",
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
    fn test_connecticut_state_id() {
        let ct = ConnecticutLaw::state_id();
        assert_eq!(ct.code, "CT");
        assert_eq!(ct.name, "Connecticut");
    }

    #[test]
    fn test_modified_comparative_51() {
        let comp_neg = ConnecticutLaw::comparative_negligence();
        assert_eq!(comp_neg.rule, StateRule::ModifiedComparative51);
        assert_eq!(comp_neg.state.code, "CT");
        assert!(comp_neg.statutory_basis.is_some());
    }

    #[test]
    fn test_joint_and_several_retained() {
        let joint_several = ConnecticutLaw::joint_and_several_liability();
        assert_eq!(joint_several.rule, StateRule::JointAndSeveralLiability);
        assert!(joint_several.statutory_basis.is_some());
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = ConnecticutLaw::state_variations();
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
