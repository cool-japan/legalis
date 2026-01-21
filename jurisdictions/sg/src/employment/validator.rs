//! Employment Act (Cap. 91) - Validation Logic
//!
//! This module provides validation functions for Singapore Employment Act compliance.
//!
//! ## Key Validations
//!
//! 1. **Working Hours** (s. 38)
//!    - Max 44 hours/week for non-shift workers
//!    - Max 48 hours/week for shift workers
//!    - Max 12 hours/day
//!    - Minimum 1 rest day per week
//!
//! 2. **Overtime** (s. 38(4))
//!    - Minimum 1.5x regular hourly rate
//!
//! 3. **Leave Entitlement** (s. 43)
//!    - Annual leave: 7 days (year 1) → 14 days (year 8+)
//!    - Sick leave: 14 days outpatient + 60 days hospitalization (after 3 months)
//!
//! 4. **Termination Notice** (s. 10/11)
//!    - 1 day (<26 weeks) → 4 weeks (5+ years)
//!
//! 5. **CPF Contributions**
//!    - Age-based rates (17%/20% for age ≤55)
//!    - Wage ceiling SGD 6,000/month

use super::error::{EmploymentError, Result};
use super::types::*;
use chrono::{DateTime, Utc};

/// Validation report for employment contracts
#[derive(Debug, Clone, PartialEq)]
pub struct ValidationReport {
    /// Whether the contract is valid
    pub is_valid: bool,
    /// Validation warnings (non-fatal issues)
    pub warnings: Vec<String>,
    /// Validation errors (fatal issues)
    pub errors: Vec<String>,
    /// EA coverage status
    pub ea_covered: bool,
    /// CPF applicability
    pub cpf_applicable: bool,
}

impl ValidationReport {
    /// Create a new validation report
    pub fn new() -> Self {
        Self {
            is_valid: true,
            warnings: Vec::new(),
            errors: Vec::new(),
            ea_covered: false,
            cpf_applicable: false,
        }
    }

    /// Add a warning
    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }

    /// Add an error (marks report as invalid)
    pub fn add_error(&mut self, error: String) {
        self.is_valid = false;
        self.errors.push(error);
    }
}

impl Default for ValidationReport {
    fn default() -> Self {
        Self::new()
    }
}

/// Validates an employment contract for EA compliance
pub fn validate_employment_contract(contract: &EmploymentContract) -> Result<ValidationReport> {
    let mut report = ValidationReport::new();

    // 1. Validate contract dates
    if let Some(end_date) = contract.end_date
        && end_date <= contract.start_date
    {
        return Err(EmploymentError::InvalidContractDates);
    }

    // 2. Set EA coverage from contract
    report.ea_covered = contract.covered_by_ea;

    // 3. Set CPF applicability from contract
    report.cpf_applicable = contract.cpf_applicable;

    // 4. Validate working hours
    if let Err(e) = validate_working_hours(&contract.working_hours) {
        report.add_error(format!("{}", e));
    }

    // 5. Warn if contract type is fixed-term without end date
    if contract.contract_type == ContractType::FixedTerm && contract.end_date.is_none() {
        report.add_warning("Fixed-term contract should have an end date specified".to_string());
    }

    // 6. Validate allowances
    if !contract.allowances.is_empty() {
        let total_allowances: u64 = contract.allowances.iter().map(|a| a.amount_cents).sum();
        let total_allowances_sgd = total_allowances as f64 / 100.0;
        let basic_salary_sgd = contract.basic_salary_cents as f64 / 100.0;
        if total_allowances_sgd > basic_salary_sgd {
            report.add_warning(format!(
                "Total allowances (SGD {:.2}) exceed base salary (SGD {:.2})",
                total_allowances_sgd, basic_salary_sgd
            ));
        }
    }

    Ok(report)
}

/// Validates working hours against Employment Act s. 38 requirements
pub fn validate_working_hours(hours: &WorkingHours) -> Result<()> {
    // Check weekly hours limit (s. 38)
    let limit = if hours.is_shift_work { 48.0 } else { 44.0 };

    if hours.hours_per_week > limit {
        return Err(EmploymentError::ExcessiveWorkingHours {
            actual: hours.hours_per_week,
            limit,
        });
    }

    // Check daily hours limit (s. 38)
    if hours.hours_per_day > 12.0 {
        return Err(EmploymentError::ExcessiveDailyHours {
            actual: hours.hours_per_day,
        });
    }

    // Check rest days (s. 38)
    let required_rest_days = 1;
    if hours.rest_days_per_week < required_rest_days {
        return Err(EmploymentError::InsufficientRestDays {
            actual: hours.rest_days_per_week,
            required: required_rest_days,
        });
    }

    Ok(())
}

