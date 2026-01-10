//! Professional Licensing Across US States
//!
//! This module provides types and validation for professional licensing requirements
//! across the 50 US states and territories, focusing on:
//!
//! - **Attorney Licensing**: Bar admission, UBE portability, pro hac vice
//! - **Medical Licensing**: IMLC compact, telemedicine, prescribing authority
//! - **Architect Licensing**: NCARB certification and reciprocity
//!
//! # Key Features
//!
//! ## Uniform Bar Examination (UBE)
//!
//! The UBE is a standardized bar examination developed by the National Conference
//! of Bar Examiners (NCBE). As of 2024, 40+ jurisdictions have adopted the UBE,
//! allowing attorneys to transfer their scores between jurisdictions.
//!
//! ```rust
//! use legalis_us::professional_licensing::{UBEStatus, bar_admission};
//!
//! // Check if a state has adopted the UBE
//! let ny_status = bar_admission::ube_status("NY");
//! assert!(matches!(ny_status, UBEStatus::Adopted { .. }));
//!
//! // Check portability (CO requires 276)
//! let can_transfer = bar_admission::can_transfer_ube_score("NY", "CO", 280);
//! assert!(can_transfer);
//! ```
//!
//! ## Interstate Medical Licensure Compact (IMLC)
//!
//! The IMLC expedites the licensing process for physicians practicing in multiple
//! states, with 35+ member states as of 2024.
//!
//! ```rust
//! use legalis_us::professional_licensing::medical;
//!
//! // Check IMLC membership
//! let is_member = medical::is_imlc_member("TX");
//! assert!(is_member);
//! ```
//!
//! # Module Organization
//!
//! - [`bar_admission`]: Attorney licensing and bar admission requirements
//! - [`medical`]: Medical licensing, IMLC, and telemedicine regulations
//! - [`architect`]: Architect licensing and NCARB certification
//! - [`types`]: Common types for professional licensing

pub mod architect;
pub mod bar_admission;
pub mod medical;
pub mod types;

// Re-exports
pub use architect::{ArchitectLicensing, NCARBStatus, can_use_ncarb_certificate};
pub use bar_admission::{
    BarAdmissionRequirements, MultijurisdictionalPractice, ProHacViceRules, UBEStatus,
    can_transfer_ube_score, ube_status,
};
pub use medical::{
    IMLCStatus, PrescribingAuthority, TelemedicineRules, is_imlc_member, telemedicine_requirements,
};
pub use types::{LicenseType, LicensingAuthority, ProfessionalLicense, ReciprocityType, StateId};
