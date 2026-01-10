//! Integration tests for HGB (German Commercial Code) partnership validation
//!
//! Comprehensive tests covering:
//! - OHG (General Partnership) validation
//! - KG (Limited Partnership) validation
//! - GmbH & Co. KG validation
//! - Partnership name validation
//! - Partner validation
//! - Business purpose and registered office validation

use chrono::Utc;
use legalis_de::gmbhg::Capital;
use legalis_de::hgb::*;

// =============================================================================
// Helper Functions
// =============================================================================

fn create_test_partner(name: &str) -> Partner {
    Partner {
        name: name.to_string(),
        address: "Berlin".to_string(),
        contribution: Some(Capital::from_euros(10_000)),
        contribution_paid: Some(Capital::from_euros(10_000)),
        partner_type: PartnerType::NaturalPerson,
        has_management_authority: true,
        has_representation_authority: true,
    }
}

fn create_test_limited_partner(name: &str, liability: u32) -> LimitedPartner {
    LimitedPartner {
        name: name.to_string(),
        address: "Hamburg".to_string(),
        liability_limit: Capital::from_euros(liability as u64),
        contribution_paid: Capital::from_euros(liability as u64),
        partner_type: PartnerType::NaturalPerson,
        has_special_representation: false,
    }
}

fn create_test_ohg() -> OHG {
    OHG {
        partnership_name: "Mustermann & Schmidt OHG".to_string(),
        registered_office: "Berlin".to_string(),
        business_purpose: "Softwareentwicklung und IT-Beratung".to_string(),
        partners: vec![
            create_test_partner("Max Mustermann"),
            create_test_partner("Erika Schmidt"),
        ],
        formation_date: Some(Utc::now()),
        fiscal_year_end: Some(FiscalYearEnd { month: 12, day: 31 }),
        unlimited_liability: true,
    }
}

fn create_test_kg() -> KG {
    KG {
        partnership_name: "Tech Ventures KG".to_string(),
        registered_office: "München".to_string(),
        business_purpose: "IT-Beratung und Softwareentwicklung".to_string(),
        general_partners: vec![create_test_partner("Max Mustermann")],
        limited_partners: vec![create_test_limited_partner("Anna Schmidt", 50_000)],
        formation_date: Some(Utc::now()),
        fiscal_year_end: Some(FiscalYearEnd { month: 12, day: 31 }),
    }
}

fn create_test_gmbh_co_kg() -> GmbHCoKG {
    GmbHCoKG {
        partnership_name: "Verwaltungs GmbH & Co. KG".to_string(),
        registered_office: "Berlin".to_string(),
        business_purpose: "Vermögensverwaltung und Beteiligungen".to_string(),
        gmbh_general_partner: GmbHPartner {
            company_name: "Verwaltungs GmbH".to_string(),
            registered_office: "Berlin".to_string(),
            managing_directors: vec!["Max Mustermann".to_string()],
            share_capital: Capital::from_euros(25_000),
        },
        limited_partners: vec![create_test_limited_partner("Anna Schmidt", 100_000)],
        formation_date: Some(Utc::now()),
        fiscal_year_end: Some(FiscalYearEnd { month: 12, day: 31 }),
    }
}

// =============================================================================
// OHG Validation Tests
// =============================================================================

#[test]
fn test_ohg_valid_standard() {
    let ohg = create_test_ohg();
    assert!(validate_ohg(&ohg).is_ok());
}

#[test]
fn test_ohg_valid_three_partners() {
    let mut ohg = create_test_ohg();
    ohg.partners.push(create_test_partner("Peter Müller"));
    assert!(validate_ohg(&ohg).is_ok());
}

#[test]
fn test_ohg_valid_full_partnership_name() {
    let mut ohg = create_test_ohg();
    ohg.partnership_name = "Mustermann Offene Handelsgesellschaft".to_string();
    assert!(validate_ohg(&ohg).is_ok());
}

#[test]
fn test_ohg_invalid_insufficient_partners() {
    let mut ohg = create_test_ohg();
    ohg.partners.truncate(1);
    let result = validate_ohg(&ohg);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        HGBError::OHGInsufficientPartners
    ));
}

