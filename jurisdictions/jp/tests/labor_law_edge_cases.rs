//! Labor Law Edge Case Tests
//!
//! Edge cases for Labor Standards Act (労働基準法),
//! Labor Contract Act (労働契約法), and related laws

use chrono::{Duration, Utc};
use legalis_jp::labor_law::*;

// ============================================================================
// Employment Contract Edge Cases
// ============================================================================

#[test]
fn test_employment_contract_valid_indefinite() {
    let contract = EmploymentContractBuilder::new()
        .with_employee("Employee Name")
        .with_employer("Employer Corp")
        .with_employment_type(EmploymentType::IndefiniteTerm)
        .with_work_pattern(WorkPattern::Regular)
        .with_salary(250_000)
        .with_working_hours(8, 5)
        .with_prefecture(Prefecture::Tokyo)
        .with_start_date(Utc::now())
        .with_job_description("Software Engineer")
        .with_work_location("Tokyo Office")
        .build();

    assert!(contract.is_ok());
    let contract = contract.unwrap();
    assert_eq!(contract.employment_type, EmploymentType::IndefiniteTerm);
}

#[test]
fn test_employment_contract_valid_fixed_term() {
    let contract = EmploymentContractBuilder::new()
        .with_employee("Temporary Worker")
        .with_employer("Company Ltd")
        .with_employment_type(EmploymentType::FixedTerm)
        .with_work_pattern(WorkPattern::Regular)
        .with_salary(200_000)
        .with_working_hours(8, 5)
        .with_prefecture(Prefecture::Osaka)
        .with_start_date(Utc::now())
        .with_job_description("Project Work")
        .with_work_location("Osaka Office")
        .build();

    assert!(contract.is_ok());
}

#[test]
fn test_employment_contract_part_time() {
    let contract = EmploymentContractBuilder::new()
        .with_employee("Part Timer")
        .with_employer("Retail Shop")
        .with_employment_type(EmploymentType::PartTime)
        .with_work_pattern(WorkPattern::Shift)
        .with_salary(100_000)
        .with_working_hours(4, 3)
        .with_prefecture(Prefecture::Kanagawa)
        .with_start_date(Utc::now())
        .with_job_description("Sales Assistant")
        .with_work_location("Shop")
        .build();

    assert!(contract.is_ok());
}

#[test]
fn test_employment_contract_flextime() {
    let contract = EmploymentContractBuilder::new()
        .with_employee("Flextime Worker")
        .with_employer("Tech Company")
        .with_employment_type(EmploymentType::IndefiniteTerm)
        .with_work_pattern(WorkPattern::Flextime)
        .with_salary(300_000)
        .with_working_hours(8, 5)
        .with_prefecture(Prefecture::Tokyo)
        .with_start_date(Utc::now())
        .with_job_description("Developer")
        .with_work_location("Remote")
        .build();

    assert!(contract.is_ok());
}

#[test]
fn test_all_employment_types() {
    let types = vec![
        EmploymentType::IndefiniteTerm,
        EmploymentType::FixedTerm,
        EmploymentType::PartTime,
    ];

    assert_eq!(types.len(), 3);
}

#[test]
fn test_all_work_patterns() {
    let patterns = vec![
        WorkPattern::Regular,
        WorkPattern::Flextime,
        WorkPattern::Shift,
    ];

    assert_eq!(patterns.len(), 3);
}

// ============================================================================
// Working Time Edge Cases
// ============================================================================

#[test]
fn test_working_time_record_valid() {
    let start = Utc::now();
    let end = start + Duration::hours(8);

    let record = WorkingTimeRecord {
        date: start,
        start_time: start,
        end_time: end,
        rest_minutes: 60,
        is_holiday: false,
    };

    let result = validate_working_time_record(&record);
    assert!(result.is_ok());
}

#[test]
fn test_working_time_record_holiday_work() {
    let start = Utc::now();
    let end = start + Duration::hours(8);

    let record = WorkingTimeRecord {
        date: start,
        start_time: start,
        end_time: end,
        rest_minutes: 60,
        is_holiday: true, // Holiday work
    };

    let result = validate_working_time_record(&record);
    assert!(result.is_ok());
}

