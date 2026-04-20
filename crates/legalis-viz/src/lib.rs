//! Legalis-Viz: Visualization engine for legal statutes.
//!
//! This crate provides visualization capabilities for legal documents:
//! - Decision trees for eligibility determination
//! - Flowcharts for legal procedures
//! - Dependency graphs between statutes
//! - Highlighting of discretionary "gray zones"

// Trait implementations for all types (Default, Display, etc.)
mod functions;
mod trait_impls;
mod types;
mod types_10;
mod types_11;
mod types_12;
mod types_3;
mod types_4;
mod types_5;
mod types_6;
mod types_7;
mod types_8;
mod types_9;

// trait_impls contains only impl blocks, not types to re-export
pub use functions::*;
pub use types::*;
pub use types_3::*;
pub use types_4::*;
pub use types_5::*;
pub use types_6::*;
pub use types_7::*;
pub use types_8::*;
pub use types_9::*;
pub use types_10::*;
pub use types_11::*;
pub use types_12::*;

#[cfg(test)]
mod tests;
