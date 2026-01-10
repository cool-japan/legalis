//! CPF Contribution Calculator Example
//!
//! This example demonstrates Central Provident Fund (CPF) contribution calculations
//! for Singapore employment, including:
//!
//! - Age-based contribution rates (employer + employee)
//! - Ordinary wage ceiling (SGD 6,000/month)
//! - CPF allocation to different accounts (OA, SA, MA, RA)
//! - Annual contribution projections
//!
//! ## Legal Context
//!
//! CPF is a mandatory retirement savings scheme for Singapore citizens and PRs:
//!
//! - **Wage Ceiling**: SGD 6,000/month (Ordinary Wage), SGD 102,000/year (Additional Wage)
//! - **Contribution Rates**: Vary by age (total 37% for age ≤55)
//! - **Employer**: 17% (age ≤55)
//! - **Employee**: 20% (age ≤55)
//!
//! ## CPF Accounts
//!
//! - **OA** (Ordinary Account): Housing, education, investment
//! - **SA** (Special Account): Retirement, investment
//! - **MA** (Medisave Account): Healthcare expenses
//! - **RA** (Retirement Account): Monthly payouts from age 65

use legalis_sg::employment::*;

fn main() {
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║      SINGAPORE CPF CONTRIBUTION CALCULATOR (2024 RATES)     ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // Age brackets for CPF rates
    let scenarios = vec![
        (25, 500_000, "Young Professional"),
        (30, 600_000, "Mid-Career (At Ceiling)"),
        (45, 800_000, "Senior Professional (Exceeds Ceiling)"),
        (58, 550_000, "Age 56-60 Bracket"),
        (63, 450_000, "Age 61-65 Bracket"),
        (68, 400_000, "Age 66-70 Bracket"),
        (72, 350_000, "Age 70+ Bracket"),
    ];

    println!("📊 CPF CONTRIBUTION BY AGE BRACKETS\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    for (age, monthly_wage_cents, description) in scenarios {
        calculate_and_display_cpf(age, monthly_wage_cents, description);
        println!();
    }

    // Detailed breakdown for age 30
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║          DETAILED CPF BREAKDOWN (Age 30, SGD 5,000)         ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    let cpf_detailed = CpfContribution::new(30, 500_000);

    println!("📋 Employee Profile:");
    println!("   Age: {} years", cpf_detailed.employee_age);
    println!(
        "   Monthly Wage: SGD {:.2}",
        cpf_detailed.monthly_wage_cents as f64 / 100.0
    );
    println!(
        "   Subject Wage: SGD {:.2} (after ceiling)",
        cpf_detailed.cpf_subject_wage_cents() as f64 / 100.0
    );

    println!("\n💰 Monthly Contributions:");
    println!("   Employer:");
    println!(
        "      Rate: {}% ({} bps)",
        cpf_detailed.employer_rate_bps as f64 / 100.0,
        cpf_detailed.employer_rate_bps
    );
    println!(
        "      Amount: SGD {:.2}",
        cpf_detailed.employer_contribution_sgd()
    );

    println!("\n   Employee:");
    println!(
        "      Rate: {}% ({} bps)",
        cpf_detailed.employee_rate_bps as f64 / 100.0,
        cpf_detailed.employee_rate_bps
    );
    println!(
        "      Amount: SGD {:.2}",
        cpf_detailed.employee_contribution_sgd()
    );

    println!("\n   Total:");
    println!(
        "      Rate: {}%",
        (cpf_detailed.employer_rate_bps + cpf_detailed.employee_rate_bps) as f64 / 100.0
    );
    println!(
        "      Amount: SGD {:.2}",
        cpf_detailed.total_contribution_sgd()
    );

    println!("\n📈 Annual Projection:");
    let annual_employer = cpf_detailed.employer_contribution_sgd() * 12.0;
    let annual_employee = cpf_detailed.employee_contribution_sgd() * 12.0;
    let annual_total = cpf_detailed.total_contribution_sgd() * 12.0;

    println!("   Employer (Annual): SGD {:.2}", annual_employer);
    println!("   Employee (Annual): SGD {:.2}", annual_employee);
    println!("   Total (Annual): SGD {:.2}", annual_total);

    // CPF account allocation (approximate for age ≤55)
    println!("\n📊 CPF Account Allocation (Age ≤55):");
    println!("   (Approximate allocation percentages)");
    println!();
    println!(
        "   Ordinary Account (OA):    ~62% → SGD {:.2}/month",
        cpf_detailed.total_contribution_sgd() * 0.62
    );
    println!("      → Housing, education, approved investments");
    println!();
    println!(
        "   Special Account (SA):     ~17% → SGD {:.2}/month",
        cpf_detailed.total_contribution_sgd() * 0.17
    );
    println!("      → Retirement savings");
    println!();
    println!(
        "   Medisave Account (MA):    ~21% → SGD {:.2}/month",
        cpf_detailed.total_contribution_sgd() * 0.21
    );
    println!("      → Healthcare expenses");

    // Wage ceiling demonstration
    println!("\n\n╔══════════════════════════════════════════════════════════════╗");
    println!("║              WAGE CEILING DEMONSTRATION (2024)               ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    let wage_ceiling_scenarios = vec![
        400_000,   // SGD 4,000
        600_000,   // SGD 6,000 (exactly at ceiling)
        800_000,   // SGD 8,000 (exceeds ceiling)
        1_000_000, // SGD 10,000 (well above ceiling)
    ];

    println!("Ordinary Wage Ceiling: SGD 6,000/month\n");
    println!("┌─────────────┬─────────────┬──────────────┬──────────────┐");
    println!("│ Monthly Wage│ Subject Wage│ Employer CPF │ Employee CPF │");
    println!("├─────────────┼─────────────┼──────────────┼──────────────┤");

    for wage_cents in wage_ceiling_scenarios {
        let cpf = CpfContribution::new(30, wage_cents);
        println!(
            "│ SGD {:>7.2} │ SGD {:>7.2} │ SGD {:>8.2} │ SGD {:>8.2} │",
            wage_cents as f64 / 100.0,
            cpf.cpf_subject_wage_cents() as f64 / 100.0,
            cpf.employer_contribution_sgd(),
            cpf.employee_contribution_sgd()
        );
    }
    println!("└─────────────┴─────────────┴──────────────┴──────────────┘");

    println!("\n💡 Note: Wages above SGD 6,000/month are capped at the ceiling.");
    println!("    Actual wage: SGD 10,000 → CPF calculated on: SGD 6,000");

    // Complete rate table
    println!("\n\n╔══════════════════════════════════════════════════════════════╗");
    println!("║         COMPLETE CPF CONTRIBUTION RATE TABLE (2024)         ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    println!("┌────────────┬──────────────┬──────────────┬──────────────┐");
    println!("│  Age Group │ Employer (%) │ Employee (%) │   Total (%)  │");
    println!("├────────────┼──────────────┼──────────────┼──────────────┤");

    let rate_table = vec![
        ("≤ 55 years", 1700, 2000),
        ("56-60 years", 1550, 1500),
        ("61-65 years", 1150, 950),
        ("66-70 years", 900, 750),
        ("> 70 years", 750, 500),
    ];

    for (age_group, employer_bps, employee_bps) in rate_table {
        let employer_pct = employer_bps as f64 / 100.0;
        let employee_pct = employee_bps as f64 / 100.0;
        let total_pct = (employer_bps + employee_bps) as f64 / 100.0;

        println!(
            "│ {:>10} │    {:>6.1}%   │    {:>6.1}%   │    {:>6.1}%   │",
            age_group, employer_pct, employee_pct, total_pct
        );
    }
    println!("└────────────┴──────────────┴──────────────┴──────────────┘");

    // Validation example
    println!("\n\n╔══════════════════════════════════════════════════════════════╗");
    println!("║                  CPF VALIDATION EXAMPLE                      ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    let cpf_to_validate = CpfContribution::new(35, 550_000);

    println!("Validating CPF calculation for:");
    println!("   Age: {} years", cpf_to_validate.employee_age);
    println!(
        "   Monthly Wage: SGD {:.2}",
        cpf_to_validate.monthly_wage_cents as f64 / 100.0
    );

    match validate_cpf_calculation(&cpf_to_validate) {
        Ok(()) => {
            println!("\n✅ CPF Calculation: VALID");
            println!("\n   Rates match statutory requirements:");
            println!(
                "   • Employer: {}% ✓",
                cpf_to_validate.employer_rate_bps as f64 / 100.0
            );
            println!(
                "   • Employee: {}% ✓",
                cpf_to_validate.employee_rate_bps as f64 / 100.0
            );
            println!("   • Wage ceiling correctly applied ✓");
        }
        Err(e) => {
            println!("\n❌ CPF Calculation: INVALID");
            println!("   Error: {}", e);
        }
    }

    // Comparison: Before and after age 55
    println!("\n\n╔══════════════════════════════════════════════════════════════╗");
    println!("║      RETIREMENT AGE IMPACT (Before/After Age 55)            ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    let wage = 500_000; // SGD 5,000

    let cpf_before = CpfContribution::new(54, wage);
    let cpf_after = CpfContribution::new(56, wage);

    println!("Monthly Wage: SGD {:.2}\n", wage as f64 / 100.0);

    println!("Age 54 (Before 55):");
    println!(
        "   Employer: {}% → SGD {:.2}/month",
        cpf_before.employer_rate_bps as f64 / 100.0,
        cpf_before.employer_contribution_sgd()
    );
    println!(
        "   Employee: {}% → SGD {:.2}/month",
        cpf_before.employee_rate_bps as f64 / 100.0,
        cpf_before.employee_contribution_sgd()
    );
    println!(
        "   Total: {}% → SGD {:.2}/month",
        (cpf_before.employer_rate_bps + cpf_before.employee_rate_bps) as f64 / 100.0,
        cpf_before.total_contribution_sgd()
    );

    println!("\nAge 56 (After 55):");
    println!(
        "   Employer: {}% → SGD {:.2}/month",
        cpf_after.employer_rate_bps as f64 / 100.0,
        cpf_after.employer_contribution_sgd()
    );
    println!(
        "   Employee: {}% → SGD {:.2}/month",
        cpf_after.employee_rate_bps as f64 / 100.0,
        cpf_after.employee_contribution_sgd()
    );
    println!(
        "   Total: {}% → SGD {:.2}/month",
        (cpf_after.employer_rate_bps + cpf_after.employee_rate_bps) as f64 / 100.0,
        cpf_after.total_contribution_sgd()
    );

    let reduction = cpf_before.total_contribution_sgd() - cpf_after.total_contribution_sgd();
    let reduction_pct = (reduction / cpf_before.total_contribution_sgd()) * 100.0;

    println!("\n📉 Impact:");
    println!(
        "   Monthly Reduction: SGD {:.2} ({:.1}%)",
        reduction, reduction_pct
    );
    println!("   Annual Reduction: SGD {:.2}", reduction * 12.0);

    // Summary
    println!("\n\n╔══════════════════════════════════════════════════════════════╗");
    println!("║                        SUMMARY                               ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    println!("📚 Key Takeaways:");
    println!("\n1. CPF Contribution Rates:");
    println!("   • Highest: Age ≤55 (37% total)");
    println!("   • Gradually decrease with age");
    println!("   • Lowest: Age >70 (12.5% total)");
    println!("\n2. Wage Ceiling:");
    println!("   • Ordinary Wage: SGD 6,000/month");
    println!("   • Additional Wage: SGD 102,000/year");
    println!("   • No CPF on amounts above ceiling");
    println!("\n3. Applicability:");
    println!("   • Singapore citizens and PRs only");
    println!("   • Not applicable to foreigners on work passes");
    println!("\n4. Account Allocation:");
    println!("   • Changes with age");
    println!("   • More to OA when younger");
    println!("   • More to MA/RA when older");

    println!("\n\n📖 Resources:");
    println!("   • CPF Board: https://www.cpf.gov.sg/");
    println!("   • Employer Guide: https://www.cpf.gov.sg/employer");
    println!("   • Contribution Calculator: https://www.cpf.gov.sg/member/tools-and-services\n");
}

fn calculate_and_display_cpf(age: u32, monthly_wage_cents: u64, description: &str) {
    let cpf = CpfContribution::new(age, monthly_wage_cents);

    let wage_sgd = monthly_wage_cents as f64 / 100.0;
    let subject_wage_sgd = cpf.cpf_subject_wage_cents() as f64 / 100.0;
    let is_capped = monthly_wage_cents > CpfContribution::ORDINARY_WAGE_CEILING_CENTS;

    println!("━━━ {} (Age: {}) ━━━", description, age);
    println!(
        "   Monthly Wage: SGD {:.2}{}",
        wage_sgd,
        if is_capped { " (exceeds ceiling)" } else { "" }
    );
    if is_capped {
        println!("   Subject Wage: SGD {:.2} (capped)", subject_wage_sgd);
    }
    println!(
        "   Employer: {}% → SGD {:.2}/month",
        cpf.employer_rate_bps as f64 / 100.0,
        cpf.employer_contribution_sgd()
    );
    println!(
        "   Employee: {}% → SGD {:.2}/month",
        cpf.employee_rate_bps as f64 / 100.0,
        cpf.employee_contribution_sgd()
    );
    println!(
        "   Total:    {}% → SGD {:.2}/month",
        (cpf.employer_rate_bps + cpf.employee_rate_bps) as f64 / 100.0,
        cpf.total_contribution_sgd()
    );
}
