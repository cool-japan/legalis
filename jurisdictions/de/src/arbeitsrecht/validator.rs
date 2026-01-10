//! Validation functions for German Labor Law
//!
//! Implements validation logic for employment contracts, dismissals, and labor rights.

use chrono::{NaiveDate, Utc};

use super::error::{LaborLawError, Result};
use super::types::*;

/// Validate employment contract (Arbeitsvertrag)
pub fn validate_employment_contract(contract: &EmploymentContract) -> Result<()> {
    validate_employee(&contract.employee)?;
    validate_employer(&contract.employer)?;

    // §2 NachwG requires written documentation
    if !contract.written {
        return Err(LaborLawError::ContractNotWritten);
    }

    // Probation period max 6 months (§622 Abs. 3 BGB)
    if let Some(months) = contract.probation_period_months {
        if months > 6 {
            return Err(LaborLawError::ProbationTooLong);
        }
    }

    // Validate contract type specific rules
    validate_contract_type(
        &contract.contract_type,
        contract.start_date,
        contract.end_date,
    )?;

    // Validate working hours
    validate_working_hours(&contract.working_hours)?;

    Ok(())
}

fn validate_contract_type(
    contract_type: &ContractType,
    start_date: NaiveDate,
    end_date: Option<NaiveDate>,
) -> Result<()> {
    match contract_type {
        ContractType::FixedTerm { reason } => {
            // Fixed-term requires end date
            let end = end_date.ok_or_else(|| LaborLawError::MissingRequiredField {
                field: "end_date for fixed-term contract".to_string(),
            })?;

            // §14 Abs. 2 TzBfG: Without reason, max 2 years
            if matches!(reason, FixedTermReason::NoReasonNeeded) {
                let duration_days = (end - start_date).num_days();
                if duration_days > 730 {
                    // 2 years
                    return Err(LaborLawError::FixedTermTooLong);
                }
            }
        }
        ContractType::Unlimited => {
            // Unlimited should not have end date
            if end_date.is_some() {
                return Err(LaborLawError::InvalidValue {
                    reason: "Unlimited contract cannot have end date".to_string(),
                });
            }
        }
        _ => {}
    }
    Ok(())
}

pub fn validate_employee(employee: &Employee) -> Result<()> {
    if employee.name.trim().is_empty() {
        return Err(LaborLawError::EmptyName);
    }
    Ok(())
}

pub fn validate_employer(employer: &Employer) -> Result<()> {
    if employer.name.trim().is_empty() {
        return Err(LaborLawError::EmptyName);
    }
    Ok(())
}

/// Validate working hours per ArbZG
pub fn validate_working_hours(hours: &WorkingHours) -> Result<()> {
    if !hours.complies_with_arbzg() {
        let hours_per_day = hours.hours_per_week as f32 / hours.days_per_week as f32;
        return Err(LaborLawError::WorkingHoursExceeded {
            hours: hours_per_day,
        });
    }
    Ok(())
}

/// Validate dismissal (Kündigung)
pub fn validate_dismissal(dismissal: &Dismissal, company_size: CompanySize) -> Result<()> {
    // §623 BGB requires written form
    if !dismissal.written {
        return Err(LaborLawError::DismissalNotWritten);
    }

    // §102 BetrVG requires works council consultation if exists
    if company_size.has_dismissal_protection() && !dismissal.works_council_consulted {
        return Err(LaborLawError::WorksCouncilNotConsulted);
    }

    // Validate dismissal grounds for companies with KSchG protection
    if company_size.has_dismissal_protection() {
        validate_dismissal_grounds(&dismissal.grounds)?;
    }

    // Validate notice period
    validate_notice_period(dismissal.dismissal_type, dismissal.notice_period_weeks)?;

    Ok(())
}

fn validate_dismissal_grounds(grounds: &DismissalGrounds) -> Result<()> {
    match grounds {
        DismissalGrounds::Conduct {
            description,
            warnings,
        } => {
            if description.trim().is_empty() {
                return Err(LaborLawError::MissingRequiredField {
                    field: "conduct description".to_string(),
                });
            }
            // Generally requires prior warnings (except severe cases)
            if *warnings == 0 {
                return Err(LaborLawError::NoJustifiedDismissalReason);
            }
        }
        DismissalGrounds::Personal { description }
        | DismissalGrounds::Operational { description }
        | DismissalGrounds::ExtraordinaryCause { description } => {
            if description.trim().is_empty() {
                return Err(LaborLawError::MissingRequiredField {
                    field: "dismissal description".to_string(),
                });
            }
        }
    }
    Ok(())
}

fn validate_notice_period(dismissal_type: DismissalType, weeks: u8) -> Result<()> {
    match dismissal_type {
        DismissalType::Ordinary => {
            // Minimum 4 weeks (§622 BGB)
            if weeks < 4 {
                return Err(LaborLawError::InsufficientNoticePeriod {
                    actual: weeks,
                    required: 4,
                });
            }
        }
        DismissalType::Extraordinary => {
            // No notice period for extraordinary dismissal
            if weeks > 0 {
                return Err(LaborLawError::InvalidValue {
                    reason: "Extraordinary dismissal has no notice period".to_string(),
                });
            }
        }
    }
    Ok(())
}

