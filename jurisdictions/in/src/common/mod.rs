//! Common utilities for Indian legal system
//!
//! This module provides fundamental utilities used across all Indian legal domains:
//! - Currency formatting (lakhs/crores)
//! - Date calculations (working days, national holidays)
//! - Name formatting (Indian naming conventions)
//! - Common types (states, addresses)

pub mod currency;
pub mod dates;
pub mod names;
pub mod types;

// Re-export commonly used items
pub use currency::{
    InrAmount, crores_to_rupees, format_crores, format_inr, format_inr_precision, format_lakhs,
    lakhs_to_rupees, parse_inr, to_crores, to_lakhs,
};

pub use dates::{
    DeadlineType, FinancialYear, NationalHoliday, business_days_between, calculate_deadline,
    is_national_holiday, is_weekend, is_working_day, national_holidays_in_year, next_working_day,
    previous_working_day, working_days_between,
};

pub use names::{IndianName, IndianNameFormatter, Title};

pub use types::{Address, State};
