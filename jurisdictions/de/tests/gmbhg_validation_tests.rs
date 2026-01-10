//! Integration tests for German company law validation (GmbHG)
//!
//! Comprehensive test coverage for:
//! - Capital validation (GmbH, UG, AG)
//! - Initial contribution requirements
//! - Company name validation
//! - Articles of association validation
//! - Managing director validation
//! - Edge cases and error handling

use chrono::{NaiveDate, Utc};
use legalis_de::gmbhg::*;

// =============================================================================
// Helper Functions
// =============================================================================

fn create_test_shareholder(name: &str, address: &str) -> Shareholder {
    Shareholder {
        name: name.to_string(),
        address: address.to_string(),
        shareholder_type: ShareholderType::NaturalPerson,
    }
}

fn create_test_share_allocation(
    name: &str,
    nominal_euros: u64,
    paid_euros: u64,
) -> ShareAllocation {
    ShareAllocation {
        shareholder: create_test_shareholder(name, "Berlin, Germany"),
        nominal_amount_cents: nominal_euros * 100,
        contribution_paid_cents: paid_euros * 100,
    }
}

fn create_test_director(name: &str, has_capacity: bool) -> ManagingDirector {
    ManagingDirector {
        name: name.to_string(),
        date_of_birth: Some(NaiveDate::from_ymd_opt(1980, 1, 1).unwrap()),
        address: "Berlin, Germany".to_string(),
        appointment_date: Utc::now(),
        representation_authority: RepresentationAuthority::Sole,
        has_capacity,
    }
}

// =============================================================================
// Capital Validation Tests
// =============================================================================

#[test]
fn test_gmbh_capital_valid_minimum() {
    let capital = Capital::from_euros(25_000);
    assert!(validate_capital(&capital, CompanyType::GmbH).is_ok());
}

#[test]
fn test_gmbh_capital_valid_above_minimum() {
    let capital = Capital::from_euros(50_000);
    assert!(validate_capital(&capital, CompanyType::GmbH).is_ok());

    let high = Capital::from_euros(1_000_000);
    assert!(validate_capital(&high, CompanyType::GmbH).is_ok());
}

#[test]
fn test_gmbh_capital_invalid_below_minimum() {
    let capital = Capital::from_euros(24_999);
    let result = validate_capital(&capital, CompanyType::GmbH);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        GmbHError::CapitalBelowMinimum { .. }
    ));
}

#[test]
fn test_gmbh_capital_invalid_zero() {
    let capital = Capital::from_cents(0);
    let result = validate_capital(&capital, CompanyType::GmbH);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), GmbHError::ZeroCapital));
}

#[test]
fn test_ug_capital_valid_minimum() {
    let capital = Capital::from_euros(1);
    assert!(validate_capital(&capital, CompanyType::UG).is_ok());
}

#[test]
fn test_ug_capital_valid_below_gmbh_threshold() {
    let capital = Capital::from_euros(10_000);
    assert!(validate_capital(&capital, CompanyType::UG).is_ok());

    let max = Capital::from_euros(24_999);
    assert!(validate_capital(&max, CompanyType::UG).is_ok());

    let max_cents = Capital::from_cents(2_499_999);
    assert!(validate_capital(&max_cents, CompanyType::UG).is_ok());
}

#[test]
fn test_ug_capital_invalid_at_gmbh_threshold() {
    let capital = Capital::from_euros(25_000);
    let result = validate_capital(&capital, CompanyType::UG);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        GmbHError::UGCapitalExceedsLimit { .. }
    ));
}

#[test]
fn test_ug_capital_invalid_above_gmbh_threshold() {
    let capital = Capital::from_euros(30_000);
    let result = validate_capital(&capital, CompanyType::UG);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        GmbHError::UGCapitalExceedsLimit { .. }
    ));
}

#[test]
fn test_ag_capital_valid_minimum() {
    let capital = Capital::from_euros(50_000);
    assert!(validate_capital(&capital, CompanyType::AG).is_ok());
}

#[test]
fn test_ag_capital_invalid_below_minimum() {
    let capital = Capital::from_euros(49_999);
    let result = validate_capital(&capital, CompanyType::AG);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        GmbHError::CapitalBelowMinimum { .. }
    ));
}

// =============================================================================
// Initial Contribution Tests (§7 Abs. 2 GmbHG)
// =============================================================================

#[test]
fn test_initial_contribution_valid_50_percent() {
    let shares = vec![create_test_share_allocation("Max", 25_000, 12_500)];
    assert!(validate_initial_contribution(&shares, CompanyType::GmbH).is_ok());
}

