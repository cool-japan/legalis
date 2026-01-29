//! # Legalis-BR: Brazilian Law Library
//!
//! Comprehensive Brazilian legal framework implementation in Pure Rust.
//!
//! ## Overview
//!
//! Brazil operates under a **Civil Law** system, part of the Romano-Germanic tradition.
//! This library provides validation, types, and utilities for key Brazilian legislation:
//!
//! | Domain | Law | Year | Key Provisions |
//! |--------|-----|------|----------------|
//! | Consumer | CDC (Lei 8.078) | 1990 | Consumer as vulnerable party, strict liability |
//! | Labor | CLT (Lei 5.452) | 1943 | 44h/week, 13th salary, FGTS |
//! | Civil | Código Civil (Lei 10.406) | 2002 | Contracts, property, obligations |
//! | Data Protection | LGPD (Lei 13.709) | 2018 | Brazil's GDPR (10 legal bases) |
//! | Corporate | Lei das S.A. (Lei 6.404) | 1976 | Corporations, governance |
//! | Constitution | CF/88 | 1988 | Fundamental rights, federalism |
//!
//! ## Brazilian Legal Citation Format
//!
//! Brazilian citations follow a specific format:
//!
//! ```text
//! Lei nº [number]/[year], Art. [article]º, §[paragraph]º, inciso [clause]
//! ```
//!
//! ### Examples
//!
//! ```rust
//! use legalis_br::citation::*;
//!
//! // Consumer Defense Code
//! let cdc = format_cdc_citation(5, None, None);
//! assert_eq!(cdc, "Lei nº 8.078/1990, Art. 5º");
//!
//! // CLT with paragraph
//! let clt = format_clt_citation(58, Some(1), None);
//! assert_eq!(clt, "Lei nº 5.452/1943, Art. 58º, §1º");
//!
//! // Constitution with clause (inciso)
//! let cf = format_constitution_citation(5, None, Some(RomanNumeral::X));
//! assert_eq!(cf, "Constituição Federal, Art. 5º, inciso X");
//! ```
//!
//! ## Currency: Brazilian Real (BRL)
//!
//! ```rust
//! use legalis_br::common::BrazilianCurrency;
//!
//! let salary = BrazilianCurrency::from_reais(1412); // Minimum wage 2024
//! assert_eq!(salary.to_string(), "R$ 1412,00");
//!
//! let fine = BrazilianCurrency::from_centavos(150050);
//! assert_eq!(fine.reais(), 1500);
//! assert_eq!(fine.cents(), 50);
//! ```
//!
//! ## Document Validation (CPF/CNPJ)
//!
//! ```rust
//! use legalis_br::common::{validate_cpf, validate_cnpj};
//!
//! // CPF - Individual taxpayer ID (11 digits)
//! assert!(validate_cpf("123.456.789-09").is_ok());
//!
//! // CNPJ - Corporate taxpayer ID (14 digits)
//! assert!(validate_cnpj("11.222.333/0001-81").is_ok());
//! ```
//!
//! ## Brazilian States
//!
//! ```rust
//! use legalis_br::common::BrazilianState;
//!
//! let sp = BrazilianState::SP;
//! assert_eq!(sp.nome_pt(), "São Paulo");
//! assert_eq!(sp.region_pt(), "Sudeste");
//! assert_eq!(sp.abbreviation(), "SP");
//! ```
//!
//! ## Key Legal Principles
//!
//! ### Consumer Law (CDC)
//!
//! | Principle | Description |
//! |-----------|-------------|
//! | Vulnerability | Consumer presumed vulnerable (Art. 4) |
//! | Strict Liability | Manufacturer/provider liable regardless of fault (Arts. 12-14) |
//! | 7-Day Withdrawal | Right to withdraw from distance contracts (Art. 49) |
//! | Contract Interpretation | Ambiguities resolved in favor of consumer (Art. 47) |
//!
//! ### Labor Law (CLT)
//!
//! | Provision | Details |
//! |-----------|---------|
//! | Working Hours | 44h/week maximum (Art. 58) |
//! | 13th Salary | Christmas bonus = 1 month salary |
//! | Vacation | 30 days + 1/3 bonus |
//! | FGTS | 8% monthly severance fund |
//! | Notice Period | 30 days minimum |
//!
//! ### Data Protection (LGPD)
//!
//! | Aspect | LGPD (Brazil) |
//! |--------|---------------|
//! | Legal Bases | 10 (Art. 7) |
//! | Data Subject Rights | 9 (Art. 18) |
//! | DPO Required | Yes (Encarregado) |
//! | Authority | ANPD |
//! | Penalties | Up to 2% revenue (R$50M max) |
//!
//! ## Bilingual Support
//!
//! All types support both Portuguese (authoritative) and English translations:
//!
//! ```rust
//! use legalis_br::common::BrazilianState;
//!
//! let state = BrazilianState::MG;
//! assert_eq!(state.nome_pt(), "Minas Gerais"); // Portuguese
//! ```
//!
//! ## Module Structure
//!
//! - [`citation`] - Brazilian legal citation formatting
//! - [`common`] - Currency, dates, documents, states
//! - [`consumer_protection`] - CDC consumer defense code
//! - [`labor_law`] - CLT labor regulations
//! - [`data_protection`] - LGPD data protection
//! - [`civil_code`] - Código Civil (Lei 10.406/2002)
//! - [`criminal_code`] - Código Penal (Decreto-Lei 2.848/1940)
//! - [`tax_law`] - Tax system (ICMS, ISS, IPI, IRPF, IRPJ)
//! - [`corporate_law`] - Corporations, limited liability, CVM
//! - [`bankruptcy_law`] - Insolvency and reorganization (Lei 11.101/2005)
//! - [`intellectual_property`] - IP rights (Lei 9.279/1996)
//! - [`environmental_law`] - Environmental legislation
//! - [`banking_law`] - Banking and financial system regulations
//!
//! ## References
//!
//! - [Planalto - Federal Legislation](http://www.planalto.gov.br/)
//! - [ANPD - National Data Protection Authority](https://www.gov.br/anpd/)
//! - [TST - Superior Labor Court](https://www.tst.jus.br/)
//! - [STF - Supreme Federal Court](https://portal.stf.jus.br/)

