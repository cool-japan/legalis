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
fn test_bulk_operation_delete() {
    use api_extensions::bulk::*;

    let registry = Arc::new(Mutex::new(StatuteRegistry::new()));
    {
        let mut reg = registry.lock().unwrap();
        reg.register(StatuteEntry::new(test_statute("S1"), "JP"))
            .unwrap();
        reg.register(StatuteEntry::new(test_statute("S2"), "JP"))
            .unwrap();
    }

    let executor = BulkOperationExecutor::new(registry);

    let request = BulkOperationRequest {
        operation_type: BulkOperationType::Delete,
        statute_ids: vec!["S1".to_string(), "S2".to_string()],
        statute_entries: vec![],
        new_status: None,
        continue_on_error: true,
    };

    let response = executor.execute(request);
    assert_eq!(response.total_processed, 2);
    assert_eq!(response.successful, 2);
    assert_eq!(response.failed, 0);
}

#[test]
fn test_bulk_operation_change_status() {
    use api_extensions::bulk::*;

    let registry = Arc::new(Mutex::new(StatuteRegistry::new()));
    {
        let mut reg = registry.lock().unwrap();
        reg.register(StatuteEntry::new(test_statute("S1"), "JP"))
            .unwrap();
        reg.register(StatuteEntry::new(test_statute("S2"), "JP"))
            .unwrap();
    }

    let executor = BulkOperationExecutor::new(registry);

    let request = BulkOperationRequest {
        operation_type: BulkOperationType::ChangeStatus,
        statute_ids: vec!["S1".to_string(), "S2".to_string()],
        statute_entries: vec![],
        new_status: Some(StatuteStatus::Repealed),
        continue_on_error: true,
    };

    let response = executor.execute(request);
    assert_eq!(response.successful, 2);
    assert_eq!(response.failed, 0);
}

#[test]
fn test_bulk_operation_type_variants() {
    use api_extensions::bulk::*;

    let _register = BulkOperationType::Register;
    let _update = BulkOperationType::Update;
    let _delete = BulkOperationType::Delete;
    let _archive = BulkOperationType::Archive;
    let _change_status = BulkOperationType::ChangeStatus;
}

#[test]
fn test_bulk_operation_response_metrics() {
    use api_extensions::bulk::*;

    let mut response = BulkOperationResponse::new(BulkOperationType::Register);
    response.total_processed = 10;
    response.successful = 7;
    response.failed = 3;

    assert_eq!(response.success_rate(), 0.7);
    assert!(!response.is_complete_success());
}

#[test]
fn test_sdk_generation_python() {
    use api_extensions::sdk_gen::*;

    let config = SdkConfig {
        language: SdkLanguage::Python,
        package_name: "legalis_sdk".to_string(),
        api_base_url: "https://api.example.com".to_string(),
        async_support: true,
        type_definitions: true,
        include_docs: true,
    };

    let code = SdkGenerator::generate(&config).unwrap();
    assert!(code.contains("legalis_sdk Python SDK"));
    assert!(code.contains("class StatuteRegistryClient"));
    assert!(code.contains("def get_statute"));
}

#[test]
fn test_sdk_generation_javascript() {
    use api_extensions::sdk_gen::*;

    let config = SdkConfig {
        language: SdkLanguage::JavaScript,
        package_name: "legalis-sdk".to_string(),
        api_base_url: "https://api.example.com".to_string(),
        async_support: true,
        type_definitions: false,
        include_docs: false,
    };

    let code = SdkGenerator::generate(&config).unwrap();
    assert!(code.contains("legalis-sdk JavaScript SDK"));
    assert!(code.contains("class StatuteRegistryClient"));
    assert!(code.contains("async getStatute"));
}

#[test]
fn test_sdk_generation_typescript() {
    use api_extensions::sdk_gen::*;

    let config = SdkConfig {
        language: SdkLanguage::TypeScript,
        package_name: "legalis-sdk".to_string(),
        api_base_url: "https://api.example.com".to_string(),
        async_support: true,
        type_definitions: true,
        include_docs: true,
    };

    let code = SdkGenerator::generate(&config).unwrap();
    assert!(code.contains("legalis-sdk TypeScript SDK"));
    assert!(code.contains("export interface Statute"));
    assert!(code.contains("export class StatuteRegistryClient"));
}

