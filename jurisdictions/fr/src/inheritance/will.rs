//! Will and testament articles (Code civil, Book III)
//!
//! This module implements articles governing wills, testamentary dispositions,
//! and legitimacy/recognition of children in French inheritance law.

use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};

/// Articles 774-792 - Types of wills and validity requirements
///
/// # French Law Recognizes Three Types of Wills
///
/// ## 1. Holographic Will (Testament olographe) - Article 970
/// - Must be **entirely handwritten** by testator
/// - Must be **dated**
/// - Must be **signed**
/// - No witnesses required
/// - Most common in France (simple, private)
///
/// ## 2. Authentic Will (Testament authentique) - Article 971
/// - Made before a **notary** (notaire)
/// - Requires **two witnesses** or second notary
/// - Testator dictates wishes
/// - Notary writes and reads back
/// - Most secure (cannot be lost or destroyed)
///
/// ## 3. Mystic Will (Testament mystique) - Article 976
/// - Testator writes or has written
/// - Sealed in envelope
/// - Presented to notary with witnesses
/// - Combines privacy with authenticity
/// - Rarely used in modern practice
///
/// # Legal Commentary
/// The three forms balance different needs:
/// - **Holographic**: Simplicity and privacy
/// - **Authentic**: Security and professional guidance
/// - **Mystic**: Privacy with formal recognition
///
/// # Historical Context
/// These forms date from the 1804 Napoleonic Code and reflect Roman law traditions.
/// The holographic will is unique to civil law systems - common law typically requires witnesses.
///
/// # Comparison with Other Jurisdictions
/// - **Common Law**: Generally requires witnessed wills (no holographic)
/// - **Germany (BGB §2247)**: Similar holographic will recognized
/// - **Japan (Minpo §968)**: Recognizes holographic wills
///
/// # Examples
///
/// ```
/// use legalis_fr::inheritance::{Will, WillType};
/// use chrono::NaiveDate;
///
/// // Holographic will - most common
/// let holographic = WillType::Holographic {
///     handwritten: true,
///     dated: true,
///     signed: true,
/// };
/// let will = Will::new(holographic, "Jean Dupont".to_string(), NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());
///
/// // Authentic will - most secure
/// let authentic = WillType::Authentic {
///     notary: "Maître Lefebvre".to_string(),
///     witnesses: vec!["Witness 1".to_string(), "Witness 2".to_string()],
/// };
/// ```
pub fn article774_792() -> Statute {
    Statute::new(
        "code-civil-774-792",
        "Code civil Articles 774-792 - Types of wills (holographic, authentic, mystic)",
        Effect::new(
            EffectType::Grant,
            "Valid will allows testamentary dispositions within available portion",
        )
        .with_parameter("holographic_requirements", "handwritten_dated_signed")
        .with_parameter("authentic_requirements", "notary_and_two_witnesses")
        .with_parameter("mystic_requirements", "sealed_presented_to_notary"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    .with_precondition(Condition::Or(
        // Holographic will (Article 970)
        Box::new(Condition::And(
            Box::new(Condition::AttributeEquals {
                key: "will_type".to_string(),
                value: "holographic".to_string(),
            }),
            Box::new(Condition::And(
                Box::new(Condition::AttributeEquals {
                    key: "handwritten".to_string(),
                    value: "true".to_string(),
                }),
                Box::new(Condition::And(
                    Box::new(Condition::AttributeEquals {
                        key: "dated".to_string(),
                        value: "true".to_string(),
                    }),
                    Box::new(Condition::AttributeEquals {
                        key: "signed".to_string(),
                        value: "true".to_string(),
                    }),
                )),
            )),
        )),
        Box::new(Condition::Or(
            // Authentic will (Article 971)
            Box::new(Condition::And(
                Box::new(Condition::AttributeEquals {
                    key: "will_type".to_string(),
                    value: "authentic".to_string(),
                }),
                Box::new(Condition::And(
                    Box::new(Condition::HasAttribute {
                        key: "notary".to_string(),
                    }),
                    Box::new(Condition::Threshold {
                        attributes: vec![("witnesses".to_string(), 1.0)],
                        operator: ComparisonOp::GreaterOrEqual,
                        value: 2.0,
                    }),
                )),
            )),
            // Mystic will (Article 976)
            Box::new(Condition::And(
                Box::new(Condition::AttributeEquals {
                    key: "will_type".to_string(),
                    value: "mystic".to_string(),
                }),
                Box::new(Condition::And(
                    Box::new(Condition::AttributeEquals {
                        key: "sealed".to_string(),
                        value: "true".to_string(),
                    }),
                    Box::new(Condition::HasAttribute {
                        key: "notary".to_string(),
                    }),
                )),
            )),
        )),
    ))
    .with_discretion(
        "Three will forms recognized in French law:\n\
         1. Holographic (Article 970): Handwritten, dated, signed - no witnesses needed. Most common.\n\
         2. Authentic (Article 971): Before notary with 2 witnesses. Most secure.\n\
         3. Mystic (Article 976): Sealed, presented to notary. Rarely used.\n\n\
         Historical note: Forms derive from Roman law and 1804 Napoleonic Code. Holographic will \
         is unique to civil law - common law typically requires witnesses."
    )
}

/// Articles 839-851 - Recognition of non-marital children
///
/// # French Text (Article 839)
/// "L'enfant né hors mariage a en principe les mêmes droits successoraux que l'enfant né dans le mariage."
///
/// # English Translation
/// "A child born outside marriage has, in principle, the same succession rights as a child born in marriage."
///
/// # Legal Commentary
/// **Major 2001 Reform**: Abolished all discrimination against non-marital children
///
/// ## Before 2001
/// - Non-marital children received half the share of marital children
/// - Termed "enfants naturels" (natural children) - discriminatory language
/// - Adulterous children had even fewer rights
///
/// ## After 2001 Reform
/// - **Complete equality** (Article 310-1: "Tous les enfants ont les mêmes droits")
/// - All children, regardless of parents' marital status, inherit equally
/// - Discriminatory terms eliminated from Code civil
///
/// ## Recognition (Reconnaissance)
/// Non-marital children must be legally recognized:
/// - By voluntary act of parent (most common)
/// - By judicial determination (action en recherche de paternité/maternité)
/// - Recognition can occur before or after birth
///
/// # Historical Evolution
/// This represents a profound social change:
/// - 1804 Code: Severe discrimination, investigation of paternity prohibited
/// - 1972: First improvements, allowing paternity suits
/// - **2001: Complete equality** - revolutionary change
///
/// # Comparison with Other Jurisdictions
/// - **Germany**: Equalized in 1970s
/// - **UK**: Equalized in 1987
/// - **Japan**: Still some distinctions in practice (Supreme Court rulings ongoing)
/// - **France**: Most progressive - complete equality since 2001
pub fn article839_851() -> Statute {
    Statute::new(
        "code-civil-839-851",
        "Code civil Articles 839-851 - Equal rights for non-marital children (2001 reform)",
        Effect::new(
            EffectType::Grant,
            "Non-marital children have equal succession rights as marital children",
        )
        .with_parameter("equality_principle", "complete_equality_since_2001")
        .with_parameter("recognition_required", "voluntary_or_judicial"),
    )
    .with_jurisdiction("FR")
    .with_version(2) // 2001 major reform
    .with_precondition(Condition::Or(
        Box::new(Condition::AttributeEquals {
            key: "child_status".to_string(),
            value: "marital".to_string(),
        }),
        Box::new(Condition::And(
            Box::new(Condition::AttributeEquals {
                key: "child_status".to_string(),
                value: "non_marital".to_string(),
            }),
            Box::new(Condition::AttributeEquals {
                key: "recognized".to_string(),
                value: "true".to_string(),
            }),
        )),
    ))
    .with_discretion(
        "Revolutionary 2001 reform: Complete equality for all children regardless of parents' \
         marital status. Before 2001, non-marital children received only half the share. \
         This reform places France among the most progressive jurisdictions.\n\n\
         Recognition required: Non-marital children must be legally recognized (voluntary or judicial). \
         Once recognized, full equal rights apply including inheritance.\n\n\
         Comparison: Germany equalized 1970s, UK 1987, France 2001 (complete). \
         Japan still debating (Supreme Court rulings ongoing)."
    )
}

/// Articles 893-894 - Effect of legacies (testamentary gifts)
///
/// # French Text (Article 893)
/// "Les libéralités, soit entre vifs, soit à cause de mort, sont ou universelles, ou à titre universel, ou à titre particulier."
///
/// # English Translation
/// "Gifts, whether inter vivos or upon death, are either universal, by universal title, or specific."
///
/// # Three Types of Testamentary Dispositions
///
/// ## 1. Universal Legacy (Legs universel)
/// - Beneficiary receives **entire estate**
/// - Subject to reserved portion for forced heirs
/// - "I leave everything to X"
///
/// ## 2. General Legacy (Legs à titre universel)
/// - Beneficiary receives **portion** of estate
/// - E.g., "1/3 of my estate," "all movables," "all real estate"
/// - Also subject to reserved portion
///
/// ## 3. Specific Legacy (Legs particulier)
/// - Beneficiary receives **specific item**
/// - E.g., "my house at 123 rue de Paris," "my art collection"
/// - Most common type
///
/// # Legal Effects (Article 894)
/// - Universal and general legatees continue deceased's legal personality (like heirs)
/// - Specific legatees only receive designated items (not heirs)
/// - All must respect reserved portion (réserve héréditaire)
///
/// # Interaction with Reserved Portion
/// Available portion (quotité disponible):
/// - 1 child: 1/2 available
/// - 2 children: 1/3 available
/// - 3+ children: 1/4 available
///
/// Legacies exceeding available portion are reduced (réduction)
///
/// # Examples
///
/// ```
/// use legalis_fr::inheritance::{Disposition, DispositionType};
///
/// // Universal legacy - entire estate
/// let universal = Disposition::new(
///     DispositionType::Universal,
///     "Marie Dupont".to_string()
/// );
///
/// // Specific legacy - particular item
/// let specific = Disposition::new(
///     DispositionType::Specific,
///     "Jean Martin".to_string()
/// ).with_description("House at 123 rue de Paris".to_string())
///   .with_value(500_000);
/// ```
pub fn article893_894() -> Statute {
    Statute::new(
        "code-civil-893-894",
        "Code civil Articles 893-894 - Types and effects of testamentary dispositions",
        Effect::new(
            EffectType::MonetaryTransfer,
            "Testamentary dispositions transfer property subject to reserved portion limits",
        )
        .with_parameter("universal_legacy", "entire_estate")
        .with_parameter("general_legacy", "portion_of_estate")
        .with_parameter("specific_legacy", "specific_items")
        .with_parameter("limit", "available_portion_only"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    .with_precondition(Condition::HasAttribute {
        key: "testamentary_disposition".to_string(),
    })
    .with_precondition(Condition::Or(
        Box::new(Condition::AttributeEquals {
            key: "disposition_type".to_string(),
            value: "universal".to_string(),
        }),
        Box::new(Condition::Or(
            Box::new(Condition::AttributeEquals {
                key: "disposition_type".to_string(),
                value: "general".to_string(),
            }),
            Box::new(Condition::AttributeEquals {
                key: "disposition_type".to_string(),
                value: "specific".to_string(),
            }),
        )),
    ))
    .with_discretion(
        "Three types of testamentary dispositions:\n\
         1. Universal (legs universel): Entire estate\n\
         2. General (legs à titre universel): Portion of estate\n\
         3. Specific (legs particulier): Specific items\n\n\
         All subject to reserved portion (réserve héréditaire):\n\
         - 1 child: 1/2 reserved (1/2 available for legacies)\n\
         - 2 children: 2/3 reserved (1/3 available)\n\
         - 3+ children: 3/4 reserved (1/4 available)\n\n\
         Legacies exceeding available portion are reduced (réduction) to respect forced heirs' rights."
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_article774_792_creation() {
        let statute = article774_792();
        assert_eq!(statute.id, "code-civil-774-792");
        assert!(statute.title.contains("wills"));
    }

    #[test]
    fn test_article774_792_three_will_types() {
        let statute = article774_792();
        let discretion = statute.discretion_logic.as_ref().unwrap();
        assert!(discretion.contains("Holographic"));
        assert!(discretion.contains("Authentic"));
        assert!(discretion.contains("Mystic"));
    }

    #[test]
    fn test_article774_792_preconditions() {
        let statute = article774_792();
        let preconditions = &statute.preconditions;
        assert!(!preconditions.is_empty());
    }

    #[test]
    fn test_article839_851_equality_reform() {
        let statute = article839_851();
        assert_eq!(statute.version, 2); // 2001 reform
        let discretion = statute.discretion_logic.as_ref().unwrap();
        assert!(discretion.contains("2001"));
        assert!(discretion.contains("equality"));
    }

    #[test]
    fn test_article839_851_recognition_required() {
        let statute = article839_851();
        let discretion = statute.discretion_logic.as_ref().unwrap();
        assert!(discretion.contains("Recognition required"));
    }

    #[test]
    fn test_article893_894_disposition_types() {
        let statute = article893_894();
        let discretion = statute.discretion_logic.as_ref().unwrap();
        assert!(discretion.contains("Universal"));
        assert!(discretion.contains("General"));
        assert!(discretion.contains("Specific"));
    }

    #[test]
    fn test_article893_894_reserved_portion() {
        let statute = article893_894();
        let discretion = statute.discretion_logic.as_ref().unwrap();
        assert!(discretion.contains("reserved portion"));
        assert!(discretion.contains("1/2 reserved"));
        assert!(discretion.contains("3/4 reserved"));
    }

    #[test]
    fn test_all_will_articles_have_jurisdiction() {
        let articles = vec![article774_792(), article839_851(), article893_894()];

        for article in articles {
            assert_eq!(article.jurisdiction.as_deref(), Some("FR"));
        }
    }

    #[test]
    fn test_all_will_articles_have_discretion() {
        let articles = vec![article774_792(), article839_851(), article893_894()];

        for article in articles {
            assert!(article.discretion_logic.is_some());
            assert!(!article.discretion_logic.as_ref().unwrap().is_empty());
        }
    }
}
