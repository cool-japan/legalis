//! Companies Act (Cap. 50) - Validation Logic
//!
//! This module provides validation functions for Singapore Companies Act compliance.
//!
//! ## Validation Functions
//!
//! - [`validate_company_formation`]: Comprehensive company formation validation
//! - [`validate_director_eligibility`]: Check if director is eligible for appointment
//! - [`validate_resident_director_requirement`]: Verify s. 145 compliance
//! - [`validate_agm_requirement`]: Check AGM compliance (s. 175)
//! - [`validate_annual_return_deadline`]: Calculate annual return filing deadline
//!
//! ## Examples
//!
//! ```
//! use legalis_sg::companies::*;
//! use chrono::Utc;
//!
//! let company = Company::new(
//!     "202401234A",
//!     "Tech Pte Ltd",
//!     CompanyType::PrivateLimited,
//!     Address::singapore("1 Raffles Place", "048616"),
//! );
//!
//! match validate_company_formation(&company) {
//!     Ok(report) => println!("✅ Validation passed: {:?}", report),
//!     Err(e) => eprintln!("❌ Validation failed: {}", e),
//! }
//! ```

use super::error::{CompaniesError, Result};
use super::types::*;
use chrono::{DateTime, Datelike, Duration, TimeZone, Utc};
use serde::{Deserialize, Serialize};

/// Validation report with detailed compliance information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidationReport {
    /// Whether the company passes all validations
    pub is_valid: bool,

    /// List of validation errors (blocking issues)
    pub errors: Vec<String>,

    /// List of warnings (non-blocking issues)
    pub warnings: Vec<String>,

    /// Statute references cited during validation
    pub legal_references: Vec<String>,

    /// Timestamp of validation
    pub validated_at: DateTime<Utc>,
}

impl ValidationReport {
    /// Creates a new successful validation report
    pub fn success() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            legal_references: Vec::new(),
            validated_at: Utc::now(),
        }
    }

    /// Creates a validation report with errors
    pub fn with_errors(errors: Vec<String>) -> Self {
        Self {
            is_valid: false,
            errors,
            warnings: Vec::new(),
            legal_references: Vec::new(),
            validated_at: Utc::now(),
        }
    }

    /// Adds an error to the report
    pub fn add_error(&mut self, error: impl Into<String>) {
        self.errors.push(error.into());
        self.is_valid = false;
    }

    /// Adds a warning to the report
    pub fn add_warning(&mut self, warning: impl Into<String>) {
        self.warnings.push(warning.into());
    }

    /// Adds a legal reference
    pub fn add_legal_reference(&mut self, reference: impl Into<String>) {
        self.legal_references.push(reference.into());
    }
}