/// Validates and calculates overtime payment
pub fn validate_overtime_payment(
    overtime_hours: f64,
    rate_multiplier: f64,
    hourly_rate_cents: u64,
) -> Result<u64> {
    const MINIMUM_RATE: f64 = 1.5;

    if rate_multiplier < MINIMUM_RATE {
        return Err(EmploymentError::OvertimeRateBelowMinimum {
            actual: rate_multiplier,
            required: MINIMUM_RATE,
        });
    }

    let overtime_pay = (overtime_hours * rate_multiplier * hourly_rate_cents as f64) as u64;
    Ok(overtime_pay)
}

/// Validates leave entitlement based on years of service
pub fn validate_leave_entitlement(
    years_of_service: u32,
    contract_type: ContractType,
) -> Result<LeaveEntitlement> {
    // Fixed-term contracts < 3 months may have prorated leave
    if contract_type == ContractType::FixedTerm && years_of_service == 0 {
        return Err(EmploymentError::InvalidLeaveEntitlement {
            reason: "Fixed-term contract less than 3 months may have prorated leave".to_string(),
        });
    }

    let leave = LeaveEntitlement::new(years_of_service);

    // Validate against statutory minimum (s. 43)
    let statutory_minimum = match years_of_service {
        0 => 7,
        1 => 8,
        2..=3 => 9,
        4..=5 => 11,
        6..=7 => 12,
        _ => 14,
    };

    if leave.annual_leave_days < statutory_minimum {
        return Err(EmploymentError::AnnualLeaveCalculationError {
            years: years_of_service,
            calculated: leave.annual_leave_days,
            expected: statutory_minimum,
        });
    }

    Ok(leave)
}

/// Validates termination notice period
pub fn validate_termination_notice(service_weeks: u32, notice_days: u32) -> Result<()> {
    let required_days = TerminationNotice::required_notice_days(service_weeks);

    if notice_days < required_days {
        return Err(EmploymentError::InsufficientNotice {
            actual: notice_days,
            required: required_days,
        });
    }

    Ok(())
}

/// Validates CPF contribution calculation
pub fn validate_cpf_calculation(cpf: &CpfContribution) -> Result<()> {
    // Validate rates match statutory rates
    let (expected_employer_bps, expected_employee_bps) =
        CpfContribution::rates_by_age(cpf.employee_age);

    if cpf.employer_rate_bps != expected_employer_bps {
        return Err(EmploymentError::InvalidCpfCalculation {
            reason: format!(
                "Employer rate {}bps does not match statutory rate {}bps for age {}",
                cpf.employer_rate_bps, expected_employer_bps, cpf.employee_age
            ),
        });
    }

    if cpf.employee_rate_bps != expected_employee_bps {
        return Err(EmploymentError::InvalidCpfCalculation {
            reason: format!(
                "Employee rate {}bps does not match statutory rate {}bps for age {}",
                cpf.employee_rate_bps, expected_employee_bps, cpf.employee_age
            ),
        });
    }

    // Validate wage ceiling application
    if cpf.monthly_wage_cents > CpfContribution::ORDINARY_WAGE_CEILING_CENTS {
        let capped_wage = cpf.cpf_subject_wage_cents();
        if capped_wage != CpfContribution::ORDINARY_WAGE_CEILING_CENTS {
            return Err(EmploymentError::InvalidCpfCalculation {
                reason: "Wage ceiling not correctly applied".to_string(),
            });
        }
    }

    Ok(())
}

/// Calculates hourly rate from monthly salary
pub fn calculate_hourly_rate(monthly_salary_cents: u64, is_shift_work: bool) -> u64 {
    // Assume 4.33 weeks per month (52 weeks / 12 months)
    const WEEKS_PER_MONTH: f64 = 4.33;

    let hours_per_week = if is_shift_work { 48.0 } else { 44.0 };
    let hours_per_month = hours_per_week * WEEKS_PER_MONTH;

    (monthly_salary_cents as f64 / hours_per_month) as u64
}

/// Calculates prorated annual leave for partial years of service
pub fn calculate_prorated_leave(annual_entitlement_days: u32, months_worked: u32) -> u32 {
    if months_worked >= 12 {
        return annual_entitlement_days;
    }

    (annual_entitlement_days * months_worked / 12).max(1)
}