#[test]
fn test_initial_contribution_valid_above_50_percent() {
    let shares = vec![create_test_share_allocation("Max", 25_000, 20_000)];
    assert!(validate_initial_contribution(&shares, CompanyType::GmbH).is_ok());
}

#[test]
fn test_initial_contribution_valid_100_percent() {
    let shares = vec![create_test_share_allocation("Max", 25_000, 25_000)];
    assert!(validate_initial_contribution(&shares, CompanyType::GmbH).is_ok());
}

#[test]
fn test_initial_contribution_valid_absolute_minimum() {
    // €25,000 capital, exactly 50% paid (€12,500) - meets both 50% and absolute minimum
    let shares = vec![create_test_share_allocation("Max", 25_000, 12_500)];
    assert!(validate_initial_contribution(&shares, CompanyType::GmbH).is_ok());
}

#[test]
fn test_initial_contribution_invalid_below_50_percent_and_absolute() {
    // €25,000 capital, only 40% paid (€10,000) - below both 50% and €12,500
    let shares = vec![create_test_share_allocation("Max", 25_000, 10_000)];
    let result = validate_initial_contribution(&shares, CompanyType::GmbH);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        GmbHError::InitialContributionTooLow { .. }
    ));
}

#[test]
fn test_initial_contribution_multiple_shareholders() {
    let shares = vec![
        create_test_share_allocation("Max", 30_000, 30_000),
        create_test_share_allocation("Anna", 20_000, 20_000),
    ];
    // Total: €50,000, paid: €50,000 (100%)
    assert!(validate_initial_contribution(&shares, CompanyType::GmbH).is_ok());
}

#[test]
fn test_initial_contribution_not_required_for_ug() {
    let shares = vec![create_test_share_allocation("Max", 1, 1)];
    // UG has no initial contribution requirement (but must be fully paid)
    assert!(validate_initial_contribution(&shares, CompanyType::UG).is_ok());
}

// =============================================================================
// Company Name Validation Tests (§4 GmbHG)
// =============================================================================

#[test]
fn test_company_name_gmbh_valid() {
    assert!(validate_company_name("Tech Solutions GmbH", CompanyType::GmbH).is_ok());
    assert!(validate_company_name("Tech Solutions G.m.b.H.", CompanyType::GmbH).is_ok());
    assert!(validate_company_name("TechSolutionsGmbH", CompanyType::GmbH).is_ok());
    assert!(
        validate_company_name(
            "Tech Solutions Gesellschaft mit beschränkter Haftung",
            CompanyType::GmbH
        )
        .is_ok()
    );
}

#[test]
fn test_company_name_ug_valid() {
    assert!(validate_company_name("Startup UG (haftungsbeschränkt)", CompanyType::UG).is_ok());
    assert!(
        validate_company_name(
            "Startup Unternehmergesellschaft (haftungsbeschränkt)",
            CompanyType::UG
        )
        .is_ok()
    );
    assert!(validate_company_name("StartupUG(haftungsbeschränkt)", CompanyType::UG).is_ok());
}

#[test]
fn test_company_name_ag_valid() {
    assert!(validate_company_name("Tech Corp AG", CompanyType::AG).is_ok());
    assert!(validate_company_name("Tech Corp Aktiengesellschaft", CompanyType::AG).is_ok());
}

#[test]
fn test_company_name_missing_suffix() {
    let result = validate_company_name("Tech Solutions", CompanyType::GmbH);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        GmbHError::MissingLegalFormSuffix { .. }
    ));
}

#[test]
fn test_company_name_empty() {
    let result = validate_company_name("", CompanyType::GmbH);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), GmbHError::EmptyCompanyName));
}

#[test]
fn test_company_name_whitespace_only() {
    let result = validate_company_name("   ", CompanyType::GmbH);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), GmbHError::EmptyCompanyName));
}

// =============================================================================
// Registered Office Validation Tests (§4a GmbHG)
// =============================================================================

#[test]
fn test_registered_office_valid() {
    let office = RegisteredOffice {
        city: "Berlin".to_string(),
        full_address: None,
    };
    assert!(validate_registered_office(&office).is_ok());
}

#[test]
fn test_registered_office_with_full_address() {
    let office = RegisteredOffice {
        city: "München".to_string(),
        full_address: Some("Marienplatz 1, 80331 München".to_string()),
    };
    assert!(validate_registered_office(&office).is_ok());
}

#[test]
fn test_registered_office_invalid_with_numbers() {
    let office = RegisteredOffice {
        city: "Berlin123".to_string(),
        full_address: None,
    };
    let result = validate_registered_office(&office);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        GmbHError::InvalidRegisteredOffice { .. }
    ));
}

