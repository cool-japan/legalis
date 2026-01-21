//! English Contract Law (Common Law)
//!
//! Implementation of common law contract principles as developed through case law
//! and applied in England & Wales.
//!
//! # Common Law vs Civil Law
//!
//! English contract law is fundamentally different from civil law systems:
//!
//! ```text
//! Common Law (England & Wales)    Civil Law (Germany, France, Japan)
//! ├── Source                      ├── Source
//! │   └── Case law (precedent)    │   └── Codified statutes (BGB, Code Civil)
//! ├── Reasoning                   ├── Reasoning
//! │   └── Inductive (facts→rule)  │   └── Deductive (rule→application)
//! └── Binding force               └── Binding force
//!     └── Stare decisis               └── Legislative text
//! ```
//!
//! # Contract Formation Elements
//!
//! Five essential elements (all must be present):
//!
//! 1. **Offer** - Definite promise to be bound
//!    - Must be distinguished from invitation to treat
//!    - Case: Pharmaceutical Society v Boots \[1953\]
//!
//! 2. **Acceptance** - Unqualified agreement (mirror image rule)
//!    - Must match offer exactly
//!    - Case: Hyde v Wrench \[1840\] - counter-offer destroys offer
//!    - Case: Adams v Lindsell \[1818\] - postal rule
//!
//! 3. **Consideration** - Something of value exchanged
//!    - Must be sufficient (not necessarily adequate)
//!    - Must not be past
//!    - Cases: Chappell v Nestlé \[1960\], Re McArdle \[1951\]
//!
//! 4. **Intention to create legal relations**
//!    - Commercial: Presumed (Esso v Commissioners \[1976\])
//!    - Domestic: Not presumed (Balfour v Balfour \[1919\])
//!
//! 5. **Capacity** - Legal ability to contract
//!    - Minors, mental incapacity, intoxication
//!
//! # Key Case Law
//!
//! ## Formation
//! - **Carlill v Carbolic Smoke Ball Co \[1893\]**: Unilateral contracts
//! - **Adams v Lindsell \[1818\]**: Postal rule
//! - **Hyde v Wrench \[1840\]**: Counter-offer destroys offer
//! - **Pharmaceutical Society v Boots \[1953\]**: Invitation to treat
//!
//! ## Consideration
//! - **Chappell v Nestlé \[1960\]**: Sufficient but need not be adequate
//! - **Re McArdle \[1951\]**: Past consideration invalid
//! - **Tweddle v Atkinson \[1861\]**: Must move from promisee
//! - **Williams v Roffey Bros \[1991\]**: Practical benefit
//!
//! ## Terms
//! - **Poussard v Spiers \[1876\]**: Condition (essential term)
//! - **Bettini v Gye \[1876\]**: Warranty (minor term)
//! - **Hong Kong Fir Shipping v Kawasaki \[1962\]**: Innominate term
//!
//! ## Damages
//! - **Hadley v Baxendale \[1854\]**: Remoteness test (two limbs)
//! - **Robinson v Harman \[1848\]**: Expectation damages
//! - **British Westinghouse v Underground Electric Railways \[1912\]**: Duty to mitigate
//!
//! ## Intention
//! - **Balfour v Balfour \[1919\]**: Domestic agreements
//! - **Esso v Commissioners \[1976\]**: Commercial agreements
//!
//! # Example Usage
//!
//! ```rust,ignore
//! use legalis_uk::contract::*;
//!
//! // Create offer
//! let offer = Offer {
//!     offeror: Party {
//!         name: "Seller".to_string(),
//!         party_type: PartyType::Individual,
//!         age: Some(40),
//!     },
//!     offeree: Party {
//!         name: "Buyer".to_string(),
//!         party_type: PartyType::Individual,
//!         age: Some(30),
//!     },
//!     terms: vec!["Sell car for £5000".to_string()],
//!     offer_date: Utc::now(),
//!     expiry_date: None,
//!     still_open: true,
//!     offer_type: OfferType::Bilateral,
//! };
//!
//! // Validate offer
//! validate_offer(&offer)?;
//!
//! // Accept offer (mirror image rule)
//! let acceptance = Acceptance {
//!     acceptance_date: Utc::now(),
//!     method: AcceptanceMethod::Written,
//!     unqualified: true,
//!     modifications: vec![], // No modifications (mirror image rule)
//! };
//!
//! validate_acceptance(&acceptance, &offer)?;
//! ```
//!
//! # Legal References
//!
//! ## Textbooks
//! - Treitel on The Law of Contract (15th ed, 2020)
//! - Chitty on Contracts (34th ed, 2021)
//! - McKendrick, Contract Law (14th ed, 2022)
//!
//! ## Online Resources
//! - [UK Supreme Court decisions](https://www.supremecourt.uk/decided-cases/)
//! - [BAILII (British and Irish Legal Information Institute)](https://www.bailii.org/)

