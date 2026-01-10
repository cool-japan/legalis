//! Validation logic for German company law (GmbHG)
//!
//! Multi-stage validation implementing §3, §5, §7 GmbHG requirements.
//!
//! # Validation Strategy
//!
//! 1. **Field-level validation**: Individual field constraints (capital >= minimum)
//! 2. **Relationship validation**: Cross-field checks (share allocations sum = capital)
//! 3. **Type-specific validation**: Rules that vary by company type (GmbH vs UG)
//! 4. **Comprehensive error reporting**: Return specific error with context

use crate::gmbhg::error::{GmbHError, Result};
use crate::gmbhg::types::*;

// =============================================================================
// Capital Validation (§5 GmbHG)
// =============================================================================

/// Validates capital meets legal requirements for company type
///
/// # Legal Requirements
///
/// - **GmbH**: Minimum €25,000 (§5 GmbHG)
/// - **UG**: Minimum €1, maximum €24,999 (§5a GmbHG)
/// - **AG**: Minimum €50,000 (§7 AktG)
///
/// # Arguments
///
/// * `capital` - Capital amount to validate
/// * `company_type` - Type of company (GmbH, UG, AG, etc.)
///
/// # Examples
///
/// ```
/// use legalis_de::gmbhg::{validate_capital, Capital, CompanyType};
///
/// let capital = Capital::from_euros(25_000);
/// assert!(validate_capital(&capital, CompanyType::GmbH).is_ok());
///
/// let invalid = Capital::from_euros(24_999);
/// assert!(validate_capital(&invalid, CompanyType::GmbH).is_err());
/// ```
pub fn validate_capital(capital: &Capital, company_type: CompanyType) -> Result<()> {
    // Check for zero capital
    if capital.amount_cents == 0 {
        return Err(GmbHError::ZeroCapital);
    }

    match company_type {
        CompanyType::GmbH => {
            if capital.amount_cents < Capital::GMBH_MINIMUM_CENTS {
                return Err(GmbHError::CapitalBelowMinimum {
                    actual_euros: capital.to_euros(),
                    minimum_euros: 25_000.0,
                });
            }
        }
        CompanyType::UG => {
            if capital.amount_cents < Capital::UG_MINIMUM_CENTS {
                return Err(GmbHError::CapitalBelowMinimum {
                    actual_euros: capital.to_euros(),
                    minimum_euros: 0.01,
                });
            }
            // UG capital must be less than €25,000 (at €25,000 it becomes GmbH)
            if capital.amount_cents >= Capital::GMBH_MINIMUM_CENTS {
                return Err(GmbHError::UGCapitalExceedsLimit {
                    actual_euros: capital.to_euros(),
                });
            }
        }
        CompanyType::AG => {
            if capital.amount_cents < Capital::AG_MINIMUM_CENTS {
                return Err(GmbHError::CapitalBelowMinimum {
                    actual_euros: capital.to_euros(),
                    minimum_euros: 50_000.0,
                });
            }
        }
        // OHG, KG, GmbHCoKG have no minimum capital requirements
        CompanyType::OHG | CompanyType::KG | CompanyType::GmbHCoKG => {}
    }

    Ok(())
}

// =============================================================================
// Initial Contribution Validation (§7 Abs. 2 GmbHG)
// =============================================================================

