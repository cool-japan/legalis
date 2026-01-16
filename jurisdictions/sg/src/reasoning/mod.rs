//! Legal reasoning engine for Singapore law.
//!
//! This module provides automated legal analysis capabilities for Singapore law,
//! converting existing validator logic into Statute-based reasoning using legalis-core.
//!
//! # Example
//!
//! ```rust,ignore
//! use legalis_sg::reasoning::LegalReasoningEngine;
//! use legalis_sg::employment::types::EmploymentContract;
//!
//! let engine = LegalReasoningEngine::new();
//! let contract = EmploymentContract { ... };
//!
//! let analysis = engine.analyze_employment_contract(&contract);
//! println!("Compliance: {:?}", analysis.compliance_status);
//! ```

pub mod context;
pub mod engine;
pub mod error;
pub mod interop;
pub mod statute_adapter;
pub mod types;
pub mod verifier;

pub use context::SingaporeEvaluationContext;
pub use engine::LegalReasoningEngine;
pub use error::{ReasoningError, ReasoningResult};
pub use interop::{
    CompaniesActConverter, ConsumerProtectionConverter, DeonticMode, EmploymentActConverter,
    L4DeonticOperator, PdpaConverter, SingaporeL4Converter, batch_export_l4, batch_import_l4,
    generate_l4_decide, generate_l4_define, generate_l4_given, generate_l4_rule,
};
pub use statute_adapter::{
    all_singapore_statutes, banking_act_statutes, companies_act_statutes, employment_act_statutes,
    payment_services_act_statutes, pdpa_statutes,
};
pub use types::{
    ComplianceStatus, LegalAnalysis, LegalOpinion, ReasoningStep, Remedy, RemedyType, RiskLevel,
    Violation, ViolationSeverity,
};
pub use verifier::{
    HierarchyRules, SgLegalSource, SgStatuteVerifier, SgVerificationReport,
    sg_constitutional_principles,
};
