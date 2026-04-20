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
fn test_permission_editor() {
    let perms = Permission::editor();
    assert!(perms.contains(&Permission::Read));
    assert!(perms.contains(&Permission::Update));
    assert!(!perms.contains(&Permission::Delete));
    assert!(!perms.contains(&Permission::ManagePermissions));
}

#[test]
fn test_role_permissions() {
    assert_eq!(Role::Viewer.permissions().len(), 2);
    assert!(Role::Editor.permissions().len() > 2);
    assert_eq!(Role::Admin.permissions().len(), 12);
}

#[test]
fn test_role_has_permission() {
    assert!(Role::Viewer.has_permission(Permission::Read));
    assert!(!Role::Viewer.has_permission(Permission::Delete));

    assert!(Role::Editor.has_permission(Permission::Read));
    assert!(Role::Editor.has_permission(Permission::Update));
    assert!(!Role::Editor.has_permission(Permission::Delete));

    assert!(Role::Admin.has_permission(Permission::Delete));
    assert!(Role::Admin.has_permission(Permission::ManagePermissions));
}

#[test]
fn test_role_hierarchy() {
    assert!(Role::Admin.is_at_least(Role::Viewer));
    assert!(Role::Admin.is_at_least(Role::Editor));
    assert!(Role::Admin.is_at_least(Role::Admin));

    assert!(Role::Editor.is_at_least(Role::Viewer));
    assert!(Role::Editor.is_at_least(Role::Editor));
    assert!(!Role::Editor.is_at_least(Role::Admin));

    assert!(Role::Viewer.is_at_least(Role::Viewer));
    assert!(!Role::Viewer.is_at_least(Role::Editor));
}

#[test]
fn test_abac_user_attribute() {
    let mut attrs = HashMap::new();
    attrs.insert("department".to_string(), "legal".to_string());

    let condition = AbacCondition::UserAttribute {
        key: "department".to_string(),
        value: "legal".to_string(),
    };

    assert!(condition.evaluate(&attrs, None));

    let condition2 = AbacCondition::UserAttribute {
        key: "department".to_string(),
        value: "finance".to_string(),
    };

    assert!(!condition2.evaluate(&attrs, None));
}

#[test]
fn test_abac_statute_tag() {
    let entry = StatuteEntry::new(test_statute("s1"), "JP").with_tag("criminal");

    let condition = AbacCondition::StatuteTag("criminal".to_string());
    assert!(condition.evaluate(&HashMap::new(), Some(&entry)));

    let condition2 = AbacCondition::StatuteTag("civil".to_string());
    assert!(!condition2.evaluate(&HashMap::new(), Some(&entry)));
}

#[test]
fn test_abac_jurisdiction() {
    let entry = StatuteEntry::new(test_statute("s1"), "JP");

    let condition = AbacCondition::Jurisdiction("JP".to_string());
    assert!(condition.evaluate(&HashMap::new(), Some(&entry)));

    let condition2 = AbacCondition::Jurisdiction("US".to_string());
    assert!(!condition2.evaluate(&HashMap::new(), Some(&entry)));
}

#[test]
fn test_abac_status() {
    let entry = StatuteEntry::new(test_statute("s1"), "JP").with_status(StatuteStatus::Active);

    let condition = AbacCondition::Status(StatuteStatus::Active);
    assert!(condition.evaluate(&HashMap::new(), Some(&entry)));

    let condition2 = AbacCondition::Status(StatuteStatus::Draft);
    assert!(!condition2.evaluate(&HashMap::new(), Some(&entry)));
}

#[test]
fn test_abac_time_range() {
    let now = Utc::now();
    let past = now - chrono::Duration::hours(1);
    let future = now + chrono::Duration::hours(1);

    let condition = AbacCondition::TimeRange {
        start: past,
        end: future,
    };
    assert!(condition.evaluate(&HashMap::new(), None));

    let expired_condition = AbacCondition::TimeRange {
        start: past - chrono::Duration::hours(2),
        end: past,
    };
    assert!(!expired_condition.evaluate(&HashMap::new(), None));
}

