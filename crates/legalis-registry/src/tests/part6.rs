use super::super::*;
use legalis_core::{Effect, EffectType};

fn test_statute(id: &str) -> Statute {
    Statute::new(
        id,
        format!("Test {}", id),
        Effect::new(EffectType::Grant, "Test"),
    )
}

#[test]
fn test_task_status_transitions() {
    use tasks::*;

    let mut task = ReviewTask::new("Task 1", "user1", "manager1", "STAT-1");

    task.start();
    assert_eq!(task.status, TaskStatus::InProgress);

    task.complete();
    assert_eq!(task.status, TaskStatus::Completed);
    assert!(task.completed_at.is_some());
}

#[test]
fn test_task_manager() {
    use tasks::*;

    let mut manager = TaskManager::new();

    let task = ReviewTask::new("Review Task", "user1", "manager1", "STAT-1");
    let id = manager.create_task(task);

    assert!(manager.get_task(id).is_some());

    let user_tasks = manager.tasks_for_user("user1");
    assert_eq!(user_tasks.len(), 1);
}

#[test]
fn test_task_manager_complete() {
    use tasks::*;

    let mut manager = TaskManager::new();

    let task = ReviewTask::new("Task", "user1", "manager1", "STAT-1");
    let id = manager.create_task(task);

    if let Some(task) = manager.get_task_mut(id) {
        task.complete();
    }

    let task = manager.get_task(id).unwrap();
    assert_eq!(task.status, TaskStatus::Completed);
}

#[test]
fn test_task_manager_by_status() {
    use tasks::*;

    let mut manager = TaskManager::new();

    let mut task1 = ReviewTask::new("Task 1", "user1", "manager1", "STAT-1");
    task1.start();
    manager.create_task(task1);

    manager.create_task(ReviewTask::new("Task 2", "user1", "manager1", "STAT-2"));

    let not_started = manager.tasks_by_status(TaskStatus::NotStarted);
    assert_eq!(not_started.len(), 1); // Only one not started
}

#[test]
fn test_sla_definition() {
    use sla::*;

    let sla = SlaDefinition::new(
        "Approval SLA",
        SlaMetric::TimeToApproval,
        3600, // 1 hour
    )
    .with_warning_threshold(0.7);

    assert_eq!(sla.name, "Approval SLA");
    assert_eq!(sla.target_seconds, 3600);
    assert_eq!(sla.warning_threshold, 0.7);

    let target = sla.target_duration();
    assert_eq!(target.num_seconds(), 3600);

    let warning = sla.warning_duration();
    assert_eq!(warning.num_seconds(), 2520); // 70% of 3600
}

#[test]
fn test_sla_measurement() {
    use sla::*;

    let sla = SlaDefinition::new("Test SLA", SlaMetric::TimeToFirstResponse, 100);
    let mut measurement = SlaMeasurement::new(sla.sla_id, "entity-1");

    assert_eq!(measurement.status, SlaStatus::Met);
    assert!(measurement.end_time.is_none());

    measurement.complete(&sla);
    assert!(measurement.end_time.is_some());
    assert!(measurement.duration_seconds.is_some());
}

#[test]
fn test_sla_tracker() {
    use sla::*;

    let mut tracker = SlaTracker::new();

    let sla = SlaDefinition::new("Approval SLA", SlaMetric::TimeToApproval, 3600);
    let sla_id = tracker.add_definition(sla);

    let measurement_id = tracker.start_tracking(sla_id, "request-123");
    assert!(measurement_id != Uuid::nil());

    let result = tracker.complete_measurement(measurement_id);
    assert!(result.is_ok());
}

#[test]
fn test_sla_completion_rate() {
    use sla::*;

    let mut tracker = SlaTracker::new();

    let sla = SlaDefinition::new("Test SLA", SlaMetric::TimeToCompletion, 1000);
    let sla_id = tracker.add_definition(sla);

    let m1 = tracker.start_tracking(sla_id, "e1");
    let m2 = tracker.start_tracking(sla_id, "e2");

    tracker.complete_measurement(m1).ok();
    tracker.complete_measurement(m2).ok();

    let rate = tracker.completion_rate(sla_id);
    assert!((0.0..=1.0).contains(&rate));
}

#[test]
fn test_escalation_rule() {
    use escalation::*;

    let rule = EscalationRule::new(
        "Overdue Escalation",
        EscalationCondition::AfterDuration { seconds: 7200 },
        EscalationAction::EscalateToManager,
    )
    .with_priority(10);

    assert_eq!(rule.name, "Overdue Escalation");
    assert_eq!(rule.priority, 10);
    assert!(rule.enabled);
}

#[test]
fn test_escalation_condition_after_duration() {
    use chrono::Duration;
    use escalation::*;

    let condition = EscalationCondition::AfterDuration { seconds: 60 };

    let old_time = Utc::now() - Duration::seconds(120);
    assert!(condition.is_met(old_time, false));

    let recent_time = Utc::now() - Duration::seconds(30);
    assert!(!condition.is_met(recent_time, false));
}

