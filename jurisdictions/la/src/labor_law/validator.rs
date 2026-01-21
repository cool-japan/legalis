//! Labor Law Validators (ຕົວກວດສອບກົດໝາຍແຮງງານ)
//!
//! Validation functions for Lao labor law compliance based on Labor Law 2013.

use super::error::{LaborLawError, Result};
use super::types::*;

// ============================================================================
// Employment Contract Validation (ການກວດສອບສັນຍາຈ້າງ)
// ============================================================================

/// Validate employment contract (ກວດສອບສັນຍາຈ້າງ)
///
/// Validates compliance with:
/// - Article 15: Essential contract terms
/// - Article 17: Fixed-term contract duration (max 3 years)
/// - Article 20: Probation period (max 60 days)
/// - Article 51: Working hours (8 hours/day, 48 hours/week)
/// - Article 58: Annual leave (minimum 15 days)
///
/// # Arguments
/// * `contract` - Employment contract to validate
///
/// # Returns
/// * `Ok(())` if contract is valid
/// * `Err(LaborLawError)` if contract violates labor laws
///
/// # Example
/// ```
/// use legalis_la::labor_law::*;
/// use chrono::Utc;
///
/// let contract = EmploymentContract {
///     employee_name: "John Doe".to_string(),
///     employee_name_lao: Some("ຈອນ ໂດ".to_string()),
///     employee_id: "P1234567".to_string(),
///     employer_name: "Tech Company Ltd".to_string(),
///     employer_name_lao: Some("ບໍລິສັດເທັກ".to_string()),
///     employer_registration: "REG001".to_string(),
///     employment_type: EmploymentType::IndefiniteTerm,
///     work_schedule: WorkSchedule::Regular,
///     start_date: Utc::now(),
///     end_date: None,
///     probation_period_days: Some(60),
///     job_title: "Software Developer".to_string(),
///     job_title_lao: Some("ນັກພັດທະນາຊອບແວ".to_string()),
///     job_description: "Develop software applications".to_string(),
///     work_location: "Vientiane".to_string(),
///     work_location_lao: Some("ວຽງຈັນ".to_string()),
///     hours_per_day: 8,
///     days_per_week: 6,
///     start_time: "08:00".to_string(),
///     end_time: "17:00".to_string(),
///     rest_period_minutes: 60,
///     base_wage_lak: 3_000_000,
///     hourly_rate_lak: None,
///     allowances: vec![],
///     payment_frequency: PaymentFrequency::Monthly,
///     payment_method: PaymentMethod::BankTransfer,
///     annual_leave_days: 15,
///     sick_leave_days: 30,
///     social_security_enrolled: true,
///     social_security_number: Some("SS123456".to_string()),
///     special_conditions: vec![],
///     renewal_count: 0,
/// };
///
/// assert!(validate_employment_contract(&contract).is_ok());
/// ```
pub fn validate_employment_contract(contract: &EmploymentContract) -> Result<()> {
    // Validate required fields (Article 15)
    if contract.employee_name.trim().is_empty() {
        return Err(LaborLawError::MissingRequiredField {
            field_name: "employee_name".to_string(),
        });
    }

    if contract.employee_id.trim().is_empty() {
        return Err(LaborLawError::MissingRequiredField {
            field_name: "employee_id".to_string(),
        });
    }

    if contract.employer_name.trim().is_empty() {
        return Err(LaborLawError::MissingRequiredField {
            field_name: "employer_name".to_string(),
        });
    }

    if contract.employer_registration.trim().is_empty() {
        return Err(LaborLawError::MissingRequiredField {
            field_name: "employer_registration".to_string(),
        });
    }

    if contract.job_title.trim().is_empty() {
        return Err(LaborLawError::MissingContractTerms {
            missing_terms: "job_title".to_string(),
        });
    }

    if contract.job_description.trim().is_empty() {
        return Err(LaborLawError::MissingContractTerms {
            missing_terms: "job_description".to_string(),
        });
    }

    if contract.work_location.trim().is_empty() {
        return Err(LaborLawError::MissingContractTerms {
            missing_terms: "work_location".to_string(),
        });
    }

    // Validate working hours (Article 51)
    validate_working_hours(contract.hours_per_day, contract.days_per_week)?;

    // Validate rest period (Article 54)
    if contract.hours_per_day >= 6 && contract.rest_period_minutes < MIN_REST_PERIOD_MINUTES {
        return Err(LaborLawError::InsufficientRestPeriod {
            actual: contract.rest_period_minutes,
            required: MIN_REST_PERIOD_MINUTES,
            working_hours: contract.hours_per_day,
        });
    }

    // Validate weekly rest (Article 55)
    let rest_days = 7 - contract.days_per_week;
    if rest_days < MIN_WEEKLY_REST_DAYS {
        return Err(LaborLawError::InsufficientWeeklyRest {
            actual: rest_days,
            required: MIN_WEEKLY_REST_DAYS,
        });
    }

    // Validate fixed-term contract (Article 17)
    if contract.employment_type == EmploymentType::FixedTerm {
        if let Some(end_date) = contract.end_date {
            if end_date <= contract.start_date {
                return Err(LaborLawError::InvalidContractDates);
            }

            let duration_days = (end_date - contract.start_date).num_days();
            let duration_years = duration_days as f64 / 365.25;

            if duration_years > MAX_FIXED_TERM_DURATION_YEARS as f64 {
                return Err(LaborLawError::FixedTermExceedsLimit {
                    years: duration_years,
                    max_years: MAX_FIXED_TERM_DURATION_YEARS,
                });
            }
        } else {
            return Err(LaborLawError::MissingContractTerms {
                missing_terms: "end_date (required for fixed-term contracts)".to_string(),
            });
        }
    }

    // Validate probation period (Article 20)
    if let Some(probation_days) = contract.probation_period_days
        && probation_days > MAX_PROBATION_PERIOD_DAYS
    {
        return Err(LaborLawError::ProbationExceedsLimit {
            actual_days: probation_days,
            max_days: MAX_PROBATION_PERIOD_DAYS,
        });
    }

    // Validate annual leave (Article 58)
    validate_annual_leave(contract.annual_leave_days)?;

    // Validate wage
    if contract.base_wage_lak == 0 && contract.hourly_rate_lak.is_none() {
        return Err(LaborLawError::MissingContractTerms {
            missing_terms: "base_wage_lak or hourly_rate_lak".to_string(),
        });
    }

    Ok(())
}