#[test]
fn test_sdk_generation_rust() {
    use api_extensions::sdk_gen::*;

    let config = SdkConfig {
        language: SdkLanguage::Rust,
        package_name: "legalis-sdk".to_string(),
        api_base_url: "https://api.example.com".to_string(),
        async_support: true,
        type_definitions: true,
        include_docs: true,
    };

    let code = SdkGenerator::generate(&config).unwrap();
    assert!(code.contains("legalis-sdk Rust SDK"));
    assert!(code.contains("pub struct Statute"));
    assert!(code.contains("pub struct StatuteRegistryClient"));
    assert!(code.contains("pub async fn get_statute"));
}

#[test]
fn test_sdk_generation_go() {
    use api_extensions::sdk_gen::*;

    let config = SdkConfig {
        language: SdkLanguage::Go,
        package_name: "legalis-sdk".to_string(),
        api_base_url: "https://api.example.com".to_string(),
        async_support: false,
        type_definitions: true,
        include_docs: true,
    };

    let code = SdkGenerator::generate(&config).unwrap();
    assert!(code.contains("legalis-sdk Go SDK"));
    assert!(code.contains("type Statute struct"));
    assert!(code.contains("type Client struct"));
}

#[test]
fn test_sdk_generation_java() {
    use api_extensions::sdk_gen::*;

    let config = SdkConfig {
        language: SdkLanguage::Java,
        package_name: "LegalisSDK".to_string(),
        api_base_url: "https://api.example.com".to_string(),
        async_support: false,
        type_definitions: true,
        include_docs: true,
    };

    let code = SdkGenerator::generate(&config).unwrap();
    assert!(code.contains("LegalisSDK Java SDK"));
    assert!(code.contains("public class StatuteRegistryClient"));
}

#[test]
fn test_sdk_generation_csharp() {
    use api_extensions::sdk_gen::*;

    let config = SdkConfig {
        language: SdkLanguage::CSharp,
        package_name: "LegalisSDK".to_string(),
        api_base_url: "https://api.example.com".to_string(),
        async_support: true,
        type_definitions: true,
        include_docs: true,
    };

    let code = SdkGenerator::generate(&config).unwrap();
    assert!(code.contains("LegalisSDK C# SDK"));
    assert!(code.contains("namespace LegalisSDK"));
    assert!(code.contains("public class StatuteRegistryClient"));
}

#[test]
fn test_sdk_generation_ruby() {
    use api_extensions::sdk_gen::*;

    let config = SdkConfig {
        language: SdkLanguage::Ruby,
        package_name: "LegalisSDK".to_string(),
        api_base_url: "https://api.example.com".to_string(),
        async_support: false,
        type_definitions: false,
        include_docs: true,
    };

    let code = SdkGenerator::generate(&config).unwrap();
    assert!(code.contains("LegalisSDK Ruby SDK"));
    assert!(code.contains("module LegalisSDK"));
    assert!(code.contains("class StatuteRegistryClient"));
}

#[test]
fn test_sdk_language_variants() {
    use api_extensions::sdk_gen::*;

    let _python = SdkLanguage::Python;
    let _javascript = SdkLanguage::JavaScript;
    let _typescript = SdkLanguage::TypeScript;
    let _rust = SdkLanguage::Rust;
    let _go = SdkLanguage::Go;
    let _java = SdkLanguage::Java;
    let _csharp = SdkLanguage::CSharp;
    let _ruby = SdkLanguage::Ruby;
}

// ========================================================================
// Event Sourcing 2.0 Tests (v0.2.6)
// ========================================================================

#[test]
fn test_time_travel_query_creation() {
    use event_sourcing_v2::*;

    let target_time = Utc::now();
    let query = TimeTravelQuery::new(target_time);
    assert_eq!(query.target_time, target_time);
    assert!(query.statute_filter.is_none());
    assert!(query.event_types.is_empty());
}

#[test]
fn test_time_travel_query_builder() {
    use event_sourcing_v2::*;

    let target_time = Utc::now();
    let query = TimeTravelQuery::new(target_time)
        .for_statute("S1".to_string())
        .with_event_types(vec!["StatuteRegistered".to_string()]);

    assert_eq!(query.statute_filter, Some("S1".to_string()));
    assert_eq!(query.event_types.len(), 1);
}

#[test]
fn test_event_replay_engine_creation() {
    use event_sourcing_v2::*;

    let store = Arc::new(Mutex::new(EventStore::new()));
    let engine = EventReplayEngine::new(store);
    assert!(format!("{:?}", engine).contains("EventReplayEngine"));
}