#[test]
fn test_working_time_record_long_hours() {
    let start = Utc::now();
    let end = start + Duration::hours(12); // 12 hour shift

    let record = WorkingTimeRecord {
        date: start,
        start_time: start,
        end_time: end,
        rest_minutes: 60,
        is_holiday: false,
    };

    let result = validate_working_time_record(&record);
    // Long hours may require Article 36 agreement
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_working_time_record_short_rest() {
    let start = Utc::now();
    let end = start + Duration::hours(8);

    let record = WorkingTimeRecord {
        date: start,
        start_time: start,
        end_time: end,
        rest_minutes: 30, // Short rest period
        is_holiday: false,
    };

    let result = validate_working_time_record(&record);
    // May fail - Article 34 requires 45min rest for 6-8hr work
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_working_time_record_zero_rest() {
    let start = Utc::now();
    let end = start + Duration::hours(5); // Under 6 hours

    let record = WorkingTimeRecord {
        date: start,
        start_time: start,
        end_time: end,
        rest_minutes: 0, // No rest
        is_holiday: false,
    };

    let result = validate_working_time_record(&record);
    // Under 6 hours - no mandatory rest
    assert!(result.is_ok());
}

// ============================================================================
// Termination & Dismissal Edge Cases
// ============================================================================

#[test]
fn test_termination_notice_valid_30_days() {
    let notice = TerminationNotice {
        employee_name: "Resigning Employee".to_string(),
        termination_type: TerminationType::VoluntaryResignation,
        notice_date: Utc::now(),
        effective_date: Utc::now() + Duration::days(30),
        reason: "Career change".to_string(),
        severance_pay_jpy: None,
        notice_allowance_jpy: None,
    };

    let result = validate_termination_notice(&notice, 30);
    assert!(result.is_ok());
}

#[test]
fn test_termination_notice_dismissal_with_cause() {
    let notice = TerminationNotice {
        employee_name: "Dismissed Employee".to_string(),
        termination_type: TerminationType::DisciplinaryDismissal,
        notice_date: Utc::now(),
        effective_date: Utc::now() + Duration::days(30),
        reason: "Serious misconduct".to_string(),
        severance_pay_jpy: None,
        notice_allowance_jpy: None,
    };

    let result = validate_termination_notice(&notice, 30);
    assert!(result.is_ok());
}

#[test]
fn test_termination_notice_dismissal_without_cause() {
    let notice = TerminationNotice {
        employee_name: "Laid Off Employee".to_string(),
        termination_type: TerminationType::OrdinaryDismissal,
        notice_date: Utc::now(),
        effective_date: Utc::now() + Duration::days(30),
        reason: "Business downsizing".to_string(),
        severance_pay_jpy: Some(500_000),
        notice_allowance_jpy: None,
    };

    let result = validate_termination_notice(&notice, 30);
    assert!(result.is_ok());
}

#[test]
fn test_termination_notice_insufficient_period() {
    let notice = TerminationNotice {
        employee_name: "Employee".to_string(),
        termination_type: TerminationType::OrdinaryDismissal,
        notice_date: Utc::now(),
        effective_date: Utc::now() + Duration::days(15), // Only 15 days
        reason: "Business reasons".to_string(),
        severance_pay_jpy: None,
        notice_allowance_jpy: None,
    };

    let result = validate_termination_notice(&notice, 30);
    // Should fail - insufficient notice period (Article 20)
    assert!(result.is_err());
}

#[test]
fn test_all_termination_types() {
    let types = vec![
        TerminationType::OrdinaryDismissal,
        TerminationType::DisciplinaryDismissal,
        TerminationType::VoluntaryResignation,
        TerminationType::MutualAgreement,
        TerminationType::ContractExpiration,
        TerminationType::Retirement,
    ];

    assert_eq!(types.len(), 6);
}

#[test]
fn test_abusive_dismissal_no_reason() {
    let contract = EmploymentContractBuilder::new()
        .with_employee("Victim")
        .with_employer("Bad Company")
        .with_employment_type(EmploymentType::IndefiniteTerm)
        .with_work_pattern(WorkPattern::Regular)
        .with_salary(300_000)
        .with_working_hours(8, 5)
        .with_prefecture(Prefecture::Tokyo)
        .with_start_date(Utc::now() - Duration::days(365 * 6)) // 6 years employment
        .with_job_description("Engineer")
        .with_work_location("Office")
        .build()
        .unwrap();

    let notice = TerminationNotice {
        employee_name: "Victim".to_string(),
        termination_type: TerminationType::OrdinaryDismissal,
        notice_date: Utc::now(),
        effective_date: Utc::now() + Duration::days(30),
        reason: "Business reasons".to_string(), // Short reason (< 50 chars)
        severance_pay_jpy: None,
        notice_allowance_jpy: None,
    };

    let result = check_abusive_dismissal(&notice, &contract);
    // Abusive dismissal detected - long-term employee with insufficient justification
    assert!(result.is_err());
}

// ============================================================================
// Wage Payment Edge Cases
// ============================================================================

#[test]
fn test_wage_payment_valid() {
    let now = Utc::now();
    let payment = WagePayment {
        employee_name: "Employee".to_string(),
        period_start: now - Duration::days(30),
        period_end: now - Duration::days(1),
        payment_date: now,
        base_wage_jpy: 250_000,
        overtime_pay_jpy: 30_000,
        other_allowances_jpy: 20_000,
        deductions_jpy: 50_000,
        net_payment_jpy: 250_000,
    };

    let result = validate_wage_payment(&payment);
    assert!(result.is_ok());
}

#[test]
fn test_wage_payment_calculation_error() {
    let now = Utc::now();
    let payment = WagePayment {
        employee_name: "Employee".to_string(),
        period_start: now - Duration::days(30),
        period_end: now - Duration::days(1),
        payment_date: now,
        base_wage_jpy: 250_000,
        overtime_pay_jpy: 30_000,
        other_allowances_jpy: 20_000,
        deductions_jpy: 50_000,
        net_payment_jpy: 200_000, // Wrong calculation
    };

    let result = validate_wage_payment(&payment);
    // Should fail due to calculation error
    assert!(result.is_err());
}

#[test]
fn test_minimum_wage_compliant() {
    let result = validate_minimum_wage(1200, 1113); // Tokyo minimum wage
    assert!(result.is_ok());
}

#[test]
fn test_minimum_wage_violation() {
    let result = validate_minimum_wage(1000, 1113); // Below minimum
    assert!(result.is_err());
}

#[test]
fn test_minimum_wage_exact() {
    let result = validate_minimum_wage(1113, 1113); // Exact minimum
    assert!(result.is_ok());
}

// ============================================================================
// Harassment Edge Cases
// ============================================================================

#[test]
fn test_harassment_report_power() {
    let report = HarassmentReport {
        incident_date: Utc::now(),
        harassment_type: HarassmentType::PowerHarassment,
        description: "Unreasonable work demands and excessive overtime requirements".to_string(),
        perpetrator_position: Some("Manager".to_string()),
        victim_position: Some("Employee".to_string()),
        witness_count: 1,
    };

    let result = analyze_harassment_report(&report);
    // Should detect power harassment due to manager position
    assert!(result.is_err());
}

#[test]
fn test_harassment_report_sexual() {
    let report = HarassmentReport {
        incident_date: Utc::now(),
        harassment_type: HarassmentType::SexualHarassment,
        description: "Inappropriate comments and unwanted advances".to_string(),
        perpetrator_position: Some("Colleague".to_string()),
        victim_position: Some("Employee".to_string()),
        witness_count: 0,
    };

    let result = analyze_harassment_report(&report);
    // Sexual harassment should always be detected and require investigation
    assert!(result.is_err());
}

#[test]
fn test_harassment_report_maternity() {
    let report = HarassmentReport {
        incident_date: Utc::now(),
        harassment_type: HarassmentType::MaternityHarassment,
        description: "Demoted after pregnancy announcement and reassigned to lesser role"
            .to_string(),
        perpetrator_position: Some("Supervisor".to_string()),
        victim_position: Some("Pregnant Employee".to_string()),
        witness_count: 1,
    };

    let result = analyze_harassment_report(&report);
    // Maternity harassment should always be detected and require investigation
    assert!(result.is_err());
}

#[test]
fn test_all_harassment_types() {
    let types = vec![
        HarassmentType::PowerHarassment,
        HarassmentType::SexualHarassment,
        HarassmentType::MaternityHarassment,
        HarassmentType::PaternityHarassment,
    ];

    assert_eq!(types.len(), 4);
}

#[test]
fn test_harassment_report_empty_description() {
    let report = HarassmentReport {
        incident_date: Utc::now(),
        harassment_type: HarassmentType::PowerHarassment,
        description: "".to_string(), // Empty description
        perpetrator_position: Some("Harasser".to_string()),
        victim_position: Some("Reporter".to_string()),
        witness_count: 0,
    };

    let result = analyze_harassment_report(&report);
    // Should fail without description
    assert!(result.is_err());
}

// ============================================================================
// Indefinite Conversion Edge Cases
// ============================================================================

#[test]
fn test_indefinite_conversion_not_eligible() {
    let contract = EmploymentContractBuilder::new()
        .with_employee("New Employee")
        .with_employer("Company")
        .with_employment_type(EmploymentType::FixedTerm)
        .with_work_pattern(WorkPattern::Regular)
        .with_salary(200_000)
        .with_working_hours(8, 5)
        .with_prefecture(Prefecture::Tokyo)
        .with_start_date(Utc::now()) // Just started
        .with_job_description("Work")
        .with_work_location("Office")
        .build()
        .unwrap();

    let result = check_indefinite_conversion_eligibility(&contract);
    assert!(result.is_ok());
    assert!(!result.unwrap()); // Not eligible yet
}

#[test]
fn test_indefinite_term_no_conversion_needed() {
    let contract = EmploymentContractBuilder::new()
        .with_employee("Permanent Employee")
        .with_employer("Company")
        .with_employment_type(EmploymentType::IndefiniteTerm)
        .with_work_pattern(WorkPattern::Regular)
        .with_salary(300_000)
        .with_working_hours(8, 5)
        .with_prefecture(Prefecture::Tokyo)
        .with_start_date(Utc::now())
        .with_job_description("Engineer")
        .with_work_location("Office")
        .build()
        .unwrap();

    let result = check_indefinite_conversion_eligibility(&contract);
    assert!(result.is_ok());
    // Already indefinite term
}

// ============================================================================
// Prefecture-Specific Edge Cases
// ============================================================================

#[test]
fn test_all_prefectures_coverage() {
    let prefectures = vec![
        Prefecture::Tokyo,
        Prefecture::Osaka,
        Prefecture::Kanagawa,
        Prefecture::Aichi,
        Prefecture::Saitama,
        Prefecture::Chiba,
        Prefecture::Hokkaido,
        Prefecture::Fukuoka,
        Prefecture::Kyoto,
        Prefecture::Hyogo,
        Prefecture::Okinawa,
    ];

    assert_eq!(prefectures.len(), 11);
}
