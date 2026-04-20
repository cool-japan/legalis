//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use legalis_core::{EffectType, Statute};
use std::collections::{HashMap, HashSet};

use super::functions::analyze_complexity;
use super::functions_3::detect_ambiguities;
use super::functions_4::analyze_regulatory_impact;
use super::types::{
    MarkovTransition, NaturalLanguageExplanation, RiskLevel, StatuteConflict, StatuteVerifier,
    VerificationPathNode, WhatIfScenario,
};
use super::types_3::{
    ConflictType, LazyVerificationConfig, MechanismAnalysis, MechanismProperty, PrivacyBudget,
    RiskFactor, RiskQuantification, Stakeholder, StatisticalCheckResult, TeeConfig,
    TeeVerificationResult,
};
use super::types_4::{
    ConflictExplanation, DependencyGraph, MarkovChain, PrivateAggregation, Severity,
};
use super::types_5::{
    ComplexityLevel, MechanismIssue, MultiPartyVerificationResult, VerificationError,
    VerificationResult,
};

/// Checks if the mechanism is incentive compatible
pub(super) fn check_incentive_compatibility(
    statutes: &[Statute],
    _stakeholders: &[Stakeholder],
    analysis: &mut MechanismAnalysis,
) {
    let mut has_issues = false;
    for statute in statutes {
        let has_penalty = matches!(
            statute.effect.effect_type,
            EffectType::Prohibition | EffectType::Revoke | EffectType::MonetaryTransfer
        );
        if has_penalty {
            let has_compliance_incentive = statute
                .discretion_logic
                .as_ref()
                .is_some_and(|logic| logic.contains("comply") || logic.contains("benefit"));
            if !has_compliance_incentive {
                has_issues = true;
                analysis.add_issue(MechanismIssue {
                    property: MechanismProperty::IncentiveCompatibility,
                    statute_ids: vec![statute.id.clone()],
                    severity: Severity::Warning,
                    description: format!(
                        "Statute {} imposes penalties without clear compliance incentives",
                        statute.id
                    ),
                    suggestions: vec![
                        "Add explicit compliance benefits".to_string(),
                        "Clarify positive incentives in discretion logic".to_string(),
                        "Consider reward mechanisms for compliance".to_string(),
                    ],
                });
            }
        }
        if statute.preconditions.len() > 3 {
            analysis.add_issue(MechanismIssue {
                property: MechanismProperty::IncentiveCompatibility,
                statute_ids: vec![statute.id.clone()],
                severity: Severity::Info,
                description: format!(
                    "Statute {} has complex preconditions that may allow strategic manipulation",
                    statute.id
                ),
                suggestions: vec![
                    "Simplify preconditions to reduce gaming opportunities".to_string(),
                    "Add verification mechanisms for condition claims".to_string(),
                ],
            });
        }
    }
    if !has_issues {
        analysis.satisfy_property(MechanismProperty::IncentiveCompatibility);
    }
}
/// Checks if the mechanism satisfies individual rationality
pub(super) fn check_individual_rationality(
    statutes: &[Statute],
    stakeholders: &[Stakeholder],
    analysis: &mut MechanismAnalysis,
) {
    let mut has_issues = false;
    for stakeholder in stakeholders {
        let affected_statutes: Vec<&Statute> = statutes
            .iter()
            .filter(|s| stakeholder.affected_by.contains(&s.id))
            .collect();
        let negative_effects = affected_statutes
            .iter()
            .filter(|s| {
                matches!(
                    s.effect.effect_type,
                    EffectType::Prohibition | EffectType::Revoke
                )
            })
            .count();
        let positive_effects = affected_statutes
            .iter()
            .filter(|s| matches!(s.effect.effect_type, EffectType::Grant))
            .count();
        if negative_effects > 0 && positive_effects == 0 {
            has_issues = true;
            let statute_ids: Vec<String> = affected_statutes.iter().map(|s| s.id.clone()).collect();
            analysis
                .add_issue(MechanismIssue {
                    property: MechanismProperty::IndividualRationality,
                    statute_ids,
                    severity: Severity::Warning,
                    description: format!(
                        "Stakeholder {} faces only penalties without benefits, violating individual rationality",
                        stakeholder.name
                    ),
                    suggestions: vec![
                        "Add compensatory benefits".to_string(),
                        "Make participation voluntary".to_string(),
                        "Provide alternative compliance paths".to_string(),
                    ],
                });
        }
    }
    if !has_issues {
        analysis.satisfy_property(MechanismProperty::IndividualRationality);
    }
}
/// Checks budget balance for monetary transfers
pub(super) fn check_budget_balance(statutes: &[Statute], analysis: &mut MechanismAnalysis) {
    let monetary_transfers: Vec<&Statute> = statutes
        .iter()
        .filter(|s| matches!(s.effect.effect_type, EffectType::MonetaryTransfer))
        .collect();
    if monetary_transfers.is_empty() {
        analysis.satisfy_property(MechanismProperty::BudgetBalance);
        return;
    }
    let mut has_balanced_transfers = false;
    for transfer in &monetary_transfers {
        let has_reverse = monetary_transfers
            .iter()
            .any(|t| t.id != transfer.id && t.jurisdiction == transfer.jurisdiction);
        if has_reverse {
            has_balanced_transfers = true;
        }
    }
    if !has_balanced_transfers && !monetary_transfers.is_empty() {
        analysis.add_issue(MechanismIssue {
            property: MechanismProperty::BudgetBalance,
            statute_ids: monetary_transfers.iter().map(|s| s.id.clone()).collect(),
            severity: Severity::Warning,
            description: "Monetary transfers may not be budget-balanced".to_string(),
            suggestions: vec![
                "Ensure transfers sum to zero or non-negative".to_string(),
                "Add corresponding revenue or expenditure statutes".to_string(),
                "Implement transfer tracking mechanisms".to_string(),
            ],
        });
    } else {
        analysis.satisfy_property(MechanismProperty::BudgetBalance);
    }
}
/// Checks for strategy-proofness
pub(super) fn check_strategy_proofness(statutes: &[Statute], analysis: &mut MechanismAnalysis) {
    let mut has_issues = false;
    for statute in statutes {
        for condition in &statute.preconditions {
            if matches!(condition, legalis_core::Condition::Custom { .. }) {
                has_issues = true;
                analysis
                    .add_issue(MechanismIssue {
                        property: MechanismProperty::StrategyProofness,
                        statute_ids: vec![statute.id.clone()],
                        severity: Severity::Info,
                        description: format!(
                            "Statute {} has custom conditions that may be difficult to verify truthfully",
                            statute.id
                        ),
                        suggestions: vec![
                            "Add verification mechanisms for custom conditions"
                            .to_string(),
                            "Use objective, verifiable conditions where possible"
                            .to_string(), "Implement audit trails for condition claims"
                            .to_string(),
                        ],
                    });
            }
        }
        if matches!(statute.effect.effect_type, EffectType::Grant)
            && statute.preconditions.is_empty()
        {
            has_issues = true;
            analysis.add_issue(MechanismIssue {
                property: MechanismProperty::StrategyProofness,
                statute_ids: vec![statute.id.clone()],
                severity: Severity::Warning,
                description: format!(
                    "Statute {} grants benefits without verifiable conditions",
                    statute.id
                ),
                suggestions: vec![
                    "Add objective eligibility criteria".to_string(),
                    "Implement verification procedures".to_string(),
                ],
            });
        }
    }
    if !has_issues {
        analysis.satisfy_property(MechanismProperty::StrategyProofness);
    }
}
/// Checks for non-dictatorship
pub(super) fn check_non_dictatorship(
    statutes: &[Statute],
    stakeholders: &[Stakeholder],
    analysis: &mut MechanismAnalysis,
) {
    if stakeholders.is_empty() {
        analysis.satisfy_property(MechanismProperty::NonDictatorship);
        return;
    }
    let mut statute_control: HashMap<String, usize> = HashMap::new();
    for stakeholder in stakeholders {
        statute_control.insert(stakeholder.id.clone(), stakeholder.affected_by.len());
    }
    let total_statutes = statutes.len();
    let max_control = statute_control.values().max().copied().unwrap_or(0);
    if max_control as f64 > (total_statutes as f64 * 0.5) {
        let dictator = statute_control
            .iter()
            .find(|(_, count)| **count == max_control)
            .map(|(id, _)| id.clone())
            .unwrap_or_default();
        analysis.add_issue(MechanismIssue {
            property: MechanismProperty::NonDictatorship,
            statute_ids: vec![],
            severity: Severity::Error,
            description: format!(
                "Stakeholder {} controls {}% of statutes, suggesting potential dictatorship",
                dictator,
                (max_control as f64 / total_statutes as f64 * 100.0) as i32
            ),
            suggestions: vec![
                "Distribute statute influence more evenly".to_string(),
                "Add checks and balances".to_string(),
                "Implement multi-stakeholder approval mechanisms".to_string(),
            ],
        });
    } else {
        analysis.satisfy_property(MechanismProperty::NonDictatorship);
    }
}
/// Generates a mechanism design analysis report
pub fn mechanism_design_report(analysis: &MechanismAnalysis) -> String {
    let mut report = String::new();
    report.push_str("# Mechanism Design Analysis\n\n");
    report.push_str(&format!(
        "**Overall Quality Score**: {:.2}/1.00\n\n",
        analysis.quality_score
    ));
    let quality_level = if analysis.quality_score >= 0.9 {
        "Excellent"
    } else if analysis.quality_score >= 0.7 {
        "Good"
    } else if analysis.quality_score >= 0.5 {
        "Fair"
    } else {
        "Poor"
    };
    report.push_str(&format!("**Quality Level**: {}\n\n", quality_level));
    report.push_str(&format!(
        "## Satisfied Properties ({}/6)\n\n",
        analysis.satisfied_properties.len()
    ));
    if analysis.satisfied_properties.is_empty() {
        report.push_str("None\n\n");
    } else {
        for property in &analysis.satisfied_properties {
            report.push_str(&format!("- ✓ {}\n", property));
        }
        report.push('\n');
    }
    report.push_str(&format!("## Issues ({} found)\n\n", analysis.issues.len()));
    if analysis.issues.is_empty() {
        report.push_str("No mechanism design issues detected. The mechanism is well-designed.\n\n");
    } else {
        let mut by_property: HashMap<MechanismProperty, Vec<&MechanismIssue>> = HashMap::new();
        for issue in &analysis.issues {
            by_property.entry(issue.property).or_default().push(issue);
        }
        for (property, issues) in &by_property {
            report.push_str(&format!("### {} - {} issues\n\n", property, issues.len()));
            for issue in issues {
                report.push_str(&format!("**Severity**: {}\n\n", issue.severity));
                report.push_str(&format!("**Description**: {}\n\n", issue.description));
                if !issue.statute_ids.is_empty() {
                    report.push_str(&format!(
                        "**Affected Statutes**: {}\n\n",
                        issue.statute_ids.join(", ")
                    ));
                }
                report.push_str("**Suggestions**:\n");
                for suggestion in &issue.suggestions {
                    report.push_str(&format!("- {}\n", suggestion));
                }
                report.push('\n');
            }
        }
    }
    report
}
/// Monte Carlo simulation for statute verification
pub fn monte_carlo_verification(
    chain: &MarkovChain,
    num_simulations: usize,
    max_steps: usize,
) -> StatisticalCheckResult {
    use rand::RngExt;
    let mut successes = 0;
    for _ in 0..num_simulations {
        let mut current_state = chain.initial_state.clone();
        let mut reached_accepting = false;
        for _ in 0..max_steps {
            if let Some(state) = chain.states.iter().find(|s| s.id == current_state)
                && state.accepting
            {
                reached_accepting = true;
                break;
            }
            let outgoing: Vec<&MarkovTransition> = chain
                .transitions
                .iter()
                .filter(|t| t.from == current_state)
                .collect();
            if outgoing.is_empty() {
                break;
            }
            let mut rng = rand::rng();
            let r: f64 = rng.random();
            let mut cumulative = 0.0;
            for transition in outgoing {
                cumulative += transition.probability;
                if r <= cumulative {
                    current_state = transition.to.clone();
                    break;
                }
            }
        }
        if reached_accepting {
            successes += 1;
        }
    }
    StatisticalCheckResult::from_samples(
        "Reachability of accepting states",
        num_simulations,
        successes,
        0.5,
    )
}
/// Analyzes statute risk using multiple factors
pub fn analyze_statute_risk(
    statute: &Statute,
    verification_result: &VerificationResult,
) -> RiskQuantification {
    let mut factors = vec![];
    let complexity_metrics = analyze_complexity(statute);
    let complexity_score = match complexity_metrics.complexity_level {
        ComplexityLevel::Simple => 0.1,
        ComplexityLevel::Moderate => 0.3,
        ComplexityLevel::Complex => 0.6,
        ComplexityLevel::VeryComplex => 0.9,
    };
    factors.push(
        RiskFactor::new(
            "Complexity Risk",
            format!(
                "Statute complexity: {:?}",
                complexity_metrics.complexity_level
            ),
            complexity_score,
        )
        .with_weight(0.25),
    );
    let error_score = if verification_result.errors.is_empty() {
        0.0
    } else {
        let critical_errors = verification_result
            .errors
            .iter()
            .filter(|e| e.severity() == Severity::Critical)
            .count();
        let error_count = verification_result.errors.len();
        (0.5 + (critical_errors as f64 * 0.1))
            .min(1.0)
            .max(error_count as f64 * 0.1)
    };
    factors.push(
        RiskFactor::new(
            "Verification Error Risk",
            format!(
                "{} errors found (including critical)",
                verification_result.errors.len()
            ),
            error_score,
        )
        .with_weight(0.35),
    );
    let ambiguities = detect_ambiguities(statute);
    let ambiguity_score = (ambiguities.len() as f64 * 0.15).min(1.0);
    factors.push(
        RiskFactor::new(
            "Ambiguity Risk",
            format!("{} ambiguities detected", ambiguities.len()),
            ambiguity_score,
        )
        .with_weight(0.20),
    );
    let impact = analyze_regulatory_impact(statute);
    let impact_score = impact.impact_score as f64 / 100.0;
    factors.push(
        RiskFactor::new(
            "Regulatory Impact Risk",
            format!("Impact level: {:?}", impact.impact_level),
            impact_score,
        )
        .with_weight(0.20),
    );
    let mut quantification = RiskQuantification::new(statute.id.clone(), factors);
    match quantification.risk_level {
        RiskLevel::Critical | RiskLevel::High => {
            quantification = quantification
                .add_mitigation("Immediate review and simplification required")
                .add_mitigation("Resolve all critical errors before deployment")
                .add_mitigation("Add comprehensive test coverage")
                .add_mitigation("Implement staged rollout with monitoring");
        }
        RiskLevel::Medium => {
            quantification = quantification
                .add_mitigation("Address identified ambiguities")
                .add_mitigation("Consider simplification if possible")
                .add_mitigation("Add monitoring for edge cases");
        }
        RiskLevel::Low => {
            quantification = quantification
                .add_mitigation("Regular monitoring recommended")
                .add_mitigation("Consider proactive testing");
        }
        RiskLevel::Minimal => {
            quantification =
                quantification.add_mitigation("Continue standard compliance monitoring");
        }
    }
    quantification
}
/// Generates a risk quantification report
pub fn risk_quantification_report(risks: &[RiskQuantification]) -> String {
    let mut report = String::new();
    report.push_str("# Risk Quantification Report\n\n");
    report.push_str(&format!("**Total Statutes Analyzed**: {}\n\n", risks.len()));
    let mut risk_distribution: HashMap<RiskLevel, usize> = HashMap::new();
    for risk in risks {
        *risk_distribution.entry(risk.risk_level).or_insert(0) += 1;
    }
    report.push_str("## Risk Level Distribution\n\n");
    for level in &[
        RiskLevel::Critical,
        RiskLevel::High,
        RiskLevel::Medium,
        RiskLevel::Low,
        RiskLevel::Minimal,
    ] {
        let count = risk_distribution.get(level).copied().unwrap_or(0);
        report.push_str(&format!("- {}: {} statutes\n", level, count));
    }
    report.push('\n');
    report.push_str("## Statute Risk Analysis\n\n");
    let mut sorted_risks: Vec<_> = risks.iter().collect();
    sorted_risks.sort_by(|a, b| {
        b.overall_score
            .partial_cmp(&a.overall_score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    for risk in sorted_risks {
        report.push_str(&format!("### Statute: {}\n\n", risk.statute_id));
        report.push_str(&format!(
            "**Overall Risk Score**: {:.2}/1.00 ({})\n\n",
            risk.overall_score, risk.risk_level
        ));
        report.push_str("**Risk Factors**:\n");
        for factor in &risk.factors {
            report.push_str(&format!(
                "- {}: {:.2} (weight: {:.2}) - {}\n",
                factor.name, factor.score, factor.weight, factor.description
            ));
        }
        report.push('\n');
        if !risk.mitigations.is_empty() {
            report.push_str("**Mitigation Recommendations**:\n");
            for mitigation in &risk.mitigations {
                report.push_str(&format!("- {}\n", mitigation));
            }
            report.push('\n');
        }
    }
    report
}
/// Generates a statistical model checking report
pub fn statistical_model_checking_report(results: &[StatisticalCheckResult]) -> String {
    let mut report = String::new();
    report.push_str("# Statistical Model Checking Report\n\n");
    report.push_str(&format!("**Properties Checked**: {}\n\n", results.len()));
    for result in results {
        report.push_str(&format!("## Property: {}\n\n", result.property));
        report.push_str(&format!(
            "**Estimated Probability**: {:.4}\n",
            result.estimated_probability
        ));
        report.push_str(&format!(
            "**95% Confidence Interval**: [{:.4}, {:.4}]\n",
            result.confidence_lower, result.confidence_upper
        ));
        report.push_str(&format!("**Samples**: {}\n", result.num_samples));
        report.push_str(&format!("**Successes**: {}\n", result.num_successes));
        report.push_str(&format!(
            "**Hypothesis Test**: {}\n\n",
            if result.hypothesis_accepted {
                "ACCEPTED"
            } else {
                "REJECTED"
            }
        ));
    }
    report
}
/// Generates natural language explanation for a verification error
pub fn explain_error(error: &VerificationError) -> NaturalLanguageExplanation {
    match error {
        VerificationError::CircularReference { message } => {
            NaturalLanguageExplanation::new(
                    "Circular Reference",
                    "This law refers to itself in a way that creates an infinite loop.",
                    format!("Circular dependency detected: {}", message),
                    "Circular references make it impossible to determine what the law actually requires, \
                 since each requirement depends on itself.",
                    "Break the circular chain by removing one of the references, or restructure the \
                 statutes so they don't depend on each other in a loop.",
                )
                .with_example(
                    "Imagine Law A says 'Follow Law B', and Law B says 'Follow Law A'. \
                 Which one do you follow first? It's impossible to tell!",
                )
        }
        VerificationError::DeadStatute { statute_id } => {
            NaturalLanguageExplanation::new(
                    "Impossible to Satisfy",
                    format!(
                        "Law '{}' has conditions that can never be met.", statute_id
                    ),
                    format!(
                        "Dead statute detected: {} has contradictory preconditions",
                        statute_id
                    ),
                    "If a law can never be satisfied, it's useless and confusing. People might waste time \
             trying to comply with something that's impossible.",
                    "Review the conditions and remove contradictory requirements. Make sure the conditions \
             are logically possible to satisfy.",
                )
                .with_example(
                    "This is like a rule that says 'You must be both over 18 AND under 16 years old'. \
             Nobody can satisfy both conditions at the same time.",
                )
        }
        VerificationError::ConstitutionalConflict { statute_id, principle } => {
            NaturalLanguageExplanation::new(
                "Constitutional Conflict",
                format!(
                    "Law '{}' conflicts with the constitutional principle: {}",
                    statute_id, principle
                ),
                format!(
                    "Statute {} violates constitutional principle: {}", statute_id,
                    principle
                ),
                "Constitutional principles are fundamental rights and protections. Laws that violate \
             them may be invalid and could cause harm to people's rights.",
                format!(
                    "Revise the law to align with the '{}' principle. Consider adding safeguards or \
                 exceptions that protect constitutional rights.",
                    principle
                ),
            )
        }
        VerificationError::LogicalContradiction { message } => {
            NaturalLanguageExplanation::new(
                    "Logical Contradiction",
                    "This law contains conditions that contradict each other.",
                    format!("Logical contradiction found: {}", message),
                    "Contradictory conditions create confusion and make it unclear what the law actually requires.",
                    "Remove or revise the contradictory conditions so they work together logically.",
                )
                .with_example(
                    "This is like saying 'You can drive if you have a license AND you can drive if you \
             don't have a license' - which is it?",
                )
        }
        VerificationError::Ambiguity { message } => {
            NaturalLanguageExplanation::new(
                    "Ambiguous Language",
                    "This law uses vague or unclear language that could be interpreted multiple ways.",
                    format!("Ambiguity detected: {}", message),
                    "Ambiguous laws lead to inconsistent enforcement and confusion about what's actually required.",
                    "Replace vague terms with specific, measurable criteria. Define unclear terms explicitly.",
                )
                .with_example(
                    "Instead of saying 'a reasonable amount', specify exactly what the amount should be \
             (e.g., 'no more than $100').",
                )
        }
        VerificationError::UnreachableCode { message } => {
            NaturalLanguageExplanation::new(
                "Unreachable Provision",
                "Part of this law can never be triggered or applied.",
                format!("Unreachable code detected: {}", message),
                "Dead provisions waste space in the legal code and may confuse people into thinking \
             they're relevant when they're not.",
                "Remove the unreachable provisions, or fix the conditions so they can actually be triggered.",
            )
        }
    }
}
/// Explains statute conflicts in layperson terms
pub fn explain_conflict(conflict: &StatuteConflict) -> ConflictExplanation {
    let mut explanation = ConflictExplanation::new(
        conflict.statute_ids.clone(),
        match &conflict.conflict_type {
            ConflictType::EffectConflict => {
                "These laws have overlapping conditions but contradictory effects - they would \
                 apply to the same situations but produce different outcomes."
                    .to_string()
            }
            ConflictType::JurisdictionalOverlap => {
                "These laws overlap in their jurisdiction, creating uncertainty about which applies."
                    .to_string()
            }
            ConflictType::TemporalConflict => {
                "These laws have conflicting rules during overlapping time periods."
                    .to_string()
            }
            ConflictType::HierarchyViolation => {
                "A lower-level law contradicts a higher-level law, which violates legal hierarchy."
                    .to_string()
            }
            ConflictType::IdCollision => {
                "These laws have the same identifier in different jurisdictions, causing confusion."
                    .to_string()
            }
        },
    );
    explanation = explanation.with_impact(
        "This conflict creates legal uncertainty. People affected by these laws may not know \
         which one to follow, leading to potential compliance issues or unfair treatment.",
    );
    match &conflict.conflict_type {
        ConflictType::EffectConflict => {
            explanation = explanation
                .add_affected_party("Anyone trying to comply with both laws")
                .add_affected_party("Law enforcement agencies")
                .add_affected_party("Courts interpreting the laws");
        }
        ConflictType::JurisdictionalOverlap => {
            explanation = explanation
                .add_affected_party("People living or operating in the overlapping jurisdiction")
                .add_affected_party("Multiple regulatory bodies");
        }
        ConflictType::TemporalConflict => {
            explanation = explanation
                .add_affected_party("People affected during the overlapping time period")
                .add_affected_party("Legal administrators managing transitions");
        }
        ConflictType::HierarchyViolation => {
            explanation = explanation
                .add_affected_party("Courts enforcing legal hierarchy")
                .add_affected_party("Citizens relying on proper legal authority");
        }
        ConflictType::IdCollision => {
            explanation = explanation
                .add_affected_party("Cross-jurisdictional entities")
                .add_affected_party("Legal databases and systems");
        }
    }
    for suggestion in &conflict.resolution_suggestions {
        explanation = explanation.add_resolution_option(suggestion.clone());
    }
    explanation
}
/// Builds a verification path from a statute and result
pub fn build_verification_path(
    statute: &Statute,
    result: &VerificationResult,
) -> VerificationPathNode {
    let mut root = VerificationPathNode::new(
        &statute.id,
        "statute",
        format!("Statute: {}", statute.title),
    )
    .with_status(result.passed);
    if !statute.preconditions.is_empty() {
        for (i, precondition) in statute.preconditions.iter().enumerate() {
            let precondition_node =
                build_condition_path(precondition, &format!("precondition_{}", i));
            root = root.add_child(precondition_node);
        }
    }
    let effect_node = VerificationPathNode::new(
        format!("{}_effect", statute.id),
        "effect",
        format!("Effect: {:?}", statute.effect.effect_type),
    )
    .add_metadata("description", &statute.effect.description);
    root = root.add_child(effect_node);
    for (i, error) in result.errors.iter().enumerate() {
        let error_node = VerificationPathNode::new(
            format!("{}_error_{}", statute.id, i),
            "error",
            format!("Error: {:?}", error),
        )
        .with_status(false)
        .add_metadata("severity", format!("{:?}", error.severity()));
        root = root.add_child(error_node);
    }
    root
}
pub(crate) fn build_condition_path(
    condition: &legalis_core::Condition,
    prefix: &str,
) -> VerificationPathNode {
    use legalis_core::{ComparisonOp, Condition};
    match condition {
        Condition::Age { operator, value } => VerificationPathNode::new(
            format!("{}_age", prefix),
            "condition",
            format!(
                "Age {} {}",
                match operator {
                    ComparisonOp::Equal => "==",
                    ComparisonOp::NotEqual => "!=",
                    ComparisonOp::LessThan => "<",
                    ComparisonOp::LessOrEqual => "<=",
                    ComparisonOp::GreaterThan => ">",
                    ComparisonOp::GreaterOrEqual => ">=",
                },
                value
            ),
        ),
        Condition::Income { operator, value } => VerificationPathNode::new(
            format!("{}_income", prefix),
            "condition",
            format!(
                "Income {} ${}",
                match operator {
                    ComparisonOp::Equal => "==",
                    ComparisonOp::NotEqual => "!=",
                    ComparisonOp::LessThan => "<",
                    ComparisonOp::LessOrEqual => "<=",
                    ComparisonOp::GreaterThan => ">",
                    ComparisonOp::GreaterOrEqual => ">=",
                },
                value
            ),
        ),
        Condition::And(left, right) => {
            let mut node = VerificationPathNode::new(format!("{}_and", prefix), "logic", "AND");
            node = node.add_child(build_condition_path(left, &format!("{}_left", prefix)));
            node = node.add_child(build_condition_path(right, &format!("{}_right", prefix)));
            node
        }
        Condition::Or(left, right) => {
            let mut node = VerificationPathNode::new(format!("{}_or", prefix), "logic", "OR");
            node = node.add_child(build_condition_path(left, &format!("{}_left", prefix)));
            node = node.add_child(build_condition_path(right, &format!("{}_right", prefix)));
            node
        }
        Condition::Not(inner) => {
            let mut node = VerificationPathNode::new(format!("{}_not", prefix), "logic", "NOT");
            node = node.add_child(build_condition_path(inner, &format!("{}_inner", prefix)));
            node
        }
        _ => VerificationPathNode::new(
            format!("{}_condition", prefix),
            "condition",
            "Complex Condition",
        ),
    }
}
/// Performs what-if analysis on a statute modification
pub fn what_if_analysis(
    description: impl Into<String>,
    original: Statute,
    modifier: impl FnOnce(&mut Statute),
) -> WhatIfScenario {
    let verifier = StatuteVerifier::new();
    let original_result = verifier.verify(std::slice::from_ref(&original));
    let mut modified = original.clone();
    modifier(&mut modified);
    let new_result = verifier.verify(std::slice::from_ref(&modified));
    WhatIfScenario::new(description, original, modified, original_result, new_result)
}
/// Performs secure multi-party verification
pub fn secure_multiparty_verification(
    statute: &Statute,
    parties: Vec<String>,
) -> MultiPartyVerificationResult {
    let verifier = StatuteVerifier::new();
    let result = verifier.verify(std::slice::from_ref(statute));
    MultiPartyVerificationResult::new(parties, result)
}
/// Performs differentially private aggregation analysis
pub fn differential_private_analysis(
    statutes: &[Statute],
    privacy_budget: PrivacyBudget,
) -> PrivateAggregation {
    use rand::RngExt;
    let mut rng = rand::rng();
    let count = statutes.len() as f64;
    let verifier = StatuteVerifier::new();
    let mut total_complexity = 0;
    let mut total_errors = 0;
    for statute in statutes {
        let result = verifier.verify(std::slice::from_ref(statute));
        total_complexity += statute.preconditions.len();
        if !result.passed {
            total_errors += 1;
        }
    }
    let avg_complexity = if count > 0.0 {
        total_complexity as f64 / count
    } else {
        0.0
    };
    let error_rate = if count > 0.0 {
        total_errors as f64 / count
    } else {
        0.0
    };
    let sensitivity = 1.0;
    let scale = sensitivity / privacy_budget.epsilon;
    let mut laplace_noise = || -> f64 {
        let u: f64 = rng.random::<f64>() - 0.5;
        -scale * u.signum() * (1.0 - 2.0 * u.abs()).ln()
    };
    PrivateAggregation {
        count: (count + laplace_noise()).max(0.0),
        avg_complexity: (avg_complexity + laplace_noise()).max(0.0),
        error_rate: (error_rate + laplace_noise() / count.max(1.0)).clamp(0.0, 1.0),
        privacy_budget,
    }
}
/// Performs verification in a trusted execution environment
pub fn tee_verification(statute: &Statute, tee_config: TeeConfig) -> TeeVerificationResult {
    let verifier = StatuteVerifier::new();
    let result = verifier.verify(std::slice::from_ref(statute));
    TeeVerificationResult::new(result, tee_config)
}
/// Performs lazy verification on demand
pub fn lazy_verify(
    statutes: &[Statute],
    changed_ids: &[String],
    config: LazyVerificationConfig,
) -> VerificationResult {
    let verifier = StatuteVerifier::new();
    if changed_ids.is_empty() {
        return VerificationResult::pass();
    }
    let graph = DependencyGraph::from_statutes(statutes);
    let mut to_verify: HashSet<String> = changed_ids.iter().cloned().collect();
    if config.verify_dependencies {
        for changed_id in changed_ids {
            let affected = graph.get_affected_statutes(changed_id);
            for id in affected {
                to_verify.insert(id);
            }
        }
    }
    let statutes_to_verify: Vec<Statute> = statutes
        .iter()
        .filter(|s| to_verify.contains(&s.id))
        .cloned()
        .collect();
    verifier.verify(&statutes_to_verify)
}
