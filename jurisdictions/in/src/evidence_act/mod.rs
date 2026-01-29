//! Indian Evidence Act 1872 / Bharatiya Sakshya Adhiniyam (BSA) 2023
//!
//! Evidence law for India covering admissibility, relevance, and burden of proof.
//!
//! ## Key Sections
//!
//! - **Section 3-55**: Relevancy of facts
//! - **Section 60-92**: Oral evidence
//! - **Section 91-100**: Documentary evidence  
//! - **Section 101-114**: Burden of proof
//! - **Section 24-30**: Confessions

pub mod error;
pub mod types;
pub mod validator;

pub use error::*;
pub use types::*;
pub use validator::*;
