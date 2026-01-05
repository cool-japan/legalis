//! Famous Tort Cases in Common Law
//!
//! This module contains implementations of landmark Common Law tort cases
//! that established fundamental principles and continue to be cited as precedent.
//!
//! ## Stare Decisis in Action
//!
//! These cases demonstrate how Common Law develops through judicial decisions:
//!
//! ```text
//! Donoghue v Stevenson (1932, UK) → Neighbor principle
//!         ↓
//! MacPherson v Buick (1916, US-NY) → Manufacturer liability
//!         ↓
//! Palsgraf v Long Island (1928, US-NY) → Proximate cause
//!         ↓
//! Modern negligence law across Common Law world
//! ```

use legalis_core::case_law::{Case, CaseRule, Court};
use legalis_core::{Condition, Effect, EffectType};

/// Palsgraf v. Long Island Railroad Co., 248 N.Y. 339 (1928)
///
/// ## Facts (事実)
///
/// A railroad employee helped a passenger board a moving train. The passenger
/// was carrying a package (unknown to the employee, containing fireworks).
/// The package fell and exploded, causing scales at the other end of the platform
/// to fall on Mrs. Palsgraf, injuring her.
///
/// ## Issue (争点)
///
/// Is the railroad liable to Mrs. Palsgraf for negligence?
///
/// ## Holding (判決)
///
/// **NO**. Negligence requires a duty to the plaintiff. The guard owed no duty to
/// Palsgraf because injury to her was not foreseeable from helping a passenger
/// with a package.
///
/// ## Ratio Decidendi (判例法理) - Cardozo Majority
///
/// > "The risk reasonably to be perceived defines the duty to be obeyed."
///
/// Negligence is **relational**: duty runs to foreseeable plaintiffs, not the world.
/// The guard's conduct may have been careless regarding the passenger, but was
/// not negligent as to Palsgraf (an unforeseeable plaintiff).
///
/// ## Dissent - Andrews, J.
///
/// > "Negligence is a matter between the defendant and the public."
///
/// If the defendant is negligent to anyone, duty extends to all who are harmed.
/// The question is **proximate cause**, not duty.
///
/// ## Legal Significance
///
/// This case established the **foreseeable plaintiff** rule in American tort law.
/// Courts must decide: Is this case more like Cardozo's view or Andrews' view?
///
/// ## Comparative Law Note
///
/// Civil Law systems (Japan, Germany, France) do not have this debate because
/// they focus on causation and protected interests, not "duty" as a separate element.
/// The concept of "duty" as a threshold requirement is **unique to Common Law**.
#[must_use]
pub fn palsgraf_v_long_island() -> Case {
    Case::new(
        "Palsgraf v. Long Island R.R., 248 N.Y. 339, 162 N.E. 99 (1928)",
        "Palsgraf v. Long Island R.R.",
        1928,
        Court::Appellate, // New York Court of Appeals (highest court in NY state)
        "US-NY",
    )
    .with_facts(
        "Railroad employee helped passenger board moving train. Passenger's package \
        (containing fireworks) fell and exploded. Explosion caused scales to fall on \
        plaintiff Palsgraf, injuring her. Employee did not know package contained fireworks.",
    )
    .with_issue(
        "Is railroad liable for negligence when injury to plaintiff was not foreseeable \
        from the defendant's conduct?",
    )
    .with_holding(
        "No liability. Negligence requires duty to the plaintiff. Injury to Palsgraf \
        was not foreseeable, so no duty was owed to her.",
    )
    .with_ratio(
        "The risk reasonably to be perceived defines the duty to be obeyed. \
        Negligence is relational - duty runs to foreseeable plaintiffs. \
        An act may be negligent as to A but not as to B (unforeseeable plaintiff).",
    )
    .with_rule(CaseRule {
        name: "Foreseeable Plaintiff Rule (Cardozo)".to_string(),
        conditions: vec![
            Condition::AttributeEquals {
                key: "defendant_conduct".to_string(),
                value: "true".to_string(),
            },
            Condition::AttributeEquals {
                key: "injury_to_foreseeable_plaintiff".to_string(),
                value: "true".to_string(), // This is the KEY requirement
            },
            Condition::AttributeEquals {
                key: "causation".to_string(),
                value: "true".to_string(),
            },
        ],
        effect: Effect::new(
            EffectType::Obligation,
            "Negligence liability (duty + breach + causation + damages)",
        )
        .with_parameter("scope", "Only to foreseeable plaintiffs"),
        exceptions: vec![
            "Plaintiff not within zone of foreseeable risk".to_string(),
            "Defendant's conduct not negligent as to this plaintiff".to_string(),
        ],
    })
}

