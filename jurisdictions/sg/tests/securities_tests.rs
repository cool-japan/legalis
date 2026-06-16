//! Integration tests for the Singapore Securities and Futures Act module.
//!
//! These exercise the public API of [`legalis_sg::securities`] across the four
//! pillars - capital markets products, offers and the prospectus regime (Part
//! 13), market conduct (Part 12) and licensing (Part 4) - through realistic
//! Singapore capital-markets scenarios.

use legalis_sg::securities::*;

/// A retail IPO of shares with no exemption and no prospectus must be stopped:
/// a registered prospectus is required (SFA s. 240).
#[test]
fn retail_ipo_requires_registered_prospectus() {
    let ipo = SecuritiesOffering::new(
        "ipo-2026-1",
        CapitalMarketsProduct::Securities,
        10_000_000_000,
    );
    match assess_prospectus_requirement(&ipo) {
        Err(SecuritiesError::ProspectusRequired { .. }) => {}
        other => panic!("expected ProspectusRequired, got {other:?}"),
    }

    let with_prospectus = ipo.with_prospectus(Prospectus::registered());
    assert!(assess_prospectus_requirement(&with_prospectus).is_ok());
}

/// A private placement to no more than 50 persons is exempt (SFA s. 272B); 51
/// persons is not.
#[test]
fn private_placement_fifty_person_limit() {
    let ok = SecuritiesOffering::new("pp-1", CapitalMarketsProduct::Securities, 20_000_000_000)
        .with_offerees(50)
        .with_exemption(OfferingExemption::PrivatePlacement);
    assert!(assess_prospectus_requirement(&ok).is_ok());

    let too_many =
        SecuritiesOffering::new("pp-2", CapitalMarketsProduct::Securities, 20_000_000_000)
            .with_offerees(51)
            .with_exemption(OfferingExemption::PrivatePlacement);
    assert!(assess_prospectus_requirement(&too_many).is_err());
}

/// The small-offers exemption caps the amount raised in any 12-month period at
/// SGD 5 million (SFA s. 272A).
#[test]
fn small_offer_five_million_ceiling() {
    let within = SecuritiesOffering::new("so-1", CapitalMarketsProduct::Securities, 500_000_000)
        .with_exemption(OfferingExemption::SmallOffer);
    assert!(assess_prospectus_requirement(&within).is_ok());

    let over = SecuritiesOffering::new("so-2", CapitalMarketsProduct::Securities, 500_000_001)
        .with_exemption(OfferingExemption::SmallOffer);
    assert!(assess_prospectus_requirement(&over).is_err());
}

/// An offer made only to accredited investors is exempt (SFA s. 275).
#[test]
fn accredited_investor_exemption() {
    let offering =
        SecuritiesOffering::new("ai-1", CapitalMarketsProduct::Securities, 30_000_000_000)
            .with_offeree_class(InvestorClass::Accredited)
            .with_exemption(OfferingExemption::AccreditedInvestors);
    let report = assess_offering_report(&offering);
    assert!(report.compliant);
    assert!(report.exemption_made_out);
    assert!(!report.prospectus_required);
}

/// An individual qualifies as an accredited investor on the wealth/income tests
/// (SFA s. 4A).
#[test]
fn accredited_investor_thresholds() {
    // Income of SGD 350,000 in the last 12 months qualifies.
    assert!(is_accredited_individual(0, 0, 0, 35_000_000));
    // Net financial assets of SGD 1.2m qualifies.
    assert!(is_accredited_individual(120_000_000, 0, 0, 0));
    // A modest profile does not qualify.
    assert!(!is_accredited_individual(20_000_000, 0, 0, 12_000_000));
    // A corporation with net assets over SGD 10m qualifies.
    assert!(is_accredited_corporation(1_500_000_000));
}

/// Insider trading by a connected director engages s. 218; by a tippee, s. 219.
#[test]
fn insider_trading_sections() {
    let director = InsiderTradingClaim::new("it-dir", true);
    match assess_insider_trading(&director) {
        Err(SecuritiesError::InsiderTrading { section, .. }) => assert_eq!(section, "s. 218"),
        other => panic!("expected InsiderTrading, got {other:?}"),
    }

    let tippee =
        InsiderTradingClaim::new("it-tip", false).with_conduct(InsiderConduct::Communicated);
    match assess_insider_trading(&tippee) {
        Err(SecuritiesError::InsiderTrading { section, .. }) => assert_eq!(section, "s. 219"),
        other => panic!("expected InsiderTrading, got {other:?}"),
    }

    // Information that is generally available is not inside information.
    let public = InsiderTradingClaim::new("it-pub", true).generally_available();
    assert!(assess_insider_trading(&public).is_ok());
}

/// False trading via wash trades (SFA s. 197) and manipulation (SFA s. 201).
#[test]
fn false_trading_and_manipulation() {
    let wash = FalseTradingClaim::new("ft-1").with_wash_trade();
    assert!(assess_false_trading(&wash).is_err());

    let manip = MarketManipulationClaim::new("mm-1");
    assert!(assess_market_manipulation(&manip).is_err());
}

