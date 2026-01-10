//! Idaho State Law
//!
//! Idaho tort law features:
//! - Modified Comparative Negligence (50% bar)
//! - Ninth Circuit federal appeals jurisdiction
//! - Mountain West region
//! - Tort reform measures

use crate::states::types::{
    LegalTopic, LegalTradition, StateId, StateLawVariation, StateRule, StatuteReference,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Idaho state law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdahoLaw;

impl IdahoLaw {
    /// Get Idaho state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::new("ID", "Idaho", LegalTradition::CommonLaw)
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
            StatuteReference::new("Idaho Code § 6-801")
                .with_title("Idaho Comparative Negligence")
                .with_year(1971),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1971, 1, 1).unwrap())
        .with_notes(
            "Idaho adopted modified comparative negligence with 50% bar in 1971. \
             Plaintiff's recovery is barred if their fault is as great as or greater \
             than the fault of all defendants combined (50% bar).",
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
            StatuteReference::new("Idaho Code § 6-803")
                .with_title("Idaho Several Liability")
                .with_year(1990),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1990, 7, 1).unwrap())
        .with_notes(
            "Idaho abolished joint and several liability in 1990. Each defendant \
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
    fn test_idaho_state_id() {
        let id = IdahoLaw::state_id();
        assert_eq!(id.code, "ID");
        assert_eq!(id.name, "Idaho");
    }

    #[test]
    fn test_modified_comparative_50() {
        let comp_neg = IdahoLaw::comparative_negligence();
        assert_eq!(comp_neg.rule, StateRule::ModifiedComparative50);
        assert_eq!(comp_neg.state.code, "ID");
        assert!(comp_neg.statutory_basis.is_some());
    }

    #[test]
    fn test_several_liability_only() {
        let joint_several = IdahoLaw::joint_and_several_liability();
        assert_eq!(joint_several.rule, StateRule::SeveralLiabilityOnly);
        assert!(joint_several.statutory_basis.is_some());
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = IdahoLaw::state_variations();
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
