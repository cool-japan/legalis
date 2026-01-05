//! Integration tests for legalis-porting.
//!
//! These tests verify that multiple components work together correctly.

use legalis_core::{Effect, EffectType, Statute};
use legalis_i18n::{CulturalParams, Jurisdiction, LegalSystem, Locale};
use legalis_porting::{
    AgreementType, EquivalenceMapping, ExpertReview, PortingEngine, PortingOptions,
    ReviewRecommendation, TermReplacement,
};

/// Creates a test jurisdiction for Japan
fn create_japan() -> Jurisdiction {
    Jurisdiction::new("JP", "Japan", Locale::new("ja").with_country("JP"))
        .with_legal_system(LegalSystem::CivilLaw)
        .with_cultural_params(CulturalParams::japan())
}

/// Creates a test jurisdiction for United States
fn create_usa() -> Jurisdiction {
    Jurisdiction::new("US", "United States", Locale::new("en").with_country("US"))
        .with_legal_system(LegalSystem::CommonLaw)
        .with_cultural_params(CulturalParams::for_country("US"))
}

/// Creates a test jurisdiction for Germany
fn create_germany() -> Jurisdiction {
    Jurisdiction::new("DE", "Germany", Locale::new("de").with_country("DE"))
        .with_legal_system(LegalSystem::CivilLaw)
        .with_cultural_params(CulturalParams::for_country("DE"))
}

#[test]
fn test_full_porting_pipeline() {
    // Test the complete porting pipeline from statute creation to reporting
    let engine = PortingEngine::new(create_japan(), create_usa());

    let statute = Statute::new(
        "test-statute",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test right"),
    );

    let options = PortingOptions {
        apply_cultural_params: true,
        translate_terms: true,
        generate_report: true,
        detect_conflicts: true,
        validate_semantics: true,
        ..Default::default()
    };

    // Port statute
    let ported = engine.port_statute(&statute, &options).unwrap();
    assert!(ported.statute.id.starts_with("us-"));
    // Changes may or may not exist depending on cultural parameters
    // The important thing is that porting succeeded

    // Generate report
    let report = engine.generate_report(std::slice::from_ref(&statute));
    assert!(report.compatibility_score >= 0.0 && report.compatibility_score <= 1.0);

    // Detect conflicts
    let conflicts = engine.detect_conflicts(&statute);
    assert!(!conflicts.is_empty()); // Should detect legal system mismatch

    // Validate semantics
    let validation = engine.validate_semantics(&statute, &ported);
    assert!(validation.preservation_score >= 0.0 && validation.preservation_score <= 1.0);

    // Assess risks
    let risk = engine.assess_risks(&ported);
    assert!(risk.risk_score >= 0.0 && risk.risk_score <= 1.0);
}

#[tokio::test]
async fn test_batch_processing_with_validation() {
    // Test batch processing with all validation features enabled
    let engine = PortingEngine::new(create_japan(), create_usa());

    let statutes = vec![
        Statute::new("s1", "Statute 1", Effect::new(EffectType::Grant, "Right 1")),
        Statute::new("s2", "Statute 2", Effect::new(EffectType::Grant, "Right 2")),
        Statute::new("s3", "Statute 3", Effect::new(EffectType::Grant, "Right 3")),
    ];

    let options = PortingOptions {
        apply_cultural_params: true,
        generate_report: true,
        detect_conflicts: true,
        validate_semantics: true,
        ..Default::default()
    };

    let result = engine.batch_port(&statutes, &options).await.unwrap();

    // Verify all statutes were ported
    assert_eq!(result.statutes.len(), 3);

    // Verify report was generated
    assert!(result.report.is_some());

    // Verify conflicts were detected
    assert!(!result.conflicts.is_empty());

    // Verify semantic validation was performed
    assert!(result.semantic_validation.is_some());

    // Verify risk assessment was performed
    assert!(result.risk_assessment.is_some());
}

#[test]
fn test_compliance_and_review_workflow() {
    // Test the complete compliance checking and expert review workflow
    let engine = PortingEngine::new(create_germany(), create_usa());

    let statute = Statute::new(
        "gdpr-statute",
        "Data Protection Statute",
        Effect::new(EffectType::Obligation, "Protect data"),
    );

    let options = PortingOptions {
        apply_cultural_params: true,
        ..Default::default()
    };

    // Port statute
    let ported = engine.port_statute(&statute, &options).unwrap();

    // Check compliance
    let compliance = engine.check_compliance(&ported);
    assert!(compliance.compliance_score >= 0.0 && compliance.compliance_score <= 1.0);
    assert!(!compliance.checks.is_empty());

    // Submit for review
    let mut review_request = engine.submit_for_review(ported.clone());
    assert_eq!(
        review_request.status,
        legalis_porting::ReviewStatus::Pending
    );

    // Assign expert
    engine.assign_expert(&mut review_request, "expert-001".to_string());
    assert_eq!(
        review_request.status,
        legalis_porting::ReviewStatus::Assigned
    );

    // Add expert review
    let expert_review = ExpertReview {
        id: "review-001".to_string(),
        expert_id: "expert-001".to_string(),
        expert_name: "Dr. Expert".to_string(),
        qualifications: vec!["Certified".to_string()],
        reviewed_at: chrono::Utc::now().to_rfc3339(),
        recommendation: ReviewRecommendation::Approve,
        comments: Vec::new(),
        confidence: 0.95,
        concerns: Vec::new(),
        suggested_modifications: Vec::new(),
    };

    engine
        .add_expert_review(&mut review_request, expert_review)
        .unwrap();
    assert_eq!(
        review_request.status,
        legalis_porting::ReviewStatus::Approved
    );
}

