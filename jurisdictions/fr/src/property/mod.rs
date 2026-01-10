//! French property law (Code civil, Book II-III)
//!
//! This module provides structured representations of French property law,
//! including ownership rights, easements, and real estate transactions.
//!
//! ## Key Features
//!
//! - **Ownership rights**: Absolute ownership and limitations (Articles 544-548)
//! - **Easements**: Legal and conventional servitudes (Articles 640-709)
//! - **Real estate transactions**: Sales, leases, and formalities
//!
//! ## Modules
//!
//! - `types`: Property, Easement, Encumbrance types
//! - `error`: Property law error types with bilingual messages
//! - `validator`: Validation functions for property transactions
//! - `ownership`: Ownership rights articles (Articles 544-572)
//! - `easements`: Easement and servitude articles (Articles 640-734)
//! - `transactions`: Real estate transaction articles
//!
//! ## Example
//!
//! ```
//! use legalis_fr::property::{Property, PropertyType, Easement, EasementType};
//!
//! let property = Property::new(
//!     PropertyType::Immovable {
//!         land_area: 1000.0,
//!         building_area: Some(200.0),
//!     },
//!     "Jean Dupont".to_string(),
//!     "15 rue de Paris, 75001 Paris".to_string(),
//!     500_000,
//! );
//!
//! let easement = Easement::new(
//!     EasementType::RightOfWay,
//!     "Property at 15 rue de Paris".to_string(),
//! ).with_description("Access to public road".to_string());
//! ```

pub mod easements;
pub mod error;
pub mod ownership;
pub mod transactions;
pub mod types;
pub mod validator;

// Re-export core types
pub use error::{PropertyLawError, PropertyLawResult};
pub use types::{
    Asset, AssetType, Easement, EasementType, Encumbrance, EncumbranceType, Property, PropertyType,
};

// Re-export validation functions
pub use validator::{
    validate_easement, validate_ownership, validate_property, validate_transaction,
};

// Re-export ownership articles
pub use ownership::{article544, article545, article546, article548, article571_572};

// Re-export easement articles
pub use easements::{article555, article640_649, article667_709, article710_734, article713};

// Re-export transaction articles
pub use transactions::{article490, article1741_1749, article1873_1878};
