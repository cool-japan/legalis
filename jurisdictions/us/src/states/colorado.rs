//! Colorado State Law
//!
//! Colorado tort law features:
//! - Modified Comparative Negligence (50% bar)
//! - Tenth Circuit federal appeals jurisdiction
//! - Mountain West legal center
//! - Tort reform with damage caps

use crate::states::types::{
    LegalTopic, LegalTradition, StateId, StateLawVariation, StateRule, StatuteReference,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Colorado state law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColoradoLaw;

impl ColoradoLaw {
    /// Get Colorado state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::new("CO", "Colorado", LegalTradition::CommonLaw)
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
            StatuteReference::new("Colo. Rev. Stat. § 13-21-111")
                .with_title("Colorado Comparative Negligence")
                .with_year(1986),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1986, 7, 1).unwrap())
        .with_notes(
            "Colorado adopted modified comparative negligence with 50% bar in 1986. \
             Plaintiff's recovery is barred if their fault is equal to or greater \
             than 50% (50% bar, not 51%).",
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
            StatuteReference::new("Colo. Rev. Stat. § 13-21-111.5")
                .with_title("Colorado Several Liability")
                .with_year(1986),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1986, 7, 1).unwrap())
        .with_notes(
            "Colorado abolished joint and several liability in 1986. Each defendant \
             is liable only for their percentage of fault.",
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
    fn test_colorado_state_id() {
        let co = ColoradoLaw::state_id();
        assert_eq!(co.code, "CO");
        assert_eq!(co.name, "Colorado");
    }

    #[test]
    fn test_modified_comparative_50() {
        let comp_neg = ColoradoLaw::comparative_negligence();
        assert_eq!(comp_neg.rule, StateRule::ModifiedComparative50);
        assert_eq!(comp_neg.state.code, "CO");
        assert!(comp_neg.statutory_basis.is_some());
    }

    #[test]
    fn test_several_liability_only() {
        let joint_several = ColoradoLaw::joint_and_several_liability();
        assert_eq!(joint_several.rule, StateRule::SeveralLiabilityOnly);
        assert!(joint_several.statutory_basis.is_some());
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = ColoradoLaw::state_variations();
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
