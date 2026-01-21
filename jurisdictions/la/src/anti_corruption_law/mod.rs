//! Anti-Corruption Law Module for Lao PDR (ກົດໝາຍຕ້ານການສໍ້ລາດບັງຫຼວງ)
//!
//! This module provides comprehensive support for Lao anti-corruption law based on
//! **Anti-Corruption Law 2012** (Law No. 03/NA, amended 2019).
//!
//! # Legal Framework
//!
//! The Anti-Corruption Law 2012 is the primary legislation governing anti-corruption
//! efforts in the Lao People's Democratic Republic. It establishes:
//!
//! - State Inspection Authority (SIA) structure and powers
//! - Definition of corruption offenses
//! - Asset declaration requirements for public officials
//! - Penalties and sanctions framework
//! - Whistleblower protection mechanisms
//! - Prevention measures and codes of conduct
//! - International cooperation (UNCAC compliance)
//!
//! # Key Provisions
//!
//! ## State Inspection Authority (SIA) - ອົງການກວດກາແຫ່ງລັດ
//!
//! **Articles 8-20** establish the State Inspection Authority:
//!
//! - **Article 8**: Organizational structure (central and provincial offices)
//! - **Article 10**: Inspection powers and jurisdiction
//! - **Article 12**: Investigation procedures
//! - **Article 15**: Cooperation with police and prosecutors
//!
//! ## Corruption Offenses - ການກະທຳຜິດສໍ້ລາດບັງຫຼວງ
//!
//! **Articles 25-45** define corruption offenses:
//!
//! - **Article 25**: Bribery (giving and receiving) - ການໃຫ້ ແລະ ຮັບສິນບົນ
//! - **Article 28**: Embezzlement of state funds - ການສໍ້ໂກງຊັບສິນຂອງລັດ
//! - **Article 32**: Abuse of position - ການໃຊ້ຕຳແໜ່ງໃນທາງທີ່ຜິດ
//! - **Article 35**: Nepotism and cronyism - ການລຳອຽງເພາະຍາດພີ່ນ້ອງ
//! - **Article 38**: Conflicts of interest - ຜົນປະໂຫຍດຂັດກັນ
//! - **Article 42**: Illicit enrichment - ການຮັ່ງມີທີ່ບໍ່ຊອບດ້ວຍກົດໝາຍ
//!
//! ## Asset Declaration - ການປະກາດຊັບສິນ
//!
//! **Articles 50-60** establish asset declaration requirements:
//!
//! - **Article 50**: Officials required to declare (by grade/position)
//! - **Article 52**: Declaration frequency (annual)
//! - **Article 55**: Content requirements (assets, liabilities, income)
//! - **Article 58**: Verification procedures
//! - **Article 60**: Public disclosure rules
//!
//! ## Penalties Framework - ໂຄງຮ່າງການລົງໂທດ
//!
//! **Articles 65-80** establish penalties:
//!
//! - **Minor corruption** (< 5 million LAK): 3 months - 1 year imprisonment
//! - **Medium corruption** (5-50 million LAK): 1-5 years imprisonment
//! - **Serious corruption** (50-500 million LAK): 5-10 years imprisonment
//! - **Very serious corruption** (> 500 million LAK): 10-20 years or life imprisonment
//!
//! ## Whistleblower Protection - ການປົກປ້ອງຜູ້ແຈ້ງຂ່າວ
//!
//! **Articles 85-95** establish whistleblower protection:
//!
//! - **Article 85**: Anonymous reporting mechanisms
//! - **Article 88**: Protection from retaliation
//! - **Article 90**: Rewards for valid reports
//! - **Article 93**: Confidentiality guarantees
//!
//! ## Prevention Measures - ມາດຕະການປ້ອງກັນ
//!
//! **Articles 100-115** establish prevention measures:
//!
//! - **Article 100**: Code of conduct for public officials
//! - **Article 105**: Procurement transparency
//! - **Article 108**: Gift restrictions and limits
//! - **Article 112**: Cooling-off periods
//!
//! ## International Cooperation - ການຮ່ວມມືສາກົນ
//!
//! **Articles 120-135** establish international cooperation:
//!
//! - **Article 120**: UNCAC compliance (UN Convention Against Corruption)
//! - **Article 125**: Mutual legal assistance
//! - **Article 128**: Asset recovery from abroad
//! - **Article 132**: Extradition procedures
//!
//! # Features
//!
//! - **Bilingual Support**: All types and errors support both Lao (ລາວ) and English
//! - **Type-safe Validation**: Compile-time guarantees for compliance
//! - **Comprehensive Coverage**: All major aspects of Anti-Corruption Law 2012
//! - **Builder Patterns**: Easy construction of complex structures
//! - **Penalty Calculation**: Automatic penalty determination based on amounts
//!
//! # Examples
//!
//! ## Creating a Corruption Offense
//!
//! ```rust
//! use legalis_la::anti_corruption_law::*;
//!
//! let offense = CorruptionOffense {
//!     offense_type: CorruptionOffenseType::Bribery {
//!         direction: BriberyDirection::Receiving,
//!         amount_lak: 10_000_000,
//!     },
//!     perpetrator: OfficialType::GovernmentOfficial {
//!         position_grade: PositionGrade::Grade5,
//!         ministry: Some("Ministry of Finance".to_string()),
//!     },
//!     date_of_offense: "2025-06-15".to_string(),
//!     location_province: "Vientiane Capital".to_string(),
//!     description_lao: "ການຮັບສິນບົນໃນການອອກໃບອະນຸຍາດ".to_string(),
//!     description_en: "Accepting bribe for issuing permits".to_string(),
//!     evidence_collected: true,
//!     investigation_status: InvestigationStatus::UnderInvestigation,
//! };
//!
//! // Validate the offense
//! assert!(validate_corruption_offense(&offense).is_ok());
//!
//! // Determine penalty range
//! let penalty = determine_penalty_range(&offense);
//! assert_eq!(penalty.severity, CorruptionSeverity::Medium);
//! ```
//!
//! ## Creating an Asset Declaration
//!
//! ```rust
//! use legalis_la::anti_corruption_law::*;
//!
//! let declaration = AssetDeclarationBuilder::new()
//!     .official_id("GOV-2025-001")
//!     .official_name_lao("ທ່ານ ສົມໃຈ")
//!     .official_name_en("Mr. Somjai")
//!     .position_grade(PositionGrade::Grade3)
//!     .ministry("Ministry of Justice")
//!     .declaration_year(2025)
//!     .add_real_estate(RealEstate {
//!         property_type: PropertyType::House,
//!         location_lao: "ບ້ານ ໂພນໄຊ, ເມືອງ ໄຊເສດຖາ".to_string(),
//!         location_en: "Phonsai Village, Xaysetha District".to_string(),
//!         estimated_value_lak: 500_000_000,
//!         acquisition_date: Some("2020-01-15".to_string()),
//!         acquisition_method: AcquisitionMethod::Purchase,
//!     })
//!     .add_income_source(IncomeSource {
//!         source_type: IncomeSourceType::Salary,
//!         description_lao: "ເງິນເດືອນລັດຖະກອນ".to_string(),
//!         description_en: "Government salary".to_string(),
//!         annual_amount_lak: 48_000_000,
//!     })
//!     .total_assets_lak(550_000_000)
//!     .total_liabilities_lak(100_000_000)
//!     .submission_date("2025-03-31")
//!     .build();
//!
//! // Validate the declaration
//! assert!(validate_asset_declaration(&declaration).is_ok());
//! ```
//!
//! ## Filing a Whistleblower Report
//!
//! ```rust
//! use legalis_la::anti_corruption_law::*;
//!
//! let report = WhistleblowerReportBuilder::new()
//!     .anonymous(true)
//!     .allegation_type(CorruptionOffenseType::Embezzlement {
//!         amount_lak: 50_000_000,
//!         fund_source: FundSource::StateBudget,
//!     })
//!     .description_lao("ການສໍ້ໂກງງົບປະມານໂຄງການ".to_string())
//!     .description_en("Embezzlement of project budget".to_string())
//!     .accused_official_description("Provincial Finance Director".to_string())
//!     .evidence_description("Financial records and witness testimony".to_string())
//!     .submission_date("2025-07-01")
//!     .build();
//!
//! // Validate the report
//! assert!(validate_whistleblower_report(&report).is_ok());
//! ```
//!
//! # Bilingual Error Messages
//!
//! All errors include both English and Lao messages:
//!
//! ```rust
//! use legalis_la::anti_corruption_law::*;
//!
//! let error = AntiCorruptionLawError::InvalidAssetDeclaration {
//!     message_lao: "ການປະກາດຊັບສິນບໍ່ຄົບຖ້ວນ".to_string(),
//!     message_en: "Asset declaration is incomplete".to_string(),
//!     article: 55,
//! };
//!
//! println!("English: {}", error.english_message());
//! println!("Lao: {}", error.lao_message());
//! ```
//!
//! # Compliance Notes
//!
//! When implementing anti-corruption compliance in Laos:
//!
//! 1. **Asset Declaration**: Officials of Grade 5 and above must declare annually
//! 2. **Gift Limits**: Maximum gift value is 500,000 LAK for official functions
//! 3. **Cooling-off Period**: 2 years before former officials can join related businesses
//! 4. **Reporting**: Mandatory reporting of suspected corruption
//! 5. **Investigation**: SIA has 90 days for preliminary investigation
//! 6. **Prosecution**: Cases referred to prosecutors within 30 days of investigation completion
//!
//! # Related Laws
//!
//! - **Criminal Code 2017** - Criminal penalties for corruption offenses
//! - **Civil Service Law** - Disciplinary measures for public officials
//! - **Public Procurement Law** - Transparency in government procurement
//! - **State Budget Law** - Management of public funds