#[test]
fn test_workflow_management() {
    // Test workflow creation and advancement
    let engine = PortingEngine::new(create_japan(), create_usa());

    let mut workflow = engine.create_workflow("test-statute".to_string());
    assert_eq!(workflow.state, legalis_porting::WorkflowState::Initiated);

    let initial_pending = workflow.pending_steps.len();
    assert!(initial_pending > 0);

    // Advance through all steps
    for _ in 0..initial_pending {
        engine.advance_workflow(&mut workflow).unwrap();
    }

    assert_eq!(workflow.completed_steps.len(), initial_pending);
    assert_eq!(workflow.pending_steps.len(), 0);
    assert_eq!(
        workflow.state,
        legalis_porting::WorkflowState::PendingReview
    );
}

#[test]
fn test_version_control_and_comparison() {
    // Test versioning and version comparison
    let engine = PortingEngine::new(create_japan(), create_usa());

    let statute1 = Statute::new("test", "Version 1", Effect::new(EffectType::Grant, "V1"));
    let statute2 = Statute::new("test", "Version 2", Effect::new(EffectType::Grant, "V2"));

    let options = PortingOptions::default();

    let ported1 = engine.port_statute(&statute1, &options).unwrap();
    let ported2 = engine.port_statute(&statute2, &options).unwrap();

    let v1 = engine.create_versioned_statute(ported1, 1, "user".to_string(), "V1".to_string());
    let v2 = engine.create_versioned_statute(ported2, 2, "user".to_string(), "V2".to_string());

    assert_eq!(v1.version, 1);
    assert_eq!(v2.version, 2);
    assert!(v1.previous_hash.is_none());
    assert!(v2.previous_hash.is_some());

    let differences = engine.compare_versions(&v1, &v2);
    assert!(!differences.is_empty());
}

#[test]
fn test_bilateral_agreement_and_equivalence() {
    // Test bilateral agreement creation and regulatory equivalence
    let engine = PortingEngine::new(create_japan(), create_usa()).with_equivalence_mappings(vec![
        EquivalenceMapping {
            source_regulation: "jp-reg-001".to_string(),
            target_regulation: "us-reg-001".to_string(),
            equivalence_score: 0.95,
            differences: vec!["Minor terminology".to_string()],
            notes: "Highly equivalent".to_string(),
        },
    ]);

    // Create bilateral agreement
    let agreement = engine.create_bilateral_agreement(AgreementType::MutualRecognition);
    assert_eq!(agreement.source_jurisdiction, "JP");
    assert_eq!(agreement.target_jurisdiction, "US");
    assert!(!agreement.mutual_recognition.is_empty());
    assert!(!agreement.adaptation_protocols.is_empty());

    // Test regulatory equivalence
    let statute = Statute::new(
        "jp-reg-001",
        "Regulation",
        Effect::new(EffectType::Grant, "Test"),
    );
    let mappings = engine.find_regulatory_equivalence(&statute);
    assert_eq!(mappings.len(), 1);
    assert_eq!(mappings[0].equivalence_score, 0.95);
}

#[test]
fn test_term_replacement_and_contextual_adjustment() {
    // Test automatic term replacement and contextual parameter adjustment
    let engine = PortingEngine::new(create_japan(), create_usa()).with_term_replacements(vec![
        TermReplacement {
            source_term: "成人".to_string(),
            target_term: "adult".to_string(),
            context: None,
            confidence: 0.95,
        },
    ]);

    let mut statute = Statute::new(
        "test",
        "成人 Rights Law",
        Effect::new(EffectType::Grant, "Test"),
    );

    // Test term replacement
    let replacements = engine.apply_term_replacement(&mut statute);
    assert_eq!(replacements.len(), 1);
    assert!(statute.title.contains("adult"));

    // Test contextual adjustment
    let statute2 = Statute::new(
        "test",
        "Fine Payment Law",
        Effect::new(EffectType::Obligation, "Pay fine"),
    );
    let adjustments = engine.adjust_parameters_contextually(&statute2);
    assert!(!adjustments.is_empty());
}

