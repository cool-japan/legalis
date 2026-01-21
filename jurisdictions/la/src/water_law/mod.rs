//! Water and Water Resources Law Module (ກົດໝາຍຊັບພະຍາກອນນໍ້າ)
//!
//! This module provides comprehensive support for Lao water and water resources law based on
//! **Water and Water Resources Law 2017** (Law No. 23/NA, dated June 30, 2017).
//!
//! # Legal Framework
//!
//! The Water and Water Resources Law 2017 is the primary legislation governing
//! water resources management in the Lao People's Democratic Republic. It establishes:
//!
//! - Water source classification and protection
//! - Water use rights and permit systems
//! - Water allocation framework
//! - Hydropower regulations and concessions
//! - Water quality standards
//! - Mekong River Commission compliance
//! - Irrigation district management
//! - Groundwater management
//! - Pollution prevention
//!
//! # Key Provisions
//!
//! ## Water Source Classifications (ການຈັດປະເພດແຫຼ່ງນໍ້າ) - Articles 15-25
//!
//! **Article 15-19**: Water source types
//! - Surface water (rivers, lakes, reservoirs) - ນໍ້າຜິວໜ້າ
//! - Groundwater (aquifers, wells) - ນໍ້າໃຕ້ດິນ
//! - Mekong River system (international treaty waters) - ລະບົບແມ່ນໍ້າຂອງ
//! - Wetlands and floodplains - ທີ່ດິນບຶງ ແລະ ທີ່ລຸ່ມ
//!
//! **Article 20**: Wetland protection requirements
//!
//! ## Water Use Rights (ສິດນຳໃຊ້ນໍ້າ) - Articles 35-44
//!
//! **Article 35-37**: Water use permit requirements
//! - Domestic use (priority right) - ການນຳໃຊ້ຄົວເຮືອນ
//! - Agricultural use (irrigation permits) - ການນຳໃຊ້ກະສິກຳ
//! - Industrial use (water extraction permits) - ການນຳໃຊ້ອຸດສາຫະກຳ
//! - Hydropower generation - ການຜະລິດໄຟຟ້ານໍ້າຕົກ
//! - Navigation rights - ການເດີນເຮືອ
//!
//! **Article 38-40**: Water allocation priority hierarchy
//! 1. Domestic use (highest priority)
//! 2. Agricultural use
//! 3. Industrial use
//! 4. Hydropower generation
//! 5. Navigation
//!
//! ## Hydropower Regulations (ລະບຽບການໄຟຟ້ານໍ້າຕົກ) - Articles 45-54
//!
//! **Article 45-47**: Hydropower concessions
//! - Small hydropower (< 15 MW) - ໄຟຟ້ານໍ້າຕົກຂະໜາດນ້ອຍ
//! - Medium hydropower (15-100 MW) - ໄຟຟ້ານໍ້າຕົກຂະໜາດກາງ
//! - Large hydropower (> 100 MW) - ໄຟຟ້ານໍ້າຕົກຂະໜາດໃຫຍ່
//! - Concession periods: 25-30 years
//!
//! **Article 48**: Minimum environmental flow requirements
//!
//! **Article 50**: Resettlement requirements for affected communities
//!
//! ## Water Quality Standards (ມາດຕະຖານຄຸນນະພາບນໍ້າ) - Articles 55-58
//!
//! **Article 55**: Drinking water standards
//! - Turbidity: max 5 NTU
//! - pH: 6.5-8.5
//! - Arsenic: max 0.01 mg/L
//! - Lead: max 0.01 mg/L
//! - E. coli: 0 CFU/100mL
//!
//! **Article 56**: Agricultural water standards
//!
//! **Article 57**: Industrial discharge limits
//! - BOD: max 20 mg/L
//! - COD: max 120 mg/L
//! - TSS: max 50 mg/L
//!
//! **Article 58**: Wastewater treatment requirements
//!
//! ## Mekong River Commission Compliance (ການປະຕິບັດຕາມ MRC) - Articles 60-63
//!
//! **Article 60**: Prior consultation requirements (PNPCA)
//! - Required for mainstream Mekong projects
//! - 6-month consultation period
//!
//! **Article 61**: Notification procedures for tributary projects
//!
//! **Article 62**: Transboundary impact assessment requirements
//!
//! **Article 63**: Data sharing obligations with MRC
//!
//! ## Irrigation Districts (ເຂດຊົນລະປະທານ) - Articles 70-73
//!
//! **Article 70**: Water User Associations (WUAs) - ສະມາຄົມຜູ້ນຳໃຊ້ນໍ້າ
//!
//! **Article 72**: Irrigation service fees - 150,000 LAK/hectare/season
//!
//! **Article 73**: Water delivery schedules
//!
//! ## Groundwater Management (ການຄຸ້ມຄອງນໍ້າໃຕ້ດິນ) - Articles 75-78
//!
//! **Article 75**: Well drilling permits (required for depth > 20m)
//!
//! **Article 76**: Extraction limits based on sustainable yield
//!
//! **Article 77**: Aquifer protection zones
//!
//! **Article 78**: Monitoring requirements (every 90 days)
//!
//! ## Pollution Prevention (ການປ້ອງກັນມົນລະພິດ) - Articles 80-82
//!
//! **Article 80**: Polluter pays principle - ຫຼັກການຜູ້ກໍ່ມົນລະພິດຕ້ອງຮັບຜິດຊອບ
//!
//! **Article 82**: Agricultural runoff controls
//!
//! # Features
//!
//! - **Bilingual Support**: All types and errors support both Lao (ລາວ) and English
//! - **Type-safe Validation**: Compile-time guarantees for water law compliance
//! - **Comprehensive Coverage**: All major aspects of Water Law 2017
//! - **Builder Patterns**: Easy construction of complex concession and permit structures
//! - **MRC Integration**: Full support for Mekong River Commission procedures
//! - **Standards Compliance**: National water quality standards built-in
//!
//! # Examples
//!
//! ## Validating Water Use Permit
//!
//! ```rust
//! use legalis_la::water_law::*;
//!
//! // Industrial use requires permit
//! let result = validate_water_use_permit(WaterUseType::Industrial, false);
//! assert!(result.is_err());
//!
//! // Domestic use doesn't require permit
//! let result = validate_water_use_permit(WaterUseType::Domestic, false);
//! assert!(result.is_ok());
//! ```
//!
//! ## Building a Hydropower Concession
//!
//! ```rust
//! use legalis_la::water_law::*;
//!
//! let concession = HydropowerConcessionBuilder::new()
//!     .concession_number("HP-2026-001")
//!     .project_name("Nam Ngum 5")
//!     .project_name_lao("ນໍ້າງື່ມ 5")
//!     .developer_name("Lao Energy Company")
//!     .installed_capacity_mw(120.0) // Large hydropower
//!     .river_name("Nam Ngum")
//!     .province("Vientiane")
//!     .on_mekong_mainstream(false)
//!     .concession_dates("2025-01-01", "2055-01-01", 30)
//!     .status(ConcessionStatus::Construction)
//!     .minimum_environmental_flow_m3s(10.0)
//!     .mrc_consultation_completed(true)
//!     .build();
//!
//! assert_eq!(concession.category, HydropowerCategory::Large);
//! ```
//!
//! ## Validating Drinking Water Quality
//!
//! ```rust
//! use legalis_la::water_law::*;
//!
//! // Check arsenic level against drinking water standards
//! let result = validate_drinking_water_quality(
//!     WaterQualityParameter::Arsenic,
//!     0.005
//! );
//! assert!(result.is_ok());
//!
//! // Arsenic exceeding limit
//! let result = validate_drinking_water_quality(
//!     WaterQualityParameter::Arsenic,
//!     0.02
//! );
//! assert!(result.is_err());
//! ```
//!
//! ## Validating MRC Compliance
//!
//! ```rust
//! use legalis_la::water_law::*;
//!
//! let mekong_mainstream = WaterSourceType::MekongRiverSystem {
//!     location: MekongLocation::Mainstream,
//!     section_name: Some("Luang Prabang".to_string()),
//!     distance_from_border_km: Some(100.0),
//! };
//!
//! // Mainstream projects require prior consultation
//! let result = validate_mrc_prior_consultation(
//!     &mekong_mainstream,
//!     "hydropower dam",
//!     false
//! );
//! assert!(result.is_err());
//! ```
//!
//! ## Validating Drought Protocol
//!
//! ```rust
//! use legalis_la::water_law::*;
//!
//! // During emergency drought, agricultural use is reduced by 50%
//! let result = validate_drought_protocol(
//!     DroughtLevel::Emergency,
//!     WaterUseType::Agricultural,
//!     400.0,  // Requested
//!     1000.0  // Baseline allocation
//! );
//! assert!(result.is_ok()); // 400 <= 500 (1000 * 0.5)
//! ```
//!
//! # Bilingual Error Messages
//!
//! All errors include both English and Lao messages:
//!
//! ```rust
//! use legalis_la::water_law::*;
//!
//! let error = WaterLawError::DrinkingWaterViolation {
//!     parameter: "Arsenic".to_string(),
//!     actual: 0.05,
//!     limit: 0.01,
//!     unit: "mg/L".to_string(),
//! };
//!
//! println!("English: {}", error.english_message());
//! // "Drinking water quality violation: Arsenic at 0.05 mg/L exceeds limit 0.01 mg/L (Article 55)"
//!
//! println!("Lao: {}", error.lao_message());
//! // "ການລະເມີດຄຸນນະພາບນໍ້າດື່ມ: Arsenic ທີ່ 0.05 mg/L ເກີນມາດຕະຖານ 0.01 mg/L (ມາດຕາ 55)"
//! ```
//!
//! # Compliance Notes
//!
//! When implementing water law compliance in Laos:
//!
//! 1. **Water Use Permits**: All non-domestic water use requires permits (Article 35)
//! 2. **Priority Hierarchy**: Domestic use has absolute priority during shortages (Article 38)
//! 3. **Hydropower**: Large projects (>100 MW) require MRC prior consultation (Article 60)
//! 4. **Environmental Flow**: Minimum environmental flows must be maintained (Article 48)
//! 5. **Water Quality**: Drinking water must meet national standards (Article 55)
//! 6. **Groundwater**: Deep wells (>20m) require permits (Article 75)
//! 7. **Irrigation Fees**: Standard fee is 150,000 LAK/hectare/season (Article 72)
//! 8. **Polluter Pays**: Polluters must pay for remediation (Article 80)
//!
//! # Related Laws
//!
//! - **Environmental Protection Law 2012** - Environmental impact assessment
//! - **Land Law 2019** - Land use in water resource areas
//! - **Mining Law 2017** - Water use in mining operations
//! - **Electricity Law 2011** - Hydropower generation regulations
//! - **Investment Promotion Law 2016** - Hydropower investment incentives

