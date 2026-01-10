//! Intellectual Property Laws Module
//!
//! This module provides comprehensive support for Singapore IP laws covering four main statutes:
//!
//! ## Statutes Covered
//!
//! ### 1. Patents Act (Cap. 221)
//! - **Term**: 20 years from filing date (s. 36)
//! - **Requirements**: Novelty, inventive step, industrial application (s. 13-16)
//! - **Protection**: Exclusive rights to make, use, sell invention (s. 66)
//! - **Key Features**: Grace period (12 months), compulsory licensing
//!
//! ### 2. Trade Marks Act (Cap. 332)
//! - **Term**: 10 years, renewable indefinitely (s. 18)
//! - **Classification**: Nice Classification (45 classes)
//! - **Requirements**: Distinctiveness, not deceptive (s. 7)
//! - **Key Features**: Well-known marks protection (s. 55), Madrid Protocol
//!
//! ### 3. Copyright Act 2021
//! - **Term**: Life + 70 years, or Publication + 70 years (s. 28)
//! - **Protection**: Automatic (no registration required)
//! - **Categories**: Literary, musical, artistic, dramatic, films, sound recordings
//! - **Key Features**: Fair dealing exceptions (s. 35-42), orphan works
//!
//! ### 4. Registered Designs Act (Cap. 266)
//! - **Term**: 5 years, renewable twice (max 15 years) (s. 21)
//! - **Requirements**: Novelty, individual character (s. 5)
//! - **Exclusions**: Purely functional designs (s. 6)
//! - **Key Features**: Locarno Classification
//!
//! ## Examples
//!
//! ### Patent Validation
//! ```
//! use legalis_sg::ip::*;
//! use chrono::NaiveDate;
//!
//! let patent = Patent::new(
//!     "SG 10201912345A",
//!     "Novel Widget Mechanism",
//!     "Acme Corporation",
//!     NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
//! );
//!
//! let report = validate_patent(&patent).unwrap();
//! if report.is_valid {
//!     println!("Patent valid, {} years remaining", report.years_remaining);
//! }
//! ```
//!
//! ### Trademark Similarity Check
//! ```
//! use legalis_sg::ip::*;
//! use chrono::NaiveDate;
//!
//! let tm1 = Trademark::new(
//!     "TM001",
//!     "BRAND",
//!     "Company A",
//!     NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
//! );
//!
//! let tm2 = Trademark::new(
//!     "TM002",
//!     "BREND",
//!     "Company B",
//!     NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
//! );
//!
//! let similarity = tm1.similarity_score(&tm2);
//! println!("Similarity: {}%", similarity);
//! ```
//!
//! ### Copyright Term Calculation
//! ```
//! use legalis_sg::ip::*;
//! use chrono::NaiveDate;
//!
//! let copyright = Copyright::new(
//!     "Great Novel",
//!     "Author Name",
//!     WorkType::Literary,
//!     NaiveDate::from_ymd_opt(1950, 1, 1).unwrap(),
//! );
//!
//! let death_date = NaiveDate::from_ymd_opt(1980, 1, 1).unwrap();
//! let expiry = copyright.expiry_date(Some(death_date));
//! println!("Copyright expires: {:?}", expiry);
//! ```
//!
//! ## IPOS (Intellectual Property Office of Singapore)
//!
//! All IP registrations in Singapore are handled by IPOS:
//! - Online filing via IP2SG portal
//! - Search databases for patents, trademarks, designs
//! - Examination and grant processes
//! - Renewal and maintenance
//!
//! ## International Treaties
//!
//! Singapore is a party to major IP treaties:
//! - **PCT (Patent Cooperation Treaty)**: International patent applications
//! - **Madrid Protocol**: International trademark registration
//! - **Paris Convention**: Priority rights (6/12 months)
//! - **Berne Convention**: Copyright protection (automatic)
//! - **TRIPS Agreement**: Minimum IP standards
//!
//! ## Key Differences from Other Jurisdictions
//!
//! ### vs. United States
//! - Singapore: First-to-file system (patents)
//! - US: First-inventor-to-file since 2013
//! - Singapore: Copyright life + 70 years
//! - US: Copyright life + 70 years (harmonized)
//!
//! ### vs. European Union
//! - Singapore: Single national system
//! - EU: European Patent system + national systems
//! - Singapore: Trademark 10-year terms
//! - EU: Trademark 10-year terms (harmonized)
//!
//! ### vs. China
//! - Singapore: English language proceedings
//! - China: Chinese language required
//! - Singapore: Common law system
//! - China: Civil law with Chinese characteristics

pub mod error;
pub mod types;
pub mod validator;

// Re-export commonly used types
pub use error::{IpError, IpType, Result};
pub use types::*;
pub use validator::*;
