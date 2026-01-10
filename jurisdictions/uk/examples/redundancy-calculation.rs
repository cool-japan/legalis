//! Redundancy Payment Calculation Examples
//!
//! Demonstrates statutory redundancy payment calculations under ERA 1996 s.162
//!
//! Age-based multipliers:
//! - Under 22: 0.5 week's pay per year
//! - 22-40: 1.0 week's pay per year
//! - 41+: 1.5 weeks' pay per year
//!
//! Limits:
//! - Maximum 20 years counted
//! - Weekly pay capped at £700 (April 2024)

use legalis_uk::employment::*;

fn main() {
    println!("=== UK Statutory Redundancy Payment Calculator ===\n");
    println!("ERA 1996 s.162\n");
    println!("Age-based multipliers:");
    println!("  • Under 22: 0.5× week's pay per year");
    println!("  • 22-40: 1.0× week's pay per year");
    println!("  • 41+: 1.5× weeks' pay per year");
    println!("Limits: Max 20 years, £700/week cap\n");
    println!("================================================\n");

    // Example 1: Employee under 22
    example_1_under_22();

    // Example 2: Employee 22-40
    example_2_age_22_to_40();

    // Example 3: Employee 41+
    example_3_age_41_plus();

    // Example 4: Long service (>20 years)
    example_4_long_service();

    // Example 5: High earner (weekly pay above cap)
    example_5_high_earner();

    // Example 6: Complex age transitions
    example_6_age_transitions();
}

fn example_1_under_22() {
    println!("Example 1: Employee Under 22");
    println!("==============================\n");

    let redundancy = RedundancyPayment {
        age: 21,
        years_of_service: 3,
        weekly_pay_gbp: 400.0,
    };

    let payment = redundancy.calculate_statutory_payment();

    println!("Employee Details:");
    println!("  Age: {}", redundancy.age);
    println!("  Years of service: {}", redundancy.years_of_service);
    println!("  Weekly pay: £{:.2}", redundancy.weekly_pay_gbp);
    println!("\nCalculation:");
    println!("  Multiplier: 0.5× (under 22)");
    println!(
        "  Formula: {} years × 0.5 × £{:.2}",
        redundancy.years_of_service, redundancy.weekly_pay_gbp
    );
    println!("\n✅ Statutory Redundancy Payment: £{:.2}\n", payment);
}

fn example_2_age_22_to_40() {
    println!("Example 2: Employee Aged 22-40");
    println!("================================\n");

    let redundancy = RedundancyPayment {
        age: 30,
        years_of_service: 8,
        weekly_pay_gbp: 650.0,
    };

    let payment = redundancy.calculate_statutory_payment();

    println!("Employee Details:");
    println!("  Age: {}", redundancy.age);
    println!("  Years of service: {}", redundancy.years_of_service);
    println!("  Weekly pay: £{:.2}", redundancy.weekly_pay_gbp);
    println!("\nCalculation:");
    println!("  Multiplier: 1.0× (age 22-40)");
    println!(
        "  Formula: {} years × 1.0 × £{:.2}",
        redundancy.years_of_service, redundancy.weekly_pay_gbp
    );
    println!("\n✅ Statutory Redundancy Payment: £{:.2}\n", payment);
}

fn example_3_age_41_plus() {
    println!("Example 3: Employee Aged 41+");
    println!("==============================\n");

    let redundancy = RedundancyPayment {
        age: 45,
        years_of_service: 10,
        weekly_pay_gbp: 600.0,
    };

    let payment = redundancy.calculate_statutory_payment();

    println!("Employee Details:");
    println!("  Age: {}", redundancy.age);
    println!("  Years of service: {}", redundancy.years_of_service);
    println!("  Weekly pay: £{:.2}", redundancy.weekly_pay_gbp);
    println!("\nCalculation:");
    println!("  Multiplier: 1.5× (age 41+)");
    println!(
        "  Formula: {} years × 1.5 × £{:.2}",
        redundancy.years_of_service, redundancy.weekly_pay_gbp
    );
    println!(
        "  = {} × 1.5 × £{:.2}",
        redundancy.years_of_service, redundancy.weekly_pay_gbp
    );
    println!("\n✅ Statutory Redundancy Payment: £{:.2}\n", payment);
}

