//! French jurisdiction support for Legalis-RS.
//!
//! This crate provides structured representations of French law, including:
//! - Code civil (Civil Code) - Napoleonic Code of 1804, Contract law (2016 reform)
//! - Code de commerce (Commercial Code) - Company law (SA/SARL/SAS)
//! - Code du travail (Labor Code) - Employment law, 35-hour week, dismissal regulations
//! - Constitution de 1958 (Fifth Republic Constitution) - All 16 titles, 89 articles
//! - Bilingual (French/English) statute handling
//!
//! ## Modules
//!
//! - **code_civil**: Tort law (Articles 1240-1242)
//! - **contract**: Contract law (Articles 1103-1231+) - 2016 reform
//! - **company**: Company law (SA/SARL/SAS formation and governance)
//! - **labor**: Labor law (CDI/CDD, 35-hour week, dismissal rules)
//! - **constitution**: Constitution de 1958 (16 titles, 89 articles)
//! - **family**: Family law (Marriage, divorce, property regimes, PACS)
//! - **inheritance**: Inheritance law (Succession, wills, reserved portions)
//! - **property**: Property law (Ownership rights, easements, accession - Articles 544-572)
//! - **evidence**: Evidence law (Burden of proof, presumptions, electronic evidence - Articles 1353-1378)
//! - **intellectual_property**: IP law (Patents, copyright, trademarks, designs - CPI Books I, V, VI, VII)
//! - **reasoning**: Legal reasoning engine for automated analysis
//!
//! ## Example
//!
//! ```
//! use legalis_fr::contract::{Contract, ContractType, validate_contract_validity};
//!
//! let contract = Contract::new()
//!     .with_type(ContractType::Sale {
//!         price: 50_000,
//!         subject: "Machine".to_string()
//!     })
//!     .with_parties(vec!["Buyer".to_string(), "Seller".to_string()])
//!     .with_consent(true);
//!
//! assert!(validate_contract_validity(&contract).is_ok());
//! ```

pub mod code_civil;
pub mod company;
pub mod constitution;
pub mod contract;
pub mod evidence;
pub mod family;
pub mod inheritance;
pub mod intellectual_property;
pub mod labor;
pub mod property;
pub mod reasoning;

// Re-export tort law
pub use code_civil::{article_1240, article_1241, article_1242};

// Re-export contract law
pub use contract::{
    BreachType, Contract, ContractError, ContractType, ValidityDefect, article1103, article1128,
    article1217, article1231, calculate_contract_damages, validate_contract_validity,
};

// Re-export company law
pub use company::{
    ArticlesOfIncorporation, BoardOfDirectors, Capital, CompanyLawError, CompanyType, Director,
    Shareholder, article_l225_1, article_l225_17, article_l225_18,
    validate_articles_of_incorporation, validate_sa_board,
};

// Re-export labor law
pub use labor::{
    CDDReason, DismissalType, EmploymentContract, EmploymentContractType, LaborLawError,
    PersonalCause, SMIC_HOURLY, TrialPeriodCategory, WorkingHours, article_l1221_1,
    article_l1221_19, article_l1231_1, article_l1232_1, article_l1232_2, article_l1233_3,
    article_l1234_1, article_l1242_2, article_l1242_8, article_l1242_12, article_l3121_18,
    article_l3121_20, article_l3121_27, article_l3121_33, article_l3121_34, validate_cdd,
    validate_dismissal, validate_employment_contract, validate_minimum_wage,
    validate_notice_period, validate_trial_period, validate_working_hours,
};

// Re-export constitution
pub use constitution::{
    ConstitutionArticle, ConstitutionTitle, FundamentalRight, Institution, all_titles, get_title,
    total_article_count,
};

