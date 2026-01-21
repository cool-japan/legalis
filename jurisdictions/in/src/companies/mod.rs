//! Companies Act 2013 Module
//!
//! # Indian Corporate Law
//!
//! This module implements India's Companies Act, 2013, the primary legislation
//! governing company formation, management, and dissolution in India.
//!
//! ## Key Legislation
//!
//! - **Companies Act, 2013**: Primary company law
//! - **Companies Rules, 2014**: Detailed procedural rules
//! - **SEBI LODR, 2015**: Listed company regulations
//!
//! ## Company Types (Section 2)
//!
//! | Type | Section | Min Directors | Description |
//! |------|---------|---------------|-------------|
//! | Private | 2(68) | 2 | Maximum 200 members, restricted share transfer |
//! | Public | 2(71) | 3 | Listed on stock exchange possible |
//! | OPC | 2(62) | 2 | Single member company |
//! | Section 8 | 8 | 2 | Non-profit organizations |
//! | Nidhi | 406 | 3 | Mutual benefit society |
//!
//! ## Board Composition Requirements
//!
//! ### Private Companies
//! - Minimum 2 directors
//! - At least 1 resident director (182 days in India)
//!
//! ### Public Companies
//! - Minimum 3 directors
//! - At least 1 resident director
//! - Woman director (if paid-up capital >= Rs. 100 crore)
//!
//! ### Listed Companies
//! - Minimum 3 directors
//! - At least 1/3 independent directors
//! - At least 1 woman director
//! - At least 1 resident director
//!
//! ## Key Managerial Personnel (Section 203)
//!
//! Required for public companies with:
//! - Paid-up capital >= Rs. 10 crore, OR
//! - Turnover >= Rs. 100 crore
//!
//! Mandatory KMP:
//! - Managing Director / CEO / Manager (any one)
//! - Company Secretary
//! - Chief Financial Officer
//!
//! ## Board Committees
//!
//! ### Audit Committee (Section 177)
//! Required for listed and prescribed public companies.
//!
//! Composition:
//! - Minimum 3 directors
//! - 2/3 must be independent directors
//! - Chairperson must be independent director
//! - All members financially literate
//!
//! ### Nomination and Remuneration Committee (Section 178)
//! Required for listed and prescribed public companies.
//!
//! ### Stakeholders Relationship Committee (Section 178)
//! Required for companies with > 1000 shareholders.
//!
//! ### CSR Committee (Section 135)
//! Required for companies meeting any of:
//! - Net worth >= Rs. 500 crore
//! - Turnover >= Rs. 1000 crore
//! - Net profit >= Rs. 5 crore
//!
//! ## Corporate Social Responsibility (Section 135)
//!
//! Applicable companies must:
//! 1. Constitute CSR Committee
//! 2. Spend at least 2% of average net profit (last 3 FYs)
//! 3. Report CSR activities in Board Report
//! 4. Transfer unspent amount to Unspent CSR Account
//!
//! Schedule VII activities include:
//! - Poverty eradication
//! - Education
//! - Gender equality
//! - Environmental sustainability
//! - Healthcare
//! - Rural development
//!
//! ## Resolutions
//!
//! ### Ordinary Resolution (Section 114(1))
//! - Simple majority (> 50%)
//! - Routine matters
//!
//! ### Special Resolution (Section 114(2))
//! - 75% majority
//! - Major decisions (alteration of articles, name change, etc.)
//!
//! ## Annual Filings
//!
//! | Form | Description | Due Date |
//! |------|-------------|----------|
//! | MGT-7/7A | Annual Return | 60 days from AGM |
//! | AOC-4 | Financial Statements | 30 days from AGM |
//! | DIR-3 KYC | Director KYC | September 30 |
//! | ADT-1 | Auditor Appointment | 15 days from AGM |
//!
//! ## Penalties
//!
//! The Companies Act provides for:
//! - Monetary penalties on company and officers
//! - Continuing penalties for ongoing violations
//! - Imprisonment for serious violations
//! - Disqualification of directors
//!
//! ## Example: Validate Board Composition
//!
//! ```rust
//! use legalis_in::companies::*;
//! use chrono::NaiveDate;
//!
//! let company = Company {
//!     cin: "U12345MH2020PTC123456".to_string(),
//!     name: "Test Company Pvt Ltd".to_string(),
//!     company_type: CompanyType::PrivateLimited,
//!     incorporation_date: NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
//!     registered_office: "Mumbai".to_string(),
//!     state: "Maharashtra".to_string(),
//!     roc: "RoC-Mumbai".to_string(),
//!     status: CompanyStatus::Active,
//!     authorized_capital: 10_000_000,
//!     paid_up_capital: 5_000_000,
//!     fy_end_month: 3,
//!     is_listed: false,
//!     directors: vec![
//!         Director {
//!             din: "12345678".to_string(),
//!             name: "Director 1".to_string(),
//!             category: DirectorCategory::Executive,
//!             appointment_date: NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
//!             term_end: None,
//!             resident_in_india: true,
//!             disqualified: false,
//!             din_status: DinStatus::Approved,
//!             other_directorships: 2,
//!         },
//!         Director {
//!             din: "87654321".to_string(),
//!             name: "Director 2".to_string(),
//!             category: DirectorCategory::NonExecutive,
//!             appointment_date: NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
//!             term_end: None,
//!             resident_in_india: false,
//!             disqualified: false,
//!             din_status: DinStatus::Approved,
//!             other_directorships: 1,
//!         },
//!     ],
//!     kmps: vec![],
//!     shareholders: vec![],
//!     committees: vec![],
//! };
//!
//! let report = validate_board_composition(&company);
//! assert!(report.compliant);
//! ```
//!
//! ## References
//!
//! - [Companies Act, 2013](https://www.mca.gov.in/content/mca/global/en/acts-rules/ebooks/acts.html)
//! - [MCA Portal](https://www.mca.gov.in/)
//! - [SEBI LODR Regulations](https://www.sebi.gov.in/legal/regulations/apr-2023/securities-and-exchange-board-of-india-listing-obligations-and-disclosure-requirements-regulations-2015-last-amended-on-june-14-2023-_73228.html)

#![allow(missing_docs)]

pub mod error;
pub mod types;
pub mod validator;

pub use error::*;
pub use types::*;
pub use validator::*;
