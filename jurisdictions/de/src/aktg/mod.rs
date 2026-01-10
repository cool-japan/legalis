//! German Stock Corporation Act (Aktiengesetz - AktG)
//!
//! Provides type-safe representations of German stock corporations (AG)
//! with comprehensive validation.
//!
//! # Legal Context
//!
//! The AktG (Aktiengesetz) regulates stock corporations in Germany:
//! - Minimum capital: €50,000 (§7 AktG)
//! - Two-tier board system (Management Board + Supervisory Board)
//! - Limited liability for shareholders
//! - Suitable for large companies and public listings
//!
//! # Covered Areas
//!
//! - **Formation** (§1-53 AktG): Share capital, shares, company name
//! - **Management Board** (§76-94 AktG): Composition, duties, representation
//! - **Supervisory Board** (§95-116 AktG): Composition, supervision, co-determination
//! - **Shares** (§6-12 AktG): Par value shares, no-par shares, certificates
//!
//! # Example
//!
//! ```rust
//! use legalis_de::aktg::{AG, validate_ag};
//! use legalis_de::gmbhg::Capital;
//!
//! // AG validation will be demonstrated in examples
//! ```

pub mod error;
pub mod types;
pub mod validator;

// Re-exports
pub use error::{AktGError, Result};
pub use types::*;
pub use validator::*;
