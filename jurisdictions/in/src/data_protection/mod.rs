//! Digital Personal Data Protection Act, 2023 Module
//!
//! # DPDPA 2023
//!
//! India's comprehensive data protection legislation effective from 2024.
//!
//! ## Key Concepts
//!
//! - **Data Principal**: Individual whose personal data is processed
//! - **Data Fiduciary**: Entity determining purpose and means of processing
//! - **Data Processor**: Entity processing data on behalf of Data Fiduciary
//! - **Consent Manager**: Registered entity enabling consent management
//! - **Significant Data Fiduciary (SDF)**: Large processors with additional obligations
//!
//! ## Lawful Processing (Section 4-7)
//!
//! Personal data may be processed only:
//! 1. With consent of data principal (Section 6)
//! 2. For certain legitimate uses (Section 7)
//!
//! ## Data Principal Rights (Chapter III)
//!
//! | Right | Section | Description |
//! |-------|---------|-------------|
//! | Access | 11 | Summary of personal data and processing |
//! | Correction | 12 | Correct inaccurate/misleading data |
//! | Erasure | 12 | Erase data no longer necessary |
//! | Grievance | 13 | Redressal mechanism |
//! | Nomination | 14 | Nominate for incapacity/death |
//!
//! ## Data Principal Duties (Section 15)
//!
//! - Comply with law when exercising rights
//! - Not register false/frivolous grievances
//! - Not file false/frivolous complaints
//! - Not impersonate another person
//! - Not suppress material information
//! - Provide authentic information
//!
//! ## Data Fiduciary Obligations
//!
//! ### General Obligations
//!
//! - Give notice before obtaining consent (Section 5)
//! - Implement reasonable security safeguards (Section 8(5))
//! - Notify breach to Board and data principals (Section 8(6))
//! - Erase data when no longer necessary (Section 8(7))
//! - Respond to data principal requests
//!
//! ### Significant Data Fiduciary (Section 10)
//!
//! Additional obligations:
//! - Appoint Data Protection Officer based in India
//! - Appoint independent data auditor
//! - Conduct periodic DPIA
//! - Undertake periodic audits
//!
//! ## Child's Data (Section 9)
//!
//! - Requires verifiable parental consent
//! - No processing detrimental to well-being
//! - No tracking or behavioral monitoring
//! - No targeted advertising
//!
//! ## Cross-Border Transfer (Section 16)
//!
//! - Transfer allowed except to countries restricted by Central Government
//! - Restriction based on factors including national security
//!
//! ## Penalties (Section 33)
//!
//! | Tier | Maximum Penalty | Violations |
//! |------|-----------------|------------|
//! | 1 | Rs. 50 crore | Security safeguard failures |
//! | 2 | Rs. 200 crore | Breach notification, child data violations |
//! | 3 | Rs. 250 crore | Processing without lawful grounds, SDF violations |
//!
//! ## Data Protection Board
//!
//! - Established under Section 18
//! - Adjudicates complaints
//! - Imposes penalties
//! - Issues directions

#![allow(missing_docs)]

pub mod error;
pub mod types;
pub mod validator;

pub use error::*;
pub use types::*;
pub use validator::*;
