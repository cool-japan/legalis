//! Labour Codes 2020 Validation
//!
//! Validation logic for India's consolidated Labour Codes

use super::error::{LabourCodeError, LabourCodeResult, LabourComplianceReport};
use super::types::*;
use chrono::NaiveDate;

// ====================
// Code on Wages Validations
// ====================

/// Validate minimum wage compliance
pub fn validate_minimum_wage(
    actual_wage: f64,
    area: GeographicalArea,
    skill_level: SkillLevel,
    floor_wage: f64,
) -> LabourCodeResult<()> {
    // Apply area and skill level multipliers
    let area_multiplier = match area {
        GeographicalArea::Metropolitan => 1.2,
        GeographicalArea::Urban => 1.1,
        GeographicalArea::Rural => 1.0,
    };

    let minimum_wage = floor_wage * area_multiplier * skill_level.wage_multiplier();

    if actual_wage < minimum_wage {
        return Err(LabourCodeError::BelowMinimumWage {
            actual: actual_wage,
            minimum: minimum_wage,
            shortfall: minimum_wage - actual_wage,
        });
    }

    Ok(())
}

/// Validate wage payment timing
pub fn validate_wage_payment(
    payment: &WagePayment,
    employee_count: u32,
    period_end: NaiveDate,
) -> LabourCodeResult<()> {
    let deadline_days = payment.period.payment_deadline(employee_count);
    let expected_deadline = period_end + chrono::Duration::days(deadline_days as i64);

    if payment.date > expected_deadline {
        let days_late = (payment.date - expected_deadline).num_days() as u32;
        return Err(LabourCodeError::PaymentDelayed { days_late });
    }

    Ok(())
}

/// Validate wage deductions
pub fn validate_deductions(deductions: &[WageDeduction], gross_wage: f64) -> LabourCodeResult<()> {
    let total_deduction: f64 = deductions.iter().map(|d| d.amount).sum();

    // Total deductions cannot exceed 50% of wages
    let max_deduction = gross_wage * 0.5;
    if total_deduction > max_deduction {
        return Err(LabourCodeError::ExcessDeduction {
            total_deduction,
            max_allowed: max_deduction,
        });
    }

    // Check individual deduction limits
    for deduction in deductions {
        if let Some(max_pct) = deduction.deduction_type.max_percentage() {
            let max_for_type = gross_wage * max_pct / 100.0;
            if deduction.amount > max_for_type {
                return Err(LabourCodeError::UnauthorizedDeduction {
                    deduction_type: format!("{:?}", deduction.deduction_type),
                    amount: deduction.amount - max_for_type,
                });
            }
        }
    }

    Ok(())
}

/// Validate bonus payment
pub fn validate_bonus(
    days_worked: u32,
    salary: f64,
    bonus_paid: f64,
    allocable_surplus: f64,
) -> LabourCodeResult<()> {
    // Check eligibility
    if !Bonus::is_eligible(days_worked) {
        return Ok(()); // Not eligible, no obligation
    }

    // Calculate salary for bonus (capped at Rs. 21,000)
    let capped_salary = salary.min(Bonus::wage_ceiling());

    // Minimum bonus is 8.33%
    let minimum_bonus = Bonus::calculate_minimum(capped_salary);

    if bonus_paid < minimum_bonus && allocable_surplus > 0.0 {
        return Err(LabourCodeError::BonusNotPaid {
            employee: String::new(),
            amount_due: minimum_bonus - bonus_paid,
        });
    }

    Ok(())
}

/// Calculate minimum wage for area and skill
pub fn calculate_minimum_wage(
    floor_wage: f64,
    area: GeographicalArea,
    skill_level: SkillLevel,
) -> f64 {
    let area_multiplier = match area {
        GeographicalArea::Metropolitan => 1.2,
        GeographicalArea::Urban => 1.1,
        GeographicalArea::Rural => 1.0,
    };

    floor_wage * area_multiplier * skill_level.wage_multiplier()
}

