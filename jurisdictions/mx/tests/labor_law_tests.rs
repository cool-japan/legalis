//! Integration tests for labor law

use chrono::Utc;
use legalis_mx::common::MexicanCurrency;
use legalis_mx::labor_law::*;

#[test]
fn test_aguinaldo_full_year() {
    let daily_salary = MexicanCurrency::from_pesos(300);
    let aguinaldo = calculate_aguinaldo(daily_salary, 365);

    // 15 days * 300 pesos = 4,500 pesos
    assert_eq!(aguinaldo.pesos(), 4500);
}

#[test]
fn test_aguinaldo_partial_year() {
    let daily_salary = MexicanCurrency::from_pesos(300);
    let aguinaldo = calculate_aguinaldo(daily_salary, 182); // ~6 months

    // Should be approximately half
    assert!(aguinaldo.pesos() >= 2200 && aguinaldo.pesos() <= 2300);
}

#[test]
fn test_vacation_days_progression() {
    assert_eq!(get_vacation_days(1), 12);
    assert_eq!(get_vacation_days(2), 14);
    assert_eq!(get_vacation_days(3), 16);
    assert_eq!(get_vacation_days(5), 20);
}

#[test]
fn test_vacation_premium() {
    let daily_salary = MexicanCurrency::from_pesos(300);
    let vacation_days = 12;

    let premium = calculate_vacation_premium(daily_salary, vacation_days);

    // 12 days * 300 = 3,600; Premium: 3,600 * 0.25 = 900
    assert_eq!(premium.pesos(), 900);
}

#[test]
fn test_working_hours_validation() {
    let schedule = WorkSchedule::standard_day();
    assert!(validate_schedule(&schedule).is_ok());
}

#[test]
fn test_overtime_calculation() {
    let overtime = calculate_overtime(8, 10, WorkDayType::Day);
    assert_eq!(overtime, 2);
}

#[test]
fn test_employment_contract_validation() {
    let contract = EmploymentContract::new(
        "Juan Pérez".to_string(),
        "Empresa SA".to_string(),
        EmploymentType::Indefinite,
        MexicanCurrency::from_pesos(300),
        WorkSchedule::standard_day(),
        Utc::now(),
    );

    assert!(validate_employment_contract(&contract).is_ok());
}
