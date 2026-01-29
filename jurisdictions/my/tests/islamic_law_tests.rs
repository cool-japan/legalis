//! Integration tests for Islamic law.

use legalis_my::islamic_law::*;

#[test]
fn test_islamic_marriage() {
    let marriage = family_law::IslamicMarriage::new(
        "Ahmad bin Abdullah",
        "850123-01-5678",
        "Fatimah binti Hassan",
        "900214-02-1234",
        "Hassan bin Ali",
        50_000, // 500 MYR in sen
    );

    let report = marriage.validate().expect("Validation succeeds");
    assert!(report.valid || report.issues.len() == 1); // May have registration warning
}

#[test]
fn test_islamic_finance_murabahah() {
    let product = finance::IslamicFinanceProduct::new(
        "Home Financing",
        finance::IslamicFinanceType::Murabahah,
        50000000, // 500,000 MYR in sen
        5000000,  // 50,000 MYR in sen
        "Property",
    )
    .with_syariah_certification("SAC");

    let report = product.validate().expect("Validation succeeds");
    assert!(report.compliant);
}

#[test]
fn test_riba_detection() {
    let contract = finance::IslamicContract::new(
        finance::IslamicFinanceType::Murabahah,
        vec!["Bank".to_string(), "Customer".to_string()],
    )
    .add_term("Interest rate of 5% per annum");

    let result = finance::validate_shariah_compliance(&contract);
    assert!(result.is_err());
}
