//! Statute adapters for EU Law.
//!
//! Converts EU regulations and directives into legalis-core Statute abstractions.

use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};

// ============================================================================
// GDPR (General Data Protection Regulation 2016/679)
// ============================================================================

/// GDPR Article 5 - Principles relating to processing of personal data
#[must_use]
pub fn gdpr_article_5_principles() -> Statute {
    Statute::new(
        "GDPR_Art5",
        "Principles of Data Processing (GDPR Art. 5)",
        Effect::new(
            EffectType::Obligation,
            "Personal data must be processed lawfully, fairly and transparently",
        )
        .with_parameter("principle_1", "lawfulness_fairness_transparency")
        .with_parameter("principle_2", "purpose_limitation")
        .with_parameter("principle_3", "data_minimisation")
        .with_parameter("principle_4", "accuracy")
        .with_parameter("principle_5", "storage_limitation")
        .with_parameter("principle_6", "integrity_confidentiality"),
    )
    .with_jurisdiction("EU")
}

/// GDPR Article 6 - Lawful Basis for Processing
#[must_use]
pub fn gdpr_article_6_lawful_basis() -> Statute {
    Statute::new(
        "GDPR_Art6",
        "Lawful Basis for Processing (GDPR Art. 6)",
        Effect::new(
            EffectType::Obligation,
            "Processing must have at least one lawful basis",
        )
        .with_parameter("basis_a", "consent")
        .with_parameter("basis_b", "contract_performance")
        .with_parameter("basis_c", "legal_obligation")
        .with_parameter("basis_d", "vital_interests")
        .with_parameter("basis_e", "public_task")
        .with_parameter("basis_f", "legitimate_interests"),
    )
    .with_jurisdiction("EU")
}

/// GDPR Article 7 - Conditions for Consent
#[must_use]
pub fn gdpr_article_7_consent() -> Statute {
    Statute::new(
        "GDPR_Art7",
        "Conditions for Consent (GDPR Art. 7)",
        Effect::new(
            EffectType::Obligation,
            "Consent must be freely given, specific, informed, and unambiguous",
        )
        .with_parameter("freely_given", "required")
        .with_parameter("specific", "required")
        .with_parameter("informed", "required")
        .with_parameter("unambiguous", "required")
        .with_parameter("withdrawable", "as_easy_as_giving"),
    )
    .with_jurisdiction("EU")
}

/// GDPR Article 9 - Special Categories of Personal Data
#[must_use]
pub fn gdpr_article_9_special_categories() -> Statute {
    Statute::new(
        "GDPR_Art9",
        "Special Categories of Data (GDPR Art. 9)",
        Effect::new(
            EffectType::Prohibition,
            "Processing of special category data is prohibited unless exception applies",
        )
        .with_parameter("category_1", "racial_ethnic_origin")
        .with_parameter("category_2", "political_opinions")
        .with_parameter("category_3", "religious_beliefs")
        .with_parameter("category_4", "trade_union_membership")
        .with_parameter("category_5", "genetic_data")
        .with_parameter("category_6", "biometric_data")
        .with_parameter("category_7", "health_data")
        .with_parameter("category_8", "sex_life_orientation"),
    )
    .with_jurisdiction("EU")
}

/// GDPR Article 17 - Right to Erasure
#[must_use]
pub fn gdpr_article_17_erasure() -> Statute {
    Statute::new(
        "GDPR_Art17",
        "Right to Erasure (GDPR Art. 17)",
        Effect::new(
            EffectType::Grant,
            "Data subject has right to erasure without undue delay",
        )
        .with_parameter("response_time", "without_undue_delay"),
    )
    .with_jurisdiction("EU")
}

/// GDPR Article 25 - Data Protection by Design and Default
#[must_use]
pub fn gdpr_article_25_dpbd() -> Statute {
    Statute::new(
        "GDPR_Art25",
        "Data Protection by Design and Default (GDPR Art. 25)",
        Effect::new(
            EffectType::Obligation,
            "Controller must implement appropriate technical and organisational measures",
        ),
    )
    .with_jurisdiction("EU")
}

