//! Types for legal reasoning and analysis.
//!
//! This module defines the core types used for automated legal analysis,
//! including analysis results, violations, compliance status, and legal opinions.

use chrono::NaiveDate;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Entity type being analyzed
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum EntityType {
    /// Contract (Code civil - Contract law)
    Contract,
    /// Employment contract (Code du travail - Labor law)
    EmploymentContract,
    /// Articles of incorporation (Code de commerce - Company law)
    ArticlesOfIncorporation,
    /// Working hours (Code du travail)
    WorkingHours,
    /// Dismissal (Code du travail)
    Dismissal,
    /// Marriage (Code civil - Family law)
    Marriage,
    /// Divorce (Code civil - Family law)
    Divorce,
}

impl EntityType {
    /// Get French name of the entity type
    #[must_use]
    pub const fn french_name(&self) -> &'static str {
        match self {
            Self::Contract => "Contrat",
            Self::EmploymentContract => "Contrat de travail",
            Self::ArticlesOfIncorporation => "Statuts de société",
            Self::WorkingHours => "Durée du travail",
            Self::Dismissal => "Licenciement",
            Self::Marriage => "Mariage",
            Self::Divorce => "Divorce",
        }
    }

    /// Get English name of the entity type
    #[must_use]
    pub const fn english_name(&self) -> &'static str {
        match self {
            Self::Contract => "Contract",
            Self::EmploymentContract => "Employment contract",
            Self::ArticlesOfIncorporation => "Articles of incorporation",
            Self::WorkingHours => "Working hours",
            Self::Dismissal => "Dismissal",
            Self::Marriage => "Marriage",
            Self::Divorce => "Divorce",
        }
    }
}

/// Comprehensive legal analysis result
///
/// This represents the complete output of a legal reasoning analysis,
/// including detected violations, compliance status, and recommendations.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LegalAnalysis {
    /// Type of entity analyzed
    pub entity_type: EntityType,

    /// List of applicable statute IDs
    pub applicable_statutes: Vec<String>,

    /// Detected violations
    pub violations: Vec<Violation>,

    /// Overall compliance status
    pub compliance_status: ComplianceStatus,

    /// Legal opinion and recommendations
    pub legal_opinion: LegalOpinion,

    /// Confidence level (0.0-1.0)
    pub confidence: f64,

    /// Reasoning chain showing how conclusion was reached
    pub reasoning_chain: Vec<ReasoningStep>,

    /// Analysis timestamp
    pub timestamp: NaiveDate,
}

impl LegalAnalysis {
    /// Create a new legal analysis
    #[must_use]
    pub fn new(entity_type: EntityType) -> Self {
        Self {
            entity_type,
            applicable_statutes: Vec::new(),
            violations: Vec::new(),
            compliance_status: ComplianceStatus::Compliant,
            legal_opinion: LegalOpinion::default(),
            confidence: 1.0,
            reasoning_chain: Vec::new(),
            timestamp: chrono::Utc::now().naive_utc().date(),
        }
    }

    /// Add an applicable statute
    pub fn add_statute(&mut self, statute_id: impl Into<String>) {
        self.applicable_statutes.push(statute_id.into());
    }

    /// Add a violation
    pub fn add_violation(&mut self, violation: Violation) {
        self.violations.push(violation);
    }

    /// Add a reasoning step
    pub fn add_reasoning_step(&mut self, step: ReasoningStep) {
        self.reasoning_chain.push(step);
    }

    /// Check if analysis found any violations
    #[must_use]
    pub fn has_violations(&self) -> bool {
        !self.violations.is_empty()
    }

    /// Check if analysis is compliant
    #[must_use]
    pub fn is_compliant(&self) -> bool {
        matches!(self.compliance_status, ComplianceStatus::Compliant)
    }

    /// Get count of critical violations
    #[must_use]
    pub fn critical_violation_count(&self) -> usize {
        self.violations
            .iter()
            .filter(|v| v.severity == ViolationSeverity::Critical)
            .count()
    }
}

/// A detected violation of a statute
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Violation {
    /// Statute ID (e.g., "code-civil-1128")
    pub article_id: String,

    /// French title of the article
    pub article_title_fr: String,

    /// English title of the article
    pub article_title_en: String,

    /// Severity of the violation
    pub severity: ViolationSeverity,

    /// French description of the violation
    pub description_fr: String,

    /// English description of the violation
    pub description_en: String,

    /// Available remedies
    pub remedies: Vec<Remedy>,
}

