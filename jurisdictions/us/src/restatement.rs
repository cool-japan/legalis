//! Restatement of Torts (American Law Institute)
//!
//! The Restatement of Torts is a treatise published by the American Law Institute (ALI)
//! that synthesizes Common Law principles from court decisions across the United States.
//! While not binding law, it is **highly persuasive** authority cited by courts.
//!
//! ## Historical Context
//!
//! - **Restatement (First) of Torts** (1934-1939): Original compilation
//! - **Restatement (Second) of Torts** (1965-1979): Major revision
//! - **Restatement (Third) of Torts** (ongoing): Modern update in specific areas
//!
//! ## Common Law vs Restatement
//!
//! The Restatement is **not a statute**. It is a distillation of judicial decisions:
//!
//! ```text
//! Case 1 → Rule A                  Restatement
//! Case 2 → Rule A (variant)  ====> § 158: Battery
//! Case 3 → Rule A (refined)        (synthesized rule)
//!         ↓
//! Case 4 cites Restatement § 158
//! ```
//!
//! Courts may:
//! - **Adopt** Restatement sections as controlling law
//! - **Reject** Restatement in favor of local precedent
//! - **Modify** Restatement to fit local policy
//!
//! ## Comparison with Civil Law Codes
//!
//! | Civil Law Code | Common Law Restatement |
//! |----------------|------------------------|
//! | Enacted by legislature | Written by scholars/judges |
//! | Binding authority | Persuasive authority |
//! | Prospective (future cases) | Retrospective (past cases) |
//! | Abstract principles | Concrete rules from cases |
//!
//! ## Modeling Strategy
//!
//! We model Restatement sections as **CaseRule** objects rather than Statutes,
//! since they represent distilled common law principles, not legislative enactments.

use legalis_core::{Condition, Effect, EffectType, Statute, case_law::CaseRule};

/// Restatement (Second) of Torts § 158 - Battery (暴行罪)
///
/// ## Black Letter Law
///
/// > An actor is subject to liability to another for battery if
/// > (a) he acts intending to cause a harmful or offensive contact
/// >     with the person of the other or a third person, or an
/// >     imminent apprehension of such a contact, and
/// > (b) a harmful contact with the person of the other directly
/// >     or indirectly results.
///
/// ## Elements (Conditions)
///
/// 1. **Act**: Voluntary movement by defendant
/// 2. **Intent**: Purpose to cause harmful/offensive contact OR substantial certainty
/// 3. **Contact**: Harmful or offensive touching
/// 4. **Causation**: Act caused the contact
///
/// ## Famous Cases Applying This Rule
///
/// - **Vosburg v. Putney**, 80 Wis. 523 (1891): Kick in classroom
/// - **Garratt v. Dailey**, 279 P.2d 1091 (1955): Child pulls chair away
///
/// ## Comparison with Other Systems
///
/// - **Japan (民法709条)**: Requires "intent OR negligence" - broader
/// - **Germany (BGB §823)**: Lists "Körper" (body) as protected interest
/// - **France (Code civil 1240)**: "Faute" covers battery within general tort
///
/// Battery is narrower than general tort law because it requires **intent**,
/// not mere negligence.
#[must_use]
pub fn section_158_battery() -> CaseRule {
    CaseRule {
        name: "Restatement (Second) of Torts § 158 - Battery".to_string(),
        conditions: vec![
            // Element 1: Voluntary act
            Condition::AttributeEquals {
                key: "voluntary_act".to_string(),
                value: "true".to_string(),
            },
            // Element 2: Intent to cause contact OR imminent apprehension
            Condition::Or(
                Box::new(Condition::AttributeEquals {
                    key: "intent_harmful_contact".to_string(),
                    value: "true".to_string(),
                }),
                Box::new(Condition::AttributeEquals {
                    key: "intent_offensive_contact".to_string(),
                    value: "true".to_string(),
                }),
            ),
            // Element 3: Harmful contact occurred
            Condition::AttributeEquals {
                key: "harmful_contact".to_string(),
                value: "true".to_string(),
            },
            // Element 4: Causation
            Condition::AttributeEquals {
                key: "causation".to_string(),
                value: "true".to_string(),
            },
        ],
        effect: Effect::new(
            EffectType::Obligation,
            "Liability for battery - compensatory and potentially punitive damages",
        )
        .with_parameter("tortfeasor", "actor who caused harmful contact")
        .with_parameter("victim", "person subjected to harmful contact")
        .with_parameter("remedy", "damages (compensatory + punitive if malicious)"),
        exceptions: vec![
            "Consent (express or implied)".to_string(),
            "Self-defense (reasonable force)".to_string(),
            "Defense of others".to_string(),
            "Parental discipline (reasonable)".to_string(),
            "Arrest by law enforcement (lawful)".to_string(),
        ],
    }
}

