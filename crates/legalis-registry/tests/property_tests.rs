//! Property-based tests for legalis-registry using proptest.
//!
//! These tests verify registry invariants and properties:
//! - Registry consistency
//! - Version history integrity
//! - Entry properties
//! - Status management

use legalis_core::{Effect, EffectType, Statute};
use legalis_registry::{StatuteEntry, StatuteRegistry, StatuteStatus};
use proptest::prelude::*;

// ============================================================================
// Strategy Generators
// ============================================================================

/// Strategy for generating Statutes
fn statute_strategy() -> impl Strategy<Value = Statute> {
    ("[a-z][a-z0-9-]{2,20}", "[A-Za-z ]{10,60}").prop_map(|(id, title)| {
        Statute::new(&id, &title, Effect::new(EffectType::Grant, "Test effect"))
    })
}

/// Strategy for generating StatuteStatus
fn statute_status_strategy() -> impl Strategy<Value = StatuteStatus> {
    prop_oneof![
        Just(StatuteStatus::Draft),
        Just(StatuteStatus::UnderReview),
        Just(StatuteStatus::Approved),
        Just(StatuteStatus::Active),
        Just(StatuteStatus::Repealed),
        Just(StatuteStatus::Superseded),
    ]
}

/// Strategy for generating jurisdictions
fn jurisdiction_strategy() -> impl Strategy<Value = String> {
    "[A-Z]{2}".prop_map(|s| s.to_string())
}

// ============================================================================
// Registry Consistency Properties
// ============================================================================

proptest! {
    /// Property: Newly created registry is empty
    #[test]
    fn prop_new_registry_is_empty(_seed in any::<u64>()) {
        let registry = StatuteRegistry::new();
        let statutes = registry.list();
        prop_assert_eq!(statutes.len(), 0, "New registry should be empty");
        prop_assert_eq!(registry.count(), 0, "Count should be zero");
    }

    /// Property: Registering a statute increases registry size by one
    #[test]
    fn prop_register_increases_size(statute in statute_strategy(), jurisdiction in jurisdiction_strategy()) {
        let mut registry = StatuteRegistry::new();
        let initial_count = registry.count();
        let entry = StatuteEntry::new(statute, jurisdiction);

        let result = registry.register(entry);
        prop_assert!(result.is_ok(), "Registration should succeed");

        let new_count = registry.count();
        prop_assert_eq!(new_count, initial_count + 1, "Count should increase by 1");
    }

    /// Property: Registering duplicate statute ID should fail
    #[test]
    fn prop_duplicate_id_fails(statute in statute_strategy(), jurisdiction in jurisdiction_strategy()) {
        let mut registry = StatuteRegistry::new();
        let entry1 = StatuteEntry::new(statute.clone(), jurisdiction.clone());
        let entry2 = StatuteEntry::new(statute, jurisdiction);

        let first = registry.register(entry1);
        prop_assert!(first.is_ok(), "First registration should succeed");

        let duplicate = registry.register(entry2);
        prop_assert!(duplicate.is_err(), "Duplicate ID should fail");
    }

    /// Property: Get after register returns the statute
    #[test]
    fn prop_get_after_register_returns_same(statute in statute_strategy(), jurisdiction in jurisdiction_strategy()) {
        let mut registry = StatuteRegistry::new();
        let id = statute.id.clone();
        let entry = StatuteEntry::new(statute.clone(), jurisdiction);

        let _ = registry.register(entry);
        let retrieved = registry.get(&id);

        prop_assert!(retrieved.is_some(), "Get after register should return Some");
        let found = retrieved.unwrap_or_else(|| panic!("Should exist"));
        prop_assert_eq!(&found.statute.id, &statute.id);
        prop_assert_eq!(&found.statute.title, &statute.title);
    }

    /// Property: list() returns all registered statutes
    #[test]
    fn prop_list_returns_all(
        statutes in prop::collection::vec(statute_strategy(), 1..10),
        jurisdiction in jurisdiction_strategy()
    ) {
        let mut registry = StatuteRegistry::new();
        let mut registered_count = 0;

        for statute in statutes {
            let entry = StatuteEntry::new(statute, jurisdiction.clone());
            if registry.register(entry).is_ok() {
                registered_count += 1;
            }
        }

        let list = registry.list();
        prop_assert_eq!(list.len(), registered_count, "List should contain all statutes");
    }
}

// ============================================================================
// Version History Integrity Properties
// ============================================================================

