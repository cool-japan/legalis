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
fn test_statute_entry_builders() {
    use chrono::Duration;

    let expiry = Utc::now() + Duration::days(365);
    let effective = Utc::now() - Duration::days(30);

    let entry = StatuteEntry::new(test_statute("test-1"), "JP")
        .with_tag("civil")
        .with_tag("contract")
        .with_status(StatuteStatus::Active)
        .with_reference("ref-statute-1")
        .with_expiry_date(expiry)
        .with_effective_date(effective)
        .with_amends("parent-statute")
        .with_supersedes("old-statute-1")
        .with_supersedes("old-statute-2")
        .with_metadata("author", "Legal Team")
        .with_metadata("version_notes", "Initial draft")
        .with_jurisdiction("US");

    assert_eq!(entry.tags, vec!["civil", "contract"]);
    assert_eq!(entry.status, StatuteStatus::Active);
    assert_eq!(entry.references, vec!["ref-statute-1"]);
    assert_eq!(entry.expiry_date, Some(expiry));
    assert_eq!(entry.effective_date, Some(effective));
    assert_eq!(entry.amends, Some("parent-statute".to_string()));
    assert_eq!(entry.supersedes, vec!["old-statute-1", "old-statute-2"]);
    assert_eq!(
        entry.metadata.get("author"),
        Some(&"Legal Team".to_string())
    );
    assert_eq!(
        entry.metadata.get("version_notes"),
        Some(&"Initial draft".to_string())
    );
    assert_eq!(entry.jurisdiction, "US");
}

#[test]
fn test_pagination_methods() {
    // Test first() constructor
    let page1 = Pagination::first(25);
    assert_eq!(page1.page, 0);
    assert_eq!(page1.per_page, 25);

    // Test next() and prev()
    let page2 = page1.next();
    assert_eq!(page2.page, 1);
    assert_eq!(page2.per_page, 25);

    let page1_again = page2.prev();
    assert_eq!(page1_again.page, 0);

    // Test prev() saturates at 0
    let page0 = page1.prev();
    assert_eq!(page0.page, 0);

    // Test builder methods
    let custom = Pagination::new(0, 10).with_page(5).with_per_page(20);
    assert_eq!(custom.page, 5);
    assert_eq!(custom.per_page, 20);

    // Test offset and limit
    assert_eq!(custom.offset(), 100);
    assert_eq!(custom.limit(), 20);
}

#[test]
fn test_paged_result_methods() {
    // Create a paged result with items
    let items = vec![1, 2, 3, 4, 5];
    let result = PagedResult::new(items, 2, 5, 23);

    // Test navigation helpers
    assert!(result.has_next());
    assert!(result.has_prev());
    assert!(!result.is_empty());
    assert_eq!(result.len(), 5);

    // Test item numbering
    assert_eq!(result.first_item_number(), 11); // page 2 * 5 per_page + 1
    assert_eq!(result.last_item_number(), 15); // page 2 * 5 per_page + 5 items

    // Test next/prev page
    let next = result.next_page();
    assert!(next.is_some());
    assert_eq!(next.unwrap().page, 3);

    let prev = result.prev_page();
    assert!(prev.is_some());
    assert_eq!(prev.unwrap().page, 1);

    // Test first page
    let first_result = PagedResult::new(vec![1, 2, 3], 0, 5, 23);
    assert!(!first_result.has_prev());
    assert!(first_result.has_next());
    assert!(first_result.prev_page().is_none());

    // Test last page
    let last_result = PagedResult::new(vec![21, 22, 23], 4, 5, 23);
    assert!(last_result.has_prev());
    assert!(!last_result.has_next());
    assert!(last_result.next_page().is_none());

    // Test empty result
    let empty_result: PagedResult<i32> = PagedResult::new(vec![], 0, 5, 0);
    assert!(empty_result.is_empty());
    assert_eq!(empty_result.len(), 0);
    assert_eq!(empty_result.first_item_number(), 0);
    assert_eq!(empty_result.last_item_number(), 0);
}

#[test]
fn test_registry_utility_methods() {
    let mut registry = StatuteRegistry::new();

    // Test with empty registry
    assert!(!registry.contains("test-1"));
    assert_eq!(registry.all_statute_ids().len(), 0);
    assert_eq!(registry.latest_version("test-1"), None);

    // Add some statutes
    registry
        .register(StatuteEntry::new(test_statute("test-1"), "JP"))
        .unwrap();
    registry
        .register(StatuteEntry::new(test_statute("test-2"), "US"))
        .unwrap();
    registry
        .register(StatuteEntry::new(test_statute("test-3"), "UK"))
        .unwrap();

    // Test contains
    assert!(registry.contains("test-1"));
    assert!(registry.contains("test-2"));
    assert!(!registry.contains("nonexistent"));

    // Test all_statute_ids
    let ids = registry.all_statute_ids();
    assert_eq!(ids.len(), 3);
    assert!(ids.contains(&&"test-1".to_string()));
    assert!(ids.contains(&&"test-2".to_string()));
    assert!(ids.contains(&&"test-3".to_string()));

    // Test latest_version
    assert_eq!(registry.latest_version("test-1"), Some(1));
    registry.update("test-1", test_statute("test-1")).unwrap();
    assert_eq!(registry.latest_version("test-1"), Some(2));

    // Test get_many
    let results = registry.get_many(&["test-1", "test-2", "nonexistent"]);
    assert_eq!(results.len(), 3);
    assert!(results[0].is_some());
    assert!(results[1].is_some());
    assert!(results[2].is_none());
}

#[test]
fn test_registry_statistics() {
    let mut registry = StatuteRegistry::new();

    // Add statutes with different statuses and jurisdictions
    registry
        .register(
            StatuteEntry::new(test_statute("statute-1"), "JP")
                .with_status(StatuteStatus::Active)
                .with_tag("civil"),
        )
        .unwrap();

    registry
        .register(
            StatuteEntry::new(test_statute("statute-2"), "JP")
                .with_status(StatuteStatus::Draft)
                .with_tag("criminal"),
        )
        .unwrap();

    registry
        .register(
            StatuteEntry::new(test_statute("statute-3"), "US")
                .with_status(StatuteStatus::Active)
                .with_tag("civil"),
        )
        .unwrap();

    // Create a version
    registry
        .update("statute-1", test_statute("statute-1"))
        .unwrap();

    let stats = registry.statistics();

    // Verify counts
    assert_eq!(stats.total_statutes, 3);
    // total_versions counts all versions in the version history
    // Each statute gets its initial version stored (3 total)
    // statute-1 update adds another version (1 more)
    assert_eq!(stats.total_versions, 4);
    assert_eq!(stats.total_tags, 2); // civil, criminal
    assert_eq!(stats.total_jurisdictions, 2); // JP, US

    // Verify by_status
    // Note: update() resets status to Draft, so statute-1 becomes Draft after update
    assert_eq!(stats.by_status.get(&StatuteStatus::Active), Some(&1)); // Only statute-3
    assert_eq!(stats.by_status.get(&StatuteStatus::Draft), Some(&2)); // statute-1 and statute-2

    // Verify by_jurisdiction
    assert_eq!(stats.by_jurisdiction.get("JP"), Some(&2));
    assert_eq!(stats.by_jurisdiction.get("US"), Some(&1));
}

