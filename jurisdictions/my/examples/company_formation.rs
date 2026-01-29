//! Example: Company formation in Malaysia (Sdn Bhd).

use legalis_my::company_law::*;

fn main() {
    println!("=== Malaysian Company Formation Example ===\n");

    // Create director
    let director = Director::new("Ahmad bin Ali", "850123-01-5678", true);
    println!("Director: {}", director.name);

    // Create shareholder
    let shareholder = Shareholder::new("Ahmad bin Ali", "850123-01-5678", 10_000);
    println!(
        "Shareholder: {} with {} shares",
        shareholder.name, shareholder.shares
    );

    // Create share capital (RM 100,000)
    let share_capital = ShareCapital::new(10000000);
    println!(
        "Share capital: RM {:.2}",
        share_capital.amount_sen as f64 / 100.0
    );

    // Build company
    let company = Company::builder()
        .name("Tech Innovations")
        .company_type(CompanyType::PrivateLimited)
        .add_director(director)
        .add_shareholder(shareholder)
        .share_capital(share_capital)
        .registered_address("Kuala Lumpur")
        .build()
        .expect("Valid company");

    println!(
        "\nCompany formed: {} {}",
        company.name,
        CompanyType::PrivateLimited.suffix()
    );

    // Validate company
    match company.validate() {
        Ok(report) => {
            if report.valid {
                println!("✅ Company formation is valid!");
            } else {
                println!("❌ Company formation has issues:");
                for issue in report.issues {
                    println!("  - {}", issue);
                }
            }
        }
        Err(e) => eprintln!("Error validating company: {}", e),
    }
}
