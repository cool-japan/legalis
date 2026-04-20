//! Legalis-I18n: Internationalization support for Legalis-RS.
//!
//! This crate provides multi-language and multi-jurisdiction support:
//! - Translation of legal terms and statutes
//! - Locale-specific legal formatting (dates, currencies, names)
//! - Jurisdiction mapping and legal system classification
//! - Cultural parameter injection for law porting
//! - ICU message format support
//! - Plural rules handling
//! - Date/time, currency, and number formatting

// Trait implementations for all types (Default, Display, etc.)
mod functions;
mod functions_3;
mod functions_4;
mod functions_5;
mod trait_impls;
mod types;
mod types_10;
mod types_11;
mod types_12;
mod types_13;
mod types_3;
mod types_4;
mod types_5;
mod types_6;
mod types_7;
mod types_8;
mod types_9;

pub use functions::*;
pub use functions_3::*;
pub use types::*;
pub use types_3::*;
pub use types_4::*;
pub use types_5::*;
pub use types_6::*;
pub use types_7::*;
pub use types_8::*;
pub use types_9::*;
pub use types_10::*;
pub use types_11::*;
pub use types_12::*;
pub use types_13::*;

#[cfg(test)]
mod tests;
