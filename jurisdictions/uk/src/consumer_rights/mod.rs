//! Consumer Rights Act 2015
//!
//! Implementation of UK consumer law covering goods, services, and digital content.
//!
//! # Key Legislation
//!
//! **Consumer Rights Act 2015** consolidates consumer law in the UK, replacing:
//! - Sale of Goods Act 1979
//! - Unfair Terms in Consumer Contracts Regulations 1999
//! - Supply of Goods and Services Act 1982
//!
//! # Structure
//!
//! ## Part 1: Consumer Contracts for Goods, Digital Content and Services
//!
//! ### Chapter 2: Goods (ss.9-18)
//! - **s.9**: Satisfactory quality
//! - **s.10**: Fitness for particular purpose
//! - **s.11**: Goods to be as described
//! - **s.12**: Goods to match sample
//! - **s.13**: Goods to match model
//!
//! ### Chapter 3: Digital Content (ss.34-47)
//! - **s.34**: Satisfactory quality
//! - **s.35**: Fitness for particular purpose
//! - **s.36**: As described
//! - **s.37**: Other pre-contract information
//!
//! ### Chapter 4: Services (ss.49-57)
//! - **s.49**: Reasonable care and skill
//! - **s.50**: Information binding
//! - **s.51**: Reasonable price
//! - **s.52**: Reasonable time
//!
//! ## Part 2: Unfair Terms (ss.61-76)
//! - **s.62**: Unfairness test (good faith, significant imbalance, detriment)
//! - **Schedule 2**: Grey list of potentially unfair terms
//!
//! # Tiered Remedy System for Goods
//!
//! CRA 2015 introduces a three-tier remedy system:
//!
//! ```text
//! Tier 1: Short-term right to reject (30 days)
//! ├─→ Full refund (s.22)
//! └─→ If consumer accepts goods or 30 days pass...
//!
//! Tier 2: Repair or replacement (one attempt)
//! ├─→ Consumer choice: repair or replacement (s.23)
//! ├─→ Must be possible and not disproportionate
//! └─→ If repair/replacement fails...
//!
//! Tier 3: Final remedy
//! ├─→ Price reduction (s.24(5))
//! └─→ Final right to reject (full/partial refund) (s.24(8))
//! ```
//!
//! # Example Usage
//!
//! ```rust,ignore
//! use legalis_uk::consumer_rights::*;
//!
//! // Goods contract
//! let contract = GoodsContract {
//!     description: "Laptop".to_string(),
//!     price_gbp: 500.0,
//!     purchase_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
//!     trader: Trader { /* ... */ },
//!     consumer: Consumer { /* ... */ },
//!     statutory_rights: vec![
//!         GoodsStatutoryRight::SatisfactoryQuality,
//!         GoodsStatutoryRight::FitForPurpose,
//!     ],
//!     remedy_stage: None,
//! };
//!
//! // Validate contract
//! validate_goods_contract(&contract)?;
//!
//! // Check if short-term reject available
//! if contract.can_short_term_reject() {
//!     println!("Within 30 days - full refund available");
//! }
//!
//! // Check satisfactory quality
//! validate_satisfactory_quality(
//!     &contract.description,
//!     "Screen has dead pixels",
//!     contract.price_gbp,
//!     false, // Not satisfactory
//! )?;
//! ```
//!
//! # Legal References
//!
//! - [Consumer Rights Act 2015](https://www.legislation.gov.uk/ukpga/2015/15)
//! - [Citizens Advice Consumer Rights Guide](https://www.citizensadvice.org.uk/consumer/)
//! - [Which? Consumer Rights](https://www.which.co.uk/consumer-rights)

#![allow(missing_docs)]

pub mod error;
pub mod types;
pub mod validator;

// Re-exports
pub use error::{ConsumerRightsError, Result};
pub use types::{
    BreachSeverity, Consumer, ConsumerContract, ConsumerRemedy, ContractBreach, ContractType,
    DigitalContentContract, DigitalContentStatutoryRight, DigitalContentType, FinalRemedyChoice,
    GoodsContract, GoodsStatutoryRight, GreyListItem, RemedyStage, RemedyType, SecondTierChoice,
    ServicesContract, ServicesStatutoryRight, TimeLimit, Trader, UnfairTerm, UnfairTermAssessment,
};
pub use validator::{
    check_short_term_reject_available, validate_as_described, validate_digital_content_contract,
    validate_fit_for_purpose, validate_goods_contract, validate_reasonable_care_and_skill,
    validate_reasonable_time, validate_satisfactory_quality, validate_services_contract,
    validate_unfair_term,
};
