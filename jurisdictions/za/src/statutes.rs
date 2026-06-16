//! `Statute`-based models of major South African legislation.
//!
//! This module lifts the validators and domain types implemented across the
//! `legalis-za` crate (companies, competition, constitution, data protection,
//! labour, environmental, insolvency and tax law) into the jurisdiction-neutral
//! [`legalis_core::Statute`] abstraction. Each builder encodes a *real* statutory
//! provision — accurate Act number, year and operative rule — as an [`Effect`]
//! with a meaningful [`Condition`] precondition where the underlying law turns on
//! a quantifiable trigger (a duration, a monetary threshold expressed as an
//! attribute, or a status flag).
//!
//! The modelled statutes can be rendered as `legalis-dsl` source text via
//! [`statutes_as_dsl`], so the South African rule-set can be inspected, diffed,
//! formatted and consumed by the DSL tooling (LSP, documentation generation,
//! structural diffing) on the same footing as every other jurisdiction.
//!
//! # Coverage
//!
//! | Builder | Act |
//! |---------|-----|
//! | [`companies_act_statute`] | Companies Act 71 of 2008 |
//! | [`competition_act_statute`] | Competition Act 89 of 1998 |
//! | [`constitution_equality_statute`] | Constitution of 1996, s.9 (Bill of Rights) |
//! | [`popia_statute`] | Protection of Personal Information Act 4 of 2013 |
//! | [`bcea_annual_leave_statute`] | Basic Conditions of Employment Act 75 of 1997, s.20 |
//! | [`nema_duty_of_care_statute`] | National Environmental Management Act 107 of 1998, s.28 |
//! | [`insolvency_act_statute`] | Insolvency Act 24 of 1936 |
//! | [`vat_act_statute`] | Value-Added Tax Act 89 of 1991, s.23 |
//!
//! # Disclaimer
//!
//! These models are simplified abstractions for computational reasoning and are
//! provided for educational and informational purposes only. They are not legal
//! advice; consult a qualified South African attorney or advocate.

use legalis_core::{ComparisonOp, Condition, DurationUnit, Effect, EffectType, Statute};

/// Companies Act 71 of 2008, s.50 — securities register / s.30 annual financial
/// statements obligation for companies.
///
/// Every company must keep a securities register and, under s.30, prepare annual
/// financial statements within six months after the end of its financial year.
/// Public and state-owned companies must additionally have those statements
/// audited. Modelled here as the s.30 obligation to finalise annual financial
/// statements within the six-month statutory window.
///
/// Real source: Companies Act 71 of 2008, s.30(1).
#[must_use]
pub fn companies_act_statute() -> Statute {
    Statute::new(
        "ZA-COMPANIES-2008",
        "Annual Financial Statements (Companies Act 71 of 2008, s.30)",
        Effect::new(
            EffectType::Obligation,
            "A company must prepare annual financial statements within 6 months after \
             the end of its financial year",
        )
        .with_parameter("act_number", "71")
        .with_parameter("act_year", "2008")
        .with_parameter("section", "30"),
    )
    .with_precondition(Condition::Duration {
        operator: ComparisonOp::LessOrEqual,
        value: 6,
        unit: DurationUnit::Months,
    })
    .with_jurisdiction("ZA")
}