// ============================================================================
// Working Hours Validation (ການກວດສອບເວລາເຮັດວຽກ)
// ============================================================================

/// Validate working hours (ກວດສອບຊົ່ວໂມງເຮັດວຽກ)
///
/// Article 51: Working hours shall not exceed:
/// - 8 hours per day
/// - 48 hours per week
/// - Maximum 6 working days per week
///
/// # Arguments
/// * `hours_per_day` - Daily working hours
/// * `days_per_week` - Working days per week
///
/// # Returns
/// * `Ok(())` if working hours are valid
/// * `Err(LaborLawError)` if working hours exceed limits
pub fn validate_working_hours(hours_per_day: u32, days_per_week: u32) -> Result<()> {
    // Check daily hours (Article 51)
    if hours_per_day > STATUTORY_HOURS_PER_DAY {
        return Err(LaborLawError::ExceedsStatutoryDailyHours {
            actual: hours_per_day,
            statutory: STATUTORY_HOURS_PER_DAY,
        });
    }

    // Check working days per week
    if days_per_week > MAX_WORKING_DAYS_PER_WEEK {
        return Err(LaborLawError::ExceedsMaxWorkingDays {
            actual: days_per_week,
            max: MAX_WORKING_DAYS_PER_WEEK,
        });
    }

    // Check weekly hours (Article 51)
    let weekly_hours = hours_per_day * days_per_week;
    if weekly_hours > STATUTORY_HOURS_PER_WEEK {
        return Err(LaborLawError::ExceedsStatutoryWeeklyHours {
            actual: weekly_hours,
            statutory: STATUTORY_HOURS_PER_WEEK,
        });
    }

    Ok(())
}

