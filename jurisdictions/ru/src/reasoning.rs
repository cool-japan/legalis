//! Legal Reasoning Engine for Russian jurisdiction.
//!
//! Integrates with legalis-core to provide:
//! - Rule-based reasoning for Russian laws
//! - Compliance checking
//! - Risk assessment
//! - Legal analysis

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors related to legal reasoning operations
#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum ReasoningError {
    /// Invalid reasoning input
    #[error("Invalid reasoning input: {0}")]
    InvalidInput(String),

    /// Reasoning failed
    #[error("Reasoning failed: {0}")]
    ReasoningFailed(String),

    /// Analysis error
    #[error("Analysis error: {0}")]
    AnalysisError(String),
}

/// Russian legal reasoning engine
pub struct ReasoningEngine {
    /// Enable strict mode
    pub strict_mode: bool,
}

impl ReasoningEngine {
    /// Creates a new reasoning engine
    pub fn new() -> Self {
        Self { strict_mode: false }
    }

    /// Enables strict mode
    pub fn with_strict_mode(mut self) -> Self {
        self.strict_mode = true;
        self
    }

    /// Analyzes legal compliance for Russian laws
    pub fn analyze_compliance(
        &self,
        context: &RuEvaluationContext,
    ) -> Result<LegalAnalysis, ReasoningError> {
        let mut violations = Vec::new();

        // Check labor law compliance
        if let Some(ref labor_context) = context.labor_context
            && labor_context.working_hours_per_week > 40
        {
            violations.push(Violation {
                law_reference: "Трудовой кодекс РФ, ст. 91".to_string(),
                severity: ViolationSeverity::High,
                description: "Working hours exceed 40 hours per week limit".to_string(),
                recommendation: "Adjust working schedule to comply with 40-hour limit".to_string(),
            });
        }

        // Check tax compliance
        if let Some(ref tax_context) = context.tax_context
            && (tax_context.vat_rate < 0.0 || tax_context.vat_rate > 20.0)
        {
            violations.push(Violation {
                law_reference: "Налоговый кодекс РФ, ст. 164".to_string(),
                severity: ViolationSeverity::Critical,
                description: "Invalid VAT rate".to_string(),
                recommendation: "Use standard rate (20%), reduced rate (10%), or zero rate"
                    .to_string(),
            });
        }

        // Check data protection compliance
        if let Some(ref data_context) = context.data_protection_context {
            if data_context.processes_personal_data && !data_context.has_consent {
                violations.push(Violation {
                    law_reference: "152-ФЗ, ст. 9".to_string(),
                    severity: ViolationSeverity::Critical,
                    description: "Processing personal data without consent".to_string(),
                    recommendation: "Obtain explicit consent from data subjects".to_string(),
                });
            }

            if data_context.processes_personal_data && !data_context.registered_with_roskomnadzor {
                violations.push(Violation {
                    law_reference: "152-ФЗ, ст. 22".to_string(),
                    severity: ViolationSeverity::High,
                    description: "Personal data operator not registered with Roskomnadzor"
                        .to_string(),
                    recommendation: "Register with Roskomnadzor as required".to_string(),
                });
            }
        }

        // Check competition law compliance
        if let Some(ref competition_context) = context.competition_context
            && competition_context.market_share > 50.0
        {
            violations.push(Violation {
                law_reference: "135-ФЗ, ст. 5".to_string(),
                severity: ViolationSeverity::Medium,
                description: "Company has dominant market position (>50%)".to_string(),
                recommendation: "Ensure compliance with dominant position regulations".to_string(),
            });
        }

        let compliance_status = if violations.is_empty() {
            ComplianceStatus::Compliant
        } else {
            let has_critical = violations
                .iter()
                .any(|v| matches!(v.severity, ViolationSeverity::Critical));
            if has_critical {
                ComplianceStatus::NonCompliant
            } else {
                ComplianceStatus::PartiallyCompliant
            }
        };

        let risk_level = Self::calculate_risk_level(&violations);

        Ok(LegalAnalysis {
            compliance_status,
            violations,
            risk_level,
            recommendations: self.generate_recommendations(context),
        })
    }

    fn calculate_risk_level(violations: &[Violation]) -> RiskLevel {
        let critical_count = violations
            .iter()
            .filter(|v| matches!(v.severity, ViolationSeverity::Critical))
            .count();
        let high_count = violations
            .iter()
            .filter(|v| matches!(v.severity, ViolationSeverity::High))
            .count();

        if critical_count > 0 {
            RiskLevel::Critical
        } else if high_count > 2 {
            RiskLevel::High
        } else if high_count > 0 {
            RiskLevel::Medium
        } else if !violations.is_empty() {
            RiskLevel::Low
        } else {
            RiskLevel::Minimal
        }
    }

    fn generate_recommendations(&self, _context: &RuEvaluationContext) -> Vec<String> {
        vec![
            "Consult with legal counsel for specific compliance requirements".to_string(),
            "Conduct regular compliance audits".to_string(),
            "Maintain up-to-date documentation".to_string(),
        ]
    }
}

impl Default for ReasoningEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Evaluation context for Russian legal analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuEvaluationContext {
    /// Labor law context
    pub labor_context: Option<LaborContext>,
    /// Tax context
    pub tax_context: Option<TaxContext>,
    /// Data protection context
    pub data_protection_context: Option<DataProtectionContext>,
    /// Competition law context
    pub competition_context: Option<CompetitionContext>,
}

