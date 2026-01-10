//! German Succession Law (Erbrecht - BGB Book 5)
//!
//! This module provides type-safe representations and validation for German succession law
//! under the Bürgerliches Gesetzbuch (BGB) Book 5 (§§1922-2385).
//!
//! # Legal Context
//!
//! German succession law regulates:
//! - **Legal Succession (Gesetzliche Erbfolge)**: Intestate succession (§§1924-1936)
//! - **Testamentary Succession (Gewillkürte Erbfolge)**: Succession by will or contract
//! - **Will Formalities (Testamentsformen)**: Requirements for valid wills (§§2231-2247)
//! - **Compulsory Portion (Pflichtteil)**: Minimum share for close relatives (§§2303-2338)
//! - **Acceptance/Renunciation (Annahme/Ausschlagung)**: Heir's decision (§§1942-2063)
//!
//! # Key Concepts
//!
//! ## Legal Succession (§§1924-1936 BGB)
//!
//! When there is no will, inheritance follows statutory rules based on "orders" (Ordnungen):
//!
//! ### Order System
//!
//! **First Order (Erste Ordnung)** - §1924 BGB
//! - Descendants: Children, grandchildren, great-grandchildren
//! - All descendants of the deceased
//! - Children inherit equally
//! - Grandchildren inherit by representation if parent deceased
//!
//! **Second Order (Zweite Ordnung)** - §1925 BGB
//! - Parents and their descendants (siblings, nieces, nephews)
//! - Only if no first order heirs exist
//!
//! **Third Order (Dritte Ordnung)** - §1926 BGB
//! - Grandparents and their descendants (uncles, aunts, cousins)
//! - Only if no first or second order heirs exist
//!
//! **Fourth Order and beyond** - §§1928-1929 BGB
//! - Great-grandparents and their descendants
//! - Rare in practice
//!
//! ### Exclusion Principle (Ausschlussprinzip)
//! Higher orders exclude lower orders completely. If any first order heir exists,
//! second order heirs inherit nothing.
//!
//! ### Spouse Inheritance (§1931 BGB)
//!
//! Spouse inherits alongside relatives:
//!
//! **With First Order (Children):**
//! - Community of accrued gains (DEFAULT): 1/2 (1/4 basic + 1/4 bonus)
//! - Separation of property: 1/4
//!
//! **With Second Order (Parents/Siblings):**
//! - Community of accrued gains: 3/4 (1/2 basic + 1/4 bonus)
//! - Separation of property: 1/2
//!
//! **Alone (No relatives):**
//! - Spouse inherits entire estate
//!
//! ## Testamentary Succession
//!
//! ### Will Types (§§2231-2247 BGB)
//!
//! **1. Holographic Will (Eigenhändiges Testament)** - §2247 BGB
//! - Must be ENTIRELY handwritten by testator
//! - Must be signed
//! - Date recommended but not mandatory
//! - Most common form
//! - No witnesses required
//!
//! **2. Public Will (Öffentliches Testament)** - §2232 BGB
//! - Declared before notary
//! - Or deposited with court (Nachlassgericht)
//! - More formal but clearer
//!
//! **3. Emergency Will (Nottestament)** - §§2249-2251 BGB
//! - In exceptional circumstances
//! - Before mayor, 3 witnesses, or orally
//! - Limited validity period
//!
//! ### Testamentary Capacity (Testierfähigkeit) - §2229 BGB
//!
//! - **Age 18+**: Full capacity
//! - **Age 16-17**: Limited capacity (holographic or public will only, not joint wills)
//! - **Under 16**: No capacity
//! - Must not be incapacitated (mental capacity required)
//!
//! ## Compulsory Portion (Pflichtteil) - §§2303-2338 BGB
//!
//! Close relatives cannot be completely disinherited. They are entitled to
//! **half of their legal share** as a monetary claim against heirs.
//!
//! ### Entitled Persons (Pflichtteilsberechtigte):
//! - Descendants (children, grandchildren if parent disinherited/predeceased)
//! - Parents (only if no descendants)
//! - Spouse
//!
//! **NOT entitled:**
//! - Siblings
//! - More distant relatives
//!
//! ### Calculation:
//! 1. Determine legal share if there were no will
//! 2. Compulsory portion = 1/2 of legal share
//! 3. Monetary claim against heirs (not right to assets)
//!
//! ### Example:
//! - Deceased has 2 children, €100,000 estate
//! - Will leaves everything to charity
//! - Each child's legal share would be 1/2
//! - Each child's compulsory portion: 1/2 × 1/2 = 1/4 → €25,000 claim
//!
//! ## Inheritance Contract (Erbvertrag) - §§2274-2302 BGB
//!
//! Binding agreement about succession:
//! - Must be notarized (§2276 BGB) - MANDATORY
//! - More formal than will
//! - Harder to revoke (contractual nature)
//! - Often used between spouses
//!
//! ## Acceptance and Renunciation (§§1942-2063 BGB)
//!
//! ### Automatic Acceptance (§1943 BGB)
//! Inheritance is accepted automatically UNLESS explicitly renounced.
//!
//! ### Renunciation (Ausschlagung) - §1944 BGB
//! - Must be declared within 6 weeks of learning of inheritance
//! - Before Nachlassgericht (probate court) or notary
//! - Cannot be withdrawn
//!
//! ### Why Renounce?
//! - Estate is insolvent (liabilities > assets)
//! - Heir doesn't want to deal with estate administration
//! - Tax reasons
//!
//! ### Effect of Renunciation:
//! - Treated as if heir predeceased testator
//! - Inheritance passes to next in line
//! - Heir's children may inherit by representation
//!
//! # Examples
//!
//! ## Holographic Will
//!
//! ```rust
//! use legalis_de::bgb::erbrecht::*;
//! use chrono::NaiveDate;
//!
//! let deceased = Deceased {
//!     name: "Hans Mueller".to_string(),
//!     date_of_birth: NaiveDate::from_ymd_opt(1950, 1, 1).unwrap(),
//!     date_of_death: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
//!     place_of_death: "Berlin".to_string(),
//!     last_residence: "Berlin".to_string(),
//!     nationality: "German".to_string(),
//! };
//!
//! let will = Will {
//!     testator: deceased,
//!     will_type: WillType::Holographic,
//!     created_at: NaiveDate::from_ymd_opt(2023, 6, 1).unwrap(),
//!     place_of_creation: "Berlin".to_string(),
//!     is_handwritten: true,  // REQUIRED for holographic will
//!     has_signature: true,    // REQUIRED
//!     has_date: true,
//!     beneficiaries: vec![
//!         WillBeneficiary {
//!             name: "Maria Mueller".to_string(),
//!             relationship: RelationshipToDeceased::Spouse,
//!             inheritance_share: InheritanceShare::Full,
//!             conditions: vec![],
//!         },
//!     ],
//!     revoked: false,
//!     revoked_at: None,
//! };
//!
//! // Validate holographic will
//! assert!(validate_holographic_will(&will).is_ok());
//! ```
//!
//! ## Legal Succession Example
//!
//! ```rust
//! use legalis_de::bgb::erbrecht::*;
//! use chrono::NaiveDate;
//!
//! # let deceased = Deceased {
//! #     name: "Hans Mueller".to_string(),
//! #     date_of_birth: NaiveDate::from_ymd_opt(1950, 1, 1).unwrap(),
//! #     date_of_death: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
//! #     place_of_death: "Berlin".to_string(),
//! #     last_residence: "Berlin".to_string(),
//! #     nationality: "German".to_string(),
//! # };
//! // Deceased has 2 children, no will
//! let succession = LegalSuccession {
//!     deceased,
//!     heirs: vec![
//!         Heir {
//!             name: "Child 1".to_string(),
//!             date_of_birth: NaiveDate::from_ymd_opt(1980, 1, 1).unwrap(),
//!             relationship: RelationshipToDeceased::Child,
//!             inheritance_share: InheritanceShare::Fraction {
//!                 numerator: 1,
//!                 denominator: 2,
//!             },
//!             is_statutory_heir: true,
//!         },
//!         Heir {
//!             name: "Child 2".to_string(),
//!             date_of_birth: NaiveDate::from_ymd_opt(1982, 1, 1).unwrap(),
//!             relationship: RelationshipToDeceased::Child,
//!             inheritance_share: InheritanceShare::Fraction {
//!                 numerator: 1,
//!                 denominator: 2,
//!             },
//!             is_statutory_heir: true,
//!         },
//!     ],
//!     succession_order: SuccessionOrder::First,  // First order - descendants
//!     spouse_inheritance: None,
//! };
//!
//! assert!(validate_legal_succession(&succession).is_ok());
//! ```

pub mod error;
pub mod types;
pub mod validator;

// Re-exports for convenience
pub use error::{Result, SuccessionLawError};
pub use types::*;
pub use validator::*;
