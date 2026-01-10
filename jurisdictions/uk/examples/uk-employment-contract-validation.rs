//! Employment Contract Validation Examples
//!
//! Demonstrates UK employment law compliance checking under:
//! - Employment Rights Act 1996 (ERA 1996)
//! - Working Time Regulations 1998 (WTR 1998)
//! - National Minimum Wage Act 1998 (NMWA 1998)

use chrono::NaiveDate;
use legalis_uk::employment::*;

fn main() {
    println!("=== UK Employment Contract Validation Examples ===\n");

    // Example 1: Valid permanent contract
    example_1_valid_permanent_contract();

    // Example 2: Missing written particulars
    example_2_missing_written_particulars();

    // Example 3: Notice period below statutory minimum
    example_3_notice_period_below_minimum();

    // Example 4: Working hours exceed 48-hour limit
    example_4_working_hours_exceed_limit();

    // Example 5: Below minimum wage
    example_5_below_minimum_wage();

    // Example 6: Illegal zero-hours exclusivity clause
    example_6_zero_hours_exclusivity();

    // Example 7: Part-time contract (compliant)
    example_7_part_time_contract();
}

fn example_1_valid_permanent_contract() {
    println!("Example 1: Valid Permanent Contract");
    println!("=====================================\n");

    let contract = EmploymentContract::builder()
        .with_employee(Employee {
            name: "Sarah Johnson".to_string(),
            date_of_birth: NaiveDate::from_ymd_opt(1985, 3, 15).unwrap(),
            address: "42 Baker Street, London, W1U 6TQ".to_string(),
            national_insurance_number: Some("AB 12 34 56 C".to_string()),
        })
        .with_employer(Employer {
            name: "Tech Innovations Ltd".to_string(),
            address: "100 Bishopsgate, London, EC2N 4AG".to_string(),
            employee_count: Some(150),
        })
        .with_contract_type(ContractType::Permanent)
        .with_start_date(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap())
        .with_probation_period_months(6)
        .with_salary(Salary {
            gross_annual_gbp: 45000.0,
            payment_frequency: PaymentFrequency::Monthly,
            payment_day: 28,
        })
        .with_working_hours(WorkingHours {
            hours_per_week: 37,
            days_per_week: 5,
            opted_out_of_48h_limit: false,
            night_work_hours: None,
        })
        .with_duties("Senior Software Engineer: Design, develop and maintain web applications using Rust and TypeScript. Lead technical projects and mentor junior developers.".to_string())
        .with_notice_period(NoticePeriod {
            employer_notice_weeks: 4,
            employee_notice_weeks: 4,
        })
        .with_written_particulars(true)
        .with_pension_scheme(PensionScheme {
            scheme_name: "Tech Innovations Group Pension Scheme".to_string(),
            employee_contribution_pct: 5.0,
            employer_contribution_pct: 3.0,
            auto_enrolled: true,
        })
        .build();

    match validate_employment_contract(&contract) {
        Ok(()) => {
            println!("✅ Contract is COMPLIANT with UK employment law\n");
            println!("Contract Details:");
            println!("  Employee: {}", contract.employee.name);
            println!("  Employer: {}", contract.employer.name);
            println!("  Type: Permanent");
            println!(
                "  Salary: £{:.2}/year (£{:.2}/month)",
                contract.salary.gross_annual_gbp,
                contract.salary.gross_monthly()
            );
            println!(
                "  Hours: {} hours/week",
                contract.working_hours.hours_per_week
            );
            println!(
                "  Probation: {} months",
                contract.probation_period_months.unwrap()
            );
            println!(
                "  Notice: {} weeks",
                contract.notice_period.employer_notice_weeks
            );
            println!("  Written particulars: Yes ✓");
            println!("  Pension auto-enrolment: Yes ✓\n");
        }
        Err(e) => {
            println!("❌ Contract VIOLATION: {}\n", e);
        }
    }
}

