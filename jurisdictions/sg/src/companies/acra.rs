//! ACRA (Accounting and Corporate Regulatory Authority) Registration
//!
//! This module provides utilities for ACRA company registration and BizFile+ integration.
//!
//! ## ACRA Functions
//!
//! - **Company Registration**: Name reservation, UEN assignment
//! - **BizFile+**: Electronic filing system
//! - **Company Search**: Name availability checking
//! - **Annual Filing**: Return submission tracking
//!
//! ## Examples
//!
//! ```
//! use legalis_sg::companies::acra::*;
//!
//! // Check if company name is available
//! let name = "Tech Innovations Pte Ltd";
//! if is_valid_company_name(name) {
//!     println!("✅ Name is valid for registration");
//! }
//!
//! // Generate UEN
//! let uen = generate_uen(UenType::LocalCompany, 2024);
//! println!("Generated UEN: {}", uen);
//! ```

use super::error::{CompaniesError, Result};
use super::types::CompanyType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// UEN (Unique Entity Number) type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UenType {
    /// Local company (starts with year + sequence)
    LocalCompany,
    /// Business (starts with 5 or 6)
    Business,
    /// Local company registered before 2008
    PreUenCompany,
}

/// ACRA registration status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RegistrationStatus {
    /// Registration pending approval
    Pending,
    /// Registration approved
    Approved {
        /// UEN assigned
        uen: String,
        /// Registration date
        registration_date: DateTime<Utc>,
    },
    /// Registration rejected
    Rejected {
        /// Reason for rejection
        reason: String,
    },
    /// Name reserved (valid for 120 days)
    NameReserved {
        /// Reserved name
        name: String,
        /// Reservation expiry
        expiry_date: DateTime<Utc>,
    },
}

/// Company name validation rules
///
/// ACRA requirements for company names:
/// - Must be unique (not identical to existing companies)
/// - Must include appropriate suffix (Pte Ltd, Ltd, LLP)
/// - Cannot contain offensive words
/// - Cannot suggest government connection without approval
/// - Cannot be misleading about nature of business
pub fn is_valid_company_name(name: &str) -> bool {
    // Must not be empty
    if name.is_empty() {
        return false;
    }

    // Must not be too long (practical limit: 120 characters)
    if name.len() > 120 {
        return false;
    }

    // Must contain valid suffix
    let valid_suffixes = [
        "Pte Ltd",
        "Private Limited",
        "Ltd",
        "Limited",
        "LLP",
        "Limited Liability Partnership",
    ];

    let has_valid_suffix = valid_suffixes
        .iter()
        .any(|suffix| name.ends_with(suffix) || name.contains(&format!(" {} ", suffix)));

    if !has_valid_suffix {
        return false;
    }

    // Check for prohibited words (simplified list)
    let prohibited = [
        "government",
        "municipal",
        "parliament",
        "royal",
        "bank", // Requires MAS approval
    ];

    let name_lower = name.to_lowercase();
    for word in prohibited {
        if name_lower.contains(word) {
            return false;
        }
    }

    true
}

/// Validates company name format and suffix
pub fn validate_company_name(name: &str, company_type: CompanyType) -> Result<()> {
    if !is_valid_company_name(name) {
        return Err(CompaniesError::ValidationError {
            message: format!("Invalid company name: {}", name),
        });
    }

    let suffix = company_type.legal_suffix();
    if !suffix.is_empty() && !name.ends_with(suffix) && !name.contains(suffix) {
        return Err(CompaniesError::MissingLegalSuffix {
            name: name.to_string(),
            suffix: suffix.to_string(),
        });
    }

    Ok(())
}

/// Generates a sample UEN for testing
///
/// Note: In production, UEN is assigned by ACRA and cannot be self-generated.
/// This function is for testing and demonstration purposes only.
///
/// ## UEN Format
///
/// - Local companies (post-2008): YYYYNNNNNC (e.g., 202401234A)
///   - YYYY: Year of registration
///   - NNNNN: Sequential number
///   - C: Check digit
/// - Businesses: 5NNNNNNNC or 6NNNNNNNC
/// - Pre-UEN companies: NNNNNNNC (8 digits)
pub fn generate_uen(uen_type: UenType, year: i32) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    chrono::Utc::now().hash(&mut hasher);
    let random = hasher.finish();

    match uen_type {
        UenType::LocalCompany => {
            let sequence = (random % 100000) as u32;
            let check_digit = calculate_check_digit(&format!("{}{:05}", year, sequence));
            format!("{}{:05}{}", year, sequence, check_digit)
        }
        UenType::Business => {
            let sequence = (random % 10000000) as u32;
            let check_digit = calculate_check_digit(&format!("5{:07}", sequence));
            format!("5{:07}{}", sequence, check_digit)
        }
        UenType::PreUenCompany => {
            let sequence = (random % 10000000) as u32;
            let check_digit = calculate_check_digit(&format!("{:07}", sequence));
            format!("{:07}{}", sequence, check_digit)
        }
    }
}

/// Calculates check digit for UEN
fn calculate_check_digit(base: &str) -> char {
    // Simplified check digit calculation (real ACRA algorithm is proprietary)
    let sum: u32 = base.chars().filter_map(|c| c.to_digit(10)).sum();
    let check = (sum % 26) as u8;
    (b'A' + check) as char
}

