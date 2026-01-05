//! Compliance-focused diffing for regulatory and policy analysis.
//!
//! This module provides specialized analysis for compliance and regulatory
//! impacts, including breaking change detection, backward compatibility
//! analysis, and enforcement date tracking.
//!
//! # Examples
//!
//! ```
//! use legalis_core::{Statute, Effect, EffectType};
//! use legalis_diff::{diff, compliance};
//!
//! let old = Statute::new("reg-123", "Regulation", Effect::new(EffectType::Grant, "Permission"));
//! let mut new = old.clone();
//! new.effect = Effect::new(EffectType::Revoke, "Revoke permission");
//!
//! let diff_result = diff(&old, &new).unwrap();
//! let compliance_analysis = compliance::analyze_regulatory_impact(&diff_result);
//!
//! assert!(compliance_analysis.has_breaking_changes);
//! ```

use crate::{Change, ChangeTarget, ChangeType, Severity, StatuteDiff};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Regulatory impact analysis results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulatoryImpactAnalysis {
    /// Statute ID.
    pub statute_id: String,
    /// Whether this change introduces breaking changes.
    pub has_breaking_changes: bool,
    /// Breaking change details.
    pub breaking_changes: Vec<BreakingChange>,
    /// Backward compatibility score (0.0 to 1.0).
    pub backward_compatibility_score: f64,
    /// Compliance gaps introduced.
    pub compliance_gaps: Vec<ComplianceGap>,
    /// Required migration steps.
    pub migration_steps: Vec<String>,
    /// Risk level.
    pub risk_level: RiskLevel,
    /// Regulatory framework impacts.
    pub framework_impacts: Vec<FrameworkImpact>,
}

/// A breaking change in a statute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakingChange {
    /// Type of breaking change.
    pub change_type: BreakingChangeType,
    /// Description of the change.
    pub description: String,
    /// What was changed.
    pub target: ChangeTarget,
    /// Impact severity (0-10).
    pub severity: u8,
    /// Entities affected.
    pub affected_entities: Vec<String>,
    /// Mitigation strategies.
    pub mitigation_strategies: Vec<String>,
}

/// Type of breaking change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BreakingChangeType {
    /// Eligibility criteria tightened.
    EligibilityTightened,
    /// Benefits or rights reduced.
    BenefitsReduced,
    /// New obligations added.
    ObligationsAdded,
    /// Effect fundamentally changed.
    EffectChanged,
    /// Discretion requirements changed.
    DiscretionChanged,
    /// Deadlines or timeframes changed.
    TimeframeChanged,
}

/// Risk level assessment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RiskLevel {
    /// Low risk to compliance.
    Low,
    /// Medium risk.
    Medium,
    /// High risk.
    High,
    /// Critical risk requiring immediate attention.
    Critical,
}

/// A compliance gap introduced by changes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceGap {
    /// Gap description.
    pub description: String,
    /// Regulatory framework affected.
    pub framework: String,
    /// Severity of the gap.
    pub severity: RiskLevel,
    /// Remediation steps.
    pub remediation: Vec<String>,
}

/// Impact on a regulatory framework.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameworkImpact {
    /// Framework name (e.g., "GDPR", "HIPAA", "SOX").
    pub framework: String,
    /// Specific requirements affected.
    pub affected_requirements: Vec<String>,
    /// Impact description.
    pub impact_description: String,
    /// Compliance status after change.
    pub compliance_status: ComplianceStatus,
}

/// Compliance status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComplianceStatus {
    /// Fully compliant.
    Compliant,
    /// Partially compliant.
    PartiallyCompliant,
    /// Non-compliant.
    NonCompliant,
    /// Unknown compliance status.
    Unknown,
}

/// Backward compatibility analysis results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackwardCompatibilityAnalysis {
    /// Statute ID.
    pub statute_id: String,
    /// Whether backward compatible.
    pub is_backward_compatible: bool,
    /// Compatibility score (0.0 to 1.0).
    pub compatibility_score: f64,
    /// Incompatible changes.
    pub incompatible_changes: Vec<IncompatibleChange>,
    /// Deprecation notices.
    pub deprecations: Vec<Deprecation>,
    /// Version compatibility matrix.
    pub version_compatibility: HashMap<String, bool>,
}

