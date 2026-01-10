//! German jurisdiction support for Legalis-RS.
//!
//! This crate provides structured representations of German law, including:
//! - **GmbHG** (GmbH-Gesetz) - Limited Liability Companies Act
//! - **HGB** (Handelsgesetzbuch) - Commercial Code
//! - **AktG** (Aktiengesetz) - Stock Corporation Act
//! - **BGB Schuldrecht** (Bürgerliches Gesetzbuch) - Contract Law [v0.3.0 Phase 4]
//! - **BGB Tort Law** (Unerlaubte Handlungen) - §§ 823, 826 [v0.3.0 Phase 5]
//! - **BGB Sachenrecht** (Property Law) - Movables, Immovables, Mortgages [v0.3.0 Phase 6]
//! - **BGB Familienrecht** (Family Law) - Marriage, Divorce, Custody [v0.3.0 Phase 7]
//! - **BGB Erbrecht** (Succession Law) - Wills, Legal Succession, Compulsory Portion [v0.3.0 Phase 8]
//! - **GG** (Grundgesetz) - German Basic Law/Constitution [NEW in v0.4.0 Phase 9]
//! - **Arbeitsrecht** (Labor Law) - Individual & Collective Labor Law \[v0.5.0 Phases 10-12\]
//! - **StGB** (Strafgesetzbuch) - German Criminal Code (planned for future)
//!
//! # GmbHG - Company Law
//!
//! Comprehensive support for GmbH (Gesellschaft mit beschränkter Haftung) and
//! UG (Unternehmergesellschaft - haftungsbeschränkt) formation and validation.
//!
//! ## Quick Start - GmbH
//!
//! ```
//! use legalis_de::gmbhg::*;
//!
//! // Create GmbH with €50,000 capital
//! let capital = Capital::from_euros(50_000);
//! assert!(capital.is_valid_for_gmbh());
//!
//! // Validate capital for company type
//! let result = validate_capital(&capital, CompanyType::GmbH);
//! assert!(result.is_ok());
//! ```
//!
//! See [`gmbhg`] module documentation for detailed examples.
//!
//! # HGB - Commercial Code (Partnerships)
//!
//! Support for German partnerships including OHG (General Partnership), KG
//! (Limited Partnership), and GmbH & Co. KG (Hybrid structure).
//!
//! ## Quick Start - OHG
//!
//! ```
//! use legalis_de::hgb::*;
//! use legalis_de::gmbhg::Capital;
//!
//! // Create OHG (General Partnership)
//! let ohg = OHG {
//!     partnership_name: "Mustermann & Schmidt OHG".to_string(),
//!     registered_office: "Berlin".to_string(),
//!     business_purpose: "Softwareentwicklung".to_string(),
//!     partners: vec![
//!         Partner {
//!             name: "Max Mustermann".to_string(),
//!             address: "Berlin".to_string(),
//!             contribution: Some(Capital::from_euros(10_000)),
//!             contribution_paid: Some(Capital::from_euros(10_000)),
//!             partner_type: PartnerType::NaturalPerson,
//!             has_management_authority: true,
//!             has_representation_authority: true,
//!         },
//!         Partner {
//!             name: "Erika Schmidt".to_string(),
//!             address: "Hamburg".to_string(),
//!             contribution: Some(Capital::from_euros(10_000)),
//!             contribution_paid: Some(Capital::from_euros(10_000)),
//!             partner_type: PartnerType::NaturalPerson,
//!             has_management_authority: true,
//!             has_representation_authority: true,
//!         },
//!     ],
//!     formation_date: None,
//!     fiscal_year_end: None,
//!     unlimited_liability: true,
//! };
//!
//! // Validate OHG
//! let result = validate_ohg(&ohg);
//! assert!(result.is_ok());
//! ```
//!
//! See [`hgb`] module documentation for more partnership types and examples.
//!
//! # BGB - Contract Law (Schuldrecht)
//!
//! Comprehensive contract law support including formation, breach, and remedies.
//!
//! ## Quick Start - Contract Formation
//!
//! ```
//! use legalis_de::bgb::schuldrecht::*;
//! use legalis_de::gmbhg::Capital;
//! use chrono::Utc;
//!
//! // Create parties with full legal capacity
//! let seller = Party {
//!     name: "Max Mustermann".to_string(),
//!     address: "Berlin".to_string(),
//!     legal_capacity: LegalCapacity::Full,
//!     legal_representative: None,
//!     party_type: PartyType::NaturalPerson,
//! };
//!
//! // Create offer
//! let offer = Offer {
//!     offeror: seller.clone(),
//!     offeree: Party {
//!         name: "Erika Schmidt".to_string(),
//!         address: "Munich".to_string(),
//!         legal_capacity: LegalCapacity::Full,
//!         legal_representative: None,
//!         party_type: PartyType::NaturalPerson,
//!     },
//!     terms: ContractTerms {
//!         subject_matter: "Sale of car".to_string(),
//!         consideration: Some(Capital::from_euros(10_000)),
//!         essential_terms: vec!["Car: VW Golf".to_string()],
//!         additional_terms: vec![],
//!         includes_gtc: false,
//!     },
//!     offered_at: Utc::now(),
//!     acceptance_deadline: Some(Utc::now() + chrono::Duration::days(7)),
//!     binding: true,
//!     revoked: false,
//! };
//!
//! // Validate offer
//! let result = validate_offer(&offer);
//! assert!(result.is_ok());
//! ```
//!
//! See [`bgb::schuldrecht`] module documentation for detailed examples.

pub mod aktg;
pub mod arbeitsrecht;
pub mod bgb;
pub mod gmbhg;
pub mod grundgesetz;
pub mod hgb;

// BGB exports (tort law)
pub use bgb::{bgb_823_1, bgb_823_2, bgb_826};

// BGB Schuldrecht exports (contract law)
pub use bgb::schuldrecht;

// BGB Familienrecht exports (family law)
pub use bgb::familienrecht;

// BGB Erbrecht exports (succession law)
pub use bgb::erbrecht;

// GmbHG exports (company law)
pub use gmbhg::{
    ArticlesOfAssociation, Capital, CompanyType, Duration, FiscalYearEnd, GmbHError,
    ManagingDirector, ManagingDirectors, RegisteredOffice, RepresentationAuthority, Result,
    ShareAllocation, Shareholder, ShareholderType, validate_articles_of_association,
    validate_capital, validate_company_name, validate_managing_directors,
};

// HGB exports (partnerships)
pub use hgb::{
    GmbHCoKG, GmbHPartner, HGBError, KG, LimitedPartner, MerchantType, OHG, Partner, PartnerType,
    validate_gmbh_co_kg, validate_kg, validate_ohg,
};