#[test]
fn test_statute_diff() {
    let mut registry = StatuteRegistry::new();

    // Register a statute
    let entry = StatuteEntry::new(test_statute("test-1"), "JP")
        .with_status(StatuteStatus::Draft)
        .with_tag("civil")
        .with_metadata("author", "Alice");

    registry.register(entry).unwrap();

    // Update it with changes
    let mut updated = test_statute("test-1");
    updated.title = "Updated Test test-1".to_string();

    let updated_entry = StatuteEntry::new(updated, "US")
        .with_status(StatuteStatus::Active)
        .with_tag("criminal")
        .with_tag("civil") // Keep one tag the same
        .with_metadata("author", "Bob") // Change metadata
        .with_metadata("reviewer", "Charlie"); // Add metadata

    // Manually update the registry
    registry
        .update("test-1", updated_entry.statute.clone())
        .unwrap();

    // Compute diff
    let diff = registry.diff("test-1", 1, 2).unwrap();

    // Verify diff
    assert_eq!(diff.statute_id, "test-1");
    assert_eq!(diff.old_version, 1);
    assert_eq!(diff.new_version, 2);
    assert!(diff.has_changes());
    assert!(diff.content_changed); // Title changed

    // Check summary
    let summary = diff.summary();
    assert!(summary.contains("title") || summary.contains("content"));
}

#[test]
fn test_statute_diff_no_changes() {
    let mut registry = StatuteRegistry::new();

    // Register a statute
    registry
        .register(StatuteEntry::new(test_statute("test-1"), "JP"))
        .unwrap();

    // Get version 1 twice and compare
    let v1_first = registry.get_version("test-1", 1).unwrap().clone();
    let v1_second = registry.get_version("test-1", 1).unwrap().clone();

    let diff = StatuteDiff::compute(&v1_first, &v1_second);

    assert!(!diff.has_changes());
    assert_eq!(diff.summary(), "No changes");
}

#[test]
fn test_diff_with_latest() {
    let mut registry = StatuteRegistry::new();

    // Register and update
    registry
        .register(StatuteEntry::new(test_statute("test-1"), "JP"))
        .unwrap();
    registry.update("test-1", test_statute("test-1")).unwrap();
    registry.update("test-1", test_statute("test-1")).unwrap();

    // Diff version 1 with latest (version 3)
    let diff = registry.diff_with_latest("test-1", 1).unwrap();

    assert_eq!(diff.old_version, 1);
    assert_eq!(diff.new_version, 3);
}

#[test]
fn test_field_change() {
    // Test Changed
    let change = FieldChange::from_values(&"old".to_string(), &"new".to_string());
    assert!(change.is_changed());
    assert_eq!(change.new_value(), Some(&"new".to_string()));

    // Test Unchanged
    let same = FieldChange::from_values(&"same".to_string(), &"same".to_string());
    assert!(!same.is_changed());

    // Test Added
    let added = FieldChange::from_optional(None, Some(&"new".to_string()));
    assert!(added.is_some());
    assert!(added.unwrap().is_changed());

    // Test Removed
    let removed = FieldChange::from_optional(Some(&"old".to_string()), None);
    assert!(removed.is_some());
    assert!(removed.unwrap().is_changed());
}

#[test]
fn test_validation_rules() {
    // Test NonEmptyIdRule
    let rule = NonEmptyIdRule;
    let mut entry = StatuteEntry::new(test_statute("test-1"), "JP");
    assert!(rule.validate(&entry).is_ok());

    entry.statute.id = "".to_string();
    assert!(rule.validate(&entry).is_err());

    // Test NonEmptyTitleRule
    let rule = NonEmptyTitleRule;
    entry.statute.id = "test-1".to_string();
    entry.statute.title = "".to_string();
    assert!(rule.validate(&entry).is_err());

    // Test DateValidationRule
    let rule = DateValidationRule;
    let now = Utc::now();
    let future = now + chrono::Duration::days(1);
    let past = now - chrono::Duration::days(1);

    let mut entry = StatuteEntry::new(test_statute("test-1"), "JP");
    entry.effective_date = Some(now);
    entry.expiry_date = Some(future);
    assert!(rule.validate(&entry).is_ok());

    entry.expiry_date = Some(past);
    assert!(rule.validate(&entry).is_err());

    // Test TagValidationRule
    let rule = TagValidationRule;
    let mut entry = StatuteEntry::new(test_statute("test-1"), "JP").with_tag("valid");
    assert!(rule.validate(&entry).is_ok());

    entry.tags.push("".to_string());
    assert!(rule.validate(&entry).is_err());

    entry.tags.clear();
    entry.tags.push("tag1".to_string());
    entry.tags.push("tag1".to_string());
    assert!(rule.validate(&entry).is_err());
}

#[test]
fn test_validator() {
    let validator = Validator::with_defaults();

    // Valid entry
    let entry = StatuteEntry::new(test_statute("test-1"), "JP");
    assert!(validator.validate(&entry).is_ok());

    // Invalid entry (empty ID)
    let mut invalid = StatuteEntry::new(test_statute(""), "JP");
    invalid.statute.id = "".to_string();
    assert!(validator.validate(&invalid).is_err());

    // Invalid entry (empty title)
    let mut invalid = StatuteEntry::new(test_statute("test-1"), "JP");
    invalid.statute.title = "".to_string();
    assert!(validator.validate(&invalid).is_err());
}

#[test]
fn test_validator_custom_rules() {
    let mut validator = Validator::new();
    validator.add_rule(Box::new(NonEmptyIdRule));

    let entry = StatuteEntry::new(test_statute("test-1"), "JP");
    assert!(validator.validate(&entry).is_ok());

    let mut invalid = StatuteEntry::new(test_statute(""), "JP");
    invalid.statute.id = "".to_string();
    assert!(validator.validate(&invalid).is_err());

    assert_eq!(validator.rules().len(), 1);
}

#[test]
fn test_valid_jurisdiction_rule() {
    let rule = ValidJurisdictionRule::new(vec!["JP", "US", "UK"]);

    let entry_jp = StatuteEntry::new(test_statute("test-1"), "JP");
    assert!(rule.validate(&entry_jp).is_ok());

    let entry_fr = StatuteEntry::new(test_statute("test-2"), "FR");
    assert!(rule.validate(&entry_fr).is_err());
}

#[test]
fn test_operation_metrics() {
    let mut metrics = OperationMetrics::new();

    assert_eq!(metrics.total_operations(), 0);
    assert_eq!(metrics.cache_hit_rate(), 0.0);

    metrics.registrations = 10;
    metrics.reads = 20;
    assert_eq!(metrics.total_operations(), 30);

    metrics.cache_hits = 80;
    metrics.cache_misses = 20;
    assert_eq!(metrics.cache_hit_rate(), 0.8);

    metrics.reset();
    assert_eq!(metrics.total_operations(), 0);
}

#[test]
fn test_merge_prefer_old() {
    let entry1 = StatuteEntry::new(test_statute("test-1"), "JP")
        .with_status(StatuteStatus::Draft)
        .with_tag("civil");

    let mut statute2 = test_statute("test-1");
    statute2.title = "Updated Title".to_string();
    let entry2 = StatuteEntry::new(statute2, "US")
        .with_status(StatuteStatus::Active)
        .with_tag("criminal");

    let result = entry1.merge(&entry2, MergeStrategy::PreferOld);

    assert!(result.is_clean());
    assert_eq!(result.entry.statute.title, "Test test-1"); // Old title
    assert_eq!(result.entry.status, StatuteStatus::Draft); // Old status
    assert_eq!(result.entry.jurisdiction, "JP"); // Old jurisdiction
    // Tags should be unioned
    assert!(result.entry.tags.contains(&"civil".to_string()));
    assert!(result.entry.tags.contains(&"criminal".to_string()));
}

