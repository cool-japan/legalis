//! `Statute`-based models of major Malaysian legislation.
//!
//! This module lifts the validators and domain types implemented across the
//! `legalis-my` crate (companies, contracts, employment, data protection,
//! competition, securities, tax and Islamic financial services law) into the
//! jurisdiction-neutral [`legalis_core::Statute`] abstraction. Each builder
//! encodes a *real* statutory provision — accurate Act name, year, section and
//! operative rule — as an [`Effect`] with a meaningful [`Condition`]
//! precondition where the underlying law turns on a quantifiable trigger (a
//! duration, a monetary threshold expressed as income, or a status flag).
//!
//! Malaysia operates a dual legal system: a common-law civil stream (applicable
//! to all persons) and a parallel Syariah stream (applicable to Muslims in
//! matters of Islamic law and Islamic finance). The Islamic Financial Services
//! Act 2013 model below represents the Syariah-governed limb of this system.
//!
//! The modelled statutes can be rendered as `legalis-dsl` source text via
//! [`statutes_as_dsl`], so the Malaysian rule-set can be inspected, diffed,
//! formatted and consumed by the DSL tooling (LSP, documentation generation,
//! structural diffing) on the same footing as every other jurisdiction.
//!
//! # Coverage
//!
//! | Builder | Act |
//! |---------|-----|
//! | [`companies_act_statute`] | Companies Act 2016, s.248 |
//! | [`contracts_act_statute`] | Contracts Act 1950, s.11 |
//! | [`employment_act_statute`] | Employment Act 1955, s.60A |
//! | [`pdpa_statute`] | Personal Data Protection Act 2010, s.6 |
//! | [`competition_act_statute`] | Competition Act 2010, s.4 |
//! | [`cmsa_statute`] | Capital Markets and Services Act 2007, s.58 |
//! | [`income_tax_act_statute`] | Income Tax Act 1967, s.77 |
//! | [`ifsa_statute`] | Islamic Financial Services Act 2013, s.8 |
//!
//! # Disclaimer
//!
//! These models are simplified abstractions for computational reasoning and are
//! provided for educational and informational purposes only. They are not legal
//! advice; consult a qualified Malaysian advocate and solicitor (peguam) or,
//! for Syariah matters, a qualified Syariah practitioner (peguam syarie).

use legalis_core::{ComparisonOp, Condition, DurationUnit, Effect, EffectType, Statute};

/// Companies Act 2016, s.248 — duty to circulate audited financial statements
/// and reports.
///
/// Section 248(1) requires the directors of every company to prepare financial
/// statements within 18 months from the date of incorporation and, thereafter,
/// within 6 months of the company's financial year end. For a public company
/// those audited financial statements and reports must be sent to members within
/// the statutory window before the annual general meeting. Modelled here as the
/// recurring s.248(1)(b) obligation to finalise financial statements within the
/// six-month statutory window after the financial year end, enforced by the
/// Companies Commission of Malaysia (SSM).
///
/// Real source: Companies Act 2016 (Act 777), s.248(1).
#[must_use]
pub fn companies_act_statute() -> Statute {
    Statute::new(
        "MY-COMPANIES-2016",
        "Audited Financial Statements (Companies Act 2016, s.248)",
        Effect::new(
            EffectType::Obligation,
            "The directors of a company must prepare financial statements within 6 months \
             after the end of the company's financial year",
        )
        .with_parameter("act_number", "777")
        .with_parameter("act_year", "2016")
        .with_parameter("section", "248")
        .with_parameter("regulator", "SSM"),
    )
    .with_precondition(Condition::Duration {
        operator: ComparisonOp::LessOrEqual,
        value: 6,
        unit: DurationUnit::Months,
    })
    .with_jurisdiction("MY")
}

/// Contracts Act 1950, s.11 — competence (capacity) to contract.
///
/// Section 11 provides that every person is competent to contract who is of the
/// age of majority according to the law to which he is subject, who is of sound
/// mind, and who is not disqualified from contracting by any law to which he is
/// subject. Read with the Age of Majority Act 1971, the age of majority in
/// Malaysia is 18 years. Modelled as a Grant of contractual capacity conditioned
/// on attaining the age of majority.
///
/// Real source: Contracts Act 1950 (Act 136), s.11; Age of Majority Act 1971.
#[must_use]
pub fn contracts_act_statute() -> Statute {
    Statute::new(
        "MY-CONTRACTS-1950",
        "Capacity to Contract (Contracts Act 1950, s.11)",
        Effect::new(
            EffectType::Grant,
            "A person of the age of majority (18 years), of sound mind and not otherwise \
             disqualified is competent to enter into a binding contract",
        )
        .with_parameter("act_number", "136")
        .with_parameter("act_year", "1950")
        .with_parameter("section", "11")
        .with_parameter("age_of_majority", "18"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 18,
    })
    .with_jurisdiction("MY")
}

