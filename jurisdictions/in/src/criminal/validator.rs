//! BNS 2023 Validation
//!
//! Validation logic for criminal proceedings under BNS 2023.

use super::error::{BnsError, BnsResult, CriminalComplianceReport};
use super::types::*;
use chrono::{NaiveDate, Utc};

/// Validate FIR registration timing
pub fn validate_fir_registration(
    offence_date: NaiveDate,
    fir_date: Option<NaiveDate>,
    is_cognizable: bool,
) -> BnsResult<()> {
    if !is_cognizable {
        return Ok(()); // Non-cognizable offences don't require immediate FIR
    }

    match fir_date {
        None => Err(BnsError::FirNotRegistered { delay_hours: 0 }),
        Some(fir) => {
            let delay = (fir - offence_date).num_hours() as u32;
            // FIR should be registered promptly
            if delay > 48 {
                // More than 48 hours delay is generally considered excessive
                return Err(BnsError::FirNotRegistered { delay_hours: delay });
            }
            Ok(())
        }
    }
}

/// Validate investigation timeline
pub fn validate_investigation_timeline(
    fir_date: NaiveDate,
    chargesheet_date: Option<NaiveDate>,
    offence: &Offence,
    accused_in_custody: bool,
) -> BnsResult<()> {
    let today = Utc::now().date_naive();

    match chargesheet_date {
        Some(cs_date) => {
            let days = (cs_date - fir_date).num_days() as u32;

            // Check based on custody type and offence severity
            let max_days = if accused_in_custody {
                if offence.is_cognizable() && !offence.is_bailable() {
                    90 // 90 days for non-bailable offences
                } else {
                    60 // 60 days for other offences
                }
            } else {
                90 // When not in custody, same timeline applies
            };

            if days > max_days {
                return Err(BnsError::ChargesheetDelay {
                    custody_type: if accused_in_custody {
                        "In custody".to_string()
                    } else {
                        "On bail".to_string()
                    },
                    days,
                });
            }
        }
        None => {
            let days_elapsed = (today - fir_date).num_days() as u32;
            if days_elapsed > 90 {
                return Err(BnsError::InvestigationDelay {
                    offence: format!("{:?}", offence),
                    days_elapsed,
                });
            }
        }
    }

    Ok(())
}

/// Validate arrest procedure
pub fn validate_arrest_procedure(
    offence: &Offence,
    arrest_date: Option<NaiveDate>,
    appearance_notice_given: bool,
    produced_before_magistrate_hours: Option<u32>,
) -> BnsResult<()> {
    if arrest_date.is_none() {
        return Ok(()); // No arrest made
    }

    // For offences punishable < 7 years, Section 41A notice required
    if !offence.is_cognizable() && !appearance_notice_given {
        return Err(BnsError::NoAppearanceNotice {
            offence: format!("{:?}", offence),
        });
    }

    // Check if produced before magistrate within 24 hours
    if let Some(hours) = produced_before_magistrate_hours
        && hours > 24
    {
        return Err(BnsError::CustodyViolation {
            hours_elapsed: hours,
        });
    }

    Ok(())
}

/// Validate bail application
pub fn validate_bail_status(offence: &Offence, bail_status: &BailStatus) -> BnsResult<()> {
    // If offence is bailable, bail should be granted
    if offence.is_bailable() && matches!(bail_status, BailStatus::InCustody) {
        return Err(BnsError::BailDenied {
            offence: format!("{:?}", offence),
        });
    }

    Ok(())
}

/// Validate police remand
pub fn validate_police_remand(total_remand_days: u32) -> BnsResult<()> {
    // Maximum police custody remand is 15 days
    if total_remand_days > 15 {
        return Err(BnsError::RemandExceeded {
            days: total_remand_days,
        });
    }

    Ok(())
}

/// Validate sentencing
pub fn validate_sentencing(
    offence: &Offence,
    awarded_years: Option<u32>,
    awarded_fine: Option<f64>,
) -> BnsResult<()> {
    let punishment = get_punishment_for_offence(offence);

    // Check if sentence exceeds maximum
    if let (Some(max), Some(awarded)) = (punishment.max_years, awarded_years)
        && awarded > max
    {
        return Err(BnsError::ExcessiveSentence {
            offence: format!("{:?}", offence),
            max_years: max,
            awarded_years: awarded,
        });
    }

    // Check if minimum sentence is met
    if let (Some(min), Some(awarded)) = (punishment.min_years, awarded_years)
        && awarded < min
    {
        return Err(BnsError::MinimumNotMet {
            offence: format!("{:?}", offence),
            min_years: min,
            awarded_years: awarded,
        });
    }

    // Validate fine if specified minimum
    if let (Some(min_fine), Some(awarded)) = (punishment.fine, awarded_fine)
        && matches!(punishment.fine_type, FineType::Minimum)
        && awarded < min_fine
    {
        return Err(BnsError::ValidationError {
            message: format!("Fine below minimum {} for offence {:?}", min_fine, offence),
        });
    }

    Ok(())
}

