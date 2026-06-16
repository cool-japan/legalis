//! `legalis-core` [`Statute`] models for major Thai legislation, with a
//! `legalis-dsl` export.
//!
//! This module lifts the domain-specific Thai law areas implemented elsewhere in
//! this crate (civil & commercial code, company / securities law, labour
//! protection, foreign business, PDPA, trade competition, the Revenue Code) into
//! the jurisdiction-neutral [`Statute`] abstraction from
//! [`legalis_core`]. Each builder encodes the real statute's identifier, an
//! authoritative title (including the Buddhist Era year), the appropriate
//! [`EffectType`], and a meaningful precondition where the underlying rule is
//! naturally conditional (length of service, ownership percentage, turnover,
//! etc.).
//!
//! The resulting statutes can be rendered as `legalis-dsl` source text via
//! [`statutes_as_dsl`], enabling inspection, formatting, structural diffing, and
//! consumption by the DSL tooling (LSP, documentation generation).
//!
//! All jurisdictions are tagged `"TH"` (Kingdom of Thailand).
//!
//! # References
//!
//! - Civil and Commercial Code (ประมวลกฎหมายแพ่งและพาณิชย์), B.E. 2535 (1992 consolidation)
//! - Public Limited Companies Act (พ.ร.บ. บริษัทมหาชนจำกัด), B.E. 2535 (1992)
//! - Labour Protection Act (พ.ร.บ. คุ้มครองแรงงาน), B.E. 2541 (1998)
//! - Foreign Business Act (พ.ร.บ. การประกอบธุรกิจของคนต่างด้าว), B.E. 2542 (1999)
//! - Personal Data Protection Act / PDPA (พ.ร.บ. คุ้มครองข้อมูลส่วนบุคคล), B.E. 2562 (2019)
//! - Trade Competition Act (พ.ร.บ. การแข่งขันทางการค้า), B.E. 2560 (2017)
//! - Securities and Exchange Act (พ.ร.บ. หลักทรัพย์และตลาดหลักทรัพย์), B.E. 2535 (1992)
//! - Revenue Code (ประมวลรัษฎากร), VAT registration threshold

use legalis_core::{ComparisonOp, Condition, DurationUnit, Effect, EffectType, Statute};