/// Validate working hours record (ກວດສອບບັນທຶກເວລາເຮັດວຽກ)
///
/// # Arguments
/// * `record` - Working hours record to validate
///
/// # Returns
/// * `Ok(())` if record is valid
/// * `Err(LaborLawError)` if record violates working hours regulations
pub fn validate_working_hours_record(record: &WorkingHoursRecord) -> Result<()> {
    // Validate daily overtime limit (Article 52)
    if record.overtime_hours > MAX_OVERTIME_HOURS_PER_DAY as f64 {
        return Err(LaborLawError::ExceedsDailyOvertimeLimit {
            actual: record.overtime_hours,
            limit: MAX_OVERTIME_HOURS_PER_DAY,
        });
    }

    // Validate rest period for shifts over 6 hours (Article 54)
    if record.regular_hours >= 6.0 && record.rest_periods_minutes < MIN_REST_PERIOD_MINUTES {
        return Err(LaborLawError::InsufficientRestPeriod {
            actual: record.rest_periods_minutes,
            required: MIN_REST_PERIOD_MINUTES,
            working_hours: record.regular_hours as u32,
        });
    }

    Ok(())
}

/// Validate monthly working summary (ກວດສອບສະຫຼຸບການເຮັດວຽກລາຍເດືອນ)
///
/// # Arguments
/// * `summary` - Monthly working summary to validate
///
/// # Returns
/// * `Ok(())` if summary is valid
/// * `Err(LaborLawError)` if summary violates monthly overtime limits
pub fn validate_monthly_summary(summary: &MonthlyWorkingSummary) -> Result<()> {
    // Validate monthly overtime limit (Article 52)
    if summary.total_overtime_hours > MAX_OVERTIME_HOURS_PER_MONTH as f64 {
        return Err(LaborLawError::ExceedsMonthlyOvertimeLimit {
            actual: summary.total_overtime_hours,
            limit: MAX_OVERTIME_HOURS_PER_MONTH,
        });
    }

    Ok(())
}

// ============================================================================
// Overtime Premium Validation (ການກວດສອບຄ່າລ່ວງເວລາ)
// ============================================================================

/// Validate overtime premium calculation (ກວດສອບການຄິດໄລ່ຄ່າລ່ວງເວລາ)
///
/// Article 53: Overtime premium rates:
/// - Regular overtime: 50% premium
/// - Night shift (22:00-06:00): 20% premium
/// - Holiday work: 100% premium
///
/// # Arguments
/// * `base_rate` - Base hourly rate in LAK
/// * `overtime_hours` - Overtime hours worked
/// * `actual_payment` - Actual overtime payment in LAK
///
/// # Returns
/// * `Ok(())` if overtime premium is correct
/// * `Err(LaborLawError)` if premium is insufficient
pub fn validate_overtime_premium(
    base_rate: u64,
    overtime_hours: f64,
    actual_payment: u64,
) -> Result<()> {
    let required_payment =
        (base_rate as f64 * overtime_hours * (1.0 + OVERTIME_PREMIUM_RATE)) as u64;

    if actual_payment < required_payment {
        return Err(LaborLawError::IncorrectOvertimePremium {
            actual_lak: actual_payment,
            required_lak: required_payment,
        });
    }

    Ok(())
}

