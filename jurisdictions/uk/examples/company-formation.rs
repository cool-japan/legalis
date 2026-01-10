//! Company Formation Examples (Companies Act 2006 Part 2)
//!
//! Demonstrates validation of company formation requirements under CA 2006.

use chrono::NaiveDate;
use legalis_uk::company::*;

fn main() {
    println!("=== UK Company Law: Company Formation Examples (CA 2006 Part 2) ===\n");

    example_1_valid_private_company();
    example_2_missing_suffix();
    example_3_sensitive_word();
    example_4_plc_insufficient_capital();
    example_5_plc_insufficient_paid_up();
    example_6_plc_missing_secretary();
    example_7_insufficient_directors();
}

/// Example 1: Valid private limited company formation
fn example_1_valid_private_company() {
    println!("Example 1: Valid Private Limited Company");
    println!("─────────────────────────────────────────");

    let formation = CompanyFormation {
        company_name: "Acme Trading Ltd".to_string(),
        company_type: CompanyType::PrivateLimitedByShares,
        registered_office: RegisteredOffice {
            address_line_1: "1 High Street".to_string(),
            address_line_2: None,
            city: "London".to_string(),
            county: Some("Greater London".to_string()),
            postcode: "SW1A 1AA".to_string(),
            country: RegisteredOfficeCountry::England,
        },
        share_capital: Some(ShareCapital {
            nominal_capital_gbp: 100.0,
            paid_up_capital_gbp: 100.0,
            number_of_shares: 100,
            nominal_value_per_share_gbp: 1.0,
            share_classes: vec![ShareClass {
                class_name: "Ordinary".to_string(),
                number_of_shares: 100,
                rights: ShareRights {
                    voting_rights: 1,
                    dividend_rights: DividendRights::Ordinary,
                    capital_rights: CapitalRights::Equal,
                },
            }],
        }),
        directors: vec![Director {
            name: "Alice Smith".to_string(),
            date_of_birth: NaiveDate::from_ymd_opt(1980, 5, 15).unwrap(),
            nationality: "British".to_string(),
            service_address: ServiceAddress {
                address_line_1: "10 Downing Street".to_string(),
                address_line_2: None,
                city: "London".to_string(),
                postcode: "SW1A 2AA".to_string(),
                country: "United Kingdom".to_string(),
            },
            appointment_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            resignation_date: None,
            director_type: DirectorType::Individual,
        }],
        shareholders: vec![Shareholder {
            name: "Alice Smith".to_string(),
            address: "10 Downing Street, London, SW1A 2AA".to_string(),
            number_of_shares: 100,
            share_class: "Ordinary".to_string(),
            amount_paid_gbp: 100.0,
            amount_unpaid_gbp: 0.0,
        }],
        secretary: None, // Optional for private companies
        statement_of_compliance: true,
        formation_date: Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
    };

    match validate_company_formation(&formation) {
        Ok(_) => {
            println!("✓ Company formation VALID");
            println!("  Company: {}", formation.company_name);
            println!("  Type: {:?}", formation.company_type);
            println!(
                "  Share Capital: £{:.2}",
                formation
                    .share_capital
                    .as_ref()
                    .unwrap()
                    .nominal_capital_gbp
            );
            println!("  Directors: {}", formation.directors.len());
            println!(
                "  Registered Office: {}, {}",
                formation.registered_office.city, formation.registered_office.postcode
            );
            println!("  Reference: CA 2006 Part 2 (ss.7-16)");
        }
        Err(e) => println!("✗ Validation failed: {}", e),
    }
    println!();
}

/// Example 2: Company name missing required suffix
fn example_2_missing_suffix() {
    println!("Example 2: Missing Required Suffix");
    println!("───────────────────────────────────");

    let result = validate_company_name("Acme Trading", CompanyType::PrivateLimitedByShares);

    match result {
        Ok(_) => println!("✓ Name valid"),
        Err(e) => {
            println!("✗ Name INVALID: {}", e);
            println!("  Explanation: Private limited companies must end with 'Limited' or 'Ltd'");
            println!("  Reference: CA 2006 s.59");
        }
    }
    println!();
}

/// Example 3: Company name contains sensitive word
fn example_3_sensitive_word() {
    println!("Example 3: Sensitive Word Requiring Approval");
    println!("─────────────────────────────────────────────");

    let result = validate_company_name("Royal Trading Ltd", CompanyType::PrivateLimitedByShares);

    match result {
        Ok(_) => println!("✓ Name valid"),
        Err(e) => {
            println!("✗ Name INVALID: {}", e);
            println!(
                "  Explanation: 'Royal' is a sensitive word requiring Secretary of State approval"
            );
            println!(
                "  Reference: CA 2006 s.55 and Company and Business Names (Miscellaneous Provisions) Regulations 2009"
            );
        }
    }
    println!();
}

/// Example 4: PLC with insufficient share capital
fn example_4_plc_insufficient_capital() {
    println!("Example 4: PLC with Insufficient Share Capital");
    println!("───────────────────────────────────────────────");

    let capital = ShareCapital {
        nominal_capital_gbp: 40_000.0, // Below £50k minimum
        paid_up_capital_gbp: 10_000.0,
        number_of_shares: 40_000,
        nominal_value_per_share_gbp: 1.0,
        share_classes: vec![],
    };

    match validate_share_capital(&capital, CompanyType::PublicLimitedCompany) {
        Ok(_) => println!("✓ Share capital valid"),
        Err(e) => {
            println!("✗ Share capital INVALID: {}", e);
            println!("  Minimum required: £50,000");
            println!("  Provided: £{:.2}", capital.nominal_capital_gbp);
            println!("  Reference: CA 2006 s.763 (Minimum share capital for public companies)");
        }
    }
    println!();
}