#[test]
fn test_abac_and_condition() {
    let mut attrs = HashMap::new();
    attrs.insert("department".to_string(), "legal".to_string());

    let entry = StatuteEntry::new(test_statute("s1"), "JP").with_tag("criminal");

    let condition = AbacCondition::And(vec![
        AbacCondition::UserAttribute {
            key: "department".to_string(),
            value: "legal".to_string(),
        },
        AbacCondition::StatuteTag("criminal".to_string()),
    ]);

    assert!(condition.evaluate(&attrs, Some(&entry)));

    // Change one condition to false
    let condition2 = AbacCondition::And(vec![
        AbacCondition::UserAttribute {
            key: "department".to_string(),
            value: "finance".to_string(),
        },
        AbacCondition::StatuteTag("criminal".to_string()),
    ]);

    assert!(!condition2.evaluate(&attrs, Some(&entry)));
}

#[test]
fn test_abac_or_condition() {
    let attrs = HashMap::new();
    let entry = StatuteEntry::new(test_statute("s1"), "JP");

    let condition = AbacCondition::Or(vec![
        AbacCondition::Jurisdiction("US".to_string()),
        AbacCondition::Jurisdiction("JP".to_string()),
    ]);

    assert!(condition.evaluate(&attrs, Some(&entry)));
}

#[test]
fn test_abac_not_condition() {
    let entry = StatuteEntry::new(test_statute("s1"), "JP");

    let condition = AbacCondition::Not(Box::new(AbacCondition::Jurisdiction("US".to_string())));

    assert!(condition.evaluate(&HashMap::new(), Some(&entry)));
}

#[test]
fn test_access_policy_creation() {
    let policy = AccessPolicy::new("Test Policy", vec![Permission::Read])
        .with_role(Role::Viewer)
        .with_priority(10);

    assert_eq!(policy.name, "Test Policy");
    assert_eq!(policy.required_role, Some(Role::Viewer));
    assert_eq!(policy.priority, 10);
    assert!(policy.enabled);
}

#[test]
fn test_access_policy_grants() {
    let policy = AccessPolicy::new("Test", vec![Permission::Read, Permission::Update]);

    assert!(policy.grants(Permission::Read));
    assert!(policy.grants(Permission::Update));
    assert!(!policy.grants(Permission::Delete));
}

#[test]
fn test_temporary_access_creation() {
    let grant = TemporaryAccess::new(
        "user1",
        vec![Permission::Read],
        24,
        "Emergency access",
        "admin",
    );

    assert_eq!(grant.user_id, "user1");
    assert_eq!(grant.permissions.len(), 1);
    assert!(grant.is_valid());
    assert!(grant.remaining_seconds() > 0);
}

#[test]
fn test_temporary_access_for_statute() {
    let grant = TemporaryAccess::new("user1", vec![Permission::Update], 1, "Quick fix", "admin")
        .for_statute("s1");

    assert!(grant.applies_to("s1"));
    assert!(!grant.applies_to("s2"));
}

#[test]
fn test_temporary_access_expiration() {
    let mut grant = TemporaryAccess::new("user1", vec![Permission::Read], 1, "Test", "admin");

    // Manually set to expired
    grant.valid_until = Utc::now() - chrono::Duration::hours(1);

    assert!(!grant.is_valid());
    assert_eq!(grant.remaining_seconds(), 0);
}

#[test]
fn test_access_user_creation() {
    let user = AccessUser::new("user1", "Alice", Role::Editor)
        .with_attribute("department", "legal")
        .with_permission(Permission::Delete);

    assert_eq!(user.user_id, "user1");
    assert_eq!(user.display_name, "Alice");
    assert_eq!(user.role, Role::Editor);
    assert_eq!(user.attributes.get("department").unwrap(), "legal");
    assert!(user.has_permission(Permission::Delete));
}

#[test]
fn test_access_user_all_permissions() {
    let user = AccessUser::new("user1", "Alice", Role::Viewer).with_permission(Permission::Update);

    let perms = user.all_permissions();
    assert!(perms.contains(&Permission::Read)); // From role
    assert!(perms.contains(&Permission::Update)); // Direct permission
}

#[test]
fn test_access_control_manager_add_user() {
    let mut acm = AccessControlManager::new();
    let user = AccessUser::new("user1", "Alice", Role::Editor);

    acm.add_user(user);
    assert_eq!(acm.user_count(), 1);
    assert!(acm.get_user("user1").is_some());
}

#[test]
fn test_access_control_manager_update_role() {
    let mut acm = AccessControlManager::new();
    let user = AccessUser::new("user1", "Alice", Role::Viewer);
    acm.add_user(user);

    assert!(acm.update_user_role("user1", Role::Admin));
    assert_eq!(acm.get_user("user1").unwrap().role, Role::Admin);

    assert!(!acm.update_user_role("nonexistent", Role::Admin));
}

