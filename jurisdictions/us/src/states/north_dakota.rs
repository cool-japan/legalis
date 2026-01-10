//! North Dakota State Law
//!
//! North Dakota tort law features:
//! - Modified Comparative Negligence (50% bar)
//! - Eighth Circuit federal appeals jurisdiction
//! - Traditional common law approach
//! - Conservative liability standards

use crate::states::types::{
    LegalTopic, LegalTradition, StateId, StateLawVariation, StateRule, StatuteReference,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// North Dakota state law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NorthDakotaLaw;

impl NorthDakotaLaw {
    /// Get North Dakota state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::new("ND", "North Dakota", LegalTradition::CommonLaw)
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
            StatuteReference::new("N.D. Cent. Code § 9-10-07")
                .with_title("North Dakota Comparative Fault")
                .with_year(1973),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1973, 1, 1).unwrap())
        .with_notes(
            "North Dakota adopted modified comparative negligence with 50% bar in 1973. \
             Plaintiff's recovery is barred if their fault is as great as or greater \
             than the combined fault of all defendants (50% bar).",
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
            StatuteReference::new("N.D. Cent. Code § 32-03.2-02")
                .with_title("North Dakota Several Liability")
                .with_year(1987),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1987, 1, 1).unwrap())
        .with_notes(
            "North Dakota abolished joint and several liability in 1987. Each defendant \
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
    fn test_north_dakota_state_id() {
        let nd = NorthDakotaLaw::state_id();
        assert_eq!(nd.code, "ND");
        assert_eq!(nd.name, "North Dakota");
    }

    #[test]
    fn test_modified_comparative_50() {
        let comp_neg = NorthDakotaLaw::comparative_negligence();
        assert_eq!(comp_neg.rule, StateRule::ModifiedComparative50);
        assert_eq!(comp_neg.state.code, "ND");
        assert!(comp_neg.statutory_basis.is_some());
    }

    #[test]
    fn test_several_liability_only() {
        let joint_several = NorthDakotaLaw::joint_and_several_liability();
        assert_eq!(joint_several.rule, StateRule::SeveralLiabilityOnly);
        assert!(joint_several.statutory_basis.is_some());
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = NorthDakotaLaw::state_variations();
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