#[test]
fn test_ohg_invalid_no_partners() {
    let mut ohg = create_test_ohg();
    ohg.partners.clear();
    assert!(validate_ohg(&ohg).is_err());
}

#[test]
fn test_ohg_invalid_limited_liability() {
    let mut ohg = create_test_ohg();
    ohg.unlimited_liability = false;
    let result = validate_ohg(&ohg);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        HGBError::OHGLimitedLiabilityNotAllowed
    ));
}

#[test]
fn test_ohg_invalid_empty_partnership_name() {
    let mut ohg = create_test_ohg();
    ohg.partnership_name = "".to_string();
    let result = validate_ohg(&ohg);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        HGBError::EmptyPartnershipName
    ));
}

#[test]
fn test_ohg_invalid_missing_suffix() {
    let mut ohg = create_test_ohg();
    ohg.partnership_name = "Mustermann & Schmidt".to_string();
    let result = validate_ohg(&ohg);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        HGBError::MissingLegalFormSuffix { .. }
    ));
}

#[test]
fn test_ohg_invalid_empty_business_purpose() {
    let mut ohg = create_test_ohg();
    ohg.business_purpose = "".to_string();
    let result = validate_ohg(&ohg);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        HGBError::EmptyBusinessPurpose
    ));
}

#[test]
fn test_ohg_invalid_business_purpose_too_short() {
    let mut ohg = create_test_ohg();
    ohg.business_purpose = "Software".to_string();
    let result = validate_ohg(&ohg);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        HGBError::InvalidBusinessPurpose
    ));
}

#[test]
fn test_ohg_invalid_fiscal_year_end() {
    let mut ohg = create_test_ohg();
    ohg.fiscal_year_end = Some(FiscalYearEnd { month: 13, day: 31 });
    let result = validate_ohg(&ohg);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        HGBError::InvalidFiscalYearEnd { .. }
    ));
}

// =============================================================================
// KG Validation Tests
// =============================================================================

#[test]
fn test_kg_valid_standard() {
    let kg = create_test_kg();
    assert!(validate_kg(&kg).is_ok());
}

#[test]
fn test_kg_valid_multiple_general_partners() {
    let mut kg = create_test_kg();
    kg.general_partners
        .push(create_test_partner("Peter Müller"));
    assert!(validate_kg(&kg).is_ok());
}

#[test]
fn test_kg_valid_multiple_limited_partners() {
    let mut kg = create_test_kg();
    kg.limited_partners
        .push(create_test_limited_partner("Peter Weber", 30_000));
    kg.limited_partners
        .push(create_test_limited_partner("Lisa Becker", 20_000));
    assert!(validate_kg(&kg).is_ok());
}

#[test]
fn test_kg_valid_full_partnership_name() {
    let mut kg = create_test_kg();
    kg.partnership_name = "Tech Ventures Kommanditgesellschaft".to_string();
    assert!(validate_kg(&kg).is_ok());
}

#[test]
fn test_kg_invalid_no_general_partner() {
    let mut kg = create_test_kg();
    kg.general_partners.clear();
    let result = validate_kg(&kg);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), HGBError::KGNoGeneralPartner));
}

#[test]
fn test_kg_invalid_no_limited_partner() {
    let mut kg = create_test_kg();
    kg.limited_partners.clear();
    let result = validate_kg(&kg);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), HGBError::KGNoLimitedPartner));
}

#[test]
fn test_kg_invalid_missing_suffix() {
    let mut kg = create_test_kg();
    kg.partnership_name = "Tech Ventures".to_string();
    let result = validate_kg(&kg);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        HGBError::MissingLegalFormSuffix { .. }
    ));
}

#[test]
fn test_kg_invalid_zero_liability_limit() {
    let mut kg = create_test_kg();
    kg.limited_partners[0].liability_limit = Capital::from_euros(0);
    kg.limited_partners[0].contribution_paid = Capital::from_euros(0);
    let result = validate_kg(&kg);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        HGBError::ZeroLiabilityLimit { .. }
    ));
}

#[test]
fn test_kg_invalid_liability_limit_too_low() {
    let mut kg = create_test_kg();
    kg.limited_partners[0].liability_limit = Capital {
        amount_cents: 50, // €0.50
    };
    kg.limited_partners[0].contribution_paid = Capital { amount_cents: 50 };
    let result = validate_kg(&kg);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        HGBError::LiabilityLimitTooLow { .. }
    ));
}

