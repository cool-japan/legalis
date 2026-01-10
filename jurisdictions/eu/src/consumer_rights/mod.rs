//! Consumer Rights Directive 2011/83/EU Implementation
//!
//! This module provides comprehensive modeling of the Consumer Rights Directive,
//! the EU's primary consumer protection law for distance and off-premises contracts.
//!
//! ## Covered Articles
//!
//! - **Article 6**: Information requirements for distance/off-premises contracts
//! - **Articles 9-16**: Right of withdrawal (14 days)
//! - **Article 17**: Exceptions to the right of withdrawal
//!
//! ## Key Concepts
//!
//! ### Contract Types
//!
//! - **Distance contracts**: Concluded without simultaneous physical presence of trader and consumer
//! - **Off-premises contracts**: Concluded outside trader's business premises
//!
//! ### Right of Withdrawal
//!
//! Consumers have 14 days to withdraw from distance and off-premises contracts without giving any reason.
//! Period extends to 12 months if trader fails to provide required information.
//!
//! ## Example Usage
//!
//! ```rust,ignore
//! use legalis_eu::consumer_rights::*;
//! use chrono::Utc;
//!
//! // Create a distance contract
//! let contract = DistanceContract::new()
//!     .with_trader("Online Shop Ltd")
//!     .with_consumer("John Doe")
//!     .with_contract_date(Utc::now())
//!     .with_goods_description("Laptop computer")
//!     .with_price_eur(999.0);
//!
//! // Check withdrawal right
//! match contract.calculate_withdrawal_period() {
//!     Ok(period) => {
//!         println!("Withdrawal deadline: {}", period.deadline);
//!         println!("Days remaining: {}", period.days_remaining);
//!     }
//!     Err(e) => println!("Error: {}", e),
//! }
//! ```

pub mod error;
pub mod types;
pub mod withdrawal;

// Re-exports
pub use error::ConsumerRightsError;
pub use types::{
    ContractType, DistanceContract, ExceptionToWithdrawal, InformationRequirement,
    OffPremisesContract, WithdrawalException,
};
pub use withdrawal::{WithdrawalPeriod, WithdrawalRight};