/// Validates initial contribution meets §7 Abs. 2 GmbHG requirements
///
/// # Legal Requirements for GmbH
///
/// At least one of the following must be satisfied:
/// 1. At least 50% of each share's nominal amount paid, OR
/// 2. At least €12,500 total paid (absolute minimum)
///
/// # Arguments
///
/// * `share_allocations` - List of share allocations with contributions
/// * `company_type` - Type of company (only enforced for GmbH)
///
/// # Examples
///
/// ```
/// use legalis_de::gmbhg::{validate_initial_contribution, ShareAllocation, Shareholder, ShareholderType, CompanyType};
///
/// let shares = vec![
///     ShareAllocation {
///         shareholder: Shareholder {
///             name: "Max".to_string(),
///             address: "Berlin".to_string(),
///             shareholder_type: ShareholderType::NaturalPerson,
///         },
///         nominal_amount_cents: 2_500_000, // €25,000
///         contribution_paid_cents: 1_250_000, // €12,500 (50%)
///     },
/// ];
///
/// assert!(validate_initial_contribution(&shares, CompanyType::GmbH).is_ok());
/// ```
pub fn validate_initial_contribution(
    share_allocations: &[ShareAllocation],
    company_type: CompanyType,
) -> Result<()> {
    // Only GmbH has initial contribution requirements
    if company_type != CompanyType::GmbH {
        return Ok(());
    }

    let total_paid: u64 = share_allocations
        .iter()
        .map(|s| s.contribution_paid_cents)
        .sum();

    let total_nominal: u64 = share_allocations
        .iter()
        .map(|s| s.nominal_amount_cents)
        .sum();

    // Check 50% requirement
    let required_fifty_percent = total_nominal / 2;

    // Check absolute minimum €12,500 (§7 Abs. 2 GmbHG)
    let required_absolute = Capital::GMBH_INITIAL_CONTRIBUTION_MIN_CENTS;

    // Must satisfy the higher of the two requirements
    let required = required_fifty_percent.max(required_absolute);

    if total_paid < required {
        return Err(GmbHError::InitialContributionTooLow {
            paid_euros: (total_paid as f64) / 100.0,
            required_euros: (required as f64) / 100.0,
        });
    }

    Ok(())
}

// =============================================================================
// Company Name Validation (§4 GmbHG)
// =============================================================================

/// Validates company name includes proper legal form suffix
///
/// # Legal Requirements
///
/// - **GmbH**: Must include "GmbH" or variants like "G.m.b.H."
/// - **UG**: Must include "UG (haftungsbeschränkt)" or "Unternehmergesellschaft (haftungsbeschränkt)"
/// - **AG**: Must include "AG" or "Aktiengesellschaft"
///
/// # Arguments
///
/// * `name` - Company name to validate
/// * `company_type` - Type of company
///
/// # Examples
///
/// ```
/// use legalis_de::gmbhg::{validate_company_name, CompanyType};
///
/// assert!(validate_company_name("Tech Solutions GmbH", CompanyType::GmbH).is_ok());
/// assert!(validate_company_name("Tech Solutions G.m.b.H.", CompanyType::GmbH).is_ok());
/// assert!(validate_company_name("Tech Solutions", CompanyType::GmbH).is_err());
/// ```
pub fn validate_company_name(name: &str, company_type: CompanyType) -> Result<()> {
    if name.trim().is_empty() {
        return Err(GmbHError::EmptyCompanyName);
    }

    // Normalize for flexible matching: remove spaces and dots, lowercase
    let normalized = name.replace([' ', '.'], "").to_lowercase();

    // Check for valid suffixes based on company type
    let valid = match company_type {
        CompanyType::GmbH => {
            normalized.contains("gmbh") || normalized.contains("gesellschaftmitbeschränkterhaftung")
        }
        CompanyType::UG => {
            normalized.contains("ug(haftungsbeschränkt)")
                || normalized.contains("ughaftungsbeschränkt")
                || normalized.contains("unternehmergesellschaft(haftungsbeschränkt)")
                || normalized.contains("unternehmergesellschafthaftungsbeschränkt")
        }
        CompanyType::AG => normalized.contains("ag") || normalized.contains("aktiengesellschaft"),
        CompanyType::OHG => {
            normalized.contains("ohg") || normalized.contains("offenehandelsgesellschaft")
        }
        CompanyType::KG => {
            normalized.contains("kg") || normalized.contains("kommanditgesellschaft")
        }
        CompanyType::GmbHCoKG => {
            normalized.contains("gmbh&cokg")
                || normalized.contains("gmbhundcokg")
                || normalized.contains("gmbh&cokommanditgesellschaft")
        }
    };

    if !valid {
        let required_suffix = match company_type {
            CompanyType::GmbH => "GmbH",
            CompanyType::UG => "UG (haftungsbeschränkt)",
            CompanyType::AG => "AG",
            CompanyType::OHG => "OHG",
            CompanyType::KG => "KG",
            CompanyType::GmbHCoKG => "GmbH & Co. KG",
        };

        return Err(GmbHError::MissingLegalFormSuffix {
            name: name.to_string(),
            required_suffix: required_suffix.to_string(),
        });
    }

    Ok(())
}