/// Restatement (Second) of Torts § 46 - Intentional Infliction of Emotional Distress (IIED)
///
/// ## Black Letter Law
///
/// > One who by extreme and outrageous conduct intentionally or recklessly
/// > causes severe emotional distress to another is subject to liability
/// > for such emotional distress, and if bodily harm to the other results
/// > from it, for such bodily harm.
///
/// ## Elements
///
/// 1. **Extreme and outrageous conduct**: Beyond all bounds of decency
/// 2. **Intent or recklessness**: Purpose OR knowing disregard of high probability
/// 3. **Severe emotional distress**: More than reasonable person could bear
/// 4. **Causation**: Conduct caused the distress
///
/// ## Comment d - "Outrageous" Standard
///
/// > Liability has been found only where the conduct has been so outrageous
/// > in character, and so extreme in degree, as to go beyond all possible
/// > bounds of decent behavior, and to be regarded as atrocious, and utterly
/// > intolerable in a civilized community.
///
/// ## Famous Cases
///
/// - **Harris v. Jones**, 281 Md. 560 (1977): Mockery of stuttering employee
/// - **Hustler Magazine v. Falwell**, 485 U.S. 46 (1988): Parody ad
///
/// ## Comparison with Civil Law
///
/// This tort is **unique to Common Law**. Civil law systems handle this through:
/// - **Japan**: Article 710 (non-pecuniary damages within general tort)
/// - **Germany**: § 823(1) "Gesundheit" (health) + § 253 (pain and suffering)
/// - **France**: Article 1240 "dommage moral" (moral damage)
///
/// The Common Law developed a separate tort because traditionally,
/// emotional distress alone was not compensable (no physical injury required).
#[must_use]
pub fn section_46_iied() -> CaseRule {
    CaseRule {
        name: "Restatement (Second) of Torts § 46 - IIED".to_string(),
        conditions: vec![
            // Element 1: Extreme and outrageous conduct
            Condition::AttributeEquals {
                key: "extreme_outrageous_conduct".to_string(),
                value: "true".to_string(),
            },
            // Element 2: Intent or recklessness
            Condition::Or(
                Box::new(Condition::AttributeEquals {
                    key: "intent_emotional_distress".to_string(),
                    value: "true".to_string(),
                }),
                Box::new(Condition::AttributeEquals {
                    key: "recklessness".to_string(),
                    value: "true".to_string(),
                }),
            ),
            // Element 3: Severe emotional distress
            Condition::AttributeEquals {
                key: "severe_emotional_distress".to_string(),
                value: "true".to_string(),
            },
            // Element 4: Causation
            Condition::AttributeEquals {
                key: "causation".to_string(),
                value: "true".to_string(),
            },
        ],
        effect: Effect::new(
            EffectType::Obligation,
            "Liability for intentional infliction of emotional distress",
        )
        .with_parameter("defendant", "actor who engaged in outrageous conduct")
        .with_parameter(
            "remedy",
            "damages for emotional distress + bodily harm if resulted",
        ),
        exceptions: vec![
            "Conduct not sufficiently outrageous (mere insults, indignities)".to_string(),
            "Emotional distress not severe (transient upset)".to_string(),
            "First Amendment protection (public figures, matters of public concern)".to_string(),
        ],
    }
}

