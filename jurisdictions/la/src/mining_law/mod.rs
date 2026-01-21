//! Mining Law Module (ກົດໝາຍບໍ່ແຮ່)
//!
//! This module provides comprehensive support for Lao mining law based on
//! **Mining Law 2017** (Law No. 31/NA, dated June 28, 2017).
//!
//! # Legal Framework
//!
//! The Mining Law 2017 is the primary legislation governing mineral exploration,
//! extraction, processing, and trade in the Lao People's Democratic Republic.
//! It establishes:
//!
//! - Mineral classification system
//! - Mining license and concession framework
//! - Royalty rates and payment obligations
//! - Environmental protection requirements
//! - Foreign investment regulations
//! - Community rights and benefits
//!
//! # Key Provisions
//!
//! ## Mineral Classifications (ການຈັດປະເພດແຮ່) - Articles 11-13
//!
//! **Article 11**: Minerals are classified into four categories:
//!
//! - **Strategic Minerals (ແຮ່ຍຸດທະສາດ)**: Gold, copper, potash, bauxite, iron ore, tin,
//!   lead, zinc. Require Government approval and joint venture with Lao entities.
//!
//! - **Common Minerals (ແຮ່ທົ່ວໄປ)**: Stone, sand, gravel, clay, limestone, gypsum.
//!   Simpler licensing requirements.
//!
//! - **Gemstones (ແກ້ວປະເສີດ)**: Sapphires, rubies, and other precious stones.
//!   Subject to 10% royalty rate.
//!
//! - **Rare Earth Elements (ທາດຫາຍາກ)**: Special environmental assessment required.
//!   Higher royalty rate of 8%.
//!
//! **Article 12**: Strategic minerals require Government approval before exploration.
//!
//! **Article 13**: Rare earth extraction requires special environmental assessment.
//!
//! ## Mining License Types (ໃບອະນຸຍາດບໍ່ແຮ່) - Articles 24-27
//!
//! **Article 25**: Exploration License (ໃບອະນຸຍາດສຳຫຼວດ)
//! - Maximum 2 years duration
//! - Renewable once
//! - For geological survey and mineral exploration
//!
//! **Article 26**: Mining License (ໃບອະນຸຍາດຂຸດຄົ້ນ)
//! - 20-30 years for strategic minerals
//! - Requires EIA approval
//! - Renewable subject to compliance
//!
//! **Article 27**: Processing License (ໃບອະນຸຍາດປຸງແຕ່ງ)
//! - Up to 20 years duration
//! - For mineral processing and refining operations
//!
//! ## Concession Framework (ກອບສຳປະທານ) - Articles 30-35
//!
//! **Article 30**: Exploration Concession
//! - Maximum 2 years, renewable once
//! - Strategic minerals: up to 10,000 hectares
//! - Common minerals: up to 5,000 hectares
//!
//! **Article 31**: Mining Concession
//! - Strategic minerals: 20-30 years
//! - Small-scale mining: maximum 10 years
//!
//! **Article 33**: Overlapping concessions are prohibited.
//!
//! **Article 35**: Concession status must allow mining operations.
//!
//! ## Royalty Rates (ອັດຕາຄ່າພາກຫຼວງ) - Article 45
//!
//! - **Gold**: 5%
//! - **Copper**: 3%
//! - **Potash**: 2%
//! - **Common minerals**: 1-3%
//! - **Gemstones**: 10%
//! - **Rare earth elements**: 8%
//! - **Bauxite**: 4%
//!
//! ## Environmental Requirements (ຂໍ້ກຳນົດສິ່ງແວດລ້ອມ) - Articles 50-54
//!
//! **Article 50**: Mandatory EIA for all mining activities.
//!
//! **Article 51**: Minimum 1km buffer from protected areas.
//!
//! **Article 52**: Rehabilitation bond requirement (minimum 5% of project cost).
//!
//! **Article 53**: Closure plan required for mining concessions.
//!
//! **Article 54**: Environmental monitoring and reporting obligations.
//!
//! ## Foreign Investment Rules (ກົດລະບຽບການລົງທຶນຕ່າງປະເທດ) - Articles 18-21
//!
//! **Article 18**: Joint venture required for strategic minerals.
//!
//! **Article 19**: Foreign ownership limits:
//! - Strategic minerals: maximum 75%
//! - Common minerals: up to 100%
//!
//! **Article 20**: Local content requirement: minimum 30%.
//!
//! **Article 21**: Technology transfer obligations.
//!
//! ## Community Rights (ສິດຂອງຊຸມຊົນ) - Articles 37-40
//!
//! **Article 37**: Prior consultation with affected communities required.
//!
//! **Article 38**: Fair compensation for affected communities.
//!
//! **Article 39**: Local employment quota: minimum 70%.
//!
//! **Article 40**: Revenue sharing with local communities: minimum 1%.
//!
//! # Features
//!
//! - **Bilingual Support**: All types and errors support both Lao (ລາວ) and English
//! - **Type-safe Validation**: Compile-time guarantees for mining compliance
//! - **Comprehensive Coverage**: All major aspects of Mining Law 2017
//! - **Builder Patterns**: Easy construction of complex license and concession structures
//! - **Royalty Calculation**: Built-in royalty rate and amount calculation
//!
//! # Examples
//!
//! ## Validating Mining License
//!
//! ```rust
//! use legalis_la::mining_law::*;
//!
//! let license = MiningLicenseBuilder::new()
//!     .license_number("ML-2026-001")
//!     .license_type(MiningLicenseType::Mining)
//!     .holder_name("Mining Company Ltd")
//!     .issue_date("2024-01-01")
//!     .expiry_date("2029-01-01")
//!     .status(LicenseStatus::Active)
//!     .province("Savannakhet")
//!     .issuing_authority("MEM")
//!     .add_mineral_type(MineralType::Gold)
//!     .build();
//!
//! let result = validate_mining_license(&license, "2026-01-15");
//! assert!(result.is_ok());
//! ```
//!
//! ## Building a Mining Concession
//!
//! ```rust
//! use legalis_la::mining_law::*;
//!
//! let concession = MiningConcessionBuilder::new()
//!     .concession_id("MC-2026-001")
//!     .concession_type(ConcessionType::Mining)
//!     .holder_name("Mining Company Ltd")
//!     .holder_name_lao("ບໍລິສັດບໍ່ແຮ່ ຈຳກັດ")
//!     .primary_mineral(MineralType::Gold)
//!     .area_hectares(500.0)
//!     .province("Savannakhet")
//!     .dates("2024-01-01", "2044-01-01", 20)
//!     .status(ConcessionStatus::InProduction)
//!     .eia_approved(true, Some("EIA-2024-001".to_string()))
//!     .rehabilitation_bond(50_000_000_000)
//!     .closure_plan_submitted(true)
//!     .foreign_ownership_percent(70.0)
//!     .local_content_percent(35.0)
//!     .distance_from_protected_area(1500)
//!     .build();
//!
//! let result = validate_mining_concession(&concession);
//! assert!(result.is_ok());
//! ```
//!
//! ## Calculating Royalty Amount
//!
//! ```rust
//! use legalis_la::mining_law::*;
//!
//! // Gold royalty at 5%
//! let market_value: u64 = 1_000_000_000; // 1 billion LAK
//! let royalty = calculate_royalty_amount(&MineralType::Gold, market_value);
//! assert_eq!(royalty, 50_000_000); // 50 million LAK (5%)
//!
//! // Copper royalty at 3%
//! let royalty = calculate_royalty_amount(&MineralType::Copper, market_value);
//! assert_eq!(royalty, 30_000_000); // 30 million LAK (3%)
//! ```
//!
//! ## Validating Foreign Investment
//!
//! ```rust
//! use legalis_la::mining_law::*;
//!
//! // Check foreign ownership for strategic minerals
//! let result = validate_foreign_ownership(MineralClassification::Strategic, 70.0);
//! assert!(result.is_ok()); // 70% is within 75% limit
//!
//! // Exceeds limit
//! let result = validate_foreign_ownership(MineralClassification::Strategic, 80.0);
//! assert!(result.is_err()); // 80% exceeds 75% limit
//! ```
//!
//! ## Validating Environmental Compliance
//!
//! ```rust
//! use legalis_la::mining_law::*;
//!
//! // Check distance from protected area
//! let result = validate_protected_area_distance(1500); // 1.5km
//! assert!(result.is_ok()); // Meets 1km minimum
//!
//! let result = validate_protected_area_distance(500); // 0.5km
//! assert!(result.is_err()); // Below 1km minimum
//! ```
//!
//! ## Validating Community Rights
//!
//! ```rust
//! use legalis_la::mining_law::*;
//!
//! // Check local employment quota
//! let employment = LocalEmployment {
//!     record_id: "LE-001".to_string(),
//!     concession_id: "MC-001".to_string(),
//!     reporting_period: "2026-Q1".to_string(),
//!     total_employees: 100,
//!     local_employees: 75,
//!     local_percentage: 75.0,
//!     lao_national_employees: 80,
//!     foreign_employees: 20,
//!     skilled_positions_local: 10,
//!     skilled_positions_total: 15,
//! };
//!
//! let result = validate_local_employment(&employment);
//! assert!(result.is_ok()); // 75% exceeds 70% minimum
//! ```
//!
//! # Bilingual Error Messages
//!
//! All errors include both English and Lao messages:
//!
//! ```rust
//! use legalis_la::mining_law::*;
//!
//! let error = MiningLawError::RoyaltyRateMismatch {
//!     mineral: "Gold".to_string(),
//!     actual_rate: 3.0,
//!     required_rate: 5.0,
//! };
//!
//! println!("English: {}", error.english_message());
//! // "Royalty rate 3% does not match required rate 5% for Gold (Article 45)"
//!
//! println!("Lao: {}", error.lao_message());
//! // "ອັດຕາຄ່າພາກຫຼວງ 3% ບໍ່ກົງກັບອັດຕາທີ່ກຳນົດ 5% ສຳລັບ Gold (ມາດຕາ 45)"
//! ```
//!
//! # Compliance Notes
//!
//! When implementing mining compliance in Laos:
//!
//! 1. **License Requirements**: All mining activities require appropriate licenses
//! 2. **EIA Mandatory**: Environmental Impact Assessment required for mining and processing
//! 3. **Royalty Payments**: Must use correct rates based on mineral type
//! 4. **Protected Areas**: Minimum 1km buffer from protected area boundaries
//! 5. **Foreign Investment**: Strategic minerals require joint venture with Lao entity
//! 6. **Local Content**: Minimum 30% local content requirement
//! 7. **Local Employment**: Minimum 70% local employment quota
//! 8. **Community Consultation**: Prior consultation required before operations begin
//! 9. **Rehabilitation Bond**: Minimum 5% of project cost
//! 10. **Closure Plan**: Required for all mining concessions
//!
//! # Related Laws
//!
//! - **Environmental Protection Law 2012** - Environmental requirements for mining
//! - **Investment Promotion Law 2016** - Foreign investment framework
//! - **Land Law 2019** - Land use rights for mining concessions
//! - **Labor Law 2013** - Employment regulations for mining operations
//! - **Tax Law 2019** - Tax obligations for mining companies

