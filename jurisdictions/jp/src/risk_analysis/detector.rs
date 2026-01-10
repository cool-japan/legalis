//! Risk Detector (リスク検出器)
//!
//! Coordinates risk detection across contract documents.

use super::error::{Result, RiskAnalysisError};
use super::rules::RuleEngine;
use super::types::*;

/// Contract document for analysis (分析対象契約文書)
#[derive(Debug, Clone)]
pub struct ContractDocument {
    /// Document title
    pub title: String,
    /// Contract type
    pub contract_type: ContractType,
    /// Contract clauses
    pub clauses: Vec<ContractClause>,
}

/// Individual contract clause (契約条項)
#[derive(Debug, Clone)]
pub struct ContractClause {
    /// Clause identifier (e.g., "Article 5", "Section 3.2")
    pub id: String,
    /// Clause text
    pub text: String,
}

impl ContractDocument {
    /// Creates a new contract document
    pub fn new(title: impl Into<String>, contract_type: ContractType) -> Self {
        Self {
            title: title.into(),
            contract_type,
            clauses: Vec::new(),
        }
    }

    /// Adds a clause
    pub fn add_clause(&mut self, id: impl Into<String>, text: impl Into<String>) {
        self.clauses.push(ContractClause {
            id: id.into(),
            text: text.into(),
        });
    }

    /// Returns the total number of clauses
    pub fn clause_count(&self) -> usize {
        self.clauses.len()
    }
}

/// Risk detector that analyzes contract documents
pub struct RiskDetector {
    rule_engine: RuleEngine,
}

impl RiskDetector {
    /// Creates a new risk detector with default rules
    pub fn new() -> Self {
        Self {
            rule_engine: RuleEngine::new(),
        }
    }

    /// Creates a risk detector with a custom rule engine
    pub fn with_rule_engine(rule_engine: RuleEngine) -> Self {
        Self { rule_engine }
    }

    /// Analyzes a contract document and generates a risk report
    pub fn analyze(&self, document: &ContractDocument) -> Result<RiskAnalysisReport> {
        if document.clauses.is_empty() {
            return Err(RiskAnalysisError::MissingRequiredData {
                field_name: "contract clauses".to_string(),
            });
        }

        let mut report = RiskAnalysisReport::new(&document.title, document.contract_type);

        // Analyze each clause
        for clause in &document.clauses {
            let findings =
                self.rule_engine
                    .analyze_text(&clause.text, &clause.id, document.contract_type);

            for finding in findings {
                report.add_finding(finding);
            }
        }

        // Calculate overall risk score
        report.calculate_risk_score();

        // Sort findings by severity
        report.sort_findings();

        // Generate summary
        report.generate_summary();

        // Generate recommendations
        self.generate_recommendations(&mut report);

        Ok(report)
    }

    /// Analyzes a single text snippet
    pub fn analyze_text(
        &self,
        text: &str,
        contract_type: ContractType,
    ) -> Result<Vec<RiskFinding>> {
        Ok(self
            .rule_engine
            .analyze_text(text, "Unknown Location", contract_type))
    }

