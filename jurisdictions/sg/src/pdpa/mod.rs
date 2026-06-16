//! Personal Data Protection Act 2012 (PDPA)
//!
//! Singapore's data protection framework — a **consent-centric** regime distinct
//! from the GDPR's six-lawful-bases model. This module implements:
//!
//! * **Consent** (s. 13-16): express and deemed consent (s. 15 conduct/contract,
//!   s. 15A notification), withdrawal of consent (s. 16).
//! * **Purpose limitation** (s. 18).
//! * **Data breach notification** (Part 6A, ss. 26B-26D): notifiable-breach
//!   determination, 3-calendar-day notification to the PDPC, and notification to
//!   affected individuals.
//! * **Do Not Call Registry** (Part 9): three registers, check-before-marketing.
//! * **Accountability** (s. 11): mandatory designation of a Data Protection
//!   Officer and publication of its business contact information.
//! * **Cross-border transfer** (s. 26): Transfer Limitation Obligation.
//! * **Access and correction** (s. 21-22): the 30-day access response rule.
//! * **Business contact information** exemption (s. 4(5) / s. 2(1)).
//!
//! All statutory references are to the PDPA 2012 as amended by the Personal Data
//! Protection (Amendment) Act 2020 and the PDP Regulations 2021.

pub mod error;
pub mod types;
pub mod validator;

pub use error::{PdpaError, Result};
pub use types::*;
pub use validator::{
    ConsentRecordBuilder, DataBreachBuilder, PdpaValidationReport, dpo_appointment_satisfied,
    validate_breach_notification, validate_consent, validate_cross_border_transfer,
    validate_data_subject_request, validate_dnc_before_marketing,
    validate_organisation_accountability, validate_purpose_limitation, validate_withdrawal,
};
