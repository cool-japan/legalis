//! Labor law validation (Validation du droit du travail)
//!
//! Comprehensive validation functions for French employment law compliance.

use super::error::{LaborLawError, ValidationResult};
use super::types::{
    CDDReason, DismissalType, EmploymentContract, EmploymentContractType, PersonalCause,
    TrialPeriodCategory, WorkingHours,
};

/// Current French minimum wage (SMIC) as of 2024
/// Updated periodically by law
pub const SMIC_HOURLY: f32 = 11.65; // €11.65/hour (2024 rate)

/// Validates a CDD (fixed-term contract) for legal compliance
///
/// ## Validation checks (Vérifications)
///
/// 1. **Duration**: Max 18 months including renewals (Article L1242-8)
/// 2. **Written form**: CDD must be written (Article L1242-12)
/// 3. **Authorized reason**: Must have valid justification (Article L1242-2)
///
/// ## Example
///
/// ```
/// use legalis_fr::labor::{validate_cdd, EmploymentContractType, CDDReason};
/// use chrono::{Utc, Duration};
///
/// let cdd = EmploymentContractType::CDD {
///     duration_months: 12,
///     reason: CDDReason::ReplacementAbsentEmployee,
///     end_date: (Utc::now() + Duration::days(365)).naive_utc().date(),
/// };
///
/// assert!(validate_cdd(&cdd, true).is_ok());
/// ```
pub fn validate_cdd(
    contract_type: &EmploymentContractType,
    is_written: bool,
) -> ValidationResult<()> {
    // Must be a CDD
    let (duration_months, reason) = match contract_type {
        EmploymentContractType::CDD {
            duration_months,
            reason,
            ..
        } => (duration_months, reason),
        _ => {
            return Err(LaborLawError::InvalidCDDReason {
                reason: CDDReason::ReplacementAbsentEmployee, // Placeholder
            });
        }
    };

    // Article L1242-12: CDD must be written
    if !is_written {
        return Err(LaborLawError::CDDNotWritten);
    }

    // Article L1242-8: Maximum duration 18 months
    if *duration_months > 18 {
        return Err(LaborLawError::CDDDurationExceeded {
            months: *duration_months,
        });
    }

    // Article L1242-2: Check if reason is authorized
    // All CDDReason enum variants are authorized, but we validate they exist
    match reason {
        CDDReason::ReplacementAbsentEmployee
        | CDDReason::TemporaryIncreaseActivity
        | CDDReason::SeasonalWork
        | CDDReason::SpecificProject
        | CDDReason::PendingRecruitment => Ok(()),
    }
}

/// Validates trial period duration for employment category
///
/// ## Maximum periods (Durées maximales) - Article L1221-19
///
/// - **Workers/Employees** (Ouvriers/Employés): 2 months
/// - **Supervisors/Technicians** (Agents de maîtrise/Techniciens): 3 months
/// - **Executives** (Cadres): 4 months
///
/// Trial periods can be renewed once (max 2x original period).
///
/// ## Example
///
/// ```
/// use legalis_fr::labor::{validate_trial_period, TrialPeriodCategory};
///
/// assert!(validate_trial_period(TrialPeriodCategory::WorkersEmployees, 2).is_ok());
/// assert!(validate_trial_period(TrialPeriodCategory::Executives, 5).is_err());
/// ```
pub fn validate_trial_period(
    category: TrialPeriodCategory,
    actual_months: u8,
) -> ValidationResult<()> {
    let max_months = match category {
        TrialPeriodCategory::WorkersEmployees => 2,
        TrialPeriodCategory::SupervisorsTechnicians => 3,
        TrialPeriodCategory::Executives => 4,
    };

    if actual_months > max_months {
        return Err(LaborLawError::TrialPeriodTooLong {
            category,
            max: max_months,
            actual: actual_months,
        });
    }

    Ok(())
}

/// Validates working hours compliance
///
/// ## Legal limits (Limites légales)
///
/// - **Weekly hours**: Max 48 hours (Article L3121-20)
/// - **Daily hours**: Max 10 hours (Article L3121-18)
/// - **Legal duration**: 35 hours/week (Article L3121-27)
///
/// ## Example
///
/// ```
/// use legalis_fr::labor::{validate_working_hours, WorkingHours};
///
/// let hours = WorkingHours {
///     weekly_hours: 39.0,
///     daily_hours: Some(8.0),
/// };
///
/// assert!(validate_working_hours(&hours).is_ok());
/// ```
pub fn validate_working_hours(hours: &WorkingHours) -> ValidationResult<()> {
    // Article L3121-20: Maximum 48 hours per week
    if hours.weekly_hours > WorkingHours::MAX_WEEKLY_HOURS {
        return Err(LaborLawError::WeeklyHoursExceeded {
            max: WorkingHours::MAX_WEEKLY_HOURS,
            actual: hours.weekly_hours,
        });
    }

    // Article L3121-18: Maximum 10 hours per day
    if let Some(daily) = hours.daily_hours {
        if daily > WorkingHours::MAX_DAILY_HOURS {
            return Err(LaborLawError::DailyHoursExceeded {
                max: WorkingHours::MAX_DAILY_HOURS,
                actual: daily,
            });
        }
    }

    Ok(())
}

