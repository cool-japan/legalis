//! Employment Contract Validation Example
//!
//! Demonstrates automatic labor law compliance checking for employment contracts.
//!
//! This example shows how to:
//! 1. Validate employment contract data against labor law
//! 2. Check minimum wage compliance (47 prefectures)
//! 3. Validate working hours (Article 32)
//! 4. Check non-compete clause reasonableness (Article 90)
//! 5. Generate compliance reports

use legalis_jp::contract_templates::employment_helper::{
    validate_employment_data, validate_non_compete,
};
use legalis_jp::labor_law::{NonCompeteClause, Prefecture};

fn main() {
    println!("{}", "=".repeat(80));
    println!("Employment Contract Validation Example (雇用契約検証の例)");
    println!("{}", "=".repeat(80));
    println!();

    // Example 1: Compliant Contract (Tokyo)
    println!("【例1】Compliant Employment Contract - Tokyo");
    println!("{}", "-".repeat(80));

    let report1 = validate_employment_data(
        "山田太郎",             // Employee name
        "テクノロジー株式会社", // Employer name
        400_000,                // ¥400,000/month
        8,                      // 8 hours/day
        5,                      // 5 days/week
        Prefecture::Tokyo,      // Tokyo
    )
    .expect("Validation failed");

    println!("Employee: 山田太郎");
    println!("Employer: テクノロジー株式会社");
    println!("Salary: ¥400,000/month");
    println!("Working Hours: 8 hours/day, 5 days/week");
    println!("Location: Tokyo (¥1,113/hour minimum)");
    println!();
    println!("Compliance Result:");
    println!(
        "  Status: {}",
        if report1.is_compliant() {
            "✓ COMPLIANT"
        } else {
            "✗ NON-COMPLIANT"
        }
    );
    println!("  Score: {}/100", report1.score());
    println!("  Violations: {}", report1.violations.len());
    println!("  Warnings: {}", report1.warnings.len());
    println!();

    // Example 2: Below Minimum Wage (Tokyo)
    println!("【例2】Below Minimum Wage - Tokyo");
    println!("{}", "-".repeat(80));

    let report2 = validate_employment_data(
        "佐藤花子",
        "株式会社ABC",
        150_000, // ¥150,000/month - TOO LOW for Tokyo
        8,
        5,
        Prefecture::Tokyo,
    )
    .expect("Validation failed");

    println!("Employee: 佐藤花子");
    println!("Salary: ¥150,000/month");
    println!("Hourly Rate: ~¥865/hour");
    println!("Tokyo Minimum: ¥1,113/hour");
    println!();
    println!("Compliance Result:");
    println!(
        "  Status: {}",
        if report2.is_compliant() {
            "✓ COMPLIANT"
        } else {
            "✗ NON-COMPLIANT"
        }
    );
    println!("  Score: {}/100", report2.score());
    println!("  Violations: {}", report2.violations.len());
    println!();
    if !report2.violations.is_empty() {
        println!("Violations:");
        for (i, violation) in report2.violations.iter().enumerate() {
            println!(
                "  {}. {} ({})",
                i + 1,
                violation.check_name,
                violation.legal_reference
            );
            println!("     {}", violation.description);
        }
    }
    println!();

    // Example 3: Regional Differences (Okinawa vs Tokyo)
    println!("【例3】Regional Minimum Wage Differences");
    println!("{}", "-".repeat(80));

    let salary = 170_000; // ¥170,000/month
    println!("Same Salary (¥170,000/month) in Different Prefectures:");
    println!();

    // Okinawa (lowest minimum wage: ¥896/hour)
    let report_okinawa = validate_employment_data(
        "田中一郎",
        "沖縄株式会社",
        salary,
        8,
        5,
        Prefecture::Okinawa,
    )
    .expect("Validation failed");

    println!("Okinawa (Minimum: ¥896/hour):");
    println!("  Hourly Rate: ~¥981/hour");
    println!(
        "  Status: {}",
        if report_okinawa.is_compliant() {
            "✓ COMPLIANT (¥981 > ¥896)"
        } else {
            "✗ NON-COMPLIANT"
        }
    );
    println!();

    // Tokyo (highest minimum wage: ¥1,113/hour)
    let report_tokyo =
        validate_employment_data("田中一郎", "東京株式会社", salary, 8, 5, Prefecture::Tokyo)
            .expect("Validation failed");

    println!("Tokyo (Minimum: ¥1,113/hour):");
    println!("  Hourly Rate: ~¥981/hour");
    println!(
        "  Status: {}",
        if report_tokyo.is_compliant() {
            "✓ COMPLIANT"
        } else {
            "✗ NON-COMPLIANT (¥981 < ¥1,113)"
        }
    );
    println!();

    // Example 4: Excessive Working Hours
    println!("【例4】Excessive Working Hours (Requires Article 36 Agreement)");
    println!("{}", "-".repeat(80));

    let report4 = validate_employment_data(
        "鈴木次郎",
        "ブラック企業株式会社",
        450_000,
        9, // 9 hours/day - exceeds statutory 8 hours
        6, // 6 days/week - exceeds statutory 5 days
        Prefecture::Tokyo,
    )
    .expect("Validation failed");

    println!("Working Hours: 9 hours/day, 6 days/week = 54 hours/week");
    println!("Statutory Limits: 8 hours/day, 40 hours/week");
    println!();
    println!("Compliance Result:");
    println!(
        "  Status: {}",
        if report4.is_compliant() {
            "✓ COMPLIANT (with warnings)"
        } else {
            "✗ NON-COMPLIANT"
        }
    );
    println!("  Score: {}/100", report4.score());
    println!("  Warnings: {}", report4.warnings.len());
    println!();
    if !report4.warnings.is_empty() {
        println!("Warnings:");
        for (i, warning) in report4.warnings.iter().enumerate() {
            println!("  {}. {}", i + 1, warning.check_name);
            println!("     {}", warning.description);
        }
    }
    println!();

    // Example 5: Non-Compete Clause Validation
    println!("【例5】Non-Compete Clause Validation (競業避止義務の合理性)");
    println!("{}", "-".repeat(80));

    // Reasonable non-compete
    let reasonable_clause = NonCompeteClause {
        duration_months: 6,
        geographic_scope: "東京23区内".to_string(),
        prohibited_activities: vec!["同業種での就業".to_string()],
        consideration_provided: true,
        compensation_amount_jpy: Some(600_000),
    };

    let report_reasonable =
        validate_non_compete(&reasonable_clause, "Software Engineer").expect("Validation failed");

    println!("Reasonable Non-Compete Clause:");
    println!("  Duration: 6 months");
    println!("  Scope: Tokyo 23 wards");
    println!("  Consideration: ¥600,000");
    println!(
        "  Status: {}",
        if report_reasonable.is_compliant() {
            "✓ REASONABLE"
        } else {
            "✗ UNREASONABLE"
        }
    );
    println!();

    // Unreasonable non-compete
    let unreasonable_clause = NonCompeteClause {
        duration_months: 36,                                       // 3 years - excessive
        geographic_scope: "全世界".to_string(),                    // Global - unreasonable
        prohibited_activities: vec!["全ての事業活動".to_string()], // Too broad
        consideration_provided: false,
        compensation_amount_jpy: None,
    };

    let report_unreasonable =
        validate_non_compete(&unreasonable_clause, "Junior Engineer").expect("Validation failed");

    println!("Unreasonable Non-Compete Clause:");
    println!("  Duration: 36 months (3 years)");
    println!("  Scope: Worldwide");
    println!("  Consideration: None");
    println!(
        "  Status: {}",
        if report_unreasonable.is_compliant() {
            "✓ REASONABLE"
        } else {
            "✗ UNREASONABLE (Violates Civil Code Article 90)"
        }
    );
    println!("  Violations: {}", report_unreasonable.violations.len());
    println!();

    // Example 6: Generate Markdown Report
    println!("【例6】Generate Compliance Report (Markdown)");
    println!("{}", "-".repeat(80));

    let report = validate_employment_data(
        "高橋三郎",
        "優良企業株式会社",
        500_000,
        8,
        5,
        Prefecture::Tokyo,
    )
    .expect("Validation failed");

    let markdown = report.to_markdown();
    println!("{}", markdown);

    // Summary
    println!("{}", "=".repeat(80));
    println!("Summary:");
    println!("  - Labor law validation automatically checks minimum wage");
    println!("  - Regional differences enforced (47 prefectures)");
    println!("  - Working hours validated against Article 32");
    println!("  - Non-compete clauses checked for reasonableness");
    println!("  - Compliance reports with scores (0-100)");
    println!("{}", "=".repeat(80));
}
