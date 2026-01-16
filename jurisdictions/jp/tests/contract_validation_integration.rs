//! Contract Validation Integration Tests (契約検証統合テスト)
//!
//! Full pipeline tests demonstrating the integration of:
//! - EmploymentContractBuilder (labor law foundation)
//! - validate_employment_data (validation helpers)
//! - ComplianceReport (reporting system)
//! - Minimum wage enforcement (47 prefectures)
//! - Non-compete validation (Article 90)

use chrono::Utc;
use legalis_jp::contract_templates::employment_helper::{
    validate_employment_data, validate_non_compete,
};
use legalis_jp::labor_law::{
    EmploymentContractBuilder, EmploymentType, NonCompeteClause, Prefecture, WorkPattern,
};

#[test]
fn test_full_pipeline_compliant_contract() {
    // Test: Complete validation pipeline with compliant contract

    let report = validate_employment_data(
        "山田太郎",
        "テクノロジー株式会社",
        400_000, // ¥400,000/month - above Tokyo minimum
        8,       // 8 hours/day - within statutory limit
        5,       // 5 days/week - within statutory limit
        Prefecture::Tokyo,
    )
    .expect("Validation should succeed");

    // Should be fully compliant
    assert!(report.is_compliant(), "Contract should be compliant");
    assert_eq!(report.violations.len(), 0, "Should have no violations");
    assert_eq!(report.warnings.len(), 0, "Should have no warnings");
    assert_eq!(report.score(), 100, "Should have perfect score");

    // All checks should pass
    assert!(
        report.checks_performed.len() >= 3,
        "Should have at least 3 checks performed"
    );
    for check in &report.checks_performed {
        assert!(
            matches!(
                check.status,
                legalis_jp::contract_templates::CheckStatus::Passed
            ),
            "All checks should pass: {}",
            check.check_name
        );
    }
}

#[test]
fn test_minimum_wage_violation_tokyo() {
    // Test: Minimum wage enforcement blocks illegal salary (Tokyo)

    let report = validate_employment_data(
        "佐藤花子",
        "株式会社ABC",
        150_000, // ¥150,000/month - too low for Tokyo
        8,
        5,
        Prefecture::Tokyo,
    )
    .expect("Validation should succeed");

    // Should have violation
    assert!(!report.is_compliant(), "Contract should not be compliant");
    assert!(!report.violations.is_empty(), "Should have violations");

    // Should have minimum wage violation
    let has_minimum_wage_violation = report
        .violations
        .iter()
        .any(|v| v.check_name.contains("Minimum Wage"));
    assert!(
        has_minimum_wage_violation,
        "Should have minimum wage violation"
    );

    // Score should be penalized
    assert!(report.score() < 100, "Score should be less than 100");
    assert!(
        report.score() <= 80,
        "Score should be 80 or less (20 point deduction)"
    );
}

#[test]
fn test_minimum_wage_regional_differences() {
    // Test: Regional minimum wage differences (same salary, different prefectures)

    let salary = 170_000; // ¥170,000/month
    let hours_per_day = 8;
    let days_per_week = 5;

    // Okinawa (lowest minimum wage: ¥896/hour)
    // Expected hourly: ¥170,000 / 173.3 ≈ ¥981/hour
    // ¥981 > ¥896 -> COMPLIANT
    let report_okinawa = validate_employment_data(
        "田中一郎",
        "沖縄株式会社",
        salary,
        hours_per_day,
        days_per_week,
        Prefecture::Okinawa,
    )
    .expect("Validation should succeed");

    assert!(
        report_okinawa.is_compliant(),
        "Should be compliant in Okinawa (¥981 > ¥896)"
    );

    // Tokyo (highest minimum wage: ¥1,113/hour)
    // ¥981 < ¥1,113 -> NON-COMPLIANT
    let report_tokyo = validate_employment_data(
        "田中一郎",
        "東京株式会社",
        salary,
        hours_per_day,
        days_per_week,
        Prefecture::Tokyo,
    )
    .expect("Validation should succeed");

    assert!(
        !report_tokyo.is_compliant(),
        "Should not be compliant in Tokyo (¥981 < ¥1,113)"
    );

    // Same salary -> different compliance based on prefecture
    assert_ne!(
        report_okinawa.is_compliant(),
        report_tokyo.is_compliant(),
        "Same salary should have different compliance in different prefectures"
    );
}

#[test]
fn test_excessive_working_hours_warning() {
    // Test: Excessive working hours trigger Article 36 warning

    let report = validate_employment_data(
        "鈴木次郎",
        "株式会社XYZ",
        450_000,
        9, // 9 hours/day - exceeds statutory 8 hours
        6, // 6 days/week - exceeds statutory 5 days
        Prefecture::Tokyo,
    )
    .expect("Validation should succeed");

    // Should be structurally compliant but with warnings
    assert!(
        report.is_compliant(),
        "Contract should be compliant (structurally valid)"
    );
    assert!(!report.warnings.is_empty(), "Should have warnings");

    // Should have working hours warnings
    let has_daily_warning = report
        .warnings
        .iter()
        .any(|w| w.check_name.contains("Daily Working Hours"));
    let has_weekly_warning = report
        .warnings
        .iter()
        .any(|w| w.check_name.contains("Weekly Working Hours"));

    assert!(
        has_daily_warning || has_weekly_warning,
        "Should have working hours warning"
    );

    // Score should be reduced but not critically
    assert!(report.score() < 100, "Score should be less than 100");
    assert!(
        report.score() >= 60,
        "Score should be at least 60 (warnings only)"
    );
}

