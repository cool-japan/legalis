//! Competition Law Validation
//!
//! This module provides comprehensive validation for competition law compliance
//! under Part IV of the Competition and Consumer Act 2010.
//!
//! ## Validation Coverage
//!
//! - Cartel conduct (ss.45AD-45AS)
//! - Misuse of market power (s.46)
//! - Exclusive dealing (s.47)
//! - Mergers and acquisitions (s.50)
//! - Resale price maintenance (s.48)
//!
//! ## Compliance Framework
//!
//! The validator supports:
//! - Pre-emptive compliance checking
//! - Risk assessment
//! - Documentation requirements
//! - Notification/authorisation pathways
//!
//! ## Usage
//!
//! ```rust,ignore
//! use legalis_au::competition::{CompetitionValidator, ValidationResult};
//!
//! let validator = CompetitionValidator::new();
//! let result = validator.validate_cartel_conduct(&conduct);
//! if result.high_risk {
//!     // Take immediate action
//! }
//! ```

use serde::{Deserialize, Serialize};

use super::cartel::{CartelAnalysisResult, CartelAnalyzer, CartelConduct};
use super::error::Result;
use super::exclusive_dealing::{
    ExclusiveDealingAnalyzer, ExclusiveDealingArrangement, ExclusiveDealingResult,
};
use super::market_power::{MarketPowerAnalyzer, Section46Analysis, Section46Result};
use super::mergers::{
    MergerAnalyzer, MergerEffectsAnalysis, MergerMarketAnalysis, MergerResult, MergerTransaction,
};

/// Validation result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidationResult {
    /// Overall compliance status
    pub compliant: bool,
    /// Risk level
    pub risk_level: RiskLevel,
    /// Issues identified
    pub issues: Vec<CompetitionIssue>,
    /// Warnings
    pub warnings: Vec<String>,
    /// Recommendations
    pub recommendations: Vec<String>,
    /// Required actions
    pub required_actions: Vec<RequiredAction>,
}

impl Default for ValidationResult {
    fn default() -> Self {
        Self {
            compliant: true,
            risk_level: RiskLevel::Low,
            issues: Vec::new(),
            warnings: Vec::new(),
            recommendations: Vec::new(),
            required_actions: Vec::new(),
        }
    }
}

impl ValidationResult {
    /// Create compliant result
    pub fn compliant() -> Self {
        Self::default()
    }

    /// Create non-compliant result
    pub fn non_compliant(risk_level: RiskLevel) -> Self {
        Self {
            compliant: false,
            risk_level,
            ..Default::default()
        }
    }

    /// Add issue
    pub fn with_issue(mut self, issue: CompetitionIssue) -> Self {
        self.compliant = false;
        self.issues.push(issue);
        self
    }

    /// Add warning
    pub fn with_warning(mut self, warning: impl Into<String>) -> Self {
        self.warnings.push(warning.into());
        self
    }

    /// Add recommendation
    pub fn with_recommendation(mut self, rec: impl Into<String>) -> Self {
        self.recommendations.push(rec.into());
        self
    }

    /// Add required action
    pub fn with_action(mut self, action: RequiredAction) -> Self {
        self.required_actions.push(action);
        self
    }

    /// Check if any high-risk issues
    pub fn has_high_risk(&self) -> bool {
        matches!(self.risk_level, RiskLevel::High | RiskLevel::Critical)
    }
}

/// Risk level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RiskLevel {
    /// Low risk - likely compliant
    Low,
    /// Medium risk - review recommended
    Medium,
    /// High risk - action required
    High,
    /// Critical - immediate action required
    Critical,
}

impl RiskLevel {
    /// Get description
    pub fn description(&self) -> &'static str {
        match self {
            RiskLevel::Low => "Low risk - likely compliant",
            RiskLevel::Medium => "Medium risk - compliance review recommended",
            RiskLevel::High => "High risk - legal advice required",
            RiskLevel::Critical => "Critical - immediate legal action required",
        }
    }
}