    /// Generates high-level recommendations based on findings
    fn generate_recommendations(&self, report: &mut RiskAnalysisReport) {
        if report.findings.is_empty() {
            report.recommendations.push(
                "契約書は主要なリスクがない状態です。定期的な見直しを推奨します。\n\
                 The contract appears to have no major risks. Regular review is recommended."
                    .to_string(),
            );
            return;
        }

        if report.critical_count() > 0 {
            report.recommendations.push(
                format!(
                    "【緊急】{}件の重大リスクが検出されました。速やかに専門家に相談し、該当条項を修正してください。\n\
                     [URGENT] {} critical risk(s) detected. Consult with legal experts immediately.",
                    report.critical_count(),
                    report.critical_count()
                )
            );
        }

        if report.high_count() > 0 {
            report.recommendations.push(format!(
                "{}件の高リスク項目があります。契約締結前に必ず修正してください。\n\
                     {} high risk item(s) found. Must be corrected before signing.",
                report.high_count(),
                report.high_count()
            ));
        }

        if report.medium_count() > 0 {
            report.recommendations.push(format!(
                "{}件の中リスク項目があります。可能な限り改善を検討してください。\n\
                     {} medium risk item(s) found. Consider improvements where possible.",
                report.medium_count(),
                report.medium_count()
            ));
        }

        // Specific category recommendations
        let mut categories = std::collections::HashMap::new();
        for finding in &report.findings {
            *categories.entry(finding.category).or_insert(0) += 1;
        }

        for (category, count) in categories {
            if count >= 2 {
                let category_rec = match category {
                    RiskCategory::ConsumerProtection => {
                        "消費者保護法関連の問題が複数検出されました。消費者契約法の要件を満たすよう全体的な見直しを推奨します。"
                    }
                    RiskCategory::LaborLaw => {
                        "労働法関連の問題が複数検出されました。労働基準法・労働契約法の専門家による確認を推奨します。"
                    }
                    RiskCategory::LiabilityExemption => {
                        "免責条項に複数の問題があります。適切な免責範囲への修正が必要です。"
                    }
                    _ => continue,
                };

                report.recommendations.push(category_rec.to_string());
            }
        }

        // Final recommendation
        report.recommendations.push(
            "詳細な個別の推奨事項については、各リスク検出項目をご確認ください。\n\
             For detailed recommendations, please review each risk finding."
                .to_string(),
        );
    }
}

impl Default for RiskDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Quick analysis function for simple use cases
pub fn quick_analyze(
    title: &str,
    contract_type: ContractType,
    text: &str,
) -> Result<RiskAnalysisReport> {
    let mut document = ContractDocument::new(title, contract_type);
    document.add_clause("Full Contract", text);

    let detector = RiskDetector::new();
    detector.analyze(&document)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contract_document_creation() {
        let mut doc = ContractDocument::new("Test Contract", ContractType::Employment);
        doc.add_clause("Article 1", "Clause text 1");
        doc.add_clause("Article 2", "Clause text 2");

        assert_eq!(doc.clause_count(), 2);
        assert_eq!(doc.contract_type, ContractType::Employment);
    }

    #[test]
    fn test_risk_detector_analysis() {
        let mut doc = ContractDocument::new("Consumer Contract", ContractType::Consumer);
        doc.add_clause("Article 5", "当社は一切責任を負いません。");

        let detector = RiskDetector::new();
        let report = detector.analyze(&doc).unwrap();

        assert!(!report.findings.is_empty());
        assert!(report.critical_count() > 0);
        assert!(report.overall_risk_score > 0);
    }

    #[test]
    fn test_empty_document() {
        let doc = ContractDocument::new("Empty Contract", ContractType::General);

        let detector = RiskDetector::new();
        let result = detector.analyze(&doc);

        assert!(result.is_err());
    }

    #[test]
    fn test_quick_analyze() {
        let result = quick_analyze(
            "Test Contract",
            ContractType::Employment,
            "退職時には違約金を定める。",
        );

        assert!(result.is_ok());
        let report = result.unwrap();
        assert!(!report.findings.is_empty());
    }

    #[test]
    fn test_analyze_text() {
        let detector = RiskDetector::new();
        let findings = detector
            .analyze_text("当社は一切責任を負いません。", ContractType::Consumer)
            .unwrap();

        assert!(!findings.is_empty());
        assert_eq!(findings[0].severity, RiskSeverity::Critical);
    }

    #[test]
    fn test_recommendation_generation() {
        let mut doc = ContractDocument::new("Test Contract", ContractType::Consumer);
        doc.add_clause("Article 1", "当社は一切責任を負いません。");
        doc.add_clause("Article 2", "違約金は契約金額の100%とする。");

        let detector = RiskDetector::new();
        let report = detector.analyze(&doc).unwrap();

        assert!(!report.recommendations.is_empty());
        assert!(report.summary.contains("検出されました") || report.summary.contains("detected"));
    }

    #[test]
    fn test_clean_contract() {
        let mut doc = ContractDocument::new("Clean Contract", ContractType::General);
        doc.add_clause("Article 1", "双方は誠実に契約を履行する。");

        let detector = RiskDetector::new();
        let report = detector.analyze(&doc).unwrap();

        assert_eq!(report.overall_risk_score, 0);
        assert_eq!(report.findings.len(), 0);
    }
}
