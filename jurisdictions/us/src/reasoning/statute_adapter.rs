//! Statute adapters for US Federal Law.
//!
//! Converts US federal law provisions into legalis-core Statute abstractions.

use legalis_core::{ComparisonOp, Condition, DurationUnit, Effect, EffectType, Statute};

// ============================================================================
// Fair Labor Standards Act (FLSA) - 29 U.S.C. § 201 et seq.
// ============================================================================

/// FLSA § 206 - Minimum Wage
#[must_use]
pub fn flsa_minimum_wage() -> Statute {
    Statute::new(
        "FLSA_206",
        "Minimum Wage (29 U.S.C. § 206)",
        Effect::new(
            EffectType::Obligation,
            "Employer must pay at least federal minimum wage",
        )
        .with_parameter("federal_minimum_wage", "7.25")
        .with_parameter("tipped_minimum", "2.13"),
    )
    .with_jurisdiction("US")
}

/// FLSA § 207 - Overtime Pay
#[must_use]
pub fn flsa_overtime() -> Statute {
    Statute::new(
        "FLSA_207",
        "Overtime Pay (29 U.S.C. § 207)",
        Effect::new(
            EffectType::Obligation,
            "Non-exempt employees must receive 1.5x pay for hours over 40/week",
        )
        .with_parameter("overtime_threshold_hours", "40")
        .with_parameter("overtime_rate", "1.5"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "exempt_status".to_string(),
        value: "non_exempt".to_string(),
    })
    .with_jurisdiction("US")
}

/// FLSA § 212 - Child Labor
#[must_use]
pub fn flsa_child_labor() -> Statute {
    Statute::new(
        "FLSA_212",
        "Child Labor Restrictions (29 U.S.C. § 212)",
        Effect::new(
            EffectType::Prohibition,
            "Minors under 18 restricted from hazardous occupations",
        )
        .with_parameter("minimum_age_non_agricultural", "14")
        .with_parameter("minimum_age_hazardous", "18"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::LessThan,
        value: 18,
    })
    .with_jurisdiction("US")
}

// ============================================================================
// Family and Medical Leave Act (FMLA) - 29 U.S.C. § 2601 et seq.
// ============================================================================

/// FMLA § 2612 - Leave Entitlement
#[must_use]
pub fn fmla_leave_entitlement() -> Statute {
    Statute::new(
        "FMLA_2612",
        "Family and Medical Leave (29 U.S.C. § 2612)",
        Effect::new(
            EffectType::Grant,
            "Eligible employees entitled to 12 weeks unpaid leave per year",
        )
        .with_parameter("leave_weeks", "12")
        .with_parameter("employer_size_threshold", "50"),
    )
    .with_precondition(Condition::Duration {
        operator: ComparisonOp::GreaterOrEqual,
        value: 12,
        unit: DurationUnit::Months,
    })
    .with_jurisdiction("US")
}

// ============================================================================
// Americans with Disabilities Act (ADA) - 42 U.S.C. § 12101 et seq.
// ============================================================================

/// ADA Title I - Employment Discrimination
#[must_use]
pub fn ada_employment() -> Statute {
    Statute::new(
        "ADA_12112",
        "Employment Discrimination (42 U.S.C. § 12112)",
        Effect::new(
            EffectType::Prohibition,
            "Employers cannot discriminate based on disability; must provide reasonable accommodation",
        )
        .with_parameter("employer_size_threshold", "15"),
    )
    .with_jurisdiction("US")
}

// ============================================================================
// Title VII - Civil Rights Act of 1964
// ============================================================================

/// Title VII § 703 - Unlawful Employment Practices
#[must_use]
pub fn title_vii_employment() -> Statute {
    Statute::new(
        "TitleVII_703",
        "Unlawful Employment Practices (42 U.S.C. § 2000e-2)",
        Effect::new(
            EffectType::Prohibition,
            "Discrimination based on race, color, religion, sex, or national origin prohibited",
        )
        .with_parameter("employer_size_threshold", "15"),
    )
    .with_jurisdiction("US")
}

// ============================================================================
// Internal Revenue Code - Tax Provisions
// ============================================================================

/// IRC § 1 - Income Tax Rates
#[must_use]
pub fn irc_income_tax() -> Statute {
    Statute::new(
        "IRC_1",
        "Federal Income Tax (26 U.S.C. § 1)",
        Effect::new(
            EffectType::Obligation,
            "Individuals must pay federal income tax on taxable income",
        )
        .with_parameter("top_marginal_rate_2024", "0.37"),
    )
    .with_jurisdiction("US")
}

/// IRC § 61 - Gross Income Definition
#[must_use]
pub fn irc_gross_income() -> Statute {
    Statute::new(
        "IRC_61",
        "Gross Income Definition (26 U.S.C. § 61)",
        Effect::new(
            EffectType::StatusChange,
            "All income from whatever source derived is included in gross income",
        ),
    )
    .with_jurisdiction("US")
}

// ============================================================================
// Uniform Commercial Code (Model State Law)
// ============================================================================

/// UCC Article 2 - Sales
#[must_use]
pub fn ucc_article_2() -> Statute {
    Statute::new(
        "UCC_Art2",
        "Sale of Goods (UCC Article 2)",
        Effect::new(
            EffectType::Grant,
            "Governs contracts for sale of goods; adopted by 49 states",
        )
        .with_parameter("adoption_count", "49"),
    )
    .with_jurisdiction("US")
}

/// UCC Article 9 - Secured Transactions
#[must_use]
pub fn ucc_article_9() -> Statute {
    Statute::new(
        "UCC_Art9",
        "Secured Transactions (UCC Article 9)",
        Effect::new(
            EffectType::Grant,
            "Governs security interests in personal property; adopted by all 50 states",
        )
        .with_parameter("adoption_count", "50"),
    )
    .with_jurisdiction("US")
}

/// Get all federal employment statutes
#[must_use]
pub fn employment_statutes() -> Vec<Statute> {
    vec![
        flsa_minimum_wage(),
        flsa_overtime(),
        flsa_child_labor(),
        fmla_leave_entitlement(),
        ada_employment(),
        title_vii_employment(),
    ]
}

/// Get all federal tax statutes
#[must_use]
pub fn tax_statutes() -> Vec<Statute> {
    vec![irc_income_tax(), irc_gross_income()]
}

/// Get all UCC model statutes
#[must_use]
pub fn ucc_statutes() -> Vec<Statute> {
    vec![ucc_article_2(), ucc_article_9()]
}

/// Get all federal statutes
#[must_use]
pub fn all_federal_statutes() -> Vec<Statute> {
    let mut statutes = Vec::new();
    statutes.extend(employment_statutes());
    statutes.extend(tax_statutes());
    statutes.extend(ucc_statutes());
    statutes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_employment_statutes() {
        let statutes = employment_statutes();
        assert!(!statutes.is_empty());
        assert!(statutes.iter().any(|s| s.id == "FLSA_206"));
        assert!(statutes.iter().any(|s| s.id == "FLSA_207"));
    }

    #[test]
    fn test_all_federal_statutes() {
        let statutes = all_federal_statutes();
        assert!(statutes.len() >= 8);
    }

    #[test]
    fn test_statute_jurisdiction() {
        let statute = flsa_minimum_wage();
        assert_eq!(statute.jurisdiction.as_deref(), Some("US"));
    }
}