#![allow(missing_docs)]

pub mod breach_contract;
pub mod error;
pub mod remedies;
pub mod terms;
pub mod types;
pub mod validator;
pub mod vitiating;

// Re-exports from core types
pub use error::{ContractError, Result};
pub use types::{
    Acceptance, AcceptanceMethod, AgreementContext, BreachType, Consideration, ConsiderationType,
    ContractBreach, ContractFormation, ContractRemedy, ContractTerm, ContractualCapacity,
    DamagesType, HadleyLimb, IncapacityType, IntentionPresumption, IntentionToCreateLegalRelations,
    Offer, OfferType, Party, PartyType, RemedyType, RemotenessTest, TermClassification, TermSource,
};
pub use validator::{
    validate_acceptance, validate_breach, validate_capacity, validate_consideration,
    validate_contract_formation, validate_intention, validate_offer, validate_remoteness,
};

// Re-exports from terms module
pub use terms::{
    ApplicableStatute, ContractContext, ExclusionClause, ExclusionClauseValidity, ImplicationTest,
    ImpliedTermAnalysis, IncorporationAnalysis, IncorporationMethod, InterpretationResult,
    InterpretationRule, LiabilityType, ReasonablenessFactor, StatutoryValidity,
    TermSource as TermSourceExpanded, TermType, interpret_exclusion_clause,
    validate_exclusion_clause,
};

// Re-exports from breach_contract module
pub use breach_contract::{
    AffirmationAnalysis, AffirmationRequirements, AnticipatoryBreachAnalysis,
    AnticipatoryBreachType, AvailableRemedy as BreachRemedy, BreachAnalysis, BreachCategory,
    BreachSeverity, BreachType as BreachTypeExpanded, Election, ElectionAnalysis, ElectionManner,
    InnocentPartyOption, RepudiationAnalysis, RepudiationBasis,
};

// Re-exports from remedies module
pub use remedies::{
    AmericanCyanamidFactors, BalanceOfConvenience, BreachNature, DamagesCalculation,
    DamagesMeasure, DeprivationAnalysis, EquitableBar, InjunctionAnalysis, InjunctionType,
    MitigationAnalysis, RemotenessAnalysis, RescissionAnalysis,
    RescissionBar as RemedyRescissionBar, RescissionGround, SpecificPerformanceAnalysis,
    SubjectMatter, TerminationAnalysis,
};

// Re-exports from vitiating module
pub use vitiating::{
    AvailableRemedy as VitiatingRemedy, CommonMistakeCategory, ContractEffect, DuressAnalysis,
    DuressType, EconomicDuressElements, IllegalityAnalysis, IllegalityType, IllegitimacyFactor,
    MisrepresentationAnalysis, MisrepresentationElements, MisrepresentationType, MistakeAnalysis,
    MistakeType, PatelVMirzaFactors, RecognizedRelationship, RescissionBar, ThirdPartyNotice,
    UndueInfluenceAnalysis, UndueInfluenceClass, UnilateralMistakeCategory, VitiatingFactorResult,
    VitiatingFactorType,
};
