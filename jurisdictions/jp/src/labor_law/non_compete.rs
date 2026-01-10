//! Non-Compete Clause Reasonableness Validation (競業避止義務の合理性検証)
//!
//! Implementation of non-compete clause validation under Japanese law,
//! particularly Civil Code Article 90 (public policy) and case law precedents.
//!
//! # Legal Basis
//!
//! ## Civil Code Article 90 (民法第90条 - Public Policy)
//!
//! Contracts that violate public policy are void. Non-compete clauses are
//! subject to strict scrutiny to ensure they don't unreasonably restrict
//! an individual's right to work (職業選択の自由 - freedom of occupation).
//!
//! ## Reasonableness Factors (合理性の判断要素)
//!
//! Courts consider:
//! 1. **Duration** (期間) - Typically 6-12 months maximum, rarely beyond 2 years
//! 2. **Geographic Scope** (地域的範囲) - Must be reasonably limited
//! 3. **Prohibited Activities** (禁止業務の範囲) - Must be specific, not overly broad
//! 4. **Consideration** (代償措置) - Compensation must be provided
//! 5. **Employee Position** (従業員の地位) - Higher for executives/key personnel
//! 6. **Legitimate Business Interest** (正当な事業上の利益) - Protection of trade secrets, etc.
//!
//! # Example
//!
//! ```
//! use legalis_jp::labor_law::{NonCompeteClause, validate_non_compete_reasonableness};
//!
//! let clause = NonCompeteClause {
//!     duration_months: 12,
//!     geographic_scope: "東京都内".to_string(),
//!     prohibited_activities: vec!["同業他社への転職".to_string()],
//!     consideration_provided: true,
//!     compensation_amount_jpy: Some(1_200_000), // 1 year of partial compensation
//! };
//!
//! let report = validate_non_compete_reasonableness(&clause, "Senior Engineer").unwrap();
//! assert!(report.is_reasonable());
//! ```

use super::error::{LaborLawError, Result};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Typical maximum duration for non-compete (months)
/// (競業避止義務の標準上限期間)
pub const TYPICAL_MAX_DURATION_MONTHS: u32 = 12;

/// Absolute maximum duration (rarely enforceable beyond this)
/// (絶対的上限期間)
pub const ABSOLUTE_MAX_DURATION_MONTHS: u32 = 24;

/// Non-compete clause structure (競業避止条項)
///
/// Represents a non-compete agreement restricting employee activities
/// after termination of employment.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct NonCompeteClause {
    /// Duration of restriction in months (制限期間)
    pub duration_months: u32,

    /// Geographic scope of restriction (地域的範囲)
    /// Examples: "東京都内", "関東圏", "全国"
    pub geographic_scope: String,

    /// List of prohibited activities (禁止業務)
    /// Should be specific, not overly broad
    pub prohibited_activities: Vec<String>,

    /// Whether consideration is provided (代償措置の有無)
    pub consideration_provided: bool,

    /// Amount of compensation in JPY (代償金額)
    /// Typically monthly payments during restriction period
    pub compensation_amount_jpy: Option<u64>,
}

/// Reasonableness assessment report (合理性判定レポート)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ReasonablenessReport {
    /// Overall reasonableness assessment (総合判定)
    pub is_reasonable: bool,

    /// Identified issues (問題点)
    pub issues: Vec<String>,

    /// Warnings (警告)
    pub warnings: Vec<String>,

    /// Positive factors (肯定的要素)
    pub positive_factors: Vec<String>,

    /// Enforceability risk level (執行可能性リスクレベル)
    pub risk_level: RiskLevel,
}

/// Risk level for enforceability (執行可能性のリスクレベル)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RiskLevel {
    /// Low risk - likely enforceable (低リスク)
    Low,
    /// Medium risk - enforceability uncertain (中リスク)
    Medium,
    /// High risk - likely unenforceable (高リスク)
    High,
    /// Critical - almost certainly void (致命的)
    Critical,
}

impl ReasonablenessReport {
    /// Check if clause is reasonable (合理的か)
    pub fn is_reasonable(&self) -> bool {
        self.is_reasonable
    }

