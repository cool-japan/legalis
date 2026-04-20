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
fn test_register_statute() {
    let mut registry = StatuteRegistry::new();
    let entry = StatuteEntry::new(test_statute("test-1"), "JP")
        .with_tag("civil")
        .with_status(StatuteStatus::Active);

    let id = registry.register(entry).unwrap();
    assert!(!id.is_nil());
    assert_eq!(registry.count(), 1);
}

#[test]
fn test_version_management() {
    let mut registry = StatuteRegistry::new();
    let entry = StatuteEntry::new(test_statute("test-1"), "JP");
    registry.register(entry).unwrap();

    let new_version = registry.update("test-1", test_statute("test-1")).unwrap();
    assert_eq!(new_version, 2);

    let versions = registry.list_versions("test-1");
    assert_eq!(versions, vec![1, 2]);
}

#[test]
fn test_query_by_tag() {
    let mut registry = StatuteRegistry::new();

    registry
        .register(StatuteEntry::new(test_statute("civil-1"), "JP").with_tag("civil"))
        .unwrap();

    registry
        .register(StatuteEntry::new(test_statute("criminal-1"), "JP").with_tag("criminal"))
        .unwrap();

    let civil = registry.query_by_tag("civil");
    assert_eq!(civil.len(), 1);
    assert_eq!(civil[0].statute.id, "civil-1");
}

#[test]
fn test_is_active() {
    let mut entry = StatuteEntry::new(test_statute("test"), "JP");
    entry.status = StatuteStatus::Active;
    assert!(entry.is_active());

    entry.status = StatuteStatus::Draft;
    assert!(!entry.is_active());
}

#[test]
fn test_fuzzy_search() {
    let mut registry = StatuteRegistry::new();
    registry
        .register(StatuteEntry::new(test_statute("civil-code-001"), "JP"))
        .unwrap();
    registry
        .register(StatuteEntry::new(test_statute("criminal-code-002"), "JP"))
        .unwrap();
    registry
        .register(StatuteEntry::new(test_statute("commercial-code-003"), "JP"))
        .unwrap();

    let results = registry.fuzzy_search("civil", 10);
    assert!(!results.is_empty());
    assert_eq!(results[0].1.statute.id, "civil-code-001");
}

#[test]
fn test_full_text_search() {
    let mut registry = StatuteRegistry::new();

    let mut statute1 = test_statute("statute-1");
    statute1.effect.description = "This statute deals with civil matters".to_string();

    let mut statute2 = test_statute("statute-2");
    statute2.effect.description = "This statute deals with criminal matters".to_string();

    registry
        .register(StatuteEntry::new(statute1, "JP"))
        .unwrap();
    registry
        .register(StatuteEntry::new(statute2, "JP"))
        .unwrap();

    let results = registry.full_text_search("civil");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].statute.id, "statute-1");
}

#[test]
fn test_advanced_search() {
    let mut registry = StatuteRegistry::new();

    registry
        .register(
            StatuteEntry::new(test_statute("civil-1"), "JP")
                .with_tag("civil")
                .with_status(StatuteStatus::Active),
        )
        .unwrap();

    registry
        .register(
            StatuteEntry::new(test_statute("criminal-1"), "JP")
                .with_tag("criminal")
                .with_status(StatuteStatus::Draft),
        )
        .unwrap();

    registry
        .register(
            StatuteEntry::new(test_statute("commercial-1"), "US")
                .with_tag("commercial")
                .with_status(StatuteStatus::Active),
        )
        .unwrap();

    // Search by tag
    let query = SearchQuery::new().with_tag("civil");
    let results = registry.search(&query);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].statute.id, "civil-1");

    // Search by jurisdiction
    let query = SearchQuery::new().with_jurisdiction("US");
    let results = registry.search(&query);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].statute.id, "commercial-1");

    // Search by status
    let query = SearchQuery::new().with_status(StatuteStatus::Active);
    let results = registry.search(&query);
    assert_eq!(results.len(), 2);

    // Active only
    let query = SearchQuery::new().active_only();
    let results = registry.search(&query);
    assert_eq!(results.len(), 2);
}

#[test]
fn test_pagination() {
    let mut registry = StatuteRegistry::new();

    for i in 0..25 {
        registry
            .register(StatuteEntry::new(
                test_statute(&format!("statute-{}", i)),
                "JP",
            ))
            .unwrap();
    }

    // First page
    let page1 = registry.list_paged(Pagination::new(0, 10));
    assert_eq!(page1.items.len(), 10);
    assert_eq!(page1.total, 25);
    assert_eq!(page1.total_pages, 3);
    assert_eq!(page1.page, 0);

    // Second page
    let page2 = registry.list_paged(Pagination::new(1, 10));
    assert_eq!(page2.items.len(), 10);
    assert_eq!(page2.page, 1);

    // Last page
    let page3 = registry.list_paged(Pagination::new(2, 10));
    assert_eq!(page3.items.len(), 5);
    assert_eq!(page3.page, 2);
}

#[test]
fn test_search_paged() {
    let mut registry = StatuteRegistry::new();

    for i in 0..15 {
        registry
            .register(
                StatuteEntry::new(test_statute(&format!("civil-{}", i)), "JP").with_tag("civil"),
            )
            .unwrap();
    }

    for i in 0..10 {
        registry
            .register(
                StatuteEntry::new(test_statute(&format!("criminal-{}", i)), "JP")
                    .with_tag("criminal"),
            )
            .unwrap();
    }

    let query = SearchQuery::new().with_tag("civil");
    let page1 = registry.search_paged(&query, Pagination::new(0, 10));

    assert_eq!(page1.items.len(), 10);
    assert_eq!(page1.total, 15);
    assert_eq!(page1.total_pages, 2);
}

#[test]
fn test_batch_register() {
    let mut registry = StatuteRegistry::new();

    let entries = vec![
        StatuteEntry::new(test_statute("statute-1"), "JP"),
        StatuteEntry::new(test_statute("statute-2"), "JP"),
        StatuteEntry::new(test_statute("statute-3"), "JP"),
    ];

    let results = registry.batch_register(entries);
    assert_eq!(results.len(), 3);
    assert!(results.iter().all(|r| r.is_ok()));
    assert_eq!(registry.count(), 3);
}

#[test]
fn test_batch_update() {
    let mut registry = StatuteRegistry::new();

    registry
        .register(StatuteEntry::new(test_statute("statute-1"), "JP"))
        .unwrap();
    registry
        .register(StatuteEntry::new(test_statute("statute-2"), "JP"))
        .unwrap();

    let updates = vec![
        ("statute-1".to_string(), test_statute("statute-1")),
        ("statute-2".to_string(), test_statute("statute-2")),
    ];

    let results = registry.batch_update(updates);
    assert_eq!(results.len(), 2);
    assert!(results.iter().all(|r| r.is_ok()));
    assert_eq!(results[0].as_ref().unwrap(), &2);
    assert_eq!(results[1].as_ref().unwrap(), &2);
}

