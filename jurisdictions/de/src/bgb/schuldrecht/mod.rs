//! BGB Contract Law (Schuldrecht - Allgemeiner Teil)
//!
//! Provides type-safe representations of German contract law under the BGB
//! (Bürgerliches Gesetzbuch - German Civil Code).
//!
//! # Legal Context
//!
//! The **Schuldrecht** (law of obligations) is Book 2 of the BGB (§§241-853),
//! divided into:
//!
//! - **General Part (Allgemeiner Teil, §§241-432)**: Applies to all obligations
//!   - Formation (Zustandekommen): Offer and acceptance (§§145-157)
//!   - Legal capacity (Geschäftsfähigkeit): §§104-115
//!   - Declaration of intent (Willenserklärung): §§116-144
//!   - Performance (Leistung): §§241-292
//!   - Breach (Pflichtverletzung): §§280-311
//!   - Termination (Rücktritt): §§323-326
//!   - Damages (Schadensersatz): §§249-283
//!
//! - **Special Part (Besonderer Teil, §§433-853)**: Specific contract types
//!   - Sales contract (Kaufvertrag): §§433-479
//!   - Lease (Mietvertrag): §§535-580a
//!   - Service contract (Dienstvertrag): §§611-630
//!   - Work contract (Werkvertrag): §§631-651
//!   - Loan (Darlehen): §§488-505
//!
//! # Core Concepts
//!
//! ## Contract Formation (§§145-157 BGB)
//!
//! Contracts are formed through **offer (Angebot)** and **acceptance (Annahme)**:
//!
//! ### Offer (§145 BGB)
//! - Must be sufficiently specific (essentialia negotii)
//! - Shows intent to be bound
//! - Generally binding upon receipt
//! - Can be revoked before acceptance (§130 Abs. 1 S. 2 BGB)
//!
//! ### Acceptance (§147-150 BGB)
//! - Must be unconditional
//! - Must be timely:
//!   - Present parties: immediate (§147 Abs. 1 BGB)
//!   - Absent parties: reasonable time (§147 Abs. 2 BGB)
//!   - With deadline: before deadline
//! - Late acceptance = new offer (§150 Abs. 1 BGB)
//! - Modified acceptance = rejection + counter-offer (§150 Abs. 2 BGB)
//!
//! ## Legal Capacity (§§104-115 BGB)
//!
//! Three levels determine ability to enter contracts:
//!
//! 1. **Full capacity (voll geschäftsfähig)**: Age 18+, can enter all contracts
//! 2. **Limited capacity (beschränkt geschäftsfähig)**: Age 7-17, requires
//!    representative consent except for:
//!    - Purely beneficial transactions (§107 BGB)
//!    - Spending allowances (§110 BGB - Taschengeldparagraph)
//! 3. **No capacity (geschäftsunfähig)**: Under age 7 or permanently incapacitated
//!    (§104 BGB) - all declarations void
//!
//! ## Breach of Contract (§280 BGB)
//!
//! General damages claim requires:
//! 1. **Schuldverhältnis**: Obligation relationship exists
//! 2. **Pflichtverletzung**: Breach of duty
//! 3. **Verschulden**: Fault (presumed unless debtor proves otherwise)
//! 4. **Schaden**: Damage
//! 5. **Kausalität**: Causation between breach and damage
//!
//! ### Types of Damages Claims
//!
//! - **§280 Abs. 1**: General damages for breach
//! - **§280 Abs. 2 + §286**: Damages for delay (Verzug)
//! - **§281**: Damages in lieu of performance (requires grace period)
//! - **§282**: Damages for breach of duty in performance
//! - **§283**: Damages after impossibility
//! - **§311 Abs. 2**: Culpa in contrahendo (precontractual liability)
//!
//! ## Termination (Rücktritt §§323-326 BGB)
//!
//! Right to rescind contract for non-performance or defective performance:
//!
//! ### Standard Rule (§323 Abs. 1 BGB)
//! - Requires setting grace period (Nachfrist)
//! - Grace period must be reasonable
//! - After grace period expires without performance: termination allowed
//!
//! ### Exceptions - No Grace Period Required (§323 Abs. 2 BGB)
//! 1. Debtor seriously and finally refuses to perform
//! 2. Fixed-date transaction (Fixgeschäft) - deadline expired
//! 3. Special circumstances justify immediate termination
//!
//! ### Limitation (§323 Abs. 5 S. 2 BGB)
//! - Minor breach (unerhebliche Pflichtverletzung) excludes termination
//!
//! # Examples
//!
//! ## Valid Contract Formation
//!
//! ```rust
//! use legalis_de::bgb::schuldrecht::*;
//! use legalis_de::gmbhg::Capital;
//! use chrono::Utc;
//!
//! // Create parties
//! let seller = Party {
//!     name: "Max Mustermann".to_string(),
//!     address: "Berlin".to_string(),
//!     legal_capacity: LegalCapacity::Full,
//!     legal_representative: None,
//!     party_type: PartyType::NaturalPerson,
//! };
//!
//! let buyer = Party {
//!     name: "Erika Schmidt".to_string(),
//!     address: "Munich".to_string(),
//!     legal_capacity: LegalCapacity::Full,
//!     legal_representative: None,
//!     party_type: PartyType::NaturalPerson,
//! };
//!
//! // Create offer
//! let offer = Offer {
//!     offeror: seller.clone(),
//!     offeree: buyer.clone(),
//!     terms: ContractTerms {
//!         subject_matter: "Sale of VW Golf, 2020 model".to_string(),
//!         consideration: Some(Capital::from_euros(15_000)),
//!         essential_terms: vec![
//!             "Car: VW Golf 2020".to_string(),
//!             "Price: €15,000".to_string(),
//!             "Delivery: 2 weeks".to_string(),
//!         ],
//!         additional_terms: vec![],
//!         includes_gtc: false,
//!     },
//!     offered_at: Utc::now(),
//!     acceptance_deadline: Some(Utc::now() + chrono::Duration::days(7)),
//!     binding: true,
//!     revoked: false,
//! };
//!
//! // Validate offer
//! match validate_offer(&offer) {
//!     Ok(()) => println!("✅ Offer valid"),
//!     Err(e) => println!("❌ Offer invalid: {}", e),
//! }
//!
//! // Create acceptance
//! let acceptance = Acceptance {
//!     acceptor: buyer.clone(),
//!     accepted_at: Utc::now() + chrono::Duration::hours(24),
//!     modifications: None,
//!     timely: true,
//! };
//!
//! // Validate contract formation
//! match validate_contract_formation(&offer, &acceptance, false, false) {
//!     Ok(()) => println!("✅ Contract concluded"),
//!     Err(e) => println!("❌ Contract not concluded: {}", e),
//! }
//! ```
//!
//! ## Breach and Damages Claim
//!
//! ```rust
//! use legalis_de::bgb::schuldrecht::*;
//! use legalis_de::gmbhg::Capital;
//! use chrono::Utc;
//!
//! // Create breach
//! let breach = Breach {
//!     contract_id: "C001".to_string(),
//!     breaching_party: "Seller".to_string(),
//!     breach_type: BreachType::NonPerformance,
//!     occurred_at: Utc::now(),
//!     fault: FaultLevel::OrdinaryNegligence,
//!     description: "Failed to deliver car within agreed timeframe".to_string(),
//! };
//!
//! // Validate breach
//! assert!(validate_breach(&breach).is_ok());
//!
//! // Create damages claim
//! let claim = DamagesClaim {
//!     contract_id: Some("C001".to_string()),
//!     claimant: "Buyer".to_string(),
//!     respondent: "Seller".to_string(),
//!     legal_basis: DamagesLegalBasis::GeneralBreach,
//!     damage_types: vec![DamageType::Positive, DamageType::Consequential],
//!     amount_claimed: Capital::from_euros(5_000),
//!     fault_proven: true,
//!     causation_proven: true,
//! };
//!
//! // Validate damages claim
//! match validate_damages_claim(&claim) {
//!     Ok(()) => println!("✅ Damages claim valid"),
//!     Err(e) => println!("❌ Damages claim invalid: {}", e),
//! }
//! ```
//!
//! ## Termination After Grace Period
//!
//! ```rust
//! use legalis_de::bgb::schuldrecht::*;
//! use chrono::Utc;
//!
//! // Set grace period and allow it to expire without performance
//! let termination = Termination {
//!     contract_id: "C001".to_string(),
//!     terminating_party: "Buyer".to_string(),
//!     grounds: TerminationGrounds::NonPerformanceAfterGracePeriod,
//!     grace_period_set_and_expired: true,
//!     declared_at: Utc::now(),
//!     effective: true,
//! };
//!
//! // Validate termination
//! match validate_termination(&termination) {
//!     Ok(()) => println!("✅ Termination valid - contract rescinded"),
//!     Err(e) => println!("❌ Termination invalid: {}", e),
//! }
//! ```

pub mod error;
pub mod types;
pub mod validator;

// Specific contract types (Besonderer Teil)
pub mod lease; // Mietvertrag (§§535-580a BGB)
pub mod sales; // Kaufvertrag (§§433-479 BGB)
pub mod service; // Dienstvertrag (§§611-630 BGB)
pub mod work; // Werkvertrag (§§631-651 BGB)

// Re-exports for convenience
pub use error::{Result, SchuldrechtError};
pub use types::*;
pub use validator::*;