#[test]
fn test_event_replay_basic() {
    use event_sourcing_v2::*;

    let store = Arc::new(Mutex::new(EventStore::new()));
    {
        let mut s = store.lock().unwrap();
        let event = RegistryEvent::StatuteRegistered {
            registry_id: Uuid::new_v4(),
            statute_id: "S1".to_string(),
            jurisdiction: "JP".to_string(),
            timestamp: Utc::now() - chrono::Duration::hours(1),
        };
        s.record(event);
    }

    let engine = EventReplayEngine::new(store);
    let query = TimeTravelQuery::new(Utc::now());
    let result = engine.replay(query).unwrap();

    assert_eq!(result.events_replayed, 1);
    assert_eq!(result.statutes.len(), 1);
    assert!(result.statutes.contains_key("S1"));
}

#[test]
fn test_event_replay_with_filter() {
    use event_sourcing_v2::*;

    let store = Arc::new(Mutex::new(EventStore::new()));
    {
        let mut s = store.lock().unwrap();
        let event1 = RegistryEvent::StatuteRegistered {
            registry_id: Uuid::new_v4(),
            statute_id: "S1".to_string(),
            jurisdiction: "JP".to_string(),
            timestamp: Utc::now() - chrono::Duration::hours(2),
        };
        let event2 = RegistryEvent::StatuteRegistered {
            registry_id: Uuid::new_v4(),
            statute_id: "S2".to_string(),
            jurisdiction: "US".to_string(),
            timestamp: Utc::now() - chrono::Duration::hours(1),
        };
        s.record(event1);
        s.record(event2);
    }

    let engine = EventReplayEngine::new(store);
    let query = TimeTravelQuery::new(Utc::now()).for_statute("S1".to_string());
    let result = engine.replay(query).unwrap();

    assert_eq!(result.events_replayed, 1);
    assert_eq!(result.statutes.len(), 1);
    assert!(result.statutes.contains_key("S1"));
}

#[test]
fn test_projection_type_variants() {
    use event_sourcing_v2::*;

    let _event_type_count = ProjectionType::EventTypeCount;
    let _statute_activity = ProjectionType::StatuteActivityCount;
    let _status_timeline = ProjectionType::StatusChangeTimeline;
    let _tag_usage = ProjectionType::TagUsageStats;
    let _daily_activity = ProjectionType::DailyActivitySummary;
}

#[test]
fn test_projection_engine_creation() {
    use event_sourcing_v2::*;

    let store = Arc::new(Mutex::new(EventStore::new()));
    let engine = ProjectionEngine::new(store);
    assert!(format!("{:?}", engine).contains("ProjectionEngine"));
}

#[test]
fn test_projection_event_type_count() {
    use event_sourcing_v2::*;

    let store = Arc::new(Mutex::new(EventStore::new()));
    {
        let mut s = store.lock().unwrap();
        s.record(RegistryEvent::StatuteRegistered {
            registry_id: Uuid::new_v4(),
            statute_id: "S1".to_string(),
            jurisdiction: "JP".to_string(),
            timestamp: Utc::now(),
        });
        s.record(RegistryEvent::StatuteUpdated {
            statute_id: "S1".to_string(),
            old_version: 1,
            new_version: 2,
            timestamp: Utc::now(),
        });
        s.record(RegistryEvent::StatuteRegistered {
            registry_id: Uuid::new_v4(),
            statute_id: "S2".to_string(),
            jurisdiction: "US".to_string(),
            timestamp: Utc::now(),
        });
    }

    let engine = ProjectionEngine::new(store);
    let result = engine.project(ProjectionType::EventTypeCount);

    assert_eq!(result.events_processed, 3);
    assert_eq!(result.data.get("StatuteRegistered"), Some(&2));
    assert_eq!(result.data.get("StatuteUpdated"), Some(&1));
}

#[test]
fn test_projection_statute_activity() {
    use event_sourcing_v2::*;

    let store = Arc::new(Mutex::new(EventStore::new()));
    {
        let mut s = store.lock().unwrap();
        s.record(RegistryEvent::StatuteRegistered {
            registry_id: Uuid::new_v4(),
            statute_id: "S1".to_string(),
            jurisdiction: "JP".to_string(),
            timestamp: Utc::now(),
        });
        s.record(RegistryEvent::TagAdded {
            statute_id: "S1".to_string(),
            tag: "important".to_string(),
            timestamp: Utc::now(),
        });
        s.record(RegistryEvent::StatusChanged {
            statute_id: "S1".to_string(),
            old_status: StatuteStatus::Draft,
            new_status: StatuteStatus::Active,
            timestamp: Utc::now(),
        });
    }

    let engine = ProjectionEngine::new(store);
    let result = engine.project(ProjectionType::StatuteActivityCount);

    assert_eq!(result.events_processed, 3);
    assert_eq!(result.data.get("S1"), Some(&3));
}

