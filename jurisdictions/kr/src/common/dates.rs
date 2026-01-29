//! Korean Date Utilities
//!
//! Date handling utilities for Korean legal contexts
//!
//! # 날짜 유틸리티 / Date Utilities

use chrono::{Datelike, Duration, NaiveDate};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Date-related errors
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum DateError {
    /// Invalid date
    #[error("Invalid date: {0}")]
    InvalidDate(String),

    /// Date calculation error
    #[error("Date calculation error: {0}")]
    CalculationError(String),

    /// Period error
    #[error("Period error: {0}")]
    PeriodError(String),
}

/// Result type for date operations
pub type DateResult<T> = Result<T, DateError>;

/// Legal deadline type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeadlineType {
    /// Calendar days
    CalendarDays,
    /// Working days (excluding weekends and holidays)
    WorkingDays,
    /// Months
    Months,
    /// Years
    Years,
}

/// Calculate deadline from a start date
pub fn calculate_deadline(
    start: NaiveDate,
    period: i32,
    deadline_type: DeadlineType,
) -> DateResult<NaiveDate> {
    match deadline_type {
        DeadlineType::CalendarDays => {
            if period < 0 {
                return Err(DateError::PeriodError(
                    "Period cannot be negative".to_string(),
                ));
            }
            start
                .checked_add_signed(Duration::days(period as i64))
                .ok_or_else(|| DateError::CalculationError("Date overflow".to_string()))
        }
        DeadlineType::WorkingDays => {
            let mut current = start;
            let mut remaining = period;

            while remaining > 0 {
                current = current
                    .checked_add_signed(Duration::days(1))
                    .ok_or_else(|| DateError::CalculationError("Date overflow".to_string()))?;

                if !crate::common::holidays::is_non_working_day(&current) {
                    remaining -= 1;
                }
            }

            Ok(current)
        }
        DeadlineType::Months => {
            let mut year = start.year();
            let mut month = start.month() as i32 + period;
            let day = start.day();

            while month > 12 {
                year += 1;
                month -= 12;
            }

            while month < 1 {
                year -= 1;
                month += 12;
            }

            // Handle day overflow (e.g., Jan 31 + 1 month -> Feb 28/29)
            let mut result_day = day;
            loop {
                if let Some(date) = NaiveDate::from_ymd_opt(year, month as u32, result_day) {
                    return Ok(date);
                }
                if result_day == 0 {
                    return Err(DateError::CalculationError("Invalid date".to_string()));
                }
                result_day -= 1;
            }
        }
        DeadlineType::Years => {
            let target_year = start.year() + period;
            NaiveDate::from_ymd_opt(target_year, start.month(), start.day()).ok_or_else(|| {
                DateError::CalculationError("Invalid date after year addition".to_string())
            })
        }
    }
}

/// Calculate period between two dates in days
pub fn days_between(start: &NaiveDate, end: &NaiveDate) -> i64 {
    (*end - *start).num_days()
}

/// Calculate period between two dates in months (approximate)
pub fn months_between(start: &NaiveDate, end: &NaiveDate) -> i32 {
    let year_diff = end.year() - start.year();
    let month_diff = end.month() as i32 - start.month() as i32;
    year_diff * 12 + month_diff
}

/// Calculate period between two dates in years
pub fn years_between(start: &NaiveDate, end: &NaiveDate) -> i32 {
    end.year() - start.year()
}

/// Check if a date is within a period
pub fn is_within_period(
    date: &NaiveDate,
    period_start: &NaiveDate,
    period_end: &NaiveDate,
) -> bool {
    date >= period_start && date <= period_end
}

/// Common legal periods in Korean law
pub mod periods {
    use super::*;

    /// Labor Standards Act - probation period limit (3 months)
    pub const PROBATION_PERIOD_MONTHS: i32 = 3;

    /// Labor Standards Act - annual leave accrual period (1 year)
    pub const ANNUAL_LEAVE_ACCRUAL_YEARS: i32 = 1;

    /// Labor Standards Act - notice period for termination (30 days)
    pub const TERMINATION_NOTICE_DAYS: i32 = 30;

    /// Commercial Code - objection period for company dissolution (2 months)
    pub const DISSOLUTION_OBJECTION_MONTHS: i32 = 2;

    /// Civil Code - prescription period for general claims (10 years)
    pub const GENERAL_PRESCRIPTION_YEARS: i32 = 10;

    /// Civil Code - short-term prescription (3 years)
    pub const SHORT_PRESCRIPTION_YEARS: i32 = 3;