/// Get punishment for an offence
pub fn get_punishment_for_offence(offence: &Offence) -> Punishment {
    match offence {
        Offence::Murder => Punishment::for_murder(),
        Offence::Theft => Punishment::for_theft(),
        Offence::Cheating | Offence::Fraud => Punishment::for_cheating(),
        Offence::OrganizedCrime => Punishment::for_organized_crime(),
        Offence::Rape => Punishment {
            imprisonment: Some(ImprisonmentType::Rigorous),
            min_years: Some(10),
            max_years: None,
            life_imprisonment: true,
            death_penalty: false,
            fine: None,
            fine_type: FineType::Discretionary,
            community_service: None,
        },
        Offence::GangRape => Punishment {
            imprisonment: Some(ImprisonmentType::Rigorous),
            min_years: Some(20),
            max_years: None,
            life_imprisonment: true,
            death_penalty: true,
            fine: None,
            fine_type: FineType::Discretionary,
            community_service: None,
        },
        Offence::Robbery => Punishment {
            imprisonment: Some(ImprisonmentType::Rigorous),
            min_years: None,
            max_years: Some(10),
            life_imprisonment: false,
            death_penalty: false,
            fine: None,
            fine_type: FineType::Discretionary,
            community_service: None,
        },
        Offence::Dacoity => Punishment {
            imprisonment: Some(ImprisonmentType::Rigorous),
            min_years: None,
            max_years: None,
            life_imprisonment: true,
            death_penalty: false,
            fine: None,
            fine_type: FineType::Discretionary,
            community_service: None,
        },
        Offence::TerroristAct => Punishment {
            imprisonment: Some(ImprisonmentType::Rigorous),
            min_years: Some(5),
            max_years: None,
            life_imprisonment: true,
            death_penalty: true,
            fine: Some(10_000_000.0),
            fine_type: FineType::Minimum,
            community_service: None,
        },
        Offence::Defamation => Punishment {
            imprisonment: Some(ImprisonmentType::Simple),
            min_years: None,
            max_years: Some(2),
            life_imprisonment: false,
            death_penalty: false,
            fine: None,
            fine_type: FineType::Discretionary,
            community_service: Some(100),
        },
        Offence::CriminalTrespass => Punishment {
            imprisonment: Some(ImprisonmentType::Either),
            min_years: None,
            max_years: Some(3),
            life_imprisonment: false,
            death_penalty: false,
            fine: None,
            fine_type: FineType::Discretionary,
            community_service: None,
        },
        Offence::HitAndRun => Punishment {
            imprisonment: Some(ImprisonmentType::Rigorous),
            min_years: None,
            max_years: Some(10),
            life_imprisonment: false,
            death_penalty: false,
            fine: Some(700_000.0),
            fine_type: FineType::Minimum,
            community_service: None,
        },
        _ => Punishment {
            imprisonment: Some(ImprisonmentType::Either),
            min_years: None,
            max_years: Some(3),
            life_imprisonment: false,
            death_penalty: false,
            fine: None,
            fine_type: FineType::Discretionary,
            community_service: None,
        },
    }
}

/// Validate trial timeline (right to speedy trial)
pub fn validate_trial_timeline(fir_date: NaiveDate, current_status: &CaseStatus) -> BnsResult<()> {
    let today = Utc::now().date_naive();
    let years_pending = (today - fir_date).num_days() as f64 / 365.0;

    // Different thresholds for different stages
    match current_status {
        CaseStatus::UnderInvestigation if years_pending > 1.0 => {
            return Err(BnsError::TrialDelayed { years_pending });
        }
        CaseStatus::PendingTrial | CaseStatus::UnderTrial if years_pending > 3.0 => {
            return Err(BnsError::TrialDelayed { years_pending });
        }
        _ => {}
    }

    Ok(())
}

/// Validate plea bargaining eligibility
pub fn validate_plea_bargaining(
    offence: &Offence,
    accused_record: &Accused,
) -> BnsResult<PleaBargaining> {
    let eligibility = PleaBargaining::check_eligibility(offence);

    // Additional checks
    if accused_record.previous_convictions > 0 {
        return Ok(PleaBargaining {
            offence: *offence,
            eligible: false,
            max_reduction: None,
            ineligibility_reason: Some("Previous convictions exist".to_string()),
        });
    }

    Ok(eligibility)
}

