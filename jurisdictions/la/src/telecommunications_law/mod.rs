//! Telecommunications Law Module for Lao PDR (ກົດໝາຍໂທລະຄົມມະນາຄົມ)
//!
//! This module models the **Law on Telecommunications (Lao PDR), No. 09/NA,
//! 2011** (ກົດໝາຍວ່າດ້ວຍໂທລະຄົມມະນາຄົມ).
//!
//! # Legal Framework
//!
//! The Law on Telecommunications governs the provision of telecommunications
//! services, the licensing of operators, the management of radio-frequency
//! spectrum, interconnection between networks, quality of service and tariffs,
//! type-approval of equipment, and the confidentiality of communications in the
//! Lao People's Democratic Republic. It is administered through the ministry
//! responsible for posts and telecommunications and its telecommunications
//! regulatory authority. Universal service and fair competition (anti-monopoly)
//! are recognised policy goals.
//!
//! # Key Provisions Modelled
//!
//! - **Licensing** — operators require a licence; categories cover network
//!   facilities, network services, application services and spectrum
//!   ([`TelecomLicense`], [`LicenseCategory`], [`validate_license`]).
//! - **Radio-frequency spectrum** — a scarce national resource assigned in
//!   non-overlapping bands ([`SpectrumAssignment`],
//!   [`validate_spectrum_assignment`], [`validate_spectrum_no_overlap`]).
//! - **Interconnection** — provided on fair, reasonable and non-discriminatory
//!   terms ([`InterconnectionRequest`], [`validate_interconnection`]).
//! - **Quality of service and tariffs** ([`ServiceQuality`], [`Tariff`],
//!   [`validate_service_quality`], [`validate_tariff`]).
//! - **Equipment type-approval** ([`EquipmentTypeApproval`],
//!   [`validate_equipment_type_approval`]).
//! - **Confidentiality of communications** — unlawful interception is prohibited
//!   ([`validate_communication_confidentiality`]).
//!
//! # Legal Accuracy Note
//!
//! The law number (No. 09/NA, 2011) is recorded as it appears in the available
//! sources. Where the precise internal article numbers cannot be independently
//! verified by this crate, provisions are cited by the law's name and year
//! ([`TELECOMMUNICATIONS_LAW_CITATION`]) together with a documented topic
//! descriptor, and quantifiable requirements are encoded as named constants (for
//! example [`LICENSE_VALIDITY_YEARS`] and [`SPECTRUM_MAX_GHZ`]) rather than as
//! fabricated article references.
//!
//! # Example
//!
//! ```
//! use legalis_la::telecommunications_law::*;
//!
//! let license = TelecomLicense {
//!     operator: "Lao Telecom".to_string(),
//!     category: LicenseCategory::NetworkServices,
//!     granted: true,
//!     validity_years: 15,
//!     start_year: 2020,
//!     status: LicenseStatus::Active,
//! };
//! assert!(validate_license(&license).is_ok());
//!
//! let assignment = SpectrumAssignment {
//!     operator: "Lao Telecom".to_string(),
//!     band_start_mhz: 900,
//!     band_end_mhz: 960,
//!     exclusive: true,
//! };
//! assert!(validate_spectrum_assignment(&assignment).is_ok());
//! ```

pub mod error;
pub mod types;
pub mod validator;

pub use error::{
    TELECOMMUNICATIONS_LAW_CITATION, TelecommunicationsLawError, TelecommunicationsResult,
};

pub use types::{
    // Equipment
    EquipmentTypeApproval,
    // Interconnection
    InterconnectionRequest,
    // Constants
    LICENSE_VALIDITY_YEARS,
    // Service & licence classification
    LicenseCategory,
    LicenseStatus,
    MAX_CALL_DROP_RATE_PERMILLE,
    MIN_SERVICE_AVAILABILITY_PERCENT,
    OperatorType,
    SPECTRUM_MAX_GHZ,
    SPECTRUM_MAX_MHZ,
    SPECTRUM_MIN_KHZ,
    ServiceQuality,
    ServiceType,
    // Spectrum
    SpectrumAssignment,
    // Quality of service & tariffs
    Tariff,
    // Licensing
    TelecomLicense,
};

pub use validator::{
    validate_communication_confidentiality, validate_equipment_type_approval,
    validate_interconnection, validate_license, validate_service_quality,
    validate_spectrum_assignment, validate_spectrum_no_overlap, validate_tariff,
};