    /// Commercial Code - commercial claims prescription (5 years)
    pub const COMMERCIAL_PRESCRIPTION_YEARS: i32 = 5;

    /// PIPA - data retention period for consent withdrawal (processing completion)
    pub const PIPA_CONSENT_WITHDRAWAL_IMMEDIATE: bool = true;

    /// Calculate probation period end date
    pub fn probation_end_date(start: NaiveDate) -> DateResult<NaiveDate> {
        calculate_deadline(start, PROBATION_PERIOD_MONTHS, DeadlineType::Months)
    }

    /// Calculate termination notice deadline
    pub fn termination_notice_deadline(notice_date: NaiveDate) -> DateResult<NaiveDate> {
        calculate_deadline(
            notice_date,
            TERMINATION_NOTICE_DAYS,
            DeadlineType::CalendarDays,
        )
    }

    /// Calculate prescription deadline (general)
    pub fn general_prescription_deadline(accrual_date: NaiveDate) -> DateResult<NaiveDate> {
        calculate_deadline(
            accrual_date,
            GENERAL_PRESCRIPTION_YEARS,
            DeadlineType::Years,
        )
    }

    /// Calculate short-term prescription deadline
    pub fn short_prescription_deadline(accrual_date: NaiveDate) -> DateResult<NaiveDate> {
        calculate_deadline(accrual_date, SHORT_PRESCRIPTION_YEARS, DeadlineType::Years)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_deadline_calendar_days() {
        if let Some(start) = NaiveDate::from_ymd_opt(2024, 1, 1) {
            let result = calculate_deadline(start, 30, DeadlineType::CalendarDays);
            assert!(result.is_ok());
            if let Ok(end) = result {
                assert_eq!(days_between(&start, &end), 30);
            }
        }
    }

    #[test]
    fn test_calculate_deadline_months() {
        if let Some(start) = NaiveDate::from_ymd_opt(2024, 1, 15) {
            let result = calculate_deadline(start, 3, DeadlineType::Months);
            assert!(result.is_ok());
            if let Ok(end) = result {
                assert_eq!(end.month(), 4);
                assert_eq!(end.day(), 15);
            }
        }
    }

    #[test]
    fn test_calculate_deadline_years() {
        if let Some(start) = NaiveDate::from_ymd_opt(2024, 1, 1) {
            let result = calculate_deadline(start, 10, DeadlineType::Years);
            assert!(result.is_ok());
            if let Ok(end) = result {
                assert_eq!(end.year(), 2034);
            }
        }
    }

    #[test]
    fn test_days_between() {
        if let (Some(start), Some(end)) = (
            NaiveDate::from_ymd_opt(2024, 1, 1),
            NaiveDate::from_ymd_opt(2024, 1, 31),
        ) {
            assert_eq!(days_between(&start, &end), 30);
        }
    }

    #[test]
    fn test_months_between() {
        if let (Some(start), Some(end)) = (
            NaiveDate::from_ymd_opt(2024, 1, 1),
            NaiveDate::from_ymd_opt(2024, 4, 1),
        ) {
            assert_eq!(months_between(&start, &end), 3);
        }
    }

    #[test]
    fn test_years_between() {
        if let (Some(start), Some(end)) = (
            NaiveDate::from_ymd_opt(2020, 1, 1),
            NaiveDate::from_ymd_opt(2024, 1, 1),
        ) {
            assert_eq!(years_between(&start, &end), 4);
        }
    }

    #[test]
    fn test_is_within_period() {
        if let (Some(start), Some(date), Some(end)) = (
            NaiveDate::from_ymd_opt(2024, 1, 1),
            NaiveDate::from_ymd_opt(2024, 6, 15),
            NaiveDate::from_ymd_opt(2024, 12, 31),
        ) {
            assert!(is_within_period(&date, &start, &end));
        }
    }

    #[test]
    fn test_probation_end_date() {
        if let Some(start) = NaiveDate::from_ymd_opt(2024, 1, 1) {
            let result = periods::probation_end_date(start);
            assert!(result.is_ok());
            if let Ok(end) = result {
                assert_eq!(end.month(), 4);
            }
        }
    }

    #[test]
    fn test_termination_notice_deadline() {
        if let Some(notice) = NaiveDate::from_ymd_opt(2024, 1, 1) {
            let result = periods::termination_notice_deadline(notice);
            assert!(result.is_ok());
            if let Ok(deadline) = result {
                assert_eq!(days_between(&notice, &deadline), 30);
            }
        }
    }
}