#[test]
fn test_batch_set_status() {
    let mut registry = StatuteRegistry::new();

    registry
        .register(StatuteEntry::new(test_statute("statute-1"), "JP"))
        .unwrap();
    registry
        .register(StatuteEntry::new(test_statute("statute-2"), "JP"))
        .unwrap();
    registry
        .register(StatuteEntry::new(test_statute("statute-3"), "JP"))
        .unwrap();

    let statute_ids = vec![
        "statute-1".to_string(),
        "statute-2".to_string(),
        "statute-3".to_string(),
    ];

    let results = registry.batch_set_status(statute_ids, StatuteStatus::Active);
    assert_eq!(results.len(), 3);
    assert!(results.iter().all(|r| r.is_ok()));

    assert_eq!(
        registry.get_uncached("statute-1").unwrap().status,
        StatuteStatus::Active
    );
    assert_eq!(
        registry.get_uncached("statute-2").unwrap().status,
        StatuteStatus::Active
    );
    assert_eq!(
        registry.get_uncached("statute-3").unwrap().status,
        StatuteStatus::Active
    );
}

#[test]
fn test_cache() {
    let mut registry = StatuteRegistry::new();
    registry
        .register(StatuteEntry::new(test_statute("statute-1"), "JP"))
        .unwrap();

    // First access - not cached
    let (cache_len, _) = registry.cache_stats();
    assert_eq!(cache_len, 0);

    // Access the statute - should be cached
    let entry = registry.get("statute-1");
    assert!(entry.is_some());

    let (cache_len, _) = registry.cache_stats();
    assert_eq!(cache_len, 1);

    // Clear cache
    registry.clear_cache();
    let (cache_len, _) = registry.cache_stats();
    assert_eq!(cache_len, 0);
}

#[test]
fn test_pagination_params() {
    let pagination = Pagination::new(2, 10);
    assert_eq!(pagination.offset(), 20);
    assert_eq!(pagination.limit(), 10);

    let default_pagination = Pagination::default();
    assert_eq!(default_pagination.page, 0);
    assert_eq!(default_pagination.per_page, 50);
}

#[test]
fn test_search_query_builder() {
    let query = SearchQuery::new()
        .with_text("test")
        .with_tag("civil")
        .with_jurisdiction("JP")
        .with_status(StatuteStatus::Active)
        .active_only();

    assert_eq!(query.text, Some("test".to_string()));
    assert_eq!(query.tags, vec!["civil"]);
    assert_eq!(query.jurisdiction, Some("JP".to_string()));
    assert_eq!(query.status, Some(StatuteStatus::Active));
    assert!(query.active_only);
}

#[test]
fn test_search_by_effect_type() {
    use legalis_core::{ComparisonOp, Condition};

    let mut registry = StatuteRegistry::new();

    // Create statutes with different effect types
    let mut grant_statute = Statute::new(
        "grant-1",
        "Grant Statute",
        Effect::new(EffectType::Grant, "Grant permission"),
    );
    grant_statute.preconditions.push(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 18,
    });

    let revoke_statute = Statute::new(
        "revoke-1",
        "Revoke Statute",
        Effect::new(EffectType::Revoke, "Revoke permission"),
    );

    let obligation_statute = Statute::new(
        "obligation-1",
        "Obligation Statute",
        Effect::new(EffectType::Obligation, "Must comply"),
    );

    registry
        .register(StatuteEntry::new(grant_statute, "JP"))
        .unwrap();
    registry
        .register(StatuteEntry::new(revoke_statute, "JP"))
        .unwrap();
    registry
        .register(StatuteEntry::new(obligation_statute, "JP"))
        .unwrap();

    let grant_results = registry.search_by_effect_type(EffectType::Grant);
    assert_eq!(grant_results.len(), 1);
    assert_eq!(grant_results[0].statute.id, "grant-1");

    let revoke_results = registry.search_by_effect_type(EffectType::Revoke);
    assert_eq!(revoke_results.len(), 1);
    assert_eq!(revoke_results[0].statute.id, "revoke-1");
}

#[test]
fn test_search_with_age_condition() {
    use legalis_core::{ComparisonOp, Condition};

    let mut registry = StatuteRegistry::new();

    let mut age_statute = Statute::new(
        "age-1",
        "Age Statute",
        Effect::new(EffectType::Grant, "Test"),
    );
    age_statute.preconditions.push(Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 18,
    });

    let mut income_statute = Statute::new(
        "income-1",
        "Income Statute",
        Effect::new(EffectType::Grant, "Test"),
    );
    income_statute.preconditions.push(Condition::Income {
        operator: ComparisonOp::LessThan,
        value: 50000,
    });

    registry
        .register(StatuteEntry::new(age_statute, "JP"))
        .unwrap();
    registry
        .register(StatuteEntry::new(income_statute, "JP"))
        .unwrap();

    let age_results = registry.search_with_age_condition();
    assert_eq!(age_results.len(), 1);
    assert_eq!(age_results[0].statute.id, "age-1");

    let income_results = registry.search_with_income_condition();
    assert_eq!(income_results.len(), 1);
    assert_eq!(income_results[0].statute.id, "income-1");
}

#[test]
fn test_search_by_condition_type_nested() {
    use legalis_core::{ComparisonOp, Condition};

    let mut registry = StatuteRegistry::new();

    let mut complex_statute = Statute::new(
        "complex-1",
        "Complex Statute",
        Effect::new(EffectType::Grant, "Test"),
    );

    // Create nested condition: (Age >= 18) AND (Income < 50000)
    let age_cond = Condition::Age {
        operator: ComparisonOp::GreaterOrEqual,
        value: 18,
    };
    let income_cond = Condition::Income {
        operator: ComparisonOp::LessThan,
        value: 50000,
    };
    let and_cond = Condition::And(Box::new(age_cond), Box::new(income_cond));

    complex_statute.preconditions.push(and_cond);

    registry
        .register(StatuteEntry::new(complex_statute, "JP"))
        .unwrap();

    // Should find the statute even though Age condition is nested
    let age_results = registry.search_with_age_condition();
    assert_eq!(age_results.len(), 1);
    assert_eq!(age_results[0].statute.id, "complex-1");

    // Should also find it by income condition
    let income_results = registry.search_with_income_condition();
    assert_eq!(income_results.len(), 1);
    assert_eq!(income_results[0].statute.id, "complex-1");
}