// =============================================================================
// Registered Office Validation (§4a GmbHG)
// =============================================================================

/// Validates registered office is a German city
///
/// # Legal Requirements
///
/// - Must be located in Germany (§4a GmbHG)
/// - Must be a specific city name (not just country)
///
/// # Arguments
///
/// * `office` - Registered office to validate
pub fn validate_registered_office(office: &RegisteredOffice) -> Result<()> {
    if office.city.trim().is_empty() {
        return Err(GmbHError::EmptyRegisteredOffice);
    }

    // Basic check: German cities don't contain numbers
    if office.city.chars().any(|c| c.is_numeric()) {
        return Err(GmbHError::InvalidRegisteredOffice {
            city: office.city.clone(),
        });
    }

    // Additional check: City name should be at least 2 characters
    if office.city.trim().len() < 2 {
        return Err(GmbHError::InvalidRegisteredOffice {
            city: office.city.clone(),
        });
    }

    Ok(())
}

// =============================================================================
// Business Purpose Validation (§3 Abs. 1 Nr. 2 GmbHG)
// =============================================================================

/// Validates business purpose is specified and adequate
///
/// # Legal Requirements
///
/// - Must be specific enough to identify the company's activities (§3 Abs. 1 Nr. 2 GmbHG)
/// - Must be lawful
///
/// # Arguments
///
/// * `purpose` - Business purpose to validate
pub fn validate_business_purpose(purpose: &str) -> Result<()> {
    let trimmed = purpose.trim();

    if trimmed.is_empty() {
        return Err(GmbHError::InvalidBusinessPurpose);
    }

    // Must be at least 10 characters for meaningful description
    if trimmed.len() < 10 {
        return Err(GmbHError::InvalidBusinessPurpose);
    }

    Ok(())
}

// =============================================================================
// Share Structure Validation
// =============================================================================

/// Validates share structure matches capital
///
/// # Legal Requirements
///
/// - Sum of nominal amounts must equal share capital (§3 Abs. 1 Nr. 4 GmbHG)
/// - At least one shareholder required
/// - Each nominal amount must be at least €1
///
/// # Arguments
///
/// * `shares` - Share allocations
/// * `capital` - Total share capital
pub fn validate_share_structure(shares: &[ShareAllocation], capital: &Capital) -> Result<()> {
    if shares.is_empty() {
        return Err(GmbHError::NoShareholders);
    }

    // Sum of nominal amounts must equal capital
    let total_nominal: u64 = shares.iter().map(|s| s.nominal_amount_cents).sum();

    if total_nominal != capital.amount_cents {
        return Err(GmbHError::ShareAllocationMismatch {
            total_shares: (total_nominal as f64) / 100.0,
            capital: capital.to_euros(),
        });
    }

    // Validate each share allocation
    for share in shares {
        validate_share_allocation(share)?;
    }

    Ok(())
}

/// Validates individual share allocation
fn validate_share_allocation(share: &ShareAllocation) -> Result<()> {
    // Check for zero nominal amount
    if share.nominal_amount_cents == 0 {
        return Err(GmbHError::ZeroNominalAmount);
    }

    // Nominal amount must be at least €1 (100 cents)
    if share.nominal_amount_cents < 100 {
        return Err(GmbHError::NominalAmountTooLow {
            name: share.shareholder.name.clone(),
        });
    }

    // Contribution paid cannot exceed nominal
    if share.contribution_paid_cents > share.nominal_amount_cents {
        return Err(GmbHError::ContributionExceedsNominal {
            name: share.shareholder.name.clone(),
            paid: (share.contribution_paid_cents as f64) / 100.0,
            nominal: (share.nominal_amount_cents as f64) / 100.0,
        });
    }

    // Validate shareholder data
    validate_shareholder(&share.shareholder)?;

    Ok(())
}

/// Validates shareholder data
fn validate_shareholder(shareholder: &Shareholder) -> Result<()> {
    if shareholder.name.trim().is_empty() {
        return Err(GmbHError::EmptyShareholderName);
    }

    if shareholder.address.trim().is_empty() {
        return Err(GmbHError::EmptyShareholderAddress);
    }

    Ok(())
}

