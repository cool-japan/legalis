//! Environmental Law Module (ກົດໝາຍສິ່ງແວດລ້ອມ)
//!
//! This module provides comprehensive support for Lao environmental law based on
//! **Environmental Protection Law 2012** (Law No. 29/NA, dated December 18, 2012).
//!
//! # Legal Framework
//!
//! The Environmental Protection Law 2012 is the primary legislation governing
//! environmental protection in the Lao People's Democratic Republic. It establishes:
//!
//! - Environmental Impact Assessment (EIA) requirements
//! - Pollution control standards
//! - Protected area management
//! - Environmental permit systems
//! - Enforcement and penalties
//!
//! # Key Provisions
//!
//! ## Environmental Impact Assessment (EIA) - ການປະເມີນຜົນກະທົບສິ່ງແວດລ້ອມ
//!
//! **Articles 18-24** establish the EIA framework:
//!
//! - **Article 18**: Project categories requiring EIA
//!   - Category A: Large-scale projects requiring full EIA
//!   - Category B: Medium-scale projects requiring IEE (Initial Environmental Examination)
//!
//! - **Article 19**: EIA submission requirements
//!   - Project description
//!   - Baseline environmental conditions
//!   - Impact assessment
//!   - Mitigation measures
//!   - Environmental Management Plan (EMP)
//!
//! - **Article 20**: EIA review process
//!   - Technical review by MONRE
//!   - Public consultation (for Category A)
//!   - Approval decision
//!
//! - **Article 21**: Public participation requirements
//!   - Mandatory for Category A projects
//!   - Community consultation
//!   - Information disclosure
//!
//! - **Article 22**: EIA certificate validity
//!   - Category A: 2 years
//!   - Category B: 3 years
//!
//! ## Pollution Control - ການຄວບຄຸມມົນລະພິດ
//!
//! **Articles 28-35** establish pollution control standards:
//!
//! - **Article 30**: Air quality standards
//!   - PM2.5: 25 μg/m³ (annual average)
//!   - PM10: 50 μg/m³ (annual average)
//!
//! - **Article 31**: Water discharge standards
//!   - BOD: 20 mg/L
//!   - COD: 120 mg/L
//!   - TSS: 50 mg/L
//!   - pH: 5.5-9.0
//!   - Temperature: max 40°C
//!
//! - **Article 32**: Noise standards
//!   - Residential (day): 55 dB
//!   - Residential (night): 45 dB
//!   - Industrial: 75 dB
//!
//! - **Article 33**: Monitoring requirements
//!   - Regular emissions monitoring
//!   - Periodic reporting
//!   - Inspection compliance
//!
//! - **Article 34-35**: Waste management
//!   - Proper disposal methods
//!   - Hazardous waste handling
//!   - Transport permits
//!
//! ## Protected Areas - ເຂດປ່າປ້ອງກັນ
//!
//! **Articles 38-45** govern protected area management:
//!
//! - **Article 38**: Protected area categories
//!   - National Protected Areas
//!   - National Parks
//!   - Wildlife Sanctuaries
//!   - Protection Forests
//!   - Conservation Forests
//!   - Wetland Reserves
//!
//! - **Article 40**: Prohibited activities
//!   - Mining in strict protection zones
//!   - Logging without permits
//!   - Infrastructure without EIA
//!
//! - **Article 41**: Buffer zone requirements
//!   - Minimum 500m buffer zone
//!   - Mining must be 1000m from boundaries
//!
//! ## Environmental Permits - ໃບອະນຸຍາດສິ່ງແວດລ້ອມ
//!
//! **Articles 25-27** establish permit requirements:
//!
//! - **Article 25**: Permit types
//!   - EIA Certificate
//!   - Emission Permit
//!   - Waste Disposal Permit
//!   - Water Extraction Permit
//!   - Forestry Permit
//!
//! - **Article 26**: Permit conditions
//!   - Compliance requirements
//!   - Monitoring obligations
//!   - Reporting duties
//!
//! - **Article 27**: Permit validity
//!   - Standard validity: 5 years
//!   - Renewal procedures
//!   - Revocation conditions
//!
//! # Features
//!
//! - **Bilingual Support**: All types and errors support both Lao (ລາວ) and English
//! - **Type-safe Validation**: Compile-time guarantees for environmental compliance
//! - **Comprehensive Coverage**: All major aspects of Environmental Protection Law 2012
//! - **Builder Patterns**: Easy construction of complex EIA and permit structures
//! - **Standards Compliance**: National environmental quality standards built-in
//!
//! # Examples
//!
//! ## Validating EIA Requirement
//!
//! ```rust
//! use legalis_la::environmental_law::*;
//!
//! // Mining project over 100 hectares requires Category A EIA
//! let project = ProjectType::Mining {
//!     area_hectares: 150.0,
//!     mineral_type: Some("Gold".to_string()),
//! };
//!
//! let result = validate_eia_requirement(&project);
//! assert!(result.is_ok());
//! assert_eq!(result.unwrap(), Some(EIACategory::CategoryA));
//! ```
//!
//! ## Building an EIA
//!
//! ```rust
//! use legalis_la::environmental_law::*;
//!
//! let eia = EnvironmentalImpactAssessmentBuilder::new()
//!     .project_name_lao("ໂຄງການບໍ່ຄຳ")
//!     .project_name_en("Gold Mining Project")
//!     .project_type(ProjectType::Mining {
//!         area_hectares: 150.0,
//!         mineral_type: Some("Gold".to_string()),
//!     })
//!     .project_developer("Mining Company Ltd")
//!     .location_province("Savannakhet")
//!     .assessment_date("2026-01-15")
//!     .eia_category(EIACategory::CategoryA)
//!     .add_impact(EnvironmentalImpact::AirPollution {
//!         severity: ImpactSeverity::Moderate,
//!         pollutants: vec![AirPollutant::PM10],
//!     })
//!     .add_mitigation(MitigationMeasure {
//!         description_lao: "ຕິດຕັ້ງລະບົບກັ່ນຕອງຝຸ່ນ".to_string(),
//!         description_en: "Install dust filtration system".to_string(),
//!         target_impact: "Air pollution".to_string(),
//!         implementation_phase: ImplementationPhase::PreConstruction,
//!         estimated_cost_lak: Some(500_000_000),
//!         responsible_party: Some("Project Developer".to_string()),
//!         monitoring_indicator: Some("PM10 < 50 μg/m³".to_string()),
//!     })
//!     .public_consultation(true, 10)
//!     .approval_status(EIAApprovalStatus::Approved)
//!     .build();
//! ```
//!
//! ## Validating Air Quality
//!
//! ```rust
//! use legalis_la::environmental_law::*;
//!
//! // Check PM2.5 against national standard
//! match validate_air_quality(AirPollutant::PM25, 20.0, "μg/m³") {
//!     Ok(()) => println!("Air quality compliant"),
//!     Err(e) => {
//!         println!("English: {}", e.english_message());
//!         println!("Lao: {}", e.lao_message());
//!     }
//! }
//! ```
//!
//! ## Validating Protected Area Activity
//!
//! ```rust
//! use legalis_la::environmental_law::*;
//!
//! let protected_area = ProtectedArea {
//!     name_lao: "ເຂດປ່າປ້ອງກັນນ້ຳງື່ມ".to_string(),
//!     name_en: "Nam Ngum National Protected Area".to_string(),
//!     area_type: ProtectedAreaType::NationalProtectedArea,
//!     area_hectares: 50000.0,
//!     province: "Vientiane".to_string(),
//!     districts: vec!["Sangthong".to_string()],
//!     establishment_date: "1993-06-29".to_string(),
//!     iucn_category: Some(IUCNCategory::II),
//!     management_authority: Some("MONRE".to_string()),
//!     key_species: vec!["Asian Elephant".to_string(), "Tiger".to_string()],
//!     buffer_zone_hectares: Some(5000.0),
//! };
//!
//! // Research is allowed
//! assert!(validate_protected_area_activity(&protected_area, ProtectedAreaActivity::Research).is_ok());
//!
//! // Mining is prohibited
//! assert!(validate_protected_area_activity(&protected_area, ProtectedAreaActivity::Mining).is_err());
//! ```
//!
//! ## Validating Environmental Permit
//!
//! ```rust
//! use legalis_la::environmental_law::*;
//!
//! let permit = EnvironmentalPermit {
//!     permit_number: "EP-2026-001".to_string(),
//!     holder_name: "Mining Company Ltd".to_string(),
//!     holder_name_lao: Some("ບໍລິສັດບໍ່ແຮ່ ຈຳກັດ".to_string()),
//!     permit_type: EnvironmentalPermitType::MiningEnvironmentalPermit,
//!     issue_date: "2024-01-01".to_string(),
//!     expiry_date: "2029-01-01".to_string(),
//!     issuing_authority: "MONRE".to_string(),
//!     conditions: vec![
//!         PermitCondition {
//!             description: "Monthly emissions monitoring".to_string(),
//!             description_lao: Some("ການຕິດຕາມການປ່ອຍມົນລະພິດລາຍເດືອນ".to_string()),
//!             compliance_deadline: None,
//!             compliant: true,
//!         }
//!     ],
//!     status: PermitStatus::Active,
//!     project_name: Some("Gold Mining Project".to_string()),
//!     location_province: Some("Savannakhet".to_string()),
//! };
//!
//! // Validate permit is valid as of 2026-06-15
//! match validate_environmental_permit(&permit, "2026-06-15") {
//!     Ok(()) => println!("Permit is valid"),
//!     Err(e) => println!("Permit error: {}", e),
//! }
//! ```
//!
//! # Bilingual Error Messages
//!
//! All errors include both English and Lao messages:
//!
//! ```rust
//! use legalis_la::environmental_law::*;
//!
//! let error = EnvironmentalLawError::AirQualityExceedsLimit {
//!     pollutant: "PM2.5".to_string(),
//!     actual: 35.0,
//!     limit: 25.0,
//!     unit: "μg/m³".to_string(),
//! };
//!
//! println!("English: {}", error.english_message());
//! // "Air quality violation: PM2.5 level 35 μg/m³ exceeds limit of 25 μg/m³ (Article 30)"
//!
//! println!("Lao: {}", error.lao_message());
//! // "ການລະເມີດຄຸນນະພາບອາກາດ: PM2.5 ລະດັບ 35 μg/m³ ເກີນມາດຕະຖານ 25 μg/m³ (ມາດຕາ 30)"
//! ```
//!
//! # Compliance Notes
//!
//! When implementing environmental compliance in Laos:
//!
//! 1. **EIA Requirements**: All projects in specified categories require EIA before commencement
//! 2. **Public Consultation**: Mandatory for Category A projects affecting communities
//! 3. **Monitoring**: Regular emissions monitoring required for industrial facilities
//! 4. **Permits**: Environmental permits must be maintained and renewed
//! 5. **Protected Areas**: Strict restrictions on activities in national protected areas
//! 6. **Hazardous Waste**: Special permits required for transport and disposal
//! 7. **Reporting**: Regular compliance reports to MONRE
//!
//! # Related Laws
//!
//! - **Forestry Law 2019** - Forest management and logging permits
//! - **Water and Water Resources Law 2017** - Water use and protection
//! - **Mining Law 2017** - Mining environmental requirements
//! - **Land Law 2019** - Land use in protected areas

