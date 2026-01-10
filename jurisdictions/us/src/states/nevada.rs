//! Nevada State Law
//!
//! Nevada tort law features:
//! - Modified Comparative Negligence (51% bar)
//! - Ninth Circuit federal appeals jurisdiction
//! - Business-friendly environment
//! - Progressive tort principles

use crate::states::types::{
    LegalTopic, LegalTradition, StateId, StateLawVariation, StateRule, StatuteReference,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Nevada state law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NevadaLaw;

impl NevadaLaw {
    /// Get Nevada state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::new("NV", "Nevada", LegalTradition::CommonLaw)
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
            StatuteReference::new("Nev. Rev. Stat. § 41.141")
                .with_title("Nevada Comparative Negligence")
                .with_year(1973),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1973, 1, 1).unwrap())
        .with_notes(
            "Nevada adopted modified comparative negligence with 51% bar in 1973. \
             Plaintiff's recovery is barred if their negligence is greater than the \
             negligence of the defendant.",
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
            StatuteReference::new("Nev. Rev. Stat. § 41.141")
                .with_title("Nevada Several Liability")
                .with_year(1987),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1987, 10, 1).unwrap())
        .with_notes(
            "Nevada abolished joint and several liability in 1987. Each defendant \
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
    fn test_nevada_state_id() {
        let nv = NevadaLaw::state_id();
        assert_eq!(nv.code, "NV");
        assert_eq!(nv.name, "Nevada");
    }

    #[test]
    fn test_modified_comparative_51() {
        let comp_neg = NevadaLaw::comparative_negligence();
        assert_eq!(comp_neg.rule, StateRule::ModifiedComparative51);
        assert_eq!(comp_neg.state.code, "NV");
        assert!(comp_neg.statutory_basis.is_some());
    }

    #[test]
    fn test_several_liability_only() {
        let joint_several = NevadaLaw::joint_and_several_liability();
        assert_eq!(joint_several.rule, StateRule::SeveralLiabilityOnly);
        assert!(joint_several.statutory_basis.is_some());
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = NevadaLaw::state_variations();
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