/// Competition issue
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompetitionIssue {
    /// Issue type
    pub issue_type: IssueType,
    /// CCA section
    pub cca_section: String,
    /// Description
    pub description: String,
    /// Severity
    pub severity: IssueSeverity,
    /// Criminal liability potential
    pub criminal_potential: bool,
    /// Mitigation available
    pub mitigation_available: bool,
}

impl CompetitionIssue {
    /// Create cartel issue
    pub fn cartel(description: impl Into<String>) -> Self {
        Self {
            issue_type: IssueType::CartelConduct,
            cca_section: "45AD-45AS".into(),
            description: description.into(),
            severity: IssueSeverity::Critical,
            criminal_potential: true,
            mitigation_available: true, // Leniency policy
        }
    }

    /// Create market power issue
    pub fn market_power(description: impl Into<String>) -> Self {
        Self {
            issue_type: IssueType::MisuseOfMarketPower,
            cca_section: "46".into(),
            description: description.into(),
            severity: IssueSeverity::High,
            criminal_potential: false,
            mitigation_available: true,
        }
    }

    /// Create exclusive dealing issue
    pub fn exclusive_dealing(description: impl Into<String>) -> Self {
        Self {
            issue_type: IssueType::ExclusiveDealing,
            cca_section: "47".into(),
            description: description.into(),
            severity: IssueSeverity::Medium,
            criminal_potential: false,
            mitigation_available: true, // Notification
        }
    }

    /// Create merger issue
    pub fn merger(description: impl Into<String>) -> Self {
        Self {
            issue_type: IssueType::MergerSLC,
            cca_section: "50".into(),
            description: description.into(),
            severity: IssueSeverity::High,
            criminal_potential: false,
            mitigation_available: true, // Undertakings
        }
    }
}

/// Issue type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IssueType {
    /// Cartel conduct
    CartelConduct,
    /// Misuse of market power
    MisuseOfMarketPower,
    /// Exclusive dealing
    ExclusiveDealing,
    /// Merger SLC
    MergerSLC,
    /// Resale price maintenance
    ResalePriceMaintenance,
    /// Other anti-competitive conduct
    Other,
}

/// Issue severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum IssueSeverity {
    /// Low - minor concern
    Low,
    /// Medium - requires attention
    Medium,
    /// High - significant concern
    High,
    /// Critical - immediate action required
    Critical,
}

/// Required action
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequiredAction {
    /// Action type
    pub action_type: ActionType,
    /// Description
    pub description: String,
    /// Priority
    pub priority: ActionPriority,
    /// Deadline (if applicable)
    pub deadline: Option<String>,
}

impl RequiredAction {
    /// Create legal advice action
    pub fn legal_advice(description: impl Into<String>) -> Self {
        Self {
            action_type: ActionType::SeekLegalAdvice,
            description: description.into(),
            priority: ActionPriority::High,
            deadline: None,
        }
    }

    /// Create documentation action
    pub fn documentation(description: impl Into<String>) -> Self {
        Self {
            action_type: ActionType::DocumentJustification,
            description: description.into(),
            priority: ActionPriority::Medium,
            deadline: None,
        }
    }

    /// Create notification action
    pub fn notification(description: impl Into<String>) -> Self {
        Self {
            action_type: ActionType::LodgeNotification,
            description: description.into(),
            priority: ActionPriority::High,
            deadline: None,
        }
    }
}

/// Action type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionType {
    /// Seek legal advice
    SeekLegalAdvice,
    /// Document business justification
    DocumentJustification,
    /// Lodge ACCC notification
    LodgeNotification,
    /// Seek authorisation
    SeekAuthorisation,
    /// Modify conduct
    ModifyConduct,
    /// Cease conduct
    CeaseConduct,
    /// Internal investigation
    InternalInvestigation,
    /// ACCC engagement
    AcccEngagement,
    /// Consider leniency
    ConsiderLeniency,
}

/// Action priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ActionPriority {
    /// Low priority
    Low,
    /// Medium priority
    Medium,
    /// High priority
    High,
    /// Urgent
    Urgent,
}

/// Competition law validator
pub struct CompetitionValidator {
    /// Risk tolerance
    risk_tolerance: RiskLevel,
    /// Include recommendations
    #[allow(dead_code)]
    include_recommendations: bool,
}

