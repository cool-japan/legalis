//! Maryland State Law
//!
//! Maryland tort law features:
//! - Contributory Negligence (MINORITY RULE - only 5 jurisdictions)
//! - Fourth Circuit federal appeals jurisdiction
//! - Traditional common law approach
//! - Complete bar to recovery if plaintiff at fault

use crate::states::types::{
    CaseReference, LegalTopic, LegalTradition, StateId, StateLawVariation, StateRule,
    StatuteReference,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Maryland state law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarylandLaw;

impl MarylandLaw {
    /// Get Maryland state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::new("MD", "Maryland", LegalTradition::CommonLaw)
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
            StatuteReference::new("Md. Cts. & Jud. Proc. Code Ann. § 11-108")
                .with_title("Maryland Contributory Negligence")
                .with_year(1973),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1973, 7, 1).unwrap())
        .with_case(
            CaseReference::new(
                "Coleman v. Soccer Ass'n of Columbia, 432 Md. 679, 69 A.3d 1149",
                "Coleman v. Soccer Ass'n",
                2013,
            )
            .with_significance(
                "Maryland Court of Appeals expressly declined to abrogate contributory negligence \
                 in favor of comparative negligence, holding that any such change is for the \
                 General Assembly, not the courts.",
            ),
        )
        .with_notes(
            "Maryland retains the traditional contributory negligence rule. Any \
             contributory negligence by the plaintiff is a complete bar to recovery. \
             One of only 5 US jurisdictions maintaining this minority rule (NC, VA, \
             MD, DC, AL). Reaffirmed by the Court of Appeals in 2013.",
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
            StatuteReference::new("Md. Cts. & Jud. Proc. Code Ann. § 3-1402")
                .with_title("Maryland Uniform Contribution Among Tortfeasors Act")
                .with_year(1941),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1941, 1, 1).unwrap())
        .with_notes(
            "Maryland retains traditional joint and several liability. Multiple \
             tortfeasors are jointly and severally liable, with right of contribution \
             among joint tortfeasors.",
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
    fn test_maryland_state_id() {
        let md = MarylandLaw::state_id();
        assert_eq!(md.code, "MD");
        assert_eq!(md.name, "Maryland");
    }

    #[test]
    fn test_contributory_negligence_minority_rule() {
        let comp_neg = MarylandLaw::comparative_negligence();
        assert_eq!(comp_neg.rule, StateRule::ContributoryNegligence);
        assert_eq!(comp_neg.state.code, "MD");
        assert!(comp_neg.statutory_basis.is_some());
        // Verify this is minority rule
        assert!(comp_neg.notes.contains("One of only 5 US jurisdictions"));
    }

    #[test]
    fn test_joint_and_several_retained() {
        let joint_several = MarylandLaw::joint_and_several_liability();
        assert_eq!(joint_several.rule, StateRule::JointAndSeveralLiability);
        assert!(joint_several.statutory_basis.is_some());
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = MarylandLaw::state_variations();
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
        let comp_neg = MarylandLaw::comparative_negligence();
        assert_eq!(comp_neg.case_basis.len(), 1);
        let case = &comp_neg.case_basis[0];
        assert_eq!(case.short_name, "Coleman v. Soccer Ass'n");
        assert_eq!(case.year, 2013);
        assert!(
            case.significance
                .as_ref()
                .expect("significance set")
                .contains("General Assembly")
        );
    }

    #[test]
    fn test_reaffirmed_2013_noted() {
        let comp_neg = MarylandLaw::comparative_negligence();
        assert!(comp_neg.notes.contains("2013"));
    }
}
