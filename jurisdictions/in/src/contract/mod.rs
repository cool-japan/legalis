//! Indian Contract Act 1872 Module
//!
//! # Contract Law in India
//!
//! This module implements the Indian Contract Act, 1872, which governs
//! the formation, performance, and enforcement of contracts in India.
//!
//! ## Historical Background
//!
//! - Enacted: April 25, 1872
//! - Based on: English common law with Indian modifications
//! - Amended: Multiple times, most recently to update e-commerce provisions
//!
//! ## Essential Elements of Valid Contract (Section 10)
//!
//! For an agreement to be a contract, it must have:
//!
//! 1. **Offer and Acceptance** - Sections 2(a), 2(b)
//! 2. **Free Consent** - Sections 13-14
//! 3. **Competent Parties** - Section 11
//! 4. **Lawful Consideration** - Section 23
//! 5. **Lawful Object** - Section 23
//! 6. **Not Expressly Declared Void** - Sections 24-30
//!
//! ## Free Consent (Section 14)
//!
//! Consent is said to be free when not caused by:
//!
//! | Factor | Section | Effect |
//! |--------|---------|--------|
//! | Coercion | 15 | Voidable |
//! | Undue Influence | 16 | Voidable |
//! | Fraud | 17 | Voidable |
//! | Misrepresentation | 18 | Voidable |
//! | Mistake of Fact | 20 | Void (if bilateral) |
//!
//! ## Competency to Contract (Section 11)
//!
//! The following persons are incompetent:
//! - **Minors**: Below 18 years (Majority Act, 1875)
//! - **Persons of unsound mind**: During periods of unsoundness (Section 12)
//! - **Persons disqualified by law**: Alien enemies, insolvents, etc.
//!
//! ## Minor's Agreements
//!
//! **Leading Case**: Mohori Bibee v. Dharmodas Ghose (1903)
//! - Agreement with a minor is void ab initio
//! - Minor cannot ratify upon attaining majority
//! - Doctrine of restitution may apply
//!
//! ## Void Agreements (Sections 24-30)
//!
//! | Type | Section | Exception |
//! |------|---------|-----------|
//! | Without consideration | 25 | Natural love, past service, time-barred debt |
//! | Restraint of marriage | 26 | None |
//! | Restraint of trade | 27 | Sale of goodwill, partnership |
//! | Restraint of legal proceedings | 28 | Arbitration agreements |
//! | Uncertain agreements | 29 | None |
//! | Wagering agreements | 30 | Horse racing, skill-based competitions |
//!
//! ## Performance of Contracts (Sections 37-67)
//!
//! - **Section 37**: Parties must perform or offer to perform
//! - **Section 46**: Time and place of performance
//! - **Section 55**: Time as essence of contract
//! - **Section 56**: Supervening impossibility (Frustration)
//!
//! ## Remedies for Breach (Sections 73-75)
//!
//! | Remedy | Section | Description |
//! |--------|---------|-------------|
//! | Damages | 73 | Compensation for loss arising naturally |
//! | Liquidated Damages | 74 | Reasonable compensation up to stipulated sum |
//! | Penalty | 74 | Court awards reasonable compensation only |
//! | Specific Performance | - | Specific Relief Act, 1963 |
//! | Injunction | - | Specific Relief Act, 1963 |
//!
//! ## Section 73: Damages
//!
//! ```text
//! When a contract has been broken, the party who suffers by such breach
//! is entitled to receive, from the party who has broken the contract,
//! compensation for any loss or damage caused to him thereby, which
//! naturally arose in the usual course of things from such breach, or
//! which the parties knew, when they made the contract, to be likely
//! to result from the breach of it.
//! ```
//!
//! ## Section 74: Liquidated Damages vs Penalty
//!
//! - India does not distinguish between penalty and liquidated damages
//! - Court awards reasonable compensation, not exceeding stipulated sum
//! - **ONGC v. Saw Pipes (2003)**: Breach must be proven; actual loss need not be
//!
//! ## Quasi-Contracts (Sections 68-72)
//!
//! | Section | Situation |
//! |---------|-----------|
//! | 68 | Necessaries supplied to incapable person |
//! | 69 | Payment of money due by another |
//! | 70 | Non-gratuitous act enjoyed by another |
//! | 71 | Finder of goods |
//! | 72 | Money paid by mistake or under coercion |
//!
//! ## Agency (Chapter X, Sections 182-238)
//!
//! - **Section 182**: Definition of agent and principal
//! - **Section 187**: Extent of agent's authority
//! - **Section 196**: Ratification of unauthorized acts
//! - **Section 211**: Agent's duty to principal
//!
//! ## Contingent Contracts (Sections 31-36)
//!
//! - **Section 31**: Definition
//! - **Section 32**: Enforcement (on happening of event)
//! - **Section 33**: Enforcement (on not happening of event)
//! - **Section 36**: Void if event becomes impossible
//!
//! ## Example: Validate Contract
//!
//! ```rust
//! use legalis_in::contract::*;
//! use chrono::NaiveDate;
//!
//! // Check consent validity
//! let result = validate_consent(&[ConsentVitiator::Coercion]);
//! assert!(result.is_err()); // Voidable due to coercion
//!
//! // Calculate damages
//! let damages = calculate_damages(100000.0, 50000.0, 20000.0, true);
//! assert_eq!(damages, 70000.0); // Ordinary + Special (if known)
//! ```
//!
//! ## References
//!
//! - [Indian Contract Act, 1872](https://legislative.gov.in/actsofparliamentfromtheyear/indian-contract-act-1872)
//! - [Specific Relief Act, 1963](https://legislative.gov.in/actsofparliamentfromtheyear/specific-relief-act-1963)
//! - [Limitation Act, 1963](https://legislative.gov.in/actsofparliamentfromtheyear/limitation-act-1963)

#![allow(missing_docs)]

pub mod error;
pub mod types;
pub mod validator;

pub use error::*;
pub use types::*;
pub use validator::*;