// Documentation warnings disabled for initial development
// #![warn(missing_docs)]
// #![warn(rustdoc::missing_crate_level_docs)]

pub mod banking_law;
pub mod bankruptcy_law;
pub mod citation;
pub mod civil_code;
pub mod common;
pub mod consumer_protection;
pub mod corporate_law;
pub mod criminal_code;
pub mod data_protection;
pub mod environmental_law;
pub mod intellectual_property;
pub mod labor_law;
pub mod tax_law;

// Re-export citation types
pub use citation::{
    BrazilianCitation, RomanNumeral, format_cdc_citation, format_civil_code_citation,
    format_clt_citation, format_constitution_citation, format_law_number, format_lei_citation,
    format_lei_das_sa_citation, format_lgpd_citation, parse_roman_numeral, to_roman_numeral,
};

// Re-export common types
pub use common::{
    BrazilianCurrency, BrazilianDate, BrazilianDocument, BrazilianState, DocumentType,
    FederalEntity, Municipality, validate_cnpj, validate_cpf,
};

// Re-export consumer protection types
pub use consumer_protection::{
    AbusiveClause, AbusiveClauseType, CdcError, CdcResult, Consumer, ConsumerCompliance,
    ConsumerContract, ConsumerRight, ContractType, Product, ProductDefect, Provider,
    ProviderLiability, Recall, WithdrawalRight,
};

// Re-export data protection types
pub use data_protection::{
    DataSubjectRight, LegalBasis, LgpdCompliance, LgpdError, LgpdResult, PersonalDataProcessing,
    ProcessingPurpose, SecurityIncident,
};

// Re-export labor law types
pub use labor_law::{
    CltError, CltResult, EmploymentContract, EmploymentType, LaborCompliance, LaborRight,
    Severance, TerminationType, WorkingHours,
};

// Re-export civil code types
pub use civil_code::{
    CivilCodeError, CivilCodeResult, Contract, ContractError, ContractResult, FamilyError,
    FamilyResult, Inheritance, LegalAct, LegalActType, LegalPerson, LegalPersonType, Marriage,
    NaturalPerson, Obligation, ObligationType, ObligationsError, ObligationsResult, Ownership,
    Possession, PossessionType, PropertyError, PropertyKind, PropertyRegime, PropertyResult,
    StableUnion, SuccessionError, SuccessionResult, SuccessionType,
};

// Re-export criminal code types
pub use criminal_code::{
    Crime, CrimeCategory, CriminalCodeError, CriminalCodeResult, CriminalProcedure, Penalty,
    PenaltyType,
};

// Re-export tax law types
pub use tax_law::{
    IcmsError, IcmsResult, IcmsTransaction, IpiError, IpiResult, IpiTransaction, IrpfCalculation,
    IrpfError, IrpfResult, IrpjCalculation, IrpjError, IrpjResult, IssError, IssResult,
    IssTransaction, TaxError, TaxResult,
};

// Re-export corporate law types
pub use corporate_law::{
    Corporation, CorporationType, CorporationsError, CorporationsResult, CvmError, CvmResult,
    LimitedLiabilityCompany, LimitedLiabilityError, LimitedLiabilityResult, Partner,
    PublicOffering, Security, SecurityType,
};

// Re-export bankruptcy law types
pub use bankruptcy_law::{
    BankruptcyError, BankruptcyResult, CreditClass, Creditor, InsolvencyProceeding, ProceedingType,
    ReorganizationPlan,
};

// Re-export intellectual property types
pub use intellectual_property::{
    IntellectualProperty, IpError, IpResult, IpType, PatentabilityCheck, TrademarkType,
};

// Re-export environmental law types
pub use environmental_law::{
    EnvironmentalError, EnvironmentalLicense, EnvironmentalResult, ForestReserve, LicenseType,
};

// Re-export banking law types
pub use banking_law::{
    BankingError, BankingOperation, BankingResult, FinancialInstitution, InstitutionType,
    OperationType,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cdc_citation() {
        let cite = format_cdc_citation(49, None, None);
        assert_eq!(cite, "Lei nº 8.078/1990, Art. 49º");
    }

    #[test]
    fn test_lgpd_citation() {
        let cite = format_lgpd_citation(7, None, Some(RomanNumeral::I));
        assert_eq!(cite, "Lei nº 13.709/2018, Art. 7º, inciso I");
    }

    #[test]
    fn test_currency_formatting() {
        let amount = BrazilianCurrency::from_reais(1412);
        assert_eq!(amount.reais(), 1412);
    }

    #[test]
    fn test_state_info() {
        let sp = BrazilianState::SP;
        assert_eq!(sp.nome_pt(), "São Paulo");
        assert_eq!(sp.region_pt(), "Sudeste");
    }

    #[test]
    fn test_cpf_validation() {
        assert!(validate_cpf("12345678909").is_ok());
        assert!(validate_cpf("123").is_err());
    }

    #[test]
    fn test_cnpj_validation() {
        assert!(validate_cnpj("11222333000181").is_ok());
        assert!(validate_cnpj("123").is_err());
    }
}
