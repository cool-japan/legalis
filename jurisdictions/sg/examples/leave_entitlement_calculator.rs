//! Leave Entitlement Calculator Example
//!
//! This example demonstrates Employment Act leave entitlement calculations including:
//!
//! - Annual leave progression (s. 43): 7→14 days by service years
//! - Sick leave entitlements (s. 89): Outpatient + hospitalization
//! - Maternity leave (CDCA): 16 weeks for citizens
//! - Prorated leave for partial years
//! - Leave validation and compliance checking
//!
//! ## Legal Context
//!
//! ### Annual Leave (Employment Act s. 43)
//! - Year 1: 7 days
//! - Year 2: 8 days
//! - Years 3-4: 9 days
//! - Years 5-6: 11 days
//! - Years 7-8: 12 days
//! - Year 8+: 14 days
//!
//! ### Sick Leave (Employment Act s. 89)
//! - After 3 months: 14 days outpatient + 60 days hospitalization
//! - After 6 months: Full entitlement
//!
//! ### Maternity Leave (Child Development Co-Savings Act)
//! - 16 weeks for Singapore citizens
//! - First 4 weeks: Employer-paid
//! - Remaining 12 weeks: Government-paid

use legalis_sg::employment::*;

fn main() {
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║    SINGAPORE EMPLOYMENT ACT - LEAVE ENTITLEMENT CALCULATOR  ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // Annual leave progression by years of service
    println!("📅 ANNUAL LEAVE PROGRESSION (Employment Act s. 43)\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    println!("┌────────────────────┬─────────────────────┬─────────────────┐");
    println!("│ Years of Service   │ Annual Leave (Days) │ Status          │");
    println!("├────────────────────┼─────────────────────┼─────────────────┤");

    for years in 0..=10 {
        let leave = LeaveEntitlement::new(years);
        let status = if years == 0 {
            "First year"
        } else if years < 8 {
            "Increasing"
        } else {
            "Maximum"
        };

        println!(
            "│ {:>18} │ {:>19} │ {:>15} │",
            format!("Year {}", years + 1),
            leave.annual_leave_days,
            status
        );
    }
    println!("└────────────────────┴─────────────────────┴─────────────────┘");

    println!("\n💡 Note: Leave entitlement increases progressively with service years.");
    println!("    Maximum of 14 days reached at 8 years of service.");

    // Sick leave entitlements
    println!("\n\n🏥 SICK LEAVE ENTITLEMENT (Employment Act s. 89)\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let leave_standard = LeaveEntitlement::new(1);

    println!("Standard Sick Leave (After 3 months of service):");
    println!(
        "   Outpatient Leave: {} days/year",
        leave_standard.sick_leave_outpatient_days
    );
    println!(
        "   Hospitalization Leave: {} days/year",
        leave_standard.sick_leave_hospitalization_days
    );
    println!(
        "   Total: {} days/year",
        leave_standard.sick_leave_outpatient_days + leave_standard.sick_leave_hospitalization_days
    );

    println!("\n📋 Requirements:");
    println!("   • Must be certified by a medical practitioner");
    println!("   • Employer may require medical certificate");
    println!("   • Hospitalization leave: Only when hospitalized");

    // Maternity leave
    println!("\n\n👶 MATERNITY LEAVE (Child Development Co-Savings Act)\n");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let leave_with_maternity = LeaveEntitlement::new(2);

    println!("Maternity Protection for Singapore Citizens:");
    println!(
        "   Total Duration: {} weeks",
        leave_with_maternity.maternity_leave_weeks.unwrap_or(0)
    );
    println!();
    println!("   Breakdown:");
    println!("   • First 4 weeks: Employer-paid");
    println!("   • Next 12 weeks: Government-paid");
    println!();
    println!("   Eligibility:");
    println!("   • Singapore citizen mother");
    println!("   • Child is Singapore citizen");
    println!("   • Employed at least 3 months");

    // Detailed scenario: New employee
    println!("\n\n╔══════════════════════════════════════════════════════════════╗");
    println!("║               SCENARIO 1: New Employee (Year 1)             ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    let years_service = 0;
    let contract_type = ContractType::Indefinite;

    match validate_leave_entitlement(years_service, contract_type) {
        Ok(leave) => {
            println!("✅ Leave Entitlement Validation: PASSED");
            println!("\n📋 Employee Profile:");
            println!("   Years of Service: {} (First year)", years_service + 1);
            println!("   Contract Type: {:?}", contract_type);
            println!("\n📅 Leave Entitlements:");
            println!("   Annual Leave: {} days", leave.annual_leave_days);
            println!(
                "   Sick Leave (Outpatient): {} days",
                leave.sick_leave_outpatient_days
            );
            println!(
                "   Sick Leave (Hospitalization): {} days",
                leave.sick_leave_hospitalization_days
            );
            if let Some(maternity_weeks) = leave.maternity_leave_weeks {
                println!(
                    "   Maternity Leave: {} weeks (if eligible)",
                    maternity_weeks
                );
            }
            println!("\n💡 Note: First year employees get 7 days annual leave.");
        }
        Err(e) => {
            println!("❌ Leave Entitlement Validation Failed: {}", e);
        }
    }

    // Detailed scenario: Senior employee
    println!("\n\n╔══════════════════════════════════════════════════════════════╗");
    println!("║          SCENARIO 2: Senior Employee (10 Years)             ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    let senior_years = 9; // 10 years service (0-indexed)

    match validate_leave_entitlement(senior_years, ContractType::Indefinite) {
        Ok(leave) => {
            println!("✅ Leave Entitlement Validation: PASSED");
            println!("\n📋 Employee Profile:");
            println!("   Years of Service: {} years", senior_years + 1);
            println!("   Contract Type: Indefinite (Permanent)");
            println!("\n📅 Leave Entitlements:");
            println!(
                "   Annual Leave: {} days (Maximum)",
                leave.annual_leave_days
            );
            println!(
                "   Sick Leave (Outpatient): {} days",
                leave.sick_leave_outpatient_days
            );
            println!(
                "   Sick Leave (Hospitalization): {} days",
                leave.sick_leave_hospitalization_days
            );
            println!("\n📊 Total Annual Leave Days:");
            let total = leave.annual_leave_days
                + leave.sick_leave_outpatient_days
                + leave.sick_leave_hospitalization_days;
            println!("   Combined: {} days", total);
            println!(
                "   (Annual: {} + Sick Outpatient: {} + Sick Hosp: {})",
                leave.annual_leave_days,
                leave.sick_leave_outpatient_days,
                leave.sick_leave_hospitalization_days
            );
        }
        Err(e) => {
            println!("❌ Leave Entitlement Validation Failed: {}", e);
        }
    }

    // Prorated leave calculation
    println!("\n\n╔══════════════════════════════════════════════════════════════╗");
    println!("║              PRORATED LEAVE (Mid-Year Scenarios)            ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    println!("For employees who join mid-year or leave before year-end:\n");

    let annual_entitlement = 14; // Senior employee entitlement

    println!("┌──────────────────┬──────────────────┬─────────────────────┐");
    println!("│ Months Worked    │ Annual (14 days) │ Prorated Leave      │");
    println!("├──────────────────┼──────────────────┼─────────────────────┤");

    for months in [3, 6, 9, 12].iter() {
        let prorated = calculate_prorated_leave(annual_entitlement, *months);
        println!(
            "│ {:>16} │ {:>16} │ {:>19} │",
            format!("{} months", months),
            "14 days",
            format!("{} days", prorated)
        );
    }
    println!("└──────────────────┴──────────────────┴─────────────────────┘");

    println!("\n💡 Calculation: (Annual entitlement × Months worked) / 12");
    println!("   Example: 14 days × 6 months / 12 = 7 days");

    // Leave type comparison
    println!("\n\n╔══════════════════════════════════════════════════════════════╗");
    println!("║                  LEAVE TYPE COMPARISON                       ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    println!("┌─────────────────┬────────────┬─────────────────────────────┐");
    println!("│ Leave Type      │ Days/Weeks │ Key Requirements            │");
    println!("├─────────────────┼────────────┼─────────────────────────────┤");
    println!("│ Annual Leave    │ 7-14 days  │ By service years (s. 43)    │");
    println!("│ Sick (Outpt)    │ 14 days    │ Medical cert (s. 89)        │");
    println!("│ Sick (Hosp)     │ 60 days    │ Hospitalization proof       │");
    println!("│ Maternity       │ 16 weeks   │ Citizen, 3 months service   │");
    println!("│ Paternity       │ 2 weeks    │ Shared parental (CDCA)      │");
    println!("│ Childcare       │ 6 days     │ Child <7 years              │");
    println!("└─────────────────┴────────────┴─────────────────────────────┘");

    // Leave accrual patterns
    println!("\n\n╔══════════════════════════════════════════════════════════════╗");
    println!("║              ANNUAL LEAVE ACCRUAL PATTERNS                   ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    println!("How annual leave increases over employment tenure:\n");

    println!("Year 1:       ■■■■■■■ (7 days)");
    println!("Year 2:       ■■■■■■■■ (8 days)");
    println!("Years 3-4:    ■■■■■■■■■ (9 days)");
    println!("Years 5-6:    ■■■■■■■■■■■ (11 days)");
    println!("Years 7-8:    ■■■■■■■■■■■■ (12 days)");
    println!("Year 8+:      ■■■■■■■■■■■■■■ (14 days - Maximum)");

    println!("\n📈 Progression:");
    let years_to_track = vec![0, 1, 2, 4, 6, 8, 10];
    for &year in &years_to_track {
        let leave = LeaveEntitlement::new(year);
        let increase = if year > 0 {
            let prev_leave = LeaveEntitlement::new(year - 1);
            format!(
                " (+{})",
                leave.annual_leave_days - prev_leave.annual_leave_days
            )
        } else {
            String::new()
        };
        println!(
            "   Year {}: {} days{}",
            year + 1,
            leave.annual_leave_days,
            increase
        );
    }

    // Validation example
    println!("\n\n╔══════════════════════════════════════════════════════════════╗");
    println!("║              LEAVE ENTITLEMENT VALIDATION                    ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    println!("━━━ Valid Case: 5 Years Service ━━━\n");

    match validate_leave_entitlement(4, ContractType::Indefinite) {
        Ok(leave) => {
            println!("✅ Validation: PASSED");
            println!("   Service: 5 years");
            println!("   Statutory Minimum: 11 days (s. 43)");
            println!("   Actual Entitlement: {} days ✓", leave.annual_leave_days);
        }
        Err(e) => println!("❌ Validation Failed: {}", e),
    }

    println!("\n━━━ Invalid Case: Fixed-term < 3 months ━━━\n");

    match validate_leave_entitlement(0, ContractType::FixedTerm) {
        Ok(leave) => {
            println!("✅ Validation: PASSED");
            println!("   Leave days: {}", leave.annual_leave_days);
        }
        Err(e) => {
            println!("❌ Validation: FAILED");
            println!("   Error: {}", e);
            println!("\n   💡 Reason: Fixed-term contracts < 3 months may have");
            println!("      prorated leave instead of standard entitlement.");
        }
    }

    // Summary
    println!("\n\n╔══════════════════════════════════════════════════════════════╗");
    println!("║                        SUMMARY                               ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    println!("📚 Key Takeaways:");
    println!("\n1. Annual Leave (s. 43):");
    println!("   • Starts at 7 days in year 1");
    println!("   • Increases progressively to 14 days at year 8+");
    println!("   • Statutory minimum enforced by law");
    println!("\n2. Sick Leave (s. 89):");
    println!("   • 14 days outpatient (with MC)");
    println!("   • 60 days hospitalization");
    println!("   • Requires medical certification");
    println!("\n3. Maternity Leave (CDCA):");
    println!("   • 16 weeks for citizen mothers");
    println!("   • First 4 weeks employer-paid");
    println!("   • Remaining 12 weeks government-paid");
    println!("\n4. Prorated Leave:");
    println!("   • For mid-year joiners/leavers");
    println!("   • Calculated monthly: (Annual × Months) / 12");
    println!("   • Minimum 1 day for any partial service");

    println!("\n\n📖 Best Practices:");
    println!("   1. Track leave accrual monthly");
    println!("   2. Communicate entitlements clearly to employees");
    println!("   3. Maintain proper leave records (MOM requirement)");
    println!("   4. Review and update annually on employment anniversary");
    println!("   5. Consider company policy for leave carry-forward\n");

    println!("📖 Resources:");
    println!("   • Employment Act: https://sso.agc.gov.sg/Act/EmPA1968");
    println!("   • MOM Leave Guide: https://www.mom.gov.sg/employment-practices/leave");
    println!(
        "   • Leave Calculator: https://www.mom.gov.sg/employment-practices/leave/annual-leave\n"
    );
}
