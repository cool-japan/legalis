//! Succession order articles (Code civil, Book III)
//!
//! This module implements the key articles governing succession
//! order, estate distribution, and debt liability in French law.

use legalis_core::{Condition, Effect, EffectType, Statute};

/// Article 720 - Succession opens at death
///
/// # French Text
/// "Les successions s'ouvrent par la mort, au dernier domicile du défunt."
///
/// # English Translation
/// "Successions open upon death, at the last domicile of the deceased."
///
/// # Legal Commentary
/// This fundamental article establishes two key principles:
/// 1. **Succession opens at the moment of death** - Not before (no succession of living persons)
/// 2. **Jurisdiction determined by last domicile** - French law applies if deceased lived in France
///
/// ## Historical Context
/// This article dates from the original Napoleonic Code of 1804 and embodies the principle
/// that succession only concerns deceased persons (no "pacte sur succession future").
///
/// ## Comparison with Other Jurisdictions
/// - **Germany (BGB §1922)**: Similar - succession opens upon death
/// - **Japan (Minpo §882)**: Similar - succession opens upon death at domicile
/// - **Common Law**: Uses "death" as trigger, but jurisdiction rules differ
///
/// ## Brussels IV Regulation (2015)
/// For international successions, EU Regulation 650/2012 allows choice between:
/// - Law of habitual residence (default)
/// - Law of nationality (if chosen in will)
///
/// # Examples
///
/// ```
/// use legalis_fr::inheritance::{Succession, Person};
/// use chrono::NaiveDate;
///
/// let deceased = Person::new("Jean Dupont".to_string(), 75);
/// let succession = Succession::new(deceased, NaiveDate::from_ymd_opt(2024, 1, 15).unwrap())
///     .with_last_domicile("Paris, France".to_string());
///
/// // Succession opens at death (Article 720)
/// assert_eq!(succession.death_date, NaiveDate::from_ymd_opt(2024, 1, 15).unwrap());
/// assert_eq!(succession.last_domicile, "Paris, France");
/// ```
pub fn article720() -> Statute {
    Statute::new(
        "code-civil-720",
        "Code civil Article 720 - Ouverture de la succession / Succession opens at death",
        Effect::new(
            EffectType::StatusChange,
            "La succession s'ouvre à la mort au dernier domicile / Succession opens at death at last domicile",
        )
        .with_parameter("opening_date", "death_date")
        .with_parameter("jurisdiction", "last_domicile"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    .with_precondition(Condition::AttributeEquals {
        key: "deceased_status".to_string(),
        value: "dead".to_string(),
    })
    .with_precondition(Condition::HasAttribute {
        key: "death_date".to_string(),
    })
    .with_precondition(Condition::HasAttribute {
        key: "last_domicile".to_string(),
    })
    .with_discretion(
        "Historical note: This article from 1804 Napoleonic Code establishes that succession \
         only opens upon death (no succession of living persons). For international successions, \
         Brussels IV Regulation (EU 650/2012) allows choice between habitual residence law and \
         nationality law."
    )
}

/// Article 721 - Freedom of disposition vs. Reserved portion
///
/// # French Text
/// "La loi assure la primauté de la volonté du défunt. Elle règle les conditions de la validité des libéralités, \
/// garantit les droits des héritiers réservataires et de l'époux survivant, et préserve ainsi un juste équilibre."
///
/// # English Translation
/// "The law ensures the primacy of the deceased's will. It regulates the conditions for the validity of gifts, \
/// guarantees the rights of forced heirs and the surviving spouse, and thus preserves a fair balance."
///
/// # Legal Commentary
/// Reformed in 2006, this article balances two fundamental principles:
/// 1. **Testamentary freedom** (liberté testamentaire)
/// 2. **Reserved portion** for descendants (réserve héréditaire)
///
/// This represents the French compromise between:
/// - Anglo-American complete testamentary freedom
/// - Historic forced heirship systems
pub fn article721() -> Statute {
    Statute::new(
        "code-civil-721",
        "Code civil Article 721 - Balance between freedom and reserved portion",
        Effect::new(
            EffectType::Obligation,
            "Respect both testamentary freedom and reserved portion rights",
        )
        .with_parameter("principle", "primacy_of_will")
        .with_parameter("guarantee", "reserved_portion_rights"),
    )
    .with_jurisdiction("FR")
    .with_version(2) // Reformed 2006
    .with_discretion(
        "2006 Reform: Modernized to emphasize testamentary freedom while maintaining \
         reserved portion protection for descendants. Reflects French balance between \
         individual autonomy and family solidarity.",
    )
}

/// Article 724 - Heirs continue legal personality
///
/// # French Text
/// "Les héritiers désignés par la loi... sont saisis de plein droit des biens, droits et actions du défunt."
///
/// # English Translation
/// "The heirs designated by law... are seized by right of the deceased's property, rights, and actions."
///
/// # Legal Commentary
/// **Saisine** is a unique French concept meaning automatic transfer upon death.
/// Heirs immediately acquire rights without formalities, subject to:
/// - Acceptance (pure, with benefit of inventory, or renunciation)
/// - Payment of debts (Article 873)
pub fn article724() -> Statute {
    Statute::new(
        "code-civil-724",
        "Code civil Article 724 - Heirs seized by right (saisine)",
        Effect::new(
            EffectType::Grant,
            "Heirs automatically acquire deceased's property, rights, and actions",
        )
        .with_parameter("principle", "saisine")
        .with_parameter("timing", "immediate_upon_death"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    .with_precondition(Condition::AttributeEquals {
        key: "heir_designated".to_string(),
        value: "true".to_string(),
    })
    .with_discretion(
        "Saisine principle: French heirs are automatically 'seized' of the estate upon death, \
         unlike common law where personal representatives must be appointed. Heirs may still \
         accept, accept with benefit of inventory, or renounce.",
    )
}

/// Article 735 - Equal distribution among descendants
///
/// # French Text
/// "Lorsque le défunt ne laisse ni postérité, ni frères ou sœurs, ni descendants de ces derniers, \
/// les biens se divisent par moitié entre les ascendants de la ligne paternelle et les ascendants de la ligne maternelle."
///
/// But more commonly applied to children:
/// "Les enfants ou leurs descendants succèdent à leurs père et mère... par égales portions et par têtes."
///
/// # English Translation
/// "Children or their descendants succeed to their father and mother... in equal shares and per capita."
///
/// # Legal Commentary
/// This establishes the French principle of **égalité successorale** (succession equality):
/// - All children inherit equally
/// - No primogeniture
/// - No gender discrimination
///
/// ## Historical Evolution
/// - Pre-1789: Primogeniture for nobility
/// - 1804 Code Napoleon: Abolished primogeniture, established equality
/// - Modern: Complete equality, including non-marital children (since 2001 reform)
pub fn article735() -> Statute {
    Statute::new(
        "code-civil-735",
        "Code civil Article 735 - Equal distribution among children",
        Effect::new(
            EffectType::MonetaryTransfer,
            "Estate divided equally among children (égalité successorale)",
        )
        .with_parameter("distribution_method", "equal_shares_per_capita")
        .with_parameter("principle", "no_primogeniture"),
    )
    .with_jurisdiction("FR")
    .with_version(2) // Updated 2001 to include non-marital children
    .with_precondition(Condition::And(
        Box::new(Condition::HasAttribute {
            key: "children".to_string(),
        }),
        Box::new(Condition::Not(Box::new(Condition::AttributeEquals {
            key: "will_specifies_unequal".to_string(),
            value: "true".to_string(),
        }))),
    ))
    .with_discretion(
        "Égalité successorale: French revolutionary principle abolishing primogeniture. \
         All children inherit equally by default. 2001 reform extended equality to non-marital \
         children, ending historical discrimination.",
    )
}

/// Article 873 - Heirs liable for debts in proportion to shares
///
/// # French Text
/// "Les héritiers sont tenus des dettes et charges de la succession, personnellement pour leur part héréditaire, \
/// et hypothécairement pour le tout."
///
/// # English Translation
/// "Heirs are liable for debts and charges of the succession, personally for their hereditary share, \
/// and secured by mortgage for the whole."
///
/// # Legal Commentary
/// This article establishes the **liability regime** for succession debts:
///
/// ## Two Types of Liability
/// 1. **Personal liability** (obligation personnelle): Each heir liable for their share only
/// 2. **Real liability** (obligation hypothécaire): Estate property itself liable for all debts
///
/// ## Protection Mechanisms
/// Heirs can protect themselves by:
/// - **Accepting with benefit of inventory** (acceptation à concurrence de l'actif net)
/// - **Renouncing** (renonciation) - not liable for any debts
///
/// ## Creditor Rights
/// - Creditors can pursue estate property for full debt
/// - Among heirs, each pays proportionally
///
/// ## Comparison with Other Systems
/// - **Germany**: Similar proportional liability
/// - **Common Law**: Estate pays debts before distribution
/// - **Japan**: Similar benefit of inventory system
pub fn article873() -> Statute {
    Statute::new(
        "code-civil-873",
        "Code civil Article 873 - Liability for estate debts",
        Effect::new(
            EffectType::Obligation,
            "Heirs liable for debts: personally pro rata, real liability on all estate property",
        )
        .with_parameter("personal_liability", "proportional_to_share")
        .with_parameter("real_liability", "entire_estate")
        .with_parameter("protection", "benefit_of_inventory_available"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    .with_precondition(Condition::HasAttribute {
        key: "estate_debts".to_string(),
    })
    .with_discretion(
        "Debt liability: Each heir personally liable for their share of debts, but estate \
         property itself (real liability) secures all debts. Heirs can limit liability by \
         accepting with benefit of inventory (Article 787 et seq).",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_article720_creation() {
        let statute = article720();
        assert_eq!(statute.id, "code-civil-720");
        assert_eq!(statute.jurisdiction.as_deref(), Some("FR"));
        assert!(statute.title.contains("720"));
    }

    #[test]
    fn test_article720_preconditions() {
        let statute = article720();
        let preconditions = &statute.preconditions;
        assert_eq!(preconditions.len(), 3);
    }

    #[test]
    fn test_article721_reformed() {
        let statute = article721();
        assert_eq!(statute.version, 2); // 2006 reform
        assert!(statute.discretion_logic.as_ref().unwrap().contains("2006"));
    }

    #[test]
    fn test_article724_saisine() {
        let statute = article724();
        assert!(statute.title.contains("saisine"));
        assert!(
            statute
                .discretion_logic
                .as_ref()
                .unwrap()
                .contains("seized")
        );
    }

    #[test]
    fn test_article735_equality() {
        let statute = article735();
        assert!(
            statute
                .discretion_logic
                .as_ref()
                .unwrap()
                .contains("Égalité")
        );
        assert_eq!(statute.version, 2); // 2001 reform
    }

    #[test]
    fn test_article873_debt_liability() {
        let statute = article873();
        assert!(
            statute
                .discretion_logic
                .as_ref()
                .unwrap()
                .contains("liability")
        );
        assert!(statute.title.contains("873"));
    }

    #[test]
    fn test_all_articles_have_jurisdiction() {
        let articles = vec![
            article720(),
            article721(),
            article724(),
            article735(),
            article873(),
        ];

        for article in articles {
            assert_eq!(article.jurisdiction.as_deref(), Some("FR"));
        }
    }

    #[test]
    fn test_all_articles_have_discretion() {
        let articles = vec![
            article720(),
            article721(),
            article724(),
            article735(),
            article873(),
        ];

        for article in articles {
            assert!(article.discretion_logic.is_some());
        }
    }
}
