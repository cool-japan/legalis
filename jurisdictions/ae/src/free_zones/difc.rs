//! Dubai International Financial Centre (DIFC) Law
//!
//! DIFC is a financial free zone in Dubai operating under **Common Law** (English law),
//! not UAE Federal Law. It has its own independent legal system.
//!
//! ## Key Features
//!
//! - Common Law jurisdiction (based on English law)
//! - Independent DIFC Courts (separate from UAE Federal Courts)
//! - DIFC Arbitration Centre
//! - Financial Services Regulatory Authority (DFSA)
//! - 100% foreign ownership
//! - 0% corporate tax (50-year guarantee)
//! - No restrictions on capital repatriation
//!
//! ## Governing Laws
//!
//! - DIFC Law No. 6/2021 - Companies Law
//! - DIFC Law No. 5/2005 - Insolvency Law
//! - DIFC Law No. 9/2004 - Contract Law
//! - DIFC Law No. 5/2004 - Law of Damages
//! - DIFC Law No. 10/2004 - Employment Law

use crate::common::Aed;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Result type for DIFC operations
pub type DifcResult<T> = Result<T, DifcError>;

/// DIFC company types under DIFC Companies Law
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DifcCompanyType {
    /// Private Company Limited by Shares
    PrivateCompanyLimitedByShares,
    /// Public Company Limited by Shares
    PublicCompanyLimitedByShares,
    /// Company Limited by Guarantee
    CompanyLimitedByGuarantee,
    /// Protected Cell Company (PCC)
    ProtectedCellCompany,
    /// Limited Liability Partnership (LLP)
    LimitedLiabilityPartnership,
    /// General Partnership
    GeneralPartnership,
    /// Limited Partnership
    LimitedPartnership,
    /// Branch of Foreign Company
    ForeignBranch,
}

impl DifcCompanyType {
    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::PrivateCompanyLimitedByShares => "Private Company Limited by Shares",
            Self::PublicCompanyLimitedByShares => "Public Company Limited by Shares",
            Self::CompanyLimitedByGuarantee => "Company Limited by Guarantee",
            Self::ProtectedCellCompany => "Protected Cell Company",
            Self::LimitedLiabilityPartnership => "Limited Liability Partnership",
            Self::GeneralPartnership => "General Partnership",
            Self::LimitedPartnership => "Limited Partnership",
            Self::ForeignBranch => "Branch of Foreign Company",
        }
    }

    /// Get abbreviation
    pub fn abbreviation(&self) -> &'static str {
        match self {
            Self::PrivateCompanyLimitedByShares => "Ltd.",
            Self::PublicCompanyLimitedByShares => "PLC",
            Self::CompanyLimitedByGuarantee => "CLG",
            Self::ProtectedCellCompany => "PCC",
            Self::LimitedLiabilityPartnership => "LLP",
            Self::GeneralPartnership => "GP",
            Self::LimitedPartnership => "LP",
            Self::ForeignBranch => "Branch",
        }
    }

    /// Minimum share capital (if applicable)
    pub fn minimum_capital(&self) -> Option<Aed> {
        match self {
            Self::PrivateCompanyLimitedByShares => Some(Aed::from_dirhams(1)), // USD 1 equivalent
            Self::PublicCompanyLimitedByShares => Some(Aed::from_dirhams(3_670)), // USD 1,000 equivalent
            Self::ProtectedCellCompany => Some(Aed::from_dirhams(367_250)), // USD 100,000 equivalent
            _ => None,
        }
    }

    /// Minimum number of shareholders/partners
    pub fn minimum_members(&self) -> u32 {
        match self {
            Self::PrivateCompanyLimitedByShares => 1,
            Self::PublicCompanyLimitedByShares => 2,
            Self::CompanyLimitedByGuarantee => 1,
            Self::ProtectedCellCompany => 1,
            Self::LimitedLiabilityPartnership => 2,
            Self::GeneralPartnership => 2,
            Self::LimitedPartnership => 2,
            Self::ForeignBranch => 0,
        }
    }

    /// Maximum number of shareholders (if limited)
    pub fn maximum_members(&self) -> Option<u32> {
        match self {
            Self::PrivateCompanyLimitedByShares => Some(50),
            _ => None,
        }
    }
}

/// DIFC regulated financial activities
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DifcFinancialActivity {
    /// Banking (requires DFSA license)
    Banking,
    /// Insurance (conventional)
    Insurance,
    /// Islamic Finance
    IslamicFinance,
    /// Investment Management
    InvestmentManagement,
    /// Securities Dealing
    SecuritiesDealing,
    /// Fund Management
    FundManagement,
    /// Trust Services
    TrustServices,
    /// Custody Services
    CustodyServices,
    /// Financial Advisory
    FinancialAdvisory,
    /// Ancillary Services (non-regulated support services)
    AncillaryServices,
}