#[test]
fn test_kg_invalid_contribution_exceeds_liability_limit() {
    let mut kg = create_test_kg();
    kg.limited_partners[0].liability_limit = Capital::from_euros(50_000);
    kg.limited_partners[0].contribution_paid = Capital::from_euros(60_000);
    let result = validate_kg(&kg);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        HGBError::ContributionExceedsLiabilityLimit { .. }
    ));
}

// =============================================================================
// GmbH & Co. KG Validation Tests
// =============================================================================

#[test]
fn test_gmbh_co_kg_valid_standard() {
    let gmbh_co_kg = create_test_gmbh_co_kg();
    assert!(validate_gmbh_co_kg(&gmbh_co_kg).is_ok());
}

#[test]
fn test_gmbh_co_kg_valid_multiple_limited_partners() {
    let mut gmbh_co_kg = create_test_gmbh_co_kg();
    gmbh_co_kg
        .limited_partners
        .push(create_test_limited_partner("Peter Weber", 50_000));
    gmbh_co_kg
        .limited_partners
        .push(create_test_limited_partner("Lisa Becker", 75_000));
    assert!(validate_gmbh_co_kg(&gmbh_co_kg).is_ok());
}

#[test]
fn test_gmbh_co_kg_valid_multiple_directors() {
    let mut gmbh_co_kg = create_test_gmbh_co_kg();
    gmbh_co_kg
        .gmbh_general_partner
        .managing_directors
        .push("Erika Schmidt".to_string());
    assert!(validate_gmbh_co_kg(&gmbh_co_kg).is_ok());
}

#[test]
fn test_gmbh_co_kg_valid_alternative_name() {
    let mut gmbh_co_kg = create_test_gmbh_co_kg();
    gmbh_co_kg.partnership_name = "Holdings GmbH und Co. KG".to_string();
    assert!(validate_gmbh_co_kg(&gmbh_co_kg).is_ok());
}

#[test]
fn test_gmbh_co_kg_invalid_gmbh_capital_too_low() {
    let mut gmbh_co_kg = create_test_gmbh_co_kg();
    gmbh_co_kg.gmbh_general_partner.share_capital = Capital::from_euros(20_000);
    let result = validate_gmbh_co_kg(&gmbh_co_kg);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        HGBError::GmbHPartnerInsufficientCapital { .. }
    ));
}

#[test]
fn test_gmbh_co_kg_invalid_no_managing_directors() {
    let mut gmbh_co_kg = create_test_gmbh_co_kg();
    gmbh_co_kg.gmbh_general_partner.managing_directors.clear();
    let result = validate_gmbh_co_kg(&gmbh_co_kg);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        HGBError::GmbHPartnerNoDirectors { .. }
    ));
}

#[test]
fn test_gmbh_co_kg_invalid_no_limited_partners() {
    let mut gmbh_co_kg = create_test_gmbh_co_kg();
    gmbh_co_kg.limited_partners.clear();
    let result = validate_gmbh_co_kg(&gmbh_co_kg);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), HGBError::KGNoLimitedPartner));
}

#[test]
fn test_gmbh_co_kg_invalid_gmbh_name_missing_suffix() {
    let mut gmbh_co_kg = create_test_gmbh_co_kg();
    gmbh_co_kg.gmbh_general_partner.company_name = "Verwaltungs AG".to_string();
    let result = validate_gmbh_co_kg(&gmbh_co_kg);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        HGBError::InvalidGmbHPartner { .. }
    ));
}

#[test]
fn test_gmbh_co_kg_invalid_partnership_name_missing_suffix() {
    let mut gmbh_co_kg = create_test_gmbh_co_kg();
    gmbh_co_kg.partnership_name = "Verwaltungs Partnership".to_string();
    let result = validate_gmbh_co_kg(&gmbh_co_kg);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        HGBError::MissingLegalFormSuffix { .. }
    ));
}

// =============================================================================
// Partner Validation Tests
// =============================================================================

#[test]
fn test_partner_valid() {
    let partner = create_test_partner("Max Mustermann");
    assert!(validate_partner(&partner).is_ok());
}

