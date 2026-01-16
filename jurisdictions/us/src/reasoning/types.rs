//! Common types for US legal reasoning.

use serde::{Deserialize, Serialize};

/// Compliance status for US law
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ComplianceStatus {
    /// Fully compliant with applicable law
    Compliant,
    /// Non-compliant - requires action
    NonCompliant {
        /// List of violations
        violations: Vec<Violation>,
    },
    /// Partially compliant - some issues identified
    PartiallyCompliant {
        /// List of violations
        violations: Vec<Violation>,
    },
    /// Unable to determine due to missing information
    Indeterminate,
}

impl ComplianceStatus {
    /// Returns true if the status is fully compliant
    #[must_use]
    pub const fn is_compliant(&self) -> bool {
        matches!(self, Self::Compliant)
    }

    /// Returns true if the status is non-compliant
    #[must_use]
    pub const fn is_non_compliant(&self) -> bool {
        matches!(self, Self::NonCompliant { .. })
    }

    /// Returns true if the status requires action
    #[must_use]
    pub const fn requires_action(&self) -> bool {
        matches!(
            self,
            Self::NonCompliant { .. } | Self::PartiallyCompliant { .. }
        )
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
    /// Legal reference (e.g., "29 U.S.C. § 206")
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

/// A step in the legal reasoning process
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReasoningStep {
    /// Step number in the analysis
    pub step: u32,
    /// Statute being evaluated
    pub statute_id: String,
    /// Description of the condition being checked
    pub condition_description: String,
    /// Result of the evaluation
    pub result: bool,
    /// Explanation of the result
    pub explanation: String,
}

/// Result of a legal analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LegalAnalysis {
    /// Entity type being analyzed
    pub entity_type: String,
    /// Overall compliance status
    pub compliance_status: ComplianceStatus,
    /// Overall risk level
    pub risk_level: RiskLevel,
    /// List of applicable statutes evaluated
    pub applicable_statutes: Vec<String>,
    /// Reasoning steps taken during analysis
    pub reasoning_steps: Vec<ReasoningStep>,
    /// Confidence in the analysis (0.0 - 1.0)
    pub confidence: f64,
}

impl LegalAnalysis {
    /// Create a new compliant analysis
    #[must_use]
    pub fn compliant(entity_type: impl Into<String>) -> Self {
        Self {
            entity_type: entity_type.into(),
            compliance_status: ComplianceStatus::Compliant,
            risk_level: RiskLevel::None,
            applicable_statutes: Vec::new(),
            reasoning_steps: Vec::new(),
            confidence: 1.0,
        }
    }

    /// Create a new non-compliant analysis with violations
    #[must_use]
    pub fn non_compliant(entity_type: impl Into<String>, violations: Vec<Violation>) -> Self {
        let risk_level = violations
            .iter()
            .map(|v| match v.severity {
                ViolationSeverity::Advisory => RiskLevel::Low,
                ViolationSeverity::Minor => RiskLevel::Medium,
                ViolationSeverity::Major => RiskLevel::High,
                ViolationSeverity::Critical => RiskLevel::Critical,
            })
            .max()
            .unwrap_or(RiskLevel::None);

        Self {
            entity_type: entity_type.into(),
            compliance_status: ComplianceStatus::NonCompliant { violations },
            risk_level,
            applicable_statutes: Vec::new(),
            reasoning_steps: Vec::new(),
            confidence: 1.0,
        }
    }

    /// Set confidence level
    #[must_use]
    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }
}
