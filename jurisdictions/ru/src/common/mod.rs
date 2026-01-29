//! Common utilities for Russian jurisdiction.
//!
//! This module provides:
//! - Russian Ruble (RUB) currency formatting
//! - Russian federal holidays
//! - Working day calculations
//! - Legal deadline calculations

pub mod currency;
pub mod holidays;

pub use currency::{Currency, format_ruble};
pub use holidays::{
    Holiday, RussianLegalCalendar, calculate_business_days, is_russian_holiday, is_working_day,
};
