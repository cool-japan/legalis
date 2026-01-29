//! # Legalis-MX: Mexican Law Library
//!
//! Comprehensive Mexican legal framework implementation in Pure Rust.
//!
//! ## Overview
//!
//! Mexico operates under a **Civil Law** system, based on the Romano-Germanic tradition
//! with influences from Spanish colonial law and French civil code.
//!
//! This library provides validation, types, and utilities for key Mexican legislation:
//!
//! | Domain | Law | Year | Key Provisions |
//! |--------|-----|------|----------------|
//! | Civil | Código Civil Federal | 1928 | Persons, property, obligations, contracts |
//! | Criminal | Código Penal Federal | 1931 | Criminal offenses and penalties |
//! | Labor | Ley Federal del Trabajo | 1970 | 8h/day, aguinaldo (15 days), vacations |
//! | Tax | Código Fiscal de la Federación | 1981 | Tax obligations and procedures |
//! | Data Protection | LFPDPPP | 2010 | ARCO rights, privacy notices |
//! | Corporate | LGSM | 1934 | SA, SRL, and other company types |
//! | IP | Ley de Propiedad Industrial | 1991 | Patents, trademarks, designs |
//! | Competition | LFCE | 2014 | Antitrust, market concentration |
//!
//! ## Mexican Legal Citation Format
//!
//! Mexican citations follow a specific format:
//!
//! ```text
//! [Law Name], Artículo [article], fracción [fraction]
//! ```
//!
//! ### Examples
//!
//! ```rust
//! use legalis_mx::citation::*;
//!
//! // Federal Civil Code
//! let ccf = format_civil_code_citation(1792, None, None);
//! assert_eq!(ccf, "Código Civil Federal, Artículo 1792");
//!
//! // Labor Law with fraction
//! let lft = format_labor_law_citation(61, None, Some(RomanNumeral::I));
//! assert_eq!(lft, "Ley Federal del Trabajo, Artículo 61, fracción I");
//!
//! // Constitution
//! let const_cite = format_constitution_citation(123, Some('A'), Some(RomanNumeral::VI));
//! assert_eq!(
//!     const_cite,
//!     "Constitución Política de los Estados Unidos Mexicanos, Artículo 123, Apartado A, fracción VI"
//! );
//! ```
//!
//! ## Currency: Mexican Peso (MXN)
//!
//! ```rust
//! use legalis_mx::common::MexicanCurrency;
//!
//! let salary = MexicanCurrency::from_pesos(300); // Daily minimum wage
//! assert_eq!(salary.pesos(), 300);
//!
//! let amount = MexicanCurrency::from_centavos(150050);
//! assert_eq!(amount.pesos(), 1500);
//! assert_eq!(amount.cents(), 50);
//! ```
//!
//! ## Document Validation (RFC/CURP/NSS)
//!
//! ```rust
//! use legalis_mx::common::{validate_rfc, validate_curp, validate_nss};
//!
//! // RFC - Tax ID (12-13 characters)
//! assert!(validate_rfc("XAXX010101000").is_ok());
//!
//! // CURP - Population Registry ID (18 characters)
//! assert!(validate_curp("XAXX010101HDFRRL00").is_ok());
//!
//! // NSS - Social Security Number (11 digits)
//! assert!(validate_nss("12345678901").is_ok());
//! ```
//!
//! ## Mexican States
//!
//! ```rust
//! use legalis_mx::common::MexicanState;
//!
//! let cdmx = MexicanState::CMX;
//! assert_eq!(cdmx.nombre_es(), "Ciudad de México");
//! assert_eq!(cdmx.region_es(), "Centro");
//!
//! let nle = MexicanState::NLE;
//! assert_eq!(nle.nombre_es(), "Nuevo León");
//! assert_eq!(nle.region_es(), "Noreste");
//! ```
//!
//! ## Key Legal Principles
//!
//! ### Civil Code
//!
//! | Provision | Details |
//! |-----------|---------|
//! | Legal Capacity | Full capacity at 18 years (Article 646) |
//! | Contracts | Consent, lawful object, consideration (Article 1794) |
//! | Property | Ownership, possession, usufruct |
//!
//! ### Labor Law
//!
//! | Provision | Details |
//! |-----------|---------|
//! | Working Hours | 8h/day maximum (Article 61) |
//! | Aguinaldo | 15 days minimum, paid before Dec 20 (Article 87) |
//! | Vacation | 12 days (1st year) + 25% premium (Articles 76, 80) |
//! | Profit Sharing | PTU - 10% of profits (Article 117) |
//!
//! ### Tax Law
//!
//! | Tax | Rate | Details |
//! |-----|------|---------|
//! | ISR | 30% (corporate) | Income tax |
//! | IVA | 16% (standard) | Value added tax |
//! | IEPS | Variable | Special production tax |
//!
//! ### Data Protection (LFPDPPP)
//!
//! | Aspect | Details |
//! |--------|---------|
//! | ARCO Rights | Access, Rectification, Cancellation, Opposition |
//! | Privacy Notice | Required for all data collection |
//! | Consent | Explicit consent for sensitive data |
//!
//! ## Bilingual Support
//!
//! All types support both Spanish (authoritative) and English translations:
//!
//! ```rust
//! use legalis_mx::common::MexicanState;
//!
//! let state = MexicanState::JAL;
//! assert_eq!(state.nombre_es(), "Jalisco"); // Spanish (authoritative)
//! ```
//!
//! ## Module Structure
//!
//! - [`citation`] - Mexican legal citation formatting
//! - [`common`] - Currency, dates, documents, states, holidays
//! - [`civil_code`] - Federal Civil Code (persons, property, obligations, contracts)
//! - [`criminal_code`] - Federal Criminal Code
//! - [`labor_law`] - Federal Labor Law (working hours, aguinaldo, vacation)
//! - [`tax_law`] - Tax Code (ISR, IVA, IEPS)
//! - [`data_protection`] - LFPDPPP data protection law
//! - [`company_law`] - LGSM company law (SA, SRL)
//! - [`intellectual_property`] - Industrial Property Law
//! - [`competition_law`] - LFCE economic competition law
//! - [`reasoning`] - Legal reasoning and interpretation
//!
//! ## References
//!
//! - [Cámara de Diputados - Federal Legislation](http://www.diputados.gob.mx/LeyesBiblio/)
//! - [Suprema Corte de Justicia de la Nación](https://www.scjn.gob.mx/)
//! - [INAI - Data Protection Authority](https://home.inai.org.mx/)
//! - [IMPI - Industrial Property Institute](https://www.gob.mx/impi)

