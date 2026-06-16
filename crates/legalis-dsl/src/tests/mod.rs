//! Test suite for the Legalis DSL parser.
//!
//! Tests are split across multiple submodules to keep files under 2000 lines.

pub use super::*;

mod advanced;
mod basic;
mod contract;
mod formal;
mod prop_tests;
mod roundtrip;
mod serialization;
mod snapshot;
mod testspec;
mod unicode;