impl Violation {
    /// Create a new violation
    #[must_use]
    pub fn new(
        article_id: impl Into<String>,
        severity: ViolationSeverity,
        description_fr: impl Into<String>,
        description_en: impl Into<String>,
    ) -> Self {
        let article_id = article_id.into();
        let (title_fr, title_en) = Self::default_titles(&article_id);

        Self {
            article_id,
            article_title_fr: title_fr,
            article_title_en: title_en,
            severity,
            description_fr: description_fr.into(),
            description_en: description_en.into(),
            remedies: Vec::new(),
        }
    }

    /// Add a remedy
    pub fn with_remedy(mut self, remedy: Remedy) -> Self {
        self.remedies.push(remedy);
        self
    }

    /// Set article titles
    pub fn with_titles(mut self, title_fr: impl Into<String>, title_en: impl Into<String>) -> Self {
        self.article_title_fr = title_fr.into();
        self.article_title_en = title_en.into();
        self
    }

    fn default_titles(article_id: &str) -> (String, String) {
        (
            format!("Article {}", article_id),
            format!("Article {}", article_id),
        )
    }
}

/// Severity level of a violation
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ViolationSeverity {
    /// Low severity - minor irregularity
    Low,
    /// Medium severity - defective performance
    Medium,
    /// High severity - breach with damages
    High,
    /// Critical severity - contract void/nullité
    Critical,
}

/// Legal remedy available for a violation
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Remedy {
    /// Type of remedy
    pub remedy_type: RemedyType,

    /// French description
    pub description_fr: String,

    /// English description
    pub description_en: String,

    /// Estimated damages amount (if applicable)
    pub estimated_damages: Option<u64>,
}

impl Remedy {
    /// Create a new remedy
    #[must_use]
    pub fn new(
        remedy_type: RemedyType,
        description_fr: impl Into<String>,
        description_en: impl Into<String>,
    ) -> Self {
        Self {
            remedy_type,
            description_fr: description_fr.into(),
            description_en: description_en.into(),
            estimated_damages: None,
        }
    }

    /// Set estimated damages
    #[must_use]
    pub fn with_damages(mut self, amount: u64) -> Self {
        self.estimated_damages = Some(amount);
        self
    }
}

/// Type of legal remedy
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RemedyType {
    /// Specific performance (exécution forcée)
    SpecificPerformance,
    /// Price reduction (réduction du prix)
    PriceReduction,
    /// Contract termination (résolution)
    Termination,
    /// Damages (dommages-intérêts)
    Damages,
    /// Exception of non-performance (exception d'inexécution)
    ExceptionNonPerformance,
    /// Nullity (nullité)
    Nullity,
    /// Other remedy
    Other(String),
}

/// Overall compliance status
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ComplianceStatus {
    /// Fully compliant with all applicable statutes
    Compliant,
    /// Minor issues that do not affect validity
    MinorIssues(Vec<String>),
    /// Major violations detected
    MajorViolations(Vec<String>),
    /// Entity is invalid
    Invalid,
}

impl ComplianceStatus {
    /// Check if status is compliant
    #[must_use]
    pub const fn is_compliant(&self) -> bool {
        matches!(self, Self::Compliant)
    }

    /// Check if status indicates invalidity
    #[must_use]
    pub const fn is_invalid(&self) -> bool {
        matches!(self, Self::Invalid)
    }
}

/// Legal opinion with recommendations
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LegalOpinion {
    /// French summary
    pub summary_fr: String,

    /// English summary
    pub summary_en: String,

    /// French recommendations
    pub recommendations_fr: Vec<String>,

    /// English recommendations
    pub recommendations_en: Vec<String>,

    /// Risk level
    pub risk_level: RiskLevel,
}

impl LegalOpinion {
    /// Create a new legal opinion
    #[must_use]
    pub fn new(
        summary_fr: impl Into<String>,
        summary_en: impl Into<String>,
        risk_level: RiskLevel,
    ) -> Self {
        Self {
            summary_fr: summary_fr.into(),
            summary_en: summary_en.into(),
            recommendations_fr: Vec::new(),
            recommendations_en: Vec::new(),
            risk_level,
        }
    }

    /// Add a bilingual recommendation
    pub fn add_recommendation(&mut self, fr: impl Into<String>, en: impl Into<String>) {
        self.recommendations_fr.push(fr.into());
        self.recommendations_en.push(en.into());
    }
}

/// Risk level assessment
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RiskLevel {
    /// Low risk
    #[default]
    Low,
    /// Medium risk
    Medium,
    /// High risk
    High,
    /// Critical risk
    Critical,
}

