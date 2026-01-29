//! Code of Civil Procedure (CPC) 1908
//!
//! # Overview
//!
//! This module implements India's procedural law for civil suits - the Code of Civil
//! Procedure (CPC) 1908. The CPC prescribes the procedure for conducting civil litigation
//! in India's civil courts.
//!
//! ## Key Features
//!
//! - **Uniform Procedure**: Applies to all civil courts in India
//! - **Hierarchical Appeal System**: First Appeal → Second Appeal → Supreme Court
//! - **Time-Bound Procedures**: Strict limitation periods
//! - **Court Fees**: Ad valorem fees based on suit value
//!
//! ## Structure of CPC
//!
//! | Part | Content | Sections |
//! |------|---------|----------|
//! | I | Preliminary | 1-8 |
//! | II | Jurisdiction | 9-28 |
//! | III | Suits in General | 29-55 |
//! | IV | Appeals | 96-112 |
//! | V | Execution | 36-74 |
//!
//! ## Orders (Most Important)
//!
//! | Order | Subject | Key Rules |
//! |-------|---------|-----------|
//! | 6 | Pleadings | Rule 17 (Amendment) |
//! | 7 | Plaint | Rule 11 (Rejection grounds) |
//! | 8 | Written Statement | Rule 1 (120 days time limit) |
//! | 9 | Default | Rules 6-8 (Ex-parte, Dismissal) |
//! | 14 | Framing of Issues | |
//! | 21 | Execution | Rules 37, 48, 54 (Modes) |
//! | 39 | Temporary Injunction | Rules 1-2 |
//! | 41 | First Appeal | Rule 1 (Security) |
//!
//! ## Example: Validate Civil Suit
//!
//! ```rust
//! use legalis_in::civil_procedure_code::*;
//! use chrono::NaiveDate;
//!
//! let suit = CivilSuit {
//!     suit_number: "CS-123/2024".to_string(),
//!     suit_type: SuitType::Money,
//!     court: CourtType::DistrictCourt,
//!     plaintiff: "ABC Ltd.".to_string(),
//!     defendant: "XYZ Pvt. Ltd.".to_string(),
//!     suit_value: 500000.0, // Rs. 5 lakhs
//!     filing_date: NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
//!     jurisdiction_basis: vec![JurisdictionBasis::DefendantResidence],
//!     court_fees_paid: 10000.0,
//!     within_limitation: true,
//!     status: SuitStatus::Filed,
//! };
//!
//! let report = validate_suit_compliance(&suit);
//! assert!(report.compliant);
//! ```
//!
//! ## Example: Calculate Court Fees
//!
//! ```rust
//! use legalis_in::civil_procedure_code::CourtFees;
//!
//! // Money suit for Rs. 10 lakhs
//! let fees = CourtFees::calculate_money_suit(1000000.0);
//! println!("Court fees: Rs. {}", fees.total_fees);
//! ```
//!
//! ## Example: Check Appeal Limitation
//!
//! ```rust
//! use legalis_in::civil_procedure_code::*;
//! use chrono::NaiveDate;
//!
//! let appeal = Appeal {
//!     appeal_number: "FA-45/2024".to_string(),
//!     appeal_type: AppealType::FirstAppeal,
//!     lower_court_suit: "CS-100/2023".to_string(),
//!     appellant: "Appellant Name".to_string(),
//!     respondent: "Respondent Name".to_string(),
//!     decree_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
//!     filing_date: NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
//!     within_limitation: true,
//!     court_fee_paid: true,
//!     security_deposited: true,
//!     status: AppealStatus::Filed,
//! };
//!
//! let report = validate_appeal_compliance(&appeal);
//! println!("Appeal compliant: {}", report.compliant);
//! ```
//!
//! ## Jurisdiction (Section 15-20)
//!
//! ### Territorial Jurisdiction
//!
//! Suits must be filed where:
//! 1. **Defendant resides/works** (Section 20)
//! 2. **Cause of action arose** (Section 20)
//! 3. **Property is situated** (Sections 16-19)
//! 4. **Contract to be performed** (Section 20)
//!
//! ### Pecuniary Jurisdiction
//!
//! | Court | Min Value | Max Value |
//! |-------|-----------|-----------|
//! | Supreme Court | No limit | No limit |
//! | High Court | Rs. 20 lakhs* | No limit |
//! | District Court | Rs. 50,000* | No limit |
//! | Sub-Judge | Rs. 10,000* | Rs. 5 lakhs* |
//! | Munsif | - | Rs. 50,000* |
//!
//! *Varies by state
//!
//! ## Limitation Periods (Limitation Act 1963)
//!
//! | Suit Type | Period | Starting Point |
//! |-----------|--------|----------------|
//! | Money suit | 3 years | When debt accrued |
//! | Specific performance | 3 years | Date of breach |
//! | Possession | 12 years | Date of dispossession |
//! | Declaratory suit | 3 years | Right to sue accrued |
//!
//! ## Court Fees (Court Fees Act 1870)
//!
//! Ad valorem fees structure:
//! - Up to Rs. 500: Rs. 5
//! - Rs. 500-1,000: Rs. 5 + 4% of excess
//! - Rs. 1,000-5,000: Rs. 25 + 3% of excess
//! - Rs. 5,000-20,000: Rs. 145 + 2.5% of excess
//! - Above Rs. 20,000: Rs. 520 + 2% of excess
//!
//! ## Appeal System
//!
//! ### First Appeal (Section 96)
//! - **From**: Original decree of subordinate court
//! - **To**: District Court / High Court
//! - **Limitation**: 90 days
//! - **Scope**: Questions of fact and law
//!
//! ### Second Appeal (Section 100)
//! - **From**: First appellate decree
//! - **To**: High Court
//! - **Limitation**: 90 days
//! - **Scope**: Substantial question of law only
//!
//! ### Revision (Section 115)
//! - **Scope**: Supervisory jurisdiction
//! - **Grounds**: Jurisdiction, material irregularity
//! - Not an appeal
//!
//! ## Execution (Order 21)
//!
//! ### Modes of Execution
//!
//! 1. **Attachment and sale** (Rule 54)
//! 2. **Arrest and detention** (Rule 37) - limited use
//! 3. **Salary attachment** (Rule 48) - max 2/3 of gross
//! 4. **Delivery of possession**
//! 5. **Appointment of receiver** (Rule 4)
//!
//! ### Limitation for Execution
//!
//! - **12 years** from date of decree (Section 48)
//! - Extendable if execution attempted within period
//!
//! ## Interim Reliefs (Order 39)
//!
//! ### Temporary Injunction (Rule 1-2)
//!
//! Granted if:
//! - Prima facie case established
//! - Balance of convenience in favor
//! - Irreparable injury likely
//!
//! ### Attachment Before Judgment (Order 38)
//!
//! Allowed if:
//! - Defendant about to remove property
//! - With intent to defraud creditors
//!
//! ## Hindi Terminology
//!
//! | English | Hindi (हिंदी) | Pronunciation |
//! |---------|--------------|---------------|
//! | Plaint | वाद पत्र | Vaad Patra |
//! | Written Statement | लिखित कथन | Likhit Kathan |
//! | Decree | आदेशिका | Aadeshika |
//! | Execution | निष्पादन | Nishpaadan |
//! | Appeal | अपील | Apeel |
//! | Limitation | परिसीमा | Pariseema |
//! | Court Fee | न्यायालय शुल्क | Nyayalaya Shulk |
//!
//! ## References
//!
//! - [Code of Civil Procedure, 1908](https://legislative.gov.in/actsofparliamentfromtheyear/code-civil-procedure-1908)
//! - [Limitation Act, 1963](https://legislative.gov.in/actsofparliamentfromtheyear/limitation-act-1963)
//! - [Court Fees Act, 1870](https://legislative.gov.in/actsofparliamentfromtheyear/court-fees-act-1870)
//! - Mulla: The Code of Civil Procedure (18th Ed. 2022)
//! - CPC Bare Act with Short Notes

pub mod error;
pub mod types;
pub mod validator;

pub use error::*;
pub use types::*;
pub use validator::*;