/// A consolidated market-conduct report collects multiple contraventions.
#[test]
fn market_conduct_report_aggregates() {
    let insider = InsiderTradingClaim::new("it-x", true);
    let false_trade = FalseTradingClaim::new("ft-x").with_wash_trade();
    let misleading = MisleadingStatementClaim::new("ms-x");
    let report = assess_market_conduct(
        Some(&insider),
        Some(&false_trade),
        None,
        Some(&misleading),
        None,
    );
    assert!(report.is_market_abuse);
    assert_eq!(report.contraventions.len(), 3);
}

/// Carrying on a regulated activity requires a CMS licence authorising it
/// (SFA s. 82).
#[test]
fn licensing_requires_cms_licence() {
    let licence = CapitalMarketsServicesLicence::new(
        "Alpha Capital Pte Ltd",
        vec![RegulatedActivity::FundManagement],
    );
    assert!(assess_licensing(Some(&licence), RegulatedActivity::FundManagement).is_ok());
    assert!(
        assess_licensing(
            Some(&licence),
            RegulatedActivity::AdvisingOnCorporateFinance
        )
        .is_err()
    );
    assert!(assess_licensing(None, RegulatedActivity::DealingInCapitalMarketsProducts).is_err());
}

/// A representative must be on the MAS public register (SFA s. 99B).
#[test]
fn representative_must_be_on_register() {
    let rep = AppointedRepresentative::new(
        "Jane Tan",
        "Alpha Capital Pte Ltd",
        vec![RegulatedActivity::FundManagement],
    );
    assert!(assess_representative(&rep, RegulatedActivity::FundManagement).is_ok());

    let off = rep.not_on_register();
    assert!(assess_representative(&off, RegulatedActivity::FundManagement).is_err());
}

/// A collective investment scheme offered to the public must be authorised or
/// recognised (SFA s. 286/s. 287).
#[test]
fn cis_public_offer_needs_authorisation() {
    let restricted =
        CollectiveInvestmentScheme::new("PE Fund LP", false, CisAuthorisationStatus::Restricted);
    assert!(assess_collective_investment_scheme(&restricted, true).is_err());
    assert!(assess_collective_investment_scheme(&restricted, false).is_ok());

    let recognised = CollectiveInvestmentScheme::new(
        "Global Bond Fund",
        false,
        CisAuthorisationStatus::Recognised,
    );
    assert!(assess_collective_investment_scheme(&recognised, true).is_ok());
}

/// The civil penalty is capped at three times the profit gained (SFA s. 232).
#[test]
fn civil_penalty_cap() {
    // Individual, profit SGD 2m -> cap SGD 6m; propose SGD 5m -> within cap.
    let payable = compute_civil_penalty_cents(200_000_000, true, 500_000_000).expect("within cap");
    assert_eq!(payable, 500_000_000);

    // Propose SGD 7m -> exceeds the SGD 6m cap.
    match compute_civil_penalty_cents(200_000_000, true, 700_000_000) {
        Err(SecuritiesError::CivilPenaltyExceedsCap { maximum_cents, .. }) => {
            assert_eq!(maximum_cents, 600_000_000);
        }
        other => panic!("expected CivilPenaltyExceedsCap, got {other:?}"),
    }
}

/// Serialization roundtrips for the aggregate offering type.
#[test]
fn offering_serde_roundtrip() {
    let offering =
        SecuritiesOffering::new("of-s", CapitalMarketsProduct::Securities, 2_500_000_000)
            .with_offerees(25)
            .with_offeree_class(InvestorClass::Accredited)
            .with_exemption(OfferingExemption::AccreditedInvestors)
            .with_prospectus(Prospectus::registered());

    let json = serde_json::to_string(&offering).expect("serialize");
    let restored: SecuritiesOffering = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(offering, restored);
}

/// Serialization roundtrips for the CMS licence aggregate.
#[test]
fn licence_serde_roundtrip() {
    let licence = CapitalMarketsServicesLicence::new(
        "Beta Securities Pte Ltd",
        vec![
            RegulatedActivity::DealingInCapitalMarketsProducts,
            RegulatedActivity::ProvidingCustodialServices,
        ],
    )
    .with_status(CmsLicenceStatus::Granted);

    let json = serde_json::to_string(&licence).expect("serialize");
    let restored: CapitalMarketsServicesLicence = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(licence, restored);
}

/// A typical prospectus assessment completes well under 1 ms.
#[test]
fn prospectus_assessment_is_fast() {
    use std::time::Instant;

    let offering =
        SecuritiesOffering::new("perf", CapitalMarketsProduct::Securities, 10_000_000_000)
            .with_offerees(30)
            .with_offeree_class(InvestorClass::Accredited)
            .with_exemption(OfferingExemption::AccreditedInvestors);

    let iterations = 10_000;
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = assess_prospectus_requirement(&offering);
    }
    let per_call = start.elapsed() / iterations;
    assert!(
        per_call.as_micros() < 1_000,
        "assessment too slow: {per_call:?} per call"
    );
}
