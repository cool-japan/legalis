//! Common utilities for Japanese legal system.
//!
//! Provides shared functionality including:
//! - Working days and holiday calculations
//! - Legal deadline calculations
//! - Date formatting utilities

mod dates;

pub use dates::{
    JapaneseLegalCalendar, calculate_legal_deadline, is_japanese_holiday, is_working_day,
};
