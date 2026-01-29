//! # Legalis-ZA: South Africa Jurisdiction Support
//!
//! Comprehensive South African legal system implementation for Legalis-RS.
//!
//! ## Legal System Overview
//!
//! South Africa has a **Mixed Legal System** comprising:
//! - **Common Law** (from English law)
//! - **Roman-Dutch Civil Law** (from Dutch settlement)
//! - **Customary Law** (indigenous African systems)
//! - **Constitution** (supreme law since 1996)
//!
//! The Constitution of 1996 is the cornerstone of the legal system, featuring
//! one of the most progressive Bills of Rights globally.
//!
//! ## Hierarchy of Laws
//!
//! 1. **Constitution** - Supreme law, Bill of Rights
//! 2. **Acts of Parliament** - National legislation
//! 3. **Provincial Legislation**
//! 4. **Regulations and Subordinate Legislation**
//! 5. **Common Law** (Roman-Dutch and English)
//! 6. **Customary Law** (where applicable)
//!
//! ## Courts Structure
//!
//! - **Constitutional Court** - Constitutional matters, highest on constitutional issues
//! - **Supreme Court of Appeal** - Highest on non-constitutional matters
//! - **High Courts** (9 divisions)
//! - **Magistrates' Courts**
//! - **Specialized Courts** (Labour Court, Labour Appeal Court, Land Claims Court)
//!
//! ## Modules
//!
//! ### [`citation`] - South African Legal Citation System
//!
//! Supports Commonwealth-style citations:
//! - Acts: `Companies Act 71 of 2008, s. 22(1)`
//! - Constitution: `Constitution of the Republic of South Africa, 1996`
//!
//! ### [`common`] - Shared Utilities
//!
//! - South African Rand (ZAR) currency formatting
//! - Public holidays
//! - CCMA regions
//! - UIF contributions
//!
//! ### [`constitution`] - Constitution of 1996
//!
//! - Bill of Rights (s7-39)
//! - Limitation test (s36)
//! - Chapter 9 institutions
//! - Constitutional Court
//!
//! ### [`tax_law`] - Tax Law
//!
//! - VAT Act (15%)
//! - Income Tax Act
//! - SARS administration
//! - Transfer duty
//!
//! ### [`criminal_law`] - Criminal Law and Procedure
//!
//! - Criminal Procedure Act
//! - Minimum sentences
//! - Bail
//! - Sentencing
//!
//! ### [`property_law`] - Property Law
//!
//! - Deeds registration
//! - Land reform
//! - ESTA (farm dwellers)
//! - Sectional titles
//!
//! ### [`customary_law`] - Customary Law
//!
//! - Recognition of Customary Marriages Act
//! - Traditional courts
//! - Customary succession
//!
//! ### [`intellectual_property`] - IP Law
//!
//! - Patents (20 years)
//! - Trademarks (10 years, renewable)
//! - Copyright (life + 50)
//! - Designs
//!
//! ### [`competition_law`] - Competition Act
//!
//! - Prohibited practices
//! - Abuse of dominance
//! - Merger control
//!
//! ### [`financial_services`] - FAIS & Financial Regulation
//!
//! - FSP licensing
//! - Twin Peaks model
//! - TCF outcomes
//!
//! ### [`insolvency`] - Insolvency Law
//!
//! - Sequestration (individuals)
//! - Liquidation (companies)
//! - Business rescue
//!
//! ### [`environmental_law`] - Environmental Law
//!
//! - NEMA principles
//! - Environmental authorization
//! - Duty of care (s28)
//!
//! ### [`labor`] - Labour Law (LRA, BCEA)
//!
//! Labour Relations Act 66 of 1995 and Basic Conditions of Employment Act 75 of 1997:
//! - Maximum 45 ordinary hours/week
//! - 21 days annual leave
//! - Unfair dismissal protection
//! - CCMA dispute resolution
//! - Severance pay (retrenchment)
//!
//! ### [`companies`] - Companies Act 71 of 2008
//!
//! Company formation and corporate governance:
//! - Company types: (Pty) Ltd, Ltd, Inc, NPC
//! - King IV governance principles
//! - Business rescue (Chapter 6)
//! - B-BBEE integration
//!
//! ### [`data_protection`] - POPIA
//!
//! Protection of Personal Information Act 4 of 2013:
//! - 8 conditions for lawful processing
//! - Information Regulator oversight
//! - Direct marketing rules
//! - Cross-border transfers
//!
//! ## Quick Start
//!
//! ```rust
//! use legalis_za::{
//!     citation::{SouthAfricanCitation, common_citations},
//!     labor::{WorkingHours, calculate_severance, LeaveType},
//!     companies::{CompanyType, BbbeeScorecard, validate_registration},
//!     common::{Zar, MinimumWageCategory},
//! };
//!
//! // Citation example
//! let citation = SouthAfricanCitation::act("Companies Act", 71, 2008)
//!     .with_section("22")
//!     .with_subsection("1");
//! println!("{}", citation); // Companies Act 71 of 2008, s. 22(1)
//!
//! // Currency formatting
//! let salary = Zar::from_rands(25000);
//! println!("{}", salary.format()); // R 25 000.00
//!
//! // Minimum wage
//! let min = MinimumWageCategory::Standard;
//! println!("Hourly rate: {}", min.hourly_rate_2024());
//!
//! // B-BBEE scorecard
//! let scorecard = BbbeeScorecard {
//!     ownership_points: 20.0,
//!     management_control_points: 15.0,
//!     skills_development_points: 18.0,
//!     enterprise_supplier_development_points: 35.0,
//!     socio_economic_development_points: 4.0,
//!     priority_elements_achieved: true,
//! };
//! println!("B-BBEE Level: {:?}", scorecard.level());
//! ```
//!
//! ## Major Laws Covered
//!
//! | Act | Name | Year | Module |
//! |-----|------|------|--------|
//! | - | Constitution | 1996 | constitution |
//! | 71 | Companies Act | 2008 | companies |
//! | 66 | Labour Relations Act | 1995 | labor |
//! | 75 | Basic Conditions of Employment Act | 1997 | labor |
//! | 4 | Protection of Personal Information Act | 2013 | data_protection |
//! | 53 | B-BBEE Act | 2003 | companies |
//! | 55 | Employment Equity Act | 1998 | labor |
//! | 58 | Income Tax Act | 1962 | tax_law |
//! | 89 | Value-Added Tax Act | 1991 | tax_law |
//! | 51 | Criminal Procedure Act | 1977 | criminal_law |
//! | 105 | Criminal Law Amendment Act | 1997 | criminal_law |
//! | 47 | Deeds Registries Act | 1937 | property_law |
//! | 95 | Sectional Titles Act | 1986 | property_law |
//! | 120 | Recognition of Customary Marriages Act | 1998 | customary_law |
//! | 57 | Patents Act | 1978 | intellectual_property |
//! | 194 | Trademarks Act | 1993 | intellectual_property |
//! | 98 | Copyright Act | 1978 | intellectual_property |
//! | 89 | Competition Act | 1998 | competition_law |
//! | 37 | FAIS Act | 2002 | financial_services |
//! | 9 | Financial Sector Regulation Act | 2017 | financial_services |
//! | 24 | Insolvency Act | 1936 | insolvency |
//! | 107 | National Environmental Management Act | 1998 | environmental_law |
//!
//! ## B-BBEE Framework
//!
//! Broad-Based Black Economic Empowerment is integral to South African business:
//! - Ownership (25 points max)
//! - Management Control (19 points)
//! - Skills Development (20 points)
//! - Enterprise & Supplier Development (40 points)
//! - Socio-Economic Development (5 points)
//! - Levels 1-8 + Non-Compliant
//!
//! ## Disclaimer
//!
//! This library is for educational and informational purposes. For legal matters,
//! consult qualified South African legal professionals (attorneys/advocates).

