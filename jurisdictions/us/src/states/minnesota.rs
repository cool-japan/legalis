//! Minnesota State Law
//!
//! Minnesota tort law features:
//! - Modified Comparative Negligence (51% bar)
//! - Better Law Approach (choice of law)
//! - Eighth Circuit federal appeals jurisdiction
//! - Progressive legal traditions

use crate::states::types::{
    CaseReference, LegalTopic, LegalTradition, StateId, StateLawVariation, StateRule,
    StatuteReference,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Minnesota state law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinnesotaLaw;

impl MinnesotaLaw {
    /// Get Minnesota state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::new("MN", "Minnesota", LegalTradition::CommonLaw)
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
            StatuteReference::new("Minn. Stat. § 604.01")
                .with_title("Minnesota Comparative Fault")
                .with_year(1978),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1978, 1, 1).unwrap())
        .with_case(
            CaseReference::new(
                "Milkovich v. Saari, 295 Minn. 155, 203 N.W.2d 408",
                "Milkovich v. Saari",
                1973,
            )
            .with_significance(
                "Leading Minnesota decision adopting Robert Leflar's 'better law' / choice-\
                 influencing-considerations approach to choice of law, applying Minnesota's \
                 comparative negligence rule over Ontario's contributory negligence bar.",
            ),
        )
        .with_notes(
            "Minnesota adopted modified comparative negligence with a 51% bar in 1978. \
             Plaintiff's contributory fault does not bar recovery unless greater than \
             defendant's fault. Minnesota is a leading 'better law' choice-of-law \
             jurisdiction (Milkovich v. Saari).",
        )
    }

    /// Joint and several liability variation.
    #[must_use]
    pub fn joint_and_several_liability() -> StateLawVariation {
        StateLawVariation::new(
            Self::state_id(),
            LegalTopic::JointAndSeveralLiability,
            StateRule::ModifiedJointAndSeveral {
                threshold_percent: 50,
            },
        )
        .with_statute(
            StatuteReference::new("Minn. Stat. § 604.02")
                .with_title("Minnesota Joint and Several Liability")
                .with_year(1988),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1988, 1, 1).unwrap())
        .with_notes(
            "Minnesota modified joint and several liability in 1988. Joint liability \
             applies if defendant is more than 50% at fault; otherwise several \
             liability applies.",
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
    fn test_minnesota_state_id() {
        let mn = MinnesotaLaw::state_id();
        assert_eq!(mn.code, "MN");
        assert_eq!(mn.name, "Minnesota");
    }

    #[test]
    fn test_modified_comparative_51() {
        let comp_neg = MinnesotaLaw::comparative_negligence();
        assert_eq!(comp_neg.rule, StateRule::ModifiedComparative51);
        assert_eq!(comp_neg.state.code, "MN");
        assert!(comp_neg.statutory_basis.is_some());
    }

    #[test]
    fn test_modified_joint_and_several() {
        let joint_several = MinnesotaLaw::joint_and_several_liability();
        if let StateRule::ModifiedJointAndSeveral { threshold_percent } = joint_several.rule {
            assert_eq!(threshold_percent, 50);
        } else {
            panic!("Expected ModifiedJointAndSeveral");
        }
        assert!(joint_several.statutory_basis.is_some());
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = MinnesotaLaw::state_variations();
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
        let comp_neg = MinnesotaLaw::comparative_negligence();
        assert_eq!(comp_neg.case_basis.len(), 1);
        let case = &comp_neg.case_basis[0];
        assert_eq!(case.short_name, "Milkovich v. Saari");
        assert_eq!(case.year, 1973);
        assert!(
            case.significance
                .as_ref()
                .expect("significance set")
                .contains("better law")
        );
    }

    #[test]
    fn test_better_law_jurisdiction_noted() {
        let comp_neg = MinnesotaLaw::comparative_negligence();
        assert!(comp_neg.notes.contains("better law"));
    }
}
