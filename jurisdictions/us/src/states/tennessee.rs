//! Tennessee State Law
//!
//! Tennessee tort law features:
//! - Modified Comparative Negligence (50% bar)
//! - Sixth Circuit federal appeals jurisdiction
//! - Southern regional influence
//! - Medical malpractice reform

use crate::states::types::{
    LegalTopic, LegalTradition, StateId, StateLawVariation, StateRule, StatuteReference,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Tennessee state law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TennesseeLaw;

impl TennesseeLaw {
    /// Get Tennessee state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::new("TN", "Tennessee", LegalTradition::CommonLaw)
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
            StatuteReference::new("Tenn. Code Ann. § 29-11-107")
                .with_title("Tennessee Comparative Fault")
                .with_year(1992),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1992, 7, 1).unwrap())
        .with_notes(
            "Tennessee adopted modified comparative negligence with 50% bar in 1992. \
             Plaintiff's recovery is barred if their fault is as great as the combined \
             fault of all defendants (50% bar, not 51%).",
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
            StatuteReference::new("Tenn. Code Ann. § 29-11-102")
                .with_title("Tennessee Several Liability")
                .with_year(1992),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1992, 7, 1).unwrap())
        .with_notes(
            "Tennessee abolished joint and several liability in 1992. Each defendant \
             is liable only for their percentage of comparative fault.",
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
    fn test_tennessee_state_id() {
        let tn = TennesseeLaw::state_id();
        assert_eq!(tn.code, "TN");
        assert_eq!(tn.name, "Tennessee");
    }

    #[test]
    fn test_modified_comparative_50() {
        let comp_neg = TennesseeLaw::comparative_negligence();
        assert_eq!(comp_neg.rule, StateRule::ModifiedComparative50);
        assert_eq!(comp_neg.state.code, "TN");
        assert!(comp_neg.statutory_basis.is_some());
    }

    #[test]
    fn test_several_liability_only() {
        let joint_several = TennesseeLaw::joint_and_several_liability();
        assert_eq!(joint_several.rule, StateRule::SeveralLiabilityOnly);
        assert!(joint_several.statutory_basis.is_some());
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = TennesseeLaw::state_variations();
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
