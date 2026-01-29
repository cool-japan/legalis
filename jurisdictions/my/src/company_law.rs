//! Companies Act 2016
//!
//! Malaysian company law governing incorporation, management, and dissolution of companies.
//!
//! # Key Provisions
//!
//! - **Section 12**: Incorporation of companies
//! - **Section 45**: Minimum share capital (RM 1)
//! - **Section 122**: Appointment of company secretary (within 30 days)
//! - **Section 195**: Directors' duties and responsibilities
//! - **Section 241**: Annual general meeting (AGM within 6 months of financial year-end)
//! - **Section 259**: Annual return filing
//!
//! # Company Types
//!
//! - **Sdn Bhd**: Private limited company (Sendirian Berhad)
//! - **Bhd**: Public limited company (Berhad)
//! - **LLP**: Limited liability partnership

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

/// Company law error types.
#[derive(Debug, Error)]
pub enum CompanyError {
    /// Invalid company formation.
    #[error("Invalid company formation: {reason}")]
    InvalidFormation { reason: String },

    /// Director requirement not met.
    #[error("Director requirement not met: {reason}")]
    DirectorRequirement { reason: String },

    /// Share capital issue.
    #[error("Share capital issue: {reason}")]
    ShareCapitalIssue { reason: String },

    /// Compliance violation.
    #[error("Compliance violation: {requirement}")]
    ComplianceViolation { requirement: String },
}

/// Result type for company law operations.
pub type Result<T> = std::result::Result<T, CompanyError>;

/// Company type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompanyType {
    /// Private limited company (Sdn Bhd).
    PrivateLimited,
    /// Public limited company (Bhd).
    PublicLimited,
    /// Limited liability partnership (LLP).
    LimitedLiabilityPartnership,
    /// Unlimited company.
    Unlimited,
}

impl CompanyType {
    /// Returns the suffix for the company name.
    #[must_use]
    pub fn suffix(self) -> &'static str {
        match self {
            Self::PrivateLimited => "Sdn Bhd",
            Self::PublicLimited => "Bhd",
            Self::LimitedLiabilityPartnership => "LLP",
            Self::Unlimited => "",
        }
    }
}

/// Director of a company.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Director {
    /// Director ID.
    pub id: Uuid,
    /// Director name.
    pub name: String,
    /// IC number.
    pub ic_number: String,
    /// Date of appointment.
    pub appointment_date: DateTime<Utc>,
    /// Whether director is Malaysian resident.
    pub resident: bool,
    /// Whether director has been disqualified.
    pub disqualified: bool,
}

impl Director {
    /// Creates a new director.
    #[must_use]
    pub fn new(name: impl Into<String>, ic_number: impl Into<String>, resident: bool) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            ic_number: ic_number.into(),
            appointment_date: Utc::now(),
            resident,
            disqualified: false,
        }
    }
}

/// Shareholder of a company.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Shareholder {
    /// Shareholder ID.
    pub id: Uuid,
    /// Shareholder name.
    pub name: String,
    /// IC/registration number.
    pub identification: String,
    /// Number of shares held.
    pub shares: u64,
}

impl Shareholder {
    /// Creates a new shareholder.
    #[must_use]
    pub fn new(name: impl Into<String>, identification: impl Into<String>, shares: u64) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            identification: identification.into(),
            shares,
        }
    }
}

/// Share capital of a company.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShareCapital {
    /// Amount in sen.
    pub amount_sen: i64,
    /// Currency (default MYR).
    pub currency: String,
}

impl ShareCapital {
    /// Creates a new share capital.
    #[must_use]
    pub fn new(amount_sen: i64) -> Self {
        Self {
            amount_sen,
            currency: "MYR".to_string(),
        }
    }

    /// Checks if share capital meets minimum requirement (RM 1).
    #[must_use]
    pub fn meets_minimum(&self) -> bool {
        self.amount_sen >= 100 // RM 1 = 100 sen
    }
}

/// Company under Companies Act 2016.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Company {
    /// Company ID.
    pub id: Uuid,
    /// Company name.
    pub name: String,
    /// Company type.
    pub company_type: CompanyType,
    /// Registration number (SSM number).
    pub registration_number: Option<String>,
    /// Directors.
    pub directors: Vec<Director>,
    /// Shareholders.
    pub shareholders: Vec<Shareholder>,
    /// Share capital.
    pub share_capital: ShareCapital,
    /// Registered address.
    pub registered_address: String,
    /// Date of incorporation.
    pub incorporation_date: DateTime<Utc>,
    /// Company secretary appointed.
    pub has_company_secretary: bool,
}

impl Company {
    /// Creates a company builder.
    #[must_use]
    pub fn builder() -> CompanyBuilder {
        CompanyBuilder::default()
    }

    /// Validates company formation.
    pub fn validate(&self) -> Result<ValidationReport> {
        validate_company_formation(self)
    }
}