#[test]
fn test_notification_channel_variants() {
    use event_sourcing_v2::*;

    let _email = NotificationChannel::Email("test@example.com".to_string());
    let _webhook = NotificationChannel::Webhook("https://example.com/hook".to_string());
    let _sms = NotificationChannel::Sms("+1234567890".to_string());
    let _in_app = NotificationChannel::InApp("user123".to_string());
}

#[test]
fn test_notification_rule_creation() {
    use event_sourcing_v2::*;

    let rule = NotificationRule::new("Test Rule".to_string(), "StatuteRegistered".to_string());

    assert!(!rule.name.is_empty());
    assert_eq!(rule.event_pattern, "StatuteRegistered");
    assert!(rule.enabled);
    assert!(rule.channels.is_empty());
}

#[test]
fn test_notification_rule_with_channels() {
    use event_sourcing_v2::*;

    let rule = NotificationRule::new("Test Rule".to_string(), "StatuteRegistered".to_string())
        .add_channel(NotificationChannel::Email("test@example.com".to_string()))
        .add_channel(NotificationChannel::Webhook(
            "https://example.com/hook".to_string(),
        ));

    assert_eq!(rule.channels.len(), 2);
}

#[test]
fn test_notification_manager_add_remove_rules() {
    use event_sourcing_v2::*;

    let manager = NotificationManager::new();
    let rule = NotificationRule::new("Test Rule".to_string(), "StatuteRegistered".to_string());
    let rule_id = rule.id;

    manager.add_rule(rule);
    assert_eq!(manager.list_rules().len(), 1);

    assert!(manager.remove_rule(rule_id));
    assert_eq!(manager.list_rules().len(), 0);
}

#[test]
fn test_notification_manager_process_event() {
    use event_sourcing_v2::*;

    let manager = NotificationManager::new();
    let rule = NotificationRule::new("Test Rule".to_string(), "StatuteRegistered".to_string())
        .add_channel(NotificationChannel::Email("test@example.com".to_string()));

    manager.add_rule(rule);

    let event = RegistryEvent::StatuteRegistered {
        registry_id: Uuid::new_v4(),
        statute_id: "S1".to_string(),
        jurisdiction: "JP".to_string(),
        timestamp: Utc::now(),
    };

    let notifications_sent = manager.process_event(&event);
    assert_eq!(notifications_sent, 1);
}

#[test]
fn test_notification_manager_wildcard_pattern() {
    use event_sourcing_v2::*;

    let manager = NotificationManager::new();
    let rule = NotificationRule::new("Catch All".to_string(), "*".to_string())
        .add_channel(NotificationChannel::Email("test@example.com".to_string()));

    manager.add_rule(rule);

    let event = RegistryEvent::TagAdded {
        statute_id: "S1".to_string(),
        tag: "important".to_string(),
        timestamp: Utc::now(),
    };

    let notifications_sent = manager.process_event(&event);
    assert_eq!(notifications_sent, 1);
}

#[test]
fn test_cold_storage_config_default() {
    use event_sourcing_v2::*;

    let config = ColdStorageConfig::default();
    assert_eq!(config.archive_after, chrono::Duration::days(90));
    assert!(config.compression);
    assert!(!config.archive_path.is_empty());
}

#[test]
fn test_event_archiver_creation() {
    use event_sourcing_v2::*;

    let config = ColdStorageConfig::default();
    let archiver = EventArchiver::new(config);
    assert!(format!("{:?}", archiver).contains("EventArchiver"));
}