// ==================================
// Social Security Validations
// ==================================

/// Validate EPF compliance
pub fn validate_epf_compliance(
    basic_wage: f64,
    employee_contribution: f64,
    employer_contribution: f64,
    deposited: bool,
) -> LabourCodeResult<()> {
    let expected = EpfContribution::calculate(basic_wage);

    // Check if contributions are correct
    let employee_diff = (employee_contribution - expected.employee).abs();
    let employer_diff = (employer_contribution - expected.employer).abs();

    if employee_diff > 1.0 || employer_diff > 1.0 {
        return Err(LabourCodeError::ValidationError {
            message: format!(
                "EPF contribution mismatch: expected employee={}, employer={}",
                expected.employee, expected.employer
            ),
        });
    }

    if !deposited {
        return Err(LabourCodeError::EpfNotDeposited {
            amount: employee_contribution + employer_contribution,
            months: 1,
        });
    }

    Ok(())
}

/// Validate ESI compliance
pub fn validate_esi_compliance(
    gross_wages: f64,
    employee_contribution: f64,
    employer_contribution: f64,
    deposited: bool,
) -> LabourCodeResult<()> {
    // Check if wages are below ESI ceiling
    if gross_wages > EsiContribution::wage_ceiling() {
        return Ok(()); // Not applicable
    }

    let expected = EsiContribution::calculate(gross_wages);

    let employee_diff = (employee_contribution - expected.employee).abs();
    let employer_diff = (employer_contribution - expected.employer).abs();

    if employee_diff > 1.0 || employer_diff > 1.0 {
        return Err(LabourCodeError::ValidationError {
            message: format!(
                "ESI contribution mismatch: expected employee={}, employer={}",
                expected.employee, expected.employer
            ),
        });
    }

    if !deposited {
        return Err(LabourCodeError::EsiNotDeposited {
            amount: employee_contribution + employer_contribution,
            months: 1,
        });
    }

    Ok(())
}

/// Validate gratuity payment
pub fn validate_gratuity(
    years_of_service: f64,
    last_salary: f64,
    gratuity_paid: f64,
    payment_date: Option<NaiveDate>,
    termination_date: NaiveDate,
) -> LabourCodeResult<()> {
    // Check eligibility
    if !Gratuity::is_eligible(years_of_service) {
        return Ok(()); // Not eligible
    }

    let expected = Gratuity::calculate(years_of_service, last_salary);

    // Check amount
    if gratuity_paid < expected.amount - 1.0 {
        return Err(LabourCodeError::GratuityNotPaid {
            amount: expected.amount,
            service_years: years_of_service,
        });
    }

    // Check timing (within 30 days)
    if let Some(paid_date) = payment_date
        && paid_date > termination_date + chrono::Duration::days(30)
    {
        return Err(LabourCodeError::GratuityNotPaid {
            amount: expected.amount,
            service_years: years_of_service,
        });
    }

    Ok(())
}

/// Calculate gratuity amount
pub fn calculate_gratuity(years: f64, last_salary: f64) -> f64 {
    Gratuity::calculate(years, last_salary).amount
}

/// Validate maternity benefit
pub fn validate_maternity_benefit(
    benefit_type: MaternityBenefitType,
    days_worked_in_12_months: u32,
    benefit_provided: bool,
    weeks_granted: u32,
) -> LabourCodeResult<()> {
    // Eligibility: 80 days worked in 12 months preceding expected delivery
    if days_worked_in_12_months < 80 {
        return Ok(()); // Not eligible
    }

    if !benefit_provided {
        return Err(LabourCodeError::MaternityBenefitDenied {
            reason: "Benefit not provided to eligible employee".to_string(),
        });
    }

    let entitled_weeks = benefit_type.leave_weeks();
    if weeks_granted < entitled_weeks {
        return Err(LabourCodeError::MaternityBenefitDenied {
            reason: format!(
                "Entitled to {} weeks but only {} weeks granted",
                entitled_weeks, weeks_granted
            ),
        });
    }

    Ok(())
}