/// Donoghue v Stevenson \[1932\] AC 562 (House of Lords, UK)
///
/// ## Facts
///
/// Mrs. Donoghue drank ginger beer from an opaque bottle purchased by her friend.
/// She poured the remainder into her glass and a decomposed snail emerged.
/// She suffered shock and gastroenteritis. She could not sue in contract
/// (she didn't buy the bottle), so sued the manufacturer in tort.
///
/// ## Issue
///
/// Does a manufacturer owe a duty of care to ultimate consumers with whom
/// they have no contractual relationship?
///
/// ## Holding
///
/// **YES**. Manufacturers owe a duty to take reasonable care to avoid injury
/// to consumers.
///
/// ## Ratio Decidendi - Lord Atkin
///
/// > "You must take reasonable care to avoid acts or omissions which you can
/// > reasonably foresee would be likely to injure your **neighbour**."
///
/// Who is my neighbor?
/// > "Persons who are so closely and directly affected by my act that I ought
/// > reasonably to have them in contemplation as being so affected when I am
/// > directing my mind to the acts or omissions in question."
///
/// ## Legal Significance
///
/// This case established the modern **duty of care** principle and is the
/// foundation of negligence law throughout the Common Law world.
///
/// It broke the doctrine of **privity of contract** (no liability without contract).
///
/// ## Global Impact
///
/// - **UK**: Still leading precedent
/// - **US**: Adopted through cases like MacPherson v. Buick (predated but confirmed)
/// - **Australia, Canada, NZ**: Binding precedent
/// - **Civil Law**: Inspired product liability laws (though via statute, not case law)
///
/// ## Comparison with Palsgraf
///
/// Both cases involve foreseeability:
/// - **Donoghue**: Broad duty to foreseeable victims (Andrews dissent in Palsgraf)
/// - **Palsgraf**: Narrow duty only to foreseeable plaintiffs (Cardozo)
///
/// This shows how Common Law develops through competing judicial philosophies.
#[must_use]
pub fn donoghue_v_stevenson() -> Case {
    Case::new(
        "Donoghue v Stevenson [1932] UKHL 100, [1932] AC 562",
        "Donoghue v Stevenson",
        1932,
        Court::Supreme, // House of Lords (now Supreme Court)
        "UK",
    )
    .with_facts(
        "Plaintiff drank ginger beer from opaque bottle purchased by friend. \
        Decomposed snail emerged from bottle. Plaintiff suffered shock and illness. \
        No contract with manufacturer (friend purchased). Sued in tort.",
    )
    .with_issue(
        "Does a manufacturer owe a duty of care in tort to ultimate consumers \
        with whom there is no contractual relationship (privity)?",
    )
    .with_holding(
        "Yes. Manufacturer owes a duty to take reasonable care to avoid injury \
        to consumers who are reasonably foreseeable victims of negligence.",
    )
    .with_ratio(
        "The neighbor principle: You must take reasonable care to avoid acts or \
        omissions which you can reasonably foresee would be likely to injure your \
        neighbor (persons so closely and directly affected by your act that you \
        ought reasonably to have them in contemplation). \
        \n\nThis duty exists independent of contract.",
    )
    .with_rule(CaseRule {
        name: "Neighbor Principle / Duty of Care (Lord Atkin)".to_string(),
        conditions: vec![
            Condition::AttributeEquals {
                key: "defendant_conduct".to_string(),
                value: "true".to_string(),
            },
            Condition::AttributeEquals {
                key: "foreseeable_victim".to_string(),
                value: "true".to_string(),
            },
            Condition::AttributeEquals {
                key: "proximity_relationship".to_string(), // Close and direct relationship
                value: "true".to_string(),
            },
            Condition::AttributeEquals {
                key: "causation".to_string(),
                value: "true".to_string(),
            },
            Condition::AttributeEquals {
                key: "damage".to_string(),
                value: "true".to_string(),
            },
        ],
        effect: Effect::new(
            EffectType::Obligation,
            "Duty of care owed to neighbors (foreseeable victims)",
        )
        .with_parameter("liable_party", "manufacturer or other actor")
        .with_parameter("remedy", "compensatory damages for injury"),
        exceptions: vec![
            "Victim not foreseeable".to_string(),
            "No proximity relationship".to_string(),
            "Policy reasons against imposing duty".to_string(),
        ],
    })
}

