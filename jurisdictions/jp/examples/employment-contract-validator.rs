//! Employment Contract Validation Example (雇用契約検証例)
//!
//! This example demonstrates validation of employment contracts according to
//! the Labor Standards Act (労働基準法) and Labor Contract Act (労働契約法).
//!
//! # Usage
//! ```bash
//! cargo run --example employment-contract-validator
//! ```

use chrono::{Duration, Utc};
use legalis_jp::labor_law::*;

fn main() {
    println!("=== Employment Contract Validation Example ===\n");
    println!("雇用契約検証例 - Employment Contract Validation\n");

    // Example 1: Valid Indefinite-term Contract
    println!("📋 Example 1: Valid Indefinite-term Contract (無期雇用契約)");
    println!("{}", "=".repeat(70));

    let valid_contract = EmploymentContract {
        employee_name: "山田太郎".to_string(),
        employer_name: "テクノロジーソリューションズ株式会社".to_string(),
        employment_type: EmploymentType::IndefiniteTerm,
        work_pattern: WorkPattern::Regular,
        start_date: Utc::now(),
        end_date: None,
        base_wage_jpy: 350_000,
        hours_per_day: 8,
        days_per_week: 5,
        job_description: "ソフトウェアエンジニア (Software Engineer)".to_string(),
        work_location: "東京オフィス (Tokyo Office)".to_string(),
        probation_period_days: Some(90),
        renewal_count: 0,
    };

    println!("Employee: {}", valid_contract.employee_name);
    println!("Employer: {}", valid_contract.employer_name);
    println!("Type: {:?}", valid_contract.employment_type);
    println!("Base Wage: ¥{}", valid_contract.base_wage_jpy);
    println!(
        "Hours: {} hours/day, {} days/week",
        valid_contract.hours_per_day, valid_contract.days_per_week
    );
    println!("Weekly Hours: {} hours", valid_contract.weekly_hours());
    println!("Job: {}", valid_contract.job_description);
    println!(
        "Probation: {} days",
        valid_contract.probation_period_days.unwrap_or(0)
    );

    match validate_employment_contract(&valid_contract) {
        Ok(()) => println!("\n✅ Validation: PASSED - Contract complies with labor laws!"),
        Err(e) => println!("\n❌ Validation: FAILED - {}", e),
    }

    println!("\n{}\n", "=".repeat(70));

    // Example 2: Fixed-term Contract with Conversion Eligibility
    println!("📋 Example 2: Fixed-term Contract (5-year rule) (有期雇用契約・無期転換)");
    println!("{}", "=".repeat(70));

    let fixed_term_contract = EmploymentContract {
        employee_name: "佐藤花子".to_string(),
        employer_name: "グローバルテック株式会社".to_string(),
        employment_type: EmploymentType::FixedTerm,
        work_pattern: WorkPattern::Regular,
        start_date: Utc::now() - Duration::days(365 * 5 + 30), // 5 years+ ago
        end_date: Some(Utc::now() + Duration::days(365)),
        base_wage_jpy: 280_000,
        hours_per_day: 7,
        days_per_week: 5,
        job_description: "プロジェクトマネージャー (Project Manager)".to_string(),
        work_location: "大阪オフィス (Osaka Office)".to_string(),
        probation_period_days: None,
        renewal_count: 4,
    };

    println!("Employee: {}", fixed_term_contract.employee_name);
    println!("Type: {:?}", fixed_term_contract.employment_type);
    println!("Contract Renewals: {}", fixed_term_contract.renewal_count);

    let years_employed = (Utc::now() - fixed_term_contract.start_date).num_days() / 365;
    println!("Years Employed: {} years", years_employed);

    if check_indefinite_conversion_eligibility(&fixed_term_contract).unwrap() {
        println!("\n⚠️  IMPORTANT: Employee is eligible for indefinite-term conversion!");
        println!("   (無期転換ルール適用 - Labor Contract Act Article 18)");
        println!("   Employee has the right to request conversion to indefinite-term employment.");
    }

    match validate_employment_contract(&fixed_term_contract) {
        Ok(()) => println!("\n✅ Validation: PASSED"),
        Err(e) => println!("\n❌ Validation: FAILED - {}", e),
    }

    println!("\n{}\n", "=".repeat(70));

    // Example 3: Invalid Contract - Excessive Hours
    println!("📋 Example 3: Invalid Contract - Excessive Working Hours");
    println!("{}", "=".repeat(70));

    let excessive_hours_contract = EmploymentContract {
        employee_name: "鈴木一郎".to_string(),
        employer_name: "テスト企業".to_string(),
        employment_type: EmploymentType::IndefiniteTerm,
        work_pattern: WorkPattern::Regular,
        start_date: Utc::now(),
        end_date: None,
        base_wage_jpy: 300_000,
        hours_per_day: 10, // Exceeds statutory 8 hours!
        days_per_week: 6,  // Only 1 day off
        job_description: "General Work".to_string(),
        work_location: "Factory".to_string(),
        probation_period_days: None,
        renewal_count: 0,
    };

    println!(
        "Hours: {} hours/day, {} days/week (PROBLEMATIC)",
        excessive_hours_contract.hours_per_day, excessive_hours_contract.days_per_week
    );
    println!(
        "Weekly Hours: {} hours (exceeds statutory 40 hours)",
        excessive_hours_contract.weekly_hours()
    );

    match validate_employment_contract(&excessive_hours_contract) {
        Ok(()) => println!("\n✅ Validation: PASSED"),
        Err(e) => println!("\n❌ Validation: FAILED (as expected)\n   Error: {}", e),
    }

    println!("\n{}\n", "=".repeat(70));

    // Example 4: Part-time Contract
    println!("📋 Example 4: Part-time Employment Contract (パートタイム雇用)");
    println!("{}", "=".repeat(70));

    let part_time_contract = EmploymentContract {
        employee_name: "高橋美咲".to_string(),
        employer_name: "リテールショップ株式会社".to_string(),
        employment_type: EmploymentType::PartTime,
        work_pattern: WorkPattern::Shift,
        start_date: Utc::now(),
        end_date: None,
        base_wage_jpy: 120_000,
        hours_per_day: 4,
        days_per_week: 5,
        job_description: "店舗スタッフ (Retail Staff)".to_string(),
        work_location: "渋谷店 (Shibuya Store)".to_string(),
        probation_period_days: Some(30),
        renewal_count: 0,
    };

    println!("Employee: {}", part_time_contract.employee_name);
    println!(
        "Type: {:?} ({:?})",
        part_time_contract.employment_type, part_time_contract.work_pattern
    );
    println!(
        "Hours: {} hours/day, {} days/week",
        part_time_contract.hours_per_day, part_time_contract.days_per_week
    );
    println!("Weekly Hours: {} hours", part_time_contract.weekly_hours());

    match validate_employment_contract(&part_time_contract) {
        Ok(()) => println!("\n✅ Validation: PASSED - Part-time contract is valid!"),
        Err(e) => println!("\n❌ Validation: FAILED - {}", e),
    }

    println!("\n{}\n", "=".repeat(70));

    // Example 5: Minimum Wage Check
    println!("📋 Example 5: Minimum Wage Validation (最低賃金検証)");
    println!("{}", "=".repeat(70));

    let tokyo_minimum_wage = 1_113; // Tokyo minimum wage 2024 (example)
    let monthly_hours = valid_contract.hours_per_day * valid_contract.days_per_week * 4;
    let implied_hourly_rate = valid_contract.base_wage_jpy / monthly_hours as u64;

    println!("Base Monthly Wage: ¥{}", valid_contract.base_wage_jpy);
    println!("Monthly Hours: ~{} hours", monthly_hours);
    println!("Implied Hourly Rate: ¥{}", implied_hourly_rate);
    println!("Tokyo Minimum Wage: ¥{}/hour", tokyo_minimum_wage);

    match validate_minimum_wage(implied_hourly_rate, tokyo_minimum_wage) {
        Ok(()) => println!("\n✅ Minimum Wage: COMPLIANT"),
        Err(e) => println!("\n❌ Minimum Wage: VIOLATION - {}", e),
    }

    // Example with below minimum wage
    let low_wage = 900;
    println!("\nTesting with low wage: ¥{}/hour", low_wage);
    match validate_minimum_wage(low_wage, tokyo_minimum_wage) {
        Ok(()) => println!("✅ COMPLIANT"),
        Err(e) => println!("❌ VIOLATION (as expected): {}", e),
    }

    println!("\n{}", "=".repeat(70));
    println!("\n✨ Employment Contract Validation Examples Complete!");
    println!("   All examples demonstrate proper compliance with");
    println!("   Labor Standards Act (労働基準法) and Labor Contract Act (労働契約法).\n");
}