pub mod citation;
pub mod civil_code;
pub mod common;
pub mod company_law;
pub mod competition_law;
pub mod criminal_code;
pub mod data_protection;
pub mod intellectual_property;
pub mod labor_law;
pub mod reasoning;
pub mod tax_law;

// Re-export citation types
pub use citation::{
    MexicanCitation, RomanNumeral, format_civil_code_citation, format_constitution_citation,
    format_criminal_code_citation, format_isr_citation, format_iva_citation,
    format_labor_law_citation, format_lfce_citation, format_lfpdppp_citation, format_lgsm_citation,
    format_tax_code_citation, to_roman_numeral,
};

// Re-export common types
pub use common::{
    DocumentError, DocumentType, FederalHoliday, MexicanCurrency, MexicanDate, MexicanDocument,
    MexicanState, Municipality, get_federal_holidays, is_federal_holiday, minimum_wage, uma,
    validate_curp, validate_nss, validate_rfc,
};

// Re-export civil code types
pub use civil_code::{
    Breach, BreachType, ConsentDefect, Contract, ContractError, ContractType, DoObligation,
    EntityType, GiveObligation, ImmovableProperty, JuridicalPerson, LegalCapacity, MovableProperty,
    NaturalPerson, NotDoObligation, ObligationError, ObligationSource, ObligationType,
    OwnershipType, Party, PartyRole, PerformanceStatus, PersonError, PersonType, PropertyError,
    PropertyRight, PropertyType, RightType, Term, ValidityRequirements,
};