/// An incompatible change.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncompatibleChange {
    /// Change description.
    pub description: String,
    /// What changed.
    pub target: ChangeTarget,
    /// Why it's incompatible.
    pub incompatibility_reason: String,
    /// Workaround or migration path.
    pub workaround: Option<String>,
}

/// A deprecation notice.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deprecation {
    /// What is being deprecated.
    pub deprecated_item: String,
    /// Replacement (if any).
    pub replacement: Option<String>,
    /// Deprecation effective date.
    pub effective_date: Option<DateTime<Utc>>,
    /// Removal date (if planned).
    pub removal_date: Option<DateTime<Utc>>,
    /// Deprecation reason.
    pub reason: String,
}

/// Enforcement date tracking.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnforcementSchedule {
    /// Statute ID.
    pub statute_id: String,
    /// Enactment date.
    pub enactment_date: Option<DateTime<Utc>>,
    /// Effective date.
    pub effective_date: Option<DateTime<Utc>>,
    /// Enforcement date.
    pub enforcement_date: Option<DateTime<Utc>>,
    /// Grace period end date.
    pub grace_period_end: Option<DateTime<Utc>>,
    /// Sunset/expiration date.
    pub sunset_date: Option<DateTime<Utc>>,
    /// Phase-in schedule.
    pub phases: Vec<EnforcementPhase>,
}

/// A phase in enforcement schedule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnforcementPhase {
    /// Phase name/description.
    pub name: String,
    /// Start date.
    pub start_date: DateTime<Utc>,
    /// End date (optional).
    pub end_date: Option<DateTime<Utc>>,
    /// What applies in this phase.
    pub requirements: Vec<String>,
}

/// Analyzes regulatory impact of a diff.
///
/// # Examples
///
/// ```
/// use legalis_core::{Statute, Effect, EffectType};
/// use legalis_diff::{diff, compliance::analyze_regulatory_impact};
///
/// let old = Statute::new("law", "Title", Effect::new(EffectType::Grant, "Benefit"));
/// let mut new = old.clone();
/// new.effect = Effect::new(EffectType::Revoke, "Revoke");
///
/// let diff_result = diff(&old, &new).unwrap();
/// let analysis = analyze_regulatory_impact(&diff_result);
///
/// assert!(analysis.has_breaking_changes);
/// ```
pub fn analyze_regulatory_impact(diff: &StatuteDiff) -> RegulatoryImpactAnalysis {
    let mut breaking_changes = Vec::new();
    let mut compliance_gaps = Vec::new();
    let mut migration_steps = Vec::new();

    // Analyze each change for breaking potential
    for change in &diff.changes {
        if let Some(breaking_change) = classify_breaking_change(change, diff) {
            breaking_changes.push(breaking_change);
        }
    }

    // Calculate backward compatibility score
    let backward_compatibility_score = calculate_compatibility_score(diff, &breaking_changes);

    // Determine risk level
    let risk_level = determine_risk_level(diff, &breaking_changes);

    // Generate migration steps
    if !breaking_changes.is_empty() {
        migration_steps.push("Review all affected use cases and dependencies".to_string());
        migration_steps.push("Update documentation and procedures".to_string());
        migration_steps.push("Notify all stakeholders of changes".to_string());

        if diff.impact.affects_eligibility {
            migration_steps.push("Re-evaluate eligibility for all existing cases".to_string());
        }

        if diff.impact.affects_outcome {
            migration_steps.push("Update expected outcomes in systems and processes".to_string());
        }
    }

    // Detect compliance gaps
    if risk_level >= RiskLevel::High {
        compliance_gaps.push(ComplianceGap {
            description: "High-risk changes require compliance review".to_string(),
            framework: "General".to_string(),
            severity: risk_level,
            remediation: vec![
                "Conduct compliance impact assessment".to_string(),
                "Obtain legal review".to_string(),
            ],
        });
    }

    RegulatoryImpactAnalysis {
        statute_id: diff.statute_id.clone(),
        has_breaking_changes: !breaking_changes.is_empty(),
        breaking_changes,
        backward_compatibility_score,
        compliance_gaps,
        migration_steps,
        risk_level,
        framework_impacts: Vec::new(),
    }
}

