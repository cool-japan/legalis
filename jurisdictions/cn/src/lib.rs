//! # Legalis-CN: China Jurisdiction Support
//!
//! # 中国法律框架 / Chinese Legal Framework
//!
//! Comprehensive implementation of Chinese law for the Legalis ecosystem.
//!
//! ## Legal System Overview / 法律体系概述
//!
//! China operates under a socialist civil law system with Chinese characteristics.
//! The legal hierarchy is:
//!
//! 1. **宪法 (Constitution)** - Supreme law
//! 2. **法律 (Laws)** - Enacted by National People's Congress
//! 3. **行政法规 (Administrative Regulations)** - Issued by State Council
//! 4. **地方性法规 (Local Regulations)** - Provincial/municipal legislation
//! 5. **规章 (Rules)** - Departmental and local government rules
//!
//! ## Implemented Modules / 已实现模块
//!
//! ### Data Protection / 数据保护
//!
//! - **PIPL (个人信息保护法)** - Personal Information Protection Law
//! - **Cybersecurity Law (网络安全法)** - Network security requirements
//! - **Data Security Law (数据安全法)** - Data classification and protection
//!
//! ### Civil Law / 民法
//!
//! - **Civil Code (民法典)** - Comprehensive civil code effective 2021
//!   - Book 1: General Provisions (总则编)
//!   - Book 2: Property Rights (物权编)
//!   - Book 3: Contracts (合同编)
//!   - Book 4: Personality Rights (人格权编)
//!   - Book 5: Marriage and Family (婚姻家庭编)
//!   - Book 6: Succession (继承编)
//!   - Book 7: Tort Liability (侵权责任编)
//!
//! ### Corporate Law / 公司法
//!
//! - Company formation and governance
//! - State-owned enterprise provisions
//! - Foreign-invested enterprises
//!
//! ### Labor Law / 劳动法
//!
//! - Labor Contract Law (劳动合同法)
//! - Employment relationships
//! - Social insurance (五险一金)
//!
//! ### Foreign Investment / 外商投资
//!
//! - Foreign Investment Law (外商投资法)
//! - Negative list system
//! - National treatment
//!
//! ### Antitrust / 反垄断
//!
//! - Anti-Monopoly Law (反垄断法)
//! - SAMR merger review
//! - Abuse of dominance
//!
//! ## Bilingual Support / 双语支持
//!
//! All types support both Chinese (中文) and English text.
//! Chinese text is authoritative in legal interpretation.
//!
//! ```rust
//! use legalis_cn::i18n::BilingualText;
//!
//! let text = BilingualText::new("个人信息保护法", "PIPL");
//! assert_eq!(text.zh, "个人信息保护法");
//! assert_eq!(text.en, "PIPL");
//! ```
//!
//! ## Citation Format / 引用格式
//!
//! Chinese legal citations follow the format: 《法律名称》第X条第Y款第Z项
//!
//! ```rust
//! use legalis_cn::citation::{cite, Citation};
//!
//! let citation = cite::pipl(13);
//! assert_eq!(citation.format_chinese(), "《中华人民共和国个人信息保护法》第13条");
//! ```
//!
//! ## Key Legislation / 主要法律
//!
//! | Law (法律) | Effective Date | Description |
//! |------------|----------------|-------------|
//! | 民法典 | 2021-01-01 | Comprehensive civil code |
//! | 个人信息保护法 | 2021-11-01 | Personal information protection |
//! | 网络安全法 | 2017-06-01 | Cybersecurity requirements |
//! | 数据安全法 | 2021-09-01 | Data security classification |
//! | 公司法 | 2024-07-01 | Company law (2023 revision) |
//! | 劳动合同法 | 2008-01-01 | Labor contract regulation |
//! | 外商投资法 | 2020-01-01 | Foreign investment framework |
//! | 反垄断法 | 2022-08-01 | Antitrust (2022 revision) |

#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

pub mod citation;
pub mod common;
pub mod company_law;
pub mod cybersecurity;
pub mod data_protection;
pub mod i18n;
pub mod labor_contract;

// New modules
pub mod antitrust;
pub mod civil_code;
pub mod data_security;
pub mod foreign_investment;
pub mod reasoning;

// Re-export commonly used types
pub use citation::{Citation, cite, laws};
pub use common::{currency, dates, names};
pub use i18n::{BilingualText, Locale};

