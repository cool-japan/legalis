//! Managed Investments Module (Corporations Act 2001 Chapter 5C)
//!
//! Managed investment scheme regulation and responsible entity obligations.
//!
//! # Key Legislation
//!
//! ## Corporations Act 2001 Chapter 5C
//!
//! - **Part 5C.1**: Registration of managed investment schemes
//! - **Part 5C.2**: Responsible entity requirements
//! - **Part 5C.3**: Compliance plans
//! - **Part 5C.4**: Powers and duties of responsible entity

pub mod error;
pub mod types;
pub mod validator;

pub use error::{ManagedInvestmentsError, Result};
pub use types::{CompliancePlan, ManagedInvestmentScheme, ResponsibleEntity, SchemeType};
pub use validator::{validate_compliance_plan, validate_responsible_entity, validate_scheme};
