//! `Statute`-based models of major Lao PDR (Laos) legislation.
//!
//! This module lifts the validators and domain types implemented across the
//! `legalis-la` crate (Civil Code obligations, Enterprise Law, Labour Law, the
//! Investment Promotion Law, Land Law, the Criminal/Penal Code, environmental
//! protection and tax law) into the jurisdiction-neutral
//! [`legalis_core::Statute`] abstraction. Each builder encodes a *real* statutory
//! provision of the Lao People's Democratic Republic — accurate law number, year
//! and operative rule — as an [`Effect`] with a meaningful [`Condition`]
//! precondition where the underlying law turns on a quantifiable trigger (a
//! duration, a monetary threshold expressed as an income figure in Lao Kip, or a
//! status flag).
//!
//! The modelled statutes can be rendered as `legalis-dsl` source text via
//! [`statutes_as_dsl`], so the Lao rule-set can be inspected, diffed, formatted
//! and consumed by the DSL tooling (LSP, documentation generation, structural
//! diffing) on the same footing as every other jurisdiction.
//!
//! # Coverage
//!
//! | Builder | Law |
//! |---------|-----|
//! | [`civil_code_contract_statute`] | Civil Code 2020 (Law No. 66/NA), Book III, Art. 500 |
//! | [`enterprise_law_statute`] | Enterprise Law 2013 (Law No. 46/NA) |
//! | [`labour_law_working_hours_statute`] | Labour Law 2013 (Law No. 43/NA), Art. 51 |
//! | [`investment_promotion_statute`] | Investment Promotion Law 2016 (Law No. 14/NA) |
//! | [`land_law_statute`] | Land Law 2019 (Law No. 70/NA), Art. 3 |
//! | [`penal_code_statute`] | Criminal (Penal) Code 2017 (Law No. 26/NA), Art. 16 |
//! | [`environmental_protection_statute`] | Environmental Protection Law 2012 (Law No. 29/NA), Art. 18 |
//! | [`vat_law_statute`] | Tax Law 2011 (Law No. 05/NA), VAT registration |
//! | [`consumer_protection_labelling_statute`] | Law on Consumer Protection 2010 (Law No. 02/NA) |
//! | [`insurance_compulsory_motor_statute`] | Law on Insurance 2011 (Law No. 06/NA) |
//! | [`telecommunications_licence_statute`] | Law on Telecommunications 2011 (Law No. 09/NA) |
//! | [`construction_permit_statute`] | Law on Construction 2009 (Law No. 05/NA) |
//! | [`securities_public_offering_statute`] | Law on Securities 2012 |
//! | [`intellectual_property_patent_statute`] | Law on Intellectual Property 2017 (Law No. 38/NA) |
//!
//! # Disclaimer
//!
//! These models are simplified abstractions for computational reasoning and are
//! provided for educational and informational purposes only. They are not legal
//! advice; consult a qualified Lao legal practitioner.

use crate::construction_law::DEFECTS_LIABILITY_PERIOD_MONTHS;
use crate::consumer_protection_law::REQUIRED_LABEL_LANGUAGE;
use crate::intellectual_property_law::PATENT_TERM_YEARS;
use crate::securities_law::MIN_PUBLIC_FLOAT_PERCENT;
use legalis_core::{ComparisonOp, Condition, DurationUnit, Effect, EffectType, Statute};