#[test]
fn test_event_archiver_archive_old_events() {
    use event_sourcing_v2::*;

    let config = ColdStorageConfig {
        archive_after: chrono::Duration::hours(1),
        compression: true,
        archive_path: std::env::temp_dir()
            .join("legalis-registry-archive-test")
            .to_string_lossy()
            .into_owned(),
    };
    let archiver = EventArchiver::new(config);
    let mut store = EventStore::new();

    // Add old event
    let old_event = RegistryEvent::StatuteRegistered {
        registry_id: Uuid::new_v4(),
        statute_id: "S1".to_string(),
        jurisdiction: "JP".to_string(),
        timestamp: Utc::now() - chrono::Duration::hours(2),
    };
    store.record(old_event);

    // Add recent event
    let recent_event = RegistryEvent::StatuteRegistered {
        registry_id: Uuid::new_v4(),
        statute_id: "S2".to_string(),
        jurisdiction: "US".to_string(),
        timestamp: Utc::now(),
    };
    store.record(recent_event);

    let archived_count = archiver.archive_old_events(&mut store).unwrap();
    assert_eq!(archived_count, 1);
    assert_eq!(store.count(), 1);
}

#[test]
fn test_event_archiver_get_archived_events() {
    use event_sourcing_v2::*;

    let config = ColdStorageConfig {
        archive_after: chrono::Duration::hours(1),
        compression: true,
        archive_path: std::env::temp_dir()
            .join("legalis-registry-archive-test")
            .to_string_lossy()
            .into_owned(),
    };
    let archiver = EventArchiver::new(config);
    let mut store = EventStore::new();

    let old_event = RegistryEvent::StatuteRegistered {
        registry_id: Uuid::new_v4(),
        statute_id: "S1".to_string(),
        jurisdiction: "JP".to_string(),
        timestamp: Utc::now() - chrono::Duration::hours(2),
    };
    store.record(old_event);

    archiver.archive_old_events(&mut store).unwrap();
    let batches = archiver.get_archived_events();
    assert_eq!(batches.len(), 1);
    assert_eq!(batches[0].events.len(), 1);
}

#[test]
fn test_event_archiver_restore_batch() {
    use event_sourcing_v2::*;

    let config = ColdStorageConfig {
        archive_after: chrono::Duration::hours(1),
        compression: true,
        archive_path: std::env::temp_dir()
            .join("legalis-registry-archive-test")
            .to_string_lossy()
            .into_owned(),
    };
    let archiver = EventArchiver::new(config);
    let mut store = EventStore::new();

    let old_event = RegistryEvent::StatuteRegistered {
        registry_id: Uuid::new_v4(),
        statute_id: "S1".to_string(),
        jurisdiction: "JP".to_string(),
        timestamp: Utc::now() - chrono::Duration::hours(2),
    };
    store.record(old_event);

    archiver.archive_old_events(&mut store).unwrap();
    assert_eq!(store.count(), 0);

    let batches = archiver.get_archived_events();
    let batch_id = batches[0].id;

    let restored_count = archiver.restore_batch(batch_id, &mut store).unwrap();
    assert_eq!(restored_count, 1);
    assert_eq!(store.count(), 1);
}

#[test]
fn test_schema_version_creation() {
    use event_sourcing_v2::*;

    let version = SchemaVersion::new(1, 2, 3);
    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 2);
    assert_eq!(version.patch, 3);
}

#[test]
fn test_schema_version_current() {
    use event_sourcing_v2::*;

    let version = SchemaVersion::current();
    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 0);
    assert_eq!(version.patch, 0);
}

#[test]
fn test_schema_version_display() {
    use event_sourcing_v2::*;

    let version = SchemaVersion::new(1, 2, 3);
    assert_eq!(format!("{}", version), "1.2.3");
}

#[test]
fn test_schema_version_comparison() {
    use event_sourcing_v2::*;

    let v1 = SchemaVersion::new(1, 0, 0);
    let v2 = SchemaVersion::new(1, 1, 0);
    let v3 = SchemaVersion::new(2, 0, 0);

    assert!(v1 < v2);
    assert!(v2 < v3);
    assert!(v1 < v3);
}

#[test]
fn test_versioned_event_creation() {
    use event_sourcing_v2::*;

    let event = RegistryEvent::StatuteRegistered {
        registry_id: Uuid::new_v4(),
        statute_id: "S1".to_string(),
        jurisdiction: "JP".to_string(),
        timestamp: Utc::now(),
    };

    let versioned = VersionedEvent::new(event);
    assert_eq!(versioned.schema_version, SchemaVersion::current());
    assert!(versioned.migration_history.is_empty());
}

#[test]
fn test_schema_evolution_manager_creation() {
    use event_sourcing_v2::*;

    let manager = SchemaEvolutionManager::new();
    assert_eq!(manager.current_version(), SchemaVersion::current());
}