fn example_2_missing_written_particulars() {
    println!("Example 2: Missing Written Particulars (ERA 1996 s.1)");
    println!("=======================================================\n");

    let contract = EmploymentContract::builder()
        .with_employee(Employee {
            name: "John Smith".to_string(),
            date_of_birth: NaiveDate::from_ymd_opt(1990, 6, 20).unwrap(),
            address: "15 High Street, Manchester, M1 1AD".to_string(),
            national_insurance_number: Some("CD 78 90 12 E".to_string()),
        })
        .with_employer(Employer {
            name: "Retail Shop Ltd".to_string(),
            address: "200 Oxford Street, London, W1D 1NU".to_string(),
            employee_count: Some(25),
        })
        .with_contract_type(ContractType::Permanent)
        .with_start_date(NaiveDate::from_ymd_opt(2024, 6, 1).unwrap())
        .with_salary(Salary {
            gross_annual_gbp: 22000.0,
            payment_frequency: PaymentFrequency::Monthly,
            payment_day: 25,
        })
        .with_working_hours(WorkingHours {
            hours_per_week: 40,
            days_per_week: 5,
            opted_out_of_48h_limit: false,
            night_work_hours: None,
        })
        .with_written_particulars(false) // ❌ NOT PROVIDED
        .build();

    match validate_employment_contract(&contract) {
        Ok(()) => {
            println!("✅ Contract compliant\n");
        }
        Err(e) => {
            println!("❌ ERA 1996 VIOLATION DETECTED:\n{}\n", e);
            println!(
                "Remedy: Employer must provide written statement within 2 months of start date.\n"
            );
        }
    }
}

fn example_3_notice_period_below_minimum() {
    println!("Example 3: Notice Period Below Statutory Minimum (ERA 1996 s.86)");
    println!("==================================================================\n");

    let contract = EmploymentContract::builder()
        .with_employee(Employee {
            name: "Emma Williams".to_string(),
            date_of_birth: NaiveDate::from_ymd_opt(1988, 9, 10).unwrap(),
            address: "78 King's Road, Edinburgh, EH1 2AB".to_string(),
            national_insurance_number: Some("EF 34 56 78 G".to_string()),
        })
        .with_employer(Employer {
            name: "Marketing Agency Ltd".to_string(),
            address: "50 Princes Street, Edinburgh, EH2 2BY".to_string(),
            employee_count: Some(40),
        })
        .with_contract_type(ContractType::Permanent)
        .with_start_date(NaiveDate::from_ymd_opt(2019, 1, 1).unwrap()) // 5 years service
        .with_salary(Salary {
            gross_annual_gbp: 35000.0,
            payment_frequency: PaymentFrequency::Monthly,
            payment_day: 1,
        })
        .with_working_hours(WorkingHours {
            hours_per_week: 37,
            days_per_week: 5,
            opted_out_of_48h_limit: false,
            night_work_hours: None,
        })
        .with_notice_period(NoticePeriod {
            employer_notice_weeks: 2, // ❌ Only 2 weeks (needs 5 weeks for 5 years service)
            employee_notice_weeks: 2,
        })
        .with_written_particulars(true)
        .build();

    match validate_employment_contract(&contract) {
        Ok(()) => {
            println!("✅ Contract compliant\n");
        }
        Err(e) => {
            println!("❌ ERA 1996 s.86 VIOLATION DETECTED:\n{}\n", e);
            println!("Remedy: Contract must specify at least statutory minimum notice.\n");
        }
    }
}