/// Analyzes backward compatibility.
pub fn analyze_backward_compatibility(diff: &StatuteDiff) -> BackwardCompatibilityAnalysis {
    let mut incompatible_changes = Vec::new();
    let deprecations = Vec::new();

    // Check for incompatible changes
    for change in &diff.changes {
        match change.change_type {
            ChangeType::Removed => {
                incompatible_changes.push(IncompatibleChange {
                    description: format!("{} was removed", change.target),
                    target: change.target.clone(),
                    incompatibility_reason: "Removal breaks existing dependencies".to_string(),
                    workaround: None,
                });
            }
            ChangeType::Modified => {
                if matches!(change.target, ChangeTarget::Effect) {
                    incompatible_changes.push(IncompatibleChange {
                        description: "Effect was modified".to_string(),
                        target: change.target.clone(),
                        incompatibility_reason: "Changes to effects may alter expected outcomes"
                            .to_string(),
                        workaround: Some(
                            "Review and update code/processes relying on previous effect"
                                .to_string(),
                        ),
                    });
                }
            }
            _ => {}
        }
    }

    let compatibility_score = if incompatible_changes.is_empty() {
        1.0
    } else {
        1.0 - (incompatible_changes.len() as f64 * 0.2).min(1.0)
    };

    BackwardCompatibilityAnalysis {
        statute_id: diff.statute_id.clone(),
        is_backward_compatible: incompatible_changes.is_empty(),
        compatibility_score,
        incompatible_changes,
        deprecations,
        version_compatibility: HashMap::new(),
    }
}

/// Classifies a change as breaking or not.
#[allow(dead_code)]
fn classify_breaking_change(change: &Change, _diff: &StatuteDiff) -> Option<BreakingChange> {
    let (is_breaking, change_type, severity) = match (&change.target, change.change_type) {
        (ChangeTarget::Precondition { .. }, ChangeType::Added) => {
            (true, BreakingChangeType::EligibilityTightened, 8)
        }
        (ChangeTarget::Effect, ChangeType::Modified) => {
            (true, BreakingChangeType::EffectChanged, 9)
        }
        (ChangeTarget::DiscretionLogic, _) => (true, BreakingChangeType::DiscretionChanged, 7),
        (ChangeTarget::Precondition { .. }, ChangeType::Removed) => {
            // Removing preconditions is generally not breaking (expands eligibility)
            (false, BreakingChangeType::EligibilityTightened, 3)
        }
        _ => (false, BreakingChangeType::EffectChanged, 0),
    };

    if is_breaking {
        Some(BreakingChange {
            change_type,
            description: change.description.clone(),
            target: change.target.clone(),
            severity,
            affected_entities: vec!["All current applicants".to_string()],
            mitigation_strategies: vec![
                "Grandfathering existing cases".to_string(),
                "Phased implementation".to_string(),
            ],
        })
    } else {
        None
    }
}

/// Calculates backward compatibility score.
#[allow(dead_code)]
fn calculate_compatibility_score(diff: &StatuteDiff, breaking_changes: &[BreakingChange]) -> f64 {
    if breaking_changes.is_empty() {
        return 1.0;
    }

    let total_changes = diff.changes.len() as f64;
    let breaking_count = breaking_changes.len() as f64;

    // Score based on ratio of breaking to total changes
    let base_score = 1.0 - (breaking_count / total_changes).min(1.0);

    // Adjust based on severity
    let avg_severity = breaking_changes
        .iter()
        .map(|bc| bc.severity as f64)
        .sum::<f64>()
        / breaking_changes.len() as f64;
    let severity_factor = 1.0 - (avg_severity / 10.0);

    (base_score + severity_factor) / 2.0
}

/// Determines overall risk level.
#[allow(dead_code)]
fn determine_risk_level(diff: &StatuteDiff, breaking_changes: &[BreakingChange]) -> RiskLevel {
    if breaking_changes.is_empty() {
        match diff.impact.severity {
            Severity::None | Severity::Minor => RiskLevel::Low,
            Severity::Moderate => RiskLevel::Medium,
            Severity::Major | Severity::Breaking => RiskLevel::High,
        }
    } else {
        let max_severity = breaking_changes
            .iter()
            .map(|bc| bc.severity)
            .max()
            .unwrap_or(0);

        match max_severity {
            0..=3 => RiskLevel::Low,
            4..=6 => RiskLevel::Medium,
            7..=8 => RiskLevel::High,
            _ => RiskLevel::Critical,
        }
    }
}

