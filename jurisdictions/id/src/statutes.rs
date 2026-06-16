//! `Statute`-based models of major Indonesian legislation.
//!
//! This module lifts the validators and domain types implemented across the
//! `legalis-id` crate (civil code, company law, manpower / omnibus law, personal
//! data protection, investment, capital markets, agrarian/land law and taxation)
//! into the jurisdiction-neutral [`legalis_core::Statute`] abstraction. Each
//! builder encodes a *real* statutory provision — accurate law number, year and
//! operative rule — as an [`Effect`] with a meaningful [`Condition`] precondition
//! where the underlying law turns on a quantifiable trigger (a duration, a
//! monetary threshold expressed as income, or a status flag).
//!
//! The modelled statutes can be rendered as `legalis-dsl` source text via
//! [`statutes_as_dsl`], so the Indonesian rule-set can be inspected, diffed,
//! formatted and consumed by the DSL tooling (LSP, documentation generation,
//! structural diffing) on the same footing as every other jurisdiction.
//!
//! # Coverage
//!
//! | Builder | Law |
//! |---------|-----|
//! | [`civil_code_contract_validity_statute`] | KUHPerdata (Indonesian Civil Code), Pasal 1320 |
//! | [`company_law_statute`] | UU No. 40 Tahun 2007 (Perseroan Terbatas), Pasal 7 |
//! | [`manpower_severance_statute`] | UU No. 6 Tahun 2023 (Cipta Kerja) amending UU No. 13/2003, Pasal 156 |
//! | [`pdp_breach_notification_statute`] | UU No. 27 Tahun 2022 (Perlindungan Data Pribadi), Pasal 46 |
//! | [`investment_equal_treatment_statute`] | UU No. 25 Tahun 2007 (Penanaman Modal), Pasal 6 |
//! | [`capital_markets_registration_statute`] | UU No. 8 Tahun 1995 (Pasar Modal), Pasal 70 |
//! | [`agrarian_hak_milik_statute`] | UU No. 5 Tahun 1960 (UUPA), Pasal 21 |
//! | [`vat_registration_statute`] | UU No. 7 Tahun 2021 (HPP) / UU PPN, PKP registration |
//!
//! # Disclaimer
//!
//! These models are simplified abstractions for computational reasoning and are
//! provided for educational and informational purposes only. They are not legal
//! advice; consult a qualified Indonesian legal professional (advokat/pengacara).

use legalis_core::{ComparisonOp, Condition, DurationUnit, Effect, EffectType, Statute};