/// Validate leave entitlement per BUrlG
pub fn validate_leave_entitlement(leave: &LeaveEntitlement) -> Result<()> {
    let minimum = LeaveEntitlement::calculate_minimum(leave.days_per_week);

    // §3 BUrlG requires minimum
    if leave.contractual_days < minimum {
        return Err(LaborLawError::LeaveBelowMinimum {
            actual: leave.contractual_days,
            required: minimum,
        });
    }

    Ok(())
}

/// Validate sick leave per EFZG
pub fn validate_sick_leave(sick_leave: &SickLeave) -> Result<()> {
    let current_date = Utc::now().date_naive();

    // §5 EFZG requires medical certificate after 3 days
    if sick_leave.duration_days(current_date) > 3 && !sick_leave.medical_certificate_provided {
        return Err(LaborLawError::MedicalCertificateMissing);
    }

    // Must notify employer
    if !sick_leave.notification_timely {
        return Err(LaborLawError::EmployerNotNotified);
    }

    Ok(())
}

/// Validate parental leave per BEEG
pub fn validate_parental_leave(leave: &ParentalLeave) -> Result<()> {
    // §15 BEEG: Max 3 years
    if leave.duration_years() > ParentalLeave::MAX_YEARS as f32 {
        return Err(LaborLawError::ParentalLeaveTooLong {
            years: leave.duration_years(),
        });
    }

    // §16 BEEG: Min 7 weeks notice
    if leave.notice_given_weeks < 7 {
        return Err(LaborLawError::ParentalLeaveNoticeTooLate {
            weeks: leave.notice_given_weeks,
        });
    }

    Ok(())
}

/// Validate works council per BetrVG
pub fn validate_works_council(council: &WorksCouncil) -> Result<()> {
    let required_size = WorksCouncil::required_size(council.employee_count);

    if council.council_members.len() != required_size as usize {
        return Err(LaborLawError::WorksCouncilSizeIncorrect {
            actual: council.council_members.len() as u8,
            required: required_size,
            employee_count: council.employee_count,
        });
    }

    Ok(())
}

// ===== Collective Labor Law Validators =====

/// Validate collective bargaining agreement (Tarifvertrag) per TVG
pub fn validate_collective_agreement(agreement: &CollectiveBargainingAgreement) -> Result<()> {
    // Agreement name must not be empty
    if agreement.agreement_name.trim().is_empty() {
        return Err(LaborLawError::EmptyName);
    }

    // Must have at least one party (union)
    if agreement.parties.union.name.trim().is_empty() {
        return Err(LaborLawError::EmptyName);
    }

    // Must have either employer association or individual employer
    if agreement.parties.employer_association.is_none()
        && agreement.parties.individual_employer.is_none()
    {
        return Err(LaborLawError::MissingRequiredField {
            field: "employer party (association or individual)".to_string(),
        });
    }

    // Effective date must be before expiry date if expiry is set
    if let Some(expiry) = agreement.expiry_date {
        if expiry <= agreement.effective_date {
            return Err(LaborLawError::InvalidValue {
                reason: "Expiry date must be after effective date".to_string(),
            });
        }
    }

    // Must have at least one normative provision (§1 TVG)
    if agreement.normative_provisions.is_empty() {
        return Err(LaborLawError::MissingRequiredField {
            field: "normative provisions (§1 TVG)".to_string(),
        });
    }

    Ok(())
}

/// Validate supervisory board composition per MitbestG/DrittelbG
pub fn validate_supervisory_board(board: &SupervisoryBoard) -> Result<()> {
    // Determine required co-determination type
    let required_type = SupervisoryBoard::required_codetermination(board.employee_count);

    // Check if correct co-determination type is used
    if board.codetermination_type != required_type {
        return Err(LaborLawError::InvalidValue {
            reason: format!(
                "Company with {} employees requires {:?} co-determination, not {:?}",
                board.employee_count, required_type, board.codetermination_type
            ),
        });
    }

    // Check board size
    let required_size = SupervisoryBoard::required_size(board.employee_count);
    if board.total_members != required_size {
        return Err(LaborLawError::WorksCouncilSizeIncorrect {
            actual: board.total_members,
            required: required_size,
            employee_count: board.employee_count,
        });
    }

    // Validate employee representation ratios
    match board.codetermination_type {
        CodeterminationType::None => {
            // No employee representatives required
            if board.employee_representatives > 0 {
                return Err(LaborLawError::InvalidValue {
                    reason: "No employee representation required for companies < 500 employees"
                        .to_string(),
                });
            }
        }
        CodeterminationType::OneThird => {
            // 1/3 employee representation (DrittelbG)
            let expected_employee_reps = board.total_members / 3;
            if board.employee_representatives != expected_employee_reps {
                return Err(LaborLawError::InvalidValue {
                    reason: format!(
                        "DrittelbG requires 1/3 employee representation ({} of {} members)",
                        expected_employee_reps, board.total_members
                    ),
                });
            }
        }
        CodeterminationType::Full => {
            // 50% employee representation (MitbestG)
            let expected_employee_reps = board.total_members / 2;
            if board.employee_representatives != expected_employee_reps {
                return Err(LaborLawError::InvalidValue {
                    reason: format!(
                        "MitbestG requires 50% employee representation ({} of {} members)",
                        expected_employee_reps, board.total_members
                    ),
                });
            }
        }
        CodeterminationType::MontanMitbestimmung => {
            // Special rules for coal and steel industry
            // Simplified check: 50% representation
            let expected_employee_reps = board.total_members / 2;
            if board.employee_representatives != expected_employee_reps {
                return Err(LaborLawError::InvalidValue {
                    reason: "Montan-Mitbestimmung requires 50% employee representation".to_string(),
                });
            }
        }
    }

    // Verify total representation adds up
    if board.employee_representatives + board.shareholder_representatives != board.total_members {
        return Err(LaborLawError::InvalidValue {
            reason: format!(
                "Employee ({}) + shareholder ({}) representatives must equal total members ({})",
                board.employee_representatives,
                board.shareholder_representatives,
                board.total_members
            ),
        });
    }

    Ok(())
}

