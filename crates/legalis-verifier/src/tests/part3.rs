use legalis_core::Statute;
use std::collections::{HashMap, HashSet};

use super::super::types_3::{
    CiConfig, CiPlatform, Clock, ClockConstraint, CtlFormula, NotificationType, PreCommitHook,
    PrincipleCheck, TimedAutomaton, TimedLocation, VerificationRequest, VerificationResponse,
};
use super::super::types_4::{
    CtlStarFormula, CtlStarPathFormula, LtlFormula, ReportSection, TemporalState, TransitionSystem,
};
use super::super::types_5::{
    NotificationConfig, ReportTemplate, TimedTransition, VerificationError, VerificationResult,
};

use super::super::*;
use legalis_core::{ComparisonOp, Condition, Effect, EffectType};

#[test]
fn test_statistics_report() {
    let statutes = vec![
        Statute::new("law1", "Test Law", Effect::grant("benefit"))
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            })
            .with_jurisdiction("US"),
    ];
    let report = statistics_report(&statutes);
    assert!(report.contains("# Statute Collection Statistics"));
    assert!(report.contains("**Total Statutes**: 1"));
    assert!(report.contains("Jurisdiction Distribution"));
}
#[test]
fn test_detect_duplicates_similar() {
    let statutes = vec![
        Statute::new("law1", "Voting Rights Act", Effect::grant("vote")).with_precondition(
            Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            },
        ),
        Statute::new("law2", "Voting Rights Act", Effect::grant("vote")).with_precondition(
            Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            },
        ),
    ];
    let duplicates = detect_duplicates(&statutes, 0.70);
    assert!(!duplicates.is_empty());
    assert!(duplicates[0].similarity_score >= 0.70);
}
#[test]
fn test_detect_duplicates_no_similarity() {
    let statutes = vec![
        Statute::new("law1", "Voting Rights", Effect::grant("vote")).with_precondition(
            Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            },
        ),
        Statute::new("law2", "Tax Code", Effect::obligation("pay_tax")).with_precondition(
            Condition::Income {
                operator: ComparisonOp::GreaterThan,
                value: 50000,
            },
        ),
    ];
    let duplicates = detect_duplicates(&statutes, 0.90);
    assert!(duplicates.is_empty());
}
#[test]
fn test_duplicate_detection_report() {
    let statutes = vec![
        Statute::new("law1", "Test Law", Effect::grant("benefit")).with_precondition(
            Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            },
        ),
    ];
    let report = duplicate_detection_report(&statutes, 0.70);
    assert!(report.contains("# Duplicate Detection Report"));
    assert!(report.contains("Minimum Similarity Threshold"));
}
#[test]
fn test_regulatory_impact_basic() {
    let statute = Statute::new(
        "test-law",
        "Test Statute",
        Effect::new(EffectType::Prohibition, "Prohibited action"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 18,
    })
    .with_precondition(Condition::Income {
        operator: ComparisonOp::LessThan,
        value: 50000,
    });
    let impact = analyze_regulatory_impact(&statute);
    assert_eq!(impact.statute_id, "test-law");
    assert!(impact.impact_score > 0);
    assert!(impact.impact_score <= 100);
    assert!(!impact.impact_level.is_empty());
}
#[test]
fn test_regulatory_impact_high() {
    let mut statute = Statute::new(
        "complex-law",
        "Complex Statute",
        Effect::new(EffectType::Prohibition, "Complex prohibition"),
    );
    for i in 0..10 {
        statute = statute.with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18 + i,
        });
    }
    let impact = analyze_regulatory_impact(&statute);
    assert!(impact.impact_score >= 50);
    assert!(impact.impact_level.contains("Impact"));
}
#[test]
fn test_regulatory_impact_report() {
    let statutes = vec![
        Statute::new("law1", "Law 1", Effect::grant("benefit")).with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        }),
        Statute::new(
            "law2",
            "Law 2",
            Effect::new(EffectType::Prohibition, "Action"),
        ),
    ];
    let report = regulatory_impact_report(&statutes);
    assert!(report.contains("# Regulatory Impact Assessment"));
    assert!(report.contains("Summary"));
    assert!(report.contains("law1"));
    assert!(report.contains("law2"));
    assert!(report.contains("Impact Score"));
}
#[test]
fn test_generate_compliance_checklist() {
    let statute = Statute::new("test-law", "Test Law", Effect::grant("benefit"))
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        })
        .with_precondition(Condition::Income {
            operator: ComparisonOp::LessThan,
            value: 50000,
        })
        .with_discretion("Optional discretion");
    let checklist = generate_compliance_checklist(&statute);
    assert!(checklist.len() >= 4);
    assert!(checklist.iter().any(|item| item.priority == "Required"));
    assert!(checklist.iter().any(|item| item.priority == "Optional"));
}
#[test]
fn test_compliance_checklist_report() {
    let statute = Statute::new("test-law", "Test Law", Effect::grant("benefit"))
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        })
        .with_jurisdiction("US");
    let report = compliance_checklist_report(&statute);
    assert!(report.contains("# Compliance Checklist"));
    assert!(report.contains("test-law"));
    assert!(report.contains("Test Law"));
    assert!(report.contains("US"));
    assert!(report.contains("[ ]"));
}
#[test]
fn test_consolidated_compliance_checklist() {
    let statutes = vec![
        Statute::new("law1", "First Law", Effect::grant("benefit")).with_precondition(
            Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            },
        ),
        Statute::new("law2", "Second Law", Effect::grant("license")).with_precondition(
            Condition::Income {
                operator: ComparisonOp::GreaterThan,
                value: 30000,
            },
        ),
    ];
    let report = consolidated_compliance_checklist(&statutes);
    assert!(report.contains("# Consolidated Compliance Checklist"));
    assert!(report.contains("**Total Statutes**: 2"));
    assert!(report.contains("law1"));
    assert!(report.contains("law2"));
}
#[test]
fn test_generate_compliance_certification() {
    let statutes = vec![
        Statute::new("law1", "Test Law 1", Effect::grant("benefit")).with_precondition(
            Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            },
        ),
        Statute::new("law2", "Test Law 2", Effect::grant("license")).with_precondition(
            Condition::Income {
                operator: ComparisonOp::GreaterThan,
                value: 30000,
            },
        ),
    ];
    let result = VerificationResult::pass();
    let cert = generate_compliance_certification(
        "CERT-2025-001",
        "Test Organization",
        "Legalis Certifying Authority",
        &statutes,
        &result,
        Some(365),
    );
    assert_eq!(cert.certificate_id, "CERT-2025-001");
    assert_eq!(cert.organization, "Test Organization");
    assert_eq!(cert.certifying_authority, "Legalis Certifying Authority");
    assert_eq!(cert.statute_ids.len(), 2);
    assert_eq!(cert.verification_summary.total_statutes, 2);
    assert_eq!(cert.verification_summary.passed_count, 2);
    assert_eq!(cert.verification_summary.failed_count, 0);
    assert_eq!(cert.verification_summary.pass_rate, 100.0);
    assert!(cert.valid_until.is_some());
}
#[test]
fn test_compliance_certification_report() {
    let statutes = vec![Statute::new("law1", "Test Law", Effect::grant("benefit"))];
    let result = VerificationResult::pass();
    let cert = generate_compliance_certification(
        "CERT-TEST",
        "Org",
        "Authority",
        &statutes,
        &result,
        None,
    );
    let report = compliance_certification_report(&cert);
    assert!(report.contains("# COMPLIANCE CERTIFICATION"));
    assert!(report.contains("CERT-TEST"));
    assert!(report.contains("Org"));
    assert!(report.contains("Authority"));
    assert!(report.contains("Verification Summary"));
    assert!(report.contains("law1"));
}
#[test]
fn test_generate_regulatory_filing() {
    let statutes = vec![
        Statute::new("law1", "Test Law 1", Effect::grant("benefit")).with_jurisdiction("US"),
        Statute::new("law2", "Test Law 2", Effect::prohibition("action")).with_jurisdiction("US"),
    ];
    let results = vec![
        VerificationResult::pass(),
        VerificationResult::fail(vec![VerificationError::Ambiguity {
            message: "Test error".to_string(),
        }]),
    ];
    let filing = generate_regulatory_filing(
        "FILING-2025-001",
        "Federal Regulatory Commission",
        "Annual Compliance",
        "US",
        &statutes,
        &results,
    );
    assert_eq!(filing.filing_id, "FILING-2025-001");
    assert_eq!(filing.regulatory_body, "Federal Regulatory Commission");
    assert_eq!(filing.filing_type, "Annual Compliance");
    assert_eq!(filing.jurisdiction, "US");
    assert_eq!(filing.statutes.len(), 2);
    assert_eq!(filing.statutes[0].status, "Compliant");
    assert_eq!(filing.statutes[1].status, "Non-Compliant");
    assert_eq!(filing.compliance_status, "Partially Compliant");
}
#[test]
fn test_regulatory_filing_report() {
    let statutes = vec![Statute::new("law1", "Test Law", Effect::grant("benefit"))];
    let results = vec![VerificationResult::pass()];
    let filing = generate_regulatory_filing(
        "FILING-TEST",
        "Test Body",
        "Test Type",
        "Test Jurisdiction",
        &statutes,
        &results,
    );
    let report = regulatory_filing_report(&filing);
    assert!(report.contains("# REGULATORY FILING REPORT"));
    assert!(report.contains("FILING-TEST"));
    assert!(report.contains("Test Body"));
    assert!(report.contains("Test Type"));
    assert!(report.contains("Test Jurisdiction"));
    assert!(report.contains("Fully Compliant"));
}
#[test]
fn test_generate_executive_summary() {
    let statutes = vec![
        Statute::new("law1", "Test Law 1", Effect::grant("benefit")).with_precondition(
            Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            },
        ),
        Statute::new("law2", "Test Law 2", Effect::grant("license")).with_precondition(
            Condition::Income {
                operator: ComparisonOp::GreaterThan,
                value: 30000,
            },
        ),
    ];
    let result = VerificationResult::pass();
    let summary = generate_executive_summary("Test Verification", &statutes, &result);
    assert_eq!(summary.title, "Test Verification");
    assert!(!summary.date.is_empty());
    assert_eq!(summary.risk_level, "Low");
    assert_eq!(summary.statistics.total_statutes, 2);
    assert_eq!(summary.statistics.statutes_with_issues, 0);
    assert_eq!(summary.statistics.total_issues, 0);
    assert!(!summary.key_findings.is_empty());
    assert!(!summary.recommendations.is_empty());
}
#[test]
fn test_executive_summary_with_errors() {
    let statutes = vec![Statute::new("law1", "Test Law", Effect::grant("benefit"))];
    let result = VerificationResult::fail(vec![VerificationError::CircularReference {
        message: "Test error".to_string(),
    }]);
    let summary = generate_executive_summary("Test", &statutes, &result);
    assert_eq!(summary.risk_level, "Critical");
    assert_eq!(summary.statistics.critical_issues, 1);
    assert!(summary.overall_assessment.contains("Critical"));
}
#[test]
fn test_executive_summary_report() {
    let statutes = vec![Statute::new("law1", "Test Law", Effect::grant("benefit"))];
    let result = VerificationResult::pass();
    let summary = generate_executive_summary("Test", &statutes, &result);
    let report = executive_summary_report(&summary);
    assert!(report.contains("# EXECUTIVE SUMMARY"));
    assert!(report.contains("Test"));
    assert!(report.contains("Risk Level"));
    assert!(report.contains("Overall Assessment"));
    assert!(report.contains("Key Findings"));
    assert!(report.contains("Statistics"));
    assert!(report.contains("Recommendations"));
}
#[test]
fn test_report_template_creation() {
    let template = ReportTemplate::new("Test Template")
        .with_header("# Test Header")
        .with_footer("Test Footer")
        .with_toc()
        .with_section(ReportSection::ExecutiveSummary)
        .with_section(ReportSection::VerificationResults);
    assert_eq!(template.name, "Test Template");
    assert_eq!(template.header, Some("# Test Header".to_string()));
    assert_eq!(template.footer, Some("Test Footer".to_string()));
    assert!(template.include_toc);
    assert_eq!(template.sections.len(), 2);
}
#[test]
fn test_generate_custom_report() {
    let statutes = vec![
        Statute::new("law1", "Test Law", Effect::grant("benefit")).with_precondition(
            Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            },
        ),
    ];
    let result = VerificationResult::pass();
    let template = ReportTemplate::new("Custom Report")
        .with_header("# Custom Header")
        .with_section(ReportSection::ExecutiveSummary)
        .with_section(ReportSection::VerificationResults)
        .with_footer("Custom Footer");
    let report = generate_custom_report(&template, &statutes, &result);
    assert!(report.contains("# Custom Header"));
    assert!(report.contains("Custom Footer"));
    assert!(report.contains("# EXECUTIVE SUMMARY"));
    assert!(report.contains("# Verification Results"));
}
#[test]
fn test_standard_report_template() {
    let template = standard_report_template();
    assert_eq!(template.name, "Standard Verification Report");
    assert!(template.include_toc);
    assert!(!template.sections.is_empty());
    assert!(template.header.is_some());
    assert!(template.footer.is_some());
}
#[test]
fn test_compliance_report_template() {
    let template = compliance_report_template();
    assert_eq!(template.name, "Compliance Verification Report");
    assert!(template.include_toc);
    assert!(!template.sections.is_empty());
}
#[test]
fn test_quality_report_template() {
    let template = quality_report_template();
    assert_eq!(template.name, "Quality Assessment Report");
    assert!(template.include_toc);
    assert!(!template.sections.is_empty());
}
#[test]
fn test_custom_report_with_all_sections() {
    let statutes = vec![
        Statute::new("law1", "Test Law", Effect::grant("benefit"))
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            })
            .with_jurisdiction("US"),
    ];
    let result = VerificationResult::pass();
    let template = ReportTemplate::new("Comprehensive Test")
        .with_toc()
        .with_section(ReportSection::ExecutiveSummary)
        .with_section(ReportSection::VerificationResults)
        .with_section(ReportSection::QualityMetrics)
        .with_section(ReportSection::ComplianceChecklist)
        .with_section(ReportSection::StatisticalAnalysis);
    let report = generate_custom_report(&template, &statutes, &result);
    assert!(report.contains("Table of Contents"));
    assert!(report.contains("Executive Summary"));
    assert!(report.contains("Verification Results"));
    assert!(report.contains("Quality"));
    assert!(report.contains("Compliance"));
    assert!(report.contains("Statistics"));
}
#[test]
fn test_custom_report_section() {
    let statutes = vec![Statute::new("law1", "Test Law", Effect::grant("benefit"))];
    let result = VerificationResult::pass();
    let template = ReportTemplate::new("Custom Section Test").with_section(ReportSection::Custom {
        title: "Custom Section Title".to_string(),
        content: "This is custom content for testing.".to_string(),
    });
    let report = generate_custom_report(&template, &statutes, &result);
    assert!(report.contains("# Custom Section Title"));
    assert!(report.contains("This is custom content for testing."));
}
#[test]
fn test_ctl_star_basic_formula() {
    let mut system = TransitionSystem::new();
    let s0 = TemporalState::new("s0").with_proposition("p");
    let s1 = TemporalState::new("s1").with_proposition("q");
    system.add_state(s0);
    system.add_state(s1);
    system.add_transition("s0", "s1");
    system.add_initial_state("s0");
    let formula = CtlStarFormula::exists(CtlStarPathFormula::eventually(
        CtlStarPathFormula::state(CtlStarFormula::atom("q")),
    ));
    assert!(verify_ctl_star(&system, &formula));
}
#[test]
fn test_ctl_star_all_paths() {
    let mut system = TransitionSystem::new();
    let s0 = TemporalState::new("s0").with_proposition("p");
    let s1 = TemporalState::new("s1").with_proposition("p");
    let s2 = TemporalState::new("s2").with_proposition("p");
    system.add_state(s0);
    system.add_state(s1);
    system.add_state(s2);
    system.add_transition("s0", "s1");
    system.add_transition("s0", "s2");
    system.add_initial_state("s0");
    let formula = CtlStarFormula::all(CtlStarPathFormula::next(CtlStarPathFormula::state(
        CtlStarFormula::atom("p"),
    )));
    assert!(verify_ctl_star(&system, &formula));
}
#[test]
fn test_ctl_star_display() {
    let formula = CtlStarFormula::exists(CtlStarPathFormula::eventually(
        CtlStarPathFormula::state(CtlStarFormula::atom("p")),
    ));
    let display = format!("{}", formula);
    assert!(display.contains("E"));
    assert!(display.contains("F"));
    assert!(display.contains("p"));
}
#[test]
fn test_ctl_star_path_formula_display() {
    let path = CtlStarPathFormula::until(
        CtlStarPathFormula::state(CtlStarFormula::atom("p")),
        CtlStarPathFormula::state(CtlStarFormula::atom("q")),
    );
    let display = format!("{}", path);
    assert!(display.contains("U"));
    assert!(display.contains("p"));
    assert!(display.contains("q"));
}
#[test]
fn test_ctl_star_complex_formula() {
    let mut system = TransitionSystem::new();
    let s0 = TemporalState::new("s0").with_proposition("p");
    let s1 = TemporalState::new("s1")
        .with_proposition("p")
        .with_proposition("q");
    let s2 = TemporalState::new("s2").with_proposition("q");
    system.add_state(s0);
    system.add_state(s1);
    system.add_state(s2);
    system.add_transition("s0", "s1");
    system.add_transition("s1", "s2");
    system.add_initial_state("s0");
    let formula = CtlStarFormula::exists(CtlStarPathFormula::until(
        CtlStarPathFormula::state(CtlStarFormula::atom("p")),
        CtlStarPathFormula::state(CtlStarFormula::atom("q")),
    ));
    assert!(verify_ctl_star(&system, &formula));
}
#[test]
fn test_ctl_star_always_path_formula() {
    let mut system = TransitionSystem::new();
    let s0 = TemporalState::new("s0").with_proposition("p");
    let s1 = TemporalState::new("s1").with_proposition("p");
    system.add_state(s0);
    system.add_state(s1);
    system.add_transition("s0", "s1");
    system.add_transition("s1", "s1");
    system.add_initial_state("s0");
    let formula = CtlStarFormula::exists(CtlStarPathFormula::always(CtlStarPathFormula::state(
        CtlStarFormula::atom("p"),
    )));
    assert!(verify_ctl_star(&system, &formula));
}
#[test]
fn test_clock_creation() {
    let clock = Clock::new("x");
    assert_eq!(clock.name, "x");
}
#[test]
fn test_clock_constraint_satisfied() {
    let clock = Clock::new("x");
    let mut valuations = HashMap::new();
    valuations.insert("x".to_string(), 5);
    let constraint = ClockConstraint::Less(clock.clone(), 10);
    assert!(constraint.satisfied(&valuations));
    let constraint2 = ClockConstraint::Greater(clock, 10);
    assert!(!constraint2.satisfied(&valuations));
}
#[test]
fn test_clock_constraint_equal() {
    let clock = Clock::new("x");
    let mut valuations = HashMap::new();
    valuations.insert("x".to_string(), 5);
    let constraint = ClockConstraint::Equal(clock, 5);
    assert!(constraint.satisfied(&valuations));
}
#[test]
fn test_clock_constraint_and() {
    let clock1 = Clock::new("x");
    let clock2 = Clock::new("y");
    let mut valuations = HashMap::new();
    valuations.insert("x".to_string(), 5);
    valuations.insert("y".to_string(), 10);
    let constraint = ClockConstraint::And(
        Box::new(ClockConstraint::Greater(clock1, 3)),
        Box::new(ClockConstraint::Less(clock2, 15)),
    );
    assert!(constraint.satisfied(&valuations));
}
#[test]
fn test_timed_location_creation() {
    let location = TimedLocation::new("l0").accepting();
    assert_eq!(location.id, "l0");
    assert!(location.accepting);
    assert!(location.invariant.is_none());
}
#[test]
fn test_timed_location_with_invariant() {
    let clock = Clock::new("x");
    let invariant = ClockConstraint::Less(clock, 10);
    let location = TimedLocation::new("l0").with_invariant(invariant);
    assert!(location.invariant.is_some());
}
#[test]
fn test_timed_transition_creation() {
    let transition = TimedTransition::new("l0", "l1", "action");
    assert_eq!(transition.from, "l0");
    assert_eq!(transition.to, "l1");
    assert_eq!(transition.action, "action");
    assert!(transition.guard.is_none());
    assert!(transition.resets.is_empty());
}
#[test]
fn test_timed_transition_with_guard_and_reset() {
    let clock = Clock::new("x");
    let guard = ClockConstraint::Greater(clock.clone(), 5);
    let transition = TimedTransition::new("l0", "l1", "action")
        .with_guard(guard)
        .with_reset(clock);
    assert!(transition.guard.is_some());
    assert_eq!(transition.resets.len(), 1);
}
#[test]
fn test_timed_automaton_creation() {
    let mut automaton = TimedAutomaton::new("l0");
    let clock = Clock::new("x");
    automaton.add_clock(clock);
    let location = TimedLocation::new("l0");
    automaton.add_location(location);
    assert_eq!(automaton.initial, "l0");
    assert_eq!(automaton.clocks.len(), 1);
    assert_eq!(automaton.locations.len(), 1);
}
#[test]
fn test_timed_reachability_simple() {
    let mut automaton = TimedAutomaton::new("l0");
    let clock = Clock::new("x");
    automaton.add_clock(clock.clone());
    let l0 = TimedLocation::new("l0");
    let l1 = TimedLocation::new("l1").accepting();
    automaton.add_location(l0);
    automaton.add_location(l1);
    let transition = TimedTransition::new("l0", "l1", "action");
    automaton.add_transition(transition);
    assert!(verify_timed_reachability(&automaton, 100));
}
#[test]
fn test_timed_reachability_with_reset() {
    let mut automaton = TimedAutomaton::new("l0");
    let clock = Clock::new("x");
    automaton.add_clock(clock.clone());
    let l0 = TimedLocation::new("l0");
    let l1 = TimedLocation::new("l1").accepting();
    automaton.add_location(l0);
    automaton.add_location(l1);
    let transition = TimedTransition::new("l0", "l1", "action").with_reset(clock);
    automaton.add_transition(transition);
    assert!(verify_timed_reachability(&automaton, 100));
}
#[test]
fn test_timed_reachability_unreachable() {
    let mut automaton = TimedAutomaton::new("l0");
    let clock = Clock::new("x");
    automaton.add_clock(clock.clone());
    let l0 = TimedLocation::new("l0");
    let l1 = TimedLocation::new("l1").accepting();
    automaton.add_location(l0);
    automaton.add_location(l1);
    assert!(!verify_timed_reachability(&automaton, 100));
}
#[test]
fn test_synthesize_ltl_always() {
    let mut state1 = HashSet::new();
    state1.insert("p".to_string());
    let mut state2 = HashSet::new();
    state2.insert("p".to_string());
    let positive_traces = vec![vec![state1.clone(), state2.clone()]];
    let mut state3 = HashSet::new();
    state3.insert("q".to_string());
    let negative_traces = vec![vec![state3]];
    let formula = synthesize_ltl_property(&positive_traces, &negative_traces);
    assert!(formula.is_some());
    let formula = formula.unwrap();
    assert!(matches!(formula, LtlFormula::Always(_)));
}
#[test]
fn test_synthesize_ltl_eventually() {
    let mut state1 = HashSet::new();
    state1.insert("p".to_string());
    let mut state2 = HashSet::new();
    state2.insert("q".to_string());
    let positive_traces = vec![vec![state1.clone(), state2.clone()]];
    let mut state3 = HashSet::new();
    state3.insert("p".to_string());
    let negative_traces = vec![vec![state3.clone(), state3]];
    let formula = synthesize_ltl_property(&positive_traces, &negative_traces);
    assert!(formula.is_some());
}
#[test]
fn test_synthesize_ltl_empty_traces() {
    let positive_traces: Vec<Vec<HashSet<String>>> = vec![];
    let negative_traces: Vec<Vec<HashSet<String>>> = vec![];
    let formula = synthesize_ltl_property(&positive_traces, &negative_traces);
    assert!(formula.is_none());
}
#[test]
fn test_synthesize_ltl_no_separation() {
    let mut state1 = HashSet::new();
    state1.insert("p".to_string());
    let positive_traces = vec![vec![state1.clone()]];
    let negative_traces = vec![vec![state1]];
    let formula = synthesize_ltl_property(&positive_traces, &negative_traces);
    assert!(formula.is_none());
}
#[test]
fn test_synthesize_ctl_exists_eventually() {
    let mut system = TransitionSystem::new();
    let s0 = TemporalState::new("s0").with_proposition("p");
    let s1 = TemporalState::new("s1").with_proposition("q");
    system.add_state(s0);
    system.add_state(s1);
    system.add_transition("s0", "s1");
    system.add_initial_state("s0");
    let desired_properties = vec!["q".to_string()];
    let formula = synthesize_ctl_property(&system, &desired_properties);
    assert!(formula.is_some());
    assert!(matches!(formula.unwrap(), CtlFormula::ExistsEventually(_)));
}
#[test]
fn test_synthesize_ctl_all_always() {
    let mut system = TransitionSystem::new();
    let s0 = TemporalState::new("s0").with_proposition("p");
    let s1 = TemporalState::new("s1").with_proposition("p");
    system.add_state(s0);
    system.add_state(s1);
    system.add_transition("s0", "s1");
    system.add_transition("s1", "s1");
    system.add_initial_state("s0");
    let desired_properties = vec!["p".to_string()];
    let formula = synthesize_ctl_property(&system, &desired_properties);
    assert!(formula.is_some());
}
#[test]
fn test_synthesize_ctl_empty_properties() {
    let system = TransitionSystem::new();
    let desired_properties: Vec<String> = vec![];
    let formula = synthesize_ctl_property(&system, &desired_properties);
    assert!(formula.is_none());
}
#[test]
fn test_check_formula_on_trace() {
    let mut state1 = HashSet::new();
    state1.insert("p".to_string());
    let mut state2 = HashSet::new();
    state2.insert("q".to_string());
    let trace = vec![state1, state2];
    let formula = LtlFormula::eventually(LtlFormula::atom("q"));
    assert!(check_formula_on_trace(&formula, &trace));
    let formula2 = LtlFormula::always(LtlFormula::atom("p"));
    assert!(!check_formula_on_trace(&formula2, &trace));
}
#[test]
fn test_ci_platform_display() {
    assert_eq!(CiPlatform::GitHubActions.to_string(), "GitHub Actions");
    assert_eq!(CiPlatform::GitLabCI.to_string(), "GitLab CI");
    assert_eq!(CiPlatform::Jenkins.to_string(), "Jenkins");
    assert_eq!(CiPlatform::CircleCI.to_string(), "CircleCI");
    assert_eq!(CiPlatform::TravisCI.to_string(), "Travis CI");
}
#[test]
fn test_ci_config_creation() {
    let config = CiConfig::new(CiPlatform::GitHubActions);
    assert_eq!(config.platform, CiPlatform::GitHubActions);
    assert!(config.fail_on_warnings);
    assert!(config.upload_reports);
    assert_eq!(config.report_dir, "verification-reports");
}
#[test]
fn test_ci_config_builder() {
    let config = CiConfig::new(CiPlatform::GitLabCI)
        .with_command("custom-verify-cmd")
        .fail_on_warnings(false)
        .upload_reports(false)
        .with_report_dir("custom-reports");
    assert_eq!(config.verify_command, "custom-verify-cmd");
    assert!(!config.fail_on_warnings);
    assert!(!config.upload_reports);
    assert_eq!(config.report_dir, "custom-reports");
}
#[test]
fn test_ci_config_github_actions() {
    let config = CiConfig::new(CiPlatform::GitHubActions);
    let output = config.generate();
    assert!(output.contains("name: Statute Verification"));
    assert!(output.contains("actions/checkout"));
    assert!(output.contains("cargo run --bin legalis-verify"));
    assert!(output.contains("upload-artifact"));
}
#[test]
fn test_ci_config_gitlab_ci() {
    let config = CiConfig::new(CiPlatform::GitLabCI);
    let output = config.generate();
    assert!(output.contains("verify:"));
    assert!(output.contains("stage: test"));
    assert!(output.contains("artifacts:"));
}
#[test]
fn test_ci_config_jenkins() {
    let config = CiConfig::new(CiPlatform::Jenkins);
    let output = config.generate();
    assert!(output.contains("pipeline"));
    assert!(output.contains("stage('Verify Statutes')"));
    assert!(output.contains("archiveArtifacts"));
}
#[test]
fn test_ci_config_circleci() {
    let config = CiConfig::new(CiPlatform::CircleCI);
    let output = config.generate();
    assert!(output.contains("version: 2.1"));
    assert!(output.contains("jobs:"));
    assert!(output.contains("store_artifacts"));
}
#[test]
fn test_ci_config_travis() {
    let config = CiConfig::new(CiPlatform::TravisCI);
    let output = config.generate();
    assert!(output.contains("language: rust"));
    assert!(output.contains("script:"));
}
#[test]
fn test_precommit_hook_creation() {
    let hook = PreCommitHook::new();
    assert!(hook.fail_on_errors);
    assert!(!hook.fail_on_warnings);
    assert!(hook.verbose);
}
#[test]
fn test_precommit_hook_builder() {
    let hook = PreCommitHook::new()
        .with_command("custom-verify")
        .fail_on_errors(false)
        .fail_on_warnings(true)
        .verbose(false);
    assert_eq!(hook.verify_command, "custom-verify");
    assert!(!hook.fail_on_errors);
    assert!(hook.fail_on_warnings);
    assert!(!hook.verbose);
}
#[test]
fn test_precommit_hook_generation() {
    let hook = PreCommitHook::new();
    let script = hook.generate();
    assert!(script.contains("#!/bin/bash"));
    assert!(script.contains("Running statute verification"));
    assert!(script.contains("cargo run --bin legalis-verify"));
    assert!(script.contains("VERIFICATION_EXIT_CODE"));
}
#[test]
fn test_precommit_hook_default() {
    let hook = PreCommitHook::default();
    assert!(hook.fail_on_errors);
}
#[test]
fn test_verification_request_creation() {
    let statutes = vec![Statute::new("law1", "Test Law", Effect::grant("benefit"))];
    let request = VerificationRequest::new(statutes.clone());
    assert_eq!(request.statutes.len(), 1);
    assert!(request.principles.is_empty());
    assert!(request.request_id.is_none());
    assert!(request.client_id.is_none());
}
#[test]
fn test_verification_request_builder() {
    let statutes = vec![Statute::new("law1", "Test Law", Effect::grant("benefit"))];
    let principles = vec![PrincipleCheck::NoDiscrimination];
    let request = VerificationRequest::new(statutes)
        .with_principles(principles.clone())
        .with_request_id("req-123")
        .with_client_id("client-456");
    assert_eq!(request.request_id, Some("req-123".to_string()));
    assert_eq!(request.client_id, Some("client-456".to_string()));
    assert_eq!(request.principles.len(), 1);
}
#[test]
fn test_verification_response_creation() {
    let results = vec![VerificationResult::pass(), VerificationResult::pass()];
    let response = VerificationResponse::new(Some("req-123".to_string()), results);
    assert_eq!(response.request_id, Some("req-123".to_string()));
    assert_eq!(response.results.len(), 2);
    assert!(response.success);
    assert_eq!(response.error_count, 0);
    assert_eq!(response.warning_count, 0);
}
#[test]
fn test_verification_response_with_errors() {
    let result = VerificationResult::fail(vec![VerificationError::DeadStatute {
        statute_id: "dead_law".to_string(),
    }]);
    let results = vec![result];
    let response = VerificationResponse::new(None, results);
    assert!(!response.success);
    assert_eq!(response.error_count, 1);
}
#[test]
fn test_verification_response_processing_time() {
    let results = vec![VerificationResult::pass()];
    let response = VerificationResponse::new(None, results).with_processing_time(150);
    assert_eq!(response.processing_time_ms, 150);
}
#[test]
fn test_notification_config_creation() {
    let config = NotificationConfig::new();
    assert!(config.channels.is_empty());
    assert_eq!(config.trigger_on.len(), 2);
    assert!(config.trigger_on.contains(&NotificationType::Error));
    assert!(config.trigger_on.contains(&NotificationType::Critical));
    assert!(config.include_details);
}
