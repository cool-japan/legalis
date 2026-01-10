//! Alabama State Law
//!
//! Alabama tort law features:
//! - Contributory Negligence (MINORITY RULE - only 5 jurisdictions)
//! - Traditional common law approach
//! - Eleventh Circuit federal appeals jurisdiction
//! - Complete bar to recovery if plaintiff at fault

use crate::states::types::{
    LegalTopic, LegalTradition, StateId, StateLawVariation, StateRule, StatuteReference,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Alabama state law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlabamaLaw;

impl AlabamaLaw {
    /// Get Alabama state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::new("AL", "Alabama", LegalTradition::CommonLaw)
    }

    /// Get comparative negligence variation.
    #[must_use]
    pub fn comparative_negligence() -> StateLawVariation {
        StateLawVariation::new(
            Self::state_id(),
            LegalTopic::ComparativeNegligence,
            StateRule::ContributoryNegligence,
        )
        .with_statute(
            StatuteReference::new("Ala. Code § 6-5-521")
                .with_title("Alabama Contributory Negligence")
                .with_year(1975),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1975, 1, 1).unwrap())
        .with_notes(
            "Alabama retains the traditional contributory negligence rule. \
             Any contributory negligence by the plaintiff is a complete bar to \
             recovery, regardless of the defendant's degree of negligence. One of \
             only 5 US jurisdictions maintaining this minority rule (NC, VA, MD, DC, AL).",
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
            StatuteReference::new("Ala. Code § 6-5-543")
                .with_title("Alabama Several Liability Act")
                .with_year(1987),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1987, 6, 11).unwrap())
        .with_notes(
            "Alabama abolished joint and several liability in 1987. Each defendant \
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
    fn test_alabama_state_id() {
        let al = AlabamaLaw::state_id();
        assert_eq!(al.code, "AL");
        assert_eq!(al.name, "Alabama");
    }

    #[test]
    fn test_contributory_negligence_minority_rule() {
        let comp_neg = AlabamaLaw::comparative_negligence();
        assert_eq!(comp_neg.rule, StateRule::ContributoryNegligence);
        assert_eq!(comp_neg.state.code, "AL");
        assert!(comp_neg.statutory_basis.is_some());
        assert!(comp_neg.notes.contains("One of only 5 US jurisdictions"));
    }

    #[test]
    fn test_several_liability_only() {
        let joint_several = AlabamaLaw::joint_and_several_liability();
        assert_eq!(joint_several.rule, StateRule::SeveralLiabilityOnly);
        assert!(joint_several.statutory_basis.is_some());
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = AlabamaLaw::state_variations();
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
