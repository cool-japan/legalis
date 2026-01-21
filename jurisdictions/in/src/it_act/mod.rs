//! Information Technology Act 2000 Module
//!
//! # Cyber Law in India
//!
//! This module implements the Information Technology Act, 2000 (as amended),
//! which provides the legal framework for e-commerce, digital signatures,
//! cyber crimes, and data protection in India.
//!
//! ## Historical Background
//!
//! - Enacted: June 9, 2000
//! - Major Amendment: 2008 (IT Amendment Act)
//! - Related Rules: IT Rules 2011, IT Rules 2021
//!
//! ## Key Features
//!
//! ### Legal Recognition of Electronic Records (Chapter III)
//!
//! - **Section 4**: Legal recognition of electronic records
//! - **Section 5**: Legal recognition of electronic signatures
//! - **Section 10A**: Validity of contracts formed through electronic means
//!
//! ### Digital Signatures (Chapter IV)
//!
//! | Class | Verification | Use Case |
//! |-------|-------------|----------|
//! | Class 1 | Email-based | Low value transactions |
//! | Class 2 | Database verification | MCA filings, ITR |
//! | Class 3 | In-person verification | e-Tendering, auctions |
//!
//! ## Cyber Offences (Chapter XI)
//!
//! ### Civil Offences (Section 43)
//!
//! | Clause | Offence | Remedy |
//! |--------|---------|--------|
//! | 43(a) | Unauthorized access | Compensation |
//! | 43(b) | Unauthorized download | Compensation |
//! | 43(c) | Introducing virus/malware | Compensation |
//! | 43(d) | Damaging computer system | Compensation |
//! | 43(e) | Disrupting service | Compensation |
//! | 43(f) | Denying access | Compensation |
//!
//! ### Criminal Offences (Sections 65-67)
//!
//! | Section | Offence | Punishment |
//! |---------|---------|------------|
//! | 65 | Tampering source code | 3 years + Rs. 2 lakhs |
//! | 66 | Hacking | 3 years + Rs. 5 lakhs |
//! | 66C | Identity theft | 3 years + Rs. 1 lakh |
//! | 66D | Cheating by personation | 3 years + Rs. 1 lakh |
//! | 66E | Privacy violation | 3 years + Rs. 2 lakhs |
//! | 66F | Cyber terrorism | Life imprisonment |
//! | 67 | Obscene material | 5 years + Rs. 10 lakhs |
//! | 67A | Sexually explicit | 7 years + Rs. 10 lakhs |
//! | 67B | Child pornography | 7 years + Rs. 10 lakhs |
//!
//! ## Intermediary Liability (Section 79)
//!
//! An intermediary is **not liable** for third-party information if:
//! 1. Function limited to providing access/transmission
//! 2. Does not initiate transmission
//! 3. Does not select receiver
//! 4. Does not select or modify information
//! 5. Observes due diligence (IT Rules 2021)
//! 6. Complies with government directions
//! 7. Does not conspire, abet, aid, or induce
//!
//! ## IT Rules 2021 (Intermediary Guidelines)
//!
//! ### Due Diligence (Rule 3)
//!
//! All intermediaries must:
//! - Publish privacy policy and user agreement
//! - Inform users about prohibited content
//! - Appoint Grievance Officer (within India)
//! - Acknowledge complaints within 24 hours
//! - Resolve complaints within 15 days
//!
//! ### SSMI Requirements (Rule 4)
//!
//! Significant Social Media Intermediaries (50 lakh+ users) must:
//! - Appoint Chief Compliance Officer (Indian resident)
//! - Appoint Nodal Contact Person (24x7 coordination)
//! - Appoint Resident Grievance Officer (India-based)
//! - File monthly compliance reports
//! - Enable first originator identification
//!
//! ## Data Protection (IT Rules 2011)
//!
//! ### Sensitive Personal Data (Rule 3)
//!
//! - Password
//! - Financial information
//! - Health information
//! - Sexual orientation
//! - Medical records
//! - Biometric information
//!
//! ### Collection Requirements (Rule 5)
//!
//! - Written consent required
//! - Purpose must be specified
//! - Option to not provide data
//! - Right to withdraw consent
//!
//! ## E-Commerce Rules 2020
//!
//! | Requirement | Marketplace | Inventory |
//! |-------------|-------------|-----------|
//! | Seller details | Required | N/A |
//! | Country of origin | Optional | Required |
//! | Return policy | Required | Required |
//! | Grievance officer | Required | Required |
//!
//! ## Example: Validate Intermediary Compliance
//!
//! ```rust
//! use legalis_in::it_act::*;
//!
//! // Check intermediary compliance
//! let report = validate_intermediary_compliance(
//!     IntermediaryType::SocialMedia,
//!     10_000_000, // 1 crore users - SSMI threshold exceeded
//!     true,  // has grievance officer
//!     true,  // has privacy policy
//!     true,  // has user agreement
//!     true,  // has compliance officer
//!     true,  // has nodal person
//!     true,  // monthly report filed
//! );
//! assert!(report.compliant);
//! ```
//!
//! ## References
//!
//! - [IT Act, 2000](https://www.meity.gov.in/content/information-technology-act-2000)
//! - [IT Rules, 2021](https://www.meity.gov.in/writereaddata/files/Intermediary_Guidelines_and_Digital_Media_Ethics_Code_Rules-2021.pdf)
//! - [IT (Reasonable Security Practices) Rules, 2011](https://www.meity.gov.in/content/information-technology-reasonable-security-practices-and-procedures-and-sensitive-personal)

#![allow(missing_docs)]

pub mod error;
pub mod types;
pub mod validator;

pub use error::*;
pub use types::*;
pub use validator::*;
