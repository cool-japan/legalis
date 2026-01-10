//! Wisconsin State Law
//!
//! Wisconsin tort law features:
//! - Modified Comparative Negligence (51% bar)
//! - Seventh Circuit federal appeals jurisdiction
//! - Midwest legal traditions
//! - Progressive consumer protection

use crate::states::types::{
    LegalTopic, LegalTradition, StateId, StateLawVariation, StateRule, StatuteReference,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Wisconsin state law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WisconsinLaw;

impl WisconsinLaw {
    /// Get Wisconsin state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::new("WI", "Wisconsin", LegalTradition::CommonLaw)
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
            StatuteReference::new("Wis. Stat. § 895.045")
                .with_title("Wisconsin Comparative Negligence")
                .with_year(1971),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1971, 1, 1).unwrap())
        .with_notes(
            "Wisconsin was an early adopter of modified comparative negligence with 51% \
             bar in 1971. Plaintiff's negligence not greater than defendant's does not \
             bar recovery.",
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
            StatuteReference::new("Wis. Stat. § 895.045")
                .with_title("Wisconsin Several Liability")
                .with_year(1995),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1995, 1, 1).unwrap())
        .with_notes(
            "Wisconsin abolished joint and several liability in 1995. Each defendant \
             is liable only for their percentage of causal negligence.",
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
    fn test_wisconsin_state_id() {
        let wi = WisconsinLaw::state_id();
        assert_eq!(wi.code, "WI");
        assert_eq!(wi.name, "Wisconsin");
    }

    #[test]
    fn test_modified_comparative_51() {
        let comp_neg = WisconsinLaw::comparative_negligence();
        assert_eq!(comp_neg.rule, StateRule::ModifiedComparative51);
        assert_eq!(comp_neg.state.code, "WI");
        assert!(comp_neg.statutory_basis.is_some());
    }

    #[test]
    fn test_several_liability_only() {
        let joint_several = WisconsinLaw::joint_and_several_liability();
        assert_eq!(joint_several.rule, StateRule::SeveralLiabilityOnly);
        assert!(joint_several.statutory_basis.is_some());
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = WisconsinLaw::state_variations();
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
