use legalis_core::Statute;

use super::super::types::{
    MarkovTransition, NaturalLanguageExplanation, StatuteConflict, VerificationPathNode,
    WhatIfScenario,
};
use super::super::types_3::{
    ConflictType, DependencyNode, LazyVerificationConfig, PrivacyBudget, ProofCache,
    StatisticalCheckResult, TeeConfig, ZeroKnowledgeProof,
};
use super::super::types_4::{
    ConflictExplanation, DependencyGraph, DependencyType, MarkovChain, Severity, VerificationDiff,
};
use super::super::types_5::{
    CachedProof, EncryptedStatute, MarkovState, VerificationError, VerificationResult,
};

use super::super::*;
use legalis_core::{ComparisonOp, Condition, Effect, EffectType};

#[test]
fn test_markov_chain_complex_reachability() {
    let chain = MarkovChain::new("complex", "s1")
        .add_state(MarkovState::new("s1", "Start"))
        .add_state(MarkovState::new("s2", "Intermediate"))
        .add_state(MarkovState::new("s3", "Accepting").accepting())
        .add_transition(MarkovTransition::new("s1", "s2", 0.5))
        .add_transition(MarkovTransition::new("s1", "s3", 0.5))
        .add_transition(MarkovTransition::new("s2", "s3", 1.0));
    let prob = chain.reachability_probability(10);
    assert!(prob > 0.9);
}
#[test]
fn test_statistical_result_confidence_interval() {
    let result = StatisticalCheckResult::from_samples("test", 10000, 5000, 0.48);
    assert!(result.confidence_lower < 0.5);
    assert!(result.confidence_upper > 0.5);
    assert!(result.confidence_upper - result.confidence_lower < 0.05);
}
#[test]
fn test_natural_language_explanation_creation() {
    let explanation = NaturalLanguageExplanation::new(
        "Test Error",
        "Simple explanation",
        "Technical explanation",
        "Why it matters",
        "How to fix",
    )
    .with_example("Example scenario");
    assert_eq!(explanation.error_type, "Test Error");
    assert_eq!(explanation.simple_explanation, "Simple explanation");
    assert!(explanation.example_scenario.is_some());
}
#[test]
fn test_natural_language_explanation_format() {
    let explanation =
        NaturalLanguageExplanation::new("Test Error", "Simple", "Technical", "Why", "Fix");
    let formatted = explanation.format(true);
    assert!(formatted.contains("# Test Error"));
    assert!(formatted.contains("## What's Wrong?"));
    assert!(formatted.contains("## Technical Details"));
    assert!(formatted.contains("## Why This Matters"));
    assert!(formatted.contains("## How to Fix It"));
    let formatted_simple = explanation.format(false);
    assert!(!formatted_simple.contains("## Technical Details"));
}
#[test]
fn test_explain_error_circular_reference() {
    let error = VerificationError::CircularReference {
        message: "Test circular ref".to_string(),
    };
    let explanation = explain_error(&error);
    assert_eq!(explanation.error_type, "Circular Reference");
    assert!(explanation.simple_explanation.contains("infinite loop"));
    assert!(explanation.example_scenario.is_some());
}
#[test]
fn test_explain_error_dead_statute() {
    let error = VerificationError::DeadStatute {
        statute_id: "statute-1".to_string(),
    };
    let explanation = explain_error(&error);
    assert_eq!(explanation.error_type, "Impossible to Satisfy");
    assert!(explanation.simple_explanation.contains("statute-1"));
    assert!(explanation.why_it_matters.contains("impossible"));
}
#[test]
fn test_explain_error_constitutional_conflict() {
    let error = VerificationError::ConstitutionalConflict {
        statute_id: "statute-1".to_string(),
        principle: "Equal Protection".to_string(),
    };
    let explanation = explain_error(&error);
    assert_eq!(explanation.error_type, "Constitutional Conflict");
    assert!(explanation.simple_explanation.contains("Equal Protection"));
    assert!(explanation.how_to_fix.contains("Equal Protection"));
}
#[test]
fn test_explain_error_ambiguity() {
    let error = VerificationError::Ambiguity {
        message: "Vague term".to_string(),
    };
    let explanation = explain_error(&error);
    assert_eq!(explanation.error_type, "Ambiguous Language");
    assert!(explanation.how_to_fix.contains("specific"));
}
#[test]
fn test_conflict_explanation_creation() {
    let explanation = ConflictExplanation::new(
        vec!["statute-1".to_string(), "statute-2".to_string()],
        "Test conflict",
    )
    .with_impact("Test impact")
    .add_affected_party("Party 1")
    .add_resolution_option("Option 1");
    assert_eq!(explanation.statute_ids.len(), 2);
    assert_eq!(explanation.impact, "Test impact");
    assert_eq!(explanation.affected_parties.len(), 1);
    assert_eq!(explanation.resolution_options.len(), 1);
}
#[test]
fn test_conflict_explanation_format() {
    let explanation = ConflictExplanation::new(
        vec!["S1".to_string(), "S2".to_string()],
        "Conflict description",
    )
    .with_impact("Impact")
    .add_affected_party("Party A")
    .add_resolution_option("Fix 1");
    let formatted = explanation.format();
    assert!(formatted.contains("# Conflict Between: S1, S2"));
    assert!(formatted.contains("## What's the Conflict?"));
    assert!(formatted.contains("## Real-World Impact"));
    assert!(formatted.contains("## Who's Affected?"));
    assert!(formatted.contains("## How to Resolve This"));
}
#[test]
fn test_explain_conflict_effect_conflict() {
    let conflict = StatuteConflict {
        conflict_type: ConflictType::EffectConflict,
        statute_ids: vec!["S1".to_string(), "S2".to_string()],
        description: "Test conflict".to_string(),
        severity: Severity::Error,
        resolution_suggestions: vec!["Suggestion 1".to_string()],
    };
    let explanation = explain_conflict(&conflict);
    assert_eq!(explanation.statute_ids.len(), 2);
    assert!(explanation.description.contains("overlapping conditions"));
    assert!(!explanation.affected_parties.is_empty());
    assert_eq!(explanation.resolution_options.len(), 1);
}
#[test]
fn test_explain_conflict_jurisdictional_overlap() {
    let conflict = StatuteConflict {
        conflict_type: ConflictType::JurisdictionalOverlap,
        statute_ids: vec!["S1".to_string(), "S2".to_string()],
        description: "Overlapping jurisdiction".to_string(),
        severity: Severity::Warning,
        resolution_suggestions: vec![],
    };
    let explanation = explain_conflict(&conflict);
    assert!(explanation.description.contains("jurisdiction"));
    assert!(
        explanation
            .affected_parties
            .iter()
            .any(|p| p.contains("jurisdiction"))
    );
}
#[test]
fn test_verification_path_node_creation() {
    let node = VerificationPathNode::new("node-1", "statute", "Test Statute")
        .with_status(true)
        .add_metadata("key", "value");
    assert_eq!(node.id, "node-1");
    assert_eq!(node.node_type, "statute");
    assert!(node.passed);
    assert_eq!(node.metadata.get("key").unwrap(), "value");
}
#[test]
fn test_verification_path_node_with_children() {
    let child = VerificationPathNode::new("child", "condition", "Age >= 18");
    let parent = VerificationPathNode::new("parent", "statute", "Parent Statute").add_child(child);
    assert_eq!(parent.children.len(), 1);
    assert_eq!(parent.children[0].id, "child");
}
#[test]
fn test_verification_path_to_dot() {
    let node = VerificationPathNode::new("root", "statute", "Test")
        .with_status(true)
        .add_child(VerificationPathNode::new("child", "condition", "Condition"));
    let dot = node.to_dot();
    assert!(dot.contains("digraph VerificationPath"));
    assert!(dot.contains("\"root\""));
    assert!(dot.contains("\"child\""));
    assert!(dot.contains("->"));
    assert!(dot.contains("green"));
}
#[test]
fn test_build_verification_path_simple() {
    let statute = Statute::new(
        "test-1",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test"),
    );
    let result = VerificationResult::pass();
    let path = build_verification_path(&statute, &result);
    assert_eq!(path.id, "test-1");
    assert!(path.passed);
    assert!(!path.children.is_empty());
}
#[test]
fn test_build_verification_path_with_preconditions() {
    let statute = Statute::new(
        "test-1",
        "Test Statute",
        Effect::new(EffectType::Grant, "Test"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 18,
    });
    let result = VerificationResult::pass();
    let path = build_verification_path(&statute, &result);
    assert_eq!(path.id, "test-1");
    assert!(path.children.len() >= 2);
}
#[test]
fn test_build_verification_path_with_errors() {
    let statute = Statute::new("test-1", "Test", Effect::new(EffectType::Grant, "Test"));
    let mut result = VerificationResult::pass();
    result.passed = false;
    result.errors.push(VerificationError::CircularReference {
        message: "Test error".to_string(),
    });
    let path = build_verification_path(&statute, &result);
    assert!(!path.passed);
    assert!(path.children.iter().any(|c| c.node_type == "error"));
}
#[test]
fn test_what_if_scenario_creation() {
    let original = Statute::new(
        "test-1",
        "Original Title",
        Effect::new(EffectType::Grant, "Test"),
    );
    let modified = Statute::new(
        "test-1",
        "Modified Title",
        Effect::new(EffectType::Grant, "Test"),
    );
    let scenario = WhatIfScenario::new(
        "Title change test",
        original.clone(),
        modified,
        VerificationResult::pass(),
        VerificationResult::pass(),
    );
    assert_eq!(scenario.description, "Title change test");
    assert!(!scenario.changes.is_empty());
    assert!(scenario.changes[0].contains("Title changed"));
}
#[test]
fn test_what_if_scenario_detect_effect_change() {
    let original = Statute::new("test-1", "Test", Effect::new(EffectType::Grant, "Test"));
    let modified = Statute::new(
        "test-1",
        "Test",
        Effect::new(EffectType::Prohibition, "Test"),
    );
    let scenario = WhatIfScenario::new(
        "Effect change",
        original,
        modified,
        VerificationResult::pass(),
        VerificationResult::pass(),
    );
    assert!(
        scenario
            .changes
            .iter()
            .any(|c| c.contains("Effect type changed"))
    );
}
#[test]
fn test_what_if_scenario_report() {
    let original = Statute::new("test-1", "Test", Effect::new(EffectType::Grant, "Test"));
    let modified = original.clone();
    let mut orig_result = VerificationResult::pass();
    orig_result.errors.push(VerificationError::Ambiguity {
        message: "Test".to_string(),
    });
    orig_result.passed = false;
    let new_result = VerificationResult::pass();
    let scenario =
        WhatIfScenario::new("Fix ambiguity", original, modified, orig_result, new_result);
    let report = scenario.report();
    assert!(report.contains("# What-If Scenario"));
    assert!(report.contains("## Impact Analysis"));
    assert!(report.contains("✓"));
}
#[test]
fn test_what_if_analysis() {
    let statute = Statute::new(
        "test-1",
        "Original Title",
        Effect::new(EffectType::Grant, "Test"),
    );
    let scenario = what_if_analysis("Change title", statute, |s| {
        s.title = "New Title".to_string();
    });
    assert_eq!(scenario.description, "Change title");
    assert_eq!(scenario.modified_statute.title, "New Title");
    assert!(scenario.changes.iter().any(|c| c.contains("Title changed")));
}
#[test]
fn test_what_if_breaking_change() {
    let statute = Statute::new("test-1", "Test", Effect::new(EffectType::Grant, "Test"));
    let scenario = what_if_analysis("Add precondition", statute, |s| {
        s.preconditions.push(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        });
    });
    assert!(
        scenario
            .changes
            .iter()
            .any(|c| c.contains("Preconditions modified"))
    );
}
#[test]
fn test_build_condition_path_age() {
    use legalis_core::{ComparisonOp, Condition};
    let condition = Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 18,
    };
    let node = build_condition_path(&condition, "test");
    assert_eq!(node.node_type, "condition");
    assert!(node.label.contains("Age"));
    assert!(node.label.contains("18"));
}
#[test]
fn test_build_condition_path_complex() {
    use legalis_core::{ComparisonOp, Condition};
    let condition = Condition::And(
        Box::new(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        }),
        Box::new(Condition::Income {
            operator: ComparisonOp::LessThan,
            value: 50000,
        }),
    );
    let node = build_condition_path(&condition, "test");
    assert_eq!(node.node_type, "logic");
    assert_eq!(node.label, "AND");
    assert_eq!(node.children.len(), 2);
}
#[test]
fn test_verification_path_failed_status() {
    let node = VerificationPathNode::new("failed", "error", "Test Error").with_status(false);
    let dot = node.to_dot();
    assert!(dot.contains("red"));
    assert!(dot.contains("bold"));
}
#[test]
fn test_zero_knowledge_proof_creation() {
    let statute = Statute::new("test-1", "Test", Effect::new(EffectType::Grant, "Test"));
    let proof = ZeroKnowledgeProof::new("statute is valid", &statute);
    assert!(proof.proof_id.starts_with("zkp-"));
    assert_eq!(proof.statement, "statute is valid");
    assert!(!proof.commitment.is_empty());
    assert_eq!(proof.challenge.len(), 32);
    assert_eq!(proof.response.len(), 32);
}
#[test]
fn test_zero_knowledge_proof_verification() {
    let statute = Statute::new("test-1", "Test", Effect::new(EffectType::Grant, "Test"));
    let proof = ZeroKnowledgeProof::new("statute is valid", &statute);
    assert!(proof.verify());
}
#[test]
fn test_zero_knowledge_proof_with_metadata() {
    let statute = Statute::new("test-1", "Test", Effect::new(EffectType::Grant, "Test"));
    let proof = ZeroKnowledgeProof::new("statute is valid", &statute)
        .with_metadata("prover", "alice")
        .with_metadata("version", "1.0");
    assert_eq!(proof.metadata.get("prover"), Some(&"alice".to_string()));
    assert_eq!(proof.metadata.get("version"), Some(&"1.0".to_string()));
}
#[test]
fn test_zero_knowledge_proof_report() {
    let statute = Statute::new("test-1", "Test", Effect::new(EffectType::Grant, "Test"));
    let proof = ZeroKnowledgeProof::new("statute is valid", &statute);
    let report = proof.report();
    assert!(report.contains("Zero-Knowledge Proof Report"));
    assert!(report.contains("statute is valid"));
    assert!(report.contains("Valid: true"));
}
#[test]
fn test_multiparty_verification_creation() {
    let statute = Statute::new("test-1", "Test", Effect::new(EffectType::Grant, "Test"));
    let parties = vec!["Alice".to_string(), "Bob".to_string(), "Carol".to_string()];
    let result = secure_multiparty_verification(&statute, parties.clone());
    assert_eq!(result.parties, parties);
    assert!(result.combined_result.passed);
    assert!(result.computation_proof.starts_with("mpc-proof-"));
}
#[test]
fn test_multiparty_verification_report() {
    let statute = Statute::new("test-1", "Test", Effect::new(EffectType::Grant, "Test"));
    let parties = vec!["Alice".to_string(), "Bob".to_string()];
    let result = secure_multiparty_verification(&statute, parties);
    let report = result.report();
    assert!(report.contains("Multi-Party Verification Report"));
    assert!(report.contains("Alice, Bob"));
    assert!(report.contains("Verification Passed: true"));
}
#[test]
fn test_privacy_budget_creation() {
    let budget = PrivacyBudget::new(1.0, 1e-5);
    assert_eq!(budget.epsilon, 1.0);
    assert_eq!(budget.delta, 1e-5);
}
#[test]
fn test_privacy_budget_presets() {
    let strict = PrivacyBudget::strict();
    assert_eq!(strict.epsilon, 0.1);
    let moderate = PrivacyBudget::moderate();
    assert_eq!(moderate.epsilon, 1.0);
    let relaxed = PrivacyBudget::relaxed();
    assert_eq!(relaxed.epsilon, 3.0);
}
#[test]
fn test_differential_private_analysis() {
    let statutes = vec![
        Statute::new("test-1", "Test 1", Effect::new(EffectType::Grant, "Test")),
        Statute::new("test-2", "Test 2", Effect::new(EffectType::Grant, "Test")),
        Statute::new("test-3", "Test 3", Effect::new(EffectType::Grant, "Test")),
    ];
    let budget = PrivacyBudget::moderate();
    let result = differential_private_analysis(&statutes, budget);
    assert!(result.count > 0.0);
    assert!(result.count < 10.0);
    assert!(result.error_rate >= 0.0);
    assert!(result.error_rate <= 1.0);
    assert_eq!(result.privacy_budget.epsilon, 1.0);
}
#[test]
fn test_differential_private_analysis_empty() {
    let statutes: Vec<Statute> = vec![];
    let budget = PrivacyBudget::strict();
    let result = differential_private_analysis(&statutes, budget);
    assert!(result.count >= 0.0);
}
#[test]
fn test_private_aggregation_report() {
    let statutes = vec![Statute::new(
        "test-1",
        "Test",
        Effect::new(EffectType::Grant, "Test"),
    )];
    let budget = PrivacyBudget::moderate();
    let result = differential_private_analysis(&statutes, budget);
    let report = result.report();
    assert!(report.contains("Differential Privacy Report"));
    assert!(report.contains("Privacy Budget"));
}
#[test]
fn test_encrypted_statute_creation() {
    let statute = Statute::new("test-1", "Test", Effect::new(EffectType::Grant, "Test"));
    let encrypted = EncryptedStatute::new(&statute);
    assert!(!encrypted.encrypted_id.is_empty());
    assert!(!encrypted.encrypted_data.is_empty());
    assert_eq!(encrypted.scheme, "Simplified-XOR");
}
#[test]
fn test_encrypted_statute_homomorphic_verify() {
    let statute = Statute::new("test-1", "Test", Effect::new(EffectType::Grant, "Test"));
    let encrypted = EncryptedStatute::new(&statute);
    let result = encrypted.homomorphic_verify();
    assert!(!result.encrypted_result.is_empty());
    assert_eq!(result.scheme, "Simplified-XOR");
}
#[test]
fn test_encrypted_verification_result_report() {
    let statute = Statute::new("test-1", "Test", Effect::new(EffectType::Grant, "Test"));
    let encrypted = EncryptedStatute::new(&statute);
    let result = encrypted.homomorphic_verify();
    let report = result.report();
    assert!(report.contains("Encrypted Verification Result"));
    assert!(report.contains("Simplified-XOR"));
    assert!(report.contains("cannot be read without decryption key"));
}
#[test]
fn test_tee_config_creation() {
    let config = TeeConfig::new("SGX");
    assert_eq!(config.tee_type, "SGX");
    assert_eq!(config.attestation.len(), 64);
}
#[test]
fn test_tee_config_attestation_verification() {
    let config = TeeConfig::new("TrustZone");
    assert!(config.verify_attestation());
}
#[test]
fn test_tee_verification() {
    let statute = Statute::new("test-1", "Test", Effect::new(EffectType::Grant, "Test"));
    let config = TeeConfig::new("SGX");
    let result = tee_verification(&statute, config);
    assert!(result.result.passed);
    assert_eq!(result.tee_config.tee_type, "SGX");
    assert!(result.attestation_proof.starts_with("tee-attestation-"));
}
#[test]
fn test_tee_verification_report() {
    let statute = Statute::new("test-1", "Test", Effect::new(EffectType::Grant, "Test"));
    let config = TeeConfig::new("SEV");
    let result = tee_verification(&statute, config);
    let report = result.report();
    assert!(report.contains("TEE Verification Report"));
    assert!(report.contains("SEV"));
    assert!(report.contains("Attestation Valid: true"));
    assert!(report.contains("Verification Passed: true"));
}
#[test]
fn test_multiparty_verification_with_multiple_parties() {
    let statute = Statute::new("test-1", "Test", Effect::new(EffectType::Grant, "Test"));
    let parties = vec![
        "Alice".to_string(),
        "Bob".to_string(),
        "Carol".to_string(),
        "David".to_string(),
    ];
    let result = secure_multiparty_verification(&statute, parties.clone());
    assert_eq!(result.parties.len(), 4);
    assert!(result.parties.contains(&"Alice".to_string()));
    assert!(result.parties.contains(&"David".to_string()));
    assert!(result.combined_result.passed);
}
#[test]
fn test_zero_knowledge_proof_different_statutes_different_commitments() {
    let statute1 = Statute::new("test-1", "Test 1", Effect::new(EffectType::Grant, "Test"));
    let statute2 = Statute::new("test-2", "Test 2", Effect::new(EffectType::Grant, "Test"));
    let proof1 = ZeroKnowledgeProof::new("statement", &statute1);
    let proof2 = ZeroKnowledgeProof::new("statement", &statute2);
    assert_ne!(proof1.commitment, proof2.commitment);
}
#[test]
fn test_dependency_node_creation() {
    let node = DependencyNode::new("statute-1", DependencyType::DerivesFrom);
    assert_eq!(node.statute_id, "statute-1");
    assert_eq!(node.dependency_type, DependencyType::DerivesFrom);
    assert!(node.dependencies.is_empty());
    assert!(node.dependents.is_empty());
    assert!(node.last_verified.is_none());
}
#[test]
fn test_dependency_node_add_dependency() {
    let mut node = DependencyNode::new("statute-1", DependencyType::DerivesFrom);
    node.add_dependency("statute-2");
    node.add_dependency("statute-3");
    assert_eq!(node.dependencies.len(), 2);
    assert!(node.dependencies.contains(&"statute-2".to_string()));
    assert!(node.dependencies.contains(&"statute-3".to_string()));
}
#[test]
fn test_dependency_node_add_dependent() {
    let mut node = DependencyNode::new("statute-1", DependencyType::DerivesFrom);
    node.add_dependent("statute-4");
    assert_eq!(node.dependents.len(), 1);
    assert!(node.dependents.contains(&"statute-4".to_string()));
}
#[test]
fn test_dependency_node_mark_verified() {
    let mut node = DependencyNode::new("statute-1", DependencyType::DerivesFrom);
    assert!(node.last_verified.is_none());
    node.mark_verified();
    assert!(node.last_verified.is_some());
}
#[test]
fn test_dependency_graph_from_statutes() {
    let mut statute1 = Statute::new("s1", "Test 1", Effect::new(EffectType::Grant, "Test"));
    statute1.derives_from = vec!["s0".to_string()];
    let statute2 = Statute::new("s2", "Test 2", Effect::new(EffectType::Grant, "Test"));
    let statutes = vec![statute1, statute2];
    let graph = DependencyGraph::from_statutes(&statutes);
    assert_eq!(graph.nodes.len(), 2);
    assert!(graph.nodes.contains_key("s1"));
    assert!(graph.nodes.contains_key("s2"));
}
#[test]
fn test_dependency_graph_transitive_dependencies() {
    let mut statute1 = Statute::new("s1", "Test 1", Effect::new(EffectType::Grant, "Test"));
    statute1.derives_from = vec!["s2".to_string()];
    let mut statute2 = Statute::new("s2", "Test 2", Effect::new(EffectType::Grant, "Test"));
    statute2.derives_from = vec!["s3".to_string()];
    let statute3 = Statute::new("s3", "Test 3", Effect::new(EffectType::Grant, "Test"));
    let statutes = vec![statute1, statute2, statute3];
    let graph = DependencyGraph::from_statutes(&statutes);
    let deps = graph.get_transitive_dependencies("s1");
    assert!(deps.contains(&"s2".to_string()));
    assert!(deps.contains(&"s3".to_string()));
}
#[test]
fn test_dependency_graph_affected_statutes() {
    let mut statute1 = Statute::new("s1", "Test 1", Effect::new(EffectType::Grant, "Test"));
    statute1.derives_from = vec!["s3".to_string()];
    let mut statute2 = Statute::new("s2", "Test 2", Effect::new(EffectType::Grant, "Test"));
    statute2.derives_from = vec!["s3".to_string()];
    let statute3 = Statute::new("s3", "Test 3", Effect::new(EffectType::Grant, "Test"));
    let statutes = vec![statute1, statute2, statute3];
    let graph = DependencyGraph::from_statutes(&statutes);
    let affected = graph.get_affected_statutes("s3");
    assert!(affected.contains(&"s1".to_string()) || affected.contains(&"s2".to_string()));
}
#[test]
fn test_lazy_verification_config_new() {
    let config = LazyVerificationConfig::new();
    assert!(config.verify_changed_only);
    assert!(config.verify_dependencies);
    assert!(config.max_depth.is_none());
}
#[test]
fn test_lazy_verification_config_changed_only() {
    let config = LazyVerificationConfig::changed_only();
    assert!(config.verify_changed_only);
    assert!(!config.verify_dependencies);
}
#[test]
fn test_lazy_verification_config_with_depth() {
    let config = LazyVerificationConfig::with_depth(3);
    assert_eq!(config.max_depth, Some(3));
}
#[test]
fn test_lazy_verify_empty() {
    let statutes = vec![Statute::new(
        "s1",
        "Test",
        Effect::new(EffectType::Grant, "Test"),
    )];
    let changed_ids: Vec<String> = vec![];
    let config = LazyVerificationConfig::new();
    let result = lazy_verify(&statutes, &changed_ids, config);
    assert!(result.passed);
}
#[test]
fn test_lazy_verify_single_change() {
    let statute = Statute::new("s1", "Test", Effect::new(EffectType::Grant, "Test"));
    let statutes = vec![statute];
    let changed_ids = vec!["s1".to_string()];
    let config = LazyVerificationConfig::changed_only();
    let result = lazy_verify(&statutes, &changed_ids, config);
    assert!(result.passed);
}
#[test]
fn test_verification_diff_no_changes() {
    let old = VerificationResult::pass();
    let new = VerificationResult::pass();
    let diff = VerificationDiff::diff(&old, &new);
    assert!(!diff.has_changes());
    assert!(!diff.status_changed);
}
#[test]
fn test_verification_diff_status_change() {
    let old = VerificationResult::pass();
    let mut new = VerificationResult::pass();
    new.passed = false;
    let diff = VerificationDiff::diff(&old, &new);
    assert!(diff.has_changes());
    assert!(diff.status_changed);
    assert!(diff.old_passed);
    assert!(!diff.new_passed);
}
#[test]
fn test_verification_diff_errors_added() {
    let old = VerificationResult::pass();
    let mut new = VerificationResult::pass();
    new.errors.push(VerificationError::Ambiguity {
        message: "Test".to_string(),
    });
    new.passed = false;
    let diff = VerificationDiff::diff(&old, &new);
    assert_eq!(diff.errors_added.len(), 1);
    assert_eq!(diff.errors_removed.len(), 0);
}
#[test]
fn test_verification_diff_errors_removed() {
    let mut old = VerificationResult::pass();
    old.errors.push(VerificationError::Ambiguity {
        message: "Test".to_string(),
    });
    old.passed = false;
    let new = VerificationResult::pass();
    let diff = VerificationDiff::diff(&old, &new);
    assert_eq!(diff.errors_added.len(), 0);
    assert_eq!(diff.errors_removed.len(), 1);
}
#[test]
fn test_verification_diff_warnings_added() {
    let old = VerificationResult::pass();
    let mut new = VerificationResult::pass();
    new.warnings.push("New warning".to_string());
    let diff = VerificationDiff::diff(&old, &new);
    assert_eq!(diff.warnings_added.len(), 1);
    assert!(diff.warnings_added.contains(&"New warning".to_string()));
}
#[test]
fn test_verification_diff_report() {
    let old = VerificationResult::pass();
    let mut new = VerificationResult::pass();
    new.passed = false;
    let diff = VerificationDiff::diff(&old, &new);
    let report = diff.report();
    assert!(report.contains("Verification Diff Report"));
    assert!(report.contains("Status Changed"));
}
#[test]
fn test_cached_proof_creation() {
    let statute = Statute::new("s1", "Test", Effect::new(EffectType::Grant, "Test"));
    let result = VerificationResult::pass();
    let proof = CachedProof::new(&statute, result);
    assert_eq!(proof.statute_id, "s1");
    assert!(proof.result.passed);
    assert!(!proof.content_hash.is_empty());
}
#[test]
fn test_cached_proof_is_valid() {
    let statute = Statute::new("s1", "Test", Effect::new(EffectType::Grant, "Test"));
    let result = VerificationResult::pass();
    let proof = CachedProof::new(&statute, result);
    assert!(proof.is_valid(&statute));
}
#[test]
fn test_cached_proof_invalid_after_change() {
    let statute = Statute::new("s1", "Test", Effect::new(EffectType::Grant, "Test"));
    let result = VerificationResult::pass();
    let proof = CachedProof::new(&statute, result);
    let mut changed_statute = statute.clone();
    changed_statute.title = "Changed Title".to_string();
    assert!(!proof.is_valid(&changed_statute));
}
#[test]
fn test_proof_cache_creation() {
    let cache = ProofCache::new();
    assert_eq!(cache.proofs.len(), 0);
}
#[test]
fn test_proof_cache_add_proof() {
    let mut cache = ProofCache::new();
    let statute = Statute::new("s1", "Test", Effect::new(EffectType::Grant, "Test"));
    let result = VerificationResult::pass();
    cache.add_proof(&statute, result);
    assert_eq!(cache.proofs.len(), 1);
}
#[test]
fn test_proof_cache_get_proof() {
    let mut cache = ProofCache::new();
    let statute = Statute::new("s1", "Test", Effect::new(EffectType::Grant, "Test"));
    let result = VerificationResult::pass();
    cache.add_proof(&statute, result);
    let cached = cache.get_proof(&statute);
    assert!(cached.is_some());
    assert_eq!(cached.unwrap().statute_id, "s1");
}
#[test]
fn test_proof_cache_invalidate() {
    let mut cache = ProofCache::new();
    let statute = Statute::new("s1", "Test", Effect::new(EffectType::Grant, "Test"));
    let result = VerificationResult::pass();
    cache.add_proof(&statute, result);
    assert_eq!(cache.proofs.len(), 1);
    cache.invalidate(&["s1".to_string()]);
    assert_eq!(cache.proofs.len(), 0);
}
#[test]
fn test_proof_cache_stats() {
    let mut cache = ProofCache::new();
    let statute = Statute::new("s1", "Test", Effect::new(EffectType::Grant, "Test"));
    let result = VerificationResult::pass();
    cache.add_proof(&statute, result);
    let stats = cache.stats();
    assert_eq!(stats.total_proofs, 1);
    assert!(stats.oldest_timestamp.is_some());
    assert!(stats.newest_timestamp.is_some());
}