/// Detects if a change requires compliance review.
pub fn requires_compliance_review(diff: &StatuteDiff) -> bool {
    diff.impact.severity >= Severity::Major
        || diff.impact.affects_outcome
        || diff.impact.affects_eligibility
}

/// Generates a compliance checklist for a diff.
pub fn generate_compliance_checklist(diff: &StatuteDiff) -> Vec<String> {
    let mut checklist = Vec::new();

    checklist.push("Review change documentation".to_string());

    if diff.impact.affects_eligibility {
        checklist.push("Verify eligibility criteria meet regulatory requirements".to_string());
        checklist.push("Check for discrimination or fairness issues".to_string());
    }

    if diff.impact.affects_outcome {
        checklist.push("Ensure outcomes comply with applicable regulations".to_string());
        checklist.push("Verify benefits/obligations are legally valid".to_string());
    }

    if diff.impact.discretion_changed {
        checklist.push("Review discretion guidelines for legal compliance".to_string());
        checklist.push("Ensure discretion criteria are objective and documented".to_string());
    }

    if diff.impact.severity >= Severity::Major {
        checklist.push("Obtain legal counsel review".to_string());
        checklist.push("Document justification for changes".to_string());
        checklist.push("Prepare stakeholder notifications".to_string());
    }

    checklist
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diff;
    use legalis_core::{ComparisonOp, Condition, Effect, EffectType, Statute};

    #[test]
    fn test_analyze_regulatory_impact_no_changes() {
        let statute = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Benefit"));
        let diff_result = diff(&statute, &statute).unwrap();
        let analysis = analyze_regulatory_impact(&diff_result);

        assert!(!analysis.has_breaking_changes);
        assert_eq!(analysis.backward_compatibility_score, 1.0);
        assert_eq!(analysis.risk_level, RiskLevel::Low);
    }

    #[test]
    fn test_analyze_regulatory_impact_effect_change() {
        let old = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Benefit"));
        let mut new = old.clone();
        new.effect = Effect::new(EffectType::Revoke, "Revoke benefit");

        let diff_result = diff(&old, &new).unwrap();
        let analysis = analyze_regulatory_impact(&diff_result);

        assert!(analysis.has_breaking_changes);
        assert!(analysis.risk_level >= RiskLevel::High);
    }

    #[test]
    fn test_analyze_backward_compatibility() {
        let old = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Benefit"));
        let mut new = old.clone();
        new.effect = Effect::new(EffectType::Revoke, "Revoke");

        let diff_result = diff(&old, &new).unwrap();
        let analysis = analyze_backward_compatibility(&diff_result);

        assert!(!analysis.is_backward_compatible);
        assert!(analysis.compatibility_score < 1.0);
    }

    #[test]
    fn test_requires_compliance_review() {
        let old = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Benefit"));
        let mut new = old.clone();
        new.effect = Effect::new(EffectType::Revoke, "Revoke");

        let diff_result = diff(&old, &new).unwrap();
        assert!(requires_compliance_review(&diff_result));
    }

    #[test]
    fn test_generate_compliance_checklist() {
        let old = Statute::new("test", "Test", Effect::new(EffectType::Grant, "Benefit"))
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            });

        let mut new = old.clone();
        new.preconditions.push(Condition::Income {
            operator: ComparisonOp::LessOrEqual,
            value: 50000,
        });

        let diff_result = diff(&old, &new).unwrap();
        let checklist = generate_compliance_checklist(&diff_result);

        assert!(!checklist.is_empty());
        assert!(checklist.iter().any(|item| item.contains("eligibility")));
    }

    #[test]
    fn test_risk_levels() {
        assert!(RiskLevel::Critical > RiskLevel::High);
        assert!(RiskLevel::High > RiskLevel::Medium);
        assert!(RiskLevel::Medium > RiskLevel::Low);
    }
}
