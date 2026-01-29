//! Example: Stamp duty calculator for various instruments.

use legalis_my::common::format_myr_cents;
use legalis_my::tax_law::stamp_duty::*;

fn main() {
    println!("=== Malaysian Stamp Duty Calculator ===\n");

    // Property transfer
    println!("--- Property Transfer ---");
    let properties = vec![
        (30000000, false),  // RM 300,000, not first-time
        (45000000, true),   // RM 450,000, first-time buyer
        (100000000, false), // RM 1,000,000
    ];

    for (value, first_time) in properties {
        let duty = StampDuty::new(StampDutyType::PropertyTransfer, value)
            .with_first_time_buyer(first_time);

        println!("Property value: {}", format_myr_cents(value));
        println!(
            "First-time buyer: {}",
            if first_time { "Yes" } else { "No" }
        );
        println!("Stamp duty: {}\n", format_myr_cents(duty.calculate()));
    }

    // Loan agreement
    println!("--- Loan Agreement (0.5%) ---");
    let loan = StampDuty::new(StampDutyType::LoanAgreement, 50000000); // RM 500,000
    println!("Loan amount: {}", format_myr_cents(50000000));
    println!("Stamp duty: {}\n", format_myr_cents(loan.calculate()));

    // Share transfer
    println!("--- Share Transfer (0.3%) ---");
    let shares = StampDuty::new(StampDutyType::ShareTransfer, 20000000); // RM 200,000
    println!("Share value: {}", format_myr_cents(20000000));
    println!("Stamp duty: {}\n", format_myr_cents(shares.calculate()));

    // Tenancy agreement
    println!("--- Tenancy Agreement ---");
    let tenancy = StampDuty::new(StampDutyType::TenancyAgreement, 2400000); // RM 24,000 annual rent
    println!("Annual rent: {}", format_myr_cents(2400000));
    println!("Stamp duty: {}", format_myr_cents(tenancy.calculate()));
}
