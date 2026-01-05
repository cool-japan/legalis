//! Diff templates for common amendment patterns.
//!
//! This module provides predefined templates for recognizing and handling
//! common types of statute amendments.

use crate::{Change, ChangeType, StatuteDiff};
use serde::{Deserialize, Serialize};

/// Common amendment patterns recognized in legal changes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AmendmentPattern {
    /// Age threshold adjustment (retirement age, voting age, etc.)
    AgeThresholdAdjustment,
    /// Income limit modification (for subsidies, tax brackets, etc.)
    IncomeLimitAdjustment,
    /// Benefit amount increase/decrease
    BenefitAmountChange,
    /// Eligibility expansion (removing or relaxing requirements)
    EligibilityExpansion,
    /// Eligibility restriction (adding or tightening requirements)
    EligibilityRestriction,
    /// Sunset clause addition/removal
    SunsetClauseModification,
    /// Discretion introduction (adding human judgment requirement)
    DiscretionIntroduction,
    /// Discretion removal (making deterministic)
    DiscretionRemoval,
    /// Technical correction (typo, clarification)
    TechnicalCorrection,
    /// Fundamental restructure
    FundamentalRestructure,
}

/// Template for recognizing patterns in diffs.
#[derive(Debug, Clone)]
pub struct DiffTemplate {
    /// Pattern name
    pub pattern: AmendmentPattern,
    /// Description of what this pattern means
    pub description: String,
    /// Common reasons for this type of amendment
    pub common_reasons: Vec<String>,
    /// Suggested review checklist
    pub review_checklist: Vec<String>,
}

impl DiffTemplate {
    /// Analyzes a diff and identifies matching patterns.
    pub fn identify_patterns(diff: &StatuteDiff) -> Vec<AmendmentPattern> {
        let mut patterns = Vec::new();

        // Check for age threshold adjustments
        if has_age_threshold_change(&diff.changes) {
            patterns.push(AmendmentPattern::AgeThresholdAdjustment);
        }

        // Check for income limit adjustments
        if has_income_limit_change(&diff.changes) {
            patterns.push(AmendmentPattern::IncomeLimitAdjustment);
        }

        // Check for benefit amount changes
        if has_benefit_amount_change(&diff.changes) {
            patterns.push(AmendmentPattern::BenefitAmountChange);
        }

        // Check for eligibility expansion
        if diff.impact.affects_eligibility {
            let removed_preconds = diff
                .changes
                .iter()
                .filter(|c| {
                    matches!(c.change_type, ChangeType::Removed)
                        && matches!(c.target, crate::ChangeTarget::Precondition { .. })
                })
                .count();

            if removed_preconds > 0 {
                patterns.push(AmendmentPattern::EligibilityExpansion);
            }
        }

        // Check for eligibility restriction
        if diff.impact.affects_eligibility {
            let added_preconds = diff
                .changes
                .iter()
                .filter(|c| {
                    matches!(c.change_type, ChangeType::Added)
                        && matches!(c.target, crate::ChangeTarget::Precondition { .. })
                })
                .count();

            if added_preconds > 0 {
                patterns.push(AmendmentPattern::EligibilityRestriction);
            }
        }

        // Check for discretion changes
        if diff.impact.discretion_changed {
            let discretion_added = diff.changes.iter().any(|c| {
                matches!(c.change_type, ChangeType::Added)
                    && matches!(c.target, crate::ChangeTarget::DiscretionLogic)
            });

            let discretion_removed = diff.changes.iter().any(|c| {
                matches!(c.change_type, ChangeType::Removed)
                    && matches!(c.target, crate::ChangeTarget::DiscretionLogic)
            });

            if discretion_added {
                patterns.push(AmendmentPattern::DiscretionIntroduction);
            } else if discretion_removed {
                patterns.push(AmendmentPattern::DiscretionRemoval);
            }
        }

        // Check for technical corrections
        if is_technical_correction(diff) {
            patterns.push(AmendmentPattern::TechnicalCorrection);
        }

        // Check for fundamental restructure
        if is_fundamental_restructure(diff) {
            patterns.push(AmendmentPattern::FundamentalRestructure);
        }

        patterns
    }

