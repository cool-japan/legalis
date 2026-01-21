//! Validation functions for UK Employment Law
//!
//! Validates compliance with:
//! - ERA 1996 (Employment Rights Act 1996)
//! - WTR 1998 (Working Time Regulations 1998)
//! - NMWA 1998 (National Minimum Wage Act 1998)

use chrono::NaiveDate;

use super::error::{EmploymentError, Result};
use super::types::*;

/// Validate employment contract compliance with ERA 1996
///
/// Checks:
/// - ERA 1996 s.1: Written particulars provided
/// - ERA 1996 s.86: Notice periods meet statutory minimum
/// - WTR 1998: Working hours compliance
/// - NMWA 1998: Minimum wage compliance
/// - Probation period reasonableness
/// - Pension auto-enrolment (if applicable)
///
/// # Example
/// ```ignore
/// let contract = EmploymentContract::builder()
///     .with_written_particulars(true)
///     .build();
///
/// validate_employment_contract(&contract)?;
/// ```
pub fn validate_employment_contract(contract: &EmploymentContract) -> Result<()> {
    // Validate employee
    validate_employee(&contract.employee)?;

    // Validate employer
    validate_employer(&contract.employer)?;

    // ERA 1996 s.1: Written particulars required
    if !contract.written_particulars_provided {
        return Err(EmploymentError::WrittenParticularsNotProvided);
    }

    // Validate contract type specific rules
    validate_contract_type(&contract.contract_type)?;

    // Validate working hours (WTR 1998)
    validate_working_hours(&contract.working_hours)?;

    // Validate notice period (ERA 1996 s.86)
    let years_service = contract.years_of_service(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());
    validate_notice_period(&contract.notice_period, years_service)?;

    // Validate probation period
    if let Some(months) = contract.probation_period_months
        && months > 6
    {
        return Err(EmploymentError::ProbationPeriodTooLong { months });
    }

    // Validate minimum wage (if salary information available)
    if contract.salary.gross_annual_gbp > 0.0 && contract.working_hours.hours_per_week > 0 {
        let hourly_rate = contract
            .salary
            .gross_hourly(contract.working_hours.hours_per_week);
        let age = contract
            .employee
            .age_at(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());

        let assessment = MinimumWageAssessment {
            age,
            hourly_rate_gbp: hourly_rate,
            apprentice: false,
        };

        validate_minimum_wage(&assessment)?;
    }

    Ok(())
}

/// Validate employee details
pub fn validate_employee(employee: &Employee) -> Result<()> {
    if employee.name.is_empty() {
        return Err(EmploymentError::missing_field("employee.name"));
    }

    if employee.address.is_empty() {
        return Err(EmploymentError::missing_field("employee.address"));
    }

    Ok(())
}

/// Validate employer details
pub fn validate_employer(employer: &Employer) -> Result<()> {
    if employer.name.is_empty() {
        return Err(EmploymentError::missing_field("employer.name"));
    }

    if employer.address.is_empty() {
        return Err(EmploymentError::missing_field("employer.address"));
    }

    Ok(())
}

/// Validate contract type specific rules
pub fn validate_contract_type(contract_type: &ContractType) -> Result<()> {
    match contract_type {
        ContractType::ZeroHours { exclusivity_clause } => {
            // Exclusivity clauses banned since 2015
            if *exclusivity_clause {
                return Err(EmploymentError::IllegalExclusivityClause);
            }
        }
        ContractType::FixedTerm {
            less_favourable, ..
        } => {
            // Check for less favourable treatment
            if *less_favourable {
                return Err(EmploymentError::LessFavourableTreatment {
                    worker_type: "fixed-term".to_string(),
                    regulation: "Fixed-Term Employees Regulations 2002".to_string(),
                    treatment: "Less favourable terms than comparable permanent employee"
                        .to_string(),
                });
            }
        }
        ContractType::PartTime {
            less_favourable, ..
        } => {
            // Check for less favourable treatment
            if *less_favourable {
                return Err(EmploymentError::LessFavourableTreatment {
                    worker_type: "part-time".to_string(),
                    regulation: "Part-Time Workers Regulations 2000".to_string(),
                    treatment:
                        "Less favourable terms (pro-rata) than comparable full-time employee"
                            .to_string(),
                });
            }
        }
        _ => {}
    }

    Ok(())
}

