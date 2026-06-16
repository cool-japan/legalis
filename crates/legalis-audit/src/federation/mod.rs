//! Global audit federation (v0.3.4, in-crate portion).
//!
//! This module implements the *pure-Rust, locally-computable* parts of global
//! audit federation: **international standard mapping** and **multi-jurisdiction
//! compliance**. (The remaining v0.3.4 items — cross-border coordination, global
//! intelligence sharing, and treaty-based cooperation — require external
//! networks and inter-organisation agreements and are out of scope for a
//! self-contained crate.)
//!
//! - [`standards`] models the world's compliance frameworks as structured data:
//!   [`Standard`]s, their [`Control`]s, and the cross-standard
//!   [`ControlObjective`] pivot that recognises when controls from different
//!   frameworks address the same capability. A [`StandardMapping`] answers
//!   cross-mapping and coverage queries and ships with a curated built-in
//!   catalogue.
//! - [`jurisdiction`] evaluates a system's provided audit capabilities — either
//!   supplied directly or *derived from a live audit trail* — against the
//!   standards each [`Jurisdiction`] mandates, producing a
//!   [`MultiJurisdictionReport`] with per-jurisdiction status and the minimal
//!   remediation set for global compliance.

pub mod jurisdiction;
pub mod standards;

pub use jurisdiction::{
    Jurisdiction, JurisdictionCompliance, MultiJurisdictionEvaluator, MultiJurisdictionReport,
    derive_objectives,
};
pub use standards::{
    Control, ControlObjective, CoverageReport, CrossMapping, Standard, StandardMapping,
};
