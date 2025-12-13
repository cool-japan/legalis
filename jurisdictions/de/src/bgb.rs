//! BGB (Bürgerliches Gesetzbuch) - German Civil Code
//!
//! The German Civil Code enacted in 1900, following the Pandekten system.
//! This module implements key provisions of the BGB, particularly tort law (§§ 823-853).
//!
//! ## Structure
//!
//! The BGB consists of 5 books (Bücher):
//! - Book 1: General Part (Allgemeiner Teil) - §§ 1-240
//! - Book 2: Law of Obligations (Recht der Schuldverhältnisse) - §§ 241-853
//! - Book 3: Law of Things (Sachenrecht) - §§ 854-1296
//! - Book 4: Family Law (Familienrecht) - §§ 1297-1921
//! - Book 5: Law of Succession (Erbrecht) - §§ 1922-2385
//!
//! ## Tort Law (Unerlaubte Handlungen)
//!
//! - § 823: Liability in damages (Schadensersatzpflicht)
//! - § 826: Intentional damage contrary to public policy
//! - § 831: Liability of the principal

use legalis_core::{Condition, Effect, EffectType, Statute};

/// BGB § 823 Abs. 1 - Liability for Damages (Schadensersatzpflicht)
///
/// ## German Text
///
/// > Wer vorsätzlich oder fahrlässig das Leben, den Körper, die Gesundheit,
/// > die Freiheit, das Eigentum oder ein sonstiges Recht eines anderen widerrechtlich
/// > verletzt, ist dem anderen zum Ersatz des daraus entstehenden Schadens verpflichtet.
///
/// ## English Translation
///
/// > A person who, intentionally or negligently, unlawfully injures the life, body, health,
/// > freedom, property or any other right of another person is liable to make compensation
/// > to the other party for the damage arising from this.
///
/// ## Legal Structure
///
/// **Requirements (Tatbestandsmerkmale)**:
/// 1. Intent (Vorsatz) OR Negligence (Fahrlässigkeit) - *Subjective element*
/// 2. Violation of enumerated protected interests:
///    - Life (Leben)
///    - Body (Körper)
///    - Health (Gesundheit)
///    - Freedom (Freiheit)
///    - Property (Eigentum)
///    - Other rights (sonstiges Recht) - e.g., personality rights, intellectual property
/// 3. Unlawfulness (Widerrechtlichkeit) - *Objective element*
/// 4. Causation (Kausalität)
///
/// **Effect (Rechtsfolge)**:
/// - Obligation to compensate for damages (Schadensersatzpflicht)
///
/// ## Comparison with Japanese Civil Code Article 709
///
/// **Similarities**:
/// - Both require intent OR negligence
/// - Both impose damages liability
/// - Both require causation
///
/// **Key Differences**:
/// - **BGB §823**: Enumerates specific protected interests (numerus clausus approach)
/// - **Japanese Art. 709**: General clause covering "rights or legally protected interests"
/// - **BGB**: Separate unlawfulness requirement (Widerrechtlichkeit)
/// - **Japanese**: Unlawfulness implied in "infringement" (侵害)
///
/// ## Example
///
/// ```rust
/// use legalis_de::bgb_823_1;
///
/// let statute = bgb_823_1();
/// println!("{}", statute);
/// // => STATUTE bgb-823-1: "BGB § 823 Abs. 1 Schadensersatzpflicht"
/// ```
#[must_use]
pub fn bgb_823_1() -> Statute {
    Statute::new(
        "bgb-823-1",
        "BGB § 823 Abs. 1 Schadensersatzpflicht / Liability for Damages",
        Effect::new(
            EffectType::Obligation,
            "Schadensersatzpflicht (Obligation to compensate for damages)",
        )
        .with_parameter("liable_party", "Schädiger (tortfeasor)")
        .with_parameter("beneficiary", "Geschädigter (injured party)")
        .with_parameter("content", "Ersatz des entstandenen Schadens (compensation for damage)"),
    )
    .with_jurisdiction("DE")
    .with_version(1)
    // Requirement 1: Vorsatz (Intent) OR Fahrlässigkeit (Negligence)
    .with_precondition(Condition::Or(
        Box::new(Condition::AttributeEquals {
            key: "vorsatz".to_string(),
            value: "true".to_string(),
        }),
        Box::new(Condition::AttributeEquals {
            key: "fahrlassigkeit".to_string(),
            value: "true".to_string(),
        }),
    ))
    // Requirement 2: Violation of protected interests (enumerated)
    // At least one of: Leben, Körper, Gesundheit, Freiheit, Eigentum, sonstiges Recht
    .with_precondition(Condition::Or(
        Box::new(Condition::Or(
            Box::new(Condition::Or(
                Box::new(Condition::AttributeEquals {
                    key: "leben_verletzt".to_string(), // Life injured
                    value: "true".to_string(),
                }),
                Box::new(Condition::AttributeEquals {
                    key: "korper_verletzt".to_string(), // Body injured
                    value: "true".to_string(),
                }),
            )),
            Box::new(Condition::Or(
                Box::new(Condition::AttributeEquals {
                    key: "gesundheit_verletzt".to_string(), // Health injured
                    value: "true".to_string(),
                }),
                Box::new(Condition::AttributeEquals {
                    key: "freiheit_verletzt".to_string(), // Freedom violated
                    value: "true".to_string(),
                }),
            )),
        )),
        Box::new(Condition::Or(
            Box::new(Condition::AttributeEquals {
                key: "eigentum_verletzt".to_string(), // Property damaged
                value: "true".to_string(),
            }),
            Box::new(Condition::AttributeEquals {
                key: "sonstiges_recht_verletzt".to_string(), // Other right violated
                value: "true".to_string(),
            }),
        )),
    ))
    // Requirement 3: Widerrechtlichkeit (Unlawfulness)
    .with_precondition(Condition::AttributeEquals {
        key: "widerrechtlich".to_string(),
        value: "true".to_string(),
    })
    // Requirement 4: Kausalität (Causation)
    .with_precondition(Condition::AttributeEquals {
        key: "kausalitat".to_string(),
        value: "true".to_string(),
    })
    .with_discretion(
        "Die Feststellung von Vorsatz oder Fahrlässigkeit erfordert eine Würdigung der Umstände des Einzelfalls. \
        Die Widerrechtlichkeit wird vermutet, kann aber durch Rechtfertigungsgründe (z.B. Notwehr, Einwilligung) entfallen. \
        \n\nDetermination of intent or negligence requires assessment of the circumstances of the individual case. \
        Unlawfulness is presumed but can be negated by justification grounds (e.g., self-defense, consent). \
        \n\n【日本法との比較】\n\
        ドイツ法は保護法益を明示的に列挙（限定列挙+「その他の権利」）するのに対し、\
        日本民法709条は「権利又は法律上保護される利益」と包括的に規定する。\
        これは法典編纂の思想の違いを反映している。",
    )
}

