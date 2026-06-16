//! Redundancy Payment Calculation Examples
//!
//! Demonstrates statutory redundancy payment calculations under ERA 1996 s.162.
//!
//! Service is reckoned **backwards** from the end of employment, allowing for each
//! complete year of service:
//! - Under 22: 0.5 week's pay
//! - 22-40: 1.0 week's pay
//! - 41+: 1.5 weeks' pay
//!
//! The band for each year is fixed by the employee's age *during that year*, so an
//! employee who crossed an age band during their employment is reckoned year-by-year
//! rather than at a single multiplier for their age at the dismissal date.
//!
//! Limits:
//! - Maximum 20 years reckoned (ERA 1996 s.162(3))
//! - A week's pay capped at £700 (ERA 1996 s.227, April 2024)

use legalis_uk::employment::*;

fn main() {
    println!("=== UK Statutory Redundancy Payment Calculator ===\n");
    println!("ERA 1996 s.162 (age-banded reckoning, counted backwards)\n");
    println!("Weeks' pay allowed per complete year of service:");
    println!("  • Under 22: 0.5 week's pay");
    println!("  • 22-40: 1.0 week's pay");
    println!("  • 41+: 1.5 weeks' pay");
    println!("Limits: max 20 years reckoned, £700/week cap (s.227)\n");
    println!("================================================\n");

    print_case(
        "Example 1: Employee Under 22",
        RedundancyPayment {
            age: 21,
            years_of_service: 3,
            weekly_pay_gbp: 400.0,
        },
    );

    print_case(
        "Example 2: Employee Aged 22-40",
        RedundancyPayment {
            age: 30,
            years_of_service: 8,
            weekly_pay_gbp: 650.0,
        },
    );

    print_case(
        "Example 3: Employee Aged 41+ (band crossing)",
        RedundancyPayment {
            age: 45,
            years_of_service: 10,
            weekly_pay_gbp: 600.0,
        },
    );

    print_case(
        "Example 4: Long Service (>20 years, capped)",
        RedundancyPayment {
            age: 55,
            years_of_service: 25,
            weekly_pay_gbp: 600.0,
        },
    );

    print_case(
        "Example 5: High Earner (weekly pay above £700 cap)",
        RedundancyPayment {
            age: 50,
            years_of_service: 15,
            weekly_pay_gbp: 1200.0,
        },
    );

    print_case(
        "Example 6: Career spanning all three age bands",
        RedundancyPayment {
            age: 45,
            years_of_service: 20,
            weekly_pay_gbp: 550.0,
        },
    );
}

/// Print a single worked redundancy calculation with its age-banded breakdown.
fn print_case(label: &str, redundancy: RedundancyPayment) {
    let reckoning = redundancy.reckoning();
    let capped_weekly_pay = redundancy.capped_weekly_pay();
    let payment = redundancy.calculate_statutory_payment();

    println!("{label}");
    println!("{}\n", "=".repeat(label.len()));

    println!("Employee Details:");
    println!("  Age at redundancy: {}", redundancy.age);
    println!("  Years of service: {}", redundancy.years_of_service);
    println!("  Weekly pay: £{:.2}", redundancy.weekly_pay_gbp);
    if redundancy.weekly_pay_gbp > capped_weekly_pay {
        println!("  Capped weekly pay: £{capped_weekly_pay:.2} (s.227 cap)");
    }
    if redundancy.years_of_service > MAX_RECKONABLE_YEARS {
        println!(
            "  Reckonable years: {MAX_RECKONABLE_YEARS} (service over 20 years is disregarded)"
        );
    }

    println!("\nAge-banded reckoning (ERA 1996 s.162(2)):");
    println!(
        "  Years at 1.5×: {} (aged 41+)",
        reckoning.years_at_one_and_half
    );
    println!("  Years at 1.0×: {} (aged 22-40)", reckoning.years_at_one);
    println!(
        "  Years at 0.5×: {} (aged under 22)",
        reckoning.years_at_half
    );
    println!("  Weeks' pay due: {:.1}", reckoning.weeks_due());

    println!(
        "\n  Formula: {:.1} weeks × £{capped_weekly_pay:.2}",
        reckoning.weeks_due()
    );
    println!("\nStatutory Redundancy Payment: £{payment:.2}\n");
}
