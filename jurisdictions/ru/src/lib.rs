//! Russian Federation jurisdiction support for Legalis-RS.
//!
//! This crate provides comprehensive support for Russian legal system:
//! - Civil Code (Гражданский кодекс РФ) - 4 parts
//! - Criminal Code (Уголовный кодекс РФ)
//! - Labor Code (Трудовой кодекс РФ)
//! - Tax Code (Налоговый кодекс РФ) - VAT, Income Tax, Corporate Tax
//! - Federal Law 152-FZ on Personal Data (О персональных данных)
//! - Federal Law on LLC (14-FZ) and JSC
//! - Protection of Competition Law (135-FZ)
//! - Intellectual Property Law
//! - Citation formatting for Russian legal documents
//! - Bilingual support (Russian/English)
//! - Russian Ruble (RUB) currency handling

pub mod citation;
pub mod civil_code;
pub mod common;
pub mod company_law;
pub mod competition_law;
pub mod criminal_code;
pub mod data_protection;
pub mod intellectual_property;
pub mod labor_code;
pub mod reasoning;
pub mod tax_code;

// Citation exports
pub use citation::{Citation, CitationFormatter, CitationStyle, DocumentType, LegalDocument};

// Civil Code exports (Гражданский кодекс)
pub use civil_code::{
    CivilCodeError, CivilLaw, ContractType, ObligationType, PropertyRight, PropertyType,
    SuccessionOrder, SuccessionRights, quick_validate_contract, validate_property_rights,
};

// Common utilities exports
pub use common::{
    Currency, Holiday, RussianLegalCalendar, calculate_business_days, format_ruble,
    is_russian_holiday, is_working_day,
};

// Company Law exports
pub use company_law::{
    CompanyError, CompanyType, FounderContribution, GovernanceStructure, JointStockCompany,
    LimitedLiabilityCompany, ShareholderRights, quick_validate_llc,
};

// Competition Law exports (135-FZ)
pub use competition_law::{
    CompetitionError, CompetitionViolation, DominantPosition, MarketShare, Monopoly,
    quick_validate_market_dominance,
};

// Criminal Code exports (Уголовный кодекс)
pub use criminal_code::{
    Crime, CrimeCategory, CriminalCodeError, CriminalLiability, PunishmentType, Sanction,
    quick_validate_criminal_liability,
};

// Data Protection exports (152-FZ)
pub use data_protection::{
    ConsentType, DataCategory, DataProtectionError, DataSubject, DataSubjectRights,
    PersonalDataOperator, ProcessingPurpose, SecurityMeasure, ThirdPartyTransfer,
    quick_validate_data_processing,
};

// Intellectual Property exports
pub use intellectual_property::{
    Copyright, IntellectualPropertyError, Patent, PatentType, Trademark, WorkType,
    quick_validate_copyright,
};

// Labor Code exports (Трудовой кодекс)
pub use labor_code::{
    EmploymentContract, EmploymentType, LaborCodeError, LaborRights, TerminationGround,
    WorkingTimeRegime, quick_validate_employment_contract,
};

// Legal Reasoning Engine exports
pub use reasoning::{
    ComplianceStatus, LegalAnalysis, ReasoningEngine, ReasoningError, ReasoningResult, RiskLevel,
    RuEvaluationContext, Violation, ViolationSeverity,
};

// Tax Code exports (Налоговый кодекс)
pub use tax_code::{
    CorporateTaxCalculation, IncomeTaxCalculation, TaxCodeError, TaxRate, VatCalculation, VatRate,
    calculate_corporate_tax, calculate_income_tax, calculate_vat, quick_validate_tax_calculation,
};