#[test]
fn test_partner_invalid_empty_name() {
    let mut partner = create_test_partner("Max Mustermann");
    partner.name = "".to_string();
    let result = validate_partner(&partner);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), HGBError::EmptyPartnerName));
}

#[test]
fn test_partner_invalid_empty_address() {
    let mut partner = create_test_partner("Max Mustermann");
    partner.address = "".to_string();
    let result = validate_partner(&partner);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), HGBError::EmptyPartnerAddress));
}

#[test]
fn test_partner_invalid_contribution_paid_exceeds() {
    let mut partner = create_test_partner("Max Mustermann");
    partner.contribution = Some(Capital::from_euros(10_000));
    partner.contribution_paid = Some(Capital::from_euros(15_000));
    let result = validate_partner(&partner);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        HGBError::ContributionPaidExceedsObligation { .. }
    ));
}

#[test]
fn test_partner_valid_no_contribution() {
    let mut partner = create_test_partner("Max Mustermann");
    partner.contribution = None;
    partner.contribution_paid = None;
    assert!(validate_partner(&partner).is_ok());
}

// =============================================================================
// Limited Partner Validation Tests
// =============================================================================

#[test]
fn test_limited_partner_valid() {
    let partner = create_test_limited_partner("Anna Schmidt", 50_000);
    assert!(validate_limited_partner(&partner).is_ok());
}

#[test]
fn test_limited_partner_valid_minimum_liability() {
    let partner = create_test_limited_partner("Anna Schmidt", 1);
    assert!(validate_limited_partner(&partner).is_ok());
}

#[test]
fn test_limited_partner_invalid_empty_name() {
    let mut partner = create_test_limited_partner("Anna Schmidt", 50_000);
    partner.name = "".to_string();
    let result = validate_limited_partner(&partner);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), HGBError::EmptyPartnerName));
}

#[test]
fn test_limited_partner_invalid_empty_address() {
    let mut partner = create_test_limited_partner("Anna Schmidt", 50_000);
    partner.address = "".to_string();
    let result = validate_limited_partner(&partner);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), HGBError::EmptyPartnerAddress));
}

#[test]
fn test_limited_partner_valid_partial_contribution() {
    let mut partner = create_test_limited_partner("Anna Schmidt", 50_000);
    partner.contribution_paid = Capital::from_euros(25_000);
    assert!(validate_limited_partner(&partner).is_ok());
}

// =============================================================================
// Partnership Name Validation Tests
// =============================================================================

#[test]
fn test_partnership_name_ohg_valid() {
    assert!(validate_partnership_name("Mustermann & Schmidt OHG", "OHG").is_ok());
    assert!(validate_partnership_name("Offene Handelsgesellschaft Müller", "OHG").is_ok());
    assert!(validate_partnership_name("Tech Solutions O.H.G.", "OHG").is_ok());
}

#[test]
fn test_partnership_name_kg_valid() {
    assert!(validate_partnership_name("Tech Ventures KG", "KG").is_ok());
    assert!(validate_partnership_name("Kommanditgesellschaft Schmidt", "KG").is_ok());
    assert!(validate_partnership_name("Innovations K.G.", "KG").is_ok());
}

#[test]
fn test_partnership_name_gmbh_co_kg_valid() {
    assert!(validate_partnership_name("Verwaltungs GmbH & Co. KG", "GmbH & Co. KG").is_ok());
    assert!(validate_partnership_name("Holdings GmbH und Co. KG", "GmbH & Co. KG").is_ok());
}

#[test]
fn test_partnership_name_invalid_empty() {
    let result = validate_partnership_name("", "OHG");
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        HGBError::EmptyPartnershipName
    ));
}

#[test]
fn test_partnership_name_invalid_too_short() {
    let result = validate_partnership_name("AB", "OHG");
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        HGBError::PartnershipNameTooShort { .. }
    ));
}

#[test]
fn test_partnership_name_invalid_missing_suffix() {
    let result = validate_partnership_name("Mustermann & Schmidt", "OHG");
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        HGBError::MissingLegalFormSuffix { .. }
    ));
}

// =============================================================================
// Business Purpose Validation Tests
// =============================================================================

