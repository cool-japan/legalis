//! Common types for legal reasoning (法的推論共通型).
//!
//! Provides types for compliance status, risk levels, and violations
//! under Japanese labor law.

use serde::{Deserialize, Serialize};

/// Compliance status (コンプライアンス状態)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComplianceStatus {
    /// 法令遵守 / Fully compliant
    Compliant,
    /// 法令違反 / Non-compliant
    NonCompliant,
    /// 部分的遵守 / Partially compliant
    PartiallyCompliant,
    /// 判定不能 / Cannot determine
    Indeterminate,
}

impl ComplianceStatus {
    #[must_use]
    pub const fn is_compliant(self) -> bool {
        matches!(self, Self::Compliant)
    }

    #[must_use]
    pub const fn requires_action(self) -> bool {
        matches!(self, Self::NonCompliant | Self::PartiallyCompliant)
    }
}

/// Risk level (リスクレベル)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RiskLevel {
    /// リスクなし / No risk
    None,
    /// 低リスク / Low risk
    Low,
    /// 中リスク / Medium risk
    Medium,
    /// 高リスク / High risk
    High,
    /// 緊急対応要 / Critical - immediate action
    Critical,
}

impl RiskLevel {
    #[must_use]
    pub const fn requires_immediate_action(self) -> bool {
        matches!(self, Self::Critical)
    }

    #[must_use]
    pub const fn requires_attention(self) -> bool {
        matches!(self, Self::Medium | Self::High | Self::Critical)
    }
}

/// Severity of violation (違反の重大度)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ViolationSeverity {
    /// 助言のみ / Advisory only
    Advisory,
    /// 軽微な違反 / Minor violation
    Minor,
    /// 中程度の違反 / Moderate violation
    Moderate,
    /// 重大な違反 / Major violation
    Major,
    /// 緊急の違反 / Critical violation
    Critical,
}

/// A legal violation (法令違反)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Violation {
    /// Statute identifier (法令ID)
    pub statute_id: String,
    /// Statute name (法令名)
    pub statute_name: String,
    /// Description (違反内容)
    pub description: String,
    /// Severity (重大度)
    pub severity: ViolationSeverity,
    /// Remediation recommendation (是正措置)
    pub remediation: Option<String>,
    /// Legal reference (法令参照 - e.g., "労働基準法第32条")
    pub legal_reference: Option<String>,
}

impl Violation {
    /// Create a new violation
    #[must_use]
    pub fn new(
        statute_id: impl Into<String>,
        statute_name: impl Into<String>,
        description: impl Into<String>,
        severity: ViolationSeverity,
    ) -> Self {
        Self {
            statute_id: statute_id.into(),
            statute_name: statute_name.into(),
            description: description.into(),
            severity,
            remediation: None,
            legal_reference: None,
        }
    }

    /// Add remediation (是正措置を追加)
    #[must_use]
    pub fn with_remediation(mut self, remediation: impl Into<String>) -> Self {
        self.remediation = Some(remediation.into());
        self
    }

    /// Add legal reference (法令参照を追加)
    #[must_use]
    pub fn with_legal_reference(mut self, reference: impl Into<String>) -> Self {
        self.legal_reference = Some(reference.into());
        self
    }
}

/// Result of legal analysis (法的分析結果)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LegalAnalysis {
    /// Compliance status (コンプライアンス状態)
    pub status: ComplianceStatus,
    /// Risk level (リスクレベル)
    pub risk_level: RiskLevel,
    /// Violations (違反事項)
    pub violations: Vec<Violation>,
    /// Confidence (信頼度 - 0.0〜1.0)
    pub confidence: f64,
    /// Summary (分析概要)
    pub summary: String,
}

impl LegalAnalysis {
    /// Create new analysis
    #[must_use]
    pub fn new(
        status: ComplianceStatus,
        risk_level: RiskLevel,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            status,
            risk_level,
            violations: Vec::new(),
            confidence: 1.0,
            summary: summary.into(),
        }
    }

    /// Add violation
    pub fn add_violation(&mut self, violation: Violation) {
        let violation_risk = match violation.severity {
            ViolationSeverity::Advisory => RiskLevel::None,
            ViolationSeverity::Minor => RiskLevel::Low,
            ViolationSeverity::Moderate => RiskLevel::Medium,
            ViolationSeverity::Major => RiskLevel::High,
            ViolationSeverity::Critical => RiskLevel::Critical,
        };
        if violation_risk > self.risk_level {
            self.risk_level = violation_risk;
        }
        self.violations.push(violation);
    }

    /// Set confidence
    #[must_use]
    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }

    /// Check for violations
    #[must_use]
    pub fn has_violations(&self) -> bool {
        !self.violations.is_empty()
    }

    /// Get critical violations only
    #[must_use]
    pub fn critical_violations(&self) -> Vec<&Violation> {
        self.violations
            .iter()
            .filter(|v| v.severity == ViolationSeverity::Critical)
            .collect()
    }
}
