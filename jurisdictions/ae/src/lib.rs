//! # Legalis-AE: United Arab Emirates Jurisdiction Support
//!
//! Comprehensive UAE legal system implementation for Legalis-RS.
//!
//! ## Legal System Overview
//!
//! The UAE operates under a **Civil Law** system influenced by:
//! - Egyptian (and thus French Napoleonic) legal tradition
//! - Islamic Sharia principles
//! - Common Law in free zones (DIFC, ADGM)
//!
//! The UAE consists of 7 emirates, each with some legislative autonomy,
//! but most commercial laws are federal.
//!
//! ## Hierarchy of Laws
//!
//! 1. **الدستور** - Constitution (1971, amended 2009)
//! 2. **القوانين الاتحادية** - Federal Laws
//! 3. **المراسيم بقوانين** - Federal Decree-Laws
//! 4. **قرارات مجلس الوزراء** - Cabinet Resolutions
//! 5. **القرارات الوزارية** - Ministerial Decisions
//! 6. **القوانين المحلية** - Local (Emirate) Laws
//!
//! ## Free Zones
//!
//! UAE has 40+ free zones with special legal frameworks:
//! - **DIFC** (Dubai International Financial Centre) - Common Law
//! - **ADGM** (Abu Dhabi Global Market) - Common Law
//! - Other free zones (JAFZA, DMCC, etc.) - UAE Federal Law
//!
//! ## Modules
//!
//! ### [`citation`] - UAE Legal Citation System
//!
//! Supports both Arabic and English citation formats:
//! - Federal Laws: `Federal Law No. [num]/[year]`
//! - Federal Decree-Laws: `Federal Decree-Law No. [num]/[year]`
//! - Cabinet Resolutions: `Cabinet Resolution No. [num]/[year]`
//!
//! ### [`common`] - Shared Utilities
//!
//! - AED (Dirham) currency formatting
//! - UAE public holidays (Islamic calendar-based)
//! - Emirates and free zone information
//!
//! ### [`labor_law`] - Federal Decree-Law No. 33/2021
//!
//! UAE's Labour Relations Law (effective 2022):
//! - 8 hours/day, 48 hours/week maximum
//! - End of Service Gratuity (EOSG)
//! - Wage Protection System (WPS)
//! - New contract types (flexible, remote)
//!
//! ### [`commercial_companies`] - Federal Decree-Law No. 32/2021
//!
//! Commercial Companies Law:
//! - 100% foreign ownership (in most sectors)
//! - LLC, PJSC, PrJSC company types
//! - Corporate governance requirements
//!
//! ### [`data_protection`] - Federal Decree-Law No. 45/2021
//!
//! UAE Personal Data Protection Law:
//! - GDPR-aligned principles
//! - 9 data subject rights
//! - Cross-border transfer restrictions
//! - Free zone variations (DIFC/ADGM)
//!
//! ## Quick Start
//!
//! ```rust
//! use legalis_ae::{
//!     citation::{UaeCitation, common_citations},
//!     labor_law::{WorkingHours, EndOfServiceGratuity, TerminationReason, calculate_eosg},
//!     commercial_companies::{CompanyType, validate_registration},
//!     common::{Aed, FreeZone},
//! };
//!
//! // Citation example
//! let citation = UaeCitation::federal_decree_law(33, 2021)
//!     .with_title_en("Labour Relations")
//!     .with_article(51);
//! println!("{}", citation); // Federal Decree-Law No. 33/2021 (Labour Relations), Article 51
//!
//! // Currency formatting
//! let salary = Aed::from_dirhams(15000);
//! println!("{}", salary.format_en()); // AED 15,000.00
//! println!("{}", salary.format_ar()); // 15,000.00 د.إ
//!
//! // EOSG calculation
//! let eosg = calculate_eosg(5, Aed::from_dirhams(10000), TerminationReason::ContractExpiry);
//! println!("Gratuity: {}", eosg.gratuity_amount);
//!
//! // Free zone check
//! assert!(FreeZone::Difc.uses_common_law());
//! ```
//!
//! ## Major Laws Covered
//!
//! | Law | Name (AR) | Name (EN) |
//! |-----|-----------|-----------|
//! | FDL 33/2021 | تنظيم علاقات العمل | Labour Relations |
//! | FDL 32/2021 | الشركات التجارية | Commercial Companies |
//! | FDL 45/2021 | حماية البيانات الشخصية | Personal Data Protection |
//! | FL 5/1985 | المعاملات المدنية | Civil Transactions |
//! | FDL 2/2015 | مكافحة التمييز | Anti-Discrimination |
//!
//! ## Workweek Changes (2022)
//!
//! As of January 2022, UAE transitioned to a Monday-Friday workweek:
//! - Saturday & Sunday are now weekends
//! - Previously was Sunday-Thursday with Friday weekend
//! - Affects government and most private sector
//!
//! ## Disclaimer
//!
//! This library is for educational and informational purposes. For legal matters,
//! consult qualified UAE legal professionals (محامي).