impl Default for CompetitionValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl CompetitionValidator {
    /// Create new validator
    pub fn new() -> Self {
        Self {
            risk_tolerance: RiskLevel::Medium,
            include_recommendations: true,
        }
    }

    /// Set risk tolerance
    pub fn with_risk_tolerance(mut self, tolerance: RiskLevel) -> Self {
        self.risk_tolerance = tolerance;
        self
    }

    /// Validate cartel conduct
    pub fn validate_cartel(&self, conduct: &CartelConduct) -> Result<ValidationResult> {
        let analysis = CartelAnalyzer::analyze(conduct);
        let result = self.cartel_analysis_to_result(&analysis);
        Ok(result)
    }

    /// Validate market power conduct
    pub fn validate_market_power(&self, analysis: &Section46Analysis) -> Result<ValidationResult> {
        let s46_result = MarketPowerAnalyzer::analyze_contravention(analysis);
        let result = self.s46_result_to_validation(&s46_result);
        Ok(result)
    }

    /// Validate exclusive dealing
    pub fn validate_exclusive_dealing(
        &self,
        arrangement: &ExclusiveDealingArrangement,
    ) -> Result<ValidationResult> {
        let ed_result = ExclusiveDealingAnalyzer::analyze(arrangement);
        let result = self.ed_result_to_validation(&ed_result);
        Ok(result)
    }

    /// Validate merger
    pub fn validate_merger(
        &self,
        transaction: &MergerTransaction,
        market_analysis: &MergerMarketAnalysis,
        effects_analysis: &MergerEffectsAnalysis,
    ) -> Result<ValidationResult> {
        let merger_result = MergerAnalyzer::analyze(transaction, market_analysis, effects_analysis);
        let result = self.merger_result_to_validation(&merger_result);
        Ok(result)
    }

    /// Convert cartel analysis to validation result
    fn cartel_analysis_to_result(&self, analysis: &CartelAnalysisResult) -> ValidationResult {
        if !analysis.is_cartel_conduct {
            return ValidationResult::compliant()
                .with_recommendation("Maintain documentation of business justification");
        }

        let risk = if analysis.criminal_liability {
            RiskLevel::Critical
        } else if analysis.civil_liability {
            RiskLevel::High
        } else {
            RiskLevel::Medium
        };

        let mut result = ValidationResult::non_compliant(risk)
            .with_issue(CompetitionIssue::cartel(&analysis.reasoning));

        if analysis.criminal_liability {
            result = result
                .with_action(RequiredAction {
                    action_type: ActionType::SeekLegalAdvice,
                    description: "Immediate legal advice required - criminal liability".into(),
                    priority: ActionPriority::Urgent,
                    deadline: None,
                })
                .with_action(RequiredAction {
                    action_type: ActionType::ConsiderLeniency,
                    description: "Consider ACCC immunity/leniency policy".into(),
                    priority: ActionPriority::Urgent,
                    deadline: None,
                });
        }

        for rec in &analysis.recommendations {
            result = result.with_recommendation(rec.clone());
        }

        result
    }

    /// Convert s.46 result to validation result
    fn s46_result_to_validation(&self, analysis: &Section46Result) -> ValidationResult {
        if !analysis.contravention_likely {
            return ValidationResult::compliant()
                .with_recommendation("Document business justification for conduct");
        }

        let risk =
            if analysis.market_power.has_substantial_power && analysis.slc_finding.any_satisfied {
                RiskLevel::High
            } else {
                RiskLevel::Medium
            };

        let mut result = ValidationResult::non_compliant(risk)
            .with_issue(CompetitionIssue::market_power(&analysis.reasoning));

        result = result.with_action(RequiredAction::legal_advice(
            "Review conduct for s.46 compliance",
        ));

        for rec in &analysis.recommendations {
            result = result.with_recommendation(rec.clone());
        }

        result
    }

