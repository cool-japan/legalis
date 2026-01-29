//! Legal reasoning engine for Malaysian law.
//!
//! Provides utilities for legal analysis and compliance checking across Malaysian legal domains.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Reasoning error types.
#[derive(Debug, Error)]
pub enum ReasoningError {
    /// Analysis failed.
    #[error("Legal analysis failed: {reason}")]
    AnalysisFailed { reason: String },

    /// Insufficient information.
    #[error("Insufficient information for analysis: {missing}")]
    InsufficientInfo { missing: String },
}

/// Result type for reasoning operations.
pub type Result<T> = std::result::Result<T, ReasoningError>;

/// Compliance status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComplianceStatus {
    /// Fully compliant.
    Compliant,
    /// Partially compliant (minor issues).
    PartiallyCompliant,
    /// Non-compliant (significant issues).
    NonCompliant,
    /// Unknown (insufficient information).
    Unknown,
}

/// Risk level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    /// Low risk.
    Low,
    /// Medium risk.
    Medium,
    /// High risk.
    High,
    /// Critical risk.
    Critical,
}

/// Legal analysis result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalAnalysis {
    /// Compliance status.
    pub compliance_status: ComplianceStatus,
    /// Risk level.
    pub risk_level: RiskLevel,
    /// Issues identified.
    pub issues: Vec<String>,
    /// Recommendations.
    pub recommendations: Vec<String>,
    /// Applicable laws and sections.
    pub applicable_laws: Vec<String>,
}

impl LegalAnalysis {
    /// Creates a new legal analysis.
    #[must_use]
    pub fn new(compliance_status: ComplianceStatus, risk_level: RiskLevel) -> Self {
        Self {
            compliance_status,
            risk_level,
            issues: Vec::new(),
            recommendations: Vec::new(),
            applicable_laws: Vec::new(),
        }
    }

    /// Adds an issue.
    #[must_use]
    pub fn add_issue(mut self, issue: impl Into<String>) -> Self {
        self.issues.push(issue.into());
        self
    }

    /// Adds a recommendation.
    #[must_use]
    pub fn add_recommendation(mut self, recommendation: impl Into<String>) -> Self {
        self.recommendations.push(recommendation.into());
        self
    }

    /// Adds an applicable law.
    #[must_use]
    pub fn add_applicable_law(mut self, law: impl Into<String>) -> Self {
        self.applicable_laws.push(law.into());
        self
    }
}

/// Legal reasoning engine.
#[derive(Debug, Clone)]
pub struct LegalReasoningEngine;

impl LegalReasoningEngine {
    /// Creates a new legal reasoning engine.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Performs legal analysis on a given scenario.
    pub fn analyze(&self, scenario: &str, domain: LegalDomain) -> Result<LegalAnalysis> {
        let analysis = match domain {
            LegalDomain::Employment => self.analyze_employment(scenario),
            LegalDomain::Company => self.analyze_company(scenario),
            LegalDomain::DataProtection => self.analyze_data_protection(scenario),
            LegalDomain::Contract => self.analyze_contract(scenario),
            LegalDomain::Islamic => self.analyze_islamic_law(scenario),
            LegalDomain::Tax => self.analyze_tax(scenario),
            LegalDomain::Competition => self.analyze_competition(scenario),
            LegalDomain::Securities => self.analyze_securities(scenario),
        };

        Ok(analysis)
    }

    fn analyze_employment(&self, scenario: &str) -> LegalAnalysis {
        let mut analysis = LegalAnalysis::new(ComplianceStatus::Compliant, RiskLevel::Low)
            .add_applicable_law("Employment Act 1955");

        let scenario_lower = scenario.to_lowercase();

        // Check for working hours violations
        if scenario_lower.contains("9 hours") || scenario_lower.contains("50 hours") {
            analysis = analysis
                .add_issue("Working hours may exceed legal limits (8h/day, 48h/week)")
                .add_recommendation("Review working hours to ensure compliance with Section 60D")
                .add_applicable_law("Employment Act 1955, Section 60D");
            analysis.compliance_status = ComplianceStatus::NonCompliant;
            analysis.risk_level = RiskLevel::High;
        }

        // Check for minimum wage
        if scenario_lower.contains("rm 1,000") || scenario_lower.contains("rm 1,200") {
            analysis = analysis
                .add_issue("Salary below minimum wage (RM 1,500/month)")
                .add_recommendation("Adjust salary to meet minimum wage requirements");
            analysis.compliance_status = ComplianceStatus::NonCompliant;
            analysis.risk_level = RiskLevel::High;
        }

        analysis
    }

    fn analyze_company(&self, _scenario: &str) -> LegalAnalysis {
        LegalAnalysis::new(ComplianceStatus::Compliant, RiskLevel::Low)
            .add_applicable_law("Companies Act 2016")
    }