// ========================================================================
// Federation Protocol Tests (v0.2.7)
// ========================================================================

#[test]
fn test_registry_metadata_creation() {
    use federation::*;

    let metadata = RegistryMetadata::new(
        "Test Registry".to_string(),
        "https://example.com".to_string(),
    );
    assert_eq!(metadata.name, "Test Registry");
    assert_eq!(metadata.endpoint, "https://example.com");
    assert_eq!(metadata.api_version, "1.0.0");
    assert_eq!(metadata.trust_level, 50);
    assert!(metadata.is_active());
}

#[test]
fn test_registry_metadata_update_last_seen() {
    use federation::*;

    let mut metadata = RegistryMetadata::new("Test".to_string(), "https://example.com".to_string());
    let old_time = metadata.last_seen;
    std::thread::sleep(std::time::Duration::from_millis(10));
    metadata.update_last_seen();
    assert!(metadata.last_seen > old_time);
}

#[test]
fn test_registry_capability_variants() {
    use federation::*;

    let _full_text = RegistryCapability::FullTextSearch;
    let _version_control = RegistryCapability::VersionControl;
    let _real_time = RegistryCapability::RealTimeUpdates;
    let _event_sourcing = RegistryCapability::EventSourcing;
    let _graphql = RegistryCapability::GraphQL;
    let _bulk = RegistryCapability::BulkOperations;
}

#[test]
fn test_registry_discovery_creation() {
    use federation::*;

    let discovery = RegistryDiscovery::new();
    assert_eq!(discovery.list_registries().len(), 0);
}

#[test]
fn test_registry_discovery_register() {
    use federation::*;

    let discovery = RegistryDiscovery::new();
    let metadata = RegistryMetadata::new(
        "Test Registry".to_string(),
        "https://example.com".to_string(),
    );
    let registry_id = metadata.registry_id;

    discovery.register(metadata);
    assert_eq!(discovery.list_registries().len(), 1);
    assert!(discovery.unregister(registry_id));
    assert_eq!(discovery.list_registries().len(), 0);
}

#[test]
fn test_registry_discovery_find_by_jurisdiction() {
    use federation::*;

    let discovery = RegistryDiscovery::new();
    let mut metadata = RegistryMetadata::new(
        "JP Registry".to_string(),
        "https://jp.example.com".to_string(),
    );
    metadata.jurisdictions.push("JP".to_string());
    discovery.register(metadata);

    let results = discovery.find_by_jurisdiction("JP");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "JP Registry");
}

#[test]
fn test_registry_discovery_get_active_registries() {
    use federation::*;

    let discovery = RegistryDiscovery::new();
    let metadata = RegistryMetadata::new(
        "Active Registry".to_string(),
        "https://example.com".to_string(),
    );
    discovery.register(metadata);

    let active = discovery.get_active_registries();
    assert_eq!(active.len(), 1);
}

#[test]
fn test_federated_query_creation() {
    use federation::*;

    let query = FederatedQuery::new("test query".to_string());
    assert_eq!(query.query, "test query");
    assert_eq!(query.max_results_per_registry, 50);
    assert_eq!(query.timeout, 30);
}

#[test]
fn test_federated_query_builder() {
    use federation::*;

    let registry_id = Uuid::new_v4();
    let query = FederatedQuery::new("test".to_string())
        .with_jurisdictions(vec!["JP".to_string()])
        .with_target_registries(vec![registry_id]);

    assert_eq!(query.jurisdictions.len(), 1);
    assert_eq!(query.target_registries.len(), 1);
}

#[test]
fn test_federated_query_engine_creation() {
    use federation::*;

    let discovery = Arc::new(RegistryDiscovery::new());
    let engine = FederatedQueryEngine::new(discovery);
    assert!(format!("{:?}", engine).contains("FederatedQueryEngine"));
}

#[test]
fn test_federated_query_engine_execute() {
    use federation::*;

    let discovery = Arc::new(RegistryDiscovery::new());
    let metadata = RegistryMetadata::new(
        "Test Registry".to_string(),
        "https://example.com".to_string(),
    );
    discovery.register(metadata);

    let engine = FederatedQueryEngine::new(discovery);
    let query = FederatedQuery::new("test".to_string());
    let result = engine.execute(query);

    assert_eq!(result.query, "test");
    assert_eq!(result.registries_queried, 1);
    assert_eq!(result.successful_queries, 1);
}