/// Validates dismissal for real and serious cause
///
/// ## Requirements (Exigences) - Article L1232-1
///
/// All dismissals require:
/// 1. **Real cause** (Cause réelle): Objectively verifiable facts
/// 2. **Serious cause** (Cause sérieuse): Sufficient gravity to justify termination
/// 3. **Pre-dismissal interview** (Entretien préalable): Employee must be heard
///
/// ### Personal dismissal (Licenciement personnel)
/// - Misconduct, incompetence, insubordination, etc.
///
/// ### Economic dismissal (Licenciement économique) - Article L1233-3
/// - Economic difficulties (Difficultés économiques)
/// - Job elimination (Suppression de poste)
/// - Technological changes, reorganization, etc.
///
/// ## Example
///
/// ```
/// use legalis_fr::labor::{validate_dismissal, DismissalType, PersonalCause};
///
/// let dismissal = DismissalType::Personal {
///     cause: PersonalCause::SimpleFault,
///     serious_misconduct: false,
/// };
///
/// assert!(validate_dismissal(&dismissal, true).is_ok());
/// ```
pub fn validate_dismissal(dismissal: &DismissalType, interview_held: bool) -> ValidationResult<()> {
    // Article L1232-2: Pre-dismissal interview required
    if !interview_held {
        return Err(LaborLawError::MissingDismissalInterview);
    }

    match dismissal {
        DismissalType::Personal { cause, .. } => {
            // Validate personal cause exists
            match cause {
                PersonalCause::SimpleFault
                | PersonalCause::SeriousMisconduct
                | PersonalCause::GrossMisconduct
                | PersonalCause::Incompetence
                | PersonalCause::OtherRealSerious => Ok(()),
            }
        }
        DismissalType::Economic {
            economic_difficulties,
            job_eliminated,
            ..
        } => {
            // Article L1233-3: Economic dismissal requires BOTH conditions
            if !(*economic_difficulties && *job_eliminated) {
                return Err(LaborLawError::InvalidEconomicDismissal);
            }
            Ok(())
        }
    }
}

/// Validates notice period for dismissal
///
/// ## Minimum notice periods (Préavis minimums) - Article L1234-1
///
/// Based on seniority (ancienneté):
/// - **< 6 months**: No legal minimum (collective agreement may apply)
/// - **6 months - 2 years**: 1 month
/// - **≥ 2 years**: 2 months
///
/// ## Example
///
/// ```
/// use legalis_fr::labor::validate_notice_period;
///
/// // 3 years seniority requires 2 months notice
/// assert!(validate_notice_period(36, 2).is_ok());
/// assert!(validate_notice_period(36, 1).is_err());
/// ```
pub fn validate_notice_period(seniority_months: u32, notice_months: u8) -> ValidationResult<()> {
    let required_notice = if seniority_months < 6 {
        0 // No legal minimum (collective agreement may impose)
    } else if seniority_months < 24 {
        1 // 1 month for 6 months - 2 years
    } else {
        2 // 2 months for ≥ 2 years
    };

    if notice_months < required_notice {
        return Err(LaborLawError::InsufficientNotice {
            required: required_notice,
            actual: notice_months,
        });
    }

    Ok(())
}

/// Validates minimum wage compliance (SMIC)
///
/// ## Legal minimum wage (Salaire minimum interprofessionnel de croissance)
///
/// - Current rate: €11.65/hour (2024)
/// - Updated regularly by government decree
/// - Applies to all employees in France
///
/// ## Example
///
/// ```
/// use legalis_fr::labor::validate_minimum_wage;
///
/// assert!(validate_minimum_wage(12.50).is_ok());  // Above SMIC
/// assert!(validate_minimum_wage(10.00).is_err()); // Below SMIC
/// ```
pub fn validate_minimum_wage(hourly_rate: f32) -> ValidationResult<()> {
    if hourly_rate < SMIC_HOURLY {
        return Err(LaborLawError::BelowMinimumWage {
            smic: SMIC_HOURLY,
            actual: hourly_rate,
        });
    }

    Ok(())
}

