use legalis_core::Statute;

use super::super::types::{
    BatchVerificationResult, QualityMetrics, StatuteConflict, StatuteVerifier,
};
use super::super::types_3::{AmbiguityType, ConflictType};
use super::super::types_4::Severity;
use super::super::types_5::{StatuteChange, VerificationError, VerificationResult};

use super::super::*;
use legalis_core::{ComparisonOp, Condition, Effect, EffectType, TemporalValidity};

#[test]
fn test_retroactivity_check_retroactive_language() {
    use chrono::NaiveDate;
    let statute = Statute::new(
        "test-2",
        "Retroactive ban",
        Effect::new(
            EffectType::Prohibition,
            "Prohibit actions taken retroactively before this date",
        ),
    )
    .with_temporal_validity(
        TemporalValidity::new().with_effective_date(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()),
    );
    let result = check_retroactivity(&statute);
    assert!(!result.passed);
    assert!(!result.issues.is_empty());
    assert!(result.issues.iter().any(|i| i.contains("retroactively")));
}
#[test]
fn test_retroactivity_check_retroactive_parameter() {
    use chrono::NaiveDate;
    let mut effect = Effect::new(EffectType::Obligation, "File report");
    effect
        .parameters
        .insert("retroactive".to_string(), "true".to_string());
    let statute = Statute::new("test-3", "Reporting requirement", effect).with_temporal_validity(
        TemporalValidity::new().with_effective_date(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()),
    );
    let result = check_retroactivity(&statute);
    assert!(!result.passed);
    assert!(!result.issues.is_empty());
    assert!(result.issues.iter().any(|i| i.contains("ex post facto")));
}
#[test]
fn test_retroactivity_check_application_before_effective() {
    use chrono::NaiveDate;
    let mut effect = Effect::new(EffectType::Prohibition, "Prohibit conduct");
    effect
        .parameters
        .insert("application_date".to_string(), "2024-12-01".to_string());
    let statute = Statute::new("test-4", "Backdated prohibition", effect).with_temporal_validity(
        TemporalValidity::new().with_effective_date(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()),
    );
    let result = check_retroactivity(&statute);
    assert!(!result.passed);
    assert!(!result.issues.is_empty());
    assert!(
        result
            .issues
            .iter()
            .any(|i| i.contains("precedes effective date"))
    );
}
#[test]
fn test_retroactivity_check_effective_before_enactment() {
    use chrono::{NaiveDate, Utc};
    let statute = Statute::new(
        "test-5",
        "Impossible retroactive law",
        Effect::new(EffectType::Prohibition, "Prohibit action"),
    )
    .with_temporal_validity(
        TemporalValidity::new()
            .with_effective_date(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())
            .with_enacted_at(Utc::now()),
    );
    let result = check_retroactivity(&statute);
    assert!(!result.passed);
    assert!(!result.issues.is_empty());
    assert!(
        result
            .issues
            .iter()
            .any(|i| i.contains("before enactment date"))
    );
}
#[test]
fn test_retroactivity_check_monetary_penalty() {
    use chrono::NaiveDate;
    let mut effect = Effect::new(EffectType::MonetaryTransfer, "Impose fine for violation");
    effect
        .parameters
        .insert("retroactive".to_string(), "true".to_string());
    let statute = Statute::new("test-6", "Retroactive fine", effect).with_temporal_validity(
        TemporalValidity::new().with_effective_date(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()),
    );
    let result = check_retroactivity(&statute);
    assert!(!result.passed);
    assert!(!result.issues.is_empty());
    assert!(result.issues.iter().any(|i| i.contains("penalty")));
}
#[test]
fn test_retroactivity_check_grant_allowed() {
    use chrono::NaiveDate;
    let mut effect = Effect::new(EffectType::Grant, "Grant benefit");
    effect
        .parameters
        .insert("retroactive".to_string(), "true".to_string());
    let statute = Statute::new("test-7", "Retroactive benefit", effect).with_temporal_validity(
        TemporalValidity::new().with_effective_date(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()),
    );
    let result = check_retroactivity(&statute);
    assert!(result.passed);
}
#[test]
fn test_retroactivity_check_no_effective_date() {
    let statute = Statute::new(
        "test-8",
        "Undated prohibition",
        Effect::new(EffectType::Prohibition, "Prohibit action"),
    );
    let result = check_retroactivity(&statute);
    assert!(result.passed);
    assert!(result.issues.is_empty());
    assert!(!result.suggestions.is_empty());
    assert!(
        result
            .suggestions
            .iter()
            .any(|s| s.contains("effective date"))
    );
}
#[test]
fn test_id_collision_detection() {
    let statute1 = Statute::new(
        "duplicate-id",
        "First Statute",
        Effect::new(EffectType::Grant, "Grant benefit"),
    )
    .with_jurisdiction("US");
    let statute2 = Statute::new(
        "duplicate-id",
        "Second Statute",
        Effect::new(EffectType::Grant, "Grant different benefit"),
    )
    .with_jurisdiction("UK");
    let conflicts = detect_statute_conflicts(&[statute1, statute2]);
    assert!(!conflicts.is_empty());
    assert!(
        conflicts
            .iter()
            .any(|c| c.conflict_type == ConflictType::IdCollision)
    );
}
#[test]
fn test_effect_conflict_detection() {
    use chrono::NaiveDate;
    let statute1 = Statute::new(
        "grant-benefit",
        "Grant Benefits",
        Effect::new(EffectType::Grant, "Grant parking permit"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 18,
    })
    .with_jurisdiction("US")
    .with_temporal_validity(
        TemporalValidity::new().with_effective_date(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()),
    );
    let statute2 = Statute::new(
        "prohibit-benefit",
        "Prohibit Benefits",
        Effect::new(EffectType::Prohibition, "Prohibit parking"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 18,
    })
    .with_jurisdiction("US")
    .with_temporal_validity(
        TemporalValidity::new().with_effective_date(NaiveDate::from_ymd_opt(2025, 6, 1).unwrap()),
    );
    let conflicts = detect_statute_conflicts(&[statute1, statute2]);
    assert!(
        conflicts
            .iter()
            .any(|c| c.conflict_type == ConflictType::EffectConflict)
    );
}
#[test]
fn test_temporal_conflict_detection() {
    use chrono::NaiveDate;
    let statute1 = Statute::new(
        "law-v1",
        "Traffic Law",
        Effect::new(EffectType::Grant, "Grant permit"),
    )
    .with_jurisdiction("US")
    .with_version(1)
    .with_temporal_validity(
        TemporalValidity::new()
            .with_effective_date(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())
            .with_expiry_date(NaiveDate::from_ymd_opt(2026, 1, 1).unwrap()),
    );
    let statute2 = Statute::new(
        "law-v2",
        "Traffic Law",
        Effect::new(EffectType::Grant, "Grant new permit"),
    )
    .with_jurisdiction("US")
    .with_version(2)
    .with_temporal_validity(
        TemporalValidity::new().with_effective_date(NaiveDate::from_ymd_opt(2025, 6, 1).unwrap()),
    );
    let conflicts = detect_statute_conflicts(&[statute1, statute2]);
    assert!(
        conflicts
            .iter()
            .any(|c| c.conflict_type == ConflictType::TemporalConflict)
    );
}
#[test]
fn test_no_conflicts_when_different_jurisdictions() {
    let statute1 = Statute::new(
        "law-1",
        "US Law",
        Effect::new(EffectType::Grant, "Grant benefit"),
    )
    .with_jurisdiction("US");
    let statute2 = Statute::new(
        "law-2",
        "UK Law",
        Effect::new(EffectType::Prohibition, "Prohibit action"),
    )
    .with_jurisdiction("UK");
    let conflicts = detect_effect_conflicts(&[statute1, statute2]);
    assert!(conflicts.is_empty());
}
#[test]
fn test_conflict_report_generation() {
    let statute1 = Statute::new("dup-id", "First", Effect::new(EffectType::Grant, "Grant"));
    let statute2 = Statute::new("dup-id", "Second", Effect::new(EffectType::Grant, "Grant"));
    let report = conflict_detection_report(&[statute1, statute2]);
    assert!(report.contains("Conflict Detection Report"));
    assert!(report.contains("ID Collision"));
}
#[test]
fn test_temporal_validity_overlap() {
    use chrono::NaiveDate;
    let tv1 = TemporalValidity::new()
        .with_effective_date(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())
        .with_expiry_date(NaiveDate::from_ymd_opt(2025, 12, 31).unwrap());
    let tv2 = TemporalValidity::new()
        .with_effective_date(NaiveDate::from_ymd_opt(2025, 6, 1).unwrap())
        .with_expiry_date(NaiveDate::from_ymd_opt(2026, 6, 1).unwrap());
    assert!(temporal_validity_overlaps(&tv1, &tv2));
    let tv3 = TemporalValidity::new()
        .with_effective_date(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())
        .with_expiry_date(NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());
    let tv4 = TemporalValidity::new()
        .with_effective_date(NaiveDate::from_ymd_opt(2027, 1, 1).unwrap())
        .with_expiry_date(NaiveDate::from_ymd_opt(2027, 12, 31).unwrap());
    assert!(!temporal_validity_overlaps(&tv3, &tv4));
}
#[test]
fn test_effects_contradict() {
    let grant = Effect::new(EffectType::Grant, "Grant permission");
    let revoke = Effect::new(EffectType::Revoke, "Revoke permission");
    let prohibition = Effect::new(EffectType::Prohibition, "Prohibit action");
    assert!(effects_contradict(&grant, &revoke));
    assert!(effects_contradict(&grant, &prohibition));
    assert!(!effects_contradict(&grant, &grant));
}
#[test]
fn test_title_similarity() {
    let sim1 = title_similarity("Traffic Law Amendment", "Traffic Law");
    assert!(sim1 > 0.5);
    let sim2 = title_similarity("Completely Different", "Another Thing");
    assert!(sim2 < 0.5);
    let sim3 = title_similarity("Same Title", "Same Title");
    assert_eq!(sim3, 1.0);
}
#[test]
fn test_conditions_overlap() {
    let cond1 = vec![Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 18,
    }];
    let cond2 = vec![Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 21,
    }];
    assert!(conditions_overlap(&cond1, &cond2));
    let cond3 = vec![Condition::Income {
        operator: ComparisonOp::LessThan,
        value: 50000,
    }];
    assert!(!conditions_overlap(&cond1, &cond3));
}
#[test]
fn test_conflict_with_suggestions() {
    let conflict = StatuteConflict::new(
        ConflictType::EffectConflict,
        vec!["law1".to_string(), "law2".to_string()],
        "Test conflict",
    )
    .with_suggestion("Fix it")
    .with_suggestion("Or do this");
    assert_eq!(conflict.resolution_suggestions.len(), 2);
    assert_eq!(conflict.severity, Severity::Critical);
}
#[test]
fn test_coverage_gap_detection() {
    let statutes = vec![
        Statute::new("young", "Young Adult Rights", Effect::grant("vote"))
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            })
            .with_precondition(Condition::Age {
                operator: ComparisonOp::LessThan,
                value: 25,
            }),
        Statute::new("senior", "Senior Rights", Effect::grant("benefits")).with_precondition(
            Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 65,
            },
        ),
    ];
    let gaps = analyze_coverage_gaps(&statutes);
    assert!(!gaps.is_empty());
    assert!(gaps.iter().any(|g| g.description.contains("age coverage")));
}
#[test]
fn test_no_coverage_gaps_simple() {
    let statutes = vec![Statute::new(
        "general",
        "General Law",
        Effect::grant("rights"),
    )];
    let gaps = analyze_coverage_gaps(&statutes);
    assert!(gaps.is_empty());
}
#[test]
fn test_jurisdiction_gap_detection() {
    let statutes = vec![
        Statute::new("us-law", "US Law", Effect::grant("benefit")).with_jurisdiction("US"),
        Statute::new("eu-law", "EU Law", Effect::grant("benefit")).with_jurisdiction("EU"),
        Statute::new("no-jurisdiction", "Unknown", Effect::grant("other")),
    ];
    let gaps = analyze_coverage_gaps(&statutes);
    assert!(
        gaps.iter()
            .any(|g| g.description.contains("no jurisdiction"))
    );
}
#[test]
fn test_optimization_report_generation() {
    let statutes = vec![
        Statute::new("complex", "Complex Law", Effect::grant("rights")).with_precondition(
            Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            },
        ),
    ];
    let report = optimization_and_gaps_report(&statutes);
    assert!(report.contains("Statute Optimization"));
    assert!(report.contains("Coverage Gaps"));
    assert!(report.contains("Summary"));
    assert!(report.contains("Total statutes analyzed: 1"));
}
#[test]
fn test_coverage_gap_severity_levels() {
    let statutes = vec![
        Statute::new("income-law", "Income-Based Law", Effect::grant("credit")).with_precondition(
            Condition::Income {
                operator: ComparisonOp::LessThan,
                value: 50000,
            },
        ),
    ];
    let gaps = analyze_coverage_gaps(&statutes);
    if let Some(gap) = gaps.iter().find(|g| g.description.contains("Income")) {
        assert_eq!(gap.severity, Severity::Info);
    }
}
#[test]
fn test_export_dependency_graph() {
    let statutes = vec![
        Statute::new("law1", "First Law", Effect::grant("right1")),
        Statute::new("law2", "Second Law", Effect::grant("right2")).with_precondition(
            Condition::Custom {
                description: "statute:law1".to_string(),
            },
        ),
    ];
    let dot = export_dependency_graph(&statutes);
    assert!(dot.contains("digraph StatuteDependencies"));
    assert!(dot.contains("law1"));
    assert!(dot.contains("law2"));
    assert!(dot.contains("law2\" -> \"law1"));
    assert!(dot.contains("[label=\"references\"]"));
}
#[test]
fn test_export_dependency_graph_with_conflicts() {
    let statutes = vec![
        Statute::new("law1", "First Law", Effect::grant("benefit")).with_precondition(
            Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            },
        ),
        Statute::new("law2", "Second Law", Effect::revoke("benefit")).with_precondition(
            Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            },
        ),
    ];
    let dot = export_dependency_graph_with_conflicts(&statutes);
    assert!(dot.contains("digraph StatuteDependenciesWithConflicts"));
    assert!(dot.contains("law1"));
    assert!(dot.contains("law2"));
    assert!(dot.contains("lightcoral") || dot.contains("lightblue"));
}
#[test]
fn test_export_dependency_graph_no_references() {
    let statutes = vec![
        Statute::new("law1", "Independent Law 1", Effect::grant("right1")),
        Statute::new("law2", "Independent Law 2", Effect::grant("right2")),
    ];
    let dot = export_dependency_graph(&statutes);
    assert!(dot.contains("law1"));
    assert!(dot.contains("law2"));
    assert!(!dot.contains("->"));
}
#[test]
fn test_quality_metrics_basic() {
    let statute = Statute::new("test-law", "Test Statute", Effect::grant("benefit"))
        .with_jurisdiction("US")
        .with_temporal_validity(TemporalValidity::new().with_enacted_at(chrono::Utc::now()));
    let metrics = analyze_quality(&statute);
    assert_eq!(metrics.statute_id, "test-law");
    assert!(metrics.overall_score >= 0.0 && metrics.overall_score <= 100.0);
    assert!(metrics.complexity_score >= 0.0 && metrics.complexity_score <= 100.0);
    assert!(metrics.readability_score >= 0.0 && metrics.readability_score <= 100.0);
}
#[test]
fn test_quality_metrics_grade() {
    let metrics = QualityMetrics::new(
        "test".to_string(),
        95.0,
        95.0,
        95.0,
        95.0,
        95.0,
        95.0,
        95.0,
        95.0,
    );
    assert_eq!(metrics.grade(), 'A');
    assert_eq!(metrics.overall_score, 95.0);
}
#[test]
fn test_quality_metrics_with_issues() {
    let statute = Statute::new("incomplete-law", "Incomplete Law", Effect::grant("benefit"));
    let metrics = analyze_quality(&statute);
    assert!(!metrics.issues.is_empty());
    assert!(metrics.issues.iter().any(|i| i.contains("jurisdiction")));
}
#[test]
fn test_quality_report_generation() {
    let statutes = vec![
        Statute::new("law1", "Good Law", Effect::grant("benefit"))
            .with_jurisdiction("US")
            .with_temporal_validity(TemporalValidity::new().with_enacted_at(chrono::Utc::now()))
            .with_discretion("A well-documented law"),
        Statute::new("law2", "Poor Law", Effect::grant("other")),
    ];
    let report = quality_report(&statutes);
    assert!(report.contains("# Statute Quality Report"));
    assert!(report.contains("law1"));
    assert!(report.contains("law2"));
    assert!(report.contains("Summary"));
    assert!(report.contains("Total statutes analyzed: 2"));
    assert!(report.contains("Grade Distribution"));
}
#[test]
fn test_quality_metrics_low_complexity_strength() {
    let statute = Statute::new("simple-law", "Simple Law", Effect::grant("benefit"))
        .with_jurisdiction("US")
        .with_temporal_validity(TemporalValidity::new().with_enacted_at(chrono::Utc::now()))
        .with_discretion("A simple law");
    let metrics = analyze_quality(&statute);
    assert!(metrics.strengths.iter().any(|s| s.contains("complexity")));
}
#[test]
fn test_drafting_quality_score_high() {
    let statute = Statute::new(
        "well-drafted-law",
        "Citizens Tax Relief Act",
        Effect::obligation("must file annual tax returns"),
    )
    .with_jurisdiction("US")
    .with_temporal_validity(
        TemporalValidity::new()
            .with_enacted_at(chrono::Utc::now())
            .with_effective_date(chrono::NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()),
    )
    .with_discretion("Applies to all US citizens earning taxable income")
    .with_precondition(legalis_core::Condition::Income {
        operator: legalis_core::ComparisonOp::GreaterOrEqual,
        value: 12000,
    });
    let metrics = analyze_quality(&statute);
    assert!(
        metrics.drafting_quality_score >= 70.0,
        "Drafting quality should be >= 70, got {}",
        metrics.drafting_quality_score
    );
}
#[test]
fn test_drafting_quality_score_low() {
    let statute = Statute::new("poor-law", "", Effect::grant(""));
    let metrics = analyze_quality(&statute);
    assert!(
        metrics.drafting_quality_score < 50.0,
        "Drafting quality should be < 50, got {}",
        metrics.drafting_quality_score
    );
}
#[test]
fn test_clarity_index_high() {
    let statute = Statute::new(
        "clear-law",
        "Simple Tax Law",
        Effect::grant("tax exemption for seniors"),
    )
    .with_discretion("Clear and simple rule")
    .with_precondition(legalis_core::Condition::Age {
        operator: legalis_core::ComparisonOp::GreaterOrEqual,
        value: 65,
    });
    let metrics = analyze_quality(&statute);
    assert!(
        metrics.clarity_index >= 70.0,
        "Clarity index should be >= 70, got {}",
        metrics.clarity_index
    );
}
#[test]
fn test_clarity_index_low() {
    let complex_desc = "This regulation establishes procedures and requirements \
        for the implementation of tax relief measures applicable to certain categories \
        of individuals meeting specific criteria as determined by the regulatory authority \
        in accordance with established guidelines and subject to periodic review";
    let statute = Statute::new(
        "complex-law",
        "Very Long Title That Exceeds Reasonable Length For A Statute Title And Becomes Confusing",
        Effect::grant(complex_desc),
    )
    .with_precondition(legalis_core::Condition::And(
        Box::new(legalis_core::Condition::Age {
            operator: legalis_core::ComparisonOp::GreaterOrEqual,
            value: 18,
        }),
        Box::new(legalis_core::Condition::Or(
            Box::new(legalis_core::Condition::Income {
                operator: legalis_core::ComparisonOp::LessThan,
                value: 50000,
            }),
            Box::new(legalis_core::Condition::Income {
                operator: legalis_core::ComparisonOp::GreaterOrEqual,
                value: 100000,
            }),
        )),
    ));
    let metrics = analyze_quality(&statute);
    assert!(
        metrics.clarity_index < 85.0,
        "Clarity index should be < 85, got {}",
        metrics.clarity_index
    );
}
#[test]
fn test_testability_score_high() {
    let statute = Statute::new(
        "testable-law",
        "Age Requirement Law",
        Effect::grant("voting rights"),
    )
    .with_jurisdiction("US")
    .with_temporal_validity(
        TemporalValidity::new()
            .with_effective_date(chrono::NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())
            .with_expiry_date(chrono::NaiveDate::from_ymd_opt(2030, 12, 31).unwrap()),
    )
    .with_precondition(legalis_core::Condition::And(
        Box::new(legalis_core::Condition::Age {
            operator: legalis_core::ComparisonOp::GreaterOrEqual,
            value: 18,
        }),
        Box::new(legalis_core::Condition::HasAttribute {
            key: "citizenship".to_string(),
        }),
    ));
    let metrics = analyze_quality(&statute);
    assert!(
        metrics.testability_score >= 70.0,
        "Testability should be >= 70, got {}",
        metrics.testability_score
    );
}
#[test]
fn test_testability_score_low() {
    let statute = Statute::new("fuzzy-law", "Vague Law", Effect::grant("some benefit"))
        .with_precondition(legalis_core::Condition::And(
            Box::new(legalis_core::Condition::Custom {
                description: "must demonstrate good character".to_string(),
            }),
            Box::new(legalis_core::Condition::Fuzzy {
                attribute: "creditworthiness".to_string(),
                membership_points: vec![(300.0, 0.0), (700.0, 0.5), (850.0, 1.0)],
                min_membership: 0.7,
            }),
        ));
    let metrics = analyze_quality(&statute);
    assert!(
        metrics.testability_score < 70.0,
        "Testability should be < 70, got {}",
        metrics.testability_score
    );
}
#[test]
fn test_maintainability_score_high() {
    let statute = Statute::new("maintainable-law", "Simple Rule", Effect::grant("benefit"))
        .with_jurisdiction("US")
        .with_temporal_validity(TemporalValidity::new().with_enacted_at(chrono::Utc::now()))
        .with_discretion("Clear documentation explaining the purpose and application")
        .with_precondition(legalis_core::Condition::Age {
            operator: legalis_core::ComparisonOp::GreaterOrEqual,
            value: 18,
        });
    let metrics = analyze_quality(&statute);
    assert!(
        metrics.maintainability_score >= 70.0,
        "Maintainability should be >= 70, got {}",
        metrics.maintainability_score
    );
}
#[test]
fn test_maintainability_score_low() {
    let statute = Statute::new("unmaintainable-law", "", Effect::grant("")).with_precondition(
        legalis_core::Condition::And(
            Box::new(legalis_core::Condition::And(
                Box::new(legalis_core::Condition::Or(
                    Box::new(legalis_core::Condition::Age {
                        operator: legalis_core::ComparisonOp::GreaterOrEqual,
                        value: 18,
                    }),
                    Box::new(legalis_core::Condition::Age {
                        operator: legalis_core::ComparisonOp::LessThan,
                        value: 65,
                    }),
                )),
                Box::new(legalis_core::Condition::And(
                    Box::new(legalis_core::Condition::Income {
                        operator: legalis_core::ComparisonOp::GreaterThan,
                        value: 25000,
                    }),
                    Box::new(legalis_core::Condition::Income {
                        operator: legalis_core::ComparisonOp::LessThan,
                        value: 75000,
                    }),
                )),
            )),
            Box::new(legalis_core::Condition::And(
                Box::new(legalis_core::Condition::HasAttribute {
                    key: "attr1".to_string(),
                }),
                Box::new(legalis_core::Condition::And(
                    Box::new(legalis_core::Condition::HasAttribute {
                        key: "attr2".to_string(),
                    }),
                    Box::new(legalis_core::Condition::HasAttribute {
                        key: "attr3".to_string(),
                    }),
                )),
            )),
        ),
    );
    let metrics = analyze_quality(&statute);
    assert!(
        metrics.maintainability_score < 60.0,
        "Maintainability should be < 60, got {}",
        metrics.maintainability_score
    );
}
#[test]
fn test_comprehensive_quality_metrics() {
    let statute = Statute::new(
        "comprehensive-law",
        "Well Designed Law",
        Effect::grant("comprehensive benefit"),
    )
    .with_jurisdiction("US")
    .with_temporal_validity(
        TemporalValidity::new()
            .with_enacted_at(chrono::Utc::now())
            .with_effective_date(chrono::NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()),
    )
    .with_discretion("Comprehensive documentation")
    .with_precondition(legalis_core::Condition::Age {
        operator: legalis_core::ComparisonOp::GreaterOrEqual,
        value: 18,
    });
    let metrics = analyze_quality(&statute);
    assert!(
        (0.0..=100.0).contains(&metrics.drafting_quality_score),
        "Drafting quality out of range: {}",
        metrics.drafting_quality_score
    );
    assert!(
        (0.0..=100.0).contains(&metrics.clarity_index),
        "Clarity index out of range: {}",
        metrics.clarity_index
    );
    assert!(
        (0.0..=100.0).contains(&metrics.testability_score),
        "Testability out of range: {}",
        metrics.testability_score
    );
    assert!(
        (0.0..=100.0).contains(&metrics.maintainability_score),
        "Maintainability out of range: {}",
        metrics.maintainability_score
    );
    let expected_avg = (metrics.complexity_score
        + metrics.readability_score
        + metrics.consistency_score
        + metrics.completeness_score
        + metrics.drafting_quality_score
        + metrics.clarity_index
        + metrics.testability_score
        + metrics.maintainability_score)
        / 8.0;
    assert!(
        (metrics.overall_score - expected_avg).abs() < 0.01,
        "Overall score mismatch: expected {}, got {}",
        expected_avg,
        metrics.overall_score
    );
}
#[test]
fn test_detect_vague_terms_in_title() {
    let statute = Statute::new(
        "vague-law",
        "Reasonable Tax Law",
        Effect::grant("tax benefit"),
    );
    let ambiguities = detect_ambiguities(&statute);
    assert!(!ambiguities.is_empty());
    assert!(
        ambiguities
            .iter()
            .any(|a| matches!(a.ambiguity_type, AmbiguityType::VagueTerm))
    );
}
#[test]
fn test_detect_vague_terms_in_description() {
    let statute = Statute::new(
        "vague-desc-law",
        "Tax Law",
        Effect::grant("may receive appropriate benefits"),
    );
    let ambiguities = detect_ambiguities(&statute);
    assert!(!ambiguities.is_empty());
    assert!(
        ambiguities
            .iter()
            .any(|a| matches!(a.ambiguity_type, AmbiguityType::VagueTerm))
    );
}
#[test]
fn test_detect_unclear_effect_empty() {
    let statute = Statute::new("unclear-law", "Test Law", Effect::grant(""));
    let ambiguities = detect_ambiguities(&statute);
    assert!(!ambiguities.is_empty());
    assert!(
        ambiguities
            .iter()
            .any(|a| matches!(a.ambiguity_type, AmbiguityType::UnclearEffect))
    );
}
#[test]
fn test_detect_unclear_effect_too_brief() {
    let statute = Statute::new("brief-law", "Test Law", Effect::grant("do it"));
    let ambiguities = detect_ambiguities(&statute);
    assert!(!ambiguities.is_empty());
    assert!(
        ambiguities
            .iter()
            .any(|a| matches!(a.ambiguity_type, AmbiguityType::UnclearEffect))
    );
}
#[test]
fn test_detect_missing_discretion() {
    let statute = Statute::new(
        "complex-law",
        "Complex Tax Law",
        Effect::grant("tax credit"),
    )
    .with_precondition(legalis_core::Condition::Age {
        operator: legalis_core::ComparisonOp::GreaterOrEqual,
        value: 18,
    })
    .with_precondition(legalis_core::Condition::Income {
        operator: legalis_core::ComparisonOp::LessThan,
        value: 50000,
    })
    .with_precondition(legalis_core::Condition::HasAttribute {
        key: "citizen".to_string(),
    })
    .with_precondition(legalis_core::Condition::HasAttribute {
        key: "resident".to_string(),
    });
    let ambiguities = detect_ambiguities(&statute);
    assert!(
        ambiguities
            .iter()
            .any(|a| matches!(a.ambiguity_type, AmbiguityType::MissingDiscretion))
    );
}
#[test]
fn test_detect_temporal_ambiguity_no_dates() {
    let statute = Statute::new("temporal-law", "Test Law", Effect::grant("benefit"));
    let ambiguities = detect_ambiguities(&statute);
    assert!(!ambiguities.is_empty());
    assert!(
        ambiguities
            .iter()
            .any(|a| matches!(a.ambiguity_type, AmbiguityType::TemporalAmbiguity))
    );
}
#[test]
fn test_detect_temporal_ambiguity_missing_effective_date() {
    let statute = Statute::new("temporal-law", "Test Law", Effect::grant("benefit"))
        .with_temporal_validity(TemporalValidity::new().with_enacted_at(chrono::Utc::now()));
    let ambiguities = detect_ambiguities(&statute);
    assert!(
        ambiguities
            .iter()
            .any(|a| matches!(a.ambiguity_type, AmbiguityType::TemporalAmbiguity))
    );
}
#[test]
fn test_detect_quantifier_ambiguity() {
    let statute = Statute::new(
        "quant-law",
        "Test Law",
        Effect::grant("some benefits for several qualified individuals"),
    );
    let ambiguities = detect_ambiguities(&statute);
    assert!(!ambiguities.is_empty());
    assert!(
        ambiguities
            .iter()
            .any(|a| matches!(a.ambiguity_type, AmbiguityType::QuantifierAmbiguity))
    );
}
#[test]
fn test_detect_implicit_assumption_custom_condition() {
    let statute = Statute::new("assumption-law", "Test Law", Effect::grant("benefit"))
        .with_precondition(legalis_core::Condition::Custom {
            description: "good".to_string(),
        });
    let ambiguities = detect_ambiguities(&statute);
    assert!(!ambiguities.is_empty());
    assert!(
        ambiguities
            .iter()
            .any(|a| matches!(a.ambiguity_type, AmbiguityType::ImplicitAssumption))
    );
}
#[test]
fn test_no_ambiguities_well_defined_statute() {
    let statute = Statute::new(
        "clear-law",
        "Senior Citizen Tax Credit",
        Effect::grant("tax credit of $1000 for qualified seniors"),
    )
    .with_jurisdiction("US")
    .with_temporal_validity(
        TemporalValidity::new()
            .with_enacted_at(chrono::Utc::now())
            .with_effective_date(chrono::NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()),
    )
    .with_discretion("Clear rule for senior tax credits")
    .with_precondition(legalis_core::Condition::Age {
        operator: legalis_core::ComparisonOp::GreaterOrEqual,
        value: 65,
    });
    let ambiguities = detect_ambiguities(&statute);
    assert!(ambiguities.is_empty() || ambiguities.len() <= 1);
}
#[test]
fn test_ambiguity_report_generation() {
    let statute = Statute::new("vague-law", "Reasonable Law", Effect::grant(""));
    let report = ambiguity_report(&statute);
    assert!(report.contains("Ambiguity Report"));
    assert!(report.contains("vague-law"));
}
#[test]
fn test_batch_ambiguity_report() {
    let statutes = vec![
        Statute::new("law1", "Reasonable Law", Effect::grant("")),
        Statute::new(
            "law2",
            "Clear Law",
            Effect::grant("specific tax credit of $500"),
        )
        .with_jurisdiction("US")
        .with_temporal_validity(
            TemporalValidity::new()
                .with_enacted_at(chrono::Utc::now())
                .with_effective_date(chrono::NaiveDate::from_ymd_opt(2025, 1, 1).unwrap()),
        ),
    ];
    let report = batch_ambiguity_report(&statutes);
    assert!(report.contains("Batch Ambiguity Detection Report"));
    assert!(report.contains("**Total Statutes Analyzed**: 2"));
    assert!(report.contains("law1"));
}
#[test]
fn test_ambiguity_severity_sorting() {
    let statute = Statute::new("multi-ambiguity-law", "Test", Effect::grant("")).with_precondition(
        legalis_core::Condition::Custom {
            description: "test".to_string(),
        },
    );
    let ambiguities = detect_ambiguities(&statute);
    for i in 0..ambiguities.len().saturating_sub(1) {
        assert!(
            ambiguities[i].severity >= ambiguities[i + 1].severity,
            "Ambiguities should be sorted by severity"
        );
    }
}
#[test]
fn test_compare_statutes_no_changes() {
    let statute1 = Statute::new("law1", "Test Law", Effect::grant("benefit"));
    let statute2 = Statute::new("law1", "Test Law", Effect::grant("benefit"));
    let changes = compare_statutes(&statute1, &statute2);
    assert!(changes.is_empty());
}
#[test]
fn test_compare_statutes_title_changed() {
    let old = Statute::new("law1", "Old Title", Effect::grant("benefit"));
    let new = Statute::new("law1", "New Title", Effect::grant("benefit"));
    let changes = compare_statutes(&old, &new);
    assert_eq!(changes.len(), 1);
    assert!(matches!(changes[0], StatuteChange::TitleChanged { .. }));
}
#[test]
fn test_compare_statutes_effect_changed() {
    let old = Statute::new("law1", "Test Law", Effect::grant("benefit"));
    let new = Statute::new("law1", "Test Law", Effect::revoke("benefit"));
    let changes = compare_statutes(&old, &new);
    assert!(
        changes
            .iter()
            .any(|c| matches!(c, StatuteChange::EffectChanged { .. }))
    );
}
#[test]
fn test_compare_statutes_preconditions_changed() {
    let old = Statute::new("law1", "Test Law", Effect::grant("benefit")).with_precondition(
        Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        },
    );
    let new = Statute::new("law1", "Test Law", Effect::grant("benefit")).with_precondition(
        Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 21,
        },
    );
    let changes = compare_statutes(&old, &new);
    assert!(
        changes
            .iter()
            .any(|c| matches!(c, StatuteChange::PreconditionsChanged { .. }))
    );
}
#[test]
fn test_analyze_change_impact_no_dependents() {
    let old = Statute::new("law1", "Old Version", Effect::grant("benefit"));
    let new = Statute::new("law1", "New Version", Effect::grant("benefit"));
    let all_statutes = vec![new.clone()];
    let impact = analyze_change_impact(&new, &old, &all_statutes);
    assert_eq!(impact.statute_id, "law1");
    assert_eq!(impact.affected_statutes.len(), 0);
    assert_eq!(impact.impact_severity, Severity::Info);
}
#[test]
fn test_analyze_change_impact_with_dependents() {
    let old = Statute::new("base-law", "Base Law Old", Effect::grant("benefit"));
    let new = Statute::new("base-law", "Base Law New", Effect::revoke("benefit"));
    let dependent = Statute::new("dependent-law", "Dependent Law", Effect::grant("other"))
        .with_precondition(Condition::Custom {
            description: "statute:base-law".to_string(),
        });
    let all_statutes = vec![new.clone(), dependent];
    let impact = analyze_change_impact(&new, &old, &all_statutes);
    assert_eq!(impact.affected_statutes.len(), 1);
    assert!(
        impact
            .affected_statutes
            .contains(&"dependent-law".to_string())
    );
    assert_eq!(impact.impact_severity, Severity::Critical);
    assert!(!impact.recommendations.is_empty());
}
#[test]
fn test_change_impact_report_generation() {
    let old = Statute::new("law1", "Old Title", Effect::grant("benefit"));
    let new = Statute::new("law1", "New Title", Effect::grant("benefit"));
    let all_statutes = vec![new.clone()];
    let impact = analyze_change_impact(&new, &old, &all_statutes);
    let report = change_impact_report(&impact);
    assert!(report.contains("# Change Impact Analysis"));
    assert!(report.contains("law1"));
    assert!(report.contains("Changes Detected"));
    assert!(report.contains("Affected Statutes"));
    assert!(report.contains("Recommendations"));
}
#[test]
fn test_statute_change_display() {
    let change = StatuteChange::TitleChanged {
        old: "Old".to_string(),
        new: "New".to_string(),
    };
    let display = format!("{}", change);
    assert!(display.contains("Title changed"));
    assert!(display.contains("Old"));
    assert!(display.contains("New"));
}
#[test]
fn test_batch_verification_basic() {
    let verifier = StatuteVerifier::new();
    let statutes = vec![
        Statute::new("law1", "Valid Law", Effect::grant("benefit")).with_precondition(
            Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            },
        ),
        Statute::new("law2", "Another Law", Effect::grant("other")).with_precondition(
            Condition::Income {
                operator: ComparisonOp::GreaterThan,
                value: 30000,
            },
        ),
    ];
    let result = batch_verify(&statutes, &verifier);
    assert_eq!(result.total_statutes, 2);
    assert_eq!(result.pass_rate(), 100.0);
}
#[test]
fn test_batch_verification_result_new() {
    let result = BatchVerificationResult::new();
    assert_eq!(result.total_statutes, 0);
    assert_eq!(result.passed, 0);
    assert_eq!(result.failed, 0);
    assert_eq!(result.pass_rate(), 0.0);
}
#[test]
fn test_batch_verification_add_result() {
    let mut batch_result = BatchVerificationResult::new();
    let result1 = VerificationResult::pass();
    let result2 = VerificationResult::fail(vec![VerificationError::DeadStatute {
        statute_id: "dead-law".to_string(),
    }]);
    batch_result.add_result("law1".to_string(), result1);
    batch_result.add_result("law2".to_string(), result2);
    assert_eq!(batch_result.total_statutes, 2);
    assert_eq!(batch_result.passed, 1);
    assert_eq!(batch_result.failed, 1);
    assert_eq!(batch_result.pass_rate(), 50.0);
    assert!(batch_result.error_counts.contains_key(&Severity::Error));
}
#[test]
fn test_batch_verification_report() {
    let mut batch_result = BatchVerificationResult::new();
    let pass = VerificationResult::pass();
    let fail = VerificationResult::fail(vec![VerificationError::DeadStatute {
        statute_id: "dead-law".to_string(),
    }]);
    batch_result.add_result("pass-law".to_string(), pass);
    batch_result.add_result("fail-law".to_string(), fail);
    batch_result.total_time_ms = 100;
    let report = batch_verification_report(&batch_result);
    assert!(report.contains("# Batch Verification Report"));
    assert!(report.contains("Summary"));
    assert!(report.contains("Total statutes: 2"));
    assert!(report.contains("Passed: 1"));
    assert!(report.contains("Failed: 1"));
    assert!(report.contains("Pass rate: 50.0%"));
    assert!(report.contains("Error Distribution"));
    assert!(report.contains("Failed Statutes"));
    assert!(report.contains("fail-law"));
}
#[test]
fn test_batch_verification_default() {
    let result = BatchVerificationResult::default();
    assert_eq!(result.total_statutes, 0);
    assert_eq!(result.pass_rate(), 0.0);
}
#[test]
fn test_batch_verification_all_pass() {
    let mut batch_result = BatchVerificationResult::new();
    for i in 1..=5 {
        batch_result.add_result(format!("law{}", i), VerificationResult::pass());
    }
    assert_eq!(batch_result.total_statutes, 5);
    assert_eq!(batch_result.passed, 5);
    assert_eq!(batch_result.failed, 0);
    assert_eq!(batch_result.pass_rate(), 100.0);
    let report = batch_verification_report(&batch_result);
    assert!(report.contains("All statutes passed verification"));
}
#[test]
fn test_statute_statistics_basic() {
    let statutes = vec![
        Statute::new("law1", "First Law", Effect::grant("benefit"))
            .with_precondition(Condition::Age {
                operator: ComparisonOp::GreaterOrEqual,
                value: 18,
            })
            .with_jurisdiction("US"),
        Statute::new("law2", "Second Law", Effect::revoke("license"))
            .with_precondition(Condition::Income {
                operator: ComparisonOp::LessThan,
                value: 50000,
            })
            .with_precondition(Condition::Age {
                operator: ComparisonOp::LessThan,
                value: 65,
            })
            .with_jurisdiction("US"),
    ];
    let stats = analyze_statute_statistics(&statutes);
    assert_eq!(stats.total_count, 2);
    assert_eq!(stats.avg_preconditions, 1.5);
    assert!(stats.jurisdiction_distribution.contains_key("US"));
    assert_eq!(stats.jurisdiction_distribution["US"], 2);
}
#[test]
fn test_statute_statistics_empty() {
    let statutes: Vec<Statute> = Vec::new();
    let stats = analyze_statute_statistics(&statutes);
    assert_eq!(stats.total_count, 0);
    assert_eq!(stats.avg_preconditions, 0.0);
    assert_eq!(stats.median_preconditions, 0.0);
}
