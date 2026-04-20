//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use legalis_core::Statute;
use std::collections::{HashMap, HashSet};

use super::functions::{analyze_complexity, conflict_detection_report};
use super::functions_2::extract_statute_references_from_conditions;
use super::functions_2::semantic_similarity;
use super::functions_3::{analyze_quality, batch_ambiguity_report, quality_report};
use super::types::{
    BatchVerificationResult, ComplexityTrend, ScheduledReportResult, StatuteVerifier,
    VerificationSummary,
};
use super::types_3::{
    CentralityMetrics, DuplicateCandidate, ExecutiveSummary, GraphMetrics, RegulatoryFiling,
    ReportOutputFormat,
};
use super::types_4::{
    ChangeImpact, ComplianceCertification, EvolutionTracker, RegulatoryImpact, ReportSchedule,
    ReportSection, Severity, StatuteCluster, StatuteStatistics, SummaryStatistics,
};
use super::types_5::{ComplianceItem, ReportTemplate, StatuteFilingInfo, VerificationResult};

/// Generates a change impact report.
pub fn change_impact_report(impact: &ChangeImpact) -> String {
    let mut report = String::from("# Change Impact Analysis\n\n");
    report.push_str(&format!("## Statute: {}\n\n", impact.statute_id));
    report.push_str(&format!(
        "**Impact Severity**: {:?}\n\n",
        impact.impact_severity
    ));
    report.push_str("### Changes Detected\n\n");
    if impact.changes.is_empty() {
        report.push_str("No changes detected.\n\n");
    } else {
        for (i, change) in impact.changes.iter().enumerate() {
            report.push_str(&format!("{}. {}\n", i + 1, change));
        }
        report.push('\n');
    }
    report.push_str("### Affected Statutes\n\n");
    if impact.affected_statutes.is_empty() {
        report.push_str("No statutes are directly affected by this change.\n\n");
    } else {
        report.push_str(&format!(
            "{} statute(s) reference this statute and may be affected:\n\n",
            impact.affected_statutes.len()
        ));
        for statute_id in &impact.affected_statutes {
            report.push_str(&format!("- {}\n", statute_id));
        }
        report.push('\n');
    }
    report.push_str("### Recommendations\n\n");
    if impact.recommendations.is_empty() {
        report.push_str("No specific recommendations.\n\n");
    } else {
        for rec in &impact.recommendations {
            report.push_str(&format!("- {}\n", rec));
        }
        report.push('\n');
    }
    report
}
/// Performs batch verification on multiple statutes and returns aggregate results.
pub fn batch_verify(statutes: &[Statute], verifier: &StatuteVerifier) -> BatchVerificationResult {
    let start = std::time::Instant::now();
    let mut batch_result = BatchVerificationResult::new();
    for statute in statutes {
        let result = verifier.verify(std::slice::from_ref(statute));
        batch_result.add_result(statute.id.clone(), result);
    }
    batch_result.total_time_ms = start.elapsed().as_millis() as u64;
    batch_result
}
/// Generates a batch verification report.
pub fn batch_verification_report(result: &BatchVerificationResult) -> String {
    let mut report = String::from("# Batch Verification Report\n\n");
    report.push_str("## Summary\n\n");
    report.push_str(&format!("- Total statutes: {}\n", result.total_statutes));
    report.push_str(&format!("- Passed: {}\n", result.passed));
    report.push_str(&format!("- Failed: {}\n", result.failed));
    report.push_str(&format!("- Pass rate: {:.1}%\n", result.pass_rate()));
    report.push_str(&format!(
        "- Total verification time: {}ms\n\n",
        result.total_time_ms
    ));
    report.push_str("## Error Distribution\n\n");
    if result.error_counts.is_empty() {
        report.push_str("No errors detected.\n\n");
    } else {
        for severity in [
            Severity::Critical,
            Severity::Error,
            Severity::Warning,
            Severity::Info,
        ] {
            if let Some(count) = result.error_counts.get(&severity) {
                report.push_str(&format!("- {}: {}\n", severity, count));
            }
        }
        report.push('\n');
    }
    report.push_str("## Failed Statutes\n\n");
    let mut failed_statutes: Vec<_> = result
        .individual_results
        .iter()
        .filter(|(_, r)| !r.passed)
        .collect();
    if failed_statutes.is_empty() {
        report.push_str("All statutes passed verification.\n\n");
    } else {
        failed_statutes.sort_by_key(|(id, _)| id.as_str());
        for (statute_id, verification_result) in failed_statutes {
            report.push_str(&format!("### {}\n\n", statute_id));
            report.push_str(&format!("- Errors: {}\n", verification_result.errors.len()));
            report.push_str(&format!(
                "- Warnings: {}\n",
                verification_result.warnings.len()
            ));
            if !verification_result.errors.is_empty() {
                report.push_str("\n**Errors:**\n\n");
                for error in &verification_result.errors {
                    report.push_str(&format!("- [{:?}] {}\n", error.severity(), error));
                }
            }
            report.push('\n');
        }
    }
    report
}
/// Analyzes a collection of statutes and returns comprehensive statistics.
pub fn analyze_statute_statistics(statutes: &[Statute]) -> StatuteStatistics {
    if statutes.is_empty() {
        return StatuteStatistics {
            total_count: 0,
            avg_preconditions: 0.0,
            median_preconditions: 0.0,
            common_condition_types: Vec::new(),
            jurisdiction_distribution: HashMap::new(),
            avg_complexity: 0.0,
            effect_type_distribution: HashMap::new(),
            discretion_count: 0,
            temporal_coverage: 0.0,
        };
    }
    let total_count = statutes.len();
    let mut precondition_counts: Vec<usize> =
        statutes.iter().map(|s| s.preconditions.len()).collect();
    precondition_counts.sort_unstable();
    let total_preconditions: usize = precondition_counts.iter().sum();
    let avg_preconditions = total_preconditions as f64 / total_count as f64;
    let median_preconditions = if precondition_counts.len().is_multiple_of(2) {
        let mid = precondition_counts.len() / 2;
        (precondition_counts[mid - 1] + precondition_counts[mid]) as f64 / 2.0
    } else {
        precondition_counts[precondition_counts.len() / 2] as f64
    };
    let mut condition_type_counts: HashMap<String, usize> = HashMap::new();
    for statute in statutes {
        for condition in &statute.preconditions {
            let type_name = format!("{:?}", condition)
                .split('{')
                .next()
                .unwrap_or("Unknown")
                .to_string();
            *condition_type_counts.entry(type_name).or_insert(0) += 1;
        }
    }
    let mut common_condition_types: Vec<(String, usize)> =
        condition_type_counts.into_iter().collect();
    common_condition_types.sort_by_key(|b| std::cmp::Reverse(b.1));
    common_condition_types.truncate(10);
    let mut jurisdiction_distribution: HashMap<String, usize> = HashMap::new();
    for statute in statutes {
        let jurisdiction = statute
            .jurisdiction
            .as_deref()
            .unwrap_or("None")
            .to_string();
        *jurisdiction_distribution.entry(jurisdiction).or_insert(0) += 1;
    }
    let total_complexity: u32 = statutes
        .iter()
        .map(|s| analyze_complexity(s).complexity_score)
        .sum();
    let avg_complexity = total_complexity as f64 / total_count as f64;
    let mut effect_type_distribution: HashMap<String, usize> = HashMap::new();
    for statute in statutes {
        let effect_type = format!("{:?}", statute.effect.effect_type);
        *effect_type_distribution.entry(effect_type).or_insert(0) += 1;
    }
    let discretion_count = statutes
        .iter()
        .filter(|s| s.discretion_logic.is_some())
        .count();
    let temporal_count = statutes
        .iter()
        .filter(|s| {
            s.temporal_validity.effective_date.is_some() || s.temporal_validity.enacted_at.is_some()
        })
        .count();
    let temporal_coverage = (temporal_count as f64 / total_count as f64) * 100.0;
    StatuteStatistics {
        total_count,
        avg_preconditions,
        median_preconditions,
        common_condition_types,
        jurisdiction_distribution,
        avg_complexity,
        effect_type_distribution,
        discretion_count,
        temporal_coverage,
    }
}
/// Generates a statistical report for a statute collection.
pub fn statistics_report(statutes: &[Statute]) -> String {
    let stats = analyze_statute_statistics(statutes);
    let mut report = String::from("# Statute Collection Statistics\n\n");
    report.push_str("## Overview\n\n");
    report.push_str(&format!("- **Total Statutes**: {}\n", stats.total_count));
    report.push_str(&format!(
        "- **Average Preconditions**: {:.2}\n",
        stats.avg_preconditions
    ));
    report.push_str(&format!(
        "- **Median Preconditions**: {:.1}\n",
        stats.median_preconditions
    ));
    report.push_str(&format!(
        "- **Average Complexity**: {:.2}\n",
        stats.avg_complexity
    ));
    report.push_str(&format!(
        "- **Statutes with Discretion Logic**: {} ({:.1}%)\n",
        stats.discretion_count,
        (stats.discretion_count as f64 / stats.total_count as f64) * 100.0
    ));
    report.push_str(&format!(
        "- **Temporal Coverage**: {:.1}%\n\n",
        stats.temporal_coverage
    ));
    report.push_str("## Common Condition Types\n\n");
    for (i, (condition_type, count)) in stats.common_condition_types.iter().enumerate() {
        report.push_str(&format!(
            "{}. **{}**: {} occurrences\n",
            i + 1,
            condition_type,
            count
        ));
    }
    report.push('\n');
    report.push_str("## Jurisdiction Distribution\n\n");
    let mut jurisdictions: Vec<_> = stats.jurisdiction_distribution.iter().collect();
    jurisdictions.sort_by(|a, b| b.1.cmp(a.1));
    for (jurisdiction, count) in jurisdictions {
        let percentage = (*count as f64 / stats.total_count as f64) * 100.0;
        report.push_str(&format!(
            "- **{}**: {} ({:.1}%)\n",
            jurisdiction, count, percentage
        ));
    }
    report.push('\n');
    report.push_str("## Effect Type Distribution\n\n");
    let mut effects: Vec<_> = stats.effect_type_distribution.iter().collect();
    effects.sort_by(|a, b| b.1.cmp(a.1));
    for (effect_type, count) in effects {
        let percentage = (*count as f64 / stats.total_count as f64) * 100.0;
        report.push_str(&format!(
            "- **{}**: {} ({:.1}%)\n",
            effect_type, count, percentage
        ));
    }
    report
}
/// Detects potential duplicate or near-duplicate statutes.
pub fn detect_duplicates(statutes: &[Statute], min_similarity: f64) -> Vec<DuplicateCandidate> {
    let mut duplicates = Vec::new();
    for i in 0..statutes.len() {
        for j in (i + 1)..statutes.len() {
            let stat1 = &statutes[i];
            let stat2 = &statutes[j];
            let similarity = semantic_similarity(stat1, stat2);
            if similarity.0 >= min_similarity {
                let similarity_type = if similarity.0 >= 0.95 {
                    "Near-identical"
                } else if similarity.0 >= 0.80 {
                    "Very similar"
                } else {
                    "Similar"
                };
                let recommendation = if similarity.0 >= 0.95 {
                    "Consider merging or removing duplicate".to_string()
                } else if similarity.0 >= 0.80 {
                    "Review for potential consolidation".to_string()
                } else {
                    "Review for consistency".to_string()
                };
                duplicates.push(DuplicateCandidate {
                    statute_ids: vec![stat1.id.clone(), stat2.id.clone()],
                    similarity_score: similarity.0,
                    similarity_type: similarity_type.to_string(),
                    recommendation,
                });
            }
        }
    }
    duplicates.sort_by(|a, b| {
        b.similarity_score
            .partial_cmp(&a.similarity_score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    duplicates
}
/// Generates a duplicate detection report.
pub fn duplicate_detection_report(statutes: &[Statute], min_similarity: f64) -> String {
    let duplicates = detect_duplicates(statutes, min_similarity);
    let mut report = String::from("# Duplicate Detection Report\n\n");
    report.push_str(&format!(
        "**Minimum Similarity Threshold**: {:.0}%\n\n",
        min_similarity * 100.0
    ));
    if duplicates.is_empty() {
        report.push_str("No duplicates or similar statutes found.\n");
        return report;
    }
    report.push_str(&format!(
        "Found **{}** potential duplicate(s) or similar statute(s):\n\n",
        duplicates.len()
    ));
    for (i, dup) in duplicates.iter().enumerate() {
        report.push_str(&format!("## Duplicate Group #{}\n\n", i + 1));
        report.push_str(&format!(
            "- **Similarity**: {:.1}% ({})\n",
            dup.similarity_score * 100.0,
            dup.similarity_type
        ));
        report.push_str("- **Statutes**:\n");
        for statute_id in &dup.statute_ids {
            report.push_str(&format!("  - {}\n", statute_id));
        }
        report.push_str(&format!("- **Recommendation**: {}\n\n", dup.recommendation));
    }
    report
}
/// Analyzes the regulatory impact of a statute.
pub fn analyze_regulatory_impact(statute: &Statute) -> RegulatoryImpact {
    let complexity_metrics = analyze_complexity(statute);
    let compliance_complexity = complexity_metrics.complexity_score;
    let mut impact_score = compliance_complexity;
    let effect_weight = match statute.effect.effect_type {
        legalis_core::EffectType::Prohibition => 30,
        legalis_core::EffectType::Obligation => 25,
        legalis_core::EffectType::Revoke => 20,
        legalis_core::EffectType::Grant => 10,
        legalis_core::EffectType::MonetaryTransfer => 20,
        legalis_core::EffectType::StatusChange => 15,
        legalis_core::EffectType::Custom => 15,
    };
    impact_score = (impact_score + effect_weight).min(100);
    let precondition_weight = (statute.preconditions.len() as u32 * 5).min(30);
    impact_score = (impact_score + precondition_weight).min(100);
    let impact_level = if impact_score >= 75 {
        "High Impact"
    } else if impact_score >= 50 {
        "Medium Impact"
    } else if impact_score >= 25 {
        "Low Impact"
    } else {
        "Minimal Impact"
    };
    let affected_entities = if statute.preconditions.is_empty() {
        "Potentially all entities"
    } else if statute.preconditions.len() <= 2 {
        "Broad population"
    } else if statute.preconditions.len() <= 5 {
        "Specific demographic"
    } else {
        "Narrow subset"
    };
    let implementation_cost = if impact_score >= 75 {
        "High - Significant resources required"
    } else if impact_score >= 50 {
        "Medium - Moderate resources required"
    } else {
        "Low - Minimal resources required"
    };
    let ongoing_cost = if complexity_metrics.complexity_score >= 70 {
        "High - Ongoing monitoring and compliance needed"
    } else if complexity_metrics.complexity_score >= 40 {
        "Medium - Periodic compliance checks needed"
    } else {
        "Low - Minimal ongoing requirements"
    };
    RegulatoryImpact {
        statute_id: statute.id.clone(),
        impact_score,
        compliance_complexity,
        affected_entities: affected_entities.to_string(),
        implementation_cost: implementation_cost.to_string(),
        ongoing_cost: ongoing_cost.to_string(),
        impact_level: impact_level.to_string(),
    }
}
/// Generates a regulatory impact report for multiple statutes.
pub fn regulatory_impact_report(statutes: &[Statute]) -> String {
    let mut report = String::from("# Regulatory Impact Assessment\n\n");
    let impacts: Vec<RegulatoryImpact> = statutes.iter().map(analyze_regulatory_impact).collect();
    let total_score: u32 = impacts.iter().map(|i| i.impact_score).sum();
    let avg_score = if !impacts.is_empty() {
        total_score as f64 / impacts.len() as f64
    } else {
        0.0
    };
    let high_impact_count = impacts
        .iter()
        .filter(|i| i.impact_level == "High Impact")
        .count();
    let medium_impact_count = impacts
        .iter()
        .filter(|i| i.impact_level == "Medium Impact")
        .count();
    let low_impact_count = impacts
        .iter()
        .filter(|i| i.impact_level == "Low Impact")
        .count();
    report.push_str("## Summary\n\n");
    report.push_str(&format!(
        "- **Total Statutes Analyzed**: {}\n",
        statutes.len()
    ));
    report.push_str(&format!(
        "- **Average Impact Score**: {:.1}/100\n",
        avg_score
    ));
    report.push_str(&format!("- **High Impact**: {}\n", high_impact_count));
    report.push_str(&format!("- **Medium Impact**: {}\n", medium_impact_count));
    report.push_str(&format!(
        "- **Low/Minimal Impact**: {}\n\n",
        low_impact_count
    ));
    report.push_str("## Individual Statute Analysis\n\n");
    for impact in &impacts {
        report.push_str(&format!(
            "### {} - {}\n\n",
            impact.statute_id, impact.impact_level
        ));
        report.push_str(&format!(
            "- **Impact Score**: {}/100\n",
            impact.impact_score
        ));
        report.push_str(&format!(
            "- **Compliance Complexity**: {}/100\n",
            impact.compliance_complexity
        ));
        report.push_str(&format!(
            "- **Affected Entities**: {}\n",
            impact.affected_entities
        ));
        report.push_str(&format!(
            "- **Implementation Cost**: {}\n",
            impact.implementation_cost
        ));
        report.push_str(&format!("- **Ongoing Cost**: {}\n\n", impact.ongoing_cost));
    }
    report
}
/// Generates a compliance checklist from a statute.
pub fn generate_compliance_checklist(statute: &Statute) -> Vec<ComplianceItem> {
    let mut items = Vec::new();
    let mut item_number = 1;
    for precondition in &statute.preconditions {
        let requirement = format!("Verify: {:?}", precondition);
        let priority = "Required";
        items.push(ComplianceItem {
            number: item_number,
            requirement,
            precondition: Some(format!("{:?}", precondition)),
            priority: priority.to_string(),
        });
        item_number += 1;
    }
    let effect_requirement = format!(
        "Implement effect: {:?} - {}",
        statute.effect.effect_type, statute.effect.description
    );
    items.push(ComplianceItem {
        number: item_number,
        requirement: effect_requirement,
        precondition: None,
        priority: "Required".to_string(),
    });
    item_number += 1;
    if let Some(ref discretion) = statute.discretion_logic {
        items.push(ComplianceItem {
            number: item_number,
            requirement: format!("Consider discretion: {}", discretion),
            precondition: None,
            priority: "Optional".to_string(),
        });
        item_number += 1;
    }
    if statute.temporal_validity.effective_date.is_some()
        || statute.temporal_validity.enacted_at.is_some()
    {
        items.push(ComplianceItem {
            number: item_number,
            requirement: "Verify statute is currently in effect".to_string(),
            precondition: None,
            priority: "Required".to_string(),
        });
    }
    items
}
/// Generates a compliance checklist report for a statute.
pub fn compliance_checklist_report(statute: &Statute) -> String {
    let items = generate_compliance_checklist(statute);
    let mut report = String::from("# Compliance Checklist\n\n");
    report.push_str(&format!(
        "**Statute**: {} - {}\n\n",
        statute.id, statute.title
    ));
    if let Some(ref jurisdiction) = statute.jurisdiction {
        report.push_str(&format!("**Jurisdiction**: {}\n", jurisdiction));
    }
    report.push_str(&format!("\n**Total Items**: {}\n\n", items.len()));
    report.push_str("## Checklist Items\n\n");
    for item in &items {
        report.push_str(&format!(
            "- [ ] **Item {}** [{}]: {}\n",
            item.number, item.priority, item.requirement
        ));
    }
    report
}
/// Generates a consolidated compliance checklist for multiple statutes.
pub fn consolidated_compliance_checklist(statutes: &[Statute]) -> String {
    let mut report = String::from("# Consolidated Compliance Checklist\n\n");
    report.push_str(&format!("**Total Statutes**: {}\n\n", statutes.len()));
    for statute in statutes {
        let items = generate_compliance_checklist(statute);
        report.push_str(&format!("## {} - {}\n\n", statute.id, statute.title));
        for item in &items {
            report.push_str(&format!(
                "- [ ] **{}**: {}\n",
                item.priority, item.requirement
            ));
        }
        report.push('\n');
    }
    report
}
/// Generates a compliance certification document
pub fn generate_compliance_certification(
    certificate_id: impl Into<String>,
    organization: impl Into<String>,
    certifying_authority: impl Into<String>,
    statutes: &[Statute],
    result: &VerificationResult,
    valid_days: Option<u32>,
) -> ComplianceCertification {
    use chrono::{Duration, Utc};
    let now = Utc::now();
    let certification_date = now.format("%Y-%m-%d %H:%M:%S UTC").to_string();
    let valid_until = valid_days.map(|days| {
        (now + Duration::days(days as i64))
            .format("%Y-%m-%d %H:%M:%S UTC")
            .to_string()
    });
    let statute_ids: Vec<String> = statutes.iter().map(|s| s.id.clone()).collect();
    let critical_errors = result
        .errors
        .iter()
        .filter(|e| e.severity() == Severity::Critical)
        .count();
    let total_statutes = statutes.len();
    let passed_count = if result.passed { total_statutes } else { 0 };
    let failed_count = total_statutes - passed_count;
    let pass_rate = if total_statutes > 0 {
        (passed_count as f64 / total_statutes as f64) * 100.0
    } else {
        0.0
    };
    let verification_summary = VerificationSummary {
        total_statutes,
        passed_count,
        failed_count,
        pass_rate,
        critical_errors,
        warnings: result.warnings.len(),
    };
    ComplianceCertification {
        certificate_id: certificate_id.into(),
        certification_date,
        organization: organization.into(),
        statute_ids,
        verification_summary,
        certifying_authority: certifying_authority.into(),
        valid_until,
        notes: Vec::new(),
    }
}
/// Exports compliance certification as a formatted report
pub fn compliance_certification_report(cert: &ComplianceCertification) -> String {
    let mut report = String::from("# COMPLIANCE CERTIFICATION\n\n");
    report.push_str("---\n\n");
    report.push_str(&format!("**Certificate ID**: {}\n\n", cert.certificate_id));
    report.push_str(&format!(
        "**Certification Date**: {}\n\n",
        cert.certification_date
    ));
    report.push_str(&format!("**Organization**: {}\n\n", cert.organization));
    report.push_str(&format!(
        "**Certifying Authority**: {}\n\n",
        cert.certifying_authority
    ));
    if let Some(ref valid_until) = cert.valid_until {
        report.push_str(&format!("**Valid Until**: {}\n\n", valid_until));
    }
    report.push_str("---\n\n");
    report.push_str("## Verification Summary\n\n");
    let summary = &cert.verification_summary;
    report.push_str(&format!(
        "- **Total Statutes Verified**: {}\n",
        summary.total_statutes
    ));
    report.push_str(&format!("- **Passed**: {}\n", summary.passed_count));
    report.push_str(&format!("- **Failed**: {}\n", summary.failed_count));
    report.push_str(&format!("- **Pass Rate**: {:.2}%\n", summary.pass_rate));
    report.push_str(&format!(
        "- **Critical Errors**: {}\n",
        summary.critical_errors
    ));
    report.push_str(&format!("- **Warnings**: {}\n\n", summary.warnings));
    report.push_str("## Certified Statutes\n\n");
    for statute_id in &cert.statute_ids {
        report.push_str(&format!("- {}\n", statute_id));
    }
    report.push('\n');
    if !cert.notes.is_empty() {
        report.push_str("## Additional Notes\n\n");
        for note in &cert.notes {
            report.push_str(&format!("- {}\n", note));
        }
        report.push('\n');
    }
    report.push_str("---\n\n");
    report.push_str("This certification confirms that the listed statutes have been verified\n");
    report.push_str("using the Legalis Verification System and meet the specified compliance\n");
    report.push_str("requirements as of the certification date.\n");
    report
}
/// Generates a regulatory filing report
#[allow(clippy::too_many_arguments)]
pub fn generate_regulatory_filing(
    filing_id: impl Into<String>,
    regulatory_body: impl Into<String>,
    filing_type: impl Into<String>,
    jurisdiction: impl Into<String>,
    statutes: &[Statute],
    results: &[VerificationResult],
) -> RegulatoryFiling {
    use chrono::Utc;
    let filing_date = Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();
    let statute_infos: Vec<StatuteFilingInfo> = statutes
        .iter()
        .zip(results.iter())
        .map(|(statute, result)| {
            let status = if result.passed {
                "Compliant".to_string()
            } else if result.has_critical_errors() {
                "Non-Compliant (Critical)".to_string()
            } else {
                "Non-Compliant".to_string()
            };
            let issues: Vec<String> = result.errors.iter().map(|e| format!("{}", e)).collect();
            StatuteFilingInfo {
                statute_id: statute.id.clone(),
                title: statute.title.clone(),
                effective_date: statute
                    .temporal_validity
                    .effective_date
                    .as_ref()
                    .map(|d| d.format("%Y-%m-%d").to_string()),
                enactment_date: statute
                    .temporal_validity
                    .enacted_at
                    .as_ref()
                    .map(|dt| dt.format("%Y-%m-%d").to_string()),
                status,
                issues,
            }
        })
        .collect();
    let all_compliant = statute_infos.iter().all(|s| s.status == "Compliant");
    let any_critical = statute_infos.iter().any(|s| s.status.contains("Critical"));
    let compliance_status = if all_compliant {
        "Fully Compliant".to_string()
    } else if any_critical {
        "Non-Compliant (Critical Issues)".to_string()
    } else {
        "Partially Compliant".to_string()
    };
    RegulatoryFiling {
        filing_id: filing_id.into(),
        filing_date,
        regulatory_body: regulatory_body.into(),
        filing_type: filing_type.into(),
        jurisdiction: jurisdiction.into(),
        statutes: statute_infos,
        compliance_status,
        documentation_refs: Vec::new(),
    }
}
/// Exports regulatory filing as a formatted report
pub fn regulatory_filing_report(filing: &RegulatoryFiling) -> String {
    let mut report = String::from("# REGULATORY FILING REPORT\n\n");
    report.push_str("---\n\n");
    report.push_str(&format!("**Filing ID**: {}\n\n", filing.filing_id));
    report.push_str(&format!("**Filing Date**: {}\n\n", filing.filing_date));
    report.push_str(&format!(
        "**Regulatory Body**: {}\n\n",
        filing.regulatory_body
    ));
    report.push_str(&format!("**Filing Type**: {}\n\n", filing.filing_type));
    report.push_str(&format!("**Jurisdiction**: {}\n\n", filing.jurisdiction));
    report.push_str(&format!(
        "**Compliance Status**: {}\n\n",
        filing.compliance_status
    ));
    report.push_str("---\n\n");
    report.push_str("## Statutes Included in Filing\n\n");
    for (idx, statute_info) in filing.statutes.iter().enumerate() {
        report.push_str(&format!("### {} - {}\n\n", idx + 1, statute_info.title));
        report.push_str(&format!("**ID**: {}\n\n", statute_info.statute_id));
        report.push_str(&format!("**Status**: {}\n\n", statute_info.status));
        if let Some(ref enactment) = statute_info.enactment_date {
            report.push_str(&format!("**Enactment Date**: {}\n\n", enactment));
        }
        if let Some(ref effective) = statute_info.effective_date {
            report.push_str(&format!("**Effective Date**: {}\n\n", effective));
        }
        if !statute_info.issues.is_empty() {
            report.push_str("**Issues Identified**:\n\n");
            for issue in &statute_info.issues {
                report.push_str(&format!("- {}\n", issue));
            }
            report.push('\n');
        }
    }
    if !filing.documentation_refs.is_empty() {
        report.push_str("## Supporting Documentation\n\n");
        for doc_ref in &filing.documentation_refs {
            report.push_str(&format!("- {}\n", doc_ref));
        }
        report.push('\n');
    }
    report.push_str("---\n\n");
    report.push_str("This filing has been prepared in accordance with applicable regulatory\n");
    report.push_str(
        "requirements and includes all necessary verification and compliance information.\n",
    );
    report
}
/// Generates an executive summary from verification results
pub fn generate_executive_summary(
    title: impl Into<String>,
    statutes: &[Statute],
    result: &VerificationResult,
) -> ExecutiveSummary {
    use chrono::Utc;
    let date = Utc::now().format("%Y-%m-%d").to_string();
    let severity_counts = result.severity_counts();
    let critical_issues = *severity_counts.get(&Severity::Critical).unwrap_or(&0);
    let high_severity = *severity_counts.get(&Severity::Error).unwrap_or(&0);
    let medium_severity = *severity_counts.get(&Severity::Warning).unwrap_or(&0);
    let total_issues = result.errors.len();
    let statutes_with_issues = if total_issues > 0 { statutes.len() } else { 0 };
    let quality_scores: Vec<f64> = statutes
        .iter()
        .map(|s| analyze_quality(s).overall_score)
        .collect();
    let average_quality_score = if !quality_scores.is_empty() {
        quality_scores.iter().sum::<f64>() / quality_scores.len() as f64
    } else {
        0.0
    };
    let statistics = SummaryStatistics {
        total_statutes: statutes.len(),
        statutes_with_issues,
        total_issues,
        critical_issues,
        high_severity_issues: high_severity,
        medium_severity_issues: medium_severity,
        average_quality_score,
    };
    let risk_level = if critical_issues > 0 {
        "Critical".to_string()
    } else if high_severity > 5 {
        "High".to_string()
    } else if high_severity > 0 || medium_severity > 5 {
        "Medium".to_string()
    } else {
        "Low".to_string()
    };
    let mut key_findings = Vec::new();
    if result.passed {
        key_findings.push("All statutes passed verification checks".to_string());
    } else {
        key_findings.push(format!(
            "Found {} total issues across {} statutes",
            total_issues, statutes_with_issues
        ));
    }
    if critical_issues > 0 {
        key_findings.push(format!(
            "{} critical issues requiring immediate attention",
            critical_issues
        ));
    }
    if average_quality_score >= 80.0 {
        key_findings.push(format!(
            "High average quality score: {:.1}/100",
            average_quality_score
        ));
    } else if average_quality_score < 60.0 {
        key_findings.push(format!(
            "Low average quality score: {:.1}/100 - improvement needed",
            average_quality_score
        ));
    }
    let overall_assessment = if critical_issues > 0 {
        "Critical issues detected. Immediate remediation required before deployment.".to_string()
    } else if high_severity > 0 {
        "Significant issues found. Review and remediation recommended.".to_string()
    } else if medium_severity > 0 {
        "Minor issues identified. Consider addressing before final deployment.".to_string()
    } else {
        "No significant issues detected. Statutes are ready for deployment.".to_string()
    };
    let mut recommendations = Vec::new();
    if critical_issues > 0 {
        recommendations.push("Address all critical issues before proceeding".to_string());
    }
    if average_quality_score < 70.0 {
        recommendations.push("Improve statute quality scores through clearer drafting".to_string());
    }
    if !result.suggestions.is_empty() {
        recommendations.push("Review and implement suggested improvements".to_string());
    }
    if recommendations.is_empty() {
        recommendations.push("Continue regular verification checks".to_string());
        recommendations.push("Monitor for any changes requiring re-verification".to_string());
    }
    ExecutiveSummary {
        title: title.into(),
        date,
        key_findings,
        overall_assessment,
        statistics,
        recommendations,
        risk_level,
    }
}
/// Exports executive summary as a formatted report
pub fn executive_summary_report(summary: &ExecutiveSummary) -> String {
    let mut report = String::from("# EXECUTIVE SUMMARY\n\n");
    report.push_str(&format!("## {}\n\n", summary.title));
    report.push_str(&format!("**Date**: {}\n\n", summary.date));
    report.push_str(&format!("**Risk Level**: {}\n\n", summary.risk_level));
    report.push_str("---\n\n");
    report.push_str("## Overall Assessment\n\n");
    report.push_str(&format!("{}\n\n", summary.overall_assessment));
    report.push_str("## Key Findings\n\n");
    for finding in &summary.key_findings {
        report.push_str(&format!("- {}\n", finding));
    }
    report.push('\n');
    report.push_str("## Statistics\n\n");
    let stats = &summary.statistics;
    report.push_str(&format!(
        "- **Total Statutes Analyzed**: {}\n",
        stats.total_statutes
    ));
    report.push_str(&format!(
        "- **Statutes with Issues**: {}\n",
        stats.statutes_with_issues
    ));
    report.push_str(&format!(
        "- **Total Issues Found**: {}\n",
        stats.total_issues
    ));
    report.push_str(&format!(
        "- **Critical Issues**: {}\n",
        stats.critical_issues
    ));
    report.push_str(&format!(
        "- **High Severity Issues**: {}\n",
        stats.high_severity_issues
    ));
    report.push_str(&format!(
        "- **Medium Severity Issues**: {}\n",
        stats.medium_severity_issues
    ));
    report.push_str(&format!(
        "- **Average Quality Score**: {:.1}/100\n\n",
        stats.average_quality_score
    ));
    report.push_str("## Recommendations\n\n");
    for (idx, rec) in summary.recommendations.iter().enumerate() {
        report.push_str(&format!("{}. {}\n", idx + 1, rec));
    }
    report.push('\n');
    report.push_str("---\n\n");
    report.push_str(
        "*This executive summary provides a high-level overview of the verification results.*\n",
    );
    report.push_str("*For detailed findings, please refer to the complete verification report.*\n");
    report
}
/// Generates a custom report based on a template
pub fn generate_custom_report(
    template: &ReportTemplate,
    statutes: &[Statute],
    result: &VerificationResult,
) -> String {
    let mut report = String::new();
    if let Some(ref header) = template.header {
        report.push_str(header);
        report.push_str("\n\n---\n\n");
    }
    if template.include_toc {
        report.push_str("## Table of Contents\n\n");
        for (idx, section) in template.sections.iter().enumerate() {
            let section_name = match section {
                ReportSection::ExecutiveSummary => "Executive Summary",
                ReportSection::VerificationResults => "Verification Results",
                ReportSection::QualityMetrics => "Quality Metrics",
                ReportSection::ComplianceChecklist => "Compliance Checklist",
                ReportSection::ConflictDetection => "Conflict Detection",
                ReportSection::StatisticalAnalysis => "Statistical Analysis",
                ReportSection::AmbiguityDetection => "Ambiguity Detection",
                ReportSection::RegulatoryImpact => "Regulatory Impact Assessment",
                ReportSection::GraphAnalysis => "Graph Analysis",
                ReportSection::Custom { title, .. } => title,
            };
            report.push_str(&format!("{}. {}\n", idx + 1, section_name));
        }
        report.push_str("\n---\n\n");
    }
    for section in &template.sections {
        match section {
            ReportSection::ExecutiveSummary => {
                let summary = generate_executive_summary(&template.name, statutes, result);
                report.push_str(&executive_summary_report(&summary));
                report.push_str("\n---\n\n");
            }
            ReportSection::VerificationResults => {
                report.push_str("# Verification Results\n\n");
                report.push_str(&format!(
                    "**Status**: {}\n\n",
                    if result.passed { "PASSED" } else { "FAILED" }
                ));
                if !result.errors.is_empty() {
                    report.push_str("## Errors\n\n");
                    for (idx, error) in result.errors.iter().enumerate() {
                        report.push_str(&format!(
                            "{}. [{:?}] {}\n",
                            idx + 1,
                            error.severity(),
                            error
                        ));
                    }
                    report.push('\n');
                }
                if !result.warnings.is_empty() {
                    report.push_str("## Warnings\n\n");
                    for (idx, warning) in result.warnings.iter().enumerate() {
                        report.push_str(&format!("{}. {}\n", idx + 1, warning));
                    }
                    report.push('\n');
                }
                report.push_str("---\n\n");
            }
            ReportSection::QualityMetrics => {
                report.push_str(&quality_report(statutes));
                report.push_str("\n---\n\n");
            }
            ReportSection::ComplianceChecklist => {
                report.push_str(&consolidated_compliance_checklist(statutes));
                report.push_str("\n---\n\n");
            }
            ReportSection::ConflictDetection => {
                report.push_str(&conflict_detection_report(statutes));
                report.push_str("\n---\n\n");
            }
            ReportSection::StatisticalAnalysis => {
                report.push_str(&statistics_report(statutes));
                report.push_str("\n---\n\n");
            }
            ReportSection::AmbiguityDetection => {
                report.push_str(&batch_ambiguity_report(statutes));
                report.push_str("\n---\n\n");
            }
            ReportSection::RegulatoryImpact => {
                report.push_str(&regulatory_impact_report(statutes));
                report.push_str("\n---\n\n");
            }
            ReportSection::GraphAnalysis => {
                report.push_str(&graph_analysis_report(statutes));
                report.push_str("\n---\n\n");
            }
            ReportSection::Custom { title, content } => {
                report.push_str(&format!("# {}\n\n", title));
                report.push_str(content);
                report.push_str("\n\n---\n\n");
            }
        }
    }
    if let Some(ref footer) = template.footer {
        report.push_str(footer);
        report.push('\n');
    }
    report
}
/// Creates a standard comprehensive report template
pub fn standard_report_template() -> ReportTemplate {
    ReportTemplate::new("Standard Verification Report")
        .with_header("# Legalis Verification Report")
        .with_toc()
        .with_section(ReportSection::ExecutiveSummary)
        .with_section(ReportSection::VerificationResults)
        .with_section(ReportSection::QualityMetrics)
        .with_section(ReportSection::StatisticalAnalysis)
        .with_footer("Generated by Legalis Verification System")
}
/// Creates a compliance-focused report template
pub fn compliance_report_template() -> ReportTemplate {
    ReportTemplate::new("Compliance Verification Report")
        .with_header("# Compliance Verification Report")
        .with_toc()
        .with_section(ReportSection::ExecutiveSummary)
        .with_section(ReportSection::ComplianceChecklist)
        .with_section(ReportSection::ConflictDetection)
        .with_section(ReportSection::AmbiguityDetection)
        .with_footer("Generated by Legalis Verification System")
}
/// Creates a quality-focused report template
pub fn quality_report_template() -> ReportTemplate {
    ReportTemplate::new("Quality Assessment Report")
        .with_header("# Quality Assessment Report")
        .with_toc()
        .with_section(ReportSection::QualityMetrics)
        .with_section(ReportSection::AmbiguityDetection)
        .with_section(ReportSection::StatisticalAnalysis)
        .with_section(ReportSection::GraphAnalysis)
        .with_footer("Generated by Legalis Verification System")
}
/// Executes a scheduled report generation
///
/// This function generates a report based on the schedule configuration
/// and saves it to the specified output directory.
pub fn execute_scheduled_report(
    schedule: &ReportSchedule,
    statutes: &[Statute],
    result: &VerificationResult,
) -> ScheduledReportResult {
    let execution_time = chrono::Utc::now().to_rfc3339();
    let report_content = generate_custom_report(&schedule.template, statutes, result);
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let extension = match schedule.output_format {
        ReportOutputFormat::Markdown => "md",
        ReportOutputFormat::Html => "html",
        ReportOutputFormat::Json => "json",
        #[cfg(feature = "pdf")]
        ReportOutputFormat::Pdf => "pdf",
    };
    let filename = format!(
        "{}_{}.{}",
        schedule.name.replace(' ', "_"),
        timestamp,
        extension
    );
    let output_path = format!("{}/{}", schedule.output_directory, filename);
    let formatted_content = match schedule.output_format {
        ReportOutputFormat::Markdown => report_content,
        ReportOutputFormat::Html => {
            format!(
                "<!DOCTYPE html>\n<html>\n<head><title>{}</title></head>\n<body>\n<pre>{}</pre>\n</body>\n</html>",
                schedule.name, report_content
            )
        }
        ReportOutputFormat::Json => serde_json::json!(
            { "schedule_id" : schedule.id, "generation_time" : execution_time,
            "report_content" : report_content, "statute_count" : statutes.len(),
            "has_errors" : ! result.errors.is_empty(), "error_count" : result.errors
            .len(), "warning_count" : result.warnings.len(), }
        )
        .to_string(),
        #[cfg(feature = "pdf")]
        ReportOutputFormat::Pdf => report_content,
    };
    match std::fs::create_dir_all(&schedule.output_directory) {
        Ok(_) => match std::fs::write(&output_path, formatted_content.as_bytes()) {
            Ok(_) => {
                let file_size = std::fs::metadata(&output_path).ok().map(|m| m.len());
                ScheduledReportResult {
                    schedule_id: schedule.id.clone(),
                    execution_time,
                    success: true,
                    output_path: Some(output_path),
                    error: None,
                    file_size_bytes: file_size,
                }
            }
            Err(e) => ScheduledReportResult {
                schedule_id: schedule.id.clone(),
                execution_time,
                success: false,
                output_path: None,
                error: Some(format!("Failed to write report file: {}", e)),
                file_size_bytes: None,
            },
        },
        Err(e) => ScheduledReportResult {
            schedule_id: schedule.id.clone(),
            execution_time,
            success: false,
            output_path: None,
            error: Some(format!("Failed to create output directory: {}", e)),
            file_size_bytes: None,
        },
    }
}
/// Creates a daily compliance report schedule
pub fn daily_compliance_schedule() -> ReportSchedule {
    ReportSchedule::new(
        "daily-compliance",
        "Daily Compliance Report",
        compliance_report_template(),
    )
    .with_cron("0 0 * * *")
    .with_format(ReportOutputFormat::Html)
}
/// Creates a weekly quality report schedule
pub fn weekly_quality_schedule() -> ReportSchedule {
    ReportSchedule::new(
        "weekly-quality",
        "Weekly Quality Assessment",
        quality_report_template(),
    )
    .with_cron("0 0 * * 0")
    .with_format(ReportOutputFormat::Markdown)
}
/// Creates a monthly comprehensive report schedule
pub fn monthly_comprehensive_schedule() -> ReportSchedule {
    ReportSchedule::new(
        "monthly-comprehensive",
        "Monthly Comprehensive Report",
        standard_report_template(),
    )
    .with_cron("0 0 1 * *")
    .with_format(ReportOutputFormat::Html)
}
/// Computes overall graph metrics for statute dependencies
pub fn analyze_graph_metrics(statutes: &[Statute]) -> GraphMetrics {
    let node_count = statutes.len();
    let mut edges = 0;
    for statute in statutes {
        let refs = extract_statute_references_from_conditions(&statute.preconditions);
        edges += refs.len();
    }
    let edge_count = edges;
    let average_degree = if node_count > 0 {
        (edge_count as f64) / (node_count as f64)
    } else {
        0.0
    };
    let max_edges = node_count * (node_count - 1);
    let density = if max_edges > 0 {
        (edge_count as f64) / (max_edges as f64)
    } else {
        0.0
    };
    let has_cycle = detect_cycles_in_graph(statutes);
    let is_acyclic = !has_cycle;
    let scc_count = count_strongly_connected_components(statutes);
    let diameter = compute_graph_diameter(statutes);
    GraphMetrics {
        node_count,
        edge_count,
        average_degree,
        density,
        strongly_connected_components: scc_count,
        is_acyclic,
        diameter,
    }
}
/// Detects cycles in the statute dependency graph
fn detect_cycles_in_graph(statutes: &[Statute]) -> bool {
    let mut visited = HashSet::new();
    let mut rec_stack = HashSet::new();
    fn dfs_cycle(
        statute_id: &str,
        statutes: &[Statute],
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
    ) -> bool {
        visited.insert(statute_id.to_string());
        rec_stack.insert(statute_id.to_string());
        if let Some(statute) = statutes.iter().find(|s| s.id == statute_id) {
            let refs = extract_statute_references_from_conditions(&statute.preconditions);
            for ref_id in refs {
                if !visited.contains(&ref_id) {
                    if dfs_cycle(&ref_id, statutes, visited, rec_stack) {
                        return true;
                    }
                } else if rec_stack.contains(&ref_id) {
                    return true;
                }
            }
        }
        rec_stack.remove(statute_id);
        false
    }
    for statute in statutes {
        if !visited.contains(&statute.id)
            && dfs_cycle(&statute.id, statutes, &mut visited, &mut rec_stack)
        {
            return true;
        }
    }
    false
}
/// Counts strongly connected components using Tarjan's algorithm
fn count_strongly_connected_components(statutes: &[Statute]) -> usize {
    if statutes.is_empty() {
        return 0;
    }
    struct TarjanState {
        index: usize,
        stack: Vec<String>,
        indices: HashMap<String, usize>,
        lowlinks: HashMap<String, usize>,
        on_stack: HashSet<String>,
        scc_count: usize,
    }
    fn strongconnect(v: String, statutes: &[Statute], state: &mut TarjanState) {
        state.indices.insert(v.clone(), state.index);
        state.lowlinks.insert(v.clone(), state.index);
        state.index += 1;
        state.stack.push(v.clone());
        state.on_stack.insert(v.clone());
        if let Some(statute) = statutes.iter().find(|s| s.id == v) {
            let refs = extract_statute_references_from_conditions(&statute.preconditions);
            for w in refs {
                if !state.indices.contains_key(&w) {
                    strongconnect(w.clone(), statutes, state);
                    let w_lowlink = *state.lowlinks.get(&w).unwrap_or(&0);
                    let v_lowlink = *state.lowlinks.get(&v).unwrap_or(&0);
                    state.lowlinks.insert(v.clone(), v_lowlink.min(w_lowlink));
                } else if state.on_stack.contains(&w) {
                    let w_index = *state.indices.get(&w).unwrap_or(&0);
                    let v_lowlink = *state.lowlinks.get(&v).unwrap_or(&0);
                    state.lowlinks.insert(v.clone(), v_lowlink.min(w_index));
                }
            }
        }
        if state.lowlinks.get(&v) == state.indices.get(&v) {
            while let Some(w) = state.stack.pop() {
                state.on_stack.remove(&w);
                if w == v {
                    break;
                }
            }
            state.scc_count += 1;
        }
    }
    let mut state = TarjanState {
        index: 0,
        stack: Vec::new(),
        indices: HashMap::new(),
        lowlinks: HashMap::new(),
        on_stack: HashSet::new(),
        scc_count: 0,
    };
    for statute in statutes {
        if !state.indices.contains_key(&statute.id) {
            strongconnect(statute.id.clone(), statutes, &mut state);
        }
    }
    state.scc_count
}
/// Computes graph diameter (longest shortest path)
fn compute_graph_diameter(statutes: &[Statute]) -> usize {
    if statutes.is_empty() {
        return 0;
    }
    let mut max_dist = 0;
    for source in statutes {
        let distances = bfs_distances(&source.id, statutes);
        if let Some(&max) = distances.values().max() {
            max_dist = max_dist.max(max);
        }
    }
    max_dist
}
/// BFS to compute distances from a source statute
fn bfs_distances(source: &str, statutes: &[Statute]) -> HashMap<String, usize> {
    let mut distances = HashMap::new();
    let mut queue = std::collections::VecDeque::new();
    distances.insert(source.to_string(), 0);
    queue.push_back(source.to_string());
    while let Some(current) = queue.pop_front() {
        let current_dist = *distances.get(&current).unwrap_or(&0);
        if let Some(statute) = statutes.iter().find(|s| s.id == current) {
            let refs = extract_statute_references_from_conditions(&statute.preconditions);
            for ref_id in refs {
                if !distances.contains_key(&ref_id) {
                    distances.insert(ref_id.clone(), current_dist + 1);
                    queue.push_back(ref_id);
                }
            }
        }
    }
    distances
}
/// Computes centrality metrics for each statute
pub fn analyze_centrality(statutes: &[Statute]) -> Vec<CentralityMetrics> {
    let mut metrics = Vec::new();
    let mut in_degree: HashMap<String, usize> = HashMap::new();
    let mut out_degree: HashMap<String, usize> = HashMap::new();
    for statute in statutes {
        out_degree.insert(statute.id.clone(), 0);
    }
    for statute in statutes {
        let refs = extract_statute_references_from_conditions(&statute.preconditions);
        *out_degree.get_mut(&statute.id).unwrap() = refs.len();
        for ref_id in refs {
            *in_degree.entry(ref_id).or_insert(0) += 1;
        }
    }
    let pagerank_scores = compute_pagerank(statutes, 0.85, 20);
    let betweenness_scores = compute_betweenness(statutes);
    for statute in statutes {
        let in_deg = *in_degree.get(&statute.id).unwrap_or(&0);
        let out_deg = *out_degree.get(&statute.id).unwrap_or(&0);
        let total_deg = in_deg + out_deg;
        let degree_centrality = if statutes.len() > 1 {
            (total_deg as f64) / ((statutes.len() - 1) as f64)
        } else {
            0.0
        };
        metrics.push(CentralityMetrics {
            statute_id: statute.id.clone(),
            degree_centrality,
            in_degree: in_deg,
            out_degree: out_deg,
            pagerank: *pagerank_scores.get(&statute.id).unwrap_or(&0.0),
            betweenness: *betweenness_scores.get(&statute.id).unwrap_or(&0.0),
        });
    }
    metrics
}
/// Computes PageRank scores for statutes
fn compute_pagerank(statutes: &[Statute], damping: f64, iterations: usize) -> HashMap<String, f64> {
    let n = statutes.len();
    if n == 0 {
        return HashMap::new();
    }
    let mut ranks: HashMap<String, f64> = statutes
        .iter()
        .map(|s| (s.id.clone(), 1.0 / (n as f64)))
        .collect();
    let mut out_degree: HashMap<String, usize> = HashMap::new();
    for statute in statutes {
        let refs = extract_statute_references_from_conditions(&statute.preconditions);
        out_degree.insert(statute.id.clone(), refs.len());
    }
    for _ in 0..iterations {
        let mut new_ranks = HashMap::new();
        for statute in statutes {
            let mut rank_sum = 0.0;
            for other in statutes {
                let refs = extract_statute_references_from_conditions(&other.preconditions);
                if refs.contains(&statute.id) {
                    let other_out = *out_degree.get(&other.id).unwrap_or(&1);
                    if other_out > 0 {
                        rank_sum += ranks.get(&other.id).unwrap_or(&0.0) / (other_out as f64);
                    }
                }
            }
            let new_rank = (1.0 - damping) / (n as f64) + damping * rank_sum;
            new_ranks.insert(statute.id.clone(), new_rank);
        }
        ranks = new_ranks;
    }
    ranks
}
/// Computes betweenness centrality (simplified version)
fn compute_betweenness(statutes: &[Statute]) -> HashMap<String, f64> {
    let n = statutes.len();
    let mut betweenness: HashMap<String, f64> =
        statutes.iter().map(|s| (s.id.clone(), 0.0)).collect();
    if n <= 2 {
        return betweenness;
    }
    for source in statutes {
        for target in statutes {
            if source.id == target.id {
                continue;
            }
            let paths = find_shortest_paths(&source.id, &target.id, statutes);
            if !paths.is_empty() {
                for path in &paths {
                    for statute_id in path {
                        if statute_id != &source.id && statute_id != &target.id {
                            *betweenness.get_mut(statute_id).unwrap() += 1.0 / (paths.len() as f64);
                        }
                    }
                }
            }
        }
    }
    let normalization = if n > 2 {
        ((n - 1) * (n - 2)) as f64
    } else {
        1.0
    };
    for value in betweenness.values_mut() {
        *value /= normalization;
    }
    betweenness
}
/// Finds all shortest paths between two statutes
fn find_shortest_paths(source: &str, target: &str, statutes: &[Statute]) -> Vec<Vec<String>> {
    let mut queue = std::collections::VecDeque::new();
    let mut distances: HashMap<String, usize> = HashMap::new();
    let mut paths: HashMap<String, Vec<Vec<String>>> = HashMap::new();
    distances.insert(source.to_string(), 0);
    paths.insert(source.to_string(), vec![vec![source.to_string()]]);
    queue.push_back(source.to_string());
    while let Some(current) = queue.pop_front() {
        if current == target {
            continue;
        }
        let current_dist = *distances.get(&current).unwrap_or(&0);
        if let Some(statute) = statutes.iter().find(|s| s.id == current) {
            let refs = extract_statute_references_from_conditions(&statute.preconditions);
            for ref_id in refs {
                let new_dist = current_dist + 1;
                if !distances.contains_key(&ref_id) {
                    distances.insert(ref_id.clone(), new_dist);
                    queue.push_back(ref_id.clone());
                    if let Some(current_paths) = paths.get(&current) {
                        let new_paths: Vec<Vec<String>> = current_paths
                            .iter()
                            .map(|path| {
                                let mut new_path = path.clone();
                                new_path.push(ref_id.clone());
                                new_path
                            })
                            .collect();
                        paths.insert(ref_id.clone(), new_paths);
                    }
                } else if distances.get(&ref_id) == Some(&new_dist)
                    && let Some(current_paths) = paths.get(&current).cloned()
                {
                    for path in current_paths {
                        let mut new_path = path.clone();
                        new_path.push(ref_id.clone());
                        paths.entry(ref_id.clone()).or_default().push(new_path);
                    }
                }
            }
        }
    }
    paths.get(target).cloned().unwrap_or_default()
}
/// Detects clusters/communities in the statute graph using simple heuristic
#[allow(dead_code)]
pub fn detect_clusters(statutes: &[Statute]) -> Vec<StatuteCluster> {
    if statutes.is_empty() {
        return Vec::new();
    }
    let mut clusters = Vec::new();
    let mut assigned: HashSet<String> = HashSet::new();
    for statute in statutes {
        if assigned.contains(&statute.id) {
            continue;
        }
        let mut component = HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(statute.id.clone());
        component.insert(statute.id.clone());
        while let Some(current) = queue.pop_front() {
            if let Some(current_statute) = statutes.iter().find(|s| s.id == current) {
                let refs =
                    extract_statute_references_from_conditions(&current_statute.preconditions);
                for ref_id in refs {
                    if !component.contains(&ref_id) {
                        component.insert(ref_id.clone());
                        queue.push_back(ref_id);
                    }
                }
                for other in statutes {
                    let other_refs =
                        extract_statute_references_from_conditions(&other.preconditions);
                    if other_refs.contains(&current) && !component.contains(&other.id) {
                        component.insert(other.id.clone());
                        queue.push_back(other.id.clone());
                    }
                }
            }
        }
        let cluster_statutes: Vec<_> = component.iter().collect();
        let cluster_size = cluster_statutes.len();
        let mut internal_edges = 0;
        for id in &cluster_statutes {
            if let Some(stat) = statutes.iter().find(|s| s.id == **id) {
                let refs = extract_statute_references_from_conditions(&stat.preconditions);
                internal_edges += refs.iter().filter(|r| cluster_statutes.contains(r)).count();
            }
        }
        let max_edges = cluster_size * (cluster_size - 1);
        let density = if max_edges > 0 {
            (internal_edges as f64) / (max_edges as f64)
        } else {
            0.0
        };
        let mut keywords = Vec::new();
        for id in &cluster_statutes {
            if let Some(stat) = statutes.iter().find(|s| s.id == **id) {
                let words: Vec<&str> = stat.title.split_whitespace().collect();
                for word in words {
                    if word.len() > 4 && !keywords.contains(&word.to_string()) {
                        keywords.push(word.to_string());
                    }
                }
            }
        }
        keywords.truncate(5);
        let statute_ids: Vec<String> = component.into_iter().collect();
        assigned.extend(statute_ids.clone());
        clusters.push(StatuteCluster {
            id: clusters.len(),
            statute_ids,
            density,
            keywords,
        });
    }
    clusters
}
/// Generates a comprehensive graph analysis report
pub fn graph_analysis_report(statutes: &[Statute]) -> String {
    let mut report = String::new();
    report.push_str("# Statute Dependency Graph Analysis\n\n");
    report.push_str("## Graph Metrics\n\n");
    let metrics = analyze_graph_metrics(statutes);
    report.push_str(&format!("- **Nodes (Statutes)**: {}\n", metrics.node_count));
    report.push_str(&format!(
        "- **Edges (Dependencies)**: {}\n",
        metrics.edge_count
    ));
    report.push_str(&format!(
        "- **Average Degree**: {:.2}\n",
        metrics.average_degree
    ));
    report.push_str(&format!("- **Graph Density**: {:.4}\n", metrics.density));
    report.push_str(&format!("- **Is Acyclic (DAG)**: {}\n", metrics.is_acyclic));
    report.push_str(&format!(
        "- **Strongly Connected Components**: {}\n",
        metrics.strongly_connected_components
    ));
    report.push_str(&format!(
        "- **Diameter (Longest Path)**: {}\n",
        metrics.diameter
    ));
    report.push('\n');
    report.push_str("## Centrality Metrics\n\n");
    let mut centrality = analyze_centrality(statutes);
    centrality.sort_by(|a, b| {
        b.pagerank
            .partial_cmp(&a.pagerank)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    report.push_str("### Top 10 Statutes by PageRank\n\n");
    for (i, metric) in centrality.iter().take(10).enumerate() {
        report.push_str(&format!(
            "{}. **{}** (PageRank: {:.4}, Degree: {:.2}, In: {}, Out: {})\n",
            i + 1,
            metric.statute_id,
            metric.pagerank,
            metric.degree_centrality,
            metric.in_degree,
            metric.out_degree
        ));
    }
    report.push('\n');
    centrality.sort_by(|a, b| {
        b.betweenness
            .partial_cmp(&a.betweenness)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    report.push_str("### Top 10 Statutes by Betweenness Centrality\n\n");
    for (i, metric) in centrality.iter().take(10).enumerate() {
        if metric.betweenness > 0.0 {
            report.push_str(&format!(
                "{}. **{}** (Betweenness: {:.4})\n",
                i + 1,
                metric.statute_id,
                metric.betweenness
            ));
        }
    }
    report.push('\n');
    report
}
/// Generates an evolution report for tracked statutes
pub fn evolution_report(tracker: &EvolutionTracker) -> String {
    let mut report = String::new();
    report.push_str("# Statute Evolution Report\n\n");
    let all_metrics = tracker.analyze_all_metrics();
    report.push_str(&format!(
        "**Total Tracked Statutes**: {}\n\n",
        all_metrics.len()
    ));
    let total_versions: usize = all_metrics.iter().map(|m| m.total_versions).sum();
    let avg_versions = if !all_metrics.is_empty() {
        total_versions as f64 / all_metrics.len() as f64
    } else {
        0.0
    };
    report.push_str("## Summary Statistics\n\n");
    report.push_str(&format!(
        "- **Total Versions Across All Statutes**: {}\n",
        total_versions
    ));
    report.push_str(&format!(
        "- **Average Versions Per Statute**: {:.2}\n",
        avg_versions
    ));
    report.push('\n');
    report.push_str("## Most Changed Statutes\n\n");
    let most_changed = tracker.most_changed_statutes(10);
    for (i, metric) in most_changed.iter().enumerate() {
        report.push_str(&format!(
            "{}. **{}** - {} versions ({} major, {} minor changes)\n",
            i + 1,
            metric.statute_id,
            metric.total_versions,
            metric.major_changes,
            metric.minor_changes
        ));
    }
    report.push('\n');
    report.push_str("## Most Stable Statutes\n\n");
    let most_stable = tracker.most_stable_statutes(10);
    for (i, metric) in most_stable.iter().enumerate() {
        report.push_str(&format!(
            "{}. **{}** - Stability: {:.2}, {} versions\n",
            i + 1,
            metric.statute_id,
            metric.stability_score,
            metric.total_versions
        ));
    }
    report.push('\n');
    report.push_str("## Complexity Trends\n\n");
    let increasing: Vec<_> = all_metrics
        .iter()
        .filter(|m| m.complexity_trend == ComplexityTrend::Increasing)
        .collect();
    let decreasing: Vec<_> = all_metrics
        .iter()
        .filter(|m| m.complexity_trend == ComplexityTrend::Decreasing)
        .collect();
    let stable: Vec<_> = all_metrics
        .iter()
        .filter(|m| m.complexity_trend == ComplexityTrend::Stable)
        .collect();
    report.push_str(&format!(
        "- **Increasing Complexity**: {} statutes\n",
        increasing.len()
    ));
    report.push_str(&format!(
        "- **Decreasing Complexity**: {} statutes\n",
        decreasing.len()
    ));
    report.push_str(&format!(
        "- **Stable Complexity**: {} statutes\n",
        stable.len()
    ));
    report.push('\n');
    report
}
