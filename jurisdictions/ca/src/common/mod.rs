//! Canada Common Module
//!
//! Shared types, utilities, and constants for Canadian law modules.
//!
//! # Overview
//!
//! Canada has a unique legal system:
//! - **Bijural**: Federal law applies both common law and civil law traditions
//! - **Civil Law**: Quebec uses the Civil Code of Québec (derived from French tradition)
//! - **Common Law**: All other provinces follow the English common law tradition
//!
//! # Division of Powers
//!
//! The Constitution Act, 1867 divides powers between federal and provincial governments:
//! - **Section 91**: Federal powers (criminal law, banking, trade, marriage/divorce)
//! - **Section 92**: Provincial powers (property, civil rights, local matters)
//!
//! # Languages
//!
//! Canada is officially bilingual (English and French):
//! - All federal statutes are enacted in both languages
//! - Both versions have equal authority
//! - New Brunswick is the only officially bilingual province

#![allow(missing_docs)]

pub mod calendar;
pub mod types;

pub use calendar::{CanadianCalendar, CanadianTimeZone, Holiday};
pub use types::{
    BilingualRequirement, CaseCitation, Court, JurisdictionalLevel, LegalSystem, OfficialLanguage,
    Province, StatuteReference,
};
