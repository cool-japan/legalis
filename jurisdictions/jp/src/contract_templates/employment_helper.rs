//! Employment Contract Validation Helper (雇用契約検証ヘルパー)
//!
//! Helper functions to validate employment contract data using labor law rules.
//!
//! # Example
//!
//! ```
//! use legalis_jp::contract_templates::employment_helper::validate_employment_data;
//! use legalis_jp::labor_law::Prefecture;
//!
//! let result = validate_employment_data(
//!     "山田太郎",
//!     "テクノロジー株式会社",
//!     400_000,
//!     8,
//!     5,
//!     Prefecture::Tokyo,
//! );
//!
//! assert!(result.is_ok());
//! let report = result.unwrap();
//! assert!(report.is_compliant());
//! ```

use super::compliance::{
    CheckStatus, ComplianceCheck, ComplianceReport, ComplianceViolation, ComplianceWarning,
};
use crate::labor_law::{
    EmploymentContractBuilder, EmploymentType, NonCompeteClause, Prefecture, WorkPattern,
    minimum_wage, validate_non_compete_reasonableness,
};
use chrono::Utc;

/// Validate employment contract data (雇用契約データ検証)
///
/// Validates employment contract parameters against Japanese labor law.
///
/// # Arguments
///
/// * `employee_name` - Employee name
/// * `employer_name` - Employer name
/// * `base_salary_jpy` - Monthly base salary in JPY
/// * `hours_per_day` - Working hours per day
/// * `days_per_week` - Working days per week
/// * `prefecture` - Work location prefecture
///
/// # Returns
///
/// ComplianceReport with validation results
///
/// # Example
///
/// ```
/// use legalis_jp::contract_templates::employment_helper::validate_employment_data;
/// use legalis_jp::labor_law::Prefecture;
///
/// let report = validate_employment_data(
///     "佐藤花子",
///     "株式会社XYZ",
///     350_000,
///     8,
///     5,
///     Prefecture::Tokyo,
/// ).unwrap();
///
/// println!("Score: {}/100", report.score());
/// ```
pub fn validate_employment_data(
    employee_name: &str,
    employer_name: &str,
    base_salary_jpy: u64,
    hours_per_day: u32,
    days_per_week: u32,
    prefecture: Prefecture,
) -> Result<ComplianceReport, String> {
    let mut report = ComplianceReport::new("Employment Contract");

    // 1. Build employment contract for structural validation
    let builder = EmploymentContractBuilder::new()
        .with_employee(employee_name)
        .with_employer(employer_name)
        .with_salary(base_salary_jpy)
        .with_working_hours(hours_per_day, days_per_week)
        .with_prefecture(prefecture)
        .with_employment_type(EmploymentType::IndefiniteTerm)
        .with_work_pattern(WorkPattern::Regular)
        .with_start_date(Utc::now())
        .with_job_description("General Employment")
        .with_work_location("Workplace");

    // Try to build contract
    match builder.build() {
        Ok(_contract) => {
            report.add_check(ComplianceCheck {
                check_name: "Contract Structure".to_string(),
                legal_reference: "Labor Standards Act Article 15".to_string(),
                status: CheckStatus::Passed,
                details: "Employment contract structure is valid".to_string(),
            });
        }
        Err(e) => {
            report.add_check(ComplianceCheck {
                check_name: "Contract Structure".to_string(),
                legal_reference: "Labor Standards Act Article 15".to_string(),
                status: CheckStatus::Failed,
                details: format!("Invalid contract structure: {}", e),
            });
            return Ok(report); // Return early if structure is invalid
        }
    }

    // 2. Validate minimum wage
    let monthly_hours = minimum_wage::calculate_monthly_hours(hours_per_day, days_per_week);
    let hourly_rate = base_salary_jpy as f64 / monthly_hours;
    let min_wage = minimum_wage::get_minimum_wage(prefecture, Utc::now().date_naive());

    if (hourly_rate as u64) < min_wage {
        report.add_check(ComplianceCheck {
            check_name: "Minimum Wage Check".to_string(),
            legal_reference: format!(
                "Minimum Wage Act - {} Prefecture",
                prefecture_name(prefecture)
            ),
            status: CheckStatus::Failed,
            details: format!(
                "Hourly rate ¥{} is below minimum wage ¥{} for {}",
                hourly_rate as u64,
                min_wage,
                prefecture_name(prefecture)
            ),
        });
    } else {
        report.add_check(ComplianceCheck {
            check_name: "Minimum Wage Check".to_string(),
            legal_reference: format!(
                "Minimum Wage Act - {} Prefecture",
                prefecture_name(prefecture)
            ),
            status: CheckStatus::Passed,
            details: format!(
                "Hourly rate ¥{}/時 meets {} minimum wage ¥{}/時",
                hourly_rate as u64,
                prefecture_name(prefecture),
                min_wage
            ),
        });
    }

    // 3. Validate statutory working hours
    if hours_per_day > 8 {
        report.add_check(ComplianceCheck {
            check_name: "Daily Working Hours".to_string(),
            legal_reference: "Labor Standards Act Article 32".to_string(),
            status: CheckStatus::Warning,
            details: format!(
                "Daily hours {} exceed statutory 8 hours. Article 36 Agreement required.",
                hours_per_day
            ),
        });
    } else {
        report.add_check(ComplianceCheck {
            check_name: "Daily Working Hours".to_string(),
            legal_reference: "Labor Standards Act Article 32".to_string(),
            status: CheckStatus::Passed,
            details: format!(
                "Daily hours {} within statutory limit of 8 hours",
                hours_per_day
            ),
        });
    }

    let weekly_hours = hours_per_day * days_per_week;
    if weekly_hours > 40 {
        report.add_check(ComplianceCheck {
            check_name: "Weekly Working Hours".to_string(),
            legal_reference: "Labor Standards Act Article 32".to_string(),
            status: CheckStatus::Warning,
            details: format!(
                "Weekly hours {} exceed statutory 40 hours. Article 36 Agreement required.",
                weekly_hours
            ),
        });
    } else {
        report.add_check(ComplianceCheck {
            check_name: "Weekly Working Hours".to_string(),
            legal_reference: "Labor Standards Act Article 32".to_string(),
            status: CheckStatus::Passed,
            details: format!(
                "Weekly hours {} within statutory limit of 40 hours",
                weekly_hours
            ),
        });
    }

    Ok(report)
}

