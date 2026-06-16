//! `Statute`-based models of major Kingdom of Saudi Arabia legislation.
//!
//! This module lifts the domain types implemented across the `legalis-sa` crate
//! (Basic Law of Governance, Islamic/Sharia commercial principles, the Companies
//! Law, the Labor Law, the Capital Market Law, the Personal Data Protection Law,
//! the Zakat/corporate income tax regime and the VAT Law) into the
//! jurisdiction-neutral [`legalis_core::Statute`] abstraction. Each builder
//! encodes a *real* statutory provision — accurate Royal Decree number, Hijri /
//! Gregorian year and operative rule — as an [`Effect`] with a meaningful
//! [`Condition`] precondition wherever the underlying law turns on a quantifiable
//! trigger (a duration, a monetary threshold expressed as an attribute, or a
//! status flag).
//!
//! The modelled statutes can be rendered as `legalis-dsl` source text via
//! [`statutes_as_dsl`], so the Saudi rule-set can be inspected, diffed, formatted
//! and consumed by the DSL tooling (LSP, documentation generation, structural
//! diffing) on the same footing as every other jurisdiction.
//!
//! # Coverage
//!
//! | Builder | Law |
//! |---------|-----|
//! | [`basic_law_governance_statute`] | Basic Law of Governance, Royal Decree A/90 (1992) |
//! | [`companies_law_statute`] | Companies Law, Royal Decree M/132 (2022) |
//! | [`labor_law_eosa_statute`] | Saudi Labor Law, Royal Decree M/51 (2005), art. 84 |
//! | [`pdpl_statute`] | Personal Data Protection Law, Royal Decree M/19 (2021/2023) |
//! | [`capital_market_law_statute`] | Capital Market Law, Royal Decree M/30 (2003) |
//! | [`riba_prohibition_statute`] | Sharia prohibition of riba (usury/interest) |
//! | [`zakat_income_tax_statute`] | Zakat & corporate income tax (ZATCA) |
//! | [`vat_law_statute`] | VAT Law, Royal Decree M/113 (2017) |
//!
//! # Disclaimer
//!
//! These models are simplified abstractions for computational reasoning and are
//! provided for educational and informational purposes only. They are not legal
//! advice; consult a qualified Saudi legal professional (محامٍ سعودي).

use legalis_core::{ComparisonOp, Condition, DurationUnit, Effect, EffectType, Statute};

/// Basic Law of Governance, Royal Decree No. A/90 of 1412H (1992), art. 1 & 7 —
/// Sharia as the supreme source of governance.
///
/// The Basic Law of Governance (النظام الأساسي للحكم) is Saudi Arabia's
/// constitutional document. Article 1 declares the Kingdom an Arab Islamic
/// sovereign state whose constitution is the Holy Quran and the Sunnah, and
/// Article 7 provides that governance derives its authority from the Quran and
/// the Sunnah, which govern this and all other laws of the State. Modelled as a
/// Grant of the constitutional supremacy of Sharia, conditioned on the instrument
/// being the Basic Law itself.
///
/// Real source: Basic Law of Governance, Royal Decree A/90 (1992), arts. 1 & 7.
#[must_use]
pub fn basic_law_governance_statute() -> Statute {
    Statute::new(
        "SA-BASIC-LAW-1992",
        "Sharia as Supreme Source of Governance (Basic Law of Governance, Royal Decree A/90 of 1992)",
        Effect::new(
            EffectType::Grant,
            "The constitution of the Kingdom is the Holy Quran and the Sunnah; governance derives \
             its authority from them and they govern this and all other laws of the State",
        )
        .with_parameter("instrument", "Basic Law of Governance")
        .with_parameter("royal_decree", "A/90")
        .with_parameter("year", "1992")
        .with_parameter("hijri_year", "1412")
        .with_parameter("articles", "1, 7"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "instrument".to_string(),
        value: "basic_law_of_governance".to_string(),
    })
    .with_jurisdiction("SA")
}