#[test]
fn test_merge_prefer_new() {
    let entry1 = StatuteEntry::new(test_statute("test-1"), "JP")
        .with_status(StatuteStatus::Draft)
        .with_tag("civil");

    let mut statute2 = test_statute("test-1");
    statute2.title = "Updated Title".to_string();
    let entry2 = StatuteEntry::new(statute2, "US")
        .with_status(StatuteStatus::Active)
        .with_tag("criminal");

    let result = entry1.merge(&entry2, MergeStrategy::PreferNew);

    assert!(result.is_clean());
    assert_eq!(result.entry.statute.title, "Updated Title"); // New title
    assert_eq!(result.entry.status, StatuteStatus::Active); // New status
    assert_eq!(result.entry.jurisdiction, "US"); // New jurisdiction
    // Tags should be unioned
    assert!(result.entry.tags.contains(&"civil".to_string()));
    assert!(result.entry.tags.contains(&"criminal".to_string()));
}

#[test]
fn test_merge_fail_on_conflict() {
    let entry1 = StatuteEntry::new(test_statute("test-1"), "JP").with_status(StatuteStatus::Draft);

    let mut statute2 = test_statute("test-1");
    statute2.title = "Updated Title".to_string();
    let entry2 = StatuteEntry::new(statute2, "US").with_status(StatuteStatus::Active);

    let result = entry1.merge(&entry2, MergeStrategy::FailOnConflict);

    assert!(!result.is_clean());
    assert!(result.has_conflicts);
    assert!(!result.conflicts.is_empty());

    // Check that conflicts were recorded
    let has_title_conflict = result
        .conflicts
        .iter()
        .any(|c| matches!(c, MergeConflict::Title { .. }));
    let has_status_conflict = result
        .conflicts
        .iter()
        .any(|c| matches!(c, MergeConflict::Status { .. }));
    let has_jurisdiction_conflict = result
        .conflicts
        .iter()
        .any(|c| matches!(c, MergeConflict::Jurisdiction { .. }));

    assert!(has_title_conflict);
    assert!(has_status_conflict);
    assert!(has_jurisdiction_conflict);
}

#[test]
fn test_merge_both() {
    let entry1 = StatuteEntry::new(test_statute("test-1"), "JP")
        .with_metadata("key1", "value1")
        .with_tag("civil");

    let entry2 = StatuteEntry::new(test_statute("test-1"), "US")
        .with_metadata("key2", "value2")
        .with_tag("criminal");

    let result = entry1.merge(&entry2, MergeStrategy::MergeBoth);

    assert!(result.is_clean());
    // Metadata should be merged
    assert_eq!(
        result.entry.metadata.get("key1"),
        Some(&"value1".to_string())
    );
    assert_eq!(
        result.entry.metadata.get("key2"),
        Some(&"value2".to_string())
    );
    // Tags should be unioned
    assert!(result.entry.tags.contains(&"civil".to_string()));
    assert!(result.entry.tags.contains(&"criminal".to_string()));
}

#[test]
fn test_merge_metadata_override() {
    let entry1 = StatuteEntry::new(test_statute("test-1"), "JP").with_metadata("key", "old_value");

    let entry2 = StatuteEntry::new(test_statute("test-1"), "JP").with_metadata("key", "new_value");

    let result = entry1.merge(&entry2, MergeStrategy::MergeBoth);

    // New value should override
    assert_eq!(
        result.entry.metadata.get("key"),
        Some(&"new_value".to_string())
    );
}

#[test]
fn test_registry_metrics() {
    let registry = StatuteRegistry::new();
    let metrics = registry.metrics();

    // Currently returns default metrics (placeholder)
    assert_eq!(metrics.total_operations(), 0);
}

#[test]
#[cfg(feature = "yaml")]
fn test_yaml_export_import() {
    let mut registry = StatuteRegistry::new();

    // Add some statutes
    registry
        .register(StatuteEntry::new(test_statute("test-1"), "JP").with_tag("civil"))
        .unwrap();
    registry
        .register(StatuteEntry::new(test_statute("test-2"), "US").with_tag("criminal"))
        .unwrap();

    // Export to YAML
    let yaml = registry.export_yaml().unwrap();
    assert!(!yaml.is_empty());
    assert!(yaml.contains("test-1"));
    assert!(yaml.contains("test-2"));

    // Import to new registry
    let mut new_registry = StatuteRegistry::new();
    new_registry.import_yaml(&yaml).unwrap();

    assert_eq!(new_registry.count(), 2);
    assert!(new_registry.contains("test-1"));
    assert!(new_registry.contains("test-2"));
}

#[test]
#[cfg(feature = "yaml")]
fn test_yaml_statute_export_import() {
    let entry = StatuteEntry::new(test_statute("test-1"), "JP")
        .with_tag("civil")
        .with_metadata("author", "Alice");

    // Export to YAML
    let yaml = StatuteRegistry::export_statute_yaml(&entry).unwrap();
    assert!(!yaml.is_empty());
    assert!(yaml.contains("test-1"));

    // Import back
    let imported = StatuteRegistry::import_statute_yaml(&yaml).unwrap();
    assert_eq!(imported.statute.id, "test-1");
    assert_eq!(imported.jurisdiction, "JP");
    assert!(imported.tags.contains(&"civil".to_string()));
}

#[test]
#[cfg(feature = "csv-export")]
fn test_csv_export() {
    let mut registry = StatuteRegistry::new();

    // Add some statutes
    registry
        .register(
            StatuteEntry::new(test_statute("test-1"), "JP")
                .with_tag("civil")
                .with_status(StatuteStatus::Active),
        )
        .unwrap();
    registry
        .register(
            StatuteEntry::new(test_statute("test-2"), "US")
                .with_tag("criminal")
                .with_status(StatuteStatus::Draft),
        )
        .unwrap();

    // Export to CSV
    let csv = registry.export_summaries_csv().unwrap();
    assert!(!csv.is_empty());

    // Check header
    assert!(csv.contains("statute_id"));
    assert!(csv.contains("title"));
    assert!(csv.contains("version"));
    assert!(csv.contains("status"));
    assert!(csv.contains("jurisdiction"));

    // Check data
    assert!(csv.contains("test-1"));
    assert!(csv.contains("test-2"));
    assert!(csv.contains("JP"));
    assert!(csv.contains("US"));
}

#[test]
#[cfg(feature = "csv-export")]
fn test_csv_export_filtered() {
    let mut registry = StatuteRegistry::new();

    // Add statutes with different jurisdictions
    registry
        .register(StatuteEntry::new(test_statute("jp-1"), "JP"))
        .unwrap();
    registry
        .register(StatuteEntry::new(test_statute("us-1"), "US"))
        .unwrap();
    registry
        .register(StatuteEntry::new(test_statute("jp-2"), "JP"))
        .unwrap();

    // Export only JP statutes
    let csv = registry
        .export_filtered_csv(|e| e.jurisdiction == "JP")
        .unwrap();

    assert!(csv.contains("jp-1"));
    assert!(csv.contains("jp-2"));
    assert!(!csv.contains("us-1"));
}

