//! Legal Reasoning Engine for EU Law.
//!
//! Provides automated compliance analysis for EU regulations including:
//! - GDPR (General Data Protection Regulation 2016/679)
//! - Competition Law (Articles 101-102 TFEU)
//! - Consumer Rights Directive (2011/83/EU)

mod context;
mod engine;
mod error;
mod lod;
mod simulation;
mod statute_adapter;
mod types;
mod verifier;

pub use context::EuEvaluationContext;
pub use engine::LegalReasoningEngine;
pub use error::{ReasoningError, ReasoningResult};
pub use lod::{CelexMapper, EurLexExporter, export_gdpr_statutes_to_eurlex, get_eurlex_uri};
pub use simulation::{
    GdprComplianceConfig, GdprComplianceCostResult, GdprFineConfig, GdprFineResult,
    GdprFineSimulator, GdprFineTier, GdprViolation, gdpr_compliance_preset,
    simulate_gdpr_compliance_costs, simulate_gdpr_economic_impact,
};
pub use statute_adapter::{
    all_eu_statutes, competition_statutes, consumer_rights_statutes, gdpr_statutes,
};
pub use types::{
    ComplianceStatus, LegalAnalysis, ReasoningStep, RiskLevel, Violation, ViolationSeverity,
};
pub use verifier::{
    EuLegalSource, EuStatuteVerifier, EuVerificationReport, GdprPrinciple, HierarchyRules,
    eu_fundamental_rights_principles,
};