/// Restatement (Second) of Torts § 402A - Strict Liability for Defective Products
///
/// ## Black Letter Law
///
/// > (1) One who sells any product in a defective condition unreasonably
/// >     dangerous to the user or consumer or to his property is subject
/// >     to liability for physical harm thereby caused to the ultimate
/// >     user or consumer, or to his property, if
/// >     (a) the seller is engaged in the business of selling such a product, and
/// >     (b) it is expected to and does reach the user or consumer without
/// >         substantial change in the condition in which it is sold.
/// > (2) The rule stated in Subsection (1) applies although
/// >     (a) the seller has exercised all possible care in the preparation
/// >         and sale of his product, and
/// >     (b) the user or consumer has not bought the product from or entered
/// >         into any contractual relation with the seller.
///
/// ## Revolutionary Aspect
///
/// This section established **strict liability** - NO FAULT REQUIRED.
/// The seller is liable even if they exercised "all possible care."
///
/// This was a major departure from negligence-based tort law.
///
/// ## Famous Cases
///
/// - **Greenman v. Yuba Power Products**, 377 P.2d 897 (1963): Defective power tool
/// - **Escola v. Coca Cola Bottling Co.**, 150 P.2d 436 (1944): Exploding bottle
///   (Traynor, J., concurring - laid groundwork for § 402A)
///
/// ## Comparison with Other Systems
///
/// | System | Product Liability Basis |
/// |--------|-------------------------|
/// | 🇺🇸 US | Strict liability (§ 402A) |
/// | 🇯🇵 Japan |製造物責任法 (Product Liability Act 1994) - statutory |
/// | 🇩🇪 Germany | Produkthaftungsgesetz (1989) - EU Directive |
/// | 🇫🇷 France | Article 1245 Code civil - EU Directive |
///
/// The US developed strict liability through **case law** (Common Law method),
/// while Civil Law countries enacted **statutes** (Civil Law method).
#[must_use]
pub fn section_402a_products_liability() -> CaseRule {
    CaseRule {
        name: "Restatement (Second) of Torts § 402A - Products Liability".to_string(),
        conditions: vec![
            // Element 1: Seller engaged in business of selling product
            Condition::AttributeEquals {
                key: "commercial_seller".to_string(),
                value: "true".to_string(),
            },
            // Element 2: Product in defective condition
            Condition::Or(
                Box::new(Condition::Or(
                    Box::new(Condition::AttributeEquals {
                        key: "manufacturing_defect".to_string(),
                        value: "true".to_string(),
                    }),
                    Box::new(Condition::AttributeEquals {
                        key: "design_defect".to_string(),
                        value: "true".to_string(),
                    }),
                )),
                Box::new(Condition::AttributeEquals {
                    key: "warning_defect".to_string(),
                    value: "true".to_string(),
                }),
            ),
            // Element 3: Unreasonably dangerous
            Condition::AttributeEquals {
                key: "unreasonably_dangerous".to_string(),
                value: "true".to_string(),
            },
            // Element 4: Product reached consumer without substantial change
            Condition::AttributeEquals {
                key: "no_substantial_change".to_string(),
                value: "true".to_string(),
            },
            // Element 5: Physical harm caused
            Condition::AttributeEquals {
                key: "physical_harm".to_string(),
                value: "true".to_string(),
            },
            // Element 6: Causation
            Condition::AttributeEquals {
                key: "causation".to_string(),
                value: "true".to_string(),
            },
        ],
        effect: Effect::new(
            EffectType::Obligation,
            "Strict liability for defective products (no fault required)",
        )
        .with_parameter("liable_party", "commercial seller in chain of distribution")
        .with_parameter("remedy", "compensatory damages for physical harm"),
        exceptions: vec![
            "Product substantially altered after sale".to_string(),
            "Misuse by plaintiff (unforeseeable)".to_string(),
            "Assumption of risk (knowing and voluntary)".to_string(),
            "Obvious danger (open and obvious to reasonable user)".to_string(),
            "Comparative/contributory negligence (jurisdiction-dependent)".to_string(),
        ],
    }
}

/// Creates a statute-like representation for a Restatement section.
///
/// While Restatement sections are not statutes, we can represent them
/// as Statute objects for simulation purposes, marking them as
/// "Restatement" jurisdiction to distinguish from legislative enactments.
#[must_use]
pub fn battery_as_statute() -> Statute {
    let rule = section_158_battery();
    Statute::new("restatement-2d-torts-158", &rule.name, rule.effect.clone())
        .with_jurisdiction("US-RESTATEMENT")
        .with_version(2) // Restatement (Second)
        .with_precondition(rule.conditions[0].clone())
        .with_precondition(rule.conditions[1].clone())
        .with_precondition(rule.conditions[2].clone())
        .with_precondition(rule.conditions[3].clone())
        .with_discretion(
            "Battery requires INTENT (purpose or substantial certainty), not mere negligence. \
            The contact must be harmful OR offensive to a reasonable person. \
            Transferred intent applies: intent to batter A but hitting B still counts. \
            \n\n【Common Law特徴】\
            \nBattery is an intentional tort - distinct from negligence-based torts. \
            This categorical distinction is central to Common Law but less emphasized in Civil Law.",
        )
}

