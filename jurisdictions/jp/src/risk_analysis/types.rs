//! Risk Analysis Types (リスク分析型定義)
//!
//! This module defines types for contract and legal document risk analysis.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Risk severity level (リスク深刻度)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum RiskSeverity {
    /// Low risk - Minor issues, recommendations only
    /// (低リスク - Tei risuku)
    Low = 1,
    /// Medium risk - Notable issues requiring attention
    /// (中リスク - Chū risuku)
    Medium = 2,
    /// High risk - Serious issues requiring immediate action
    /// (高リスク - Kō risuku)
    High = 3,
    /// Critical risk - Severe legal violations or major problems
    /// (重大リスク - Jūdai risuku)
    Critical = 4,
}

impl RiskSeverity {
    /// Returns the Japanese name
    pub fn japanese_name(&self) -> &'static str {
        match self {
            RiskSeverity::Low => "低リスク",
            RiskSeverity::Medium => "中リスク",
            RiskSeverity::High => "高リスク",
            RiskSeverity::Critical => "重大リスク",
        }
    }

    /// Returns the English name
    pub fn english_name(&self) -> &'static str {
        match self {
            RiskSeverity::Low => "Low Risk",
            RiskSeverity::Medium => "Medium Risk",
            RiskSeverity::High => "High Risk",
            RiskSeverity::Critical => "Critical Risk",
        }
    }

    /// Returns a numeric score (1-4)
    pub fn score(&self) -> u8 {
        *self as u8
    }

    /// Returns an emoji representation
    pub fn emoji(&self) -> &'static str {
        match self {
            RiskSeverity::Low => "⚡",
            RiskSeverity::Medium => "⚠️",
            RiskSeverity::High => "🚨",
            RiskSeverity::Critical => "❌",
        }
    }
}

/// Risk category (リスク分類)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RiskCategory {
    /// Consumer protection violation (消費者保護法違反)
    ConsumerProtection,
    /// Labor law violation (労働法違反)
    LaborLaw,
    /// Unfair contract term (不当契約条項)
    UnfairTerm,
    /// Liability exemption issue (免責条項問題)
    LiabilityExemption,
    /// Excessive penalty (過大な違約金)
    ExcessivePenalty,
    /// Ambiguous clause (曖昧な条項)
    AmbiguousClause,
    /// Intellectual property issue (知的財産権問題)
    IntellectualProperty,
    /// Data protection issue (個人情報保護問題)
    DataProtection,
    /// Illegal non-compete (不当な競業避止義務)
    IllegalNonCompete,
    /// Unreasonable jurisdiction (不当な管轄合意)
    UnreasonableJurisdiction,
    /// Public policy violation (公序良俗違反)
    PublicPolicyViolation,
    /// Missing required clause (必須条項欠如)
    MissingRequiredClause,
}

impl RiskCategory {
    /// Returns the Japanese name
    pub fn japanese_name(&self) -> &'static str {
        match self {
            RiskCategory::ConsumerProtection => "消費者保護法違反",
            RiskCategory::LaborLaw => "労働法違反",
            RiskCategory::UnfairTerm => "不当契約条項",
            RiskCategory::LiabilityExemption => "免責条項問題",
            RiskCategory::ExcessivePenalty => "過大な違約金",
            RiskCategory::AmbiguousClause => "曖昧な条項",
            RiskCategory::IntellectualProperty => "知的財産権問題",
            RiskCategory::DataProtection => "個人情報保護問題",
            RiskCategory::IllegalNonCompete => "不当な競業避止義務",
            RiskCategory::UnreasonableJurisdiction => "不当な管轄合意",
            RiskCategory::PublicPolicyViolation => "公序良俗違反",
            RiskCategory::MissingRequiredClause => "必須条項欠如",
        }
    }
}

/// Contract type for risk analysis (契約種別)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContractType {
    /// Employment contract (雇用契約)
    Employment,
    /// Consumer contract (消費者契約)
    Consumer,
    /// Sales contract (売買契約)
    Sales,
    /// Service agreement (業務委託契約)
    Service,
    /// NDA (秘密保持契約)
    NDA,
    /// Lease agreement (賃貸借契約)
    Lease,
    /// Partnership agreement (パートナーシップ契約)
    Partnership,
    /// License agreement (ライセンス契約)
    License,
    /// General contract (一般契約)
    General,
}

impl ContractType {
    /// Returns the Japanese name
    pub fn japanese_name(&self) -> &'static str {
        match self {
            ContractType::Employment => "雇用契約",
            ContractType::Consumer => "消費者契約",
            ContractType::Sales => "売買契約",
            ContractType::Service => "業務委託契約",
            ContractType::NDA => "秘密保持契約",
            ContractType::Lease => "賃貸借契約",
            ContractType::Partnership => "パートナーシップ契約",
            ContractType::License => "ライセンス契約",
            ContractType::General => "一般契約",
        }
    }
}