#[test]
fn test_business_purpose_valid() {
    assert!(validate_business_purpose("Softwareentwicklung und IT-Beratung").is_ok());
    assert!(validate_business_purpose("Handel mit Waren aller Art").is_ok());
}

#[test]
fn test_business_purpose_invalid_empty() {
    let result = validate_business_purpose("");
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        HGBError::EmptyBusinessPurpose
    ));
}

#[test]
fn test_business_purpose_invalid_too_short() {
    let result = validate_business_purpose("Software");
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        HGBError::InvalidBusinessPurpose
    ));
}

// =============================================================================
// Registered Office Validation Tests
// =============================================================================

#[test]
fn test_registered_office_valid() {
    assert!(validate_registered_office("Berlin").is_ok());
    assert!(validate_registered_office("München").is_ok());
    assert!(validate_registered_office("Frankfurt am Main").is_ok());
}

#[test]
fn test_registered_office_invalid_empty() {
    let result = validate_registered_office("");
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        HGBError::EmptyRegisteredOffice
    ));
}

// =============================================================================
// Fiscal Year End Validation Tests
// =============================================================================

#[test]
fn test_fiscal_year_end_valid() {
    assert!(validate_fiscal_year_end(FiscalYearEnd { month: 12, day: 31 }).is_ok());
    assert!(validate_fiscal_year_end(FiscalYearEnd { month: 6, day: 30 }).is_ok());
    assert!(validate_fiscal_year_end(FiscalYearEnd { month: 2, day: 29 }).is_ok()); // Leap year
    assert!(validate_fiscal_year_end(FiscalYearEnd { month: 3, day: 31 }).is_ok());
}

#[test]
fn test_fiscal_year_end_invalid_month() {
    let result = validate_fiscal_year_end(FiscalYearEnd { month: 0, day: 31 });
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        HGBError::InvalidFiscalYearEnd { .. }
    ));

    let result = validate_fiscal_year_end(FiscalYearEnd { month: 13, day: 31 });
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        HGBError::InvalidFiscalYearEnd { .. }
    ));
}

#[test]
fn test_fiscal_year_end_invalid_day() {
    let result = validate_fiscal_year_end(FiscalYearEnd { month: 2, day: 30 });
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        HGBError::InvalidFiscalYearEnd { .. }
    ));

    let result = validate_fiscal_year_end(FiscalYearEnd { month: 4, day: 31 });
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        HGBError::InvalidFiscalYearEnd { .. }
    ));

    let result = validate_fiscal_year_end(FiscalYearEnd { month: 1, day: 32 });
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        HGBError::InvalidFiscalYearEnd { .. }
    ));
}

// =============================================================================
// Edge Cases and Complex Scenarios
// =============================================================================

#[test]
fn test_ohg_edge_case_exactly_two_partners() {
    let ohg = create_test_ohg();
    assert_eq!(ohg.partners.len(), 2);
    assert!(validate_ohg(&ohg).is_ok());
}

#[test]
fn test_kg_edge_case_one_of_each_partner_type() {
    let kg = create_test_kg();
    assert_eq!(kg.general_partners.len(), 1);
    assert_eq!(kg.limited_partners.len(), 1);
    assert!(validate_kg(&kg).is_ok());
}

#[test]
fn test_gmbh_co_kg_edge_case_exactly_minimum_capital() {
    let gmbh_co_kg = create_test_gmbh_co_kg();
    assert_eq!(
        gmbh_co_kg.gmbh_general_partner.share_capital.to_euros(),
        25_000.0
    );
    assert!(validate_gmbh_co_kg(&gmbh_co_kg).is_ok());
}

#[test]
fn test_limited_partner_edge_case_minimum_liability() {
    let partner = LimitedPartner {
        name: "Test Partner".to_string(),
        address: "Test Address".to_string(),
        liability_limit: Capital { amount_cents: 100 }, // €1
        contribution_paid: Capital { amount_cents: 100 },
        partner_type: PartnerType::NaturalPerson,
        has_special_representation: false,
    };
    assert!(validate_limited_partner(&partner).is_ok());
}

#[test]
fn test_partnership_name_case_insensitive() {
    assert!(validate_partnership_name("Mustermann & Schmidt ohg", "OHG").is_ok());
    assert!(validate_partnership_name("Tech Ventures kg", "KG").is_ok());
}
