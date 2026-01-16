//! Statute adapters for German Labor Law (Arbeitsrecht Gesetzesadapter).
//!
//! Converts German labor law provisions into legalis-core Statute abstractions.
//! Konvertiert deutsches Arbeitsrecht in legalis-core Statute-Abstraktionen.

use legalis_core::{ComparisonOp, Condition, DurationUnit, Effect, EffectType, Statute};

// ============================================================================
// Arbeitszeitgesetz (ArbZG) - Working Hours Act
// ============================================================================

/// Creates a Statute for §3 ArbZG - Daily Working Hours
/// §3 ArbZG - Tägliche Arbeitszeit
#[must_use]
pub fn arbzg_section_3_daily_hours() -> Statute {
    Statute::new(
        "ArbZG_§3",
        "Arbeitszeit / Daily Working Hours (§3 ArbZG)",
        Effect::new(
            EffectType::Prohibition,
            "Max. 8 Stunden täglich, verlängerbar auf 10 Stunden / Max 8h/day, extendable to 10h",
        )
        .with_parameter("max_hours_regular", "8")
        .with_parameter("max_hours_extended", "10"),
    )
    .with_jurisdiction("DE")
}

/// Creates a Statute for §4 ArbZG - Rest Breaks
/// §4 ArbZG - Ruhepausen
#[must_use]
pub fn arbzg_section_4_rest_breaks() -> Statute {
    Statute::new(
        "ArbZG_§4",
        "Ruhepausen / Rest Breaks (§4 ArbZG)",
        Effect::new(
            EffectType::Grant,
            "30 Min bei 6-9h, 45 Min bei >9h Arbeitszeit / 30min for 6-9h, 45min for >9h work",
        )
        .with_parameter("break_6_9h_minutes", "30")
        .with_parameter("break_9h_plus_minutes", "45"),
    )
    .with_jurisdiction("DE")
}

/// Creates a Statute for §5 ArbZG - Daily Rest Period
/// §5 ArbZG - Ruhezeit
#[must_use]
pub fn arbzg_section_5_daily_rest() -> Statute {
    Statute::new(
        "ArbZG_§5",
        "Ruhezeit / Daily Rest Period (§5 ArbZG)",
        Effect::new(
            EffectType::Grant,
            "Min. 11 Stunden ununterbrochene Ruhezeit / Min 11 hours uninterrupted rest",
        )
        .with_parameter("min_rest_hours", "11"),
    )
    .with_jurisdiction("DE")
}

// ============================================================================
// Kündigungsschutzgesetz (KSchG) - Dismissal Protection Act
// ============================================================================

/// Creates a Statute for §1 KSchG - Dismissal Protection
/// §1 KSchG - Kündigungsschutz
#[must_use]
pub fn kschg_section_1_dismissal_protection() -> Statute {
    Statute::new(
        "KSchG_§1",
        "Kündigungsschutz / Dismissal Protection (§1 KSchG)",
        Effect::new(
            EffectType::Grant,
            "Kündigungsschutz ab 6 Monaten in Betrieben mit 10+ AN / Protection after 6 months in 10+ employee firms",
        ),
    )
    .with_precondition(Condition::Duration {
        operator: ComparisonOp::GreaterOrEqual,
        value: 6,
        unit: DurationUnit::Months,
    })
    .with_precondition(Condition::AttributeEquals {
        key: "company_size".to_string(),
        value: "medium".to_string(),
    })
    .with_jurisdiction("DE")
}

// ============================================================================
// BGB §622 - Notice Periods
// ============================================================================

/// Creates a Statute for §622 BGB - Statutory Notice Periods
/// §622 BGB - Gesetzliche Kündigungsfristen
#[must_use]
pub fn bgb_section_622_notice_periods() -> Statute {
    Statute::new(
        "BGB_§622",
        "Kündigungsfristen / Notice Periods (§622 BGB)",
        Effect::new(
            EffectType::Obligation,
            "Staffelung nach Betriebszugehörigkeit / Graduated by length of service",
        )
        .with_parameter("basic_notice_weeks", "4"),
    )
    .with_jurisdiction("DE")
}