#[test]
fn test_escalation_manager() {
    use escalation::*;

    let mut manager = EscalationManager::new();

    let rule = EscalationRule::new(
        "Auto Escalate",
        EscalationCondition::AfterDuration { seconds: 3600 },
        EscalationAction::Notify {
            users: vec!["manager1".to_string()],
        },
    );

    manager.add_rule(rule);

    let old_time = Utc::now() - chrono::Duration::seconds(7200);
    let actions = manager.check_escalations("entity-1", old_time, false);

    assert_eq!(actions.len(), 1);
}

#[test]
fn test_escalation_manager_priority() {
    use escalation::*;

    let mut manager = EscalationManager::new();

    let rule1 = EscalationRule::new(
        "Low Priority",
        EscalationCondition::AfterDuration { seconds: 60 },
        EscalationAction::Notify {
            users: vec!["user1".to_string()],
        },
    )
    .with_priority(1);

    let rule2 = EscalationRule::new(
        "High Priority",
        EscalationCondition::AfterDuration { seconds: 60 },
        EscalationAction::EscalateToManager,
    )
    .with_priority(10);

    manager.add_rule(rule1);
    manager.add_rule(rule2);

    let old_time = Utc::now() - chrono::Duration::seconds(120);
    let actions = manager.check_escalations("entity-1", old_time, false);

    // Both should trigger, but order should be by priority
    assert_eq!(actions.len(), 2);
}

#[test]
fn test_workflow_status_variants() {
    use workflow::*;

    let _draft = WorkflowStatus::Draft;
    let _pending = WorkflowStatus::PendingApproval;
    let _approved = WorkflowStatus::Approved;
    let _rejected = WorkflowStatus::Rejected;
    let _cancelled = WorkflowStatus::Cancelled;
}

#[test]
fn test_change_type_variants() {
    use workflow::*;

    let _create = ChangeType::Create;
    let _update = ChangeType::Update {
        statute_id: "S1".to_string(),
    };
    let _delete = ChangeType::Delete {
        statute_id: "S2".to_string(),
    };
    let _status = ChangeType::StatusChange {
        statute_id: "S3".to_string(),
        new_status: StatuteStatus::Active,
    };
    let _bulk = ChangeType::Bulk {
        operation_count: 10,
    };
}

#[test]
fn test_notification_type_variants() {
    use notifications::*;

    let _requested = NotificationType::ApprovalRequested;
    let _granted = NotificationType::ApprovalGranted;
    let _rejected = NotificationType::ApprovalRejected;
    let _assigned = NotificationType::TaskAssigned;
    let _completed = NotificationType::TaskCompleted;
    let _warning = NotificationType::SlaWarning;
    let _breach = NotificationType::SlaBreach;
    let _updated = NotificationType::StatuteUpdated;
    let _custom = NotificationType::Custom("test".to_string());
}

#[test]
fn test_task_status_variants() {
    use tasks::*;

    let _not_started = TaskStatus::NotStarted;
    let _in_progress = TaskStatus::InProgress;
    let _blocked = TaskStatus::Blocked;
    let _completed = TaskStatus::Completed;
    let _cancelled = TaskStatus::Cancelled;
}

#[test]
fn test_sla_metric_variants() {
    use sla::*;

    let _first_response = SlaMetric::TimeToFirstResponse;
    let _approval = SlaMetric::TimeToApproval;
    let _completion = SlaMetric::TimeToCompletion;
    let _custom = SlaMetric::Custom("custom_metric".to_string());
}

#[test]
fn test_escalation_action_variants() {
    use escalation::*;

    let _notify = EscalationAction::Notify {
        users: vec!["u1".to_string()],
    };
    let _reassign = EscalationAction::Reassign {
        to_user: "u2".to_string(),
    };
    let _escalate = EscalationAction::EscalateToManager;
    let _auto_approve = EscalationAction::AutoApprove;
    let _custom = EscalationAction::Custom("custom".to_string());
}

// ========== Advanced Search Tests (v0.1.2) ==========

#[test]
fn test_facet_result() {
    use advanced_search::*;

    let facet = FacetResult {
        facet_type: FacetType::Status,
        values: vec![
            FacetValue {
                value: "Active".to_string(),
                count: 10,
            },
            FacetValue {
                value: "Repealed".to_string(),
                count: 5,
            },
            FacetValue {
                value: "Draft".to_string(),
                count: 3,
            },
        ],
        total_values: 3,
    };

    let top = facet.top_values(2);
    assert_eq!(top.len(), 2);
    assert_eq!(top[0].value, "Active");
    assert_eq!(top[0].count, 10);

    let found = facet.find_value("Repealed");
    assert!(found.is_some());
    assert_eq!(found.unwrap().count, 5);
}

#[test]
fn test_autocomplete_provider() {
    use advanced_search::*;

    let mut provider = AutocompleteProvider::new();
    let mut registry = StatuteRegistry::new();

    registry
        .register(StatuteEntry::new(test_statute("GDPR-2016"), "EU"))
        .ok();
    registry
        .register(StatuteEntry::new(test_statute("CCPA-2018"), "US-CA"))
        .ok();

    for (_, entry) in registry.statutes.iter() {
        provider.index_statute(entry);
    }

    let suggestions = provider.suggest("GDP", 5);
    assert!(!suggestions.is_empty());

    let gdpr_suggestion = suggestions.iter().find(|s| s.text.contains("GDPR"));
    assert!(gdpr_suggestion.is_some());
}

