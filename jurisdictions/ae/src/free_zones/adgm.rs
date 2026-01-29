//! Abu Dhabi Global Market (ADGM) Law
//!
//! ADGM is Abu Dhabi's international financial centre operating under **Common Law** (English law),
//! similar to DIFC but independent and located in Abu Dhabi.
//!
//! ## Key Features
//!
//! - Common Law jurisdiction (based on English law)
//! - Independent ADGM Courts (separate from UAE Federal Courts)
//! - ADGM Arbitration Centre
//! - Financial Services Regulatory Authority (FSRA)
//! - 100% foreign ownership
//! - 0% corporate tax (50-year guarantee)
//! - No personal income tax
//!
//! ## Governing Regulations
//!
//! - ADGM Companies Regulations 2020
//! - ADGM Employment Regulations 2019
//! - ADGM Insolvency Regulations 2015
//! - ADGM Data Protection Regulations 2021
//! - FSRA Financial Services and Markets Regulations

use crate::common::Aed;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Result type for ADGM operations
pub type AdgmResult<T> = Result<T, AdgmError>;

/// ADGM company types under ADGM Companies Regulations
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AdgmCompanyType {
    /// Private Company Limited by Shares (SPV)
    PrivateCompanyLimitedByShares,
    /// Public Company Limited by Shares
    PublicCompanyLimitedByShares,
    /// Company Limited by Guarantee
    CompanyLimitedByGuarantee,
    /// Protected Cell Company (PCC)
    ProtectedCellCompany,
    /// Limited Liability Company (LLC)
    LimitedLiabilityCompany,
    /// Limited Liability Partnership (LLP)
    LimitedLiabilityPartnership,
    /// Limited Partnership
    LimitedPartnership,
    /// Branch of Foreign Company
    ForeignBranch,
}

impl AdgmCompanyType {
    /// Get English name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::PrivateCompanyLimitedByShares => "Private Company Limited by Shares",
            Self::PublicCompanyLimitedByShares => "Public Company Limited by Shares",
            Self::CompanyLimitedByGuarantee => "Company Limited by Guarantee",
            Self::ProtectedCellCompany => "Protected Cell Company",
            Self::LimitedLiabilityCompany => "Limited Liability Company",
            Self::LimitedLiabilityPartnership => "Limited Liability Partnership",
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
            Self::LimitedLiabilityCompany => "LLC",
            Self::LimitedLiabilityPartnership => "LLP",
            Self::LimitedPartnership => "LP",
            Self::ForeignBranch => "Branch",
        }
    }

    /// Minimum share capital (if applicable)
    pub fn minimum_capital(&self) -> Option<Aed> {
        match self {
            Self::PrivateCompanyLimitedByShares => Some(Aed::from_fils(100)), // Nominal USD 1
            Self::PublicCompanyLimitedByShares => Some(Aed::from_dirhams(3_670)), // USD 1,000
            Self::ProtectedCellCompany => Some(Aed::from_dirhams(367_250)),   // USD 100,000
            Self::LimitedLiabilityCompany => Some(Aed::from_fils(100)),
            _ => None,
        }
    }

    /// Minimum number of members
    pub fn minimum_members(&self) -> u32 {
        match self {
            Self::PrivateCompanyLimitedByShares => 1,
            Self::PublicCompanyLimitedByShares => 2,
            Self::CompanyLimitedByGuarantee => 1,
            Self::ProtectedCellCompany => 1,
            Self::LimitedLiabilityCompany => 1,
            Self::LimitedLiabilityPartnership => 2,
            Self::LimitedPartnership => 2,
            Self::ForeignBranch => 0,
        }
    }
}

/// ADGM regulated financial activities
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AdgmFinancialActivity {
    /// Banking
    Banking,
    /// Insurance and Reinsurance
    Insurance,
    /// Islamic Finance
    IslamicFinance,
    /// Asset Management
    AssetManagement,
    /// Securities Trading
    SecuritiesTrading,
    /// Funds Management
    FundsManagement,
    /// Trust and Fiduciary Services
    TrustServices,
    /// Custody and Clearing
    CustodyServices,
    /// Commodities Trading
    CommoditiesTrading,
    /// FinTech (Financial Technology)
    FinTech,
}

impl AdgmFinancialActivity {
    /// Check if activity requires FSRA authorization
    pub fn requires_fsra_authorization(&self) -> bool {
        true // All listed activities require FSRA authorization
    }

    /// Get activity name
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Banking => "Banking",
            Self::Insurance => "Insurance and Reinsurance",
            Self::IslamicFinance => "Islamic Finance",
            Self::AssetManagement => "Asset Management",
            Self::SecuritiesTrading => "Securities Trading",
            Self::FundsManagement => "Funds Management",
            Self::TrustServices => "Trust and Fiduciary Services",
            Self::CustodyServices => "Custody and Clearing",
            Self::CommoditiesTrading => "Commodities Trading",
            Self::FinTech => "Financial Technology (FinTech)",
        }
    }
}