fn example_4_working_hours_exceed_limit() {
    println!("Example 4: Working Hours Exceed 48-Hour Limit (WTR 1998 Reg 4)");
    println!("================================================================\n");

    let contract = EmploymentContract::builder()
        .with_employee(Employee {
            name: "David Chen".to_string(),
            date_of_birth: NaiveDate::from_ymd_opt(1992, 11, 5).unwrap(),
            address: "23 Park Lane, Birmingham, B1 1BB".to_string(),
            national_insurance_number: Some("GH 90 12 34 I".to_string()),
        })
        .with_employer(Employer {
            name: "Finance Corp Ltd".to_string(),
            address: "1 Canary Wharf, London, E14 5AB".to_string(),
            employee_count: Some(500),
        })
        .with_contract_type(ContractType::Permanent)
        .with_start_date(NaiveDate::from_ymd_opt(2023, 1, 1).unwrap())
        .with_salary(Salary {
            gross_annual_gbp: 65000.0,
            payment_frequency: PaymentFrequency::Monthly,
            payment_day: 15,
        })
        .with_working_hours(WorkingHours {
            hours_per_week: 55, // ❌ Exceeds 48-hour limit
            days_per_week: 6,
            opted_out_of_48h_limit: false, // ❌ No opt-out signed
            night_work_hours: None,
        })
        .with_written_particulars(true)
        .build();

    match validate_employment_contract(&contract) {
        Ok(()) => {
            println!("✅ Contract compliant\n");
        }
        Err(e) => {
            println!("❌ WTR 1998 VIOLATION DETECTED:\n{}\n", e);
            println!(
                "Remedy: Either reduce hours to 48/week or obtain written opt-out from employee.\n"
            );
        }
    }
}

fn example_5_below_minimum_wage() {
    println!("Example 5: Hourly Rate Below National Minimum Wage (NMWA 1998)");
    println!("================================================================\n");

    let contract = EmploymentContract::builder()
        .with_employee(Employee {
            name: "Lucy Brown".to_string(),
            date_of_birth: NaiveDate::from_ymd_opt(2004, 4, 12).unwrap(), // Age 19
            address: "67 Station Road, Bristol, BS1 6QA".to_string(),
            national_insurance_number: Some("IJ 56 78 90 K".to_string()),
        })
        .with_employer(Employer {
            name: "Cafe & Bistro Ltd".to_string(),
            address: "12 Harbour Street, Bristol, BS1 2LN".to_string(),
            employee_count: Some(8),
        })
        .with_contract_type(ContractType::PartTime {
            hours_per_week: 20,
            less_favourable: false,
        })
        .with_start_date(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())
        .with_salary(Salary {
            gross_annual_gbp: 8000.0, // ❌ £8,000/year ÷ 52 weeks ÷ 20 hours = £7.69/hour (below £8.60 for 18-20)
            payment_frequency: PaymentFrequency::Monthly,
            payment_day: 1,
        })
        .with_working_hours(WorkingHours {
            hours_per_week: 20,
            days_per_week: 4,
            opted_out_of_48h_limit: false,
            night_work_hours: None,
        })
        .with_written_particulars(true)
        .build();

    match validate_employment_contract(&contract) {
        Ok(()) => {
            println!("✅ Contract compliant\n");
        }
        Err(e) => {
            println!("❌ NMWA 1998 VIOLATION DETECTED:\n{}\n", e);
            let age = contract
                .employee
                .age_at(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());
            let hourly_rate = contract
                .salary
                .gross_hourly(contract.working_hours.hours_per_week);
            let assessment = MinimumWageAssessment {
                age,
                hourly_rate_gbp: hourly_rate,
                apprentice: false,
            };
            println!("Current: £{:.2}/hour", hourly_rate);
            println!(
                "Required: £{:.2}/hour (age {})",
                assessment.applicable_minimum_wage(),
                age
            );
            println!(
                "Remedy: Increase annual salary to at least £{:.2}\n",
                assessment.applicable_minimum_wage() * 20.0 * 52.0
            );
        }
    }
}

