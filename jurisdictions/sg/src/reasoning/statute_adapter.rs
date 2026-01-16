//! Statute adapters for Singapore law.
//!
//! This module converts Singapore legal provisions into `legalis-core::Statute`
//! representations for use with the reasoning engine.

use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};

/// Employment Act (Cap. 91) statutes
pub fn employment_act_statutes() -> Vec<Statute> {
    vec![
        ea_section_38_working_hours(),
        ea_section_38_overtime(),
        ea_section_43_annual_leave(),
        ea_section_89_sick_leave(),
        ea_section_10_notice_period(),
    ]
}

/// Companies Act (Cap. 50) statutes
pub fn companies_act_statutes() -> Vec<Statute> {
    vec![
        ca_section_145_resident_director(),
        ca_section_171_company_secretary(),
        ca_section_175_agm(),
        ca_section_197_annual_return(),
    ]
}

/// Banking Act (Cap. 19) statutes
pub fn banking_act_statutes() -> Vec<Statute> {
    vec![
        ba_section_10_capital_adequacy(),
        ba_section_27_aml_compliance(),
    ]
}

/// Payment Services Act 2019 statutes
pub fn payment_services_act_statutes() -> Vec<Statute> {
    vec![
        psa_section_6_license_requirement(),
        psa_section_23_safeguarding(),
    ]
}

/// PDPA statutes
pub fn pdpa_statutes() -> Vec<Statute> {
    vec![
        pdpa_section_13_consent(),
        pdpa_section_26b_breach_notification(),
    ]
}

/// All Singapore statutes
pub fn all_singapore_statutes() -> Vec<Statute> {
    let mut statutes = Vec::new();
    statutes.extend(employment_act_statutes());
    statutes.extend(companies_act_statutes());
    statutes.extend(banking_act_statutes());
    statutes.extend(payment_services_act_statutes());
    statutes.extend(pdpa_statutes());
    statutes
}

// ============================================================================
// Employment Act (Cap. 91) Statute Definitions
// ============================================================================

/// EA s. 38(1) - Maximum working hours
fn ea_section_38_working_hours() -> Statute {
    Statute::new(
        "EA_s38_1",
        "Working Hours - Employment Act s. 38(1)",
        Effect::new(
            EffectType::Obligation,
            "Employer must limit working hours to statutory maximum",
        )
        .with_parameter("max_hours_non_shift", "44")
        .with_parameter("max_hours_shift", "48")
        .with_parameter("max_hours_daily", "12"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "covered_by_ea".to_string(),
        value: "true".to_string(),
    })
    .with_precondition(Condition::Calculation {
        formula: "weekly_hours".to_string(),
        operator: ComparisonOp::LessOrEqual,
        value: 44.0,
    })
    .with_jurisdiction("SG")
}

/// EA s. 38(4) - Overtime pay
fn ea_section_38_overtime() -> Statute {
    Statute::new(
        "EA_s38_4",
        "Overtime Pay - Employment Act s. 38(4)",
        Effect::new(
            EffectType::Obligation,
            "Employer must pay overtime at 1.5x regular rate",
        )
        .with_parameter("overtime_rate", "1.5")
        .with_parameter("max_overtime_monthly", "72"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "overtime_eligible".to_string(),
        value: "true".to_string(),
    })
    .with_precondition(Condition::Calculation {
        formula: "overtime_hours".to_string(),
        operator: ComparisonOp::GreaterThan,
        value: 0.0,
    })
    .with_jurisdiction("SG")
}

/// EA s. 43 - Annual leave entitlement
fn ea_section_43_annual_leave() -> Statute {
    Statute::new(
        "EA_s43",
        "Annual Leave - Employment Act s. 43",
        Effect::new(
            EffectType::Grant,
            "Employee entitled to paid annual leave based on service years",
        )
        .with_parameter("year_1", "7")
        .with_parameter("year_2", "8")
        .with_parameter("year_3_4", "9")
        .with_parameter("year_5_6", "11")
        .with_parameter("year_7_8", "12")
        .with_parameter("year_8_plus", "14"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "covered_by_ea".to_string(),
        value: "true".to_string(),
    })
    .with_precondition(Condition::Duration {
        operator: ComparisonOp::GreaterOrEqual,
        value: 3,
        unit: legalis_core::DurationUnit::Months,
    })
    .with_jurisdiction("SG")
}

/// EA s. 89 - Sick leave
fn ea_section_89_sick_leave() -> Statute {
    Statute::new(
        "EA_s89",
        "Sick Leave - Employment Act s. 89",
        Effect::new(
            EffectType::Grant,
            "Employee entitled to paid sick leave after 3 months service",
        )
        .with_parameter("outpatient_days", "14")
        .with_parameter("hospitalization_days", "60"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "covered_by_ea".to_string(),
        value: "true".to_string(),
    })
    .with_precondition(Condition::Duration {
        operator: ComparisonOp::GreaterOrEqual,
        value: 3,
        unit: legalis_core::DurationUnit::Months,
    })
    .with_jurisdiction("SG")
}

/// EA s. 10/11 - Notice period
fn ea_section_10_notice_period() -> Statute {
    Statute::new(
        "EA_s10_11",
        "Notice of Termination - Employment Act s. 10/11",
        Effect::new(
            EffectType::Obligation,
            "Either party must give notice based on service length",
        )
        .with_parameter("less_26_weeks", "1_day")
        .with_parameter("26_weeks_to_2_years", "1_week")
        .with_parameter("2_to_5_years", "2_weeks")
        .with_parameter("over_5_years", "4_weeks"),
    )
    .with_jurisdiction("SG")
}

// ============================================================================
// Companies Act (Cap. 50) Statute Definitions
// ============================================================================