impl DifcFinancialActivity {
    /// Check if activity requires DFSA license
    pub fn requires_dfsa_license(&self) -> bool {
        !matches!(self, Self::AncillaryServices)
    }

    /// Get activity name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Banking => "Banking",
            Self::Insurance => "Insurance",
            Self::IslamicFinance => "Islamic Finance",
            Self::InvestmentManagement => "Investment Management",
            Self::SecuritiesDealing => "Securities Dealing",
            Self::FundManagement => "Fund Management",
            Self::TrustServices => "Trust Services",
            Self::CustodyServices => "Custody Services",
            Self::FinancialAdvisory => "Financial Advisory",
            Self::AncillaryServices => "Ancillary Services",
        }
    }
}

/// DIFC Employment Contract details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DifcEmployment {
    /// Basic salary (monthly)
    pub basic_salary: Aed,
    /// Contract term (months, 0 = indefinite)
    pub term_months: u32,
    /// Notice period (days)
    pub notice_period_days: u32,
    /// Annual leave days
    pub annual_leave_days: u32,
    /// Is probation period
    pub is_probation: bool,
    /// Probation duration (days)
    pub probation_days: u32,
}

impl DifcEmployment {
    /// Create standard DIFC employment contract
    pub fn standard(basic_salary: Aed) -> Self {
        Self {
            basic_salary,
            term_months: 0, // Indefinite
            notice_period_days: 30,
            annual_leave_days: 30,
            is_probation: true,
            probation_days: 90, // Up to 6 months allowed
        }
    }

    /// Validate contract under DIFC Employment Law
    pub fn is_valid(&self) -> DifcResult<()> {
        if self.probation_days > 180 {
            return Err(DifcError::InvalidEmployment {
                reason: "Probation period cannot exceed 6 months".to_string(),
            });
        }

        if self.annual_leave_days < 30 {
            return Err(DifcError::InvalidEmployment {
                reason: "Minimum annual leave is 30 days".to_string(),
            });
        }

        Ok(())
    }

    /// Calculate end of service gratuity
    ///
    /// DIFC: 21 days for first 5 years, 30 days thereafter
    /// (Similar to UAE Federal Law but with some differences)
    pub fn calculate_gratuity(&self, years_of_service: u32) -> Aed {
        if years_of_service == 0 {
            return Aed::from_fils(0);
        }

        let daily_wage = Aed::from_fils(self.basic_salary.fils() / 30);

        let years_first_five = years_of_service.min(5);
        let years_after_five = years_of_service.saturating_sub(5);

        let gratuity_first_five = daily_wage.fils() * 21 * years_first_five as i64;
        let gratuity_after_five = daily_wage.fils() * 30 * years_after_five as i64;

        Aed::from_fils(gratuity_first_five + gratuity_after_five)
    }
}

/// DIFC Court jurisdiction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DifcCourtJurisdiction {
    /// Case involves DIFC entity
    pub involves_difc_entity: bool,
    /// Case involves DIFC property
    pub involves_difc_property: bool,
    /// Parties opted for DIFC Courts
    pub parties_opted_in: bool,
    /// Claim amount (if applicable)
    pub claim_amount: Option<Aed>,
}

impl DifcCourtJurisdiction {
    /// Check if DIFC Courts have jurisdiction
    pub fn has_jurisdiction(&self) -> bool {
        self.involves_difc_entity || self.involves_difc_property || self.parties_opted_in
    }

    /// Get applicable court tier
    pub fn court_tier(&self) -> &'static str {
        if let Some(amount) = self.claim_amount
            && amount.dirhams() < 500_000
        {
            "Small Claims Tribunal"
        } else {
            "Court of First Instance"
        }
    }
}

/// DIFC errors
#[derive(Debug, Error)]
pub enum DifcError {
    /// Invalid company registration
    #[error("Invalid DIFC company registration: {reason}")]
    InvalidRegistration { reason: String },

    /// DFSA license required
    #[error("DFSA license required for activity: {activity}")]
    DfsaLicenseRequired { activity: String },

    /// Invalid employment contract
    #[error("Invalid DIFC employment contract: {reason}")]
    InvalidEmployment { reason: String },

    /// Jurisdiction error
    #[error("DIFC Courts lack jurisdiction: {reason}")]
    JurisdictionError { reason: String },
}

