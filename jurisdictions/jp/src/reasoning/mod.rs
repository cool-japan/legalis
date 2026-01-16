//! Legal reasoning engine for Japanese law (日本法).
//!
//! This module provides automated legal analysis capabilities for Japanese law,
//! converting existing validator logic into Statute-based reasoning using legalis-core.
//!
//! 日本法の法的推論エンジン。legalis-coreを使用してStatuteベースの推論を提供。
//!
//! # Key Components / 主要コンポーネント
//!
//! - [`LegalReasoningEngine`]: Main engine for analyzing legal entities / 法的エンティティ分析エンジン
//! - [`JpEvaluationContext`]: Bridges JP types to legalis-core EvaluationContext
//! - Statute definitions for Labor Standards Act, Labor Contract Act, etc.
//!
//! # Example / 使用例
//!
//! ```rust,ignore
//! use legalis_jp::reasoning::LegalReasoningEngine;
//! use legalis_jp::labor_law::types::EmploymentContract;
//!
//! let engine = LegalReasoningEngine::new();
//! let contract = EmploymentContract { ... };
//!
//! let analysis = engine.analyze_employment_contract(&contract)?;
//! println!("コンプライアンス: {:?}", analysis.status);
//! ```

pub mod context;
pub mod engine;
pub mod error;
pub mod simulation;
pub mod statute_adapter;
pub mod types;
pub mod verifier;

pub use context::JpEvaluationContext;
pub use engine::LegalReasoningEngine;
pub use error::{ReasoningError, ReasoningResult};
pub use simulation::{
    MinimumWageConfig, MinimumWageResult, MinimumWageSimulator, PaidLeaveConfig, PaidLeaveResult,
    PaidLeaveSimulator, WorkStyleReformConfig, WorkStyleReformResult, WorkStyleReformSimulator,
    jp_employment_insurance_preset, jp_income_tax_preset, jp_labor_market,
    jp_macroeconomic_indicators_2024, simulate_employment_insurance, simulate_jp_income_tax,
};
pub use statute_adapter::{
    all_labor_statutes, lca_article_16_abusive_dismissal, lca_article_18_indefinite_conversion,
    lsa_article_20_dismissal_notice, lsa_article_32_working_hours, lsa_article_34_rest_periods,
    lsa_article_35_weekly_day_off, lsa_article_36_overtime_agreement,
    lsa_article_37_overtime_premium, lsa_article_39_annual_leave, minimum_wage_act,
    overtime_limit_regulation,
};
pub use types::{ComplianceStatus, LegalAnalysis, RiskLevel, Violation, ViolationSeverity};
pub use verifier::{
    HierarchyRules, JpStatuteVerifier, JpVerificationReport, LegalHierarchy,
    jp_constitutional_principles,
};
