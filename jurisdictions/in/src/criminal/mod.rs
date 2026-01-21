//! Bharatiya Nyaya Sanhita (BNS) 2023 - Indian Criminal Law
//!
//! # Overview
//!
//! This module implements India's new criminal code - the Bharatiya Nyaya Sanhita
//! (BNS) 2023, which replaced the Indian Penal Code (IPC) 1860 on 1st July 2024.
//!
//! The new criminal justice laws include:
//! - **BNS 2023**: Substantive criminal law (replaces IPC 1860)
//! - **BNSS 2023**: Criminal procedure (replaces CrPC 1973)
//! - **BSA 2023**: Evidence law (replaces Indian Evidence Act 1872)
//!
//! ## Key Changes from IPC
//!
//! | Aspect | IPC 1860 | BNS 2023 |
//! |--------|----------|----------|
//! | Total Sections | 511 | 358 |
//! | New Offences | - | Organized crime, terrorism, mob lynching |
//! | Sedition | Section 124A | Section 152 (modified) |
//! | Community Service | Not available | Available for petty offences |
//! | Hit and Run | Not specific | Section 106(2) specific |
//!
//! ## Offence Categories
//!
//! BNS organizes offences into clear chapters:
//!
//! | Chapter | Subject | Key Offences |
//! |---------|---------|--------------|
//! | IV | Organized Crime | Sections 111-113 |
//! | V | Sexual Offences | Sections 63-78 |
//! | VI | Human Body | Sections 100-148 (Murder, Hurt) |
//! | VII | Against State | Sections 147-160 |
//! | XVII | Property | Sections 303-334 (Theft, Robbery) |
//!
//! ## Example: Offence Analysis
//!
//! ```rust
//! use legalis_in::criminal::*;
//!
//! // Check offence characteristics
//! let offence = Offence::Theft;
//! assert_eq!(offence.section(), 303);
//! assert_eq!(offence.ipc_equivalent(), Some(378));
//! assert!(offence.is_bailable());
//! assert!(!offence.is_cognizable());
//!
//! // Get punishment
//! let punishment = get_punishment_for_offence(&offence);
//! assert_eq!(punishment.max_years, Some(3));
//! ```
//!
//! ## Example: Case Compliance Check
//!
//! ```rust
//! use legalis_in::criminal::*;
//! use chrono::NaiveDate;
//!
//! let case = CriminalCase {
//!     case_number: "CR-2024-001".to_string(),
//!     offence: Offence::Theft,
//!     sections: vec![303],
//!     offence_date: NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid"),
//!     fir_number: Some("FIR-001/2024".to_string()),
//!     fir_date: Some(NaiveDate::from_ymd_opt(2024, 1, 2).expect("valid")),
//!     investigation_status: InvestigationStatus::Ongoing,
//!     accused: vec![],
//!     complainant: Some("Victim Name".to_string()),
//!     court: Court::JudicialMagistrateFirstClass,
//!     status: CaseStatus::UnderInvestigation,
//! };
//!
//! let report = validate_criminal_compliance(&case);
//! println!("Compliant: {}", report.compliant);
//! ```
//!
//! ## Punishment Structure
//!
//! BNS punishments include:
//! - **Death Penalty**: For gravest offences (murder, gang rape, terrorism)
//! - **Life Imprisonment**: May extend to remainder of natural life
//! - **Rigorous Imprisonment**: With hard labor
//! - **Simple Imprisonment**: Without hard labor
//! - **Fine**: Fixed, minimum, or discretionary
//! - **Community Service**: New in BNS for petty offences
//!
//! ## Court Jurisdiction
//!
//! | Court | Max Imprisonment | Max Fine |
//! |-------|------------------|----------|
//! | JMSC | 1 year | Rs. 5,000 |
//! | JMFC/MM | 3 years | Rs. 10,000 |
//! | CJM | 7 years | No limit |
//! | Sessions | No limit | No limit |
//!
//! ## Procedural Safeguards (BNSS 2023)
//!
//! - **Section 35**: Arrest guidelines
//! - **Section 41A**: Notice of appearance before arrest
//! - **Section 187**: Production before magistrate within 24 hours
//! - **Section 193**: Chargesheet within 60/90 days
//!
//! ## Statutory Bail
//!
//! ```rust
//! use legalis_in::criminal::*;
//! use chrono::NaiveDate;
//!
//! let fir_date = NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid");
//! let eligible = calculate_statutory_bail_eligibility(
//!     fir_date,
//!     &Offence::Theft,
//!     false, // Chargesheet not filed
//! );
//! ```
//!
//! ## Plea Bargaining
//!
//! Available for offences:
//! - Not affecting socio-economic conditions
//! - Not committed against women/children
//! - Not punishable with death/life imprisonment
//! - Not punishable with > 7 years imprisonment
//!
//! ```rust
//! use legalis_in::criminal::*;
//!
//! let eligibility = PleaBargaining::check_eligibility(&Offence::Theft);
//! assert!(eligibility.eligible);
//! assert_eq!(eligibility.max_reduction, Some(0.25)); // Up to 25% reduction
//! ```
//!
//! ## New Offences in BNS
//!
//! | Offence | Section | Punishment |
//! |---------|---------|------------|
//! | Organized Crime | 111 | Life/Death + Rs. 50 lakh |
//! | Terrorist Act | 113 | Life/Death + Rs. 1 crore |
//! | Mob Lynching | 103(2) | Death/Life |
//! | Hit and Run | 106(2) | Up to 10 years + Rs. 7 lakh |
//! | Snatching | 304 | Up to 3 years |
//!
//! ## References
//!
//! - [Bharatiya Nyaya Sanhita, 2023](https://legislative.gov.in/acts/bharatiya-nyaya-sanhita-2023)
//! - [Bharatiya Nagarik Suraksha Sanhita, 2023](https://legislative.gov.in/acts/bharatiya-nagarik-suraksha-sanhita-2023)
//! - [Bharatiya Sakshya Adhiniyam, 2023](https://legislative.gov.in/acts/bharatiya-sakshya-adhiniyam-2023)

pub mod error;
pub mod types;
pub mod validator;

pub use error::*;
pub use types::*;
pub use validator::*;