#[tokio::test]
async fn test_similar_statute_finding() {
    // Test similar statute finding across jurisdictions
    let engine = PortingEngine::new(create_japan(), create_usa());

    let statute = Statute::new(
        "test",
        "Adult Rights Law",
        Effect::new(EffectType::Grant, "Test"),
    );

    let candidates = vec![
        Statute::new(
            "c1",
            "Adult Rights Statute",
            Effect::new(EffectType::Grant, "C1"),
        ),
        Statute::new(
            "c2",
            "Child Protection Law",
            Effect::new(EffectType::Grant, "C2"),
        ),
        Statute::new(
            "c3",
            "Adult Legal Capacity",
            Effect::new(EffectType::Grant, "C3"),
        ),
    ];

    let similar = engine.find_similar_statutes(&statute, &candidates).await;
    assert!(!similar.is_empty());

    // First result should be most similar
    assert!(similar[0].1 >= similar[similar.len() - 1].1);

    // All similarity scores should be in valid range
    for (_statute, score) in &similar {
        assert!(*score >= 0.0 && *score <= 1.0);
    }
}

#[test]
fn test_batch_compliance_checking() {
    // Test batch compliance checking with summary generation
    let engine = PortingEngine::new(create_japan(), create_usa());

    let statutes = (0..10)
        .map(|i| {
            Statute::new(
                format!("s{}", i),
                format!("Statute {}", i),
                Effect::new(EffectType::Grant, format!("Right {}", i)),
            )
        })
        .collect::<Vec<_>>();

    let options = PortingOptions {
        apply_cultural_params: true,
        ..Default::default()
    };

    let ported: Vec<_> = statutes
        .iter()
        .map(|s| engine.port_statute(s, &options).unwrap())
        .collect();

    // Batch compliance check
    let results = engine.batch_check_compliance(&ported);
    assert_eq!(results.len(), 10);

    // All results should have valid compliance scores
    for result in &results {
        assert!(result.compliance_score >= 0.0 && result.compliance_score <= 1.0);
        assert!(!result.checks.is_empty());
    }

    // Generate compliance summary
    let summary = engine.generate_compliance_summary(&results);
    assert_eq!(summary.total_statutes, 10);
    assert!(summary.average_compliance_score >= 0.0 && summary.average_compliance_score <= 1.0);
}

#[test]
fn test_partial_porting_with_sections() {
    // Test partial porting of specific statute sections
    let engine = PortingEngine::new(create_japan(), create_usa());

    let statute = Statute::new(
        "complex-statute",
        "Complex Multi-Section Statute",
        Effect::new(EffectType::Grant, "Various rights"),
    );

    let section_ids = vec![
        "section-1".to_string(),
        "section-3".to_string(),
        "section-5".to_string(),
    ];

    let options = PortingOptions::default();

    let ported = engine
        .port_sections(&statute, &section_ids, &options)
        .unwrap();

    // Verify partial porting was recorded
    let has_partial_change = ported.changes.iter().any(|c| {
        matches!(
            c.change_type,
            legalis_porting::ChangeType::ComplianceAddition
        )
    });
    assert!(has_partial_change);
}

#[test]
fn test_reverse_porting_analysis() {
    // Test reverse porting analysis (target to source)
    let engine = PortingEngine::new(create_japan(), create_usa());

    let statute = Statute::new(
        "test",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test"),
    );

    let changes = engine.reverse_port_analysis(&statute).unwrap();

    // Should detect changes needed to port back
    // The specific changes depend on cultural parameter differences
    for change in &changes {
        assert!(!change.description.is_empty());
        assert!(!change.reason.is_empty());
    }
}

#[test]
fn test_end_to_end_cross_jurisdiction_porting() {
    // End-to-end test of porting through multiple jurisdictions
    let jp = create_japan();
    let us = create_usa();
    let de = create_germany();

    // Port from Japan to US
    let engine_jp_us = PortingEngine::new(jp.clone(), us.clone());
    let statute = Statute::new(
        "original",
        "Original Statute",
        Effect::new(EffectType::Grant, "Original right"),
    );

    let options = PortingOptions {
        apply_cultural_params: true,
        ..Default::default()
    };

    let ported_us = engine_jp_us.port_statute(&statute, &options).unwrap();

    // Port from US to Germany
    let engine_us_de = PortingEngine::new(us, de);
    let ported_de = engine_us_de
        .port_statute(&ported_us.statute, &options)
        .unwrap();

    // Verify chain of adaptations
    assert!(ported_us.statute.id.starts_with("us-"));
    assert!(ported_de.statute.id.starts_with("de-"));
    // Changes are recorded if there are cultural parameter differences
    // The important verification is that IDs were properly transformed
}
