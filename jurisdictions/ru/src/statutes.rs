//! `Statute`-based models of major Russian Federation legislation.
//!
//! This module lifts the validators and domain types implemented across the
//! `legalis-ru` crate (civil, company, competition, criminal, data-protection,
//! labour and tax law) into the jurisdiction-neutral [`legalis_core::Statute`]
//! abstraction. Each builder encodes a *real* statutory provision — accurate code
//! or federal-law number, year and operative rule — as an [`Effect`] with a
//! meaningful [`Condition`] precondition wherever the underlying law turns on a
//! quantifiable trigger (a duration, a monetary or percentage threshold expressed
//! as an attribute, or a status flag).
//!
//! The modelled statutes can be rendered as `legalis-dsl` source text via
//! [`statutes_as_dsl`], so the Russian rule-set can be inspected, diffed,
//! formatted and consumed by the DSL tooling (LSP, documentation generation,
//! structural diffing) on the same footing as every other jurisdiction.
//!
//! # Coverage
//!
//! | Builder | Law |
//! |---------|-----|
//! | [`civil_code_statute`] | Civil Code of the Russian Federation (ГК РФ), Art. 309 |
//! | [`llc_law_statute`] | Federal Law No. 14-FZ on Limited Liability Companies, Art. 14 |
//! | [`joint_stock_company_law_statute`] | Federal Law No. 208-FZ on Joint-Stock Companies, Art. 26 |
//! | [`labour_code_statute`] | Labour Code of the Russian Federation (ТК РФ), Art. 91 |
//! | [`personal_data_law_statute`] | Federal Law No. 152-FZ on Personal Data, Art. 6 |
//! | [`competition_law_statute`] | Federal Law No. 135-FZ on Protection of Competition, Art. 5 |
//! | [`criminal_code_statute`] | Criminal Code of the Russian Federation (УК РФ), Art. 158 |
//! | [`tax_code_vat_statute`] | Tax Code of the Russian Federation (НК РФ), Art. 164 |
//!
//! # Disclaimer
//!
//! These models are simplified abstractions for computational reasoning and are
//! provided for educational and informational purposes only. They are not legal
//! advice; consult a qualified Russian lawyer (advokat / yurist).

use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};