pub mod error;
pub mod types;
pub mod validator;

// Re-export commonly used types and functions
pub use error::{AntiCorruptionLawError, AntiCorruptionLawResult};

pub use types::{
    // Constants
    ANNUAL_DECLARATION_DEADLINE_MONTH,
    // Acquisition
    AcquisitionMethod,
    // Asset Declaration
    AssetDeclaration,
    AssetDeclarationBuilder,
    AssetDeclarationStatus,
    // Bribery
    BriberyDirection,
    COOLING_OFF_PERIOD_YEARS,
    // Code of Conduct
    CodeOfConductViolation,
    CodeOfConductViolationType,
    // Corruption Offense
    CorruptionOffense,
    CorruptionOffenseType,
    CorruptionSeverity,
    // Fund Source
    FundSource,
    GIFT_LIMIT_OFFICIAL_FUNCTION_LAK,
    // Gift
    Gift,
    GiftType,
    INVESTIGATION_FULL_DAYS,
    INVESTIGATION_PRELIMINARY_DAYS,
    // Income Source
    IncomeSource,
    IncomeSourceType,
    // International Cooperation
    InternationalCooperation,
    InternationalCooperationType,
    // Investigation
    Investigation,
    InvestigationStatus,
    InvestigationType,
    MEDIUM_CORRUPTION_THRESHOLD_LAK,
    MINOR_CORRUPTION_THRESHOLD_LAK,
    // Official Types
    OfficialCategory,
    OfficialType,
    PROSECUTION_REFERRAL_DAYS,
    // Penalty
    PenaltyRange,
    PenaltyType,
    // Position
    PositionGrade,
    // Prevention Measure
    PreventionMeasure,
    PreventionMeasureType,
    // Property
    PropertyType,
    // Prosecution
    ProsecutionReferral,
    ProsecutionStatus,
    // Real Estate
    RealEstate,
    SERIOUS_CORRUPTION_THRESHOLD_LAK,
    // SIA
    SIAOffice,
    SIAOfficeLevel,
    SIAPower,
    VERY_SERIOUS_CORRUPTION_THRESHOLD_LAK,
    // Vehicle
    Vehicle,
    VehicleType,
    // Verification
    VerificationResult,
    VerificationStatus,
    WHISTLEBLOWER_REWARD_MAX_PERCENT,
    WHISTLEBLOWER_REWARD_MIN_PERCENT,
    // Whistleblower
    WhistleblowerProtection,
    WhistleblowerProtectionType,
    WhistleblowerReport,
    WhistleblowerReportBuilder,
    WhistleblowerReportStatus,
};

pub use validator::{
    // Penalty Validators
    determine_penalty_range,
    // Asset Declaration Validators
    validate_asset_declaration,
    validate_asset_declaration_completeness,
    // Code of Conduct Validators
    validate_code_of_conduct_compliance,
    // Prevention Measure Validators
    validate_cooling_off_period,
    // Corruption Offense Validators
    validate_corruption_offense,
    validate_declaration_deadline,
    validate_declaration_required,
    // Gift Validators
    validate_gift,
    validate_gift_limit,
    // International Cooperation Validators
    validate_international_cooperation,
    // Investigation Validators
    validate_investigation,
    validate_investigation_timeline,
    // Official Validators
    validate_official_category,
    validate_penalty,
    validate_prevention_measure,
    // SIA Validators
    validate_sia_jurisdiction,
    validate_sia_powers,
    // Whistleblower Validators
    validate_whistleblower_protection,
    validate_whistleblower_report,
    validate_whistleblower_reward,
};