/// Creates a Statute for §623 BGB - Written Form Requirement
/// §623 BGB - Schriftformerfordernis
#[must_use]
pub fn bgb_section_623_written_form() -> Statute {
    Statute::new(
        "BGB_§623",
        "Schriftformerfordernis / Written Form Requirement (§623 BGB)",
        Effect::new(
            EffectType::StatusChange,
            "Kündigung bedarf der Schriftform / Dismissal requires written form",
        ),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "written".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("DE")
}

// ============================================================================
// Bundesurlaubsgesetz (BUrlG) - Federal Leave Act
// ============================================================================

/// Creates a Statute for §3 BUrlG - Minimum Leave
/// §3 BUrlG - Mindesturlaub
#[must_use]
pub fn burlg_section_3_minimum_leave() -> Statute {
    Statute::new(
        "BUrlG_§3",
        "Mindesturlaub / Minimum Leave (§3 BUrlG)",
        Effect::new(
            EffectType::Grant,
            "24 Werktage bei 6-Tage-Woche (4 Wochen) / 24 working days for 6-day week (4 weeks)",
        )
        .with_parameter("min_days_6_day_week", "24")
        .with_parameter("weeks", "4"),
    )
    .with_jurisdiction("DE")
}

// ============================================================================
// Entgeltfortzahlungsgesetz (EFZG) - Continued Remuneration Act
// ============================================================================

/// Creates a Statute for §3 EFZG - Sick Pay
/// §3 EFZG - Entgeltfortzahlung im Krankheitsfall
#[must_use]
pub fn efzg_section_3_sick_pay() -> Statute {
    Statute::new(
        "EFZG_§3",
        "Entgeltfortzahlung / Sick Pay (§3 EFZG)",
        Effect::new(
            EffectType::Grant,
            "6 Wochen Entgeltfortzahlung bei Krankheit / 6 weeks continued pay during illness",
        )
        .with_parameter("weeks", "6")
        .with_parameter("days", "42"),
    )
    .with_precondition(Condition::Duration {
        operator: ComparisonOp::LessOrEqual,
        value: 6,
        unit: DurationUnit::Weeks, // First 6 weeks
    })
    .with_jurisdiction("DE")
}

// ============================================================================
// Teilzeit- und Befristungsgesetz (TzBfG)
// ============================================================================

/// Creates a Statute for §14 TzBfG - Fixed-term Contracts
/// §14 TzBfG - Befristete Arbeitsverträge
#[must_use]
pub fn tzbfg_section_14_fixed_term() -> Statute {
    Statute::new(
        "TzBfG_§14",
        "Befristung / Fixed-term Contracts (§14 TzBfG)",
        Effect::new(
            EffectType::Prohibition,
            "Sachgrundlose Befristung max. 2 Jahre mit max. 3 Verlängerungen / Max 2 years without reason, 3 extensions",
        )
        .with_parameter("max_months", "24")
        .with_parameter("max_extensions", "3"),
    )
    .with_jurisdiction("DE")
}

// ============================================================================
// Betriebsverfassungsgesetz (BetrVG) - Works Constitution Act
// ============================================================================

/// Creates a Statute for §102 BetrVG - Works Council Consultation
/// §102 BetrVG - Anhörung des Betriebsrats
#[must_use]
pub fn betrvg_section_102_works_council() -> Statute {
    Statute::new(
        "BetrVG_§102",
        "Betriebsratsanhörung / Works Council Consultation (§102 BetrVG)",
        Effect::new(
            EffectType::Obligation,
            "Anhörung des Betriebsrats vor jeder Kündigung / Consultation before any dismissal",
        ),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "has_works_council".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("DE")
}

// ============================================================================
// Mindestlohngesetz (MiLoG) - Minimum Wage Act
// ============================================================================

/// Creates a Statute for Minimum Wage
/// Mindestlohn
#[must_use]
pub fn milog_minimum_wage() -> Statute {
    Statute::new(
        "MiLoG",
        "Mindestlohn / Minimum Wage (MiLoG)",
        Effect::new(
            EffectType::Grant,
            "Gesetzlicher Mindestlohn / Statutory minimum wage",
        )
        .with_parameter("rate_eur", "12.41"),
    )
    .with_jurisdiction("DE")
}

/// Get all German labor law statutes
/// Alle deutschen Arbeitsrechtsgesetze
#[must_use]
pub fn all_labor_statutes() -> Vec<Statute> {
    vec![
        arbzg_section_3_daily_hours(),
        arbzg_section_4_rest_breaks(),
        arbzg_section_5_daily_rest(),
        kschg_section_1_dismissal_protection(),
        bgb_section_622_notice_periods(),
        bgb_section_623_written_form(),
        burlg_section_3_minimum_leave(),
        efzg_section_3_sick_pay(),
        tzbfg_section_14_fixed_term(),
        betrvg_section_102_works_council(),
        milog_minimum_wage(),
    ]
}