#[test]
fn test_access_control_manager_add_policy() {
    let mut acm = AccessControlManager::new();
    let policy = AccessPolicy::new("Policy1", vec![Permission::Read]).with_priority(10);

    acm.add_policy(policy);
    assert_eq!(acm.policy_count(), 1);
}

#[test]
fn test_access_control_manager_check_permission_direct() {
    let mut acm = AccessControlManager::new();
    let user = AccessUser::new("user1", "Alice", Role::Admin);
    acm.add_user(user);

    // Admin has all permissions
    assert!(acm.check_permission("user1", Permission::Delete, None, None));
    assert!(acm.check_permission("user1", Permission::Read, None, None));
}

#[test]
fn test_access_control_manager_check_permission_unknown_user() {
    let acm = AccessControlManager::new();

    // Unknown user should be denied
    assert!(!acm.check_permission("unknown", Permission::Read, None, None));
}

#[test]
fn test_access_control_manager_temporary_grant() {
    let mut acm = AccessControlManager::new();
    let user = AccessUser::new("user1", "Alice", Role::Viewer);
    acm.add_user(user);

    let grant = TemporaryAccess::new("user1", vec![Permission::Delete], 1, "Emergency", "admin")
        .for_statute("s1");

    acm.grant_temporary_access(grant);

    // User should have delete permission on s1 via temporary grant
    assert!(acm.check_permission("user1", Permission::Delete, Some("s1"), None));
    // But not on s2
    assert!(!acm.check_permission("user1", Permission::Delete, Some("s2"), None));
}

#[test]
fn test_access_control_manager_policy_with_abac() {
    let mut acm = AccessControlManager::new();
    let user =
        AccessUser::new("user1", "Alice", Role::Editor).with_attribute("department", "legal");
    acm.add_user(user);

    let entry = StatuteEntry::new(test_statute("s1"), "JP").with_tag("criminal");

    // Policy that requires legal department AND criminal tag
    let policy = AccessPolicy::new("Legal Only", vec![Permission::Delete]).with_condition(
        AbacCondition::And(vec![
            AbacCondition::UserAttribute {
                key: "department".to_string(),
                value: "legal".to_string(),
            },
            AbacCondition::StatuteTag("criminal".to_string()),
        ]),
    );

    acm.add_policy(policy);

    // Should grant permission because conditions are met
    assert!(acm.check_permission("user1", Permission::Delete, Some("s1"), Some(&entry)));
}

#[test]
fn test_access_control_manager_cleanup_grants() {
    let mut acm = AccessControlManager::new();

    let mut expired_grant =
        TemporaryAccess::new("user1", vec![Permission::Read], 1, "Test", "admin");
    expired_grant.valid_until = Utc::now() - chrono::Duration::hours(1);

    let valid_grant = TemporaryAccess::new("user2", vec![Permission::Read], 24, "Test", "admin");

    acm.grant_temporary_access(expired_grant);
    acm.grant_temporary_access(valid_grant);

    assert_eq!(acm.temporary_grants.len(), 2);
    assert_eq!(acm.active_grant_count(), 1);

    acm.cleanup_expired_grants();
    assert_eq!(acm.temporary_grants.len(), 1);
}

#[test]
fn test_access_control_manager_list_user_grants() {
    let mut acm = AccessControlManager::new();

    let grant1 = TemporaryAccess::new("user1", vec![Permission::Read], 1, "Test", "admin");
    let grant2 = TemporaryAccess::new("user1", vec![Permission::Update], 1, "Test", "admin");
    let grant3 = TemporaryAccess::new("user2", vec![Permission::Delete], 1, "Test", "admin");

    acm.grant_temporary_access(grant1);
    acm.grant_temporary_access(grant2);
    acm.grant_temporary_access(grant3);

    let user1_grants = acm.list_user_grants("user1");
    assert_eq!(user1_grants.len(), 2);
}

#[test]
fn test_access_control_manager_revoke_grant() {
    let mut acm = AccessControlManager::new();
    let grant = TemporaryAccess::new("user1", vec![Permission::Read], 1, "Test", "admin");
    let grant_id = grant.grant_id;

    acm.grant_temporary_access(grant);
    assert_eq!(acm.temporary_grants.len(), 1);

    assert!(acm.revoke_grant(grant_id));
    assert_eq!(acm.temporary_grants.len(), 0);

    // Revoking again should return false
    assert!(!acm.revoke_grant(grant_id));
}

