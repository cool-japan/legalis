//! Compliance Report System (コンプライアンスレポートシステム)
//!
//! Structured compliance validation results for contract generation.
//!
//! # Example
//!
//! ```
//! use legalis_jp::contract_templates::compliance::{ComplianceReport, ComplianceCheck, CheckStatus};
//! use chrono::Utc;
//!
//! let mut report = ComplianceReport::new("Employment Contract");
//!
//! report.add_check(ComplianceCheck {
//!     check_name: "Minimum Wage Check".to_string(),
//!     legal_reference: "Minimum Wage Act".to_string(),
//!     status: CheckStatus::Passed,
//!     details: "Hourly rate ¥2,307 meets Tokyo minimum wage ¥1,113".to_string(),
//! });
//!
//! assert!(report.is_compliant());
//! assert_eq!(report.score(), 100);
//! ```

use chrono::{DateTime, Utc};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Compliance report for labor law validation (労働法コンプライアンスレポート)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ComplianceReport {
    /// Contract type validated (検証対象契約種別)
    pub contract_type: String,

    /// Validation timestamp (検証日時)
    pub validation_date: DateTime<Utc>,

    /// Checks performed (実施済みチェック)
    pub checks_performed: Vec<ComplianceCheck>,

    /// Violations found (違反事項)
    pub violations: Vec<ComplianceViolation>,

    /// Warnings (警告)
    pub warnings: Vec<ComplianceWarning>,
}

impl ComplianceReport {
    /// Create new compliance report (新規レポート作成)
    pub fn new(contract_type: impl Into<String>) -> Self {
        Self {
            contract_type: contract_type.into(),
            validation_date: Utc::now(),
            checks_performed: Vec::new(),
            violations: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// Add a compliance check (チェック追加)
    pub fn add_check(&mut self, check: ComplianceCheck) -> &mut Self {
        // Add to violations or warnings based on status
        match check.status {
            CheckStatus::Failed => {
                self.violations.push(ComplianceViolation {
                    check_name: check.check_name.clone(),
                    legal_reference: check.legal_reference.clone(),
                    description: check.details.clone(),
                });
            }
            CheckStatus::Warning => {
                self.warnings.push(ComplianceWarning {
                    check_name: check.check_name.clone(),
                    description: check.details.clone(),
                });
            }
            CheckStatus::Passed => {
                // Just add to checks
            }
        }

        self.checks_performed.push(check);
        self
    }

    /// Add a violation (違反追加)
    pub fn add_violation(&mut self, violation: ComplianceViolation) -> &mut Self {
        self.violations.push(violation);
        self
    }

    /// Add a warning (警告追加)
    pub fn add_warning(&mut self, warning: ComplianceWarning) -> &mut Self {
        self.warnings.push(warning);
        self
    }

    /// Check if contract is compliant (準拠しているか)
    pub fn is_compliant(&self) -> bool {
        self.violations.is_empty()
    }

    /// Get compliance score (0-100) (コンプライアンススコア)
    pub fn score(&self) -> u32 {
        if self.violations.is_empty() {
            if self.warnings.is_empty() {
                100
            } else {
                // Deduct 5 points per warning, minimum 60
                100u32
                    .saturating_sub((self.warnings.len() as u32) * 5)
                    .max(60)
            }
        } else {
            // Deduct 20 points per violation, minimum 0
            100u32.saturating_sub((self.violations.len() as u32) * 20)
        }
    }

    /// Generate markdown report (Markdownレポート生成)
    pub fn to_markdown(&self) -> String {
        let mut report = String::new();

        report.push_str("# 労働法コンプライアンスレポート\n\n");
        report.push_str(&format!("**契約種別**: {}\n", self.contract_type));
        report.push_str(&format!(
            "**検証日時**: {}\n",
            self.validation_date.format("%Y年%m月%d日 %H:%M")
        ));
        report.push_str(&format!("**スコア**: {}/100\n\n", self.score()));

        // Violations
        if !self.violations.is_empty() {
            report.push_str("## ✗ 違反事項 (Violations)\n\n");
            for (i, violation) in self.violations.iter().enumerate() {
                report.push_str(&format!(
                    "{}. **{}** ({})\n   - {}\n\n",
                    i + 1,
                    violation.check_name,
                    violation.legal_reference,
                    violation.description
                ));
            }
        }

        // Warnings
        if !self.warnings.is_empty() {
            report.push_str("## ⚠ 警告 (Warnings)\n\n");
            for (i, warning) in self.warnings.iter().enumerate() {
                report.push_str(&format!(
                    "{}. **{}**\n   - {}\n\n",
                    i + 1,
                    warning.check_name,
                    warning.description
                ));
            }
        }

        // Passed checks
        let passed_checks: Vec<_> = self
            .checks_performed
            .iter()
            .filter(|c| c.status == CheckStatus::Passed)
            .collect();

        if !passed_checks.is_empty() {
            report.push_str("## ✓ 合格チェック (Passed Checks)\n\n");
            for check in passed_checks {
                report.push_str(&format!(
                    "- {} ({})\n",
                    check.check_name, check.legal_reference
                ));
            }
            report.push('\n');
        }

        report
    }
}

/// Compliance check result (コンプライアンスチェック結果)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ComplianceCheck {
    /// Check name (チェック名)
    pub check_name: String,

    /// Legal reference (法的根拠)
    pub legal_reference: String,

    /// Check status (ステータス)
    pub status: CheckStatus,

    /// Details (詳細)
    pub details: String,
}

/// Compliance check status (チェックステータス)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CheckStatus {
    /// Passed (合格)
    Passed,
    /// Failed (不合格)
    Failed,
    /// Warning (警告)
    Warning,
}