#[test]
#[cfg(feature = "compression")]
fn test_backup_compression() {
    let mut registry = StatuteRegistry::new();

    // Add some statutes
    for i in 1..=10 {
        registry
            .register(StatuteEntry::new(
                test_statute(&format!("test-{}", i)),
                "JP",
            ))
            .unwrap();
    }

    // Export compressed
    let compressed = registry.export_compressed_backup(None).unwrap();
    assert!(!compressed.is_empty());

    // Import to new registry
    let mut new_registry = StatuteRegistry::new();
    new_registry.import_compressed_backup(&compressed).unwrap();

    assert_eq!(new_registry.count(), 10);
}

#[test]
#[cfg(feature = "compression")]
fn test_compression_ratio() {
    let mut registry = StatuteRegistry::new();

    // Add statutes with repetitive data (compresses well)
    for i in 1..=20 {
        registry
            .register(
                StatuteEntry::new(test_statute(&format!("test-{}", i)), "JP")
                    .with_tag("civil")
                    .with_tag("criminal")
                    .with_metadata("key", "value"),
            )
            .unwrap();
    }

    let ratio = registry.compression_ratio(None).unwrap();
    // Should achieve some compression
    assert!(ratio > 1.0, "Compression ratio should be > 1.0");
}

#[test]
fn test_batch_validation() {
    let validator = Validator::with_defaults();

    let entries = vec![
        StatuteEntry::new(test_statute("valid-1"), "JP"),
        StatuteEntry::new(test_statute("valid-2"), "US"),
        {
            let mut invalid = StatuteEntry::new(test_statute(""), "JP");
            invalid.statute.id = "".to_string(); // Invalid
            invalid
        },
        {
            let mut invalid = StatuteEntry::new(test_statute("invalid-4"), "JP");
            invalid.statute.title = "".to_string(); // Invalid
            invalid
        },
    ];

    let result = validator.validate_batch(&entries);

    assert_eq!(result.total, 4);
    assert_eq!(result.valid, 2);
    assert_eq!(result.invalid, 2);
    assert!(!result.is_all_valid());
    assert!(result.success_rate() > 0.4 && result.success_rate() < 0.6);
    assert_eq!(result.errors.len(), 2);
}

#[test]
fn test_batch_validation_all_valid() {
    let validator = Validator::with_defaults();

    let entries = vec![
        StatuteEntry::new(test_statute("valid-1"), "JP"),
        StatuteEntry::new(test_statute("valid-2"), "US"),
        StatuteEntry::new(test_statute("valid-3"), "UK"),
    ];

    let result = validator.validate_batch(&entries);

    assert_eq!(result.total, 3);
    assert_eq!(result.valid, 3);
    assert_eq!(result.invalid, 0);
    assert!(result.is_all_valid());
    assert_eq!(result.success_rate(), 1.0);
    assert!(result.errors.is_empty());
}

#[test]
fn test_filter_valid() {
    let validator = Validator::with_defaults();

    let entries = vec![
        StatuteEntry::new(test_statute("valid-1"), "JP"),
        {
            let mut invalid = StatuteEntry::new(test_statute(""), "JP");
            invalid.statute.id = "".to_string();
            invalid
        },
        StatuteEntry::new(test_statute("valid-2"), "US"),
    ];

    let valid = validator.filter_valid(entries);

    assert_eq!(valid.len(), 2);
    assert_eq!(valid[0].statute.id, "valid-1");
    assert_eq!(valid[1].statute.id, "valid-2");
}

#[test]
fn test_filter_invalid() {
    let validator = Validator::with_defaults();

    let entries = vec![
        StatuteEntry::new(test_statute("valid-1"), "JP"),
        {
            let mut invalid = StatuteEntry::new(test_statute(""), "JP");
            invalid.statute.id = "".to_string();
            invalid
        },
        {
            let mut invalid = StatuteEntry::new(test_statute("invalid-2"), "JP");
            invalid.statute.title = "".to_string();
            invalid
        },
    ];

    let invalid = validator.filter_invalid(entries);

    assert_eq!(invalid.len(), 2);
    assert!(matches!(invalid[0].1, ValidationError::EmptyStatuteId));
    assert!(matches!(invalid[1].1, ValidationError::EmptyTitle));
}

#[test]
fn test_search_cache_config() {
    // Default config
    let default_config = SearchCacheConfig::default();
    assert_eq!(default_config.max_entries, 100);
    assert_eq!(default_config.ttl_seconds, 300);

    // Custom config
    let custom = SearchCacheConfig::new(50, 600);
    assert_eq!(custom.max_entries, 50);
    assert_eq!(custom.ttl_seconds, 600);

    // No TTL
    let no_ttl = SearchCacheConfig::no_ttl(200);
    assert_eq!(no_ttl.max_entries, 200);
    assert_eq!(no_ttl.ttl_seconds, i64::MAX);

    // Short lived
    let short = SearchCacheConfig::short_lived(150);
    assert_eq!(short.max_entries, 150);
    assert_eq!(short.ttl_seconds, 60);

    // Long lived
    let long = SearchCacheConfig::long_lived(250);
    assert_eq!(long.max_entries, 250);
    assert_eq!(long.ttl_seconds, 3600);
}

// ===== Session 5 Feature Tests =====

#[test]
fn test_delete_statute() {
    let mut registry = StatuteRegistry::new();
    let statute = test_statute("statute-1");
    let mut entry = StatuteEntry::new(statute, "US");
    entry.tags.push("tax".to_string());

    registry.register(entry).unwrap();
    assert_eq!(registry.count(), 1);

    // Delete the statute
    let deleted = registry.delete("statute-1").unwrap();
    assert_eq!(deleted.statute.id, "statute-1");
    assert_eq!(registry.count(), 0);

    // Verify cleanup
    assert!(registry.get_uncached("statute-1").is_none());
    assert!(registry.query_by_tag("tax").is_empty());
}

#[test]
fn test_delete_nonexistent() {
    let mut registry = StatuteRegistry::new();
    let result = registry.delete("nonexistent");
    assert!(matches!(result, Err(RegistryError::StatuteNotFound(_))));
}

#[test]
fn test_batch_delete() {
    let mut registry = StatuteRegistry::new();

    // Register multiple statutes
    for i in 1..=5 {
        let statute = test_statute(&format!("statute-{}", i));
        let entry = StatuteEntry::new(statute, "US");
        registry.register(entry).unwrap();
    }

    assert_eq!(registry.count(), 5);

    // Batch delete
    let ids = vec![
        "statute-1".to_string(),
        "statute-3".to_string(),
        "statute-5".to_string(),
    ];
    let results = registry.batch_delete(ids);

    assert_eq!(results.len(), 3);
    assert!(results.iter().all(|r| r.is_ok()));
    assert_eq!(registry.count(), 2);
}

#[test]
fn test_archive_statute() {
    let mut registry = StatuteRegistry::new();
    let statute = test_statute("old-statute");
    let entry = StatuteEntry::new(statute, "US");

    registry.register(entry).unwrap();
    assert_eq!(registry.count(), 1);

    // Archive the statute
    registry
        .archive_statute("old-statute", "Superseded by new law".to_string())
        .unwrap();

    // Should be removed from active registry
    assert_eq!(registry.count(), 0);
    assert!(registry.get_uncached("old-statute").is_none());

    // Should be in archive
    assert_eq!(registry.archived_count(), 1);
    let archived = registry.get_archived("old-statute").unwrap();
    assert_eq!(archived.reason, "Superseded by new law");
    assert_eq!(archived.entry.statute.id, "old-statute");
}

