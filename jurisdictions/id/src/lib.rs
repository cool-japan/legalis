//! # Legalis-ID: Indonesia Jurisdiction Support
//!
//! Comprehensive Indonesian legal system implementation for Legalis-RS.
//!
//! ## Legal System Overview
//!
//! Indonesia uses a **Civil Law** system with significant **Islamic Law** (Hukum Islam)
//! influence, particularly in family and inheritance matters. The legal system is based on:
//!
//! - **Pancasila** (Five Principles) as state philosophy
//! - **UUD 1945** (Constitution of 1945)
//! - **Dutch colonial legal inheritance** (KUHPerdata, KUHPidana)
//! - **Islamic law** (particularly in marriage, inheritance via Religious Courts)
//!
//! ## Hierarchy of Laws
//!
//! 1. **UUD 1945** - Constitution
//! 2. **TAP MPR** - People's Consultative Assembly Decrees
//! 3. **UU/Perppu** - Laws/Government Regulations in Lieu of Law
//! 4. **PP** - Government Regulations
//! 5. **Perpres** - Presidential Regulations
//! 6. **Perda** - Regional Regulations
//!
//! ## Modules
//!
//! ### [`citation`] - Indonesian Legal Citation System
//!
//! Supports all Indonesian legal instrument types:
//! - `UU No. [num] Tahun [year]` - Laws
//! - `PP No. [num] Tahun [year]` - Government Regulations
//! - `Perpres No. [num] Tahun [year]` - Presidential Regulations
//! - `KUHPerdata Pasal [num]` - Civil Code articles
//!
//! ### [`common`] - Shared Utilities
//!
//! - Indonesian public holidays (Hari Libur Nasional)
//! - Rupiah currency formatting
//! - Province information and minimum wages
//!
//! ### [`data_protection`] - UU PDP No. 27/2022
//!
//! Indonesia's comprehensive data protection law:
//! - 7 data subject rights
//! - Legal bases for processing
//! - 3x24 hours breach notification
//! - Cross-border transfer rules
//!
//! ### [`labor_law`] - UU Ketenagakerjaan No. 13/2003
//!
//! As amended by Omnibus Law (UU Cipta Kerja):
//! - PKWT/PKWTT contract types
//! - Severance calculation (9-32 months)
//! - BPJS contributions
//! - Minimum wage by province
//!
//! ### [`investment`] - UU Penanaman Modal No. 25/2007
//!
//! As amended by Omnibus Law:
//! - Negative Investment List (DNI)
//! - OSS risk-based licensing
//! - Foreign ownership limits
//! - Priority sector incentives
//!
//! ### [`civil_code`] - KUHPerdata
//!
//! Indonesian Civil Code (based on Dutch BW 1838):
//! - Contract validity (Pasal 1320)
//! - Legal capacity
//! - Obligations and property
//!
//! ## Key Legal Concepts
//!
//! ### Pancasila (Five Principles)
//!
//! The philosophical foundation of Indonesian law:
//! 1. Belief in One Supreme God (Ketuhanan Yang Maha Esa)
//! 2. Just and Civilized Humanity (Kemanusiaan yang Adil dan Beradab)
//! 3. Unity of Indonesia (Persatuan Indonesia)
//! 4. Democracy through Deliberation (Kerakyatan yang Dipimpin oleh Hikmat Kebijaksanaan)
//! 5. Social Justice (Keadilan Sosial bagi Seluruh Rakyat Indonesia)
//!
//! ### Dual Court System
//!
//! - **Peradilan Umum** (General Courts) - Civil and criminal matters
//! - **Peradilan Agama** (Religious Courts) - Islamic family/inheritance for Muslims
//! - **Peradilan Tata Usaha Negara** (Administrative Courts)
//! - **Peradilan Militer** (Military Courts)
//!
//! ### Aceh Special Autonomy
//!
//! Aceh province has special authority to implement Sharia law (Qanun Aceh)
//! for Muslim residents, including in criminal matters.
//!
//! ## Quick Start
//!
//! ```rust
//! use legalis_id::{
//!     citation::{IndonesianCitation, common_citations},
//!     data_protection::{validate_pdp_compliance, PersonalDataProcessing, DataCategory, LegalBasis},
//!     labor_law::{validate_contract, EmploymentContract, ContractType, WorkingHours},
//!     investment::{validate_foreign_investment, ForeignInvestment},
//!     civil_code::{validate_contract_validity, ContractValidity},
//!     common::{Rupiah, Province},
//! };
//!
//! // Citation example
//! let citation = IndonesianCitation::undang_undang(13, 2003)
//!     .with_article(1)
//!     .with_paragraph(1);
//! println!("{}", citation); // UU No. 13 Tahun 2003, Pasal 1 ayat (1)
//!
//! // Currency formatting
//! let salary = Rupiah::from_juta(10);
//! println!("{}", salary); // Rp 10.000.000
//!
//! // Province minimum wage
//! let min_wage = Province::DkiJakarta.minimum_wage_2024();
//! println!("Jakarta UMP 2024: {}", min_wage);
//! ```
//!
//! ## Major Laws Covered
//!
//! | Law | Name (ID) | Name (EN) |
//! |-----|-----------|-----------|
//! | UU 13/2003 | Ketenagakerjaan | Manpower/Labor |
//! | UU 27/2022 | Perlindungan Data Pribadi | Personal Data Protection |
//! | UU 25/2007 | Penanaman Modal | Investment |
//! | UU 6/2023 | Cipta Kerja (Omnibus) | Job Creation |
//! | UU 40/2007 | Perseroan Terbatas | Limited Liability Company |
//! | UU 21/2008 | Perbankan Syariah | Islamic Banking |
//! | KUHPerdata | Kitab UU Hukum Perdata | Civil Code |
//!
//! ## Bilingual Support
//!
//! All types include both Indonesian (`_id`) and English (`_en`) descriptions
//! for international accessibility while maintaining authoritative Indonesian text.
//!
//! ## Disclaimer
//!
//! This library is for educational and informational purposes. For legal matters,
//! consult qualified Indonesian legal professionals (advokat/pengacara).