    /// Gets the template for a specific pattern.
    pub fn for_pattern(pattern: &AmendmentPattern) -> Self {
        match pattern {
            AmendmentPattern::AgeThresholdAdjustment => Self {
                pattern: pattern.clone(),
                description: "Modification of age-based eligibility requirements".to_string(),
                common_reasons: vec![
                    "Demographic changes".to_string(),
                    "International standard alignment".to_string(),
                    "Social policy adjustment".to_string(),
                ],
                review_checklist: vec![
                    "Verify impact on currently eligible individuals".to_string(),
                    "Check for transitional provisions".to_string(),
                    "Assess budgetary impact".to_string(),
                    "Review for age discrimination concerns".to_string(),
                ],
            },

            AmendmentPattern::IncomeLimitAdjustment => Self {
                pattern: pattern.clone(),
                description: "Adjustment of income-based eligibility thresholds".to_string(),
                common_reasons: vec![
                    "Inflation adjustment".to_string(),
                    "Cost of living changes".to_string(),
                    "Budget constraints".to_string(),
                    "Expanding/restricting benefit coverage".to_string(),
                ],
                review_checklist: vec![
                    "Calculate affected population size".to_string(),
                    "Check for indexation mechanism".to_string(),
                    "Verify income calculation method".to_string(),
                    "Assess interaction with other income-tested benefits".to_string(),
                ],
            },

            AmendmentPattern::BenefitAmountChange => Self {
                pattern: pattern.clone(),
                description: "Modification of benefit/subsidy amounts".to_string(),
                common_reasons: vec![
                    "Inflation adjustment".to_string(),
                    "Budget reallocation".to_string(),
                    "Policy effectiveness review".to_string(),
                    "Addressing adequacy concerns".to_string(),
                ],
                review_checklist: vec![
                    "Verify fiscal impact".to_string(),
                    "Check effective date and transitional rules".to_string(),
                    "Review notification requirements to recipients".to_string(),
                    "Assess adequacy of new amount".to_string(),
                ],
            },

            AmendmentPattern::EligibilityExpansion => Self {
                pattern: pattern.clone(),
                description: "Broadening of eligibility criteria".to_string(),
                common_reasons: vec![
                    "Expanding social protection".to_string(),
                    "Court decision compliance".to_string(),
                    "Addressing coverage gaps".to_string(),
                    "Policy liberalization".to_string(),
                ],
                review_checklist: vec![
                    "Estimate newly eligible population".to_string(),
                    "Assess budgetary implications".to_string(),
                    "Check administrative capacity for increased caseload".to_string(),
                    "Review outreach and notification strategy".to_string(),
                ],
            },

            AmendmentPattern::EligibilityRestriction => Self {
                pattern: pattern.clone(),
                description: "Narrowing of eligibility criteria".to_string(),
                common_reasons: vec![
                    "Budget constraints".to_string(),
                    "Targeting improvement".to_string(),
                    "Preventing abuse".to_string(),
                    "Policy refinement".to_string(),
                ],
                review_checklist: vec![
                    "Identify individuals losing eligibility".to_string(),
                    "Check for grandfathering provisions".to_string(),
                    "Review procedural safeguards".to_string(),
                    "Assess potential legal challenges".to_string(),
                    "Ensure adequate notice period".to_string(),
                ],
            },

            AmendmentPattern::SunsetClauseModification => Self {
                pattern: pattern.clone(),
                description: "Addition, removal, or modification of expiration dates".to_string(),
                common_reasons: vec![
                    "Trial period extension".to_string(),
                    "Making temporary measures permanent".to_string(),
                    "Adding review requirements".to_string(),
                    "Phasing out outdated provisions".to_string(),
                ],
                review_checklist: vec![
                    "Verify evaluation mechanisms before expiry".to_string(),
                    "Check renewal/extension process".to_string(),
                    "Assess need for continued program".to_string(),
                    "Review data collection for evaluation".to_string(),
                ],
            },

            AmendmentPattern::DiscretionIntroduction => Self {
                pattern: pattern.clone(),
                description: "Introduction of administrative discretion".to_string(),
                common_reasons: vec![
                    "Accommodating exceptional circumstances".to_string(),
                    "Addressing rigid rules".to_string(),
                    "Enabling case-by-case assessment".to_string(),
                    "Responding to unanticipated situations".to_string(),
                ],
                review_checklist: vec![
                    "Review discretion guidelines and criteria".to_string(),
                    "Check for appeal mechanisms".to_string(),
                    "Assess potential for inconsistent application".to_string(),
                    "Verify training for decision-makers".to_string(),
                    "Ensure transparency and accountability measures".to_string(),
                ],
            },

            AmendmentPattern::DiscretionRemoval => Self {
                pattern: pattern.clone(),
                description: "Removal of discretionary elements (making deterministic)".to_string(),
                common_reasons: vec![
                    "Ensuring consistency".to_string(),
                    "Reducing processing time".to_string(),
                    "Preventing discrimination".to_string(),
                    "Automation enablement".to_string(),
                ],
                review_checklist: vec![
                    "Verify that rules cover all relevant scenarios".to_string(),
                    "Check for hardship exception mechanisms".to_string(),
                    "Assess impact on complex cases".to_string(),
                    "Review transition from discretionary decisions".to_string(),
                ],
            },

            AmendmentPattern::TechnicalCorrection => Self {
                pattern: pattern.clone(),
                description: "Minor corrections without substantive policy change".to_string(),
                common_reasons: vec![
                    "Correcting drafting errors".to_string(),
                    "Fixing cross-references".to_string(),
                    "Clarifying ambiguous language".to_string(),
                    "Updating terminology".to_string(),
                ],
                review_checklist: vec![
                    "Confirm no unintended substantive changes".to_string(),
                    "Verify consistency with intent".to_string(),
                    "Check legislative history if needed".to_string(),
                ],
            },

            AmendmentPattern::FundamentalRestructure => Self {
                pattern: pattern.clone(),
                description: "Major structural changes to the statute".to_string(),
                common_reasons: vec![
                    "Policy paradigm shift".to_string(),
                    "Comprehensive reform".to_string(),
                    "Responding to systemic issues".to_string(),
                    "Constitutional compliance".to_string(),
                ],
                review_checklist: vec![
                    "Conduct comprehensive impact assessment".to_string(),
                    "Review all transitional arrangements".to_string(),
                    "Assess implementation feasibility".to_string(),
                    "Verify stakeholder consultation".to_string(),
                    "Check for unintended consequences".to_string(),
                    "Ensure adequate implementation timeline".to_string(),
                ],
            },
        }
    }