// ==================================
// Industrial Relations Validations
// ==================================

/// Validate strike/lockout notice
pub fn validate_strike_lockout_notice(
    action: &StrikeLockout,
    notice_given: bool,
    is_public_utility: bool,
) -> LabourCodeResult<()> {
    // Notice required for public utility services
    if is_public_utility && !notice_given {
        match action.action_type {
            IndustrialActionType::Strike => {
                return Err(LabourCodeError::StrikeNoticeNotGiven);
            }
            IndustrialActionType::Lockout => {
                return Err(LabourCodeError::LockoutNoticeNotGiven);
            }
            _ => {}
        }
    }

    // Check notice period (14 days)
    let notice_period = (action.start_date - action.notice_date).num_days();
    if notice_period < StrikeLockout::notice_period_days() as i64 {
        match action.action_type {
            IndustrialActionType::Strike => {
                return Err(LabourCodeError::StrikeNoticeNotGiven);
            }
            IndustrialActionType::Lockout => {
                return Err(LabourCodeError::LockoutNoticeNotGiven);
            }
            _ => {}
        }
    }

    Ok(())
}

/// Validate retrenchment procedure
pub fn validate_retrenchment(
    retrenchment: &Retrenchment,
    total_workers: u32,
    government_permission_obtained: bool,
) -> LabourCodeResult<()> {
    // Check minimum service
    if retrenchment.years_of_service < Retrenchment::minimum_service_years() {
        return Err(LabourCodeError::ValidationError {
            message: "Minimum 1 year service required for retrenchment provisions".to_string(),
        });
    }

    // Check if permission required (300+ workers)
    if total_workers >= 300 && !government_permission_obtained {
        return Err(LabourCodeError::RetrenchmentWithoutPermission {
            workers_affected: 1,
        });
    }

    // Check notice period
    if retrenchment.notice_period < Retrenchment::notice_period_days()
        && retrenchment.notice_pay.is_none()
    {
        return Err(LabourCodeError::ValidationError {
            message: "30 days notice or notice pay required".to_string(),
        });
    }

    // Check compensation calculation
    let expected_comp = Retrenchment::calculate_compensation(
        retrenchment.years_of_service,
        retrenchment.last_wages / 26.0,
    );
    if retrenchment.compensation < expected_comp - 1.0 {
        return Err(LabourCodeError::ValidationError {
            message: format!(
                "Compensation shortfall: expected {}, got {}",
                expected_comp, retrenchment.compensation
            ),
        });
    }

    Ok(())
}

/// Validate layoff procedure
pub fn validate_layoff(layoff: &Layoff, total_workers: u32) -> LabourCodeResult<()> {
    // Check if permission required (300+ workers)
    if total_workers >= Layoff::permission_threshold() && !layoff.permission_required {
        return Err(LabourCodeError::LayoffWithoutPermission {
            workers_affected: layoff.workers_affected,
        });
    }

    // Check compensation rate
    if layoff.compensation_rate < Layoff::compensation_percentage() {
        return Err(LabourCodeError::ValidationError {
            message: format!(
                "Layoff compensation should be at least {}%",
                Layoff::compensation_percentage()
            ),
        });
    }

    Ok(())
}

/// Validate standing orders
pub fn validate_standing_orders(
    orders: &StandingOrders,
    worker_count: u32,
    certified: bool,
) -> LabourCodeResult<()> {
    // Standing orders required for 300+ workers
    if worker_count >= 300 && !certified {
        return Err(LabourCodeError::StandingOrdersNotCertified);
    }

    // Check required elements
    if !orders.grievance_mechanism {
        return Err(LabourCodeError::GrievanceNotAddressed {
            grievance_id: "Standing orders missing grievance mechanism".to_string(),
        });
    }

    Ok(())
}