/// Employment Act 1955, s.60A — limitation on hours of work.
///
/// Section 60A(1) prohibits an employee from being required under the terms of a
/// contract of service to work more than 8 hours in one day or more than 45 hours
/// in one week (with limited exceptions). Following the Employment (Amendment)
/// Act 2022 the statutory weekly ceiling was reduced from 48 to 45 hours.
/// Modelled as a Prohibition on contractually requiring weekly hours in excess of
/// the 45-hour ceiling unless a permitted exception applies.
///
/// Real source: Employment Act 1955 (Act 265), s.60A(1), as amended by Act A1651.
#[must_use]
pub fn employment_act_statute() -> Statute {
    Statute::new(
        "MY-EA-1955",
        "Limitation on Hours of Work (Employment Act 1955, s.60A)",
        Effect::new(
            EffectType::Prohibition,
            "An employee must not be required to work more than 8 hours a day or 45 hours \
             a week unless a statutory exception applies",
        )
        .with_parameter("act_number", "265")
        .with_parameter("act_year", "1955")
        .with_parameter("section", "60A")
        .with_parameter("max_daily_hours", "8")
        .with_parameter("max_weekly_hours", "45"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "hours_exception_applies".to_string(),
        value: "false".to_string(),
    })
    .with_jurisdiction("MY")
}

