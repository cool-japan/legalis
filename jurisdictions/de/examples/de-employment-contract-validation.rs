//! Employment Contract Validation Example
//!
//! Demonstrates validation of employment contracts under German labor law,
//! covering written form requirements (§2 NachwG), probation periods (§622 BGB),
//! working hours (ArbZG), and contract types.

use chrono::NaiveDate;
use legalis_de::arbeitsrecht::*;
use legalis_de::gmbhg::Capital;

fn main() {
    println!("=== German Employment Contract Validation ===\n");
    println!("Arbeitsvertragsvalidierung nach deutschem Arbeitsrecht\n");

    // Example 1: Valid Unlimited Employment Contract
    println!("📋 Example 1: Valid Unlimited Contract (Unbefristeter Arbeitsvertrag)");

    let valid_contract = EmploymentContract {
        employee: Employee {
            name: "Anna Müller".to_string(),
            date_of_birth: NaiveDate::from_ymd_opt(1990, 5, 15).unwrap(),
            address: "Hauptstraße 10, 10115 Berlin".to_string(),
            social_security_number: Some("12 345678 A 901".to_string()),
        },
        employer: Employer {
            name: "Tech Solutions GmbH".to_string(),
            address: "Alexanderplatz 1, 10178 Berlin".to_string(),
            company_size: CompanySize::Medium, // 50-250 employees
        },
        contract_type: ContractType::Unlimited,
        start_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        end_date: None,
        probation_period_months: Some(6), // Maximum allowed: 6 months
        salary: Salary {
            gross_monthly: Capital::from_euros(4_500),
            payment_day: 1, // First day of month
            includes_overtime: false,
        },
        working_hours: WorkingHours {
            hours_per_week: 40,
            days_per_week: 5,
            overtime_allowed: true,
        },
        duties: "Software development, code review, technical documentation, team collaboration"
            .to_string(),
        written: true, // §2 NachwG requires written documentation
    };

    match validate_employment_contract(&valid_contract) {
        Ok(()) => {
            println!("✅ Contract Valid!");
            println!("   Employee: {}", valid_contract.employee.name);
            println!("   Employer: {}", valid_contract.employer.name);
            println!("   Type: Unlimited (Unbefristet)");
            println!(
                "   Probation: {} months",
                valid_contract.probation_period_months.unwrap()
            );
            println!(
                "   Salary: €{:.2}/month",
                valid_contract.salary.gross_monthly.to_euros()
            );
            println!(
                "   Hours: {}h/week over {} days",
                valid_contract.working_hours.hours_per_week,
                valid_contract.working_hours.days_per_week
            );

            // Check ArbZG compliance
            if valid_contract.working_hours.complies_with_arbzg() {
                println!("   ArbZG: ✅ Compliant with §3 ArbZG (max 10h/day)");
            }
        }
        Err(e) => println!("❌ Validation Failed: {}", e),
    }

    // Example 2: Invalid - Probation Period Too Long
    println!("\n📋 Example 2: Invalid Contract - Probation Period Exceeds 6 Months");

    let mut invalid_probation = valid_contract.clone();
    invalid_probation.probation_period_months = Some(8); // Exceeds §622 Abs. 3 BGB limit

    match validate_employment_contract(&invalid_probation) {
        Ok(()) => println!("✅ Valid (unexpected)"),
        Err(e) => {
            println!("❌ Expected Error Caught:");
            println!("   {}", e);
            println!("   Legal Basis: §622 Abs. 3 BGB limits probation to 6 months");
        }
    }

    // Example 3: Invalid - Contract Not Written
    println!("\n📋 Example 3: Invalid Contract - Missing Written Form");

    let mut invalid_not_written = valid_contract.clone();
    invalid_not_written.written = false; // Violates §2 NachwG

    match validate_employment_contract(&invalid_not_written) {
        Ok(()) => println!("✅ Valid (unexpected)"),
        Err(e) => {
            println!("❌ Expected Error Caught:");
            println!("   {}", e);
            println!("   Legal Basis: §2 NachwG requires written documentation");
        }
    }

    // Example 4: Invalid - Working Hours Exceed ArbZG Limit
    println!("\n📋 Example 4: Invalid Contract - Working Hours Exceed Legal Maximum");

    let mut invalid_hours = valid_contract.clone();
    invalid_hours.working_hours = WorkingHours {
        hours_per_week: 60, // 12 hours/day for 5 days - exceeds ArbZG
        days_per_week: 5,
        overtime_allowed: true,
    };

    match validate_employment_contract(&invalid_hours) {
        Ok(()) => println!("✅ Valid (unexpected)"),
        Err(e) => {
            println!("❌ Expected Error Caught:");
            println!("   {}", e);
            println!(
                "   Legal Basis: §3 ArbZG limits regular working time to 8h/day (10h with compensation)"
            );
        }
    }

    // Example 5: Valid Fixed-Term Contract
    println!("\n📋 Example 5: Valid Fixed-Term Contract (Befristeter Arbeitsvertrag)");

    let fixed_term_contract = EmploymentContract {
        employee: Employee {
            name: "Thomas Schmidt".to_string(),
            date_of_birth: NaiveDate::from_ymd_opt(1985, 8, 20).unwrap(),
            address: "Berliner Straße 25, 80333 München".to_string(),
            social_security_number: Some("98 765432 B 109".to_string()),
        },
        employer: Employer {
            name: "Marketing Solutions GmbH".to_string(),
            address: "Maximilianstraße 10, 80539 München".to_string(),
            company_size: CompanySize::Small,
        },
        contract_type: ContractType::FixedTerm {
            reason: FixedTermReason::TemporaryNeed, // Temporary project need
        },
        start_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        end_date: Some(NaiveDate::from_ymd_opt(2024, 12, 31).unwrap()), // 1 year fixed term
        probation_period_months: None, // Often omitted for short fixed-term contracts
        salary: Salary {
            gross_monthly: Capital::from_euros(3_800),
            payment_day: 15,
            includes_overtime: true,
        },
        working_hours: WorkingHours {
            hours_per_week: 38,
            days_per_week: 5,
            overtime_allowed: true,
        },
        duties: "Digital marketing strategy, content creation, social media management".to_string(),
        written: true,
    };

    match validate_employment_contract(&fixed_term_contract) {
        Ok(()) => {
            println!("✅ Fixed-Term Contract Valid!");
            println!("   Employee: {}", fixed_term_contract.employee.name);
            println!("   Type: Fixed-Term (Befristet) - Temporary Need");
            println!(
                "   Duration: {} to {}",
                fixed_term_contract.start_date,
                fixed_term_contract.end_date.unwrap()
            );
            println!("   Legal Basis: §14 TzBfG (temporary need fixed term)");
        }
        Err(e) => println!("❌ Validation Failed: {}", e),
    }

    // Example 6: Invalid Fixed-Term - Too Long Without Reason
    println!("\n📋 Example 6: Invalid Fixed-Term - Duration Exceeds 2 Years Without Reason");

    let invalid_fixed_term = EmploymentContract {
        employee: Employee {
            name: "Laura Weber".to_string(),
            date_of_birth: NaiveDate::from_ymd_opt(1992, 3, 10).unwrap(),
            address: "Rheinstraße 5, 50668 Köln".to_string(),
            social_security_number: None,
        },
        employer: Employer {
            name: "Retail GmbH".to_string(),
            address: "Hohe Straße 50, 50667 Köln".to_string(),
            company_size: CompanySize::Medium,
        },
        contract_type: ContractType::FixedTerm {
            reason: FixedTermReason::NoReasonNeeded, // No reason - max 2 years
        },
        start_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        end_date: Some(NaiveDate::from_ymd_opt(2027, 1, 1).unwrap()), // 3 years - too long!
        probation_period_months: None,
        salary: Salary {
            gross_monthly: Capital::from_euros(2_800),
            payment_day: 1,
            includes_overtime: false,
        },
        working_hours: WorkingHours {
            hours_per_week: 40,
            days_per_week: 5,
            overtime_allowed: false,
        },
        duties: "Sales and customer service".to_string(),
        written: true,
    };

    match validate_employment_contract(&invalid_fixed_term) {
        Ok(()) => println!("✅ Valid (unexpected)"),
        Err(e) => {
            println!("❌ Expected Error Caught:");
            println!("   {}", e);
            println!(
                "   Legal Basis: §14 Abs. 2 TzBfG limits fixed terms without reason to 2 years"
            );
        }
    }

    // Example 7: Valid Part-Time Contract
    println!("\n📋 Example 7: Valid Part-Time Contract (Teilzeitvertrag)");

    let part_time_contract = EmploymentContract {
        employee: Employee {
            name: "Maria Hoffmann".to_string(),
            date_of_birth: NaiveDate::from_ymd_opt(1988, 11, 5).unwrap(),
            address: "Gartenstraße 15, 60594 Frankfurt".to_string(),
            social_security_number: Some("45 678901 C 234".to_string()),
        },
        employer: Employer {
            name: "Finance Consulting GmbH".to_string(),
            address: "Taunusanlage 8, 60329 Frankfurt".to_string(),
            company_size: CompanySize::Large,
        },
        contract_type: ContractType::PartTime { hours_per_week: 20 },
        start_date: NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
        end_date: None,
        probation_period_months: Some(3), // Shorter probation for part-time
        salary: Salary {
            gross_monthly: Capital::from_euros(2_200), // Proportional to 20h/week
            payment_day: 1,
            includes_overtime: false,
        },
        working_hours: WorkingHours {
            hours_per_week: 20,
            days_per_week: 3, // Mon/Wed/Fri
            overtime_allowed: false,
        },
        duties: "Financial consulting, client support (part-time)".to_string(),
        written: true,
    };

    match validate_employment_contract(&part_time_contract) {
        Ok(()) => {
            println!("✅ Part-Time Contract Valid!");
            println!("   Employee: {}", part_time_contract.employee.name);
            println!("   Type: Part-Time (Teilzeit)");
            println!(
                "   Hours: {}h/week over {} days",
                part_time_contract.working_hours.hours_per_week,
                part_time_contract.working_hours.days_per_week
            );
            println!(
                "   Salary: €{:.2}/month (proportional)",
                part_time_contract.salary.gross_monthly.to_euros()
            );
            println!("   Legal Basis: TzBfG (Part-Time and Fixed-Term Employment Act)");
        }
        Err(e) => println!("❌ Validation Failed: {}", e),
    }

    println!("\n=== Summary ===");
    println!("✅ Employment contract validation covers:");
    println!("   • Written form requirement (§2 NachwG)");
    println!("   • Probation period limits (§622 Abs. 3 BGB - max 6 months)");
    println!("   • Working hours compliance (§3 ArbZG - max 10h/day)");
    println!("   • Fixed-term duration limits (§14 TzBfG - 2 years without reason)");
    println!("   • Part-time employment (TzBfG)");
    println!("   • Contract type validation");
    println!("\n📚 Key German Labor Law Statutes:");
    println!("   • NachwG - Employment Documentation Act");
    println!("   • BGB §622 - Notice Periods and Probation");
    println!("   • ArbZG - Working Hours Act");
    println!("   • TzBfG - Part-Time and Fixed-Term Employment Act");
    println!("   • KSchG - Protection Against Dismissal Act");
}
