//! Example: Islamic finance product (Murabahah home financing).

use legalis_my::common::format_myr_cents;
use legalis_my::islamic_law::finance::*;

fn main() {
    println!("=== Islamic Finance Product Example: Murabahah ===\n");

    // Create Murabahah home financing product
    let product = IslamicFinanceProduct::new(
        "Home Financing-i",
        IslamicFinanceType::Murabahah,
        50000000, // RM 500,000 principal
        10000000, // RM 100,000 profit
        "Residential property at Kuala Lumpur",
    )
    .with_syariah_certification("Syariah Advisory Council");

    println!("Product: {}", product.name);
    println!(
        "Type: {:?} - {}",
        product.product_type,
        product.product_type.description()
    );
    println!("Principal: {}", format_myr_cents(product.principal_sen));
    println!("Profit: {}", format_myr_cents(product.profit_sen));
    println!("Underlying Asset: {}", product.underlying_asset);
    println!(
        "Syariah Certified: {}",
        if product.syariah_certified {
            "Yes"
        } else {
            "No"
        }
    );

    if let Some(advisor) = &product.syariah_advisor {
        println!("Syariah Advisor: {}", advisor);
    }

    // Validate Syariah compliance
    match product.validate() {
        Ok(report) => {
            println!("\n=== Syariah Compliance Report ===");
            println!(
                "Status: {}",
                if report.compliant {
                    "✅ Compliant"
                } else {
                    "❌ Non-compliant"
                }
            );

            if !report.issues.is_empty() {
                println!("\nIssues:");
                for issue in report.issues {
                    println!("  - {}", issue);
                }
            }

            if !report.recommendations.is_empty() {
                println!("\nRecommendations:");
                for rec in report.recommendations {
                    println!("  - {}", rec);
                }
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
