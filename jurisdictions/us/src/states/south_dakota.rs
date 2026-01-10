//! South Dakota State Law
//!
//! South Dakota tort law features:
//! - Modified Comparative Negligence (50% bar)
//! - Eighth Circuit federal appeals jurisdiction
//! - Traditional common law approach
//! - Conservative liability standards

use crate::states::types::{
    LegalTopic, LegalTradition, StateId, StateLawVariation, StateRule, StatuteReference,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// South Dakota state law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SouthDakotaLaw;

impl SouthDakotaLaw {
    /// Get South Dakota state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::new("SD", "South Dakota", LegalTradition::CommonLaw)
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
            StatuteReference::new("S.D. Codified Laws § 20-9-2")
                .with_title("South Dakota Comparative Fault")
                .with_year(1968),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1968, 1, 1).unwrap())
        .with_notes(
            "South Dakota adopted modified comparative negligence with 50% bar in 1968. \
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
            StatuteReference::new("S.D. Codified Laws § 15-8-15.1")
                .with_title("South Dakota Several Liability")
                .with_year(1988),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1988, 1, 1).unwrap())
        .with_notes(
            "South Dakota abolished joint and several liability in 1988. Each defendant \
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
    fn test_south_dakota_state_id() {
        let sd = SouthDakotaLaw::state_id();
        assert_eq!(sd.code, "SD");
        assert_eq!(sd.name, "South Dakota");
    }

    #[test]
    fn test_modified_comparative_50() {
        let comp_neg = SouthDakotaLaw::comparative_negligence();
        assert_eq!(comp_neg.rule, StateRule::ModifiedComparative50);
        assert_eq!(comp_neg.state.code, "SD");
        assert!(comp_neg.statutory_basis.is_some());
    }

    #[test]
    fn test_several_liability_only() {
        let joint_several = SouthDakotaLaw::joint_and_several_liability();
        assert_eq!(joint_several.rule, StateRule::SeveralLiabilityOnly);
        assert!(joint_several.statutory_basis.is_some());
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = SouthDakotaLaw::state_variations();
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
