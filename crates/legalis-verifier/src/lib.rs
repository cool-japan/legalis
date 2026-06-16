#![allow(clippy::needless_range_loop)]
#![allow(clippy::format_in_format_args)]

//! Legalis-Verifier: Formal verification for Legalis-RS legal statutes.
//!
//! This crate provides static analysis and verification tools for detecting
//! logical inconsistencies, circular references, and constitutional conflicts
//! in legal statutes.

#[cfg(feature = "smt-solver")]
mod smt;

#[cfg(feature = "smt-solver")]
pub use smt::SmtVerifier;

pub mod autonomous_agents;
pub mod certification_framework;
pub mod compliance_frameworks;
pub mod conflict_of_laws;
pub mod cross_domain_verification;
pub mod distributed_verification;
pub mod formal_methods;
pub mod ml_verification;
pub mod model_checking;
pub mod quantum_verification;
pub mod realtime_verification;
pub mod self_healing;
pub mod streaming_verification;

mod functions;
mod functions_2;
mod functions_3;
mod functions_4;
mod functions_5;
mod functions_6;
mod trait_impls;
mod types;
mod types_3;
mod types_4;
mod types_5;

pub use functions::*;
pub use functions_2::*;
pub use functions_3::*;
pub use functions_4::*;
pub use functions_5::*;
pub use functions_6::*;
pub use types::*;
pub use types_3::*;
pub use types_4::*;
pub use types_5::*;

// Re-export legalis_core types used by sub-crates
pub use legalis_core::Statute;

#[cfg(test)]
mod tests;