#[test]
fn test_dependency_graph() {
    let mut registry = StatuteRegistry::new();

    // Create a dependency chain: A -> B -> C
    let statute_a = StatuteEntry::new(test_statute("statute-a"), "JP")
        .with_reference("statute-b")
        .with_reference("statute-c");

    let statute_b = StatuteEntry::new(test_statute("statute-b"), "JP").with_reference("statute-c");

    let statute_c = StatuteEntry::new(test_statute("statute-c"), "JP");

    registry.register(statute_a).unwrap();
    registry.register(statute_b).unwrap();
    registry.register(statute_c).unwrap();

    // Test dependency graph for statute-a
    let graph = registry.get_dependency_graph("statute-a").unwrap();
    assert_eq!(graph.root_id, "statute-a");

    let all_deps = graph.all_dependencies();
    assert!(all_deps.contains("statute-b"));
    assert!(all_deps.contains("statute-c"));

    // Test reverse dependencies for statute-c
    let graph_c = registry.get_dependency_graph("statute-c").unwrap();
    let dependents = graph_c.all_dependents();
    assert!(dependents.contains("statute-a") || dependents.contains("statute-b"));

    // Test depth
    assert!(graph.depth() > 0);
}

#[test]
fn test_dependency_graph_nonexistent() {
    let registry = StatuteRegistry::new();
    let graph = registry.get_dependency_graph("nonexistent");
    assert!(graph.is_none());
}

#[test]
fn test_optimistic_concurrency_control() {
    let mut registry = StatuteRegistry::new();

    let entry = StatuteEntry::new(test_statute("statute-1"), "JP");
    registry.register(entry).unwrap();

    // Get the entry and its ETag
    let statute = registry.get_uncached("statute-1").unwrap();
    let etag = statute.etag.clone();

    // Update with correct ETag should succeed
    let result = registry.update_with_etag("statute-1", test_statute("statute-1"), &etag);
    assert!(result.is_ok());

    // Update with old ETag should fail
    let result = registry.update_with_etag("statute-1", test_statute("statute-1"), &etag);
    assert!(result.is_err());

    match result {
        Err(RegistryError::ConcurrentModification { .. }) => {
            // Expected error
        }
        _ => panic!("Expected ConcurrentModification error"),
    }
}

#[test]
fn test_set_status_with_etag() {
    let mut registry = StatuteRegistry::new();

    let entry = StatuteEntry::new(test_statute("statute-1"), "JP");
    registry.register(entry).unwrap();

    // Get the entry and its ETag
    let statute = registry.get_uncached("statute-1").unwrap();
    let etag = statute.etag.clone();

    // Set status with correct ETag should succeed
    let result = registry.set_status_with_etag("statute-1", StatuteStatus::Active, &etag);
    assert!(result.is_ok());

    // Set status with old ETag should fail
    let result = registry.set_status_with_etag("statute-1", StatuteStatus::Repealed, &etag);
    assert!(result.is_err());
}

#[test]
fn test_etag_changes_on_update() {
    let mut registry = StatuteRegistry::new();

    let entry = StatuteEntry::new(test_statute("statute-1"), "JP");
    registry.register(entry).unwrap();

    let etag1 = registry.get_uncached("statute-1").unwrap().etag.clone();

    // Update the statute
    registry
        .update("statute-1", test_statute("statute-1"))
        .unwrap();

    let etag2 = registry.get_uncached("statute-1").unwrap().etag.clone();

    // ETag should have changed
    assert_ne!(etag1, etag2);
}

#[test]
fn test_cache_invalidation_on_update() {
    let mut registry = StatuteRegistry::new();

    let entry = StatuteEntry::new(test_statute("statute-1"), "JP");
    registry.register(entry).unwrap();

    // Access to cache it
    registry.get("statute-1");
    assert_eq!(registry.cache_stats().0, 1);

    // Update should invalidate cache
    registry
        .update("statute-1", test_statute("statute-1"))
        .unwrap();

    // Cache should be empty
    assert_eq!(registry.cache_stats().0, 0);
}

#[test]
fn test_event_sourcing_register() {
    let mut registry = StatuteRegistry::new();

    let entry = StatuteEntry::new(test_statute("statute-1"), "JP");
    registry.register(entry).unwrap();

    // Check that event was recorded
    assert_eq!(registry.event_count(), 1);

    let events = registry.all_events();
    assert_eq!(events.len(), 1);

    match events[0] {
        RegistryEvent::StatuteRegistered {
            statute_id,
            jurisdiction,
            ..
        } => {
            assert_eq!(statute_id, "statute-1");
            assert_eq!(jurisdiction, "JP");
        }
        _ => panic!("Expected StatuteRegistered event"),
    }
}

#[test]
fn test_event_sourcing_update() {
    let mut registry = StatuteRegistry::new();

    let entry = StatuteEntry::new(test_statute("statute-1"), "JP");
    registry.register(entry).unwrap();

    registry
        .update("statute-1", test_statute("statute-1"))
        .unwrap();

    // Should have 2 events: register + update
    assert_eq!(registry.event_count(), 2);

    let events = registry.all_events();
    match events[1] {
        RegistryEvent::StatuteUpdated {
            statute_id,
            old_version,
            new_version,
            ..
        } => {
            assert_eq!(statute_id, "statute-1");
            assert_eq!(*old_version, 1);
            assert_eq!(*new_version, 2);
        }
        _ => panic!("Expected StatuteUpdated event"),
    }
}

#[test]
fn test_event_sourcing_status_change() {
    let mut registry = StatuteRegistry::new();

    let entry = StatuteEntry::new(test_statute("statute-1"), "JP");
    registry.register(entry).unwrap();

    registry
        .set_status("statute-1", StatuteStatus::Active)
        .unwrap();

    // Should have 2 events: register + status change
    assert_eq!(registry.event_count(), 2);

    let events = registry.all_events();
    match events[1] {
        RegistryEvent::StatusChanged {
            statute_id,
            old_status,
            new_status,
            ..
        } => {
            assert_eq!(statute_id, "statute-1");
            assert_eq!(*old_status, StatuteStatus::Draft);
            assert_eq!(*new_status, StatuteStatus::Active);
        }
        _ => panic!("Expected StatusChanged event"),
    }
}

#[test]
fn test_events_for_statute() {
    let mut registry = StatuteRegistry::new();

    let entry1 = StatuteEntry::new(test_statute("statute-1"), "JP");
    let entry2 = StatuteEntry::new(test_statute("statute-2"), "JP");

    registry.register(entry1).unwrap();
    registry.register(entry2).unwrap();

    registry
        .update("statute-1", test_statute("statute-1"))
        .unwrap();

    // Get events for statute-1
    let events = registry.events_for_statute("statute-1");
    assert_eq!(events.len(), 2); // register + update

    // Get events for statute-2
    let events = registry.events_for_statute("statute-2");
    assert_eq!(events.len(), 1); // register only
}

#[test]
fn test_events_in_range() {
    use chrono::Duration;

    let mut registry = StatuteRegistry::new();

    let now = Utc::now();
    let past = now - Duration::hours(1);
    let future = now + Duration::hours(1);

    let entry = StatuteEntry::new(test_statute("statute-1"), "JP");
    registry.register(entry).unwrap();

    // All events should be in range
    let events = registry.events_in_range(past, future);
    assert_eq!(events.len(), 1);

    // No events before the past
    let events = registry.events_in_range(past - Duration::hours(2), past);
    assert_eq!(events.len(), 0);
}

