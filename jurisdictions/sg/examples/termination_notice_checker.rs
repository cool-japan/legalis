//! Termination Notice Checker Example
//!
//! This example demonstrates Employment Act termination notice requirements including:
//!
//! - Notice period calculation by service length (s. 10/11)
//! - Payment in lieu of notice
//! - Notice date vs effective date calculation
//! - Employer vs employee termination
//! - Validation of notice compliance
//!
//! ## Legal Context
//!
//! ### Notice Periods (Employment Act s. 10/11)
//!
//! | Service Length    | Notice Period  |
//! |-------------------|----------------|
//! | < 26 weeks        | 1 day          |
//! | 26 weeks-2 years  | 1 week (7 days)|
//! | 2 years-5 years   | 2 weeks (14 days)|
//! | 5+ years          | 4 weeks (28 days)|
//!
//! ### Key Points
//!
//! - Notice can be waived by mutual agreement
//! - Payment in lieu allowed (s. 11)
//! - Notice during probation: Usually shorter or none
//! - Summary dismissal: No notice (for misconduct)

use chrono::Utc;
use legalis_sg::employment::*;

fn main() {
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║   SINGAPORE EMPLOYMENT ACT - TERMINATION NOTICE CHECKER     ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // Notice period requirements table
    println!("📋 NOTICE PERIOD REQUIREMENTS (Employment Act s. 10/11)\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("┌──────────────────────┬────────────────┬─────────────────────┐");
    println!("│ Service Length       │ Service Weeks  │ Notice Period       │");
    println!("├──────────────────────┼────────────────┼─────────────────────┤");

    let notice_examples = vec![
        ("< 26 weeks", 20, 1),
        ("26 weeks - 2 years", 52, 7),
        ("2 years - 5 years", 156, 14),
        ("5+ years", 260, 28),
    ];

    for (description, weeks, days) in notice_examples {
        let required_days = TerminationNotice::required_notice_days(weeks);
        println!(
            "│ {:>20} │ {:>14} │ {:>19} │",
            description,
            weeks,
            format!("{} days", required_days)
        );
        assert_eq!(
            required_days, days,
            "Notice period mismatch for {}",
            description
        );
    }
    println!("└──────────────────────┴────────────────┴─────────────────────┘");

    println!("\n💡 Note: Notice periods are statutory minimums.");
    println!("    Employment contracts may specify longer notice periods.");

    // Scenario 1: Valid resignation (2 years service)
    println!("\n\n╔══════════════════════════════════════════════════════════════╗");
    println!("║         SCENARIO 1: Employee Resignation (2 Years)          ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    let service_weeks_1 = 104; // 2 years
    let notice_days_1 = 14; // 2 weeks

    println!("📋 Termination Details:");
    println!(
        "   Service Length: {} weeks ({:.1} years)",
        service_weeks_1,
        service_weeks_1 as f64 / 52.0
    );
    println!(
        "   Required Notice: {} days (s. 10/11)",
        TerminationNotice::required_notice_days(service_weeks_1)
    );
    println!("   Given Notice: {} days", notice_days_1);
    println!("   Terminating Party: Employee (Resignation)");

    match validate_termination_notice(service_weeks_1, notice_days_1) {
        Ok(()) => {
            println!("\n✅ Notice Period: VALID");
            println!("   Notice meets statutory requirement ✓");

            let notice_date = Utc::now();
            let effective_date = calculate_last_employment_day(notice_date, notice_days_1);

            println!("\n📅 Timeline:");
            println!("   Notice Date: {}", notice_date.format("%Y-%m-%d"));
            println!("   Notice Period: {} days", notice_days_1);
            println!("   Last Working Day: {}", effective_date.format("%Y-%m-%d"));
            println!(
                "   (Employee works {} more days including notice period)",
                notice_days_1
            );
        }
        Err(e) => {
            println!("\n❌ Notice Period: INVALID");
            println!("   Error: {}", e);
        }
    }

    // Scenario 2: Insufficient notice
    println!("\n\n╔══════════════════════════════════════════════════════════════╗");
    println!("║      SCENARIO 2: Insufficient Notice (5 Years Service)      ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    let service_weeks_2 = 260; // 5 years
    let notice_days_2 = 14; // Only 2 weeks (should be 4)

    println!("📋 Termination Details:");
    println!(
        "   Service Length: {} weeks ({} years)",
        service_weeks_2,
        service_weeks_2 / 52
    );
    println!(
        "   Required Notice: {} days (4 weeks)",
        TerminationNotice::required_notice_days(service_weeks_2)
    );
    println!("   Given Notice: {} days (2 weeks)", notice_days_2);
    println!(
        "   Shortfall: {} days",
        TerminationNotice::required_notice_days(service_weeks_2) - notice_days_2
    );

    match validate_termination_notice(service_weeks_2, notice_days_2) {
        Ok(()) => {
            println!("\n✅ Notice Period: VALID");
        }
        Err(e) => {
            println!("\n❌ Notice Period: INVALID");
            println!("   Error: {}", e);
            println!("\n⚖️  Legal Implications:");
            println!("   • Employer: Must pay salary in lieu for shortfall");
            println!("   • Employee: May forfeit salary for notice not served");
            println!(
                "   • Shortfall: {} days must be compensated",
                TerminationNotice::required_notice_days(service_weeks_2) - notice_days_2
            );
        }
    }

    // Scenario 3: Payment in lieu of notice
    println!("\n\n╔══════════════════════════════════════════════════════════════╗");
    println!("║         SCENARIO 3: Payment in Lieu (Immediate Exit)        ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    let service_weeks_3 = 156; // 3 years
    let required_notice_3 = TerminationNotice::required_notice_days(service_weeks_3);
    let daily_salary = 20_000_u64; // SGD 200/day in cents

    println!("📋 Termination Details:");
    println!("   Service Length: {} weeks (3 years)", service_weeks_3);
    println!("   Required Notice: {} days", required_notice_3);
    println!("   Actual Notice: 0 days (Immediate termination)");
    println!("   Payment in Lieu: Yes");

    println!("\n💰 Payment Calculation:");
    println!("   Daily Salary: SGD {:.2}", daily_salary as f64 / 100.0);
    println!("   Notice Days: {}", required_notice_3);
    let payment_in_lieu = daily_salary * required_notice_3 as u64;
    println!(
        "   Payment in Lieu: SGD {:.2}",
        payment_in_lieu as f64 / 100.0
    );
    println!(
        "   (SGD {:.2} × {} days)",
        daily_salary as f64 / 100.0,
        required_notice_3
    );

    println!("\n📅 Timeline:");
    let termination_date = Utc::now();
    println!(
        "   Termination Notice: {}",
        termination_date.format("%Y-%m-%d")
    );
    println!(
        "   Last Working Day: {} (Same day - immediate)",
        termination_date.format("%Y-%m-%d")
    );
    println!("   Payment Date: Within 7 days (MOM guideline)");

    // Scenario 4: New employee (< 26 weeks)
    println!("\n\n╔══════════════════════════════════════════════════════════════╗");
    println!("║            SCENARIO 4: Probation Period (3 Months)          ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    let service_weeks_4 = 12; // 3 months
    let notice_days_4 = 1;

    println!("📋 Termination Details:");
    println!("   Service Length: {} weeks (3 months)", service_weeks_4);
    println!(
        "   Required Notice: {} day (s. 10)",
        TerminationNotice::required_notice_days(service_weeks_4)
    );
    println!("   Given Notice: {} day", notice_days_4);
    println!("   Status: Probation period");

    match validate_termination_notice(service_weeks_4, notice_days_4) {
        Ok(()) => {
            println!("\n✅ Notice Period: VALID");
            println!("   Minimum 1-day notice during probation ✓");
            println!("\n💡 Note: Many contracts waive notice during probation.");
            println!("    Check employment contract for specific terms.");
        }
        Err(e) => {
            println!("\n❌ Notice Period: INVALID");
            println!("   Error: {}", e);
        }
    }

    // Comprehensive notice calculator
    println!("\n\n╔══════════════════════════════════════════════════════════════╗");
    println!("║            COMPREHENSIVE NOTICE CALCULATOR                   ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    println!("Calculate required notice for any service length:\n");

    let test_scenarios = vec![
        (10, "2.5 months"),
        (26, "6 months (threshold)"),
        (52, "1 year"),
        (104, "2 years (threshold)"),
        (130, "2.5 years"),
        (260, "5 years (threshold)"),
        (520, "10 years"),
    ];

    println!("┌──────────────┬────────────────┬────────────────────────────┐");
    println!("│ Service      │ Required       │ Calculation Basis          │");
    println!("│ (Weeks)      │ Notice (Days)  │                            │");
    println!("├──────────────┼────────────────┼────────────────────────────┤");

    for (weeks, desc) in test_scenarios {
        let notice_days = TerminationNotice::required_notice_days(weeks);
        let basis = if weeks < 26 {
            "< 26 weeks → 1 day"
        } else if weeks < 104 {
            "26 weeks-2 yrs → 1 week"
        } else if weeks < 260 {
            "2-5 years → 2 weeks"
        } else {
            "5+ years → 4 weeks"
        };

        println!(
            "│ {:>12} │ {:>14} │ {:>26} │",
            format!("{} ({})", weeks, desc),
            notice_days,
            basis
        );
    }
    println!("└──────────────┴────────────────┴────────────────────────────┘");

    // Notice date calculation examples
    println!("\n\n╔══════════════════════════════════════════════════════════════╗");
    println!("║              NOTICE DATE CALCULATIONS                        ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    let today = Utc::now();

    println!("If notice given today ({}):\n", today.format("%Y-%m-%d"));

    let notice_periods = vec![1, 7, 14, 28];

    for days in notice_periods {
        let last_day = calculate_last_employment_day(today, days);
        let weeks = days / 7;
        let extra_days = days % 7;
        let period_desc = if weeks > 0 && extra_days > 0 {
            format!("{} week(s) {} day(s)", weeks, extra_days)
        } else if weeks > 0 {
            format!("{} week(s)", weeks)
        } else {
            format!("{} day(s)", days)
        };

        println!("   {} days notice ({}):", days, period_desc);
        println!(
            "      → Last working day: {}",
            last_day.format("%Y-%m-%d %A")
        );
        println!();
    }

    // Employer vs Employee termination
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║          EMPLOYER vs EMPLOYEE TERMINATION                    ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    println!("┌─────────────────────┬──────────────────────────────────────┐");
    println!("│ Terminating Party   │ Considerations                       │");
    println!("├─────────────────────┼──────────────────────────────────────┤");
    println!("│ Employer            │ • Must give proper notice or pay     │");
    println!("│                     │ • Cannot terminate without cause     │");
    println!("│                     │ • Must provide termination letter    │");
    println!("│                     │ • Consider unfair dismissal risk     │");
    println!("├─────────────────────┼──────────────────────────────────────┤");
    println!("│ Employee            │ • Must give notice per contract      │");
    println!("│ (Resignation)       │ • Employer may waive notice          │");
    println!("│                     │ • May forfeit pay for unserved notice│");
    println!("│                     │ • Resignation letter required        │");
    println!("└─────────────────────┴──────────────────────────────────────┘");

    // Summary
    println!("\n\n╔══════════════════════════════════════════════════════════════╗");
    println!("║                        SUMMARY                               ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    println!("📚 Key Takeaways:");
    println!("\n1. Notice Period Requirements (s. 10/11):");
    println!("   • < 26 weeks: 1 day");
    println!("   • 26 weeks-2 years: 1 week (7 days)");
    println!("   • 2-5 years: 2 weeks (14 days)");
    println!("   • 5+ years: 4 weeks (28 days)");
    println!("\n2. Payment in Lieu (s. 11):");
    println!("   • Allowed instead of working notice");
    println!("   • Calculate based on gross salary");
    println!("   • Payment within 7 days (MOM guideline)");
    println!("\n3. Notice Calculation:");
    println!("   • Based on length of service");
    println!("   • Calendar days, not working days");
    println!("   • Starts from day after notice given");
    println!("\n4. Special Cases:");
    println!("   • Probation: Usually shorter/no notice");
    println!("   • Misconduct: Summary dismissal (no notice)");
    println!("   • Mutual agreement: Notice can be waived");
    println!("   • Fixed-term: May specify different terms");

    println!("\n\n⚠️  Important Notes:");
    println!("   • Statutory notice is MINIMUM requirement");
    println!("   • Employment contract may require longer notice");
    println!("   • Always check contract terms");
    println!("   • Document all notice communications");
    println!("   • Seek MOM/legal advice for disputes");

    println!("\n\n📖 Resources:");
    println!("   • Employment Act s. 10-11: Termination provisions");
    println!("   • MOM Termination Guide: https://www.mom.gov.sg/employment-practices/termination");
    println!("   • TADM: Tripartite Alliance for Dispute Management\n");
}
