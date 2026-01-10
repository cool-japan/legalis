//! German Limited Liability Companies Act (GmbH-Gesetz)
//!
//! Provides type-safe representations of GmbH and UG formation, validation,
//! and managing director appointment under German law.
//!
//! # Legal Context
//!
//! ## GmbH (Gesellschaft mit beschränkter Haftung)
//!
//! The GmbH is Germany's most popular limited liability company form, requiring
//! **€25,000 minimum capital** (§5 GmbHG). Initial contributions must be at least
//! 50% of each share or €12,500 total (§7 Abs. 2 GmbHG).
//!
//! ### Key Requirements
//!
//! - **Capital**: €25,000 minimum (§5 GmbHG)
//! - **Initial Contribution**: 50% or €12,500 minimum (§7 Abs. 2 GmbHG)
//! - **Company Name**: Must include "GmbH" suffix (§4 GmbHG)
//! - **Registered Office**: German city (§4a GmbHG)
//! - **Managing Director**: At least one, natural person with full capacity (§6 GmbHG)
//! - **Notarization**: Articles must be notarially certified (§2 GmbHG)
//!
//! ## UG (Unternehmergesellschaft - haftungsbeschränkt)
//!
//! The UG is a "mini-GmbH" allowing formation with as little as **€1 capital** (§5a GmbHG).
//! UGs must allocate 25% of annual profits to reserves until reaching €25,000,
//! at which point they can convert to a regular GmbH.
//!
//! ### Key Requirements
//!
//! - **Capital**: €1 minimum, €24,999 maximum (§5a GmbHG)
//! - **Reserve Accumulation**: 25% of profits until €25,000 reached
//! - **Company Name**: Must include "UG (haftungsbeschränkt)" or full form
//! - **Full Payment**: Capital must be fully paid immediately (no partial payment)
//!
//! # Module Structure
//!
//! - [`types`]: Core data types (Capital, ArticlesOfAssociation, etc.)
//! - [`error`]: Error types with bilingual messages (German/English)
//! - [`validator`]: Multi-stage validation functions
//!
//! # Examples
//!
//! ## Valid GmbH Formation
//!
//! ```
//! use legalis_de::gmbhg::*;
//! use chrono::Utc;
//!
//! let articles = ArticlesOfAssociation {
//!     company_name: "Tech Solutions GmbH".to_string(),
//!     registered_office: RegisteredOffice {
//!         city: "Berlin".to_string(),
//!         full_address: None,
//!     },
//!     business_purpose: "Software development and IT consulting".to_string(),
//!     share_capital: Capital::from_euros(50_000),
//!     share_structure: vec![
//!         ShareAllocation {
//!             shareholder: Shareholder {
//!                 name: "Max Mustermann".to_string(),
//!                 address: "Berlin, Germany".to_string(),
//!                 shareholder_type: ShareholderType::NaturalPerson,
//!             },
//!             nominal_amount_cents: 5_000_000, // €50,000
//!             contribution_paid_cents: 5_000_000, // 100% paid
//!         },
//!     ],
//!     duration: Some(Duration::Unlimited),
//!     fiscal_year_end: Some(FiscalYearEnd { month: 12, day: 31 }),
//!     formation_date: Some(Utc::now()),
//!     resolution_requirements: None,
//! };
//!
//! // Validate articles
//! let result = validate_articles_of_association(&articles, CompanyType::GmbH);
//! assert!(result.is_ok());
//! ```
//!
//! ## Valid UG Formation
//!
//! ```
//! use legalis_de::gmbhg::*;
//! use chrono::Utc;
//!
//! let articles = ArticlesOfAssociation {
//!     company_name: "Startup UG (haftungsbeschränkt)".to_string(),
//!     registered_office: RegisteredOffice {
//!         city: "Hamburg".to_string(),
//!         full_address: None,
//!     },
//!     business_purpose: "E-commerce services and online retail".to_string(),
//!     share_capital: Capital::from_euros(1), // Minimum €1
//!     share_structure: vec![
//!         ShareAllocation {
//!             shareholder: Shareholder {
//!                 name: "Anna Schmidt".to_string(),
//!                 address: "Hamburg, Germany".to_string(),
//!                 shareholder_type: ShareholderType::NaturalPerson,
//!             },
//!             nominal_amount_cents: 100, // €1
//!             contribution_paid_cents: 100, // Fully paid
//!         },
//!     ],
//!     duration: Some(Duration::Unlimited),
//!     fiscal_year_end: Some(FiscalYearEnd { month: 12, day: 31 }),
//!     formation_date: Some(Utc::now()),
//!     resolution_requirements: None,
//! };
//!
//! // Validate articles
//! let result = validate_articles_of_association(&articles, CompanyType::UG);
//! assert!(result.is_ok());
//! ```
//!
//! ## Managing Director Validation
//!
//! ```
//! use legalis_de::gmbhg::*;
//! use chrono::{NaiveDate, Utc};
//!
//! let directors = ManagingDirectors {
//!     directors: vec![
//!         ManagingDirector {
//!             name: "Max Mustermann".to_string(),
//!             date_of_birth: Some(NaiveDate::from_ymd_opt(1980, 5, 15).unwrap()),
//!             address: "Berlin, Germany".to_string(),
//!             appointment_date: Utc::now(),
//!             representation_authority: RepresentationAuthority::Sole,
//!             has_capacity: true,
//!         },
//!     ],
//! };
//!
//! let result = validate_managing_directors(&directors);
//! assert!(result.is_ok());
//! ```
//!
//! # Legal References
//!
//! - **GmbHG** (GmbH-Gesetz): Limited Liability Companies Act
//! - **§2 GmbHG**: Notarization requirement
//! - **§3 GmbHG**: Articles of association content
//! - **§4 GmbHG**: Company name requirements
//! - **§5 GmbHG**: Capital requirements (€25,000)
//! - **§5a GmbHG**: UG special provisions (€1 minimum, reserve accumulation)
//! - **§6 GmbHG**: Managing director requirements
//! - **§7 GmbHG**: Initial contribution requirements (50% or €12,500)
//! - **§35 GmbHG**: Managing director appointment

pub mod error;
pub mod types;
pub mod validator;

// Re-export all public types for convenience
pub use error::{GmbHError, Result};
pub use types::*;
pub use validator::*;