#[test]
fn test_registered_office_empty() {
    let office = RegisteredOffice {
        city: "".to_string(),
        full_address: None,
    };
    let result = validate_registered_office(&office);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        GmbHError::EmptyRegisteredOffice
    ));
}

#[test]
fn test_registered_office_too_short() {
    let office = RegisteredOffice {
        city: "A".to_string(),
        full_address: None,
    };
    let result = validate_registered_office(&office);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        GmbHError::InvalidRegisteredOffice { .. }
    ));
}

// =============================================================================
// Business Purpose Validation Tests (§3 Abs. 1 Nr. 2 GmbHG)
// =============================================================================

#[test]
fn test_business_purpose_valid() {
    assert!(validate_business_purpose("Software development and IT consulting").is_ok());
    assert!(validate_business_purpose("Entwicklung und Vertrieb von Softwarelösungen").is_ok());
}

#[test]
fn test_business_purpose_invalid_too_short() {
    let result = validate_business_purpose("Software");
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        GmbHError::InvalidBusinessPurpose
    ));
}

#[test]
fn test_business_purpose_invalid_empty() {
    let result = validate_business_purpose("");
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        GmbHError::InvalidBusinessPurpose
    ));
}

// =============================================================================
// Share Structure Validation Tests
// =============================================================================

#[test]
fn test_share_structure_valid_single_shareholder() {
    let shares = vec![create_test_share_allocation("Max", 25_000, 25_000)];
    let capital = Capital::from_euros(25_000);
    assert!(validate_share_structure(&shares, &capital).is_ok());
}

#[test]
fn test_share_structure_valid_multiple_shareholders() {
    let shares = vec![
        create_test_share_allocation("Max", 30_000, 30_000),
        create_test_share_allocation("Anna", 20_000, 20_000),
    ];
    let capital = Capital::from_euros(50_000);
    assert!(validate_share_structure(&shares, &capital).is_ok());
}

#[test]
fn test_share_structure_invalid_mismatch() {
    let shares = vec![create_test_share_allocation("Max", 30_000, 30_000)];
    let capital = Capital::from_euros(50_000); // Mismatch!
    let result = validate_share_structure(&shares, &capital);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        GmbHError::ShareAllocationMismatch { .. }
    ));
}

#[test]
fn test_share_structure_invalid_no_shareholders() {
    let shares: Vec<ShareAllocation> = vec![];
    let capital = Capital::from_euros(25_000);
    let result = validate_share_structure(&shares, &capital);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), GmbHError::NoShareholders));
}

#[test]
fn test_share_structure_invalid_contribution_exceeds_nominal() {
    let mut share = create_test_share_allocation("Max", 25_000, 30_000);
    share.contribution_paid_cents = 3_000_000; // €30,000 > €25,000
    let shares = vec![share];
    let capital = Capital::from_euros(25_000);
    let result = validate_share_structure(&shares, &capital);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        GmbHError::ContributionExceedsNominal { .. }
    ));
}

#[test]
fn test_share_structure_invalid_nominal_below_1_euro() {
    let mut share = create_test_share_allocation("Max", 0, 0);
    share.nominal_amount_cents = 50; // €0.50 < €1
    let shares = vec![share];
    let capital = Capital::from_cents(50);
    let result = validate_share_structure(&shares, &capital);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        GmbHError::NominalAmountTooLow { .. }
    ));
}

// =============================================================================
// Fiscal Year End Validation Tests
// =============================================================================

#[test]
fn test_fiscal_year_end_valid() {
    let fye = FiscalYearEnd { month: 12, day: 31 };
    assert!(validate_fiscal_year_end(fye).is_ok());

    let mid_year = FiscalYearEnd { month: 6, day: 30 };
    assert!(validate_fiscal_year_end(mid_year).is_ok());
}

#[test]
fn test_fiscal_year_end_valid_leap_year() {
    let feb29 = FiscalYearEnd { month: 2, day: 29 };
    assert!(validate_fiscal_year_end(feb29).is_ok());
}

#[test]
fn test_fiscal_year_end_invalid_month() {
    let fye = FiscalYearEnd { month: 13, day: 31 };
    let result = validate_fiscal_year_end(fye);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        GmbHError::InvalidFiscalYearEnd { .. }
    ));

    let zero_month = FiscalYearEnd { month: 0, day: 15 };
    let result = validate_fiscal_year_end(zero_month);
    assert!(result.is_err());
}