/// Civil and Commercial Code (ประมวลกฎหมายแพ่งและพาณิชย์), B.E. 2535,
/// Book II Title V (Tort) §420: a person who wilfully or negligently injures the
/// life, body, health, liberty, property or any right of another is bound to make
/// compensation for the resulting wrongful act.
#[must_use]
pub fn ccc_tort_liability_statute() -> Statute {
    Statute::new(
        "TH-CCC-2535-s420",
        "Liability for Wrongful Acts (Civil and Commercial Code B.E. 2535, §420)",
        Effect::new(
            EffectType::Obligation,
            "A person who wilfully or negligently and unlawfully injures the life, body, \
             health, liberty, property or right of another must compensate the wrongful act",
        ),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "unlawful_injury".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("TH")
}

/// Public Limited Companies Act (พ.ร.บ. บริษัทมหาชนจำกัด), B.E. 2535:
/// a public limited company must maintain the statutory minimum registered
/// capital (THB 5,000,000) and may offer shares to the public, complementing the
/// private-company provisions of the Civil and Commercial Code.
#[must_use]
pub fn public_company_capital_statute() -> Statute {
    Statute::new(
        "TH-PLCA-2535-cap",
        "Public Limited Company Minimum Capital (Public Limited Companies Act B.E. 2535)",
        Effect::new(
            EffectType::Obligation,
            "A public limited company must maintain registered capital of at least \
             THB 5,000,000 before offering shares to the public",
        )
        .with_parameter("min_registered_capital_thb", "5000000"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "company_type".to_string(),
        value: "public_limited".to_string(),
    })
    .with_jurisdiction("TH")
}

/// Labour Protection Act (พ.ร.บ. คุ้มครองแรงงาน), B.E. 2541, §23 (normal working
/// time of no more than 8 hours/day and 48 hours/week) and §118 (statutory
/// severance pay of 30 to 400 days' wages, payable to an employee with at least
/// 120 days of continuous service who is terminated without cause).
#[must_use]
pub fn lpa_working_hours_severance_statute() -> Statute {
    Statute::new(
        "TH-LPA-2541-s118",
        "Working Hours and Severance Pay (Labour Protection Act B.E. 2541, §§23, 118)",
        Effect::new(
            EffectType::Grant,
            "Normal working time is capped at 8 hours/day and 48 hours/week; an employee \
             terminated without cause is entitled to severance pay of 30 to 400 days' wages \
             based on length of continuous service",
        )
        .with_parameter("max_hours_per_week", "48")
        .with_parameter("min_severance_days", "30")
        .with_parameter("max_severance_days", "400"),
    )
    .with_precondition(Condition::Duration {
        operator: ComparisonOp::GreaterOrEqual,
        value: 120,
        unit: DurationUnit::Days,
    })
    .with_jurisdiction("TH")
}

/// Foreign Business Act (พ.ร.บ. การประกอบธุรกิจของคนต่างด้าว), B.E. 2542:
/// businesses in List 3 (e.g. retail, construction, most service activities) are
/// restricted to foreigners and require a Foreign Business Licence unless the
/// enterprise is majority Thai-owned (Thai shareholding of 50% or more).
#[must_use]
pub fn fba_list3_license_statute() -> Statute {
    Statute::new(
        "TH-FBA-2542-list3",
        "Foreign Business Licence for List 3 Activities (Foreign Business Act B.E. 2542)",
        Effect::new(
            EffectType::Prohibition,
            "A foreign-majority enterprise must obtain a Foreign Business Licence before \
             engaging in a restricted List 3 activity",
        )
        .with_parameter("restriction_list", "3"),
    )
    .with_precondition(Condition::Percentage {
        operator: ComparisonOp::LessThan,
        value: 50,
        context: "thai_shareholding".to_string(),
    })
    .with_jurisdiction("TH")
}

/// Personal Data Protection Act / PDPA (พ.ร.บ. คุ้มครองข้อมูลส่วนบุคคล),
/// B.E. 2562 §37(4): upon becoming aware of a personal data breach, the data
/// controller must notify the Office of the PDPC without delay and, where
/// feasible, within 72 hours.
#[must_use]
pub fn pdpa_breach_notification_statute() -> Statute {
    Statute::new(
        "TH-PDPA-2562-s37",
        "Data Breach Notification (Personal Data Protection Act B.E. 2562, §37)",
        Effect::new(
            EffectType::Obligation,
            "A data controller must notify the Office of the PDPC of a personal data breach \
             without delay and, where feasible, within 72 hours of becoming aware of it",
        )
        .with_parameter("notification_deadline_hours", "72")
        .with_parameter("max_admin_fine_thb", "5000000"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "personal_data_breach".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("TH")
}

/// Trade Competition Act (พ.ร.บ. การแข่งขันทางการค้า), B.E. 2560 §50:
/// a business operator holding a dominant market position (market share of 50% or
/// more, subject to the OTCC thresholds) is prohibited from abusing that dominance
/// (e.g. unfair price-fixing or restricting supply).
#[must_use]
pub fn tca_abuse_of_dominance_statute() -> Statute {
    Statute::new(
        "TH-TCA-2560-s50",
        "Prohibition on Abuse of Dominance (Trade Competition Act B.E. 2560, §50)",
        Effect::new(
            EffectType::Prohibition,
            "A business operator with a dominant market position must not abuse that \
             dominance through unfair pricing or restriction of supply",
        )
        .with_parameter("dominance_market_share_pct", "50"),
    )
    .with_precondition(Condition::Percentage {
        operator: ComparisonOp::GreaterOrEqual,
        value: 50,
        context: "market_share".to_string(),
    })
    .with_jurisdiction("TH")
}

/// Securities and Exchange Act (พ.ร.บ. หลักทรัพย์และตลาดหลักทรัพย์), B.E. 2535
/// (insider dealing provisions, as strengthened by the B.E. 2559 amendments):
/// a person possessing material non-public information must not trade the relevant
/// securities or disclose that information to others for the purpose of trading.
#[must_use]
pub fn sea_insider_trading_statute() -> Statute {
    Statute::new(
        "TH-SEA-2535-insider",
        "Prohibition on Insider Trading (Securities and Exchange Act B.E. 2535)",
        Effect::new(
            EffectType::Prohibition,
            "A person who possesses material non-public information must not trade the \
             relevant securities or pass that information to others to trade",
        ),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "possesses_material_nonpublic_info".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("TH")
}

/// Revenue Code (ประมวลรัษฎากร) §85/1: a business whose annual turnover from the
/// sale of goods or provision of services reaches THB 1,800,000 must register for
/// Value Added Tax (standard rate 7%).
#[must_use]
pub fn revenue_code_vat_registration_statute() -> Statute {
    Statute::new(
        "TH-RC-vat-reg",
        "VAT Registration Threshold (Revenue Code §85/1)",
        Effect::new(
            EffectType::Obligation,
            "A business with annual turnover of THB 1,800,000 or more must register for \
             Value Added Tax (standard rate 7%)",
        )
        .with_parameter("vat_registration_threshold_thb", "1800000")
        .with_parameter("standard_vat_rate_pct", "7"),
    )
    .with_precondition(Condition::Income {
        operator: ComparisonOp::GreaterOrEqual,
        value: 1_800_000,
    })
    .with_jurisdiction("TH")
}

/// Returns every modelled Thai statute.
#[must_use]
pub fn all_statutes() -> Vec<Statute> {
    vec![
        ccc_tort_liability_statute(),
        public_company_capital_statute(),
        lpa_working_hours_severance_statute(),
        fba_list3_license_statute(),
        pdpa_breach_notification_statute(),
        tca_abuse_of_dominance_statute(),
        sea_insider_trading_statute(),
        revenue_code_vat_registration_statute(),
    ]
}

/// Renders every modelled Thai statute as `legalis-dsl` source text.
///
/// Each statute is emitted as a `STATUTE … { WHEN … THEN … }` block by
/// [`legalis_dsl::format_statutes`].
#[must_use]
pub fn statutes_as_dsl() -> String {
    legalis_dsl::format_statutes(&all_statutes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn statutes_render_as_valid_dsl() {
        let statutes = all_statutes();
        assert!(!statutes.is_empty(), "TH must model at least one statute");
        assert_eq!(statutes.len(), 8, "TH must model exactly 8 statutes");

        let dsl = statutes_as_dsl();
        assert!(!dsl.is_empty(), "DSL export must not be empty");

        // Every modelled statute's id must appear in the rendered DSL, proving
        // the printer handled each one (covers the full range of condition
        // kinds the TH statutes use: Duration, Income, Percentage,
        // AttributeEquals).
        for statute in &statutes {
            assert!(
                dsl.contains(statute.id.as_str()),
                "statute {} missing from DSL export",
                statute.id
            );
        }
    }
}