fn example_6_zero_hours_exclusivity() {
    println!("Example 6: Illegal Zero-Hours Exclusivity Clause");
    println!("=================================================\n");

    let contract = EmploymentContract::builder()
        .with_employee(Employee {
            name: "Michael Taylor".to_string(),
            date_of_birth: NaiveDate::from_ymd_opt(1995, 8, 22).unwrap(),
            address: "89 Market Street, Leeds, LS1 6AA".to_string(),
            national_insurance_number: Some("KL 12 34 56 M".to_string()),
        })
        .with_employer(Employer {
            name: "Delivery Services Ltd".to_string(),
            address: "45 Industrial Estate, Leeds, LS10 1AB".to_string(),
            employee_count: Some(200),
        })
        .with_contract_type(ContractType::ZeroHours {
            exclusivity_clause: true, // ❌ ILLEGAL since 2015
        })
        .with_start_date(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())
        .with_salary(Salary {
            gross_annual_gbp: 0.0, // Variable (zero-hours)
            payment_frequency: PaymentFrequency::Weekly,
            payment_day: 5,
        })
        .with_working_hours(WorkingHours {
            hours_per_week: 0, // Variable
            days_per_week: 0,
            opted_out_of_48h_limit: false,
            night_work_hours: None,
        })
        .with_written_particulars(true)
        .build();

    match validate_employment_contract(&contract) {
        Ok(()) => {
            println!("✅ Contract compliant\n");
        }
        Err(e) => {
            println!("❌ ILLEGAL EXCLUSIVITY CLAUSE DETECTED:\n{}\n", e);
            println!(
                "Remedy: Remove exclusivity clause. Worker has right to work for other employers.\n"
            );
        }
    }
}

fn example_7_part_time_contract() {
    println!("Example 7: Compliant Part-Time Contract");
    println!("=========================================\n");

    let contract = EmploymentContract::builder()
        .with_employee(Employee {
            name: "Rebecca Wilson".to_string(),
            date_of_birth: NaiveDate::from_ymd_opt(1987, 2, 14).unwrap(),
            address: "34 Church Lane, Cardiff, CF10 1BH".to_string(),
            national_insurance_number: Some("MN 78 90 12 N".to_string()),
        })
        .with_employer(Employer {
            name: "Accounting Firm LLP".to_string(),
            address: "88 St Mary Street, Cardiff, CF10 1DX".to_string(),
            employee_count: Some(75),
        })
        .with_contract_type(ContractType::PartTime {
            hours_per_week: 25,
            less_favourable: false, // Pro-rata rights same as full-time
        })
        .with_start_date(NaiveDate::from_ymd_opt(2022, 9, 1).unwrap())
        .with_salary(Salary {
            gross_annual_gbp: 28000.0, // Pro-rata of full-time equivalent
            payment_frequency: PaymentFrequency::Monthly,
            payment_day: 28,
        })
        .with_working_hours(WorkingHours {
            hours_per_week: 25,
            days_per_week: 5,
            opted_out_of_48h_limit: false,
            night_work_hours: None,
        })
        .with_notice_period(NoticePeriod {
            employer_notice_weeks: 2,
            employee_notice_weeks: 2,
        })
        .with_written_particulars(true)
        .build();

    match validate_employment_contract(&contract) {
        Ok(()) => {
            println!("✅ Contract is COMPLIANT with UK employment law\n");
            println!("Contract Details:");
            println!("  Type: Part-Time (25 hours/week)");
            println!("  Salary: £{:.2}/year", contract.salary.gross_annual_gbp);
            println!(
                "  Hourly rate: £{:.2}/hour",
                contract
                    .salary
                    .gross_hourly(contract.working_hours.hours_per_week)
            );

            // Calculate annual leave entitlement
            let leave = AnnualLeaveEntitlement {
                days_per_week: 5,
                leave_year_start: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            };
            println!(
                "  Annual leave: {:.1} days (5.6 weeks)",
                leave.statutory_minimum_days()
            );
            println!("  Part-Time Workers Regulations 2000: Pro-rata rights apply ✓\n");
        }
        Err(e) => {
            println!("❌ Contract VIOLATION: {}\n", e);
        }
    }
}