/// Validate juvenile case handling
pub fn validate_juvenile_handling(accused: &Accused, offence: &Offence) -> BnsResult<()> {
    if let Some(age) = accused.age
        && age < 18
    {
        // Should be handled by Juvenile Justice Board
        // Check if offence is heinous (punishable with 7+ years)
        let punishment = get_punishment_for_offence(offence);
        if punishment.max_years.is_some_and(|y| y >= 7)
            || punishment.life_imprisonment
            || punishment.death_penalty
        {
            // Heinous offence - check if referred to JJB
            // For now, just validate age threshold
            return Err(BnsError::JuvenileNotReferred { age });
        }
    }

    Ok(())
}

/// Calculate statutory bail eligibility
pub fn calculate_statutory_bail_eligibility(
    fir_date: NaiveDate,
    offence: &Offence,
    chargesheet_filed: bool,
) -> bool {
    if chargesheet_filed {
        return false; // No statutory bail after chargesheet
    }

    let today = Utc::now().date_naive();
    let days_in_custody = (today - fir_date).num_days() as u32;

    // Statutory bail periods
    let max_days = if offence.is_bailable() {
        0 // Bailable offences - bail should be granted anyway
    } else {
        let punishment = get_punishment_for_offence(offence);
        if punishment.life_imprisonment || punishment.death_penalty {
            90 // 90 days for offences with life/death
        } else {
            60 // 60 days for other non-bailable offences
        }
    };

    days_in_custody >= max_days
}

/// Get limitation period for an offence
pub fn get_limitation_period(offence: &Offence) -> LimitationPeriod {
    let punishment = get_punishment_for_offence(offence);

    if punishment.death_penalty || punishment.life_imprisonment {
        LimitationPeriod::NoLimit
    } else if let Some(max_years) = punishment.max_years {
        if max_years > 3 {
            LimitationPeriod::ThreeYears
        } else if max_years > 1 {
            LimitationPeriod::OneYear
        } else {
            LimitationPeriod::SixMonths
        }
    } else {
        LimitationPeriod::ThreeYears
    }
}

/// Comprehensive criminal case compliance check
pub fn validate_criminal_compliance(case: &CriminalCase) -> CriminalComplianceReport {
    let mut report = CriminalComplianceReport {
        compliant: true,
        investigation_compliant: true,
        procedural_compliant: true,
        victim_rights_compliant: true,
        violations: Vec::new(),
        warnings: Vec::new(),
        recommendations: Vec::new(),
    };

    // FIR registration check
    if let Err(e) = validate_fir_registration(
        case.offence_date,
        case.fir_date,
        case.offence.is_cognizable(),
    ) {
        report.investigation_compliant = false;
        report.compliant = false;
        report.violations.push(e);
    }

    // Investigation timeline
    if let Some(fir_date) = case.fir_date {
        let accused_in_custody = case
            .accused
            .iter()
            .any(|a| matches!(a.bail_status, BailStatus::InCustody));

        if let Err(e) = validate_investigation_timeline(
            fir_date,
            None, // Chargesheet date would come from case status
            &case.offence,
            accused_in_custody,
        ) {
            report.investigation_compliant = false;
            report.compliant = false;
            report.violations.push(e);
        }

        // Trial timeline
        if let Err(e) = validate_trial_timeline(fir_date, &case.status) {
            report.procedural_compliant = false;
            report.compliant = false;
            report.violations.push(e);
        }
    }

    // Bail status check
    for accused in &case.accused {
        if let Err(e) = validate_bail_status(&case.offence, &accused.bail_status) {
            report.procedural_compliant = false;
            report.compliant = false;
            report.violations.push(e);
        }

        // Juvenile check
        if let Err(e) = validate_juvenile_handling(accused, &case.offence) {
            report.procedural_compliant = false;
            report.compliant = false;
            report.violations.push(e);
        }
    }

    // Add recommendations
    if case.offence.is_compoundable() {
        report
            .recommendations
            .push("Consider compounding if parties agree".to_string());
    }

    let plea = PleaBargaining::check_eligibility(&case.offence);
    if plea.eligible {
        report
            .recommendations
            .push("Plea bargaining may be available".to_string());
    }

    report
}

