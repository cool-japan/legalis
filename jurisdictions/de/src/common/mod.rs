//! Common utilities for German legal system.
//!
//! Provides shared functionality including:
//! - Working days and holiday calculations (Feiertage und Arbeitstage)
//! - Legal deadline calculations (Fristberechnung)
//! - Date formatting utilities

mod dates;

pub use dates::{
    GermanLegalCalendar, GermanState, calculate_legal_deadline, is_german_holiday, is_working_day,
};