#[test]
fn test_fiscal_year_end_invalid_day() {
    let feb30 = FiscalYearEnd { month: 2, day: 30 }; // Feb 30 doesn't exist
    let result = validate_fiscal_year_end(feb30);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        GmbHError::InvalidFiscalYearEnd { .. }
    ));

    let apr31 = FiscalYearEnd { month: 4, day: 31 }; // April has 30 days
    let result = validate_fiscal_year_end(apr31);
    assert!(result.is_err());
}

// =============================================================================
// Managing Director Validation Tests (§35 GmbHG)
// =============================================================================

#[test]
fn test_managing_directors_valid_single() {
    let directors = ManagingDirectors {
        directors: vec![create_test_director("Max Mustermann", true)],
    };
    assert!(validate_managing_directors(&directors).is_ok());
}

#[test]
fn test_managing_directors_valid_multiple() {
    let directors = ManagingDirectors {
        directors: vec![
            create_test_director("Max Mustermann", true),
            create_test_director("Anna Schmidt", true),
        ],
    };
    assert!(validate_managing_directors(&directors).is_ok());
}

#[test]
fn test_managing_directors_invalid_empty() {
    let directors = ManagingDirectors { directors: vec![] };
    let result = validate_managing_directors(&directors);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        GmbHError::NoManagingDirectors
    ));
}

#[test]
fn test_managing_director_invalid_lacks_capacity() {
    let directors = ManagingDirectors {
        directors: vec![create_test_director("Max Mustermann", false)],
    };
    let result = validate_managing_directors(&directors);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        GmbHError::DirectorLacksCapacity { .. }
    ));
}

#[test]
fn test_managing_director_invalid_empty_name() {
    let mut director = create_test_director("", true);
    director.name = "".to_string();
    let directors = ManagingDirectors {
        directors: vec![director],
    };
    let result = validate_managing_directors(&directors);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), GmbHError::EmptyDirectorName));
}

#[test]
fn test_managing_director_invalid_empty_address() {
    let mut director = create_test_director("Max", true);
    director.address = "".to_string();
    let directors = ManagingDirectors {
        directors: vec![director],
    };
    let result = validate_managing_directors(&directors);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        GmbHError::EmptyDirectorAddress
    ));
}

// =============================================================================
// Complete Articles of Association Validation Tests
// =============================================================================

#[test]
fn test_articles_of_association_valid_gmbh() {
    let articles = ArticlesOfAssociation {
        company_name: "Tech Solutions GmbH".to_string(),
        registered_office: RegisteredOffice {
            city: "Berlin".to_string(),
            full_address: None,
        },
        business_purpose: "Software development and IT consulting services".to_string(),
        share_capital: Capital::from_euros(50_000),
        share_structure: vec![create_test_share_allocation("Max", 50_000, 50_000)],
        duration: Some(Duration::Unlimited),
        fiscal_year_end: Some(FiscalYearEnd { month: 12, day: 31 }),
        formation_date: Some(Utc::now()),
        resolution_requirements: None,
    };

    assert!(validate_articles_of_association(&articles, CompanyType::GmbH).is_ok());
}

#[test]
fn test_articles_of_association_valid_ug() {
    let articles = ArticlesOfAssociation {
        company_name: "Startup UG (haftungsbeschränkt)".to_string(),
        registered_office: RegisteredOffice {
            city: "Hamburg".to_string(),
            full_address: None,
        },
        business_purpose: "E-commerce services and online retail operations".to_string(),
        share_capital: Capital::from_euros(1),
        share_structure: vec![create_test_share_allocation("Anna", 1, 1)],
        duration: Some(Duration::Unlimited),
        fiscal_year_end: Some(FiscalYearEnd { month: 12, day: 31 }),
        formation_date: Some(Utc::now()),
        resolution_requirements: None,
    };

    assert!(validate_articles_of_association(&articles, CompanyType::UG).is_ok());
}

#[test]
fn test_articles_of_association_invalid_capital() {
    let articles = ArticlesOfAssociation {
        company_name: "Tech Solutions GmbH".to_string(),
        registered_office: RegisteredOffice {
            city: "Berlin".to_string(),
            full_address: None,
        },
        business_purpose: "Software development".to_string(),
        share_capital: Capital::from_euros(20_000), // Below minimum!
        share_structure: vec![create_test_share_allocation("Max", 20_000, 20_000)],
        duration: Some(Duration::Unlimited),
        fiscal_year_end: None,
        formation_date: None,
        resolution_requirements: None,
    };

    let result = validate_articles_of_association(&articles, CompanyType::GmbH);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        GmbHError::CapitalBelowMinimum { .. }
    ));
}