/// Validate DIFC company registration
pub fn validate_company_registration(
    company_type: &DifcCompanyType,
    members: u32,
    capital: Aed,
) -> DifcResult<()> {
    let min_members = company_type.minimum_members();
    if members < min_members {
        return Err(DifcError::InvalidRegistration {
            reason: format!("Minimum {} members required, got {}", min_members, members),
        });
    }

    if let Some(max) = company_type.maximum_members()
        && members > max
    {
        return Err(DifcError::InvalidRegistration {
            reason: format!("Maximum {} members allowed, got {}", max, members),
        });
    }

    if let Some(min_capital) = company_type.minimum_capital()
        && capital.fils() < min_capital.fils()
    {
        return Err(DifcError::InvalidRegistration {
            reason: format!("Minimum capital {} required", min_capital.format_en()),
        });
    }

    Ok(())
}

/// Get DIFC benefits checklist
pub fn get_difc_benefits() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Common Law", "English law-based legal system"),
        ("DIFC Courts", "Independent common law courts"),
        ("100% Ownership", "Full foreign ownership allowed"),
        ("0% Tax", "Corporate tax exemption (50-year guarantee)"),
        (
            "Capital Repatriation",
            "No restrictions on profit repatriation",
        ),
        ("DFSA Regulation", "World-class financial regulator"),
        ("DIFC Arbitration", "International arbitration centre"),
        ("Strategic Location", "Gateway to MEASA markets"),
        ("Talent Pool", "Access to regional financial professionals"),
        ("Innovation Hub", "FinTech and innovation initiatives"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_difc_company_types() {
        let private = DifcCompanyType::PrivateCompanyLimitedByShares;
        assert_eq!(private.abbreviation(), "Ltd.");
        assert_eq!(private.minimum_members(), 1);
        assert_eq!(private.maximum_members(), Some(50));
    }

    #[test]
    fn test_difc_minimum_capital() {
        let pcc = DifcCompanyType::ProtectedCellCompany;
        assert!(pcc.minimum_capital().is_some());
        let min_cap = pcc
            .minimum_capital()
            .expect("PCC should have minimum capital");
        assert!(min_cap.dirhams() > 100_000);
    }

    #[test]
    fn test_financial_activities() {
        let banking = DifcFinancialActivity::Banking;
        assert!(banking.requires_dfsa_license());

        let ancillary = DifcFinancialActivity::AncillaryServices;
        assert!(!ancillary.requires_dfsa_license());
    }

    #[test]
    fn test_difc_employment() {
        let employment = DifcEmployment::standard(Aed::from_dirhams(15_000));
        assert!(employment.is_valid().is_ok());
        assert_eq!(employment.annual_leave_days, 30);
        assert_eq!(employment.notice_period_days, 30);
    }

    #[test]
    fn test_difc_employment_invalid_probation() {
        let mut employment = DifcEmployment::standard(Aed::from_dirhams(10_000));
        employment.probation_days = 200; // Exceeds 180 day limit
        assert!(employment.is_valid().is_err());
    }

    #[test]
    fn test_difc_gratuity_calculation() {
        let employment = DifcEmployment::standard(Aed::from_dirhams(10_000));
        let gratuity = employment.calculate_gratuity(3);
        assert!(gratuity.dirhams() > 0);
    }

    #[test]
    fn test_difc_court_jurisdiction() {
        let case = DifcCourtJurisdiction {
            involves_difc_entity: true,
            involves_difc_property: false,
            parties_opted_in: false,
            claim_amount: Some(Aed::from_dirhams(300_000)),
        };

        assert!(case.has_jurisdiction());
        assert_eq!(case.court_tier(), "Small Claims Tribunal");

        let large_case = DifcCourtJurisdiction {
            involves_difc_entity: true,
            involves_difc_property: false,
            parties_opted_in: false,
            claim_amount: Some(Aed::from_dirhams(2_000_000)),
        };

        assert_eq!(large_case.court_tier(), "Court of First Instance");
    }

    #[test]
    fn test_validate_company_registration() {
        let result = validate_company_registration(
            &DifcCompanyType::PrivateCompanyLimitedByShares,
            2,
            Aed::from_dirhams(1000),
        );
        assert!(result.is_ok());

        // Too many members for private company
        let result_invalid = validate_company_registration(
            &DifcCompanyType::PrivateCompanyLimitedByShares,
            100,
            Aed::from_dirhams(1000),
        );
        assert!(result_invalid.is_err());
    }

    #[test]
    fn test_difc_benefits() {
        let benefits = get_difc_benefits();
        assert!(!benefits.is_empty());
        assert!(benefits.len() >= 10);
    }

    #[test]
    fn test_opt_in_jurisdiction() {
        let case = DifcCourtJurisdiction {
            involves_difc_entity: false,
            involves_difc_property: false,
            parties_opted_in: true, // Parties can opt into DIFC Courts
            claim_amount: None,
        };

        assert!(case.has_jurisdiction());
    }
}
