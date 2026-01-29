//! Common utilities for Malaysian jurisdiction.
//!
//! This module provides shared utilities for Malaysian legal modeling:
//!
//! - Currency formatting (Malaysian Ringgit - MYR)
//! - Legal calendar and public holidays
//! - Date calculations (working days, deadlines)

pub mod currency;
pub mod holidays;

pub use currency::{MalaysianCurrency, format_myr, format_myr_cents};
pub use holidays::{
    MalaysianLegalCalendar, MalaysianPublicHoliday, calculate_legal_deadline, is_malaysian_holiday,
    is_working_day,
};
