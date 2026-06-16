//! Integration tests for CPF (Central Provident Fund) contribution rates across
//! all age brackets, the Ordinary Wage ceiling, and contribution calculations.
//!
//! Rates reflect the schedule documented in the `employment` module. The tests
//! lock in the bracket boundaries and the internal consistency of the
//! calculation (rate -> contribution -> wage ceiling) so that regressions in the
//! bracketing logic are caught.

use legalis_sg::employment::*;

#[test]
fn test_cpf_rates_each_bracket() {
    assert_eq!(CpfContribution::rates_by_age(25), (1700, 2000)); // age <= 55
    assert_eq!(CpfContribution::rates_by_age(55), (1700, 2000)); // boundary
    assert_eq!(CpfContribution::rates_by_age(56), (1550, 1500)); // 56-60
    assert_eq!(CpfContribution::rates_by_age(60), (1550, 1500));
    assert_eq!(CpfContribution::rates_by_age(61), (1150, 950)); // 61-65
    assert_eq!(CpfContribution::rates_by_age(65), (1150, 950));
    assert_eq!(CpfContribution::rates_by_age(66), (900, 750)); // 66-70
    assert_eq!(CpfContribution::rates_by_age(70), (900, 750));
    assert_eq!(CpfContribution::rates_by_age(71), (750, 500)); // > 70
}

#[test]
fn test_cpf_bracket_boundaries_switch() {
    // Rates must change exactly at each statutory age boundary.
    assert_ne!(
        CpfContribution::rates_by_age(55),
        CpfContribution::rates_by_age(56)
    );
    assert_ne!(
        CpfContribution::rates_by_age(60),
        CpfContribution::rates_by_age(61)
    );
    assert_ne!(
        CpfContribution::rates_by_age(65),
        CpfContribution::rates_by_age(66)
    );
    assert_ne!(
        CpfContribution::rates_by_age(70),
        CpfContribution::rates_by_age(71)
    );
}

#[test]
fn test_cpf_contribution_amounts_under_ceiling() {
    let cpf = CpfContribution::new(30, 500_000); // age 30, SGD 5,000
    assert_eq!(cpf.employer_contribution_cents(), 85_000); // 17%
    assert_eq!(cpf.employee_contribution_cents(), 100_000); // 20%
    assert_eq!(cpf.total_contribution_cents(), 185_000); // 37%
}

#[test]
fn test_cpf_wage_ceiling_applied() {
    let cpf = CpfContribution::new(30, 800_000); // SGD 8,000 exceeds the ceiling
    assert_eq!(
        cpf.cpf_subject_wage_cents(),
        CpfContribution::ORDINARY_WAGE_CEILING_CENTS
    );
    let expected_employer = CpfContribution::ORDINARY_WAGE_CEILING_CENTS * 1700 / 10_000;
    assert_eq!(cpf.employer_contribution_cents(), expected_employer);
}

#[test]
fn test_cpf_calculation_validator_consistency() {
    for age in [25u32, 55, 56, 60, 61, 65, 66, 70, 71] {
        let cpf = CpfContribution::new(age, 400_000);
        validate_cpf_calculation(&cpf).expect("rates should be self-consistent");
    }
}

#[test]
fn test_cpf_senior_rates_lower_than_young() {
    let young = CpfContribution::new(30, 500_000);
    let senior = CpfContribution::new(68, 500_000);
    assert!(senior.total_contribution_cents() < young.total_contribution_cents());
}
