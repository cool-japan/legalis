//! Integration tests for Employment Act (Cap. 91 / Employment Act 1968):
//! contract validation, working hours (s. 38), overtime (s. 38(4)), termination
//! notice (s. 10/11), annual leave (s. 43) and EA coverage (Part IV s. 35).

use chrono::{Duration, Utc};
use legalis_sg::employment::*;

fn sample_contract() -> EmploymentContract {
    EmploymentContract {
        employee_name: "Jane Tan".to_string(),
        employer_name: "Tech Innovations Pte Ltd".to_string(),
        contract_type: ContractType::Indefinite,
        start_date: Utc::now() - Duration::days(400),
        end_date: None,
        basic_salary_cents: 350_000, // SGD 3,500
        allowances: vec![Allowance::new("Transport", 20_000, true)],
        working_hours: WorkingHours::standard(),
        leave_entitlement: LeaveEntitlement::new(1),
        cpf_applicable: true,
        covered_by_ea: true,
    }
}

#[test]
fn test_valid_contract() {
    let contract = sample_contract();
    let report = validate_employment_contract(&contract).expect("validation runs");
    assert!(report.is_valid, "errors: {:?}", report.errors);
    assert!(report.ea_covered);
    assert!(report.cpf_applicable);
}

#[test]
fn test_invalid_contract_dates() {
    let mut contract = sample_contract();
    contract.contract_type = ContractType::FixedTerm;
    contract.start_date = Utc::now();
    contract.end_date = Some(Utc::now() - Duration::days(10)); // end before start
    assert!(validate_employment_contract(&contract).is_err());
}

#[test]
fn test_working_hours_limits() {
    let ok = WorkingHours::standard();
    assert!(validate_working_hours(&ok).is_ok());

    let mut excessive = WorkingHours::standard();
    excessive.hours_per_week = 50.0; // > 44h/week (non-shift)
    assert!(validate_working_hours(&excessive).is_err());
}

#[test]
fn test_overtime_minimum_rate() {
    // 1.5x is the statutory minimum overtime rate (s. 38(4)).
    assert!(validate_overtime_payment(5.0, 1.5, 2_000).is_ok());
    assert!(validate_overtime_payment(5.0, 1.2, 2_000).is_err());
}

#[test]
fn test_termination_notice_periods() {
    assert_eq!(TerminationNotice::required_notice_days(10), 1); // < 26 weeks
    assert_eq!(TerminationNotice::required_notice_days(52), 7); // 26 wk - 2 yr
    assert_eq!(TerminationNotice::required_notice_days(150), 14); // 2 - 5 yr
    assert_eq!(TerminationNotice::required_notice_days(300), 28); // 5+ yr

    assert!(validate_termination_notice(300, 28).is_ok());
    assert!(validate_termination_notice(300, 14).is_err());
}

#[test]
fn test_annual_leave_progression() {
    assert_eq!(LeaveEntitlement::new(0).annual_leave_days, 7);
    assert_eq!(LeaveEntitlement::new(4).annual_leave_days, 11);
    assert_eq!(LeaveEntitlement::new(10).annual_leave_days, 14);

    let leave = validate_leave_entitlement(5, ContractType::Indefinite).expect("leave");
    assert_eq!(leave.annual_leave_days, 11);
    assert_eq!(leave.sick_leave_outpatient_days, 14);
    assert_eq!(leave.sick_leave_hospitalization_days, 60);
}

#[test]
fn test_ea_coverage_part_iv_thresholds() {
    // Workman earning at most SGD 4,500 → covered, including Part IV.
    assert_eq!(
        determine_ea_coverage(EmployeeCategory::Workman, 450_000, false),
        EaCoverage::GeneralAndPartIv
    );
    assert_eq!(
        determine_ea_coverage(EmployeeCategory::Workman, 500_000, false),
        EaCoverage::GeneralOnly
    );

    // Non-workman threshold is SGD 2,600.
    assert!(is_covered_by_part_iv(EmployeeCategory::NonWorkman, 260_000));
    assert!(!is_covered_by_part_iv(
        EmployeeCategory::NonWorkman,
        300_000
    ));

    // Managers/executives are never under Part IV.
    assert!(!is_covered_by_part_iv(
        EmployeeCategory::ManagerOrExecutive,
        100_000
    ));

    // Seafarers/domestic workers/public officers are excluded entirely.
    assert!(!determine_ea_coverage(EmployeeCategory::Workman, 100_000, true).is_covered());
}