/// Validate night shift premium calculation (ກວດສອບການຄິດໄລ່ຄ່າກະກາງຄືນ)
///
/// Article 53: Night shift (22:00-06:00) requires 20% premium
///
/// # Arguments
/// * `base_rate` - Base hourly rate in LAK
/// * `night_hours` - Night shift hours worked
/// * `actual_payment` - Actual night shift payment in LAK
///
/// # Returns
/// * `Ok(())` if night shift premium is correct
/// * `Err(LaborLawError)` if premium is insufficient
pub fn validate_night_shift_premium(
    base_rate: u64,
    night_hours: f64,
    actual_payment: u64,
) -> Result<()> {
    let required_payment =
        (base_rate as f64 * night_hours * (1.0 + NIGHT_SHIFT_PREMIUM_RATE)) as u64;

    if actual_payment < required_payment {
        return Err(LaborLawError::IncorrectNightShiftPremium {
            actual_lak: actual_payment,
            required_lak: required_payment,
        });
    }

    Ok(())
}

/// Validate holiday work premium calculation (ກວດສອບການຄິດໄລ່ຄ່າເຮັດວຽກວັນພັກ)
///
/// Article 53: Holiday work requires 100% premium (double pay)
///
/// # Arguments
/// * `base_rate` - Base hourly rate in LAK
/// * `holiday_hours` - Holiday work hours
/// * `actual_payment` - Actual holiday work payment in LAK
///
/// # Returns
/// * `Ok(())` if holiday work premium is correct
/// * `Err(LaborLawError)` if premium is insufficient
pub fn validate_holiday_work_premium(
    base_rate: u64,
    holiday_hours: f64,
    actual_payment: u64,
) -> Result<()> {
    let required_payment =
        (base_rate as f64 * holiday_hours * (1.0 + HOLIDAY_WORK_PREMIUM_RATE)) as u64;

    if actual_payment < required_payment {
        return Err(LaborLawError::IncorrectHolidayWorkPremium {
            actual_lak: actual_payment,
            required_lak: required_payment,
        });
    }

    Ok(())
}

// ============================================================================
// Minimum Wage Validation (ການກວດສອບຄ່າແຮງງານຂັ້ນຕ່ຳ)
// ============================================================================

/// Validate minimum wage compliance (ກວດສອບການປະຕິບັດຕາມຄ່າແຮງງານຂັ້ນຕ່ຳ)
///
/// # Arguments
/// * `actual_wage` - Actual monthly wage in LAK
/// * `minimum_wage` - Minimum wage requirement in LAK
///
/// # Returns
/// * `Ok(())` if wage meets minimum requirement
/// * `Err(LaborLawError)` if wage is below minimum
pub fn validate_minimum_wage(actual_wage: u64, minimum_wage: u64) -> Result<()> {
    if actual_wage < minimum_wage {
        return Err(LaborLawError::BelowMinimumWage {
            actual_lak: actual_wage,
            minimum_lak: minimum_wage,
        });
    }

    Ok(())
}

/// Validate minimum hourly rate (ກວດສອບອັດຕາຕໍ່ຊົ່ວໂມງຂັ້ນຕ່ຳ)
///
/// # Arguments
/// * `actual_rate` - Actual hourly rate in LAK
/// * `minimum_rate` - Minimum hourly rate in LAK
///
/// # Returns
/// * `Ok(())` if hourly rate meets minimum requirement
/// * `Err(LaborLawError)` if rate is below minimum
pub fn validate_hourly_rate(actual_rate: u64, minimum_rate: u64) -> Result<()> {
    if actual_rate < minimum_rate {
        return Err(LaborLawError::HourlyRateBelowMinimum {
            actual_lak: actual_rate,
            minimum_lak: minimum_rate,
        });
    }

    Ok(())
}

// ============================================================================
// Annual Leave Validation (ການກວດສອບການລາພັກປະຈຳປີ)
// ============================================================================

/// Validate annual leave entitlement (ກວດສອບສິດການລາພັກປະຈຳປີ)
///
/// Article 58: Employees are entitled to at least 15 days of paid annual leave
///
/// # Arguments
/// * `annual_leave_days` - Number of annual leave days provided
///
/// # Returns
/// * `Ok(())` if annual leave meets minimum requirement
/// * `Err(LaborLawError)` if annual leave is insufficient
pub fn validate_annual_leave(annual_leave_days: u32) -> Result<()> {
    if annual_leave_days < MIN_ANNUAL_LEAVE_DAYS {
        return Err(LaborLawError::InsufficientAnnualLeave {
            actual_days: annual_leave_days,
            min_days: MIN_ANNUAL_LEAVE_DAYS,
        });
    }

    Ok(())
}

