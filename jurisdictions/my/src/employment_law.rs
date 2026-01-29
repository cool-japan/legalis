//! Employment Act 1955
//!
//! Malaysian employment law governing employment contracts, working hours, wages, and termination.
//!
//! # Key Provisions
//!
//! - **Section 60D**: Working hours - 8 hours/day, 48 hours/week
//! - **Section 60**: Overtime pay - 1.5x regular rate
//! - **Section 60E**: Annual leave - minimum 8 days (< 2 years service) to 16 days (>= 5 years)
//! - **Section 60F**: Sick leave - 14 days (outpatient) + 60 days (hospitalization)
//! - **Section 12**: Termination notice periods
//!
//! # EPF (Employees Provident Fund)
//!
//! Mandatory retirement savings:
//! - Employer contribution: 12% (salary < RM 5,000) or 13% (salary >= RM 5,000)
//! - Employee contribution: 11%
//! - Wage ceiling: RM 5,000/month

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

/// Employment law error types.
#[derive(Debug, Error)]
pub enum EmploymentError {
    /// Invalid employment contract.
    #[error("Invalid employment contract: {reason}")]
    InvalidContract { reason: String },

    /// Working hours violation.
    #[error("Working hours violation: {reason}")]
    WorkingHoursViolation { reason: String },

    /// Wage violation.
    #[error("Wage violation: {reason}")]
    WageViolation { reason: String },

    /// Leave entitlement issue.
    #[error("Leave entitlement issue: {reason}")]
    LeaveIssue { reason: String },
}

/// Result type for employment law operations.
pub type Result<T> = std::result::Result<T, EmploymentError>;

/// Working hours configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkingHours {
    /// Hours per day.
    pub hours_per_day: u8,
    /// Hours per week.
    pub hours_per_week: u8,
    /// Whether employee works shifts.
    pub shift_work: bool,
}

impl WorkingHours {
    /// Creates standard working hours (8h/day, 48h/week).
    #[must_use]
    pub fn standard() -> Self {
        Self {
            hours_per_day: 8,
            hours_per_week: 48,
            shift_work: false,
        }
    }

    /// Validates working hours under Section 60D.
    #[must_use]
    pub fn is_compliant(&self) -> bool {
        self.hours_per_day <= 8 && self.hours_per_week <= 48
    }
}

/// Leave entitlement based on years of service.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LeaveEntitlement {
    /// Years of service.
    pub years_of_service: u8,
    /// Annual leave days.
    pub annual_leave_days: u8,
    /// Sick leave days (outpatient).
    pub sick_leave_outpatient: u8,
    /// Sick leave days (hospitalization).
    pub sick_leave_hospitalization: u8,
}

impl LeaveEntitlement {
    /// Calculates leave entitlement based on years of service (Section 60E).
    #[must_use]
    pub fn calculate(years_of_service: u8) -> Self {
        let annual_leave_days = if years_of_service < 2 {
            8
        } else if years_of_service < 5 {
            12
        } else {
            16
        };

        Self {
            years_of_service,
            annual_leave_days,
            sick_leave_outpatient: 14,
            sick_leave_hospitalization: 60,
        }
    }
}

/// EPF (Employees Provident Fund) contribution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EpfContribution {
    /// Employee age.
    pub age: u8,
    /// Monthly salary in sen.
    pub monthly_salary_sen: i64,
    /// Employer contribution rate (%).
    pub employer_rate: f64,
    /// Employee contribution rate (%).
    pub employee_rate: f64,
}

impl EpfContribution {
    /// Creates a new EPF contribution calculation.
    #[must_use]
    pub fn new(age: u8, monthly_salary_sen: i64) -> Self {
        // Standard rates for employees under 60
        let employer_rate = if monthly_salary_sen >= 500_000 {
            13.0
        } else {
            12.0
        };
        let employee_rate = 11.0;

        Self {
            age,
            monthly_salary_sen,
            employer_rate,
            employee_rate,
        }
    }

