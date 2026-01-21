//! Criminal Code of Lao PDR (2017) - ກົດໝາຍອາຍາ
//!
//! Comprehensive implementation of the Criminal Code of the Lao People's Democratic Republic.
//!
//! **Legal Basis**: Law No. 26/NA (Criminal Code 2017), effective May 27, 2018
//!
//! # Overview
//!
//! The Criminal Code 2017 is the primary source of criminal law in Laos, replacing the
//! previous code and introducing modern criminal law principles adapted to Lao context.
//!
//! ## Structure
//!
//! The Criminal Code is organized into several parts:
//!
//! ### Part I: General Provisions (Articles 1-70)
//! - **Criminal Liability** (Articles 13-30): Mens rea, actus reus, causation
//! - **Age of Criminal Responsibility** (Article 16): 16 years general, 14 years serious crimes
//! - **Mental Capacity** (Articles 19-21): Full, diminished, no capacity
//! - **Penalties** (Articles 31-60): Death, imprisonment, fines, re-education
//! - **Justifications** (Articles 61-70): Self-defense, necessity, superior orders
//!
//! ### Part II: Specific Crimes (Articles 71-200+)
//! - **Crimes Against Persons** (Articles 121-160)
//!   - Homicide (Articles 121-128)
//!   - Bodily harm (Articles 129-140)
//!   - Sexual crimes (Articles 141-150)
//! - **Crimes Against Property** (Articles 161-200)
//!   - Theft, robbery, fraud, embezzlement, arson
//!
//! # Key Legal Provisions
//!
//! ## Age Requirements
//!
//! - **Criminal responsibility**: 16 years (general), 14 years (serious crimes) - Article 16
//! - **Age of consent**: 15 years (sexual crimes) - Article 141
//! - **Full capacity**: 18 years (aligned with Civil Code)
//!
//! ## Penalties
//!
//! The Criminal Code provides for several types of penalties:
//!
//! 1. **Death penalty** (Article 32): Reserved for most serious crimes
//!    - May be commuted to life imprisonment
//!    - Limited application in practice
//!
//! 2. **Imprisonment** (Articles 33-36)
//!    - Life imprisonment
//!    - Fixed-term imprisonment (minimum to maximum years)
//!
//! 3. **Fines** (Articles 37-40)
//!    - Minimum and maximum amounts in LAK
//!    - May be converted to imprisonment if unpaid
//!
//! 4. **Re-education** (Article 41)
//!    - Without detention (6-24 months typically)
//!    - Community service component
//!
//! 5. **Additional penalties** (Articles 45-46)
//!    - Confiscation of property
//!    - Deprivation of rights
//!
//! ## Justification Grounds
//!
//! The following defenses may negate criminal liability:
//!
//! - **Self-defense** (Article 61): Proportional response to imminent threat
//! - **Necessity** (Article 62): Lesser harm principle, no alternative
//! - **Superior orders** (Article 63): Lawful orders, not manifestly illegal
//! - **Consent** (Article 64): Freely given, informed consent
//! - **Lawful authority** (Article 65): Acting within legal authority
//!
//! # Example Usage
//!
//! ## Validating Criminal Liability
//!
//! ```
//! use legalis_la::criminal_code::{
//!     CriminalLiability, MensRea, ActusReus, MentalCapacityStatus,
//!     validate_criminal_liability,
//! };
//! use chrono::Utc;
//!
//! let liability = CriminalLiability {
//!     mens_rea: MensRea::DirectIntent,
//!     actus_reus: ActusReus::Commission {
//!         act_description_lao: "ການລັກຊັບ".to_string(),
//!         act_description_en: "Theft".to_string(),
//!         time_of_commission: Utc::now(),
//!         location: "Vientiane".to_string(),
//!     },
//!     age_at_offense: 20,
//!     mental_capacity: MentalCapacityStatus::Full,
//!     article_reference: vec![13, 14, 161],
//!     description_lao: "ຄວາມຮັບຜິດຊອບທາງອາຍາສຳລັບການລັກຊັບ".to_string(),
//!     description_en: "Criminal liability for theft".to_string(),
//! };
//!
//! assert!(validate_criminal_liability(&liability).is_ok());
//! ```
//!
//! ## Validating Age of Consent
//!
//! ```
//! use legalis_la::criminal_code::{SexualCrime, validate_sexual_crime};
//!
//! // Statutory rape - victim under 15
//! let statutory_rape = SexualCrime::StatutoryRape {
//!     victim_age: 14,
//!     perpetrator_age: 25,
//!     victim_consented: true, // Consent irrelevant if under 15
//!     relationship: "None".to_string(),
//! };
//!
//! // This will fail validation due to age of consent violation
//! assert!(validate_sexual_crime(&statutory_rape).is_err());
//! ```
//!
//! ## Validating Penalties
//!
//! ```
//! use legalis_la::criminal_code::{Penalty, PenaltySeverity, validate_penalty};
//!
//! let penalty = Penalty::FixedTermImprisonment {
//!     years: 5,
//!     months: 0,
//!     minimum_years: 3,
//!     maximum_years: 10,
//! };
//!
//! assert!(validate_penalty(&penalty, PenaltySeverity::Felony).is_ok());
//! ```
//!
//! # Bilingual Support
//!
//! All types include bilingual (Lao/English) fields throughout:
//!
//! - Type descriptions
//! - Error messages
//! - Documentation
//!
//! This enables proper legal analysis in both languages and supports
//! Lao legal professionals working in multilingual contexts.
//!
//! # Modules
//!
//! - [`types`]: Core type definitions for criminal law
//! - [`error`]: Comprehensive error types with article references
//! - [`validator`]: Validation functions for all criminal law aspects

pub mod error;
pub mod types;
pub mod validator;

// Re-export main types
pub use error::{
    AgeError, BodilyHarmError, CriminalCodeError, HomicideError, JustificationError,
    LiabilityError, PenaltyError, PropertyCrimeError, Result, SexualCrimeError,
};

pub use types::{
    ActusReus, BodilyHarmType, Crime, CrimeType, CriminalLiability, HomicideType,
    JustificationGround, MensRea, MentalCapacity, MentalCapacityStatus, NegligenceType, Penalty,
    PenaltySeverity, Perpetrator, PropertyCrime, SexualCrime, Victim, VictimCategory,
};

pub use validator::{
    validate_actus_reus, validate_age_for_serious_crime, validate_age_of_consent,
    validate_age_of_responsibility, validate_criminal_liability, validate_homicide,
    validate_justification, validate_mens_rea, validate_mental_capacity,
    validate_mental_capacity_status, validate_penalty, validate_property_crime,
    validate_sexual_crime,
};