#[test]
fn test_event_store_max_events() {
    let mut store = EventStore::with_max_events(2);

    store.record(RegistryEvent::StatuteRegistered {
        registry_id: Uuid::new_v4(),
        statute_id: "statute-1".to_string(),
        jurisdiction: "JP".to_string(),
        timestamp: Utc::now(),
    });

    store.record(RegistryEvent::StatuteRegistered {
        registry_id: Uuid::new_v4(),
        statute_id: "statute-2".to_string(),
        jurisdiction: "JP".to_string(),
        timestamp: Utc::now(),
    });

    store.record(RegistryEvent::StatuteRegistered {
        registry_id: Uuid::new_v4(),
        statute_id: "statute-3".to_string(),
        jurisdiction: "JP".to_string(),
        timestamp: Utc::now(),
    });

    // Should only keep the last 2 events
    assert_eq!(store.count(), 2);
    assert_eq!(store.all_events().len(), 2);
}

#[test]
fn test_export_events() {
    let mut registry = StatuteRegistry::new();

    let entry = StatuteEntry::new(test_statute("statute-1"), "JP");
    registry.register(entry).unwrap();

    let exported = registry.export_events().unwrap();
    assert!(!exported.is_empty());
    assert!(exported.contains("StatuteRegistered"));
}

#[test]
fn test_clear_events() {
    let mut registry = StatuteRegistry::new();

    let entry = StatuteEntry::new(test_statute("statute-1"), "JP");
    registry.register(entry).unwrap();

    assert_eq!(registry.event_count(), 1);

    registry.clear_events();
    assert_eq!(registry.event_count(), 0);
}

#[test]
fn test_create_backup() {
    let mut registry = StatuteRegistry::new();

    let entry1 = StatuteEntry::new(test_statute("statute-1"), "JP").with_tag("civil");
    let entry2 = StatuteEntry::new(test_statute("statute-2"), "US").with_tag("commercial");

    registry.register(entry1).unwrap();
    registry.register(entry2).unwrap();

    registry
        .update("statute-1", test_statute("statute-1"))
        .unwrap();

    let backup = registry.create_backup(Some("Test backup".to_string()));

    assert_eq!(backup.metadata.statute_count, 2);
    assert_eq!(backup.metadata.event_count, 3); // 2 registers + 1 update
    assert_eq!(backup.metadata.format_version, "1.0");
    assert_eq!(backup.metadata.description, Some("Test backup".to_string()));
    assert_eq!(backup.statutes.len(), 2);
    assert_eq!(backup.events.len(), 3);
}

#[test]
fn test_export_and_import_backup() {
    let mut registry = StatuteRegistry::new();

    let entry1 = StatuteEntry::new(test_statute("statute-1"), "JP").with_tag("civil");
    let entry2 = StatuteEntry::new(test_statute("statute-2"), "US").with_tag("commercial");

    registry.register(entry1).unwrap();
    registry.register(entry2).unwrap();

    // Export backup
    let json = registry.export_backup(Some("Test".to_string())).unwrap();
    assert!(!json.is_empty());

    // Create a new registry and import
    let mut new_registry = StatuteRegistry::new();
    new_registry.import_backup(&json).unwrap();

    // Verify the data was restored
    assert_eq!(new_registry.count(), 2);
    assert!(new_registry.get_uncached("statute-1").is_some());
    assert!(new_registry.get_uncached("statute-2").is_some());
    assert_eq!(new_registry.event_count(), 2);
}

#[test]
fn test_restore_from_backup() {
    let mut registry = StatuteRegistry::new();

    let entry1 = StatuteEntry::new(test_statute("statute-1"), "JP").with_tag("civil");
    let entry2 = StatuteEntry::new(test_statute("statute-2"), "US").with_tag("commercial");

    registry.register(entry1).unwrap();
    registry.register(entry2).unwrap();

    let backup = registry.create_backup(None);

    // Create a new registry and restore
    let mut new_registry = StatuteRegistry::new();
    new_registry.restore_from_backup(backup).unwrap();

    // Verify the data was restored
    assert_eq!(new_registry.count(), 2);
    assert!(new_registry.get_uncached("statute-1").is_some());
    assert!(new_registry.get_uncached("statute-2").is_some());

    // Verify tags were restored
    let civil_statutes = new_registry.query_by_tag("civil");
    assert_eq!(civil_statutes.len(), 1);

    // Verify jurisdictions were restored
    let jp_statutes = new_registry.query_by_jurisdiction("JP");
    assert_eq!(jp_statutes.len(), 1);
}

#[test]
fn test_merge_backup() {
    let mut registry1 = StatuteRegistry::new();
    let mut registry2 = StatuteRegistry::new();

    // Registry 1 has statute-1 and statute-2
    let entry1 = StatuteEntry::new(test_statute("statute-1"), "JP");
    let entry2 = StatuteEntry::new(test_statute("statute-2"), "JP");
    registry1.register(entry1).unwrap();
    registry1.register(entry2).unwrap();

    // Registry 2 has statute-2 and statute-3
    let entry2_dup = StatuteEntry::new(test_statute("statute-2"), "JP");
    let entry3 = StatuteEntry::new(test_statute("statute-3"), "JP");
    registry2.register(entry2_dup).unwrap();
    registry2.register(entry3).unwrap();

    let backup2 = registry2.create_backup(None);

    // Merge registry2 into registry1
    let merged_ids = registry1.merge_backup(backup2).unwrap();

    // Only statute-3 should be merged (statute-2 already exists)
    assert_eq!(merged_ids.len(), 1);
    assert_eq!(merged_ids[0], "statute-3");

    // Registry1 should now have all three statutes
    assert_eq!(registry1.count(), 3);
    assert!(registry1.get_uncached("statute-1").is_some());
    assert!(registry1.get_uncached("statute-2").is_some());
    assert!(registry1.get_uncached("statute-3").is_some());
}

#[test]
fn test_backup_preserves_version_history() {
    let mut registry = StatuteRegistry::new();

    let entry = StatuteEntry::new(test_statute("statute-1"), "JP");
    registry.register(entry).unwrap();

    // Create multiple versions
    registry
        .update("statute-1", test_statute("statute-1"))
        .unwrap();
    registry
        .update("statute-1", test_statute("statute-1"))
        .unwrap();

    let versions_before = registry.list_versions("statute-1");
    assert_eq!(versions_before, vec![1, 2, 3]);

    // Create backup and restore
    let backup = registry.create_backup(None);
    let mut new_registry = StatuteRegistry::new();
    new_registry.restore_from_backup(backup).unwrap();

    // Verify version history was preserved
    let versions_after = new_registry.list_versions("statute-1");
    assert_eq!(versions_after, vec![1, 2, 3]);

    // Verify we can retrieve old versions
    let v1 = new_registry.get_version("statute-1", 1).unwrap();
    assert_eq!(v1.version, 1);

    let v2 = new_registry.get_version("statute-1", 2).unwrap();
    assert_eq!(v2.version, 2);
}

