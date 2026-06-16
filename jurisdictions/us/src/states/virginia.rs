//! Virginia State Law
//!
//! Virginia tort law features:
//! - Contributory Negligence (MINORITY RULE - only 5 jurisdictions)
//! - Traditional common law approach
//! - Fourth Circuit federal appeals jurisdiction
//! - Complete bar to recovery if plaintiff at fault

use crate::states::types::{
    CaseReference, LegalTopic, LegalTradition, StateId, StateLawVariation, StateRule,
    StatuteReference,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Virginia state law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirginiaLaw;

impl VirginiaLaw {
    /// Get Virginia state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::new("VA", "Virginia", LegalTradition::CommonLaw)
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
            StatuteReference::new("Va. Code Ann. § 8.01-34")
                .with_title("Virginia Contributory Negligence")
                .with_year(1950),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1950, 1, 1).unwrap())
        .with_case(
            CaseReference::new(
                "Baskett v. Banks, 186 Va. 1022, 45 S.E.2d 173",
                "Baskett v. Banks",
                1947,
            )
            .with_significance(
                "Virginia Supreme Court of Appeals applied contributory negligence as a complete \
                 bar to recovery, while recognizing the 'last clear chance' doctrine as a limited \
                 exception.",
            ),
        )
        .with_notes(
            "Virginia retains the traditional contributory negligence rule. \
             Any contributory negligence by the plaintiff is a complete bar to \
             recovery, regardless of the defendant's negligence. One of only 5 US \
             jurisdictions maintaining this minority rule (NC, VA, MD, DC, AL). \
             The 'last clear chance' doctrine remains a limited exception.",
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
            StatuteReference::new("Va. Code Ann. § 8.01-35.1")
                .with_title("Virginia Joint Tortfeasors")
                .with_year(1982),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1982, 1, 1).unwrap())
        .with_notes(
            "Virginia retains traditional joint and several liability. Multiple \
             tortfeasors are jointly and severally liable, with right of contribution \
             among defendants based on comparative fault.",
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
    fn test_virginia_state_id() {
        let va = VirginiaLaw::state_id();
        assert_eq!(va.code, "VA");
        assert_eq!(va.name, "Virginia");
    }

    #[test]
    fn test_contributory_negligence_minority_rule() {
        let comp_neg = VirginiaLaw::comparative_negligence();
        assert_eq!(comp_neg.rule, StateRule::ContributoryNegligence);
        assert_eq!(comp_neg.state.code, "VA");
        assert!(comp_neg.statutory_basis.is_some());
        // Verify this is minority rule
        assert!(comp_neg.notes.contains("One of only 5 US jurisdictions"));
    }

    #[test]
    fn test_joint_and_several_retained() {
        let joint_several = VirginiaLaw::joint_and_several_liability();
        assert_eq!(joint_several.rule, StateRule::JointAndSeveralLiability);
        assert!(joint_several.statutory_basis.is_some());
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = VirginiaLaw::state_variations();
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

    #[test]
    fn test_landmark_case_present() {
        let comp_neg = VirginiaLaw::comparative_negligence();
        assert_eq!(comp_neg.case_basis.len(), 1);
        let case = &comp_neg.case_basis[0];
        assert_eq!(case.short_name, "Baskett v. Banks");
        assert_eq!(case.year, 1947);
        assert!(
            case.significance
                .as_ref()
                .expect("significance set")
                .contains("complete")
        );
    }

    #[test]
    fn test_last_clear_chance_exception_noted() {
        let comp_neg = VirginiaLaw::comparative_negligence();
        assert!(comp_neg.notes.contains("last clear chance"));
    }
}