pub mod citation;
pub mod common;
pub mod companies;
pub mod competition_law;
pub mod constitution;
pub mod criminal_law;
pub mod customary_law;
pub mod data_protection;
pub mod environmental_law;
pub mod financial_services;
pub mod insolvency;
pub mod intellectual_property;
pub mod labor;
pub mod property_law;
pub mod tax_law;

// Re-export citation types
pub use citation::{LegalInstrumentType, Province, SouthAfricanCitation, common_citations};

// Re-export common types
pub use common::{
    CcmaRegion, MinimumWageCategory, SouthAfricanHoliday, SouthAfricanHolidayType, UifContribution,
    Zar, get_public_holidays, is_public_holiday, is_working_day, working_days_between,
};

// Re-export labor types
pub use labor::{
    AutomaticallyUnfairGround, CcmaTimeframes, ContractType, LaborError, LaborResult, LeaveType,
    SeverancePay, TerminationType, UnfairDismissalType, WorkingHours, calculate_severance,
    get_labor_checklist, validate_working_hours,
};

// Re-export companies types
pub use companies::{
    BbbeeLevel, BbbeeScorecard, BusinessRescueStatus, CompanyError, CompanyRegistration,
    CompanyResult, CompanyType, KingIvPrinciple, get_company_checklist, validate_registration,
};