#[test]
fn test_access_control_manager_disabled() {
    let mut acm = AccessControlManager::new();
    acm.set_enabled(false);

    // When disabled, all permissions should be granted
    assert!(acm.check_permission("unknown", Permission::Delete, None, None));
    assert!(!acm.is_enabled());

    acm.set_enabled(true);
    assert!(!acm.check_permission("unknown", Permission::Delete, None, None));
    assert!(acm.is_enabled());
}

#[test]
fn test_access_policy_priority_sorting() {
    let mut acm = AccessControlManager::new();

    let policy1 = AccessPolicy::new("Low", vec![Permission::Read]).with_priority(1);
    let policy2 = AccessPolicy::new("High", vec![Permission::Update]).with_priority(10);
    let policy3 = AccessPolicy::new("Medium", vec![Permission::Delete]).with_priority(5);

    acm.add_policy(policy1);
    acm.add_policy(policy2);
    acm.add_policy(policy3);

    // Policies should be sorted by priority (descending)
    assert_eq!(acm.policies[0].name, "High");
    assert_eq!(acm.policies[1].name, "Medium");
    assert_eq!(acm.policies[2].name, "Low");
}

// ========================================================================
// Import/Export Extensions Tests (v0.1.5)
// ========================================================================

#[test]
fn test_import_source_creation() {
    use government_import::*;

    let source = ImportSource::new("test", "http://example.com", GovernmentDataFormat::Json)
        .with_credentials("token123")
        .with_metadata("version", "1.0");

    assert_eq!(source.name, "test");
    assert_eq!(source.location, "http://example.com");
    assert_eq!(source.format, GovernmentDataFormat::Json);
    assert_eq!(source.credentials, Some("token123".to_string()));
    assert_eq!(source.metadata.get("version"), Some(&"1.0".to_string()));
}

#[test]
fn test_bulk_import_result() {
    use government_import::*;

    let mut result = BulkImportResult::new("test");
    result.imported = 10;
    result.skipped = 2;
    result.failed = 1;

    assert_eq!(result.total_processed(), 13);
    assert_eq!(result.success_rate(), 10.0 / 13.0);
    assert!(!result.is_success());

    let success_result = BulkImportResult::new("success");
    assert!(success_result.is_success());
}

#[test]
fn test_bulk_importer_skip_strategy() {
    use government_import::*;

    let mut registry = StatuteRegistry::new();
    let importer = BulkImporter::new().with_strategy(ImportStrategy::Skip);

    let statute1 = test_statute("TEST-1");
    let entry1 = StatuteEntry::new(statute1.clone(), "US");

    // First import should succeed
    registry.register(entry1.clone()).unwrap();

    let statute2 = test_statute("TEST-2");
    let entry2 = StatuteEntry::new(statute2, "US");

    let source = ImportSource::new("test", "local", GovernmentDataFormat::Json);
    let result = importer.import(&mut registry, &source, vec![entry1, entry2]);

    assert_eq!(result.imported, 1); // Only TEST-2
    assert_eq!(result.skipped, 1); // TEST-1 already exists
    assert_eq!(result.failed, 0);
}

#[test]
fn test_bulk_importer_update_strategy() {
    use government_import::*;

    let mut registry = StatuteRegistry::new();
    let importer = BulkImporter::new().with_strategy(ImportStrategy::Update);

    let statute1 = test_statute("TEST-1");
    let entry1 = StatuteEntry::new(statute1.clone(), "US");

    registry.register(entry1.clone()).unwrap();

    let mut updated_statute = test_statute("TEST-1");
    updated_statute.title = "Updated Title".to_string();
    let updated_entry = StatuteEntry::new(updated_statute, "US");

    let source = ImportSource::new("test", "local", GovernmentDataFormat::Json);
    let result = importer.import(&mut registry, &source, vec![updated_entry]);

    assert_eq!(result.imported, 1);
    assert_eq!(result.skipped, 0);
    assert_eq!(result.failed, 0);

    let stored = registry.get("TEST-1").unwrap();
    assert_eq!(stored.statute.title, "Updated Title");
}

#[test]
fn test_bulk_importer_validation() {
    use government_import::*;

    let mut registry = StatuteRegistry::new();
    let importer = BulkImporter::new()
        .with_validation(true)
        .with_strategy(ImportStrategy::Skip);

    // Create an invalid entry (empty ID)
    let mut statute = test_statute("TEST-1");
    statute.id = "".to_string(); // Invalid: empty ID
    let entry = StatuteEntry::new(statute, "US");

    let source = ImportSource::new("test", "local", GovernmentDataFormat::Json);
    let result = importer.import(&mut registry, &source, vec![entry]);

    assert_eq!(result.imported, 0);
    assert_eq!(result.failed, 1);
    assert!(!result.errors.is_empty());
}