/// ADGM Employment details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdgmEmployment {
    /// Basic salary (monthly)
    pub basic_salary: Aed,
    /// Contract type (fixed-term or indefinite)
    pub is_fixed_term: bool,
    /// Contract duration (months, if fixed-term)
    pub term_months: Option<u32>,
    /// Notice period (days)
    pub notice_period_days: u32,
    /// Annual leave days
    pub annual_leave_days: u32,
    /// Probation period (days)
    pub probation_days: u32,
}

impl AdgmEmployment {
    /// Create standard ADGM employment contract
    pub fn standard(basic_salary: Aed) -> Self {
        Self {
            basic_salary,
            is_fixed_term: false,
            term_months: None,
            notice_period_days: 30,
            annual_leave_days: 22, // 22 working days minimum
            probation_days: 90,    // Up to 6 months allowed
        }
    }

    /// Validate contract under ADGM Employment Regulations
    pub fn is_valid(&self) -> AdgmResult<()> {
        if self.probation_days > 180 {
            return Err(AdgmError::InvalidEmployment {
                reason: "Probation period cannot exceed 6 months".to_string(),
            });
        }

        if self.annual_leave_days < 22 {
            return Err(AdgmError::InvalidEmployment {
                reason: "Minimum annual leave is 22 working days".to_string(),
            });
        }

        if self.notice_period_days < 30 {
            return Err(AdgmError::InvalidEmployment {
                reason: "Minimum notice period is 30 days".to_string(),
            });
        }

        Ok(())
    }

    /// Calculate end of employment payment
    ///
    /// ADGM: 21 days for first 5 years, 30 days thereafter
    pub fn calculate_end_of_employment_payment(&self, years_of_service: u32) -> Aed {
        if years_of_service == 0 {
            return Aed::from_fils(0);
        }

        let daily_wage = Aed::from_fils(self.basic_salary.fils() / 30);

        let years_first_five = years_of_service.min(5);
        let years_after_five = years_of_service.saturating_sub(5);

        let payment_first_five = daily_wage.fils() * 21 * years_first_five as i64;
        let payment_after_five = daily_wage.fils() * 30 * years_after_five as i64;

        Aed::from_fils(payment_first_five + payment_after_five)
    }
}

/// ADGM Court jurisdiction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdgmCourtJurisdiction {
    /// Case involves ADGM entity
    pub involves_adgm_entity: bool,
    /// Case involves ADGM contract
    pub involves_adgm_contract: bool,
    /// Parties opted for ADGM Courts
    pub parties_opted_in: bool,
    /// Claim amount
    pub claim_amount: Option<Aed>,
}

impl AdgmCourtJurisdiction {
    /// Check if ADGM Courts have jurisdiction
    pub fn has_jurisdiction(&self) -> bool {
        self.involves_adgm_entity || self.involves_adgm_contract || self.parties_opted_in
    }

    /// Get applicable court division
    pub fn court_division(&self) -> &'static str {
        if let Some(amount) = self.claim_amount
            && amount.dirhams() < 367_250
        {
            // Less than USD 100,000
            "Small Claims Division"
        } else {
            "Court of First Instance"
        }
    }
}

/// ADGM Data Protection compliance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdgmDataProtection {
    /// Is data controller
    pub is_controller: bool,
    /// Is data processor
    pub is_processor: bool,
    /// Processes personal data
    pub processes_personal_data: bool,
    /// Has appointed Data Protection Officer
    pub has_dpo: bool,
}

impl AdgmDataProtection {
    /// Check if DPO appointment is required
    pub fn requires_dpo(&self) -> bool {
        self.processes_personal_data && (self.is_controller || self.is_processor)
    }

    /// Validate ADGM data protection compliance
    pub fn is_compliant(&self) -> AdgmResult<()> {
        if self.requires_dpo() && !self.has_dpo {
            return Err(AdgmError::DataProtectionViolation {
                requirement: "Data Protection Officer must be appointed".to_string(),
            });
        }
        Ok(())
    }
}

/// ADGM errors
#[derive(Debug, Error)]
pub enum AdgmError {
    /// Invalid company registration
    #[error("Invalid ADGM company registration: {reason}")]
    InvalidRegistration { reason: String },

    /// FSRA authorization required
    #[error("FSRA authorization required for activity: {activity}")]
    FsraAuthorizationRequired { activity: String },

    /// Invalid employment contract
    #[error("Invalid ADGM employment contract: {reason}")]
    InvalidEmployment { reason: String },

    /// Jurisdiction error
    #[error("ADGM Courts lack jurisdiction: {reason}")]
    JurisdictionError { reason: String },

    /// Data protection violation
    #[error("ADGM Data Protection violation: {requirement}")]
    DataProtectionViolation { requirement: String },
}

