//! Integration tests for Malaysian tax law.

use legalis_my::tax_law::*;

#[test]
fn test_income_tax_calculation() {
    let tax = calculate_income_tax(10000000); // RM 100,000
    assert!(tax > 0);
}

#[test]
fn test_sst_calculation() {
    let sales_tax = SalesTax::new("Electronics", 100_000); // RM 1,000
    let tax = sales_tax.calculate();
    assert_eq!(tax, 10_000); // 10%

    let service_tax = ServiceTax::new("Legal services", 200_000, sst::ServiceType::Professional);
    let stax = service_tax.calculate();
    assert_eq!(stax, 12_000); // 6%
}

#[test]
fn test_stamp_duty() {
    let duty = calculate_stamp_duty(StampDutyType::PropertyTransfer, 30000000); // RM 300,000
    assert!(duty > 0);
}
