//! Contract Template Generation System (契約書生成システム)
//!
//! This module provides a complete system for generating Japanese legal contracts
//! from templates with variable substitution.
//!
//! # Features
//!
//! - **Template Engine** (テンプレートエンジン)
//!   - Variable substitution with {{variable}} syntax
//!   - Conditional blocks with {{#if}}/{{/if}} and {{#unless}}/{{/unless}}
//!   - Type-safe variable validation
//!
//! - **Clause Library** (条項ライブラリ)
//!   - Pre-built standard clauses for common scenarios
//!   - Risk level classification
//!   - Bilingual support (Japanese/English)
//!
//! - **Contract Templates** (契約テンプレート)
//!   - Employment contracts (雇用契約書)
//!   - Non-disclosure agreements (秘密保持契約書)
//!   - Service agreements (業務委託契約書)
//!   - Sales contracts (売買契約書)
//!   - Lease agreements (賃貸借契約書)
//!
//! # Examples
//!
//! ## Basic Template Rendering
//!
//! ```rust
//! use legalis_jp::contract_templates::*;
//!
//! // Create template engine
//! let mut engine = TemplateEngine::new();
//!
//! // Create a simple template
//! let template = ContractTemplate::new(
//!     "simple_agreement",
//!     "業務委託契約書",
//!     TemplateType::ServiceAgreement,
//!     "{{client}}（甲）と{{contractor}}（乙）は、{{service}}に関する業務委託契約を締結する。"
//! );
//!
//! engine.register_template(template);
//!
//! // Create context with variables
//! let mut context = TemplateContext::new();
//! context.set_string("client", "株式会社ABC");
//! context.set_string("contractor", "山田太郎");
//! context.set_string("service", "ウェブサイト開発");
//!
//! // Render contract
//! let contract = engine.render("simple_agreement", &context).unwrap();
//! println!("{}", contract.content_ja);
//! ```
//!
//! ## Using the Clause Library
//!
//! ```rust
//! use legalis_jp::contract_templates::*;
//!
//! // Access standard clauses
//! let library = ClauseLibrary::new();
//! let purpose_clause = library.get_clause("purpose").unwrap();
//!
//! println!("Title: {}", purpose_clause.title_ja);
//! println!("Risk Level: {:?}", purpose_clause.risk_level);
//!
//! // Get all confidentiality clauses
//! let confidentiality = library.get_clauses_by_category(ClauseCategory::Confidentiality);
//! for clause in confidentiality {
//!     println!("- {}", clause.title_ja);
//! }
//! ```
//!
//! ## Conditional Content
//!
//! ```rust
//! use legalis_jp::contract_templates::*;
//!
//! let template_content = "
//! 雇用期間: {{start_date}}から{{end_date}}まで
//!
//! {{#if has_probation}}
//! 試用期間: {{probation_months}}ヶ月
//! {{/if}}
//!
//! {{#unless is_part_time}}
//! フルタイム雇用として、週5日勤務とする。
//! {{/unless}}
//! ";
//!
//! let mut context = TemplateContext::new();
//! context.set_string("start_date", "2024-04-01");
//! context.set_string("end_date", "2025-03-31");
//! context.set_boolean("has_probation", true);
//! context.set_integer("probation_months", 3);
//! context.set_boolean("is_part_time", false);
//! ```

pub mod compliance;
pub mod employment_helper;
pub mod engine;
pub mod error;
pub mod library;
pub mod types;

// Re-export commonly used types
pub use compliance::{
    CheckStatus, ComplianceCheck, ComplianceReport, ComplianceViolation, ComplianceWarning,
};
pub use engine::TemplateEngine;
pub use error::{Result, TemplateError};
pub use library::ClauseLibrary;
pub use types::{
    Clause, ClauseCategory, ContractTemplate, EmploymentSubtype, GeneratedContract, LeaseSubtype,
    NDASubtype, RiskLevel, TemplateContext, TemplateType, VariableValue,
};