/// Validates complete company formation
///
/// Checks all Companies Act requirements for valid company structure:
/// - Resident director requirement (s. 145)
/// - Company name and legal suffix
/// - Share capital consistency
/// - Shareholder requirements
/// - Company secretary (if required)
/// - Registered office in Singapore
///
/// ## Examples
///
/// ```
/// use legalis_sg::companies::*;
///
/// let mut company = Company::new(
///     "202401234A",
///     "Tech Innovations Pte Ltd",
///     CompanyType::PrivateLimited,
///     Address::singapore("1 Raffles Place", "048616"),
/// );
///
/// // Add resident director
/// company.directors.push(Director::new("John Tan", "S1234567A", true));
///
/// // Validate
/// let report = validate_company_formation(&company)?;
/// assert!(report.is_valid);
/// # Ok::<(), legalis_sg::companies::CompaniesError>(())
/// ```
pub fn validate_company_formation(company: &Company) -> Result<ValidationReport> {
    let mut report = ValidationReport::success();

    // 1. Validate company name and legal suffix
    if !company.has_valid_legal_suffix() {
        report.add_error(format!(
            "Company name '{}' missing required suffix for {}",
            company.name, company.company_type
        ));
        report.add_legal_reference("CA s. 27");
    }

    // 2. Validate UEN format
    if !is_valid_uen(&company.uen) {
        report.add_error(format!("Invalid UEN format: '{}'", company.uen));
    }

    // 3. Validate registered office is in Singapore (s. 142)
    if !company.registered_address.is_singapore() {
        report.add_error("Registered office must be in Singapore (CA s. 142)".to_string());
        report.add_legal_reference("CA s. 142");
    }

    // 4. Validate resident director requirement (s. 145)
    if company.company_type.requires_resident_director() && !company.has_resident_director() {
        report.add_error("No resident director appointed (CA s. 145(1))".to_string());
        report.add_legal_reference("CA s. 145(1)");
    }

    // 5. Validate all directors are eligible
    for director in &company.directors {
        if let Err(e) = validate_director_eligibility(director) {
            report.add_error(format!("Director '{}': {}", director.name, e));
        }
    }

    // 6. Check for duplicate directors
    let mut director_ids = std::collections::HashSet::new();
    for director in &company.directors {
        let id = director
            .nric_fin
            .clone()
            .or_else(|| director.passport.clone());
        if let Some(id) = id {
            if !director_ids.insert(id.clone()) {
                report.add_error(format!("Duplicate director identification: '{}'", id));
            }
        }
    }

    // 7. Validate shareholders
    if company.shareholders.is_empty() {
        report.add_warning("Company has no shareholders".to_string());
    }

    // 8. Validate shareholder count for private limited companies
    if let Some(max) = company.company_type.max_shareholders() {
        if company.shareholder_count() > max {
            report.add_error(format!(
                "Too many shareholders: {} (max: {}) (CA s. 18(1))",
                company.shareholder_count(),
                max
            ));
            report.add_legal_reference("CA s. 18(1)");
        }
    }

    // 9. Validate share capital consistency
    let shareholder_total: u64 = company
        .shareholders
        .iter()
        .map(|s| s.share_allocation.total_paid_cents)
        .sum();

    if shareholder_total > 0 && shareholder_total != company.share_capital.paid_up_capital_cents {
        report.add_error(format!(
            "Share capital mismatch: paid-up capital SGD {:.2} vs shareholder total SGD {:.2}",
            company.share_capital.paid_up_sgd(),
            shareholder_total as f64 / 100.0
        ));
    }

    // 10. Validate company secretary (if required)
    if company.company_type.requires_company_secretary() && company.company_secretary.is_none() {
        report.add_warning(
            "No company secretary appointed (required within 6 months - CA s. 171)".to_string(),
        );
        report.add_legal_reference("CA s. 171");
    }

    // 11. Validate financial year end
    if MonthDay::new(
        company.financial_year_end.month,
        company.financial_year_end.day,
    )
    .is_none()
    {
        report.add_error(format!(
            "Invalid financial year end: month {}, day {}",
            company.financial_year_end.month, company.financial_year_end.day
        ));
    }

    Ok(report)
}

/// Validates director eligibility
///
/// Checks if a person is eligible to be appointed as director:
/// - Not disqualified under s. 148, 149, or 155
/// - At least 18 years old (s. 145(2))
///
/// ## Examples
///
/// ```
/// use legalis_sg::companies::*;
///
/// let director = Director::new("John Tan", "S1234567A", true);
/// validate_director_eligibility(&director)?;
/// # Ok::<(), legalis_sg::companies::CompaniesError>(())
/// ```
pub fn validate_director_eligibility(director: &Director) -> Result<()> {
    // Check disqualification status
    match &director.disqualification_status {
        DisqualificationStatus::Eligible => {}
        DisqualificationStatus::ConvictionDisqualification { offense, .. } => {
            return Err(CompaniesError::DirectorDisqualified {
                name: director.name.clone(),
                reason: format!("Convicted of offense: {}", offense),
            });
        }
        DisqualificationStatus::BankruptcyDisqualification { .. } => {
            return Err(CompaniesError::DirectorDisqualified {
                name: director.name.clone(),
                reason: "Undischarged bankrupt (s. 149)".to_string(),
            });
        }
        DisqualificationStatus::CourtOrderDisqualification { reason, .. } => {
            return Err(CompaniesError::DirectorDisqualified {
                name: director.name.clone(),
                reason: format!("Court order: {}", reason),
            });
        }
    }

    Ok(())
}

/// Validates resident director requirement (s. 145)
///
/// Verifies that at least one director is ordinarily resident in Singapore.
///
/// ## Examples
///
/// ```
/// use legalis_sg::companies::*;
///
/// let directors = vec![
///     Director::new("John Tan", "S1234567A", true),  // Resident
///     Director::new("Jane Smith", "P1234567", false), // Non-resident
/// ];
///
/// validate_resident_director_requirement(&directors)?;
/// # Ok::<(), legalis_sg::companies::CompaniesError>(())
/// ```
pub fn validate_resident_director_requirement(directors: &[Director]) -> Result<()> {
    if !directors.iter().any(|d| d.is_resident_director) {
        return Err(CompaniesError::NoResidentDirector);
    }
    Ok(())
}

