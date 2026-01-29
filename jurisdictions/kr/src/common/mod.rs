//! Common Utilities for Korean Jurisdiction
//!
//! # 공통 유틸리티 / Common Utilities
//!
//! Shared utilities for Korean legal system including:
//! - Currency (KRW)
//! - Dates and deadlines
//! - Public holidays
//! - Names (personal and company)

pub mod currency;
pub mod dates;
pub mod holidays;
pub mod names;

pub use currency::{KrwAmount, thresholds};
pub use dates::{
    DateError, DateResult, DeadlineType, calculate_deadline, days_between, is_within_period,
    months_between, periods, years_between,
};
pub use holidays::{
    HolidayType, PublicHoliday, count_working_days as count_working_days_holidays,
    get_public_holidays, is_non_working_day, is_public_holiday, is_weekend,
    next_working_day as next_working_day_holidays,
};
pub use names::{CompanyName, KoreanName, OrganizationForm};