/// BGB § 823 Abs. 2 - Liability for Violation of Protective Statutes
///
/// ## German Text
///
/// > Die gleiche Verpflichtung trifft denjenigen, welcher gegen ein den Schutz eines anderen
/// > bezweckendes Gesetz verstößt. Ist nach dem Inhalt des Gesetzes ein Verstoß gegen dieses
/// > auch ohne Verschulden möglich, so tritt die Ersatzpflicht nur im Falle des Verschuldens ein.
///
/// ## English Translation
///
/// > The same obligation is incumbent on a person who infringes a statute
/// > that is intended to protect another person. If, under the contents of the statute,
/// > an infringement of this is possible even without fault, then liability to pay damages
/// > only comes into being in the case of fault.
///
/// ## Legal Structure
///
/// This paragraph creates liability for violation of protective statutes (Schutzgesetze).
/// A statute is "protective" if it aims to protect individual interests, not just public order.
///
/// **Requirements**:
/// 1. Violation of a protective statute (Verstoß gegen Schutzgesetz)
/// 2. The statute must aim to protect the injured party
/// 3. Fault (Verschulden) - if the statute allows violation without fault
/// 4. Causation
///
/// **Examples of protective statutes**:
/// - Traffic laws (StVO)
/// - Product safety regulations
/// - Environmental protection laws (when protecting individuals)
#[must_use]
pub fn bgb_823_2() -> Statute {
    Statute::new(
        "bgb-823-2",
        "BGB § 823 Abs. 2 Schutzgesetzverletzung / Violation of Protective Statute",
        Effect::new(
            EffectType::Obligation,
            "Schadensersatzpflicht wegen Schutzgesetzverletzung",
        )
        .with_parameter("liable_party", "Verletzer des Schutzgesetzes")
        .with_parameter("beneficiary", "Geschützter"),
    )
    .with_jurisdiction("DE")
    .with_version(1)
    // Requirement 1: Violation of a protective statute
    .with_precondition(Condition::AttributeEquals {
        key: "schutzgesetz_verletzt".to_string(),
        value: "true".to_string(),
    })
    // Requirement 2: The statute protects the injured party's interests
    .with_precondition(Condition::AttributeEquals {
        key: "schutzbereich_erfasst".to_string(),
        value: "true".to_string(),
    })
    // Requirement 3: Fault (if statute allows violation without fault)
    .with_precondition(Condition::Or(
        Box::new(Condition::AttributeEquals {
            key: "verschulden".to_string(),
            value: "true".to_string(),
        }),
        Box::new(Condition::AttributeEquals {
            key: "strict_liability_statute".to_string(),
            value: "true".to_string(),
        }),
    ))
    // Requirement 4: Causation
    .with_precondition(Condition::AttributeEquals {
        key: "kausalitat".to_string(),
        value: "true".to_string(),
    })
    .with_discretion(
        "Die Bestimmung, ob ein Gesetz Schutzgesetzcharakter hat, ist Auslegungsfrage. \
        Ein Gesetz ist nur dann Schutzgesetz, wenn es (auch) dem Schutz individueller Rechtsgüter dient. \
        \n\nDetermining whether a statute has protective character is a matter of interpretation. \
        A statute is protective only if it serves (also) to protect individual legal interests.",
    )
}