/// Validates UEN format
pub fn validate_uen(uen: &str) -> Result<()> {
    if uen.len() < 9 || uen.len() > 10 {
        return Err(CompaniesError::InvalidUen {
            uen: uen.to_string(),
        });
    }

    // Must be alphanumeric
    if !uen.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err(CompaniesError::InvalidUen {
            uen: uen.to_string(),
        });
    }

    // Check format patterns
    let is_valid_format =
        // Local company: YYYYNNNNNC (10 chars)
        (uen.len() == 10 && uen[..4].chars().all(|c| c.is_ascii_digit())) ||
        // Business: 5NNNNNNNC or 6NNNNNNNC (9 chars)
        (uen.len() == 9 && (uen.starts_with('5') || uen.starts_with('6'))) ||
        // Pre-UEN company: NNNNNNNC (8-9 chars)
        (uen.len() >= 8 && uen.len() <= 9);

    if !is_valid_format {
        return Err(CompaniesError::InvalidUen {
            uen: uen.to_string(),
        });
    }

    Ok(())
}

/// ACRA filing requirement
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilingRequirement {
    /// Type of filing
    pub filing_type: FilingType,
    /// Due date
    pub due_date: DateTime<Utc>,
    /// Whether filing is overdue
    pub is_overdue: bool,
    /// Days until due (negative if overdue)
    pub days_until_due: i64,
}

/// Type of ACRA filing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FilingType {
    /// Annual return (s. 197)
    AnnualReturn,
    /// Change of directors
    DirectorChange,
    /// Change of shareholders
    ShareholderChange,
    /// Change of company secretary
    SecretaryChange,
    /// Change of registered address
    AddressChange,
    /// Change of share capital
    ShareCapitalChange,
    /// Annual General Meeting minutes
    AgmMinutes,
}

impl FilingType {
    /// Returns the statute reference for this filing
    pub fn statute_reference(&self) -> &'static str {
        match self {
            FilingType::AnnualReturn => "CA s. 197",
            FilingType::DirectorChange => "CA s. 145",
            FilingType::ShareholderChange => "CA s. 128",
            FilingType::SecretaryChange => "CA s. 171",
            FilingType::AddressChange => "CA s. 142",
            FilingType::ShareCapitalChange => "CA s. 64",
            FilingType::AgmMinutes => "CA s. 175",
        }
    }

    /// Returns typical deadline after event (in days)
    pub fn typical_deadline_days(&self) -> i64 {
        match self {
            FilingType::AnnualReturn => 210, // 7 months
            FilingType::DirectorChange => 14,
            FilingType::ShareholderChange => 14,
            FilingType::SecretaryChange => 14,
            FilingType::AddressChange => 14,
            FilingType::ShareCapitalChange => 30,
            FilingType::AgmMinutes => 30,
        }
    }
}

/// Checks if filing is overdue
pub fn is_filing_overdue(due_date: DateTime<Utc>) -> bool {
    Utc::now() > due_date
}

/// Calculates days until filing due
pub fn days_until_filing_due(due_date: DateTime<Utc>) -> i64 {
    (due_date - Utc::now()).num_days()
}

/// BizFile+ submission reference
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BizFileSubmission {
    /// Submission reference number
    pub reference_number: String,
    /// Submission date/time
    pub submission_date: DateTime<Utc>,
    /// Filing type
    pub filing_type: FilingType,
    /// Processing status
    pub status: BizFileStatus,
}

/// BizFile+ processing status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BizFileStatus {
    /// Submitted, pending processing
    Submitted,
    /// Under review by ACRA
    UnderReview,
    /// Approved
    Approved,
    /// Rejected
    Rejected,
    /// Requires clarification
    RequiresClarification,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_company_name() {
        assert!(is_valid_company_name("Tech Innovations Pte Ltd"));
        assert!(is_valid_company_name("Global Solutions Ltd"));
        assert!(is_valid_company_name("Consulting Partners LLP"));

        // Missing suffix
        assert!(!is_valid_company_name("Tech Innovations"));

        // Empty name
        assert!(!is_valid_company_name(""));

        // Prohibited word
        assert!(!is_valid_company_name("Government Services Pte Ltd"));
    }

    #[test]
    fn test_generate_uen() {
        let uen = generate_uen(UenType::LocalCompany, 2024);
        assert_eq!(uen.len(), 10);
        assert!(uen.starts_with("2024"));

        let business_uen = generate_uen(UenType::Business, 2024);
        assert_eq!(business_uen.len(), 9);
        assert!(business_uen.starts_with('5'));
    }

    #[test]
    fn test_validate_uen() {
        assert!(validate_uen("202401234A").is_ok());
        assert!(validate_uen("53123456B").is_ok());

        assert!(validate_uen("12345").is_err()); // Too short
        assert!(validate_uen("2024-01234").is_err()); // Invalid chars
    }

    #[test]
    fn test_filing_type_deadline() {
        assert_eq!(FilingType::AnnualReturn.typical_deadline_days(), 210);
        assert_eq!(FilingType::DirectorChange.typical_deadline_days(), 14);
    }

    #[test]
    fn test_validate_company_name() {
        let result = validate_company_name("Tech Pte Ltd", CompanyType::PrivateLimited);
        assert!(result.is_ok());

        let missing_suffix = validate_company_name("Tech", CompanyType::PrivateLimited);
        assert!(missing_suffix.is_err());
    }
}
