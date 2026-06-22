#![allow(clippy::needless_range_loop)]
#![allow(clippy::format_in_format_args)]

//! # Legalis-Verifier: Formal Verification for Legalis-RS Legal Statutes
//!
//! `legalis-verifier` provides static analysis and formal-verification tools
//! that detect logical inconsistencies, circular references, dead statutes,
//! constitutional conflicts, and temporal-logic violations in collections of
//! [`Statute`] objects.
//!
//! ## Overview
//!
//! Legal rule sets grow complex: statutes reference each other, preconditions
//! overlap, and constitutional constraints limit what any rule may enact.
//! `legalis-verifier` automates the detection of:
//!
//! - **Circular references** — A → B → A dependency cycles that make statutes
//!   impossible to evaluate.
//! - **Dead statutes** — rules whose preconditions can never be satisfied.
//! - **Constitutional conflicts** — violations of equality, due-process, and
//!   other constitutional principles.
//! - **Logical contradictions** — two statutes that simultaneously grant and
//!   deny the same right under identical conditions.
//! - **Redundant conditions** — preconditions that are subsumed by others and
//!   add no discriminating power.
//! - **Cross-reference errors** — references to statutes that do not exist or
//!   that create ambiguous matches.
//!
//! ## Verification Engine
//!
//! The core engine is [`StatuteVerifier`].  It applies a pipeline of checks to
//! a slice of statutes and accumulates results into a [`VerificationResult`]:
//!
//! ```text
//! &[Statute]
//!      │
//!      ▼
//! ┌────────────────────────────────────────────────────┐
//! │                  StatuteVerifier                   │
//! │  1. check_circular_references                      │
//! │  2. check_dead_statutes                            │
//! │  3. check_constitutional_compliance  (per statute) │
//! │  4. check_contradictions                           │
//! │  5. check_redundant_conditions       (per statute) │
//! │  (optional SMT back-end via `smt-solver` feature)  │
//! └────────────────────────────┬───────────────────────┘
//!                              │
//!                     VerificationResult
//!                  { passed, errors, warnings, suggestions }
//! ```
//!
//! Results are cached by default with [`StatuteVerifier::with_caching`] to
//! avoid re-verifying unchanged statutes in interactive workflows.
//!
//! ## Quick Start
//!
//! ```no_run
//! use legalis_verifier::{StatuteVerifier, VerificationResult};
//! use legalis_core::{Statute, Effect, EffectType};
//!
//! // Build a simple statute
//! let statute = Statute::new(
//!     "welfare-benefit-101",
//!     "Welfare Benefit Act",
//!     Effect::new(EffectType::Grant, "Monthly welfare payment"),
//! );
//!
//! // Verify a set of statutes
//! let verifier = StatuteVerifier::new().with_caching();
//! let result: VerificationResult = verifier.verify(&[statute]);
//!
//! if result.passed {
//!     println!("All checks passed.");
//! } else {
//!     for error in &result.errors {
//!         eprintln!("Error ({}): {}", error.severity(), error);
//!     }
//! }
//! ```
//!
//! ## Key Types
//!
//! | Type | Description |
//! |------|-------------|
//! | [`StatuteVerifier`] | Main verification engine; runs all checks against a statute slice |
//! | [`VerificationResult`] | Aggregated outcome: `passed`, `errors`, `warnings`, `suggestions` |
//! | [`VerificationError`] | Enum of all detectable error kinds with severity annotations |
//! | [`BatchVerificationResult`] | Results from verifying many statute groups in one call |
//! | [`VerificationBudget`] | Resource limits (max statutes, checks, or wall-clock time) |
//! | [`StatuteConflict`] | Conflict record between two or more statutes |
//! | [`QualityMetrics`] | Composite quality score (clarity, readability, completeness, …) |
//!
//! ## SMT Integration
//!
//! When compiled with the `smt-solver` feature, [`SmtVerifier`] is available.
//! It encodes statute preconditions as first-order logic formulae and dispatches
//! them to an embedded **OxiZ** solver (pure-Rust SMT, no C/C++ dependencies)
//! for sound satisfiability checking.  This catches subtle contradictions that
//! the syntactic engine cannot detect.
//!
//! ```toml
//! # Cargo.toml
//! [dependencies]
//! legalis-verifier = { version = "*", features = ["smt-solver"] }
//! ```

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
