//! Common utilities for UK legal system.
//!
//! Provides shared functionality including:
//! - Working days and bank holiday calculations
//! - Legal deadline calculations
//! - UK timezone handling (GMT/BST)
//! - Date formatting utilities

mod dates;
mod timezone;

pub use dates::{
    UkLegalCalendar, UkRegion, calculate_legal_deadline, is_bank_holiday, is_working_day,
};

pub use timezone::{UkTimeZone, convert_to_uk_local, current_uk_offset, is_bst};