/// A step in the reasoning chain
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ReasoningStep {
    /// Step number
    pub step: usize,

    /// Statute applied
    pub statute_id: String,

    /// Condition evaluated
    pub condition_description: String,

    /// Result of evaluation
    pub result: bool,

    /// Explanation (French)
    pub explanation_fr: String,

    /// Explanation (English)
    pub explanation_en: String,
}

impl ReasoningStep {
    /// Create a new reasoning step
    #[must_use]
    pub fn new(
        step: usize,
        statute_id: impl Into<String>,
        condition_description: impl Into<String>,
        result: bool,
        explanation_fr: impl Into<String>,
        explanation_en: impl Into<String>,
    ) -> Self {
        Self {
            step,
            statute_id: statute_id.into(),
            condition_description: condition_description.into(),
            result,
            explanation_fr: explanation_fr.into(),
            explanation_en: explanation_en.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legal_analysis_creation() {
        let analysis = LegalAnalysis::new(EntityType::Contract);
        assert_eq!(analysis.entity_type, EntityType::Contract);
        assert!(analysis.applicable_statutes.is_empty());
        assert!(analysis.violations.is_empty());
        assert!(analysis.is_compliant());
        assert_eq!(analysis.confidence, 1.0);
    }

    #[test]
    fn test_violation_creation() {
        let violation = Violation::new(
            "code-civil-1128",
            ViolationSeverity::Critical,
            "Absence de consentement",
            "No consent given",
        );

        assert_eq!(violation.article_id, "code-civil-1128");
        assert_eq!(violation.severity, ViolationSeverity::Critical);
        assert!(violation.remedies.is_empty());
    }

    #[test]
    fn test_violation_with_remedy() {
        let remedy = Remedy::new(
            RemedyType::Nullity,
            "Nullité du contrat",
            "Contract nullity",
        );

        let violation = Violation::new(
            "code-civil-1128",
            ViolationSeverity::Critical,
            "Absence de consentement",
            "No consent given",
        )
        .with_remedy(remedy);

        assert_eq!(violation.remedies.len(), 1);
    }

    #[test]
    fn test_compliance_status() {
        assert!(ComplianceStatus::Compliant.is_compliant());
        assert!(!ComplianceStatus::Invalid.is_compliant());
        assert!(ComplianceStatus::Invalid.is_invalid());
    }

    #[test]
    fn test_violation_severity_ordering() {
        assert!(ViolationSeverity::Low < ViolationSeverity::Medium);
        assert!(ViolationSeverity::Medium < ViolationSeverity::High);
        assert!(ViolationSeverity::High < ViolationSeverity::Critical);
    }

    #[test]
    fn test_risk_level_ordering() {
        assert!(RiskLevel::Low < RiskLevel::Medium);
        assert!(RiskLevel::Medium < RiskLevel::High);
        assert!(RiskLevel::High < RiskLevel::Critical);
    }

    #[test]
    fn test_entity_type_names() {
        assert_eq!(EntityType::Contract.french_name(), "Contrat");
        assert_eq!(EntityType::Contract.english_name(), "Contract");
        assert_eq!(
            EntityType::EmploymentContract.french_name(),
            "Contrat de travail"
        );
        assert_eq!(
            EntityType::EmploymentContract.english_name(),
            "Employment contract"
        );
    }

    #[test]
    fn test_legal_analysis_mutations() {
        let mut analysis = LegalAnalysis::new(EntityType::Contract);

        analysis.add_statute("code-civil-1128");
        assert_eq!(analysis.applicable_statutes.len(), 1);

        let violation = Violation::new(
            "code-civil-1128",
            ViolationSeverity::High,
            "Violation détectée",
            "Violation detected",
        );
        analysis.add_violation(violation);
        assert_eq!(analysis.violations.len(), 1);
        assert!(analysis.has_violations());
    }

    #[test]
    fn test_remedy_with_damages() {
        let remedy =
            Remedy::new(RemedyType::Damages, "Dommages-intérêts", "Damages").with_damages(10_000);

        assert_eq!(remedy.estimated_damages, Some(10_000));
    }

    #[test]
    fn test_legal_opinion() {
        let mut opinion = LegalOpinion::new("Contrat valide", "Contract valid", RiskLevel::Low);

        opinion.add_recommendation("Aucune action requise", "No action required");

        assert_eq!(opinion.recommendations_fr.len(), 1);
        assert_eq!(opinion.recommendations_en.len(), 1);
    }

    #[test]
    fn test_reasoning_step() {
        let step = ReasoningStep::new(
            1,
            "code-civil-1128",
            "Consent check",
            true,
            "Consentement vérifié",
            "Consent verified",
        );

        assert_eq!(step.step, 1);
        assert!(step.result);
    }
}
