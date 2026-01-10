//! Personal Data Protection Act 2012 (PDPA)
//!
//! Singapore's data protection framework - distinct from GDPR with consent-centric model.

pub mod error;
pub mod types;
pub mod validator;

pub use error::{PdpaError, Result};
pub use types::*;
pub use validator::*;