pub mod citation;
pub mod civil_code;
pub mod common;
pub mod data_protection;
pub mod investment;
pub mod labor_law;

// Re-export commonly used items
pub use citation::{CodeType, IndonesianCitation, LegalInstrumentType, Ministry, common_citations};

pub use common::{
    IndonesianHoliday, IndonesianHolidayType, Province, Rupiah, get_national_holidays,
    is_national_holiday, is_working_day, working_days_between,
};

pub use data_protection::{
    ConsentRecord, DataCategory, DataSubjectRight, LegalBasis, PdpCompliance, PdpError, PdpResult,
    PersonalData, PersonalDataProcessing, ProcessingPurpose, RiskLevel, SecurityIncident,
    SpecificDataType, get_pdp_checklist, validate_consent, validate_cross_border_transfer,
    validate_data_retention, validate_incident_notification, validate_legal_basis,
    validate_pdp_compliance,
};

pub use labor_law::{
    BpjsContribution, ContractType, EmploymentContract, LaborCompliance, LaborError, LaborResult,
    LeaveType, Severance, TerminationType, WorkingHours, calculate_bpjs_contribution,
    calculate_overtime_pay, calculate_severance, get_labor_checklist, validate_contract,
    validate_labor_compliance, validate_minimum_wage, validate_working_hours,
};

pub use investment::{
    BusinessLicense, BusinessRisk, ForeignInvestment, InvestmentCompliance, InvestmentError,
    InvestmentResult, InvestmentSector, LicenseType, OwnershipLimit, PriorityInvestment,
    PrioritySector, check_ownership_limit, get_investment_checklist, validate_business_license,
    validate_foreign_investment, validate_investment_compliance, validate_sector_eligibility,
};

pub use civil_code::{
    CivilCodeError, CivilCodeResult, Contract, ContractCompliance, ContractFormation,
    ContractTermination, ContractValidity, ContractValidityStatus, LegalCapacity, ObligationType,
    PropertyRight, PropertyType, get_contract_checklist, validate_contract_compliance,
    validate_contract_formation, validate_contract_validity, validate_legal_capacity,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_citation_display() {
        let citation = IndonesianCitation::undang_undang(27, 2022)
            .with_article(5)
            .with_paragraph(1);

        assert_eq!(
            citation.to_string(),
            "UU No. 27 Tahun 2022, Pasal 5 ayat (1)"
        );
    }

    #[test]
    fn test_rupiah_formatting() {
        let amount = Rupiah::from_juta(10);
        assert_eq!(amount.format_id(), "Rp 10.000.000");
        assert_eq!(amount.format_juta(), "Rp 10 juta");
    }

    #[test]
    fn test_province_minimum_wage() {
        let jakarta_wage = Province::DkiJakarta.minimum_wage_2024();
        assert!(jakarta_wage.amount() > 5_000_000);
    }

    #[test]
    fn test_working_hours() {
        let hours = WorkingHours::standard_5_day();
        assert!(hours.is_within_limits());
        assert_eq!(hours.total_weekly_hours(), 40);
    }

    #[test]
    fn test_contract_validity() {
        let validity = ContractValidity {
            has_agreement: true,
            agreement_free_from_defects: true,
            parties_have_capacity: true,
            has_specific_object: true,
            object_is_determinable: true,
            has_lawful_cause: true,
            not_contrary_to_law: true,
        };

        assert!(validity.is_valid());
        assert!(validate_contract_validity(&validity).is_ok());
    }

    #[test]
    fn test_legal_capacity() {
        let full = LegalCapacity::Full;
        assert!(full.can_contract());
        assert!(validate_legal_capacity(&full).is_ok());
    }

    #[test]
    fn test_holidays_2024() {
        let holidays = get_national_holidays(2024);
        assert!(!holidays.is_empty());

        // Check Independence Day
        let aug_17 = chrono::NaiveDate::from_ymd_opt(2024, 8, 17).unwrap();
        assert!(is_national_holiday(aug_17));
    }

    #[test]
    fn test_common_citations() {
        let labor = common_citations::labor_law();
        assert_eq!(labor.number, 13);
        assert_eq!(labor.year, 2003);

        let pdp = common_citations::pdp_law();
        assert_eq!(pdp.number, 27);
        assert_eq!(pdp.year, 2022);
    }
}
