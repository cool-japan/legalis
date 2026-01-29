//! Integration tests for Malaysian company law.

use legalis_my::company_law::*;

#[test]
fn test_sdn_bhd_formation() {
    let director = Director::new("Ahmad bin Ali", "850123-01-5678", true);
    let shareholder = Shareholder::new("Ahmad bin Ali", "850123-01-5678", 100);
    let share_capital = ShareCapital::new(10000000); // 100,000 MYR in sen

    let company = Company::builder()
        .name("Tech Innovations Sdn Bhd")
        .company_type(CompanyType::PrivateLimited)
        .add_director(director)
        .add_shareholder(shareholder)
        .share_capital(share_capital)
        .registered_address("Kuala Lumpur")
        .build()
        .expect("Valid company");

    assert_eq!(company.name, "Tech Innovations Sdn Bhd");
    assert_eq!(company.directors.len(), 1);
}

#[test]
fn test_minimum_share_capital_violation() {
    let director = Director::new("Ahmad bin Ali", "850123-01-5678", true);
    let shareholder = Shareholder::new("Ahmad bin Ali", "850123-01-5678", 100);
    let share_capital = ShareCapital::new(50); // Less than RM 1

    let company = Company::builder()
        .name("Invalid Co")
        .company_type(CompanyType::PrivateLimited)
        .add_director(director)
        .add_shareholder(shareholder)
        .share_capital(share_capital)
        .registered_address("KL")
        .build()
        .expect("Company built");

    let report = company.validate().expect("Validation succeeds");
    assert!(!report.valid);
}
