//! Employment Contract Validation Example
//!
//! This example demonstrates comprehensive Employment Act (Cap. 91) contract validation
//! for Singapore employment contracts, including:
//!
//! - EA coverage determination (workmen vs non-workmen, salary thresholds)
//! - Working hours compliance (s. 38)
//! - CPF applicability (citizens/PRs)
//! - Contract date validation
//! - Allowance validation
//!
//! ## Legal Context
//!
//! The Employment Act covers:
//! - **Workmen** earning ≤ SGD 4,500/month → Fully covered
//! - **Non-workmen** earning ≤ SGD 2,600/month → Fully covered
//! - **Earning SGD 2,601-4,500/month** → Partially covered
//! - **Earning > SGD 4,500/month** → Not covered
//!
//! ## Key Provisions
//!
//! - **s. 38**: Working hours (44h/week non-shift, 48h/week shift)
//! - **s. 38(4)**: Overtime at 1.5x minimum
//! - **s. 43**: Annual leave (7-14 days by service years)
//! - **s. 89**: Sick leave (14 outpatient + 60 hospitalization)

use chrono::Utc;
use legalis_sg::employment::*;

fn main() {
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║   SINGAPORE EMPLOYMENT ACT - CONTRACT VALIDATION EXAMPLES   ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // Scenario 1: Valid EA-covered contract (workman)
    println!("━━━ Scenario 1: Valid EA-Covered Contract (Workman) ━━━\n");

    let contract1 = EmploymentContract {
        employee_name: "Ahmad bin Hassan".to_string(),
        employer_name: "Construction Excellence Pte Ltd".to_string(),
        contract_type: ContractType::Indefinite,
        start_date: Utc::now(),
        end_date: None,
        basic_salary_cents: 400_000, // SGD 4,000/month
        allowances: vec![
            Allowance::new("Transport", 15_000, true), // SGD 150
            Allowance::new("Site", 10_000, true),      // SGD 100
        ],
        working_hours: WorkingHours {
            hours_per_day: 8.0,
            hours_per_week: 44.0,
            is_shift_work: false,
            rest_days_per_week: 1,
            overtime_eligible: true,
            working_days_per_week: 5,
        },
        leave_entitlement: LeaveEntitlement::new(0), // First year
        cpf_applicable: true,
        covered_by_ea: true, // Workman earning ≤ SGD 4,500
    };

    match validate_employment_contract(&contract1) {
        Ok(report) => {
            println!("✅ Contract Validation: PASSED");
            println!("\n📋 Contract Details:");
            println!("   Employee: {}", contract1.employee_name);
            println!("   Employer: {}", contract1.employer_name);
            println!(
                "   Basic Salary: SGD {:.2}/month",
                contract1.basic_salary_sgd()
            );
            println!(
                "   Total Salary: SGD {:.2}/month",
                contract1.total_monthly_salary_sgd()
            );
            println!("   Contract Type: {:?}", contract1.contract_type);
            println!("\n📊 EA Coverage:");
            println!(
                "   Covered by EA: {}",
                if report.ea_covered {
                    "Yes ✓"
                } else {
                    "No ✗"
                }
            );
            println!(
                "   CPF Applicable: {}",
                if report.cpf_applicable {
                    "Yes ✓"
                } else {
                    "No ✗"
                }
            );
            println!("\n⏰ Working Hours:");
            println!(
                "   Hours/Week: {} (limit: 44h for non-shift)",
                contract1.working_hours.hours_per_week
            );
            println!("   Hours/Day: {}", contract1.working_hours.hours_per_day);
            println!(
                "   Rest Days: {} day/week",
                contract1.working_hours.rest_days_per_week
            );

            if !report.warnings.is_empty() {
                println!("\n⚠️  Warnings:");
                for warning in &report.warnings {
                    println!("   • {}", warning);
                }
            }
        }
        Err(e) => println!("❌ Contract Validation Failed: {}", e),
    }

    // Scenario 2: Non-EA covered contract (high earner)
    println!("\n\n━━━ Scenario 2: Non-EA Covered Contract (High Earner) ━━━\n");

    let contract2 = EmploymentContract {
        employee_name: "Jane Tan Wei Ling".to_string(),
        employer_name: "Finance Hub Pte Ltd".to_string(),
        contract_type: ContractType::Indefinite,
        start_date: Utc::now(),
        end_date: None,
        basic_salary_cents: 800_000, // SGD 8,000/month
        allowances: vec![
            Allowance::new("Transport", 50_000, true), // SGD 500
            Allowance::new("Meal", 30_000, true),      // SGD 300
        ],
        working_hours: WorkingHours::standard(),
        leave_entitlement: LeaveEntitlement::new(3), // 3 years service
        cpf_applicable: true,
        covered_by_ea: false, // Earning > SGD 4,500
    };

    match validate_employment_contract(&contract2) {
        Ok(report) => {
            println!("✅ Contract Validation: PASSED");
            println!("\n📋 Contract Details:");
            println!("   Employee: {}", contract2.employee_name);
            println!(
                "   Basic Salary: SGD {:.2}/month",
                contract2.basic_salary_sgd()
            );
            println!(
                "   Total Salary: SGD {:.2}/month",
                contract2.total_monthly_salary_sgd()
            );
            println!("\n📊 EA Coverage:");
            println!(
                "   Covered by EA: {} (salary exceeds threshold)",
                if report.ea_covered {
                    "Yes ✓"
                } else {
                    "No ✗"
                }
            );
            println!(
                "   CPF Applicable: {}",
                if report.cpf_applicable {
                    "Yes ✓"
                } else {
                    "No ✗"
                }
            );
            println!("\n💡 Note: Not covered by EA but still subject to CPF contributions");

            if !report.warnings.is_empty() {
                println!("\n⚠️  Warnings:");
                for warning in &report.warnings {
                    println!("   • {}", warning);
                }
            }
        }
        Err(e) => println!("❌ Contract Validation Failed: {}", e),
    }

    // Scenario 3: Invalid contract (excessive working hours)
    println!("\n\n━━━ Scenario 3: Invalid Contract (Excessive Hours) ━━━\n");

    let contract3 = EmploymentContract {
        employee_name: "Kumar Rajesh".to_string(),
        employer_name: "Tech Startup Pte Ltd".to_string(),
        contract_type: ContractType::Indefinite,
        start_date: Utc::now(),
        end_date: None,
        basic_salary_cents: 350_000, // SGD 3,500/month
        allowances: vec![],
        working_hours: WorkingHours {
            hours_per_day: 10.0,
            hours_per_week: 60.0, // ❌ Exceeds 44h limit
            is_shift_work: false,
            rest_days_per_week: 1,
            overtime_eligible: true,
            working_days_per_week: 6,
        },
        leave_entitlement: LeaveEntitlement::new(0),
        cpf_applicable: true,
        covered_by_ea: true,
    };

    match validate_employment_contract(&contract3) {
        Ok(report) => {
            if report.is_valid {
                println!("✅ Contract Validation: PASSED");
            } else {
                println!("❌ Contract Validation: FAILED");
                println!("\n🚨 Errors:");
                for error in &report.errors {
                    println!("   • {}", error);
                }
            }
            println!("\n📋 Contract Details:");
            println!("   Employee: {}", contract3.employee_name);
            println!(
                "   Working Hours: {} hours/week",
                contract3.working_hours.hours_per_week
            );
            println!("   Limit: 44 hours/week (Employment Act s. 38)");
            println!("\n⚖️  Legal Violation:");
            println!("   This contract violates s. 38 of the Employment Act");
            println!("   Employer must reduce working hours or apply for exemption");
        }
        Err(e) => println!("❌ Contract Validation Failed: {}", e),
    }

    // Scenario 4: Fixed-term contract without end date (warning)
    println!("\n\n━━━ Scenario 4: Fixed-Term Contract Issues ━━━\n");

    let contract4 = EmploymentContract {
        employee_name: "Maria Santos".to_string(),
        employer_name: "Retail Shop Pte Ltd".to_string(),
        contract_type: ContractType::FixedTerm,
        start_date: Utc::now(),
        end_date: None,              // ⚠️ Missing for fixed-term
        basic_salary_cents: 250_000, // SGD 2,500/month
        allowances: vec![],
        working_hours: WorkingHours::standard(),
        leave_entitlement: LeaveEntitlement::new(0),
        cpf_applicable: false, // Non-citizen/PR
        covered_by_ea: true,
    };

    match validate_employment_contract(&contract4) {
        Ok(report) => {
            println!("✅ Contract Validation: PASSED (with warnings)");
            println!("\n📋 Contract Details:");
            println!("   Employee: {}", contract4.employee_name);
            println!("   Contract Type: Fixed-Term");
            println!("   End Date: Not specified ⚠️");
            println!("\n⚠️  Warnings:");
            for warning in &report.warnings {
                println!("   • {}", warning);
            }
            println!("\n📊 EA Coverage:");
            println!(
                "   Covered by EA: {}",
                if report.ea_covered {
                    "Yes ✓"
                } else {
                    "No ✗"
                }
            );
            println!(
                "   CPF Applicable: {} (non-citizen/PR)",
                if report.cpf_applicable {
                    "Yes ✓"
                } else {
                    "No ✗"
                }
            );
        }
        Err(e) => println!("❌ Contract Validation Failed: {}", e),
    }

    // Scenario 5: Part-time contract
    println!("\n\n━━━ Scenario 5: Part-Time Contract ━━━\n");

    let contract5 = EmploymentContract {
        employee_name: "Siti Nurhaliza".to_string(),
        employer_name: "Cafe Delight Pte Ltd".to_string(),
        contract_type: ContractType::PartTime,
        start_date: Utc::now(),
        end_date: None,
        basic_salary_cents: 180_000, // SGD 1,800/month
        allowances: vec![Allowance::new("Meal", 5_000, true)], // SGD 50
        working_hours: WorkingHours {
            hours_per_day: 5.0,
            hours_per_week: 25.0, // Part-time
            is_shift_work: false,
            rest_days_per_week: 2,
            overtime_eligible: false, // Part-time typically not eligible
            working_days_per_week: 5,
        },
        leave_entitlement: LeaveEntitlement::new(1), // 1 year service
        cpf_applicable: true,
        covered_by_ea: true,
    };

    match validate_employment_contract(&contract5) {
        Ok(report) => {
            println!("✅ Contract Validation: PASSED");
            println!("\n📋 Contract Details:");
            println!("   Employee: {}", contract5.employee_name);
            println!("   Contract Type: Part-Time");
            println!(
                "   Basic Salary: SGD {:.2}/month",
                contract5.basic_salary_sgd()
            );
            println!(
                "   Working Hours: {} hours/week",
                contract5.working_hours.hours_per_week
            );
            println!("\n📊 EA Coverage:");
            println!(
                "   Covered by EA: {}",
                if report.ea_covered {
                    "Yes ✓"
                } else {
                    "No ✗"
                }
            );
            println!(
                "   CPF Applicable: {}",
                if report.cpf_applicable {
                    "Yes ✓"
                } else {
                    "No ✗"
                }
            );
            println!("\n💡 Leave Entitlement:");
            println!(
                "   Annual Leave: {} days",
                contract5.leave_entitlement.annual_leave_days
            );
            println!(
                "   Sick Leave: {} outpatient + {} hospitalization",
                contract5.leave_entitlement.sick_leave_outpatient_days,
                contract5.leave_entitlement.sick_leave_hospitalization_days
            );
        }
        Err(e) => println!("❌ Contract Validation Failed: {}", e),
    }

    // Summary
    println!("\n\n╔══════════════════════════════════════════════════════════════╗");
    println!("║                        SUMMARY                               ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    println!("📚 Key Takeaways:");
    println!("\n1. EA Coverage Thresholds:");
    println!("   • Workmen: ≤ SGD 4,500/month → Fully covered");
    println!("   • Non-workmen: ≤ SGD 2,600/month → Fully covered");
    println!("   • SGD 2,601-4,500/month → Partially covered");
    println!("   • > SGD 4,500/month → Not covered");
    println!("\n2. Working Hours (s. 38):");
    println!("   • Non-shift: Max 44 hours/week");
    println!("   • Shift: Max 48 hours/week");
    println!("   • Daily max: 12 hours/day");
    println!("   • Rest days: Minimum 1 day/week");
    println!("\n3. CPF Contributions:");
    println!("   • Only applicable to Singapore citizens and PRs");
    println!("   • Wage ceiling: SGD 6,000/month");
    println!("   • Rates vary by age (17%/20% for age ≤55)");
    println!("\n4. Contract Types:");
    println!("   • Indefinite: Open-ended employment");
    println!("   • Fixed-term: Must specify end date");
    println!("   • Part-time: Prorated benefits");

    println!("\n\n📖 Next Steps:");
    println!("   1. Review Employment Act (Cap. 91) full text");
    println!("   2. Consult MOM website for latest updates");
    println!("   3. Consider industry-specific exemptions");
    println!("   4. Seek legal advice for complex cases\n");
}
