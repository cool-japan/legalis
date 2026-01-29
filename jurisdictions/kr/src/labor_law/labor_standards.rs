//! Labor Standards Act (근로기준법)
//!
//! # 근로기준법 / Labor Standards Act
//!
//! Enacted: 1953
//! Last amendment: 2023
//!
//! Key provisions:
//! - Maximum 40 hours per week (Article 50)
//! - Maximum 12 hours overtime per week (Article 53)
//! - Severance pay (퇴직금) after 1 year (Article 34)
//! - Annual leave
//! - Minimum wage

use crate::common::KrwAmount;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Labor Standards Act errors
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum LaborStandardsError {
    /// Invalid employment contract
    #[error("Invalid employment contract: {0}")]
    InvalidContract(String),

    /// Working hours violation
    #[error("Working hours violation: {0}")]
    WorkingHoursViolation(String),

    /// Wage calculation error
    #[error("Wage calculation error: {0}")]
    WageError(String),

    /// Severance pay error
    #[error("Severance pay error: {0}")]
    SeveranceError(String),
}

/// Result type for labor standards operations
pub type LaborStandardsResult<T> = Result<T, LaborStandardsError>;

/// Employment contract (근로계약)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmploymentContract {
    /// Employer (사용자)
    pub employer: String,
    /// Employee (근로자)
    pub employee: String,
    /// Start date
    pub start_date: NaiveDate,
    /// End date (if fixed-term)
    pub end_date: Option<NaiveDate>,
    /// Base salary
    pub base_salary: KrwAmount,
    /// Working hours per week
    pub working_hours_per_week: u32,
}

impl EmploymentContract {
    /// Create new employment contract
    pub fn new(
        employer: impl Into<String>,
        employee: impl Into<String>,
        start_date: NaiveDate,
        base_salary: KrwAmount,
        working_hours_per_week: u32,
    ) -> Self {
        Self {
            employer: employer.into(),
            employee: employee.into(),
            start_date,
            end_date: None,
            base_salary,
            working_hours_per_week,
        }
    }

    /// Set end date (for fixed-term contract)
    pub fn with_end_date(mut self, end_date: NaiveDate) -> Self {
        self.end_date = Some(end_date);
        self
    }

    /// Check if contract is fixed-term
    pub fn is_fixed_term(&self) -> bool {
        self.end_date.is_some()
    }

    /// Calculate service years
    pub fn service_years(&self) -> f64 {
        let today = chrono::Utc::now().date_naive();
        let days = (today - self.start_date).num_days();
        days as f64 / 365.25
    }
}

/// Validate working hours
/// Article 50: Maximum 40 hours per week
/// Article 53: Maximum 12 hours overtime per week
pub fn validate_working_hours(regular_hours: u32, overtime_hours: u32) -> LaborStandardsResult<()> {
    if regular_hours > 40 {
        return Err(LaborStandardsError::WorkingHoursViolation(
            "Regular working hours cannot exceed 40 hours per week".to_string(),
        ));
    }

    if overtime_hours > 12 {
        return Err(LaborStandardsError::WorkingHoursViolation(
            "Overtime hours cannot exceed 12 hours per week".to_string(),
        ));
    }

    let total = regular_hours + overtime_hours;
    if total > 52 {
        return Err(LaborStandardsError::WorkingHoursViolation(
            "Total working hours cannot exceed 52 hours per week".to_string(),
        ));
    }

    Ok(())
}

/// Calculate overtime pay
/// Article 56: Overtime is paid at 150% of normal wage
pub fn calculate_overtime_pay(
    hourly_wage: &KrwAmount,
    overtime_hours: f64,
) -> LaborStandardsResult<KrwAmount> {
    if overtime_hours < 0.0 {
        return Err(LaborStandardsError::WageError(
            "Overtime hours cannot be negative".to_string(),
        ));
    }

    let overtime_rate = 1.5;
    let overtime_pay = hourly_wage.multiply(overtime_hours * overtime_rate);

    Ok(overtime_pay)
}