#[test]
fn test_non_compete_reasonable() {
    // Test: Reasonable non-compete clause validation

    let clause = NonCompeteClause {
        duration_months: 6,                         // Short duration - reasonable
        geographic_scope: "東京23区内".to_string(), // Limited scope - reasonable
        prohibited_activities: vec!["同業種での就業".to_string()], // Specific - reasonable
        consideration_provided: true,               // Compensation provided
        compensation_amount_jpy: Some(600_000),     // Adequate compensation
    };

    let report =
        validate_non_compete(&clause, "Software Engineer").expect("Validation should succeed");

    assert!(
        report.is_compliant(),
        "Reasonable clause should be compliant"
    );
    assert_eq!(report.violations.len(), 0, "Should have no violations");
    assert_eq!(report.score(), 100, "Should have perfect score");
}

#[test]
fn test_non_compete_unreasonable() {
    // Test: Unreasonable non-compete clause detection

    let clause = NonCompeteClause {
        duration_months: 36,                                       // 3 years - excessive
        geographic_scope: "全世界".to_string(),                    // Global - unreasonable
        prohibited_activities: vec!["全ての事業活動".to_string()], // Too broad
        consideration_provided: false,                             // No compensation
        compensation_amount_jpy: None,
    };

    let report =
        validate_non_compete(&clause, "Junior Engineer").expect("Validation should succeed");

    assert!(
        !report.is_compliant(),
        "Unreasonable clause should not be compliant"
    );
    assert!(!report.violations.is_empty(), "Should have violations");

    // Score should be heavily penalized
    assert!(report.score() < 80, "Score should be significantly reduced");
}

#[test]
fn test_builder_integration_with_validation() {
    // Test: EmploymentContractBuilder integration with validation

    // Build contract using builder
    let contract = EmploymentContractBuilder::new()
        .with_employee("高橋三郎")
        .with_employer("優良企業株式会社")
        .with_salary(500_000)
        .with_working_hours(8, 5)
        .with_prefecture(Prefecture::Tokyo)
        .with_employment_type(EmploymentType::IndefiniteTerm)
        .with_work_pattern(WorkPattern::Regular)
        .with_start_date(Utc::now())
        .with_job_description("Senior Engineer")
        .with_work_location("Tokyo Office")
        .build()
        .expect("Contract build should succeed");

    // Validate the same parameters
    let report = validate_employment_data(
        "高橋三郎",
        "優良企業株式会社",
        500_000,
        8,
        5,
        Prefecture::Tokyo,
    )
    .expect("Validation should succeed");

    // Both builder and validation should succeed
    assert!(report.is_compliant(), "Validation should pass");
    assert_eq!(contract.employee_name, "高橋三郎");
    assert_eq!(contract.base_wage_jpy, 500_000);
}

#[test]
fn test_compliance_report_scoring() {
    // Test: Compliance report scoring system

    // Perfect score (100)
    let perfect = validate_employment_data("太郎", "株式会社A", 400_000, 8, 5, Prefecture::Tokyo)
        .expect("Should succeed");
    assert_eq!(perfect.score(), 100, "Perfect contract should score 100");

    // With warnings (95)
    let with_warning =
        validate_employment_data("花子", "株式会社B", 450_000, 9, 5, Prefecture::Tokyo)
            .expect("Should succeed");
    assert!(with_warning.score() < 100, "Should be less than 100");
    assert!(
        with_warning.score() >= 90,
        "Should be at least 90 (one warning = -5)"
    );

    // With violation (80 or less)
    let with_violation =
        validate_employment_data("次郎", "株式会社C", 150_000, 8, 5, Prefecture::Tokyo)
            .expect("Should succeed");
    assert!(
        with_violation.score() <= 80,
        "Should be 80 or less (violation = -20)"
    );
}

#[test]
fn test_markdown_report_generation() {
    // Test: Markdown report generation

    let report = validate_employment_data(
        "レポート太郎",
        "報告株式会社",
        400_000,
        8,
        5,
        Prefecture::Tokyo,
    )
    .expect("Validation should succeed");

    let markdown = report.to_markdown();

    // Should contain key sections
    assert!(
        markdown.contains("労働法コンプライアンスレポート"),
        "Should contain report title"
    );
    assert!(
        markdown.contains("Employment Contract"),
        "Should contain contract type"
    );
    assert!(markdown.contains("スコア"), "Should contain score");
    assert!(
        markdown.contains("100/100") || markdown.contains("Score"),
        "Should contain score value"
    );
}