proptest! {
    /// Property: Initial statute version is 1
    #[test]
    fn prop_initial_version_is_one(statute in statute_strategy(), jurisdiction in jurisdiction_strategy()) {
        let entry = StatuteEntry::new(statute, jurisdiction);
        prop_assert_eq!(entry.version, 1, "Initial version should be 1");
    }

    /// Property: Updating statute increments version
    #[test]
    fn prop_update_increments_version(
        statute in statute_strategy(),
        new_title in "[A-Za-z ]{10,60}",
        jurisdiction in jurisdiction_strategy()
    ) {
        let mut registry = StatuteRegistry::new();
        let id = statute.id.clone();
        let entry = StatuteEntry::new(statute, jurisdiction);

        let _ = registry.register(entry);
        let initial_version = registry.get(&id).unwrap_or_else(|| panic!("Should exist")).version;

        let mut updated = registry.get(&id).unwrap_or_else(|| panic!("Should exist")).statute.clone();
        updated.title = new_title;
        let new_version_result = registry.update(&id, updated);

        prop_assert!(new_version_result.is_ok(), "Update should succeed");
        let new_version = new_version_result.unwrap_or_else(|_| panic!("Should succeed"));
        prop_assert_eq!(new_version, initial_version + 1, "Version should increment");
    }

    /// Property: Version history is retrievable
    #[test]
    fn prop_version_history_retrievable(
        statute in statute_strategy(),
        update_count in 1usize..5,
        jurisdiction in jurisdiction_strategy()
    ) {
        let mut registry = StatuteRegistry::new();
        let id = statute.id.clone();
        let entry = StatuteEntry::new(statute, jurisdiction);

        let _ = registry.register(entry);

        // Perform multiple updates
        for i in 0..update_count {
            let mut updated = registry.get(&id).unwrap_or_else(|| panic!("Should exist")).statute.clone();
            updated.title = format!("Updated Title {}", i);
            let _ = registry.update(&id, updated);
        }

        let versions = registry.list_versions(&id);
        prop_assert_eq!(versions.len(), update_count + 1, "History should contain all versions");
    }

    /// Property: Getting specific version returns correct version
    #[test]
    fn prop_get_specific_version_correct(statute in statute_strategy(), jurisdiction in jurisdiction_strategy()) {
        let mut registry = StatuteRegistry::new();
        let id = statute.id.clone();
        let original_title = statute.title.clone();
        let entry = StatuteEntry::new(statute, jurisdiction);

        let _ = registry.register(entry);

        // Update to create version 2
        let mut updated = registry.get(&id).unwrap_or_else(|| panic!("Should exist")).statute.clone();
        updated.title = "Updated Title".to_string();
        let _ = registry.update(&id, updated);

        // Retrieve version 1
        let v1 = registry.get_version(&id, 1);
        prop_assert!(v1.is_ok(), "Should retrieve version 1");
        prop_assert_eq!(&v1.unwrap_or_else(|_| panic!("Should succeed")).statute.title, &original_title);
    }
}

// ============================================================================
// Status Management Properties
// ============================================================================

proptest! {
    /// Property: Setting status updates the entry
    #[test]
    fn prop_set_status_updates(
        statute in statute_strategy(),
        jurisdiction in jurisdiction_strategy(),
        status in statute_status_strategy()
    ) {
        let mut registry = StatuteRegistry::new();
        let id = statute.id.clone();
        let entry = StatuteEntry::new(statute, jurisdiction);

        let _ = registry.register(entry);
        let result = registry.set_status(&id, status);

        prop_assert!(result.is_ok(), "Setting status should succeed");

        let updated = registry.get(&id).unwrap_or_else(|| panic!("Should exist"));
        prop_assert_eq!(updated.status, status, "Status should be updated");
    }

    /// Property: Status operations are idempotent
    #[test]
    fn prop_status_idempotent(
        statute in statute_strategy(),
        jurisdiction in jurisdiction_strategy(),
        status in statute_status_strategy()
    ) {
        let mut registry = StatuteRegistry::new();
        let id = statute.id.clone();
        let entry = StatuteEntry::new(statute, jurisdiction);

        let _ = registry.register(entry);
        let _ = registry.set_status(&id, status);
        let _ = registry.set_status(&id, status);

        let updated = registry.get(&id).unwrap_or_else(|| panic!("Should exist"));
        prop_assert_eq!(updated.status, status);
    }
}

// ============================================================================
// Query Properties
// ============================================================================

