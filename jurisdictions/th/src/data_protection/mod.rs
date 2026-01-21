//! Thai Personal Data Protection Act (PDPA) - พ.ร.บ. คุ้มครองข้อมูลส่วนบุคคล พ.ศ. 2562
//!
//! Thailand's PDPA (B.E. 2562 / 2019 CE) is Thailand's comprehensive data protection law,
//! similar to GDPR but with Thai-specific characteristics.
//!
//! ## Key Features
//!
//! - 6 legal bases for processing (Section 24)
//! - 8 data subject rights (Sections 30-36)
//! - 72-hour breach notification (Section 37)
//! - Data Protection Officer (DPO) requirements (Section 41)
//! - PDPC as supervisory authority

mod error;
mod types;
mod validator;

pub use error::{PdpaError, PdpaResult};
pub use types::{
    ConsentRecord, DataCategory, DataSubjectRight, LegalBasis, PersonalDataProcessing,
    ProcessingPurpose, SecurityIncident,
};
pub use validator::{
    PdpaCompliance, get_pdpa_checklist, validate_consent, validate_cross_border_transfer,
    validate_data_minimization, validate_dsr_response, validate_incident_notification,
    validate_legal_basis, validate_pdpa_compliance,
};