/// KUHPerdata (Indonesian Civil Code), Pasal 1320 — conditions for a valid
/// agreement.
///
/// Article 1320 of the Burgerlijk Wetboek (Kitab Undang-Undang Hukum Perdata)
/// lays down the four cumulative requirements for a legally binding contract:
/// (1) consent of the parties bound (*kesepakatan*), (2) capacity to enter into
/// an obligation (*kecakapan*), (3) a specific subject matter (*suatu hal
/// tertentu*), and (4) a lawful cause (*suatu sebab yang halal*). Where all four
/// are satisfied the agreement binds the parties as law (Pasal 1338). Modelled
/// as a Grant of binding force once the four-element test is met.
///
/// Real source: KUHPerdata (Civil Code), Pasal 1320.
#[must_use]
pub fn civil_code_contract_validity_statute() -> Statute {
    Statute::new(
        "ID-KUHPERDATA-1320",
        "Conditions for a Valid Agreement (KUHPerdata Pasal 1320)",
        Effect::new(
            EffectType::Grant,
            "An agreement is legally binding on the parties as law where there is \
             consent, capacity, a specific subject matter and a lawful cause",
        )
        .with_parameter("code", "KUHPerdata")
        .with_parameter("article", "1320")
        .with_parameter("required_elements", "4"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "valid_contract_elements_met".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("ID")
}

/// UU No. 40 Tahun 2007 tentang Perseroan Terbatas, Pasal 7 — establishment of a
/// limited liability company (PT) by at least two founders.
///
/// Article 7(1) of the Limited Liability Company Law requires that a *Perseroan
/// Terbatas* be established by two or more persons by notarial deed drawn up in
/// the Indonesian language. Each founder must subscribe to shares on
/// establishment (Pasal 7(2)), and the company acquires legal personality once
/// the deed is ratified by the Minister of Law and Human Rights (Pasal 7(4)).
/// Modelled as the obligation that a PT have at least two founding shareholders.
///
/// Real source: UU No. 40 Tahun 2007 (Perseroan Terbatas), Pasal 7(1).
#[must_use]
pub fn company_law_statute() -> Statute {
    Statute::new(
        "ID-PT-2007",
        "Establishment of a Limited Liability Company (UU 40/2007, Pasal 7)",
        Effect::new(
            EffectType::Obligation,
            "A limited liability company (PT) must be established by at least two \
             founders by notarial deed and obtain ministerial ratification to acquire \
             legal personality",
        )
        .with_parameter("law_number", "40")
        .with_parameter("law_year", "2007")
        .with_parameter("article", "7")
        .with_parameter("min_founders", "2"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "founders".to_string(),
        value: "2".to_string(),
    })
    .with_jurisdiction("ID")
}

/// UU No. 6 Tahun 2023 (Cipta Kerja / Omnibus Law) amending UU No. 13 Tahun 2003
/// tentang Ketenagakerjaan, Pasal 156 — severance pay on termination of
/// employment.
///
/// As reshaped by the Job Creation Law, Article 156(1) provides that where an
/// employment relationship is terminated the employer must pay severance pay
/// (*uang pesangon*), long-service pay (*uang penghargaan masa kerja*) and
/// compensation for entitlements (*uang penggantian hak*). The severance and
/// long-service components scale with the worker's continuous length of service.
/// Modelled as an obligation triggered once the worker completes a full year of
/// service.
///
/// Real source: UU No. 6 Tahun 2023 (Cipta Kerja), amending UU No. 13/2003,
/// Pasal 156.
#[must_use]
pub fn manpower_severance_statute() -> Statute {
    Statute::new(
        "ID-CIPTAKERJA-2023-PESANGON",
        "Severance Pay on Termination (UU 6/2023 Cipta Kerja, Pasal 156)",
        Effect::new(
            EffectType::Obligation,
            "On termination of employment the employer must pay severance pay, \
             long-service pay and compensation for entitlements scaled by length of \
             service",
        )
        .with_parameter("law_number", "6")
        .with_parameter("law_year", "2023")
        .with_parameter("amends", "UU 13/2003")
        .with_parameter("article", "156"),
    )
    .with_precondition(Condition::Duration {
        operator: ComparisonOp::GreaterOrEqual,
        value: 12,
        unit: DurationUnit::Months,
    })
    .with_jurisdiction("ID")
}

/// UU No. 27 Tahun 2022 tentang Pelindungan Data Pribadi (UU PDP), Pasal 46 —
/// notification of a personal data breach.
///
/// Article 46(1) of the Personal Data Protection Law requires a data controller,
/// in the event of a failure to protect personal data, to notify in writing the
/// affected data subject and the supervisory authority no later than 3 x 24 hours
/// (72 hours). The notification must state the personal data disclosed, when and
/// how the breach occurred, and the controller's handling and recovery measures.
/// Modelled as an obligation triggered when a personal data breach occurs.
///
/// Real source: UU No. 27 Tahun 2022 (Pelindungan Data Pribadi), Pasal 46.
#[must_use]
pub fn pdp_breach_notification_statute() -> Statute {
    Statute::new(
        "ID-PDP-2022",
        "Personal Data Breach Notification (UU 27/2022 PDP, Pasal 46)",
        Effect::new(
            EffectType::Obligation,
            "A data controller must notify the affected data subject and the \
             supervisory authority of a personal data breach within 3 x 24 hours \
             (72 hours)",
        )
        .with_parameter("law_number", "27")
        .with_parameter("law_year", "2022")
        .with_parameter("article", "46")
        .with_parameter("deadline_hours", "72"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "personal_data_breach".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("ID")
}

/// UU No. 25 Tahun 2007 tentang Penanaman Modal (Investment Law), Pasal 6 —
/// equal treatment of investors.
///
/// Article 6(1) of the Investment Law guarantees that the Government accords
/// equal treatment to all investors, whether domestic or foreign, originating
/// from any country, who carry out investment activities in Indonesia, subject
/// to the provisions of law. This non-discrimination guarantee underpins the
/// risk-based licensing regime (later reinforced by the Omnibus Law and OSS).
/// Modelled as a Grant of the equal-treatment right to a registered investor.
///
/// Real source: UU No. 25 Tahun 2007 (Penanaman Modal), Pasal 6(1).
#[must_use]
pub fn investment_equal_treatment_statute() -> Statute {
    Statute::new(
        "ID-INVESTMENT-2007",
        "Equal Treatment of Investors (UU 25/2007 Penanaman Modal, Pasal 6)",
        Effect::new(
            EffectType::Grant,
            "The Government accords equal treatment to all investors, domestic and \
             foreign, carrying out investment activities in Indonesia, subject to law",
        )
        .with_parameter("law_number", "25")
        .with_parameter("law_year", "2007")
        .with_parameter("article", "6"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "investor_registered".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("ID")
}

/// UU No. 8 Tahun 1995 tentang Pasar Modal (Capital Markets Law), Pasal 70 —
/// public offering only after an effective registration statement.
///
/// Article 70(1) of the Capital Markets Law prohibits any party from making a
/// public offering of securities unless a registration statement (*pernyataan
/// pendaftaran*) submitted to the supervisory authority has become effective.
/// Supervision originally vested in Bapepam-LK and has since transferred to the
/// Financial Services Authority (Otoritas Jasa Keuangan, OJK). Modelled as the
/// obligation to obtain an effective registration statement before a public
/// offering.
///
/// Real source: UU No. 8 Tahun 1995 (Pasar Modal), Pasal 70.
#[must_use]
pub fn capital_markets_registration_statute() -> Statute {
    Statute::new(
        "ID-PASARMODAL-1995",
        "Registration Before Public Offering (UU 8/1995 Pasar Modal, Pasal 70)",
        Effect::new(
            EffectType::Obligation,
            "A party may make a public offering of securities only after a \
             registration statement filed with the supervisory authority (OJK) has \
             become effective",
        )
        .with_parameter("law_number", "8")
        .with_parameter("law_year", "1995")
        .with_parameter("article", "70")
        .with_parameter("regulator", "OJK"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "public_offering".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("ID")
}

/// UU No. 5 Tahun 1960 tentang Peraturan Dasar Pokok-Pokok Agraria (UUPA, Basic
/// Agrarian Law), Pasal 21 — only Indonesian citizens may hold freehold title.
///
/// Article 21(1) of the Basic Agrarian Law provides that only Indonesian citizens
/// may hold the right of ownership (*Hak Milik*), the strongest and freehold form
/// of land title. A foreigner who acquires Hak Milik (for example by inheritance
/// or mixed marriage without a prenuptial agreement) must relinquish it within one
/// year, failing which the land falls to the State (Pasal 21(3)). Modelled as a
/// prohibition on a non-citizen holding Hak Milik.
///
/// Real source: UU No. 5 Tahun 1960 (UUPA), Pasal 21(1) and 21(3).
#[must_use]
pub fn agrarian_hak_milik_statute() -> Statute {
    Statute::new(
        "ID-UUPA-1960-HAKMILIK",
        "Freehold Title Restricted to Citizens (UU 5/1960 UUPA, Pasal 21)",
        Effect::new(
            EffectType::Prohibition,
            "Only Indonesian citizens may hold Hak Milik (freehold title); a foreign \
             holder must relinquish it within one year or the land falls to the State",
        )
        .with_parameter("law_number", "5")
        .with_parameter("law_year", "1960")
        .with_parameter("article", "21")
        .with_parameter("right_type", "Hak Milik"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "is_indonesian_citizen".to_string(),
        value: "false".to_string(),
    })
    .with_jurisdiction("ID")
}

/// UU No. 7 Tahun 2021 tentang Harmonisasi Peraturan Perpajakan (HPP), amending
/// the VAT Law (UU PPN) — compulsory registration as a Taxable Entrepreneur (PKP)
/// and the value-added tax rate.
///
/// Under the VAT Law as harmonised by the HPP Law, an entrepreneur whose gross
/// turnover from the delivery of taxable goods or services in a financial year
/// exceeds the small-entrepreneur threshold of Rp 4,800,000,000 must register as
/// a Taxable Entrepreneur (*Pengusaha Kena Pajak*, PKP), collect value-added tax
/// (*Pajak Pertambahan Nilai*, PPN) and issue tax invoices. The HPP Law set the
/// standard VAT rate at 11% (effective 1 April 2022). Modelled as the obligation
/// to register as PKP once turnover exceeds the threshold.
///
/// Real source: UU No. 7 Tahun 2021 (HPP), amending UU PPN; PKP threshold per
/// PMK 197/2013.
#[must_use]
pub fn vat_registration_statute() -> Statute {
    Statute::new(
        "ID-PPN-2021",
        "Compulsory PKP Registration for VAT (UU 7/2021 HPP / UU PPN)",
        Effect::new(
            EffectType::Obligation,
            "An entrepreneur whose annual taxable turnover exceeds Rp 4,800,000,000 \
             must register as a Taxable Entrepreneur (PKP) and collect VAT at the \
             standard rate of 11%",
        )
        .with_parameter("law_number", "7")
        .with_parameter("law_year", "2021")
        .with_parameter("tax", "PPN")
        .with_parameter("threshold_idr", "4800000000")
        .with_parameter("standard_rate_pct", "11"),
    )
    .with_precondition(Condition::Income {
        operator: ComparisonOp::GreaterThan,
        value: 4_800_000_000,
    })
    .with_jurisdiction("ID")
}

/// Returns every modelled Indonesian statute.
#[must_use]
pub fn all_statutes() -> Vec<Statute> {
    vec![
        civil_code_contract_validity_statute(),
        company_law_statute(),
        manpower_severance_statute(),
        pdp_breach_notification_statute(),
        investment_equal_treatment_statute(),
        capital_markets_registration_statute(),
        agrarian_hak_milik_statute(),
        vat_registration_statute(),
    ]
}

/// Renders every modelled Indonesian statute as `legalis-dsl` source text.
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
        assert!(!statutes.is_empty(), "ID must model at least one statute");
        assert_eq!(statutes.len(), 8, "ID must model exactly 8 statutes");

        let dsl = statutes_as_dsl();
        assert!(!dsl.is_empty(), "DSL export must not be empty");

        // Every modelled statute's id must appear in the rendered DSL, proving the
        // printer handled each one (covers the range of condition kinds the ID
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