// Re-export data protection types
pub use data_protection::{
    DataSubjectRight, InformationOfficer, LegalBasis, PersonalInformationCategory, PopiaCompliance,
    PopiaError, PopiaResult, ProcessingCondition, SpecialPersonalInformation, TransferBasis,
    get_popia_checklist, validate_processing,
};

// Re-export constitution types
pub use constitution::{
    BillOfRightsGuarantee, Chapter9Institution, ConstitutionalCourtJurisdiction,
    ConstitutionalError, ConstitutionalResult, DiscriminationGround, FoundingValue, LimitationTest,
    get_constitutional_checklist, validate_limitation,
};

// Re-export tax law types
pub use tax_law::{
    CgtInclusionRate, PersonalIncomeTaxBracket, SarsFilingRequirement, TaxError, TaxRebate,
    TaxResult, TaxType, VatRegistration, VatSupplyType, get_tax_checklist,
    validate_vat_registration,
};

// Re-export criminal law types
pub use criminal_law::{
    AppealCourt, BailConsideration, CrimeElement, CriminalCapacity, CriminalError, CriminalResult,
    Fault, GroundExcludingFault, GroundOfJustification, IntentionType, ScheduleOffence,
    SentenceType, get_criminal_procedure_checklist, validate_bail, validate_sentence,
};

// Re-export property law types
pub use property_law::{
    DeedsOffice, DeedsRegistration, EstaProtection, LandReformType, OwnershipType, PropertyError,
    PropertyResult, RealRight, RestitutionClaim, SectionalTitle, ServitudeType, TransferDuty,
    get_property_checklist, validate_deeds_registration,
};

// Re-export customary law types
pub use customary_law::{
    CustomaryError, CustomaryMarriage, CustomaryResult, CustomarySuccession, Lobolo,
    MarriageRegime, TraditionalCommunity, TraditionalCourt, TraditionalLeaderRole,
    get_customary_law_checklist, validate_customary_marriage, validate_succession_rule,
};

// Re-export intellectual property types
pub use intellectual_property::{
    Copyright, CopyrightWorkType, Design, DesignType, FairDealingPurpose, IntellectualPropertyType,
    IpEnforcement, IpError, IpResult, NonPatentableSubject, Patent, Trademark, get_ip_checklist,
    validate_patent, validate_trademark,
};

// Re-export competition law types
pub use competition_law::{
    AbuseOfDominance, CompetitionError, CompetitionResult, HorizontalPractice, LeniencyApplication,
    MergerAssessment, MergerCategory, MergerThresholds, PublicInterestFactor, VerticalPractice,
    get_competition_checklist, validate_horizontal_agreement, validate_merger_notification,
};

