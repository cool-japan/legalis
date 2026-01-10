//! French inheritance law (Code civil, Book III - Successions)
//!
//! This module provides structured representations of French inheritance law,
//! including succession rules, wills, reserved portions (réserve héréditaire),
//! and estate distribution.
//!
//! ## Key Features
//!
//! - **Succession rules**: Legal succession order and intestate distribution
//! - **Wills**: Three types (authentic, holographic, mystic)
//! - **Reserved portions**: Compulsory portions for descendants (réserve héréditaire)
//! - **Estate distribution**: Partition rules and debt allocation
//!
//! ## Modules
//!
//! - `types`: Succession, Will, Heir, Estate types
//! - `error`: Inheritance law error types with bilingual messages
//! - `validator`: Validation functions for successions and wills
//! - `succession`: Succession order articles (Articles 720-873)
//! - `will`: Will and testament articles (Articles 774-894)
//! - `reserved_portion`: Reserved portion articles (Articles 912-913, 1493)
//!
//! ## Example
//!
//! ```
//! use legalis_fr::inheritance::{Succession, Person, Heir, Relationship, validate_succession};
//! use chrono::NaiveDate;
//!
//! let deceased = Person::new("Jean Dupont".to_string(), 75);
//! let child = Person::new("Marie Dupont".to_string(), 45);
//!
//! let heir = Heir::new(child, Relationship::Child)
//!     .with_reserved_portion(0.5) // 1 child = 1/2 reserved
//!     .with_actual_share(1.0); // Sole heir receives entire estate
//!
//! let succession = Succession::new(deceased, NaiveDate::from_ymd_opt(2024, 1, 15).unwrap())
//!     .with_last_domicile("Paris, France".to_string())
//!     .with_heir(heir);
//!
//! // Validation will check Article 720 (succession opens at death)
//! assert!(validate_succession(&succession).is_ok());
//! ```

pub mod error;
pub mod reserved_portion;
pub mod succession;
pub mod types;
pub mod validator;
pub mod will;

// Re-export core types
pub use error::{InheritanceLawError, InheritanceLawResult};
pub use types::{
    Asset, Debt, Disposition, DispositionType, Heir, Person, Relationship, ReservedPortion,
    Succession, Will, WillType,
};

// Re-export validation functions
pub use validator::{
    validate_heir_shares, validate_reserved_portion, validate_succession, validate_will,
};

// Re-export succession articles
pub use succession::{article720, article721, article724, article735, article873};

// Re-export will articles
pub use will::{article774_792, article839_851, article893_894};

// Re-export reserved portion articles
pub use reserved_portion::{article912, article913, article1493};