#[test]
fn test_autocomplete_scoring() {
    use advanced_search::*;

    let mut provider = AutocompleteProvider::new();
    let mut registry = StatuteRegistry::new();

    registry
        .register(StatuteEntry::new(test_statute("TEST-123"), "US"))
        .ok();
    registry
        .register(StatuteEntry::new(test_statute("TEST-456"), "US"))
        .ok();
    registry
        .register(StatuteEntry::new(test_statute("EXAMPLE-789"), "US"))
        .ok();

    for (_, entry) in registry.statutes.iter() {
        provider.index_statute(entry);
    }

    let suggestions = provider.suggest("TEST", 10);

    // Exact or prefix matches should score higher
    assert!(suggestions.len() >= 2);
    for suggestion in &suggestions[0..2] {
        assert!(suggestion.text.contains("TEST"));
        assert!(suggestion.score >= 0.5);
    }
}

#[test]
fn test_saved_search() {
    use advanced_search::*;

    let query = SearchQuery::default();
    let search = SavedSearch::new("My Search", query, "user123").with_alert(3600);

    assert_eq!(search.name, "My Search");
    assert_eq!(search.owner, "user123");
    assert!(search.alert_enabled);
    assert_eq!(search.alert_frequency_seconds, Some(3600));
}

#[test]
fn test_saved_search_alert_trigger() {
    use advanced_search::*;

    let query = SearchQuery::default();
    let mut search = SavedSearch::new("Test", query, "user1").with_alert(60);

    // Never executed, should trigger
    assert!(search.should_trigger_alert());

    // Just executed, should not trigger
    search.update_execution(5);
    assert!(!search.should_trigger_alert());
}

#[test]
fn test_search_analytics() {
    use advanced_search::*;

    let mut analytics = SearchAnalytics::new();

    analytics.record_search("test query", 5);
    analytics.record_search("another query", 10);
    analytics.record_search("test query", 3);

    assert_eq!(analytics.total_searches(), 3);

    let top = analytics.top_queries(5);
    assert_eq!(top.len(), 2);
    assert_eq!(top[0].0, "test query");
    assert_eq!(top[0].1, 2);

    let avg = analytics.average_result_count();
    assert!((avg - 6.0).abs() < 0.1); // (5 + 10 + 3) / 3 = 6
}

#[test]
fn test_search_analytics_zero_results() {
    use advanced_search::*;

    let mut analytics = SearchAnalytics::new();

    analytics.record_search("query1", 5);
    analytics.record_search("query2", 0);
    analytics.record_search("query3", 0);

    let zero_results = analytics.zero_result_queries();
    assert_eq!(zero_results.len(), 2);
}

#[test]
fn test_search_analytics_time_range() {
    use advanced_search::*;
    use chrono::Duration;

    let mut analytics = SearchAnalytics::new();

    analytics.record_search("query1", 5);
    analytics.record_search("query2", 10);

    let start = Utc::now() - Duration::seconds(60);
    let end = Utc::now() + Duration::seconds(60);

    let count = analytics.searches_in_range(start, end);
    assert_eq!(count, 2);
}

#[test]
fn test_semantic_search() {
    use advanced_search::*;

    let mut semantic = SemanticSearch::new(768);

    assert_eq!(semantic.dimension(), 768);
    assert!(!semantic.is_enabled());

    semantic.enable();
    assert!(semantic.is_enabled());

    // Placeholder search returns empty (no ML integration yet)
    let results = semantic.search("test query", 10);
    assert!(results.is_empty());
}

#[test]
fn test_semantic_search_default() {
    use advanced_search::*;

    let semantic = SemanticSearch::default();
    assert_eq!(semantic.dimension(), 384); // Default BERT dimension
}

#[test]
fn test_facet_type_variants() {
    use advanced_search::*;

    let _status = FacetType::Status;
    let _jurisdiction = FacetType::Jurisdiction;
    let _tags = FacetType::Tags;
    let _year = FacetType::Year;
    let _month = FacetType::Month;
    let _custom = FacetType::Custom("custom".to_string());
}

#[test]
fn test_suggestion_type_variants() {
    use advanced_search::*;

    let _statute_id = SuggestionType::StatuteId;
    let _title = SuggestionType::Title;
    let _tag = SuggestionType::Tag;
    let _jurisdiction = SuggestionType::Jurisdiction;
    let _term = SuggestionType::Term;
}

#[test]
fn test_faceted_search_result() {
    use advanced_search::*;

    let result = FacetedSearchResult {
        statute_ids: vec!["S1".to_string(), "S2".to_string()],
        facets: vec![FacetResult {
            facet_type: FacetType::Status,
            values: vec![FacetValue {
                value: "Active".to_string(),
                count: 2,
            }],
            total_values: 1,
        }],
        total_matches: 2,
    };

    assert_eq!(result.statute_ids.len(), 2);
    assert_eq!(result.facets.len(), 1);
    assert_eq!(result.total_matches, 2);
}

#[test]
fn test_search_suggestion() {
    use advanced_search::*;

    let suggestion = SearchSuggestion {
        text: "GDPR".to_string(),
        suggestion_type: SuggestionType::StatuteId,
        score: 0.9,
    };

    assert_eq!(suggestion.text, "GDPR");
    assert_eq!(suggestion.suggestion_type, SuggestionType::StatuteId);
    assert!((suggestion.score - 0.9).abs() < 0.01);
}