/// BGB § 826 - Intentional Damage Contrary to Public Policy
///
/// ## German Text
///
/// > Wer in einer gegen die guten Sitten verstoßenden Weise einem anderen vorsätzlich
/// > Schaden zufügt, ist dem anderen zum Ersatz des Schadens verpflichtet.
///
/// ## English Translation
///
/// > A person who, in a manner contrary to public policy (good morals), intentionally
/// > inflicts damage on another person is liable to the other person to make compensation
/// > for the damage.
///
/// ## Legal Structure
///
/// § 826 is a general clause (Generalklausel) that catches intentional harmful conduct
/// that violates good morals but might not fall under § 823.
///
/// **Requirements**:
/// 1. Intent (Vorsatz) - both to perform the act AND to cause damage
/// 2. Conduct contrary to good morals (Sittenwidrigkeit)
/// 3. Damage (Schaden)
/// 4. Causation
///
/// **Key differences from § 823**:
/// - Requires INTENT (no negligence liability)
/// - No enumeration of protected interests (catches all damages)
/// - Requires violation of good morals (höhere Schwelle)
///
/// **Examples**:
/// - Fraudulent misrepresentation (arglistige Täuschung)
/// - Abuse of legal rights
/// - Economic boycotts
/// - Exploitation of distress
#[must_use]
pub fn bgb_826() -> Statute {
    Statute::new(
        "bgb-826",
        "BGB § 826 Sittenwidrige vorsätzliche Schädigung / Intentional Damage Contrary to Public Policy",
        Effect::new(
            EffectType::Obligation,
            "Schadensersatzpflicht (Liability for damages)",
        )
        .with_parameter("liable_party", "Sittenwidrig Handelnder (wrongdoer)")
        .with_parameter("beneficiary", "Geschädigter (injured party)"),
    )
    .with_jurisdiction("DE")
    .with_version(1)
    // Requirement 1: Intent to cause damage (Schädigungsvorsatz)
    .with_precondition(Condition::AttributeEquals {
        key: "schadigungsvorsatz".to_string(),
        value: "true".to_string(),
    })
    // Requirement 2: Conduct contrary to good morals (Sittenwidrigkeit)
    .with_precondition(Condition::AttributeEquals {
        key: "sittenwidrig".to_string(),
        value: "true".to_string(),
    })
    // Requirement 3: Damage occurred
    .with_precondition(Condition::AttributeEquals {
        key: "schaden_entstanden".to_string(),
        value: "true".to_string(),
    })
    // Requirement 4: Causation
    .with_precondition(Condition::AttributeEquals {
        key: "kausalitat".to_string(),
        value: "true".to_string(),
    })
    .with_discretion(
        "Die Sittenwidrigkeit ist eine Generalklausel und erfordert eine Gesamtwürdigung \
        der Umstände. Maßgeblich ist das Anstandsgefühl aller billig und gerecht Denkenden. \
        \n\nViolation of good morals is a general clause requiring comprehensive assessment \
        of circumstances. The standard is the sense of propriety of all fair-minded people. \
        \n\n【日本法との対比】\n\
        § 826 は民法709条よりも狭い（故意のみ、良俗違反が必要）が、\
        保護範囲は広い（あらゆる損害を捕捉）。日本法には類似の包括条項は少ない。",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bgb_823_1_creation() {
        let statute = bgb_823_1();
        assert_eq!(statute.id, "bgb-823-1");
        assert_eq!(statute.jurisdiction, Some("DE".to_string()));
        assert_eq!(statute.version, 1);
        assert_eq!(statute.effect.effect_type, EffectType::Obligation);

        // Should have 4 preconditions: (vorsatz OR fahrlässigkeit), protected interests, unlawfulness, causation
        assert_eq!(statute.preconditions.len(), 4);

        // First precondition should be OR (vorsatz OR fahrlässigkeit)
        assert!(matches!(statute.preconditions[0], Condition::Or(_, _)));

        // Should have discretion logic
        assert!(statute.discretion_logic.is_some());
    }

    #[test]
    fn test_bgb_823_1_validation() {
        let statute = bgb_823_1();
        assert!(statute.is_valid());
        assert_eq!(statute.validate().len(), 0);
    }

    #[test]
    fn test_bgb_823_1_display() {
        let statute = bgb_823_1();
        let display = format!("{}", statute);

        assert!(display.contains("bgb-823-1"));
        assert!(display.contains("BGB § 823"));
        assert!(display.contains("DE"));
        assert!(display.contains("OBLIGATION"));
    }

    #[test]
    fn test_bgb_823_2_creation() {
        let statute = bgb_823_2();
        assert_eq!(statute.id, "bgb-823-2");
        assert_eq!(statute.jurisdiction, Some("DE".to_string()));

        // Should have 4 preconditions
        assert_eq!(statute.preconditions.len(), 4);
    }

    #[test]
    fn test_bgb_823_2_validation() {
        let statute = bgb_823_2();
        assert!(statute.is_valid());
    }

    #[test]
    fn test_bgb_826_creation() {
        let statute = bgb_826();
        assert_eq!(statute.id, "bgb-826");
        assert_eq!(statute.jurisdiction, Some("DE".to_string()));

        // Should have 4 preconditions: intent, sittenwidrig, damage, causation
        assert_eq!(statute.preconditions.len(), 4);
    }

    #[test]
    fn test_bgb_826_validation() {
        let statute = bgb_826();
        assert!(statute.is_valid());
    }

    #[test]
    fn test_bgb_826_only_intent() {
        let statute = bgb_826();

        // § 826 should only have intent requirement, not negligence
        let has_intent_req = statute.preconditions.iter().any(
            |c| matches!(c, Condition::AttributeEquals { key, .. } if key == "schadigungsvorsatz"),
        );

        assert!(has_intent_req);
    }
}