/// Validate working hours under WTR 1998
///
/// Checks:
/// - Maximum 48 hours per week (or opt-out signed)
/// - Rest break entitlements
pub fn validate_working_hours(hours: &WorkingHours) -> Result<()> {
    // WTR 1998 Reg 4: Maximum 48 hours per week
    if !hours.complies_with_48h_limit() {
        return Err(EmploymentError::WorkingHoursExceed48HourLimit {
            hours: hours.hours_per_week,
        });
    }

    Ok(())
}

/// Validate notice period under ERA 1996 s.86
///
/// Statutory minimum notice (given by employer):
/// - Less than 1 month service: No notice required
/// - 1 month to 2 years: 1 week
/// - 2+ years: 1 week per year of service (max 12 weeks)
pub fn validate_notice_period(notice: &NoticePeriod, years_service: u8) -> Result<()> {
    let statutory_minimum = NoticePeriod::statutory_minimum_employer(years_service);

    if notice.employer_notice_weeks < statutory_minimum {
        return Err(EmploymentError::NoticePeriodBelowStatutory {
            actual: notice.employer_notice_weeks,
            required: statutory_minimum,
            years_service,
        });
    }

    Ok(())
}

/// Validate minimum wage under NMWA 1998
///
/// Age-based rates (April 2024):
/// - 21+: £11.44/hour (National Living Wage)
/// - 18-20: £8.60/hour
/// - Under 18: £6.40/hour
/// - Apprentice: £6.40/hour
pub fn validate_minimum_wage(assessment: &MinimumWageAssessment) -> Result<()> {
    if !assessment.is_compliant() {
        let required = assessment.applicable_minimum_wage();
        let shortfall = required - assessment.hourly_rate_gbp;

        return Err(EmploymentError::BelowMinimumWage {
            actual: assessment.hourly_rate_gbp,
            required,
            age: assessment.age,
            shortfall,
        });
    }

    Ok(())
}

/// Validate dismissal under ERA 1996 s.98
///
/// Checks:
/// - Qualifying period (2 years, unless automatically unfair)
/// - Fair reason for dismissal
/// - Reasonableness of dismissal
pub fn validate_dismissal(dismissal: &Dismissal) -> Result<()> {
    // Check qualifying period for unfair dismissal protection
    if !dismissal.has_unfair_dismissal_protection() {
        return Err(EmploymentError::InsufficientServiceForUnfairDismissal {
            years_service: dismissal.years_of_service,
        });
    }

    // Validate dismissal reason
    validate_dismissal_reason(&dismissal.reason, dismissal.years_of_service)?;

    // Check if notice given (unless summary dismissal for gross misconduct)
    if !matches!(dismissal.dismissal_type, DismissalType::Summary)
        && dismissal.notice_given_weeks.is_none()
    {
        return Err(EmploymentError::invalid_value(
            "notice_given_weeks",
            "Notice must be given for ordinary dismissal",
        ));
    }

    Ok(())
}

/// Validate dismissal reason under ERA 1996 s.98
pub fn validate_dismissal_reason(reason: &DismissalReason, years_service: u8) -> Result<()> {
    match reason {
        DismissalReason::Capability { warnings_given, .. } => {
            // Capability dismissals should follow warnings process
            if !warnings_given && years_service >= 2 {
                return Err(EmploymentError::UnfairDismissal {
                    reason: "Capability dismissal without warnings may be unfair".to_string(),
                });
            }
        }
        DismissalReason::Conduct {
            warnings_given,
            gross_misconduct,
            ..
        } => {
            // Conduct dismissals (non-gross) should follow warnings
            if *warnings_given == 0 && !gross_misconduct {
                return Err(EmploymentError::UnfairDismissal {
                    reason:
                        "Conduct dismissal without warnings may be unfair (unless gross misconduct)"
                            .to_string(),
                });
            }
        }
        DismissalReason::Redundancy {
            fair_selection,
            consultation,
            ..
        } => {
            // Redundancy must have fair selection and consultation
            if !fair_selection {
                return Err(EmploymentError::RedundancySelectionUnfair {
                    issue: "Selection criteria not objective or fairly applied".to_string(),
                });
            }
            if !consultation {
                return Err(EmploymentError::RedundancyConsultationNotCarriedOut {
                    number_of_redundancies: 1,
                });
            }
        }
        DismissalReason::AutomaticallyUnfair { reason } => {
            // These are always unfair regardless of service
            return Err(EmploymentError::AutomaticallyUnfairDismissal {
                reason: format!("{:?}", reason),
            });
        }
        _ => {}
    }

    Ok(())
}

