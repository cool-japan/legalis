//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use legalis_core::{EffectType, Statute};
use std::collections::{HashMap, HashSet};

use super::functions::detect_statute_conflicts;
use super::functions::{analyze_complexity, analyze_coverage};
use super::functions::{
    conditions_overlap, effects_contradict, temporal_validity_overlaps, title_similarity,
};
use super::functions_2::extract_statute_references_from_conditions;
use super::functions_2::semantic_similarity;
use super::functions_3::analyze_quality;
use super::functions_4::{analyze_centrality, analyze_graph_metrics, analyze_statute_statistics};
use super::functions_6::{
    check_budget_balance, check_incentive_compatibility, check_individual_rationality,
    check_non_dictatorship, check_strategy_proofness,
};
use super::types::{
    ConflictCascade, OverlapArea, ProofStep, StatuteConflict, Strategy, VerificationProof,
};
use super::types_3::{
    Coalition, ConflictNature, GapType, MechanismAnalysis, MetricsDashboard, ProofStepType,
    RedundancyInstance, Stakeholder, StatuteInteraction,
};
use super::types_4::{
    EnhancedCoverageGap, EvolutionTracker, GameTheoreticModel, InteractionType, QualitySummary,
    RedundancyType, Severity, StatutePattern,
};
use super::types_5::{
    ConflictSummary, EvolutionSummary, GameOutcome, PatternType, RegulatoryOverlap,
    StakeholderConflict,
};