/// Companies Law, Royal Decree No. M/132 of 1443H (2022) — minimum number of
/// partners for a limited liability company.
///
/// The Companies Law (نظام الشركات), enacted by Royal Decree M/132 dated
/// 1/12/1443H and effective in January 2023, modernised and replaced the 2015
/// Companies Law. It governs the formation and governance of joint stock
/// companies, limited liability companies, the new simplified joint stock company
/// and other forms. A limited liability company may be formed by one or more
/// partners, and the number of partners must not exceed fifty. Modelled here as
/// the obligation that an LLC be constituted by at least one partner.
///
/// Real source: Companies Law, Royal Decree M/132 (2022).
#[must_use]
pub fn companies_law_statute() -> Statute {
    Statute::new(
        "SA-COMPANIES-2022",
        "Limited Liability Company Formation (Companies Law, Royal Decree M/132 of 2022)",
        Effect::new(
            EffectType::Obligation,
            "A limited liability company must be constituted by at least one and no more than \
             fifty partners and registered in the commercial register",
        )
        .with_parameter("royal_decree", "M/132")
        .with_parameter("year", "2022")
        .with_parameter("hijri_year", "1443")
        .with_parameter("min_partners", "1")
        .with_parameter("max_partners", "50"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "company_type".to_string(),
        value: "llc".to_string(),
    })
    .with_jurisdiction("SA")
}

/// Saudi Labor Law, Royal Decree No. M/51 of 1426H (2005), art. 84 — End of
/// Service Award (مكافأة نهاية الخدمة).
///
/// Article 84 of the Labor Law (نظام العمل) entitles a worker, upon the end of
/// the work relationship, to an End of Service Award of half a month's wage for
/// each of the first five years of service and one month's wage for each
/// subsequent year, calculated on the last wage. The entitlement accrues with
/// continuous service; modelled here as a Grant conditioned on at least twelve
/// months of service.
///
/// Real source: Saudi Labor Law, Royal Decree M/51 (2005), art. 84.
#[must_use]
pub fn labor_law_eosa_statute() -> Statute {
    Statute::new(
        "SA-LABOR-2005-EOSA",
        "End of Service Award (Labor Law, Royal Decree M/51 of 2005, art. 84)",
        Effect::new(
            EffectType::Grant,
            "On termination, a worker is entitled to an End of Service Award of half a month's \
             wage for each of the first five years and one month's wage for each subsequent year",
        )
        .with_parameter("royal_decree", "M/51")
        .with_parameter("year", "2005")
        .with_parameter("hijri_year", "1426")
        .with_parameter("article", "84")
        .with_parameter("award_first_five_years", "0.5 month/year")
        .with_parameter("award_subsequent_years", "1 month/year"),
    )
    .with_precondition(Condition::Duration {
        operator: ComparisonOp::GreaterOrEqual,
        value: 12,
        unit: DurationUnit::Months,
    })
    .with_jurisdiction("SA")
}