/// Garratt v. Dailey, 279 P.2d 1091 (Wash. 1955)
///
/// ## Facts
///
/// Five-year-old Brian Dailey pulled a chair away as Ruth Garratt (elderly woman)
/// was sitting down. She fell and fractured her hip.
///
/// ## Issue
///
/// Can a five-year-old child have the intent required for battery?
///
/// ## Holding
///
/// **YES**. Intent for battery requires either:
/// 1. Purpose to cause harmful/offensive contact, OR
/// 2. **Substantial certainty** that such contact would occur
///
/// If Brian knew with substantial certainty that Garratt would attempt to sit
/// and would fall, that satisfies the intent requirement.
///
/// ## Legal Significance
///
/// This case clarified that intent in tort law does NOT require:
/// - Malice
/// - Desire to harm
/// - Understanding that the act is wrongful
///
/// Intent = purpose OR substantial certainty (objective test).
///
/// ## Child Liability in Comparative Law
///
/// - **US (Common Law)**: Children liable if they can form intent (age varies by tort)
/// - **Japan**: Article 712 Minpo - children under 12 generally not liable (責任能力)
/// - **Germany**: § 828 BGB - children under 7 not liable, 7-17 limited liability
/// - **France**: Article 1241 - children can be liable but parents often liable under 1242
///
/// Common Law tends to hold children liable more readily if intent can be shown.
#[must_use]
pub fn garratt_v_dailey() -> Case {
    Case::new(
        "Garratt v. Dailey, 279 P.2d 1091 (Wash. 1955)",
        "Garratt v. Dailey",
        1955,
        Court::Supreme, // Washington Supreme Court
        "US-WA",
    )
    .with_facts(
        "Five-year-old defendant pulled chair away as elderly plaintiff was sitting down. \
        Plaintiff fell and fractured hip. Question whether child had requisite intent.",
    )
    .with_issue("What level of intent is required for battery? Can a child have that intent?")
    .with_holding(
        "Intent for battery exists if defendant acts with (1) purpose to cause contact OR \
        (2) substantial certainty that contact will occur. Age is not a defense if intent exists.",
    )
    .with_ratio(
        "Intent in tort law is satisfied by substantial certainty, not just purpose. \
        A five-year-old can have the requisite intent if he knows with substantial certainty \
        that the harmful contact will occur. Intent does not require malice or understanding \
        of wrongfulness.",
    )
    .with_rule(CaseRule {
        name: "Substantial Certainty Test for Intent".to_string(),
        conditions: vec![
            Condition::Or(
                Box::new(Condition::AttributeEquals {
                    key: "purpose_to_cause_contact".to_string(),
                    value: "true".to_string(),
                }),
                Box::new(Condition::AttributeEquals {
                    key: "substantial_certainty_of_contact".to_string(),
                    value: "true".to_string(),
                }),
            ),
            Condition::AttributeEquals {
                key: "harmful_or_offensive_contact".to_string(),
                value: "true".to_string(),
            },
        ],
        effect: Effect::new(EffectType::Obligation, "Battery liability")
            .with_parameter("intent_standard", "purpose OR substantial certainty"),
        exceptions: vec![
            "No substantial certainty and no purpose".to_string(),
            "Consent (express or implied)".to_string(),
        ],
    })
}