#[must_use]
pub fn iied_as_statute() -> Statute {
    let rule = section_46_iied();
    Statute::new("restatement-2d-torts-46", &rule.name, rule.effect.clone())
        .with_jurisdiction("US-RESTATEMENT")
        .with_version(2)
        .with_precondition(rule.conditions[0].clone())
        .with_precondition(rule.conditions[1].clone())
        .with_precondition(rule.conditions[2].clone())
        .with_precondition(rule.conditions[3].clone())
        .with_discretion(
            "The 'outrageous' standard is VERY HIGH - courts reject most IIED claims. \
            Mere insults, indignities, annoyances, petty oppressions do NOT suffice. \
            The conduct must be 'atrocious' and 'utterly intolerable in a civilized community.' \
            This is a quintessentially discretionary determination requiring judicial evaluation. \
            \n\n【比較法的考察】\
            \nIIED developed in Common Law because emotional distress alone was historically \
            not compensable (parasitic damages only). Civil Law systems had no such barrier.",
        )
}

#[must_use]
pub fn products_liability_as_statute() -> Statute {
    let rule = section_402a_products_liability();
    Statute::new("restatement-2d-torts-402a", &rule.name, rule.effect.clone())
        .with_jurisdiction("US-RESTATEMENT")
        .with_version(2)
        .with_precondition(rule.conditions[0].clone())
        .with_precondition(rule.conditions[1].clone())
        .with_precondition(rule.conditions[2].clone())
        .with_precondition(rule.conditions[3].clone())
        .with_precondition(rule.conditions[4].clone())
        .with_precondition(rule.conditions[5].clone())
        .with_discretion(
            "§ 402A represents a shift from negligence to strict liability in product cases. \
            The plaintiff need NOT prove the seller was negligent - defect + causation suffice. \
            'Unreasonably dangerous' is evaluated under consumer expectations OR risk-utility. \
            Restatement (Third) later refined this into three defect categories. \
            \n\n【立法政策】\
            \nThis Common Law development (through cases like Greenman) preceded Civil Law \
            statutory regimes (EU Product Liability Directive 1985). \
            Case law → Restatement → Widespread adoption is the Common Law path.",
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_section_158_battery() {
        let rule = section_158_battery();
        assert_eq!(rule.name, "Restatement (Second) of Torts § 158 - Battery");
        assert_eq!(rule.conditions.len(), 4);
        assert!(rule.exceptions.len() >= 3);
    }

    #[test]
    fn test_section_46_iied() {
        let rule = section_46_iied();
        assert_eq!(rule.name, "Restatement (Second) of Torts § 46 - IIED");
        assert_eq!(rule.conditions.len(), 4);
        assert!(!rule.exceptions.is_empty());
    }

    #[test]
    fn test_section_402a_products() {
        let rule = section_402a_products_liability();
        assert!(rule.name.contains("402A"));
        assert_eq!(rule.conditions.len(), 6); // More conditions than fault-based torts
        assert!(rule.exceptions.len() >= 4);
    }

    #[test]
    fn test_battery_as_statute() {
        let statute = battery_as_statute();
        assert_eq!(statute.id, "restatement-2d-torts-158");
        assert_eq!(statute.jurisdiction, Some("US-RESTATEMENT".to_string()));
        assert_eq!(statute.version, 2);
        assert_eq!(statute.preconditions.len(), 4);
        assert!(statute.discretion_logic.is_some());
    }

    #[test]
    fn test_iied_as_statute() {
        let statute = iied_as_statute();
        assert_eq!(statute.id, "restatement-2d-torts-46");
        assert!(statute.discretion_logic.is_some());
    }

    #[test]
    fn test_products_liability_as_statute() {
        let statute = products_liability_as_statute();
        assert_eq!(statute.id, "restatement-2d-torts-402a");
        assert_eq!(statute.preconditions.len(), 6);
    }

    #[test]
    fn test_all_statutes_valid() {
        let statutes = vec![
            battery_as_statute(),
            iied_as_statute(),
            products_liability_as_statute(),
        ];

        for statute in statutes {
            assert!(statute.is_valid(), "{} should be valid", statute.id);
        }
    }
}