/// Validate ADGM company registration
pub fn validate_company_registration(
    company_type: &AdgmCompanyType,
    members: u32,
    capital: Aed,
) -> AdgmResult<()> {
    let min_members = company_type.minimum_members();
    if members < min_members {
        return Err(AdgmError::InvalidRegistration {
            reason: format!("Minimum {} members required, got {}", min_members, members),
        });
    }

    if let Some(min_capital) = company_type.minimum_capital()
        && capital.fils() < min_capital.fils()
    {
        return Err(AdgmError::InvalidRegistration {
            reason: format!("Minimum capital {} required", min_capital.format_en()),
        });
    }

    Ok(())
}

/// Get ADGM benefits checklist
pub fn get_adgm_benefits() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Common Law", "English common law jurisdiction"),
        ("ADGM Courts", "Independent common law courts"),
        ("100% Ownership", "Full foreign ownership"),
        ("0% Tax", "Corporate and personal income tax exemption"),
        ("FSRA Regulation", "World-class financial regulator"),
        ("RegLab", "Regulatory sandbox for FinTech"),
        ("Strategic Hub", "Gateway to regional markets"),
        ("Data Protection", "GDPR-aligned data protection law"),
        ("Digital Assets", "Progressive crypto/blockchain framework"),
        (
            "Abu Dhabi",
            "Capital city location with government proximity",
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adgm_company_types() {
        let llc = AdgmCompanyType::LimitedLiabilityCompany;
        assert_eq!(llc.abbreviation(), "LLC");
        assert_eq!(llc.minimum_members(), 1);
        assert!(llc.minimum_capital().is_some());
    }

    #[test]
    fn test_adgm_pcc_capital() {
        let pcc = AdgmCompanyType::ProtectedCellCompany;
        assert!(pcc.minimum_capital().is_some());
        let min_cap = pcc
            .minimum_capital()
            .expect("PCC should have minimum capital");
        assert!(min_cap.dirhams() > 100_000);
    }

    #[test]
    fn test_financial_activities() {
        let fintech = AdgmFinancialActivity::FinTech;
        assert!(fintech.requires_fsra_authorization());
        assert_eq!(fintech.name_en(), "Financial Technology (FinTech)");
    }

    #[test]
    fn test_adgm_employment() {
        let employment = AdgmEmployment::standard(Aed::from_dirhams(20_000));
        assert!(employment.is_valid().is_ok());
        assert_eq!(employment.annual_leave_days, 22);
    }

    #[test]
    fn test_adgm_employment_invalid_leave() {
        let mut employment = AdgmEmployment::standard(Aed::from_dirhams(15_000));
        employment.annual_leave_days = 15; // Below minimum 22
        assert!(employment.is_valid().is_err());
    }

    #[test]
    fn test_end_of_employment_payment() {
        let employment = AdgmEmployment::standard(Aed::from_dirhams(12_000));
        let payment = employment.calculate_end_of_employment_payment(6);
        assert!(payment.dirhams() > 0);
        // 5 years * 21 days + 1 year * 30 days
    }

    #[test]
    fn test_adgm_court_jurisdiction() {
        let case = AdgmCourtJurisdiction {
            involves_adgm_entity: true,
            involves_adgm_contract: false,
            parties_opted_in: false,
            claim_amount: Some(Aed::from_dirhams(200_000)),
        };

        assert!(case.has_jurisdiction());
        assert_eq!(case.court_division(), "Small Claims Division");
    }

    #[test]
    fn test_adgm_data_protection() {
        let mut dp = AdgmDataProtection {
            is_controller: true,
            is_processor: false,
            processes_personal_data: true,
            has_dpo: false,
        };

        assert!(dp.requires_dpo());
        assert!(dp.is_compliant().is_err());

        dp.has_dpo = true;
        assert!(dp.is_compliant().is_ok());
    }

    #[test]
    fn test_validate_company_registration() {
        let result = validate_company_registration(
            &AdgmCompanyType::PrivateCompanyLimitedByShares,
            1,
            Aed::from_dirhams(1000),
        );
        assert!(result.is_ok());

        let result_invalid = validate_company_registration(
            &AdgmCompanyType::ProtectedCellCompany,
            1,
            Aed::from_dirhams(50_000), // Below minimum
        );
        assert!(result_invalid.is_err());
    }

    #[test]
    fn test_adgm_benefits() {
        let benefits = get_adgm_benefits();
        assert!(!benefits.is_empty());
        assert!(benefits.len() >= 10);
    }

    #[test]
    fn test_opt_in_jurisdiction() {
        let case = AdgmCourtJurisdiction {
            involves_adgm_entity: false,
            involves_adgm_contract: false,
            parties_opted_in: true,
            claim_amount: None,
        };

        assert!(case.has_jurisdiction());
    }
}