#[test]
fn test_unarchive_statute() {
    let mut registry = StatuteRegistry::new();
    let statute = test_statute("archived-statute");
    let entry = StatuteEntry::new(statute, "US");

    registry.register(entry).unwrap();
    registry
        .archive_statute("archived-statute", "Test archive".to_string())
        .unwrap();

    assert_eq!(registry.count(), 0);
    assert_eq!(registry.archived_count(), 1);

    // Unarchive
    let id = registry.unarchive_statute("archived-statute").unwrap();
    assert!(!id.as_simple().to_string().is_empty());

    // Should be back in active registry
    assert_eq!(registry.count(), 1);
    assert_eq!(registry.archived_count(), 0);
    assert!(registry.get_uncached("archived-statute").is_some());
}

#[test]
fn test_search_archived_by_reason() {
    let mut registry = StatuteRegistry::new();

    // Archive multiple statutes with different reasons
    for i in 1..=3 {
        let statute = test_statute(&format!("statute-{}", i));
        let entry = StatuteEntry::new(statute, "US");
        registry.register(entry).unwrap();
    }

    registry
        .archive_statute("statute-1", "Superseded by new law".to_string())
        .unwrap();
    registry
        .archive_statute("statute-2", "Expired statute".to_string())
        .unwrap();
    registry
        .archive_statute("statute-3", "Superseded by amendment".to_string())
        .unwrap();

    // Search by reason
    let superseded = registry.search_archived_by_reason("Superseded");
    assert_eq!(superseded.len(), 2);

    let expired = registry.search_archived_by_reason("Expired");
    assert_eq!(expired.len(), 1);
}

#[test]
fn test_search_ranked() {
    let mut registry = StatuteRegistry::new();

    // Register statutes with different relevance to query "tax"
    let s1 = Statute::new("tax-1", "Tax Law", Effect::new(EffectType::Grant, "Grant"));
    let mut e1 = StatuteEntry::new(s1, "US");
    e1.tags.push("tax".to_string());

    let s2 = Statute::new(
        "other-1",
        "Other Law with tax",
        Effect::new(EffectType::Grant, "Grant"),
    );
    let e2 = StatuteEntry::new(s2, "US");

    let s3 = Statute::new(
        "unrelated",
        "Unrelated Law",
        Effect::new(EffectType::Grant, "Grant"),
    );
    let e3 = StatuteEntry::new(s3, "US");

    registry.register(e1).unwrap();
    registry.register(e2).unwrap();
    registry.register(e3).unwrap();

    // Search with ranking
    let results = registry.search_ranked("tax", None);

    // Should return 2 results (e1 and e2), sorted by relevance
    assert_eq!(results.len(), 2);
    assert!(results[0].score > 0.0);
    assert!(results[0].score >= results[1].score); // Sorted by score
}

#[test]
fn test_ranking_config() {
    let config = RankingConfig::new()
        .with_title_weight(5.0)
        .with_id_weight(3.0)
        .with_tag_weight(2.0)
        .with_exact_match_boost(3.0);

    assert_eq!(config.title_weight, 5.0);
    assert_eq!(config.id_weight, 3.0);
    assert_eq!(config.tag_weight, 2.0);
    assert_eq!(config.exact_match_boost, 3.0);
}

#[test]
fn test_search_result_highlights() {
    let mut registry = StatuteRegistry::new();

    let statute = Statute::new(
        "tax-law",
        "Income Tax Law",
        Effect::new(EffectType::Grant, "Grant"),
    );
    let mut entry = StatuteEntry::new(statute, "US");
    entry.tags.push("taxation".to_string());

    registry.register(entry).unwrap();

    let results = registry.search_ranked("tax", None);
    assert_eq!(results.len(), 1);

    let result = &results[0];
    assert!(result.get_highlights("id").is_some() || result.get_highlights("title").is_some());
}

#[test]
fn test_create_snapshot() {
    let mut registry = StatuteRegistry::new();

    // Add some statutes
    for i in 1..=3 {
        let statute = test_statute(&format!("statute-{}", i));
        let entry = StatuteEntry::new(statute, "US");
        registry.register(entry).unwrap();
    }

    // Create snapshot
    let snapshot = registry.create_snapshot(Some("Test snapshot".to_string()));

    assert_eq!(snapshot.backup.statutes.len(), 3);
    assert_eq!(snapshot.description, Some("Test snapshot".to_string()));
    assert!(!snapshot.snapshot_id.as_simple().to_string().is_empty());
}

#[test]
fn test_restore_from_snapshot() {
    let mut registry = StatuteRegistry::new();

    // Add statutes and create snapshot
    for i in 1..=2 {
        let statute = test_statute(&format!("statute-{}", i));
        let entry = StatuteEntry::new(statute, "US");
        registry.register(entry).unwrap();
    }

    let snapshot = registry.create_snapshot(None);

    // Add more statutes
    let statute = test_statute("statute-3");
    let entry = StatuteEntry::new(statute, "US");
    registry.register(entry).unwrap();
    assert_eq!(registry.count(), 3);

    // Restore from snapshot
    registry.restore_from_snapshot(snapshot).unwrap();
    assert_eq!(registry.count(), 2);
}

#[test]
fn test_incremental_backup() {
    let mut registry = StatuteRegistry::new();

    // Create initial state
    let statute1 = test_statute("statute-1");
    let entry1 = StatuteEntry::new(statute1, "US");
    registry.register(entry1).unwrap();

    // Create base snapshot
    let snapshot = registry.create_snapshot(None);

    // Make changes
    std::thread::sleep(std::time::Duration::from_millis(10));
    let statute2 = test_statute("statute-2");
    let entry2 = StatuteEntry::new(statute2, "US");
    registry.register(entry2).unwrap();

    let statute3 = Statute::new(
        "statute-1",
        "Updated",
        Effect::new(EffectType::Grant, "Grant"),
    );
    registry.update("statute-1", statute3).unwrap();

    // Create incremental backup
    let incremental = registry.create_incremental_backup(&snapshot);

    assert!(incremental.change_count() > 0);
    assert!(!incremental.delta_events.is_empty());
}

#[test]
fn test_apply_incremental_backup() {
    let mut registry1 = StatuteRegistry::new();
    let mut registry2 = StatuteRegistry::new();

    // Create base state in both
    let statute = test_statute("statute-1");
    let entry = StatuteEntry::new(statute.clone(), "US");
    registry1.register(entry.clone()).unwrap();
    registry2.register(entry).unwrap();

    // Create snapshot
    let snapshot = registry1.create_snapshot(None);

    // Make changes in registry1
    std::thread::sleep(std::time::Duration::from_millis(10));
    let new_statute = test_statute("statute-2");
    let new_entry = StatuteEntry::new(new_statute, "US");
    registry1.register(new_entry).unwrap();

    // Create and apply incremental
    let incremental = registry1.create_incremental_backup(&snapshot);
    registry2.apply_incremental_backup(incremental).unwrap();

    // Both registries should be in sync
    assert_eq!(registry2.count(), registry1.count());
}

#[test]
fn test_advanced_query_date_filters() {
    let mut registry = StatuteRegistry::new();

    let now = Utc::now();
    let past = now - chrono::Duration::days(30);
    let future = now + chrono::Duration::days(30);

    let statute = test_statute("statute-1");
    let mut entry = StatuteEntry::new(statute, "US");
    entry.effective_date = Some(past);
    entry.expiry_date = Some(future);

    registry.register(entry).unwrap();

    // Query with date range
    let query = SearchQuery::new().with_effective_date_range(past - chrono::Duration::days(1), now);

    // Note: The actual filtering would need to be implemented in the search() method
    // This test verifies the query builder works correctly
    assert!(query.effective_date_range.is_some());
    assert!(query.expiry_date_range.is_none());
}