#[test]
fn test_multi_tenant_create_and_list() {
    let mut mt_registry = MultiTenantRegistry::new();

    mt_registry.create_tenant("tenant1").unwrap();
    mt_registry.create_tenant("tenant2").unwrap();
    mt_registry.create_tenant("tenant3").unwrap();

    assert_eq!(mt_registry.tenant_count(), 3);

    let tenants = mt_registry.list_tenants();
    assert_eq!(tenants.len(), 3);
    assert!(tenants.contains(&&"tenant1".to_string()));
    assert!(tenants.contains(&&"tenant2".to_string()));
    assert!(tenants.contains(&&"tenant3".to_string()));
}

#[test]
fn test_multi_tenant_isolation() {
    let mut mt_registry = MultiTenantRegistry::new();

    mt_registry.create_tenant("tenant1").unwrap();
    mt_registry.create_tenant("tenant2").unwrap();

    // Add statute to tenant1
    let entry1 = StatuteEntry::new(test_statute("statute-1"), "JP");
    mt_registry
        .get_tenant_mut("tenant1")
        .unwrap()
        .register(entry1)
        .unwrap();

    // Add statute to tenant2
    let entry2 = StatuteEntry::new(test_statute("statute-2"), "US");
    mt_registry
        .get_tenant_mut("tenant2")
        .unwrap()
        .register(entry2)
        .unwrap();

    // Verify isolation
    let tenant1 = mt_registry.get_tenant("tenant1").unwrap();
    assert_eq!(tenant1.count(), 1);
    assert!(tenant1.get_uncached("statute-1").is_some());
    assert!(tenant1.get_uncached("statute-2").is_none());

    let tenant2 = mt_registry.get_tenant("tenant2").unwrap();
    assert_eq!(tenant2.count(), 1);
    assert!(tenant2.get_uncached("statute-1").is_none());
    assert!(tenant2.get_uncached("statute-2").is_some());
}

#[test]
fn test_multi_tenant_default() {
    let mut mt_registry = MultiTenantRegistry::with_default_tenant("default");

    assert_eq!(mt_registry.tenant_count(), 1);
    assert!(mt_registry.has_tenant("default"));

    // Can access default tenant
    let default = mt_registry.get_default().unwrap();
    assert_eq!(default.count(), 0);

    // Add statute to default tenant
    let entry = StatuteEntry::new(test_statute("statute-1"), "JP");
    mt_registry
        .get_default_mut()
        .unwrap()
        .register(entry)
        .unwrap();

    assert_eq!(mt_registry.get_default().unwrap().count(), 1);
}

#[test]
fn test_multi_tenant_delete() {
    let mut mt_registry = MultiTenantRegistry::new();

    mt_registry.create_tenant("tenant1").unwrap();
    mt_registry.create_tenant("tenant2").unwrap();

    assert_eq!(mt_registry.tenant_count(), 2);

    mt_registry.delete_tenant("tenant1").unwrap();
    assert_eq!(mt_registry.tenant_count(), 1);
    assert!(!mt_registry.has_tenant("tenant1"));
    assert!(mt_registry.has_tenant("tenant2"));

    // Deleting non-existent tenant should fail
    assert!(mt_registry.delete_tenant("tenant1").is_err());
}

#[test]
fn test_multi_tenant_clone() {
    let mut mt_registry = MultiTenantRegistry::new();

    mt_registry.create_tenant("source").unwrap();

    // Add some data to source
    let entry1 = StatuteEntry::new(test_statute("statute-1"), "JP");
    let entry2 = StatuteEntry::new(test_statute("statute-2"), "US");
    mt_registry
        .get_tenant_mut("source")
        .unwrap()
        .register(entry1)
        .unwrap();
    mt_registry
        .get_tenant_mut("source")
        .unwrap()
        .register(entry2)
        .unwrap();

    // Clone to new tenant
    mt_registry.clone_tenant("source", "clone").unwrap();

    // Verify clone has the same data
    let clone = mt_registry.get_tenant("clone").unwrap();
    assert_eq!(clone.count(), 2);
    assert!(clone.get_uncached("statute-1").is_some());
    assert!(clone.get_uncached("statute-2").is_some());

    // Verify independence - add to source
    let entry3 = StatuteEntry::new(test_statute("statute-3"), "FR");
    mt_registry
        .get_tenant_mut("source")
        .unwrap()
        .register(entry3)
        .unwrap();

    // Clone should still have 2
    assert_eq!(mt_registry.get_tenant("clone").unwrap().count(), 2);
    assert_eq!(mt_registry.get_tenant("source").unwrap().count(), 3);
}

#[test]
fn test_multi_tenant_statistics() {
    let mut mt_registry = MultiTenantRegistry::new();

    mt_registry.create_tenant("tenant1").unwrap();
    mt_registry.create_tenant("tenant2").unwrap();

    // Add statutes to tenant1
    let entry1 = StatuteEntry::new(test_statute("statute-1"), "JP")
        .with_tag("civil")
        .with_status(StatuteStatus::Active);
    mt_registry
        .get_tenant_mut("tenant1")
        .unwrap()
        .register(entry1)
        .unwrap();

    // Add statutes to tenant2
    let entry2 = StatuteEntry::new(test_statute("statute-2"), "US")
        .with_tag("commercial")
        .with_status(StatuteStatus::Draft);
    mt_registry
        .get_tenant_mut("tenant2")
        .unwrap()
        .register(entry2)
        .unwrap();

    let stats = mt_registry.tenant_statistics();

    assert_eq!(stats.len(), 2);

    let tenant1_stats = stats.get("tenant1").unwrap();
    assert_eq!(tenant1_stats.statute_count, 1);
    assert_eq!(tenant1_stats.active_statute_count, 1);
    assert_eq!(tenant1_stats.event_count, 1);
    assert_eq!(tenant1_stats.tag_count, 1);
    assert_eq!(tenant1_stats.jurisdiction_count, 1);

    let tenant2_stats = stats.get("tenant2").unwrap();
    assert_eq!(tenant2_stats.statute_count, 1);
    assert_eq!(tenant2_stats.active_statute_count, 0); // Draft status
    assert_eq!(tenant2_stats.event_count, 1);
}

#[test]
fn test_multi_tenant_export_import() {
    let mut mt_registry = MultiTenantRegistry::new();

    mt_registry.create_tenant("tenant1").unwrap();

    // Add data
    let entry = StatuteEntry::new(test_statute("statute-1"), "JP");
    mt_registry
        .get_tenant_mut("tenant1")
        .unwrap()
        .register(entry)
        .unwrap();

    // Export
    let backup_json = mt_registry
        .export_tenant("tenant1", Some("Test export".to_string()))
        .unwrap();

    // Create new tenant and import
    mt_registry.create_tenant("tenant2").unwrap();
    mt_registry.import_tenant("tenant2", &backup_json).unwrap();

    // Verify import
    let tenant2 = mt_registry.get_tenant("tenant2").unwrap();
    assert_eq!(tenant2.count(), 1);
    assert!(tenant2.get_uncached("statute-1").is_some());
}