// =============================================================================
// Fiscal Year End Validation
// =============================================================================

/// Validates fiscal year end date
///
/// # Arguments
///
/// * `fye` - Fiscal year end to validate
pub fn validate_fiscal_year_end(fye: FiscalYearEnd) -> Result<()> {
    if !(1..=12).contains(&fye.month) {
        return Err(GmbHError::InvalidFiscalYearEnd {
            month: fye.month,
            day: fye.day,
        });
    }

    // Validate day based on month
    let max_day = match fye.month {
        2 => 29, // Allow Feb 29 (leap years)
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    };

    if !(1..=max_day).contains(&fye.day) {
        return Err(GmbHError::InvalidFiscalYearEnd {
            month: fye.month,
            day: fye.day,
        });
    }

    Ok(())
}

// =============================================================================
// Articles of Association Validation (§3 GmbHG)
// =============================================================================

/// Comprehensive validation of articles per §3 GmbHG
///
/// Performs multi-stage validation:
/// 1. Company name validation (§4 GmbHG)
/// 2. Registered office validation (§4a GmbHG)
/// 3. Business purpose validation (§3 Abs. 1 Nr. 2)
/// 4. Capital validation (§5 GmbHG)
/// 5. Share structure validation (§3 Abs. 1 Nr. 4)
/// 6. Initial contribution validation (§7 Abs. 2)
/// 7. Optional field validation
///
/// # Arguments
///
/// * `articles` - Articles of association to validate
/// * `company_type` - Type of company (GmbH, UG, etc.)
///
/// # Examples
///
/// ```no_run
/// use legalis_de::gmbhg::{validate_articles_of_association, ArticlesOfAssociation, CompanyType};
///
/// // let articles = ArticlesOfAssociation { ... };
/// // let result = validate_articles_of_association(&articles, CompanyType::GmbH);
/// ```
pub fn validate_articles_of_association(
    articles: &ArticlesOfAssociation,
    company_type: CompanyType,
) -> Result<()> {
    // 1. Validate company name (§4 GmbHG)
    validate_company_name(&articles.company_name, company_type)?;

    // 2. Validate registered office (§4a GmbHG)
    validate_registered_office(&articles.registered_office)?;

    // 3. Validate business purpose (§3 Abs. 1 Nr. 2)
    validate_business_purpose(&articles.business_purpose)?;

    // 4. Validate capital (§5 GmbHG)
    validate_capital(&articles.share_capital, company_type)?;

    // 5. Validate share structure (§3 Abs. 1 Nr. 4)
    validate_share_structure(&articles.share_structure, &articles.share_capital)?;

    // 6. Validate initial contributions (§7 Abs. 2)
    validate_initial_contribution(&articles.share_structure, company_type)?;

    // 7. Validate fiscal year end (if specified)
    if let Some(fye) = articles.fiscal_year_end {
        validate_fiscal_year_end(fye)?;
    }

    Ok(())
}

// =============================================================================
// Managing Director Validation (§35 GmbHG)
// =============================================================================

/// Validates managing directors meet requirements
///
/// # Legal Requirements
///
/// - At least one managing director required (§6 Abs. 3 GmbHG)
/// - Must be natural person with full capacity (§6 Abs. 2 S. 2 GmbHG)
/// - Must have valid name and address
///
/// # Arguments
///
/// * `directors` - Managing directors to validate
///
/// # Examples
///
/// ```no_run
/// use legalis_de::gmbhg::{validate_managing_directors, ManagingDirectors};
///
/// // let directors = ManagingDirectors { directors: vec![...] };
/// // let result = validate_managing_directors(&directors);
/// ```
pub fn validate_managing_directors(directors: &ManagingDirectors) -> Result<()> {
    if directors.directors.is_empty() {
        return Err(GmbHError::NoManagingDirectors);
    }

    for director in &directors.directors {
        validate_managing_director(director)?;
    }

    Ok(())
}