/// CA s. 145 - Resident director requirement
fn ca_section_145_resident_director() -> Statute {
    Statute::new(
        "CA_s145",
        "Resident Director - Companies Act s. 145",
        Effect::new(
            EffectType::Obligation,
            "Company must have at least one director ordinarily resident in Singapore",
        )
        .with_parameter("minimum_resident_directors", "1"),
    )
    .with_precondition(Condition::Geographic {
        region_type: legalis_core::RegionType::Country,
        region_id: "SG".to_string(),
    })
    .with_jurisdiction("SG")
}

/// CA s. 171 - Company secretary requirement
fn ca_section_171_company_secretary() -> Statute {
    Statute::new(
        "CA_s171",
        "Company Secretary - Companies Act s. 171",
        Effect::new(
            EffectType::Obligation,
            "Company must appoint a secretary within 6 months of incorporation",
        )
        .with_parameter("deadline_months", "6"),
    )
    .with_jurisdiction("SG")
}

/// CA s. 175 - AGM requirement
fn ca_section_175_agm() -> Statute {
    Statute::new(
        "CA_s175",
        "Annual General Meeting - Companies Act s. 175",
        Effect::new(
            EffectType::Obligation,
            "Company must hold AGM within 18 months of incorporation, then annually",
        )
        .with_parameter("first_agm_months", "18")
        .with_parameter("subsequent_agm_months", "15"),
    )
    .with_jurisdiction("SG")
}

/// CA s. 197 - Annual return requirement
fn ca_section_197_annual_return() -> Statute {
    Statute::new(
        "CA_s197",
        "Annual Return - Companies Act s. 197",
        Effect::new(
            EffectType::Obligation,
            "Company must file annual return within 7 months of FYE",
        )
        .with_parameter("deadline_months", "7"),
    )
    .with_jurisdiction("SG")
}

// ============================================================================
// Banking Act (Cap. 19) Statute Definitions
// ============================================================================

/// BA s. 10 - Capital adequacy
fn ba_section_10_capital_adequacy() -> Statute {
    Statute::new(
        "BA_s10",
        "Capital Adequacy - Banking Act s. 10",
        Effect::new(
            EffectType::Obligation,
            "Bank must maintain minimum capital adequacy ratio (Basel III)",
        )
        .with_parameter("tier1_ratio", "6.0")
        .with_parameter("total_car", "10.0")
        .with_parameter("leverage_ratio", "3.0"),
    )
    .with_jurisdiction("SG")
}

/// BA s. 27 - AML compliance
fn ba_section_27_aml_compliance() -> Statute {
    Statute::new(
        "BA_s27",
        "AML Compliance - Banking Act s. 27",
        Effect::new(
            EffectType::Obligation,
            "Bank must implement AML/CFT compliance program",
        )
        .with_parameter("ctr_threshold_sgd", "20000"),
    )
    .with_jurisdiction("SG")
}

// ============================================================================
// Payment Services Act 2019 Statute Definitions
// ============================================================================

/// PSA s. 6 - License requirement
fn psa_section_6_license_requirement() -> Statute {
    Statute::new(
        "PSA_s6",
        "License Requirement - Payment Services Act s. 6",
        Effect::new(
            EffectType::Prohibition,
            "No person shall carry on payment service business without license",
        ),
    )
    .with_jurisdiction("SG")
}

/// PSA s. 23 - Safeguarding requirement
fn psa_section_23_safeguarding() -> Statute {
    Statute::new(
        "PSA_s23",
        "Safeguarding - Payment Services Act s. 23",
        Effect::new(
            EffectType::Obligation,
            "Licensee must safeguard customer moneys",
        )
        .with_parameter("safeguarding_ratio", "100"),
    )
    .with_jurisdiction("SG")
}

// ============================================================================
// PDPA Statute Definitions
// ============================================================================

/// PDPA s. 13 - Consent requirement
fn pdpa_section_13_consent() -> Statute {
    Statute::new(
        "PDPA_s13",
        "Consent - PDPA s. 13",
        Effect::new(
            EffectType::Prohibition,
            "Organisation shall not collect/use/disclose personal data without consent",
        ),
    )
    .with_jurisdiction("SG")
}

/// PDPA s. 26B/C - Breach notification
fn pdpa_section_26b_breach_notification() -> Statute {
    Statute::new(
        "PDPA_s26B",
        "Data Breach Notification - PDPA s. 26B/26C",
        Effect::new(
            EffectType::Obligation,
            "Organisation must notify PDPC of notifiable data breach within 3 days",
        )
        .with_parameter("notification_days", "3"),
    )
    .with_jurisdiction("SG")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_employment_act_statutes() {
        let statutes = employment_act_statutes();
        assert!(!statutes.is_empty());
        assert!(statutes.iter().any(|s| s.id == "EA_s38_1"));
        assert!(statutes.iter().any(|s| s.id == "EA_s43"));
    }

    #[test]
    fn test_companies_act_statutes() {
        let statutes = companies_act_statutes();
        assert!(!statutes.is_empty());
        assert!(statutes.iter().any(|s| s.id == "CA_s145"));
    }

    #[test]
    fn test_all_singapore_statutes() {
        let statutes = all_singapore_statutes();
        assert!(statutes.len() >= 10);
    }

    #[test]
    fn test_statute_jurisdiction() {
        let statute = ea_section_38_working_hours();
        assert_eq!(statute.jurisdiction.as_deref(), Some("SG"));
    }

    #[test]
    fn test_statute_effects() {
        let statute = ea_section_38_overtime();
        assert_eq!(statute.effect.effect_type, EffectType::Obligation);
        assert!(statute.effect.get_parameter("overtime_rate").is_some());
    }
}