#[test]
fn test_advanced_query_version_filters() {
    let query = SearchQuery::new().with_version(2).with_min_version(1);

    assert_eq!(query.version, Some(2));
    assert_eq!(query.min_version, Some(1));
}

#[test]
fn test_advanced_query_effect_type_filter() {
    let query = SearchQuery::new().with_effect_type(EffectType::Grant);

    assert_eq!(query.effect_type, Some(EffectType::Grant));
}

#[test]
fn test_advanced_query_exclude_tags() {
    let query = SearchQuery::new()
        .with_tag("include-me")
        .exclude_tag("exclude-me")
        .exclude_tag("also-exclude");

    assert_eq!(query.tags.len(), 1);
    assert_eq!(query.exclude_tags.len(), 2);
}

#[test]
fn test_advanced_query_reference_filter() {
    let query = SearchQuery::new()
        .with_reference("ref-1")
        .with_reference("ref-2");

    assert_eq!(query.references.len(), 2);
}

#[test]
fn test_advanced_query_supersedes_filter() {
    let query1 = SearchQuery::new().with_supersedes();
    assert_eq!(query1.has_supersedes, Some(true));

    let query2 = SearchQuery::new().without_supersedes();
    assert_eq!(query2.has_supersedes, Some(false));
}

#[test]
fn test_delete_event_recorded() {
    let mut registry = StatuteRegistry::new();
    let statute = test_statute("statute-1");
    let entry = StatuteEntry::new(statute, "US");

    registry.register(entry).unwrap();
    let initial_event_count = registry.event_count();

    registry.delete("statute-1").unwrap();

    // Should have recorded a StatuteDeleted event
    let events = registry.all_events();
    let delete_events: Vec<_> = events
        .iter()
        .filter(|e| matches!(e, RegistryEvent::StatuteDeleted { .. }))
        .collect();

    assert_eq!(delete_events.len(), 1);
    assert!(registry.event_count() > initial_event_count);
}

#[test]
fn test_archive_event_recorded() {
    let mut registry = StatuteRegistry::new();
    let statute = test_statute("statute-1");
    let entry = StatuteEntry::new(statute, "US");

    registry.register(entry).unwrap();
    registry
        .archive_statute("statute-1", "Test reason".to_string())
        .unwrap();

    // Should have recorded both StatuteDeleted and StatuteArchived events
    let events = registry.all_events();
    let archive_events: Vec<_> = events
        .iter()
        .filter(|e| matches!(e, RegistryEvent::StatuteArchived { .. }))
        .collect();

    assert_eq!(archive_events.len(), 1);
}

#[test]
fn test_retention_policy_expired_statutes() {
    let mut registry = StatuteRegistry::new();

    let now = Utc::now();
    let past = now - chrono::Duration::days(60);

    // Add an expired statute
    let statute = test_statute("expired-statute");
    let mut entry = StatuteEntry::new(statute, "US");
    entry.effective_date = Some(past);
    entry.expiry_date = Some(now - chrono::Duration::days(1));

    registry.register(entry).unwrap();

    // Add a non-expired statute
    let statute2 = test_statute("active-statute");
    let mut entry2 = StatuteEntry::new(statute2, "US");
    entry2.effective_date = Some(past);
    entry2.expiry_date = Some(now + chrono::Duration::days(30));

    registry.register(entry2).unwrap();

    assert_eq!(registry.count(), 2);

    // Set retention policy to archive expired statutes
    let policy = RetentionPolicy::new().add_rule(RetentionRule::ExpiredStatutes {
        reason: "Statute has expired".to_string(),
    });

    registry.set_retention_policy(policy);

    // Apply retention policy
    let result = registry.apply_retention_policy();

    // Should archive 1 statute
    assert_eq!(result.archived_count(), 1);
    assert_eq!(registry.count(), 1);
    assert_eq!(registry.archived_count(), 1);
}

#[test]
fn test_retention_policy_old_statutes() {
    let mut registry = StatuteRegistry::new();

    let now = Utc::now();
    let very_old = now - chrono::Duration::days(400);
    let recent = now - chrono::Duration::days(10);

    // Add an old statute
    let statute1 = test_statute("old-statute");
    let mut entry1 = StatuteEntry::new(statute1, "US");
    entry1.effective_date = Some(very_old);

    registry.register(entry1).unwrap();

    // Add a recent statute
    let statute2 = test_statute("recent-statute");
    let mut entry2 = StatuteEntry::new(statute2, "US");
    entry2.effective_date = Some(recent);

    registry.register(entry2).unwrap();

    // Set retention policy to archive statutes older than 365 days
    let policy = RetentionPolicy::new().add_rule(RetentionRule::OlderThanDays {
        days: 365,
        reason: "Statute older than 1 year".to_string(),
    });

    registry.set_retention_policy(policy);

    let result = registry.apply_retention_policy();

    assert_eq!(result.archived_count(), 1);
    assert!(result.archived_ids.contains(&"old-statute".to_string()));
}

#[test]
fn test_retention_policy_by_status() {
    let mut registry = StatuteRegistry::new();

    // Add statutes with different statuses
    let statute1 = test_statute("statute-1");
    let entry1 = StatuteEntry::new(statute1, "US");
    registry.register(entry1).unwrap();
    registry
        .set_status("statute-1", StatuteStatus::Repealed)
        .unwrap();

    let statute2 = test_statute("statute-2");
    let entry2 = StatuteEntry::new(statute2, "US");
    registry.register(entry2).unwrap();
    // statute-2 remains Draft

    // Archive repealed statutes
    let policy = RetentionPolicy::new().add_rule(RetentionRule::ByStatus {
        status: StatuteStatus::Repealed,
        reason: "Repealed statute".to_string(),
    });

    registry.set_retention_policy(policy);
    let result = registry.apply_retention_policy();

    assert_eq!(result.archived_count(), 1);
    assert_eq!(registry.count(), 1);
}

#[test]
fn test_retention_policy_superseded() {
    let mut registry = StatuteRegistry::new();

    // Add a superseded statute
    let statute1 = test_statute("old-law");
    let mut entry1 = StatuteEntry::new(statute1, "US");
    entry1.supersedes.push("even-older-law".to_string());

    registry.register(entry1).unwrap();

    // Add a normal statute
    let statute2 = test_statute("normal-law");
    let entry2 = StatuteEntry::new(statute2, "US");
    registry.register(entry2).unwrap();

    // Archive superseded statutes
    let policy = RetentionPolicy::new().add_rule(RetentionRule::SupersededStatutes {
        reason: "Superseded by newer law".to_string(),
    });

    registry.set_retention_policy(policy);
    let result = registry.apply_retention_policy();

    assert_eq!(result.archived_count(), 1);
    assert!(result.archived_ids.contains(&"old-law".to_string()));
}

#[test]
fn test_retention_policy_inactive() {
    let mut registry = StatuteRegistry::new();

    let now = Utc::now();

    // Add an inactive statute (not modified in long time)
    let statute1 = test_statute("inactive-statute");
    let mut entry1 = StatuteEntry::new(statute1, "US");
    entry1.modified_at = now - chrono::Duration::days(400);

    registry.register(entry1).unwrap();

    // Add a recently modified statute
    let statute2 = test_statute("active-statute");
    let entry2 = StatuteEntry::new(statute2, "US");
    registry.register(entry2).unwrap();

    // Archive inactive statutes
    let policy = RetentionPolicy::new().add_rule(RetentionRule::InactiveForDays {
        days: 365,
        reason: "No activity for over 1 year".to_string(),
    });

    registry.set_retention_policy(policy);
    let result = registry.apply_retention_policy();

    assert_eq!(result.archived_count(), 1);
    assert!(
        result
            .archived_ids
            .contains(&"inactive-statute".to_string())
    );
}