    /// Generates a summary of a diff using template information.
    pub fn generate_summary(diff: &StatuteDiff) -> String {
        let patterns = Self::identify_patterns(diff);

        if patterns.is_empty() {
            return format!(
                "Amendment to '{}' with {} change(s). No common patterns identified.",
                diff.statute_id,
                diff.changes.len()
            );
        }

        let mut summary = format!(
            "Amendment to '{}' (Severity: {:?})\n\n",
            diff.statute_id, diff.impact.severity
        );

        summary.push_str("Identified Patterns:\n");
        for pattern in &patterns {
            let template = Self::for_pattern(pattern);
            summary.push_str(&format!(
                "• {}: {}\n",
                pattern_name(pattern),
                template.description
            ));
        }

        summary.push_str("\nKey Review Points:\n");
        for pattern in &patterns {
            let template = Self::for_pattern(pattern);
            for item in template.review_checklist.iter().take(3) {
                summary.push_str(&format!("• {}\n", item));
            }
        }

        summary
    }
}

fn pattern_name(pattern: &AmendmentPattern) -> &'static str {
    match pattern {
        AmendmentPattern::AgeThresholdAdjustment => "Age Threshold Adjustment",
        AmendmentPattern::IncomeLimitAdjustment => "Income Limit Adjustment",
        AmendmentPattern::BenefitAmountChange => "Benefit Amount Change",
        AmendmentPattern::EligibilityExpansion => "Eligibility Expansion",
        AmendmentPattern::EligibilityRestriction => "Eligibility Restriction",
        AmendmentPattern::SunsetClauseModification => "Sunset Clause Modification",
        AmendmentPattern::DiscretionIntroduction => "Discretion Introduction",
        AmendmentPattern::DiscretionRemoval => "Discretion Removal",
        AmendmentPattern::TechnicalCorrection => "Technical Correction",
        AmendmentPattern::FundamentalRestructure => "Fundamental Restructure",
    }
}

fn has_age_threshold_change(changes: &[Change]) -> bool {
    changes.iter().any(|c| {
        matches!(c.target, crate::ChangeTarget::Precondition { .. })
            && (c.old_value.as_ref().is_some_and(|v| v.contains("Age"))
                || c.new_value.as_ref().is_some_and(|v| v.contains("Age")))
    })
}

fn has_income_limit_change(changes: &[Change]) -> bool {
    changes.iter().any(|c| {
        matches!(c.target, crate::ChangeTarget::Precondition { .. })
            && (c.old_value.as_ref().is_some_and(|v| v.contains("Income"))
                || c.new_value.as_ref().is_some_and(|v| v.contains("Income")))
    })
}

fn has_benefit_amount_change(changes: &[Change]) -> bool {
    changes.iter().any(|c| {
        matches!(c.target, crate::ChangeTarget::Effect)
            && c.old_value.is_some()
            && c.new_value.is_some()
    })
}

fn is_technical_correction(diff: &StatuteDiff) -> bool {
    // Technical corrections are typically:
    // - Only title changes
    // - Low severity
    // - Don't affect eligibility or outcomes
    diff.changes.len() == 1
        && matches!(diff.changes[0].target, crate::ChangeTarget::Title)
        && matches!(
            diff.impact.severity,
            crate::Severity::Minor | crate::Severity::None
        )
        && !diff.impact.affects_eligibility
        && !diff.impact.affects_outcome
}

