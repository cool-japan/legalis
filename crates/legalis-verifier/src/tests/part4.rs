use legalis_core::Statute;

use super::super::types::{
    MarkovTransition, NotificationMessage, RiskLevel, StatuteVerifier, Strategy,
};
use super::super::types_3::{
    Coalition, ConflictNature, MechanismAnalysis, MechanismProperty, NotificationChannel,
    NotificationType, RiskFactor, RiskQuantification, Stakeholder, StatisticalCheckResult,
};
use super::super::types_4::{GameTheoreticModel, MarkovChain, Severity};
use super::super::types_5::{
    GameOutcome, MarkovState, MechanismIssue, NotificationConfig, StakeholderConflict,
    VerificationError, VerificationResult,
};

use super::super::*;
use legalis_core::{ComparisonOp, Condition, Effect, EffectType};

#[test]
fn test_notification_config_webhooks() {
    let config = NotificationConfig::new().with_webhook("https://example.com/webhook");
    assert_eq!(config.channels.len(), 1);
    match &config.channels[0] {
        NotificationChannel::Webhook { url, .. } => {
            assert_eq!(url, "https://example.com/webhook");
        }
        _ => panic!("Expected webhook channel"),
    }
}
#[test]
fn test_notification_config_email() {
    let config = NotificationConfig::new()
        .with_email(vec!["test@example.com".to_string()], "Verification Alert");
    assert_eq!(config.channels.len(), 1);
    match &config.channels[0] {
        NotificationChannel::Email { to, subject } => {
            assert_eq!(to.len(), 1);
            assert_eq!(subject, "Verification Alert");
        }
        _ => panic!("Expected email channel"),
    }
}
#[test]
fn test_notification_config_trigger() {
    let config = NotificationConfig::new()
        .trigger_on(vec![NotificationType::Success, NotificationType::Warning]);
    assert_eq!(config.trigger_on.len(), 2);
    assert!(config.trigger_on.contains(&NotificationType::Success));
    assert!(config.trigger_on.contains(&NotificationType::Warning));
}
#[test]
fn test_notification_config_details() {
    let config = NotificationConfig::new().include_details(false);
    assert!(!config.include_details);
}
#[test]
fn test_notification_config_default() {
    let config = NotificationConfig::default();
    assert!(config.channels.is_empty());
}
#[test]
fn test_notification_message_creation() {
    let message = NotificationMessage::new(
        NotificationType::Success,
        "Verification Passed",
        "All statutes verified successfully",
    );
    assert_eq!(message.notification_type, NotificationType::Success);
    assert_eq!(message.title, "Verification Passed");
    assert_eq!(message.message, "All statutes verified successfully");
    assert!(!message.timestamp.is_empty());
    assert!(message.results.is_none());
}
#[test]
fn test_notification_message_with_results() {
    let results = vec![VerificationResult::pass()];
    let message = NotificationMessage::new(NotificationType::Success, "Test", "Test message")
        .with_results(results);
    assert!(message.results.is_some());
    assert_eq!(message.results.as_ref().unwrap().len(), 1);
}
#[test]
fn test_notification_message_to_json() {
    let message = NotificationMessage::new(
        NotificationType::Error,
        "Verification Failed",
        "Errors found",
    );
    let json = message.to_json();
    assert!(json.contains("\"notification_type\":"));
    assert!(json.contains("\"title\":"));
    assert!(json.contains("\"message\":"));
}
#[test]
fn test_send_notification_no_trigger() {
    let config = NotificationConfig::new().with_webhook("https://example.com");
    let message = NotificationMessage::new(NotificationType::Success, "Test", "Message");
    assert!(!send_notification(&config, &message));
}
#[test]
fn test_send_notification_with_trigger() {
    let config = NotificationConfig::new().with_webhook("https://example.com");
    let message = NotificationMessage::new(NotificationType::Error, "Test", "Message");
    assert!(send_notification(&config, &message));
}
#[test]
fn test_send_notification_no_channels() {
    let config = NotificationConfig::new();
    let message = NotificationMessage::new(NotificationType::Error, "Test", "Message");
    assert!(!send_notification(&config, &message));
}
#[test]
fn test_stakeholder_creation() {
    let stakeholder = Stakeholder::new("S1", "Alice")
        .with_type("individual")
        .with_interest("privacy")
        .with_interest("fairness")
        .affected_by_statute("statute-1");
    assert_eq!(stakeholder.id, "S1");
    assert_eq!(stakeholder.name, "Alice");
    assert_eq!(stakeholder.stakeholder_type, "individual");
    assert_eq!(stakeholder.interests.len(), 2);
    assert_eq!(stakeholder.affected_by.len(), 1);
}
#[test]
fn test_analyze_stakeholder_conflicts_prohibition() {
    let stakeholders = vec![
        Stakeholder::new("S1", "Alice")
            .with_interest("freedom")
            .affected_by_statute("statute-1"),
        Stakeholder::new("S2", "Bob")
            .with_interest("security")
            .affected_by_statute("statute-1"),
    ];
    let statutes = vec![Statute::new(
        "statute-1",
        "Prohibition Law",
        Effect::new(EffectType::Prohibition, "Prohibit certain actions"),
    )];
    let conflicts = analyze_stakeholder_conflicts(&stakeholders, &statutes);
    assert!(!conflicts.is_empty());
    assert_eq!(conflicts[0].conflict_type, ConflictNature::DirectOpposition);
    assert_eq!(conflicts[0].severity, Severity::Warning);
}
#[test]
fn test_analyze_stakeholder_conflicts_grant() {
    let stakeholders = vec![
        Stakeholder::new("S1", "Alice").affected_by_statute("statute-1"),
        Stakeholder::new("S2", "Bob").affected_by_statute("statute-1"),
    ];
    let statutes = vec![Statute::new(
        "statute-1",
        "Grant Law",
        Effect::new(EffectType::Grant, "Grant benefits"),
    )];
    let conflicts = analyze_stakeholder_conflicts(&stakeholders, &statutes);
    assert!(!conflicts.is_empty());
    assert!(
        conflicts
            .iter()
            .any(|c| c.conflict_type == ConflictNature::ResourceCompetition)
    );
}
#[test]
fn test_analyze_stakeholder_conflicts_conflicting_interests() {
    let stakeholders = vec![
        Stakeholder::new("S1", "Alice")
            .with_interest("privacy")
            .affected_by_statute("statute-1"),
        Stakeholder::new("S2", "Bob")
            .with_interest("transparency")
            .affected_by_statute("statute-1"),
    ];
    let statutes = vec![Statute::new(
        "statute-1",
        "Data Law",
        Effect::new(EffectType::Grant, "Grant access"),
    )];
    let conflicts = analyze_stakeholder_conflicts(&stakeholders, &statutes);
    assert!(
        conflicts
            .iter()
            .any(|c| c.conflict_type == ConflictNature::InterpretationDifference)
    );
}
#[test]
fn test_stakeholder_conflict_report() {
    let conflicts = vec![StakeholderConflict {
        stakeholders: vec!["S1".to_string(), "S2".to_string()],
        statutes: vec!["statute-1".to_string()],
        conflict_type: ConflictNature::DirectOpposition,
        severity: Severity::Warning,
        description: "Test conflict".to_string(),
        resolutions: vec!["Resolution 1".to_string(), "Resolution 2".to_string()],
    }];
    let report = stakeholder_conflict_report(&conflicts);
    assert!(report.contains("Multi-Stakeholder Conflict Analysis"));
    assert!(report.contains("Direct Opposition"));
    assert!(report.contains("Test conflict"));
    assert!(report.contains("Resolution 1"));
}
#[test]
fn test_stakeholder_conflict_report_empty() {
    let conflicts = vec![];
    let report = stakeholder_conflict_report(&conflicts);
    assert!(report.contains("No stakeholder conflicts detected"));
}
#[test]
fn test_strategy_creation() {
    let strategy = Strategy::new("S1", "Full Compliance")
        .with_description("Comply with all laws")
        .with_statute_action("statute-1")
        .with_statute_action("statute-2");
    assert_eq!(strategy.stakeholder_id, "S1");
    assert_eq!(strategy.name, "Full Compliance");
    assert_eq!(strategy.description, "Comply with all laws");
    assert_eq!(strategy.statute_actions.len(), 2);
}
#[test]
fn test_game_theoretic_model_creation() {
    let mut model = GameTheoreticModel::new(vec!["S1".to_string(), "S2".to_string()]);
    assert_eq!(model.stakeholders.len(), 2);
    assert_eq!(model.strategies.len(), 2);
    assert_eq!(model.outcomes.len(), 0);
    let strategy1 = Strategy::new("S1", "Comply");
    model.add_strategy(0, strategy1);
    assert_eq!(model.strategies[0].len(), 1);
}
#[test]
fn test_detect_nash_equilibria() {
    let mut model = GameTheoreticModel::new(vec!["S1".to_string(), "S2".to_string()]);
    model.add_outcome(GameOutcome {
        strategies: vec!["Comply".to_string(), "Comply".to_string()],
        payoffs: vec![5, 5],
        is_nash_equilibrium: true,
        description: "Both comply".to_string(),
    });
    model.add_outcome(GameOutcome {
        strategies: vec!["Comply".to_string(), "Defect".to_string()],
        payoffs: vec![2, 7],
        is_nash_equilibrium: false,
        description: "Asymmetric".to_string(),
    });
    let equilibria = detect_nash_equilibria(&model);
    assert_eq!(equilibria.len(), 1);
    assert_eq!(equilibria[0].payoffs, vec![5, 5]);
}
#[test]
fn test_predict_game_outcomes_two_players() {
    let stakeholders = vec![
        Stakeholder::new("S1", "Alice").affected_by_statute("statute-1"),
        Stakeholder::new("S2", "Bob").affected_by_statute("statute-1"),
    ];
    let statutes = vec![Statute::new(
        "statute-1",
        "Test Law",
        Effect::new(EffectType::Grant, "Grant benefit"),
    )];
    let model = predict_game_outcomes(&stakeholders, &statutes);
    assert_eq!(model.stakeholders.len(), 2);
    assert_eq!(model.strategies.len(), 2);
    assert!(!model.strategies[0].is_empty());
    assert!(!model.strategies[1].is_empty());
    assert_eq!(model.outcomes.len(), 4);
    let equilibria = detect_nash_equilibria(&model);
    assert_eq!(equilibria.len(), 2);
}
#[test]
fn test_game_theoretic_report() {
    let mut model = GameTheoreticModel::new(vec!["S1".to_string(), "S2".to_string()]);
    model.add_strategy(
        0,
        Strategy::new("S1", "Comply").with_description("Full compliance"),
    );
    model.add_strategy(
        1,
        Strategy::new("S2", "Comply").with_description("Full compliance"),
    );
    model.add_outcome(GameOutcome {
        strategies: vec!["Comply".to_string(), "Comply".to_string()],
        payoffs: vec![5, 5],
        is_nash_equilibrium: true,
        description: "Both comply equilibrium".to_string(),
    });
    let report = game_theoretic_report(&model);
    assert!(report.contains("Game-Theoretic Outcome Prediction"));
    assert!(report.contains("Nash Equilibria"));
    assert!(report.contains("Full compliance"));
    assert!(report.contains("Equilibrium 1"));
}
#[test]
fn test_coalition_creation() {
    let coalition = Coalition::new(vec!["S1".to_string(), "S2".to_string()])
        .with_objective("Privacy protection")
        .with_collective_effect("Influence statute-1")
        .with_strength(0.75)
        .with_stability(true);
    assert_eq!(coalition.members.len(), 2);
    assert_eq!(coalition.objectives.len(), 1);
    assert_eq!(coalition.collective_effects.len(), 1);
    assert_eq!(coalition.strength, 0.75);
    assert!(coalition.is_stable);
}
#[test]
fn test_coalition_strength_clamping() {
    let coalition1 = Coalition::new(vec!["S1".to_string()]).with_strength(1.5);
    assert_eq!(coalition1.strength, 1.0);
    let coalition2 = Coalition::new(vec!["S1".to_string()]).with_strength(-0.5);
    assert_eq!(coalition2.strength, 0.0);
}
#[test]
fn test_analyze_coalitions() {
    let stakeholders = vec![
        Stakeholder::new("S1", "Alice")
            .with_interest("privacy")
            .affected_by_statute("statute-1"),
        Stakeholder::new("S2", "Bob")
            .with_interest("privacy")
            .affected_by_statute("statute-1"),
        Stakeholder::new("S3", "Carol")
            .with_interest("security")
            .affected_by_statute("statute-2"),
    ];
    let statutes = vec![
        Statute::new(
            "statute-1",
            "Privacy Law",
            Effect::new(EffectType::Grant, "Grant privacy rights"),
        ),
        Statute::new(
            "statute-2",
            "Security Law",
            Effect::new(EffectType::Grant, "Grant security"),
        ),
    ];
    let coalitions = analyze_coalitions(&stakeholders, &statutes);
    assert!(!coalitions.is_empty());
    let privacy_coalition = coalitions
        .iter()
        .find(|c| c.objectives.contains(&"privacy".to_string()));
    assert!(privacy_coalition.is_some());
    assert_eq!(privacy_coalition.unwrap().members.len(), 2);
}
#[test]
fn test_analyze_coalitions_stable() {
    let stakeholders = vec![
        Stakeholder::new("S1", "Alice")
            .with_interest("education")
            .affected_by_statute("statute-1")
            .affected_by_statute("statute-2"),
        Stakeholder::new("S2", "Bob")
            .with_interest("education")
            .affected_by_statute("statute-1")
            .affected_by_statute("statute-2"),
    ];
    let statutes = vec![
        Statute::new(
            "statute-1",
            "Education Law 1",
            Effect::new(EffectType::Grant, "Grant education"),
        ),
        Statute::new(
            "statute-2",
            "Education Law 2",
            Effect::new(EffectType::Grant, "Grant education"),
        ),
    ];
    let coalitions = analyze_coalitions(&stakeholders, &statutes);
    assert!(!coalitions.is_empty());
    assert!(coalitions[0].is_stable);
}
#[test]
fn test_coalition_analysis_report() {
    let coalitions = vec![
        Coalition::new(vec!["S1".to_string(), "S2".to_string()])
            .with_objective("Privacy")
            .with_collective_effect("Effect 1")
            .with_strength(0.8)
            .with_stability(true),
        Coalition::new(vec!["S3".to_string(), "S4".to_string()])
            .with_objective("Security")
            .with_strength(0.5)
            .with_stability(false),
    ];
    let report = coalition_analysis_report(&coalitions);
    assert!(report.contains("Coalition Analysis"));
    assert!(report.contains("**Total Coalitions Detected**: 2"));
    assert!(report.contains("**Stable Coalitions**: 1"));
    assert!(report.contains("**Unstable Coalitions**: 1"));
    assert!(report.contains("Privacy"));
    assert!(report.contains("Security"));
}
#[test]
fn test_coalition_analysis_report_empty() {
    let coalitions = vec![];
    let report = coalition_analysis_report(&coalitions);
    assert!(report.contains("No coalitions detected"));
    assert!(report.contains("divergent interests"));
}
#[test]
fn test_coalition_sorting_by_strength() {
    let stakeholders = vec![
        Stakeholder::new("S1", "Alice")
            .with_interest("privacy")
            .affected_by_statute("statute-1")
            .affected_by_statute("statute-2"),
        Stakeholder::new("S2", "Bob")
            .with_interest("privacy")
            .affected_by_statute("statute-1"),
        Stakeholder::new("S3", "Carol")
            .with_interest("security")
            .affected_by_statute("statute-3"),
        Stakeholder::new("S4", "Dave")
            .with_interest("security")
            .affected_by_statute("statute-3"),
    ];
    let statutes = vec![
        Statute::new(
            "statute-1",
            "Privacy Law 1",
            Effect::new(EffectType::Grant, "Grant"),
        ),
        Statute::new(
            "statute-2",
            "Privacy Law 2",
            Effect::new(EffectType::Grant, "Grant"),
        ),
        Statute::new(
            "statute-3",
            "Security Law",
            Effect::new(EffectType::Grant, "Grant"),
        ),
    ];
    let coalitions = analyze_coalitions(&stakeholders, &statutes);
    for i in 1..coalitions.len() {
        assert!(coalitions[i - 1].strength >= coalitions[i].strength);
    }
}
#[test]
fn test_mechanism_analysis_creation() {
    let analysis = MechanismAnalysis::new();
    assert!(analysis.issues.is_empty());
    assert!(analysis.satisfied_properties.is_empty());
    assert_eq!(analysis.quality_score, 1.0);
}
#[test]
fn test_mechanism_analysis_add_issue() {
    let mut analysis = MechanismAnalysis::new();
    analysis.add_issue(MechanismIssue {
        property: MechanismProperty::IncentiveCompatibility,
        statute_ids: vec!["S1".to_string()],
        severity: Severity::Warning,
        description: "Test issue".to_string(),
        suggestions: vec!["Fix it".to_string()],
    });
    assert_eq!(analysis.issues.len(), 1);
    assert!(analysis.quality_score < 1.0);
}
#[test]
fn test_mechanism_analysis_satisfy_property() {
    let mut analysis = MechanismAnalysis::new();
    analysis.satisfy_property(MechanismProperty::IncentiveCompatibility);
    analysis.satisfy_property(MechanismProperty::BudgetBalance);
    assert_eq!(analysis.satisfied_properties.len(), 2);
}
#[test]
fn test_mechanism_design_incentive_compatibility_violation() {
    let stakeholders = vec![Stakeholder::new("S1", "Alice")];
    let statutes = vec![Statute::new(
        "statute-1",
        "Prohibition Law",
        Effect::new(EffectType::Prohibition, "Prohibit action"),
    )];
    let analysis = verify_mechanism_design(&statutes, &stakeholders);
    assert!(
        analysis
            .issues
            .iter()
            .any(|i| i.property == MechanismProperty::IncentiveCompatibility)
    );
}
#[test]
fn test_mechanism_design_incentive_compatibility_satisfied() {
    let stakeholders = vec![Stakeholder::new("S1", "Alice")];
    let statutes = vec![
        Statute::new(
            "statute-1",
            "Grant Law",
            Effect::new(EffectType::Grant, "Grant benefit"),
        )
        .with_discretion("Comply to receive benefit")
        .with_precondition(Condition::Age {
            operator: ComparisonOp::GreaterOrEqual,
            value: 18,
        }),
    ];
    let analysis = verify_mechanism_design(&statutes, &stakeholders);
    assert!(
        analysis
            .satisfied_properties
            .contains(&MechanismProperty::IncentiveCompatibility)
    );
}
#[test]
fn test_mechanism_design_individual_rationality_violation() {
    let stakeholders = vec![
        Stakeholder::new("S1", "Alice")
            .affected_by_statute("statute-1")
            .affected_by_statute("statute-2"),
    ];
    let statutes = vec![
        Statute::new(
            "statute-1",
            "Prohibition 1",
            Effect::new(EffectType::Prohibition, "Prohibit A"),
        ),
        Statute::new(
            "statute-2",
            "Prohibition 2",
            Effect::new(EffectType::Revoke, "Revoke B"),
        ),
    ];
    let analysis = verify_mechanism_design(&statutes, &stakeholders);
    assert!(
        analysis
            .issues
            .iter()
            .any(|i| i.property == MechanismProperty::IndividualRationality)
    );
}
#[test]
fn test_mechanism_design_individual_rationality_satisfied() {
    let stakeholders = vec![
        Stakeholder::new("S1", "Alice")
            .affected_by_statute("statute-1")
            .affected_by_statute("statute-2"),
    ];
    let statutes = vec![
        Statute::new(
            "statute-1",
            "Grant Law",
            Effect::new(EffectType::Grant, "Grant benefit"),
        ),
        Statute::new(
            "statute-2",
            "Prohibition Law",
            Effect::new(EffectType::Prohibition, "Prohibit action"),
        ),
    ];
    let analysis = verify_mechanism_design(&statutes, &stakeholders);
    assert!(
        analysis
            .satisfied_properties
            .contains(&MechanismProperty::IndividualRationality)
    );
}
#[test]
fn test_mechanism_design_budget_balance_no_transfers() {
    let stakeholders = vec![Stakeholder::new("S1", "Alice")];
    let statutes = vec![Statute::new(
        "statute-1",
        "Grant Law",
        Effect::new(EffectType::Grant, "Grant benefit"),
    )];
    let analysis = verify_mechanism_design(&statutes, &stakeholders);
    assert!(
        analysis
            .satisfied_properties
            .contains(&MechanismProperty::BudgetBalance)
    );
}
#[test]
fn test_mechanism_design_budget_balance_with_transfers() {
    let stakeholders = vec![Stakeholder::new("S1", "Alice")];
    let statutes = vec![
        Statute::new(
            "statute-1",
            "Transfer Law",
            Effect::new(EffectType::MonetaryTransfer, "Transfer money"),
        )
        .with_jurisdiction("US"),
    ];
    let analysis = verify_mechanism_design(&statutes, &stakeholders);
    assert!(
        analysis
            .issues
            .iter()
            .any(|i| i.property == MechanismProperty::BudgetBalance)
    );
}
#[test]
fn test_mechanism_design_strategy_proofness_custom_condition() {
    let stakeholders = vec![Stakeholder::new("S1", "Alice")];
    let statutes = vec![
        Statute::new(
            "statute-1",
            "Custom Condition Law",
            Effect::new(EffectType::Grant, "Grant benefit"),
        )
        .with_precondition(Condition::Custom {
            description: "Custom check".to_string(),
        }),
    ];
    let analysis = verify_mechanism_design(&statutes, &stakeholders);
    assert!(
        analysis
            .issues
            .iter()
            .any(|i| i.property == MechanismProperty::StrategyProofness)
    );
}
#[test]
fn test_mechanism_design_strategy_proofness_grant_no_conditions() {
    let stakeholders = vec![Stakeholder::new("S1", "Alice")];
    let statutes = vec![Statute::new(
        "statute-1",
        "Unconditional Grant",
        Effect::new(EffectType::Grant, "Grant benefit"),
    )];
    let analysis = verify_mechanism_design(&statutes, &stakeholders);
    assert!(
        analysis
            .issues
            .iter()
            .any(|i| i.property == MechanismProperty::StrategyProofness)
    );
}
#[test]
fn test_mechanism_design_non_dictatorship_violation() {
    let stakeholders = vec![
        Stakeholder::new("S1", "Alice")
            .affected_by_statute("statute-1")
            .affected_by_statute("statute-2")
            .affected_by_statute("statute-3"),
        Stakeholder::new("S2", "Bob").affected_by_statute("statute-4"),
    ];
    let statutes = vec![
        Statute::new(
            "statute-1",
            "Law 1",
            Effect::new(EffectType::Grant, "Grant"),
        ),
        Statute::new(
            "statute-2",
            "Law 2",
            Effect::new(EffectType::Grant, "Grant"),
        ),
        Statute::new(
            "statute-3",
            "Law 3",
            Effect::new(EffectType::Grant, "Grant"),
        ),
        Statute::new(
            "statute-4",
            "Law 4",
            Effect::new(EffectType::Grant, "Grant"),
        ),
    ];
    let analysis = verify_mechanism_design(&statutes, &stakeholders);
    assert!(
        analysis
            .issues
            .iter()
            .any(|i| i.property == MechanismProperty::NonDictatorship)
    );
}
#[test]
fn test_mechanism_design_non_dictatorship_satisfied() {
    let stakeholders = vec![
        Stakeholder::new("S1", "Alice").affected_by_statute("statute-1"),
        Stakeholder::new("S2", "Bob").affected_by_statute("statute-2"),
    ];
    let statutes = vec![
        Statute::new(
            "statute-1",
            "Law 1",
            Effect::new(EffectType::Grant, "Grant"),
        ),
        Statute::new(
            "statute-2",
            "Law 2",
            Effect::new(EffectType::Grant, "Grant"),
        ),
    ];
    let analysis = verify_mechanism_design(&statutes, &stakeholders);
    assert!(
        analysis
            .satisfied_properties
            .contains(&MechanismProperty::NonDictatorship)
    );
}
#[test]
fn test_mechanism_design_report_no_issues() {
    let analysis = MechanismAnalysis {
        issues: vec![],
        satisfied_properties: vec![
            MechanismProperty::IncentiveCompatibility,
            MechanismProperty::BudgetBalance,
        ],
        quality_score: 1.0,
    };
    let report = mechanism_design_report(&analysis);
    assert!(report.contains("Mechanism Design Analysis"));
    assert!(report.contains("No mechanism design issues detected"));
    assert!(report.contains("Incentive Compatibility"));
}
#[test]
fn test_mechanism_design_report_with_issues() {
    let mut analysis = MechanismAnalysis::new();
    analysis.add_issue(MechanismIssue {
        property: MechanismProperty::IncentiveCompatibility,
        statute_ids: vec!["S1".to_string()],
        severity: Severity::Warning,
        description: "Test issue".to_string(),
        suggestions: vec!["Fix suggestion".to_string()],
    });
    let report = mechanism_design_report(&analysis);
    assert!(report.contains("Mechanism Design Analysis"));
    assert!(report.contains("Incentive Compatibility"));
    assert!(report.contains("Test issue"));
    assert!(report.contains("Fix suggestion"));
}
#[test]
fn test_mechanism_property_display() {
    assert_eq!(
        MechanismProperty::IncentiveCompatibility.to_string(),
        "Incentive Compatibility"
    );
    assert_eq!(
        MechanismProperty::IndividualRationality.to_string(),
        "Individual Rationality"
    );
    assert_eq!(
        MechanismProperty::BudgetBalance.to_string(),
        "Budget Balance"
    );
    assert_eq!(
        MechanismProperty::StrategyProofness.to_string(),
        "Strategy-Proofness"
    );
    assert_eq!(
        MechanismProperty::NonDictatorship.to_string(),
        "Non-Dictatorship"
    );
}
#[test]
fn test_mechanism_quality_score_calculation() {
    let mut analysis = MechanismAnalysis::new();
    analysis.add_issue(MechanismIssue {
        property: MechanismProperty::IncentiveCompatibility,
        statute_ids: vec![],
        severity: Severity::Critical,
        description: "Critical".to_string(),
        suggestions: vec![],
    });
    assert!(analysis.quality_score <= 0.7);
    analysis.satisfy_property(MechanismProperty::BudgetBalance);
    analysis.satisfy_property(MechanismProperty::NonDictatorship);
    assert!(analysis.quality_score > 0.0);
}
#[test]
fn test_markov_state_creation() {
    let state = MarkovState::new("s1", "Initial State").accepting();
    assert_eq!(state.id, "s1");
    assert_eq!(state.description, "Initial State");
    assert!(state.accepting);
}
#[test]
fn test_markov_transition_creation() {
    let transition = MarkovTransition::new("s1", "s2", 0.7).with_action("comply");
    assert_eq!(transition.from, "s1");
    assert_eq!(transition.to, "s2");
    assert_eq!(transition.probability, 0.7);
    assert_eq!(transition.action.as_ref().unwrap(), "comply");
}
#[test]
fn test_markov_chain_validation_valid() {
    let chain = MarkovChain::new("test", "s1")
        .add_state(MarkovState::new("s1", "Start"))
        .add_state(MarkovState::new("s2", "End").accepting())
        .add_transition(MarkovTransition::new("s1", "s2", 0.6))
        .add_transition(MarkovTransition::new("s1", "s1", 0.4));
    assert!(chain.validate().is_ok());
}
#[test]
fn test_markov_chain_validation_invalid() {
    let chain = MarkovChain::new("test", "s1")
        .add_state(MarkovState::new("s1", "Start"))
        .add_state(MarkovState::new("s2", "End"))
        .add_transition(MarkovTransition::new("s1", "s2", 0.3));
    assert!(chain.validate().is_err());
}
#[test]
fn test_markov_chain_reachability_probability() {
    let chain = MarkovChain::new("test", "s1")
        .add_state(MarkovState::new("s1", "Start"))
        .add_state(MarkovState::new("s2", "Accepting").accepting())
        .add_transition(MarkovTransition::new("s1", "s2", 1.0));
    let prob = chain.reachability_probability(5);
    assert!((prob - 1.0).abs() < 0.01);
}
#[test]
fn test_markov_chain_steady_state() {
    let chain = MarkovChain::new("test", "s1")
        .add_state(MarkovState::new("s1", "State 1"))
        .add_state(MarkovState::new("s2", "State 2"))
        .add_transition(MarkovTransition::new("s1", "s2", 0.5))
        .add_transition(MarkovTransition::new("s1", "s1", 0.5))
        .add_transition(MarkovTransition::new("s2", "s1", 0.5))
        .add_transition(MarkovTransition::new("s2", "s2", 0.5));
    let probs = chain.steady_state_probabilities(100);
    let p1 = probs.get("s1").copied().unwrap_or(0.0);
    let p2 = probs.get("s2").copied().unwrap_or(0.0);
    assert!((p1 - 0.5).abs() < 0.1);
    assert!((p2 - 0.5).abs() < 0.1);
}
#[test]
fn test_statistical_check_result_from_samples() {
    let result = StatisticalCheckResult::from_samples("test property", 1000, 750, 0.7);
    assert_eq!(result.num_samples, 1000);
    assert_eq!(result.num_successes, 750);
    assert!((result.estimated_probability - 0.75).abs() < 0.01);
    assert!(result.hypothesis_accepted);
}
#[test]
fn test_statistical_check_result_hypothesis_rejected() {
    let result = StatisticalCheckResult::from_samples("test property", 1000, 400, 0.5);
    assert_eq!(result.num_samples, 1000);
    assert_eq!(result.num_successes, 400);
    assert!((result.estimated_probability - 0.4).abs() < 0.01);
    assert!(!result.hypothesis_accepted);
}
#[test]
fn test_risk_level_from_score() {
    assert_eq!(RiskLevel::from_score(0.1), RiskLevel::Minimal);
    assert_eq!(RiskLevel::from_score(0.3), RiskLevel::Low);
    assert_eq!(RiskLevel::from_score(0.6), RiskLevel::Medium);
    assert_eq!(RiskLevel::from_score(0.8), RiskLevel::High);
    assert_eq!(RiskLevel::from_score(0.95), RiskLevel::Critical);
}
#[test]
fn test_risk_level_display() {
    assert_eq!(RiskLevel::Minimal.to_string(), "Minimal");
    assert_eq!(RiskLevel::Low.to_string(), "Low");
    assert_eq!(RiskLevel::Medium.to_string(), "Medium");
    assert_eq!(RiskLevel::High.to_string(), "High");
    assert_eq!(RiskLevel::Critical.to_string(), "Critical");
}
#[test]
fn test_risk_factor_creation() {
    let factor = RiskFactor::new("Test Risk", "Description", 0.7).with_weight(0.5);
    assert_eq!(factor.name, "Test Risk");
    assert_eq!(factor.description, "Description");
    assert_eq!(factor.score, 0.7);
    assert_eq!(factor.weight, 0.5);
}
#[test]
fn test_risk_factor_score_clamping() {
    let factor = RiskFactor::new("Test", "Desc", 1.5);
    assert_eq!(factor.score, 1.0);
    let factor2 = RiskFactor::new("Test", "Desc", -0.5);
    assert_eq!(factor2.score, 0.0);
}
#[test]
fn test_risk_quantification_creation() {
    let factors = vec![
        RiskFactor::new("Factor 1", "Desc 1", 0.5).with_weight(0.5),
        RiskFactor::new("Factor 2", "Desc 2", 0.9).with_weight(0.5),
    ];
    let quant = RiskQuantification::new("statute-1", factors);
    assert_eq!(quant.statute_id, "statute-1");
    assert_eq!(quant.factors.len(), 2);
    assert!((quant.overall_score - 0.7).abs() < 0.01);
    assert_eq!(quant.risk_level, RiskLevel::Medium);
}
#[test]
fn test_risk_quantification_with_mitigations() {
    let factors = vec![RiskFactor::new("Test", "Desc", 0.95)];
    let quant = RiskQuantification::new("statute-1", factors)
        .add_mitigation("Mitigation 1")
        .add_mitigation("Mitigation 2");
    assert_eq!(quant.mitigations.len(), 2);
    assert_eq!(quant.mitigations[0], "Mitigation 1");
}
#[test]
fn test_analyze_statute_risk_simple() {
    let statute = Statute::new(
        "test-1",
        "Simple Statute",
        Effect::new(EffectType::Grant, "Test effect"),
    )
    .with_precondition(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 18,
    });
    let verifier = StatuteVerifier::new();
    let result = verifier.verify(std::slice::from_ref(&statute));
    let risk = analyze_statute_risk(&statute, &result);
    assert_eq!(risk.statute_id, "test-1");
    assert_eq!(risk.factors.len(), 4);
    assert!(risk.overall_score >= 0.0 && risk.overall_score <= 1.0);
    assert!(!risk.mitigations.is_empty());
}
#[test]
fn test_analyze_statute_risk_with_errors() {
    let statute = Statute::new(
        "test-1",
        "Statute with Issues",
        Effect::new(EffectType::Prohibition, "Prohibit something"),
    );
    let mut result = VerificationResult::pass();
    result.errors.push(VerificationError::CircularReference {
        message: "Test error".to_string(),
    });
    result.passed = false;
    let risk = analyze_statute_risk(&statute, &result);
    assert!(risk.overall_score > 0.3);
    assert!(!risk.mitigations.is_empty());
}
#[test]
fn test_risk_quantification_report() {
    let factors = vec![RiskFactor::new("Test Factor", "Description", 0.6)];
    let risk = RiskQuantification::new("statute-1", factors).add_mitigation("Fix issue");
    let report = risk_quantification_report(&[risk]);
    assert!(report.contains("Risk Quantification Report"));
    assert!(report.contains("statute-1"));
    assert!(report.contains("Test Factor"));
    assert!(report.contains("Fix issue"));
}
#[test]
fn test_statistical_model_checking_report() {
    let results = vec![
        StatisticalCheckResult::from_samples("Property 1", 1000, 800, 0.75),
        StatisticalCheckResult::from_samples("Property 2", 500, 250, 0.5),
    ];
    let report = statistical_model_checking_report(&results);
    assert!(report.contains("Statistical Model Checking Report"));
    assert!(report.contains("Property 1"));
    assert!(report.contains("Property 2"));
    assert!(report.contains("ACCEPTED"));
}
#[test]
fn test_monte_carlo_verification_simple() {
    let chain = MarkovChain::new("test", "s1")
        .add_state(MarkovState::new("s1", "Start"))
        .add_state(MarkovState::new("s2", "Accept").accepting())
        .add_transition(MarkovTransition::new("s1", "s2", 1.0));
    let result = monte_carlo_verification(&chain, 100, 10);
    assert_eq!(result.num_samples, 100);
    assert!(result.num_successes > 90);
    assert!(result.estimated_probability > 0.9);
}
#[test]
fn test_monte_carlo_verification_probabilistic() {
    let chain = MarkovChain::new("test", "s1")
        .add_state(MarkovState::new("s1", "Start"))
        .add_state(MarkovState::new("s2", "Accept").accepting())
        .add_state(MarkovState::new("s3", "Reject"))
        .add_transition(MarkovTransition::new("s1", "s2", 0.5))
        .add_transition(MarkovTransition::new("s1", "s3", 0.5));
    let result = monte_carlo_verification(&chain, 1000, 10);
    assert_eq!(result.num_samples, 1000);
    assert!(result.estimated_probability > 0.4 && result.estimated_probability < 0.6);
}
#[test]
fn test_risk_quantification_critical_level() {
    let factors = vec![RiskFactor::new("Critical Factor", "Very high risk", 0.95)];
    let quant = RiskQuantification::new("statute-critical", factors);
    assert_eq!(quant.risk_level, RiskLevel::Critical);
    assert!(quant.overall_score >= 0.9);
}
#[test]
fn test_risk_quantification_minimal_level() {
    let factors = vec![RiskFactor::new("Low Risk Factor", "Very low risk", 0.1)];
    let quant = RiskQuantification::new("statute-safe", factors);
    assert_eq!(quant.risk_level, RiskLevel::Minimal);
    assert!(quant.overall_score < 0.25);
}
