//! Ohio State Law
//!
//! Ohio tort law features:
//! - Modified Comparative Negligence (51% bar)
//! - Tort reform legislation with damage caps
//! - Midwest bellwether state
//! - Sixth Circuit federal appeals jurisdiction

use crate::states::types::{
    DamagesType, LegalTopic, LegalTradition, StateId, StateLawVariation, StateRule,
    StatuteReference,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Ohio state law system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OhioLaw;

impl OhioLaw {
    /// Get Ohio state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::new("OH", "Ohio", LegalTradition::CommonLaw)
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
            StatuteReference::new("Ohio Rev. Code § 2315.33")
                .with_title("Ohio Comparative Negligence")
                .with_year(1980),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1980, 1, 1).unwrap())
        .with_notes(
            "Ohio adopted modified comparative negligence with 51% bar in 1980. \
             Contributory fault does not bar recovery if plaintiff's fault is not \
             greater than the combined tortfeasor fault.",
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
            StatuteReference::new("Ohio Rev. Code § 2315.33")
                .with_title("Ohio Joint and Several Liability")
                .with_year(2005),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(2005, 4, 7).unwrap())
        .with_notes(
            "Ohio tort reform (2005) abolished joint and several liability. \
             Each defendant is liable only for their percentage of tortious conduct.",
        )
    }

    /// Damage caps variation.
    #[must_use]
    pub fn damage_caps() -> StateLawVariation {
        StateLawVariation::new(
            Self::state_id(),
            LegalTopic::DamagesCaps,
            StateRule::DamagesCap {
                damage_type: DamagesType::NonEconomic,
                cap_amount: 250_000,
                conditions: vec![
                    "Greater of $250,000 or 3x economic damages".to_string(),
                    "Maximum cap: $350,000 per plaintiff or $500,000 per occurrence".to_string(),
                ],
            },
        )
        .with_statute(
            StatuteReference::new("Ohio Rev. Code § 2315.18")
                .with_title("Ohio Tort Reform Act - Damage Caps")
                .with_year(2005),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(2005, 4, 7).unwrap())
        .with_notes(
            "Non-economic damages capped at greater of $250,000 or 3x economic damages, \
             with maximum limits of $350,000 per plaintiff or $500,000 per occurrence.",
        )
    }

    /// Get all state law variations.
    #[must_use]
    pub fn state_variations() -> Vec<StateLawVariation> {
        vec![
            Self::comparative_negligence(),
            Self::joint_and_several_liability(),
            Self::damage_caps(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ohio_state_id() {
        let oh = OhioLaw::state_id();
        assert_eq!(oh.code, "OH");
        assert_eq!(oh.name, "Ohio");
    }

    #[test]
    fn test_modified_comparative_51() {
        let comp_neg = OhioLaw::comparative_negligence();
        assert_eq!(comp_neg.rule, StateRule::ModifiedComparative51);
        assert_eq!(comp_neg.state.code, "OH");
        assert!(comp_neg.statutory_basis.is_some());
    }

    #[test]
    fn test_several_liability_only() {
        let joint_several = OhioLaw::joint_and_several_liability();
        assert_eq!(joint_several.rule, StateRule::SeveralLiabilityOnly);
        assert!(joint_several.statutory_basis.is_some());
    }

    #[test]
    fn test_damage_caps() {
        let caps = OhioLaw::damage_caps();
        if let StateRule::DamagesCap {
            damage_type,
            cap_amount,
            conditions,
        } = &caps.rule
        {
            assert_eq!(*damage_type, DamagesType::NonEconomic);
            assert_eq!(*cap_amount, 250_000);
            assert_eq!(conditions.len(), 2);
        } else {
            panic!("Expected DamagesCap");
        }
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = OhioLaw::state_variations();
        assert_eq!(variations.len(), 3);
        assert!(
            variations
                .iter()
                .any(|v| v.topic == LegalTopic::ComparativeNegligence)
        );
        assert!(
            variations
                .iter()
                .any(|v| v.topic == LegalTopic::DamagesCaps)
        );
    }
}
