//! Statute adapters for UK Employment Law.
//!
//! Converts UK employment law provisions into legalis-core Statute abstractions.

use legalis_core::{ComparisonOp, Condition, DurationUnit, Effect, EffectType, Statute};

/// Creates a Statute for ERA 1996 s.1 - Written Particulars requirement
#[must_use]
pub fn era_section_1_written_particulars() -> Statute {
    Statute::new(
        "ERA_s1",
        "Written Particulars of Employment (ERA 1996 s.1)",
        Effect::new(
            EffectType::Obligation,
            "Written particulars must be provided to employee within 2 months",
        ),
    )
    .with_precondition(Condition::Duration {
        operator: ComparisonOp::GreaterOrEqual,
        value: 2,
        unit: DurationUnit::Months,
    })
    .with_jurisdiction("GB")
}

/// Creates a Statute for ERA 1996 s.86 - Statutory Notice Periods
#[must_use]
pub fn era_section_86_notice_periods() -> Statute {
    Statute::new(
        "ERA_s86",
        "Statutory Minimum Notice Periods (ERA 1996 s.86)",
        Effect::new(
            EffectType::Grant,
            "Statutory minimum notice period applies based on length of service",
        ),
    )
    .with_precondition(Condition::Duration {
        operator: ComparisonOp::GreaterOrEqual,
        value: 1,
        unit: DurationUnit::Months,
    })
    .with_jurisdiction("GB")
}

/// Creates a Statute for ERA 1996 s.98 - Unfair Dismissal Protection
#[must_use]
pub fn era_section_98_unfair_dismissal() -> Statute {
    Statute::new(
        "ERA_s98",
        "Unfair Dismissal Protection (ERA 1996 s.98)",
        Effect::new(
            EffectType::Grant,
            "Protection from unfair dismissal after 2 years continuous service",
        ),
    )
    .with_precondition(Condition::Duration {
        operator: ComparisonOp::GreaterOrEqual,
        value: 24,
        unit: DurationUnit::Months,
    })
    .with_jurisdiction("GB")
}

/// Creates a Statute for ERA 1996 s.162 - Redundancy Payment
#[must_use]
pub fn era_section_162_redundancy() -> Statute {
    Statute::new(
        "ERA_s162",
        "Statutory Redundancy Payment (ERA 1996 s.162)",
        Effect::new(
            EffectType::Grant,
            "Entitlement to statutory redundancy payment based on age and service",
        ),
    )
    .with_precondition(Condition::Duration {
        operator: ComparisonOp::GreaterOrEqual,
        value: 24,
        unit: DurationUnit::Months,
    })
    .with_jurisdiction("GB")
}

/// Creates a Statute for WTR 1998 Reg 4 - 48-hour working week limit
#[must_use]
pub fn wtr_regulation_4_working_time() -> Statute {
    Statute::new(
        "WTR_Reg4",
        "Maximum Working Week (WTR 1998 Reg 4)",
        Effect::new(
            EffectType::Prohibition,
            "Working hours must not exceed 48 hours per week unless opted out",
        )
        .with_parameter("max_hours", "48"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "opted_out_48h".to_string(),
        value: "false".to_string(),
    })
    .with_jurisdiction("GB")
}

/// Creates a Statute for WTR 1998 Reg 12 - Rest Breaks
#[must_use]
pub fn wtr_regulation_12_rest_breaks() -> Statute {
    Statute::new(
        "WTR_Reg12",
        "Rest Breaks (WTR 1998 Reg 12)",
        Effect::new(
            EffectType::Grant,
            "Entitled to 20-minute uninterrupted rest break when working 6+ hours",
        )
        .with_parameter("break_minutes", "20"),
    )
    .with_jurisdiction("GB")
}

/// Creates a Statute for WTR 1998 Reg 13 - Annual Leave
#[must_use]
pub fn wtr_regulation_13_annual_leave() -> Statute {
    Statute::new(
        "WTR_Reg13",
        "Statutory Annual Leave (WTR 1998 Reg 13)",
        Effect::new(EffectType::Grant, "5.6 weeks paid annual leave per year")
            .with_parameter("weeks", "5.6"),
    )
    .with_jurisdiction("GB")
}

/// Creates a Statute for NMWA 1998 - National Minimum Wage
#[must_use]
pub fn nmwa_minimum_wage() -> Statute {
    Statute::new(
        "NMWA_1998",
        "National Minimum Wage (NMWA 1998)",
        Effect::new(
            EffectType::Grant,
            "Entitlement to at least the applicable minimum wage rate",
        )
        .with_parameter("nlw_rate_gbp", "11.44"),
    )
    .with_jurisdiction("GB")
}

/// Creates a Statute for Pension Auto-Enrolment
#[must_use]
pub fn pension_auto_enrolment() -> Statute {
    Statute::new(
        "PENSION_AE",
        "Workplace Pension Auto-Enrolment",
        Effect::new(
            EffectType::Obligation,
            "Employer must auto-enrol eligible workers into pension scheme",
        )
        .with_parameter("min_employer_contribution_pct", "3"),
    )
    .with_precondition(Condition::Income {
        operator: ComparisonOp::GreaterOrEqual,
        value: 1_000_000, // £10,000 in pence
    })
    .with_jurisdiction("GB")
}

/// Get all UK employment law statutes
#[must_use]
pub fn all_employment_statutes() -> Vec<Statute> {
    vec![
        era_section_1_written_particulars(),
        era_section_86_notice_periods(),
        era_section_98_unfair_dismissal(),
        era_section_162_redundancy(),
        wtr_regulation_4_working_time(),
        wtr_regulation_12_rest_breaks(),
        wtr_regulation_13_annual_leave(),
        nmwa_minimum_wage(),
        pension_auto_enrolment(),
    ]
}