pub mod error;
pub mod types;
pub mod validator;

// Re-export commonly used types and functions
pub use error::{MiningLawError, Result};
pub use types::{
    ARTISANAL_MINING_MAX_HECTARES,
    COMMUNITY_REVENUE_SHARE_MIN_PERCENT,
    // Community Types
    CommunityCompensation,
    CommunityConsultation,
    CompensationType,
    // Concession Types
    ConcessionStatus,
    ConcessionType,
    // Constants - Area Limits
    EXPLORATION_AREA_COMMON_MAX_HECTARES,
    EXPLORATION_AREA_STRATEGIC_MAX_HECTARES,
    // Constants - Concession Limits
    EXPLORATION_LICENSE_MAX_RENEWALS,
    EXPLORATION_LICENSE_MAX_YEARS,
    // Environmental Types
    EnvironmentalViolation,
    // Constants - Foreign Investment Limits
    FOREIGN_OWNERSHIP_COMMON_MAX_PERCENT,
    FOREIGN_OWNERSHIP_STRATEGIC_MAX_PERCENT,
    // Foreign Investment Types
    ForeignInvestment,
    // Constants - Community Requirements
    LOCAL_CONTENT_MIN_PERCENT,
    LOCAL_EMPLOYMENT_MIN_PERCENT,
    // License Types
    LicenseCondition,
    LicenseStatus,
    LocalEmployment,
    // Constants - Environmental Requirements
    MIN_DISTANCE_FROM_PROTECTED_AREA_METERS,
    MINING_CONCESSION_STRATEGIC_MAX_YEARS,
    MINING_CONCESSION_STRATEGIC_MIN_YEARS,
    // Mineral Types
    MineralClassification,
    MineralType,
    MiningConcession,
    MiningConcessionBuilder,
    MiningEnvironmentalCompliance,
    MiningLicense,
    MiningLicenseBuilder,
    MiningLicenseType,
    PROCESSING_LICENSE_MAX_YEARS,
    // Royalty Types
    PaymentStatus,
    REHABILITATION_BOND_MIN_PERCENT,
    // Constants - Royalty Rates
    ROYALTY_RATE_BAUXITE,
    ROYALTY_RATE_COMMON_MAX,
    ROYALTY_RATE_COMMON_MIN,
    ROYALTY_RATE_COPPER,
    ROYALTY_RATE_GEMSTONES,
    ROYALTY_RATE_GOLD,
    ROYALTY_RATE_POTASH,
    ROYALTY_RATE_RARE_EARTH,
    RoyaltyPayment,
    SMALL_SCALE_MINING_MAX_HECTARES,
    SMALL_SCALE_MINING_MAX_YEARS,
    TechnologyTransfer,
    ViolationSeverity,
};
pub use validator::{
    // Royalty Validators
    calculate_royalty_amount,
    // Community Validators
    validate_community_compensation,
    // Concession Validators
    validate_concession_area,
    validate_concession_duration,
    // Environmental Compliance Validators
    validate_environmental_compliance,
    // Foreign Investment Validators
    validate_foreign_investment,
    validate_foreign_ownership,
    validate_license_for_activity,
    // Local Content Validators
    validate_local_content,
    // Community Rights Validators
    validate_local_employment,
    // Mineral Validators
    validate_mineral_classification,
    validate_mineral_export,
    // Comprehensive Validators
    validate_mining_compliance,
    validate_mining_concession,
    // License Validators
    validate_mining_license,
    validate_prior_consultation,
    // Environmental Validators
    validate_protected_area_distance,
    validate_rehabilitation_bond,
    validate_revenue_sharing,
    validate_royalty_payment,
    validate_royalty_rate,
    // Small-Scale Validators
    validate_small_scale_mining,
};