// Re-export financial services types
pub use financial_services::{
    CodeOfConduct, FaisLicense, FinancialProduct, FinancialService, FinancialServicesError,
    FinancialServicesResult, FitAndProper, FspCategory, RegulatoryObjective, TcfOutcome,
    TwinPeaksRegulator, get_financial_services_checklist, validate_fais_license,
    validate_fit_and_proper,
};

// Re-export insolvency types
pub use insolvency::{
    ActOfInsolvency, BusinessRescue, CreditorClass, InsolvencyError, InsolvencyResult, Liquidation,
    LiquidationType, PreferentClaim, Rehabilitation, Sequestration, SequestrationType,
    VoidableDisposition, get_insolvency_checklist, validate_business_rescue,
    validate_sequestration,
};

// Re-export environmental law types
pub use environmental_law::{
    AirQualityLicense, AuthorizationType, BiodiversityThreat, ComplianceNotice, DutyOfCare,
    EnvironmentalAuthorization, EnvironmentalError, EnvironmentalResult, EnvironmentalRight,
    NemaPrinciple, WasteActivity, WasteClassification, WaterUse, get_environmental_checklist,
    validate_duty_of_care, validate_environmental_authorization,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_citation_display() {
        let citation = SouthAfricanCitation::act("Companies Act", 71, 2008)
            .with_section("22")
            .with_subsection("1");

        let formatted = citation.to_string();
        assert!(formatted.contains("Companies Act 71 of 2008"));
        assert!(formatted.contains("s. 22(1)"));
    }

    #[test]
    fn test_zar_formatting() {
        let amount = Zar::from_rands(25000);
        assert_eq!(amount.format(), "R 25 000.00");
    }

    #[test]
    fn test_working_hours() {
        let hours = WorkingHours::standard();
        assert!(hours.is_within_limits());
        assert_eq!(hours.total_weekly_hours(), 45);
    }

    #[test]
    fn test_severance_calculation() {
        let weekly = Zar::from_rands(5000);
        let severance = calculate_severance(5, weekly);
        assert_eq!(severance.severance_amount.rands(), 25000);
    }

    #[test]
    fn test_company_types() {
        let private = CompanyType::PrivateCompany;
        assert_eq!(private.suffix(), "(Pty) Ltd");
        assert!(private.has_limited_liability());
    }

    #[test]
    fn test_bbbee_scorecard() {
        let scorecard = BbbeeScorecard {
            ownership_points: 20.0,
            management_control_points: 15.0,
            skills_development_points: 18.0,
            enterprise_supplier_development_points: 35.0,
            socio_economic_development_points: 4.0,
            priority_elements_achieved: true,
        };

        assert_eq!(scorecard.level(), BbbeeLevel::Level3);
    }

    #[test]
    fn test_leave_entitlements() {
        assert_eq!(LeaveType::Annual.statutory_days(), 21);
        assert_eq!(LeaveType::Maternity.statutory_days(), 120);
    }

    #[test]
    fn test_popia_processing() {
        let result = validate_processing(
            &PersonalInformationCategory::General,
            &LegalBasis::Contract,
            false,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_public_holidays() {
        let holidays = get_public_holidays(2024);
        assert!(!holidays.is_empty());

        // Check Freedom Day
        let freedom_day = chrono::NaiveDate::from_ymd_opt(2024, 4, 27).unwrap();
        assert!(is_public_holiday(freedom_day));
    }

    #[test]
    fn test_common_citations() {
        let companies = common_citations::companies_act();
        assert_eq!(companies.number, Some(71));
        assert_eq!(companies.year, 2008);

        let popia = common_citations::popia();
        assert_eq!(popia.number, Some(4));
        assert_eq!(popia.year, 2013);
    }

    #[test]
    fn test_minimum_wage() {
        let standard = MinimumWageCategory::Standard;
        assert!(standard.hourly_rate_2024().cents() > 0);
    }
}