// Re-export data protection types
pub use data_protection::{
    // Cross-border
    AssessmentResult,
    // Types
    ConsentMethod,
    ConsentRecord,
    ConsentType,
    ContactInfo,
    CrossBorderTransferRecord,
    DataProtectionOfficer,
    HandlerCategory,
    IndividualRight,
    OverseasRecipient,
    // Errors
    PenaltyRange,
    PersonalInformationCategory,
    PersonalInformationHandler,
    // Validator
    PiplComplianceReport,
    PiplError,
    PiplResult,
    PrivacyPolicy,
    ProcessingActivityRecord,
    ProcessingBasis,
    ProcessingVolume,
    RequiredMechanism,
    SensitivePersonalInformation,
    TransferMechanism,
    WithdrawalRecord,
    determine_transfer_mechanism,
    standard_contract_required_clauses,
    validate_automated_decision,
    validate_consent,
    validate_cross_border_transfer,
    validate_handler_compliance,
    validate_minor_processing,
    validate_privacy_policy,
    validate_processing_record,
    validate_security_measures,
};

// Re-export cybersecurity types
pub use cybersecurity::{
    // MLPS
    AssessorQualification,
    // Types
    CiiSector,
    ControlDomain,
    ControlsAssessment,
    // Validator
    CybersecurityComplianceReport,
    // Errors
    CybersecurityError,
    CybersecurityPenalty,
    CybersecurityResult,
    DataVolume,
    FilingStatus,
    Finding,
    FindingSeverity,
    IncidentSeverity,
    LevelDeterminationFactors,
    MlpsAssessment,
    MlpsLevel,
    NetworkOperator,
    NetworkOperatorCategory,
    RemediationStatus,
    ReviewTrigger,
    SecurityContact,
    SecurityIncident,
    SocialImpact,
    SystemType,
    ThirdPartyAssessor,
    check_review_required,
    determine_mlps_level,
    validate_cii_data_localization,
    validate_incident_reporting,
    validate_operator_compliance,
};

// Re-export company law types
pub use company_law::{
    // Types
    BoardOfDirectors,
    CapitalReductionMethod,
    // Validator
    CompanyComplianceReport,
    // Errors
    CompanyLawError,
    CompanyLawResult,
    CompanyRegistration,
    CompanyStatus,
    CompanyType,
    ContributionMethod,
    Director,
    DirectorPosition,
    DissolutionReason,
    DividendDistribution,
    EquityTransfer,
    LiabilityType,
    ResolutionType,
    Shareholder,
    ShareholderRight,
    ShareholderType,
    SpecialResolutionMatter,
    Supervisor,
    SupervisoryBoard,
    check_director_eligibility,
    check_veil_piercing_risk,
    dissolution_procedures,
    validate_board,
    validate_capital_contribution,
    validate_company_formation,
    validate_dividend_distribution,
    validate_equity_transfer,
    validate_resolution,
    validate_supervisory_board,
};

// Re-export labor contract types
pub use labor_contract::{
    // Types
    AnnualLeaveEntitlement,
    ContractType,
    DispatchPositionType,
    EconomicLayoff,
    EmploymentStatus,
    HousingFundStatus,
    // Validator
    LaborComplianceReport,
    LaborContract,
    // Errors
    LaborContractError,
    LaborContractResult,
    LaborDispatch,
    LaborPenalty,
    LayoffReason,
    NonCompeteAgreement,
    OvertimeType,
    ProbationLimit,
    ProtectedCategory,
    SeveranceCalculation,
    SocialInsuranceStatus,
    TerminationReason,
    WorkingHoursType,
    calculate_double_wages_penalty,
    calculate_illegal_termination_compensation,
    calculate_service_years,
    calculate_severance,
    should_offer_open_ended,
    validate_annual_leave,
    validate_contract,
    validate_dispatch,
    validate_economic_layoff,
    validate_minimum_wage,
    validate_non_compete,
    validate_overtime_pay,
    validate_termination,
};

// Re-export common types
pub use common::currency::CnyAmount;
pub use common::dates::{DeadlineType, PublicHoliday};
pub use common::names::{ChineseName, CompanyName, OrganizationForm};

