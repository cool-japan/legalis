//! Types for legal reasoning analysis results.

use serde::{Deserialize, Serialize};

/// Compliance status of an entity against applicable statutes
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComplianceStatus {
    /// Fully compliant with all applicable statutes
    Compliant,
    /// Partially compliant with warnings
    PartiallyCompliant { warnings: Vec<String> },
    /// Non-compliant with violations
    NonCompliant { violations: Vec<Violation> },
    /// Unable to determine (missing information)
    Indeterminate { reason: String },
}

impl ComplianceStatus {
    /// Returns true if compliant
    #[must_use]
    pub const fn is_compliant(&self) -> bool {
        matches!(self, Self::Compliant)
    }

    /// Returns true if non-compliant
    #[must_use]
    pub const fn is_non_compliant(&self) -> bool {
        matches!(self, Self::NonCompliant { .. })
    }
}

/// A violation of a statute
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Violation {
    /// Statute ID that was violated
    pub statute_id: String,
    /// Description of the violation
    pub description: String,
    /// Severity of the violation
    pub severity: ViolationSeverity,
    /// Recommended remedies
    pub remedies: Vec<Remedy>,
    /// Legal reference (e.g., "EA s. 38(1)")
    pub legal_reference: String,
}

/// Severity level of a violation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ViolationSeverity {
    /// Critical - requires immediate action
    Critical,
    /// Major - significant compliance issue
    Major,
    /// Minor - should be addressed but not urgent
    Minor,
    /// Advisory - recommendation for best practice
    Advisory,
}

/// A remedy for a violation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Remedy {
    /// Type of remedy
    pub remedy_type: RemedyType,
    /// Description of the remedy
    pub description: String,
    /// Estimated effort to implement
    pub effort: Option<String>,
}

/// Type of remedy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RemedyType {
    /// Corrective action required
    Corrective,
    /// Preventive measure
    Preventive,
    /// Documentation update
    Documentation,
    /// Process change
    ProcessChange,
    /// Financial remedy (payment, compensation)
    Financial,
    /// Notification to authority (MOM, ACRA, PDPC)
    Notification,
}

/// Risk level assessment
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum RiskLevel {
    /// No risk identified
    None,
    /// Low risk
    Low,
    /// Medium risk
    Medium,
    /// High risk
    High,
    /// Critical risk
    Critical,
}

/// Complete legal analysis result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LegalAnalysis {
    /// Entity type analyzed
    pub entity_type: String,
    /// Overall compliance status
    pub compliance_status: ComplianceStatus,
    /// Risk level
    pub risk_level: RiskLevel,
    /// Reasoning steps taken
    pub reasoning_steps: Vec<ReasoningStep>,
    /// Legal opinions generated
    pub opinions: Vec<LegalOpinion>,
    /// Confidence score (0.0 - 1.0)
    pub confidence: f64,
    /// Applicable statute IDs
    pub applicable_statutes: Vec<String>,
    /// Timestamp of analysis
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl LegalAnalysis {
    /// Creates a new compliant analysis
    #[must_use]
    pub fn compliant(entity_type: impl Into<String>) -> Self {
        Self {
            entity_type: entity_type.into(),
            compliance_status: ComplianceStatus::Compliant,
            risk_level: RiskLevel::None,
            reasoning_steps: Vec::new(),
            opinions: Vec::new(),
            confidence: 1.0,
            applicable_statutes: Vec::new(),
            timestamp: chrono::Utc::now(),
        }
    }

    /// Creates a non-compliant analysis with violations
    #[must_use]
    pub fn non_compliant(entity_type: impl Into<String>, violations: Vec<Violation>) -> Self {
        let risk_level = violations
            .iter()
            .map(|v| match v.severity {
                ViolationSeverity::Critical => RiskLevel::Critical,
                ViolationSeverity::Major => RiskLevel::High,
                ViolationSeverity::Minor => RiskLevel::Medium,
                ViolationSeverity::Advisory => RiskLevel::Low,
            })
            .max()
            .unwrap_or(RiskLevel::None);

        Self {
            entity_type: entity_type.into(),
            compliance_status: ComplianceStatus::NonCompliant { violations },
            risk_level,
            reasoning_steps: Vec::new(),
            opinions: Vec::new(),
            confidence: 1.0,
            applicable_statutes: Vec::new(),
            timestamp: chrono::Utc::now(),
        }
    }

    /// Adds a reasoning step
    pub fn with_step(mut self, step: ReasoningStep) -> Self {
        self.reasoning_steps.push(step);
        self
    }

    /// Adds an applicable statute
    pub fn with_statute(mut self, statute_id: impl Into<String>) -> Self {
        self.applicable_statutes.push(statute_id.into());
        self
    }
}

/// A step in the reasoning process
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReasoningStep {
    /// Step number
    pub step: u32,
    /// Statute being evaluated
    pub statute_id: String,
    /// Condition evaluated
    pub condition_description: String,
    /// Result of evaluation
    pub result: bool,
    /// Explanation
    pub explanation: String,
}

/// A legal opinion generated during analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LegalOpinion {
    /// Topic of the opinion
    pub topic: String,
    /// The opinion text
    pub opinion: String,
    /// Supporting legal references
    pub references: Vec<String>,
    /// Confidence level
    pub confidence: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compliance_status() {
        let status = ComplianceStatus::Compliant;
        assert!(status.is_compliant());
        assert!(!status.is_non_compliant());

        let non_compliant = ComplianceStatus::NonCompliant { violations: vec![] };
        assert!(!non_compliant.is_compliant());
        assert!(non_compliant.is_non_compliant());
    }

    #[test]
    fn test_risk_level_ordering() {
        assert!(RiskLevel::Critical > RiskLevel::High);
        assert!(RiskLevel::High > RiskLevel::Medium);
        assert!(RiskLevel::Medium > RiskLevel::Low);
        assert!(RiskLevel::Low > RiskLevel::None);
    }

    #[test]
    fn test_legal_analysis_compliant() {
        let analysis = LegalAnalysis::compliant("EmploymentContract");
        assert!(analysis.compliance_status.is_compliant());
        assert_eq!(analysis.risk_level, RiskLevel::None);
    }

    #[test]
    fn test_legal_analysis_non_compliant() {
        let violations = vec![Violation {
            statute_id: "EA_s38".to_string(),
            description: "Working hours exceed maximum".to_string(),
            severity: ViolationSeverity::Major,
            remedies: vec![],
            legal_reference: "EA s. 38(1)".to_string(),
        }];

        let analysis = LegalAnalysis::non_compliant("EmploymentContract", violations);
        assert!(analysis.compliance_status.is_non_compliant());
        assert_eq!(analysis.risk_level, RiskLevel::High);
    }
}