#[test]
fn test_autocomplete_multiple_types() {
    use advanced_search::*;

    let mut provider = AutocompleteProvider::new();
    let mut registry = StatuteRegistry::new();

    let mut entry = StatuteEntry::new(test_statute("TEST-1"), "TEST-JURISDICTION");
    entry.tags.push("test-tag".to_string());
    registry
        .statutes
        .insert("TEST-1".to_string(), entry.clone());

    provider.index_statute(&entry);

    let suggestions = provider.suggest("test", 10);

    // Should find suggestions from multiple types
    assert!(!suggestions.is_empty());

    let has_id = suggestions
        .iter()
        .any(|s| s.suggestion_type == SuggestionType::StatuteId);
    let has_tag = suggestions
        .iter()
        .any(|s| s.suggestion_type == SuggestionType::Tag);
    let has_jurisdiction = suggestions
        .iter()
        .any(|s| s.suggestion_type == SuggestionType::Jurisdiction);

    assert!(has_id || has_tag || has_jurisdiction);
}

#[test]
fn test_saved_search_update_execution() {
    use advanced_search::*;

    let query = SearchQuery::default();
    let mut search = SavedSearch::new("Test", query, "user1");

    assert!(search.last_executed.is_none());
    assert!(search.last_result_count.is_none());

    search.update_execution(42);

    assert!(search.last_executed.is_some());
    assert_eq!(search.last_result_count, Some(42));
}

#[test]
fn test_search_analytics_empty() {
    use advanced_search::*;

    let analytics = SearchAnalytics::new();

    assert_eq!(analytics.total_searches(), 0);
    assert_eq!(analytics.average_result_count(), 0.0);
    assert!(analytics.top_queries(5).is_empty());
    assert!(analytics.zero_result_queries().is_empty());
}

// ========== Version Control Tests ==========

#[test]
fn test_version_control_branch_creation() {
    use version_control::*;

    let mut vc = VersionControlManager::new();

    // Main branch should exist
    assert!(vc.get_branch("main").is_some());
    assert_eq!(vc.list_branches().len(), 1);

    // Create a new branch
    let result = vc.create_branch("feature-1", Some("main".to_string()), "alice");
    assert!(result.is_ok());

    let branch = result.unwrap();
    assert_eq!(branch.name, "feature-1");
    assert_eq!(branch.parent_branch, Some("main".to_string()));
    assert_eq!(branch.created_by, "alice");
    assert!(!branch.protected);

    assert_eq!(vc.list_branches().len(), 2);
}

#[test]
fn test_version_control_branch_deletion() {
    use version_control::*;

    let mut vc = VersionControlManager::new();

    // Cannot delete main branch
    let result = vc.delete_branch("main");
    assert!(result.is_err());

    // Create and delete a branch
    vc.create_branch("feature-1", None, "alice").unwrap();
    assert_eq!(vc.list_branches().len(), 2);

    let result = vc.delete_branch("feature-1");
    assert!(result.is_ok());
    assert_eq!(vc.list_branches().len(), 1);

    // Cannot delete non-existent branch
    let result = vc.delete_branch("feature-1");
    assert!(result.is_err());
}

#[test]
fn test_version_control_protected_branch() {
    use version_control::*;

    let mut vc = VersionControlManager::new();

    // Create a protected branch by creating it and then modifying it
    vc.create_branch("protected-feature", Some("main".to_string()), "alice")
        .unwrap();

    // Manually protect the branch for testing
    // In production, this would be done through an administrative API
    if let Some(branch) = vc.get_branch_mut("protected-feature") {
        branch.protected = true;
    }

    // Cannot delete protected branch
    let result = vc.delete_branch("protected-feature");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("protected"));
}

#[test]
fn test_version_control_commit() {
    use version_control::*;

    let mut vc = VersionControlManager::new();
    let entry = StatuteEntry::new(test_statute("S1"), "JP");

    // Create a commit
    let result = vc.commit("main", "S1", entry.clone(), "Initial commit", "alice");
    assert!(result.is_ok());

    let commit_id = result.unwrap();

    // Check commit exists
    let commit = vc.get_commit(commit_id);
    assert!(commit.is_some());

    let commit = commit.unwrap();
    assert_eq!(commit.branch_name, "main");
    assert_eq!(commit.statute_id, "S1");
    assert_eq!(commit.message, "Initial commit");
    assert_eq!(commit.author, "alice");
    assert_eq!(commit.parent_commits.len(), 0); // First commit has no parents

    // Check branch head updated
    let branch = vc.get_branch("main").unwrap();
    assert_eq!(branch.head_commit, Some(commit_id));

    // Verify commit hash
    assert!(!commit.commit_hash.is_empty());
    assert_eq!(commit.short_hash().len(), 8);
}

