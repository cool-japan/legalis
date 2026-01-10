//! Indiana State Law
//!
//! Indiana tort law features:
//! - Modified Comparative Negligence (51% bar)
//! - Seventh Circuit federal appeals jurisdiction
//! - Midwest regional influence
//! - Tort reform with damage caps

use crate::states::types::{
    LegalTopic, LegalTradition, StateId, StateLawVariation, StateRule, StatuteReference,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Indiana state law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndianaLaw;

impl IndianaLaw {
    /// Get Indiana state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::new("IN", "Indiana", LegalTradition::CommonLaw)
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
            StatuteReference::new("Ind. Code § 34-51-2-6")
                .with_title("Indiana Comparative Fault")
                .with_year(1983),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1983, 1, 1).unwrap())
        .with_notes(
            "Indiana adopted modified comparative negligence with 51% bar in 1983. \
             Contributory fault does not bar recovery unless plaintiff's fault is \
             greater than the fault of all defendants.",
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
            StatuteReference::new("Ind. Code § 34-51-2-7")
                .with_title("Indiana Several Liability")
                .with_year(1995),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1995, 1, 1).unwrap())
        .with_notes(
            "Indiana abolished joint and several liability in 1995. Each defendant is \
             liable only for their percentage of comparative fault.",
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
    fn test_indiana_state_id() {
        let ind = IndianaLaw::state_id();
        assert_eq!(ind.code, "IN");
        assert_eq!(ind.name, "Indiana");
    }

    #[test]
    fn test_modified_comparative_51() {
        let comp_neg = IndianaLaw::comparative_negligence();
        assert_eq!(comp_neg.rule, StateRule::ModifiedComparative51);
        assert_eq!(comp_neg.state.code, "IN");
        assert!(comp_neg.statutory_basis.is_some());
    }

    #[test]
    fn test_several_liability_only() {
        let joint_several = IndianaLaw::joint_and_several_liability();
        assert_eq!(joint_several.rule, StateRule::SeveralLiabilityOnly);
        assert!(joint_several.statutory_basis.is_some());
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = IndianaLaw::state_variations();
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