/// Mines common patterns from a collection of statutes
pub fn mine_patterns(statutes: &[Statute]) -> Vec<StatutePattern> {
    let mut patterns = Vec::new();
    let age_statutes: Vec<_> = statutes
        .iter()
        .filter(|s| has_age_condition(&s.preconditions))
        .map(|s| s.id.clone())
        .collect();
    if !age_statutes.is_empty() {
        patterns.push(StatutePattern {
            id: "age-eligibility".to_string(),
            description: "Statutes with age-based eligibility requirements".to_string(),
            frequency: age_statutes.len(),
            examples: age_statutes.iter().take(5).cloned().collect(),
            pattern_type: PatternType::AgeEligibility,
        });
    }
    let income_statutes: Vec<_> = statutes
        .iter()
        .filter(|s| has_income_condition(&s.preconditions))
        .map(|s| s.id.clone())
        .collect();
    if !income_statutes.is_empty() {
        patterns.push(StatutePattern {
            id: "income-qualification".to_string(),
            description: "Statutes with income-based qualification criteria".to_string(),
            frequency: income_statutes.len(),
            examples: income_statutes.iter().take(5).cloned().collect(),
            pattern_type: PatternType::IncomeQualification,
        });
    }
    let combined_statutes: Vec<_> = statutes
        .iter()
        .filter(|s| has_age_condition(&s.preconditions) && has_income_condition(&s.preconditions))
        .map(|s| s.id.clone())
        .collect();
    if !combined_statutes.is_empty() {
        patterns.push(StatutePattern {
            id: "age-and-income".to_string(),
            description: "Statutes combining age and income requirements".to_string(),
            frequency: combined_statutes.len(),
            examples: combined_statutes.iter().take(5).cloned().collect(),
            pattern_type: PatternType::AgeAndIncome,
        });
    }
    let prohibition_statutes: Vec<_> = statutes
        .iter()
        .filter(|s| {
            matches!(s.effect.effect_type, EffectType::Prohibition)
                && has_negation(&s.preconditions)
        })
        .map(|s| s.id.clone())
        .collect();
    if !prohibition_statutes.is_empty() {
        patterns.push(StatutePattern {
            id: "prohibition-with-exceptions".to_string(),
            description: "Prohibitions with exception conditions (NOT clauses)".to_string(),
            frequency: prohibition_statutes.len(),
            examples: prohibition_statutes.iter().take(5).cloned().collect(),
            pattern_type: PatternType::ProhibitionWithExceptions,
        });
    }
    let temporal_statutes: Vec<_> = statutes
        .iter()
        .filter(|s| {
            s.temporal_validity.has_effective_date() || s.temporal_validity.has_expiry_date()
        })
        .map(|s| s.id.clone())
        .collect();
    if !temporal_statutes.is_empty() {
        patterns.push(StatutePattern {
            id: "temporal-restriction".to_string(),
            description: "Statutes with temporal validity constraints".to_string(),
            frequency: temporal_statutes.len(),
            examples: temporal_statutes.iter().take(5).cloned().collect(),
            pattern_type: PatternType::TemporalRestriction,
        });
    }
    let mut jurisdiction_map: HashMap<String, Vec<String>> = HashMap::new();
    for statute in statutes {
        if let Some(jurisdiction) = &statute.jurisdiction {
            jurisdiction_map
                .entry(jurisdiction.clone())
                .or_default()
                .push(statute.id.clone());
        }
    }
    for (jurisdiction, statute_ids) in jurisdiction_map {
        if statute_ids.len() >= 3 {
            patterns.push(StatutePattern {
                id: format!("jurisdiction-{}", jurisdiction.to_lowercase()),
                description: format!("Statutes specific to {} jurisdiction", jurisdiction),
                frequency: statute_ids.len(),
                examples: statute_ids.iter().take(5).cloned().collect(),
                pattern_type: PatternType::JurisdictionalPattern,
            });
        }
    }
    patterns.sort_by_key(|b| std::cmp::Reverse(b.frequency));
    patterns
}
/// Helper: checks if conditions contain age requirement
fn has_age_condition(conditions: &[legalis_core::Condition]) -> bool {
    conditions
        .iter()
        .any(|c| matches!(c, legalis_core::Condition::Age { .. }))
        || conditions.iter().any(|c| {
            check_condition_recursive(c, |cond| {
                matches!(cond, legalis_core::Condition::Age { .. })
            })
        })
}
/// Helper: checks if conditions contain income requirement
fn has_income_condition(conditions: &[legalis_core::Condition]) -> bool {
    conditions
        .iter()
        .any(|c| matches!(c, legalis_core::Condition::Income { .. }))
        || conditions.iter().any(|c| {
            check_condition_recursive(c, |cond| {
                matches!(cond, legalis_core::Condition::Income { .. })
            })
        })
}
/// Helper: checks if conditions contain negation
fn has_negation(conditions: &[legalis_core::Condition]) -> bool {
    conditions
        .iter()
        .any(|c| matches!(c, legalis_core::Condition::Not(_)))
        || conditions.iter().any(|c| {
            check_condition_recursive(c, |cond| matches!(cond, legalis_core::Condition::Not(_)))
        })
}
/// Helper: recursively checks a condition with a predicate
fn check_condition_recursive<F>(condition: &legalis_core::Condition, predicate: F) -> bool
where
    F: Fn(&legalis_core::Condition) -> bool + Copy,
{
    use legalis_core::Condition;
    if predicate(condition) {
        return true;
    }
    match condition {
        Condition::And(left, right) | Condition::Or(left, right) => {
            check_condition_recursive(left, predicate)
                || check_condition_recursive(right, predicate)
        }
        Condition::Not(inner) => check_condition_recursive(inner, predicate),
        _ => false,
    }
}
/// Generates a pattern mining report
pub fn pattern_mining_report(statutes: &[Statute]) -> String {
    let mut report = String::new();
    report.push_str("# Statute Pattern Mining Report\n\n");
    let patterns = mine_patterns(statutes);
    report.push_str(&format!(
        "**Total Statutes Analyzed**: {}\n",
        statutes.len()
    ));
    report.push_str(&format!("**Patterns Found**: {}\n\n", patterns.len()));
    report.push_str("## Discovered Patterns\n\n");
    for (i, pattern) in patterns.iter().enumerate() {
        report.push_str(&format!(
            "### {}. {} ({})\n\n",
            i + 1,
            pattern.description,
            pattern.pattern_type
        ));
        report.push_str(&format!(
            "- **Frequency**: {} statutes ({:.1}%)\n",
            pattern.frequency,
            (pattern.frequency as f64 / statutes.len() as f64) * 100.0
        ));
        report.push_str("- **Examples**: ");
        report.push_str(&pattern.examples.join(", "));
        report.push_str("\n\n");
    }
    report
}
/// Generates a comprehensive metrics dashboard
pub fn generate_metrics_dashboard(
    statutes: &[Statute],
    evolution_tracker: Option<&EvolutionTracker>,
) -> MetricsDashboard {
    let statistics = analyze_statute_statistics(statutes);
    let graph_metrics = analyze_graph_metrics(statutes);
    let mut centrality = analyze_centrality(statutes);
    centrality.sort_by(|a, b| {
        b.pagerank
            .partial_cmp(&a.pagerank)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let top_centrality: Vec<_> = centrality.into_iter().take(10).collect();
    let quality_metrics: Vec<_> = statutes.iter().map(analyze_quality).collect();
    let average_score = if !quality_metrics.is_empty() {
        quality_metrics.iter().map(|q| q.overall_score).sum::<f64>() / quality_metrics.len() as f64
    } else {
        0.0
    };
    let mut grade_distribution = HashMap::new();
    for qm in &quality_metrics {
        *grade_distribution
            .entry(qm.grade().to_string())
            .or_insert(0) += 1;
    }
    let statutes_with_issues = quality_metrics
        .iter()
        .filter(|q| !q.issues.is_empty())
        .count();
    let total_issues: usize = quality_metrics.iter().map(|q| q.issues.len()).sum();
    let quality_summary = QualitySummary {
        average_score,
        grade_distribution,
        statutes_with_issues,
        total_issues,
    };
    let conflicts = detect_statute_conflicts(statutes);
    let mut conflicts_by_type = HashMap::new();
    for conflict in &conflicts {
        let type_name = format!("{:?}", conflict.conflict_type);
        *conflicts_by_type.entry(type_name).or_insert(0) += 1;
    }
    let critical_conflicts = conflicts
        .iter()
        .filter(|c| matches!(c.severity, Severity::Critical))
        .count();
    let conflict_summary = ConflictSummary {
        total_conflicts: conflicts.len(),
        conflicts_by_type,
        critical_conflicts,
    };
    let coverage_info = analyze_coverage(statutes);
    let evolution_summary = evolution_tracker.map(|tracker| {
        let all_metrics = tracker.analyze_all_metrics();
        let total_tracked = all_metrics.len();
        let total_versions: usize = all_metrics.iter().map(|m| m.total_versions).sum();
        let avg_versions = if total_tracked > 0 {
            total_versions as f64 / total_tracked as f64
        } else {
            0.0
        };
        let most_changed = tracker
            .most_changed_statutes(1)
            .first()
            .map(|m| m.statute_id.clone());
        let most_stable = tracker
            .most_stable_statutes(1)
            .first()
            .map(|m| m.statute_id.clone());
        EvolutionSummary {
            total_tracked,
            avg_versions,
            total_versions,
            most_changed,
            most_stable,
        }
    });
    let patterns = mine_patterns(statutes);
    MetricsDashboard {
        generated_at: chrono::Utc::now().naive_utc(),
        statistics,
        graph_metrics,
        top_centrality,
        quality_summary,
        conflict_summary,
        coverage_info,
        evolution_summary,
        patterns,
    }
}
/// Exports dashboard to JSON
pub fn export_dashboard_json(dashboard: &MetricsDashboard) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(dashboard)
}
/// Exports dashboard to HTML
pub fn export_dashboard_html(dashboard: &MetricsDashboard, title: &str) -> String {
    let mut html = String::new();
    html.push_str("<!DOCTYPE html>\n<html>\n<head>\n");
    html.push_str(&format!("<title>{}</title>\n", title));
    html.push_str("<style>\n");
    html.push_str("body { font-family: Arial, sans-serif; margin: 20px; background: #f5f5f5; }\n");
    html.push_str("h1 { color: #333; border-bottom: 2px solid #007bff; padding-bottom: 10px; }\n");
    html.push_str("h2 { color: #555; margin-top: 30px; }\n");
    html.push_str(
        ".card { background: white; padding: 20px; margin: 20px 0; border-radius: 5px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }\n",
    );
    html.push_str(".metric { display: inline-block; margin: 10px 20px 10px 0; }\n");
    html.push_str(".metric-label { font-weight: bold; color: #666; }\n");
    html.push_str(".metric-value { font-size: 1.2em; color: #007bff; }\n");
    html.push_str("table { width: 100%; border-collapse: collapse; margin-top: 10px; }\n");
    html.push_str("th, td { padding: 10px; text-align: left; border-bottom: 1px solid #ddd; }\n");
    html.push_str("th { background: #007bff; color: white; }\n");
    html.push_str("tr:hover { background: #f9f9f9; }\n");
    html.push_str(".critical { color: #dc3545; font-weight: bold; }\n");
    html.push_str(".warning { color: #ffc107; }\n");
    html.push_str(".success { color: #28a745; }\n");
    html.push_str("</style>\n</head>\n<body>\n");
    html.push_str(&format!("<h1>{}</h1>\n", title));
    html.push_str(&format!(
        "<p><em>Generated: {}</em></p>\n",
        dashboard.generated_at
    ));
    html.push_str("<div class=\"card\">\n<h2>Overview</h2>\n");
    html.push_str(
        &format!(
            "<div class=\"metric\"><span class=\"metric-label\">Total Statutes:</span> <span class=\"metric-value\">{}</span></div>\n",
            dashboard.statistics.total_count
        ),
    );
    html.push_str(
        &format!(
            "<div class=\"metric\"><span class=\"metric-label\">Average Quality:</span> <span class=\"metric-value\">{:.1}</span></div>\n",
            dashboard.quality_summary.average_score
        ),
    );
    html.push_str(
        &format!(
            "<div class=\"metric\"><span class=\"metric-label\">Total Conflicts:</span> <span class=\"metric-value {}\">{}</span></div>\n",
            if dashboard.conflict_summary.total_conflicts > 0 { "critical" } else {
            "success" }, dashboard.conflict_summary.total_conflicts
        ),
    );
    html.push_str("</div>\n");
    html.push_str("<div class=\"card\">\n<h2>Dependency Graph</h2>\n");
    html.push_str(
        &format!(
            "<div class=\"metric\"><span class=\"metric-label\">Nodes:</span> <span class=\"metric-value\">{}</span></div>\n",
            dashboard.graph_metrics.node_count
        ),
    );
    html.push_str(
        &format!(
            "<div class=\"metric\"><span class=\"metric-label\">Edges:</span> <span class=\"metric-value\">{}</span></div>\n",
            dashboard.graph_metrics.edge_count
        ),
    );
    html.push_str(
        &format!(
            "<div class=\"metric\"><span class=\"metric-label\">Density:</span> <span class=\"metric-value\">{:.4}</span></div>\n",
            dashboard.graph_metrics.density
        ),
    );
    html.push_str(
        &format!(
            "<div class=\"metric\"><span class=\"metric-label\">Is DAG:</span> <span class=\"metric-value {}\">{}</span></div>\n",
            if dashboard.graph_metrics.is_acyclic { "success" } else { "critical" },
            dashboard.graph_metrics.is_acyclic
        ),
    );
    html.push_str("</div>\n");
    html.push_str("<div class=\"card\">\n<h2>Top 10 Statutes by Importance (PageRank)</h2>\n");
    html.push_str(
        "<table>\n<tr><th>Rank</th><th>Statute ID</th><th>PageRank</th><th>In-Degree</th><th>Out-Degree</th></tr>\n",
    );
    for (i, metric) in dashboard.top_centrality.iter().enumerate() {
        html.push_str(&format!(
            "<tr><td>{}</td><td>{}</td><td>{:.4}</td><td>{}</td><td>{}</td></tr>\n",
            i + 1,
            metric.statute_id,
            metric.pagerank,
            metric.in_degree,
            metric.out_degree
        ));
    }
    html.push_str("</table>\n</div>\n");
    html.push_str("<div class=\"card\">\n<h2>Quality Summary</h2>\n");
    html.push_str("<table>\n<tr><th>Grade</th><th>Count</th></tr>\n");
    let mut grades: Vec<_> = dashboard
        .quality_summary
        .grade_distribution
        .iter()
        .collect();
    grades.sort_by(|a, b| a.0.cmp(b.0));
    for (grade, count) in grades {
        html.push_str(&format!("<tr><td>{}</td><td>{}</td></tr>\n", grade, count));
    }
    html.push_str("</table>\n");
    html.push_str(&format!(
        "<p>Statutes with Issues: <span class=\"warning\">{}</span></p>\n",
        dashboard.quality_summary.statutes_with_issues
    ));
    html.push_str("</div>\n");
    html.push_str("<div class=\"card\">\n<h2>Common Patterns</h2>\n");
    html.push_str(
        "<table>\n<tr><th>Pattern</th><th>Type</th><th>Frequency</th><th>Percentage</th></tr>\n",
    );
    for pattern in &dashboard.patterns {
        let percentage =
            (pattern.frequency as f64 / dashboard.statistics.total_count as f64) * 100.0;
        html.push_str(&format!(
            "<tr><td>{}</td><td>{}</td><td>{}</td><td>{:.1}%</td></tr>\n",
            pattern.description, pattern.pattern_type, pattern.frequency, percentage
        ));
    }
    html.push_str("</table>\n</div>\n");
    if let Some(evolution) = &dashboard.evolution_summary {
        html.push_str("<div class=\"card\">\n<h2>Evolution Summary</h2>\n");
        html.push_str(
            &format!(
                "<div class=\"metric\"><span class=\"metric-label\">Tracked Statutes:</span> <span class=\"metric-value\">{}</span></div>\n",
                evolution.total_tracked
            ),
        );
        html.push_str(
            &format!(
                "<div class=\"metric\"><span class=\"metric-label\">Avg Versions:</span> <span class=\"metric-value\">{:.2}</span></div>\n",
                evolution.avg_versions
            ),
        );
        if let Some(most_changed) = &evolution.most_changed {
            html.push_str(&format!(
                "<p>Most Changed: <strong>{}</strong></p>\n",
                most_changed
            ));
        }
        if let Some(most_stable) = &evolution.most_stable {
            html.push_str(&format!(
                "<p>Most Stable: <strong>{}</strong></p>\n",
                most_stable
            ));
        }
        html.push_str("</div>\n");
    }
    html.push_str("</body>\n</html>");
    html
}
/// Generates a markdown summary of the dashboard
pub fn dashboard_markdown_summary(dashboard: &MetricsDashboard) -> String {
    let mut report = String::new();
    report.push_str("# Comprehensive Metrics Dashboard\n\n");
    report.push_str(&format!("**Generated**: {}\n\n", dashboard.generated_at));
    report.push_str("## Overview\n\n");
    report.push_str(&format!(
        "- **Total Statutes**: {}\n",
        dashboard.statistics.total_count
    ));
    report.push_str(&format!(
        "- **Average Quality Score**: {:.1}/100\n",
        dashboard.quality_summary.average_score
    ));
    report.push_str(&format!(
        "- **Total Conflicts**: {}\n",
        dashboard.conflict_summary.total_conflicts
    ));
    report.push_str(&format!(
        "- **Critical Conflicts**: {}\n",
        dashboard.conflict_summary.critical_conflicts
    ));
    report.push('\n');
    report.push_str("## Graph Structure\n\n");
    report.push_str(&format!(
        "- **Nodes**: {}\n",
        dashboard.graph_metrics.node_count
    ));
    report.push_str(&format!(
        "- **Edges**: {}\n",
        dashboard.graph_metrics.edge_count
    ));
    report.push_str(&format!(
        "- **Density**: {:.4}\n",
        dashboard.graph_metrics.density
    ));
    report.push_str(&format!(
        "- **Is Acyclic**: {}\n",
        dashboard.graph_metrics.is_acyclic
    ));
    report.push_str(&format!(
        "- **Diameter**: {}\n",
        dashboard.graph_metrics.diameter
    ));
    report.push('\n');
    report.push_str("## Quality Distribution\n\n");
    let mut grades: Vec<_> = dashboard
        .quality_summary
        .grade_distribution
        .iter()
        .collect();
    grades.sort_by(|a, b| a.0.cmp(b.0));
    for (grade, count) in grades {
        report.push_str(&format!("- Grade {}: {} statutes\n", grade, count));
    }
    report.push('\n');
    report.push_str("## Top Patterns\n\n");
    for (i, pattern) in dashboard.patterns.iter().take(5).enumerate() {
        let percentage =
            (pattern.frequency as f64 / dashboard.statistics.total_count as f64) * 100.0;
        report.push_str(&format!(
            "{}. {} - {} statutes ({:.1}%)\n",
            i + 1,
            pattern.description,
            pattern.frequency,
            percentage
        ));
    }
    report.push('\n');
    if let Some(evolution) = &dashboard.evolution_summary {
        report.push_str("## Evolution Tracking\n\n");
        report.push_str(&format!(
            "- **Tracked Statutes**: {}\n",
            evolution.total_tracked
        ));
        report.push_str(&format!(
            "- **Average Versions**: {:.2}\n",
            evolution.avg_versions
        ));
        if let Some(most_changed) = &evolution.most_changed {
            report.push_str(&format!("- **Most Changed**: {}\n", most_changed));
        }
        if let Some(most_stable) = &evolution.most_stable {
            report.push_str(&format!("- **Most Stable**: {}\n", most_stable));
        }
        report.push('\n');
    }
    report
}
/// Analyzes interactions between statutes
pub fn analyze_statute_interactions(statutes: &[Statute]) -> Vec<StatuteInteraction> {
    let mut interactions = Vec::new();
    for i in 0..statutes.len() {
        for j in (i + 1)..statutes.len() {
            let statute_a = &statutes[i];
            let statute_b = &statutes[j];
            let a_refs_b = extract_statute_references_from_conditions(&statute_a.preconditions)
                .contains(&statute_b.id);
            let b_refs_a = extract_statute_references_from_conditions(&statute_b.preconditions)
                .contains(&statute_a.id);
            if a_refs_b && b_refs_a {
                interactions.push(StatuteInteraction {
                    statute_a: statute_a.id.clone(),
                    statute_b: statute_b.id.clone(),
                    interaction_type: InteractionType::MutualDependency,
                    description: format!(
                        "{} and {} have mutual dependencies",
                        statute_a.id, statute_b.id
                    ),
                    severity: Severity::Warning,
                    recommendation:
                        "Review mutual dependencies for circular logic and consider refactoring"
                            .to_string(),
                });
            }
            if a_refs_b && matches!(statute_a.effect.effect_type, EffectType::Revoke) {
                interactions.push(StatuteInteraction {
                    statute_a: statute_a.id.clone(),
                    statute_b: statute_b.id.clone(),
                    interaction_type: InteractionType::Modification,
                    description: format!("{} modifies or revokes {}", statute_a.id, statute_b.id),
                    severity: Severity::Info,
                    recommendation: "Ensure modification is intentional and properly documented"
                        .to_string(),
                });
            }
            if a_refs_b && matches!(statute_a.effect.effect_type, EffectType::Grant) {
                interactions.push(StatuteInteraction {
                    statute_a: statute_a.id.clone(),
                    statute_b: statute_b.id.clone(),
                    interaction_type: InteractionType::Extension,
                    description: format!("{} extends {}", statute_a.id, statute_b.id),
                    severity: Severity::Info,
                    recommendation: "Verify that extension is coherent with base statute"
                        .to_string(),
                });
            }
            if effects_contradict(&statute_a.effect, &statute_b.effect)
                && conditions_overlap(&statute_a.preconditions, &statute_b.preconditions)
            {
                interactions.push(StatuteInteraction {
                    statute_a: statute_a.id.clone(),
                    statute_b: statute_b.id.clone(),
                    interaction_type: InteractionType::Contradiction,
                    description: format!(
                        "{} and {} have contradictory effects with overlapping conditions",
                        statute_a.id, statute_b.id
                    ),
                    severity: Severity::Critical,
                    recommendation:
                        "Resolve contradiction by clarifying precedence or narrowing conditions"
                            .to_string(),
                });
            }
            if statute_a.jurisdiction == statute_b.jurisdiction {
                let similarity = semantic_similarity(statute_a, statute_b).0;
                if similarity > 0.6 {
                    interactions.push(StatuteInteraction {
                        statute_a: statute_a.id.clone(),
                        statute_b: statute_b.id.clone(),
                        interaction_type: InteractionType::Overlap,
                        description: format!(
                            "{} and {} have significant overlap (similarity: {:.1}%)",
                            statute_a.id,
                            statute_b.id,
                            similarity * 100.0
                        ),
                        severity: Severity::Warning,
                        recommendation: "Consider consolidating overlapping statutes".to_string(),
                    });
                }
            }
            if statute_a.jurisdiction == statute_b.jurisdiction
                && !effects_contradict(&statute_a.effect, &statute_b.effect)
                && (a_refs_b || b_refs_a)
            {
                interactions.push(StatuteInteraction {
                    statute_a: statute_a.id.clone(),
                    statute_b: statute_b.id.clone(),
                    interaction_type: InteractionType::Complementary,
                    description: format!(
                        "{} and {} complement each other",
                        statute_a.id, statute_b.id
                    ),
                    severity: Severity::Info,
                    recommendation: "Document complementary relationship for clarity".to_string(),
                });
            }
        }
    }
    interactions
}
/// Report on statute interactions
pub fn statute_interaction_report(interactions: &[StatuteInteraction]) -> String {
    let mut report = String::new();
    report.push_str("# Statute Interaction Analysis\n\n");
    report.push_str(&format!(
        "**Total Interactions**: {}\n\n",
        interactions.len()
    ));
    let mut by_type: HashMap<InteractionType, Vec<&StatuteInteraction>> = HashMap::new();
    for interaction in interactions {
        by_type
            .entry(interaction.interaction_type)
            .or_default()
            .push(interaction);
    }
    for (interaction_type, items) in by_type.iter() {
        report.push_str(&format!(
            "## {} ({} interactions)\n\n",
            interaction_type,
            items.len()
        ));
        for interaction in items {
            report.push_str(&format!(
                "### {} ↔ {}\n\n",
                interaction.statute_a, interaction.statute_b
            ));
            report.push_str(&format!("- **Severity**: {}\n", interaction.severity));
            report.push_str(&format!("- **Description**: {}\n", interaction.description));
            report.push_str(&format!(
                "- **Recommendation**: {}\n\n",
                interaction.recommendation
            ));
        }
    }
    report
}
/// Detects regulatory overlaps between statutes
pub fn detect_regulatory_overlaps(statutes: &[Statute]) -> Vec<RegulatoryOverlap> {
    let mut overlaps = Vec::new();
    let mut by_jurisdiction: HashMap<String, Vec<&Statute>> = HashMap::new();
    for statute in statutes {
        if let Some(jurisdiction) = &statute.jurisdiction {
            by_jurisdiction
                .entry(jurisdiction.clone())
                .or_default()
                .push(statute);
        }
    }
    for (jurisdiction, group) in by_jurisdiction.iter() {
        if group.len() < 2 {
            continue;
        }
        for i in 0..group.len() {
            for j in (i + 1)..group.len() {
                let statute_a = group[i];
                let statute_b = group[j];
                let tv_a = &statute_a.temporal_validity;
                let tv_b = &statute_b.temporal_validity;
                if temporal_validity_overlaps(tv_a, tv_b) {
                    overlaps.push(RegulatoryOverlap {
                        statute_ids: vec![statute_a.id.clone(), statute_b.id.clone()],
                        overlap_area: OverlapArea::Temporal,
                        description: format!(
                            "{} and {} have overlapping validity periods in {}",
                            statute_a.id, statute_b.id, jurisdiction
                        ),
                        severity: Severity::Warning,
                        resolution: "Clarify which statute takes precedence during overlap period"
                            .to_string(),
                    });
                }
                let a_has_age = has_age_condition(&statute_a.preconditions);
                let b_has_age = has_age_condition(&statute_b.preconditions);
                let a_has_income = has_income_condition(&statute_a.preconditions);
                let b_has_income = has_income_condition(&statute_b.preconditions);
                if (a_has_age && b_has_age) || (a_has_income && b_has_income) {
                    let cond_overlap =
                        conditions_overlap(&statute_a.preconditions, &statute_b.preconditions);
                    if cond_overlap {
                        overlaps.push(RegulatoryOverlap {
                            statute_ids: vec![statute_a.id.clone(), statute_b.id.clone()],
                            overlap_area: OverlapArea::Population,
                            description: format!(
                                "{} and {} target overlapping populations",
                                statute_a.id, statute_b.id
                            ),
                            severity: Severity::Info,
                            resolution: "Verify that overlapping coverage is intentional"
                                .to_string(),
                        });
                    }
                }
                let title_sim = title_similarity(&statute_a.title, &statute_b.title);
                if title_sim > 0.5 {
                    overlaps.push(RegulatoryOverlap {
                        statute_ids: vec![statute_a.id.clone(), statute_b.id.clone()],
                        overlap_area: OverlapArea::SubjectMatter,
                        description: format!(
                            "{} and {} address similar subject matter (similarity: {:.1}%)",
                            statute_a.id,
                            statute_b.id,
                            title_sim * 100.0
                        ),
                        severity: Severity::Info,
                        resolution: "Consider consolidating if they address the same topic"
                            .to_string(),
                    });
                }
            }
        }
    }
    overlaps
}
/// Report on regulatory overlaps
pub fn regulatory_overlap_report(overlaps: &[RegulatoryOverlap]) -> String {
    let mut report = String::new();
    report.push_str("# Regulatory Overlap Analysis\n\n");
    report.push_str(&format!("**Total Overlaps**: {}\n\n", overlaps.len()));
    let mut by_area: HashMap<OverlapArea, Vec<&RegulatoryOverlap>> = HashMap::new();
    for overlap in overlaps {
        by_area
            .entry(overlap.overlap_area.clone())
            .or_default()
            .push(overlap);
    }
    for (area, items) in by_area.iter() {
        report.push_str(&format!("## {} Overlaps ({} found)\n\n", area, items.len()));
        for overlap in items {
            report.push_str(&format!(
                "### Statutes: {}\n\n",
                overlap.statute_ids.join(", ")
            ));
            report.push_str(&format!("- **Severity**: {}\n", overlap.severity));
            report.push_str(&format!("- **Description**: {}\n", overlap.description));
            report.push_str(&format!("- **Resolution**: {}\n\n", overlap.resolution));
        }
    }
    report
}
/// Predicts conflict cascades based on statute dependencies
pub fn predict_conflict_cascades(
    statutes: &[Statute],
    conflicts: &[StatuteConflict],
) -> Vec<ConflictCascade> {
    let mut cascades = Vec::new();
    let mut deps: HashMap<String, Vec<String>> = HashMap::new();
    for statute in statutes {
        let refs = extract_statute_references_from_conditions(&statute.preconditions);
        deps.insert(statute.id.clone(), refs.into_iter().collect());
    }
    for conflict in conflicts {
        let origin = conflict.statute_ids.clone();
        let mut affected = HashSet::new();
        let mut to_visit = origin.clone();
        let mut depth = 0;
        while !to_visit.is_empty() && depth < 10 {
            let mut next_level = Vec::new();
            for statute in statutes {
                if affected.contains(&statute.id) || origin.contains(&statute.id) {
                    continue;
                }
                let refs = extract_statute_references_from_conditions(&statute.preconditions);
                for visited in &to_visit {
                    if refs.contains(visited) {
                        affected.insert(statute.id.clone());
                        next_level.push(statute.id.clone());
                    }
                }
            }
            to_visit = next_level;
            depth += 1;
        }
        if !affected.is_empty() {
            let severity = if depth > 3 {
                Severity::Critical
            } else if depth > 1 {
                Severity::Error
            } else {
                Severity::Warning
            };
            let affected_count = affected.len();
            let affected_statutes: Vec<_> = affected.into_iter().collect();
            cascades.push(ConflictCascade {
                origin_statutes: origin,
                affected_statutes,
                depth,
                description: format!(
                    "Conflict cascade affecting {} statutes across {} levels",
                    affected_count, depth
                ),
                severity,
            });
        }
    }
    cascades
}
/// Report on conflict cascades
pub fn conflict_cascade_report(cascades: &[ConflictCascade]) -> String {
    let mut report = String::new();
    report.push_str("# Conflict Cascade Analysis\n\n");
    report.push_str(&format!("**Total Cascades**: {}\n\n", cascades.len()));
    if cascades.is_empty() {
        report.push_str("No conflict cascades detected. This is good!\n");
        return report;
    }
    let mut sorted_cascades = cascades.to_vec();
    sorted_cascades.sort_by(|a, b| b.severity.cmp(&a.severity).then(b.depth.cmp(&a.depth)));
    for cascade in &sorted_cascades {
        report.push_str(&format!(
            "## Cascade from: {}\n\n",
            cascade.origin_statutes.join(", ")
        ));
        report.push_str(&format!("- **Severity**: {}\n", cascade.severity));
        report.push_str(&format!("- **Depth**: {} levels\n", cascade.depth));
        report.push_str(&format!(
            "- **Affected Statutes** ({}):\n",
            cascade.affected_statutes.len()
        ));
        for statute_id in &cascade.affected_statutes {
            report.push_str(&format!("  - {}\n", statute_id));
        }
        report.push_str(&format!("\n{}\n\n", cascade.description));
        if cascade.depth > 2 {
            report
                .push_str(
                    "⚠️ **Warning**: Deep cascade detected. Consider refactoring to reduce dependencies.\n\n",
                );
        }
    }
    report
}
/// Analyzes coverage gaps in statutes with enhanced detection
#[allow(clippy::too_many_arguments)]
pub fn analyze_enhanced_coverage_gaps(statutes: &[Statute]) -> Vec<EnhancedCoverageGap> {
    let mut gaps = Vec::new();
    let mut age_thresholds: Vec<(i32, &Statute)> = Vec::new();
    for statute in statutes {
        if let Some(age) = extract_age_threshold(&statute.preconditions) {
            age_thresholds.push((age, statute));
        }
    }
    age_thresholds.sort_by_key(|(age, _)| *age);
    for i in 0..age_thresholds.len().saturating_sub(1) {
        let (age1, statute1) = age_thresholds[i];
        let (age2, statute2) = age_thresholds[i + 1];
        let gap_size = age2 - age1;
        if gap_size > 5 {
            gaps.push(EnhancedCoverageGap {
                gap_type: GapType::AgeGap,
                description: format!("Age gap between {} and {}", age1, age2),
                example_scenario: format!("Individuals aged {} are not covered", age1 + 1),
                severity: if gap_size > 10 {
                    Severity::Warning
                } else {
                    Severity::Info
                },
                related_statutes: vec![statute1.id.clone(), statute2.id.clone()],
                suggested_coverage: format!(
                    "Consider adding statute for ages {} to {}",
                    age1 + 1,
                    age2 - 1
                ),
            });
        }
    }
    let mut income_thresholds: Vec<(i32, &Statute)> = Vec::new();
    for statute in statutes {
        if let Some(income) = extract_income_threshold(&statute.preconditions) {
            income_thresholds.push((income, statute));
        }
    }
    income_thresholds.sort_by_key(|(income, _)| *income);
    for i in 0..income_thresholds.len().saturating_sub(1) {
        let (income1, statute1) = income_thresholds[i];
        let (income2, statute2) = income_thresholds[i + 1];
        let gap_size = income2 - income1;
        if gap_size > 10000 {
            gaps.push(EnhancedCoverageGap {
                gap_type: GapType::IncomeGap,
                description: format!("Income gap between ${} and ${}", income1, income2),
                example_scenario: format!("Individuals earning ${} are not covered", income1 + 1),
                severity: if gap_size > 50000 {
                    Severity::Warning
                } else {
                    Severity::Info
                },
                related_statutes: vec![statute1.id.clone(), statute2.id.clone()],
                suggested_coverage: format!(
                    "Consider adding statute for income range ${} to ${}",
                    income1 + 1,
                    income2 - 1
                ),
            });
        }
    }
    let missing_jurisdiction_statutes: Vec<_> = statutes
        .iter()
        .filter(|s| s.jurisdiction.is_none())
        .collect();
    if !missing_jurisdiction_statutes.is_empty() {
        gaps.push(EnhancedCoverageGap {
            gap_type: GapType::JurisdictionGap,
            description: format!(
                "{} statutes without jurisdiction",
                missing_jurisdiction_statutes.len()
            ),
            example_scenario: "Statutes without jurisdiction may be ambiguous".to_string(),
            severity: Severity::Warning,
            related_statutes: missing_jurisdiction_statutes
                .iter()
                .map(|s| s.id.clone())
                .collect(),
            suggested_coverage: "Add jurisdiction to all statutes".to_string(),
        });
    }
    let mut temporal_ranges: Vec<(&Statute, &legalis_core::TemporalValidity)> = Vec::new();
    for statute in statutes {
        let tv = &statute.temporal_validity;
        temporal_ranges.push((statute, tv));
    }
    temporal_ranges.sort_by_key(|a| a.1.effective_date);
    for i in 0..temporal_ranges.len().saturating_sub(1) {
        let (statute1, tv1) = temporal_ranges[i];
        let (statute2, tv2) = temporal_ranges[i + 1];
        if let (Some(end1), Some(start2)) = (&tv1.expiry_date, &tv2.effective_date)
            && start2 > end1
        {
            let gap_days = (start2.signed_duration_since(*end1)).num_days();
            if gap_days > 30 {
                gaps.push(EnhancedCoverageGap {
                    gap_type: GapType::TemporalGap,
                    description: format!("Temporal gap of {} days", gap_days),
                    example_scenario: format!("Period from {} to {} is not covered", end1, start2),
                    severity: if gap_days > 365 {
                        Severity::Warning
                    } else {
                        Severity::Info
                    },
                    related_statutes: vec![statute1.id.clone(), statute2.id.clone()],
                    suggested_coverage: format!(
                        "Consider adding coverage for the period {} to {}",
                        end1, start2
                    ),
                });
            }
        }
    }
    gaps
}
/// Report on enhanced coverage gaps
pub fn enhanced_coverage_gap_report(gaps: &[EnhancedCoverageGap]) -> String {
    let mut report = String::new();
    report.push_str("# Enhanced Coverage Gap Analysis\n\n");
    report.push_str(&format!("**Total Gaps**: {}\n\n", gaps.len()));
    if gaps.is_empty() {
        report.push_str("No significant coverage gaps detected.\n");
        return report;
    }
    let mut by_type: HashMap<GapType, Vec<&EnhancedCoverageGap>> = HashMap::new();
    for gap in gaps {
        by_type.entry(gap.gap_type).or_default().push(gap);
    }
    for (gap_type, items) in by_type.iter() {
        report.push_str(&format!("## {} ({} gaps)\n\n", gap_type, items.len()));
        for gap in items {
            report.push_str(&format!("### {}\n\n", gap.description));
            report.push_str(&format!("- **Severity**: {}\n", gap.severity));
            report.push_str(&format!("- **Example**: {}\n", gap.example_scenario));
            report.push_str(&format!(
                "- **Related Statutes**: {}\n",
                gap.related_statutes.join(", ")
            ));
            report.push_str(&format!("- **Suggestion**: {}\n\n", gap.suggested_coverage));
        }
    }
    report
}
/// Detects redundancies and suggests elimination strategies
pub fn suggest_redundancy_elimination(statutes: &[Statute]) -> Vec<RedundancyInstance> {
    let mut redundancies = Vec::new();
    for i in 0..statutes.len() {
        for j in (i + 1)..statutes.len() {
            let statute_a = &statutes[i];
            let statute_b = &statutes[j];
            let similarity = semantic_similarity(statute_a, statute_b).0;
            if similarity > 0.95 {
                let complexity_a = analyze_complexity(statute_a).complexity_score;
                let complexity_b = analyze_complexity(statute_b).complexity_score;
                redundancies.push(RedundancyInstance {
                    statute_ids: vec![statute_a.id.clone(), statute_b.id.clone()],
                    redundancy_type: RedundancyType::Duplicate,
                    description: format!(
                        "{} and {} are nearly identical (similarity: {:.1}%)",
                        statute_a.id,
                        statute_b.id,
                        similarity * 100.0
                    ),
                    elimination_strategy: if complexity_a <= complexity_b {
                        format!(
                            "Consider removing {} and keeping {}",
                            statute_b.id, statute_a.id
                        )
                    } else {
                        format!(
                            "Consider removing {} and keeping {}",
                            statute_a.id, statute_b.id
                        )
                    },
                    potential_savings: (complexity_a + complexity_b) as f64 / 2.0,
                });
            } else if similarity > 0.8 {
                redundancies.push(RedundancyInstance {
                    statute_ids: vec![statute_a.id.clone(), statute_b.id.clone()],
                    redundancy_type: RedundancyType::Subsumed,
                    description: format!(
                        "{} may be subsumed by {} (similarity: {:.1}%)",
                        statute_a.id,
                        statute_b.id,
                        similarity * 100.0
                    ),
                    elimination_strategy: "Review whether one statute can be merged into the other"
                        .to_string(),
                    potential_savings: 10.0,
                });
            }
            if conditions_overlap(&statute_a.preconditions, &statute_b.preconditions)
                && statute_a.effect.effect_type == statute_b.effect.effect_type
            {
                redundancies.push(RedundancyInstance {
                    statute_ids: vec![statute_a.id.clone(), statute_b.id.clone()],
                    redundancy_type: RedundancyType::OverlappingConditions,
                    description: format!(
                        "{} and {} have overlapping conditions and similar effects",
                        statute_a.id, statute_b.id
                    ),
                    elimination_strategy:
                        "Consider consolidating into a single statute with combined conditions"
                            .to_string(),
                    potential_savings: 15.0,
                });
            }
        }
    }
    redundancies
}
/// Report on redundancy elimination suggestions
pub fn redundancy_elimination_report(redundancies: &[RedundancyInstance]) -> String {
    let mut report = String::new();
    report.push_str("# Redundancy Elimination Analysis\n\n");
    report.push_str(&format!(
        "**Total Redundancies**: {}\n\n",
        redundancies.len()
    ));
    if redundancies.is_empty() {
        report.push_str("No redundancies detected. Statute set is lean!\n");
        return report;
    }
    let total_savings: f64 = redundancies.iter().map(|r| r.potential_savings).sum();
    report.push_str(&format!(
        "**Potential Complexity Savings**: {:.1}\n\n",
        total_savings
    ));
    let mut by_type: HashMap<RedundancyType, Vec<&RedundancyInstance>> = HashMap::new();
    for redundancy in redundancies {
        by_type
            .entry(redundancy.redundancy_type)
            .or_default()
            .push(redundancy);
    }
    for (redundancy_type, items) in by_type.iter() {
        report.push_str(&format!(
            "## {} ({} instances)\n\n",
            redundancy_type,
            items.len()
        ));
        for redundancy in items {
            report.push_str(&format!(
                "### Statutes: {}\n\n",
                redundancy.statute_ids.join(", ")
            ));
            report.push_str(&format!("- **Description**: {}\n", redundancy.description));
            report.push_str(&format!(
                "- **Strategy**: {}\n",
                redundancy.elimination_strategy
            ));
            report.push_str(&format!(
                "- **Savings**: {:.1} complexity points\n\n",
                redundancy.potential_savings
            ));
        }
    }
    report
}
/// Extracts age threshold from conditions
fn extract_age_threshold(conditions: &[legalis_core::Condition]) -> Option<i32> {
    for cond in conditions {
        if let Some(age) = extract_age_from_condition(cond) {
            return Some(age);
        }
    }
    None
}
/// Helper to extract age from a single condition (recursively)
fn extract_age_from_condition(cond: &legalis_core::Condition) -> Option<i32> {
    use legalis_core::Condition;
    match cond {
        Condition::Age { value, .. } => Some(*value as i32),
        Condition::And(left, right) | Condition::Or(left, right) => {
            extract_age_from_condition(left).or_else(|| extract_age_from_condition(right))
        }
        Condition::Not(inner) => extract_age_from_condition(inner),
        _ => None,
    }
}
/// Extracts income threshold from conditions
fn extract_income_threshold(conditions: &[legalis_core::Condition]) -> Option<i32> {
    for cond in conditions {
        if let Some(income) = extract_income_from_condition(cond) {
            return Some(income);
        }
    }
    None
}
/// Helper to extract income from a single condition (recursively)
fn extract_income_from_condition(cond: &legalis_core::Condition) -> Option<i32> {
    use legalis_core::Condition;
    match cond {
        Condition::Income { value, .. } => Some(*value as i32),
        Condition::And(left, right) | Condition::Or(left, right) => {
            extract_income_from_condition(left).or_else(|| extract_income_from_condition(right))
        }
        Condition::Not(inner) => extract_income_from_condition(inner),
        _ => None,
    }
}
/// Generates a proof for circular reference detection
pub fn generate_circular_reference_proof(
    _statutes: &[Statute],
    cycle: &[String],
) -> VerificationProof {
    let mut proof = VerificationProof::new(
        cycle.first().cloned().unwrap_or_default(),
        format!(
            "Circular reference detected in statutes: {}",
            cycle.join(" → ")
        ),
    );
    proof.add_step(ProofStep {
        step_number: 1,
        step_type: ProofStepType::Premise,
        description: "Statutes involved in potential cycle".to_string(),
        formula: format!("Cycle = [{}]", cycle.join(", ")),
        justification: "Identified through dependency graph analysis".to_string(),
        depends_on: vec![],
    });
    for (i, (from, to)) in cycle
        .iter()
        .zip(cycle.iter().cycle().skip(1))
        .enumerate()
        .take(cycle.len())
    {
        proof.add_step(ProofStep {
            step_number: i + 2,
            step_type: ProofStepType::Deduction,
            description: format!("Reference from {} to {}", from, to),
            formula: format!("{} → {}", from, to),
            justification: "Extracted from statute preconditions".to_string(),
            depends_on: vec![1],
        });
    }
    let final_step = cycle.len() + 2;
    proof
        .add_step(ProofStep {
            step_number: final_step,
            step_type: ProofStepType::Contradiction,
            description: "Circular reference detected".to_string(),
            formula: format!("{} → {} → ... → {}", cycle[0], cycle[1], cycle[0]),
            justification: format!(
                "The chain of references forms a cycle, violating acyclicity requirement. {} steps in cycle.",
                cycle.len()
            ),
            depends_on: (2..final_step).collect(),
        });
    proof.complete()
}
/// Exports proof in DOT format for visualization
pub fn export_proof_dot(proof: &VerificationProof) -> String {
    let mut dot = String::new();
    dot.push_str("digraph VerificationProof {\n");
    dot.push_str("  rankdir=TB;\n");
    dot.push_str("  node [shape=box, style=filled, fillcolor=lightblue];\n\n");
    for step in &proof.steps {
        let color = match step.step_type {
            ProofStepType::Premise => "lightgreen",
            ProofStepType::Deduction => "lightblue",
            ProofStepType::Contradiction => "salmon",
            ProofStepType::SmtResult => "lightyellow",
            ProofStepType::Simplification => "lightcyan",
            ProofStepType::Conclusion => "lightgreen",
        };
        let label = format!(
            "Step {}\\n{}\\n{}",
            step.step_number,
            step.step_type,
            step.description.chars().take(40).collect::<String>()
        );
        dot.push_str(&format!(
            "  step{} [label=\"{}\", fillcolor={}];\n",
            step.step_number, label, color
        ));
    }
    dot.push('\n');
    for step in &proof.steps {
        for dep in &step.depends_on {
            dot.push_str(&format!("  step{} -> step{};\n", dep, step.step_number));
        }
    }
    dot.push_str("}\n");
    dot
}
/// Compresses a proof by removing redundant steps
pub fn compress_proof(proof: VerificationProof) -> VerificationProof {
    let mut compressed = VerificationProof::new(&proof.statute_id, &proof.claim);
    compressed.generated_at = proof.generated_at;
    let mut essential_steps: Vec<ProofStep> = Vec::new();
    let mut step_mapping: HashMap<usize, usize> = HashMap::new();
    let mut new_step_number = 1;
    for step in &proof.steps {
        let is_essential = matches!(
            step.step_type,
            ProofStepType::Premise | ProofStepType::Contradiction | ProofStepType::Conclusion
        ) || step.depends_on.is_empty();
        if is_essential {
            step_mapping.insert(step.step_number, new_step_number);
            let mut new_step = step.clone();
            new_step.step_number = new_step_number;
            new_step.depends_on = step
                .depends_on
                .iter()
                .filter_map(|&old_num| step_mapping.get(&old_num).copied())
                .collect();
            essential_steps.push(new_step);
            new_step_number += 1;
        }
    }
    compressed.steps = essential_steps;
    compressed.is_complete = proof.is_complete;
    compressed
}
/// Generates a proof comparison report
pub fn proof_comparison_report(
    original: &VerificationProof,
    compressed: &VerificationProof,
) -> String {
    let mut report = String::new();
    report.push_str("# Proof Compression Analysis\n\n");
    report.push_str(&format!("**Original Steps**: {}\n", original.steps.len()));
    report.push_str(&format!(
        "**Compressed Steps**: {}\n",
        compressed.steps.len()
    ));
    report.push_str(&format!(
        "**Compression Ratio**: {:.1}%\n\n",
        (1.0 - (compressed.steps.len() as f64 / original.steps.len() as f64)) * 100.0
    ));
    report.push_str("## Retained Steps\n\n");
    for step in &compressed.steps {
        report.push_str(&format!(
            "- Step {}: {} - {}\n",
            step.step_number, step.step_type, step.description
        ));
    }
    report
}
/// Analyzes conflicts between multiple stakeholders
pub fn analyze_stakeholder_conflicts(
    stakeholders: &[Stakeholder],
    statutes: &[Statute],
) -> Vec<StakeholderConflict> {
    let mut conflicts = Vec::new();
    let mut statute_to_stakeholders: HashMap<String, Vec<String>> = HashMap::new();
    for stakeholder in stakeholders {
        for statute_id in &stakeholder.affected_by {
            statute_to_stakeholders
                .entry(statute_id.clone())
                .or_default()
                .push(stakeholder.id.clone());
        }
    }
    for (statute_id, affected_stakeholders) in &statute_to_stakeholders {
        if affected_stakeholders.len() < 2 {
            continue;
        }
        let statute = statutes.iter().find(|s| &s.id == statute_id);
        if statute.is_none() {
            continue;
        }
        let statute = statute.expect("invariant: statute.is_none() checked above");
        let has_prohibition = matches!(statute.effect.effect_type, EffectType::Prohibition);
        let has_grant = matches!(statute.effect.effect_type, EffectType::Grant);
        let has_revoke = matches!(statute.effect.effect_type, EffectType::Revoke);
        if has_prohibition || has_revoke {
            let mut resolutions = vec![
                "Provide clear appeal mechanism".to_string(),
                "Ensure proportionality of enforcement".to_string(),
            ];
            if has_revoke {
                resolutions.push("Implement grandfathering provisions".to_string());
            }
            conflicts.push(StakeholderConflict {
                stakeholders: affected_stakeholders.clone(),
                statutes: vec![statute_id.clone()],
                conflict_type: ConflictNature::DirectOpposition,
                severity: Severity::Warning,
                description: format!(
                    "Statute {} creates potential opposition between {} stakeholders",
                    statute_id,
                    affected_stakeholders.len()
                ),
                resolutions,
            });
        }
        if has_grant {
            conflicts.push(StakeholderConflict {
                stakeholders: affected_stakeholders.clone(),
                statutes: vec![statute_id.clone()],
                conflict_type: ConflictNature::ResourceCompetition,
                severity: Severity::Info,
                description: format!(
                    "Statute {} may create resource competition among {} stakeholders",
                    statute_id,
                    affected_stakeholders.len()
                ),
                resolutions: vec![
                    "Define clear eligibility criteria".to_string(),
                    "Establish priority ranking system".to_string(),
                    "Set resource allocation caps".to_string(),
                ],
            });
        }
    }
    for i in 0..stakeholders.len() {
        for j in (i + 1)..stakeholders.len() {
            let s1 = &stakeholders[i];
            let s2 = &stakeholders[j];
            let common_statutes: Vec<String> = s1
                .affected_by
                .iter()
                .filter(|id| s2.affected_by.contains(id))
                .cloned()
                .collect();
            if !common_statutes.is_empty() {
                let conflicting_interests = !s1.interests.is_empty()
                    && !s2.interests.is_empty()
                    && s1
                        .interests
                        .iter()
                        .all(|i1| s2.interests.iter().all(|i2| i1 != i2));
                if conflicting_interests {
                    conflicts
                        .push(StakeholderConflict {
                            stakeholders: vec![s1.id.clone(), s2.id.clone()],
                            statutes: common_statutes.clone(),
                            conflict_type: ConflictNature::InterpretationDifference,
                            severity: Severity::Warning,
                            description: format!(
                                "Stakeholders {} and {} have conflicting interests regarding {} statutes",
                                s1.name, s2.name, common_statutes.len()
                            ),
                            resolutions: vec![
                                "Provide detailed implementation guidelines".to_string(),
                                "Establish mediation process".to_string(),
                                "Create stakeholder consultation mechanism".to_string(),
                            ],
                        });
                }
            }
        }
    }
    conflicts
}
/// Generates a stakeholder conflict analysis report
pub fn stakeholder_conflict_report(conflicts: &[StakeholderConflict]) -> String {
    let mut report = String::new();
    report.push_str("# Multi-Stakeholder Conflict Analysis\n\n");
    report.push_str(&format!(
        "**Total Conflicts Detected**: {}\n\n",
        conflicts.len()
    ));
    if conflicts.is_empty() {
        report.push_str("No stakeholder conflicts detected.\n");
        return report;
    }
    let mut by_type: HashMap<ConflictNature, Vec<&StakeholderConflict>> = HashMap::new();
    for conflict in conflicts {
        by_type
            .entry(conflict.conflict_type)
            .or_default()
            .push(conflict);
    }
    for (conflict_type, type_conflicts) in &by_type {
        report.push_str(&format!(
            "## {} ({} conflicts)\n\n",
            conflict_type,
            type_conflicts.len()
        ));
        for conflict in type_conflicts {
            report.push_str(&format!(
                "### Conflict: {} stakeholders involved\n\n",
                conflict.stakeholders.len()
            ));
            report.push_str(&format!("- **Severity**: {}\n", conflict.severity));
            report.push_str(&format!(
                "- **Stakeholders**: {}\n",
                conflict.stakeholders.join(", ")
            ));
            report.push_str(&format!(
                "- **Statutes**: {}\n",
                conflict.statutes.join(", ")
            ));
            report.push_str(&format!("- **Description**: {}\n", conflict.description));
            report.push_str("\n**Potential Resolutions**:\n");
            for resolution in &conflict.resolutions {
                report.push_str(&format!("- {}\n", resolution));
            }
            report.push('\n');
        }
    }
    report
}
/// Detects Nash equilibria in statute interactions
pub fn detect_nash_equilibria(model: &GameTheoreticModel) -> Vec<&GameOutcome> {
    model
        .outcomes
        .iter()
        .filter(|outcome| outcome.is_nash_equilibrium)
        .collect()
}
/// Predicts game-theoretic outcomes from statute interactions
pub fn predict_game_outcomes(
    stakeholders: &[Stakeholder],
    _statutes: &[Statute],
) -> GameTheoreticModel {
    let stakeholder_ids: Vec<String> = stakeholders.iter().map(|s| s.id.clone()).collect();
    let mut model = GameTheoreticModel::new(stakeholder_ids);
    for (idx, stakeholder) in stakeholders.iter().enumerate() {
        let comply_strategy = Strategy::new(&stakeholder.id, "Full Compliance")
            .with_description("Comply with all applicable statutes");
        model.add_strategy(idx, comply_strategy);
        if !stakeholder.affected_by.is_empty() {
            let selective = Strategy::new(&stakeholder.id, "Selective Compliance")
                .with_description("Comply only with high-priority statutes");
            model.add_strategy(idx, selective);
        }
        let non_comply = Strategy::new(&stakeholder.id, "Non-Compliance")
            .with_description("Minimal or no compliance");
        model.add_strategy(idx, non_comply);
    }
    if stakeholders.len() == 2 {
        model.add_outcome(GameOutcome {
            strategies: vec!["Full Compliance".to_string(), "Full Compliance".to_string()],
            payoffs: vec![5, 5],
            is_nash_equilibrium: true,
            description: "Both stakeholders comply, creating stable equilibrium".to_string(),
        });
        model.add_outcome(GameOutcome {
            strategies: vec!["Full Compliance".to_string(), "Non-Compliance".to_string()],
            payoffs: vec![2, 7],
            is_nash_equilibrium: false,
            description: "Asymmetric compliance creates instability".to_string(),
        });
        model.add_outcome(GameOutcome {
            strategies: vec!["Non-Compliance".to_string(), "Full Compliance".to_string()],
            payoffs: vec![7, 2],
            is_nash_equilibrium: false,
            description: "Asymmetric compliance creates instability".to_string(),
        });
        model.add_outcome(GameOutcome {
            strategies: vec!["Non-Compliance".to_string(), "Non-Compliance".to_string()],
            payoffs: vec![1, 1],
            is_nash_equilibrium: true,
            description: "Both stakeholders defect, creating suboptimal equilibrium".to_string(),
        });
    }
    model
}
/// Generates a game-theoretic analysis report
pub fn game_theoretic_report(model: &GameTheoreticModel) -> String {
    let mut report = String::new();
    report.push_str("# Game-Theoretic Outcome Prediction\n\n");
    report.push_str(&format!("**Players**: {}\n", model.stakeholders.len()));
    report.push_str(&format!("**Total Outcomes**: {}\n\n", model.outcomes.len()));
    report.push_str("## Stakeholders and Strategies\n\n");
    for (idx, stakeholder_id) in model.stakeholders.iter().enumerate() {
        report.push_str(&format!("### {}\n\n", stakeholder_id));
        if idx < model.strategies.len() {
            report.push_str("**Available Strategies**:\n");
            for strategy in &model.strategies[idx] {
                report.push_str(&format!(
                    "- **{}**: {}\n",
                    strategy.name, strategy.description
                ));
            }
            report.push('\n');
        }
    }
    let equilibria = detect_nash_equilibria(model);
    report.push_str(&format!(
        "## Nash Equilibria ({} found)\n\n",
        equilibria.len()
    ));
    if equilibria.is_empty() {
        report.push_str("No pure-strategy Nash equilibria found.\n\n");
    } else {
        for (i, outcome) in equilibria.iter().enumerate() {
            report.push_str(&format!("### Equilibrium {}\n\n", i + 1));
            report.push_str(&format!("- **Description**: {}\n", outcome.description));
            report.push_str("- **Strategies**: ");
            report.push_str(&outcome.strategies.join(" vs. "));
            report.push('\n');
            report.push_str("- **Payoffs**: ");
            report.push_str(
                &outcome
                    .payoffs
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
            );
            report.push_str("\n\n");
        }
    }
    report.push_str("## All Possible Outcomes\n\n");
    for (i, outcome) in model.outcomes.iter().enumerate() {
        report.push_str(&format!("{}. ", i + 1));
        report.push_str(&outcome.strategies.join(" vs. "));
        report.push_str(&format!(
            " → Payoffs: ({})",
            outcome
                .payoffs
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        ));
        if outcome.is_nash_equilibrium {
            report.push_str(" **[Nash Equilibrium]**");
        }
        report.push('\n');
    }
    report
}
/// Analyzes potential coalitions among stakeholders
pub fn analyze_coalitions(stakeholders: &[Stakeholder], statutes: &[Statute]) -> Vec<Coalition> {
    let mut coalitions = Vec::new();
    let mut interest_groups: HashMap<String, Vec<String>> = HashMap::new();
    for stakeholder in stakeholders {
        for interest in &stakeholder.interests {
            interest_groups
                .entry(interest.clone())
                .or_default()
                .push(stakeholder.id.clone());
        }
    }
    for (interest, members) in &interest_groups {
        if members.len() >= 2 {
            let affected_statutes: HashSet<String> = stakeholders
                .iter()
                .filter(|s| members.contains(&s.id))
                .flat_map(|s| s.affected_by.iter().cloned())
                .collect();
            let strength = (affected_statutes.len() as f64 / statutes.len().max(1) as f64).min(1.0);
            let common_statutes = stakeholders
                .iter()
                .filter(|s| members.contains(&s.id))
                .fold(None, |acc: Option<HashSet<String>>, s| {
                    let current: HashSet<String> = s.affected_by.iter().cloned().collect();
                    match acc {
                        None => Some(current),
                        Some(prev) => Some(prev.intersection(&current).cloned().collect()),
                    }
                });
            let is_stable = common_statutes.is_some_and(|s| !s.is_empty());
            let mut coalition = Coalition::new(members.clone())
                .with_objective(interest.clone())
                .with_strength(strength)
                .with_stability(is_stable);
            for statute_id in &affected_statutes {
                if let Some(statute) = statutes.iter().find(|s| &s.id == statute_id) {
                    coalition = coalition.with_collective_effect(format!(
                        "Collectively influenced by statute {} ({})",
                        statute_id, statute.title
                    ));
                }
            }
            coalitions.push(coalition);
        }
    }
    coalitions.sort_by(|a, b| {
        b.strength
            .partial_cmp(&a.strength)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    coalitions
}
/// Generates a coalition analysis report
pub fn coalition_analysis_report(coalitions: &[Coalition]) -> String {
    let mut report = String::new();
    report.push_str("# Coalition Analysis\n\n");
    report.push_str(&format!(
        "**Total Coalitions Detected**: {}\n\n",
        coalitions.len()
    ));
    if coalitions.is_empty() {
        report.push_str("No coalitions detected. Stakeholders may have divergent interests.\n");
        return report;
    }
    let stable_count = coalitions.iter().filter(|c| c.is_stable).count();
    report.push_str(&format!("**Stable Coalitions**: {}\n", stable_count));
    report.push_str(&format!(
        "**Unstable Coalitions**: {}\n\n",
        coalitions.len() - stable_count
    ));
    for (i, coalition) in coalitions.iter().enumerate() {
        report.push_str(&format!(
            "## Coalition {} - {} members\n\n",
            i + 1,
            coalition.members.len()
        ));
        report.push_str(&format!(
            "- **Members**: {}\n",
            coalition.members.join(", ")
        ));
        report.push_str(&format!("- **Strength**: {:.2}\n", coalition.strength));
        report.push_str(&format!(
            "- **Stability**: {}\n",
            if coalition.is_stable {
                "Stable"
            } else {
                "Unstable"
            }
        ));
        if !coalition.objectives.is_empty() {
            report.push_str("\n**Shared Objectives**:\n");
            for objective in &coalition.objectives {
                report.push_str(&format!("- {}\n", objective));
            }
        }
        if !coalition.collective_effects.is_empty() {
            report.push_str("\n**Collective Effects**:\n");
            for effect in &coalition.collective_effects {
                report.push_str(&format!("- {}\n", effect));
            }
        }
        report.push('\n');
    }
    report
}
/// Verifies mechanism design properties of statutes
pub fn verify_mechanism_design(
    statutes: &[Statute],
    stakeholders: &[Stakeholder],
) -> MechanismAnalysis {
    let mut analysis = MechanismAnalysis::new();
    check_incentive_compatibility(statutes, stakeholders, &mut analysis);
    check_individual_rationality(statutes, stakeholders, &mut analysis);
    check_budget_balance(statutes, &mut analysis);
    check_strategy_proofness(statutes, &mut analysis);
    check_non_dictatorship(statutes, stakeholders, &mut analysis);
    analysis
}