#[test]
fn test_multi_tenant_set_default() {
    let mut mt_registry = MultiTenantRegistry::new();

    mt_registry.create_tenant("tenant1").unwrap();
    mt_registry.create_tenant("tenant2").unwrap();

    // Set default
    mt_registry.set_default_tenant("tenant1").unwrap();

    // Verify default
    let entry = StatuteEntry::new(test_statute("statute-1"), "JP");
    mt_registry
        .get_default_mut()
        .unwrap()
        .register(entry)
        .unwrap();

    assert_eq!(mt_registry.get_default().unwrap().count(), 1);
    assert_eq!(mt_registry.get_tenant("tenant1").unwrap().count(), 1);
    assert_eq!(mt_registry.get_tenant("tenant2").unwrap().count(), 0);

    // Change default
    mt_registry.set_default_tenant("tenant2").unwrap();
    let entry2 = StatuteEntry::new(test_statute("statute-2"), "US");
    mt_registry
        .get_default_mut()
        .unwrap()
        .register(entry2)
        .unwrap();

    assert_eq!(mt_registry.get_tenant("tenant2").unwrap().count(), 1);
}

#[test]
fn test_lazy_loading_summaries() {
    let mut registry = StatuteRegistry::new();

    let entry1 = StatuteEntry::new(test_statute("statute-1"), "JP")
        .with_tag("civil")
        .with_status(StatuteStatus::Active);
    let entry2 = StatuteEntry::new(test_statute("statute-2"), "US")
        .with_tag("commercial")
        .with_status(StatuteStatus::Draft);

    registry.register(entry1).unwrap();
    registry.register(entry2).unwrap();

    // Get summaries (lazy loaded)
    let summaries = registry.list_summaries();
    assert_eq!(summaries.len(), 2);

    // Verify summary contains essential data
    let summary1 = summaries
        .iter()
        .find(|s| s.statute_id == "statute-1")
        .unwrap();
    assert_eq!(summary1.title, "Test statute-1");
    assert_eq!(summary1.jurisdiction, "JP");
    assert_eq!(summary1.status, StatuteStatus::Active);
    assert!(summary1.tags.contains(&"civil".to_string()));
    assert!(summary1.is_active);

    let summary2 = summaries
        .iter()
        .find(|s| s.statute_id == "statute-2")
        .unwrap();
    assert_eq!(summary2.title, "Test statute-2");
    assert_eq!(summary2.jurisdiction, "US");
    assert_eq!(summary2.status, StatuteStatus::Draft);
    assert!(!summary2.is_active);
}

#[test]
fn test_lazy_loading_summaries_paged() {
    let mut registry = StatuteRegistry::new();

    for i in 0..25 {
        registry
            .register(StatuteEntry::new(
                test_statute(&format!("statute-{}", i)),
                "JP",
            ))
            .unwrap();
    }

    // First page
    let page1 = registry.list_summaries_paged(Pagination::new(0, 10));
    assert_eq!(page1.items.len(), 10);
    assert_eq!(page1.total, 25);
    assert_eq!(page1.total_pages, 3);

    // Last page
    let page3 = registry.list_summaries_paged(Pagination::new(2, 10));
    assert_eq!(page3.items.len(), 5);
}

#[test]
fn test_search_summaries() {
    let mut registry = StatuteRegistry::new();

    let entry1 = StatuteEntry::new(test_statute("civil-1"), "JP")
        .with_tag("civil")
        .with_status(StatuteStatus::Active);
    let entry2 = StatuteEntry::new(test_statute("criminal-1"), "JP")
        .with_tag("criminal")
        .with_status(StatuteStatus::Draft);

    registry.register(entry1).unwrap();
    registry.register(entry2).unwrap();

    // Search for active statutes
    let query = SearchQuery::new().with_status(StatuteStatus::Active);
    let summaries = registry.search_summaries(&query);
    assert_eq!(summaries.len(), 1);
    assert_eq!(summaries[0].statute_id, "civil-1");
}

#[test]
fn test_search_summaries_paged() {
    let mut registry = StatuteRegistry::new();

    for i in 0..15 {
        registry
            .register(
                StatuteEntry::new(test_statute(&format!("civil-{}", i)), "JP").with_tag("civil"),
            )
            .unwrap();
    }

    for i in 0..10 {
        registry
            .register(
                StatuteEntry::new(test_statute(&format!("criminal-{}", i)), "JP")
                    .with_tag("criminal"),
            )
            .unwrap();
    }

    let query = SearchQuery::new().with_tag("civil");
    let page1 = registry.search_summaries_paged(&query, Pagination::new(0, 10));

    assert_eq!(page1.items.len(), 10);
    assert_eq!(page1.total, 15);
    assert_eq!(page1.total_pages, 2);
}

#[test]
fn test_query_summaries_by_tag() {
    let mut registry = StatuteRegistry::new();

    registry
        .register(StatuteEntry::new(test_statute("civil-1"), "JP").with_tag("civil"))
        .unwrap();
    registry
        .register(StatuteEntry::new(test_statute("criminal-1"), "JP").with_tag("criminal"))
        .unwrap();

    let summaries = registry.query_summaries_by_tag("civil");
    assert_eq!(summaries.len(), 1);
    assert_eq!(summaries[0].statute_id, "civil-1");
}

#[test]
fn test_query_summaries_by_jurisdiction() {
    let mut registry = StatuteRegistry::new();

    registry
        .register(StatuteEntry::new(test_statute("statute-1"), "JP"))
        .unwrap();
    registry
        .register(StatuteEntry::new(test_statute("statute-2"), "US"))
        .unwrap();

    let summaries = registry.query_summaries_by_jurisdiction("JP");
    assert_eq!(summaries.len(), 1);
    assert_eq!(summaries[0].statute_id, "statute-1");
}

#[test]
fn test_list_active_summaries() {
    let mut registry = StatuteRegistry::new();

    registry
        .register(
            StatuteEntry::new(test_statute("active-1"), "JP").with_status(StatuteStatus::Active),
        )
        .unwrap();
    registry
        .register(
            StatuteEntry::new(test_statute("draft-1"), "JP").with_status(StatuteStatus::Draft),
        )
        .unwrap();

    let summaries = registry.list_active_summaries();
    assert_eq!(summaries.len(), 1);
    assert_eq!(summaries[0].statute_id, "active-1");
}

#[test]
fn test_lazy_load_config() {
    let config_all = LazyLoadConfig::all();
    assert!(config_all.lazy_content);
    assert!(config_all.lazy_versions);
    assert!(config_all.lazy_events);

    let config_none = LazyLoadConfig::none();
    assert!(!config_none.lazy_content);
    assert!(!config_none.lazy_versions);
    assert!(!config_none.lazy_events);

    let config_default = LazyLoadConfig::default();
    assert!(!config_default.lazy_content);
}

