//! California State Law Module
//!
//! California is the most populous US state and has significant influence on American law.
//! Key features:
//! - **Pure Comparative Negligence** (Li v. Yellow Cab, 1975)
//! - **Interest Analysis** for choice of law
//! - **CCPA** - California Consumer Privacy Act (unique)
//! - **Community Property** state
//! - Largest state economy ($3.9 trillion GDP)

use crate::states::types::{
    CaseReference, LegalTopic, StateId, StateLawVariation, StateRule, StatuteReference,
};
use chrono::NaiveDate;
use legalis_core::{
    Condition, Effect, EffectType, Statute,
    case_law::{Case, CaseRule, Court},
};

/// California state law module.
pub struct CaliforniaLaw;

impl CaliforniaLaw {
    /// Get California state ID.
    #[must_use]
    pub fn state_id() -> StateId {
        StateId::california()
    }

    // ===== Tort Law =====

    /// Pure comparative negligence rule adopted in California.
    ///
    /// California follows **pure comparative negligence** where damages are reduced by
    /// the plaintiff's percentage of fault, with no bar to recovery.
    /// Even if plaintiff is 99% at fault, they can still recover 1% of damages.
    ///
    /// ## Statutory Basis
    /// Cal. Civ. Code § 1714 - General duty of care
    ///
    /// ## Case Law Basis
    /// Li v. Yellow Cab Co., 13 Cal.3d 804 (1975) - Landmark case adopting pure comparative
    #[must_use]
    pub fn comparative_negligence() -> StateLawVariation {
        StateLawVariation::new(
            Self::state_id(),
            LegalTopic::ComparativeNegligence,
            StateRule::PureComparativeNegligence,
        )
        .with_statute(
            StatuteReference::new("Cal. Civ. Code § 1714")
                .with_title("Duty of Care")
                .with_year(1872),
        )
        .with_case(
            CaseReference::new(
                "Li v. Yellow Cab Co., 13 Cal.3d 804",
                "Li v. Yellow Cab",
                1975,
            )
            .with_significance(
                "Abolished contributory negligence in favor of pure comparative negligence",
            ),
        )
        .with_adoption_date(NaiveDate::from_ymd_opt(1975, 5, 29).unwrap())
        .with_notes(
            "Prior to Li (1975), California followed strict contributory negligence, \
             where any plaintiff fault barred recovery. The California Supreme Court \
             recognized this as unjust and adopted pure comparative negligence.",
        )
    }

    /// Li v. Yellow Cab Co., 13 Cal.3d 804 (1975)
    ///
    /// ## Facts
    /// Plaintiff Li was driving and made a left turn across oncoming traffic.
    /// Defendant Yellow Cab taxi struck her vehicle. Both drivers were negligent.
    ///
    /// ## Issue
    /// Should California retain contributory negligence (complete bar if plaintiff at fault)
    /// or adopt comparative negligence?
    ///
    /// ## Holding
    /// California Supreme Court abolished contributory negligence and adopted **pure
    /// comparative negligence**. Damages reduced by plaintiff's percentage of fault, no bar.
    ///
    /// ## Significance
    /// Major shift in tort law. California joined the growing comparative negligence movement.
    /// This case is cited nationwide as a model for tort reform.
    #[must_use]
    pub fn li_v_yellow_cab() -> Case {
        Case::new(
            "Li v. Yellow Cab Co., 13 Cal.3d 804, 532 P.2d 1226 (1975)",
            "Li v. Yellow Cab",
            1975,
            Court::Supreme, // California Supreme Court
            "US-CA",
        )
        .with_facts(
            "Plaintiff Li made a left turn across oncoming traffic and was struck by \
             defendant's taxi. Both drivers were found negligent. Under contributory \
             negligence doctrine, plaintiff's fault would bar all recovery.",
        )
        .with_issue(
            "Should California abolish contributory negligence in favor of comparative negligence?",
        )
        .with_holding(
            "Yes. California adopts pure comparative negligence. Plaintiff's recovery is \
             reduced by their percentage of fault, but not barred entirely.",
        )
        .with_ratio(
            "Contributory negligence is unjust - a plaintiff 1% at fault recovers nothing. \
             Pure comparative negligence better apportions loss according to fault. \
             Damages should be reduced by plaintiff's proportion of negligence.",
        )
        .with_rule(CaseRule {
            name: "Pure Comparative Negligence (California)".to_string(),
            conditions: vec![
                Condition::AttributeEquals {
                    key: "plaintiff_negligence".to_string(),
                    value: "true".to_string(),
                },
                Condition::AttributeEquals {
                    key: "defendant_negligence".to_string(),
                    value: "true".to_string(),
                },
                Condition::AttributeEquals {
                    key: "causation".to_string(),
                    value: "true".to_string(),
                },
                Condition::AttributeEquals {
                    key: "damages".to_string(),
                    value: "true".to_string(),
                },
            ],
            effect: Effect::new(
                EffectType::Obligation,
                "Damages reduced by plaintiff's percentage of fault (no bar)",
            )
            .with_parameter("rule", "pure_comparative")
            .with_parameter("no_bar", "true"),
            exceptions: vec![
                "Intentional torts (comparative negligence applies to negligence only)".to_string(),
                "Strict liability for abnormally dangerous activities".to_string(),
            ],
        })
    }

