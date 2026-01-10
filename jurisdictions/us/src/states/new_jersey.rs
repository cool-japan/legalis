//! New Jersey State Law
//!
//! New Jersey tort law features:
//! - Modified Comparative Negligence (51% bar)
//! - Proximity to New York legal market
//! - Third Circuit federal appeals jurisdiction
//! - Consumer protection leadership

use crate::states::types::{
    LegalTopic, LegalTradition, StateId, StateLawVariation, StateRule, StatuteReference,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// New Jersey state law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewJerseyLaw;

impl NewJerseyLaw {
    /// Get New Jersey state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::new("NJ", "New Jersey", LegalTradition::CommonLaw)
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
            StatuteReference::new("N.J.S.A. 2A:15-5.1")
                .with_title("New Jersey Comparative Negligence Act")
                .with_year(1973),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1973, 8, 28).unwrap())
        .with_notes(
            "New Jersey adopted modified comparative negligence with 51% bar in 1973 \
             (one of the earliest states). Plaintiff's recovery is barred if their \
             negligence is greater than the negligence of the defendant.",
        )
    }

    /// Joint and several liability variation.
    #[must_use]
    pub fn joint_and_several_liability() -> StateLawVariation {
        StateLawVariation::new(
            Self::state_id(),
            LegalTopic::JointAndSeveralLiability,
            StateRule::JointAndSeveralLiability,
        )
        .with_statute(
            StatuteReference::new("N.J.S.A. 2A:15-5.3")
                .with_title("New Jersey Joint Tortfeasors Contribution Act")
                .with_year(1953),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1953, 1, 1).unwrap())
        .with_notes(
            "New Jersey retains traditional joint and several liability. Multiple \
             tortfeasors who contribute to a single indivisible injury are jointly \
             and severally liable, with right of contribution among defendants.",
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
    fn test_new_jersey_state_id() {
        let nj = NewJerseyLaw::state_id();
        assert_eq!(nj.code, "NJ");
        assert_eq!(nj.name, "New Jersey");
    }

    #[test]
    fn test_modified_comparative_51() {
        let comp_neg = NewJerseyLaw::comparative_negligence();
        assert_eq!(comp_neg.rule, StateRule::ModifiedComparative51);
        assert_eq!(comp_neg.state.code, "NJ");
        assert!(comp_neg.statutory_basis.is_some());
        let statute = comp_neg.statutory_basis.as_ref().unwrap();
        assert_eq!(statute.citation, "N.J.S.A. 2A:15-5.1");
    }

    #[test]
    fn test_joint_and_several_retained() {
        let joint_several = NewJerseyLaw::joint_and_several_liability();
        assert_eq!(joint_several.rule, StateRule::JointAndSeveralLiability);
        assert!(joint_several.statutory_basis.is_some());
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = NewJerseyLaw::state_variations();
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
