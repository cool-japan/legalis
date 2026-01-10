//! Arizona State Law
//!
//! Arizona tort law features:
//! - Pure Comparative Negligence
//! - Ninth Circuit federal appeals jurisdiction
//! - Southwest growth state
//! - Progressive tort reform

use crate::states::types::{
    LegalTopic, LegalTradition, StateId, StateLawVariation, StateRule, StatuteReference,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Arizona state law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArizonaLaw;

impl ArizonaLaw {
    /// Get Arizona state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::new("AZ", "Arizona", LegalTradition::CommonLaw)
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
            StatuteReference::new("Ariz. Rev. Stat. § 12-2505")
                .with_title("Arizona Comparative Negligence")
                .with_year(1984),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1984, 1, 1).unwrap())
        .with_notes(
            "Arizona adopted pure comparative negligence in 1984. Plaintiff's damages \
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
            StatuteReference::new("Ariz. Rev. Stat. § 12-2506")
                .with_title("Arizona Several Liability")
                .with_year(1987),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1987, 1, 1).unwrap())
        .with_notes(
            "Arizona abolished joint and several liability in 1987. Each defendant \
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
    fn test_arizona_state_id() {
        let az = ArizonaLaw::state_id();
        assert_eq!(az.code, "AZ");
        assert_eq!(az.name, "Arizona");
    }

    #[test]
    fn test_pure_comparative_negligence() {
        let comp_neg = ArizonaLaw::comparative_negligence();
        assert_eq!(comp_neg.rule, StateRule::PureComparativeNegligence);
        assert_eq!(comp_neg.state.code, "AZ");
        assert!(comp_neg.statutory_basis.is_some());
    }

    #[test]
    fn test_several_liability_only() {
        let joint_several = ArizonaLaw::joint_and_several_liability();
        assert_eq!(joint_several.rule, StateRule::SeveralLiabilityOnly);
        assert!(joint_several.statutory_basis.is_some());
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = ArizonaLaw::state_variations();
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
