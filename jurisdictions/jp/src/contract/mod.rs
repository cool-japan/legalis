//! Japanese contract law API (契約法 - Book 3: Claims/債権)
//!
//! This module provides a type-safe API for working with Japanese contract law,
//! specifically Article 415 (債務不履行による損害賠償 - Breach of Obligation).
//!
//! ## Features
//!
//! - **Builder Pattern**: Fluent API for constructing breach claims
//! - **Type Safety**: Rust's type system ensures all required elements are present
//! - **Automatic Validation**: Built-in validation logic for Article 415 requirements
//! - **Bilingual Support**: Full support for Japanese and English (日本語/English)
//!
//! ## Example
//!
//! ```rust
//! use legalis_jp::contract::{Article415, Attribution, AttributionType, BreachType, ObligationType};
//! use legalis_jp::contract::validate_breach_claim;
//! use legalis_jp::tort::{Damage, CausalLink};
//!
//! let claim = Article415::new()
//!     .with_obligation(ObligationType::Monetary {
//!         amount: 1_000_000,
//!         currency: "JPY".to_string(),
//!     })
//!     .with_breach(BreachType::NonPerformance)
//!     .with_attribution(Attribution::new(
//!         AttributionType::Negligence,
//!         "正当な理由なく履行を拒否"
//!     ))
//!     .with_damage(Damage::new(1_000_000, "契約金額"))
//!     .with_causal_link(CausalLink::Direct)
//!     .creditor("会社A")
//!     .debtor("供給業者B");
//!
//! let result = validate_breach_claim(&claim);
//! if let Ok(liability) = result {
//!     assert!(liability.is_liability_established());
//! }
//! ```

pub mod article415;
pub mod error;
pub mod types;
pub mod validator;

// Re-export commonly used types
pub use article415::{Article415, ArticleReference, BreachLiability, LiabilityStatus};
pub use error::ContractLiabilityError;
pub use types::{Attribution, AttributionType, BreachType, ObligationType};
pub use validator::{meets_all_requirements, validate_breach_claim};

// Re-export shared types from tort module
pub use crate::tort::types::{CausalLink, Damage};