/// Calculate severance pay (퇴직금)
/// Article 34: One month's average wage for each year of service
/// Applicable to employees with 1+ years of service
pub fn calculate_severance_pay(
    average_monthly_wage: &KrwAmount,
    service_years: f64,
) -> LaborStandardsResult<KrwAmount> {
    if service_years < 1.0 {
        return Err(LaborStandardsError::SeveranceError(
            "Severance pay not applicable for service less than 1 year".to_string(),
        ));
    }

    let severance = average_monthly_wage.multiply(service_years);
    Ok(severance)
}

/// Annual leave entitlement (연차휴가)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnnualLeave {
    /// Service years
    pub service_years: u32,
    /// Days entitled
    pub days: u32,
}

/// Calculate annual leave entitlement
/// Article 60:
/// - 1 year: 15 days
/// - 2+ years: +1 day per 2 years (max 25 days)
pub fn calculate_annual_leave(service_years: u32) -> AnnualLeave {
    let days = if service_years == 0 {
        0
    } else if service_years == 1 {
        15
    } else {
        let additional = (service_years / 2).min(10);
        15 + additional
    };

    AnnualLeave {
        service_years,
        days,
    }
}

/// Validate minimum wage compliance
pub fn validate_minimum_wage(
    hourly_wage: &KrwAmount,
    minimum_wage: &KrwAmount,
) -> LaborStandardsResult<()> {
    if hourly_wage.won < minimum_wage.won {
        return Err(LaborStandardsError::WageError(format!(
            "Hourly wage {} is below minimum wage {}",
            hourly_wage.format_korean(),
            minimum_wage.format_korean()
        )));
    }

    Ok(())
}

/// Probation period limit
pub mod probation {
    /// Maximum probation period - 3 months
    pub const MAX_PROBATION_MONTHS: u32 = 3;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_employment_contract_creation() {
        if let Some(date) = NaiveDate::from_ymd_opt(2024, 1, 1) {
            let contract =
                EmploymentContract::new("회사", "김철수", date, KrwAmount::from_man(300.0), 40);

            assert_eq!(contract.employer, "회사");
            assert_eq!(contract.working_hours_per_week, 40);
        }
    }

    #[test]
    fn test_validate_working_hours() {
        let result = validate_working_hours(40, 10);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_working_hours_violation() {
        let result = validate_working_hours(45, 10);
        assert!(result.is_err());
    }

    #[test]
    fn test_calculate_overtime_pay() {
        let hourly_wage = KrwAmount::new(10_000.0);
        let result = calculate_overtime_pay(&hourly_wage, 10.0);
        assert!(result.is_ok());

        if let Ok(overtime_pay) = result {
            // 10,000 * 10 * 1.5 = 150,000
            assert!((overtime_pay.won - 150_000.0).abs() < 0.01);
        }
    }

    #[test]
    fn test_calculate_severance_pay() {
        let monthly_wage = KrwAmount::from_man(300.0);
        let result = calculate_severance_pay(&monthly_wage, 5.0);
        assert!(result.is_ok());

        if let Ok(severance) = result {
            // 3,000,000 * 5 = 15,000,000
            assert!((severance.won - 15_000_000.0).abs() < 0.01);
        }
    }

    #[test]
    fn test_calculate_severance_pay_insufficient_service() {
        let monthly_wage = KrwAmount::from_man(300.0);
        let result = calculate_severance_pay(&monthly_wage, 0.5);
        assert!(result.is_err());
    }

    #[test]
    fn test_calculate_annual_leave() {
        let leave1 = calculate_annual_leave(1);
        assert_eq!(leave1.days, 15);

        let leave3 = calculate_annual_leave(3);
        assert_eq!(leave3.days, 16);

        let leave11 = calculate_annual_leave(11);
        assert_eq!(leave11.days, 20);

        let leave20 = calculate_annual_leave(20);
        assert_eq!(leave20.days, 25); // Max
    }

    #[test]
    fn test_validate_minimum_wage() {
        let hourly_wage = KrwAmount::new(10_000.0);
        let minimum_wage = KrwAmount::new(9_860.0);
        let result = validate_minimum_wage(&hourly_wage, &minimum_wage);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_minimum_wage_violation() {
        let hourly_wage = KrwAmount::new(9_000.0);
        let minimum_wage = KrwAmount::new(9_860.0);
        let result = validate_minimum_wage(&hourly_wage, &minimum_wage);
        assert!(result.is_err());
    }
}