/// Validate leave request (ກວດສອບການຂໍລາພັກ)
///
/// # Arguments
/// * `request` - Leave request to validate
///
/// # Returns
/// * `Ok(())` if leave request is valid
/// * `Err(LaborLawError)` if request has invalid dates
pub fn validate_leave_request(request: &LeaveRequest) -> Result<()> {
    // Validate dates
    if request.end_date <= request.start_date {
        return Err(LaborLawError::InvalidLeaveDates);
    }

    // Validate maternity leave duration (Article 62)
    if request.leave_type == LeaveType::MaternityLeave && request.days < MATERNITY_LEAVE_DAYS {
        return Err(LaborLawError::InsufficientMaternityLeave {
            actual_days: request.days,
            statutory_days: MATERNITY_LEAVE_DAYS,
        });
    }

    Ok(())
}

// ============================================================================
// Termination Validation (ການກວດສອບການເລີກຈ້າງ)
// ============================================================================

/// Validate termination notice (ກວດສອບການແຈ້ງການເລີກຈ້າງ)
///
/// Article 74: Employer must provide at least 30 days notice or payment in lieu
/// Article 75: Termination for cause does not require notice
///
/// # Arguments
/// * `notice` - Termination notice to validate
/// * `monthly_wage` - Employee's monthly wage for severance calculation
///
/// # Returns
/// * `Ok(())` if termination notice is valid
/// * `Err(LaborLawError)` if notice period or severance is insufficient
pub fn validate_termination_notice(notice: &TerminationNotice, monthly_wage: u64) -> Result<()> {
    let notice_days = notice.notice_period_days();

    // Check notice period (Article 74)
    match notice.termination_type {
        TerminationType::TerminationForCause | TerminationType::Death => {
            // No notice required for these types
        }
        TerminationType::ContractExpiration => {
            // No notice required for contract expiration
        }
        _ => {
            if notice_days < TERMINATION_NOTICE_DAYS as i64 {
                // Check if notice allowance is provided
                let required_allowance =
                    monthly_wage / 30 * (TERMINATION_NOTICE_DAYS - notice_days as u32) as u64;

                if notice.notice_allowance_lak.unwrap_or(0) < required_allowance {
                    return Err(LaborLawError::InsufficientNotice {
                        actual_days: notice_days,
                        required_days: TERMINATION_NOTICE_DAYS,
                    });
                }
            }
        }
    }

    // Validate severance pay (Article 77)
    validate_severance_pay(notice, monthly_wage)?;

    Ok(())
}

/// Validate severance pay calculation (ກວດສອບການຄິດໄລ່ຄ່າຊົດເຊີຍ)
///
/// Article 77: Severance pay based on years of service:
/// - Less than 1 year: 0 months
/// - 1-3 years: 1 month salary
/// - 3-5 years: 2 months salary
/// - 5-10 years: 3 months salary
/// - 10+ years: 6 months salary
///
/// # Arguments
/// * `notice` - Termination notice
/// * `monthly_wage` - Employee's monthly wage
///
/// # Returns
/// * `Ok(())` if severance pay is sufficient
/// * `Err(LaborLawError)` if severance pay is insufficient
pub fn validate_severance_pay(notice: &TerminationNotice, monthly_wage: u64) -> Result<()> {
    let required_severance = notice.calculate_severance_pay(monthly_wage);

    // Termination for cause and voluntary resignation don't require severance
    if required_severance == 0 {
        return Ok(());
    }

    let actual_severance = notice.severance_pay_lak.unwrap_or(0);

    if actual_severance < required_severance {
        let months = if notice.years_of_service < 1.0 {
            0
        } else if notice.years_of_service < 3.0 {
            1
        } else if notice.years_of_service < 5.0 {
            2
        } else if notice.years_of_service < 10.0 {
            3
        } else {
            6
        };

        return Err(LaborLawError::InsufficientSeverancePay {
            actual_lak: actual_severance,
            required_lak: required_severance,
            months,
        });
    }

    Ok(())
}