/// Validate trade union registration
pub fn validate_trade_union(union: &TradeUnion, total_workers: u32) -> LabourCodeResult<()> {
    // Check minimum members
    let min_members = TradeUnion::minimum_members(total_workers);
    if union.members < min_members {
        return Err(LabourCodeError::ValidationError {
            message: format!(
                "Minimum {} members required for registration, have {}",
                min_members, union.members
            ),
        });
    }

    if union.registration_number.is_empty() {
        return Err(LabourCodeError::TradeUnionNotRegistered);
    }

    Ok(())
}

// ==================================
// OSH Code Validations
// ==================================

/// Validate working hours
pub fn validate_working_hours(hours: &WorkingHours) -> LabourCodeResult<()> {
    if hours.daily > WorkingHours::max_daily() {
        return Err(LabourCodeError::WorkingHoursExceeded {
            actual: hours.daily,
            maximum: WorkingHours::max_daily(),
        });
    }

    if hours.weekly > WorkingHours::max_weekly() {
        return Err(LabourCodeError::WorkingHoursExceeded {
            actual: hours.weekly,
            maximum: WorkingHours::max_weekly(),
        });
    }

    if hours.spread_over > WorkingHours::max_spread_over() {
        return Err(LabourCodeError::ValidationError {
            message: format!(
                "Spread over {} hours exceeds maximum {} hours",
                hours.spread_over,
                WorkingHours::max_spread_over()
            ),
        });
    }

    if hours.rest_interval_minutes < WorkingHours::min_rest_interval() {
        return Err(LabourCodeError::ValidationError {
            message: format!(
                "Rest interval {} minutes below minimum {} minutes",
                hours.rest_interval_minutes,
                WorkingHours::min_rest_interval()
            ),
        });
    }

    Ok(())
}

/// Validate overtime payment
pub fn validate_overtime(
    overtime_hours: f64,
    ordinary_rate: f64,
    amount_paid: f64,
) -> LabourCodeResult<()> {
    let expected = Overtime::calculate(overtime_hours, ordinary_rate);

    if amount_paid < expected.amount - 1.0 {
        return Err(LabourCodeError::OvertimeNotPaid {
            hours: overtime_hours,
            amount_due: expected.amount - amount_paid,
        });
    }

    Ok(())
}

/// Validate leave provisions
pub fn validate_leave(
    days_worked: u32,
    leave_granted: u32,
    leave_type: &str,
) -> LabourCodeResult<()> {
    let entitled = LeaveProvisions::calculate_earned_leave(days_worked);

    if leave_type == "annual" && leave_granted < entitled {
        return Err(LabourCodeError::AnnualLeaveNotGranted {
            days_due: entitled - leave_granted,
        });
    }

    Ok(())
}

/// Validate safety committee
pub fn validate_safety_committee(
    worker_count: u32,
    is_hazardous: bool,
    committee: Option<&SafetyCommittee>,
) -> LabourCodeResult<()> {
    if SafetyCommittee::is_required(worker_count, is_hazardous) {
        match committee {
            None => {
                return Err(LabourCodeError::SafetyCommitteeNotConstituted { worker_count });
            }
            Some(c) => {
                if !c.has_safety_officer && worker_count >= 500 {
                    return Err(LabourCodeError::SafetyOfficerNotAppointed);
                }
            }
        }
    }

    Ok(())
}

/// Validate contract labour
pub fn validate_contract_labour(contract: &ContractLabour) -> LabourCodeResult<()> {
    // Check for core activity prohibition
    if ContractLabour::is_core_prohibited(contract.work_nature) {
        return Err(LabourCodeError::ContractLabourInCore);
    }

    // Check registration
    if contract.worker_count >= ContractLabour::registration_threshold()
        && contract.registration_number.is_none()
    {
        return Err(LabourCodeError::LicenseNotObtained {
            license_type: "Principal employer registration".to_string(),
        });
    }

    // Check license
    if contract.worker_count >= ContractLabour::license_threshold()
        && contract.license_number.is_none()
    {
        return Err(LabourCodeError::LicenseNotObtained {
            license_type: "Contractor license".to_string(),
        });
    }

    Ok(())
}