/// Compliance violation (コンプライアンス違反)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ComplianceViolation {
    /// Check name (チェック名)
    pub check_name: String,

    /// Legal reference (法的根拠)
    pub legal_reference: String,

    /// Description (説明)
    pub description: String,
}

/// Compliance warning (コンプライアンス警告)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ComplianceWarning {
    /// Check name (チェック名)
    pub check_name: String,

    /// Description (説明)
    pub description: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compliance_report_creation() {
        let report = ComplianceReport::new("Employment Contract");
        assert_eq!(report.contract_type, "Employment Contract");
        assert!(report.checks_performed.is_empty());
        assert!(report.violations.is_empty());
        assert!(report.warnings.is_empty());
    }

    #[test]
    fn test_perfect_score() {
        let report = ComplianceReport::new("Test");
        assert_eq!(report.score(), 100);
        assert!(report.is_compliant());
    }

    #[test]
    fn test_score_with_warnings() {
        let mut report = ComplianceReport::new("Test");
        report.add_warning(ComplianceWarning {
            check_name: "Test Warning".to_string(),
            description: "Test".to_string(),
        });

        assert_eq!(report.score(), 95); // 100 - 5
        assert!(report.is_compliant()); // Warnings don't break compliance
    }

    #[test]
    fn test_score_with_violations() {
        let mut report = ComplianceReport::new("Test");
        report.add_violation(ComplianceViolation {
            check_name: "Test Violation".to_string(),
            legal_reference: "Article X".to_string(),
            description: "Test violation".to_string(),
        });

        assert_eq!(report.score(), 80); // 100 - 20
        assert!(!report.is_compliant());
    }

    #[test]
    fn test_add_check_passed() {
        let mut report = ComplianceReport::new("Test");
        report.add_check(ComplianceCheck {
            check_name: "Minimum Wage".to_string(),
            legal_reference: "Minimum Wage Act".to_string(),
            status: CheckStatus::Passed,
            details: "Passed".to_string(),
        });

        assert_eq!(report.checks_performed.len(), 1);
        assert_eq!(report.violations.len(), 0);
        assert_eq!(report.warnings.len(), 0);
    }

    #[test]
    fn test_add_check_failed() {
        let mut report = ComplianceReport::new("Test");
        report.add_check(ComplianceCheck {
            check_name: "Minimum Wage".to_string(),
            legal_reference: "Minimum Wage Act".to_string(),
            status: CheckStatus::Failed,
            details: "Below minimum".to_string(),
        });

        assert_eq!(report.checks_performed.len(), 1);
        assert_eq!(report.violations.len(), 1);
        assert_eq!(report.warnings.len(), 0);
    }

    #[test]
    fn test_add_check_warning() {
        let mut report = ComplianceReport::new("Test");
        report.add_check(ComplianceCheck {
            check_name: "Working Hours".to_string(),
            legal_reference: "Article 32".to_string(),
            status: CheckStatus::Warning,
            details: "Exceeds 40 hours".to_string(),
        });

        assert_eq!(report.checks_performed.len(), 1);
        assert_eq!(report.violations.len(), 0);
        assert_eq!(report.warnings.len(), 1);
    }

    #[test]
    fn test_markdown_generation() {
        let mut report = ComplianceReport::new("Employment Contract");

        report.add_check(ComplianceCheck {
            check_name: "Minimum Wage".to_string(),
            legal_reference: "Minimum Wage Act".to_string(),
            status: CheckStatus::Passed,
            details: "¥2,307/時 >= ¥1,113/時".to_string(),
        });

        let markdown = report.to_markdown();
        assert!(markdown.contains("労働法コンプライアンスレポート"));
        assert!(markdown.contains("Employment Contract"));
        assert!(markdown.contains("100/100"));
    }
}
