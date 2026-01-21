//! Indonesian Personal Data Protection Law (UU PDP) - UU No. 27 Tahun 2022
//!
//! Indonesia's comprehensive data protection law, enacted in 2022, establishing
//! GDPR-inspired privacy rights with Pancasila values integration.
//!
//! ## Key Features
//!
//! - 7 data subject rights (Pasal 5-13)
//! - Data controller and processor obligations (Pasal 20-38)
//! - Cross-border data transfer rules (Pasal 55-56)
//! - Data breach notification within 3x24 hours (Pasal 46)
//! - Criminal sanctions up to 6 years imprisonment (Pasal 67-73)
//!
//! ## Legal Basis for Processing (Pasal 20)
//!
//! 1. Consent (persetujuan)
//! 2. Contract performance
//! 3. Legal obligation
//! 4. Vital interests
//! 5. Public interest
//! 6. Legitimate interests

mod error;
mod types;
mod validator;

pub use error::{PdpError, PdpResult};
pub use types::{
    ConsentRecord, DataCategory, DataSubjectRight, LegalBasis, PersonalData,
    PersonalDataProcessing, ProcessingPurpose, RiskLevel, SecurityIncident, SpecificDataType,
};
pub use validator::{
    PdpCompliance, get_pdp_checklist, validate_consent, validate_cross_border_transfer,
    validate_data_retention, validate_incident_notification, validate_legal_basis,
    validate_pdp_compliance,
};
