//! Legal reasoning engine for UK law (England & Wales).
//!
//! This module provides automated legal analysis capabilities for UK law,
//! converting existing validator logic into Statute-based reasoning using legalis-core.
//!
//! # Key Components
//!
//! - [`LegalReasoningEngine`]: Main engine for analyzing legal entities
//! - [`UkEvaluationContext`]: Bridges UK types to legalis-core EvaluationContext
//! - Statute definitions for ERA 1996, WTR 1998, NMWA 1998, etc.
//!
//! # Example
//!
//! ```rust,ignore
//! use legalis_uk::reasoning::LegalReasoningEngine;
//! use legalis_uk::employment::types::EmploymentContract;
//!
//! let engine = LegalReasoningEngine::new();
//! let contract = EmploymentContract { ... };
//!
//! let analysis = engine.analyze_employment_contract(&contract)?;
//! println!("Compliance: {:?}", analysis.status);
//! ```

pub mod context;
pub mod engine;
pub mod error;
pub mod lod;
pub mod simulation;
pub mod statute_adapter;
pub mod types;
pub mod verifier;

pub use context::UkEvaluationContext;
pub use engine::LegalReasoningEngine;
pub use error::{ReasoningError, ReasoningResult};
pub use lod::{
    LegislationExporter, LegislationMapper, export_employment_statutes_to_legislation,
    get_legislation_uri,
};
pub use simulation::{
    AnnualLeaveConfig, AnnualLeaveResult, AnnualLeaveSimulator, NationalLivingWageConfig,
    NationalLivingWageResult, NationalLivingWageSimulator, WTRConfig, WTRResult, WTRSimulator,
    simulate_uk_income_tax, simulate_unemployment_benefit, uk_income_tax_preset, uk_labor_market,
    uk_macroeconomic_indicators_2024, uk_unemployment_benefit_preset,
};
pub use statute_adapter::{
    all_employment_statutes, era_section_1_written_particulars, era_section_86_notice_periods,
    era_section_98_unfair_dismissal, era_section_162_redundancy, nmwa_minimum_wage,
    pension_auto_enrolment, wtr_regulation_4_working_time, wtr_regulation_12_rest_breaks,
    wtr_regulation_13_annual_leave,
};
pub use types::{ComplianceStatus, LegalAnalysis, RiskLevel, Violation, ViolationSeverity};
pub use verifier::{
    HierarchyRules, UkLegalSource, UkStatuteVerifier, UkVerificationReport,
    uk_human_rights_principles,
};