pub mod arbitration;
pub mod banking_finance;
pub mod citation;
pub mod civil_code;
pub mod commercial_companies;
pub mod common;
pub mod criminal_code;
pub mod cybercrime;
pub mod data_protection;
pub mod free_zones;
pub mod intellectual_property;
pub mod islamic_law;
pub mod labor_law;
pub mod real_estate;
pub mod tax_law;

// Re-export citation types
pub use citation::{Emirate, LegalInstrumentType, UaeCitation, common_citations};

// Re-export common types
pub use common::{
    Aed, FreeZone, SkillLevel, UaeHoliday, UaeHolidayType, get_public_holidays, is_public_holiday,
    is_working_day, working_days_between,
};

// Re-export labor law types
pub use labor_law::{
    ContractType, EndOfServiceGratuity, LaborError, LaborResult, LeaveType, TerminationReason,
    WorkingHours, calculate_eosg, get_labor_checklist, validate_contract, validate_working_hours,
};

// Re-export commercial companies types
pub use commercial_companies::{
    CompanyError, CompanyRegistration, CompanyResult, CompanyType, GovernanceRequirements,
    get_company_checklist, get_restricted_sectors, validate_registration,
};

// Re-export data protection types
pub use data_protection::{
    DataCategory, DataProtectionAssessment, DataProtectionError, DataProtectionResult,
    DataSubjectRight, FreeZoneFramework, LegalBasis, RiskLevel, SensitiveDataType,
    TransferMechanism, get_pdpl_checklist, validate_processing, validate_transfer,
};

// Re-export Islamic law types
pub use islamic_law::{
    HeritageRelationship, InheritanceShare, IslamicFinanceContract, IslamicLawError,
    IslamicLawResult, MadhabSchool, Waqf, WaqfType, calculate_simple_inheritance, check_riba,
    get_sharia_compliance_checklist,
};

// Re-export free zone types
pub use free_zones::{
    // ADGM
    AdgmCompanyType,
    AdgmCourtJurisdiction,
    AdgmDataProtection,
    AdgmEmployment,
    AdgmError,
    AdgmFinancialActivity,
    AdgmResult,
    // DIFC
    DifcCompanyType,
    DifcCourtJurisdiction,
    DifcEmployment,
    DifcError,
    DifcFinancialActivity,
    DifcResult,
    // JAFZA
    JafzaActivity,
    JafzaCompanyType,
    JafzaError,
    JafzaLicenseType,
    JafzaOfficeType,
    JafzaRegistration,
    JafzaResult,
    estimate_setup_cost,
    get_adgm_benefits,
    get_difc_benefits,
    get_jafza_benefits,
    validate_adgm_registration,
    validate_difc_registration,
};

// Re-export tax law types
pub use tax_law::{
    CorporateTax, CorporateTaxRate, TaxError, TaxResult, VatRate, VatRegistration,
    VatRegistrationStatus, VatSupplyCategory, get_corporate_tax_checklist, get_vat_checklist,
    validate_trn,
};

