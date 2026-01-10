//! Enhanced tort law API with builder pattern
//!
//! This module provides a type-safe, ergonomic API for working with Japanese tort law,
//! specifically Article 709 (不法行為による損害賠償).
//!
//! ## Features
//!
//! - **Builder Pattern**: Fluent API for constructing tort claims
//! - **Type Safety**: Rust's type system ensures all required elements are present
//! - **Automatic Validation**: Built-in validation logic for Article 709 requirements
//! - **Japanese Support**: Full bilingual support (日本語/English)
//!
//! ## Example
//!
//! ```rust
//! use legalis_jp::tort::{Article709, Intent, Damage, CausalLink, ProtectedInterest};
//! use legalis_jp::tort::validate_tort_claim;
//!
//! let claim = Article709::new()
//!     .with_act("交通事故で相手の車に衝突")
//!     .with_intent(Intent::Negligence)
//!     .with_victim_interest(ProtectedInterest::Property("車両所有権"))
//!     .with_damage(Damage::new(500_000, "修理費 + レッカー代"))
//!     .with_causal_link(CausalLink::Direct);
//!
//! let result = validate_tort_claim(&claim);
//! if let Ok(liability) = result {
//!     assert!(liability.is_liability_established());
//! }
//! ```

pub mod article709;
pub mod article710;
pub mod article715;
pub mod error;
pub mod types;
pub mod validator;

// Re-export commonly used types
pub use article709::{
    Article709, Article715Builder, ArticleReference, LiabilityStatus, TortLiability, article_715_1,
};
pub use article710::{Article710, Article710Liability, NonPecuniaryLiabilityStatus};
pub use article715::{Article715, Article715Liability, VicariousLiabilityStatus};
pub use error::{TortClaimError, ValidationError};
pub use types::{
    CausalLink, Damage, DamageType, EmploymentRelationship, EmploymentType, HarmSeverity, Intent,
    Negligence, NonPecuniaryDamageType, ProtectedInterest,
};
pub use validator::{
    calculate_compensation, meets_all_requirements, validate_article_710, validate_article_715,
    validate_tort_claim,
};