#[test]
fn test_version_control_commit_chain() {
    use version_control::*;

    let mut vc = VersionControlManager::new();
    let entry = StatuteEntry::new(test_statute("S1"), "JP");

    // First commit
    let commit1 = vc
        .commit("main", "S1", entry.clone(), "First commit", "alice")
        .unwrap();

    // Second commit
    let commit2 = vc
        .commit("main", "S1", entry.clone(), "Second commit", "bob")
        .unwrap();

    // Check parent relationship
    let commit2_obj = vc.get_commit(commit2).unwrap();
    assert_eq!(commit2_obj.parent_commits.len(), 1);
    assert_eq!(commit2_obj.parent_commits[0], commit1);

    // Check commit history
    let history = vc.get_commit_history("main");
    assert_eq!(history.len(), 2);
    assert_eq!(history[0].commit_id, commit1);
    assert_eq!(history[1].commit_id, commit2);
}

#[test]
fn test_version_control_commit_signing() {
    use version_control::*;

    let mut vc = VersionControlManager::new();
    let entry = StatuteEntry::new(test_statute("S1"), "JP");

    let commit_id = vc
        .commit("main", "S1", entry, "Signed commit", "alice")
        .unwrap();

    // Sign the commit
    let result = vc.sign_commit(commit_id, "SIG:alice:abcdef1234567890");
    assert!(result.is_ok());

    let commit = vc.get_commit(commit_id).unwrap();
    assert!(commit.signature.is_some());

    // Verify signature
    assert!(commit.verify_signature("alice"));
    assert!(!commit.verify_signature("bob"));
}

#[test]
fn test_version_control_branch_merge_success() {
    use version_control::*;

    let mut vc = VersionControlManager::new();
    let entry = StatuteEntry::new(test_statute("S1"), "JP");

    // Make an initial commit on main
    vc.commit("main", "S1", entry.clone(), "Initial commit", "alice")
        .unwrap();

    // Create a feature branch
    vc.create_branch("feature", Some("main".to_string()), "alice")
        .unwrap();

    // Make a commit on feature branch
    let mut entry2 = StatuteEntry::new(test_statute("S1"), "JP");
    entry2.tags.push("feature".to_string());
    vc.commit("feature", "S1", entry2, "Feature work", "alice")
        .unwrap();

    // Merge feature into main
    let result = vc.merge_branch("feature", "main", "alice");
    assert!(result.success);
    assert!(!result.has_conflicts());
    assert!(result.merge_commit_id.is_some());

    // Check merge commit exists
    let merge_commit = vc.get_commit(result.merge_commit_id.unwrap()).unwrap();
    assert!(merge_commit.message.contains("Merge branch"));
    assert_eq!(merge_commit.parent_commits.len(), 2); // Merge commits have 2 parents
}

#[test]
fn test_version_control_branch_merge_conflict() {
    use version_control::*;

    let mut vc = VersionControlManager::new();

    // Create conflicting commits on different branches
    let mut entry1 = StatuteEntry::new(test_statute("S1"), "JP");
    vc.commit("main", "S1", entry1.clone(), "Main commit", "alice")
        .unwrap();

    vc.create_branch("feature", Some("main".to_string()), "alice")
        .unwrap();

    // Different jurisdiction on feature branch
    entry1.jurisdiction = "US".to_string();
    vc.commit("feature", "S1", entry1, "Feature commit", "bob")
        .unwrap();

    // Merge should detect conflict
    let result = vc.merge_branch("feature", "main", "alice");
    assert!(!result.success);
    assert!(result.has_conflicts());
    assert_eq!(result.conflict_count(), 1);
    assert_eq!(result.conflicts[0].field_name, "jurisdiction");
}

#[test]
fn test_version_control_pull_request_creation() {
    use version_control::*;

    let mut vc = VersionControlManager::new();

    vc.create_branch("feature", Some("main".to_string()), "alice")
        .unwrap();

    let result = vc.create_pull_request(
        "Add new feature",
        "This PR adds a new feature",
        "feature",
        "main",
        "alice",
    );

    assert!(result.is_ok());

    let pr_id = result.unwrap();
    let pr = vc.get_pull_request(pr_id).unwrap();

    assert_eq!(pr.pr_number, 1);
    assert_eq!(pr.title, "Add new feature");
    assert_eq!(pr.source_branch, "feature");
    assert_eq!(pr.target_branch, "main");
    assert_eq!(pr.author, "alice");
    assert_eq!(pr.status, PullRequestStatus::Open);
    assert_eq!(pr.reviews.len(), 0);
}

#[test]
fn test_version_control_pull_request_review() {
    use version_control::*;

    let mut vc = VersionControlManager::new();

    vc.create_branch("feature", Some("main".to_string()), "alice")
        .unwrap();
    let pr_id = vc
        .create_pull_request("Add feature", "Description", "feature", "main", "alice")
        .unwrap();

    // Add approval review
    let result = vc.add_review(pr_id, "bob", ReviewDecision::Approve, "Looks good!");
    assert!(result.is_ok());

    let pr = vc.get_pull_request(pr_id).unwrap();
    assert_eq!(pr.reviews.len(), 1);
    assert_eq!(pr.status, PullRequestStatus::Approved);
    assert!(pr.is_approved());
}