#[test]
fn test_peering_status_variants() {
    use federation::*;

    let _pending = PeeringStatus::Pending;
    let _active = PeeringStatus::Active;
    let _suspended = PeeringStatus::Suspended;
    let _terminated = PeeringStatus::Terminated;
}

#[test]
fn test_sharing_level_variants() {
    use federation::*;

    let _public = SharingLevel::Public;
    let _metadata = SharingLevel::Metadata;
    let _full = SharingLevel::Full;
    let _bidirectional = SharingLevel::Bidirectional;
}

#[test]
fn test_peering_agreement_creation() {
    use federation::*;

    let local_id = Uuid::new_v4();
    let peer_id = Uuid::new_v4();
    let agreement = PeeringAgreement::new(local_id, peer_id, SharingLevel::Full);

    assert_eq!(agreement.local_registry, local_id);
    assert_eq!(agreement.peer_registry, peer_id);
    assert_eq!(agreement.status, PeeringStatus::Pending);
    assert_eq!(agreement.sharing_level, SharingLevel::Full);
}

#[test]
fn test_peering_agreement_activate() {
    use federation::*;

    let mut agreement = PeeringAgreement::new(Uuid::new_v4(), Uuid::new_v4(), SharingLevel::Full);
    agreement.activate();
    assert_eq!(agreement.status, PeeringStatus::Active);
    assert!(agreement.is_valid());
}

#[test]
fn test_peering_agreement_suspend() {
    use federation::*;

    let mut agreement = PeeringAgreement::new(Uuid::new_v4(), Uuid::new_v4(), SharingLevel::Full);
    agreement.activate();
    agreement.suspend();
    assert_eq!(agreement.status, PeeringStatus::Suspended);
    assert!(!agreement.is_valid());
}

#[test]
fn test_peering_agreement_terminate() {
    use federation::*;

    let mut agreement = PeeringAgreement::new(Uuid::new_v4(), Uuid::new_v4(), SharingLevel::Full);
    agreement.activate();
    agreement.terminate();
    assert_eq!(agreement.status, PeeringStatus::Terminated);
    assert!(!agreement.is_valid());
}

#[test]
fn test_peering_manager_creation() {
    use federation::*;

    let manager = PeeringManager::new();
    assert!(format!("{:?}", manager).contains("PeeringManager"));
}

#[test]
fn test_peering_manager_create_agreement() {
    use federation::*;

    let manager = PeeringManager::new();
    let agreement = PeeringAgreement::new(Uuid::new_v4(), Uuid::new_v4(), SharingLevel::Full);
    let id = agreement.id;

    let created_id = manager.create_agreement(agreement);
    assert_eq!(created_id, id);
    assert!(manager.get_agreement(id).is_some());
}

#[test]
fn test_peering_manager_list_agreements() {
    use federation::*;

    let manager = PeeringManager::new();
    let registry_id = Uuid::new_v4();
    let agreement = PeeringAgreement::new(registry_id, Uuid::new_v4(), SharingLevel::Full);

    manager.create_agreement(agreement);
    let agreements = manager.list_agreements(registry_id);
    assert_eq!(agreements.len(), 1);
}

#[test]
fn test_peering_manager_get_active_agreements() {
    use federation::*;

    let manager = PeeringManager::new();
    let registry_id = Uuid::new_v4();
    let mut agreement = PeeringAgreement::new(registry_id, Uuid::new_v4(), SharingLevel::Full);
    agreement.activate();

    manager.create_agreement(agreement);
    let active = manager.get_active_agreements(registry_id);
    assert_eq!(active.len(), 1);
}

#[test]
fn test_ranking_strategy_variants() {
    use federation::*;

    let _relevance = RankingStrategy::Relevance;
    let _trust = RankingStrategy::TrustLevel;
    let _recency = RankingStrategy::Recency;
    let _combined = RankingStrategy::Combined;
}

#[test]
fn test_federated_search_aggregator_creation() {
    use federation::*;

    let aggregator = FederatedSearchAggregator::new(RankingStrategy::Combined);
    assert!(format!("{:?}", aggregator).contains("FederatedSearchAggregator"));
}

