//! Personal Data Protection Act 2012 — type definitions.
//!
//! This module is split into focused submodules, each modelling one part of
//! Singapore's PDPA framework. All types are re-exported at this level so that
//! `use legalis_sg::pdpa::types::*;` continues to bring everything into scope.
//!
//! | Submodule | PDPA coverage |
//! |-----------|---------------|
//! | [`consent`] | Consent (s. 13-16), deemed consent (s. 15/15A), purpose limitation (s. 18), business contact information exemption (s. 4(5)/s. 2(1)) |
//! | [`breach`] | Notifiable data breach determination (s. 26B), assessment (s. 26C), notification to PDPC and individuals (s. 26D) |
//! | [`dnc`] | Do Not Call Registry (Part 9): three registers and the 21-day check-before-marketing rule |
//! | [`organisation`] | Regulated organisations, mandatory DPO designation (s. 11), financial penalties (s. 48J) |
//! | [`transfer`] | Cross-border transfer / Transfer Limitation Obligation (s. 26) |
//! | [`access`] | Access (s. 21) and correction (s. 22) requests with the 30-day response rule |
//!
//! ## Key differences from the GDPR
//!
//! | Feature | GDPR | PDPA (Singapore) |
//! |---------|------|------------------|
//! | Legal basis | 6 lawful bases (Art. 6) | Consent-centric, with deemed consent (s. 15/15A) and Schedule exceptions |
//! | DPO | Mandatory only for certain processing (Art. 37) | Always mandatory to designate (s. 11(3)) |
//! | Breach notification | 72 hours to authority (Art. 33) | 3 calendar days to PDPC from assessment (s. 26D(1)) |
//! | Marketing | Consent / legitimate interests | Consent **plus** DNC register check for tele-marketing (Part 9) |
//! | Max fine | €20M / 4% global turnover | SGD 1M, or 10% of Singapore turnover if turnover > SGD 10M (s. 48J(3)) |

pub mod access;
pub mod breach;
pub mod consent;
pub mod dnc;
pub mod organisation;
pub mod transfer;

pub use access::{ACCESS_REQUEST_RESPONSE_DAYS, DataSubjectRequest, DataSubjectRequestKind};
pub use breach::{
    BreachScope, BreachType, DataBreachNotification, IndividualNotificationExemption,
    NotifiabilityAssessment, PDPC_NOTIFICATION_DEADLINE_DAYS, SIGNIFICANT_SCALE_THRESHOLD,
    calendar_days_between,
};
pub use consent::{
    ConsentMethod, ConsentRecord, DataContext, DeemedConsentBasis, PersonalDataCategory,
    PurposeOfCollection, is_business_contact_information,
};
pub use dnc::{
    DNC_CONFIRMATION_VALIDITY_DAYS, DncCheckConfirmation, DncRegisterKind, DncRegistration,
};
pub use organisation::{
    DpoContact, DpoStaffingRecommendation, HIGH_TURNOVER_PENALTY_PERCENT,
    HIGH_TURNOVER_THRESHOLD_SGD, MAX_PENALTY_SGD, OrganisationType, PdpaOrganisation,
    max_financial_penalty_sgd,
};
pub use transfer::{DataTransfer, TransferMechanism};