/// Company builder.
#[derive(Debug, Clone, Default)]
pub struct CompanyBuilder {
    name: Option<String>,
    company_type: Option<CompanyType>,
    directors: Vec<Director>,
    shareholders: Vec<Shareholder>,
    share_capital: Option<ShareCapital>,
    registered_address: Option<String>,
    has_company_secretary: bool,
}

impl CompanyBuilder {
    /// Sets the company name.
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Sets the company type.
    #[must_use]
    pub fn company_type(mut self, company_type: CompanyType) -> Self {
        self.company_type = Some(company_type);
        self
    }

    /// Adds a director.
    #[must_use]
    pub fn add_director(mut self, director: Director) -> Self {
        self.directors.push(director);
        self
    }

    /// Adds a shareholder.
    #[must_use]
    pub fn add_shareholder(mut self, shareholder: Shareholder) -> Self {
        self.shareholders.push(shareholder);
        self
    }

    /// Sets the share capital.
    #[must_use]
    pub fn share_capital(mut self, share_capital: ShareCapital) -> Self {
        self.share_capital = Some(share_capital);
        self
    }

    /// Sets the registered address.
    #[must_use]
    pub fn registered_address(mut self, address: impl Into<String>) -> Self {
        self.registered_address = Some(address.into());
        self
    }

    /// Sets whether company secretary is appointed.
    #[must_use]
    pub fn has_company_secretary(mut self, has: bool) -> Self {
        self.has_company_secretary = has;
        self
    }

    /// Builds the company.
    pub fn build(self) -> Result<Company> {
        let name = self.name.ok_or_else(|| CompanyError::InvalidFormation {
            reason: "Company name not specified".to_string(),
        })?;

        let company_type = self
            .company_type
            .ok_or_else(|| CompanyError::InvalidFormation {
                reason: "Company type not specified".to_string(),
            })?;

        let share_capital = self
            .share_capital
            .ok_or_else(|| CompanyError::ShareCapitalIssue {
                reason: "Share capital not specified".to_string(),
            })?;

        let registered_address =
            self.registered_address
                .ok_or_else(|| CompanyError::InvalidFormation {
                    reason: "Registered address not specified".to_string(),
                })?;

        Ok(Company {
            id: Uuid::new_v4(),
            name,
            company_type,
            registration_number: None,
            directors: self.directors,
            shareholders: self.shareholders,
            share_capital,
            registered_address,
            incorporation_date: Utc::now(),
            has_company_secretary: self.has_company_secretary,
        })
    }
}

/// Validation report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    /// Whether company formation is valid.
    pub valid: bool,
    /// Issues found.
    pub issues: Vec<String>,
}

/// Validates company formation under Companies Act 2016.
pub fn validate_company_formation(company: &Company) -> Result<ValidationReport> {
    let mut issues = Vec::new();

    // At least 1 director (Section 195)
    if company.directors.is_empty() {
        issues.push("Company must have at least one director".to_string());
    }

    // Check for disqualified directors
    for director in &company.directors {
        if director.disqualified {
            issues.push(format!("Director '{}' is disqualified", director.name));
        }
    }

    // At least 1 shareholder
    if company.shareholders.is_empty() {
        issues.push("Company must have at least one shareholder".to_string());
    }

    // Minimum share capital (RM 1)
    if !company.share_capital.meets_minimum() {
        issues.push("Share capital must be at least RM 1 (Section 45)".to_string());
    }

    // Private limited company cannot have more than 50 shareholders
    if company.company_type == CompanyType::PrivateLimited && company.shareholders.len() > 50 {
        issues.push("Private limited company cannot have more than 50 shareholders".to_string());
    }

    let valid = issues.is_empty();

    Ok(ValidationReport { valid, issues })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_company_formation() {
        let director = Director::new("Ahmad bin Ali", "850123-01-5678", true);
        let shareholder = Shareholder::new("Ahmad bin Ali", "850123-01-5678", 100);
        let share_capital = ShareCapital::new(10000000); // RM 100,000

        let company = Company::builder()
            .name("Tech Innovations")
            .company_type(CompanyType::PrivateLimited)
            .add_director(director)
            .add_shareholder(shareholder)
            .share_capital(share_capital)
            .registered_address("Kuala Lumpur")
            .build()
            .expect("Valid company");

        let report = company.validate().expect("Validation succeeds");
        assert!(report.valid);
    }

    #[test]
    fn test_invalid_share_capital() {
        let director = Director::new("Ahmad bin Ali", "850123-01-5678", true);
        let shareholder = Shareholder::new("Ahmad bin Ali", "850123-01-5678", 100);
        let share_capital = ShareCapital::new(50); // Less than RM 1

        let company = Company::builder()
            .name("Tech Innovations")
            .company_type(CompanyType::PrivateLimited)
            .add_director(director)
            .add_shareholder(shareholder)
            .share_capital(share_capital)
            .registered_address("Kuala Lumpur")
            .build()
            .expect("Company built");

        let report = company.validate().expect("Validation succeeds");
        assert!(!report.valid);
        assert!(report.issues.iter().any(|i| i.contains("Share capital")));
    }
}