/// Calculates the last day of employment based on termination date and notice period
pub fn calculate_last_employment_day(
    termination_date: DateTime<Utc>,
    notice_period_days: u32,
) -> DateTime<Utc> {
    termination_date + chrono::Duration::days(notice_period_days as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_working_hours_valid_non_shift() {
        let hours = WorkingHours {
            hours_per_day: 8.0,
            hours_per_week: 44.0,
            is_shift_work: false,
            rest_days_per_week: 1,
            overtime_eligible: true,
            working_days_per_week: 5,
        };

        assert!(validate_working_hours(&hours).is_ok());
    }

    #[test]
    fn test_working_hours_excessive_non_shift() {
        let hours = WorkingHours {
            hours_per_day: 10.0,
            hours_per_week: 50.0,
            is_shift_work: false,
            rest_days_per_week: 1,
            overtime_eligible: true,
            working_days_per_week: 5,
        };

        match validate_working_hours(&hours) {
            Err(EmploymentError::ExcessiveWorkingHours { actual, limit }) => {
                assert_eq!(actual, 50.0);
                assert_eq!(limit, 44.0);
            }
            _ => panic!("Expected ExcessiveWorkingHours error"),
        }
    }

    #[test]
    fn test_working_hours_insufficient_rest_days() {
        let hours = WorkingHours {
            hours_per_day: 8.0,
            hours_per_week: 44.0,
            is_shift_work: false,
            rest_days_per_week: 0,
            overtime_eligible: true,
            working_days_per_week: 7,
        };

        match validate_working_hours(&hours) {
            Err(EmploymentError::InsufficientRestDays { actual, required }) => {
                assert_eq!(actual, 0);
                assert_eq!(required, 1);
            }
            _ => panic!("Expected InsufficientRestDays error"),
        }
    }

    #[test]
    fn test_overtime_payment_valid() {
        let overtime_pay =
            validate_overtime_payment(5.0, 1.5, 2_000).expect("Valid overtime payment");
        // 5 hours * 1.5x * SGD 20 = SGD 150
        assert_eq!(overtime_pay, 15_000);
    }

    #[test]
    fn test_overtime_payment_below_minimum() {
        match validate_overtime_payment(5.0, 1.2, 2_000) {
            Err(EmploymentError::OvertimeRateBelowMinimum { actual, required }) => {
                assert_eq!(actual, 1.2);
                assert_eq!(required, 1.5);
            }
            _ => panic!("Expected OvertimeRateBelowMinimum error"),
        }
    }

    #[test]
    fn test_leave_entitlement_year_1() {
        let leave = validate_leave_entitlement(0, ContractType::Indefinite)
            .expect("Valid leave entitlement");
        assert_eq!(leave.annual_leave_days, 7);
    }

    #[test]
    fn test_leave_entitlement_year_8_plus() {
        let leave = validate_leave_entitlement(8, ContractType::Indefinite)
            .expect("Valid leave entitlement");
        assert_eq!(leave.annual_leave_days, 14);
    }

    #[test]
    fn test_termination_notice_valid() {
        let service_weeks = 104; // 2 years
        let notice_days = 14;

        assert!(validate_termination_notice(service_weeks, notice_days).is_ok());
    }

    #[test]
    fn test_termination_notice_insufficient() {
        let service_weeks = 260; // 5 years
        let notice_days = 14; // Should be 28

        match validate_termination_notice(service_weeks, notice_days) {
            Err(EmploymentError::InsufficientNotice { actual, required }) => {
                assert_eq!(actual, 14);
                assert_eq!(required, 28);
            }
            _ => panic!("Expected InsufficientNotice error"),
        }
    }

    #[test]
    fn test_cpf_calculation_age_30() {
        let cpf = CpfContribution::new(30, 500_000); // Age 30, SGD 5,000
        validate_cpf_calculation(&cpf).expect("Valid CPF calculation");

        // Age 30: Employer 17%, Employee 20%
        assert_eq!(cpf.employer_rate_bps, 1700);
        assert_eq!(cpf.employee_rate_bps, 2000);
        assert_eq!(cpf.employer_contribution_cents(), 85_000); // SGD 850
        assert_eq!(cpf.employee_contribution_cents(), 100_000); // SGD 1,000
    }

    #[test]
    fn test_cpf_calculation_wage_ceiling() {
        let cpf = CpfContribution::new(30, 700_000); // Age 30, SGD 7,000 (exceeds ceiling)
        validate_cpf_calculation(&cpf).expect("Valid CPF calculation");

        // Should cap at SGD 6,000
        let expected_employer = 600_000 * 1700 / 10_000; // SGD 1,020
        let expected_employee = 600_000 * 2000 / 10_000; // SGD 1,200

        assert_eq!(cpf.employer_contribution_cents(), expected_employer);
        assert_eq!(cpf.employee_contribution_cents(), expected_employee);
    }

    #[test]
    fn test_hourly_rate_calculation() {
        let monthly_salary_cents = 500_000; // SGD 5,000
        let hourly_rate = calculate_hourly_rate(monthly_salary_cents, false);

        // 44 hours/week * 4.33 weeks/month = 190.52 hours/month
        // SGD 5,000 / 190.52 ≈ SGD 26.24/hour
        assert!((2_600..=2_700).contains(&hourly_rate));
    }

    #[test]
    fn test_prorated_leave_half_year() {
        let prorated = calculate_prorated_leave(14, 6);
        assert_eq!(prorated, 7);
    }

    #[test]
    fn test_prorated_leave_full_year() {
        let prorated = calculate_prorated_leave(14, 12);
        assert_eq!(prorated, 14);
    }
}