// Re-export family law
pub use family::{
    // Types
    Asset,
    BilingualString,

    Child,
    Divorce,
    DivorceType,
    FamilyLawError,
    FamilyLawResult,
    FaultType,
    MaritalStatus,
    Marriage,
    MarriageOpposition,
    Nationality,
    OppositionGround,
    PACS,
    PACSPropertyRegime,
    Person,
    PropertyRegime,
    Relationship,
    // Marriage articles and validators
    article143,
    article144,
    article146,
    article146_1,
    article147,
    article161,
    article165,
    article180,
    // Divorce articles and validators
    article229,
    article230,
    article233,
    article237,
    article242,
    article247,
    // Property regime articles and validators
    article1387,
    article1400,
    article1401,
    article1404,
    article1536,
    check_oppositions,

    is_default_regime,
    regime_name_en,

    regime_name_fr,
    validate_acceptance_principle_divorce,
    validate_banns_publication,
    validate_consent,
    validate_definitive_alteration_divorce,
    validate_divorce,
    validate_divorce_proceedings,
    validate_fault_divorce,

    validate_marriage,
    validate_marriage_conditions,
    validate_minimum_age,
    validate_mutual_consent_divorce,
    validate_no_bigamy,
    validate_no_consanguinity,
    // PACS validators
    validate_pacs,
    validate_pacs_dissolution,
    validate_pacs_property_regime,
    validate_personal_presence,
    validate_property_regime,
    validate_property_regime_contract,
};

// Re-export inheritance law
pub use inheritance::{
    // Types
    Asset as InheritanceAsset,
    Debt,
    Disposition,
    DispositionType,
    Heir,
    // Error types
    InheritanceLawError,
    InheritanceLawResult,

    Person as InheritancePerson,
    Relationship as InheritanceRelationship,
    ReservedPortion,
    Succession,
    Will,
    WillType,

    // Succession articles
    article720,
    article721,
    article724,
    article735,
    // Will articles
    article774_792,
    article839_851,
    article873,

    article893_894,

    // Reserved portion articles
    article912,
    article913,
    article1493,

    validate_heir_shares,
    validate_reserved_portion,
    // Validators
    validate_succession,
    validate_will,
};

// Re-export property law
pub use property::{
    // Types
    Asset as PropertyAsset,
    AssetType,
    Easement,
    EasementType,
    Encumbrance,
    EncumbranceType,
    Property,
    PropertyLawError,
    PropertyLawResult,
    PropertyType,
    // Ownership articles
    article544,
    article545,
    article546,
    article548,
    article571_572,
    // Validation functions
    validate_easement,
    validate_ownership,
    validate_property,
    validate_transaction,
};

// Re-export evidence law
pub use evidence::{
    // Types
    BurdenOfProof,
    Evidence,
    EvidenceLawError,
    EvidenceLawResult,
    EvidenceType,
    ExpertReport,
    PresumptionType,
    WitnessTestimony,
    // Burden of proof articles
    article1353,
    article1354,
    article1355,
    // Validators
    validate_burden_of_proof,
    validate_evidence,
    validate_presumption,
};

// Re-export intellectual property law
pub use intellectual_property::{
    // Core types
    Copyright,
    CopyrightBuilder,
    // Error types
    CopyrightErrorKind,
    Design,
    DesignBuilder,
    DesignErrorKind,
    IPLawError,
    IPLawResult,
    Patent,
    PatentBuilder,
    PatentErrorKind,
    Trademark,
    TrademarkBuilder,
    TrademarkErrorKind,
    WorkType,
    // Article functions
    article_l122_1,
    article_l123_1,
    article_l511_1,
    article_l513_1,
    article_l611_10,
    article_l611_11,
    article_l711_1,
    article_l712_1,
    // Validation functions
    validate_copyright,
    validate_copyright_duration,
    validate_copyright_originality,
    validate_design,
    validate_design_duration,
    validate_design_individual_character,
    validate_design_novelty,
    validate_patent,
    validate_patent_duration,
    validate_patent_industrial_applicability,
    validate_patent_inventive_step,
    validate_patent_novelty,
    validate_trademark,
    validate_trademark_classes,
    validate_trademark_distinctiveness,
    validate_trademark_duration,
};

// Re-export reasoning engine
pub use reasoning::{
    CompanyAnalyzer, ComplianceStatus, ContractAnalyzer, EntityType, FrenchLawAnalyzer,
    LaborAnalyzer, LegalAnalysis, LegalOpinion, LegalReasoningEngine, ReasoningError,
    ReasoningResult, ReasoningStep, Remedy, RemedyType, RiskLevel, Violation, ViolationSeverity,
    all_french_statutes, company_law_statutes, contract_law_statutes, labor_law_statutes,
};
