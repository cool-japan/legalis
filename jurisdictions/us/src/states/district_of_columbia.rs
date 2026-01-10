//! District of Columbia Law
//!
//! DC tort law features:
//! - Contributory Negligence (MINORITY RULE - only 5 jurisdictions)
//! - DC Circuit federal appeals jurisdiction
//! - Traditional common law approach
//! - Complete bar to recovery if plaintiff at fault

use crate::states::types::{
    LegalTopic, LegalTradition, StateId, StateLawVariation, StateRule, StatuteReference,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// District of Columbia law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistrictOfColumbiaLaw;

impl DistrictOfColumbiaLaw {
    /// Get District of Columbia state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::new("DC", "District of Columbia", LegalTradition::CommonLaw)
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
            StatuteReference::new("D.C. Code § 12-309")
                .with_title("District of Columbia Contributory Negligence")
                .with_year(1901),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1901, 1, 1).unwrap())
        .with_notes(
            "The District of Columbia retains the traditional contributory negligence rule. \
             Any contributory negligence by the plaintiff is a complete bar to \
             recovery, regardless of the defendant's degree of negligence. One of \
             only 5 US jurisdictions maintaining this minority rule (NC, VA, MD, AL, DC).",
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
            StatuteReference::new("D.C. Code § 16-2501")
                .with_title("District of Columbia Several Liability")
                .with_year(2012),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(2012, 1, 1).unwrap())
        .with_notes(
            "The District of Columbia abolished joint and several liability in 2012. \
             Each defendant is liable only for their proportionate share of damages \
             based on their percentage of fault.",
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
    fn test_district_of_columbia_state_id() {
        let dc = DistrictOfColumbiaLaw::state_id();
        assert_eq!(dc.code, "DC");
        assert_eq!(dc.name, "District of Columbia");
    }

    #[test]
    fn test_contributory_negligence_minority_rule() {
        let comp_neg = DistrictOfColumbiaLaw::comparative_negligence();
        assert_eq!(comp_neg.rule, StateRule::ContributoryNegligence);
        assert_eq!(comp_neg.state.code, "DC");
        assert!(comp_neg.statutory_basis.is_some());
        assert!(comp_neg.notes.contains("One of only 5 US jurisdictions"));
    }

    #[test]
    fn test_several_liability_only() {
        let joint_several = DistrictOfColumbiaLaw::joint_and_several_liability();
        assert_eq!(joint_several.rule, StateRule::SeveralLiabilityOnly);
        assert!(joint_several.statutory_basis.is_some());
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = DistrictOfColumbiaLaw::state_variations();
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