    /// Calculates the breakdown.
    #[must_use]
    pub fn calculate(&self) -> EpfBreakdown {
        // Apply wage ceiling of RM 5,000
        let capped_salary = self.monthly_salary_sen.min(500_000);

        let employer_amount_sen =
            ((capped_salary as f64) * (self.employer_rate / 100.0)).round() as i64;
        let employee_amount_sen =
            ((capped_salary as f64) * (self.employee_rate / 100.0)).round() as i64;

        EpfBreakdown {
            employer_amount_sen,
            employee_amount_sen,
            total_amount_sen: employer_amount_sen + employee_amount_sen,
        }
    }
}

/// EPF contribution breakdown.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EpfBreakdown {
    /// Employer contribution in sen.
    pub employer_amount_sen: i64,
    /// Employee contribution in sen.
    pub employee_amount_sen: i64,
    /// Total contribution in sen.
    pub total_amount_sen: i64,
}

impl EpfBreakdown {
    /// Returns employer amount in RM.
    #[must_use]
    pub fn employer_amount(&self) -> f64 {
        self.employer_amount_sen as f64 / 100.0
    }

    /// Returns employee amount in RM.
    #[must_use]
    pub fn employee_amount(&self) -> f64 {
        self.employee_amount_sen as f64 / 100.0
    }

    /// Returns total amount in RM.
    #[must_use]
    pub fn total_amount(&self) -> f64 {
        self.total_amount_sen as f64 / 100.0
    }
}

/// Employment contract.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmploymentContract {
    /// Contract ID.
    pub id: Uuid,
    /// Employer name.
    pub employer: String,
    /// Employee name.
    pub employee: String,
    /// Employee IC number.
    pub employee_ic: String,
    /// Job title.
    pub job_title: String,
    /// Monthly salary in sen.
    pub monthly_salary_sen: i64,
    /// Working hours.
    pub working_hours: WorkingHours,
    /// Leave entitlement.
    pub leave_entitlement: LeaveEntitlement,
    /// Contract start date.
    pub start_date: DateTime<Utc>,
    /// Notice period in days.
    pub notice_period_days: u8,
}

impl EmploymentContract {
    /// Creates an employment contract builder.
    #[must_use]
    pub fn builder() -> EmploymentContractBuilder {
        EmploymentContractBuilder::default()
    }

    /// Validates the employment contract.
    pub fn validate(&self) -> Result<ValidationReport> {
        validate_employment_contract(self)
    }

    /// Calculates EPF contribution for this contract.
    #[must_use]
    pub fn calculate_epf(&self, employee_age: u8) -> EpfContribution {
        EpfContribution::new(employee_age, self.monthly_salary_sen)
    }
}

/// Employment contract builder.
#[derive(Debug, Clone, Default)]
pub struct EmploymentContractBuilder {
    employer: Option<String>,
    employee: Option<String>,
    employee_ic: Option<String>,
    job_title: Option<String>,
    monthly_salary_sen: Option<i64>,
    working_hours: Option<WorkingHours>,
    leave_entitlement: Option<LeaveEntitlement>,
    notice_period_days: u8,
}

impl EmploymentContractBuilder {
    /// Sets the employer.
    #[must_use]
    pub fn employer(mut self, employer: impl Into<String>) -> Self {
        self.employer = Some(employer.into());
        self
    }

    /// Sets the employee.
    #[must_use]
    pub fn employee(mut self, employee: impl Into<String>, ic: impl Into<String>) -> Self {
        self.employee = Some(employee.into());
        self.employee_ic = Some(ic.into());
        self
    }

    /// Sets the job title.
    #[must_use]
    pub fn job_title(mut self, job_title: impl Into<String>) -> Self {
        self.job_title = Some(job_title.into());
        self
    }

    /// Sets the monthly salary.
    #[must_use]
    pub fn monthly_salary_sen(mut self, salary_sen: i64) -> Self {
        self.monthly_salary_sen = Some(salary_sen);
        self
    }

    /// Sets the working hours.
    #[must_use]
    pub fn working_hours(mut self, working_hours: WorkingHours) -> Self {
        self.working_hours = Some(working_hours);
        self
    }

