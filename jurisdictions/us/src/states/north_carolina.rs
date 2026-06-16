//! North Carolina State Law
//!
//! North Carolina tort law features:
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

/// North Carolina state law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NorthCarolinaLaw;

impl NorthCarolinaLaw {
    /// Get North Carolina state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::new("NC", "North Carolina", LegalTradition::CommonLaw)
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
            StatuteReference::new("N.C. Gen. Stat. § 1-139")
                .with_title("North Carolina Contributory Negligence")
                .with_year(1868),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1868, 1, 1).unwrap())
        .with_case(
            CaseReference::new(
                "Smith v. Fiber Controls Corp., 300 N.C. 669, 268 S.E.2d 504",
                "Smith v. Fiber Controls",
                1980,
            )
            .with_significance(
                "North Carolina Supreme Court reaffirmed that contributory negligence is a \
                 complete bar to recovery and declined to adopt comparative negligence, holding \
                 any change is for the legislature.",
            ),
        )
        .with_notes(
            "North Carolina retains the traditional contributory negligence rule. \
             Any contributory negligence by the plaintiff completely bars recovery, \
             regardless of the defendant's degree of fault. One of only 5 US \
             jurisdictions maintaining this minority rule (NC, VA, MD, DC, AL). \
             The 'last clear chance' doctrine remains an exception.",
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
            StatuteReference::new("N.C. Gen. Stat. § 1B-1")
                .with_title("North Carolina Joint Tortfeasors Contribution")
                .with_year(1967),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1967, 1, 1).unwrap())
        .with_notes(
            "North Carolina retains traditional joint and several liability. \
             Multiple tortfeasors are jointly and severally liable for indivisible \
             injuries, with right of contribution among joint tortfeasors.",
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
    fn test_north_carolina_state_id() {
        let nc = NorthCarolinaLaw::state_id();
        assert_eq!(nc.code, "NC");
        assert_eq!(nc.name, "North Carolina");
    }

    #[test]
    fn test_contributory_negligence_minority_rule() {
        let comp_neg = NorthCarolinaLaw::comparative_negligence();
        assert_eq!(comp_neg.rule, StateRule::ContributoryNegligence);
        assert_eq!(comp_neg.state.code, "NC");
        assert!(comp_neg.statutory_basis.is_some());
        // Verify this is minority rule
        assert!(comp_neg.notes.contains("One of only 5 US jurisdictions"));
    }

    #[test]
    fn test_joint_and_several_retained() {
        let joint_several = NorthCarolinaLaw::joint_and_several_liability();
        assert_eq!(joint_several.rule, StateRule::JointAndSeveralLiability);
        assert!(joint_several.statutory_basis.is_some());
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = NorthCarolinaLaw::state_variations();
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
        let comp_neg = NorthCarolinaLaw::comparative_negligence();
        assert_eq!(comp_neg.case_basis.len(), 1);
        let case = &comp_neg.case_basis[0];
        assert_eq!(case.short_name, "Smith v. Fiber Controls");
        assert_eq!(case.year, 1980);
        assert!(
            case.significance
                .as_ref()
                .expect("significance set")
                .contains("complete bar")
        );
    }

    #[test]
    fn test_last_clear_chance_exception_noted() {
        let comp_neg = NorthCarolinaLaw::comparative_negligence();
        assert!(comp_neg.notes.contains("last clear chance"));
    }
}
