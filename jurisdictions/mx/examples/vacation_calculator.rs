//! Example: Calculate vacation days and premium

use legalis_mx::common::MexicanCurrency;
use legalis_mx::labor_law::*;

fn main() {
    println!("=== Vacation Calculator ===\n");

    let daily_salary = MexicanCurrency::from_pesos(300);

    for years in [1, 2, 3, 5, 10, 20, 30] {
        let days = get_vacation_days(years);
        let premium = calculate_vacation_premium(daily_salary, days);
        let total = calculate_total_vacation_compensation(daily_salary, years);

        println!("After {} year(s) of service:", years);
        println!("  Vacation days: {} days", days);
        println!(
            "  Vacation salary: ${}.00 MXN",
            daily_salary.pesos() * days as i64
        );
        println!("  Vacation premium (25%): ${}.00 MXN", premium.pesos());
        println!("  Total compensation: ${}.00 MXN\n", total.pesos());
    }

    println!("Note: Vacation premium is 25% of vacation salary (Article 80 LFT)");
}
