//! Kentucky State Law
//!
//! Kentucky tort law features:
//! - Pure Comparative Negligence
//! - Sixth Circuit federal appeals jurisdiction
//! - Southern regional influence
//! - Traditional common law approach

use crate::states::types::{
    LegalTopic, LegalTradition, StateId, StateLawVariation, StateRule, StatuteReference,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Kentucky state law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KentuckyLaw;

impl KentuckyLaw {
    /// Get Kentucky state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::new("KY", "Kentucky", LegalTradition::CommonLaw)
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
            StatuteReference::new("Ky. Rev. Stat. Ann. § 411.182")
                .with_title("Kentucky Comparative Negligence")
                .with_year(1984),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1984, 7, 13).unwrap())
        .with_notes(
            "Kentucky adopted pure comparative negligence in 1984. Plaintiff's damages \
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
            StateRule::SeveralLiabilityOnly,
        )
        .with_statute(
            StatuteReference::new("Ky. Rev. Stat. Ann. § 411.182")
                .with_title("Kentucky Several Liability")
                .with_year(1984),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1984, 7, 13).unwrap())
        .with_notes(
            "Kentucky abolished joint and several liability in 1984. Each defendant \
             is liable only for their proportionate share of damages based on their \
             percentage of fault.",
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
    fn test_kentucky_state_id() {
        let ky = KentuckyLaw::state_id();
        assert_eq!(ky.code, "KY");
        assert_eq!(ky.name, "Kentucky");
    }

    #[test]
    fn test_pure_comparative_negligence() {
        let comp_neg = KentuckyLaw::comparative_negligence();
        assert_eq!(comp_neg.rule, StateRule::PureComparativeNegligence);
        assert_eq!(comp_neg.state.code, "KY");
        assert!(comp_neg.statutory_basis.is_some());
    }

    #[test]
    fn test_several_liability_only() {
        let joint_several = KentuckyLaw::joint_and_several_liability();
        assert_eq!(joint_several.rule, StateRule::SeveralLiabilityOnly);
        assert!(joint_several.statutory_basis.is_some());
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = KentuckyLaw::state_variations();
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
