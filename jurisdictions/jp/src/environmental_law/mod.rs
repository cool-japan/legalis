//! Environmental Law
//!
//! Implementation of:
//! - Air Pollution Control Act (大気汚染防止法 Act No. 97 of 1968)
//! - Water Pollution Prevention Act (水質汚濁防止法 Act No. 138 of 1970)
//! - Waste Management Act (廃棄物の処理及び清掃に関する法律 Act No. 137 of 1970)
//!
//! ## Air Pollution Control Act (大気汚染防止法)
//!
//! Key provisions:
//! - **Article 3**: Emission standards for soot and smoke
//! - **Article 6**: Prior notification of factory installation (60 days before)
//! - **Article 16**: Monitoring and measurement obligations
//!
//! ## Water Pollution Prevention Act (水質汚濁防止法)
//!
//! Key provisions:
//! - **Article 3**: Effluent standards
//! - **Article 5**: Notification obligations for specified facilities
//! - **Article 14**: Water quality monitoring
//!
//! ## Waste Management Act (廃棄物処理法)
//!
//! Key provisions:
//! - **Article 7**: Collection and transport business permit (5-year validity)
//! - **Article 8**: Technical standards for facilities
//! - **Article 12-3**: Manifest system for industrial waste
//! - **Article 14**: Disposal business permit (7-year validity)
//!
//! ## Examples
//!
//! ### Pollution Prevention Agreement Validation
//!
//! ```
//! use legalis_jp::environmental_law::{
//!     PollutionPreventionAgreement, FacilityType, PollutionType,
//!     EmissionLimit, Pollutant, validate_pollution_prevention_agreement
//! };
//! use chrono::Utc;
//!
//! let agreement = PollutionPreventionAgreement {
//!     facility_name: "東京発電所".to_string(),
//!     facility_type: FacilityType::PowerPlant,
//!     operator: "株式会社テスト電力".to_string(),
//!     location: "東京都".to_string(),
//!     pollution_types: vec![PollutionType::Air],
//!     emission_limits: vec![EmissionLimit {
//!         pollutant: Pollutant::SulfurOxides,
//!         limit_value: 80.0,
//!         unit: "ppm".to_string(),
//!         legal_basis: "大気汚染防止法第3条".to_string(),
//!     }],
//!     monitoring_requirements: vec![],
//!     effective_date: Utc::now().date_naive(),
//! };
//!
//! let report = validate_pollution_prevention_agreement(&agreement)?;
//! assert!(report.is_valid());
//! # Ok::<(), legalis_jp::environmental_law::EnvironmentalError>(())
//! ```
//!
//! ### Waste Management Permit Validation
//!
//! ```
//! use legalis_jp::environmental_law::{
//!     WasteManagementPermit, WastePermitType, WasteType,
//!     validate_waste_management_permit
//! };
//! use chrono::{Utc, Duration};
//!
//! let permit = WasteManagementPermit {
//!     permit_type: WastePermitType::Collection,
//!     permit_number: "廃棄-001".to_string(),
//!     operator_name: "株式会社テスト廃棄物".to_string(),
//!     waste_types: vec![WasteType::Industrial],
//!     processing_capacity_tons_per_day: 100.0,
//!     issue_date: Utc::now().date_naive(),
//!     expiration_date: Utc::now().date_naive() + Duration::days(365 * 5),
//!     facility_standards_met: true,
//! };
//!
//! let report = validate_waste_management_permit(&permit)?;
//! assert!(report.is_valid());
//! # Ok::<(), legalis_jp::environmental_law::EnvironmentalError>(())
//! ```

pub mod error;
pub mod types;
pub mod validator;

// Re-export commonly used types
pub use error::{EnvironmentalError, Result};
pub use types::{
    ControlEquipment, EmissionEstimate, EmissionLimit, FacilityType, FactorySetupNotification,
    HeavyMetal, MonitoringRequirement, Party, Pollutant, PollutionPreventionAgreement,
    PollutionType, WasteManagementPermit, WasteManifest, WastePermitType, WasteType,
};
pub use validator::{
    quick_validate_pollution, quick_validate_waste, validate_factory_setup_notification,
    validate_pollution_prevention_agreement, validate_waste_management_permit,
    validate_waste_manifest,
};
