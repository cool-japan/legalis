//! # Data Protection - LGPD (Lei Geral de Proteção de Dados)
//!
//! Lei nº 13.709/2018 - Brazil's comprehensive data protection law.
//!
//! ## Overview
//!
//! The LGPD is Brazil's GDPR-equivalent law, establishing rules for the
//! processing of personal data by individuals and organizations.
//!
//! ## Key Concepts
//!
//! | Concept | Portuguese | Description |
//! |---------|------------|-------------|
//! | Data Subject | Titular | Natural person whose data is processed |
//! | Controller | Controlador | Decides processing purposes |
//! | Processor | Operador | Processes on behalf of controller |
//! | DPO | Encarregado | Data Protection Officer |
//! | ANPD | ANPD | National Data Protection Authority |
//!
//! ## Legal Bases (Art. 7)
//!
//! The LGPD provides 10 legal bases for processing:
//!
//! | # | Legal Basis | Example |
//! |---|-------------|---------|
//! | I | Consent | Marketing emails |
//! | II | Legal/regulatory obligation | Tax records |
//! | III | Public administration | Government services |
//! | IV | Research (anonymized) | Academic studies |
//! | V | Contract execution | Service delivery |
//! | VI | Legal proceedings | Court cases |
//! | VII | Life/safety protection | Emergency medical care |
//! | VIII | Health (by professionals) | Medical records |
//! | IX | Legitimate interest | Fraud prevention |
//! | X | Credit protection | Credit scoring |
//!
//! ## Data Subject Rights (Art. 18)
//!
//! 1. Confirmation of processing
//! 2. Access to data
//! 3. Correction of inaccurate data
//! 4. Anonymization/blocking/deletion
//! 5. Portability
//! 6. Deletion (with consent)
//! 7. Information about sharing
//! 8. Right to refuse consent
//! 9. Revocation of consent
//!
//! ## Penalties (Art. 52)
//!
//! - Warning with deadline
//! - Simple fine: up to 2% of revenue (max R$ 50M per violation)
//! - Daily fine
//! - Publicization of violation
//! - Blocking of data
//! - Deletion of data
//!
//! ## ANPD Enforcement
//!
//! The National Data Protection Authority (ANPD) enforces the LGPD:
//! - Receives complaints
//! - Conducts investigations
//! - Applies sanctions
//! - Issues guidelines

pub mod error;
pub mod types;
pub mod validator;

pub use error::*;
pub use types::*;
pub use validator::*;