/// Individual risk finding (リスク検出結果)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFinding {
    /// Unique identifier
    pub id: String,
    /// Risk severity
    pub severity: RiskSeverity,
    /// Risk category
    pub category: RiskCategory,
    /// Location in document (e.g., "Article 5, Paragraph 2")
    pub location: String,
    /// The problematic text/clause
    pub problematic_text: String,
    /// Description of the issue
    pub issue_description: String,
    /// Legal reference (e.g., "Labor Standards Act Article 16")
    pub legal_reference: Option<String>,
    /// Recommended action
    pub recommendation: String,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
}

impl RiskFinding {
    /// Creates a new risk finding
    pub fn new(
        id: impl Into<String>,
        severity: RiskSeverity,
        category: RiskCategory,
        location: impl Into<String>,
        problematic_text: impl Into<String>,
        issue_description: impl Into<String>,
        recommendation: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            severity,
            category,
            location: location.into(),
            problematic_text: problematic_text.into(),
            issue_description: issue_description.into(),
            legal_reference: None,
            recommendation: recommendation.into(),
            confidence: 1.0,
        }
    }

    /// Sets the legal reference
    pub fn with_legal_reference(mut self, reference: impl Into<String>) -> Self {
        self.legal_reference = Some(reference.into());
        self
    }

    /// Sets the confidence score
    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }
}

/// Risk analysis report (リスク分析レポート)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAnalysisReport {
    /// Document/contract being analyzed
    pub document_title: String,
    /// Contract type
    pub contract_type: ContractType,
    /// Analysis timestamp
    pub analysis_date: chrono::DateTime<chrono::Utc>,
    /// List of risk findings
    pub findings: Vec<RiskFinding>,
    /// Overall risk score (0-100)
    pub overall_risk_score: u8,
    /// Severity breakdown
    pub severity_counts: HashMap<RiskSeverity, usize>,
    /// Summary text
    pub summary: String,
    /// Recommendations
    pub recommendations: Vec<String>,
}

impl RiskAnalysisReport {
    /// Creates a new risk analysis report
    pub fn new(document_title: impl Into<String>, contract_type: ContractType) -> Self {
        Self {
            document_title: document_title.into(),
            contract_type,
            analysis_date: chrono::Utc::now(),
            findings: Vec::new(),
            overall_risk_score: 0,
            severity_counts: HashMap::new(),
            summary: String::new(),
            recommendations: Vec::new(),
        }
    }

    /// Adds a risk finding
    pub fn add_finding(&mut self, finding: RiskFinding) {
        *self.severity_counts.entry(finding.severity).or_insert(0) += 1;
        self.findings.push(finding);
    }

    /// Calculates the overall risk score
    pub fn calculate_risk_score(&mut self) {
        if self.findings.is_empty() {
            self.overall_risk_score = 0;
            return;
        }

        let mut total_score = 0u32;
        let mut total_weight = 0u32;

        for finding in &self.findings {
            let severity_weight = finding.severity.score() as u32;
            let confidence_weight = (finding.confidence * 100.0) as u32;

            total_score += severity_weight * confidence_weight;
            total_weight += confidence_weight;
        }

        if let Some(weighted_score) = (total_score * 25).checked_div(total_weight) {
            self.overall_risk_score = weighted_score.min(100) as u8;
        }
    }

    /// Returns the number of critical findings
    pub fn critical_count(&self) -> usize {
        *self
            .severity_counts
            .get(&RiskSeverity::Critical)
            .unwrap_or(&0)
    }

    /// Returns the number of high findings
    pub fn high_count(&self) -> usize {
        *self.severity_counts.get(&RiskSeverity::High).unwrap_or(&0)
    }

    /// Returns the number of medium findings
    pub fn medium_count(&self) -> usize {
        *self
            .severity_counts
            .get(&RiskSeverity::Medium)
            .unwrap_or(&0)
    }

    /// Returns the number of low findings
    pub fn low_count(&self) -> usize {
        *self.severity_counts.get(&RiskSeverity::Low).unwrap_or(&0)
    }

    /// Checks if the document has any critical or high risks
    pub fn has_serious_risks(&self) -> bool {
        self.critical_count() > 0 || self.high_count() > 0
    }

