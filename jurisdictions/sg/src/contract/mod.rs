//! Contract Law (Singapore common law)
//!
//! Type-safe modelling of the Singapore common law of contract. Singapore is a
//! common-law jurisdiction whose contract law derives from English law and
//! continues to be developed by the Singapore courts. This module encodes the
//! core doctrines with their leading authorities, allowing programmatic analysis
//! of formation, breach, vitiating factors, discharge, and remedies.
//!
//! ## Doctrines covered
//!
//! ### 1. Formation
//!
//! A binding contract requires four things (*Gay Choon Ing v Loh Sze Ti Terence
//! Peter* \[2009\] SGCA 3):
//!
//! 1. **Offer** — distinguished from an invitation to treat (*Pharmaceutical
//!    Society of Great Britain v Boots* \[1953\] 1 QB 401). An offer may lapse,
//!    be revoked (*Byrne v Van Tienhoven* (1880) 5 CPD 344) or be destroyed by a
//!    counter-offer (*Hyde v Wrench* (1840) 3 Beav 334).
//! 2. **Acceptance** — an unqualified assent communicated to the offeror; the
//!    postal rule applies to acceptance by post (*Adams v Lindsell* (1818)),
//!    while instantaneous and (generally) electronic acceptances take effect on
//!    receipt (*Entores v Miles Far East* \[1955\] 2 QB 327; Electronic
//!    Transactions Act 2010).
//! 3. **Consideration** — must be sufficient but need not be adequate
//!    (*Chappell v Nestlé* \[1960\] AC 87); past consideration is generally bad
//!    (*Roscorla v Thomas*); performance of an existing duty requires a
//!    practical benefit (*Williams v Roffey Bros* \[1991\] 1 QB 1).
//! 4. **Intention to create legal relations** — presumed in commercial dealings,
//!    presumed absent in social/domestic ones (*Balfour v Balfour* \[1919\] 2 KB
//!    571; cf *Merritt v Merritt*).
//!
//! ### 2. Terms
//!
//! Terms are classified as **conditions**, **warranties**, or **innominate
//! terms**; the classification fixes the remedy for breach (*Hongkong Fir
//! Shipping v Kawasaki Kisen Kaisha* \[1962\] 2 QB 26; *RDC Concrete v Sato
//! Kogyo* \[2007\] SGCA 1).
//!
//! ### 3. Vitiating factors
//!
//! - **Misrepresentation** — fraudulent (*Derry v Peek*), negligent
//!   (Misrepresentation Act 1967 s. 2(1)) and innocent (s. 2(2)).
//! - **Mistake** — common (*Great Peace Shipping*), mutual (*Raffles v
//!   Wichelhaus*) and unilateral (*Chwee Kin Keong v Digilandmall.com* \[2005\]
//!   SGCA 2).
//! - **Duress** — including economic duress (*The Universe Sentinel*;
//!   *E C Investment Holding v Ridout Residence* \[2011\] SGHC 231).
//! - **Undue influence** — actual and presumed (*RBS v Etridge (No 2)*;
//!   *BOM v BOK* \[2018\] SGCA 83).
//!
//! ### 4. Discharge
//!
//! By performance, agreement, breach, or **frustration** (*Davis Contractors v
//! Fareham UDC* \[1956\] AC 696; consequences under the Frustrated Contracts Act
//! 1959).
//!
//! ### 5. Remedies
//!
//! - **Damages** on the expectation measure (*Robinson v Harman*), limited by
//!   **remoteness** (*Hadley v Baxendale* (1854) 9 Exch 341; *Robertson Quay
//!   Investment v Steen Consultants* \[2008\] SGCA 8) and **mitigation**
//!   (*British Westinghouse* \[1912\] AC 673).
//! - **Specific performance** — equitable and discretionary, available only
//!   where damages are inadequate.
//! - **Termination** — election following a repudiatory breach.
//!
//! ## Example
//!
//! ```rust
//! use legalis_sg::contract::*;
//!
//! // Build a commercial contract.
//! let offer = Offer::new("o1", "Seller Pte Ltd", "Buyer Pte Ltd", "sale of CNC machine");
//! let mut contract = Contract::new("k1", offer, AgreementContext::Commercial)
//!     .with_acceptance(Acceptance::new("o1", "Buyer Pte Ltd", AcceptanceMode::Electronic));
//! contract.add_consideration(Consideration::promise("Seller Pte Ltd", "deliver machine"));
//! contract.add_consideration(Consideration::promise("Buyer Pte Ltd", "pay SGD 80,000"));
//!
//! assert!(validate_formation(&contract).is_ok());
//!
//! // Classify a breach of a condition.
//! let term = ContractTerm::new("t1", "machine must be new", TermClassification::Condition);
//! assert!(classify_breach(&term, false).may_terminate);
//!
//! // Assess damages with Hadley v Baxendale remoteness.
//! let heads = vec![
//!     HeadOfLoss::ordinary("cost of replacement", 800_000),
//!     HeadOfLoss::special("lost downstream tender", 5_000_000, false), // not communicated
//! ];
//! let award = assess_damages(DamagesMeasure::Expectation, &heads).unwrap();
//! assert_eq!(award.recoverable_cents, 800_000); // remote head excluded
//! ```
//!
//! ## Authorities referenced
//!
//! - Gay Choon Ing v Loh Sze Ti Terence Peter \[2009\] SGCA 3
//! - Hongkong Fir Shipping v Kawasaki Kisen Kaisha \[1962\] 2 QB 26
//! - RDC Concrete v Sato Kogyo \[2007\] SGCA 1
//! - Chwee Kin Keong v Digilandmall.com \[2005\] SGCA 2
//! - Hadley v Baxendale (1854) 9 Exch 341
//! - Robertson Quay Investment v Steen Consultants \[2008\] SGCA 8
//! - Misrepresentation Act 1967; Frustrated Contracts Act 1959
//!
//! ## Submodules
//!
//! - [`types`] — formation, terms, vitiating factors and discharge models.
//! - [`remedies`] — damages, remoteness, mitigation and specific performance.
//! - [`validator`] — analysis functions and the validation report.
//! - [`error`] — [`error::ContractError`] with attributed authorities.

pub mod error;
pub mod remedies;
pub mod types;
pub mod validator;

pub use error::{ContractError, Result};
pub use remedies::{
    DamagesAward, DamagesMeasure, HeadOfLoss, RemotenessLimb, SpecificPerformanceFactors,
};
pub use types::{
    Acceptance, AcceptanceMode, AgreementContext, Consideration, ConsiderationKind, Contract,
    ContractTerm, DischargeMode, DuressClaim, DuressKind, FrustratingEvent, Misrepresentation,
    MisrepresentationCategory, MistakeKind, Offer, OfferStatus, OperativeMistake,
    TermClassification, TermSource, UndueInfluenceClaim, UndueInfluenceClass,
};
pub use validator::{
    BreachConsequence, ContractValidationReport, analyse_contract, assess_damages, assess_duress,
    assess_frustration, assess_misrepresentation, assess_mistake, assess_specific_performance,
    assess_undue_influence, classify_breach, is_formed, require_termination_right,
    validate_formation,
};