    /// Summers v. Tice, 33 Cal.2d 80 (1948)
    ///
    /// ## Facts
    /// Plaintiff was hunting with two defendants. Both defendants negligently fired in
    /// plaintiff's direction simultaneously. A pellet struck plaintiff in the eye, but
    /// it was impossible to determine which defendant's gun fired the injuring pellet.
    ///
    /// ## Holding
    /// Burden of proof shifts to defendants to prove they did not cause the injury.
    /// If both cannot exculpate themselves, both are jointly liable.
    ///
    /// ## Legal Doctrine: Alternative Liability
    /// When two or more negligent actors create a situation where it's impossible to
    /// determine which one caused the harm, the burden shifts to defendants.
    ///
    /// ## Influence
    /// This case influenced the development of **market share liability** in
    /// Sindell v. Abbott Laboratories (DES drug case).
    #[must_use]
    pub fn summers_v_tice() -> Case {
        Case::new(
            "Summers v. Tice, 33 Cal.2d 80, 199 P.2d 1 (1948)",
            "Summers v. Tice",
            1948,
            Court::Supreme,
            "US-CA",
        )
        .with_facts(
            "Plaintiff hunting with two defendants. Both fired negligently in plaintiff's \
             direction. One pellet struck plaintiff in the eye. Impossible to determine \
             which defendant's gun fired the injuring pellet.",
        )
        .with_issue(
            "How to determine liability when two negligent actors might have caused injury, \
             but causation cannot be proven as to either?",
        )
        .with_holding(
            "Burden of proof shifts to defendants to exculpate themselves. If neither can \
             prove they did not cause injury, both are jointly liable.",
        )
        .with_ratio(
            "When plaintiff cannot identify which of two negligent defendants caused injury \
             due to circumstances created by defendants, fairness requires shifting burden \
             to defendants. Otherwise, wrongdoers would escape liability due to uncertainty \
             they created.",
        )
        .with_rule(CaseRule {
            name: "Alternative Liability (Summers v. Tice)".to_string(),
            conditions: vec![
                Condition::AttributeEquals {
                    key: "multiple_defendants_negligent".to_string(),
                    value: "true".to_string(),
                },
                Condition::AttributeEquals {
                    key: "injury_caused_by_one".to_string(),
                    value: "true".to_string(),
                },
                Condition::AttributeEquals {
                    key: "cannot_identify_which_defendant".to_string(),
                    value: "true".to_string(),
                },
            ],
            effect: Effect::new(
                EffectType::Obligation,
                "Burden shifts to defendants to prove non-causation",
            )
            .with_parameter("liability", "joint and several if cannot exculpate"),
            exceptions: vec![
                "One defendant successfully proves they did not cause injury".to_string(),
                "Plaintiff has other means to identify actual tortfeasor".to_string(),
            ],
        })
    }

