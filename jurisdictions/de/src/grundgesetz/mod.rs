//! German Constitutional Law (Grundgesetz - GG)
//!
//! This module provides type-safe representations and validation for German constitutional law
//! under the Grundgesetz (Basic Law), adopted May 23, 1949.
//!
//! # Legal Context
//!
//! The Grundgesetz is Germany's constitution, establishing:
//! - **Basic Rights (Grundrechte)**: Articles 1-19
//! - **Federal Structure (Bundesstruktur)**: Articles 20-146
//! - **Constitutional Review (Verfassungsgerichtsbarkeit)**: Art. 93, 94 GG
//!
//! # Key Constitutional Principles
//!
//! ## 1. Basic Rights (Grundrechte - Articles 1-19)
//!
//! Fundamental rights protecting individuals against state power:
//!
//! ### Human Rights (Menschenrechte)
//! Rights for all persons in Germany:
//! - **Art. 1**: Human dignity - inviolable, state must protect
//! - **Art. 2 Para. 1**: General freedom of action
//! - **Art. 2 Para. 2**: Right to life and physical integrity
//! - **Art. 3**: Equality before the law (no discrimination)
//! - **Art. 4**: Freedom of faith, conscience, religious profession
//! - **Art. 5**: Freedom of expression, press, art, and science
//! - **Art. 6**: Protection of marriage and family
//! - **Art. 14**: Property rights and inheritance
//!
//! ### Citizens' Rights (Deutschenrechte)
//! Rights limited to German citizens:
//! - **Art. 8**: Freedom of assembly (Germans only)
//! - **Art. 9**: Freedom of association (Germans only)
//! - **Art. 11**: Freedom of movement within Germany (Germans only)
//! - **Art. 12**: Occupational freedom (Germans only)
//! - **Art. 16**: Protection against extradition
//!
//! ### Institutional Guarantees
//! - **Art. 7**: Education system (Schulwesen)
//! - **Art. 13**: Inviolability of home
//! - **Art. 17**: Right to petition
//! - **Art. 19 Para. 4**: Right to legal recourse (Rechtsweg)
//!
//! ## 2. Proportionality Test (Verhältnismäßigkeitsprüfung)
//!
//! Three-step test for justifying restrictions on basic rights:
//!
//! ### Step 1: Suitability (Geeignetheit)
//! The measure must be suitable to achieve the legitimate purpose.
//! - Question: Can the measure achieve the stated goal?
//! - Standard: Measure must promote the purpose (not necessarily achieve it perfectly)
//!
//! ### Step 2: Necessity (Erforderlichkeit)
//! The measure must be necessary (no equally effective but less restrictive alternative).
//! - Question: Is there a milder means that would achieve the same result?
//! - Standard: Measure must be the least restrictive option
//!
//! ### Step 3: Proportionality Stricto Sensu (Angemessenheit)
//! The measure must be proportionate in the narrow sense.
//! - Question: Does the public benefit outweigh the private burden?
//! - Standard: Balancing of interests (Abwägung)
//!
//! ### Example:
//! ```rust
//! use legalis_de::grundgesetz::*;
//! use chrono::NaiveDate;
//!
//! # let authority = PublicAuthority {
//! #     name: "Bundestag".to_string(),
//! #     authority_type: AuthorityType::Legislative,
//! #     level: FederalLevel::Federal,
//! # };
//! let test = ProportionalityTest {
//!     restriction: RightsRestriction {
//!         restricting_authority: authority,
//!         legal_basis: "Public Assembly Act".to_string(),
//!         restriction_type: RestrictionType::PermitRequirement,
//!         date_of_restriction: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
//!         justification: "Public order protection".to_string(),
//!     },
//!     legitimate_purpose: "Prevent violence at demonstrations".to_string(),
//!     suitable: SuitabilityAssessment {
//!         is_suitable: true,
//!         reasoning: "Permit requirement allows advance planning".to_string(),
//!     },
//!     necessary: NecessityAssessment {
//!         is_necessary: true,
//!         alternative_measures: vec![],
//!         reasoning: "No less restrictive alternative available".to_string(),
//!     },
//!     proportionate_stricto_sensu: ProportionalityStrictoSensu {
//!         is_proportionate: true,
//!         public_interest: "Public order and safety".to_string(),
//!         private_interest: "Freedom of assembly".to_string(),
//!         balancing: "Minor administrative burden justified by public safety".to_string(),
//!     },
//! };
//!
//! assert!(validate_proportionality_test(&test).is_ok());
//! assert!(test.passes_test());
//! ```
//!
//! ## 3. Constitutional Complaint (Verfassungsbeschwerde)
//!
//! Individual complaint to Federal Constitutional Court (BVerfG) - Art. 93 Para. 1 No. 4a GG
//!
//! ### Admissibility Requirements:
//!
//! **1. Exhaustion of Legal Remedies (Subsidiaritätsprinzip)** - Art. 90 Para. 2 BVerfGG
//! - Must exhaust all other legal remedies first
//! - Exception: Direct fundamental rights violation by statute
//!
//! **2. Standing (Beschwerdebefugnis)**
//! - **Self** (selbst): Complainant personally affected
//! - **Current** (gegenwärtig): Currently affected (not hypothetical)
//! - **Immediate** (unmittelbar): Directly affected by state action
//!
//! **3. Deadline (Beschwerdefrist)** - §93 BVerfGG
//! - **One month** from service of decision (for court/administrative decisions)
//! - **One year** from enactment (for statutes)
//!
//! **4. Violation of Basic Right**
//! - Complainant must allege specific basic right violation
//! - Art. 93 Para. 1 No. 4a GG limits to basic rights or rights equivalent to basic rights
//!
//! ## 4. Essential Content Guarantee (Wesensgehaltsgarantie)
//!
//! Art. 19 Para. 2 GG: "In no case may the essential content of a basic right be affected."
//!
//! - Core of basic right cannot be eliminated
//! - Protects minimum substance of rights
//! - Absolute limit on legislative power
//!
//! ## 5. Federal Structure (Bundesstruktur)
//!
//! ### Bundestag (Federal Parliament) - Art. 38-49 GG
//! - Directly elected by people (Art. 38 GG)
//! - Electoral term: 4 years (Wahlperiode)
//! - Free mandate: Representatives bound only by conscience (Art. 38 Para. 1 Sent. 2 GG)
//! - Functions: Legislation, budget, government control
//!
//! ### Bundesrat (Federal Council) - Art. 50-53 GG
//! - Represents the Länder (states)
//! - Not elected, members appointed by state governments
//! - Votes based on population (Art. 51 Para. 2 GG):
//!   - < 2 million: 3 votes
//!   - 2-6 million: 4 votes
//!   - 6-7 million: 5 votes
//!   - > 7 million: 6 votes
//! - Functions: Consent to federal laws affecting states
//!
//! ### Federal President (Bundespräsident) - Art. 54-61 GG
//! - Head of state (representative function)
//! - Elected by Federal Convention (Bundesversammlung)
//! - Term: 5 years, maximum 2 consecutive terms (Art. 54 Para. 2 GG)
//! - Powers: Signs laws, appoints/dismisses officials, represents Germany abroad
//!
//! ### Federal Government (Bundesregierung) - Art. 62-69 GG
//! - Federal Chancellor (Bundeskanzler) - Art. 63-67 GG
//!   - Determines policy guidelines (Richtlinienkompetenz) - Art. 65 Sent. 1 GG
//!   - Heads the government
//!   - Elected by Bundestag
//! - Federal Ministers (Bundesminister)
//!   - Manage their departments autonomously (Ressortprinzip) - Art. 65 Sent. 2 GG
//!   - Within Chancellor's policy guidelines
//!
//! ## 6. Legislative Competence (Gesetzgebungskompetenz)
//!
//! ### Art. 70 GG: Presumption of State Competence
//! States have legislative power unless Basic Law confers it on federation.
//!
//! ### Art. 71-73 GG: Exclusive Federal Competence (Ausschließliche Gesetzgebung)
//! - Foreign affairs, defense, citizenship, currency
//! - Federal railways, postal services, telecommunications
//! - States may NOT legislate in these areas
//!
//! ### Art. 72, 74 GG: Concurrent Competence (Konkurrierende Gesetzgebung)
//! - Civil law, criminal law, labor law, social welfare
//! - States may legislate ONLY if and to the extent that federal government has not
//! - Federal necessity requirement (Art. 72 Para. 2 GG)
//!
//! ### Art. 70 GG: Residual State Competence (Länderzuständigkeit)
//! - Education, police, local government
//! - Cultural affairs (Kulturhoheit)
//! - All matters not explicitly federal
//!
//! # Examples
//!
//! See `examples/basic-rights.rs` for demonstrations of:
//! - Basic rights with restrictions
//! - Proportionality test application
//! - Constitutional complaint filing
//!
//! See `examples/proportionality-test.rs` for detailed proportionality analysis.

pub mod error;
pub mod types;
pub mod validator;

// Re-exports for convenience
pub use error::{ConstitutionalError, Result};
pub use types::*;
pub use validator::*;