/// Validate inter-state migrant worker provisions
pub fn validate_migrant_worker(worker: &InterStateMigrantWorker) -> LabourCodeResult<()> {
    if !worker.passbook_issued {
        return Err(LabourCodeError::MigrantWorkerFacilitiesNotProvided);
    }

    if !worker.journey_allowance_paid {
        return Err(LabourCodeError::MigrantWorkerFacilitiesNotProvided);
    }

    Ok(())
}

/// Validate establishment registration
pub fn validate_establishment_registration(
    establishment_type: EstablishmentType,
    worker_count: u32,
    registered: bool,
) -> LabourCodeResult<()> {
    if establishment_type.requires_registration()
        && worker_count >= establishment_type.applicability_threshold()
        && !registered
    {
        return Err(LabourCodeError::EstablishmentNotRegistered);
    }

    Ok(())
}

// ==================================
// Comprehensive Validation
// ==================================

/// Input parameters for labour compliance check
#[derive(Debug, Clone)]
pub struct LabourComplianceCheck<'a> {
    /// Number of workers
    pub worker_count: u32,
    /// Type of establishment
    pub establishment_type: EstablishmentType,
    /// Whether establishment handles hazardous materials
    pub is_hazardous: bool,
    /// Working hours configuration
    pub working_hours: &'a WorkingHours,
    /// Whether EPF contributions are compliant
    pub epf_compliant: bool,
    /// Whether ESI contributions are compliant
    pub esi_compliant: bool,
    /// Safety committee if constituted
    pub safety_committee: Option<&'a SafetyCommittee>,
    /// Whether standing orders are certified
    pub standing_orders_certified: bool,
}

/// Comprehensive labour compliance check
pub fn validate_labour_compliance(check: &LabourComplianceCheck<'_>) -> LabourComplianceReport {
    let worker_count = check.worker_count;
    let is_hazardous = check.is_hazardous;
    let working_hours = check.working_hours;
    let epf_compliant = check.epf_compliant;
    let esi_compliant = check.esi_compliant;
    let safety_committee = check.safety_committee;
    let standing_orders_certified = check.standing_orders_certified;
    let mut report = LabourComplianceReport {
        compliant: true,
        wages_compliant: true,
        social_security_compliant: true,
        ir_compliant: true,
        osh_compliant: true,
        violations: Vec::new(),
        warnings: Vec::new(),
        recommendations: Vec::new(),
        penalty_exposure: 0.0,
    };

    // OSH Code: Working hours
    if let Err(e) = validate_working_hours(working_hours) {
        report.osh_compliant = false;
        report.compliant = false;
        report.penalty_exposure += 100_000.0;
        report.violations.push(e);
    }

    // Social Security: EPF
    if !epf_compliant {
        report.social_security_compliant = false;
        report.compliant = false;
        report.violations.push(LabourCodeError::EpfNotDeposited {
            amount: 0.0,
            months: 1,
        });
        report.penalty_exposure += 200_000.0;
    }

    // Social Security: ESI
    if !esi_compliant && worker_count >= 10 {
        report.social_security_compliant = false;
        report.compliant = false;
        report.violations.push(LabourCodeError::EsiNotDeposited {
            amount: 0.0,
            months: 1,
        });
        report.penalty_exposure += 200_000.0;
    }

    // OSH Code: Safety committee
    if let Err(e) = validate_safety_committee(worker_count, is_hazardous, safety_committee) {
        report.osh_compliant = false;
        report.compliant = false;
        report.penalty_exposure += 300_000.0;
        report.violations.push(e);
    }

    // IR Code: Standing orders
    if worker_count >= 300 && !standing_orders_certified {
        report.ir_compliant = false;
        report.compliant = false;
        report
            .violations
            .push(LabourCodeError::StandingOrdersNotCertified);
        report.penalty_exposure += 100_000.0;
    }

    // Add recommendations
    if worker_count >= 100 {
        report
            .recommendations
            .push("Consider appointing a welfare officer".to_string());
    }

    if is_hazardous {
        report
            .recommendations
            .push("Conduct regular safety audits and training".to_string());
    }

    if worker_count >= 300 {
        report
            .recommendations
            .push("Establish internal dispute resolution mechanism".to_string());
    }

    report
}