    /// Convert exclusive dealing result to validation result
    fn ed_result_to_validation(&self, analysis: &ExclusiveDealingResult) -> ValidationResult {
        if !analysis.is_exclusive_dealing || !analysis.likely_slc {
            return ValidationResult::compliant()
                .with_recommendation("Monitor for cumulative market effects");
        }

        let risk = if analysis.contravention_likely {
            RiskLevel::Medium
        } else {
            RiskLevel::Low
        };

        let mut result = ValidationResult::non_compliant(risk)
            .with_issue(CompetitionIssue::exclusive_dealing(&analysis.reasoning));

        if analysis.contravention_likely {
            result = result.with_action(RequiredAction::notification(
                "Consider lodging notification with ACCC",
            ));
        }

        for rec in &analysis.recommendations {
            result = result.with_recommendation(rec.clone());
        }

        result
    }

    /// Convert merger result to validation result
    fn merger_result_to_validation(&self, analysis: &MergerResult) -> ValidationResult {
        use super::mergers::AcccOutcome;

        if !analysis.likely_slc {
            return ValidationResult::compliant()
                .with_recommendation("Proceed with informal clearance process");
        }

        let risk = match analysis.accc_likely_outcome {
            AcccOutcome::OppositionLikely => RiskLevel::High,
            AcccOutcome::ConditionalClearance => RiskLevel::Medium,
            AcccOutcome::Uncertain => RiskLevel::Medium,
            AcccOutcome::ClearanceLikely => RiskLevel::Low,
        };

        let mut result = ValidationResult::non_compliant(risk)
            .with_issue(CompetitionIssue::merger(&analysis.reasoning));

        result = result.with_action(RequiredAction {
            action_type: ActionType::AcccEngagement,
            description: "Engage with ACCC merger review process".into(),
            priority: ActionPriority::High,
            deadline: None,
        });

        if !analysis.possible_conditions.is_empty() {
            result = result.with_action(RequiredAction {
                action_type: ActionType::ModifyConduct,
                description: "Consider undertakings/divestiture to address concerns".into(),
                priority: ActionPriority::Medium,
                deadline: None,
            });
        }

        for rec in &analysis.recommendations {
            result = result.with_recommendation(rec.clone());
        }

        result
    }

    /// Quick validation check
    pub fn quick_check(&self, risk_level: RiskLevel) -> bool {
        risk_level <= self.risk_tolerance
    }
}

/// Compliance report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReport {
    /// Report date
    pub date: String,
    /// Entity name
    pub entity_name: String,
    /// Overall compliance status
    pub overall_status: OverallStatus,
    /// Validation results
    pub results: Vec<ValidationResult>,
    /// Summary
    pub summary: String,
    /// Executive recommendations
    pub executive_recommendations: Vec<String>,
}

/// Overall compliance status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OverallStatus {
    /// Compliant
    Compliant,
    /// Compliant with warnings
    CompliantWithWarnings,
    /// Non-compliant
    NonCompliant,
    /// Critical issues
    Critical,
}

impl ComplianceReport {
    /// Generate compliance report from multiple results
    pub fn generate(entity_name: impl Into<String>, results: Vec<ValidationResult>) -> Self {
        let overall = Self::determine_overall_status(&results);
        let summary = Self::generate_summary(&results, overall);
        let recommendations = Self::generate_executive_recommendations(&results);

        Self {
            date: chrono::Utc::now().format("%Y-%m-%d").to_string(),
            entity_name: entity_name.into(),
            overall_status: overall,
            results,
            summary,
            executive_recommendations: recommendations,
        }
    }

    fn determine_overall_status(results: &[ValidationResult]) -> OverallStatus {
        let has_critical = results.iter().any(|r| r.risk_level == RiskLevel::Critical);
        let has_high = results.iter().any(|r| r.risk_level == RiskLevel::High);
        let has_issues = results.iter().any(|r| !r.compliant);
        let has_warnings = results.iter().any(|r| !r.warnings.is_empty());

        if has_critical {
            OverallStatus::Critical
        } else if has_high || has_issues {
            OverallStatus::NonCompliant
        } else if has_warnings {
            OverallStatus::CompliantWithWarnings
        } else {
            OverallStatus::Compliant
        }
    }