#[test]
fn test_webhook_subscription() {
    use std::sync::{Arc, Mutex};

    let mut registry = StatuteRegistry::new();
    let counter = Arc::new(Mutex::new(0));
    let counter_clone = counter.clone();

    // Subscribe to all events
    let webhook_id = registry.subscribe_webhook(
        Some("Test Webhook".to_string()),
        Some(WebhookEventFilter::All),
        move |_event| {
            let mut count = counter_clone.lock().unwrap();
            *count += 1;
        },
    );

    assert_eq!(registry.webhook_count(), 1);

    // Register a statute - should trigger webhook
    let entry = StatuteEntry::new(test_statute("statute-1"), "JP");
    registry.register(entry).unwrap();

    assert_eq!(*counter.lock().unwrap(), 1);

    // Unsubscribe
    assert!(registry.unsubscribe_webhook(webhook_id));
    assert_eq!(registry.webhook_count(), 0);
}

#[test]
fn test_webhook_filtered_events() {
    use std::sync::{Arc, Mutex};

    let mut registry = StatuteRegistry::new();
    let status_change_count = Arc::new(Mutex::new(0));
    let status_change_clone = status_change_count.clone();

    // Subscribe only to status changes
    registry.subscribe_webhook(
        None,
        Some(WebhookEventFilter::StatusChanged),
        move |_event| {
            let mut count = status_change_clone.lock().unwrap();
            *count += 1;
        },
    );

    // Register statute - should NOT trigger webhook
    let entry = StatuteEntry::new(test_statute("statute-1"), "JP");
    registry.register(entry).unwrap();
    assert_eq!(*status_change_count.lock().unwrap(), 0);

    // Change status - SHOULD trigger webhook
    registry
        .set_status("statute-1", StatuteStatus::Active)
        .unwrap();
    assert_eq!(*status_change_count.lock().unwrap(), 1);

    // Update statute - should NOT trigger webhook
    registry
        .update("statute-1", test_statute("statute-1"))
        .unwrap();
    assert_eq!(*status_change_count.lock().unwrap(), 1);
}

#[test]
fn test_multiple_webhooks() {
    use std::sync::{Arc, Mutex};

    let mut registry = StatuteRegistry::new();
    let counter1 = Arc::new(Mutex::new(0));
    let counter2 = Arc::new(Mutex::new(0));
    let counter1_clone = counter1.clone();
    let counter2_clone = counter2.clone();

    // First webhook - all events
    registry.subscribe_webhook(None, Some(WebhookEventFilter::All), move |_event| {
        let mut count = counter1_clone.lock().unwrap();
        *count += 1;
    });

    // Second webhook - only registrations
    registry.subscribe_webhook(
        None,
        Some(WebhookEventFilter::StatuteRegistered),
        move |_event| {
            let mut count = counter2_clone.lock().unwrap();
            *count += 1;
        },
    );

    assert_eq!(registry.webhook_count(), 2);

    // Register statute - both should trigger
    let entry = StatuteEntry::new(test_statute("statute-1"), "JP");
    registry.register(entry).unwrap();

    assert_eq!(*counter1.lock().unwrap(), 1);
    assert_eq!(*counter2.lock().unwrap(), 1);

    // Update statute - only first should trigger
    registry
        .update("statute-1", test_statute("statute-1"))
        .unwrap();

    assert_eq!(*counter1.lock().unwrap(), 2);
    assert_eq!(*counter2.lock().unwrap(), 1);
}

#[test]
fn test_webhook_event_filter_matching() {
    // Test StatuteRegistered filter
    let filter = WebhookEventFilter::StatuteRegistered;
    let event = RegistryEvent::StatuteRegistered {
        registry_id: Uuid::new_v4(),
        statute_id: "test".to_string(),
        jurisdiction: "JP".to_string(),
        timestamp: Utc::now(),
    };
    assert!(filter.matches(&event));

    let other_event = RegistryEvent::StatuteUpdated {
        statute_id: "test".to_string(),
        old_version: 1,
        new_version: 2,
        timestamp: Utc::now(),
    };
    assert!(!filter.matches(&other_event));

    // Test All filter
    let all_filter = WebhookEventFilter::All;
    assert!(all_filter.matches(&event));
    assert!(all_filter.matches(&other_event));
}

#[test]
fn test_list_webhooks() {
    let registry = StatuteRegistry::new();

    let id1 = registry.subscribe_webhook(
        Some("Webhook 1".to_string()),
        Some(WebhookEventFilter::All),
        |_| {},
    );
    let id2 = registry.subscribe_webhook(None, Some(WebhookEventFilter::StatusChanged), |_| {});

    let webhooks = registry.list_webhooks();
    assert_eq!(webhooks.len(), 2);

    let (webhook1_id, webhook1_name) = &webhooks[0];
    assert_eq!(webhook1_id, &id1);
    assert_eq!(webhook1_name, &Some("Webhook 1".to_string()));

    let (webhook2_id, webhook2_name) = &webhooks[1];
    assert_eq!(webhook2_id, &id2);
    assert_eq!(webhook2_name, &None);
}

#[test]
fn test_clear_webhooks() {
    let registry = StatuteRegistry::new();

    registry.subscribe_webhook(None, Some(WebhookEventFilter::All), |_| {});
    registry.subscribe_webhook(None, Some(WebhookEventFilter::All), |_| {});

    assert_eq!(registry.webhook_count(), 2);

    registry.clear_webhooks();
    assert_eq!(registry.webhook_count(), 0);
}

// =============================================================================
// Transaction Tests
// =============================================================================

#[test]
fn test_transaction_register() {
    use crate::transaction::Transaction;

    let mut registry = StatuteRegistry::new();

    let entry1 = StatuteEntry::new(test_statute("statute-1"), "JP");
    let entry2 = StatuteEntry::new(test_statute("statute-2"), "JP");

    let tx = Transaction::new().register(entry1).register(entry2);

    let result = tx.commit(&mut registry).unwrap();

    assert!(result.is_success());
    assert_eq!(result.successful, 2);
    assert_eq!(result.failed, 0);
    assert_eq!(registry.count(), 2);
}

#[test]
fn test_transaction_mixed_operations() {
    use crate::transaction::Transaction;

    let mut registry = StatuteRegistry::new();

    // Register a statute first
    let entry = StatuteEntry::new(test_statute("statute-1"), "JP");
    registry.register(entry).unwrap();

    // Create a transaction with mixed operations
    let tx = Transaction::new()
        .add_tag("statute-1", "test-tag")
        .add_metadata("statute-1", "key1", "value1")
        .set_status("statute-1", StatuteStatus::Active);

    let result = tx.commit(&mut registry).unwrap();

    assert!(result.is_success());
    assert_eq!(result.successful, 3);
    assert_eq!(result.failed, 0);

    // Verify the changes
    let statute = registry.get_uncached("statute-1").unwrap();
    assert!(statute.tags.contains(&"test-tag".to_string()));
    assert_eq!(statute.metadata.get("key1"), Some(&"value1".to_string()));
    assert_eq!(statute.status, StatuteStatus::Active);
}