    /// Sorts findings by severity (critical first)
    pub fn sort_findings(&mut self) {
        self.findings.sort_by(|a, b| {
            b.severity.cmp(&a.severity).then_with(|| {
                b.confidence
                    .partial_cmp(&a.confidence)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
        });
    }

    /// Generates a summary
    pub fn generate_summary(&mut self) {
        let total = self.findings.len();

        if total == 0 {
            self.summary = "分析の結果、重大なリスクは検出されませんでした。\n\
                          No significant risks were detected in the analysis."
                .to_string();
            return;
        }

        let critical = self.critical_count();
        let high = self.high_count();
        let medium = self.medium_count();
        let low = self.low_count();

        let mut parts = Vec::new();

        if critical > 0 {
            parts.push(format!("重大リスク: {}件", critical));
        }
        if high > 0 {
            parts.push(format!("高リスク: {}件", high));
        }
        if medium > 0 {
            parts.push(format!("中リスク: {}件", medium));
        }
        if low > 0 {
            parts.push(format!("低リスク: {}件", low));
        }

        self.summary = format!(
            "合計{}件のリスクが検出されました。({})\n\
             Total {} risk(s) detected.",
            total,
            parts.join(", "),
            total
        );
    }
}

/// Compliance check result (コンプライアンスチェック結果)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceCheckResult {
    /// Legal area being checked
    pub legal_area: String,
    /// Whether compliant
    pub is_compliant: bool,
    /// Violations found
    pub violations: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
}

impl ComplianceCheckResult {
    /// Creates a compliant result
    pub fn compliant(legal_area: impl Into<String>) -> Self {
        Self {
            legal_area: legal_area.into(),
            is_compliant: true,
            violations: Vec::new(),
            recommendations: Vec::new(),
        }
    }

    /// Creates a non-compliant result
    pub fn non_compliant(
        legal_area: impl Into<String>,
        violations: Vec<String>,
        recommendations: Vec<String>,
    ) -> Self {
        Self {
            legal_area: legal_area.into(),
            is_compliant: false,
            violations,
            recommendations,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_risk_severity_ordering() {
        assert!(RiskSeverity::Critical > RiskSeverity::High);
        assert!(RiskSeverity::High > RiskSeverity::Medium);
        assert!(RiskSeverity::Medium > RiskSeverity::Low);
    }

    #[test]
    fn test_risk_severity_score() {
        assert_eq!(RiskSeverity::Low.score(), 1);
        assert_eq!(RiskSeverity::Medium.score(), 2);
        assert_eq!(RiskSeverity::High.score(), 3);
        assert_eq!(RiskSeverity::Critical.score(), 4);
    }

    #[test]
    fn test_risk_finding_creation() {
        let finding = RiskFinding::new(
            "risk-001",
            RiskSeverity::High,
            RiskCategory::UnfairTerm,
            "Article 5",
            "Problematic clause text",
            "This is unfair",
            "Revise the clause",
        )
        .with_legal_reference("Consumer Contract Act Article 10")
        .with_confidence(0.85);

        assert_eq!(finding.severity, RiskSeverity::High);
        assert_eq!(finding.confidence, 0.85);
        assert_eq!(
            finding.legal_reference.unwrap(),
            "Consumer Contract Act Article 10"
        );
    }

    #[test]
    fn test_risk_report_calculation() {
        let mut report = RiskAnalysisReport::new("Test Contract", ContractType::Employment);

        report.add_finding(RiskFinding::new(
            "r1",
            RiskSeverity::Critical,
            RiskCategory::LaborLaw,
            "Article 1",
            "Bad clause",
            "Issue",
            "Fix it",
        ));

        report.add_finding(RiskFinding::new(
            "r2",
            RiskSeverity::Low,
            RiskCategory::AmbiguousClause,
            "Article 2",
            "Unclear clause",
            "Minor issue",
            "Clarify",
        ));

        report.calculate_risk_score();

        assert_eq!(report.findings.len(), 2);
        assert_eq!(report.critical_count(), 1);
        assert_eq!(report.low_count(), 1);
        assert!(report.has_serious_risks());
        assert!(report.overall_risk_score > 0);
    }

    #[test]
    fn test_report_sorting() {
        let mut report = RiskAnalysisReport::new("Test", ContractType::General);

        report.add_finding(RiskFinding::new(
            "r1",
            RiskSeverity::Low,
            RiskCategory::AmbiguousClause,
            "A1",
            "text1",
            "issue1",
            "rec1",
        ));

        report.add_finding(RiskFinding::new(
            "r2",
            RiskSeverity::Critical,
            RiskCategory::LaborLaw,
            "A2",
            "text2",
            "issue2",
            "rec2",
        ));

        report.sort_findings();

        assert_eq!(report.findings[0].severity, RiskSeverity::Critical);
        assert_eq!(report.findings[1].severity, RiskSeverity::Low);
    }

    #[test]
    fn test_compliance_check_result() {
        let compliant = ComplianceCheckResult::compliant("Labor Law");
        assert!(compliant.is_compliant);
        assert!(compliant.violations.is_empty());

        let non_compliant = ComplianceCheckResult::non_compliant(
            "Consumer Protection",
            vec!["Violation 1".to_string()],
            vec!["Fix it".to_string()],
        );
        assert!(!non_compliant.is_compliant);
        assert_eq!(non_compliant.violations.len(), 1);
    }

    #[test]
    fn test_category_names() {
        assert_eq!(RiskCategory::LaborLaw.japanese_name(), "労働法違反");
        assert_eq!(RiskCategory::UnfairTerm.japanese_name(), "不当契約条項");
    }
}