#[test]
fn test_sync_schedule() {
    use sync::*;

    let now = Utc::now();

    // Manual schedule should never be due
    let manual = SyncSchedule::Manual;
    assert!(!manual.is_due(now, now + chrono::Duration::hours(1)));

    // Hourly schedule
    let hourly = SyncSchedule::Hourly;
    assert!(!hourly.is_due(now, now + chrono::Duration::minutes(30)));
    assert!(hourly.is_due(now, now + chrono::Duration::hours(1)));

    // Daily schedule
    let daily = SyncSchedule::Daily { hour: 10 };
    assert!(daily.next_sync(now).is_some());

    // Interval schedule
    let interval = SyncSchedule::Interval { seconds: 3600 };
    assert!(!interval.is_due(now, now + chrono::Duration::minutes(30)));
    assert!(interval.is_due(now, now + chrono::Duration::hours(1)));
}

#[test]
fn test_sync_job() {
    use government_import::*;
    use sync::*;

    let source = ImportSource::new("test", "local", GovernmentDataFormat::Json);
    let mut job = SyncJob::new("Test Job", source, SyncSchedule::Hourly);

    assert!(job.enabled);
    assert!(job.is_due(Utc::now())); // Never synced, so it's due

    let result = BulkImportResult::new("test");
    job.mark_completed(result);

    assert!(job.last_sync.is_some());
    assert!(job.last_result.is_some());
}

#[test]
fn test_sync_manager() {
    use government_import::*;
    use sync::*;

    let mut manager = SyncManager::new();

    let source = ImportSource::new("test", "local", GovernmentDataFormat::Json);
    let job = SyncJob::new("Test Job", source, SyncSchedule::Hourly);
    let job_id = job.id;

    manager.add_job(job);
    assert_eq!(manager.jobs().len(), 1);

    // Get due jobs
    let due = manager.due_jobs(Utc::now());
    assert_eq!(due.len(), 1);

    // Disable job
    assert!(manager.set_job_enabled(job_id, false));
    let due_after_disable = manager.due_jobs(Utc::now());
    assert_eq!(due_after_disable.len(), 0);

    // Remove job
    assert!(manager.remove_job(job_id));
    assert_eq!(manager.jobs().len(), 0);
}

#[test]
fn test_format_migrator() {
    use migration::*;

    let migrator = FormatMigrator::new();

    let data = r#"{"test": "data"}"#;
    let result = migrator.migrate(
        MigrationFormat::JsonCurrent,
        MigrationFormat::JsonCurrent,
        data,
    );

    assert!(result.is_ok());
    let (migrated_data, migration_result) = result.unwrap();
    assert_eq!(migrated_data, data);
    assert_eq!(migration_result.migrated, 1);
    assert_eq!(migration_result.failed, 0);
    assert_eq!(migration_result.success_rate(), 1.0);
}

#[test]
fn test_format_migrator_unsupported() {
    use migration::*;

    let migrator = FormatMigrator::new();
    let data = "<xml></xml>";

    let result = migrator.migrate(
        MigrationFormat::XmlLegacy,
        MigrationFormat::JsonCurrent,
        data,
    );

    assert!(result.is_err());
}

#[test]
fn test_report_template() {
    use templates::*;

    let template = ReportTemplate::new("Test", TemplateType::Summary, ExportFormat::Json)
        .with_field("id")
        .with_field("title")
        .with_filter("status", "active")
        .with_sort_by("created_at");

    assert_eq!(template.name, "Test");
    assert_eq!(template.template_type, TemplateType::Summary);
    assert_eq!(template.format, ExportFormat::Json);
    assert_eq!(template.fields.len(), 2);
    assert_eq!(template.filters.get("status"), Some(&"active".to_string()));
    assert_eq!(template.sort_by, Some("created_at".to_string()));
}

#[test]
fn test_report_template_factories() {
    use templates::*;

    let summary = ReportTemplate::summary(ExportFormat::Json);
    assert_eq!(summary.template_type, TemplateType::Summary);
    assert!(summary.fields.contains(&"id".to_string()));

    let detailed = ReportTemplate::detailed(ExportFormat::Csv);
    assert_eq!(detailed.template_type, TemplateType::Detailed);
    assert!(detailed.fields.contains(&"metadata".to_string()));

    let compliance = ReportTemplate::compliance(ExportFormat::Html);
    assert_eq!(compliance.template_type, TemplateType::Compliance);
    assert!(compliance.fields.contains(&"effective_date".to_string()));
}

