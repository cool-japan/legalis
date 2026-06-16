//! `Statute`-based models of major United Arab Emirates legislation.
//!
//! This module lifts the validators and domain types implemented across the
//! `legalis-ae` crate (civil code, commercial companies, labour law, data
//! protection, criminal code, tax law, cybercrime, real estate and Islamic law)
//! into the jurisdiction-neutral [`legalis_core::Statute`] abstraction. Each
//! builder encodes a *real* provision of UAE federal law — accurate instrument
//! number, year and operative rule — as an [`Effect`] carrying a meaningful
//! [`Condition`] precondition wherever the underlying law turns on a quantifiable
//! trigger (a statutory duration, a monetary threshold expressed as an attribute,
//! or a status flag).
//!
//! The UAE is a civil-law jurisdiction whose federal legislation is enacted as
//! Federal Laws (`Federal Law No. X/YYYY`) and, increasingly, Federal Decree-Laws
//! (`Federal Decree-Law No. X/YYYY`). The statutes modelled here span the core of
//! the post-2021 reform wave (companies, labour and personal data) together with
//! the long-standing Civil Transactions Law, the Penal Code, VAT, the
//! Anti-Cybercrime Law and the real-estate / Islamic-finance domains.
//!
//! The modelled statutes can be rendered as `legalis-dsl` source text via
//! [`statutes_as_dsl`], so the UAE rule-set can be inspected, diffed, formatted
//! and consumed by the DSL tooling (LSP, documentation generation, structural
//! diffing) on the same footing as every other jurisdiction.
//!
//! # Coverage
//!
//! | Builder | Instrument |
//! |---------|------------|
//! | [`civil_transactions_statute`] | Federal Law No. 5 of 1985 (Civil Transactions Law / Civil Code) |
//! | [`commercial_companies_statute`] | Federal Decree-Law No. 32 of 2021 (Commercial Companies) |
//! | [`labour_law_statute`] | Federal Decree-Law No. 33 of 2021 (Regulation of Labour Relations) |
//! | [`data_protection_statute`] | Federal Decree-Law No. 45 of 2021 (Personal Data Protection) |
//! | [`penal_code_statute`] | Federal Decree-Law No. 31 of 2021 (Penal Code / Crimes and Penalties) |
//! | [`vat_statute`] | Federal Decree-Law No. 8 of 2017 (Value Added Tax) |
//! | [`cybercrime_statute`] | Federal Decree-Law No. 34 of 2021 (Combating Rumours and Cybercrime) |
//! | [`real_estate_escrow_statute`] | Dubai Law No. 8 of 2007 (Escrow Accounts for Real Estate Development) |
//!
//! # Disclaimer
//!
//! These models are simplified abstractions for computational reasoning and are
//! provided for educational and informational purposes only. They are not legal
//! advice; consult a qualified UAE legal professional (محامي).

use legalis_core::{ComparisonOp, Condition, DurationUnit, Effect, EffectType, Statute};