/// Example 5: PLC with insufficient paid up capital
fn example_5_plc_insufficient_paid_up() {
    println!("Example 5: PLC with Insufficient Paid Up Capital");
    println!("─────────────────────────────────────────────────");

    let capital = ShareCapital {
        nominal_capital_gbp: 50_000.0,
        paid_up_capital_gbp: 10_000.0, // Only 20%, need 25%
        number_of_shares: 50_000,
        nominal_value_per_share_gbp: 1.0,
        share_classes: vec![],
    };

    println!("Nominal capital: £{:.2}", capital.nominal_capital_gbp);
    println!("Paid up capital: £{:.2}", capital.paid_up_capital_gbp);
    println!("Percentage paid up: {:.1}%", capital.percentage_paid_up());

    match validate_share_capital(&capital, CompanyType::PublicLimitedCompany) {
        Ok(_) => println!("✓ Share capital valid"),
        Err(e) => {
            println!("✗ Share capital INVALID: {}", e);
            println!("  Minimum required: 25% paid up");
            println!("  Reference: CA 2006 s.586 (Public companies: allotment of shares)");
        }
    }
    println!();
}

/// Example 6: PLC missing company secretary
fn example_6_plc_missing_secretary() {
    println!("Example 6: PLC Missing Company Secretary");
    println!("─────────────────────────────────────────");

    let formation = CompanyFormation {
        company_name: "Tech Innovations PLC".to_string(),
        company_type: CompanyType::PublicLimitedCompany,
        registered_office: RegisteredOffice {
            address_line_1: "1 Business Park".to_string(),
            address_line_2: None,
            city: "Manchester".to_string(),
            county: Some("Greater Manchester".to_string()),
            postcode: "M1 1AA".to_string(),
            country: RegisteredOfficeCountry::England,
        },
        share_capital: Some(ShareCapital {
            nominal_capital_gbp: 50_000.0,
            paid_up_capital_gbp: 12_500.0, // 25%
            number_of_shares: 50_000,
            nominal_value_per_share_gbp: 1.0,
            share_classes: vec![],
        }),
        directors: vec![
            Director {
                name: "Director 1".to_string(),
                date_of_birth: NaiveDate::from_ymd_opt(1975, 3, 10).unwrap(),
                nationality: "British".to_string(),
                service_address: ServiceAddress {
                    address_line_1: "Address 1".to_string(),
                    address_line_2: None,
                    city: "Manchester".to_string(),
                    postcode: "M1 2AA".to_string(),
                    country: "United Kingdom".to_string(),
                },
                appointment_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                resignation_date: None,
                director_type: DirectorType::Individual,
            },
            Director {
                name: "Director 2".to_string(),
                date_of_birth: NaiveDate::from_ymd_opt(1980, 7, 20).unwrap(),
                nationality: "British".to_string(),
                service_address: ServiceAddress {
                    address_line_1: "Address 2".to_string(),
                    address_line_2: None,
                    city: "Manchester".to_string(),
                    postcode: "M1 3AA".to_string(),
                    country: "United Kingdom".to_string(),
                },
                appointment_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                resignation_date: None,
                director_type: DirectorType::Individual,
            },
        ],
        shareholders: vec![],
        secretary: None, // MISSING - required for PLC
        statement_of_compliance: true,
        formation_date: Some(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
    };

    match validate_company_formation(&formation) {
        Ok(_) => println!("✓ Formation valid"),
        Err(e) => {
            println!("✗ Formation INVALID: {}", e);
            println!("  Explanation: Public companies MUST have a company secretary");
            println!("  Private companies: Secretary is optional (CA 2006 s.270)");
            println!(
                "  Reference: CA 2006 s.271 (Public companies: requirement to have secretary)"
            );
        }
    }
    println!();
}

/// Example 7: PLC with insufficient directors
fn example_7_insufficient_directors() {
    println!("Example 7: PLC with Insufficient Directors");
    println!("───────────────────────────────────────────");

    let directors = vec![Director {
        name: "Solo Director".to_string(),
        date_of_birth: NaiveDate::from_ymd_opt(1975, 6, 15).unwrap(),
        nationality: "British".to_string(),
        service_address: ServiceAddress {
            address_line_1: "1 Business Street".to_string(),
            address_line_2: None,
            city: "Birmingham".to_string(),
            postcode: "B1 1AA".to_string(),
            country: "United Kingdom".to_string(),
        },
        appointment_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        resignation_date: None,
        director_type: DirectorType::Individual,
    }];

    match validate_directors(&directors, CompanyType::PublicLimitedCompany) {
        Ok(_) => println!("✓ Directors valid"),
        Err(e) => {
            println!("✗ Directors INVALID: {}", e);
            println!("  Private company requirement: Minimum 1 director");
            println!("  Public company requirement: Minimum 2 directors");
            println!("  Reference: CA 2006 s.154 (Companies required to have directors)");
        }
    }
    println!();
}
