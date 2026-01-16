//! Legal Reasoning Engine for US Law.
//!
//! Provides integration with legalis-core reasoning framework for
//! federal and state law compliance analysis.

mod context;
mod engine;
mod error;
mod simulation;
mod statute_adapter;
mod types;
mod verifier;

pub use context::UsEvaluationContext;
pub use engine::LegalReasoningEngine;
pub use error::{ReasoningError, ReasoningResult};
pub use simulation::{
    FlsaMinimumWageConfig, FlsaMinimumWageSimulator, FlsaOvertimeConfig, FlsaOvertimeResult,
    FlsaOvertimeSimulator, FlsaSimulationResult, simulate_federal_income_tax,
    simulate_unemployment_insurance, us_federal_income_tax_preset, us_labor_market,
    us_macroeconomic_indicators_2024, us_unemployment_insurance_preset,
};
pub use statute_adapter::{all_federal_statutes, employment_statutes, tax_statutes};
pub use types::{
    ComplianceStatus, LegalAnalysis, ReasoningStep, RiskLevel, Violation, ViolationSeverity,
};
pub use verifier::{
    HierarchyRules, LegalHierarchy, PreemptionType, UsStatuteVerifier, UsVerificationReport,
    us_constitutional_principles,
};