    /// Thing v. La Chusa, 48 Cal.3d 644 (1989)
    ///
    /// ## Issue: Negligent Infliction of Emotional Distress (NIED)
    ///
    /// When can a plaintiff recover for emotional distress from witnessing injury to another?
    ///
    /// ## Holding
    /// California adopts strict requirements for "bystander NIED":
    /// 1. Plaintiff must be closely related to victim
    /// 2. Plaintiff must be present at scene of accident
    /// 3. Plaintiff must suffer emotional distress beyond that of disinterested witness
    ///
    /// ## Significance
    /// Limits emotional distress claims to prevent unlimited liability.
    #[must_use]
    pub fn thing_v_la_chusa() -> Case {
        Case::new(
            "Thing v. La Chusa, 48 Cal.3d 644, 771 P.2d 814 (1989)",
            "Thing v. La Chusa",
            1989,
            Court::Supreme,
            "US-CA",
        )
        .with_facts(
            "Plaintiff's child was struck by defendant's car. Plaintiff was not present at \
             scene but arrived shortly after and saw injured child. Plaintiff sought damages \
             for emotional distress from witnessing aftermath.",
        )
        .with_issue(
            "Can plaintiff recover for negligent infliction of emotional distress when not \
             present at time of injury?",
        )
        .with_holding(
            "No. California requires plaintiff to be present at scene of accident to recover \
             for bystander NIED.",
        )
        .with_ratio(
            "To prevent unlimited liability, NIED claims require: (1) close relationship, \
             (2) presence at scene, (3) contemporaneous sensory observation of injury.",
        )
        .with_rule(CaseRule {
            name: "Bystander NIED (Thing Factors)".to_string(),
            conditions: vec![
                Condition::And(
                    Box::new(Condition::And(
                        Box::new(Condition::AttributeEquals {
                            key: "close_relationship_to_victim".to_string(),
                            value: "true".to_string(),
                        }),
                        Box::new(Condition::AttributeEquals {
                            key: "present_at_scene".to_string(),
                            value: "true".to_string(),
                        }),
                    )),
                    Box::new(Condition::AttributeEquals {
                        key: "contemporaneous_sensory_observation".to_string(),
                        value: "true".to_string(),
                    }),
                ),
                Condition::AttributeEquals {
                    key: "severe_emotional_distress".to_string(),
                    value: "true".to_string(),
                },
            ],
            effect: Effect::new(
                EffectType::Obligation,
                "Recovery for negligent infliction of emotional distress",
            ),
            exceptions: vec![
                "Direct victim (not bystander) - different standards apply".to_string(),
                "Intentional infliction of emotional distress".to_string(),
            ],
        })
    }

    // ===== Privacy Law =====

    /// California Consumer Privacy Act (CCPA) - Cal. Civ. Code § 1798.100 et seq.
    ///
    /// ## Overview
    /// Enacted in 2018, CCPA is the strongest state-level privacy protection in the US,
    /// modeled after EU GDPR. Gives California residents rights over their personal data.
    ///
    /// ## Key Rights
    /// - Right to know what personal information is collected
    /// - Right to delete personal information
    /// - Right to opt-out of sale of personal information
    /// - Right to non-discrimination for exercising rights
    ///
    /// ## Applicability
    /// Applies to businesses that:
    /// - Have gross revenues > $25 million, OR
    /// - Process data of 100,000+ consumers/households, OR
    /// - Derive 50%+ of revenue from selling consumer data
    ///
    /// ## Influence
    /// CCPA influenced other states to enact similar privacy laws (Virginia CDPA, Colorado CPA).
    #[must_use]
    pub fn ccpa_statute() -> Statute {
        Statute::new(
            "ca-ccpa-1798-100",
            "California Consumer Privacy Act (CCPA)",
            Effect::new(
                EffectType::Grant,
                "California residents have rights over personal information",
            )
            .with_parameter("right_to_know", "true")
            .with_parameter("right_to_delete", "true")
            .with_parameter("right_to_opt_out", "true")
            .with_parameter("right_to_non_discrimination", "true"),
        )
        .with_jurisdiction("US-CA")
        .with_version(1)
        .with_precondition(Condition::Or(
            Box::new(Condition::Or(
                Box::new(Condition::AttributeEquals {
                    key: "business_revenue_exceeds_25m".to_string(),
                    value: "true".to_string(),
                }),
                Box::new(Condition::AttributeEquals {
                    key: "processes_100k_consumers".to_string(),
                    value: "true".to_string(),
                }),
            )),
            Box::new(Condition::AttributeEquals {
                key: "revenue_50pct_from_data_sales".to_string(),
                value: "true".to_string(),
            }),
        ))
        .with_discretion(
            "CCPA is unique to California and represents the strongest state-level privacy \
             protection in the US. Influenced by EU GDPR. Businesses subject to CCPA must \
             provide notice, honor deletion requests, and allow opt-out of data sales. \
             \n\nEnforcement: California Attorney General + private right of action for data breaches.",
        )
    }

