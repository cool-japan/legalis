//! Labor Law Validators (労働法バリデータ)
//!
//! Validation functions for Japanese labor law compliance.

use super::error::{LaborLawError, Result};
use super::types::*;

// ============================================================================
// Employment Contract Validation (雇用契約検証)
// ============================================================================

/// Validate employment contract (雇用契約の検証)
///
/// Validates compliance with:
/// - Article 15: Contract terms disclosure
/// - Article 32: Working hours limits
/// - Labor Contract Act Article 17: Fixed-term contract limits
///
/// # Arguments
/// * `contract` - Employment contract to validate
///
/// # Returns
/// * `Ok(())` if contract is valid
/// * `Err(LaborLawError)` if contract violates labor laws
pub fn validate_employment_contract(contract: &EmploymentContract) -> Result<()> {
    // Validate required fields
    if contract.employee_name.trim().is_empty() {
        return Err(LaborLawError::MissingRequiredField {
            field_name: "employee_name".to_string(),
        });
    }

    if contract.employer_name.trim().is_empty() {
        return Err(LaborLawError::MissingRequiredField {
            field_name: "employer_name".to_string(),
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

    // Validate working hours (Article 32)
    if contract.hours_per_day > STATUTORY_HOURS_PER_DAY {
        return Err(LaborLawError::ExceedsStatutoryDailyHours {
            actual: contract.hours_per_day as f64,
            statutory: STATUTORY_HOURS_PER_DAY,
        });
    }

    let weekly_hours = contract.weekly_hours();
    if weekly_hours > STATUTORY_HOURS_PER_WEEK {
        return Err(LaborLawError::ExceedsStatutoryWeeklyHours {
            actual: weekly_hours,
            statutory: STATUTORY_HOURS_PER_WEEK,
        });
    }

    // Validate days per week
    if contract.days_per_week > 7 {
        return Err(LaborLawError::ValidationError {
            message: "days_per_week cannot exceed 7".to_string(),
        });
    }

    let days_off = 7 - contract.days_per_week;
    if days_off < MIN_DAYS_OFF_PER_WEEK {
        return Err(LaborLawError::InsufficientDaysOff {
            actual: days_off,
            required: MIN_DAYS_OFF_PER_WEEK,
        });
    }

    // Validate fixed-term contract (Labor Contract Act Article 17)
    if contract.employment_type == EmploymentType::FixedTerm {
        if let Some(end_date) = contract.end_date {
            let duration_days = (end_date - contract.start_date).num_days();
            let duration_years = duration_days as f64 / 365.0;

            // Fixed-term contracts generally limited to 3 years (5 years for specialized)
            if duration_years > 3.0 {
                return Err(LaborLawError::FixedTermExceedsLimit { duration_years });
            }
        } else {
            return Err(LaborLawError::MissingContractTerms {
                missing_terms: "end_date (required for fixed-term contracts)".to_string(),
            });
        }
    }

    // Validate probation period
    if let Some(probation_days) = contract.probation_period_days {
        // Probation period typically should not exceed 180 days
        if probation_days > 180 {
            return Err(LaborLawError::InvalidProbationPeriod {
                actual: probation_days,
            });
        }
    }

    // Validate base wage (should be at least minimum wage equivalent)
    if contract.base_wage_jpy == 0 {
        return Err(LaborLawError::MissingContractTerms {
            missing_terms: "base_wage_jpy".to_string(),
        });
    }

    Ok(())
}

// ============================================================================
// Working Time Validation (労働時間検証)
// ============================================================================

/// Validate working time record (労働時間記録の検証)
///
/// Validates compliance with:
/// - Article 32: Statutory working hours
/// - Article 34: Rest periods
///
/// # Arguments
/// * `record` - Working time record to validate
///
/// # Returns
/// * `Ok(())` if record is valid
/// * `Err(LaborLawError)` if record violates labor laws
pub fn validate_working_time_record(record: &WorkingTimeRecord) -> Result<()> {
    // Validate date consistency
    if record.end_time <= record.start_time {
        return Err(LaborLawError::InvalidDate {
            reason: "end_time must be after start_time".to_string(),
        });
    }

    let working_hours = record.actual_working_hours();

    // Validate rest periods (Article 34)
    let required_rest = if working_hours > 8.0 {
        MIN_REST_8_HOURS_MINUTES
    } else if working_hours > 6.0 {
        MIN_REST_6_HOURS_MINUTES
    } else {
        0
    };

    if record.rest_minutes < required_rest {
        return Err(LaborLawError::InsufficientRestPeriod {
            actual: record.rest_minutes,
            required: required_rest,
            working_hours,
        });
    }

    // Note: We don't enforce strict daily hour limits here because overtime
    // may be permitted under Article 36 agreements (36協定)
    // However, we can warn if it exceeds statutory hours
    if working_hours > STATUTORY_HOURS_PER_DAY as f64 + 8.0 {
        // More than 16 hours is highly suspicious
        return Err(LaborLawError::ValidationError {
            message: format!("Working hours {} appears unreasonably high", working_hours),
        });
    }

    Ok(())
}

/// Validate monthly working summary (月間労働時間集計の検証)
///
/// Validates compliance with:
/// - Article 36: Overtime limits
/// - Monthly overtime regulations
///
/// # Arguments
/// * `summary` - Monthly working summary to validate
///
/// # Returns
/// * `Ok(())` if summary is valid
/// * `Err(LaborLawError)` if summary violates overtime limits
pub fn validate_monthly_working_summary(summary: &MonthlyWorkingSummary) -> Result<()> {
    // Validate month
    if !(1..=12).contains(&summary.month) {
        return Err(LaborLawError::DateOutOfRange {
            field: "month".to_string(),
        });
    }

    // Check monthly overtime limit (requires Article 36 agreement for over 45 hours)
    // The 60-hour limit triggers higher premium rate
    if summary.overtime_hours > 100.0 {
        // Over 100 hours is extremely concerning
        return Err(LaborLawError::ExceedsMonthlyOvertimeLimit {
            actual: summary.overtime_hours,
            limit: 100,
        });
    }

    // Validate consistency
    if summary.total_hours < summary.overtime_hours {
        return Err(LaborLawError::ValidationError {
            message: "total_hours cannot be less than overtime_hours".to_string(),
        });
    }

    Ok(())
}

// ============================================================================
// Termination Validation (解雇・退職検証)
// ============================================================================

/// Validate termination notice (解雇予告・退職通知の検証)
///
/// Validates compliance with:
/// - Article 20: Advance notice or allowance in lieu
/// - Labor Contract Act Article 16: Abuse of dismissal rights
///
/// # Arguments
/// * `notice` - Termination notice to validate
/// * `average_daily_wage_jpy` - Employee's average daily wage
///
/// # Returns
/// * `Ok(())` if termination is valid
/// * `Err(LaborLawError)` if termination violates labor laws
pub fn validate_termination_notice(
    notice: &TerminationNotice,
    average_daily_wage_jpy: u64,
) -> Result<()> {
    // Validate dates
    if notice.effective_date < notice.notice_date {
        return Err(LaborLawError::InvalidDate {
            reason: "effective_date must be after notice_date".to_string(),
        });
    }

    // Validate notice period for dismissals (Article 20)
    match notice.termination_type {
        TerminationType::OrdinaryDismissal | TerminationType::DisciplinaryDismissal => {
            let notice_days = notice.notice_period_days();

            if notice_days < TERMINATION_NOTICE_DAYS as i64 {
                // Check if notice allowance is provided
                let required_allowance =
                    notice.calculate_required_notice_allowance(average_daily_wage_jpy);

                if let Some(required) = required_allowance {
                    if let Some(provided) = notice.notice_allowance_jpy {
                        if provided < required {
                            return Err(LaborLawError::IncorrectOvertimePremium {
                                actual: provided,
                                required,
                            });
                        }
                    } else {
                        return Err(LaborLawError::MissingNoticeAllowance { required });
                    }
                }
            }

            // Check for potentially abusive dismissal (Labor Contract Act Article 16)
            if notice.reason.trim().is_empty() {
                return Err(LaborLawError::AbusiveDismissal {
                    reason: "No reason provided for dismissal".to_string(),
                });
            }

            // Check for vague or potentially abusive reasons
            let reason_lower = notice.reason.to_lowercase();
            if reason_lower.len() < 10 {
                return Err(LaborLawError::AbusiveDismissal {
                    reason: "Dismissal reason is too vague and may constitute abuse".to_string(),
                });
            }
        }
        _ => {
            // Other termination types don't require advance notice
        }
    }

    Ok(())
}

/// Check for potentially abusive dismissal (解雇権濫用のチェック)
///
/// Checks if dismissal may constitute abuse under Labor Contract Act Article 16.
/// This is a preliminary check - final determination requires case-by-case analysis.
///
/// # Arguments
/// * `notice` - Termination notice
/// * `contract` - Employment contract
///
/// # Returns
/// * `Ok(())` if dismissal appears reasonable
/// * `Err(LaborLawError)` if dismissal may be abusive
pub fn check_abusive_dismissal(
    notice: &TerminationNotice,
    contract: &EmploymentContract,
) -> Result<()> {
    if notice.termination_type != TerminationType::OrdinaryDismissal
        && notice.termination_type != TerminationType::DisciplinaryDismissal
    {
        return Ok(());
    }

    // Check employment duration - dismissing long-term employees requires stronger justification
    let employment_duration = (notice.notice_date - contract.start_date).num_days();

    if employment_duration > 365 * 5 && notice.reason.len() < 50 {
        return Err(LaborLawError::AbusiveDismissal {
            reason: format!(
                "Employee with {} years of service requires detailed justification for dismissal",
                employment_duration / 365
            ),
        });
    }

    // Check for dismissal during protected periods
    // (This is simplified - actual implementation would need more context)

    Ok(())
}

// ============================================================================
// Wage Payment Validation (賃金支払検証)
// ============================================================================

/// Validate wage payment (賃金支払の検証)
///
/// Validates compliance with:
/// - Article 24: Full payment, direct payment, cash payment, regular payment
/// - Article 37: Overtime premiums
///
/// # Arguments
/// * `payment` - Wage payment record to validate
///
/// # Returns
/// * `Ok(())` if payment is valid
/// * `Err(LaborLawError)` if payment violates labor laws
pub fn validate_wage_payment(payment: &WagePayment) -> Result<()> {
    // Validate dates
    if payment.period_end < payment.period_start {
        return Err(LaborLawError::InvalidDate {
            reason: "period_end must be after period_start".to_string(),
        });
    }

    // Payment should be made promptly (Article 24 - regular payment)
    let days_after_period = (payment.payment_date - payment.period_end).num_days();
    if days_after_period > 30 {
        return Err(LaborLawError::LateWagePayment {
            payment_date: format!("{}", payment.payment_date),
            period_end: format!("{}", payment.period_end),
        });
    }

    // Validate net payment calculation
    if !payment.validate_net_payment() {
        return Err(LaborLawError::ValidationError {
            message: "Net payment calculation is incorrect".to_string(),
        });
    }

    // Validate non-negative values
    if payment.net_payment_jpy == 0 && payment.gross_wage() > payment.deductions_jpy {
        return Err(LaborLawError::ValidationError {
            message: "Net payment should not be zero when gross exceeds deductions".to_string(),
        });
    }

    Ok(())
}

/// Validate minimum wage compliance (最低賃金の検証)
///
/// # Arguments
/// * `hourly_wage_jpy` - Hourly wage to validate
/// * `minimum_wage_jpy` - Applicable minimum wage
///
/// # Returns
/// * `Ok(())` if wage meets minimum
/// * `Err(LaborLawError)` if wage is below minimum
pub fn validate_minimum_wage(hourly_wage_jpy: u64, minimum_wage_jpy: u64) -> Result<()> {
    // Use Tokyo as default prefecture for backward compatibility
    use crate::labor_law::minimum_wage::Prefecture;

    if hourly_wage_jpy < minimum_wage_jpy {
        return Err(LaborLawError::BelowMinimumWage {
            actual_hourly: hourly_wage_jpy,
            required_minimum: minimum_wage_jpy,
            prefecture: Prefecture::Tokyo,
        });
    }
    Ok(())
}

// ============================================================================
// Harassment Detection (ハラスメント検出)
// ============================================================================

/// Analyze harassment report (ハラスメント報告の分析)
///
/// Performs preliminary analysis of harassment reports.
/// This is a basic check - proper investigation requires human review.
///
/// # Arguments
/// * `report` - Harassment report to analyze
///
/// # Returns
/// * `Ok(())` if report requires investigation
/// * `Err(LaborLawError)` with severity indication
pub fn analyze_harassment_report(report: &HarassmentReport) -> Result<()> {
    // Check if description is sufficiently detailed
    if report.description.trim().len() < 20 {
        return Err(LaborLawError::ValidationError {
            message: "Harassment report description is too brief for proper analysis".to_string(),
        });
    }

    // Check for power imbalance in power harassment cases
    if report.harassment_type == HarassmentType::PowerHarassment {
        if let (Some(perp_pos), Some(victim_pos)) =
            (&report.perpetrator_position, &report.victim_position)
        {
            // Simple check: if perpetrator is in management position
            if perp_pos.to_lowercase().contains("manager")
                || perp_pos.to_lowercase().contains("director")
                || perp_pos.to_lowercase().contains("chief")
            {
                return Err(LaborLawError::PowerHarassmentDetected {
                    description: format!(
                        "Power imbalance detected: {} to {}",
                        perp_pos, victim_pos
                    ),
                });
            }
        }
    }

    // Sexual harassment detection
    if report.harassment_type == HarassmentType::SexualHarassment {
        return Err(LaborLawError::SexualHarassmentDetected {
            description: "Sexual harassment report requires immediate investigation".to_string(),
        });
    }

    // Maternity harassment detection
    if report.harassment_type == HarassmentType::MaternityHarassment {
        return Err(LaborLawError::MaternityHarassmentDetected {
            description: "Maternity harassment report requires immediate investigation".to_string(),
        });
    }

    Ok(())
}

// ============================================================================
// Fixed-term Contract Conversion (無期転換ルール)
// ============================================================================

/// Check if employee is eligible for indefinite-term conversion (無期転換権の確認)
///
/// Validates Labor Contract Act Article 18 (5-year rule)
///
/// # Arguments
/// * `contract` - Employment contract
///
/// # Returns
/// * `Ok(true)` if eligible for conversion
/// * `Ok(false)` if not yet eligible
pub fn check_indefinite_conversion_eligibility(contract: &EmploymentContract) -> Result<bool> {
    Ok(contract.is_eligible_for_indefinite_conversion())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};

    #[test]
    fn test_validate_employment_contract_valid() {
        let contract = EmploymentContract {
            employee_name: "山田太郎".to_string(),
            employer_name: "テスト株式会社".to_string(),
            employment_type: EmploymentType::IndefiniteTerm,
            work_pattern: WorkPattern::Regular,
            start_date: Utc::now(),
            end_date: None,
            base_wage_jpy: 300_000,
            hours_per_day: 8,
            days_per_week: 5,
            job_description: "Software Development".to_string(),
            work_location: "Tokyo Office".to_string(),
            probation_period_days: Some(90),
            renewal_count: 0,
        };

        assert!(validate_employment_contract(&contract).is_ok());
    }

    #[test]
    fn test_validate_employment_contract_exceeds_hours() {
        let contract = EmploymentContract {
            employee_name: "Test".to_string(),
            employer_name: "Test Company".to_string(),
            employment_type: EmploymentType::IndefiniteTerm,
            work_pattern: WorkPattern::Regular,
            start_date: Utc::now(),
            end_date: None,
            base_wage_jpy: 300_000,
            hours_per_day: 10,
            days_per_week: 5,
            job_description: "Test".to_string(),
            work_location: "Test".to_string(),
            probation_period_days: None,
            renewal_count: 0,
        };

        assert!(matches!(
            validate_employment_contract(&contract),
            Err(LaborLawError::ExceedsStatutoryDailyHours { .. })
        ));
    }

    #[test]
    fn test_validate_working_time_record() {
        let now = Utc::now();
        let record = WorkingTimeRecord {
            date: now,
            start_time: now,
            end_time: now + Duration::hours(9),
            rest_minutes: 60,
            is_holiday: false,
        };

        assert!(validate_working_time_record(&record).is_ok());
    }

    #[test]
    fn test_validate_working_time_insufficient_rest() {
        let now = Utc::now();
        let record = WorkingTimeRecord {
            date: now,
            start_time: now,
            end_time: now + Duration::hours(9),
            rest_minutes: 30, // Should be 60 for 9 hour shift
            is_holiday: false,
        };

        assert!(matches!(
            validate_working_time_record(&record),
            Err(LaborLawError::InsufficientRestPeriod { .. })
        ));
    }

    #[test]
    fn test_validate_termination_notice_valid() {
        let notice = TerminationNotice {
            employee_name: "Test Employee".to_string(),
            termination_type: TerminationType::OrdinaryDismissal,
            notice_date: Utc::now(),
            effective_date: Utc::now() + Duration::days(30),
            reason: "Performance issues documented over 6 months".to_string(),
            severance_pay_jpy: Some(500_000),
            notice_allowance_jpy: None,
        };

        assert!(validate_termination_notice(&notice, 10_000).is_ok());
    }

    #[test]
    fn test_validate_termination_insufficient_notice() {
        let notice = TerminationNotice {
            employee_name: "Test Employee".to_string(),
            termination_type: TerminationType::OrdinaryDismissal,
            notice_date: Utc::now(),
            effective_date: Utc::now() + Duration::days(15), // Only 15 days
            reason: "Performance issues".to_string(),
            severance_pay_jpy: None,
            notice_allowance_jpy: None, // Missing required allowance
        };

        assert!(matches!(
            validate_termination_notice(&notice, 10_000),
            Err(LaborLawError::MissingNoticeAllowance { .. })
        ));
    }

    #[test]
    fn test_validate_minimum_wage() {
        assert!(validate_minimum_wage(1_500, 1_000).is_ok());
        assert!(matches!(
            validate_minimum_wage(900, 1_000),
            Err(LaborLawError::BelowMinimumWage { .. })
        ));
    }

    #[test]
    fn test_validate_wage_payment() {
        let now = Utc::now();
        let payment = WagePayment {
            employee_name: "Test".to_string(),
            period_start: now,
            period_end: now + Duration::days(30),
            payment_date: now + Duration::days(31),
            base_wage_jpy: 300_000,
            overtime_pay_jpy: 50_000,
            other_allowances_jpy: 10_000,
            deductions_jpy: 60_000,
            net_payment_jpy: 300_000,
        };

        assert!(validate_wage_payment(&payment).is_ok());
    }
}