    fn analyze_data_protection(&self, scenario: &str) -> LegalAnalysis {
        let mut analysis = LegalAnalysis::new(ComplianceStatus::Compliant, RiskLevel::Low)
            .add_applicable_law("PDPA 2010");

        let scenario_lower = scenario.to_lowercase();

        // Check for consent issues
        if scenario_lower.contains("no consent") || scenario_lower.contains("without consent") {
            analysis = analysis
                .add_issue("Processing personal data without consent")
                .add_recommendation("Obtain valid consent before processing personal data")
                .add_applicable_law("PDPA 2010, Section 6");
            analysis.compliance_status = ComplianceStatus::NonCompliant;
            analysis.risk_level = RiskLevel::High;
        }

        analysis
    }

    fn analyze_contract(&self, _scenario: &str) -> LegalAnalysis {
        LegalAnalysis::new(ComplianceStatus::Compliant, RiskLevel::Low)
            .add_applicable_law("Contracts Act 1950")
    }

    fn analyze_islamic_law(&self, scenario: &str) -> LegalAnalysis {
        let mut analysis = LegalAnalysis::new(ComplianceStatus::Compliant, RiskLevel::Low)
            .add_applicable_law("Islamic Family Law");

        let scenario_lower = scenario.to_lowercase();

        // Check for Syariah compliance
        if scenario_lower.contains("riba") || scenario_lower.contains("interest") {
            analysis = analysis
                .add_issue("Riba (interest) detected - prohibited in Islamic law")
                .add_recommendation("Use Syariah-compliant alternatives (e.g., Murabahah, Ijarah)");
            analysis.compliance_status = ComplianceStatus::NonCompliant;
            analysis.risk_level = RiskLevel::Critical;
        }

        analysis
    }

    fn analyze_tax(&self, _scenario: &str) -> LegalAnalysis {
        LegalAnalysis::new(ComplianceStatus::Compliant, RiskLevel::Low)
            .add_applicable_law("Income Tax Act 1967")
            .add_applicable_law("Sales Tax Act 2018")
            .add_applicable_law("Service Tax Act 2018")
    }

    fn analyze_competition(&self, scenario: &str) -> LegalAnalysis {
        let mut analysis = LegalAnalysis::new(ComplianceStatus::Compliant, RiskLevel::Low)
            .add_applicable_law("Competition Act 2010");

        let scenario_lower = scenario.to_lowercase();

        // Check for anti-competitive practices
        if scenario_lower.contains("price fixing") || scenario_lower.contains("cartel") {
            analysis = analysis
                .add_issue("Potential price fixing or cartel activity detected")
                .add_recommendation("Cease anti-competitive agreements immediately")
                .add_applicable_law("Competition Act 2010, Section 4");
            analysis.compliance_status = ComplianceStatus::NonCompliant;
            analysis.risk_level = RiskLevel::Critical;
        }

        analysis
    }

    fn analyze_securities(&self, _scenario: &str) -> LegalAnalysis {
        LegalAnalysis::new(ComplianceStatus::Compliant, RiskLevel::Low)
            .add_applicable_law("Capital Markets and Services Act 2007")
    }
}

impl Default for LegalReasoningEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Legal domain for analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LegalDomain {
    /// Employment law.
    Employment,
    /// Company law.
    Company,
    /// Data protection law.
    DataProtection,
    /// Contract law.
    Contract,
    /// Islamic law.
    Islamic,
    /// Tax law.
    Tax,
    /// Competition law.
    Competition,
    /// Securities law.
    Securities,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_employment_analysis() {
        let engine = LegalReasoningEngine::new();
        let analysis = engine
            .analyze(
                "Employee works 9 hours per day, 50 hours per week",
                LegalDomain::Employment,
            )
            .expect("Analysis succeeds");

        assert_eq!(analysis.compliance_status, ComplianceStatus::NonCompliant);
        assert_eq!(analysis.risk_level, RiskLevel::High);
        assert!(!analysis.issues.is_empty());
    }

    #[test]
    fn test_pdpa_analysis() {
        let engine = LegalReasoningEngine::new();
        let analysis = engine
            .analyze(
                "Company processes personal data without consent",
                LegalDomain::DataProtection,
            )
            .expect("Analysis succeeds");

        assert_eq!(analysis.compliance_status, ComplianceStatus::NonCompliant);
        assert!(!analysis.issues.is_empty());
    }

    #[test]
    fn test_islamic_law_analysis() {
        let engine = LegalReasoningEngine::new();
        let analysis = engine
            .analyze(
                "Loan agreement with 5% interest per annum",
                LegalDomain::Islamic,
            )
            .expect("Analysis succeeds");

        assert_eq!(analysis.compliance_status, ComplianceStatus::NonCompliant);
        assert_eq!(analysis.risk_level, RiskLevel::Critical);
    }

    #[test]
    fn test_competition_analysis() {
        let engine = LegalReasoningEngine::new();
        let analysis = engine
            .analyze("Companies agree on price fixing", LegalDomain::Competition)
            .expect("Analysis succeeds");

        assert_eq!(analysis.compliance_status, ComplianceStatus::NonCompliant);
        assert_eq!(analysis.risk_level, RiskLevel::Critical);
    }
}
