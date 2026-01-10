//! Legal reasoning engine for French law.
//!
//! This module provides automated legal analysis capabilities for French law,
//! converting existing validator logic into Statute-based reasoning.
//!
//! # Example
//!
//! ```rust,ignore
//! use legalis_fr::reasoning::LegalReasoningEngine;
//! use legalis_fr::contract::Contract;
//!
//! let engine = LegalReasoningEngine::new();
//! let contract = Contract::new()
//!     .with_type(ContractType::Sale { price: 50_000, subject: "Machine".to_string() })
//!     .with_consent(true);
//!
//! let analysis = engine.analyze_contract(&contract);
//! println!("Compliance: {:?}", analysis.compliance_status);
//! ```

pub mod analyzer;
pub mod context;
pub mod engine;
pub mod error;
pub mod statute_adapter;
pub mod types;

pub use analyzer::{CompanyAnalyzer, ContractAnalyzer, FrenchLawAnalyzer, LaborAnalyzer};
pub use engine::LegalReasoningEngine;
pub use error::{ReasoningError, ReasoningResult};
pub use statute_adapter::{
    all_french_statutes, company_law_statutes, contract_law_statutes, labor_law_statutes,
};
pub use types::{
    ComplianceStatus, EntityType, LegalAnalysis, LegalOpinion, ReasoningStep, Remedy, RemedyType,
    RiskLevel, Violation, ViolationSeverity,
};