/// Civil Code of the Russian Federation (Гражданский кодекс РФ), Art. 309 —
/// proper performance of obligations.
///
/// Article 309 of Part One of the Civil Code requires that obligations be
/// performed properly in accordance with their terms and the requirements of
/// law, other legal acts and, in the absence thereof, in accordance with the
/// customs of business turnover or other usually imposed requirements. It is the
/// cornerstone *pacta sunt servanda* rule of Russian obligations law, reinforced
/// by Art. 310's prohibition on the unilateral refusal to perform.
///
/// Real source: Civil Code of the Russian Federation, Part One (FZ No. 51-FZ of
/// 30 November 1994), Art. 309.
#[must_use]
pub fn civil_code_statute() -> Statute {
    Statute::new(
        "RU-GK-51FZ-1994-ART309",
        "Proper Performance of Obligations (Civil Code of the RF, Art. 309)",
        Effect::new(
            EffectType::Obligation,
            "Obligations must be performed properly in accordance with their terms and \
             the requirements of law, other legal acts and the customs of business turnover",
        )
        .with_parameter("code", "Civil Code (GK RF)")
        .with_parameter("federal_law", "51-FZ")
        .with_parameter("year", "1994")
        .with_parameter("article", "309"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "obligation_exists".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("RU")
}

/// Federal Law No. 14-FZ on Limited Liability Companies (Об обществах с
/// ограниченной ответственностью), Art. 14 — minimum authorised capital.
///
/// Article 14 requires that the charter (authorised) capital of a limited
/// liability company be no less than 10,000 roubles. The charter capital is made
/// up of the nominal value of the participants' shares and determines the minimum
/// size of the company's property guaranteeing the interests of its creditors.
///
/// Real source: Federal Law No. 14-FZ of 8 February 1998 "On Limited Liability
/// Companies", Art. 14(1).
#[must_use]
pub fn llc_law_statute() -> Statute {
    Statute::new(
        "RU-14FZ-1998-ART14",
        "Minimum Charter Capital of an LLC (Federal Law No. 14-FZ, Art. 14)",
        Effect::new(
            EffectType::Obligation,
            "The charter capital of a limited liability company must be no less than \
             10,000 roubles",
        )
        .with_parameter("federal_law", "14-FZ")
        .with_parameter("year", "1998")
        .with_parameter("article", "14")
        .with_parameter("min_charter_capital_rub", "10000"),
    )
    .with_precondition(Condition::Income {
        operator: ComparisonOp::GreaterOrEqual,
        value: 10_000,
    })
    .with_jurisdiction("RU")
}

/// Federal Law No. 208-FZ on Joint-Stock Companies (Об акционерных обществах),
/// Art. 26 — minimum charter capital of public and non-public companies.
///
/// Article 26 fixes the minimum charter capital of a public joint-stock company
/// (PAO) at 100,000 roubles and that of a non-public joint-stock company (AO) at
/// 10,000 roubles. The charter capital is composed of the nominal value of the
/// shares acquired by the shareholders and secures the interests of creditors.
///
/// Real source: Federal Law No. 208-FZ of 26 December 1995 "On Joint-Stock
/// Companies", Art. 26.
#[must_use]
pub fn joint_stock_company_law_statute() -> Statute {
    Statute::new(
        "RU-208FZ-1995-ART26",
        "Minimum Charter Capital of a Joint-Stock Company (Federal Law No. 208-FZ, Art. 26)",
        Effect::new(
            EffectType::Obligation,
            "A public joint-stock company must have a charter capital of at least \
             100,000 roubles; a non-public joint-stock company at least 10,000 roubles",
        )
        .with_parameter("federal_law", "208-FZ")
        .with_parameter("year", "1995")
        .with_parameter("article", "26")
        .with_parameter("min_capital_public_rub", "100000")
        .with_parameter("min_capital_non_public_rub", "10000"),
    )
    .with_precondition(Condition::Income {
        operator: ComparisonOp::GreaterOrEqual,
        value: 100_000,
    })
    .with_jurisdiction("RU")
}

/// Labour Code of the Russian Federation (Трудовой кодекс РФ), Art. 91 — normal
/// working hours.
///
/// Article 91 defines normal working time and sets the maximum normal working
/// week at 40 hours. The employer must keep a record of the time actually worked
/// by each employee. Reduced working time applies to specified categories of
/// employees under Art. 92.
///
/// Real source: Labour Code of the Russian Federation (FZ No. 197-FZ of 30
/// December 2001), Art. 91.
#[must_use]
pub fn labour_code_statute() -> Statute {
    Statute::new(
        "RU-TK-197FZ-2001-ART91",
        "Normal Working Hours (Labour Code of the RF, Art. 91)",
        Effect::new(
            EffectType::Prohibition,
            "Normal working time must not exceed 40 hours per week",
        )
        .with_parameter("code", "Labour Code (TK RF)")
        .with_parameter("federal_law", "197-FZ")
        .with_parameter("year", "2001")
        .with_parameter("article", "91")
        .with_parameter("max_hours_per_week", "40"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "is_employee".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("RU")
}

/// Federal Law No. 152-FZ on Personal Data (О персональных данных), Art. 6 —
/// lawful conditions for processing personal data.
///
/// Article 6 provides that the processing of personal data is permitted only
/// where at least one statutory condition is met — in the general case, the
/// consent of the data subject to the processing of their personal data.
/// Processing must be limited to the achievement of specific, predetermined and
/// legitimate purposes.
///
/// Real source: Federal Law No. 152-FZ of 27 July 2006 "On Personal Data",
/// Art. 6(1).
#[must_use]
pub fn personal_data_law_statute() -> Statute {
    Statute::new(
        "RU-152FZ-2006-ART6",
        "Lawful Processing of Personal Data (Federal Law No. 152-FZ, Art. 6)",
        Effect::new(
            EffectType::Obligation,
            "Processing of personal data is permitted only where a statutory condition \
             is met, as a rule the consent of the data subject",
        )
        .with_parameter("federal_law", "152-FZ")
        .with_parameter("year", "2006")
        .with_parameter("article", "6"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "data_subject_consent".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("RU")
}

/// Federal Law No. 135-FZ on Protection of Competition (О защите конкуренции),
/// Art. 5 — dominant position on a commodity market.
///
/// Article 5 establishes that the position of an economic entity is presumed
/// dominant where its share on a given commodity market exceeds 50 per cent,
/// unless it is shown that, despite exceeding that share, the entity's position
/// is not dominant. A position below 35 per cent is, as a rule, not regarded as
/// dominant. Dominance is the gateway to the Art. 10 prohibition on abuse.
///
/// Real source: Federal Law No. 135-FZ of 26 July 2006 "On Protection of
/// Competition", Art. 5(1).
#[must_use]
pub fn competition_law_statute() -> Statute {
    Statute::new(
        "RU-135FZ-2006-ART5",
        "Dominant Position on a Commodity Market (Federal Law No. 135-FZ, Art. 5)",
        Effect::new(
            EffectType::StatusChange,
            "An economic entity whose share on a commodity market exceeds 50 per cent is \
             presumed to hold a dominant position",
        )
        .with_parameter("federal_law", "135-FZ")
        .with_parameter("year", "2006")
        .with_parameter("article", "5")
        .with_parameter("dominance_threshold_pct", "50"),
    )
    .with_precondition(Condition::Percentage {
        operator: ComparisonOp::GreaterThan,
        value: 50,
        context: "market_share".to_string(),
    })
    .with_jurisdiction("RU")
}

/// Criminal Code of the Russian Federation (Уголовный кодекс РФ), Art. 158 —
/// theft (krazha).
///
/// Article 158 defines theft as the secret stealing of another's property and
/// prescribes criminal liability ranging from a fine up to deprivation of liberty,
/// with aggravated penalties for theft committed by a group, with unlawful entry,
/// on a significant, large or especially large scale. Criminal liability for
/// theft attaches from the age of 14 under Art. 20(2).
///
/// Real source: Criminal Code of the Russian Federation (FZ No. 63-FZ of 13 June
/// 1996), Art. 158.
#[must_use]
pub fn criminal_code_statute() -> Statute {
    Statute::new(
        "RU-UK-63FZ-1996-ART158",
        "Theft / Krazha (Criminal Code of the RF, Art. 158)",
        Effect::new(
            EffectType::Prohibition,
            "The secret stealing of another's property (theft) is a criminal offence \
             punishable by a fine up to deprivation of liberty",
        )
        .with_parameter("code", "Criminal Code (UK RF)")
        .with_parameter("federal_law", "63-FZ")
        .with_parameter("year", "1996")
        .with_parameter("article", "158")
        .with_parameter("min_age_of_liability", "14"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 14,
    })
    .with_jurisdiction("RU")
}

/// Tax Code of the Russian Federation (Налоговый кодекс РФ), Art. 164 — rates of
/// value-added tax (VAT / NDS).
///
/// Article 164 of Part Two sets the standard rate of value-added tax at 20 per
/// cent, with a reduced 10 per cent rate for specified socially significant goods
/// (certain foodstuffs, children's goods, medicines) and a 0 per cent rate for
/// exports and related supplies. A taxpayer carrying on taxable operations must
/// charge VAT at the applicable rate.
///
/// Real source: Tax Code of the Russian Federation, Part Two (FZ No. 117-FZ of 5
/// August 2000), Art. 164.
#[must_use]
pub fn tax_code_vat_statute() -> Statute {
    Statute::new(
        "RU-NK-117FZ-2000-ART164",
        "Value-Added Tax Rates (Tax Code of the RF, Art. 164)",
        Effect::new(
            EffectType::MonetaryTransfer,
            "A taxpayer carrying on taxable operations must charge value-added tax at the \
             standard rate of 20 per cent (10 per cent reduced, 0 per cent for exports)",
        )
        .with_parameter("code", "Tax Code (NK RF)")
        .with_parameter("federal_law", "117-FZ")
        .with_parameter("year", "2000")
        .with_parameter("article", "164")
        .with_parameter("standard_rate_pct", "20")
        .with_parameter("reduced_rate_pct", "10"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "is_vat_taxpayer".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("RU")
}

/// Returns every modelled Russian Federation statute.
#[must_use]
pub fn all_statutes() -> Vec<Statute> {
    vec![
        civil_code_statute(),
        llc_law_statute(),
        joint_stock_company_law_statute(),
        labour_code_statute(),
        personal_data_law_statute(),
        competition_law_statute(),
        criminal_code_statute(),
        tax_code_vat_statute(),
    ]
}

/// Renders every modelled Russian Federation statute as `legalis-dsl` source text.
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
        assert!(!statutes.is_empty(), "RU must model at least one statute");
        assert_eq!(statutes.len(), 8, "RU must model exactly 8 statutes");

        let dsl = statutes_as_dsl();
        assert!(!dsl.is_empty(), "DSL export must not be empty");

        // Every modelled statute's id must appear in the rendered DSL, proving the
        // printer handled each one (covers the range of condition kinds the RU
        // adapters use: Income, Percentage, Age, AttributeEquals).
        for statute in &statutes {
            assert!(
                dsl.contains(statute.id.as_str()),
                "statute {} missing from DSL export",
                statute.id
            );
        }
    }
}