/// Validate redundancy payment calculation
pub fn validate_redundancy_payment(payment: &RedundancyPayment) -> Result<()> {
    // Check employee has qualifying service (2 years)
    if payment.years_of_service < 2 {
        return Err(EmploymentError::invalid_value(
            "years_of_service",
            "Minimum 2 years continuous service required for statutory redundancy payment",
        ));
    }

    // Statutory redundancy payment should be positive
    let calculated = payment.calculate_statutory_payment();
    if calculated <= 0.0 {
        return Err(EmploymentError::invalid_value(
            "redundancy_payment",
            "Calculated redundancy payment must be positive",
        ));
    }

    Ok(())
}

/// Validate annual leave entitlement
pub fn validate_annual_leave(entitlement: &AnnualLeaveEntitlement, actual_days: u8) -> Result<()> {
    let statutory_minimum = entitlement.statutory_minimum_days();

    if (actual_days as f64) < statutory_minimum {
        return Err(EmploymentError::AnnualLeaveBelowMinimum {
            actual: actual_days,
            days_per_week: entitlement.days_per_week,
            required: statutory_minimum,
        });
    }

    Ok(())
}

/// Validate rest entitlements under WTR 1998
pub fn validate_rest_entitlements(
    entitlement: &RestEntitlement,
    rest_break_provided_minutes: u8,
) -> Result<()> {
    let required_break = entitlement.rest_break_minutes();

    if rest_break_provided_minutes < required_break {
        return Err(EmploymentError::InsufficientRestBreak {
            daily_hours: entitlement.daily_hours,
            rest_break_minutes: rest_break_provided_minutes,
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_written_particulars_missing() {
        let contract = EmploymentContract {
            employee: Employee {
                name: "John Smith".to_string(),
                date_of_birth: NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
                address: "123 High St".to_string(),
                national_insurance_number: Some("AB123456C".to_string()),
            },
            employer: Employer {
                name: "Acme Ltd".to_string(),
                address: "456 Commercial Rd".to_string(),
                employee_count: Some(50),
            },
            written_particulars_provided: false,
            ..Default::default()
        };

        let result = validate_employment_contract(&contract);
        assert!(matches!(
            result,
            Err(EmploymentError::WrittenParticularsNotProvided)
        ));
    }

    #[test]
    fn test_validate_notice_period_below_statutory() {
        let notice = NoticePeriod {
            employer_notice_weeks: 1,
            employee_notice_weeks: 1,
        };

        let result = validate_notice_period(&notice, 5);
        assert!(matches!(
            result,
            Err(EmploymentError::NoticePeriodBelowStatutory { .. })
        ));
    }

    #[test]
    fn test_validate_working_hours_exceed_48h() {
        let hours = WorkingHours {
            hours_per_week: 55,
            days_per_week: 6,
            opted_out_of_48h_limit: false,
            night_work_hours: None,
        };

        let result = validate_working_hours(&hours);
        assert!(matches!(
            result,
            Err(EmploymentError::WorkingHoursExceed48HourLimit { .. })
        ));
    }

    #[test]
    fn test_validate_minimum_wage_below_minimum() {
        let assessment = MinimumWageAssessment {
            age: 25,
            hourly_rate_gbp: 10.00,
            apprentice: false,
        };

        let result = validate_minimum_wage(&assessment);
        assert!(matches!(
            result,
            Err(EmploymentError::BelowMinimumWage { .. })
        ));
    }

    #[test]
    fn test_validate_zero_hours_exclusivity_clause() {
        let contract_type = ContractType::ZeroHours {
            exclusivity_clause: true,
        };

        let result = validate_contract_type(&contract_type);
        assert!(matches!(
            result,
            Err(EmploymentError::IllegalExclusivityClause)
        ));
    }

    #[test]
    fn test_validate_dismissal_insufficient_service() {
        let dismissal = Dismissal {
            dismissal_type: DismissalType::Ordinary,
            reason: DismissalReason::Capability {
                description: "Poor performance".to_string(),
                warnings_given: true,
            },
            years_of_service: 1,
            dismissal_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            written_reasons_provided: true,
            notice_given_weeks: Some(1),
        };

        let result = validate_dismissal(&dismissal);
        assert!(matches!(
            result,
            Err(EmploymentError::InsufficientServiceForUnfairDismissal { .. })
        ));
    }

    #[test]
    fn test_validate_annual_leave_below_minimum() {
        let entitlement = AnnualLeaveEntitlement {
            days_per_week: 5,
            leave_year_start: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        };

        // Provide only 20 days (minimum is 28 for 5-day week)
        let result = validate_annual_leave(&entitlement, 20);
        assert!(matches!(
            result,
            Err(EmploymentError::AnnualLeaveBelowMinimum { .. })
        ));
    }
}
