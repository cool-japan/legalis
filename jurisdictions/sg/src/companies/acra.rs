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

/// Structural classification of a Singapore UEN.
///
/// ACRA issues UENs in three documented structural formats (see
/// <https://www.uen.gov.sg>). This enum captures which format a given string
/// matches. The classification is *structural* only: ACRA's check-digit
/// algorithm is proprietary and is therefore not verified here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UenFormat {
    /// Business registered with ACRA's Registry of Businesses.
    ///
    /// Format: `NNNNNNNNC` (9 characters) — 8 running digits followed by a
    /// check alphabet (e.g., `53123456B`).
    Business,

    /// Local company registered with ACRA's Registry of Companies.
    ///
    /// Format: `YYYYNNNNNC` (10 characters) — 4-digit year of incorporation,
    /// 5 running digits, then a check alphabet (e.g., `202401234A`).
    LocalCompany,

    /// Any other entity issued a new-style UEN (LLPs, societies, statutory
    /// bodies, healthcare institutions, representative offices, etc.).
    ///
    /// Format: `(T|S|R)YYTTNNNNC` (10 characters) — a century prefix
    /// (`T` = 20xx, `S` = 19xx, `R` = 18xx), a 2-digit year, a 2-letter
    /// entity-type code, 4 running digits, then a check alphabet
    /// (e.g., `T08LP1234C` for a 2008 LLP).
    OtherEntity,
}

/// Classifies a UEN string into one of ACRA's documented structural formats.
///
/// Returns [`None`] when the string matches none of the recognised formats.
/// Only the structure (length, digit/alpha placement, prefix) is validated;
/// the proprietary check digit is not recomputed.
///
/// ## Examples
///
/// ```
/// use legalis_sg::companies::acra::{classify_uen, UenFormat};
///
/// assert_eq!(classify_uen("202401234A"), Some(UenFormat::LocalCompany));
/// assert_eq!(classify_uen("53123456B"), Some(UenFormat::Business));
/// assert_eq!(classify_uen("T08LP1234C"), Some(UenFormat::OtherEntity));
/// assert_eq!(classify_uen("bad-uen"), None);
/// ```
pub fn classify_uen(uen: &str) -> Option<UenFormat> {
    let bytes = uen.as_bytes();
    match bytes.len() {
        9 => {
            // Business: 8 digits + check alphabet.
            let digits_ok = bytes[..8].iter().all(u8::is_ascii_digit);
            let check_ok = bytes[8].is_ascii_alphabetic();
            (digits_ok && check_ok).then_some(UenFormat::Business)
        }
        10 => {
            // Local company: YYYY + 5 digits + check alphabet.
            let year_ok = bytes[..4].iter().all(u8::is_ascii_digit);
            let seq_ok = bytes[4..9].iter().all(u8::is_ascii_digit);
            let check_ok = bytes[9].is_ascii_alphabetic();
            if year_ok && seq_ok && check_ok {
                return Some(UenFormat::LocalCompany);
            }

            // Other entity: (T|S|R) + YY + 2-letter type + 4 digits + check alphabet.
            let prefix_ok = matches!(bytes[0], b'T' | b'S' | b'R');
            let year_ok = bytes[1..3].iter().all(u8::is_ascii_digit);
            let type_ok = bytes[3..5].iter().all(u8::is_ascii_alphabetic);
            let seq_ok = bytes[5..9].iter().all(u8::is_ascii_digit);
            let check_ok = bytes[9].is_ascii_alphabetic();
            (prefix_ok && year_ok && type_ok && seq_ok && check_ok)
                .then_some(UenFormat::OtherEntity)
        }
        _ => None,
    }
}

/// Validates that a UEN matches one of ACRA's documented structural formats.
///
/// See [`classify_uen`] for the recognised formats. Returns
/// [`CompaniesError::InvalidUen`] when the string is not a structurally valid
/// UEN.
pub fn validate_uen(uen: &str) -> Result<()> {
    if classify_uen(uen).is_some() {
        Ok(())
    } else {
        Err(CompaniesError::InvalidUen {
            uen: uen.to_string(),
        })
    }
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
    fn test_classify_uen_business() {
        // ROB business: 8 digits + check alphabet.
        assert_eq!(classify_uen("53123456B"), Some(UenFormat::Business));
        assert_eq!(classify_uen("00000001Z"), Some(UenFormat::Business));
        // Missing check alphabet (all numeric).
        assert_eq!(classify_uen("531234567"), None);
        // Letter inside the digit run.
        assert_eq!(classify_uen("5312A456B"), None);
    }

    #[test]
    fn test_classify_uen_local_company() {
        // ROC local company: YYYY + 5 digits + check alphabet.
        assert_eq!(classify_uen("202401234A"), Some(UenFormat::LocalCompany));
        assert_eq!(classify_uen("199912345Z"), Some(UenFormat::LocalCompany));
        // 10 chars but all numeric (no check alphabet).
        assert_eq!(classify_uen("2024012345"), None);
    }

    #[test]
    fn test_classify_uen_other_entity() {
        // New-style UEN for other entities: (T|S|R) + YY + 2 letters + 4 digits + check.
        assert_eq!(classify_uen("T08LP1234C"), Some(UenFormat::OtherEntity)); // 2008 LLP
        assert_eq!(classify_uen("S98SS0001A"), Some(UenFormat::OtherEntity)); // 1998 society
        assert_eq!(classify_uen("R88PQ9999X"), Some(UenFormat::OtherEntity)); // 1888 prefix
        // Invalid century prefix.
        assert_eq!(classify_uen("X08LP1234C"), None);
        // Entity-type code must be alphabetic.
        assert_eq!(classify_uen("T0812341234"), None);
    }

    #[test]
    fn test_classify_uen_rejects_malformed() {
        assert_eq!(classify_uen(""), None);
        assert_eq!(classify_uen("12345"), None); // Too short
        assert_eq!(classify_uen("12345678901"), None); // Too long
        assert_eq!(classify_uen("2024-01234"), None); // Punctuation
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