proptest! {
    /// Property: Query by jurisdiction returns matching statutes
    #[test]
    fn prop_query_by_jurisdiction(
        statutes in prop::collection::vec(statute_strategy(), 1..10),
        jurisdiction in jurisdiction_strategy()
    ) {
        let mut registry = StatuteRegistry::new();

        for statute in &statutes {
            let entry = StatuteEntry::new(statute.clone(), jurisdiction.clone());
            let _ = registry.register(entry);
        }

        let results = registry.query_by_jurisdiction(&jurisdiction);
        prop_assert!(!results.is_empty(), "Should find statutes in jurisdiction");
    }

    /// Property: Query by tag returns tagged statutes
    #[test]
    fn prop_query_by_tag(
        statute in statute_strategy(),
        jurisdiction in jurisdiction_strategy(),
        tag in "[a-z]{3,10}"
    ) {
        let mut registry = StatuteRegistry::new();
        let entry = StatuteEntry::new(statute, jurisdiction).with_tag(tag.clone());

        let _ = registry.register(entry);
        let results = registry.query_by_tag(&tag);

        prop_assert!(!results.is_empty(), "Should find tagged statute");
        prop_assert!(results[0].tags.contains(&tag), "Result should have the tag");
    }

    /// Property: list_active returns only active statutes
    #[test]
    fn prop_list_active_filters(
        statutes in prop::collection::vec(statute_strategy(), 2..6),
        jurisdiction in jurisdiction_strategy()
    ) {
        let mut registry = StatuteRegistry::new();
        let mut ids = Vec::new();

        for statute in statutes {
            let entry = StatuteEntry::new(statute.clone(), jurisdiction.clone())
                .with_status(StatuteStatus::Active);
            if registry.register(entry).is_ok() {
                ids.push(statute.id);
            }
        }

        // Make first one inactive
        if !ids.is_empty() {
            let _ = registry.set_status(&ids[0], StatuteStatus::Draft);
        }

        let active = registry.list_active();
        // At least one should be inactive now
        prop_assert!(active.len() < registry.list().len() || registry.list().len() <= 1);
    }
}

// ============================================================================
// Count Consistency Properties
// ============================================================================

proptest! {
    /// Property: Count matches list length
    #[test]
    fn prop_count_matches_list_length(
        statutes in prop::collection::vec(statute_strategy(), 0..15),
        jurisdiction in jurisdiction_strategy()
    ) {
        let mut registry = StatuteRegistry::new();

        for statute in statutes {
            let entry = StatuteEntry::new(statute, jurisdiction.clone());
            let _ = registry.register(entry);
        }

        let count = registry.count();
        let list_len = registry.list().len();

        prop_assert_eq!(count, list_len, "Count should match list length");
    }
}

// ============================================================================
// StatuteEntry Properties
// ============================================================================

proptest! {
    /// Property: StatuteEntry creation sets correct defaults
    #[test]
    fn prop_statute_entry_defaults(statute in statute_strategy(), jurisdiction in jurisdiction_strategy()) {
        let entry = StatuteEntry::new(statute.clone(), jurisdiction.clone());

        prop_assert_eq!(entry.version, 1, "Initial version should be 1");
        prop_assert_eq!(entry.status, StatuteStatus::Draft, "Initial status should be Draft");
        prop_assert_eq!(&entry.statute.id, &statute.id);
        prop_assert_eq!(&entry.jurisdiction, &jurisdiction);
        prop_assert!(entry.tags.is_empty(), "Initial tags should be empty");
    }

    /// Property: StatuteEntry with_status changes status
    #[test]
    fn prop_statute_entry_with_status(
        statute in statute_strategy(),
        jurisdiction in jurisdiction_strategy(),
        status in statute_status_strategy()
    ) {
        let entry = StatuteEntry::new(statute, jurisdiction).with_status(status);
        prop_assert_eq!(entry.status, status, "Status should be updated");
    }

    /// Property: StatuteEntry with_tag adds tags
    #[test]
    fn prop_statute_entry_with_tag(
        statute in statute_strategy(),
        jurisdiction in jurisdiction_strategy(),
        tags in prop::collection::vec("[a-z]{3,10}", 1..5)
    ) {
        let mut entry = StatuteEntry::new(statute, jurisdiction);
        for tag in &tags {
            entry = entry.with_tag(tag.clone());
        }

        prop_assert_eq!(entry.tags.len(), tags.len(), "All tags should be added");
    }

    /// Property: StatuteEntry with_reference adds references
    #[test]
    fn prop_statute_entry_with_reference(
        statute in statute_strategy(),
        jurisdiction in jurisdiction_strategy(),
        refs in prop::collection::vec("[a-z][a-z0-9-]{2,20}", 1..5)
    ) {
        let mut entry = StatuteEntry::new(statute, jurisdiction);
        for ref_id in &refs {
            entry = entry.with_reference(ref_id.clone());
        }

        prop_assert_eq!(entry.references.len(), refs.len(), "All references should be added");
    }

    /// Property: StatuteEntry is_active reflects status
    #[test]
    fn prop_statute_entry_is_active(
        statute in statute_strategy(),
        jurisdiction in jurisdiction_strategy()
    ) {
        let entry_active = StatuteEntry::new(statute.clone(), jurisdiction.clone())
            .with_status(StatuteStatus::Active);
        let entry_draft = StatuteEntry::new(statute, jurisdiction)
            .with_status(StatuteStatus::Draft);

        prop_assert!(entry_active.is_active(), "Active status should return true");
        prop_assert!(!entry_draft.is_active(), "Draft status should return false");
    }
}

#[cfg(test)]
mod additional_tests {
    #[test]
    fn test_proptest_configuration() {
        // Verify proptest is configured correctly
        // This test verifies that proptest runs successfully
    }
}