#[test]
fn test_version_control_pull_request_changes_requested() {
    use version_control::*;

    let mut vc = VersionControlManager::new();

    vc.create_branch("feature", Some("main".to_string()), "alice")
        .unwrap();
    let pr_id = vc
        .create_pull_request("Add feature", "Description", "feature", "main", "alice")
        .unwrap();

    // Request changes
    vc.add_review(
        pr_id,
        "bob",
        ReviewDecision::RequestChanges,
        "Please fix this",
    )
    .unwrap();

    let pr = vc.get_pull_request(pr_id).unwrap();
    assert_eq!(pr.status, PullRequestStatus::ChangesRequested);
    assert!(!pr.is_approved());
}

#[test]
fn test_version_control_pull_request_merge() {
    use version_control::*;

    let mut vc = VersionControlManager::new();
    let entry = StatuteEntry::new(test_statute("S1"), "JP");

    vc.create_branch("feature", Some("main".to_string()), "alice")
        .unwrap();
    vc.commit("feature", "S1", entry, "Feature commit", "alice")
        .unwrap();

    let pr_id = vc
        .create_pull_request("Add feature", "Description", "feature", "main", "alice")
        .unwrap();

    // Cannot merge without approval
    let result = vc.merge_pull_request(pr_id, "bob");
    assert!(result.is_err());

    // Add approval
    vc.add_review(pr_id, "bob", ReviewDecision::Approve, "LGTM")
        .unwrap();

    // Now merge should work
    let result = vc.merge_pull_request(pr_id, "bob");
    assert!(result.is_ok());

    let merge_result = result.unwrap();
    assert!(merge_result.success);

    // Check PR status
    let pr = vc.get_pull_request(pr_id).unwrap();
    assert_eq!(pr.status, PullRequestStatus::Merged);
    assert!(pr.merged_at.is_some());
    assert_eq!(pr.merged_by, Some("bob".to_string()));
}

#[test]
fn test_version_control_field_blame() {
    use version_control::*;

    let mut vc = VersionControlManager::new();
    let entry = StatuteEntry::new(test_statute("S1"), "JP");

    // First commit
    vc.commit("main", "S1", entry.clone(), "Initial commit", "alice")
        .unwrap();

    // Check field blame
    let blame = vc.get_field_blame("S1", "title");
    assert!(blame.is_some());

    let blame = blame.unwrap();
    assert_eq!(blame.field_name, "title");
    assert_eq!(blame.last_author, "alice");
    assert_eq!(blame.modification_count(), 1);

    let authors = blame.all_authors();
    assert_eq!(authors.len(), 1);
    assert!(authors.contains("alice"));
}

#[test]
fn test_version_control_field_blame_history() {
    use version_control::*;

    let mut vc = VersionControlManager::new();

    // Multiple commits changing the same statute
    let entry1 = StatuteEntry::new(test_statute("S1"), "JP");
    vc.commit("main", "S1", entry1, "First commit", "alice")
        .unwrap();

    let entry2 = StatuteEntry::new(test_statute("S1"), "US");
    vc.commit("main", "S1", entry2, "Second commit", "bob")
        .unwrap();

    let entry3 = StatuteEntry::new(test_statute("S1"), "UK");
    vc.commit("main", "S1", entry3, "Third commit", "charlie")
        .unwrap();

    // Check jurisdiction field blame
    let blame = vc.get_field_blame("S1", "jurisdiction").unwrap();
    assert_eq!(blame.current_value, "UK");
    assert_eq!(blame.last_author, "charlie");
    assert_eq!(blame.modification_count(), 3);

    // Check all authors
    let authors = blame.all_authors();
    assert_eq!(authors.len(), 3);
    assert!(authors.contains("alice"));
    assert!(authors.contains("bob"));
    assert!(authors.contains("charlie"));

    // Check history
    assert_eq!(blame.history.len(), 3);
    assert_eq!(blame.history[0].new_value, "JP");
    assert_eq!(blame.history[1].new_value, "US");
    assert_eq!(blame.history[2].new_value, "UK");
}

#[test]
fn test_version_control_statute_blame() {
    use version_control::*;

    let mut vc = VersionControlManager::new();
    let entry = StatuteEntry::new(test_statute("S1"), "JP");

    vc.commit("main", "S1", entry, "Initial commit", "alice")
        .unwrap();

    // Get all field blames for the statute
    let statute_blame = vc.get_statute_blame("S1");
    assert!(statute_blame.is_some());

    let statute_blame = statute_blame.unwrap();
    assert!(statute_blame.contains_key("title"));
    assert!(statute_blame.contains_key("jurisdiction"));
    assert!(statute_blame.contains_key("status"));
}

#[test]
fn test_version_control_list_pull_requests() {
    use version_control::*;

    let mut vc = VersionControlManager::new();

    vc.create_branch("feature1", Some("main".to_string()), "alice")
        .unwrap();
    vc.create_branch("feature2", Some("main".to_string()), "bob")
        .unwrap();

    vc.create_pull_request("PR 1", "Desc 1", "feature1", "main", "alice")
        .unwrap();
    vc.create_pull_request("PR 2", "Desc 2", "feature2", "main", "bob")
        .unwrap();

    let all_prs = vc.list_pull_requests();
    assert_eq!(all_prs.len(), 2);

    let open_prs = vc.list_open_pull_requests();
    assert_eq!(open_prs.len(), 2);
}