/// Get compliance checklist
pub fn get_compliance_checklist(
    establishment_type: EstablishmentType,
    worker_count: u32,
) -> Vec<ComplianceChecklistItem> {
    let mut checklist = vec![
        ComplianceChecklistItem {
            code: "Wages".to_string(),
            requirement: "Pay minimum wages".to_string(),
            section: "Section 9".to_string(),
            applicable: true,
        },
        ComplianceChecklistItem {
            code: "Wages".to_string(),
            requirement: "Timely payment of wages".to_string(),
            section: "Section 17".to_string(),
            applicable: true,
        },
        ComplianceChecklistItem {
            code: "Social Security".to_string(),
            requirement: "EPF registration and contribution".to_string(),
            section: "Section 6".to_string(),
            applicable: worker_count >= 20,
        },
        ComplianceChecklistItem {
            code: "Social Security".to_string(),
            requirement: "ESI registration and contribution".to_string(),
            section: "Section 28".to_string(),
            applicable: worker_count >= 10,
        },
        ComplianceChecklistItem {
            code: "IR Code".to_string(),
            requirement: "Standing orders certification".to_string(),
            section: "Section 30".to_string(),
            applicable: worker_count >= 300,
        },
        ComplianceChecklistItem {
            code: "OSH Code".to_string(),
            requirement: "Establishment registration".to_string(),
            section: "Section 3".to_string(),
            applicable: worker_count >= establishment_type.applicability_threshold(),
        },
        ComplianceChecklistItem {
            code: "OSH Code".to_string(),
            requirement: "Safety committee constitution".to_string(),
            section: "Section 22".to_string(),
            applicable: worker_count >= 250,
        },
        ComplianceChecklistItem {
            code: "OSH Code".to_string(),
            requirement: "Maintain registers and records".to_string(),
            section: "Section 35".to_string(),
            applicable: true,
        },
    ];

    // Filter by applicability
    checklist.retain(|item| item.applicable);
    checklist
}

