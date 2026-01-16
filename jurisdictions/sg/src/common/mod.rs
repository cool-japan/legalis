//! Common utilities for Singapore legal system.
//!
//! Provides shared functionality including:
//! - Working days and holiday calculations (Public Holidays)
//! - Legal deadline calculations
//! - Currency formatting (SGD)
//! - Multi-ethnic name formatting (Chinese, Malay, Indian, Western)

mod currency;
mod dates;
mod names;

pub use dates::{
    SingaporeLegalCalendar, calculate_legal_deadline, is_singapore_holiday, is_working_day,
};

pub use currency::{SingaporeCurrency, format_sgd, format_sgd_cents};

pub use names::{EthnicGroup, SingaporeNameFormatter, SingaporePersonName};