pub mod error;
pub mod types;
pub mod validator;

// Re-export commonly used types and functions
pub use error::{Result, WaterLawError};
pub use types::{
    // Water Source Types
    AquiferProtectionZone,
    AquiferType,
    // Hydropower
    ConcessionStatus,
    // Constants
    DRINKING_WATER_MAX_ARSENIC_MG_L,
    DRINKING_WATER_MAX_ECOLI,
    DRINKING_WATER_MAX_LEAD_MG_L,
    DRINKING_WATER_MAX_PH,
    DRINKING_WATER_MAX_TURBIDITY_NTU,
    DRINKING_WATER_MIN_PH,
    // Water Allocation
    DroughtLevel,
    DroughtRestrictions,
    EcologicalSignificance,
    // Pollution Prevention
    FacilityStatus,
    // Irrigation Districts
    FeePaymentStatus,
    GROUNDWATER_MONITORING_INTERVAL_DAYS,
    // Groundwater
    GroundwaterMonitoringRecord,
    HYDROPOWER_CONCESSION_MAX_YEARS,
    HYDROPOWER_CONCESSION_MIN_YEARS,
    HydropowerCategory,
    HydropowerConcession,
    HydropowerConcessionBuilder,
    INDUSTRIAL_DISCHARGE_MAX_BOD_MG_L,
    INDUSTRIAL_DISCHARGE_MAX_COD_MG_L,
    INDUSTRIAL_DISCHARGE_MAX_TSS_MG_L,
    IRRIGATION_FEE_PER_HECTARE_LAK,
    IrrigationServiceFee,
    MEDIUM_HYDROPOWER_THRESHOLD_MW,
    MRC_PRIOR_CONSULTATION_MONTHS,
    // MRC Compliance
    MRCComplianceRecord,
    MRCComplianceStatus,
    MRCProcedureType,
    MekongLocation,
    // Water Use Rights
    PermitCondition,
    PolluterRecord,
    PollutionType,
    PowerPurchaseAgreement,
    ProtectionLevel,
    RemediationStatus,
    ResettlementApprovalStatus,
    ResettlementPlan,
    SMALL_HYDROPOWER_THRESHOLD_MW,
    Season,
    SurfaceWaterBodyType,
    TreatmentType,
    WATER_PERMIT_VALIDITY_YEARS,
    WELL_PERMIT_DEPTH_THRESHOLD_M,
    WUAStatus,
    WastewaterTreatmentFacility,
    WaterAllocation,
    WaterPermitStatus,
    // Water Quality
    WaterQualityClass,
    WaterQualityMeasurement,
    WaterQualityParameter,
    WaterSourceType,
    WaterUseRight,
    WaterUseRightBuilder,
    WaterUseType,
    WaterUserAssociation,
    WellPermit,
    WetlandType,
};
pub use validator::{
    // Water Use Rights Validators
    calculate_irrigation_fee,
    validate_agricultural_runoff,
    validate_aquifer_zone_activity,
    validate_drinking_water_quality,
    validate_drought_protocol,
    validate_extraction_limit,
    validate_groundwater_extraction,
    validate_groundwater_monitoring,
    validate_hydropower_category,
    validate_hydropower_concession,
    validate_industrial_discharge,
    validate_irrigation_fee,
    validate_minimum_environmental_flow,
    validate_mrc_data_sharing,
    validate_mrc_notification,
    validate_mrc_prior_consultation,
    validate_polluter_pays,
    validate_seasonal_allocation,
    validate_transboundary_assessment,
    validate_wastewater_treatment,
    // Comprehensive Validator
    validate_water_law_compliance,
    validate_water_permit,
    validate_water_use_permit,
    validate_water_use_priority,
    validate_well_drilling_permit,
    validate_well_permit,
    validate_wetland_protection,
    validate_wua_registration,
};