#[test]
fn test_federated_search_aggregator_deduplicate() {
    use federation::*;

    let aggregator = FederatedSearchAggregator::new(RankingStrategy::Relevance);
    let results = vec![
        AggregatedSearchResult {
            statute_id: "S1".to_string(),
            registry_id: Uuid::new_v4(),
            registry_name: "R1".to_string(),
            relevance_score: 1.0,
            trust_level: 50,
            combined_score: 0.0,
        },
        AggregatedSearchResult {
            statute_id: "S1".to_string(),
            registry_id: Uuid::new_v4(),
            registry_name: "R2".to_string(),
            relevance_score: 0.9,
            trust_level: 60,
            combined_score: 0.0,
        },
    ];

    let deduplicated = aggregator.deduplicate(results);
    assert_eq!(deduplicated.len(), 1);
}

#[test]
fn test_trust_level_variants() {
    use federation::*;

    let _untrusted = TrustLevel::Untrusted;
    let _low = TrustLevel::Low;
    let _medium = TrustLevel::Medium;
    let _high = TrustLevel::High;
    let _verified = TrustLevel::Verified;
}

#[test]
fn test_trust_level_from_score() {
    use federation::*;

    assert_eq!(TrustLevel::from_score(10), TrustLevel::Untrusted);
    assert_eq!(TrustLevel::from_score(30), TrustLevel::Low);
    assert_eq!(TrustLevel::from_score(50), TrustLevel::Medium);
    assert_eq!(TrustLevel::from_score(70), TrustLevel::High);
    assert_eq!(TrustLevel::from_score(90), TrustLevel::Verified);
}

#[test]
fn test_trust_level_to_score() {
    use federation::*;

    assert_eq!(TrustLevel::Untrusted.to_score(), 10);
    assert_eq!(TrustLevel::Low.to_score(), 30);
    assert_eq!(TrustLevel::Medium.to_score(), 50);
    assert_eq!(TrustLevel::High.to_score(), 70);
    assert_eq!(TrustLevel::Verified.to_score(), 90);
}

#[test]
fn test_trust_metric_creation() {
    use federation::*;

    let registry_id = Uuid::new_v4();
    let metric = TrustMetric::new(registry_id);
    assert_eq!(metric.registry_id, registry_id);
    assert_eq!(metric.trust_score, 50);
    assert_eq!(metric.trust_level(), TrustLevel::Medium);
}

#[test]
fn test_trust_metric_calculate_trust_score() {
    use federation::*;

    let registry_id = Uuid::new_v4();
    let mut metric = TrustMetric::new(registry_id);
    metric.uptime = 99.9;
    metric.avg_response_time = 50;
    metric.success_rate = 98.0;
    metric.data_quality = 80;
    metric.reputation = 85;

    metric.calculate_trust_score();
    assert!(metric.trust_score > 80);
    assert_eq!(metric.trust_level(), TrustLevel::Verified);
}

#[test]
fn test_trust_framework_creation() {
    use federation::*;

    let framework = TrustFramework::new();
    assert!(format!("{:?}", framework).contains("TrustFramework"));
}

#[test]
fn test_trust_framework_update_metrics() {
    use federation::*;

    let framework = TrustFramework::new();
    let registry_id = Uuid::new_v4();
    let metric = TrustMetric::new(registry_id);

    framework.update_metrics(metric);
    assert!(framework.get_metrics(registry_id).is_some());
}

#[test]
fn test_trust_framework_get_trust_score() {
    use federation::*;

    let framework = TrustFramework::new();
    let registry_id = Uuid::new_v4();
    let mut metric = TrustMetric::new(registry_id);
    metric.trust_score = 75;

    framework.update_metrics(metric);
    assert_eq!(framework.get_trust_score(registry_id), 75);
}

#[test]
fn test_trust_framework_list_by_trust_level() {
    use federation::*;

    let framework = TrustFramework::new();
    let registry_id1 = Uuid::new_v4();
    let registry_id2 = Uuid::new_v4();

    let mut metric1 = TrustMetric::new(registry_id1);
    metric1.trust_score = 85;
    framework.update_metrics(metric1);

    let mut metric2 = TrustMetric::new(registry_id2);
    metric2.trust_score = 45;
    framework.update_metrics(metric2);

    let verified = framework.list_by_trust_level(TrustLevel::Verified);
    assert_eq!(verified.len(), 1);
    assert_eq!(verified[0], registry_id1);
}

#[test]
fn test_trust_framework_recalculate_all() {
    use federation::*;

    let framework = TrustFramework::new();
    let registry_id = Uuid::new_v4();
    let metric = TrustMetric::new(registry_id);

    framework.update_metrics(metric);
    framework.recalculate_all();
    assert!(framework.get_metrics(registry_id).is_some());
}
