//! Financial Advice Module (Corporations Act 2001 Part 7.7)
//!
//! Best interests duty, disclosure obligations, and conflicted remuneration rules.
//!
//! # Key Legislation
//!
//! ## Best Interests Duty (Division 2, Part 7.7A)
//!
//! **Section 961B: Duty to act in the best interests of the client**
//!
//! When providing personal advice to retail clients, advisers must:
//! - Act in the best interests of the client (s.961B(1))
//! - Follow safe harbour steps (s.961B(2))
//! - Give priority to client's interests (s.961J)
//!
//! ## Disclosure Documents
//!
//! - **FSG**: Financial Services Guide (s.941A-942C)
//! - **PDS**: Product Disclosure Statement (s.1012A-1013L)
//! - **SOA**: Statement of Advice (s.946A-947D)
//!
//! ## Conflicted Remuneration (Division 4, Part 7.7A)
//!
//! **Section 963E**: Ban on conflicted remuneration
//! - Volume-based benefits prohibited
//! - Soft dollar benefits generally prohibited
//! - Asset-based fees on borrowed amounts prohibited in super

pub mod error;
pub mod types;
pub mod validator;

pub use error::{AdviceError, Result};
pub use types::{
    AdviceDocument, AdviceType, BestInterestsAssessment, ConflictedRemuneration,
    FinancialServicesGuide, ProductDisclosureStatement, SafeHarbourStep, StatementOfAdvice,
};
pub use validator::{
    validate_best_interests_duty, validate_conflicted_remuneration, validate_fsg, validate_pds,
    validate_soa,
};