    /// Sets the leave entitlement.
    #[must_use]
    pub fn leave_entitlement(mut self, leave: LeaveEntitlement) -> Self {
        self.leave_entitlement = Some(leave);
        self
    }

    /// Sets the notice period.
    #[must_use]
    pub fn notice_period_days(mut self, days: u8) -> Self {
        self.notice_period_days = days;
        self
    }

    /// Builds the contract.
    pub fn build(self) -> Result<EmploymentContract> {
        Ok(EmploymentContract {
            id: Uuid::new_v4(),
            employer: self
                .employer
                .ok_or_else(|| EmploymentError::InvalidContract {
                    reason: "Employer not specified".to_string(),
                })?,
            employee: self
                .employee
                .ok_or_else(|| EmploymentError::InvalidContract {
                    reason: "Employee not specified".to_string(),
                })?,
            employee_ic: self
                .employee_ic
                .ok_or_else(|| EmploymentError::InvalidContract {
                    reason: "Employee IC not specified".to_string(),
                })?,
            job_title: self
                .job_title
                .ok_or_else(|| EmploymentError::InvalidContract {
                    reason: "Job title not specified".to_string(),
                })?,
            monthly_salary_sen: self.monthly_salary_sen.ok_or_else(|| {
                EmploymentError::InvalidContract {
                    reason: "Monthly salary not specified".to_string(),
                }
            })?,
            working_hours: self.working_hours.unwrap_or_else(WorkingHours::standard),
            leave_entitlement: self
                .leave_entitlement
                .unwrap_or_else(|| LeaveEntitlement::calculate(0)),
            start_date: Utc::now(),
            notice_period_days: self.notice_period_days,
        })
    }
}

/// Validation report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    /// Whether contract is valid.
    pub valid: bool,
    /// Issues found.
    pub issues: Vec<String>,
}

/// Validates an employment contract under Employment Act 1955.
pub fn validate_employment_contract(contract: &EmploymentContract) -> Result<ValidationReport> {
    let mut issues = Vec::new();

    // Validate working hours (Section 60D)
    if !contract.working_hours.is_compliant() {
        issues.push(format!(
            "Working hours exceed legal limit: {}h/day, {}h/week (max 8h/day, 48h/week)",
            contract.working_hours.hours_per_day, contract.working_hours.hours_per_week
        ));
    }

    // Validate minimum wage (as of 2024: RM 1,500/month)
    if contract.monthly_salary_sen < 150_000 {
        issues.push("Salary below minimum wage (RM 1,500/month)".to_string());
    }

    let valid = issues.is_empty();

    Ok(ValidationReport { valid, issues })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_employment_contract() {
        let contract = EmploymentContract::builder()
            .employer("Tech Innovations Sdn Bhd")
            .employee("Ahmad bin Ali", "850123-01-5678")
            .job_title("Software Engineer")
            .monthly_salary_sen(500_000) // RM 5,000
            .working_hours(WorkingHours::standard())
            .leave_entitlement(LeaveEntitlement::calculate(3))
            .notice_period_days(30)
            .build()
            .expect("Valid contract");

        let report = contract.validate().expect("Validation succeeds");
        assert!(report.valid);
    }

    #[test]
    fn test_epf_calculation() {
        let epf = EpfContribution::new(30, 300_000); // RM 3,000, age 30
        let breakdown = epf.calculate();

        assert_eq!(breakdown.employer_amount_sen, 36_000); // 12% of RM 3,000 = RM 360
        assert_eq!(breakdown.employee_amount_sen, 33_000); // 11% of RM 3,000 = RM 330
    }

    #[test]
    fn test_leave_entitlement() {
        let leave1 = LeaveEntitlement::calculate(1);
        assert_eq!(leave1.annual_leave_days, 8);

        let leave3 = LeaveEntitlement::calculate(3);
        assert_eq!(leave3.annual_leave_days, 12);

        let leave6 = LeaveEntitlement::calculate(6);
        assert_eq!(leave6.annual_leave_days, 16);
    }
}