/// Personal Data Protection Act 2010, s.6 — the General Principle (consent).
///
/// Section 6(1) prohibits a data user from processing personal data about a data
/// subject unless the data subject has given his consent to the processing. This
/// is the first of the seven Personal Data Protection Principles and is enforced
/// by the Personal Data Protection Commissioner (Jabatan Perlindungan Data
/// Peribadi). Modelled as an Obligation triggered once personal data is being
/// processed, requiring that consent has been obtained.
///
/// Real source: Personal Data Protection Act 2010 (Act 709), s.6(1).
#[must_use]
pub fn pdpa_statute() -> Statute {
    Statute::new(
        "MY-PDPA-2010",
        "General Principle — Consent to Process (PDPA 2010, s.6)",
        Effect::new(
            EffectType::Obligation,
            "A data user must not process a data subject's personal data unless the data \
             subject has given consent to the processing",
        )
        .with_parameter("act_number", "709")
        .with_parameter("act_year", "2010")
        .with_parameter("section", "6")
        .with_parameter("regulator", "JPDP"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "processing_personal_data".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("MY")
}

/// Competition Act 2010, s.4 — prohibition on anti-competitive agreements.
///
/// Section 4(1) prohibits horizontal or vertical agreements between enterprises
/// that have the object or effect of significantly preventing, restricting or
/// distorting competition in any market for goods or services. Section 4(2)
/// deems certain horizontal agreements — price fixing, market or supply sharing,
/// output limitation and bid rigging — to have that object. The prohibition is
/// enforced by the Malaysia Competition Commission (MyCC).
///
/// Real source: Competition Act 2010 (Act 712), s.4(1)–(2).
#[must_use]
pub fn competition_act_statute() -> Statute {
    Statute::new(
        "MY-COMPETITION-2010",
        "Anti-Competitive Agreements (Competition Act 2010, s.4)",
        Effect::new(
            EffectType::Prohibition,
            "Agreements between enterprises that fix prices, share markets, limit output or \
             rig bids are prohibited as anti-competitive",
        )
        .with_parameter("act_number", "712")
        .with_parameter("act_year", "2010")
        .with_parameter("section", "4")
        .with_parameter("regulator", "MyCC"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "parties_are_competitors".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("MY")
}

/// Capital Markets and Services Act 2007, s.58 — requirement to hold a Capital
/// Markets Services Licence.
///
/// Section 58(1) prohibits any person from carrying on a business in, or holding
/// himself out as carrying on a business in, any regulated activity (such as
/// dealing in securities, fund management or investment advice) unless he holds a
/// Capital Markets Services Licence (CMSL). Licensing is administered by the
/// Securities Commission Malaysia (SC). Modelled as an Obligation to hold a CMSL
/// once a person carries on a regulated activity.
///
/// Real source: Capital Markets and Services Act 2007 (Act 671), s.58(1).
#[must_use]
pub fn cmsa_statute() -> Statute {
    Statute::new(
        "MY-CMSA-2007",
        "Capital Markets Services Licence (CMSA 2007, s.58)",
        Effect::new(
            EffectType::Obligation,
            "A person carrying on a business in a regulated activity must hold a Capital \
             Markets Services Licence issued by the Securities Commission",
        )
        .with_parameter("act_number", "671")
        .with_parameter("act_year", "2007")
        .with_parameter("section", "58")
        .with_parameter("regulator", "SC"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "carries_on_regulated_activity".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("MY")
}

/// Income Tax Act 1967, s.77 — duty to furnish a return of income.
///
/// Section 77 requires every person (other than a company, trust body or
/// co-operative society, which file under s.77A) chargeable to tax for a year of
/// assessment to furnish a return of income to the Director General of Inland
/// Revenue (Lembaga Hasil Dalam Negeri, LHDN) in the prescribed form. For an
/// individual with business income the return must be furnished by 30 June in the
/// following year of assessment. Modelled as an Obligation to file the annual
/// return, triggered once chargeable income arises.
///
/// Real source: Income Tax Act 1967 (Act 53), s.77(1).
#[must_use]
pub fn income_tax_act_statute() -> Statute {
    Statute::new(
        "MY-ITA-1967",
        "Return of Income (Income Tax Act 1967, s.77)",
        Effect::new(
            EffectType::Obligation,
            "A chargeable person must furnish a return of income to the Director General of \
             Inland Revenue (LHDN) for each year of assessment",
        )
        .with_parameter("act_number", "53")
        .with_parameter("act_year", "1967")
        .with_parameter("section", "77")
        .with_parameter("regulator", "LHDN"),
    )
    .with_precondition(Condition::Income {
        operator: ComparisonOp::GreaterThan,
        value: 0,
    })
    .with_jurisdiction("MY")
}

/// Islamic Financial Services Act 2013, s.8 — requirement to be licensed to carry
/// on Islamic banking business.
///
/// Section 8(1) prohibits any person from carrying on Islamic banking business,
/// takaful business, international Islamic banking business or international
/// takaful business unless licensed by the Minister on the recommendation of Bank
/// Negara Malaysia. The Act establishes a Syariah-compliance framework under the
/// oversight of the Shariah Advisory Council. Modelled as an Obligation,
/// reflecting the Syariah-governed limb of Malaysia's dual financial system, to
/// be licensed before carrying on Islamic banking business.
///
/// Real source: Islamic Financial Services Act 2013 (Act 759), s.8(1).
#[must_use]
pub fn ifsa_statute() -> Statute {
    Statute::new(
        "MY-IFSA-2013",
        "Licensing of Islamic Banking Business (IFSA 2013, s.8)",
        Effect::new(
            EffectType::Obligation,
            "A person must hold a licence granted on the recommendation of Bank Negara \
             Malaysia before carrying on Islamic banking or takaful business",
        )
        .with_parameter("act_number", "759")
        .with_parameter("act_year", "2013")
        .with_parameter("section", "8")
        .with_parameter("regulator", "BNM")
        .with_parameter("legal_stream", "syariah"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "carries_on_islamic_banking_business".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("MY")
}

/// Returns every modelled Malaysian statute.
#[must_use]
pub fn all_statutes() -> Vec<Statute> {
    vec![
        companies_act_statute(),
        contracts_act_statute(),
        employment_act_statute(),
        pdpa_statute(),
        competition_act_statute(),
        cmsa_statute(),
        income_tax_act_statute(),
        ifsa_statute(),
    ]
}

/// Renders every modelled Malaysian statute as `legalis-dsl` source text.
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
        assert!(!statutes.is_empty(), "MY must model at least one statute");
        assert_eq!(statutes.len(), 8, "MY must model exactly 8 statutes");

        let dsl = statutes_as_dsl();
        assert!(!dsl.is_empty(), "DSL export must not be empty");

        // Every modelled statute's id must appear in the rendered DSL, proving the
        // printer handled each one (covers the range of condition kinds the MY
        // adapters use: Duration, Age, Income, AttributeEquals).
        for statute in &statutes {
            assert!(
                dsl.contains(statute.id.as_str()),
                "statute {} missing from DSL export",
                statute.id
            );
        }
    }
}