#[test]
fn test_transaction_partial_failure() {
    use crate::transaction::Transaction;

    let mut registry = StatuteRegistry::new();

    // Register one statute
    let entry1 = StatuteEntry::new(test_statute("statute-1"), "JP");
    registry.register(entry1).unwrap();

    // Create a transaction that includes an operation on a non-existent statute
    let tx = Transaction::new()
        .add_tag("statute-1", "tag1")
        .add_tag("non-existent", "tag2")
        .add_metadata("statute-1", "key1", "value1");

    let result = tx.commit(&mut registry).unwrap();

    assert!(result.has_failures());
    assert_eq!(result.successful, 2); // tag1 and metadata
    assert_eq!(result.failed, 1); // non-existent statute

    // Verify partial success
    let statute = registry.get_uncached("statute-1").unwrap();
    assert!(statute.tags.contains(&"tag1".to_string()));
    assert_eq!(statute.metadata.get("key1"), Some(&"value1".to_string()));
}

#[test]
fn test_add_tag() {
    let mut registry = StatuteRegistry::new();

    let entry = StatuteEntry::new(test_statute("statute-1"), "JP");
    registry.register(entry).unwrap();

    // Add a tag
    registry.add_tag("statute-1", "criminal-law").unwrap();

    let statute = registry.get_uncached("statute-1").unwrap();
    assert!(statute.tags.contains(&"criminal-law".to_string()));

    // Verify tag index
    let statutes_with_tag = registry.query_by_tag("criminal-law");
    assert_eq!(statutes_with_tag.len(), 1);
}

#[test]
fn test_remove_tag() {
    let mut registry = StatuteRegistry::new();

    let mut entry = StatuteEntry::new(test_statute("statute-1"), "JP");
    entry = entry.with_tag("criminal-law");
    registry.register(entry).unwrap();

    // Remove the tag
    registry.remove_tag("statute-1", "criminal-law").unwrap();

    let statute = registry.get_uncached("statute-1").unwrap();
    assert!(!statute.tags.contains(&"criminal-law".to_string()));

    // Verify tag index
    let statutes_with_tag = registry.query_by_tag("criminal-law");
    assert_eq!(statutes_with_tag.len(), 0);
}

#[test]
fn test_add_metadata() {
    let mut registry = StatuteRegistry::new();

    let entry = StatuteEntry::new(test_statute("statute-1"), "JP");
    registry.register(entry).unwrap();

    // Add metadata
    registry
        .add_metadata("statute-1", "author", "Test Author")
        .unwrap();

    let statute = registry.get_uncached("statute-1").unwrap();
    assert_eq!(
        statute.metadata.get("author"),
        Some(&"Test Author".to_string())
    );
}

#[test]
fn test_remove_metadata() {
    let mut registry = StatuteRegistry::new();

    let entry = StatuteEntry::new(test_statute("statute-1"), "JP");
    registry.register(entry).unwrap();

    // Add and then remove metadata
    registry
        .add_metadata("statute-1", "author", "Test Author")
        .unwrap();
    registry.remove_metadata("statute-1", "author").unwrap();

    let statute = registry.get_uncached("statute-1").unwrap();
    assert_eq!(statute.metadata.get("author"), None);
}

// =============================================================================
// Concurrent Access Tests
// =============================================================================

#[test]
fn test_concurrent_reads() {
    use std::sync::Arc;
    use std::thread;

    let mut registry = StatuteRegistry::new();

    // Register some statutes
    for i in 1..=10 {
        let entry = StatuteEntry::new(test_statute(&format!("statute-{}", i)), "JP");
        registry.register(entry).unwrap();
    }

    let registry = Arc::new(Mutex::new(registry));
    let mut handles = vec![];

    // Spawn multiple reader threads
    for _ in 0..5 {
        let registry_clone = Arc::clone(&registry);
        let handle = thread::spawn(move || {
            for i in 1..=10 {
                let registry = registry_clone.lock().unwrap();
                let statute_id = format!("statute-{}", i);
                assert!(registry.get_uncached(&statute_id).is_some());
            }
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_concurrent_writes() {
    use std::sync::Arc;
    use std::thread;

    let registry = StatuteRegistry::new();
    let registry = Arc::new(Mutex::new(registry));
    let mut handles = vec![];

    // Spawn multiple writer threads
    for i in 1..=5 {
        let registry_clone = Arc::clone(&registry);
        let handle = thread::spawn(move || {
            let mut registry = registry_clone.lock().unwrap();
            let entry = StatuteEntry::new(test_statute(&format!("statute-{}", i)), "JP");
            registry.register(entry).unwrap();
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Verify all statutes were registered
    let registry = registry.lock().unwrap();
    assert_eq!(registry.count(), 5);
}

#[test]
fn test_concurrent_mixed_operations() {
    use std::sync::Arc;
    use std::thread;

    let mut registry = StatuteRegistry::new();

    // Register initial statutes
    for i in 1..=3 {
        let entry = StatuteEntry::new(test_statute(&format!("statute-{}", i)), "JP");
        registry.register(entry).unwrap();
    }

    let registry = Arc::new(Mutex::new(registry));
    let mut handles = vec![];

    // Reader threads
    for _ in 0..3 {
        let registry_clone = Arc::clone(&registry);
        let handle = thread::spawn(move || {
            let registry = registry_clone.lock().unwrap();
            let _count = registry.count();
            let _list = registry.list();
        });
        handles.push(handle);
    }

    // Writer threads
    for i in 4..=6 {
        let registry_clone = Arc::clone(&registry);
        let handle = thread::spawn(move || {
            let mut registry = registry_clone.lock().unwrap();
            let statute_id = format!("statute-{}", i);
            let entry = StatuteEntry::new(test_statute(&statute_id), "JP");
            let _ = registry.register(entry);
        });
        handles.push(handle);
    }

    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }

    // Final count should be 6
    let registry = registry.lock().unwrap();
    assert_eq!(registry.count(), 6);
}

#[test]
fn test_optimistic_concurrency_with_etag() {
    let mut registry = StatuteRegistry::new();

    let entry = StatuteEntry::new(test_statute("statute-1"), "JP");
    registry.register(entry).unwrap();

    // Get the current ETag
    let statute = registry.get_uncached("statute-1").unwrap();
    let etag = statute.etag.clone();

    // Successful update with correct ETag
    let result = registry.update_with_etag("statute-1", test_statute("statute-1"), &etag);
    assert!(result.is_ok());

    // Failed update with outdated ETag
    let result = registry.update_with_etag("statute-1", test_statute("statute-1"), &etag);
    assert!(result.is_err());
    match result {
        Err(RegistryError::ConcurrentModification { .. }) => {}
        _ => panic!("Expected ConcurrentModification error"),
    }
}
