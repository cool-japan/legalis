//! Securities Law Module for Lao PDR (ກົດໝາຍຫຼັກຊັບ)
//!
//! This module models the **Law on Securities (Lao PDR), 2012**
//! (ກົດໝາຍວ່າດ້ວຍຫຼັກຊັບ).
//!
//! # Legal Framework
//!
//! The Law on Securities governs the Lao securities (capital) market: the public
//! offering and listing of securities, the licensing of securities companies and
//! intermediaries, disclosure obligations, and prohibited market conduct. The
//! market operator is the Lao Securities Exchange (LSX, ຕະຫຼາດຫຼັກຊັບລາວ), which
//! opened in 2011, and the regulator is the Lao Securities and Exchange Commission
//! (Lao SEC, ຄະນະກຳມະການຄຸ້ມຄອງຫຼັກຊັບ).
//!
//! # Key Provisions Modelled
//!
//! - **Securities** — ordinary and preferred shares, corporate and government
//!   bonds, debentures, warrants and investment-fund units ([`SecurityType`]).
//! - **Public offerings** — an IPO, secondary public offering or public bond issue
//!   requires a prospectus with full, accurate disclosure and Lao SEC approval; a
//!   private placement is exempt ([`PublicOffering`], [`OfferingType`],
//!   [`validate_public_offering`], [`validate_prospectus`]).
//! - **Listing** — minimum public float, continuous disclosure of material
//!   information and current periodic financial reporting ([`ListedCompany`],
//!   [`validate_listing`]).
//! - **Foreign ownership** — a representative cap on foreign holding in a listed
//!   company ([`validate_foreign_ownership`]).
//! - **Securities companies / intermediaries** — broker-dealers, underwriters,
//!   investment advisors, fund managers and custodians must be licensed and
//!   adequately capitalised ([`MarketParticipantType`], [`SecuritiesCompany`],
//!   [`validate_securities_company_license`]).
//! - **Prohibited conduct** — insider trading, market manipulation, fraud and
//!   front-running ([`ProhibitedConduct`], [`SecuritiesTrade`], [`validate_trade`],
//!   [`validate_insider_trading`], [`validate_market_manipulation`]).
//! - **Disclosure** — timely disclosure of material events ([`DisclosureEvent`],
//!   [`validate_disclosure`]).
//!
//! # Numeric thresholds
//!
//! Quantifiable requirements are encoded as named, documented constants
//! ([`MIN_PUBLIC_FLOAT_PERCENT`], [`FOREIGN_OWNERSHIP_LIMIT_PERCENT`],
//! [`MATERIAL_DISCLOSURE_DEADLINE_DAYS`]). Several are representative regulatory
//! thresholds used as modelling defaults and are documented as such.
//!
//! # Legal Accuracy Note
//!
//! Where the precise internal article numbers of the 2012 law cannot be
//! independently verified by this crate, provisions are cited by the law's name
//! and year ([`SECURITIES_LAW_CITATION`]) together with a documented topic
//! descriptor, and quantifiable requirements are encoded as named constants rather
//! than as fabricated article references.
//!
//! # Example
//!
//! ```rust
//! use legalis_la::securities_law::*;
//!
//! // A compliant IPO: prospectus filed, complete, and approved by the Lao SEC.
//! let ipo = PublicOffering {
//!     issuer: "Lao Brewery Co.".to_string(),
//!     offering_type: OfferingType::Ipo,
//!     has_prospectus: true,
//!     prospectus_complete: true,
//!     sec_approved: true,
//!     total_value_lak: 50_000_000_000,
//! };
//! assert!(validate_public_offering(&ipo).is_ok());
//!
//! // Insider trading is prohibited market conduct.
//! let trade = SecuritiesTrade {
//!     security: SecurityType::OrdinaryShares,
//!     used_inside_information: true,
//!     manipulative: false,
//! };
//! assert!(validate_trade(&trade).is_err());
//! ```

pub mod error;
pub mod types;
pub mod validator;

pub use error::{SECURITIES_LAW_CITATION, SecuritiesLawError, SecuritiesResult};

pub use types::{
    // Disclosure & trading
    DisclosureEvent,
    // Constants
    FOREIGN_OWNERSHIP_LIMIT_PERCENT,
    // Listed companies
    ListedCompany,
    ListingStatus,
    MATERIAL_DISCLOSURE_DEADLINE_DAYS,
    MIN_PUBLIC_FLOAT_PERCENT,
    // Market participants
    MarketParticipantType,
    OfferingType,
    // Prohibited conduct
    ProhibitedConduct,
    // Public offerings
    PublicOffering,
    SECURITY_TYPE_COUNT,
    SecuritiesCompany,
    SecuritiesTrade,
    // Securities
    SecurityType,
};

pub use validator::{
    validate_disclosure, validate_foreign_ownership, validate_insider_trading, validate_listing,
    validate_market_manipulation, validate_prospectus, validate_public_offering,
    validate_securities_company_license, validate_trade,
};