/// Get applicable court for offence
pub fn get_applicable_court(offence: &Offence) -> Court {
    let punishment = get_punishment_for_offence(offence);

    if punishment.death_penalty || punishment.life_imprisonment {
        Court::SessionsCourt // Death/Life sentence only by Sessions Court
    } else if let Some(max_years) = punishment.max_years {
        if max_years > 7 {
            Court::SessionsCourt
        } else if max_years > 3 {
            Court::ChiefJudicialMagistrate
        } else {
            Court::JudicialMagistrateFirstClass
        }
    } else {
        Court::SessionsCourt
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fir_registration() {
        let offence_date = NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date");
        let fir_date = NaiveDate::from_ymd_opt(2024, 1, 2).expect("valid date");

        let result = validate_fir_registration(offence_date, Some(fir_date), true);
        assert!(result.is_ok());

        // No FIR registered
        let result = validate_fir_registration(offence_date, None, true);
        assert!(result.is_err());
    }

    #[test]
    fn test_bail_status_validation() {
        // Bailable offence should get bail
        let result = validate_bail_status(&Offence::Theft, &BailStatus::InCustody);
        assert!(result.is_err());

        // Non-bailable offence in custody is ok
        let result = validate_bail_status(&Offence::Murder, &BailStatus::InCustody);
        assert!(result.is_ok());
    }

    #[test]
    fn test_police_remand() {
        assert!(validate_police_remand(10).is_ok());
        assert!(validate_police_remand(20).is_err());
    }

    #[test]
    fn test_punishment_lookup() {
        let murder_punishment = get_punishment_for_offence(&Offence::Murder);
        assert!(murder_punishment.death_penalty);
        assert!(murder_punishment.life_imprisonment);

        let theft_punishment = get_punishment_for_offence(&Offence::Theft);
        assert!(!theft_punishment.death_penalty);
        assert_eq!(theft_punishment.max_years, Some(3));
    }

    #[test]
    fn test_limitation_period() {
        let murder_limit = get_limitation_period(&Offence::Murder);
        assert_eq!(murder_limit, LimitationPeriod::NoLimit);

        let theft_limit = get_limitation_period(&Offence::Theft);
        assert_eq!(theft_limit, LimitationPeriod::OneYear);
    }

    #[test]
    fn test_applicable_court() {
        let murder_court = get_applicable_court(&Offence::Murder);
        assert_eq!(murder_court, Court::SessionsCourt);

        let theft_court = get_applicable_court(&Offence::Theft);
        assert_eq!(theft_court, Court::JudicialMagistrateFirstClass);
    }

    #[test]
    fn test_statutory_bail_eligibility() {
        let fir_date = NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date");

        // Non-bailable offence, chargesheet not filed, would need to check days
        let _eligible = calculate_statutory_bail_eligibility(fir_date, &Offence::Murder, false);
        // Function executed without panic - result depends on current date

        // After chargesheet, no statutory bail
        let eligible = calculate_statutory_bail_eligibility(fir_date, &Offence::Murder, true);
        assert!(!eligible);
    }

    #[test]
    fn test_plea_bargaining_validation() {
        let accused = Accused {
            name: "Test".to_string(),
            age: Some(30),
            address: None,
            arrested: false,
            arrest_date: None,
            bail_status: BailStatus::NotArrested,
            previous_convictions: 0,
        };

        let result = validate_plea_bargaining(&Offence::Theft, &accused);
        assert!(result.is_ok());
        let plea = result.expect("Should be ok");
        assert!(plea.eligible);

        // With previous convictions
        let accused_with_record = Accused {
            previous_convictions: 1,
            ..accused
        };
        let result = validate_plea_bargaining(&Offence::Theft, &accused_with_record);
        let plea = result.expect("Should be ok");
        assert!(!plea.eligible);
    }

    #[test]
    fn test_juvenile_handling() {
        let minor = Accused {
            name: "Minor".to_string(),
            age: Some(16),
            address: None,
            arrested: false,
            arrest_date: None,
            bail_status: BailStatus::NotArrested,
            previous_convictions: 0,
        };

        // Heinous offence with minor
        let result = validate_juvenile_handling(&minor, &Offence::Murder);
        assert!(result.is_err());

        // Adult accused
        let adult = Accused {
            age: Some(25),
            ..minor
        };
        let result = validate_juvenile_handling(&adult, &Offence::Murder);
        assert!(result.is_ok());
    }

    #[test]
    fn test_sentencing_validation() {
        // Valid sentence
        let result = validate_sentencing(&Offence::Theft, Some(2), None);
        assert!(result.is_ok());

        // Excessive sentence
        let result = validate_sentencing(&Offence::Theft, Some(10), None);
        assert!(result.is_err());
    }
}
