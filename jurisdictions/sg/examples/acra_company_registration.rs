//! ACRA Company Registration Example
//!
//! This example demonstrates the complete process of forming a Singapore Pte Ltd company
//! and validating it against Companies Act requirements.
//!
//! ## Running this example
//!
//! ```bash
//! cargo run --example acra_company_registration
//! ```
//!
//! ## Legal Context
//!
//! This example shows:
//! - Company formation with resident director (s. 145)
//! - Share capital allocation
//! - ACRA UEN assignment
//! - Validation against Companies Act requirements

use chrono::Utc;
use legalis_sg::companies::*;

fn main() {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║   Singapore Company Registration - ACRA Formation Example     ║");
    println!("║                 新加坡公司注册 - ACRA 成立示例                      ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    // Example 1: Valid Pte Ltd Formation
    example_1_valid_pte_ltd();

    println!("\n{}\n", "═".repeat(70));

    // Example 2: Invalid - No Resident Director
    example_2_no_resident_director();

    println!("\n{}\n", "═".repeat(70));

    // Example 3: Share Capital Validation
    example_3_share_capital();

    println!("\n═════════════════════════════════════════════════════════════════");
    println!("✅ Examples completed successfully");
    println!("═════════════════════════════════════════════════════════════════\n");
}

fn example_1_valid_pte_ltd() {
    println!("📋 Example 1: Valid Private Limited Company Formation");
    println!("{}\n", "─".repeat(70));

    // Generate UEN
    let uen = generate_uen(UenType::LocalCompany, 2024);
    println!("Generated UEN: {}", uen);

    // Create company
    let mut company = Company::new(
        uen,
        "Tech Innovations Pte Ltd",
        CompanyType::PrivateLimited,
        Address::singapore("1 Raffles Place #12-34", "048616"),
    );

    // Set share capital
    company.share_capital = ShareCapital::new(100_000_00); // SGD 100,000
    company.share_capital.issued_shares = 10_000;
    company
        .share_capital
        .add_share_class(ShareClass::ordinary(10_000, Some(10_00)));

    // Add resident director (s. 145 requirement)
    company.directors.push(Director::new(
        "John Tan Wei Ming",
        "S1234567A",
        true, // Resident director
    ));

    // Add second director (non-resident)
    let mut foreign_director = Director::new(
        "Jane Smith",
        "P1234567",
        false, // Non-resident
    );
    foreign_director.nationality = "United States".to_string();
    company.directors.push(foreign_director);

    // Add shareholders
    company.shareholders.push(Shareholder {
        name: "John Tan Wei Ming".to_string(),
        identification: "S1234567A".to_string(),
        nationality_or_jurisdiction: "Singapore".to_string(),
        address: Address::singapore("123 Orchard Road", "238858"),
        share_allocation: ShareAllocation::new("Ordinary", 7_000, 10_00),
        acquisition_date: Utc::now(),
    });

    company.shareholders.push(Shareholder {
        name: "Jane Smith".to_string(),
        identification: "P1234567".to_string(),
        nationality_or_jurisdiction: "United States".to_string(),
        address: Address::foreign("456 Main St", "10001", "USA"),
        share_allocation: ShareAllocation::new("Ordinary", 3_000, 10_00),
        acquisition_date: Utc::now(),
    });

    // Validate total paid-up capital
    company.share_capital.paid_up_capital_cents = 100_000_00; // 10,000 shares × SGD 10

    // Add company secretary
    company.company_secretary = Some(CompanySecretary::new(
        "Corporate Secretary Services Pte Ltd",
        "202301234B",
    ));

    // Validate company formation
    println!("\n🔍 Validating company formation...\n");

    match validate_company_formation(&company) {
        Ok(report) => {
            if report.is_valid {
                println!("✅ Company formation is VALID");
                println!("   Status: Ready for ACRA registration");
            } else {
                println!("⚠️  Company formation has {} errors", report.errors.len());
                for error in &report.errors {
                    println!("   ❌ {}", error);
                }
            }

            if !report.warnings.is_empty() {
                println!("\n⚠️  Warnings ({}):", report.warnings.len());
                for warning in &report.warnings {
                    println!("   ⚠️  {}", warning);
                }
            }

            if !report.legal_references.is_empty() {
                println!("\n📖 Legal References:");
                for reference in &report.legal_references {
                    println!("   • {}", reference);
                }
            }
        }
        Err(e) => {
            println!("❌ Validation FAILED: {}", e);
        }
    }

    // Print company details
    println!("\n📊 Company Details:");
    println!("   Name: {}", company.name);
    println!("   Type: {}", company.company_type);
    println!("   UEN: {}", company.uen);
    println!("   Directors: {}", company.directors.len());
    println!(
        "     - {} (Resident: {})",
        company.directors[0].name, company.directors[0].is_resident_director
    );
    println!(
        "     - {} (Resident: {})",
        company.directors[1].name, company.directors[1].is_resident_director
    );
    println!("   Shareholders: {}", company.shareholders.len());
    println!(
        "   Share Capital: SGD {:.2}",
        company.share_capital.paid_up_sgd()
    );
    println!("   Registered Address: {}", company.registered_address);
}

fn example_2_no_resident_director() {
    println!("📋 Example 2: Invalid Formation - No Resident Director (s. 145 violation)");
    println!("{}\n", "─".repeat(70));

    let uen = generate_uen(UenType::LocalCompany, 2024);
    let mut company = Company::new(
        uen,
        "Global Services Pte Ltd",
        CompanyType::PrivateLimited,
        Address::singapore("2 Shenton Way", "068804"),
    );

    // Add only non-resident directors (VIOLATION of s. 145)
    let mut director1 = Director::new("Alice Johnson", "P7654321", false);
    director1.nationality = "United Kingdom".to_string();
    company.directors.push(director1);

    let mut director2 = Director::new("Bob Williams", "P9876543", false);
    director2.nationality = "Australia".to_string();
    company.directors.push(director2);

    println!("🔍 Validating company with no resident director...\n");

    match validate_company_formation(&company) {
        Ok(report) => {
            if !report.is_valid {
                println!("❌ Company formation is INVALID (as expected)");
                println!("   Errors found: {}", report.errors.len());
                for error in &report.errors {
                    println!("   ❌ {}", error);
                }
            }
        }
        Err(e) => {
            println!("❌ Validation error: {}", e);
        }
    }

    println!("\n💡 Solution:");
    println!(
        "   Appoint at least one director who is ordinarily resident in Singapore (CA s. 145)."
    );
}

fn example_3_share_capital() {
    println!("📋 Example 3: Share Capital Validation");
    println!("{}\n", "─".repeat(70));

    println!("Creating company with SGD 1,000,000 share capital...");

    let uen = generate_uen(UenType::LocalCompany, 2024);
    let mut company = Company::new(
        uen,
        "Capital Holdings Pte Ltd",
        CompanyType::PrivateLimited,
        Address::singapore("3 Marina Boulevard", "018982"),
    );

    // Set up share capital: 1 million shares at SGD 1 each
    company.share_capital = ShareCapital::new(1_000_000_00); // SGD 1 million
    company.share_capital.issued_shares = 1_000_000;
    company
        .share_capital
        .add_share_class(ShareClass::ordinary(1_000_000, Some(100))); // Par value SGD 1

    // Add resident director
    company
        .directors
        .push(Director::new("David Lim", "S9876543B", true));

    // Add major shareholder
    company.shareholders.push(Shareholder {
        name: "Investment Fund Ltd".to_string(),
        identification: "202000001C".to_string(),
        nationality_or_jurisdiction: "Singapore".to_string(),
        address: Address::singapore("50 Raffles Place", "048623"),
        share_allocation: ShareAllocation::new("Ordinary", 1_000_000, 100),
        acquisition_date: Utc::now(),
    });

    println!("\n📊 Share Capital Structure:");
    println!(
        "   Authorized Capital: SGD {:.2}",
        company.share_capital.authorized_sgd().unwrap_or(0.0)
    );
    println!(
        "   Paid-up Capital: SGD {:.2}",
        company.share_capital.paid_up_sgd()
    );
    println!("   Issued Shares: {}", company.share_capital.issued_shares);
    println!(
        "   Share Classes: {}",
        company.share_capital.share_classes.len()
    );

    match validate_share_capital(&company.share_capital) {
        Ok(()) => println!("\n✅ Share capital structure is VALID"),
        Err(e) => println!("\n❌ Share capital validation failed: {}", e),
    }

    // Calculate shareholder ownership
    println!("\n👥 Shareholder Ownership:");
    for shareholder in &company.shareholders {
        let ownership_pct = shareholder
            .share_allocation
            .ownership_percentage(company.share_capital.issued_shares);
        println!(
            "   {} - {:.2}% ({} shares, SGD {:.2})",
            shareholder.name,
            ownership_pct,
            shareholder.share_allocation.number_of_shares,
            shareholder.share_allocation.total_paid_sgd()
        );
    }
}