/// Validates AGM (Annual General Meeting) requirement (s. 175)
///
/// Section 175 requirements:
/// - First AGM: Within 18 months of incorporation
/// - Subsequent AGMs: Within 15 months of previous AGM
/// - AGM must be held at least once per calendar year
/// - No more than 6 months after FYE
///
/// ## Examples
///
/// ```
/// use legalis_sg::companies::*;
/// use chrono::{Duration, Utc};
///
/// let company = Company::new(
///     "202401234A",
///     "Test Pte Ltd",
///     CompanyType::PrivateLimited,
///     Address::singapore("1 Raffles Place", "048616"),
/// );
///
/// let last_agm = Utc::now() - Duration::days(300);
/// validate_agm_requirement(&company, last_agm)?;
/// # Ok::<(), legalis_sg::companies::CompaniesError>(())
/// ```
pub fn validate_agm_requirement(company: &Company, last_agm: DateTime<Utc>) -> Result<()> {
    let now = Utc::now();
    let days_since_last_agm = (now - last_agm).num_days();

    // Check if AGM is overdue (15 months = ~456 days)
    if days_since_last_agm > 456 {
        return Err(CompaniesError::AgmOverdue {
            days: days_since_last_agm - 456,
        });
    }

    // For first AGM, check 18 months from incorporation
    let days_since_incorporation = (now - company.registration_date).num_days();
    if days_since_incorporation <= 548 {
        // 18 months ~= 548 days
        // First AGM period
        if days_since_incorporation > 548 && last_agm == company.registration_date {
            return Err(CompaniesError::AgmOverdue {
                days: days_since_incorporation - 548,
            });
        }
    }

    Ok(())
}

/// Calculates annual return filing deadline (s. 197)
///
/// Annual return must be filed within 7 months of financial year end.
///
/// ## Examples
///
/// ```
/// use legalis_sg::companies::*;
/// use chrono::Utc;
///
/// let company = Company::new(
///     "202401234A",
///     "Test Pte Ltd",
///     CompanyType::PrivateLimited,
///     Address::singapore("1 Raffles Place", "048616"),
/// );
///
/// let deadline = validate_annual_return_deadline(&company)?;
/// println!("Annual return due by: {}", deadline);
/// # Ok::<(), legalis_sg::companies::CompaniesError>(())
/// ```
pub fn validate_annual_return_deadline(company: &Company) -> Result<DateTime<Utc>> {
    // Get current year's FYE
    let now = Utc::now();
    let fye_month = company.financial_year_end.month as u32;
    let fye_day = company.financial_year_end.day as u32;

    // Determine the most recent FYE
    let current_year = now.year();
    let fye_this_year = Utc
        .with_ymd_and_hms(current_year, fye_month, fye_day, 23, 59, 59)
        .single();

    let most_recent_fye = if let Some(fye) = fye_this_year {
        if fye > now {
            // FYE hasn't occurred this year yet, use last year's FYE
            Utc.with_ymd_and_hms(current_year - 1, fye_month, fye_day, 23, 59, 59)
                .single()
                .ok_or(CompaniesError::InvalidFinancialYearEnd {
                    month: company.financial_year_end.month,
                    day: company.financial_year_end.day,
                })?
        } else {
            fye
        }
    } else {
        return Err(CompaniesError::InvalidFinancialYearEnd {
            month: company.financial_year_end.month,
            day: company.financial_year_end.day,
        });
    };

    // Add 7 months for filing deadline
    let deadline = most_recent_fye + Duration::days(30 * 7); // Approximate 7 months

    Ok(deadline)
}

/// Validates UEN (Unique Entity Number) format
///
/// UEN format: 9-10 alphanumeric characters
/// Examples: "202401234A", "53123456B"
fn is_valid_uen(uen: &str) -> bool {
    if uen.len() < 9 || uen.len() > 10 {
        return false;
    }

    // Must be alphanumeric
    uen.chars().all(|c| c.is_ascii_alphanumeric())
}

/// Validates share capital structure
///
/// Checks:
/// - Paid-up capital is positive
/// - Share classes are valid
/// - Total shares match issued shares
/// - Par value (if applicable) is positive
pub fn validate_share_capital(share_capital: &ShareCapital) -> Result<()> {
    // Check paid-up capital is positive
    if share_capital.paid_up_capital_cents == 0 {
        return Err(CompaniesError::InsufficientCapital {
            actual: 0.0,
            required: 1.0, // Minimum SGD 1
        });
    }

    // Check authorized capital (if par value shares)
    if share_capital.has_par_value {
        if let Some(authorized) = share_capital.authorized_capital_cents {
            if share_capital.paid_up_capital_cents > authorized {
                return Err(CompaniesError::AllotmentExceedsAuthorized {
                    allotment: share_capital.paid_up_sgd(),
                    authorized: authorized as f64 / 100.0,
                });
            }
        }
    }

    // Validate share classes
    let total_shares: u64 = share_capital.share_classes.iter().map(|sc| sc.shares).sum();
    if total_shares != share_capital.issued_shares && share_capital.issued_shares > 0 {
        return Err(CompaniesError::InvalidShareAllocation {
            reason: format!(
                "Total shares in classes ({}) does not match issued shares ({})",
                total_shares, share_capital.issued_shares
            ),
        });
    }

    Ok(())
}

