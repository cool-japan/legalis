//! Leave Entitlement Calculation Example
//!
//! Demonstrates calculation and validation of annual leave (BUrlG),
//! sick leave (EFZG), maternity leave (MuSchG), and parental leave (BEEG)
//! under German labor law.

use chrono::NaiveDate;
use legalis_de::arbeitsrecht::*;

fn main() {
    println!("=== German Leave Entitlement Calculation ===\n");
    println!("Urlaubsanspruchsberechnung nach deutschem Arbeitsrecht\n");

    // Example 1: Valid Annual Leave Entitlement (5-day week)
    println!("📋 Example 1: Annual Leave Entitlement - 5-Day Work Week");

    let leave_5_day = LeaveEntitlement {
        employee_name: "Anna Müller".to_string(),
        year: 2024,
        days_per_week: 5,
        minimum_days: LeaveEntitlement::calculate_minimum(5), // 20 days
        contractual_days: 30,                                 // Generous employer provision
        days_taken: 10,
        days_carried_over: 0,
    };

    println!("   Employee: {}", leave_5_day.employee_name);
    println!("   Work Week: {} days/week", leave_5_day.days_per_week);
    println!(
        "   Minimum Legal: {} days (§3 BUrlG)",
        leave_5_day.minimum_days
    );
    println!("   Contractual: {} days", leave_5_day.contractual_days);
    println!("   Taken: {} days", leave_5_day.days_taken);
    println!(
        "   Remaining: {} days",
        leave_5_day.contractual_days - leave_5_day.days_taken
    );

    match validate_leave_entitlement(&leave_5_day) {
        Ok(()) => {
            println!("   ✅ Leave Entitlement Valid!");
            println!(
                "   Note: Exceeds §3 BUrlG minimum by {} days",
                leave_5_day.contractual_days - leave_5_day.minimum_days
            );
        }
        Err(e) => println!("   ❌ Validation Failed: {}", e),
    }

    // Example 2: Valid Annual Leave Entitlement (6-day week)
    println!("\n📋 Example 2: Annual Leave Entitlement - 6-Day Work Week");

    let leave_6_day = LeaveEntitlement {
        employee_name: "Thomas Schmidt".to_string(),
        year: 2024,
        days_per_week: 6,
        minimum_days: LeaveEntitlement::calculate_minimum(6), // 24 days
        contractual_days: 28,                                 // Above minimum
        days_taken: 12,
        days_carried_over: 0,
    };

    println!("   Employee: {}", leave_6_day.employee_name);
    println!("   Work Week: {} days/week", leave_6_day.days_per_week);
    println!(
        "   Minimum Legal: {} days (§3 BUrlG)",
        leave_6_day.minimum_days
    );
    println!("   Contractual: {} days", leave_6_day.contractual_days);

    match validate_leave_entitlement(&leave_6_day) {
        Ok(()) => println!("   ✅ Leave Entitlement Valid!"),
        Err(e) => println!("   ❌ Validation Failed: {}", e),
    }

    // Example 3: Invalid - Below BUrlG Minimum
    println!("\n📋 Example 3: Invalid Leave Entitlement - Below Legal Minimum");

    let invalid_leave = LeaveEntitlement {
        employee_name: "Laura Weber".to_string(),
        year: 2024,
        days_per_week: 5,
        minimum_days: 20,
        contractual_days: 15, // Below §3 BUrlG minimum!
        days_taken: 5,
        days_carried_over: 0,
    };

    match validate_leave_entitlement(&invalid_leave) {
        Ok(()) => println!("   ✅ Valid (unexpected)"),
        Err(e) => {
            println!("   ❌ Expected Error Caught:");
            println!("   {}", e);
            println!(
                "   Legal Basis: §3 BUrlG guarantees minimum 4 weeks (20 days for 5-day week)"
            );
            println!("   Note: Employers cannot offer less, even by agreement");
        }
    }

    // Example 4: Part-Time Leave Calculation
    println!("\n📋 Example 4: Part-Time Leave Entitlement (3-day week)");

    let leave_part_time = LeaveEntitlement {
        employee_name: "Maria Hoffmann".to_string(),
        year: 2024,
        days_per_week: 3,
        minimum_days: LeaveEntitlement::calculate_minimum(3), // 12 days
        contractual_days: 18,                                 // Above minimum
        days_taken: 6,
        days_carried_over: 0,
    };

    println!("   Employee: {}", leave_part_time.employee_name);
    println!(
        "   Work Week: {} days/week (Part-Time)",
        leave_part_time.days_per_week
    );
    println!(
        "   Minimum Legal: {} days (§3 BUrlG)",
        leave_part_time.minimum_days
    );
    println!("   Contractual: {} days", leave_part_time.contractual_days);
    println!(
        "   Calculation: 24 days × (3/6) = {} days minimum",
        leave_part_time.minimum_days
    );

    match validate_leave_entitlement(&leave_part_time) {
        Ok(()) => {
            println!("   ✅ Part-Time Leave Valid!");
            println!("   Note: BUrlG minimum scales proportionally with work days");
        }
        Err(e) => println!("   ❌ Validation Failed: {}", e),
    }

    // Example 5: Valid Sick Leave
    println!("\n📋 Example 5: Valid Sick Leave (Krankheit) with Medical Certificate");

    let sick_leave = SickLeave {
        employee_name: "Peter Klein".to_string(),
        start_date: NaiveDate::from_ymd_opt(2024, 3, 1).unwrap(),
        end_date: Some(NaiveDate::from_ymd_opt(2024, 3, 10).unwrap()), // 10 days
        medical_certificate_provided: true,                            // Required after 3 days
        notification_timely: true, // Employee informed employer immediately
    };

    println!("   Employee: {}", sick_leave.employee_name);
    println!(
        "   Duration: {} to {}",
        sick_leave.start_date,
        sick_leave.end_date.unwrap()
    );
    let current_date = NaiveDate::from_ymd_opt(2024, 3, 10).unwrap();
    println!("   Days: {} days", sick_leave.duration_days(current_date));
    println!("   Medical Certificate: ✅ Provided");
    println!("   Notification: ✅ Timely");

    match validate_sick_leave(&sick_leave) {
        Ok(()) => {
            println!("   ✅ Sick Leave Valid!");
            println!("\n   Legal Framework:");
            println!("   • §3 EFZG - 6 weeks sick pay at 100% salary");
            println!("   • §5 EFZG - Medical certificate required after 3 days");
            println!("   • Employee must notify employer immediately");
            println!("   • After 6 weeks: Krankengeld from health insurance (70%)");
        }
        Err(e) => println!("   ❌ Validation Failed: {}", e),
    }

    // Example 6: Invalid Sick Leave - No Medical Certificate
    println!("\n📋 Example 6: Invalid Sick Leave - Missing Medical Certificate");

    let mut invalid_sick_leave = sick_leave.clone();
    invalid_sick_leave.medical_certificate_provided = false; // Missing after 3 days

    match validate_sick_leave(&invalid_sick_leave) {
        Ok(()) => println!("   ✅ Valid (unexpected)"),
        Err(e) => {
            println!("   ❌ Expected Error Caught:");
            println!("   {}", e);
            println!("   Legal Basis: §5 EFZG requires medical certificate after 3 calendar days");
            println!("   Note: Employer can require certificate from day 1 in employment contract");
        }
    }

    // Example 7: Valid Parental Leave
    println!("\n📋 Example 7: Valid Parental Leave (Elternzeit)");

    let parental_leave = ParentalLeave {
        employee_name: "Michael Richter".to_string(),
        child_birth_date: NaiveDate::from_ymd_opt(2024, 1, 15).unwrap(),
        leave_start: NaiveDate::from_ymd_opt(2024, 4, 1).unwrap(),
        leave_end: NaiveDate::from_ymd_opt(2025, 4, 1).unwrap(), // 1 year
        notice_given_weeks: 10,                                  // Minimum 7 weeks required
    };

    let duration_years = parental_leave.duration_years();

    println!("   Employee: {}", parental_leave.employee_name);
    println!("   Child Birth: {}", parental_leave.child_birth_date);
    println!(
        "   Leave Period: {} to {}",
        parental_leave.leave_start, parental_leave.leave_end
    );
    println!("   Duration: {:.1} years", duration_years);
    println!(
        "   Notice Given: {} weeks (min: 7)",
        parental_leave.notice_given_weeks
    );

    match validate_parental_leave(&parental_leave) {
        Ok(()) => {
            println!("   ✅ Parental Leave Valid!");
            println!("\n   Legal Framework:");
            println!("   • §15 BEEG - Up to 3 years parental leave");
            println!("   • §16 BEEG - Minimum 7 weeks notice");
            println!("   • §18 BEEG - Dismissal protection");
            println!("   • Can be split into 3 periods");
            println!("   • Elterngeld: Income-based allowance (65-67% of net income)");
            println!("   • Part-time work (25-30h/week) allowed during leave");
        }
        Err(e) => println!("   ❌ Validation Failed: {}", e),
    }

    // Example 8: Invalid Parental Leave - Too Long
    println!("\n📋 Example 8: Invalid Parental Leave - Exceeds 3 Years");

    let invalid_parental_leave = ParentalLeave {
        employee_name: "Sarah Vogel".to_string(),
        child_birth_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        leave_start: NaiveDate::from_ymd_opt(2024, 2, 1).unwrap(),
        leave_end: NaiveDate::from_ymd_opt(2027, 8, 1).unwrap(), // 3.5 years - too long!
        notice_given_weeks: 8,
    };

    match validate_parental_leave(&invalid_parental_leave) {
        Ok(()) => println!("   ✅ Valid (unexpected)"),
        Err(e) => {
            println!("   ❌ Expected Error Caught:");
            println!("   {}", e);
            println!("   Legal Basis: §15 BEEG limits parental leave to 3 years per child");
        }
    }

    // Example 9: Invalid Parental Leave - Insufficient Notice
    println!("\n📋 Example 9: Invalid Parental Leave - Insufficient Notice Period");

    let invalid_notice = ParentalLeave {
        employee_name: "Daniel Keller".to_string(),
        child_birth_date: NaiveDate::from_ymd_opt(2024, 3, 1).unwrap(),
        leave_start: NaiveDate::from_ymd_opt(2024, 4, 15).unwrap(),
        leave_end: NaiveDate::from_ymd_opt(2025, 4, 15).unwrap(),
        notice_given_weeks: 4, // Below 7-week minimum!
    };

    match validate_parental_leave(&invalid_notice) {
        Ok(()) => println!("   ✅ Valid (unexpected)"),
        Err(e) => {
            println!("   ❌ Expected Error Caught:");
            println!("   {}", e);
            println!("   Legal Basis: §16 BEEG requires minimum 7 weeks notice before leave start");
            println!("   Note: For 3rd year of leave, 13 weeks notice required");
        }
    }

    // Example 10: Works Council Size Calculation
    println!("\n📋 Example 10: Works Council Size Calculation (Betriebsrat)");

    let examples = vec![
        (8, WorksCouncil::required_size(8)),
        (25, WorksCouncil::required_size(25)),
        (75, WorksCouncil::required_size(75)),
        (150, WorksCouncil::required_size(150)),
        (350, WorksCouncil::required_size(350)),
        (900, WorksCouncil::required_size(900)),
        (1200, WorksCouncil::required_size(1200)),
        (2500, WorksCouncil::required_size(2500)),
    ];

    println!("\n   Employee Count → Required Works Council Size (§9 BetrVG):");
    for (employees, council_size) in examples {
        println!(
            "   {:>5} employees → {:>2} council members",
            employees, council_size
        );
    }

    println!("\n   Legal Basis: §9 BetrVG - Works Council Size Thresholds:");
    println!("   • 5-20 employees: 1 member");
    println!("   • 21-50 employees: 3 members");
    println!("   • 51-100 employees: 5 members");
    println!("   • Continues scaling with employee count");
    println!("   • For 1,501+: 15 + 1 per 500 additional employees");

    println!("\n=== Leave Entitlement Summary ===");
    println!("\n📚 Key German Leave Statutes:");
    println!("\n1️⃣  Annual Leave (BUrlG - Bundesurlaubsgesetz):");
    println!("   • Minimum: 24 working days for 6-day week (§3 BUrlG)");
    println!("   • Scales proportionally: 20 days for 5-day week");
    println!("   • Cannot be reduced by agreement (mandatory minimum)");
    println!("   • Must be taken in current year, carryover until March 31");
    println!("\n2️⃣  Sick Leave (EFZG - Entgeltfortzahlungsgesetz):");
    println!("   • 6 weeks at 100% salary (§3 EFZG)");
    println!("   • Medical certificate after 3 days (§5 EFZG)");
    println!("   • Immediate employer notification required");
    println!("   • After 6 weeks: Krankengeld (70% from insurance)");
    println!("\n3️⃣  Maternity Leave (MuSchG - Mutterschutzgesetz):");
    println!("   • 6 weeks before birth (§3 MuSchG)");
    println!("   • 8 weeks after birth (12 for multiples)");
    println!("   • Full salary (Mutterschutzlohn)");
    println!("   • Dismissal prohibition (§17 MuSchG)");
    println!("\n4️⃣  Parental Leave (BEEG - Bundeselterngeld- und Elternzeitgesetz):");
    println!("   • Up to 3 years per child (§15 BEEG)");
    println!("   • Minimum 7 weeks notice (§16 BEEG)");
    println!("   • Dismissal protection (§18 BEEG)");
    println!("   • Part-time work allowed (25-30h/week)");
    println!("   • Elterngeld: 65-67% of net income");
    println!("\n5️⃣  Works Council (BetrVG - Betriebsverfassungsgesetz):");
    println!("   • Required for companies with 5+ employees (§1 BetrVG)");
    println!("   • Size scales with employee count (§9 BetrVG)");
    println!("   • Co-determination rights (§87 BetrVG)");
    println!("   • Consultation required for dismissals (§102 BetrVG)");
}