/// Validate co-determination rights per BetrVG §87
pub fn validate_codetermination_rights(rights: &CodeterminationRights) -> Result<()> {
    // Company must have works council (5+ employees)
    if !WorksCouncil::is_required(rights.employee_count) {
        return Err(LaborLawError::InvalidValue {
            reason: "Co-determination rights require works council (5+ employees)".to_string(),
        });
    }

    // Must have at least one right defined
    if rights.rights.is_empty() {
        return Err(LaborLawError::MissingRequiredField {
            field: "co-determination rights".to_string(),
        });
    }

    // Validate each right has proper description and legal basis
    for right in &rights.rights {
        if right.description.trim().is_empty() {
            return Err(LaborLawError::MissingRequiredField {
                field: "co-determination right description".to_string(),
            });
        }
        if right.legal_basis.trim().is_empty() {
            return Err(LaborLawError::MissingRequiredField {
                field: "legal basis for co-determination right".to_string(),
            });
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gmbhg::Capital;
    use chrono::NaiveDate;

    #[test]
    fn test_valid_employment_contract() {
        let contract = EmploymentContract {
            employee: Employee {
                name: "Max Mustermann".to_string(),
                date_of_birth: NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
                address: "Berlin".to_string(),
                social_security_number: None,
            },
            employer: Employer {
                name: "Tech GmbH".to_string(),
                address: "Munich".to_string(),
                company_size: CompanySize::Medium,
            },
            contract_type: ContractType::Unlimited,
            start_date: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
            end_date: None,
            probation_period_months: Some(6),
            salary: Salary {
                gross_monthly: Capital::from_euros(3000),
                payment_day: 1,
                includes_overtime: false,
            },
            working_hours: WorkingHours {
                hours_per_week: 40,
                days_per_week: 5,
                overtime_allowed: true,
            },
            duties: "Software development".to_string(),
            written: true,
        };

        assert!(validate_employment_contract(&contract).is_ok());
    }

    #[test]
    fn test_probation_too_long() {
        let mut contract = create_valid_contract();
        contract.probation_period_months = Some(7);

        assert!(matches!(
            validate_employment_contract(&contract),
            Err(LaborLawError::ProbationTooLong)
        ));
    }

    #[test]
    fn test_working_hours_exceeded() {
        let hours = WorkingHours {
            hours_per_week: 60, // 12 hours/day for 5 days - exceeds ArbZG
            days_per_week: 5,
            overtime_allowed: true,
        };

        assert!(matches!(
            validate_working_hours(&hours),
            Err(LaborLawError::WorkingHoursExceeded { .. })
        ));
    }

    #[test]
    fn test_leave_below_minimum() {
        let leave = LeaveEntitlement {
            employee_name: "Test".to_string(),
            year: 2024,
            days_per_week: 5,
            minimum_days: 20,
            contractual_days: 15, // Below minimum!
            days_taken: 0,
            days_carried_over: 0,
        };

        assert!(matches!(
            validate_leave_entitlement(&leave),
            Err(LaborLawError::LeaveBelowMinimum { .. })
        ));
    }

    fn create_valid_contract() -> EmploymentContract {
        EmploymentContract {
            employee: Employee {
                name: "Test Employee".to_string(),
                date_of_birth: NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
                address: "Berlin".to_string(),
                social_security_number: None,
            },
            employer: Employer {
                name: "Test Company".to_string(),
                address: "Munich".to_string(),
                company_size: CompanySize::Medium,
            },
            contract_type: ContractType::Unlimited,
            start_date: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
            end_date: None,
            probation_period_months: Some(6),
            salary: Salary {
                gross_monthly: Capital::from_euros(3000),
                payment_day: 1,
                includes_overtime: false,
            },
            working_hours: WorkingHours {
                hours_per_week: 40,
                days_per_week: 5,
                overtime_allowed: true,
            },
            duties: "Test".to_string(),
            written: true,
        }
    }
}