    fn generate_summary(results: &[ValidationResult], status: OverallStatus) -> String {
        let issue_count: usize = results.iter().map(|r| r.issues.len()).sum();
        let warning_count: usize = results.iter().map(|r| r.warnings.len()).sum();

        match status {
            OverallStatus::Compliant => "No compliance issues identified".into(),
            OverallStatus::CompliantWithWarnings => {
                format!(
                    "{} warning(s) identified - review recommended",
                    warning_count
                )
            }
            OverallStatus::NonCompliant => {
                format!("{} issue(s) identified - action required", issue_count)
            }
            OverallStatus::Critical => {
                format!(
                    "CRITICAL: {} issue(s) identified - immediate action required",
                    issue_count
                )
            }
        }
    }

    fn generate_executive_recommendations(results: &[ValidationResult]) -> Vec<String> {
        let mut recs = Vec::new();

        let has_cartel = results.iter().any(|r| {
            r.issues
                .iter()
                .any(|i| i.issue_type == IssueType::CartelConduct)
        });

        if has_cartel {
            recs.push("URGENT: Engage external competition law counsel immediately".into());
            recs.push("Consider ACCC immunity/leniency policy".into());
        }

        let high_risk_count = results.iter().filter(|r| r.has_high_risk()).count();
        if high_risk_count > 0 {
            recs.push(format!(
                "Address {} high-risk issue(s) as priority",
                high_risk_count
            ));
        }

        if recs.is_empty() {
            recs.push("Continue regular competition compliance monitoring".into());
        }

        recs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_result_default() {
        let result = ValidationResult::default();
        assert!(result.compliant);
        assert_eq!(result.risk_level, RiskLevel::Low);
    }

    #[test]
    fn test_validation_result_with_issue() {
        let result =
            ValidationResult::compliant().with_issue(CompetitionIssue::cartel("Test cartel issue"));

        assert!(!result.compliant);
        assert_eq!(result.issues.len(), 1);
    }

    #[test]
    fn test_risk_level_ordering() {
        assert!(RiskLevel::Low < RiskLevel::Medium);
        assert!(RiskLevel::Medium < RiskLevel::High);
        assert!(RiskLevel::High < RiskLevel::Critical);
    }

    #[test]
    fn test_competition_issue_creation() {
        let cartel = CompetitionIssue::cartel("Price fixing");
        assert!(cartel.criminal_potential);
        assert_eq!(cartel.cca_section, "45AD-45AS");

        let market_power = CompetitionIssue::market_power("Predatory pricing");
        assert!(!market_power.criminal_potential);
        assert_eq!(market_power.cca_section, "46");
    }

    #[test]
    fn test_validator_creation() {
        let validator = CompetitionValidator::new().with_risk_tolerance(RiskLevel::High);

        assert!(validator.quick_check(RiskLevel::Medium));
        assert!(validator.quick_check(RiskLevel::High));
        assert!(!validator.quick_check(RiskLevel::Critical));
    }

    #[test]
    fn test_compliance_report_generation() {
        let results = vec![
            ValidationResult::compliant(),
            ValidationResult::compliant().with_warning("Minor concern"),
        ];

        let report = ComplianceReport::generate("Test Corp", results);

        assert_eq!(report.overall_status, OverallStatus::CompliantWithWarnings);
        assert!(report.summary.contains("warning"));
    }

    #[test]
    fn test_compliance_report_critical() {
        let results = vec![
            ValidationResult::non_compliant(RiskLevel::Critical)
                .with_issue(CompetitionIssue::cartel("Price fixing cartel")),
        ];

        let report = ComplianceReport::generate("Bad Corp", results);

        assert_eq!(report.overall_status, OverallStatus::Critical);
        assert!(
            report
                .executive_recommendations
                .iter()
                .any(|r| r.contains("URGENT"))
        );
    }

    #[test]
    fn test_required_action_creation() {
        let action = RequiredAction::legal_advice("Review conduct");
        assert_eq!(action.action_type, ActionType::SeekLegalAdvice);
        assert_eq!(action.priority, ActionPriority::High);
    }
}