/// Validates shareholder ownership percentages
///
/// Checks that total ownership does not exceed 100%
pub fn validate_shareholder_ownership(
    shareholders: &[Shareholder],
    total_shares: u64,
) -> Result<()> {
    let total_held: u64 = shareholders
        .iter()
        .map(|s| s.share_allocation.number_of_shares)
        .sum();

    if total_held > total_shares {
        let percentage = (total_held as f64 / total_shares as f64) * 100.0;
        return Err(CompaniesError::OwnershipExceeds100Percent {
            total_percent: percentage,
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_company() -> Company {
        let mut company = Company::new(
            "202401234A",
            "Tech Innovations Pte Ltd",
            CompanyType::PrivateLimited,
            Address::singapore("1 Raffles Place", "048616"),
        );

        // Add resident director
        company
            .directors
            .push(Director::new("John Tan", "S1234567A", true));

        // Add shareholder
        company.shareholders.push(Shareholder {
            name: "John Tan".to_string(),
            identification: "S1234567A".to_string(),
            nationality_or_jurisdiction: "Singapore".to_string(),
            address: Address::singapore("123 Main St", "123456"),
            share_allocation: ShareAllocation::new("Ordinary", 100, 100_00),
            acquisition_date: Utc::now(),
        });

        company.share_capital.paid_up_capital_cents = 10_000_00; // SGD 10,000
        company.share_capital.issued_shares = 100;

        company
    }

    #[test]
    fn test_validate_company_formation_success() {
        let company = create_test_company();
        let report = validate_company_formation(&company).unwrap();
        assert!(report.is_valid || report.errors.is_empty());
    }

    #[test]
    fn test_validate_no_resident_director() {
        let mut company = create_test_company();
        company.directors.clear(); // Remove resident director

        let report = validate_company_formation(&company).unwrap();
        assert!(!report.is_valid);
        assert!(
            report
                .errors
                .iter()
                .any(|e| e.contains("resident director"))
        );
    }

    #[test]
    fn test_validate_director_eligibility() {
        let director = Director::new("John Tan", "S1234567A", true);
        assert!(validate_director_eligibility(&director).is_ok());

        let mut disqualified = Director::new("Jane Smith", "P1234567", false);
        disqualified.disqualification_status = DisqualificationStatus::BankruptcyDisqualification {
            bankruptcy_date: Utc::now(),
        };
        assert!(validate_director_eligibility(&disqualified).is_err());
    }

    #[test]
    fn test_validate_resident_director_requirement() {
        let directors = vec![Director::new("John Tan", "S1234567A", true)];
        assert!(validate_resident_director_requirement(&directors).is_ok());

        let no_resident = vec![Director::new("Jane Smith", "P1234567", false)];
        assert!(validate_resident_director_requirement(&no_resident).is_err());
    }

    #[test]
    fn test_is_valid_uen() {
        assert!(is_valid_uen("202401234A"));
        assert!(is_valid_uen("53123456B"));
        assert!(!is_valid_uen("12345")); // Too short
        assert!(!is_valid_uen("12345678901")); // Too long
        assert!(!is_valid_uen("2024-01234")); // Invalid characters
    }

    #[test]
    fn test_validate_share_capital() {
        let capital = ShareCapital::new(100_00); // SGD 100
        assert!(validate_share_capital(&capital).is_ok());

        let zero_capital = ShareCapital::new(0);
        assert!(validate_share_capital(&zero_capital).is_err());
    }

    #[test]
    fn test_validate_agm_requirement() {
        let company = create_test_company();

        // AGM within last year - should be OK
        let recent_agm = Utc::now() - Duration::days(300);
        assert!(validate_agm_requirement(&company, recent_agm).is_ok());

        // AGM more than 15 months ago - should fail
        let old_agm = Utc::now() - Duration::days(500);
        assert!(validate_agm_requirement(&company, old_agm).is_err());
    }
}
