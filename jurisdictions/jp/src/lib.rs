//! Japanese jurisdiction support for Legalis-RS.
//!
//! This crate provides:
//! - Japanese era (和暦) handling
//! - e-Gov XML law parser (法令XML解析)
//! - e-Gov Electronic Filing System (電子申請システム) - XML/JSON application management
//! - Administrative Procedure Act (行政手続法) - Procedures and electronic signatures
//! - Construction & Real Estate Acts (建設業法・宅建業法) - Licensing and transactions
//! - Environmental Law (環境法) - Pollution control and waste management
//! - Personal Information Protection Act (個人情報保護法) - Data protection and privacy
//! - Japanese Constitution support
//! - Civil Code (民法 - Minpo) implementation
//! - Enhanced tort law API with builder pattern (Articles 709, 710, 715)
//! - Contract law API with builder pattern (Article 415)
//! - Commercial Law (商法・会社法) - Companies Act and Commercial Code
//! - Labor Law (労働法) - Labor Standards Act and Labor Contract Act
//! - Intellectual Property Law (知的財産法) - Patent, Copyright, Trademark, Design Acts
//! - Consumer Protection Law (消費者保護法) - Consumer Contract Act and SCTA
//! - Case Law Database (判例データベース) - Court decision search and citation
//! - Risk Analysis System (リスク分析システム) - Contract risk detection and compliance checking
//! - Contract Templates (契約テンプレート) - Automated contract generation from templates
//! - Bilingual (Japanese/English) statute handling

pub mod administrative_procedure; // Administrative Procedure Act (行政手続法)
pub mod case_law;
pub mod commercial_law;
pub mod common; // Common utilities (共通ユーティリティ) - holidays, working days
pub mod constitution;
pub mod construction_real_estate; // Construction & Real Estate Acts (建設業法・宅建業法)
pub mod consumer_protection;
pub mod contract;
pub mod contract_templates;
pub mod egov; // Electronic filing system (申請システム)
pub mod egov_law; // e-Gov law statute parser (法令XML解析)
pub mod environmental_law; // Environmental Law (環境法)
pub mod era;
pub mod intellectual_property;
pub mod labor_law;
pub mod law;
pub mod minpo;
pub mod personal_info_protection; // Personal Information Protection Act (個人情報保護法)
pub mod reasoning; // Legal Reasoning Engine (法的推論エンジン) - legalis-core integration
pub mod risk_analysis;
pub mod tort;

// Administrative Procedure Act exports (行政手続法)
pub use administrative_procedure::{
    AdministrativeError, AdministrativeFilingService, AdministrativeProcedure, Applicant,
    ApplicantType, Certificate, CertificateBuilder, ContactInfo, Document, DocumentType,
    ElectronicSignature, Identification, ProcedureBuilder, ProcedureType, SignatureAlgorithm,
    SignatureBuilder, quick_validate as quick_validate_procedure, validate_electronic_signature,
    validate_procedure, validate_signature_algorithm,
};

// Case Law Database exports (判例データベース)
pub use case_law::{
    CaseLawDatabase, CaseLawError, CaseLawSearchEngine, CitationFormatter, CourtDecision,
    CourtLevel, InMemoryCaseDatabase,
};

// Commercial Law exports (商法・会社法)
pub use commercial_law::{
    ArticlesOfIncorporation, Capital, CommercialLawError, CompanyType, ShareholdersMeeting,
};

// Construction & Real Estate Law exports (建設業法・宅建業法)
pub use construction_real_estate::{
    ConstructionBusinessLicense, ConstructionLicenseType, ConstructionRealEstateError,
    ConstructionType, LicensedAgent, LicensedBroker, Manager, ManagerQualification, Party,
    Property, PropertyType, RealEstateLicense, RealEstateTransaction, TransactionType,
    quick_validate_construction, quick_validate_real_estate, validate_construction_license,
    validate_real_estate_transaction,
};

// Constitution exports (憲法)
pub use constitution::{Constitution, ConstitutionArticle, ConstitutionChapter};

