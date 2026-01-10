//! Florida State Law Module
//!
//! Florida is the third-largest US state by population.
//! Key features:
//! - **Pure Comparative Negligence** (Fla. Stat. § 768.81)
//! - **Stand Your Ground Law** (Fla. Stat. § 776.032) - controversial self-defense statute
//! - **No state income tax**
//! - Large retiree population affecting legal landscape
//! - Significant tourism industry

use crate::states::types::{LegalTopic, StateId, StateLawVariation, StateRule, StatuteReference};
use chrono::NaiveDate;
use legalis_core::{Condition, Effect, EffectType, Statute};

/// Florida state law module.
pub struct FloridaLaw;

impl FloridaLaw {
    /// Get Florida state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::florida()
    }

    // ===== Tort Law =====

    /// Pure comparative negligence in Florida.
    ///
    /// Florida follows **pure comparative negligence** like California and New York.
    /// Damages are reduced by plaintiff's percentage of fault, with no bar to recovery.
    ///
    /// Even if plaintiff is 99% at fault, they can still recover 1% of damages.
    ///
    /// ## Statutory Basis
    /// Fla. Stat. § 768.81
    #[must_use]
    pub fn comparative_negligence() -> StateLawVariation {
        StateLawVariation::new(
            Self::state_id(),
            LegalTopic::ComparativeNegligence,
            StateRule::PureComparativeNegligence,
        )
        .with_statute(
            StatuteReference::new("Fla. Stat. § 768.81")
                .with_title("Comparative Fault")
                .with_year(1973),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1973, 7, 1).unwrap())
        .with_notes(
            "Florida adopted pure comparative negligence in 1973, earlier than California \
             (1975) and New York (1975). Prior law was contributory negligence. \
             Florida Statute § 768.81 explicitly codifies pure comparative fault.",
        )
    }

    // ===== Criminal Law / Self-Defense =====

    /// Florida Stand Your Ground Law - Fla. Stat. § 776.032
    ///
    /// ## Overview
    /// Florida's Stand Your Ground law removes the duty to retreat before using deadly force
    /// in self-defense. This is controversial and differs from traditional self-defense law.
    ///
    /// ## Traditional Self-Defense (Common Law)
    /// - Duty to retreat if safely possible (except in one's home - "Castle Doctrine")
    /// - Deadly force only if no safe retreat available
    ///
    /// ## Florida Stand Your Ground (2005)
    /// - **No duty to retreat** anywhere person has legal right to be
    /// - May use deadly force if reasonably believes necessary to prevent:
    ///   - Death or great bodily harm, OR
    ///   - Commission of forcible felony
    /// - **Immunity from prosecution** if justified
    ///
    /// ## Controversy
    /// Critics argue it encourages confrontation and vigilantism.
    /// Supporters argue it protects innocent victims from prosecution.
    ///
    /// ## Influence
    /// Many states adopted similar laws after Florida (2005). Now ~30 states have
    /// Stand Your Ground laws, though specifics vary.
    ///
    /// ## Famous Cases
    /// - Trayvon Martin case (2012) - Brought national attention
    #[must_use]
    pub fn stand_your_ground() -> Statute {
        Statute::new(
            "fla-stat-776-032",
            "Florida Stand Your Ground Law",
            Effect::new(
                EffectType::Grant,
                "Immunity from prosecution for justified use of force",
            )
            .with_parameter("no_duty_to_retreat", "true")
            .with_parameter("burden_shifts_to_state", "true")
            .with_parameter("immunity", "criminal_and_civil"),
        )
        .with_jurisdiction("US-FL")
        .with_version(1)
        .with_precondition(Condition::And(
            Box::new(Condition::AttributeEquals {
                key: "legal_right_to_be_present".to_string(),
                value: "true".to_string(),
            }),
            Box::new(Condition::Or(
                Box::new(Condition::AttributeEquals {
                    key: "reasonable_belief_death_or_harm".to_string(),
                    value: "true".to_string(),
                }),
                Box::new(Condition::AttributeEquals {
                    key: "reasonable_belief_forcible_felony".to_string(),
                    value: "true".to_string(),
                }),
            )),
        ))
        .with_discretion(
            "Florida's Stand Your Ground law (enacted 2005) is controversial. Removes duty to \
             retreat before using deadly force in self-defense. Person has no duty to retreat \
             if they have legal right to be in that location. \
             \n\nKey provisions: \
             \n- No duty to retreat anywhere person has right to be \
             \n- May use deadly force if reasonably believes necessary to prevent death/great bodily harm \
             \n- Immunity from criminal prosecution AND civil suit if justified \
             \n- Burden shifts to state to prove use of force was NOT justified \
             \n\nThis differs from traditional Common Law self-defense requiring retreat if safely possible. \
             \n\nCritics: Encourages confrontation, vigilantism, disproportionate impact on minorities \
             \nSupporters: Protects victims, deters criminals, affirms natural right to self-defense",
        )
    }

    /// Get all Florida state law variations.
    #[must_use]
    pub fn state_variations() -> Vec<StateLawVariation> {
        vec![Self::comparative_negligence()]
    }

    /// Get Florida-specific statutes.
    #[must_use]
    pub fn florida_statutes() -> Vec<Statute> {
        vec![Self::stand_your_ground()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_florida_state_id() {
        let fl = FloridaLaw::state_id();
        assert_eq!(fl.code, "FL");
        assert_eq!(fl.name, "Florida");
    }

    #[test]
    fn test_pure_comparative_negligence() {
        let variation = FloridaLaw::comparative_negligence();

        assert_eq!(variation.state.code, "FL");
        assert_eq!(variation.topic, LegalTopic::ComparativeNegligence);
        assert_eq!(variation.rule, StateRule::PureComparativeNegligence);
        assert!(variation.statutory_basis.is_some());

        let statute = variation.statutory_basis.unwrap();
        assert!(statute.citation.contains("768.81"));

        // Florida adopted pure comparative in 1973 (earlier than CA/NY)
        assert_eq!(
            variation.adoption_date,
            Some(NaiveDate::from_ymd_opt(1973, 7, 1).unwrap())
        );
    }

    #[test]
    fn test_stand_your_ground() {
        let statute = FloridaLaw::stand_your_ground();

        assert_eq!(statute.id, "fla-stat-776-032");
        assert_eq!(statute.jurisdiction, Some("US-FL".to_string()));
        assert!(!statute.preconditions.is_empty());
        assert!(statute.discretion_logic.is_some());

        let discretion = statute.discretion_logic.unwrap();
        assert!(discretion.contains("controversial"));
        assert!(discretion.contains("no duty to retreat"));
    }

    #[test]
    fn test_state_variations() {
        let variations = FloridaLaw::state_variations();

        assert!(!variations.is_empty());
        assert!(
            variations
                .iter()
                .any(|v| v.topic == LegalTopic::ComparativeNegligence)
        );
    }

    #[test]
    fn test_florida_statutes() {
        let statutes = FloridaLaw::florida_statutes();

        assert!(!statutes.is_empty());
        assert!(
            statutes
                .iter()
                .any(|s| s.id.contains("stand-your-ground") || s.id.contains("776"))
        );
    }

    #[test]
    fn test_florida_vs_california() {
        // Both have pure comparative negligence
        let fl = FloridaLaw::comparative_negligence();

        assert_eq!(fl.rule, StateRule::PureComparativeNegligence);

        // But Florida adopted it earlier (1973 vs 1975)
        assert_eq!(
            fl.adoption_date,
            Some(NaiveDate::from_ymd_opt(1973, 7, 1).unwrap())
        );
    }
}
