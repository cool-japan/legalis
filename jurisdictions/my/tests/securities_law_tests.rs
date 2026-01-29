//! Integration tests for securities law.

use legalis_my::securities_law::*;

#[test]
fn test_licensed_intermediary() {
    let intermediary = LicensedIntermediary::new(
        "Securities Firm Sdn Bhd",
        "SC-12345",
        vec![
            LicenseType::DealingSecurities,
            LicenseType::InvestmentAdvice,
        ],
    );

    assert!(intermediary.is_licensed_for(LicenseType::DealingSecurities));
    assert!(!intermediary.is_licensed_for(LicenseType::FundManagement));
}

#[test]
fn test_listed_company() {
    let company = ListedCompany::new(
        "Tech Corporation Bhd",
        "TECH",
        Market::MainMarket,
        50_000_000_000, // 500,000,000 MYR in sen
    );

    assert_eq!(company.stock_code, "TECH");
    assert!(company.compliant);
}

#[test]
fn test_ipo_prospectus() {
    let prospectus = Prospectus::new("NewCo Bhd", 10_000, 10_000_000).with_sc_approval(true);

    assert!(prospectus.sc_approved);
}