// Re-export criminal code types
pub use criminal_code::{
    CriminalError, CriminalOffense, CriminalResponsibility, OffenseClassification, PenaltyRange,
};

// Re-export labor law types
pub use labor_law::{
    EmploymentContract, EmploymentType, LaborRight, LaborValidationError, MINIMUM_DAYS,
    VACATION_PREMIUM_PERCENT, WorkDayType, WorkSchedule, WorkingHoursError, calculate_aguinaldo,
    calculate_overtime, calculate_proportional, calculate_total_vacation_compensation,
    calculate_vacation_premium, get_vacation_days, limits, validate_employment_contract,
    validate_schedule,
};

// Re-export tax law types
pub use tax_law::{
    BORDER_RATE, CORPORATE_TAX_RATE, IEPSCategory, IVARate, STANDARD_RATE, TAX_BRACKETS,
    TaxObligation, TaxPeriod, TaxType, TaxValidationError, Taxpayer, TaxpayerType, ZERO_RATE,
    calculate_corporate_isr, calculate_ieps, calculate_individual_isr, calculate_iva,
    calculate_sugary_drinks_ieps, calculate_with_iva, extract_iva_from_total, validate_taxpayer,
};

// Re-export data protection types
pub use data_protection::{
    ARCORight, DataCategory, DataProtectionError, DataSubject, LegalBasis, PersonalDataProcessing,
    PrivacyNotice,
};

// Re-export company law types
pub use company_law::{
    CompanyError, CompanyType, GovernanceRequirements, LimitedLiabilityCompany, StockCorporation,
    VariableCapitalCompany,
};

// Re-export intellectual property types
pub use intellectual_property::{
    Copyright, IPError, IPType, IndustrialDesign, Patent, PatentType, Trademark, TrademarkType,
    WorkType, protection_duration,
};

// Re-export competition law types
pub use competition_law::{
    AbsolutePractice, AnticompetitivePractice, CompetitionError, ConcentrationType,
    MarketConcentration, MarketParticipant, RelativePractice, thresholds,
};

// Re-export reasoning types
pub use reasoning::{InterpretationMethod, LegalPrinciple, LegalReasoning, principles};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_civil_code_citation() {
        let cite = format_civil_code_citation(1792, None, None);
        assert_eq!(cite, "Código Civil Federal, Artículo 1792");
    }

    #[test]
    fn test_labor_law_citation() {
        let cite = format_labor_law_citation(61, None, Some(RomanNumeral::I));
        assert_eq!(cite, "Ley Federal del Trabajo, Artículo 61, fracción I");
    }

    #[test]
    fn test_currency() {
        let amount = MexicanCurrency::from_pesos(300);
        assert_eq!(amount.pesos(), 300);
    }

    #[test]
    fn test_state_info() {
        let cdmx = MexicanState::CMX;
        assert_eq!(cdmx.nombre_es(), "Ciudad de México");
        assert_eq!(cdmx.region_es(), "Centro");
    }

    #[test]
    fn test_rfc_validation() {
        assert!(validate_rfc("XAXX010101000").is_ok());
        assert!(validate_rfc("123").is_err());
    }

    #[test]
    fn test_aguinaldo_calculation() {
        let daily_salary = MexicanCurrency::from_pesos(300);
        let aguinaldo = calculate_aguinaldo(daily_salary, 365);
        assert_eq!(aguinaldo.pesos(), 4500); // 15 days * 300
    }

    #[test]
    fn test_iva_calculation() {
        let base = MexicanCurrency::from_pesos(1000);
        let iva = calculate_iva(base, IVARate::Standard);
        assert_eq!(iva.pesos(), 160); // 16% of 1000
    }
}
