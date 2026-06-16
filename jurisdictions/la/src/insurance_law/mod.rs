//! Insurance Law Module for Lao PDR (ກົດໝາຍປະກັນໄພ)
//!
//! This module models the **Law on Insurance (Lao PDR), No. 06/NA, 2011**
//! (ກົດໝາຍວ່າດ້ວຍການປະກັນໄພ).
//!
//! # Legal Framework
//!
//! The Law on Insurance establishes the licensing of insurers, the conduct of
//! insurance business, the classes of insurance, the formation and content of
//! insurance contracts, the handling of claims, and the regulation of insurance
//! intermediaries in the Lao People's Democratic Republic. It is administered by
//! the Ministry of Finance.
//!
//! # Key Provisions Modelled
//!
//! - **Classes of insurance** — life, health, motor, property/fire, liability,
//!   marine, agricultural, travel, reinsurance and microinsurance
//!   ([`InsuranceClass`]). Motor third-party liability insurance is compulsory
//!   ([`MOTOR_THIRD_PARTY_COMPULSORY`], [`validate_compulsory_insurance`]).
//! - **Insurer licensing and solvency** — insurers must be licensed by the
//!   Ministry of Finance, hold positive registered capital, and remain solvent
//!   (admitted assets at least equal to liabilities) ([`Insurer`],
//!   [`validate_insurer_license`], [`validate_solvency`],
//!   [`MIN_SOLVENCY_RATIO_PERCENT`]).
//! - **Insurance contracts** — insurable interest, premium, sum insured, policy
//!   duration and the principle of utmost good faith ([`InsurancePolicy`],
//!   [`validate_policy`]).
//! - **Claims and the principle of indemnity** — claims must be notified,
//!   fraudulent claims are rejected, and for indemnity insurance the payout may
//!   not exceed the actual loss or the sum insured ([`InsuranceClaim`],
//!   [`validate_claim`], [`validate_indemnity_principle`]).
//! - **Intermediaries** — insurance agents and brokers must be licensed
//!   ([`Intermediary`], [`validate_intermediary`]).
//!
//! # Legal Accuracy Note
//!
//! Where the precise internal article numbers of the 2011 law cannot be
//! independently verified by this crate, provisions are cited by the law's name
//! and year ([`INSURANCE_LAW_CITATION`]) together with a documented topic
//! descriptor, and quantifiable requirements are encoded as named constants (for
//! example [`MIN_SOLVENCY_RATIO_PERCENT`]) rather than as fabricated article
//! references or unverified monetary thresholds (such as a specific minimum
//! registered capital in LAK).
//!
//! # Example
//!
//! ```
//! use legalis_la::insurance_law::*;
//!
//! let policy = InsurancePolicy {
//!     policyholder: "Somchai".to_string(),
//!     insurance_class: InsuranceClass::Motor,
//!     insurable_interest: true,
//!     sum_insured_lak: 50_000_000,
//!     premium_lak: 1_200_000,
//!     is_indemnity: true,
//!     start_date: "2025-01-01".to_string(),
//!     end_date: "2026-01-01".to_string(),
//!     status: PolicyStatus::Active,
//! };
//!
//! assert!(validate_policy(&policy).is_ok());
//! // Motor third-party liability insurance is compulsory in Lao PDR.
//! assert!(InsuranceClass::Motor.is_compulsory());
//! assert!(validate_compulsory_insurance(InsuranceClass::Motor, true).is_ok());
//! ```

pub mod error;
pub mod types;
pub mod validator;

pub use error::{INSURANCE_LAW_CITATION, InsuranceLawError, InsuranceResult};

pub use types::{
    // Enums (classes, insurer types, statuses, intermediary types)
    ClaimStatus,
    // Constants
    INSURANCE_CLASS_COUNT,
    // Structs (insurers, policies, claims, intermediaries)
    InsuranceClaim,
    InsuranceClass,
    InsurancePolicy,
    Insurer,
    InsurerType,
    Intermediary,
    IntermediaryType,
    MIN_SOLVENCY_RATIO_PERCENT,
    MOTOR_THIRD_PARTY_COMPULSORY,
    PolicyStatus,
};

pub use validator::{
    validate_claim, validate_compulsory_insurance, validate_indemnity_principle,
    validate_insurer_license, validate_intermediary, validate_policy, validate_solvency,
};