// Re-export civil code types
pub use civil_code::{
    // General Provisions
    ActNature,
    ActValidity,
    // Marriage and Family
    Adoption,
    Agency,
    AgencyType,
    // Contracts
    BreachOfContract,
    BreachRemedy,
    BreachType,
    // Property Rights
    ConstructionLandTerm,
    ConstructionLandUseRight,
    Contract,
    ContractFormationStatus,
    ContractType as CivilContractType,
    ContractValidityStatus,
    ContractsError,
    ContractsResult,
    // Succession
    DisinheritanceReason,
    DivorceGrounds,
    DivorceType,
    // Tort Liability
    EnvironmentalPollution,
    Estate,
    GeneralProvisionsError,
    GeneralProvisionsResult,
    Heir,
    HeirType,
    HighlyDangerousActivity,
    // Personality Rights
    ImageUse,
    IntestateSuccessionOrder,
    JuristicAct,
    LandContractualManagementRight,
    LeaseContract,
    LegalCapacity,
    LegalPerson,
    LegalPersonType,
    LiabilityPrinciple,
    Lien,
    LimitationPeriod,
    MaritalPropertyRegime,
    Marriage,
    MarriageFamilyError,
    MarriageFamilyResult,
    MarriageRequirements,
    MedicalMalpractice,
    Mortgage,
    NaturalPerson,
    OwnershipType,
    ParentChildRelationship,
    PerformancePeriod,
    PersonalInfoProcessingPrinciple,
    PersonalInformation,
    PersonalityRight,
    PersonalityRightsError,
    PersonalityRightsResult,
    Pledge,
    PrivacyInfringement,
    PrivacyInfringementMethod,
    PrivacyScope,
    ProductDefectType,
    ProductsLiability,
    Property,
    PropertyRightType,
    PropertyRightsError,
    PropertyRightsResult,
    ReputationInfringement,
    ReputationInfringementType,
    ResidentialLandUseRight,
    SaleContract,
    SuccessionError,
    SuccessionResult,
    SuccessionType,
    SupportObligation,
    Tort,
    TortDefense,
    TortLiabilityError,
    TortLiabilityResult,
    TortType,
    UnincorporatedOrganization,
    Will,
    WillType,
    calculate_damages,
    calculate_damages_with_defense,
    calculate_intestate_shares,
    calculate_security_interest_priority,
    check_disinheritance,
    check_limitation_period,
    check_privacy_infringement,
    check_reputation_infringement,
    determine_breach_remedies,
    validate_adoption,
    validate_agency,
    validate_construction_land_use_right,
    validate_contract_formation,
    validate_contract_validity,
    validate_divorce_by_mutual_consent,
    validate_image_use,
    validate_juristic_act,
    validate_lease_term,
    validate_marriage_eligibility,
    validate_medical_malpractice,
    validate_mortgage,
    validate_personal_info_processing,
    validate_pledge,
    validate_products_liability,
    validate_tort_liability,
    validate_will,
};

// Re-export data security types
pub use data_security::{
    CrossBorderDataTransfer as DataSecurityCrossBorderTransfer, DataClassification,
    DataProcessingActivity, DataProcessingRecord, DataProcessor, DataSecurityError,
    DataSecurityObligation, DataSecurityResult, DataSecurityReview, ReviewResult, ReviewStatus,
    determine_data_classification, requires_security_review,
    validate_cross_border_transfer as validate_data_security_cross_border_transfer,
    validate_data_processor,
};

// Re-export foreign investment types
pub use foreign_investment::{
    ForeignInvestedEnterprise, ForeignInvestmentError, ForeignInvestmentProject,
    ForeignInvestmentResult, ForeignInvestor, ForeignInvestorType, InvestmentForm,
    NegativeListCategory, Sector, SecurityReview, SecurityReviewResult, SecurityReviewStatus,
    check_sector_openness, validate_fie_compliance, validate_foreign_investment_project,
};

// Re-export antitrust types
pub use antitrust::{
    AbuseOfDominance, AbuseType, AntitrustError, AntitrustResult, ConcentrationReviewDecision,
    ConcentrationTransaction, ConcentrationType, MarketDominance, MonopolisticConductType,
    MonopolyAgreement, MonopolyAgreementType, validate_abuse_of_dominance,
    validate_concentration_transaction, validate_monopoly_agreement,
};

