//! Constitution of India
//!
//! # Overview
//!
//! The Constitution of India is the supreme law of India, adopted on 26th November
//! 1949 and came into effect on 26th January 1950. It is the longest written
//! constitution of any country in the world.
//!
//! ## Key Features
//!
//! - **Sovereign, Socialist, Secular, Democratic Republic** (Preamble)
//! - **Parliamentary form of government**
//! - **Federal structure with unitary features**
//! - **Independent judiciary with power of judicial review**
//! - **Fundamental Rights (Part III)**
//! - **Directive Principles of State Policy (Part IV)**
//! - **Fundamental Duties (Part IVA)**
//!
//! ## Structure
//!
//! | Parts | Subject | Articles |
//! |-------|---------|----------|
//! | Part I | Union and Territory | 1-4 |
//! | Part II | Citizenship | 5-11 |
//! | Part III | Fundamental Rights | 12-35 |
//! | Part IV | Directive Principles | 36-51 |
//! | Part IVA | Fundamental Duties | 51A |
//! | Part V | The Union | 52-151 |
//! | Part VI | The States | 152-237 |
//! | Part XVIII | Emergency | 352-360 |
//! | Part XX | Amendment | 368 |
//!
//! ## Fundamental Rights
//!
//! ```rust
//! use legalis_in::constitution::*;
//!
//! // Check fundamental right
//! let right = FundamentalRight::RightToLife;
//! assert_eq!(right.article(), "21");
//!
//! // Check if suspendable during emergency
//! assert!(!right.suspendable_during_emergency()); // Art 21 cannot be suspended
//! ```
//!
//! ## Writs under Article 32/226
//!
//! | Writ | Latin Meaning | Purpose |
//! |------|---------------|---------|
//! | Habeas Corpus | Produce the body | Personal liberty |
//! | Mandamus | We command | Compel public duty |
//! | Prohibition | To forbid | Stop inferior court |
//! | Certiorari | To be informed | Quash void order |
//! | Quo Warranto | By what authority | Challenge office holder |
//!
//! ```rust
//! use legalis_in::constitution::*;
//!
//! let writ = WritType::HabeasCorpus;
//! assert_eq!(writ.meaning(), "produce the body");
//! assert!(writ.purpose().contains("Personal liberty"));
//! ```
//!
//! ## Amendment Procedure (Article 368)
//!
//! | Category | Procedure | Examples |
//! |----------|-----------|----------|
//! | Simple | Simple majority | Admission of new states |
//! | Special | 2/3 + majority | Most amendments |
//! | Special + Ratification | 2/3 + majority + half states | Federal provisions |
//!
//! ## Basic Structure Doctrine
//!
//! The Supreme Court in **Kesavananda Bharati v. State of Kerala (1973)** held
//! that Parliament cannot amend the Constitution to destroy its basic structure.
//!
//! Recognized features include:
//! - Supremacy of the Constitution
//! - Republican and democratic form
//! - Secular character
//! - Separation of powers
//! - Federal character
//! - Judicial review
//! - Rule of law
//! - Free and fair elections
//!
//! ## Emergency Provisions
//!
//! | Type | Article | Duration | Approval |
//! |------|---------|----------|----------|
//! | National | 352 | Unlimited | Cabinet + Parliament |
//! | State | 356 | Max 3 years | Parliament |
//! | Financial | 360 | Unlimited | Parliament |
//!
//! ```rust
//! use legalis_in::constitution::*;
//!
//! let emergency = EmergencyType::PresidentsRule;
//! assert_eq!(emergency.article(), 356);
//! assert_eq!(emergency.max_duration_months(), Some(36)); // 3 years
//! ```
//!
//! ## Article 19 Freedoms and Restrictions
//!
//! | Freedom | Clause | Reasonable Restrictions |
//! |---------|--------|------------------------|
//! | Speech | 19(1)(a) | Security, public order, defamation |
//! | Assembly | 19(1)(b) | Sovereignty, public order |
//! | Association | 19(1)(c) | Sovereignty, morality |
//! | Movement | 19(1)(d) | General public, tribal protection |
//! | Residence | 19(1)(e) | General public, tribal protection |
//! | Profession | 19(1)(g) | General public, qualifications |
//!
//! ## Landmark Cases
//!
//! | Case | Year | Principle |
//! |------|------|-----------|
//! | Kesavananda Bharati | 1973 | Basic Structure Doctrine |
//! | Maneka Gandhi | 1978 | Due Process under Art 21 |
//! | Minerva Mills | 1980 | Judicial Review part of Basic Structure |
//! | S.R. Bommai | 1994 | Secularism is Basic Structure |
//! | L. Chandra Kumar | 1997 | Judicial Review cannot be excluded |
//!
//! ## Public Interest Litigation
//!
//! PIL evolved through judicial innovation to provide access to justice for
//! disadvantaged groups. Key features:
//! - Relaxed locus standi
//! - Letter petitions
//! - Suo motu cognizance
//! - Continuing mandamus
//!
//! ## References
//!
//! - [Constitution of India](https://legislative.gov.in/constitution-of-india/)
//! - [Constituent Assembly Debates](https://eparlib.nic.in/handle/123456789/1)
//! - [Supreme Court of India](https://main.sci.gov.in/)

pub mod error;
pub mod types;
pub mod validator;

pub use error::*;
pub use types::*;
pub use validator::*;