/// Validates complete employment contract for legal compliance
///
/// ## Comprehensive validation (Validation complète)
///
/// Checks:
/// 1. Contract type specific requirements (CDD/CDI)
/// 2. Minimum wage compliance
/// 3. Working hours limits
///
/// ## Example
///
/// ```
/// use legalis_fr::labor::{validate_employment_contract, EmploymentContract, EmploymentContractType, WorkingHours};
///
/// let contract = EmploymentContract::new(
///     EmploymentContractType::CDI,
///     "Jean Dupont".to_string(),
///     "TechCorp SA".to_string(),
/// )
/// .with_hourly_rate(12.50)
/// .with_working_hours(WorkingHours {
///     weekly_hours: 35.0,
///     daily_hours: Some(7.0),
/// });
///
/// assert!(validate_employment_contract(&contract, true).is_ok());
/// ```
pub fn validate_employment_contract(
    contract: &EmploymentContract,
    is_written: bool,
) -> ValidationResult<()> {
    let mut errors = Vec::new();

    // Validate minimum wage
    if let Err(e) = validate_minimum_wage(contract.hourly_rate) {
        errors.push(e);
    }

    // Validate working hours
    if let Err(e) = validate_working_hours(&contract.working_hours) {
        errors.push(e);
    }

    // Validate contract-type specific requirements
    match &contract.contract_type {
        EmploymentContractType::CDD { .. } => {
            if let Err(e) = validate_cdd(&contract.contract_type, is_written) {
                errors.push(e);
            }
        }
        EmploymentContractType::CDI => {
            // CDI has no duration limit, minimal specific requirements
        }
        EmploymentContractType::Interim { .. } => {
            // Interim contracts have specific rules (simplified here)
            if !is_written {
                errors.push(LaborLawError::CDDNotWritten); // Interim also requires written form
            }
        }
        EmploymentContractType::Apprenticeship { .. } => {
            // Apprenticeship contracts have specific rules (simplified here)
            if !is_written {
                errors.push(LaborLawError::CDDNotWritten); // Also requires written form
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else if errors.len() == 1 {
        Err(errors.into_iter().next().unwrap())
    } else {
        Err(LaborLawError::MultipleErrors(errors))
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, Utc};

    use super::*;

    #[test]
    fn test_validate_cdd_valid() {
        let cdd = EmploymentContractType::CDD {
            duration_months: 12,
            reason: CDDReason::ReplacementAbsentEmployee,
            end_date: (Utc::now() + Duration::days(365)).naive_utc().date(),
        };

        assert!(validate_cdd(&cdd, true).is_ok());
    }

    #[test]
    fn test_validate_cdd_too_long() {
        let cdd = EmploymentContractType::CDD {
            duration_months: 24, // Exceeds 18 months!
            reason: CDDReason::TemporaryIncreaseActivity,
            end_date: (Utc::now() + Duration::days(730)).naive_utc().date(),
        };

        let result = validate_cdd(&cdd, true);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            LaborLawError::CDDDurationExceeded { months: 24 }
        ));
    }

    #[test]
    fn test_validate_cdd_not_written() {
        let cdd = EmploymentContractType::CDD {
            duration_months: 6,
            reason: CDDReason::SeasonalWork,
            end_date: (Utc::now() + Duration::days(180)).naive_utc().date(),
        };

        let result = validate_cdd(&cdd, false); // Not written!
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), LaborLawError::CDDNotWritten));
    }

    #[test]
    fn test_validate_trial_period_workers() {
        assert!(validate_trial_period(TrialPeriodCategory::WorkersEmployees, 2).is_ok());
        assert!(validate_trial_period(TrialPeriodCategory::WorkersEmployees, 3).is_err());
    }

    #[test]
    fn test_validate_trial_period_executives() {
        assert!(validate_trial_period(TrialPeriodCategory::Executives, 4).is_ok());
        assert!(validate_trial_period(TrialPeriodCategory::Executives, 5).is_err());
    }

    #[test]
    fn test_validate_working_hours_valid() {
        let hours = WorkingHours {
            weekly_hours: 35.0,
            daily_hours: Some(7.0),
        };
        assert!(validate_working_hours(&hours).is_ok());
    }

    #[test]
    fn test_validate_working_hours_weekly_exceeded() {
        let hours = WorkingHours {
            weekly_hours: 50.0, // Exceeds 48!
            daily_hours: Some(8.0),
        };
        let result = validate_working_hours(&hours);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            LaborLawError::WeeklyHoursExceeded { .. }
        ));
    }

    #[test]
    fn test_validate_working_hours_daily_exceeded() {
        let hours = WorkingHours {
            weekly_hours: 40.0,
            daily_hours: Some(12.0), // Exceeds 10!
        };
        let result = validate_working_hours(&hours);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            LaborLawError::DailyHoursExceeded { .. }
        ));
    }

    #[test]
    fn test_validate_dismissal_personal_valid() {
        let dismissal = DismissalType::Personal {
            cause: PersonalCause::SimpleFault,
            serious_misconduct: false,
        };
        assert!(validate_dismissal(&dismissal, true).is_ok());
    }

    #[test]
    fn test_validate_dismissal_no_interview() {
        let dismissal = DismissalType::Personal {
            cause: PersonalCause::Incompetence,
            serious_misconduct: false,
        };
        let result = validate_dismissal(&dismissal, false);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            LaborLawError::MissingDismissalInterview
        ));
    }

    #[test]
    fn test_validate_dismissal_economic_valid() {
        let dismissal = DismissalType::Economic {
            economic_difficulties: true,
            job_eliminated: true,
            affected_count: 1,
        };
        assert!(validate_dismissal(&dismissal, true).is_ok());
    }

    #[test]
    fn test_validate_dismissal_economic_invalid() {
        let dismissal = DismissalType::Economic {
            economic_difficulties: true,
            job_eliminated: false, // Missing job elimination!
            affected_count: 1,
        };
        let result = validate_dismissal(&dismissal, true);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            LaborLawError::InvalidEconomicDismissal
        ));
    }

    #[test]
    fn test_validate_notice_period_short_seniority() {
        // 3 months seniority - no legal minimum
        assert!(validate_notice_period(3, 0).is_ok());
    }

    #[test]
    fn test_validate_notice_period_medium_seniority() {
        // 12 months seniority - 1 month required
        assert!(validate_notice_period(12, 1).is_ok());
        assert!(validate_notice_period(12, 0).is_err());
    }

    #[test]
    fn test_validate_notice_period_long_seniority() {
        // 36 months seniority - 2 months required
        assert!(validate_notice_period(36, 2).is_ok());
        assert!(validate_notice_period(36, 1).is_err());
    }

    #[test]
    fn test_validate_minimum_wage_valid() {
        assert!(validate_minimum_wage(12.50).is_ok()); // Above SMIC
        assert!(validate_minimum_wage(SMIC_HOURLY).is_ok()); // Exactly SMIC
    }

    #[test]
    fn test_validate_minimum_wage_too_low() {
        let result = validate_minimum_wage(10.00); // Below SMIC
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            LaborLawError::BelowMinimumWage { .. }
        ));
    }

    #[test]
    fn test_validate_employment_contract_cdi_valid() {
        let contract = EmploymentContract::new(
            EmploymentContractType::CDI,
            "Jean Dupont".to_string(),
            "TechCorp SA".to_string(),
        )
        .with_hourly_rate(12.50)
        .with_working_hours(WorkingHours {
            weekly_hours: 35.0,
            daily_hours: Some(7.0),
        });

        assert!(validate_employment_contract(&contract, true).is_ok());
    }

    #[test]
    fn test_validate_employment_contract_below_smic() {
        let contract = EmploymentContract::new(
            EmploymentContractType::CDI,
            "Jean Dupont".to_string(),
            "BadEmployer".to_string(),
        )
        .with_hourly_rate(9.00); // Below SMIC!

        let result = validate_employment_contract(&contract, true);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_employment_contract_excessive_hours() {
        let contract = EmploymentContract::new(
            EmploymentContractType::CDI,
            "Jean Dupont".to_string(),
            "Employer".to_string(),
        )
        .with_hourly_rate(12.50)
        .with_working_hours(WorkingHours {
            weekly_hours: 55.0, // Exceeds 48!
            daily_hours: Some(8.0),
        });

        let result = validate_employment_contract(&contract, true);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_employment_contract_multiple_errors() {
        let contract = EmploymentContract::new(
            EmploymentContractType::CDI,
            "Jean Dupont".to_string(),
            "BadEmployer".to_string(),
        )
        .with_hourly_rate(9.00) // Below SMIC!
        .with_working_hours(WorkingHours {
            weekly_hours: 55.0,      // Exceeds 48!
            daily_hours: Some(12.0), // Exceeds 10!
        });

        let result = validate_employment_contract(&contract, true);
        assert!(result.is_err());
        if let Err(LaborLawError::MultipleErrors(errors)) = result {
            assert!(errors.len() >= 2); // At least wage and weekly hours errors
        } else {
            panic!("Expected MultipleErrors");
        }
    }
}