// Re-export reasoning types
pub use reasoning::{
    ConflictResolutionRule, GuidingCase, InterpretationMethod, LegalAnalysis, LegalConclusion,
    LegalFact, LegalHierarchy, LegalIssue, LegalProvision, ReasoningError, ReasoningResult,
    apply_analogical_reasoning, build_argument_chain, find_applicable_provisions, resolve_conflict,
    validate_reasoning,
};

use legalis_core::{Effect, EffectType, Statute};

// ============================================================================
// Statute Builders
// ============================================================================

/// Create PIPL statute
pub fn create_pipl_statute() -> Statute {
    Statute::new(
        "CN-PIPL-2021",
        "中华人民共和国个人信息保护法 / Personal Information Protection Law",
        Effect::new(
            EffectType::Obligation,
            "个人信息保护 / Personal information protection",
        ),
    )
    .with_jurisdiction("CN")
}

/// Create Cybersecurity Law statute
pub fn create_cybersecurity_statute() -> Statute {
    Statute::new(
        "CN-CSL-2017",
        "中华人民共和国网络安全法 / Cybersecurity Law",
        Effect::new(
            EffectType::Obligation,
            "网络安全保护 / Network security protection",
        ),
    )
    .with_jurisdiction("CN")
}

/// Create Data Security Law statute
pub fn create_data_security_statute() -> Statute {
    Statute::new(
        "CN-DSL-2021",
        "中华人民共和国数据安全法 / Data Security Law",
        Effect::new(
            EffectType::Obligation,
            "数据安全保护 / Data security protection",
        ),
    )
    .with_jurisdiction("CN")
}

/// Create Civil Code statute
pub fn create_civil_code_statute() -> Statute {
    Statute::new(
        "CN-CC-2020",
        "中华人民共和国民法典 / Civil Code",
        Effect::new(
            EffectType::Grant,
            "民事法律关系 / Civil legal relationships",
        ),
    )
    .with_jurisdiction("CN")
}

/// Create Company Law statute
pub fn create_company_law_statute() -> Statute {
    Statute::new(
        "CN-CL-2023",
        "中华人民共和国公司法 / Company Law",
        Effect::new(
            EffectType::Grant,
            "公司设立与治理 / Company formation and governance",
        ),
    )
    .with_jurisdiction("CN")
}

/// Create Labor Contract Law statute
pub fn create_labor_contract_statute() -> Statute {
    Statute::new(
        "CN-LCL-2008",
        "中华人民共和国劳动合同法 / Labor Contract Law",
        Effect::new(EffectType::Obligation, "劳动合同 / Labor contracts"),
    )
    .with_jurisdiction("CN")
}

/// Create Foreign Investment Law statute
pub fn create_foreign_investment_statute() -> Statute {
    Statute::new(
        "CN-FIL-2019",
        "中华人民共和国外商投资法 / Foreign Investment Law",
        Effect::new(EffectType::Grant, "外商投资 / Foreign investment"),
    )
    .with_jurisdiction("CN")
}

/// Create Anti-Monopoly Law statute
pub fn create_anti_monopoly_statute() -> Statute {
    Statute::new(
        "CN-AML-2022",
        "中华人民共和国反垄断法 / Anti-Monopoly Law",
        Effect::new(
            EffectType::Prohibition,
            "垄断行为禁止 / Prohibition of monopolistic conduct",
        ),
    )
    .with_jurisdiction("CN")
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_pipl_statute() {
        let statute = create_pipl_statute();
        assert!(statute.id.contains("PIPL"));
        assert!(statute.title.contains("个人信息保护法"));
    }

    #[test]
    fn test_bilingual_text() {
        let text = BilingualText::new("测试", "Test");
        assert_eq!(text.zh, "测试");
        assert_eq!(text.en, "Test");
    }

    #[test]
    fn test_citation_format() {
        let citation = cite::pipl(13);
        let formatted = citation.format_chinese();
        assert!(formatted.contains("个人信息保护法"));
        assert!(formatted.contains("第13条"));
    }

    #[test]
    fn test_cny_amount() {
        let amount = CnyAmount::from_wan(100.0); // 100万元
        assert_eq!(amount.format_chinese(), "100.00万元");
    }

    #[test]
    fn test_organization_form() {
        let form = OrganizationForm::LimitedLiabilityCompany;
        assert_eq!(form.name_zh(), "有限责任公司");
    }
}
