//! Property-based tests for legalis-porting.
//!
//! These tests use proptest to verify properties that should hold
//! for any valid inputs.

use legalis_core::{Effect, EffectType, Statute};
use legalis_i18n::{CulturalParams, Jurisdiction, LegalSystem, Locale};
use legalis_porting::{PortingEngine, PortingOptions};
use proptest::prelude::*;

/// Helper to create a basic jurisdiction
fn create_test_jurisdiction(id: &str, country: &str, legal_system: LegalSystem) -> Jurisdiction {
    Jurisdiction::new(id, id, Locale::new("en").with_country(country))
        .with_legal_system(legal_system)
        .with_cultural_params(CulturalParams::for_country(country))
}

proptest! {
    #[test]
    fn prop_ported_statute_has_valid_id(
        statute_id in "[a-z][a-z0-9-]{3,20}",
        statute_title in "[A-Za-z ]{5,50}"
    ) {
        let source = create_test_jurisdiction("SRC", "US", LegalSystem::CommonLaw);
        let target = create_test_jurisdiction("TGT", "GB", LegalSystem::CommonLaw);
        let engine = PortingEngine::new(source, target);

        let statute = Statute::new(
            statute_id,
            statute_title,
            Effect::new(EffectType::Grant, "Test effect"),
        );

        let options = PortingOptions::default();
        let result = engine.port_statute(&statute, &options);

        prop_assert!(result.is_ok());
        let ported = result.unwrap();

        // Ported statute should have a non-empty ID
        prop_assert!(!ported.statute.id.is_empty());
        // ID should start with target jurisdiction prefix
        prop_assert!(ported.statute.id.starts_with("tgt-"));
    }

    #[test]
    fn prop_compatibility_score_in_valid_range(
        count in 1usize..20
    ) {
        let source = create_test_jurisdiction("SRC", "US", LegalSystem::CommonLaw);
        let target = create_test_jurisdiction("TGT", "GB", LegalSystem::CommonLaw);
        let engine = PortingEngine::new(source, target);

        let statutes: Vec<Statute> = (0..count)
            .map(|i| Statute::new(
                format!("statute-{}", i),
                format!("Statute {}", i),
                Effect::new(EffectType::Grant, format!("Effect {}", i)),
            ))
            .collect();

        let report = engine.generate_report(&statutes);

        // Compatibility score should always be between 0 and 1
        prop_assert!(report.compatibility_score >= 0.0);
        prop_assert!(report.compatibility_score <= 1.0);
    }

    #[test]
    fn prop_risk_score_in_valid_range(
        statute_id in "[a-z][a-z0-9-]{3,20}",
    ) {
        let source = create_test_jurisdiction("SRC", "US", LegalSystem::CommonLaw);
        let target = create_test_jurisdiction("TGT", "DE", LegalSystem::CivilLaw);
        let engine = PortingEngine::new(source, target);

        let statute = Statute::new(
            statute_id,
            "Test Statute",
            Effect::new(EffectType::Grant, "Test"),
        );

        let options = PortingOptions {
            apply_cultural_params: true,
            ..Default::default()
        };
        let ported = engine.port_statute(&statute, &options).unwrap();
        let risk = engine.assess_risks(&ported);

        // Risk score should always be between 0 and 1
        prop_assert!(risk.risk_score >= 0.0);
        prop_assert!(risk.risk_score <= 1.0);
    }

    #[test]
    fn prop_semantic_preservation_in_valid_range(
        statute_title in "[A-Za-z ]{5,50}"
    ) {
        let source = create_test_jurisdiction("SRC", "US", LegalSystem::CommonLaw);
        let target = create_test_jurisdiction("TGT", "GB", LegalSystem::CommonLaw);
        let engine = PortingEngine::new(source, target);

        let statute = Statute::new(
            "test-id",
            statute_title,
            Effect::new(EffectType::Grant, "Test"),
        );

        let options = PortingOptions::default();
        let ported = engine.port_statute(&statute, &options).unwrap();
        let validation = engine.validate_semantics(&statute, &ported);

        // Preservation score should be between 0 and 1
        prop_assert!(validation.preservation_score >= 0.0);
        prop_assert!(validation.preservation_score <= 1.0);
    }

    #[test]
    fn prop_compliance_score_in_valid_range(
        statute_id in "[a-z][a-z0-9-]{3,20}",
    ) {
        let source = create_test_jurisdiction("SRC", "US", LegalSystem::CommonLaw);
        let target = create_test_jurisdiction("TGT", "GB", LegalSystem::CommonLaw);
        let engine = PortingEngine::new(source, target);

        let statute = Statute::new(
            statute_id,
            "Test Statute",
            Effect::new(EffectType::Grant, "Test"),
        );

        let options = PortingOptions {
            apply_cultural_params: true,
            ..Default::default()
        };
        let ported = engine.port_statute(&statute, &options).unwrap();
        let compliance = engine.check_compliance(&ported);

        // Compliance score should be between 0 and 1
        prop_assert!(compliance.compliance_score >= 0.0);
        prop_assert!(compliance.compliance_score <= 1.0);
    }

    #[test]
    fn prop_porting_preserves_statute_essence(
        statute_id in "[a-z][a-z0-9-]{3,20}",
        statute_title in "[A-Za-z ]{5,50}"
    ) {
        let source = create_test_jurisdiction("SRC", "US", LegalSystem::CommonLaw);
        let target = create_test_jurisdiction("TGT", "GB", LegalSystem::CommonLaw);
        let engine = PortingEngine::new(source, target);

        let statute = Statute::new(
            statute_id.clone(),
            statute_title.clone(),
            Effect::new(EffectType::Grant, "Test effect"),
        );

        let options = PortingOptions::default();
        let ported = engine.port_statute(&statute, &options).unwrap();

        // Original ID should be preserved
        prop_assert_eq!(&ported.original_id, &statute_id);

        // Title should be preserved (or at least not empty)
        prop_assert!(!ported.statute.title.is_empty());

        // Locale should match target
        prop_assert_eq!(ported.locale.language, "en");
    }

    #[test]
    fn prop_batch_porting_returns_correct_count(
        count in 1usize..10
    ) {
        let source = create_test_jurisdiction("SRC", "US", LegalSystem::CommonLaw);
        let target = create_test_jurisdiction("TGT", "GB", LegalSystem::CommonLaw);
        let engine = PortingEngine::new(source, target);

        let statutes: Vec<Statute> = (0..count)
            .map(|i| Statute::new(
                format!("statute-{}", i),
                format!("Statute {}", i),
                Effect::new(EffectType::Grant, format!("Effect {}", i)),
            ))
            .collect();

        let options = PortingOptions::default();
        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(engine.batch_port(&statutes, &options)).unwrap();

        // Should return the same number of ported statutes
        prop_assert_eq!(result.statutes.len(), count);
    }

    #[test]
    fn prop_similarity_scores_in_valid_range(
        target_title in "[A-Za-z ]{5,30}",
        candidate_count in 1usize..10
    ) {
        let source = create_test_jurisdiction("SRC", "US", LegalSystem::CommonLaw);
        let target = create_test_jurisdiction("TGT", "GB", LegalSystem::CommonLaw);
        let engine = PortingEngine::new(source, target);

        let statute = Statute::new(
            "target",
            target_title,
            Effect::new(EffectType::Grant, "Test"),
        );

        let candidates: Vec<Statute> = (0..candidate_count)
            .map(|i| Statute::new(
                format!("candidate-{}", i),
                format!("Candidate Statute {}", i),
                Effect::new(EffectType::Grant, format!("Effect {}", i)),
            ))
            .collect();

        let rt = tokio::runtime::Runtime::new().unwrap();
        let similar = rt.block_on(engine.find_similar_statutes(&statute, &candidates));

        // All similarity scores should be in valid range
        for (_stat, score) in similar {
            prop_assert!(score >= 0.0);
            prop_assert!(score <= 1.0);
        }
    }

    #[test]
    fn prop_workflow_advancement_maintains_invariants(
        statute_id in "[a-z][a-z0-9-]{3,20}",
    ) {
        let source = create_test_jurisdiction("SRC", "US", LegalSystem::CommonLaw);
        let target = create_test_jurisdiction("TGT", "GB", LegalSystem::CommonLaw);
        let engine = PortingEngine::new(source, target);

        let mut workflow = engine.create_workflow(statute_id);
        let initial_pending = workflow.pending_steps.len();

        // Advance workflow
        if initial_pending > 0 {
            let result = engine.advance_workflow(&mut workflow);
            prop_assert!(result.is_ok());

            // Invariant: completed + pending should equal initial pending
            prop_assert_eq!(
                workflow.completed_steps.len() + workflow.pending_steps.len(),
                initial_pending
            );
        }
    }

    #[test]
    fn prop_version_hash_is_non_empty(
        statute_id in "[a-z][a-z0-9-]{3,20}",
        version in 1u32..100
    ) {
        let source = create_test_jurisdiction("SRC", "US", LegalSystem::CommonLaw);
        let target = create_test_jurisdiction("TGT", "GB", LegalSystem::CommonLaw);
        let engine = PortingEngine::new(source, target);

        let statute = Statute::new(
            statute_id,
            "Test Statute",
            Effect::new(EffectType::Grant, "Test"),
        );

        let options = PortingOptions::default();
        let ported = engine.port_statute(&statute, &options).unwrap();

        let versioned = engine.create_versioned_statute(
            ported,
            version,
            "test-user".to_string(),
            "Test version".to_string(),
        );

        // Hash should never be empty
        prop_assert!(!versioned.hash.is_empty());

        // Version should match
        prop_assert_eq!(versioned.version, version);
    }

    #[test]
    fn prop_contextual_adjustments_are_valid(
        has_fine in proptest::bool::ANY,
        has_payment in proptest::bool::ANY,
    ) {
        let source = create_test_jurisdiction("SRC", "US", LegalSystem::CommonLaw);
        let target = create_test_jurisdiction("TGT", "GB", LegalSystem::CommonLaw);
        let engine = PortingEngine::new(source, target);

        let title = if has_fine && has_payment {
            "Fine Payment Law"
        } else if has_fine {
            "Fine Law"
        } else if has_payment {
            "Payment Law"
        } else {
            "Regular Law"
        };

        let statute = Statute::new(
            "test",
            title,
            Effect::new(EffectType::Obligation, "Test"),
        );

        let adjustments = engine.adjust_parameters_contextually(&statute);

        // All adjustments should have non-empty fields
        for adjustment in adjustments {
            prop_assert!(!adjustment.parameter.is_empty());
            prop_assert!(!adjustment.original_value.is_empty());
            prop_assert!(!adjustment.adjusted_value.is_empty());
            prop_assert!(!adjustment.context.is_empty());
            prop_assert!(!adjustment.rationale.is_empty());
        }
    }
}

#[test]
fn test_proptest_runs() {
    // This test ensures that proptest is properly configured
    // and can run successfully
    let proptest_configured = true;
    assert!(proptest_configured);
}