#[test]
fn test_template_manager() {
    use templates::*;

    let mut manager = TemplateManager::new();

    let template = ReportTemplate::summary(ExportFormat::Json);
    manager.add_template(template);

    assert_eq!(manager.list_templates().len(), 1);
    assert!(manager.get_template("Summary Report").is_some());

    assert!(manager.remove_template("Summary Report"));
    assert_eq!(manager.list_templates().len(), 0);
}

#[test]
fn test_template_export_json() {
    use templates::*;

    let mut registry = StatuteRegistry::new();
    let statute = test_statute("TEST-1");
    let entry = StatuteEntry::new(statute, "US");
    registry.register(entry).unwrap();

    let mut manager = TemplateManager::new();
    let template = ReportTemplate::summary(ExportFormat::Json);
    manager.add_template(template);

    let result = manager.export(&registry, "Summary Report");
    assert!(result.is_ok());
    let json = result.unwrap();
    assert!(json.contains("TEST-1"));
}

#[test]
fn test_template_export_csv() {
    use templates::*;

    let mut registry = StatuteRegistry::new();
    let statute = test_statute("TEST-1");
    let entry = StatuteEntry::new(statute, "US");
    registry.register(entry).unwrap();

    let mut manager = TemplateManager::new();
    let template = ReportTemplate::summary(ExportFormat::Csv);
    manager.add_template(template);

    let result = manager.export(&registry, "Summary Report");
    assert!(result.is_ok());
    let csv = result.unwrap();
    assert!(csv.contains("id,title,status,jurisdiction"));
    assert!(csv.contains("TEST-1"));
}

#[test]
fn test_template_export_html() {
    use templates::*;

    let mut registry = StatuteRegistry::new();
    let statute = test_statute("TEST-1");
    let entry = StatuteEntry::new(statute, "US");
    registry.register(entry).unwrap();

    let mut manager = TemplateManager::new();
    let template = ReportTemplate::summary(ExportFormat::Html);
    manager.add_template(template);

    let result = manager.export(&registry, "Summary Report");
    assert!(result.is_ok());
    let html = result.unwrap();
    assert!(html.contains("<html>"));
    assert!(html.contains("<table"));
    assert!(html.contains("TEST-1"));
}

#[test]
fn test_template_export_markdown() {
    use templates::*;

    let mut registry = StatuteRegistry::new();
    let statute = test_statute("TEST-1");
    let entry = StatuteEntry::new(statute, "US");
    registry.register(entry).unwrap();

    let mut manager = TemplateManager::new();
    let template = ReportTemplate::summary(ExportFormat::Markdown);
    manager.add_template(template);

    let result = manager.export(&registry, "Summary Report");
    assert!(result.is_ok());
    let md = result.unwrap();
    assert!(md.contains("# Summary Report"));
    assert!(md.contains("|"));
    assert!(md.contains("TEST-1"));
}

#[test]
fn test_template_export_not_found() {
    use templates::*;

    let registry = StatuteRegistry::new();
    let manager = TemplateManager::new();

    let result = manager.export(&registry, "Nonexistent Template");
    assert!(result.is_err());
}

#[test]
fn test_export_filtered_statutes() {
    let mut registry = StatuteRegistry::new();

    let statute1 = test_statute("TEST-1");
    let mut entry1 = StatuteEntry::new(statute1, "US");
    entry1.tags.push("tax".to_string());

    let statute2 = test_statute("TEST-2");
    let mut entry2 = StatuteEntry::new(statute2, "EU");
    entry2.tags.push("gdpr".to_string());

    registry.register(entry1).unwrap();
    registry.register(entry2).unwrap();

    let result = registry.export_filtered_statutes(|e| e.jurisdiction == "US");
    assert!(result.is_ok());
    let json = result.unwrap();
    assert!(json.contains("TEST-1"));
    assert!(!json.contains("TEST-2"));
}

#[test]
fn test_export_by_status() {
    let mut registry = StatuteRegistry::new();

    let statute = test_statute("TEST-1");
    let entry = StatuteEntry::new(statute, "US");
    registry.register(entry).unwrap();
    registry
        .set_status("TEST-1", StatuteStatus::Active)
        .unwrap();

    let result = registry.export_by_status(StatuteStatus::Active);
    assert!(result.is_ok());
    let json = result.unwrap();
    assert!(json.contains("TEST-1"));
}