/// Personal Data Protection Law, Royal Decree No. M/19 of 1443H (2021, amended
/// 2023), art. 20 — notification of a personal data breach.
///
/// The Personal Data Protection Law (نظام حماية البيانات الشخصية), issued by
/// Royal Decree M/19 dated 9/2/1443H, amended by Royal Decree M/148 of 1444H and
/// enforced from September 2023, is overseen by SDAIA. Article 20 requires a
/// controller, upon becoming aware of a personal data breach or leak that may
/// cause harm to the data or the data subject, to notify the competent authority
/// (SDAIA) and, where the breach may cause serious harm, to notify the affected
/// data subjects. Modelled as an obligation triggered by a personal data breach.
///
/// Real source: Personal Data Protection Law, Royal Decree M/19 (2021/2023),
/// art. 20.
#[must_use]
pub fn pdpl_statute() -> Statute {
    Statute::new(
        "SA-PDPL-2023",
        "Personal Data Breach Notification (PDPL, Royal Decree M/19 of 2021/2023, art. 20)",
        Effect::new(
            EffectType::Obligation,
            "A controller must notify the competent authority (SDAIA) upon becoming aware of a \
             personal data breach, and notify affected data subjects where serious harm may result",
        )
        .with_parameter("royal_decree", "M/19")
        .with_parameter("year", "2021")
        .with_parameter("hijri_year", "1443")
        .with_parameter("enforced_year", "2023")
        .with_parameter("article", "20")
        .with_parameter("regulator", "SDAIA"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "personal_data_breach".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("SA")
}

/// Capital Market Law, Royal Decree No. M/30 of 1424H (2003), art. 31 & 49 —
/// prohibition of insider trading on the Saudi Exchange (Tadawul).
///
/// The Capital Market Law (نظام السوق المالية), promulgated by Royal Decree M/30
/// dated 2/6/1424H, established the Capital Market Authority (CMA, هيئة السوق
/// المالية) and the Saudi Exchange (Tadawul). It prohibits market manipulation
/// and insider trading: a person who obtains, through a position or relationship,
/// inside (non-public, price-sensitive) information must not trade on it or
/// disclose it to others to trade. Modelled as a Prohibition triggered where the
/// trader possesses material non-public information.
///
/// Real source: Capital Market Law, Royal Decree M/30 (2003), arts. 31 & 49.
#[must_use]
pub fn capital_market_law_statute() -> Statute {
    Statute::new(
        "SA-CML-2003",
        "Prohibition of Insider Trading (Capital Market Law, Royal Decree M/30 of 2003)",
        Effect::new(
            EffectType::Prohibition,
            "A person possessing material non-public inside information must not trade in the \
             relevant security on the Saudi Exchange or disclose that information for others to trade",
        )
        .with_parameter("royal_decree", "M/30")
        .with_parameter("year", "2003")
        .with_parameter("hijri_year", "1424")
        .with_parameter("articles", "31, 49")
        .with_parameter("regulator", "CMA"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "possesses_inside_information".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("SA")
}

/// Sharia commercial principle — prohibition of riba (الربا, usury/interest).
///
/// The prohibition of riba is a foundational principle of Islamic commercial law
/// (فقه المعاملات), derived from the Holy Quran (Al-Baqarah 2:275–279) and the
/// Sunnah, and applied in the Kingdom through the Hanbali school of jurisprudence.
/// Any predetermined increase (interest) stipulated on a loan or debt is unlawful,
/// rendering the offending term void; Sharia-compliant alternatives (murabaha,
/// ijara, mudaraba, musharaka) are used instead. Modelled as a Prohibition
/// triggered where a transaction stipulates interest on a loan or debt.
///
/// Real source: Holy Quran, Surah Al-Baqarah (2:275–279); Hanbali fiqh
/// al-mu'amalat.
#[must_use]
pub fn riba_prohibition_statute() -> Statute {
    Statute::new(
        "SA-SHARIA-RIBA",
        "Prohibition of Riba (Usury/Interest) — Sharia Commercial Principle",
        Effect::new(
            EffectType::Prohibition,
            "Any predetermined increase (interest) stipulated on a loan or debt is riba and is \
             prohibited; the offending stipulation is void and Sharia-compliant structures must be used",
        )
        .with_parameter("source", "Quran 2:275-279; Sunnah")
        .with_parameter("school", "Hanbali")
        .with_parameter("principle", "riba_prohibition"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "stipulates_interest_on_loan".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("SA")
}

/// Zakat and corporate income tax (ZATCA) — Zakat collection rule and Income Tax
/// Law, Royal Decree No. M/1 of 1425H (2004).
///
/// The Zakat, Tax and Customs Authority (ZATCA, هيئة الزكاة والضريبة والجمارك)
/// administers Zakat and corporate income tax. Zakat (الزكاة), a religious levy,
/// is assessed at 2.5% of the Zakat base on the Saudi/GCC-owned share of a
/// company, while the non-Saudi (foreign) share of resident capital companies is
/// subject to corporate income tax at 20% of the net adjusted profit. Modelled as
/// a MonetaryTransfer (the levy) triggered where the taxpayer's annual profit is
/// positive (greater than zero, expressed in halalas).
///
/// Real source: Income Tax Law, Royal Decree M/1 (2004); Zakat collection
/// regulations administered by ZATCA.
#[must_use]
pub fn zakat_income_tax_statute() -> Statute {
    Statute::new(
        "SA-ZATCA-ZAKAT-TAX",
        "Zakat and Corporate Income Tax (ZATCA; Income Tax Law, Royal Decree M/1 of 2004)",
        Effect::new(
            EffectType::MonetaryTransfer,
            "Zakat is levied at 2.5% of the Zakat base on the Saudi/GCC-owned share, and corporate \
             income tax at 20% of net adjusted profit on the non-Saudi share, payable to ZATCA",
        )
        .with_parameter("royal_decree", "M/1")
        .with_parameter("year", "2004")
        .with_parameter("hijri_year", "1425")
        .with_parameter("zakat_rate_pct", "2.5")
        .with_parameter("corporate_income_tax_rate_pct", "20")
        .with_parameter("authority", "ZATCA"),
    )
    .with_precondition(Condition::Income {
        operator: ComparisonOp::GreaterThan,
        value: 0,
    })
    .with_jurisdiction("SA")
}

/// VAT Law, Royal Decree No. M/113 of 1438H (2017), art. 2 & 50 — mandatory VAT
/// registration above the registration threshold.
///
/// The Value Added Tax Law (نظام ضريبة القيمة المضافة), issued by Royal Decree
/// M/113 dated 2/11/1438H and effective 1 January 2018, implements the GCC VAT
/// Framework Agreement. A person carrying on an economic activity whose annual
/// taxable supplies exceed the mandatory registration threshold of SAR 375,000
/// must register with ZATCA and charge VAT, levied at the standard rate (raised
/// to 15% with effect from 1 July 2020). Modelled as an obligation triggered
/// where annual taxable supplies exceed the SAR 375,000 threshold (in halalas).
///
/// Real source: VAT Law, Royal Decree M/113 (2017), arts. 2 & 50.
#[must_use]
pub fn vat_law_statute() -> Statute {
    Statute::new(
        "SA-VAT-2017",
        "Mandatory VAT Registration (VAT Law, Royal Decree M/113 of 2017)",
        Effect::new(
            EffectType::Obligation,
            "A person whose annual taxable supplies exceed SAR 375,000 must register for VAT with \
             ZATCA and charge VAT at the standard rate of 15%",
        )
        .with_parameter("royal_decree", "M/113")
        .with_parameter("year", "2017")
        .with_parameter("hijri_year", "1438")
        .with_parameter("mandatory_threshold_sar", "375000")
        .with_parameter("standard_rate_pct", "15")
        .with_parameter("authority", "ZATCA"),
    )
    .with_precondition(Condition::Income {
        operator: ComparisonOp::GreaterThan,
        value: 37_500_000,
    })
    .with_jurisdiction("SA")
}

/// Returns every modelled Saudi Arabian statute.
#[must_use]
pub fn all_statutes() -> Vec<Statute> {
    vec![
        basic_law_governance_statute(),
        companies_law_statute(),
        labor_law_eosa_statute(),
        pdpl_statute(),
        capital_market_law_statute(),
        riba_prohibition_statute(),
        zakat_income_tax_statute(),
        vat_law_statute(),
    ]
}

/// Renders every modelled Saudi Arabian statute as `legalis-dsl` source text.
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
        assert!(!statutes.is_empty(), "SA must model at least one statute");
        assert_eq!(statutes.len(), 8, "SA must model exactly 8 statutes");

        let dsl = statutes_as_dsl();
        assert!(!dsl.is_empty(), "DSL export must not be empty");

        // Every modelled statute's id must appear in the rendered DSL, proving the
        // printer handled each one (covers the range of condition kinds the SA
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
