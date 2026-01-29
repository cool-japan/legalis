//! Integration tests for Malaysian employment law.

use legalis_my::employment_law::*;

#[test]
fn test_epf_calculation() {
    let epf = EpfContribution::new(30, 300_000); // RM 3,000
    let breakdown = epf.calculate();

    assert_eq!(breakdown.employer_amount_sen, 36_000); // 12%
    assert_eq!(breakdown.employee_amount_sen, 33_000); // 11%
}

#[test]
fn test_leave_entitlement_calculation() {
    let leave1 = LeaveEntitlement::calculate(1);
    assert_eq!(leave1.annual_leave_days, 8);

    let leave3 = LeaveEntitlement::calculate(3);
    assert_eq!(leave3.annual_leave_days, 12);

    let leave6 = LeaveEntitlement::calculate(6);
    assert_eq!(leave6.annual_leave_days, 16);
}

#[test]
fn test_working_hours_compliance() {
    let standard = WorkingHours::standard();
    assert!(standard.is_compliant());

    let excessive = WorkingHours {
        hours_per_day: 10,
        hours_per_week: 60,
        shift_work: false,
    };
    assert!(!excessive.is_compliant());
}