pub mod error;
pub mod types;
pub mod validator;

// Re-export commonly used types and functions
pub use error::{EnvironmentalLawError, Result};
pub use types::{
    // Pollutant Types
    AirPollutant,
    EIA_VALIDITY_YEARS_CATEGORY_A,
    EIA_VALIDITY_YEARS_CATEGORY_B,
    EIAApprovalStatus,
    EIACategory,
    ENVIRONMENTAL_PERMIT_VALIDITY_YEARS,
    EmissionType,
    EnvironmentalImpact,
    // EIA Types
    EnvironmentalImpactAssessment,
    EnvironmentalImpactAssessmentBuilder,
    // Permits
    EnvironmentalPermit,
    EnvironmentalPermitType,
    IUCNCategory,
    // Impact Types
    ImpactSeverity,
    ImplementationPhase,
    MAX_BOD_DISCHARGE,
    MAX_COD_DISCHARGE,
    MAX_NOISE_COMMERCIAL_DAY,
    MAX_NOISE_COMMERCIAL_NIGHT,
    MAX_NOISE_INDUSTRIAL,
    MAX_NOISE_RESIDENTIAL_DAY,
    MAX_NOISE_RESIDENTIAL_NIGHT,
    MAX_PH_DISCHARGE,
    MAX_PM10_ANNUAL,
    // Constants
    MAX_PM25_ANNUAL,
    MAX_TEMPERATURE_DISCHARGE,
    MAX_TSS_DISCHARGE,
    MIN_BUFFER_ZONE_METERS,
    MIN_MINING_DISTANCE_FROM_PROTECTED_AREA,
    MIN_PH_DISCHARGE,
    // Mitigation
    MitigationMeasure,
    PermitCondition,
    PermitStatus,
    // Pollution Sources
    PollutionSource,
    PollutionSourceType,
    // Project Types
    ProjectType,
    // Protected Areas
    ProtectedArea,
    ProtectedAreaActivity,
    ProtectedAreaType,
    RestrictionLevel,
    WasteDisposalMethod,
    // Waste Types
    WasteType,
    WaterPollutant,
    // Zones
    ZoneType,
};
pub use validator::{
    // Air Quality Validators
    validate_air_quality,
    validate_air_quality_batch,
    validate_eia_approval,
    validate_eia_completeness,
    // EIA Validators
    validate_eia_requirement,
    validate_endangered_species_impact,
    // Comprehensive Validators
    validate_environmental_compliance,
    // Permit Validators
    validate_environmental_permit,
    validate_hazardous_waste_transport,
    validate_noise_impact,
    // Noise Validators
    validate_noise_level,
    validate_permit_for_activity,
    validate_pollution_monitoring,
    // Pollution Source Validators
    validate_pollution_source,
    // Protected Area Validators
    validate_protected_area_activity,
    validate_protected_area_distance,
    // Waste Validators
    validate_waste_disposal,
    validate_water_discharge_comprehensive,
    // Water Quality Validators
    validate_water_quality,
};
