//! Example: EPF contribution calculator.

use legalis_my::common::format_myr_cents;
use legalis_my::employment_law::*;

fn main() {
    println!("=== Malaysian EPF Contribution Calculator ===\n");

    let salaries = vec![
        (25, 200_000), // RM 2,000
        (30, 300_000), // RM 3,000
        (35, 500_000), // RM 5,000
        (40, 700_000), // RM 7,000 (above ceiling)
    ];

    for (age, salary_sen) in salaries {
        let epf = EpfContribution::new(age, salary_sen);
        let breakdown = epf.calculate();

        println!("Age: {}, Salary: {}", age, format_myr_cents(salary_sen));
        println!(
            "  Employer ({}%): {}",
            epf.employer_rate,
            format_myr_cents(breakdown.employer_amount_sen)
        );
        println!(
            "  Employee ({}%): {}",
            epf.employee_rate,
            format_myr_cents(breakdown.employee_amount_sen)
        );
        println!(
            "  Total EPF: {}",
            format_myr_cents(breakdown.total_amount_sen)
        );
        println!();
    }
}