    /// Check if there are critical issues (致命的問題があるか)
    pub fn has_critical_issues(&self) -> bool {
        self.risk_level == RiskLevel::Critical || self.risk_level == RiskLevel::High
    }

    /// Get issue count (問題数)
    pub fn issue_count(&self) -> usize {
        self.issues.len()
    }

    /// Generate markdown report (レポート生成)
    pub fn to_markdown(&self) -> String {
        let mut report = String::new();

        report.push_str("# 競業避止条項合理性判定レポート\n\n");

        // Overall assessment
        let assessment = if self.is_reasonable {
            "✓ 合理的 (Reasonable)"
        } else {
            "✗ 不合理 (Unreasonable)"
        };
        report.push_str(&format!("**総合判定**: {}\n\n", assessment));

        // Risk level
        let risk = match self.risk_level {
            RiskLevel::Low => "低 (Low)",
            RiskLevel::Medium => "中 (Medium)",
            RiskLevel::High => "高 (High)",
            RiskLevel::Critical => "致命的 (Critical)",
        };
        report.push_str(&format!("**リスクレベル**: {}\n\n", risk));

        // Issues
        if !self.issues.is_empty() {
            report.push_str("## 問題点 (Issues)\n\n");
            for issue in &self.issues {
                report.push_str(&format!("- ✗ {}\n", issue));
            }
            report.push('\n');
        }

        // Warnings
        if !self.warnings.is_empty() {
            report.push_str("## 警告 (Warnings)\n\n");
            for warning in &self.warnings {
                report.push_str(&format!("- ⚠ {}\n", warning));
            }
            report.push('\n');
        }

        // Positive factors
        if !self.positive_factors.is_empty() {
            report.push_str("## 肯定的要素 (Positive Factors)\n\n");
            for factor in &self.positive_factors {
                report.push_str(&format!("- ✓ {}\n", factor));
            }
            report.push('\n');
        }

        report
    }
}

