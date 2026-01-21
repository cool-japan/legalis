//! Consumer Rights and Protection in the EU
//!
//! This module provides comprehensive modeling of EU consumer protection law:
//! - Consumer Rights Directive 2011/83/EU
//! - Unfair Commercial Practices Directive 2005/29/EC
//!
//! ## Consumer Rights Directive (2011/83/EU)
//!
//! ### Covered Articles
//!
//! - **Article 6**: Information requirements for distance/off-premises contracts
//! - **Articles 9-16**: Right of withdrawal (14 days)
//! - **Article 17**: Exceptions to the right of withdrawal
//!
//! ### Key Concepts
//!
//! **Contract Types**:
//! - **Distance contracts**: Concluded without simultaneous physical presence of trader and consumer
//! - **Off-premises contracts**: Concluded outside trader's business premises
//!
//! **Right of Withdrawal**:
//! Consumers have 14 days to withdraw from distance and off-premises contracts without giving any reason.
//! Period extends to 12 months if trader fails to provide required information.
//!
//! ## Unfair Commercial Practices Directive (2005/29/EC)
//!
//! Protects consumers from unfair business-to-consumer commercial practices across the EU.
//!
//! ### Categories of Unfair Practices
//!
//! 1. **Misleading Actions** (Article 6): False or deceptive information
//! 2. **Misleading Omissions** (Article 7): Hiding or omitting material information
//! 3. **Aggressive Practices** (Articles 8-9): Harassment, coercion, undue influence
//! 4. **Prohibited Practices** (Annex I): 31 practices always considered unfair ("blacklist")
//!
//! ### Average Consumer Standard
//!
//! Practices are assessed from the perspective of the "average consumer" who is:
//! - Reasonably well-informed
//! - Reasonably observant
//! - Reasonably circumspect
//!
//! Special protection for vulnerable groups (children, elderly, disabled).
//!
//! ## Example Usage
//!
//! ### Consumer Rights Directive
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
//!
//! ### Unfair Commercial Practices
//!
//! ```rust,ignore
//! use legalis_eu::consumer_rights::*;
//!
//! // Example: False scarcity claim
//! let practice = UnfairCommercialPractice::ProhibitedPractice(
//!     ProhibitedPractice::FalseScarcity
//! );
//!
//! // This is on the blacklist - always unfair!
//!
//! // Example: Misleading price
//! let misleading = UnfairCommercialPractice::MisleadingAction(
//!     MisleadingAction::FalsePrice {
//!         description: "Showing fake 'original price' that was never charged".to_string(),
//!     }
//! );
//! ```

pub mod error;
pub mod types;
pub mod unfair_practices;
pub mod withdrawal;

// Re-exports
pub use error::ConsumerRightsError;
pub use types::{
    ContractType, DistanceContract, ExceptionToWithdrawal, InformationRequirement,
    OffPremisesContract, WithdrawalException,
};
pub use unfair_practices::{
    AggressivePractice, AverageConsumerStandard, MisleadingAction, MisleadingOmission,
    ProhibitedPractice, TargetAudience, TransactionalDecision, UnfairCommercialPractice,
    UnfairPracticeAssessment,
};
pub use withdrawal::{WithdrawalPeriod, WithdrawalRight};
