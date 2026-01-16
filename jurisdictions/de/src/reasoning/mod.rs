//! Legal reasoning engine for German law (Deutsches Recht).
//!
//! This module provides automated legal analysis capabilities for German law,
//! converting existing validator logic into Statute-based reasoning using legalis-core.
//!
//! Rechtsanalyse-Engine für deutsches Recht. Konvertiert Validierungslogik in
//! Statute-basierte Analyse mit legalis-core.
//!
//! # Key Components / Hauptkomponenten
//!
//! - [`LegalReasoningEngine`]: Main engine for analyzing legal entities
//! - [`DeEvaluationContext`]: Bridges DE types to legalis-core EvaluationContext
//! - Statute definitions for ArbZG, KSchG, BGB, BUrlG, EFZG, etc.
//!
//! # Example / Beispiel
//!
//! ```rust,ignore
//! use legalis_de::reasoning::LegalReasoningEngine;
//! use legalis_de::arbeitsrecht::types::EmploymentContract;
//!
//! let engine = LegalReasoningEngine::new();
//! let contract = EmploymentContract { ... };
//!
//! let analysis = engine.analyze_employment_contract(&contract)?;
//! println!("Konformität: {:?}", analysis.status);
//! ```

pub mod context;
pub mod engine;
pub mod error;
pub mod simulation;
pub mod statute_adapter;
pub mod types;
pub mod verifier;

pub use context::DeEvaluationContext;
pub use engine::LegalReasoningEngine;
pub use error::{ReasoningError, ReasoningResult};
pub use simulation::{
    ArbZGConfig, ArbZGResult, ArbZGSimulator, BUrlGConfig, BUrlGResult, BUrlGSimulator,
    MindestlohnConfig, MindestlohnResult, MindestlohnSimulator, de_income_tax_preset,
    de_labor_market, de_macroeconomic_indicators_2024, de_unemployment_insurance_preset,
    simulate_de_income_tax, simulate_unemployment_insurance,
};
pub use statute_adapter::{
    all_labor_statutes, arbzg_section_3_daily_hours, arbzg_section_4_rest_breaks,
    arbzg_section_5_daily_rest, betrvg_section_102_works_council, bgb_section_622_notice_periods,
    bgb_section_623_written_form, burlg_section_3_minimum_leave, efzg_section_3_sick_pay,
    kschg_section_1_dismissal_protection, milog_minimum_wage, tzbfg_section_14_fixed_term,
};
pub use types::{ComplianceStatus, LegalAnalysis, RiskLevel, Violation, ViolationSeverity};
pub use verifier::{
    DeStatuteVerifier, DeVerificationReport, HierarchyRules, Rechtsquelle,
    de_constitutional_principles,
};