    // ===== Utility Functions =====

    /// Get all major California landmark cases.
    #[must_use]
    pub fn landmark_cases() -> Vec<Case> {
        vec![
            Self::li_v_yellow_cab(),
            Self::summers_v_tice(),
            Self::thing_v_la_chusa(),
        ]
    }

    /// Get all California state law variations for comparison.
    #[must_use]
    pub fn state_variations() -> Vec<StateLawVariation> {
        vec![Self::comparative_negligence()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_california_state_id() {
        let ca = CaliforniaLaw::state_id();
        assert_eq!(ca.code, "CA");
        assert_eq!(ca.name, "California");
    }

    #[test]
    fn test_comparative_negligence_variation() {
        let variation = CaliforniaLaw::comparative_negligence();

        assert_eq!(variation.state.code, "CA");
        assert_eq!(variation.topic, LegalTopic::ComparativeNegligence);
        assert_eq!(variation.rule, StateRule::PureComparativeNegligence);
        assert!(variation.statutory_basis.is_some());
        assert!(!variation.case_basis.is_empty());
        assert!(variation.adoption_date.is_some());
    }

    #[test]
    fn test_li_v_yellow_cab_case() {
        let case = CaliforniaLaw::li_v_yellow_cab();

        assert_eq!(case.short_name, "Li v. Yellow Cab");
        assert_eq!(case.year, 1975);
        assert_eq!(case.jurisdiction, "US-CA");
        assert_eq!(case.court, Court::Supreme);
        assert!(case.rule.is_some());

        let rule = case.rule.unwrap();
        assert!(rule.name.contains("Pure Comparative"));
    }

    #[test]
    fn test_summers_v_tice_alternative_liability() {
        let case = CaliforniaLaw::summers_v_tice();

        assert_eq!(case.year, 1948);
        assert!(case.rule.is_some());

        let rule = case.rule.unwrap();
        assert!(rule.name.contains("Alternative Liability"));
        assert_eq!(rule.conditions.len(), 3);
    }

    #[test]
    fn test_thing_v_la_chusa_nied() {
        let case = CaliforniaLaw::thing_v_la_chusa();

        assert_eq!(case.year, 1989);
        assert!(case.rule.is_some());

        let rule = case.rule.unwrap();
        assert!(rule.name.contains("NIED"));
    }

    #[test]
    fn test_ccpa_statute() {
        let ccpa = CaliforniaLaw::ccpa_statute();

        assert_eq!(ccpa.id, "ca-ccpa-1798-100");
        assert_eq!(ccpa.jurisdiction, Some("US-CA".to_string()));
        assert!(!ccpa.preconditions.is_empty());
        assert!(ccpa.discretion_logic.is_some());
    }

    #[test]
    fn test_landmark_cases_collection() {
        let cases = CaliforniaLaw::landmark_cases();

        assert_eq!(cases.len(), 3);
        assert!(cases.iter().any(|c| c.short_name == "Li v. Yellow Cab"));
        assert!(cases.iter().any(|c| c.short_name == "Summers v. Tice"));
        assert!(cases.iter().any(|c| c.short_name == "Thing v. La Chusa"));
    }

    #[test]
    fn test_state_variations_collection() {
        let variations = CaliforniaLaw::state_variations();

        assert!(!variations.is_empty());
        assert!(
            variations
                .iter()
                .any(|v| v.topic == LegalTopic::ComparativeNegligence)
        );
    }
}