/// Validate non-compete clause reasonableness (競業避止条項の合理性検証)
///
/// Assesses whether a non-compete clause is likely to be enforceable
/// under Japanese law, considering case law precedents.
///
/// # Arguments
///
/// * `clause` - The non-compete clause to validate
/// * `employee_position` - Employee's position (e.g., "Software Engineer", "Executive")
///
/// # Returns
///
/// A detailed reasonableness report with assessment and recommendations.
///
/// # Example
///
/// ```
/// use legalis_jp::labor_law::{NonCompeteClause, validate_non_compete_reasonableness};
///
/// let clause = NonCompeteClause {
///     duration_months: 6,
///     geographic_scope: "東京23区内".to_string(),
///     prohibited_activities: vec!["同業種での就業".to_string()],
///     consideration_provided: true,
///     compensation_amount_jpy: Some(600_000),
/// };
///
/// let report = validate_non_compete_reasonableness(&clause, "Engineer").unwrap();
/// assert!(report.is_reasonable());
/// ```
pub fn validate_non_compete_reasonableness(
    clause: &NonCompeteClause,
    employee_position: &str,
) -> Result<ReasonablenessReport> {
    let mut issues = Vec::new();
    let mut warnings = Vec::new();
    let mut positive_factors = Vec::new();
    let mut risk_score: u32 = 0; // 0-10, higher = more risky

    // 1. Duration validation (期間の検証)
    if clause.duration_months == 0 {
        return Err(LaborLawError::ValidationError {
            message: "Non-compete duration cannot be zero".to_string(),
        });
    }

    if clause.duration_months > ABSOLUTE_MAX_DURATION_MONTHS {
        issues.push(format!(
            "Duration {} months exceeds absolute maximum {} months - likely void under Civil Code Article 90",
            clause.duration_months, ABSOLUTE_MAX_DURATION_MONTHS
        ));
        risk_score += 5; // Critical issue - exceeds absolute maximum
    } else if clause.duration_months > TYPICAL_MAX_DURATION_MONTHS {
        warnings.push(format!(
            "Duration {} months exceeds typical maximum {} months - enforceability questionable",
            clause.duration_months, TYPICAL_MAX_DURATION_MONTHS
        ));
        risk_score += 2;
    } else if clause.duration_months <= 6 {
        positive_factors.push(format!(
            "Short duration ({} months) increases enforceability",
            clause.duration_months
        ));
    }

    // 2. Consideration validation (代償措置の検証)
    if !clause.consideration_provided {
        issues.push(
            "No consideration provided - non-compete likely unenforceable without compensation"
                .to_string(),
        );
        risk_score += 3;
    } else {
        positive_factors.push("Consideration provided enhances enforceability".to_string());

        // Check compensation adequacy
        if let Some(compensation) = clause.compensation_amount_jpy {
            let monthly_compensation = compensation / clause.duration_months as u64;
            if monthly_compensation < 100_000 {
                warnings.push(format!(
                    "Monthly compensation ¥{} may be insufficient for {} months restriction",
                    monthly_compensation, clause.duration_months
                ));
                risk_score += 1;
            } else {
                positive_factors.push(format!(
                    "Adequate monthly compensation (¥{}) provided",
                    monthly_compensation
                ));
            }
        } else {
            warnings
                .push("Consideration type not specified (compensation amount missing)".to_string());
        }
    }

    // 3. Geographic scope validation (地域的範囲の検証)
    let scope_lower = clause.geographic_scope.to_lowercase();

    if scope_lower.contains("全世界") || scope_lower.contains("worldwide") {
        issues.push("Global geographic scope is typically unreasonable".to_string());
        risk_score += 3;
    } else if scope_lower.contains("全国") || scope_lower.contains("nationwide") {
        warnings.push(
            "Nationwide scope may be excessive unless justified by business necessity".to_string(),
        );
        risk_score += 2;
    } else if scope_lower.contains("都内")
        || scope_lower.contains("区内")
        || scope_lower.contains("市内")
    {
        positive_factors.push("Limited geographic scope (city/ward level)".to_string());
    }

    // 4. Prohibited activities validation (禁止業務の検証)
    if clause.prohibited_activities.is_empty() {
        return Err(LaborLawError::ValidationError {
            message: "Prohibited activities cannot be empty".to_string(),
        });
    }

    if clause.prohibited_activities.len() > 10 {
        warnings.push(
            "Large number of prohibited activities may be considered overly broad".to_string(),
        );
        risk_score += 1;
    }

    // Check for overly broad prohibitions
    for activity in &clause.prohibited_activities {
        let activity_lower = activity.to_lowercase();
        if activity_lower.contains("全て") || activity_lower.contains("any") {
            issues.push(format!(
                "Overly broad prohibition: '{}' - must be specific",
                activity
            ));
            risk_score += 2;
        }
    }

    if clause.prohibited_activities.len() <= 3 {
        positive_factors.push("Limited, specific prohibited activities".to_string());
    }

    // 5. Employee position consideration (従業員の地位の考慮)
    let position_lower = employee_position.to_lowercase();
    let is_executive = position_lower.contains("executive")
        || position_lower.contains("取締役")
        || position_lower.contains("役員")
        || position_lower.contains("ceo")
        || position_lower.contains("cto");

    if is_executive {
        positive_factors
            .push("Executive/key personnel position justifies stronger restrictions".to_string());
        risk_score = risk_score.saturating_sub(1); // Reduce risk for executives
    } else if position_lower.contains("junior") || position_lower.contains("新卒") {
        warnings
            .push("Junior employee position may make restriction harder to justify".to_string());
        risk_score += 1;
    }

    // Determine overall risk level
    let risk_level = match risk_score {
        0..=2 => RiskLevel::Low,
        3..=4 => RiskLevel::Medium,
        5..=7 => RiskLevel::High,
        _ => RiskLevel::Critical,
    };

    // Overall reasonableness: reasonable if no critical issues
    let is_reasonable = issues.is_empty() && risk_level != RiskLevel::Critical;

    Ok(ReasonablenessReport {
        is_reasonable,
        issues,
        warnings,
        positive_factors,
        risk_level,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reasonable_clause() {
        let clause = NonCompeteClause {
            duration_months: 6,
            geographic_scope: "東京23区内".to_string(),
            prohibited_activities: vec!["同業種での就業".to_string()],
            consideration_provided: true,
            compensation_amount_jpy: Some(600_000),
        };

        let report = validate_non_compete_reasonableness(&clause, "Software Engineer").unwrap();

        assert!(report.is_reasonable());
        assert_eq!(report.risk_level, RiskLevel::Low);
        assert!(report.issues.is_empty());
    }

    #[test]
    fn test_excessive_duration() {
        let clause = NonCompeteClause {
            duration_months: 36, // 3 years - excessive
            geographic_scope: "東京都内".to_string(),
            prohibited_activities: vec!["競合他社への転職".to_string()],
            consideration_provided: true,
            compensation_amount_jpy: Some(3_600_000),
        };

        let report = validate_non_compete_reasonableness(&clause, "Engineer").unwrap();

        assert!(!report.is_reasonable());
        assert!(report.has_critical_issues());
        assert!(!report.issues.is_empty());
    }

    #[test]
    fn test_no_consideration() {
        let clause = NonCompeteClause {
            duration_months: 12,
            geographic_scope: "関東圏".to_string(),
            prohibited_activities: vec!["同業種での就業".to_string()],
            consideration_provided: false,
            compensation_amount_jpy: None,
        };

        let report = validate_non_compete_reasonableness(&clause, "Manager").unwrap();

        assert!(!report.is_reasonable());
        assert!(report.issues.iter().any(|i| i.contains("consideration")));
    }

    #[test]
    fn test_global_scope() {
        let clause = NonCompeteClause {
            duration_months: 12,
            geographic_scope: "全世界".to_string(),
            prohibited_activities: vec!["IT業界での就業".to_string()],
            consideration_provided: true,
            compensation_amount_jpy: Some(1_200_000),
        };

        let report = validate_non_compete_reasonableness(&clause, "Engineer").unwrap();

        assert!(!report.is_reasonable());
        assert!(report.issues.iter().any(|i| i.contains("Global")));
    }

    #[test]
    fn test_overly_broad_activities() {
        let clause = NonCompeteClause {
            duration_months: 12,
            geographic_scope: "東京都内".to_string(),
            prohibited_activities: vec!["全ての事業活動".to_string()],
            consideration_provided: true,
            compensation_amount_jpy: Some(1_200_000),
        };

        let report = validate_non_compete_reasonableness(&clause, "Engineer").unwrap();

        assert!(!report.is_reasonable());
        assert!(report.issues.iter().any(|i| i.contains("broad")));
    }

    #[test]
    fn test_executive_position_favorable() {
        let clause = NonCompeteClause {
            duration_months: 12,
            geographic_scope: "全国".to_string(), // Nationwide but executive
            prohibited_activities: vec!["競合企業での経営活動".to_string()],
            consideration_provided: true,
            compensation_amount_jpy: Some(2_400_000),
        };

        let report = validate_non_compete_reasonableness(&clause, "取締役").unwrap();

        // Executive position + consideration makes nationwide scope more reasonable
        assert!(
            report
                .positive_factors
                .iter()
                .any(|f| f.contains("Executive"))
        );
    }

    #[test]
    fn test_zero_duration_error() {
        let clause = NonCompeteClause {
            duration_months: 0,
            geographic_scope: "東京都内".to_string(),
            prohibited_activities: vec!["同業種での就業".to_string()],
            consideration_provided: true,
            compensation_amount_jpy: Some(600_000),
        };

        let result = validate_non_compete_reasonableness(&clause, "Engineer");
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_activities_error() {
        let clause = NonCompeteClause {
            duration_months: 12,
            geographic_scope: "東京都内".to_string(),
            prohibited_activities: vec![],
            consideration_provided: true,
            compensation_amount_jpy: Some(1_200_000),
        };

        let result = validate_non_compete_reasonableness(&clause, "Engineer");
        assert!(result.is_err());
    }

    #[test]
    fn test_markdown_report_generation() {
        let clause = NonCompeteClause {
            duration_months: 6,
            geographic_scope: "東京都内".to_string(),
            prohibited_activities: vec!["同業種での就業".to_string()],
            consideration_provided: true,
            compensation_amount_jpy: Some(600_000),
        };

        let report = validate_non_compete_reasonableness(&clause, "Engineer").unwrap();
        let markdown = report.to_markdown();

        assert!(markdown.contains("合理性判定レポート"));
        assert!(markdown.contains("総合判定"));
        assert!(markdown.contains("リスクレベル"));
    }
}