/// Validate non-compete clause in employment context (競業避止条項検証)
///
/// # Example
///
/// ```
/// use legalis_jp::contract_templates::employment_helper::validate_non_compete;
/// use legalis_jp::labor_law::NonCompeteClause;
///
/// let clause = NonCompeteClause {
///     duration_months: 12,
///     geographic_scope: "東京都内".to_string(),
///     prohibited_activities: vec!["同業種での就業".to_string()],
///     consideration_provided: true,
///     compensation_amount_jpy: Some(1_200_000),
/// };
///
/// let report = validate_non_compete(&clause, "Engineer").unwrap();
/// assert!(report.is_compliant());
/// ```
pub fn validate_non_compete(
    clause: &NonCompeteClause,
    employee_position: &str,
) -> Result<ComplianceReport, String> {
    let mut report = ComplianceReport::new("Non-Compete Clause");

    match validate_non_compete_reasonableness(clause, employee_position) {
        Ok(reasonableness_report) => {
            if reasonableness_report.is_reasonable() {
                report.add_check(ComplianceCheck {
                    check_name: "Non-Compete Reasonableness".to_string(),
                    legal_reference: "Civil Code Article 90".to_string(),
                    status: CheckStatus::Passed,
                    details: format!(
                        "Non-compete clause is reasonable ({} positive factors)",
                        reasonableness_report.positive_factors.len()
                    ),
                });
            } else {
                // Add violations from reasonableness report
                for issue in &reasonableness_report.issues {
                    report.add_violation(ComplianceViolation {
                        check_name: "Non-Compete Reasonableness".to_string(),
                        legal_reference: "Civil Code Article 90 (Public Policy)".to_string(),
                        description: issue.clone(),
                    });
                }
            }

            // Add warnings
            for warning in &reasonableness_report.warnings {
                report.add_warning(ComplianceWarning {
                    check_name: "Non-Compete Warning".to_string(),
                    description: warning.clone(),
                });
            }
        }
        Err(e) => {
            report.add_check(ComplianceCheck {
                check_name: "Non-Compete Validation".to_string(),
                legal_reference: "Civil Code Article 90".to_string(),
                status: CheckStatus::Failed,
                details: format!("Validation error: {}", e),
            });
        }
    }

    Ok(report)
}

