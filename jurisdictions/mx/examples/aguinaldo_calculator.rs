//! Example: Calculate aguinaldo (Christmas bonus)

use legalis_mx::common::MexicanCurrency;
use legalis_mx::labor_law::*;

fn main() {
    println!("=== Aguinaldo Calculator ===\n");

    // Daily salary: 300 pesos
    let daily_salary = MexicanCurrency::from_pesos(300);

    // Full year employee
    println!("Full year employee (365 days):");
    let aguinaldo_full = calculate_aguinaldo(daily_salary, 365);
    println!("  Daily salary: {} pesos", daily_salary.pesos());
    println!("  Aguinaldo: {} pesos", aguinaldo_full.pesos());
    println!("  (15 days minimum)\n");

    // 6-month employee
    println!("Six-month employee (182 days):");
    let aguinaldo_half = calculate_aguinaldo(daily_salary, 182);
    println!("  Daily salary: {} pesos", daily_salary.pesos());
    println!("  Aguinaldo: {} pesos", aguinaldo_half.pesos());
    println!("  (Proportional to days worked)\n");

    // 3-month employee
    println!("Three-month employee (91 days):");
    let aguinaldo_quarter = calculate_aguinaldo(daily_salary, 91);
    println!("  Daily salary: {} pesos", daily_salary.pesos());
    println!("  Aguinaldo: {} pesos", aguinaldo_quarter.pesos());
    println!("  (Proportional to days worked)");
}