// Re-export civil code types
pub use civil_code::{
    CivilCodeError, CivilCodeResult, Compensation, ContractFormation,
    ContractType as CivilContractType, InvalidityGround, LimitationPeriod, TortLiability,
    get_civil_code_checklist, validate_contract_formation,
};

// Re-export criminal code types
pub use criminal_code::{
    Circumstance, CircumstanceType, CrimeClassification, CriminalLawError, CriminalLawResult,
    CriminalOffense, Penalty, ProceduralRights, get_common_offenses,
    get_criminal_procedure_checklist,
};

// Re-export real estate types
pub use real_estate::{
    EscrowAccount, FreeholdArea, OwnershipType, PropertyType, RealEstateError, RealEstateResult,
    RentalContract, StrataProperty, TransferCosts, get_transaction_checklist,
};

// Re-export banking and finance types
pub use banking_finance::{
    AmlCompliance, BankingError, BankingResult, ConsumerProtection, FinancialInstitution,
    RegulatoryCompliance, get_banking_checklist,
};

// Re-export intellectual property types
pub use intellectual_property::{
    CopyrightWork, EnforcementAction, IpError, IpResult, IpType, Patent, PatentType, Trademark,
    get_ip_checklist,
};

// Re-export cybercrime types
pub use cybercrime::{
    CybercrimeError, CybercrimeResult, CybercrimeType, CybersecurityCompliance,
    DataBreachNotification, ElectronicEvidence, get_common_cybercrimes,
    get_cybersecurity_checklist,
};

// Re-export arbitration types
pub use arbitration::{
    ArbitrationAgreement, ArbitrationCosts, ArbitrationError, ArbitrationInstitution,
    ArbitrationResult, ArbitrationType, ChallengeGround, get_arbitration_checklist,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_citation_display() {
        let citation = UaeCitation::federal_decree_law(33, 2021)
            .with_title_en("Labour Relations")
            .with_article(51);

        let formatted = citation.to_string();
        assert!(formatted.contains("33/2021"));
        assert!(formatted.contains("Article 51"));
    }

    #[test]
    fn test_aed_formatting() {
        let amount = Aed::from_dirhams(15000);
        assert_eq!(amount.format_en(), "AED 15,000.00");
    }

    #[test]
    fn test_eosg_calculation() {
        let eosg = calculate_eosg(
            5,
            Aed::from_dirhams(10000),
            TerminationReason::ContractExpiry,
        );
        // 5 years * 21 days * (10000/30) daily wage
        assert!(eosg.gratuity_amount.dirhams() > 0);
    }

    #[test]
    fn test_working_hours() {
        let hours = WorkingHours::standard();
        assert!(hours.is_within_limits());
        assert_eq!(hours.total_weekly_hours(), 40);
    }

    #[test]
    fn test_company_types() {
        let llc = CompanyType::Llc;
        assert!(llc.has_limited_liability());
        assert_eq!(llc.minimum_shareholders(), 1);
    }

    #[test]
    fn test_free_zones() {
        assert!(FreeZone::Difc.uses_common_law());
        assert!(FreeZone::Adgm.uses_common_law());
        assert!(!FreeZone::Jafza.uses_common_law());
    }

    #[test]
    fn test_data_protection() {
        let result = validate_processing(
            &DataCategory::General,
            &LegalBasis::ContractPerformance,
            false,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_public_holidays() {
        let holidays = get_public_holidays(2024);
        assert!(!holidays.is_empty());

        // Check National Day
        let national_day = chrono::NaiveDate::from_ymd_opt(2024, 12, 2).unwrap();
        assert!(is_public_holiday(national_day));
    }

    #[test]
    fn test_common_citations() {
        let labor = common_citations::labor_law_2021();
        assert_eq!(labor.number, 33);
        assert_eq!(labor.year, 2021);

        let companies = common_citations::commercial_companies_law();
        assert_eq!(companies.number, 32);
    }

    #[test]
    fn test_leave_entitlements() {
        assert_eq!(LeaveType::Annual.statutory_days(), 30);
        assert_eq!(LeaveType::Maternity.statutory_days(), 60);
    }
}
