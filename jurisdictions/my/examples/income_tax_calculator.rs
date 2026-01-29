//! Example: Malaysian personal income tax calculator.

use legalis_my::common::format_myr_cents;
use legalis_my::tax_law::*;

fn main() {
    println!("=== Malaysian Personal Income Tax Calculator (2024) ===\n");

    let calculator = IncomeTax::new();

    let incomes = vec![
        300_000,   // RM 3,000
        1000000,   // RM 10,000
        5000000,   // RM 50,000
        10000000,  // RM 100,000
        50000000,  // RM 500,000
        100000000, // RM 1,000,000
    ];

    for income_sen in incomes {
        let tax = calculator.calculate(income_sen);
        let effective_rate = calculator.effective_rate(income_sen);

        println!("Income: {}", format_myr_cents(income_sen));
        println!("  Tax: {}", format_myr_cents(tax));
        println!("  Effective Rate: {:.2}%", effective_rate);
        println!();
    }
}