/// Validates individual managing director
fn validate_managing_director(director: &ManagingDirector) -> Result<()> {
    if director.name.trim().is_empty() {
        return Err(GmbHError::EmptyDirectorName);
    }

    if !director.has_capacity {
        return Err(GmbHError::DirectorLacksCapacity {
            name: director.name.clone(),
        });
    }

    if director.address.trim().is_empty() {
        return Err(GmbHError::EmptyDirectorAddress);
    }

    Ok(())
}

// =============================================================================
// Unit Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn create_test_shareholder(name: &str) -> Shareholder {
        Shareholder {
            name: name.to_string(),
            address: "Berlin, Germany".to_string(),
            shareholder_type: ShareholderType::NaturalPerson,
        }
    }

    #[test]
    fn test_validate_gmbh_capital_valid() {
        let capital = Capital::from_euros(25_000);
        assert!(validate_capital(&capital, CompanyType::GmbH).is_ok());

        let high = Capital::from_euros(1_000_000);
        assert!(validate_capital(&high, CompanyType::GmbH).is_ok());
    }

    #[test]
    fn test_validate_gmbh_capital_invalid() {
        let invalid = Capital::from_euros(24_999);
        let result = validate_capital(&invalid, CompanyType::GmbH);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            GmbHError::CapitalBelowMinimum { .. }
        ));
    }

    #[test]
    fn test_validate_ug_capital_valid() {
        let min = Capital::from_euros(1);
        assert!(validate_capital(&min, CompanyType::UG).is_ok());

        let mid = Capital::from_euros(10_000);
        assert!(validate_capital(&mid, CompanyType::UG).is_ok());

        let max = Capital::from_cents(2_499_999);
        assert!(validate_capital(&max, CompanyType::UG).is_ok());
    }

    #[test]
    fn test_validate_ug_capital_too_high() {
        let too_high = Capital::from_euros(25_000);
        let result = validate_capital(&too_high, CompanyType::UG);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            GmbHError::UGCapitalExceedsLimit { .. }
        ));
    }

    #[test]
    fn test_validate_zero_capital() {
        let zero = Capital::from_cents(0);
        let result = validate_capital(&zero, CompanyType::GmbH);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GmbHError::ZeroCapital));
    }

    #[test]
    fn test_validate_initial_contribution_50_percent() {
        let shares = vec![ShareAllocation {
            shareholder: create_test_shareholder("Max"),
            nominal_amount_cents: 2_500_000,    // €25,000
            contribution_paid_cents: 1_250_000, // €12,500 (50%)
        }];
        assert!(validate_initial_contribution(&shares, CompanyType::GmbH).is_ok());
    }

    #[test]
    fn test_validate_initial_contribution_absolute_minimum() {
        let shares = vec![ShareAllocation {
            shareholder: create_test_shareholder("Max"),
            nominal_amount_cents: 2_500_000,    // €25,000
            contribution_paid_cents: 1_250_000, // €12,500 (50%, meets both requirements)
        }];
        assert!(validate_initial_contribution(&shares, CompanyType::GmbH).is_ok());
    }

    #[test]
    fn test_validate_initial_contribution_too_low() {
        let shares = vec![ShareAllocation {
            shareholder: create_test_shareholder("Max"),
            nominal_amount_cents: 2_500_000,    // €25,000
            contribution_paid_cents: 1_000_000, // €10,000 (40%, below minimum)
        }];
        let result = validate_initial_contribution(&shares, CompanyType::GmbH);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            GmbHError::InitialContributionTooLow { .. }
        ));
    }

    #[test]
    fn test_validate_company_name_gmbh() {
        assert!(validate_company_name("Tech Solutions GmbH", CompanyType::GmbH).is_ok());
        assert!(validate_company_name("Tech Solutions G.m.b.H.", CompanyType::GmbH).is_ok());
        assert!(validate_company_name("TechSolutionsGmbH", CompanyType::GmbH).is_ok());
    }

    #[test]
    fn test_validate_company_name_ug() {
        assert!(validate_company_name("Startup UG (haftungsbeschränkt)", CompanyType::UG).is_ok());
        assert!(validate_company_name("StartupUG(haftungsbeschränkt)", CompanyType::UG).is_ok());
    }

    #[test]
    fn test_validate_company_name_missing_suffix() {
        let result = validate_company_name("Tech Solutions", CompanyType::GmbH);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            GmbHError::MissingLegalFormSuffix { .. }
        ));
    }

    #[test]
    fn test_validate_company_name_empty() {
        let result = validate_company_name("", CompanyType::GmbH);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), GmbHError::EmptyCompanyName));
    }

    #[test]
    fn test_validate_registered_office_valid() {
        let office = RegisteredOffice {
            city: "Berlin".to_string(),
            full_address: None,
        };
        assert!(validate_registered_office(&office).is_ok());
    }

    #[test]
    fn test_validate_registered_office_with_numbers() {
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
    fn test_validate_business_purpose_valid() {
        let purpose = "Entwicklung und Vertrieb von Softwarelösungen";
        assert!(validate_business_purpose(purpose).is_ok());
    }

    #[test]
    fn test_validate_business_purpose_too_short() {
        let purpose = "Software";
        let result = validate_business_purpose(purpose);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            GmbHError::InvalidBusinessPurpose
        ));
    }

    #[test]
    fn test_validate_share_structure_valid() {
        let shares = vec![
            ShareAllocation {
                shareholder: create_test_shareholder("Max"),
                nominal_amount_cents: 3_000_000, // €30,000
                contribution_paid_cents: 3_000_000,
            },
            ShareAllocation {
                shareholder: create_test_shareholder("Anna"),
                nominal_amount_cents: 2_000_000, // €20,000
                contribution_paid_cents: 2_000_000,
            },
        ];
        let capital = Capital::from_euros(50_000);
        assert!(validate_share_structure(&shares, &capital).is_ok());
    }

    #[test]
    fn test_validate_share_structure_mismatch() {
        let shares = vec![ShareAllocation {
            shareholder: create_test_shareholder("Max"),
            nominal_amount_cents: 3_000_000, // €30,000
            contribution_paid_cents: 3_000_000,
        }];
        let capital = Capital::from_euros(50_000); // Mismatch!
        let result = validate_share_structure(&shares, &capital);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            GmbHError::ShareAllocationMismatch { .. }
        ));
    }

    #[test]
    fn test_validate_fiscal_year_end_valid() {
        let fye = FiscalYearEnd { month: 12, day: 31 };
        assert!(validate_fiscal_year_end(fye).is_ok());

        let feb29 = FiscalYearEnd { month: 2, day: 29 };
        assert!(validate_fiscal_year_end(feb29).is_ok());
    }

    #[test]
    fn test_validate_fiscal_year_end_invalid_month() {
        let fye = FiscalYearEnd { month: 13, day: 31 };
        let result = validate_fiscal_year_end(fye);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            GmbHError::InvalidFiscalYearEnd { .. }
        ));
    }

    #[test]
    fn test_validate_fiscal_year_end_invalid_day() {
        let fye = FiscalYearEnd { month: 2, day: 30 }; // Feb 30 doesn't exist
        let result = validate_fiscal_year_end(fye);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            GmbHError::InvalidFiscalYearEnd { .. }
        ));
    }

    #[test]
    fn test_validate_managing_directors_valid() {
        let directors = ManagingDirectors {
            directors: vec![ManagingDirector {
                name: "Max Mustermann".to_string(),
                date_of_birth: None,
                address: "Berlin".to_string(),
                appointment_date: Utc::now(),
                representation_authority: RepresentationAuthority::Sole,
                has_capacity: true,
            }],
        };
        assert!(validate_managing_directors(&directors).is_ok());
    }

    #[test]
    fn test_validate_managing_directors_empty() {
        let directors = ManagingDirectors { directors: vec![] };
        let result = validate_managing_directors(&directors);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            GmbHError::NoManagingDirectors
        ));
    }

    #[test]
    fn test_validate_managing_director_lacks_capacity() {
        let director = ManagingDirector {
            name: "Max".to_string(),
            date_of_birth: None,
            address: "Berlin".to_string(),
            appointment_date: Utc::now(),
            representation_authority: RepresentationAuthority::Sole,
            has_capacity: false, // Invalid!
        };
        let result = validate_managing_director(&director);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            GmbHError::DirectorLacksCapacity { .. }
        ));
    }
}