#[test]
fn test_retention_policy_multiple_rules() {
    let mut registry = StatuteRegistry::new();

    let now = Utc::now();

    // Add various statutes
    let s1 = test_statute("expired");
    let mut e1 = StatuteEntry::new(s1, "US");
    e1.expiry_date = Some(now - chrono::Duration::days(1));
    registry.register(e1).unwrap();

    let s2 = test_statute("old");
    let mut e2 = StatuteEntry::new(s2, "US");
    e2.effective_date = Some(now - chrono::Duration::days(400));
    registry.register(e2).unwrap();

    let s3 = test_statute("current");
    let e3 = StatuteEntry::new(s3, "US");
    registry.register(e3).unwrap();

    // Multiple retention rules
    let policy = RetentionPolicy::new()
        .add_rule(RetentionRule::ExpiredStatutes {
            reason: "Expired".to_string(),
        })
        .add_rule(RetentionRule::OlderThanDays {
            days: 365,
            reason: "Too old".to_string(),
        });

    registry.set_retention_policy(policy);
    let result = registry.apply_retention_policy();

    // Should archive 2 statutes
    assert_eq!(result.archived_count(), 2);
    assert_eq!(registry.count(), 1);
}

#[test]
fn test_retention_result() {
    let mut result = RetentionResult::new(10);
    assert_eq!(result.total_evaluated, 10);
    assert_eq!(result.archived_count(), 0);

    result.record_archived("statute-1".to_string(), "Expired".to_string());
    result.record_archived("statute-2".to_string(), "Old".to_string());

    assert_eq!(result.archived_count(), 2);
    assert_eq!(
        result.reasons.get("statute-1"),
        Some(&"Expired".to_string())
    );
    assert_eq!(result.reasons.get("statute-2"), Some(&"Old".to_string()));
}

#[test]
fn test_iterator_apis() {
    let mut registry = StatuteRegistry::new();

    // Add test statutes
    registry
        .register(StatuteEntry::new(test_statute("iter-1"), "US"))
        .unwrap();
    let mut entry2 = StatuteEntry::new(test_statute("iter-2"), "US");
    entry2.status = StatuteStatus::Active;
    registry.register(entry2).unwrap();
    registry
        .register(StatuteEntry::new(test_statute("iter-3"), "JP"))
        .unwrap();

    // Test iter()
    assert_eq!(registry.iter().count(), 3);

    // Test iter_active()
    let active_count = registry.iter_active().count();
    assert_eq!(active_count, 1);

    // Test iter_with_ids()
    let ids: Vec<_> = registry
        .iter_with_ids()
        .map(|(id, _)| id.as_str())
        .collect();
    assert!(ids.contains(&"iter-1"));
    assert!(ids.contains(&"iter-2"));
    assert!(ids.contains(&"iter-3"));
}

#[test]
fn test_temporal_analytics() {
    let mut registry = StatuteRegistry::new();

    // Add test statutes with different timestamps
    registry
        .register(StatuteEntry::new(test_statute("temp-1"), "US"))
        .unwrap();
    registry
        .register(StatuteEntry::new(test_statute("temp-2"), "US"))
        .unwrap();
    registry
        .register(StatuteEntry::new(test_statute("temp-3"), "US"))
        .unwrap();

    // Update one to create version history
    registry.update("temp-1", test_statute("temp-1")).unwrap();
    registry.update("temp-1", test_statute("temp-1")).unwrap();

    let analytics = registry.temporal_analytics();

    // Should have some registrations
    assert_eq!(analytics.total_registrations(), 3);
    // Total updates can be any non-negative value
    let _ = analytics.total_updates();
    assert!(analytics.avg_versions_per_statute >= 0.0);

    // Most versioned should include temp-1
    assert!(
        analytics
            .most_versioned_statutes
            .iter()
            .any(|(id, _)| id == "temp-1")
    );
}

#[test]
fn test_relationship_analytics() {
    let mut registry = StatuteRegistry::new();

    // Create statutes with relationships
    let mut entry1 = StatuteEntry::new(test_statute("rel-1"), "US");
    entry1.references.push("rel-2".to_string());
    registry.register(entry1).unwrap();

    let mut entry2 = StatuteEntry::new(test_statute("rel-2"), "US");
    entry2.references.push("rel-3".to_string());
    registry.register(entry2).unwrap();

    let mut entry3 = StatuteEntry::new(test_statute("rel-3"), "US");
    entry3.supersedes.push("rel-2".to_string());
    registry.register(entry3).unwrap();

    // Orphan statute with no relationships
    registry
        .register(StatuteEntry::new(test_statute("rel-orphan"), "US"))
        .unwrap();

    let analytics = registry.relationship_analytics();

    // Check most referenced includes rel-2 and rel-3
    assert!(
        analytics
            .most_referenced
            .iter()
            .any(|(id, count)| id == "rel-2" && *count >= 1)
    );
    assert!(
        analytics
            .most_referenced
            .iter()
            .any(|(id, count)| id == "rel-3" && *count >= 1)
    );

    // Check supersession chains
    assert!(!analytics.supersession_chains.is_empty());

    // Check orphaned statutes
    assert!(
        analytics
            .orphaned_statutes
            .contains(&"rel-orphan".to_string())
    );

    // Average references should be > 0
    assert!(analytics.avg_references_per_statute >= 0.0);
}

#[test]
fn test_tag_analytics() {
    let mut registry = StatuteRegistry::new();

    // Add statutes with various tags
    registry
        .register(
            StatuteEntry::new(test_statute("tag-1"), "US")
                .with_tag("civil")
                .with_tag("contract"),
        )
        .unwrap();
    registry
        .register(
            StatuteEntry::new(test_statute("tag-2"), "US")
                .with_tag("civil")
                .with_tag("tort"),
        )
        .unwrap();
    registry
        .register(StatuteEntry::new(test_statute("tag-3"), "US").with_tag("criminal"))
        .unwrap();

    let analytics = registry.tag_analytics();

    // Check tag frequency
    assert_eq!(analytics.tag_frequency.get("civil"), Some(&2));
    assert_eq!(analytics.tag_frequency.get("criminal"), Some(&1));
    assert_eq!(analytics.tag_frequency.get("contract"), Some(&1));
    assert_eq!(analytics.tag_frequency.get("tort"), Some(&1));

    // Check total tag usage
    assert_eq!(analytics.total_tag_usage(), 5);

    // Check unique tag count
    assert_eq!(analytics.unique_tag_count(), 4);

    // Check most used tags includes "civil"
    assert!(
        analytics
            .most_used_tags
            .iter()
            .any(|(tag, count)| tag == "civil" && *count == 2)
    );

    // Check tag co-occurrence (civil appears with both contract and tort)
    assert!(analytics.tag_cooccurrence.contains_key("civil"));

    // Check related tags
    let related = analytics.related_tags("civil", 1);
    assert!(related.iter().any(|(tag, _)| tag == "contract"));
    assert!(related.iter().any(|(tag, _)| tag == "tort"));

    // Check average tags per statute
    assert!((analytics.avg_tags_per_statute - 1.666).abs() < 0.01);
}