impl RuEvaluationContext {
    /// Creates a new empty evaluation context
    pub fn new() -> Self {
        Self {
            labor_context: None,
            tax_context: None,
            data_protection_context: None,
            competition_context: None,
        }
    }

    /// Sets labor context
    pub fn with_labor_context(mut self, context: LaborContext) -> Self {
        self.labor_context = Some(context);
        self
    }

    /// Sets tax context
    pub fn with_tax_context(mut self, context: TaxContext) -> Self {
        self.tax_context = Some(context);
        self
    }

    /// Sets data protection context
    pub fn with_data_protection_context(mut self, context: DataProtectionContext) -> Self {
        self.data_protection_context = Some(context);
        self
    }

    /// Sets competition context
    pub fn with_competition_context(mut self, context: CompetitionContext) -> Self {
        self.competition_context = Some(context);
        self
    }
}

impl Default for RuEvaluationContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Labor law context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaborContext {
    /// Working hours per week
    pub working_hours_per_week: u32,
    /// Has written employment contract
    pub has_written_contract: bool,
    /// Monthly salary in rubles
    pub monthly_salary_rubles: i64,
}

/// Tax context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxContext {
    /// VAT rate applied
    pub vat_rate: f64,
    /// Income tax rate
    pub income_tax_rate: f64,
    /// Corporate tax applicable
    pub corporate_tax_applicable: bool,
}

/// Data protection context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataProtectionContext {
    /// Processes personal data
    pub processes_personal_data: bool,
    /// Has data subject consent
    pub has_consent: bool,
    /// Registered with Roskomnadzor
    pub registered_with_roskomnadzor: bool,
    /// Has security measures
    pub has_security_measures: bool,
}

/// Competition law context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompetitionContext {
    /// Market share percentage
    pub market_share: f64,
    /// Is in dominant position
    pub is_dominant: bool,
}

/// Legal analysis result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalAnalysis {
    /// Compliance status
    pub compliance_status: ComplianceStatus,
    /// List of violations
    pub violations: Vec<Violation>,
    /// Risk level
    pub risk_level: RiskLevel,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// Compliance status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComplianceStatus {
    /// Fully compliant
    Compliant,
    /// Partially compliant
    PartiallyCompliant,
    /// Non-compliant
    NonCompliant,
}

/// Legal violation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Violation {
    /// Reference to law (e.g., "ТК РФ, ст. 91")
    pub law_reference: String,
    /// Severity of violation
    pub severity: ViolationSeverity,
    /// Description of violation
    pub description: String,
    /// Recommendation to fix
    pub recommendation: String,
}

/// Violation severity
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViolationSeverity {
    /// Critical violation
    Critical,
    /// High severity
    High,
    /// Medium severity
    Medium,
    /// Low severity
    Low,
}

/// Risk level assessment
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    /// Critical risk
    Critical,
    /// High risk
    High,
    /// Medium risk
    Medium,
    /// Low risk
    Low,
    /// Minimal risk
    Minimal,
}

/// Reasoning result type alias
pub type ReasoningResult = Result<LegalAnalysis, ReasoningError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_labor_compliance() {
        let engine = ReasoningEngine::new();

        let context = RuEvaluationContext::new().with_labor_context(LaborContext {
            working_hours_per_week: 45,
            has_written_contract: true,
            monthly_salary_rubles: 50_000,
        });

        let analysis = engine.analyze_compliance(&context).expect("Should succeed");

        assert_eq!(
            analysis.compliance_status,
            ComplianceStatus::PartiallyCompliant
        );
        assert!(!analysis.violations.is_empty());
    }

    #[test]
    fn test_data_protection_compliance() {
        let engine = ReasoningEngine::new();

        let context =
            RuEvaluationContext::new().with_data_protection_context(DataProtectionContext {
                processes_personal_data: true,
                has_consent: false,
                registered_with_roskomnadzor: false,
                has_security_measures: true,
            });

        let analysis = engine.analyze_compliance(&context).expect("Should succeed");

        assert_eq!(analysis.compliance_status, ComplianceStatus::NonCompliant);
        assert!(analysis.violations.len() >= 2);
        assert_eq!(analysis.risk_level, RiskLevel::Critical);
    }

    #[test]
    fn test_competition_compliance() {
        let engine = ReasoningEngine::new();

        let context = RuEvaluationContext::new().with_competition_context(CompetitionContext {
            market_share: 55.0,
            is_dominant: true,
        });

        let analysis = engine.analyze_compliance(&context).expect("Should succeed");

        assert!(!analysis.violations.is_empty());
    }

    #[test]
    fn test_fully_compliant() {
        let engine = ReasoningEngine::new();

        let context = RuEvaluationContext::new()
            .with_labor_context(LaborContext {
                working_hours_per_week: 40,
                has_written_contract: true,
                monthly_salary_rubles: 50_000,
            })
            .with_tax_context(TaxContext {
                vat_rate: 20.0,
                income_tax_rate: 13.0,
                corporate_tax_applicable: true,
            });

        let analysis = engine.analyze_compliance(&context).expect("Should succeed");

        assert_eq!(analysis.compliance_status, ComplianceStatus::Compliant);
        assert!(analysis.violations.is_empty());
        assert_eq!(analysis.risk_level, RiskLevel::Minimal);
    }
}
