//! Integration tests for Companies Act (Cap. 50 / Companies Act 1967) validation:
//! formation, UEN classification, share capital, director eligibility and
//! disqualification, and the company secretary requirement.

use chrono::{Duration, Utc};
use legalis_sg::citation::Statute;
use legalis_sg::companies::*;

/// Builds a fully compliant private limited company.
fn valid_company() -> Company {
    let mut company = Company::new(
        "202401234A",
        "Tech Innovations Pte Ltd",
        CompanyType::PrivateLimited,
        Address::singapore("1 Raffles Place", "048616"),
    );
    company
        .directors
        .push(Director::new("John Tan", "S1234567A", true));
    company.share_capital = ShareCapital::no_par_value(1_000_000, 100); // SGD 10,000 / 100 shares
    company.shareholders.push(Shareholder {
        name: "John Tan".to_string(),
        identification: "S1234567A".to_string(),
        nationality_or_jurisdiction: "Singapore".to_string(),
        address: Address::singapore("123 Main Street", "123456"),
        share_allocation: ShareAllocation::new("Ordinary", 100, 10_000),
        acquisition_date: Utc::now(),
    });
    company.company_secretary = Some(CompanySecretary::new("Mary Lim", "S7654321B"));
    company
}

#[test]
fn test_valid_company_formation() {
    let company = valid_company();
    let report = validate_company_formation(&company).expect("validation should run");
    assert!(report.is_valid, "unexpected errors: {:?}", report.errors);
    assert!(report.errors.is_empty());
}

#[test]
fn test_invalid_uen_blocks_formation() {
    let mut company = valid_company();
    company.uen = "BAD-UEN".to_string();
    let report = validate_company_formation(&company).expect("validation should run");
    assert!(!report.is_valid);
    assert!(report.errors.iter().any(|e| e.contains("UEN")));
}

#[test]
fn test_missing_resident_director_blocks_formation() {
    let mut company = valid_company();
    company.directors = vec![Director::new("Jane Doe", "P1234567", false)];
    let report = validate_company_formation(&company).expect("validation should run");
    assert!(!report.is_valid);
    assert!(
        report
            .errors
            .iter()
            .any(|e| e.contains("resident director"))
    );
}

#[test]
fn test_uen_classification_formats() {
    assert_eq!(classify_uen("53123456B"), Some(UenFormat::Business));
    assert_eq!(classify_uen("202401234A"), Some(UenFormat::LocalCompany));
    assert_eq!(classify_uen("T08LP1234C"), Some(UenFormat::OtherEntity));
    assert_eq!(classify_uen("not-a-uen"), None);

    assert!(validate_uen("202401234A").is_ok());
    assert!(validate_uen("12345").is_err());
}

#[test]
fn test_share_capital_validation() {
    let ok = ShareCapital::no_par_value(1_000_000, 0);
    assert!(validate_share_capital(&ok).is_ok());

    let zero = ShareCapital::new(0);
    assert!(validate_share_capital(&zero).is_err());
}

#[test]
fn test_par_and_no_par_value_capital() {
    // No-par-value capital (default for modern Singapore companies, s. 62A).
    let no_par = ShareCapital::no_par_value(5_000_000, 5_000);
    assert!(!no_par.has_par_value);
    assert_eq!(no_par.paid_up_sgd(), 50_000.0);

    // Par-value class can still be represented.
    let par_class = ShareClass::ordinary(1_000, Some(100)); // SGD 1.00 par
    assert_eq!(par_class.par_value_cents, Some(100));
}

#[test]
fn test_shareholder_ownership_exceeds_issued() {
    let shareholders = vec![Shareholder {
        name: "Greedy Holdings".to_string(),
        identification: "202000001C".to_string(),
        nationality_or_jurisdiction: "Singapore".to_string(),
        address: Address::singapore("50 Raffles Place", "048623"),
        share_allocation: ShareAllocation::new("Ordinary", 150, 100),
        acquisition_date: Utc::now(),
    }];
    assert!(validate_shareholder_ownership(&shareholders, 100).is_err());
    assert!(validate_shareholder_ownership(&shareholders, 200).is_ok());
}

#[test]
fn test_director_disqualification_expiry() {
    let now = Utc::now();
    let mut director = Director::new("Alex Tan", "S3456789C", true);
    director.disqualification_status = DisqualificationStatus::ConvictionDisqualification {
        conviction_date: now - Duration::days(2000),
        offense: "Fraud".to_string(),
        disqualification_until: now - Duration::days(1),
    };
    // Recorded status is not Eligible, but the 5-year period has elapsed.
    assert!(!director.is_eligible());
    assert!(validate_director_disqualification(&director, now).is_ok());
    assert!(validate_director_disqualification(&director, now - Duration::days(30)).is_err());
}

#[test]
fn test_director_disqualification_sections() {
    let now = Utc::now();
    let bankruptcy = DisqualificationStatus::BankruptcyDisqualification {
        bankruptcy_date: now,
    };
    assert_eq!(bankruptcy.statute_section(), Some("CA s. 148"));

    let conviction = DisqualificationStatus::ConvictionDisqualification {
        conviction_date: now,
        offense: "Cheating".to_string(),
        disqualification_until: now + Duration::days(1825),
    };
    assert_eq!(conviction.statute_section(), Some("CA s. 154"));
}

#[test]
fn test_company_secretary_requirement_grace_and_breach() {
    // Within the 6-month grace, absence of a secretary is acceptable (s. 171(1)).
    let mut company = valid_company();
    company.company_secretary = None;
    company.registration_date = Utc::now();
    assert!(validate_company_secretary_requirement(&company, Utc::now()).is_ok());

    // Past the grace period, an unfilled office is a breach.
    company.registration_date = Utc::now() - Duration::days(250);
    assert!(matches!(
        validate_company_secretary_requirement(&company, Utc::now()),
        Err(CompaniesError::CompanySecretaryVacancyExceeded { .. })
    ));
}

#[test]
fn test_citation_system_links_to_statute() {
    // The validators are grounded in the Companies Act 1967 (Cap. 50); the
    // citation module can render the governing section.
    let companies_act =
        Statute::with_chapter("Companies Act", 50, Some(1967)).with_short_name("CA");
    let s145 = companies_act.section(145, Some(1));
    assert_eq!(s145.short_citation(), "CA s. 145(1)");
    assert_eq!(
        CompaniesError::NoResidentDirector.statute_reference(),
        Some("CA s. 145(1)")
    );
}
