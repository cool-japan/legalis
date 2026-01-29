//! Common Mexican legal utilities

pub mod currency;
pub mod holidays;
pub mod types;

// Re-export common types
pub use currency::{minimum_wage, uma};
pub use holidays::{FederalHoliday, get_federal_holidays, is_federal_holiday};
pub use types::{
    DocumentError, DocumentType, MexicanCurrency, MexicanDate, MexicanDocument, MexicanState,
    Municipality, validate_curp, validate_nss, validate_rfc,
};