#[test]
fn test_version_control_pr_close() {
    use version_control::*;

    let mut vc = VersionControlManager::new();

    vc.create_branch("feature", Some("main".to_string()), "alice")
        .unwrap();
    let pr_id = vc
        .create_pull_request("Add feature", "Description", "feature", "main", "alice")
        .unwrap();

    // Close the PR
    vc.close_pull_request(pr_id).unwrap();

    let pr = vc.get_pull_request(pr_id).unwrap();
    assert_eq!(pr.status, PullRequestStatus::Closed);

    // Closed PR should not appear in open PRs
    let open_prs = vc.list_open_pull_requests();
    assert_eq!(open_prs.len(), 0);
}

#[test]
fn test_version_control_branch_merge_conflict_display() {
    use version_control::*;

    let conflict = BranchMergeConflict {
        field_name: "title".to_string(),
        source_value: "Source Title".to_string(),
        target_value: "Target Title".to_string(),
        base_value: Some("Base Title".to_string()),
    };

    let display = format!("{}", conflict);
    assert!(display.contains("title"));
    assert!(display.contains("Source Title"));
    assert!(display.contains("Target Title"));
}

#[test]
fn test_version_control_merge_branch_result() {
    use version_control::*;

    let result = MergeBranchResult {
        merge_commit_id: None,
        conflicts: vec![BranchMergeConflict {
            field_name: "title".to_string(),
            source_value: "A".to_string(),
            target_value: "B".to_string(),
            base_value: None,
        }],
        success: false,
        message: "Conflicts detected".to_string(),
    };

    assert!(result.has_conflicts());
    assert_eq!(result.conflict_count(), 1);
}

#[test]
fn test_version_control_commit_on_nonexistent_branch() {
    use version_control::*;

    let mut vc = VersionControlManager::new();
    let entry = StatuteEntry::new(test_statute("S1"), "JP");

    let result = vc.commit("nonexistent", "S1", entry, "Commit", "alice");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("does not exist"));
}

#[test]
fn test_version_control_pr_status_variants() {
    use version_control::*;

    let _open = PullRequestStatus::Open;
    let _in_review = PullRequestStatus::InReview;
    let _approved = PullRequestStatus::Approved;
    let _changes_requested = PullRequestStatus::ChangesRequested;
    let _merged = PullRequestStatus::Merged;
    let _closed = PullRequestStatus::Closed;
}

#[test]
fn test_version_control_review_decision_variants() {
    use version_control::*;

    let _approve = ReviewDecision::Approve;
    let _request_changes = ReviewDecision::RequestChanges;
    let _comment = ReviewDecision::Comment;
}

#[test]
fn test_version_control_branch_with_description() {
    use version_control::*;

    let branch = Branch::new("feature", "alice")
        .with_description("This is a feature branch")
        .with_protected(true);

    assert_eq!(
        branch.description,
        Some("This is a feature branch".to_string())
    );
    assert!(branch.protected);
}

// ========== API Extensions Tests ==========

#[test]
fn test_subscription_manager_subscribe() {
    use api_extensions::*;

    let manager = SubscriptionManager::new();
    let filter = SubscriptionFilter {
        statute_ids: Some(vec!["S1".to_string()]),
        jurisdictions: None,
        tags: None,
        event_types: None,
    };

    let subscription_id = manager.subscribe(filter);
    assert_eq!(manager.subscription_count(), 1);

    let success = manager.unsubscribe(subscription_id);
    assert!(success);
    assert_eq!(manager.subscription_count(), 0);
}

#[test]
fn test_subscription_manager_publish() {
    use api_extensions::*;

    let manager = SubscriptionManager::new();
    let event = SubscriptionEvent::StatuteRegistered {
        statute_id: "S1".to_string(),
        timestamp: Utc::now(),
    };

    manager.publish(event.clone());
    let events = manager.get_published_events();
    assert_eq!(events.len(), 1);

    manager.clear_events();
    assert_eq!(manager.get_published_events().len(), 0);
}

#[test]
fn test_subscription_event_variants() {
    use api_extensions::*;

    let _registered = SubscriptionEvent::StatuteRegistered {
        statute_id: "S1".to_string(),
        timestamp: Utc::now(),
    };

    let _updated = SubscriptionEvent::StatuteUpdated {
        statute_id: "S1".to_string(),
        version: 2,
        timestamp: Utc::now(),
    };

    let _deleted = SubscriptionEvent::StatuteDeleted {
        statute_id: "S1".to_string(),
        timestamp: Utc::now(),
    };

    let _status_changed = SubscriptionEvent::StatusChanged {
        statute_id: "S1".to_string(),
        old_status: StatuteStatus::Draft,
        new_status: StatuteStatus::Active,
        timestamp: Utc::now(),
    };
}

#[test]
fn test_grpc_service_get_statute() {
    use api_extensions::grpc::*;

    let registry = Arc::new(Mutex::new(StatuteRegistry::new()));
    let service = GrpcStatuteService::new(registry.clone());

    // Add a statute
    {
        let mut reg = registry.lock().unwrap();
        let entry = StatuteEntry::new(test_statute("S1"), "JP");
        reg.register(entry).unwrap();
    }

    // Get it via gRPC
    let request = GetStatuteRequest {
        statute_id: "S1".to_string(),
    };
    let response = service.get_statute(request);
    assert!(response.found);
    assert!(response.statute.is_some());
}