fn example_4_long_service() {
    println!("Example 4: Long Service (>20 Years)");
    println!("=====================================\n");

    let redundancy = RedundancyPayment {
        age: 55,
        years_of_service: 25, // Only 20 years counted
        weekly_pay_gbp: 600.0,
    };

    let payment = redundancy.calculate_statutory_payment();

    println!("Employee Details:");
    println!("  Age: {}", redundancy.age);
    println!("  Years of service: {} ⚠️", redundancy.years_of_service);
    println!("  Weekly pay: £{:.2}", redundancy.weekly_pay_gbp);
    println!("\nCalculation:");
    println!("  ⚠️ Maximum 20 years counted (ERA 1996 s.162)");
    println!("  Years used: 20 (not 25)");
    println!("  Multiplier: 1.5× (age 41+)");
    println!(
        "  Formula: 20 years × 1.5 × £{:.2}",
        redundancy.weekly_pay_gbp
    );
    println!("  = 20 × 1.5 × £{:.2}", redundancy.weekly_pay_gbp);
    println!("\n✅ Statutory Redundancy Payment: £{:.2}", payment);
    println!(
        "   (Would be £{:.2} if all 25 years counted)\n",
        25.0 * 1.5 * redundancy.weekly_pay_gbp
    );
}

fn example_5_high_earner() {
    println!("Example 5: High Earner (Weekly Pay Above £700 Cap)");
    println!("====================================================\n");

    let redundancy = RedundancyPayment {
        age: 50,
        years_of_service: 15,
        weekly_pay_gbp: 1200.0, // Above £700 cap
    };

    let payment = redundancy.calculate_statutory_payment();

    println!("Employee Details:");
    println!("  Age: {}", redundancy.age);
    println!("  Years of service: {}", redundancy.years_of_service);
    println!("  Actual weekly pay: £{:.2} ⚠️", redundancy.weekly_pay_gbp);
    println!("\nCalculation:");
    println!("  ⚠️ Weekly pay capped at £700 (April 2024)");
    println!("  Capped weekly pay: £700.00");
    println!("  Multiplier: 1.5× (age 41+)");
    println!(
        "  Formula: {} years × 1.5 × £700.00",
        redundancy.years_of_service
    );
    println!("  = {} × 1.5 × £700.00", redundancy.years_of_service);
    println!("\n✅ Statutory Redundancy Payment: £{:.2}", payment);
    println!(
        "   (Would be £{:.2} without cap)\n",
        redundancy.years_of_service as f64 * 1.5 * redundancy.weekly_pay_gbp
    );
}

fn example_6_age_transitions() {
    println!("Example 6: Complex Age Transitions");
    println!("====================================\n");
    println!("Employee started at age 19, redundant at age 45");
    println!("26 years service (20 counted)\n");

    // This demonstrates how age multipliers change over career
    // In practice, each year would use the multiplier for that year's age
    // For statutory calculation, we use the age at redundancy

    let redundancy = RedundancyPayment {
        age: 45,
        years_of_service: 20, // Max counted
        weekly_pay_gbp: 550.0,
    };

    let payment = redundancy.calculate_statutory_payment();

    println!("Service Breakdown:");
    println!("  Age 19-21 (3 years): 0.5× multiplier");
    println!("  Age 22-40 (19 years): 1.0× multiplier");
    println!("  Age 41-45 (4 years): 1.5× multiplier");
    println!("  Total: 26 years (20 counted max)");
    println!("\nNote: Statutory calculation uses age at redundancy date");
    println!("  Age: {}", redundancy.age);
    println!("  Multiplier: 1.5× (age 41+)");
    println!("  Years: {} (max 20)", redundancy.years_of_service);
    println!("  Weekly pay: £{:.2}", redundancy.weekly_pay_gbp);
    println!("\nCalculation:");
    println!(
        "  Formula: 20 years × 1.5 × £{:.2}",
        redundancy.weekly_pay_gbp
    );
    println!("\n✅ Statutory Redundancy Payment: £{:.2}\n", payment);
}