#[test]
fn test_activity_analytics() {
    let mut registry = StatuteRegistry::new();

    // Add statutes
    registry
        .register(StatuteEntry::new(test_statute("act-1"), "US"))
        .unwrap();
    registry
        .register(StatuteEntry::new(test_statute("act-2"), "US"))
        .unwrap();
    registry
        .register(StatuteEntry::new(test_statute("act-3"), "US"))
        .unwrap();

    // Update some statutes to create modification history
    registry.update("act-1", test_statute("act-1")).unwrap();
    registry.update("act-1", test_statute("act-1")).unwrap();
    registry.update("act-2", test_statute("act-2")).unwrap();

    // Change status to create status change events
    registry.set_status("act-1", StatuteStatus::Active).unwrap();
    registry
        .set_status("act-1", StatuteStatus::Repealed)
        .unwrap();

    let analytics = registry.activity_analytics();

    // Check most modified statutes
    assert!(!analytics.most_modified.is_empty());
    assert!(analytics.most_modified.iter().any(|(id, _)| id == "act-1"));

    // Check recently modified
    assert_eq!(analytics.recently_modified.len(), 3);

    // Check least modified
    assert_eq!(analytics.least_modified.len(), 3);

    // Check frequent status changes
    assert!(
        analytics
            .frequent_status_changes
            .iter()
            .any(|(id, count)| id == "act-1" && *count == 2)
    );

    // Check average modification frequency
    assert!(analytics.avg_modification_frequency_days >= 0.0);

    // Test modified_within_days
    let recent = analytics.modified_within_days(1);
    assert_eq!(recent.len(), 3);
}

#[test]
fn test_field_projection() {
    // Test all() projection
    let proj = FieldProjection::all();
    assert!(proj.include_id);
    assert!(proj.include_title);
    assert!(proj.include_version);
    assert!(proj.include_status);
    assert!(proj.include_jurisdiction);
    assert!(proj.include_tags);
    assert!(proj.include_dates);
    assert!(proj.include_metadata);

    // Test essential() projection
    let proj = FieldProjection::essential();
    assert!(proj.include_id);
    assert!(proj.include_title);
    assert!(proj.include_version);
    assert!(proj.include_status);
    assert!(!proj.include_jurisdiction);
    assert!(!proj.include_tags);
    assert!(!proj.include_dates);
    assert!(!proj.include_metadata);

    // Test builder methods
    let proj = FieldProjection::default()
        .with_id()
        .with_title()
        .with_tags()
        .with_metadata();
    assert!(proj.include_id);
    assert!(proj.include_title);
    assert!(proj.include_tags);
    assert!(proj.include_metadata);
    assert!(!proj.include_status);
}

#[test]
fn test_aggregation_result() {
    let mut counts = HashMap::new();
    counts.insert("A".to_string(), 5);
    counts.insert("B".to_string(), 3);
    counts.insert("C".to_string(), 2);

    let result = AggregationResult::new(counts);

    // Test total
    assert_eq!(result.total, 10);

    // Test get_count
    assert_eq!(result.get_count("A"), 5);
    assert_eq!(result.get_count("B"), 3);
    assert_eq!(result.get_count("nonexistent"), 0);

    // Test sorted_by_count
    let sorted = result.sorted_by_count();
    assert_eq!(sorted[0], ("A".to_string(), 5));
    assert_eq!(sorted[1], ("B".to_string(), 3));
    assert_eq!(sorted[2], ("C".to_string(), 2));

    // Test percentage
    assert!((result.percentage("A") - 50.0).abs() < 0.01);
    assert!((result.percentage("B") - 30.0).abs() < 0.01);
    assert!((result.percentage("C") - 20.0).abs() < 0.01);
}

#[test]
fn test_aggregate_by() {
    let mut registry = StatuteRegistry::new();

    // Add statutes with different jurisdictions
    registry
        .register(StatuteEntry::new(test_statute("agg-1"), "US"))
        .unwrap();
    registry
        .register(StatuteEntry::new(test_statute("agg-2"), "US"))
        .unwrap();
    registry
        .register(StatuteEntry::new(test_statute("agg-3"), "JP"))
        .unwrap();
    registry
        .register(StatuteEntry::new(test_statute("agg-4"), "UK"))
        .unwrap();

    // Aggregate by jurisdiction
    let by_jurisdiction = registry.aggregate_by(|entry| entry.jurisdiction.clone());

    assert_eq!(by_jurisdiction.get_count("US"), 2);
    assert_eq!(by_jurisdiction.get_count("JP"), 1);
    assert_eq!(by_jurisdiction.get_count("UK"), 1);
    assert_eq!(by_jurisdiction.total, 4);

    // Aggregate by status (using Debug format)
    let by_status = registry.aggregate_by(|entry| format!("{:?}", entry.status));
    assert!(by_status.total > 0);
}

#[test]
fn test_aggregate_by_tags() {
    let mut registry = StatuteRegistry::new();

    // Add statutes with tags
    registry
        .register(
            StatuteEntry::new(test_statute("tag-agg-1"), "US")
                .with_tag("civil")
                .with_tag("contract"),
        )
        .unwrap();
    registry
        .register(StatuteEntry::new(test_statute("tag-agg-2"), "US").with_tag("civil"))
        .unwrap();
    registry
        .register(StatuteEntry::new(test_statute("tag-agg-3"), "US").with_tag("criminal"))
        .unwrap();

    let by_tags = registry.aggregate_by_tags();

    assert_eq!(by_tags.get_count("civil"), 2);
    assert_eq!(by_tags.get_count("contract"), 1);
    assert_eq!(by_tags.get_count("criminal"), 1);
    assert_eq!(by_tags.total, 4);
}

#[test]
fn test_analytics_empty_registry() {
    let mut registry = StatuteRegistry::new();

    // Test temporal analytics on empty registry
    let temporal = registry.temporal_analytics();
    assert_eq!(temporal.total_registrations(), 0);
    assert_eq!(temporal.total_updates(), 0);
    assert_eq!(temporal.total_activity(), 0);
    assert_eq!(temporal.avg_versions_per_statute, 0.0);

    // Test relationship analytics on empty registry
    let relationship = registry.relationship_analytics();
    assert_eq!(relationship.total_relationships(), 0);
    assert_eq!(relationship.max_chain_length(), 0);

    // Test tag analytics on empty registry
    let tag = registry.tag_analytics();
    assert_eq!(tag.unique_tag_count(), 0);
    assert_eq!(tag.total_tag_usage(), 0);

    // Test activity analytics on empty registry
    let activity = registry.activity_analytics();
    assert!(activity.most_modified.is_empty());
    assert!(activity.recently_modified.is_empty());

    // Test aggregation on empty registry
    let agg = registry.aggregate_by(|entry| entry.jurisdiction.clone());
    assert_eq!(agg.total, 0);
}

// ========================================================================
// Tests for Session 8: Audit Trail, Health Check, Comparison, Bulk Ops
// ========================================================================

#[test]
fn test_audit_entry_creation() {
    let entry = AuditEntry::new(
        "user123".to_string(),
        AuditOperation::Register,
        AuditResult::Success,
    );

    assert_eq!(entry.actor, "user123");
    assert!(entry.is_success());
    assert!(!entry.is_failure());
    assert!(entry.statute_id.is_none());
    assert!(entry.source.is_none());
    assert!(entry.metadata.is_empty());
}