#[test]
fn test_multiple_prefectures_minimum_wage() {
    // Test: Multiple prefectures minimum wage enforcement

    let test_cases = vec![
        (Prefecture::Tokyo, 400_000, true),     // High salary in Tokyo
        (Prefecture::Osaka, 350_000, true),     // Medium salary in Osaka
        (Prefecture::Okinawa, 180_000, true),   // Lower salary in Okinawa (still compliant)
        (Prefecture::Tokyo, 150_000, false),    // Too low for Tokyo
        (Prefecture::Kanagawa, 160_000, false), // Too low for Kanagawa
    ];

    for (prefecture, salary, should_comply) in test_cases {
        let report = validate_employment_data("従業員", "会社", salary, 8, 5, prefecture)
            .expect("Validation should succeed");

        assert_eq!(
            report.is_compliant(),
            should_comply,
            "Prefecture {:?} with salary {} should be compliant: {}",
            prefecture,
            salary,
            should_comply
        );
    }
}

#[test]
fn test_contract_structure_validation() {
    // Test: Contract structure validation via builder

    // Valid structure
    let valid = EmploymentContractBuilder::new()
        .with_employee("従業員")
        .with_employer("会社")
        .with_salary(400_000)
        .with_working_hours(8, 5)
        .with_prefecture(Prefecture::Tokyo)
        .with_employment_type(EmploymentType::IndefiniteTerm)
        .with_work_pattern(WorkPattern::Regular)
        .with_start_date(Utc::now())
        .with_job_description("Job")
        .with_work_location("Location")
        .build();

    assert!(valid.is_ok(), "Valid structure should build successfully");

    // Invalid structure (missing required fields) - should fail at build time
    let invalid = EmploymentContractBuilder::new()
        .with_employee("従業員")
        // Missing employer
        .with_salary(400_000)
        .build();

    assert!(invalid.is_err(), "Invalid structure should fail to build");
}

#[test]
fn test_warning_deduction_calculation() {
    // Test: Warning score deduction calculation

    // 2 warnings: daily hours + weekly hours
    let report = validate_employment_data("従業員", "会社", 450_000, 10, 6, Prefecture::Tokyo)
        .expect("Should succeed");

    // Should have at least 2 warnings
    assert!(
        report.warnings.len() >= 2,
        "Should have at least 2 warnings"
    );

    // Score calculation: 100 - (warnings * 5), minimum 60
    let expected_max = 100 - (report.warnings.len() as u32 * 5);
    let expected_max = expected_max.max(60);

    assert!(
        report.score() <= expected_max,
        "Score should be at most {} but got {}",
        expected_max,
        report.score()
    );
    assert!(report.score() >= 60, "Score should be at least 60");
}

#[test]
fn test_violation_deduction_calculation() {
    // Test: Violation score deduction calculation

    // Should have minimum wage violation
    let report = validate_employment_data("従業員", "会社", 100_000, 8, 5, Prefecture::Tokyo)
        .expect("Should succeed");

    assert!(
        !report.violations.is_empty(),
        "Should have at least one violation"
    );

    // Score calculation: 100 - (violations * 20), minimum 0
    let expected_max = 100u32.saturating_sub(report.violations.len() as u32 * 20);

    assert!(
        report.score() <= expected_max,
        "Score should be at most {} but got {}",
        expected_max,
        report.score()
    );
    assert!(report.score() <= 80, "Should be 80 or less with violation");
}

#[test]
fn test_non_compete_with_no_consideration() {
    // Test: Non-compete without consideration should fail

    let clause = NonCompeteClause {
        duration_months: 12,
        geographic_scope: "東京都".to_string(),
        prohibited_activities: vec!["競合他社への就職".to_string()],
        consideration_provided: false, // No compensation
        compensation_amount_jpy: None,
    };

    let report = validate_non_compete(&clause, "Manager").expect("Should succeed");

    // Should have violations or warnings about lack of consideration
    assert!(
        !report.is_compliant() || !report.warnings.is_empty(),
        "No consideration should result in violation or warning"
    );
}

#[test]
fn test_integration_all_validation_types() {
    // Test: Complete integration of all validation types

    // 1. Compliant employment contract
    let employment_report =
        validate_employment_data("統合太郎", "総合株式会社", 500_000, 8, 5, Prefecture::Tokyo)
            .expect("Should succeed");

    assert!(employment_report.is_compliant());

    // 2. Reasonable non-compete
    let non_compete = NonCompeteClause {
        duration_months: 6,
        geographic_scope: "東京23区".to_string(),
        prohibited_activities: vec!["直接競合".to_string()],
        consideration_provided: true,
        compensation_amount_jpy: Some(500_000),
    };

    let non_compete_report =
        validate_non_compete(&non_compete, "Engineer").expect("Should succeed");

    assert!(non_compete_report.is_compliant());

    // 3. Both should have high scores (compliant, may have minor warnings)
    assert!(
        employment_report.score() >= 90,
        "Employment report score should be at least 90"
    );
    assert!(
        non_compete_report.score() >= 90,
        "Non-compete report score should be at least 90"
    );

    // 4. Both should generate markdown reports
    let employment_md = employment_report.to_markdown();
    let non_compete_md = non_compete_report.to_markdown();

    assert!(!employment_md.is_empty());
    assert!(!non_compete_md.is_empty());
}