/// GDPR Article 32 - Security of Processing
#[must_use]
pub fn gdpr_article_32_security() -> Statute {
    Statute::new(
        "GDPR_Art32",
        "Security of Processing (GDPR Art. 32)",
        Effect::new(
            EffectType::Obligation,
            "Controller and processor must implement appropriate security measures",
        )
        .with_parameter("measure_1", "pseudonymisation")
        .with_parameter("measure_2", "encryption")
        .with_parameter("measure_3", "confidentiality")
        .with_parameter("measure_4", "integrity")
        .with_parameter("measure_5", "availability")
        .with_parameter("measure_6", "resilience"),
    )
    .with_jurisdiction("EU")
}

/// GDPR Article 33 - Breach Notification to Authority
#[must_use]
pub fn gdpr_article_33_breach_notification() -> Statute {
    Statute::new(
        "GDPR_Art33",
        "Breach Notification to Supervisory Authority (GDPR Art. 33)",
        Effect::new(
            EffectType::Obligation,
            "Controller must notify supervisory authority within 72 hours of breach",
        )
        .with_parameter("notification_deadline_hours", "72"),
    )
    .with_jurisdiction("EU")
}

/// GDPR Article 35 - Data Protection Impact Assessment
#[must_use]
pub fn gdpr_article_35_dpia() -> Statute {
    Statute::new(
        "GDPR_Art35",
        "Data Protection Impact Assessment (GDPR Art. 35)",
        Effect::new(
            EffectType::Obligation,
            "DPIA required for high-risk processing operations",
        ),
    )
    .with_jurisdiction("EU")
}

/// GDPR Article 37 - Designation of DPO
#[must_use]
pub fn gdpr_article_37_dpo() -> Statute {
    Statute::new(
        "GDPR_Art37",
        "Designation of Data Protection Officer (GDPR Art. 37)",
        Effect::new(
            EffectType::Obligation,
            "Controller must designate DPO in specified circumstances",
        ),
    )
    .with_jurisdiction("EU")
}

/// GDPR Article 83 - Administrative Fines
#[must_use]
pub fn gdpr_article_83_fines() -> Statute {
    Statute::new(
        "GDPR_Art83",
        "Administrative Fines (GDPR Art. 83)",
        Effect::new(
            EffectType::MonetaryTransfer,
            "Infringements subject to administrative fines",
        )
        .with_parameter("tier_1_max_eur", "10000000")
        .with_parameter("tier_1_max_percent", "2")
        .with_parameter("tier_2_max_eur", "20000000")
        .with_parameter("tier_2_max_percent", "4"),
    )
    .with_jurisdiction("EU")
}

// ============================================================================
// Competition Law (Articles 101-102 TFEU)
// ============================================================================

/// TFEU Article 101 - Anti-competitive Agreements
#[must_use]
pub fn tfeu_article_101() -> Statute {
    Statute::new(
        "TFEU_Art101",
        "Anti-competitive Agreements (TFEU Art. 101)",
        Effect::new(
            EffectType::Prohibition,
            "Agreements between undertakings which restrict competition are prohibited",
        )
        .with_parameter("price_fixing", "prohibited")
        .with_parameter("market_sharing", "prohibited")
        .with_parameter("output_limitation", "prohibited"),
    )
    .with_jurisdiction("EU")
}

/// TFEU Article 101(3) - Exemption Criteria
#[must_use]
pub fn tfeu_article_101_3_exemption() -> Statute {
    Statute::new(
        "TFEU_Art101_3",
        "Exemption Criteria (TFEU Art. 101(3))",
        Effect::new(
            EffectType::Grant,
            "Article 101(1) may be declared inapplicable if conditions met",
        )
        .with_parameter("condition_1", "efficiency_gains")
        .with_parameter("condition_2", "fair_share_to_consumers")
        .with_parameter("condition_3", "indispensable_restrictions")
        .with_parameter("condition_4", "no_elimination_of_competition"),
    )
    .with_jurisdiction("EU")
}

/// TFEU Article 102 - Abuse of Dominant Position
#[must_use]
pub fn tfeu_article_102() -> Statute {
    Statute::new(
        "TFEU_Art102",
        "Abuse of Dominant Position (TFEU Art. 102)",
        Effect::new(
            EffectType::Prohibition,
            "Abuse of dominant position within internal market is prohibited",
        )
        .with_parameter("abuse_a", "unfair_prices")
        .with_parameter("abuse_b", "limiting_production")
        .with_parameter("abuse_c", "discrimination")
        .with_parameter("abuse_d", "tying"),
    )
    .with_precondition(Condition::Percentage {
        context: "market_share".to_string(),
        operator: ComparisonOp::GreaterOrEqual,
        value: 40,
    })
    .with_jurisdiction("EU")
}

