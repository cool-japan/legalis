#![allow(clippy::unwrap_in_result)]

//! Legalis-Chain: Smart contract export for Legalis-RS.
//!
//! This crate provides export functionality to convert deterministic
//! legal statutes into smart contracts (WASM/Solidity).

mod contractgenerator_impl;
mod contractgenerator_impl_1;
mod contractgenerator_impl_10;
mod contractgenerator_impl_11;
mod contractgenerator_impl_12;
mod contractgenerator_impl_13;
mod contractgenerator_impl_14;
mod contractgenerator_impl_2;
mod contractgenerator_impl_3;
mod contractgenerator_impl_4;
mod contractgenerator_impl_5;
mod contractgenerator_impl_6;
mod contractgenerator_impl_7;
mod contractgenerator_impl_8;
mod contractgenerator_impl_9;
mod contractgenerator_type;
mod functions;
mod trait_impls;
mod types;
mod types_19;

// Re-export public API
pub use contractgenerator_type::*;
pub use functions::*;
pub use types::*;
pub use types_19::*;