/// Vosburg v. Putney, 80 Wis. 523, 50 N.W. 403 (1891)
///
/// ## Facts
///
/// During class, 11-year-old Andrew Putney kicked 14-year-old George Vosburg
/// in the shin. The kick was slight and would normally cause no harm, but
/// Vosburg had a preexisting condition that made the injury severe (infection,
/// permanent disability).
///
/// ## Issue
///
/// Is defendant liable for unforeseeable extent of injury ("eggshell skull" rule)?
///
/// ## Holding
///
/// **YES**. "The wrongdoer is liable for all injuries resulting directly from
/// the wrongful act, whether they could or could not have been foreseen by him."
///
/// ## Legal Significance
///
/// This case established the **"thin skull" (eggshell plaintiff) rule**:
/// You take your victim as you find them. If your intentional tort causes
/// greater harm due to victim's preexisting condition, you are liable for
/// the full extent of the harm.
///
/// ## Thin Skull Rule in Comparative Law
///
/// - **US/UK Common Law**: Well-established through cases like this
/// - **Japan**: 民法416条 (proximate cause) + case law recognition
/// - **Germany**: Adäquanztheorie (adequacy theory) - controversial application
/// - **France**: Accepted under lien de causalité (causal link)
///
/// The rule is most clearly established in Common Law systems.
#[must_use]
pub fn vosburg_v_putney() -> Case {
    Case::new(
        "Vosburg v. Putney, 80 Wis. 523, 50 N.W. 403 (1891)",
        "Vosburg v. Putney",
        1891,
        Court::Supreme, // Wisconsin Supreme Court
        "US-WI",
    )
    .with_facts(
        "Defendant (11 years old) kicked plaintiff (14) in shin during class. \
        Kick was slight, but plaintiff had preexisting condition. Injury became severe \
        (infection, permanent disability). Far worse outcome than foreseeable.",
    )
    .with_issue(
        "Is defendant liable for unforeseeable extent of injury caused by intentional tort?",
    )
    .with_holding(
        "Yes. Wrongdoer liable for all direct injuries from wrongful act, even if \
        extent of injury was unforeseeable. 'Thin skull' / 'eggshell plaintiff' rule.",
    )
    .with_ratio(
        "The wrongdoer is liable for all injuries resulting directly from the wrongful act, \
        whether they could or could not have been foreseen. You take your victim as you find them. \
        The defendant's intent to commit battery is sufficient; foreseeability of extent of harm \
        is not required.",
    )
    .with_rule(CaseRule {
        name: "Thin Skull Rule / Eggshell Plaintiff Rule".to_string(),
        conditions: vec![
            Condition::AttributeEquals {
                key: "intentional_tort".to_string(),
                value: "true".to_string(),
            },
            Condition::AttributeEquals {
                key: "causation".to_string(),
                value: "true".to_string(),
            },
            Condition::AttributeEquals {
                key: "injury_greater_than_foreseeable".to_string(),
                value: "true".to_string(), // Preexisting condition made harm worse
            },
        ],
        effect: Effect::new(
            EffectType::Obligation,
            "Liability for full extent of injury, even if unforeseeable",
        )
        .with_parameter("rule", "take victim as you find them"),
        exceptions: vec![
            "Superseding cause broke chain of causation".to_string(),
            "Plaintiff's intentional self-harm".to_string(),
        ],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palsgraf_creation() {
        let case = palsgraf_v_long_island();
        assert_eq!(case.year, 1928);
        assert_eq!(case.court, Court::Appellate);
        assert_eq!(case.jurisdiction, "US-NY");
        assert!(case.rule.is_some());
        assert!(!case.facts.is_empty());
        assert!(!case.holding.is_empty());
    }

    #[test]
    fn test_donoghue_creation() {
        let case = donoghue_v_stevenson();
        assert_eq!(case.year, 1932);
        assert_eq!(case.court, Court::Supreme);
        assert_eq!(case.jurisdiction, "UK");
        assert!(case.citation.contains("AC 562"));
    }

    #[test]
    fn test_garratt_creation() {
        let case = garratt_v_dailey();
        assert_eq!(case.year, 1955);
        assert_eq!(case.jurisdiction, "US-WA");
        assert!(case.rule.is_some());
    }

    #[test]
    fn test_vosburg_creation() {
        let case = vosburg_v_putney();
        assert_eq!(case.year, 1891);
        assert_eq!(case.jurisdiction, "US-WI");
        let rule = case.rule.as_ref().unwrap();
        assert!(rule.name.contains("Thin Skull"));
    }

    #[test]
    fn test_all_cases_have_complete_data() {
        let cases = vec![
            palsgraf_v_long_island(),
            donoghue_v_stevenson(),
            garratt_v_dailey(),
            vosburg_v_putney(),
        ];

        for case in cases {
            assert!(
                !case.citation.is_empty(),
                "{} missing citation",
                case.short_name
            );
            assert!(!case.facts.is_empty(), "{} missing facts", case.short_name);
            assert!(
                !case.holding.is_empty(),
                "{} missing holding",
                case.short_name
            );
            assert!(!case.ratio.is_empty(), "{} missing ratio", case.short_name);
            assert!(case.rule.is_some(), "{} missing rule", case.short_name);
        }
    }

    #[test]
    fn test_case_rules_have_conditions() {
        let cases = vec![
            palsgraf_v_long_island(),
            donoghue_v_stevenson(),
            garratt_v_dailey(),
            vosburg_v_putney(),
        ];

        for case in cases {
            let rule = case.rule.as_ref().unwrap();
            assert!(
                !rule.conditions.is_empty(),
                "{} rule has no conditions",
                case.short_name
            );
            assert!(
                !rule.exceptions.is_empty(),
                "{} rule has no exceptions",
                case.short_name
            );
        }
    }

    #[test]
    fn test_precedent_dates() {
        // Verify temporal ordering makes sense
        let vosburg = vosburg_v_putney();
        let palsgraf = palsgraf_v_long_island();
        let donoghue = donoghue_v_stevenson();
        let garratt = garratt_v_dailey();

        assert!(vosburg.year < palsgraf.year); // 1891 < 1928
        assert!(palsgraf.year < donoghue.year); // 1928 < 1932
        assert!(donoghue.year < garratt.year); // 1932 < 1955
    }
}