/// Competition Act 89 of 1998, s.4 — prohibition on horizontal restrictive
/// practices between competitors.
///
/// Section 4(1)(b) renders per se unlawful any agreement between competitors that
/// involves price fixing, dividing markets, or collusive tendering. This is one
/// of the cornerstone prohibitions enforced by the Competition Commission and
/// Tribunal.
///
/// Real source: Competition Act 89 of 1998, s.4(1)(b).
#[must_use]
pub fn competition_act_statute() -> Statute {
    Statute::new(
        "ZA-COMPETITION-1998",
        "Prohibited Horizontal Practices (Competition Act 89 of 1998, s.4)",
        Effect::new(
            EffectType::Prohibition,
            "Agreements between competitors involving price fixing, market division \
             or collusive tendering are per se prohibited",
        )
        .with_parameter("act_number", "89")
        .with_parameter("act_year", "1998")
        .with_parameter("section", "4(1)(b)"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "parties_are_competitors".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("ZA")
}

/// Constitution of the Republic of South Africa, 1996, s.9 — Equality (Bill of
/// Rights).
///
/// Section 9 of the Bill of Rights guarantees equality before the law and
/// prohibits unfair discrimination, directly or indirectly, on grounds including
/// race, gender, sex, pregnancy, marital status, ethnic or social origin, colour,
/// sexual orientation, age, disability, religion, conscience, belief, culture,
/// language and birth. Modelled as a Grant of the equality right.
///
/// Real source: Constitution of the Republic of South Africa, 1996, s.9(1)–(3).
#[must_use]
pub fn constitution_equality_statute() -> Statute {
    Statute::new(
        "ZA-CONSTITUTION-1996-S9",
        "Right to Equality (Constitution of 1996, s.9)",
        Effect::new(
            EffectType::Grant,
            "Everyone is equal before the law and has the right to equal protection \
             and benefit of the law, free from unfair discrimination on listed grounds",
        )
        .with_parameter("instrument", "Constitution 1996")
        .with_parameter("section", "9"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "is_person".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("ZA")
}

/// Protection of Personal Information Act 4 of 2013 (POPIA), s.22 — notification
/// of a security compromise.
///
/// Where there are reasonable grounds to believe that the personal information of
/// a data subject has been accessed or acquired by an unauthorised person, the
/// responsible party must notify the Information Regulator and the affected data
/// subject as soon as reasonably possible after discovery of the compromise.
///
/// Real source: Protection of Personal Information Act 4 of 2013, s.22.
#[must_use]
pub fn popia_statute() -> Statute {
    Statute::new(
        "ZA-POPIA-2013",
        "Security Compromise Notification (POPIA Act 4 of 2013, s.22)",
        Effect::new(
            EffectType::Obligation,
            "A responsible party must notify the Information Regulator and affected \
             data subjects as soon as reasonably possible after a security compromise",
        )
        .with_parameter("act_number", "4")
        .with_parameter("act_year", "2013")
        .with_parameter("section", "22"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "personal_information_compromised".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("ZA")
}

/// Basic Conditions of Employment Act 75 of 1997, s.20 — annual leave
/// entitlement.
///
/// Section 20 entitles an employee to at least 21 consecutive days' annual leave
/// on full remuneration in respect of each annual leave cycle (equivalent to one
/// day for every 17 days worked, or one hour for every 17 hours worked). The
/// entitlement accrues over the 12-month leave cycle.
///
/// Real source: Basic Conditions of Employment Act 75 of 1997, s.20(2).
#[must_use]
pub fn bcea_annual_leave_statute() -> Statute {
    Statute::new(
        "ZA-BCEA-1997-LEAVE",
        "Annual Leave Entitlement (BCEA 75 of 1997, s.20)",
        Effect::new(
            EffectType::Grant,
            "An employee is entitled to at least 21 consecutive days of annual leave \
             on full pay per annual leave cycle",
        )
        .with_parameter("act_number", "75")
        .with_parameter("act_year", "1997")
        .with_parameter("section", "20")
        .with_parameter("leave_days", "21"),
    )
    .with_precondition(Condition::Duration {
        operator: ComparisonOp::GreaterOrEqual,
        value: 12,
        unit: DurationUnit::Months,
    })
    .with_jurisdiction("ZA")
}

/// National Environmental Management Act 107 of 1998 (NEMA), s.28 — general duty
/// of care.
///
/// Section 28(1) imposes a duty on every person who causes, has caused, or may
/// cause significant pollution or degradation of the environment to take
/// reasonable measures to prevent it from occurring, continuing or recurring, or,
/// where it cannot reasonably be avoided or stopped, to minimise and rectify it.
///
/// Real source: National Environmental Management Act 107 of 1998, s.28(1).
#[must_use]
pub fn nema_duty_of_care_statute() -> Statute {
    Statute::new(
        "ZA-NEMA-1998-S28",
        "Duty of Care for the Environment (NEMA 107 of 1998, s.28)",
        Effect::new(
            EffectType::Obligation,
            "A person who causes or may cause significant environmental pollution or \
             degradation must take reasonable measures to prevent, minimise and rectify it",
        )
        .with_parameter("act_number", "107")
        .with_parameter("act_year", "1998")
        .with_parameter("section", "28"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "causes_significant_environmental_harm".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("ZA")
}

/// Insolvency Act 24 of 1936, s.10 — provisional sequestration of a debtor's
/// estate.
///
/// On an application for the sequestration of a debtor's estate, the court may
/// grant a provisional order where there is reason to believe that the debtor is
/// insolvent (liabilities exceed assets) and that sequestration will be to the
/// advantage of creditors. Modelled as a status change effecting provisional
/// sequestration once the insolvency precondition is met.
///
/// Real source: Insolvency Act 24 of 1936, s.10.
#[must_use]
pub fn insolvency_act_statute() -> Statute {
    Statute::new(
        "ZA-INSOLVENCY-1936",
        "Provisional Sequestration (Insolvency Act 24 of 1936, s.10)",
        Effect::new(
            EffectType::StatusChange,
            "A court may place a debtor's estate under provisional sequestration where \
             the debtor is insolvent and sequestration is to the advantage of creditors",
        )
        .with_parameter("act_number", "24")
        .with_parameter("act_year", "1936")
        .with_parameter("section", "10"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "liabilities_exceed_assets".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("ZA")
}

/// Value-Added Tax Act 89 of 1991, s.23 — compulsory registration as a VAT
/// vendor.
///
/// A person carrying on an enterprise is liable to register for VAT where the
/// total value of taxable supplies made in any consecutive 12-month period has
/// exceeded, or is likely to exceed, the compulsory registration threshold of
/// R1 000 000. VAT is levied at the standard rate of 15%.
///
/// Real source: Value-Added Tax Act 89 of 1991, s.23(1).
#[must_use]
pub fn vat_act_statute() -> Statute {
    Statute::new(
        "ZA-VAT-1991",
        "Compulsory VAT Registration (VAT Act 89 of 1991, s.23)",
        Effect::new(
            EffectType::Obligation,
            "A person whose taxable supplies exceed R1 000 000 in any 12-month period \
             must register as a VAT vendor and levy VAT at the standard rate of 15%",
        )
        .with_parameter("act_number", "89")
        .with_parameter("act_year", "1991")
        .with_parameter("section", "23")
        .with_parameter("threshold_zar", "1000000")
        .with_parameter("standard_rate_pct", "15"),
    )
    .with_precondition(Condition::Income {
        operator: ComparisonOp::GreaterThan,
        value: 1_000_000,
    })
    .with_jurisdiction("ZA")
}

/// Returns every modelled South African statute.
#[must_use]
pub fn all_statutes() -> Vec<Statute> {
    vec![
        companies_act_statute(),
        competition_act_statute(),
        constitution_equality_statute(),
        popia_statute(),
        bcea_annual_leave_statute(),
        nema_duty_of_care_statute(),
        insolvency_act_statute(),
        vat_act_statute(),
    ]
}

/// Renders every modelled South African statute as `legalis-dsl` source text.
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
        assert!(!statutes.is_empty(), "ZA must model at least one statute");
        assert_eq!(statutes.len(), 8, "ZA must model exactly 8 statutes");

        let dsl = statutes_as_dsl();
        assert!(!dsl.is_empty(), "DSL export must not be empty");

        // Every modelled statute's id must appear in the rendered DSL, proving the
        // printer handled each one (covers the range of condition kinds the ZA
        // adapters use: Duration, Income, AttributeEquals).
        for statute in &statutes {
            assert!(
                dsl.contains(statute.id.as_str()),
                "statute {} missing from DSL export",
                statute.id
            );
        }
    }
}
