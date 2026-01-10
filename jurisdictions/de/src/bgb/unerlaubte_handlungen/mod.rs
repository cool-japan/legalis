//! BGB Tort Law (Unerlaubte Handlungen)
//!
//! Comprehensive tort law module with builder patterns, damage calculation,
//! and causation analysis for German civil law (§§823-853 BGB).
//!
//! # Legal Context
//!
//! **Unerlaubte Handlungen** (torts/delicts) are covered in the BGB §§823-853.
//! German tort law follows a **numerus clausus** approach in §823 Abs. 1,
//! enumerating specific protected interests, contrasted with the general clause
//! in §826 for intentional damage contrary to public policy.
//!
//! # Core Provisions
//!
//! ## §823 Abs. 1 BGB - General Tort Liability
//!
//! **Requirements**:
//! 1. **Protected Interest Violated**: Life, Body, Health, Freedom, Property, or Other Rights
//! 2. **Fault (Verschulden)**: Intent (Vorsatz) OR Negligence (Fahrlässigkeit)
//! 3. **Unlawfulness (Widerrechtlichkeit)**: Presumed unless justified
//! 4. **Causation (Kausalität)**: Factual and legal causation
//! 5. **Damage (Schaden)**: Actual harm suffered
//!
//! ## §823 Abs. 2 BGB - Protective Statute Violation
//!
//! Liability for violation of statutes intended to protect individual interests.
//! Examples: Traffic laws (StVO), product safety regulations.
//!
//! ## §826 BGB - Intentional Immoral Damage
//!
//! **Requirements**:
//! 1. **Intent to Cause Damage (Schädigungsvorsatz)**: Both act and harm intended
//! 2. **Contrary to Good Morals (Sittenwidrigkeit)**: Violates sense of propriety
//! 3. **Damage**: Any type of damage (broader than §823 Abs. 1)
//! 4. **Causation**
//!
//! ## §831 BGB - Vicarious Liability
//!
//! Liability for damage caused by agents (Verrichtungsgehilfen) acting in
//! scope of their duties, unless principal proves proper selection/supervision.
//!
//! # Builder Pattern Usage
//!
//! This module provides ergonomic builder APIs for constructing tort claims:
//!
//! ```rust
//! use legalis_de::bgb::unerlaubte_handlungen::*;
//! use legalis_de::gmbhg::Capital;
//! use chrono::Utc;
//!
//! // Build a §823 Abs. 1 claim
//! let claim = TortClaim823_1Builder::new()
//!     .tortfeasor("Max Mustermann", "Berlin")
//!     .injured_party("Erika Schmidt", "Munich")
//!     .protected_interest(ProtectedInterest::Body)
//!     .violation_direct_injury("Fractured leg", "Severe")
//!     .verschulden(Verschulden::EinfacheFahrlassigkeit)
//!     .widerrechtlich(true)
//!     .incident_date(Utc::now())
//!     .damages_medical(Capital::from_euros(8_000))
//!     .damages_lost_income(Capital::from_euros(5_000))
//!     .damages_pain_suffering(Capital::from_euros(10_000))
//!     .causation_established(true)
//!     .build()
//!     .unwrap();
//!
//! // Validate the claim
//! match validate_tort_claim_823_1(&claim) {
//!     Ok(()) => println!("✅ Valid tort claim under §823 Abs. 1 BGB"),
//!     Err(e) => println!("❌ Invalid claim: {}", e),
//! }
//! ```
//!
//! # Damage Types
//!
//! German tort law recognizes several damage categories:
//!
//! - **Vermögensschaden** (Pecuniary Damage):
//!   - Property damage (Sachschaden)
//!   - Lost income (entgangener Gewinn §252 BGB)
//!   - Medical expenses (Heilungskosten)
//!   - Consequential damages (Folgeschäden)
//!
//! - **Nichtvermögensschaden** (Non-Pecuniary Damage):
//!   - Pain and suffering (Schmerzensgeld §253 Abs. 2 BGB)
//!   - Only for personal injury to body or health
//!
//! # Causation Analysis
//!
//! German law uses two causation tests:
//!
//! 1. **Haftungsbegründende Kausalität** (Factual Causation):
//!    - Conditio sine qua non test (but-for causation)
//!    - Without the tortious act, damage would not have occurred
//!
//! 2. **Haftungsausfüllende Kausalität** (Legal Causation):
//!    - Adäquanztheorie (adequacy theory)
//!    - Damage must be typical consequence of the act
//!    - Schutzzwecklehre (protective purpose doctrine)
//!
//! # Justification Grounds (Rechtfertigungsgründe)
//!
//! Unlawfulness can be negated by:
//! - **Notwehr** (Self-defense §32 StGB)
//! - **Notstand** (Necessity §34 StGB, §228, §904 BGB)
//! - **Einwilligung** (Consent)
//! - **Gesetzliche Befugnis** (Legal authorization)
//! - **Rechtausübung** (Exercise of rights)
//!
//! # Contributory Negligence (Mitverschulden)
//!
//! Per §254 BGB, damages may be reduced if injured party contributed to harm
//! through own fault.
//!
//! # Examples
//!
//! ## Traffic Accident (§823 Abs. 1)
//!
//! ```rust
//! use legalis_de::bgb::unerlaubte_handlungen::*;
//! use legalis_de::gmbhg::Capital;
//! use chrono::Utc;
//!
//! let claim = TortClaim823_1Builder::new()
//!     .tortfeasor("Driver A", "Hamburg")
//!     .injured_party("Pedestrian B", "Hamburg")
//!     .protected_interest(ProtectedInterest::Health)
//!     .violation_direct_injury("Multiple fractures", "Severe")
//!     .verschulden(Verschulden::GrobeFahrlassigkeit) // Gross negligence
//!     .widerrechtlich(true)
//!     .incident_date(Utc::now())
//!     .damages_medical(Capital::from_euros(15_000))
//!     .damages_pain_suffering(Capital::from_euros(20_000))
//!     .damages_lost_income(Capital::from_euros(8_000))
//!     .causation_established(true)
//!     .notes("Red light violation, witnesses available")
//!     .build();
//!
//! if let Ok(claim) = claim {
//!     println!("Total damages: €{:.2}", claim.damages.total.to_euros());
//! }
//! ```
//!
//! ## Property Damage
//!
//! ```rust
//! use legalis_de::bgb::unerlaubte_handlungen::*;
//! use legalis_de::gmbhg::Capital;
//! use chrono::Utc;
//!
//! let claim = TortClaim823_1Builder::new()
//!     .tortfeasor("Company X GmbH", "Frankfurt")
//!     .injured_party("Neighbor Y", "Frankfurt")
//!     .protected_interest(ProtectedInterest::Property)
//!     .violation_property_damage("Building facade", Capital::from_euros(50_000))
//!     .verschulden(Verschulden::EinfacheFahrlassigkeit)
//!     .widerrechtlich(true)
//!     .incident_date(Utc::now())
//!     .damages_property(Capital::from_euros(30_000)) // Repair cost
//!     .causation_established(true)
//!     .build()
//!     .unwrap();
//!
//! assert_eq!(claim.damages.total.to_euros(), 30_000.0);
//! ```
//!
//! ## Intentional Tort (§826)
//!
//! ```rust
//! use legalis_de::bgb::unerlaubte_handlungen::*;
//! use legalis_de::gmbhg::Capital;
//! use chrono::Utc;
//!
//! let claim = TortClaim826 {
//!     tortfeasor: TortParty {
//!         name: "Competitor Corp.".to_string(),
//!         address: Some("Berlin".to_string()),
//!         is_natural_person: false,
//!     },
//!     injured_party: TortParty {
//!         name: "Victim GmbH".to_string(),
//!         address: Some("Munich".to_string()),
//!         is_natural_person: false,
//!     },
//!     conduct: "Fraudulent misrepresentation to steal customers".to_string(),
//!     sittenwidrig: true,
//!     schadigungsvorsatz: true,
//!     incident_date: Utc::now(),
//!     damages: DamageClaim {
//!         property_damage: None,
//!         personal_injury: None,
//!         pain_and_suffering: None,
//!         lost_income: Some(Capital::from_euros(100_000)),
//!         medical_expenses: None,
//!         consequential_damages: Some(Capital::from_euros(50_000)),
//!         total: Capital::from_euros(150_000),
//!     },
//!     causation_established: true,
//!     notes: Some("Pattern of systematic interference".to_string()),
//! };
//!
//! assert!(validate_tort_claim_826(&claim).is_ok());
//! ```

pub mod error;
pub mod types;
pub mod validator;

// Re-exports for convenience
pub use error::{Result, TortError};
pub use types::*;
pub use validator::*;