// ============================================================================
// Social Security Validation (ການກວດສອບປະກັນສັງຄົມ)
// ============================================================================

/// Validate social security enrollment (ກວດສອບການຂຶ້ນທະບຽນປະກັນສັງຄົມ)
///
/// # Arguments
/// * `contract` - Employment contract to check
///
/// # Returns
/// * `Ok(())` if employee is enrolled in social security
/// * `Err(LaborLawError)` if enrollment is missing
pub fn validate_social_security_enrollment(contract: &EmploymentContract) -> Result<()> {
    if !contract.social_security_enrolled {
        return Err(LaborLawError::NotEnrolledInSocialSecurity);
    }

    if contract.social_security_number.is_none()
        || contract
            .social_security_number
            .as_ref()
            .is_none_or(|s| s.trim().is_empty())
    {
        return Err(LaborLawError::InvalidSocialSecurityNumber {
            number: "missing or empty".to_string(),
        });
    }

    Ok(())
}

/// Validate social security contribution (ກວດສອບການປະກອບສ່ວນປະກັນສັງຄົມ)
///
/// # Arguments
/// * `contribution` - Social security contribution to validate
/// * `actual_total` - Actual total contribution paid in LAK
///
/// # Returns
/// * `Ok(())` if contribution is sufficient
/// * `Err(LaborLawError)` if contribution is insufficient
pub fn validate_social_security_contribution(
    contribution: &SocialSecurityContribution,
    actual_total: u64,
) -> Result<()> {
    let required_total = contribution.total_contribution();

    if actual_total < required_total {
        return Err(LaborLawError::IncorrectSocialSecurityContribution {
            actual_lak: actual_total,
            required_lak: required_total,
        });
    }

    Ok(())
}

// ============================================================================
// Comprehensive Contract Validation (ການກວດສອບສັນຍາແບບຄົບຖ້ວນ)
// ============================================================================