/// Get prefecture name in Japanese (都道府県名取得)
fn prefecture_name(prefecture: Prefecture) -> &'static str {
    match prefecture {
        Prefecture::Tokyo => "東京都",
        Prefecture::Osaka => "大阪府",
        Prefecture::Kanagawa => "神奈川県",
        Prefecture::Saitama => "埼玉県",
        Prefecture::Chiba => "千葉県",
        Prefecture::Aichi => "愛知県",
        Prefecture::Hokkaido => "北海道",
        Prefecture::Fukuoka => "福岡県",
        Prefecture::Kyoto => "京都府",
        Prefecture::Hyogo => "兵庫県",
        _ => "その他",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_compliant_contract() {
        let report = validate_employment_data(
            "山田太郎",
            "テクノロジー株式会社",
            400_000, // ¥400,000/month
            8,       // 8 hours/day
            5,       // 5 days/week
            Prefecture::Tokyo,
        )
        .unwrap();

        assert!(report.is_compliant());
        assert_eq!(report.violations.len(), 0);
        assert!(report.score() >= 95); // Perfect or near-perfect
    }

    #[test]
    fn test_validate_below_minimum_wage() {
        let report = validate_employment_data(
            "佐藤花子",
            "株式会社ABC",
            150_000, // Too low for Tokyo
            8,
            5,
            Prefecture::Tokyo,
        )
        .unwrap();

        assert!(!report.is_compliant());
        assert!(!report.violations.is_empty());
        assert!(
            report
                .violations
                .iter()
                .any(|v| v.check_name.contains("Minimum Wage"))
        );
    }

    #[test]
    fn test_validate_excessive_hours() {
        let report = validate_employment_data(
            "田中一郎",
            "株式会社XYZ",
            400_000,
            9, // 9 hours/day - exceeds statutory 8
            5,
            Prefecture::Tokyo,
        )
        .unwrap();

        // Should be compliant but with warnings
        assert!(report.is_compliant());
        assert!(!report.warnings.is_empty());
        assert!(
            report
                .warnings
                .iter()
                .any(|w| w.check_name.contains("Daily Working Hours"))
        );
    }

    #[test]
    fn test_validate_non_compete_reasonable() {
        let clause = NonCompeteClause {
            duration_months: 6,
            geographic_scope: "東京23区内".to_string(),
            prohibited_activities: vec!["同業種での就業".to_string()],
            consideration_provided: true,
            compensation_amount_jpy: Some(600_000),
        };

        let report = validate_non_compete(&clause, "Software Engineer").unwrap();

        assert!(report.is_compliant());
        assert_eq!(report.violations.len(), 0);
    }

    #[test]
    fn test_validate_non_compete_unreasonable() {
        let clause = NonCompeteClause {
            duration_months: 36,                                       // 3 years - excessive
            geographic_scope: "全世界".to_string(),                    // Global - unreasonable
            prohibited_activities: vec!["全ての事業活動".to_string()], // Too broad
            consideration_provided: false,                             // No consideration
            compensation_amount_jpy: None,
        };

        let report = validate_non_compete(&clause, "Junior Engineer").unwrap();

        assert!(!report.is_compliant());
        assert!(!report.violations.is_empty());
    }

    #[test]
    fn test_prefecture_name() {
        assert_eq!(prefecture_name(Prefecture::Tokyo), "東京都");
        assert_eq!(prefecture_name(Prefecture::Osaka), "大阪府");
        assert_eq!(prefecture_name(Prefecture::Hokkaido), "北海道");
    }

    #[test]
    fn test_markdown_report_generation() {
        let report = validate_employment_data(
            "山田太郎",
            "テクノロジー株式会社",
            400_000,
            8,
            5,
            Prefecture::Tokyo,
        )
        .unwrap();

        let markdown = report.to_markdown();
        assert!(markdown.contains("労働法コンプライアンスレポート"));
        assert!(markdown.contains("Employment Contract"));
    }
}
