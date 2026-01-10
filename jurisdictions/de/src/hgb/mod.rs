//! German Commercial Code (Handelsgesetzbuch - HGB)
//!
//! Provides type-safe representations of German commercial law entities:
//! - General Partnership (OHG - Offene Handelsgesellschaft)
//! - Limited Partnership (KG - Kommanditgesellschaft)
//! - GmbH & Co. KG (Hybrid structure)
//! - Merchant status (Kaufmannseigenschaft)
//!
//! # Legal Context
//!
//! The HGB (Handelsgesetzbuch) is Germany's Commercial Code, regulating:
//! - Commercial activities and business entities
//! - Merchant status and obligations
//! - Partnerships and their liability structures
//! - Commercial transactions and business relationships
//!
//! # Covered Areas
//!
//! ## General Partnership (OHG) - §105-160 HGB
//!
//! The OHG is a partnership where **all partners have unlimited personal liability**
//! for partnership debts. Key characteristics:
//!
//! - **Minimum partners**: 2 (§105 Abs. 1 HGB)
//! - **Liability**: Unlimited and joint for all partners (§128 HGB)
//! - **Management**: Each partner has management authority (§114 HGB)
//! - **Representation**: Each partner can represent the partnership (§125 HGB)
//! - **Profit sharing**: Equal unless otherwise agreed (§121 HGB)
//! - **Commercial register**: Registration required (§106 HGB)
//! - **Partnership name**: Must include "OHG" or "offene Handelsgesellschaft" (§19 HGB)
//!
//! ## Limited Partnership (KG) - §161-177a HGB
//!
//! The KG has **two types of partners with different liability**:
//! - **General partners (Komplementäre)**: Unlimited liability
//! - **Limited partners (Kommanditisten)**: Limited to contribution amount
//!
//! Key characteristics:
//!
//! - **Minimum partners**: 1 general partner + 1 limited partner (§161 Abs. 1 HGB)
//! - **General partners**: Unlimited liability like OHG (§161 Abs. 2 HGB)
//! - **Limited partners**: Liability limited to agreed amount (§171 HGB)
//! - **Management**: Only general partners (§164 HGB)
//! - **Commercial register**: Registration required with liability limits (§162 HGB)
//! - **Partnership name**: Must include "KG" or "Kommanditgesellschaft" (§19 HGB)
//!
//! ## GmbH & Co. KG - Hybrid Structure
//!
//! Special form of KG where the **general partner is a GmbH**, combining:
//! - Limited liability of GmbH (shareholders only risk their shares)
//! - Tax advantages of partnership (transparent taxation)
//! - Professional management through GmbH managing directors
//!
//! Key characteristics:
//!
//! - **General partner**: GmbH with minimum €25,000 capital (GmbHG §5)
//! - **Limited partners**: Natural persons or legal entities
//! - **Effective liability**: Limited for all parties (GmbH shareholder limited,
//!   Kommanditisten limited to Haftsumme)
//! - **Management**: Through GmbH managing directors
//! - **Partnership name**: Must include "GmbH & Co. KG" or similar variant
//!
//! # Examples
//!
//! ## OHG Formation Example
//!
//! ```rust
//! use legalis_de::hgb::{OHG, Partner, PartnerType, validate_ohg};
//! use legalis_de::gmbhg::Capital;
//!
//! let ohg = OHG {
//!     partnership_name: "Mustermann & Schmidt OHG".to_string(),
//!     registered_office: "Berlin".to_string(),
//!     business_purpose: "Softwareentwicklung und IT-Beratung".to_string(),
//!     partners: vec![
//!         Partner {
//!             name: "Max Mustermann".to_string(),
//!             address: "Musterstraße 1, 10115 Berlin".to_string(),
//!             contribution: Some(Capital::from_euros(10_000)),
//!             contribution_paid: Some(Capital::from_euros(10_000)),
//!             partner_type: PartnerType::NaturalPerson,
//!             has_management_authority: true,
//!             has_representation_authority: true,
//!         },
//!         Partner {
//!             name: "Erika Schmidt".to_string(),
//!             address: "Beispielweg 5, 20095 Hamburg".to_string(),
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
//! // Validate the OHG
//! match validate_ohg(&ohg) {
//!     Ok(()) => println!("✅ OHG valid: {}", ohg.partnership_name),
//!     Err(e) => println!("❌ Validation failed: {}", e),
//! }
//! ```
//!
//! ## KG Formation Example
//!
//! ```rust
//! use legalis_de::hgb::{KG, Partner, LimitedPartner, PartnerType, validate_kg};
//! use legalis_de::gmbhg::Capital;
//!
//! let kg = KG {
//!     partnership_name: "Tech Ventures KG".to_string(),
//!     registered_office: "München".to_string(),
//!     business_purpose: "IT-Beratung und Softwareentwicklung".to_string(),
//!     general_partners: vec![
//!         Partner {
//!             name: "Max Mustermann".to_string(),
//!             address: "München".to_string(),
//!             contribution: Some(Capital::from_euros(20_000)),
//!             contribution_paid: Some(Capital::from_euros(20_000)),
//!             partner_type: PartnerType::NaturalPerson,
//!             has_management_authority: true,
//!             has_representation_authority: true,
//!         },
//!     ],
//!     limited_partners: vec![
//!         LimitedPartner {
//!             name: "Anna Schmidt".to_string(),
//!             address: "Hamburg".to_string(),
//!             liability_limit: Capital::from_euros(50_000),
//!             contribution_paid: Capital::from_euros(50_000),
//!             partner_type: PartnerType::NaturalPerson,
//!             has_special_representation: false,
//!         },
//!     ],
//!     formation_date: None,
//!     fiscal_year_end: None,
//! };
//!
//! // Validate the KG
//! match validate_kg(&kg) {
//!     Ok(()) => {
//!         println!("✅ KG valid: {}", kg.partnership_name);
//!         println!("   General partners: {}", kg.general_partners.len());
//!         println!("   Limited partners: {}", kg.limited_partners.len());
//!     }
//!     Err(e) => println!("❌ Validation failed: {}", e),
//! }
//! ```
//!
//! ## GmbH & Co. KG Formation Example
//!
//! ```rust
//! use legalis_de::hgb::{GmbHCoKG, GmbHPartner, LimitedPartner, PartnerType, validate_gmbh_co_kg};
//! use legalis_de::gmbhg::Capital;
//!
//! let gmbh_co_kg = GmbHCoKG {
//!     partnership_name: "Verwaltungs GmbH & Co. KG".to_string(),
//!     registered_office: "Berlin".to_string(),
//!     business_purpose: "Vermögensverwaltung und Beteiligungen".to_string(),
//!     gmbh_general_partner: GmbHPartner {
//!         company_name: "Verwaltungs GmbH".to_string(),
//!         registered_office: "Berlin".to_string(),
//!         managing_directors: vec!["Max Mustermann".to_string()],
//!         share_capital: Capital::from_euros(25_000),
//!     },
//!     limited_partners: vec![
//!         LimitedPartner {
//!             name: "Anna Schmidt".to_string(),
//!             address: "Hamburg".to_string(),
//!             liability_limit: Capital::from_euros(100_000),
//!             contribution_paid: Capital::from_euros(100_000),
//!             partner_type: PartnerType::NaturalPerson,
//!             has_special_representation: false,
//!         },
//!         LimitedPartner {
//!             name: "Peter Müller".to_string(),
//!             address: "München".to_string(),
//!             liability_limit: Capital::from_euros(150_000),
//!             contribution_paid: Capital::from_euros(150_000),
//!             partner_type: PartnerType::NaturalPerson,
//!             has_special_representation: false,
//!         },
//!     ],
//!     formation_date: None,
//!     fiscal_year_end: None,
//! };
//!
//! // Validate the GmbH & Co. KG
//! match validate_gmbh_co_kg(&gmbh_co_kg) {
//!     Ok(()) => {
//!         println!("✅ GmbH & Co. KG valid: {}", gmbh_co_kg.partnership_name);
//!         println!("   GmbH partner: {}", gmbh_co_kg.gmbh_general_partner.company_name);
//!         println!("   GmbH capital: €{:.2}", gmbh_co_kg.gmbh_general_partner.share_capital.to_euros());
//!         println!("   Limited partners: {}", gmbh_co_kg.limited_partners.len());
//!     }
//!     Err(e) => println!("❌ Validation failed: {}", e),
//! }
//! ```
//!
//! # Comparison: OHG vs KG vs GmbH & Co. KG
//!
//! | Feature | OHG | KG | GmbH & Co. KG |
//! |---------|-----|----|--------------
//! | General Partners | 2+ (unlimited) | 1+ (unlimited) | 1 GmbH (formally unlimited) |
//! | Limited Partners | None | 1+ (limited) | 1+ (limited) |
//! | Effective Liability | Unlimited for all | Mixed | Limited for all* |
//! | Management | All partners | General partners only | GmbH managing directors |
//! | Taxation | Transparent | Transparent | Transparent |
//! | Formation Cost | Low | Low | Medium (GmbH + KG) |
//! | Complexity | Low | Medium | High |
//!
//! *GmbH & Co. KG achieves effective limited liability because the general
//! partner (GmbH) itself has limited liability, and Kommanditisten are limited.
//!
//! # Validation Functions
//!
//! This module provides comprehensive validation for all partnership types:
//!
//! - [`validate_ohg`] - Validate OHG structure and requirements
//! - [`validate_kg`] - Validate KG structure and requirements
//! - [`validate_gmbh_co_kg`] - Validate GmbH & Co. KG structure and requirements
//! - [`validate_partnership_name`] - Validate partnership name with legal suffix
//! - [`validate_partner`] - Validate individual partner
//! - [`validate_limited_partner`] - Validate limited partner with liability limits
//! - [`validate_business_purpose`] - Validate business purpose
//! - [`validate_registered_office`] - Validate registered office
//! - [`validate_fiscal_year_end`] - Validate fiscal year end date
//!
//! All validation functions return [`Result<()>`] with comprehensive bilingual
//! error messages (German primary, English secondary).

pub mod error;
pub mod types;
pub mod validator;

// Re-exports for convenience
pub use error::{HGBError, Result};
pub use types::{
    FiscalYearEnd, GmbHCoKG, GmbHPartner, KG, LimitedPartner, MerchantType, OHG, Partner,
    PartnerType,
};
pub use validator::{
    validate_business_purpose, validate_fiscal_year_end, validate_gmbh_co_kg, validate_kg,
    validate_limited_partner, validate_ohg, validate_partner, validate_partnership_name,
    validate_registered_office,
};