/// De Minimis Notice - Safe harbor for minor agreements
#[must_use]
pub fn de_minimis_notice() -> Statute {
    Statute::new(
        "EU_DeMinimis",
        "De Minimis Notice (2014/C 291/01)",
        Effect::new(
            EffectType::Grant,
            "Agreements of minor importance fall outside Article 101(1)",
        )
        .with_parameter("competitor_threshold", "10")
        .with_parameter("non_competitor_threshold", "15"),
    )
    .with_precondition(Condition::Percentage {
        context: "market_share".to_string(),
        operator: ComparisonOp::LessThan,
        value: 10,
    })
    .with_jurisdiction("EU")
}

// ============================================================================
// Consumer Rights Directive (2011/83/EU)
// ============================================================================

/// Consumer Rights Directive - Withdrawal Right
#[must_use]
pub fn consumer_rights_withdrawal() -> Statute {
    Statute::new(
        "CRD_Art9",
        "Right of Withdrawal (Consumer Rights Directive Art. 9)",
        Effect::new(
            EffectType::Grant,
            "Consumer has 14-day withdrawal period for distance contracts",
        )
        .with_parameter("withdrawal_period_days", "14"),
    )
    .with_jurisdiction("EU")
}

/// Consumer Rights Directive - Information Requirements
#[must_use]
pub fn consumer_rights_information() -> Statute {
    Statute::new(
        "CRD_Art6",
        "Information Requirements (Consumer Rights Directive Art. 6)",
        Effect::new(
            EffectType::Obligation,
            "Trader must provide specified information before consumer is bound",
        ),
    )
    .with_jurisdiction("EU")
}

// ============================================================================
// Aggregation Functions
// ============================================================================

/// Get all GDPR statutes
#[must_use]
pub fn gdpr_statutes() -> Vec<Statute> {
    vec![
        gdpr_article_5_principles(),
        gdpr_article_6_lawful_basis(),
        gdpr_article_7_consent(),
        gdpr_article_9_special_categories(),
        gdpr_article_17_erasure(),
        gdpr_article_25_dpbd(),
        gdpr_article_32_security(),
        gdpr_article_33_breach_notification(),
        gdpr_article_35_dpia(),
        gdpr_article_37_dpo(),
        gdpr_article_83_fines(),
    ]
}

/// Get all competition law statutes
#[must_use]
pub fn competition_statutes() -> Vec<Statute> {
    vec![
        tfeu_article_101(),
        tfeu_article_101_3_exemption(),
        tfeu_article_102(),
        de_minimis_notice(),
    ]
}

/// Get all consumer rights statutes
#[must_use]
pub fn consumer_rights_statutes() -> Vec<Statute> {
    vec![consumer_rights_withdrawal(), consumer_rights_information()]
}

/// Get all EU statutes
#[must_use]
pub fn all_eu_statutes() -> Vec<Statute> {
    let mut statutes = Vec::new();
    statutes.extend(gdpr_statutes());
    statutes.extend(competition_statutes());
    statutes.extend(consumer_rights_statutes());
    statutes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gdpr_statutes() {
        let statutes = gdpr_statutes();
        assert!(!statutes.is_empty());
        assert!(statutes.iter().any(|s| s.id == "GDPR_Art6"));
        assert!(statutes.iter().any(|s| s.id == "GDPR_Art83"));
    }

    #[test]
    fn test_competition_statutes() {
        let statutes = competition_statutes();
        assert!(!statutes.is_empty());
        assert!(statutes.iter().any(|s| s.id == "TFEU_Art101"));
        assert!(statutes.iter().any(|s| s.id == "TFEU_Art102"));
    }

    #[test]
    fn test_all_eu_statutes() {
        let statutes = all_eu_statutes();
        assert!(statutes.len() >= 17);
    }

    #[test]
    fn test_statute_jurisdiction() {
        let statute = gdpr_article_6_lawful_basis();
        assert_eq!(statute.jurisdiction.as_deref(), Some("EU"));
    }
}