/// Federal Law No. 5 of 1985 (Civil Transactions Law / Civil Code), Article 246 —
/// performance of contracts in good faith.
///
/// The Civil Transactions Law is the UAE's civil code, derived from the Egyptian
/// (and thus French Napoleonic) tradition and Islamic Sharia. Article 246(1)
/// provides that a contract must be performed in accordance with its contents and
/// in a manner consistent with the requirements of good faith. This pacta sunt
/// servanda / good-faith duty is the cornerstone obligation binding every
/// validly concluded contract, modelled here as an obligation triggered once a
/// binding contract exists.
///
/// Real source: Federal Law No. 5 of 1985, Article 246(1).
#[must_use]
pub fn civil_transactions_statute() -> Statute {
    Statute::new(
        "AE-CIVIL-1985-ART246",
        "Performance of Contracts in Good Faith (Federal Law No. 5 of 1985, Art. 246)",
        Effect::new(
            EffectType::Obligation,
            "A contract must be performed in accordance with its contents and in a \
             manner consistent with the requirements of good faith",
        )
        .with_parameter("instrument", "Federal Law")
        .with_parameter("number", "5")
        .with_parameter("year", "1985")
        .with_parameter("article", "246"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "binding_contract_concluded".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("AE")
}

/// Federal Decree-Law No. 32 of 2021 (Commercial Companies Law), Article 27 —
/// appointment of an auditor and audited annual financial statements.
///
/// The Commercial Companies Law overhauled UAE corporate law, notably permitting
/// up to 100% foreign ownership in most onshore activities. Under Article 27 a
/// company must appoint one or more auditors to audit the accounts of each
/// financial year, and the company must prepare annual financial statements
/// within a defined period after the end of the financial year. Modelled here as
/// the obligation to finalise audited annual financial statements within the
/// statutory window following the financial year-end.
///
/// Real source: Federal Decree-Law No. 32 of 2021, Article 27.
#[must_use]
pub fn commercial_companies_statute() -> Statute {
    Statute::new(
        "AE-COMPANIES-2021",
        "Audited Annual Financial Statements (Federal Decree-Law No. 32 of 2021, Art. 27)",
        Effect::new(
            EffectType::Obligation,
            "A company must appoint an auditor and prepare audited annual financial \
             statements within the statutory period after the end of its financial year",
        )
        .with_parameter("instrument", "Federal Decree-Law")
        .with_parameter("number", "32")
        .with_parameter("year", "2021")
        .with_parameter("article", "27"),
    )
    .with_precondition(Condition::Duration {
        operator: ComparisonOp::LessOrEqual,
        value: 4,
        unit: DurationUnit::Months,
    })
    .with_jurisdiction("AE")
}

/// Federal Decree-Law No. 33 of 2021 (Regulation of Labour Relations), Article 51 —
/// End of Service Gratuity entitlement.
///
/// The UAE Labour Law (effective February 2022) governs private-sector employment.
/// Article 51 entitles a full-time worker who has completed one or more years of
/// continuous service to an End of Service Gratuity calculated on the basic wage:
/// 21 days' wage for each of the first five years of service, and 30 days' wage
/// for each subsequent year. The entitlement accrues only once one full year of
/// continuous service is reached.
///
/// Real source: Federal Decree-Law No. 33 of 2021, Article 51.
#[must_use]
pub fn labour_law_statute() -> Statute {
    Statute::new(
        "AE-LABOUR-2021",
        "End of Service Gratuity (Federal Decree-Law No. 33 of 2021, Art. 51)",
        Effect::new(
            EffectType::Grant,
            "A full-time worker who completes one year or more of continuous service is \
             entitled to End of Service Gratuity of 21 days' basic wage per year for the \
             first five years and 30 days' basic wage per subsequent year",
        )
        .with_parameter("instrument", "Federal Decree-Law")
        .with_parameter("number", "33")
        .with_parameter("year", "2021")
        .with_parameter("article", "51")
        .with_parameter("days_per_year_first_five", "21")
        .with_parameter("days_per_year_thereafter", "30"),
    )
    .with_precondition(Condition::Duration {
        operator: ComparisonOp::GreaterOrEqual,
        value: 1,
        unit: DurationUnit::Years,
    })
    .with_jurisdiction("AE")
}

/// Federal Decree-Law No. 45 of 2021 (Personal Data Protection Law / PDPL),
/// Article 9 — notification of a personal data breach.
///
/// The PDPL is the UAE's first comprehensive, GDPR-aligned federal data protection
/// statute. Article 9 requires a Controller, upon becoming aware of a breach of
/// personal data that would prejudice the privacy, confidentiality or security of
/// a data subject, to notify the UAE Data Office (and, where applicable, the data
/// subject) immediately upon becoming aware of the breach. Modelled as an
/// obligation triggered once such a personal-data breach has occurred.
///
/// Real source: Federal Decree-Law No. 45 of 2021, Article 9.
#[must_use]
pub fn data_protection_statute() -> Statute {
    Statute::new(
        "AE-PDPL-2021",
        "Personal Data Breach Notification (Federal Decree-Law No. 45 of 2021, Art. 9)",
        Effect::new(
            EffectType::Obligation,
            "A controller must notify the UAE Data Office immediately upon becoming aware \
             of a personal data breach that prejudices the privacy, confidentiality or \
             security of a data subject",
        )
        .with_parameter("instrument", "Federal Decree-Law")
        .with_parameter("number", "45")
        .with_parameter("year", "2021")
        .with_parameter("article", "9"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "personal_data_breach".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("AE")
}

/// Federal Decree-Law No. 31 of 2021 (Penal Code / Crimes and Penalties Law),
/// Article 384 — criminal breach of trust (misappropriation).
///
/// The Penal Code (effective January 2022) is the UAE's principal criminal statute,
/// replacing Federal Law No. 3 of 1987. Article 384 penalises a person who
/// misappropriates, uses or dissipates money, goods or any movable property
/// entrusted to them on the basis of a deposit, lease, pledge, loan for use or
/// agency, to the prejudice of those entitled to it (criminal breach of trust).
/// Modelled as a prohibition triggered when property held on trust is
/// misappropriated.
///
/// Real source: Federal Decree-Law No. 31 of 2021, Article 384.
#[must_use]
pub fn penal_code_statute() -> Statute {
    Statute::new(
        "AE-PENAL-2021-ART384",
        "Criminal Breach of Trust (Federal Decree-Law No. 31 of 2021, Art. 384)",
        Effect::new(
            EffectType::Prohibition,
            "A person entrusted with money or movable property under a deposit, lease, \
             pledge, loan for use or agency must not misappropriate or dissipate it to \
             the prejudice of those entitled",
        )
        .with_parameter("instrument", "Federal Decree-Law")
        .with_parameter("number", "31")
        .with_parameter("year", "2021")
        .with_parameter("article", "384"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "entrusted_property_misappropriated".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("AE")
}

/// Federal Decree-Law No. 8 of 2017 (Value Added Tax Law), Article 13 —
/// mandatory registration for VAT.
///
/// VAT was introduced in the UAE on 1 January 2018 at the standard rate of 5%.
/// Under Article 13, read with the Executive Regulations, a taxable person whose
/// total value of taxable supplies and imports exceeds the mandatory registration
/// threshold of AED 375,000 over the preceding 12 months (or is expected to exceed
/// it in the next 30 days) must register with the Federal Tax Authority and charge
/// VAT at the standard rate. Threshold expressed in fils (1 dirham = 100 fils).
///
/// Real source: Federal Decree-Law No. 8 of 2017, Article 13.
#[must_use]
pub fn vat_statute() -> Statute {
    Statute::new(
        "AE-VAT-2017",
        "Mandatory VAT Registration (Federal Decree-Law No. 8 of 2017, Art. 13)",
        Effect::new(
            EffectType::Obligation,
            "A taxable person whose taxable supplies and imports exceed AED 375,000 over \
             12 months must register with the Federal Tax Authority and charge VAT at the \
             standard rate of 5%",
        )
        .with_parameter("instrument", "Federal Decree-Law")
        .with_parameter("number", "8")
        .with_parameter("year", "2017")
        .with_parameter("article", "13")
        .with_parameter("threshold_aed", "375000")
        .with_parameter("standard_rate_pct", "5"),
    )
    .with_precondition(Condition::Income {
        operator: ComparisonOp::GreaterThan,
        value: 37_500_000,
    })
    .with_jurisdiction("AE")
}

/// Federal Decree-Law No. 34 of 2021 (Combating Rumours and Cybercrime), Article 2 —
/// prohibition on unauthorised access to information systems.
///
/// The Anti-Cybercrime Law (effective January 2022) replaced Federal Law No. 5 of
/// 2012 and is the UAE's principal statute against information-technology crimes.
/// Article 2 criminalises gaining access, without authorisation, to a website,
/// electronic information system, information network or information technology
/// means, with penalties escalating where data is obtained, deleted, altered,
/// damaged or disclosed. Modelled as a prohibition triggered by unauthorised
/// system access.
///
/// Real source: Federal Decree-Law No. 34 of 2021, Article 2.
#[must_use]
pub fn cybercrime_statute() -> Statute {
    Statute::new(
        "AE-CYBERCRIME-2021",
        "Unauthorised Access to Information Systems (Federal Decree-Law No. 34 of 2021, Art. 2)",
        Effect::new(
            EffectType::Prohibition,
            "Gaining access without authorisation to a website, electronic information \
             system, information network or information technology means is a criminal \
             offence",
        )
        .with_parameter("instrument", "Federal Decree-Law")
        .with_parameter("number", "34")
        .with_parameter("year", "2021")
        .with_parameter("article", "2"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "unauthorised_system_access".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("AE")
}

/// Dubai Law No. 8 of 2007 (Escrow Accounts for Real Estate Development), Article 7 —
/// deposit of off-plan purchaser funds into a guarantee (escrow) account.
///
/// Off-plan property sales are central to the UAE real-estate market. Dubai Law
/// No. 8 of 2007 requires a developer selling units off-plan to deposit all
/// amounts paid by purchasers (and any project financing) into a dedicated
/// guarantee account held with an accredited escrow agent, from which funds may be
/// released only against certified construction progress. This ring-fences
/// purchaser money and is enforced by the Dubai Land Department (RERA). Modelled
/// as an obligation triggered once off-plan units are sold.
///
/// Real source: Dubai Law No. 8 of 2007, Article 7.
#[must_use]
pub fn real_estate_escrow_statute() -> Statute {
    Statute::new(
        "AE-DUBAI-ESCROW-2007",
        "Real Estate Development Escrow Account (Dubai Law No. 8 of 2007, Art. 7)",
        Effect::new(
            EffectType::Obligation,
            "A developer selling real estate units off-plan must deposit all purchaser \
             payments into an accredited guarantee (escrow) account, releasable only \
             against certified construction progress",
        )
        .with_parameter("instrument", "Dubai Law")
        .with_parameter("number", "8")
        .with_parameter("year", "2007")
        .with_parameter("article", "7"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "off_plan_units_sold".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("AE")
}

/// Returns every modelled United Arab Emirates statute.
#[must_use]
pub fn all_statutes() -> Vec<Statute> {
    vec![
        civil_transactions_statute(),
        commercial_companies_statute(),
        labour_law_statute(),
        data_protection_statute(),
        penal_code_statute(),
        vat_statute(),
        cybercrime_statute(),
        real_estate_escrow_statute(),
    ]
}

/// Renders every modelled UAE statute as `legalis-dsl` source text.
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
        assert!(!statutes.is_empty(), "AE must model at least one statute");
        assert_eq!(statutes.len(), 8, "AE must model exactly 8 statutes");

        let dsl = statutes_as_dsl();
        assert!(!dsl.is_empty(), "DSL export must not be empty");

        // Every modelled statute's id must appear in the rendered DSL, proving the
        // printer handled each one (covers the range of condition kinds the AE
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
