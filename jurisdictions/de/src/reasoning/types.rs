//! Common types for legal reasoning (Gemeinsame Typen für Rechtsanalyse).
//!
//! Provides types for compliance status, risk levels, and violations
//! under German labor law (Arbeitsrecht).

use serde::{Deserialize, Serialize};

/// Compliance status (Compliance-Status)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComplianceStatus {
    /// Rechtskonform / Fully compliant
    Compliant,
    /// Rechtswidrig / Non-compliant
    NonCompliant,
    /// Teilweise konform / Partially compliant
    PartiallyCompliant,
    /// Unbestimmt / Cannot determine
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

/// Risk level (Risikostufe)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RiskLevel {
    /// Kein Risiko / No risk
    None,
    /// Niedriges Risiko / Low risk
    Low,
    /// Mittleres Risiko / Medium risk
    Medium,
    /// Hohes Risiko / High risk
    High,
    /// Kritisches Risiko / Critical - immediate action
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

/// Severity of violation (Schweregrad des Verstoßes)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ViolationSeverity {
    /// Hinweis / Advisory only
    Advisory,
    /// Geringfügiger Verstoß / Minor violation
    Minor,
    /// Mittlerer Verstoß / Moderate violation
    Moderate,
    /// Schwerer Verstoß / Major violation
    Major,
    /// Kritischer Verstoß / Critical violation
    Critical,
}

/// A legal violation (Rechtsverstoß)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Violation {
    /// Statute identifier (Gesetzes-ID)
    pub statute_id: String,
    /// Statute name (Gesetzesname)
    pub statute_name: String,
    /// Description (Beschreibung)
    pub description: String,
    /// Severity (Schweregrad)
    pub severity: ViolationSeverity,
    /// Remediation recommendation (Abhilfemaßnahme)
    pub remediation: Option<String>,
    /// Legal reference (Gesetzliche Grundlage - e.g., "§3 ArbZG")
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

    /// Add remediation (Abhilfemaßnahme hinzufügen)
    #[must_use]
    pub fn with_remediation(mut self, remediation: impl Into<String>) -> Self {
        self.remediation = Some(remediation.into());
        self
    }

    /// Add legal reference (Gesetzliche Grundlage hinzufügen)
    #[must_use]
    pub fn with_legal_reference(mut self, reference: impl Into<String>) -> Self {
        self.legal_reference = Some(reference.into());
        self
    }
}

/// Result of legal analysis (Ergebnis der Rechtsanalyse)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LegalAnalysis {
    /// Compliance status (Compliance-Status)
    pub status: ComplianceStatus,
    /// Risk level (Risikostufe)
    pub risk_level: RiskLevel,
    /// Violations (Verstöße)
    pub violations: Vec<Violation>,
    /// Confidence (Konfidenz - 0.0-1.0)
    pub confidence: f64,
    /// Summary (Zusammenfassung)
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