#[test]
fn test_grpc_service_list_statutes() {
    use api_extensions::grpc::*;

    let registry = Arc::new(Mutex::new(StatuteRegistry::new()));
    let service = GrpcStatuteService::new(registry.clone());

    // Add statutes
    {
        let mut reg = registry.lock().unwrap();
        for i in 1..=10 {
            let entry = StatuteEntry::new(test_statute(&format!("S{}", i)), "JP");
            reg.register(entry).unwrap();
        }
    }

    // List with pagination
    let request = ListStatutesRequest {
        page: 0,
        page_size: 5,
        jurisdiction: None,
        tags: vec![],
    };
    let response = service.list_statutes(request);
    assert_eq!(response.statutes.len(), 5);
    assert_eq!(response.total_count, 10);
    assert!(response.has_more);
}

#[test]
fn test_grpc_service_register_statute() {
    use api_extensions::grpc::*;

    let registry = Arc::new(Mutex::new(StatuteRegistry::new()));
    let service = GrpcStatuteService::new(registry);

    let entry = StatuteEntry::new(test_statute("S1"), "JP");
    let request = RegisterStatuteRequest { statute: entry };
    let response = service.register_statute(request);

    assert!(response.success);
    assert_eq!(response.error, None);
    assert!(!response.statute_id.is_empty());
}

#[test]
fn test_stream_config() {
    use api_extensions::streaming::*;

    let mut auth = HashMap::new();
    auth.insert("token".to_string(), "secret".to_string());

    let config = StreamConfig::new(
        "kafka-stream",
        StreamDestination::Kafka,
        "localhost:9092",
        "statutes",
    )
    .with_auth(auth.clone())
    .with_buffer_size(500)
    .with_enabled(true);

    assert_eq!(config.name, "kafka-stream");
    assert_eq!(config.destination, StreamDestination::Kafka);
    assert_eq!(config.connection, "localhost:9092");
    assert_eq!(config.topic, "statutes");
    assert_eq!(config.buffer_size, 500);
    assert!(config.enabled);
    assert_eq!(config.auth, Some(auth));
}

#[test]
fn test_stream_destination_variants() {
    use api_extensions::streaming::*;

    let _kafka = StreamDestination::Kafka;
    let _nats = StreamDestination::Nats;
    let _kinesis = StreamDestination::Kinesis;
    let _webhook = StreamDestination::Webhook;
}

#[test]
fn test_stream_message() {
    use api_extensions::streaming::*;

    let message = StreamMessage::new("statute.registered", "S1", "{\"id\": \"S1\"}")
        .with_metadata("source", "api")
        .with_metadata("version", "1.0");

    assert_eq!(message.event_type, "statute.registered");
    assert_eq!(message.statute_id, "S1");
    assert_eq!(message.payload, "{\"id\": \"S1\"}");
    assert_eq!(message.metadata.get("source"), Some(&"api".to_string()));
    assert_eq!(message.metadata.get("version"), Some(&"1.0".to_string()));
}

#[test]
fn test_event_stream_manager() {
    use api_extensions::streaming::*;

    let mut manager = EventStreamManager::new();

    let config = StreamConfig::new(
        "test-stream",
        StreamDestination::Kafka,
        "localhost:9092",
        "test",
    );
    manager.add_stream(config);

    assert!(manager.get_stream("test-stream").is_some());
    assert_eq!(manager.list_streams().len(), 1);
    assert_eq!(manager.get_message_count("test-stream"), 0);

    let message = StreamMessage::new("test", "S1", "payload");
    manager.publish("test-stream", message).unwrap();
    assert_eq!(manager.get_message_count("test-stream"), 1);

    manager.reset_count("test-stream");
    assert_eq!(manager.get_message_count("test-stream"), 0);

    let removed = manager.remove_stream("test-stream");
    assert!(removed);
    assert_eq!(manager.list_streams().len(), 0);
}

#[test]
fn test_event_stream_publish_disabled() {
    use api_extensions::streaming::*;

    let mut manager = EventStreamManager::new();
    let config = StreamConfig::new(
        "test-stream",
        StreamDestination::Nats,
        "localhost:4222",
        "test",
    )
    .with_enabled(false);
    manager.add_stream(config);

    let message = StreamMessage::new("test", "S1", "payload");
    let result = manager.publish("test-stream", message);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("disabled"));
}

#[test]
fn test_bulk_operation_register() {
    use api_extensions::bulk::*;

    let registry = Arc::new(Mutex::new(StatuteRegistry::new()));
    let executor = BulkOperationExecutor::new(registry);

    let entries = vec![
        StatuteEntry::new(test_statute("S1"), "JP"),
        StatuteEntry::new(test_statute("S2"), "JP"),
        StatuteEntry::new(test_statute("S3"), "JP"),
    ];

    let request = BulkOperationRequest {
        operation_type: BulkOperationType::Register,
        statute_ids: vec![],
        statute_entries: entries,
        new_status: None,
        continue_on_error: true,
    };

    let response = executor.execute(request);
    assert_eq!(response.total_processed, 3);
    assert_eq!(response.successful, 3);
    assert_eq!(response.failed, 0);
    assert!(response.is_complete_success());
    assert!((response.success_rate() - 1.0).abs() < 0.01);
}