/// Civil Code 2020 (Law No. 66/NA), Book III "Obligations", Article 500 —
/// formation of a contract.
///
/// Article 500 provides that a contract is formed when an offer is met by
/// acceptance. Under the surrounding Book III provisions a valid contract
/// additionally requires a lawful purpose, the verified legal capacity of the
/// parties, and freely given consent. The Lao Civil Code (effective 9 July 2021)
/// was developed with Japanese (JICA) legal assistance and follows the structure
/// of the Japanese saiken-hō (債権法). Modelled here as a Grant: once the parties
/// have legal capacity and a valid offer is accepted, the contract takes legal
/// effect and binds them.
///
/// Real source: Lao Civil Code 2020 (Law No. 66/NA), Art. 500.
#[must_use]
pub fn civil_code_contract_statute() -> Statute {
    Statute::new(
        "LA-CIVIL-CODE-2020-ART500",
        "Formation of Contract (Civil Code 2020, Law No. 66/NA, Art. 500)",
        Effect::new(
            EffectType::Grant,
            "A contract is formed and legally binding when a valid offer is met by \
             acceptance, the purpose is lawful and the parties have capacity and \
             freely given consent",
        )
        .with_parameter("law_number", "66/NA")
        .with_parameter("law_year", "2020")
        .with_parameter("book", "III")
        .with_parameter("article", "500"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "capacity_verified".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("LA")
}

/// Enterprise Law 2013 (Law No. 46/NA) — minimum registered capital for a
/// limited company.
///
/// The Enterprise Law 2013 (effective 21 June 2014) recognises five forms of
/// business entity and governs their registration. A private limited company
/// (ບໍລິສັດຈໍາກັດ) must be incorporated with a minimum registered capital of
/// 50,000,000 LAK, of which at least 30% must be paid up. Modelled here as the
/// obligation to maintain that minimum registered capital when a limited company
/// is registered.
///
/// Real source: Enterprise Law 2013 (Law No. 46/NA).
#[must_use]
pub fn enterprise_law_statute() -> Statute {
    Statute::new(
        "LA-ENTERPRISE-2013",
        "Limited Company Minimum Capital (Enterprise Law 2013, Law No. 46/NA)",
        Effect::new(
            EffectType::Obligation,
            "A limited company must be registered with at least 50,000,000 LAK of \
             registered capital, of which a minimum of 30% must be paid up",
        )
        .with_parameter("law_number", "46/NA")
        .with_parameter("law_year", "2013")
        .with_parameter("min_registered_capital_lak", "50000000")
        .with_parameter("min_paid_up_ratio_pct", "30"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "entity_type".to_string(),
        value: "limited_company".to_string(),
    })
    .with_jurisdiction("LA")
}

/// Labour Law 2013 (Law No. 43/NA), Article 51 — statutory working hours.
///
/// Article 51 sets the statutory maximum working time at 8 hours per day and 48
/// hours per week, over a maximum of 6 working days per week. Overtime beyond
/// these limits is regulated separately (Article 52) and attracts premium pay
/// (Article 53). Modelled here as a Prohibition on scheduling ordinary working
/// time in excess of 48 hours per week.
///
/// Real source: Lao Labour Law 2013 (Law No. 43/NA), Art. 51.
#[must_use]
pub fn labour_law_working_hours_statute() -> Statute {
    Statute::new(
        "LA-LABOUR-2013-ART51",
        "Maximum Working Hours (Labour Law 2013, Law No. 43/NA, Art. 51)",
        Effect::new(
            EffectType::Prohibition,
            "Ordinary working time must not exceed 8 hours per day or 48 hours per \
             week, across a maximum of 6 working days per week",
        )
        .with_parameter("law_number", "43/NA")
        .with_parameter("law_year", "2013")
        .with_parameter("article", "51")
        .with_parameter("max_hours_per_day", "8")
        .with_parameter("max_hours_per_week", "48")
        .with_parameter("max_days_per_week", "6"),
    )
    .with_precondition(Condition::Duration {
        operator: ComparisonOp::GreaterThan,
        value: 48,
        unit: DurationUnit::Weeks,
    })
    .with_jurisdiction("LA")
}

/// Investment Promotion Law 2016 (Law No. 14/NA) — profit-tax incentives for
/// promoted investment.
///
/// The Investment Promotion Law 2016 (amended 2017) consolidates Lao investment
/// regulation and grants incentives to investment in promoted sectors and zones,
/// including profit (corporate income) tax exemptions for a defined number of
/// years and exemptions from import duties on capital goods. Modelled here as a
/// Grant of profit-tax exemption to a registered investment that has been
/// approved as a promoted project.
///
/// Real source: Investment Promotion Law 2016 (Law No. 14/NA, amended 2017).
#[must_use]
pub fn investment_promotion_statute() -> Statute {
    Statute::new(
        "LA-INVESTMENT-2016",
        "Promoted Investment Tax Incentives (Investment Promotion Law 2016, Law No. 14/NA)",
        Effect::new(
            EffectType::Grant,
            "An approved promoted investment is entitled to profit (corporate income) \
             tax exemption for the prescribed period and exemption from import duties \
             on capital goods",
        )
        .with_parameter("law_number", "14/NA")
        .with_parameter("law_year", "2016")
        .with_parameter("amended_year", "2017")
        .with_parameter("incentive", "profit_tax_exemption"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "promoted_investment_approved".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("LA")
}

/// Land Law 2019 (Law No. 70/NA), Article 3 — state ownership of land and land
/// use rights.
///
/// Article 3 establishes the foundational principle of Lao land law: all land is
/// the property of the national community under state management. Individuals and
/// organisations do not own land in fee simple; they instead hold land *use
/// rights* (ສິດນຳໃຊ້ທີ່ດິນ). A perpetual use right is available only to Lao
/// citizens and domestic legal entities, while foreigners may obtain only
/// temporary use rights. Modelled here as a Grant of a land use right (rather than
/// ownership) to a holder allocated land by the state.
///
/// Real source: Lao Land Law 2019 (Law No. 70/NA), Art. 3.
#[must_use]
pub fn land_law_statute() -> Statute {
    Statute::new(
        "LA-LAND-2019-ART3",
        "State Land Ownership and Use Rights (Land Law 2019, Law No. 70/NA, Art. 3)",
        Effect::new(
            EffectType::Grant,
            "All land is the property of the national community under state management; \
             holders are allocated land use rights, not ownership, with perpetual use \
             rights reserved to Lao citizens and domestic legal entities",
        )
        .with_parameter("law_number", "70/NA")
        .with_parameter("law_year", "2019")
        .with_parameter("article", "3"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "land_allocated_by_state".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("LA")
}

/// Criminal (Penal) Code 2017 (Law No. 26/NA), Article 16 — age of criminal
/// responsibility.
///
/// Article 16 fixes the general age of criminal responsibility at 16 years, with
/// a lower threshold of 14 years for serious crimes. The Criminal Code 2017
/// (effective 27 May 2018) is the primary source of Lao criminal law. Modelled
/// here as a StatusChange: a person attains criminal responsibility for ordinary
/// offences upon reaching 16 years of age.
///
/// Real source: Lao Criminal (Penal) Code 2017 (Law No. 26/NA), Art. 16.
#[must_use]
pub fn penal_code_statute() -> Statute {
    Statute::new(
        "LA-PENAL-CODE-2017-ART16",
        "Age of Criminal Responsibility (Criminal Code 2017, Law No. 26/NA, Art. 16)",
        Effect::new(
            EffectType::StatusChange,
            "A person bears criminal responsibility for ordinary offences upon reaching \
             16 years of age, with a lower threshold of 14 years for serious crimes",
        )
        .with_parameter("law_number", "26/NA")
        .with_parameter("law_year", "2017")
        .with_parameter("article", "16")
        .with_parameter("age_general", "16")
        .with_parameter("age_serious_crimes", "14"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 16,
    })
    .with_jurisdiction("LA")
}

/// Environmental Protection Law 2012 (Law No. 29/NA), Article 18 — Environmental
/// Impact Assessment requirement.
///
/// Article 18 requires that projects meeting prescribed type, scale or location
/// criteria undergo an Environmental Impact Assessment (EIA) before approval.
/// Large-scale (Category A) projects — such as mining, hydropower above 15 MW and
/// major infrastructure — require a full EIA, while medium-scale (Category B)
/// projects require an Initial Environmental Examination. Modelled here as the
/// obligation of a Category A project developer to complete a full EIA.
///
/// Real source: Lao Environmental Protection Law 2012 (Law No. 29/NA), Art. 18.
#[must_use]
pub fn environmental_protection_statute() -> Statute {
    Statute::new(
        "LA-ENVIRONMENT-2012-ART18",
        "Environmental Impact Assessment (Environmental Protection Law 2012, Law No. 29/NA, Art. 18)",
        Effect::new(
            EffectType::Obligation,
            "A large-scale (Category A) project must complete a full Environmental \
             Impact Assessment, including public consultation, before it may be approved",
        )
        .with_parameter("law_number", "29/NA")
        .with_parameter("law_year", "2012")
        .with_parameter("article", "18")
        .with_parameter("eia_category", "A"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "eia_category".to_string(),
        value: "A".to_string(),
    })
    .with_jurisdiction("LA")
}

/// Tax Law 2011 (Law No. 05/NA) — compulsory Value Added Tax registration.
///
/// Under the Lao VAT regime a person carrying on a business whose annual turnover
/// reaches or exceeds the registration threshold of 400,000,000 LAK must register
/// for VAT and charge VAT at the standard rate of 10%. Exports are zero-rated and
/// certain supplies (financial services, education, healthcare, agriculture) are
/// exempt. Modelled here as the obligation to register for VAT once the turnover
/// threshold is met.
///
/// Real source: Lao Tax Law 2011 (Law No. 05/NA, effective 20 October 2011), VAT
/// provisions.
#[must_use]
pub fn vat_law_statute() -> Statute {
    Statute::new(
        "LA-VAT-2011",
        "Compulsory VAT Registration (Tax Law 2011, Law No. 05/NA)",
        Effect::new(
            EffectType::Obligation,
            "A business whose annual turnover reaches or exceeds 400,000,000 LAK must \
             register for VAT and charge VAT at the standard rate of 10%",
        )
        .with_parameter("law_number", "05/NA")
        .with_parameter("law_year", "2011")
        .with_parameter("threshold_lak", "400000000")
        .with_parameter("standard_rate_pct", "10"),
    )
    .with_precondition(Condition::Income {
        operator: ComparisonOp::GreaterOrEqual,
        value: 400_000_000,
    })
    .with_jurisdiction("LA")
}

/// Law on Consumer Protection 2010 (Law No. 02/NA) — mandatory Lao-language
/// product labelling.
///
/// The Law on Consumer Protection (No. 02/NA, 30 June 2010) protects the
/// fundamental rights of consumers, including the right to be informed. Goods
/// supplied to consumers in the Lao PDR must be labelled in the Lao language so
/// that consumers receive accurate, intelligible product information. Modelled
/// here as an Obligation to provide Lao-language labelling whenever a product is
/// offered to consumers.
///
/// Real source: Lao Law on Consumer Protection 2010 (Law No. 02/NA).
#[must_use]
pub fn consumer_protection_labelling_statute() -> Statute {
    Statute::new(
        "LA-CONSUMER-PROTECTION-2010",
        "Mandatory Lao-Language Labelling (Law on Consumer Protection 2010, Law No. 02/NA)",
        Effect::new(
            EffectType::Obligation,
            "Goods offered to consumers must be labelled in the Lao language, \
             disclosing accurate product information, in accordance with the \
             consumer's right to be informed",
        )
        .with_parameter("law_number", "02/NA")
        .with_parameter("law_year", "2010")
        .with_parameter("required_label_language", REQUIRED_LABEL_LANGUAGE),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "product_offered_to_consumers".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("LA")
}

/// Law on Insurance 2011 (Law No. 06/NA) — compulsory motor third-party liability
/// insurance.
///
/// The Law on Insurance (No. 06/NA, 2011), administered by the Ministry of
/// Finance, governs insurers, insurance contracts and intermediaries. Motor
/// vehicle third-party liability insurance is compulsory so that road-accident
/// victims are assured of compensation. Modelled here as an Obligation to hold
/// third-party liability cover when a motor vehicle is operated on public roads.
///
/// Real source: Lao Law on Insurance 2011 (Law No. 06/NA).
#[must_use]
pub fn insurance_compulsory_motor_statute() -> Statute {
    Statute::new(
        "LA-INSURANCE-2011",
        "Compulsory Motor Third-Party Liability Insurance (Law on Insurance 2011, Law No. 06/NA)",
        Effect::new(
            EffectType::Obligation,
            "The operator of a motor vehicle on public roads must hold valid \
             third-party liability insurance",
        )
        .with_parameter("law_number", "06/NA")
        .with_parameter("law_year", "2011")
        .with_parameter("motor_third_party_compulsory", "true"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "vehicle_operated_on_public_road".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("LA")
}

/// Law on Telecommunications 2011 (Law No. 09/NA) — telecommunications service
/// licence requirement.
///
/// The Law on Telecommunications (No. 09/NA, 2011) regulates the provision of
/// telecommunications services and the use of the radio-frequency spectrum, a
/// scarce national resource. A person may not provide telecommunications services
/// to the public without a licence from the regulator. Modelled here as an
/// Obligation to hold a telecommunications licence when such services are provided.
///
/// Real source: Lao Law on Telecommunications 2011 (Law No. 09/NA).
#[must_use]
pub fn telecommunications_licence_statute() -> Statute {
    Statute::new(
        "LA-TELECOM-2011",
        "Telecommunications Service Licence (Law on Telecommunications 2011, Law No. 09/NA)",
        Effect::new(
            EffectType::Obligation,
            "A person providing telecommunications services to the public must hold a \
             licence from the telecommunications regulator and use only assigned \
             radio-frequency spectrum",
        )
        .with_parameter("law_number", "09/NA")
        .with_parameter("law_year", "2011"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "telecom_service_provided".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("LA")
}

/// Law on Construction 2009 (Law No. 05/NA) — building-permit requirement.
///
/// The Law on Construction (No. 05/NA, 2009) governs construction activity,
/// contractor licensing, technical and safety standards and the defects-liability
/// of completed works. A construction permit must be obtained before construction
/// may lawfully commence. Modelled here as an Obligation to hold a building permit
/// once construction is commenced.
///
/// Real source: Lao Law on Construction 2009 (Law No. 05/NA).
#[must_use]
pub fn construction_permit_statute() -> Statute {
    Statute::new(
        "LA-CONSTRUCTION-2009",
        "Building Permit Requirement (Law on Construction 2009, Law No. 05/NA)",
        Effect::new(
            EffectType::Obligation,
            "Construction works require a building permit issued by the competent \
             authority before construction may commence",
        )
        .with_parameter("law_number", "05/NA")
        .with_parameter("law_year", "2009")
        .with_parameter(
            "defects_liability_months",
            DEFECTS_LIABILITY_PERIOD_MONTHS.to_string(),
        ),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "construction_commenced".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("LA")
}

/// Law on Securities 2012 — prospectus and disclosure for a public offering.
///
/// The Law on Securities (2012) established the framework for the Lao securities
/// market — the Lao Securities Exchange (LSX) and its regulator, the Lao
/// Securities and Exchange Commission. A public offering of securities requires an
/// approved prospectus making full and accurate disclosure to investors. Modelled
/// here as an Obligation to publish an approved prospectus when securities are
/// offered to the public.
///
/// Real source: Lao Law on Securities 2012.
#[must_use]
pub fn securities_public_offering_statute() -> Statute {
    Statute::new(
        "LA-SECURITIES-2012",
        "Public Offering Prospectus and Disclosure (Law on Securities 2012)",
        Effect::new(
            EffectType::Obligation,
            "A public offering of securities requires a prospectus approved by the \
             securities regulator with full and accurate disclosure to investors",
        )
        .with_parameter("law_year", "2012")
        .with_parameter("min_public_float_pct", MIN_PUBLIC_FLOAT_PERCENT.to_string()),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "public_offering".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("LA")
}

/// Law on Intellectual Property 2017 (Law No. 38/NA) — patent term of protection.
///
/// The consolidated Law on Intellectual Property (No. 38/NA, 2017) protects
/// patents, trademarks, copyright, industrial designs, geographical indications,
/// trade secrets, layout-designs and plant varieties. Consistent with the TRIPS
/// Agreement, a granted patent confers exclusive rights for 20 years from the
/// filing date. Modelled here as a Grant of exclusive rights to the holder of a
/// granted patent.
///
/// Real source: Lao Law on Intellectual Property 2017 (Law No. 38/NA).
#[must_use]
pub fn intellectual_property_patent_statute() -> Statute {
    Statute::new(
        "LA-IP-2017-PATENT",
        "Patent Term of Protection (Law on Intellectual Property 2017, Law No. 38/NA)",
        Effect::new(
            EffectType::Grant,
            "A granted patent confers on its holder the exclusive right to exploit the \
             invention for 20 years from the filing date",
        )
        .with_parameter("law_number", "38/NA")
        .with_parameter("law_year", "2017")
        .with_parameter("patent_term_years", PATENT_TERM_YEARS.to_string()),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "patent_granted".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("LA")
}

/// Returns every modelled Lao PDR statute.
#[must_use]
pub fn all_statutes() -> Vec<Statute> {
    vec![
        civil_code_contract_statute(),
        enterprise_law_statute(),
        labour_law_working_hours_statute(),
        investment_promotion_statute(),
        land_law_statute(),
        penal_code_statute(),
        environmental_protection_statute(),
        vat_law_statute(),
        consumer_protection_labelling_statute(),
        insurance_compulsory_motor_statute(),
        telecommunications_licence_statute(),
        construction_permit_statute(),
        securities_public_offering_statute(),
        intellectual_property_patent_statute(),
    ]
}

/// Renders every modelled Lao PDR statute as `legalis-dsl` source text.
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
        assert!(!statutes.is_empty(), "LA must model at least one statute");
        assert_eq!(statutes.len(), 14, "LA must model exactly 14 statutes");

        let dsl = statutes_as_dsl();
        assert!(!dsl.is_empty(), "DSL export must not be empty");

        // Every modelled statute's id must appear in the rendered DSL, proving the
        // printer handled each one (covers the range of condition kinds the LA
        // adapters use: Age, Income, Duration, AttributeEquals).
        for statute in &statutes {
            assert!(
                dsl.contains(statute.id.as_str()),
                "statute {} missing from DSL export",
                statute.id
            );
        }
    }
}