#[test]
fn test_export_by_jurisdiction() {
    let mut registry = StatuteRegistry::new();

    let statute1 = test_statute("TEST-1");
    let entry1 = StatuteEntry::new(statute1, "US");

    let statute2 = test_statute("TEST-2");
    let entry2 = StatuteEntry::new(statute2, "EU");

    registry.register(entry1).unwrap();
    registry.register(entry2).unwrap();

    let result = registry.export_by_jurisdiction("US");
    assert!(result.is_ok());
    let json = result.unwrap();
    assert!(json.contains("TEST-1"));
    assert!(!json.contains("TEST-2"));
}

#[test]
fn test_export_by_tag() {
    let mut registry = StatuteRegistry::new();

    let statute1 = test_statute("TEST-1");
    let mut entry1 = StatuteEntry::new(statute1, "US");
    entry1.tags.push("tax".to_string());

    let statute2 = test_statute("TEST-2");
    let mut entry2 = StatuteEntry::new(statute2, "US");
    entry2.tags.push("gdpr".to_string());

    registry.register(entry1).unwrap();
    registry.register(entry2).unwrap();

    let result = registry.export_by_tag("tax");
    assert!(result.is_ok());
    let json = result.unwrap();
    assert!(json.contains("TEST-1"));
    assert!(!json.contains("TEST-2"));
}

#[test]
fn test_export_by_date_range() {
    let mut registry = StatuteRegistry::new();

    let statute = test_statute("TEST-1");
    let entry = StatuteEntry::new(statute, "US");
    registry.register(entry).unwrap();

    let start = Utc::now() - chrono::Duration::days(1);
    let end = Utc::now() + chrono::Duration::days(1);

    let result = registry.export_by_date_range(start, end);
    assert!(result.is_ok());
    let json = result.unwrap();
    assert!(json.contains("TEST-1"));
}

#[test]
fn test_government_data_format_variants() {
    use government_import::*;

    let _json = GovernmentDataFormat::Json;
    let _xml = GovernmentDataFormat::Xml;
    let _csv = GovernmentDataFormat::Csv;
    let _dsv = GovernmentDataFormat::Dsv { delimiter: '|' };
    let _akoma = GovernmentDataFormat::AkomaNtoso;
    let _legal = GovernmentDataFormat::LegalDocML;
}

#[test]
fn test_import_strategy_variants() {
    use government_import::*;

    let _skip = ImportStrategy::Skip;
    let _update = ImportStrategy::Update;
    let _new_version = ImportStrategy::NewVersion;
    let _fail = ImportStrategy::FailOnDuplicate;
}

#[test]
fn test_migration_format_variants() {
    use migration::*;

    let _v1 = MigrationFormat::JsonV1;
    let _v2 = MigrationFormat::JsonV2;
    let _current = MigrationFormat::JsonCurrent;
    let _xml = MigrationFormat::XmlLegacy;
    let _akoma = MigrationFormat::AkomaNtoso;
    let _csv = MigrationFormat::Csv;
}

#[test]
fn test_template_type_variants() {
    use templates::*;

    let _summary = TemplateType::Summary;
    let _detailed = TemplateType::Detailed;
    let _compliance = TemplateType::Compliance;
    let _audit = TemplateType::AuditTrail;
    let _custom = TemplateType::Custom("MyTemplate".to_string());
}

#[test]
fn test_export_format_variants() {
    use templates::*;

    let _json = ExportFormat::Json;
    let _csv = ExportFormat::Csv;
    let _html = ExportFormat::Html;
    let _md = ExportFormat::Markdown;
    let _pdf = ExportFormat::Pdf;
}

// ========== Workflow Integration Tests (v0.1.6) ==========

#[test]
fn test_workflow_approval_request() {
    use workflow::*;

    let request = ApprovalRequest::new(ChangeType::Create, "user123", "statute_data")
        .with_justification("Adding new statute")
        .with_approver("approver1")
        .with_approver("approver2");

    assert_eq!(request.submitter, "user123");
    assert_eq!(request.status, WorkflowStatus::Draft);
    assert_eq!(request.approvers.len(), 2);
    assert!(request.justification.is_some());
}

