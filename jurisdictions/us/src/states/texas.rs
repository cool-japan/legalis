//! Texas State Law Module
//!
//! Texas is the second-largest US state by population and economy.
//! Key features:
//! - **Modified Comparative Negligence - 51% bar** (Tex. Civ. Prac. & Rem. Code § 33.001)
//! - **Tort Reform** with medical malpractice damage caps
//! - **Community Property** state
//! - **No state income tax**
//! - Conservative approach to tort liability

use crate::states::types::{
    DamagesType, LegalTopic, StateId, StateLawVariation, StateRule, StatuteReference,
};
use chrono::NaiveDate;
use legalis_core::{Condition, Effect, EffectType, Statute};
use std::collections::HashMap;

/// Texas state law module.
pub struct TexasLaw;

impl TexasLaw {
    /// Get Texas state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::texas()
    }

    // ===== Tort Law =====

    /// Modified comparative negligence in Texas - 51% bar rule.
    ///
    /// Texas follows **modified comparative negligence with a 51% bar**.
    /// - Plaintiff can recover if they are 50% or less at fault
    /// - If plaintiff is 51% or more at fault, recovery is barred entirely
    ///
    /// This differs from:
    /// - **Pure comparative** (CA, NY, FL): No bar, recovery even if 99% at fault
    /// - **Modified 50%** (12 states): Plaintiff can recover if ≤50% at fault
    /// - **Contributory** (NC, VA, MD, DC, AL): Any plaintiff fault bars recovery
    ///
    /// ## Statutory Basis
    /// Tex. Civ. Prac. & Rem. Code § 33.001
    #[must_use]
    pub fn comparative_negligence() -> StateLawVariation {
        StateLawVariation::new(
            Self::state_id(),
            LegalTopic::ComparativeNegligence,
            StateRule::ModifiedComparative51,
        )
        .with_statute(
            StatuteReference::new("Tex. Civ. Prac. & Rem. Code § 33.001")
                .with_title("Proportionate Responsibility")
                .with_year(1995), // Part of 1995 tort reform
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1995, 9, 1).unwrap())
        .with_notes(
            "Texas adopted modified comparative negligence with 51% bar as part of \
             comprehensive tort reform in 1995. Prior law was contributory negligence \
             (complete bar if any plaintiff fault). The 51% bar strikes a middle ground \
             between pure comparative and contributory negligence.",
        )
    }

    /// Texas medical malpractice damage caps.
    ///
    /// Texas enacted strict caps on non-economic damages in medical malpractice cases
    /// as part of tort reform efforts.
    ///
    /// ## Caps (as of 2003 reform)
    /// - **$250,000** cap on non-economic damages per healthcare provider
    /// - **$500,000** total cap if multiple healthcare institutions involved
    /// - **No cap** on economic damages (medical bills, lost wages)
    ///
    /// ## Controversy
    /// Supporters argue caps reduce frivolous lawsuits and healthcare costs.
    /// Critics argue caps deny full compensation to severely injured patients.
    ///
    /// ## Statutory Basis
    /// Tex. Civ. Prac. & Rem. Code § 74.301
    #[must_use]
    pub fn medical_malpractice_caps() -> Statute {
        Statute::new(
            "tex-cprc-74-301",
            "Texas Medical Malpractice Non-Economic Damages Cap",
            Effect::new(
                EffectType::Prohibition,
                "Non-economic damages capped at $250,000 per healthcare provider",
            )
            .with_parameter("cap_amount", "250000")
            .with_parameter("cap_type", "non_economic_only")
            .with_parameter("economic_damages", "no_cap"),
        )
        .with_jurisdiction("US-TX")
        .with_version(1)
        .with_precondition(Condition::And(
            Box::new(Condition::AttributeEquals {
                key: "claim_type".to_string(),
                value: "medical_malpractice".to_string(),
            }),
            Box::new(Condition::AttributeEquals {
                key: "damages_type".to_string(),
                value: "non_economic".to_string(),
            }),
        ))
        .with_discretion(
            "Texas medical malpractice caps are controversial. Enacted in 2003 as part \
             of tort reform to address perceived 'lawsuit crisis.' Cap applies per \
             healthcare provider, so total can reach $500,000 if multiple institutions \
             involved. Economic damages (medical bills, lost income) have NO cap. \
             \n\nConstitutional challenges have been rejected by Texas Supreme Court.",
        )
    }

    /// Joint and several liability in Texas.
    ///
    /// Texas limits joint and several liability as part of tort reform.
    ///
    /// ## Rule
    /// - Defendants >50% at fault: Joint and several liability (liable for full amount)
    /// - Defendants ≤50% at fault: Several liability only (liable for own share)
    ///
    /// ## Purpose
    /// Prevents "deep pocket" defendants from being liable for full damages when
    /// they are only marginally at fault.
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
            StatuteReference::new("Tex. Civ. Prac. & Rem. Code § 33.013")
                .with_title("Proportionate Responsibility - Joint and Several Liability"),
        )
        .with_notes(
            "Modified joint and several liability: Defendants >50% at fault face joint \
             liability (liable for full damages). Defendants ≤50% at fault have several \
             liability only (liable for proportionate share). Prevents deep pocket liability.",
        )
    }

    /// Get damage caps by claim type.
    #[must_use]
    pub fn damage_caps() -> HashMap<&'static str, (DamagesType, u64)> {
        let mut caps = HashMap::new();

        caps.insert("medical_malpractice", (DamagesType::NonEconomic, 250_000));

        caps
    }

    /// Get all Texas state law variations.
    #[must_use]
    pub fn state_variations() -> Vec<StateLawVariation> {
        vec![
            Self::comparative_negligence(),
            Self::joint_and_several_liability(),
        ]
    }

    /// Get Texas tort reform statutes.
    #[must_use]
    pub fn tort_reform_statutes() -> Vec<Statute> {
        vec![Self::medical_malpractice_caps()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_texas_state_id() {
        let tx = TexasLaw::state_id();
        assert_eq!(tx.code, "TX");
        assert_eq!(tx.name, "Texas");
    }

    #[test]
    fn test_modified_comparative_51() {
        let variation = TexasLaw::comparative_negligence();

        assert_eq!(variation.state.code, "TX");
        assert_eq!(variation.topic, LegalTopic::ComparativeNegligence);
        assert_eq!(variation.rule, StateRule::ModifiedComparative51);
        assert!(variation.statutory_basis.is_some());

        let statute = variation.statutory_basis.unwrap();
        assert!(statute.citation.contains("33.001"));
    }

    #[test]
    fn test_medical_malpractice_caps() {
        let statute = TexasLaw::medical_malpractice_caps();

        assert_eq!(statute.id, "tex-cprc-74-301");
        assert_eq!(statute.jurisdiction, Some("US-TX".to_string()));
        assert!(!statute.preconditions.is_empty());
        assert!(statute.discretion_logic.is_some());
    }

    #[test]
    fn test_joint_and_several_modified() {
        let variation = TexasLaw::joint_and_several_liability();

        assert_eq!(variation.state.code, "TX");
        assert_eq!(variation.topic, LegalTopic::JointAndSeveralLiability);

        match variation.rule {
            StateRule::ModifiedJointAndSeveral { threshold_percent } => {
                assert_eq!(threshold_percent, 50);
            }
            _ => panic!("Expected ModifiedJointAndSeveral rule"),
        }
    }

    #[test]
    fn test_damage_caps_collection() {
        let caps = TexasLaw::damage_caps();

        assert!(caps.contains_key("medical_malpractice"));

        let (damage_type, amount) = caps.get("medical_malpractice").unwrap();
        assert_eq!(*damage_type, DamagesType::NonEconomic);
        assert_eq!(*amount, 250_000);
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = TexasLaw::state_variations();

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
    fn test_tort_reform_statutes_collection() {
        let statutes = TexasLaw::tort_reform_statutes();

        assert!(!statutes.is_empty());
        assert!(statutes.iter().any(|s| s.id.contains("74-301")));
    }

    #[test]
    fn test_texas_vs_california_comparison() {
        let tx = TexasLaw::comparative_negligence();
        let ca_rule = StateRule::PureComparativeNegligence;

        // Texas is modified (51% bar), California is pure (no bar)
        assert_eq!(tx.rule, StateRule::ModifiedComparative51);
        assert_ne!(tx.rule, ca_rule);
    }
}