/// Perform comprehensive validation of all labor law requirements
/// ກວດສອບຄວາມຖືກຕ້ອງຄົບຖ້ວນຕາມກົດໝາຍແຮງງານ
///
/// This function performs all available validations on an employment contract.
///
/// # Arguments
/// * `contract` - Employment contract to validate
///
/// # Returns
/// * `Ok(Vec<String>)` - List of warnings (non-critical issues)
/// * `Err(LaborLawError)` - Critical violation found
pub fn validate_comprehensive(contract: &EmploymentContract) -> Result<Vec<String>> {
    let mut warnings = Vec::new();

    // Critical validations (will return error)
    validate_employment_contract(contract)?;
    validate_social_security_enrollment(contract)?;

    // Non-critical checks (will add warnings)
    if contract.employee_name_lao.is_none() {
        warnings.push("Consider adding employee name in Lao (ຄວນເພີ່ມຊື່ພາສາລາວ)".to_string());
    }

    if contract.employer_name_lao.is_none() {
        warnings.push("Consider adding employer name in Lao (ຄວນເພີ່ມຊື່ບໍລິສັດພາສາລາວ)".to_string());
    }

    if contract.job_title_lao.is_none() {
        warnings.push("Consider adding job title in Lao (ຄວນເພີ່ມຕຳແໜ່ງພາສາລາວ)".to_string());
    }

    if contract.work_location_lao.is_none() {
        warnings.push("Consider adding work location in Lao (ຄວນເພີ່ມສະຖານທີ່ວຽກພາສາລາວ)".to_string());
    }

    if contract.allowances.is_empty() {
        warnings.push(
            "No allowances specified (ບໍ່ມີເບີ້ຍເລີ້ຍງ) - consider adding if applicable".to_string(),
        );
    }

    Ok(warnings)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn create_valid_contract() -> EmploymentContract {
        EmploymentContract {
            employee_name: "John Doe".to_string(),
            employee_name_lao: Some("ຈອນ ໂດ".to_string()),
            employee_id: "P1234567".to_string(),
            employer_name: "Tech Company Ltd".to_string(),
            employer_name_lao: Some("ບໍລິສັດເທັກ".to_string()),
            employer_registration: "REG001".to_string(),
            employment_type: EmploymentType::IndefiniteTerm,
            work_schedule: WorkSchedule::Regular,
            start_date: Utc::now(),
            end_date: None,
            probation_period_days: Some(60),
            job_title: "Software Developer".to_string(),
            job_title_lao: Some("ນັກພັດທະນາຊອບແວ".to_string()),
            job_description: "Develop software applications".to_string(),
            work_location: "Vientiane".to_string(),
            work_location_lao: Some("ວຽງຈັນ".to_string()),
            hours_per_day: 8,
            days_per_week: 6,
            start_time: "08:00".to_string(),
            end_time: "17:00".to_string(),
            rest_period_minutes: 60,
            base_wage_lak: 3_000_000,
            hourly_rate_lak: None,
            allowances: vec![],
            payment_frequency: PaymentFrequency::Monthly,
            payment_method: PaymentMethod::BankTransfer,
            annual_leave_days: 15,
            sick_leave_days: 30,
            social_security_enrolled: true,
            social_security_number: Some("SS123456".to_string()),
            special_conditions: vec![],
            renewal_count: 0,
        }
    }

    #[test]
    fn test_valid_contract() {
        let contract = create_valid_contract();
        assert!(validate_employment_contract(&contract).is_ok());
    }

    #[test]
    fn test_exceeds_daily_hours() {
        let mut contract = create_valid_contract();
        contract.hours_per_day = 10;

        let result = validate_employment_contract(&contract);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            LaborLawError::ExceedsStatutoryDailyHours { .. }
        ));
    }

    #[test]
    fn test_exceeds_weekly_hours() {
        let mut contract = create_valid_contract();
        contract.hours_per_day = 8;
        contract.days_per_week = 7; // 56 hours/week

        let result = validate_employment_contract(&contract);
        assert!(result.is_err());
    }

    #[test]
    fn test_insufficient_annual_leave() {
        let mut contract = create_valid_contract();
        contract.annual_leave_days = 10;

        let result = validate_employment_contract(&contract);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            LaborLawError::InsufficientAnnualLeave { .. }
        ));
    }

    #[test]
    fn test_probation_exceeds_limit() {
        let mut contract = create_valid_contract();
        contract.probation_period_days = Some(90);

        let result = validate_employment_contract(&contract);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            LaborLawError::ProbationExceedsLimit { .. }
        ));
    }

    #[test]
    fn test_overtime_premium_validation() {
        let base_rate = 20_000; // 20,000 LAK/hour
        let overtime_hours = 10.0;
        let correct_payment = (base_rate as f64 * overtime_hours * 1.5) as u64; // 50% premium

        assert!(validate_overtime_premium(base_rate, overtime_hours, correct_payment).is_ok());

        let insufficient_payment = (base_rate as f64 * overtime_hours) as u64; // No premium
        assert!(
            validate_overtime_premium(base_rate, overtime_hours, insufficient_payment).is_err()
        );
    }

    #[test]
    fn test_minimum_wage_validation() {
        assert!(validate_minimum_wage(3_000_000, 2_500_000).is_ok());
        assert!(validate_minimum_wage(2_000_000, 2_500_000).is_err());
    }

    #[test]
    fn test_comprehensive_validation() {
        let contract = create_valid_contract();
        let result = validate_comprehensive(&contract);
        assert!(result.is_ok());

        // Should have no warnings since we provided all Lao translations
        let warnings = result.unwrap();
        assert_eq!(warnings.len(), 1); // Only allowances warning
    }
}