fn is_fundamental_restructure(diff: &StatuteDiff) -> bool {
    // Fundamental restructures have:
    // - Many changes
    // - Breaking severity
    // - Affects both eligibility and outcome
    diff.changes.len() >= 3
        && matches!(diff.impact.severity, crate::Severity::Breaking)
        && diff.impact.affects_eligibility
        && diff.impact.affects_outcome
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ChangeTarget, ImpactAssessment, Severity};

    #[test]
    fn test_identify_age_threshold_pattern() {
        let diff = StatuteDiff {
            statute_id: "test".to_string(),
            version_info: None,
            changes: vec![Change {
                change_type: ChangeType::Modified,
                target: ChangeTarget::Precondition { index: 0 },
                description: "Age requirement changed".to_string(),
                old_value: Some("Age { operator: GreaterOrEqual, value: 20 }".to_string()),
                new_value: Some("Age { operator: GreaterOrEqual, value: 18 }".to_string()),
            }],
            impact: ImpactAssessment {
                severity: Severity::Moderate,
                affects_eligibility: true,
                affects_outcome: false,
                discretion_changed: false,
                notes: vec![],
            },
        };

        let patterns = DiffTemplate::identify_patterns(&diff);
        assert!(patterns.contains(&AmendmentPattern::AgeThresholdAdjustment));
    }

    #[test]
    fn test_identify_income_limit_pattern() {
        let diff = StatuteDiff {
            statute_id: "test".to_string(),
            version_info: None,
            changes: vec![Change {
                change_type: ChangeType::Modified,
                target: ChangeTarget::Precondition { index: 0 },
                description: "Income limit changed".to_string(),
                old_value: Some("Income { operator: LessOrEqual, value: 3000000 }".to_string()),
                new_value: Some("Income { operator: LessOrEqual, value: 5000000 }".to_string()),
            }],
            impact: ImpactAssessment {
                severity: Severity::Major,
                affects_eligibility: true,
                affects_outcome: false,
                discretion_changed: false,
                notes: vec![],
            },
        };

        let patterns = DiffTemplate::identify_patterns(&diff);
        assert!(patterns.contains(&AmendmentPattern::IncomeLimitAdjustment));
    }

    #[test]
    fn test_identify_eligibility_expansion() {
        let diff = StatuteDiff {
            statute_id: "test".to_string(),
            version_info: None,
            changes: vec![Change {
                change_type: ChangeType::Removed,
                target: ChangeTarget::Precondition { index: 1 },
                description: "Precondition removed".to_string(),
                old_value: Some("Income requirement".to_string()),
                new_value: None,
            }],
            impact: ImpactAssessment {
                severity: Severity::Major,
                affects_eligibility: true,
                affects_outcome: false,
                discretion_changed: false,
                notes: vec![],
            },
        };

        let patterns = DiffTemplate::identify_patterns(&diff);
        assert!(patterns.contains(&AmendmentPattern::EligibilityExpansion));
    }

    #[test]
    fn test_identify_technical_correction() {
        let diff = StatuteDiff {
            statute_id: "test".to_string(),
            version_info: None,
            changes: vec![Change {
                change_type: ChangeType::Modified,
                target: ChangeTarget::Title,
                description: "Title typo fixed".to_string(),
                old_value: Some("Old Titla".to_string()),
                new_value: Some("Old Title".to_string()),
            }],
            impact: ImpactAssessment {
                severity: Severity::Minor,
                affects_eligibility: false,
                affects_outcome: false,
                discretion_changed: false,
                notes: vec![],
            },
        };

        let patterns = DiffTemplate::identify_patterns(&diff);
        assert!(patterns.contains(&AmendmentPattern::TechnicalCorrection));
    }

    #[test]
    fn test_generate_summary() {
        let diff = StatuteDiff {
            statute_id: "housing-subsidy-2024".to_string(),
            version_info: None,
            changes: vec![Change {
                change_type: ChangeType::Modified,
                target: ChangeTarget::Precondition { index: 0 },
                description: "Age requirement lowered".to_string(),
                old_value: Some("Age { operator: GreaterOrEqual, value: 20 }".to_string()),
                new_value: Some("Age { operator: GreaterOrEqual, value: 18 }".to_string()),
            }],
            impact: ImpactAssessment {
                severity: Severity::Moderate,
                affects_eligibility: true,
                affects_outcome: false,
                discretion_changed: false,
                notes: vec![],
            },
        };

        let summary = DiffTemplate::generate_summary(&diff);
        assert!(summary.contains("housing-subsidy-2024"));
        assert!(summary.contains("Age Threshold Adjustment"));
        assert!(summary.contains("Review Points"));
    }

    #[test]
    fn test_template_for_pattern() {
        let template = DiffTemplate::for_pattern(&AmendmentPattern::AgeThresholdAdjustment);
        assert!(!template.description.is_empty());
        assert!(!template.common_reasons.is_empty());
        assert!(!template.review_checklist.is_empty());
    }
}