/// Compliance checklist item
#[derive(Debug, Clone)]
pub struct ComplianceChecklistItem {
    /// Labour code
    pub code: String,
    /// Requirement description
    pub requirement: String,
    /// Section reference
    pub section: String,
    /// Is applicable
    pub applicable: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_wage_validation() {
        // Below minimum
        let result = validate_minimum_wage(
            8000.0,
            GeographicalArea::Rural,
            SkillLevel::Unskilled,
            10000.0,
        );
        assert!(result.is_err());

        // Above minimum
        let result = validate_minimum_wage(
            12000.0,
            GeographicalArea::Rural,
            SkillLevel::Unskilled,
            10000.0,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_minimum_wage_calculation() {
        let wage =
            calculate_minimum_wage(10000.0, GeographicalArea::Metropolitan, SkillLevel::Skilled);
        // 10000 * 1.2 * 1.30 = 15600
        assert_eq!(wage, 15600.0);
    }

    #[test]
    fn test_deduction_validation() {
        let deductions = vec![
            WageDeduction {
                deduction_type: DeductionType::ProvidentFund,
                amount: 1800.0,
                section: "Section 18".to_string(),
            },
            WageDeduction {
                deduction_type: DeductionType::TaxAtSource,
                amount: 1000.0,
                section: "Section 18".to_string(),
            },
        ];

        let result = validate_deductions(&deductions, 15000.0);
        assert!(result.is_ok());

        // Excess deduction
        let excess_deductions = vec![WageDeduction {
            deduction_type: DeductionType::Advances,
            amount: 10000.0,
            section: "Section 18".to_string(),
        }];

        let result = validate_deductions(&excess_deductions, 15000.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_bonus_validation() {
        // Eligible and paid
        let result = validate_bonus(200, 20000.0, 2000.0, 100000.0);
        assert!(result.is_ok());

        // Not eligible
        let result = validate_bonus(20, 20000.0, 0.0, 100000.0);
        assert!(result.is_ok()); // No obligation
    }

    #[test]
    fn test_epf_validation() {
        let result = validate_epf_compliance(15000.0, 1800.0, 1800.0, true);
        assert!(result.is_ok());

        let result = validate_epf_compliance(15000.0, 1800.0, 1800.0, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_gratuity_validation() {
        let termination = NaiveDate::from_ymd_opt(2024, 6, 1).expect("valid date");
        let payment = NaiveDate::from_ymd_opt(2024, 6, 15).expect("valid date");

        let result = validate_gratuity(10.0, 30000.0, 173077.0, Some(payment), termination);
        assert!(result.is_ok());

        // Not eligible (less than 5 years)
        let result = validate_gratuity(4.0, 30000.0, 0.0, None, termination);
        assert!(result.is_ok()); // No obligation
    }

    #[test]
    fn test_working_hours_validation() {
        let valid_hours = WorkingHours::default();
        assert!(validate_working_hours(&valid_hours).is_ok());

        let invalid_hours = WorkingHours {
            daily: 10.0,
            weekly: 60.0,
            spread_over: 12.0,
            rest_interval_minutes: 20,
            weekly_off: WeeklyOff::Sunday,
        };
        assert!(validate_working_hours(&invalid_hours).is_err());
    }

    #[test]
    fn test_overtime_validation() {
        // Properly paid
        let result = validate_overtime(2.0, 100.0, 400.0);
        assert!(result.is_ok());

        // Underpaid
        let result = validate_overtime(2.0, 100.0, 200.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_safety_committee_validation() {
        // Required but not constituted
        let result = validate_safety_committee(300, false, None);
        assert!(result.is_err());

        // Required and constituted
        let committee = SafetyCommittee {
            employer_reps: 2,
            worker_reps: 2,
            has_safety_officer: true,
            quarterly_meetings: 4,
        };
        let result = validate_safety_committee(300, false, Some(&committee));
        assert!(result.is_ok());
    }

    #[test]
    fn test_contract_labour_validation() {
        let contract = ContractLabour {
            principal_employer: "ABC Corp".to_string(),
            contractor: "XYZ Services".to_string(),
            worker_count: 30,
            registration_number: Some("REG001".to_string()),
            license_number: Some("LIC001".to_string()),
            work_nature: ContractWorkNature::SupportServices,
        };
        assert!(validate_contract_labour(&contract).is_ok());

        // Core activity
        let core_contract = ContractLabour {
            work_nature: ContractWorkNature::Core,
            ..contract
        };
        assert!(validate_contract_labour(&core_contract).is_err());
    }

    #[test]
    fn test_comprehensive_compliance() {
        let working_hours = WorkingHours::default();
        let committee = SafetyCommittee {
            employer_reps: 2,
            worker_reps: 2,
            has_safety_officer: true,
            quarterly_meetings: 4,
        };

        let check = LabourComplianceCheck {
            worker_count: 500,
            establishment_type: EstablishmentType::Factory,
            is_hazardous: false,
            working_hours: &working_hours,
            epf_compliant: true,
            esi_compliant: true,
            safety_committee: Some(&committee),
            standing_orders_certified: true,
        };

        let report = validate_labour_compliance(&check);
        assert!(report.compliant);
    }

    #[test]
    fn test_compliance_checklist() {
        let checklist = get_compliance_checklist(EstablishmentType::Factory, 500);
        assert!(!checklist.is_empty());

        // Should include EPF (20+ workers)
        assert!(checklist.iter().any(|i| i.requirement.contains("EPF")));

        // Should include safety committee (250+ workers)
        assert!(checklist.iter().any(|i| i.requirement.contains("Safety")));
    }
}