// Consumer Protection Law exports (消費者保護法)
pub use consumer_protection::{
    ConsumerContract, ConsumerProtectionError, CoolingOffExercise, SpecifiedCommercialTransaction,
};

// Environmental Law exports (環境法)
pub use environmental_law::{
    ControlEquipment, EmissionEstimate, EmissionLimit, EnvironmentalError, FacilityType,
    FactorySetupNotification, HeavyMetal, MonitoringRequirement, Pollutant,
    PollutionPreventionAgreement, PollutionType, WasteManagementPermit, WasteManifest,
    WastePermitType, WasteType, quick_validate_pollution, quick_validate_waste,
    validate_factory_setup_notification, validate_pollution_prevention_agreement,
    validate_waste_management_permit, validate_waste_manifest,
};

// Contract Law exports (契約法)
pub use contract::{
    Article415, Attribution, AttributionType, BreachLiability, BreachType, ContractLiabilityError,
    ObligationType, validate_breach_claim,
};

// Contract Templates exports (契約テンプレート)
pub use contract_templates::{
    ClauseLibrary, ContractTemplate, GeneratedContract, TemplateContext, TemplateEngine,
    TemplateError, TemplateType,
};

// e-Gov Law XML Parser exports (法令XML解析)
pub use egov_law::{EGovArticle, EGovLaw, EGovLawParser};

// e-Gov Electronic Filing System exports (電子申請システム)
pub use egov::{
    ApplicationMetadata, ApplicationStatus, Attachment, EgovApplication, EgovError, EgovFieldValue,
    EgovJsonFormatter, EgovXmlParser, GovernmentAgency, ValidationReport, quick_validate,
    validate_application, validate_status_transition,
};

/// Re-export the egov module as egov_filing for clarity
pub mod egov_filing {
    pub use crate::egov::*;
}

// Era System exports (和暦)
pub use era::{Era, EraError, JapaneseDate};

// Intellectual Property Law exports (知的財産法)
pub use intellectual_property::{
    CopyrightedWork, IntellectualPropertyError, PatentApplication, TrademarkApplication,
};

// Labor Law exports (労働法)
pub use labor_law::{
    EmploymentContract, EmploymentType, LaborLawError, TerminationNotice, WorkingTimeRecord,
};

// Personal Information Protection Act exports (個人情報保護法)
pub use personal_info_protection::{
    AiRiskAssessment, AppiError, BusinessType, CrossBorderTransfer, DataHandlingVolume,
    DataSubject, DataSubjectRequest, PersonalInfoType, PersonalInformationHandler, ProvisionType,
    PurposeType, RequestType, RiskLevel, RiskReport, SecurityMeasure, SecurityMeasureType,
    ThirdPartyProvision, UsagePurpose, assess_ai_risk, quick_validate_handler,
    quick_validate_request, validate_data_subject_request, validate_personal_info_handling,
};

// General Law System exports
pub use law::{JapaneseLaw, LawNumber, LawType};

// Civil Code (Minpo) exports (民法)
pub use minpo::{article_709, article_710, article_715_1};

// Risk Analysis System exports (リスク分析システム)
pub use risk_analysis::{
    ContractDocument, RiskAnalysisError, RiskAnalysisReport, RiskCategory, RiskDetector,
    RiskFinding, RiskSeverity, quick_analyze,
};

// Tort Law exports (不法行為法)
pub use tort::{
    Article709, Article710, Article715, CausalLink, Damage, Intent, ProtectedInterest,
    TortClaimError, TortLiability, validate_tort_claim,
};

// Legal Reasoning Engine exports (法的推論エンジン - legalis-core integration)
pub use reasoning::{
    ComplianceStatus, JpEvaluationContext, LegalAnalysis, LegalReasoningEngine, ReasoningError,
    ReasoningResult, RiskLevel as LegalRiskLevel, Violation, ViolationSeverity,
};

// Common utilities exports (共通ユーティリティ - legalis-i18n integration)
pub use common::{
    JapaneseLegalCalendar, calculate_legal_deadline, is_japanese_holiday, is_working_day,
};
