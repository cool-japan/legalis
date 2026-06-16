//! Intellectual Property Law Module for Lao PDR (ກົດໝາຍຊັບສິນທາງປັນຍາ)
//!
//! This module models the **Law on Intellectual Property (Lao PDR), No. 38/NA,
//! 2017** (ກົດໝາຍວ່າດ້ວຍຊັບສິນທາງປັນຍາ), the consolidated/amended IP Law
//! (originally No. 01/NA 2011, amended 2017).
//!
//! # Legal Framework
//!
//! The consolidated Law on Intellectual Property governs the full spectrum of IP
//! rights in the Lao People's Democratic Republic: patents (inventions), petty
//! patents (utility/minor innovations), industrial designs, trademarks/marks,
//! trade names, geographical indications, copyright and related rights, trade
//! secrets (undisclosed information), layout-designs of integrated circuits, new
//! plant varieties, and traditional knowledge. As a WTO member, Lao PDR
//! implements the minimum standards of the TRIPS Agreement, and it is a party to
//! the Paris and Berne Conventions and the Patent Cooperation Treaty (PCT).
//!
//! # Key Provisions Modelled
//!
//! - **IP right categories** — the twelve categories of protected right, each
//!   with its fixed term where one applies ([`IpRightType`]).
//! - **Patents** — novelty, inventive step and industrial applicability, plus
//!   the 20-year term ([`PatentApplication`], [`validate_patentability`],
//!   [`validate_patent_term`]).
//! - **Trademarks** — distinctiveness, non-deception and absence of conflict,
//!   plus the renewable 10-year term ([`TrademarkRegistration`],
//!   [`validate_trademark_registrability`], [`validate_trademark_renewal`]).
//! - **Copyright** — subsistence in original works and the life-plus-50-years
//!   term ([`CopyrightWork`], [`validate_copyright`]).
//! - **Trade secrets** — secrecy, commercial value and reasonable steps
//!   ([`TradeSecret`], [`validate_trade_secret`]).
//! - **Industrial designs, geographical indications and new plant varieties**
//!   ([`IndustrialDesign`], [`GeographicalIndication`], [`PlantVariety`]).
//! - **Infringement** — unauthorised use of a protected right
//!   ([`IpInfringement`], [`validate_infringement`]).
//!
//! # Legal Accuracy Note
//!
//! Where the precise internal article numbers of the 2017 law cannot be
//! independently verified by this crate, provisions are cited by the law's name
//! and year ([`IP_LAW_CITATION`]) together with a documented topic descriptor,
//! and the quantifiable protection terms are encoded as named constants whose
//! values are the well-established TRIPS/Berne minima implemented by the law (for
//! example [`PATENT_TERM_YEARS`] and [`COPYRIGHT_TERM_AFTER_DEATH_YEARS`]) rather
//! than as fabricated article references.
//!
//! # Example
//!
//! ```
//! use legalis_la::intellectual_property_law::*;
//!
//! let application = PatentApplication {
//!     title: "Solar-powered irrigation pump".to_string(),
//!     is_novel: true,
//!     has_inventive_step: true,
//!     is_industrially_applicable: true,
//!     filing_year: 2020,
//! };
//!
//! assert!(validate_patentability(&application).is_ok());
//! assert_eq!(IpRightType::Patent.protection_term_years(), Some(PATENT_TERM_YEARS));
//! ```

pub mod error;
pub mod types;
pub mod validator;

pub use error::{IP_LAW_CITATION, IpLawError, IpResult};

pub use types::{
    // Constants
    COPYRIGHT_TERM_AFTER_DEATH_YEARS,
    // Copyright
    CopyrightWork,
    // Geographical indications
    GeographicalIndication,
    INDUSTRIAL_DESIGN_TERM_YEARS,
    IP_RIGHT_TYPE_COUNT,
    // Industrial designs
    IndustrialDesign,
    // Status & categories
    IpApplicationStatus,
    // Infringement
    IpInfringement,
    IpRightType,
    LAYOUT_DESIGN_TERM_YEARS,
    PATENT_TERM_YEARS,
    PETTY_PATENT_TERM_YEARS,
    PLANT_VARIETY_TERM_YEARS,
    // Patents
    PatentApplication,
    // Plant varieties
    PlantVariety,
    RegistrationStatus,
    TRADEMARK_TERM_YEARS,
    // Trade secrets
    TradeSecret,
    // Trademarks
    TrademarkRegistration,
};

pub use validator::{
    validate_copyright, validate_geographical_indication, validate_industrial_design,
    validate_infringement, validate_patent_term, validate_patentability, validate_plant_variety,
    validate_trade_secret, validate_trademark_registrability, validate_trademark_renewal,
};
