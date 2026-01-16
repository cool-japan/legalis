//! Common types for legal reasoning (UK Employment Law).

use serde::{Deserialize, Serialize};

/// Compliance status for UK employment law
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComplianceStatus {
    /// Fully compliant with applicable law
    Compliant,
    /// Non-compliant - requires immediate action
    NonCompliant,
    /// Partially compliant - some issues identified
    PartiallyCompliant,
    /// Unable to determine due to missing information
    Indeterminate,
}

impl ComplianceStatus {
    /// Returns true if the status is fully compliant
    #[must_use]
    pub const fn is_compliant(self) -> bool {
        matches!(self, Self::Compliant)
    }

    /// Returns true if the status requires action
    #[must_use]
    pub const fn requires_action(self) -> bool {
        matches!(self, Self::NonCompliant | Self::PartiallyCompliant)
    }
}

/// Risk level for legal analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RiskLevel {
    /// No identified risk
    None,
    /// Minor risk - advisory
    Low,
    /// Moderate risk - attention recommended
    Medium,
    /// Significant risk - action required
    High,
    /// Critical risk - immediate action required
    Critical,
}

impl RiskLevel {
    /// Returns true if this risk level requires immediate action
    #[must_use]
    pub const fn requires_immediate_action(self) -> bool {
        matches!(self, Self::Critical)
    }

    /// Returns true if this risk level requires attention
    #[must_use]
    pub const fn requires_attention(self) -> bool {
        matches!(self, Self::Medium | Self::High | Self::Critical)
    }
}

/// Severity of a legal violation
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ViolationSeverity {
    /// Advisory only - not a true violation
    Advisory,
    /// Minor violation - low impact
    Minor,
    /// Moderate violation - significant impact
    Moderate,
    /// Major violation - serious consequences
    Major,
    /// Critical violation - immediate legal exposure
    Critical,
}

/// A legal violation identified during analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Violation {
    /// Unique identifier for the violated statute/regulation
    pub statute_id: String,
    /// Human-readable name of the statute
    pub statute_name: String,
    /// Description of the violation
    pub description: String,
    /// Severity level
    pub severity: ViolationSeverity,
    /// Recommended remediation
    pub remediation: Option<String>,
    /// Legal reference (e.g., "ERA 1996 s.86")
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

    /// Add remediation recommendation
    #[must_use]
    pub fn with_remediation(mut self, remediation: impl Into<String>) -> Self {
        self.remediation = Some(remediation.into());
        self
    }

    /// Add legal reference
    #[must_use]
    pub fn with_legal_reference(mut self, reference: impl Into<String>) -> Self {
        self.legal_reference = Some(reference.into());
        self
    }
}

/// Result of a legal analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LegalAnalysis {
    /// Overall compliance status
    pub status: ComplianceStatus,
    /// Overall risk level
    pub risk_level: RiskLevel,
    /// Identified violations
    pub violations: Vec<Violation>,
    /// Confidence in the analysis (0.0 - 1.0)
    pub confidence: f64,
    /// Summary of the analysis
    pub summary: String,
}

impl LegalAnalysis {
    /// Create a new legal analysis
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

    /// Add a violation
    pub fn add_violation(&mut self, violation: Violation) {
        // Update risk level based on violation severity
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

    /// Set confidence level
    #[must_use]
    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }

    /// Check if analysis found any violations
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