#[test]
fn test_workflow_submit() {
    use workflow::*;

    let mut request = ApprovalRequest::new(
        ChangeType::Update {
            statute_id: "STAT-1".to_string(),
        },
        "user456",
        "updated_data",
    );

    request.submit();
    assert_eq!(request.status, WorkflowStatus::PendingApproval);
}

#[test]
fn test_workflow_approval_response() {
    use workflow::*;

    let response =
        ApprovalResponse::new("approver1", ApprovalDecision::Approved).with_comments("Looks good");

    assert_eq!(response.approver, "approver1");
    assert_eq!(response.decision, ApprovalDecision::Approved);
    assert!(response.comments.is_some());
}

#[test]
fn test_workflow_manager_submit() {
    use workflow::*;

    let mut manager = WorkflowManager::new();
    let request = ApprovalRequest::new(ChangeType::Create, "user123", "data");

    let id = manager.submit_request(request);
    assert!(manager.get_request(id).is_some());
}

#[test]
fn test_workflow_manager_add_response() {
    use workflow::*;

    let mut manager = WorkflowManager::new();
    let mut request =
        ApprovalRequest::new(ChangeType::Create, "user123", "data").with_approver("approver1");

    request.submit();
    let id = manager.submit_request(request);

    let response = ApprovalResponse::new("approver1", ApprovalDecision::Approved);
    let result = manager.add_response(id, response);

    assert!(result.is_ok());
    let req = manager.get_request(id).unwrap();
    assert_eq!(req.status, WorkflowStatus::Approved);
}

#[test]
fn test_workflow_manager_pending_requests() {
    use workflow::*;

    let mut manager = WorkflowManager::new();

    let req1 = ApprovalRequest::new(ChangeType::Create, "user1", "data1");
    manager.submit_request(req1);

    let req2 = ApprovalRequest::new(ChangeType::Create, "user2", "data2");
    manager.submit_request(req2);

    let pending = manager.pending_requests();
    assert_eq!(pending.len(), 2); // Both are pending approval
}

#[test]
fn test_notification_creation() {
    use notifications::*;

    let notification = Notification::new(
        "user123",
        NotificationType::ApprovalRequested,
        "New Approval Request",
        "Please review the statute change",
    )
    .with_priority(NotificationPriority::High)
    .with_related_entity("request-123")
    .with_channel(NotificationChannel::Email);

    assert_eq!(notification.recipient, "user123");
    assert_eq!(notification.priority, NotificationPriority::High);
    assert!(notification.related_entity_id.is_some());
    assert_eq!(notification.channels.len(), 2); // InApp (default) + Email
}

#[test]
fn test_notification_mark_sent_read() {
    use notifications::*;

    let mut notification = Notification::new(
        "user123",
        NotificationType::ApprovalGranted,
        "Approved",
        "Your request was approved",
    );

    assert!(!notification.is_sent());
    assert!(!notification.is_read());

    notification.mark_sent();
    assert!(notification.is_sent());

    notification.mark_read();
    assert!(notification.is_read());
}

#[test]
fn test_notification_manager() {
    use notifications::*;

    let mut manager = NotificationManager::new();

    let notification = Notification::new(
        "user123",
        NotificationType::TaskAssigned,
        "New Task",
        "You have a new review task",
    );

    let id = notification.notification_id;
    manager.send(notification);

    let unread = manager.unread_for_user("user123");
    assert_eq!(unread.len(), 1);

    manager.mark_as_read(id);
    let unread_after = manager.unread_for_user("user123");
    assert_eq!(unread_after.len(), 0);
}

#[test]
fn test_notification_priority_filter() {
    use notifications::*;

    let mut manager = NotificationManager::new();

    manager.send(
        Notification::new("user1", NotificationType::TaskAssigned, "Low", "msg")
            .with_priority(NotificationPriority::Low),
    );
    manager.send(
        Notification::new("user1", NotificationType::SlaBreach, "Critical", "msg")
            .with_priority(NotificationPriority::Critical),
    );

    let high_priority = manager.by_priority(NotificationPriority::High);
    assert_eq!(high_priority.len(), 1); // Only critical meets threshold
}

#[test]
fn test_review_task_creation() {
    use tasks::*;

    let task = ReviewTask::new(
        "Review GDPR Statute",
        "user123",
        "manager456",
        "STATUTE-GDPR",
    )
    .with_description("Please review the GDPR implementation");

    assert_eq!(task.title, "Review GDPR Statute");
    assert_eq!(task.assigned_to, "user123");
    assert_eq!(task.status, TaskStatus::NotStarted);
    assert!(task.description.is_some());
}
