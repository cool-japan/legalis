//! Statute adapter - converts existing French law validators to Statute types.
//!
//! This module bridges the existing validator-based approach with the new
//! Statute-based reasoning engine, enabling automated legal analysis.

use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};

/// Convert Article 1128 (Contract validity requirements) to Statute
///
/// Article 1128: Three requirements for contract validity:
/// 1. Consent of the parties (Consentement des parties)
/// 2. Capacity to contract (Capacité de contracter)
/// 3. Lawful and certain content (Contenu licite et certain)
#[must_use]
pub fn article1128_to_statute() -> Statute {
    Statute::new(
        "code-civil-1128",
        "Code civil Article 1128 - Conditions de validité du contrat / Contract validity requirements",
        Effect::new(
            EffectType::Grant,
            "Le contrat est valide / Contract is valid",
        )
        .with_parameter("requirement_1", "Consentement des parties / Consent of parties")
        .with_parameter("requirement_2", "Capacité de contracter / Capacity to contract")
        .with_parameter("requirement_3", "Contenu licite et certain / Lawful and certain content"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    // Requirement 1: Consent given
    .with_precondition(Condition::AttributeEquals {
        key: "consent_given".to_string(),
        value: "true".to_string(),
    })
    // Requirement 2: Capacity (age >= 18 AND not under guardianship)
    .with_precondition(Condition::And(
        Box::new(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        }),
        Box::new(Condition::AttributeEquals {
            key: "not_under_guardianship".to_string(),
            value: "true".to_string(),
        }),
    ))
    // Requirement 3: Lawful and certain content
    .with_precondition(Condition::And(
        Box::new(Condition::AttributeEquals {
            key: "content_lawful".to_string(),
            value: "true".to_string(),
        }),
        Box::new(Condition::AttributeEquals {
            key: "content_certain".to_string(),
            value: "true".to_string(),
        }),
    ))
    .with_discretion(
        "La réforme de 2016 a codifié les conditions de validité du contrat. \
        Le consentement doit être libre et éclairé. La capacité est présumée pour les majeurs. \
        Le contenu doit être licite (conforme à l'ordre public) et certain (déterminé ou déterminable).\
        \n\n\
        The 2016 reform codified contract validity requirements. \
        Consent must be free and informed. Capacity is presumed for adults. \
        Content must be lawful (compliant with public order) and certain (determined or determinable)."
    )
}

/// Convert Article 1231 (Damages for breach) to Statute
///
/// Article 1231: Debtor who fails to perform is liable for damages
#[must_use]
pub fn article1231_to_statute() -> Statute {
    Statute::new(
        "code-civil-1231",
        "Code civil Article 1231 - Dommages-intérêts / Damages for breach",
        Effect::new(
            EffectType::MonetaryTransfer,
            "Paiement de dommages-intérêts / Payment of damages",
        )
        .with_parameter("payor", "débiteur / debtor")
        .with_parameter("payee", "créancier / creditor")
        .with_parameter("basis", "inexécution ou retard / non-performance or delay")
        .with_parameter(
            "calculation",
            "perte éprouvée + gain manqué / actual loss + lost profit",
        ),
    )
    .with_jurisdiction("FR")
    // Precondition 1: Non-performance or delayed performance
    .with_precondition(Condition::Or(
        Box::new(Condition::AttributeEquals {
            key: "non_performance".to_string(),
            value: "true".to_string(),
        }),
        Box::new(Condition::AttributeEquals {
            key: "delayed_performance".to_string(),
            value: "true".to_string(),
        }),
    ))
    // Precondition 2: No force majeure
    .with_precondition(Condition::AttributeEquals {
        key: "force_majeure".to_string(),
        value: "false".to_string(),
    })
    // Precondition 3: Harm suffered
    .with_precondition(Condition::AttributeEquals {
        key: "harm_suffered".to_string(),
        value: "true".to_string(),
    })
    .with_discretion(
        "Les dommages-intérêts comprennent la perte éprouvée (damnum emergens) \
        et le gain manqué (lucrum cessans). L'article 1231-5 prévoit que les clauses \
        pénales peuvent fixer à l'avance le montant, mais le juge peut les modérer.\
        \n\n\
        Damages include actual loss (damnum emergens) and lost profit (lucrum cessans). \
        Article 1231-5 provides that penalty clauses may fix the amount in advance, \
        but the judge may moderate them.",
    )
}

/// Convert Article L3121-27 (35-hour work week) to Statute
///
/// Article L3121-27: Legal working duration is 35 hours per week
#[must_use]
pub fn article_l3121_27_to_statute() -> Statute {
    Statute::new(
        "code-travail-l3121-27",
        "Code du travail Article L3121-27 - Durée légale du travail / Legal working duration",
        Effect::new(
            EffectType::Obligation,
            "Prime de heures supplémentaires requise / Overtime premium required",
        )
        .with_parameter("legal_duration", "35 heures / 35 hours")
        .with_parameter("premium_rate_first_8h", "25% / 25%")
        .with_parameter("premium_rate_beyond_8h", "50% / 50%"),
    )
    .with_jurisdiction("FR")
    // Precondition: Weekly hours > 35
    .with_precondition(Condition::Threshold {
        attributes: vec![("weekly_hours".to_string(), 1.0)],
        operator: ComparisonOp::GreaterThan,
        value: 35.0,
    })
    .with_discretion(
        "La durée légale de travail effectif est fixée à 35 heures par semaine depuis 2000. \
        Les heures accomplies au-delà donnent lieu à majoration: +25% pour les 8 premières, \
        +50% au-delà (Article L3121-33). Cette durée légale est un seuil de déclenchement \
        des heures supplémentaires, pas un maximum (voir L3121-20 pour le maximum de 48h).\
        \n\n\
        The legal working duration has been set at 35 hours per week since 2000. \
        Hours worked beyond this are subject to premium rates: +25% for the first 8 hours, \
        +50% beyond (Article L3121-33). This legal duration is a trigger threshold for overtime, \
        not a maximum (see L3121-20 for the 48-hour maximum).",
    )
}

/// Convert Article L225-1 (SA formation) to Statute
///
/// Article L225-1: SA (Société Anonyme) requirements including €37,000 minimum capital
#[must_use]
pub fn article_l225_1_to_statute() -> Statute {
    Statute::new(
        "code-commerce-l225-1",
        "Code de commerce Article L225-1 - Société Anonyme (SA) / Public Limited Company",
        Effect::new(
            EffectType::Grant,
            "Formation de SA valide / Valid SA formation",
        )
        .with_parameter("minimum_capital", "37 000 € / €37,000")
        .with_parameter(
            "minimum_shareholders",
            "7 si cotée, 2 sinon / 7 if listed, 2 otherwise",
        ),
    )
    .with_jurisdiction("FR")
    // Precondition: Capital >= €37,000
    .with_precondition(Condition::Threshold {
        attributes: vec![("capital_eur".to_string(), 1.0)],
        operator: ComparisonOp::GreaterOrEqual,
        value: 37_000.0,
    })
    // Precondition: Company name contains "SA"
    .with_precondition(Condition::Pattern {
        attribute: "company_name".to_string(),
        pattern: "SA".to_string(),
        negated: false,
    })
    .with_discretion(
        "La SA est une société de capitaux dont le capital minimum est de 37 000 € \
        (Article L224-2). Elle doit comporter au moins 7 actionnaires si elle fait appel public \
        à l'épargne, 2 dans les autres cas. La dénomination sociale doit inclure la mention 'SA'.\
        \n\n\
        The SA is a capital company with a minimum capital of €37,000 (Article L224-2). \
        It must have at least 7 shareholders if publicly traded, 2 otherwise. \
        The company name must include the designation 'SA'.",
    )
}

/// Convert Article 1217 (Breach remedies) to Statute
///
/// Article 1217: Five remedies available for breach of contract
#[must_use]
pub fn article1217_to_statute() -> Statute {
    Statute::new(
        "code-civil-1217",
        "Code civil Article 1217 - Sanctions de l'inexécution / Breach remedies",
        Effect::new(
            EffectType::Custom,
            "Remèdes disponibles / Available remedies",
        )
        .with_parameter(
            "remedy_1",
            "Exception d'inexécution / Exception of non-performance",
        )
        .with_parameter("remedy_2", "Exécution forcée / Specific performance")
        .with_parameter("remedy_3", "Réduction du prix / Price reduction")
        .with_parameter("remedy_4", "Résolution / Termination")
        .with_parameter("remedy_5", "Dommages-intérêts / Damages"),
    )
    .with_jurisdiction("FR")
    // Precondition: Breach occurred
    .with_precondition(Condition::AttributeEquals {
        key: "breach_occurred".to_string(),
        value: "true".to_string(),
    })
    .with_discretion(
        "L'article 1217 énumère les cinq sanctions principales de l'inexécution: \
        (1) l'exception d'inexécution permet de suspendre sa propre obligation; \
        (2) l'exécution forcée en nature; (3) la réduction du prix; \
        (4) la résolution (rupture du contrat); (5) les dommages-intérêts. \
        Ces sanctions peuvent se cumuler (par ex. résolution + dommages).\
        \n\n\
        Article 1217 lists the five main remedies for breach: \
        (1) exception of non-performance allows suspending one's own obligation; \
        (2) specific performance; (3) price reduction; \
        (4) termination (contract dissolution); (5) damages. \
        These remedies may be cumulative (e.g., termination + damages).",
    )
}

/// Convert Article L1242-8 (CDD maximum duration) to Statute
///
/// Article L1242-8: Fixed-term contracts (CDD) limited to 18 months maximum
#[must_use]
pub fn article_l1242_8_to_statute() -> Statute {
    Statute::new(
        "code-travail-l1242-8",
        "Code du travail Article L1242-8 - Durée maximale du CDD / CDD maximum duration",
        Effect::new(
            EffectType::Prohibition,
            "CDD de plus de 18 mois interdit / CDD exceeding 18 months prohibited",
        )
        .with_parameter("maximum_duration", "18 mois / 18 months")
        .with_parameter("renewals_included", "oui / yes"),
    )
    .with_jurisdiction("FR")
    // Precondition: Contract type is CDD
    .with_precondition(Condition::AttributeEquals {
        key: "contract_type".to_string(),
        value: "CDD".to_string(),
    })
    // Precondition: Duration > 18 months
    .with_precondition(Condition::Duration {
        operator: ComparisonOp::GreaterThan,
        value: 18,
        unit: legalis_core::DurationUnit::Months,
    })
    .with_discretion(
        "Le CDD ne peut excéder 18 mois, renouvellements compris, sauf exceptions \
        (Article L1242-8). Les renouvellements sont limités à deux fois maximum. \
        Au-delà de 18 mois, le contrat est réputé à durée indéterminée (CDI).\
        \n\n\
        Fixed-term contracts cannot exceed 18 months, renewals included, except in special cases \
        (Article L1242-8). Renewals are limited to a maximum of twice. \
        Beyond 18 months, the contract is deemed indefinite (CDI).",
    )
}

/// Get all contract law statutes
#[must_use]
pub fn contract_law_statutes() -> Vec<Statute> {
    vec![
        article1128_to_statute(),
        article1217_to_statute(),
        article1231_to_statute(),
        // TODO: Add remaining contract law articles:
        // - article1103_to_statute() - Binding force
        // - article1104_to_statute() - Good faith
        // - article1218_to_statute() - Force majeure
        // - etc.
    ]
}

/// Get all labor law statutes
#[must_use]
pub fn labor_law_statutes() -> Vec<Statute> {
    vec![
        article_l3121_27_to_statute(),
        article_l1242_8_to_statute(),
        // TODO: Add remaining labor law articles:
        // - article_l1221_1_to_statute() - CDI principle
        // - article_l1221_19_to_statute() - Trial period
        // - article_l3121_18_to_statute() - Daily maximum (10h)
        // - article_l3121_20_to_statute() - Weekly maximum (48h)
        // - article_l1232_1_to_statute() - Dismissal cause
        // - article_l1232_2_to_statute() - Pre-dismissal interview
        // - article_l1234_1_to_statute() - Notice period
        // - etc. (15 articles total)
    ]
}

/// Get all company law statutes
#[must_use]
pub fn company_law_statutes() -> Vec<Statute> {
    vec![
        article_l225_1_to_statute(),
        // TODO: Add remaining company law articles:
        // - article_l225_17_to_statute() - SA board (3-18 directors)
        // - article_l225_18_to_statute() - Director terms
        // - article_l223_1_to_statute() - SARL definition
        // - article_l223_3_to_statute() - SARL max 100 partners
        // - etc.
    ]
}

/// Get all French law statutes (all 54 articles)
#[must_use]
pub fn all_french_statutes() -> Vec<Statute> {
    let mut statutes = Vec::new();
    statutes.extend(contract_law_statutes());
    statutes.extend(labor_law_statutes());
    statutes.extend(company_law_statutes());
    // TODO: Add family law statutes when implemented in Phase 2
    statutes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_article1128_statute() {
        let statute = article1128_to_statute();
        assert_eq!(statute.id, "code-civil-1128");
        assert_eq!(statute.jurisdiction, Some("FR".to_string()));
        assert_eq!(statute.preconditions.len(), 3); // Three requirements
    }

    #[test]
    fn test_article1231_statute() {
        let statute = article1231_to_statute();
        assert_eq!(statute.id, "code-civil-1231");
        assert!(matches!(
            statute.effect.effect_type,
            EffectType::MonetaryTransfer
        ));
        assert_eq!(statute.preconditions.len(), 3);
    }

    #[test]
    fn test_article_l3121_27_statute() {
        let statute = article_l3121_27_to_statute();
        assert_eq!(statute.id, "code-travail-l3121-27");
        assert!(matches!(statute.effect.effect_type, EffectType::Obligation));
        assert!(!statute.preconditions.is_empty());
    }

    #[test]
    fn test_article_l225_1_statute() {
        let statute = article_l225_1_to_statute();
        assert_eq!(statute.id, "code-commerce-l225-1");
        assert!(matches!(statute.effect.effect_type, EffectType::Grant));
        assert_eq!(statute.preconditions.len(), 2);
    }

    #[test]
    fn test_article1217_statute() {
        let statute = article1217_to_statute();
        assert_eq!(statute.id, "code-civil-1217");
        assert!(statute.effect.parameters.contains_key("remedy_1"));
        assert!(statute.effect.parameters.contains_key("remedy_5"));
    }

    #[test]
    fn test_article_l1242_8_statute() {
        let statute = article_l1242_8_to_statute();
        assert_eq!(statute.id, "code-travail-l1242-8");
        assert!(matches!(
            statute.effect.effect_type,
            EffectType::Prohibition
        ));
    }

    #[test]
    fn test_contract_law_statutes() {
        let statutes = contract_law_statutes();
        assert!(!statutes.is_empty());
        assert!(statutes.iter().any(|s| s.id == "code-civil-1128"));
        assert!(statutes.iter().any(|s| s.id == "code-civil-1231"));
    }

    #[test]
    fn test_labor_law_statutes() {
        let statutes = labor_law_statutes();
        assert!(!statutes.is_empty());
        assert!(statutes.iter().any(|s| s.id == "code-travail-l3121-27"));
    }

    #[test]
    fn test_company_law_statutes() {
        let statutes = company_law_statutes();
        assert!(!statutes.is_empty());
        assert!(statutes.iter().any(|s| s.id == "code-commerce-l225-1"));
    }

    #[test]
    fn test_all_french_statutes() {
        let statutes = all_french_statutes();
        assert!(!statutes.is_empty());
        // Should contain statutes from all domains
        assert!(statutes.iter().any(|s| s.id.starts_with("code-civil")));
        assert!(statutes.iter().any(|s| s.id.starts_with("code-travail")));
        assert!(statutes.iter().any(|s| s.id.starts_with("code-commerce")));
    }

    #[test]
    fn test_all_statutes_have_jurisdiction() {
        let statutes = all_french_statutes();
        for statute in statutes {
            assert_eq!(statute.jurisdiction, Some("FR".to_string()));
        }
    }

    #[test]
    fn test_all_statutes_have_discretion() {
        let statutes = all_french_statutes();
        for statute in statutes {
            assert!(statute.discretion_logic.is_some());
            assert!(!statute.discretion_logic.as_ref().unwrap().is_empty());
        }
    }
}
